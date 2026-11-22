"""🧍 worker-carrying-box, second round pass 1 (continued): 1:500 body with a rounded shoulder cap, legs without a knee step and a rounded seat, and the decisions that state the new rib cage and gluteal forms (English first, German second).

Usage: python3 rework-wcb-coarse500.py    rewrites the volumes source; publish with `volumes-insert.py`

@see ./rework-wcb-form.py
"""
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes", "humans", "worker-carrying-box.json")
SLEEVE = [(-0.05, 0, -0.02, 0.02, 0.026), (-0.02, -0.001, -0.006, 0.056, 0.062), (0.04, -0.002, 0.006, 0.063, 0.07), (0.16, 0.001, 0.003, 0.054, 0.058), (0.32, 0, 0, 0.048, 0.046)]
THIGH = [(-0.0444, -0.004, 0.0045, 0.088, 0.106), (0.16, 0, 0.003, 0.084, 0.095), (0.4206, 0.007, 0.0005, 0.066, 0.07)]
SHANK = [(-0.0494, 0.008, 0.0005, 0.066, 0.072), (0.12, 0.004, 0.0005, 0.066, 0.071), (0.4206, -0.003, 0.0005, 0.058, 0.062)]
SEAT = [(-0.105, 0.002, 0.11, 0.085, 2.3), (-0.07, 0.005, 0.175, 0.125, 2.35), (-0.0106, 0.01, 0.192, 0.142, 2.4), (0.1344, 0.004, 0.181, 0.14, 2.4)]
TEXTS = [
    ("50", "trouser seat with gluteal masses", "trouser seat with gluteal masses lofted from the gluteal fold to the iliac crest"),
    ("50", "Hosengesäß mit Gesäßmassen", "Hosengesäß mit Gesäßmassen als Lofts von der Gesäßfalte bis zum Beckenkamm"),
    ("100", "four rib hoops per side", "all twelve ribs per side with their costal cartilages and the costal arch (ribs are 12 to 15 mm deep and stand at the 15 mm feature)"),
    ("100", "vier Rippenbögen je Seite", "alle zwölf Rippen je Seite mit ihren Rippenknorpeln und dem Rippenbogen (Rippen sind 12 bis 15 mm hoch und stehen im 15-mm-Mindestmaß)"),
    ("200", "three rib hoops per side, sternum,", "the rib cage as one closed basket from the first rib to the costal margin (ribs and intercostal spaces are below the feature),"),
    ("200", "drei Rippenbögen je Seite, Brustbein,", "der Brustkorb als ein geschlossener Korb von der ersten Rippe bis zum Rippenbogen (Rippen und Zwischenrippenräume liegen unter dem Mindestmaß),"),
    ("200", "The three rib hoops per side end on the sternum through their cartilage.", "The rib basket encloses the thoracic spine."),
    ("200", "Die drei Rippenbögen je Seite enden über ihren Knorpel am Brustbein.", "Der Rippenkorb umschließt die Brustwirbelsäule."),
    ("500", "Open ribs retain the vertebral and sternal attachments of this figure.", "The rib cage is one closed basket around the thoracic spine."),
    ("500", "Offene Rippenzüge erhalten die Ansätze an Wirbelsäule und Brustbein dieser Figur.", "Der Brustkorb ist ein geschlossener Korb um die Brustwirbelsäule."),
]
ADDED = {
    "en": " The sleeve begins with a rounded shoulder cap, thigh and shank run through three sections each so that the knee has no step, and the seat is rounded into the thighs.",
    "de": " Der Ärmel beginnt mit einer gerundeten Schulterkappe, Ober- und Unterschenkel laufen durch je drei Querschnitte, sodass das Knie keine Stufe hat, und das Gesäß ist in die Oberschenkel gerundet.",
}


def resection(solid, rows, sign):
    like = solid["sections"][0]
    solid["sections"] = [{"at": [x, y, sign * z], "rx": rx, "ry": ry, "exponent": like["exponent"], "rotation": like["rotation"]} for x, y, z, rx, ry in rows]


document = json.load(open(SOURCE, encoding="utf-8"))
coarse = document["volumes"]["500"]
body = {solid["id"]: solid for solid in coarse["body"]}
for side, sign in (("l", 1), ("r", -1)):
    assert body[f"sleeve-{side}"]["sections"][0]["at"][2] == sign * -0.02 and body[f"thigh-{side}"]["sections"][0]["at"][2] == 0.0045 * sign, side
    resection(body[f"sleeve-{side}"], SLEEVE, sign)
    resection(body[f"thigh-{side}"], THIGH, sign)
    resection(body[f"shank-{side}"], SHANK, sign)
body["seat"]["sections"] = [{"at": [x, y, 0], "rx": rx, "ry": ry, "exponent": exponent, "rotation": body["seat"]["sections"][0]["rotation"]} for x, y, rx, ry, exponent in SEAT]
for scale, old, new in TEXTS:
    hits = 0
    for decision in document["volumes"][scale]["decisions"]:
        for language in ("en", "de"):
            if old in decision[language]:
                decision[language] = decision[language].replace(old, new)
                hits += 1
    assert hits == 1 or any(new in decision[language] for decision in document["volumes"][scale]["decisions"] for language in ("en", "de")), (scale, old, hits)
first = coarse["decisions"][0]
if "rounded shoulder cap" not in first["en"]:
    first["en"] += ADDED["en"]
    first["de"] += ADDED["de"]
json.dump(document, open(SOURCE, "w", encoding="utf-8"), ensure_ascii=False, separators=(",", ":"))
print("written")
