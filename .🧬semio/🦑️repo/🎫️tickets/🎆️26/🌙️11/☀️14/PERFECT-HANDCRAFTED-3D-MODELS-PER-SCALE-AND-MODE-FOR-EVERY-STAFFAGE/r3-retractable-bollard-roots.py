"""🪛 Round-3 root fixes of `objects/retractable-bollard` outside `volumes`: the rig (three vertical bones), anchors, poses, bone sections, the 2D drawings of every scale and view redrawn by hand as the true views of the handcrafted body (ruling E1), the true-extent symbol and the corrected thermal areas.

Usage: python3 r3-retractable-bollard-roots.py <out.json>     candidate document with the kit's volumes, for `r3-validate.ts --file`
       python3 r3-retractable-bollard-roots.py --write        re-reads the catalog file and replaces only the changed members
@see ./r3-retractable-bollard-kit.py
@see ./r3-objects-retractable-bollard.md
"""
import importlib.util
import json
import math
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/retractable-bollard/🔣️.json"
_spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "r3-retractable-bollard-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)
r6, su, text = K.r6, K.su, K.text

AREAS = {"cover-frame": 0.13, "hydraulic-unit": 0.58}
MEMBERS = ("materials", "rig", "anchors", "representations", "symbols", "poses", "thermal")
STEEL, FILM, PC, IRON, GALV, HYD = K.STEEL, K.FILM, K.PC, K.IRON, K.GALV, K.HYD
SHAFT, BAND, LED, FRAME, SLEEVE, UNIT = "shaft-steel", "reflector-band", "beacon-led", "cover-frame", "sleeve-steel", "hydraulic-unit"


def shape(sid, bone, role, z, filled, tone, geometry, region=None, material=None, **more):
    """✏️ One shape with the member order of the catalog."""
    out = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    out.update(geometry)
    if isinstance(region, list):
        out["regions"] = region
    elif region:
        out["region"] = region
    if material:
        out["material"] = material
    out.update(more)
    return out


def e_rect(bone, x0, x1, y0, y1):
    """📐 Elevation rectangle from world X and Y into the vertical bone frame (x up, y = -X)."""
    return {"kind": "rect", "x": r6(y0 - K.ORIGIN[bone]), "y": r6(-x1), "w": r6(y1 - y0), "h": r6(x1 - x0)}


def e_poly(bone, points, closed=True):
    """📐 Elevation polygon from world (X, Y)."""
    return {"kind": "poly", "points": [[r6(y - K.ORIGIN[bone]), r6(-x)] for x, y in points], "closed": closed}


def e_path(bone, commands):
    """📐 Elevation path from world (X, Y) commands."""
    parts = []
    for command in commands:
        parts.append(command[0])
        for x, y in command[1:]:
            parts.append(f"{r6(y - K.ORIGIN[bone])} {r6(-x)}")
    return {"kind": "path", "d": " ".join(parts)}


def rect(x0, x1, y0, y1, r=None):
    """▭ Front (Z, Y) or plan (X, Z) rectangle."""
    out = {"kind": "rect", "x": r6(x0), "y": r6(y0), "w": r6(x1 - x0), "h": r6(y1 - y0)}
    if r:
        out["r"] = r
    return out


def poly(points, closed=True):
    """⬠ Front or plan polygon."""
    return {"kind": "poly", "points": [[r6(x), r6(y)] for x, y in points], "closed": closed}


def path(commands):
    """➰ Front or plan path."""
    parts = []
    for command in commands:
        parts.append(command[0])
        for x, y in command[1:]:
            parts.append(f"{r6(x)} {r6(y)}")
    return {"kind": "path", "d": " ".join(parts)}


def ellipse(cx, cy, rx, ry=None):
    """⚪ Front or plan ellipse."""
    return {"kind": "ellipse", "cx": r6(cx), "cy": r6(cy), "rx": r6(rx), "ry": r6(rx if ry is None else ry)}


def crown(y0):
    """🛎 The crown outline from the height y0 on both sides over the dome, in (lateral, Y) of the rest pose."""
    return [("M", (-0.1, y0)), ("L", (-0.1, su(1.1))), ("C", (-0.1, su(1.135)), (-0.06, su(1.15)), (0, su(1.15))), ("C", (0.06, su(1.15)), (0.1, su(1.135)), (0.1, su(1.1))), ("L", (0.1, y0)), ("Z",)]


def sleeve_u(wall, floor, top):
    """🇺 The cut sleeve wall and floor as one U polygon in world (X, Y)."""
    a, b = 0.15, 0.15 - wall
    return [(-a, top), (-a, -1.25), (a, -1.25), (a, top), (b, top), (b, -1.25 + floor), (-b, -1.25 + floor), (-b, top)]


def frame_cut(side, splay):
    """🧱 The cut cover frame on one side (side = 1 ahead, -1 behind): collar and plate, with the splayed edges at 1:50."""
    pts = [(0.142, -0.01), (0.15, -0.01), (0.15, -0.07), (0.168, -0.07), (0.168, -0.01), (0.2, -0.01)]
    pts += [(0.2, 0.0), (0.194, 0.015), (0.1377, 0.015), (0.142, 0.0)] if splay else [(0.2, 0.015), (0.142, 0.015)]
    return [(side * x, y) for x, y in pts]


def pair(sid, bone, role, z, tone, x0, x1, y0, y1, region=None, material=None, **more):
    """👯 The same cut rectangle behind (-l) and ahead (-r) of the axis in the elevation."""
    return [
        shape(f"{sid}-l", bone, role, z, True, tone, e_rect(bone, -x1, -x0, y0, y1), region, material, **more),
        shape(f"{sid}-r", bone, role, z, True, tone, e_rect(bone, x0, x1, y0, y1), region, material, **more),
    ]


