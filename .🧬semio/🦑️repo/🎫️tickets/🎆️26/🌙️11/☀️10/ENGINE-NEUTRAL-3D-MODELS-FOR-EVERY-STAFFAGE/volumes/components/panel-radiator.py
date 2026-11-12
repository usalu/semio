import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", ".."))
import authoring_components_lib as L
R, V = "radiator", "valve-head"
PS, BR = "painted-steel", "brass"
def fin(v0, v1, pts):
    d = 0.0004
    up = [(u, v + d) for u, v in pts]
    dn = [(u, v - d) for u, v in reversed(pts)]
    return up + dn
rear = [(0.03 + 0.050455 * k, -0.022 if k % 2 == 0 else -0.05) for k in range(12)] + [(0.585, -0.05)]
rear = [(0.03, -0.022), (0.08045, -0.05), (0.13091, -0.022), (0.18136, -0.05), (0.23182, -0.022), (0.28227, -0.05), (0.33273, -0.022), (0.38318, -0.05), (0.43364, -0.022), (0.48409, -0.05), (0.53455, -0.022), (0.585, -0.05)]
front = [(0.03, -0.05), (0.08045, -0.078), (0.13091, -0.05), (0.18136, -0.078), (0.23182, -0.05), (0.28227, -0.078), (0.33273, -0.05), (0.38318, -0.078), (0.43364, -0.05), (0.48409, -0.078), (0.53455, -0.05), (0.585, -0.078)]
s = [
 L.box("panel-rear", R, PS, "body", 0.7, [0.585, 0.022, 1.0], [0.2925, -0.011, 0], "panel-rear", corner=0.004),
 L.box("panel-front-lower", R, PS, "body", 0.7, [0.195, 0.022, 1.0], [0.0975, -0.089, 0], "panel-front-lower", corner=0.004),
 L.box("panel-front-mid", R, PS, "body", 0.7, [0.195, 0.022, 1.0], [0.2925, -0.089, 0], "panel-front-mid", corner=0.004),
 L.box("panel-front-upper", R, PS, "body", 0.7, [0.195, 0.022, 1.0], [0.4875, -0.089, 0], "panel-front-upper", corner=0.004),
 L.ext("convector-rear", R, PS, "detail", 0.5, fin(0, 0, rear), "xy", -0.47, 0.47, "convector"),
 L.ext("convector-front", R, PS, "detail", 0.5, fin(0, 0, front), "xy", -0.47, 0.47, "convector"),
 L.box("end-cap", R, PS, "body", 0.5, [0.585, 0.056, 0.03], [0.2925, -0.05, 0.485], "panel-rear", repeat={"count": 2, "translation": [0, 0, -0.97]}),
 L.box("bottom-cap", R, PS, "body", 0.5, [0.003, 0.1, 1.0], [0.0015, -0.05, 0], "panel-rear"),
 L.box("top-grille", R, PS, "detail", 0.35, [0.015, 0.1, 1.0], [0.5925, -0.05, 0], "top-grille"),
 L.box("bracket-l", R, PS, "detail", 0.4, [0.012, 0.035, 0.04], [0.056, 0.0175, 0.4], "bracket", repeat={"count": 2, "translation": [0.42, 0, 0]}),
 L.box("bracket-r", R, PS, "detail", 0.4, [0.012, 0.035, 0.04], [0.056, 0.0175, -0.4], "bracket", repeat={"count": 2, "translation": [0.42, 0, 0]}),
 L.cyl("supply-pipe-l", R, BR, "detail", 0.3, [-0.11, -0.055, -0.335], [1, 0, 0], 0.07, 0.011, "valve"),
 L.cyl("supply-pipe-r", R, BR, "detail", 0.3, [-0.11, -0.055, 0.395], [1, 0, 0], 0.07, 0.011, "valve"),
 L.box("valve-body-l", R, BR, "detail", 0.3, [0.06, 0.044, 0.07], [-0.03, -0.055, -0.335], "valve"),
 L.box("valve-body-r", R, BR, "detail", 0.3, [0.06, 0.044, 0.07], [-0.03, -0.055, 0.395], "valve"),
 L.cyl("lockshield-cap", R, BR, "detail", 0.3, [-0.033, -0.055, 0.43], [0, 0, 1], 0.032, 0.021, "valve"),
 L.rev("valve-head", V, BR, "detail", 0.3, [(0, 0.018), (0.0035, 0.0265), (0.012, 0.03), (0.068, 0.03), (0.0765, 0.0265), (0.08, 0.018)], [0, 0, 1], [0, 0, -0.45], region="valve"),
 L.box("valve-index", V, BR, "detail", 0.1, [0.004, 0.027, 0.002], [0, 0.0135, -0.451], "valve", maxD=50),
]
L.write("panel-radiator", s, [
 ("A type 22 panel radiator is built from its thermal regions: a 22 mm rear panel, three 22 mm front panel sections of 195 mm (lower, mid, upper, so the thermal view shows the vertical temperature gradient), two thin zigzag convector fins between them, end caps, the top grille plate and the bottom cap. Elevation panels end at 585 mm; the front view draws 565 mm (2D inconsistency of 20 mm), the elevation values are used.", "Ein Plattenheizkörper Typ 22 ist aus seinen thermischen Regionen aufgebaut: eine 22 mm Rückplatte, drei 22 mm Frontplattenabschnitte von 195 mm (unten, Mitte, oben, damit die thermische Ansicht den vertikalen Temperaturverlauf zeigt), zwei dünne Zickzack-Konvektorlamellen dazwischen, Endkappen, die Abdeckgitterplatte und die untere Kappe. Die Platten enden in der Ansicht bei 585 mm; die Vorderansicht zeichnet 565 mm (2D-Widerspruch von 20 mm), die Werte der Ansicht sind verwendet."),
 ("The thermostatic head is a revolve about the lateral axis on the rotating valve-head bone, so the knob keeps its shape in every pose; a small dark index ridge on its outer end face makes the rotation (0 to 270 degrees) visible. Its centre follows the elevation (bone origin 85 mm above the floor); the front view draws it 15 mm lower (2D inconsistency).", "Der Thermostatkopf ist ein Rotationskörper um die Querachse am drehenden Knochen valve-head, sodass der Knopf in jeder Pose seine Form behält; ein kleiner dunkler Indexsteg an der äusseren Stirnfläche macht die Drehung (0 bis 270 Grad) sichtbar. Sein Mittelpunkt folgt der Ansicht (Knochenursprung 85 mm über dem Boden); die Vorderansicht zeichnet ihn 15 mm tiefer (2D-Widerspruch)."),
 ("Left and right valve bodies sit on 22 mm brass pipes rising from the floor; the right one carries the lockshield cap. Grille slots, convector hatching and seam lines are 2D texture and not modelled.", "Linker und rechter Ventilkörper sitzen auf 22 mm Messingrohren vom Boden; der rechte trägt die Rücklaufverschraubungskappe. Gitterschlitze, Konvektorschraffur und Nahtlinien sind 2D-Textur und nicht modelliert."),
])
