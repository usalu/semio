#!/usr/bin/env python3
"""🎬 One staged flat batch of lane hotpath: `python3 -I stage.py <batch-dir> check|apply|revert [ROOT]` (ROOT default: walk up to the directory holding 🧰️framework).
check : exit 0 when every file is `base` or `applied`, 1 otherwise (prints each state).
apply : every live file must hold its base hash (absent for new files) or already the new one, else exit 2 (drift); atomic replace.
revert: every file must hold the new hash (or already base), else exit 2; restores base bytes, removes new files and the directories they emptied."""
import hashlib, json, os, sys, tempfile


def digest(path):
    if not os.path.exists(path):
        return None
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def root_of(start):
    here = start
    while not os.path.isdir(os.path.join(here, "🧰️framework")) and here != "/":
        here = os.path.dirname(here)
    return here


def atomic(target, data):
    os.makedirs(os.path.dirname(target), exist_ok=True)
    fd, tmp = tempfile.mkstemp(dir=os.path.dirname(target), prefix=".stage-")
    with os.fdopen(fd, "wb") as f:
        f.write(data)
    os.replace(tmp, target)


def main():
    batch, verb = os.path.abspath(sys.argv[1]), sys.argv[2]
    root = os.path.abspath(sys.argv[3]) if len(sys.argv) > 3 else root_of(batch)
    rows = json.load(open(os.path.join(batch, "manifest.json"), encoding="utf8"))
    state = []
    for rel, row in rows.items():
        live = digest(os.path.join(root, rel))
        state.append((rel, row, "applied" if live == row["new"] else "base" if live == row["base"] else "drifted"))
    drifted = [rel for rel, _row, kind in state if kind == "drifted"]
    if verb == "check":
        for rel, _row, kind in state:
            print(kind, rel[-90:])
        sys.exit(1 if drifted else 0)
    if verb not in ("apply", "revert"):
        sys.exit("verb: check|apply|revert")
    if drifted:
        for rel in drifted:
            print("DRIFTED", rel[-90:])
        sys.exit(2)
    for rel, row, kind in state:
        target = os.path.join(root, rel)
        if verb == "apply" and kind != "applied":
            atomic(target, open(os.path.join(batch, "files", row["file"]), "rb").read())
            print("applied", rel[-90:])
        elif verb == "revert" and kind == "applied":
            if row["base"] is None:
                os.remove(target)
                parent = os.path.dirname(target)
                while parent != root and os.path.isdir(parent) and not os.listdir(parent):
                    os.rmdir(parent)
                    parent = os.path.dirname(parent)
            else:
                atomic(target, open(os.path.join(batch, "base", row["file"]), "rb").read())
            print("reverted", rel[-90:])


main()
