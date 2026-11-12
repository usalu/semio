from lib import *

BS = Ctx("base", "powder-steel", None, "body", 0.35)
CP = Ctx("cap", "cap-plastic", "top-cap", "body", 0.78)
CN = Ctx("connections", "brass", "connections", "body", 0.7)
R = 0.3
S = []

for name, x in (("feet-left", -0.25), ("feet-right", 0.18)):
    S.append(box(name, BS, (x, 0, -0.25), (x + 0.07, 0.06, -0.18), repeat={"count": 2, "translation": [0, 0, 0.43]}))
S.append(cyl("base-ring", BS(tone=0.4), "v", (0, 0.0525, 0), 0.025, 0.28, maxd=50))

for i, t in ((4, 0.9), (3, 0.88), (2, 0.86), (1, 0.84)):
    S.append(rev("zone-%d" % i, Ctx("zone-%d" % i, "foam-jacket", "zone-%d" % i, "body", t), [[0, R], [0.33, R]], "v"))

dome = [[0.1 * math.sin(math.radians(a)), 0.3 * math.cos(math.radians(a))] for a in range(0, 91, 10)]
S.append(rev("cap", CP, dome, "v"))

S.append(cyl("hot-out", CN, "v", (0.2039, 1.454, 0), 0.048, 0.015))
nut_prof = hexagon(0, 0, 0.019, 0)
for side, xs, ys, cnt, step, nm in (("left", -0.315, 1.1977, 2, -0.4125, "left"), ("left-low", -0.315, 0.1893, 1, 0, "left-low"), ("right", 0.315, 0.9227, 2, -0.4125, "right")):
    rep = {"count": cnt, "translation": [0, step, 0]} if cnt > 1 else None
    S.append(cyl("stubs-" + nm, CN, "u", (xs, ys, 0), 0.03, 0.012, repeat=rep))
    xn = -0.324 if xs < 0 else 0.324
    S.append(ext("nuts-" + nm, CN(tone=0.45), [[ys + a, b] for a, b in hexagon(0, 0, 0.019, 0)], "yz", xn - 0.006, xn + 0.006, repeat=rep, maxd=50))

Z2 = Ctx("zone-2", "window-glass", None, "detail", 0.95)
S.append(cyl("dial", Z2, "w", (0, 0.165, -0.2992), 0.0015 + 0.0015, 0.05, maxd=100))
for name, p, q in (("tick-cold", (-0.0319, 0.1766), (-0.0395, 0.1794)), ("tick-mid", (0, 0.199), (0, 0.207)), ("tick-hot", (0.0319, 0.1766), (0.0395, 0.1794))):
    S.append(cap(name, Z2(tone=0.4, material="foam-jacket"), (p[0], p[1], -0.3004), (q[0], q[1], -0.3004), 0.0008, maxd=50))
S.append(cap("needle", Ctx("needle", "window-glass", None, "detail", 0.2), (0, 0, -0.3012), (0, 0.036, -0.3012), 0.0015, maxd=50))

half = 0.68
S.append(rev("label", Ctx("zone-3", "foam-jacket", "zone-3", "detail", 0.4), [[0.085, 0.3008], [0.185, 0.3008]], "v", sweep=half, rot=qy(math.degrees(math.pi / 2 - half / 2)), maxd=50))
S.append(cap("zip", Ctx("zone-3", "foam-jacket", "zone-3", "detail", 0.4), (0.22, 0.03, -0.2043), (0.22, 0.3, -0.2043), 0.0015, maxd=50))

D = [
    text("The cylinder is a stack of four foam-jacket cylinders of 0.6 m diameter and 0.33 m height, one per thermal zone bone (zone-4 at the bottom), under a 0.1 m elliptical dome; the plan confirms the round footprint. The four 0.07 m square feet sit at (+-0.215, +-0.215), the only layout that gives the two feet seen in both side views.",
         "Der Speicher ist ein Stapel aus vier Schaumdämmzylindern von 0,6 m Durchmesser und 0,33 m Höhe, einer je thermischer Zonen-Knochen (Zone 4 unten), unter einer 0,1 m hohen elliptischen Haube; der Grundriss bestätigt die runde Grundfläche. Die vier quadratischen Füsse von 0,07 m stehen bei (+-0,215, +-0,215), die einzige Anordnung, die in beiden Seitenansichten je zwei Füsse ergibt."),
    text("The 2D plan omits the feet that stick out beyond the round jacket at the diagonals; they are kept because the elevation and front view both draw them. All brass connections lie on the x axis (plan and front put them at z = 0): three on the -x side, two on +x, each a 30 mm stub with a hex nut at the outer end, plus the vertical hot-water outlet at x = 0.204 on the dome.",
         "Der 2D-Grundriss lässt die Füsse weg, die auf den Diagonalen über den runden Mantel hinausragen; sie bleiben, weil Aufriss und Ansicht sie beide zeichnen. Alle Messinganschlüsse liegen auf der x-Achse (Grundriss und Ansicht setzen sie bei z = 0): drei auf der Seite -x, zwei auf +x, je ein 30 mm langer Stutzen mit Sechskantmutter am äusseren Ende, dazu der senkrechte Warmwasserabgang bei x = 0,204 auf der Haube."),
    text("The thermometer dial is a flush glass disc on the -z wall with three tick bars and a needle on the revolute needle bone (turns about w). The label is a 0.68 rad partial revolve (a sector of the jacket cylinder, rotated to face -z) so it follows the curved wall; the zip is a vertical bar where the elevation and front view agree (x 0.22, z -0.204). The dome seam ring is not modelled.",
         "Das Thermometer ist eine bündige Glasscheibe an der Wand -z mit drei Skalenstrichen und einer Nadel am Drehknochen Nadel (dreht um w). Das Etikett ist ein Teilrotationskörper von 0,68 rad (ein Sektor des Mantelzylinders, nach -z gedreht), der der gekrümmten Wand folgt; der Reissverschluss ist ein senkrechter Balken dort, wo Aufriss und Ansicht übereinstimmen (x 0,22, z -0,204). Der Nahtring der Haube ist nicht modelliert."),
]
write("hot-water-cylinder", S, D)
