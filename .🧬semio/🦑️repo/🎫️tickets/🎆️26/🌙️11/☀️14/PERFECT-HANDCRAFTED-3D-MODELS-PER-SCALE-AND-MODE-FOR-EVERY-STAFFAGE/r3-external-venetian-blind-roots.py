"""🏗 Root fixes of components/external-venetian-blind: rig (18 lamellae, blind centred in the cover), poses, clips, anchors, materials, thermal regions, masses and the 2D drawings of all four scales.

Without flag: writes the candidate document to 🗑️generated/r3-external-venetian-blind/🔣️.json (with the volumes source) for
`r3-validate.ts --file` and `r3-2d.ts --file`. With --apply: re-reads the catalog file, replaces only the members
listed in MEMBERS (and the manifest entry's distinction), keeps the file layout and verifies nothing else changed.
"""

import importlib.util
import json
import os
import re
import sys

T = os.path.dirname(os.path.abspath(__file__))
R = os.path.abspath(os.path.join(T, *[".."] * 7))
S = os.path.join(R, "🎓️teaching", "🏛️architecture", "🧍️staffage")
CATALOG = os.path.join(S, "🗂️catalog", "🧱️components", "external-venetian-blind", "🔣️.json")
MANIFEST = os.path.join(S, "🗂️catalog", "🔣️.json")
SOURCE = os.path.join(T, "volumes", "components", "external-venetian-blind.json")
spec = importlib.util.spec_from_file_location("kit", os.path.join(T, "r3-external-venetian-blind-kit.py"))
k = importlib.util.module_from_spec(spec)
spec.loader.exec_module(k)

MEMBERS = ["summary", "provenance", "dimensions", "materials", "rig", "anchors", "representations", "poses", "clips", "thermal", "structure"]
AREAS = {"lamellae": 4.28, "box-and-rails": 2.05, "glazing-behind": 1.35, "facade-wall": 2.33, "window-frame": 0.28}
Z = k.Z
N = k.N
BOXV = k.BOX_Y0


def r(x):
    return round(x + 0.0, 5)


def tx(en, de):
    return {"en": en, "de": de}


def shape(id, bone, role, z, filled, tone, geometry, region=None, material=None, hatch=None, pen=None, solid=None):
    s = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    if region is not None:
        s["region"] = region
    if material is not None:
        s["material"] = material
    if hatch is not None:
        s["hatch"] = hatch
    if pen is not None:
        s["penMm"] = pen
    s.update(geometry)
    if solid is not None:
        s["solid"] = {"kind": "extruded", "z0": r(-solid), "z1": r(solid), "sweep": "w"}
    return s


def rect(x0, y0, x1, y1):
    return {"kind": "rect", "x": r(min(x0, x1)), "y": r(min(y0, y1)), "w": r(abs(x1 - x0)), "h": r(abs(y1 - y0))}


def poly(points, closed=True):
    return {"kind": "poly", "points": [[r(a), r(b)] for a, b in points], "closed": closed}


def path(*rings):
    return {"kind": "path", "d": " ".join("M " + " L ".join("%s %s" % (r(a), r(b)) for a, b in ring) + " Z" for ring in rings)}


def ellipse(cx, cy, radius):
    return {"kind": "ellipse", "cx": r(cx), "cy": r(cy), "rx": r(radius), "ry": r(radius)}


def line(points):
    return poly(points, False)


WALL, GLASS, FRAME, HARD, LAM = "facade-wall", "glazing-behind", "window-frame", "box-and-rails", "lamellae"
F, B, RL = "facade", "box", "bottom-rail"
XI, XB0, XB1, XE1 = k.X_IN, k.X_BLOCK0, k.X_BLOCK1, k.X_EPS1
SILL, HEAD, H = k.SILL_Y, k.HEAD_Y, k.H
GV0 = k.GUIDE_Y0 - BOXV


def ring_rects(y0, y1, z, face):
    outer = [(-z, y0), (z, y0), (z, y1), (-z, y1)]
    hole = [(-z + face, y0 + face), (-z + face, y1 - face), (z - face, y1 - face), (z - face, y0 + face)]
    return outer, hole


def slats(scale, view):
    out = []
    if scale == 50:
        pts, pen = k.SLAT_LINE, 0.13
    elif scale == 100:
        pts, pen = [(-0.045, 0.008), (-0.038, 0.0), (0.038, 0.0), (0.045, -0.008)], 0.13
    else:
        pts, pen = [(-0.045, 0.008), (-0.038, 0.0), (0.038, 0.0), (0.045, -0.008)], 0.1
    ids = [1, 5, 9, 13, 17] if scale == 500 else range(N)
    half = k.RAIL_GZ0 if scale == 500 else k.SLAT_Z
    for i in ids:
        out.append(shape("slat-%d" % i, "slat-%d" % i, "texture", 5.6 if view == "elevation" else 3, False, 0.4, line(pts), region=LAM, material="aluminium", pen=pen, solid=None if view == "elevation" else half))
    return out


def bottom_rail(scale, view):
    solid = None if view == "elevation" else (k.RAIL_Z + k.CAP if scale <= 100 else k.RAIL_GZ0)
    z = {"elevation": 5.7, "front": 4, "plan": 3.5}[view]
    if scale == 50:
        out = [shape("bottom-rail", RL, "body", z, True, 0.35, poly(k.bottom_rail()), region=HARD, material="aluminium", pen=0.18, solid=solid)]
        if view == "elevation":
            out.append(shape("bottom-rail-chamber", RL, "texture", z + 0.1, False, 0.3, poly(k.bottom_rail_hole()), region=HARD))
        return out
    return [shape("bottom-rail", RL, "body", z, True, 0.35, rect(-k.RAIL_W / 2, -k.RAIL_H / 2, k.RAIL_W / 2, k.RAIL_H / 2), region=HARD, material="aluminium", pen=0.18 if scale == 500 else 0.13, solid=solid)]