def elevation50():
    """📏 Section A-A at 1:50: the half behind the axis plane, cut faces dark, the far inner faces light."""
    s, h, f = "sleeve", "hydraulic", "shaft"
    out = [
        shape("sleeve-bore", s, "body", 2.0, True, 0.9, e_rect(s, -0.142, 0.142, -1.242, -0.01), SLEEVE),
        shape("sleeve", s, "body", 2.2, True, 0.3, e_poly(s, sleeve_u(0.008, 0.008, -0.01)), SLEEVE, GALV, penMm=0.13),
        *pair("tab-upper", s, "detail", 2.3, 0.3, 0.15, 0.19, -0.34, -0.26, SLEEVE),
        *pair("tab-lower", s, "detail", 2.3, 0.3, 0.15, 0.19, -1.14, -1.06, SLEEVE),
        shape("buffer-far", s, "detail", 2.3, True, 0.3, e_rect(s, 0.0385, 0.0785, -1.242, -1.15)),
        shape("buffer-cut", s, "detail", 2.4, True, 0.15, e_rect(s, -0.137, -0.097, -1.242, -1.15)),
        shape("frame-bore", s, "body", 2.5, True, 0.75, e_rect(s, -0.142, 0.142, -0.01, 0.015), FRAME),
        *pair("bushing", s, "detail", 2.6, 0.5, 0.102, 0.142, -0.07, -0.01, SLEEVE),
        shape("frame-l", s, "body", 3, True, 0.6, e_poly(s, frame_cut(-1, True)), FRAME, IRON),
        shape("frame-r", s, "body", 3, True, 0.6, e_poly(s, frame_cut(1, True)), FRAME, IRON),
        *pair("seal", s, "detail", 3.1, 0.5, 0.112, 0.137, 0.005, 0.015, FRAME),
        *pair("wiper", s, "detail", 3.1, 0.1, 0.1005, 0.112, -0.008, 0.015, FRAME),
        *pair("frame-bolt", s, "detail", 3.1, 0.2, 0.159, 0.181, 0.015, 0.023, FRAME),
        shape("shaft-flange", f, "detail", 3.45, True, 0.3, e_rect(f, -0.14, 0.14, su(0), su(0.012)), SHAFT),
        shape("shaft", f, "body", 3.5, True, 0.5, e_rect(f, -0.1, 0.1, su(0.012), su(1.02)), SHAFT, STEEL),
        *pair("reflector", f, "body", 3.55, 0.92, 0.1, 0.1005, su(0.91), su(1.02), BAND, FILM),
        shape("lens-far", f, "body", 3.58, True, 0.8, e_rect(f, -0.097, 0.097, su(1.02), su(1.08)), LED),
        *pair("lens", f, "body", 3.6, 0.7, 0.097, 0.1, su(1.02), su(1.08), LED, PC),
        *pair("led", f, "detail", 3.62, 0.1, 0.088, 0.1005, su(1.04), su(1.06), LED),
        shape("head", f, "body", 3.65, True, 0.35, e_path(f, [("M", (-0.095, su(0.99))), ("L", (-0.095, su(1.02))), ("L", (-0.085, su(1.02))), ("L", (-0.085, su(1.08))), ("L", (-0.1, su(1.08))), ("L", (-0.1, su(1.1))), ("C", (-0.1, su(1.135)), (-0.06, su(1.15)), (0, su(1.15))), ("C", (0.06, su(1.15)), (0.1, su(1.135)), (0.1, su(1.1))), ("L", (0.1, su(1.08))), ("L", (0.085, su(1.08))), ("L", (0.085, su(1.02))), ("L", (0.095, su(1.02))), ("L", (0.095, su(0.99))), ("Z",)]), SHAFT, STEEL),
        shape("crosshead", f, "detail", 3.7, True, 0.35, e_poly(f, [(-0.095, su(0.952)), (-0.095, su(0.94)), (-0.03, su(0.94)), (-0.03, su(0.925)), (0.03, su(0.925)), (0.03, su(0.94)), (0.095, su(0.94)), (0.095, su(0.952))]), SHAFT),
        shape("rod-nut", f, "detail", 3.7, True, 0.3, e_rect(f, -0.0173, 0.0173, su(0.952), su(0.97)), UNIT),
        shape("rod", f, "body", 3.85, True, 0.85, e_rect(f, -0.018, 0.018, su(0.17), su(0.925)), UNIT, HYD),
        shape("piston", f, "detail", 3.85, True, 0.3, e_rect(f, -0.03, 0.03, su(0.13), su(0.17)), UNIT),
        shape("bracket-upper", h, "detail", 2.9, True, 0.3, e_rect(h, 0.15, 0.18, -0.604, -0.596)),
        shape("bracket-lower", h, "detail", 2.9, True, 0.3, e_rect(h, 0.15, 0.18, -0.904, -0.896)),
        shape("conduit", h, "detail", 2.9, True, 0.35, e_rect(h, 0.15, 0.181, -0.7625, -0.7375), UNIT),
        shape("power-unit", h, "body", 3, True, 0.5, e_rect(h, 0.18, 0.35, -0.95, -0.565), UNIT, HYD),
        shape("unit-lid", h, "body", 3.1, True, 0.4, e_rect(h, 0.17, 0.36, -0.565, -0.55), UNIT, HYD),
        shape("unit-lid-bolt-0", h, "detail", 3.2, True, 0.2, e_rect(h, 0.1775, 0.1925, -0.55, -0.542), UNIT),
        shape("unit-lid-bolt-1", h, "detail", 3.2, True, 0.2, e_rect(h, 0.3375, 0.3525, -0.55, -0.542), UNIT),
        shape("cable-gland", h, "detail", 3.2, True, 0.25, e_poly(h, [(0.284, -0.55), (0.316, -0.55), (0.316, -0.535), (0.312, -0.535), (0.312, -0.52), (0.288, -0.52), (0.288, -0.535), (0.284, -0.535)]), UNIT),
        shape("cable", h, "detail", 3.2, False, 0.3, e_poly(h, [(0.3, -0.52), (0.3, -0.49), (0.292, -0.465), (0.27, -0.45), (0.24, -0.447)], False), UNIT, penMm=0.24),
        shape("hose", h, "detail", 3.3, False, 0.3, e_poly(h, [(x, y) for x, y, _ in K.HOSE], False), UNIT, penMm=0.32),
        shape("hose-grommet", h, "detail", 3.4, True, 0.15, e_rect(h, 0.136, 0.158, -1.235, -1.191), UNIT),
        shape("hose-union", h, "detail", 3.4, True, 0.3, e_rect(h, 0.251, 0.279, -0.972, -0.95), UNIT),
        shape("cylinder-base", h, "body", 3.6, True, 0.4, e_poly(h, [(-0.06, -1.242), (0.06, -1.242), (0.06, -1.23), (0.04, -1.23), (0.04, -1.197), (-0.04, -1.197), (-0.04, -1.23), (-0.06, -1.23)]), UNIT, HYD),
        shape("cylinder", h, "body", 3.6, True, 0.7, e_rect(h, -0.035, 0.035, -1.197, -0.31), UNIT, HYD),
        shape("cylinder-bore", h, "detail", 3.62, True, 0.92, e_rect(h, -0.03, 0.03, -1.197, -0.31), UNIT),
        shape("hose-port", h, "detail", 3.7, True, 0.3, e_rect(h, 0.038, 0.062, -1.227, -1.199), UNIT),
        shape("cylinder-gland", h, "body", 3.7, True, 0.4, e_poly(h, [(-0.04, -0.31), (0.04, -0.31), (0.04, -0.278), (0.034, -0.27), (-0.034, -0.27), (-0.04, -0.278)]), UNIT, HYD),
    ]
    return out


