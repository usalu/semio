"""✏️ 1:50 drawings of `machinery/concrete-mixer-truck` (round 3): the side elevation, the front elevation and the plan as true orthographic views of the 1:50 body, shape by shape, from the same reference numbers as the solids."""

import math
import os as _o
from importlib import util as _u

_s = _u.spec_from_file_location("kit", _o.path.join(_o.path.dirname(_o.path.abspath(__file__)), "r3-concrete-mixer-truck-kit.py"))
K = _u.module_from_spec(_s)
_s.loader.exec_module(K)

poly, rect, ellipse = K.poly, K.rect, K.ellipse
PS, DPM, GSM, GLM, RBM, CIM, EXM = "paint-steel", "drum-paint", "galvanised-steel", "glass", "rubber", "cast-iron", "exhaust-steel"
REV = {"kind": "revolved", "axisBone": "drum", "axis": "u", "axisX": 0, "axisZ": 0, "mirror": True}
RING = {"kind": "revolved", "axisBone": "chute-swivel", "axis": "u", "axisX": 0, "axisZ": 0, "mirror": True}
FLIGHT = [(-0.2925, 0.5), (-0.2143, 0.3333), (-0.1454, 0.1667), (-0.08, 0), (-0.0146, -0.1667), (0.0543, -0.3333), (0.1325, -0.5)]
FLIGHT_S = [0.50, 1.35, 2.20, 3.05, 3.90]
CAB_SIDE = [(4.48, 0.95), (4.48, 1.95), (4.465, 2.02), (4.385, 2.90), (4.34, 2.98), (4.23, 3.05), (4.00, 3.10), (2.80, 3.10), (2.64, 3.05), (2.585, 2.98), (2.58, 2.90)]
DRUM_HALF = [(0, 0), (0, 0.30), (0.03, 0.62), (0.09, 0.76), (0.18, 0.83), (1.10, 1.15), (1.90, 1.15), (4.20, 0.762), (4.20, 0.80), (4.32, 0.80), (4.32, 0.742), (5.33, 0.574), (5.33, 0.595), (5.40, 0.595), (5.40, 0)]


def arch(n):
    a0 = math.degrees(math.asin((1.007 - K.WR) / K.ARCH_R))
    a1 = math.degrees(math.asin((0.62 - K.WR) / K.ARCH_R))
    return K.arc(K.AX[0], K.WR, K.ARCH_R, 180 - a0, a1, n)


def box4(x0, x1, y0, y1):
    return [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]


def band(cx, cy, r0, r1, a0, a1, n):
    return K.arc(cx, cy, r1, a0, a1, n) + K.arc(cx, cy, r0, a1, a0, n)


def drum_outline(s0, s1):
    up = [(s, r) for s, r in K.DRUM if s0 < s < s1]
    up = [(s0, K.drum_r(s0))] + up + [(s1, K.drum_r(s1))]
    return up + [(s, -r) for s, r in up[::-1]]


def roller_side():
    c = K.dw(4.26, K.ROLLER * math.cos(math.radians(38)))
    return [(c[0] + a * K.DU[0] + b * K.DV[0], c[1] + a * K.DU[1] + b * K.DV[1]) for a, b in [(-0.07, -0.11), (0.07, -0.11), (0.07, 0.11), (-0.07, 0.11)]]


def wheel_elevation(out, i, holes):
    b = f"wheel-{i + 1}"
    n = i + 1
    rear = i >= 2
    out.append(ellipse(f"tyre-{n}", b, "body", 4, 0.12, "tyres", RBM, 0, 0, K.WR, K.WR, hatch="tread") if holes == 10 else ellipse(f"tyre-{n}", b, "body", 4, 0.12, "tyres", RBM, 0, 0, K.WR, K.WR))
    out.append(ellipse(f"rim-{n}", b, "detail", 4.1, 0.6, "chassis", PS, 0, 0, 0.31, 0.31))
    if rear:
        out.append(ellipse(f"rim-well-{n}", b, "detail", 4.15, 0.42, "chassis", PS, 0, 0, 0.262, 0.262))
    out.append(ellipse(f"brake-drum-{n}", b, "detail", 4.2, 0.5, "brakes", CIM, 0, 0, 0.15 if rear else 0.14, 0.15 if rear else 0.14))
    out.append(ellipse(f"hub-{n}", b, "detail", 4.3, 0.3, "brakes", CIM, 0, 0, 0.08 if rear else 0.06, 0.08 if rear else 0.06))
    for k in range(holes):
        a = 2 * math.pi * k / holes + math.pi / holes
        out.append(ellipse(f"handhole-{n}-{k + 1}", b, "detail", 4.3, 0, "chassis", PS, 0.222 * math.cos(a), 0.222 * math.sin(a), 0.024, 0.024, filled=False))


