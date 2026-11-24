"""🪚 The parts of the used shoe cabinet per authored scale: every body solid with its 2D twins in elevation, front and plan, and the thermal and structural lists.

A part is extruded in the bone plane (profile u, v; swept along w), so its elevation is the profile, its front and plan are the same profile as an
extruded `solid` (README "Solid placements"), and the 3D solid is the same profile extruded: 2D and 3D agree by construction. Parts of another kind
(the yz frame of the coarse carcass, the zx top with the torn-out notch) state their own 2D shapes."""
from author_kit import *

ELEV, FRONT, PLAN = "elevation", "front", "plan"


def part(pid, bone, material, profile, w0, w1, tone, z, views=(ELEV, FRONT, PLAN), region=None, regions=None, role="body", sheet=False, pen=None, front_z=None):
    """🧩 One extruded part: profile in the bone plane, lateral span w0..w1 in rest asset coordinates, drawn in `views`."""
    return {"id": pid, "bone": bone, "material": material, "profile": profile, "w0": r(w0), "w1": r(w1), "tone": tone, "z": z, "views": views, "region": region, "regions": regions, "role": role, "sheet": sheet, "pen": pen, "front_z": front_z if front_z is not None else z}


def solid_of(p):
    """🧊 The body solid of a part."""
    s = {"id": p["id"], "kind": "extrude", "bone": p["bone"], "material": p["material"], "role": p["role"], "tone": p["tone"], "profile": p["profile"], "plane": "xy", "from": p["w0"], "to": p["w1"]}
    if p["sheet"]:
        s["sheet"] = True
    return s


def tagged(shape, p):
    """🏷️ Region, material and pen of a part on one of its shapes."""
    if p.get("regions"):
        shape["regions"] = p["regions"]
    elif p.get("region"):
        shape["region"] = p["region"]
    shape["material"] = p["material"]
    if p.get("pen") is not None:
        shape["penMm"] = p["pen"]
    return shape


def shapes_of(p):
    """✏️ The 2D twins of a part: the profile in elevation, the profile as a lateral extrusion in front and plan."""
    out = {ELEV: [], FRONT: [], PLAN: []}
    for view in p["views"]:
        s = {"id": p["id"], "bone": p["bone"], "role": p["role"], "z": p["z"] if view == ELEV else p["front_z"], "filled": True, "tone": p["tone"], "kind": "poly", "points": p["profile"], "closed": True}
        if view != ELEV:
            s["solid"] = {"kind": "extruded", "sweep": "w", "z0": p["w0"], "z1": p["w1"]}
        out[view].append(tagged(s, p))
    return out


def stroke(sid, bone, points, role="texture", z=20, tone=0.3, closed=False, filled=False, pen=None):
    """〰️ A plain stroke or small mark in the rest frame of its view (no solid): scuffs, old screw holes, the torn crater."""
    s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": "poly", "points": [[r(x), r(y)] for x, y in points], "closed": closed}
    if pen is not None:
        s["penMm"] = pen
    return s


def circle(sid, bone, cx, cy, rad, role="texture", z=21, tone=0.2, n=10):
    """⚫ A small closed ring (an open screw hole) as a polygon outline."""
    pts = [[cx + rad * math.cos(2 * math.pi * k / n), cy + rad * math.sin(2 * math.pi * k / n)] for k in range(n)]
    return stroke(sid, bone, pts, role=role, z=z, tone=tone, closed=True, filled=False)


def flap_bone(i):
    """🚪 The bone id of flap i (0 = bottom)."""
    return f"flap-{i + 1}"


def handle_parts(i, sizes, material, scale):
    """🔩 The bar handle of flap i: a 10 x 10 mm bar on two legs (1:50) or one block (1:100)."""
    b = flap_bone(i)
    tone = 0.4 if material == "aluminium-handle" else 0.3
    if scale == 50:
        out = [part(f"handle-{i + 1}-bar", b, material, rect(HANDLE_U[0], HANDLE_U[1], HANDLE_BAR_V[0], HANDLE_BAR_V[1]), -sizes["bar"], sizes["bar"], tone, 8, front_z=8)]
        for side, sign in (("r", -1), ("l", 1)):
            c = sign * sizes["leg"]
            out.append(part(f"handle-{i + 1}-leg-{side}", b, material, rect(HANDLE_U[0], HANDLE_U[1], HANDLE_LEG_V[0], HANDLE_LEG_V[1]), min(c - LEG_HALF, c + LEG_HALF), max(c - LEG_HALF, c + LEG_HALF), tone, 8, front_z=7))
        return out
    return [part(f"handle-{i + 1}", b, material, rect(HANDLE_U[0], HANDLE_U[1], HANDLE_BAR_V[0], 0.0), -sizes["bar"], sizes["bar"], tone, 8, front_z=8)]