def front50():
    """📏 Front view at 1:50: the outside of the bollard seen from ahead; the shaft slides behind the sleeve."""
    s, h, f = "sleeve", "hydraulic", "shaft"
    leds = []
    for k, a in enumerate((-60, -30, 0, 30, 60)):
        leds.append(shape(f"led-{k}", f, "detail", 2.2, True, 0.1, ellipse(0.0995 * math.sin(math.radians(a)), su(1.05), 0.01 * math.cos(math.radians(a)), 0.01), LED))
    return [
        shape("shaft", f, "body", 2.0, True, 0.5, rect(-0.1, 0.1, su(0.012), su(0.91)), SHAFT, STEEL),
        shape("shaft-highlight", f, "texture", 2.2, False, 0.3, poly([(-0.065, su(0.05)), (-0.065, su(0.87))], False), SHAFT),
        shape("reflector", f, "body", 2.05, True, 0.92, rect(-0.1005, 0.1005, su(0.91), su(1.02)), BAND, FILM, hatch="chevron"),
        shape("beacon", f, "body", 2.1, True, 0.7, rect(-0.1, 0.1, su(1.02), su(1.08)), LED, PC),
        *leds,
        shape("cap", f, "body", 2.15, True, 0.5, path(crown(su(1.08))), SHAFT, STEEL),
        shape("cap-seam", f, "texture", 2.3, False, 0.3, poly([(-0.0975, su(1.0985)), (0.0975, su(1.0985))], False), SHAFT),
        shape("sleeve", s, "body", 3.0, True, 0.4, rect(-0.15, 0.15, -1.25, -0.07), SLEEVE, GALV),
        shape("tab-upper-l", s, "detail", 3.0, True, 0.3, rect(-0.19, -0.15, -0.34, -0.26), SLEEVE),
        shape("tab-upper-r", s, "detail", 3.0, True, 0.3, rect(0.15, 0.19, -0.34, -0.26), SLEEVE),
        shape("tab-lower-l", s, "detail", 3.0, True, 0.3, rect(-0.19, -0.15, -1.14, -1.06), SLEEVE),
        shape("tab-lower-r", s, "detail", 3.0, True, 0.3, rect(0.15, 0.19, -1.14, -1.06), SLEEVE),
        shape("tab-upper-front", s, "detail", 3.1, True, 0.3, rect(-0.004, 0.004, -0.34, -0.26), SLEEVE),
        shape("tab-lower-front", s, "detail", 3.1, True, 0.3, rect(-0.004, 0.004, -1.14, -1.06), SLEEVE),
        shape("collar", s, "body", 3.0, True, 0.6, rect(-0.168, 0.168, -0.07, -0.01), FRAME, IRON),
        shape("frame", s, "body", 3.2, True, 0.6, poly([(-0.2, -0.01), (0.2, -0.01), (0.2, 0.0), (0.194, 0.015), (-0.194, 0.015), (-0.2, 0.0)]), FRAME, IRON),
        shape("frame-bolt-0", s, "detail", 3.3, True, 0.2, rect(-0.1795, -0.1605, 0.015, 0.023), FRAME),
        shape("frame-bolt-1", s, "detail", 3.3, True, 0.2, rect(0.1605, 0.1795, 0.015, 0.023), FRAME),
        shape("hose-grommet", h, "detail", 3.3, True, 0.15, ellipse(0, -1.213, 0.022), UNIT),
        shape("hose", h, "detail", 3.4, False, 0.3, poly([(z, y) for _, y, z in K.HOSE[1:]], False), UNIT, penMm=0.32),
        shape("hose-union", h, "detail", 4.0, True, 0.3, rect(0.092, 0.12, -0.972, -0.95), UNIT),
        shape("power-unit", h, "body", 4.0, True, 0.5, rect(0.022, 0.19, -0.95, -0.565), UNIT, HYD),
        shape("unit-lid", h, "body", 4.1, True, 0.4, rect(0.012, 0.2, -0.565, -0.55), UNIT, HYD),
        shape("unit-lid-bolt-0", h, "detail", 4.2, True, 0.2, rect(0.0205, 0.0335, -0.55, -0.542), UNIT),
        shape("unit-lid-bolt-1", h, "detail", 4.2, True, 0.2, rect(0.1785, 0.1915, -0.55, -0.542), UNIT),
        shape("cable-gland", h, "detail", 4.2, True, 0.25, poly([(0.09, -0.55), (0.122, -0.55), (0.122, -0.535), (0.118, -0.535), (0.118, -0.52), (0.094, -0.52), (0.094, -0.535), (0.09, -0.535)]), UNIT),
        shape("cable", h, "detail", 4.2, False, 0.3, poly([(0.106, -0.52), (0.106, -0.441)], False), UNIT, penMm=0.24),
    ]


def plan50():
    """📏 Plan at 1:50: seen from above, the buried unit beside the frame."""
    s, h, f = "sleeve", "hydraulic", "shaft"
    out = [
        shape("power-unit", h, "body", 2, True, 0.8, rect(0.17, 0.36, 0.012, 0.2, 0.012), UNIT, HYD),
        *[shape(f"unit-lid-bolt-{k}", h, "detail", 2.1, True, 0.3, ellipse(x, z, 0.0075), UNIT) for k, (x, z) in enumerate(((0.185, 0.027), (0.345, 0.027), (0.185, 0.185), (0.345, 0.185)))],
        shape("cable-gland", h, "detail", 2.1, True, 0.4, ellipse(0.3, 0.106, 0.016), UNIT),
        shape("cable", h, "detail", 2.2, False, 0.3, poly([(0.3, 0.106), (0.234, 0.106)], False), UNIT, penMm=0.24),
        shape("plan-frame", s, "body", 3, True, 0.6, rect(-0.2, 0.2, -0.2, 0.2, 0.02), FRAME, IRON),
        shape("plan-frame-edge", s, "texture", 3.05, False, 0.3, rect(-0.194, 0.194, -0.194, 0.194, 0.0194), FRAME),
        *[shape(f"frame-bolt-{k}", s, "detail", 3.1, True, 0.2, ellipse(x, z, 0.0105), FRAME) for k, (x, z) in enumerate(((-0.17, -0.17), (0.17, -0.17), (-0.17, 0.17), (0.17, 0.17)))],
        shape("plan-retainer", s, "detail", 3.2, True, 0.5, ellipse(0, 0, 0.137), FRAME),
        shape("plan-wiper", s, "detail", 3.3, True, 0.1, ellipse(0, 0, 0.112), FRAME),
        shape("plan-cap", f, "body", 3.6, True, 0.5, ellipse(0, 0, 0.1005), SHAFT, STEEL),
        *[shape(f"cap-screw-{k}", f, "detail", 3.7, True, 0.2, ellipse(x, z, 0.008), SHAFT) for k, (x, z) in enumerate(((-0.05, 0.0), (0.025, 0.0433), (0.025, -0.0433)))],
    ]
    return out


