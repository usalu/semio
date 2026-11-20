"""🚧 1:100 solids of `machinery/concrete-mixer-truck` (round 3): the 8x4 truck mixer restated for a 15 mm smallest feature, in world rest coordinates (x ahead, y up, z left): assemblies stay, their small fittings merge into them."""

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
F = 0.016


def tilt(sg, deg):
    c, s = cos(rad(deg)), sin(rad(deg))
    return ([1, 0, 0], [0, c, -sg * s], [0, sg * s, c])


def arch():
    r = K.ARCH_R
    a0 = math.degrees(math.asin((1.007 - K.WR) / r))
    a1 = math.degrees(math.asin((0.62 - K.WR) / r))
    return [(K.CAB_R, 1.22)] + K.arc(K.AX[0], K.WR, r, 180 - a0, a1, 10) + [(3.70, 0.62), (3.70, 1.22)]


def frame(V):
    for n, sg in SIDES:
        V.box(f"frame-rail-{n}", C, [(K.RAIL_X0 + K.RAIL_X1) / 2, 0.945, sg * K.RAIL_Z], [K.RAIL_X1 - K.RAIL_X0, 0.27, 0.085], tone=0.4, region="chassis", **PS)
        V.box(f"subframe-rail-{n}", C, [(K.SUB_X0 + K.SUB_X1) / 2, 1.12, sg * K.RAIL_Z], [K.SUB_X1 - K.SUB_X0, 0.08, 0.10], tone=0.42, region="chassis", **PS)
    for i, x in enumerate([3.95, 1.95, -0.62, -1.975, -3.68]):
        V.box(f"frame-cross-{i + 1}", C, [x, 0.945, 0], [0.09, 0.2, 0.595], tone=0.38, region="chassis", **PS)


def cab(V):
    secs = [(1.22, 4.48, 2.58, 1.14), (2.0, 4.468, 2.58, 1.14), (2.90, 4.385, 2.58, 1.10), (3.03, 4.26, 2.62, 1.035), (3.10, 4.00, 2.80, 0.84)]
    V.loft("cab-shell", C, [([(xf + xr) / 2, y, 0], (xf - xr) / 2, hw, 10) for y, xf, xr, hw in secs], hint=(1, 0, 0), axis=(0, 1, 0), tone=0.5, region="cab", **PS)
    V.loft("cab-front-lower", C, [([4.09, 0.95, 0], 0.39, 1.14, 10), ([4.09, 1.22, 0], 0.39, 1.14, 10)], hint=(1, 0, 0), axis=(0, 1, 0), tone=0.5, region="cab", **PS)
    V.loft("bumper", C, [([4.28, 0.42, 0], 0.22, 1.16, 8), ([4.28, 0.95, 0], 0.22, 1.16, 8)], hint=(1, 0, 0), axis=(0, 1, 0), tone=0.4, region="chassis", **PS)
    for n, sg in SIDES:
        z0, z1 = (0.80, 1.14) if sg > 0 else (-1.14, -0.80)
        V.ext(f"cab-wheelhouse-{n}", C, "side", arch(), z0, z1, tone=0.46, region="cab", **PS)
        V.box(f"cab-step-back-{n}", C, [3.875, 0.70, sg * 0.86], [0.33, 0.50, 0.03], tone=0.44, region="cab", **PS)
        for k, y in enumerate([0.47, 0.74]):
            V.box(f"cab-step-{n}-{k + 1}", C, [3.875, y, sg * 0.99], [0.33, 0.03, 0.30], tone=0.3, region="chassis", **PSD)
        V.box(f"door-glass-{n}", C, [3.89, 2.46, sg * 1.122], [0.74, 0.76, F], axes=tilt(sg, 2.6), tone=0.85, region="cab-glazing", **GL)
        V.box(f"mirror-{n}", C, [4.45, 2.56, sg * 1.205], [0.05, 0.42, 0.14], cr=0.02, tone=0.2, region="cab", **PSD)
        V.cap(f"mirror-arm-{n}", C, [4.36, 2.84, sg * 1.09], [4.44, 2.78, sg * 1.20], 0.016, tone=0.2, region="cab", **PSD)
        V.box(f"head-lamp-{n}", C, [4.49, 0.70, sg * 0.86], [0.03, 0.17, 0.34], tone=0.9, region="cab", material="glass", role="detail")
    V.box("windscreen", C, [4.427, 2.46, 0], [F, 0.82, 1.90], rz=5.19, tone=0.85, region="cab-glazing", **GL)
    V.box("front-grille", C, [4.486, 1.56, 0], [0.02, 0.56, 1.30], tone=0.2, region="cab", **PSD)
    V.box("sun-visor", C, [4.435, 2.905, 0], [0.085, 0.022, 2.0], rz=-12, tone=0.25, region="cab", **PSD)
    V.box("engine", C, [3.40, 0.86, 0], [1.00, 0.76, 0.60], cr=0.05, tone=0.3, region="engine-bay", **PSD)
    V.box("radiator", C, [4.02, 0.98, 0], [0.10, 0.74, 0.86], tone=0.26, region="engine-bay", **PSD)
    V.box("gearbox", C, [2.74, 0.70, 0], [0.50, 0.36, 0.40], cr=0.06, tone=0.32, region="chassis", **PSD)
    V.cyl("prop-shaft", C, [2.50, 0.66, 0], [K.AX[3] + 0.26, 0.54, 0.05], 0.05, tone=0.25, region="chassis", **PSD)


