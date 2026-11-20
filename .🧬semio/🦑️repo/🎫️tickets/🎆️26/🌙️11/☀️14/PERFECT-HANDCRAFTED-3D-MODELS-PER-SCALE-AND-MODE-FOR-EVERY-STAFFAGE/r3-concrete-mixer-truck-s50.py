"""🏗️ 1:50 solids of `machinery/concrete-mixer-truck` (round 3): every assembly of the real 8x4 truck mixer stated by hand, part by part, in world rest coordinates (x ahead, y up, z left)."""

import math

from importlib import util as _u
import os as _o

_s = _u.spec_from_file_location("kit", _o.path.join(_o.path.dirname(_o.path.abspath(__file__)), "r3-concrete-mixer-truck-kit.py"))
K = _u.module_from_spec(_s)
_s.loader.exec_module(K)

sin, cos, pi, rad = math.sin, math.cos, math.pi, math.radians
PS = dict(material="paint-steel", role="body")
PSD = dict(material="paint-steel", role="detail")
DP = dict(material="drum-paint", role="body", region="drum")
GS = dict(material="galvanised-steel", role="body")
CI = dict(material="cast-iron", role="detail")
GL = dict(material="glass", role="glazing")
RB = dict(material="rubber", role="body")
EX = dict(material="exhaust-steel", role="body")
SIDES = (("l", 1.0), ("r", -1.0))
CAB_SEC = [(1.22, 4.48, 2.58, 1.14), (1.95, 4.48, 2.58, 1.14), (2.02, 4.465, 2.58, 1.14), (2.90, 4.385, 2.58, 1.10), (2.98, 4.34, 2.585, 1.08), (3.05, 4.23, 2.64, 1.02), (3.10, 4.00, 2.80, 0.84)]
CAB_E = 12


def circle(r, n, cx=0.0, cy=0.0, a0=0.0):
    return [(cx + r * cos(a0 + 2 * pi * i / n), cy + r * sin(a0 + 2 * pi * i / n)) for i in range(n)]


def tilt_axes(sg, deg):
    c, s = cos(rad(deg)), sin(rad(deg))
    return ([1, 0, 0], [0, c, -sg * s], [0, sg * s, c])


def arch_profile():
    cx, cy, r = K.AX[0], K.WR, K.ARCH_R
    a0 = math.degrees(math.asin((1.007 - cy) / r))
    a1 = math.degrees(math.asin((0.62 - cy) / r))
    arc = K.arc(cx, cy, r, 180 - a0, a1, 16)
    return [(K.CAB_R, 1.22)] + arc + [(3.70, 0.62), (3.70, 1.22)]


def frame(V):
    for n, sg in SIDES:
        c = [(0.81, 0.381), (1.08, 0.381), (1.08, 0.296), (1.072, 0.296), (1.072, 0.373), (0.818, 0.373), (0.818, 0.296), (0.81, 0.296)]
        V.ext(f"frame-rail-{n}", "chassis", "front", [(y, sg * z) for y, z in c], K.RAIL_X0, K.RAIL_X1, tone=0.4, region="chassis", **PS)
        V.box(f"subframe-rail-{n}", "chassis", [(K.SUB_X0 + K.SUB_X1) / 2, 1.12, sg * K.RAIL_Z], [K.SUB_X1 - K.SUB_X0, 0.08, 0.10], tone=0.42, region="chassis", **PS)
    for i, x in enumerate([3.95, 2.62, 1.95, 0.40, -0.62, -1.975, -3.68]):
        V.box(f"frame-cross-{i + 1}", "chassis", [x, 0.945, 0], [0.09, 0.2, 0.592], tone=0.38, region="chassis", **PS)
    for i, x in enumerate([1.20, -0.30, -1.60, -4.20]):
        V.box(f"subframe-cross-{i + 1}", "chassis", [x, 1.12, 0], [0.08, 0.08, 0.58], tone=0.42, region="chassis", **PS)