def elevation100():
    """📏 Section A-A at 1:100."""
    s, h, f = "sleeve", "hydraulic", "shaft"
    return [
        shape("sleeve-bore", s, "body", 2.0, True, 0.9, e_rect(s, -0.142, 0.142, -1.235, -0.01), SLEEVE),
        shape("sleeve", s, "body", 2.2, True, 0.3, e_poly(s, sleeve_u(0.008, 0.015, -0.01)), SLEEVE, GALV, penMm=0.13),
        *pair("tab-upper", s, "detail", 2.3, 0.3, 0.15, 0.19, -0.34, -0.26, SLEEVE),
        *pair("tab-lower", s, "detail", 2.3, 0.3, 0.15, 0.19, -1.14, -1.06, SLEEVE),
        shape("frame-bore", s, "body", 2.5, True, 0.75, e_rect(s, -0.142, 0.142, -0.01, 0.015), FRAME),
        shape("frame-l", s, "body", 3, True, 0.6, e_poly(s, frame_cut(-1, False)), FRAME, IRON),
        shape("frame-r", s, "body", 3, True, 0.6, e_poly(s, frame_cut(1, False)), FRAME, IRON),
        *pair("seal", s, "detail", 3.1, 0.2, 0.1005, 0.142, 0.0, 0.015, FRAME),
        shape("shaft-flange", f, "detail", 3.45, True, 0.3, e_rect(f, -0.14, 0.14, su(0), su(0.015)), SHAFT),
        shape("shaft", f, "body", 3.5, True, 0.5, e_rect(f, -0.1, 0.1, su(0.015), su(1.02)), SHAFT, STEEL),
        *pair("reflector", f, "body", 3.55, 0.92, 0.1, 0.1005, su(0.91), su(1.02), BAND, FILM),
        *pair("lens", f, "body", 3.6, 0.7, 0.097, 0.1, su(1.02), su(1.08), LED, PC),
        shape("head", f, "body", 3.65, True, 0.35, e_path(f, [("M", (-0.095, su(0.99))), ("L", (-0.095, su(1.02))), ("L", (-0.097, su(1.02))), ("L", (-0.097, su(1.08))), ("L", (-0.1, su(1.08))), ("L", (-0.1, su(1.1))), ("C", (-0.1, su(1.135)), (-0.06, su(1.15)), (0, su(1.15))), ("C", (0.06, su(1.15)), (0.1, su(1.135)), (0.1, su(1.1))), ("L", (0.1, su(1.08))), ("L", (0.097, su(1.08))), ("L", (0.097, su(1.02))), ("L", (0.095, su(1.02))), ("L", (0.095, su(0.99))), ("Z",)]), SHAFT, STEEL),
        shape("rod", f, "body", 3.85, True, 0.85, e_rect(f, -0.018, 0.018, su(0.17), su(0.99)), UNIT, HYD),
        shape("piston", f, "detail", 3.85, True, 0.3, e_rect(f, -0.03, 0.03, su(0.13), su(0.17)), UNIT),
        shape("bracket-upper", h, "detail", 2.9, True, 0.3, e_rect(h, 0.15, 0.18, -0.6075, -0.5925)),
        shape("bracket-lower", h, "detail", 2.9, True, 0.3, e_rect(h, 0.15, 0.18, -0.9075, -0.8925)),
        shape("power-unit", h, "body", 3, True, 0.5, e_rect(h, 0.17, 0.36, -0.95, -0.55), UNIT, HYD),
        shape("hose", h, "detail", 3.3, False, 0.3, e_poly(h, [(x, y) for x, y, _ in K.HOSE], False), UNIT, penMm=0.16),
        shape("cylinder-base", h, "body", 3.6, True, 0.4, e_rect(h, -0.045, 0.045, -1.235, -1.197), UNIT, HYD),
        shape("cylinder", h, "body", 3.6, True, 0.7, e_rect(h, -0.035, 0.035, -1.197, -0.31), UNIT, HYD),
        shape("cylinder-gland", h, "body", 3.7, True, 0.4, e_rect(h, -0.04, 0.04, -0.31, -0.27), UNIT, HYD),
    ]


def front100():
    """📏 Front view at 1:100."""
    s, h, f = "sleeve", "hydraulic", "shaft"
    return [
        shape("shaft", f, "body", 2.0, True, 0.5, rect(-0.1, 0.1, su(0.015), su(0.91)), SHAFT, STEEL),
        shape("reflector", f, "body", 2.05, True, 0.92, rect(-0.1005, 0.1005, su(0.91), su(1.02)), BAND, FILM),
        shape("beacon", f, "body", 2.1, True, 0.7, rect(-0.1, 0.1, su(1.02), su(1.08)), LED, PC),
        shape("cap", f, "body", 2.15, True, 0.5, path(crown(su(1.08))), SHAFT, STEEL),
        shape("sleeve", s, "body", 3.0, True, 0.4, rect(-0.15, 0.15, -1.25, -0.07), SLEEVE, GALV),
        shape("tab-upper-l", s, "detail", 3.0, True, 0.3, rect(-0.19, -0.15, -0.34, -0.26), SLEEVE),
        shape("tab-upper-r", s, "detail", 3.0, True, 0.3, rect(0.15, 0.19, -0.34, -0.26), SLEEVE),
        shape("tab-lower-l", s, "detail", 3.0, True, 0.3, rect(-0.19, -0.15, -1.14, -1.06), SLEEVE),
        shape("tab-lower-r", s, "detail", 3.0, True, 0.3, rect(0.15, 0.19, -1.14, -1.06), SLEEVE),
        shape("tab-upper-front", s, "detail", 3.1, True, 0.3, rect(-0.0075, 0.0075, -0.34, -0.26), SLEEVE),
        shape("tab-lower-front", s, "detail", 3.1, True, 0.3, rect(-0.0075, 0.0075, -1.14, -1.06), SLEEVE),
        shape("collar", s, "body", 3.0, True, 0.6, rect(-0.168, 0.168, -0.07, -0.01), FRAME, IRON),
        shape("frame", s, "body", 3.2, True, 0.6, rect(-0.2, 0.2, -0.01, 0.015), FRAME, IRON),
        shape("hose", h, "detail", 3.4, False, 0.3, poly([(z, y) for _, y, z in K.HOSE[1:]], False), UNIT, penMm=0.16),
        shape("power-unit", h, "body", 4.0, True, 0.5, rect(0.012, 0.2, -0.95, -0.55), UNIT, HYD),
    ]


def plan100():
    """📏 Plan at 1:100."""
    s, h, f = "sleeve", "hydraulic", "shaft"
    return [
        shape("power-unit", h, "body", 2, True, 0.8, rect(0.17, 0.36, 0.012, 0.2, 0.012), UNIT, HYD),
        shape("plan-frame", s, "body", 3, True, 0.6, rect(-0.2, 0.2, -0.2, 0.2, 0.02), FRAME, IRON),
        shape("plan-seal", s, "detail", 3.2, True, 0.2, ellipse(0, 0, 0.142), FRAME),
        shape("plan-cap", f, "body", 3.6, True, 0.5, ellipse(0, 0, 0.1005), SHAFT, STEEL),
    ]


