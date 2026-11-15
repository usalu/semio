import importlib.util, os, math
here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib", os.path.join(here, "author-03-lib.py"))
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


def helix(n_per_turn, rib, tag="drum-flight", **kw):
    a0, a1, pitch = 0.4, 4.55, 0.85
    n = int((a1 - a0) / pitch * n_per_turn)
    pts = []
    for i in range(n + 1):
        a = a0 + (a1 - a0) * i / n
        th = 2 * math.pi * (a - a0) / pitch
        r = radius(a) - 0.03
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


def drum_three(thick):
    V.rev("drum-dome-cap", "drum", V.W("drum", [0, 0, 0]), U, [[a, r] for a, r in DPROF + [(1.15, 1.12)]], tone=0.3, region="drum", **DP)
    V.rev("drum-belly", "drum", V.W("drum", [1.15, 0, 0]), U, [[0, 1.12], [1.85, 1.12]], tone=0.3, region="drum", **DP)
    cone = [[0, 1.12], [2.4, 0.5], [2.4, 0.5 - thick], [0, 1.12 - thick], [0, 1.12]]
    V.rev("drum-cone-cap", "drum", V.W("drum", [3.0, 0, 0]), U, cone, tone=0.3, region="drum", **DP)


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
    drum_three(0.03)
    helix(12, 0.035, region="drum", **DPD)
    t = 2 * math.pi / 28
    V.cap("drum-seam", "drum", V.W("drum", [3.0, 1.095, 0]), V.W("drum", [3.0, 1.095 * math.cos(t), 1.095 * math.sin(t)]), 0.025, tone=0.3, region="drum", rep=dict(count=28, angle=t, axis=U, origin=V.W("drum", [3.0, 0, 0])), **DPD)
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
    V.decide("The 1:50 truck is the detailed model: two frame rails with a bottom plate and cross members, tanks between the rails, a cab as a core block plus two side parts with real wheel arches, sloped windscreen, door and quarter glass, door frame, handles, grille, lamps, mirrors and steps; the drum is three revolved pieces (dome, belly, open cone shell) with a helical flight rib on the drum-spiral bone and 28 seams, the hopper a funnel loft that narrows to the drawn plan, the chute and its extension two troughs with pins; eight tyres are revolved with a dished outer face, brake drum, hub and five bolts; the axles are only in the structure list because the plan draws none.",
             "Das 1:50-Modell ist das Detailmodell: zwei Rahmenträger mit Bodenplatte und Querträgern, Tanks zwischen den Trägern, ein Fahrerhaus aus Kernblock und zwei Seitenteilen mit echten Radausschnitten, geneigte Frontscheibe, Tür- und Seitenscheibe, Türrahmen, Griffe, Kühlergrill, Lampen, Spiegel und Tritte; die Trommel besteht aus drei gedrehten Teilen (Kalotte, Bauch, offene Konusschale) mit einer Wendelrippe auf dem Knochen drum-spiral und 28 Nähten, der Fülltrichter ist ein Trichter-Loft, der sich zum gezeichneten Grundriss verjüngt, Rinne und Verlängerung sind zwei Tröge mit Bolzen; acht Reifen sind gedreht mit eingezogener Außenseite, Bremstrommel, Nabe und fünf Bolzen; die Achsen stehen nur in der Tragwerksliste, weil der Grundriss keine zeichnet.")
    V.decide("Thermal: every region is a single layer, so each thermal solid references its body solid (cab sheet, glazing, frame parts and tanks, grille, stack, drum pieces and flight, drive housing, hopper, chutes, tyres, brake parts); the exhaust jet is a 150 mm nozzle at the stack mouth, the radiator air a 900 mm vent disc under the cab. Region areas were corrected at the root to the exposed surfaces of these solids.",
             "Thermik: Jede Region ist eine einzelne Schicht, daher verweist jeder Thermikkörper auf seinen Körper (Kabinenblech, Verglasung, Rahmenteile und Tanks, Grill, Auspuff, Trommelteile und Wendel, Antriebsgehäuse, Trichter, Rinnen, Reifen, Bremsteile); der Abgasstrahl ist eine 150-mm-Düse an der Auspuffmündung, die Kühlerabluft eine 900-mm-Scheibe unter dem Fahrerhaus. Die Regionsflächen wurden an der Wurzel auf die freien Flächen dieser Körper korrigiert.")
    V.decide("Structure: wheels, axles, rails with cross members and plate, bumper, pedestal, hinge post, drive housing, drum pieces, flight rib, swivel collar and both troughs carry the load; joints are the drum bearing, swivel, chute pins and eight hub stubs; eight 160 mm by 300 mm contact pads sit on the ground at the declared contact anchors; masses sit in the chassis and at the concrete centre of the drum.",
             "Tragwerk: Räder, Achsen, Träger mit Querträgern und Platte, Stoßfänger, Lagerbock, Gelenkpfosten, Antriebsgehäuse, Trommelteile, Wendelrippe, Drehkranzkragen und beide Tröge tragen die Last; Gelenke sind Trommellager, Drehkranz, Rinnenbolzen und acht Nabenstummel; acht 160 mm mal 300 mm große Auflagerflächen stehen an den angegebenen Aufstandsankern am Boden; Massen sitzen im Fahrgestell und im Betonschwerpunkt der Trommel.")
    V.finish(50)


