import sys, os, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _lib import *

ASSET = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/shower-head/🔣️.json"
a = json.load(open(ASSET))
a = a.get("asset", a)
geo = a["representations"]["50"]["geometry"]
front = next(s for s in geo["front"] if s["id"] == "hose")["points"]
plan = next(s for s in geo["plan"] if s["id"] == "hose")["points"]
O = (0.03, 0.04, 0.0)
curve = [(plan[k][0], front[k][1], plan[k][1]) for k in range(len(plan))]
picks = list(range(0, 37, 3))
pts = [tuple(c - o for c, o in zip(curve[k], O)) for k in picks]

hose = []
last = len(pts) - 2
ramp = {last: 1.0, last - 1: 0.6, last - 2: 0.3}
for k in range(len(pts) - 1):
    wt = ramp.get(k, 0.0)
    kw = {}
    if wt > 0:
        kw["weights"] = [{"bone": "elbow", "weight": r(1 - wt)}, {"bone": "shower", "weight": r(wt)}] if wt < 1 else [{"bone": "shower", "weight": 1}]
    hose.append(capsule("hose-%02d" % k, "elbow", "epdm-hose", "body", 0.55, "hose", pts[k], pts[k + 1], 0.006, **kw))

n = (0.839, -0.545, 0.0)
nn = math.hypot(n[0], n[1])
n = (n[0] / nn, n[1] / nn, 0)
centre = (0.0592, 0.1269, 0)
phi = math.atan2(0.17302 - 0.08076, 0.08913 - 0.02921)
edge = [(0.0135 * math.sin(math.radians(p)), 0.055 + 0.0135 * math.cos(math.radians(p))) for p in range(-90, 91, 30)]
tube_arc = [(0.1 + 0.008 * (1 - math.cos(math.radians(p))), 0.003 + 0.008 * math.sin(math.radians(p))) for p in range(0, 91, 30)]
tube = tube_arc + [(0.79 - (y - 0.1), x) for y, x in reversed(tube_arc)]
tube = [(y, x) if False else (y, x) for y, x in tube]

S = [
    box("bracket-low", "rail", "chrome", "body", 0.6, "rail", (0.05, 0.035, 0.05), at=(0.025, 0.0975, 0), corner=0.004, maxd=100),
    box("bracket-top", "rail", "chrome", "body", 0.6, "rail", (0.05, 0.035, 0.05), at=(0.025, 0.7925, 0), corner=0.004, maxd=200),
    revolve("tube", "rail", "chrome", "body", 0.8, "rail",
            [(0.1 + 0.008 * (1 - math.cos(math.radians(p))), 0.003 + 0.008 * math.sin(math.radians(p))) for p in range(0, 91, 30)]
            + [(0.79 - 0.008 * (1 - math.cos(math.radians(p))), 0.003 + 0.008 * math.sin(math.radians(p))) for p in range(90, -1, -30)],
            axis=(0, 1, 0), origin=(0.06, 0, 0), maxd=200),
    box("elbow", "elbow", "chrome", "body", 0.7, "valve", (0.06, 0.08, 0.06), corner=0.006, maxd=100),
    revolve("lever-boss", "elbow", "chrome", "detail", 0.7, "valve", [(-0.03, 0.012), (-0.0325, 0.011), (-0.0335, 0.0095)], axis=(0, 0, 1), maxd=50),
    capsule("lever", "lever", "chrome", "detail", 0.85, "valve", (0, 0, -0.035), (0.045, 0, -0.035), 0.005, maxd=50),
    box("holder", "holder", "chrome", "body", 0.7, "rail", (0.056, 0.06, 0.056), corner=0.008, maxd=100),
    capsule("handle", "shower", "abs-chrome", "body", 0.82, "handle", (-0.03804, -0.08157, 0), (0.03804, 0.08157, 0), 0.0165, maxd=200),
    box("head", "shower", "abs-chrome", "body", 0.9, "head", (0.137, 0.027, 0.137), at=centre, corner=0.0135, rotate=[0, 0, r(math.sin(phi / 2)), r(math.cos(phi / 2))], maxd=200),
    box("face", "shower", "rubber-nozzle", "detail", 0.4, "head", (0.1, 0.0016, 0.1), at=tuple(c + 0.0135 * k for c, k in zip(centre, n)), corner=0.0007, rotate=[0, 0, r(math.sin(phi / 2)), r(math.cos(phi / 2))], maxd=50),
] + hose
D = [dec("The hose is read from the authored front and plan polylines, which share one parameter: point k gives (x, y) from plan and elevation and z from both. Twelve capsules of 12 mm diameter follow the sampled 3D curve; the last three are blended towards the handset bone so the hose end follows the handle when the head swings.",
         "Der Schlauch wird aus den gezeichneten Vorder- und Grundrisspolylinien gelesen, die einen gemeinsamen Parameter haben: Punkt k liefert (x, y) aus Grundriss und Aufriss und z aus beiden. Zwölf Kapseln von 12 mm Durchmesser folgen der abgetasteten 3D-Kurve; die letzten drei sind zum Brausenknochen hin gemischt, sodass das Schlauchende dem Griff folgt, wenn der Kopf schwenkt."),
     dec("The spray head is a flat square plate: the elevation capsule is its edge-on view (27 mm slab) and the authored extrusion across 137 mm shows a rectangle in front and plan, so it is a fully rounded box turned about w to the capsule direction, with a rubber nozzle plate 1.6 mm thick on the face; the grip is a capsule through the bone origin.",
         "Der Brausekopf ist eine flache quadratische Platte: die Kapsel der Seitenansicht ist ihre Kantenansicht (27 mm dick), die gezeichnete Extrusion über 137 mm ergibt in Vorderansicht und Grundriss ein Rechteck; daher ein rundum abgerundeter Quader, um w in die Kapselrichtung gedreht, mit einer 1,6 mm dicken Gummidüsenplatte auf der Fläche; der Griff ist eine Kapsel durch den Knochenursprung."),
     dec("The rail is a revolve about the vertical axis (plan: circle of 22 mm), the mixer, brackets and holder are rounded boxes of the sizes of the three views; lever and boss exist only at 1:50. The hanging hose drives the declared height of 1.12 m; the authored hose is an open stroke in the 2D views and therefore not part of the closed-shape silhouette comparison.",
         "Die Schiene ist ein Rotationskörper um die Hochachse (Grundriss: Kreis von 22 mm), Mischer, Halter und Gleiter sind abgerundete Quader in den Maßen der drei Ansichten; Hebel und Rosette gibt es nur im Maßstab 1:50. Der hängende Schlauch bestimmt die deklarierte Höhe von 1,12 m; der gezeichnete Schlauch ist in den 2D-Ansichten ein offener Strich und daher nicht Teil des Silhouettenvergleichs geschlossener Formen.")]
write("shower-head", S, D)
