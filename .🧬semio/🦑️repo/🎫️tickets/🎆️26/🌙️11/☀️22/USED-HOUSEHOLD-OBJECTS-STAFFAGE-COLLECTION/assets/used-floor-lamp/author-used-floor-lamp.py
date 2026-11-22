"""💡 Authoritative author script of the staffage asset `used-floor-lamp`: states every hand-chosen number of the used 1990s halogen uplighter (Deckenfluter) with its rotary dimmer, yellowed glass bowl, domed base plate and taped cable, stored on a cardboard sheet, and writes `🔣️.json` next to this file.

Run: `python3 author-used-floor-lamp.py` (writes the candidate and prints the reference measurements with a `[DEBUG] ` prefix).

@see ./author_parts.py — the numbers and the geometric helpers
@see ./author_texts.py — the bilingual decisions of every scale
@see ../../brief.md — the brief this asset follows
"""
import json
import math
import os

from author_parts import (BASE, BASE_TOP, BOWL, BOWL_RIM, BOWL_T, BOWL_U, CABLE_EXIT, CABLE_L, CABLE_R, CARD_S, CARD_SHIFT, CARD_T, CARRY_LIFT, CARRY_TILT, COUPLER_HALF, COUPLER_R, COUPLERS,
                          DIMMER, DIMMER_SEAM, FEATURE, G, GRIP_LOWER, GRIP_UPPER, GROMMET, GUARD_HALF, GUARD_R, GUARD_U, HEAD_U, HOLDER, KNOB, KNOB_OUT, KNOB_U, KNOB_V, LAMP_H, MOUTH_R,
                          PAD_R, PLUG_BODY, PLUG_COLLAR, PLUG_L, REFLECTOR, REFLECTOR_OUTLINE, RHO_GLASS, RHO_STEEL, RISER_L, SCALES, STEM_R, STEM_T, STEM_U, TAPE, TUBE_HALF, TUBE_R, TUBE_U, T,
                          base_profile, cable_angles, coupler_profile, d, revolve_area, revolve_moment, revolve_volume, rnd)
from author_texts import REP_DECISIONS, VOLUME_DECISIONS

HERE = os.path.dirname(os.path.abspath(__file__))
TONE = {"black": 0.14, "chrome": 0.62, "bowl": 0.84, "abs": 0.12, "knob": 0.55, "cable": 0.86, "tape": 0.08, "plug": 0.92, "card": 0.6, "ceramic": 0.9, "tube": 0.96, "mouth": 0.42}
PENS = {
    50: {"body": 0.25, "detail": 0.18, "texture": 0.13, "glazing": None, "ground": 0.35},
    100: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.35},
    200: {"body": 0.2, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.25},
    500: {"body": 0.2, "detail": None, "texture": None, "glazing": None, "ground": 0.25},
}
REST_CABLE, REST_RISER, _ = cable_angles(CARD_T, 0.0)
CARRY_CABLE, CARRY_RISER, _ = cable_angles(CARD_T + CARRY_LIFT, CARRY_TILT)
HOME_CABLE, HOME_RISER, _ = cable_angles(0.0, 0.0)
KNOB_FULL = 270.0
KNOB_DIM = 120.0
KNOB_CLICK = 25.0


def shape(id, bone, role, z, filled, tone, geometry, region=None, material=None, pen=None, regions=None, solid=None):
    """✏️ One 2D shape of a representation."""
    out = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, **geometry}
    if region is not None:
        out["region"] = region
    if regions is not None:
        out["regions"] = regions
    if material is not None:
        out["material"] = material
    if pen is not None:
        out["penMm"] = pen
    if solid is not None:
        out["solid"] = solid
    return out


def poly(points, closed=True):
    """🔷 A poly geometry."""
    return {"kind": "poly", "points": rnd([list(p) for p in points]), "closed": closed}


def turned(id, bone, z, tone, profile, region=None, material=None, pen=None, regions=None, role="body"):
    """🏺 A turned part drawn in any view as the revolved solid of its closed (axial, radial) half profile about its bone axis."""
    return shape(id, bone, role, z, True, tone, poly(profile), region, material, pen, regions, {"kind": "revolved", "axisBone": bone, "axis": "u", "axisX": 0, "axisZ": 0, "mirror": True})


def plug_shape(view, s, z):
    """🔌 The Euro contour plug body in a view: its side outline swept across its 36 mm width."""
    p = PLUG_BODY
    u0 = p["u0"] if s == 50 else 0.0
    prof = [(u0, p["bottom"]), (u0, p["top"] + 0.003), (u0 + 0.003, p["top"]), (p["u1"], p["top"]), (p["u1"], p["bottom"])] if s < 200 else [(0.0, p["bottom"]), (0.0, p["top"]), (p["u1"], p["top"]), (p["u1"], p["bottom"])]
    solid = None if view == "elevation" else {"kind": "extruded", "sweep": "w", "z0": -p["half"], "z1": p["half"]}
    return shape("plug", "plug", "body", z, True, TONE["plug"], poly(prof), "cable", "pp-white", None, None, solid)


def rod(id, bone, z, u0, u1, r, region, material, tone, view):
    """🥢 A straight round rod along its bone: a band in the elevation, the revolved solid in front and plan."""
    if view == "elevation":
        return shape(id, bone, "body", z, True, tone, poly([(u0, -r), (u1, -r), (u1, r), (u0, r)]), region, material)
    return turned(id, bone, z, tone, [(u0, 0.0), (u0, r), (u1, r), (u1, 0.0)], region, material)


def outline_of(loop):
    """🫥 The filled silhouette half profile of a closed shell loop: its outer run from the first point to the rim, closed through the axis."""
    k = max(range(len(loop)), key=lambda i: (loop[i][0], loop[i][1]))
    run = loop[: k + 1]
    return [(run[0][0], 0.0)] + run + [(run[-1][0], 0.0)]


def geometry(s):
    """🖼️ The three views of one scale, every part drawn individually for that scale."""
    views = {}
    bp = base_profile(s)
    for view in ("elevation", "front", "plan"):
        out = []
        plan = view == "plan"
        if view == "elevation":
            out.append(shape("cardboard", "cardboard", "body", 1, True, TONE["card"], poly([(0.0, 0.0), (CARD_S, 0.0), (CARD_S, CARD_T), (0.0, CARD_T)]), "cardboard", "cardboard"))
        elif view == "front":
            out.append(shape("cardboard", "cardboard", "body", 1, True, TONE["card"], {"kind": "rect", "x": -CARD_S / 2, "y": 0.0, "w": CARD_S, "h": CARD_T}, "cardboard", "cardboard"))
        else:
            out.append(shape("cardboard", "cardboard", "body", 1, True, TONE["card"], {"kind": "rect", "x": -CARD_S / 2, "y": -CARD_S / 2, "w": CARD_S, "h": CARD_S, "r": 0.004 if s < 200 else 0.0}, "cardboard", "cardboard"))
        zc = 2 if view != "front" else 1.2
        riser_end = RISER_L if s < 500 else RISER_L + PLUG_L
        out.append(rod("cable", "cable", zc, -0.010, CABLE_L, CABLE_R, "cable", "pvc-cable", TONE["cable"], view))
        out.append(rod("riser", "riser", zc + 0.1, 0.0, riser_end, CABLE_R, "cable", "pvc-cable", TONE["cable"], view))
        if s < 500:
            out.append(plug_shape(view, s, zc + 0.2))
        if s == 50:
            out.append(rod("tape", "cable", zc + 0.3, TAPE[0], TAPE[1], TAPE[2], "cable", "pvc-tape", TONE["tape"], view))
            if view == "elevation":
                out.append(shape("grommet", "cable", "body", zc + 0.4, True, TONE["tape"], poly([(a, -r) for a, r in GROMMET[1:-1]] + [(a, r) for a, r in reversed(GROMMET[1:-1])]), "cable", "pvc-black"))
                out.append(shape("plug-collar", "plug", "body", zc + 0.25, True, TONE["plug"], poly([(a, -r) for a, r in PLUG_COLLAR[1:-1]] + [(a, r) for a, r in reversed(PLUG_COLLAR[1:-1])]), "cable", "pp-white"))
            else:
                out.append(turned("grommet", "cable", zc + 0.4, TONE["tape"], GROMMET, "cable", "pvc-black"))
                out.append(turned("plug-collar", "plug", zc + 0.25, TONE["plug"], PLUG_COLLAR, "cable", "pp-white"))
        zl = 1.6 if plan else 0
        out.append(turned("base", "lamp", 3 - zl, TONE["black"], bp, "base", "steel-black"))
        stem = [(STEM_U[0], 0.0), (STEM_U[0], STEM_R), (STEM_U[1], STEM_R), (STEM_U[1], 0.0)]
        if s == 500:
            out.append(turned("stem", "lamp", 4 - zl, TONE["black"], stem, None, "steel-black", None, ["stem", "dimmer"]))
        else:
            out.append(turned("stem", "lamp", 4 - zl, TONE["black"], stem, "stem", "steel-black"))
        if s == 50:
            for k, u in enumerate(COUPLERS):
                out.append(turned(f"coupler-{k + 1}", "lamp", 4.3 + k * 0.01 - zl, 0.22, coupler_profile(u), "stem", "steel-black"))
        if s in DIMMER:
            out.append(turned("dimmer", "lamp", 5 - zl, TONE["abs"], DIMMER[s], "dimmer", "abs-black"))
        if s in KNOB:
            if view == "front":
                out.append(shape("knob", "knob", "body", 6, True, TONE["knob"], {"kind": "ellipse", "cx": 0.0, "cy": rnd(KNOB_U + CARD_T), "rx": 0.015, "ry": 0.015}, "dimmer", "abs-grey"))
                if s == 50:
                    out.append(shape("knob-pointer", "knob", "detail", 6.1, False, 0.95, poly([(0.0, KNOB_U + CARD_T + 0.004), (0.0, KNOB_U + CARD_T + 0.0125)], False), "dimmer", "abs-grey"))
            else:
                out.append(turned("knob", "knob", 6 - zl, TONE["knob"], KNOB[s], "dimmer", "abs-grey"))
        head_regions = ["reflector", "lamp-tube"] if s == 500 and plan else None
        if plan:
            out.append(shape("bowl", "bowl", "body", 5, True, TONE["bowl"], {"kind": "ellipse", "cx": 0.0, "cy": 0.0, "rx": BOWL_RIM, "ry": BOWL_RIM}, "bowl", "glass-yellowed"))
            out.append(shape("mouth", "head", "body", 6, True, TONE["mouth"], {"kind": "ellipse", "cx": 0.0, "cy": 0.0, "rx": 0.0688 if s < 500 else 0.07, "ry": 0.0688 if s < 500 else 0.07}, None if head_regions else "reflector", "chrome-steel", None, head_regions))
            if s < 500:
                out.append(shape("tube", "head", "body", 6.5, True, TONE["tube"], {"kind": "rect", "x": -TUBE_R, "y": -TUBE_HALF - TUBE_R, "w": 2 * TUBE_R, "h": 2 * (TUBE_HALF + TUBE_R), "r": TUBE_R if s == 50 else 0.0}, "lamp-tube", "quartz"))
            if s == 50:
                for sign in (-1, 1):
                    hw = HOLDER["size"]
                    out.append(shape(f"holder-{'l' if sign > 0 else 'r'}", "head", "body", 6.6, True, TONE["ceramic"], {"kind": "rect", "x": -hw[1] / 2, "y": sign * HOLDER["w"] - hw[2] / 2, "w": hw[1], "h": hw[2]}, "reflector", "steatite"))
            if s in (50, 100):
                out.append(shape("guard-v", "head", "detail", 7, False, 0.3, poly([(-GUARD_HALF, 0.0), (GUARD_HALF, 0.0)], False), "reflector", "chrome-steel"))
                out.append(shape("guard-w", "head", "detail", 7.1, False, 0.3, poly([(0.0, -GUARD_HALF), (0.0, GUARD_HALF)], False), "reflector", "chrome-steel"))
                out.append(shape("bowl-rim", "bowl", "detail", 5.5, False, 0.5, {"kind": "path", "d": circle_path(BOWL_RIM - BOWL_T)}, "bowl", "glass-yellowed"))
        else:
            if s < 500:
                if view == "elevation":
                    out.append(shape("tube", "head", "body", 6.9, True, TONE["tube"], {"kind": "ellipse", "cx": TUBE_U, "cy": 0.0, "rx": TUBE_R, "ry": TUBE_R}, "lamp-tube", "quartz"))
                else:
                    out.append(shape("tube", "head", "body", 6.9, True, TONE["tube"], {"kind": "rect", "x": -TUBE_HALF - TUBE_R, "y": rnd(HEAD_U + TUBE_U - TUBE_R + CARD_T), "w": 2 * (TUBE_HALF + TUBE_R), "h": 2 * TUBE_R}, "lamp-tube", "quartz"))
            head_regions = ["reflector", "lamp-tube"] if s == 500 else None
            out.append(turned("head", "head", 7, TONE["chrome"], REFLECTOR_OUTLINE[s], None if head_regions else "reflector", "chrome-steel", None, head_regions))
            out.append(turned("bowl", "bowl", 8, TONE["bowl"], outline_of(BOWL[s]), "bowl", "glass-yellowed"))
        out += details(view, s)
        views[view] = out
    return views