def elevation():
    o = []
    C, D = "chassis", "drum"
    o.append(poly("fuel-tank", C, "detail", 1.2, 0.6, "chassis", GSM, K.superellipse(-0.16, 0.74, 0.46, 0.31, 6, 20)))
    o.append(poly("battery-box", C, "detail", 1.2, 0.3, "chassis", PS, box4(0.35, 0.65, 0.49, 0.91)))
    o.append(poly("air-filter", C, "detail", 1.2, 0.33, "chassis", PS, box4(1.88, 2.30, 0.67, 1.29)))
    o.append(poly("air-intake", C, "detail", 1.25, 0.3, "chassis", PS, [(2.385, 1.30), (2.535, 1.30), (2.535, 2.95), (2.59, 2.95), (2.59, 3.05), (2.55, 3.09), (2.37, 3.09), (2.33, 3.05), (2.33, 2.95), (2.385, 2.95)]))
    o.append(poly("engine", C, "detail", 1.3, 0.3, "engine-bay", PS, [(2.90, 0.61), (3.05, 0.61), (3.05, 0.45), (3.67, 0.45), (3.67, 0.61), (3.90, 0.61), (3.90, 1.23), (2.90, 1.23)]))
    o.append(poly("radiator", C, "detail", 1.3, 0.26, "engine-bay", PS, box4(3.97, 4.07, 0.61, 1.35)))
    o.append(poly("gearbox", C, "detail", 1.3, 0.32, "chassis", PS, box4(2.49, 2.99, 0.52, 0.88)))
    o.append(poly("prop-shaft", C, "detail", 1.3, 0.25, "chassis", PS, [(2.50, 0.61), (2.50, 0.71), (-1.015, 0.59), (-1.015, 0.49)]))
    o.append(poly("prop-shaft-2", C, "detail", 1.3, 0.25, "chassis", PS, box4(-2.415, -1.495, 0.495, 0.585)))
    for i in (0, 1):
        o.append(poly(f"spring-{i + 1}", C, "detail", 1.4, 0.28, "chassis", PS, box4(K.AX[i] - 0.78, K.AX[i] + 0.78, 0.59, 0.68)))
        o.append(poly(f"spring-hanger-{i + 1}", C, "detail", 1.4, 0.33, "chassis", PS, box4(K.AX[i] - 0.79, K.AX[i] - 0.69, 0.68, 0.81)))
        o.append(poly(f"spring-hanger-{i + 1}-rear", C, "detail", 1.4, 0.33, "chassis", PS, box4(K.AX[i] + 0.69, K.AX[i] + 0.79, 0.68, 0.81)))
    o.append(poly("bogie-spring", C, "detail", 1.4, 0.28, "chassis", PS, [(K.AX[2] + 0.02, 0.62), (-1.975, 0.65), (K.AX[3] - 0.02, 0.62), (K.AX[3] - 0.02, 0.69), (-1.975, 0.79), (K.AX[2] + 0.02, 0.69)]))
    o.append(poly("bogie-saddle", C, "detail", 1.4, 0.32, "chassis", PS, [(-2.095, 0.30), (-1.855, 0.30), (-1.855, 0.75), (-1.825, 0.75), (-1.825, 0.85), (-2.125, 0.85), (-2.125, 0.75), (-2.095, 0.75)]))
    o.append(poly("frame", C, "body", 1.8, 0.4, "chassis", PS, box4(K.RAIL_X0, K.RAIL_X1, K.RAIL_Y0, K.RAIL_Y1)))
    o.append(poly("subframe", C, "body", 1.8, 0.42, "chassis", PS, box4(K.SUB_X0, K.SUB_X1, 1.08, K.SUB_Y1)))
    o.append(poly("air-tank", C, "detail", 1.9, 0.35, "chassis", PS, K.superellipse(-0.09, 0.54, 0.535, 0.125, 5, 16)))
    o.append(poly("air-tank-2", C, "detail", 1.9, 0.35, "chassis", PS, K.superellipse(-0.09, 0.84, 0.535, 0.125, 5, 16)))
    o.append(poly("silencer", C, "detail", 1.9, 0.3, "chassis", PS, box4(1.88, 2.30, 0.44, 1.00)))
    o.append(poly("cab", C, "body", 2, 0.5, "cab", PS, [(2.58, 1.007)] + arch(14)[1:] + [(3.70, 0.62), (3.70, 0.95)] + CAB_SIDE))
    o.append(poly("bumper", C, "body", 2.1, 0.4, "chassis", PS, box4(4.06, 4.50, 0.42, 0.95)))
    o.append(poly("cab-step", C, "detail", 1.95, 0.3, "chassis", PS, box4(3.72, 4.05, 0.455, 0.485)))
    o.append(poly("cab-step-upper", C, "detail", 1.95, 0.3, "chassis", PS, box4(3.72, 4.05, 0.725, 0.755)))
    o.append(poly("cab-step-cheek", C, "detail", 1.95, 0.44, "cab", PS, box4(3.70, 3.73, 0.44, 0.95)))
    o.append(poly("cab-step-cheek-front", C, "detail", 1.95, 0.44, "cab", PS, box4(4.02, 4.05, 0.44, 0.95)))
    o.append(poly("door-glass", C, "glazing", 2.4, 0.85, "cab-glazing", GLM, box4(3.52, 4.26, 2.08, 2.84)))
    o.append(poly("door-line", C, "detail", 2.3, 0.3, "cab", PS, [(4.34, 2.92), (4.34, 1.265), (3.36, 1.265), (3.36, 2.92)], filled=False, closed=False))
    o.append(poly("door-handle", C, "detail", 2.5, 0.2, "cab", PS, box4(3.44, 3.60, 1.78, 1.82)))
    o.append(poly("mirror-arm", C, "detail", 2.6, 0.2, "cab", PS, box4(4.425, 4.475, 2.35, 2.77)))
    o.append(poly("sun-visor", C, "detail", 2.6, 0.25, "cab", PS, [(4.39, 2.905), (4.475, 2.885), (4.48, 2.91), (4.395, 2.93)]))
    o.append(poly("exhaust-stack", C, "body", 2.2, 0.2, "exhaust-stack", EXM, box4(2.398, 2.522, 1.00, 3.22)))
    o.append(poly("stack-shield", C, "detail", 2.25, 0.24, "exhaust-stack", EXM, box4(2.37, 2.55, 1.55, 2.90)))
    o.append(poly("stack-cap", C, "detail", 2.3, 0.25, "exhaust-stack", EXM, [(2.398, 3.22), (2.522, 3.22), (2.483, 3.30), (2.367, 3.30)]))
    o.append(ellipse("water-tank", C, "body", 2.3, 0.62, "chassis", GSM, 2.25, 2.78, 0.25, 0.25))
    o.append(poly("water-tank-post", C, "detail", 1.6, 0.44, "chassis", PS, [(2.21, 1.62), (2.29, 1.62), (2.29, 2.495), (2.43, 2.495), (2.43, 2.545), (2.07, 2.545), (2.07, 2.495), (2.21, 2.495)]))
    o.append(poly("oil-cooler", C, "detail", 1.45, 0.3, "drum-drive", PS, box4(1.75, 2.21, 1.30, 1.74)))
    o.append({**K.shape("path", "front-pedestal", C, "body", 1.5, 0.48, "chassis", PS, d="M 1.5 1.16 L 2.48 1.16 L 2.4 1.62 L 2.05 1.7 L 1.74 1.78 Z M 1.8 1.28 L 1.9 1.58 L 2.18 1.52 L 2.24 1.28 Z")})
    o.append(poly("front-pedestal-brace", C, "detail", 1.5, 0.46, "chassis", PS, box4(1.50, 2.48, 1.16, 1.22)))
    o.append({**K.shape("path", "pedestal", C, "body", 1.5, 0.5, "chassis", PS, d="M -1.7 1.16 L -3.2 1.16 L -2.82 1.98 L -2.3 1.98 Z M -2.05 1.3 L -2.38 1.8 L -2.72 1.8 L -2.95 1.3 Z")})
    o.append(poly("pedestal-foot", C, "detail", 1.5, 0.46, "chassis", PS, box4(-3.23, -1.67, 1.16, 1.22)))
    o.append(poly("pedestal-beam", C, "detail", 1.45, 0.48, "chassis", PS, box4(-2.71, -2.41, 1.80, 1.96)))
    o.append(poly("roller-bracket", C, "detail", 1.55, 0.4, "chassis", PS, box4(-2.719, -2.379, 1.95, 2.05)))
    o.append(poly("support-roller", C, "detail", 1.6, 0.3, "chassis", PS, roller_side()))
    o.append(poly("hinge-post", C, "body", 1.4, 0.5, "chassis", PS, [(-3.40, 1.16), (-3.62, 1.16), (-3.62, 2.20), (-4.05, 2.95), (-3.90, 2.95), (-3.55, 2.45), (-3.40, 2.45)]))
    o.append(poly("hopper-stay", C, "detail", 1.4, 0.4, "chassis", PS, box4(-4.005, -3.955, 2.95, 3.40)))
    o.append(poly("discharge-funnel", C, "body", 1.45, 0.5, "chute", GSM, [(-3.835, 2.587), (-3.435, 2.587), (-3.58, 2.36), (-3.74, 2.10), (-4.12, 2.10), (-4.02, 2.36)]))
    o.append(poly("rear-ladder", C, "detail", 1.6, 0.4, "chassis", PS, [(-4.20, 0.78), (-4.44, 2.50), (-4.46, 3.50)], filled=False, closed=False))
    o.append(poly("platform", C, "detail", 1.6, 0.42, "chassis", PS, box4(-4.46, -3.94, 2.485, 2.515)))
    o.append(poly("underrun-bar", C, "detail", 1.9, 0.36, "chassis", PS, box4(-4.50, -4.38, 0.44, 0.56)))
    o.append(poly("underrun-bracket", C, "detail", 1.85, 0.38, "chassis", PS, [(-4.42, 0.49), (-4.32, 0.47), (-4.18, 1.11), (-4.28, 1.13)]))
    o.append(poly("tail-lamp", C, "detail", 1.95, 0.75, "chassis", GLM, box4(-4.49, -4.43, 0.73, 0.87)))
    o.append(poly("fender-2", C, "detail", 3.9, 0.3, "chassis", PS, band(K.AX[1], K.WR, 0.60, 0.62, 5, 175, 17)))
    o.append(poly("fender-rear", C, "detail", 3.9, 0.3, "chassis", PS, K.arc(K.AX[2], K.WR, 0.62, 10, 90, 8) + K.arc(K.AX[3], K.WR, 0.62, 90, 170, 8) + K.arc(K.AX[3], K.WR, 0.60, 170, 90, 8) + K.arc(K.AX[2], K.WR, 0.60, 90, 10, 8)))
    o.append(poly("mud-flap", C, "detail", 3.9, 0.15, "chassis", RBM, [(-3.31, 0.65), (-3.31, 0.23)], filled=False, closed=False))
    o.append(poly("mud-flap-front", C, "detail", 3.9, 0.15, "chassis", RBM, [(2.352, 0.58), (2.352, 0.24)], filled=False, closed=False))
    o.append(poly("fender-1-rear", C, "detail", 3.9, 0.3, "chassis", PS, band(K.AX[0], K.WR, 0.66, 0.68, 132, 176, 6)))
    o.append(poly("tail-lamp-beam", C, "detail", 1.9, 0.36, "chassis", PS, box4(-4.43, -4.38, 0.75, 0.85)))
    o.append(poly("spare-chute", C, "detail", 3.95, 0.58, "chute", GSM, box4(-2.60, -1.45, 1.165, 1.345)))
    o.append(poly("drive-housing", D, "body", 3.1, 0.35, "drum-drive", PS, [(-0.46, -0.25), (-0.42, -0.33), (-0.12, -0.33), (-0.10, -0.39), (0.02, -0.39), (0.02, 0.39), (-0.10, 0.39), (-0.12, 0.33), (-0.42, 0.33), (-0.46, 0.25)], world=False))
    o.append(poly("drive-motor", D, "detail", 3.05, 0.3, "drum-drive", PS, [(-0.72, -0.125), (-0.52, -0.145), (-0.46, -0.145), (-0.46, 0.065), (-0.52, 0.065), (-0.72, 0.045)], world=False))
    o.append(poly("drum-belly", D, "body", 3, 0.3, "drum", DPM, drum_outline(0.0, 5.40), world=False))
    for k, s in enumerate(FLIGHT_S):
        pts = [(s + a, v) for a, v in FLIGHT] + [(s + a + 0.16, v) for a, v in FLIGHT[::-1]]
        o.append(poly(f"flight-{k + 1}", "drum-spiral", "detail", 3.4, 0.75, "drum", DPM, pts, world=False))
    o.append(poly("drum-dome-cap", D, "body", 3.6, 0.3, "drum", DPM, drum_outline(0.0, 1.10), world=False))
    o.append(poly("drum-cone-cap", D, "body", 3.6, 0.3, "drum", DPM, drum_outline(4.20, 5.40), world=False))
    o.append(poly("drum-track-ring", D, "body", 3.65, 0.36, "drum", DPM, box4(K.RING_S0, K.RING_S1, -K.RING_R, K.RING_R), world=False))
    o.append(poly("drum-mouth-rim", D, "body", 3.65, 0.34, "drum", DPM, box4(5.33, 5.40, -K.LIP_R, K.LIP_R), world=False))
    o.append(poly("drum-seam", D, "detail", 3.7, 0.3, "drum", DPM, [(1.90, -1.15), (1.90, 1.15)], filled=False, closed=False, world=False))
    o.append(poly("drum-seam-front", D, "detail", 3.7, 0.3, "drum", DPM, [(1.10, -1.15), (1.10, 1.15)], filled=False, closed=False, world=False))
    o.append(poly("drum-hatch", D, "detail", 3.7, 0.36, "drum", DPM, K.superellipse(1.50, 0, 0.23, 0.18, 4, 16), filled=False, world=False))
    o.append(poly("hopper-spout", D, "body", 3.75, 0.55, "hopper", GSM, [(5.40, -0.375), (5.70, -0.39), (5.70, 0.23), (5.40, 0.215)], world=False))
    o.append(poly("hopper", D, "body", 3.8, 0.55, "hopper", GSM, [(-4.23, 3.65), (-3.49, 3.65), (-3.51, 3.42), (-3.47, 3.22), (-4.01, 3.22), (-4.17, 3.42)]))
    o.append(poly("swivel-ring", "chute-swivel", "detail", 3.85, 0.4, "chute", GSM, box4(0.02, 0.08, 0, 0.24), world=False, solid=RING))
    o.append(poly("chute-main", "chute", "body", 3.9, 0.5, "chute", GSM, box4(0, K.CHUTE_L, 0, K.CHUTE_D), world=False, solid={"kind": "extruded", "sweep": "w", "z0": -K.CHUTE_HW, "z1": K.CHUTE_HW}))
    o.append(poly("chute-ext-folded", "chute-ext", "body", 4, 0.6, "chute", GSM, box4(0, K.EXT_L, 0, K.EXT_D), world=False, solid={"kind": "extruded", "sweep": "w", "z0": -K.EXT_HW, "z1": K.EXT_HW}))
    for i in range(4):
        wheel_elevation(o, i, 10)
    return o


