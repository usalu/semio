import sys, os, math
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _lib import *

frames("city-car")
C = "chassis"
S = []


def body_section(x, bot, top, rx, e=3.4):
    return ((x, (bot + top) / 2, 0), rx, (top - bot) / 2, e)


S.append(loft("body-lower", C, "paint", "body", 0.62, "body-panels", [
    body_section(-1.95, 0.39, 0.76, 0.54),
    body_section(-1.9, 0.31, 0.8, 0.66),
    body_section(-1.5, 0.3, 0.805, 0.76),
    body_section(-1.3, 0.2, 0.97, 0.76),
    body_section(0.9, 0.2, 0.98, 0.76),
    body_section(1.0, 0.2, 0.9, 0.76),
    body_section(1.3, 0.3, 0.865, 0.75),
    body_section(1.7, 0.3, 0.79, 0.7),
    body_section(1.9, 0.31, 0.76, 0.58),
    body_section(1.95, 0.36, 0.7, 0.34),
]))

cab = [(0.9, 0.998, 0.70), (0.6, 1.195, 0.705), (0.3, 1.392, 0.705), (0.1, 1.47, 0.70), (-0.2, 1.494, 0.70), (-0.9, 1.495, 0.70), (-1.17, 1.481, 0.69), (-1.35, 1.444, 0.67), (-1.5, 1.376, 0.64)]
S.append(loft("cabin", C, "paint", "body", 0.58, "body-panels", [((x, (0.8 + t) / 2, 0), rx, (t - 0.8) / 2, 4.6) for x, t, rx in cab]))
S.append(loft("roof-skin", C, "paint", "body", 0.5, "roof", [((0.22, 1.468, 0), 0.5, 0.02, 6), ((-0.2, 1.485, 0), 0.52, 0.015, 6), ((-0.9, 1.486, 0), 0.52, 0.015, 6), ((-1.3, 1.46, 0), 0.5, 0.017, 6), ((-1.46, 1.405, 0), 0.46, 0.017, 6)]))

S.append(loft("windscreen", C, "glass", "glazing", 0.85, "glazing", [((0.9, 1.012, 0), 0.7, 0.012, 8), ((0.26, 1.432, 0), 0.6, 0.012, 8)]))


def side_window(id, stations, bottom, side):
    secs = []
    wb, wt = 0.712, 0.6
    dy = 0.415
    n = math.hypot(dy, wt - wb)
    ax = [0.0, dy / n, side * (wt - wb) / n]
    for x, top in stations:
        h = max(top - bottom, 0.02)
        yc = bottom + h / 2
        wc = side * (wb + (wt - wb) * (yc - 1.04) / 0.415)
        rx = h / 2 * n / dy
        z = [1.0, 0.0, 0.0]
        y = cross3(z, ax)
        secs.append(((x, yc, wc), rx, 0.011, 8, [r(c) for c in qbasis(ax, y, z)]))
    return loft(id, C, "glass", "glazing", 0.85, "glazing", secs)


front_win = [(0.7, 1.05), (0.5, 1.2), (0.3, 1.405), (0.16, 1.448), (0.0, 1.455), (-0.02, 1.455)]
rear_win = [(-0.14, 1.452), (-0.5, 1.456), (-0.9, 1.452), (-1.22, 1.435), (-1.3, 1.325), (-1.34, 1.166), (-1.347, 1.05)]
for s_, n in ((1, "l"), (-1, "r")):
    S.append(side_window("side-glass-front-" + n, front_win, 1.04, s_))
    S.append(side_window("side-glass-rear-" + n, rear_win, 1.04, s_))

for s_, n in ((1, "l"), (-1, "r")):
    S.append(extrude("quarter-" + n, C, "paint", "body", 0.58, "body-panels", [(-1.49, 0.8), (-1.49, 1.43), (-1.62, 1.31), (-1.78, 1.09), (-1.94, 0.84), (-1.94, 0.8)], "xy", 0.62 if s_ > 0 else -0.7, 0.7 if s_ > 0 else -0.62))
S.append(box("boot-floor", C, "interior-trim", "detail", 0.3, "interior", (0.45, 0.04, 1.24), at=(-1.72, 0.82, 0), maxd=50))

