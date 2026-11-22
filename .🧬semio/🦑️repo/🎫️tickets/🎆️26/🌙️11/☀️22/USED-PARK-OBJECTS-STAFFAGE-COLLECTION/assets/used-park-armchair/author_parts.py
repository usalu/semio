"""🧩 The parts of the used park armchair per authored scale (1:50, 1:100, 1:200, 1:500) as hand-chosen primitives, and their 3D solids and 2D outlines.

Each scale is its own model for its smallest feature (7.5 / 15 / 30 / 75 mm) with the same bones and stable ids; `author-armchair.py` turns them into the document.
@see author_geometry.py
"""
import math
from author_geometry import (BACK_DIR, BONES, CARRIER_BACK_TOP_H, CARRIER_BACK_U, CARRIER_H, CARRIER_SEAT_U, CARRIER_W, CARRIER_Z1, CORNER_FRONT, CORNER_REAR, FLANGE_R, FLANGE_T, FOOT_FRONT,
                             FOOT_H, FOOT_REAR, HEM_R, RAIL_R, SEAT_HALF, SHEET_T, STRETCH_R, STRETCH_Y, T_R, add, circle, dir_local, hull, loft_sections, mul, norm, offset_path,
                             side_path, sub, to_local, to_world, unit, view_of, zleg)

FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}
SIDES = (("l", 1), ("r", -1))
TONE = {"steel": 0.32, "rust": 0.12, "sheet": 0.42, "hem": 0.36, "grip": 0.7, "sticker": 0.8}


def leg_axis(foot, corner, side):
    """🦵 World 3D points of a leg centre line from its foot to height y."""
    def at(y):
        x = foot[0] + (corner[0] - foot[0]) * y / corner[1]
        return (x, y, side * zleg(y))
    return at


def frame_parts(scale):
    """🦿 The two bent side frames: rusted feet, straight legs, the armrest with both bends; per scale its own generalisation."""
    steps = {50: 8, 100: 5, 200: 3, 500: 2}[scale]
    parts = []
    for tag, side in SIDES:
        path, (b0, b1, f0, f1) = side_path(side, steps)
        rear_at = leg_axis(FOOT_REAR, CORNER_REAR, side)
        front_at = leg_axis(FOOT_FRONT, CORNER_FRONT, side)
        foot_top = FOOT_H if scale in (50, 100) else 0.0
        slender = False
        parts.append({"id": f"leg-rear-{tag}", "bone": "leg-rear", "type": "tube", "pts": [rear_at(foot_top), path[b0]], "r": T_R, "region": "frame-steel", "material": "painted-steel", "tone": TONE["steel"], "slender": slender, "side": side,
                      "regions2d": ["frame-steel", "feet-rust"] if not foot_top else None})
        parts.append({"id": f"armrest-{tag}", "bone": "armrest", "type": "tube", "pts": path[b0:f1 + 1], "r": T_R, "region": "frame-steel", "material": "painted-steel", "tone": TONE["steel"], "slender": slender, "side": side,
                      "front": [(path[b0][2], path[b0][1]), (side * zleg(0.6), 0.6)], "frontCaps": ("flat", "round")})
        parts.append({"id": f"leg-front-{tag}", "bone": "leg-front", "type": "tube", "pts": [path[f1], front_at(foot_top)], "r": T_R, "region": "frame-steel", "material": "painted-steel", "tone": TONE["steel"], "slender": slender, "side": side,
                      "regions2d": ["frame-steel", "feet-rust"] if not foot_top else None})
        if foot_top:
            for name, at, bone_id in (("rear", rear_at, "leg-rear"), ("front", front_at, "leg-front")):
                parts.append({"id": f"foot-{name}-{tag}", "bone": bone_id, "type": "foot", "p0": at(0.0), "p1": at(FOOT_H), "r": T_R, "flange": scale == 50, "region": "feet-rust", "material": "rusted-steel", "tone": TONE["rust"], "side": side})
    return parts