def front():
    o = []
    C, D = "chassis", "drum"
    o.append(poly("chute-ext-folded", "chute-ext", "body", 1.95, 0.6, "chute", GSM, box4(0, K.EXT_L, 0, K.EXT_D), world=False, solid={"kind": "extruded", "sweep": "w", "z0": -K.EXT_HW, "z1": K.EXT_HW}))
    o.append(poly("chute-main", "chute", "body", 1.9, 0.5, "chute", GSM, box4(0, K.CHUTE_L, 0, K.CHUTE_D), world=False, solid={"kind": "extruded", "sweep": "w", "z0": -K.CHUTE_HW, "z1": K.CHUTE_HW}))
    o.append(poly("hopper", D, "body", 1.98, 0.55, "hopper", GSM, [(-0.50, 3.65), (0.50, 3.65), (0.44, 3.42), (0.36, 3.22), (-0.36, 3.22), (-0.44, 3.42)], world=False))
    o.append(poly("drum", D, "body", 2, 0.3, "drum", DPM, DRUM_HALF, world=False, solid=REV))
    o.append(ellipse("drum-seam", D, "detail", 2.1, 0.3, "drum", DPM, 0, 2.4025, 1.15, 1.129, filled=False))
    o.append(poly("rear-ladder", C, "detail", 2.15, 0.4, "chassis", PS, [(0.80, 0.78), (0.80, 3.50), (1.22, 3.50), (1.22, 0.78)], filled=False, closed=False, world=False))
    o.append(poly("guard-rail", C, "detail", 2.15, 0.4, "chassis", PS, [(0.80, 3.00), (1.22, 3.00)], filled=False, closed=False, world=False))
    o.append(rect("platform", C, "detail", 2.15, 0.42, "chassis", PS, 0.78, 2.485, 0.46, 0.03))
    for n, sg in (("left", 1), ("right", -1)):
        o.append(rect(f"fender-rear-{n}", C, "detail", 2.3, 0.3, "chassis", PS, min(sg * 0.56, sg * 1.275), 0.628, 0.715, 0.512))
        o.append(rect(f"spare-chute-{n}", C, "detail", 2.35, 0.58, "chute", GSM, min(sg * 0.72, sg * 1.22), 1.165, 0.50, 0.18))
        o.append(rect(f"fender-2-{n}", C, "detail", 2.4, 0.3, "chassis", PS, min(sg * 0.84, sg * 1.24), 0.574, 0.40, 0.566))
        o.append(rect(f"tyre-3-{n}", "wheel-3", "body", 2.5, 0.12, ["tyres"], RBM, min(sg * 0.9375, sg * 1.2525), 0, K.TW, 2 * K.WR, hatch="tread"))
        o.append(rect(f"tyre-3-{n}-inner", "wheel-3", "body", 2.5, 0.12, ["tyres"], RBM, min(sg * 0.5875, sg * 0.9025), 0, K.TW, 2 * K.WR, hatch="tread"))
    o.append(ellipse("differential", "wheel-3", "detail", 2.45, 0.3, "chassis", PS, 0.06, K.WR, 0.20, 0.23))
    o.append(rect("bogie-saddle", C, "detail", 2.45, 0.32, "chassis", PS, -0.25, 0.30, 0.50, 0.40))
    o.append(rect("exhaust-stack", C, "body", 2.2, 0.2, "exhaust-stack", EXM, -1.012, 1.00, 0.124, 2.22))
    o.append(rect("stack-cap", C, "detail", 2.3, 0.25, "exhaust-stack", EXM, -1.012, 3.22, 0.124, 0.08))
    o.append(poly("air-intake", C, "detail", 2.2, 0.3, "chassis", PS, [(0.875, 1.30), (1.025, 1.30), (1.025, 2.95), (1.08, 2.95), (1.08, 3.05), (1.04, 3.09), (0.86, 3.09), (0.82, 3.05), (0.82, 2.95), (0.875, 2.95)], world=False))
    o.append(rect("bumper-frame", C, "body", 2.9, 0.4, "chassis", PS, -1.16, 0.42, 2.32, 0.53, r=0.06))
    o.append(poly("cab", C, "body", 3, 0.5, "cab", PS, [(-1.14, 0.95), (1.14, 0.95), (1.14, 2.02), (1.10, 2.90), (1.08, 2.98), (1.02, 3.05), (0.84, 3.10), (-0.84, 3.10), (-1.02, 3.05), (-1.08, 2.98), (-1.10, 2.90), (-1.14, 2.02)], world=False))
    o.append(rect("front-grille", C, "detail", 3.2, 0.2, "engine-bay", PS, -0.65, 1.28, 1.30, 0.56))
    o.append(rect("windscreen", C, "glazing", 3.5, 0.85, "cab-glazing", GLM, -0.95, 2.052, 1.90, 0.816))
    o.append(rect("sun-visor", C, "detail", 3.55, 0.25, "cab", PS, -1.0, 2.885, 2.0, 0.045))
    for n, sg in (("left", 1), ("right", -1)):
        o.append(rect(f"head-lamp-{n}", C, "detail", 3.3, 0.9, "cab-glazing", GLM, min(sg * 0.69, sg * 1.03), 0.615, 0.34, 0.17))
        o.append(poly(f"mirror-arm-{n}", C, "detail", 3.3, 0.2, "cab", PS, box4(*sorted([sg * 1.135, sg * 1.275]), 2.35, 2.77), world=False))
        o.append(rect(f"tyre-1-{n}", "wheel-1", "body", 4, 0.12, ["tyres"], RBM, min(sg * 0.8625, sg * 1.1775), 0, K.TW, 2 * K.WR, hatch="tread"))
    return o