def cab(V):
    secs = [([(xf + xr) / 2, y, 0], (xf - xr) / 2, hw, CAB_E) for y, xf, xr, hw in CAB_SEC]
    V.loft("cab-shell", "chassis", secs, hint=(1, 0, 0), axis=(0, 1, 0), tone=0.5, region="cab", **PS)
    low = [([4.09, 0.95, 0], 0.39, 1.14, CAB_E), ([4.09, 1.22, 0], 0.39, 1.14, CAB_E)]
    V.loft("cab-front-lower", "chassis", low, hint=(1, 0, 0), axis=(0, 1, 0), tone=0.5, region="cab", **PS)
    for n, sg in SIDES:
        z0, z1 = (0.80, 1.14) if sg > 0 else (-1.14, -0.80)
        V.ext(f"cab-wheelhouse-{n}", "chassis", "side", arch_profile(), z0, z1, tone=0.46, region="cab", **PS)
        for k, x in enumerate([3.715, 4.035]):
            V.box(f"cab-step-cheek-{n}-{k + 1}", "chassis", [x, 0.695, sg * 0.99], [0.03, 0.51, 0.30], tone=0.44, region="cab", **PS)
        for k, y in enumerate([0.47, 0.74]):
            V.box(f"cab-step-{n}-{k + 1}", "chassis", [3.875, y, sg * 0.99], [0.29, 0.03, 0.30], tone=0.3, region="chassis", **PSD)
        V.box(f"door-glass-{n}", "chassis", [3.89, 2.46, sg * 1.124], [0.74, 0.76, 0.012], axes=tilt_axes(sg, 2.6), cr=0.004, tone=0.85, region="cab-glazing", **GL)
        V.box(f"door-handle-{n}", "chassis", [3.52, 1.80, sg * 1.15], [0.16, 0.04, 0.03], cr=0.01, tone=0.2, region="cab", **PSD)
        for k, x in enumerate([3.36, 4.34]):
            V.box(f"door-line-{n}-{k + 1}", "chassis", [x, 1.64, sg * 1.141], [0.012, 0.76, 0.008], texture=True, tone=0.3, region="cab", **PSD)
            V.box(f"door-line-{n}-{k + 3}", "chassis", [x, 2.47, sg * 1.121], [0.012, 0.90, 0.008], axes=tilt_axes(sg, 2.6), texture=True, tone=0.3, region="cab", **PSD)
        V.box(f"door-line-{n}-5", "chassis", [3.85, 1.265, sg * 1.141], [0.98, 0.012, 0.008], texture=True, tone=0.3, region="cab", **PSD)
        V.cap(f"mirror-arm-{n}", "chassis", [4.36, 2.84, sg * 1.09], [4.44, 2.78, sg * 1.20], 0.016, tone=0.2, region="cab", **PSD)
        V.box(f"mirror-head-{n}", "chassis", [4.45, 2.56, sg * 1.205], [0.05, 0.42, 0.14], cr=0.02, tone=0.2, region="cab", **PSD)
        V.box(f"head-lamp-{n}", "chassis", [4.49, 0.70, sg * 0.86], [0.03, 0.17, 0.34], cr=0.012, tone=0.9, region="cab", material="glass", role="detail")
    V.box("windscreen", "chassis", [4.429, 2.46, 0], [0.012, 0.82, 1.90], rz=5.19, cr=0.004, tone=0.85, region="cab-glazing", **GL)
    V.box("sun-visor", "chassis", [4.435, 2.905, 0], [0.085, 0.022, 2.0], rz=-12, cr=0.008, tone=0.25, region="cab", **PSD)
    V.box("front-grille", "chassis", [4.486, 1.56, 0], [0.02, 0.56, 1.30], cr=0.008, tone=0.2, region="cab", **PSD)
    for k in range(5):
        V.box(f"grille-bar-{k + 1}", "chassis", [4.497, 1.36 + 0.10 * k, 0], [0.006, 0.03, 1.22], texture=True, tone=0.32, region="cab", **PSD)
    bump = [([4.28, 0.42, 0], 0.22, 1.16, 8), ([4.28, 0.95, 0], 0.22, 1.16, 8)]
    V.loft("bumper", "chassis", bump, hint=(1, 0, 0), axis=(0, 1, 0), tone=0.4, region="chassis", **PS)
    V.box("engine", "chassis", [3.40, 0.92, 0], [1.00, 0.62, 0.60], cr=0.05, tone=0.3, region="engine-bay", **PSD)
    V.box("oil-sump", "chassis", [3.36, 0.535, 0], [0.62, 0.17, 0.36], cr=0.04, tone=0.28, region="engine-bay", **PSD)
    V.box("radiator", "chassis", [4.02, 0.98, 0], [0.10, 0.74, 0.86], cr=0.01, tone=0.26, region="engine-bay", **PSD)
    V.box("gearbox", "chassis", [2.74, 0.70, 0], [0.50, 0.36, 0.40], cr=0.06, tone=0.32, region="chassis", **PSD)