def elevation200():
    """📏 Section A-A at 1:200."""
    s, h, f = "sleeve", "hydraulic", "shaft"
    return [
        shape("sleeve-bore", s, "body", 2.0, True, 0.9, e_rect(s, -0.135, 0.135, -1.22, -0.03), SLEEVE),
        shape("sleeve", s, "body", 2.2, True, 0.3, e_poly(s, sleeve_u(0.015, 0.03, -0.03)), SLEEVE, GALV, penMm=0.13),
        shape("frame-bore", s, "body", 2.5, True, 0.75, e_rect(s, -0.135, 0.135, -0.03, 0.015), FRAME),
        shape("frame-l", s, "body", 3, True, 0.6, e_rect(s, -0.2, -0.135, -0.03, 0.015), FRAME, IRON),
        shape("frame-r", s, "body", 3, True, 0.6, e_rect(s, 0.135, 0.2, -0.03, 0.015), FRAME, IRON),
        shape("shaft", f, "body", 3.5, True, 0.5, e_rect(f, -0.1, 0.1, su(0), su(0.91)), SHAFT, STEEL),
        shape("reflector", f, "body", 3.55, True, 0.92, e_rect(f, -0.1, 0.1, su(0.91), su(1.02)), BAND, FILM),
        shape("cap", f, "body", 3.6, True, 0.4, e_path(f, crown(su(1.02))), [SHAFT, LED], STEEL),
        shape("rod", f, "body", 3.85, True, 0.85, e_rect(f, -0.018, 0.018, su(0.17), su(1.02)), UNIT, HYD),
        shape("bracket", h, "body", 2.9, True, 0.3, e_rect(h, 0.15, 0.2, -0.9, -0.6), penMm=0.2),
        shape("power-unit", h, "body", 3, True, 0.5, e_rect(h, 0.17, 0.36, -0.95, -0.55), UNIT, HYD),
        shape("cylinder", h, "body", 3.6, True, 0.7, e_rect(h, -0.035, 0.035, -1.22, -0.27), UNIT, HYD),
    ]


def front200():
    """📏 Front view at 1:200."""
    s, h, f = "sleeve", "hydraulic", "shaft"
    return [
        shape("shaft", f, "body", 2.0, True, 0.5, rect(-0.1, 0.1, su(0), su(0.91)), SHAFT, STEEL),
        shape("reflector", f, "body", 2.05, True, 0.92, rect(-0.1, 0.1, su(0.91), su(1.02)), BAND, FILM),
        shape("cap", f, "body", 2.15, True, 0.5, path(crown(su(1.02))), [SHAFT, LED], STEEL),
        shape("sleeve", s, "body", 3.0, True, 0.4, rect(-0.15, 0.15, -1.25, -0.03), SLEEVE, GALV),
        shape("frame", s, "body", 3.2, True, 0.6, rect(-0.2, 0.2, -0.03, 0.015), FRAME, IRON),
        shape("power-unit", h, "body", 4.0, True, 0.5, rect(0.012, 0.2, -0.95, -0.55), UNIT, HYD),
    ]


def plan200():
    """📏 Plan at 1:200."""
    s, h, f = "sleeve", "hydraulic", "shaft"
    return [
        shape("power-unit", h, "body", 2, True, 0.8, rect(0.17, 0.36, 0.012, 0.2), UNIT, HYD),
        shape("plan-frame", s, "body", 3, True, 0.6, rect(-0.2, 0.2, -0.2, 0.2, 0.02), FRAME, IRON),
        shape("plan-cap", f, "body", 3.6, True, 0.5, ellipse(0, 0, 0.1), [SHAFT, LED], STEEL),
    ]


def cap500():
    """🛎 The faceted cap of 1:500 in (lateral, Y)."""
    return [(-0.1, su(1.02)), (-0.1, su(1.1)), (-0.085, su(1.135)), (-0.04, su(1.15)), (0.04, su(1.15)), (0.085, su(1.135)), (0.1, su(1.1)), (0.1, su(1.02))]


def elevation500():
    """📏 Section A-A at 1:500."""
    s, h, f = "sleeve", "hydraulic", "shaft"
    return [
        shape("sleeve-bore", s, "body", 2.0, True, 0.9, e_rect(s, -0.13, 0.13, -1.175, -0.06), SLEEVE),
        shape("sleeve", s, "body", 2.2, True, 0.3, e_poly(s, sleeve_u(0.02, 0.075, -0.06)), SLEEVE, GALV),
        shape("frame-bore", s, "body", 2.5, True, 0.75, e_rect(s, -0.13, 0.13, -0.06, 0.015), FRAME),
        shape("frame-l", s, "body", 3, True, 0.6, e_rect(s, -0.2, -0.13, -0.06, 0.015), FRAME, IRON),
        shape("frame-r", s, "body", 3, True, 0.6, e_rect(s, 0.13, 0.2, -0.06, 0.015), FRAME, IRON),
        shape("shaft", f, "body", 3.5, True, 0.5, e_rect(f, -0.1, 0.1, su(0), su(1.02)), [SHAFT, BAND], STEEL),
        shape("cap", f, "body", 3.6, True, 0.4, e_poly(f, cap500()), [SHAFT, LED], STEEL),
        shape("power-unit", h, "body", 3.1, True, 0.5, e_rect(h, 0.14, 0.36, -0.95, -0.55), UNIT, HYD),
    ]


def front500():
    """📏 Front view at 1:500."""
    s, h, f = "sleeve", "hydraulic", "shaft"
    return [
        shape("shaft", f, "body", 2.0, True, 0.5, rect(-0.1, 0.1, su(0), su(1.02)), [SHAFT, BAND], STEEL),
        shape("cap", f, "body", 2.15, True, 0.5, poly(cap500()), [SHAFT, LED], STEEL),
        shape("sleeve", s, "body", 3.0, True, 0.4, rect(-0.15, 0.15, -1.25, -0.06), SLEEVE, GALV),
        shape("frame", s, "body", 3.2, True, 0.6, rect(-0.2, 0.2, -0.06, 0.015), FRAME, IRON),
        shape("power-unit", h, "body", 4.0, True, 0.5, rect(0.012, 0.2, -0.95, -0.55), UNIT, HYD),
    ]


def plan500():
    """📏 Plan at 1:500."""
    s, h, f = "sleeve", "hydraulic", "shaft"
    return [
        shape("power-unit", h, "body", 2, True, 0.8, rect(0.14, 0.36, 0.012, 0.2), UNIT, HYD),
        shape("plan-frame", s, "body", 3, True, 0.6, rect(-0.2, 0.2, -0.2, 0.2, 0.02), FRAME, IRON),
        shape("plan-cap", f, "body", 3.6, True, 0.5, ellipse(0, 0, 0.1), [SHAFT, LED], STEEL),
    ]