def plan():
    o = []
    C, D = "chassis", "drum"
    sol_m = {"kind": "extruded", "sweep": "w", "z0": -K.CHUTE_HW, "z1": K.CHUTE_HW}
    sol_e = {"kind": "extruded", "sweep": "w", "z0": -K.EXT_HW, "z1": K.EXT_HW}
    o.append(rect("underrun-bar", C, "detail", 1.3, 0.36, "chassis", PS, -4.50, -1.20, 0.12, 2.40))
    o.append(rect("tail-lamp-beam", C, "detail", 1.3, 0.36, "chassis", PS, -4.43, -1.18, 0.05, 2.36))
    o.append(rect("frame", C, "body", 1.4, 0.4, "chassis", PS, K.SUB_X0, -0.39, K.RAIL_X1 - K.SUB_X0, 0.78))
    o.append(rect("pedestal", C, "body", 1.45, 0.5, "chassis", PS, -3.23, -0.71, 1.56, 1.42))
    o.append(rect("front-pedestal", C, "body", 1.45, 0.48, "chassis", PS, 1.50, -0.47, 0.98, 0.94))
    for n, sg in (("left", 1), ("right", -1)):
        o.append(rect(f"tyre-1-{n}", "wheel-1", "body", 1.5, 0.12, ["tyres"], RBM, K.AX[0] - K.WR, min(sg * 0.8625, sg * 1.1775), 2 * K.WR, K.TW, hatch="tread"))
        o.append(rect(f"tyre-2-{n}", "wheel-2", "body", 1.5, 0.12, ["tyres"], RBM, K.AX[1] - K.WR, min(sg * 0.8625, sg * 1.1775), 2 * K.WR, K.TW, hatch="tread"))
        for i in (2, 3):
            o.append(rect(f"tyre-{i + 1}-{n}", f"wheel-{i + 1}", "body", 1.5, 0.12, ["tyres"], RBM, K.AX[i] - K.WR, min(sg * 0.9375, sg * 1.2525), 2 * K.WR, K.TW, hatch="tread"))
            o.append(rect(f"tyre-{i + 1}-{n}-inner", f"wheel-{i + 1}", "body", 1.5, 0.12, ["tyres"], RBM, K.AX[i] - K.WR, min(sg * 0.5875, sg * 0.9025), 2 * K.WR, K.TW, hatch="tread"))
        o.append(rect(f"fender-2-{n}", C, "detail", 1.6, 0.3, "chassis", PS, K.AX[1] - 0.618, min(sg * 0.84, sg * 1.24), 1.236, 0.40))
        o.append(rect(f"fender-rear-{n}", C, "detail", 1.6, 0.3, "chassis", PS, K.AX[3] - 0.611, min(sg * 0.56, sg * 1.275), K.AX[2] - K.AX[3] + 1.222, 0.715))
        o.append(rect(f"spare-chute-{n}", C, "detail", 1.7, 0.58, "chute", GSM, -2.60, min(sg * 0.72, sg * 1.22), 1.15, 0.50))
        o.append(rect(f"hinge-post-{n}", C, "body", 1.45, 0.5, "chassis", PS, -4.05, min(sg * 0.50, sg * 0.58), 0.65, 0.08))
        o.append(rect(f"tail-lamp-{n}", C, "detail", 1.35, 0.75, "chassis", GLM, -4.49, min(sg * 0.78, sg * 1.18), 0.06, 0.40))
        o.append(rect(f"fender-1-rear-{n}", C, "detail", 1.6, 0.3, "chassis", PS, K.AX[0] - 0.678, min(sg * 0.84, sg * 1.16), 0.223, 0.32))
    o.append(rect("fuel-tank", C, "detail", 1.55, 0.6, "chassis", GSM, -0.62, 0.49, 0.92, 0.62))
    o.append(rect("silencer", C, "detail", 1.55, 0.3, "chassis", PS, 1.88, -1.13, 0.42, 0.60))
    o.append(rect("air-filter", C, "detail", 1.55, 0.33, "chassis", PS, 1.88, 0.58, 0.42, 0.50))
    o.append(rect("platform", C, "detail", 1.7, 0.42, "chassis", PS, -4.46, 0.78, 0.52, 0.46))
    o.append(poly("rear-ladder", C, "detail", 1.75, 0.4, "chassis", PS, [(-4.20, 0.80), (-4.46, 0.80), (-4.46, 1.22), (-4.20, 1.22)], filled=False, closed=False, world=False))
    o.append(poly("discharge-funnel", C, "body", 1.8, 0.5, "chute", GSM, K.superellipse(-3.80, 0, 0.33, 0.36, 2.5, 16), world=False))
    o.append(poly("swivel-ring", "chute-swivel", "detail", 1.85, 0.4, "chute", GSM, box4(0.02, 0.08, 0, 0.24), world=False, solid=RING))
    o.append(poly("chute-main", "chute", "body", 3.9, 0.5, "chute", GSM, box4(0, K.CHUTE_L, 0, K.CHUTE_D), world=False, solid=sol_m))
    o.append(poly("chute-ext-folded", "chute-ext", "body", 4, 0.6, "chute", GSM, box4(0, K.EXT_L, 0, K.EXT_D), world=False, solid=sol_e))
    o.append(poly("bumper", C, "body", 1.9, 0.4, "chassis", PS, K.superellipse(4.28, 0, 0.22, 1.16, 8, 28), world=False))
    o.append(poly("cab", C, "body", 2, 0.5, "cab", PS, K.superellipse(3.53, 0, 0.95, 1.14, 12, 36), world=False))
    o.append(rect("windscreen", C, "glazing", 2.6, 0.85, "cab-glazing", GLM, 4.385, -0.95, 0.08, 1.90))
    for n, sg in (("left", 1), ("right", -1)):
        o.append(poly(f"mirror-arm-{n}", C, "detail", 2.5, 0.2, "cab", PS, box4(4.425, 4.475, *sorted([sg * 1.135, sg * 1.275])), world=False))
    o.append(rect("water-tank", C, "body", 2.3, 0.62, "chassis", GSM, 2.0, -0.80, 0.50, 1.60, r=0.06))
    o.append(ellipse("exhaust-stack", C, "body", 2.4, 0.2, "exhaust-stack", EXM, K.STACK[0], K.STACK[1], 0.09, 0.09))
    o.append(ellipse("stack-cap", C, "detail", 2.45, 0.25, "exhaust-stack", EXM, K.STACK[0] - 0.02, K.STACK[1], 0.062, 0.062, filled=False))
    o.append(ellipse("air-intake", C, "detail", 2.4, 0.3, "chassis", PS, K.INTAKE[0], K.INTAKE[1], 0.13, 0.13))
    o.append(poly("drive-housing", D, "body", 2.9, 0.35, "drum-drive", PS, [(0, 0), (-0.72, 0), (-0.72, 0.085), (-0.46, 0.105), (-0.46, 0.25), (-0.42, 0.33), (-0.12, 0.33), (-0.10, 0.39), (0.02, 0.39), (0.02, 0)], world=False, solid=REV))
    o.append(poly("drum-belly", D, "body", 3, 0.3, "drum", DPM, DRUM_HALF, world=False, solid=REV))
    for k, s in enumerate(FLIGHT_S):
        x = K.DRUM_O[0] - s * 0.9816
        o.append(poly(f"flight-{k + 1}", "drum-spiral", "detail", 3.4, 0.75, "drum", DPM, [(x + 0.41, 0.5), (x - 0.41, -0.5), (x - 0.57, -0.5), (x + 0.25, 0.5)], filled=False, world=False))
    o.append(poly("drum-seam", D, "detail", 3.7, 0.3, "drum", DPM, [(K.DRUM_O[0] - 1.9 * 0.9816, -1.15), (K.DRUM_O[0] - 1.9 * 0.9816, 1.15)], filled=False, closed=False, world=False))
    o.append(poly("hopper", D, "body", 3.8, 0.55, "hopper", GSM, K.superellipse(-3.86, 0, 0.37, 0.50, 4, 24), world=False))
    return o


DECISIONS = None
