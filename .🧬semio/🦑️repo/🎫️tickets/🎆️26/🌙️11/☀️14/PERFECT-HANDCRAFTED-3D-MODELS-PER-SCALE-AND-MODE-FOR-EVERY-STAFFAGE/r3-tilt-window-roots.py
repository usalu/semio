"""🏗 Root fixes of components/tilt-window: rig, anchors, dimensions, poses, clips, 2D drawings, symbols, texts, thermal areas.

Without flag: writes the candidate document to 🗑️generated/r3-tilt-window/🔣️.json (with the volumes source) for
`r3-validate.ts --file` and `r3-2d.ts --file`. With --apply: re-reads the catalog file, replaces only the members
listed in MEMBERS (and the manifest entry's heightM), keeps the file layout and verifies nothing else changed.
"""

import copy
import importlib.util
import json
import math
import os
import re
import sys

T = os.path.dirname(os.path.abspath(__file__))
R = os.path.abspath(os.path.join(T, *[".."] * 7))
S = os.path.join(R, "🎓️teaching", "🏛️architecture", "🧍️staffage")
CATALOG = os.path.join(S, "🗂️catalog", "🧱️components", "tilt-window", "🔣️.json")
MANIFEST = os.path.join(S, "🗂️catalog", "🔣️.json")
spec = importlib.util.spec_from_file_location("kit", os.path.join(T, "r3-tilt-window-kit.py"))
k = importlib.util.module_from_spec(spec)
spec.loader.exec_module(k)

MEMBERS = ["summary", "provenance", "dimensions", "rig", "anchors", "representations", "symbols", "poses", "clips", "thermal", "structure"]
AREAS = {"pane-glass": 0.47, "pane-edge": 0.2, "frame-timber": None, "frame-sill": None}
r = k.r


def tx(en, de):
    return {"en": en, "de": de}


def shape(id, bone, role, z, filled, tone, geometry, region=None, material=None, hatch=None, pen=None, solid=None, regions=None, follow=False):
    s = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    if region is not None:
        s["region"] = region
    if regions is not None:
        s["regions"] = regions
    if material is not None:
        s["material"] = material
    if hatch is not None:
        s["hatch"] = hatch
    if pen is not None:
        s["penMm"] = pen
    s.update(geometry)
    if solid is not None:
        s["solid"] = {"kind": "extruded", "z0": r(solid[0]), "z1": r(solid[1]), "sweep": "w"}
    if follow:
        s["followTip"] = True
    return s


def rect(x0, y0, x1, y1):
    return {"kind": "rect", "x": r(min(x0, x1)), "y": r(min(y0, y1)), "w": r(abs(x1 - x0)), "h": r(abs(y1 - y0))}


def poly(points, closed=True):
    return {"kind": "poly", "points": [[r(a), r(b)] for a, b in points], "closed": closed}


def ellipse(cx, cy, rad):
    return {"kind": "ellipse", "cx": r(cx), "cy": r(cy), "rx": r(rad), "ry": r(rad)}


def sash_poly(points):
    return poly([k.sash_pt(X, Y) for X, Y in points])


def sill_poly(points):
    return poly([k.sill_pt(X, Y) for X, Y in points])


def sash_rect(Xa, Ya, Xb, Yb):
    return rect(k.su(Ya), k.sv(Xa), k.su(Yb), k.sv(Xb))


def sill_rect(Xa, Ya, Xb, Yb):
    return rect(k.lu(Xa), k.lv(Ya), k.lu(Xb), k.lv(Yb))


TIMBER = dict(region="frame-timber", material="timber")
SILL = dict(region="frame-sill", material="timber")
STEEL = dict(material="steel")
SZ = k.SASH_Z
SIGHT_Z = k.SIGHT_Z
STOP_Z = k.Z_IN - k.STOP_W


def glass_lines(scale_pen):
    out = []
    for name, xc in (("inner", k.GLASS_X0 + k.PANE / 2), ("outer", k.LIP_X - k.PANE / 2)):
        v = k.sv(xc)
        for part, ya, yb, region in (("edge-bottom", k.GLASS_Y0, k.CENTRE_Y0, "pane-edge"), ("centre", k.CENTRE_Y0, k.CENTRE_Y1, "pane-glass"), ("edge-top", k.CENTRE_Y1, k.GLASS_Y1, "pane-edge")):
            out.append(shape("pane-%s-%s" % (name, part), "sash", "glazing", 4.2, False, 0.9, poly([[k.su(ya), v], [k.su(yb), v]], False), region=region, pen=scale_pen))
    return out


