"""✏️ The hand-drawn 2D views of the used electric cooker per authored scale: side elevation (bone-local), front elevation and plan (world rest coordinates), drawn to the same numbers as the solids of author_geometry.py.

See also author-used-electric-cooker.py.
"""
from author_geometry import (
    BLOCK_X, BLOCK_Y1, BOARD_T, BOX_X0, BOX_Y0, BOX_Y1, COIL, COIL_R, COIL_T, DOOR_H, DOOR_T, DOOR_Z, DRAWER_X0, DRAWER_X1, DRAWER_Y0, DRAWER_Y1, DRAWER_Z,
    FASCIA_X0, FASCIA_X1, FASCIA_Y0, FEET, FOOT_H, FOOT_R, HANDLE_R, HANDLE_U, HANDLE_V, HANDLE_Z, HINGE_X, HINGE_Y, KNOB, KNOB_100, KNOB_OLD, KNOB_Y, KNOB_ZS,
    LAMP_ZS, PALLET_H, PALLET_L, PALLET_X0, PALLET_Z, PLATES, PLATE_TOP, REPLACED_KNOB, ROW_Z, STANDOFF_Z, STRINGER_Y1, T, TOP_BOARDS, TOP_BOARDS_100, TOP_X1, TOP_Y0, TOP_Y1,
    TRAY_H, TRAY_X0, TRAY_X1, TRAY_Y0, WIN_U0, WIN_U1, WIN_W, X_SIDE, ZI, ZO, pts, r6,
)

Y0 = PALLET_H


def sh(sid, bone, kind, role, z, tone, filled=True, region=None, regions=None, material=None, pen=None, **geo):
    """✒️ One 2D shape."""
    s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": kind}
    s.update(geo)
    if region:
        s["region"] = region
    if regions:
        s["regions"] = regions
    if material:
        s["material"] = material
    if pen is not None:
        s["penMm"] = pen
    return s


def rect(sid, bone, x0, y0, x1, y1, role="body", z=5, tone=0.6, **kw):
    """⬛ A rectangle from two corners."""
    return sh(sid, bone, "rect", role, z, tone, x=r6(min(x0, x1)), y=r6(min(y0, y1)), w=r6(abs(x1 - x0)), h=r6(abs(y1 - y0)), **kw)


def poly(sid, bone, points, role="body", z=5, tone=0.6, closed=True, filled=True, **kw):
    """🔷 A polygon or polyline."""
    return sh(sid, bone, "poly", role, z, tone, filled=filled, points=pts(points), closed=closed, **kw)


def circle(sid, bone, cx, cy, r, role="detail", z=7, tone=0.4, **kw):
    """⭕ A circle."""
    return sh(sid, bone, "ellipse", role, z, tone, cx=r6(cx), cy=r6(cy), rx=r6(r), ry=r6(r), **kw)


def tex(sid, bone, points, z=6.5, closed=False, tone=0.3):
    """〰️ A texture stroke (traces of use, grain, seams)."""
    return poly(sid, bone, points, role="texture", z=z, tone=tone, closed=closed, filled=False)


def ring(sid, bone, cx, cy, r, z=6.5, tone=0.3):
    """➰ An unfilled texture circle (ring marks, rust rings)."""
    return sh(sid, bone, "ellipse", "texture", z, tone, filled=False, cx=r6(cx), cy=r6(cy), rx=r6(r), ry=r6(r))


def knob_outline(profile, cx, cy):
    """🎛️ The side silhouette of a turned knob standing proud of a face at cx."""
    up = [(cx + a, cy + r) for a, r in profile]
    down = [(cx + a, cy - r) for a, r in reversed(profile) if r > 0]
    return up + down


def plate_row(cx, cy, rp, ro, scale):
    """🔥 Side silhouette of a row of hotplates with their trim rings standing on the hob at height cy."""
    if scale == "200":
        return [(cx - ro, cy), (cx + ro, cy), (cx + ro, cy + 0.007), (cx - ro, cy + 0.007)]
    if scale == "100":
        return [(cx - ro, cy), (cx + ro, cy), (cx + ro, cy + 0.0015), (cx + rp, cy + 0.006), (cx + rp - 0.004, cy + 0.007), (cx - rp + 0.004, cy + 0.007), (cx - rp, cy + 0.006), (cx - ro, cy + 0.0015)]
    return [(cx - ro, cy), (cx + ro, cy), (cx + ro, cy + 0.0012), (cx + rp + 0.0035, cy + 0.0018), (cx + rp + 0.002, cy + 0.006), (cx + rp - 0.003, cy + 0.007), (cx - rp + 0.003, cy + 0.007), (cx - rp - 0.002, cy + 0.006), (cx - rp - 0.0035, cy + 0.0018), (cx - ro, cy + 0.0012)]


def plate_band(sid, bone, lines, cy, scale):
    """➖ At 1:100 and 1:200 the two plate rows (or columns) that stand 7 mm proud are one band over the hob edge: the 41 to 51 mm between them would print as a gap under the office-laser minimum."""
    a0 = lines[0][1] - lines[0][3]
    a1 = lines[1][1] + lines[1][3]
    h = 0.007 if scale == "200" else 0.006
    return poly(sid, bone, [(a0, cy), (a1, cy), (a1, cy + h), (a0, cy + h)], z=5.2, tone=0.12, region="hotplates", material="cast-iron-aged")


