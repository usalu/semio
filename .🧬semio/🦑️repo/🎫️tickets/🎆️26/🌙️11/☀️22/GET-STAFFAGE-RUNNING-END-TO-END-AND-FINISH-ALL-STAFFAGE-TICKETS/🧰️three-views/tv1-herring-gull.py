"""🕊️ Herring gull, three-views sweep (poses landing, approach-a, flare-key and clip landing). Two causes. (1) The side-elevation prisms of the ten primaries and of every exposed rachis stated their lateral span from the feather bone instead of in asset coordinates (README, solid placements: `z0`, `z1`, `zTip` are positions of the rest pose), so each feather stood 40 to 48 mm beside its bone and swung about it in the twisted wing: 25 mm too high in the flare, 39 mm too low at the end of the landing. The rest z of the feather bone is added (`tv1-rest.ts`; the near wing mirrors the far one). (2) The front toes and webs were end-on outlines carried as plates, so the trailing feet stayed 4 mm tall in the front elevation: they become prisms of the side profile of the same scale over the width the front drawing had (the splayed toes of 1:50 diagonal with `zTip` and a five-point profile, because the union of the twelve-point one does not close at pose approach-a, the known `unionPolygons` defect). The `primary-lines` of 1:50, one drawing bound to the ninth primary but spread over the folded stack of all ten, take the lateral span of that feather, so they stay on it when the wing opens. `python3 tv1-herring-gull.py [target json]`."""
import re
import sys

import tv1_edit as e

doc = e.Doc(e.catalog_file("herring-gull"))
REST_Z = {"p1-f": 0.047631, "p2-f": 0.047186, "p3-f": 0.046742, "p4-f": 0.046298, "p5-f": 0.045853, "p6-f": 0.045409, "p7-f": 0.044964, "p8-f": 0.04452, "p9-f": 0.044075, "p10-f": 0.040075, "s12-f": 0.042001, "s11-f": 0.042183, "s10-f": 0.042365, "s9-f": 0.042546, "s8-f": 0.042728, "s7-f": 0.04291, "s6-f": 0.043092, "s5-f": 0.043274, "s4-f": 0.043455, "s3-f": 0.043637, "s2-f": 0.043819, "s1-f": 0.044001}

FEATHER = re.compile(r"^(far|near)-(primary-\d+|primary-\d+-exposed-rachis|secondary-\d+-exposed-rachis)$")
for scale in (50, 100, 200):
    for path, shape in doc.shapes(scale, "elevation"):
        solid = shape.get("solid")
        if solid is None or FEATHER.match(shape["id"]) is None:
            continue
        assert solid["kind"] == "extruded" and solid["sweep"] == "w" and abs(solid["z0"]) < 0.03
        z = REST_Z[shape["bone"][:-1] + "f"] * (1 if shape["bone"].endswith("-f") else -1)
        moved = {**solid, "z0": round(solid["z0"] + z, 8), "z1": round(solid["z1"] + z, 8)}
        if "zTip" in solid:
            moved["zTip"] = [round(t + z, 8) for t in solid["zTip"]]
        doc.set(path + ("solid",), moved)

TOE = lambda length: [[-0.003, -0.004], [0.0, -0.0046], [length, -0.0035], [length - 0.004, -0.0012], [-0.002, -0.0008]]
path, lines = doc.shape(50, "elevation", "primary-lines", "p9-n")
doc.set(path + ("solid",), {"kind": "extruded", "sweep": "w", "z0": round(-REST_Z["p9-f"] - 0.001, 6), "z1": round(-REST_Z["p9-f"] + 0.001, 6), "zTip": [round(-REST_Z["p9-f"] + 0.0403, 6), round(-REST_Z["p9-f"] + 0.0405, 6)]})

FEET = {
    50: {"foot": ((0.0065, 0.0395), None, None), "toe-inner": ((0.0205, 0.0235), (-0.0058, -0.0028), TOE(0.0384)), "toe-outer": ((0.0205, 0.0235), (0.0446, 0.0476), TOE(0.0482)), "toe-middle": ((0.0207, 0.0233), None, None)},
    100: {"foot": ((0.0043, 0.0389), None, None), "toe-middle": ((0.0209, 0.0229), None, None)},
    200: {"foot": ((0.021, 0.023), None, None)},
}
for scale, parts in FEET.items():
    for part, (span, tip, drawn) in parts.items():
        for side, sign in (("far", 1), ("near", -1)):
            bone = f"foot-{side[0]}"
            flip = (lambda pair: [pair[0], pair[1]]) if sign > 0 else (lambda pair: [-pair[1], -pair[0]])
            _, profile = doc.shape(scale, "elevation", f"{side}-{part}", bone)
            path, shape = doc.shape(scale, "front", f"{side}-{part}", bone)
            solid = {"kind": "extruded", "sweep": "w", "z0": flip(span)[0], "z1": flip(span)[1]}
            if tip is not None:
                solid["zTip"] = flip(tip)
            doc.set(path, {**shape, "points": drawn or profile["points"], "solid": solid})

print(doc.save(sys.argv[1] if len(sys.argv) > 1 else None), "edits")
