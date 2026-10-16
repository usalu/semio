"""union oracle + autowiden loop. usage: converge.py <crate-id> [max-iterations] [cfgs]"""
import json, os, sys, io, contextlib
sys.path.insert(0, os.path.dirname(__file__)); sys.path.insert(0, os.path.dirname(__file__) + "/../frs-tools")
import union, autowiden
from autowiden import ROOT

cid = sys.argv[1]
maxit = int(sys.argv[2]) if len(sys.argv) > 2 else 8
cfgs = tuple(sys.argv[3].split(",")) if len(sys.argv) > 3 else ("test",)
cfg = union.cfg_of(cid)
for it in range(maxit):
    buf = io.StringIO()
    with contextlib.redirect_stdout(buf):
        res, A, B = union.run(cid, cfgs)
    text = buf.getvalue()
    new_total = 0
    for cn in cfgs:
        ea, eb, onlyA, onlyB = res[cn]
        newkeys = set(onlyB)
        for d in eb:
            d["__new"] = union.key_of(d, B)[0] in newkeys
        diags = eb
        prod_files = set()
        for d in cfg["p_dirs"]:
            for dp, dn, fn in os.walk(ROOT + "/" + d):
                if "node_modules" in dp: continue
                for x in fn:
                    if x.endswith(".rs"): prod_files.add(os.path.relpath(os.path.join(dp, x), ROOT))
        for c in cfg["comps"]:
            cdir = os.path.dirname(c["lib"])
            prod_files = {p for p in prod_files if not p.startswith(cdir + "/")}
        import re as _re
        base_unres = {m.group(1) for d in ea for m in [_re.match(r"cannot find [\w, ]+ `(\w+)` in ", d["message"])] if m}
        log = autowiden.apply(cid, diags, prod_files, True, base_unres)
        edits = [l for l in log if l.startswith("EDIT")]
        new_total += len(edits)
        print(f"-- iteration {it} cfg {cn}: baseline={sum(1 for _ in ea)} current={sum(1 for _ in eb)} new-kinds={sum(onlyB.values())} edits={len(edits)}")
        for l in log:
            if not l.startswith("EDIT"):
                print("   ", l[:200])
    if new_total == 0:
        print(text)
        break
