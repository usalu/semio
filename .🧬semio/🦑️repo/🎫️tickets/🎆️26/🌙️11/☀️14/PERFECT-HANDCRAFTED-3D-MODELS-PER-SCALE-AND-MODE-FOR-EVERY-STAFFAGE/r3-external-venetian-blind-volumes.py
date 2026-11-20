"""🪟 Every solid of components/external-venetian-blind at 1:50, 1:100, 1:200 and 1:500, stated part by part; writes volumes/components/external-venetian-blind.json."""

import importlib.util
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("kit", os.path.join(T, "r3-external-venetian-blind-kit.py"))
k = importlib.util.module_from_spec(spec)
spec.loader.exec_module(k)
OUT = os.path.join(T, "volumes", "components", "external-venetian-blind.json")
Z = k.Z


def r(x):
    return round(x + 0.0, 6)


def tx(en, de):
    return {"en": en, "de": de}


def base(id, kind, bone, material, role, tone):
    return {"id": id, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}


def rep(step, count=2, axis=2):
    t = [0, 0, 0]
    t[axis] = r(step)
    return {"count": count, "translation": t}


def box(id, bone, material, role, tone, lo, hi, repeat=None, **extra):
    s = base(id, "box", bone, material, role, tone)
    s["size"] = [r(hi[i] - lo[i]) for i in range(3)]
    s["transform"] = {"translation": [r((hi[i] + lo[i]) / 2) for i in range(3)]}
    if repeat is not None:
        s["repeat"] = repeat
    s.update(extra)
    return s


def extrude(id, bone, material, role, tone, plane, profile, a, b, holes=None, repeat=None, **extra):
    s = base(id, "extrude", bone, material, role, tone)
    s["profile"] = [[r(p), r(q)] for p, q in profile]
    if holes:
        s["holes"] = [[[r(p), r(q)] for p, q in hole] for hole in holes]
    s["plane"] = plane
    s["from"], s["to"] = r(min(a, b)), r(max(a, b))
    if repeat is not None:
        s["repeat"] = repeat
    s.update(extra)
    return s


def rod(id, bone, material, role, tone, origin, axis, profile, repeat=None, **extra):
    s = base(id, "revolve", bone, material, role, tone)
    s["profile"] = [[r(a), r(b)] for a, b in profile]
    s["axis"] = axis
    s["origin"] = [r(c) for c in origin]
    if repeat is not None:
        s["repeat"] = repeat
    s.update(extra)
    return s


def ellipsoid(id, bone, material, role, tone, centre, radii, **extra):
    s = base(id, "ellipsoid", bone, material, role, tone)
    s["radii"] = [r(x) for x in radii]
    s["transform"] = {"translation": [r(c) for c in centre]}
    s.update(extra)
    return s


def ring(id, bone, material, role, tone, x0, x1, outer, hole, **extra):
    """🔲 Rectangular ring in the facade plane (Y, Z rectangles) extruded through X."""
    (oy0, oy1, oz), (hy0, hy1, hz) = outer, hole
    return extrude(id, bone, material, role, tone, "yz", [(oy0, -oz), (oy1, -oz), (oy1, oz), (oy0, oz)], x0, x1, holes=[[(hy0, -hz), (hy0, hz), (hy1, hz), (hy1, -hz)]], **extra)


def circle(cx, cy, radius, n=12):
    import math
    return [(cx + radius * math.cos(2 * math.pi * i / n), cy + radius * math.sin(2 * math.pi * i / n)) for i in range(n)]


def vbox(points):
    return [(x, y - k.BOX_Y0) for x, y in points]


FACADE, BOX, RAIL = "facade", "box", "bottom-rail"
X_IN, XB0, XB1, XE1 = k.X_IN, k.X_BLOCK0, k.X_BLOCK1, k.X_EPS1
SILL, HEAD, H = k.SILL_Y, k.HEAD_Y, k.H
GUIDE_V0 = k.GUIDE_Y0 - k.BOX_Y0
CUT_SILL = ["-v", "-w", "+w"]
CUT_HEAD = ["+v", "-w", "+w"]


def wall50():
    courses = [
        box("block-sill", FACADE, "masonry", "body", 0.55, (XB0, 0, -Z), (XB1, k.COURSE, Z), repeat=rep(k.COURSE + k.BED, 4, 1)),
        box("bed-joint-sill", FACADE, "masonry", "detail", 0.75, (XB0, k.COURSE, -Z), (XB1, k.COURSE + k.BED, Z), repeat=rep(k.COURSE + k.BED, 4, 1)),
        box("lintel", FACADE, "masonry", "body", 0.45, (XB0, HEAD, -Z), (XB1, HEAD + k.LINTEL_H, Z)),
        box("bed-joint-head", FACADE, "masonry", "detail", 0.75, (XB0, HEAD + k.LINTEL_H, -Z), (XB1, HEAD + k.LINTEL_H + k.BED, Z), repeat=rep(k.COURSE + k.BED, 2, 1)),
        box("block-head-full", FACADE, "masonry", "body", 0.55, (XB0, HEAD + 0.125, -Z), (XB1, HEAD + 0.125 + k.COURSE, Z)),
        box("block-head-cut", FACADE, "masonry", "body", 0.55, (XB0, HEAD + 0.375, -Z), (XB1, H, Z)),
    ]
    skins = [
        box("plaster-sill", FACADE, "masonry", "body", 0.85, (X_IN, 0, -Z), (XB0, SILL, Z)),
        box("plaster-reveal-sill", FACADE, "masonry", "detail", 0.85, (X_IN, SILL, -Z), (k.FR_X0, SILL + 0.015, Z)),
        box("eps-sill", FACADE, "eps", "body", 0.92, (XB1, 0, -Z), (XE1, SILL, Z)),
        box("render-sill", FACADE, "masonry", "body", 0.7, (XE1, 0, -Z), (0, SILL, Z)),
        extrude("sill-bed", FACADE, "eps", "detail", 0.92, "xy", k.sill_bed(), -Z, Z),
        box("plaster-head", FACADE, "masonry", "body", 0.85, (X_IN, HEAD, -Z), (XB0, H, Z)),
        box("plaster-reveal-head", FACADE, "masonry", "detail", 0.85, (X_IN, HEAD - 0.015, -Z), (k.FR_X0, HEAD, Z)),
        box("eps-head", FACADE, "eps", "body", 0.92, (XB1, HEAD, -Z), (XE1, H, Z)),
        box("render-head", FACADE, "masonry", "body", 0.7, (XE1, HEAD, -Z), (0, H, Z)),
        box("render-soffit", FACADE, "masonry", "detail", 0.7, (k.FR_X1, HEAD - 0.01, -Z), (0, HEAD, Z)),
    ]
    return courses + skins