def rows():
    """📏 The two hotplate rows (rear, front) seen from the side and the two columns (right, left) seen from the front: centre and the largest plate and ring radii."""
    rear = [p for p in PLATES if p[0] in ("rl", "rr")]
    front = [p for p in PLATES if p[0] in ("fl", "fr")]
    right = [p for p in PLATES if p[0] in ("fr", "rr")]
    left = [p for p in PLATES if p[0] in ("fl", "rl")]
    side = [("rear", rear[0][1], max(p[3] for p in rear), max(p[4] for p in rear)), ("front", front[0][1], max(p[3] for p in front), max(p[4] for p in front))]
    col = [("right", right[0][2], max(p[3] for p in right), max(p[4] for p in right)), ("left", left[0][2], max(p[3] for p in left), max(p[4] for p in left))]
    return side, col


SIDE_ROWS, FRONT_COLS = rows()


def pallet_elevation(scale):
    """🪵 Pallet seen from the right: bottom board, blocks, stringer, the ends of the top boards (1:200: deck and blocks to the floor)."""
    b, R = "pallet", {"region": "pallet", "material": "spruce-pallet"}
    if scale == "200":
        out = [rect("pallet-deck", "pallet-deck", 0, BLOCK_Y1, PALLET_L, PALLET_H, z=1.5, tone=0.6, **R)]
        out += [rect(f"pallet-block-{i + 1}", b, x0, 0, x1, BLOCK_Y1, z=1, tone=0.45, **R) for i, (x0, x1) in enumerate(BLOCK_X)]
        return out
    out = [rect("pallet-bottom", b, 0, 0, PALLET_L, BOARD_T, z=1, tone=0.55, **R)]
    out += [rect(f"pallet-block-{i + 1}", b, x0, BOARD_T, x1, BLOCK_Y1, z=1, tone=0.45, **R) for i, (x0, x1) in enumerate(BLOCK_X)]
    out.append(rect("pallet-stringer", b, 0, BLOCK_Y1, PALLET_L, STRINGER_Y1, z=1.2, tone=0.55, **R))
    out += [rect(f"pallet-top-{i + 1}", "pallet-deck", x0, STRINGER_Y1, x1, PALLET_H, z=1.4, tone=0.6, **R) for i, (x0, x1) in enumerate(TOP_BOARDS if scale == "50" else TOP_BOARDS_100)]
    if scale == "50":
        out.append(rect("pallet-stamp", b, 0.27, 0.045, 0.33, 0.08, role="detail", z=1.6, tone=0.3))
        out.append(tex("pallet-grain-bottom", b, [(0.02, 0.011), (0.25, 0.012), (0.58, 0.010)], z=1.6))
        out.append(tex("pallet-grain-stringer", b, [(0.03, 0.111), (0.31, 0.112), (0.57, 0.110)], z=1.6))
        out.append(tex("pallet-split-block", b, [(0.52, 0.03), (0.54, 0.06), (0.535, 0.09)], z=1.6))
    return out


def door_elevation(scale):
    """🚪 The door edge on its bone (u up from the hinge, v into the oven) with the handle in front of it."""
    d, R = "door", {"region": "oven-door", "material": "enamel-white-aged"}
    out = [rect("door", d, 0, 0, DOOR_H, DOOR_T, z=6, tone=0.9, **R)]
    out.append(rect("window-edge", d, WIN_U0, 0.002 if scale != "200" else 0.0, WIN_U1, 0.006 if scale != "200" else 0.004, role="glazing", z=6.1, tone=0.25, region="door-window", material="door-glass"))
    if scale == "50":
        out.append(rect("window-edge-inner", d, WIN_U0, 0.032, WIN_U1, 0.036, role="glazing", z=6.1, tone=0.3, region="door-window", material="door-glass"))
    if scale == "200":
        out.append(rect("handle", d, HANDLE_U - 0.015, -0.045, HANDLE_U + 0.015, 0, z=6.5, tone=0.75, region="oven-door", material="chrome-steel"))
        return out
    out.append(rect("standoff", d, HANDLE_U - 0.010, HANDLE_V, HANDLE_U + 0.010, 0, role="detail", z=6.4, tone=0.7, region="oven-door", material="chrome-steel"))
    out.append(circle("handle-bar", d, HANDLE_U, HANDLE_V, HANDLE_R, role="body", z=6.5, tone=0.75, region="oven-door", material="chrome-steel"))
    if scale == "50":
        out.append(tex("door-gasket-line", d, [(0.02, 0.036), (0.50, 0.036)], z=6.2))
    return out