def equipment(V):
    sx, sz = K.STACK
    ix, iz = K.INTAKE
    V.box("silencer", C, [2.09, 0.72, -0.83], [0.42, 0.56, 0.60], cr=0.06, tone=0.3, region="chassis", **PS)
    V.cyl("exhaust-stack", C, [sx, 1.0, sz], [sx, 3.22, sz], 0.062, tone=0.2, region="exhaust-stack", material="exhaust-steel", role="body")
    V.cyl("stack-shield", C, [sx, 1.55, sz], [sx, 2.90, sz], 0.09, tone=0.24, region="exhaust-stack", material="exhaust-steel", role="body")
    V.cyl("stack-cap", C, [sx, 3.22, sz], [sx - 0.035, 3.30, sz], 0.062, 0.058, tone=0.25, region="exhaust-stack", material="exhaust-steel", role="detail")
    V.rev("air-intake", C, [ix, 1.30, iz], [0, 1, 0], [[0, 0.075], [1.62, 0.075], [1.65, 0.13], [1.75, 0.13], [1.79, 0.09], [1.79, 0]], tone=0.3, region="chassis", **PS)
    V.box("air-filter", C, [2.09, 0.98, 0.83], [0.42, 0.62, 0.50], cr=0.1, tone=0.33, region="chassis", **PS)
    V.loft("fuel-tank", C, [([0.30, 0.74, 0.80], 0.31, 0.31, 3.5), ([-0.62, 0.74, 0.80], 0.31, 0.31, 3.5)], hint=(0, 0, 1), tone=0.6, region="chassis", material="galvanised-steel", role="body")
    V.box("battery-box", C, [0.50, 0.70, 0.78], [0.30, 0.42, 0.56], tone=0.3, region="chassis", **PS)
    for k, y in enumerate([0.54, 0.84]):
        V.cap(f"air-tank-{k + 1}", C, [0.32, y, -0.74], [-0.50, y, -0.74], 0.125, tone=0.35, region="chassis", **PS)
    for n, sg in SIDES:
        z0, z1 = (0.84, 1.24) if sg > 0 else (-1.24, -0.84)
        V.ext(f"fender-2-{n}", C, "side", K.arc(K.AX[1], K.WR, 0.625, 5, 175, 12) + K.arc(K.AX[1], K.WR, 0.60, 175, 5, 12), z0, z1, tone=0.3, region="chassis", **PS)
        z0, z1 = (0.84, 1.16) if sg > 0 else (-1.16, -0.84)
        V.ext(f"fender-1-rear-{n}", C, "side", K.arc(K.AX[0], K.WR, 0.685, 132, 176, 4) + K.arc(K.AX[0], K.WR, 0.66, 176, 132, 4), z0, z1, tone=0.3, region="chassis", **PS)
        z0, z1 = (0.56, 1.275) if sg > 0 else (-1.275, -0.56)
        pr = K.arc(K.AX[2], K.WR, 0.625, 10, 90, 6) + K.arc(K.AX[3], K.WR, 0.625, 90, 170, 6) + K.arc(K.AX[3], K.WR, 0.60, 170, 90, 6) + K.arc(K.AX[2], K.WR, 0.60, 90, 10, 6)
        V.ext(f"fender-rear-{n}", C, "side", pr, z0, z1, tone=0.3, region="chassis", **PS)
        V.box(f"mud-flap-{n}", C, [-3.31, 0.44, sg * 0.92], [F, 0.42, 0.66], tone=0.15, region="chassis", material="rubber", role="detail")
        V.box(f"tail-lamp-{n}", C, [-4.46, 0.80, sg * 0.98], [0.06, 0.14, 0.40], tone=0.75, region="chassis", material="glass", role="detail")
        V.box(f"underrun-bracket-{n}", C, [-4.30, 0.80, sg * 0.34], [0.10, 0.66, 0.08], rz=-12, tone=0.38, region="chassis", **PS)
        U = [(0.25 * cos(rad(a)), 0.18 * sin(rad(a))) for a in range(180, 361, 20)] + [(0.234 * cos(rad(a)), 0.164 * sin(rad(a))) for a in range(360, 179, -20)]
        V.ext(f"spare-chute-{n}", C, "front", [(1.345 + y, sg * 0.97 + z) for z, y in U], -2.60, -1.45, tone=0.58, region="chute", **GS)
    V.box("tail-lamp-beam", C, [-4.405, 0.80, 0], [0.05, 0.10, 2.36], tone=0.36, region="chassis", **PS)
    V.box("underrun-bar", C, [-4.44, 0.50, 0], [0.12, 0.12, 2.40], tone=0.36, region="chassis", **PS)