def window50():
    jamb = [(k.JAMB_Z, k.FR_X0), (Z, k.FR_X0), (Z, k.FR_X1), (k.JAMB_Z, k.FR_X1)]
    return [
        extrude("frame-sill", FACADE, "aluminium", "body", 0.35, "xy", k.frame_rail(True), -k.JAMB_Z, k.JAMB_Z),
        extrude("frame-head", FACADE, "aluminium", "body", 0.35, "xy", k.frame_rail(False), -k.JAMB_Z, k.JAMB_Z),
        extrude("frame-jamb-left", FACADE, "aluminium", "body", 0.35, "zx", jamb, SILL, HEAD),
        extrude("frame-jamb-right", FACADE, "aluminium", "body", 0.35, "zx", [(-w, u) for w, u in reversed(jamb)], SILL, HEAD),
        box("frame-stop-left", FACADE, "aluminium", "body", 0.35, (k.FR_STOP_X, k.SIGHT_Y0, k.SIGHT_Z), (k.FR_X1, k.SIGHT_Y1, k.JAMB_Z)),
        box("frame-stop-right", FACADE, "aluminium", "body", 0.35, (k.FR_STOP_X, k.SIGHT_Y0, -k.JAMB_Z), (k.FR_X1, k.SIGHT_Y1, -k.SIGHT_Z)),
        ring("glazing-bead", FACADE, "aluminium", "detail", 0.4, k.BEAD_X0, k.GL_X0, (k.REB_Y0, k.REB_Y1, k.JAMB_Z), (k.SIGHT_Y0, k.SIGHT_Y1, k.SIGHT_Z)),
        box("pane-in", FACADE, "glass", "glazing", 0.92, (k.GL_X0, k.GL_Y0, -k.GL_Z), (k.GL_X0 + k.PANE, k.GL_Y1, k.GL_Z)),
        ring("edge-spacer", FACADE, "aluminium", "detail", 0.25, k.GL_X0 + k.PANE, k.GL_X1 - k.PANE, (k.GL_Y0, k.GL_Y1, k.GL_Z), (k.GL_Y0 + k.SPACER, k.GL_Y1 - k.SPACER, k.GL_Z - k.SPACER)),
        box("pane-out", FACADE, "glass", "glazing", 0.92, (k.GL_X1 - k.PANE, k.GL_Y0, -k.GL_Z), (k.GL_X1, k.GL_Y1, k.GL_Z)),
        extrude("sill-board", FACADE, "aluminium", "detail", 0.6, "xy", k.sill_board(), -Z, Z),
    ]


def box50():
    bearing = [(k.HR_X0 + 0.0025, 0.1505), (k.HR_X1 - 0.0025, 0.1505), (k.HR_X1 - 0.0025, 0.2005), (k.HR_X0 + 0.0025, 0.2005)]
    return [
        extrude("cover", BOX, "aluminium", "body", 0.5, "xy", k.cover(), -Z, Z),
        extrude("end-plate-left", BOX, "aluminium", "body", 0.5, "zx", k.end_plate(1), k.SHEET, k.BOX_H - k.SHEET),
        extrude("end-plate-right", BOX, "aluminium", "body", 0.5, "zx", k.end_plate(-1), k.SHEET, k.BOX_H - k.SHEET),
        extrude("head-rail", BOX, "steel", "body", 0.4, "xy", k.head_rail(), -k.HR_Z, k.HR_Z),
        extrude("bracket", BOX, "steel", "detail", 0.3, "xy", k.bracket(), k.BRACKET_Z0, k.BRACKET_Z1, repeat=rep(-(k.BRACKET_Z0 + k.BRACKET_Z1))),
        rod("bracket-nut", BOX, "steel", "detail", 0.25, (0.004, k.ANCHOR_V, (k.BRACKET_Z0 + k.BRACKET_Z1) / 2), [1, 0, 0], [(0, 0.01), (0.002, 0.01), (0.002, 0.0085), (0.01, 0.0085)], repeat=rep(-(k.BRACKET_Z0 + k.BRACKET_Z1))),
        rod("motor", BOX, "steel", "detail", 0.3, (k.BX, k.SHAFT_V, -k.MOTOR_Z), [0, 0, 1], [(0, k.MOTOR_R), (2 * k.MOTOR_Z, k.MOTOR_R)]),
        rod("tilt-shaft", BOX, "steel", "detail", 0.25, (k.BX, k.SHAFT_V, k.MOTOR_Z), [0, 0, 1], [(0, k.SHAFT_R), (0.56 - k.MOTOR_Z, k.SHAFT_R)], repeat=rep(-(0.56 + k.MOTOR_Z))),
        extrude("tape-bearing", BOX, "polyamide", "detail", 0.2, "xy", bearing, k.BEARING_Z0, k.BEARING_Z1, holes=[circle(k.BX, k.SHAFT_V, k.SHAFT_R + 0.0005)], repeat=rep(-(k.BEARING_Z0 + k.BEARING_Z1))),
        extrude("guide-left", BOX, "aluminium", "body", 0.45, "zx", k.guide_rail(1), GUIDE_V0, k.SHEET),
        extrude("guide-right", BOX, "aluminium", "body", 0.45, "zx", k.guide_rail(-1), GUIDE_V0, k.SHEET),
        rod("rail-holder-top", BOX, "steel", "detail", 0.3, (0, k.HOLDER_TOP_Y - k.BOX_Y0, k.HOLDER_Z), [1, 0, 0], [(0, 0.018), (0.003, 0.018), (0.003, 0.007), (k.RAIL_GX0, 0.007)], repeat=rep(-2 * k.HOLDER_Z)),
        rod("rail-holder-bottom", BOX, "steel", "detail", 0.3, (0, k.HOLDER_BOTTOM_Y - k.BOX_Y0, k.HOLDER_Z), [1, 0, 0], [(0, 0.018), (0.003, 0.018), (0.003, 0.007), (k.RAIL_GX0, 0.007)], repeat=rep(-2 * k.HOLDER_Z)),
    ]


