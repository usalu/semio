"""🛣️ 1:200 solids of `machinery/concrete-mixer-truck` (round 3): the truck mixer for a 30 mm smallest feature, in world rest coordinates (x ahead, y up, z left): frame and subframe are one bar per side, the cab stands on one front block, every wheel is a tyre with a dish, the drum keeps its hollow form."""

import math
import os as _o
from importlib import util as _u

_s = _u.spec_from_file_location("kit", _o.path.join(_o.path.dirname(_o.path.abspath(__file__)), "r3-concrete-mixer-truck-kit.py"))
K = _u.module_from_spec(_s)
_s.loader.exec_module(K)

sin, cos, pi, rad = math.sin, math.cos, math.pi, math.radians
C = "chassis"
PS = dict(material="paint-steel", role="body")
PSD = dict(material="paint-steel", role="detail")
DP = dict(material="drum-paint", role="body", region="drum")
GS = dict(material="galvanised-steel", role="body")
CI = dict(material="cast-iron", role="detail")
GL = dict(material="glass", role="glazing")
SIDES = (("l", 1.0), ("r", -1.0))
F = 0.031
RAIL = [(-3.75, 0.81), (4.10, 0.81), (4.10, 1.08), (2.45, 1.08), (2.45, 1.16), (-4.25, 1.16), (-4.25, 1.08), (-3.75, 1.08)]
SHELL_IN = [(0.04, 0.30), (0.065, 0.60), (0.115, 0.73), (0.195, 0.795), (1.105, 1.12), (1.905, 1.12), (5.40, 0.53)]
TYRE = [[-0.13, 0.29], [-0.157, 0.38], [-0.14, 0.49], [-0.10, 0.52], [0.10, 0.52], [0.14, 0.49], [0.157, 0.38], [0.13, 0.29], [-0.13, 0.29]]


def tilt(sg, deg):
    c, s = cos(rad(deg)), sin(rad(deg))
    return ([1, 0, 0], [0, c, -sg * s], [0, sg * s, c])


def arch():
    r = K.ARCH_R
    a0 = math.degrees(math.asin((1.007 - K.WR) / r))
    a1 = math.degrees(math.asin((0.62 - K.WR) / r))
    return [(K.CAB_R, 1.22)] + K.arc(K.AX[0], K.WR, r, 180 - a0, a1, 7) + [(3.70, 0.62), (3.70, 1.22)]


