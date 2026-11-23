"""🪵️ The spreader stay of the painter's A-frame easel: a beech bar of 25 by 22 mm from the front legs to the back leg at the height of the centre of mass of the easel (0.85 m in the easel frame), as field easels have one to hold the spread (at 1:500 it keeps its true size and gets a 0.20 mm pen so that it prints). It is a body solid on the bone `easel` at every scale and a drawn prism of the same profile in the elevation, the front and the plan, so the mass marker of the easel lies in the body it belongs to. The drawn back leg is also moved by the translation that brought its body onto the mast (`bridge.ts`), so the drawing and the body are one bar again. Writes its ops into `repair/painter-at-easel.json` (replacing earlier ops of the same ids).
python3 easel-stay.py"""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/painter-at-easel/🔣️.json"
OPS = f"{HERE}/repair/painter-at-easel.json"
PROFILE = [[0.13, 0.8375], [0.645, 0.8375], [0.645, 0.8625], [0.13, 0.8625]]
HALF = 0.011

asset = json.load(open(CATALOG, encoding="utf8"))["asset"]
ops = [o for o in (json.load(open(OPS, encoding="utf8")) if os.path.exists(OPS) else []) if "easel-stay" not in json.dumps(o.get("solid", o.get("shape", {})).get("id", ""))]
added = []
for scale in ("50", "100", "200", "500"):
    body = asset["volumes"][scale]["body"]
    like = next((s for s in body if s["id"] in ("easel-brace", "easel-leg-back", "easel-mast") or s["bone"].startswith("easel")), None)
    if like is None:
        continue
    solid = {"id": "easel-stay", "kind": "extrude", "bone": "easel", "material": like["material"], "role": like["role"], "tone": like["tone"], "profile": PROFILE, "plane": "xy", "from": -HALF, "to": HALF, "slender": True}
    added.append({"op": "solid-add", "scale": scale, "list": "body", "after": like["id"], "solid": solid})
    for view in ("elevation", "front", "plan"):
        shapes = asset["representations"][scale]["geometry"][view]
        twin = next((s for s in shapes if s["bone"].startswith("easel") and s.get("solid", {}).get("kind") == "extruded" and s.get("filled", True)), None)
        if twin is None:
            continue
        shape = {k: v for k, v in twin.items() if k not in ("points", "d", "solid", "id", "bone", "kind", "closed")}
        shape.update({"id": "easel-stay", "bone": "easel", "kind": "poly", "points": PROFILE, "closed": True, "solid": {"kind": "extruded", "sweep": "w", "z0": -HALF, "z1": HALF}})
        if scale == "500":
            shape["penMm"] = 0.2
        added.append({"op": "shape-add", "scale": scale, "view": view, "after": twin["id"], "shape": shape, "manual": True})
for o in [o for o in ops if o["op"] == "solid-set" and o.get("id") == "easel-leg-back" and o.get("list", "body") == "body"]:
    tx, ty, _ = o["set"]["transform"]["translation"]
    for view in ("elevation", "front", "plan"):
        leg = next((s for s in asset["representations"][o["scale"]]["geometry"][view] if s["id"] == "easel-leg-back"), None)
        if leg is None or abs(min(p[0] for p in leg["points"])) > 1e-9:
            continue
        added.append({"op": "shape-set", "scale": o["scale"], "view": view, "id": "easel-leg-back", "set": {"points": [[round(p[0] + tx, 5), round(p[1] + ty, 5)] for p in leg["points"]]}, "manual": True})
ops = [o for o in ops if not (o["op"] == "shape-set" and o.get("id") == "easel-leg-back" and any(a["op"] == "shape-set" and a["scale"] == o["scale"] and a["view"] == o["view"] for a in added))]
volume = [o for o in ops if not o["op"].startswith("shape")] + [o for o in added if o["op"] == "solid-add"]
drawing = [o for o in added if o["op"].startswith("shape")] + [o for o in ops if o["op"].startswith("shape")]
with open(OPS, "w", encoding="utf8") as f:
    f.write("[\n" + ",\n".join(json.dumps(o, ensure_ascii=False) for o in volume + drawing) + "\n]\n")
print(f"easel stay: {len([o for o in added if o['op'] == 'solid-add'])} solids, {len([o for o in added if o['op'] == 'shape-add'])} shapes; back leg drawn where its body stands in {len([o for o in added if o['op'] == 'shape-set'])} views")