def tape_piece(id, bone, lo, hi, weights, role="texture", **extra):
    return box(id, bone, "polyamide", role, 0.3, (lo[0] - k.TAPE_T / 2, lo[1], k.TAPE_Z - k.TAPE_W / 2), (hi[0] + k.TAPE_T / 2, hi[1], k.TAPE_Z + k.TAPE_W / 2), repeat=rep(-2 * k.TAPE_Z), weights=[{"bone": b, "weight": w} for b, w in weights], **extra)


def tapes(**extra):
    """🎗 Lift tapes: each run between two lamellae is two pieces blended between the two slides, so the tape shortens as the stack closes."""
    out = [tape_piece("lift-tape-top", BOX, (k.BX, k.slat_y(0) - k.BOX_Y0), (k.BX, 0.15), [(BOX, 0.5), ("slide-0", 0.5)], **extra)]
    for i in range(k.N - 1):
        out.append(tape_piece("lift-tape-%d-a" % i, "slide-%d" % i, (0, -k.PITCH / 2), (0, 0), [("slide-%d" % i, 0.75), ("slide-%d" % (i + 1), 0.25)], **extra))
        out.append(tape_piece("lift-tape-%d-b" % i, "slide-%d" % i, (0, -k.PITCH), (0, -k.PITCH / 2), [("slide-%d" % i, 0.25), ("slide-%d" % (i + 1), 0.75)], **extra))
    last = k.N - 1
    gap = k.slat_y(last) - (k.RAIL_Y + k.RAIL_H / 2)
    out.append(tape_piece("lift-tape-rail", "slide-%d" % last, (0, -gap + 0.003), (0, -0.003), [("slide-%d" % last, 0.5), (RAIL, 0.5)], **extra))
    return out


def blind50():
    out = []
    section = k.slat()
    for i in range(k.N):
        out.append(extrude("slat-%d" % i, "slat-%d" % i, "aluminium", "texture", 0.6, "xy", section, -k.SLAT_Z, k.SLAT_Z, texture=True))
    for i in range(1, k.N, 2):
        out.append(rod("guide-pin-%d" % i, "slat-%d" % i, "polyamide", "detail", 0.2, (0, 0.001, k.SLAT_Z), [0, 0, 1], [(0, k.PIN_R), (k.PIN_Z1 - k.SLAT_Z, k.PIN_R)], repeat=rep(-(k.PIN_Z1 + k.SLAT_Z))))
    out += [
        extrude("bottom-rail", RAIL, "aluminium", "body", 0.45, "xy", k.bottom_rail(), -k.RAIL_Z, k.RAIL_Z, holes=[k.bottom_rail_hole()]),
        extrude("bottom-rail-cap", RAIL, "polyamide", "detail", 0.2, "xy", k.bottom_rail(), k.RAIL_Z, k.RAIL_Z + k.CAP, repeat=rep(-(2 * k.RAIL_Z + k.CAP))),
        rod("bottom-rail-pin", RAIL, "polyamide", "detail", 0.2, (0, 0, k.RAIL_Z + k.CAP), [0, 0, 1], [(0, k.PIN_R), (k.PIN_Z1 - k.RAIL_Z - k.CAP, k.PIN_R)], repeat=rep(-(k.PIN_Z1 + k.RAIL_Z + k.CAP))),
    ]
    return out + tapes(texture=True)


def of(id, target, **extra):
    s = {"id": id, "of": target}
    s.update(extra)
    return s


def th(target, region, layer=None, faces=None):
    s = {"id": "th-" + target, "of": target, "region": region}
    if layer:
        s["layer"] = layer
    if faces:
        s["sectionFaces"] = faces
    return s


def thermal50():
    wall = "facade-wall"
    out = [
        th("plaster-sill", wall, "plaster", ["-u"] + CUT_SILL), th("plaster-reveal-sill", wall, "plaster", ["-u", "-w", "+w"]),
        th("plaster-head", wall, "plaster", ["-u"] + CUT_HEAD), th("plaster-reveal-head", wall, "plaster", ["-u", "-w", "+w"]),
        th("block-sill", wall, "block", CUT_SILL), th("bed-joint-sill", wall, "block", ["-w", "+w"]),
        th("lintel", wall, "block", ["-w", "+w"]), th("bed-joint-head", wall, "block", ["-w", "+w"]), th("block-head-full", wall, "block", ["-w", "+w"]), th("block-head-cut", wall, "block", CUT_HEAD),
        th("eps-sill", wall, "eps", CUT_SILL), th("sill-bed", wall, "eps", ["-w", "+w"]), th("eps-head", wall, "eps", CUT_HEAD),
        th("render-sill", wall, "render", ["-u", "+v"] + CUT_SILL), th("render-head", wall, "render", ["-u"] + CUT_HEAD), th("render-soffit", wall, "render", ["-u", "+v", "-w", "+w"]),
        th("pane-in", "glazing-behind", "pane-in", ["-u"]),
        box("th-cavity", FACADE, "glass", "glazing", 0.97, (k.GL_X0 + k.PANE, k.GL_Y0 + k.SPACER, -k.GL_Z + k.SPACER), (k.GL_X1 - k.PANE, k.GL_Y1 - k.SPACER, k.GL_Z - k.SPACER), region="glazing-behind", layer="cavity"),
        th("edge-spacer", "glazing-behind", "cavity"),
        th("pane-out", "glazing-behind", "pane-out"),
        th("frame-sill", "window-frame", "frame-profile", ["-u", "-v", "+v"]), th("frame-head", "window-frame", "frame-profile", ["-u", "-v", "+v"]),
        th("frame-jamb-left", "window-frame", "frame-profile", ["-u", "-w", "+w", "-v", "+v"]), th("frame-jamb-right", "window-frame", "frame-profile", ["-u", "-w", "+w", "-v", "+v"]),
        th("frame-stop-left", "window-frame", "frame-profile", ["-u", "+w", "-v", "+v"]), th("frame-stop-right", "window-frame", "frame-profile", ["-u", "-w", "-v", "+v"]),
        th("glazing-bead", "window-frame", "frame-profile", ["-u", "+u", "-v", "+v", "-w", "+w"]),
        th("sill-board", wall, "render", ["-v", "-w", "+w"]),
    ]
    for target in ("cover", "end-plate-left", "end-plate-right", "guide-left", "guide-right", "rail-holder-top", "rail-holder-bottom", "bottom-rail", "bottom-rail-cap"):
        out.append(th(target, "box-and-rails", "box-wall"))
    for i in range(k.N):
        out.append(th("slat-%d" % i, "lamellae", "lam-sheet"))
    return out


def member(id, bone, material, lo, hi, tone=0.4, **extra):
    return box(id, bone, material, "body", tone, lo, hi, member=bone, part="member", **extra)


