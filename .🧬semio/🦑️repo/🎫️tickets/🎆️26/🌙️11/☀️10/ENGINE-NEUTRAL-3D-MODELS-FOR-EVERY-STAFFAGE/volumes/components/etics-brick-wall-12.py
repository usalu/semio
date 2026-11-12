import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", ".."))
import authoring_components_lib as L
B = "root"; R = "wall-field"
s = [
 L.box("plaster", B, "lime-plaster", "body", 0.9, [0.015, 1, 1], [0.0075, 0.5, 0], R),
 L.box("brick", B, "solid-brick", "body", 0.55, [0.365, 1, 1], [0.1975, 0.5, 0], R),
 L.box("insulation", B, "mineral-wool", "body", 0.8, [0.12, 1, 1], [0.44, 0.5, 0], R),
 L.box("render", B, "mineral-render", "body", 0.85, [0.02, 1, 1], [0.51, 0.5, 0], R),
 L.box("bed-joints", B, "lime-plaster", "detail", 0.3, [0.365, 0.01, 1.004], [0.1975, 0.075, 0], R, maxD=50, repeat={"count": 12, "translation": [0, 0.075, 0]}),
]
L.write("etics-brick-wall-12", s, [
 ("Each construction layer of the wall strip is its own full-width box (plaster, brick, mineral wool, render) in the order of the thermal layers, so the thermal view colours them separately.", "Jede Schicht des Wandstreifens ist ein eigener Quader über die ganze Breite (Putz, Ziegel, Mineralwolle, Putz) in der Reihenfolge der thermischen Schichten, sodass die thermische Ansicht sie getrennt einfärbt."),
 ("Bed joints are 8 mm mortar bands pushed 2 mm proud of the brick strip on both sides, repeated 12 times at 75 mm; only shown at 1:50 like the 2D joints.", "Lagerfugen sind 8 mm breite Mörtelbänder, beidseitig 2 mm über den Ziegelstreifen hinaus, 12 Mal im Abstand von 75 mm; wie die 2D-Fugen nur bei 1:50 sichtbar."),
])