def running_gear(V):
    for i in (0, 1):
        V.box(f"axle-beam-{i + 1}", f"wheel-{i + 1}", [K.AX[i], 0.52, 0], [0.10, 0.09, 1.84], tone=0.3, region="chassis", **PS)
        for n, sg in SIDES:
            V.box(f"spring-{i + 1}-{n}", C, [K.AX[i], 0.66, sg * K.RAIL_Z], [1.56, 0.14, 0.09], tone=0.28, region="chassis", **PSD)
    for i in (2, 3):
        V.cyl(f"axle-housing-{i + 1}", f"wheel-{i + 1}", [K.AX[i], 0.52, -0.66], [K.AX[i], 0.52, 0.66], 0.085, tone=0.3, region="chassis", **PS)
        V.ell(f"differential-{i + 1}", f"wheel-{i + 1}", [K.AX[i] + 0.03, 0.52, 0.06], [0.25, 0.23, 0.20], tone=0.3, region="chassis", **PS)
    for n, sg in SIDES:
        sp = [([K.AX[2] + 0.02, 0.655, sg * 0.50], 0.05, 0.035, 6), ([-1.975, 0.72, sg * 0.50], 0.05, 0.07, 6), ([K.AX[3] - 0.02, 0.655, sg * 0.50], 0.05, 0.035, 6)]
        V.loft(f"bogie-spring-{n}", C, sp, hint=(0, 0, 1), tone=0.28, region="chassis", **PSD)
    V.cyl("bogie-trunnion", C, [-1.975, 0.72, -0.58], [-1.975, 0.72, 0.58], 0.055, tone=0.3, region="chassis", **PS)
    V.box("bogie-saddle", C, [-1.975, 0.56, 0], [0.24, 0.52, 0.50], tone=0.32, region="chassis", **PSD)


