import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", ".."))
import authoring_components_lib as L
S = "stringer"
STEP = 0.38523
rep = {"count": 12, "translation": [STEP, 0, 0]}
s = [
 L.ext("stringer-web", S, "steel", "body", 0.5, [(0.03867, -0.03115), (0.23733, -0.19118), (4.79634, -0.19118), (4.59768, -0.03115)], "xy", -0.08, 0.08, "stringer-steel"),
 L.ext("tread", S, "timber", "body", 0.6, [(0.10315, 0.17587), (0.36014, -0.03115), (0.38523, 0), (0.12824, 0.20702)], "xy", -0.6, 0.6, "tread-top", repeat=rep),
 L.ext("gusset", S, "steel", "body", 0.3, [(0.26669, 0.04413), (0.33678, -0.01233), (0.32161, -0.03115), (0.20605, -0.03115)], "xy", -0.006, 0.006, "stringer-steel", repeat=rep),
]
for pos, (u, v) in (("a", (0.15463, 0.16008)), ("b", (0.33375, 0.01579))):
    for side, (z0, dz) in (("l", (0.6, 1)), ("r", (-0.6, -1))):
        s.append(L.cyl("screw-%s-%s" % (pos, side), S, "steel", "detail", 0.2, [u, v, z0], [0, 0, dz], 0.003, 0.009, "tread-top", maxD=50, repeat=rep))
L.write("stair-flight", s, [
 ("The flight is one central steel box stringer 160 x 160 mm (extrude of the elevation parallelogram with level-cut ends) carrying twelve 1200 x 330 x 40 mm timber treads; the treads and their gusset brackets are one solid each with a repeat of 12 at the tread pitch along the inclined bone axis.", "Der Lauf besteht aus einem mittigen Stahl-Kastenwangenträger 160 x 160 mm (Extrusion des Parallelogramms der Ansicht mit waagrecht geschnittenen Enden), der zwölf Holztritte 1200 x 330 x 40 mm trägt; Tritte und Knotenbleche sind je ein Körper mit zwölf Wiederholungen im Trittabstand entlang der geneigten Knochenachse."),
 ("The elevation screw circles are modelled as 3 mm screw heads on the two tread end faces; gussets are 12 mm plates under each tread. The plan walking line, arrow and start marker are pictograms and not modelled; the walker carrier bone is a load point without volume.", "Die Schraubenkreise der Ansicht sind als 3 mm hohe Schraubenköpfe an den beiden Trittstirnseiten modelliert; die Knotenbleche sind 12 mm dicke Platten unter jedem Tritt. Gehlinie, Pfeil und Startpunkt der Draufsicht sind Piktogramme und nicht modelliert; der Gehlastträgerknochen ist ein Lastpunkt ohne Volumen."),
])