def elevation(scale):
    """🖼️ Side elevation from the right: pallet, feet, right side panel, cable coil behind, the hob edge with the plate rows, fascia and knobs, drawer front, door; drawer box and tray behind the side panel so they appear when pulled."""
    c, CAS = "carcass", {"region": "casing", "material": "enamel-white-aged"}
    out = pallet_elevation(scale)
    if scale == "200":
        out.append(poly("side", c, [(0, 0), (0.55, 0), (0.55, HINGE_Y), (X_SIDE, HINGE_Y), (X_SIDE, 0.82), (0, 0.82)], z=5, tone=0.88, **CAS))
        out.append(rect("hob-edge", c, 0, 0.82, TOP_X1, TOP_Y1, z=5.3, tone=0.82, region="hob-top", material="enamel-hob-stained"))
        out.append(rect("fascia", c, X_SIDE, FASCIA_Y0, FASCIA_X1, 0.82, z=5.5, tone=0.86, region="fascia", material="enamel-white-aged"))
    else:
        out += [rect(f"foot-{i + 1}", c, x - FOOT_R, 0, x + FOOT_R, FOOT_H, z=4, tone=0.15, region="casing", material="pp-black") for i, (x, zz) in enumerate(FEET) if zz < 0]
        out.append(rect("side", c, 0, FOOT_H, X_SIDE, TOP_Y0, z=5, tone=0.88, **CAS))
        out.append(rect("hob-edge", c, 0, TOP_Y0 if scale == "50" else 0.835, TOP_X1, TOP_Y1, z=5.3, tone=0.82, region="hob-top", material="enamel-hob-stained"))
        out.append(rect("fascia", c, X_SIDE, FASCIA_Y0, FASCIA_X1, TOP_Y0 if scale == "50" else 0.835, z=5.5, tone=0.86, region="fascia", material="enamel-white-aged"))
        out.append(poly("knobs", c, knob_outline(KNOB if scale == "50" else KNOB_100, FASCIA_X1, KNOB_Y), role="detail", z=7, tone=0.85, region="fascia", material="abs-knob-aged"))
        cx, cy, cz = COIL
        out.append(rect("cable-coil", c, cx - COIL_T, cy - COIL_R - COIL_T, cx + COIL_T, cy + COIL_R + COIL_T, z=4.5, tone=0.3, region="casing", material="pvc-cable"))
    if scale == "50":
        for name, x, rp, ro in SIDE_ROWS:
            out.append(poly(f"plates-{name}", c, plate_row(x, TOP_Y1, rp, ro, scale), z=5.2, tone=0.12, region="hotplates", material="cast-iron-aged"))
    else:
        out.append(plate_band("plates", c, SIDE_ROWS, TOP_Y1, scale))
    if scale == "50":
        out.append(rect("price-sticker", c, 0.40, 0.70, 0.46, 0.74, role="detail", z=6, tone=0.95))
        out.append(circle("dent", c, 0.25, 0.32, 0.03, role="texture", z=6, tone=0.3, filled=False))
        out.append(tex("scuff-low", c, [(0.06, 0.05), (0.20, 0.055), (0.34, 0.048)], z=6))
        out.append(tex("rust-foot", c, [(0.50, 0.018), (0.515, 0.03), (0.53, 0.02)], z=6))
        out.append(tex("grease-run", c, [(0.53, 0.84), (0.535, 0.78), (0.53, 0.72)], z=6))
        out.append(tex("tape-on-coil", c, [(-0.024, 0.40), (0.0, 0.40)], z=6))
    if scale == "100":
        out.append(rect("price-sticker", c, 0.40, 0.70, 0.46, 0.74, role="detail", z=6, tone=0.95))
    dr = "drawer"
    if scale == "200":
        out.append(rect("drawer-front", dr, 0.55, 0.03, DRAWER_X1, DRAWER_Y1, z=5.6, tone=0.86, region="drawer", material="enamel-white-aged"))
        out.append(rect("drawer-box", dr, 0.0, 0.03, 0.55, DRAWER_Y1, z=3, tone=0.65, material="steel-galvanised"))
        out.append(rect("tray", "tray", TRAY_X0, 0.435, TRAY_X1, 0.465, z=2.5, tone=0.25, material="enamel-oven-dark"))
    else:
        out.append(rect("drawer-front", dr, DRAWER_X0, DRAWER_Y0, DRAWER_X1, DRAWER_Y1, z=5.6, tone=0.86, region="drawer", material="enamel-white-aged"))
        out.append(rect("drawer-box", dr, BOX_X0, BOX_Y0 if scale == "50" else 0.031, DRAWER_X0, BOX_Y1, z=3, tone=0.65, material="steel-galvanised"))
        out.append(rect("tray", "tray", TRAY_X0, TRAY_Y0, TRAY_X1, TRAY_Y0 + (TRAY_H if scale == "50" else 0.016), z=2.5, tone=0.25, material="enamel-oven-dark"))
    return out + door_elevation(scale)


