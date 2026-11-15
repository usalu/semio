"""Reviewer 3: tabulates 🗑️generated/areareview3/measure.json (declared against measured outer area). Usage: python3 areareview3-table.py [--all] [slug ...]"""
import json, os, sys
T = os.path.dirname(os.path.abspath(__file__))
M = json.load(open(os.path.join(T, "🗑️generated", "areareview3", "measure.json")))
args = [a for a in sys.argv[1:] if not a.startswith("--")]
for slug, regs in M.items():
    if args and slug not in args: continue
    if "_error" in regs: print(slug, "ERROR", regs["_error"]); continue
    rows = []
    for rid, r in regs.items():
        if rid.startswith("_"): continue
        o, dcl = r.get("outer"), r.get("decl")
        if o is None:
            rows.append((rid, dcl, None, None, r)); continue
        rows.append((rid, dcl, o, o / dcl if dcl else None, r))
    bad = [x for x in rows if x[2] is None or (x[3] is not None and not (0.75 <= x[3] <= 1.3))]
    if "--all" in sys.argv or args or bad:
        print(f"## {slug}  findings={regs.get('_findings')}")
        for rid, dcl, o, ratio, r in rows:
            fl = "" if (ratio and 0.75 <= ratio <= 1.3) else " <<"
            if not ("--all" in sys.argv or args) and not fl: continue
            print(f"  {rid:24} {r.get('kind')}/{r.get('model')} {r.get('basis') or ''} decl={dcl} outer={None if o is None else round(o,4)} ratio={None if ratio is None else round(ratio,2)} expM={None if r.get('expM') is None else round(r['expM'],2)}/{r.get('expT')}{fl}")
