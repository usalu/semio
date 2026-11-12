import sys, os, math
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _lib import *

frames("vintage-car-1929")
C = "chassis"
S = []


def sec(x, bot, top, rx, e=4):
    return ((x, (bot + top) / 2, 0), rx, (top - bot) / 2, e)


for s_, n in ((1, "l"), (-1, "r")):
    S.append(capsule("frame-" + n, C, "exhaust-steel", "body", 0.18, "body-panels", (-1.98, 0.38, s_ * 0.38), (2.08, 0.38, s_ * 0.38), 0.03))
for ax in ("axle-f", "axle-r"):
    S.append(capsule(ax + "-beam", ax, "cast-iron", "detail", 0.3, "brakes", (0, 0, -0.66), (0, 0, 0.66), 0.035))

S.append(loft("tub", C, "paint", "body", 0.6, "body-panels", [sec(-1.985, 0.62, 1.0, 0.46, 3), sec(-1.95, 0.545, 1.1, 0.58, 3.5), sec(-1.88, 0.52, 1.136, 0.62), sec(0.6, 0.52, 1.14, 0.62)]))

slant = (-0.105, 0.43)
sl = math.hypot(*slant)
d = (slant[0] / sl, slant[1] / sl)
x_s = [0.0, 0.0, 1.0]
y_s = [d[0], d[1], 0.0]
z_s = cross3(x_s, y_s)
front_rot = [r(c) for c in qbasis(x_s, y_s, z_s)]
cab = [(0.5, 1.64, 0.595), (0.2, 1.64, 0.6), (-1.5, 1.64, 0.6), (-1.64, 1.6, 0.56), (-1.72, 1.5, 0.5), (-1.8, 1.3, 0.46), (-1.84, 1.17, 0.45)]
secs = [((0.512, 1.385, 0), 0.595, 0.257, 6, front_rot)] + [((x, (1.12 + t) / 2, 0), rx, (t - 1.12) / 2, 6) for x, t, rx in cab[1:]]
S.append(loft("cabin", C, "paint", "body", 0.58, "body-panels", secs))
S.append(loft("roof-slab", C, "fabric-roof", "detail", 0.78, "roof", [((0.6, 1.665, 0), 0.5, 0.035, 5), ((0.45, 1.665, 0), 0.6, 0.035, 5), ((-1.5, 1.665, 0), 0.6, 0.035, 5), ((-1.622, 1.645, 0), 0.585, 0.035, 5), ((-1.7, 1.605, 0), 0.54, 0.03, 5)]))
S.append(loft("windscreen", C, "glass", "glazing", 0.85, "glazing", [((0.575, 1.17, 0), 0.52, 0.01, 8), ((0.47, 1.6, 0), 0.52, 0.01, 8)]))
for s_, n in ((1, "l"), (-1, "r")):
    lo, hi = (0.59, 0.608) if s_ > 0 else (-0.608, -0.59)
    S.append(extrude("window-front-" + n, C, "glass", "glazing", 0.85, "glazing", [(0.51, 1.21), (0.44, 1.58), (-0.27, 1.58), (-0.27, 1.21)], "xy", lo, hi))
    S.append(extrude("window-rear-" + n, C, "glass", "glazing", 0.85, "glazing", [(-0.41, 1.21), (-0.41, 1.58), (-1.07, 1.58), (-1.07, 1.21)], "xy", lo, hi))
    S.append(extrude("window-quarter-" + n, C, "glass", "glazing", 0.85, "glazing", [(-1.17, 1.21), (-1.17, 1.58), (-1.62, 1.58), (-1.74, 1.21)], "xy", lo, hi))
    S.append(capsule("handle-front-" + n, C, "nickel-plate", "detail", 0.8, "interior", (-0.212, 1.04, s_ * 0.62), (-0.088, 1.04, s_ * 0.62), 0.012, maxd=50))
    S.append(capsule("handle-rear-" + n, C, "nickel-plate", "detail", 0.8, "interior", (-0.512, 1.04, s_ * 0.62), (-0.388, 1.04, s_ * 0.62), 0.012, maxd=50))
    S.append(box("running-board-" + n, C, "paint", "body", 0.42, "fenders", (2.0, 0.06, 0.23), at=(0, 0.43, s_ * 0.705), corner=0.01))
    S.append(box("seat-front-back-" + n, C, "upholstery", "detail", 0.3, "interior", (0.1, 0.3, 0.4), at=(0.08, 1.35, s_ * 0.28), corner=0.04, maxd=50))
    S.append(capsule("louvre-" + n, C, "engine-metal", "detail", 0.12, "engine-bay", (0.92, 0.97, s_ * 0.335), (0.92, 1.12, s_ * 0.335), 0.012, repeat={"count": 10, "translation": [0.07, 0, 0]}, maxd=100))
    S.append(ellipsoid("headlamp-bowl-" + n, C, "nickel-plate", "body", 0.74, "fenders", (0.105, 0.11, 0.11), at=(1.78, 0.99, s_ * 0.55)))
    S.append(ellipsoid("headlamp-lens-" + n, C, "glass", "glazing", 0.88, "glazing", (0.02, 0.085, 0.085), at=(1.866, 0.99, s_ * 0.55)))
    S.append(capsule("lamp-stalk-" + n, C, "nickel-plate", "detail", 0.5, "fenders", (1.76, 0.785, s_ * 0.55), (1.76, 0.915, s_ * 0.55), 0.015))
    S.append(capsule("bracket-front-" + n, C, "exhaust-steel", "detail", 0.6, "body-panels", (2.04, 0.418, s_ * 0.37), (2.212, 0.512, s_ * 0.3), 0.02))
    S.append(capsule("bracket-rear-" + n, C, "exhaust-steel", "detail", 0.6, "body-panels", (-1.94, 0.418, s_ * 0.37), (-2.11, 0.512, s_ * 0.3), 0.02))
    mx = 0.675 if s_ > 0 else -0.675

    def arch(prefix, pts, half, wc):
        return loft(prefix + "-" + n, C, "paint", "body", 0.5, "fenders", [((x, y, wc), half, 0.022, 6) for x, y in pts])

    S.append(arch("fender-front", [(0.985, 0.46), (1.06, 0.6), (1.17, 0.72), (1.31, 0.8), (1.47, 0.825), (1.67, 0.81), (1.85, 0.76), (1.99, 0.66), (2.12, 0.49)], 0.175, mx))
    S.append(arch("fender-rear", [(-1.0, 0.46), (-1.045, 0.63), (-1.13, 0.72), (-1.28, 0.8), (-1.46, 0.825), (-1.64, 0.81), (-1.82, 0.755), (-1.95, 0.65), (-2.025, 0.5)], 0.15, s_ * 0.70))

