"""🙂 person-with-umbrella: the head of the 1:50 body and thermal lists, modelled as a face instead of a flat mask.

Head frame: x up from the atlanto-occipital joint, y back, z left. `FACE` states the profile of the skin per height (front edge, back edge, half breadth): chin prominence, mentolabial fold, lips, upper lip, cheek, infraorbital plane, the hollow at eye level under the brow ridge, forehead and vault. Hair lies on the vault as a second shell, stated the same way per height: its front edge is the hairline (above the forehead, back over the temples, behind the ears, down to the nape), its back runs into the bun.

Usage: python3 rework-05-person-with-umbrella-head.py            prints what it would change
       python3 rework-05-person-with-umbrella-head.py --apply    volumes source and catalog `asset.volumes`
"""
import importlib.util
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("skeleton", os.path.join(T, "rework-05-person-with-umbrella-skeleton.py"))
skeleton = importlib.util.module_from_spec(spec)
spec.loader.exec_module(skeleton)

FACE = [(-0.085, -0.094, -0.060, 0.018), (-0.078, -0.104, -0.036, 0.031), (-0.066, -0.108, -0.016, 0.041), (-0.054, -0.103, -0.002, 0.048), (-0.043, -0.106, 0.010, 0.054), (-0.030, -0.109, 0.026, 0.059), (-0.015, -0.108, 0.044, 0.064), (0.005, -0.109, 0.058, 0.069), (0.025, -0.107, 0.066, 0.0715), (0.036, -0.104, 0.070, 0.0725), (0.050, -0.113, 0.072, 0.073), (0.065, -0.112, 0.0725, 0.073), (0.085, -0.108, 0.0725, 0.072), (0.105, -0.094, 0.066, 0.066), (0.122, -0.072, 0.052, 0.054), (0.134, -0.048, 0.032, 0.036), (0.141, -0.028, 0.010, 0.016)]
HAIR = [(-0.022, 0.030, 0.060, 0.030), (-0.005, 0.020, 0.070, 0.050), (0.015, 0.010, 0.078, 0.066), (0.035, -0.002, 0.081, 0.076), (0.050, -0.030, 0.082, 0.079), (0.065, -0.065, 0.0825, 0.080), (0.085, -0.092, 0.082, 0.079), (0.100, -0.103, 0.078, 0.075), (0.112, -0.091, 0.068, 0.069), (0.123, -0.075, 0.055, 0.060), (0.133, -0.056, 0.038, 0.046), (0.141, -0.037, 0.020, 0.030), (0.146, -0.022, 0.006, 0.014)]
LEVEL = [0, -0.70711, 0, 0.70711]
NOSE = [((0.040, -0.103, 0), 0.007, 0.006), ((0.022, -0.108, 0), 0.008, 0.008), ((0.004, -0.113, 0), 0.011, 0.010), ((-0.012, -0.117, 0), 0.015, 0.011), ((-0.021, -0.113, 0), 0.016, 0.008), ((-0.026, -0.106, 0), 0.010, 0.004)]
ELLIPSOIDS = {
    "head-lips": ((-0.0405, -0.107, 0), (0.0045, 0.006, 0.022)),
    "head-lip-lower": ((-0.0485, -0.105, 0), (0.005, 0.006, 0.019)),
    "head-eye-l": ((0.036, -0.094, 0.031), (0.0055, 0.0055, 0.012)),
    "head-eye-r": ((0.036, -0.094, -0.031), (0.0055, 0.0055, 0.012)),
    "head-brow-l": ((0.052, -0.102, 0.032), (0.0038, 0.0045, 0.019)),
    "head-brow-r": ((0.052, -0.102, -0.032), (0.0038, 0.0045, 0.019)),
}


def level(rows):
    """📐 Horizontal sections of a form that stands on the head bone: front edge, back edge and half breadth per height."""
    return [{"at": [x, round((front + back) / 2, 5), 0], "rx": wide, "ry": round((back - front) / 2, 5), "exponent": 2, "rotation": list(LEVEL)} for x, front, back, wide in rows]


def lofts():
    return {"head": level(FACE), "hair": level(HAIR), "head-nose": skeleton.sections(NOSE, (0, 0, 1))}


def shape(solid, made):
    """✍️ Replaces the geometry of one solid and keeps what it is (bone, material, region, layer)."""
    key = solid["id"][3:] if solid["id"].startswith("th-") else solid["id"]
    if "kind" not in solid:
        return False
    if key in made:
        solid["kind"] = "loft"
        solid["sections"] = [dict(row, exponent=2.4 if key == "hair" else 2) for row in made[key]]
        solid["capStart"] = solid["capEnd"] = True
        solid.pop("transform", None)
        solid.pop("radii", None)
        return True
    if key in ELLIPSOIDS:
        centre, radii = ELLIPSOIDS[key]
        solid["kind"] = "ellipsoid"
        solid["radii"] = list(radii)
        solid["transform"] = {"translation": list(centre)}
        solid.pop("sections", None)
        return True
    return False


def rebuild(volumes):
    made = lofts()
    for name in ("body", "thermal"):
        solids = volumes["50"][name]
        prefix = "th-" if name == "thermal" else ""
        if not any(solid["id"] == prefix + "head-lip-lower" for solid in solids):
            at = next(i for i, solid in enumerate(solids) if solid["id"] == prefix + "head-lips")
            solids.insert(at + 1, {**json.loads(json.dumps(solids[at])), "id": prefix + "head-lip-lower"})
        for solid in solids:
            if solid["id"] == "th-head-lip-lower" and "of" in solid:
                solid["of"] = "head-lip-lower"
        changed = [solid["id"] for solid in solids if shape(solid, made)]
        print(f"[DEBUG] 1:50 {name}: {' '.join(changed)}")


def main():
    source = json.load(open(skeleton.SOURCE, encoding="utf-8"))
    rebuild(source["volumes"])
    if "--apply" not in sys.argv:
        return
    document = json.load(open(skeleton.CATALOG, encoding="utf-8"))
    document["asset"]["volumes"] = source["volumes"]
    for path, value in ((skeleton.SOURCE, source), (skeleton.CATALOG, document)):
        with open(path, "w", encoding="utf-8") as handle:
            json.dump(value, handle, ensure_ascii=False, indent=2)
            handle.write("\n")
    print("[DEBUG] applied to catalog and source")


if __name__ == "__main__":
    main()
