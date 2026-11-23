"""🧸️ Parent carrying toddler, three-views sweep (pose hoisting, clips shift-toddler and walk-carrying). The toddler rides on the parent's right forearm, which rests hanging and is raised 100 degrees in the default pose. The front and plan drawings of that forearm and of every toddler part were flat outlines stretched until they looked right at exactly that angle (the front head was 3.9 m tall in rest coordinates); at 112 degrees the front toddler stood 46 cm above the side elevation and the 3D body. In front and plan each of these parts becomes a solid of its side profile of the same scale: sleeves, limbs, torso, neck, head and hat revolved about their bone (`mirror`, the true body of revolution, as `sleeve-fore-l` of this asset already is) with the profile narrowed to the lateral half width the old drawing had (`lateralScale` is not read for 2D solids), face details, mittens and boots prisms over the lateral span the old drawing had (the toddler boots of 1:500 over the 74 mm of their solid). The parent's own legs had the same kind of flat front and plan outlines (the plan shank kept its rest length with the knee bent 54 degrees, 147 mm short of the body) and are drawn the same way: thighs, shanks and boot shafts revolved, boots, toes and soles prisms. `python3 tv1-parent-carrying-toddler.py [target json]`."""
import sys

import tv1_edit as e

doc = e.Doc(e.catalog_file("parent-carrying-toddler"))

REST_Z = {"forearm-r": -0.182651, "toddler-torso": -0.162651, "toddler-neck": -0.162651, "toddler-head": -0.162651}
for side, sign in (("l", 1), ("r", -1)):
    for bone in ("arm", "forearm", "hand"):
        REST_Z[f"toddler-{bone}-{side}"] = round(-0.162651 + sign * 0.09, 6)
    for bone in ("thigh", "shank", "foot"):
        REST_Z[f"toddler-{bone}-{side}"] = round(-0.162651 + sign * 0.05, 6)

for side, sign in (("l", 1), ("r", -1)):
    for bone in ("thigh", "shank", "foot", "toe"):
        REST_Z[f"{bone}-{side}"] = sign * 0.090902

ROUND = {"sleeve-fore-r", "glove-cuff-r", "toddler-head", "toddler-neck", "toddler-hat", "toddler-hat-cuff", "toddler-pompom", "toddler-suit", "toddler-collar"}
FLAT = {"toddler-nose", "toddler-lips", "toddler-brow"}
for side in ("l", "r"):
    ROUND |= {f"jeans-thigh-{side}", f"jeans-shank-{side}", f"boot-shaft-{side}"}
    FLAT |= {f"boot-{side}", f"boot-sole-{side}", f"boot-toe-{side}", f"boot-toe-sole-{side}"}
    ROUND |= {f"toddler-sleeve-{side}", f"toddler-forearm-{side}", f"toddler-thigh-{side}", f"toddler-shank-{side}", f"toddler-boot-shaft-{side}"}
    FLAT |= {f"toddler-ear-{side}", f"toddler-cheek-{side}", f"toddler-eye-{side}", f"toddler-mitten-{side}", f"toddler-mitten-thumb-{side}", f"toddler-boot-{side}", f"toddler-boot-sole-{side}"}

THIN = {(500, "toddler-boot-l"): (-0.1487, -0.0747), (500, "toddler-boot-r"): (-0.2487, -0.1747)}
count = 0
for scale in (50, 100, 200, 500):
    side_shapes = {(s["id"], s["bone"]): s for _, s in doc.shapes(scale, "elevation")}
    spans = {}
    for view, k in (("front", 0), ("plan", 1)):
        for path, shape in doc.shapes(scale, view):
            if shape["id"] in ROUND | FLAT and "solid" not in shape:
                lateral = [p[k] for p in shape["points"]]
                spans.setdefault((shape["id"], shape["bone"]), (round(min(lateral), 4), round(max(lateral), 4)))
    for view in ("front", "plan"):
        for path, shape in doc.shapes(scale, view):
            key = (shape["id"], shape["bone"])
            if key not in spans or key not in side_shapes or "solid" in shape:
                continue
            lo, hi = THIN.get((scale, shape["id"]), spans[key])
            profile = side_shapes[key]["points"]
            if shape["id"] in ROUND:
                reach = max(abs(p[1]) for p in profile)
                profile = [[p[0], round(p[1] * (hi - lo) / 2 / reach, 5)] for p in profile]
                solid = {"kind": "revolved", "axisBone": shape["bone"], "axis": "u", "axisX": 0, "axisZ": round((lo + hi) / 2 - REST_Z[shape["bone"]], 4), "mirror": True}
            else:
                solid = {"kind": "extruded", "sweep": "w", "z0": lo, "z1": hi}
            doc.set(path, {**shape, "points": profile, "solid": solid})
            count += 1

print(doc.save(sys.argv[1] if len(sys.argv) > 1 else None), "edits")