def elevation50():
    g = [shape("ground", F, "ground", 0, False, 0.5, line([(XI, 0), (k.BOX_D, 0)]))]
    g += [
        shape("frame-jamb", F, "detail", 0.5, True, 0.82, rect(k.FR_X0, SILL, k.FR_X1, HEAD), region=FRAME, material="aluminium", pen=0.13),
        shape("plaster-sill", F, "detail", 1, True, 0.85, rect(XI, 0, XB0, SILL), region=WALL, material="masonry", pen=0.13),
        shape("block-sill", F, "body", 1, True, 0.5, rect(XB0, 0, XB1, SILL), region=WALL, material="masonry", hatch="cut-masonry", pen=0.25),
        shape("insulation-sill", F, "detail", 1, True, 0.7, rect(XB1, 0, XE1, SILL), region=WALL, material="eps", hatch="insulation"),
        shape("render-sill", F, "detail", 1, False, 0.35, line([(XE1 / 2, 0), (XE1 / 2, SILL)]), region=WALL, material="masonry", pen=0.2),
        shape("plaster-reveal-sill", F, "detail", 1.2, True, 0.85, rect(XI, SILL, k.FR_X0, SILL + 0.015), region=WALL, material="masonry", pen=0.13),
        shape("sill-bed", F, "detail", 1.2, True, 0.7, poly(k.sill_bed()), region=WALL, material="eps", hatch="insulation", pen=0.1),
        shape("sill-board", F, "detail", 2.5, False, 0.2, line(k.sill_board_line()), region=WALL, material="aluminium", pen=0.13),
        shape("plaster-head", F, "detail", 1, True, 0.85, rect(XI, HEAD, XB0, H), region=WALL, material="masonry", pen=0.13),
        shape("lintel", F, "body", 1, True, 0.4, rect(XB0, HEAD, XB1, HEAD + k.LINTEL_H), region=WALL, material="masonry", hatch="cut-masonry", pen=0.25),
        shape("block-head", F, "body", 1, True, 0.5, rect(XB0, HEAD + k.LINTEL_H, XB1, H), region=WALL, material="masonry", hatch="cut-masonry", pen=0.25),
        shape("insulation-head", F, "detail", 1, True, 0.7, rect(XB1, HEAD, XE1, H), region=WALL, material="eps", hatch="insulation"),
        shape("render-head", F, "detail", 1, False, 0.35, line([(k.FR_X1, HEAD + XE1 / 2), (XE1 / 2, HEAD + XE1 / 2), (XE1 / 2, H)]), region=WALL, material="masonry", pen=0.2),
        shape("plaster-reveal-head", F, "detail", 1.2, True, 0.85, rect(XI, HEAD - 0.015, k.FR_X0, HEAD), region=WALL, material="masonry", pen=0.13),
    ]
    for n, y in enumerate([k.COURSE + k.BED / 2 + i * 0.25 for i in range(4)] + [HEAD + k.LINTEL_H + k.BED / 2, HEAD + 0.375 - k.BED / 2]):
        g.append(shape("bed-joint-%d" % n, F, "texture", 1.5, False, 0.3, line([(XB0, y), (XB1, y)]), region=WALL))
    g += [
        shape("frame-sill", F, "body", 2, True, 0.25, poly(k.frame_rail(True)), region=FRAME, material="aluminium", pen=0.18),
        shape("frame-head", F, "body", 2, True, 0.25, poly(k.frame_rail(False)), region=FRAME, material="aluminium", pen=0.18),
        shape("bead-sill", F, "detail", 2, True, 0.3, rect(k.BEAD_X0, k.REB_Y0, k.GL_X0, k.SIGHT_Y0), region=FRAME, material="aluminium", pen=0.13),
        shape("bead-head", F, "detail", 2, True, 0.3, rect(k.BEAD_X0, k.SIGHT_Y1, k.GL_X0, k.REB_Y1), region=FRAME, material="aluminium", pen=0.13),
        shape("frame-sill-edge", F, "detail", 2.1, False, 0.1, line([(k.FR_X0, k.REB_Y0), (k.BEAD_X0, k.REB_Y0), (k.BEAD_X0, k.SIGHT_Y0), (k.GL_X0, k.SIGHT_Y0), (k.GL_X0, k.REB_Y0), (k.FR_STOP_X, k.REB_Y0), (k.FR_STOP_X, k.SIGHT_Y0), (k.FR_X1, k.SIGHT_Y0)]), region=FRAME, pen=0.18),
        shape("frame-head-edge", F, "detail", 2.1, False, 0.1, line([(k.FR_X0, k.REB_Y1), (k.BEAD_X0, k.REB_Y1), (k.BEAD_X0, k.SIGHT_Y1), (k.GL_X0, k.SIGHT_Y1), (k.GL_X0, k.REB_Y1), (k.FR_STOP_X, k.REB_Y1), (k.FR_STOP_X, k.SIGHT_Y1), (k.FR_X1, k.SIGHT_Y1)]), region=FRAME, pen=0.18),
        shape("spacer-sill", F, "detail", 2.2, True, 0.2, rect(k.GL_X0 + k.PANE, k.GL_Y0, k.GL_X1 - k.PANE, k.GL_Y0 + k.SPACER), region=GLASS, material="aluminium", pen=0.1),
        shape("spacer-head", F, "detail", 2.2, True, 0.2, rect(k.GL_X0 + k.PANE, k.GL_Y1 - k.SPACER, k.GL_X1 - k.PANE, k.GL_Y1), region=GLASS, material="aluminium", pen=0.1),
        shape("glass-a", F, "glazing", 2.3, False, 0.2, line([(k.GL_X0 + k.PANE / 2, k.GL_Y0), (k.GL_X0 + k.PANE / 2, k.GL_Y1)]), region=GLASS, material="glass"),
        shape("glass-b", F, "glazing", 2.3, False, 0.2, line([(k.GL_X1 - k.PANE / 2, k.GL_Y0), (k.GL_X1 - k.PANE / 2, k.GL_Y1)]), region=GLASS, material="glass"),
    ]
    t = k.SHEET
    g += [
        shape("holder-top", B, "detail", 1.8, True, 0.4, rect(0, k.HOLDER_TOP_Y - BOXV - 0.007, k.RAIL_GX0, k.HOLDER_TOP_Y - BOXV + 0.007), region=HARD, material="steel", pen=0.1),
        shape("holder-bottom", B, "detail", 1.8, True, 0.4, rect(0, k.HOLDER_BOTTOM_Y - BOXV - 0.007, k.RAIL_GX0, k.HOLDER_BOTTOM_Y - BOXV + 0.007), region=HARD, material="steel", pen=0.1),
        shape("guide", B, "detail", 2, True, 0.8, rect(k.RAIL_GX0, GV0, k.RAIL_GX1, t), region=HARD, material="aluminium", pen=0.13),
        shape("box", B, "body", 4.5, True, 0.8, rect(t, t, k.BOX_D - t, k.BOX_H - t), region=HARD, material="aluminium", pen=0.1),
        shape("bracket", B, "detail", 4.8, False, 0.3, line(k.bracket_line()), region=HARD, material="steel", pen=0.13),
        shape("head-rail", B, "detail", 5.2, False, 0.15, line(k.head_rail_line()), region=HARD, material="steel", pen=0.18),
        shape("motor", B, "detail", 5.3, True, 0.3, ellipse(k.BX, k.SHAFT_V, k.MOTOR_R), region=HARD, material="steel", pen=0.13),
        shape("motor-case", B, "detail", 5.4, False, 0.1, ellipse(k.BX, k.SHAFT_V, k.MOTOR_R), region=HARD, pen=0.13),
        shape("tilt-shaft", B, "detail", 5.5, False, 0.1, ellipse(k.BX, k.SHAFT_V, k.SHAFT_R), region=HARD, pen=0.1),
        shape("cover", B, "body", 6, False, 0.1, line(k.cover_line()), region=HARD, material="aluminium", pen=0.25),
    ]
    return g + slats(50, "elevation") + bottom_rail(50, "elevation")


