"""🧊️ The handcrafted 3D solids of the used close-coupled WC per scale (1:50, 1:100, 1:200), their thermal and structural lists and the masses derived from them.

@see ./author_geometry.py — the hand-chosen numbers
@see ./author-used-wc-pan-cistern.py — the chisel that writes the asset document
"""
import math

from author_geometry import (BOLTS_Z, BOWL_LOOP, BOWL_OUTER, BRACKET, BUFFER, BUFFERS, BUTTON, CAP, CARD, CARD_T, CIS, CLID, DECK, FILL_VALVE, FLUSH_VALVE, FOOT, FOOT_PLATE, G, HINGE, LID, LID_LUG_Z, LS, LUG, NECK_TOP, NECK_Z, ORIGIN, PAL_BOARD_W, PAL_ROW_X, PAL_ROW_Z, PAL_TOP_Z, PAL_X, PAL_Z, PALLET_H, PH, PIN, REAR, SEAT, SEAT_LUG_Z, SPIGOT, TRAP, TRAP_WALL, XB, Y_BLK1, Y_BOT1, Y_CROSS1, clipped_ellipse, ellipse_pts, loc, poly_area, poly_centroid, r6, r_envelope, revolve_loop_volume, revolve_open_volume, rounded_rect, scale_about, superellipse)

DENS = {"vitreous-china-pan": 2350, "vitreous-china-cistern": 2350, "duroplast-seat": 1500, "rubber-buffer": 1200, "chromed-zinc": 6600, "pp-fittings": 905, "pe-cap": 950, "spruce-pallet": 470, "corrugated-board": 120}


def box(id, bone, x0, x1, y0, y1, z0, z1, material, tone=0.6, role="body", **kw):
    """📦️ An axis-aligned box from world rest bounds (y from the floor), written in the frame of its bone."""
    u, v = loc(bone, (x0 + x1) / 2, (y0 + y1) / 2)
    s = {"id": id, "kind": "box", "bone": bone, "material": material, "role": role, "tone": tone, "size": [r6(x1 - x0), r6(y1 - y0), r6(z1 - z0)], "transform": {"translation": [u, v, r6((z0 + z1) / 2)]}}
    s.update(kw)
    return s


def ext_zx(id, bone, outline, y0, y1, material, tone, holes=None, taper=None, **kw):
    """🧱️ A vertical extrusion of a plan outline (world x, z) from world height y0 to y1, with optional holes and taper."""
    ox, oy = ORIGIN[bone]
    s = {"id": id, "kind": "extrude", "bone": bone, "material": material, "role": "body", "tone": tone, "plane": "zx", "profile": [[r6(z), r6(x - ox)] for x, z in outline], "from": r6(y0 - oy), "to": r6(y1 - oy)}
    if holes:
        s["holes"] = [[[r6(z), r6(x - ox)] for x, z in h] for h in holes]
    if taper:
        s["taperTo"] = list(taper)
    s.update(kw)
    return s


def ext_xy(id, bone, pts, z0, z1, material, tone, **kw):
    """🪚️ A side profile (world x, y) swept laterally from z0 to z1."""
    s = {"id": id, "kind": "extrude", "bone": bone, "material": material, "role": "body", "tone": tone, "plane": "xy", "profile": [list(loc(bone, x, y)) for x, y in pts], "from": r6(z0), "to": r6(z1)}
    s.update(kw)
    return s


def ext_yz(id, bone, pts, x0, x1, material, tone, **kw):
    """🪵️ A front profile (world z, y) swept along x from x0 to x1."""
    ox, oy = ORIGIN[bone]
    s = {"id": id, "kind": "extrude", "bone": bone, "material": material, "role": "body", "tone": tone, "plane": "yz", "profile": [[r6(y - oy), r6(z)] for z, y in pts], "from": r6(x0 - ox), "to": r6(x1 - ox)}
    s.update(kw)
    return s


def rev_y(id, bone, x, y, z, profile, material, tone, ls=None, **kw):
    """🏺️ A profile [height above y, radius ahead] revolved about the vertical through world (x, y, z); with ls the lateral half-width is ls times the radius (the kernel keeps the radius across and stretches the ahead direction by lateralScale, so the profile is written at the lateral radius and stretched by 1/ls)."""
    u, v = loc(bone, x, y)
    k = ls or 1.0
    s = {"id": id, "kind": "revolve", "bone": bone, "material": material, "role": "body", "tone": tone, "axis": [0, 1, 0], "origin": [u, v, r6(z)], "profile": [[r6(a), r6(r * k)] for a, r in profile]}
    if ls:
        s["lateralScale"] = r6(1 / ls)
    s.update(kw)
    return s