def handle_sizes(i):
    """🔁 Flap 1 carries the longer replacement handle, flaps 2 and 3 the original ones."""
    return (HANDLE_REPLACED, "brushed-steel") if i == 0 else (HANDLE_ORIGINAL, "aluminium-handle")


def flap_front(i, top_extra=0.0, sheet=False, pen=None):
    """🪟 The 16 mm front panel of flap i in its bone plane (u up from the pivot, v backwards from the front face)."""
    return part(f"flap-{i + 1}-front", flap_bone(i), "chipboard-foil", rect(0.0, FLAP_H + top_extra, 0.0, T), -FLAP_HW, FLAP_HW, 0.88, 6, region="flaps", sheet=sheet, pen=pen, front_z=6)


def blanket_part(scale):
    """🧺 The folded moving blanket the cabinet is stored on (5 layers of a 1.8 x 1.5 m blanket, 0.60 x 0.90 x 0.015 m)."""
    profile = blanket_profile() if scale == 50 else rect(0.0, BLANKET_L, 0.0, BLANKET_T)
    pen = 0.2 if scale == 500 else None
    return part("blanket", "blanket", "polyester-nonwoven", profile, -BLANKET_HW, BLANKET_HW, 0.35, 1, region="blanket", sheet=scale >= 200, pen=pen, front_z=1)


def back_part(scale):
    """🟫 The 3 mm HDF back panel nailed on the rear edges, raw brown side outwards."""
    return part("back", "carcass", "hdf-back", rect(0.0, TB, BOTTOM[0], H), -HW, HW, 0.55, 3, views=(ELEV, FRONT, PLAN), region="back-panel", sheet=True, front_z=0)


def frame_yz(pid, holes, notch_top, top_v, u1=CARCASS_FRONT, open_top=None):
    """🗄️ A coarse carcass: one extrusion along u of its front-view section (sides, bottom, dividers, optionally the top band) with the compartments as holes, the open plinth as a notch and, where the top is a board of its own, the top compartment open upwards (`open_top` = the height of its floor)."""
    bottom = [[r(0.0), r(-HW)], [r(0.0), r(-INNER)], [r(notch_top), r(-INNER)], [r(notch_top), r(INNER)], [r(0.0), r(INNER)], [r(0.0), r(HW)]] if notch_top > 0 else [[0.0, r(-HW)], [0.0, r(HW)]]
    upper = [[r(top_v), r(HW)], [r(top_v), r(INNER)], [r(open_top), r(INNER)], [r(open_top), r(-INNER)], [r(top_v), r(-INNER)], [r(top_v), r(-HW)]] if open_top is not None else [[r(top_v), r(HW)], [r(top_v), r(-HW)]]
    profile = bottom + upper
    return {"id": pid, "kind": "extrude", "bone": "carcass", "material": "chipboard-foil", "role": "body", "tone": 0.85, "profile": profile, "holes": [[[r(a), r(-INNER)], [r(a), r(INNER)], [r(b), r(INNER)], [r(b), r(-INNER)]] for a, b in holes], "plane": "yz", "from": TB, "to": r(u1)}


def frame_shapes(pid, top_v, regions, z=4, u1=CARCASS_FRONT, pen=None):
    """🖼️ The 2D of a coarse carcass frame: its side outline in elevation, the same outline swept across the width in front and plan."""
    p = part(pid, "carcass", "chipboard-foil", rect(TB, u1, 0.0, top_v), -HW, HW, 0.85, z, regions=regions, pen=pen, front_z=2)
    return shapes_of(p)


def merge(into, more):
    """➕ Appends the shapes of `more` view by view."""
    for k in into:
        into[k].extend(more[k])
    return into