def drawing50():
    hx = "timber-cross"
    seal_x0, seal_x1 = k.GLASS_X0 + k.PANE, k.LIP_X - k.PANE
    elevation = [
        shape("jamb-profile", "sill", "body", 0.5, True, 0.75, sill_rect(k.X0, 0.0, k.X1, k.H), **TIMBER),
        shape("jamb-stop-far", "sill", "detail", 0.7, True, 0.62, sill_rect(k.STOP_X0, 0.05, k.STOP_X1, k.HEAD_Y0), **TIMBER),
        shape("stay-arc-far", "sill", "detail", 0.8, True, 0.35, sill_poly(k.arc_bar()), **STEEL),
        shape("stay-slot-far", "sill", "detail", 0.85, False, 0.2, sill_poly(k.arc_slot())),
        shape("sill", "sill", "body", 1, True, 0.6, sill_poly(k.sill_section()), hatch=hx, **SILL),
        shape("head", "sill", "body", 1, True, 0.6, sill_rect(k.X0, k.HEAD_Y0, k.X1, k.H), hatch=hx, **TIMBER),
        shape("sill-stop", "sill", "body", 1.2, True, 0.6, sill_rect(k.STOP_X0, 0.05, k.STOP_X1, k.STOP_SILL_Y1), hatch=hx, **TIMBER),
        shape("head-stop", "sill", "body", 1.2, True, 0.6, sill_rect(k.STOP_X0, k.STOP_HEAD_Y0, k.STOP_X1, k.HEAD_Y0), hatch=hx, **TIMBER),
        shape("hinge-leaf-frame", "sill", "detail", 1.5, True, 0.3, sill_rect(k.HX - 0.06, 0.05, k.HX, 0.05 + k.LEAF), **STEEL),
        shape("stile-far", "sash", "body", 2, True, 0.66, sash_rect(k.SASH_X0, k.SASH_Y0, k.SASH_X1, k.SASH_Y1), **TIMBER),
        shape("bead-far", "sash", "detail", 2.1, False, 0.5, sash_poly([(k.SASH_X0, k.SIGHT_Y0), (k.SASH_X0, k.SIGHT_Y1)]) | {"closed": False}, **TIMBER),
        shape("hinge-leaf-sash", "sash", "detail", 2.2, True, 0.3, sash_rect(k.HX, k.SASH_Y0, k.SASH_X0, k.SASH_Y0 + 0.06), **STEEL),
        shape("stay-pin-far", "sash", "detail", 2.4, True, 0.25, ellipse(k.KEEPER_U, k.KEEPER_V, k.KEEPER_R), **STEEL),
        shape("rail-bottom", "sash", "body", 3, True, 0.6, sash_poly(k.rail_section(k.SASH_Y0, k.SIGHT_Y0, True)), hatch=hx, **TIMBER),
        shape("rail-top", "sash", "body", 3, True, 0.6, sash_poly(k.rail_section(k.SIGHT_Y1, k.SASH_Y1, False)), hatch=hx, **TIMBER),
        shape("bead-bottom", "sash", "body", 3, True, 0.6, sash_poly(k.bead_section(False)), hatch=hx, **TIMBER),
        shape("bead-top", "sash", "body", 3, True, 0.6, sash_poly(k.bead_section(True)), hatch=hx, **TIMBER),
        shape("weather-bar", "sash", "body", 3, True, 0.6, sash_poly(k.WEATHER), hatch=hx, **TIMBER),
        shape("spacer-bottom", "sash", "body", 4, True, 0.3, sash_rect(seal_x0, k.GLASS_Y0, seal_x1, k.GLASS_Y0 + k.SEAL), region="pane-edge", material="edge-zone", pen=0.1),
        shape("spacer-top", "sash", "body", 4, True, 0.3, sash_rect(seal_x0, k.GLASS_Y1 - k.SEAL, seal_x1, k.GLASS_Y1), region="pane-edge", material="edge-zone", pen=0.1),
    ] + glass_lines(0.1) + [
        shape("hinge-pin", "sash", "detail", 5, True, 0.3, ellipse(0, 0, k.HINGE_R), **STEEL),
        shape("grip-post", "sash", "detail", 5, True, 0.3, sash_rect(k.SASH_X0, k.GRIP_Y - k.GRIP_R, k.SASH_X0 - k.GRIP_REACH, k.GRIP_Y + k.GRIP_R), **STEEL),
        shape("grip-bar", "sash", "detail", 5.1, True, 0.2, ellipse(k.su(k.GRIP_Y), k.sv(k.SASH_X0 - k.GRIP_REACH), k.GRIP_R), **STEEL),
    ]
    front = [
        shape("sill", "sill", "body", 1, True, 0.6, rect(-k.Z_IN, 0.0, k.Z_IN, 0.05), **SILL),
        shape("sill-weathering", "sill", "detail", 1.5, False, 0.35, poly([[-k.Z_IN, 0.05 - k.SILL_DROP], [k.Z_IN, 0.05 - k.SILL_DROP]], False), region="frame-sill"),
        shape("head", "sill", "body", 1, True, 0.6, rect(-k.Z_IN, k.HEAD_Y0, k.Z_IN, k.H), **TIMBER),
        shape("jamb-left", "sill", "body", 2, True, 0.6, rect(k.Z_IN, 0.0, k.Z_OUT, k.H), **TIMBER),
        shape("jamb-right", "sill", "body", 2, True, 0.6, rect(-k.Z_OUT, 0.0, -k.Z_IN, k.H), **TIMBER),
        shape("stile-left", "sash", "body", 3, True, 0.66, sash_rect(k.SASH_X0, k.SASH_Y0, k.SASH_X1, k.SASH_Y1), solid=(SIGHT_Z, SZ), **TIMBER),
        shape("stile-right", "sash", "body", 3, True, 0.66, sash_rect(k.SASH_X0, k.SASH_Y0, k.SASH_X1, k.SASH_Y1), solid=(-SZ, -SIGHT_Z), **TIMBER),
        shape("rail-bottom", "sash", "body", 3, True, 0.66, sash_rect(k.SASH_X0, k.SASH_Y0, k.SASH_X1, k.SIGHT_Y0), solid=(-SIGHT_Z, SIGHT_Z), **TIMBER),
        shape("rail-top", "sash", "body", 3, True, 0.66, sash_rect(k.SASH_X0, k.SIGHT_Y1, k.SASH_X1, k.SASH_Y1), solid=(-SIGHT_Z, SIGHT_Z), **TIMBER),
        shape("weather-bar", "sash", "body", 3.2, True, 0.55, sash_poly(k.WEATHER), solid=(-k.WEATHER_Z, k.WEATHER_Z), **TIMBER),
        shape("pane", "sash", "glazing", 2.9, True, 0.9, sash_rect(k.GLASS_X0, k.GLASS_Y0, k.LIP_X, k.GLASS_Y1), solid=(-k.GLASS_Z, k.GLASS_Z), region="pane-glass", material="glass"),
        shape("pane-spacer", "sash", "glazing", 2.95, False, 0.5, sash_rect(k.GLASS_X0 + k.PANE, k.GLASS_Y0 + k.SEAL, k.LIP_X - k.PANE, k.GLASS_Y1 - k.SEAL), solid=(-(k.GLASS_Z - k.SEAL), k.GLASS_Z - k.SEAL), region="pane-edge", material="edge-zone"),
        shape("glass-reflection-a", "sash", "texture", 3.6, False, 0.35, poly([[-0.2, 0.3], [0.1, 0.9]], False), region="pane-glass", follow=True),
        shape("glass-reflection-b", "sash", "texture", 3.6, False, 0.35, poly([[-0.1, 0.25], [0.2, 0.85]], False), region="pane-glass", follow=True),
        shape("jamb-stop-left", "sill", "body", 4, True, 0.55, rect(STOP_Z, 0.05, k.Z_IN, k.HEAD_Y0), **TIMBER),
        shape("jamb-stop-right", "sill", "body", 4, True, 0.55, rect(-k.Z_IN, 0.05, -STOP_Z, k.HEAD_Y0), **TIMBER),
        shape("sill-stop", "sill", "body", 4, True, 0.55, rect(-STOP_Z, 0.05, STOP_Z, k.STOP_SILL_Y1), **TIMBER),
        shape("head-stop", "sill", "body", 4, True, 0.55, rect(-STOP_Z, k.STOP_HEAD_Y0, STOP_Z, k.HEAD_Y0), **TIMBER),
    ]
    plan = [
        shape("sill", "sill", "body", 1, True, 0.6, rect(k.X0, -k.Z_IN, k.X1, k.Z_IN), **SILL),
        shape("jamb-left", "sill", "body", 2, True, 0.6, rect(k.X0, k.Z_IN, k.X1, k.Z_OUT), **TIMBER),
        shape("jamb-right", "sill", "body", 2, True, 0.6, rect(k.X0, -k.Z_OUT, k.X1, -k.Z_IN), **TIMBER),
        shape("stile-left", "sash", "body", 3, True, 0.66, sash_rect(k.SASH_X0, k.SASH_Y0, k.SASH_X1, k.SASH_Y1), solid=(SIGHT_Z, SZ), **TIMBER),
        shape("stile-right", "sash", "body", 3, True, 0.66, sash_rect(k.SASH_X0, k.SASH_Y0, k.SASH_X1, k.SASH_Y1), solid=(-SZ, -SIGHT_Z), **TIMBER),
        shape("rail-bottom", "sash", "body", 3, True, 0.66, sash_rect(k.SASH_X0, k.SASH_Y0, k.SASH_X1, k.SIGHT_Y0), solid=(-SIGHT_Z, SIGHT_Z), **TIMBER),
        shape("rail-top", "sash", "body", 3, True, 0.66, sash_rect(k.SASH_X0, k.SIGHT_Y1, k.SASH_X1, k.SASH_Y1), solid=(-SIGHT_Z, SIGHT_Z), **TIMBER),
        shape("weather-bar", "sash", "body", 3.2, True, 0.55, sash_poly(k.WEATHER), solid=(-k.WEATHER_Z, k.WEATHER_Z), **TIMBER),
        shape("pane", "sash", "glazing", 2.9, True, 0.9, sash_rect(k.GLASS_X0, k.GLASS_Y0, k.LIP_X, k.GLASS_Y1), solid=(-k.GLASS_Z, k.GLASS_Z), region="pane-glass", material="glass"),
        shape("grip", "sash", "detail", 3.5, True, 0.3, sash_rect(k.SASH_X0, k.GRIP_Y - k.GRIP_R, k.SASH_X0 - k.GRIP_REACH - k.GRIP_R, k.GRIP_Y + k.GRIP_R), solid=(-k.GRIP_HALF - k.GRIP_R, k.GRIP_HALF + k.GRIP_R), **STEEL),
        shape("head", "sill", "body", 6, True, 0.6, rect(k.X0, -k.Z_IN, k.X1, k.Z_IN), **TIMBER),
    ]
    return {"elevation": elevation, "front": front, "plan": plan}


