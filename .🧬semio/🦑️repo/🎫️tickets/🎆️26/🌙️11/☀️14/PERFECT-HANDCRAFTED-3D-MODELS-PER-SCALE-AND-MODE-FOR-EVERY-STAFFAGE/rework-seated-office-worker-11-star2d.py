"""⭐ Stage 11 of the seated office worker: the drawn star base states its thickness.

The four star arms that leave the side plane (72 and 144 degrees of rest spread) and their castors were flat elevation shapes; a flat shape on a bone that is turned out of the paper has no defined depth. Each of them now states the lateral span of its part as an extrusion along w (arm 44 mm, twin wheels 22 mm each side of a 12 mm gap, hood 60 mm, stem 12 mm; one 75 mm wheel at 1:500), taken from the solids of the same names.

Run: `python3 rework-seated-office-worker-11-star2d.py [--drop-views]` (candidate), `--apply` writes `representations` of the catalog file. `--drop-views` removes the hand-drawn front and plan twins that the extrusion now generates.
"""
import sys

from rework_03_lib import *

A = "humans/seated-office-worker"
BONES = [f"chair-{part}-{arm}" for part in ("arm", "castor") for arm in ("side-a", "side-b", "back-a", "back-b")]


def span(shape, scale):
    """📏 The lateral span of one star-base part by its name."""
    name = shape["id"]
    if "arm" in name:
        return (-0.0375, 0.0375) if scale == "500" else (-0.022, 0.022)
    if "hood" in name:
        return (-0.03, 0.03)
    if "stem" in name:
        return (-0.006, 0.006)
    if scale == "500" or not name.endswith(("-a", "-b")) or name.count("-") < 4:
        return (-0.0375, 0.0375) if scale == "500" else (-0.028, 0.028)
    return (0.006, 0.028) if name.endswith("-a") else (-0.028, -0.006)


doc = read(catalog_path(A))
a = doc["asset"]
a["volumes"] = read(source_path(A))["volumes"]
for scale, representation in a["representations"].items():
    geometry = representation["geometry"]
    done = set()
    for shape in geometry["elevation"]:
        if shape.get("bone") in BONES and (shape.get("closed") or shape.get("kind") == "path"):
            z0, z1 = span(shape, scale)
            shape["solid"] = {"kind": "extruded", "sweep": "w", "z0": z0, "z1": z1}
            done.add(shape["id"])
    if "--drop-views" in sys.argv:
        for view in ("front", "plan"):
            geometry[view] = [s for s in geometry[view] if not (s.get("bone") in BONES and s["id"] in done)]
    print("[DEBUG]", scale, sorted(done))
write(candidate_path(A), doc)
if "--apply" in sys.argv:
    live = read(catalog_path(A))
    live["asset"]["representations"] = a["representations"]
    write(catalog_path(A), live)
    print("applied")
