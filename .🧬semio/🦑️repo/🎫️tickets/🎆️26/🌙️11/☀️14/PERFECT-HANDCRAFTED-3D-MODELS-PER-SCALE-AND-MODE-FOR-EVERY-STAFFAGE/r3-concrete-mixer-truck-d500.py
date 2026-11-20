"""🧭 1:500 drawings of `machinery/concrete-mixer-truck` (round 3): side elevation, front elevation and plan as true orthographic views of the 1:500 body, a handful of closed forms."""

import math
import os as _o
from importlib import util as _u

_s = _u.spec_from_file_location("kit", _o.path.join(_o.path.dirname(_o.path.abspath(__file__)), "r3-concrete-mixer-truck-kit.py"))
K = _u.module_from_spec(_s)
_s.loader.exec_module(K)

poly, rect, ellipse = K.poly, K.rect, K.ellipse
PS, DPM, GSM, GLM, RBM, EXM = "paint-steel", "drum-paint", "galvanised-steel", "glass", "rubber", "exhaust-steel"
C, D = "chassis", "drum"
REV = {"kind": "revolved", "axisBone": "drum", "axis": "u", "axisX": 0, "axisZ": 0, "mirror": True}
DRUM_HALF = [(-0.70, 0), (-0.70, 0.12), (-0.46, 0.12), (-0.46, 0.35), (0, 0.35), (0.09, 0.76), (0.18, 0.83), (1.10, 1.15), (1.90, 1.15), (4.20, 0.762), (4.20, 0.80), (4.32, 0.80), (4.32, 0.742), (5.40, 0.56), (5.40, 0)]
BODY = ["cab", "chassis", "engine-bay", "running-gear"]


def box4(x0, x1, y0, y1):
    return [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]


def arch(n):
    a0 = math.degrees(math.asin((1.007 - K.WR) / K.ARCH_R))
    a1 = math.degrees(math.asin((0.62 - K.WR) / K.ARCH_R))
    return K.arc(K.AX[0], K.WR, K.ARCH_R, 180 - a0, a1, n)


def elevation():
    o = []
    truck = [(-4.50, 0.44), (-4.38, 0.44), (-4.25, 0.55), (-4.25, 0.81), (-0.625, 0.81), (-0.625, 0.42), (0.645, 0.42), (0.645, 0.81), (1.88, 0.81), (1.88, 0.44), (2.30, 0.44), (2.30, 0.81), (2.58, 0.81), (2.58, 1.007)] + arch(5)[1:] + [(3.70, 0.62), (3.70, 0.42), (4.50, 0.42), (4.50, 1.22), (4.48, 1.22), (4.48, 1.95), (4.37, 2.92), (4.02, 3.10), (2.78, 3.10), (2.585, 2.92), (2.58, 1.16), (-4.25, 1.16), (-4.37, 1.17), (-4.50, 0.62)]
    o.append(poly("truck-body", C, "body", 2, 0.45, BODY, PS, truck))
    o.append(poly("cab-glass", C, "glazing", 2.5, 0.85, "cab-glazing", GLM, box4(3.52, 4.26, 2.08, 2.84)))
    o.append(K.shape("path", "pedestal", C, "body", 1.5, 0.5, "superstructure", PS, d="M -1.7 1.16 L -3.2 1.16 L -2.82 1.98 L -2.76 1.98 L -2.76 2.04 L -2.36 2.04 L -2.36 1.98 L -2.3 1.98 Z M -2.05 1.32 L -2.38 1.8 L -2.72 1.8 L -2.95 1.32 Z"))
    o.append(poly("front-pedestal", C, "body", 1.5, 0.48, "superstructure", PS, [(1.57, 1.16), (2.43, 1.16), (2.43, 1.70), (2.30, 1.70), (2.30, 2.54), (2.20, 2.54), (2.20, 1.72), (1.57, 1.72)]))
    o.append(ellipse("water-tank", C, "body", 2.3, 0.62, "superstructure", GSM, 2.25, 2.78, 0.25, 0.25))
    o.append(poly("hinge-post", C, "body", 1.4, 0.5, "superstructure", PS, [(-3.40, 1.16), (-3.62, 1.16), (-3.62, 2.20), (-4.05, 2.95), (-3.98, 3.40), (-3.90, 3.40), (-3.90, 2.95), (-3.55, 2.45), (-3.40, 2.45)]))
    o.append(poly("discharge-funnel", C, "body", 1.45, 0.5, "chute", GSM, [(-3.835, 2.587), (-3.435, 2.587), (-3.74, 2.03), (-4.12, 2.03)]))
    o.append(poly("rear-ladder", C, "body", 1.6, 0.4, "superstructure", PS, [(-4.20, 0.78), (-4.46, 3.50)], filled=False, closed=False))
    o.append(poly("platform", C, "body", 1.6, 0.42, "superstructure", PS, box4(-4.46, -3.94, 2.462, 2.538)))
    o.append(poly("exhaust-stack", C, "body", 2.2, 0.2, "exhaust-stack", EXM, box4(2.385, 2.535, 1.16, 3.30)))
    o.append(poly("air-intake", C, "body", 2.1, 0.3, "chassis", PS, box4(2.37, 2.55, 1.16, 3.09)))
    o.append(poly("fender-rear", C, "body", 3.9, 0.3, "running-gear", PS, K.arc(K.AX[2], K.WR, 0.675, 12, 90, 3) + K.arc(K.AX[3], K.WR, 0.675, 90, 168, 3) + K.arc(K.AX[3], K.WR, 0.60, 168, 90, 3) + K.arc(K.AX[2], K.WR, 0.60, 90, 12, 3)))
    o.append(poly("fender-2", C, "body", 3.9, 0.3, "running-gear", PS, K.arc(K.AX[1], K.WR, 0.675, 8, 172, 6) + K.arc(K.AX[1], K.WR, 0.60, 172, 8, 6)))
    o.append(poly("drum-body", D, "body", 3, 0.3, ["drum", "drum-drive"], DPM, DRUM_HALF[:-1] + [(s, -r) for s, r in DRUM_HALF[-2:0:-1]], world=False))
    o.append(poly("hopper", D, "body", 3.8, 0.55, "hopper", GSM, [(-4.23, 3.65), (-3.49, 3.65), (-3.51, 3.42), (-3.47, 3.20), (-4.01, 3.20), (-4.17, 3.42)]))
    o.append(poly("chute-main", "chute", "body", 3.9, 0.5, "chute", GSM, box4(0, K.CHUTE_L, 0, K.CHUTE_D), world=False, solid={"kind": "extruded", "sweep": "w", "z0": -K.CHUTE_HW, "z1": K.CHUTE_HW}))
    o.append(poly("chute-ext-folded", "chute-ext", "body", 4, 0.6, "chute", GSM, box4(0, K.EXT_L, 0, K.EXT_D), world=False, solid={"kind": "extruded", "sweep": "w", "z0": -K.EXT_HW, "z1": K.EXT_HW}))
    for i in range(4):
        o.append(ellipse(f"wheel-{i + 1}-disc", f"wheel-{i + 1}", "body", 4, 0.15, ["tyres", "brakes"], RBM, 0, 0, K.WR, K.WR))
    return o


