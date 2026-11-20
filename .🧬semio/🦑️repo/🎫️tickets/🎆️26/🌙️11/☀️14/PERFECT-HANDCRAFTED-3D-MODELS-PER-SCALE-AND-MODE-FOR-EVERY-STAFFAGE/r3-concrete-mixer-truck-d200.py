"""🗺️ 1:200 drawings of `machinery/concrete-mixer-truck` (round 3): side elevation, front elevation and plan as true orthographic views of the 1:200 body with outlines generalised to a 30 mm feature."""

import math
import os as _o
from importlib import util as _u

_s = _u.spec_from_file_location("kit", _o.path.join(_o.path.dirname(_o.path.abspath(__file__)), "r3-concrete-mixer-truck-kit.py"))
K = _u.module_from_spec(_s)
_s.loader.exec_module(K)

poly, rect, ellipse = K.poly, K.rect, K.ellipse
PS, DPM, GSM, GLM, RBM, CIM, EXM = "paint-steel", "drum-paint", "galvanised-steel", "glass", "rubber", "cast-iron", "exhaust-steel"
C, D = "chassis", "drum"
REV = {"kind": "revolved", "axisBone": "drum", "axis": "u", "axisX": 0, "axisZ": 0, "mirror": True}
RING = {"kind": "revolved", "axisBone": "chute-swivel", "axis": "u", "axisX": 0, "axisZ": 0, "mirror": True}
FLIGHT = [(-0.2925, 0.5), (0.1325, -0.5)]
FLIGHT_S = [0.50, 1.35, 2.20, 3.05, 3.90]
DRUM_HALF = [(0, 0), (0, 0.30), (0.03, 0.62), (0.09, 0.76), (0.18, 0.83), (1.10, 1.15), (1.90, 1.15), (4.20, 0.762), (4.20, 0.80), (4.32, 0.80), (4.32, 0.742), (5.40, 0.56), (5.40, 0)]
RAIL = [(-3.75, 0.81), (4.10, 0.81), (4.10, 1.08), (2.45, 1.08), (2.45, 1.16), (-4.25, 1.16), (-4.25, 1.08), (-3.75, 1.08)]


def box4(x0, x1, y0, y1):
    return [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]


def band(cx, cy, r0, r1, a0, a1, n):
    return K.arc(cx, cy, r1, a0, a1, n) + K.arc(cx, cy, r0, a1, a0, n)


def arch(n):
    a0 = math.degrees(math.asin((1.007 - K.WR) / K.ARCH_R))
    a1 = math.degrees(math.asin((0.62 - K.WR) / K.ARCH_R))
    return K.arc(K.AX[0], K.WR, K.ARCH_R, 180 - a0, a1, n)


def drum_outline(s0, s1):
    up = [(s0, K.drum_r(s0))] + [(s, r) for s, r in K.DRUM if s0 < s < s1] + [(s1, K.drum_r(s1))]
    return up + [(s, -r) for s, r in up[::-1]]


