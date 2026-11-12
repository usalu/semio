import sys, os, math
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _lib import *

frames("delivery-van")
C = "chassis"
S = []
AR = 0.48
FRONT_X, REAR_X, AXY = 1.7, -1.6, 0.356


def bottom(x):
    for ax in (FRONT_X, REAR_X):
        dx = x - ax
        if abs(dx) < AR:
            return AXY + math.sqrt(AR * AR - dx * dx)
    if x < -2.08:
        return 0.38
    if x > 2.6:
        return 0.356 + (x - 2.6) / 0.07 * 0.07
    return AXY


def top(x):
    pts = [(-2.67, 1.9), (-2.6, 1.95), (-2.55, 1.97), (1.12, 1.97), (1.2, 1.93), (1.4, 1.66), (1.6, 1.39), (1.82, 1.09), (2.3, 1.0), (2.55, 0.91), (2.64, 0.88), (2.67, 0.74)]
    for (x0, y0), (x1, y1) in zip(pts, pts[1:]):
        if x0 <= x <= x1:
            return y0 + (y1 - y0) * (x - x0) / (x1 - x0)
    return pts[-1][1]


def width(x):
    pts = [(-2.67, 0.82), (-2.6, 0.9), (-2.55, 0.91), (1.8, 0.91), (2.35, 0.88), (2.6, 0.8), (2.67, 0.65)]
    for (x0, y0), (x1, y1) in zip(pts, pts[1:]):
        if x0 <= x <= x1:
            return y0 + (y1 - y0) * (x - x0) / (x1 - x0)
    return pts[-1][1]


xs = [-2.67, -2.6, -2.2, -2.08]
for ax in (REAR_X, FRONT_X):
    xs += [ax + AR * k for k in (-0.995, -0.97, -0.92, -0.85, -0.75, -0.6, -0.4, -0.2, 0.0, 0.2, 0.4, 0.6, 0.75, 0.85, 0.92, 0.97, 0.995)]
    if ax == REAR_X:
        xs += [-0.8, 0.4, 1.12]
    else:
        xs += [1.9]
xs += [1.4, 1.2, 2.18, 2.3, 2.55, 2.6, 2.64, 2.67]
xs = sorted(set(round(x, 4) for x in xs))
secs = []
for x in xs:
    b, t = bottom(x), top(x)
    if t - b < 0.05:
        t = b + 0.05
    secs.append(((x, (b + t) / 2, 0), width(x), (t - b) / 2, 5, VERT))
S.append(loft("body-shell", C, "paint", "body", 0.65, "body-panels", secs))

S.append(loft("roof-cap", C, "paint", "body", 0.75, "roof", [((-2.55, 1.965, 0), 0.8, 0.025, 8), ((1.12, 1.965, 0), 0.78, 0.025, 8)]))
S.append(loft("windscreen", C, "glass", "glazing", 0.88, "cab-glazing", [((1.79, 1.13, 0), 0.84, 0.014, 8), ((1.19, 1.93, 0), 0.79, 0.014, 8)]))
for s_, n in ((1, "l"), (-1, "r")):
    lo, hi = (0.885, 0.915) if s_ > 0 else (-0.915, -0.885)
    S.append(extrude("door-window-" + n, C, "glass", "glazing", 0.88, "cab-glazing", [(1.6, 1.2), (1.24, 1.78), (0.45, 1.78), (0.45, 1.2)], "xy", lo, hi))
    S.append(ellipsoid("mirror-" + n, C, "plastic-trim", "detail", 0.3, None, (0.07, 0.11, 0.0425), at=(1.98, 1.3, s_ * 0.9525)))
    S.append(capsule("mirror-arm-" + n, C, "plastic-trim", "detail", 0.3, None, (1.95, 1.3, s_ * 0.9), (1.98, 1.3, s_ * 0.93), 0.012, maxd=100))
    S.append(ellipsoid("headlamp-" + n, C, "glass", "glazing", 0.9, None, (0.1, 0.08, 0.14), at=(2.55, 0.76, s_ * 0.64)))
    S.append(box("tail-lamp-" + n, C, "plastic-trim", "detail", 0.3, None, (0.05, 0.5, 0.14), at=(-2.645, 1.1, s_ * 0.77), corner=0.01))
S.append(loft("bumper", C, "paint", "body", 0.45, "body-panels", [((2.3, 0.45, 0), 0.89, 0.09, 4), ((2.6, 0.45, 0), 0.81, 0.09, 4), ((2.665, 0.46, 0), 0.65, 0.085, 4)]))
S.append(box("grille", C, "engine-metal", "detail", 0.3, "engine-bay", (0.02, 0.28, 0.92), at=(2.665, 0.74, 0), corner=0.006))
S.append(capsule("tail-pipe", C, "exhaust-steel", "detail", 0.4, "exhaust-system", (-2.66, 0.42, -0.31), (-2.36, 0.42, -0.31), 0.04, 0.04))
for ax in ("axle-f", "axle-r"):
    S.append(capsule(ax + "-beam", ax, "cast-iron", "detail", 0.45, "brakes", (0, 0, -0.84), (0, 0, 0.84), 0.05, 0.05, maxd=100))
S.append(capsule("exhaust-pipe", C, "exhaust-steel", "detail", 0.4, "exhaust-system", (-2.4, 0.405, -0.31), (-0.9, 0.405, -0.31), 0.03, 0.03, maxd=100))

