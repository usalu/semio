#!/usr/bin/env python3
"""Copies the `new/` bytes of staged batches into a mirror root: `overlay-batch.py <mirror-root> <batch-dir>...` (base check against the mirror, drift = exit 2)."""
import hashlib, json, os, shutil, sys
root = sys.argv[1]
def sha(p):
    return hashlib.sha256(open(p, "rb").read()).hexdigest() if os.path.exists(p) else None
for batch in sys.argv[2:]:
    rows = json.load(open(os.path.join(batch, "bases.json"), encoding="utf8"))
    for rel, row in rows.items():
        target = os.path.join(root, rel)
        live = sha(target)
        if live == row["new"]:
            continue
        if live != row["base"]:
            print("DRIFT", rel, file=sys.stderr)
            sys.exit(2)
        if row["new"] is None:
            os.remove(target)
            continue
        os.makedirs(os.path.dirname(target), exist_ok=True)
        tmp = target + ".ov"
        shutil.copyfile(os.path.join(batch, "new", rel), tmp)
        os.replace(tmp, target)
        print("applied", rel[-60:])
