from lib import *

SH = Ctx("body", "die-cast-aluminium", "shell", "body", 0.6)
IX = Ctx("body", "abs-plastic", "index", "body", 0.8)
PP = Ctx("body", "steel-pipe", "pipes", "body", 0.5)
PT = IX(bone="pointer", tone=0.1)
Z0, Z1 = -0.07, 0.085
ZI = -0.085
S = []

S.append(box("lower-shell", SH(tone=0.55), (-0.12, -0.11, Z0), (0.12, -0.006, Z1), cr=0.012))
S.append(box("upper-shell", SH, (-0.12, 0.006, Z0), (0.12, 0.11, Z1), cr=0.012))
S.append(ext("seam", SH(role="detail", tone=0.3), rrect(Z0, -0.12, Z1, 0.12, 0.012), "zx", -0.006, 0.006))
S.append(ext("bead", SH(role="detail", tone=0.3), [[Z0 + 0.012 - 0.0005, -0.1085], [Z1 - 0.012 + 0.0005, -0.1085], [Z1 - 0.012 + 0.0005, 0.1085], [Z0 + 0.012 - 0.0005, 0.1085]], "zx", 0.11, 0.1115, holes=[[[Z0 + 0.012 + 0.0015, -0.1065], [Z1 - 0.012 - 0.0015, -0.1065], [Z1 - 0.012 - 0.0015, 0.1065], [Z0 + 0.012 + 0.0015, 0.1065]]], maxd=50))

S.append(ext("index-frame", IX, rrect(-0.095, 0, 0.095, 0.098, 0.006), "xy", ZI, -0.078, holes=[rrect(-0.0755, 0.058, 0.0755, 0.087, 0.002, 2)]))
S.append(box("index-base", IX, (-0.095, 0, -0.078), (0.095, 0.098, Z0), cr=0.004))
S.append(box("counter", IX(tone=0.1), (-0.0755, 0.058, -0.0785), (0.0755, 0.087, -0.0775)))
dg = IX(role="detail", tone=0.92)
S.append(box("digits-black", dg, (-0.0715, 0.0615, -0.0795), (-0.056, 0.0835, -0.0785), repeat={"count": 5, "translation": [0.0182, 0, 0]}, maxd=50))
S.append(box("digits-red", dg(tone=0.5), (0.0195, 0.0615, -0.0795), (0.035, 0.0835, -0.0785), repeat={"count": 3, "translation": [0.0182, 0, 0]}, maxd=50))
S.append(cyl("dial", IX(tone=0.95), "w", (0, 0.018, ZI - 0.00025), 0.0005, 0.028))
S.append(box("ticks", IX(role="detail", tone=0.2), (0.021, 0.0174, ZI - 0.001), (0.027, 0.0186, ZI - 0.0005), repeat={"count": 4, "rotationAxis": [0, 0, 1], "rotationRad": math.pi / 2, "origin": [0, 0.018, 0]}, maxd=50))

S.append(box("plate", SH(tone=0.9), (-0.07, -0.085, Z0 - 0.002), (0.07, -0.04, Z0), cr=0.001))
S.append(box("plate-line-1", SH(role="detail", tone=0.25), (-0.055, -0.1003, Z0 - 0.0005), (0.055, -0.0997, Z0), maxd=50))
S.append(box("plate-line-2", SH(role="detail", tone=0.25), (-0.055, -0.1063, Z0 - 0.0005), (0.04, -0.1057, Z0), maxd=50))

PZ = 0.0075
S.append(pipe("stubs", PP, "v", (-0.055, 0.132, PZ), 0.044, 0.014, 0.009, n=32, repeat={"count": 2, "translation": [0.11, 0, 0]}))
S.append(pipe("unions", PP(tone=0.35), "v", (-0.055, 0.127, PZ), 0.018, 0.02, 0.009, n=32, repeat={"count": 2, "translation": [0.11, 0, 0]}, maxd=100))
S.append(pipe("faces", PP(role="detail", tone=0.4), "v", (-0.055, 0.157, PZ), 0.006, 0.017, 0.009, n=32, repeat={"count": 2, "translation": [0.11, 0, 0]}, maxd=50))

S.append(ext("needle", PT(role="body"), [[-0.006, -0.006], [0.006, -0.006], [0.0025, 0.026], [-0.0025, 0.026]], "xy", ZI - 0.0015, ZI - 0.001))
S.append(cyl("pivot", PT(role="detail"), "w", (0, 0, ZI - 0.00125), 0.0005, 0.006, maxd=50))

D = [
    text("The meter is a two-shell diecast housing (0.24 m wide, 0.22 m tall, 0.155 m deep from plan and front view) with a rolled seam flange and a 15 mm index head on the -z side; depth follows the plan: index front at z -0.085, housing front -0.07, back +0.085. Both pipe connections stand in a row along x at z = 0.0075.",
         "Der Zähler ist ein zweischaliges Druckgussgehäuse (0,24 m breit, 0,22 m hoch, 0,155 m tief nach Grundriss und Ansicht) mit umlaufendem Fugenflansch und einem 15 mm tiefen Zählwerkkopf an der Seite -z; die Tiefe folgt dem Grundriss: Zählwerkfront bei z -0,085, Gehäusefront -0,07, Rückseite +0,085. Beide Rohranschlüsse stehen in einer Reihe entlang x bei z = 0,0075."),
    text("Stubs, nut collars and end faces are hollow rings (bore 0.018 m as the plan draws the dark bores). The index head is a frame with the counter opening plus a base behind it, so the 7 mm counter recess drawn in the front view is real; digits, dial ticks and plate lines are 0.5 mm relief at 1:50 only.",
         "Stutzen, Überwurfmuttern und Stirnflächen sind Hohlringe (Bohrung 0,018 m, wie im Grundriss die dunklen Bohrungen gezeichnet). Der Zählwerkkopf ist ein Rahmen mit der Zählwerköffnung und einer Basis dahinter, sodass die im Aufriss gezeichnete 7 mm tiefe Zählwerknische real ist; Ziffern, Skalenstriche und Schildlinien sind 0,5 mm Relief nur bei 1:50."),
    text("The needle (extruded trapezoid) and its pivot hang on the revolute pointer bone at the dial centre and turn about w in every pose and clip; the plan bead is a 1 mm rim on the top face. The second name-plate line is drawn at y -0.115, 5 mm below the housing bottom (-0.11); it is placed at y -0.106 so the body keeps the 0.27 m height.",
         "Der Zeiger (extrudiertes Trapez) und sein Drehpunkt hängen am Drehknochen Zeiger in der Skalenmitte und drehen in jeder Pose und jedem Clip um w; die Perle im Grundriss ist ein 1 mm hoher Rand auf der Oberseite. Die zweite Schildlinie ist bei y -0,115 gezeichnet, 5 mm unter der Gehäuseunterkante (-0,11); sie liegt bei y -0,106, damit der Körper 0,27 m hoch bleibt."),
]
write("gas-meter", S, D)