def masonry_structure(feature):
    return [
        member("member-masonry-sill", FACADE, "masonry", (XB0, 0, -Z), (XB1, SILL, Z)),
        member("member-lintel", FACADE, "masonry", (XB0, HEAD, -Z), (XB1, HEAD + k.LINTEL_H, Z), tone=0.3),
        member("member-masonry-head", FACADE, "masonry", (XB0, HEAD + k.LINTEL_H, -Z), (XB1, H, Z)),
        box("joint-lintel-bearing", FACADE, "masonry", "detail", 0.2, (XB0, HEAD, Z - max(feature, 0.02)), (XB1, HEAD + k.LINTEL_H, Z), repeat=rep(-(2 * Z - max(feature, 0.02))), member=FACADE, part="joint"),
        box("support-wall-foot", FACADE, "masonry", "ground", 0.2, (X_IN, 0, -Z), (0, max(feature, 0.02), Z), member=FACADE, part="support", anchor="wall-foot"),
        ellipsoid("mass-masonry-sill", FACADE, "masonry", "detail", 0.3, (-0.1675, SILL / 2, 0), (0.1, 0.1, 0.1), member=FACADE, part="mass"),
        ellipsoid("mass-masonry-head", FACADE, "masonry", "detail", 0.3, (-0.1675, (HEAD + H) / 2, 0), (0.1, 0.08, 0.08), member=FACADE, part="mass"),
    ]


def structure50():
    out = masonry_structure(0.0075)
    out += [
        of("member-head-rail", "head-rail", member=BOX, part="member"),
        of("member-bracket", "bracket", member=BOX, part="member"),
        rod("member-anchor", BOX, "steel", "detail", 0.25, (-0.14, k.ANCHOR_V, (k.BRACKET_Z0 + k.BRACKET_Z1) / 2), [1, 0, 0], [(0, 0.005), (0.152, 0.005)], repeat=rep(-(k.BRACKET_Z0 + k.BRACKET_Z1)), member=BOX, part="member"),
        of("joint-anchor-nut", "bracket-nut", member=BOX, part="joint"),
        of("joint-tape-bearing", "tape-bearing", member=BOX, part="joint"),
        of("member-guide-left", "guide-left", member=BOX, part="member"),
        of("member-guide-right", "guide-right", member=BOX, part="member"),
        of("joint-rail-holder-top", "rail-holder-top", member=BOX, part="joint"),
        of("joint-rail-holder-bottom", "rail-holder-bottom", member=BOX, part="joint"),
        of("member-bottom-rail", "bottom-rail", member=RAIL, part="member"),
        of("joint-bottom-rail-pin", "bottom-rail-pin", member=RAIL, part="joint"),
        rod("mass-motor", BOX, "steel", "detail", 0.3, (k.BX, k.SHAFT_V, -k.MOTOR_Z), [0, 0, 1], [(0, k.MOTOR_R), (2 * k.MOTOR_Z, k.MOTOR_R)], member=BOX, part="mass"),
    ]
    for i in range(k.N):
        out.append(of("member-slat-%d" % i, "slat-%d" % i, member="slat-%d" % i, part="member"))
    for i in range(1, k.N, 2):
        out.append(of("joint-guide-pin-%d" % i, "guide-pin-%d" % i, member="slat-%d" % i, part="joint"))
    for piece in tapes():
        out.append(of("member-" + piece["id"], piece["id"], member=RAIL, part="member"))
    return out


def scale50():
    return {"body": wall50() + window50() + box50() + blind50(), "thermal": thermal50(), "structure": structure50()}


def window_coarse(tone=0.35):
    return [
        ring("window-frame", FACADE, "aluminium", "body", tone, k.FR_X0, k.FR_X1, (SILL, HEAD, Z), (k.SIGHT_Y0, k.SIGHT_Y1, k.SIGHT_Z)),
        box("glass", FACADE, "glass", "glazing", 0.92, (k.GL_X0, k.SIGHT_Y0, -k.SIGHT_Z), (k.GL_X1, k.SIGHT_Y1, k.SIGHT_Z)),
    ]


def thermal_coarse(body, slats, hardware):
    wall = "facade-wall"
    merged = any(s["id"] == "lintel" for s in body)
    out = []
    for s in body:
        i = s["id"]
        if i == "masonry-sill":
            out.append(th(i, wall, None, ["-u", "+v"] + CUT_SILL))
        elif i == "lintel":
            out.append(th(i, wall, None, ["-v", "-w", "+w"]))
        elif i == "masonry-head":
            out.append(th(i, wall, None, (["-u"] if merged else ["-u", "-v"]) + CUT_HEAD))
        elif i == "plaster-sill":
            out.append(th(i, wall, None, ["-u"] + CUT_SILL))
        elif i == "plaster-head":
            out.append(th(i, wall, None, ["-u"] + CUT_HEAD))
        elif i == "insulation-sill":
            out.append(th(i, wall, None, ["+v"] + CUT_SILL))
        elif i == "insulation-head":
            out.append(th(i, wall, None, CUT_HEAD))
        elif i == "window-frame":
            out.append(th(i, "window-frame", None, ["-u", "-v", "+v", "-w", "+w"]))
        elif i == "sill-board":
            out.append(th(i, wall, None, ["-v", "-u", "-w", "+w"]))
        elif i == "glass":
            out.append(th(i, "glazing-behind", None, ["-u", "-v", "+v", "-w", "+w"]))
    for target in hardware:
        out.append(th(target, "box-and-rails"))
    for i in slats:
        out.append(th("slat-%d" % i, "lamellae"))
    return out


