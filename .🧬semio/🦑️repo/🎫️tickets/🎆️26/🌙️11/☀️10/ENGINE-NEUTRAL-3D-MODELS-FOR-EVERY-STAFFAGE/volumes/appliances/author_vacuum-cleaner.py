import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _lib import *


def xrange_at(poly, y):
    xs = []
    for i in range(len(poly)):
        (x0, y0), (x1, y1) = poly[i], poly[(i + 1) % len(poly)]
        if (y0 - y) * (y1 - y) <= 0 and y0 != y1:
            xs.append(x0 + (x1 - x0) * (y - y0) / (y1 - y0))
    return min(xs), max(xs)


def halfz_at(right, y):
    for i in range(len(right) - 1):
        (z0, y0), (z1, y1) = right[i], right[i + 1]
        if (y0 - y) * (y1 - y) <= 0 and y0 != y1:
            return z0 + (z1 - z0) * (y - y0) / (y1 - y0)
    return right[-1][0]


shell_poly = ([(-0.215, 0.045)] + bezier((-0.215, 0.045), (-0.238, 0.045), (-0.252, 0.07), (-0.252, 0.1)) + [(-0.252, 0.17)] + bezier((-0.252, 0.17), (-0.252, 0.19), (-0.238, 0.2), (-0.218, 0.2)) + [(0.2, 0.2)]
              + bezier((0.2, 0.2), (0.23, 0.2), (0.255, 0.175), (0.262, 0.145), n=8) + [(0.262, 0.1)] + bezier((0.262, 0.1), (0.256, 0.07), (0.24, 0.045), (0.215, 0.045)))
shell_right = [(0.08, 0.045)] + bezier((0.08, 0.045), (0.10762, 0.045), (0.13, 0.06739), (0.13, 0.095), n=8) + [(0.13, 0.17)] + bezier((0.13, 0.17), (0.13, 0.18657), (0.11657, 0.2), (0.1, 0.2), n=8)
lid_poly = ([(-0.24, 0.2)] + bezier((-0.24, 0.2), (-0.24, 0.24), (-0.205, 0.268), (-0.15, 0.268), n=8) + [(0.11, 0.268)] + bezier((0.11, 0.268), (0.2, 0.268), (0.248, 0.245), (0.258, 0.2), n=8))
lid_right = bezier((0, 0.268), (0.07, 0.268), (0.12, 0.24), (0.12, 0.2), n=16, first=True)


def sections(poly, right, ys, lo, hi, nexp, minz=0.004):
    out = []
    for y in ys:
        ys_ = min(max(y, lo + 1e-7), hi - 1e-7)
        a, b = xrange_at(poly, ys_)
        hz = max(halfz_at(right, ys_), minz)
        out.append(((r((a + b) / 2), y, 0), hz, (b - a) / 2, nexp))
    return out


shell_ys = [0.045, 0.05, 0.058, 0.07, 0.085, 0.1, 0.12, 0.15, 0.17, 0.18, 0.19, 0.197, 0.2]
lid_ys = [0.2, 0.21, 0.22, 0.23, 0.24, 0.25, 0.258, 0.264, 0.268]
handle_poly = [(-0.07, 0.266), (-0.058, 0.296)] + bezier((-0.058, 0.296), (-0.056, 0.303), (-0.05, 0.305), (-0.04, 0.305), n=4) + [(0.06, 0.305)] + bezier((0.06, 0.305), (0.07, 0.305), (0.076, 0.303), (0.078, 0.296), n=4) + [(0.09, 0.266)]
nozzle_poly = [(-0.09, -0.045), (0.17, -0.045)] + bezier((0.17, -0.045), (0.185, -0.045), (0.19, -0.035), (0.19, -0.025), n=4) + [(0.19, -0.005)] + bezier((0.19, -0.005), (0.19, 0.005), (0.18, 0.012), (0.17, 0.015), n=4) + [(0.06, 0.028), (0.035, 0.05), (-0.035, 0.05), (-0.06, 0.022), (-0.09, -0.01)]


def tyre(radius, half, rim=0.0105):
    return [(-half, radius - 0.01), (-half + 0.0025, radius - 0.002), (-half + 0.008, radius), (half - 0.008, radius), (half - 0.0025, radius - 0.002), (half, radius - 0.01)]


hose = []
for k in range(1, 11):
    bone = "hose-%d" % k
    hose.append(capsule("hose-%d-tube" % k, bone, "hose-pvc", "body", 0.35, "hose", (0, 0, 0), (0.11, 0, 0), 0.019))
    hose.append(revolve("hose-%d-rib" % k, bone, "hose-pvc", "texture", 0.15, "hose", [(0.027, 0.0215), (0.033, 0.0215)], axis=(1, 0, 0), repeat={"count": 2, "translation": [0.05, 0, 0]}, maxd=50))

