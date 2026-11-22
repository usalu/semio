"""👔 Stage 13 of the seated office worker: the shirt yoke loses its box corners, and the 1:100 text names the finger blocks.

The two shirt sections under the shoulder line (0.262 and 0.28 m above T12) had superellipse exponents 2.6 and 2.7: from behind the upper back was a board with an edge down each shoulder blade. They are 2.35 and 2.3 now, in the shirt and in every thermal shell of the thorax alike, so the layers stay nested.

Run: `python3 rework-seated-office-worker-13-yoke.py` (candidate), `--apply` writes the volumes source.
"""
import sys

from rework_03_lib import *

A = "humans/seated-office-worker"
YOKE = {0.262: 2.35, 0.28: 2.3}
doc = read(catalog_path(A))
a = doc["asset"]
a["volumes"] = read(source_path(A))["volumes"]
v = a["volumes"]
for name in ("body", "thermal"):
    for solid in v["50"][name]:
        if solid.get("bone") == "torso" and solid.get("kind") == "loft":
            for section in solid["sections"]:
                station = round(section["at"][0], 3)
                if station in YOKE and section.get("exponent", 2) > YOKE[station]:
                    section["exponent"] = YOKE[station]
                    print("[DEBUG]", name, solid["id"], station)
EN = " The four fingers of each hand are three blocks on the phalanx bones of the middle finger, the thumb is two solids on its metacarpal and proximal bones, so the hands curl and tap with the rig as at 1:50."
DE = " Die vier Finger jeder Hand sind drei Blöcke auf den Gliedknochen des Mittelfingers, der Daumen zwei Körper auf seinem Mittelhand- und Grundgliedknochen, sodass die Hände wie in 1:50 mit dem Rig greifen und tippen."
first = v["100"]["decisions"][0]
if EN not in first["en"]:
    first["en"] += EN
    first["de"] += DE
write(candidate_path(A), doc)
if "--apply" in sys.argv:
    write(source_path(A), {"volumes": v})
    print("applied")
