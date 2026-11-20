"""🧰 Round 3 sectional garage door: the root fixes outside `volumes` (rig, anchors, poses, dimensions, materials, references, thermal areas, the 2D drawings of every scale and view), applied to the catalog asset.

Usage: python3 r3-sectional-garage-door-roots.py            writes the candidate document (with the volumes source) to 🗑️generated/r3-sectional-garage-door/🔣️.json
       python3 r3-sectional-garage-door-roots.py --apply    writes the changed members into the catalog file (re-read right before, only these members change)
       --area=<region>:<m2> sets a thermal region area in either mode
"""
import copy
import importlib.util
import json
import math
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))


def load(name, file):
    spec = importlib.util.spec_from_file_location(name, os.path.join(T, file))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


K = load("kit", "r3-sectional-garage-door-kit.py")
L = load("layout", "r3-sliding-door-layout.py")
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/sectional-garage-door/🔣️.json"
MEMBERS = ["provenance", "dimensions", "materials", "rig", "anchors", "representations", "poses", "thermal"]
AREAS = {"track-steel": 4.2}
TB, SB = "track", "spring-shaft"
PS, HW, TR, SP = "panel-sandwich", "hardware-steel", "track-steel", "spring-assembly"
SIDES = (("left", 1), ("right", -1))


def r5(x):
    return round(x + 0.0, 5)