def equipment(V):
    sx, sz = K.STACK
    V.box("silencer", "chassis", [2.09, 0.72, -0.83], [0.42, 0.56, 0.60], cr=0.06, tone=0.3, region="chassis", **PS)
    V.cyl("exhaust-elbow", "chassis", [2.30, 0.98, sz], [sx, 1.12, sz], 0.062, tone=0.2, region="exhaust-stack", **EX)
    V.cyl("exhaust-stack", "chassis", [sx, 1.10, sz], [sx, 3.22, sz], 0.062, tone=0.2, region="exhaust-stack", **EX)
    V.revl("stack-shield", "chassis", [sx, 1.55, sz], [0, 1, 0], [[0, 0.06], [0, 0.09], [1.35, 0.09], [1.35, 0.06], [0, 0.06]], tone=0.24, region="exhaust-stack", **EX)
    V.cyl("stack-cap", "chassis", [sx, 3.22, sz], [sx - 0.035, 3.30, sz], 0.062, 0.058, tone=0.25, region="exhaust-stack", material="exhaust-steel", role="detail")
    ix, iz = K.INTAKE
    V.cyl("air-intake-pipe", "chassis", [ix, 1.30, iz], [ix, 2.92, iz], 0.075, tone=0.3, region="chassis", **PS)
    V.cyl("air-intake-elbow", "chassis", [2.28, 1.18, 0.90], [ix, 1.32, iz], 0.075, tone=0.3, region="chassis", **PS)
    V.rev("air-intake-cap", "chassis", [ix, 2.92, iz], [0, 1, 0], [[0, 0.075], [0.03, 0.13], [0.13, 0.13], [0.17, 0.09], [0.18, 0]], tone=0.28, region="chassis", **PSD)
    V.box("air-filter", "chassis", [2.09, 0.98, 0.83], [0.42, 0.62, 0.50], cr=0.1, tone=0.33, region="chassis", **PS)
    tank = [([0.30, 0.74, 0.80], 0.31, 0.31, 3.5), ([-0.62, 0.74, 0.80], 0.31, 0.31, 3.5)]
    V.loft("fuel-tank", "chassis", tank, hint=(0, 0, 1), tone=0.6, region="chassis", material="galvanised-steel", role="body")
    for k, x in enumerate([0.10, -0.42]):
        V.loft(f"fuel-tank-strap-{k + 1}", "chassis", [([x + 0.025, 0.74, 0.80], 0.318, 0.318, 3.5), ([x - 0.025, 0.74, 0.80], 0.318, 0.318, 3.5)], hint=(0, 0, 1), tone=0.3, region="chassis", **PSD)
    V.cyl("fuel-filler", "chassis", [0.12, 1.04, 0.95], [0.12, 1.085, 0.95], 0.05, tone=0.25, region="chassis", **PSD)
    V.box("battery-box", "chassis", [0.50, 0.70, 0.78], [0.30, 0.42, 0.56], cr=0.02, tone=0.3, region="chassis", **PS)
    for k, y in enumerate([0.54, 0.84]):
        V.cap(f"air-tank-{k + 1}", "chassis", [0.32, y, -0.74], [-0.50, y, -0.74], 0.125, tone=0.35, region="chassis", **PS)
    V.box("air-tank-bracket", "chassis", [-0.09, 0.80, -0.50], [0.08, 0.50, 0.20], tone=0.33, region="chassis", **PSD)
    for n, sg in SIDES:
        f2 = K.arc(K.AX[1], K.WR, 0.62, 5, 175, 17) + K.arc(K.AX[1], K.WR, 0.60, 175, 5, 17)
        z0, z1 = (0.84, 1.24) if sg > 0 else (-1.24, -0.84)
        V.ext(f"fender-2-{n}", "chassis", "side", f2, z0, z1, tone=0.3, region="chassis", **PS)
        pr = K.arc(K.AX[2], K.WR, 0.62, 10, 90, 8) + K.arc(K.AX[3], K.WR, 0.62, 90, 170, 8)
        inner = K.arc(K.AX[3], K.WR, 0.60, 170, 90, 8) + K.arc(K.AX[2], K.WR, 0.60, 90, 10, 8)
        z0, z1 = (0.56, 1.275) if sg > 0 else (-1.275, -0.56)
        V.ext(f"fender-rear-{n}", "chassis", "side", pr + inner, z0, z1, tone=0.3, region="chassis", **PS)
        V.box(f"mud-flap-{n}", "chassis", [-3.31, 0.44, sg * 0.92], [0.012, 0.42, 0.66], tone=0.15, region="chassis", material="rubber", role="detail")
        z0, z1 = (0.84, 1.16) if sg > 0 else (-1.16, -0.84)
        V.ext(f"fender-1-rear-{n}", "chassis", "side", K.arc(K.AX[0], K.WR, 0.68, 132, 176, 6) + K.arc(K.AX[0], K.WR, 0.66, 176, 132, 6), z0, z1, tone=0.3, region="chassis", **PS)
        V.box(f"mud-flap-1-{n}", "chassis", [2.352, 0.41, sg * 1.0], [0.012, 0.34, 0.30], tone=0.15, region="chassis", material="rubber", role="detail")
        V.box(f"tail-lamp-{n}", "chassis", [-4.46, 0.80, sg * 0.98], [0.06, 0.14, 0.40], cr=0.012, tone=0.75, region="chassis", material="glass", role="detail")
        V.box(f"underrun-bracket-{n}", "chassis", [-4.30, 0.80, sg * 0.34], [0.10, 0.66, 0.08], rz=-12, tone=0.38, region="chassis", **PS)
        U = [(0.25 * cos(rad(a)), 0.18 * sin(rad(a))) for a in range(180, 361, 15)]
        Ui = [(0.245 * cos(rad(a)), 0.175 * sin(rad(a))) for a in range(360, 179, -15)]
        V.ext(f"spare-chute-{n}", "chassis", "front", [(1.345 + y, sg * 0.97 + z) for z, y in U + Ui], -2.60, -1.45, tone=0.58, region="chute", **GS)
    V.box("tail-lamp-beam", "chassis", [-4.405, 0.80, 0], [0.05, 0.10, 2.36], tone=0.36, region="chassis", **PS)
    V.box("underrun-bar", "chassis", [-4.44, 0.50, 0], [0.12, 0.12, 2.40], cr=0.012, tone=0.36, region="chassis", **PS)


def running_gear(V):
    for i in (0, 1):
        b = f"wheel-{i + 1}"
        V.box(f"axle-beam-{i + 1}", b, [K.AX[i], 0.52, 0], [0.10, 0.09, 1.84], cr=0.02, tone=0.3, region="chassis", **PS)
        for n, sg in SIDES:
            V.box(f"spring-{i + 1}-{n}", "chassis", [K.AX[i], 0.635, sg * K.RAIL_Z], [1.56, 0.09, 0.09], tone=0.28, region="chassis", **PSD)
            for k, dx in enumerate([-0.74, 0.74]):
                V.box(f"spring-hanger-{i + 1}-{n}-{k + 1}", "chassis", [K.AX[i] + dx, 0.745, sg * K.RAIL_Z], [0.10, 0.13, 0.10], tone=0.33, region="chassis", **PSD)
            V.cap(f"damper-{i + 1}-{n}", "chassis", [K.AX[i] + 0.16, 0.56, sg * 0.50], [K.AX[i] + 0.30, 1.02, sg * 0.44], 0.03, tone=0.25, region="chassis", **PSD)
    for i in (2, 3):
        b = f"wheel-{i + 1}"
        V.cyl(f"axle-housing-{i + 1}", b, [K.AX[i], 0.52, -0.66], [K.AX[i], 0.52, 0.66], 0.085, tone=0.3, region="chassis", **PS)
        V.ell(f"differential-{i + 1}", b, [K.AX[i] + 0.03, 0.52, 0.06], [0.25, 0.23, 0.20], tone=0.3, region="chassis", **PS)
    for n, sg in SIDES:
        sp = [([K.AX[2] + 0.02, 0.655, sg * 0.50], 0.05, 0.035, 6), ([-1.975, 0.72, sg * 0.50], 0.05, 0.07, 6), ([K.AX[3] - 0.02, 0.655, sg * 0.50], 0.05, 0.035, 6)]
        V.loft(f"bogie-spring-{n}", "chassis", sp, hint=(0, 0, 1), tone=0.28, region="chassis", **PSD)
        V.box(f"bogie-hanger-{n}", "chassis", [-1.975, 0.80, sg * 0.44], [0.30, 0.10, 0.20], tone=0.33, region="chassis", **PSD)
        for k, (xa, xb) in enumerate([(K.AX[2] - 0.05, -1.95), (K.AX[3] + 0.05, -2.0)]):
            V.cap(f"torque-rod-{n}-{k + 1}", "chassis", [xa, 0.42, sg * 0.42], [xb, 0.50, sg * 0.42], 0.025, tone=0.25, region="chassis", **PSD)
    V.cyl("bogie-trunnion", "chassis", [-1.975, 0.72, -0.58], [-1.975, 0.72, 0.58], 0.055, tone=0.3, region="chassis", **PS)
    V.box("bogie-saddle", "chassis", [-1.975, 0.50, 0], [0.24, 0.40, 0.50], cr=0.04, tone=0.32, region="chassis", **PSD)
    V.cyl("prop-shaft", "chassis", [2.50, 0.66, 0], [K.AX[2] + 0.26, 0.54, 0.06], 0.05, tone=0.25, region="chassis", **PSD)
    V.cyl("prop-shaft-2", "chassis", [K.AX[2] - 0.22, 0.54, 0.06], [K.AX[3] + 0.26, 0.54, 0.06], 0.045, tone=0.25, region="chassis", **PSD)


