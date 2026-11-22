"""🧩 The parts of the used cello asset per scale: 3D solids (body, thermal, structure) and the 2D shapes of the three views.

Each builder takes the scale denominator and returns plain dictionaries for the asset document. Numbers come from `author_model.py`.

@see ./author_model.py — the hand-chosen numbers
@see ./author-cello.py — assembles the document
"""
import math
from author_geom import circle, ccw, pt, r6, simplify
from author_model import (ARCH_BACK, ARCH_TOP, B, BODY_L, BODY_U0, BOW_FROG, BOW_TIP, BRIDGE_U, CASE_STATIONS, CASE_TOP, CXC, CYC, ENDPIN_LEN,
                          ENDPIN_R, ENDPIN_REST_TIP, FB_END_U, LID_ORIGIN, NUT_U, RIB, SCROLL_TOP, STRINGS, TAIL_U0, TAIL_U1, WALL, XP, arch_long,
                          back_v, case_b, case_back_tub, case_back_x, case_lid_tub, case_lid_x, case_xb, case_xf, corpus_polyhedron, corpus_profile,
                          corpus_stations, fb_top, outline_face, plate_polyhedron, string_v, top_v)

FEATURE = {50: 0.0075, 100: 0.015, 200: 0.030, 500: 0.075}
Q_LIMB = [0.5, 0.5, 0.5, 0.5]

WHEEL_C = (-0.112, 0.032)
WHEEL_R, WHEEL_W, WHEEL_Z = 0.032, 0.024, 0.100
P_WHEEL_SHEET_V = 0.0335
STOPPER_C = (CXC, 0.006 + WALL)
STOPPER_R, STOPPER_H = 0.0425, 0.018
HANDLE_Y, HANDLE_L = 0.620, 0.130
STRAP_Z, STRAP_W, STRAP_T = 0.085, 0.050, 0.016
STRAP_Y0, STRAP_Y1 = 0.300, 1.020
LATCH_YS = (0.250, 0.620, 1.060)
HINGE_YS = (0.250, 0.620)

BOW_PHI = -8.0
BOW_PIVOT_Z = 0.088
BOW_HAIR_X = 0.080


def stick_v(u):
    """🏹 The v of the bow stick axis above the hair (negative = away from the hair), cambered towards the hair in the middle."""
    t = (u - (-0.06)) / 0.37
    return -(0.0095 + 0.0115 * t * t)


def stick_r(u):
    """🏹 Radius of the bow stick: 8.6 mm at the frog end tapering to 5.6 mm at the tip."""
    t = (u - BOW_FROG) / (BOW_TIP - BOW_FROG)
    return 0.0043 - 0.0015 * t


def bow_axes():
    """🧭 The bow frame axes (u, v, w) in cello-local coordinates at rest: turned by the rest abduction of the bow-angle bone about v."""
    p = math.radians(BOW_PHI)
    return (math.cos(p), 0.0, math.sin(p)), (0.0, 1.0, 0.0), (-math.sin(p), 0.0, math.cos(p))


def bow_pivot_cello():
    """📍 The rest position of the bow pivot (on the hair line, 0.41 m from the frog end) in cello-local (u, v, w)."""
    v = -(BOW_HAIR_X - CXC)
    return (0.540, v, BOW_PIVOT_Z)


def bow_to_cello(p):
    """🔁 Bow-local (u, v, w) to cello-local (u, v, w) at rest."""
    o = bow_pivot_cello()
    eu, ev, ew = bow_axes()
    return tuple(o[i] + p[0] * eu[i] + p[1] * ev[i] + p[2] * ew[i] for i in range(3))


