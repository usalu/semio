"""🦾️ Further repair ops of `excavator` (merged into ops/excavator.json after excavator.sh): parts of the body of 1:50 and 1:100 that no view drew, each as an extrusion over the lateral range of its solid on the bone of the solid (the idiom of this asset, so nothing flat rides on a turning bone): the bucket lip, the two cab lights, the pin of the idler link, and in plan and front the root boss of the stick. At 1:100 the bucket lip is drawn in the side elevation only (in front and plan the print profile finds 0.192 mm of it against 0.2 mm) and pin and boss in plan only (in the front elevation they raised the largest distance from 78.8 to 93.8 mm)."""
import json, os
import oplib

asset = oplib.load("excavator")
path = os.path.join(oplib.HERE, "..", "ops", "excavator.json")
ops = json.load(open(path))


def shapes_of(s):
    """🧩️ The extruded shapes of one solid: an xy extrude, a revolve about the lateral axis, or an unrotated box, with its repeats along the lateral axis."""
    base = {"bone": s["bone"], "role": "detail", "filled": True, "tone": s.get("tone", 0.4), "material": s["material"]}
    if s["kind"] == "extrude" and s.get("plane") == "xy":
        one = dict(base, kind="path", d="M " + " L ".join(f"{x},{y}" for x, y in s["profile"]) + " Z")
        lo, hi = s["from"], s["to"]
    elif s["kind"] == "revolve" and [abs(v) for v in s["axis"]] == [0, 0, 1]:
        r, n = max(p[1] for p in s["profile"]), max(p[0] for p in s["profile"])
        ox, oy, oz = s["origin"]
        one = dict(base, kind="ellipse", cx=ox, cy=oy, rx=r, ry=r)
        lo, hi = sorted((oz, oz + n * s["axis"][2]))
    elif s["kind"] == "box" and "rotation" not in s.get("transform", {}):
        (sx, sy, sz), (tx, ty, tz) = s["size"], s["transform"]["translation"]
        one = dict(base, kind="path", d=f"M {round(tx - sx / 2, 4)},{round(ty - sy / 2, 4)} L {round(tx + sx / 2, 4)},{round(ty - sy / 2, 4)} L {round(tx + sx / 2, 4)},{round(ty + sy / 2, 4)} L {round(tx - sx / 2, 4)},{round(ty + sy / 2, 4)} Z")
        lo, hi = tz - sz / 2, tz + sz / 2
    else:
        raise SystemExit(f"{s['id']}: kind {s['kind']} not supported")
    rep = s.get("repeat", {"count": 1, "translation": [0, 0, 0]})
    return [dict(one, id=s["id"] if k == 0 else f"{s['id']}-{k + 1}", solid={"kind": "extruded", "sweep": "w", "z0": round(lo + k * rep["translation"][2], 4), "z1": round(hi + k * rep["translation"][2], 4)}) for k in range(rep["count"])]


for scale in ("50", "100"):
    fine = scale == "50"
    for ident, views in (("bucket-lip", ("elevation", "front", "plan") if fine else ("elevation",)), ("cab-light", ("elevation", "front", "plan")), ("link-pin", ("front", "plan") if fine else ("plan",)), ("stick-root-boss", ("front", "plan") if fine else ("plan",))):
        for view in views:
            shapes = asset["representations"][scale]["geometry"][view]
            low = min(s.get("z", 0) for s in shapes)
            for shape in shapes_of(oplib.solid(asset, scale, ident)):
                if any(s["id"] == shape["id"] for s in shapes) and not any(o.get("shape", {}).get("id") == shape["id"] and o["scale"] == scale and o["view"] == view for o in ops):
                    continue
                shape["z"] = round(low - 0.1, 3)
                ops = [o for o in ops if not (o["op"] == "shape-add" and o["scale"] == scale and o["view"] == view and o["shape"]["id"] == shape["id"])]
                ops.append({"op": "shape-add", "scale": scale, "view": view, "after": shapes[0]["id"], "shape": shape})
with open(path, "w") as f:
    f.write("[\n" + ",\n".join(json.dumps(o, ensure_ascii=False) for o in ops) + "\n]\n")
print(f"excavator: {len(ops)} ops")