S.append(loft("hood", C, "paint", "body", 0.62, "hood", [sec(0.58, 0.58, 1.17, 0.34), sec(0.9, 0.58, 1.195, 0.34), sec(1.27, 0.58, 1.205, 0.34), sec(1.68, 0.58, 1.186, 0.34), sec(1.78, 0.58, 1.16, 0.34)]))
S.append(box("radiator-shell", C, "nickel-plate", "body", 0.78, "radiator", (0.1, 0.7, 0.44), at=(1.83, 0.91, 0), corner=0.03))
S.append(capsule("radiator-neck", C, "nickel-plate", "detail", 0.78, "radiator", (1.83, 1.26, 0), (1.83, 1.32, 0), 0.018))
S.append(ellipsoid("radiator-cap", C, "nickel-plate", "detail", 0.78, "radiator", (0.05, 0.032, 0.04), at=(1.83, 1.335, 0)))
S.append(capsule("bumper-front-upper", C, "nickel-plate", "body", 0.8, "body-panels", (2.22, 0.662, -0.7), (2.22, 0.662, 0.7), 0.022))
S.append(capsule("bumper-front-lower", C, "nickel-plate", "body", 0.8, "body-panels", (2.22, 0.4, -0.7), (2.22, 0.4, 0.7), 0.022))
S.append(capsule("bumper-front-post", C, "nickel-plate", "body", 0.8, "body-panels", (2.22, 0.4, 0), (2.22, 0.662, 0), 0.022))
S.append(capsule("bumper-rear-upper", C, "nickel-plate", "body", 0.8, "body-panels", (-2.12, 0.662, -0.7), (-2.12, 0.662, 0.7), 0.022))
S.append(capsule("bumper-rear-lower", C, "nickel-plate", "body", 0.8, "body-panels", (-2.12, 0.4, -0.7), (-2.12, 0.4, 0.7), 0.022))
S.append(capsule("bumper-rear-post", C, "nickel-plate", "body", 0.8, "body-panels", (-2.12, 0.4, 0), (-2.12, 0.662, 0), 0.022))
S.append(box("number-plate", C, "paint", "detail", 0.9, "body-panels", (0.03, 0.12, 0.4), at=(-2.0, 0.64, 0), maxd=100))
S.append(box("tail-lamp", C, "paint", "detail", 0.3, "body-panels", (0.05, 0.1, 0.1), at=(-2.005, 0.95, 0.4), maxd=100))
S.append(capsule("fender-lamp", C, "nickel-plate", "detail", 0.85, "fenders", (1.99, 0.57, 0.62), (2.12, 0.67, 0.62), 0.022, maxd=100))
S.append(capsule("exhaust-pipe", C, "exhaust-steel", "detail", 0.35, "exhaust-system", (-1.08, 0.27, -0.3), (0.93, 0.27, -0.3), 0.03))
S.append(capsule("silencer", C, "exhaust-steel", "detail", 0.4, "exhaust-system", (-1.825, 0.27, -0.3), (-1.025, 0.27, -0.3), 0.075, lateral=1.33))
S.append(capsule("tail-pipe", C, "exhaust-steel", "detail", 0.45, "exhaust-system", (-2.105, 0.262, -0.35), (-1.72, 0.262, -0.35), 0.0375))

