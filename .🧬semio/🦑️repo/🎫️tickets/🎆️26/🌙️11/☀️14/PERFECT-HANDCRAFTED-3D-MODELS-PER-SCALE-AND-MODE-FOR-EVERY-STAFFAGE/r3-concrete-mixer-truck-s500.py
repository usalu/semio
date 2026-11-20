"""🏙️ 1:500 solids of `machinery/concrete-mixer-truck` (round 3): the truck mixer for a 75 mm smallest feature, in world rest coordinates (x ahead, y up, z left): one frame bar, a cab on its front block, eight wheels (the twin tyres merged), a solid drum on its two supports, hopper, chute and stack."""

import math
import os as _o
from importlib import util as _u

_s = _u.spec_from_file_location("kit", _o.path.join(_o.path.dirname(_o.path.abspath(__file__)), "r3-concrete-mixer-truck-kit.py"))
K = _u.module_from_spec(_s)
_s.loader.exec_module(K)

sin, cos, pi, rad = math.sin, math.cos, math.pi, math.radians
C = "chassis"
PS = dict(material="paint-steel", role="body")
GS = dict(material="galvanised-steel", role="body")
GL = dict(material="glass", role="glazing")
SIDES = (("l", 1.0), ("r", -1.0))
F = 0.076
DRUM_SOLID = [[0, 0], [0, 0.30], [0.09, 0.76], [0.18, 0.83], [1.10, 1.15], [1.90, 1.15], [4.20, 0.762], [4.20, 0.80], [4.32, 0.80], [4.32, 0.742], [5.40, 0.56], [5.40, 0.485], [1.92, 1.075], [1.10, 1.075], [0.22, 0.765], [0.165, 0.71], [0.076, 0.30], [0.076, 0], [0, 0]]


def tilt(sg, deg):
    c, s = cos(rad(deg)), sin(rad(deg))
    return ([1, 0, 0], [0, c, -sg * s], [0, sg * s, c])


def arch():
    r = K.ARCH_R
    a0 = math.degrees(math.asin((1.007 - K.WR) / r))
    a1 = math.degrees(math.asin((0.62 - K.WR) / r))
    return [(K.CAB_R, 1.22)] + K.arc(K.AX[0], K.WR, r, 180 - a0, a1, 5) + [(3.70, 0.62), (3.70, 1.22)]


def wheel(V, id, b, i, sg, z0, z1):
    w = z1 - z0
    V.revl(id, b, [0, 0, sg * z0], [0, 0, sg], [[0, 0], [0, 0.44], [0.06, 0.52], [w - 0.06, 0.52], [w, 0.44], [w, 0.20], [w - F, 0.20], [w - F, 0]], tone=0.12, region="tyres", material="rubber", role="body")
    V.cyl(id.replace("wheel", "wheel-hub"), b, [K.AX[i], K.WR, sg * (z1 - F)], [K.AX[i], K.WR, sg * (z1 + 0.02)], 0.15, tone=0.4, region="brakes", material="cast-iron", role="detail")