TYRE = [[-0.13, 0.29], [-0.157, 0.36], [-0.157, 0.46], [-0.12, 0.52], [0.12, 0.52], [0.157, 0.46], [0.157, 0.36], [0.13, 0.29], [-0.13, 0.29]]


def wheel(V, i, n, sg, zc, kind):
    b = f"wheel-{i + 1}"
    tag = f"{i + 1}-{n}" + ("" if kind == "front" else "-" + kind)
    o = [0, 0, sg * zc]
    ax = [0, 0, sg]
    d = {"front": 0.11, "outer": -0.11, "inner": 0.11}[kind]
    V.revl(f"tyre-{tag}", b, o, ax, TYRE, tone=0.12, region="tyres", material="rubber", role="body")
    lo, hi = sorted([d - 0.01, d + 0.01])
    dish = [[-0.13, 0.305], [0.13, 0.305], [0.13, 0.275], [hi, 0.275], [hi, 0], [lo, 0], [lo, 0.275], [-0.13, 0.275], [-0.13, 0.305]]
    V.revl(f"wheel-dish-{tag}", b, o, ax, dish, tone=0.6, region="chassis", material="paint-steel", role="detail")
    if kind == "front":
        V.revl(f"hub-{tag}", b, [0, 0, sg * (zc + d)], ax, [[0, 0.14], [0.03, 0.14], [0.04, 0.10], [0.115, 0.085], [0.13, 0]], tone=0.3, region="brakes", **CI)
    if kind == "outer":
        V.revl(f"hub-{tag}", b, [0, 0, sg * (zc + d)], ax, [[0, 0.15], [0.03, 0.15], [0.04, 0.125], [0.215, 0.115], [0.24, 0]], tone=0.3, region="brakes", **CI)


def wheels(V):
    for i in range(4):
        for n, sg in SIDES:
            if i < 2:
                wheel(V, i, n, sg, K.ZF, "front")
            else:
                wheel(V, i, n, sg, K.ZO, "outer")
                wheel(V, i, n, sg, K.ZI, "inner")
            zb = (K.ZF if i < 2 else K.ZI) - 0.02
            V.cyl(f"brake-drum-{i + 1}-{n}", f"wheel-{i + 1}", [K.AX[i], K.WR, sg * (zb - 0.09)], [K.AX[i], K.WR, sg * (zb + 0.09)], 0.21, tone=0.6, region="brakes", **CI)


