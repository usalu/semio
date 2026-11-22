"""🛋️ Three-seat sofa, author pass 1: the timber frame as a real load path and the upholstery layers nested at 1:50, 1:100 and 1:200.

Hand-chosen numbers, bone-local metres (frame bone: y = world y - 0.165, x along the sofa, z towards the back).
Run from the ticket folder: python3 rework-three-seat-sofa-frame.py <catalog asset json>
@see ./rework-brief.md
@see ./rework/assets/furniture--three-seat-sofa.md
"""
import copy
import json
import sys

SOURCE = "volumes/furniture/three-seat-sofa.json"
ROT = [0.707107, 0.0, 0.0, 0.707107]
FABRIC = 0.001
CUSHION_JOINTS_X = [0.7333, 1.2667]
UPRIGHTS_X = {"member-upright-1": 0.2, "member-upright-2": 0.7208, "member-upright-3": 1.2542, "member-upright-4": 1.775}
UPRIGHT_PROFILE = [[0.065, 0.325], [0.065, 0.44], [0.595, 0.44], [0.595, 0.39]]
BACK_FACE_PROFILE = [[0.065, 0.313], [0.065, 0.325], [0.595, 0.39], [0.595, 0.378]]
STRETCHER = {"50": ([0.025, 0.09, 0.8], -0.005), "100": ([0.03, 0.11, 0.8], -0.01), "200": ([0.03, 0.098, 0.79], -0.016)}
PLY_BACK = {
    "50": [(0.086, 0.37, 0.779, 0.059), (0.355, 0.39037, 0.779, 0.03863), (0.645, 0.41, 0.779, 0.019), (0.662, 0.41, 0.775, 0.012)],
    "100": [(0.086, 0.37, 0.779, 0.059), (0.645, 0.41, 0.779, 0.019), (0.662, 0.41, 0.775, 0.012)],
    "200": [(0.086, 0.37, 0.779, 0.059), (0.645, 0.41, 0.779, 0.019), (0.662, 0.41, 0.775, 0.012)],
}
CORES = [
    ("th-base", "th-base-ply", "base", "ply-skirt"),
    ("th-back-frame", "th-back-frame-batting", "back-frame", "batting-back"),
    ("th-back-frame-batting", "th-back-frame-ply", "back-frame", "ply-back"),
    ("th-arm-l", "th-arm-l-core", "arm-l", "batting-arm"),
    ("th-arm-r", "th-arm-r-core", "arm-r", "batting-arm"),
] + [(f"th-{part}-{i}", f"th-{part}-{i}-core", f"{part}-{i}", f"foam-{part}") for i in (1, 2, 3) for part in ("seat", "back")]


def unit(a, b):
    """📐 Unit vector from a to b."""
    d = [b[k] - a[k] for k in range(3)]
    n = sum(v * v for v in d) ** 0.5
    return [v / n for v in d]


def inset_loft(body, t):
    """🧅 The same loft one layer thickness further in: radii minus t, both ends pulled in by t."""
    sections = copy.deepcopy(body["sections"])
    first, last = unit(sections[0]["at"], sections[1]["at"]), unit(sections[-2]["at"], sections[-1]["at"])
    sections[0]["at"] = [round(sections[0]["at"][k] + t * first[k], 5) for k in range(3)]
    sections[-1]["at"] = [round(sections[-1]["at"][k] - t * last[k], 5) for k in range(3)]
    for s in sections:
        s["rx"], s["ry"] = round(s["rx"] - t, 5), round(s["ry"] - t, 5)
    return sections


def inset_extrude(body, t):
    """🧅 The same prism one layer thickness further in, scaled about the middle of its section."""
    p = body["profile"]
    lo = [min(q[k] for q in p) for k in (0, 1)]
    hi = [max(q[k] for q in p) for k in (0, 1)]
    c = [(lo[k] + hi[k]) / 2 for k in (0, 1)]
    f = [1 - t / ((hi[k] - lo[k]) / 2) for k in (0, 1)]
    return [[round(c[k] + (q[k] - c[k]) * f[k], 5) for k in (0, 1)] for q in p], body["from"] + t, body["to"] - t