def front_common(scale, face, wall_pen, reflexes):
    outer, hole = ring_rects(SILL, HEAD, Z, face)
    g = [
        shape("wall-sill", F, "body", 1, True, 0.6, rect(-Z, 0, Z, SILL), region=WALL, material="masonry", pen=wall_pen),
        shape("wall-head", F, "body", 1, True, 0.6, rect(-Z, HEAD, Z, H), region=WALL, material="masonry", pen=wall_pen),
        shape("window-frame", F, "body", 1.5, True, 0.35, path(outer, hole), region=FRAME, material="aluminium", pen=0.13),
        shape("glass", F, "glazing", 2, True, 0.9, rect(-Z + face, SILL + face, Z - face, HEAD - face), region=GLASS, material="glass", pen=0.1),
    ]
    for n, x in enumerate(reflexes):
        g.append(shape("glass-reflex-%s" % "ab"[n], F, "glazing", 2.5, False, 0.2, line([(x, 1.3), (x + 0.2, 1.8)]), region=GLASS))
    return g


def front50():
    g = front_common(50, k.FR_FACE, 0.25, [-0.4, -0.3])
    g += [
        shape("render-soffit", F, "detail", 1.2, True, 0.5, rect(-Z, HEAD - 0.01, Z, HEAD), region=WALL, material="masonry", pen=0.1),
        shape("sill-board", F, "detail", 2.6, True, 0.75, rect(-Z, k.SB_DRIP_Y, Z, k.SB_BACK_Y + 0.015), region=WALL, material="aluminium", pen=0.13),
        shape("holder-top-left", B, "detail", 1.8, True, 0.4, ellipse(k.HOLDER_Z, k.HOLDER_TOP_Y, 0.018), region=HARD, material="steel", pen=0.1),
        shape("holder-top-right", B, "detail", 1.8, True, 0.4, ellipse(-k.HOLDER_Z, k.HOLDER_TOP_Y, 0.018), region=HARD, material="steel", pen=0.1),
        shape("holder-bottom-left", B, "detail", 1.8, True, 0.4, ellipse(k.HOLDER_Z, k.HOLDER_BOTTOM_Y, 0.018), region=HARD, material="steel", pen=0.1),
        shape("holder-bottom-right", B, "detail", 1.8, True, 0.4, ellipse(-k.HOLDER_Z, k.HOLDER_BOTTOM_Y, 0.018), region=HARD, material="steel", pen=0.1),
        shape("box", B, "body", 5, True, 0.5, rect(-Z, BOXV, Z, BOXV + k.BOX_H), region=HARD, material="aluminium"),
        shape("end-plate-left", B, "texture", 6, False, 0.3, line([(Z - k.SHEET, BOXV), (Z - k.SHEET, BOXV + k.BOX_H)]), region=HARD),
        shape("end-plate-right", B, "texture", 6, False, 0.3, line([(-Z + k.SHEET, BOXV), (-Z + k.SHEET, BOXV + k.BOX_H)]), region=HARD),
        shape("guide-left", B, "detail", 4.5, True, 0.45, rect(k.RAIL_GZ0, k.GUIDE_Y0, Z, BOXV + k.SHEET), region=HARD, material="aluminium", pen=0.13),
        shape("guide-right", B, "detail", 4.5, True, 0.45, rect(-Z, k.GUIDE_Y0, -k.RAIL_GZ0, BOXV + k.SHEET), region=HARD, material="aluminium", pen=0.13),
    ]
    return g + slats(50, "front") + bottom_rail(50, "front")


def plan50():
    t = k.SHEET
    g = slats(50, "plan") + bottom_rail(50, "plan")
    g += [
        shape("plaster-head", F, "detail", 1, True, 0.85, rect(XI, -Z, XB0, Z), region=WALL, material="masonry", pen=0.13),
        shape("block-head", F, "body", 1, True, 0.5, rect(XB0, -Z, XB1, Z), region=WALL, material="masonry", hatch="cut-masonry", pen=0.25),
        shape("insulation-head", F, "detail", 1, True, 0.7, rect(XB1, -Z, XE1, Z), region=WALL, material="eps", hatch="insulation"),
        shape("render-head", F, "detail", 1, False, 0.35, line([(XE1 / 2, -Z), (XE1 / 2, Z)]), region=WALL, material="masonry", pen=0.2),
        shape("box", B, "body", 5, True, 0.5, rect(0, -Z, k.BOX_D, Z), region=HARD, material="aluminium"),
        shape("flange-fold", B, "texture", 5.5, False, 0.3, line([(t, -Z), (t, Z)]), region=HARD),
        shape("end-plate-left", B, "texture", 5.5, False, 0.3, line([(0, Z - t), (k.BOX_D, Z - t)]), region=HARD),
        shape("end-plate-right", B, "texture", 5.5, False, 0.3, line([(0, -Z + t), (k.BOX_D, -Z + t)]), region=HARD),
    ]
    return g