def rev_x(id, bone, x, y, z, profile, material, tone, **kw):
    """🔩️ A profile [length ahead of x, radius] revolved about the forward axis through world (x, y, z)."""
    u, v = loc(bone, x, y)
    s = {"id": id, "kind": "revolve", "bone": bone, "material": material, "role": "body", "tone": tone, "axis": [1, 0, 0], "origin": [u, v, r6(z)], "profile": [[r6(a), r6(r)] for a, r in profile]}
    s.update(kw)
    return s


def capsule(id, bone, p0, p1, r, material, tone, z0=0.0, z1=0.0, **kw):
    """💊️ A capsule between two world points (x, y) at lateral z0 and z1."""
    a, b = loc(bone, *p0), loc(bone, *p1)
    s = {"id": id, "kind": "capsule", "bone": bone, "material": material, "role": "body", "tone": tone, "p0": [a[0], a[1], r6(z0)], "p1": [b[0], b[1], r6(z1)], "r0": r6(r), "r1": r6(r)}
    s.update(kw)
    return s


def ring(cx, cz, rx, rz, t, n):
    """⭕️ Outer and inner superellipse rings of a wall of thickness t."""
    return superellipse(cx, cz, rx, rz, n), superellipse(cx, cz, rx - t, rz - t, n)


def rect_ring(x0, x1, z0, z1, r, t):
    """▢️ Outer and inner rounded rectangles of a wall of thickness t."""
    return rounded_rect(x0, x1, z0, z1, r), rounded_rect(x0 + t, x1 - t, z0 + t, z1 - t, max(r - t, 0.004))


def seat_outline():
    """🪑️ Plan outline of the seat ring and the lid: the rim ellipse cut straight at the back."""
    return clipped_ellipse(SEAT["cx"], SEAT["rx"], SEAT["rz"], SEAT["xcut"])


def inset(pts, d):
    """↘️ A convex plan ring moved inwards by d along the bisectors of its corners."""
    out = []
    n = len(pts)
    for i in range(n):
        (x0, z0), (x1, z1), (x2, z2) = pts[i - 1], pts[i], pts[(i + 1) % n]
        e0 = (x1 - x0, z1 - z0)
        e1 = (x2 - x1, z2 - z1)
        l0, l1 = math.hypot(*e0), math.hypot(*e1)
        n0 = (-e0[1] / l0, e0[0] / l0)
        n1 = (-e1[1] / l1, e1[0] / l1)
        bx, bz = n0[0] + n1[0], n0[1] + n1[1]
        bl = math.hypot(bx, bz)
        cosh = (n0[0] * bx + n0[1] * bz) / bl
        out.append((x1 + bx / bl * d / cosh, z1 + bz / bl * d / cosh))
    return out


def seat_hole():
    """🕳️ Plan outline of the seat opening."""
    return ellipse_pts(SEAT["hx"], 0.0, SEAT["hrx"], SEAT["hrz"])


def neck_profile(inset=0.003):
    """🦢️ Side profile (world x, y) of the neck that fills between the rear body and the back of the bowl, on the foot plate."""
    ys = [0.235, 0.25, 0.265, 0.28, 0.30, NECK_TOP]
    back = REAR["x1"] + 0.004
    pts = [(back, PH + 0.235), (back, PH + NECK_TOP)]
    for y in reversed(ys):
        pts.append((XB - r_envelope(BOWL_LOOP, y) + inset, PH + y))
    return pts


def pallet50():
    """🪵️ The 1:50 half pallet (800 x 600 x 144 mm): five top boards, three cross boards, nine blocks, three bottom boards, and the cardboard sheet under the WC."""
    b = "pallet"
    w = PAL_BOARD_W / 2
    s = [box("pallet-top-boards", b, -PAL_X, PAL_X, Y_CROSS1, PALLET_H, PAL_TOP_Z[0] - w, PAL_TOP_Z[0] + w, "spruce-pallet", 0.72, repeat={"count": 5, "translation": [0, 0, 0.125]}),
         box("pallet-cross-boards", b, PAL_ROW_X[0] - w, PAL_ROW_X[0] + w, Y_BLK1, Y_CROSS1, -PAL_Z, PAL_Z, "spruce-pallet", 0.64, repeat={"count": 3, "translation": [0.35, 0, 0]})]
    for name, x in zip(("rear", "middle", "front"), PAL_ROW_X):
        s.append(box(f"pallet-blocks-{name}", b, x - w, x + w, Y_BOT1, Y_BLK1, PAL_ROW_Z[0] - w, PAL_ROW_Z[0] + w, "spruce-pallet", 0.58, repeat={"count": 3, "translation": [0, 0, 0.25]}))
    s.append(box("pallet-bottom-boards", b, -PAL_X, PAL_X, 0.0, Y_BOT1, PAL_ROW_Z[0] - w, PAL_ROW_Z[0] + w, "spruce-pallet", 0.6, repeat={"count": 3, "translation": [0, 0, 0.25]}))
    s.append(box("cardboard", b, CARD["x0"], CARD["x1"], PALLET_H, PH, -CARD["z"], CARD["z"], "corrugated-board", 0.5, sheet=True))
    return s


