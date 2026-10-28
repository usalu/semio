#!/usr/bin/env python3
"""📦 Flat staged batch `envleak`: `python3 -I stage.py <check|apply|revert|status> [--root DIR]` (or STAGE_ROOT=DIR; default: the repository above this file).
manifest.json = {rel: {file, base, new}} (sha256, null = absent); `staged/files/<file>` = the new bytes, `staged/base/<file>` = the live bytes the batch was cut from.
check : exit 0 when every file is `base` or `applied`, else 1.   apply : every file must be base (or already applied), replaced atomically; exit 2 on drift.
revert: every applied file is replaced by its base bytes (or removed); exit 2 on drift."""
import hashlib, json, os, sys, tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
BATCH = os.path.join(HERE, "staged")

def digest(path):
    if not os.path.exists(path): return None
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""): h.update(chunk)
    return h.hexdigest()

def repository(start):
    here = os.path.abspath(start)
    while not os.path.isdir(os.path.join(here, "🧰️framework")) and here != os.path.dirname(here): here = os.path.dirname(here)
    return here

def atomic(target, data):
    os.makedirs(os.path.dirname(target), exist_ok=True)
    fd, tmp = tempfile.mkstemp(dir=os.path.dirname(target), prefix=".stage-")
    with os.fdopen(fd, "wb") as f: f.write(data)
    os.replace(tmp, target)

def main():
    args = sys.argv[1:]
    root = os.environ.get("STAGE_ROOT")
    if "--root" in args:
        i = args.index("--root"); root = args[i + 1]; del args[i:i + 2]
    verb = args[0] if args else "status"
    root = os.path.abspath(root) if root else repository(HERE)
    rows = json.load(open(os.path.join(BATCH, "manifest.json"), encoding="utf8"))
    state = []
    for rel, row in rows.items():
        live = digest(os.path.join(root, rel))
        state.append((rel, row, live, "applied" if live == row["new"] else "base" if live == row["base"] else "drifted"))
    if verb in ("check", "status"):
        for rel, _row, live, kind in state: print(kind, rel[-90:], (live or "absent")[:12])
        sys.exit(0 if all(k in ("base", "applied") for *_x, k in state) else 1)
    bad = [r for r in state if r[3] == "drifted"]
    if bad:
        for rel, row, live, _k in bad: print("DRIFTED", rel[-90:], "live", (live or "absent")[:12], "base", (row["base"] or "absent")[:12], "new", (row["new"] or "absent")[:12])
        sys.exit(2)
    if verb == "apply":
        for rel, row, _l, kind in state:
            if kind == "applied": continue
            atomic(os.path.join(root, rel), open(os.path.join(BATCH, "files", row["file"]), "rb").read()); print("applied", rel[-90:])
    elif verb == "revert":
        for rel, row, _l, kind in state:
            if kind != "applied": continue
            target = os.path.join(root, rel)
            if row["base"] is None:
                os.remove(target)
                parent = os.path.dirname(target)
                while parent != root and not os.listdir(parent): os.rmdir(parent); parent = os.path.dirname(parent)
            else: atomic(target, open(os.path.join(BATCH, "base", row["file"]), "rb").read())
            print("reverted", rel[-90:])
    else:
        sys.exit("verb: check|apply|revert|status")
main()
