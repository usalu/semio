"""🧸️ Parent carrying toddler, three-views sweep (pose hoisting, clips shift-toddler and walk-carrying). The toddler rides on the parent's right forearm, which rests hanging and is raised 100 degrees in the default pose. The front and plan drawings of that forearm and of every toddler part were flat outlines stretched until they looked right at exactly that angle (the front head was 3.9 m tall in rest coordinates); at 112 degrees the front toddler stood 46 cm above the side elevation and the 3D body. In front and plan each of these parts becomes a solid of its side profile of the same scale: sleeves, limbs, torso, neck, head and hat revolved about their bone (`mirror`, the true body of revolution, as `sleeve-fore-l` of this asset already is) with the profile narrowed to the lateral half width the old drawing had (`lateralScale` is not read for 2D solids), face details, mittens and boots prisms over the lateral span the old drawing had (the toddler boots of 1:500 over the 74 mm of their solid). The parent's own legs had the same kind of flat front and plan outlines (the plan shank kept its rest length with the knee bent 54 degrees, 147 mm short of the body) and become prisms too, but of their own outlines: legs hang and feet point ahead at rest, so the hand-drawn front and plan outlines are true there and are kept as the section of a prism along the direction the view looks (front: legs swept along their normal, feet along the foot; plan: legs along the leg, feet along their normal), as long as the side profile says. One consequence at 1:500: the toddler now hides the parent's right upper arm as the body does, and the strip left visible printed at 0.186 mm; the front outline of that sleeve is widened by hand to 142 mm and the decision text says so. `python3 tv1-parent-carrying-toddler.py [target json [scale/view/id prefixes to leave as they are, for trials]]`."""
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
LEG, FOOT = set(), set()
ORIGIN = {"thigh": (0.0, 0.0), "shank": (0.0, -0.4), "foot": (0.0, -0.818), "toe": (0.13, -0.906)}
TOE_500 = {"u": (-0.012, 0.086), "v": (0.0, 0.066)}
for side in ("l", "r"):
    LEG |= {f"jeans-thigh-{side}", f"jeans-shank-{side}", f"boot-shaft-{side}"}
    FOOT |= {f"boot-{side}", f"boot-sole-{side}", f"boot-toe-{side}", f"boot-toe-sole-{side}"}
    ROUND |= {f"toddler-sleeve-{side}", f"toddler-forearm-{side}", f"toddler-thigh-{side}", f"toddler-shank-{side}", f"toddler-boot-shaft-{side}"}
    FLAT |= {f"toddler-ear-{side}", f"toddler-cheek-{side}", f"toddler-eye-{side}", f"toddler-mitten-{side}", f"toddler-mitten-thumb-{side}", f"toddler-boot-{side}", f"toddler-boot-sole-{side}"}

THIN = {(500, "toddler-boot-l"): (-0.1485, -0.075), (500, "toddler-boot-r"): (-0.2485, -0.175)}
SKIP = tuple(sys.argv[2].split(",")) if len(sys.argv) > 2 else ()


def own(scale, view, shape, side_shapes):
    """🦵️ A leg or foot part of the parent: the hand-drawn front or plan outline of the rest pose (legs hang, feet point ahead) becomes the section of a prism along the direction that view looks, as long as the side profile of the same scale says."""
    ox, oy = ORIGIN[shape["bone"][:-2]]
    z = REST_Z[shape["bone"]]
    twin = side_shapes.get((shape["id"], shape["bone"]))
    along = lambda k, name: (round(min(p[k] for p in twin["points"]), 4), round(max(p[k] for p in twin["points"]), 4)) if twin is not None else TOE_500[name]
    upright = shape["bone"][:-2] in ("thigh", "shank")
    if view == "front":
        if upright:
            return [[round(oy - y, 5), round(x - z, 5)] for x, y in shape["points"]], "v", along(1, "v")
        return [[round(y - oy, 5), round(x - z, 5)] for x, y in shape["points"]], "u", along(0, "u")
    if upright:
        return [[round(x - ox, 5), round(y - z, 5)] for x, y in shape["points"]], "u", along(0, "u")
    return [[round(x - ox, 5), round(y - z, 5)] for x, y in shape["points"]], "v", along(1, "v")


THIN = {(500, "toddler-boot-l"): (-0.1485, -0.075), (500, "toddler-boot-r"): (-0.2485, -0.175)}
SKIP = tuple(sys.argv[2].split(",")) if len(sys.argv) > 2 else ()
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
            if "solid" in shape or SKIP and f"{scale}/{view}/{shape['id']}".startswith(SKIP):
                continue
            if shape["id"] in LEG | FOOT:
                profile, sweep, (z0, z1) = own(scale, view, shape, side_shapes)
                doc.set(path, {**shape, "points": profile, "solid": {"kind": "extruded", "sweep": sweep, "z0": z0, "z1": z1}})
                continue
            if key not in spans or key not in side_shapes:
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

path, sleeve = doc.shape(500, "front", "sleeve-upper-r", "upper-arm-r")
assert [round(x, 3) for x, _ in sleeve["points"]] == [-0.19, -0.119, -0.11, -0.143, -0.199, -0.237, -0.223]
doc.set(path + ("points",), [[x, y] for x, (_, y) in zip((-0.192, -0.1125, -0.1024, -0.1393, -0.2021, -0.2446, -0.2289), sleeve["points"])])
note = ("asset", "representations", "500", "decisions", "thinElements")
text = doc.data["asset"]["representations"]["500"]["decisions"]["thinElements"]
doc.set(note, {"en": text["en"] + " The parent's right upper sleeve is drawn 15 mm wider than the arm in the front elevation (142 mm instead of 127 mm), so that the strip of it that stays visible beside the lifted toddler prints at 0.2 mm.", "de": text["de"] + " Der rechte Oberärmel des Elternteils ist in der Vorderansicht 15 mm breiter gezeichnet als der Arm (142 mm statt 127 mm), damit der Streifen, der neben dem angehobenen Kleinkind sichtbar bleibt, mit 0,2 mm druckt."})

print(doc.save(sys.argv[1] if len(sys.argv) > 1 else None), "edits")
