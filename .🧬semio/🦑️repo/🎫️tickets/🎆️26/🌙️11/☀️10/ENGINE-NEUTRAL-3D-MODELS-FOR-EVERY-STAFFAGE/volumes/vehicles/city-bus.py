import sys, os, math
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _lib import *

frames("city-bus")
B = "body"
S = []
AX_F, AX_R, AXY, AR = 9.35, 3.45, 0.48, 0.64
W = 1.275


def bottom(x):
    for ax in (AX_F, AX_R):
        dx = x - ax
        if abs(dx) < 0.614:
            return AXY + math.sqrt(AR * AR - dx * dx)
    if x < 0.05:
        return 0.478 - (0.05 - x) * 0.0
    if x > 11.85:
        return 0.3 + (x - 11.85) / 0.15 * 0.13
    return 0.3


def interp(pts, x):
    for (x0, y0), (x1, y1) in zip(pts, pts[1:]):
        if x0 <= x <= x1:
            return y0 + (y1 - y0) * (x - x0) / (x1 - x0) if x1 > x0 else y1
    return pts[-1][1]


TOP = [(0.0, 2.6), (0.06, 2.8), (0.265, 2.925), (1.0, 2.95), (11.2, 2.95), (11.62, 2.915), (11.72, 2.848), (11.78, 2.7), (12.0, 0.95)]
xs = [0.0, 0.05, 0.2, 0.6, 1.0, 2.2, 2.7]
for ax in (AX_R, AX_F):
    xs += [ax + 0.614 * k for k in (-0.995, -0.97, -0.92, -0.85, -0.75, -0.6, -0.4, -0.2, 0.0, 0.2, 0.4, 0.6, 0.75, 0.85, 0.92, 0.97, 0.995)]
xs += [4.4, 5.6, 6.8, 8.0, 10.2, 11.0, 11.62, 11.72, 11.78, 11.85, 11.95, 12.0]
xs = sorted(set(round(x, 4) for x in xs))
secs = []
for x in xs:
    b, t = bottom(x), interp(TOP, x)
    if x >= 11.78:
        b = 0.3 if x < 11.85 else bottom(x)
    if t - b < 0.04:
        t = b + 0.04
    rx = 1.24 if x < 0.02 or x > 11.99 else W
    secs.append(((x, (b + t) / 2, 0), rx, (t - b) / 2, 12, VERT))
S.append(loft("body-shell", B, "paint", "body", 0.7, "body-panels", secs))
S.append(loft("roof-panel", B, "paint", "detail", 0.8, "roof", [((0.3, 2.935, 0), 1.2, 0.045, 10, VERT), ((1.0, 2.935, 0), 1.2, 0.045, 10, VERT), ((11.0, 2.935, 0), 1.2, 0.045, 10, VERT), ((11.4, 2.915, 0), 1.2, 0.045, 10, VERT), ((11.6, 2.89, 0), 1.18, 0.04, 10, VERT)]))

BT = "battery"
Lb = lambda p: loc(BT, p)
S.append(loft("battery-pack", BT, "battery-housing", "body", 0.55, "battery-pack", [(Lb((x, yc, 0)), rx, ry, 6, VERT) for x, yc, ry, rx in ((1.0, 2.99, 0.015, 1.0), (1.12, 3.05, 0.07, 1.075), (7.1, 3.05, 0.07, 1.075), (7.3, 2.99, 0.015, 1.0))]))
HV = "hvac"
Lh = lambda p: loc(HV, p)
S.append(loft("hvac-unit", HV, "hvac-housing", "body", 0.6, "hvac-unit", [(Lh((x, yc, 0)), rx, ry, 6, VERT) for x, yc, ry, rx in ((8.0, 2.99, 0.015, 0.95), (8.1, 3.06, 0.08, 1.0), (10.3, 3.06, 0.08, 1.0), (10.5, 2.99, 0.015, 0.95))]))
S.append(box("hvac-louvre", HV, "hvac-housing", "detail", 0.3, None, (0.03, 0.012, 1.7), at=Lh((8.6, 3.145, 0)), repeat={"count": 9, "translation": [0.15625, 0, 0]}, maxd=50))

wins_lo = [(0.3, 1.45), (1.55, 2.7), (2.8, 4.1), (4.2, 5.35), (5.45, 6.25), (7.55, 8.65), (8.75, 9.85), (9.95, 10.25)]
wins_hi = [(0.3, 1.45), (1.55, 2.7), (2.8, 4.1), (4.2, 5.35), (5.45, 6.25), (6.35, 7.45), (7.55, 8.65), (8.75, 9.85), (9.95, 10.25), (10.35, 11.45)]
GL = 1.2765
for i, (a, b) in enumerate(wins_lo):
    S.append(box(f"side-window-{i + 1}", B, "glass", "glazing", 0.8, "glazing", (b - a, 1.2, 0.0135), at=((a + b) / 2, 1.8, GL), repeat={"count": 2, "translation": [0, 0, -2 * GL]}))