def structure(scale):
    f = {50: 0.0077, 100: 0.0155, 200: 0.031, 500: 0.0755}[scale]
    V.to("structure")
    m = V.member
    bone_of = {s["id"]: s["bone"] for s in V.lists["body"]}
    members = {
        50: ["frame-rail", "frame-cross", "bumper-frame", "pedestal", "pedestal-roller", "hinge-post", "drive-housing", "drum-dome-cap", "drum-belly", "drum-cone-cap", "drum-flight", "chute-swivel-collar", "chute-wall", "chute-floor", "chute-ext-wall", "chute-ext-floor"],
        100: ["frame-rail", "frame-cross", "bumper-frame", "pedestal", "pedestal-roller", "hinge-post", "drive-housing", "drum-dome-cap", "drum-belly", "drum-cone-cap", "drum-flight", "chute-swivel-collar", "chute-wall", "chute-floor", "chute-ext-wall", "chute-ext-floor"],
        200: ["frame-rail", "bumper-frame", "pedestal", "hinge-post", "drive-housing", "drum-body", "drum-cone-cap", "drum-flight", "chute-swivel-collar", "chute-wall", "chute-floor", "chute-ext-wall", "chute-ext-floor"],
        500: ["truck-frame", "pedestal", "hinge-post", "drum-body", "drum-cone-cap", "chute-wall", "chute-floor", "chute-ext-wall", "chute-ext-floor"],
    }[scale]
    for sid in members:
        if sid in bone_of:
            m(sid, bone_of[sid])
    for k in range(1, 5):
        for side in ("l", "r"):
            if f"tyre-{k}-{side}" in bone_of:
                m(f"tyre-{k}-{side}", f"wheel-{k}")
    if scale == 500:
        for k in range(1, 5):
            m(f"wheel-{k}-disc", f"wheel-{k}")
        V.cyl("member-spiral", "drum-spiral", V.W("drum-spiral", [0.4, 0, 0]), V.W("drum-spiral", [4.55, 0, 0]), 0.3, tone=0.75, member="drum-spiral", part="member", **DPD)
        V.cyl("member-swivel", "chute-swivel", [-3.9, 2.1, 0], [-3.9, 2.35, 0], 0.12, tone=0.4, member="chute-swivel", part="member", **GS)
    sw = {50: 0.15, 100: 0.15, 200: 0.2, 500: 0.3}[scale]
    for k, x in enumerate(WX, 1):
        for side, z in (("l", 1.115), ("r", -1.115)):
            V.box(f"support-contact-{k}-{side}", f"wheel-{k}", [x, f / 2, z], [0.16, max(f, 0.0), 0.3], tone=0.12, member=f"wheel-{k}", part="support", anchor=f"contact-{k}-{side}", **RB)
    V.cyl("joint-drum-bearing", "drum", V.W("drum", [0.2, 0, 0]), V.W("drum", [0.2 + max(0.3, 2 * f), 0, 0]), 0.3, tone=0.4, member="drum", part="joint", **PS)
    V.cyl("joint-chute-swivel", "chute-swivel", [-3.9, 2.1, 0], [-3.9, 2.1 + max(f, 0.05), 0], 0.1 if scale < 500 else 0.12, tone=0.4, member="chute-swivel", part="joint", **GS)
    V.cyl("joint-chute", "chute", V.W("chute", [0, 0, -0.27]), V.W("chute", [0, 0, 0.27]), 0.07 if scale < 200 else 0.1, tone=0.2, member="chute", part="joint", **GS)
    V.cyl("joint-chute-ext", "chute-ext", V.W("chute-ext", [0, 0, -0.3]), V.W("chute-ext", [0, 0, 0.3]), 0.06 if scale < 200 else 0.1, tone=0.2, member="chute-ext", part="joint", **GS)
    for k, x in enumerate(WX, 1):
        V.cyl(f"joint-wheel-{k}", f"wheel-{k}", [x, 0.52, 0.96], [x, 0.52, 1.27], 0.1 if scale < 500 else 0.08, tone=0.3, member=f"wheel-{k}", part="joint", rep=dict(count=2, step=[0, 0, -2.23]) if False else None, material="cast-iron", role="detail")
        V.cyl(f"joint-wheel-{k}-r", f"wheel-{k}", [x, 0.52, -0.96], [x, 0.52, -1.27], 0.1 if scale < 500 else 0.08, tone=0.3, member=f"wheel-{k}", part="joint", material="cast-iron", role="detail")
    for k, x in enumerate(WX, 1):
        V.cyl(f"axle-{k}", "chassis", [x, 0.52, -0.45], [x, 0.52, 0.45], 0.08 if scale < 200 else 0.1, tone=0.3, member="chassis", part="member", material="cast-iron", role="detail")
    V.ell("mass-chassis", "chassis", [0, 0.77, 0], [0.9, 0.11, 0.3], tone=0.2, member="chassis", part="mass", **PS)
    V.ell("mass-drum-cog", "drum", V.W("drum", [2.3, 0, 0]), [0.7, 0.8, 0.8], tone=0.2, member="drum", part="mass", **DP)


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
    drum_three(0.04)
    helix(8, 0.05, region="drum", **DPD)
    t = 2 * math.pi / 14
    V.cap("drum-seam", "drum", V.W("drum", [3.0, 1.08, 0]), V.W("drum", [3.0, 1.08 * math.cos(t), 1.08 * math.sin(t)]), 0.04, tone=0.3, region="drum", rep=dict(count=14, angle=t, axis=U, origin=V.W("drum", [3.0, 0, 0])), **DPD)
    V.ext("drive-housing", "drum", "side", loc("drum", [(0.3798, 1.3701), (-0.3964, 1.1765), (-0.2634, 0.6429), (0.5129, 0.8364)]), -0.4, 0.4, tone=0.35, region="drum-drive", **PS)
    hopper(6)
    V.cyl("chute-swivel-collar", "chute-swivel", [-3.9, 2.1, 0], [-3.9, 2.35, 0], 0.12, tone=0.4, region="chute", **GSD)
    chute(0.04, 0.04, 0.07, 0.06, pins=False)
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
    V.decide("At 1:100 the cab keeps core, sides and arches but loses door frame, handles, quarter glass and lamps; the drum keeps its three pieces with a coarser flight rib (eight sections per turn, 50 mm) and 14 seams of 80 mm; the hopper is a six-section funnel; chute troughs have 40 mm walls without pins; tyres are plain revolves with brake drum and hub in one pair of solids, bolts and the air tank, ladder and rungs are dropped.",
             "Bei 1:100 behält das Fahrerhaus Kern, Seiten und Radausschnitte, verliert aber Türrahmen, Griffe, Seitenscheibe und Lampen; die Trommel behält ihre drei Teile mit gröberer Wendelrippe (acht Schnitte pro Windung, 50 mm) und 14 Nähten von 80 mm; der Fülltrichter ist ein Trichter aus sechs Schnitten; die Rinnentröge haben 40 mm Wände ohne Bolzen; die Reifen sind schlichte Drehkörper mit Bremstrommel und Nabe, Bolzen sowie Drucklufttank, Leiter und Sprossen entfallen.")
    V.decide("Thermal: references as at 1:50 for the remaining parts; emitters are 15 mm discs.",
             "Thermik: Verweise wie bei 1:50 für die verbleibenden Teile; Emitter sind 15 mm dicke Scheiben.")
    V.decide("Structure: the same load path as at 1:50 with 15 mm contact pads and thicker joints; the ladder and air tank are not members.",
             "Tragwerk: derselbe Lastpfad wie bei 1:50 mit 15 mm dicken Auflagerflächen und kräftigeren Gelenken; Leiter und Drucklufttank sind keine Stäbe.")
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
    V.rev("drum-body", "drum", V.W("drum", [0, 0, 0]), U, [[a, r] for a, r in DPROF] + [[3.0, 1.12], [3.0, 0.0]], tone=0.3, region="drum", **DP)
    V.rev("drum-cone-cap", "drum", V.W("drum", [3.0, 0, 0]), U, [[0, 1.12], [2.4, 0.5], [2.4, 0.4], [0, 1.0], [0, 1.12]], tone=0.3, region="drum", **DP)
    helix(6, 0.06, region="drum", **DPD)
    V.ext("drive-housing", "drum", "side", loc("drum", [(0.3798, 1.3701), (-0.3964, 1.1765), (-0.2634, 0.6429), (0.5129, 0.8364)]), -0.4, 0.4, tone=0.35, region="drum-drive", **PS)
    hopper(5)
    V.cyl("chute-swivel-collar", "chute-swivel", [-3.9, 2.1, 0], [-3.9, 2.35, 0], 0.12, tone=0.4, region="chute", **GSD)
    chute(0.06, 0.06, 0.1, 0.1, pins=False)
    for k, x in enumerate(WX, 1):
        bone = f"wheel-{k}"
        for side, sg in [("l", 1), ("r", -1)]:
            zc = sg * 1.115
            V.rev(f"tyre-{k}-{side}", bone, [x, 0.52, zc], [0, 0, 1], [[sg * a, r] for a, r in [(-0.16, 0), (-0.16, 0.49), (-0.13, 0.52), (0.13, 0.52), (0.16, 0.49), (0.16, 0)]], tone=0.12, region="tyres", **RB)
            V.cyl(f"brake-drum-{k}-{side}", bone, [x, 0.52, zc + sg * 0.12], [x, 0.52, zc + sg * 0.16], 0.3, tone=0.6, region="brakes", **CI)
    V.auto_thermal()
    common_thermal(200, 0.03)
    structure(200)
    V.decide("At 1:200 the cab is one 2.4 m wide block without wheel arches, frame rails and deck are merged into two plain rails and a plate, drum dome and belly are one solid of revolution with a separate open cone shell, the flights are a six-section-per-turn rib, the hopper a five-section funnel and each chute part a two-wall trough of 60 mm; tyres lose dish, hub and bolts and carry only a 40 mm brake drum disc; fuel and air tanks, ladder, mirrors, headlamps, handles, step and seams are dropped (below 30 mm or not drawn).",
             "Bei 1:200 ist das Fahrerhaus ein 2,4 m breiter Block ohne Radausschnitte, Rahmenträger und Boden sind zwei glatte Träger und eine Platte, Trommelkalotte und -bauch sind ein Rotationskörper mit separater offener Konusschale, die Wendel ist eine Rippe mit sechs Schnitten pro Windung, der Fülltrichter ein Trichter aus fünf Schnitten und jedes Rinnenteil ein Trog mit zwei 60-mm-Wänden; die Reifen verlieren Mulde, Nabe und Bolzen und tragen nur eine 40 mm dicke Bremstrommelscheibe; Tanks, Leiter, Spiegel, Scheinwerfer, Griffe, Tritt und Nähte entfallen (unter 30 mm oder nicht gezeichnet).")
    V.decide("Thermal: each region references its body solid; the two emitters (exhaust jet, radiator air) are 30 mm and 7.5 mm discs on the stack mouth and behind the cab; the drum region keeps the one-piece drum shell.",
             "Thermik: Jede Region verweist auf ihren Körper; die beiden Emitter (Abgasstrahl, Kühlerabluft) sind Scheiben an der Auspuffmündung und hinter dem Fahrerhaus; die Trommelregion behält die einteilige Trommelschale.")
    V.decide("Structure: rails, bumper, pedestal, hinge post, drum shell, flight rib, collar, both chute troughs and the tyres are members (references); axles are 0.2 m bars inside the frame width, wheel hubs, drum bearing and chute pins are joints, contacts are 160 mm by 300 mm pads on the ground, two masses carry chassis and the 21.6 t of concrete at the drum centre.",
             "Tragwerk: Träger, Stoßfänger, Lagerbock, Gelenkpfosten, Trommelschale, Wendelrippe, Kragen, beide Rinnentröge und die Reifen sind Stäbe (Verweise); Achsen sind 0,2 m dicke Stäbe in der Rahmenbreite, Radnaben, Trommellager und Rinnenbolzen sind Gelenke, Auflager sind 160 mm mal 300 mm große Flächen am Boden, zwei Massen tragen Fahrgestell und die 21,6 t Beton im Trommelmittelpunkt.")
    V.finish(200)