def elevation():
    o = []
    o.append(poly("fuel-tank", C, "detail", 1.2, 0.6, "chassis", GSM, K.superellipse(-0.16, 0.74, 0.46, 0.31, 8, 8)))
    o.append(poly("battery-box", C, "detail", 1.2, 0.3, "chassis", PS, box4(0.35, 0.65, 0.49, 0.91)))
    o.append(poly("air-intake", C, "detail", 1.25, 0.3, "chassis", PS, [(2.385, 0.67), (2.535, 0.67), (2.535, 2.94), (2.59, 2.94), (2.59, 3.09), (2.33, 3.09), (2.33, 2.94), (2.385, 2.94)]))
    o.append(poly("engine", C, "detail", 1.3, 0.3, "engine-bay", PS, box4(2.89, 4.07, 0.48, 1.24)))
    o.append(poly("gearbox", C, "detail", 1.3, 0.32, "chassis", PS, [(2.49, 0.52), (2.89, 0.52), (2.89, 0.88), (2.49, 0.88), (2.49, 0.71), (-2.415, 0.59), (-2.415, 0.49), (2.49, 0.61)]))
    for i in (0, 1):
        o.append(poly(f"spring-{i + 1}", C, "detail", 1.4, 0.28, "chassis", PS, box4(K.AX[i] - 0.78, K.AX[i] + 0.78, 0.60, 0.76)))
    o.append(poly("bogie-spring", C, "detail", 1.4, 0.28, "chassis", PS, [(K.AX[2] + 0.02, 0.62), (-1.855, 0.64), (-1.855, 0.30), (-2.095, 0.30), (-2.095, 0.64), (K.AX[3] - 0.02, 0.62), (K.AX[3] - 0.02, 0.69), (-1.975, 0.82), (K.AX[2] + 0.02, 0.69)]))
    o.append(poly("frame", C, "body", 1.8, 0.4, "chassis", PS, RAIL))
    o.append(poly("air-tanks", C, "detail", 1.9, 0.35, "chassis", PS, [(-0.625, 0.415), (0.445, 0.415), (0.445, 0.965), (-0.625, 0.965)]))
    o.append(poly("air-filter", C, "detail", 1.9, 0.33, "chassis", PS, box4(1.88, 2.30, 0.44, 1.29)))
    o.append(poly("cab", C, "body", 2, 0.5, "cab", PS, [(2.58, 1.007)] + arch(7)[1:] + [(3.70, 0.62), (3.70, 0.95), (4.06, 0.95), (4.06, 0.42), (4.50, 0.42), (4.50, 0.95), (4.48, 0.95), (4.48, 1.95), (4.37, 2.92), (4.02, 3.10), (2.78, 3.10), (2.585, 2.92)]))
    o.append(poly("door-glass", C, "glazing", 2.4, 0.85, "cab-glazing", GLM, box4(3.52, 4.26, 2.08, 2.84)))
    o.append(poly("door-line", C, "detail", 2.3, 0.3, "cab", PS, [(4.34, 2.92), (4.34, 1.265), (3.36, 1.265), (3.36, 2.92)], filled=False, closed=False))
    o.append(poly("exhaust-stack", C, "body", 2.2, 0.2, "exhaust-stack", EXM, box4(2.385, 2.535, 1.00, 3.30)))
    o.append(ellipse("water-tank", C, "body", 2.3, 0.62, "chassis", GSM, 2.25, 2.78, 0.25, 0.25))
    o.append(poly("front-pedestal", C, "body", 1.5, 0.48, "chassis", PS, [(1.50, 1.16), (2.48, 1.16), (2.40, 1.62), (2.29, 1.65), (2.29, 2.54), (2.21, 2.54), (2.21, 1.67), (1.74, 1.78)]))
    o.append(K.shape("path", "pedestal", C, "body", 1.5, 0.5, "chassis", PS, d="M -1.7 1.16 L -3.2 1.16 L -2.82 1.98 L -2.76 1.98 L -2.76 2.04 L -2.36 2.04 L -2.36 1.98 L -2.3 1.98 Z M -2.05 1.3 L -2.38 1.8 L -2.72 1.8 L -2.95 1.3 Z"))
    o.append(poly("hinge-post", C, "body", 1.4, 0.5, "chassis", PS, [(-3.40, 1.16), (-3.62, 1.16), (-3.62, 2.20), (-4.05, 2.95), (-3.98, 3.40), (-3.92, 3.40), (-3.90, 2.95), (-3.55, 2.45), (-3.40, 2.45)]))
    o.append(poly("discharge-funnel", C, "body", 1.45, 0.5, "chute", GSM, [(-3.835, 2.587), (-3.435, 2.587), (-3.74, 2.10), (-4.12, 2.10)]))
    o.append(poly("rear-ladder", C, "detail", 1.6, 0.4, "chassis", PS, [(-4.20, 0.78), (-4.44, 2.50), (-4.46, 3.50)], filled=False, closed=False))
    o.append(poly("platform", C, "detail", 1.6, 0.42, "chassis", PS, box4(-4.46, -3.94, 2.48, 2.52)))
    o.append(poly("underrun-bar", C, "detail", 1.9, 0.36, "chassis", PS, box4(-4.50, -4.38, 0.44, 0.56)))
    o.append(poly("underrun-bracket", C, "detail", 1.85, 0.38, "chassis", PS, [(-4.42, 0.49), (-4.32, 0.47), (-4.18, 1.11), (-4.28, 1.13)]))
    o.append(poly("tail-lamp", C, "detail", 1.95, 0.5, "chassis", PS, box4(-4.49, -4.38, 0.73, 0.87)))
    o.append(poly("fender-2", C, "detail", 3.9, 0.3, "chassis", PS, band(K.AX[1], K.WR, 0.60, 0.635, 5, 175, 9)))
    o.append(poly("fender-rear", C, "detail", 3.9, 0.3, "chassis", PS, K.arc(K.AX[2], K.WR, 0.635, 10, 90, 5) + K.arc(K.AX[3], K.WR, 0.635, 90, 170, 5) + K.arc(K.AX[3], K.WR, 0.60, 170, 90, 5) + K.arc(K.AX[2], K.WR, 0.60, 90, 10, 5)))
    o.append(poly("spare-chute", C, "detail", 3.95, 0.58, "chute", GSM, box4(-2.60, -1.45, 1.165, 1.345)))
    o.append(poly("drive-housing", D, "body", 3.1, 0.35, "drum-drive", PS, [(-0.72, -0.12), (-0.46, -0.12), (-0.46, -0.35), (-0.10, -0.35), (-0.10, -0.40), (0.02, -0.40), (0.02, 0.36), (-0.10, 0.36), (-0.10, 0.31), (-0.46, 0.31), (-0.46, 0.08), (-0.72, 0.08)], world=False))
    o.append(poly("drum-belly", D, "body", 3, 0.3, "drum", DPM, drum_outline(0.0, 5.40), world=False))
    for k, s in enumerate(FLIGHT_S):
        o.append(poly(f"flight-{k + 1}", "drum-spiral", "detail", 3.4, 0.75, "drum", DPM, [(s + a, v) for a, v in FLIGHT] + [(s + a + 0.16, v) for a, v in FLIGHT[::-1]], world=False))
    o.append(poly("drum-dome-cap", D, "body", 3.6, 0.3, "drum", DPM, drum_outline(0.0, 1.10), world=False))
    o.append(poly("drum-cone-cap", D, "body", 3.6, 0.3, "drum", DPM, drum_outline(4.20, 5.40), world=False))
    o.append(poly("drum-track-ring", D, "body", 3.65, 0.36, "drum", DPM, box4(K.RING_S0, K.RING_S1, -K.RING_R, K.RING_R), world=False))
    o.append(poly("hopper", D, "body", 3.8, 0.55, "hopper", GSM, [(-4.23, 3.65), (-3.49, 3.65), (-3.51, 3.42), (-3.47, 3.22)] + [(-4.01, 3.22), (-4.17, 3.42)]))
    o.append(poly("hopper-spout", D, "body", 3.75, 0.55, "hopper", GSM, [(5.40, -0.375), (5.70, -0.39), (5.70, 0.23), (5.40, 0.215)], world=False))
    o.append(poly("swivel-ring", "chute-swivel", "detail", 3.85, 0.4, "chute", GSM, box4(0.02, 0.08, 0, 0.24), world=False, solid=RING))
    o.append(poly("chute-main", "chute", "body", 3.9, 0.5, "chute", GSM, box4(0, K.CHUTE_L, 0, K.CHUTE_D), world=False, solid={"kind": "extruded", "sweep": "w", "z0": -K.CHUTE_HW, "z1": K.CHUTE_HW}))
    o.append(poly("chute-ext-folded", "chute-ext", "body", 4, 0.6, "chute", GSM, box4(0, K.EXT_L, 0, K.EXT_D), world=False, solid={"kind": "extruded", "sweep": "w", "z0": -K.EXT_HW, "z1": K.EXT_HW}))
    for i in range(4):
        b, n = f"wheel-{i + 1}", i + 1
        o.append(ellipse(f"tyre-{n}", b, "body", 4, 0.12, "tyres", RBM, 0, 0, K.WR, K.WR))
        o.append(ellipse(f"rim-{n}", b, "detail", 4.1, 0.6, "chassis", PS, 0, 0, 0.305, 0.305))
        o.append(ellipse(f"brake-drum-{n}", b, "detail", 4.2, 0.4, "brakes", CIM, 0, 0, 0.12, 0.12))
    return o


