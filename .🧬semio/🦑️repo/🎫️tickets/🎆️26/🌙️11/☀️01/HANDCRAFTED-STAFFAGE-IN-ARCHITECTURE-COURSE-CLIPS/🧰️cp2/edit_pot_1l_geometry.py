"""🥣 Pot for 1 L of water: 0.172 m outer, 0.10 m tall, 50 mm fill, masses, areas and texts (runs once)."""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[8]
F = ROOT / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/cooking-pot/🔣️.json"
INDEX = ROOT / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔣️.json"
s = F.read_text(encoding="utf-8")


def sub(old, new, count=1):
    global s
    assert s.count(old) == count, (old, s.count(old))
    s = s.replace(old, new)


sub("A 0.22 m stainless stock pot cut open through its axis with 2 L of water", "A 0.17 m stainless saucepan cut open through its axis with 1 L of water")
sub("Ein 0,22 m großer Edelstahl-Kochtopf, durch die Achse aufgeschnitten, mit 2 l Wasser", "Ein 0,17 m großer Edelstahl-Kochtopf, durch die Achse aufgeschnitten, mit 1 l Wasser")
sub("A stainless stock pot of 0.22 m inner width and 0.13 m height with 0.8 mm walls, a 5 mm sandwich base and two side handles reaching to 0.30 m overall, filled with 2 L of water to 57 mm.",
    "A stainless saucepan of 0.16 m inner width (0.172 m outer) and 0.10 m height with 0.8 mm walls, a 5 mm sandwich base and two side handles reaching to 0.25 m overall, filled with 1 L of water to 50 mm.")
sub("Ein Edelstahl-Kochtopf von 0,22 m Innenweite und 0,13 m Höhe mit 0,8 mm Wänden, einem 5 mm starken Sandwichboden und zwei Seitengriffen bis 0,30 m Gesamtbreite, mit 2 l Wasser bis 57 mm gefüllt.",
    "Ein Edelstahl-Kochtopf von 0,16 m Innenweite (0,172 m außen) und 0,10 m Höhe mit 0,8 mm Wänden, einem 5 mm starken Sandwichboden und zwei Seitengriffen bis 0,25 m Gesamtbreite, mit 1 l Wasser bis 50 mm gefüllt.")
sub("Pot and handles are 1.04 kg, the water 2.0 kg;", "Pot and handles are 0.64 kg, the water 1.0 kg;")
sub("wiegen 1,04 kg, das Wasser 2,0 kg;", "wiegen 0,64 kg, das Wasser 1,0 kg;")
sub("A 0.30 m wide stock pot cut open through its axis: two thin walls, a thick base, water to 57 mm and a handle at each side.",
    "A 0.25 m wide saucepan cut open through its axis: two thin walls, a thick base, water to 50 mm and a handle at each side.")
sub("Ein 0,30 m breiter Kochtopf, durch die Achse aufgeschnitten: zwei dünne Wände, ein dicker Boden, Wasser bis 57 mm und je ein Griff an den Seiten.",
    "Ein 0,25 m breiter Kochtopf, durch die Achse aufgeschnitten: zwei dünne Wände, ein dicker Boden, Wasser bis 50 mm und je ein Griff an den Seiten.")
sub("A 3 mm wide cut pot", "A 2.5 mm wide cut pot")
sub("Ein 3 mm breiter aufgeschnittener Topf", "Ein 2,5 mm breiter aufgeschnittener Topf")
sub("A 1.5 mm wide pot with a bar", "A 1.25 mm wide pot with a bar")
sub("Ein 1,5 mm breiter Topf mit einem Balken", "Ein 1,25 mm breiter Topf mit einem Balken")
sub("the asset is only 0.6 mm wide on paper", "the asset is only 0.5 mm wide on paper")
sub("das Gerät nur 0.6 mm breit auf Papier", "das Gerät nur 0.5 mm breit auf Papier")
sub("The 3 kg filled pot stands on its base: a unilateral contact of 0.2 m diameter", "The 1.6 kg filled pot stands on its base: a unilateral contact of 0.15 m diameter")
sub("Der 3 kg schwere gefüllte Topf steht auf seinem Boden: ein einseitiger Kontakt von 0,2 m Durchmesser", "Der 1,6 kg schwere gefüllte Topf steht auf seinem Boden: ein einseitiger Kontakt von 0,15 m Durchmesser")
doc = json.loads(s)
a = doc["asset"]