def tyre_profile():
    h = [(-0.128, 0.288), (-0.15, 0.33), (-0.1575, 0.41), (-0.152, 0.47), (-0.135, 0.508), (-0.118, 0.52)]
    pts = h + [(-a, r) for a, r in h[::-1]]
    return [list(p) for p in pts] + [list(pts[0])]


def tread(V, tag, b, o, ax):
    for k, g in enumerate((-0.066, 0.0, 0.066)):
        V.revl(f"tread-groove-{tag}-{k + 1}", b, o, ax, [[g - 0.008, 0.5195], [g - 0.008, 0.5205], [g + 0.008, 0.5205], [g + 0.008, 0.5195], [g - 0.008, 0.5195]], texture=True, tone=0.05, material="rubber", role="detail")


def wheel(V, i, n, sg, zc, kind):
    b = f"wheel-{i + 1}"
    tag = f"{i + 1}-{n}" + ("" if kind == "front" else "-" + kind)
    o = lambda dz: [0, 0, sg * (zc + dz)]
    ax = [0, 0, sg]
    V.revl(f"tyre-{tag}", b, o(0), ax, tyre_profile(), tone=0.12, region="tyres", **RB)
    tread(V, tag, b, o(0), ax)
    disc = {"front": 0.111, "outer": -0.168, "inner": 0.168}[kind]
    V.revl(f"rim-{tag}", b, o(0), ax, [[-0.13, 0.275], [-0.13, 0.31], [-0.118, 0.31], [-0.118, 0.293], [0.118, 0.293], [0.118, 0.31], [0.13, 0.31], [0.13, 0.275], [-0.13, 0.275]], tone=0.6, region="chassis", material="paint-steel", role="detail")
    if kind != "front":
        e0, e1 = (-0.176, -0.13) if kind == "outer" else (0.13, 0.176)
        V.revl(f"rim-dish-{tag}", b, o(0), ax, [[e0, 0.262], [e0, 0.29], [e1, 0.29], [e1, 0.262], [e0, 0.262]], tone=0.6, region="chassis", material="paint-steel", role="detail")
    holes = [circle(0.024, 10, 0.222 * cos(2 * pi * k / 10 + pi / 10), 0.222 * sin(2 * pi * k / 10 + pi / 10)) for k in range(10)]
    za, zb = sorted([sg * (zc + disc - 0.007), sg * (zc + disc + 0.007)])
    V.extl(f"wheel-disc-{tag}", b, "xy", circle(0.27, 40), za, zb, holes=holes, tone=0.6, region="chassis", material="paint-steel", role="detail")
    if kind == "inner":
        return
    face = disc + 0.007
    if kind == "front":
        V.revl(f"hub-{tag}", b, o(face - 0.02), ax, [[0, 0.14], [0.03, 0.14], [0.04, 0.10], [0.11, 0.085], [0.125, 0.06], [0.125, 0]], tone=0.3, region="brakes", **CI)
    else:
        V.revl(f"hub-{tag}", b, o(face - 0.02), ax, [[0, 0.15], [0.03, 0.15], [0.04, 0.125], [0.21, 0.115], [0.235, 0.08], [0.235, 0]], tone=0.3, region="brakes", **CI)
    V.cyl(f"wheel-nuts-{tag}", b, [K.AX[i] + 0.1675, K.WR, sg * (zc + face)], [K.AX[i] + 0.1675, K.WR, sg * (zc + face + 0.03)], 0.016, rep={"count": 10, "angle": 2 * pi / 10, "axis": [0, 0, 1], "origin": [K.AX[i], K.WR, 0]}, tone=0.5, region="brakes", **CI)


def wheels(V):
    for i in range(4):
        for n, sg in SIDES:
            if i < 2:
                wheel(V, i, n, sg, K.ZF, "front")
                zb = K.ZF - 0.02
            else:
                wheel(V, i, n, sg, K.ZO, "outer")
                wheel(V, i, n, sg, K.ZI, "inner")
                zb = K.ZI - 0.02
            V.cyl(f"brake-drum-{i + 1}-{n}", f"wheel-{i + 1}", [K.AX[i], K.WR, sg * (zb - 0.09)], [K.AX[i], K.WR, sg * (zb + 0.09)], 0.21, tone=0.6, region="brakes", **CI)