def seat_back_parts(scale):
    """🪑 Seat and back: perforated sheets with rolled hems on flat-bar carriers at 1:50 and 1:100, merged slabs at 1:200 and 1:500."""
    parts = []
    seat_len = BONES["seat"]["lengthM"]
    back_len = BONES["back"]["lengthM"]
    if scale in (50, 100):
        cw = CARRIER_W if scale == 50 else FEATURE[100]
        parts.append({"id": "seat-sheet", "bone": "seat", "type": "prism", "prof": [(0.0, -SHEET_T), (seat_len, -SHEET_T), (seat_len, 0.0), (0.0, 0.0)], "z0": -SEAT_HALF, "z1": SEAT_HALF, "sheet": True, "region": "seat-sheet", "material": "perforated-sheet", "tone": TONE["sheet"], "hatch": "plan"})
        parts.append({"id": "seat-hem", "bone": "seat", "type": "lat", "c": (seat_len, -HEM_R), "r": HEM_R, "z0": -SEAT_HALF, "z1": SEAT_HALF, "region": "seat-sheet", "material": "painted-steel", "tone": TONE["hem"]})
        parts.append({"id": "back-sheet", "bone": "back", "type": "prism", "prof": [(0.0, 0.0), (back_len, 0.0), (back_len, SHEET_T), (0.0, SHEET_T)], "z0": -SEAT_HALF, "z1": SEAT_HALF, "sheet": True, "region": "back-sheet", "material": "perforated-sheet", "tone": TONE["sheet"], "hatch": "front"})
        parts.append({"id": "back-hem", "bone": "back", "type": "lat", "c": (back_len, 0.007), "r": HEM_R, "z0": -SEAT_HALF, "z1": SEAT_HALF, "region": "back-sheet", "material": "painted-steel", "tone": TONE["hem"]})
        for tag, side in SIDES:
            z0, z1 = (CARRIER_Z1 - cw, CARRIER_Z1) if side > 0 else (-CARRIER_Z1, -CARRIER_Z1 + cw)
            u0, u1 = CARRIER_SEAT_U
            parts.append({"id": f"seat-carrier-{tag}", "bone": "seat", "type": "prism", "prof": [(u0, -SHEET_T - CARRIER_H), (u1, -SHEET_T - CARRIER_H), (u1, -SHEET_T), (u0, -SHEET_T)], "z0": z0, "z1": z1, "region": "frame-steel", "material": "painted-steel", "tone": TONE["steel"], "side": side})
            u0, u1 = CARRIER_BACK_U
            parts.append({"id": f"back-carrier-{tag}", "bone": "back", "type": "prism", "prof": [(u0, SHEET_T), (u1, SHEET_T), (u1, SHEET_T + CARRIER_BACK_TOP_H), (u0, SHEET_T + CARRIER_H)], "z0": z0, "z1": z1, "region": "frame-steel", "material": "painted-steel", "tone": TONE["steel"], "side": side})
    else:
        t = FEATURE[scale]
        half = 0.27
        parts.append({"id": "seat-slab", "bone": "seat", "type": "prism", "prof": [(-0.01, -t), (seat_len + HEM_R, -t), (seat_len + HEM_R, 0.0), (-0.01, 0.0)], "z0": -half, "z1": half, "region": "seat-sheet", "material": "perforated-sheet", "tone": TONE["sheet"]})
        parts.append({"id": "back-slab", "bone": "back", "type": "prism", "prof": [(-0.02 if scale == 200 else -0.03, 0.012 - t), (back_len + HEM_R - (t - 0.012) * 0.23, 0.012 - t), (back_len + HEM_R, 0.012), (-0.02 if scale == 200 else -0.03, 0.012)], "z0": -half, "z1": half, "region": "back-sheet", "material": "perforated-sheet", "tone": TONE["sheet"]})
    return parts


def rail_parts(scale):
    """➖ The welded cross rails: the Ø20 front seat rail between the front legs and the Ø16 rear stretcher between the rear legs."""
    parts = []
    if scale == 500:
        return parts
    fy = 0.373
    fx = FOOT_FRONT[0] + (CORNER_FRONT[0] - FOOT_FRONT[0]) * fy / CORNER_FRONT[1]
    parts.append({"id": "front-rail", "bone": "leg-front", "type": "lat", "c": to_local("leg-front", (fx, fy)), "r": RAIL_R, "z0": -zleg(fy), "z1": zleg(fy), "slender": scale == 200, "region": "frame-steel", "material": "painted-steel", "tone": TONE["steel"]})
    sx = FOOT_REAR[0] + (CORNER_REAR[0] - FOOT_REAR[0]) * STRETCH_Y / CORNER_REAR[1]
    parts.append({"id": "rear-stretcher", "bone": "leg-rear", "type": "lat", "c": to_local("leg-rear", (sx, STRETCH_Y)), "r": STRETCH_R, "z0": -zleg(STRETCH_Y), "z1": zleg(STRETCH_Y), "slender": scale == 200, "region": "frame-steel", "material": "painted-steel", "tone": TONE["steel"]})
    return parts


