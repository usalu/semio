use std::os::unix::io::AsRawFd;
fn main() {
    for k in ["CARGO_MAKEFLAGS", "MAKEFLAGS", "MFLAGS", "NUM_JOBS"] {
        println!("cargo:warning={}={:?}", k, std::env::var(k));
    }
    let mut open = vec![];
    for fd in 3..64 { let r = unsafe { libc_fcntl(fd) }; if r >= 0 { open.push(fd); } }
    println!("cargo:warning=open fds {:?}", open);
    let _ = std::io::stdin().as_raw_fd();
}
extern "C" { fn fcntl(fd: i32, cmd: i32, ...) -> i32; }
unsafe fn libc_fcntl(fd: i32) -> i32 { fcntl(fd, 1) }
