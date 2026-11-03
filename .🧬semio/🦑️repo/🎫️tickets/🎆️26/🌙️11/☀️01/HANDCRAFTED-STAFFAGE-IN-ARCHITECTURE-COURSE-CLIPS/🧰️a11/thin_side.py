"""Thin the outlines of the water panels in the 1:50 section so the heat colour of the 22 mm plates reads at true scale in a room section."""
import json
import sys

path = sys.argv[1]
with open(path, encoding="utf-8") as handle:
    doc = json.load(handle)
rep = doc["asset"]["representations"]["50"]
ids = {"panel-rear", "panel-front-upper", "panel-front-mid", "panel-front-lower", "panel-upper", "panel-mid", "panel-lower"}
for shape in rep["geometry"]["elevation"]:
    if shape.get("region") in ("panel-rear", "panel-front-upper", "panel-front-mid", "panel-front-lower"):
        shape["penMm"] = 0.1
        print("thin", shape["id"])
add = {"en": " Water panels: the section outline of the 22 mm plates is drawn with the 0.1 mm minimum pen so the heat colour of the plate stays readable when the section is used at true size in a room.", "de": " Wasserplatten: die Schnittkontur der 22-mm-Platten ist mit der kleinsten Feder 0,1 mm gezeichnet, damit die Wärmefarbe der Platte lesbar bleibt, wenn der Schnitt in Originalgröße in einem Raum steht."}
rep["decisions"]["thinElements"] = {k: rep["decisions"]["thinElements"][k] + add[k] for k in ("en", "de")}
json.dump(doc, open(path, "w", encoding="utf-8", newline="\n"), ensure_ascii=False, indent=1)