def truck(V):
    for n, sg in SIDES:
        z0, z1 = sorted([sg * 0.295, sg * 0.385])
        V.ext(f"frame-rail-{n}", C, "side", RAIL, z0, z1, tone=0.4, region="chassis", **PS)
    for i, x in enumerate([3.95, 0.40, -3.68]):
        V.box(f"frame-cross-{i + 1}", C, [x, 0.945, 0], [0.09, 0.2, 0.60], tone=0.38, region="chassis", **PS)
    secs = [(1.22, 4.48, 2.58, 1.14), (2.0, 4.468, 2.58, 1.14), (2.92, 4.37, 2.585, 1.095), (3.10, 4.02, 2.78, 0.86)]
    V.loft("cab-shell", C, [([(xf + xr) / 2, y, 0], (xf - xr) / 2, hw, 8) for y, xf, xr, hw in secs], hint=(1, 0, 0), axis=(0, 1, 0), tone=0.5, region="cab", **PS)
    front = [([4.28, 0.42, 0], 0.22, 1.16, 8), ([4.28, 0.95, 0], 0.22, 1.16, 8), ([4.09, 0.95, 0], 0.39, 1.14, 8), ([4.09, 1.22, 0], 0.39, 1.14, 8)]
    V.loft("cab-front", C, front, hint=(1, 0, 0), axis=(0, 1, 0), tone=0.45, region="cab", **PS)
    for n, sg in SIDES:
        z0, z1 = (0.80, 1.14) if sg > 0 else (-1.14, -0.80)
        V.ext(f"cab-wheelhouse-{n}", C, "side", arch(), z0, z1, tone=0.46, region="cab", **PS)
        V.box(f"door-glass-{n}", C, [3.89, 2.46, sg * 1.115], [0.74, 0.76, F], axes=tilt(sg, 2.6), tone=0.85, region="cab-glazing", **GL)
        V.box(f"mirror-{n}", C, [4.44, 2.56, sg * 1.20], [0.06, 0.42, 0.15], tone=0.2, region="cab", **PSD)
    V.box("windscreen", C, [4.42, 2.46, 0], [F, 0.82, 1.90], rz=5.19, tone=0.85, region="cab-glazing", **GL)
    V.box("front-grille", C, [4.482, 1.56, 0], [F, 0.56, 1.30], tone=0.2, region="cab", **PSD)
    V.box("engine", C, [3.48, 0.86, 0], [1.18, 0.76, 0.60], tone=0.3, region="engine-bay", **PSD)
    V.box("gearbox", C, [2.69, 0.70, 0], [0.40, 0.36, 0.40], tone=0.32, region="chassis", **PSD)
    V.cyl("prop-shaft", C, [2.50, 0.66, 0], [K.AX[3] + 0.26, 0.54, 0.05], 0.05, tone=0.25, region="chassis", **PSD)
    sx, sz = K.STACK
    ix, iz = K.INTAKE
    V.box("silencer", C, [2.09, 0.72, -0.83], [0.42, 0.56, 0.60], tone=0.3, region="chassis", **PS)
    V.cyl("exhaust-stack", C, [sx, 1.0, sz], [sx, 3.30, sz], 0.075, tone=0.2, region="exhaust-stack", material="exhaust-steel", role="body")
    V.rev("air-intake", C, [ix, 1.30, iz], [0, 1, 0], [[0, 0.075], [1.64, 0.075], [1.64, 0.13], [1.79, 0.13], [1.79, 0]], tone=0.3, region="chassis", **PS)
    V.box("air-filter", C, [2.09, 0.98, 0.83], [0.42, 0.62, 0.50], tone=0.33, region="chassis", **PS)
    V.box("fuel-tank", C, [-0.16, 0.74, 0.80], [0.92, 0.62, 0.62], cr=0.12, tone=0.6, region="chassis", material="galvanised-steel", role="body")
    V.box("battery-box", C, [0.50, 0.70, 0.78], [0.30, 0.42, 0.56], tone=0.3, region="chassis", **PS)
    V.box("air-tanks", C, [-0.09, 0.69, -0.74], [1.07, 0.55, 0.25], cr=0.1, tone=0.35, region="chassis", **PS)
    for n, sg in SIDES:
        z0, z1 = (0.84, 1.24) if sg > 0 else (-1.24, -0.84)
        V.ext(f"fender-2-{n}", C, "side", K.arc(K.AX[1], K.WR, 0.635, 5, 175, 9) + K.arc(K.AX[1], K.WR, 0.60, 175, 5, 9), z0, z1, tone=0.3, region="chassis", **PS)
        z0, z1 = (0.56, 1.275) if sg > 0 else (-1.275, -0.56)
        pr = K.arc(K.AX[2], K.WR, 0.635, 10, 90, 5) + K.arc(K.AX[3], K.WR, 0.635, 90, 170, 5) + K.arc(K.AX[3], K.WR, 0.60, 170, 90, 5) + K.arc(K.AX[2], K.WR, 0.60, 90, 10, 5)
        V.ext(f"fender-rear-{n}", C, "side", pr, z0, z1, tone=0.3, region="chassis", **PS)
        V.box(f"underrun-bracket-{n}", C, [-4.30, 0.80, sg * 0.34], [0.10, 0.66, 0.08], rz=-12, tone=0.38, region="chassis", **PS)
        U = [(0.25 * cos(rad(a)), 0.18 * sin(rad(a))) for a in range(180, 361, 30)] + [(0.215 * cos(rad(a)), 0.145 * sin(rad(a))) for a in range(360, 179, -30)]
        V.ext(f"spare-chute-{n}", C, "front", [(1.345 + y, sg * 0.97 + z) for z, y in U], -2.60, -1.45, tone=0.58, region="chute", **GS)
    V.box("tail-lamp-beam", C, [-4.435, 0.80, 0], [0.11, 0.14, 2.36], tone=0.5, region="chassis", **PS)
    V.box("underrun-bar", C, [-4.44, 0.50, 0], [0.12, 0.12, 2.40], tone=0.36, region="chassis", **PS)


