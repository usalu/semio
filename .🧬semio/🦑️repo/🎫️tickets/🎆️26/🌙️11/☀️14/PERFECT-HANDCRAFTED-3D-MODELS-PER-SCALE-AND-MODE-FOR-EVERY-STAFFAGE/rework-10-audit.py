"""🔎 Audit of one asset of author 10: rig table, poses, clips, and per scale the solids that are identical to the 1:50 list or thinner than the scale's smallest feature.

Usage: python3 rework-10-audit.py <slug> [--rig] [--thin]
"""
import json, sys, glob, math
slug = sys.argv[1]
asset = json.load(open(glob.glob(f"/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/{slug}/*.json")[0]))["asset"]
V = json.load(open(f"volumes/machinery/{slug}.json"))["volumes"]
FEATURE = {"50": 0.0075, "100": 0.015, "200": 0.03, "500": 0.075}
if "--rig" in sys.argv:
    print("dimensions", asset["dimensions"])
    for b in asset["rig"]["bones"]:
        print(" ", b["id"], "<-", b["parent"], b["x"], b["y"], b["angleDeg"], "L", b["lengthM"], {k: v for k, v in b["joint"].items()})
    print("poses", [p["id"] for p in asset["poses"]], "clips", [(c["id"], c["durationS"]) for c in asset["clips"]], "default", asset["presentation"])
    print("regions", [(r["id"], r.get("areaM2")) for r in asset["thermal"]["regions"]])
    print("structure", json.dumps(asset["structure"])[:700])


def dims(s):
    k = s["kind"]
    if k == "box":
        return list(s["size"])
    if k == "ellipsoid":
        return [2 * r for r in s["radii"]]
    if k == "capsule":
        return [2 * max(s["r0"], s["r1"])]
    if k == "revolve":
        p = s["profile"]
        closed = p[0] == p[-1]
        ax = [q[0] for q in p]
        rad = [q[1] for q in p]
        if closed:
            return [max(ax) - min(ax), max(rad) - min(rad)]
        return [max(ax) - min(ax), 2 * max(rad)]
    if k == "extrude":
        p = s["profile"]
        return [s["to"] - s["from"], max(q[0] for q in p) - min(q[0] for q in p), max(q[1] for q in p) - min(q[1] for q in p)]
    if k == "loft":
        return [2 * min(min(c["rx"], c["ry"]) for c in s["sections"] if c["rx"] > 0 and c["ry"] > 0)] if any(c["rx"] > 0 for c in s["sections"]) else [1]
    return [1]


fine = {s["id"]: json.dumps({k: v for k, v in s.items() if k not in ("tone", "role")}, sort_keys=True) for s in V["50"]["body"]}
for scale in ("50", "100", "200", "500"):
    vol = V[scale]
    body = vol["body"]
    same = [s["id"] for s in body if scale != "50" and fine.get(s["id"]) == json.dumps({k: v for k, v in s.items() if k not in ("tone", "role")}, sort_keys=True)]
    thin = [(s["id"], round(min(dims(s)) * 1000, 1)) for s in body if min(dims(s)) < FEATURE[scale] - 1e-9 and s.get("role") not in ("glazing", "texture", "ground") and not s.get("slender") and not s.get("texture")]
    print(f"1:{scale} body {len(body)} thermal {len(vol['thermal'])} structure {len(vol['structure'])}; identical to 1:50: {len(same)}; under feature: {len(thin)}")
    if "--thin" in sys.argv:
        print("   same:", " ".join(same))
        print("   thin:", thin)