def front():
    o = []
    o.append(poly("chute-ext-folded", "chute-ext", "body", 1.95, 0.6, "chute", GSM, box4(0, K.EXT_L, 0, K.EXT_D), world=False, solid={"kind": "extruded", "sweep": "w", "z0": -K.EXT_HW, "z1": K.EXT_HW}))
    o.append(poly("chute-main", "chute", "body", 1.9, 0.5, "chute", GSM, box4(0, K.CHUTE_L, 0, K.CHUTE_D), world=False, solid={"kind": "extruded", "sweep": "w", "z0": -K.CHUTE_HW, "z1": K.CHUTE_HW}))
    o.append(poly("hopper", D, "body", 1.98, 0.55, "hopper", GSM, [(-0.50, 3.65), (0.50, 3.65), (0.44, 3.42), (0.36, 3.20), (-0.36, 3.20), (-0.44, 3.42)], world=False))
    o.append(poly("drum-body", D, "body", 2, 0.3, ["drum", "drum-drive"], DPM, DRUM_HALF, world=False, solid=REV))
    o.append(poly("rear-ladder", C, "body", 2.15, 0.4, "superstructure", PS, [(0.80, 0.78), (0.80, 3.50)], filled=False, closed=False, world=False))
    o.append(poly("rear-ladder-outer", C, "body", 2.15, 0.4, "superstructure", PS, [(1.22, 0.78), (1.22, 3.50)], filled=False, closed=False, world=False))
    o.append(rect("exhaust-stack", C, "body", 2.2, 0.2, "exhaust-stack", EXM, -1.025, 1.16, 0.15, 2.14))
    o.append(rect("air-intake", C, "body", 2.2, 0.3, "chassis", PS, 0.86, 1.16, 0.18, 1.93))
    for n, sg in (("left", 1), ("right", -1)):
        o.append(rect(f"fender-rear-{n}", C, "body", 2.3, 0.3, "running-gear", PS, min(sg * 0.56, sg * 1.275), 0.645, 0.715, 0.55))
        o.append(rect(f"tyre-3-{n}", "wheel-3", "body", 2.5, 0.15, ["tyres", "brakes"], RBM, min(sg * 0.5875, sg * 1.2725), 0, 0.685, 2 * K.WR))
    o.append(poly("truck-body", C, "body", 3, 0.45, BODY, PS, [(-1.15, 0.42), (1.15, 0.42), (1.15, 1.22), (1.14, 1.22), (1.14, 2.0), (1.095, 2.92), (0.86, 3.10), (-0.86, 3.10), (-1.095, 2.92), (-1.14, 2.0), (-1.14, 1.22), (-1.15, 1.22)], world=False))
    o.append(rect("cab-glass", C, "glazing", 3.5, 0.85, "cab-glazing", GLM, -0.95, 2.052, 1.90, 0.816))
    for n, sg in (("left", 1), ("right", -1)):
        o.append(rect(f"tyre-1-{n}", "wheel-1", "body", 4, 0.15, ["tyres", "brakes"], RBM, min(sg * 0.8625, sg * 1.1975), 0, 0.335, 2 * K.WR))
    return o