def shell_loop(t):
    inner = [(0.012, 0.30), (0.038, 0.615), (0.096, 0.754), (0.184, 0.824), (1.101, 1.15 - t), (1.901, 1.15 - t), (5.40, 0.56 - t)]
    return [list(p) for p in K.DRUM] + [list(p) for p in inner[::-1]] + [list(K.DRUM[0])]


def blade(V, id, phase, **kw):
    n = 26
    d = V._meta(id, "drum-spiral", "loft", dict(tone=0.75, **kw))
    out = []
    rx, ry = 0.012, 0.18
    for k in range(n + 1):
        s = 0.30 + 1.25 * k / n
        th = phase + 2 * pi * (s - 0.30) / K.PITCH
        rr = K.drum_r(s) - 0.186
        at = [s, rr * cos(th), rr * sin(th)]
        ex = [1.0, 0.0, 0.0]
        ey = [0.0, cos(th), sin(th)]
        ez = [0.0, -sin(th), cos(th)]
        out.append({"at": K.lib.rv(at), "rx": rx, "ry": ry, "exponent": 6, "rotation": K.lib.rv(K.lib.quat(ex, ey, ez))})
    d["sections"] = out
    d["capStart"] = True
    d["capEnd"] = True
    return V.put(d, {})


def drum(V):
    O = [0, 0, 0]
    U = [1, 0, 0]
    V.revl("drum-shell", "drum", O, U, shell_loop(0.005), tone=0.3, **DP)
    V.revl("drum-track-ring", "drum", O, U, [[K.RING_S0, K.RING_IN], [K.RING_S0, K.RING_R], [K.RING_S1, K.RING_R], [K.RING_S1, K.RING_IN], [K.RING_S0, K.RING_IN]], tone=0.36, **DP)
    V.revl("drum-mouth-rim", "drum", O, U, [[5.33, 0.565], [5.33, K.LIP_R], [5.40, K.LIP_R], [5.40, 0.553], [5.33, 0.565]], tone=0.34, **DP)
    for k, s in enumerate([1.10, 1.90]):
        V.revl(f"drum-seam-{k + 1}", "drum", O, U, [[s - 0.008, 1.148], [s - 0.008, 1.154], [s + 0.008, 1.154], [s + 0.008, 1.148], [s - 0.008, 1.148]], texture=True, tone=0.3, material="drum-paint", role="detail", region="drum")
    V.boxl("drum-hatch", "drum", [1.50, 0, -1.142], [0.46, 0.36, 0.04], cr=0.016, tone=0.36, material="drum-paint", role="detail", region="drum")
    V.revl("drum-flange", "drum", O, U, [[-0.05, 0], [-0.05, 0.36], [0.02, 0.36], [0.02, 0]], tone=0.33, **PSD)
    blade(V, "drum-blade-1", 0.0, material="drum-paint", role="detail", region="drum")
    blade(V, "drum-blade-2", pi, material="drum-paint", role="detail", region="drum")
    V.revl("drive-gearbox", "drum", O, U, [[-0.46, 0], [-0.46, 0.25], [-0.42, 0.33], [-0.12, 0.33], [-0.10, 0.39], [-0.05, 0.39], [-0.05, 0]], tone=0.35, region="drum-drive", **PS)
    V.revl("drive-motor", "drum", [0, -0.04, 0], U, [[-0.72, 0], [-0.72, 0.085], [-0.52, 0.105], [-0.46, 0.105], [-0.46, 0]], tone=0.3, region="drum-drive", **PSD)
    V.box("oil-cooler", "chassis", [1.98, 1.52, 0.56], [0.46, 0.44, 0.16], cr=0.02, tone=0.3, region="drum-drive", **PS)


