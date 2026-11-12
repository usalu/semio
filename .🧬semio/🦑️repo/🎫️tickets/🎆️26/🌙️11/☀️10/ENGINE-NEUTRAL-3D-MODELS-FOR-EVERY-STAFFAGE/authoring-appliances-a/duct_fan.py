from lib import *

P = Ctx("plate", "abs-plastic", "casing", "body", 0.88)
RG = Ctx("ring", "abs-plastic", "casing", "body", 0.7)
RT = Ctx("rotor", "abs-plastic", "blades", "body", 0.82)
HUB = Ctx("rotor", "motor-housing", "hub", "body", 0.55)
BORE = 0.088
shapes = asset_shapes("duct-fan")
S = []

S.append(ext("plate", P, rrect(-0.11, -0.11, 0.11, 0.11, 0.014), "xy", -0.045, -0.033, holes=[ngon(BORE, 48)]))
screw = P(role="detail", tone=0.45)
for name, v in (("screws-bottom", -0.092), ("screws-top", 0.092)):
    S.append(cyl(name, screw, "w", (-0.092, v, -0.0455), 0.001, 0.008, maxd=50, repeat={"count": 2, "translation": [0.184, 0, 0]}))

S.append(pipe("ring", RG, "w", (0, 0, 0.006), 0.078, 0.1, BORE))
S.append(pipe("lip", RG(tone=0.6), "w", (0, 0, 0.0415), 0.007, 0.104, BORE))
S.append(pipe("seam", RG(role="detail", tone=0.3), "w", (0, 0, -0.015), 0.002, 0.1015, 0.0985, maxd=50))

for i in range(1, 6):
    pts = path_points(shapes["blade-%d" % i]["d"])[0]
    S.append(ext("blade-%d" % i, RT, pts, "xy", -0.0285, -0.0185))
S.append(cyl("hub", HUB, "w", (0, 0, -0.0035), 0.051, 0.034))
S.append(cyl("hub-cap", HUB(role="detail", tone=0.9), "w", (0.004, 0.004, -0.0293), 0.0008, 0.012, maxd=50))
struts = RG(tone=0.6, role="detail")
S.append(cap("strut-1", struts, (0.03, 0, 0.02), (0.09, 0, 0.02), 0.003, maxd=50, rot=None, repeat={"count": 4, "rotationAxis": [0, 0, 1], "rotationRad": math.pi / 2}))

D = [
    text("The fan axis is the lateral axis w with the origin at the centre of the hub in the wall plane (as in the asset notes): rounded-corner wall plate with a round 0.176 m opening at z -0.045..-0.033, the 0.2 m housing ring behind it and a 0.208 m lip at the rear edge, as drawn in plan and front view.",
         "Die Ventilatorachse ist die Querachse w, der Ursprung liegt in der Nabenmitte in der Wandebene (wie in den Notizen des Assets): Wandplatte mit gerundeten Ecken und rundem Ausschnitt von 0,176 m bei z -0,045..-0,033, dahinter der 0,2 m Gehäusering und am hinteren Rand eine 0,208 m Lippe, wie in Grundriss und Ansicht gezeichnet."),
    text("The 2D views give no depth for blades and hub: the five drawn blade outlines are extruded 10 mm along w behind the plate, the 51 mm motor hub carries them and a hub cap marks the front. Four thin motor struts at the rear join hub and housing, so the opening is not a see-through void; they turn with nothing and are therefore on the housing bone.",
         "Die 2D-Ansichten geben keine Tiefe für Schaufeln und Nabe: die fünf gezeichneten Schaufelumrisse sind 10 mm entlang w hinter der Platte extrudiert, die 51 mm lange Motornabe trägt sie und eine Nabenkappe markiert die Vorderseite. Vier dünne Motorstreben hinten verbinden Nabe und Gehäuse; sie stehen still und liegen daher am Gehäuseknochen."),
    text("Blades, hub and cap sit on the revolute rotor bone and turn about w in every pose and clip; the screws, seam bead, hub cap and struts only appear at 1:50 (maxDenominator).",
         "Schaufeln, Nabe und Kappe sitzen am Drehknochen Rotor und drehen in jeder Pose und jedem Clip um w; Schrauben, Fugenwulst, Nabenkappe und Streben erscheinen nur bei 1:50 (maxDenominator)."),
]
write("duct-fan", S, D)