def scale100():
    line = [(-0.045, 0.008), (-0.038, 0.0), (0.038, 0.0), (0.045, -0.008)]
    section = k.slat(bead=0, line=line)
    cover = [(0, k.BOX_H - k.FLANGE), (0, k.BOX_H), (k.BOX_D, k.BOX_H), (k.BOX_D, 0), (k.BOX_D - k.SHEET, 0), (k.BOX_D - k.SHEET, k.BOX_H - k.SHEET), (k.SHEET, k.BOX_H - k.SHEET), (k.SHEET, k.BOX_H - k.FLANGE)]
    board = [(k.FR_X1, SILL), (k.SB_X1, SILL), (k.SB_X1, k.sb_top(k.SB_X1)), (k.FR_X1, k.SB_BACK_Y)]
    body = [
        box("plaster-sill", FACADE, "masonry", "body", 0.85, (X_IN, 0, -Z), (XB0, SILL, Z)),
        box("masonry-sill", FACADE, "masonry", "body", 0.55, (XB0, 0, -Z), (XB1, SILL, Z)),
        box("insulation-sill", FACADE, "eps", "body", 0.9, (XB1, 0, -Z), (0, SILL, Z)),
        box("plaster-head", FACADE, "masonry", "body", 0.85, (X_IN, HEAD, -Z), (XB0, H, Z)),
        box("lintel", FACADE, "masonry", "body", 0.45, (XB0, HEAD, -Z), (XB1, HEAD + 0.125, Z)),
        box("masonry-head", FACADE, "masonry", "body", 0.55, (XB0, HEAD + 0.125, -Z), (XB1, H, Z)),
        box("insulation-head", FACADE, "eps", "body", 0.9, (XB1, HEAD, -Z), (0, H, Z)),
        extrude("sill-board", FACADE, "aluminium", "detail", 0.6, "xy", board, -Z, Z),
    ] + window_coarse() + [
        extrude("cover", BOX, "aluminium", "body", 0.5, "xy", cover, -Z, Z),
        extrude("end-plate-left", BOX, "aluminium", "body", 0.5, "zx", k.end_plate(1, 0.015), 0, k.BOX_H - k.SHEET),
        extrude("end-plate-right", BOX, "aluminium", "body", 0.5, "zx", k.end_plate(-1, 0.015), 0, k.BOX_H - k.SHEET),
        box("head-rail", BOX, "steel", "body", 0.4, (k.HR_X0, k.HR_V0, -k.HR_Z + 0.001), (k.HR_X1, k.HR_V1, k.HR_Z - 0.001)),
        box("bracket", BOX, "steel", "detail", 0.3, (k.SHEET, 0.16, k.BRACKET_Z0), (k.HR_X0, 0.175, k.BRACKET_Z1), repeat=rep(-(k.BRACKET_Z0 + k.BRACKET_Z1))),
        box("guide-left", BOX, "aluminium", "body", 0.45, (k.RAIL_GX0, GUIDE_V0, k.RAIL_GZ0), (k.RAIL_GX1, 0, Z)),
        box("guide-right", BOX, "aluminium", "body", 0.45, (k.RAIL_GX0, GUIDE_V0, -Z), (k.RAIL_GX1, 0, -k.RAIL_GZ0)),
        rod("rail-holder", BOX, "steel", "detail", 0.3, (0, k.HOLDER_TOP_Y - k.BOX_Y0, k.HOLDER_Z), [1, 0, 0], [(0, 0.0075), (k.RAIL_GX0, 0.0075)], repeat=rep(-2 * k.HOLDER_Z)),
        rod("rail-holder-low", BOX, "steel", "detail", 0.3, (0, k.HOLDER_BOTTOM_Y - k.BOX_Y0, k.HOLDER_Z), [1, 0, 0], [(0, 0.0075), (k.RAIL_GX0, 0.0075)], repeat=rep(-2 * k.HOLDER_Z)),
    ]
    for i in range(k.N):
        body.append(extrude("slat-%d" % i, "slat-%d" % i, "aluminium", "texture", 0.6, "xy", section, -k.SLAT_Z, k.SLAT_Z, texture=True))
    body.append(box("bottom-rail", RAIL, "aluminium", "body", 0.45, (-k.RAIL_W / 2, -k.RAIL_H / 2, -k.RAIL_Z - k.CAP), (k.RAIL_W / 2, k.RAIL_H / 2, k.RAIL_Z + k.CAP)))
    thermal = thermal_coarse(body, range(k.N), ["cover", "end-plate-left", "end-plate-right", "guide-left", "guide-right", "rail-holder", "rail-holder-low", "bottom-rail"])
    structure = masonry_structure(0.015) + [
        of("member-head-rail", "head-rail", member=BOX, part="member"),
        of("member-bracket", "bracket", member=BOX, part="member"),
        of("member-guide-left", "guide-left", member=BOX, part="member"),
        of("member-guide-right", "guide-right", member=BOX, part="member"),
        of("joint-rail-holder", "rail-holder", member=BOX, part="joint"),
        of("joint-rail-holder-low", "rail-holder-low", member=BOX, part="joint"),
        box("joint-tape-bearing", BOX, "polyamide", "detail", 0.2, (k.HR_X0, k.HR_V0 - 0.004, k.BEARING_Z0), (k.HR_X1, k.HR_V0 + 0.011, k.BEARING_Z1), repeat=rep(-(k.BEARING_Z0 + k.BEARING_Z1)), member=BOX, part="joint"),
        of("member-bottom-rail", "bottom-rail", member=RAIL, part="member"),
    ]
    for piece in tapes():
        piece.update(member=RAIL, part="member", id="member-" + piece["id"])
        structure.append(piece)
    return {"body": body, "thermal": thermal, "structure": structure}


ZSTRIP = k.slat(thickness=0.001, bead=0, line=[(-0.045, 0.008), (-0.038, 0.0), (0.038, 0.0), (0.045, -0.008)])