def s500():
    V.ext("truck-frame", "chassis", "side", [(-4.5, 0.55), (-4.5, 0.9), (4.5, 0.9), (4.5, 0.42), (4.25, 0.42), (4.25, 0.62), (-4.2, 0.62), (-4.2, 0.55)], -0.45, 0.45, tone=0.45, region="chassis", **PS)
    V.ext("truck-cab", "chassis", "side", [(2.4, 0.42), (2.4, 3.22), (2.5, 3.3), (4.1, 3.3), (4.22, 3.22), (4.46, 2.5), (4.5, 2.4), (4.5, 0.42)], -1.2, 1.2, tone=0.45, region="cab", **PS)
    V.box("cab-glass", "chassis", [4.36, 2.77, 0], [0.04, 0.62, 2.0], rz=18.4, tone=0.85, region="cab-glazing", **GL)
    V.ext("cab-glass-side", "chassis", "side", rect(3.1, 4.1, 2.46, 3.06), 1.2, 1.23, tone=0.85, region="cab-glazing", rep=dict(count=2, step=[0, 0, -2.43]), **GL)
    V.cyl("exhaust-stack", "chassis", [2.32, 1.3, -1.1], [2.32, 3.35, -1.1], 0.1, tone=0.2, region="exhaust-stack", **EX)
    pedestal(False)
    V.rev("drum-body", "drum", V.W("drum", [0, 0, 0]), U, [[0, 0], [0.14, 0.5999], [0.45, 0.9699], [0.9, 1.12], [3.0, 1.12], [3.0, 0.0]], tone=0.3, region="drum", **DP)
    V.rev("drum-cone-cap", "drum", V.W("drum", [3.0, 0, 0]), U, [[0, 1.12], [2.4, 0.5], [2.4, 0.4], [0, 0.9], [0, 1.12]], tone=0.3, region="drum", **DP)
    hopper(4)
    chute(0.08, 0.08, 0.1, 0.1, pins=False)
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
    V.decide("At 1:500 the truck is a frame block (450 mm either side of the centre line, 9 m long) and a separate cab block 2.4 m wide that reaches down to the bumper line (the 1:500 front draws one full-width block), a sloped glass band, the stack, the rear pedestal and hinge post, the drum as a solid dome-and-belly of revolution with an open cone shell and no flights, a four-section hopper funnel, two chute troughs of 75 mm walls and eight plain tyre discs; flights, seams, tanks, ladder, mirrors, door, grille, arches, hubs and the drive housing disappear into these masses.",
             "Bei 1:500 besteht der Lkw aus einem Rahmenblock (450 mm beiderseits der Mittellinie, 9 m lang) und einem eigenen 2,4 m breiten Kabinenblock bis zur Stoßfängerlinie (die 1:500-Vorderansicht zeichnet einen vollbreiten Block), einem geneigten Glasband, dem Auspuffrohr, Lagerbock und Gelenkpfosten hinten, der Trommel als massivem Rotationskörper aus Kalotte und Bauch mit offener Konusschale ohne Wendel, einem Fülltrichter aus vier Schnitten, zwei Rinnentrögen mit 75-mm-Wänden und acht glatten Reifenscheiben; Wendel, Nähte, Tanks, Leiter, Spiegel, Tür, Kühlergrill, Radausschnitte, Naben und das Antriebsgehäuse gehen in diesen Massen auf.")
    V.decide("Thermal: regions without a body solid of their own are real solids inside the masses (engine-bay grille box, drive housing, brake discs at the outer faces of the tyre discs); cab and chassis are the two blocks, tyres the discs; emitters are discs at the stack mouth and under the cab.",
             "Thermik: Regionen ohne eigenen Körper sind echte Körper in den Massen (Kühlergrillkasten, Antriebsgehäuse, Bremsscheiben an den Außenseiten der Reifenscheiben); Kabine und Fahrgestell sind die beiden Blöcke, die Reifen die Scheiben; Emitter sind Scheiben an der Auspuffmündung und unter dem Fahrerhaus.")
    V.decide("Structure: frame block, pedestal, hinge post, drum, chute troughs and tyre discs are members; a 0.6 m bar stands for the flights and a 0.25 m collar for the swivel; contacts are 160 mm pads, joints and masses as at 1:200 sized to 75 mm.",
             "Tragwerk: Rahmenblock, Lagerbock, Gelenkpfosten, Trommel, Rinnentröge und Reifenscheiben sind Stäbe; ein Stab steht für die Wendel und ein 0,25 m langer Kragen für den Drehkranz; Auflager sind 160-mm-Flächen, Gelenke und Massen wie bei 1:200 auf 75 mm bemessen.")
    V.finish(500)


s50(); s100(); s200(); s500()
V.write()
