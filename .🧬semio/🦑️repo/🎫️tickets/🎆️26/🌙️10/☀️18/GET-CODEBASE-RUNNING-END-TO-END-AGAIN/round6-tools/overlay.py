#!/usr/bin/env python3
"""Round-6 overlay: a copy-on-write clone (APFS clonefile) of every tracked and untracked-not-ignored file of the checkout at /private/tmp/i6/ov-<tag>,
with node_modules symlinked, a private git repository (never the checkout's) and the live root literal replaced in tool scripts.
usage: overlay.py [--tag a] [--refresh-tools]   -> prints the overlay path. Nothing is written inside the checkout."""
import ctypes, ctypes.util, os, shutil, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
LIVE = "/Users/ueli/Documents/semio"
CREL = os.path.relpath(C, LIVE)
args = sys.argv[1:]
TAG = args[args.index("--tag") + 1] if "--tag" in args else "a"
OV = f"/private/tmp/i6/ov-{TAG}"
SHIM = OV
libc = ctypes.CDLL(ctypes.util.find_library("c"), use_errno=True)
clone = libc.clonefile
clone.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.c_uint32]


def files():
    out = subprocess.run(["git", "ls-files", "-co", "--exclude-standard", "-z"], capture_output=True, cwd=LIVE).stdout
    return [p.decode("utf8") for p in out.split(b"\0") if p]


def copy_all(rows):
    made = set()
    n = 0
    for rel in rows:
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
    return n


def fill_ticket():
    """Ticket-folder files that .gitignore hides (e.g. `target*` matches targets.py) but the tools need."""
    rows = []
    for base, dirs, fs in os.walk(C):
        dirs[:] = [d for d in dirs if d not in ("node_modules", "__pycache__", "🗑\ufe0fgenerated")]
        for f in fs:
            rel = os.path.relpath(os.path.join(base, f), LIVE)
            if not os.path.lexists(os.path.join(OV, rel)):
                rows.append(rel)
    return copy_all(rows)


def shim():
    n = 0
    for base, dirs, fs in os.walk(os.path.join(OV, CREL)):
        dirs[:] = [d for d in dirs if d not in ("node_modules", "backup", "originals", "🗑️generated", "__pycache__") and not d.startswith("backup")]
        rel = os.path.relpath(base, os.path.join(OV, CREL))
        if "/staged" in "/" + rel or "/src" in "/" + rel or "/files" in "/" + rel or "/fixtures" in "/" + rel:
            dirs[:] = []
            continue
        for f in fs:
            if f.endswith((".py", ".ts", ".cjs", ".mjs", ".sh")):
                p = os.path.join(base, f)
                if os.path.islink(p):
                    continue
                try:
                    t = open(p, encoding="utf8").read()
                except Exception:
                    continue
                if LIVE in t:
                    os.unlink(p)
                    open(p, "w", encoding="utf8").write(t.replace(LIVE, SHIM))
                    n += 1
    return n


def main():
    if "--shim-only" in args:
        print("filled", fill_ticket(), "ignored ticket files; shimmed", shim(), "tool files")
        return
    shutil.rmtree(OV, ignore_errors=True)
    os.makedirs(OV)
    rows = files()
    print("cloned", copy_all(rows), "files of", len(rows))
    print("filled", fill_ticket(), "ignored ticket files")
    for name in ("node_modules",):
        if os.path.exists(os.path.join(LIVE, name)):
            os.symlink(os.path.join(LIVE, name), os.path.join(OV, name))
    print("shimmed", shim(), "tool files")
    env = {**os.environ, "GIT_DIR": os.path.join(OV, ".git"), "GIT_WORK_TREE": OV}
    subprocess.run(["git", "init", "-q"], env=env, cwd=OV)
    for key, value in (("core.quotepath", "off"), ("core.longpaths", "true"), ("core.symlinks", "true")):
        subprocess.run(["git", "config", key, value], env=env, cwd=OV)
    print(OV)


if __name__ == "__main__":
    main()
