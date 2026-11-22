"""🪑 Café chair, second round pass 1: carries the 12 mm plywood back (15 mm at 1:100) into the catalog root (2D shapes, thermal layer, masses, texts) and into the volume decisions.

Usage: python3 rework-cafe-chair-root.py [--write]   (without --write: candidate under 🗑️generated/r3-cafe-chair/)
"""
import json
import os
import sys

CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/cafe-chair/🔣️.json"
SOURCE = "volumes/furniture/cafe-chair.json"
CANDIDATE = "🗑️generated/r3-cafe-chair/🔣️.json"
ELEVATION = {"50": (-0.0283, 0.0433), "100": (-0.0313, 0.0463)}
PLAN_SHIFT = {"50": 0.0059, "100": 0.003}
BOARDS = ("back-panel", "back-lower-rail", "back-rail")
TEXTS = [
    ("18 mm curved beech backrest board", "12 mm curved beech plywood backrest board"),
    ("gebogenes 18-mm-Buchenrückenbrett", "gebogenes 12-mm-Buchensperrholz-Rückenbrett"),
    ("boards (18 mm)", "boards (15 mm)"),
    ("Bretter (18 mm)", "Bretter (15 mm)"),
    ("a curved 18 mm beech backrest board and a lower rail screwed to the post fronts.", "a curved 12 mm beech plywood backrest board and a lower rail lying on the post fronts, each fixed through board and tube with dome-head rivets (two per board end, one per rail end); the apron corners are coped round the tubes and held by one coach bolt through each tube."),
    ("ein gebogenes 18-mm-Buchenrückenbrett und eine untere Leiste, auf die Pfostenvorderseiten geschraubt.", "ein gebogenes 12-mm-Buchensperrholz-Rückenbrett und eine untere Leiste, die auf den Pfostenvorderseiten liegen und mit Halbrundnieten durch Brett und Rohr befestigt sind (zwei je Brettende, eine je Leistenende); die Zargenecken sind um die Rohre ausgeklinkt und mit je einer Schlossschraube durch das Rohr gehalten."),
    ("back board and rail keep their curve with five sections each.", "back board and rail keep their curve with five sections each and are 15 mm thick (the 12 mm plywood raised to the feature); rivets and bolts are below the feature."),
    ("Rückenbrett und Leiste behalten ihre Krümmung mit je fünf Schnitten.", "Rückenbrett und Leiste behalten ihre Krümmung mit je fünf Schnitten und sind 15 mm dick (das 12-mm-Sperrholz auf das Mindestmaß angehoben); Nieten und Schrauben liegen unter dem Mindestmaß."),
    ("Seat and back screws and the welded tabs (5-6 mm, 0.1 mm on paper, and hidden under the board)", "The back rivets and the seat coach bolts (12-13 mm heads, under the 0.2 mm print minimum at the 1:75 end of the band)"),
    ("Sitz- und Rückenschrauben und die angeschweißten Laschen (5-6 mm, 0,1 mm auf Papier, und unter dem Brett verborgen)", "Die Rückennieten und die Schlossschrauben des Sitzes (Köpfe 12-13 mm, unter dem Druckminimum von 0,2 mm am 1:75-Ende des Bandes)"),
]


def retext(value):
    if isinstance(value, str):
        for old, new in TEXTS:
            value = value.replace(old, new)
        return value
    if isinstance(value, list):
        return [retext(item) for item in value]
    if isinstance(value, dict):
        return {key: retext(item) for key, item in value.items()}
    return value


def root(asset):
    for scale, (y, h) in ELEVATION.items():
        geometry = asset["representations"][scale]["geometry"]
        for shape in geometry["elevation"]:
            if shape["id"] in BOARDS and shape["kind"] == "rect":
                shape["y"], shape["h"] = y, h
        for shape in geometry["plan"]:
            if shape["id"] in BOARDS and shape["kind"] == "poly":
                half = len(shape["points"]) // 2
                front = [[round(x - PLAN_SHIFT[scale], 4), z] for x, z in shape["points"][:half]]
                if abs(shape["points"][0][0] - shape["points"][-1][0] - (0.0415 if shape["id"] == "back-panel" else 0.0248)) < 1e-6:
                    shape["points"] = front + shape["points"][half:]
    region = next(r for r in asset["thermal"]["regions"] if r["id"] == "back-beech")
    region["areaM2"] = 0.205
    region["heatCapacityJM2K"] = 13824
    region["layers"][0].update({"thicknessM": 0.012, "resistanceM2KW": 0.066667, "heatCapacityJM2K": 13824})
    back = next(b for b in asset["rig"]["bones"] if b["id"] == "back")
    back["mass"].update({"kg": 1.64, "comX": 0.25})
    asset["dimensions"]["massKg"] = 7.4


source = json.load(open(SOURCE))
source = retext(source)
json.dump(source, open(SOURCE, "w"), ensure_ascii=False, indent=1)
text = open(CATALOG, encoding="utf-8").read()
document = retext(json.loads(text))
root(document["asset"])
if "--write" in sys.argv:
    if open(CATALOG, encoding="utf-8").read() != text:
        raise SystemExit("catalog changed while editing; retry")
    open(CATALOG, "w", encoding="utf-8").write(json.dumps(document, ensure_ascii=False, indent=2) + "\n")
else:
    document["asset"]["volumes"] = source["volumes"]
    os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
    open(CANDIDATE, "w", encoding="utf-8").write(json.dumps(document, ensure_ascii=False, indent=2) + "\n")


def strings(value, path=""):
    if isinstance(value, str):
        if any(word in value for word in ("7.8 kg", "7,8 kg", "screw", "schraub", "18 mm", "18-mm")) and "volumes" not in path:
            print("[DEBUG] ", path, "::", value[:260])
    elif isinstance(value, list):
        for index, item in enumerate(value):
            strings(item, f"{path}/{index}")
    elif isinstance(value, dict):
        for key, item in value.items():
            strings(item, f"{path}/{key}")


strings(document["asset"])
