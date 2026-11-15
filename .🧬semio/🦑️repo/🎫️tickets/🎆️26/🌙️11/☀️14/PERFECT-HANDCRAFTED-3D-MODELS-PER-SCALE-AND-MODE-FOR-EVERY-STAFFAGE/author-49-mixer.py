import importlib.util, os, math
here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib", os.path.join(here, "author-36-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)
V = lib.Volume("concrete-mixer-truck")
PS = dict(material="paint-steel", role="body")
PSD = dict(material="paint-steel", role="detail")
DP = dict(material="drum-paint", role="body")
DPD = dict(material="drum-paint", role="detail")
GS = dict(material="galvanised-steel", role="body")
GSD = dict(material="galvanised-steel", role="detail")
CI = dict(material="cast-iron", role="detail")
GL = dict(material="glass", role="glazing")
RB = dict(material="rubber", role="body")
EX = dict(material="exhaust-steel", role="body")
rect = lambda a0, a1, b0, b1: [(a0, b0), (a1, b0), (a1, b1), (a0, b1)]
loc = lambda bone, pts: [tuple(V.W(bone, [p[0], p[1], 0])[:2]) for p in pts]
U = list(V.frames["drum"]["u"])
WX = [3.55, 1.95, -1.7, -3.05]
CAB = [(2.4, 0.9), (2.4, 3.14), (2.43, 3.24), (2.5, 3.29), (2.62, 3.3), (4.02, 3.3), (4.14, 3.27), (4.22, 3.2), (4.36, 2.66), (4.46, 2.52), (4.5, 2.36), (4.5, 0.9)]
DPROF = [(0, 0), (0.05, 0.3681), (0.14, 0.5999), (0.28, 0.8119), (0.45, 0.9699), (0.65, 1.0759), (0.9, 1.12)]


def side_outline():
    cx, cy, r = 3.55, 0.52, 0.62
    t0 = math.asin((0.9 - cy) / r)
    arc = sorted([(cx + r * math.cos(math.pi - t0 - i * (math.pi - 2 * t0) / 8), cy + r * math.sin(math.pi - t0 - i * (math.pi - 2 * t0) / 8)) for i in range(9)], key=lambda p: p[0])
    return [(2.4, 0.9), (arc[0][0], 0.9)] + arc[1:-1] + [(arc[-1][0], 0.9), (4.5, 0.9)] + CAB[::-1][0:-1][::-1][0:0] + [(4.5, 2.36), (4.46, 2.52), (4.36, 2.66), (4.22, 3.2), (4.14, 3.27), (4.02, 3.3), (2.62, 3.3), (2.5, 3.29), (2.43, 3.24), (2.4, 3.14)]


def tyre_profile(sg, dish=True, simple=False):
    if simple:
        pts = [(-0.16, 0), (-0.16, 0.49), (-0.13, 0.52), (0.13, 0.52), (0.16, 0.49), (0.16, 0.3), (0.13, 0.3), (0.13, 0)]
    else:
        pts = [(-0.16, 0), (-0.16, 0.5), (-0.15, 0.52), (0.15, 0.52), (0.16, 0.5), (0.16, 0.3), (0.13, 0.3), (0.13, 0)]
    return [[sg * a, r] for a, r in pts]


def radius(a):
    pts = DPROF + [(3.0, 1.12), (5.4, 0.5)]
    for (a0, r0), (a1, r1) in zip(pts, pts[1:]):
        if a0 <= a <= a1:
            return r0 + (r1 - r0) * (a - a0) / (a1 - a0)
    return 0.5



def ext_local(id, bone, plane, pts, lo, hi, **m):
    d = V._meta(id, bone, "extrude", m)
    d["profile"] = [lib.rv(p) for p in pts]
    d["plane"] = plane
    d["from"], d["to"] = lib.r5(lo), lib.r5(hi)
    return V.put(d, {})


OUTER = DPROF + [(1.15, 1.12), (3.0, 1.12), (5.4, 0.5)]
BELLY_END = 8


def inner_of(t):
    out = []
    n = len(OUTER)
    for i, (a, r) in enumerate(OUTER):
        ns = []
        if i > 0:
            da, dr = a - OUTER[i - 1][0], r - OUTER[i - 1][1]
            ln = math.hypot(da, dr)
            ns.append((dr / ln, -da / ln))
        if i < n - 1:
            da, dr = OUTER[i + 1][0] - a, OUTER[i + 1][1] - r
            ln = math.hypot(da, dr)
            ns.append((dr / ln, -da / ln))
        mx, my = sum(x for x, _ in ns), sum(y for _, y in ns)
        k = 1.0 + (ns[0][0] * ns[1][0] + ns[0][1] * ns[1][1] if len(ns) == 2 else 1.0)
        out.append((a + t * mx / k, max(0.0, r + t * my / k)))
    return out


def shell_loop(i0, i1, t):
    inner = inner_of(t)
    return OUTER[i0:i1 + 1] + inner[i0:i1 + 1][::-1] + [OUTER[i0]]


def drum_shells(t, one=False):
    o = V.W("drum", [0, 0, 0])
    if one:
        V.rev("drum-shell", "drum", o, U, shell_loop(0, len(OUTER) - 1, t), tone=0.3, region="drum", **DP)
    else:
        V.rev("drum-body", "drum", o, U, shell_loop(0, BELLY_END, t), tone=0.3, region="drum", **DP)
        V.rev("drum-cone", "drum", o, U, shell_loop(BELLY_END, len(OUTER) - 1, t), tone=0.3, region="drum", **DP)


