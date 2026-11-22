"""📝 Stage 16 of the seated office worker: the 1:50 decisions name the skull with sockets and the girdle bones.

Run: `python3 rework-seated-office-worker-16-decisions.py --apply` writes the volumes source.
"""
import sys

from rework_03_lib import *

A = "humans/seated-office-worker"
EN = " The skull is the cranium with the maxilla, the nasal bridge, the zygomatic bones with their arches, the supraorbital bar and the mandible, so the orbits are open recesses; clavicle with scapula and each hip bone hang on their own laterally placed bones (sternoclavicular to acromioclavicular joint, sacroiliac joint to hip joint), from which arms and thighs leave at the true breadths."
DE = " Der Schädel besteht aus Hirnschädel, Oberkiefer, Nasenrücken, Jochbeinen mit Jochbögen, Überaugenwulst und Unterkiefer, sodass die Augenhöhlen offene Vertiefungen sind; Schlüsselbein mit Schulterblatt und jedes Hüftbein hängen an eigenen, seitlich gelegten Knochen (Sternoklavikular- bis Akromioklavikulargelenk, Iliosakralgelenk bis Hüftgelenk), von denen Arme und Oberschenkel in den wahren Breiten abgehen."
source = read(source_path(A))
decisions = source["volumes"]["50"]["decisions"]
for d in decisions:
    print("[DEBUG]", d["en"][:90])
target = decisions[0]
for d in decisions[1:]:
    d["en"] = d["en"].replace(EN, "")
    d["de"] = d["de"].replace(DE, "")
if EN not in target["en"]:
    target["en"] += EN
    target["de"] += DE
if "--apply" in sys.argv:
    write(source_path(A), source)
    print("applied")