def stringer_comb():
    """🪮️ Front profile (world z, y) of a 1:100 stringer: the cross board over its three blocks."""
    w = PAL_BOARD_W / 2
    pts = [(-PAL_Z, Y_CROSS1), (-PAL_Z, Y_BOT1)]
    for k, z in enumerate(PAL_ROW_Z):
        if k:
            pts += [(z - w, Y_BLK1), (z - w, Y_BOT1)]
        pts += [(z + w, Y_BOT1), (z + w, Y_BLK1)] if k < len(PAL_ROW_Z) - 1 else [(PAL_Z, Y_BOT1)]
    return pts + [(PAL_Z, Y_CROSS1)]


def pallet100():
    """🪵️ The 1:100 half pallet: top boards, the three stringers of cross board and blocks as one comb profile each, bottom boards and the cardboard."""
    b = "pallet"
    w = PAL_BOARD_W / 2
    comb = stringer_comb()
    s = [box("pallet-top-boards-100", b, -PAL_X, PAL_X, Y_CROSS1, PALLET_H, PAL_TOP_Z[0] - w, PAL_TOP_Z[0] + w, "spruce-pallet", 0.72, repeat={"count": 5, "translation": [0, 0, 0.125]}),
         ext_yz("pallet-stringers-100", b, comb, PAL_ROW_X[0] - w, PAL_ROW_X[0] + w, "spruce-pallet", 0.62, repeat={"count": 3, "translation": [0.35, 0, 0]}),
         box("pallet-bottom-boards-100", b, -PAL_X, PAL_X, 0.0, Y_BOT1, PAL_ROW_Z[0] - w, PAL_ROW_Z[0] + w, "spruce-pallet", 0.6, repeat={"count": 3, "translation": [0, 0, 0.25]}),
         box("cardboard-100", b, CARD["x0"], CARD["x1"], PALLET_H, PH, -CARD["z"], CARD["z"], "corrugated-board", 0.5, sheet=True)]
    return s


PAL200_BOT = 0.03


def pallet200():
    """🪵️ The 1:200 half pallet: the top deck as one slab, three runners of bottom board and blocks as comb profiles, and the cardboard."""
    b = "pallet"
    w = PAL_BOARD_W / 2
    pts = [(-PAL_X, 0.0), (PAL_X, 0.0), (PAL_X, Y_BLK1), (PAL_ROW_X[2] - w, Y_BLK1), (PAL_ROW_X[2] - w, PAL200_BOT), (PAL_ROW_X[1] + w, PAL200_BOT), (PAL_ROW_X[1] + w, Y_BLK1), (PAL_ROW_X[1] - w, Y_BLK1), (PAL_ROW_X[1] - w, PAL200_BOT), (PAL_ROW_X[0] + w, PAL200_BOT), (PAL_ROW_X[0] + w, Y_BLK1), (-PAL_X, Y_BLK1)]
    return [box("pallet-deck-200", b, -PAL_X, PAL_X, Y_BLK1, PALLET_H, -PAL_Z, PAL_Z, "spruce-pallet", 0.7),
            ext_xy("pallet-runners-200", b, pts, PAL_ROW_Z[0] - w, PAL_ROW_Z[0] + w, "spruce-pallet", 0.6, repeat={"count": 3, "translation": [0, 0, 0.25]}),
            box("cardboard-200", b, CARD["x0"], CARD["x1"], PALLET_H, PH, -CARD["z"], CARD["z"], "corrugated-board", 0.5, sheet=True)]


def foot_rings():
    """🦶️ Outer and inner plan rings of the hollow foot at the floor."""
    return ring(FOOT["cx"], 0.0, FOOT["rx"], FOOT["rz"], FOOT["t"], FOOT["n"])


def foot_top_outline():
    """🦶️ Plan outline of the foot where it meets the bowl (the tapered outer ring)."""
    out, _ = foot_rings()
    return scale_about(out, FOOT["top"], poly_centroid(out))


