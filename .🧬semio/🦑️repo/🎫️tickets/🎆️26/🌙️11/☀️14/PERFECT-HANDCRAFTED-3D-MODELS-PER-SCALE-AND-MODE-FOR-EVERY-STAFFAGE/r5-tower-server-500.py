"""🗄️ Round 5, appliances/tower-server: the digital 1:500 body rebuilt as a crisp two-tone massing (steel case block, dark bezel block, rubber plinth) without the 75 mm deep window pockets of round 3; decisions rewritten.

Usage: python3 r5-tower-server-500.py  (edits volumes/appliances/tower-server.json in place)
@see ./r5-brief.md
@see ./r3-appliances-tower-server.md
"""
import json
import pathlib

SRC = pathlib.Path(__file__).with_name("volumes") / "appliances" / "tower-server.json"

BODY = [
    {"id": "plinth", "kind": "box", "bone": "case", "material": "rubber-foot", "role": "body", "tone": 0.2,
     "size": [0.012, 0.4, 0.18], "transform": {"translation": [0.006, 0.0, 0.0]}},
    {"id": "case", "kind": "box", "bone": "case", "material": "chassis-steel", "role": "body", "tone": 0.55,
     "size": [0.438, 0.375, 0.2], "transform": {"translation": [0.231, 0.0375, 0.0]}},
    {"id": "front-bezel", "kind": "box", "bone": "case", "material": "bezel-abs", "role": "body", "tone": 0.2,
     "size": [0.438, 0.075, 0.2], "transform": {"translation": [0.231, -0.1875, 0.0]}},
]

DECISIONS = [
    {"en": "Body: a digital model only, the drawing is not printable at this scale. The pedestal server is 6 x 2.7 x 6 smallest features, so it is told by its massing alone: the light steel case block (rear panel merged into it), the dark ABS bezel block of one feature in front, and the rubber plinth of the four feet. The grille, intake, power supply and fan windows of 1:200 are dropped: at 75 mm they would be pockets as deep as they are wide and read as holes in the block.",
     "de": "Körper: nur ein digitales Modell, die Zeichnung ist in diesem Maßstab nicht druckbar. Der Standserver misst 6 x 2,7 x 6 kleinste Merkmale und wird allein durch seine Masse erkannt: der helle Stahlblock des Gehäuses (die Rückwand darin aufgegangen), vorn der dunkle ABS-Blendenblock von einem Merkmal Tiefe und der Gummisockel der vier Füße. Die Fenster für Gitter, Einlass, Netzteil und Lüfter aus 1:200 entfallen: bei 75 mm wären es Nischen, so tief wie breit, die als Löcher im Block gelesen würden."},
    {"en": "Thermal: sleeve, grille patch, bezel face, the two power supply faces, rear face, the plinth as the feet and the two outlets, each as a skin on the faces of the block where the region lies.",
     "de": "Thermisch: Hülse, Gitterfeld, Blendenfläche, die beiden Netzteilflächen, Rückwand, der Sockel als Füße und die zwei Auslässe, jeweils als Haut auf den Flächen des Blocks, wo die Zone liegt."},
    {"en": "Structure: floor, mainboard wall, two posts and top frame in 30 mm, the plinth on the floor patch of the contact, and two masses: the power supply and everything else.",
     "de": "Tragwerk: Boden, Mainboard-Wand, zwei Pfosten und Dachrahmen in 30 mm, der Sockel auf dem Bodenfeld des Kontakts und zwei Massen: das Netzteil und alles Übrige."},
]

doc = json.loads(SRC.read_text())
vol = doc["volumes"]["500"]
vol["body"] = BODY
vol["decisions"] = DECISIONS
SRC.write_text(json.dumps(doc, indent=1, ensure_ascii=False) + "\n")
print("[r5] tower-server 1:500 body", [s["id"] for s in BODY])
