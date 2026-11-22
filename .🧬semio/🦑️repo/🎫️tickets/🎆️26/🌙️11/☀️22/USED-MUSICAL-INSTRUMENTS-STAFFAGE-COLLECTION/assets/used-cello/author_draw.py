"""🖊️ The 2D drawings of the used cello asset: side elevation, front elevation and plan per scale, drawn to the numbers of `author_model.py`.

Elevation shapes are in the local (u, v) frame of their bone; front shapes in rest (z, y); plan shapes in rest (x, z). Cello, bow and endpin
parts carry `solid` placements in front and plan so that they read right when the cello leans and the bow crosses the strings.

@see ./author_parts.py — the 3D solids drawn here
"""
import math
from author_geom import circle, pt, r6, simplify
from author_model import (CASE_BAND, B, BODY_L, BODY_U0, BOW_FROG, BOW_TIP, BRIDGE_U, CASE_N, CASE_STATIONS, CXC, CYC, ENDPIN_LEN, ENDPIN_R, ENDPIN_REST_TIP,
                          FB_END_U, LID_ORIGIN, NUT_U, RIB, SCROLL_TOP, STRINGS, TAIL_U0, TAIL_U1, WALL, XP, back_v, case_b, case_back_x, case_xb,
                          case_xf, corpus_profile, corpus_stations, fb_top, outline_face, string_v, superellipse_half, top_v)
from author_parts import (BOW_PIVOT_Z, HANDLE_L, HANDLE_Y, HINGE_YS, LATCH_YS, SCROLL_C, SCROLL_R, STOPPER_C, STOPPER_H, STOPPER_R, STRAP_T,
                          STRAP_W, STRAP_Y0, STRAP_Y1, STRAP_Z, WHEEL_C, WHEEL_R, WHEEL_W, WHEEL_Z, case_stations, fingerboard_sections,
                          neck_sections, pegbox_sections, shape, stick_r, stick_v, strap_profile, tail_sections, bridge_profile)

CELLO_L = SCROLL_TOP


def taper(u0, w0, u1, w1):
    """📐 The lateral span (z0, z1, zTip) of a part whose half-width runs linearly from w0 at u0 to w1 at u1, extrapolated to the joint and the tip of the cello bone."""
    slope = (w1 - w0) / (u1 - u0)
    at0 = w0 - slope * u0
    atl = w0 + slope * (CELLO_L - u0)
    return {"z0": r6(-at0), "z1": r6(at0), "zTip": [r6(-atl), r6(atl)]}


def sweep_w(span):
    """🧊 An extruded solid hint swept laterally over `span`."""
    out = {"kind": "extruded", "sweep": "w"}
    out.update(span)
    return out


def side_poly(sections, front_offset=None):
    """📐 The side outline of a loft of (u, v-centre, half depth, half width) sections."""
    upper = [(u, vc - rx) for u, vc, rx, ry in sections]
    lower = [(u, vc + rx) for u, vc, rx, ry in reversed(sections)]
    return upper + lower


def face_poly(sections):
    """📐 The face outline of a loft of sections in rest front coordinates (z, y)."""
    right = [(-ry, CYC + u) for u, vc, rx, ry in sections]
    left = [(ry, CYC + u) for u, vc, rx, ry in reversed(sections)]
    return right + left


