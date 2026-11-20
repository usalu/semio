"""🔥 Round 3 gas boiler: states every solid of every authored scale (1:50, 1:100, 1:200; 1:500 is unavailable) and list (body, thermal, structure) by hand and writes `volumes/appliances/gas-boiler.json`.

The drawn detail strokes on the front (text lines, ticks, needle, notch) are read from the 1:50 elevation of the candidate document so that the texture solids lie exactly on the redrawn strokes.

Usage: python3 r3-gas-boiler-model.py
"""
import importlib.util
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "r3-gas-boiler-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)
SOURCE = os.path.join(HERE, "volumes", "appliances", "gas-boiler.json")
TEXTS = os.path.join(HERE, "r3-gas-boiler-texts.json")
CANDIDATE = os.path.join(HERE, "🗑️generated", "r3-gas-boiler", "🔣️.json")
CATALOG = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/gas-boiler/🔣️.json"))

PS, BR, PC, GL, BM, FP, GS, SS, AL = "painted-steel", "brass", "polycarbonate", "sight-glass", "burner-mesh", "flue-pipe", "galvanised-steel", "stainless-steel", "die-cast-aluminium"
WX, WY = K.WIN
ZAX = (0, 0, 1)
YAX = (0, 1, 0)
XAX = (1, 0, 0)


def strokes():
    """✒️ The open detail strokes of the 1:50 elevation by id (world frame of the casing, bone frame for the knob)."""
    path = CANDIDATE if os.path.exists(CANDIDATE) else CATALOG
    asset = json.load(open(path, encoding="utf-8"))["asset"]
    return {s["id"]: s for s in asset["representations"]["50"]["geometry"]["elevation"]}


def hood_outline(r=K.RC, inset=0.0, n=6):
    """▢ The elevation outline of the hood: 0.44 x 0.72 m with the 14 mm rounded side edges."""
    return K.densify(K.rrect(-K.XW + inset, K.Y0 + inset, K.XW - inset, K.Y1 - inset, max(r - inset, 0), n), 0.04)


def stroke(sid, bone, pts2, z, r=0.0007, tone=0.2, material=PC):
    """✏️ A drawn detail stroke as a hair-thin texture capsule lying on a face at depth z."""
    (x0, y0), (x1, y1) = pts2[0], pts2[-1]
    return K.cap(sid, bone, material, "detail", tone, (x0, y0, z), (x1, y1, z), r, texture=True)


def flames(scale):
    """🔥 The two flame crowns over the burner bar: three tongues each, lofted with real depth along the burner (the outer flame 32 mm deep, the inner one 24 mm and 2 mm nearer the glass so it shows in front)."""
    out = []
    z_out, z_in = K.BURNER_Z, K.BURNER_Z - 0.006
    if scale == 50:
        specs = [
            ("flame-tall", "flame-tall", 0.75, z_out, 0.016, [(0.0, 0.012, 0.032), (-0.015, 0.009, 0.022), (0.015, 0.009, 0.022)]),
            ("flame", "flame", 0.95, z_in, 0.012, [(0.0, 0.0085, 0.017), (-0.012, 0.0065, 0.014), (0.012, 0.0065, 0.014)]),
        ]
    else:
        specs = [
            ("flame-tall", "flame-tall", 0.75, z_out, 0.016, [(0.0, 0.0264, 0.0383)]),
            ("flame", "flame", 0.95, z_in, 0.012, [(0.0, 0.018, 0.017)]),
        ]
    for sid, bone, tone, zc, depth, tongues in specs:
        for i, (cx, half, h) in enumerate(tongues):
            name = sid if i == 0 else f"{sid}-{'l' if cx < 0 else 'r'}"
            secs = [((cx, 0.0, zc), half, depth, 2), ((cx, 0.35 * h, zc), half * 0.86, depth * 0.95, 2), ((cx, 0.7 * h, zc), half * 0.5, depth * 0.75, 2), ((cx, h, zc), 0.0008, depth * 0.3, 2)]
            if scale != 50:
                secs = [((cx, 0.0, zc), half, depth, 2.4), ((cx, 0.3 * h, zc), half * 0.8, depth * 0.95, 2), ((cx, 0.65 * h, zc), half * 0.42, depth * 0.75, 2), ((cx, h, zc), 0.002, depth * 0.4, 2)]
            out.append(K.loft(name, bone, BM, "detail", tone, secs))
    return out