def elevation100():
    t = k.SHEET
    board = [(k.FR_X1, SILL), (k.SB_X1, SILL), (k.SB_X1, k.sb_top(k.SB_X1)), (k.FR_X1, k.SB_BACK_Y)]
    g = [
        shape("ground", F, "ground", 0, False, 0.5, line([(XI, 0), (k.BOX_D, 0)])),
        shape("frame-jamb", F, "detail", 0.5, True, 0.82, rect(k.FR_X0, SILL, k.FR_X1, HEAD), region=FRAME, material="aluminium", pen=0.1),
        shape("wall-sill", F, "body", 1, True, 0.5, rect(XI, 0, XB1, SILL), region=WALL, material="masonry", hatch="cut-masonry", pen=0.18),
        shape("insulation-sill", F, "detail", 1, True, 0.8, rect(XB1, 0, 0, SILL), region=WALL, material="eps", pen=0.13),
        shape("sill-board", F, "detail", 2.5, True, 0.3, poly(board), region=WALL, material="aluminium", pen=0.1),
        shape("lintel", F, "body", 1, True, 0.4, rect(XI, HEAD, XB1, HEAD + 0.125), region=WALL, material="masonry", hatch="cut-masonry", pen=0.18),
        shape("wall-head", F, "body", 1, True, 0.5, rect(XI, HEAD + 0.125, XB1, H), region=WALL, material="masonry", hatch="cut-masonry", pen=0.18),
        shape("insulation-head", F, "detail", 1, True, 0.8, rect(XB1, HEAD, 0, H), region=WALL, material="eps", pen=0.13),
        shape("frame-sill", F, "body", 2, True, 0.25, rect(k.FR_X0, SILL, k.FR_X1, k.SIGHT_Y0), region=FRAME, material="aluminium", pen=0.13),
        shape("frame-head", F, "body", 2, True, 0.25, rect(k.FR_X0, k.SIGHT_Y1, k.FR_X1, HEAD), region=FRAME, material="aluminium", pen=0.13),
        shape("frame-sill-edge", F, "detail", 2.1, False, 0.1, line([(k.FR_X0, k.SIGHT_Y0), (k.FR_X1, k.SIGHT_Y0)]), region=FRAME, pen=0.13),
        shape("frame-head-edge", F, "detail", 2.1, False, 0.1, line([(k.FR_X0, k.SIGHT_Y1), (k.FR_X1, k.SIGHT_Y1)]), region=FRAME, pen=0.13),
        shape("glass", F, "glazing", 2.3, False, 0.2, line([(-0.188, k.SIGHT_Y0), (-0.188, k.SIGHT_Y1)]), region=GLASS, material="glass", pen=0.13),
        shape("holder-top", B, "detail", 1.8, True, 0.4, rect(0, k.HOLDER_TOP_Y - BOXV - 0.0075, k.RAIL_GX0, k.HOLDER_TOP_Y - BOXV + 0.0075), region=HARD, material="steel", pen=0.1),
        shape("holder-bottom", B, "detail", 1.8, True, 0.4, rect(0, k.HOLDER_BOTTOM_Y - BOXV - 0.0075, k.RAIL_GX0, k.HOLDER_BOTTOM_Y - BOXV + 0.0075), region=HARD, material="steel", pen=0.1),
        shape("guide", B, "detail", 2, True, 0.8, rect(k.RAIL_GX0, GV0, k.RAIL_GX1, 0), region=HARD, material="aluminium", pen=0.13),
        shape("box", B, "body", 4.5, True, 0.8, rect(0, 0, k.BOX_D, k.BOX_H), region=HARD, material="aluminium", pen=0.1),
        shape("bracket", B, "detail", 4.8, True, 0.4, rect(t, 0.16, k.HR_X0, 0.175), region=HARD, material="steel", pen=0.1),
        shape("head-rail", B, "detail", 5.2, True, 0.3, rect(k.HR_X0, k.HR_V0, k.HR_X1, k.HR_V1), region=HARD, material="steel", pen=0.13),
        shape("head-rail-edge", B, "detail", 5.3, False, 0.1, rect(k.HR_X0, k.HR_V0, k.HR_X1, k.HR_V1), region=HARD, pen=0.13),
        shape("cover", B, "body", 6, False, 0.1, line([(t / 2, k.BOX_H - k.FLANGE), (t / 2, k.BOX_H - t / 2), (k.BOX_D - t / 2, k.BOX_H - t / 2), (k.BOX_D - t / 2, 0)]), region=HARD, material="aluminium", pen=0.25),
    ]
    return g + slats(100, "elevation") + bottom_rail(100, "elevation")


def front100():
    g = front_common(100, k.FR_FACE, 0.18, [-0.4])
    g += [
        shape("sill-board", F, "detail", 2.6, True, 0.75, rect(-Z, SILL, Z, k.SB_BACK_Y), region=WALL, material="aluminium", pen=0.1),
        shape("box", B, "body", 5, True, 0.5, rect(-Z, BOXV, Z, BOXV + k.BOX_H), region=HARD, material="aluminium", pen=0.25),
        shape("guide-left", B, "detail", 4.5, True, 0.45, rect(k.RAIL_GZ0, k.GUIDE_Y0, Z, BOXV), region=HARD, material="aluminium", pen=0.13),
        shape("guide-right", B, "detail", 4.5, True, 0.45, rect(-Z, k.GUIDE_Y0, -k.RAIL_GZ0, BOXV), region=HARD, material="aluminium", pen=0.13),
    ]
    return g + slats(100, "front") + bottom_rail(100, "front")


def plan100():
    g = slats(100, "plan") + bottom_rail(100, "plan")
    g += [
        shape("wall-head", F, "body", 1, True, 0.5, rect(XI, -Z, XB1, Z), region=WALL, material="masonry", hatch="cut-masonry", pen=0.18),
        shape("insulation-head", F, "detail", 1, True, 0.8, rect(XB1, -Z, 0, Z), region=WALL, material="eps", pen=0.13),
        shape("box", B, "body", 5, True, 0.5, rect(0, -Z, k.BOX_D, Z), region=HARD, material="aluminium", pen=0.25),
    ]
    return g


def coarse(scale, face, pen):
    outer, hole = ring_rects(SILL, HEAD, Z, face)
    guide_w = k.RAIL_GX1 - k.RAIL_GX0
    elevation = [
        shape("ground", F, "ground", 0, False, 0.5, line([(XI, 0), (k.BOX_D, 0)])),
        shape("window", F, "detail", 0.5, True, 0.8, rect(k.FR_X0, SILL, k.FR_X1, HEAD), region=FRAME, material="aluminium", pen=pen),
        shape("wall-sill", F, "body", 1, True, 0.5, rect(XI, 0, XB1, SILL), region=WALL, material="masonry", pen=pen),
        shape("insulation-sill", F, "detail", 1, True, 0.8, rect(XB1, 0, 0, SILL), region=WALL, material="eps", pen=pen),
        shape("wall-head", F, "body", 1, True, 0.5, rect(XI, HEAD, XB1, H), region=WALL, material="masonry", pen=pen),
        shape("insulation-head", F, "detail", 1, True, 0.8, rect(XB1, HEAD, 0, H), region=WALL, material="eps", pen=pen),
        shape("glass", F, "glazing", 2.3, False, 0.2, line([(-0.188, SILL + face), (-0.188, HEAD - face)]), region=GLASS, material="glass"),
        shape("guide", B, "detail" if scale == 200 else "body", 2, True, 0.6, rect(k.RAIL_GX0, GV0, k.RAIL_GX1, 0), region=HARD, material="aluminium", pen=pen),
        shape("box", B, "body", 5, True, 0.45, rect(0, 0, k.BOX_D, k.BOX_H), region=HARD, material="aluminium", pen=pen),
    ] + slats(scale, "elevation") + bottom_rail(scale, "elevation")
    role = "detail" if scale == 200 else "body"
    front = [
        shape("wall-sill", F, "body", 1, True, 0.6, rect(-Z, 0, Z, SILL), region=WALL, material="masonry", pen=pen),
        shape("wall-head", F, "body", 1, True, 0.6, rect(-Z, HEAD, Z, H), region=WALL, material="masonry", pen=pen),
        shape("window-frame", F, "body", 1.5, True, 0.35, path(outer, hole), region=FRAME, material="aluminium", pen=0.1),
        shape("glass", F, "glazing", 2, True, 0.9, rect(-Z + face, SILL + face, Z - face, HEAD - face), region=GLASS, material="glass"),
        shape("box", B, "body", 5, True, 0.45, rect(-Z, BOXV, Z, BOXV + k.BOX_H), region=HARD, material="aluminium", pen=pen),
        shape("guide-left", B, role, 4.5, True, 0.45, rect(k.RAIL_GZ0, k.GUIDE_Y0, Z, BOXV), region=HARD, material="aluminium", pen=pen),
        shape("guide-right", B, role, 4.5, True, 0.45, rect(-Z, k.GUIDE_Y0, -k.RAIL_GZ0, BOXV), region=HARD, material="aluminium", pen=pen),
    ] + slats(scale, "front") + bottom_rail(scale, "front")
    plan = slats(scale, "plan") + bottom_rail(scale, "plan") + [
        shape("wall-head", F, "body", 1, True, 0.5, rect(XI, -Z, XB1, Z), region=WALL, material="masonry", pen=pen),
        shape("insulation-head", F, "detail", 1, True, 0.8, rect(XB1, -Z, 0, Z), region=WALL, material="eps", pen=pen),
        shape("box", B, "body", 5, True, 0.45, rect(0, -Z, k.BOX_D, Z), region=HARD, material="aluminium", pen=pen),
    ]
    return {"elevation": elevation, "front": front, "plan": plan}