def plan():
    o = []
    sol_m = {"kind": "extruded", "sweep": "w", "z0": -K.CHUTE_HW, "z1": K.CHUTE_HW}
    sol_e = {"kind": "extruded", "sweep": "w", "z0": -K.EXT_HW, "z1": K.EXT_HW}
    for n, sg in (("left", 1), ("right", -1)):
        for i in (0, 1):
            o.append(rect(f"tyre-{i + 1}-{n}", f"wheel-{i + 1}", "body", 1.5, 0.15, ["tyres", "brakes"], RBM, K.AX[i] - K.WR, min(sg * 0.8625, sg * 1.1975), 2 * K.WR, 0.335))
        for i in (2, 3):
            o.append(rect(f"tyre-{i + 1}-{n}", f"wheel-{i + 1}", "body", 1.5, 0.15, ["tyres", "brakes"], RBM, K.AX[i] - K.WR, min(sg * 0.5875, sg * 1.2725), 2 * K.WR, 0.685))
        o.append(rect(f"fender-rear-{n}", C, "body", 1.6, 0.3, "running-gear", PS, K.AX[3] - 0.66, min(sg * 0.56, sg * 1.275), K.AX[2] - K.AX[3] + 1.32, 0.715))
        o.append(rect(f"fender-2-{n}", C, "body", 1.6, 0.3, "running-gear", PS, K.AX[1] - 0.668, min(sg * 0.84, sg * 1.24), 1.336, 0.40))
    truck = [(-4.50, -1.20), (-4.38, -1.20), (-4.38, -0.39), (-0.625, -0.39), (-0.625, -0.865), (0.445, -0.865), (0.445, -0.39), (1.88, -0.39), (1.88, -1.13), (2.30, -1.13), (2.30, -0.39), (2.58, -0.39), (2.58, -1.14), (3.70, -1.14), (3.70, -1.15), (4.50, -1.15), (4.50, 1.15), (3.70, 1.15), (3.70, 1.14), (2.58, 1.14), (2.58, 0.39), (0.645, 0.39), (0.645, 1.10), (-0.625, 1.10), (-0.625, 0.39), (-4.38, 0.39), (-4.38, 1.20), (-4.50, 1.20)]
    o.append(poly("truck-body", C, "body", 2, 0.45, BODY, PS, truck, world=False))
    o.append(rect("pedestal", C, "body", 1.45, 0.5, "superstructure", PS, -3.20, -0.70, 1.50, 1.40))
    for n, sg in (("left", 1), ("right", -1)):
        o.append(rect(f"hinge-post-{n}", C, "body", 1.45, 0.5, "superstructure", PS, -4.05, min(sg * 0.50, sg * 0.58), 0.65, 0.08))
    o.append(rect("platform", C, "body", 1.7, 0.42, "superstructure", PS, -4.46, 0.78, 0.52, 0.46))
    o.append(poly("discharge-funnel", C, "body", 1.8, 0.5, "chute", GSM, K.superellipse(-3.80, 0, 0.33, 0.36, 2.5, 8), world=False))
    o.append(poly("chute-main", "chute", "body", 3.9, 0.5, "chute", GSM, box4(0, K.CHUTE_L, 0, K.CHUTE_D), world=False, solid=sol_m))
    o.append(poly("chute-ext-folded", "chute-ext", "body", 4, 0.6, "chute", GSM, box4(0, K.EXT_L, 0, K.EXT_D), world=False, solid=sol_e))
    o.append(rect("cab-glass", C, "glazing", 2.6, 0.85, "cab-glazing", GLM, 4.345, -0.95, 0.13, 1.90))
    o.append(rect("water-tank", C, "body", 2.3, 0.62, "superstructure", GSM, 2.0, -0.80, 0.50, 1.60))
    o.append(ellipse("exhaust-stack", C, "body", 2.4, 0.2, "exhaust-stack", EXM, K.STACK[0], K.STACK[1], 0.075, 0.075))
    o.append(ellipse("air-intake", C, "body", 2.4, 0.3, "chassis", PS, K.INTAKE[0], K.INTAKE[1], 0.09, 0.09))
    o.append(poly("drum-body", D, "body", 3, 0.3, ["drum", "drum-drive"], DPM, DRUM_HALF, world=False, solid=REV))
    o.append(poly("hopper", D, "body", 3.8, 0.55, "hopper", GSM, K.superellipse(-3.86, 0, 0.37, 0.50, 4, 12), world=False))
    return o


DECISIONS = None