def front(scale):
    """🪟 Front elevation from ahead (paper x = lateral, the asset's left on the right): pallet ends, feet, casing, drawer, door with window and handle, fascia with knobs and lamps, hob edge and plate columns."""
    b, PAL = "pallet", {"region": "pallet", "material": "spruce-pallet"}
    out = []
    if scale == "200":
        out += [rect(f"pallet-block-{i + 1}", b, z0, 0, z1, BLOCK_Y1, z=1, tone=0.45, **PAL) for i, (z0, z1) in enumerate(ROW_Z)]
        out.append(rect("pallet-deck", "pallet-deck", -PALLET_Z, BLOCK_Y1, PALLET_Z, PALLET_H, z=1.5, tone=0.6, **PAL))
    else:
        for i, (z0, z1) in enumerate(ROW_Z):
            out.append(rect(f"pallet-bottom-{i + 1}", b, z0, 0, z1, BOARD_T, z=1, tone=0.55, **PAL))
            out.append(rect(f"pallet-block-{i + 1}", b, z0, BOARD_T, z1, BLOCK_Y1, z=1, tone=0.45, **PAL))
            out.append(rect(f"pallet-stringer-{i + 1}", b, z0, BLOCK_Y1, z1, STRINGER_Y1, z=1.2, tone=0.55, **PAL))
        out.append(rect("pallet-top-front", "pallet-deck", -PALLET_Z, STRINGER_Y1, PALLET_Z, PALLET_H, z=1.4, tone=0.6, **PAL))
        if scale == "50":
            out.append(tex("pallet-top-grain", "pallet-deck", [(-0.38, 0.133), (0.0, 0.134), (0.38, 0.132)], z=1.6))
    c = "carcass"
    if scale == "200":
        out.append(rect("casing", c, -ZO, Y0, ZO, Y0 + 0.82, z=2, tone=0.88, region="casing", material="enamel-white-aged"))
        out.append(rect("hob-edge", c, -ZO, Y0 + 0.82, ZO, Y0 + TOP_Y1, z=5.3, tone=0.82, region="hob-top", material="enamel-hob-stained"))
        out.append(rect("fascia", c, -ZO, Y0 + FASCIA_Y0, ZO, Y0 + 0.82, z=5, tone=0.86, region="fascia", material="enamel-white-aged"))
        out.append(rect("drawer-front", "drawer", -DRAWER_Z, Y0 + 0.03, DRAWER_Z, Y0 + DRAWER_Y1, z=5, tone=0.86, region="drawer", material="enamel-white-aged"))
    else:
        out += [rect(f"foot-{i + 1}", c, zz - FOOT_R, Y0, zz + FOOT_R, Y0 + FOOT_H, z=1.8, tone=0.15, region="casing", material="pp-black") for i, (x, zz) in enumerate(FEET) if x > 0.3]
        out.append(rect("casing", c, -ZO, Y0 + FOOT_H, ZO, Y0 + TOP_Y0, z=2, tone=0.88, region="casing", material="enamel-white-aged"))
        out.append(rect("hob-edge", c, -ZO, Y0 + (TOP_Y0 if scale == "50" else 0.835), ZO, Y0 + TOP_Y1, z=5.3, tone=0.82, region="hob-top", material="enamel-hob-stained"))
        out.append(rect("fascia", c, -ZI if scale == "50" else -ZO, Y0 + FASCIA_Y0, ZI if scale == "50" else ZO, Y0 + (TOP_Y0 if scale == "50" else 0.835), z=5, tone=0.86, region="fascia", material="enamel-white-aged"))
        out.append(rect("drawer-front", "drawer", -DRAWER_Z, Y0 + DRAWER_Y0, DRAWER_Z, Y0 + DRAWER_Y1, z=5, tone=0.86, region="drawer", material="enamel-white-aged"))
        for i, zz in enumerate(KNOB_ZS):
            old = abs(zz - REPLACED_KNOB) < 1e-9
            out.append(circle(f"knob-{i + 1}", c, zz, Y0 + KNOB_Y, 0.017 if old else 0.019, z=7, tone=0.12 if old else 0.85, material="pp-black" if old else "abs-knob-aged"))
        if scale == "50":
            for i, zz in enumerate(LAMP_ZS):
                out.append(circle(f"lamp-{i + 1}", c, zz, Y0 + KNOB_Y, 0.005, z=7, tone=0.55))
            for i, zz in enumerate(KNOB_ZS):
                out.append(tex(f"knob-mark-{i + 1}", c, [(zz, Y0 + KNOB_Y + 0.004), (zz, Y0 + KNOB_Y + 0.016)], z=7.5, tone=0.2))
            out.append(tex("drawer-grip", "drawer", [(-0.20, Y0 + 0.180), (0.20, Y0 + 0.180)], z=6))
            out.append(tex("drawer-chip", "drawer", [(0.225, Y0 + 0.03), (0.240, Y0 + 0.038), (0.236, Y0 + 0.024), (0.225, Y0 + 0.03)], z=6, closed=True))
            out.append(tex("fascia-wear", c, [(-0.15, Y0 + 0.760), (-0.08, Y0 + 0.758), (-0.02, Y0 + 0.761)], z=6))
    if scale == "50":
        for name, zz, rp, ro in FRONT_COLS:
            out.append(poly(f"plates-{name}", c, plate_row(zz, Y0 + TOP_Y1, rp, ro, scale), z=5.2, tone=0.12, region="hotplates", material="cast-iron-aged"))
    else:
        out.append(plate_band("plates", c, FRONT_COLS, Y0 + TOP_Y1, scale))
    d = "door"
    yd = Y0 + HINGE_Y
    out.append(rect("door", d, -DOOR_Z, yd, DOOR_Z, yd + DOOR_H, z=5, tone=0.9, region="oven-door", material="enamel-white-aged"))
    out.append(rect("window", d, -WIN_W, yd + WIN_U0, WIN_W, yd + WIN_U1, role="glazing", z=5.5, tone=0.25, region="door-window", material="door-glass"))
    if scale == "200":
        out.append(rect("handle", d, -0.21, yd + HANDLE_U - 0.015, 0.21, yd + HANDLE_U + 0.015, z=6, tone=0.75, region="oven-door", material="chrome-steel"))
    else:
        out.append(sh("handle-bar", d, "capsule", "body", 6, 0.75, region="oven-door", material="chrome-steel", x0=-HANDLE_Z, y0=r6(yd + HANDLE_U), x1=HANDLE_Z, y1=r6(yd + HANDLE_U), r0=HANDLE_R, r1=HANDLE_R))
    if scale == "50":
        out.append(tex("window-scratch-1", d, [(-0.10, yd + 0.30), (-0.03, yd + 0.33)], z=6))
        out.append(tex("window-scratch-2", d, [(0.04, yd + 0.22), (0.11, yd + 0.24)], z=6))
        out.append(tex("window-grease", d, [(-0.15, yd + 0.17), (0.0, yd + 0.175), (0.15, yd + 0.168)], z=6))
        out.append(tex("door-grime", d, [(-0.24, yd + 0.012), (0.0, yd + 0.010), (0.24, yd + 0.013)], z=6))
    return out