def drawing100():
    elevation = [
        shape("jamb-profile", "sill", "body", 0.5, True, 0.75, sill_rect(k.X0, 0.0, k.X1, k.H), **TIMBER),
        shape("sill", "sill", "body", 1, True, 0.6, sill_rect(k.X0, 0.0, k.X1, 0.05), **SILL),
        shape("head", "sill", "body", 1, True, 0.6, sill_rect(k.X0, k.HEAD_Y0, k.X1, k.H), **TIMBER),
        shape("sill-stop", "sill", "body", 1.2, True, 0.6, sill_rect(k.STOP_X0, 0.05, k.STOP_X1, k.STOP_SILL_Y1), **TIMBER),
        shape("head-stop", "sill", "body", 1.2, True, 0.6, sill_rect(k.STOP_X0, k.STOP_HEAD_Y0, k.STOP_X1, k.HEAD_Y0), **TIMBER),
        shape("rail-bottom", "sash", "body", 3, True, 0.6, sash_rect(k.SASH_X0, k.SASH_Y0, k.SASH_X1, k.SIGHT_Y0), pen=0.13, **TIMBER),
        shape("rail-top", "sash", "body", 3, True, 0.6, sash_rect(k.SASH_X0, k.SIGHT_Y1, k.SASH_X1, k.SASH_Y1), pen=0.13, **TIMBER),
        shape("weather-bar", "sash", "body", 3, True, 0.6, sash_rect(k.SASH_X1, k.STOP_SILL_Y1, 0.083, k.SIGHT_Y0), pen=0.13, **TIMBER),
        shape("glass-edge-bottom", "sash", "glazing", 4, True, 0.8, sash_rect(k.GLASS_X0, k.SIGHT_Y0, k.LIP_X, k.CENTRE_Y0), region="pane-edge", material="glass", pen=0.1),
        shape("glass-centre", "sash", "glazing", 4, True, 0.92, sash_rect(k.GLASS_X0, k.CENTRE_Y0, k.LIP_X, k.CENTRE_Y1), region="pane-glass", material="glass", pen=0.1),
        shape("glass-edge-top", "sash", "glazing", 4, True, 0.8, sash_rect(k.GLASS_X0, k.CENTRE_Y1, k.LIP_X, k.SIGHT_Y1), region="pane-edge", material="glass", pen=0.1),
        shape("hinge-pin", "sash", "detail", 5, True, 0.3, ellipse(0, 0, k.HINGE_R), **STEEL),
    ]
    front = [
        shape("sill", "sill", "body", 1, True, 0.6, rect(-k.Z_IN, 0.0, k.Z_IN, 0.05), **SILL),
        shape("head", "sill", "body", 1, True, 0.6, rect(-k.Z_IN, k.HEAD_Y0, k.Z_IN, k.H), **TIMBER),
        shape("jamb-left", "sill", "body", 2, True, 0.6, rect(k.Z_IN, 0.0, k.Z_OUT, k.H), **TIMBER),
        shape("jamb-right", "sill", "body", 2, True, 0.6, rect(-k.Z_OUT, 0.0, -k.Z_IN, k.H), **TIMBER),
        shape("stile-left", "sash", "body", 3, True, 0.66, sash_rect(k.SASH_X0, k.SASH_Y0, k.SASH_X1, k.SASH_Y1), solid=(SIGHT_Z, SZ), **TIMBER),
        shape("stile-right", "sash", "body", 3, True, 0.66, sash_rect(k.SASH_X0, k.SASH_Y0, k.SASH_X1, k.SASH_Y1), solid=(-SZ, -SIGHT_Z), **TIMBER),
        shape("rail-bottom", "sash", "body", 3, True, 0.66, sash_rect(k.SASH_X0, k.SASH_Y0, k.SASH_X1, k.SIGHT_Y0), solid=(-SIGHT_Z, SIGHT_Z), **TIMBER),
        shape("rail-top", "sash", "body", 3, True, 0.66, sash_rect(k.SASH_X0, k.SIGHT_Y1, k.SASH_X1, k.SASH_Y1), solid=(-SIGHT_Z, SIGHT_Z), **TIMBER),
        shape("weather-bar", "sash", "body", 3.2, True, 0.55, sash_rect(k.SASH_X1, k.STOP_SILL_Y1, 0.083, k.SIGHT_Y0), solid=(-k.WEATHER_Z, k.WEATHER_Z), **TIMBER),
        shape("pane", "sash", "glazing", 2.9, True, 0.9, sash_rect(k.GLASS_X0, k.SIGHT_Y0, k.LIP_X, k.SIGHT_Y1), solid=(-SIGHT_Z, SIGHT_Z), region="pane-glass", material="glass"),
        shape("glass-reflection-a", "sash", "texture", 3.6, False, 0.35, poly([[-0.2, 0.3], [0.1, 0.9]], False), region="pane-glass", follow=True),
        shape("stop-left", "sill", "body", 4, True, 0.55, rect(STOP_Z, 0.05, k.Z_IN, k.HEAD_Y0), **TIMBER),
        shape("stop-right", "sill", "body", 4, True, 0.55, rect(-k.Z_IN, 0.05, -STOP_Z, k.HEAD_Y0), **TIMBER),
        shape("stop-bottom", "sill", "body", 4, True, 0.55, rect(-STOP_Z, 0.05, STOP_Z, k.STOP_SILL_Y1), **TIMBER),
        shape("stop-top", "sill", "body", 4, True, 0.55, rect(-STOP_Z, k.STOP_HEAD_Y0, STOP_Z, k.HEAD_Y0), **TIMBER),
    ]
    plan = [
        shape("sill", "sill", "body", 1, True, 0.6, rect(k.X0, -k.Z_IN, k.X1, k.Z_IN), **SILL),
        shape("jamb-left", "sill", "body", 2, True, 0.6, rect(k.X0, k.Z_IN, k.X1, k.Z_OUT), **TIMBER),
        shape("jamb-right", "sill", "body", 2, True, 0.6, rect(k.X0, -k.Z_OUT, k.X1, -k.Z_IN), **TIMBER),
        shape("stile-left", "sash", "body", 3, True, 0.66, sash_rect(k.SASH_X0, k.SASH_Y0, k.SASH_X1, k.SASH_Y1), solid=(SIGHT_Z, SZ), **TIMBER),
        shape("stile-right", "sash", "body", 3, True, 0.66, sash_rect(k.SASH_X0, k.SASH_Y0, k.SASH_X1, k.SASH_Y1), solid=(-SZ, -SIGHT_Z), **TIMBER),
        shape("rail-bottom", "sash", "body", 3, True, 0.66, sash_rect(k.SASH_X0, k.SASH_Y0, k.SASH_X1, k.SIGHT_Y0), solid=(-SIGHT_Z, SIGHT_Z), **TIMBER),
        shape("rail-top", "sash", "body", 3, True, 0.66, sash_rect(k.SASH_X0, k.SIGHT_Y1, k.SASH_X1, k.SASH_Y1), solid=(-SIGHT_Z, SIGHT_Z), **TIMBER),
        shape("weather-bar", "sash", "body", 3.2, True, 0.55, sash_rect(k.SASH_X1, k.STOP_SILL_Y1, 0.083, k.SIGHT_Y0), solid=(-k.WEATHER_Z, k.WEATHER_Z), **TIMBER),
        shape("pane", "sash", "glazing", 2.9, True, 0.9, sash_rect(k.GLASS_X0, k.SIGHT_Y0, k.LIP_X, k.SIGHT_Y1), solid=(-SIGHT_Z, SIGHT_Z), region="pane-glass", material="glass"),
        shape("head", "sill", "body", 6, True, 0.6, rect(k.X0, -k.Z_IN, k.X1, k.Z_IN), **TIMBER),
    ]
    return {"elevation": elevation, "front": front, "plan": plan}