GEOMETRY = {"50": (elevation50, front50, plan50), "100": (elevation100, front100, plan100), "200": (elevation200, front200, plan200), "500": (elevation500, front500, plan500)}

REP_DECISIONS = {
    "50": {
        "silhouette": text(
            "Side view: section A-A through the axis, 11 mm wide and 38 mm high. Cut faces are dark: the 8 mm sleeve wall and floor, the cast frame with collar and splayed plate, guide bushing, retainer and wiper, the anchor tabs, the head casting with the lens ring, the crosshead. Inside the open shaft stand the cylinder with its foot flange, bore and gland, the piston and the rod up to the crosshead nut; behind the plane hang the power unit with lid, bolts, cable gland and conduit on its two bracket plates and the hose from the cylinder port through the grommet to the union. Front view: the outside seen from ahead, the shaft with highlight, chevron band, lens ring with five of the twelve LED windows and the crown with its shadow gap rising out of the frame, the sleeve with its tabs and collar below the ground, the power unit in front of it. Plan: the 0.40 m frame with splayed edge and four bolts, retainer, wiper, the cap with three screws and the lid of the buried unit.",
            "Seitenansicht: Schnitt A-A durch die Achse, 11 mm breit und 38 mm hoch. Schnittflächen sind dunkel: Hülsenwand und Boden von 8 mm, der Gussrahmen mit Kragen und geneigter Platte, Führungsbuchse, Haltering und Abstreifer, die Ankerlaschen, das Kopfgussteil mit dem Linsenring, die Traverse. Im offenen Schaft stehen der Zylinder mit Fußflansch, Bohrung und Stopfbuchse, der Kolben und die Stange bis zur Traversenmutter; hinter der Ebene hängen die Antriebseinheit mit Deckel, Schrauben, Kabelverschraubung und Schutzrohr an zwei Konsolblechen und der Schlauch vom Zylinderanschluss durch die Tülle zur Verschraubung. Vorderansicht: das Äußere von vorn, der Schaft mit Lichtkante, Chevronband, Linsenring mit fünf der zwölf LED-Fenster und der Kalotte mit Schattenfuge über dem Rahmen, darunter im Boden die Hülse mit Laschen und Kragen, davor die Antriebseinheit. Draufsicht: der Rahmen von 0,40 m mit geneigter Kante und vier Schrauben, Haltering, Abstreifer, die Kappe mit drei Schrauben und der Deckel der eingegrabenen Einheit."),
        "retained": text(
            "Every part at true size and position; shaft, head, rod and piston ride on the prismatic shaft bone, so the piston travels inside the fixed cylinder and the shaft slides behind the sleeve in the front view.",
            "Alle Teile in wahrer Größe und Lage; Schaft, Kopf, Stange und Kolben fahren mit dem Schubknochen des Schafts, der Kolben läuft also im festen Zylinder und der Schaft gleitet in der Vorderansicht hinter die Hülse."),
        "merged": text(
            "The 5 mm shaft wall is the outline of the shaft; the two cut halves of the grommet are one block; the LED carrier is part of the head casting.",
            "Die Schaftwand von 5 mm ist die Kontur des Schafts; die zwei geschnittenen Hälften der Tülle sind ein Block; der LED-Träger ist Teil des Kopfgussteils."),
        "omitted": text(
            "The spiral cable of the beacon, valves, motor and tank inside the sealed unit, the seals of the piston, the drain holes in the section and the buffer in front of the plane.",
            "Das Spiralkabel der Leuchte, Ventile, Motor und Tank in der dichten Einheit, die Kolbendichtungen, die Entwässerungslöcher im Schnitt und der Puffer vor der Ebene."),
        "thinElements": text(
            "The reflective film is cut 0.5 mm thick (0.01 mm) and shows as a line; the sleeve wall of 8 mm is 0.16 mm; hose 16 mm and cable 12 mm are strokes of their true width (0.32 and 0.24 mm).",
            "Die Reflexfolie ist 0,5 mm dick geschnitten (0,01 mm) und erscheint als Linie; die Hülsenwand von 8 mm misst 0,16 mm; Schlauch 16 mm und Kabel 12 mm sind Striche in wahrer Breite (0,32 und 0,24 mm)."),
        "hatching": text("Chevron hatch on the reflective band in the front view (45 degrees, 0.5 mm).", "Chevronschraffur auf dem Reflexband in der Vorderansicht (45 Grad, 0,5 mm)."),
        "styles": text(
            "Cut steel 0.3, cut casting 0.35, far faces 0.75 to 0.9, shaft 0.5, film 0.92, lens 0.7, frame 0.6, cylinder 0.7, rod 0.85, unit 0.5; body pen 0.35 mm, detail 0.18 mm, texture 0.13 mm.",
            "Geschnittener Stahl 0,3, geschnittener Guss 0,35, ferne Flächen 0,75 bis 0,9, Schaft 0,5, Folie 0,92, Linse 0,7, Rahmen 0,6, Zylinder 0,7, Stange 0,85, Einheit 0,5; Körperstift 0,35 mm, Detail 0,18 mm, Textur 0,13 mm."),
        "overlays": text(
            "Lens and LED windows carry beacon-led, the film reflector-band; cylinder, rod, piston, hose and unit carry hydraulic-unit, so the thermal view shows the pump heat in the unit and the cylinder.",
            "Linse und LED-Fenster tragen beacon-led, die Folie reflector-band; Zylinder, Stange, Kolben, Schlauch und Einheit tragen hydraulic-unit, die Thermalansicht zeigt so die Pumpenwärme in Einheit und Zylinder."),
    },
    "100": {
        "silhouette": text(
            "Side view: section A-A, 5.6 mm wide and 19 mm high: sleeve wall and floor, tabs, frame with collar, one seal block, the shaft with flange, film, lens and head casting, the rod and piston in the cylinder with foot and gland, the hose, the power unit on two bracket plates. Front view: the outside from ahead with shaft, white band, amber ring and crown over the frame, the sleeve with tabs and collar, the unit in front. Plan: frame, seal ring, cap and the buried unit.",
            "Seitenansicht: Schnitt A-A, 5,6 mm breit und 19 mm hoch: Hülsenwand und Boden, Laschen, Rahmen mit Kragen, ein Dichtungsblock, der Schaft mit Flansch, Folie, Linse und Kopfgussteil, Stange und Kolben im Zylinder mit Fuß und Stopfbuchse, der Schlauch, die Antriebseinheit an zwei Konsolblechen. Vorderansicht: das Äußere von vorn mit Schaft, weißem Band, bernsteinfarbenem Ring und Kalotte über dem Rahmen, die Hülse mit Laschen und Kragen, davor die Einheit. Draufsicht: Rahmen, Dichtungsring, Kappe und die eingegrabene Einheit."),
        "retained": text("All main parts at true size; rod and piston ride with the shaft.", "Alle Hauptteile in wahrer Größe; Stange und Kolben fahren mit dem Schaft."),
        "merged": text(
            "Retainer and wiper are one seal block, head core and crown one casting, lid and housing one unit, foot flange and end cap one cylinder foot.",
            "Haltering und Abstreifer sind ein Dichtungsblock, Kopfkern und Kalotte ein Gussteil, Deckel und Gehäuse eine Einheit, Fußflansch und Boden ein Zylinderfuß."),
        "omitted": text(
            "Bolts, buffers, bushing, crosshead, LED windows, cap screws, glands, cable, conduit, port and grommet.",
            "Schrauben, Puffer, Buchse, Traverse, LED-Fenster, Kappenschrauben, Verschraubungen, Kabel, Schutzrohr, Anschluss und Tülle."),
        "thinElements": text(
            "Film 0.5 mm and lens 3 mm are cut slivers; the sleeve wall of 8 mm is 0.08 mm and reads as its outline; the hose is a 0.16 mm stroke.",
            "Folie 0,5 mm und Linse 3 mm sind geschnittene Streifen; die Hülsenwand von 8 mm misst 0,08 mm und liest sich als ihre Kontur; der Schlauch ist ein Strich von 0,16 mm."),
        "hatching": text("None.", "Keine."),
        "styles": text(
            "Cut steel 0.3, cut casting 0.35, far faces 0.75 to 0.9, shaft 0.5, film 0.92, lens 0.7, frame 0.6, cylinder 0.7, unit 0.5; body pen 0.25 mm, detail 0.13 mm.",
            "Geschnittener Stahl 0,3, geschnittener Guss 0,35, ferne Flächen 0,75 bis 0,9, Schaft 0,5, Folie 0,92, Linse 0,7, Rahmen 0,6, Zylinder 0,7, Einheit 0,5; Körperstift 0,25 mm, Detail 0,13 mm."),
        "overlays": text(
            "Lens carries beacon-led, film reflector-band; cylinder, rod, piston, hose and unit carry hydraulic-unit.",
            "Die Linse trägt beacon-led, die Folie reflector-band; Zylinder, Stange, Kolben, Schlauch und Einheit tragen hydraulic-unit."),
    },
    "200": {
        "silhouette": text(
            "Side view: section A-A, 2.8 mm wide and 9.5 mm high: the sleeve as a cut cup, two frame blocks, the shaft with white band and head, the rod in the cylinder, the unit on its bracket web. Front view: the outside from ahead: shaft, band and head over the frame, the sleeve below, the unit in front. Plan: frame, cap and the buried unit.",
            "Seitenansicht: Schnitt A-A, 2,8 mm breit und 9,5 mm hoch: die Hülse als geschnittener Becher, zwei Rahmenblöcke, der Schaft mit weißem Band und Kopf, die Stange im Zylinder, die Einheit an ihrem Konsolsteg. Vorderansicht: das Äußere von vorn: Schaft, Band und Kopf über dem Rahmen, darunter die Hülse, davor die Einheit. Draufsicht: Rahmen, Kappe und die eingegrabene Einheit."),
        "retained": text("Shaft 0.20 m, sleeve 0.30 m, frame 0.40 m, raised height 0.65 m and depth 1.25 m at true size.", "Schaft 0,20 m, Hülse 0,30 m, Rahmen 0,40 m, Ausfahrhöhe 0,65 m und Tiefe 1,25 m in wahrer Größe."),
        "merged": text(
            "Collar and plate are one frame block, lens ring and crown one head, cylinder foot, tube and gland one cylinder, the two bracket plates one web.",
            "Kragen und Platte sind ein Rahmenblock, Linsenring und Kalotte ein Kopf, Zylinderfuß, Rohr und Stopfbuchse ein Zylinder, die zwei Konsolbleche ein Steg."),
        "omitted": text("Tabs, seals, shaft flange, piston, hose and all fixings.", "Laschen, Dichtungen, Schaftflansch, Kolben, Schlauch und alle Befestigungen."),
        "thinElements": text(
            "The sleeve wall is drawn 15 mm (0.075 mm) and the shaft wall 10 mm so that they stay visible; the 36 mm rod is 0.18 mm.",
            "Die Hülsenwand ist mit 15 mm (0,075 mm) und die Schaftwand mit 10 mm gezeichnet, damit sie sichtbar bleiben; die Stange von 36 mm misst 0,18 mm."),
        "hatching": text("None.", "Keine."),
        "styles": text("Cut steel 0.3, far faces 0.75 to 0.9, shaft 0.5, band 0.92, frame 0.6, cylinder 0.7, unit 0.5; body pen 0.18 mm.", "Geschnittener Stahl 0,3, ferne Flächen 0,75 bis 0,9, Schaft 0,5, Band 0,92, Rahmen 0,6, Zylinder 0,7, Einheit 0,5; Körperstift 0,18 mm."),
        "overlays": text("The head lists shaft-steel and beacon-led; cylinder, rod and unit carry hydraulic-unit.", "Der Kopf nennt shaft-steel und beacon-led; Zylinder, Stange und Einheit tragen hydraulic-unit."),
    },
    "500": {
        "silhouette": text(
            "Side view: a section sign 1.1 mm wide and 3.8 mm high: the cut sleeve cup, two frame blocks at the ground, the shaft with its faceted cap rising 1.3 mm above the ground and the unit block against the sleeve. Front view: shaft and cap over the frame, the sleeve below and the unit in front of it. Plan: the frame square, the cap disc and the buried unit.",
            "Seitenansicht: ein Schnittzeichen, 1,1 mm breit und 3,8 mm hoch: der geschnittene Hülsenbecher, zwei Rahmenblöcke am Boden, der Schaft mit facettierter Kappe 1,3 mm über dem Boden und der Block der Einheit an der Hülse. Vorderansicht: Schaft und Kappe über dem Rahmen, darunter die Hülse und davor die Einheit. Draufsicht: das Rahmenquadrat, die Kappenscheibe und die eingegrabene Einheit."),
        "retained": text("Shaft 0.20 m, sleeve 0.30 m and frame 0.40 m at true width, raised height 0.65 m, depth 1.25 m.", "Schaft 0,20 m, Hülse 0,30 m und Rahmen 0,40 m in wahrer Breite, Ausfahrhöhe 0,65 m, Tiefe 1,25 m."),
        "merged": text(
            "Tube and reflective band are one shaft, lens ring and crown one cap, the unit and its bracket one block reaching the sleeve.",
            "Rohr und Reflexband sind ein Schaft, Linsenring und Kalotte eine Kappe, die Einheit und ihre Konsole ein Block bis an die Hülse."),
        "omitted": text("Cylinder, rod, hose, tabs, seals and all fixings.", "Zylinder, Stange, Schlauch, Laschen, Dichtungen und alle Befestigungen."),
        "thinElements": text(
            "The sleeve wall is drawn 20 mm (0.04 mm) and reads as the outline of the cup; the frame is 75 mm deep (0.15 mm).",
            "Die Hülsenwand ist mit 20 mm (0,04 mm) gezeichnet und liest sich als Kontur des Bechers; der Rahmen ist 75 mm tief (0,15 mm)."),
        "hatching": text("None.", "Keine."),
        "styles": text("Cut steel 0.3, shaft 0.5, frame 0.6, unit 0.5; body pen 0.13 mm, ground 0.18 mm.", "Geschnittener Stahl 0,3, Schaft 0,5, Rahmen 0,6, Einheit 0,5; Körperstift 0,13 mm, Boden 0,18 mm."),
        "overlays": text(
            "The shaft lists shaft-steel and reflector-band, the cap shaft-steel and beacon-led; the sleeve carries sleeve-steel.",
            "Der Schaft nennt shaft-steel und reflector-band, die Kappe shaft-steel und beacon-led; die Hülse trägt sleeve-steel."),
    },
}