def pan50():
    """🚽️ The 1:50 pan: the bowl with its rim as one revolved shell, the hollow tapered foot and its top plate, the hollow rear body with its bottom plate, the neck, the cistern deck, the horizontal outlet spigot with its red cap, the trapway, and the chromed hinge brackets with their pin."""
    p = "pan"
    out, inn = foot_rings()
    rear_out, rear_in = rect_ring(REAR["x0"], REAR["x1"], -REAR["z"], REAR["z"], REAR["r"], REAR["t"])
    s = [rev_y("bowl", p, XB, PH, 0.0, BOWL_LOOP, "vitreous-china-pan", 0.92, ls=LS),
         ext_zx("foot", p, out, PH, PH + FOOT["y1"], "vitreous-china-pan", 0.86, holes=[inn], taper=(1.0, FOOT["top"])),
         ext_zx("foot-plate", p, foot_top_outline(), PH + FOOT_PLATE[0], PH + FOOT_PLATE[1], "vitreous-china-pan", 0.86),
         ext_zx("rear-body", p, rear_out, PH + REAR["yw"], PH + REAR["y1"], "vitreous-china-pan", 0.88, holes=[rear_in]),
         ext_zx("rear-plate", p, rear_out, PH + REAR["y0"], PH + REAR["yw"], "vitreous-china-pan", 0.84),
         ext_xy("neck", p, neck_profile(), -NECK_Z, NECK_Z, "vitreous-china-pan", 0.86),
         ext_zx("deck", p, rounded_rect(DECK["x0"], DECK["x1"], -DECK["z"], DECK["z"], DECK["r"]), PH + DECK["y0"], PH + DECK["y1"], "vitreous-china-pan", 0.9),
         rev_x("spigot", p, SPIGOT["x0"], PH + SPIGOT["y"], 0.0, [(0, SPIGOT["ri"]), (SPIGOT["x1"] - SPIGOT["x0"], SPIGOT["ri"]), (SPIGOT["x1"] - SPIGOT["x0"], SPIGOT["ro"]), (0, SPIGOT["ro"]), (0, SPIGOT["ri"])], "vitreous-china-pan", 0.84),
         rev_x("outlet-cap", p, CAP["x0"], PH + SPIGOT["y"], 0.0, [(0, 0), (0, CAP["ro"]), (CAP["x1"] - CAP["x0"], CAP["ro"]), (CAP["x1"] - CAP["x0"], CAP["ri"]), (CAP["end"], CAP["ri"]), (CAP["end"], 0)], "pe-cap", 0.3)]
    for k, (a, b, r) in enumerate(TRAP):
        s.append(capsule(f"trapway-{k + 1}", p, (a[0], PH + a[1]), (b[0], PH + b[1]), r, "vitreous-china-pan", 0.5))
    s.append(box("hinge-brackets", p, BRACKET["x0"], BRACKET["x1"], PH + BRACKET["y0"], PH + BRACKET["y1"], -BRACKET["z"] - BRACKET["w"] / 2, -BRACKET["z"] + BRACKET["w"] / 2, "chromed-zinc", 0.55, repeat={"count": 2, "translation": [0, 0, 2 * BRACKET["z"]]}, cornerRadius=0.004))
    s.append(capsule("hinge-pin", p, (HINGE[0], PH + HINGE[1]), (HINGE[0], PH + HINGE[1]), PIN["r"], "chromed-zinc", 0.5, z0=-PIN["z"], z1=PIN["z"]))
    return s


def bowl_profile100():
    """🏺️ The 1:100 bowl shell: the same rim and bowl with fewer profile points and a plain rim."""
    outer = [(0.200, 0.048), (0.240, 0.135), (0.300, 0.198), (0.355, 0.238), (0.365, 0.245), (0.400, 0.245)]
    inner = [(0.400, 0.224), (0.372, 0.214), (0.305, 0.180), (0.235, 0.105), (0.210, 0.032)]
    return outer + inner + [outer[0]]