def body50():
    """🎨 1:50 body: the pressed steel hood on its galvanised chassis, the porthole with bezel, glass, combustion chamber, burner and flames, the moulded control fascia with display, mode knob, set knob and gauge, the concentric 80/125 flue adapter and five brass stubs with union nuts; seams, slits, logo and LED as texture."""
    sk = strokes()
    out = []
    out.append(K.ext("casing", "casing", PS, "body", 0.88, "xy", hood_outline(), K.ZF, K.ZSEAM, holes=[K.circle(WX, WY, K.WIN_R, 40)]))
    out.append(K.ext("chassis", "casing", GS, "body", 0.72, "xy", hood_outline(inset=0.0015), K.ZSEAM, K.ZW))
    out.append(K.rev("bezel", "casing", GL, "body", 0.55, K.loop([(-0.151, 0.0355), (-0.151, 0.0365), (-0.155, 0.0365), (-0.155, 0.0445), (-0.1605, 0.0445), (-0.163, 0.042), (-0.163, 0.0378), (-0.1612, 0.0355)]), (WX, WY, 0), ZAX))
    out.append(K.rev("sight-glass", "casing", GL, "glazing", 0.3, [(-0.151, 0.0), (-0.151, 0.0355), (-0.147, 0.0355), (-0.147, 0.0)], (WX, WY, 0), ZAX))
    out.append(K.rev("chamber", "casing", SS, "body", 0.1, [(-0.027, 0.0), (-0.027, 0.0365), (-0.147, 0.0365), (-0.147, 0.0355), (-0.035, 0.0355), (-0.035, 0.0)], (WX, WY, 0), ZAX))
    out.append(K.box("burner-tube", "casing", BM, "body", 0.45, (0.052, 0.011, 0.046), (0.0, 0.5355, K.BURNER_Z), radius=0.002))
    out += flames(50)
    out.append(K.ext("panel", "casing", PC, "body", 0.35, "xy", K.rrect(-0.205, 0.065, 0.205, 0.21, 0.008), -0.165, K.ZF + 0.002))
    out.append(K.box("display", "casing", PC, "detail", 0.95, (0.11, 0.06, 0.0006), (-0.115, 0.155, -0.1653), radius=None, texture=True))
    out.append(stroke("text-1", "casing", sk["text-1"]["points"], -0.1658, tone=0.3))
    out.append(stroke("text-2", "casing", sk["text-2"]["points"], -0.1658, tone=0.3))
    for t in ("tick-off", "tick-heat", "tick-water"):
        out.append(stroke(t, "casing", sk[t]["points"], -0.1652, tone=0.4))
    out.append(K.rev("knob", "mode-knob", PC, "body", 0.72, [(-0.163, 0.0), (-0.163, 0.006), (-0.165, 0.006), (-0.165, 0.026), (-0.1715, 0.026), (-0.1735, 0.0245), (-0.175, 0.0225), (-0.175, 0.0)], (0, 0, 0), ZAX))
    out.append(K.cap("pointer", "mode-knob", PC, "detail", 0.2, (0.0, 0.003, -0.1752), (0.0, 0.0205, -0.1752), 0.0012, texture=True))
    out.append(K.rev("set-knob", "casing", PC, "body", 0.72, [(-0.165, 0.0), (-0.165, 0.018), (-0.1685, 0.018), (-0.170, 0.0165), (-0.170, 0.0)], (0.12, 0.14, 0), ZAX))
    out.append(stroke("set-notch", "casing", sk["set-notch"]["points"], -0.1702, r=0.001))
    out.append(K.rev("gauge", "casing", GL, "glazing", 0.92, [(-0.165, 0.0), (-0.165, 0.016), (-0.168, 0.016), (-0.170, 0.013), (-0.170, 0.0)], (0.173, 0.14, 0), ZAX))
    out.append(stroke("gauge-needle", "casing", sk["gauge-needle"]["points"], -0.1702, r=0.0007))
    door = sk["door"]["points"]
    xs, ys = [p[0] for p in door], [p[1] for p in door]
    out.append(K.ext("door", "casing", PS, "detail", 0.4, "xy", K.rrect(min(xs) - 0.00075, min(ys) - 0.00075, max(xs) + 0.00075, max(ys) + 0.00075, 0.01275), K.ZF - 0.0005, K.ZF + 0.001, holes=[K.rrect(min(xs) + 0.00075, min(ys) + 0.00075, max(xs) - 0.00075, max(ys) - 0.00075, 0.01125)], texture=True))
    s0 = sk["slit-0"]["points"]
    out.append(K.box("slits", "casing", PS, "detail", 0.25, (0.004, abs(s0[1][1] - s0[0][1]), 0.001), (s0[0][0], 0.5 * (s0[0][1] + s0[1][1]), K.ZF), repeat={"count": 4, "translation": [0.06, 0, 0]}, texture=True))
    for lid in ("logo-1", "logo-2"):
        p = sk[lid]["points"]
        out.append(K.box(lid, "casing", PS, "detail", 0.4, (abs(p[1][0] - p[0][0]), 0.002, 0.001), (0.5 * (p[0][0] + p[1][0]), p[0][1], K.ZF), texture=True))
    led = sk["led"]
    out.append(K.rev("led", "casing", PC, "detail", 0.95, [(K.ZF - 0.0008, 0.0), (K.ZF - 0.0008, led["rx"]), (K.ZF + 0.001, led["rx"]), (K.ZF + 0.001, 0.0)], (led["cx"], led["cy"], 0), ZAX, texture=True))
    out.append(K.rev("collar-base", "collar", FP, "body", 0.7, K.loop([(-0.003, 0.0405), (-0.003, 0.0625), (0.026, 0.0625), (0.03, 0.0585), (0.03, 0.055), (0.027, 0.055), (0.027, 0.0405)]), (0, 0, K.COLLAR_Z), YAX))
    out.append(K.rev("collar-pipe", "collar", FP, "body", 0.55, K.loop([(0.0, 0.0400), (0.0475, 0.0400), (0.05, 0.0378), (0.05, 0.032), (0.046, 0.032), (0.046, 0.037), (0.0, 0.037)]), (0, 0, K.COLLAR_Z), YAX))
    out.append(K.rev("flue-bore", "collar", FP, "body", 0.12, [(0.0005, 0.0), (0.0005, 0.0372), (0.0025, 0.0372), (0.0025, 0.0)], (0, 0, K.COLLAR_Z), YAX, texture=True))
    out.append(K.rev("stubs", "connections", BR, "body", 0.7, K.loop([(0.0, 0.0095), (0.0, 0.0105), (0.0015, 0.011), (0.056, 0.011), (0.056, 0.0095)]), (K.STUB_X[0], 0, K.STUB_Z), YAX, repeat={"count": 5, "translation": [0.08, 0, 0]}))
    out.append(K.ext("nuts", "connections", BR, "body", 0.45, "zx", K.hexagon(K.STUB_Z, K.STUB_X[0], 0.0179), 0.018, 0.038, repeat={"count": 5, "translation": [0.08, 0, 0]}))
    return out