def body(V):
    V.box("frame", C, [-0.075, 0.985, 0], [8.35, 0.35, 0.78], tone=0.4, region="chassis", **PS)
    secs = [(1.22, 4.48, 2.58, 1.14), (2.0, 4.465, 2.58, 1.14), (2.92, 4.37, 2.585, 1.095), (3.10, 4.02, 2.78, 0.86)]
    V.loft("cab-shell", C, [([(xf + xr) / 2, y, 0], (xf - xr) / 2, hw, 6) for y, xf, xr, hw in secs], hint=(1, 0, 0), axis=(0, 1, 0), tone=0.5, region="cab", **PS)
    V.box("cab-front", C, [4.10, 0.82, 0], [0.80, 0.80, 2.30], cr=0.08, tone=0.45, region="cab", **PS)
    for n, sg in SIDES:
        z0, z1 = (0.80, 1.14) if sg > 0 else (-1.14, -0.80)
        V.ext(f"cab-wheelhouse-{n}", C, "side", arch(), z0, z1, tone=0.46, region="cab", **PS)
        V.box(f"door-glass-{n}", C, [3.89, 2.46, sg * 1.105], [0.74, 0.76, 0.05], axes=tilt(sg, 2.6), tone=0.85, region="cab-glazing", **GL)
        V.box(f"front-pedestal-{n}", C, [2.0, 1.44, sg * 0.43], [0.86, 0.56, 0.08], tone=0.48, region="chassis", **PS)
        z0, z1 = sorted([sg * 0.62, sg * 0.70])
        V.ext(f"rear-pedestal-{n}", C, "side", [(-1.70, 1.16), (-3.20, 1.16), (-2.82, 1.98), (-2.30, 1.98)], z0, z1, holes=[[(-2.05, 1.32), (-2.95, 1.32), (-2.72, 1.80), (-2.38, 1.80)]], tone=0.5, region="chassis", **PS)
        z0, z1 = sorted([sg * 0.50, sg * 0.58])
        V.ext(f"rear-frame-{n}", C, "side", [(-3.40, 1.16), (-3.62, 1.16), (-3.62, 2.20), (-4.05, 2.95), (-3.98, 3.40), (-3.90, 3.40), (-3.90, 2.95), (-3.55, 2.45), (-3.40, 2.45)], z0, z1, tone=0.5, region="chassis", **PS)
        z0, z1 = (0.56, 1.275) if sg > 0 else (-1.275, -0.56)
        pr = K.arc(K.AX[2], K.WR, 0.675, 12, 90, 3) + K.arc(K.AX[3], K.WR, 0.675, 90, 168, 3) + K.arc(K.AX[3], K.WR, 0.60, 168, 90, 3) + K.arc(K.AX[2], K.WR, 0.60, 90, 12, 3)
        V.ext(f"fender-rear-{n}", C, "side", pr, z0, z1, tone=0.3, region="chassis", **PS)
        z0, z1 = (0.84, 1.24) if sg > 0 else (-1.24, -0.84)
        V.ext(f"fender-2-{n}", C, "side", K.arc(K.AX[1], K.WR, 0.675, 8, 172, 6) + K.arc(K.AX[1], K.WR, 0.60, 172, 8, 6), z0, z1, tone=0.3, region="chassis", **PS)
        for i in (0, 1):
            wheel(V, f"wheel-{i + 1}-{n}", f"wheel-{i + 1}", i, sg, K.ZF - 0.1575, K.ZF + 0.1575)
        for i in (2, 3):
            wheel(V, f"wheel-{i + 1}-{n}", f"wheel-{i + 1}", i, sg, K.ZI - 0.1575, K.ZO + 0.1575)
    V.box("windscreen", C, [4.41, 2.46, 0], [0.05, 0.82, 1.90], rz=5.19, tone=0.85, region="cab-glazing", **GL)
    V.box("engine", C, [3.25, 0.84, 0], [0.90, 0.60, 0.60], tone=0.3, region="engine-bay", material="paint-steel", role="detail")
    for i in range(4):
        V.cyl(f"axle-{i + 1}", f"wheel-{i + 1}", [K.AX[i], 0.52, -0.87 if i < 2 else -0.60], [K.AX[i], 0.52, 0.87 if i < 2 else 0.60], 0.08, tone=0.3, region="chassis", **PS)
    sx, sz = K.STACK
    V.cyl("exhaust-stack", C, [sx, 1.16, sz], [sx, 3.30, sz], 0.075, tone=0.2, region="exhaust-stack", material="exhaust-steel", role="body")
    V.cyl("air-intake", C, [K.INTAKE[0], 1.16, K.INTAKE[1]], [K.INTAKE[0], 3.09, K.INTAKE[1]], 0.09, tone=0.3, region="chassis", **PS)
    V.box("fuel-tank", C, [0.01, 0.72, 0.80], [1.27, 0.60, 0.60], cr=0.1, tone=0.6, region="chassis", material="galvanised-steel", role="body")
    V.box("air-tanks", C, [-0.09, 0.69, -0.74], [1.07, 0.55, 0.25], tone=0.35, region="chassis", **PS)
    V.box("silencer", C, [2.09, 0.72, -0.83], [0.42, 0.56, 0.60], tone=0.3, region="chassis", **PS)
    V.box("underrun-bar", C, [-4.44, 0.53, 0], [0.12, 0.18, 2.40], tone=0.36, region="chassis", **PS)
    V.box("underrun-bracket", C, [-4.31, 0.86, 0], [0.12, 0.62, 0.78], tone=0.38, region="chassis", **PS)
    V.box("rear-pedestal-beam", C, [-2.56, 1.94, 0], [0.40, 0.20, 1.40], tone=0.48, region="chassis", **PS)
    V.cyl("water-tank", C, [2.25, 2.78, -0.80], [2.25, 2.78, 0.80], 0.25, tone=0.62, region="chassis", material="galvanised-steel", role="body")
    V.box("water-tank-post", C, [2.25, 2.12, 0], [0.10, 0.84, 0.90], tone=0.44, region="chassis", **PS)
    V.revl("drum-body", "drum", [0, 0, 0], [1, 0, 0], DRUM_SOLID, tone=0.3, region="drum", material="drum-paint", role="body")
    V.revl("drive-housing", "drum", [0, -0.02, 0], [1, 0, 0], [[-0.70, 0], [-0.70, 0.12], [-0.46, 0.12], [-0.46, 0.35], [0.0, 0.35], [0.0, 0]], tone=0.35, region="drum-drive", **PS)
    V.loft("hopper", C, [([-3.86, 3.65, 0], 0.37, 0.50, 4), ([-3.84, 3.42, 0], 0.33, 0.44, 4), ([-3.74, 3.20, 0], 0.27, 0.36, 3)], hint=(1, 0, 0), axis=(0, 1, 0), tone=0.55, region="hopper", **GS)
    V.loft("discharge-funnel", C, [(K.dw(5.44, 0.50, 0), 0.20, 0.40, 3), ([-3.80, 2.36, 0], 0.22, 0.32, 2.5), ([-3.93, 2.03, 0], 0.19, 0.19, 2)], hint=(1, 0, 0), axis=(0, 1, 0), tone=0.5, region="chute", **GS)
    D = lambda a, d: [(0.0, a), (d * 0.7, a * 0.75), (d, 0.0), (d * 0.7, -a * 0.75), (0.0, -a)]
    V.extl("chute-trough", "chute", "yz", D(K.CHUTE_HW, K.CHUTE_D), 0.0, K.CHUTE_L, tone=0.5, region="chute", **GS)
    V.extl("chute-ext-trough", "chute-ext", "yz", D(K.EXT_HW, K.EXT_D), 0.0, K.EXT_L, tone=0.6, region="chute", **GS)
    for k, z in enumerate([0.80, 1.22]):
        V.cap(f"ladder-stile-{k + 1}", C, [-4.20, 0.78, z], [-4.46, 3.50, z], 0.018, slender=True, tone=0.4, region="chassis", material="paint-steel", role="detail")
    V.box("platform", C, [-4.20, 2.50, 1.01], [0.52, F, 0.46], tone=0.42, region="chassis", material="paint-steel", role="detail")