def superstructure(V):
    for n, sg in SIDES:
        z0, z1 = sorted([sg * 0.40, sg * 0.45])
        V.ext(f"front-pedestal-{n}", C, "side", [(1.50, 1.16), (2.48, 1.16), (2.40, 1.62), (1.74, 1.78)], z0, z1, holes=[[(1.80, 1.28), (2.24, 1.28), (2.18, 1.52), (1.90, 1.58)]], tone=0.48, region="chassis", **PS)
        z0, z1 = sorted([sg * 0.62, sg * 0.70])
        V.ext(f"rear-pedestal-{n}", C, "side", [(-1.70, 1.16), (-3.20, 1.16), (-2.82, 1.98), (-2.30, 1.98)], z0, z1, holes=[[(-2.05, 1.30), (-2.95, 1.30), (-2.72, 1.80), (-2.38, 1.80)]], tone=0.5, region="chassis", **PS)
        a = K.dw(4.19, K.ROLLER * cos(rad(38)), sg * K.ROLLER * sin(rad(38)))
        b = K.dw(4.33, K.ROLLER * cos(rad(38)), sg * K.ROLLER * sin(rad(38)))
        V.cyl(f"support-roller-{n}", C, a, b, 0.11, tone=0.3, region="chassis", **PSD)
        z0, z1 = sorted([sg * 0.50, sg * 0.58])
        V.ext(f"rear-frame-{n}", C, "side", [(-3.40, 1.16), (-3.62, 1.16), (-3.62, 2.20), (-4.05, 2.95), (-3.90, 2.95), (-3.55, 2.45), (-3.40, 2.45)], z0, z1, tone=0.5, region="chassis", **PS)
        V.cap(f"hopper-stay-{n}", C, [-3.98, 2.95, sg * 0.54], [-3.98, 3.40, sg * 0.50], 0.025, tone=0.4, region="chassis", **PSD)
        V.box(f"water-tank-post-{n}", C, [2.25, 2.08, sg * 0.425], [0.08, 0.92, 0.05], tone=0.44, region="chassis", **PSD)
    V.box("front-pedestal-top", C, [2.07, 1.635, 0], [0.56, 0.05, 0.94], rz=-11, tone=0.46, region="chassis", **PS)
    V.box("rear-pedestal-beam", C, [-2.56, 1.94, 0], [0.34, 0.20, 1.40], tone=0.48, region="chassis", **PS)
    V.box("rear-frame-top-beam", C, [-3.975, 2.92, 0], [0.10, 0.08, 1.08], tone=0.48, region="chassis", **PS)
    V.rev("water-tank", C, [2.25, 2.78, -0.80], [0, 0, 1], [[0, 0], [0.01, 0.14], [0.04, 0.22], [0.08, 0.25], [1.52, 0.25], [1.56, 0.22], [1.59, 0.14], [1.60, 0]], tone=0.62, region="chassis", material="galvanised-steel", role="body")


def blade(V, id, phase):
    d = V._meta(id, "drum-spiral", "loft", dict(tone=0.75, material="drum-paint", role="detail", region="drum"))
    out = []
    for k in range(13):
        s = 0.30 + 1.25 * k / 12
        th = phase + 2 * pi * (s - 0.30) / K.PITCH
        rr = K.drum_r(s) - 0.19
        out.append({"at": K.lib.rv([s, rr * cos(th), rr * sin(th)]), "rx": F / 2, "ry": 0.18, "exponent": 6, "rotation": K.lib.rv(K.lib.quat([1.0, 0.0, 0.0], [0.0, cos(th), sin(th)], [0.0, -sin(th), cos(th)]))})
    d["sections"] = out
    d["capStart"] = True
    d["capEnd"] = True
    return V.put(d, {})


def drum(V):
    O, U = [0, 0, 0], [1, 0, 0]
    t = 0.005
    inner = [(0.012, 0.30), (0.038, 0.615), (0.096, 0.754), (0.184, 0.824), (1.101, 1.15 - t), (1.901, 1.15 - t), (5.40, 0.56 - t)]
    V.revl("drum-shell", "drum", O, U, [list(p) for p in K.DRUM] + [list(p) for p in inner[::-1]] + [list(K.DRUM[0])], tone=0.3, **DP)
    V.revl("drum-track-ring", "drum", O, U, [[K.RING_S0, K.RING_IN], [K.RING_S0, K.RING_R], [K.RING_S1, K.RING_R], [K.RING_S1, K.RING_IN], [K.RING_S0, K.RING_IN]], tone=0.36, **DP)
    V.revl("drum-mouth-rim", "drum", O, U, [[5.33, 0.565], [5.33, K.LIP_R], [5.40, K.LIP_R], [5.40, 0.553], [5.33, 0.565]], tone=0.34, **DP)
    V.revl("drive-housing", "drum", [0, -0.02, 0], U, [[-0.72, 0], [-0.72, 0.09], [-0.46, 0.11], [-0.46, 0.25], [-0.42, 0.33], [-0.12, 0.33], [-0.10, 0.38], [0.02, 0.38], [0.02, 0]], tone=0.35, region="drum-drive", **PS)
    V.box("oil-cooler", C, [1.98, 1.52, 0.56], [0.46, 0.44, 0.16], tone=0.3, region="drum-drive", **PS)
    blade(V, "drum-blade-1", 0.0)
    blade(V, "drum-blade-2", pi)
    V.loft("hopper-bowl", C, [([-3.86, 3.65, 0], 0.37, 0.50, 4), ([-3.84, 3.42, 0], 0.33, 0.44, 4), ([-3.74, 3.22, 0], 0.27, 0.36, 3)], hint=(1, 0, 0), axis=(0, 1, 0), caps=(False, False), tone=0.55, region="hopper", **GS)
    V.revl("hopper-spout", "drum", [0, -0.08, 0], U, [[5.22, 0.275], [5.22, 0.29], [5.70, 0.31], [5.70, 0.295], [5.22, 0.275]], tone=0.55, region="hopper", **GS)
    fn = [(K.dw(5.44, 0.50, 0), 0.20, 0.40, 3), ([-3.80, 2.36, 0], 0.22, 0.32, 2.5), ([-3.93, 2.10, 0], 0.19, 0.19, 2)]
    V.loft("discharge-funnel", C, fn, hint=(1, 0, 0), axis=(0, 1, 0), caps=(False, False), tone=0.5, region="chute", **GS)