def case_drawing(scale):
    """🧳 The shapes of the case in the three views at a scale."""
    pens = {50: (0.5, 0.25, 0.13), 100: (0.35, 0.18, 0.13), 200: (0.25, 0.13, 0.1), 500: (0.18, 0.1, 0.1)}[scale]
    body, detail, texture = pens
    st = case_stations(scale)
    ev, fr, pl = [], [], []
    back = [(y, -xb) for y, b, xb, xf in st] + [(y, -XP) for y, b, xb, xf in reversed(st)]
    merged = {50: None, 100: None, 200: ["case-shell", "case-straps", "case-handle", "case-hardware"], 500: ["case-shell", "case-straps", "case-handle", "case-hardware", "case-wheels"]}[scale]
    ev.append(shape("case-back-shell", "case", "body", 3.0, True, 0.16, back, region=None if merged else "case-shell", regions=merged, material="abs-eps"))
    lid = [(y - LID_ORIGIN[1], 0.0) for y, b, xb, xf in st] + [(y - LID_ORIGIN[1], -(xf - XP)) for y, b, xb, xf in reversed(st)]
    ev.append(shape("case-lid-side", "case-lid", "body", 3.1, True, 0.2, lid, region="case-shell" if scale != 100 else None, regions=["case-shell", "case-hardware"] if scale == 100 else None, material="abs-eps"))
    face_t = max(0.012, 0.15 * scale / 1000)
    flat = [r for r in st if r[3] >= 0.143]
    skin = [(y - LID_ORIGIN[1], -(xf - XP) + face_t) for y, b, xb, xf in flat] + [(y - LID_ORIGIN[1], -(xf - XP)) for y, b, xb, xf in reversed(flat)]
    ev.append(shape("case-lid-shell", "case-lid", "body", 3.12, True, 0.2, skin, region="case-shell", material="abs-eps"))
    outline = [(-b, y) for y, b, xb, xf in st] + [(b, y) for y, b, xb, xf in reversed(st)]
    fr.append(shape("case-back-shell", "case", "body", 3.0, True, 0.16, outline, region="case-shell", material="abs-eps"))
    fr.append(shape("case-lid-shell", "case-lid", "body", 3.1, True, 0.2, [(z - LID_ORIGIN[2] + LID_ORIGIN[2], y) for z, y in outline], region="case-shell", material="abs-eps"))
    big = max(st, key=lambda r: r[1])
    count = {50: 24, 100: 16, 200: 12, 500: 8}[scale]
    bh = superellipse_half(XP, abs(big[2] - XP), big[1], CASE_N, count, -1, CASE_BAND)
    lh = superellipse_half(XP, abs(big[3] - XP), big[1], CASE_N, count, 1, CASE_BAND)
    pl.append(shape("case-back-shell", "case", "body", 3.0, True, 0.16, [(x, z) for x, z in bh], region="case-shell", material="abs-eps"))
    pl.append(shape("case-lid-shell", "case-lid", "body", 3.1, True, 0.2, [(x, z) for x, z in lh], region="case-shell", material="abs-eps"))
    if scale <= 200:
        top = st[-1]
        th = superellipse_half(XP, abs(top[2] - XP), top[1], CASE_N, 8, -1, CASE_BAND) + list(reversed(superellipse_half(XP, abs(top[3] - XP), top[1], CASE_N, 8, 1, CASE_BAND)))[1:-1]
        pl.append(shape("case-top-end", "case", "detail", 3.3, False, 0.3, th, pen=detail, region="case-shell"))
        neck_y = 1.0
        nb = case_b(neck_y)
        nh = superellipse_half(XP, abs(case_xb(neck_y) - XP), nb, CASE_N, 10, -1, CASE_BAND) + list(reversed(superellipse_half(XP, abs(case_xf(neck_y) - XP), nb, CASE_N, 10, 1, CASE_BAND)))[1:-1]
        pl.append(shape("case-neck-section", "case", "detail", 3.25, False, 0.3, nh, pen=detail, region="case-shell"))
        crown = [(-0.72 * b, y) for y, b, xb, xf in st[2:-2]] + [(0.72 * b, y) for y, b, xb, xf in reversed(st[2:-2])]
        fr.append(shape("lid-crown", "case-lid", "detail", 3.15, False, 0.3, crown, pen=detail, region="case-shell"))
        ev.append(shape("parting-line", "case", "detail", 3.2, False, 0.3, [(st[0][0], -XP), (st[-1][0], -XP)], closed=False, pen=detail, region="case-shell"))
    if scale <= 100:
        sp = strap_profile(-STRAP_Z, STRAP_Y0, STRAP_Y1, 14 if scale == 50 else 8)
        ev.append(shape("strap-r", "case", "body", 3.4, True, 0.3, sp, region="case-straps", material="webbing"))
        for side, z in (("l", STRAP_Z), ("r", -STRAP_Z)):
            zs = [z - STRAP_W / 2 + STRAP_W * i / 6 for i in range(7)]
            y = 0.40
            inner = [(case_back_x(y, zz), zz) for zz in zs]
            outer = [(case_back_x(y, zz) - STRAP_T, zz) for zz in reversed(zs)]
            pl.append(shape(f"strap-{side}", "case", "body", 2.9, True, 0.3, inner + outer, region="case-straps", material="webbing"))
        zr = -case_b(HANDLE_Y)
        ev.append(shape("handle-grip", "case", "body", 3.6, True, 0.12, [(HANDLE_Y - HANDLE_L / 2 - 0.0125, -XP - 0.0125), (HANDLE_Y + HANDLE_L / 2 + 0.0125, -XP - 0.0125), (HANDLE_Y + HANDLE_L / 2 + 0.0125, -XP + 0.0125), (HANDLE_Y - HANDLE_L / 2 - 0.0125, -XP + 0.0125)], region="case-handle", material="rubber-grip"))
        grip = [(zr - 0.0465, HANDLE_Y - HANDLE_L / 2 - 0.0125), (zr - 0.0215, HANDLE_Y - HANDLE_L / 2 - 0.0125), (zr - 0.0215, HANDLE_Y + HANDLE_L / 2 + 0.0125), (zr - 0.0465, HANDLE_Y + HANDLE_L / 2 + 0.0125)]
        fr.append(shape("handle-grip", "case", "body", 3.6, True, 0.12, grip, region="case-handle", material="rubber-grip"))
        pl.append(shape("handle-grip", "case", "body", 3.6, True, 0.12, [(XP - 0.0125, zr - 0.0465), (XP + 0.0125, zr - 0.0465), (XP + 0.0125, zr - 0.0215), (XP - 0.0125, zr - 0.0215)], region="case-handle", material="rubber-grip"))
        for k, dy in (("lo", -0.068), ("hi", 0.068)):
            yy = HANDLE_Y + dy
            fr.append(shape(f"handle-mount-{k}", "case", "body", 3.55, True, 0.14, [(zr - 0.035, yy - 0.011), (zr + 0.002, yy - 0.011), (zr + 0.002, yy + 0.011), (zr - 0.035, yy + 0.011)], region="case-handle", material="rubber-grip"))
        for i, y in enumerate(LATCH_YS):
            zl = case_b(y)
            t = 0.012 if scale == 50 else 0.015
            fr.append(shape(f"latch-{i + 1}", "case", "body", 3.6, True, 0.55, [(zl - 0.002, y - 0.03), (zl - 0.002 + t, y - 0.03), (zl - 0.002 + t, y + 0.03), (zl - 0.002, y + 0.03)], region="case-hardware", material="nickel-steel"))
            pl.append(shape(f"latch-{i + 1}", "case", "body", 3.6 + 0.01 * i, True, 0.55, [(XP - 0.02, zl - 0.002), (XP + 0.02, zl - 0.002), (XP + 0.02, zl - 0.002 + t), (XP - 0.02, zl - 0.002 + t)], region="case-hardware", material="nickel-steel"))
        if scale == 50:
            for i, y in enumerate(HINGE_YS):
                zr2 = -case_b(y)
                ev.append(shape(f"hinge-{i + 1}", "case", "body", 3.5, True, 0.5, [(y - 0.025, -XP - 0.012), (y + 0.025, -XP - 0.012), (y + 0.025, -XP + 0.012), (y - 0.025, -XP + 0.012)], region="case-hardware", material="nickel-steel"))
                fr.append(shape(f"hinge-{i + 1}", "case", "body", 3.5, True, 0.5, [(zr2 - 0.010, y - 0.025), (zr2 + 0.002, y - 0.025), (zr2 + 0.002, y + 0.025), (zr2 - 0.010, y + 0.025)], region="case-hardware", material="nickel-steel"))
    if scale >= 200:
        sp = strap_profile(-STRAP_Z, STRAP_Y0, STRAP_Y1, 4)
        ev.append(shape("straps", "case", "body", 3.4, True, 0.3, sp, region="case-straps", material="webbing"))
        zs = [-STRAP_Z - STRAP_W / 2 + (2 * STRAP_Z + STRAP_W) * i / 6 for i in range(7)]
        pl.append(shape("straps", "case", "body", 2.9, True, 0.3, [(case_back_x(0.40, zz), zz) for zz in zs] + [(case_back_x(0.40, zz) - STRAP_T, zz) for zz in reversed(zs)], region="case-straps", material="webbing"))
        zr = -case_b(HANDLE_Y)
        grip = [(zr - 0.046, HANDLE_Y - 0.0775), (zr + 0.002, HANDLE_Y - 0.0775), (zr + 0.002, HANDLE_Y + 0.0775), (zr - 0.046, HANDLE_Y + 0.0775)]
        fr.append(shape("handle-grip", "case", "body", 3.6, True, 0.12, grip, region="case-handle", material="rubber-grip"))
        pl.append(shape("handle-grip", "case", "body", 3.6, True, 0.12, [(XP - 0.015, zr - 0.046), (XP + 0.015, zr - 0.046), (XP + 0.015, zr + 0.002), (XP - 0.015, zr + 0.002)], region="case-handle", material="rubber-grip"))
    if scale <= 200:
        ww = WHEEL_W if scale < 200 else 0.030
        ev.append(shape("wheel-r", "case-wheels", "body", 3.7, True, 0.1, circle(0.0, 0.0, WHEEL_R, 24 if scale == 50 else 16), region="case-wheels", material="pu-wheel"))
        if scale == 50:
            ev.append(shape("wheel-hub-r", "case-wheels", "detail", 3.75, False, 0.4, circle(0.0, 0.0, 0.012, 12), pen=detail, region="case-wheels"))
        hx = WHEEL_C[0] + 0.006
        ev.append(shape("wheel-housing-r", "case", "body", 3.65, True, 0.2, [(0.012, -hx - 0.025), (0.068, -hx - 0.025), (0.068, -hx + 0.025), (0.012, -hx + 0.025)], region="case-shell", material="abs-eps"))
        for side, z in (("l", WHEEL_Z), ("r", -WHEEL_Z)):
            fr.append(shape(f"wheel-{side}", "case-wheels", "body", 2.5, True, 0.1, [(z - ww / 2, 0.0), (z + ww / 2, 0.0), (z + ww / 2, 2 * WHEEL_R), (z - ww / 2, 2 * WHEEL_R)], region="case-wheels", material="pu-wheel"))
        h = {50: 0.009, 100: 0.016, 200: 0.032}[scale]
        ev.append(shape("bumper-front", "case", "body", 3.7, True, 0.08, [(0.0, 0.080), (h, 0.080), (h, 0.110), (0.0, 0.110)], region="case-feet", material="rubber-bumper"))
        fr.append(shape("bumper-front", "case", "body", 3.7, True, 0.08, [(-0.07, 0.0), (0.07, 0.0), (0.07, h), (-0.07, h)], region="case-feet", material="rubber-bumper"))
    else:
        ev.append(shape("case-foot", "case", "body", 3.7, True, 0.1, [(0.0, -0.12), (0.08, -0.12), (0.08, 0.12), (0.0, 0.12)], region="case-feet", material="rubber-bumper"))
        fr.append(shape("case-foot", "case", "body", 3.7, True, 0.1, [(-0.13, 0.0), (0.13, 0.0), (0.13, 0.08), (-0.13, 0.08)], region="case-feet", material="rubber-bumper"))
    if scale == 50:
        fr.append(shape("sticker-round", "case-lid", "texture", 3.8, True, 0.85, circle(0.085, 0.31, 0.038, 20), pen=texture, region="case-shell"))
        fr.append(shape("sticker-label", "case-lid", "texture", 3.8, True, 0.9, [(-0.11, 0.60), (-0.02, 0.60), (-0.02, 0.655), (-0.11, 0.655)], pen=texture, region="case-shell"))
        for i, (z0, y0, z1, y1) in enumerate(((-0.14, 0.012, -0.10, 0.030), (0.10, 0.014, 0.14, 0.028), (-0.215, 0.09, -0.225, 0.16), (0.212, 0.10, 0.224, 0.17), (-0.07, 1.33, -0.03, 1.352))):
            fr.append(shape(f"scuff-{i + 1}", "case-lid", "texture", 3.85, False, 0.7, [(z0, y0), (z1, y1)], closed=False, pen=texture, region="case-shell"))
        for i, (y0, x0, y1, x1) in enumerate(((0.010, 0.10, 0.040, 0.115), (0.012, -0.10, 0.045, -0.118), (1.33, 0.06, 1.355, 0.072))):
            ev.append(shape(f"scuff-side-{i + 1}", "case", "texture", 3.85, False, 0.7, [(y0, -x0), (y1, -x1)], closed=False, pen=texture, region="case-shell"))
    order = {"case-back-shell": 0.5, "handle-grip": 0.7, "handle-mount-lo": 0.68, "handle-mount-hi": 0.68, "wheel-l": 0.6, "wheel-r": 0.6, "bumper-front": 0.75, "case-foot": 0.75}
    for sh in fr:
        if sh["id"] in order:
            sh["z"] = order[sh["id"]]
        elif sh["id"].startswith(("latch-", "hinge-")):
            sh["z"] = 0.8
    flat = [r for r in st if r[3] >= 0.143]
    face_v = -(flat[0][3] - XP)
    for sh in [x for x in fr if x["bone"] == "case-lid" and x["id"] not in ("case-lid-shell",)]:
        ys = [p[1] for p in sh["points"]]
        y0, y1 = max(min(ys), flat[0][0]), min(max(ys), flat[-1][0])
        if y1 <= y0:
            y0, y1 = flat[0][0], flat[-1][0]
        ev.append(shape(sh["id"], "case-lid", sh["role"], 3.13, False, sh["tone"], [(y0 - LID_ORIGIN[1], face_v), (y1 - LID_ORIGIN[1], face_v)], closed=False, pen=sh.get("penMm", texture), region="case-shell"))
    return ev, fr, pl