def drawing200():
    elevation = [
        shape("jamb-profile", "sill", "body", 0.5, True, 0.75, sill_rect(k.X0, 0.0, k.X1, k.H), **TIMBER),
        shape("sill", "sill", "body", 1, True, 0.6, sill_rect(k.X0, 0.0, k.X1, 0.05), pen=0.13, **SILL),
        shape("head", "sill", "body", 1, True, 0.6, sill_rect(k.X0, k.HEAD_Y0, k.X1, k.H), pen=0.13, **TIMBER),
        shape("rail-bottom", "sash", "body", 3, True, 0.6, sash_rect(k.SASH_X0, k.SASH_Y0, k.SASH_X1, k.SIGHT_Y0), pen=0.1, **TIMBER),
        shape("rail-top", "sash", "body", 3, True, 0.6, sash_rect(k.SASH_X0, k.SIGHT_Y1, k.SASH_X1, k.SASH_Y1), pen=0.1, **TIMBER),
        shape("glass", "sash", "glazing", 4, False, 0.92, poly([[k.su(k.SIGHT_Y0), k.sv(k.GLASS_X0 + 0.012)], [k.su(k.SIGHT_Y1), k.sv(k.GLASS_X0 + 0.012)]], False), regions=["pane-glass", "pane-edge"], material="glass", pen=0.1),
    ]
    front = [
        shape("sill", "sill", "body", 1, True, 0.6, rect(-k.Z_IN, 0.0, k.Z_IN, 0.05), **SILL),
        shape("head", "sill", "body", 1, True, 0.6, rect(-k.Z_IN, k.HEAD_Y0, k.Z_IN, k.H), **TIMBER),
        shape("jamb-left", "sill", "body", 2, True, 0.6, rect(k.Z_IN, 0.0, k.Z_OUT, k.H), **TIMBER),
        shape("jamb-right", "sill", "body", 2, True, 0.6, rect(-k.Z_OUT, 0.0, -k.Z_IN, k.H), **TIMBER),
        shape("sash-frame", "sash", "body", 3, True, 0.66, sash_rect(k.SASH_X0, k.SASH_Y0, k.SASH_X1, k.SASH_Y1), solid=(-SZ, SZ), **TIMBER),
        shape("pane", "sash", "glazing", 3.5, True, 0.9, sash_rect(k.GLASS_X0, k.SIGHT_Y0, k.LIP_X, k.SIGHT_Y1), solid=(-SIGHT_Z, SIGHT_Z), region="pane-glass", material="glass"),
    ]
    plan = [
        shape("sill", "sill", "body", 1, True, 0.6, rect(k.X0, -k.Z_IN, k.X1, k.Z_IN), **SILL),
        shape("jamb-left", "sill", "body", 2, True, 0.6, rect(k.X0, k.Z_IN, k.X1, k.Z_OUT), **TIMBER),
        shape("jamb-right", "sill", "body", 2, True, 0.6, rect(k.X0, -k.Z_OUT, k.X1, -k.Z_IN), **TIMBER),
        shape("head", "sill", "body", 6, True, 0.6, rect(k.X0, -k.Z_IN, k.X1, k.Z_IN), **TIMBER),
    ]
    return {"elevation": elevation, "front": front, "plan": plan}