def scale_50():
    """🔍 1:50 (feature 7.5 mm): every board, the flap trays, the handles with their legs, the remaining wall bracket and the torn-out notch."""
    parts = [blanket_part(50), back_part(50)]
    for side, (a, b) in (("r", (-HW, -INNER)), ("l", (INNER, HW))):
        parts.append(part(f"side-{side}", "carcass", "chipboard-foil", rect(TB, CARCASS_FRONT, 0.0, TOP[0]), a, b, 0.85, 4, views=(ELEV, FRONT) if side == "r" else (FRONT,), region="carcass", front_z=2))
    parts.append(part("bottom", "carcass", "chipboard-foil", rect(TB, CARCASS_FRONT, BOTTOM[0], BOTTOM[1]), -INNER, INNER, 0.8, 2, views=(), region="carcass"))
    for k, (a, b) in enumerate(DIVIDERS):
        parts.append(part(f"divider-{k + 1}", "carcass", "chipboard-foil", rect(TB, CARCASS_FRONT, a, b), -INNER, INNER, 0.8, 2, views=(), region="carcass"))
    parts.append(part("kick", "carcass", "chipboard-foil", rect(CARCASS_FRONT - T, CARCASS_FRONT, 0.0, PLINTH_H), -INNER, INNER, 0.6, 2, views=(ELEV, FRONT), region="plinth", front_z=1))
    bracket = [[0.0, TOP[1]], [BRACKET_L, TOP[1]], [BRACKET_L, TOP[1] + BRACKET_S], [BRACKET_S, TOP[1] + BRACKET_S], [BRACKET_S, TOP[1] + BRACKET_H], [0.0, TOP[1] + BRACKET_H]]
    parts.append(part("bracket", "carcass", "zinc-plated-steel", [[r(a), r(b)] for a, b in bracket], BRACKET_W[0], BRACKET_W[1], 0.3, 9, front_z=9))
    for i in range(3):
        parts.append(flap_front(i))
        for side, (a, b) in (("r", (-CHEEK_W[1], -CHEEK_W[0])), ("l", CHEEK_W)):
            parts.append(part(f"flap-{i + 1}-cheek-{side}", flap_bone(i), "chipboard-foil", CHEEK, a, b, 0.7, 2, views=(ELEV,) if side == "r" else (), role="detail"))
        parts.append(part(f"flap-{i + 1}-tray", flap_bone(i), "chipboard-foil", tray_profile(), -CHEEK_W[0], CHEEK_W[0], 0.65, 2, views=(ELEV,), role="detail"))
        sizes, mat = handle_sizes(i)
        parts.extend(handle_parts(i, sizes, mat, 50))
    body = [solid_of(p) for p in parts]
    crater = [[r(TOP[0]), r(-HW)], [r(TOP[0]), r(HW)], [r(TOP[1]), r(HW)], [r(TOP[1]), r(NOTCH_W[1])], [r(TOP[1] - CRATER_DEPTH), r(NOTCH_W[1])], [r(TOP[1] - CRATER_DEPTH), r(NOTCH_W[0])], [r(TOP[1]), r(NOTCH_W[0])], [r(TOP[1]), r(-HW)]]
    body.append({"id": "top-rear", "kind": "extrude", "bone": "carcass", "material": "chipboard-foil", "role": "body", "tone": 0.85, "profile": crater, "plane": "yz", "from": TB, "to": NOTCH_U[1]})
    body.append({"id": "top", "kind": "extrude", "bone": "carcass", "material": "chipboard-foil", "role": "body", "tone": 0.85, "profile": rect(NOTCH_U[1], D, TOP[0], TOP[1]), "plane": "xy", "from": -HW, "to": HW})
    shapes = {ELEV: [], FRONT: [], PLAN: []}
    for p in parts:
        merge(shapes, shapes_of(p))
    top2d = part("top", "carcass", "chipboard-foil", rect(TB, D, TOP[0], TOP[1]), -HW, HW, 0.85, 5, region="carcass", front_z=3)
    merge(shapes, shapes_of(top2d))
    y0 = BLANKET_T
    shapes[PLAN].append(stroke("torn-notch", "carcass", [(NOTCH_U[0], NOTCH_W[0]), (NOTCH_U[1], NOTCH_W[0]), (NOTCH_U[1], NOTCH_W[1]), (NOTCH_U[0], NOTCH_W[1])], role="detail", z=21, tone=0.5, closed=True))
    shapes[PLAN].append(stroke("torn-crater", "carcass", [(0.003, 0.279), (0.019, 0.283), (0.022, 0.300), (0.019, 0.318), (0.003, 0.321)], z=22, tone=0.25))
    shapes[PLAN].append(circle("torn-screw-hole-1", "carcass", 0.028, 0.293, 0.0025))
    shapes[PLAN].append(circle("torn-screw-hole-2", "carcass", 0.028, 0.307, 0.0025))
    f1 = FLAP_BOTTOMS[0] + y0
    shapes[FRONT].append(circle("old-hole-r", "flap-1", -HANDLE_ORIGINAL["leg"], f1 + 0.415, 0.002))
    shapes[FRONT].append(circle("old-hole-l", "flap-1", HANDLE_ORIGINAL["leg"], f1 + 0.415, 0.002))
    for k, (za, zb, ya, yb) in enumerate([(-0.31, -0.22, 0.05, 0.07), (-0.12, 0.02, 0.03, 0.04), (0.10, 0.21, 0.06, 0.045), (0.24, 0.33, 0.02, 0.035), (-0.05, 0.06, 0.12, 0.11)]):
        shapes[FRONT].append(stroke(f"scuff-{k + 1}", "flap-1", [(za, f1 + ya), (zb, f1 + yb)], z=22, tone=0.35))
    shapes[FRONT].append(stroke("chipped-corner", "flap-1", [(-0.398, f1 + 0.018), (-0.386, f1 + 0.009), (-0.380, f1 + 0.0)], z=22, tone=0.3))
    thermal = [{"id": "th-blanket", "of": "blanket", "region": "blanket"}, {"id": "th-back", "of": "back", "region": "back-panel"}]
    thermal += [{"id": f"th-{x}", "of": x, "region": "carcass"} for x in ("side-r", "side-l", "top-rear", "top", "bottom", "divider-1", "divider-2")]
    thermal += [{"id": "th-kick", "of": "kick", "region": "plinth"}]
    thermal += [{"id": f"th-flap-{i + 1}", "of": f"flap-{i + 1}-front", "region": "flaps"} for i in range(3)]
    structure = [{"id": f"m-{x}", "of": x, "member": "carcass", "part": "member"} for x in ("side-r", "side-l", "top-rear", "top", "bottom", "divider-1", "divider-2", "back")]
    structure.append({"id": "j-bracket", "of": "bracket", "member": "carcass", "part": "joint"})
    for i in range(3):
        b = flap_bone(i)
        structure += [{"id": f"m-flap-{i + 1}-front", "of": f"flap-{i + 1}-front", "member": b, "part": "member"}, {"id": f"m-flap-{i + 1}-cheek-r", "of": f"flap-{i + 1}-cheek-r", "member": b, "part": "member"}, {"id": f"m-flap-{i + 1}-cheek-l", "of": f"flap-{i + 1}-cheek-l", "member": b, "part": "member"}, {"id": f"m-flap-{i + 1}-tray", "of": f"flap-{i + 1}-tray", "member": b, "part": "member"}]
        structure.append(pivot(i, 0.010, 0.016))
    return {"body": body, "shapes": shapes, "thermal": thermal, "structure": structure}