def cello_drawing(scale):
    """🎻 The shapes of the cello, endpin, stopper and bow in the three views at a scale (front and plan as solid placements)."""
    pens = {50: (0.35, 0.18, 0.13), 100: (0.25, 0.13, 0.1), 200: (0.18, 0.1, 0.1), 500: (0.13, 0.1, 0.1)}[scale]
    body, detail, texture = pens
    count = {50: 60, 100: 40, 200: 24, 500: 14}[scale]
    ev, fr, pl = [], [], []

    def both(s):
        fr.append(s)
        pl.append(dict(s))

    prof = corpus_profile(count)
    ev.append(shape("corpus", "cello", "body", 1.0, True, 0.42, prof, region="cello-corpus", material="spruce-varnish"))
    face = outline_face(count)
    vt, vb = top_v(0.3775, 0.0), back_v(0.3775, 0.0)
    fr.append(shape("corpus", "cello", "body", 1.0, True, 0.42, [(w, CYC + u) for u, w in face], region="cello-corpus", material="spruce-varnish"))
    pl.append(shape("corpus", "cello", "body", 1.0, True, 0.42, face, region="cello-corpus", material="spruce-varnish", solid_hint={"kind": "extruded", "sweep": "v", "z0": r6(vt), "z1": r6(vb)}))
    if scale <= 200:
        ev.append(shape("rib-top-edge", "cello", "detail", 1.05, False, 0.3, [(BODY_U0 + 0.004, -RIB), (BODY_U0 + BODY_L - 0.004, -RIB)], closed=False, pen=detail, region="cello-corpus"))
        ev.append(shape("rib-back-edge", "cello", "detail", 1.05, False, 0.3, [(BODY_U0 + 0.004, RIB), (BODY_U0 + BODY_L - 0.004, RIB)], closed=False, pen=detail, region="cello-corpus"))
    if scale < 500:
        ns = neck_sections(scale)
        ev.append(shape("neck", "cello", "body", 1.1, True, 0.38, side_poly(ns), region="cello-neck", material="maple-varnish"))
        both(shape("neck", "cello", "body", 1.1, True, 0.38, side_poly(ns), region="cello-neck", material="maple-varnish", solid_hint=sweep_w(taper(ns[1][0], ns[1][3], ns[-1][0], ns[-1][3]))))
        fs = fingerboard_sections(scale)
        ev.append(shape("fingerboard", "cello", "body", 1.3, True, 0.08, side_poly(fs), region="cello-fingerboard", material="ebony"))
        both(shape("fingerboard", "cello", "body", 1.3, True, 0.08, side_poly(fs), region="cello-fingerboard", material="ebony", solid_hint=sweep_w(taper(fs[0][0], fs[0][3], fs[-1][0], fs[-1][3]))))
        if scale < 200:
            ps = pegbox_sections(scale)
        else:
            ps = [(NUT_U - 0.004, fb_top(NUT_U) + 0.031, 0.032, 0.0205), (1.150, fb_top(1.15) + 0.030, 0.031, 0.0225), (1.225, SCROLL_C[1], 0.022, 0.0225)]
        ev.append(shape("pegbox", "cello", "body", 1.2, True, 0.36, side_poly(ps), region="cello-neck", material="maple-varnish"))
        both(shape("pegbox", "cello", "body", 1.2, True, 0.36, side_poly(ps), region="cello-neck", material="maple-varnish", solid_hint=sweep_w({"z0": -0.0225, "z1": 0.0225})))
        if scale < 200:
            sc = circle(SCROLL_C[0], SCROLL_C[1], SCROLL_R, 24 if scale == 50 else 16)
            ev.append(shape("scroll", "cello", "body", 1.25, True, 0.34, sc, region="cello-neck", material="maple-varnish"))
            both(shape("scroll", "cello", "body", 1.25, True, 0.34, sc, region="cello-neck", material="maple-varnish", solid_hint=sweep_w({"z0": -0.024, "z1": 0.024})))
            spiral = [(SCROLL_C[0] + (SCROLL_R - 0.004) * (1 - t / 10) * math.cos(2.6 * math.pi * t / 10 + 1.2), SCROLL_C[1] + (SCROLL_R - 0.004) * (1 - t / 10) * math.sin(2.6 * math.pi * t / 10 + 1.2)) for t in range(0, 9)]
            ev.append(shape("scroll-volute", "cello", "detail", 1.27, False, 0.3, spiral, closed=False, pen=detail, region="cello-neck"))
            for name, u, side in (("a", 1.075, -1), ("g", 1.104, 1), ("d", 1.133, -1), ("c", 1.160, 1)):
                vpeg = fb_top(u) + 0.030
                head = [(u + 0.016 * math.cos(a), vpeg + (0.006 if scale == 50 else 0.0076) * math.sin(a)) for a in [2 * math.pi * i / 12 for i in range(12)]]
                if side < 0:
                    ev.append(shape(f"peg-{name}", "cello", "body", 1.4, True, 0.1, head, region="cello-fittings", material="ebony"))
                zc = side * 0.064
                both(shape(f"peg-{name}", "cello", "body", 1.4, True, 0.1, head, region="cello-fittings", material="ebony", solid_hint=sweep_w({"z0": r6(min(zc - 0.016, -side * 0.020)) if side < 0 else r6(-0.020), "z1": r6(0.020) if side < 0 else r6(zc + 0.016)})))
            ts = tail_sections(scale)
            ev.append(shape("tailpiece", "cello", "body", 1.35, True, 0.1, side_poly(ts), region="cello-fittings", material="ebony"))
            both(shape("tailpiece", "cello", "body", 1.35, True, 0.1, side_poly(ts), region="cello-fittings", material="ebony", solid_hint=sweep_w(taper(ts[0][0], ts[0][3], ts[-1][0], ts[-1][3]))))
            h = 0.012 if scale == 50 else 0.015
            col = [(BODY_U0 - h, -0.016), (BODY_U0, -0.016), (BODY_U0, 0.016), (BODY_U0 - h, 0.016)]
            ev.append(shape("endpin-collar", "cello", "body", 1.15, True, 0.12, col, region="cello-fittings", material="ebony"))
            both(shape("endpin-collar", "cello", "body", 1.15, True, 0.12, [(BODY_U0 - h, 0.0), (BODY_U0 - h, 0.016), (BODY_U0, 0.016), (BODY_U0, 0.0)], region="cello-fittings", material="ebony", solid_hint={"kind": "revolved", "axisBone": "cello", "axis": "u", "axisX": 0.0, "mirror": True}))
        bp = [(BRIDGE_U - 0.002, -0.0828), (BRIDGE_U + 0.002, -0.0828), (BRIDGE_U + 0.001, -0.174), (BRIDGE_U - 0.001, -0.174)]
        ev.append(shape("bridge", "cello", "body", 1.45, True, 0.6, bp, region="cello-bridge", material="maple-bridge"))
        both(shape("bridge", "cello", "body", 1.45, True, 0.6, bp, region="cello-bridge", material="maple-bridge", solid_hint=sweep_w({"z0": -0.047, "z1": 0.047})))
        if scale <= 200:
            for name, wb, wn, r in STRINGS:
                vb = string_v(BRIDGE_U, wb)
                vn = fb_top(NUT_U) - 0.002
                line = [(BRIDGE_U, vb - r), (NUT_U, vn - r), (NUT_U + 0.004, vn - r + 0.0002), (BRIDGE_U - 0.0005, vb - r + 0.0002)]
                slope = (wn - wb) / (NUT_U - BRIDGE_U)
                at0 = wb - slope * BRIDGE_U
                atl = wb + slope * (CELLO_L - BRIDGE_U)
                if name == "a" or scale < 200:
                    ev.append(shape(f"string-{name}", "cello", "detail" if scale < 200 else "body", 1.5, False, 0.7, [(BRIDGE_U, vb - r), (NUT_U, vn - r)], closed=False, pen=texture if scale == 50 else detail, region="cello-strings"))
                both(shape(f"string-{name}", "cello", "detail" if scale < 200 else "body", 1.5, True, 0.7, [(BRIDGE_U, vb - 2 * r), (NUT_U, vn - 2 * r), (NUT_U, vn), (BRIDGE_U, vb)], pen=texture if scale == 50 else detail, region="cello-strings", solid_hint=sweep_w({"z0": r6(at0 - r), "z1": r6(at0 + r), "zTip": [r6(atl - r), r6(atl + r)]})))
                if scale < 200:
                    ta = (0.240, -0.151)
                    ev.append(shape(f"after-{name}", "cello", "detail", 1.5, False, 0.7, [ta, (BRIDGE_U, vb - r)], closed=False, pen=texture if scale == 50 else detail, region="cello-strings"))
    else:
        neck = [(0.745, -0.050 - 0.0115), (1.215, 0.004 - 0.0115), (1.215, 0.004 + 0.0115), (0.745, -0.050 + 0.0115)]
        ev.append(shape("neck", "cello", "body", 1.1, True, 0.38, neck, region="cello-neck", material="maple-varnish"))
        both(shape("neck", "cello", "body", 1.1, True, 0.38, neck, region="cello-neck", material="maple-varnish", solid_hint=sweep_w({"z0": -0.0115, "z1": 0.0115})))
    if scale == 50:
        for i, (u0, v0, u1, v1) in enumerate(((0.170, -0.030, 0.205, -0.012), (0.185, 0.004, 0.236, 0.018), (0.215, -0.020, 0.262, -0.026), (0.240, 0.010, 0.275, 0.030))):
            ev.append(shape(f"rib-scratch-{i + 1}", "cello", "texture", 1.08, False, 0.75, [(u0, v0), (u1, v1)], closed=False, pen=texture, region="cello-corpus"))
        wear = [(0.640 + BODY_U0, -RIB - 0.002), (0.735 + BODY_U0, -RIB - 0.002), (0.715 + BODY_U0, -RIB - 0.012), (0.650 + BODY_U0, -RIB - 0.014)]
        ev.append(shape("varnish-wear-shoulder", "cello", "texture", 1.09, True, 0.68, wear, pen=texture, region="cello-corpus"))
        for side, sgn in (("l", 1), ("r", -1)):
            fh = [(u, w) for w, u in (__import__("author_parts").fhole(side, 50))]
            both(shape(f"fhole-{side}", "cello", "detail", 1.6, True, 0.05, fh, pen=texture, region="cello-corpus", solid_hint={"kind": "extruded", "sweep": "v", "z0": r6(top_v(0.37, 0.06) - 0.001), "z1": r6(top_v(0.37, 0.06) + 0.001)}))
        rd = [(0.385, -0.040), (0.455, -0.040), (0.455, 0.040), (0.385, 0.040)]
        both(shape("rosin-dust", "cello", "texture", 1.42, True, 0.85, rd, pen=texture, region="cello-corpus", solid_hint={"kind": "extruded", "sweep": "v", "z0": r6(top_v(0.41, 0.0) - 0.001), "z1": r6(top_v(0.41, 0.0) + 0.001)}))
    ep = [(0.0, 0.0), (0.028, -ENDPIN_R), (ENDPIN_LEN, -ENDPIN_R), (ENDPIN_LEN, ENDPIN_R), (0.028, ENDPIN_R)] if scale < 200 else [(0.0, -ENDPIN_R), (ENDPIN_LEN, -ENDPIN_R), (ENDPIN_LEN, ENDPIN_R), (0.0, ENDPIN_R)]
    ev.append(shape("endpin", "endpin", "body", 0.9, True, 0.6, ep, region="endpin", material="endpin-steel", pen=None if scale < 200 else body))
    rev = [(0.0, 0.0), (0.028, ENDPIN_R), (ENDPIN_LEN, ENDPIN_R), (ENDPIN_LEN, 0.0)] if scale < 200 else [(0.0, 0.0), (0.0, ENDPIN_R), (ENDPIN_LEN, ENDPIN_R), (ENDPIN_LEN, 0.0)]
    both(shape("endpin", "endpin", "body", 0.9, True, 0.6, rev, region="endpin", material="endpin-steel", solid_hint={"kind": "revolved", "axisBone": "endpin", "axis": "u", "axisX": 0.0, "mirror": True}))
    if scale < 200:
        sp = [(0.0, -STOPPER_R), (STOPPER_H, -STOPPER_R + 0.003), (STOPPER_H, STOPPER_R - 0.003), (0.0, STOPPER_R)]
        ev.append(shape("stopper", "stopper", "body", 0.8, True, 0.3, sp, region="stopper", material="rosewood"))
        fr.append(shape("stopper", "stopper", "body", 0.8, True, 0.3, [(-STOPPER_R, STOPPER_C[1]), (STOPPER_R, STOPPER_C[1]), (STOPPER_R - 0.003, STOPPER_C[1] + STOPPER_H), (-STOPPER_R + 0.003, STOPPER_C[1] + STOPPER_H)], region="stopper", material="rosewood"))
        pl.append(shape("stopper", "stopper", "body", 0.8, True, 0.3, circle(STOPPER_C[0], 0.0, STOPPER_R, 24 if scale == 50 else 16), region="stopper", material="rosewood"))
    return ev, fr, pl


