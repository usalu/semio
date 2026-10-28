#!/usr/bin/env python3
"""Runs the stored sealed snapshot like the executor seam does (bun --eval loader -- digest args, snapshot on fd 3) and prints wall/user/sys/maxrss of the child tree.
usage: run-sealed.py <store-dir> [--bun PATH] [--env K=V]... -- args...   (stdout/stderr pass through; metrics on stderr, prefixed `#metrics`)"""
import json, os, resource, subprocess, sys, threading, time
store = sys.argv[1]; a = sys.argv[2:]; bun = "/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"; env = dict(os.environ)
bunargs = []; loader_file = None; file_fd = False
while a and a[0] != "--":
    if a[0] == "--bun": bun = a[1]; a = a[2:]
    elif a[0] == "--file-fd": file_fd = True; a = a[1:]
    elif a[0] == "--loader": loader_file = a[1]; a = a[2:]
    elif a[0] == "--bun-arg": bunargs.append(a[1]); a = a[2:]
    elif a[0] == "--env": k, v = a[1].split("=", 1); env[k] = v; a = a[2:]
    else: sys.exit("bad arg " + a[0])
args = a[1:]
cur = json.load(open(os.path.join(store, "current.json")))
loader = open(loader_file or os.path.join(store, cur["loader"] + ".loader.js")).read()
if file_fd:
    rfd = os.open(os.path.join(store, cur["digest"] + ".json"), os.O_RDONLY)
else:
    data = open(os.path.join(store, cur["digest"] + ".json"), "rb").read()
    import socket
    sa, sb = socket.socketpair()
    rfd, wfd = sa.detach(), sb.detach()
    def feed():
        off = 0
        while off < len(data):
            off += os.write(wfd, data[off:off + 1048576])
        os.close(wfd)
    threading.Thread(target=feed, daemon=True).start()
if rfd != 3:
    os.dup2(rfd, 3); os.close(rfd)
t0 = time.time(); r0 = resource.getrusage(resource.RUSAGE_CHILDREN)
p = subprocess.Popen([bun, *bunargs, "--eval", loader, "--", cur["digest"], *args], pass_fds=[3], env=env, stdin=subprocess.DEVNULL)
code = p.wait(); r1 = resource.getrusage(resource.RUSAGE_CHILDREN)
print(f"#metrics exit={code} wall={time.time()-t0:.2f}s user={r1.ru_utime-r0.ru_utime:.2f}s sys={r1.ru_stime-r0.ru_stime:.2f}s maxrss={r1.ru_maxrss/1048576:.0f}MB", file=sys.stderr)