def seam_ring(t, w, h, texture=True):
    R = 1.12
    prof = [(3.0 - w / 2, R - 0.002), (3.0 + w / 2, R - 0.002), (3.0 + w / 2, R + h), (3.0 - w / 2, R + h), (3.0 - w / 2, R - 0.002)]
    extra = dict(texture=True) if texture else {}
    V.rev("drum-seam", "drum", V.W("drum", [0, 0, 0]), U, [list(p) for p in prof], tone=0.3, region="drum", **extra, **DPD)


def mix_mass(t):
    inner = inner_of(t * 0.6)
    cut = 3.9
    k = max(i for i, p in enumerate(inner) if p[0] <= cut)
    a0, r0 = inner[k]
    a1, r1 = inner[k + 1]
    rc = r0 + (r1 - r0) * (cut - a0) / (a1 - a0)
    pts = [list(p) for p in inner[:k + 1]] + [[cut, rc], [cut, 0.0]]
    V.rev("mass-mix", "drum", V.W("drum", [0, 0, 0]), U, pts, tone=0.2, member="drum", part="mass", **DP)


def chute_u(t):
    ext_local("chute-trough", "chute", "yz", [(-0.14, -0.275), (-0.14, 0.275), (0.14, 0.275), (0.14, 0.275 - t), (-0.14 + t, 0.275 - t), (-0.14 + t, -0.275 + t), (0.14, -0.275 + t), (0.14, -0.275)], 0.0, 2.4, tone=0.5, region="chute", **GS)
    ext_local("chute-ext-trough", "chute-ext", "yz", [(0.0, -0.3), (0.0, 0.3), (0.22, 0.3), (0.22, 0.3 - t), (t, 0.3 - t), (t, -0.3 + t), (0.22, -0.3 + t), (0.22, -0.3)], 0.0, 1.3, tone=0.6, region="chute", **GS)


def helix(n_per_turn, rib, wall, tag="drum-flight", **kw):
    a0, a1, pitch = 0.4, 4.55, 0.85
    n = int((a1 - a0) / pitch * n_per_turn)
    pts = []
    for i in range(n + 1):
        a = a0 + (a1 - a0) * i / n
        th = 2 * math.pi * (a - a0) / pitch
        r = radius(a) - wall - rib + 0.002
        pts.append(V.W("drum-spiral", [a, r * math.cos(th), r * math.sin(th)]))
    V.loft(tag, "drum-spiral", [(p, rib, rib) for p in pts], tone=0.75, caps=(True, True), **kw)

def hopper_pts():
    return loc("drum", [(5.1936, 0.4575), (5.4677, 0.598), (6.0058, 0.0932), (6.1026, -0.2949), (5.4429, -0.3357)])


def _interp(poly, a):
    for (a0, v0), (a1, v1) in zip(poly, poly[1:]):
        if min(a0, a1) <= a <= max(a0, a1) and a0 != a1:
            return v0 + (v1 - v0) * (a - a0) / (a1 - a0)
    return poly[-1][1]


def hopper(n):
    up = [(5.1936, 0.4575), (5.4677, 0.598), (6.0058, 0.0932), (6.1026, -0.2949)]
    lo = [(5.1936, 0.4575), (5.4429, -0.3357), (6.1026, -0.2949)]
    width = [(5.1936, 0.55), (5.2, 0.55), (5.98, 0.35), (6.1026, 0.02)]
    secs = []
    for i in range(n + 1):
        a = 5.21 + (6.09 - 5.21) * i / n
        u, l = _interp(up, a), _interp(lo, a)
        secs.append((V.W("drum", [a, (u + l) / 2, 0]), max(0.04, _interp(width, a)), max(0.03, (u - l) / 2), 6.0))
    V.loft("hopper-funnel", "drum", secs, hint=(0, 0, 1), axis=U, tone=0.55, region="hopper", **GS)


def chute(t_wall, t_floor, pin_r, ext_pin_r, pins=True):
    cm = [(0, -0.14), (2.4, -0.1), (2.4, 0.14), (0, 0.14)]
    V.ext("chute-wall", "chute", "side", loc("chute", cm), 0.275 - t_wall, 0.275, tone=0.5, region="chute", rep=dict(count=2, step=[0, 0, -(0.55 - t_wall)]), **GS)
    V.box("chute-floor", "chute", V.W("chute", [1.2, -0.125, 0]), [2.4, t_floor, 0.52], rz=V.angle("chute"), tone=0.5, region="chute", **GS)
    V.ext("chute-ext-wall", "chute-ext", "side", loc("chute-ext", rect(0, 1.3, 0, 0.22)), 0.3 - t_wall, 0.3, tone=0.6, region="chute", rep=dict(count=2, step=[0, 0, -(0.6 - t_wall)]), **GS)
    V.box("chute-ext-floor", "chute-ext", V.W("chute-ext", [0.65, 0.015, 0]), [1.3, t_floor, 0.54], rz=V.angle("chute-ext"), tone=0.6, region="chute", **GS)
    if pins:
        V.cyl("chute-pin", "chute", V.W("chute", [0, 0, -0.35]), V.W("chute", [0, 0, 0.35]), pin_r, tone=0.2, region="chute", **GSD)
        V.cyl("chute-ext-pin", "chute-ext", V.W("chute-ext", [0, 0, -0.32]), V.W("chute-ext", [0, 0, 0.32]), ext_pin_r, tone=0.2, region="chute", **GSD)


def pedestal(roller):
    V.box("pedestal", "chassis", [-2.325, 1.825, 0.55], [0.55, 1.85, 0.2], tone=0.5, region="chassis", rep=dict(count=2, step=[0, 0, -1.1]), **PS)
    if roller:
        V.cyl("pedestal-roller", "chassis", [-2.325, 2.7, -0.55], [-2.325, 2.7, 0.55], 0.1, tone=0.3, region="chassis", **GSD)
    V.box("hinge-post", "chassis", [-3.925, 1.625, 0], [0.25, 1.45, 0.3], tone=0.5, region="chassis", **PS)


