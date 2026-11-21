"""🚧 Round-3 kit of `objects/retractable-bollard`: the real dimensions of the electro-hydraulic rising bollard and every solid of every scale and list, stated by hand in world coordinates (X ahead, Y up, Z left, ground at Y = 0, shaft in its lowered rest) and carried into the bone frames (u up, v = -X, w = Z).

@see ./r5-retractable-bollard-model.py
@see ./r5-retractable-bollard-roots.py
@see ./r3-objects-retractable-bollard.md
"""
import math

STEEL, FILM, PC, IRON, GALV, HYD = "steel-grey", "retro-film", "pc-amber", "cast-iron-frame", "steel-sleeve", "steel-hyd"
TONE = {STEEL: 0.5, FILM: 0.92, PC: 0.7, IRON: 0.45, GALV: 0.4, HYD: 0.55}
ORIGIN = {"sleeve": -1.25, "shaft": -1.15, "hydraulic": -1.242}

DEPTH = 1.25
STROKE = 0.65
SHAFT_R, SHAFT_WALL, SHAFT_L = 0.1, 0.005, 1.15
SLEEVE_R, SLEEVE_WALL = 0.15, 0.008
FRAME_HALF, FRAME_TOP, FRAME_BOTTOM, COLLAR_R, COLLAR_BOTTOM = 0.2, 0.015, -0.01, 0.168, -0.07
BAND_U0, BAND_U1, LENS_U1, SKIRT_U1 = 0.91, 1.02, 1.08, 1.1
CYL_R, CYL_BORE, ROD_R = 0.035, 0.03, 0.018
CYL_Y0, CYL_Y1, GLAND_Y1 = -1.197, -0.31, -0.27
UNIT_X0, UNIT_X1, UNIT_Z0, UNIT_Z1, UNIT_Y0, UNIT_Y1 = 0.17, 0.36, 0.012, 0.2, -0.95, -0.55
HOSE = [(0.04, -1.213, 0.0), (0.15, -1.213, 0.0), (0.215, -1.213, 0.03), (0.255, -1.19, 0.08), (0.265, -1.13, 0.106), (0.265, -0.95, 0.106)]
BRACKET = [(0.1443, 0.022), (0.18, 0.022), (0.18, 0.19), (0.073, 0.1265), (0.1014, 0.105), (0.125, 0.0752), (0.1372, 0.0499)]


def r6(x):
    """🔢 Six decimals, no negative zero."""
    y = round(float(x), 6)
    return 0.0 if y == 0 else y


def dome(n=10):
    """🛎 The crown of the head as (u, radius): a cubic from the skirt edge (1.10, 0.10) to the apex (1.15, 0)."""
    p0, p1, p2, p3 = (SKIRT_U1, 0.1), (1.135, 0.1), (1.15, 0.06), (SHAFT_L, 0.0)
    out = []
    for i in range(n + 1):
        t = i / n
        a, b, c, d = (1 - t) ** 3, 3 * t * (1 - t) ** 2, 3 * t * t * (1 - t), t ** 3
        out.append((a * p0[0] + b * p1[0] + c * p2[0] + d * p3[0], a * p0[1] + b * p1[1] + c * p2[1] + d * p3[1]))
    return out


def circle(r, n, cx=0.0, cz=0.0, phase=0.0):
    """⭕ A polygon of n points on a circle in plan (X, Z)."""
    return [(cx + r * math.cos(phase + 2 * math.pi * k / n), cz + r * math.sin(phase + 2 * math.pi * k / n)) for k in range(n)]


def rounded(x0, x1, z0, z1, r, n=4):
    """🔲 A rounded rectangle in plan (X, Z)."""
    out = []
    for cx, cz, a0 in ((x1 - r, z1 - r, 0), (x0 + r, z1 - r, 90), (x0 + r, z0 + r, 180), (x1 - r, z0 + r, 270)):
        for k in range(n + 1):
            a = math.radians(a0 + 90 * k / n)
            out.append((cx + r * math.cos(a), cz + r * math.sin(a)))
    return out