def hx_profile(rin, rout, z0, z1, door, back, hole):
    """♨️ The heat exchanger housing about the window axis as (z, r): burner door in front with the sight hole, the coil pack as a ring, the back plate behind; the combustion chamber is the cavity inside."""
    return [(z1 + back, 0.0), (z1 + back, rout), (z0, rout), (z0, hole), (z0 + door, hole), (z0 + door, rin), (z1, rin), (z1, 0.0)]


def thermal50():
    """🌡️ 1:50 thermal: casing sheet (closed hood, chassis, fascia frame, bezel), controls (display plate, knobs, gauge), sight glass (front face only), burner and flames, flue collar with the outlet patch, connections, and the hidden heat exchanger with combustion chamber and flue collector, premix fan, expansion vessel and hydraulic group."""
    out = []
    out.append(K.th(K.ext("th-casing", "casing", PS, "body", 0.88, "xy", hood_outline(), K.ZF, K.ZSEAM), "casing", "sheet"))
    out.append(K.ref("th-chassis", "chassis", region="casing", layer="sheet"))
    out.append(K.ref("th-panel", "panel", region="casing", layer="sheet"))
    out.append(K.ref("th-bezel", "bezel", region="casing", layer="sheet"))
    out.append(K.th(K.box("th-display", "casing", PC, "body", 0.3, (0.11, 0.06, 0.002), (-0.115, 0.155, -0.166)), "controls", "panel"))
    out.append(K.ref("th-knob", "knob", region="controls", layer="panel"))
    out.append(K.ref("th-set-knob", "set-knob", region="controls", layer="panel"))
    out.append(K.ref("th-gauge", "gauge", region="controls", layer="panel"))
    out.append(K.th(K.rev("th-sight-glass", "casing", GL, "glazing", 0.85, [(-0.151, 0.0), (-0.151, 0.0365), (-0.147, 0.0365), (-0.147, 0.0)], (WX, WY, 0), ZAX, sectionFaces=["+w", "-u", "+u", "-v", "+v"]), "sight-glass", "glass"))
    out.append(K.ref("th-burner-tube", "burner-tube", region="burner", layer="mesh"))
    for f in ("flame-tall", "flame-tall-l", "flame-tall-r", "flame", "flame-l", "flame-r"):
        out.append(K.ref(f"th-{f}", f, region="burner", layer="mesh"))
    out.append(K.ref("th-collar-base", "collar-base", region="flue-collar", layer="pipe"))
    out.append(K.ref("th-collar-pipe", "collar-pipe", region="flue-collar", layer="pipe"))
    out.append(K.th(K.rev("th-flue-gas", "collar", FP, "detail", 0.5, [(0.044, 0.0), (0.044, 0.03), (0.0515, 0.03), (0.0515, 0.0)], (0, 0, K.COLLAR_Z), YAX), "flue-collar", None, tags=["flue-gas"]))
    out.append(K.ref("th-stubs", "stubs", region="connections", layer="tube"))
    out.append(K.ref("th-nuts", "nuts", region="connections", layer="tube"))
    out += hidden50()
    return out


