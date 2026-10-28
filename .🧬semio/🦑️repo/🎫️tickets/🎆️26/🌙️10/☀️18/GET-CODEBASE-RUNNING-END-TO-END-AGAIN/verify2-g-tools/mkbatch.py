#!/usr/bin/env python3
"""`mkbatch.py <batch-dir> <work-dir> <rel-path>...`: `base/` = the live bytes now, `new/` = the same relative path under <work-dir>, then `bases.json` with both hashes plus the three wrapper scripts.
A path absent live gets base null (apply creates it, revert removes it); a path absent in <work-dir> is a deletion. Run it only while the live files are untouched."""
import json, os, shutil, sys
import sys as _s, os as _o; _s.path.insert(0, _o.path.dirname(_o.path.abspath(__file__)))
from stage import ROOT, digest

batch, work, rels = os.path.abspath(sys.argv[1]), os.path.abspath(sys.argv[2]), sys.argv[3:]
rows = {}
for rel in rels:
    live, new = os.path.join(ROOT, rel), os.path.join(work, rel)
    for side, source in (("base", live), ("new", new)):
        if os.path.exists(source):
            target = os.path.join(batch, side, rel)
            os.makedirs(os.path.dirname(target), exist_ok=True)
            shutil.copyfile(source, target)
    rows[rel] = {"base": digest(live), "new": digest(new)}
json.dump(rows, open(os.path.join(batch, "bases.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
here = os.path.dirname(os.path.abspath(__file__))
for verb in ("apply", "check", "revert"):
    path = os.path.join(batch, verb + ".sh")
    open(path, "w").write('#!/bin/sh\nexec python3 -I "%s/stage.py" "$(dirname "$0")" %s\n' % (here, verb))
    os.chmod(path, 0o755)
print(json.dumps({k: [v["base"] and v["base"][:10], v["new"] and v["new"][:10]] for k, v in rows.items()}))