CONVENTION_EN = " Convention: the side elevation is the vertical section A-A on the mid plane, looking along the facade, with everything beyond the cut (far jamb, far guide rail with its holders, far end plate of the cover) in light tone; the front view is the blind seen from outside (lateral x, left of the blind is +x, 1.2 m wide); the plan is the view from above, where the lamellae and the bottom rail stay hidden under the cover in every pose."
CONVENTION_DE = " Konvention: die Seitenansicht ist der Vertikalschnitt A-A in der Mittelebene mit Blick längs der Fassade, alles hinter dem Schnitt (hinterer Rahmenpfosten, hintere Führungsschiene mit Haltern, hinteres Seitenteil der Blende) in hellem Ton; die Vorderansicht ist die Jalousie von außen (seitliches x, links von der Jalousie ist +x, 1,2 m breit); die Draufsicht ist der Blick von oben, in dem Lamellen und Unterschiene in jeder Pose unter der Blende verdeckt bleiben."

TEXTS = {
    "50": {
        "silhouette": tx("Vertical section of a facade-mounted external venetian blind in front of a fixed aluminium window in an ETICS wall: the layered wall stubs with lintel and bed joints, the window frame with glazing bead and the insulating glass unit, the sloped sill board, the bent cover panel with the head rail, motor and bracket inside, the guide rail on its holders, 18 Z lamellae as section lines tilting about their centres and the hollow bottom rail." + CONVENTION_EN,
                         "Vertikalschnitt einer Vorbau-Außenjalousie vor einem festverglasten Aluminiumfenster in einer WDVS-Wand: die geschichteten Wandstücke mit Sturz und Lagerfugen, der Fensterrahmen mit Glasleiste und Isolierglas, die geneigte Fensterbank, die gekantete Blende mit Oberschiene, Motor und Träger darin, die Führungsschiene auf ihren Haltern, 18 Z-Lamellen als Schnittlinien, die um ihre Mitte kippen, und die hohle Unterschiene." + CONVENTION_DE),
        "retained": tx("Plaster 15 mm, block 175 mm with 12 mm bed joints, lintel 113 mm, EPS 70 mm, render 10 mm with the soffit return; frame 70 x 50 mm with stop and bead, both panes and the edge spacer; sill board with upstand, fall and drip; cover sheet, head rail 58 x 56 mm, motor, bracket, guide rail 30 x 20 mm, two holders, the Z section of every lamella, bottom rail 90 x 20 mm with its chamber.",
                       "Putz 15 mm, Ziegel 175 mm mit 12 mm Lagerfugen, Sturz 113 mm, EPS 70 mm, Außenputz 10 mm mit Untersicht; Rahmen 70 x 50 mm mit Anschlag und Glasleiste, beide Scheiben und der Randverbund; Fensterbank mit Aufkantung, Gefälle und Tropfkante; Blendenblech, Oberschiene 58 x 56 mm, Motor, Träger, Führungsschiene 30 x 20 mm, zwei Halter, das Z-Profil jeder Lamelle, Unterschiene 90 x 20 mm mit Kammer."),
        "merged": tx("The far jamb, the stops and the pane edges beyond the cut are one light band; the tape bearings and the tilt shaft lie inside the head rail outline.",
                     "Der hintere Pfosten, die Anschläge und die Scheibenkanten hinter dem Schnitt sind ein helles Band; Bandlager und Wendewelle liegen im Umriss der Oberschiene."),
        "omitted": tx("Ladder cords and lift tapes (a shape bound to one bone cannot follow the lift), gaskets, the thermal break of the frame, the lamella beads (0.08 mm on paper), screws, anchors inside the wall; plan: everything below the cover and the wall head.",
                      "Leiterkordeln und Aufzugsbänder (eine an einen Knochen gebundene Form kann dem Hub nicht folgen), Dichtungen, die thermische Trennung des Rahmens, die Lamellenbördel (0,08 mm auf dem Papier), Schrauben, Anker in der Wand; Draufsicht: alles unter Blende und Wandkopf."),
        "thinElements": tx("Sheet metal is drawn as a stroke on its centre line: lamellae (0.45 mm) and sill board (2 mm) with 0.13 mm, head rail (1.5 mm) with 0.18 mm, cover (2 mm) with 0.25 mm; the panes are 0.13 mm lines; plaster and render are filled bands of 0.3 and 0.2 mm.",
                           "Blech ist als Strich auf seiner Mittellinie gezeichnet: Lamellen (0,45 mm) und Fensterbank (2 mm) mit 0,13 mm, Oberschiene (1,5 mm) mit 0,18 mm, Blende (2 mm) mit 0,25 mm; die Scheiben sind 0,13-mm-Linien; Innen- und Außenputz sind gefüllte Bänder von 0,3 und 0,2 mm."),
        "hatching": tx("Cut masonry and lintel as lines at 0.5 mm, EPS and the sill wedge as stipple, cut aluminium filled dark.", "Geschnittenes Mauerwerk und Sturz als Linien mit 0,5 mm, EPS und Fensterbankkeil punktiert, geschnittenes Aluminium dunkel gefüllt."),
        "styles": tx("Cut outlines 0.25 mm, cover 0.25 mm, frame 0.18 mm, parts beyond the cut 0.10 to 0.13 mm, lamellae 0.13 mm.", "Schnittkonturen 0,25 mm, Blende 0,25 mm, Rahmen 0,18 mm, Teile hinter dem Schnitt 0,10 bis 0,13 mm, Lamellen 0,13 mm."),
        "overlays": tx("One cell per region: wall with sill board, glazing, window frame, blind hardware; each lamella is its own stroke so tilt and shade stay readable.", "Eine Zelle je Region: Wand mit Fensterbank, Verglasung, Fensterrahmen, Behangteile; jede Lamelle ist ein eigener Strich, damit Kippung und Schatten lesbar bleiben."),
    },
    "100": {
        "silhouette": tx("Hatched cut masonry with the lintel, the insulation band, the cut frame rails with the far jamb and the glass line, the sill wedge, the cover with head rail and bracket arm, the guide rail on two holders, 18 lamellae as Z strokes and the bottom rail." + CONVENTION_EN,
                         "Schraffiertes geschnittenes Mauerwerk mit Sturz, das Dämmband, die geschnittenen Rahmenriegel mit hinterem Pfosten und Glaslinie, der Fensterbankkeil, die Blende mit Oberschiene und Trägerarm, die Führungsschiene auf zwei Haltern, 18 Lamellen als Z-Striche und die Unterschiene." + CONVENTION_DE),
        "retained": tx("Masonry 190 mm, insulation 80 mm, lintel, frame rails 70 x 50 mm, glass line, cover sheet, head rail, guide rail, holders, bottom rail 90 x 20 mm.", "Mauerwerk 190 mm, Dämmung 80 mm, Sturz, Rahmenriegel 70 x 50 mm, Glaslinie, Blendenblech, Oberschiene, Führungsschiene, Halter, Unterschiene 90 x 20 mm."),
        "merged": tx("Plaster with the block, render with the EPS, stop and bead with the frame, both panes in one line, the sill board with its wedge.", "Innenputz mit dem Ziegel, Außenputz mit dem EPS, Anschlag und Glasleiste mit dem Rahmen, beide Scheiben in einer Linie, die Fensterbank mit ihrem Keil."),
        "omitted": tx("Bed joints, motor, tape bearings, cords and tapes, lamella camber and beads, the chamber of the bottom rail, the hem and end-plate seams of the cover.", "Lagerfugen, Motor, Bandlager, Kordeln und Bänder, Wölbung und Bördel der Lamellen, die Kammer der Unterschiene, Umbug und Seitenteilfugen der Blende."),
        "thinElements": tx("Lamellae are 0.13 mm strokes, the cover a 0.25 mm stroke, the glass a 0.13 mm line.", "Lamellen sind 0,13-mm-Striche, die Blende ein 0,25-mm-Strich, das Glas eine 0,13-mm-Linie."),
        "hatching": tx("Cut masonry as lines at 0.5 mm; insulation as a light tone.", "Geschnittenes Mauerwerk als Linien mit 0,5 mm; Dämmung als heller Ton."),
        "styles": tx("Cut outlines 0.18 mm, cover 0.25 mm, details 0.10 to 0.13 mm.", "Schnittkonturen 0,18 mm, Blende 0,25 mm, Details 0,10 bis 0,13 mm."),
        "overlays": tx("One cell per region; each lamella is its own stroke.", "Eine Zelle je Region; jede Lamelle ist ein eigener Strich."),
    },
    "200": {
        "silhouette": tx("Masonry and insulation of the cut wall stubs, the window as a light band with the glass line, the box as a filled block, the guide rail and 18 lamella strokes with the bottom rail." + CONVENTION_EN,
                         "Mauerwerk und Dämmung der geschnittenen Wandstücke, das Fenster als helles Band mit Glaslinie, der Kasten als gefüllter Block, die Führungsschiene und 18 Lamellenstriche mit Unterschiene." + CONVENTION_DE),
        "retained": tx("Masonry 190 mm, insulation 80 mm, window depth 70 mm, box 150 x 210 mm, guide rail, every lamella, bottom rail.", "Mauerwerk 190 mm, Dämmung 80 mm, Fenstertiefe 70 mm, Kasten 150 x 210 mm, Führungsschiene, jede Lamelle, Unterschiene."),
        "merged": tx("Lintel and plaster with the masonry, frame and jamb in one band, cover, end plates and head rail in the box block.", "Sturz und Putz mit dem Mauerwerk, Rahmen und Pfosten in einem Band, Blende, Seitenteile und Oberschiene im Kastenblock."),
        "omitted": tx("Sill board, holders, brackets, camber and beads of the lamellae.", "Fensterbank, Halter, Träger, Wölbung und Bördel der Lamellen."),
        "thinElements": tx("Lamellae are 0.10 mm strokes of their Z section (0.45 mm long, 0.4 mm pitch on paper); the bottom rail is a 0.45 x 0.10 mm bar.", "Lamellen sind 0,10-mm-Striche ihres Z-Profils (0,45 mm lang, 0,4 mm Teilung auf dem Papier); die Unterschiene ist ein Balken von 0,45 x 0,10 mm."),
        "hatching": tx("No hatching; masonry mid grey, insulation light.", "Keine Schraffur; Mauerwerk mittelgrau, Dämmung hell."),
        "styles": tx("Outlines 0.13 mm, details 0.10 mm.", "Konturen 0,13 mm, Details 0,10 mm."),
        "overlays": tx("One cell per region.", "Eine Zelle je Region."),
    },
    "500": {
        "silhouette": tx("A 5.5 mm tall mark: the cut wall stubs in masonry and insulation, the window band, the box as a block and five lamella strokes (every fourth lamella) with the bottom rail." + CONVENTION_EN,
                         "Ein 5,5 mm hohes Zeichen: die geschnittenen Wandstücke in Mauerwerk und Dämmung, das Fensterband, der Kasten als Block und fünf Lamellenstriche (jede vierte Lamelle) mit Unterschiene." + CONVENTION_DE),
        "retained": tx("Box, bottom rail, guide rail, the wall stubs and lamellae 2, 6, 10, 14 and 18, each bound to its own bone, so lift and tilt move them in every view.", "Kasten, Unterschiene, Führungsschiene, die Wandstücke und die Lamellen 2, 6, 10, 14 und 18, je an ihrem eigenen Knochen, sodass Hub und Kippen sie in jeder Ansicht bewegen."),
        "merged": tx("The 18 lamellae of 0.08 m pitch (0.16 mm at 1:500) are thinned to every fourth, a 0.32 m pitch of 0.64 mm: a simplified lamella hatch whose line count is a convention, not the real number.", "Die 18 Lamellen mit 0,08 m Teilung (0,16 mm bei 1:500) sind auf jede vierte ausgedünnt, 0,32 m Teilung und 0,64 mm: eine vereinfachte Lamellenschraffur, deren Linienzahl eine Konvention ist, nicht die echte Anzahl."),
        "omitted": tx("Thirteen lamellae, sill board, holders, everything inside the box.", "Dreizehn Lamellen, Fensterbank, Halter, alles im Kasten."),
        "thinElements": tx("Lamellae are 0.10 mm strokes; the guide rail and the window band are 0.04 and 0.14 mm wide fills that the outline pen carries.", "Lamellen sind 0,10-mm-Striche; Führungsschiene und Fensterband sind 0,04 und 0,14 mm breite Füllungen, die der Konturstift trägt."),
        "hatching": tx("No hatching.", "Keine Schraffur."),
        "styles": tx("One outline weight 0.18 mm, lamellae and glass 0.10 mm.", "Eine Konturstärke 0,18 mm, Lamellen und Glas 0,10 mm."),
        "overlays": tx("One cell per region.", "Eine Zelle je Region."),
    },
}