def running_gear(V):
    for i in range(4):
        b = f"wheel-{i + 1}"
        V.cyl(f"axle-{i + 1}", b, [K.AX[i], 0.52, -0.92 if i < 2 else -0.66], [K.AX[i], 0.52, 0.92 if i < 2 else 0.66], 0.06 if i < 2 else 0.085, tone=0.3, region="chassis", **PS)
        if i >= 2:
            V.ell(f"differential-{i + 1}", b, [K.AX[i] + 0.03, 0.52, 0.06], [0.25, 0.23, 0.20], tone=0.3, region="chassis", **PS)
        for n, sg in SIDES:
            if i < 2:
                V.box(f"spring-{i + 1}-{n}", C, [K.AX[i], 0.68, sg * K.RAIL_Z], [1.56, 0.16, 0.09], tone=0.28, region="chassis", **PSD)
            for kind, zc, d in [("", K.ZF, 0.11)] if i < 2 else [("-outer", K.ZO, -0.11), ("-inner", K.ZI, 0.11)]:
                o, ax = [0, 0, sg * zc], [0, 0, sg]
                V.revl(f"tyre-{i + 1}-{n}{kind}", b, o, ax, TYRE, tone=0.12, region="tyres", material="rubber", role="body")
                lo, hi = sorted([d - 0.016, d + 0.016])
                V.revl(f"wheel-dish-{i + 1}-{n}{kind}", b, o, ax, [[-0.13, 0.305], [0.13, 0.305], [0.13, 0.27], [hi, 0.27], [hi, 0], [lo, 0], [lo, 0.27], [-0.13, 0.27], [-0.13, 0.305]], tone=0.6, region="chassis", material="paint-steel", role="detail")
            z0 = (K.ZF if i < 2 else K.ZI) - 0.13
            z1 = K.ZF + 0.24 if i < 2 else K.ZO + 0.08
            V.revl(f"wheel-hub-{i + 1}-{n}", b, [0, 0, sg * z0], [0, 0, sg], [[0, 0.21], [0.20, 0.21], [0.20, 0.14], [z1 - z0 - 0.03, 0.12], [z1 - z0, 0.08], [z1 - z0, 0]], tone=0.4, region="brakes", **CI)
    for n, sg in SIDES:
        sp = [([K.AX[2] + 0.02, 0.655, sg * 0.50], 0.05, 0.035, 6), ([-1.975, 0.72, sg * 0.50], 0.05, 0.07, 6), ([K.AX[3] - 0.02, 0.655, sg * 0.50], 0.05, 0.035, 6)]
        V.loft(f"bogie-spring-{n}", C, sp, hint=(0, 0, 1), tone=0.28, region="chassis", **PSD)
    V.box("bogie-saddle", C, [-1.975, 0.56, 0], [0.24, 0.52, 0.50], tone=0.32, region="chassis", **PSD)


def superstructure(V):
    for n, sg in SIDES:
        z0, z1 = sorted([sg * 0.40, sg * 0.46])
        V.ext(f"front-pedestal-{n}", C, "side", [(1.50, 1.16), (2.48, 1.16), (2.40, 1.62), (1.74, 1.78)], z0, z1, tone=0.48, region="chassis", **PS)
        z0, z1 = sorted([sg * 0.62, sg * 0.70])
        V.ext(f"rear-pedestal-{n}", C, "side", [(-1.70, 1.16), (-3.20, 1.16), (-2.82, 1.98), (-2.30, 1.98)], z0, z1, holes=[[(-2.05, 1.30), (-2.95, 1.30), (-2.72, 1.80), (-2.38, 1.80)]], tone=0.5, region="chassis", **PS)
        z0, z1 = sorted([sg * 0.50, sg * 0.58])
        V.ext(f"rear-frame-{n}", C, "side", [(-3.40, 1.16), (-3.62, 1.16), (-3.62, 2.20), (-4.05, 2.95), (-3.98, 3.40), (-3.92, 3.40), (-3.90, 2.95), (-3.55, 2.45), (-3.40, 2.45)], z0, z1, tone=0.5, region="chassis", **PS)
        V.box(f"water-tank-post-{n}", C, [2.25, 2.08, sg * 0.425], [0.08, 0.92, 0.05], tone=0.44, region="chassis", **PSD)
    V.box("front-pedestal-top", C, [2.07, 1.635, 0], [0.56, 0.06, 0.94], rz=-11, tone=0.46, region="chassis", **PS)
    V.box("rear-pedestal-beam", C, [-2.56, 1.94, 0], [0.40, 0.20, 1.40], tone=0.48, region="chassis", **PS)
    V.box("rear-frame-top-beam", C, [-3.975, 2.92, 0], [0.10, 0.08, 1.08], tone=0.48, region="chassis", **PS)
    V.cyl("water-tank", C, [2.25, 2.78, -0.80], [2.25, 2.78, 0.80], 0.25, tone=0.62, region="chassis", material="galvanised-steel", role="body")