def pan100():
    """🚽️ The 1:100 pan: bowl shell, hollow foot with a 15 mm top plate, the rear body and neck as solid masses, the deck, spigot and cap as one turned part, a two-piece trapway and the hinge brackets."""
    p = "pan"
    out, inn = foot_rings()
    rear = rounded_rect(REAR["x0"], REAR["x1"], -REAR["z"], REAR["z"], REAR["r"])
    s = [rev_y("bowl-100", p, XB, PH, 0.0, bowl_profile100(), "vitreous-china-pan", 0.92, ls=LS),
         ext_zx("foot-100", p, out, PH, PH + FOOT["y1"], "vitreous-china-pan", 0.86, holes=[inn], taper=(1.0, FOOT["top"])),
         ext_zx("foot-plate-100", p, foot_top_outline(), PH + 0.22, PH + FOOT_PLATE[1], "vitreous-china-pan", 0.86),
         ext_zx("rear-body-100", p, rear, PH + REAR["y0"], PH + REAR["y1"], "vitreous-china-pan", 0.88),
         ext_xy("neck-100", p, neck_profile(), -NECK_Z, NECK_Z, "vitreous-china-pan", 0.86),
         ext_zx("deck-100", p, rounded_rect(DECK["x0"], DECK["x1"], -DECK["z"], DECK["z"], DECK["r"]), PH + DECK["y0"], PH + DECK["y1"], "vitreous-china-pan", 0.9),
         rev_x("spigot-100", p, CAP["x1"], PH + SPIGOT["y"], 0.0, [(0, 0), (0, SPIGOT["ro"]), (SPIGOT["x1"] - CAP["x1"], SPIGOT["ro"]), (SPIGOT["x1"] - CAP["x1"], 0)], "vitreous-china-pan", 0.84),
         rev_x("outlet-cap-100", p, CAP["x0"], PH + SPIGOT["y"], 0.0, [(0, 0), (0, CAP["ro"]), (CAP["x1"] - CAP["x0"], CAP["ro"]), (CAP["x1"] - CAP["x0"], 0)], "pe-cap", 0.3),
         capsule("trapway-100-1", p, (XB, PH + 0.205), (0.040, PH + 0.100), 0.032, "vitreous-china-pan", 0.5),
         capsule("trapway-100-2", p, (0.040, PH + 0.100), (-0.205, PH + 0.180), 0.036, "vitreous-china-pan", 0.5),
         box("hinge-brackets-100", p, BRACKET["x0"], BRACKET["x1"], PH + BRACKET["y0"], PH + BRACKET["y1"], -0.11, -0.095, "chromed-zinc", 0.55, repeat={"count": 2, "translation": [0, 0, 0.205]})]
    return s


def bowl_profile200():
    """🏺️ The 1:200 bowl as a filled turned body with a dished top (the bowl 0.10 m deep at its centre)."""
    return [(0.200, 0.0), (0.200, 0.06), (0.240, 0.135), (0.300, 0.198), (0.360, 0.243), (0.400, 0.245), (0.400, 0.215), (0.340, 0.15), (0.300, 0.0)]


def pan200():
    """🚽️ The 1:200 pan: dished bowl body, solid tapered foot, rear body, neck, 30 mm deck and the outlet with its cap as one turned part."""
    p = "pan"
    out, _ = foot_rings()
    rear = rounded_rect(REAR["x0"], REAR["x1"], -REAR["z"], REAR["z"], REAR["r"])
    return [rev_y("bowl-200", p, XB, PH, 0.0, bowl_profile200(), "vitreous-china-pan", 0.92, ls=LS),
            ext_zx("foot-200", p, out, PH, PH + FOOT["y1"], "vitreous-china-pan", 0.86, taper=(1.0, FOOT["top"])),
            ext_zx("rear-body-200", p, rear, PH + REAR["y0"], PH + 0.37, "vitreous-china-pan", 0.88),
            ext_xy("neck-200", p, neck_profile(), -NECK_Z, NECK_Z, "vitreous-china-pan", 0.86),
            ext_zx("deck-200", p, rounded_rect(DECK["x0"], DECK["x1"], -DECK["z"], DECK["z"], DECK["r"]), PH + 0.37, PH + DECK["y1"], "vitreous-china-pan", 0.9),
            rev_x("outlet-200", p, CAP["x0"], PH + SPIGOT["y"], 0.0, [(0, 0), (0, CAP["ro"]), (CAP["x1"] - CAP["x0"], CAP["ro"]), (CAP["x1"] - CAP["x0"], SPIGOT["ro"]), (SPIGOT["x1"] - CAP["x0"], SPIGOT["ro"]), (SPIGOT["x1"] - CAP["x0"], 0)], "pe-cap", 0.3)]


def cistern_outline():
    """🧰️ Plan outline of the cistern tank."""
    return rounded_rect(CIS["x0"], CIS["x1"], -CIS["z"], CIS["z"], CIS["r"])


def clid_outline():
    """🧰️ Plan outline of the cistern lid."""
    return rounded_rect(CLID["x0"], CLID["x1"], -CLID["z"], CLID["z"], CLID["r"])


def button_hole():
    """🔘️ Plan ring of the hole of the push button in the cistern lid."""
    return ellipse_pts(BUTTON["x"], 0.0, BUTTON["hole"], BUTTON["hole"], 32)


def tube(r_in, r_out, h):
    """🧪️ The closed [axial, radial] loop of a thin-walled tube of height h."""
    return [(0, r_in), (h, r_in), (h, r_out), (0, r_out), (0, r_in)]