def core(scale, body, region, layer, new_id, material):
    """🧱 One nested layer solid of a region."""
    solid = {"id": new_id, "kind": body["kind"], "bone": body["bone"], "material": material, "role": "body", "tone": body["tone"]}
    if layer == "ply-back":
        solid["sections"] = [{"at": [1.0, y, z], "rx": rx, "ry": ry, "exponent": 12, "rotation": ROT} for y, z, rx, ry in PLY_BACK[scale]]
        solid["capStart"] = solid["capEnd"] = True
    elif body["kind"] == "loft":
        solid["sections"] = inset_loft(body, FABRIC)
        solid["capStart"] = solid["capEnd"] = True
    else:
        solid["profile"], solid["from"], solid["to"] = inset_extrude(body, FABRIC)
        solid["plane"] = body["plane"]
    solid["region"], solid["layer"] = region, layer
    return solid


def thermal(scale, v, layers):
    """🌡️ Every region with its layers nested in their real build-up, each in its own material."""
    bodies = {b["id"]: b for b in v["body"]}
    out = [s for s in v["thermal"] if s["id"] not in {c[1] for c in CORES}]
    for parent, new_id, body_id, layer in CORES:
        at = next(i for i, s in enumerate(out) if s["id"] == parent)
        region = out[next(i for i, s in enumerate(out) if s["id"] == parent.replace("-batting", ""))]["region"]
        out.insert(at + 1, core(scale, bodies[body_id], region, layer, new_id, layers[(region, layer)]))
    v["thermal"] = out


def structure(scale, v):
    """🪚 Frame joinery: arm frames down onto the feet, cross stretchers under the deck, raked back uprights with a ply face."""
    added = {"member-stretcher-1", "member-stretcher-2"} | ({"member-back-face"} if scale == "50" else set())
    out = [s for s in v["structure"] if s["id"] not in added]
    member = {"material": "beech-timber", "role": "body", "tone": 0.3, "member": "frame", "part": "member"}
    size, y = STRETCHER[scale]
    at = next(i for i, s in enumerate(out) if s["id"] == "member-deck")
    for n, x in enumerate(CUSHION_JOINTS_X):
        out.insert(at + n, {"id": f"member-stretcher-{n + 1}", "kind": "box", "bone": "frame", "size": size, "transform": {"translation": [x, y, 0.005]}, **member})
    if scale == "50":
        for s in out:
            if s["id"] in UPRIGHTS_X:
                x = UPRIGHTS_X[s["id"]]
                s.pop("size", None), s.pop("transform", None)
                s.update({"kind": "extrude", "profile": UPRIGHT_PROFILE, "plane": "yz", "from": x, "to": round(x + 0.025, 4)})
            if s["id"] in ("member-arm-left-post", "member-arm-right-post"):
                s["size"], s["transform"]["translation"][0] = [0.48, 0.14, 0.025], 0.24
            if s["id"] in ("member-arm-left-bottom-rail", "member-arm-right-bottom-rail"):
                s["size"], s["transform"]["translation"][0] = [0.03, 0.14, 0.81], 0.015
        at = next(i for i, s in enumerate(out) if s["id"] == "member-top-rail")
        out.insert(at + 1, {"id": "member-back-face", "kind": "extrude", "bone": "frame", "profile": BACK_FACE_PROFILE, "plane": "yz", "from": 0.2, "to": 1.8, **member, "material": "birch-plywood", "tone": 0.35})
    v["structure"] = out


def main():
    """✍️ Read the catalog layer table and the source, apply the hand edits, write the source."""
    asset = json.load(open(sys.argv[1], encoding="utf-8"))["asset"]
    layers = {(r["id"], l["id"]): l["material"] for r in asset["thermal"]["regions"] for l in r.get("layers", [])}
    doc = json.load(open(SOURCE, encoding="utf-8"))
    for scale in ("50", "100", "200"):
        thermal(scale, doc["volumes"][scale], layers)
        structure(scale, doc["volumes"][scale])
    json.dump(doc, open(SOURCE, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    for scale in ("50", "100", "200"):
        print(scale, {k: len(doc["volumes"][scale][k]) for k in ("body", "thermal", "structure")})


main()