def hidden50():
    """🫥 The hidden internals at 1:50, each a real assembly inside the casing."""
    out = []
    out.append(K.th(K.rev("th-hx", "casing", SS, "body", 0.5, hx_profile(0.065, 0.115, -0.147, 0.033, 0.01, 0.01, 0.0355), (WX, WY, 0), ZAX), "heat-exchanger", "wall"))
    out.append(K.th(K.rev("th-hx-collector", "casing", SS, "body", 0.5, [(0.67, 0.0), (0.67, 0.04), (0.77, 0.04), (0.77, 0.0)], (0, 0, K.COLLAR_Z), YAX), "heat-exchanger", "wall"))
    out.append(K.th(K.rev("th-fan", "casing", AL, "body", 0.6, [(0.0, 0.0), (0.0, 0.045), (0.07, 0.045), (0.07, 0.0)], (-0.205, 0.60, -0.05), XAX), "fan", "housing"))
    out.append(K.th(K.cap("th-fan-duct", "casing", AL, "body", 0.6, (-0.15, 0.585, -0.09), (-0.095, 0.545, -0.142), 0.012), "fan", "housing"))
    lens = [(0.065, 0.0), (0.068, 0.06), (0.075, 0.10), (0.085, 0.13), (0.0935, 0.148), (0.1035, 0.15), (0.1035, 0.158), (0.1065, 0.158), (0.1065, 0.15), (0.1165, 0.148), (0.125, 0.13), (0.135, 0.10), (0.142, 0.06), (0.145, 0.0)]
    out.append(K.th(K.rev("th-vessel", "casing", PS, "body", 0.35, lens, (0.0, 0.40, 0), ZAX), "expansion-vessel", "shell"))
    out.append(K.th(K.rev("th-pump", "casing", BR, "body", 0.4, [(-0.09, 0.0), (-0.09, 0.036), (-0.085, 0.04), (-0.02, 0.04), (-0.02, 0.05), (0.03, 0.05), (0.03, 0.0)], (-0.12, 0.17, 0), ZAX), "hydraulics", "body"))
    out.append(K.th(K.box("th-gas-valve", "casing", BR, "body", 0.4, (0.06, 0.07, 0.05), (-0.025, 0.15, 0.03), radius=0.004), "hydraulics", "body"))
    out.append(K.th(K.box("th-plate-hx", "casing", SS, "body", 0.4, (0.07, 0.13, 0.045), (0.10, 0.165, 0.02), radius=0.003), "hydraulics", "body"))
    out.append(K.th(K.box("th-manifold", "casing", BR, "body", 0.4, (0.38, 0.03, 0.05), (0.0, 0.075, 0.07), radius=0.003), "hydraulics", "body"))
    out.append(K.th(K.rev("th-siphon", "casing", PC, "body", 0.4, [(0.07, 0.0), (0.07, 0.022), (0.17, 0.022), (0.17, 0.0)], (0.175, 0, 0.05), YAX), "hydraulics", "body"))
    out.append(K.th(K.cap("th-flow-pipe", "casing", BR, "body", 0.4, (-0.06, 0.455, 0.0), (-0.06, 0.09, 0.0), 0.0075), "hydraulics", "body"))
    out.append(K.th(K.cap("th-return-pipe", "casing", BR, "body", 0.4, (0.045, 0.455, 0.0), (0.045, 0.19, 0.0), 0.0075), "hydraulics", "body"))
    return out


def angle_yz(y0, z0, leg_y, leg_z, t):
    """∟ An angle section in the yz plane: a horizontal leg from z0 by leg_z and a vertical leg from y0 by leg_y, wall t."""
    sy = 1 if leg_y > 0 else -1
    sz = 1 if leg_z > 0 else -1
    return [(y0, z0), (y0 + leg_y, z0), (y0 + leg_y, z0 + sz * t), (y0 + sy * t, z0 + sz * t), (y0 + sy * t, z0 + leg_z), (y0, z0 + leg_z)]