SUMMARY = tx(
    "A 1.2 m wide facade-mounted external venetian blind (raffstore) of 18 aluminium Z 90 lamellae (0.45 mm sheet, 90 mm wide, 80 mm pitch) hanging on two lift tapes from a 58 x 56 mm steel head rail behind a 150 x 210 mm aluminium cover panel, with two guide rails and a 90 x 20 mm bottom rail, shown in vertical section in front of a fixed double-glazed aluminium window with sill board in an insulated clay-block wall; retracted, lowered with slats horizontal, at 45 degrees or closed, with the shading effect on the glazing from the louvre geometry.",
    "Eine 1,2 m breite Vorbau-Außenjalousie (Raffstore) aus 18 Aluminium-Z-90-Lamellen (0,45-mm-Blech, 90 mm breit, 80 mm Abstand), die an zwei Aufzugsbändern an einer Stahl-Oberschiene 58 x 56 mm hinter einer 150 x 210 mm großen Aluminiumblende hängen, mit zwei Führungsschienen und einer Unterschiene 90 x 20 mm, im Vertikalschnitt vor einem festverglasten, doppelt verglasten Aluminiumfenster mit Fensterbank in einer gedämmten Ziegelwand; hochgezogen, abgelassen mit waagerechten, 45-Grad- oder geschlossenen Lamellen, mit der Verschattungswirkung auf die Verglasung aus der Lamellengeometrie.")