def circle_path(r, cx=0.0, cy=0.0):
    """🔵 An SVG path of a full circle."""
    return f"M {rnd(cx + r)} {rnd(cy)} A {rnd(r)} {rnd(r)} 0 1 0 {rnd(cx - r)} {rnd(cy)} A {rnd(r)} {rnd(r)} 0 1 0 {rnd(cx + r)} {rnd(cy)} Z"


def details(view, s):
    """🔎 Detail and texture strokes: dimmer seam, stem splices, chipped lacquer, tape wraps, the yellowed stain line of the bowl and the dead insects in it."""
    out = []
    y0 = CARD_T
    if s in (50, 100):
        if view == "elevation":
            out.append(shape("dimmer-seam", "lamp", "detail", 9, False, 0.4, poly([(DIMMER_SEAM, -0.022), (DIMMER_SEAM, -0.006)], False), "dimmer", "abs-black"))
            out.append(shape("dimmer-seam-rear", "lamp", "detail", 9.01, False, 0.4, poly([(DIMMER_SEAM, 0.022), (DIMMER_SEAM, 0.006)], False), "dimmer", "abs-black"))
        elif view == "front":
            out.append(shape("dimmer-seam", "lamp", "detail", 9, False, 0.4, poly([(-0.022, DIMMER_SEAM + y0), (-0.016, DIMMER_SEAM + y0)], False), "dimmer", "abs-black"))
            out.append(shape("dimmer-seam-left", "lamp", "detail", 9.01, False, 0.4, poly([(0.016, DIMMER_SEAM + y0), (0.022, DIMMER_SEAM + y0)], False), "dimmer", "abs-black"))
    if s == 100 and view != "plan":
        for k, u in enumerate(COUPLERS):
            if view == "elevation":
                out.append(shape(f"splice-{k + 1}", "lamp", "detail", 9.1 + k * 0.01, False, 0.4, poly([(u, -STEM_R), (u, STEM_R)], False), "stem", "steel-black"))
            else:
                out.append(shape(f"splice-{k + 1}", "lamp", "detail", 9.1 + k * 0.01, False, 0.4, poly([(-STEM_R, u + y0), (STEM_R, u + y0)], False), "stem", "steel-black"))
    if s == 100 and view == "elevation":
        for k, u in enumerate((TAPE[0] + 0.01, TAPE[1] - 0.01)):
            out.append(shape(f"tape-{k + 1}", "cable", "detail", 9.3 + k * 0.01, False, 0.1, poly([(u, -CABLE_R), (u, CABLE_R)], False), "cable", "pvc-tape"))
    if s != 50:
        return out
    if view == "elevation":
        for k, (h, v) in enumerate(((0.005, -0.143), (0.009, -0.139), (0.006, 0.141), (0.38, -0.0125), (0.72, -0.0125), (1.31, 0.0125))):
            sg = -1 if v < 0 else 1
            if h < 0.05:
                out.append(shape(f"chip-{k + 1}", "lamp", "texture", 9.4 + k * 0.01, False, 0.75, poly([(h, v), (h + 0.002, v + sg * 0.006)], False), "base", "steel-black"))
            else:
                out.append(shape(f"scratch-{k + 1}", "lamp", "texture", 9.4 + k * 0.01, False, 0.75, poly([(h, v), (h + 0.03, v)], False), "stem", "steel-black"))
        for k, u in enumerate((0.20, 0.215, 0.23, 0.245)):
            out.append(shape(f"tape-wrap-{k + 1}", "cable", "texture", 9.6 + k * 0.01, False, 0.35, poly([(u, -TAPE[2]), (u + 0.006, TAPE[2])], False), "cable", "pvc-tape"))
        out.append(shape("stain-line", "bowl", "texture", 9.7, False, 0.6, poly([(0.03, -0.157), (0.03, 0.157)], False), "bowl", "glass-yellowed"))
        out.append(shape("rim-line", "bowl", "detail", 9.71, False, 0.5, poly([(0.1, -0.176), (0.1, 0.176)], False), "bowl", "glass-yellowed"))
    elif view == "front":
        for k, (z, h) in enumerate(((-0.141, 0.006), (0.137, 0.010), (-0.0125, 0.52), (0.0125, 0.86), (-0.0125, 1.38))):
            if h < 0.05:
                out.append(shape(f"chip-{k + 1}", "lamp", "texture", 9.4 + k * 0.01, False, 0.75, poly([(z, h + y0), (z + (0.006 if z < 0 else -0.006), h + 0.002 + y0)], False), "base", "steel-black"))
            else:
                out.append(shape(f"scratch-{k + 1}", "lamp", "texture", 9.4 + k * 0.01, False, 0.75, poly([(z, h + y0), (z, h + 0.03 + y0)], False), "stem", "steel-black"))
        out.append(shape("stain-line", "bowl", "texture", 9.7, False, 0.6, poly([(-0.157, LAMP_H - 0.07 + y0), (0.157, LAMP_H - 0.07 + y0)], False), "bowl", "glass-yellowed"))
        out.append(shape("rim-line", "bowl", "detail", 9.71, False, 0.5, poly([(-0.176, LAMP_H + y0), (0.176, LAMP_H + y0)], False), "bowl", "glass-yellowed"))
    else:
        for k, (x, z, a) in enumerate(((0.10, 0.04, 30), (-0.06, 0.11, 110), (0.02, -0.12, 70), (-0.11, -0.05, 150), (0.13, -0.06, 10), (0.07, 0.12, 60))):
            dx, dz = 0.006 * math.cos(math.radians(a)), 0.006 * math.sin(math.radians(a))
            out.append(shape(f"insect-{k + 1}", "bowl", "texture", 7.5 + k * 0.01, False, 0.2, poly([(x - dx, z - dz), (x + dx, z + dz)], False), "bowl", "glass-yellowed"))
        for k, x in enumerate((-0.335, -0.35, -0.365, -0.38)):
            out.append(shape(f"tape-wrap-{k + 1}", "cable", "texture", 2.6 + k * 0.01, False, 0.35, poly([(x, -TAPE[2]), (x - 0.006, TAPE[2])], False), "cable", "pvc-tape"))
    return out


def revolve(id, bone, material, tone, profile, **extra):
    """🏺 A revolve solid about the bone axis u."""
    return {"id": id, "kind": "revolve", "bone": bone, "material": material, "role": "body", "tone": tone, "profile": rnd([list(p) for p in profile]), "axis": [1.0, 0.0, 0.0], "origin": [0.0, 0.0, 0.0], **extra}


def capsule(id, bone, material, tone, p0, p1, r, **extra):
    """💊 A capsule solid."""
    return {"id": id, "kind": "capsule", "bone": bone, "material": material, "role": "body", "tone": tone, "p0": rnd(list(p0)), "p1": rnd(list(p1)), "r0": r, "r1": r, **extra}


def box(id, bone, material, tone, size, at, corner=0.0, **extra):
    """📦 A box solid."""
    return {"id": id, "kind": "box", "bone": bone, "material": material, "role": "body", "tone": tone, "size": rnd(list(size)), "cornerRadius": corner, "transform": {"translation": rnd(list(at))}, **extra}


