import sys, os, math
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _lib import *

frames("cargo-bike")
F = "frame"
S = []
AL = "aluminium"


def tube(id, bone, a, b, rr, tone=0.35, region="frame", mat=AL, role="body", **kw):
    return capsule(id, bone, mat, role, tone, region, a, b, rr, rr, **kw)


S.append(flat_tube("main-beam", F, AL, "body", 0.35, "frame", (0.55, 0.32), (1.795, 0.4), 0.03, 0.02))
S.append(tube("head-riser", F, (1.79, 0.39, 0), (1.866, 0.628, 0), 0.016))
S.append(tube("seat-tube", F, (0.55, 0.32, 0), (0.3, 0.99, 0), 0.017))
S.append(tube("seat-post", F, (0.296, 0.975, 0), (0.31, 1.05, 0), 0.012, 0.3, role="detail"))
S.append(tube("bb-shell", F, (0.55, 0.32, -0.05), (0.55, 0.32, 0.05), 0.03))
for s_, n in ((1, "l"), (-1, "r")):
    S.append(tube("chainstay-" + n, F, (0.0, 0.33, s_ * 0.045), (0.55, 0.32, s_ * 0.045), 0.018))
    S.append(tube("seatstay-" + n, F, (0.0, 0.33, s_ * 0.045), (0.34, 0.86, s_ * 0.02), 0.012))
    S.append(tube("rack-support-" + n, F, (-0.02, 0.33, s_ * 0.045), (-0.22, 0.79, s_ * 0.14), 0.008, 0.3, role="detail"))
S.append(slab("rear-rack", F, AL, "detail", 0.3, "frame", (-0.27, 0.8, 0), (0.3, 0.86, 0), 0.02, 0.3, corner=0.004))
S.append(box("rear-light", F, "plastic", "detail", 0.7, "fenders", (0.04, 0.06, 0.06), at=(-0.28, 0.81, 0), corner=0.008))
S.append(extrude("rear-fender", F, "plastic", "detail", 0.2, "fenders", arc_strip((0.0, 0.33), 0.345, 0.358, 25, 150), "xy", -0.04, 0.04))
S.append(tube("rear-fender-stay", F, (0.0, 0.69, 0), (0.0, 0.84, 0), 0.006, 0.2, "fenders", "plastic", "detail", maxd=100))
S.append(loft("saddle", F, "plastic", "body", 0.15, "touch-parts", [((0.158, 1.062, 0), 0.085, 0.02, 2.4), ((0.3, 1.07, 0), 0.09, 0.028, 2.4), ((0.43, 1.06, 0), 0.02, 0.022, 2.4)]))
S.append(loft("battery-case", "battery", "battery-case", "detail", 0.2, "battery-pack", [(loc("battery", (0.537, 0.5, 0)), 0.06, 0.028, 6), (loc("battery", (0.393, 0.88, 0)), 0.06, 0.028, 6)]))
S.append(box("motor-housing", "motor", "motor-case", "detail", 0.3, "drive-motor", (0.14, 0.14, 0.14), corner=0.03))
S.append(tube("steer-rod", F, (0.732, 0.62, 0), (1.858, 0.62, 0), 0.01, 0.3, "frame", AL, "detail"))

CR = "crank"
S.append(tube("crank-axle", CR, (0, 0, -0.1), (0, 0, 0.1), 0.012, 0.3, "frame", "alloy-rim", "detail"))
S.append(tube("arm-n", CR, (0, 0, -0.1), (0.17, 0, -0.1), 0.015, 0.3, "frame", "alloy-rim", "detail"))
S.append(tube("arm-f", CR, (0, 0, 0.1), (-0.17, 0, 0.1), 0.015, 0.3, "frame", "alloy-rim", "detail"))
S.append(revolve("chainring", CR, "alloy-rim", "detail", 0.3, None, [(-0.085, 0.095), (-0.08, 0.095)], axis=(0, 0, 1)))
S.append(box("pedal-a", CR, "plastic", "detail", 0.15, "frame", (0.1, 0.024, 0.09), at=(0.17, 0, -0.145), corner=0.006))
S.append(box("pedal-b", CR, "plastic", "detail", 0.15, "frame", (0.06, 0.02, 0.09), at=(-0.16, 0, 0.145), corner=0.006))

ST = "stand"
S.append(tube("stand-pivot", ST, (0, 0, -0.12), (0, 0, 0.12), 0.012, 0.3, "frame", AL, "detail"))
for s_, n in ((1, "l"), (-1, "r")):
    S.append(tube("stand-leg-" + n, ST, (0, 0, s_ * 0.12), (0.343, 0, s_ * 0.12), 0.012, 0.3, "frame", AL, "detail"))