def structure50():
    """🏗️ 1:50 structure: the galvanised chassis (back sheet with folded side, top and bottom edges), two pairs of angle brackets under and over the heat exchanger, the flue collector into the collar, the manifold on its angle, the stub tubes, joints at the collar flange, the bulkheads, the knob shaft and the flame slides, the wall rail as support and the casing mass at its centre of mass."""
    out = []
    t = 0.0015
    xo = K.XW - 0.0015
    u = [(K.ZSEAM, -xo), (K.ZW, -xo), (K.ZW, xo), (K.ZSEAM, xo), (K.ZSEAM, xo - t), (K.ZW - t, xo - t), (K.ZW - t, -xo + t), (K.ZSEAM, -xo + t)]
    out.append(K.st(K.ext("member-chassis", "casing", GS, "body", 0.7, "zx", u, K.Y0 + 0.0015, K.Y1 - 0.0015), "casing", "member"))
    out.append(K.st(K.ext("member-chassis-top", "casing", GS, "body", 0.7, "yz", angle_yz(K.Y1 - 0.0015, K.ZW - t, -0.02, -0.02, t), -xo + t, xo - t), "casing", "member"))
    out.append(K.st(K.ext("member-chassis-bottom", "casing", GS, "body", 0.7, "yz", angle_yz(K.Y0 + 0.0015, K.ZW - t, 0.02, -0.02, t), -xo + t, xo - t), "casing", "member"))
    br = [(-0.095, 0.0), (-0.065, 0.0), (-0.065, 0.003), (-0.092, 0.003), (-0.092, 0.03), (-0.095, 0.03)]
    out.append(K.st(K.ext("member-hx-brackets-low", "casing", GS, "body", 0.7, "xy", [(x, y + 0.447) for x, y in br], 0.043, K.ZW - t, repeat={"count": 2, "translation": [0.16, 0, 0]}), "casing", "member"))
    out.append(K.st(K.ext("member-hx-brackets-high", "casing", GS, "body", 0.7, "xy", [(x, 0.643 + 0.03 - y) for x, y in br], 0.043, K.ZW - t, repeat={"count": 2, "translation": [0.16, 0, 0]}), "casing", "member"))
    out.append(K.st(K.rev("member-hx", "casing", SS, "body", 0.5, hx_profile(0.065, 0.115, -0.147, 0.033, 0.01, 0.01, 0.0355), (WX, WY, 0), ZAX), "casing", "member"))
    out.append(K.st(K.rev("member-flue-collector", "casing", FP, "body", 0.5, K.loop([(0.66, 0.037), (0.66, 0.040), (0.77, 0.040), (0.77, 0.037)]), (0, 0, K.COLLAR_Z), YAX), "casing", "member"))
    out.append(K.st(K.box("member-manifold", "casing", BR, "body", 0.45, (0.38, 0.03, 0.05), (0.0, 0.075, 0.07), radius=0.003), "casing", "member"))
    out.append(K.st(K.ext("member-manifold-bracket", "casing", GS, "body", 0.7, "yz", angle_yz(0.09, K.ZW - t, -0.03, -0.0785, 0.003), -0.15, 0.15), "casing", "member"))
    out.append(K.st(K.rev("member-collar-spigot", "collar", FP, "body", 0.5, K.loop([(-0.01, 0.037), (-0.01, 0.040), (0.05, 0.040), (0.05, 0.037)]), (0, 0, K.COLLAR_Z), YAX), "collar", "member"))
    out.append(K.st(K.rev("member-collar-turret", "collar", FP, "body", 0.5, K.loop([(0.0, 0.0585), (0.0, 0.0625), (0.03, 0.0625), (0.03, 0.0585)]), (0, 0, K.COLLAR_Z), YAX), "collar", "member"))
    out.append(K.st(K.rev("joint-collar-flange", "collar", FP, "body", 0.5, K.loop([(-0.004, 0.0405), (-0.004, 0.07), (0.004, 0.07), (0.004, 0.0405)]), (0, 0, K.COLLAR_Z), YAX), "collar", "joint"))
    out.append(K.st(K.rev("member-connections-pipes", "connections", BR, "body", 0.45, K.loop([(0.0, 0.0095), (0.0, 0.011), (0.06, 0.011), (0.06, 0.0095)]), (K.STUB_X[0], 0, K.STUB_Z), YAX, repeat={"count": 5, "translation": [0.08, 0, 0]}), "connections", "member"))
    out.append(K.st(K.ext("joint-connections-bulkheads", "connections", BR, "body", 0.45, "zx", K.hexagon(K.STUB_Z, K.STUB_X[0], 0.0179), 0.046, 0.06, repeat={"count": 5, "translation": [0.08, 0, 0]}), "connections", "joint"))
    out.append(K.st(K.rev("joint-mode-knob", "mode-knob", PC, "body", 0.45, [(-0.172, 0.0), (-0.172, 0.004), (-0.145, 0.004), (-0.145, 0.0)], (0, 0, 0), ZAX), "mode-knob", "joint"))
    for f in ("flame", "flame-tall"):
        out.append(K.st(K.box(f"joint-{f}", f, BM, "detail", 0.9, (0.008, 0.008, 0.008), (0.0, -0.004 if f == "flame" else -0.002, K.BURNER_Z - (0.006 if f == "flame" else 0.0))), f, "joint"))
    rail = [(0.725, K.ZW - 0.0005), (0.755, K.ZW - 0.0005), (0.755, 0.158), (0.738, 0.158), (0.738, 0.162), (0.751, 0.162), (0.751, K.ZW - 0.004), (0.725, K.ZW - 0.004)]
    out.append(K.st(K.ext("support-wall-bracket", "casing", GS, "body", 0.3, "yz", rail, -0.1, 0.1), "casing", "support", anchor="wall-bracket"))
    out.append(K.st(K.box("support-wall-rest", "casing", GS, "body", 0.3, (0.3, 0.03, 0.016), (0.0, 0.075, K.ZW - 0.0085)), "casing", "support", anchor="wall-rest"))
    out.append(K.st(K.ell("mass-casing", "casing", PS, "detail", 0.5, (0.03, 0.03, 0.03), (0.0, 0.42, 0.0)), "casing", "mass"))
    return out


def body100():
    """🎨 1:100 body: the hood in one piece to the wall, the round window into the dark chamber with the two flame crowns, the control band, the mode knob, the two-step collar and five plain stubs of 26 mm."""
    out = []
    out.append(K.ext("casing", "casing", PS, "body", 0.88, "xy", hood_outline(n=4), K.ZF, K.ZSEAM, holes=[K.circle(WX, WY, 0.039, 28)]))
    out.append(K.ext("chassis", "casing", GS, "body", 0.72, "xy", hood_outline(n=4), K.ZSEAM, K.ZW))
    out.append(K.rev("chamber", "casing", SS, "body", 0.1, [(-0.09, 0.0), (-0.09, 0.039), (-0.155, 0.039), (-0.155, 0.0375), (-0.105, 0.0375), (-0.105, 0.0)], (WX, WY, 0), ZAX))
    out.append(K.box("burner-tube", "casing", BM, "body", 0.45, (0.06, 0.015, 0.03), (0.0, 0.5335, -0.12)))
    out += flames(100)
    out.append(K.ext("panel", "casing", PC, "body", 0.35, "xy", K.rrect(-0.205, 0.065, 0.205, 0.21, 0.0), -0.165, K.ZF + 0.003))
    out.append(K.rev("knob", "mode-knob", PC, "body", 0.72, [(-0.16, 0.0), (-0.16, 0.028), (-0.173, 0.028), (-0.175, 0.026), (-0.175, 0.0)], (0, 0, 0), ZAX))
    out.append(K.rev("collar-base", "collar", FP, "body", 0.7, [(-0.003, 0.0), (-0.003, 0.0625), (0.03, 0.0625), (0.03, 0.0)], (0, 0, K.COLLAR_Z), YAX))
    out.append(K.rev("collar-pipe", "collar", FP, "body", 0.55, [(0.02, 0.0), (0.02, 0.04), (0.05, 0.04), (0.05, 0.0)], (0, 0, K.COLLAR_Z), YAX))
    out.append(K.rev("stubs", "connections", BR, "body", 0.7, [(0.0, 0.0), (0.0, 0.013), (0.056, 0.013), (0.056, 0.0)], (K.STUB_X[0], 0, K.STUB_Z), YAX, repeat={"count": 5, "translation": [0.08, 0, 0]}))
    return out


