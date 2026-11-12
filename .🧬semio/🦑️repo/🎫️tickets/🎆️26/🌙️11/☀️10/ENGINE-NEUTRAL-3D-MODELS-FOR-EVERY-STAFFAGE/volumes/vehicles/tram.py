import sys, os, math
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _lib import *

frames("tram")
BE, BM, GW = "body-end", "body-mid", "gangway"
S = []
AR_X = (1.0, 2.8, 6.3, 8.1)
AR = 0.37


def interp(pts, x):
    for (x0, y0), (x1, y1) in zip(pts, pts[1:]):
        if x0 <= x <= x1:
            return y0 + (y1 - y0) * (x - x0) / (x1 - x0) if x1 > x0 else y1
    return pts[-1][1]


def bottom_end(x):
    for ax in AR_X:
        if abs(x - ax) < 0.3695:
            return 0.3 + math.sqrt(AR * AR - (x - ax) ** 2)
    if x > 9.2:
        return interp([(9.2, 0.28), (9.4, 0.4), (9.45, 0.55)], x)
    return 0.28


TOP_E = [(0.45, 3.38), (8.35, 3.38), (8.55, 3.0), (8.9, 2.444), (9.4, 1.65), (9.45, 1.5)]
RX_E = [(0.45, 1.2), (8.9, 1.2), (9.3, 1.1), (9.45, 0.8)]
xs = [0.45, 0.6, 3.5, 4.5, 5.2, 7.0, 7.5, 8.35, 8.55, 8.9, 9.2, 9.3, 9.4, 9.45]
for ax in AR_X:
    xs += [ax + 0.3695 * k for k in (-0.995, -0.97, -0.92, -0.85, -0.75, -0.6, -0.4, -0.2, 0.0, 0.2, 0.4, 0.6, 0.75, 0.85, 0.92, 0.97, 0.995)]
xs = sorted(set(round(x, 4) for x in xs))
secs = []
for x in xs:
    b, t = bottom_end(x), interp(TOP_E, x)
    secs.append(((x, (b + t) / 2, 0), interp(RX_E, x), (t - b) / 2, 8, VERT))
S.append(loft("shell-end", BE, "paint", "body", 0.62, "body-panels", [(loc(BE, s[0]), s[1], s[2], s[3], s[4]) for s in secs]))
S.append(loft("roof-end", BE, "paint", "detail", 0.8, "roof", [(loc(BE, (x, 3.38, 0)), 1.0, 0.02, 8, VERT) for x in (0.6, 8.3, 8.9)][:2]))

MX = [-0.45, -1.5, -3.0, -4.5, -5.5, -5.85, -6.0]
RX_M = [(-6.0, 0.8), (-5.85, 1.1), (-5.5, 1.2), (-0.45, 1.2)]


def rx_m(x):
    return interp(RX_M, x)


S.append(loft("shell-mid", BM, "paint", "body", 0.62, "body-panels", [(loc(BM, (x, 1.83, 0)), rx_m(x), 1.55, 8, VERT) for x in MX]))
S.append(loft("roof-mid", BM, "paint", "detail", 0.8, "roof", [(loc(BM, (x, 3.38, 0)), 1.0, 0.02, 8, VERT) for x in (-0.65, -5.85)]))
S.append(loft("gangway", GW, "rubber", "body", 0.2, "gangway", [(loc(GW, (x, 1.85, 0)), 1.0, 1.4, 10, VERT) for x in (-0.45, -0.2, 0.0, 0.2, 0.45)]))

S.append(box("resistor-box", BE, "resistor-steel", "body", 0.35, "brake-resistor", (2.0, 0.24, 1.75), at=loc(BE, (5.5, 3.52, 0)), corner=0.03))
S.append(box("hvac-box", BM, "paint", "body", 0.7, "hvac-unit", (3.8, 0.26, 1.8), at=loc(BM, (-3.3, 3.53, 0)), corner=0.04))
S.append(box("pan-base", BE, "pan-alloy", "detail", 0.4, "pantograph", (0.6, 0.15, 0.8), at=loc(BE, (2.6, 3.475, 0)), corner=0.02))

GLW = 1.2025
panes_e = [(0.9, 3.3), (5.0, 6.55), (6.7, 8.0)]
for i, (a, b) in enumerate(panes_e):
    S.append(box(f"pane-e{i + 1}", BE, "glass", "glazing", 0.88, "glazing", (b - a, 1.5, 0.012), at=loc(BE, ((a + b) / 2, 2.0, GLW)), repeat={"count": 2, "translation": [0, 0, -2 * GLW]}))