def solid(sid, kind, bone, material, tone, role="body", **fields):
    """🧱 A volume solid dictionary with rounded numbers."""
    out = {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
    for key, value in fields.items():
        out[key] = value
    return out


def poly_solid(sid, bone, material, tone, mesh, role="body", **extra):
    """🪨 A polyhedron solid from (vertices, triangles, area, volume)."""
    v, t = mesh[0], mesh[1]
    return solid(sid, "polyhedron", bone, material, tone, role, vertices=v, triangles=[list(x) for x in t], **extra)


def capsule(sid, bone, material, tone, p0, p1, r0, r1=None, role="body", **extra):
    """💊 A capsule between two bone-local points."""
    return solid(sid, "capsule", bone, material, tone, role, p0=pt(p0), p1=pt(p1), r0=r6(r0), r1=r6(r0 if r1 is None else r1), **extra)


def box(sid, bone, material, tone, centre, size, role="body", corner=None, **extra):
    """📦 A box centred at a bone-local point."""
    fields = {"size": pt(size), "transform": {"translation": pt(centre)}}
    if corner:
        fields["cornerRadius"] = corner
    fields.update(extra)
    return solid(sid, "box", bone, material, tone, role, **fields)


def ellipsoid(sid, bone, material, tone, centre, radii, role="detail", **extra):
    """🥚 An ellipsoid centred at a bone-local point."""
    return solid(sid, "ellipsoid", bone, material, tone, role, radii=pt(radii), transform={"translation": pt(centre)}, **extra)


def loft_u(sid, bone, material, tone, sections, exponent=2.0, role="body", **extra):
    """🌀 A loft along the bone axis: sections (u, v-centre, half depth along v, half width along w)."""
    secs = []
    for u, vc, rx, ry in sections:
        s = {"at": pt((u, vc, 0.0)), "rx": r6(rx), "ry": r6(ry), "rotation": Q_LIMB}
        if exponent != 2.0:
            s["exponent"] = exponent
        secs.append(s)
    return solid(sid, "loft", bone, material, tone, role, sections=secs, capStart=True, capEnd=True, **extra)


def revolve_u(sid, bone, material, tone, profile, origin=(0, 0, 0), role="body", **extra):
    """🏺 A revolve about the bone axis u through `origin`, profile (axial, radial)."""
    return solid(sid, "revolve", bone, material, tone, role, profile=[pt(p) for p in profile], axis=[1.0, 0.0, 0.0], origin=pt(origin), **extra)


def revolve_w(sid, bone, material, tone, profile, origin, role="body", **extra):
    """🛞 A revolve about the lateral axis w through `origin`, profile (axial along w, radial)."""
    return solid(sid, "revolve", bone, material, tone, role, profile=[pt(p) for p in profile], axis=[0.0, 0.0, 1.0], origin=pt(origin), **extra)


def extrude(sid, bone, material, tone, profile, plane, a, b, role="body", holes=None, **extra):
    """🧈 An extrusion of a bone-local profile on `plane` from `a` to `b`."""
    fields = {"profile": [pt(p) for p in ccw(profile)], "plane": plane, "from": r6(a), "to": r6(b)}
    if holes:
        fields["holes"] = [[pt(p) for p in ccw(h)] for h in holes]
    fields.update(extra)
    return solid(sid, "extrude", bone, material, tone, role, **fields)


def shape(sid, bone, role, z, filled, tone, points, closed=True, region=None, material=None, pen=None, hatch=None, solid_hint=None, regions=None):
    """✏️ A 2D poly shape for the representation geometry."""
    out = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    if pen is not None:
        out["penMm"] = pen
    if region:
        out["region"] = region
    if regions:
        out["regions"] = regions
    if material:
        out["material"] = material
    if hatch:
        out["hatch"] = hatch
    out["kind"] = "poly"
    out["points"] = [pt(p) for p in points]
    out["closed"] = closed
    if solid_hint:
        out["solid"] = solid_hint
    return out


def case_station_count(scale):
    """🔢 Points per half-section of the case shells at a scale."""
    return {50: 24, 100: 16, 200: 12, 500: 8}[scale]


def case_stations(scale):
    """🔢 The case stations kept at a scale (coarser scales drop the intermediate ones)."""
    keep = {50: None, 100: None, 200: (0, 1, 3, 4, 5, 7, 9, 10, 11, 13, 15, 17, 18), 500: (0, 2, 4, 5, 7, 9, 11, 13, 16, 18)}[scale]
    return CASE_STATIONS if keep is None else [CASE_STATIONS[i] for i in keep]


def case_tubs(scale):
    """🛁 The back shell and lid polyhedra of the case at a scale."""
    import author_model
    saved = author_model.CASE_STATIONS
    author_model.CASE_STATIONS = case_stations(scale)
    try:
        back = case_back_tub(case_station_count(scale))
        lid = case_lid_tub(case_station_count(scale))
    finally:
        author_model.CASE_STATIONS = saved
    return back, lid


def wheel_local(p):
    """🛞 World (x, y, z) to the local frame of the wheel axle bone."""
    return (p[1] - WHEEL_C[1], -(p[0] - WHEEL_C[0]), p[2])


def stopper_local(p):
    """🪨 World (x, y, z) to the local frame of the stopper bone."""
    return (p[1] - STOPPER_C[1], -(p[0] - STOPPER_C[0]), p[2])


def endpin_local(p):
    """📌 Cello-local (u, v, w) to endpin-local."""
    return (p[0] - ENDPIN_REST_TIP, p[1], p[2])


def strap_profile(z, y0, y1, steps):
    """🎒 The side profile of a backpack strap lying on the back shell at lateral z, in case-local (u, v): inner edge on the shell, outer edge 16 mm out."""
    ys = [y0 + (y1 - y0) * i / steps for i in range(steps + 1)]
    inner = [(y, -case_back_x(y, z) - 0.0004) for y in ys]
    outer = [(y, -case_back_x(y, z) + STRAP_T) for y in reversed(ys)]
    return inner + outer


def case_parts(scale):
    """🧳 Body solids of the case at a scale, with their thermal regions and structural roles: (body, thermal, structure)."""
    body, thermal, structure = [], [], []
    back, lid = case_tubs(scale)
    body.append(poly_solid("case-back-shell", "case", "abs-eps", 0.16, back))
    body.append(poly_solid("case-lid-shell", "case-lid", "abs-eps", 0.2, lid))
    thermal += [{"id": "th-case-back-shell", "of": "case-back-shell", "region": "case-shell"}, {"id": "th-case-lid-shell", "of": "case-lid-shell", "region": "case-shell"}]
    structure += [{"id": "st-case-back-shell", "of": "case-back-shell", "member": "case", "part": "member"}, {"id": "st-case-lid-shell", "of": "case-lid-shell", "member": "case-lid", "part": "member"}]
    if scale in (50, 100):
        for side, z in (("l", STRAP_Z), ("r", -STRAP_Z)):
            prof = strap_profile(z, STRAP_Y0, STRAP_Y1, 14 if scale == 50 else 8)
            body.append(extrude(f"strap-{side}", "case", "webbing", 0.3, prof, "xy", z - STRAP_W / 2, z + STRAP_W / 2))
            thermal.append({"id": f"th-strap-{side}", "of": f"strap-{side}", "region": "case-straps"})
        hy = HANDLE_Y
        zr = -case_b(hy)
        body.append(capsule("handle-grip", "case", "rubber-grip", 0.12, (hy - HANDLE_L / 2, -XP, zr - 0.034), (hy + HANDLE_L / 2, -XP, zr - 0.034), 0.0125))
        for k, dy in (("lo", -0.068), ("hi", 0.068)):
            body.append(box(f"handle-mount-{k}", "case", "rubber-grip", 0.14, (hy + dy, -XP, zr - 0.017), (0.022, 0.026, 0.036), corner=0.004))
        thermal += [{"id": "th-handle-grip", "of": "handle-grip", "region": "case-handle"}] + [{"id": f"th-handle-mount-{k}", "of": f"handle-mount-{k}", "region": "case-handle"} for k in ("lo", "hi")]
        structure.append({"id": "st-handle-grip", "of": "handle-grip", "member": "case", "part": "member"})
        for i, y in enumerate(LATCH_YS):
            zl = case_b(y)
            t = 0.012 if scale == 50 else 0.015
            body.append(box(f"latch-{i + 1}", "case", "nickel-steel", 0.55, (y, -XP, zl + t / 2 - 0.002), (0.060, 0.040, t), corner=0.003))
            thermal.append({"id": f"th-latch-{i + 1}", "of": f"latch-{i + 1}", "region": "case-hardware"})
        if scale == 50:
            for i, y in enumerate(HINGE_YS):
                zr = -case_b(y)
                body.append(box(f"hinge-{i + 1}", "case", "nickel-steel", 0.5, (y, -XP, zr - 0.004), (0.050, 0.024, 0.012), corner=0.002))
                thermal.append({"id": f"th-hinge-{i + 1}", "of": f"hinge-{i + 1}", "region": "case-hardware"})
                structure.append({"id": f"st-hinge-{i + 1}", "of": f"hinge-{i + 1}", "member": "case-lid", "part": "joint"})
    if scale in (50, 100, 200):
        ww = WHEEL_W if scale < 200 else 0.030
        for side, z in (("l", WHEEL_Z), ("r", -WHEEL_Z)):
            prof = [(-ww / 2, 0.0), (-ww / 2, WHEEL_R - 0.004), (-ww / 2 + 0.004, WHEEL_R), (ww / 2 - 0.004, WHEEL_R), (ww / 2, WHEEL_R - 0.004), (ww / 2, 0.0)]
            body.append(revolve_w(f"wheel-{side}", "case-wheels", "pu-wheel", 0.1, prof, (0.0, 0.0, z)))
            thermal.append({"id": f"th-wheel-{side}", "of": f"wheel-{side}", "region": "case-wheels"})
            structure.append({"id": f"st-wheel-{side}", "of": f"wheel-{side}", "member": "case-wheels", "part": "member"})
            hx = WHEEL_C[0] + 0.006
            body.append(box(f"wheel-housing-{side}", "case", "abs-eps", 0.2, (0.040, -hx, z), (0.056, 0.050, ww + 0.016), corner=0.006))
            thermal.append({"id": f"th-wheel-housing-{side}", "of": f"wheel-housing-{side}", "region": "case-shell"})
            structure.append({"id": f"st-wheel-housing-{side}", "of": f"wheel-housing-{side}", "member": "case", "part": "member"})
        body.append(capsule("wheel-axle", "case-wheels", "nickel-steel", 0.5, (0.0, 0.0, -WHEEL_Z - ww / 2 - 0.006), (0.0, 0.0, WHEEL_Z + ww / 2 + 0.006), 0.006 if scale < 200 else 0.0055, slender=True))
        thermal.append({"id": "th-wheel-axle", "of": "wheel-axle", "region": "case-hardware"})
        structure.append({"id": "st-wheel-axle", "of": "wheel-axle", "member": "case-wheels", "part": "joint"})
    if scale in (50, 100, 200):
        h = {50: 0.009, 100: 0.016, 200: 0.032}[scale]
        body.append(box("bumper-front", "case", "rubber-bumper", 0.08, (h / 2, -0.095, 0.0), (h, 0.030 if scale < 200 else 0.034, 0.140), corner=0.003))
        thermal.append({"id": "th-bumper-front", "of": "bumper-front", "region": "case-feet"})
        structure.append({"id": "st-bumper-front", "of": "bumper-front", "member": "case", "part": "member"})
    if scale == 500:
        body.append(box("case-foot", "case", "rubber-bumper", 0.1, (0.040, 0.0, 0.0), (0.080, 0.240, 0.260), corner=0.01))
        thermal.append({"id": "th-case-foot", "of": "case-foot", "region": "case-feet"})
        structure.append({"id": "st-case-foot", "of": "case-foot", "member": "case", "part": "member"})
        thermal.append(box("th-wheels-500", "case-wheels", "pu-wheel", 0.1, (0.010, 0.0245, 0.0), (0.080, 0.003, 0.260), region="case-wheels", sheet=True))
        thermal.append(box("th-straps-500", "case", "webbing", 0.3, (0.66, -case_back_x(0.66, 0.0) + 0.0015, 0.0), (0.70, 0.003, 0.22), region="case-straps", sheet=True))
    if scale in (200, 500):
        hy = HANDLE_Y
        zr = -case_b(hy)
        if scale == 200:
            body.append(box("handle-grip", "case", "rubber-grip", 0.12, (hy, -XP, zr - 0.023), (0.155, 0.030, 0.046), corner=0.006))
            thermal.append({"id": "th-handle-grip", "of": "handle-grip", "region": "case-handle"})
            structure.append({"id": "st-handle-grip", "of": "handle-grip", "member": "case", "part": "member"})
            for side, z in (("l", STRAP_Z), ("r", -STRAP_Z)):
                prof = strap_profile(z, STRAP_Y0, STRAP_Y1, 4)
                thermal.append(extrude(f"th-strap-{side}-coarse", "case", "webbing", 0.3, prof, "xy", z - STRAP_W / 2, z + STRAP_W / 2, region="case-straps"))
        else:
            thermal.append(box("th-handle-500", "case", "rubber-grip", 0.12, (hy, -XP, zr - 0.0015), (0.155, 0.080, 0.003), region="case-handle", sheet=True))
        thermal.append(box("th-latches-coarse", "case", "nickel-steel", 0.55, (0.62, -XP, case_b(0.62) + 0.00125), (0.080 if scale == 500 else 0.060, 0.080 if scale == 500 else 0.060, 0.0025), region="case-hardware", sheet=True))
    return body, thermal, structure


def neck_sections(scale):
    """🎻 Loft sections of the neck from the heel on the body to the nut (u, v-centre, half depth, half width)."""
    min_half = FEATURE[scale] / 2 if scale >= 200 else 0.0
    rows = []
    for u, depth, half_w in ((0.752, 0.150, 0.030), (0.775, 0.095, 0.026), (0.800, 0.062, 0.0235), (0.840, 0.044, 0.0215), (0.950, 0.037, 0.0165), (NUT_U, 0.033, 0.0128)):
        top = fb_top(u) + 0.010
        rows.append((u, top + depth / 2, depth / 2, max(half_w, min_half)))
    return rows


def fingerboard_sections(scale):
    """🎼 Loft sections of the fingerboard from its free end over the body to the nut."""
    rows = []
    for u in (FB_END_U, 0.60, 0.767, 0.90, NUT_U):
        t = (u - FB_END_U) / (NUT_U - FB_END_U)
        half_w = 0.021 - 0.0085 * t
        th = 0.012 - 0.003 * t
        rows.append((u, fb_top(u) + th / 2, th / 2, half_w))
    return rows


def pegbox_sections(scale):
    """🗝️ Loft sections of the pegbox from the nut to the scroll."""
    rows = []
    for u, depth, half_w in ((NUT_U - 0.004, 0.064, 0.0205), (1.110, 0.066, 0.0225), (1.180, 0.060, 0.0220)):
        front = fb_top(u) - 0.001
        rows.append((u, front + depth / 2, depth / 2, half_w))
    return rows


SCROLL_C = (1.205, 0.014)
SCROLL_R = 0.036


def tail_sections(scale):
    """🎚️ Loft sections of the tailpiece from the tailcord end to the string end."""
    rows = []
    for u in (TAIL_U0, 0.090, 0.170, TAIL_U1):
        t = (u - TAIL_U0) / (TAIL_U1 - TAIL_U0)
        rows.append((u, -0.074 - 0.056 * t, 0.008, 0.017 + 0.015 * t))
    return rows


def tail_anchor(w):
    """🎯 The point on the fine tuner where a string ends, in cello-local (u, v)."""
    return (0.240, -0.151)


def bridge_profile(scale):
    """🌉 The bridge outline in the (v, w) plane: two feet on the arch, legs, the heart region and the arched top (cello bridge 92 mm wide, 90 mm high)."""
    foot = -0.0828
    top = -0.174
    pts = [(foot, -0.047), (foot, -0.024), (foot - 0.020, -0.016), (foot - 0.024, 0.0), (foot - 0.020, 0.016), (foot, 0.024), (foot, 0.047),
           (foot - 0.012, 0.046), (-0.125, 0.036), (-0.150, 0.046), (-0.163, 0.046), (top + 0.0035, 0.030), (top, 0.0), (top + 0.0035, -0.030),
           (-0.163, -0.046), (-0.150, -0.046), (-0.125, -0.036), (foot - 0.012, -0.046)]
    if scale >= 200:
        pts = [(foot, -0.047), (foot, 0.047), (-0.150, 0.046), (top, 0.0), (-0.150, -0.046)]
    return pts


def fhole(side, scale):
    """🎭 The outline of one f-hole in the (w, u) plane of the top (cello-local), lower and upper eyes joined by the stem."""
    sgn = 1 if side == "l" else -1
    pts = []
    for i in range(9):
        t = i / 8
        u = 0.322 + 0.120 * t
        w = sgn * (0.060 + 0.020 * math.sin(math.pi * (t - 0.5)) * -1)
        pts.append((w - 0.003 * sgn, u))
    for i in range(9):
        t = 1 - i / 8
        u = 0.322 + 0.120 * t
        w = sgn * (0.060 + 0.020 * math.sin(math.pi * (t - 0.5)) * -1)
        pts.append((w + 0.003 * sgn, u))
    return pts


def copy_as(src, new_id, **extra):
    """📋 A copy of a solid with a new id and extra fields (thermal copies of hidden regions are own solids, never references)."""
    out = dict(src)
    out["id"] = new_id
    out.update(extra)
    return out


def cello_parts(scale):
    """🎻 The cello, endpin, stopper and their thermal and structural solids at a scale: (body, thermal, structure, masses_volumes)."""
    body, thermal, structure = [], [], []
    volumes = {}
    count, k = {50: (60, 9), 100: (40, 7), 200: (24, 5), 500: (14, 3)}[scale]
    corpus = corpus_polyhedron(count, k)
    body.append(poly_solid("corpus", "cello", "spruce-varnish", 0.42, corpus))
    thermal.append(copy_as(body[-1], "th-corpus", region="cello-corpus"))
    if scale < 500:
        neck = loft_u("neck", "cello", "maple-varnish", 0.38, neck_sections(scale), 2.4)
        body.append(neck)
        thermal.append(copy_as(neck, "th-neck", region="cello-neck"))
        structure.append({"id": "st-neck", "of": "neck", "member": "cello", "part": "member"})
        fb = loft_u("fingerboard", "cello", "ebony", 0.08, fingerboard_sections(scale), 3.0)
        body.append(fb)
        thermal.append(copy_as(fb, "th-fingerboard", region="cello-fingerboard"))
        structure.append({"id": "st-fingerboard", "of": "fingerboard", "member": "cello", "part": "member"})
        if scale < 200:
            pb = loft_u("pegbox", "cello", "maple-varnish", 0.36, pegbox_sections(scale), 3.0)
            body.append(pb)
            thermal.append(copy_as(pb, "th-pegbox", region="cello-neck"))
            prof = [(-0.024, 0.0), (-0.024, SCROLL_R - 0.006), (-0.019, SCROLL_R), (0.019, SCROLL_R), (0.024, SCROLL_R - 0.006), (0.024, 0.0)]
            body.append(revolve_w("scroll", "cello", "maple-varnish", 0.34, prof, (SCROLL_C[0], SCROLL_C[1], 0.0)))
            thermal.append(copy_as(body[-1], "th-scroll", region="cello-neck"))
            for name, u, side in (("a", 1.075, -1), ("g", 1.104, 1), ("d", 1.133, -1), ("c", 1.160, 1)):
                vpeg = fb_top(u) + 0.030
                body.append(capsule(f"peg-{name}-shaft", "cello", "ebony", 0.1, (u, vpeg, -side * 0.020), (u, vpeg, side * 0.050), 0.0045 if scale == 50 else 0.0075))
                body.append(ellipsoid(f"peg-{name}-head", "cello", "ebony", 0.1, (u, vpeg, side * 0.064), (0.016, 0.006 if scale == 50 else 0.0076, 0.016)))
                thermal.append(copy_as(body[-2], f"th-peg-{name}-shaft", region="cello-fittings"))
                thermal.append(copy_as(body[-1], f"th-peg-{name}-head", region="cello-fittings"))
        else:
            secs = [(NUT_U - 0.004, fb_top(NUT_U) + 0.031, 0.032, 0.0205), (1.150, fb_top(1.15) + 0.030, 0.031, 0.0225), (1.225, SCROLL_C[1], 0.022, 0.0225)]
            body.append(loft_u("pegbox", "cello", "maple-varnish", 0.36, secs, 2.4))
            thermal.append(copy_as(body[-1], "th-pegbox", region="cello-neck"))
        if scale < 200:
            body.append(loft_u("tailpiece", "cello", "ebony", 0.1, tail_sections(scale), 4.0))
            thermal.append(copy_as(body[-1], "th-tailpiece", region="cello-fittings"))
            structure.append({"id": "st-tailpiece", "of": "tailpiece", "member": "cello", "part": "member"})
            body.append(capsule("tailcord", "cello", "ebony", 0.12, (0.010, -0.045, 0.0), (TAIL_U0 + 0.004, -0.072, 0.0), 0.004 if scale == 50 else 0.0075))
            thermal.append(copy_as(body[-1], "th-tailcord", region="cello-fittings"))
            structure.append({"id": "st-tailcord", "of": "tailcord", "member": "cello", "part": "member"})
            h = 0.012 if scale == 50 else 0.015
            body.append(revolve_u("endpin-collar", "cello", "ebony", 0.12, [(0.0, 0.0), (0.0, 0.016), (h, 0.016), (h, 0.0)], (BODY_U0 - h, 0.0, 0.0)))
            thermal.append(copy_as(body[-1], "th-endpin-collar", region="cello-fittings"))
            structure.append({"id": "st-endpin-collar", "of": "endpin-collar", "member": "endpin", "part": "joint"})
        if scale == 50:
            for i, (name, wt, wn, r) in enumerate(STRINGS):
                body.append(box(f"tuner-{name}", "cello", "nickel-steel", 0.55, (0.236, -0.1405, wt * 0.62), (0.020, 0.013, 0.0085)))
                thermal.append(copy_as(body[-1], f"th-tuner-{name}", region="cello-fittings"))
            body.append(box("nut", "cello", "ebony", 0.1, (NUT_U - 0.001, fb_top(NUT_U) - 0.002, 0.0), (0.008, 0.012, 0.026)))
            thermal.append(copy_as(body[-1], "th-nut", region="cello-fittings"))
            body.append(box("saddle", "cello", "ebony", 0.1, (BODY_U0 + 0.004, -0.063, 0.0), (0.008, 0.010, 0.040)))
            thermal.append(copy_as(body[-1], "th-saddle", region="cello-fittings"))
        if scale < 500:
            bp = bridge_profile(scale)
            body.append(extrude("bridge", "cello", "maple-bridge", 0.6, bp, "yz", BRIDGE_U - 0.002, BRIDGE_U + 0.002, sheet=True))
            thermal.append(copy_as(body[-1], "th-bridge", region="cello-bridge"))
            structure.append({"id": "st-bridge", "of": "bridge", "member": "cello", "part": "member"})
            for name, wb, wn, r in STRINGS:
                vb = string_v(BRIDGE_U, wb)
                ta = tail_anchor(wb)
                body.append(capsule(f"string-{name}", "cello", "steel-string", 0.7, (BRIDGE_U, vb - r, wb), (NUT_U, fb_top(NUT_U) - 0.002 - r, wn), r, slender=True))
                thermal.append(copy_as(body[-1], f"th-string-{name}", region="cello-strings"))
                structure.append({"id": f"st-string-{name}", "of": f"string-{name}", "member": "cello", "part": "member"})
                if scale < 200:
                    body.append(capsule(f"after-{name}", "cello", "steel-string", 0.7, (ta[0], ta[1], wb * 0.62), (BRIDGE_U, vb - r, wb), r, slender=True))
                    thermal.append(copy_as(body[-1], f"th-after-{name}", region="cello-strings"))
    else:
        body.append(capsule("neck", "cello", "maple-varnish", 0.38, (0.745, -0.050, 0.0), (1.215, 0.004, 0.0), 0.0115, slender=True))
        thermal.append(copy_as(body[-1], "th-neck", region="cello-neck"))
        structure.append({"id": "st-neck", "of": "neck", "member": "cello", "part": "member"})
        thermal.append(capsule("th-fingerboard", "cello", "ebony", 0.08, (0.60, fb_top(0.60) + 0.006, 0.0), (0.95, fb_top(0.95) + 0.006, 0.0), 0.012, slender=False, region="cello-fingerboard"))
        thermal.append(box("th-fittings", "cello", "ebony", 0.1, (0.14, -0.10, 0.0), (0.20, 0.024, 0.050), region="cello-fittings"))
        thermal.append(extrude("th-bridge", "cello", "maple-bridge", 0.6, bridge_profile(200), "yz", BRIDGE_U - 0.002, BRIDGE_U + 0.002, sheet=True, region="cello-bridge"))
        thermal.append(capsule("th-strings", "cello", "steel-string", 0.7, (BRIDGE_U, -0.175, 0.0), (NUT_U, fb_top(NUT_U) - 0.003, 0.0), 0.002, slender=True, region="cello-strings"))
    if scale == 200:
        thermal.append(loft_u("th-tailpiece", "cello", "ebony", 0.1, tail_sections(scale), 4.0, region="cello-fittings"))
    if scale == 50:
        for side in ("l", "r"):
            fh = fhole(side, scale)
            vtop = top_v(0.37, 0.060) + 0.0012
            body.append(extrude(f"fhole-{side}", "cello", "ebony", 0.02, fh, "zx", vtop - 0.0008, vtop + 0.0008, role="detail", texture=True))
        body.append(extrude("rosin-dust", "cello", "rosin", 0.85, [(-0.040, 0.385), (0.040, 0.385), (0.040, 0.455), (-0.040, 0.455)], "zx", top_v(0.41, 0.0) - 0.0006, top_v(0.41, 0.0) + 0.0004, role="texture", texture=True))
        body.append(extrude("varnish-wear-shoulder", "cello", "spruce-bare", 0.68, [(0.110, 0.640), (0.165, 0.660), (0.150, 0.735), (0.095, 0.715)], "zx", top_v(0.66, 0.13) - 0.0006, top_v(0.66, 0.13) + 0.0004, role="texture", texture=True))
        sc = [(BODY_U0 + 0.150, 0.0400), (BODY_U0 + 0.290, 0.0400), (BODY_U0 + 0.290, -0.0400), (BODY_U0 + 0.150, -0.0400)]
        body.append(extrude("rib-scratch", "cello", "maple-bare", 0.72, sc, "xy", -B(0.22) - 0.0010, -B(0.22) - 0.0002, role="texture", texture=True))
    endpin_parts(scale, body, thermal, structure)
    stopper_parts(scale, body, thermal, structure)
    if scale < 500:
        tp = plate_polyhedron({50: 60, 100: 40, 200: 24}[scale], {50: 9, 100: 7, 200: 5}[scale], True, 0.004)
        bp = plate_polyhedron({50: 60, 100: 40, 200: 24}[scale], {50: 9, 100: 7, 200: 5}[scale], False, 0.004)
        structure.append(poly_solid("st-top-plate", "cello", "spruce-varnish", 0.5, tp, member="cello", part="member"))
        structure.append(poly_solid("st-back-plate", "cello", "maple-varnish", 0.5, bp, member="cello", part="member"))
        volumes["top-plate"] = tp[3]
        volumes["back-plate"] = bp[3]
    face = outline_face({50: 60, 100: 40, 200: 24, 500: 14}[scale])
    inner = outline_face({50: 60, 100: 40, 200: 24, 500: 14}[scale], 0.0014 if scale < 500 else 0.004)
    inner = [(u, w) for u, w in inner if BODY_U0 + 0.0015 < u < BODY_U0 + BODY_L - 0.0015]
    structure.append(extrude("st-ribs", "cello", "maple-varnish", 0.5, [(w, u) for u, w in face], "zx", -RIB, RIB, holes=[[(w, u) for u, w in inner]], member="cello", part="member"))
    if scale < 500:
        bar = box("st-bass-bar", "cello", "spruce-varnish", 0.5, (BODY_U0 + 0.38, top_v(0.38, 0.035) + 0.004 + 0.0125, 0.035), (0.60, 0.025, 0.012 if scale == 50 else FEATURE[scale]), member="cello", part="member")
        structure.append(bar)
        sp_u = BRIDGE_U - 0.012
        structure.append(capsule("st-sound-post", "cello", "spruce-varnish", 0.5, (sp_u, top_v(sp_u - BODY_U0, -0.030) + 0.004, -0.030), (sp_u, back_v(sp_u - BODY_U0, -0.030) - 0.004, -0.030), 0.0055, slender=True, member="cello", part="member"))
        for name, u0, u1 in (("lower-block", BODY_U0, BODY_U0 + 0.040), ("upper-block", BODY_U0 + BODY_L - 0.045, BODY_U0 + BODY_L)):
            structure.append(box(f"st-{name}", "cello", "spruce-varnish", 0.5, ((u0 + u1) / 2, 0.0, 0.0), (u1 - u0, 2 * RIB, 0.080), member="cello", part="member"))
        structure.append(ellipsoid("st-neck-joint", "cello", "maple-varnish", 0.3, (0.767, -0.040, 0.0), (0.02, 0.03, 0.022), member="cello", part="joint"))
    else:
        structure.append(box("st-lower-block", "cello", "spruce-varnish", 0.5, (BODY_U0 + 0.040, 0.0, 0.0), (0.080, 2 * RIB, 0.080), member="cello", part="member"))
    return body, thermal, structure, volumes


def endpin_parts(scale, body, thermal, structure):
    """📌 The endpin rod (steel, 10 mm, 500 mm) with its turned point."""
    if scale == 50:
        body.append(revolve_u("endpin-point", "endpin", "endpin-steel", 0.55, [(0.0, 0.0), (0.028, ENDPIN_R), (0.028, 0.0)]))
        thermal.append(copy_as(body[-1], "th-endpin-point", region="endpin"))
        body.append(capsule("endpin-rod", "endpin", "endpin-steel", 0.6, (0.028, 0.0, 0.0), (ENDPIN_LEN, 0.0, 0.0), ENDPIN_R, slender=True))
    else:
        body.append(capsule("endpin-rod", "endpin", "endpin-steel", 0.6, (ENDPIN_R, 0.0, 0.0), (ENDPIN_LEN, 0.0, 0.0), ENDPIN_R, slender=True))
    thermal.append(copy_as(body[-1], "th-endpin-rod", region="endpin"))
    structure.append({"id": "st-endpin-rod", "of": "endpin-rod", "member": "endpin", "part": "member"})


def stopper_parts(scale, body, thermal, structure):
    """🪨 The endpin stopper: a rosewood disc of 85 mm with a rubber sole and a cup for the endpin point."""
    if scale == 50:
        prof = [(0.0, 0.0), (0.0, STOPPER_R), (0.003, STOPPER_R), (0.016, STOPPER_R), (STOPPER_H, STOPPER_R - 0.003), (STOPPER_H, 0.013), (0.011, 0.007), (0.011, 0.0)]
        body.append(revolve_u("stopper-disc", "stopper", "rosewood", 0.3, prof))
    elif scale == 100:
        body.append(revolve_u("stopper-disc", "stopper", "rosewood", 0.3, [(0.0, 0.0), (0.0, STOPPER_R), (STOPPER_H, STOPPER_R - 0.003), (STOPPER_H, 0.0)]))
    if scale < 200:
        thermal.append(copy_as(body[-1], "th-stopper-disc", region="stopper"))
        structure.append({"id": "st-stopper-disc", "of": "stopper-disc", "member": "stopper", "part": "member"})
    elif scale == 200:
        thermal.append(revolve_u("th-stopper-disc", "stopper", "rosewood", 0.3, [(0.0, 0.0), (0.0, STOPPER_R), (0.030, STOPPER_R - 0.003), (0.030, 0.0)], region="stopper"))
    else:
        thermal.append(revolve_u("th-stopper-disc", "stopper", "rosewood", 0.3, [(0.0, 0.0), (0.0, STOPPER_R), (0.003, STOPPER_R), (0.003, 0.0)], region="stopper", sheet=True))


def bow_parts(scale):
    """🏹 The bow (pernambuco stick, ebony frog, horsehair) in bow-local coordinates, and the lid spinners that hold it in the case."""
    body, thermal, structure = [], [], []
    us = [BOW_FROG + 0.005, -0.33, -0.20, -0.06, 0.08, 0.20, BOW_TIP - 0.004]
    secs = [(u, stick_v(u), stick_r(u), stick_r(u)) for u in us]
    body.append(loft_u("bow-stick", "bow", "pernambuco", 0.3, secs, 2.0, slender=True))
    thermal.append(copy_as(body[-1], "th-bow-stick", region="bow-stick"))
    structure.append({"id": "st-bow-stick", "of": "bow-stick", "member": "bow", "part": "member"})
    if scale < 500:
        body.append(box("bow-hair", "bow", "horsehair", 0.85, ((-0.378 + 0.287) / 2, 0.0005, 0.0), (0.665, 0.001, 0.012), slender=True))
        thermal.append(copy_as(body[-1], "th-bow-hair", region="bow-hair"))
        structure.append({"id": "st-bow-hair", "of": "bow-hair", "member": "bow", "part": "member"})
    else:
        thermal.append(box("th-bow-hair", "bow", "horsehair", 0.85, ((-0.378 + 0.287) / 2, 0.0005, 0.0), (0.665, 0.001, 0.012), slender=True, region="bow-hair"))
    if scale < 200:
        v0, v1 = stick_v(-0.38) + stick_r(-0.38) * 0.5, 0.0035
        body.append(box("bow-frog", "bow", "ebony", 0.06, (-0.380, (v0 + v1) / 2, 0.0), (0.047, v1 - v0, 0.015), corner=0.003))
        thermal.append(copy_as(body[-1], "th-bow-frog", region="bow-frog"))
        structure.append({"id": "st-bow-frog", "of": "bow-frog", "member": "bow", "part": "joint"})
    else:
        thermal.append(box("th-bow-frog", "bow", "ebony", 0.06, (-0.380, -0.010, 0.0), (0.047, 0.025, 0.015), region="bow-frog"))
    if scale == 50:
        body.append(box("bow-head", "bow", "pernambuco", 0.28, (0.297, -0.0095, 0.0), (0.026, 0.023, 0.0092), corner=0.002))
        thermal.append(copy_as(body[-1], "th-bow-head", region="bow-stick"))
        body.append(capsule("bow-button", "bow", "ebony", 0.1, (BOW_FROG + 0.004, stick_v(BOW_FROG), 0.0), (BOW_FROG + 0.020, stick_v(BOW_FROG), 0.0), 0.0046))
        thermal.append(copy_as(body[-1], "th-bow-button", region="bow-frog"))
        body.append(loft_u("bow-lapping", "bow", "leather-wrap", 0.2, [(u, stick_v(u), stick_r(u) + 0.0009, stick_r(u) + 0.0009) for u in (-0.352, -0.325, -0.298)], 2.0, role="detail"))
        thermal.append(copy_as(body[-1], "th-bow-lapping", region="bow-stick"))
    return body, thermal, structure


def interior_parts(scale):
    """🛏️ The padded interior of the case that carries the cello and the bow: lower-bout cradle, endpin-pocket blocks, neck rest, bow spinners."""
    body, structure = [], []
    f = FEATURE[scale]
    xc0 = CXC - back_v(0.328, 0.0)
    body.append(box("pad-cradle", "case", "foam-lining", 0.35, (CYC + BODY_U0 + 0.328, -((case_xb(0.40) + WALL) + xc0 + 0.0004) / 2, 0.0), (0.24, abs((case_xb(0.40) + WALL) - xc0) + 0.0004 if scale < 500 else 0.004, 0.10), sheet=True))
    structure.append({"id": "st-pad-cradle", "of": "pad-cradle", "member": "case", "part": "member"})
    inner_bottom = 0.006 + WALL
    if scale < 500:
        for side, sgn in (("l", 1), ("r", -1)):
            ws = [0.030 + 0.008 * i for i in range(11)]
            top = []
            for w in ws:
                s0 = next(s for s in [i * 0.0005 for i in range(200)] if B(s) >= w)
                top.append((sgn * w, CYC + BODY_U0 + s0 + 0.0005))
            prof = [(sgn * ws[0], inner_bottom), (sgn * ws[-1], inner_bottom)] + list(reversed(top))
            body.append(extrude(f"pad-pocket-{side}", "case", "foam-lining", 0.35, prof, "zx", -(CXC + RIB), -(CXC - RIB)))
            structure.append({"id": f"st-pad-pocket-{side}", "of": f"pad-pocket-{side}", "member": "case", "part": "member"})
    else:
        body.append(box("pad-pocket", "case", "foam-lining", 0.35, ((inner_bottom + 0.090) / 2, -CXC, 0.0), (0.090 - inner_bottom, 2 * RIB, 0.22)))
        structure.append({"id": "st-pad-pocket", "of": "pad-pocket", "member": "case", "part": "member"})
    nx0 = case_xb(0.94) + WALL - 0.0005
    nx1 = CXC + 0.0215
    nx1 = max(nx1, nx0 + f + 0.001)
    body.append(box("pad-neck-rest", "case", "foam-lining", 0.35, (0.94, -(nx0 + nx1) / 2, 0.0), (0.080, nx1 - nx0, max(0.050, f))))
    structure.append({"id": "st-pad-neck-rest", "of": "pad-neck-rest", "member": "case", "part": "member"})
    if scale <= 200:
        from author_model import lid_local, case_lid_x, cello_to_world
        for name, u in (("lo", -0.30), ("hi", 0.20)):
            c = bow_to_cello((u, stick_v(u) - stick_r(u), 0.0))
            X, Y, Z = cello_to_world(c)
            xin = section_x_lid(Y, Z)
            size = max(0.012, f)
            centre = lid_local(((X + xin) / 2, Y, Z))
            body.append(box(f"spinner-{name}", "case-lid", "nickel-steel", 0.5, centre, (size, (xin - X) + 0.004, size), corner=0.002))
            structure.append({"id": f"st-spinner-{name}", "of": f"spinner-{name}", "member": "case-lid", "part": "member"})
    return body, structure


def section_x_lid(y, z):
    """📏 The x of the inner lid surface at height y and lateral z."""
    from author_geom import section_x
    from author_model import CASE_N
    from author_model import CASE_BAND
    return section_x(XP, case_xf(y) - WALL, case_b(y) - WALL, CASE_N, z, CASE_BAND)
