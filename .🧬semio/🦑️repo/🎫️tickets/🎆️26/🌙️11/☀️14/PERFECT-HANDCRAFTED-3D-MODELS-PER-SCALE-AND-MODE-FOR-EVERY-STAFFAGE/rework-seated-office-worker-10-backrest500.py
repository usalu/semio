"""🪑 Stage 10 of the seated office worker: the 1:500 backrest inside the real depth of the chair.

At 1:500 every part is 75 mm thick, and the J-bar stood behind the pad, 37 mm beyond the real back of the chair (the drawing was 1.347 m long, the asset is 1.308 m). The pad now ends at the real back face (58 mm behind the backrest pivot) and the bar rises inside the same 75 mm, visible below the pad only; solid, thermal shells and the three drawings state the same numbers.

Run: `python3 rework-seated-office-worker-10-backrest500.py` (candidate), `--apply` writes the volumes source and `representations.500` of the catalog file.
"""
import sys

from rework_03_lib import *

A = "humans/seated-office-worker"
BACK = 0.058
STRUT = [[-0.02, -0.09], [-0.02, 0.02], [0.0, 0.045], [0.03, BACK], [0.21, BACK], [0.21, -0.017], [0.07, -0.017], [0.055, -0.03], [0.055, -0.09]]
DRAWN = {
    "elevation": {
        "chair-backrest-pad": [[0.175, 0.0535], [0.175, -0.0215], [0.3, -0.042], [0.47, -0.017], [0.618, -0.017], [0.618, BACK], [0.47, BACK], [0.3, 0.033]],
        "chair-backrest-strut": [[-0.02, 0.02], [-0.02, -0.09], [0.055, -0.09], [0.055, -0.03], [0.07, -0.017], [0.21, -0.017], [0.21, BACK], [0.03, BACK], [0.0, 0.045]],
    },
    "front": {"chair-backrest-strut": [[-0.0375, 0.38], [0.0375, 0.38], [0.0375, 0.61], [-0.0375, 0.61]]},
    "plan": {
        "chair-backrest-pad": [[-0.228, -0.19], [-0.2, -0.222], [-0.14, -0.222], [-0.128, -0.19], [-0.128, 0.19], [-0.14, 0.222], [-0.2, 0.222], [-0.228, 0.19]],
        "chair-backrest-strut": [[-0.228, -0.0375], [-0.08, -0.0375], [-0.08, 0.0375], [-0.228, 0.0375]],
    },
}

doc = read(catalog_path(A))
a = doc["asset"]
a["volumes"] = read(source_path(A))["volumes"]
for name in ("body", "thermal", "structure"):
    for solid in a["volumes"]["500"][name]:
        if solid.get("bone") != "chair-backrest":
            continue
        if "sections" in solid:
            for section in solid["sections"]:
                section["at"][1] = round(min(section["at"][1], BACK - section["ry"]), 5)
        if "profile" in solid and solid.get("plane") == "xy":
            solid["profile"] = [list(p) for p in STRUT]
        print("[DEBUG] 1:500", name, solid["id"], solid["kind"])
for view, shapes in DRAWN.items():
    for shape in a["representations"]["500"]["geometry"][view]:
        if shape["id"] in shapes:
            shape["points"] = shapes[shape["id"]]
write(candidate_path(A), doc)
if "--apply" in sys.argv:
    write(source_path(A), {"volumes": a["volumes"]})
    live = read(catalog_path(A))
    live["asset"]["representations"]["500"] = a["representations"]["500"]
    write(catalog_path(A), live)
    print("applied")