def front():
    o = []
    o.append(poly("chute-ext-folded", "chute-ext", "body", 1.95, 0.6, "chute", GSM, box4(0, K.EXT_L, 0, K.EXT_D), world=False, solid={"kind": "extruded", "sweep": "w", "z0": -K.EXT_HW, "z1": K.EXT_HW}))
    o.append(poly("chute-main", "chute", "body", 1.9, 0.5, "chute", GSM, box4(0, K.CHUTE_L, 0, K.CHUTE_D), world=False, solid={"kind": "extruded", "sweep": "w", "z0": -K.CHUTE_HW, "z1": K.CHUTE_HW}))
    o.append(poly("hopper", D, "body", 1.98, 0.55, "hopper", GSM, [(-0.50, 3.65), (0.50, 3.65), (0.44, 3.42), (0.36, 3.22), (-0.36, 3.22), (-0.44, 3.42)], world=False))
    o.append(poly("drum", D, "body", 2, 0.3, "drum", DPM, DRUM_HALF, world=False, solid=REV))
    o.append(poly("rear-ladder", C, "detail", 2.15, 0.4, "chassis", PS, [(0.80, 0.78), (0.80, 3.50)], filled=False, closed=False, world=False))
    o.append(poly("rear-ladder-outer", C, "detail", 2.15, 0.4, "chassis", PS, [(1.22, 0.78), (1.22, 3.50)], filled=False, closed=False, world=False))
    for n, sg in (("left", 1), ("right", -1)):
        o.append(rect(f"fender-rear-{n}", C, "detail", 2.3, 0.3, "chassis", PS, min(sg * 0.56, sg * 1.275), 0.628, 0.715, 0.527))
        o.append(rect(f"spare-chute-{n}", C, "detail", 2.35, 0.58, "chute", GSM, min(sg * 0.72, sg * 1.22), 1.165, 0.50, 0.18))
        o.append(rect(f"fender-2-{n}", C, "detail", 2.4, 0.3, "chassis", PS, min(sg * 0.84, sg * 1.24), 0.574, 0.40, 0.581))
        o.append(rect(f"tyre-3-{n}", "wheel-3", "body", 2.5, 0.12, ["tyres"], RBM, min(sg * 0.5875, sg * 1.2525), 0, 0.665, 2 * K.WR))
    o.append(rect("exhaust-stack", C, "body", 2.2, 0.2, "exhaust-stack", EXM, -1.025, 1.00, 0.15, 2.30))
    o.append(rect("air-intake", C, "detail", 2.2, 0.3, "chassis", PS, 0.82, 2.94, 0.26, 0.15))
    o.append(rect("bumper-frame", C, "body", 2.9, 0.4, "chassis", PS, -1.16, 0.42, 2.32, 0.53))
    o.append(poly("cab", C, "body", 3, 0.5, "cab", PS, [(-1.14, 0.95), (1.14, 0.95), (1.14, 2.0), (1.095, 2.92), (0.86, 3.10), (-0.86, 3.10), (-1.095, 2.92), (-1.14, 2.0)], world=False))
    o.append(rect("front-grille", C, "detail", 3.2, 0.2, "cab", PS, -0.65, 1.28, 1.30, 0.56))
    o.append(rect("windscreen", C, "glazing", 3.5, 0.85, "cab-glazing", GLM, -0.95, 2.052, 1.90, 0.816))
    for n, sg in (("left", 1), ("right", -1)):
        o.append(rect(f"mirror-{n}", C, "detail", 3.3, 0.2, "cab", PS, min(sg * 1.125, sg * 1.275), 2.35, 0.15, 0.42))
        o.append(rect(f"tyre-1-{n}", "wheel-1", "body", 4, 0.12, ["tyres"], RBM, min(sg * 0.8625, sg * 1.1775), 0, K.TW, 2 * K.WR))
    return o