def body(s):
    """🧊 The handcrafted body list of one scale."""
    out = [box("cardboard", "cardboard", "cardboard", TONE["card"], (CARD_S, CARD_T, CARD_S), (CARD_S / 2, CARD_T / 2, 0.0), 0.002 if s < 200 else 0.0, sheet=True)]
    out.append(revolve("base", "lamp", "steel-black", TONE["black"], base_profile(s)))
    out.append(revolve("stem", "lamp", "steel-black", TONE["black"], [(STEM_U[0], 0.0), (STEM_U[0], STEM_R), (STEM_U[1], STEM_R), (STEM_U[1], 0.0)], slender=True))
    if s == 50:
        for k, u in enumerate(COUPLERS):
            out.append(revolve(f"coupler-{k + 1}", "lamp", "steel-black", 0.22, coupler_profile(u)))
    if s in DIMMER:
        out.append(revolve("dimmer", "lamp", "abs-black", TONE["abs"], DIMMER[s]))
    if s in KNOB:
        out.append(revolve("knob", "knob", "abs-grey", TONE["knob"], KNOB[s]))
    if s == 50:
        out.append(box("knob-pointer", "knob", "abs-white", 0.95, (0.001, 0.009, 0.002), (KNOB_OUT + 0.0003, 0.0075, 0.0), texture=True))
    out.append(revolve("reflector", "head", "chrome-steel", TONE["chrome"], REFLECTOR[s]))
    if s == 50:
        out.append(capsule("tube", "head", "quartz", TONE["tube"], (TUBE_U, 0.0, -TUBE_HALF), (TUBE_U, 0.0, TUBE_HALF), TUBE_R))
        for sign in (-1, 1):
            out.append(box(f"holder-{'l' if sign > 0 else 'r'}", "head", "steatite", TONE["ceramic"], HOLDER["size"], (HOLDER["u"], 0.0, sign * HOLDER["w"]), 0.002))
    if s in (50, 100):
        out.append(capsule("guard-v", "head", "chrome-steel", TONE["chrome"], (GUARD_U, -GUARD_HALF, 0.0), (GUARD_U, GUARD_HALF, 0.0), GUARD_R, slender=True))
        out.append(capsule("guard-w", "head", "chrome-steel", TONE["chrome"], (GUARD_U, 0.0, -GUARD_HALF), (GUARD_U, 0.0, GUARD_HALF), GUARD_R, slender=True))
    out.append(revolve("bowl", "bowl", "glass-yellowed", TONE["bowl"], BOWL[s]))
    out.append(revolve("cable", "cable", "pvc-cable", TONE["cable"], [(-0.010, 0.0), (-0.010, CABLE_R), (CABLE_L, CABLE_R), (CABLE_L, 0.0)], slender=True))
    riser_end = RISER_L if s < 500 else RISER_L + PLUG_L
    out.append(revolve("riser", "riser", "pvc-cable", TONE["cable"], [(0.0, 0.0), (0.0, CABLE_R), (riser_end, CABLE_R), (riser_end, 0.0)], slender=True))
    if s == 50:
        out.append(revolve("grommet", "cable", "pvc-black", TONE["tape"], GROMMET))
        out.append(revolve("tape", "cable", "pvc-tape", TONE["tape"], [(TAPE[0], 0.0), (TAPE[0], TAPE[2]), (TAPE[1], TAPE[2]), (TAPE[1], 0.0)]))
        out.append(revolve("plug-collar", "plug", "pp-white", TONE["plug"], PLUG_COLLAR))
    p = PLUG_BODY
    if s == 50:
        out.append(box("plug", "plug", "pp-white", TONE["plug"], (p["u1"] - p["u0"], p["bottom"] - p["top"], 2 * p["half"]), ((p["u0"] + p["u1"]) / 2, (p["top"] + p["bottom"]) / 2, 0.0), 0.003))
    elif s in (100, 200):
        out.append(box("plug", "plug", "pp-white", TONE["plug"], (p["u1"], p["bottom"] - p["top"], 2 * p["half"]), (p["u1"] / 2, (p["top"] + p["bottom"]) / 2, 0.0), 0.002 if s == 100 else 0.0))
    return out


REGION_OF = {"cardboard": "cardboard", "base": "base", "stem": "stem", "coupler-1": "stem", "coupler-2": "stem", "dimmer": "dimmer", "knob": "dimmer", "reflector": "reflector", "tube": "lamp-tube",
             "holder-l": "reflector", "holder-r": "reflector", "guard-v": "reflector", "guard-w": "reflector", "bowl": "bowl", "cable": "cable", "riser": "cable", "grommet": "cable", "tape": "cable",
             "plug-collar": "cable", "plug": "cable"}


def thermal(s, bodies):
    """🔥 The thermal list of one scale: body surfaces as their regions, the halogen tube and the dimmer at true size as shells where the body leaves them out, and the plume outlet over the reflector mouth."""
    out = [{"id": f"th-{b['id']}", "of": b["id"], "region": REGION_OF[b["id"]]} for b in bodies if not b.get("texture")]
    if s != 50:
        half = TUBE_HALF if s == 100 else TUBE_HALF + 0.008
        out.append({**capsule("th-tube", "head", "quartz", TONE["tube"], (TUBE_U, 0.0, -half), (TUBE_U, 0.0, half), TUBE_R), "region": "lamp-tube", "layer": "quartz-wall"})
    if s == 500:
        out.append({**revolve("th-dimmer", "lamp", "abs-black", TONE["abs"], DIMMER[200]), "region": "dimmer", "layer": "abs-shell"})
    t = {50: 0.002, 100: 0.0025, 200: 0.003, 500: 0.0035}[s]
    out.append({**revolve("th-plume", "head", "chrome-steel", TONE["tube"], [(0.1415, 0.0), (0.1415, MOUTH_R), (0.1415 + t, MOUTH_R), (0.1415 + t, 0.0)], sheet=True), "role": "detail", "region": "lamp-tube", "tags": ["lamp-plume"]})
    return out


def ellipsoid(id, bone, r, at, material, part, member):
    """⚫ A compact joint or mass marker of the structure list."""
    return {"id": id, "kind": "ellipsoid", "bone": bone, "material": material, "role": "detail", "tone": 0.4, "radii": [r, r, r], "transform": {"translation": rnd(list(at))}, "member": member, "part": part}


def structure(s, coms):
    """🏗️ The structure list of one scale: the stem tube as a slender member at its true Ø25 section, the base plate, reflector, bowl, cable and cardboard as the members they are, joints at the boss, the splices and the neck, masses at their centres."""
    out = [{**capsule("m-stem", "lamp", "steel-black", 0.5, (BASE_TOP - 0.002, 0.0, 0.0), (HEAD_U + 0.002, 0.0, 0.0), STEM_R, slender=True), "member": "lamp", "part": "member"}]
    out.append({"id": "m-base", "of": "base", "member": "lamp", "part": "member"})
    out.append({"id": "m-head", "of": "reflector", "member": "head", "part": "member"})
    out.append({"id": "m-bowl", "of": "bowl", "member": "bowl", "part": "member"})
    out.append({"id": "m-cable", "of": "cable", "member": "cable", "part": "member"})
    out.append({"id": "m-riser", "of": "riser", "member": "riser", "part": "member"})
    out.append({"id": "m-cardboard", "of": "cardboard", "member": "cardboard", "part": "member"})
    if s in (50, 100, 200):
        jr = {50: 0.02, 100: 0.02, 200: 0.022}[s]
        out.append(ellipsoid("j-boss", "lamp", jr, (BASE_TOP - 0.01, 0.0, 0.0), "steel-black", "joint", "lamp"))
        out.append(ellipsoid("j-neck", "head", jr, (0.012, 0.0, 0.0), "chrome-steel", "joint", "head"))
    if s == 50:
        for k, u in enumerate(COUPLERS):
            out.append(ellipsoid(f"j-splice-{k + 1}", "lamp", 0.015, (u, 0.0, 0.0), "steel-black", "joint", "lamp"))
    mr = max(FEATURE[s] / 2, 0.02) if s > 50 else 0.02
    out.append(ellipsoid("mass-lamp", "lamp", mr, (coms["lamp"], 0.0, 0.0), "steel-black", "mass", "lamp"))
    out.append(ellipsoid("mass-head", "head", mr, (coms["head"], 0.0, 0.0), "chrome-steel", "mass", "head"))
    out.append(ellipsoid("mass-bowl", "bowl", mr, (coms["bowl"], 0.0, 0.0), "glass-yellowed", "mass", "bowl"))
    cr = {50: 0.004, 100: 0.0075, 200: 0.015, 500: 0.0375}[s]
    out.append(ellipsoid("mass-cardboard", "cardboard", cr, (CARD_S / 2, cr, 0.0), "cardboard", "mass", "cardboard"))
    return out


def masses():
    """⚖️ Masses and centres of every part from section x density, and the 1:50 surface areas of every thermal region."""
    m = {}
    ballast_r, ballast_t = 0.135, 0.006
    ballast = math.pi * ballast_r ** 2 * ballast_t * RHO_STEEL
    bp = base_profile(50)
    cover_area = revolve_area(bp) - math.pi * 0.145 ** 2
    cover = cover_area * 0.0008 * RHO_STEEL
    boss = 0.12
    stem_len = STEM_U[1] - STEM_U[0]
    stem = math.pi / 4 * ((2 * STEM_R) ** 2 - (2 * STEM_R - 2 * STEM_T) ** 2) * stem_len * RHO_STEEL
    couplers = 2 * 0.015
    cord_in_stem = 1.62 * 0.045
    dimmer = 0.10
    parts_lamp = [("ballast plate Ø270 x 6 mm", ballast, 0.004), ("pressed steel cover 0.8 mm", cover, 0.016), ("threaded boss", boss, 0.036), ("stem tube Ø25 x 1.0", stem, (STEM_U[0] + STEM_U[1]) / 2),
                  ("two splice couplers", couplers, sum(COUPLERS) / 2), ("cord inside the stem", cord_in_stem, 0.85), ("dimmer with triac", dimmer, DIMMER_SEAM)]
    m["lamp"] = parts_lamp
    refl_area = revolve_area(REFLECTOR[50])
    reflector = refl_area / 2 * 0.0008 * RHO_STEEL
    parts_head = [("spun steel reflector 0.8 mm", reflector, 0.075), ("nipple and lock nut", 0.03, 0.015), ("two steatite lamp holders", 2 * 0.012, HOLDER["u"]), ("R7s halogen tube", 0.008, TUBE_U), ("wire guard", 0.006, GUARD_U)]
    m["head"] = parts_head
    bowl_area = revolve_area(BOWL[50])
    bowl_vol = revolve_volume(BOWL[50])
    m["bowl"] = [("pressed glass bowl 4 mm", bowl_vol * RHO_GLASS, revolve_moment(BOWL[50]) / bowl_vol)]
    m["knob"] = [("replacement knob", 0.008, 0.012)]
    m["cable"] = [("cable H03VV-F 2x0.75, 0.5 m", 0.5 * 0.045, 0.25), ("grommet and tape", 0.006, 0.1)]
    m["riser"] = [("cable H03VV-F 2x0.75, 0.3 m", 0.3 * 0.045, 0.15)]
    m["plug"] = [("Euro contour plug", 0.035, 0.03)]
    m["cardboard"] = [("double-wall corrugated sheet 7 mm, 750 g/m2", CARD_S * CARD_S * 0.75, CARD_S / 2)]
    area = {}
    area["base"] = revolve_area(bp)
    stem_a = 2 * math.pi * STEM_R * stem_len + 2 * math.pi * STEM_R ** 2
    cpl_a = revolve_area(coupler_profile(0.0)) - 2 * math.pi * STEM_R ** 2
    area["stem"] = stem_a + 2 * (cpl_a - 2 * math.pi * STEM_R * 2 * COUPLER_HALF)
    knob_exposed = 2 * math.pi * 0.015 * (KNOB_OUT - 0.0065) + math.pi * 0.015 ** 2
    area["dimmer"] = revolve_area(DIMMER[50]) - math.pi * 0.015 ** 2 + knob_exposed
    hs = HOLDER["size"]
    holders = 2 * (2 * (hs[0] * hs[1] + hs[0] * hs[2] + hs[1] * hs[2]) - 2 * hs[0] * hs[1])
    guards = 2 * (2 * math.pi * GUARD_R * 2 * GUARD_HALF)
    area["reflector"] = refl_area + holders + guards
    area["lamp-tube"] = 2 * math.pi * TUBE_R * 2 * TUBE_HALF + 4 * math.pi * TUBE_R ** 2 - 2 * math.pi * TUBE_R ** 2
    area["bowl"] = bowl_area
    cable_a = 2 * math.pi * CABLE_R * (CABLE_L + 0.010) - 2 * math.pi * CABLE_R * 0.014 - 2 * math.pi * CABLE_R * (TAPE[1] - TAPE[0])
    riser_a = 2 * math.pi * CABLE_R * (RISER_L - 0.002)
    tape_a = 2 * math.pi * TAPE[2] * (TAPE[1] - TAPE[0]) + 2 * math.pi * (TAPE[2] ** 2 - CABLE_R ** 2)
    grom_a = revolve_area(GROMMET) - 2 * math.pi * CABLE_R ** 2
    p = PLUG_BODY
    pw, ph, pl = 2 * p["half"], p["bottom"] - p["top"], p["u1"] - p["u0"]
    plug_a = 2 * (pw * ph + pw * pl + ph * pl) + revolve_area(PLUG_COLLAR) - 2 * math.pi * 0.0055 ** 2
    area["cable"] = cable_a + riser_a + tape_a + grom_a + plug_a
    area["cardboard"] = 2 * CARD_S * CARD_S + 4 * CARD_S * CARD_T
    return m, area


