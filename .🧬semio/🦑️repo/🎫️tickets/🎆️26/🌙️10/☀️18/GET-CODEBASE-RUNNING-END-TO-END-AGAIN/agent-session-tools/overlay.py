#!/usr/bin/env python3
"""Overlay for the agent-session set: an APFS clone (clonefile) of every tracked and untracked-not-ignored file under the listed top-level paths of the checkout at /private/tmp/agent-session/ov-<tag>, root node_modules symlinked, nothing written inside the checkout.
usage: overlay.py [--tag a] [--path <top-level> ...]   -> prints the overlay path"""
import ctypes, ctypes.util, os, shutil, subprocess, sys

LIVE = "/Users/ueli/Documents/semio"
args = sys.argv[1:]
TAG = args[args.index("--tag") + 1] if "--tag" in args else "a"
PATHS = [args[i + 1] for i, a in enumerate(args) if a == "--path"] or ["🧰️framework", ":(glob)*"]
OV = f"/private/tmp/agent-session/ov-{TAG}"
libc = ctypes.CDLL(ctypes.util.find_library("c"), use_errno=True)
clone = libc.clonefile
clone.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.c_uint32]


def main():
    shutil.rmtree(OV, ignore_errors=True)
    os.makedirs(OV)
    out = subprocess.run(["git", "ls-files", "-co", "--exclude-standard", "-z", "--", *PATHS], capture_output=True, cwd=LIVE).stdout
    made, n = set(), 0
    for raw in out.split(b"\0"):
        if not raw:
            continue
        rel = raw.decode("utf8")
        src, dst = os.path.join(LIVE, rel), os.path.join(OV, rel)
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
            n += 1
    for name in ("package.json", "bunfig.toml", "tsconfig.json", "tsconfig.base.json", "AGENTS.md"):
        s = os.path.join(LIVE, name)
        if os.path.isfile(s) and not os.path.lexists(os.path.join(OV, name)):
            if clone(s.encode("utf8"), os.path.join(OV, name).encode("utf8"), 0) != 0:
                shutil.copy2(s, os.path.join(OV, name))
    if os.path.exists(os.path.join(LIVE, "node_modules")) and not os.path.lexists(os.path.join(OV, "node_modules")):
        os.symlink(os.path.join(LIVE, "node_modules"), os.path.join(OV, "node_modules"))
    print("cloned", n, "files", file=sys.stderr)
    print(OV)


main()
