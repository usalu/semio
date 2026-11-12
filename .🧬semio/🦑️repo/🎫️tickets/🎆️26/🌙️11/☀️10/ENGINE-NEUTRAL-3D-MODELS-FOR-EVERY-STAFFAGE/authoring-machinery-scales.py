"""🔭️ Lists per machinery asset which 1:50 shape ids are no longer drawn at 1:100, 1:200 and 1:500 (guides `maxDenominator`)."""
import glob, json, os, sys
root = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "..", "..", "..", "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog"))
cat = [d for d in glob.glob(os.path.join(root, "*")) if d.endswith("machinery")][0]
out = []
for slug in sys.argv[1:]:
    a = json.load(open(os.path.join(cat, slug, "🔣️.json"))); a = a.get("asset", a)
    reps = a["representations"]
    ids = {}
    for d in ["50", "100", "200", "500"]:
        g = reps[d].get("geometry")
        ids[d] = None if g is None else {x["id"] for v in g for x in g[v]}
    out.append(f"## {slug}  status " + ", ".join(f"{d}:{reps[d]['status']}" for d in ids))
    prev = "50"
    for d in ["100", "200", "500"]:
        if ids[prev] is not None and ids[d] is not None:
            out.append(f"  dropped {prev}->{d}: " + ", ".join(sorted(ids[prev] - ids[d])))
        if ids[d] is not None:
            prev = d
print("\n".join(out))