def thermal100():
    """🌡️ 1:100 thermal: the closed hood with its fascia, the display plate and knob, the window glass, the burner bar and flames, the collar with its outlet, the stubs, and the hidden heat exchanger (housing and collector), fan, vessel and one hydraulic block."""
    out = []
    out.append(K.th(K.ext("th-casing", "casing", PS, "body", 0.88, "xy", hood_outline(n=4), K.ZF, K.ZW), "casing", "sheet"))
    out.append(K.ref("th-panel", "panel", region="casing", layer="sheet"))
    out.append(K.th(K.box("th-display", "casing", PC, "body", 0.3, (0.11, 0.06, 0.002), (-0.115, 0.155, -0.166)), "controls", "panel"))
    out.append(K.ref("th-knob", "knob", region="controls", layer="panel"))
    out.append(K.th(K.rev("th-sight-glass", "casing", GL, "glazing", 0.85, [(-0.151, 0.0), (-0.151, 0.0365), (-0.147, 0.0365), (-0.147, 0.0)], (WX, WY, 0), ZAX, sectionFaces=["+w", "-u", "+u", "-v", "+v"]), "sight-glass", "glass"))
    out.append(K.ref("th-burner-tube", "burner-tube", region="burner", layer="mesh"))
    out.append(K.ref("th-flame-tall", "flame-tall", region="burner", layer="mesh"))
    out.append(K.ref("th-flame", "flame", region="burner", layer="mesh"))
    out.append(K.ref("th-collar-base", "collar-base", region="flue-collar", layer="pipe"))
    out.append(K.ref("th-collar-pipe", "collar-pipe", region="flue-collar", layer="pipe"))
    out.append(K.th(K.rev("th-flue-gas", "collar", FP, "detail", 0.5, [(0.042, 0.0), (0.042, 0.03), (0.0575, 0.03), (0.0575, 0.0)], (0, 0, K.COLLAR_Z), YAX), "flue-collar", None, tags=["flue-gas"]))
    out.append(K.th(K.rev("th-stubs", "connections", BR, "body", 0.7, K.loop([(0.0, 0.0115), (0.0, 0.013), (0.056, 0.013), (0.056, 0.0115)]), (K.STUB_X[0], 0, K.STUB_Z), YAX, repeat={"count": 5, "translation": [0.08, 0, 0]}), "connections", "tube"))
    out.append(K.th(K.rev("th-hx", "casing", SS, "body", 0.5, hx_profile(0.065, 0.115, -0.147, 0.033, 0.015, 0.015, 0.0365), (WX, WY, 0), ZAX), "heat-exchanger", "wall"))
    out.append(K.th(K.rev("th-hx-collector", "casing", SS, "body", 0.5, [(0.67, 0.0), (0.67, 0.04), (0.77, 0.04), (0.77, 0.0)], (0, 0, K.COLLAR_Z), YAX), "heat-exchanger", "wall"))
    out.append(K.th(K.rev("th-fan", "casing", AL, "body", 0.6, [(0.0, 0.0), (0.0, 0.045), (0.07, 0.045), (0.07, 0.0)], (-0.205, 0.60, -0.05), XAX), "fan", "housing"))
    out.append(K.th(K.rev("th-vessel", "casing", PS, "body", 0.35, [(0.065, 0.0), (0.07, 0.09), (0.085, 0.14), (0.105, 0.158), (0.125, 0.14), (0.14, 0.09), (0.145, 0.0)], (0.0, 0.40, 0), ZAX), "expansion-vessel", "shell"))
    out.append(K.th(K.box("th-hydraulics", "casing", BR, "body", 0.4, (0.36, 0.12, 0.09), (0.0, 0.13, 0.04), radius=0.005), "hydraulics", "body"))
    out.append(K.th(K.cap("th-pipes", "casing", BR, "body", 0.4, (-0.01, 0.455, 0.0), (-0.01, 0.18, 0.0), 0.0175), "hydraulics", "body"))
    return out