TEXTS = {
    "50": {
        "silhouette": tx("A vertical section on the centre line of a bottom-hung timber sash in a 300 mm timber lining frame: cut and cross-hatched the sill board with its weathered top and drip groove, the head board, the two exterior stop strips, both sash rails with their room-side glass rebates, the chamfered glazing beads and the weather bar; the insulating unit as two pane lines with its edge seals; beyond the cut the far jamb with its stop strip, the far stile, the hinge leaves and the hinge pin on the room-side bottom corner, the quadrant stay on the jamb with its slot and pin, and the pull grip on the top rail. Front (exterior face): lining boards, stop strips over the sash edges, stiles, rails, weather bar, glass, the edge-seal line and two reflection lines. Plan (from above): the head board over the sash with the jambs; the sash, its weather bar and the grip extruded from their elevation profiles, so they appear inside the room when the sash tilts.", "Ein Vertikalschnitt auf der Mittelachse eines unten angeschlagenen Holzkippflügels in einem 300 mm tiefen Holzfutterrahmen: geschnitten und kreuzschraffiert das Sohlbrett mit Wetterschräge und Tropfnut, das Kopfbrett, die zwei äußeren Anschlagleisten, beide Flügelriegel mit raumseitigem Glasfalz, die gefasten Glasleisten und der Wetterschenkel; die Isolierverglasung als zwei Scheibenlinien mit Randverbund; hinter dem Schnitt das ferne Futterbrett mit seiner Anschlagleiste, das ferne Flügelholz, die Bandlappen und der Bandstift an der raumseitigen unteren Ecke, der Kippbogen am Futter mit Schlitz und Zapfen und der Ziehgriff am oberen Riegel. Ansicht (Außenseite): Futterbretter, Anschlagleisten über den Flügelkanten, Flügelhölzer, Riegel, Wetterschenkel, Glas, die Linie des Randverbunds und zwei Spiegellinien. Draufsicht: das Kopfbrett über dem Flügel mit den Futterbrettern; Flügel, Wetterschenkel und Griff aus ihren Schnittprofilen extrudiert, so dass sie beim Kippen im Raum erscheinen."),
        "retained": tx("Lining boards 50 mm (1 mm), stop strips 25 x 30 mm, sash 90 mm deep with stiles and top rail 70 mm and bottom rail 80 mm, 18 mm rebates, beads 26 x 18 mm with 8 mm chamfer, weather bar 38 mm proud, 12 mm rebate clearance all round, drip groove 8 x 10 mm, hinge pin 16 mm, stay arc 24 x 8 mm with a 10 mm slot, grip bar 12 mm.", "Futterbretter 50 mm (1 mm), Anschlagleisten 25 x 30 mm, Flügel 90 mm tief mit Flügelhölzern und oberem Riegel 70 mm und unterem Riegel 80 mm, Falze 18 mm, Glasleisten 26 x 18 mm mit 8 mm Fase, Wetterschenkel 38 mm vorstehend, 12 mm Falzluft rundum, Tropfnut 8 x 10 mm, Bandstift 16 mm, Kippbogen 24 x 8 mm mit 10 mm Schlitz, Griffstange 12 mm."),
        "merged": tx("Each 4 mm pane is a line; the argon gap is the space between the lines; the spring catch is mortised into the top rail and not visible.", "Jede 4-mm-Scheibe ist eine Linie; der Argonraum ist der Zwischenraum; der Schnäpper ist in den oberen Riegel eingelassen und nicht sichtbar."),
        "omitted": tx("Gaskets, setting blocks, screws, the Euro groove (hidden in the rebate clearance) and the near stay (in front of the section plane).", "Dichtungen, Klötze, Schrauben, die Beschlagnut (in der Falzluft verborgen) und der nahe Kippbogen (vor der Schnittebene)."),
        "thinElements": tx("The 4 mm panes are 0.1 mm pen lines 20 mm apart (0.4 mm); the edge seals are 0.32 x 0.24 mm blocks.", "Die 4-mm-Scheiben sind 0,1-mm-Linien im Abstand von 20 mm (0,4 mm); die Randverbünde sind 0,32 x 0,24 mm große Blöcke."),
        "hatching": tx("Cut timber is cross-hatched at 45 degrees, 0.8 mm (DIN ISO 128-50 timber across the grain); glass stays unhatched; parts beyond the cut are toned, not hatched.", "Geschnittenes Holz ist 45° kreuzschraffiert, 0,8 mm (DIN ISO 128-50 Holz quer zur Faser); Glas bleibt ungeschraffiert; Teile hinter dem Schnitt sind getönt, nicht schraffiert."),
        "styles": tx("Outline 0.35 mm, details 0.18 mm, glazing 0.13 mm, hatch 0.1 mm; steel hardware dark.", "Kontur 0,35 mm, Details 0,18 mm, Verglasung 0,13 mm, Schraffur 0,1 mm; Stahlbeschläge dunkel."),
        "overlays": tx("Lining, sill, sash timber, glass edge zones and glass centre are separate overlay cells; the pane lines are bound to the zone they lie in.", "Futter, Sohlbrett, Flügelholz, Glasrandzonen und Glasmitte sind getrennte Überlagerungszellen; die Scheibenlinien gehören zur Zone, in der sie liegen."),
    },
    "100": {
        "silhouette": tx("A vertical section of the sash in its lining: sill and head boards with the two stop strips, the two rail blocks (beads merged), the weather bar block and the 24 mm glass band between the rails, split into centre and edge zones, with the hinge pin. Front: lining boards, the stop strips, stiles, rails, weather bar and pane extruded from the sash profile, one reflection line. Plan: the head board over the sash, the sash parts extruded so that they appear inside the room when it tilts.", "Ein Vertikalschnitt des Flügels im Futter: Sohl- und Kopfbrett mit den zwei Anschlagleisten, die zwei Riegelblöcke (Glasleisten zusammengefasst), der Wetterschenkel als Block und das 24-mm-Glasband zwischen den Riegeln, geteilt in Mitte und Randzonen, mit dem Bandstift. Ansicht: Futterbretter, Anschlagleisten, Flügelhölzer, Riegel, Wetterschenkel und Scheibe aus dem Flügelprofil extrudiert, eine Spiegellinie. Draufsicht: das Kopfbrett über dem Flügel, die Flügelteile extrudiert, so dass sie beim Kippen im Raum erscheinen."),
        "retained": tx("Boards (0.5 mm), stops (0.25 x 0.3 mm), rails (0.9 x 0.8 / 0.7 mm), weather bar (0.4 x 0.6 mm), glass band 0.24 mm with 0.65 mm edge zones, hinge pin 0.16 mm.", "Bretter (0,5 mm), Anschläge (0,25 x 0,3 mm), Riegel (0,9 x 0,8 / 0,7 mm), Wetterschenkel (0,4 x 0,6 mm), Glasband 0,24 mm mit 0,65-mm-Randzonen, Bandstift 0,16 mm."),
        "merged": tx("Panes and cavity are one glass band; beads and rebates are merged into the rails; the weather bar is a plain block.", "Scheiben und Gasraum sind ein Glasband; Glasleisten und Falze sind in den Riegeln zusammengefasst; der Wetterschenkel ist ein einfacher Block."),
        "omitted": tx("Pull grip, quadrant stays, hinge leaves, edge seals, drip groove and sill weathering (all below 15 mm).", "Ziehgriff, Kippbögen, Bandlappen, Randverbund, Tropfnut und Wetterschräge (alle unter 15 mm)."),
        "thinElements": tx("The glass band is a 0.24 mm filled strip with a 0.1 mm outline.", "Das Glasband ist ein 0,24 mm breiter gefüllter Streifen mit 0,1 mm Kontur."),
        "hatching": tx("No hatch: the cut timber is at most 1 mm wide, below the printable hatch spacing.", "Keine Schraffur: das geschnittene Holz ist höchstens 1 mm breit und unter dem druckbaren Schraffurabstand."),
        "styles": tx("Cut timber dark, glass light with darker edge zones.", "Geschnittenes Holz dunkel, Glas hell mit dunkleren Randzonen."),
        "overlays": tx("Glass centre and the two edge zones are separate overlay cells, so the colder centre and the warmer spacer edge read differently.", "Glasmitte und die beiden Randzonen sind getrennte Überlagerungszellen, so lesen sich die kältere Mitte und der wärmere Abstandhalterrand verschieden."),
    },
    "200": {
        "silhouette": tx("A section of the lining with sill and head boards and of the sash as two small rail blocks at the ends of one glass line. Front: lining boards fixed, the sash as one frame with the pane, both extruded from the sash profile, so the sash shortens as it tilts. Plan: the head board and the jambs; the sash lies under the head board and shows beyond it only as a strip of at most 21 mm at the stay stop (0.1 mm at 1:200), so the plan does not draw it.", "Ein Schnitt des Futters mit Sohl- und Kopfbrett und des Flügels als zwei kleine Riegelblöcke an den Enden einer Glaslinie. Ansicht: Futterbretter fest, der Flügel als ein Rahmen mit der Scheibe, beide aus dem Flügelprofil extrudiert, so dass der Flügel beim Kippen kürzer wird. Draufsicht: Kopfbrett und Futterbretter; der Flügel liegt unter dem Kopfbrett und zeigt sich am Anschlag der Kippbögen nur als Streifen von höchstens 21 mm (0,1 mm bei 1:200), daher zeichnet die Draufsicht ihn nicht."),
        "retained": tx("Boards (0.25 mm), the two sash rails (0.45 x 0.4 / 0.35 mm) and the glass line between them.", "Bretter (0,25 mm), die beiden Flügelriegel (0,45 x 0,4 / 0,35 mm) und die Glaslinie dazwischen."),
        "merged": tx("Both panes, the cavity and the edge seal are one glass line; glass centre and edge zones share it; stop strips and weather bar are merged into the boards and rails.", "Beide Scheiben, Gasraum und Randverbund sind eine Glaslinie; Glasmitte und Randzonen teilen sie; Anschlagleisten und Wetterschenkel sind in Bretter und Riegel übergegangen."),
        "omitted": tx("Hinges, stays, grip, beads, seals and the drip groove.", "Bänder, Kippbögen, Griff, Glasleisten, Dichtungen und Tropfnut."),
        "thinElements": tx("The 24 mm glass is a line of 0.1 mm pen; the rails have 0.1 mm outlines.", "Das 24-mm-Glas ist eine Linie mit 0,1 mm Feder; die Riegel haben 0,1-mm-Konturen."),
        "hatching": tx("No hatch: the blocks are too small.", "Keine Schraffur: die Blöcke sind zu klein."),
        "styles": tx("Dark timber blocks, a thin light glass line.", "Dunkle Holzblöcke, eine dünne helle Glaslinie."),
        "overlays": tx("The glass line carries merged centre and edge values; rails and boards are separate cells.", "Die Glaslinie trägt zusammengefasste Werte für Mitte und Rand; Riegel und Bretter sind getrennte Zellen."),
    },
}