S = [
    loft("shell", "body", "abs-red", "body", 0.5, "body-shell", sections(shell_poly, shell_right, shell_ys, 0.045, 0.2, 6), along="v"),
    loft("lid", "body", "abs-red", "body", 0.62, "lid", sections(lid_poly, lid_right, lid_ys, 0.2, 0.268, 5), along="v", maxd=100),
    extrude("handle", "body", "abs-red", "body", 0.62, "lid", handle_poly, "xy", -0.04, 0.04, holes=[rrect(0.005, 0.283, 0.09, 0.016, rad=0.007)], maxd=100),
    ellipsoid("rewind-button", "body", "abs-grey", "detail", 0.8, "lid", (0.014, 0.007, 0.014), at=(0.175, 0.256, 0), maxd=50),
    box("exhaust-grille", "body", "abs-grey", "body", 0.25, "exhaust", (0.02, 0.1, 0.18), at=(-0.252, 0.14, 0), corner=0.005, maxd=100),
    box("front-bumper", "body", "abs-grey", "body", 0.25, "body-shell", (0.02, 0.1, 0.1), at=(0.255, 0.11, 0), corner=0.008, maxd=50),
    revolve("rear-wheel", "wheel", "rubber-wheel", "body", 0.2, "wheels", tyre(0.06, 0.015), axis=(0, 0, 1), translate=[0, 0, 0.135], repeat={"count": 2, "translation": [0, 0, -0.27]}),
    revolve("rear-hub", "wheel", "abs-grey", "body", 0.75, "wheels", [(-0.0195, 0.026), (0.0195, 0.026)], axis=(0, 0, 1), translate=[0, 0, 0.135], repeat={"count": 2, "translation": [0, 0, -0.27]}, maxd=100),
    revolve("caster-wheel", "caster", "rubber-wheel", "body", 0.2, "wheels", tyre(0.025, 0.01), axis=(0, 0, 1), translate=[0, 0, 0.05], repeat={"count": 2, "translation": [0, 0, -0.1]}, maxd=100),
    revolve("caster-hub", "caster", "abs-grey", "body", 0.7, "wheels", [(-0.0125, 0.009), (0.0125, 0.009)], axis=(0, 0, 1), translate=[0, 0, 0.05], repeat={"count": 2, "translation": [0, 0, -0.1]}, maxd=50),
] + hose + [
    capsule("cuff", "hose-1", "abs-grey", "body", 0.25, "hose", (-0.03, 0, 0), (0.012, 0, 0), 0.026, maxd=100),
    capsule("grip-sleeve", "grip", "abs-grey", "body", 0.3, "grip", (0, 0, 0), (0.14, 0, 0), 0.026),
    ellipsoid("air-slide", "grip", "abs-grey", "detail", 0.7, "grip", (0.014, 0.007, 0.012), at=(0.06, 0.026, 0), maxd=50),
    capsule("wand-tube", "wand", "tube-steel", "body", 0.8, "wand", (0, 0, 0), (0.78, 0, 0), 0.0175),
    extrude("nozzle", "nozzle", "abs-grey", "body", 0.45, "nozzle", nozzle_poly, "xy", -0.14, 0.14),
    ellipsoid("swivel", "nozzle", "abs-grey", "body", 0.5, "nozzle", (0.028, 0.028, 0.028), maxd=50),
    box("brush", "nozzle", "nylon-bristle", "body", 0.15, "brush", (0.18, 0.0125, 0.28), at=(0.09, -0.03925, 0), maxd=100),
]
D = [dec("The dimensions (1.5 x 0.76 x 0.3 m) describe a posed state (the parked pose folds hose, grip and wand), whereas the rig rest and the authored 1:50 shapes are the straight chain: hose segments, grip, wand and nozzle in a line along x. At rest the volume is therefore about 2.6 m long and 0.305 m high and misses the x and y bounding-box gates by construction; the silhouette gates are met against the authored rest shapes.",
         "Die Abmessungen (1,5 x 0,76 x 0,3 m) beschreiben einen Posenzustand (die Parkpose faltet Schlauch, Griff und Rohr), während Ruhelage des Gerüsts und die gezeichneten 1:50-Formen die gestreckte Kette sind: Schlauchglieder, Griff, Rohr und Düse in einer Linie entlang x. In Ruhe ist das Volumen daher etwa 2,6 m lang und 0,305 m hoch und verfehlt die Hüllquader-Grenzen in x und y systembedingt; die Silhouettengrenzen werden gegen die gezeichneten Ruheformen erfüllt."),
     dec("Shell and lid are lofts of superelliptic sections whose x range comes from the elevation outline and whose half width comes from the front outline (exponents 6 and 5 give the rounded plan corners); the carrying handle is the extrusion of its outline with a through opening; wheels and casters are revolves about the lateral axis in pairs (one solid repeated across z).",
         "Gehäuse und Deckel sind Lofts aus superelliptischen Schnitten, deren x-Bereich der Seitenansichtskontur und deren halbe Breite der Vorderansichtskontur entnommen ist (die Exponenten 6 und 5 ergeben die gerundeten Grundrissecken); der Tragegriff ist die Extrusion seiner Kontur mit durchgehender Öffnung; Räder und Rollen sind Rotationskörper um die Querachse paarweise (ein über z wiederholter Körper)."),
     dec("Each of the ten hose segments is a capsule on its own bone with one repeated pair of rib collars (texture, 1:50 only); the nozzle is the elevation outline extruded across 0.28 m with a ball swivel and a bristle strip proud by 0.5 mm; the exhaust grille width (0.18 m) is an estimate, its louvre lines and the wand joint line are not modelled.",
         "Jedes der zehn Schlauchglieder ist eine Kapsel an einem eigenen Knochen mit einem wiederholten Paar Rippenringe (Textur, nur 1:50); die Düse ist die Seitenansichtskontur über 0,28 m extrudiert mit Kugelgelenk und 0,5 mm überstehendem Borstenstreifen; die Breite des Abluftgitters (0,18 m) ist geschätzt, seine Lamellenlinien und die Rohrfugenlinie werden nicht modelliert.")]
write("vacuum-cleaner", S, D)