def cistern50():
    """🧰️ The 1:50 cistern: bottom, 9 mm walls, the flush valve and the fill valve inside, the lid with the hole of the replaced push button and the button."""
    c = "cistern"
    out, inn = rect_ring(CIS["x0"], CIS["x1"], -CIS["z"], CIS["z"], CIS["r"], CIS["t"])
    return [ext_zx("cistern-bottom", c, out, PH + CIS["y0"], PH + CIS["yb"], "vitreous-china-cistern", 0.86),
            ext_zx("cistern-tank", c, out, PH + CIS["yb"], PH + CIS["y1"], "vitreous-china-cistern", 0.9, holes=[inn]),
            rev_y("flush-valve", c, FLUSH_VALVE["x"], PH + CIS["yb"], FLUSH_VALVE["z"], tube(FLUSH_VALVE["r"] - 0.003, FLUSH_VALVE["r"], FLUSH_VALVE["y1"] - CIS["yb"]), "pp-fittings", 0.7),
            rev_y("fill-valve", c, FILL_VALVE["x"], PH + CIS["yb"], FILL_VALVE["z"], tube(FILL_VALVE["r"] - 0.003, FILL_VALVE["r"], FILL_VALVE["y1"] - CIS["yb"]), "pp-fittings", 0.7),
            ext_zx("cistern-lid", "cistern-lid", clid_outline(), PH + CLID["y0"], PH + CLID["y1"], "vitreous-china-cistern", 0.94, holes=[button_hole()]),
            rev_y("button", "button", BUTTON["x"], PH + BUTTON["y0"], 0.0, [(0, 0), (0, BUTTON["r"]), (BUTTON["y1"] - BUTTON["y0"], BUTTON["r"]), (BUTTON["y1"] - BUTTON["y0"], 0)], "pp-fittings", 0.98)]


def cistern100():
    """🧰️ The 1:100 cistern: the tank as one mass, the lid and the button."""
    return [ext_zx("cistern-100", "cistern", cistern_outline(), PH + CIS["y0"], PH + CIS["y1"], "vitreous-china-cistern", 0.9),
            ext_zx("cistern-lid-100", "cistern-lid", clid_outline(), PH + CLID["y0"], PH + CLID["y1"], "vitreous-china-cistern", 0.94, holes=[button_hole()]),
            rev_y("button-100", "button", BUTTON["x"], PH + BUTTON["y0"], 0.0, [(0, 0), (0, BUTTON["r"]), (BUTTON["y1"] - BUTTON["y0"], BUTTON["r"]), (BUTTON["y1"] - BUTTON["y0"], 0)], "pp-fittings", 0.98)]


def cistern200():
    """🧰️ The 1:200 cistern: tank, a 30 mm lid and the button as a 30 mm disc through it."""
    return [ext_zx("cistern-200", "cistern", cistern_outline(), PH + CIS["y0"], PH + 0.76, "vitreous-china-cistern", 0.9),
            ext_zx("cistern-lid-200", "cistern-lid", clid_outline(), PH + 0.76, PH + CLID["y1"], "vitreous-china-cistern", 0.94, holes=[button_hole()]),
            rev_y("button-200", "button", BUTTON["x"], PH + 0.765, 0.0, [(0, 0), (0, BUTTON["r"]), (0.03, BUTTON["r"]), (0.03, 0)], "pp-fittings", 0.98)]


def lugs(prefix, bone, zc, h0=LUG["y0"], h1=LUG["y1"]):
    """🔗️ The two hinge lugs of the seat or the lid around the pin."""
    return box(prefix, bone, LUG["x0"], LUG["x1"], PH + h0, PH + h1, -zc - LUG["w"] / 2, -zc + LUG["w"] / 2, "duroplast-seat", 0.8, repeat={"count": 2, "translation": [0, 0, 2 * zc]})


