"""🔎️ The assets whose flat front or plan shapes can change with MV-L1: an elevation `loft` or `solid` stands on a bone that also carries a flat front or plan shape (any scale): affected.py <catalog root> -> `id<TAB>scales` per affected asset and the count of the others."""
import glob, json, sys
root = sys.argv[1]
affected, other = [], 0
for f in sorted(glob.glob(root + "/*/*/🔣️.json")):
    asset = json.load(open(f))["asset"]
    hit = {}
    for key, rep in asset.get("representations", {}).items():
        g = rep.get("geometry") if isinstance(rep, dict) else None
        if not g: continue
        deep = {s["bone"] for s in g.get("elevation", []) if s.get("solid") is not None or s.get("kind") == "loft"}
        flat = {s["bone"] for v in ("front", "plan") for s in g.get(v, []) if s.get("solid") is None and s.get("kind") not in ("loft", "member")}
        if deep & flat: hit[key] = sorted(deep & flat)
    if hit: affected.append((asset["id"], f.split("/")[-3], hit))
    else: other += 1
for id_, cat, hit in affected: print(id_, cat, " ".join(f"{k}:{len(v)}" for k, v in hit.items()))
print("affected", len(affected), "other", other)