def blade(V, id, phase, thick, n=7):
    d = V._meta(id, "drum-spiral", "loft", dict(tone=0.75, material="drum-paint", role="detail", region="drum"))
    out = []
    for k in range(n):
        s = 0.35 + 1.20 * k / (n - 1)
        th = phase + 2 * pi * (s - 0.35) / K.PITCH
        rr = K.drum_r(s) - 0.22
        out.append({"at": K.lib.rv([s, rr * cos(th), rr * sin(th)]), "rx": thick / 2, "ry": 0.18, "exponent": 4, "rotation": K.lib.rv(K.lib.quat([1.0, 0.0, 0.0], [0.0, cos(th), sin(th)], [0.0, -sin(th), cos(th)]))})
    d["sections"] = out
    d["capStart"] = True
    d["capEnd"] = True
    return V.put(d, {})


def drum(V):
    O, U = [0, 0, 0], [1, 0, 0]
    V.revl("drum-shell", "drum", O, U, [list(p) for p in K.DRUM] + [list(p) for p in SHELL_IN[::-1]] + [list(K.DRUM[0])], tone=0.3, **DP)
    V.revl("drum-track-ring", "drum", O, U, [[K.RING_S0, K.RING_IN], [K.RING_S0, K.RING_R], [K.RING_S1, K.RING_R], [K.RING_S1, K.RING_IN], [K.RING_S0, K.RING_IN]], tone=0.36, **DP)
    V.revl("drive-housing", "drum", [0, -0.02, 0], U, [[-0.72, 0], [-0.72, 0.10], [-0.46, 0.10], [-0.46, 0.33], [-0.10, 0.33], [-0.10, 0.38], [0.02, 0.38], [0.02, 0]], tone=0.35, region="drum-drive", **PS)
    blade(V, "drum-blade-1", 0.0, F)
    blade(V, "drum-blade-2", pi, F)
    V.loft("hopper-bowl", C, [([-3.86, 3.65, 0], 0.37, 0.50, 4), ([-3.84, 3.42, 0], 0.33, 0.44, 4), ([-3.74, 3.22, 0], 0.27, 0.36, 3)], hint=(1, 0, 0), axis=(0, 1, 0), caps=(False, False), tone=0.55, region="hopper", **GS)
    V.revl("hopper-spout", "drum", [0, -0.08, 0], U, [[5.22, 0.26], [5.22, 0.29], [5.70, 0.31], [5.70, 0.28], [5.22, 0.26]], tone=0.55, region="hopper", **GS)
    fn = [(K.dw(5.44, 0.50, 0), 0.20, 0.40, 3), ([-3.80, 2.36, 0], 0.22, 0.32, 2.5), ([-3.93, 2.10, 0], 0.19, 0.19, 2)]
    V.loft("discharge-funnel", C, fn, hint=(1, 0, 0), axis=(0, 1, 0), caps=(False, False), tone=0.5, region="chute", **GS)
    V.revl("chute-swivel-ring", "chute-swivel", [0, 0, 0], [1, 0, 0], [[0.02, 0.14], [0.02, 0.24], [0.08, 0.24], [0.08, 0.14], [0.02, 0.14]], tone=0.4, region="chute", **GS)
    U2 = lambda a, d: [(d * sin(rad(t)), a * cos(rad(t))) for t in range(0, 181, 30)] + [((d - F) * sin(rad(t)), (a - F) * cos(rad(t))) for t in range(180, -1, -30)]
    V.extl("chute-trough", "chute", "yz", U2(K.CHUTE_HW, K.CHUTE_D), 0.0, K.CHUTE_L, tone=0.5, region="chute", **GS)
    V.extl("chute-ext-trough", "chute-ext", "yz", U2(K.EXT_HW, K.EXT_D), 0.0, K.EXT_L, tone=0.6, region="chute", **GS)
    z0, z1 = 0.80, 1.22
    for k, z in enumerate([z0, z1]):
        V.cap(f"ladder-stile-{k + 1}", C, [-4.20, 0.78, z], [-4.44, 2.50, z], 0.018, slender=True, tone=0.4, region="chassis", **PSD)
        V.cap(f"guard-post-{k + 1}", C, [-4.44, 2.50, z], [-4.46, 3.50, z], 0.016, slender=True, tone=0.4, region="chassis", **PSD)
    for k in range(4):
        t = (k + 0.5) / 4
        V.cap(f"ladder-rung-{k + 1}", C, [-4.20 - 0.24 * t, 0.78 + 1.72 * t, z0], [-4.20 - 0.24 * t, 0.78 + 1.72 * t, z1], 0.016, tone=0.4, region="chassis", **PSD)
    V.box("platform", C, [-4.20, 2.50, 1.01], [0.52, 0.04, 0.46], tone=0.42, region="chassis", **PSD)


