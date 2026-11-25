"""🧮️ The records of matrix run folders of lane MU: `python3 table.py [--detail] [--md] <dir>…` prints per asset the cells by status and, for an asset with fail cells, the failing checks with the number of cells, scale entries and views, and the worst subject. A later folder replaces the record of an earlier one."""
import json, sys, glob, collections, os
detail = "--detail" in sys.argv; md = "--md" in sys.argv; only = "--fail" in sys.argv
rec = {}
for d in [a for a in sys.argv[1:] if not a.startswith("--")]:
    for f in sorted(glob.glob(d + "/cells/*.json")):
        rec[os.path.basename(f)[:-5]] = json.load(open(f))
bad = 0; tot = collections.Counter()
for a in sorted(rec):
    r = rec[a]; cells = r.get("cells") or []
    st = collections.Counter(c["status"] for c in cells); tot.update(st)
    if st.get("fail"): bad += 1
    elif only: continue
    codes = collections.Counter()
    for c in cells:
        for k in (c.get("failures") or {}): codes[k] += 1
    s = ", ".join(f"{n} {k}" for k, n in sorted(st.items())); t = ", ".join(f"{k} {n}" for k, n in codes.most_common())
    print(f"| `{a}` | {r.get('outcome')} | {s} | {t} |" if md else f"{a}: {r.get('outcome')} {s} :: {t}")
    if detail and st.get("fail"):
        by = collections.defaultdict(lambda: collections.defaultdict(list))
        for c in cells:
            for k, f in (c.get("failures") or {}).items(): by[k][(c["scale"], c["view"])].append(f)
        for k, g in sorted(by.items()):
            for (sc, v), fs in sorted(g.items()):
                f = fs[0]; w = f.get("worst") or {}; first = (f.get("first") or [{}])[0]
                print(f"     {k} {sc:>13} {v:<9} ×{len(fs)} n={f.get('count')} worst {w.get('deltaMm', 0):.1f} mm {w.get('pair', '')} @ {w.get('subject', first.get('subject', ''))} :: {first.get('message', '')[:170]}")
print(len(rec), "records,", bad, "with fail cells,", dict(tot))