def frame(rail_t, cross):
    V.ext("frame-rail", "chassis", "side", [(-4.5, 0.5), (-4.5, 0.9), (4.3, 0.9), (4.3, 0.42), (4.2, 0.42), (4.2, 0.62), (-4.2, 0.62), (-4.2, 0.5)], 0.45 - rail_t, 0.45, tone=0.4, region="chassis", rep=dict(count=2, step=[0, 0, -(0.75 + 0.3 - 0.3 + (0.0 if True else 0))]) if False else dict(count=2, step=[0, 0, -(0.9 - rail_t)]), **PS)
    if cross:
        V.box("frame-cross", "chassis", [-3.5, 0.76, 0], [0.12, 0.28, 0.6], tone=0.4, region="chassis", rep=dict(count=5, step=[1.8, 0, 0]), **PSD)
    V.box("bumper-frame", "chassis", [4.25, 0.66, 0], [0.1, 0.48, 2.3], tone=0.4, region="chassis", **PS)
    V.box("frame-deck", "chassis", [-0.1, 0.64, 0], [8.8, 0.04, 0.62], tone=0.4, region="chassis", **PS)


def s50():
    frame(0.15, True)
    V.cyl("fuel-tank", "chassis", [0, 0.775, 0], [1.2, 0.775, 0], 0.11, tone=0.35, region="chassis", **PSD)
    V.cyl("air-tank", "chassis", [1.5, 0.775, 0], [2.2, 0.775, 0], 0.11, tone=0.35, region="chassis", **PSD)
    V.ext("cab-core", "chassis", "side", CAB, -0.95, 0.95, tone=0.5, region="cab", **PS)
    V.ext("cab-side", "chassis", "side", side_outline(), 0.95, 1.2, tone=0.5, region="cab", rep=dict(count=2, step=[0, 0, -2.15]), **PS)
    V.box("cab-windscreen", "chassis", [4.31, 2.85, 0], [0.02, 0.62, 2.1], rz=14.5, tone=0.85, region="cab-glazing", **GL)
    V.ext("cab-door-glass", "chassis", "side", rect(3.0, 3.76, 2.52, 3.14), 1.2, 1.21, tone=0.85, region="cab-glazing", rep=dict(count=2, step=[0, 0, -2.41]), **GL)
    V.ext("cab-quarter-glass", "chassis", "side", [(4.3, 2.52), (4.14, 3.12), (3.92, 3.12), (3.92, 2.52)], 1.2, 1.21, tone=0.85, region="cab-glazing", rep=dict(count=2, step=[0, 0, -2.41]), **GL)
    V.ext("door-line", "chassis", "side", [(2.7, 1.12), (3.9, 1.12), (3.9, 3.0), (2.7, 3.0)], 1.2, 1.21, holes=[[(2.73, 1.15), (2.73, 2.97), (3.87, 2.97), (3.87, 1.15)]], tone=0.3, region="cab", rep=dict(count=2, step=[0, 0, -2.41]), **PSD)
    V.box("door-handle", "chassis", [3.77, 2.03, 1.2], [0.3, 0.06, 0.03], tone=0.2, region="cab", rep=dict(count=2, step=[0, 0, -2.4]), **PSD)
    V.box("front-grille", "chassis", [4.495, 1.625, 0], [0.01, 0.65, 1.4], tone=0.2, region="engine-bay", **PSD)
    V.box("head-lamp", "chassis", [4.465, 1.35, 0.9], [0.07, 0.3, 0.4], tone=0.9, region="cab", rep=dict(count=2, step=[0, 0, -1.8]), material="glass", role="detail")
    V.ext("mirror-arm", "chassis", "side", [(4.3, 2.7), (4.5, 2.7), (4.5, 2.95), (4.4, 2.95)], 1.2, 1.275, tone=0.2, region="cab", rep=dict(count=2, step=[0, 0, -2.475]), **PSD)
    V.box("cab-step", "chassis", [4.1, 0.87, 1.05], [0.4, 0.26, 0.3], tone=0.3, region="chassis", rep=dict(count=2, step=[0, 0, -2.1]), **PSD)
    V.cyl("exhaust-stack", "chassis", [2.32, 1.3, -1.1], [2.32, 3.35, -1.1], 0.1, tone=0.2, region="exhaust-stack", **EX)
    V.cyl("stack-cap", "chassis", [2.32, 3.35, -1.1], [2.32, 3.45, -1.1], 0.12, tone=0.25, region="exhaust-stack", material="exhaust-steel", role="detail")
    pedestal(True)
    V.box("rear-ladder-rail", "chassis", [-4.2, 1.81, 0.3], [0.2, 1.38, 0.04], tone=0.4, region="chassis", rep=dict(count=2, step=[0, 0, -0.6]), **PSD)
    V.cyl("rear-ladder-rung", "chassis", [-4.2, 1.3, -0.3], [-4.2, 1.3, 0.3], 0.02, tone=0.4, region="chassis", rep=dict(count=5, step=[0, 0.27, 0]), **PSD)
    drum_shells(0.008)
    helix(12, 0.035, 0.008, region="drum", **DPD)
    seam_ring(0.008, 0.06, 0.012)
    V.ext("drive-housing", "drum", "side", loc("drum", [(0.3798, 1.3701), (-0.3964, 1.1765), (-0.2634, 0.6429), (0.5129, 0.8364)]), -0.4, 0.4, tone=0.35, region="drum-drive", **PS)
    hopper(9)
    V.cyl("chute-swivel-collar", "chute-swivel", [-3.9, 2.1, 0], [-3.9, 2.35, 0], 0.12, tone=0.4, region="chute", **GSD)
    chute(0.03, 0.03, 0.07, 0.06)
    for k, x in enumerate(WX, 1):
        bone = f"wheel-{k}"
        for side, sg in [("l", 1), ("r", -1)]:
            zc = sg * 1.115
            V.rev(f"tyre-{k}-{side}", bone, [x, 0.52, zc], [0, 0, 1], tyre_profile(sg), tone=0.12, region="tyres", **RB)
            V.cyl(f"brake-drum-{k}-{side}", bone, [x, 0.52, zc + sg * 0.13], [x, 0.52, zc + sg * 0.155], 0.3, tone=0.6, region="brakes", **CI)
            V.cyl(f"hub-{k}-{side}", bone, [x, 0.52, zc + sg * 0.155], [x, 0.52, zc + sg * 0.1625], 0.1, tone=0.3, region="brakes", **CI)
            V.cyl(f"handhole-{k}-{side}", bone, [x, 0.72, zc + sg * 0.155], [x, 0.72, zc + sg * 0.1625], 0.05, tone=0.0, region="brakes", rep=dict(count=5, angle=2 * math.pi / 5, axis=[0, 0, 1], origin=[x, 0.52, 0]), **CI)
    V.auto_thermal()
    V.to("thermal")
    V.cyl("th-exhaust-stack-jet", "chassis", [2.32, 3.4, -1.1], [2.32, 3.4078, -1.1], 0.075, tone=0.2, region="exhaust-stack", tags=["exhaust-stack-jet"], **EX)
    V.cyl("th-radiator-air", "chassis", [2.3, 0.75, 0], [2.2922, 0.75, 0], 0.45, tone=0.2, region="engine-bay", tags=["radiator-air"], **PSD)

    structure(50)
    V.decide("1:50 is the detailed model. The drum is now a real revolved shell: the dome and belly are one 8 mm hollow loop revolved about the drum axis, the cone a second 8 mm loop that ends in the open mouth, a 60 mm weld band on the seam, and the helical flight is a 35 mm rib that sits on the inside of the wall (it was a solid cylinder with a hole-less belly before). The concrete charge is a hidden mass solid in the structure list that fills the dome, the belly and the first third of the cone up to 3.9 m along the axis (12.6 m3 as an envelope of the 9 m3 nominal charge; the whole drum holds 16 m3, the lump simplifies the free surface and stops where the rest-pose chute enters the cone) and so closes the inner face of the shell and the flight there against the exposed-area measurement (the open cone beyond the charge stays hollow, and the chute of the rest pose passes through it); the rest of the truck is as before: two frame rails with deck, cross members and bumper, tanks, a cab of core block and two side parts with real wheel arches, sloped windscreen, door and quarter glass, door frame, handles, grille, lamps, mirrors, steps, the hopper funnel, two chute troughs with walls and floor, eight dished tyres with brake drums, hubs and bolts.",
             "1:50 ist das Detailmodell. Die Trommel ist jetzt eine echte Rotationsschale: Kalotte und Bauch sind eine 8 mm dicke hohle Schleife um die Trommelachse, der Konus eine zweite 8-mm-Schleife mit offener Mündung, ein 60 mm breites Schweißband an der Naht, und die Wendel ist eine 35-mm-Rippe auf der Wandinnenseite (vorher war der Bauch ein voller Zylinder ohne Hohlraum). Die Betonladung ist ein verdeckter Massenkörper in der Tragwerksliste, der das ganze Innere bis zur Mündung füllt (er steht für die Hüllform der Ladung: die Trommel fasst geometrisch 16 m3 für die Nennladung von 9 m3, der Klumpen vereinfacht die freie Oberfläche) und damit die Innenfläche der Schale und die Wendel gegenüber der Flächenmessung schließt; der übrige Lkw wie zuvor: zwei Rahmenträger mit Boden, Querträgern und Stoßfänger, Tanks, ein Fahrerhaus aus Kernblock und zwei Seitenteilen mit echten Radausschnitten, geneigte Frontscheibe, Tür- und Seitenscheibe, Türrahmen, Griffe, Grill, Lampen, Spiegel, Tritte, der Fülltrichter, zwei Rinnentröge mit Wänden und Boden, acht gewölbte Reifen mit Bremstrommeln, Naben und Schrauben.")
    V.decide("Thermal: every region is a single layer, so each thermal solid references its body solid; the drum region references the two shell loops, the weld band and the flight; the exhaust jet is a 150 mm nozzle at the stack mouth, the radiator air a 900 mm vent disc behind the grille.",
             "Thermik: Jede Region ist eine einzelne Schicht, daher verweist jeder Thermikkörper auf seinen Körper; die Trommelregion verweist auf die beiden Schalenschleifen, das Schweißband und die Wendel; der Abgasstrahl ist eine 150-mm-Düse an der Auspuffmündung, die Kühlerabluft eine 900-mm-Austrittsscheibe hinter dem Grill.")
    V.decide("Structure: the frame rails are real 130 mm beams (cores inside the rail walls), the cross members, bumper, pedestal, hinge post, drive housing, drum shells, flight rib, swivel collar and both chute troughs (walls and floor form a channel beam) carry the load; joints are the drum bearing, swivel, chute pins and eight hub stubs; eight 160 mm by 300 mm contact pads sit on the ground at the declared anchors; the concrete charge and the chassis are the two masses.",
             "Tragwerk: Die Rahmenträger sind echte 130-mm-Balken (Kerne in den Trägerwänden), Querträger, Stoßfänger, Lagerbock, Gelenkpfosten, Antriebsgehäuse, Trommelschalen, Wendelrippe, Drehkranzkragen und beide Rinnentröge (Wände und Boden bilden einen Kastenträger) tragen die Last; Gelenke sind Trommellager, Drehkranz, Rinnenbolzen und acht Nabenstummel; acht Auflager von 160 mm mal 300 mm liegen am Boden an den angegebenen Ankern; die Betonladung und das Fahrgestell sind die zwei Massen.")
    V.finish(50)