def plan():
    o = []
    sol_m = {"kind": "extruded", "sweep": "w", "z0": -K.CHUTE_HW, "z1": K.CHUTE_HW}
    sol_e = {"kind": "extruded", "sweep": "w", "z0": -K.EXT_HW, "z1": K.EXT_HW}
    o.append(rect("underrun-bar", C, "detail", 1.3, 0.36, "chassis", PS, -4.50, -1.20, 0.12, 2.40))
    o.append(rect("frame", C, "body", 1.4, 0.4, "chassis", PS, K.SUB_X0, -0.385, K.RAIL_X1 - K.SUB_X0, 0.77))
    o.append(rect("pedestal", C, "body", 1.45, 0.5, "chassis", PS, -3.20, -0.70, 1.50, 1.40))
    for n, sg in (("left", 1), ("right", -1)):
        for i in (0, 1):
            o.append(rect(f"tyre-{i + 1}-{n}", f"wheel-{i + 1}", "body", 1.5, 0.12, ["tyres"], RBM, K.AX[i] - K.WR, min(sg * 0.8625, sg * 1.1775), 2 * K.WR, K.TW))
        for i in (2, 3):
            o.append(rect(f"tyre-{i + 1}-{n}", f"wheel-{i + 1}", "body", 1.5, 0.12, ["tyres"], RBM, K.AX[i] - K.WR, min(sg * 0.5875, sg * 1.2525), 2 * K.WR, 0.665))
        o.append(rect(f"fender-2-{n}", C, "detail", 1.6, 0.3, "chassis", PS, K.AX[1] - 0.633, min(sg * 0.84, sg * 1.24), 1.266, 0.40))
        o.append(rect(f"fender-rear-{n}", C, "detail", 1.6, 0.3, "chassis", PS, K.AX[3] - 0.626, min(sg * 0.56, sg * 1.275), K.AX[2] - K.AX[3] + 1.252, 0.715))
        o.append(rect(f"spare-chute-{n}", C, "detail", 1.7, 0.58, "chute", GSM, -2.60, min(sg * 0.72, sg * 1.22), 1.15, 0.50))
        o.append(rect(f"hinge-post-{n}", C, "body", 1.45, 0.5, "chassis", PS, -4.05, min(sg * 0.50, sg * 0.58), 0.65, 0.08))
        o.append(rect(f"mirror-{n}", C, "detail", 2.5, 0.2, "cab", PS, 4.41, min(sg * 1.125, sg * 1.275), 0.06, 0.15))
    o.append(rect("fuel-tank", C, "detail", 1.55, 0.6, "chassis", GSM, -0.62, 0.49, 1.27, 0.62))
    o.append(rect("silencer", C, "detail", 1.55, 0.3, "chassis", PS, 1.88, -1.13, 0.42, 0.60))
    o.append(rect("air-tanks", C, "detail", 1.55, 0.35, "chassis", PS, -0.625, -0.865, 1.07, 0.25))
    o.append(rect("air-filter", C, "detail", 1.55, 0.33, "chassis", PS, 1.88, 0.58, 0.42, 0.50))
    o.append(rect("platform", C, "detail", 1.7, 0.42, "chassis", PS, -4.46, 0.78, 0.52, 0.46))
    o.append(poly("discharge-funnel", C, "body", 1.8, 0.5, "chute", GSM, K.superellipse(-3.80, 0, 0.33, 0.36, 2.5, 8), world=False))
    o.append(poly("swivel-ring", "chute-swivel", "detail", 1.85, 0.4, "chute", GSM, box4(0.02, 0.08, 0, 0.24), world=False, solid=RING))
    o.append(poly("chute-main", "chute", "body", 3.9, 0.5, "chute", GSM, box4(0, K.CHUTE_L, 0, K.CHUTE_D), world=False, solid=sol_m))
    o.append(poly("chute-ext-folded", "chute-ext", "body", 4, 0.6, "chute", GSM, box4(0, K.EXT_L, 0, K.EXT_D), world=False, solid=sol_e))
    o.append(poly("cab", C, "body", 2, 0.5, "cab", PS, K.superellipse(3.53, 0, 0.95, 1.14, 8, 20), world=False))
    o.append(poly("bumper", C, "body", 1.9, 0.45, "cab", PS, K.superellipse(4.28, 0, 0.22, 1.16, 8, 12), world=False))
    o.append(rect("windscreen", C, "glazing", 2.6, 0.85, "cab-glazing", GLM, 4.37, -0.95, 0.09, 1.90))
    o.append(rect("water-tank", C, "body", 2.3, 0.62, "chassis", GSM, 2.0, -0.80, 0.50, 1.60))
    o.append(ellipse("exhaust-stack", C, "body", 2.4, 0.2, "exhaust-stack", EXM, K.STACK[0], K.STACK[1], 0.075, 0.075))
    o.append(ellipse("air-intake", C, "detail", 2.4, 0.3, "chassis", PS, K.INTAKE[0], K.INTAKE[1], 0.13, 0.13))
    o.append(poly("drive-housing", D, "body", 2.9, 0.35, "drum-drive", PS, [(0, 0), (-0.72, 0), (-0.72, 0.10), (-0.46, 0.10), (-0.46, 0.33), (-0.10, 0.33), (-0.10, 0.38), (0.02, 0.38), (0.02, 0)], world=False, solid=REV))
    o.append(poly("drum-belly", D, "body", 3, 0.3, "drum", DPM, DRUM_HALF, world=False, solid=REV))
    for k, s in enumerate(FLIGHT_S):
        x = K.DRUM_O[0] - s * 0.9816
        o.append(poly(f"flight-{k + 1}", "drum-spiral", "detail", 3.4, 0.75, "drum", DPM, [(x + 0.41, 0.5), (x - 0.41, -0.5), (x - 0.57, -0.5), (x + 0.25, 0.5)], filled=False, world=False))
    o.append(poly("hopper", D, "body", 3.8, 0.55, "hopper", GSM, K.superellipse(-3.86, 0, 0.37, 0.50, 4, 12), world=False))
    return o


DECISIONS = None
