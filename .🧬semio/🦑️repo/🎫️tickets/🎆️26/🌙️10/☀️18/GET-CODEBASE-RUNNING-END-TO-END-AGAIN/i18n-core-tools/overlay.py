#!/usr/bin/env python3
"""🧪️ i18n-core overlay: APFS-clone (clonefile) of the framework tree plus the root files of a checkout or of another overlay into /private/tmp/i18n-core/<tag>.
usage: overlay.py [--from /Users/ueli/Documents/semio] [--tag ov] ; nothing is written inside the source."""
import ctypes, ctypes.util, os, shutil, subprocess, sys

args = sys.argv[1:]
SRC = args[args.index("--from") + 1] if "--from" in args else "/Users/ueli/Documents/semio"
TAG = args[args.index("--tag") + 1] if "--tag" in args else "ov"
OV = f"/private/tmp/i18n-core/{TAG}"
libc = ctypes.CDLL(ctypes.util.find_library("c"), use_errno=True)
clone = libc.clonefile
clone.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.c_uint32]


def listing():
    if os.path.isdir(os.path.join(SRC, ".git")):
        out = subprocess.run(["git", "ls-files", "-co", "--exclude-standard", "-z", "--", "🧰️framework"], capture_output=True, cwd=SRC).stdout
        rows = [p.decode("utf8") for p in out.split(b"\0") if p]
        rows += [n for n in sorted(os.listdir(SRC)) if os.path.isfile(os.path.join(SRC, n)) and not n.startswith(".")]
        return rows
    rows = []
    for base, dirs, files in os.walk(os.path.join(SRC, "🧰️framework")):
        dirs[:] = [d for d in dirs if d not in ("node_modules", ".cache")]
        rows += [os.path.relpath(os.path.join(base, f), SRC) for f in files]
    rows += [n for n in sorted(os.listdir(SRC)) if os.path.isfile(os.path.join(SRC, n)) and not n.startswith(".")]
    return rows


def main():
    shutil.rmtree(OV, ignore_errors=True)
    made, count = set(), 0
    for rel in listing():
        src, dst = os.path.join(SRC, rel), os.path.join(OV, rel)
        if not os.path.lexists(src):
            continue
        d = os.path.dirname(dst)
        if d not in made:
            os.makedirs(d, exist_ok=True)
            made.add(d)
        if os.path.islink(src):
            os.symlink(os.readlink(src), dst)
        elif os.path.isfile(src):
            if clone(src.encode("utf8"), dst.encode("utf8"), 0) != 0:
                shutil.copy2(src, dst)
            count += 1
    live_nm = os.path.join("/Users/ueli/Documents/semio", "node_modules")
    if not os.path.lexists(os.path.join(OV, "node_modules")):
        os.symlink(live_nm, os.path.join(OV, "node_modules"))
    print("cloned", count, "files ->", OV)


if __name__ == "__main__":
    main()