def scale200():
    cover = [(0, k.BOX_H - k.SHEET), (0, k.BOX_H), (k.BOX_D, k.BOX_H), (k.BOX_D, 0), (k.BOX_D - k.SHEET, 0), (k.BOX_D - k.SHEET, k.BOX_H - k.SHEET)]
    body = [
        box("masonry-sill", FACADE, "masonry", "body", 0.55, (X_IN, 0, -Z), (XB1, SILL, Z)),
        box("insulation-sill", FACADE, "eps", "body", 0.9, (XB1, 0, -Z), (0, SILL, Z)),
        box("masonry-head", FACADE, "masonry", "body", 0.55, (X_IN, HEAD, -Z), (XB1, H, Z)),
        box("insulation-head", FACADE, "eps", "body", 0.9, (XB1, HEAD, -Z), (0, H, Z)),
    ] + window_coarse(0.4) + [
        extrude("cover", BOX, "aluminium", "body", 0.5, "xy", cover, -Z, Z),
        extrude("end-plate-left", BOX, "aluminium", "body", 0.5, "zx", k.end_plate(1, 0.02, 0), 0, k.BOX_H - k.SHEET),
        extrude("end-plate-right", BOX, "aluminium", "body", 0.5, "zx", k.end_plate(-1, 0.02, 0), 0, k.BOX_H - k.SHEET),
        box("head-rail", BOX, "steel", "body", 0.4, (k.HR_X0, k.HR_V0, -Z + 0.02), (k.HR_X1, k.HR_V1, Z - 0.02)),
        box("guide-left", BOX, "aluminium", "detail", 0.45, (k.RAIL_GX0, GUIDE_V0, k.RAIL_GZ0), (k.RAIL_GX1, 0, Z)),
        box("guide-right", BOX, "aluminium", "detail", 0.45, (k.RAIL_GX0, GUIDE_V0, -Z), (k.RAIL_GX1, 0, -k.RAIL_GZ0)),
    ]
    for i in range(k.N):
        body.append(extrude("slat-%d" % i, "slat-%d" % i, "aluminium", "texture", 0.6, "xy", ZSTRIP, -k.SLAT_Z, k.SLAT_Z, texture=True))
    body.append(box("bottom-rail", RAIL, "aluminium", "body", 0.45, (-k.RAIL_W / 2, -k.RAIL_H / 2, -k.RAIL_GZ0), (k.RAIL_W / 2, k.RAIL_H / 2, k.RAIL_GZ0)))
    thermal = thermal_coarse(body, range(k.N), ["cover", "end-plate-left", "end-plate-right", "guide-left", "guide-right", "bottom-rail"])
    structure = [
        member("member-masonry-sill", FACADE, "masonry", (X_IN, 0, -Z), (XB1, SILL, Z)),
        member("member-masonry-head", FACADE, "masonry", (X_IN, HEAD, -Z), (XB1, H, Z)),
        box("joint-lintel-bearing", FACADE, "masonry", "detail", 0.2, (X_IN, HEAD, Z - 0.03), (XB1, HEAD + 0.12, Z), repeat=rep(-(2 * Z - 0.03)), member=FACADE, part="joint"),
        box("support-wall-foot", FACADE, "masonry", "ground", 0.2, (X_IN, 0, -Z), (0, 0.03, Z), member=FACADE, part="support", anchor="wall-foot"),
        ellipsoid("mass-masonry-sill", FACADE, "masonry", "detail", 0.3, (-0.175, SILL / 2, 0), (0.12, 0.12, 0.12), member=FACADE, part="mass"),
        ellipsoid("mass-masonry-head", FACADE, "masonry", "detail", 0.3, (-0.175, (HEAD + H) / 2, 0), (0.12, 0.1, 0.1), member=FACADE, part="mass"),
        of("member-head-rail", "head-rail", member=BOX, part="member"),
        of("member-guide-left", "guide-left", member=BOX, part="member"),
        of("member-guide-right", "guide-right", member=BOX, part="member"),
        box("joint-head-rail-fixing", BOX, "steel", "detail", 0.3, (0, k.HR_V0, k.BRACKET_Z0 - 0.0), (k.HR_X0, k.HR_V0 + 0.03, k.BRACKET_Z1), repeat=rep(-(k.BRACKET_Z0 + k.BRACKET_Z1)), member=BOX, part="joint"),
        of("member-bottom-rail", "bottom-rail", member=RAIL, part="member"),
    ]
    return {"body": body, "thermal": thermal, "structure": structure}


SLATS_500 = [1, 5, 9, 13, 17]


def scale500():
    body = [
        box("masonry-sill", FACADE, "masonry", "body", 0.55, (X_IN, 0, -Z), (XB1, SILL, Z)),
        box("insulation-sill", FACADE, "eps", "body", 0.9, (XB1, 0, -Z), (0, SILL, Z)),
        box("masonry-head", FACADE, "masonry", "body", 0.55, (X_IN, HEAD, -Z), (XB1, H, Z)),
        box("insulation-head", FACADE, "eps", "body", 0.9, (XB1, HEAD, -Z), (0, H, Z)),
        ring("window-frame", FACADE, "aluminium", "body", 0.4, k.FR_X0, k.FR_X1, (SILL, HEAD, Z), (SILL + 0.075, HEAD - 0.075, Z - 0.075)),
        box("glass", FACADE, "glass", "glazing", 0.92, (k.GL_X0, SILL + 0.075, -Z + 0.075), (k.GL_X1, HEAD - 0.075, Z - 0.075)),
        box("cover", BOX, "aluminium", "body", 0.5, (0, 0, -Z), (k.BOX_D, k.BOX_H, Z)),
        box("guide-left", BOX, "aluminium", "detail", 0.45, (k.RAIL_GX0, GUIDE_V0, k.RAIL_GZ0), (k.RAIL_GX1, 0, Z)),
        box("guide-right", BOX, "aluminium", "detail", 0.45, (k.RAIL_GX0, GUIDE_V0, -Z), (k.RAIL_GX1, 0, -k.RAIL_GZ0)),
    ]
    for i in SLATS_500:
        body.append(extrude("slat-%d" % i, "slat-%d" % i, "aluminium", "texture", 0.6, "xy", ZSTRIP, -k.RAIL_GZ0, k.RAIL_GZ0, texture=True))
    body.append(box("bottom-rail", RAIL, "aluminium", "body", 0.45, (-k.RAIL_W / 2, -k.RAIL_H / 2, -k.RAIL_GZ0), (k.RAIL_W / 2, k.RAIL_H / 2, k.RAIL_GZ0)))
    thermal = thermal_coarse(body, SLATS_500, ["cover", "guide-left", "guide-right", "bottom-rail"])
    for i in range(k.N):
        if i not in SLATS_500:
            thermal.append(extrude("th-slat-%d" % i, "slat-%d" % i, "aluminium", "texture", 0.6, "xy", ZSTRIP, -k.RAIL_GZ0, k.RAIL_GZ0, texture=True, region="lamellae"))
    structure = [
        member("member-masonry-sill", FACADE, "masonry", (X_IN, 0, -Z), (XB1, SILL, Z)),
        member("member-masonry-head", FACADE, "masonry", (X_IN, HEAD, -Z), (XB1, H, Z)),
        box("support-wall-foot", FACADE, "masonry", "ground", 0.2, (X_IN, 0, -Z), (0, 0.075, Z), member=FACADE, part="support", anchor="wall-foot"),
        ellipsoid("mass-masonry-sill", FACADE, "masonry", "detail", 0.3, (-0.175, SILL / 2, 0), (0.14, 0.14, 0.14), member=FACADE, part="mass"),
        ellipsoid("mass-masonry-head", FACADE, "masonry", "detail", 0.3, (-0.175, (HEAD + H) / 2, 0), (0.14, 0.12, 0.12), member=FACADE, part="mass"),
        box("member-head-rail", BOX, "steel", "body", 0.4, (k.BX - 0.0375, k.BOX_H - 0.08, -Z), (k.BX + 0.0375, k.BOX_H - 0.005, Z), member=BOX, part="member"),
        of("member-guide-left", "guide-left", member=BOX, part="member"),
        of("member-guide-right", "guide-right", member=BOX, part="member"),
        of("member-bottom-rail", "bottom-rail", member=RAIL, part="member"),
    ]
    return {"body": body, "thermal": thermal, "structure": structure}


