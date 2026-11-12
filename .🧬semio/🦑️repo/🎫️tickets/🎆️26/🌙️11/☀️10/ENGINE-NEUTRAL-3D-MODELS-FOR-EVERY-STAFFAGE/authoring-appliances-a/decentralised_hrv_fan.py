from lib import *

P = Ctx("plate", "abs-plastic", "casing", "body", 0.88)
RG = Ctx("ring", "abs-plastic", "casing", "body", 0.7)
CO = Ctx("core", "ceramic-honeycomb", "core", "body", 0.62)
RT = Ctx("rotor", "abs-plastic", "blades", "body", 0.82)
HUB = Ctx("rotor", "motor-housing", "hub", "body", 0.55)
BORE = 0.088
shapes = asset_shapes("decentralised-hrv-fan")
S = []

S.append(ext("plate", P, rrect(-0.13, -0.13, 0.13, 0.13, 0.02), "xy", -0.12, -0.10, holes=[ngon(BORE, 48)]))
screw = P(role="detail", tone=0.45)
for name, v in (("screws-bottom", -0.108), ("screws-top", 0.108)):
    S.append(cyl(name, screw, "w", (-0.108, v, -0.12075), 0.0015, 0.007, maxd=50, repeat={"count": 2, "translation": [0.216, 0, 0]}))

S.append(pipe("ring", RG, "w", (0, 0, 0.01), 0.22, 0.098, BORE))
S.append(pipe("rim", RG(tone=0.6), "w", (0, 0, 0.1125), 0.015, 0.104, BORE))
S.append(pipe("cover-joint", RG(role="detail", tone=0.3), "w", (0, 0, -0.085), 0.002, 0.0995, 0.0975, maxd=50))
S.append(pipe("wall-line", RG(role="detail", tone=0.5), "w", (0, 0, 0.02), 0.002, 0.0995, 0.0975, maxd=50))

cells = [hexagon(cx, cy, 0.0098) for cx in [] for cy in []]
centres = []
for sub in path_points(shapes["core-cells"]["d"]):
    centres.append((sum(p[0] for p in sub) / 6, sum(p[1] for p in sub) / 6))
cells = [hexagon(round(cx, 5), round(cy, 5), 0.0098) for cx, cy in centres]
S.append(ext("core", CO, ngon(0.08, 48), "xy", -0.07, -0.02, holes=cells))

for i in range(1, 6):
    pts = path_points(shapes["blade-%d" % i]["d"])[0]
    S.append(ext("blade-%d" % i, RT, pts, "xy", -0.0975, -0.0865))
S.append(cyl("hub", HUB, "w", (0, 0, -0.0845), 0.027, 0.028))
S.append(cyl("hub-cap", HUB(role="detail", tone=0.9), "w", (0.003, 0.003, -0.09875), 0.0005, 0.01, maxd=50))

D = [
    text("The fan axis is the lateral axis w: wall plate (z -0.12..-0.10) with a round opening, a 0.22 m long pipe of 0.196 m outer diameter behind it, an outer rim at z 0.105..0.12, as drawn in plan and front view.",
         "Die Ventilatorachse ist die Querachse w: Wandplatte (z -0,12..-0,10) mit rundem Ausschnitt, dahinter ein 0,22 m langes Rohr mit 0,196 m Aussendurchmesser und ein Aussenkranz bei z 0,105..0,12, wie in Grundriss und Ansicht gezeichnet."),
    text("The 2D views give no depth for core and rotor. Following the painting order of the elevation (blades over the core) the impeller sits nearest the plate (z -0.0975..-0.0865, hub 27 mm deep behind it) and the 50 mm ceramic core follows at z -0.07..-0.02. The core is a disc with the 37 hexagonal cells of the drawing as through holes (cell walls kept by shrinking the drawn hexagons to 78 %).",
         "Die 2D-Ansichten geben keine Tiefe für Kern und Laufrad. Der Malreihenfolge der Ansicht folgend (Schaufeln über dem Kern) sitzt das Laufrad nahe der Platte (z -0,0975..-0,0865, Nabe 27 mm tief dahinter), der 50 mm dicke Keramikkern folgt bei z -0,07..-0,02. Der Kern ist eine Scheibe mit den 37 sechseckigen Zellen der Zeichnung als Durchgangslöcher (Stege durch Verkleinern der gezeichneten Sechsecke auf 78 %)."),
    text("The five blades are the drawn outlines extruded 11 mm along w (flat vanes, no pitch); they and the hub sit on the revolute rotor bone, so every pose and the spinning clips turn them about w. Screws, joint bead, wall gasket and hub cap appear only at 1:50.",
         "Die fünf Schaufeln sind die gezeichneten Umrisse, 11 mm entlang w extrudiert (flache Flügel ohne Anstellwinkel); sie und die Nabe sitzen am Drehknochen Rotor, sodass jede Pose und die Drehclips sie um w drehen. Schrauben, Fugenwulst, Wanddichtung und Nabenkappe erscheinen nur bei 1:50."),
]
write("decentralised-hrv-fan", S, D)