def base(sid, kind, bone, material, role="body", tone=None, **more):
    """🏷 Common members of a solid."""
    out = {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": TONE[material] if tone is None else tone}
    out.update(more)
    return out


def rev(sid, bone, material, profile, x=0.0, z=0.0, closed=False, **more):
    """🌀 A revolve about the vertical through (X, Z); profile is (world Y of the rest pose, radius)."""
    o = ORIGIN[bone]
    pts = [[r6(y - o), r6(r)] for y, r in profile]
    if closed:
        pts.append(pts[0])
    return base(sid, "revolve", bone, material, profile=pts, axis=[1, 0, 0], origin=[0, r6(-x), r6(z)], **more)


def hrev(sid, bone, material, profile, x, y, z, direction=(1.0, 0.0), closed=False, **more):
    """➡️ A revolve about a horizontal axis from (X, Y, Z) along direction (dX, dZ); profile is (distance, radius)."""
    pts = [[r6(a), r6(r)] for a, r in profile]
    if closed:
        pts.append(pts[0])
    return base(sid, "revolve", bone, material, profile=pts, axis=[0, r6(-direction[0]), r6(direction[1])], origin=[r6(y - ORIGIN[bone]), r6(-x), r6(z)], **more)


def slab(sid, bone, material, plan, y0, y1, holes=None, **more):
    """🧱 A plan polygon (X, Z) extruded vertically from Y0 to Y1."""
    o = ORIGIN[bone]
    out = base(sid, "extrude", bone, material, profile=[[r6(-x), r6(z)] for x, z in plan], plane="yz", **{"from": r6(y0 - o), "to": r6(y1 - o)})
    if holes:
        out["holes"] = [[[r6(-x), r6(z)] for x, z in h] for h in holes]
    out.update(more)
    return out


def box(sid, bone, material, x0, x1, y0, y1, z0, z1, radius=0.0, **more):
    """📦 A box between world corners."""
    out = base(sid, "box", bone, material, size=[r6(y1 - y0), r6(x1 - x0), r6(z1 - z0)])
    if radius:
        out["cornerRadius"] = r6(radius)
    out["transform"] = {"translation": [r6((y0 + y1) / 2 - ORIGIN[bone]), r6(-(x0 + x1) / 2), r6((z0 + z1) / 2)]}
    out.update(more)
    return out


def tube(sid, bone, material, path, r, cap=True, **more):
    """〰️ A round loft along world points (X, Y, Z)."""
    o = ORIGIN[bone]
    return base(sid, "loft", bone, material, sections=[{"at": [r6(y - o), r6(-x), r6(z)], "rx": r6(r), "ry": r6(r)} for x, y, z in path], capStart=cap, capEnd=cap, **more)


def ball(sid, bone, material, x, y, z, r, **more):
    """🔮 A sphere at a world point."""
    return base(sid, "ellipsoid", bone, material, radii=[r6(r)] * 3, transform={"translation": [r6(y - ORIGIN[bone]), r6(-x), r6(z)]}, **more)


def turn(count):
    """🔁 Copies about the vertical axis of the bone."""
    return {"count": count, "rotationAxis": [1, 0, 0], "rotationRad": r6(2 * math.pi / count), "origin": [0, 0, 0]}


def su(u):
    """⬆️ World Y of the lowered rest pose for a height u above the shaft base."""
    return u + ORIGIN["shaft"]


def th(sid, of, region, **more):
    """🌡 A thermal reference to a body solid."""
    return {"id": sid, "of": of, "region": region, **more}


def st(sid, of, member, part, **more):
    """🏗 A structural reference to a body solid."""
    return {"id": sid, "of": of, "member": member, "part": part, **more}


def skin(sid, bone, material, r, y0, y1, region):
    """🧻 An open cylindrical skin: the outer surface of a tube between two heights."""
    out = tube(sid, bone, material, [(0, y0, 0), (0, y1, 0)], r, cap=False)
    out["region"] = region
    return out


def text(en, de):
    """🗣 A bilingual text, English first."""
    return {"en": en, "de": de}


def frame_plan(half=FRAME_HALF, hole=SLEEVE_R - SLEEVE_WALL, n=48):
    """🔳 The cover plate in plan: rounded square and round opening."""
    return rounded(-half, half, -half, half, 0.02), [circle(hole, n)]


def hexagon(x, z, across=0.019):
    """⬡ A hexagon across flats in plan."""
    return circle(across / math.sqrt(3), 6, x, z)


def body50():
    """🧊 Body at 1:50: every assembly of the real bollard."""
    plate, opening = frame_plan()
    crown = [(su(LENS_U1), 0.0), (su(LENS_U1), 0.1), (su(1.097), 0.1), (su(1.097), 0.0975), (su(SKIRT_U1), 0.0975)] + [(su(u), r) for u, r in dome(12)]
    return [
        rev("sleeve-tube", "sleeve", GALV, [(-1.242, 0.142), (-1.242, 0.15), (FRAME_BOTTOM, 0.15), (FRAME_BOTTOM, 0.142)], closed=True),
        slab("sleeve-floor", "sleeve", GALV, circle(0.15, 48), -1.25, -1.242, holes=[circle(0.015, 12, 0.115 * math.cos(a), 0.115 * math.sin(a)) for a in (0, 2 * math.pi / 3, 4 * math.pi / 3)]),
        box("anchor-tab-upper", "sleeve", GALV, 0.146, 0.19, -0.34, -0.26, -0.004, 0.004, role="detail", repeat=turn(4)),
        box("anchor-tab-lower", "sleeve", GALV, 0.146, 0.19, -1.14, -1.06, -0.004, 0.004, role="detail", repeat=turn(4)),
        rev("guide-bushing", "sleeve", GALV, [(COLLAR_BOTTOM, 0.102), (COLLAR_BOTTOM, 0.142), (FRAME_BOTTOM, 0.142), (FRAME_BOTTOM, 0.102)], closed=True, role="detail", tone=0.6),
        rev("buffer", "sleeve", IRON, [(-1.242, 0.0), (-1.242, 0.02), (-1.158, 0.02), (-1.15, 0.014), (-1.15, 0.0)], x=-0.117, role="detail", tone=0.15, repeat=turn(3)),
        rev("frame-collar", "sleeve", IRON, [(COLLAR_BOTTOM, 0.15), (COLLAR_BOTTOM, COLLAR_R), (FRAME_BOTTOM, COLLAR_R), (FRAME_BOTTOM, 0.15)], closed=True),
        slab("frame-plate", "sleeve", IRON, plate, FRAME_BOTTOM, 0.0, holes=opening),
        slab("frame-top", "sleeve", IRON, plate, 0.0, FRAME_TOP, holes=opening, taperTo=[1, 0.97]),
        slab("frame-bolt", "sleeve", IRON, hexagon(0.17, 0.17), FRAME_TOP, 0.023, role="detail", tone=0.2, repeat=turn(4)),
        rev("seal-retainer", "sleeve", GALV, [(0.005, 0.112), (0.005, 0.137), (FRAME_TOP, 0.137), (FRAME_TOP, 0.112)], closed=True, role="detail", tone=0.55),
        rev("wiper-seal", "sleeve", IRON, [(-0.008, 0.1005), (-0.008, 0.112), (FRAME_TOP, 0.112), (FRAME_TOP, 0.1005)], closed=True, role="detail", tone=0.12),
        rev("shaft-tube", "shaft", STEEL, [(su(0.012), 0.095), (su(0.012), 0.1), (su(BAND_U1), 0.1), (su(BAND_U1), 0.095)], closed=True),
        rev("shaft-flange", "shaft", STEEL, [(su(0), 0.095), (su(0), 0.14), (su(0.012), 0.14), (su(0.012), 0.095)], closed=True, role="detail", tone=0.4),
        rev("reflector", "shaft", FILM, [(su(BAND_U0), 0.1), (su(BAND_U0), 0.1005), (su(BAND_U1), 0.1005), (su(BAND_U1), 0.1)], closed=True),
        rev("head-core", "shaft", STEEL, [(su(0.99), 0.0), (su(0.99), 0.095), (su(BAND_U1), 0.095), (su(BAND_U1), 0.085), (su(LENS_U1), 0.085), (su(LENS_U1), 0.0)], tone=0.4),
        rev("lens", "shaft", PC, [(su(BAND_U1), 0.097), (su(BAND_U1), 0.1), (su(LENS_U1), 0.1), (su(LENS_U1), 0.097)], closed=True),
        hrev("led", "shaft", PC, [(0.088, 0.0), (0.088, 0.01), (0.1005, 0.01), (0.1005, 0.0)], 0, su(1.05), 0, role="detail", tone=0.1, repeat=turn(12)),
        rev("head-crown", "shaft", STEEL, crown),
        rev("cap-screw", "shaft", STEEL, [(su(1.135), 0.0), (su(1.135), 0.008), (su(1.1475), 0.008), (su(1.1495), 0.004), (su(1.1495), 0.0)], x=-0.05, role="detail", tone=0.2, repeat=turn(3)),
        rev("crosshead", "shaft", STEEL, [(su(0.925), 0.0), (su(0.925), 0.03), (su(0.94), 0.03), (su(0.94), 0.095), (su(0.952), 0.095), (su(0.952), 0.0)], role="detail", tone=0.4),
        slab("rod-nut", "shaft", HYD, hexagon(0, 0, 0.03), su(0.952), su(0.97), role="detail", tone=0.3),
        rev("rod", "shaft", HYD, [(su(0.17), 0.0), (su(0.17), ROD_R), (su(0.925), ROD_R), (su(0.925), 0.0)], tone=0.85),
        rev("piston", "shaft", HYD, [(su(0.13), 0.0), (su(0.13), CYL_BORE), (su(0.17), CYL_BORE), (su(0.17), 0.0)], role="detail", tone=0.3),
        slab("cyl-flange", "hydraulic", HYD, rounded(-0.06, 0.06, -0.06, 0.06, 0.01), -1.242, -1.23, tone=0.4),
        slab("cyl-flange-bolt", "hydraulic", HYD, hexagon(0.045, 0.045, 0.016), -1.23, -1.222, role="detail", tone=0.2, repeat=turn(4)),
        rev("cyl-cap", "hydraulic", HYD, [(-1.23, 0.0), (-1.23, 0.04), (CYL_Y0, 0.04), (CYL_Y0, 0.0)], tone=0.4),
        rev("cyl-tube", "hydraulic", HYD, [(CYL_Y0, CYL_BORE), (CYL_Y0, CYL_R), (CYL_Y1, CYL_R), (CYL_Y1, CYL_BORE)], closed=True, tone=0.7),
        rev("cyl-gland", "hydraulic", HYD, [(CYL_Y1, ROD_R), (CYL_Y1, 0.04), (-0.278, 0.04), (GLAND_Y1, 0.034), (GLAND_Y1, ROD_R)], closed=True, tone=0.4),
        hrev("hose-port", "hydraulic", HYD, [(0.038, 0.0), (0.038, 0.014), (0.062, 0.014), (0.062, 0.0)], 0, -1.213, 0, role="detail", tone=0.3),
        tube("hose", "hydraulic", HYD, HOSE, 0.008, role="detail", tone=0.2, slender=True),
        hrev("hose-grommet", "hydraulic", HYD, [(0.136, 0.011), (0.136, 0.022), (0.158, 0.022), (0.158, 0.011)], 0, -1.213, 0, closed=True, role="detail", tone=0.15),
        rev("hose-union", "hydraulic", HYD, [(-0.972, 0.0), (-0.972, 0.014), (UNIT_Y0, 0.014), (UNIT_Y0, 0.0)], x=0.265, z=0.106, role="detail", tone=0.3),
        slab("unit-body", "hydraulic", HYD, rounded(0.18, 0.35, 0.022, 0.19, 0.012), UNIT_Y0, -0.565),
        slab("unit-lid", "hydraulic", HYD, rounded(UNIT_X0, UNIT_X1, UNIT_Z0, UNIT_Z1, 0.012), -0.565, UNIT_Y1, tone=0.45),
        slab("unit-lid-bolt", "hydraulic", HYD, hexagon(0.185, 0.027, 0.013), UNIT_Y1, -0.542, role="detail", tone=0.2, repeat={"count": 2, "translation": [0, r6(-0.16), 0]}),
        slab("unit-lid-bolt-left", "hydraulic", HYD, hexagon(0.185, 0.185, 0.013), UNIT_Y1, -0.542, role="detail", tone=0.2, repeat={"count": 2, "translation": [0, r6(-0.16), 0]}),
        rev("cable-gland", "hydraulic", HYD, [(UNIT_Y1, 0.0), (UNIT_Y1, 0.016), (-0.535, 0.016), (-0.535, 0.012), (-0.52, 0.012), (-0.52, 0.0)], x=0.3, z=0.106, role="detail", tone=0.25),
        tube("cable", "hydraulic", HYD, [(0.3, -0.52, 0.106), (0.3, -0.49, 0.106), (0.292, -0.465, 0.106), (0.27, -0.45, 0.106), (0.24, -0.447, 0.106)], 0.006, role="detail", tone=0.15),
        hrev("conduit", "hydraulic", HYD, [(0.136, 0.0), (0.136, 0.0125), (0.181, 0.0125), (0.181, 0.0)], 0, -0.75, 0.06, role="detail", tone=0.35),
        slab("bracket-upper", "hydraulic", GALV, BRACKET, -0.604, -0.596, role="detail"),
        slab("bracket-lower", "hydraulic", GALV, BRACKET, -0.904, -0.896, role="detail"),
    ]


def thermal50():
    """🌡 Thermal at 1:50: one surface per region, following the body."""
    return [
        skin("th-shaft-skin", "shaft", STEEL, 0.1, su(0.5), su(BAND_U0), "shaft-steel"),
        th("th-head-crown", "head-crown", "shaft-steel"),
        th("th-reflector", "reflector", "reflector-band"),
        th("th-lens", "lens", "beacon-led"),
        th("th-led", "led", "beacon-led"),
        th("th-frame-top", "frame-top", "cover-frame", sectionFaces=["-u"]),
        skin("th-sleeve-skin", "sleeve", GALV, 0.15, -1.242, COLLAR_BOTTOM, "sleeve-steel"),
        th("th-sleeve-floor", "sleeve-floor", "sleeve-steel"),
        th("th-anchor-tab-upper", "anchor-tab-upper", "sleeve-steel"),
        th("th-anchor-tab-lower", "anchor-tab-lower", "sleeve-steel"),
        th("th-unit-body", "unit-body", "hydraulic-unit"),
        th("th-unit-lid", "unit-lid", "hydraulic-unit"),
        th("th-hose", "hose", "hydraulic-unit"),
        cylinder_skin(-1.23, 0.04),
    ]


def cylinder_skin(y0, foot):
    """🛢 The outer surface of the cylinder (foot, tube, gland) as one closed thermal solid of the hydraulic region."""
    out = rev("th-cylinder", "hydraulic", HYD, [(y0, 0.0), (y0, foot), (CYL_Y0, foot), (CYL_Y0, CYL_R), (CYL_Y1, CYL_R), (CYL_Y1, 0.04), (-0.278, 0.04), (GLAND_Y1, 0.034), (GLAND_Y1, 0.0)], tone=0.7)
    out["region"] = "hydraulic-unit"
    return out


def masses(r, unit=None):
    """⚖️ The three concentrated masses at the centres of mass of the rig."""
    return [
        ball("mass-sleeve", "sleeve", GALV, 0, -0.6, 0, r, member="sleeve", part="mass"),
        ball("mass-shaft", "shaft", STEEL, 0, su(0.6), 0, r, member="shaft", part="mass"),
        ball("mass-hydraulic", "hydraulic", HYD, 0.2, -0.8, 0.08, unit or r, member="hydraulic", part="mass"),
    ]


def support(r_in, r_out, depth):
    """🧷 The fixed support: the ring of the frame cast into the foundation at ground level."""
    return rev("support-sleeve-fixed", "sleeve", IRON, [(-depth, r_in), (-depth, r_out), (0.0, r_out), (0.0, r_in)], closed=True, member="sleeve", part="support", anchor="sleeve-fixed")


def structure50():
    """🏗 Structure at 1:50: sleeve tube, shaft tube, cylinder and rod with their joints."""
    return [
        st("member-sleeve", "sleeve-tube", "sleeve", "member"),
        st("member-shaft", "shaft-tube", "shaft", "member"),
        st("member-cylinder", "cyl-tube", "hydraulic", "member"),
        st("member-cyl-cap", "cyl-cap", "hydraulic", "member"),
        st("member-rod", "rod", "hydraulic", "member"),
        st("joint-guide-bushing", "guide-bushing", "shaft", "joint"),
        st("joint-shaft-flange", "shaft-flange", "shaft", "joint"),
        st("joint-crosshead", "crosshead", "hydraulic", "joint"),
        st("joint-piston", "piston", "hydraulic", "joint"),
        st("joint-cyl-flange", "cyl-flange", "hydraulic", "joint"),
        st("joint-bracket-upper", "bracket-upper", "hydraulic", "joint"),
        st("joint-bracket-lower", "bracket-lower", "hydraulic", "joint"),
        support(0.15, COLLAR_R, 0.07),
    ] + masses(0.03, 0.02)


def body100():
    """🧊 Body at 1:100: fixings, buffers, glands and bolts merged away, walls kept."""
    plate, opening = frame_plan()
    head = [(su(0.99), 0.0), (su(0.99), 0.095), (su(BAND_U1), 0.095), (su(BAND_U1), 0.097), (su(LENS_U1), 0.097), (su(LENS_U1), 0.1)] + [(su(u), r) for u, r in dome(8)]
    return [
        rev("sleeve-tube", "sleeve", GALV, [(-1.235, 0.142), (-1.235, 0.15), (FRAME_BOTTOM, 0.15), (FRAME_BOTTOM, 0.142)], closed=True),
        rev("sleeve-floor", "sleeve", GALV, [(-1.25, 0.0), (-1.25, 0.15), (-1.235, 0.15), (-1.235, 0.0)]),
        box("anchor-tab-upper", "sleeve", GALV, 0.146, 0.19, -0.34, -0.26, -0.0075, 0.0075, role="detail", repeat=turn(4)),
        box("anchor-tab-lower", "sleeve", GALV, 0.146, 0.19, -1.14, -1.06, -0.0075, 0.0075, role="detail", repeat=turn(4)),
        rev("frame-collar", "sleeve", IRON, [(COLLAR_BOTTOM, 0.15), (COLLAR_BOTTOM, COLLAR_R), (FRAME_BOTTOM, COLLAR_R), (FRAME_BOTTOM, 0.15)], closed=True),
        slab("frame-plate", "sleeve", IRON, plate, FRAME_BOTTOM, FRAME_TOP, holes=opening),
        rev("seal-ring", "sleeve", IRON, [(0.0, 0.1005), (0.0, 0.142), (FRAME_TOP, 0.142), (FRAME_TOP, 0.1005)], closed=True, role="detail", tone=0.2),
        rev("shaft-tube", "shaft", STEEL, [(su(0.015), 0.095), (su(0.015), 0.1), (su(BAND_U1), 0.1), (su(BAND_U1), 0.095)], closed=True),
        rev("shaft-flange", "shaft", STEEL, [(su(0), 0.095), (su(0), 0.14), (su(0.015), 0.14), (su(0.015), 0.095)], closed=True, role="detail", tone=0.4),
        rev("reflector", "shaft", FILM, [(su(BAND_U0), 0.1), (su(BAND_U0), 0.1005), (su(BAND_U1), 0.1005), (su(BAND_U1), 0.1)], closed=True),
        rev("lens", "shaft", PC, [(su(BAND_U1), 0.097), (su(BAND_U1), 0.1), (su(LENS_U1), 0.1), (su(LENS_U1), 0.097)], closed=True),
        rev("head", "shaft", STEEL, head),
        rev("rod", "shaft", HYD, [(su(0.17), 0.0), (su(0.17), ROD_R), (su(0.99), ROD_R), (su(0.99), 0.0)], tone=0.85),
        rev("piston", "shaft", HYD, [(su(0.13), 0.0), (su(0.13), CYL_BORE), (su(0.17), CYL_BORE), (su(0.17), 0.0)], role="detail", tone=0.3),
        rev("cyl-base", "hydraulic", HYD, [(-1.235, 0.0), (-1.235, 0.045), (CYL_Y0, 0.045), (CYL_Y0, 0.0)], tone=0.4),
        rev("cyl-tube", "hydraulic", HYD, [(CYL_Y0, CYL_BORE), (CYL_Y0, CYL_R), (CYL_Y1, CYL_R), (CYL_Y1, CYL_BORE)], closed=True, tone=0.7),
        rev("cyl-gland", "hydraulic", HYD, [(CYL_Y1, ROD_R), (CYL_Y1, 0.04), (GLAND_Y1, 0.04), (GLAND_Y1, ROD_R)], closed=True, tone=0.4),
        tube("hose", "hydraulic", HYD, HOSE, 0.008, role="detail", tone=0.2, slender=True),
        slab("unit", "hydraulic", HYD, rounded(UNIT_X0, UNIT_X1, UNIT_Z0, UNIT_Z1, 0.012, 2), UNIT_Y0, UNIT_Y1),
        slab("bracket-upper", "hydraulic", GALV, BRACKET, -0.6075, -0.5925, role="detail"),
        slab("bracket-lower", "hydraulic", GALV, BRACKET, -0.9075, -0.8925, role="detail"),
    ]


def thermal100():
    """🌡 Thermal at 1:100."""
    return [
        skin("th-shaft-skin", "shaft", STEEL, 0.1, su(0.5), su(BAND_U0), "shaft-steel"),
        skin("th-head-skirt", "shaft", STEEL, 0.1, su(LENS_U1), su(SKIRT_U1), "shaft-steel"),
        rev_region(rev("th-head-crown", "shaft", STEEL, [(su(SKIRT_U1), 0.0)] + [(su(u), r) for u, r in dome(8)]), "shaft-steel"),
        th("th-reflector", "reflector", "reflector-band"),
        th("th-lens", "lens", "beacon-led"),
        slab_region(slab("th-frame-top", "sleeve", IRON, frame_plan()[0], 0.0, FRAME_TOP, holes=frame_plan()[1], sectionFaces=["-u"]), "cover-frame"),
        skin("th-sleeve-skin", "sleeve", GALV, 0.15, -1.235, COLLAR_BOTTOM, "sleeve-steel"),
        th("th-sleeve-floor", "sleeve-floor", "sleeve-steel"),
        th("th-unit", "unit", "hydraulic-unit"),
        th("th-hose", "hose", "hydraulic-unit"),
        cylinder_skin(-1.235, 0.045),
    ]


def rev_region(solid, region):
    """🎯 Gives an own thermal solid its region."""
    solid["region"] = region
    return solid


slab_region = rev_region


def structure100():
    """🏗 Structure at 1:100."""
    return [
        st("member-sleeve", "sleeve-tube", "sleeve", "member"),
        st("member-shaft", "shaft-tube", "shaft", "member"),
        st("member-cylinder", "cyl-tube", "hydraulic", "member"),
        st("member-rod", "rod", "hydraulic", "member"),
        st("joint-shaft-flange", "shaft-flange", "shaft", "joint"),
        rev("joint-guide", "sleeve", GALV, [(COLLAR_BOTTOM, 0.102), (COLLAR_BOTTOM, 0.142), (FRAME_BOTTOM, 0.142), (FRAME_BOTTOM, 0.102)], closed=True, member="shaft", part="joint"),
        st("joint-cyl-base", "cyl-base", "hydraulic", "joint"),
        st("joint-bracket-upper", "bracket-upper", "hydraulic", "joint"),
        st("joint-bracket-lower", "bracket-lower", "hydraulic", "joint"),
        support(0.15, COLLAR_R, 0.07),
    ] + masses(0.03)


def body200():
    """🧊 Body at 1:200: one cup-walled sleeve, one frame, tube, band, head, cylinder, rod, unit and its bracket web."""
    plate, opening = frame_plan(hole=0.135)
    head = [(su(BAND_U1), 0.0), (su(BAND_U1), 0.1)] + [(su(u), r) for u, r in dome(6)]
    return [
        rev("sleeve-tube", "sleeve", GALV, [(-1.22, 0.135), (-1.22, 0.15), (-0.03, 0.15), (-0.03, 0.135)], closed=True),
        rev("sleeve-floor", "sleeve", GALV, [(-1.25, 0.0), (-1.25, 0.15), (-1.22, 0.15), (-1.22, 0.0)]),
        slab("frame", "sleeve", IRON, plate, -0.03, FRAME_TOP, holes=opening),
        rev("shaft-tube", "shaft", STEEL, [(su(0), 0.09), (su(0), 0.1), (su(BAND_U0), 0.1), (su(BAND_U0), 0.09)], closed=True),
        rev("reflector", "shaft", FILM, [(su(BAND_U0), 0.09), (su(BAND_U0), 0.1), (su(BAND_U1), 0.1), (su(BAND_U1), 0.09)], closed=True),
        rev("head", "shaft", STEEL, head),
        rev("rod", "shaft", HYD, [(su(0.17), 0.0), (su(0.17), ROD_R), (su(BAND_U1), ROD_R), (su(BAND_U1), 0.0)], tone=0.85),
        rev("cylinder", "hydraulic", HYD, [(-1.22, 0.0), (-1.22, CYL_R), (GLAND_Y1, CYL_R), (GLAND_Y1, ROD_R), (-0.3, ROD_R), (-0.3, 0.0)], tone=0.7),
        box("unit", "hydraulic", HYD, UNIT_X0, UNIT_X1, UNIT_Y0, UNIT_Y1, UNIT_Z0, UNIT_Z1),
        slab("bracket", "hydraulic", GALV, BRACKET, -0.9, -0.6, role="detail"),
    ]


def thermal200():
    """🌡 Thermal at 1:200."""
    return [
        skin("th-shaft-skin", "shaft", STEEL, 0.1, su(0.5), su(BAND_U0), "shaft-steel"),
        rev_region(rev("th-head-crown", "shaft", STEEL, [(su(LENS_U1), 0.0), (su(LENS_U1), 0.1)] + [(su(u), r) for u, r in dome(6)]), "shaft-steel"),
        skin("th-reflector", "shaft", FILM, 0.1, su(BAND_U0), su(BAND_U1), "reflector-band"),
        skin("th-beacon", "shaft", PC, 0.1, su(BAND_U1), su(LENS_U1), "beacon-led"),
        th("th-frame", "frame", "cover-frame", sectionFaces=["-u"]),
        skin("th-sleeve-skin", "sleeve", GALV, 0.15, -1.22, -0.03, "sleeve-steel"),
        th("th-sleeve-floor", "sleeve-floor", "sleeve-steel"),
        th("th-unit", "unit", "hydraulic-unit"),
        th("th-cylinder", "cylinder", "hydraulic-unit"),
    ]


def structure200():
    """🏗 Structure at 1:200."""
    return [
        st("member-sleeve", "sleeve-tube", "sleeve", "member"),
        st("member-shaft", "shaft-tube", "shaft", "member"),
        st("member-shaft-band", "reflector", "shaft", "member"),
        st("member-cylinder", "cylinder", "hydraulic", "member"),
        st("member-rod", "rod", "hydraulic", "member"),
        rev("joint-guide", "sleeve", GALV, [(-0.09, 0.1), (-0.09, 0.135), (-0.03, 0.135), (-0.03, 0.1)], closed=True, member="shaft", part="joint"),
        st("joint-bracket", "bracket", "hydraulic", "joint"),
        support(0.15, 0.18, 0.03),
    ] + masses(0.03)


def body500():
    """🧊 Body at 1:500: sleeve, frame, shaft with its head, unit block."""
    plate, opening = frame_plan(hole=0.13)
    head = [(su(BAND_U1), 0.0), (su(BAND_U1), 0.1)] + [(su(u), r) for u, r in dome(4)]
    return [
        rev("sleeve", "sleeve", GALV, [(-1.175, 0.13), (-1.175, 0.15), (-0.06, 0.15), (-0.06, 0.13)], closed=True),
        rev("sleeve-floor", "sleeve", GALV, [(-1.25, 0.0), (-1.25, 0.15), (-1.175, 0.15), (-1.175, 0.0)]),
        slab("frame", "sleeve", IRON, plate, -0.06, FRAME_TOP, holes=opening),
        rev("shaft", "shaft", STEEL, [(su(0), 0.08), (su(0), 0.1), (su(BAND_U1), 0.1), (su(BAND_U1), 0.08)], closed=True),
        rev("head", "shaft", STEEL, head),
        box("unit", "hydraulic", HYD, 0.14, UNIT_X1, UNIT_Y0, UNIT_Y1, UNIT_Z0, UNIT_Z1),
    ]


def thermal500():
    """🌡 Thermal at 1:500."""
    return [
        skin("th-shaft-skin", "shaft", STEEL, 0.1, su(0.5), su(BAND_U0), "shaft-steel"),
        skin("th-reflector", "shaft", FILM, 0.1, su(BAND_U0), su(BAND_U1), "reflector-band"),
        th("th-head", "head", "beacon-led"),
        th("th-frame", "frame", "cover-frame", sectionFaces=["-u"]),
        skin("th-sleeve-skin", "sleeve", GALV, 0.15, -1.25, -0.06, "sleeve-steel"),
        th("th-unit", "unit", "hydraulic-unit"),
    ]


def structure500():
    """🏗 Structure at 1:500."""
    return [
        st("member-sleeve", "sleeve", "sleeve", "member"),
        st("member-shaft", "shaft", "shaft", "member"),
        rev("member-cylinder", "hydraulic", HYD, [(-1.175, 0.0), (-1.175, 0.0375), (su(0.925) + STROKE, 0.0375), (su(0.925) + STROKE, 0.0)], member="hydraulic", part="member", tone=0.7),
        rev("joint-guide", "sleeve", GALV, [(-0.135, 0.1), (-0.135, 0.13), (-0.06, 0.13), (-0.06, 0.1)], closed=True, member="shaft", part="joint"),
        support(0.15, 0.2, 0.075),
    ] + masses(0.04)


DECISIONS = {
    "50": [
        text(
            "Body: the bollard as it is built. Fixed in the ground: the galvanised sleeve tube of 0.30 m with its 8 mm wall, the floor plate with three drain holes and three rubber buffers, eight anchor tabs for the concrete, the bronze-lined guide bushing under the cover, the cast-iron cover frame (collar around the sleeve head, plate of 0.40 m standing 15 mm proud with a splayed edge, four hexagon anchor bolts), the seal retainer ring and the wiper seal against the shaft. Rising: the shaft tube of 0.20 m with its 5 mm wall and the guide flange at its foot, the 0.5 mm reflective film, the head casting with the amber lens ring, twelve LED windows, the crown with its shadow gap and three screws, the crosshead with the rod nut, the 36 mm piston rod and the piston. Hydraulics: the single-acting cylinder of 70 mm standing on its bolted foot flange inside the shaft (end cap, tube, gland), the port, the hose through the grommet in the sleeve wall up to the union under the power unit, the sealed unit housing with its bolted lid, cable gland, cable and conduit, hung on two bracket plates welded to the sleeve.",
            "Körper: der Poller, wie er gebaut ist. Fest im Boden: das verzinkte Hülsenrohr von 0,30 m mit 8 mm Wand, die Bodenplatte mit drei Entwässerungslöchern und drei Gummipuffern, acht Ankerlaschen für den Beton, die Führungsbuchse unter der Abdeckung, der Gusseisen-Abdeckrahmen (Kragen um den Hülsenkopf, Platte von 0,40 m mit 15 mm Überstand und geneigter Kante, vier Sechskant-Ankerschrauben), der Dichtungshaltering und der Abstreifer am Schaft. Hebend: das Schaftrohr von 0,20 m mit 5 mm Wand und dem Führungsflansch am Fuß, die Reflexfolie von 0,5 mm, das Kopfgussteil mit dem bernsteinfarbenen Linsenring, zwölf LED-Fenstern, der Kalotte mit Schattenfuge und drei Schrauben, die Traverse mit der Stangenmutter, die Kolbenstange von 36 mm und der Kolben. Hydraulik: der einfachwirkende Zylinder von 70 mm auf seinem verschraubten Fußflansch im Schaft (Boden, Rohr, Stopfbuchse), der Anschluss, der Schlauch durch die Tülle in der Hülsenwand bis zur Verschraubung unter der Antriebseinheit, das dichte Gehäuse mit verschraubtem Deckel, Kabelverschraubung, Kabel und Schutzrohr, an zwei mit der Hülse verschweißten Konsolblechen."),
        text(
            "Thermal: each region has one layer, so each is one surface. Shaft steel is the tube skin that stands in the air when raised and the crown; the reflective film and the lens ring with its LED windows are their own shells; the cover frame is the part of the plate above the ground (its underside continues into the casting); the buried sleeve is its outer skin, the floor plate and the anchor tabs; the hydraulic region is the unit housing with its lid, the hose and the cylinder inside the sleeve.",
            "Thermisch: jede Region hat eine Schicht und ist daher eine Fläche. Schaftstahl ist die Rohrhaut, die ausgefahren in der Luft steht, und die Kalotte; die Reflexfolie und der Linsenring mit den LED-Fenstern sind eigene Schalen; der Abdeckrahmen ist der Teil der Platte über dem Boden (die Unterseite setzt sich im Gussteil fort); die eingegrabene Hülse ist ihre Außenhaut, die Bodenplatte und die Ankerlaschen; die Hydraulikregion ist das Gehäuse der Einheit mit Deckel, der Schlauch und der Zylinder in der Hülse."),
        text(
            "Structure: the load path. A nudge bends the shaft tube, which hands its moment to the sleeve tube through the guide bushing and the guide flange; the sleeve is clamped by the collar cast into the foundation (support). The weight of the shaft goes through the crosshead into the piston rod, the piston, the oil column of the cylinder tube and its foot flange to the sleeve floor. The power unit hangs on its two bracket plates. Three masses at the centres of mass of the rig.",
            "Tragwerk: der Lastpfad. Ein Anstoß biegt das Schaftrohr, das sein Moment über Führungsbuchse und Führungsflansch an das Hülsenrohr abgibt; die Hülse ist durch den ins Fundament eingegossenen Kragen eingespannt (Auflager). Das Schaftgewicht geht über die Traverse in die Kolbenstange, den Kolben, die Ölsäule des Zylinderrohrs und dessen Fußflansch auf den Hülsenboden. Die Antriebseinheit hängt an zwei Konsolblechen. Drei Massen in den Schwerpunkten des Rigs."),
    ],
    "100": [
        text(
            "Body: drawn anew for 15 mm features. The sleeve tube and a 15 mm floor, eight anchor tabs of 15 mm, the collar and one plate without the splayed edge, one seal ring for retainer and wiper; the shaft tube with a 15 mm flange, the film, the lens ring and one head casting (core and crown in one piece); rod and piston, the cylinder as foot, tube and gland, the hose, the unit as one rounded housing on two bracket plates. Bolts, buffers, drain holes, LED windows, screws, crosshead, glands, cable and conduit are below the scale.",
            "Körper: neu gezeichnet für Merkmale von 15 mm. Das Hülsenrohr und ein Boden von 15 mm, acht Ankerlaschen von 15 mm, der Kragen und eine Platte ohne geneigte Kante, ein Dichtungsring für Haltering und Abstreifer; das Schaftrohr mit einem Flansch von 15 mm, die Folie, der Linsenring und ein Kopfgussteil (Kern und Kalotte in einem Stück); Stange und Kolben, der Zylinder als Fuß, Rohr und Stopfbuchse, der Schlauch, die Einheit als ein gerundetes Gehäuse an zwei Konsolblechen. Schrauben, Puffer, Entwässerungslöcher, LED-Fenster, Traverse, Verschraubungen, Kabel und Schutzrohr liegen unter dem Maßstab."),
        text(
            "Thermal: the shaft skin above the ground with the skirt and the crown of the head, the film, the lens ring, the plate above the ground, the sleeve skin with its floor, the unit housing, the hose and the cylinder.",
            "Thermisch: die Schafthaut über dem Boden mit Rand und Kalotte des Kopfes, die Folie, der Linsenring, die Platte über dem Boden, die Hülsenhaut mit Boden, das Gehäuse der Einheit, der Schlauch und der Zylinder."),
        text(
            "Structure: sleeve tube, shaft tube, cylinder tube and rod as members; the guide ring in the sleeve head, the shaft flange, the cylinder foot and the two bracket plates as joints; the collar as the fixed support; three masses.",
            "Tragwerk: Hülsenrohr, Schaftrohr, Zylinderrohr und Stange als Stäbe; der Führungsring im Hülsenkopf, der Schaftflansch, der Zylinderfuß und die zwei Konsolbleche als Gelenke; der Kragen als Einspannung; drei Massen."),
    ],
    "200": [
        text(
            "Body: drawn anew for 30 mm features. The sleeve with a 15 mm wall and a 30 mm floor, collar and plate as one frame of 45 mm, the shaft tube with a 10 mm wall, the reflective band as a ring of the tube, the head with lens and crown as one piece, the rod, the cylinder as one piece, the unit as a plain block and its two bracket plates as one web. Tabs, seals, flange, piston and hose are below the scale.",
            "Körper: neu gezeichnet für Merkmale von 30 mm. Die Hülse mit 15 mm Wand und 30 mm Boden, Kragen und Platte als ein Rahmen von 45 mm, das Schaftrohr mit 10 mm Wand, das Reflexband als Ring des Rohrs, der Kopf mit Linse und Kalotte als ein Stück, die Stange, der Zylinder als ein Stück, die Einheit als schlichter Block und ihre zwei Konsolbleche als ein Steg. Laschen, Dichtungen, Flansch, Kolben und Schlauch liegen unter dem Maßstab."),
        text(
            "Thermal: one skin per region (shaft above the ground and crown, band, beacon ring, frame, sleeve with floor) and the unit block with the cylinder; only the sum of the areas is compared at this scale.",
            "Thermisch: eine Haut je Region (Schaft über dem Boden und Kalotte, Band, Leuchtring, Rahmen, Hülse mit Boden) und der Block der Einheit mit dem Zylinder; in diesem Maßstab wird nur die Flächensumme verglichen."),
        text(
            "Structure: sleeve tube, shaft tube, cylinder and rod as members, the guide ring and the bracket web as joints, the ring of the frame in the foundation as the fixed support, three masses.",
            "Tragwerk: Hülsenrohr, Schaftrohr, Zylinder und Stange als Stäbe, der Führungsring und der Konsolsteg als Gelenke, der Rahmenring im Fundament als Einspannung, drei Massen."),
    ],
    "500": [
        text(
            "Body: drawn anew for 75 mm features and already minimal. The sleeve with a 20 mm wall and a 75 mm floor, the frame as one slab of 75 mm, the shaft as one tube with a 20 mm wall carrying the band, the head as one domed cap, the unit as one block reaching the sleeve. Cylinder, rod, bracket, hose and every fixing are below the scale.",
            "Körper: neu gezeichnet für Merkmale von 75 mm und bereits minimal. Die Hülse mit 20 mm Wand und 75 mm Boden, der Rahmen als eine Platte von 75 mm, der Schaft als ein Rohr mit 20 mm Wand samt Band, der Kopf als eine gewölbte Kappe, die Einheit als ein Block bis an die Hülse. Zylinder, Stange, Konsole, Schlauch und alle Befestigungen liegen unter dem Maßstab."),
        text(
            "Thermal: one skin per region (shaft above the ground, band, the cap for the beacon ring, the frame, sleeve) and the unit block; only the sum of the areas is compared.",
            "Thermisch: eine Haut je Region (Schaft über dem Boden, Band, die Kappe für den Leuchtring, der Rahmen, Hülse) und der Block der Einheit; nur die Flächensumme wird verglichen."),
        text(
            "Structure: sleeve tube and shaft tube as members, the cylinder with its rod as one column inside them (kept here although the body omits it), the guide ring as joint, the ring of the frame as the fixed support, three masses.",
            "Tragwerk: Hülsenrohr und Schaftrohr als Stäbe, der Zylinder mit Stange als eine Säule darin (hier behalten, obwohl der Körper ihn weglässt), der Führungsring als Gelenk, der Rahmenring als Einspannung, drei Massen."),
    ],
}


def volumes():
    """📚 The whole `volumes` member."""
    lists = {"50": (body50, thermal50, structure50), "100": (body100, thermal100, structure100), "200": (body200, thermal200, structure200), "500": (body500, thermal500, structure500)}
    return {k: {"status": "authored", "denominator": int(k), "body": b(), "thermal": t(), "structure": s(), "decisions": DECISIONS[k]} for k, (b, t, s) in lists.items()}