def bow_drawing(scale):
    """🏹 The shapes of the bow (bow-local side profile; front and plan as lateral sweeps placed by the bone)."""
    pens = {50: (0.25, 0.13, 0.13), 100: (0.18, 0.13, 0.1), 200: (0.13, 0.1, 0.1), 500: (0.13, 0.1, 0.1)}[scale]
    body, detail, texture = pens
    ev, fr, pl = [], [], []
    us = [BOW_FROG + 0.005 + (BOW_TIP - 0.009 - BOW_FROG) * i / 12 for i in range(13)]
    upper = [(u, stick_v(u) - stick_r(u)) for u in us]
    lower = [(u, stick_v(u) + stick_r(u)) for u in reversed(us)]
    stick = upper + lower
    hw = 0.0043
    for lst, view in ((ev, None), (fr, "f"), (pl, "p")):
        hint = None if view is None else sweep_w({"z0": r6(BOW_PIVOT_Z - hw), "z1": r6(BOW_PIVOT_Z + hw)})
        lst.append(shape("bow-stick", "bow", "body", 2.0, True, 0.3, stick, region="bow-stick", material="pernambuco", solid_hint=hint))
        if scale < 500:
            hair = [(-0.378, 0.0), (0.287, 0.0), (0.287, 0.001), (-0.378, 0.001)]
            hint = None if view is None else sweep_w({"z0": r6(BOW_PIVOT_Z - 0.006), "z1": r6(BOW_PIVOT_Z + 0.006)})
            if view is None:
                lst.append(shape("bow-hair", "bow", "detail", 2.05, False, 0.85, [(-0.378, 0.0005), (0.287, 0.0005)], closed=False, pen=detail, region="bow-hair"))
            else:
                lst.append(shape("bow-hair", "bow", "detail", 2.05, True, 0.85, hair, pen=detail, region="bow-hair", solid_hint=hint))
        if scale < 200:
            v0, v1 = stick_v(-0.38) + stick_r(-0.38) * 0.5, 0.0035
            frog = [(-0.4035, v0), (-0.3565, v0), (-0.3565, v1), (-0.4035, v1)]
            hint = None if view is None else sweep_w({"z0": r6(BOW_PIVOT_Z - 0.0075), "z1": r6(BOW_PIVOT_Z + 0.0075)})
            lst.append(shape("bow-frog", "bow", "body", 2.1, True, 0.06, frog, region="bow-frog", material="ebony", solid_hint=hint))
        if scale == 50:
            head = [(0.284, -0.021), (0.310, -0.021), (0.310, 0.0020), (0.284, 0.0020)]
            hint = None if view is None else sweep_w({"z0": r6(BOW_PIVOT_Z - 0.0046), "z1": r6(BOW_PIVOT_Z + 0.0046)})
            lst.append(shape("bow-head", "bow", "body", 2.1, True, 0.28, head, region="bow-stick", material="pernambuco", solid_hint=hint))
    return ev, fr, pl