def seat50():
    """🪑️ The 1:50 seat set: seat ring with its four rubber buffers and hinge lugs, the lid with its lugs."""
    s = [ext_zx("seat-ring", "seat", seat_outline(), PH + SEAT["yb"], PH + SEAT["y1"], "duroplast-seat", 0.82, holes=[seat_hole()]),
         ext_zx("seat-skirt", "seat", seat_outline(), PH + SEAT["y0"], PH + SEAT["yb"], "duroplast-seat", 0.78, holes=[inset(seat_outline(), SEAT["skirt"])])]
    for name, x in (("front", BUFFERS[0][0]), ("rear", BUFFERS[2][0])):
        s.append(box(f"seat-buffers-{name}", "seat", x - BUFFER["dx"] / 2, x + BUFFER["dx"] / 2, PH + BUFFER["y0"], PH + BUFFER["y1"], -0.10 - BUFFER["dz"] / 2, -0.10 + BUFFER["dz"] / 2, "rubber-buffer", 0.25, repeat={"count": 2, "translation": [0, 0, 0.2]}))
    s.append(lugs("seat-lugs", "seat", SEAT_LUG_Z))
    s.append(ext_zx("lid", "lid", seat_outline(), PH + LID["ys"], PH + LID["y1"], "duroplast-seat", 0.86, sheet=True))
    s.append(ext_zx("lid-skirt", "lid", seat_outline(), PH + LID["y0"], PH + LID["ys"], "duroplast-seat", 0.8, holes=[inset(seat_outline(), LID["skirt"])]))
    s.append(lugs("lid-lugs", "lid", LID_LUG_Z))
    return s


def seat100():
    """🪑️ The 1:100 seat set: seat ring down on the rim (buffers merged), lugs, lid."""
    return [ext_zx("seat-ring-100", "seat", seat_outline(), PH + BUFFER["y0"], PH + SEAT["y1"], "duroplast-seat", 0.82, holes=[seat_hole()]),
            lugs("seat-lugs-100", "seat", SEAT_LUG_Z),
            ext_zx("lid-100", "lid", seat_outline(), PH + LID["y0"], PH + LID["y1"], "duroplast-seat", 0.86),
            lugs("lid-lugs-100", "lid", LID_LUG_Z)]


def seat_outline200():
    """🪑️ The seat and lid outline at 1:200, its back edge carried to the hinge axis."""
    return clipped_ellipse(SEAT["cx"], SEAT["rx"], SEAT["rz"], HINGE[0])


def seat200():
    """🪑️ The 1:200 seat set: a 30 mm seat ring on the rim and a 30 mm lid, both reaching back to the hinge axis."""
    return [ext_zx("seat-ring-200", "seat", seat_outline200(), PH + 0.40, PH + 0.43, "duroplast-seat", 0.82, holes=[seat_hole()]),
            ext_zx("lid-200", "lid", seat_outline200(), PH + 0.43, PH + 0.46, "duroplast-seat", 0.86)]


def solid_volume(s):
    """📐️ Volume and bone-local centre of a body solid (repeat copies included)."""
    k = s["kind"]
    n = s.get("repeat", {}).get("count", 1)
    if k == "box":
        a, b, c = s["size"]
        vol = a * b * c
        cen = list(s["transform"]["translation"])
    elif k == "extrude":
        prof = [tuple(p) for p in s["profile"]]
        area = poly_area(prof) - sum(poly_area([tuple(p) for p in h]) for h in s.get("holes", []))
        t0, t1 = s.get("taperTo", [1, 1])
        vol = area * (s["to"] - s["from"]) * (t0 * t0 + t0 * t1 + t1 * t1) / 3
        ca, cb = poly_centroid(prof)
        mid = (s["from"] + s["to"]) / 2
        cen = {"zx": [cb, mid, ca], "xy": [ca, cb, mid], "yz": [mid, ca, cb]}[s["plane"]]
    elif k == "revolve":
        prof = [tuple(p) for p in s["profile"]]
        ls = s.get("lateralScale", 1.0)
        if prof[0] == prof[-1]:
            vol, ac = revolve_loop_volume(prof, ls)
        else:
            vol, ac = revolve_open_volume(prof, ls)
        o, ax = s["origin"], s["axis"]
        cen = [o[i] + ax[i] * ac for i in range(3)]
    elif k == "capsule":
        p0, p1, r = s["p0"], s["p1"], s["r0"]
        L = math.dist(p0, p1)
        vol = math.pi * r * r * L + 4 / 3 * math.pi * r ** 3
        if s["id"].startswith("trapway"):
            ri = r - TRAP_WALL
            vol *= (r * r - ri * ri) / (r * r)
        cen = [(p0[i] + p1[i]) / 2 for i in range(3)]
    else:
        raise ValueError(k)
    rep = s.get("repeat")
    if rep:
        t = rep["translation"]
        cen = [cen[i] + t[i] * (n - 1) / 2 for i in range(3)]
    return vol * n, cen


def mass_of(solids):
    """⚖️ Masses and bone-local centres per bone from section x density of the 1:50 body solids."""
    out = {}
    for s in solids:
        if s.get("texture"):
            continue
        vol, c = solid_volume(s)
        kg = vol * DENS[s["material"]]
        acc = out.setdefault(s["bone"], [0.0, 0.0, 0.0, 0.0, {}])
        acc[0] += kg
        acc[1] += kg * c[0]
        acc[2] += kg * c[1]
        acc[3] += kg * c[2]
        acc[4][s["id"]] = round(kg, 4)
    return {b: {"kg": a[0], "comX": a[1] / a[0], "comY": a[2] / a[0], "comZ": a[3] / a[0], "parts": a[4]} for b, a in out.items()}