sw = (0.86, 0.78, -0.5575)
S.append(revolve("spare-tyre", C, "rubber", "body", 0.08, "tyres", tyre_ring(0.31, 0.0575), axis=(0, 0, 1), origin=sw))
S.append(revolve("spare-cover", C, "paint", "detail", 0.62, "body-panels", [(-0.0655, 0.23), (-0.0575, 0.23)], axis=(0, 0, 1), origin=sw))
S.append(revolve("spare-boss", C, "nickel-plate", "detail", 0.8, "body-panels", [(-0.0705, 0.04), (-0.0655, 0.04)], axis=(0, 0, 1), origin=sw))
S.append(capsule("steering-column", C, "exhaust-steel", "detail", 0.3, "interior", (0.43, 1.23, 0.28), (0.32, 1.37, 0.28), 0.018, maxd=50))
S.append(revolve("steering-wheel", C, "exhaust-steel", "detail", 0.3, "interior", [(-0.016, 0.1), (0.016, 0.1)], axis=(0.9, 0.0, 0.43), origin=(0.31, 1.39, 0.28), maxd=50))

for s_, n in ((1, "l"), (-1, "r")):
    for bone, p in (("wheel-f", "f"), ("wheel-r", "r")):
        wc = s_ * 0.6705
        o = [0, 0, wc]
        S.append(revolve(f"{p}{n}-tyre", bone, "rubber", "body", 0.08, "tyres", tyre_ring(0.37, 0.0575, shoulder=0.16), axis=(0, 0, 1), origin=o))
        S.append(revolve(f"{p}{n}-rim", bone, "steel-rim", "detail", 0.55, "tyres", [(-0.0585, 0.25), (0.0585, 0.25)], axis=(0, 0, 1), origin=o))
        S.append(revolve(f"{p}{n}-hub", bone, "nickel-plate", "detail", 0.8, "tyres", sorted([(0.0, 0.065), (s_ * 0.0975, 0.065)]) if s_ > 0 else [(-0.0975, 0.065), (0.0, 0.065)], axis=(0, 0, 1), origin=o))
        S.append(capsule(f"{p}{n}-spoke", bone, "steel-rim", "detail", 0.7, "tyres", (0.07, 0, wc + s_ * 0.0645), (0.245, 0, wc + s_ * 0.0645), 0.011, repeat={"count": 12, "rotationAxis": [0, 0, 1], "rotationRad": r(2 * math.pi / 12)}, maxd=100))

D = [
    dec("The tub and cabin are lofts along the car axis (0.62 m and 0.60 m half width from the plan and front outlines); the windscreen leans 14 degrees, so the front section of the cabin loft is tilted into the screen plane and the screen is a thin loft on it; side windows are glass plates 18 mm proud of the cabin walls.",
        "Wanne und Kabine sind Lofts entlang der Fahrzeugachse (0,62 m und 0,60 m halbe Breite aus Grund- und Vorderriss); die Frontscheibe ist um 14 Grad geneigt, daher ist der vordere Kabinenschnitt in die Scheibenebene gekippt und die Scheibe ein dünner Loft darauf; Seitenfenster sind Glasplatten, 18 mm vor den Kabinenwänden."),
    dec("Fenders are thin arched lofts following the mid-line of the drawn mudguard outline, 0.35 m wide at the front and 0.30 m at the rear (plan), one per wheel; running boards are boxes 0.23 m wide (the front view draws 0.28 m, the plan 0.18 m).",
        "Kotflügel sind dünne gewölbte Lofts entlang der Mittellinie des gezeichneten Schutzblechumrisses, vorn 0,35 m und hinten 0,30 m breit (Grundriss), einer pro Rad; Trittbretter sind Quader von 0,23 m Breite (die Vorderansicht zeichnet 0,28 m, der Grundriss 0,18 m)."),
    dec("Wheels are revolves about the axle at a 1.341 m track: rounded tyre, steel rim disc, protruding nickel hub cap and 12 radial wire spokes (one solid with repeat) on the outer face; left and right are separate solids on the shared wheel bone so both spin in the driving clips.",
        "Die Räder sind Rotationskörper um die Achse bei 1,341 m Spurweite: gerundeter Reifen, Stahlfelgenscheibe, vorstehende Nickel-Nabenkappe und 12 radiale Drahtspeichen (ein Körper mit repeat) auf der Außenseite; links und rechts sind eigene Körper auf dem gemeinsamen Radknochen und drehen in den Fahrtclips gemeinsam."),
    dec("The spare wheel sits on the right side of the hood at w = -0.5575 as drawn in front and plan view; the radiator shell, bumpers (two bars and a post each) and lamps are separate nickel-plated solids.",
        "Das Reserverad sitzt rechts neben der Motorhaube bei w = -0,5575, wie in Vorderansicht und Grundriss gezeichnet; Kühlerschale, Stoßstangen (je zwei Stangen und ein Pfosten) und Lampen sind eigene vernickelte Körper."),
]
write("vintage-car-1929", S, D)