def symbols(asset):
    out = copy.deepcopy(asset["symbols"])
    h, sy0, sy1, sz, gx = r(k.H), r(k.SASH_Y0), r(k.SASH_Y1), r(k.SASH_Z), r(k.GLASS_X0 + 0.012)
    for sym in out:
        g = sym["geometry"]
        for s in g["elevation"]:
            if s["id"] == "window-outline":
                s["points"] = [[-0.15, 0], [0.15, 0], [0.15, h], [-0.15, h]]
            if s["id"] == "sash-line":
                s["points"] = [[gx, sy0], [gx, sy1]]
            if s["id"] == "tilt-arrow":
                s["points"] = [[gx, 1.15], [-0.12, 1.15]]
        for s in g["front"]:
            if s["id"] == "sill-block":
                s.update({"x": -k.Z_IN, "w": 2 * k.Z_IN})
            if s["id"] == "window-outline":
                s["h"] = h
            if s["id"] == "sash-outline":
                s.update({"x": -sz, "y": sy0, "w": r(2 * sz), "h": r(k.SASH_H)})
            if s["id"] == "tilt-line-a":
                s["points"] = [[-sz, sy1], [0, sy0]]
            if s["id"] == "tilt-line-b":
                s["points"] = [[sz, sy1], [0, sy0]]
        for s in g["plan"]:
            if s["id"] == "glass-line":
                s["points"] = [[gx, -k.Z_IN], [gx, k.Z_IN]]
            if s["id"] == "frame-interior":
                s["points"] = [[k.SASH_X0, -k.Z_IN], [k.SASH_X0, k.Z_IN]]
            if s["id"] == "frame-exterior":
                s["points"] = [[k.SASH_X1, -k.Z_IN], [k.SASH_X1, k.Z_IN]]
    out[0]["description"] = tx("The conventional plan sign of a window in a wall opening, with its side and front view at real size: in plan the reveal linings cut and three lines across the opening for the sash faces and the glass (DIN 1356-1), the room side to the rear (-x); in the side view the section across the lining with the glass line; in the front view the outline of the lining frame and the sash.", "Das Grundrisszeichen eines Fensters in einer Wandöffnung mit Seiten- und Vorderansicht in wahrer Größe: im Grundriss die Laibungsfutter geschnitten und drei Linien quer über die Öffnung für die Flügelflächen und das Glas (DIN 1356-1), die Raumseite hinten (-x); in der Seitenansicht der Schnitt quer zum Futter mit der Glaslinie; in der Vorderansicht der Umriss von Futterrahmen und Flügel.")
    out[0]["dimensionalNote"] = tx("Real size: lining frame 0.90 m wide and 1.324 m high, 0.30 m deep; the three lines are 45 mm apart, which is 0.09 mm at 1:500 and merges into one line.", "Echte Größe: Futterrahmen 0,90 m breit und 1,324 m hoch, 0,30 m tief; die drei Linien liegen 45 mm auseinander, bei 1:500 sind das 0,09 mm und sie verschmelzen zu einer Linie.")
    out[1]["dimensionalNote"] = tx("Real size: lining frame 0.90 x 1.324 m in the facade plane, sash 0.776 x 1.20 m, reveal 0.30 m deep; the lines and the arrow carry only the opening direction and have no dimension.", "Echte Größe: Futterrahmen 0,90 x 1,324 m in der Fassadenebene, Flügel 0,776 x 1,20 m, Laibung 0,30 m tief; die Linien und der Pfeil tragen nur die Öffnungsrichtung und haben keine Abmessung.")
    return out


def thermal_binding():
    regions = {"pane-glass": {"exposedFraction": 1, "sunlitFraction": 1}, "pane-edge": {"exposedFraction": 1, "sunlitFraction": 1}, "frame-timber": {"exposedFraction": 1, "sunlitFraction": 1}, "frame-sill": {"exposedFraction": 1, "sunlitFraction": 0.5}}
    return {"regions": regions, "emitters": {}}


