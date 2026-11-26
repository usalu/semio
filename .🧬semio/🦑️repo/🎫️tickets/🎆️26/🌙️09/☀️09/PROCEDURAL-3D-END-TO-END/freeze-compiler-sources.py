"""🧊 Pin a read-only compiler input copy while the replicated source tree evolves."""
import concurrent.futures
import ctypes
import os
import pathlib
import shutil
import stat
import sys

source = pathlib.Path.cwd()
name = sys.argv[2] if len(sys.argv) > 2 else "s"
if len(name) != 1 or name not in "abcdefghijklmnopqrstuvwxyz":
    raise ValueError("Compiler input copy requires a single lowercase directory name")
destination = source / ".cache" / "g3d-native" / name
destination.mkdir(exist_ok=False)
paths = sorted({os.fsdecode(p) for p in pathlib.Path(sys.argv[1]).read_bytes().split(b"\0") if p})
roots = {pathlib.Path(p).parts[0] for p in paths}
for p in source.iterdir():
    if p.name in roots:
        continue
    if p.name == ".cargo":
        shutil.copytree(p, destination / p.name, symlinks=True)
        continue
    if p.name in {"Cargo.toml", "Cargo.lock", "rust-toolchain.toml", "package.json", "📋️project.json"}:
        shutil.copy2(p, destination / p.name)
        continue
    if ord(p.name[0]) > 127:
        (destination / p.name).symlink_to(p, target_is_directory=p.is_dir())
clone = getattr(ctypes.CDLL(None, use_errno=True), "clonefile", None)
def copy(path):
    original, target = source / path, destination / path
    if len(str(target)) > 256:
        raise ValueError(f"Path exceeds compiler copy limit: {path}")
    try:
        s = original.lstat()
    except FileNotFoundError:
        return
    if stat.S_ISDIR(s.st_mode):
        return
    target.parent.mkdir(parents=True, exist_ok=True)
    if original.is_symlink():
        target.symlink_to(os.readlink(original))
    elif clone:
        if clone(os.fsencode(original), os.fsencode(target), 0) != 0:
            err = ctypes.get_errno()
            if err == 2 and not original.exists(): return
            raise OSError(err, os.strerror(err), str(original))
    else:
        shutil.copy2(original, target)
with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:
    for _ in pool.map(copy, paths):
        pass
print("[DEBUG] Captured compiler inputs", len(paths), destination, flush=True)