S.append(extrude("pane-cab", BE, "glass", "glazing", 0.88, "glazing", [loc(BE, (x, y, 0))[:2] for x, y in [(8.15, 1.25), (8.4, 1.25), (8.5, 2.75), (8.15, 2.75)]], "xy", 1.196 - 0.0, 1.209, repeat={"count": 2, "translation": [0, 0, -2.405]}))
for i, (a, b) in enumerate([(-5.85, -4.25), (-2.55, -0.85)]):
    S.append(box(f"pane-m{i + 1}", BM, "glass", "glazing", 0.88, "glazing", (b - a, 1.5, 0.012), at=loc(BM, ((a + b) / 2, 2.0, GLW)), repeat={"count": 2, "translation": [0, 0, -2 * GLW]}))
S.append(box("pane-door-gap-a", BE, "glass", "glazing", 0.88, "glazing", (1.3, 1.5, 0.012), at=loc(BE, (4.15, 2.0, GLW))))
S.append(box("pane-door-gap-b", BE, "glass", "glazing", 0.88, "glazing", (1.3, 1.5, 0.012), at=loc(BE, (-3.35, 2.0, GLW))))
S.append(loft("windscreen", BE, "glass", "glazing", 0.88, "glazing", [(loc(BE, (9.452, 1.25, 0)), 1.05, 0.012, 8), (loc(BE, (9.44, 1.6, 0)), 1.03, 0.012, 8), (loc(BE, (8.7, 2.75, 0)), 0.98, 0.012, 8)]))
S.append(box("destination-sign", BE, "glass", "glazing", 0.6, "glazing", (0.04, 0.2, 1.4), at=loc(BE, (8.6, 2.95, 0)), corner=0.01))
for s_, n in ((1, "l"), (-1, "r")):
    S.append(box("headlamp-" + n, BE, "glass", "glazing", 0.9, None, (0.1, 0.25, 0.3), at=loc(BE, (9.4, 0.975, s_ * 0.85)), corner=0.04))

S.append(box("door-opening-a", BE, "interior-trim", "detail", 0.06, None, (1.3, 2.42, 0.004), at=loc(BE, (4.15, 1.49, -1.2005))))
S.append(box("door-opening-b", BE, "interior-trim", "detail", 0.06, None, (1.3, 2.42, 0.004), at=loc(BE, (-3.35, 1.49, -1.2005))))
for bone, nm in (("door-a1", "a1"), ("door-a2", "a2"), ("door-b1", "b1"), ("door-b2", "b2")):
    S.append(box("leaf-" + nm, bone, "paint", "body", 0.6, "doors", (0.65, 2.42, 0.06), at=(0.325, 1.19, -1.2), corner=0.012))
    S.append(box("leaf-glass-" + nm, bone, "glass", "glazing", 0.88, "glazing", (0.41, 1.3, 0.01), at=(0.325, 1.6, -1.2345)))

PL, PU, PH = "pan-lower", "pan-upper", "pan-head"
S.append(capsule("pan-lower", PL, "pan-alloy", "body", 0.3, "pantograph", (0, 0, 0), (1.9, 0, 0), 0.03, 0.03))
S.append(capsule("pan-upper", PU, "pan-alloy", "body", 0.3, "pantograph", (0, 0, 0), (1.9, 0, 0), 0.03, 0.03))
S.append(capsule("pan-head", PH, "carbon", "detail", 0.2, "pantograph", (0, 0, -0.57), (0, 0, 0.57), 0.03, 0.03))
S.append(capsule("pan-head-post", PH, "pan-alloy", "detail", 0.3, "pantograph", (0, 0, 0), (0, -0.03, 0), 0.025, 0.025, maxd=100))
S.append(ellipsoid("pan-knuckle", PL, "pan-alloy", "detail", 0.3, "pantograph", (0.05, 0.05, 0.05), at=(0, 0, 0), maxd=100))

for bone, bx in (("bogie-f", 7.2), ("bogie-r", 1.9)):
    Lb = lambda p, b=bone: loc(b, p)
    for s_, n in ((1, "l"), (-1, "r")):
        S.append(capsule(f"{bone}-frame-{n}", bone, "motor-steel", "detail", 0.25, "traction-motors", Lb((bx - 0.9, 0.45, s_ * 0.6)), Lb((bx + 0.9, 0.45, s_ * 0.6)), 0.06, 0.06))
    S.append(capsule(f"{bone}-cross", bone, "motor-steel", "detail", 0.25, "traction-motors", Lb((bx, 0.45, -0.6)), Lb((bx, 0.45, 0.6)), 0.06, 0.06))
    S.append(box("motor-" + bone[-1], bone, "motor-steel", "detail", 0.25, "traction-motors", (0.9, 0.32, 0.4), at=Lb((bx, 0.44, 0.35)), corner=0.03, repeat={"count": 2, "translation": [0, 0, -0.7]}))

