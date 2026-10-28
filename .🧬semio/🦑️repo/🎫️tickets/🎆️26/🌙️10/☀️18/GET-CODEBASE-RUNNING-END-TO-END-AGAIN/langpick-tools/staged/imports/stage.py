#!/usr/bin/env python3
"""🗣️ Staged batch of lane langpick (one language protocol): see `batch.txt` next to this file for what it changes.
usage: python3 -I stage.py check|apply|revert [ROOT | --root ROOT]   (default root: walk up from this file to the directory holding 🧰️framework)
check : exit 0 when every file is `base` or `applied`, 1 otherwise (prints each state).
apply : every live file must hold its base hash (absent for new files) or already the new one, else exit 2 (drift); atomic replace that keeps the mode of the file it replaces (a new file gets 0666 minus the umask, never the 0600 of `mkstemp`); `new: null` removes the file and the directories it emptied.
revert: every file must hold the new hash (or already base), else exit 2; restores base bytes with the mode the file had, removes new files and the directories they emptied."""
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
def atomic(target, data, mode=None):
    os.makedirs(os.path.dirname(target), exist_ok=True)
    mode = mode if mode is not None else stat.S_IMODE(os.stat(target).st_mode) if os.path.exists(target) else 0o666 & ~umask()
    fd, tmp = tempfile.mkstemp(dir=os.path.dirname(target), prefix=".stage-")
    with os.fdopen(fd, "wb") as f: f.write(data)
    os.chmod(tmp, mode)
    os.replace(tmp, target)
def remove(root, target):
    os.remove(target); parent = os.path.dirname(target)
    while parent != root and os.path.isdir(parent) and not os.listdir(parent): os.rmdir(parent); parent = os.path.dirname(parent)
def main():
    verb = sys.argv[1] if len(sys.argv) > 1 else "check"
    rest = [a for a in sys.argv[2:] if a != "--root"]
    root = os.path.abspath(rest[0]) if rest else root_of(HERE)
    rows = json.load(open(os.path.join(HERE, "manifest.json"), encoding="utf8"))
    state = []
    for rel, row in rows.items():
        live = digest(os.path.join(root, rel))
        state.append((rel, row, "applied" if live == row["new"] else "base" if live == row["base"] else "drifted"))
    drifted = [rel for rel, _row, kind in state if kind == "drifted"]
    if verb == "check":
        for rel, _row, kind in state: print(kind, rel[-90:])
        sys.exit(1 if drifted else 0)
    if verb not in ("apply", "revert"): sys.exit("verb: check|apply|revert")
    if drifted:
        for rel in drifted: print("DRIFTED", rel[-90:])
        sys.exit(2)
    modes = json.load(open(os.path.join(HERE, "modes.json"), encoding="utf8")) if os.path.exists(os.path.join(HERE, "modes.json")) else {}
    for rel, row, kind in state:
        target = os.path.join(root, rel)
        if verb == "apply" and kind != "applied":
            if row["new"] is None: remove(root, target); print("removed", rel[-90:])
            else: atomic(target, open(os.path.join(HERE, "files", row["file"]), "rb").read()); print("applied", rel[-90:])
        if verb == "revert" and kind == "applied" and row["new"] != row["base"]:
            if row["base"] is None: remove(root, target); print("removed", rel[-90:])
            else: atomic(target, open(os.path.join(HERE, "base", row["file"]), "rb").read(), int(modes[rel], 8) if rel in modes and not os.path.exists(target) else None); print("reverted", rel[-90:])
main()