S.append(tube("stand-foot", ST, (0.343, 0, -0.15), (0.343, 0, 0.15), 0.02, 0.15, None, "plastic", "detail"))

B = "bar"
Lb = lambda p: loc(B, p)
S.append(capsule("bar-stem", B, AL, "body", 0.35, "frame", Lb((0.74, 0.607, 0)), Lb((0.74, 1.06, 0)), 0.0125, 0.0125))
S.append(capsule("bar-cross", B, AL, "body", 0.35, "frame", Lb((0.74, 1.06, -0.213)), Lb((0.74, 1.06, 0.213)), 0.0125, 0.0125))
for s_, n in ((1, "l"), (-1, "r")):
    S.append(capsule("handlebar-" + n, B, "plastic", "body", 0.3, "touch-parts", Lb((0.74, 1.06, s_ * 0.213)), Lb((0.603, 1.12, s_ * 0.255)), 0.0125, 0.0125))
    S.append(capsule("grip-" + n, B, "plastic", "body", 0.15, "touch-parts", Lb((0.6, 1.1, s_ * 0.255)), Lb((0.6, 1.1, s_ * 0.305)), 0.02, 0.02))
    S.append(capsule("brake-lever-" + n, B, AL, "detail", 0.3, None, Lb((0.712, 1.05, s_ * 0.235)), Lb((0.64, 1.09, s_ * 0.255)), 0.005, 0.005, maxd=50))

FK = "fork"
Lf = lambda p: loc(FK, p)
for s_, n in ((1, "l"), (-1, "r")):
    S.append(capsule("fork-leg-" + n, FK, AL, "body", 0.35, "frame", Lf((1.852, 0.622, s_ * 0.06)), Lf((2.1, 0.255, s_ * 0.06)), 0.012, 0.012))
S.append(capsule("fork-crown", FK, AL, "body", 0.35, "frame", Lf((1.85, 0.62, -0.07)), Lf((1.85, 0.62, 0.07)), 0.02, 0.02))
S.append(capsule("steerer", FK, AL, "body", 0.35, "frame", Lf((1.85, 0.62, 0)), Lf((1.82, 0.58, 0)), 0.016, 0.016))
S.append(ellipsoid("head-lamp", FK, "plastic", "detail", 0.9, None, (0.04, 0.03, 0.04), at=Lf((1.95, 0.62, 0))))
S.append(capsule("head-lamp-bracket", FK, "plastic", "detail", 0.5, None, Lf((1.88, 0.62, 0)), Lf((1.93, 0.62, 0)), 0.008, 0.008, maxd=100))
S.append(extrude("front-fender", FK, "plastic", "detail", 0.2, "fenders", [loc(FK, (x, y, 0))[:2] for x, y in arc_strip((2.1, 0.255), 0.277, 0.29, 30, 150)], "xy", -0.04, 0.04))
S.append(capsule("front-fender-stay", FK, "plastic", "detail", 0.2, "fenders", Lf((1.86, 0.6, 0)), Lf((1.92, 0.48, 0)), 0.006, 0.006, maxd=100))

for bone, R, Rr, hub_w, pre, hubr in (("wheel-r", 0.33, 0.28, 0.04, "r", 0.03), ("wheel-f", 0.255, 0.203, 0.04, "f", 0.03)):
    S.append(ring(pre + "-tyre", bone, "rubber", "body", 0.1, "tyres", R - 0.027, 0.027, n=40))
    S.append(ring(pre + "-rim", bone, "alloy-rim", "detail", 0.6, None, Rr - 0.01, 0.01, n=36))
    S.append(revolve(pre + "-hub", bone, "alloy-rim", "detail", 0.45, None, [(-0.035, hubr), (0.035, hubr)], axis=(0, 0, 1)))
    S.append(revolve(pre + "-axle", bone, "alloy-rim", "detail", 0.45, None, [(-0.065, 0.01), (0.065, 0.01)], axis=(0, 0, 1)))
    S.append(revolve(pre + "-disc", bone, "steel-disc", "detail", 0.5, "brakes", [(0.036, 0.08), (0.04, 0.08)], axis=(0, 0, 1)))
    S += spokes(pre + "-spoke", bone, "alloy-rim", 0.6, None, 0.04, Rr - 0.01, 0.035, 8, 0.0017, maxd=100)