def plan(scale):
    """🗺️ Plan from above (paper x ahead, paper y lateral): pallet boards, cable coil, hob top with plates and trim rings and the burnt-on stains, knobs and door edge in front, the handle."""
    b, PAL = "pallet", {"region": "pallet", "material": "spruce-pallet"}
    out = []
    if scale == "200":
        out.append(rect("pallet-deck", "pallet-deck", PALLET_X0, -PALLET_Z, PALLET_X0 + PALLET_L, PALLET_Z, z=1, tone=0.6, **PAL))
    else:
        out += [rect(f"pallet-stringer-{i + 1}", b, PALLET_X0, z0, PALLET_X0 + PALLET_L, z1, z=1, tone=0.55, **PAL) for i, (z0, z1) in enumerate(ROW_Z)]
        out += [rect(f"pallet-top-{i + 1}", "pallet-deck", PALLET_X0 + x0, -PALLET_Z, PALLET_X0 + x1, PALLET_Z, z=1.4, tone=0.6, **PAL) for i, (x0, x1) in enumerate(TOP_BOARDS if scale == "50" else TOP_BOARDS_100)]
        if scale == "50":
            out.append(tex("pallet-nail-row", "pallet-deck", [(PALLET_X0 + 0.07, -0.39), (PALLET_X0 + 0.07, -0.30)], z=1.6))
    c = "carcass"
    if scale != "200":
        cx, cy, cz = COIL
        out.append(rect("cable-coil", c, cx - COIL_T, cz - COIL_R - COIL_T, cx + COIL_T, cz + COIL_R + COIL_T, z=4.5, tone=0.3, region="casing", material="pvc-cable"))
    d = "door"
    out.append(rect("door", d, X_SIDE, -DOOR_Z, HINGE_X, DOOR_Z, z=4, tone=0.9, region="oven-door", material="enamel-white-aged"))
    if scale == "200":
        out.append(rect("handle", d, HINGE_X, -0.21, HINGE_X + 0.045, 0.21, z=4.5, tone=0.75, region="oven-door", material="chrome-steel"))
    else:
        for side, sgn in (("r", -1), ("l", 1)):
            out.append(rect(f"standoff-{side}", d, HINGE_X, sgn * STANDOFF_Z - 0.008, HINGE_X - HANDLE_V, sgn * STANDOFF_Z + 0.008, role="detail", z=4.4, tone=0.7, region="oven-door", material="chrome-steel"))
        out.append(sh("handle-bar", d, "capsule", "body", 4.5, 0.75, region="oven-door", material="chrome-steel", x0=r6(HINGE_X - HANDLE_V), y0=-HANDLE_Z, x1=r6(HINGE_X - HANDLE_V), y1=HANDLE_Z, r0=HANDLE_R, r1=HANDLE_R))
        for i, zz in enumerate(KNOB_ZS):
            old = abs(zz - REPLACED_KNOB) < 1e-9
            prof = KNOB_OLD if old else (KNOB if scale == "50" else KNOB_100)
            out.append(poly(f"knob-{i + 1}", c, knob_outline(prof, FASCIA_X1, zz), role="detail", z=6, tone=0.12 if old else 0.85, region="fascia", material="pp-black" if old else "abs-knob-aged"))
    out.append(rect("hob-top", c, 0, -ZO, TOP_X1, ZO, z=5, tone=0.82, region="hob-top", material="enamel-hob-stained"))
    for key, x, zz, rp, ro, _, _ in PLATES:
        if scale == "200":
            out.append(circle(f"plate-{key}", c, x, zz, ro, role="body", z=5.6, tone=0.12, region="hotplates", material="cast-iron-aged"))
            continue
        out.append(circle(f"ring-{key}", c, x, zz, ro, role="body", z=5.5, tone=0.8, region="hotplates", material="stainless-dulled"))
        out.append(circle(f"plate-{key}", c, x, zz, rp, role="detail", z=5.6, tone=0.12, region="hotplates", material="cast-iron-aged"))
        if scale == "50":
            out.append(circle(f"plate-centre-{key}", c, x, zz, 0.010, role="detail", z=5.7, tone=0.3))
            out.append(ring(f"plate-groove-{key}", c, x, zz, rp * 0.6, z=5.8, tone=0.25))
    if scale == "50":
        out.append(circle("rapid-dot", c, 0.16, 0.12, 0.006, role="detail", z=5.9, tone=0.45))
        out.append(tex("stain-fl", c, [(0.30, 0.07), (0.33, 0.02), (0.40, 0.006), (0.47, 0.02), (0.52, 0.06), (0.53, 0.13), (0.50, 0.21), (0.44, 0.225), (0.36, 0.22), (0.31, 0.17)], closed=True, z=5.8))
        out.append(tex("stain-rr", c, [(0.055, -0.10), (0.08, -0.21), (0.16, -0.235), (0.24, -0.21), (0.27, -0.13), (0.25, -0.05), (0.19, -0.015), (0.12, -0.02)], closed=True, z=5.8))
        out.append(tex("spill-front", c, [(0.53, -0.02), (0.55, -0.05), (0.56, -0.09)], z=5.8))
        out.append(ring("rust-ring-fr", c, 0.41, -0.12, 0.074, z=5.8, tone=0.35))
        out.append(rect("vent-slot", c, 0.02, -0.15, 0.05, 0.15, role="detail", z=5.8, tone=0.3))
        out.append(rect("price-tag", c, 0.50, -0.235, 0.555, -0.195, role="detail", z=5.9, tone=0.95))
        out.append(tex("chip-front-edge", c, [(0.565, 0.18), (0.57, 0.19), (0.565, 0.20)], z=5.8))
        out.append(tex("tape-coil", c, [(-0.024, 0.10), (0.0, 0.10)], z=5))
    if scale == "100":
        out.append(rect("vent-slot", c, 0.02, -0.15, 0.05, 0.15, role="detail", z=5.8, tone=0.3))
    return out


