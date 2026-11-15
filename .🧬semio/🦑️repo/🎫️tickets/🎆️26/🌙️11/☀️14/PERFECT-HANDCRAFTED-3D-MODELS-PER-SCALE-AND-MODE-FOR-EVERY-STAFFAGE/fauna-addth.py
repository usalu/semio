"""🌡️ Appends thermal references `{ id: th-<id>, of: <id>, region }` for new body solids (idempotent: replaces earlier entries of the same ids).
Usage: python3 fauna-addth.py <asset> <scale> <region>=<id>+<id>[,<region>=...]"""
import json, sys
asset, scale, spec = sys.argv[1], sys.argv[2], sys.argv[3]
f = f"volumes/{asset}.json"
d = json.load(open(f))
th = d["volumes"][scale]["thermal"]
body = {b["id"] for b in d["volumes"][scale]["body"]}
for part in spec.split(","):
    region, ids = part.split("=")
    for i in ids.split("+"):
        if i not in body: print("missing body", i); continue
        th[:] = [t for t in th if t.get("of") != i]
        th.append({"id": f"th-{i}", "of": i, "region": region})
json.dump(d, open(f, "w"), separators=(",", ":"), ensure_ascii=False)
print("thermal refs", len(th))