for s_, n in ((1, "l"), (-1, "r")):
    for bone, p in (("wheel-f", "f"), ("wheel-r", "r")):
        wc = s_ * 0.84
        o = [0, 0, wc]
        S.append(revolve(f"{p}{n}-tyre", bone, "rubber", "body", 0.1, "tyres", tyre_ring(0.356, 0.1, shoulder=0.12), axis=(0, 0, 1), origin=o))
        S.append(revolve(f"{p}{n}-rim", bone, "steel-rim", "detail", 0.65, None, [(-0.09, 0.2), (0.09, 0.2)], axis=(0, 0, 1), origin=o))
        S.append(revolve(f"{p}{n}-hub", bone, "steel-rim", "detail", 0.75, None, sorted([(0.0, 0.055), (s_ * 0.112, 0.055)]) if s_ > 0 else [(-0.112, 0.055), (0.0, 0.055)], axis=(0, 0, 1), origin=o))
        S.append(capsule(f"{p}{n}-nut", bone, "steel-rim", "detail", 0.75, None, (0.09, 0, wc + s_ * 0.092), (0.09, 0, wc + s_ * 0.102), 0.012, 0.012, repeat={"count": 6, "rotationAxis": [0, 0, 1], "rotationRad": r(2 * math.pi / 6)}, maxd=50))
        S.append(revolve(f"{p}{n}-disc", bone, "cast-iron", "detail", 0.45, "brakes", [(s_ * -0.05, 0.14), (s_ * -0.03, 0.14)] if s_ > 0 else [(0.03, 0.14), (0.05, 0.14)], axis=(0, 0, 1), origin=o, maxd=100))

SL = "slider"
S.append(box("slider-panel", SL, "paint", "body", 0.6, "slider-door", (1.05, 1.444, 0.05), at=(0.525, 0.722, -0.885), corner=0.02))
S.append(capsule("handle-slider", SL, "plastic-trim", "detail", 0.2, None, (0.78, 0.764, -0.925), (0.99, 0.764, -0.925), 0.018, 0.018))
S.append(box("slider-track", C, "paint", "body", 0.4, "slider-door", (2.0, 0.03, 0.04), at=(-0.7, 0.5, -0.925), repeat={"count": 2, "translation": [0, 1.3, 0]}, maxd=100))

for bone, s_, n in (("rear-door-r", -1, "r"), ("rear-door-l", 1, "l")):
    S.append(box("rear-leaf-" + n, bone, "paint", "body", 0.6, "rear-doors", (1.35, 0.9, 0.05), at=(0.675, 0.45, 0), corner=0.02))
    S.append(capsule("rear-hinge-" + n, bone, "plastic-trim", "detail", 0.3, None, (0.0, 0, 0), (1.35, 0, 0), 0.015, 0.015, maxd=100))

D = [
    dec("The body shell is one loft along the van axis whose stations are read off the elevation outline (floor 0.356 m, circular wheel arches of 0.48 m radius about both axles, roof 1.97 m, the steep windscreen line, the bonnet and nose); the width follows the plan (0.91 m at the cab and cargo box, rounding to 0.65 m at the nose, 0.82 m at the tail) and a superelliptic section of exponent 5 gives the roof tumblehome of the front view.",
        "Die Karosserie ist ein einziger Loft entlang der Fahrzeugachse, dessen Stationen dem Umriss der Seitenansicht entnommen sind (Boden 0,356 m, kreisrunde Radkästen von 0,48 m Radius um beide Achsen, Dach 1,97 m, die steile Frontscheibenlinie, Haube und Nase); die Breite folgt dem Grundriss (0,91 m an Kabine und Laderaum, auf 0,65 m an der Nase und 0,82 m am Heck abgerundet) und ein superelliptischer Querschnitt mit Exponent 5 ergibt den Dacheinzug der Vorderansicht."),
    dec("The rest frame has both rear leaves swung out 90 degrees behind the van (as in the 2D views), the poses twist them shut; so the volume at rest extends 0.9 m past the 5.34 m body, while the default pose Parked matches the dimensions. Leaves are 50 mm boxes hinged at w = +-0.88 m.",
        "Im Ruhezustand sind beide Hecktürflügel um 90 Grad hinter den Transporter geschwenkt (wie in den 2D-Ansichten), die Posen drehen sie zu; das Volumen reicht im Ruhezustand daher 0,9 m über den 5,34 m langen Aufbau hinaus, während die Standardpose Geparkt zu den Abmessungen passt. Die Flügel sind 50 mm starke Quader mit Scharnier bei w = +-0,88 m."),
    dec("Wheels are revolves about the axle at the 1.68 m track (toroid-like tyre with rounded shoulders, steel rim disc, hub cap, six wheel nuts and a brake disc each); the sliding door panel rides on the slider bone along two rails on the right side; glazing is a tilted loft for the windscreen and 30 mm plates for the cab door windows.",
        "Die Räder sind Rotationskörper um die Achse bei 1,68 m Spurweite (Reifen mit gerundeten Schultern, Stahlfelgenscheibe, Nabenkappe, sechs Radmuttern und je eine Bremsscheibe); das Schiebetürblatt läuft am Schieberknochen auf zwei Schienen an der rechten Seite; die Verglasung ist ein geneigter Loft für die Frontscheibe und 30-mm-Platten für die Kabinenfenster."),
]
write("delivery-van", S, D)
