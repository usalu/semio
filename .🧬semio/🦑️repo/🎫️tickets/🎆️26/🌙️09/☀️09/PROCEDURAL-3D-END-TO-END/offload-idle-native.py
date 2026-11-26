"""📦️ Move verified idle native intermediates to ticket scratch on the shared fleet store."""
from pathlib import Path
import fcntl
import hashlib
import os
import shutil
import sys
import time

def digest(path):
    h = hashlib.sha256()
    with path.open("rb") as stream:
        for data in iter(lambda: stream.read(1024*1024), b""):
            h.update(data)
    return h.digest()

root = Path(sys.argv[1]).resolve()
ticket = Path(sys.argv[2]).resolve()
if root.name not in ("flow-native-n25","flow-web-n25","g3d-p-n25","engine-n25"):
    raise RuntimeError("expected the ticket's private native Cargo store")
allow_readers = len(sys.argv) > 3 and sys.argv[3] == "--sealed-native"
if allow_readers and root.name!="flow-native-n25": raise ValueError("the sealed native mode applies only to this private native test store")
destination = (ticket / "cold-compiler.tar").resolve(strict=True).parent / "idle-native" / root.name
destination.mkdir(exist_ok=True, parents=True)
candidates = []
for path in (*root.glob("debug/build/*/*/out/*.rlib"), *root.glob("*/debug/build/*/*/out/*.rlib")):
    if path.is_symlink():
        continue
    try: stat = path.stat()
    except FileNotFoundError: continue
    if stat.st_nlink == 1 and stat.st_size >= 512*1024 and time.time() - stat.st_mtime > 1800:
        candidates.append((stat.st_size,path,stat))
for size, path, old in sorted(candidates,reverse=True):
    if shutil.disk_usage(root).free > 15*2**30: break
    lock_path = path.parent.parent / ".lock"
    if not lock_path.is_file(): continue
    with lock_path.open("rb") as lock:
        try: fcntl.flock(lock,(fcntl.LOCK_SH if allow_readers else fcntl.LOCK_EX)|fcntl.LOCK_NB)
        except BlockingIOError: continue
        if path.is_symlink() or not path.exists() or path.stat().st_mtime_ns != old.st_mtime_ns: continue
        check=digest(path)
        remote = destination / path.name
        if remote.exists() and digest(remote) != check:
            remote = destination / (check.hex()[:16]+"-"+path.name)
        if not remote.exists(): shutil.copy2(path,remote)
        if check != digest(remote): raise ValueError(path)
        pending=path.with_suffix(".ticket-offload")
        os.symlink(remote,pending)
        os.utime(pending,ns=(old.st_atime_ns,old.st_mtime_ns),follow_symlinks=False)
        os.replace(pending,path)
        print(check.hex(),size,path.relative_to(root),flush=True)
