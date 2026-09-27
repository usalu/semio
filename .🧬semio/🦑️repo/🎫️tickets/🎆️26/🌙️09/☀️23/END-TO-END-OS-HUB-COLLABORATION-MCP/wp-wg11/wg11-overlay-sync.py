#!/usr/bin/env python3
"""🪞️ WG11: mirrors the working tree's tracked + untracked-not-ignored files into WG11's scratch overlay (gitignored) as APFS clones
(`clonefile`, copy-on-write: no data copied, and an edit in the overlay never touches the tree), re-cloning only files whose size
or mtime differ; files the tree no longer has are removed from the overlay. Usage: python3 wg11-overlay-sync.py
"""
import ctypes
import os
import subprocess
from concurrent.futures import ThreadPoolExecutor

ROOT = "/Users/ueli/Documents/semio"
DEST = os.path.join(ROOT, ".🧬semio/🌐hub/s14-wg11-overlay")
LIBC = ctypes.CDLL("libc.dylib", use_errno=True)
LIBC.clonefile.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.c_uint32]


def clone(rel: str) -> int:
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
    elif LIBC.clonefile(src.encode(), dst.encode(), 0) != 0:
        raise OSError(ctypes.get_errno(), f"clonefile {rel}")
    return 1


def listed(*args: str) -> list[str]:
    out = subprocess.run(["git", "ls-files", "-z", *args], cwd=ROOT, capture_output=True, check=True).stdout.decode("utf-8")
    return [path for path in out.split("\0") if path and not path.startswith(".🧬semio/") and not path.startswith(".tmp-ticket")]


def main() -> None:
    files = sorted(set(listed()) | set(listed("--others", "--exclude-standard")))
    with ThreadPoolExecutor(max_workers=8) as pool:
        cloned = sum(pool.map(clone, files))
    wanted = set(files)
    removed = 0
    for directory, _, names in os.walk(DEST):
        if "/target" in directory or "/.cargo-build" in directory:
            continue
        for name in names:
            rel = os.path.relpath(os.path.join(directory, name), DEST)
            if rel not in wanted and not rel.startswith(("target/", ".cargo-build/")):
                os.remove(os.path.join(directory, name))
                removed += 1
    print(f"files={len(files)} cloned={cloned} removed={removed}")


if __name__ == "__main__":
    main()