def thermal(V):
    V.reg = {sid: [(K.region_of(sid, r), lay) for r, lay in regs] for sid, regs in V.reg.items()}
    V.auto_thermal(skip=("drum-body", "windscreen", "door-glass-l", "door-glass-r"))
    V.to("thermal")
    V.lists["thermal"].append({"id": "th-windscreen", "of": "windscreen", "region": "cab-glazing", "sectionFaces": ["-u"]})
    V.lists["thermal"].append({"id": "th-door-glass-l", "of": "door-glass-l", "region": "cab-glazing", "sectionFaces": ["-w"]})
    V.lists["thermal"].append({"id": "th-door-glass-r", "of": "door-glass-r", "region": "cab-glazing", "sectionFaces": ["+w"]})
    V.revl("th-drum-skin", "drum", [0, 0, 0], [1, 0, 0], [[0, 0]] + [list(p) for p in K.DRUM] + [[5.40, 0]], tone=0.3, material="drum-paint", role="body", region="drum")
    sx, sz = K.STACK
    V.cyl("th-exhaust-stack-jet", C, [sx, 3.30, sz], [sx, 3.30 + F, sz], 0.075, tone=0.2, region="exhaust-stack", tags=["exhaust-stack-jet"], material="exhaust-steel", role="detail")
    V.box("th-radiator-air", C, [2.762, 0.84, 0], [F, 0.52, 0.60], tone=0.2, region="engine-bay", tags=["radiator-air"], material="paint-steel", role="detail")
    V.to("body")


def blade(V, id, phase):
    d = V._meta(id, "drum-spiral", "loft", dict(tone=0.75, material="drum-paint", role="detail", member="drum-spiral", part="member"))
    out = []
    for k in range(5):
        s = 0.45 + 1.0 * k / 4
        th = phase + 2 * pi * (s - 0.45) / K.PITCH
        rr = K.drum_r(s) - 0.30
        out.append({"at": K.lib.rv([s, rr * cos(th), rr * sin(th)]), "rx": F / 2, "ry": 0.18, "exponent": 4, "rotation": K.lib.rv(K.lib.quat([1.0, 0.0, 0.0], [0.0, cos(th), sin(th)], [0.0, -sin(th), cos(th)]))})
    d["sections"] = out
    d["capStart"] = True
    d["capEnd"] = True
    return V.put(d, {})