def structure(scale):
    f = {50: 0.0077, 100: 0.0155, 200: 0.031, 500: 0.0755}[scale]
    V.to("structure")
    m = V.member
    bone_of = {s["id"]: s["bone"] for s in V.lists["body"]}
    members = {
        50: ["frame-cross", "bumper-frame", "pedestal", "pedestal-roller", "hinge-post", "drive-housing", "drum-body", "drum-cone", "drum-flight", "chute-swivel-collar", "chute-wall", "chute-floor", "chute-ext-wall", "chute-ext-floor"],
        100: ["frame-cross", "bumper-frame", "pedestal", "pedestal-roller", "hinge-post", "drive-housing", "drum-body", "drum-cone", "drum-flight", "chute-swivel-collar", "chute-trough", "chute-ext-trough"],
        200: ["bumper-frame", "pedestal", "hinge-post", "drive-housing", "drum-shell", "drum-flight", "chute-swivel-collar", "chute-trough", "chute-ext-trough"],
        500: ["truck-frame", "pedestal", "hinge-post", "drum-shell", "chute-trough", "chute-ext-trough"],
    }[scale]
    for sid in members:
        assert sid in bone_of, sid
        m(sid, bone_of[sid])
    if scale < 500:
        V.box("member-frame-rail", "chassis", [0, 0.765, 0.375], [8.4, 0.23, 0.13 if scale < 200 else 0.15], tone=0.4, member="chassis", part="member", rep=dict(count=2, step=[0, 0, -0.75]), **PSD)
    for k in range(1, 5):
        for side in ("l", "r"):
            if f"tyre-{k}-{side}" in bone_of:
                m(f"tyre-{k}-{side}", f"wheel-{k}")
    if scale == 500:
        for k in range(1, 5):
            m(f"wheel-{k}-disc", f"wheel-{k}")
        V.cyl("member-spiral", "drum-spiral", V.W("drum-spiral", [0.4, 0, 0]), V.W("drum-spiral", [4.55, 0, 0]), 0.3, tone=0.75, member="drum-spiral", part="member", **DPD)
        V.cyl("member-swivel", "chute-swivel", [-3.9, 2.1, 0], [-3.9, 2.35, 0], 0.12, tone=0.4, member="chute-swivel", part="member", **GS)
    for k, x in enumerate(WX, 1):
        for side, z in (("l", 1.115), ("r", -1.115)):
            V.box(f"support-contact-{k}-{side}", f"wheel-{k}", [x, f / 2, z], [0.16, max(f, 0.0), 0.3], tone=0.12, member=f"wheel-{k}", part="support", anchor=f"contact-{k}-{side}", **RB)
    V.cyl("joint-drum-bearing", "drum", V.W("drum", [0.2, 0, 0]), V.W("drum", [0.2 + max(0.3, 2 * f), 0, 0]), 0.3, tone=0.4, member="drum", part="joint", **PS)
    V.cyl("joint-chute-swivel", "chute-swivel", [-3.9, 2.1, 0], [-3.9, 2.1 + max(f, 0.05), 0], 0.1 if scale < 500 else 0.12, tone=0.4, member="chute-swivel", part="joint", **GS)
    V.cyl("joint-chute", "chute", V.W("chute", [0, 0, -0.27]), V.W("chute", [0, 0, 0.27]), 0.07 if scale < 200 else 0.1, tone=0.2, member="chute", part="joint", **GS)
    V.cyl("joint-chute-ext", "chute-ext", V.W("chute-ext", [0, 0, -0.3]), V.W("chute-ext", [0, 0, 0.3]), 0.06 if scale < 200 else 0.1, tone=0.2, member="chute-ext", part="joint", **GS)
    for k, x in enumerate(WX, 1):
        V.cyl(f"joint-wheel-{k}", f"wheel-{k}", [x, 0.52, 0.96], [x, 0.52, 1.27], 0.1 if scale < 500 else 0.08, tone=0.3, member=f"wheel-{k}", part="joint", material="cast-iron", role="detail")
        V.cyl(f"joint-wheel-{k}-r", f"wheel-{k}", [x, 0.52, -0.96], [x, 0.52, -1.27], 0.1 if scale < 500 else 0.08, tone=0.3, member=f"wheel-{k}", part="joint", material="cast-iron", role="detail")
    for k, x in enumerate(WX, 1):
        V.cyl(f"axle-{k}", "chassis", [x, 0.52, -0.45], [x, 0.52, 0.45], 0.08 if scale < 200 else 0.1, tone=0.3, member="chassis", part="member", material="cast-iron", role="detail")
    V.ell("mass-chassis", "chassis", [0, 0.77, 0], [0.9, 0.11, 0.3], tone=0.2, member="chassis", part="mass", **PS)
    mix_mass({50: 0.008, 100: 0.0155, 200: 0.031, 500: 0.0755}[scale])


