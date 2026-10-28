"""⚖️ [audit5] Compares two replay outputs (rows dumps + projects) modulo the D1 implementation files themselves: rows whose path is a file any D1 batch modified or added are dropped (the library is an input of its own graph), analysis revisions are normalised, and in the projects JSON `SEMIO_SOURCE_CLOSURE_<digest>` names are normalised. usage: python3 -I d1cmp.py <labelA> <labelB>"""
import json, os, re, sys
C = os.path.dirname(os.path.dirname(os.path.abspath(__file__))); G = C + "/🗑️generated/audit5"
d1 = set()
for b in ["g-walk-walks", "g-gram2-row-table", "g-gram3-index-chunks", "g-gram3-churn", "g-gram4-shape-keys"]:
    d1 |= set(json.load(open(f"{C}/backlog-tools/backup/{b}@live/manifest.json", encoding="utf8")))
def rows(label, name):
    out = []
    for line in open(f"{G}/{label}.rows/{name}", encoding="utf8"):
        if not line.strip(): continue
        r = json.loads(line); rel = r[1].split("/sm/a5/", 1)[-1]
        if rel in d1: continue
        if r[0] == "analysis": f = dict(r[2]); f["provider"] = re.sub(r":[0-9a-f]{64}$", ":rev", str(f.get("provider"))); f["revision"] = "rev"; r = [r[0], r[1], f]
        out.append(json.dumps(r, sort_keys=True))
    return out
a, b = sys.argv[1:3]; bad = 0
for name in sorted(set(os.listdir(f"{G}/{a}.rows")) | set(os.listdir(f"{G}/{b}.rows"))):
    ra, rb = rows(a, name), rows(b, name)
    first = next((i for i, (x, y) in enumerate(zip(ra, rb)) if x != y), None)
    same = ra == rb; bad += not same
    print(f"{name}: {len(ra)} vs {len(rb)} rows (D1 files excluded)", "IDENTICAL" if same else f"DIFFER at {first}: {ra[first][:200] if first is not None else ''} | {rb[first][:200] if first is not None else ''}")
norm = lambda t: re.sub(r"SEMIO_SOURCE_CLOSURE_[0-9a-f]{64}", "SEMIO_SOURCE_CLOSURE_x", t)
pa, pb = norm(open(f"{G}/{a}.projects.json", encoding="utf8").read()), norm(open(f"{G}/{b}.projects.json", encoding="utf8").read())
print("projects (closure digests normalised):", "IDENTICAL" if pa == pb else "DIFFER"); bad += pa != pb
sys.exit(1 if bad else 0)
