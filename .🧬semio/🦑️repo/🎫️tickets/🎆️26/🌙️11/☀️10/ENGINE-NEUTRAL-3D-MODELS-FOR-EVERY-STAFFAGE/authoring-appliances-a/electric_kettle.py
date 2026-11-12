from lib import *

BS = Ctx("base", "pp-plastic", "plate", "body", 0.5)
SH = Ctx("body", "stainless", "shell", "body", 0.4)
WA = Ctx("water", "water", "water", "body", 0.45)
HD = Ctx("handle", "pp-plastic", "handle", "body", 0.2)
LD = Ctx("lid", "stainless", "lid", "body", 0.45)
S = []

S.append(rev("base", BS, [[0, 0.086], [0.004, 0.09], [0.026, 0.09], [0.03, 0.086]], "v"))

N = 48
S.append(ext("shell", SH, ngon(0.075, N), "zx", 0, 0.175, holes=[ngon(0.073, N)], taper=(1, 70 / 75)))
S.append(pipe("body-shoulder", SH(role="detail", tone=0.3), "v", (0, 0.09, 0), 0.002, 0.0734, 0.0714, n=N, maxd=50))
S.append(lf("spout", SH(tone=0.4), "u", [(-0.055, 0.027, 0.0235, 2.5, 0, 0.1575), (-0.09, 0.0185, 0.0099, 2.5, 0, 0.1725), (-0.116, 0.012, 0.003, 2.5, 0, 0.19)]))
rw = lambda v: 0.075 - 0.005 * v / 0.175
S.append(lf("gauge", WA(bone="body", role="glazing", tone=0.7), "v", [(0.02, 0.002, 0.014, 6, -(rw(0.02) - 0.001), 0), (0.14, 0.002, 0.014, 6, -(rw(0.14) - 0.001), 0)]))
S.append(rev("water", WA, [[0, 0.0722], [0.125, 0.0688]], "v"))

S.append(ext("handle", HD, [[-0.028, 0.08], [0.022, 0.075], [0.027, -0.05], [-0.025, -0.075], [-0.025, -0.052], [0.01, -0.035], [0.007, 0.058], [-0.028, 0.06]], "xy", -0.016, 0.016))

S.append(rev("lid", LD, [[0, 0.066], [0.013, 0.066], [0.016, 0.063]], "v"))
S.append(rev("lid-button", LD(material="pp-plastic", tone=0.2), [[0, 0.012], [0.013, 0.012], [0.016, 0.009]], "v", origin=(0, 0.016, 0)))

D = [
    text("The jug shell is a hollow tapered ring (0.15 m to 0.14 m outer diameter, 2 mm wall, extrude with a hole and a taper), closed by the heating base below and the lid above; the water is a separate solid inside it, so thermal sections can show the water region. The 2D views draw the water behind the shell, the 3D body keeps it inside.",
         "Der Krugmantel ist ein hohler konischer Ring (0,15 m auf 0,14 m Aussendurchmesser, 2 mm Wand, Extrusion mit Loch und Verjüngung), unten durch den Heizsockel und oben durch den Deckel geschlossen; das Wasser ist ein eigener Körper darin, sodass thermische Schnitte die Wasserregion zeigen können. Die 2D-Ansichten zeichnen das Wasser hinter dem Mantel, der 3D-Körper behält es innen."),
    text("The spout is a three-section loft from the plan wedge (0.052 m wide at the wall, 0.024 m at the tip) and the elevation triangle; the handle is the drawn closed strap outline extruded to the 0.032 m width of plan and front (rounded lateral edges are not modelled, the C-grip fills 1:50 well enough).",
         "Die Tülle ist ein Loft aus drei Querschnitten nach dem Grundrisskeil (0,052 m breit an der Wand, 0,024 m an der Spitze) und dem Aufrissdreieck; der Griff ist die gezeichnete geschlossene Bandkontur, auf die 0,032 m Breite von Grundriss und Ansicht extrudiert (gerundete Seitenkanten sind nicht modelliert)."),
    text("The water gauge is drawn only in elevation: a 3 mm strip on the -z wall follows the cone wall. The lid bone (prismatic, up to 5 mm) carries lid and button, so the boiling pose lifts both; the default pose adds 3 mm to the rest height. Shoulder band and handle ridge: only the band is modelled, at 1:50.",
         "Das Wasserstandsglas ist nur im Aufriss gezeichnet: ein 3 mm starker Streifen an der -z-Wand folgt der Kegelwand. Der Deckelknochen (translatorisch, bis 5 mm) trägt Deckel und Knopf, sodass die Siedepose beide hebt; die Standardpose addiert 3 mm zur Ruhehöhe. Schulterband und Griffrippe: nur das Band ist modelliert, bei 1:50."),
]
write("electric-kettle", S, D)
