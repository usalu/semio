#!/usr/bin/env python3
"""🪞️ H14 scratch overlay: mirrors the working tree's tracked files into `.🧬semio/🌐hub/s14-h14-overlay` as APFS clones
(clonefile: no data copied), refreshing only files whose size or mtime differ, and never touching the paths listed with
`--keep` (the overlay's own prepared edits). Extra untracked inputs a law reads (staged plugin components) are named with
`--extra <path>` or listed one per line in `--extras-file <file>` (gitignored generated sources the build reads).
usage: h14-overlay-sync.py [--keep <rel> …] [--extra <rel> …] [--extras-file <file>]"""
import ctypes
import os
import subprocess
import sys
from concurrent.futures import ThreadPoolExecutor

ROOT = "/Users/ueli/Documents/semio"
DEST = os.path.join(ROOT, ".🧬semio/🌐hub/s14-h14-overlay")
LIBC = ctypes.CDLL("libc.dylib", use_errno=True)
LIBC.clonefile.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.c_uint32]


def mirror(rel: str, keep: set) -> int:
    if rel in keep:
        return 0
    src, dst = os.path.join(ROOT, rel), os.path.join(DEST, rel)
    try:
        st = os.lstat(src)
    except FileNotFoundError:
        return 0
    if os.path.isdir(src) and not os.path.islink(src):
        return 0
    try:
        dt = os.lstat(dst)
        if dt.st_size == st.st_size and int(dt.st_mtime) == int(st.st_mtime):
            return 0
        os.remove(dst)
    except FileNotFoundError:
        pass
    os.makedirs(os.path.dirname(dst), exist_ok=True)
    if os.path.islink(src):
        os.symlink(os.readlink(src), dst)
        return 1
    if LIBC.clonefile(src.encode(), dst.encode(), 0) != 0:
        raise OSError(ctypes.get_errno(), f"clonefile {rel}")
    return 1


def main() -> None:
    args = sys.argv[1:]
    keep = {args[i + 1] for i, arg in enumerate(args) if arg == "--keep"}
    extra = [args[i + 1] for i, arg in enumerate(args) if arg == "--extra"]
    for listing in [args[i + 1] for i, arg in enumerate(args) if arg == "--extras-file"]:
        extra += [line for line in open(listing, encoding="utf-8").read().splitlines() if line]
    files = [f for f in subprocess.run(["git", "ls-files", "-z"], cwd=ROOT, capture_output=True, check=True).stdout.decode("utf-8").split("\0") if f]
    with ThreadPoolExecutor(max_workers=8) as pool:
        copied = sum(pool.map(lambda rel: mirror(rel, keep), files + extra))
    print(f"tracked={len(files)} extra={len(extra)} kept={len(keep)} refreshed={copied}")


if __name__ == "__main__":
    main()
