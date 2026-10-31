"""concrete-mixer-truck generator (8x4, 9 m3 drum, facing right, front bumper at x=+4.5, origin mid-length on the ground)."""
import math, sys
from common import *

AX = [3.55, 1.95, -1.7, -3.05]
WR = 0.52
F = (1.9, 2.1375)
TILT = 14.0
U = (-math.cos(math.radians(TILT)), math.sin(math.radians(TILT)))
N = (math.sin(math.radians(TILT)), math.cos(math.radians(TILT)))
RB = 1.12
PITCH = 0.85

rig = Rig("chassis")
rig.add("chassis", None, 0, 0, 0, 9.0, joint_fixed(), "Chassis with cab, frame and pedestals", "Fahrgestell mit Fahrerhaus, Rahmen und Lagerböcken", 7000, (0, 0.9))
rig.add("drum", "chassis", F[0], F[1], 180 - TILT, 5.4, joint_fixed(), "Mixer drum with hopper and hydraulic drive", "Mischtrommel mit Einfülltrichter und Hydraulikantrieb", 3600, (2.6, 0), "uniform")
rig.add("drum-spiral", "drum", 0, 0, 0, 3.0, joint_pris(0, 0, PITCH), "Spiral flight marking (drifts one pitch per drum revolution)", "Wendelmarkierung (wandert eine Steigung je Trommelumdrehung)")
rig.add("chute", "chassis", -3.9, 2.2, 40, 2.4, joint_rev(-20, 170), "Main discharge chute", "Hauptrutsche", 330, (1.2, 0))
rig.add("chute-ext", "chute", 2.4, 0, 180, 1.3, joint_rev(-185, 5), "Chute extension", "Rutschenverlängerung", 170, (0.65, 0))
for i, x in enumerate(AX, 1):
    rig.add(f"wheel-{i}", "chassis", x, WR, 0, WR, joint_rev(), f"Wheel and axle {i}", f"Rad und Achse {i}", 600, (0, 0))
G = Geo(rig)


def D(s, t):
    return (F[0] + s * U[0] + t * N[0], F[1] + s * U[1] + t * N[1])


def rprof(s):
    if s < 0.9:
        return RB * math.sqrt(max(0.0, 1 - ((0.9 - s) / 0.9) ** 2))
    if s <= 3.0:
        return RB
    return RB - (RB - 0.5) * (s - 3.0) / 2.4


SS = [0.0, 0.05, 0.14, 0.28, 0.45, 0.65, 0.9, 3.0, 5.4]


def drum_poly(s0, s1, coarse=False):
    ss = [s0] + [x for x in SS if s0 < x < s1] + [s1]
    if coarse:
        ss = [s for s in ss if s in (s0, s1) or s in (0.14, 0.45, 0.9, 3.0)]
    return [D(s, rprof(s)) for s in ss] + [D(s, -rprof(s)) for s in reversed(ss)]


DRUM_FULL = drum_poly(0.0, 5.4)
DRUM_500 = drum_poly(0.0, 5.4, coarse=True)
HOPPER = [(-3.25, 2.95), (-3.55, 2.88), (-3.95, 3.5), (-3.95, 3.9), (-3.3, 3.78)]


def rib(s0, h, w, n=6):
    left, right = [], []
    for k in range(n + 1):
        u = -1 + 2 * k / n
        sc = s0 + 0.2125 * math.asin(0.66 * u) / math.asin(0.66)
        left.append(D(sc - w / 2, h * u))
        right.append(D(sc + w / 2, h * u))
    return left + list(reversed(right))


def lw(bone, pts):
    return [ap(rig.W[bone], p) for p in pts]


WHEEL = [ap(rig.W[f"wheel-{i}"], (0, 0)) for i in range(1, 5)]


def frame_poly():
    return [(-4.5, 0.5), (-4.5, 0.9), (4.3, 0.9), (4.3, 0.42), (4.2, 0.42), (4.2, 0.62), (-4.2, 0.62), (-4.2, 0.5)]


def cab_poly(den):
    if den == 200:
        return [(2.4, 0.9), (2.4, 3.2), (2.52, 3.3), (4.08, 3.3), (4.2, 3.24), (4.44, 2.52), (4.5, 2.4), (4.5, 0.9)]
    if den == 100:
        return [(2.4, 0.9), (2.4, 3.16), (2.45, 3.26), (2.58, 3.3), (4.05, 3.3), (4.18, 3.25), (4.38, 2.62), (4.46, 2.5), (4.5, 2.34), (4.5, 0.9)]
    return [(2.4, 0.9), (2.4, 3.14), (2.43, 3.24), (2.5, 3.29), (2.62, 3.3), (4.02, 3.3), (4.14, 3.27), (4.22, 3.2), (4.36, 2.66), (4.46, 2.52), (4.5, 2.36), (4.5, 0.9)]