def thermal(V):
    V.reg = {sid: [(K.region_of(sid, r), lay) for r, lay in regs] for sid, regs in V.reg.items()}
    V.auto_thermal(skip=("drum-shell", "drum-blade-1", "drum-blade-2", "windscreen", "door-glass-l", "door-glass-r"))
    V.to("thermal")
    V.lists["thermal"].append({"id": "th-windscreen", "of": "windscreen", "region": "cab-glazing", "sectionFaces": ["-u"]})
    V.lists["thermal"].append({"id": "th-door-glass-l", "of": "door-glass-l", "region": "cab-glazing", "sectionFaces": ["-w"]})
    V.lists["thermal"].append({"id": "th-door-glass-r", "of": "door-glass-r", "region": "cab-glazing", "sectionFaces": ["+w"]})
    V.revl("th-drum-skin", "drum", [0, 0, 0], [1, 0, 0], [[0, 0]] + [list(p) for p in K.DRUM] + [[5.40, 0]], tone=0.3, material="drum-paint", role="body", region="drum")
    sx, sz = K.STACK
    V.cyl("th-exhaust-stack-jet", C, [sx, 3.30, sz], [sx, 3.30 + F, sz], 0.075, tone=0.2, region="exhaust-stack", tags=["exhaust-stack-jet"], material="exhaust-steel", role="detail")
    V.box("th-radiator-air", C, [2.875, 0.80, 0], [F, 0.52, 0.60], tone=0.2, region="engine-bay", tags=["radiator-air"], material="paint-steel", role="detail")
    V.to("body")


def structure(V):
    V.to("structure")
    for sid in ["frame-rail-l", "frame-rail-r", "frame-cross-1", "frame-cross-2", "frame-cross-3", "front-pedestal-l", "front-pedestal-r", "rear-pedestal-l", "rear-pedestal-r", "rear-pedestal-beam", "rear-frame-l", "rear-frame-r", "bogie-spring-l", "bogie-spring-r", "spring-1-l", "spring-1-r", "spring-2-l", "spring-2-r"]:
        V.member(sid, C)
    V.revl("drum-tube", "drum", [0, 0, 0], [1, 0, 0], [[0.18, 0.83], [1.10, 1.15], [1.90, 1.15], [4.32, 0.742], [4.32, 0.712], [1.905, 1.12], [1.10, 1.12], [0.19, 0.80], [0.18, 0.83]], member="drum", part="member", material="drum-paint", role="body", tone=0.3)
    V.revl("drum-head", "drum", [0, 0, 0], [1, 0, 0], [[0.0, 0], [0.0, 0.30], [0.03, 0.62], [0.09, 0.76], [0.18, 0.83], [0.19, 0.80], [0.115, 0.73], [0.065, 0.60], [0.04, 0.30], [0.04, 0]], member="drum", part="member", material="drum-paint", role="body", tone=0.32)
    V.member("drum-track-ring", "drum")
    V.member("drum-blade-1", "drum-spiral")
    V.member("drum-blade-2", "drum-spiral")
    V.member("chute-swivel-ring", "chute-swivel")
    V.member("chute-trough", "chute")
    V.member("chute-ext-trough", "chute-ext")
    for i in range(4):
        b = f"wheel-{i + 1}"
        V.member(f"axle-{i + 1}", b)
        for n, sg in SIDES:
            zs = K.ZF if i < 2 else (K.ZI + K.ZO) / 2
            V.cyl(f"joint-wheel-{i + 1}-{n}", b, [K.AX[i], K.WR, sg * (zs - 0.16)], [K.AX[i], K.WR, sg * (zs + 0.16)], 0.10, member=b, part="joint", material="cast-iron", role="detail", tone=0.3)
            V.box(f"support-contact-{i + 1}-{n}", b, [K.AX[i], F / 2, sg * zs], [0.30, F, 0.30 if i < 2 else 0.66], member=b, part="support", anchor=f"contact-{i + 1}-{n}", material="rubber", role="body", tone=0.12)
    V.revl("joint-drum-bearing", "drum", [0, 0, 0], [1, 0, 0], [[-0.06, 0], [-0.06, 0.18], [0.04, 0.18], [0.04, 0]], member="drum", part="joint", material="cast-iron", role="detail", tone=0.3)
    for n, sg in SIDES:
        a = K.dw(4.18, K.ROLLER * cos(rad(38)), sg * K.ROLLER * sin(rad(38)))
        b2 = K.dw(4.34, K.ROLLER * cos(rad(38)), sg * K.ROLLER * sin(rad(38)))
        V.cyl(f"joint-drum-roller-{n}", "drum", a, b2, 0.11, member="drum", part="joint", material="cast-iron", role="detail", tone=0.3)
    V.revl("joint-chute-swivel", "chute-swivel", [0, 0, 0], [1, 0, 0], [[0.02, 0], [0.02, 0.10], [0.08, 0.10], [0.08, 0]], member="chute-swivel", part="joint", material="cast-iron", role="detail", tone=0.3)
    V.revl("joint-chute", "chute", [0.03, 0.2, 0], [0, 0, 1], [[-0.17, 0], [-0.17, 0.03], [0.17, 0.03], [0.17, 0]], member="chute", part="joint", material="cast-iron", role="detail", tone=0.3)
    V.revl("joint-chute-ext", "chute-ext", [0.03, 0.05, 0], [0, 0, 1], [[-0.21, 0], [-0.21, 0.03], [0.21, 0.03], [0.21, 0]], member="chute-ext", part="joint", material="cast-iron", role="detail", tone=0.3)
    V.ell("mass-chassis", C, [0.9, 1.0, 0], [0.42, 0.13, 0.30], member=C, part="mass", material="paint-steel", role="body", tone=0.2)
    V.ell("mass-drum", "drum", K.dw(2.6), [0.34, 0.34, 0.34], member="drum", part="mass", material="drum-paint", role="body", tone=0.2)
    V.to("body")