ek = [(1.02, 0.88, 0.9), (1.4, 0.80, 0.85), (1.82, 0.75, 0.78)]
S.append(loft("engine-block", C, "engine-metal", "detail", 0.12, "engine-bay", [((x, (b + t) / 2, 0), 0.6, (t - b) / 2, 3) for x, b, t in ek]))
S.append(box("grille", C, "engine-metal", "detail", 0.3, "engine-bay", (0.03, 0.08, 0.8), at=(1.94, 0.57, 0), corner=0.01))
S.append(loft("bumper-front", C, "paint", "detail", 0.45, "body-panels", [((1.56, 0.285, 0), 0.74, 0.085, 3.4), ((1.9, 0.285, 0), 0.6, 0.085, 3.4), ((1.955, 0.285, 0), 0.4, 0.083, 3.4)]))
S.append(loft("bumper-rear", C, "paint", "detail", 0.45, "body-panels", [((-1.58, 0.285, 0), 0.74, 0.085, 3.4), ((-1.9, 0.285, 0), 0.62, 0.085, 3.4), ((-1.955, 0.285, 0), 0.45, 0.083, 3.4)]))
for s_, n in ((1, "l"), (-1, "r")):
    S.append(ellipsoid("headlamp-" + n, C, "glass", "glazing", 0.9, "glazing", (0.12, 0.05, 0.1), at=(1.82, 0.725, s_ * 0.6)))
    S.append(ellipsoid("mirror-" + n, C, "plastic-trim", "detail", 0.35, "body-panels", (0.065, 0.045, 0.05), at=(0.86, 1.07, s_ * 0.8)))
    S.append(capsule("mirror-arm-" + n, C, "plastic-trim", "detail", 0.35, "body-panels", (0.86, 1.04, s_ * 0.7), (0.86, 1.06, s_ * 0.78), 0.012, maxd=50))
    S.append(capsule("door-handle-front-" + n, C, "plastic-trim", "detail", 0.3, "body-panels", (0.268, 0.94, s_ * 0.755), (0.372, 0.94, s_ * 0.755), 0.012, maxd=50))
    S.append(capsule("door-handle-rear-" + n, C, "plastic-trim", "detail", 0.3, "body-panels", (-0.642, 0.94, s_ * 0.755), (-0.538, 0.94, s_ * 0.755), 0.012, maxd=50))
    S.append(box("seat-front-back-" + n, C, "interior-trim", "detail", 0.3, "interior", (0.12, 0.2, 0.26), at=(0.02, 1.08, s_ * 0.29), corner=0.03, maxd=50))
    S.append(box("seat-front-headrest-" + n, C, "interior-trim", "detail", 0.3, "interior", (0.1, 0.16, 0.18), at=(-0.02, 1.26, s_ * 0.29), corner=0.03, maxd=50))
    S.append(box("seat-front-cushion-" + n, C, "interior-trim", "detail", 0.3, "interior", (0.5, 0.1, 0.4), at=(0.28, 0.99, s_ * 0.29), corner=0.03, maxd=50))
S.append(box("seat-rear-back", C, "interior-trim", "detail", 0.3, "interior", (0.14, 0.17, 1.1), at=(-0.6, 1.07, 0), corner=0.03, maxd=50))
S.append(box("seat-rear-headrest", C, "interior-trim", "detail", 0.3, "interior", (0.1, 0.15, 0.2), at=(-0.63, 1.2, 0), corner=0.03, maxd=50))
S.append(revolve("steering-wheel", C, "interior-trim", "detail", 0.2, "interior", [(-0.012, 0.09), (0.012, 0.09)], axis=(0, 0, 1), origin=(0.56, 1.09, 0.29), maxd=50))
S.append(capsule("exhaust-pipe", C, "exhaust-steel", "detail", 0.35, "exhaust-system", (-1.13, 0.12, -0.35), (0.38, 0.12, -0.35), 0.03))
S.append(capsule("silencer", C, "exhaust-steel", "detail", 0.4, "exhaust-system", (-1.775, 0.13, -0.35), (-1.025, 0.13, -0.35), 0.075, lateral=1.33))
S.append(capsule("tail-pipe", C, "exhaust-steel", "detail", 0.45, "exhaust-system", (-1.97, 0.2, -0.3), (-1.67, 0.2, -0.3), 0.04, 0.04))
for ax, nm in (("axle-f", 1.25), ("axle-r", -1.2)):
    S.append(capsule(ax + "-beam", ax, "cast-iron", "detail", 0.35, "brakes", (0, 0, -0.64), (0, 0, 0.64), 0.04, maxd=50))