def shapes500():
    s = []
    body = [(-4.5, 0.55), (-4.5, 0.9), (2.4, 0.9), (2.4, 3.22), (2.5, 3.3), (4.1, 3.3), (4.22, 3.22), (4.46, 2.5), (4.5, 2.4), (4.5, 0.42), (4.25, 0.42), (4.25, 0.62), (-4.2, 0.62), (-4.2, 0.55)]
    s.append(G.poly("truck-body", "chassis", "body", 2, body, tone=0.45, regions=["cab", "chassis", "engine-bay"], material="paint-steel"))
    s.append(G.poly("cab-glass", "chassis", "glazing", 2.5, [(4.30, 2.46), (4.10, 3.06), (3.1, 3.06), (3.1, 2.46)], tone=0.85, region="cab-glazing", material="glass"))
    s.append(G.poly("pedestal", "chassis", "body", 1.5, [(-2.6, 0.9), (-2.05, 0.9), (-2.05, 2.75), (-2.6, 2.75)], tone=0.5, region="chassis", material="paint-steel"))
    s.append(G.poly("hinge-post", "chassis", "body", 1.4, [(-4.05, 0.9), (-3.8, 0.9), (-3.8, 2.35), (-4.05, 2.35)], tone=0.5, region="chassis", material="paint-steel"))
    s.append(G.rect("exhaust-stack", "chassis", "body", 2.2, 2.22, 1.3, 0.2, 2.05, tone=0.2, region="exhaust-stack", material="exhaust-steel"))
    s.append(G.poly("drum-body", "drum", "body", 3, DRUM_500, tone=0.3, regions=["drum", "drum-drive"], material="drum-paint"))
    s.append(G.poly("hopper", "drum", "body", 3.2, HOPPER, tone=0.55, region="hopper", material="galvanised-steel"))
    s.append(G.poly("chute-main", "chute", "body", 3.4, lw("chute", [(0, -0.14), (2.4, -0.1), (2.4, 0.14), (0, 0.14)]), tone=0.5, region="chute", material="galvanised-steel"))
    s.append(G.poly("chute-ext-folded", "chute-ext", "body", 3.5, lw("chute-ext", [(0, 0.0), (1.3, 0.0), (1.3, 0.22), (0, 0.22)]), tone=0.6, region="chute", material="galvanised-steel"))
    for i in range(1, 5):
        c = WHEEL[i - 1]
        s.append(G.ell(f"wheel-{i}-disc", f"wheel-{i}", "body", 4, c[0], c[1], WR, tone=0.15, regions=["tyres", "brakes"], material="rubber"))
    return s