def chute(V):
    V.revl("chute-swivel-ring", "chute-swivel", [0, 0, 0], [1, 0, 0], [[0.02, 0.17], [0.02, 0.24], [0.08, 0.24], [0.08, 0.17], [0.02, 0.17]], tone=0.4, region="chute", **GS)
    U = lambda a, d: [(d * sin(rad(t)), a * cos(rad(t))) for t in range(0, 181, 20)] + [((d - F) * sin(rad(t)), (a - F) * cos(rad(t))) for t in range(180, -1, -20)]
    V.extl("chute-trough", "chute", "yz", U(K.CHUTE_HW, K.CHUTE_D), 0.0, K.CHUTE_L, tone=0.5, region="chute", **GS)
    V.extl("chute-ext-trough", "chute-ext", "yz", U(K.EXT_HW, K.EXT_D), 0.0, K.EXT_L, tone=0.6, region="chute", **GS)


def ladder(V):
    z0, z1 = 0.80, 1.22
    for k, z in enumerate([z0, z1]):
        V.cap(f"ladder-stile-{k + 1}", C, [-4.20, 0.78, z], [-4.44, 2.50, z], 0.018, slender=True, tone=0.4, region="chassis", **PSD)
        V.cap(f"guard-post-{k + 1}", C, [-4.44, 2.50, z], [-4.46, 3.50, z], 0.016, slender=True, tone=0.4, region="chassis", **PSD)
    for k in range(5):
        t = (k + 0.5) / 5
        V.cap(f"ladder-rung-{k + 1}", C, [-4.20 - 0.24 * t, 0.78 + 1.72 * t, z0], [-4.20 - 0.24 * t, 0.78 + 1.72 * t, z1], 0.014, tone=0.4, region="chassis", **PSD)
    V.box("platform", C, [-4.20, 2.50, 1.01], [0.52, 0.03, 0.46], tone=0.42, region="chassis", **PSD)
    V.cap("guard-rail-top", C, [-4.46, 3.50, z0], [-4.46, 3.50, z1], 0.016, tone=0.4, region="chassis", **PSD)
    V.cap("platform-stay", C, [-3.80, 2.50, 0.58], [-3.95, 2.50, 0.80], 0.02, tone=0.4, region="chassis", **PSD)