for ws in ("ws-f1", "ws-f2", "ws-r1", "ws-r2"):
    S.append(capsule(ws + "-axle", ws, "steel-wheel", "detail", 0.4, "wheelsets", (0, 0, -0.75), (0, 0, 0.75), 0.06, 0.06))
    S.append(box(ws + "-axlebox", ws, "cast-iron", "detail", 0.4, "wheelsets", (0.22, 0.2, 0.16), at=(0, 0.08, 0.6), corner=0.02, repeat={"count": 2, "translation": [0, 0, -1.2]}))
    for s_, n in ((1, "l"), (-1, "r")):
        o = [0, 0, s_ * 0.715]
        prof = [(-0.065, 0.315), (-0.045, 0.315), (-0.04, 0.3), (0.065, 0.3)]
        prof = prof if s_ > 0 else [(-a, b) for a, b in prof]
        S.append(revolve(f"{ws}-wheel-{n}", ws, "steel-wheel", "body", 0.2, "wheelsets", prof, axis=(0, 0, 1), origin=o))
        S.append(revolve(f"{ws}-disc-{n}", ws, "cast-iron", "detail", 0.55, "brake-discs", [(-0.065 * s_ - 0.02, 0.2), (-0.065 * s_ - 0.005, 0.2)] if s_ > 0 else [(0.005 + 0.065, 0.2), (0.02 + 0.065, 0.2)], axis=(0, 0, 1), origin=o, maxd=100))
        S.append(revolve(f"{ws}-hub-{n}", ws, "steel-wheel", "detail", 0.75, None, [(0.0, 0.07), (0.09, 0.07)] if s_ > 0 else [(-0.09, 0.07), (0.0, 0.07)], axis=(0, 0, 1), origin=o, maxd=100))

D = [
    dec("The two car bodies are lofts with vertical slices along the car axis: the end body (x 0.45 to 9.45 m) takes its stations from the elevation outline (four circular wheel arches of 0.37 m radius about the wheelsets, flat roof at 3.38 m, steep cab front with the screen line from 3.4 m down to 0.55 m) and its plan (2.4 m wide, narrowing to 1.6 m at the nose), the mid body (x -6 to -0.45 m) is a box with rounded plan ends on the mirrored body-mid bone; the rubber gangway is a loft on its own bone.",
        "Die beiden Wagenkästen sind Lofts mit senkrechten Schnitten entlang der Wagenachse: der Endkasten (x 0,45 bis 9,45 m) übernimmt seine Stationen aus dem Seitenriss (vier kreisrunde Radausschnitte von 0,37 m Radius um die Radsätze, flaches Dach bei 3,38 m, steile Kopfpartie mit der Scheibenlinie von 3,4 m bis 0,55 m) und dem Grundriss (2,4 m breit, an der Nase auf 1,6 m verjüngt), der Mittelkasten (x -6 bis -0,45 m) ist ein Kasten mit gerundeten Grundrissenden am gespiegelten Knochen body-mid; der Gummifaltenbalg ist ein Loft auf eigenem Knochen."),
    dec("Each of the four wheelsets is an axle capsule with two steel wheels (revolve with an inboard flange) at the 1.43 m gauge, brake discs and hubs, plus axle boxes; each bogie has two side frames, a cross member and two traction motors (one solid with repeat); the bogie bones carry them, so the wheelsets spin with the clips and the bogies follow the track.",
        "Jeder der vier Radsätze ist eine Achskapsel mit zwei Stahlrädern (Rotationskörper mit innenliegendem Spurkranz) bei 1,43 m Spurweite, Bremsscheiben und Naben sowie Achslagern; jedes Drehgestell hat zwei Längsträger, einen Querträger und zwei Fahrmotoren (ein Körper mit repeat); die Drehgestellknochen tragen sie, sodass sich die Radsätze mit den Clips drehen und die Drehgestelle dem Gleis folgen."),
    dec("The single-arm pantograph is two arm capsules on its own bones (diamond kinematics) and a carbon strip 1.2 m wide at the head; the door leaves sit 0.03 m proud of the body side on their prismatic bones (the 2D plan draws them 0.06 m inside the 1.2 m half width, where a solid body would hide them), over dark door-opening plates on the right side; windows exist on both sides, with two extra windows on the left where the doors are.",
        "Der Einholmstromabnehmer besteht aus zwei Armkapseln auf eigenen Knochen (Scherenkinematik) und einer 1,2 m breiten Kohleschleifleiste am Kopf; die Türflügel stehen 0,03 m vor der Wagenseite auf ihren Schiebeknochen (der 2D-Grundriss zeichnet sie 0,06 m innerhalb der halben Breite von 1,2 m, wo ein voller Kasten sie verdecken würde), über dunklen Türöffnungsplatten an der rechten Seite; Fenster gibt es auf beiden Seiten, links mit zwei zusätzlichen Fenstern an den Türpositionen."),
]
write("tram", S, D)
