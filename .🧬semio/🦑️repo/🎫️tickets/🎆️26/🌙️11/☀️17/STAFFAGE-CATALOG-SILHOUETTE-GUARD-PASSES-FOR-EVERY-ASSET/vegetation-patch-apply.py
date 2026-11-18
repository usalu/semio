"""Inserts the patch polygons of vegetation-patch.ts: python3 vegetation-patch-apply.py <slug> <patches.json>  (writes the edits for the refinement of vegetation-refine.ts)"""
import sys, json, importlib
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
mv = importlib.import_module("mvlib")
HERE = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET/vegetation-work/"
slug = sys.argv[1]
d = mv.D("🌳️vegetation/" + slug); a = d.val("asset")
def parse(rings): return [[tuple(float(v) for v in p.split(" ")) for p in r.split(";")] for r in rings]
def pts(r): return [[round(x, 4), round(y, 4)] for x, y in r]
edits = []
for p in json.load(open(sys.argv[2])):
    geo = a["representations"][p["scale"]]["geometry"][p["view"]]
    zs = [s["z"] for s in geo]
    rings = parse(p["rings"])
    for k, r in enumerate(rings):
        sid = p["id"] + ("" if len(rings) == 1 else "-" + str(k))
        new = {"id": sid, "bone": p["bone"], "role": p["role"], "z": round(max(zs) + 0.01, 2), "filled": True, "tone": p["tone"], "kind": "poly", "points": pts(r), "closed": True}
        have = next((s for s in geo if s["id"] == sid), None)
        if have is not None: d.replace_obj(d.shape(p["scale"], p["view"], sid), new)
        else:
            last = [s for s in geo if s["bone"] == p["bone"]] or geo
            d.insert_after(d.shape(p["scale"], p["view"], last[-1]["id"]), new)
        edits.append({"scale": p["scale"], "view": p["view"], "id": sid, "bone": p["bone"], "target": [p["target"][k]]})
cuts = []
try: cuts = json.load(open(sys.argv[2].replace(".json", "-cuts.json")))
except Exception: pass
KEYS = ("d", "points", "closed", "x", "y", "w", "h", "r", "cx", "cy", "rx", "ry", "x0", "y0", "x1", "y1", "r0", "r1", "rotationDeg")
for c in cuts:
    geo = a["representations"][c["scale"]]["geometry"][c["view"]]
    shape = next((s for s in geo if s["id"] == c["id"] and s["bone"] == c["bone"]), None)
    if shape is None or not c["rings"]: continue
    rings = parse(c["rings"]); tgt = c["target"]
    if shape.get("followTip") is True and len(rings) > 1: rings = rings[:1]; tgt = tgt[:1]
    node = d.shape(c["scale"], c["view"], c["id"])
    o = {k: v for k, v in shape.items() if k not in KEYS}
    if len(rings) == 1: o["kind"] = "poly"; o["points"] = pts(rings[0]); o["closed"] = True
    else: o["kind"] = "path"; o["d"] = " ".join("M " + " L ".join(("%.4f %.4f" % (x, y)).replace("-0.0000", "0") for x, y in r) + " Z" for r in rings)
    order = [k for k in shape if k in o] + [k for k in o if k not in shape]
    d.replace_obj(node, {k: o[k] for k in order})
    edits.append({"scale": c["scale"], "view": c["view"], "id": c["id"], "bone": c["bone"], "target": tgt})
d.commit(); json.dump(edits, open(HERE + "edits-" + slug + ".json", "w")); print("patched", len(edits))