def tube_section(outer, inner, material, en, de):
    """⭕ The section of a round tube for stress and deflection."""
    return {"areaM2": round(math.pi * (outer ** 2 - inner ** 2), 6), "secondMomentM4": float(f"{math.pi / 4 * (outer ** 4 - inner ** 4):.4g}"), "extremeFibreM": outer, "material": material, "title": text(en, de)}


def roots(doc):
    """🌱 Applies every root fix to the asset document in place."""
    a = doc["asset"]
    for m in a["materials"]:
        if m["id"] == HYD:
            m.setdefault("elasticModulusPa", 210000000000)
    bones = {b["id"]: b for b in a["rig"]["bones"]}
    s, h, f = bones["sleeve"], bones["hydraulic"], bones["shaft"]
    s.update({"x": 0, "y": -1.25, "angleDeg": 90, "lengthM": 1.25})
    s["mass"].update({"comX": 0.65, "comY": 0})
    s["section"] = tube_section(0.15, 0.142, GALV, "Sleeve tube 300 x 8 mm", "Hülsenrohr 300 x 8 mm")
    h.update({"x": 0.008, "y": 0, "angleDeg": 0, "lengthM": 0.972})
    h["mass"].update({"comX": 0.442, "comY": -0.2})
    h["section"] = tube_section(0.035, 0.03, HYD, "Cylinder tube 70 x 5 mm", "Zylinderrohr 70 x 5 mm")
    f.update({"x": 0.1, "y": 0, "angleDeg": 0, "lengthM": 1.15})
    f["joint"] = {"type": "prismatic", "minM": 0, "maxM": 0.65, "axisDeg": 0}
    f["mass"].update({"comX": 0.6, "comY": 0})
    f["section"] = tube_section(0.1, 0.095, STEEL, "Shaft tube 200 x 5 mm", "Schaftrohr 200 x 5 mm")
    anchors = {x["id"]: x for x in a["anchors"]}
    anchors["sleeve-fixed"].update({"x": 1.25, "y": 0})
    anchors["push-point"].update({"x": 1.05, "y": 0})
    anchors["beacon-glow"].update({"x": 1.05, "y": 0})
    for pose in a["poses"]:
        offset = pose["bones"].get("shaft")
        if offset is not None and "dy" in offset:
            pose["bones"]["shaft"] = {"dx": offset["dy"]}
    for scale, (elevation, front, plan) in GEOMETRY.items():
        rep = a["representations"][scale]
        rep["geometry"] = {"elevation": elevation(), "front": front(), "plan": plan()}
        ordered = {}
        for key, value in rep.items():
            if key == "sections":
                continue
            ordered[key] = value
            if key == "geometry":
                ordered["sections"] = {"elevation": {"plane": 0.0, "label": text("A-A", "A-A")}}
        rep.clear()
        rep.update(ordered)
        rep["decisions"] = REP_DECISIONS[scale]
    symbol = a["symbols"][0]["geometry"]
    for view, edits in (
        ("elevation", {"power-unit": {"x": 0.17, "y": -0.95, "w": 0.19, "h": 0.4}, "ram": {"x": -0.035, "y": -1.242, "w": 0.07, "h": 0.972}, "hose": {"points": [[r6(x), r6(y)] for x, y, _ in K.HOSE]}}),
        ("front", {"power-unit": {"x": 0.012, "y": -0.95, "w": 0.188, "h": 0.4}, "ram": {"x": -0.035, "y": -1.242, "w": 0.07, "h": 0.972}}),
        ("plan", {"power-unit": {"x": 0.17, "y": 0.012, "w": 0.19, "h": 0.188}, "motor": {"x": 0.284, "y": 0.09, "w": 0.032, "h": 0.032}}),
    ):
        ids = {x["id"]: x for x in symbol[view]}
        for sid, patch in edits.items():
            ids[sid].update(patch)
    for region in a["thermal"]["regions"]:
        if region["id"] in AREAS:
            region["areaM2"] = AREAS[region["id"]]
    return doc