def wear_parts(scale):
    """🩹 Traces of use as texture solids at 1:50: hand-worn bare steel on both armrest fronts, rust bloom at the rear carrier welds, sticker residue on the back."""
    if scale != 50:
        return []
    parts = []
    for tag, side in SIDES:
        parts.append({"id": f"grip-wear-{tag}", "bone": "armrest", "type": "ell", "c": (0.40, 0.0, side * 0.275), "radii": (0.05, T_R + 0.0006, T_R + 0.0006), "material": "bare-steel", "tone": TONE["grip"], "texture": True, "role": "detail", "side": side})
        weld = to_local("seat", (0.0608, 0.35))
        parts.append({"id": f"weld-rust-{tag}", "bone": "seat", "type": "ell", "c": (CARRIER_SEAT_U[0] + 0.006, -SHEET_T - CARRIER_H / 2, side * (CARRIER_Z1 - 0.004)), "radii": (0.010, 0.017, 0.0046), "material": "rusted-steel", "tone": TONE["rust"], "texture": True, "role": "detail", "side": side})
    parts.append({"id": "sticker-residue", "bone": "back", "type": "box", "c": (0.27, -0.0003, -0.12), "size": (0.055, 0.0008, 0.075), "material": "sticker-paper", "tone": TONE["sticker"], "texture": True, "role": "detail"})
    return parts


def parts_of(scale):
    """📦 Every part of one scale, in painting order from far to near."""
    return rail_parts(scale) + frame_parts(scale) + seat_back_parts(scale) + wear_parts(scale)


def r6(v):
    """🔢 Rounded for the document."""
    return round(v, 6)


def solid(part):
    """🧊 The bone-local 3D solid of a part."""
    b = part["bone"]
    base = {"id": part["id"], "bone": b, "material": part["material"], "role": part.get("role", "body"), "tone": part["tone"]}
    for flag in ("slender", "sheet", "texture"):
        if part.get(flag):
            base[flag] = True
    t = part["type"]
    if t == "tube":
        loc = [to_local(b, p) for p in part["pts"]]
        if len(loc) == 2:
            axis = unit(sub(loc[1], loc[0]))
            length = norm(sub(loc[1], loc[0]))
            return {**base, "kind": "revolve", "profile": [[0, 0], [0, part["r"]], [r6(length), part["r"]], [r6(length), 0]], "axis": [r6(c) for c in axis], "origin": [r6(c) for c in loc[0]]}
        return {**base, "kind": "loft", "sections": loft_sections(loc, part["r"]), "capStart": True, "capEnd": True}
    if t == "foot":
        p0, p1 = to_local(b, part["p0"]), to_local(b, part["p1"])
        axis = unit(sub(p1, p0))
        length = norm(sub(p1, p0))
        r = part["r"]
        prof = [[0, 0], [0, FLANGE_R], [FLANGE_T, FLANGE_R], [FLANGE_T, r], [r6(length), r], [r6(length), 0]] if part["flange"] else [[0, 0], [0, r], [r6(length), r], [r6(length), 0]]
        return {**base, "kind": "revolve", "profile": prof, "axis": [r6(c) for c in axis], "origin": [r6(c) for c in p0]}
    if t == "prism":
        return {**base, "kind": "extrude", "profile": [[r6(u), r6(v)] for u, v in part["prof"]], "plane": "xy", "from": r6(part["z0"]), "to": r6(part["z1"])}
    if t == "lat":
        c = part["c"]
        return {**base, "kind": "revolve", "profile": [[0, 0], [0, part["r"]], [r6(part["z1"] - part["z0"]), part["r"]], [r6(part["z1"] - part["z0"]), 0]], "axis": [0, 0, 1], "origin": [r6(c[0]), r6(c[1]), r6(part["z0"])]}
    if t == "ell":
        return {**base, "kind": "ellipsoid", "radii": [r6(c) for c in part["radii"]], "transform": {"translation": [r6(c) for c in part["c"]]}}
    if t == "box":
        return {**base, "kind": "box", "size": [r6(c) for c in part["size"]], "transform": {"translation": [r6(c) for c in part["c"]]}}
    raise ValueError(t)


