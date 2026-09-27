"""🪞️ Mirrors the working tree's tracked files into the EN2 scratch overlay (gitignored), copying only files whose size or mtime differ."""
import os, shutil, subprocess, sys
from concurrent.futures import ThreadPoolExecutor

ROOT = "/Users/ueli/Documents/semio"
DEST = os.path.join(ROOT, ".🧬semio/🌐hub/s14-en2-overlay")


def copy(rel: str) -> int:
    src = os.path.join(ROOT, rel)
    dst = os.path.join(DEST, rel)
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
    except FileNotFoundError:
        pass
    os.makedirs(os.path.dirname(dst), exist_ok=True)
    if os.path.islink(src):
        if os.path.lexists(dst):
            os.remove(dst)
        os.symlink(os.readlink(src), dst)
    else:
        shutil.copy2(src, dst)
    return 1


def main() -> None:
    only = sys.argv[1:]
    files = subprocess.run(["git", "ls-files", "-z", *only], cwd=ROOT, capture_output=True, check=True).stdout.decode("utf-8").split("\0")
    files = [f for f in files if f]
    with ThreadPoolExecutor(max_workers=8) as pool:
        copied = sum(pool.map(copy, files))
    print(f"tracked={len(files)} copied={copied}")


if __name__ == "__main__":
    main()