for i, (a, b) in enumerate(wins_hi):
    S.append(box(f"upper-window-{i + 1}", B, "glass", "glazing", 0.7, "glazing", (b - a, 0.3, 0.0135), at=((a + b) / 2, 2.63, GL), repeat={"count": 2, "translation": [0, 0, -2 * GL]}))
S.append(box("side-window-door-gap-m", B, "glass", "glazing", 0.8, "glazing", (1.1, 1.2, 0.0135), at=(6.9, 1.8, GL)))
S.append(box("side-window-door-gap-f", B, "glass", "glazing", 0.8, "glazing", (1.1, 1.2, 0.0135), at=(10.9, 1.8, GL)))
S.append(box("side-window-front", B, "glass", "glazing", 0.8, "glazing", (0.3, 1.2, 0.0135), at=(11.65, 1.8, GL), repeat={"count": 2, "translation": [0, 0, -2 * GL]}))

slant = ((11.99, 1.2), (11.8, 2.68))
S.append(loft("windscreen", B, "glass", "glazing", 0.75, "glazing", [((11.99, 1.2, 0.625), 0.595, 0.012, 8), ((11.8, 2.68, 0.625), 0.575, 0.012, 8)], repeat={"count": 2, "translation": [0, 0, -1.25]}))
S.append(box("destination-sign", B, "glass", "glazing", 0.8, "glazing", (0.03, 0.14, 1.9), at=(11.75, 2.79, 0), corner=0.01))
S.append(box("bumper-front", B, "paint", "detail", 0.5, "body-panels", (0.1, 0.3, 2.45), at=(11.95, 0.45, 0), corner=0.04))
S.append(box("bumper-rear", B, "paint", "detail", 0.5, "body-panels", (0.1, 0.3, 2.45), at=(0.05, 0.45, 0), corner=0.04))
for s_, n in ((1, "l"), (-1, "r")):
    S.append(box("headlamp-" + n, B, "glass", "glazing", 0.9, None, (0.06, 0.22, 0.35), at=(11.97, 0.73, s_ * 0.975), corner=0.02))
    S.append(box("tail-lamp-" + n, B, "glass", "detail", 0.7, None, (0.06, 0.5, 0.35), at=(0.03, 0.87, s_ * 0.975), corner=0.02))
    S.append(box("mirror-" + n, B, "paint", "detail", 0.3, None, (0.12, 0.37, 0.075), at=(11.85, 2.3, s_ * 1.2375), corner=0.02))
    S.append(capsule("mirror-arm-" + n, B, "paint", "detail", 0.3, None, (11.8, 2.45, s_ * 1.2), (11.85, 2.45, s_ * 1.24), 0.015, 0.015, maxd=100))

S.append(box("door-opening-m", B, "interior-trim", "detail", 0.06, None, (1.1, 2.12, 0.004), at=(6.9, 1.4, -1.2765)))
S.append(box("door-opening-f", B, "interior-trim", "detail", 0.06, None, (1.1, 2.12, 0.004), at=(10.9, 1.4, -1.2765)))
for bone, nm in (("door-m-aft", "m-aft"), ("door-m-fwd", "m-fwd"), ("door-f-aft", "f-aft"), ("door-f-fwd", "f-fwd")):
    S.append(box("leaf-" + nm, bone, "door-leaf", "body", 0.5, "doors", (0.55, 2.12, 0.04), at=(0.275, 1.06, -1.265), corner=0.01))
    S.append(box("leaf-" + nm + "-glass", bone, "glass", "glazing", 0.8, "glazing", (0.45, 1.5, 0.008), at=(0.275, 1.25, -1.2885)))
    S.append(capsule("leaf-handrail-" + nm, bone, "interior-trim", "detail", 0.3, None, (0.04, 0.34, -1.3), (0.51, 0.34, -1.3), 0.015, 0.015, maxd=50))

for ax, nm, bone in ((AX_R, "r", "axle-r"), (AX_F, "f", "axle-f")):
    S.append(ellipsoid("e-axle-casting" if nm == "r" else "front-axle-casting", bone, "e-axle" if nm == "r" else "cast-iron", "detail", 0.3, "e-axle" if nm == "r" else "brakes", (0.4, 0.15, 0.7), at=(0, 0.0, 0)))
    S.append(capsule(f"axle-beam-{nm}", bone, "e-axle" if nm == "r" else "cast-iron", "detail", 0.3, "e-axle" if nm == "r" else "brakes", (0, 0, -1.05), (0, 0, 1.05), 0.07, 0.07))
    S.append(box(f"air-spring-{nm}", bone, "steel-wheel", "detail", 0.2, None, (0.28, 0.3, 0.18), at=(0, 0.6, 0.8), repeat={"count": 2, "translation": [0, 0, -1.6]}))