def structure(V):
    V.to("structure")
    ST = dict(material="paint-steel", role="body", tone=0.4, member=C, part="member")
    for n, sg in SIDES:
        V.box(f"frame-rail-{n}", C, [0.175, 0.945, sg * K.RAIL_Z], [7.85, 0.27, 0.09], **ST)
        V.box(f"subframe-rail-{n}", C, [-0.90, 1.12, sg * K.RAIL_Z], [6.70, 0.08, 0.10], **ST)
        V.member(f"front-pedestal-{n}", C)
        V.member(f"rear-pedestal-{n}", C)
        V.member(f"rear-frame-{n}", C)
    for k, x in enumerate([3.95, 0.40, -3.68]):
        V.box(f"frame-cross-{k + 1}", C, [x, 0.945, 0], [0.09, 0.2, 0.60], **ST)
    V.member("rear-pedestal-beam", C)
    V.member("drum-body", "drum")
    blade(V, "drum-blade-1", 0.0)
    blade(V, "drum-blade-2", pi)
    V.revl("chute-swivel-ring", "chute-swivel", [0, 0, 0], [1, 0, 0], [[0.0, 0.12], [0.0, 0.24], [0.08, 0.24], [0.08, 0.12], [0.0, 0.12]], member="chute-swivel", part="member", material="galvanised-steel", role="body", tone=0.4)
    V.member("chute-trough", "chute")
    V.member("chute-ext-trough", "chute-ext")
    for i in range(4):
        b = f"wheel-{i + 1}"
        V.member(f"axle-{i + 1}", b)
        for n, sg in SIDES:
            zs = K.ZF if i < 2 else (K.ZI + K.ZO) / 2
            V.box(f"support-contact-{i + 1}-{n}", b, [K.AX[i], F / 2, sg * zs], [0.30, F, 0.30 if i < 2 else 0.66], member=b, part="support", anchor=f"contact-{i + 1}-{n}", material="rubber", role="body", tone=0.12)
    V.revl("joint-drum-bearing", "drum", [0, 0, 0], [1, 0, 0], [[-0.08, 0], [-0.08, 0.2], [0.04, 0.2], [0.04, 0]], member="drum", part="joint", material="cast-iron", role="detail", tone=0.3)
    V.ell("mass-chassis", C, [0.9, 1.0, 0], [0.42, 0.13, 0.30], member=C, part="mass", material="paint-steel", role="body", tone=0.2)
    V.ell("mass-drum", "drum", K.dw(2.6), [0.34, 0.34, 0.34], member="drum", part="mass", material="drum-paint", role="body", tone=0.2)
    V.to("body")


def build(V):
    body(V)
    thermal(V)
    structure(V)
    V.decide(
        "At 1:500 the truck is a frame bar with fuel tank, air tanks, silencer and underrun, a three-section cab on its front block with wheel houses and three panes, eight wheels (each twin pair one wide wheel) with a hub disc on four axle tubes, two fender arcs per side, the drum as one hollow revolve of 75 mm wall with its track ring on a drive housing and an open rear pedestal, the water tank, rear frame, hopper and funnel as closed forms, the chutes as half-round bars, stack, intake, two ladder stiles and the platform.",
        "Bei 1:500 ist der Lkw ein Rahmenstab mit Kraftstofftank, Luftkesseln, Schalldämpfer und Unterfahrschutz, ein Fahrerhaus aus drei Schnitten auf seinem Frontblock mit Radhäusern und drei Scheiben, acht Räder (jedes Zwillingspaar ein breites Rad) mit Nabenscheibe auf vier Achsrohren, je Seite zwei Kotflügelbögen, die Trommel als ein hohler Drehkörper mit 75 mm Wand und Laufring auf Antriebsgehäuse und offenem hinterem Lagerbock, Wassertank, Heckrahmen, Trichter und Auslauf als geschlossene Formen, die Schurren als Halbrundstäbe, Auspuff, Ansaugrohr, zwei Leiterholme und das Podest.",
    )
    V.decide(
        "Thermal at 1:500 references the merged body masses by region, with the drum as its closed outer skin and the two outlet patches. Structure has its own load path: two frame rails with subframe rails and three cross members, pedestal plates, rear frame, the drum shell with two blade ribs, swivel ring, chute bars, axle tubes, the drum bearing, a contact pad per wheel and the two masses.",
        "Thermisch verweist 1:500 regionsweise auf die zusammengefassten Körpermassen, mit der Trommel als geschlossener Außenhaut und den beiden Auslassfeldern. Das Tragwerk hat seinen eigenen Lastpfad: zwei Rahmenlängsträger mit Hilfsrahmenträgern und drei Querträgern, Lagerbockbleche, Heckrahmen, die Trommelschale mit zwei Wendelrippen, Drehkranz, Schurrenstäbe, Achsrohre, das Trommellager, je Rad eine Aufstandsfläche und die beiden Massen.",
    )
