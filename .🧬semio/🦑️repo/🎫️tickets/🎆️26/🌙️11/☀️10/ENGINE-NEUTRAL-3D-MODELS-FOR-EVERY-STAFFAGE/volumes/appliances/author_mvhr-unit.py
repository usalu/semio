import sys, os, json, re
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _lib import *

ASSET = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/mvhr-unit/🔣️.json"
a = json.load(open(ASSET))
a = a.get("asset", a)
d = next(s for s in a["representations"]["50"]["geometry"]["elevation"] if s["id"] == "blade-1")["d"]
nums = [float(x) for x in re.findall(r"-?\d+\.?\d*", d)]
blade = [(nums[i], nums[i + 1]) for i in range(0, len(nums), 2)]
if blade[0] == blade[-1]:
    blade = blade[:-1]

circle = lambda cw, cu, rad, n=32: [(cw + rad * math.cos(2 * math.pi * k / n), cu + rad * math.sin(2 * math.pi * k / n)) for k in range(n)]
body_profile = rrect(0, 0.3, 0.6, 0.6, rad=0.025)

S = [
    extrude("casing-wall", "casing", "painted-steel", "body", 0.82, "casing", body_profile, "xy", -0.15, -0.12, holes=[circle(0, 0.3, 0.112)]),
    extrude("casing", "casing", "painted-steel", "body", 0.82, "casing", body_profile, "xy", -0.12, 0.15),
    extrude("window-ring", "casing", "painted-steel", "body", 0.6, "casing", circle(0, 0.3, 0.128), "xy", -0.1525, -0.15, holes=[circle(0, 0.3, 0.112)], maxd=50),
    revolve("window", "casing", "polycarbonate", "glazing", 0.14, "window", [(-0.1495, 0.112), (-0.1515, 0.112)], axis=(0, 0, 1), origin=(0, 0.3, 0), maxd=200),
    revolve("hub", "impeller", "pp-impeller", "body", 0.5, "impeller", [(-0.142, 0.04), (-0.12, 0.04)], axis=(0, 0, 1), maxd=200),
    extrude("blade", "impeller", "pp-impeller", "body", 0.8, "impeller", blade, "xy", -0.141, -0.123, repeat={"count": 9, "rotationAxis": [0, 0, 1], "rotationRad": r(2 * math.pi / 9), "origin": [0, 0, 0]}, maxd=200),
    ellipsoid("led", "casing", "painted-steel", "detail", 0.95, "casing", (0.014, 0.014, 0.002), at=(0.215, 0.1, -0.15), maxd=50),
    capsule("handle", "casing", "painted-steel", "detail", 0.45, "casing", (-0.068, 0.042, -0.15), (0.068, 0.042, -0.15), 0.012, lateral=0.25, maxd=50),
    extrude("spigot-a", "spigots", "galvanised-steel", "body", 0.66, "ducts", circle(0.07, -0.225, 0.049), "zx", 0, 0.062, holes=[circle(0.07, -0.225, 0.045)], repeat={"count": 2, "translation": [0.3, 0, 0]}),
    extrude("spigot-b", "spigots", "galvanised-steel", "body", 0.66, "ducts", circle(-0.07, -0.075, 0.049), "zx", 0, 0.062, holes=[circle(-0.07, -0.075, 0.045)], repeat={"count": 2, "translation": [0.3, 0, 0]}),
    extrude("bead-a", "spigots", "galvanised-steel", "detail", 0.5, "ducts", circle(0.07, -0.225, 0.057), "zx", 0.048, 0.062, holes=[circle(0.07, -0.225, 0.045)], repeat={"count": 2, "translation": [0.3, 0, 0]}, maxd=100),
    extrude("bead-b", "spigots", "galvanised-steel", "detail", 0.5, "ducts", circle(-0.07, -0.075, 0.057), "zx", 0.048, 0.062, holes=[circle(-0.07, -0.075, 0.045)], repeat={"count": 2, "translation": [0.3, 0, 0]}, maxd=100),
]
D = [dec("The four duct spigots read from the plan are circles of 98 mm at x = -0.225, -0.075, 0.075, 0.225 with alternating z = +0.07 and -0.07 (the front view confirms z); each is a hollow tube (an extrusion of an annulus) with a bead ring, built as two solids repeated along x at a pitch of 0.3 m.",
         "Die vier Kanalstutzen aus dem Grundriss sind Kreise von 98 mm bei x = -0,225, -0,075, 0,075, 0,225 mit abwechselnd z = +0,07 und -0,07 (die Vorderansicht bestätigt z); jeder ist ein Hohlrohr (Extrusion eines Kreisrings) mit Sickenring, aufgebaut als zwei Körper, entlang x im Abstand 0,3 m wiederholt."),
     dec("The side wall carrying the window is a separate 30 mm plate with a 224 mm opening, so the impeller (one backward-curved blade read from the elevation path, repeated nine times by 40 degrees about the shaft, plus a hub) is really visible behind the polycarbonate window and turns about its own axis in every pose.",
         "Die Seitenwand mit dem Fenster ist eine eigene 30-mm-Platte mit 224-mm-Öffnung, sodass das Laufrad (eine rückwärts gekrümmte Schaufel aus dem Seitenansichtspfad, neunmal um 40 Grad um die Welle wiederholt, plus Nabe) hinter dem Polycarbonatfenster wirklich sichtbar ist und sich in jeder Pose um seine eigene Achse dreht."),
     dec("The casing is an extrusion of the 25 mm rounded elevation outline across 0.3 m; its edges around the large faces are sharp (the front and plan corner radii of 25 mm are drawn but cost 0.3 % of the silhouette). Handle and LED are 2 to 3 mm proud of the window-side face, the only parts that increase the depth to 0.3025 m; the cover seam lines are not modelled.",
         "Das Gehäuse ist die Extrusion der in der Seitenansicht mit 25 mm gerundeten Kontur über 0,3 m; die Kanten um die großen Flächen sind scharf (die Eckradien von 25 mm in Vorderansicht und Grundriss sind gezeichnet, kosten aber 0,3 % der Silhouette). Griff und LED stehen 2 bis 3 mm vor der Fensterseite und erhöhen die Tiefe auf 0,3025 m; die Deckelnahtlinien werden nicht modelliert.")]
write("mvhr-unit", S, D)