REFERENCES = [
    {"id": "hella-af-80", "citation": "Hella, external venetian blind AF 80 K, installation and operating manual (06/2006), component list: top rail 58 x 56 mm of galvanised sheet steel, slat 80 x 0.44 mm, bottom rail 80 x 18.6 mm of extruded aluminium, lifting tape 6.0 x 0.28 mm, plastic blind bearings.", "url": "https://www.manualslib.com/manual/1670901/Hella-Af-80-K.html?page=9"},
    {"id": "z90-price-list", "citation": "Raffstore price list for Z 90 lamellae (rev. 2026): top rail as a galvanised roll-formed U profile 58 x 56 mm, Z 90 lamella profile 93 x 20 mm, aluminium guide rails; package heights include the mounting bracket.", "url": "https://cdn.nextproducts.at/m20/30/43576730/etc/22/50116222.pdf"},
]

MATERIALS = [
    {"id": "steel", "title": tx("Galvanised steel (head rail, tilt shaft, brackets, holders)", "Verzinkter Stahl (Oberschiene, Wendewelle, Träger, Halter)"), "emissivity": 0.28, "solarAbsorptivity": 0.65, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210000000000, "strengthPa": 235000000, "source": "literature", "reference": "en-iso-10456"},
    {"id": "polyamide", "title": tx("Polyamide and polyester (tape bearings, guide pins, end caps, lift tapes)", "Polyamid und Polyester (Bandlager, Führungsnippel, Endkappen, Aufzugsbänder)"), "emissivity": 0.9, "solarAbsorptivity": 0.85, "densityKgM3": 1150, "conductivityWmK": 0.25, "specificHeatJKgK": 1600, "source": "literature", "reference": "en-iso-10456"},
]

FRAME_REGION = {
    "id": "window-frame", "title": tx("Aluminium window frame", "Aluminium-Fensterrahmen"), "kind": "surface", "model": "conditioned", "material": "aluminium",
    "areaM2": 0.28, "characteristicLengthM": 1.25, "geometry": "vertical-plate", "skyViewFactor": 0.5,
    "layers": [{"id": "frame-profile", "title": tx("Thermally broken aluminium profile 70 mm", "Thermisch getrenntes Aluminiumprofil 70 mm"), "kind": "metal", "material": "aluminium", "resistanceM2KW": 0.187, "source": "authored-estimate", "thicknessM": 0.07, "heatCapacityJM2K": 28000, "reference": "en-iso-10077-1"}],
    "heatCapacityJM2K": 28000, "conditioned": {"interiorTempC": 20, "interiorFilmM2KW": 0.13}, "source": "authored-estimate", "reference": "en-iso-10077-1",
}

LIFTS = {"retracted": k.LIFT_MAX, "lift-1": 0.27, "lift-2": 0.54, "lift-3": 0.81, "lift-4": 1.08}
TILTS = {"lowered-cut-off-45": -45, "lowered-closed": -90, "closed-in-gust": -90, "tilt-1": 30, "tilt-2": -20, "tilt-3": -30, "tilt-4": -60, "tilt-5": -75}


def rig(asset):
    m = k.masses()
    old = {b["id"]: b for b in asset["rig"]["bones"]}
    facade, boxb, rail = old["facade"], old["box"], old["bottom-rail"]
    facade["mass"]["kg"] = round(m["facade"], 1)
    boxb["title"] = tx("Cover panel with head rail and guide rails", "Blende mit Oberschiene und Führungsschienen")
    boxb["mass"].update({"kg": round(m["box"], 1), "comX": 0.08, "comY": 0.13})
    rail.update({"x": k.BX, "y": k.RAIL_Y})
    rail["mass"]["kg"] = round(m["bottom-rail"], 2)
    bones = [facade, boxb, rail]
    for i in range(N):
        slide = dict(old["slide-0"])
        slide.update({"id": "slide-%d" % i, "title": tx("Lift of slat %d" % (i + 1), "Hub der Lamelle %d" % (i + 1)), "x": k.BX, "y": r(k.slat_y(i))})
        bones.append(slide)
    for i in range(N):
        slat = json.loads(json.dumps(old["slat-0"]))
        slat.update({"id": "slat-%d" % i, "parent": "slide-%d" % i, "title": tx("Slat %d (top is 1) tilting about its centre" % (i + 1), "Lamelle %d (oben ist 1), kippt um ihre Mitte" % (i + 1))})
        slat["mass"]["kg"] = round(m["slat"], 3)
        bones.append(slat)
    asset["rig"]["bones"] = bones
    asset["dimensions"]["massKg"] = round(sum(b.get("mass", {}).get("kg", 0) for b in bones), 2)


def poses(asset):
    for pose in asset["poses"]:
        bones = {}
        if pose["id"] in LIFTS:
            dy, lift = k.lifted(LIFTS[pose["id"]])
            bones["bottom-rail"] = {"dy": r(lift)}
            for i, d in enumerate(dy):
                if d > 0:
                    bones["slide-%d" % i] = {"dy": r(d)}
        if pose["id"] in TILTS:
            for i in range(N):
                bones["slat-%d" % i] = {"angleDeg": TILTS[pose["id"]]}
        pose["bones"] = bones
        regions = pose["thermal"]["regions"]
        regions["window-frame"] = dict(regions["glazing-behind"])
        pose["thermal"]["regions"] = {key: regions[key] for key in ("lamellae", "box-and-rails", "glazing-behind", "window-frame", "facade-wall")}
    for clip in asset["clips"]:
        tracks = []
        for track in clip["tracks"]:
            if track["target"].startswith("region:window-frame:"):
                continue
            tracks.append(track)
            if track["target"] == "region:glazing-behind:sunlitFraction":
                twin = json.loads(json.dumps(track))
                twin["target"] = "region:window-frame:sunlitFraction"
                tracks.append(twin)
        clip["tracks"] = tracks


