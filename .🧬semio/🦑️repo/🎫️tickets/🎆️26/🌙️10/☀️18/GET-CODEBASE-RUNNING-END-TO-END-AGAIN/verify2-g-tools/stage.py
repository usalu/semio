#!/usr/bin/env python3
"""Generic apply/check/revert of a staged batch: `stage.py <batch-dir> apply|check|revert|status`.

A batch dir holds `bases.json` {relative path: {"base": sha256|null, "new": sha256|null}} plus `base/<path>` and `new/<path>` byte copies.
apply  : every live file must still hold the base hash (or already the new hash), then it is replaced by the new bytes (atomic rename).
check  : exit 0 when every live file holds the base hash (applicable) or the new hash (applied); prints one line per file.
revert : every live file must hold the new hash, then it is replaced by the base bytes (or removed when base is null).
Paths are relative to the repository root, found by walking up from this file until `🧰️framework` exists."""
import hashlib, json, os, sys, tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = HERE
while not os.path.isdir(os.path.join(ROOT, "🧰️framework")) and ROOT != "/":
    ROOT = os.path.dirname(ROOT)

def digest(path):
    if not os.path.exists(path):
        return None
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()

def state(batch):
    rows = json.load(open(os.path.join(batch, "bases.json"), encoding="utf8"))
    out = []
    for rel, row in rows.items():
        live = digest(os.path.join(ROOT, rel))
        kind = "applied" if live == row["new"] else "base" if live == row["base"] else "drifted"
        out.append((rel, row, live, kind))
    return out

def write(path, data):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    fd, tmp = tempfile.mkstemp(dir=os.path.dirname(path), prefix=".stage-")
    with os.fdopen(fd, "wb") as f:
        f.write(data)
    os.replace(tmp, path)

def main():
    batch, verb = os.path.abspath(sys.argv[1]), sys.argv[2]
    rows = state(batch)
    if verb in ("check", "status"):
        for rel, _row, live, kind in rows:
            print(kind, rel, (live or "absent")[:12])
        sys.exit(0 if all(k in ("base", "applied") for _r, _w, _l, k in rows) else 1)
    if verb == "apply":
        bad = [r for r in rows if r[3] == "drifted"]
        if bad:
            for rel, row, live, _k in bad:
                print("DRIFTED", rel, "live", (live or "absent")[:12], "base", (row["base"] or "absent")[:12])
            sys.exit(2)
        for rel, row, _l, kind in rows:
            if kind == "applied":
                continue
            target = os.path.join(ROOT, rel)
            if row["new"] is None:
                os.remove(target)
            else:
                write(target, open(os.path.join(batch, "new", rel), "rb").read())
            print("applied", rel)
    elif verb == "revert":
        bad = [r for r in rows if r[3] == "drifted"]
        if bad:
            for rel, row, live, _k in bad:
                print("DRIFTED", rel, "live", (live or "absent")[:12], "new", (row["new"] or "absent")[:12])
            sys.exit(2)
        for rel, row, _l, kind in rows:
            if kind == "base":
                continue
            target = os.path.join(ROOT, rel)
            if row["base"] is None:
                os.remove(target)
            else:
                write(target, open(os.path.join(batch, "base", rel), "rb").read())
            print("reverted", rel)
    else:
        sys.exit("usage: stage.py <batch-dir> apply|check|revert|status")

if __name__ == "__main__":
    main()