def structure100():
    """🏗️ 1:100 structure: chassis channel with its top and bottom edges, one pair of heat-exchanger brackets, the heat exchanger and collector, collar spigot and turret, the stub tubes on the manifold, the collar joint, the knob shaft, the flame slides, the rail and the mass."""
    out = []
    t = 0.0015
    xo = K.XW - 0.0015
    u = [(K.ZSEAM, -xo), (K.ZW, -xo), (K.ZW, xo), (K.ZSEAM, xo), (K.ZSEAM, xo - t), (K.ZW - t, xo - t), (K.ZW - t, -xo + t), (K.ZSEAM, -xo + t)]
    out.append(K.st(K.ext("member-chassis", "casing", GS, "body", 0.7, "zx", u, K.Y0 + 0.0015, K.Y1 - 0.0015), "casing", "member"))
    out.append(K.st(K.ext("member-chassis-edges", "casing", GS, "body", 0.7, "yz", angle_yz(K.Y1 - 0.0015, K.ZW - t, -0.02, -0.02, t), -xo + t, xo - t, repeat={"count": 2, "translation": [0, -(K.Y1 - K.Y0 - 0.003 - 0.02), 0]}), "casing", "member"))
    br = [(-0.095, 0.447), (-0.065, 0.447), (-0.065, 0.451), (-0.091, 0.451), (-0.091, 0.477), (-0.095, 0.477)]
    out.append(K.st(K.ext("member-hx-brackets", "casing", GS, "body", 0.7, "xy", br, 0.043, K.ZW - t, repeat={"count": 2, "translation": [0.16, 0, 0]}), "casing", "member"))
    out.append(K.st(K.rev("member-hx", "casing", SS, "body", 0.5, [(-0.147, 0.0), (-0.147, 0.115), (0.043, 0.115), (0.043, 0.0)], (WX, WY, 0), ZAX), "casing", "member"))
    out.append(K.st(K.rev("member-flue-collector", "casing", FP, "body", 0.5, [(0.66, 0.0), (0.66, 0.040), (0.77, 0.040), (0.77, 0.0)], (0, 0, K.COLLAR_Z), YAX), "casing", "member"))
    out.append(K.st(K.box("member-manifold", "casing", BR, "body", 0.45, (0.38, 0.03, 0.1285), (0.0, 0.075, 0.10925), radius=0.003), "casing", "member"))
    out.append(K.st(K.rev("member-collar", "collar", FP, "body", 0.5, [(0.0, 0.0), (0.0, 0.0625), (0.03, 0.0625), (0.03, 0.04), (0.05, 0.04), (0.05, 0.0)], (0, 0, K.COLLAR_Z), YAX), "collar", "member"))
    out.append(K.st(K.rev("joint-collar-flange", "collar", FP, "body", 0.5, [(-0.008, 0.0), (-0.008, 0.07), (0.008, 0.07), (0.008, 0.0)], (0, 0, K.COLLAR_Z), YAX), "collar", "joint"))
    out.append(K.st(K.rev("member-connections-pipes", "connections", BR, "body", 0.45, [(0.0, 0.0), (0.0, 0.011), (0.062, 0.011), (0.062, 0.0)], (K.STUB_X[0], 0, K.STUB_Z), YAX, repeat={"count": 5, "translation": [0.08, 0, 0]}), "connections", "member"))
    out.append(K.st(K.rev("joint-mode-knob", "mode-knob", PC, "body", 0.45, [(-0.172, 0.0), (-0.172, 0.0075), (-0.145, 0.0075), (-0.145, 0.0)], (0, 0, 0), ZAX), "mode-knob", "joint"))
    out.append(K.st(K.box("joint-flames", "flame-tall", BM, "detail", 0.9, (0.015, 0.015, 0.015), (0.0, -0.002, K.BURNER_Z)), "flame-tall", "joint"))
    rail = [(0.725, K.ZW - 0.0005), (0.755, K.ZW - 0.0005), (0.755, 0.158), (0.738, 0.158), (0.738, 0.162), (0.751, 0.162), (0.751, K.ZW - 0.004), (0.725, K.ZW - 0.004)]
    out.append(K.st(K.ext("support-wall-bracket", "casing", GS, "body", 0.3, "yz", rail, -0.1, 0.1), "casing", "support", anchor="wall-bracket"))
    out.append(K.st(K.box("support-wall-rest", "casing", GS, "body", 0.3, (0.3, 0.03, 0.016), (0.0, 0.075, K.ZW - 0.0085)), "casing", "support", anchor="wall-rest"))
    out.append(K.st(K.ell("mass-casing", "casing", PS, "detail", 0.5, (0.03, 0.03, 0.03), (0.0, 0.42, 0.0)), "casing", "mass"))
    return out


def body200():
    """🎨 1:200 body: one hood block with its rounded side edges, the control band, one collar drum and the connection row as one bar."""
    return [
        K.ext("casing", "casing", PS, "body", 0.88, "xy", hood_outline(n=3), -0.15, K.ZW),
        K.ext("panel", "casing", PC, "body", 0.35, "xy", K.rrect(-0.2, 0.065, 0.2, 0.21, 0.0), -0.165, -0.149),
        K.rev("collar", "collar", FP, "body", 0.6, [(-0.004, 0.0), (-0.004, 0.0625), (0.05, 0.0625), (0.05, 0.0)], (0, 0, K.COLLAR_Z), YAX),
        K.box("stubs", "connections", BR, "body", 0.6, (0.34, 0.054, 0.04), (0.0, 0.027, K.STUB_Z)),
    ]