def pedestals(V):
    for n, sg in SIDES:
        z0, z1 = sorted([sg * 0.40, sg * 0.45])
        V.ext(f"front-pedestal-{n}", "chassis", "side", [(1.50, 1.16), (2.48, 1.16), (2.40, 1.62), (2.05, 1.70), (1.74, 1.78)], z0, z1, holes=[[(1.80, 1.28), (2.24, 1.28), (2.18, 1.52), (1.90, 1.58)]], tone=0.48, region="chassis", **PS)
        z0, z1 = sorted([sg * 0.62, sg * 0.70])
        V.ext(f"rear-pedestal-{n}", "chassis", "side", [(-1.70, 1.16), (-3.20, 1.16), (-2.82, 1.98), (-2.30, 1.98)], z0, z1, holes=[[(-2.05, 1.30), (-2.95, 1.30), (-2.72, 1.80), (-2.38, 1.80)]], tone=0.5, region="chassis", **PS)
        V.box(f"rear-pedestal-foot-{n}", "chassis", [-2.45, 1.19, sg * 0.50], [1.56, 0.06, 0.42], tone=0.46, region="chassis", **PS)
        rc = K.dw(4.26, K.ROLLER * cos(rad(38)), sg * K.ROLLER * sin(rad(38)))
        a = K.dw(4.19, K.ROLLER * cos(rad(38)), sg * K.ROLLER * sin(rad(38)))
        b2 = K.dw(4.33, K.ROLLER * cos(rad(38)), sg * K.ROLLER * sin(rad(38)))
        V.cyl(f"support-roller-{n}", "chassis", a, b2, 0.11, tone=0.3, region="chassis", **PSD)
        V.box(f"roller-bracket-{n}", "chassis", [rc[0] - 0.02, 2.0, sg * 0.60], [0.34, 0.10, 0.30], tone=0.4, region="chassis", **PSD)
        z0, z1 = sorted([sg * 0.50, sg * 0.58])
        V.ext(f"rear-frame-{n}", "chassis", "side", [(-3.40, 1.16), (-3.62, 1.16), (-3.62, 2.20), (-4.05, 2.95), (-3.90, 2.95), (-3.55, 2.45), (-3.40, 2.45)], z0, z1, tone=0.5, region="chassis", **PS)
        V.cap(f"hopper-stay-{n}", "chassis", [-3.98, 2.95, sg * 0.54], [-3.98, 3.40, sg * 0.50], 0.025, tone=0.4, region="chassis", **PSD)
    V.box("front-pedestal-top", "chassis", [2.07, 1.635, 0], [0.56, 0.05, 0.94], rz=-11, tone=0.46, region="chassis", **PS)
    V.box("front-pedestal-brace", "chassis", [1.99, 1.19, 0], [0.98, 0.06, 0.94], tone=0.46, region="chassis", **PS)
    V.box("rear-pedestal-beam", "chassis", [-2.56, 1.88, 0], [0.30, 0.16, 1.40], cr=0.02, tone=0.48, region="chassis", **PS)
    for n, sg in SIDES:
        V.cap(f"swivel-stay-{n}", "chassis", [-3.80, 2.55, sg * 0.50], [-3.95, 2.03, sg * 0.22], 0.022, tone=0.4, region="chassis", **PSD)
    V.box("rear-frame-top-beam", "chassis", [-3.975, 2.92, 0], [0.10, 0.08, 1.08], tone=0.48, region="chassis", **PS)
    V.cyl("water-tank", "chassis", [2.25, 2.78, -0.72], [2.25, 2.78, 0.72], 0.25, tone=0.62, region="chassis", material="galvanised-steel", role="body")
    for n, sg in SIDES:
        V.rev(f"water-tank-end-{n}", "chassis", [2.25, 2.78, sg * 0.72], [0, 0, sg], [[0, 0.25], [0.04, 0.22], [0.07, 0.14], [0.08, 0]], tone=0.62, region="chassis", material="galvanised-steel", role="body")
        V.box(f"water-tank-post-{n}", "chassis", [2.25, 2.08, sg * 0.425], [0.08, 0.92, 0.05], tone=0.44, region="chassis", **PSD)
        V.box(f"water-tank-saddle-{n}", "chassis", [2.25, 2.52, sg * 0.425], [0.36, 0.05, 0.07], tone=0.44, region="chassis", **PSD)


def hopper(V):
    secs = [([-3.86, 3.65, 0], 0.37, 0.50, 4), ([-3.84, 3.42, 0], 0.33, 0.44, 4), ([-3.74, 3.22, 0], 0.27, 0.36, 3)]
    V.loft("hopper-bowl", "chassis", secs, hint=(1, 0, 0), axis=(0, 1, 0), caps=(False, False), tone=0.55, region="hopper", **GS)
    V.revl("hopper-spout", "drum", [0, -0.08, 0], [1, 0, 0], [[5.22, 0.285], [5.22, 0.29], [5.70, 0.31], [5.70, 0.305], [5.22, 0.285]], tone=0.55, region="hopper", **GS)
    fn = [(K.dw(5.44, 0.50, 0), 0.20, 0.40, 3), ([-3.80, 2.36, 0], 0.22, 0.32, 2.5), ([-3.93, 2.10, 0], 0.19, 0.19, 2)]
    V.loft("discharge-funnel", "chassis", fn, hint=(1, 0, 0), axis=(0, 1, 0), caps=(False, False), tone=0.5, region="chute", **GS)


def chute(V):
    V.revl("chute-swivel-ring", "chute-swivel", [0, 0, 0], [1, 0, 0], [[0.02, 0.17], [0.02, 0.24], [0.08, 0.24], [0.08, 0.17], [0.02, 0.17]], tone=0.4, region="chute", **GS)
    U = lambda a, d, a2, d2: [(d * sin(rad(t)), a * cos(rad(t))) for t in range(0, 181, 12)] + [(d2 * sin(rad(t)), a2 * cos(rad(t))) for t in range(180, -1, -12)]
    V.extl("chute-trough", "chute", "yz", U(K.CHUTE_HW, K.CHUTE_D, K.CHUTE_HW - 0.005, K.CHUTE_D - 0.005), 0.0, K.CHUTE_L, tone=0.5, region="chute", **GS)
    V.extl("chute-ext-trough", "chute-ext", "yz", U(K.EXT_HW, K.EXT_D, K.EXT_HW - 0.005, K.EXT_D - 0.005), 0.0, K.EXT_L, tone=0.6, region="chute", **GS)
    V.boxl("chute-yoke", "chute", [0.12, 0.215, 0], [0.22, 0.03, 0.30], tone=0.4, region="chute", material="galvanised-steel", role="detail")
    for k, w in enumerate([-0.27, 0.27]):
        V.boxl(f"chute-hinge-{k + 1}", "chute", [K.CHUTE_L - 0.02, -0.01, w], [0.07, 0.05, 0.03], tone=0.35, region="chute", material="galvanised-steel", role="detail")