BX = "box"
Lx = lambda p: loc(BX, p)
side = [(0.95, 0.35), (1.78, 0.35), (1.839, 0.461), (1.95, 0.52), (1.95, 1.0), (0.95, 1.0)]
for s_, n in ((1, "l"), (-1, "r")):
    lo, hi = (0.295, 0.325) if s_ > 0 else (-0.325, -0.295)
    S.append(extrude("box-wall-" + n, BX, "plywood", "body", 0.7, "box-wood", [Lx((x, y, 0))[:2] for x, y in side], "xy", lo, hi))
S.append(box("box-floor", BX, "plywood", "body", 0.7, "box-wood", (0.83, 0.03, 0.59), at=Lx((1.365, 0.365, 0))))
S.append(box("box-rear-wall", BX, "plywood", "body", 0.7, "box-wood", (0.03, 0.65, 0.59), at=Lx((0.965, 0.675, 0))))
S.append(box("box-front-wall", BX, "plywood", "body", 0.7, "box-wood", (0.03, 0.46, 0.59), at=Lx((1.935, 0.77, 0))))
S.append(slab("box-nose-a", BX, "plywood", "body", 0.7, "box-wood", Lx((1.78, 0.35, 0)), Lx((1.839, 0.461, 0)), 0.03, 0.59))
S.append(slab("box-nose-b", BX, "plywood", "body", 0.7, "box-wood", Lx((1.839, 0.461, 0)), Lx((1.95, 0.52, 0)), 0.03, 0.59))
S.append(box("box-rim-side", BX, "plywood", "detail", 0.5, "box-wood", (0.94, 0.02, 0.03), at=Lx((1.43, 0.99, 0.31)), repeat={"count": 2, "translation": [0, 0, -0.62]}, maxd=100))
S.append(box("box-rim-end", BX, "plywood", "detail", 0.5, "box-wood", (0.03, 0.02, 0.59), at=Lx((1.005, 0.99, 0)), repeat={"count": 2, "translation": [0.89, 0, 0]}, maxd=100))
S.append(box("box-plank", BX, "plywood", "detail", 0.45, None, (0.002, 0.012, 0.59), at=Lx((0.949, 0.65, 0)), repeat={"count": 2, "translation": [0, 0.2, 0]}, maxd=50))

CRT = "crate"
S.append(box("crate-lower", CRT, "plastic", "body", 0.55, "cargo", (0.5, 0.28, 0.52), at=(0.25, 0.14, 0), corner=0.02))
S.append(box("crate-upper", CRT, "plastic", "body", 0.62, "cargo", (0.46, 0.28, 0.5), at=(0.25, 0.42, 0), corner=0.02))

D = [
    dec("The wheel-base frame is a set of tube capsules along the drawn tube lines; the main beam is a flat capsule (0.03 m high, 0.02 m wide, built on the u axis and tilted so that the lateral stretch acts along w), seat and chain stays pair up with the plan offsets (0.045 m, converging to 0.02 m).",
        "Der Radstandsrahmen besteht aus Rohrkapseln entlang der gezeichneten Rohrlinien; der Hauptträger ist eine flache Kapsel (0,03 m hoch, 0,02 m breit, auf der u-Achse gebaut und gekippt, damit die Querstreckung entlang w wirkt), Sitz- und Kettenstreben paaren sich mit den Grundrissabständen (0,045 m, zusammenlaufend auf 0,02 m)."),
    dec("The cargo box is hollow: two 30 mm side walls (extrude of the side outline with the bevelled nose), floor, end walls, two nose panels and an inner rim; the crates are two stacked rounded boxes on the crate bone so they travel with the load-box clip; the long steering rod runs inside the box at y = 0.62 m as drawn.",
        "Die Transportbox ist hohl: zwei 30 mm starke Seitenwände (Extrusion des Seitenumrisses mit abgeschrägter Nase), Boden, Stirnwände, zwei Nasenplatten und ein innerer Rand; die Kisten sind zwei gestapelte, abgerundete Quader am Kistenknochen, sodass sie mit dem Beladeclip mitfahren; die lange Lenkstange verläuft wie gezeichnet bei y = 0,62 m im Inneren der Box."),
    dec("Tyres and rims are capsule rings (one solid each with repeat about the axle); the rear wheel (R 0.33 m) and the small front wheel (R 0.255 m) each have a hub, a disc brake rotor, an axle and 16 dished spokes in two sets; the front wheel hangs on the fork bone.",
        "Reifen und Felgen sind Kapselringe (je ein Körper mit repeat um die Achse); das Hinterrad (R 0,33 m) und das kleine Vorderrad (R 0,255 m) haben je eine Nabe, eine Scheibenbremse, eine Achse und 16 gedishte Speichen in zwei Sätzen; das Vorderrad hängt am Gabelknochen."),
]
write("cargo-bike", S, D)
