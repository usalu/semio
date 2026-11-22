"""🌟 Stage 1b of the seated office worker: the five base arm bones lie horizontal (the abduction turns about the bone's own normal, so a dipped bone would rise as it turns); the 4.19 degree dip of the arms is stated in the solids, the castor bones hang 75 mm below at 280 mm radius.

Run once after stage 1: `python3 rework-03-seated-office-worker-1b-base.py`, validate the candidate with `--file`, then `--write`.
"""
import math
import sys

from rework_03_lib import *

A = "humans/seated-office-worker"
DIP = 4.1942
KEYS = {"front": 0, "side-a": 72, "back-a": 144, "back-b": -144, "side-b": -72}

if "--write" in sys.argv:
    cand = read(candidate_path(A))
    assert read(catalog_path(A))["asset"]["id"] == cand["asset"]["id"]
    write(catalog_path(A), cand)
    write(source_path(A), {"volumes": cand["asset"]["volumes"]})
    sys.exit(0)

doc = read(catalog_path(A))
a = doc["asset"]
a["volumes"] = read(source_path(A))["volumes"]
B = {b["id"]: b for b in a["rig"]["bones"]}
assert B["chair-arm-front"]["angleDeg"] != 0, "stage 1b already applied"
d = math.radians(DIP)
c, s = math.cos(d), math.sin(d)
for key in KEYS:
    B[f"chair-arm-{key}"].update({"angleDeg": 0, "lengthM": 0.3})
    B[f"chair-castor-{key}"].update({"x": 0.28, "y": -0.075, "angleDeg": 0})
dip = quat([0, 0, 1], -DIP)
for vol in a["volumes"].values():
    for name in ("body", "thermal", "structure"):
        for solid in vol[name]:
            if "of" in solid or not solid["bone"].startswith("chair-arm-"):
                continue
            t = solid.setdefault("transform", {})
            tr = t.get("translation", [0, 0, 0])
            t["translation"] = [round(tr[0] * c + tr[1] * s, 5), round(-tr[0] * s + tr[1] * c, 5), tr[2]]
            t["rotation"] = qmul(dip, t["rotation"]) if "rotation" in t else dip
for rep in a["representations"].values():
    for shape in rep["geometry"]["elevation"]:
        for key, phi in KEYS.items():
            if shape["bone"] != f"chair-arm-{key}":
                continue
            k = math.cos(math.radians(phi))
            if key == "front":
                shape["points"] = [[round(x * c + y * s, 4), round(-x * s + y * c, 4)] for x, y in shape["points"]]
            else:
                out = []
                for x, y in shape["points"]:
                    wx, wy = x * c * k + y * s, -x * s + y * c
                    out.append([round(wx / k, 4), round(wy, 4)])
                shape["points"] = out
write(candidate_path(A), doc)