def ladder(V):
    z0, z1 = 0.80, 1.22
    for k, z in enumerate([z0, z1]):
        V.cap(f"ladder-stile-{k + 1}", "chassis", [-4.20, 0.78, z], [-4.44, 2.50, z], 0.018, slender=True, tone=0.4, region="chassis", **PSD)
        V.cap(f"guard-post-{k + 1}", "chassis", [-4.44, 2.50, z], [-4.46, 3.50, z], 0.016, slender=True, tone=0.4, region="chassis", **PSD)
    for k in range(6):
        t = (k + 0.5) / 6
        V.cap(f"ladder-rung-{k + 1}", "chassis", [-4.20 - 0.24 * t, 0.78 + 1.72 * t, z0], [-4.20 - 0.24 * t, 0.78 + 1.72 * t, z1], 0.014, tone=0.4, region="chassis", **PSD)
    V.box("platform", "chassis", [-4.20, 2.50, 1.01], [0.52, 0.03, 0.46], tone=0.42, region="chassis", **PSD)
    V.cap("guard-rail-top", "chassis", [-4.46, 3.50, z0], [-4.46, 3.50, z1], 0.016, tone=0.4, region="chassis", **PSD)
    V.cap("guard-rail-mid", "chassis", [-4.45, 3.00, z0], [-4.45, 3.00, z1], 0.014, tone=0.4, region="chassis", **PSD)
    V.cap("platform-stay", "chassis", [-3.80, 2.50, 0.58], [-3.95, 2.50, 0.80], 0.02, tone=0.4, region="chassis", **PSD)


def body(V):
    frame(V)
    cab(V)
    equipment(V)
    running_gear(V)
    wheels(V)
    pedestals(V)
    drum(V)
    hopper(V)
    chute(V)
    ladder(V)


def thermal(V):
    V.reg = {sid: [(K.region_of(sid, r), lay) for r, lay in regs] for sid, regs in V.reg.items()}
    V.auto_thermal(skip=("drum-shell", "drum-blade-1", "drum-blade-2", "windscreen", "door-glass-l", "door-glass-r"))
    V.to("thermal")
    V.lists["thermal"].append({"id": "th-windscreen", "of": "windscreen", "region": "cab-glazing", "sectionFaces": ["-u"]})
    V.lists["thermal"].append({"id": "th-door-glass-l", "of": "door-glass-l", "region": "cab-glazing", "sectionFaces": ["-w"]})
    V.lists["thermal"].append({"id": "th-door-glass-r", "of": "door-glass-r", "region": "cab-glazing", "sectionFaces": ["+w"]})
    V.revl("th-drum-skin", "drum", [0, 0, 0], [1, 0, 0], [[0, 0]] + [list(p) for p in K.DRUM] + [[5.40, 0]], tone=0.3, material="drum-paint", role="body", region="drum")
    sx, sz = K.STACK
    V.cyl("th-exhaust-stack-jet", "chassis", [sx - 0.035, 3.30, sz], [sx - 0.0385, 3.308, sz], 0.058, tone=0.2, region="exhaust-stack", tags=["exhaust-stack-jet"], material="exhaust-steel", role="detail")
    V.box("th-radiator-air", "chassis", [2.985, 0.80, 0], [0.008, 0.52, 0.60], tone=0.2, region="engine-bay", tags=["radiator-air"], material="paint-steel", role="detail")
    V.to("body")


ST = dict(material="paint-steel", role="body", tone=0.4)


def structure(V):
    V.to("structure")
    C = "chassis"
    for sid in ["frame-rail-l", "frame-rail-r", "subframe-rail-l", "subframe-rail-r"] + [f"frame-cross-{k}" for k in range(1, 8)] + [f"subframe-cross-{k}" for k in range(1, 5)]:
        V.member(sid, C)
    for sid in ["front-pedestal-l", "front-pedestal-r", "front-pedestal-top", "front-pedestal-brace", "rear-pedestal-l", "rear-pedestal-r", "rear-pedestal-foot-l", "rear-pedestal-foot-r", "rear-pedestal-beam", "roller-bracket-l", "roller-bracket-r", "rear-frame-l", "rear-frame-r", "rear-frame-top-beam", "swivel-stay-l", "swivel-stay-r", "hopper-stay-l", "hopper-stay-r"]:
        V.member(sid, C)
    for sid in ["bogie-spring-l", "bogie-spring-r", "bogie-hanger-l", "bogie-hanger-r", "bogie-trunnion", "bogie-saddle"] + [f"spring-{i}-{n}" for i in (1, 2) for n in "lr"] + [f"spring-hanger-{i}-{n}-{k}" for i in (1, 2) for n in "lr" for k in (1, 2)] + [f"torque-rod-{n}-{k}" for n in "lr" for k in (1, 2)]:
        V.member(sid, C)
    for sid in ["drum-shell", "drum-track-ring", "drum-mouth-rim", "drum-flange"]:
        V.member(sid, "drum")
    V.member("drum-blade-1", "drum-spiral")
    V.member("drum-blade-2", "drum-spiral")
    V.member("chute-swivel-ring", "chute-swivel")
    V.member("chute-trough", "chute")
    V.member("chute-yoke", "chute")
    V.member("chute-ext-trough", "chute-ext")
    for i in range(4):
        b = f"wheel-{i + 1}"
        V.member(f"axle-beam-{i + 1}" if i < 2 else f"axle-housing-{i + 1}", b)
        for n, sg in SIDES:
            kinds = [("", K.ZF)] if i < 2 else [("-outer", K.ZO), ("-inner", K.ZI)]
            for kind, zc in kinds:
                V.member(f"rim-{i + 1}-{n}{kind}", b)
                V.member(f"wheel-disc-{i + 1}-{n}{kind}", b)
            zs = K.ZF if i < 2 else (K.ZI + K.ZO) / 2
            V.cyl(f"joint-wheel-{i + 1}-{n}", b, [K.AX[i], K.WR, sg * (zs - 0.16)], [K.AX[i], K.WR, sg * (zs + 0.16)], 0.07, member=b, part="joint", material="cast-iron", role="detail", tone=0.3)
            V.box(f"support-contact-{i + 1}-{n}", b, [K.AX[i], 0.00375, sg * zs], [0.30, 0.0075, 0.30 if i < 2 else 0.66], member=b, part="support", anchor=f"contact-{i + 1}-{n}", material="rubber", role="body", tone=0.12)
    V.revl("joint-drum-bearing", "drum", [0, 0, 0], [1, 0, 0], [[-0.05, 0], [-0.05, 0.16], [0.03, 0.16], [0.03, 0]], member="drum", part="joint", material="cast-iron", role="detail", tone=0.3)
    V.member("support-roller-l", "drum", part="joint", new_id="joint-drum-roller-l")
    V.member("support-roller-r", "drum", part="joint", new_id="joint-drum-roller-r")
    V.revl("joint-chute-swivel", "chute-swivel", [0, 0, 0], [1, 0, 0], [[0.02, 0], [0.02, 0.10], [0.08, 0.10], [0.08, 0]], member="chute-swivel", part="joint", material="cast-iron", role="detail", tone=0.3)
    V.revl("joint-chute", "chute", [0.03, 0.2, 0], [0, 0, 1], [[-0.17, 0], [-0.17, 0.025], [0.17, 0.025], [0.17, 0]], member="chute", part="joint", material="cast-iron", role="detail", tone=0.3)
    V.revl("joint-chute-ext", "chute-ext", [0, 0, 0], [0, 0, 1], [[-0.27, 0], [-0.27, 0.02], [0.27, 0.02], [0.27, 0]], member="chute-ext", part="joint", material="cast-iron", role="detail", tone=0.3)
    V.ell("mass-chassis", C, [0.9, 1.0, 0], [0.42, 0.13, 0.30], member=C, part="mass", material="paint-steel", role="body", tone=0.2)
    m = K.dw(2.6)
    V.ell("mass-drum", "drum", m, [0.34, 0.34, 0.34], member="drum", part="mass", material="drum-paint", role="body", tone=0.2)
    V.to("body")


