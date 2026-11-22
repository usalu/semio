"""📌 Stage 8 of the seated office worker: root members that follow the rebuilt model: contact anchors at the real contact faces (seat, thighs, lumbar pad, heels of the hands on the palm rest), dimensions with the 1200 x 800 mm desk, the steel area of the chair.

Run: `python3 rework-03-seated-office-worker-8-root.py` writes the candidate (with the volumes source merged); `--write` copies the root members into the catalog after `r3-validate.ts --file`.
"""
import sys

from rework_03_lib import *

A = "humans/seated-office-worker"
KEYS = ("anchors", "poses", "clips", "dimensions", "thermal")
if "--write" in sys.argv:
    cand, doc = read(candidate_path(A)), read(catalog_path(A))
    for key in KEYS:
        doc["asset"][key] = cand["asset"][key]
    write(catalog_path(A), doc)
    sys.exit(0)

doc = read(catalog_path(A))
a = doc["asset"]
a["volumes"] = read(source_path(A))["volumes"]
anchors = {x["id"]: x for x in a["anchors"]}
anchors["seat"].update({"x": -0.088, "y": 0.025})
anchors["thigh-support"].update({"y": -0.067})
anchors["backrest"].update({"bone": "lumbar", "x": 0.11, "y": 0.062})
for side, (en, de) in (("l", ("left", "Linker")), ("r", ("right", "Rechter"))):
    if f"palm-{side}" not in anchors:
        at = next(i for i, x in enumerate(a["anchors"]) if x["id"] == "ball-r") + 1
        a["anchors"].insert(at, {"id": f"palm-{side}", "bone": f"hand-{side}", "x": 0.012, "y": -0.019, "kind": "contact", "title": {"en": f"Heel of the {en} hand on the palm rest", "de": f"{de} Handballen auf der Handauflage"},
                                 "support": "contact", "normalDeg": 90, "footprintLengthM": 0.04, "footprintWidthM": 0.05, "frictionCoefficient": 0.6})
P = {p["id"]: p for p in a["poses"]}
for pose, extra in (("typing", ["palm-l", "palm-r"]), ("reading-screen", ["palm-r"])):
    for id in extra:
        if id not in P[pose]["contacts"]:
            P[pose]["contacts"].append(id)
clip = next(c for c in a["clips"] if c["id"] == "typing")
for id in ("palm-l", "palm-r"):
    if not any(c["anchor"] == id for c in clip["contacts"]):
        clip["contacts"].append({"anchor": id, "fromS": 0, "toS": clip["durationS"]})
a["dimensions"].update({"widthM": 1.308, "depthM": 1.2})
for region in a["thermal"]["regions"]:
    if region["id"] == "chair-steel":
        region["areaM2"] = 0.3
        region["title"] = {"en": "Steel column, mechanism, backrest bar and armrest posts", "de": "Stahlsäule, Mechanik, Lehnenträger und Armlehnenstützen"}
write(candidate_path(A), doc)