def shape(i, bone, role, z, filled, tone, geometry, region=None, regions=None, material=None, pen=None, solid=None):
    out = {"id": i, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    if region:
        out["region"] = region
    if regions:
        out["regions"] = regions
    if material:
        out["material"] = material
    if pen:
        out["penMm"] = pen
    out.update(geometry)
    if solid:
        out["solid"] = solid
    return out


def rect(x0, y0, x1, y1):
    return {"kind": "rect", "x": r5(min(x0, x1)), "y": r5(min(y0, y1)), "w": r5(abs(x1 - x0)), "h": r5(abs(y1 - y0))}


def poly(points, closed=True):
    return {"kind": "poly", "points": [[r5(x), r5(y)] for x, y in points], "closed": closed}


def disc(cx, cy, r):
    return {"kind": "ellipse", "cx": r5(cx), "cy": r5(cy), "rx": r5(r), "ry": r5(r)}


def span(z0, z1):
    return {"kind": "extruded", "z0": r5(min(z0, z1)), "z1": r5(max(z0, z1)), "sweep": "w"}


def pb(k):
    return f"panel-{k}"


def steel(region):
    return dict(region=region, material="steel")


def strip(a, b, w):
    """📏 A straight bar of width w from a to b as a polygon."""
    dx, dy = b[0] - a[0], b[1] - a[1]
    n = math.hypot(dx, dy)
    ox, oy = -dy / n * w / 2, dx / n * w / 2
    return [[a[0] + ox, a[1] + oy], [b[0] + ox, b[1] + oy], [b[0] - ox, b[1] - oy], [a[0] - ox, a[1] - oy]]


def hinge_block():
    eye = [[K.EYE_R * math.cos(math.radians(a)), K.EYE_R * math.sin(math.radians(a))] for a in range(135, 406, 45)]
    return [[0.07, K.V_IN], [0.002, K.V_IN]] + eye + [[0.016, K.V_IN - K.SHEET], [0.07, K.V_IN - K.SHEET]]


def bulb():
    a = K.RAIL_U
    outer = [[a, 0.024], [a, 0.056], [0.012, 0.058], [0.004, 0.054], [0.0, 0.046], [0.0, 0.034], [0.004, 0.026], [0.012, 0.022]]
    bore = [[a - 0.003, 0.027], [a - 0.003, 0.053], [0.012, 0.055], [0.006, 0.052], [0.003, 0.045], [0.003, 0.035], [0.006, 0.028], [0.012, 0.025]]
    ring = lambda pts: "M " + " L ".join("%g %g" % (r5(x), r5(y)) for x, y in pts) + " Z"
    return {"kind": "path", "d": ring(outer) + " " + ring(bore)}, poly(outer)


def rail_u():
    lo, hi, a, b, c = K.V_IN - 0.0015, K.V_OUT + 0.0015, K.RAIL_U, K.RAIL_U + 0.006, K.RAIL_TOP
    return [[a, lo], [c, lo], [c, K.V_IN], [b, K.V_IN], [b, K.V_OUT], [c, K.V_OUT], [c, hi], [a, hi]]


def cap_u():
    lo, hi, t0, t1, e = K.V_IN - 0.0015, K.V_OUT + 0.0015, K.TOP_CAP_U, K.TOP_FLAT, K.PANEL
    return [[e, lo], [e, hi], [t0, hi], [t0, K.V_OUT], [t1, K.V_OUT], [t1, K.V_IN], [t0, K.V_IN], [t0, lo]]


ROLLERS = [("0", 1, K.BOTTOM_ROLLER_U), ("1", 1, K.PANEL), ("2", 2, K.PANEL), ("3", 3, K.PANEL), ("4", 4, K.TOP_ROLLER_U)]


def leaf_section(scale):
    """🚪 The door leaf in the section A-A: every part in the frame of its panel bone (x = u up the panel from the hinge pin, y = v towards the outside)."""
    out = []
    if scale == 50:
        for k in range(1, 5):
            out.append(shape(pb(k), pb(k), "body", 3, True, 0.55, poly(K.panel_outline(k)[0]), region=PS, material="sandwich-skin"))
        out.append(shape("bottom-rail", pb(1), "detail", 3.2, True, 0.3, poly(rail_u()), region=PS, material="steel"))
        out.append(shape("bottom-seal", pb(1), "detail", 3.2, True, 0.15, bulb()[0], region=PS, material="epdm"))
        out.append(shape("top-cap", pb(4), "detail", 3.2, True, 0.3, poly(cap_u()), region=PS, material="steel"))
        for name, k, u in ROLLERS:
            out.append(shape(f"roller-{name}", pb(k), "detail", 3.8, True, 0.2, disc(u, 0, K.WHEEL_R), **steel(HW)))
        out.append(shape("bottom-bracket", pb(1), "detail", 4, True, 0.3, poly(K.roller_bracket(K.BOTTOM_ROLLER_U, K.RAIL_TOP + 0.001, 0.15)), **steel(HW)))
        out.append(shape("cable-stud", pb(1), "detail", 4.1, True, 0.2, disc(K.CABLE_EYE[0], K.CABLE_EYE[1], 0.0075), **steel(HW)))
        out.append(shape("top-bracket", pb(4), "detail", 4, True, 0.3, poly(K.roller_bracket(K.TOP_ROLLER_U, 0.37, K.TOP_CAP_U - 0.002)), **steel(HW)))
        for j in range(1, 4):
            out.append(shape(f"hinge-low-{j}", pb(j), "detail", 4, True, 0.3, poly(K.leaf_lower()), **steel(HW)))
            out.append(shape(f"hinge-{j}", pb(j + 1), "detail", 4.05, True, 0.3, poly(K.leaf_upper()), **steel(HW)))
    elif scale == 100:
        for k in range(1, 5):
            out.append(shape(pb(k), pb(k), "body", 3, True, 0.55, poly(K.panel_outline_100(k)), region=PS, material="sandwich-skin"))
        out.append(shape("bottom-seal", pb(1), "detail", 3.2, True, 0.15, rect(0, 0.022, K.RAIL_U, 0.058), region=PS, material="epdm"))
        for name, k, u in ROLLERS:
            out.append(shape(f"roller-{name}", pb(k), "detail", 3.8, True, 0.2, disc(u, 0, K.WHEEL_R), **steel(HW)))
        for j in range(1, 4):
            out.append(shape(f"hinge-low-{j}", pb(j), "detail", 4, True, 0.3, poly([[K.PANEL - u, v] for u, v in hinge_block()][::-1]), **steel(HW)))
            out.append(shape(f"hinge-{j}", pb(j + 1), "detail", 4.05, True, 0.3, poly(hinge_block()), **steel(HW)))
    elif scale == 200:
        for k in range(1, 5):
            mid = K.V_IN + K.THICK / 2
            out.append(shape(pb(k), pb(k), "body", 3, False, 0.5, poly([[0, mid], [K.PANEL, mid]], False), regions=[PS, HW], material="sandwich-skin", pen=0.2))
    else:
        for k in range(1, 5):
            out.append(shape(pb(k), pb(k), "body", 3, True, 0.5, rect(0, K.V_IN, K.PANEL, K.V_OUT), regions=[PS, HW], material="sandwich-skin"))
    return out


def frame_section(scale):
    """🛤️ The fixed frame in the section A-A (track bone = world x ahead, y up): what lies beyond the cut plane on the left side, and the cut spreader."""
    s = steel(TR)
    out = [shape("ground", TB, "ground", 0, False, 0.5, poly([[-0.2, 0], [2.65, 0]], False))]
    if scale in (50, 100):
        for n, y in enumerate(K.JAMB_Y if scale == 50 else ()):
            out.append(shape(f"jamb-bracket-{n}", TB, "detail", 0.8, True, 0.35, rect(K.WALL_X, y, 0.02, y + K.JAMB_H), **s))
        out.append(shape("flag-bracket", TB, "detail", 0.8, True, 0.35, rect(K.WALL_X, 1.86, 0.06, 2.14), **s))
        out.append(shape("bearing-flag", TB, "detail", 0.8, True, 0.4, rect(K.WALL_X, 2.66, 0.13, 2.84), **s))
        out.append(shape("track-band", TB, "detail", 1, True, 0.4, poly(K.ribbon(2 * K.TRACK_HALF, 12 if scale == 50 else 8)), **s))
        out.append(shape("track-angle", TB, "detail", 1.1, True, 0.4, rect(0.5, 2.526, K.TRACK_END, 2.575), **s))
        for n, x in enumerate(K.HANGER_X):
            out.append(shape(f"hanger-{n}", TB, "detail", 1.2, True, 0.4, rect(x - 0.015, 2.5285, x + 0.015, K.CEILING), **s))
            if scale == 50:
                out.append(shape(f"ceiling-foot-{n}", TB, "detail", 1.3, True, 0.35, rect(x - 0.04, 2.80, x + 0.04, K.CEILING), **s))
        out.append(shape("brace", TB, "detail", 1.25, True, 0.4, poly(strip(K.BRACE_A, K.BRACE_B, 0.03)), **s))
        if scale == 50:
            out.append(shape("brace-foot", TB, "detail", 1.3, True, 0.35, rect(K.BRACE_B[0] - 0.05, 2.80, K.BRACE_B[0] + 0.03, K.CEILING), **s))
        out.append(shape("track-end-bar", TB, "detail", 1.4, True, 0.3, rect(2.56, 2.48, 2.6, 2.52), **s))
    elif scale == 200:
        for n, y in enumerate(K.JAMB_Y):
            out.append(shape(f"jamb-bracket-{n}", TB, "detail", 0.8, True, 0.35, rect(K.WALL_X, y, 0.02, y + K.JAMB_H), **s))
        out.append(shape("flag-bracket", TB, "detail", 0.8, True, 0.35, rect(K.WALL_X, 1.86, 0.06, 2.14), **s))
        out.append(shape("bearing-flag", TB, "detail", 0.8, True, 0.4, rect(K.WALL_X, 2.66, 0.13, 2.84), **s))
        out.append(shape("track", TB, "detail", 1, False, 0.4, poly(K.centre_line(8), False), pen=0.26, **s))
        for n, x in enumerate(K.HANGER_X):
            out.append(shape(f"hanger-{n}", TB, "detail", 1.2, True, 0.4, rect(x - 0.015, 2.526, x + 0.015, K.CEILING), **s))
        out.append(shape("brace", TB, "detail", 1.25, True, 0.4, poly(strip(K.BRACE_A, K.BRACE_B, 0.03)), **s))
        out.append(shape("track-end-bar", TB, "detail", 1.4, True, 0.3, rect(2.56, 2.48, 2.6, 2.52), **s))
    else:
        out.append(shape("track", TB, "body", 1, True, 0.4, poly(K.ribbon(2 * K.TRACK_HALF, 5)), **s))
    return out


def spring_section(scale):
    s = steel(SP)
    out = [shape("drum", SB, "body", 2, True, 0.5 if scale == 50 else 0.4, disc(0, 0, K.DRUM_R), **s)]
    if scale == 50:
        out += [
            shape("cone-winding", SB, "detail", 2.5, True, 0.3, disc(0, 0, 0.04), **s),
            shape("coil", SB, "detail", 3, False, 0.4, disc(0, 0, K.COIL_R + K.WIRE_R), **s),
            shape("shaft", SB, "detail", 4, True, 0.2, disc(0, 0, K.SHAFT_R), **s),
            shape("drum-index", SB, "texture", 5, False, 0.9, poly([[0.045, 0], [K.DRUM_R, 0]], False), region=SP),
        ]
    elif scale == 100:
        out += [
            shape("coil", SB, "detail", 3, True, 0.3, disc(0, 0, 0.045), **s),
            shape("drum-index", SB, "detail", 5, False, 0.9, poly([[0.045, 0], [K.DRUM_R, 0]], False), region=SP),
        ]
    elif scale == 200:
        out.append(shape("drum-index", SB, "detail", 3, False, 0.9, poly([[0, 0], [K.DRUM_R, 0]], False), region=SP, pen=0.1))
    return out


def elevation(scale):
    return frame_section(scale) + leaf_section(scale) + spring_section(scale)


def leaf_solids(scale, plan):
    """🧊 The leaf parts for the front elevation and the plan: each one its bone-frame profile with its lateral span, so both views are projections of one body in every pose."""
    out = []
    name = (lambda k: f"panel-edge-{k}") if plan else pb
    if scale == 50:
        for k in range(1, 5):
            out.append(shape(name(k), pb(k), "body", 3, True, 0.55, poly(K.panel_outline(k)[0]), region=PS, material="sandwich-skin", solid=span(-K.HALF, K.HALF)))
            u0, u1 = (K.RAIL_TOP if k == 1 else 0.001), (K.TOP_CAP_U if k == 4 else K.PANEL)
            for sd, sg in SIDES:
                out.append(shape(f"stile-{k}-{sd}", pb(k), "detail", 3.3, True, 0.45, rect(u0, K.V_IN - 0.0015, u1, K.V_OUT + 0.0015), region=PS, material="steel", solid=span(sg * 1.18, sg * K.CAP)))
        out.append(shape("bottom-rail", pb(1), "detail", 3.2, True, 0.3, poly(rail_u()), region=PS, material="steel", solid=span(-K.HALF, K.HALF)))
        out.append(shape("bottom-seal", pb(1), "detail", 3.2, True, 0.15, bulb()[1], region=PS, material="epdm", solid=span(-K.HALF, K.HALF)))
        out.append(shape("top-cap", pb(4), "detail", 3.2, True, 0.3, poly(cap_u()), region=PS, material="steel", solid=span(-K.HALF, K.HALF)))
        for j in range(1, 4):
            for n, c in enumerate(K.HINGE_C):
                out.append(shape(f"hinge-low-{j}-{n}", pb(j), "detail", 4, True, 0.3, poly(K.leaf_lower()), solid=span(c - 0.04, c + 0.04), **steel(HW)))
                out.append(shape(f"hinge-{j}-{n}", pb(j + 1), "detail", 4.05, True, 0.3, poly(K.leaf_upper()), solid=span(c - 0.0175, c + 0.0175), **steel(HW)))
            for n, c in enumerate(K.HINGE_C[1:3]):
                out.append(shape(f"hinge-pin-{j}-{n}", pb(j + 1), "detail", 4.1, True, 0.2, disc(0, 0, 0.004), solid=span(c - 0.043, c + 0.043), **steel(HW)))
        for nm, k, u in ROLLERS:
            for sd, sg in SIDES:
                out.append(shape(f"roller-stem-{nm}-{sd}", pb(k), "detail", 4.1, True, 0.2, disc(u, 0, K.STEM_R), solid=span(sg * K.STEM_Z0, sg * K.WHEEL_Z0), **steel(HW)))
                out.append(shape(f"roller-{nm}-{sd}", pb(k), "detail", 0.9, True, 0.2, disc(u, 0, K.WHEEL_R), solid=span(sg * K.WHEEL_Z0, sg * K.WHEEL_Z1), **steel(HW)))
        for sd, sg in SIDES:
            out.append(shape(f"bottom-bracket-{sd}", pb(1), "detail", 4, True, 0.3, poly(K.roller_bracket(K.BOTTOM_ROLLER_U, K.RAIL_TOP + 0.001, 0.15)), solid=span(sg * 1.11, sg * 1.18), **steel(HW)))
            out.append(shape(f"top-bracket-{sd}", pb(4), "detail", 4, True, 0.3, poly(K.roller_bracket(K.TOP_ROLLER_U, 0.37, K.TOP_CAP_U - 0.002)), solid=span(sg * 1.11, sg * 1.18), **steel(HW)))
            out.append(shape(f"cable-stud-{sd}", pb(1), "detail", 4.1, True, 0.2, disc(K.CABLE_EYE[0], K.CABLE_EYE[1], 0.0075), solid=span(sg * 1.14, sg * 1.2165), **steel(HW)))
    elif scale == 100:
        for k in range(1, 5):
            out.append(shape(name(k), pb(k), "body", 3, True, 0.55, poly(K.panel_outline_100(k)), region=PS, material="sandwich-skin", solid=span(-K.CAP, K.CAP)))
        out.append(shape("bottom-seal", pb(1), "detail", 3.2, True, 0.15, rect(0, 0.022, K.RAIL_U, 0.058), region=PS, material="epdm", solid=span(-K.CAP, K.CAP)))
        for j in range(1, 4):
            for n, c in enumerate(K.HINGE_C):
                out.append(shape(f"hinge-{j}-{n}", pb(j + 1), "detail", 4, True, 0.3, poly(hinge_block()), solid=span(c - 0.04, c + 0.04), **steel(HW)))
                out.append(shape(f"hinge-low-{j}-{n}", pb(j), "detail", 4, True, 0.3, poly([[K.PANEL - u, v] for u, v in hinge_block()][::-1]), solid=span(c - 0.04, c + 0.04), **steel(HW)))
        for nm, k, u in ROLLERS:
            for sd, sg in SIDES:
                out.append(shape(f"roller-{nm}-{sd}", pb(k), "detail", 0.9, True, 0.2, disc(u, 0, K.WHEEL_R), solid=span(sg * K.WHEEL_Z0, sg * K.WHEEL_Z1), **steel(HW)))
    else:
        for k in range(1, 5):
            out.append(shape(name(k), pb(k), "body", 3, True, 0.55 if scale == 200 else 0.5, rect(0, K.V_IN, K.PANEL, K.V_OUT), material="sandwich-skin", regions=[PS, HW], solid=span(-K.CAP, K.CAP)))
        if scale == 200:
            for j in range(1, 4):
                out.append(shape(f"hinge-axis-{j}", pb(j + 1), "detail", 4, True, 0.3, disc(0, 0, K.STEM_R), solid=span(-K.WHEEL_Z1, K.WHEEL_Z1), pen=0.15, **steel(HW)))
    return out


def spring_solids(scale):
    s = steel(SP)
    out = []
    for sd, sg in SIDES:
        out.append(shape(f"drum-{sd}", SB, "body", 6, True, 0.5 if scale == 50 else 0.4, disc(0, 0, K.DRUM_R), solid=span(sg * K.DRUM_Z0, sg * (K.DRUM_Z0 + K.DRUM_W)), **s))
    if scale in (50, 100):
        out.append(shape("shaft", SB, "detail", 5.8, True, 0.2, disc(0, 0, K.SHAFT_R), solid=span(-1.25, 1.25), **s))
        out.append(shape("spring-coil", SB, "detail", 6, True, 0.4, disc(0, 0, K.COIL_R + K.WIRE_R), solid=span(-K.COIL_HALF + (0.02 if scale == 100 else 0), K.COIL_HALF - (0.02 if scale == 100 else 0)), **s))
        if scale == 50:
            out.append(shape("cone-winding", SB, "detail", 6.1, True, 0.3, disc(0, 0, 0.04), solid=span(K.COIL_HALF, K.COIL_HALF + 0.04), **s))
        else:
            for sd, sg in SIDES:
                out.append(shape(f"spring-collar-{sd}", SB, "detail", 6.1, True, 0.3, disc(0, 0, 0.045), solid=span(sg * (K.COIL_HALF - 0.02), sg * K.COIL_HALF), **s))
    return out


def front(scale):
    """🔭 The front elevation, seen from ahead, that is from inside the garage (x = lateral, y = height): inner face of the leaf with hinges and stems, tracks, hangers, spreader and the spring line in front."""
    s = steel(TR)
    out = []
    if scale == 50:
        for sd, sg in SIDES:
            for n, y in enumerate(K.JAMB_Y):
                out.append(shape(f"jamb-bracket-{sd}-{n}", TB, "detail", 1, True, 0.35, rect(sg * 1.205, y, sg * 1.25, y + K.JAMB_H), **s))
            out.append(shape(f"flag-bracket-{sd}", TB, "detail", 1, True, 0.35, rect(sg * 1.205, 1.86, sg * 1.25, 2.14), **s))
    out += leaf_solids(scale, False)
    for sd, sg in SIDES:
        if scale == 500:
            out.append(shape(f"track-{sd}", TB, "body", 5, True, 0.4, rect(sg * K.TRACK_Z0, 0, sg * K.TRACK_Z1, 2.526), pen=0.14, **s))
            continue
        out.append(shape(f"track-{sd}", TB, "detail", 5, True, 0.4, rect(sg * K.TRACK_Z0, 0, sg * K.TRACK_Z1, 2.526), **s))
        out.append(shape(f"bearing-flag-{sd}", TB, "detail", 5, True, 0.4, rect(sg * 1.205, 2.66, sg * 1.25, 2.84), **s))
        if scale in (50, 100):
            out.append(shape(f"track-angle-{sd}", TB, "detail", 5.1, True, 0.4, rect(sg * K.TRACK_Z0, 2.526, sg * 1.25, 2.575), **s))
            out.append(shape(f"hanger-{sd}", TB, "detail", 5.2, True, 0.4, rect(sg * K.TRACK_Z0, 2.5285, sg * K.TRACK_Z1, K.CEILING), **s))
            if scale == 50:
                out.append(shape(f"ceiling-foot-{sd}", TB, "detail", 5.3, True, 0.35, rect(sg * 1.19, 2.80, sg * 1.245, K.CEILING), **s))
        else:
            out.append(shape(f"hanger-{sd}", TB, "detail", 5.2, True, 0.4, rect(sg * K.TRACK_Z0, 2.526, sg * K.TRACK_Z1, K.CEILING), **s))
    if scale != 500:
        out.append(shape("track-end-bar", TB, "detail", 7, True, 0.4, rect(-K.TRACK_Z0, 2.48, K.TRACK_Z0, 2.52), **s))
    else:
        out.append(shape("track-end-bar", TB, "body", 7, False, 0.4, poly([[-K.TRACK_Z0, 2.5], [K.TRACK_Z0, 2.5]], False), pen=0.1, **s))
    if scale in (50, 100):
        out.append(shape("anchor-bracket", TB, "detail", 5.5, True, 0.35, rect(-K.COIL_HALF - 0.072, 2.68, -K.COIL_HALF - 0.012, 2.82), **s))
    if scale == 50:
        out.append(shape("cone-stationary", TB, "detail", 6.1, True, 0.3, rect(-K.COIL_HALF - 0.012, K.SHAFT[1] - 0.045, -K.COIL_HALF, K.SHAFT[1] + 0.045), **s))
    out += spring_solids(scale)
    if scale in (200, 500):
        role = "detail" if scale == 200 else "body"
        out.append(shape("shaft", SB, role, 5.8, False, 0.4, poly([[-1.25, K.SHAFT[1]], [1.25, K.SHAFT[1]]], False), pen=0.13, **steel(SP)))
    if scale == 200:
        out.append(shape("spring-coil", SB, "detail", 6, True, 0.4, disc(0, 0, K.COIL_R + K.WIRE_R), solid=span(-K.COIL_HALF, K.COIL_HALF), **steel(SP)))
    return out


def plan(scale):
    """🗺️ The plan, seen from above (x = ahead, y = lateral): tracks with their angles, hangers and braces, spreader, the leaf under the spring line, bearing plates and anchor bracket on the wall plane."""
    s = steel(TR)
    out = leaf_solids(scale, True)
    for sd, sg in SIDES:
        if scale == 500:
            out.append(shape(f"track-{sd}", TB, "body", 5, True, 0.4, rect(-K.TRACK_HALF, sg * K.TRACK_Z0, K.TRACK_END, sg * K.TRACK_Z1), pen=0.14, **s))
            continue
        out.append(shape(f"track-{sd}", TB, "detail", 5, True, 0.4, rect(-K.TRACK_HALF, sg * K.TRACK_Z0, K.TRACK_END, sg * K.TRACK_Z1), **s))
        if scale in (50, 100):
            out.append(shape(f"track-angle-{sd}", TB, "detail", 5.1, True, 0.4, rect(0.5, sg * K.TRACK_Z0, K.TRACK_END, sg * 1.25), **s))
        if scale != 500:
            out.append(shape(f"flag-bracket-{sd}", TB, "detail", 4.5, False, 0.35, poly([[K.WALL_X, sg * 1.205], [K.WALL_X, sg * 1.25], [0.06, sg * 1.25]], False), **s))
        out.append(shape(f"bearing-flag-{sd}", TB, "detail", 5.4, False, 0.4, poly([[K.WALL_X, sg * 1.205], [K.WALL_X, sg * 1.25], [0.13, sg * 1.25]], False), **s))
        for n, x in enumerate(K.HANGER_X):
            tag = "ab"[n]
            out.append(shape(f"hanger-{sd}-{tag}", TB, "detail", 5.5, True, 0.3, rect(x - 0.015, sg * K.TRACK_Z0, x + 0.015, sg * K.TRACK_Z1), **s))
            if scale == 50:
                out.append(shape(f"ceiling-foot-{sd}-{tag}", TB, "detail", 5.6, True, 0.35, rect(x - 0.04, sg * 1.19, x + 0.04, sg * 1.245), **s))
        out.append(shape(f"brace-{sd}", TB, "detail", 5.5, True, 0.4, rect(K.BRACE_B[0], sg * 1.2125, K.BRACE_A[0], sg * 1.2425), **s))
        if scale == 50:
            out.append(shape(f"brace-foot-{sd}", TB, "detail", 5.6, True, 0.35, rect(K.BRACE_B[0] - 0.05, sg * 1.19, K.BRACE_B[0] + 0.03, sg * 1.245), **s))
    if scale != 500:
        out.append(shape("track-end-bar", TB, "detail", 5.2, True, 0.4, rect(2.56, -K.TRACK_Z0, 2.6, K.TRACK_Z0), **s))
    else:
        out.append(shape("track-end-bar", TB, "body", 5.2, False, 0.4, poly([[2.58, -K.TRACK_Z0], [2.58, K.TRACK_Z0]], False), pen=0.1, **s))
    if scale in (50, 100):
        z = -K.COIL_HALF - 0.012
        out.append(shape("anchor-bracket", TB, "detail", 5.4, False, 0.35, poly([[K.WALL_X, z - 0.06], [K.WALL_X, z], [0.125, z]], False), **s))
    if scale == 50:
        out.append(shape("cone-stationary", TB, "detail", 6.1, True, 0.3, rect(K.SHAFT[0] - 0.045, -K.COIL_HALF - 0.012, K.SHAFT[0] + 0.045, -K.COIL_HALF), **s))
    out += spring_solids(scale)
    if scale in (200, 500):
        role = "detail" if scale == 200 else "body"
        out.append(shape("shaft", SB, role, 5.8, False, 0.4, poly([[K.SHAFT[0], -1.25], [K.SHAFT[0], 1.25]], False), pen=0.13, **steel(SP)))
    if scale == 200:
        out.append(shape("spring-coil", SB, "detail", 6, True, 0.4, disc(0, 0, K.COIL_R + K.WIRE_R), solid=span(-K.COIL_HALF, K.COIL_HALF), **steel(SP)))
    return out


def text(en, de):
    return {"en": en, "de": de}


CONVENTION_EN = "Convention: the elevation is the section A-A along the travel at the door centre, looking at the left half; the bone line of every panel is its hinge line, 20 mm behind the inner face, and lies on the roller path. The front view is the door seen from ahead, that is from inside the garage (lateral x: left of the door is +x), the plan view the view from above; front and plan are projections of the same parts with their lateral spans. The lateral extent of 2.5 m is the width over the brackets."
CONVENTION_DE = "Konvention: die Ansicht ist der Schnitt A-A in Fahrtrichtung in Tormitte mit Blick auf die linke Hälfte; die Knochenlinie jedes Paneels ist seine Scharnierlinie, 20 mm hinter der Innenseite, und liegt auf der Rollenbahn. Die Vorderansicht ist das Tor von vorn gesehen, also aus der Garage (seitliches x: links vom Tor ist +x), die Draufsicht der Blick von oben; Vorderansicht und Draufsicht sind Projektionen derselben Teile mit ihren seitlichen Spannen. Die seitliche Ausdehnung von 2,5 m ist die Breite über die Winkel."

DECISIONS = {
    "50": {
        "silhouette": text(
            "The full section of a sectional door: four panels cut through their finger-protection joints (nose, socket, lip, rib), bottom rail with bulb seal, top cap, hinge leaves with their eyes on the roller line, 46 mm rollers in the 52 mm track band, jamb brackets, flag bracket, stiffening angle, hangers with ceiling feet and brace, the cut spreader, bearing plate, drum, winding cone, coil ring, shaft and index. " + CONVENTION_EN + " At 1:50 the front view shows the inner face with end stiles, four hinges per joint, roller stems, carriers, tracks with brackets, hangers, the spreader and the spring line with anchor bracket; the plan shows the same parts from above.",
            "Der volle Schnitt eines Sektionaltors: vier Paneele, durch ihre Fingerschutzfugen geschnitten (Nase, Pfanne, Lippe, Sicke), Bodenschiene mit Wulstdichtung, Abdeckkappe, Scharnierlaschen mit ihren Augen auf der Rollenlinie, 46-mm-Rollen im 52 mm breiten Schienenband, Zargenwinkel, Bogenwinkel, Versteifungswinkel, Abhänger mit Deckenfüßen und Strebe, die geschnittene Querstrebe, Lagerplatte, Trommel, Spannkonus, Federring, Welle und Marke. " + CONVENTION_DE + " Bei 1:50 zeigt die Vorderansicht die Innenseite mit Endkappen, vier Scharnieren je Fuge, Rollenschäften, Haltern, Schienen mit Winkeln, Abhängern, der Querstrebe und der Federlinie mit Federbock; die Draufsicht zeigt dieselben Teile von oben.",
        ),
        "retained": text("Five rollers (0.046 m), three hinge joints with both leaves, carriers, cable stud, drum, cone, coil, shaft, index.", "Fünf Rollen (0,046 m), drei Scharniergelenke mit beiden Laschen, Halter, Seilbolzen, Trommel, Konus, Feder, Welle, Marke."),
        "merged": text("Nothing merged.", "Nichts zusammengefasst."),
        "omitted": text("The running cable (its length changes with the pose), handle, lock, screws and bolts below 7.5 mm.", "Das laufende Seil (seine Länge ändert sich mit der Pose), Griff, Schloss, Schrauben und Bolzen unter 7,5 mm."),
        "thinElements": text("Hinge leaves, carriers, rail and cap at their real 1.5 to 2.5 mm; the panel skins (0.5 mm) are the outline.", "Scharnierlaschen, Halter, Schiene und Kappe in ihren echten 1,5 bis 2,5 mm; die Paneelhäute (0,5 mm) sind die Kontur."),
        "hatching": text("None.", "Keine."),
        "styles": text("Panels 0.35 mm outline, hardware and frame 0.18 mm, index 0.1 mm.", "Paneele 0,35-mm-Kontur, Beschläge und Rahmen 0,18 mm, Marke 0,1 mm."),
        "overlays": text("One cell per region: panels with their edge profiles, hardware (rollers, hinges, carriers), track frame, spring assembly.", "Eine Zelle je Region: Paneele mit ihren Randprofilen, Beschläge (Rollen, Scharniere, Halter), Schienenrahmen, Federbaugruppe."),
    },
    "100": {
        "silhouette": text(
            "Four panel sections with the simplified nose and socket on the 52 mm track band, hinge leaves with eyes, roller wheels, seal block, brackets, stiffening angle, hangers, brace, spreader, bearing plate, drum with spring collar and index. " + CONVENTION_EN + " At 1:100 the front view shows the panels with four hinges per joint, tracks with brackets, angles and hangers, the spreader, drums, the spring with its collars and the shaft; the plan shows the same parts from above.",
            "Vier Paneelquerschnitte mit vereinfachter Nase und Pfanne auf dem 52 mm breiten Schienenband, Scharnierlaschen mit Augen, Rollenräder, Dichtungsblock, Winkel, Versteifungswinkel, Abhänger, Strebe, Querstrebe, Lagerplatte, Trommel mit Federbund und Marke. " + CONVENTION_DE + " Bei 1:100 zeigt die Vorderansicht die Paneele mit vier Scharnieren je Fuge, Schienen mit Winkeln und Abhängern, die Querstrebe, Trommeln, die Feder mit ihren Bunden und die Welle; die Draufsicht zeigt dieselben Teile von oben.",
        ),
        "retained": text("Panels at 0.04 x 0.5 m (0.4 mm) with their joint, track band of 0.052 m, rollers 0.046 m, hinges, hangers, drum with index.", "Paneele mit 0,04 x 0,5 m (0,4 mm) mit ihrer Fuge, Schienenband 0,052 m, Rollen 0,046 m, Scharniere, Abhänger, Trommel mit Marke."),
        "merged": text("Both leaves and the pin of a hinge are one leaf; stiles, rail and cap are merged into the panels; the spring and its cones are one body.", "Beide Laschen und der Bolzen eines Scharniers sind eine Lasche; Endkappen, Schiene und Kappe sind in die Paneele eingeschmolzen; Feder und Konen sind ein Körper."),
        "omitted": text("Jamb brackets in the section and all brackets in the front view (hidden behind panel and track but for slivers below 0.2 mm), roller stems (0.011 m), carriers, cable stud, ceiling feet, rib and shadow groove, cable and handle.", "Zargenwinkel im Schnitt und alle Winkel in der Vorderansicht (hinter Paneel und Schiene bis auf Streifen unter 0,2 mm verdeckt), Rollenschäfte (0,011 m), Halter, Seilbolzen, Deckenfüße, Sicke und Schattenfuge, Seil und Griff."),
        "thinElements": text("None thickened.", "Nichts verdickt."),
        "hatching": text("None: the door is an elevation of a section.", "Keine: das Tor ist eine Schnittansicht."),
        "styles": text("Panels 0.25 mm outline filled mid-grey, hardware and frame 0.13 mm.", "Paneele mit 0,25-mm-Kontur mittelgrau gefüllt, Beschläge und Rahmen 0,13 mm."),
        "overlays": text("Cells: panel sections, hardware, track frame, spring assembly.", "Zellen: Paneelquerschnitte, Beschläge, Schienenrahmen, Federbaugruppe."),
    },
    "200": {
        "silhouette": text(
            "Four panel lines of 0.2 mm on their true line 40 mm in front of the roller path, the track as a line of its 52 mm, hangers, brace, spreader, bearing plate, the drum as a filled disc with an index mark. " + CONVENTION_EN + " At 1:200 the front view shows four panel bars, the two track bars with hangers, the bearing plates, the spreader, the two drums, the spring and the shaft; the plan shows the leaf, the tracks with hangers and braces, the spreader, drums, spring and shaft.",
            "Vier 0,2-mm-Paneellinien auf ihrer wahren Linie 40 mm vor der Rollenbahn, die Schiene als Linie ihrer 52 mm, Abhänger, Strebe, Querstrebe, Lagerplatte, die Trommel als gefüllte Scheibe mit Marke. " + CONVENTION_DE + " Bei 1:200 zeigt die Vorderansicht vier Paneelbalken, die beiden Schienenbalken mit Abhängern, die Lagerplatten, die Querstrebe, die beiden Trommeln, die Feder und die Welle; die Draufsicht zeigt das Torblatt, die Schienen mit Abhängern und Streben, die Querstrebe, Trommeln, Feder und Welle.",
        ),
        "retained": text("Panels at the real 0.5 m height and 0.04 m thickness (0.2 mm line), track line 0.26 mm (0.052 m), drum 0.18 m with its index line that shows the winding.", "Paneele in der echten Höhe 0,5 m und Dicke 0,04 m (0,2-mm-Linie), Schienenlinie 0,26 mm (0,052 m), Trommel 0,18 m mit Markierungslinie, die das Aufwickeln zeigt."),
        "merged": text("Hinges and rollers are merged into the panel lines; the stiffening angle into the track.", "Scharniere und Rollen sind in die Paneellinien eingeschmolzen; der Versteifungswinkel in die Schiene."),
        "omitted": text("Ceiling feet, hinge leaves, roller wheels, carriers, cable.", "Deckenfüße, Scharnierlaschen, Rollenräder, Halter, Seil."),
        "thinElements": text("Panel line 0.2 mm and track line 0.26 mm are the real 0.04 m and 0.052 m; the shaft line 0.13 mm is the real 0.025 m.", "Paneellinie 0,2 mm und Schienenlinie 0,26 mm sind die realen 0,04 m und 0,052 m; die Wellenlinie 0,13 mm entspricht den realen 0,025 m."),
        "hatching": text("None.", "Keine."),
        "styles": text("Panels bold, track and frame light.", "Paneele kräftig, Schiene und Rahmen leicht."),
        "overlays": text("Cells: door line (panel and hardware merged), track frame, spring assembly.", "Zellen: Torlinie (Paneel und Beschläge zusammengefasst), Schienenrahmen, Federbaugruppe."),
    },
    "500": {
        "silhouette": text(
            "Four panel lines joined at the hinges on their true line in front of the roller path, one track line, the spring drum as one dot. " + CONVENTION_EN + " At 1:500 the front view is four panel bars stacked between two track lines with the spring shaft as a line and the drums; the plan is two track lines, the shaft line with the drums and the four panels as bars that fold with the door.",
            "Vier an den Scharnieren verbundene Paneellinien auf ihrer wahren Linie vor der Rollenbahn, eine Schienenlinie, die Federtrommel als Punkt. " + CONVENTION_DE + " Bei 1:500 ist die Vorderansicht vier gestapelte Paneelbalken zwischen zwei Schienenlinien mit der Federwelle als Linie und den Trommeln; die Draufsicht ist zwei Schienenlinien, die Wellenlinie mit den Trommeln und die vier Paneele als Balken, die mit dem Tor falten.",
        ),
        "retained": text("The 2.0 m door as four 0.5 m panels on the real quarter-circle track (R 0.5 m) and 2.1 m horizontal run, drum 0.18 m.", "Das 2,0 m hohe Tor als vier 0,5-m-Paneele auf der echten Viertelkreisschiene (R 0,5 m) mit 2,1 m Horizontalstück, Trommel 0,18 m."),
        "merged": text("Hinges, rollers and carriers are merged into the panel lines (regions listed); the sandwich panel and its steel hardware share one stroke.", "Scharniere, Rollen und Halter sind in die Paneellinien eingeschmolzen (Regionen aufgeführt); Sandwichpaneel und Stahlbeschläge teilen einen Strich."),
        "omitted": text("Hangers, brackets, coil, cable, seal, handle.", "Abhänger, Winkel, Feder, Seil, Dichtung, Griff."),
        "thinElements": text("A 0.04 m panel is a bar of its real 0.04 m in every view, outlined with the 0.13 mm pen, so it prints as one line; the track is a bar of its real 0.052 x 0.03 m in the same way, the spreader a line of 0.1 mm.", "Ein 0,04 m dickes Paneel ist in jeder Ansicht ein Balken mit seinen realen 0,04 m, mit dem 0,13-mm-Stift umrandet, es druckt also als eine Linie; die Schiene ist ebenso ein Balken mit ihren realen 0,052 x 0,03 m, die Querstrebe eine Linie von 0,1 mm."),
        "hatching": text("None.", "Keine."),
        "styles": text("Black lines for panels and track; the drum a filled dot.", "Schwarze Linien für Paneele und Schiene; die Trommel ein gefüllter Punkt."),
        "overlays": text("Cells: door line (all panels), track line, drum.", "Zellen: Torlinie (alle Paneele), Schienenlinie, Trommel."),
    },
}


def forward(bones):
    """🦴 The hinge points of a pose from its bone values (panel-1 rests at 90 degrees)."""
    x, y, a = bones["carriage-x"]["dx"], bones["carriage-y"]["dy"], 90.0
    pts = [(x, y)]
    for k in range(1, 5):
        a += bones[pb(k)]["angleDeg"]
        x, y = x + K.PANEL * math.cos(math.radians(a)), y + K.PANEL * math.sin(math.radians(a))
        pts.append((x, y))
    return pts


def apply(asset):
    """🔧 Writes every root fix into the asset in memory."""
    refs = asset["provenance"]["references"]
    extra = [
        {"id": "en-12604", "citation": "EN 12604:2017 Industrial, commercial and garage doors and gates, mechanical aspects, requirements and test methods (safeguarding of the section joints against finger trapping, safety against falling, cable and spring suspension)."},
        {"id": "door-hardware", "citation": "Sectional door hardware as sold by door and hardware manufacturers, typical dimensions consulted, not a table look-up: 2 inch roller track of 2 mm galvanised steel (channel about 52 x 30 mm with curled lips), nylon rollers 46 mm on 11 mm stems of about 110 mm, hinge leaves of 2.5 mm steel with the pin behind the inner face, jamb and flag brackets, punched angle 30 x 30 x 2.5 mm for hangers and braces, torsion shaft tube 25.4 mm (1 inch) in flanged bearings on end bearing plates, cast cable drums, torsion spring between a stationary cone on the anchor bracket and a winding cone; sandwich sections 40 to 42 mm with finger-protection joint, end stiles and a bottom rail with EPDM bulb seal."},
    ]
    for ref in extra:
        if all(r["id"] != ref["id"] for r in refs):
            refs.append(ref)
    asset["provenance"]["notes"] = text(
        "Panel poses are solved analytically: the bone line of every panel is its hinge line 20 mm behind the inner face; the bottom roller (80 mm above the bottom edge), the three hinge rollers and the top roller (60 mm below the top edge) lie on the track path, every hinge-to-hinge chord is 0.5 m. The spring shaft turns by the travel of the bottom roller over the 80 mm cable radius of the drum. Cable tension is 0.95 of the weight component along the panels, an authored estimate of a spring-balanced door.",
        "Die Paneelposen sind analytisch berechnet: die Knochenlinie jedes Paneels ist seine Scharnierlinie 20 mm hinter der Innenseite; die untere Rolle (80 mm über der Unterkante), die drei Scharnierrollen und die obere Rolle (60 mm unter der Oberkante) liegen auf der Schienenbahn, jede Scharnier-zu-Scharnier-Sehne misst 0,5 m. Die Federwelle dreht um den Weg der unteren Rolle über dem Seilradius der Trommel von 80 mm. Der Seilzug beträgt 0,95 der Gewichtskomponente entlang der Paneele, eine Schätzung für ein federausgeglichenes Tor.",
    )
    asset["dimensions"].update({"heightM": K.CEILING, "widthM": round(K.TRACK_END - K.WALL_X, 3), "depthM": 2.5})
    if all(m["id"] != "epdm" for m in asset["materials"]):
        asset["materials"].append({"id": "epdm", "title": text("EPDM rubber seal", "EPDM-Gummidichtung"), "emissivity": 0.9, "solarAbsorptivity": 0.9, "densityKgM3": 1150, "conductivityWmK": 0.25, "specificHeatJKgK": 1000, "source": "literature", "reference": "en-iso-10456"})
    for bone in asset["rig"]["bones"]:
        if bone["id"].startswith("panel-"):
            bone["mass"]["comY"] = round(K.V_IN + K.THICK / 2, 3)
        if bone["id"] == "spring-shaft":
            bone["x"], bone["y"] = K.SHAFT
        if bone["id"] == "carriage-x":
            bone["joint"]["minM"] = -0.05
    for anchor in asset["anchors"]:
        i = anchor["id"]
        if i.startswith("wind-"):
            anchor["y"] = K.V_OUT
        elif i == "cable-eye":
            anchor["x"], anchor["y"] = K.CABLE_EYE
            anchor["title"] = text("Cable stud under the bottom roller carrier", "Seilbolzen unter dem unteren Rollenhalter")
        elif i == "drum-cable":
            anchor["x"] = -K.CABLE_R
        elif i == "jamb-fix":
            anchor["x"], anchor["y"] = K.WALL_X, K.JAMB_H / 2
            anchor["title"] = text("Foot bracket of the vertical track anchored in the jamb", "Fußwinkel der Vertikalschiene im Pfosten verankert")
    for pose in asset["poses"]:
        s1 = K.path_s(forward(pose["bones"])[1])
        if abs(s1 - 0.5) < 2e-4:
            s1 = 0.5
        if abs(s1 - (K.ARC_C[1] + K.ARC_LEN + 0.5)) < 6e-4:
            s1 = K.ARC_C[1] + K.ARC_LEN + 0.5
        pose["bones"], _ = K.solve_pose(s1)
    for key, rep in asset["representations"].items():
        scale = int(key)
        rep["geometry"] = {"elevation": elevation(scale), "front": front(scale), "plan": plan(scale)}
        rep["decisions"] = DECISIONS[key]
    for region in asset["thermal"]["regions"]:
        if region["id"] == SP:
            region["areaBasis"] = "exchange"
        if region["id"] in AREAS:
            region["areaM2"] = AREAS[region["id"]]


if __name__ == "__main__":
    for arg in sys.argv[1:]:
        if arg.startswith("--area="):
            rid, value = arg[7:].split(":")
            AREAS[rid] = float(value)
    source = open(CATALOG, encoding="utf-8").read()
    document = json.loads(source)
    before = copy.deepcopy(document)
    apply(document["asset"])
    for pose in document["asset"]["poses"]:
        print(pose["id"], {k: list(v.values())[0] for k, v in pose["bones"].items()})
    if "--apply" in sys.argv:
        for key in MEMBERS:
            start, end = L.member(source, key)
            source = source[:start] + L.dumped(document["asset"][key]) + source[end:]
        check = json.loads(source)
        for key in check["asset"]:
            if key not in MEMBERS and check["asset"][key] != before["asset"][key]:
                raise SystemExit("member %s changed unexpectedly" % key)
            if key in MEMBERS and check["asset"][key] != document["asset"][key]:
                raise SystemExit("member %s not written as intended" % key)
        with open(CATALOG, "w", encoding="utf-8") as f:
            f.write(source)
        print("catalog members written:", ", ".join(MEMBERS))
    else:
        document["asset"]["volumes"] = json.load(open(os.path.join(T, "volumes", "components", "sectional-garage-door.json"), encoding="utf-8"))["volumes"]
        folder = os.path.join(T, "🗑️generated", "r3-sectional-garage-door")
        os.makedirs(folder, exist_ok=True)
        with open(os.path.join(folder, "🔣️.json"), "w", encoding="utf-8") as f:
            json.dump(document, f, ensure_ascii=False, indent=2)
        print("candidate written")