def thermal200():
    """🌡️ 1:200 thermal: hood, control band, a window disc, a merged burner block, the collar with its outlet, the connection bar, and one block per hidden region."""
    return [
        K.ref("th-casing", "casing", region="casing", layer="sheet"),
        K.ref("th-panel", "panel", region="controls", layer="panel"),
        K.th(K.rev("th-sight-glass", "casing", GL, "glazing", 0.85, [(-0.154, 0.0), (-0.154, 0.0365), (-0.15, 0.0365), (-0.15, 0.0)], (WX, WY, 0), ZAX, sectionFaces=["+w", "-u", "+u", "-v", "+v"]), "sight-glass", "glass"),
        K.th(K.box("th-burner", "casing", BM, "body", 0.9, (0.06, 0.04, 0.03), (0.0, 0.55, -0.11)), "burner", "mesh"),
        K.ref("th-collar", "collar", region="flue-collar", layer="pipe"),
        K.th(K.rev("th-flue-gas", "collar", FP, "detail", 0.5, [(0.035, 0.0), (0.035, 0.03), (0.065, 0.03), (0.065, 0.0)], (0, 0, K.COLLAR_Z), YAX), "flue-collar", None, tags=["flue-gas"]),
        K.ref("th-stubs", "stubs", region="connections", layer="tube"),
        K.th(K.rev("th-hx", "casing", SS, "body", 0.5, [(-0.14, 0.0), (-0.14, 0.115), (0.043, 0.115), (0.043, 0.0)], (WX, WY, 0), ZAX), "heat-exchanger", "wall"),
        K.th(K.rev("th-hx-collector", "casing", SS, "body", 0.5, [(0.67, 0.0), (0.67, 0.04), (0.77, 0.04), (0.77, 0.0)], (0, 0, K.COLLAR_Z), YAX), "heat-exchanger", "wall"),
        K.th(K.box("th-fan", "casing", AL, "body", 0.6, (0.07, 0.09, 0.09), (-0.17, 0.60, -0.05)), "fan", "housing"),
        K.th(K.box("th-vessel", "casing", PS, "body", 0.3, (0.31, 0.31, 0.08), (0.0, 0.40, 0.105)), "expansion-vessel", "shell"),
        K.th(K.box("th-hydraulics", "casing", BR, "body", 0.36, (0.36, 0.12, 0.09), (0.0, 0.13, 0.04)), "hydraulics", "body"),
    ]


def structure200():
    """🏗️ 1:200 structure: the chassis channel, the heat exchanger on its bracket pair, the collar drum on the collector, the connection bar, the rail and the mass."""
    t = 0.0015
    xo = K.XW - 0.0015
    u = [(K.ZSEAM, -xo), (K.ZW, -xo), (K.ZW, xo), (K.ZSEAM, xo), (K.ZSEAM, xo - t), (K.ZW - t, xo - t), (K.ZW - t, -xo + t), (K.ZSEAM, -xo + t)]
    rail = [(0.725, K.ZW - 0.0005), (0.755, K.ZW - 0.0005), (0.755, 0.158), (0.738, 0.158), (0.738, 0.162), (0.751, 0.162), (0.751, K.ZW - 0.004), (0.725, K.ZW - 0.004)]
    return [
        K.st(K.ext("member-chassis", "casing", GS, "body", 0.7, "zx", u, K.Y0 + 0.0015, K.Y1 - 0.0015), "casing", "member"),
        K.st(K.box("member-hx-brackets", "casing", GS, "body", 0.7, (0.22, 0.03, 0.13), (0.0, 0.462, 0.108)), "casing", "member"),
        K.st(K.rev("member-hx", "casing", SS, "body", 0.5, [(-0.14, 0.0), (-0.14, 0.115), (0.043, 0.115), (0.043, 0.0)], (WX, WY, 0), ZAX), "casing", "member"),
        K.st(K.rev("member-flue-collector", "casing", FP, "body", 0.5, [(0.66, 0.0), (0.66, 0.040), (0.77, 0.040), (0.77, 0.0)], (0, 0, K.COLLAR_Z), YAX), "casing", "member"),
        K.st(K.rev("member-collar", "collar", FP, "body", 0.5, [(0.0, 0.0), (0.0, 0.0625), (0.05, 0.0625), (0.05, 0.0)], (0, 0, K.COLLAR_Z), YAX), "collar", "member"),
        K.st(K.box("member-connections", "connections", BR, "body", 0.45, (0.34, 0.06, 0.03), (0.0, 0.03, K.STUB_Z)), "connections", "member"),
        K.st(K.ext("support-wall-bracket", "casing", GS, "body", 0.3, "yz", rail, -0.1, 0.1), "casing", "support", anchor="wall-bracket"),
        K.st(K.box("support-wall-rest", "casing", GS, "body", 0.3, (0.3, 0.03, 0.016), (0.0, 0.075, K.ZW - 0.0085)), "casing", "support", anchor="wall-rest"),
        K.st(K.ell("mass-casing", "casing", PS, "detail", 0.5, (0.035, 0.035, 0.035), (0.0, 0.42, 0.0)), "casing", "mass"),
    ]


def main():
    """💾 Writes the volumes source."""
    texts = json.load(open(TEXTS, encoding="utf-8"))
    vols = {
        "50": {"status": "authored", "denominator": 50, "body": body50(), "thermal": thermal50(), "structure": structure50(), "decisions": texts["50"]},
        "100": {"status": "authored", "denominator": 100, "body": body100(), "thermal": thermal100(), "structure": structure100(), "decisions": texts["100"]},
        "200": {"status": "authored", "denominator": 200, "body": body200(), "thermal": thermal200(), "structure": structure200(), "decisions": texts["200"]},
        "500": {"status": "unavailable", "denominator": 500},
    }
    for s, v in vols.items():
        if v["status"] == "authored":
            for lst in ("body", "thermal", "structure"):
                ids = [x["id"] for x in v[lst]]
                if len(ids) != len(set(ids)):
                    raise SystemExit(f"duplicate ids in {s} {lst}")
    json.dump({"volumes": vols}, open(SOURCE, "w", encoding="utf-8"), indent=2, ensure_ascii=False)
    print("written", SOURCE, {s: {k: len(v[k]) for k in ("body", "thermal", "structure")} for s, v in vols.items() if v["status"] == "authored"})


if __name__ == "__main__":
    main()