B50 = pallet50() + pan50() + cistern50() + seat50()
B100 = pallet100() + pan100() + cistern100() + seat100()
B200 = pallet200() + pan200() + cistern200() + seat200()
MASS = mass_of(B50)
TOTAL_KG = sum(m["kg"] for m in MASS.values())
MASS_BONES = ("pallet", "pan", "cistern", "cistern-lid", "button", "seat", "lid")
MARKER_MATERIAL = {"pallet": "spruce-pallet", "pan": "vitreous-china-pan", "cistern": "vitreous-china-cistern", "cistern-lid": "vitreous-china-cistern", "button": "pp-fittings", "seat": "duroplast-seat", "lid": "duroplast-seat"}


def ref(id, of, **kw):
    """🔗️ A thermal or structural reference to a body solid of the same scale."""
    s = {"id": id, "of": of}
    s.update(kw)
    return s


def markers(suffix, r, skip=()):
    """🎯️ The mass markers of a scale at the centres of mass of the bones."""
    out = []
    for b in MASS_BONES:
        if b in skip:
            continue
        m = MASS[b]
        out.append({"id": f"mass-{b}{suffix}", "kind": "ellipsoid", "bone": b, "material": MARKER_MATERIAL[b], "role": "detail", "tone": 0.3, "radii": [r, r, r], "transform": {"translation": [r6(m["comX"]), r6(m["comY"]), r6(m["comZ"])]}, "member": b, "part": "mass"})
    return out


def bolts(suffix, r, h0, h1):
    """🔩️ The two cistern bolts through the cistern bottom and the deck, as joints of the cistern."""
    return capsule(f"j-cistern-bolts{suffix}", "cistern", (-0.255, PH + h0), (-0.255, PH + h1), r, "chromed-zinc", 0.3, z0=-BOLTS_Z, z1=-BOLTS_Z, repeat={"count": 2, "translation": [0, 0, 2 * BOLTS_Z]}, member="cistern", part="joint")


def structure_list(body, suffix, joints, r):
    """🏗️ A structure list: every body solid a member of its bone (buffers, pin and brackets as joints), the given joints and the mass markers."""
    st = []
    for s in body:
        part = "joint" if any(k in s["id"] for k in ("buffers", "hinge-pin", "hinge-brackets")) else "member"
        st.append(ref("m-" + s["id"], s["id"], member=s["bone"], part=part))
    return st + joints + markers(suffix, r)


REGION_OF = {"pallet": "pallet", "cardboard": "cardboard", "bowl": "pan", "foot": "pan", "rear": "pan", "neck": "pan", "deck": "pan", "spigot": "pan", "trapway": "pan", "outlet": "outlet-cap", "hinge": "seat-set", "cistern": "cistern", "flush": "cistern", "fill": "cistern", "button": "cistern", "seat": "seat-set", "lid": "seat-set"}


def region_of(id):
    """🌡️ The thermal region of a body solid by the first word of its id."""
    if id.startswith("outlet-cap"):
        return "outlet-cap"
    return REGION_OF[id.split("-")[0]]


def thermal_list(body, seat_gap_closer=None):
    """🌡️ A thermal list: every body solid in its region; at 1:50 the seat ring is replaced by one that reaches down to the rim, so the 8 mm buffer gap closes the bowl under the shut lid."""
    out = []
    for s in body:
        if seat_gap_closer and s["id"] == "seat-ring":
            out.append(dict(seat_gap_closer, region="seat-set"))
            continue
        out.append(ref("th-" + s["id"], s["id"], region=region_of(s["id"])))
    return out


SEAT_GAP_CLOSER = ext_zx("th-seat-ring-to-rim", "seat", seat_outline(), PH + BUFFER["y0"], PH + SEAT["y1"], "duroplast-seat", 0.82, holes=[seat_hole()])
TH50 = thermal_list(B50, SEAT_GAP_CLOSER)
TH100 = thermal_list(B100)
TH200 = thermal_list(B200)
ST50 = structure_list(B50, "", [bolts("", 0.005, 0.37, 0.43)], 0.012)
ST100 = structure_list(B100, "-100", [bolts("-100", 0.0075, 0.36, 0.44)], 0.012)
ST200 = structure_list(B200, "-200", [bolts("-200", 0.015, 0.35, 0.45)], 0.018)