def s100():
    frame(0.15, True)
    V.cyl("fuel-tank", "chassis", [0, 0.775, 0], [1.2, 0.775, 0], 0.11, tone=0.35, region="chassis", **PSD)
    V.ext("cab-core", "chassis", "side", CAB, -0.95, 0.95, tone=0.5, region="cab", **PS)
    V.ext("cab-side", "chassis", "side", side_outline(), 0.95, 1.2, tone=0.5, region="cab", rep=dict(count=2, step=[0, 0, -2.15]), **PS)
    V.box("cab-windscreen", "chassis", [4.31, 2.85, 0], [0.03, 0.62, 2.1], rz=14.5, tone=0.85, region="cab-glazing", **GL)
    V.ext("cab-door-glass", "chassis", "side", rect(3.0, 3.76, 2.52, 3.14), 1.2, 1.215, tone=0.85, region="cab-glazing", rep=dict(count=2, step=[0, 0, -2.415]), **GL)
    V.box("front-grille", "chassis", [4.505, 1.625, 0], [0.016, 0.65, 1.4], tone=0.2, region="engine-bay", **PSD)
    V.ext("mirror-arm", "chassis", "side", [(4.3, 2.7), (4.5, 2.7), (4.5, 2.95), (4.4, 2.95)], 1.2, 1.275, tone=0.2, region="cab", rep=dict(count=2, step=[0, 0, -2.475]), **PSD)
    V.box("cab-step", "chassis", [4.1, 0.87, 1.05], [0.4, 0.26, 0.3], tone=0.3, region="chassis", rep=dict(count=2, step=[0, 0, -2.1]), **PSD)
    V.cyl("exhaust-stack", "chassis", [2.32, 1.3, -1.1], [2.32, 3.35, -1.1], 0.1, tone=0.2, region="exhaust-stack", **EX)
    V.cyl("stack-cap", "chassis", [2.32, 3.35, -1.1], [2.32, 3.45, -1.1], 0.12, tone=0.25, region="exhaust-stack", material="exhaust-steel", role="detail")
    pedestal(True)
    drum_shells(0.0155)
    helix(8, 0.05, 0.0155, region="drum", **DPD)
    seam_ring(0.0155, 0.1, 0.02)
    V.ext("drive-housing", "drum", "side", loc("drum", [(0.3798, 1.3701), (-0.3964, 1.1765), (-0.2634, 0.6429), (0.5129, 0.8364)]), -0.4, 0.4, tone=0.35, region="drum-drive", **PS)
    hopper(6)
    V.cyl("chute-swivel-collar", "chute-swivel", [-3.9, 2.1, 0], [-3.9, 2.35, 0], 0.12, tone=0.4, region="chute", **GSD)
    chute_u(0.0155)
    for k, x in enumerate(WX, 1):
        bone = f"wheel-{k}"
        for side, sg in [("l", 1), ("r", -1)]:
            zc = sg * 1.115
            V.rev(f"tyre-{k}-{side}", bone, [x, 0.52, zc], [0, 0, 1], tyre_profile(sg, simple=True), tone=0.12, region="tyres", **RB)
            V.cyl(f"brake-drum-{k}-{side}", bone, [x, 0.52, zc + sg * 0.13], [x, 0.52, zc + sg * 0.165], 0.3, tone=0.6, region="brakes", **CI)
            V.cyl(f"hub-{k}-{side}", bone, [x, 0.52, zc + sg * 0.165], [x, 0.52, zc + sg * 0.1805], 0.1, tone=0.3, region="brakes", **CI)
    V.auto_thermal()
    V.to("thermal")
    V.cyl("th-exhaust-stack-jet", "chassis", [2.32, 3.4, -1.1], [2.32, 3.416, -1.1], 0.075, tone=0.2, region="exhaust-stack", tags=["exhaust-stack-jet"], **EX)
    V.cyl("th-radiator-air", "chassis", [2.3, 0.75, 0], [2.284, 0.75, 0], 0.45, tone=0.2, region="engine-bay", tags=["radiator-air"], **PSD)
    structure(100)
    V.decide("At 1:100 the drum shell is rebuilt with 15.5 mm walls (dome and belly one loop, cone a second loop, 100 mm weld band, eight-section flight rib of 50 mm on the inner wall); each chute is one U-section trough extruded along its bone with 15.5 mm walls (the 1:50 walls and floor merged); the cab keeps core, sides and arches but loses door frame, handles, quarter glass and lamps; the hopper is a six-section funnel; tyres are plain dished revolves with a brake drum and a hub.",
             "Bei 1:100 ist die Trommelschale mit 15,5-mm-Wänden neu aufgebaut (Kalotte und Bauch eine Schleife, der Konus eine zweite, 100 mm breites Schweißband, Wendelrippe mit acht Schnitten und 50 mm auf der Innenwand); jede Rinne ist ein entlang ihres Knochens extrudierter U-Trog mit 15,5-mm-Wänden (die 1:50-Wände und -Böden zusammengefasst); das Fahrerhaus behält Kern, Seiten und Radausschnitte, verliert aber Türrahmen, Griffe, Seitenscheibe und Lampen; der Fülltrichter ist ein Trichter aus sechs Schnitten; die Reifen sind schlichte gewölbte Drehkörper mit Bremstrommel und Nabe.")
    V.decide("Thermal: references as at 1:50 for the remaining parts; the drum region references the two shell loops, the weld band and the flight, the emitters are 15 mm discs.",
             "Thermik: Verweise wie bei 1:50 für die verbleibenden Teile; die Trommelregion verweist auf die zwei Schalenschleifen, das Schweißband und die Wendel, die Emitter sind 15-mm-Scheiben.")
    V.decide("Structure: the same load path as at 1:50 with real 130 mm frame rail cores, 15.5 mm pads and thicker joints, the U troughs as channel members, the ladder and air tank are not members; the concrete charge is the hidden mass inside the 15.5 mm shell.",
             "Tragwerk: derselbe Lastpfad wie bei 1:50 mit echten 130-mm-Trägerkernen, 15,5-mm-Auflagern und kräftigeren Gelenken, die U-Tröge sind Kastenstäbe, Leiter und Drucklufttank sind keine Stäbe; die Betonladung ist die verdeckte Masse in der 15,5-mm-Schale.")
    V.finish(100)


