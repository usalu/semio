"""Independent visibility audit on the INSTALLED tree: private definitions of a child that another file of the crate still references."""
import os, sys, json
sys.path.insert(0, os.path.dirname(__file__))
from splitter import *

for name in sys.argv[1:]:
    spec_plan = Plan.__new__(Plan)
    t = TARGETS[name]
    inst = json.loads(read(f"{W}/{name}/installed.json"))["files"]
    tree = os.path.join(ROOT, t["tree"])
    all_rs = []
    for dp, dn, fn in os.walk(tree):
        all_rs.extend(os.path.join(dp, f) for f in fn if f.endswith(".rs"))
    texts = {p: read(p) for p in all_rs}
    bad = 0
    for rel in inst:
        child = os.path.join(ROOT, rel)
        if rel == t["file"]:
            continue
        others = [p for p in all_rs if os.path.normpath(p) != os.path.normpath(child)]
        ea, ef, em = set(), set(), set()
        for p in others:
            f, a = tokenize(texts[p]); ea |= a
            fl, me = member_refs(texts[p]); ef |= fl; em |= me
        body = lines_of(texts[child])
        edits = spec_plan.widen_edits(body, ea, ef, em)
        print(f"[{name}] {rel.split('/')[-2]}: private-but-referenced-elsewhere={len(edits)}", [e[2] for e in edits][:20])
        bad += len(edits)
    print(f"[{name}] AUDIT", "CLEAN" if bad == 0 else f"{bad} NOT WIDENED")