def thermal(V):
    V.reg = {sid: [(K.region_of(sid, r), lay) for r, lay in regs] for sid, regs in V.reg.items()}
    V.auto_thermal(skip=("drum-shell", "drum-blade-1", "drum-blade-2", "windscreen", "door-glass-l", "door-glass-r"))
    V.to("thermal")
    V.lists["thermal"].append({"id": "th-windscreen", "of": "windscreen", "region": "cab-glazing", "sectionFaces": ["-u"]})
    V.lists["thermal"].append({"id": "th-door-glass-l", "of": "door-glass-l", "region": "cab-glazing", "sectionFaces": ["-w"]})
    V.lists["thermal"].append({"id": "th-door-glass-r", "of": "door-glass-r", "region": "cab-glazing", "sectionFaces": ["+w"]})
    V.revl("th-drum-skin", "drum", [0, 0, 0], [1, 0, 0], [[0, 0]] + [list(p) for p in K.DRUM] + [[5.40, 0]], tone=0.3, material="drum-paint", role="body", region="drum")
    sx, sz = K.STACK
    V.cyl("th-exhaust-stack-jet", C, [sx - 0.035, 3.30, sz], [sx - 0.042, 3.316, sz], 0.058, tone=0.2, region="exhaust-stack", tags=["exhaust-stack-jet"], material="exhaust-steel", role="detail")
    V.box("th-radiator-air", C, [2.892, 0.80, 0], [F, 0.52, 0.60], tone=0.2, region="engine-bay", tags=["radiator-air"], material="paint-steel", role="detail")
    V.to("body")


def structure(V):
    V.to("structure")
    for sid in ["frame-rail-l", "frame-rail-r", "subframe-rail-l", "subframe-rail-r"] + [f"frame-cross-{k}" for k in range(1, 6)]:
        V.member(sid, C)
    for sid in ["front-pedestal-l", "front-pedestal-r", "front-pedestal-top", "rear-pedestal-l", "rear-pedestal-r", "rear-pedestal-beam", "rear-frame-l", "rear-frame-r", "rear-frame-top-beam", "bogie-spring-l", "bogie-spring-r", "bogie-trunnion", "bogie-saddle"] + [f"spring-{i}-{n}" for i in (1, 2) for n in "lr"]:
        V.member(sid, C)
    for sid in ["drum-shell", "drum-track-ring"]:
        V.member(sid, "drum")
    V.member("drum-blade-1", "drum-spiral")
    V.member("drum-blade-2", "drum-spiral")
    V.member("chute-swivel-ring", "chute-swivel")
    V.member("chute-trough", "chute")
    V.member("chute-ext-trough", "chute-ext")
    for i in range(4):
        b = f"wheel-{i + 1}"
        V.member(f"axle-beam-{i + 1}" if i < 2 else f"axle-housing-{i + 1}", b)
        for n, sg in SIDES:
            for kind in [""] if i < 2 else ["-outer", "-inner"]:
                V.member(f"wheel-dish-{i + 1}-{n}{kind}", b)
            zs = K.ZF if i < 2 else (K.ZI + K.ZO) / 2
            V.cyl(f"joint-wheel-{i + 1}-{n}", b, [K.AX[i], K.WR, sg * (zs - 0.16)], [K.AX[i], K.WR, sg * (zs + 0.16)], 0.07, member=b, part="joint", material="cast-iron", role="detail", tone=0.3)
            V.box(f"support-contact-{i + 1}-{n}", b, [K.AX[i], F / 2, sg * zs], [0.30, F, 0.30 if i < 2 else 0.66], member=b, part="support", anchor=f"contact-{i + 1}-{n}", material="rubber", role="body", tone=0.12)
    V.revl("joint-drum-bearing", "drum", [0, 0, 0], [1, 0, 0], [[-0.05, 0], [-0.05, 0.16], [0.03, 0.16], [0.03, 0]], member="drum", part="joint", material="cast-iron", role="detail", tone=0.3)
    V.member("support-roller-l", "drum", part="joint", new_id="joint-drum-roller-l")
    V.member("support-roller-r", "drum", part="joint", new_id="joint-drum-roller-r")
    V.revl("joint-chute-swivel", "chute-swivel", [0, 0, 0], [1, 0, 0], [[0.02, 0], [0.02, 0.10], [0.08, 0.10], [0.08, 0]], member="chute-swivel", part="joint", material="cast-iron", role="detail", tone=0.3)
    V.revl("joint-chute", "chute", [0.03, 0.16, 0], [0, 0, 1], [[-0.17, 0], [-0.17, 0.025], [0.17, 0.025], [0.17, 0]], member="chute", part="joint", material="cast-iron", role="detail", tone=0.3)
    V.revl("joint-chute-ext", "chute-ext", [0.03, 0.05, 0], [0, 0, 1], [[-0.225, 0], [-0.225, 0.02], [0.225, 0.02], [0.225, 0]], member="chute-ext", part="joint", material="cast-iron", role="detail", tone=0.3)
    V.ell("mass-chassis", C, [0.9, 1.0, 0], [0.42, 0.13, 0.30], member=C, part="mass", material="paint-steel", role="body", tone=0.2)
    V.ell("mass-drum", "drum", K.dw(2.6), [0.34, 0.34, 0.34], member="drum", part="mass", material="drum-paint", role="body", tone=0.2)
    V.to("body")


