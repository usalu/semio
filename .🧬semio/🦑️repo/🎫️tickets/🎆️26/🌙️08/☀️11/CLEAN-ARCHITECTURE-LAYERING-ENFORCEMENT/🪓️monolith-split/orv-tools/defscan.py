"""Definition scan on staged/installed files: every private top-level item, field or inherent method whose name is used by another file must already carry a visibility (dry run of the widening planner = 0 further edits).
usage: defscan.py [stage|real]"""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from rcrlib import *
from spec import SPEC
import split
from verify import files_of

which = sys.argv[1] if len(sys.argv) > 1 else "stage"
total = 0
for name, s in SPEC.items():
    t = TARGETS[name]
    base, root, kids = files_of(name, which)
    texts = {"<root>": read(root)}
    for f, p in kids:
        texts[f] = read(p)
    desc, _ = split.descendant_texts(name)
    ids = {k: split.file_ids(v) for k, v in texts.items()}
    dids = {p: split.file_ids(x) for p, x in desc.items()}
    n = 0
    for c in s["children"]:
        f = c["folder"]
        if c.get("widen") is False:
            continue
        child = dict(spec=c, body=texts[f])
        own = split.own_descendants(name, child)
        others = [v for k, v in ids.items() if k != f] + [v for p, v in dids.items() if p not in own]
        edits = split.plan_widen(texts[f], others)
        for e in edits:
            print("  PENDING", name, f, e[2])
        n += len(edits)
    total += n
    print(f"{name}: further widenings proposed = {n}")
print("TOTAL pending", total)