def apply(asset):
    kg, cu, cv, _ = k.sash_mass()
    kg, cu, cv = round(kg, 2), round(cu, 3), round(cv, 4)
    total = round(kg + k.lining_mass(), 1)
    area, cx, second = k.second_moment_x(k.stile_section())
    asset["summary"] = tx(
        "A 0.776 x 1.2 m timber sash with 4-16-4 low-e insulating glass in a 0.9 x 1.324 m timber lining frame 300 mm deep, hinged at its bottom on two tilt hinges and tilting inward up to 6 degrees (126 mm at the head) against two quadrant stays, drawn in vertical section; conditioned element with interior on the left at 20 degC and a winter reference case.",
        "Ein 0,776 x 1,2 m großer Holzflügel mit Wärmeschutz-Isolierglas 4-16-4 in einem 0,9 x 1,324 m großen, 300 mm tiefen Holzfutterrahmen, unten an zwei Kippbändern angeschlagen und bis 6 Grad (126 mm am Kopf) gegen zwei Kippbögen nach innen kippend, im Vertikalschnitt gezeichnet; konditioniertes Bauteil mit Innenraum links bei 20 °C und Winter-Referenzfall.")
    prov = asset["provenance"]
    refs = [x for x in prov["references"] if x["id"] not in ("din-68121-1", "tilt-hardware")]
    refs.append({"id": "din-68121-1", "citation": "DIN 68121-1:1993 Timber profiles for windows and window doors: sash and frame sections, 12 mm rebate clearance and the 16 x 12 mm Euro hardware groove; sash 90 mm deep, glass let 14 mm into an 18 mm rebate behind a room-side glazing bead."})
    refs.append({"id": "tilt-hardware", "citation": "Tilt hardware data: tilt opening 130 mm at the head for a tilt-and-turn stay (Winkhaus stay SH.SE data sheet) and 70 to 80 mm with a tilt limiter (Baunetz Wissen, Fensterbeschläge: Öffnungsbegrenzer); bottom-hung sashes run on two tilt hinges at the room-side bottom corner and are held by a stay at each side."})
    prov["references"] = refs
    prov["notes"] = tx(
        "The sash hinges at its room-side bottom corner on two tilt hinges and tips inward; two quadrant stays on the lining, concentric with the hinge axis, guide a pin on each stile and stop the sash at 6 degrees (126 mm at the head, the tilt opening of real hardware); a spring catch in the top rail holds it closed, opened with the pull grip. The room is on the left, the exterior on the right. The lining frame (sill, jambs, head, 50 mm, 300 mm deep) carries exterior stop strips; the sash keeps 12 mm rebate clearance all round. The glass is an insulating unit of two 4 mm panes with a 16 mm gas gap (U_g 1.1 W/(m2 K)); the sash mass counts the two panes, the edge seal, timber at 420 kg/m3 and the steel hardware (%.2f kg)." % kg,
        "Der Flügel ist an seiner raumseitigen unteren Ecke an zwei Kippbändern angeschlagen und kippt nach innen; zwei Kippbögen am Futter, konzentrisch zur Bandachse, führen je einen Zapfen am Flügelholz und halten den Flügel bei 6 Grad (126 mm am Kopf, die Kippweite echter Beschläge); ein Schnäpper im oberen Riegel hält ihn geschlossen, geöffnet mit dem Ziehgriff. Der Raum liegt links, außen rechts. Der Futterrahmen (Sohl-, Seiten- und Kopfbrett, 50 mm, 300 mm tief) trägt äußere Anschlagleisten; der Flügel hat rundum 12 mm Falzluft. Das Glas ist eine Isolierverglasung aus zwei 4-mm-Scheiben mit 16 mm Gasfüllung (U_g 1,1 W/(m² K)); die Flügelmasse zählt die zwei Scheiben, den Randverbund, Holz mit 420 kg/m³ und die Stahlbeschläge (%s kg)." % ("%.2f" % kg).replace(".", ","))
    asset["dimensions"] = {"heightM": r(k.H), "widthM": 0.3, "depthM": 0.9, "massKg": total}
    bones = asset["rig"]["bones"]
    sash = next(b for b in bones if b["id"] == "sash")
    sash["x"], sash["y"] = k.lu(k.HX), k.lv(k.HY)
    sash["joint"] = {"type": "revolute", "minDeg": 0, "maxDeg": k.TILT_FULL}
    sash["title"] = tx("Sash, revolute at the tilt-hinge axis", "Flügel, Drehgelenk in der Kippbandachse")
    sash["mass"] = {"kg": kg, "comX": cu, "comY": cv, "distribution": "uniform", "source": "authored-estimate", "reference": "en-1991-1-1"}
    sash["section"] = {"areaM2": r(2 * area, 6), "secondMomentM4": float("%.4g" % (2 * second)), "extremeFibreM": r(max(k.SASH_X1 - cx, cx - k.SASH_X0), 4), "material": "timber", "title": tx("Two sash stiles 70 x 90 mm with rebate and hardware groove", "Zwei Flügelhölzer 70 x 90 mm mit Falz und Beschlagnut")}
    sill = next(b for b in bones if b["id"] == "sill")
    sill["title"] = tx("Lining frame (rigid root)", "Futterrahmen (starre Wurzel)")
    asset["anchors"] = [
        {"id": "hinge", "bone": "sash", "x": 0, "y": 0, "kind": "support", "title": tx("Tilt hinges at the room-side bottom corner, pin", "Kippbänder an der raumseitigen unteren Ecke, Festlager"), "support": "pin"},
        {"id": "stay", "bone": "sash", "x": k.KEEPER_U, "y": r(k.KEEPER_V), "kind": "contact", "title": tx("Quadrant-stay pins, horizontal roller", "Zapfen der Kippbögen, waagerechte Rolle"), "support": "roller", "normalDeg": 0},
    ]
    for pose in asset["poses"]:
        pose["thermal"] = thermal_binding()
        if pose["id"] == "tilted-open":
            pose["title"] = tx("Tilted open 3 deg", "Gekippt 3 Grad")
            pose["description"] = tx("Sash tilted 3 degrees inward about the tilt hinges (63 mm at the head); the self-weight moment is taken by the quadrant stays.", "Flügel 3 Grad nach innen um die Kippbänder gekippt (63 mm am Kopf); das Eigengewichtsmoment nehmen die Kippbögen auf.")
            pose["bones"] = {"sash": {"angleDeg": k.TILT_HALF}}
        if pose["id"] == "fully-tilted":
            pose["title"] = tx("Fully tilted with wind pressure", "Voll gekippt mit Winddruck")
            pose["description"] = tx("Sash at the stay stop, 6 degrees inward (126 mm at the head), under 0.8 kN/m2 wind pressure on the exterior face over 0.9 m width.", "Flügel am Anschlag der Kippbögen, 6 Grad nach innen (126 mm am Kopf), unter 0,8 kN/m² Winddruck auf der Außenseite über 0,9 m Breite.")
            pose["bones"] = {"sash": {"angleDeg": k.TILT_FULL}}
            t = math.radians(k.TILT_FULL)
            for load in pose["loads"]:
                load["distributed"]["wxNPerM"] = round(-720 * math.cos(t), 1)
                load["distributed"]["wyNPerM"] = round(-720 * math.sin(t), 1)
        if pose["id"] == "closed":
            pose["description"] = tx("Sash vertical against the stop strips, held by the spring catch and the stays.", "Flügel senkrecht an den Anschlagleisten, gehalten von Schnäpper und Kippbögen.")
    for clip in asset["clips"]:
        if clip["id"] == "tilt-open-close":
            clip["description"] = tx("The sash tilts inward to 3 degrees, on to the stay stop at 6 degrees and returns smoothly to closed.", "Der Flügel kippt nach innen auf 3 Grad, weiter bis zum Anschlag der Kippbögen bei 6 Grad und kehrt weich in die Schließlage zurück.")
            clip["events"][0]["title"] = tx("Stay stop reached", "Anschlag der Kippbögen erreicht")
    th = asset["thermal"]
    for region in th["regions"]:
        if AREAS.get(region["id"]) is not None:
            region["areaM2"] = AREAS[region["id"]]
        if region["id"] == "frame-timber":
            region["title"] = tx("Timber sash and lining frame", "Holzflügel und Futterrahmen")
            region["characteristicLengthM"] = 0.09
        if region["id"] == "frame-sill":
            region["title"] = tx("Lining sill board (outer face)", "Sohlbrett des Futters (Außenseite)")
    th["assumptions"][-1] = tx("The two 4 mm panes, the 16 mm cavity and the edge seal are modelled at their real thickness; the structural mass counts the glass, not the gas.", "Die zwei 4-mm-Scheiben, der 16-mm-Gasraum und der Randverbund sind in ihrer wahren Dicke modelliert; die Tragwerksmasse zählt das Glas, nicht das Gas.")
    th["assumptions"][3] = tx("Sun-exposed and shaded faces differ through the pose sunlit fractions: glass, sash and lining in the sun, the sill board half shaded by the lining and the weather bar.", "Besonnte und verschattete Flächen unterscheiden sich über die Besonnungsanteile der Posen: Glas, Flügel und Futter in der Sonne, das Sohlbrett halb durch Futter und Wetterschenkel verschattet.")
    st = asset["structure"]
    st["assumptions"] = [
        tx("The sash is one rigid body pinned on the tilt-hinge axis at its room-side bottom corner; the two quadrant stays are concentric with that axis, so their pins slide freely until they reach the end of the slot at 6 degrees; in every pose the stay pins are idealised as one horizontal roller at the pin height (1.15 m above the axis), so the system is statically determinate.", "Der Flügel ist ein starrer Körper mit Festlager in der Kippbandachse an seiner raumseitigen unteren Ecke; die zwei Kippbögen sind konzentrisch zu dieser Achse, ihre Zapfen gleiten frei bis zum Schlitzende bei 6 Grad; in jeder Pose sind die Zapfen als eine waagerechte Rolle in Zapfenhöhe (1,15 m über der Achse) idealisiert, das System ist statisch bestimmt."),
        tx("The wind is pressure on the exterior face, normal to the sash in the section plane, 0.8 kN/m2 over 0.9 m; the stay stiffness and the bending of the arcs are not modelled.", "Der Wind ist Druck auf die Außenseite, senkrecht zum Flügel in der Schnittebene, 0,8 kN/m² über 0,9 m; Steifigkeit der Kippbögen und ihre Biegung sind nicht modelliert."),
    ]
    drawings = {"50": drawing50(), "100": drawing100(), "200": drawing200()}
    for scale, geometry in drawings.items():
        rep = asset["representations"][scale]
        rep["geometry"] = geometry
        rep["decisions"].update(TEXTS[scale])
    rep500 = asset["representations"]["500"]
    rep500["reason"] = tx("A window section is 0.30 x 1.32 m, i.e. 0.6 x 2.6 mm at 1:500; the sash is 0.18 mm and the 24 mm glass 0.05 mm thick, below every printable stroke, and any thickening would multiply the real thickness.", "Ein Fensterschnitt ist 0,30 x 1,32 m groß, bei 1:500 also 0,6 x 2,6 mm; der Flügel ist 0,18 mm und das 24-mm-Glas 0,05 mm dick, unter jeder druckbaren Strichstärke, und jede Verdickung würde die echte Dicke vervielfachen.")
    asset["symbols"] = symbols(asset)
    return asset


