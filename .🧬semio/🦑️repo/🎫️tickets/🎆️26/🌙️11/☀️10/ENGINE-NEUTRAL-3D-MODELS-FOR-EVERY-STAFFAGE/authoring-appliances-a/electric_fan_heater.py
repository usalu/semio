from lib import *

B = Ctx("body", "abs-cream", "casing", "body", 0.78)
TC = B(region="top-cover", tone=0.6)
OG = Ctx("body", "steel-mesh", "grille", "body", 0.2)
IN = Ctx("body", "pc-heat", "intake", "body", 0.3)
CT = Ctx("body", "pp-dial", "control", "body", 0.2)
KN = CT(bone="knob")
FT = Ctx("body", "rubber-foot", "feet", "body", 0.15)
S = []
Z0, Z1 = -0.115, 0.12

for name, x in (("foot-back", -0.075), ("foot-front", 0.04)):
    S.append(box(name, FT, (x, 0, -0.1), (x + 0.035, 0.016, -0.07), cr=0.004, repeat={"count": 2, "translation": [0, 0, 0.17]}))

S.append(box("housing", B, (-0.09, 0.016, Z0), (0.078, 0.272, Z1), cr=0.02))
FR = 0.095
S.append(ext("housing-front", B, rrect(0.016, Z0, 0.272, Z1, 0.02), "yz", 0.078, FR, holes=[rrect(0.06, -0.105, 0.23, 0.105, 0.004)]))
S.append(box("top-cover", TC, (-0.0903, 0.25, Z0 - 0.0003), (0.0953, 0.272, Z1 + 0.0003), cr=0.01))
S.append(ext("belt", B(role="detail", tone=0.3), rrect(Z0 - 0.0005, -0.0905, Z1 + 0.0005, FR + 0.0005, 0.0205), "zx", 0.0445, 0.0455, holes=[rrect(Z0, -0.09, Z1, FR, 0.02)], maxd=50))

def bez(p, n=8):
    (x0, y0), (x1, y1), (x2, y2), (x3, y3) = p
    return [[(1 - t) ** 3 * x0 + 3 * (1 - t) ** 2 * t * x1 + 3 * (1 - t) * t ** 2 * x2 + t ** 3 * x3, (1 - t) ** 3 * y0 + 3 * (1 - t) ** 2 * t * y1 + 3 * (1 - t) * t ** 2 * y2 + t ** 3 * y3] for t in [k / n for k in range(1, n)]]

outline = [[-0.045, 0.268]] + bez([(-0.045, 0.268), (-0.045, 0.3), (-0.04, 0.31), (-0.02, 0.31)]) + [[-0.02, 0.31], [0.03, 0.31]] + bez([(0.03, 0.31), (0.05, 0.31), (0.055, 0.3), (0.055, 0.268)]) + [[0.055, 0.268]]
opening = rrect(-0.028, 0.278, 0.037, 0.298, 0.008, 4)
S.append(ext("handle", B(tone=0.75), outline, "xy", -0.065, 0.065, holes=[opening]))

S.append(box("outlet-grille", OG, (0.078, 0.06, -0.105), (0.0805, 0.23, 0.105), cr=0.001))
ang = math.degrees(math.atan2(-0.009, 0.019))
fin_len = math.hypot(0.019, 0.009)
for name, y0, count, md in (("louvres-lower", 0.0705, 4, 100), ("louvres-upper", 0.1505, 4, 50)):
    S.append(box(name, OG(role="detail", tone=0.9), (-fin_len / 2, -0.00075, -0.1), (fin_len / 2, 0.00075, 0.1), tr=(0.0885, y0, 0), rot=qz(ang), repeat={"count": count, "translation": [0, 0.02, 0]}, maxd=md))

S.append(box("inlet-plate", IN(tone=0.15), (-0.0915, 0.05, -0.105), (-0.0895, 0.21, 0.105), cr=0.001))
pitch = 0.021667
S.append(box("inlet-slat-bottom", IN(tone=0.3), (-0.098, 0.05, -0.105), (-0.0905, 0.059, 0.105), cr=0.003))
S.append(box("inlet-slats", IN(tone=0.3), (-0.098, 0.061, -0.105), (-0.0905, 0.061 + 0.019667, 0.105), cr=0.003, repeat={"count": 6, "translation": [0, pitch, 0]}))
S.append(box("inlet-slat-top", IN(tone=0.3), (-0.098, 0.191, -0.105), (-0.0905, 0.21, 0.105), cr=0.003))

S.append(cyl("dial", KN, "w", (0, 0, -0.1175), 0.005, 0.024))
S.append(cyl("dial-hub", KN(tone=0.35), "w", (0, 0, -0.1205), 0.001, 0.012))
S.append(box("dial-pointer", KN(role="detail", tone=0.95), (-0.001, 0.006, -0.1215), (0.001, 0.022, -0.121)))

S.append(box("mode-switch", CT, (-0.02, 0.092, -0.119), (0.02, 0.112, Z0), cr=0.0019))
S.append(box("mode-mark", CT(role="detail", tone=0.95), (-0.01, 0.1015, -0.1195), (0.01, 0.1025, -0.119), maxd=50))

D = [
    text("The housing is a rounded box (0.02 m corner radius from plan and front) offset 2.5 mm to +z as the plan and front views draw it, with the dial on the -z side face; a separate front frame (extrude with a hole) forms the outlet opening in which a recessed mesh plate and slanted louvre fins stand, the fins ending at the drawn +x extreme of 0.098 m.",
         "Das Gehäuse ist ein abgerundeter Quader (Eckenradius 0,02 m aus Grundriss und Ansicht), wie in Grundriss und Ansicht gezeichnet 2,5 mm nach +z versetzt, mit dem Drehknopf an der Seitenfläche -z; ein separater Frontrahmen (Extrusion mit Loch) bildet die Ausblasöffnung, in der eine zurückgesetzte Gitterplatte und schräge Lamellen stehen, deren Enden das gezeichnete Maximum +x von 0,098 m erreichen."),
    text("The handle is the drawn side outline extruded 0.13 m along w with the drawn finger opening as a true through-hole (the 2D plan and front rounding of the handle ends is not modelled). The intake is a dark plate behind eight slats whose gaps are the drawn slot lines.",
         "Der Griff ist die gezeichnete Seitenkontur, 0,13 m entlang w extrudiert, mit der gezeichneten Grifföffnung als echtem Durchgangsloch (die Rundung der Griffenden aus Grundriss und Ansicht ist nicht modelliert). Der Ansaugbereich ist eine dunkle Platte hinter acht Lamellen, deren Spalten die gezeichneten Schlitzlinien sind."),
    text("The knob bone carries dial, hub and pointer (revolute 0 to 270 degrees about w); the mode switch is view-only in the 2D drawings and is given an assumed 4 mm relief. The highlight streak is a rendering aid of the drawing and has no solid; lower louvres, belt, hub, mode mark and the upper louvres carry maxDenominator 100 or 50 as in the coarser drawings.",
         "Der Knopfknochen trägt Skala, Nabe und Zeiger (Drehgelenk 0 bis 270 Grad um w); der Modusschalter kommt nur in der 2D-Ansicht vor und erhält ein angenommenes Relief von 4 mm. Der Glanzstreifen ist ein Zeichnungshilfsmittel und hat keinen Körper; untere Lamellen, Gürtel, Nabe, Modusmarke und obere Lamellen tragen maxDenominator 100 bzw. 50 wie in den gröberen Zeichnungen."),
]
write("electric-fan-heater", S, D)
