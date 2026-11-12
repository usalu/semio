import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", ".."))
import authoring_components_lib as L
W, LI, SI = "wall", "lintel", "sill"
F = "masonry-field"
outer = [(0, 0), (3, 0), (3, 2.75), (0, 2.75)]
hole = [(0.625, 2.5), (2.375, 2.5), (2.375, 2.25), (2.125, 2.25), (2.125, 0.91667), (2.2, 0.91667), (2.2, 0.83333), (0.8, 0.83333), (0.8, 0.91667), (0.875, 0.91667), (0.875, 2.25), (0.625, 2.25)]
Z = [-0.2225, -0.1075, 0.0325, 0.2075, 0.2225]
s = []
for id, mat, tone, a, b in [("facing", "brick", 0.55, Z[0], Z[1]), ("fill", "wool", 0.9, Z[1], Z[2]), ("inner-leaf", "block", 0.7, Z[2], Z[3]), ("plaster", "plaster", 0.85, Z[3], Z[4])]:
    s.append(L.ext(id, W, mat, "body", tone, outer, "xy", a, b, F, holes=[hole]))
def band(prefix, bone, x0, length, y0, height, layers, region):
    out = []
    for id, mat, tone, a, b in layers:
        out.append(L.box(prefix + id, bone, mat, "body", tone, [length, height, b - a], [length / 2, height / 2, (a + b) / 2], region))
    return out
lintel = [("shell", "concrete", 0.3, -0.2225, -0.1825), ("ins", "wool", 0.9, -0.1825, -0.1025), ("inner", "concrete", 0.3, -0.1025, 0.0725), ("plaster", "plaster", 0.85, 0.0725, 0.0875)]
s += band("lintel-", LI, 0.625, 1.75, 2.25, 0.25, lintel, "lintel")
sill = [("shell", "concrete", 0.3, -0.2225, -0.1525), ("ins", "wool", 0.9, -0.1525, -0.0725), ("block", "block", 0.7, -0.0725, 0.1025), ("plaster", "plaster", 0.85, 0.1025, 0.1175)]
s += band("sill-", SI, 0.8, 1.4, 0.83333, 0.08334, sill, "sill")
s += [
 L.box("lintel-back", W, "block", "body", 0.7, [1.75, 0.25, 0.12], [1.5, 2.375, 0.1475], F),
 L.box("lintel-back-plaster", W, "plaster", "body", 0.85, [1.75, 0.25, 0.015], [1.5, 2.375, 0.215], F),
 L.box("sill-back", W, "block", "body", 0.7, [1.4, 0.08334, 0.09], [1.5, 0.875, 0.1625], F),
 L.box("sill-back-plaster", W, "plaster", "body", 0.85, [1.4, 0.08334, 0.015], [1.5, 0.875, 0.215], F),
]
L.write("masonry-wall-with-lintel", s, [
 ("The wall field is four z-layers (facing brick 115, mineral wool fill 140, block inner leaf 175, plaster 15 mm), each one extrude of the full wall outline with the window pocket as hole, so every construction layer is its own solid for the thermal view.", "Das Wandfeld besteht aus vier Schichten in z (Vormauerziegel 115, Kerndämmung 140, Blockinnenschale 175, Putz 15 mm); jede ist eine Extrusion des Wandumrisses mit der Fensteröffnung samt Lager als Loch, sodass jede Schicht ein eigener Körper für die thermische Ansicht ist."),
 ("Lintel and sill fill their pockets with their own layer sequence (concrete shell, wool, inner concrete or block, plaster) on their own bones; the remaining wall depth behind them is block and plaster of the masonry field.", "Sturz und Brüstungsband füllen ihre Aussparungen mit der eigenen Schichtfolge (Betonschale, Dämmung, Innenbeton bzw. Block, Putz) an den eigenen Knochen; die restliche Wandtiefe dahinter ist Block und Putz des Mauerwerksfeldes."),
 ("Brick courses and the zigzag fill hatching are 2D texture and are not modelled; the load carrier bone is a load point without volume.", "Ziegelschichten und die Zickzackschraffur der Dämmung sind 2D-Textur und nicht modelliert; der Lastträgerknochen ist ein Lastpunkt ohne Volumen."),
])