def shapes200():
    s = []
    s.append(G.poly("frame", "chassis", "body", 1.8, frame_poly(), tone=0.4, region="chassis", material="paint-steel"))
    s.append(G.poly("cab", "chassis", "body", 2, cab_poly(200), tone=0.5, region="cab", material="paint-steel"))
    s.append(G.poly("front-grille", "chassis", "detail", 2.2, [(4.5, 1.3), (4.5, 1.95), (4.0, 1.95), (4.0, 1.3)], tone=0.2, region="engine-bay", material="paint-steel"))
    s.append(G.poly("windscreen", "chassis", "glazing", 2.4, [(4.30, 2.5), (4.14, 3.1), (3.9, 3.1), (3.9, 2.5)], tone=0.85, region="cab-glazing", material="glass"))
    s.append(G.poly("door-glass", "chassis", "glazing", 2.4, [(3.76, 2.5), (3.76, 3.12), (3.0, 3.12), (3.0, 2.5)], tone=0.85, region="cab-glazing", material="glass"))
    s.append(G.line("door-line", "chassis", "detail", 2.3, [(2.7, 1.12), (2.7, 3.0), (3.9, 3.0)], tone=0.3, region="cab", material="paint-steel"))
    s.append(G.poly("pedestal", "chassis", "body", 1.5, [(-2.6, 0.9), (-2.05, 0.9), (-2.05, 2.75), (-2.6, 2.75)], tone=0.5, region="chassis", material="paint-steel"))
    s.append(G.poly("hinge-post", "chassis", "body", 1.4, [(-4.05, 0.9), (-3.8, 0.9), (-3.8, 2.35), (-4.05, 2.35)], tone=0.5, region="chassis", material="paint-steel"))
    s.append(G.rect("exhaust-stack", "chassis", "body", 2.2, 2.22, 1.3, 0.2, 2.05, tone=0.2, region="exhaust-stack", material="exhaust-steel"))
    s.append(G.poly("drive-housing", "drum", "body", 3.1, [(1.2, 0.9), (2.0, 0.9), (2.0, 1.45), (1.2, 1.45)], tone=0.35, region="drum-drive", material="paint-steel"))
    s += drum_shapes(200)
    s.append(G.poly("chute-main", "chute", "body", 3.9, lw("chute", [(0, -0.14), (2.4, -0.1), (2.4, 0.14), (0, 0.14)]), tone=0.5, region="chute", material="galvanised-steel"))
    s.append(G.poly("chute-ext-folded", "chute-ext", "body", 4.0, lw("chute-ext", [(0, 0.0), (1.3, 0.0), (1.3, 0.22), (0, 0.22)]), tone=0.6, region="chute", material="galvanised-steel"))
    for i in range(1, 5):
        c = WHEEL[i - 1]
        s.append(G.ell(f"tyre-{i}", f"wheel-{i}", "body", 4, c[0], c[1], WR, tone=0.12, region="tyres", material="rubber"))
        s.append(G.ell(f"brake-drum-{i}", f"wheel-{i}", "detail", 4.2, c[0], c[1], 0.3, tone=0.6, region="brakes", material="cast-iron"))
    return s


def drum_shapes(den):
    s = [G.poly("drum-belly", "drum", "body", 3, DRUM_FULL, tone=0.3, region="drum", material="drum-paint")]
    w = 0.2 if den == 200 else 0.16
    for k in range(5):
        s.append(G.poly(f"flight-{k + 1}", "drum-spiral", "detail", 3.4, rib(0.7 + PITCH * k, 0.5, w), tone=0.75, region="drum", material="drum-paint"))
    s.append(G.poly("drum-dome-cap", "drum", "body", 3.6, drum_poly(0.0, 1.15), tone=0.3, region="drum", material="drum-paint"))
    s.append(G.poly("drum-cone-cap", "drum", "body", 3.6, drum_poly(2.85, 5.4), tone=0.3, region="drum", material="drum-paint"))
    s.append(G.poly("hopper", "drum", "body", 3.8, HOPPER, tone=0.55, region="hopper", material="galvanised-steel"))
    return s