for s_, n in ((1, "l"), (-1, "r")):
    for bone, p in (("wheel-f", "f"), ("wheel-r", "r")):
        wc = s_ * 1.05
        o = [0, 0, wc]
        S.append(revolve(f"{p}{n}-tyre", bone, "rubber", "body", 0.1, "tyres", tyre_ring(0.48, 0.15, shoulder=0.14), axis=(0, 0, 1), origin=o))
        S.append(revolve(f"{p}{n}-rim", bone, "steel-wheel", "detail", 0.7, None, [(-0.14, 0.286), (0.14, 0.286)], axis=(0, 0, 1), origin=o))
        S.append(revolve(f"{p}{n}-hub", bone, "steel-wheel", "detail", 0.55, None, [(0.0, 0.11), (0.17, 0.11)] if s_ > 0 else [(-0.17, 0.11), (0.0, 0.11)], axis=(0, 0, 1), origin=o))
        S.append(capsule(f"{p}{n}-stud", bone, "steel-wheel", "detail", 0.2, None, (0.075, 0, wc + s_ * 0.142), (0.075, 0, wc + s_ * 0.16), 0.013, 0.013, repeat={"count": 8, "rotationAxis": [0, 0, 1], "rotationRad": r(2 * math.pi / 8)}, maxd=50))
        S.append(revolve(f"{p}{n}-drum", bone, "cast-iron", "detail", 0.4, "brakes", [(-0.14, 0.235), (-0.1, 0.235)] if s_ > 0 else [(0.1, 0.235), (0.14, 0.235)], axis=(0, 0, 1), origin=o, maxd=100))

D = [
    dec("The shell is one loft along the bus axis (12 m) with vertical slices every 0.1 to 0.3 m near the wheel arches; the stations come from the elevation outline (circular arches of 0.64 m about both axles, floor 0.3 m, rounded roof ends, raked front face down to 0.95 m), the width is the plan's 2.55 m and an exponent of 12 gives the front view's small roof radii; the arches are open across the whole width as drawn.",
        "Die Karosserie ist ein Loft entlang der Busachse (12 m) mit senkrechten Schnitten alle 0,1 bis 0,3 m nahe der Radkästen; die Stationen stammen aus dem Umriss der Seitenansicht (kreisrunde Radkästen von 0,64 m um beide Achsen, Boden 0,3 m, gerundete Dachenden, geneigte Front bis 0,95 m), die Breite ist die 2,55 m des Grundrisses und ein Exponent von 12 ergibt die kleinen Dachradien der Vorderansicht; die Radkästen sind wie gezeichnet über die ganze Breite offen."),
    dec("Glazing is a set of thin plates flush with the sides (windows on both sides as one solid with repeat, the two door-side gaps only on the left), two raked windscreen panes (one solid with repeat) and the destination sign; the four door leaves are boxes with a glass inset on their own prismatic bones, riding over dark door-opening plates on the right side.",
        "Die Verglasung besteht aus dünnen, bündigen Platten (Fenster beider Seiten als ein Körper mit repeat, die zwei Türfelder nur links), zwei geneigten Frontscheiben (ein Körper mit repeat) und dem Zielschild; die vier Türflügel sind Quader mit Glaseinsatz auf eigenen Schiebeknochen und laufen auf der rechten Seite über dunklen Türöffnungsplatten."),
    dec("Wheels are revolves about the axle at a 2.1 m track (rounded tyre of 0.3 m width, steel rim disc, hub, eight wheel studs with repeat, brake drum); axle bones carry the axle beam, the casting (e-axle at the rear) and the two air springs, so the suspension travel and the kneeling pose move them together with the wheels.",
        "Die Räder sind Rotationskörper um die Achse bei 2,1 m Spurweite (Reifen mit gerundeten Schultern von 0,3 m Breite, Stahlfelgenscheibe, Nabe, acht Radbolzen mit repeat, Bremstrommel); die Achsknochen tragen den Achskörper, das Gehäuse (hinten E-Achse) und die zwei Luftfedern, sodass Federweg und Kneeling-Pose sie gemeinsam mit den Rädern bewegen."),
    dec("The battery pack and the HVAC unit on the roof are thin lofts with tapered ends on their own bones (2.1 m and 2.0 m wide as in the front view), the HVAC louvres are nine ribs with repeat shown at 1:50 only.",
        "Das Batteriepaket und die Klimaanlage auf dem Dach sind flache Lofts mit verjüngten Enden auf eigenen Knochen (2,1 m und 2,0 m breit wie in der Vorderansicht), die Lüftungslamellen sind neun Rippen mit repeat, nur im Maßstab 1:50 sichtbar."),
]
write("city-bus", S, D)