def thermal(asset):
    regions = [x for x in asset["thermal"]["regions"] if x["id"] != "window-frame"]
    by = {x["id"]: x for x in regions}
    by["lamellae"]["title"] = tx("Venetian lamellae, both faces", "Jalousielamellen, beide Seiten")
    by["box-and-rails"]["title"] = tx("Cover panel, guide rails with holders and bottom rail", "Blende, Führungsschienen mit Haltern und Unterschiene")
    by["facade-wall"]["title"] = tx("Insulated wall above and below the window with reveal soffit and sill board", "Gedämmte Wand über und unter dem Fenster mit Sturzuntersicht und Fensterbank")
    by["glazing-behind"]["title"] = tx("Double glazing behind the blind", "Doppelverglasung hinter der Jalousie")
    by["glazing-behind"]["characteristicLengthM"] = 1.18
    frame = json.loads(json.dumps(FRAME_REGION))
    out = [by["lamellae"], by["box-and-rails"], by["glazing-behind"], frame, by["facade-wall"]]
    for region in out:
        region["areaM2"] = AREAS[region["id"]]
    asset["thermal"]["regions"] = out
    notes = asset["thermal"]["assumptions"]
    extra = tx("The window is a fixed light: the 1.13 x 1.18 m insulating glass unit (1.1 x 1.15 m sight size) in a thermally broken aluminium frame of 50 mm face with Uf about 2.8 W/m2K (an authored estimate for a 70 mm profile); the frame is its own region with its projected face as area and is shaded like the glazing; the sill board counts to the wall surface it covers. The head rail with motor, shaft and brackets is the mechanism inside the cover and no surface of a region.",
               "Das Fenster ist eine Festverglasung: die Isolierglasscheibe 1,13 x 1,18 m (Lichtmaß 1,1 x 1,15 m) in einem thermisch getrennten Aluminiumrahmen mit 50 mm Ansicht und Uf von etwa 2,8 W/m2K (Schätzwert für ein 70-mm-Profil); der Rahmen ist eine eigene Region mit seiner Ansichtsfläche und wird wie die Verglasung beschattet; die Fensterbank zählt zur Wandoberfläche, die sie abdeckt. Die Oberschiene mit Motor, Welle und Trägern ist die Mechanik in der Blende und keine Regionsoberfläche.")
    asset["thermal"]["assumptions"] = [n for n in notes if "fixed light" not in n["en"]] + [extra]


def apply(asset):
    asset["summary"] = SUMMARY
    refs = [x for x in asset["provenance"]["references"] if x["id"] not in ("hella-af-80", "z90-price-list")]
    for ref in refs:
        if ref["id"] == "blind-catalogues":
            ref["citation"] = "Manufacturer data for external venetian blinds (raffstore) with Z lamellae 90 mm, pitch about 80 mm, lamella thickness 0.4 to 0.5 mm aluminium (about 0.15 kg per 1.13 m lamella), cover panels of about 150 x 200 mm for blind heights up to 1.5 m, bottom rail and lateral guide rails. Typical values consulted."
    asset["provenance"]["references"] = refs + REFERENCES
    asset["provenance"]["notes"] = tx(
        "Glazing sunlit fractions are the direct-beam transmission of the louvre array at a reference solar profile angle of 35 degrees (tau = 1 - w sin(alpha + beta) / (p cos alpha)); diffuse light is not blocked. The blind is modelled as built: 18 lamellae fill the height from 65 mm below the head rail to the bottom rail, which hangs 25 mm below the sill line in front of the drip; retracted, the stack of 6 mm pitch and the bottom rail fit under the head rail inside the 210 mm cover.",
        "Die Besonnungsanteile der Verglasung sind die Direktstrahlungs-Durchlässigkeit der Lamellenfläche bei einem Referenz-Sonnenhöhenwinkel von 35 Grad (tau = 1 - w sin(alpha + beta) / (p cos alpha)); diffuses Licht wird nicht abgeschirmt. Die Jalousie ist wie gebaut modelliert: 18 Lamellen füllen die Höhe von 65 mm unter der Oberschiene bis zur Unterschiene, die 25 mm unter der Brüstungslinie vor der Tropfkante hängt; hochgezogen passen das Paket mit 6 mm Teilung und die Unterschiene unter der Oberschiene in die 210 mm hohe Blende.")
    asset["materials"] = [x for x in asset["materials"] if x["id"] not in ("steel", "polyamide")] + MATERIALS
    rig(asset)
    for anchor in asset["anchors"]:
        if anchor["id"] == "blind-centre":
            anchor["x"] = k.BX
        if anchor["id"] == "box-bracket":
            anchor.update({"x": 0, "y": k.ANCHOR_V, "title": tx("Head rail bracket anchors in the head wall", "Anker der Oberschienenträger in der Sturzwand")})
    poses(asset)
    thermal(asset)
    asset["structure"]["assumptions"][0] = tx(
        "The masonry leaf forms the rigid fixed root; cover with head rail, bottom rail and every lamella are masses at their poses (lamella pack positions are exact). The blind hangs on two lift tapes from the head rail, which two brackets anchor through the insulation into the block; the guide rails take the wind through the guide pins.",
        "Die Mauerschale bildet die starre, eingespannte Wurzel; Blende mit Oberschiene, Unterschiene und jede Lamelle sind Massen in ihren Posen (Lamellenpaketpositionen sind exakt). Der Behang hängt an zwei Aufzugsbändern an der Oberschiene, die zwei Träger durch die Dämmung im Ziegel verankern; die Führungsschienen nehmen den Wind über die Führungsnippel auf.")
    geometry = {
        "50": {"elevation": elevation50(), "front": front50(), "plan": plan50()},
        "100": {"elevation": elevation100(), "front": front100(), "plan": plan100()},
        "200": coarse(200, k.FR_FACE, 0.13),
        "500": coarse(500, 0.075, 0.18),
    }
    for scale, rep in asset["representations"].items():
        new = {}
        for key, value in rep.items():
            if key == "decisions":
                new["sections"] = {"elevation": {"plane": 0, "label": tx("A-A", "A-A")}}
            if key != "sections":
                new[key] = value
        new["geometry"] = geometry[scale]
        new["decisions"] = TEXTS[scale]
        if new["pens"].get("texture") is None:
            new["pens"]["texture"] = 0.1
        asset["representations"][scale] = new
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


def write_manifest(old_summary):
    source = open(MANIFEST, encoding="utf-8").read()
    updated = source
    for lang in ("en", "de"):
        a, b = json.dumps(old_summary[lang], ensure_ascii=False), json.dumps(SUMMARY[lang], ensure_ascii=False)
        if a == b:
            continue
        if updated.count(a) != 1:
            raise SystemExit("manifest distinction (%s) found %d times" % (lang, updated.count(a)))
        updated = updated.replace(a, b)
    json.loads(updated)
    if updated != source:
        with open(MANIFEST, "w", encoding="utf-8") as f:
            f.write(updated)
        print("manifest distinction updated")


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
        write_manifest(before["asset"]["summary"])
    else:
        document["asset"]["volumes"] = json.load(open(SOURCE, encoding="utf-8"))["volumes"]
        folder = os.path.join(T, "🗑️generated", "r3-external-venetian-blind")
        os.makedirs(folder, exist_ok=True)
        with open(os.path.join(folder, "🔣️.json"), "w", encoding="utf-8") as f:
            json.dump(document, f, ensure_ascii=False, indent=2)
        print("candidate written; mass %.2f kg" % document["asset"]["dimensions"]["massKg"])