def shapes100(fine=False):
    s = []
    den = 50 if fine else 100
    s.append(G.poly("frame", "chassis", "body", 1.8, frame_poly(), tone=0.4, region="chassis", material="paint-steel"))
    s.append(G.poly("cab", "chassis", "body", 2, cab_poly(50 if fine else 100), tone=0.5, region="cab", material="paint-steel"))
    s.append(G.poly("front-grille", "chassis", "detail", 2.2, [(4.5, 1.3), (4.5, 1.95), (4.0, 1.95), (4.0, 1.3)], tone=0.2, region="engine-bay", material="paint-steel"))
    s.append(G.poly("windscreen", "chassis", "glazing", 2.4, [(4.30, 2.52), (4.14, 3.12), (3.92, 3.12), (3.92, 2.52)], tone=0.85, region="cab-glazing", material="glass"))
    s.append(G.poly("door-glass", "chassis", "glazing", 2.4, [(3.76, 2.52), (3.76, 3.14), (3.0, 3.14), (3.0, 2.52)], tone=0.85, region="cab-glazing", material="glass"))
    s.append(G.line("door-line", "chassis", "detail", 2.3, [(2.7, 1.12), (2.7, 3.0), (3.9, 3.0)], tone=0.3, region="cab", material="paint-steel"))
    s.append(G.poly("mirror-arm", "chassis", "detail", 2.6, [(4.3, 2.7), (4.5, 2.7), (4.5, 2.95), (4.4, 2.95)], tone=0.2, region="cab", material="paint-steel"))
    s.append(G.poly("cab-step", "chassis", "detail", 1.9, [(3.9, 0.74), (4.3, 0.74), (4.3, 1.0), (3.9, 1.0)], tone=0.3, region="chassis", material="paint-steel"))
    s.append(G.poly("fuel-tank", "chassis", "detail", 1.9, [(0.0, 0.64), (1.2, 0.64), (1.2, 0.9), (0.0, 0.9)], tone=0.35, region="chassis", material="paint-steel"))
    s.append(G.poly("pedestal", "chassis", "body", 1.5, [(-2.6, 0.9), (-2.05, 0.9), (-2.05, 2.75), (-2.6, 2.75)], tone=0.5, region="chassis", material="paint-steel"))
    s.append(G.poly("hinge-post", "chassis", "body", 1.4, [(-4.05, 0.9), (-3.8, 0.9), (-3.8, 2.35), (-4.05, 2.35)], tone=0.5, region="chassis", material="paint-steel"))
    s.append(G.rect("exhaust-stack", "chassis", "body", 2.2, 2.22, 1.3, 0.2, 2.05, tone=0.2, region="exhaust-stack", material="exhaust-steel"))
    s.append(G.rect("stack-cap", "chassis", "detail", 2.3, 2.2, 3.35, 0.24, 0.1, tone=0.25, region="exhaust-stack", material="exhaust-steel"))
    s.append(G.poly("drive-housing", "drum", "body", 3.1, [(1.2, 0.9), (2.0, 0.9), (2.0, 1.45), (1.2, 1.45)], tone=0.35, region="drum-drive", material="paint-steel"))
    s += drum_shapes(100)
    s.append(G.poly("chute-main", "chute", "body", 3.9, lw("chute", [(0, -0.14), (2.4, -0.1), (2.4, 0.14), (0, 0.14)]), tone=0.5, region="chute", material="galvanised-steel"))
    s.append(G.poly("chute-ext-folded", "chute-ext", "body", 4.0, lw("chute-ext", [(0, 0.0), (1.3, 0.0), (1.3, 0.22), (0, 0.22)]), tone=0.6, region="chute", material="galvanised-steel"))
    for i in range(1, 5):
        c = WHEEL[i - 1]
        b = f"wheel-{i}"
        s.append(G.ell(f"tyre-{i}", b, "body", 4, c[0], c[1], WR, tone=0.12, region="tyres", material="rubber", hatch="tread" if fine else None))
        s.append(G.ell(f"brake-drum-{i}", b, "detail", 4.2, c[0], c[1], 0.3, tone=0.6, region="brakes", material="cast-iron"))
        s.append(G.ell(f"hub-{i}", b, "detail", 4.3, c[0], c[1], 0.1, tone=0.3, region="brakes", material="cast-iron"))
        for k in range(5):
            a = math.radians(90 + 72 * k)
            s.append(G.ell(f"handhole-{i}-{k + 1}", b, "detail", 4.3, c[0] + 0.2 * math.cos(a), c[1] + 0.2 * math.sin(a), 0.05, tone=0.0, filled=False, region="brakes", material="cast-iron"))
    if fine:
        s.append(G.poly("head-lamp", "chassis", "detail", 2.5, [(4.5, 1.2), (4.5, 1.5), (4.26, 1.5), (4.26, 1.2)], tone=0.9, region="cab", material="glass"))
        s.append(G.poly("door-handle", "chassis", "detail", 2.5, [(3.62, 2.0), (3.92, 2.0), (3.92, 2.06), (3.62, 2.06)], tone=0.2, region="cab", material="paint-steel"))
        s.append(G.poly("rear-ladder", "chassis", "detail", 1.6, [(-4.3, 1.12), (-4.1, 1.12), (-4.1, 2.5), (-4.3, 2.5)], tone=0.4, filled=False, region="chassis", material="paint-steel"))
        s.append(G.poly("air-tank", "chassis", "detail", 1.9, [(1.5, 0.66), (2.2, 0.66), (2.2, 0.96), (1.5, 0.96)], tone=0.35, region="chassis", material="paint-steel"))
        s.append(G.line("drum-seam", "drum", "detail", 3.7, [D(3.0, RB), D(3.0, -RB)], tone=0.3, region="drum", material="drum-paint"))
    return s


def hatches50():
    return [hatch("tread", "Tyre tread blocks", "Reifenprofilblöcke", "lines", 45, 0.8, 0.13)]


