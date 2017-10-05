#[allow(non_camel_case_types)]
pub enum SYSCALL {
    read                = 0,
    write               = 1,
    open                = 2,
    close               = 3,
    stat                = 4,
    fstat               = 5,
    lstat               = 6,
    poll                = 7,
    mmap                = 9,
    mprotect            = 10,
    munmap              = 11,
    ioctl               = 16,
    socket              = 41,
    connect             = 42,
    sendto              = 44,
    recvfrom            = 45,
    sendmsg             = 46,
    recvmsg             = 47,
    bind                = 49,
    getsockname         = 51,
    setsockopt          = 54,
    getsockopt          = 55,
    clone               = 56,
    uname               = 63,
    fcntl               = 72,
    getdents            = 78,
    geteuid             = 107,
    getresuid           = 118,
    getresgid           = 120,
    sigaltstack         = 131,
    prctl               = 157,
    futex               = 202,
    sched_getaffinity   = 204,
    clock_getres        = 229,
    exit_group          = 231,
    set_robust_list     = 273,
    openat              = 257,
    seccomp             = 317,
    getrandom           = 318,
}