def build(V):
    truck(V)
    running_gear(V)
    superstructure(V)
    drum(V)
    thermal(V)
    structure(V)
    V.decide(
        "At 1:200 frame rail and subframe are one stepped bar per side with three cross members, the cab is a four-section loft on one front block that merges bumper and lower front, with wheel houses, glazing, grille and mirror heads; engine and radiator are one block, the air tanks one pack, every axle one tube, each wheel a tyre with a one-piece dish and one hub with brake drum per wheel end, the drum a hollow shell of 30 mm with track ring and two blades of seven sections, the rear frame carries the hopper stays, the ladder keeps four rungs; steps, lamps, visor, stack shield and cap, mud flaps, rollers and trunnion are left out.",
        "Bei 1:200 sind Rahmenlängsträger und Hilfsrahmen je Seite ein gestufter Stab mit drei Querträgern, das Fahrerhaus ist ein Loft aus vier Schnitten auf einem Frontblock, der Stoßfänger und untere Front zusammenfasst, mit Radhäusern, Verglasung, Grill und Spiegelköpfen; Motor und Kühler sind ein Block, die Luftkessel ein Paket, jede Achse ein Rohr, jedes Rad ein Reifen mit einteiliger Schüssel und je Radende eine Nabe mit Bremstrommel, die Trommel eine hohle Schale von 30 mm mit Laufring und zwei Wendeln aus sieben Schnitten, der Heckrahmen trägt die Trichterstützen, die Leiter behält vier Sprossen; Tritte, Leuchten, Blende, Auspuffverkleidung und -kappe, Schmutzfänger, Laufrollen und Pendellager entfallen.",
    )
    V.decide(
        "Thermal at 1:200 references the body by region with the drum as closed skin and the two outlet patches. Structure is the load path with its own drum tube and head of 30 mm wall, the track ring, blades, frame bars, pedestals, rear frame, springs and axle tubes, joints at wheel bearings, drum bearing, rollers and chute hinges, contact pads and the two masses.",
        "Thermisch verweist 1:200 regionsweise auf den Körper, mit der Trommel als geschlossener Haut und den beiden Auslassfeldern. Das Tragwerk ist der Lastpfad mit eigenem Trommelrohr und -boden von 30 mm Wand, Laufring, Wendeln, Rahmenstäben, Lagerböcken, Heckrahmen, Federn und Achsrohren, Gelenken an Radlagern, Trommellager, Rollen und Schurrengelenken, Aufstandsflächen und den beiden Massen.",
    )
