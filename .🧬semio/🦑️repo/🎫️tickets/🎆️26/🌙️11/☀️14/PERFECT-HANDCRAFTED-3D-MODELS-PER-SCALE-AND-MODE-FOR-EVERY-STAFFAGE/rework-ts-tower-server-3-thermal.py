"""🌡️ Tower server, second round pass 1, stage 3: the bezel skirt closes the thermal skin between bezel face and side skins at 1:200 and 1:500 (a 2.5 mm ABS tube as deep as the bezel slab of the body at that scale). Writes the candidate document from catalog and source."""
import json, os
T = os.path.dirname(os.path.abspath(__file__))
A = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/tower-server/🔣️.json"
SRC = T + "/volumes/appliances/tower-server.json"
OUT = T + "/🗑️generated/r3-tower-server/🔣️.json"
doc = json.load(open(A)); V = json.load(open(SRC))["volumes"]; doc["asset"]["volumes"] = V
for sc, rear in (("200", -0.195), ("500", -0.15)):
    th = V[sc]["thermal"]
    if any(s["id"] == "th-bezel-skirt" for s in th): continue
    i = next(k for k, s in enumerate(th) if s["id"] == "th-bezel-face")
    skirt = dict(th[i], id="th-bezel-skirt", profile=[[-0.1, 0.012], [0.1, 0.012], [0.1, 0.45], [-0.1, 0.45]], holes=[[[-0.0975, 0.4475], [0.0975, 0.4475], [0.0975, 0.0145], [-0.0975, 0.0145]]], sectionFaces=["+v", "-v"])
    skirt["from"], skirt["to"] = -0.225, rear
    th.insert(i + 1, skirt)
    d = next(e for e in V[sc]["decisions"] if e["en"].startswith("Thermal"))
    d["en"] += " The bezel skirt is a 2.5 mm tube as deep as the bezel slab of the body at this scale, so the skin is closed between bezel face and side skins."
    d["de"] += " Die Blendenzarge ist ein 2,5 mm starkes Rohr in der Tiefe der Blendenscheibe des Körpers in diesem Maßstab, sodass die Haut zwischen Blendenfläche und Seitenhäuten geschlossen ist."
json.dump(doc, open(OUT, "w"), ensure_ascii=False, indent=2)
print("written", OUT)
