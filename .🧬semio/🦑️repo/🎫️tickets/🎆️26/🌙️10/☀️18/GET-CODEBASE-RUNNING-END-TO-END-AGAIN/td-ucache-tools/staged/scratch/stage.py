#!/usr/bin/env python3
"""🏖️ Staged batch `scratch` (lane td-ucache): unit-cache tests that start Cargo create their scratch workspaces below `cargoScratchBaseV1`, outside every enclosing Cargo configuration, workspace and toolchain file.
usage: python3 -I stage.py check|apply|revert [--root DIR]   (default root: walk up from this file to the directory holding 🧰️framework)
check : exit 0 when every file is `base` or `applied`, 1 otherwise (prints each state).
apply : every live file must hold its base hash (absent for new files) or already the new one, else exit 2 (drift); atomic replace that keeps the mode of the file it replaces (a new file gets 0666 minus the umask, never the 0600 of `mkstemp`).
revert: every file must hold the new hash (or already base), else exit 2; restores base bytes, removes new files and the directories they emptied."""
import hashlib, json, os, stat, sys, tempfile
HERE = os.path.dirname(os.path.abspath(__file__))
def digest(path):
    if not os.path.exists(path): return None
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""): h.update(chunk)
    return h.hexdigest()
def root_of(start):
    here = start
    while not os.path.isdir(os.path.join(here, "🧰️framework")) and os.path.dirname(here) != here: here = os.path.dirname(here)
    return here
def umask():
    mask = os.umask(0); os.umask(mask); return mask
def atomic(target, data):
    os.makedirs(os.path.dirname(target), exist_ok=True)
    mode = stat.S_IMODE(os.stat(target).st_mode) if os.path.exists(target) else 0o666 & ~umask()
    fd, tmp = tempfile.mkstemp(dir=os.path.dirname(target), prefix=".stage-")
    with os.fdopen(fd, "wb") as f: f.write(data)
    os.chmod(tmp, mode)
    os.replace(tmp, target)
def main():
    verb = sys.argv[1] if len(sys.argv) > 1 else "check"
    root = os.path.abspath(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else root_of(HERE)
    rows = json.load(open(os.path.join(HERE, "manifest.json"), encoding="utf8"))
    state = []
    for rel, row in rows.items():
        live = digest(os.path.join(root, rel))
        state.append((rel, row, "applied" if live == row["new"] else "base" if live == row["base"] else "drifted"))
    if verb == "check":
        for rel, _row, kind in state: print(kind, rel[-90:])
        sys.exit(0 if all(k != "drifted" for *_x, k in state) else 1)
    if verb == "apply":
        if any(k == "drifted" for *_x, k in state):
            for rel, _row, kind in state:
                if kind == "drifted": print("DRIFTED", rel[-90:])
            sys.exit(2)
        for rel, row, kind in state:
            if kind != "applied": atomic(os.path.join(root, rel), open(os.path.join(HERE, "files", row["file"]), "rb").read()); print("applied", rel[-90:])
        return
    if verb == "revert":
        if any(k == "drifted" for *_x, k in state):
            for rel, _row, kind in state:
                if kind == "drifted": print("DRIFTED", rel[-90:])
            sys.exit(2)
        for rel, row, kind in state:
            if kind != "applied": continue
            target = os.path.join(root, rel)
            if row["base"] is None:
                os.remove(target); parent = os.path.dirname(target)
                while parent != root and os.path.isdir(parent) and not os.listdir(parent): os.rmdir(parent); parent = os.path.dirname(parent)
            else: atomic(target, open(os.path.join(HERE, "base", row["file"]), "rb").read())
            print("reverted", rel[-90:])
        return
    sys.exit("verb: check|apply|revert")
main()