def build_reps():
    r = {}
    p500 = pens(0.15, None, None, 0.13, 0.25)
    p200 = pens(0.18, 0.13, None, 0.13, 0.25)
    p100 = pens(0.25, 0.18, None, 0.13, 0.35)
    p50 = pens(0.35, 0.25, 0.13, 0.18, 0.5)
    r["500"] = rep(500, 350, 1000, p500, [], shapes500(), [
        tx("One composed truck silhouette (frame, cab and bumper), the drum with its dome, belly and cone as one closed form, a hopper wedge, four wheel discs, a chute bar with folded extension and the exhaust stack.", "Eine zusammengesetzte Lkw-Silhouette (Rahmen, Fahrerhaus, Stoßfänger), die Trommel mit Kuppe, Bauch und Konus als eine geschlossene Form, ein Trichterkeil, vier Radscheiben, eine Rutschenleiste mit eingeklappter Verlängerung und das Abgasrohr."),
        tx("Overall 9.0 x 3.95 m, 14 degree drum axis, cab-glass band, rear pedestal, the stack at 0.2 m (0.4 mm) and both chute parts.", "Gesamtmaß 9,0 x 3,95 m, 14 Grad Trommelachse, Glasband des Fahrerhauses, hinterer Lagerbock, das Abgasrohr mit 0,2 m (0,4 mm) und beide Rutschenteile."),
        tx("Frame, cab, bumper and engine bay form one body; tyres and brake drums are one disc per wheel; the hydraulic drive housing is part of the drum form.", "Rahmen, Fahrerhaus, Stoßfänger und Motorraum bilden einen Körper; Reifen und Bremstrommeln sind je Rad eine Scheibe; das Hydraulikantriebsgehäuse gehört zur Trommelform."),
        tx("Spiral flights, seams, door, mirrors, tanks, rim details and the bumper step are dropped; they are below 0.2 mm or cannot be told apart at 1:500.", "Wendelmarkierung, Nähte, Tür, Spiegel, Tanks, Felgendetails und der Trittstufe am Stoßfänger entfallen; sie liegen unter 0,2 mm oder sind bei 1:500 nicht zu unterscheiden."),
        tx("The stack (0.2 m) and the chute (0.24 to 0.28 m) are drawn at about real width, 0.4 to 0.56 mm; nothing is thickened.", "Das Rohr (0,2 m) und die Rutsche (0,24 bis 0,28 m) sind in etwa Realbreite gezeichnet, 0,4 bis 0,56 mm; nichts wird verdickt."),
        tx("None.", "Keine."),
        tx("Body and cab mid-dark, drum dark, glass lightest, hopper and chute mid-tone, wheels darkest; pens 0.15 mm body, 0.13 mm glass.", "Körper und Fahrerhaus mitteldunkel, Trommel dunkel, Glas am hellsten, Trichter und Rutsche mittlerer Ton, Räder am dunkelsten; Stifte 0,15 mm Körper, 0,13 mm Glas."),
        tx("The truck-body form carries cab, chassis and engine-bay (merged); the drum form carries drum and drive; the wheel disc carries tyres and brakes.", "Die Lkw-Körperform trägt Fahrerhaus, Fahrgestell und Motorraum (zusammengefasst); die Trommelform trägt Trommel und Antrieb; die Radscheibe trägt Reifen und Bremsen."),
    ])
    r["200"] = rep(200, 150, 350, p200, [], shapes200(), [
        tx("Frame band, cab, two glass panes, grille panel, the drum as belly with dome and cone caps and five flight marks, hopper, drive housing, chute with extension, four tyres with brake drums.", "Rahmenband, Fahrerhaus, zwei Glasscheiben, Kühlerfeld, die Trommel als Bauch mit Kuppen- und Konuskappe und fünf Wendelmarken, Trichter, Antriebsgehäuse, Rutsche mit Verlängerung, vier Reifen mit Bremstrommeln."),
        tx("Door line, pillar between windscreen and door glass, grille, drive housing, flight marks that drift with drum rotation, brake drum inside each tyre.", "Türlinie, Pfosten zwischen Frontscheibe und Türglas, Kühlerfeld, Antriebsgehäuse, mit der Trommeldrehung wandernde Wendelmarken, Bremstrommel in jedem Reifen."),
        tx("Windscreen and door glass are separate panes; mirrors, steps, tanks, hub and hand holes merge into the surrounding forms.", "Frontscheibe und Türglas sind getrennte Scheiben; Spiegel, Tritte, Tanks, Nabe und Handlöcher gehen in den umgebenden Formen auf."),
        tx("Mirrors, steps, tanks, hub, bolts, tread, hinges.", "Spiegel, Tritte, Tanks, Nabe, Schrauben, Profil, Scharniere."),
        tx("Flight marks are 0.2 m (1 mm) wide; the stack 0.2 m (1 mm); chute 0.24 m (1.2 mm).", "Wendelmarken sind 0,2 m (1 mm) breit; das Rohr 0,2 m (1 mm); die Rutsche 0,24 m (1,2 mm)."),
        tx("None.", "Keine."),
        tx("Tyres darkest, drum dark with lighter flight marks, cab mid-tone, glass lightest, brake drums mid-tone.", "Reifen am dunkelsten, Trommel dunkel mit helleren Wendelmarken, Fahrerhaus mittlerer Ton, Glas am hellsten, Bremstrommeln mittlerer Ton."),
        tx("Each region is drawn as its own form: cab, chassis frame, engine-bay grille, glazing, drum (belly and caps), drive housing, hopper, chute, exhaust stack, tyres and brake drums.", "Jede Region ist eine eigene Form: Fahrerhaus, Rahmen, Kühlerfeld, Verglasung, Trommel (Bauch und Kappen), Antriebsgehäuse, Trichter, Rutsche, Abgasrohr, Reifen und Bremstrommeln."),
    ])
    r["100"] = rep(100, 75, 150, p100, [], shapes100(False), [
        tx("Frame, cab with chamfered roof and sloped screen, glazing, grille, mirror arm, step, fuel tank, drum with flights, hopper, drive housing, chute pair, wheels with brake drum, hub and five hand holes.", "Rahmen, Fahrerhaus mit abgeschrägtem Dach und geneigter Scheibe, Verglasung, Kühlerfeld, Spiegelarm, Tritt, Kraftstofftank, Trommel mit Wendelmarken, Trichter, Antriebsgehäuse, Rutschenpaar, Räder mit Bremstrommel, Nabe und fünf Handlöchern."),
        tx("Door, mirror arm, step, tank, stack cap, hub and hand holes (drawn unfilled, they rotate with the wheel).", "Tür, Spiegelarm, Tritt, Tank, Rohrkappe, Nabe und Handlöcher (unausgefüllt gezeichnet, sie drehen mit dem Rad)."),
        tx("Wheel studs and the tyre tread merge into the tyre disc.", "Radbolzen und Reifenprofil gehen in der Reifenscheibe auf."),
        tx("Door handle, lamps, tyre tread, hinges, rails.", "Türgriff, Leuchten, Reifenprofil, Scharniere, Geländer."),
        tx("Mirror arm 0.2 m (2 mm), hand holes 0.1 m (1 mm) as outlines.", "Spiegelarm 0,2 m (2 mm), Handlöcher 0,1 m (1 mm) als Umriss."),
        tx("None.", "Keine."),
        tx("As 1:200 with detail lines in the 0.18 mm pen.", "Wie 1:200 mit Detaillinien im 0,18-mm-Stift."),
        tx("As 1:200; brake drum, hub and hand holes all carry the brakes region.", "Wie 1:200; Bremstrommel, Nabe und Handlöcher tragen die Bremsregion."),
    ])
    r["50"] = rep(50, 20, 75, p50, hatches50(), shapes100(True), [
        tx("Cab drawn with rounded roof edges and door details, full chassis equipment, drum with flights, hopper, drive, chute pair, wheels with studs.", "Fahrerhaus mit gerundeten Dachkanten und Türdetails, volle Fahrgestellausrüstung, Trommel mit Wendelmarken, Trichter, Antrieb, Rutschenpaar, Räder mit Bolzen."),
        tx("All 1:100 parts plus tread hatch on the tyre.", "Alle 1:100-Teile plus Profilschraffur am Reifen."),
        tx("Nothing merged.", "Nichts zusammengefasst."),
        tx("Lamp lenses, door handle, rails, hinges.", "Leuchtengläser, Türgriff, Geländer, Scharniere."),
        tx("Hand holes 0.1 m (2 mm) as outlines, stack cap 0.1 m.", "Handlöcher 0,1 m (2 mm) als Umriss, Rohrkappe 0,1 m."),
        tx("Tread hatch at 0.8 mm spacing on tyres only.", "Profilschraffur mit 0,8 mm Abstand nur auf Reifen."),
        tx("Heavier pens 0.35 / 0.25 mm; tone as 1:200.", "Stärkere Stifte 0,35 / 0,25 mm; Ton wie 1:200."),
        tx("As 1:100.", "Wie 1:100."),
    ])
    return r


if __name__ == "__main__":
    for den, fn in [(200, shapes200), (100, lambda: shapes100(False)), (50, lambda: shapes100(True))]:
        print(den, len(fn()))