def build(V):
    body(V)
    thermal(V)
    structure(V)
    V.decide(
        "At 1:50 the truck is built assembly by assembly: two channel frame rails with seven cross members and the mixer subframe, a cab-over cab lofted from seven sections with wheel houses cut round the first axle, steps, doors, glazing, mirrors, lamps and grille, engine with sump and radiator, gearbox, propeller shafts, leaf springs with hangers, the bogie with trunnion, inverted springs and torque rods, drive axles with differentials, eight single and twin wheels as tyre, rim, perforated disc, hub and nuts, fenders, tanks, silencer with shielded stack, air intake, the drum as a hollow 5 mm revolve with track ring, mouth rim, hatch and two spiral blades inside, hydraulic drive, pedestals with support rollers, water tank, charge hopper, discharge funnel, swivel and folding chute, two spare chutes, ladder with platform and guard rail.",
        "Bei 1:50 ist der Lkw Baugruppe für Baugruppe gebaut: zwei U-Rahmenlängsträger mit sieben Querträgern und dem Mischer-Hilfsrahmen, ein Frontlenker-Fahrerhaus als Loft aus sieben Schnitten mit um die erste Achse ausgeschnittenen Radhäusern, Tritten, Türen, Verglasung, Spiegeln, Leuchten und Kühlergrill, Motor mit Ölwanne und Kühler, Getriebe, Gelenkwellen, Blattfedern mit Böcken, das Doppelachsaggregat mit Pendellager, hängenden Federn und Lenkern, Antriebsachsen mit Differentialen, acht Einzel- und Zwillingsräder aus Reifen, Felge, gelochter Schüssel, Nabe und Muttern, Kotflügel, Tanks, Schalldämpfer mit verkleidetem Auspuffrohr, Luftansaugung, die Trommel als hohler 5-mm-Drehkörper mit Laufring, Mündungsring, Mannloch und zwei Wendeln innen, Hydraulikantrieb, Lagerböcke mit Laufrollen, Wassertank, Einfülltrichter, Auslauftrichter, Schwenk- und Klappschurre, zwei Ersatzschurren, Leiter mit Podest und Geländer.",
    )
    V.decide(
        "Thermal at 1:50 references the body surfaces region by region; the drum region is the closed outer skin (the inside faces the charge), the glass panes count their outer face only (the inner face lies on the cab), the exhaust jet is the mouth of the stack cap and the radiator air leaves through a patch behind the engine under the cab.",
        "Thermisch verweist 1:50 Region für Region auf die Körperflächen; die Trommelregion ist die geschlossene Außenhaut (innen liegt die Füllung an), die Scheiben zählen nur mit der Außenfläche (die Innenfläche liegt am Fahrerhaus), der Abgasstrahl ist die Mündung der Auspuffkappe, die Kühlerabluft tritt durch ein Feld hinter dem Motor unter dem Fahrerhaus aus.",
    )
    V.decide(
        "Structure at 1:50 is the real load path: frame rails, cross members and subframe, the two pedestals with roller brackets and the rear frame, leaf springs with hangers, bogie springs, trunnion and torque rods, axle beams and housings, rims and wheel discs, the drum shell with track ring and flange on the drive bearing and two rollers, the blades, swivel ring and chute troughs; joints at the wheel bearings, drum bearing, rollers and chute hinges, a contact pad per tyre set, masses at the centres of chassis and drum.",
        "Das Tragwerk ist bei 1:50 der wirkliche Lastpfad: Rahmenlängsträger, Querträger und Hilfsrahmen, die beiden Lagerböcke mit Rollenkonsolen und der Heckrahmen, Blattfedern mit Böcken, Aggregatfedern, Pendellager und Lenker, Achskörper und Achsgehäuse, Felgen und Radschüsseln, die Trommelschale mit Laufring und Flansch auf Antriebslager und zwei Rollen, die Wendeln, Drehkranz und Schurrentröge; Gelenke an Radlagern, Trommellager, Rollen und Schurrengelenken, je Reifensatz eine Aufstandsfläche, Massen in den Schwerpunkten von Fahrgestell und Trommel.",
    )