PENS = {
    "50": {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": 0.25, "ground": 0.5},
    "100": {"body": 0.25, "detail": 0.18, "texture": 0.13, "glazing": 0.25, "ground": 0.35},
    "200": {"body": 0.25, "detail": 0.18, "texture": None, "glazing": 0.25, "ground": 0.25},
}
RANGES = {"50": (20, 75), "100": (75, 150), "200": (150, 350)}

REP_DECISIONS = {
    "50": {
        "silhouette": T("Cooker 11.4 x 17.1 mm in the side view on the 2.9 mm half pallet: the right side panel on two adjustable feet, the hob edge with the two rows of hotplates standing 7 mm proud, the fascia and its knobs, the drawer front, the door edge and the bar handle 45 mm in front of it, the cable coil behind the back. Front: drawer, door with its window and handle bar, fascia with six knobs and two pilot lamps, the plate columns on the hob edge, the pallet ends. Plan: hob with two 180 mm and two 145 mm plates in their trim rings over the four top boards.", "Herd 11,4 x 17,1 mm in der Seitenansicht auf der 2,9 mm hohen Halbpalette: rechte Seitenwand auf zwei Stellfüßen, die Muldenkante mit den zwei Reihen Kochplatten, die 7 mm vorstehen, die Schalterblende mit Knebeln, die Schubladenfront, die Türkante und der Stangengriff 45 mm davor, das Kabelbündel hinter der Rückwand. Vorderansicht: Schublade, Tür mit Sichtfenster und Griffstange, Blende mit sechs Knebeln und zwei Kontrolllampen, die Plattenspalten auf der Muldenkante, die Palettenstirnseiten. Grundriss: Mulde mit zwei 180-mm- und zwei 145-mm-Platten in ihren Abdeckringen über den vier Deckbrettern."),
        "retained": T("True sheet gauges in the outline, the 2 mm dished plate centres, the red dot of the rapid plate, the black replacement knob on the second hotplate among five yellowed white ones, burnt-on stains around two plates, a rust ring on the front right plate, a chipped front edge, scratches and a grease film on the door glass, a dent and scuffs in the side panel, the depot price sticker and tag, the taped cable coil, the pallet stamp and a split block.", "Wahre Blechstärken im Umriss, die 2 mm tiefen Plattenmitten, der rote Punkt der Blitzkochplatte, der schwarze Ersatzknebel der zweiten Kochplatte zwischen fünf vergilbten weißen, eingebrannte Flecken um zwei Platten, ein Rostring auf der Platte vorn rechts, eine abgeplatzte Vorderkante, Kratzer und ein Fettfilm auf dem Türglas, eine Delle und Schrammen in der Seitenwand, Preisaufkleber und Preisschild des Lagers, das abgeklebte Kabelbündel, der Palettenstempel und ein gespaltener Klotz."),
        "merged": T("Each knob row reads as one silhouette in the side view and the two plates of a row as one plate profile; the fascia return flanges are merged into the fascia.", "Jede Knebelreihe ist in der Seitenansicht ein Umriss, die zwei Platten einer Reihe ein Plattenprofil; die Abkantungen der Blende gehen in der Blende auf."),
        "omitted": T("Screws, the oven lamp, the door hinges inside the frame, the bottom heating element, wiring and the anti-tilt bracket.", "Schrauben, die Backofenlampe, die Türscharniere im Rahmen, der Unterhitzeheizkörper, Verdrahtung und der Kippschutzwinkel."),
        "thinElements": T("The 0.7 mm side panels and the 1 mm hob plate are read by their outlines; the 1.5 mm gap between drawer and door is drawn at true size.", "Die 0,7-mm-Seitenwände und das 1-mm-Muldenblech wirken über ihre Konturen; die 1,5-mm-Fuge zwischen Schublade und Tür ist in wahrer Größe gezeichnet."),
        "hatching": T("None; enamel, cast iron, glass, plastics and spruce are told apart by tone.", "Keine; Emaille, Gusseisen, Glas, Kunststoffe und Fichte werden durch Ton unterschieden."),
        "styles": T("Yellowed enamel 0.88, stained hob 0.82, cast-iron plates 0.12, trim rings 0.8, glass 0.25, knobs 0.85 and the black replacement 0.12, pallet 0.45 to 0.6; traces of use as 0.13 mm texture strokes.", "Vergilbte Emaille 0,88, fleckige Mulde 0,82, Gussplatten 0,12, Abdeckringe 0,8, Glas 0,25, Knebel 0,85 und der schwarze Ersatz 0,12, Palette 0,45 bis 0,6; Gebrauchsspuren als 0,13-mm-Texturstriche."),
        "overlays": T("Door, drawer and baking tray are drawn on their own bones, drawer box and tray behind the side panel, so an opened door, a pulled drawer or tray reads in every view; the pallet stays on the floor.", "Tür, Schublade und Backblech sind auf eigenen Knochen gezeichnet, Schubladenkasten und Blech hinter der Seitenwand, so liest sich eine geöffnete Tür, eine gezogene Schublade oder ein gezogenes Blech in jeder Ansicht; die Palette bleibt am Boden."),
    },
    "100": {
        "silhouette": T("Cooker 5.7 x 8.6 mm on the pallet: side panel on its feet, hob edge 15 mm, simplified plate rows, fascia and knob row, drawer and door edge with the handle; front with drawer, door, window, handle, knobs; plan with plates in their rings.", "Herd 5,7 x 8,6 mm auf der Palette: Seitenwand auf den Füßen, Muldenkante 15 mm, vereinfachte Plattenreihen, Blende und Knebelreihe, Schublade und Türkante mit Griff; Vorderansicht mit Schublade, Tür, Fenster, Griff, Knebeln; Grundriss mit Platten in ihren Ringen."),
        "retained": T("All pallet boards and blocks, the six knobs with the black replacement, the plates and rings, the handle on its stand-offs, the cable coil, the price sticker and the vent slot.", "Alle Palettenbretter und -klötze, die sechs Knebel mit dem schwarzen Ersatz, Platten und Ringe, der Griff auf seinen Stützen, das Kabelbündel, der Preisaufkleber und der Wrasenschlitz."),
        "merged": T("Hob plate and the casing below into a 15 mm hob edge; plate and ring into one profile; knobs as three-point profiles.", "Muldenblech und Gehäuse darunter in eine 15-mm-Muldenkante; Platte und Ring in ein Profil; Knebel als Dreipunktprofile."),
        "omitted": T("Stains, rust ring, scratches, dent, scuffs, the pilot lamps, knob marks and the pallet grain.", "Flecken, Rostring, Kratzer, Delle, Schrammen, die Kontrolllampen, Knebelmarken und die Palettenmaserung."),
        "thinElements": T("Side sheets and gaps are carried by the outline.", "Seitenbleche und Fugen werden von der Kontur getragen."),
        "hatching": T("None.", "Keine."),
        "styles": T("Tones as 1:50, body pen 0.25 mm.", "Töne wie 1:50, Körperstift 0,25 mm."),
        "overlays": T("Door, drawer and tray on their own bones.", "Tür, Schublade und Blech auf eigenen Knochen."),
    },
    "200": {
        "silhouette": T("Cooker 2.9 x 4.3 mm on the pallet: one casing outline with the 30 mm hob edge and the thin plate band, fascia, drawer front, door edge and the handle block; front with the four fields of drawer, door with window, fascia and hob edge; plan with four plate discs.", "Herd 2,9 x 4,3 mm auf der Palette: ein Gehäuseumriss mit 30 mm Muldenkante und dem schmalen Plattenband, Blende, Schubladenfront, Türkante und Griffblock; Vorderansicht mit den vier Feldern Schublade, Tür mit Fenster, Blende und Muldenkante; Grundriss mit vier Plattenscheiben."),
        "retained": T("Overall 0.50 x 0.60 x 0.86 m, the 7 mm the plates stand proud, the door window, the handle as a block, the pallet deck and its blocks.", "Gesamtmaß 0,50 x 0,60 x 0,86 m, die 7 mm Plattenüberstand, das Türfenster, der Griff als Block, Palettendeck und Klötze."),
        "merged": T("Feet, base and drawer bay into the lower casing; plate and ring into one disc; top boards and stringers of the pallet into one deck, blocks with their bottom boards.", "Füße, Boden und Schubkastenfach in das untere Gehäuse; Platte und Ring in eine Scheibe; Deckbretter und Querbretter der Palette in ein Deck, Klötze mit ihren Bodenbrettern."),
        "omitted": T("Knobs (27 mm), pilot lamps, cable coil (24 mm), stand-offs and every trace of use: they are under a readable line here.", "Knebel (27 mm), Kontrolllampen, Kabelbündel (24 mm), Griffstützen und alle Gebrauchsspuren: sie liegen hier unter einer lesbaren Linie."),
        "thinElements": T("The 7 mm plate band and the 30 mm hob edge are carried by the pen.", "Das 7-mm-Plattenband und die 30-mm-Muldenkante werden vom Stift getragen."),
        "hatching": T("None.", "Keine."),
        "styles": T("Body pen 0.25 mm, tones as 1:50.", "Körperstift 0,25 mm, Töne wie 1:50."),
        "overlays": T("Door, drawer and tray on their own bones.", "Tür, Schublade und Blech auf eigenen Knochen."),
    },
}


def representation(scale):
    """🖨️ One authored representation."""
    lo, hi = RANGES[scale]
    return {"status": "authored", "denominator": int(scale), "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[scale], "hatches": [], "geometry": {"elevation": elevation(scale), "front": front(scale), "plan": plan(scale)}, "decisions": REP_DECISIONS[scale]}


UNAVAILABLE_500 = {
    "status": "unavailable",
    "denominator": 500,
    "cause": "below-printable-feature",
    "reason": T("At 1:500 the cooker on its pallet is 1.3 by 2.0 mm on paper, under the 3 mm a recognisable drawing needs; its 40 mm door would be 0.08 mm and the 7 mm hotplates 0.014 mm, far under the 0.2 mm office-laser minimum feature; draw it at 1:200 or use the symbol used-electric-cooker-symbol.", "Bei 1:500 misst der Herd auf seiner Palette 1,3 mal 2,0 mm auf dem Papier, unter den 3 mm, die eine erkennbare Zeichnung braucht; seine 40-mm-Tür wäre 0,08 mm und die 7-mm-Kochplatten 0,014 mm, weit unter dem Mindestmerkmal von 0,2 mm des Büro-Laserdrucks; bei 1:200 zeichnen oder das Symbol used-electric-cooker-symbol verwenden."),
    "limitingFeature": T("Oven door of 40 mm", "Backofentür von 40 mm"),
    "limitingSizeM": 0.04,
    "alternatives": [{"kind": "scale", "denominator": 200}, {"kind": "symbol", "symbol": "used-electric-cooker-symbol"}],
}


def symbol():
    """🔣 The cooker on its pallet as a true-extent pictogram for 1:500 and 1:200 plans and sections."""
    g = "floor"
    S = {"role": "body", "filled": True}

    def p(sid, z, tone, ring_):
        """🔲 One symbol polygon."""
        return {"id": sid, "bone": g, "z": z, "tone": tone, "kind": "poly", "points": pts(ring_), "closed": True, **S}

    return {
        "id": "used-electric-cooker-symbol",
        "title": T("Electric cooker on its pallet, true extent", "Elektroherd auf seiner Palette, wahre Ausdehnung"),
        "description": T("The 50 cm cooker as a pictogram in three views: casing with door and hob on the half pallet from the side, the door field and hob edge from the front, the hob with its four plate squares over the pallet in plan.", "Der 50-cm-Herd als Piktogramm in drei Ansichten: Gehäuse mit Tür und Mulde auf der Halbpalette von der Seite, Türfeld und Muldenkante von vorn, die Mulde mit vier Plattenquadraten über der Palette im Grundriss."),
        "semantics": "true-extent",
        "units": "model-m",
        "denominators": [500, 200],
        "pens": {"body": 0.25, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.35},
        "hatches": [],
        "geometry": {
            "elevation": [p("sym-pallet", 1, 0.5, [(PALLET_X0, 0), (PALLET_X0 + PALLET_L, 0), (PALLET_X0 + PALLET_L, Y0), (PALLET_X0, Y0)]), p("sym-cooker", 2, 0.88, [(0, Y0), (HINGE_X, Y0), (HINGE_X, Y0 + HINGE_Y + HANDLE_U - 0.01), (HINGE_X + 0.045, Y0 + HINGE_Y + HANDLE_U - 0.01), (HINGE_X + 0.045, Y0 + HINGE_Y + HANDLE_U + 0.01), (HINGE_X, Y0 + HINGE_Y + HANDLE_U + 0.01), (HINGE_X, Y0 + 0.715), (FASCIA_X1, Y0 + 0.715), (FASCIA_X1, Y0 + TOP_Y1), (0, Y0 + TOP_Y1), (0, Y0 + COIL[1] + COIL_R + COIL_T), (-2 * COIL_T, Y0 + COIL[1] + COIL_R + COIL_T), (-2 * COIL_T, Y0 + COIL[1] - COIL_R - COIL_T), (0, Y0 + COIL[1] - COIL_R - COIL_T)])],
            "front": [p("sym-pallet", 1, 0.5, [(-PALLET_Z, 0), (PALLET_Z, 0), (PALLET_Z, Y0), (-PALLET_Z, Y0)]), p("sym-cooker", 2, 0.88, [(-ZO, Y0), (ZO, Y0), (ZO, Y0 + TOP_Y1), (-ZO, Y0 + TOP_Y1)]), {"id": "sym-door", "bone": g, "z": 3, "tone": 0.5, "kind": "poly", "points": pts([(-DOOR_Z, Y0 + HINGE_Y), (DOOR_Z, Y0 + HINGE_Y), (DOOR_Z, Y0 + 0.715), (-DOOR_Z, Y0 + 0.715)]), "closed": True, "role": "detail", "filled": False}],
            "plan": [p("sym-pallet", 1, 0.5, [(PALLET_X0, -PALLET_Z), (PALLET_X0 + PALLET_L, -PALLET_Z), (PALLET_X0 + PALLET_L, PALLET_Z), (PALLET_X0, PALLET_Z)]), p("sym-cooker", 2, 0.82, [(0, -ZO), (HINGE_X, -ZO), (HINGE_X, -0.21), (HINGE_X + 0.045, -0.21), (HINGE_X + 0.045, 0.21), (HINGE_X, 0.21), (HINGE_X, ZO), (0, ZO), (0, COIL[2] + COIL_R + COIL_T), (-2 * COIL_T, COIL[2] + COIL_R + COIL_T), (-2 * COIL_T, COIL[2] - COIL_R - COIL_T), (0, COIL[2] - COIL_R - COIL_T)])] + [{"id": f"sym-plate-{k}", "bone": g, "z": 3, "tone": 0.2, "kind": "poly", "points": pts([(x - ro, zz - ro), (x + ro, zz - ro), (x + ro, zz + ro), (x - ro, zz + ro)]), "closed": True, "role": "detail", "filled": True} for k, x, zz, rp, ro, _, _ in PLATES],
        },
        "dimensionalNote": T("Model metres at real size in the frame of the asset (x ahead, lateral positive to the left, y up), stored pose: cooker 0.60 m deep with its door, 0.50 m wide and 0.85 m to the hob on the 0.144 m half pallet of 0.60 x 0.80 m.", "Modellmeter in wahrer Größe im Koordinatensystem des Objekts (x nach vorn, seitlich positiv nach links, y nach oben), Lagerpose: Herd mit Tür 0,60 m tief, 0,50 m breit und 0,85 m bis zur Mulde auf der 0,144 m hohen Halbpalette von 0,60 x 0,80 m."),
    }