def common_thermal(scale, f):
    V.to("thermal")
    V.cyl("th-exhaust-stack-jet", "chassis", [2.32, 3.4, -1.1], [2.32, 3.4 + max(f, 0.0075) * 1.03, -1.1], 0.075, tone=0.2, region="exhaust-stack", tags=["exhaust-stack-jet"], **EX)
    V.cyl("th-radiator-air", "chassis", [2.3, 0.75, 0], [2.3 - max(f, 0.0075) * 1.03, 0.75, 0], 0.45, tone=0.2, region="engine-bay", tags=["radiator-air"], **PSD)


def s200():
    frame(0.15, False)
    V.ext("cab", "chassis", "side", CAB, -1.2, 1.2, tone=0.5, region="cab", **PS)
    V.box("cab-windscreen", "chassis", [4.31, 2.85, 0], [0.03, 0.62, 2.1], rz=14.5, tone=0.85, region="cab-glazing", **GL)
    V.ext("cab-door-glass", "chassis", "side", rect(3.0, 3.76, 2.5, 3.12), 1.2, 1.23, tone=0.85, region="cab-glazing", rep=dict(count=2, step=[0, 0, -2.43]), **GL)
    V.box("front-grille", "chassis", [4.5, 1.625, 0], [0.031, 0.65, 1.4], tone=0.2, region="engine-bay", **PSD)
    V.cyl("exhaust-stack", "chassis", [2.32, 1.3, -1.1], [2.32, 3.35, -1.1], 0.1, tone=0.2, region="exhaust-stack", **EX)
    pedestal(False)
    drum_shells(0.031, one=True)
    helix(6, 0.06, 0.031, region="drum", **DPD)
    V.ext("drive-housing", "drum", "side", loc("drum", [(0.3798, 1.3701), (-0.3964, 1.1765), (-0.2634, 0.6429), (0.5129, 0.8364)]), -0.4, 0.4, tone=0.35, region="drum-drive", **PS)
    hopper(5)
    V.cyl("chute-swivel-collar", "chute-swivel", [-3.9, 2.1, 0], [-3.9, 2.35, 0], 0.12, tone=0.4, region="chute", **GSD)
    chute_u(0.031)
    for k, x in enumerate(WX, 1):
        bone = f"wheel-{k}"
        for side, sg in [("l", 1), ("r", -1)]:
            zc = sg * 1.115
            V.rev(f"tyre-{k}-{side}", bone, [x, 0.52, zc], [0, 0, 1], [[sg * a, r] for a, r in [(-0.16, 0), (-0.16, 0.49), (-0.13, 0.52), (0.13, 0.52), (0.16, 0.49), (0.16, 0)]], tone=0.12, region="tyres", **RB)
            V.cyl(f"brake-drum-{k}-{side}", bone, [x, 0.52, zc + sg * 0.12], [x, 0.52, zc + sg * 0.16], 0.3, tone=0.6, region="brakes", **CI)
    V.auto_thermal()
    common_thermal(200, 0.03)
    structure(200)
    V.decide("1:200 is modelled from its own drawings: the cab is one 2.4 m block without wheel arches, the frame is two plain 150 mm rails with a deck and the bumper, the drum is one 31 mm revolved shell from the dome to the open mouth (the 1:200 drawing no longer separates dome, belly and cone) with a six-section flight rib on its inner wall, the hopper a five-section funnel, each chute one 31 mm U-section trough, the tyres plain revolves with a 40 mm brake drum on the outer face. Door frame, handles, mirrors, steps, lamps, tanks, ladder and the weld band are gone.",
             "1:200 ist aus den eigenen Zeichnungen modelliert: Das Fahrerhaus ist ein 2,4 m breiter Block ohne Radausschnitte, der Rahmen besteht aus zwei glatten 150-mm-Trägern mit Boden und Stoßfänger, die Trommel ist eine 31 mm dicke Rotationsschale von der Kalotte bis zur offenen Mündung (die 1:200-Zeichnung trennt Kalotte, Bauch und Konus nicht mehr) mit einer Wendelrippe aus sechs Schnitten an der Innenwand, der Trichter ein Trichter aus fünf Schnitten, jede Rinne ein 31-mm-U-Trog, die Reifen schlichte Drehkörper mit einer 40-mm-Bremstrommel an der Außenseite. Türrahmen, Griffe, Spiegel, Tritte, Lampen, Tanks, Leiter und das Schweißband entfallen.")
    V.decide("Thermal: each region references its body solid; the drum region references the one-piece shell and the flight; the two emitters (exhaust jet, radiator air) are 31 mm and 7.5 mm discs on the stack mouth and behind the cab.",
             "Thermik: Jede Region verweist auf ihren Körper; die Trommelregion verweist auf die einteilige Schale und die Wendel; die beiden Emitter (Abgasstrahl, Kühlerabluft) sind 31-mm- und 7,5-mm-Scheiben an der Auspuffmündung und hinter dem Fahrerhaus.")
    V.decide("Structure: 150 mm frame rail cores, bumper, pedestal, hinge post, drum shell, flight rib, collar, both chute troughs and the tyres are members; axles are 0.2 m bars inside the frame width, wheel hubs, drum bearing and chute pins are joints, contacts are 160 mm by 300 mm pads on the ground, the concrete charge and the chassis are the two masses.",
             "Tragwerk: 150-mm-Trägerkerne, Stoßfänger, Lagerbock, Gelenkpfosten, Trommelschale, Wendelrippe, Kragen, beide Rinnentröge und die Reifen sind Stäbe; Achsen sind 0,2 m dicke Stäbe in der Rahmenbreite, Radnaben, Trommellager und Rinnenbolzen sind Gelenke, Auflager sind 160 mm mal 300 mm große Flächen am Boden, die Betonladung und das Fahrgestell sind die zwei Massen.")
    V.finish(200)