DECISIONS = {
    "50": [
        tx("Body: the facade strip as built: 15 mm plaster with its reveal returns, four courses of 238 mm clay block on 12 mm bed joints, the 113 mm flat lintel with one and a half courses above, 70 mm EPS, 10 mm render with the soffit return, the insulation wedge and the bent 2 mm aluminium sill board with upstand, 5 degree fall, drip leg and lip. The fixed light is a 70 mm aluminium frame of 50 mm face (rails, jambs, exterior stops, interior glazing bead ring) with the 4-16-4 unit as two panes and a 12 mm edge spacer ring.",
           "Körper: der Fassadenstreifen wie gebaut: 15 mm Putz mit Laibungsrückführung, vier Schichten 238-mm-Ziegel auf 12 mm Lagerfugen, der 113 mm hohe Flachsturz mit eineinhalb Schichten darüber, 70 mm EPS, 10 mm Außenputz mit Sturzuntersicht, der Dämmkeil und die gekantete 2-mm-Aluminiumfensterbank mit Aufkantung, 5 Grad Gefälle, Tropfkante und Umbug. Die Festverglasung ist ein 70 mm tiefer Aluminiumrahmen mit 50 mm Ansicht (Riegel, Pfosten, äußere Anschläge, innerer Glasleistenring) mit der Scheibe 4-16-4 als zwei Gläser und einem 12 mm Randverbundring."),
        tx("Body, blind: the cover panel is a bent 2 mm sheet (wall flange, top, fascia, hem) open below, closed by two flanged end plates; inside hang the 58 x 56 mm steel head rail (open below, with lips) in two strap brackets on stand-off anchors, the tubular motor on the 12 mm tilt shaft and the two tape bearings. Two 30 x 20 mm C guide rails on four spacer holders, 18 Z 90 lamellae as real 0.45 mm sections with cambered web, opposed edge flanges and rolled beads (every second one with guide pins), the hollow 90 x 20 mm bottom rail with end caps and pins, and two 8 mm lift tapes whose pieces are blended between neighbouring slides so that they shorten with the stack.",
           "Körper, Behang: die Blende ist ein gekantetes 2-mm-Blech (Wandflansch, Deckel, Front, Umbug), unten offen, mit zwei geflanschten Seitenteilen geschlossen; darin hängen die Stahl-Oberschiene 58 x 56 mm (unten offen, mit Lippen) in zwei Bügelträgern auf Abstandsankern, der Rohrmotor auf der 12-mm-Wendewelle und die zwei Bandlager. Zwei C-Führungsschienen 30 x 20 mm auf vier Abstandshaltern, 18 Z-90-Lamellen als echte 0,45-mm-Profile mit gewölbtem Steg, gegenläufigen Randflanschen und Bördeln (jede zweite mit Führungsnippeln), die hohle Unterschiene 90 x 20 mm mit Endkappen und Nippeln sowie zwei 8-mm-Aufzugsbänder, deren Stücke zwischen benachbarten Hubknochen überblendet sind, sodass sie sich mit dem Paket verkürzen."),
        tx("Omitted in the body: the ladder cords (2 mm, below the 7.5 mm feature, and a cord cannot follow lift and tilt on rigid bones), the gaskets (4 mm) and the thermal break strips of the frame, screws and rivets.",
           "Im Körper weggelassen: die Leiterkordeln (2 mm, unter der Einzelheit von 7,5 mm, und eine Kordel kann Hub und Wendung auf starren Knochen nicht folgen), die Dichtungen (4 mm) und die Dämmstege des Rahmens, Schrauben und Niete."),
        tx("Thermal: the wall in its four real layers (plaster, block with joints and lintel, EPS with the sill wedge, render with the soffit; cut faces and glued faces marked), the glazing as inner pane, cavity with the edge spacer and outer pane, the window frame as its own region with its projected face (stops; bead and rebate faces marked), the sill board on the wall surface it covers, the blind surfaces (cover, end plates, rails, holders, bottom rail) as box-and-rails; the head rail with motor, shaft and brackets is mechanism inside the cover and no region surface, and every lamella with both faces of its 0.45 mm sheet.",
           "Thermisch: die Wand in ihren vier echten Schichten (Putz, Ziegel mit Fugen und Sturz, EPS mit Fensterbankkeil, Außenputz mit Untersicht; Schnitt- und Klebeflächen markiert), die Verglasung als Innenscheibe, Zwischenraum mit Randverbund und Außenscheibe, der Fensterrahmen als eigene Region mit seiner Ansichtsfläche (Anschläge; Glasleiste und Falzflächen markiert), die Fensterbank auf der Wandoberfläche, die sie abdeckt, die Behangflächen (Blende, Seitenteile, Schienen, Halter, Unterschiene) als Kasten und Schienen; die Oberschiene mit Motor, Welle und Trägern ist Mechanik in der Blende und keine Regionsoberfläche, und jede Lamelle mit beiden Seiten ihres 0,45-mm-Blechs."),
        tx("Structure: the load path of the suspended blind: lamellae (beams between the guide pins) and bottom rail hang on the lift tapes from the tape bearings on the head rail, the head rail sits in two brackets whose anchors pass the insulation into the block; wind goes through the guide pins into the rails and their four holders. The masonry leaf carries: sill courses on the fixed foot, the lintel on its bearings at the cut reveals, the head courses on the lintel; the motor is the concentrated mass in the box.",
           "Tragwerk: der Lastpfad des hängenden Behangs: Lamellen (Träger zwischen den Führungsnippeln) und Unterschiene hängen an den Aufzugsbändern in den Bandlagern der Oberschiene, die Oberschiene sitzt in zwei Trägern, deren Anker durch die Dämmung in den Ziegel greifen; Wind geht über die Führungsnippel in die Schienen und ihre vier Halter. Die Mauerschale trägt: Brüstungsschichten auf dem eingespannten Fuß, der Sturz auf seinen Auflagern an den geschnittenen Laibungen, die Sturzschichten auf dem Sturz; der Motor ist die Einzelmasse im Kasten."),
    ],
    "100": [
        tx("Body: plaster, block leaf and insulation (EPS and render merged to 80 mm) per stub, the lintel as its own block, the sill board as one wedge, the window as one 70 mm frame ring of 50 mm face with the glass as a 24 mm slab; the cover is a plain three-sided sheet with flat end plates, the head rail a solid 58 x 56 mm bar on two bracket arms, the guide rails solid 30 x 20 mm bars on four 15 mm holders, the lamellae Z sections without camber and beads, the bottom rail a solid 90 x 20 mm bar.",
           "Körper: Putz, Ziegelschale und Dämmung (EPS und Außenputz zu 80 mm zusammengefasst) je Wandstück, der Sturz als eigener Block, die Fensterbank als ein Keil, das Fenster als ein 70 mm tiefer Rahmenring mit 50 mm Ansicht und dem Glas als 24-mm-Platte; die Blende ist ein glattes dreiseitiges Blech mit ebenen Seitenteilen, die Oberschiene ein voller Stab 58 x 56 mm auf zwei Trägerarmen, die Führungsschienen volle Stäbe 30 x 20 mm auf vier 15-mm-Haltern, die Lamellen Z-Profile ohne Wölbung und Bördel, die Unterschiene ein voller Stab 90 x 20 mm."),
        tx("Thermal: one solid per region part from this body (layers merged, B2): wall masses with cut and glued faces marked, the glass slab with only its outer face, the frame ring with its outer face, the sill wedge on the wall, the blind surfaces without the mechanism and the 18 lamella sheets.",
           "Thermisch: je Regionsteil ein Körper aus diesem Modell (Schichten zusammengefasst, B2): Wandmassen mit markierten Schnitt- und Klebeflächen, die Glasplatte nur mit ihrer Außenfläche, der Rahmenring mit Außenfläche, der Fensterbankkeil an der Wand, die Behangflächen ohne Mechanik und die 18 Lamellenbleche."),
        tx("Structure: block leaf, lintel with bearings, head rail, bracket arms, rails with holders, bottom rail and the two lift tapes hanging from 15 mm bearing marks under the head rail; lamella beams, anchors and pins are below 15 mm and left out.",
           "Tragwerk: Ziegelschale, Sturz mit Auflagern, Oberschiene, Trägerarme, Schienen mit Haltern, Unterschiene und die zwei Aufzugsbänder an 15-mm-Lagermarken unter der Oberschiene; Lamellenträger, Anker und Nippel liegen unter 15 mm und entfallen."),
    ],
    "200": [
        tx("Body: each wall stub is a masonry mass (plaster and block, lintel merged) and an 80 mm insulation mass; the window is the frame ring with the glass slab, without sill board; the cover is an angle of top and fascia between end plates with the head rail as a bar through the full width, without brackets and holders; guide rails are slender bars, the lamellae 18 plain Z strips of 1 mm (the drawing strokes every lamella), the bottom rail a bar between the rails.",
           "Körper: jedes Wandstück ist eine Mauermasse (Putz und Ziegel, Sturz eingeschlossen) und eine 80-mm-Dämmmasse; das Fenster ist der Rahmenring mit Glasplatte, ohne Fensterbank; die Blende ist ein Winkel aus Deckel und Front zwischen Seitenteilen mit der Oberschiene als durchgehendem Stab, ohne Träger und Halter; die Führungsschienen sind schlanke Stäbe, die Lamellen 18 schlichte Z-Streifen von 1 mm (die Zeichnung strichelt jede Lamelle), die Unterschiene ein Stab zwischen den Schienen."),
        tx("Thermal: the same masses per region; only the total area is compared at this scale.", "Thermisch: dieselben Massen je Region; in diesem Maßstab wird nur die Gesamtfläche verglichen."),
        tx("Structure: two masonry members with the lintel bearings and the foot support, the head rail with its two fixings to the wall, both guide rails and the bottom rail; tapes and lamellae are not resolvable at 30 mm.",
           "Tragwerk: zwei Mauerwerksglieder mit Sturzauflagern und Fußauflager, die Oberschiene mit ihren zwei Wandbefestigungen, beide Führungsschienen und die Unterschiene; Bänder und Lamellen sind bei 30 mm nicht auflösbar."),
    ],
    "500": [
        tx("Body: masonry and insulation mass per stub, the window as a 75 mm frame ring with the glass slab, the box as one closed 150 x 210 mm block (the head rail and the stack are inside it), two slender guide bars, five lamella strips (every fourth, as the drawing) across the full width and the bottom rail; the model is minimal.",
           "Körper: Mauer- und Dämmmasse je Wandstück, das Fenster als 75-mm-Rahmenring mit Glasplatte, der Kasten als ein geschlossener Block 150 x 210 mm (Oberschiene und Paket liegen darin), zwei schlanke Führungsstäbe, fünf Lamellenstreifen (jede vierte, wie die Zeichnung) über die volle Breite und die Unterschiene; das Modell ist minimal."),
        tx("Thermal: the same solids per region; the lamella region keeps all 18 sheets (the thirteen that the body does not show are thermal solids of their own), because the lamella surface is the largest exchange surface of the asset and five strips would leave three quarters of it out.", "Thermisch: dieselben Körper je Region; die Lamellenregion behält alle 18 Bleche (die dreizehn, die der Körper nicht zeigt, sind eigene thermische Körper), weil die Lamellenfläche die größte Austauschfläche des Assets ist und fünf Streifen drei Viertel davon wegließen."),
        tx("Structure: the two masonry members on the foot support, the head rail as a 75 mm bar inside the box, the guide rails and the bottom rail.", "Tragwerk: die zwei Mauerwerksglieder auf dem Fußauflager, die Oberschiene als 75-mm-Stab im Kasten, die Führungsschienen und die Unterschiene."),
    ],
}


def build():
    volumes = {}
    for scale, make in (("50", scale50), ("100", scale100), ("200", scale200), ("500", scale500)):
        lists = make()
        volumes[scale] = {"status": "authored", "denominator": int(scale), "body": lists["body"], "thermal": lists["thermal"], "structure": lists["structure"], "decisions": DECISIONS[scale]}
    return volumes


if __name__ == "__main__":
    volumes = build()
    with open(OUT, "w", encoding="utf-8") as f:
        json.dump({"volumes": volumes}, f, ensure_ascii=False, indent=1)
        f.write("\n")
    for scale, v in volumes.items():
        print(scale, {l: len(v[l]) for l in ("body", "thermal", "structure")})