a["dimensions"] = {"heightM": 0.1, "widthM": 0.25, "depthM": 0.172, "massKg": 1.64}

bones = {b["id"]: b for b in a["rig"]["bones"]}
bones["pot"]["lengthM"] = 0.1
bones["pot"]["mass"].update(kg=0.54, comY=0.025)
bones["handles"].update(y=0.07, lengthM=0.25)
bones["handles"]["mass"]["kg"] = 0.1
bones["water"]["lengthM"] = 0.05
bones["water"]["mass"].update(kg=1, comY=0.025)
bones["surface"]["y"] = 0.05

for anchor in a["anchors"]:
    if anchor["id"] == "base-contact":
        anchor["footprintLengthM"] = 0.15
        anchor["footprintWidthM"] = 0.15

shapes = {k: {sh["id"]: sh for sh in a["representations"][k]["geometry"]["elevation"]} for k in ("50", "100", "200")}


def put(rep, sid, **kw):
    shapes[rep][sid].update(kw)


def poly(rep, sid, pts):
    shapes[rep][sid]["points"] = pts


put("50", "water", x=-0.08, w=0.16, h=0.05)
poly("50", "surface", [[-0.08, 0], [-0.04, 0.002], [0, -0.001], [0.04, 0.002], [0.08, 0]])
put("50", "wall-left", x=-0.086, h=0.1)
put("50", "wall-right", x=0.08, h=0.1)
put("50", "base", x=-0.086, w=0.172)
put("50", "handle-left", x=-0.12, w=0.036)
put("50", "handle-right", x=0.084, w=0.036)

put("100", "water", x=-0.08, w=0.16, h=0.05)
poly("100", "pot", [[-0.088, 0], [0.088, 0], [0.088, 0.1], [0.08, 0.1], [0.08, 0.008], [-0.08, 0.008], [-0.08, 0.1], [-0.088, 0.1]])
put("100", "handle-left", x=-0.124, w=0.04)
put("100", "handle-right", x=0.084, w=0.04)

put("200", "water", x=-0.085, w=0.17, y=0.005, h=0.045)
poly("200", "pot", [[-0.1, 0], [0.1, 0], [0.1, 0.1], [0.085, 0.1], [0.085, 0.012], [-0.085, 0.012], [-0.085, 0.1], [-0.1, 0.1]])
put("200", "handles", x=-0.125, w=0.25)

regs = {r["id"]: r for r in a["thermal"]["regions"]}
for rid, area, length in (("wall", 0.054, 0.1), ("base", 0.0232, 0.172), ("handle", 0.003, 0.035), ("water", 0.0201, 0.16)):
    regs[rid]["areaM2"] = area
    regs[rid]["characteristicLengthM"] = length
regs["water"]["layers"][0]["thicknessM"] = 0.05
for em in a["thermal"]["emitters"]:
    em["diameterM"] = 0.16

F.write_bytes((json.dumps(doc, ensure_ascii=False, indent=2) + "\n").replace("\n", "\r\n").encode("utf-8"))

ix = INDEX.read_text(encoding="utf-8")
old = '"id": "cooking-pot",'
i = ix.index(old)
j = ix.index('"widthM": 0.3,', i)
assert j - i < 400
ix = ix[:i] + ix[i:j].replace('"heightM": 0.13,', '"heightM": 0.1,') + '"widthM": 0.25,' + ix[j + len('"widthM": 0.3,'):]
INDEX.write_bytes(ix.encode("utf-8"))
