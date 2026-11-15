"""🩹 Heat-pump contact anchors repaired at the root (author 36): the anchors described a front and a rear rail (x +-0.13, 0.08 x 0.45 m) while every drawing shows two rubber feet at the left and right (front view z +-0.34..0.52, elevation x -0.17..0.17): 0.34 m deep, 0.18 m wide. Run once."""
import glob
P = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/heat-pump-outdoor-unit/*.json")[0]
t = open(P, encoding="utf8").read()
a = '''    "id": "contact-r",
    "bone": "unit",
    "x": 0.13,
    "y": 0,
    "kind": "contact",
    "title": {
     "en": "Front foot contact",
     "de": "Auflager vorderer Fuß"
    },
    "support": "contact",
    "normalDeg": 90,
    "footprintLengthM": 0.08,
    "footprintWidthM": 0.45,'''
b = '''    "id": "contact-r",
    "bone": "unit",
    "x": 0,
    "y": 0,
    "z": -0.43,
    "kind": "contact",
    "title": {
     "en": "Right rubber foot contact",
     "de": "Auflager rechter Gummifuß"
    },
    "support": "contact",
    "normalDeg": 90,
    "footprintLengthM": 0.34,
    "footprintWidthM": 0.18,'''
c = a.replace("contact-r", "contact-l").replace('"x": 0.13', '"x": -0.13').replace("Front foot", "Rear foot").replace("vorderer", "hinterer")
d = b.replace("contact-r", "contact-l").replace('"z": -0.43', '"z": 0.43').replace("Right", "Left").replace("rechter", "linker")
n = 0
for x, y in ((a, b), (c, d)):
    if x in t:
        t = t.replace(x, y)
        n += 1
print("edits", n)
import json
json.loads(t)
open(P, "w", encoding="utf8").write(t)