frames("city-car")
for s_, n in ((1, "l"), (-1, "r")):
    for bone, p in (("wheel-f", "f"), ("wheel-r", "r")):
        S += wheel(f"{p}{n}", bone, "rubber", "tyres", s_ * 0.72, 0.3, 0.0925, 0.18, 0.19, 0.075, spokes=5, spoke_r=0.014, hub_r=0.045, flip=s_, maxd=100)

B = "bonnet"
S.append(loft("bonnet-slab", B, "paint", "body", 0.62, "bonnet", [(loc(B, (0.95, 0.96, 0)), 0.68, 0.03, 8), (loc(B, (1.529, 0.8755, 0)), 0.67, 0.027, 8), (loc(B, (1.88, 0.8, 0)), 0.64, 0.027, 8)]))
H = "hatch"
S.append(loft("hatch-slab", H, "paint", "body", 0.62, "tailgate", [(loc(H, (-1.49, 1.43, 0)), 0.7, 0.03, 8), (loc(H, (-1.915, 0.82, 0)), 0.7, 0.03, 8)]))
S.append(loft("hatch-glass", H, "glass", "glazing", 0.85, "glazing", [(loc(H, (-1.51 - 0.012, 1.41 + 0.01, 0)), 0.48, 0.014, 8), (loc(H, (-1.8, 1.05, 0)), 0.48, 0.014, 8)]))
for s_, n in ((1, "l"), (-1, "r")):
    S.append(box("tail-lamp-" + n, H, "plastic-trim", "detail", 0.7, "tailgate", (0.13, 0.03, 0.22), at=loc(H, (-1.918, 0.93, s_ * 0.61)), maxd=100))

D = [
    dec("The body is a lower loft (sill to beltline, superelliptic sections 1.52 m wide) with a separate cabin loft above it; stations follow the elevation outline, widths come from the plan outline (0.76 m at the beltline) and the front outline (tumblehome from 0.76 m to 0.6 m at the roof).",
        "Die Karosserie besteht aus einem unteren Loft (Schweller bis Gürtellinie, superelliptische Querschnitte 1,52 m breit) und einem eigenen Kabinen-Loft darüber; die Stationen folgen dem Seitenriss, die Breiten dem Grundriss (0,76 m an der Gürtellinie) und der Vorderansicht (Einzug von 0,76 m auf 0,6 m am Dach)."),
    dec("Windscreen, side and tailgate glass are thin lofts laid on the sloped body surface (side glass tilted with the tumblehome), the two quarter panels and the boot floor frame the open tailgate so that the boot reads as a bay in the tailgate-open pose.",
        "Frontscheibe, Seitenscheiben und Heckscheibe sind dünne Lofts auf der geneigten Karosseriefläche (die Seitenscheiben folgen dem Einzug); die zwei Seitenteile und der Kofferraumboden fassen den geöffneten Heckklappenraum ein, sodass der Kofferraum in der Pose mit offener Klappe als Mulde lesbar ist."),
    dec("Wheels are revolves about the axle (w axis) at the real track of 1.44 m: a rounded-shoulder tyre, a rim disc, hub and five spokes (one solid with repeat); the left and right wheel are separate solids on the shared wheel bone so they spin together in the driving clips.",
        "Die Räder sind Rotationskörper um die Achse (w-Achse) bei der echten Spurweite von 1,44 m: ein Reifen mit gerundeten Schultern, eine Felgenscheibe, Nabe und fünf Speichen (ein Körper mit repeat); linkes und rechtes Rad sind eigene Körper auf dem gemeinsamen Radknochen und drehen daher in den Fahrtclips mit."),
    dec("The bonnet slab and the tailgate (slab, glass, lamps) are bound to their hinged bones; the engine block and grille stay on the chassis so that they show when the bonnet is open. Seats, steering wheel and handles are 1:50 detail inside the cabin.",
        "Motorhaubenblech und Heckklappe (Blech, Scheibe, Leuchten) hängen an ihren Scharnierknochen; Motorblock und Kühlergrill bleiben am Chassis und zeigen sich bei offener Haube. Sitze, Lenkrad und Griffe sind 1:50-Detail im Innenraum."),
]
write("city-car", S, D)