def pivot(i, du, dv):
    """🔗 The joint of flap i: the tip-out fittings idealised as a hinge strip along the front-bottom edge, at the size the scale can show."""
    return {"id": f"j-flap-{i + 1}", "kind": "box", "bone": flap_bone(i), "material": "chipboard-foil", "role": "detail", "tone": 0.3, "size": [r(du), r(dv), r(2 * FLAP_HW)], "transform": {"translation": [r(du / 2), r(T / 2), 0.0]}, "member": flap_bone(i), "part": "joint"}


def wedge_profile():
    """🥾 Cheeks and tray of a flap merged into one wedge (1:100)."""
    t = tray_profile()
    return [t[3], t[2], t[1], CHEEK[2], CHEEK[3]]


def scale_100():
    """🔎 1:100 (feature 15 mm): the carcass as one frame with the compartments as holes, the top and kick board as boards, the flap fronts, one wedge per tray, one block per handle, the remaining wall bracket as an angle of 15 mm legs (it stands 25 mm above the top, more than one smallest feature, and is the top of the declared height; without it the body of 1:100 was 1.465 m against 1.49 m: silhouette repair SF of 2026-11-23)."""
    parts = [blanket_part(100), back_part(100)]
    top = part("top", "carcass", "chipboard-foil", rect(TB, D, TOP[0], TOP[1]), -HW, HW, 0.85, 5, region="carcass", front_z=3)
    kick = part("kick", "carcass", "chipboard-foil", rect(CARCASS_FRONT - T, CARCASS_FRONT, 0.0, PLINTH_H), -INNER, INNER, 0.6, 2, views=(ELEV, FRONT), region="plinth", front_z=1)
    parts += [top, kick]
    bracket = [[0.0, TOP[1]], [BRACKET_L, TOP[1]], [BRACKET_L, TOP[1] + 0.015], [0.015, TOP[1] + 0.015], [0.015, TOP[1] + BRACKET_H], [0.0, TOP[1] + BRACKET_H]]
    parts.append(part("bracket", "carcass", "zinc-plated-steel", [[r(a), r(c)] for a, c in bracket], BRACKET_W[0], BRACKET_W[1], 0.3, 9, front_z=9))
    for i in range(3):
        parts.append(flap_front(i))
        parts.append(part(f"flap-{i + 1}-tray", flap_bone(i), "chipboard-foil", wedge_profile(), -CHEEK_W[1], CHEEK_W[1], 0.65, 2, views=(ELEV,), role="detail"))
        sizes, mat = handle_sizes(i)
        parts.extend(handle_parts(i, sizes, mat, 100))
    frame = frame_yz("frame", [(BOTTOM[1], DIVIDERS[0][0]), (DIVIDERS[0][1], DIVIDERS[1][0])], PLINTH_H, TOP[0], open_top=DIVIDERS[1][1])
    body = [solid_of(p) for p in parts] + [frame]
    shapes = {ELEV: [], FRONT: [], PLAN: []}
    for p in parts:
        merge(shapes, shapes_of(p))
    fs = frame_shapes("frame", TOP[0], None)
    for v in fs:
        for s in fs[v]:
            s["region"] = "carcass"
            s.pop("regions", None)
    merge(shapes, {ELEV: fs[ELEV], FRONT: fs[FRONT], PLAN: []})
    f1 = FLAP_BOTTOMS[0] + BLANKET_T
    for k, (za, zb, ya, yb) in enumerate([(-0.30, -0.18, 0.05, 0.07), (0.08, 0.24, 0.06, 0.04)]):
        shapes[FRONT].append(stroke(f"scuff-{k + 1}", "flap-1", [(za, f1 + ya), (zb, f1 + yb)], z=22, tone=0.35))
    thermal = [{"id": "th-blanket", "of": "blanket", "region": "blanket"}, {"id": "th-back", "of": "back", "region": "back-panel"}, {"id": "th-frame", "of": "frame", "region": "carcass"}, {"id": "th-top", "of": "top", "region": "carcass"}, {"id": "th-kick", "of": "kick", "region": "plinth"}]
    thermal += [{"id": f"th-flap-{i + 1}", "of": f"flap-{i + 1}-front", "region": "flaps"} for i in range(3)]
    structure = [{"id": "m-frame", "of": "frame", "member": "carcass", "part": "member"}, {"id": "m-top", "of": "top", "member": "carcass", "part": "member"}, {"id": "m-back", "of": "back", "member": "carcass", "part": "member"}]
    structure.append({"id": "j-bracket", "of": "bracket", "member": "carcass", "part": "joint"})
    for i in range(3):
        b = flap_bone(i)
        structure += [{"id": f"m-flap-{i + 1}-front", "of": f"flap-{i + 1}-front", "member": b, "part": "member"}, {"id": f"m-flap-{i + 1}-tray", "of": f"flap-{i + 1}-tray", "member": b, "part": "member"}, pivot(i, 0.015, 0.016)]
    return {"body": body, "shapes": shapes, "thermal": thermal, "structure": structure}