def bone_mass(parts):
    """⚖️ Total mass and centre along the bone of a list of parts (name, kg, u)."""
    kg = sum(p[1] for p in parts)
    return kg, sum(p[1] * p[2] for p in parts) / kg


def region_binding(over=None, contact=None):
    """🌬️ The region binding of a pose: every region exposed, none sunlit, with optional exposure overrides and contacts."""
    regions = ["bowl", "lamp-tube", "reflector", "dimmer", "stem", "base", "cable", "cardboard"]
    out = {r: {"exposedFraction": (over or {}).get(r, 1.0), "sunlitFraction": 0.0} for r in regions}
    for r, c in (contact or {}).items():
        out[r]["contact"] = c
    return out


def main():
    """💡 Builds and writes the asset document."""
    parts, area = masses()
    bm = {b: bone_mass(p) for b, p in parts.items()}
    total_kg = sum(v[0] for v in bm.values())
    coms = {"lamp": bm["lamp"][1], "head": bm["head"][1], "bowl": bm["bowl"][1]}
    lamp_kg = sum(bm[b][0] for b in ("lamp", "head", "bowl", "knob"))
    com_h = (bm["lamp"][0] * bm["lamp"][1] + bm["head"][0] * (HEAD_U + bm["head"][1]) + bm["bowl"][0] * (HEAD_U + BOWL_U + bm["bowl"][1]) + bm["knob"][0] * KNOB_U) / lamp_kg
    W = lamp_kg * G
    edge = 0.145
    push_crit = W * edge / LAMP_H
    push = 2.0
    safety = push_crit / push
    tilt_crit = math.degrees(math.atan(edge / com_h))
    base_floor_share = round(math.pi * 0.145 ** 2 / area["base"], 3)
    card_floor_share = round(CARD_S ** 2 / area["cardboard"], 3)

    stored_th = {"operating": {"power": 0.0}, "regions": region_binding(contact={"base": {"fraction": base_floor_share, "tempC": 17.0}, "cardboard": {"fraction": card_floor_share, "tempC": 16.0}, "cable": {"fraction": 0.12, "tempC": 16.0}}), "emitters": {"lamp-plume": 0.0}}
    carried_th = {"operating": {"power": 0.0}, "regions": region_binding(contact={"stem": {"fraction": 0.03, "tempC": 32.0}, "cardboard": {"fraction": card_floor_share, "tempC": 16.0}, "cable": {"fraction": 0.05, "tempC": 16.0}}), "emitters": {"lamp-plume": 0.0}}

    def home(level):
        return {"operating": {"power": level}, "regions": region_binding(contact={"base": {"fraction": base_floor_share, "tempC": 20.0}, "cardboard": {"fraction": card_floor_share, "tempC": 20.0}, "cable": {"fraction": 0.08, "tempC": 20.0}}), "emitters": {"lamp-plume": level}}

    stored_bones = {}
    carried_bones = {"lift": {"dy": CARRY_LIFT}, "lamp": {"angleDeg": CARRY_TILT}, "cable": {"angleDeg": rnd(CARRY_CABLE - REST_CABLE, 3)}, "riser": {"angleDeg": rnd(CARRY_RISER - REST_RISER, 3)}}
    riser_home = -HOME_CABLE - REST_RISER

    def home_bones(knob):
        return {"lift": {"dy": -CARD_T}, "cardboard": {"dx": CARD_SHIFT}, "cable": {"angleDeg": rnd(HOME_CABLE - REST_CABLE, 3)}, "riser": {"angleDeg": rnd(riser_home, 3)}, "plug": {"angleDeg": 90.0}, "knob": {"twistDeg": knob}}

    under = ["card-under-fl", "card-under-fr", "card-under-rl", "card-under-rr"]
    pads = ["pad-fl", "pad-fr", "pad-rl", "pad-rr"]
    grips = ["grip-lower", "grip-upper"]
    push_load = {"id": "push", "kind": "imposed", "title": T(f"Horizontal push of {push:.1f} N forward at the rim of the bowl ({push / push_crit * 100:.0f} % of the {push_crit:.1f} N that tips the lamp)", f"Waagrechter Stoß von {d(push, 1)} N nach vorn am Rand der Schale ({d(push / push_crit * 100, 0)} % der {d(push_crit, 1)} N, die die Leuchte kippen)"), "anchor": "bowl-rim-front", "fxN": push, "fyN": 0.0, "source": "authored-estimate", "reference": "iec-60598-2-4"}
    poses = [
        {"id": "stored", "title": T("Stored on a cardboard sheet", "Gelagert auf einem Kartonbogen"), "description": T("In the second-hand department store: the emptied, wiped uplighter stands switched off on a flattened moving-box sheet, its cable with the taped repair laid out behind it on the floor, the Euro plug at its end.", "Im Gebrauchtwarenkaufhaus: der geleerte, abgewischte Deckenfluter steht ausgeschaltet auf einem flachgelegten Umzugskartonbogen, sein Kabel mit der geklebten Reparatur liegt dahinter auf dem Boden, am Ende der Eurostecker."), "bones": stored_bones, "ground": "authored", "contacts": under, "thermal": stored_th, "loads": []},
        {"id": "grip-key", "title": T("Gripped (key)", "Gegriffen (Schlüssel)"), "description": T("One hand round the stem below the dimmer, the other under the bowl at the top of the stem; the lamp still stands on its cardboard.", "Eine Hand um die Stange unter dem Dimmer, die andere unter der Schale oben an der Stange; die Leuchte steht noch auf ihrem Karton."), "key": True, "bones": stored_bones, "ground": "authored", "contacts": under + grips, "thermal": carried_th, "loads": []},
        {"id": "lifted-key", "title": T("Lifted off the cardboard (key)", "Vom Karton gehoben (Schlüssel)"), "description": T("Lifted straight up by 30 mm, upright, the cable still on the floor.", "Senkrecht um 30 mm angehoben, aufrecht, das Kabel noch am Boden."), "key": True, "bones": {"lift": {"dy": 0.03}, "cable": {"angleDeg": rnd(cable_angles(CARD_T + 0.03, 0.0)[0] - REST_CABLE, 3)}, "riser": {"angleDeg": rnd(cable_angles(CARD_T + 0.03, 0.0)[1] - REST_RISER, 3)}}, "ground": "authored", "contacts": grips, "thermal": carried_th, "loads": []},
        {"id": "carried", "title": T("Carried upright by one person", "Von einer Person aufrecht getragen"), "description": T(f"Carried by one person at the side: the lower hand round the stem at {GRIP_LOWER:.2f} m holds the weight, the upper hand at {GRIP_UPPER:.2f} m steadies the stem under the bowl; the lamp leans 6 degrees ahead with its base 50 mm above the cardboard, the cable hangs from the base and drags on the floor, the cardboard stays behind.", f"Von einer Person seitlich getragen: die untere Hand um die Stange bei {d(GRIP_LOWER, 2)} m hält das Gewicht, die obere bei {d(GRIP_UPPER, 2)} m stützt die Stange unter der Schale; die Leuchte neigt sich 6 Grad nach vorn, der Fuß 50 mm über dem Karton, das Kabel hängt vom Fuß herab und schleift am Boden, der Karton bleibt liegen."), "bones": carried_bones, "ground": "authored", "contacts": grips, "thermal": carried_th, "loads": []},
        {"id": "set-up", "title": T("Set up in a living room, off", "Im Wohnzimmer aufgestellt, aus"), "description": T("Back in a home: standing on its felt ring on the floor, the cardboard slid out from under it and lying in front, the cable run back to the wall and the plug in the socket at 0.30 m; the dimmer at its off stop.", "Wieder in einer Wohnung: auf ihrem Filzring auf dem Boden, der Karton unter ihr hervorgezogen und davor liegend, das Kabel zur Wand geführt und der Stecker in der Steckdose auf 0,30 m; der Dimmer am Aus-Anschlag."), "bones": home_bones(0.0), "ground": "authored", "contacts": pads, "thermal": home(0.0), "loads": []},
        {"id": "in-use", "title": T("Lit at full power", "Mit voller Leistung eingeschaltet"), "description": T("The dimmer turned to its end stop: the 300 W halogen tube glows inside the reflector, throws its light up to the ceiling through the open bowl and heats the reflector and the yellowed glass; a warm plume rises over the bowl.", "Der Dimmer bis zum Endanschlag gedreht: der 300-W-Halogenstab glüht im Reflektor, wirft sein Licht durch die offene Schale an die Decke und erwärmt Reflektor und vergilbtes Glas; über der Schale steigt eine warme Fahne auf."), "bones": home_bones(KNOB_FULL), "ground": "authored", "contacts": pads, "thermal": home(1.0), "loads": []},
        {"id": "dimmed", "title": T("Dimmed to 40 %", "Auf 40 % gedimmt"), "description": T("The dimmer turned back to 120 degrees: the phase-cut triac passes 40 % of the power, 120 W, the tube glows orange and the bowl is lukewarm.", "Der Dimmer auf 120 Grad zurückgedreht: der Phasenanschnitt-Triac lässt 40 % der Leistung durch, 120 W, der Stab glüht orange und die Schale ist handwarm."), "bones": home_bones(KNOB_DIM), "ground": "authored", "contacts": pads, "thermal": home(0.4), "loads": []},
        {"id": "stability-check", "title": T("Stability check, pushed at the bowl", "Standsicherheit, an der Schale angestoßen"), "description": T(f"Lit at full power and pushed forward at the rim of the bowl with {push:.1f} N: the lamp stays on its felt ring with a safety of {safety:.2f} against tipping over the front of its base.", f"Mit voller Leistung eingeschaltet und am Rand der Schale mit {d(push, 1)} N nach vorn gestoßen: die Leuchte bleibt auf ihrem Filzring mit einer Sicherheit von {d(safety, 2)} gegen Kippen über die Vorderkante ihres Fußes."), "bones": home_bones(KNOB_FULL), "ground": "authored", "contacts": pads, "thermal": home(1.0), "loads": [push_load]},
    ]
    clips = [
        {"id": "carry", "title": T("Lift off the cardboard and carry", "Vom Karton heben und tragen"), "description": T("Both hands take the stem, lift the lamp 30 mm straight off its cardboard, raise it to 50 mm while it leans 6 degrees ahead and carry it upright; the cable swings down from the base and drags on the floor, the cardboard stays where it was.", "Beide Hände fassen die Stange, heben die Leuchte 30 mm senkrecht von ihrem Karton, heben sie auf 50 mm, während sie sich 6 Grad nach vorn neigt, und tragen sie aufrecht; das Kabel schwingt vom Fuß herab und schleift am Boden, der Karton bleibt liegen."),
         "durationS": 4.0, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0.0, "pose": "stored", "ease": "linear"}, {"t": 1.0, "pose": "grip-key", "ease": "in"}, {"t": 2.0, "pose": "lifted-key", "ease": "out"}, {"t": 3.5, "pose": "carried", "ease": "linear"}, {"t": 4.0, "pose": "carried", "ease": "linear"}],
         "contacts": [{"anchor": a, "fromS": 0.0, "toS": 1.2} for a in under] + [{"anchor": a, "fromS": 0.8, "toS": 4.0} for a in grips],
         "phases": [{"id": "grip", "fromS": 0.0, "toS": 1.0, "title": T("Grip", "Greifen")}, {"id": "lift", "fromS": 1.0, "toS": 2.0, "title": T("Lift", "Anheben")}, {"id": "lean", "fromS": 2.0, "toS": 3.5, "title": T("Raise and lean", "Heben und neigen")}, {"id": "carry", "fromS": 3.5, "toS": 4.0, "title": T("Carry", "Tragen")}],
         "events": [{"t": 0.8, "kind": "contact-on", "title": T("Hands take the stem", "Hände fassen die Stange"), "target": "grip-lower"}, {"t": 1.2, "kind": "contact-off", "title": T("Base leaves the cardboard", "Fuß verlässt den Karton"), "target": "card-under-fl"}, {"t": 3.5, "kind": "motion", "title": T("Carrying position reached", "Trageposition erreicht")}],
         "tracks": []},
        {"id": "switch-and-dim", "title": T("Switch on, dim and switch off", "Einschalten, dimmen und ausschalten"), "description": T("In the living room the rotary dimmer is turned: it clicks on at 25 degrees, runs up to full power at its 270 degree stop within three seconds, is turned back to 40 % after ten seconds and finally off; the tube, the reflector and the bowl warm up and cool down with it.", "Im Wohnzimmer wird der Drehdimmer gedreht: er schaltet bei 25 Grad mit einem Klick ein, läuft binnen drei Sekunden bis zur vollen Leistung am 270-Grad-Anschlag, wird nach zehn Sekunden auf 40 % zurückgedreht und schließlich ausgeschaltet; Stab, Reflektor und Schale wärmen sich dabei auf und kühlen ab."),
         "durationS": 24.0, "fps": 10, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0.0, "pose": "set-up", "ease": "linear"}, {"t": 0.5, "pose": "set-up", "ease": "linear", "bones": {"knob": {"twistDeg": KNOB_CLICK}}}, {"t": 3.0, "pose": "in-use", "ease": "linear"}, {"t": 10.0, "pose": "in-use", "ease": "smooth"}, {"t": 13.0, "pose": "dimmed", "ease": "linear"}, {"t": 20.0, "pose": "dimmed", "ease": "linear"}, {"t": 21.5, "pose": "set-up", "ease": "linear", "bones": {"knob": {"twistDeg": KNOB_CLICK}}}, {"t": 22.0, "pose": "set-up", "ease": "linear"}, {"t": 24.0, "pose": "set-up", "ease": "linear"}],
         "contacts": [{"anchor": a, "fromS": 0.0, "toS": 24.0} for a in pads],
         "phases": [{"id": "off", "fromS": 0.0, "toS": 0.5, "title": T("Off", "Aus")}, {"id": "turn-up", "fromS": 0.5, "toS": 3.0, "title": T("Turned up", "Hochgedreht")}, {"id": "full", "fromS": 3.0, "toS": 10.0, "title": T("Full power", "Volle Leistung")}, {"id": "dim", "fromS": 10.0, "toS": 13.0, "title": T("Dimming down", "Herunterdimmen")}, {"id": "dimmed", "fromS": 13.0, "toS": 20.0, "title": T("Dimmed", "Gedimmt")}, {"id": "switch-off", "fromS": 20.0, "toS": 24.0, "title": T("Switched off", "Ausgeschaltet")}],
         "events": [{"t": 0.5, "kind": "operating", "title": T("Dimmer clicks on", "Dimmer schaltet mit Klick ein"), "target": "power"}, {"t": 3.0, "kind": "operating", "title": T("Full power at the end stop", "Volle Leistung am Endanschlag"), "target": "power"}, {"t": 13.0, "kind": "operating", "title": T("Dimmed to 40 %", "Auf 40 % gedimmt"), "target": "power"}, {"t": 21.5, "kind": "shutdown", "title": T("Dimmer clicks off", "Dimmer schaltet mit Klick aus"), "target": "power"}],
         "tracks": [{"target": "operating:power", "ease": "linear", "keys": [{"t": 0.0, "v": 0.0}, {"t": 0.5, "v": 0.0}, {"t": 0.51, "v": 0.06}, {"t": 3.0, "v": 1.0}, {"t": 10.0, "v": 1.0}, {"t": 13.0, "v": 0.4}, {"t": 20.0, "v": 0.4}, {"t": 21.5, "v": 0.06}, {"t": 21.51, "v": 0.0}, {"t": 24.0, "v": 0.0}]},
                    {"target": "emitter:lamp-plume", "ease": "linear", "keys": [{"t": 0.0, "v": 0.0}, {"t": 0.5, "v": 0.0}, {"t": 0.51, "v": 0.06}, {"t": 3.0, "v": 1.0}, {"t": 10.0, "v": 1.0}, {"t": 13.0, "v": 0.4}, {"t": 20.0, "v": 0.4}, {"t": 21.5, "v": 0.06}, {"t": 21.51, "v": 0.0}, {"t": 24.0, "v": 0.0}]}]},
    ]

    anchors = []
    for tag, (sx, sz), names in (("fl", (1, 1), ("front left", "vorn links")), ("fr", (1, -1), ("front right", "vorn rechts")), ("rl", (-1, 1), ("rear left", "hinten links")), ("rr", (-1, -1), ("rear right", "hinten rechts"))):
        c = PAD_R / math.sqrt(2)
        anchors.append({"id": f"card-under-{tag}", "bone": "cardboard", "x": rnd(CARD_S / 2 + sx * c), "y": 0.0, "z": rnd(sz * c), "kind": "contact", "title": T(f"Cardboard on the depot floor under the base, {names[0]}", f"Karton auf dem Lagerboden unter dem Fuß, {names[1]}"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.06, "footprintWidthM": 0.06, "frictionCoefficient": 0.5})
        anchors.append({"id": f"pad-{tag}", "bone": "lamp", "x": 0.0, "y": rnd(-sx * c), "z": rnd(sz * c), "kind": "contact", "title": T(f"Felt ring of the base on the floor, {names[0]}", f"Filzring des Fußes auf dem Boden, {names[1]}"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.05, "footprintWidthM": 0.012, "frictionCoefficient": 0.3})
    anchors.append({"id": "grip-lower", "bone": "hand-lower", "x": 0.0, "y": 0.0, "kind": "support", "title": T("Lower hand round the stem (holds the weight)", "Untere Hand um die Stange (hält das Gewicht)"), "support": "pin"})
    anchors.append({"id": "grip-upper", "bone": "hand-upper", "x": 0.0, "y": 0.0, "kind": "support", "title": T("Upper hand round the stem under the bowl (steadies)", "Obere Hand um die Stange unter der Schale (stützt)"), "support": "roller", "normalDeg": 0})
    anchors.append({"id": "lamp-mouth", "bone": "head", "x": 0.1425, "y": 0.0, "kind": "emitter", "title": T("Mouth of the reflector over the halogen tube", "Reflektormündung über dem Halogenstab")})
    anchors.append({"id": "bowl-rim-front", "bone": "bowl", "x": BOWL_U, "y": -BOWL_RIM, "kind": "load", "title": T("Front rim of the bowl", "Vorderer Rand der Schale")})

    vols = {}
    reps = {}
    for s in SCALES:
        b = body(s)
        vols[str(s)] = {"status": "authored", "denominator": s, "body": b, "thermal": thermal(s, b), "structure": structure(s, coms), "decisions": VOLUME_DECISIONS[s]}
        reps[str(s)] = {"status": "authored", "denominator": s, "validRange": {"minDenominator": {50: 20, 100: 75, 200: 150, 500: 350}[s], "maxDenominator": {50: 75, 100: 150, 200: 350, 500: 1000}[s]},
                        "pens": PENS[s], "hatches": [], "geometry": geometry(s), "decisions": REP_DECISIONS[s]}

    def layer(id, title, kind, material, t, k, rho, c, src="literature", ref="iso-10456-2007"):
        return {"id": id, "title": title, "kind": kind, "material": material, "thicknessM": rnd(t, 6), "resistanceM2KW": rnd(t / k, 8), "heatCapacityJM2K": round(rho * c * t), "source": src, "reference": ref}

    base_t = (parts["lamp"][0][1] + parts["lamp"][1][1] + parts["lamp"][2][1]) / RHO_STEEL / area["base"]
    regions = [
        {"id": "lamp-tube", "title": T("R7s halogen tube 118 mm, 300 W (quartz)", "R7s-Halogenstab 118 mm, 300 W (Quarz)"), "kind": "lamp", "model": "operating", "material": "quartz", "areaM2": rnd(area["lamp-tube"], 5), "characteristicLengthM": 0.01, "geometry": "cylinder", "skyViewFactor": 0.5,
         "layers": [layer("quartz-wall", T("Quartz glass wall 1 mm", "Quarzglaswand 1 mm"), "glass", "quartz", 0.001, 1.4, 2200, 750)], "heatCapacityJM2K": round(2200 * 750 * 0.001), "operating": {"key": "power", "heatW": 200.0}, "ratedPowerW": 300.0, "ratedPowerBasis": "electrical-input", "source": "authored-estimate", "reference": "iec-60357"},
        {"id": "reflector", "title": T("Spun steel reflector with lamp holders and wire guard (chrome dulled)", "Gedrückter Stahlreflektor mit Fassungen und Schutzgitter (Chrom stumpf)"), "kind": "lamp", "model": "operating", "material": "chrome-steel", "areaM2": rnd(area["reflector"], 4), "characteristicLengthM": 0.14, "geometry": "cylinder", "skyViewFactor": 0.5,
         "layers": [layer("reflector-steel", T("Steel sheet 0.8 mm", "Stahlblech 0,8 mm"), "metal", "chrome-steel", 0.0008, 50, RHO_STEEL, 460)], "heatCapacityJM2K": round(RHO_STEEL * 460 * 0.0008), "operating": {"key": "power", "heatW": 60.0}, "source": "authored-estimate", "reference": "incropera-2018"},
        {"id": "bowl", "title": T("Pressed glass bowl Ø360 mm, yellowed", "Pressglasschale Ø360 mm, vergilbt"), "kind": "lamp", "model": "operating", "material": "glass-yellowed", "areaM2": rnd(area["bowl"], 4), "characteristicLengthM": 0.36, "geometry": "horizontal-plate-up", "skyViewFactor": 0.5,
         "layers": [layer("bowl-glass", T("Soda-lime glass 4 mm", "Kalk-Natron-Glas 4 mm"), "glass", "glass-yellowed", BOWL_T, 1.0, RHO_GLASS, 840)], "heatCapacityJM2K": round(RHO_GLASS * 840 * BOWL_T), "operating": {"key": "power", "heatW": 25.0}, "source": "authored-estimate", "reference": "incropera-2018"},
        {"id": "dimmer", "title": T("Rotary dimmer on the stem with its replacement knob", "Drehdimmer an der Stange mit Ersatzknopf"), "kind": "surface", "model": "operating", "material": "abs-black", "areaM2": rnd(area["dimmer"], 5), "characteristicLengthM": 0.044, "geometry": "cylinder", "skyViewFactor": 0.5,
         "layers": [layer("abs-shell", T("ABS housing 2 mm", "ABS-Gehäuse 2 mm"), "polymer", "abs-black", 0.002, 0.17, 1050, 1400)], "heatCapacityJM2K": round(1050 * 1400 * 0.002), "operating": {"key": "power", "heatW": 3.0}, "source": "authored-estimate", "reference": "en-60669-2-1"},
        {"id": "stem", "title": T("Stem tube Ø25 mm in three screwed lengths (black lacquer chipped)", "Stangenrohr Ø25 mm in drei verschraubten Längen (schwarzer Lack abgeplatzt)"), "kind": "surface", "model": "passive", "material": "steel-black", "areaM2": rnd(area["stem"], 4), "characteristicLengthM": 0.025, "geometry": "cylinder", "skyViewFactor": 0.5,
         "layers": [layer("stem-steel", T("Steel tube wall 1.0 mm", "Stahlrohrwand 1,0 mm"), "metal", "steel-black", STEM_T, 50, RHO_STEEL, 460)], "heatCapacityJM2K": round(RHO_STEEL * 460 * STEM_T), "source": "authored-estimate", "reference": "incropera-2018"},
        {"id": "base", "title": T("Domed base plate Ø290 mm over its ballast plate (lacquer scratched)", "Gewölbter Fußteller Ø290 mm über seiner Ballastplatte (Lack verkratzt)"), "kind": "surface", "model": "passive", "material": "steel-black", "areaM2": rnd(area["base"], 4), "characteristicLengthM": 0.29, "geometry": "horizontal-plate-up", "skyViewFactor": 0.5,
         "layers": [layer("base-steel", T(f"Steel cover and ballast, {base_t * 1000:.1f} mm (volume over surface)", f"Stahlhaube und Ballast, {d(base_t * 1000, 1)} mm (Volumen durch Oberfläche)"), "metal", "steel-black", base_t, 50, RHO_STEEL, 460)], "heatCapacityJM2K": round(RHO_STEEL * 460 * base_t), "source": "authored-estimate", "reference": "incropera-2018"},
        {"id": "cable", "title": T("Cable H03VV-F 2 x 0.75 mm2 with taped repair and Euro plug", "Leitung H03VV-F 2 x 0,75 mm2 mit geklebter Reparatur und Eurostecker"), "kind": "surface", "model": "passive", "material": "pvc-cable", "areaM2": rnd(area["cable"], 5), "characteristicLengthM": 0.006, "geometry": "cylinder", "skyViewFactor": 0.5,
         "layers": [layer("cable-pvc", T("PVC sheath and copper, 1.5 mm (volume over surface)", "PVC-Mantel und Kupfer, 1,5 mm (Volumen durch Oberfläche)"), "polymer", "pvc-cable", 0.0015, 0.3, 2400, 700, "authored-estimate")], "heatCapacityJM2K": round(2400 * 700 * 0.0015), "source": "authored-estimate", "reference": "en-50525-2-11"},
        {"id": "cardboard", "title": T("Flattened moving-box sheet, double-wall corrugated", "Flachgelegter Umzugskartonbogen, Doppelwelle"), "kind": "surface", "model": "passive", "material": "cardboard", "areaM2": rnd(area["cardboard"], 4), "characteristicLengthM": CARD_S, "geometry": "horizontal-plate-up", "skyViewFactor": 0.5,
         "layers": [layer("board", T("Corrugated board 7 mm", "Wellpappe 7 mm"), "insulation", "cardboard", CARD_T, 0.065, 107, 1340, "authored-estimate")], "heatCapacityJM2K": round(107 * 1340 * CARD_T), "source": "authored-estimate", "reference": "iso-10456-2007"},
    ]
    thermal_model = {
        "environment": {"airTempC": 20.0, "meanRadiantTempC": 20.0, "relativeHumidity": 0.45, "windMS": 0.05, "windDirectionDeg": 0.0, "solarWM2": 0.0, "diffuseFraction": 1.0, "pressurePa": 101325.0},
        "passengers": [],
        "operating": [{"id": "power", "title": T("Electrical power through the dimmer", "Elektrische Leistung über den Dimmer"), "levels": [{"v": 0, "title": T("Off", "Aus")}, {"v": 0.4, "title": T("Dimmed (120 W)", "Gedimmt (120 W)")}, {"v": 1, "title": T("Full (300 W)", "Voll (300 W)")}]}],
        "regions": regions,
        "emitters": [{"id": "lamp-plume", "kind": "plume", "title": T("Hot air rising from the reflector out of the bowl", "Heißluft, die vom Reflektor aus der Schale aufsteigt"), "anchor": "lamp-mouth", "directionDeg": 0, "diameterM": 0.14, "region": "lamp-tube", "source": "authored-estimate", "reference": "incropera-2018"}],
        "assumptions": [
            T("The 300 W of the luminaire nameplate are stated once, on the halogen tube (electrical input, catalogue-typical for the uplighters of the 1990s with an R7s 118 mm tube). At full power 288 W leave the lamp as heat from its surfaces: the tube 200 W, the reflector with its holders 60 W (it takes the downward radiation and the hot air of the tube), the bowl 25 W (radiation and convection from inside), the triac dimmer 3 W (about 1 % loss); the visible light of about 12 W (4 %) leaves upwards and is absorbed by the ceiling, where it also ends as heat. Dimmed, all scale with the level.", "Die 300 W des Typenschilds der Leuchte stehen einmal, am Halogenstab (elektrische Aufnahme, katalogtypisch für die Deckenfluter der 1990er Jahre mit R7s-118-mm-Stab). Bei voller Leistung verlassen 288 W die Leuchte als Wärme über ihre Flächen: der Stab 200 W, der Reflektor mit Fassungen 60 W (er nimmt die nach unten gehende Strahlung und die Heißluft des Stabs auf), die Schale 25 W (Strahlung und Konvektion von innen), der Triac-Dimmer 3 W (etwa 1 % Verlust); das sichtbare Licht von etwa 12 W (4 %) geht nach oben und wird von der Decke absorbiert, wo es ebenfalls als Wärme endet. Gedimmt skaliert alles mit der Stufe."),
            T("Aged surfaces: the clear pressed glass has yellowed over thirty years of halogen UV and kitchen fumes, so its solar absorptance is 0.18 instead of about 0.08 for new glass; the dulled chrome of the reflector exterior reads 0.12 (polished 0.06); the chipped black lacquer of stem and base 0.90 emissivity (authored estimates after Incropera Table A.11).", "Gealterte Oberflächen: das klare Pressglas ist in dreißig Jahren Halogen-UV und Küchendunst vergilbt, seine solare Absorption daher 0,18 statt etwa 0,08 bei neuem Glas; das stumpfe Chrom der Reflektoraußenseite 0,12 (poliert 0,06); der abgeplatzte schwarze Lack von Stange und Fuß Emissionsgrad 0,90 (Schätzwerte nach Incropera Tabelle A.11)."),
            T("Indoors at 20 degC without sun (depot and living room). The base lies with its underside on the cardboard or the floor, the cardboard with its underside on the floor, the cable with a strip of its surface; the hands of the carrier warm 3 % of the stem to 32 degC.", "Innenraum bei 20 degC ohne Sonne (Lager und Wohnzimmer). Der Fuß liegt mit seiner Unterseite auf dem Karton oder dem Boden, der Karton mit seiner Unterseite auf dem Boden, das Kabel mit einem Streifen seiner Oberfläche; die Hände der tragenden Person wärmen 3 % der Stange auf 32 degC."),
        ],
    }
    tube_section = {"areaM2": rnd(math.pi / 4 * (0.025 ** 2 - 0.023 ** 2), 8), "secondMomentM4": rnd(math.pi / 64 * (0.025 ** 4 - 0.023 ** 4), 12), "extremeFibreM": STEM_R, "material": "steel-black", "title": T("Steel tube Ø25 x 1.0 mm", "Stahlrohr Ø25 x 1,0 mm")}
    mass = lambda b: {"kg": rnd(bm[b][0], 4), "comX": rnd(bm[b][1], 4), "comY": 0, "distribution": "point", "source": "authored-estimate", "reference": "semio-used-household"}
    rig = {"kind": "mechanical", "root": "ground", "bones": [
        {"id": "ground", "parent": None, "title": T("Floor marker under the stem axis", "Bodenmarke unter der Stangenachse"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"}},
        {"id": "cardboard", "parent": "ground", "title": T("Cardboard sheet (stays behind; slid out from under the lamp when it is set up)", "Kartonbogen (bleibt liegen; beim Aufstellen unter der Leuchte hervorgezogen)"), "x": -CARD_S / 2, "y": 0, "angleDeg": 0, "lengthM": CARD_S, "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0.0, "maxM": 0.8}, "mass": {**mass("cardboard"), "comY": CARD_T / 2}},
        {"id": "lift", "parent": "ground", "title": T("Lift of the lamp: from the cardboard down to the floor or up into the carrying height", "Hub der Leuchte: vom Karton hinab zum Boden oder hinauf in die Tragehöhe"), "x": 0, "y": CARD_T, "angleDeg": 90, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -CARD_T, "maxM": 0.1}},
        {"id": "lamp", "parent": "lift", "title": T("Lamp: base plate, stem and dimmer (leans about the centre of its base when carried)", "Leuchte: Fußteller, Stange und Dimmer (neigt sich beim Tragen um die Mitte ihres Fußes)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": HEAD_U, "joint": {"type": "revolute", "minDeg": -15, "maxDeg": 15}, "mass": mass("lamp"), "section": tube_section},
        {"id": "head", "parent": "lamp", "title": T("Reflector head with the halogen tube", "Reflektorkopf mit dem Halogenstab"), "x": HEAD_U, "y": 0, "angleDeg": 0, "lengthM": 0.14, "joint": {"type": "fixed"}, "mass": mass("head")},
        {"id": "bowl", "parent": "head", "title": T("Glass bowl resting on the reflector flange", "Glasschale auf dem Reflektorflansch"), "x": BOWL_U, "y": 0, "angleDeg": 0, "lengthM": 0.10, "joint": {"type": "fixed"}, "mass": mass("bowl")},
        {"id": "knob", "parent": "lamp", "title": T("Replacement knob of the rotary dimmer (turns 270 degrees)", "Ersatzknopf des Drehdimmers (dreht 270 Grad)"), "x": KNOB_U, "y": KNOB_V, "angleDeg": -90, "lengthM": KNOB_OUT, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 0, "twistMinDeg": 0, "twistMaxDeg": 270}, "mass": mass("knob")},
        {"id": "cable", "parent": "lamp", "title": T("Cable run from the base exit back along the floor", "Kabelstrecke vom Fußaustritt nach hinten über den Boden"), "x": CABLE_EXIT[0], "y": CABLE_EXIT[1], "angleDeg": rnd(REST_CABLE, 4), "lengthM": CABLE_L, "joint": {"type": "revolute", "minDeg": -10, "maxDeg": 25}, "mass": mass("cable")},
        {"id": "riser", "parent": "cable", "title": T("Cable end, flat on the floor or rising to the wall socket", "Kabelende, flach am Boden oder zur Wandsteckdose aufsteigend"), "x": CABLE_L, "y": 0, "angleDeg": rnd(REST_RISER, 4), "lengthM": RISER_L, "joint": {"type": "revolute", "minDeg": -95, "maxDeg": 15}, "mass": mass("riser")},
        {"id": "plug", "parent": "riser", "title": T("Euro contour plug", "Eurostecker"), "x": RISER_L, "y": 0, "angleDeg": 0, "lengthM": PLUG_L, "joint": {"type": "revolute", "minDeg": -5, "maxDeg": 95}, "mass": mass("plug")},
        {"id": "hand-lower", "parent": "lamp", "title": T("Lower hand of the carrier round the stem", "Untere Hand der tragenden Person um die Stange"), "x": GRIP_LOWER, "y": 0, "angleDeg": -90, "lengthM": 0.08, "joint": {"type": "fixed"}},
        {"id": "hand-upper", "parent": "lamp", "title": T("Upper hand of the carrier round the stem under the bowl", "Obere Hand der tragenden Person um die Stange unter der Schale"), "x": GRIP_UPPER, "y": 0, "angleDeg": -90, "lengthM": 0.08, "joint": {"type": "fixed"}},
    ]}
    structure_model = {"gravityMS2": G, "dynamics": "quasi-static", "supports": [], "assumptions": [
        T(f"Stored: the lamp ({lamp_kg:.2f} kg without its cable) stands with the felt ring of its base on the cardboard sheet ({bm['cardboard'][0]:.2f} kg), which carries it into the depot floor, modelled as four contacts under the base on the diagonals at {PAD_R:.2f} m from the axis. Set up and in use: the felt ring stands on the floor, modelled by four contacts at the same places (felt on parquet, friction 0.3).", f"Gelagert: die Leuchte ({d(lamp_kg, 2)} kg ohne Kabel) steht mit dem Filzring ihres Fußes auf dem Kartonbogen ({d(bm['cardboard'][0], 2)} kg), der sie in den Lagerboden trägt, modelliert als vier Kontakte unter dem Fuß auf den Diagonalen {d(PAD_R, 2)} m von der Achse. Aufgestellt und in Gebrauch: der Filzring steht auf dem Boden, modelliert durch vier Kontakte an denselben Stellen (Filz auf Parkett, Reibung 0,3)."),
        T(f"Carried: the lower hand is a pin round the stem at {GRIP_LOWER:.2f} m, the upper hand a horizontal roller at {GRIP_UPPER:.2f} m that keeps the 6 degree lean. The structural model is one rigid body: the cable that drags on the floor and, when carried and set up, the cardboard that lies apart are carried by the hands or the felt ring as well (0.27 kg of the reactions, stated plainly, not hidden).", f"Getragen: die untere Hand ist ein Gelenklager um die Stange bei {d(GRIP_LOWER, 2)} m, die obere ein waagrechtes Rollenlager bei {d(GRIP_UPPER, 2)} m, das die Neigung von 6 Grad hält. Das Tragwerksmodell ist ein starrer Körper: das am Boden schleifende Kabel und, getragen und aufgestellt, der abseits liegende Karton werden ebenfalls von den Händen oder dem Filzring getragen (0,27 kg der Reaktionen, offen angegeben)."),
        T(f"Members: the stem is a steel tube Ø25 x 1.0 mm (S235, A {tube_section['areaM2'] * 1e6:.1f} mm2, I {tube_section['secondMomentM4'] * 1e8:.3f} cm4) screwed together from three lengths with threaded couplers and into the boss of the base; the base plate is a pressed steel dome over a 6 mm steel ballast plate; the reflector carries the bowl on its flange. Masses are section x density: steel 7850 kg/m3, glass 2500 kg/m3.", f"Stäbe: die Stange ist ein Stahlrohr Ø25 x 1,0 mm (S235, A {d(tube_section['areaM2'] * 1e6, 1)} mm2, I {d(tube_section['secondMomentM4'] * 1e8, 3)} cm4), aus drei Längen mit Gewindemuffen verschraubt und in den Gewindezapfen des Fußes geschraubt; der Fußteller ist eine gedrückte Stahlhaube über einer 6-mm-Stahlballastplatte; der Reflektor trägt die Schale auf seinem Flansch. Massen aus Querschnitt mal Dichte: Stahl 7850 kg/m3, Glas 2500 kg/m3."),
        T(f"Stability after the principle of the stability test of IEC 60598-2-4 for portable luminaires (not a test result): the centre of mass of the lamp lies {com_h:.2f} m above the floor, so it tips only when tilted {tilt_crit:.1f} degrees; a horizontal push at the bowl rim ({LAMP_H:.2f} m) tips it at {push_crit:.1f} N (weight {W:.1f} N on the {edge:.3f} m lever to the front edge of the base). The pose applies {push:.1f} N, safety {safety:.2f}. Uplighters with heavy bowls are known to tip easily when bumped.", f"Standsicherheit nach dem Grundsatz der Standsicherheitsprüfung der IEC 60598-2-4 für ortsveränderliche Leuchten (kein Prüfergebnis): der Schwerpunkt der Leuchte liegt {d(com_h, 2)} m über dem Boden, sie kippt erst bei {d(tilt_crit, 1)} Grad Neigung; ein waagrechter Stoß am Schalenrand ({d(LAMP_H, 2)} m) kippt sie bei {d(push_crit, 1)} N (Gewicht {d(W, 1)} N am Hebel von {d(edge, 3)} m zur Vorderkante des Fußes). Die Pose setzt {d(push, 1)} N an, Sicherheit {d(safety, 2)}. Deckenfluter mit schweren Schalen kippen bekanntlich leicht, wenn man sie anstößt."),
    ], "clinical": False}
    materials = [
        {"id": "steel-black", "title": T("Steel, black stove lacquer, chipped and scratched", "Stahl, schwarzer Einbrennlack, abgeplatzt und verkratzt"), "emissivity": 0.9, "solarAbsorptivity": 0.9, "densityKgM3": RHO_STEEL, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210e9, "strengthPa": 235e6, "source": "literature", "reference": "incropera-2018"},
        {"id": "chrome-steel", "title": T("Chromed steel sheet, dulled", "Verchromtes Stahlblech, stumpf"), "emissivity": 0.12, "solarAbsorptivity": 0.4, "densityKgM3": RHO_STEEL, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210e9, "strengthPa": 235e6, "source": "authored-estimate", "reference": "incropera-2018"},
        {"id": "glass-yellowed", "title": T("Pressed soda-lime glass, frosted, yellowed", "Kalk-Natron-Pressglas, satiniert, vergilbt"), "emissivity": 0.92, "solarAbsorptivity": 0.18, "densityKgM3": RHO_GLASS, "conductivityWmK": 1.0, "specificHeatJKgK": 840, "source": "authored-estimate", "reference": "incropera-2018"},
        {"id": "quartz", "title": T("Fused quartz glass of the halogen tube", "Quarzglas des Halogenstabs"), "emissivity": 0.9, "solarAbsorptivity": 0.05, "densityKgM3": 2200, "conductivityWmK": 1.4, "specificHeatJKgK": 750, "source": "literature", "reference": "incropera-2018"},
        {"id": "steatite", "title": T("Steatite ceramic of the lamp holders", "Steatitkeramik der Fassungen"), "emissivity": 0.9, "solarAbsorptivity": 0.3, "densityKgM3": 2700, "conductivityWmK": 2.5, "specificHeatJKgK": 830, "source": "authored-estimate", "reference": "incropera-2018"},
        {"id": "abs-black", "title": T("Black ABS of the dimmer housing", "Schwarzes ABS des Dimmergehäuses"), "emissivity": 0.92, "solarAbsorptivity": 0.9, "densityKgM3": 1050, "conductivityWmK": 0.17, "specificHeatJKgK": 1400, "source": "literature", "reference": "iso-10456-2007"},
        {"id": "abs-grey", "title": T("Grey ABS of the replacement knob (from another device)", "Graues ABS des Ersatzknopfs (von einem anderen Gerät)"), "emissivity": 0.92, "solarAbsorptivity": 0.6, "densityKgM3": 1050, "conductivityWmK": 0.17, "specificHeatJKgK": 1400, "source": "literature", "reference": "iso-10456-2007"},
        {"id": "abs-white", "title": T("White paint of the pointer line", "Weiße Farbe des Zeigerstrichs"), "emissivity": 0.92, "solarAbsorptivity": 0.25, "densityKgM3": 1200, "conductivityWmK": 0.2, "specificHeatJKgK": 1400, "source": "authored-estimate", "reference": "iso-10456-2007"},
        {"id": "pvc-cable", "title": T("White PVC sheath, yellowed, over copper", "Weißer PVC-Mantel, vergilbt, über Kupfer"), "emissivity": 0.92, "solarAbsorptivity": 0.4, "densityKgM3": 2400, "conductivityWmK": 0.3, "specificHeatJKgK": 700, "source": "authored-estimate", "reference": "en-50525-2-11"},
        {"id": "pvc-black", "title": T("Black PVC of the strain-relief grommet", "Schwarzes PVC der Knickschutztülle"), "emissivity": 0.92, "solarAbsorptivity": 0.9, "densityKgM3": 1400, "conductivityWmK": 0.17, "specificHeatJKgK": 1000, "source": "literature", "reference": "iso-10456-2007"},
        {"id": "pvc-tape", "title": T("Black PVC insulating tape", "Schwarzes PVC-Isolierband"), "emissivity": 0.92, "solarAbsorptivity": 0.93, "densityKgM3": 1400, "conductivityWmK": 0.17, "specificHeatJKgK": 1000, "source": "literature", "reference": "iso-10456-2007"},
        {"id": "pp-white", "title": T("White polypropylene of the plug", "Weißes Polypropylen des Steckers"), "emissivity": 0.92, "solarAbsorptivity": 0.35, "densityKgM3": 1300, "conductivityWmK": 0.22, "specificHeatJKgK": 1700, "source": "literature", "reference": "iso-10456-2007"},
        {"id": "cardboard", "title": T("Double-wall corrugated board, 750 g/m2", "Doppelwellpappe, 750 g/m2"), "emissivity": 0.9, "solarAbsorptivity": 0.7, "densityKgM3": 107, "conductivityWmK": 0.065, "specificHeatJKgK": 1340, "source": "authored-estimate", "reference": "iso-10456-2007"},
    ]
    references = [
        {"id": "iec-60598-2-4", "citation": "IEC 60598-2-4:2017 Luminaires - Part 2-4: Particular requirements - Portable general purpose luminaires (stability of free-standing luminaires on an inclined plane; used here as the principle of the stability check, not as a test result)."},
        {"id": "iec-60357", "citation": "IEC 60357:2002+A1 Tungsten halogen lamps (non-vehicle) - Performance specifications (double-ended R7s lamps: overall length 117.6 mm, tube diameter about 10 mm, 300 W class of the household uplighters)."},
        {"id": "en-60669-2-1", "citation": "EN 60669-2-1:2004 Switches for household and similar fixed electrical installations - Part 2-1: Particular requirements - Electronic switches (phase-cut dimmers; triac losses about 1 % of the switched load)."},
        {"id": "en-50525-2-11", "citation": "EN 50525-2-11:2011 Electric cables - Low voltage energy cables of rated voltages up to and including 450/750 V - Part 2-11: Cables for general applications - Flexible cables with thermoplastic PVC insulation (H03VV-F 2 x 0.75 mm2, outer diameter about 6 mm, about 45 g/m)."},
        {"id": "din-18015-3", "citation": "DIN 18015-3:2016-09 Elektrische Anlagen in Wohngebäuden - Teil 3: Leitungsführung und Anordnung der Betriebsmittel (installation zones; socket outlets about 0.30 m above the finished floor)."},
        {"id": "en-10025-2", "citation": "EN 10025-2:2019 Hot rolled products of structural steels - Part 2: Technical delivery conditions for non-alloy structural steels (S235: yield strength 235 N/mm2; E 210 kN/mm2)."},
        {"id": "iso-10456-2007", "citation": "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values and procedures for determining declared and design thermal values (polymers, paper and board, glass)."},
        {"id": "incropera-2018", "citation": "Bergman, T. L., Lavine, A. S., Incropera, F. P., DeWitt, D. P. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (Table A.11 total emissivities of lacquer, chromium, glass and fused quartz; Table A.1 and A.3 properties of steel and glass)."},
        {"id": "semio-used-household", "citation": "Semio staffage content, used household objects wave 2026-11-22: dimensions, masses and power authored from catalogue-typical second-hand halogen uplighters of the 1990s (1.75-1.85 m, glass bowl Ø0.33-0.40 m, domed base Ø0.25-0.30 m, 5-8 kg, R7s 300 W with a rotary or foot dimmer), corrugated moving-box board and household cables; tagged authored-estimate, nothing is measured."},
    ]
    summary = T(f"A used halogen uplighter of the 1990s from a flat clearance, {LAMP_H:.2f} m tall: a domed black steel base plate Ø0.29 m over its ballast, a Ø25 mm stem screwed together from three lengths, a rotary dimmer at 0.99 m whose lost knob was replaced by a grey one from another device, a chromed reflector with a 300 W R7s halogen tube under a wire guard and a pressed glass bowl Ø0.36 m that has yellowed and holds a few dead insects; the lacquer is chipped at the base rim and scratched on the stem, and the cable carries a black insulating-tape repair 0.2 m from the base. It is stored on a cardboard sheet, carried upright at the stem, and lit, dimmed and checked for stability in a living room.",
                f"Ein gebrauchter Halogen-Deckenfluter der 1990er Jahre aus einer Haushaltsauflösung, {d(LAMP_H, 2)} m hoch: ein gewölbter schwarzer Stahlfußteller Ø0,29 m über seinem Ballast, eine Stange Ø25 mm aus drei verschraubten Längen, ein Drehdimmer auf 0,99 m, dessen verlorener Knopf durch einen grauen von einem anderen Gerät ersetzt ist, ein verchromter Reflektor mit einem 300-W-R7s-Halogenstab unter einem Schutzgitter und eine Pressglasschale Ø0,36 m, die vergilbt ist und ein paar tote Insekten enthält; der Lack ist am Fußrand abgeplatzt und an der Stange verkratzt, und das Kabel trägt 0,2 m vom Fuß eine Reparatur mit schwarzem Isolierband. Er lagert auf einem Kartonbogen, wird an der Stange aufrecht getragen und im Wohnzimmer eingeschaltet, gedimmt und auf Standsicherheit geprüft.")
    x_min = -(CABLE_EXIT[1] + 0.0) + (CABLE_L * math.cos(math.radians(90 + REST_CABLE)) + (RISER_L + PLUG_L) * math.cos(math.radians(90 + REST_CABLE + REST_RISER)))
    asset = {
        "id": "used-floor-lamp", "category": "appliances", "title": T("Used Floor Lamp", "Gebrauchte Stehleuchte"), "summary": summary,
        "tags": ["used", "household", "second-hand", "living", "floor-lamp", "uplighter", "halogen", "dimmer", "glass", "steel", "luminaire", "internal-gain", "electrical", "heat-source"],
        "provenance": {"author": "Semio staffage content (used household objects wave, author 28)", "method": "handcrafted", "created": "2026-11-22", "references": references,
                       "notes": T(f"Modelled to a second-hand German halogen uplighter of the 1990s: lamp {LAMP_H:.3f} m from the underside of the base to the bowl rim, bowl Ø{2 * BOWL_RIM:.2f} m, base Ø0.29 m, cable shown over its first {CABLE_L + RISER_L + PLUG_L:.2f} m to the plug. Lamp {lamp_kg:.2f} kg from section x density (base, stem and dimmer {bm['lamp'][0]:.2f} kg, head {bm['head'][0]:.2f} kg, bowl {bm['bowl'][0]:.2f} kg, knob {bm['knob'][0]:.3f} kg), cable and plug {bm['cable'][0] + bm['riser'][0] + bm['plug'][0]:.3f} kg, cardboard {bm['cardboard'][0]:.3f} kg.", f"Nach einem gebrauchten deutschen Halogen-Deckenfluter der 1990er Jahre modelliert: Leuchte {d(LAMP_H, 3)} m von der Fußunterseite bis zum Schalenrand, Schale Ø{d(2 * BOWL_RIM, 2)} m, Fuß Ø0,29 m, Kabel auf seinen ersten {d(CABLE_L + RISER_L + PLUG_L, 2)} m bis zum Stecker gezeigt. Leuchte {d(lamp_kg, 2)} kg aus Querschnitt mal Dichte (Fuß, Stange und Dimmer {d(bm['lamp'][0], 2)} kg, Kopf {d(bm['head'][0], 2)} kg, Schale {d(bm['bowl'][0], 2)} kg, Knopf {d(bm['knob'][0], 3)} kg), Kabel und Stecker {d(bm['cable'][0] + bm['riser'][0] + bm['plug'][0], 3)} kg, Karton {d(bm['cardboard'][0], 3)} kg.")},
        "dimensions": {"heightM": rnd(CARD_T + LAMP_H, 3), "widthM": rnd(CARD_S / 2 - x_min, 3), "depthM": CARD_S, "massKg": rnd(total_kg, 3)},
        "materials": materials, "rig": rig, "anchors": anchors, "representations": reps, "symbols": [], "poses": poses, "clips": clips,
        "thermal": thermal_model, "structure": structure_model, "volumes": vols,
        "presentation": {"defaultPose": "stored", "defaultClip": "switch-and-dim", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}
    with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf-8") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
    print(f"[DEBUG] total {total_kg:.3f} kg, lamp {lamp_kg:.3f} kg, com {com_h:.3f} m, x_min {x_min:.3f}")
    for b, p in parts.items():
        print(f"[DEBUG] bone {b}: {bm[b][0]:.4f} kg at u {bm[b][1]:.4f} :: " + ", ".join(f"{n} {k:.3f}" for n, k, _ in p))
    print(f"[DEBUG] areas {json.dumps({k: round(v, 5) for k, v in area.items()})}")
    print(f"[DEBUG] rest cable {REST_CABLE:.4f} riser {REST_RISER:.4f}; carry {CARRY_CABLE:.3f} {CARRY_RISER:.3f}; home {HOME_CABLE:.3f} riser {riser_home:.3f}")
    print(f"[DEBUG] stability: W {W:.1f} N push_crit {push_crit:.2f} N tilt_crit {tilt_crit:.1f} deg safety {safety:.2f}")


if __name__ == "__main__":
    main()