def s500():
    V.ext("truck-frame", "chassis", "side", [(-4.5, 0.55), (-4.5, 0.9), (4.5, 0.9), (4.5, 0.42), (4.25, 0.42), (4.25, 0.62), (-4.2, 0.62), (-4.2, 0.55)], -0.45, 0.45, tone=0.45, region="chassis", **PS)
    V.ext("truck-cab", "chassis", "side", [(2.4, 0.42), (2.4, 3.22), (2.5, 3.3), (4.1, 3.3), (4.22, 3.22), (4.46, 2.5), (4.5, 2.4), (4.5, 0.42)], -1.2, 1.2, tone=0.45, region="cab", **PS)
    V.box("cab-glass", "chassis", [4.36, 2.77, 0], [0.04, 0.62, 2.0], rz=18.4, tone=0.85, region="cab-glazing", **GL)
    V.ext("cab-glass-side", "chassis", "side", rect(3.1, 4.1, 2.46, 3.06), 1.2, 1.23, tone=0.85, region="cab-glazing", rep=dict(count=2, step=[0, 0, -2.43]), **GL)
    V.cyl("exhaust-stack", "chassis", [2.32, 1.3, -1.1], [2.32, 3.35, -1.1], 0.1, tone=0.2, region="exhaust-stack", **EX)
    pedestal(False)
    drum_shells(0.0755, one=True)
    hopper(4)
    chute_u(0.0755)
    for k, x in enumerate(WX, 1):
        V.cyl(f"wheel-{k}-disc", f"wheel-{k}", [x, 0.52, 0.955], [x, 0.52, 1.275], 0.52, tone=0.15, region="tyres", rep=dict(count=2, step=[0, 0, -2.23]), **RB)
    V.auto_thermal()
    V.to("thermal")
    V.box("th-front-grille", "chassis", [4.46, 1.625, 0], [0.08, 0.65, 1.4], tone=0.2, region="engine-bay", **PSD)
    V.ext("th-drive-housing", "drum", "side", loc("drum", [(0.3798, 1.3701), (-0.3964, 1.1765), (-0.2634, 0.6429), (0.5129, 0.8364)]), -0.4, 0.4, tone=0.35, region="drum-drive", **PS)
    for k, x in enumerate(WX, 1):
        V.cyl(f"th-brake-{k}", f"wheel-{k}", [x, 0.52, 1.2], [x, 0.52, 1.31], 0.3, tone=0.6, region="brakes", rep=dict(count=2, step=[0, 0, -2.51]), **CI)
    common_thermal(500, 0.075)
    structure(500)
    V.decide("1:500 is modelled from its own drawings: a frame block (450 mm either side of the centre line, 9 m long), a separate 2.4 m wide cab block down to the bumper line (the 1:500 front draws one full-width block), a sloped glass band, the stack, the rear pedestal and hinge post, the drum as ONE 75 mm revolved shell (dome, belly and cone in one loop, open at the mouth, hollow inside), the hopper as a four-section funnel, each chute as one 75 mm U-section trough and the tyres as discs. Engine grille, drive housing and brakes are below the drawing's resolution and exist as thermal solids only.",
             "1:500 ist aus den eigenen Zeichnungen modelliert: ein Rahmenblock (450 mm beiderseits der Mittellinie, 9 m lang), ein eigener 2,4 m breiter Kabinenblock bis zur Stoßfängerlinie (die 1:500-Vorderansicht zeichnet einen vollbreiten Block), ein geneigtes Glasband, das Auspuffrohr, Lagerbock und Gelenkpfosten hinten, die Trommel als EINE 75 mm dicke Rotationsschale (Kalotte, Bauch und Konus in einer Schleife, an der Mündung offen, innen hohl), der Fülltrichter als Trichter aus vier Schnitten, jede Rinne als ein 75-mm-U-Trog und die Reifen als Scheiben. Kühlergrill, Antriebsgehäuse und Bremsen liegen unter der Auflösung der Zeichnung und bestehen nur als Thermikkörper.")
    V.decide("Thermal: regions without a body solid of their own are real solids inside the masses (engine-bay grille box, drive housing, brake discs at the outer faces of the tyre discs); cab and chassis are the two blocks, tyres the discs, the drum the one shell; emitters are discs at the stack mouth and under the cab.",
             "Thermik: Regionen ohne eigenen Körper sind echte Körper in den Massen (Kühlergrillkasten, Antriebsgehäuse, Bremsscheiben an den Außenseiten der Reifenscheiben); Kabine und Fahrgestell sind die beiden Blöcke, die Reifen die Scheiben, die Trommel die eine Schale; Emitter sind Scheiben an der Auspuffmündung und unter dem Fahrerhaus.")
    V.decide("Structure: frame block, pedestal, hinge post, drum shell, chute troughs and tyre discs are members; a 0.6 m bar stands for the flights and a 0.25 m collar for the swivel; contacts are 160 mm pads, joints as at 1:200 sized to 75 mm; the concrete charge fills the shell as the hidden mass and the chassis is the second mass.",
             "Tragwerk: Rahmenblock, Lagerbock, Gelenkpfosten, Trommelschale, Rinnentröge und Reifenscheiben sind Stäbe; ein 0,6-m-Stab steht für die Wendel und ein 0,25 m langer Kragen für den Drehkranz; Auflager sind 160-mm-Flächen, Gelenke wie bei 1:200 auf 75 mm bemessen; die Betonladung füllt die Schale als verdeckte Masse, das Fahrgestell ist die zweite Masse.")
    V.finish(500)


import sys
sel = os.environ.get("SC", "50,100,200,500").split(",")
for k in sel:
    {"50": s50, "100": s100, "200": s200, "500": s500}[k]()
V.write([int(k) for k in sel])