def scale_200():
    """🗺️ 1:200 (feature 30 mm): the carcass frame takes the top band and the bottom in, a 30 mm plinth zone, the back as a sheet, the flap fronts as sheets; trays and handles are left out."""
    parts = [blanket_part(200), back_part(200)]
    kick = part("kick", "carcass", "chipboard-foil", rect(CARCASS_FRONT - 0.030, CARCASS_FRONT, 0.0, PLINTH_H), -INNER, INNER, 0.6, 2, views=(ELEV, FRONT), region="plinth", front_z=1)
    parts.append(kick)
    for i in range(3):
        parts.append(flap_front(i, sheet=True))
    frame = frame_yz("frame", [(BOTTOM[1], DIVIDERS[0][0]), (DIVIDERS[0][1], DIVIDERS[1][0]), (DIVIDERS[1][1], TOP[0])], PLINTH_H, H)
    body = [solid_of(p) for p in parts] + [frame]
    shapes = {ELEV: [], FRONT: [], PLAN: []}
    for p in parts:
        merge(shapes, shapes_of(p))
    fs = frame_shapes("frame", H, None)
    for v in fs:
        for s in fs[v]:
            s["region"] = "carcass"
            s.pop("regions", None)
    merge(shapes, fs)
    thermal = [{"id": "th-blanket", "of": "blanket", "region": "blanket"}, {"id": "th-back", "of": "back", "region": "back-panel"}, {"id": "th-frame", "of": "frame", "region": "carcass"}, {"id": "th-kick", "of": "kick", "region": "plinth"}]
    thermal += [{"id": f"th-flap-{i + 1}", "of": f"flap-{i + 1}-front", "region": "flaps"} for i in range(3)]
    structure = [{"id": "m-frame", "of": "frame", "member": "carcass", "part": "member"}, {"id": "m-back", "of": "back", "member": "carcass", "part": "member"}]
    for i in range(3):
        b = flap_bone(i)
        structure += [{"id": f"m-flap-{i + 1}-front", "of": f"flap-{i + 1}-front", "member": b, "part": "member"}, pivot(i, 0.030, 0.030)]
    return {"body": body, "shapes": shapes, "thermal": thermal, "structure": structure}