def build(V):
    frame(V)
    cab(V)
    equipment(V)
    running_gear(V)
    wheels(V)
    superstructure(V)
    drum(V)
    chute(V)
    ladder(V)
    thermal(V)
    structure(V)
    V.decide(
        "At 1:100 every assembly of the truck stays but is restated for a 15 mm feature: the frame rails are plain bars with five cross members, the cab is a five-section loft with wheel houses, one step unit per side, glazing, grille, visor, mirrors and lamps, engine and sump are one block, one propeller shaft runs through, each wheel is a plain tyre with a one-piece dished wheel, hub and brake drum, the leaf springs are single bars, hangers, dampers, torque rods, tank straps, door lines, wheel nuts and tread grooves are left out, the drum keeps its hollow shell, track ring, mouth rim and two blades of thirteen sections, the drive is one housing.",
        "Bei 1:100 bleibt jede Baugruppe des Lkw, neu gefasst für 15 mm Kleinstmaß: die Rahmenlängsträger sind schlichte Stäbe mit fünf Querträgern, das Fahrerhaus ist ein Loft aus fünf Schnitten mit Radhäusern, je Seite ein Trittblock, Verglasung, Grill, Blende, Spiegel und Leuchten, Motor und Ölwanne sind ein Block, eine Gelenkwelle läuft durch, jedes Rad ist ein schlichter Reifen mit einteiliger Radschüssel, Nabe und Bremstrommel, die Blattfedern sind einzelne Stäbe, Federböcke, Dämpfer, Lenker, Tankbänder, Türfugen, Radmuttern und Profilrillen entfallen, die Trommel behält hohle Schale, Laufring, Mündungsring und zwei Wendeln aus dreizehn Schnitten, der Antrieb ist ein Gehäuse.",
    )
    V.decide(
        "Thermal at 1:100 references the body region by region; the drum is its closed outer skin, the panes count their outer face, the two outlets are the stack mouth and a patch behind the engine. Structure references the frame, subframe, pedestals, rear frame, springs, axles, wheels, drum shell with track ring, blades and chute troughs, with joints at wheel bearings, drum bearing, rollers and chute hinges, one contact pad per tyre set and the two masses.",
        "Thermisch verweist 1:100 Region für Region auf den Körper; die Trommel ist ihre geschlossene Außenhaut, die Scheiben zählen mit der Außenfläche, die beiden Auslässe sind die Auspuffmündung und ein Feld hinter dem Motor. Das Tragwerk verweist auf Rahmen, Hilfsrahmen, Lagerböcke, Heckrahmen, Federn, Achsen, Räder, Trommelschale mit Laufring, Wendeln und Schurrentröge, mit Gelenken an Radlagern, Trommellager, Rollen und Schurrengelenken, je Reifensatz einer Aufstandsfläche und den beiden Massen.",
    )
