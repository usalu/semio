import sys
sys.path.insert(0, ".")
from author_69_lib import *
ALL = '--all' in sys.argv  # the first steps were applied once (h100 is not idempotent)
def f(b, v):
    by_id(b, "pad-cover")["size"][1] = 0.03
    p = by_id(b, "page-sheet"); p["size"] = [0.076, 0.076, 0.076]; p["transform"]["translation"] = [0.05, -0.0085, 0]
if ALL: edit("seated-reader", 500, "body", f)
def g(b, v):
    by_id(b, "member-pad")["size"][1] = 0.03
if ALL: edit("seated-reader", 500, "structure", g)
def h50(b, v):
    for s in b:
        if s["id"] in ("hand-l", "hand-r"):
            for x in s["sections"]:
                if x["rx"] > 0.02: x["rx"] = max(x["rx"], 0.034)
if ALL: edit("seated-reader", 50, "body", h50)
def h100(b, v):
    for s in b:
        if s["id"] in ("hand-l", "hand-r"):
            for x in s["sections"]: x["ry"] = round(x["ry"] * 0.92, 5)
if ALL: edit("seated-reader", 100, "body", h100)
def lid(b, v):
    s = by_id(b, "cup-lid")
    if s["profile"][0][1] != 0:
        s["profile"] = [[s["profile"][0][0], 0]] + s["profile"] + [[s["profile"][-1][0], 0]]
if ALL:
    for sc in (50, 100): edit("seated-reader", sc, "body", lid)
def dec(b, v):
    for x in v["decisions"]:
        x["en"] = x["en"].replace("the notebook is a 79 mm slab (the 2D bounding extents of its drawings forbid a thinner one, so it shows blue beside the drawn thin notebook)", "the notebook cover is the drawn 30 mm slab and its loose page a 76 mm block inside the cover's footprint (the page bone has no elevation shape, so the 75 mm minimum holds for it; the 1:500 elevation now draws the near and far forearm and the left hand that 1:200 draws, so the hands are no longer blue)")
        x["de"] = x["de"].replace("das Notizbuch ist eine 79-mm-Platte (die 2D-Hüllmasse seiner Zeichnungen verbieten eine dünnere, daher blau neben dem gezeichneten dünnen Notizbuch)", "der Notizbuchdeckel ist die gezeichnete 30-mm-Platte und sein loses Blatt ein 76-mm-Block innerhalb der Deckelfläche (die Blatt-Bone hat keine Aufrissform, daher gilt für sie das 75-mm-Minimum; der Aufriss 1:500 zeichnet jetzt den nahen und fernen Unterarm und die linke Hand wie 1:200, die Hände sind nicht mehr blau)")
edit("seated-reader", 500, "decisions", dec)
def dec50(b, v):
    b.append({"en": "Author 69: hands widened to 34 mm half-width like the 1:100 fit so that the gap between the thighs in plan keeps the size the drawn plan shows (the product fills plan holes below 36 cm2 at 1:100; 44 cm2 at 1:50 against 31 cm2 drawn made the cross-scale plan line fail); cup lid explicit on the axis; folded loft sections of cowl, neck, head, hair and toe caps repaired (cap planes made parallel to their neighbour).", "de": "Autor 69: Hände auf 34 mm Halbbreite wie in der Anpassung von 1:100 verbreitert, damit die Lücke zwischen den Oberschenkeln im Grundriss die Größe der gezeichneten Lücke behält (das Produkt füllt Löcher unter 36 cm2 bei 1:100; 44 cm2 bei 1:50 gegen 31 cm2 gezeichnet ließen die Cross-Scale-Zeile im Grundriss scheitern); Becherdeckel ausdrücklich auf der Achse; gefaltete Loft-Schnitte von Rollkragen, Hals, Kopf, Haar und Zehenkappen repariert (Kappenebenen parallel zum Nachbarn)."})

edit('seated-reader', 500, 'decisions', dec)
edit('seated-reader', 50, 'decisions', dec50)