def scale_500():
    """🌍 1:500 (feature 75 mm): one carcass frame with one cavity (dividers and plinth merged), the back as a sheet, three flap sheets that close the 3 mm gaps; the skeleton keeps the frame, the back and one plate per flap."""
    parts = [blanket_part(500), back_part(500)]
    parts[1]["regions"] = None
    for i in range(3):
        ext = FLAP_BOTTOMS[i + 1] - FLAP_BOTTOMS[i] - FLAP_H - 0.001 if i < 2 else TOP[0] - FLAP_BOTTOMS[2] - FLAP_H - 0.0005
        parts.append(flap_front(i, top_extra=ext, sheet=True, pen=0.2))
    frame = frame_yz("frame", [(BOTTOM[1], TOP[0])], 0.0, H)
    body = [solid_of(p) for p in parts] + [frame]
    shapes = {ELEV: [], FRONT: [], PLAN: []}
    for p in parts:
        merge(shapes, shapes_of(p))
    fs = frame_shapes("frame", H, ["carcass", "plinth"], pen=0.2)
    merge(shapes, fs)
    t_frame = frame_yz("th-frame", [(BOTTOM[1], TOP[0])], PLINTH_H, H)
    t_frame.update({"region": "carcass"})
    plinth = {"id": "th-plinth", "kind": "box", "bone": "carcass", "material": "chipboard-foil", "role": "body", "tone": 0.6, "size": [0.075, 0.075, r(2 * INNER)], "transform": {"translation": [r(CARCASS_FRONT - 0.0375), 0.0375, 0.0]}, "region": "plinth"}
    thermal = [{"id": "th-blanket", "of": "blanket", "region": "blanket"}, {"id": "th-back", "of": "back", "region": "back-panel"}, t_frame, plinth]
    thermal += [{"id": f"th-flap-{i + 1}", "of": f"flap-{i + 1}-front", "region": "flaps"} for i in range(3)]
    structure = [{"id": "m-frame", "of": "frame", "member": "carcass", "part": "member"}, {"id": "m-back", "of": "back", "member": "carcass", "part": "member"}]
    structure += [{"id": f"m-flap-{i + 1}-front", "of": f"flap-{i + 1}-front", "member": flap_bone(i), "part": "member"} for i in range(3)]
    return {"body": body, "shapes": shapes, "thermal": thermal, "structure": structure}