def number(value):
    if isinstance(value, float) and value != 0 and abs(value) < 1e-4:
        return ("%.12f" % value).rstrip("0")
    return json.dumps(value)


def dumps(value, indent):
    pad = "  " * indent
    if isinstance(value, dict):
        if not value:
            return "{}"
        inner = ",\n".join("%s  %s: %s" % (pad, json.dumps(key, ensure_ascii=False), dumps(v, indent + 1)) for key, v in value.items())
        return "{\n%s\n%s}" % (inner, pad)
    if isinstance(value, list):
        if not value:
            return "[]"
        inner = ",\n".join("%s  %s" % (pad, dumps(v, indent + 1)) for v in value)
        return "[\n%s\n%s]" % (inner, pad)
    if isinstance(value, float):
        return number(value)
    return json.dumps(value, ensure_ascii=False)


def member(source, key):
    found = re.search(r'\n    "' + key + r'"\s*:\s*', source)
    depth, index, quoted = 0, found.end(), False
    while True:
        char = source[index]
        if quoted:
            if char == "\\":
                index += 1
            elif char == '"':
                quoted = False
        elif char == '"':
            quoted = True
        elif char in "{[":
            depth += 1
        elif char in "}]":
            depth -= 1
            if depth == 0:
                return found.end(), index + 1
        index += 1


def write_manifest():
    source = open(MANIFEST, encoding="utf-8").read()
    found = re.search(r'("id": "tilt-window",\n(?:.*\n){1,12}?\s*"heightM": )([0-9.]+)', source)
    if found is None:
        raise SystemExit("manifest entry not found")
    updated = source[:found.start(2)] + number(r(k.H)) + source[found.end(2):]
    if json.loads(updated) is None:
        raise SystemExit("manifest broken")
    with open(MANIFEST, "w", encoding="utf-8") as f:
        f.write(updated)
    print("manifest heightM", found.group(2), "->", r(k.H))


if __name__ == "__main__":
    source = open(CATALOG, encoding="utf-8").read()
    document = json.loads(source)
    before = json.loads(source)
    apply(document["asset"])
    if "--apply" in sys.argv:
        for key in MEMBERS:
            start, end = member(source, key)
            if json.loads(source[start:end]) != before["asset"][key]:
                raise SystemExit("member %s not located" % key)
            source = source[:start] + dumps(document["asset"][key], 2) + source[end:]
        check = json.loads(source)
        for key in check["asset"]:
            if key not in MEMBERS and check["asset"][key] != before["asset"][key]:
                raise SystemExit("member %s changed unexpectedly" % key)
            if key in MEMBERS and check["asset"][key] != document["asset"][key]:
                raise SystemExit("member %s not written as intended" % key)
        with open(CATALOG, "w", encoding="utf-8") as f:
            f.write(source)
        print("catalog members written:", ", ".join(MEMBERS))
        write_manifest()
    else:
        document["asset"]["volumes"] = json.load(open(os.path.join(T, "volumes", "components", "tilt-window.json"), encoding="utf-8"))["volumes"]
        folder = os.path.join(T, "🗑️generated", "r3-tilt-window")
        os.makedirs(folder, exist_ok=True)
        with open(os.path.join(folder, "🔣️.json"), "w", encoding="utf-8") as f:
            json.dump(document, f, ensure_ascii=False, indent=2)
        kg, cu, cv, _ = k.sash_mass()
        print("candidate written; sash %.2f kg com %.3f %.4f; total %.1f kg; H %.3f" % (kg, cu, cv, kg + k.lining_mass(), k.H))