def member_span(source, key):
    """📍 Character span of the value of a direct member of `asset` (two-space indent)."""
    m = re.search(r'\n  "' + key + r'": ', source)
    i, depth, quoted = m.end(), 0, False
    while True:
        c = source[i]
        if quoted:
            if c == "\\":
                i += 1
            elif c == '"':
                quoted = False
        elif c == '"':
            quoted = True
        elif c in "{[":
            depth += 1
        elif c in "}]":
            depth -= 1
            if depth == 0:
                return m.end(), i + 1
        i += 1


def main():
    """🚀 Candidate document or catalog write."""
    with open(CATALOG, encoding="utf-8") as fh:
        source = fh.read()
    doc = roots(json.loads(source))
    if "--write" in sys.argv:
        for key in MEMBERS:
            a, b = member_span(source, key)
            source = source[:a] + json.dumps(doc["asset"][key], ensure_ascii=False, indent=1).replace("\n", "\n  ") + source[b:]
        if json.loads(source) != doc:
            raise SystemExit("textual splice does not round-trip")
        with open(CATALOG, "w", encoding="utf-8") as fh:
            fh.write(source)
        print(CATALOG)
        return
    doc["asset"]["volumes"] = K.volumes()
    out = sys.argv[1]
    os.makedirs(os.path.dirname(out), exist_ok=True)
    with open(out, "w", encoding="utf-8") as fh:
        json.dump(doc, fh, ensure_ascii=False, indent=1)
        fh.write("\n")
    print(out)


if __name__ == "__main__":
    main()