def corners(part):
    """📍 World 3D vertices of a prism or box part."""
    b = part["bone"]
    if part["type"] == "prism":
        return [to_world(b, (u, v, z)) for u, v in part["prof"] for z in (part["z0"], part["z1"])]
    c, s = part["c"], part["size"]
    return [to_world(b, (c[0] + du * s[0] / 2, c[1] + dv * s[1] / 2, c[2] + dw * s[2] / 2)) for du in (-1, 1) for dv in (-1, 1) for dw in (-1, 1)]


def outline(part, view):
    """✏️ The outline of a part in one view as (kind, data): `poly` rings in view coordinates (elevation bone-local, front and plan world rest) or an `ellipse`."""
    b = part["bone"]
    t = part["type"]
    if t == "tube":
        if view == "front" and "front" in part:
            return "poly", offset_path(part["front"], part["r"], *part["frontCaps"])
        pts = [view_of(p, view) for p in part["pts"]]
        if view == "elevation":
            pts = [to_local(b, p) for p in pts]
            return "poly", offset_path(pts, part["r"])
        caps = []
        for i, j in ((0, 1), (-1, -2)):
            d = unit(sub(part["pts"][i], part["pts"][j]))
            caps.append("round" if (view == "plan" and abs(d[1]) > 0.7) else "flat")
        return "poly", offset_path(pts, part["r"], caps[0], caps[1])
    if t == "foot":
        p0, p1 = part["p0"], part["p1"]
        if view == "plan":
            return "ellipse", ((p0[0], p0[2]), (FLANGE_R if part["flange"] else part["r"],) * 2)
        a = view_of(p0, view) if view == "front" else to_local(b, (p0[0], p0[1]))
        c = view_of(p1, view) if view == "front" else to_local(b, (p1[0], p1[1]))
        d = unit(sub(c, a))
        n = (-d[1], d[0])
        r = part["r"]
        rings = []
        if part["flange"]:
            f = add(a, mul(d, FLANGE_T))
            rings = [add(a, mul(n, FLANGE_R)), add(f, mul(n, FLANGE_R)), add(f, mul(n, r)), add(c, mul(n, r)), sub(c, mul(n, r)), sub(f, mul(n, r)), sub(f, mul(n, FLANGE_R)), sub(a, mul(n, FLANGE_R))]
        else:
            rings = [add(a, mul(n, r)), add(c, mul(n, r)), sub(c, mul(n, r)), sub(a, mul(n, r))]
        return "poly", rings
    if t in ("prism", "box"):
        if view == "elevation":
            if t == "prism":
                return "poly", list(part["prof"])
            c, s = part["c"], part["size"]
            return "poly", [(c[0] - s[0] / 2, c[1] - s[1] / 2), (c[0] + s[0] / 2, c[1] - s[1] / 2), (c[0] + s[0] / 2, c[1] + s[1] / 2), (c[0] - s[0] / 2, c[1] + s[1] / 2)]
        return "poly", hull([view_of(p, view) for p in corners(part)])
    if t == "lat":
        c = part["c"]
        if view == "elevation":
            return "ellipse", ((c[0], c[1]), (part["r"], part["r"]))
        w = to_world(b, c)
        if view == "front":
            return "poly", [(part["z0"], w[1] - part["r"]), (part["z1"], w[1] - part["r"]), (part["z1"], w[1] + part["r"]), (part["z0"], w[1] + part["r"])]
        return "poly", [(w[0] - part["r"], part["z0"]), (w[0] + part["r"], part["z0"]), (w[0] + part["r"], part["z1"]), (w[0] - part["r"], part["z1"])]
    if t == "ell":
        c, rr = part["c"], part["radii"]
        if view == "elevation":
            return "ellipse", ((c[0], c[1]), (rr[0], rr[1]))
        w = to_world(b, c)
        ang = math.radians(BONES[b]["angleDeg"])
        ry_world = abs(rr[0] * math.sin(ang)) + abs(rr[1] * math.cos(ang))
        rx_world = abs(rr[0] * math.cos(ang)) + abs(rr[1] * math.sin(ang))
        if view == "front":
            return "ellipse", ((w[2], w[1]), (rr[2], ry_world))
        return "ellipse", ((w[0], w[2]), (rx_world, rr[2]))
    raise ValueError(t)
