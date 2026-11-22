"""🏋️ Authoritative author script of `used-outdoor-fitness-station`: states every hand-chosen number of the used municipal calisthenics station (pull-up frame with a high and a low bar, parallel dip bars, decline sit-up bench, all on buried concrete point and strip footings) in world rest metres and writes the asset document `🔣️.json` next to it.

World frame of the staffage: x ahead (from the pull-up frame along the row to the sit-up bench), y up (y = 0 the loose-gravel impact surface), z left (the side elevation looks from -z).

@see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md
@see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts
@see ../used-chess-table/author-chess-table.py
"""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
SLUG = "used-outdoor-fitness-station"
G = 9.80665
SCALES = (50, 100, 200, 500)
FEAT = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}


def T(en, de):
    """🗣️ A bilingual text, English first."""
    return {"en": en, "de": de}


def r6(v):
    """✂️ Rounds a number for the document."""
    return round(v + 0.0, 6)


FT, FB = -0.10, -0.80
PA_X, PB_X = 0.0, 1.40
P_D, P_T = 0.1143, 0.0036
P_R = P_D / 2
P_BOT, P_TOP = -0.60, 2.50
CAP_SK, CAP_H, CAP_LIP = 0.015, 0.020, 0.002
FA = 0.50
CL_IN, CL_OUT, CL_H, CL_W = 0.045, 0.105, 0.09, 0.07
BAR_D, BAR_T = 0.0337, 0.0032
BAR_R = BAR_D / 2
BAR_HI, BAR_LO = 2.35, 1.00
BAR_X0, BAR_X1 = PA_X + CL_OUT, PB_X - CL_OUT
DIP_XF, DIP_XR, DIP_Z = 2.10, 3.50, 0.26
DIP_D, DIP_T = 0.0483, 0.0032
DIP_R = DIP_D / 2
DIP_Y, DIP_X0, DIP_X1, DIP_BOT = 1.25, 1.95, 3.65, -0.50
DIP_FX, DIP_FZ = 0.40, 0.92
B_X0, B_Y0, B_RUN, B_RISE = 4.30, 0.355, 1.30, 0.40
TH = math.degrees(math.atan2(B_RISE, B_RUN))
B_L = math.hypot(B_RUN, B_RISE)
CS, SN = math.cos(math.radians(TH)), math.sin(math.radians(TH))
SLAT_W, SLAT_T, SLAT_GAP = 0.095, 0.045, 0.012
SLAT_Z = (-(SLAT_W + SLAT_GAP), 0.0, SLAT_W + SLAT_GAP)
BOARD_HALF = 1.5 * SLAT_W + SLAT_GAP
BF_D, BF_T = 0.0424, 0.0026
BF_R = BF_D / 2
U_LOW, U_HIGH = 0.12, 1.22
LOW_Z, HIGH_Z = 0.12, 0.19
FOOT_Y = 0.93
FOOT_HALF = HIGH_Z + BF_R
ROLL_R, ROLL_HALF = 0.055, 0.15
B_BOT = -0.50
BFL_X, BFL_Z, BFH_X, BFH_Z = 0.30, 0.50, 0.30, 0.60
RHO = {"steel": 7850, "concrete": 2400, "robinia": 740, "epdm": 520}


def rr(r, s):
    """📏️ The radius of a tube at a scale: true down to 1:200, the 75 mm print floor at 1:500."""
    return max(r, 0.0375) if s == 500 else r


def board_vb(s):
    """🪵️ The underside offset of the board along its normal: 0, or 30 mm deeper at 1:500 where the board is 75 mm."""
    return -0.03 if s == 500 else 0.0


def cross_centre(u, s):
    """⭕️ World centre of a lateral cross tube tangent to the board underside at the distance `u` along the board."""
    r = rr(BF_R, s)
    vb = board_vb(s)
    x = B_X0 + u * CS + (vb - r) * -SN
    y = B_Y0 + u * SN + (vb - r) * CS
    return (x, y)


XL, YL = cross_centre(U_LOW, 50)
XH, YH = cross_centre(U_HIGH, 50)

BONES = {
    "ground": {"o": (0.0, 0.0), "a": 0, "p": None},
    "footing-a": {"o": (PA_X, FB), "a": 90, "p": "ground"},
    "footing-b": {"o": (PB_X, FB), "a": 90, "p": "ground"},
    "footing-dip-front": {"o": (DIP_XF, FB), "a": 90, "p": "ground"},
    "footing-dip-rear": {"o": (DIP_XR, FB), "a": 90, "p": "ground"},
    "footing-bench-low": {"o": (XL, FB), "a": 90, "p": "ground"},
    "footing-bench-high": {"o": (XH, FB), "a": 90, "p": "ground"},
    "post-a": {"o": (PA_X, P_BOT), "a": 90, "p": "footing-a"},
    "post-b": {"o": (PB_X, P_BOT), "a": 90, "p": "footing-b"},
    "bar-high": {"o": (BAR_X0, BAR_HI), "a": 0, "p": "post-a"},
    "bar-low": {"o": (BAR_X0, BAR_LO), "a": 0, "p": "post-a"},
    "dip-post-front": {"o": (DIP_XF, DIP_BOT), "a": 90, "p": "footing-dip-front"},
    "dip-post-rear": {"o": (DIP_XR, DIP_BOT), "a": 90, "p": "footing-dip-rear"},
    "dip-bar-l": {"o": (DIP_X0, DIP_Y), "a": 0, "p": "dip-post-front", "z": DIP_Z},
    "dip-bar-r": {"o": (DIP_X0, DIP_Y), "a": 0, "p": "dip-post-front", "z": -DIP_Z},
    "bench-frame-low": {"o": (XL, B_BOT), "a": 90, "p": "footing-bench-low"},
    "bench-frame-high": {"o": (XH, B_BOT), "a": 90, "p": "footing-bench-high"},
    "bench-board": {"o": (B_X0, B_Y0), "a": TH, "p": "bench-frame-low"},
}


def frame(b):
    """🧭️ Origin, cosine and sine of the rest frame of a bone in the world."""
    ox, oy = BONES[b]["o"]
    a = math.radians(BONES[b]["a"])
    return ox, oy, math.cos(a), math.sin(a)


def loc(b, x, y):
    """📐️ World rest (x, y) to bone-local (u, v)."""
    ox, oy, c, s = frame(b)
    dx, dy = x - ox, y - oy
    return (r6(dx * c + dy * s), r6(-dx * s + dy * c))


def ldir(b, dx, dy):
    """➡️ A world direction (x, y) in bone-local (u, v)."""
    _, _, c, s = frame(b)
    return (dx * c + dy * s, -dx * s + dy * c)


def loc3(b, x, y, z):
    """📍️ World rest point to bone-local (u, v, w)."""
    u, v = loc(b, x, y)
    return [u, v, r6(z - BONES[b].get("z", 0.0))]


# ---------------------------------------------------------------- solids


def base(id_, bone, mat, tone, role="body", **kw):
    """🧱️ The common fields of a solid."""
    s = {"id": id_, "bone": bone, "material": mat, "role": role, "tone": tone}
    s.update(kw)
    return s


def wbox(id_, bone, mat, tone, c, size, role="body", corner=None, **kw):
    """⬛️ A box from its world centre and world extents on a bone resting at 0 or 90 degrees."""
    sx, sy, sz = size
    lsize = [r6(sx), r6(sy), r6(sz)] if BONES[bone]["a"] == 0 else [r6(sy), r6(sx), r6(sz)]
    s = dict(base(id_, bone, mat, tone, role, **kw), kind="box", size=lsize, transform={"translation": loc3(bone, *c)})
    if corner:
        s["cornerRadius"] = corner
    return s


def lbox(id_, bone, mat, tone, c, size, role="body", **kw):
    """▪️ A box from its bone-local centre and bone-local extents."""
    return dict(base(id_, bone, mat, tone, role, **kw), kind="box", size=[r6(v) for v in size], transform={"translation": [r6(v) for v in c]})


def cyl(id_, bone, mat, tone, p0, d, length, r, role="body", prof=None, **kw):
    """🛢️ A solid of revolution from the world point `p0` along the world unit direction `d`; `prof` is [(axial, radial)], default a plain cylinder of `length` and radius `r`."""
    a = ldir(bone, d[0], d[1])
    profile = prof if prof is not None else [(0, 0), (0, r), (length, r), (length, 0)]
    return dict(base(id_, bone, mat, tone, role, **kw), kind="revolve", profile=[[r6(p), r6(q)] for p, q in profile], axis=[r6(a[0]), r6(a[1]), r6(d[2])], origin=loc3(bone, *p0))


def ring(id_, bone, mat, tone, p0, d, a0, a1, r0, r1, **kw):
    """💍️ A thin texture band around an axis (closed profile loop from axial `a0` to `a1`, radial `r0` to `r1`)."""
    return cyl(id_, bone, mat, tone, p0, d, 0, 0, role="texture", prof=[(a0, r0), (a1, r0), (a1, r1), (a0, r1), (a0, r0)], texture=True, **kw)


def ell(id_, bone, mat, tone, c, radii, role="texture", **kw):
    """🥚️ An ellipsoid from its world centre and bone-local semi-axes."""
    return dict(base(id_, bone, mat, tone, role, **kw), kind="ellipsoid", radii=[r6(v) for v in radii], transform={"translation": loc3(bone, *c)})


UP, AHEAD, LEFT = (0, 1, 0), (1, 0, 0), (0, 0, 1)


def post_parts(bone, x, s):
    """🗼️ One painted pull-up post with its cap (1:50, 1:100) and the cast clamps of both bars on its inner face (to 1:200)."""
    tag = bone[-1]
    out = []
    top = P_TOP if s in (50, 100) else P_TOP + CAP_H
    out.append(cyl(f"post-{tag}", bone, "steel-painted", 0.42, (x, P_BOT, 0), UP, top - P_BOT, P_R))
    if s in (50, 100):
        prof = [(0, 0), (0, P_R + CAP_LIP), (CAP_SK, P_R + CAP_LIP), (CAP_SK + 0.006, P_R * 0.86), (CAP_SK + 0.013, P_R * 0.6), (CAP_SK + CAP_H, 0)]
        out.append(cyl(f"cap-{tag}", bone, "steel-painted", 0.36, (x, P_TOP - CAP_SK, 0), UP, 0, 0, prof=prof))
    if s != 500:
        side = 1 if x < 0.7 else -1
        for name, y in (("high", BAR_HI), ("low", BAR_LO)):
            cx = x + side * (CL_IN + CL_OUT) / 2
            out.append(wbox(f"clamp-{tag}-{name}", bone, "steel-painted", 0.34, (cx, y, 0), (CL_OUT - CL_IN, CL_H, CL_W), corner=0.006 if s == 50 else None))
    return out


def bar_parts(bone, y, s):
    """➖️ One galvanised pull-up bar between the clamps (at 1:500 between the post faces, at the print floor)."""
    r = rr(BAR_R, s)
    if s == 500:
        x0, x1 = PA_X + 0.03, PB_X - 0.03
    else:
        x0, x1 = BAR_X0, BAR_X1
    return [cyl(bone, bone, "steel-galvanised", 0.62, (x0, y, 0), AHEAD, x1 - x0, r)]


def dip_post_parts(bone, x, s):
    """🏗️ The two galvanised posts of one end of the parallel bars, mitred into the bars."""
    r = rr(DIP_R, s)
    tag = "front" if x < 3 else "rear"
    return [cyl(f"dip-post-{tag}-{side}", bone, "steel-galvanised", 0.58, (x, DIP_BOT, sz * DIP_Z), UP, DIP_Y - DIP_BOT, r) for side, sz in (("r", -1), ("l", 1))]


def dip_bar_parts(s):
    """🤸️ The two parallel dip bars, with welded domed end caps at 1:50."""
    r = rr(DIP_R, s)
    L = DIP_X1 - DIP_X0
    out = []
    for side, sz in (("r", -1), ("l", 1)):
        prof = None
        if s == 50:
            prof = [(0, 0), (0, 0.016), (0.004, 0.021), (0.009, r), (L - 0.009, r), (L - 0.004, 0.021), (L, 0.016), (L, 0)]
        out.append(cyl(f"dip-bar-{side}", f"dip-bar-{side}", "steel-galvanised", 0.6, (DIP_X0, DIP_Y, sz * DIP_Z), AHEAD, L, r, prof=prof))
    return out


def board_parts(s):
    """🪵️ The greyed Robinia board: three 95 x 45 mm slats with 12 mm gaps at 1:50, one 309 mm board above."""
    if s == 50:
        return [lbox(f"slat-{n}", "bench-board", "robinia-weathered", 0.7, (B_L / 2, SLAT_T / 2, z), (B_L, SLAT_T, SLAT_W)) for n, z in zip(("r", "m", "l"), SLAT_Z)]
    vb = board_vb(s)
    return [lbox("board", "bench-board", "robinia-weathered", 0.7, (B_L / 2, (vb + SLAT_T) / 2, 0), (B_L, SLAT_T - vb, 2 * BOARD_HALF))]


def bench_low_parts(s):
    """🦵️ The low leg frame: two legs under a lateral cross tube tangent to the board."""
    r = rr(BF_R, s)
    cx, cy = cross_centre(U_LOW, s)
    top = cy + r if s == 500 else cy
    out = [cyl(f"leg-low-{side}", "bench-frame-low", "steel-galvanised", 0.56, (cx, B_BOT, sz * LOW_Z), UP, top - B_BOT, r) for side, sz in (("r", -1), ("l", 1))]
    if s == 500:
        return out
    out.append(cyl("cross-low", "bench-frame-low", "steel-galvanised", 0.56, (cx, cy, -BOARD_HALF), LEFT, 2 * BOARD_HALF, r))
    return out


def bench_high_parts(s):
    """🦶️ The high leg frame: two legs rising beside the board to the foot bar, the cross tube under the board and the EPDM foot roller."""
    r = rr(BF_R, s)
    cx, cy = cross_centre(U_HIGH, s)
    half = HIGH_Z + BF_R
    extra = {"slender": True} if s == 500 else {}
    out = [cyl(f"leg-high-{side}", "bench-frame-high", "steel-galvanised", 0.56, (cx, B_BOT, sz * HIGH_Z), UP, FOOT_Y - B_BOT, BF_R, **extra) for side, sz in (("r", -1), ("l", 1))]
    if s != 500:
        out.append(cyl("cross-high", "bench-frame-high", "steel-galvanised", 0.56, (cx, cy, -HIGH_Z), LEFT, 2 * HIGH_Z, r))
    out.append(cyl("foot-bar", "bench-frame-high", "steel-galvanised", 0.56, (cx, FOOT_Y, -half), LEFT, 2 * half, r))
    out.append(cyl("foot-roller", "bench-frame-high", "epdm-worn", 0.18, (cx, FOOT_Y, -ROLL_HALF), LEFT, 2 * ROLL_HALF, ROLL_R))
    return out


FOOTINGS = [
    ("footing-a", PA_X, FA, FA),
    ("footing-b", PB_X, FA, FA),
    ("footing-dip-front", DIP_XF, DIP_FX, DIP_FZ),
    ("footing-dip-rear", DIP_XR, DIP_FX, DIP_FZ),
    ("footing-bench-low", XL, BFL_X, BFL_Z),
    ("footing-bench-high", XH, BFH_X, BFH_Z),
]


def footing_parts():
    """🧱️ The six buried C25/30 footings, 0.70 m deep with their top 100 mm under the impact surface."""
    return [wbox(b, b, "concrete", 0.74, (x, (FT + FB) / 2, 0), (sx, FT - FB, sz)) for b, x, sx, sz in FOOTINGS]


GRIPS_HI = ((0.40, 0.52), (0.88, 1.00))
GRIPS_DIP = ((2.62, 2.98),)
GUM = ((4.78, 0.0, 0.009), (5.12, SLAT_Z[2], 0.007))


def textures_50():
    """🟤️ The traces of use as texture solids at 1:50: rust collars at the ground line and at the welds, the grips worn to bare zinc, trodden gum on the board."""
    t = []
    for bone, x, tag in (("post-a", PA_X, "a"), ("post-b", PB_X, "b")):
        t.append(ring(f"rust-ground-{tag}", bone, "rust", 0.22, (x, -0.02, 0), UP, 0, 0.06, P_R, P_R + 0.0008))
        side = 1 if x < 0.7 else -1
        for name, y in (("high", BAR_HI), ("low", BAR_LO)):
            t.append(lbox(f"rust-clamp-{tag}-{name}", bone, "rust", 0.24, (y - P_BOT, -side * (CL_IN + 0.006), 0), (CL_H + 0.006, 0.006, CL_W + 0.006), role="texture", texture=True))
    for bone in ("bar-high", "bar-low"):
        y = BAR_HI if bone == "bar-high" else BAR_LO
        for i, (a, b) in enumerate(GRIPS_HI):
            t.append(ring(f"grip-{bone}-{i + 1}", bone, "steel-worn", 0.86, (a, y, 0), AHEAD, 0, b - a, BAR_R, BAR_R + 0.0006))
    for x, tag in ((DIP_XF, "front"), (DIP_XR, "rear")):
        for side, sz in (("r", -1), ("l", 1)):
            t.append(ring(f"rust-weld-{tag}-{side}", f"dip-post-{tag}", "rust", 0.24, (x, DIP_Y - 0.05, sz * DIP_Z), UP, 0, 0.026, DIP_R, DIP_R + 0.0012))
            t.append(ring(f"rust-ground-{tag}-{side}", f"dip-post-{tag}", "rust", 0.24, (x, -0.02, sz * DIP_Z), UP, 0, 0.05, DIP_R, DIP_R + 0.0008))
    for side, sz in (("r", -1), ("l", 1)):
        for i, (a, b) in enumerate(GRIPS_DIP):
            t.append(ring(f"grip-dip-{side}", f"dip-bar-{side}", "steel-worn", 0.86, (a, DIP_Y, sz * DIP_Z), AHEAD, 0, b - a, DIP_R, DIP_R + 0.0006))
    for side, sz in (("r", -1), ("l", 1)):
        t.append(ring(f"rust-leg-high-{side}", "bench-frame-high", "rust", 0.24, (XH, YH - BF_R - 0.02, sz * HIGH_Z), UP, 0, 0.02, BF_R, BF_R + 0.0012))
    for i, (x, z, r) in enumerate(GUM):
        u = (x - B_X0) / CS
        t.append(lbox(f"gum-{i + 1}", "bench-board", "chewing-gum", 0.2, (u, SLAT_T + 0.0007, z), (2 * r, 0.0014, 2 * r * 0.8), role="texture", texture=True))
    return t


def body(s):
    """🧍️ The body list of one scale."""
    b = footing_parts()
    b += post_parts("post-a", PA_X, s) + post_parts("post-b", PB_X, s)
    b += bar_parts("bar-high", BAR_HI, s) + bar_parts("bar-low", BAR_LO, s)
    b += dip_post_parts("dip-post-front", DIP_XF, s) + dip_post_parts("dip-post-rear", DIP_XR, s)
    b += dip_bar_parts(s)
    b += bench_low_parts(s) + bench_high_parts(s) + board_parts(s)
    if s == 50:
        b += textures_50()
    return b


def region_of(sid):
    """🏷️ The thermal region of a body solid by its id."""
    if sid.startswith("footing"):
        return "footings"
    if sid.startswith(("post-", "cap-", "clamp-")):
        return "pull-up-posts"
    if sid.startswith("bar-"):
        return "pull-up-bars"
    if sid.startswith("dip-"):
        return "dip-frame"
    if sid.startswith(("slat-", "board")):
        return "bench-board"
    if sid == "foot-roller":
        return "foot-roller"
    return "bench-frame"


def thermal(bod):
    """🌡️ Thermal references to every non-texture body solid of a scale."""
    return [{"id": "th-" + s["id"], "of": s["id"], "region": region_of(s["id"])} for s in bod if not s.get("texture")]


MEMBER_OF = {
    "post-a": "post-a", "cap-a": "post-a", "post-b": "post-b", "cap-b": "post-b", "bar-high": "bar-high", "bar-low": "bar-low",
    "dip-bar-r": "dip-bar-r", "dip-bar-l": "dip-bar-l", "board": "bench-board", "slat-r": "bench-board", "slat-m": "bench-board", "slat-l": "bench-board",
}


def member_of(s):
    """🦴️ The member bone of a load-bearing body solid, or None for parts that are not members (caps, clamps, roller, textures)."""
    sid = s["id"]
    if s.get("texture") or sid.startswith(("cap-", "clamp-")) or sid == "foot-roller":
        return None
    return s["bone"]


def structure(s, bod):
    """🏗️ The structural idealisation of one scale: the tubes, slats and footings are the members, joints at the embedments, clamps and welds, soil supports under every footing, masses at the centres."""
    f = FEAT[s]
    st = []
    for sol in bod:
        m = member_of(sol)
        if m:
            st.append({"id": "m-" + sol["id"], "of": sol["id"], "member": m, "part": "member"})
    j = max(0.04, 2 * f)
    jt = max(0.03, 2 * f)

    def joint(id_, bone, c, size=None):
        st.append(dict(wbox(id_, bone, "steel-galvanised", 0.3, c, size or (j, jt, j), role="detail"), member=bone, part="joint"))

    def fit(d):
        return max(f, 0.85 * d)

    pj = fit(P_D)
    joint("joint-embed-a", "post-a", (PA_X, FT, 0), (pj, jt, pj))
    joint("joint-embed-b", "post-b", (PB_X, FT, 0), (pj, jt, pj))
    bj = 2 * rr(BAR_R, s) if s == 500 else 0.04
    for bone, y in (("bar-high", BAR_HI), ("bar-low", BAR_LO)):
        x1 = BAR_X0 if s != 500 else PA_X + 0.03
        for tag, xp, xe, side in (("a", PA_X, x1, 1), ("b", PB_X, PB_X - (x1 - PA_X), -1)):
            inner = xp + side * (P_R - 0.012)
            reach = xe + side * 0.012
            if abs(reach - inner) < f:
                reach = inner + side * f
            xa, xb = sorted((inner, reach))
            joint(f"joint-{bone}-{tag}", bone, ((xa + xb) / 2, y, 0), (xb - xa, bj, bj))
    dj = fit(2 * rr(DIP_R, s))
    for bone, x in (("dip-post-front", DIP_XF), ("dip-post-rear", DIP_XR)):
        for sz in (-1, 1):
            joint(f"joint-{bone}-embed-{'l' if sz > 0 else 'r'}", bone, (x, FT, sz * DIP_Z), (dj, jt, dj))
    for x, tag in ((DIP_XF, "front"), (DIP_XR, "rear")):
        for sz in (-1, 1):
            sd = "l" if sz > 0 else "r"
            joint(f"joint-dip-weld-{tag}-{sd}", f"dip-bar-{sd}", (x, DIP_Y, sz * DIP_Z), (dj, dj, dj))
    cl = cross_centre(U_LOW, s)
    ch = cross_centre(U_HIGH, s)
    lj = fit(2 * rr(BF_R, s))
    for sz in (-1, 1):
        sd = "l" if sz > 0 else "r"
        joint(f"joint-leg-low-embed-{sd}", "bench-frame-low", (cl[0], FT, sz * LOW_Z), (lj, jt, lj))
        joint(f"joint-leg-high-embed-{sd}", "bench-frame-high", (ch[0], FT, sz * HIGH_Z), (lj, jt, lj))
    for tag, c in (("low", cl), ("high", ch)):
        joint(f"joint-board-{tag}", "bench-board", (c[0], c[1] + rr(BF_R, s), 0), (lj, max(f, 0.02), 2 * BOARD_HALF - 0.02))
    for b, x, sx, sz in FOOTINGS:
        st.append(dict(wbox(f"support-{b}", b, "concrete", 0.15, (x, FB - f / 2, 0), (sx - 0.02, f, sz - 0.02), role="ground"), member=b, part="support", anchor="soil-" + b.replace("footing-", "")))
    mr = max(0.015, f / 2 + 0.0001)
    for b, (cx, cy, cz) in COM_WORLD.items():
        st.append(dict(wbox(f"mass-{b}", b, "steel-galvanised", 0.3, (cx, cy, cz), (2 * mr, 2 * mr, 2 * mr), role="detail") | {"kind": "ellipsoid", "radii": [r6(mr)] * 3}, member=b, part="mass"))
    for sol in st:
        if sol.get("kind") == "ellipsoid":
            sol.pop("size", None)
    return st


# ---------------------------------------------------------------- masses and areas


def tube_area(d, t):
    """⭕️ Steel area of a round hollow section."""
    return math.pi / 4 * (d * d - (d - 2 * t) ** 2)


def tube_i(d, t):
    """🌀️ Second moment of area of a round hollow section."""
    return math.pi / 64 * (d**4 - (d - 2 * t) ** 4)


def masses():
    """⚖️ Masses from section x length x density per bone, with world centres of mass."""
    st = RHO["steel"]
    m, com = {}, {}
    post_tube = tube_area(P_D, P_T) * (P_TOP - P_BOT) * st
    cap = 0.003 * (2 * math.pi * (P_R + CAP_LIP) * CAP_SK + math.pi * (P_R + CAP_LIP) ** 2 * 1.12) * st
    clamp = (CL_OUT - CL_IN) * CL_H * CL_W * 0.6 * st
    for b, x in (("post-a", PA_X), ("post-b", PB_X)):
        side = 1 if x < 0.7 else -1
        tot = post_tube + cap + 2 * clamp
        cy = (post_tube * (P_BOT + P_TOP) / 2 + cap * (P_TOP + 0.005) + clamp * (BAR_HI + BAR_LO)) / tot
        cx = x + side * (2 * clamp * (CL_IN + CL_OUT) / 2) / tot
        m[b], com[b] = tot, (cx, cy, 0.0)
    bar = tube_area(BAR_D, BAR_T) * (BAR_X1 - BAR_X0) * st
    for b, y in (("bar-high", BAR_HI), ("bar-low", BAR_LO)):
        m[b], com[b] = bar, ((BAR_X0 + BAR_X1) / 2, y, 0.0)
    dpost = 2 * tube_area(DIP_D, DIP_T) * (DIP_Y - DIP_BOT) * st
    for b, x in (("dip-post-front", DIP_XF), ("dip-post-rear", DIP_XR)):
        m[b], com[b] = dpost, (x, (DIP_Y + DIP_BOT) / 2, 0.0)
    dcap = 2 * 2 * 0.003 * math.pi * DIP_R**2 * 1.1 * st
    for b, sz in (("dip-bar-l", DIP_Z), ("dip-bar-r", -DIP_Z)):
        m[b], com[b] = tube_area(DIP_D, DIP_T) * (DIP_X1 - DIP_X0) * st + dcap / 2, ((DIP_X0 + DIP_X1) / 2, DIP_Y, sz)
    bfa = tube_area(BF_D, BF_T) * st
    legs_low = 2 * bfa * (YL - B_BOT)
    cross_low = bfa * 2 * BOARD_HALF
    m["bench-frame-low"] = legs_low + cross_low
    com["bench-frame-low"] = (XL, (legs_low * (YL + B_BOT) / 2 + cross_low * YL) / m["bench-frame-low"], 0.0)
    legs_high = 2 * bfa * (FOOT_Y - B_BOT)
    cross_high = bfa * 2 * HIGH_Z
    foot_bar = bfa * 2 * FOOT_HALF
    roller = math.pi * (ROLL_R**2 - BF_R**2) * 2 * ROLL_HALF * RHO["epdm"]
    m["bench-frame-high"] = legs_high + cross_high + foot_bar + roller
    com["bench-frame-high"] = (XH, (legs_high * (FOOT_Y + B_BOT) / 2 + cross_high * YH + (foot_bar + roller) * FOOT_Y) / m["bench-frame-high"], 0.0)
    m["bench-board"] = 3 * B_L * SLAT_W * SLAT_T * RHO["robinia"]
    com["bench-board"] = (B_X0 + B_L / 2 * CS - SLAT_T / 2 * SN, B_Y0 + B_L / 2 * SN + SLAT_T / 2 * CS, 0.0)
    for b, x, sx, sz in FOOTINGS:
        m[b], com[b] = sx * sz * (FT - FB) * RHO["concrete"], (x, (FT + FB) / 2, 0.0)
    parts = {"post-tube": post_tube, "cap": cap, "clamp": clamp, "bar": bar, "dip-post-pair": dpost, "roller": roller, "legs-low": legs_low, "legs-high": legs_high, "cross-low": cross_low, "cross-high": cross_high, "foot-bar": foot_bar}
    return m, com, parts


M, COM_WORLD, PARTS = masses()
TOTAL = sum(M.values())


def cyl_area(r, length, ends=2):
    """🧮️ Outer area of a cylinder with `ends` free end discs."""
    return 2 * math.pi * r * length + ends * math.pi * r * r


def areas():
    """📏️ Total outer areas of the 1:50 thermal regions (C29: faces against another region count, seams of one region do not)."""
    a = {}
    rc = math.sqrt((P_R + CAP_LIP) ** 2 + CAP_H**2)
    cap = 2 * math.pi * (P_R + CAP_LIP) * CAP_SK + math.pi * ((P_R + CAP_LIP) ** 2 + CAP_H**2) * 0.98 + (math.pi * (P_R + CAP_LIP) ** 2 - math.pi * P_R**2)
    half = math.asin((CL_W / 2) / P_R)
    xs = P_R * math.cos(half)
    patch = 2 * half * P_R * CL_H
    clamp = CL_H * CL_W + 2 * CL_W * (CL_OUT - (xs + P_R) / 2) + 2 * CL_H * (CL_OUT - xs)
    post = 2 * math.pi * P_R * (P_TOP - P_BOT - CAP_SK) + math.pi * P_R**2
    a["pull-up-posts"] = 2 * (post + cap + 2 * (clamp - patch))
    a["pull-up-bars"] = 2 * cyl_area(BAR_R, BAR_X1 - BAR_X0)
    dpost = cyl_area(DIP_R, DIP_Y - DIP_BOT - DIP_R, ends=1)
    dbar = 2 * math.pi * DIP_R * (DIP_X1 - DIP_X0 - 0.018) + 2 * (math.pi * 0.016**2 + math.pi * (0.016 + DIP_R) * math.hypot(0.009, DIP_R - 0.016))
    a["dip-frame"] = 4 * dpost + 2 * dbar - 4 * math.pi * DIP_R**2
    a["bench-board"] = 3 * 2 * (B_L * SLAT_W + B_L * SLAT_T + SLAT_W * SLAT_T)
    legs_low = 2 * cyl_area(BF_R, YL - BF_R / 2 - B_BOT, ends=1)
    cross_low = cyl_area(BF_R, 2 * BOARD_HALF) - 2 * math.pi * BF_R**2
    legs_high = 2 * cyl_area(BF_R, FOOT_Y - B_BOT - BF_R / 2, ends=1)
    cross_high = 2 * math.pi * BF_R * (2 * HIGH_Z - 2 * BF_R)
    foot = cyl_area(BF_R, 2 * FOOT_HALF) - 2 * math.pi * BF_R * BF_R
    a["bench-frame"] = legs_low + cross_low + legs_high + cross_high + foot
    a["foot-roller"] = cyl_area(ROLL_R, 2 * ROLL_HALF)
    a["footings"] = sum(2 * sx * sz + 2 * (sx + sz) * (FT - FB) for _, _, sx, sz in FOOTINGS)
    return a


A = areas()


# ---------------------------------------------------------------- 2D shapes


def shape(id_, bone, role, z, filled, tone, geom, **kw):
    """✏️ One 2D shape."""
    s = {"id": id_, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    s.update(geom)
    s.update(kw)
    return s


def epoly(id_, bone, role, z, filled, tone, pts, closed=True, **kw):
    """✒️ An elevation polygon from world (x, y) points."""
    return shape(id_, bone, role, z, filled, tone, {"kind": "poly", "points": [list(loc(bone, *p)) for p in pts], "closed": closed}, **kw)


def erect(id_, bone, role, z, tone, x0, y0, x1, y1, **kw):
    """▭️ An elevation rectangle from world corners."""
    return epoly(id_, bone, role, z, True, tone, [(x0, y0), (x1, y0), (x1, y1), (x0, y1)], **kw)


def ecirc(id_, bone, role, z, tone, x, y, r, **kw):
    """🔘️ An elevation circle at a world point."""
    u, v = loc(bone, x, y)
    return shape(id_, bone, role, z, True, tone, {"kind": "ellipse", "cx": u, "cy": v, "rx": r6(r), "ry": r6(r)}, **kw)


def wpoly(id_, bone, role, z, filled, tone, pts, closed=True, **kw):
    """🖊️ A front or plan polygon in world rest coordinates (front: z, y; plan: x, z)."""
    return shape(id_, bone, role, z, filled, tone, {"kind": "poly", "points": [[r6(a), r6(b)] for a, b in pts], "closed": closed}, **kw)


def wrect(id_, bone, role, z, tone, a0, b0, a1, b1, **kw):
    """▫️ A front or plan rectangle in world rest coordinates."""
    return wpoly(id_, bone, role, z, True, tone, [(a0, b0), (a1, b0), (a1, b1), (a0, b1)], **kw)


def wcirc(id_, bone, role, z, tone, cx, cy, r, filled=True, **kw):
    """⭕️ A front or plan circle in world rest coordinates."""
    return shape(id_, bone, role, z, filled, tone, {"kind": "ellipse", "cx": r6(cx), "cy": r6(cy), "rx": r6(r), "ry": r6(r)}, **kw)


RPOST = dict(region="pull-up-posts", material="steel-painted")
RBAR = dict(region="pull-up-bars", material="steel-galvanised")
RDIP = dict(region="dip-frame", material="steel-galvanised")
RBOARD = dict(region="bench-board", material="robinia-weathered")
RFRAME = dict(region="bench-frame", material="steel-galvanised")
RROLL = dict(region="foot-roller", material="epdm-worn")
RFOOT = dict(region="footings", material="concrete")


def cap_outline(x, s):
    """🎩️ Elevation and front outline of a post cap: skirt with its lip and the dome."""
    if s not in (50, 100):
        return None
    R = P_R + CAP_LIP
    y0 = P_TOP - CAP_SK
    pts = [(x - R, y0), (x + R, y0), (x + R, P_TOP), (x + P_R * 0.86, P_TOP + 0.006), (x + P_R * 0.6, P_TOP + 0.013), (x, P_TOP + CAP_H), (x - P_R * 0.6, P_TOP + 0.013), (x - P_R * 0.86, P_TOP + 0.006), (x - R, P_TOP)]
    return pts


def board_corners(s):
    """🪵️ The four world corners of the board in elevation: underside low, underside high, top high, top low."""
    vb = board_vb(s)
    n = (-SN, CS)
    p0 = (B_X0 + vb * n[0], B_Y0 + vb * n[1])
    p1 = (B_X0 + B_L * CS + vb * n[0], B_Y0 + B_L * SN + vb * n[1])
    p2 = (B_X0 + B_L * CS + SLAT_T * n[0], B_Y0 + B_L * SN + SLAT_T * n[1])
    p3 = (B_X0 + SLAT_T * n[0], B_Y0 + SLAT_T * n[1])
    return [p0, p1, p2, p3]


def hatch_c(s):
    """▦️ The concrete hatch of a scale, where it is drawn."""
    return {"hatch": "concrete"} if s in (50, 100, 200) else {}


def elevation(s):
    """🖼️ The side elevation seen from the right (-z): buried footings below the impact surface, the painted pull-up frame with both bars, the parallel bars and the decline sit-up bench, with what years of use left on them."""
    e = []
    for b, x, sx, sz in FOOTINGS:
        e.append(erect(b, b, "body", 1, 0.74, x - sx / 2, FB, x + sx / 2, FT, **hatch_c(s), **RFOOT))
    top = P_TOP if s in (50, 100) else P_TOP + CAP_H
    for bone, x in (("post-a", PA_X), ("post-b", PB_X)):
        tag = bone[-1]
        e.append(erect(f"post-{tag}", bone, "body", 3, 0.42, x - P_R, FT, x + P_R, top, **RPOST))
        if s in (50, 100):
            e.append(epoly(f"cap-{tag}", bone, "body", 3.2, True, 0.36, cap_outline(x, s), **RPOST))
        if s != 500:
            side = 1 if x < 0.7 else -1
            for name, y in (("high", BAR_HI), ("low", BAR_LO)):
                xa, xb = sorted((x + side * CL_IN, x + side * CL_OUT))
                e.append(erect(f"clamp-{tag}-{name}", bone, "body", 3.4, 0.34, xa, y - CL_H / 2, xb, y + CL_H / 2, **RPOST))
    for bone, y in (("bar-high", BAR_HI), ("bar-low", BAR_LO)):
        r = rr(BAR_R, s)
        x0, x1 = (BAR_X0, BAR_X1) if s != 500 else (PA_X + P_R, PB_X - P_R)
        e.append(erect(bone, bone, "body", 4, 0.62, x0, y - r, x1, y + r, **RBAR))
        if s in (50, 100):
            for i, (a, b) in enumerate(GRIPS_HI):
                e.append(erect(f"grip-{bone}-{i + 1}", bone, "texture", 4.2, 0.9, a, y - r, b, y + r, **RBAR))
    r = rr(DIP_R, s)
    for bone, x, tag in (("dip-post-front", DIP_XF, "front"), ("dip-post-rear", DIP_XR, "rear")):
        e.append(erect(f"dip-post-{tag}-r", bone, "body", 3, 0.58, x - r, FT, x + r, DIP_Y, **RDIP))
    if s == 50:
        L = DIP_X1 - DIP_X0
        pts = [(DIP_X0, DIP_Y - 0.016), (DIP_X0 + 0.004, DIP_Y - 0.021), (DIP_X0 + 0.009, DIP_Y - r), (DIP_X1 - 0.009, DIP_Y - r), (DIP_X1 - 0.004, DIP_Y - 0.021), (DIP_X1, DIP_Y - 0.016), (DIP_X1, DIP_Y + 0.016), (DIP_X1 - 0.004, DIP_Y + 0.021), (DIP_X1 - 0.009, DIP_Y + r), (DIP_X0 + 0.009, DIP_Y + r), (DIP_X0 + 0.004, DIP_Y + 0.021), (DIP_X0, DIP_Y + 0.016)]
        for side, zz in (("r", 4), ("l", 3.9)):
            e.append(epoly(f"dip-bar-{side}", f"dip-bar-{side}", "body", zz, True, 0.6, pts, **RDIP))
    else:
        for side, zz in (("r", 4), ("l", 3.9)):
            e.append(erect(f"dip-bar-{side}", f"dip-bar-{side}", "body", zz, 0.6, DIP_X0, DIP_Y - r, DIP_X1, DIP_Y + r, **RDIP))
    if s in (50, 100):
        for a, b in GRIPS_DIP:
            e.append(erect("grip-dip-r", "dip-bar-r", "texture", 4.2, 0.9, a, DIP_Y - r, b, DIP_Y + r, **RDIP))
    rb = rr(BF_R, s)
    cl, ch = cross_centre(U_LOW, s), cross_centre(U_HIGH, s)
    e.append(erect("leg-low-r", "bench-frame-low", "body", 3, 0.56, cl[0] - rb, FT, cl[0] + rb, cl[1] + (rb if s == 500 else 0), **RFRAME))
    e.append(erect("leg-high-r", "bench-frame-high", "body", 3, 0.56, ch[0] - BF_R, FT, ch[0] + BF_R, FOOT_Y, **RFRAME))
    if s != 500:
        e.append(ecirc("cross-low", "bench-frame-low", "body", 3.6, 0.56, cl[0], cl[1], rb, **RFRAME))
        e.append(ecirc("cross-high", "bench-frame-high", "body", 3.6, 0.56, ch[0], ch[1], rb, **RFRAME))
    e.append(ecirc("foot-roller", "bench-frame-high", "body", 3.8, 0.18, ch[0], FOOT_Y, ROLL_R, **RROLL))
    e.append(epoly("slat-r" if s == 50 else "board", "bench-board", "body", 5, True, 0.7, board_corners(s), **({"hatch": "timber"} if s == 50 else {}), **RBOARD))
    if s == 50:
        for tag, x in (("a", PA_X), ("b", PB_X)):
            bone = f"post-{tag}"
            e.append(erect(f"rust-ground-{tag}", bone, "texture", 3.5, 0.22, x - P_R, -0.02, x + P_R, 0.04, hatch="rust", **RPOST))
            e.append(epoly(f"flaking-{tag}", bone, "texture", 3.6, True, 0.78, [(x - P_R, 0.06), (x - 0.02, 0.09), (x + 0.01, 0.07), (x + P_R, 0.12), (x + P_R, 0.31), (x + 0.02, 0.27), (x - 0.015, 0.34), (x - P_R, 0.29)], hatch="flaking", **RPOST))
            side = 1 if x < 0.7 else -1
            for name, y in (("high", BAR_HI), ("low", BAR_LO)):
                xa = x + side * CL_IN
                e.append(erect(f"rust-clamp-{tag}-{name}", bone, "texture", 3.5, 0.24, min(xa, xa + side * 0.012), y - CL_H / 2 - 0.004, max(xa, xa + side * 0.012), y + CL_H / 2 + 0.004, **RPOST))
        e.append(erect("sticker-b", "post-b", "texture", 3.6, 0.86, PB_X - 0.04, 1.52, PB_X + 0.03, 1.60, **RPOST))
        for bone, x, tag in (("dip-post-front", DIP_XF, "front"), ("dip-post-rear", DIP_XR, "rear")):
            e.append(erect(f"rust-weld-{tag}-r", bone, "texture", 3.5, 0.24, x - DIP_R - 0.0012, DIP_Y - 0.05, x + DIP_R + 0.0012, DIP_Y - 0.024, **RDIP))
            e.append(erect(f"rust-ground-{tag}-r", bone, "texture", 3.5, 0.24, x - DIP_R, -0.02, x + DIP_R, 0.03, hatch="rust", **RDIP))
        e.append(erect("rust-leg-high-r", "bench-frame-high", "texture", 3.5, 0.24, ch[0] - BF_R - 0.0012, ch[1] - BF_R - 0.02, ch[0] + BF_R + 0.0012, ch[1] - BF_R, **RFRAME))
        for i, (u0, u1, dv) in enumerate(((0.10, 0.55, 0.012), (0.70, 1.25, 0.03), (0.35, 0.80, 0.022))):
            pts = [(B_X0 + u * CS - (SLAT_T - dv) * SN, B_Y0 + u * SN + (SLAT_T - dv) * CS) for u in (u0, (u0 + u1) / 2, u1)]
            pts[1] = (pts[1][0], pts[1][1] + 0.004)
            e.append(epoly(f"check-{i + 1}", "bench-board", "texture", 5.2, False, 0.35, pts, closed=False, **RBOARD))
    if s == 100:
        for tag, x in (("a", PA_X), ("b", PB_X)):
            e.append(erect(f"rust-ground-{tag}", f"post-{tag}", "texture", 3.5, 0.22, x - P_R, -0.02, x + P_R, 0.05, **RPOST))
    return e


def front(s):
    """🖼️ The front elevation seen from ahead (+x, the sit-up bench nearest): the bench before the parallel bars before the pull-up frame."""
    fr = []
    for b, x, sx, sz in FOOTINGS:
        fr.append(wrect(b, b, "body", {"footing-a": 1, "footing-b": 1.05, "footing-dip-front": 1.1, "footing-dip-rear": 1.15, "footing-bench-low": 1.2, "footing-bench-high": 1.25}[b], 0.74, -sz / 2, FB, sz / 2, FT, **hatch_c(s), **RFOOT))
    top = P_TOP if s in (50, 100) else P_TOP + CAP_H
    for bone, tag in (("post-a", "a"), ("post-b", "b")):
        fr.append(wrect(f"post-{tag}", bone, "body", 2 if tag == "a" else 2.05, 0.42, -P_R, FT, P_R, top, **RPOST))
        if s in (50, 100):
            fr.append(wpoly(f"cap-{tag}", bone, "body", 2.1, True, 0.36, [(p[0] - (PA_X if tag == "a" else PB_X), p[1]) for p in cap_outline(PA_X if tag == "a" else PB_X, s)], **RPOST))
    if s != 500:
        for name, y in (("high", BAR_HI), ("low", BAR_LO)):
            fr.append(wrect(f"clamp-b-{name}", "post-b", "body", 2.2, 0.34, -CL_W / 2, y - CL_H / 2, CL_W / 2, y + CL_H / 2, **RPOST))
    for bone, y in (("bar-high", BAR_HI), ("bar-low", BAR_LO)):
        fr.append(wcirc(bone, bone, "body", 2.3, 0.62, 0, y, rr(BAR_R, s), **RBAR))
    r = rr(DIP_R, s)
    for bone, tag, zz in (("dip-post-front", "front", 3.0), ("dip-post-rear", "rear", 2.9)):
        for side, sz in (("r", -1), ("l", 1)):
            fr.append(wrect(f"dip-post-{tag}-{side}", bone, "body", zz, 0.58, sz * DIP_Z - r, FT, sz * DIP_Z + r, DIP_Y, **RDIP))
    for side, sz in (("r", -1), ("l", 1)):
        fr.append(wcirc(f"dip-bar-{side}", f"dip-bar-{side}", "body", 3.1, 0.6, sz * DIP_Z, DIP_Y, r, **RDIP))
    rb = rr(BF_R, s)
    cl, ch = cross_centre(U_LOW, s), cross_centre(U_HIGH, s)
    for side, sz in (("r", -1), ("l", 1)):
        fr.append(wrect(f"leg-low-{side}", "bench-frame-low", "body", 4, 0.56, sz * LOW_Z - rb, FT, sz * LOW_Z + rb, cl[1] + (rb if s == 500 else 0), **RFRAME))
        fr.append(wrect(f"leg-high-{side}", "bench-frame-high", "body", 3.8, 0.56, sz * HIGH_Z - BF_R, FT, sz * HIGH_Z + BF_R, FOOT_Y, **RFRAME))
    if s != 500:
        fr.append(wrect("cross-low", "bench-frame-low", "body", 4.1, 0.56, -BOARD_HALF, cl[1] - rb, BOARD_HALF, cl[1] + rb, **RFRAME))
        fr.append(wrect("cross-high", "bench-frame-high", "body", 3.85, 0.56, -HIGH_Z, ch[1] - rb, HIGH_Z, ch[1] + rb, **RFRAME))
    fr.append(wrect("foot-bar", "bench-frame-high", "body", 3.9, 0.56, -FOOT_HALF, FOOT_Y - rb, FOOT_HALF, FOOT_Y + rb, **RFRAME))
    fr.append(wrect("foot-roller", "bench-frame-high", "body", 4.5, 0.18, -ROLL_HALF, FOOT_Y - ROLL_R, ROLL_HALF, FOOT_Y + ROLL_R, **RROLL))
    bc = board_corners(s)
    ylo, yhi = min(p[1] for p in bc), max(p[1] for p in bc)
    if s == 50:
        for n, z in zip(("r", "m", "l"), SLAT_Z):
            fr.append(wrect(f"slat-{n}", "bench-board", "body", 4.3, 0.7, z - SLAT_W / 2, ylo, z + SLAT_W / 2, yhi, **RBOARD))
    else:
        fr.append(wrect("board", "bench-board", "body", 4.3, 0.7, -BOARD_HALF, ylo, BOARD_HALF, yhi, **RBOARD))
    if s == 50:
        fr.append(wrect("roller-wear", "bench-frame-high", "texture", 4.6, 0.4, -0.11, FOOT_Y - ROLL_R, 0.11, FOOT_Y - ROLL_R + 0.022, **RROLL))
        fr.append(wrect("rust-ground-front-r", "dip-post-front", "texture", 3.05, 0.24, -DIP_Z - DIP_R, -0.02, -DIP_Z + DIP_R, 0.03, hatch="rust", **RDIP))
        fr.append(wrect("rust-ground-front-l", "dip-post-front", "texture", 3.05, 0.24, DIP_Z - DIP_R, -0.02, DIP_Z + DIP_R, 0.03, hatch="rust", **RDIP))
    return fr


def plan(s):
    """🗺️ The plan from above: the buried footings, the pull-up frame with its two bars, the parallel bars and the sit-up bench, the scuffed hollow under the bars and the worn grips."""
    p = []
    for b, x, sx, sz in FOOTINGS:
        p.append(wrect(b, b, "body", 1, 0.8, x - sx / 2, -sz / 2, x + sx / 2, sz / 2, **({"hatch": "concrete"} if s in (50, 100) else {}), **RFOOT))
    if s == 50:
        p.append(shape("scuffed-hollow", "footing-a", "texture", 1.5, True, 0.9, {"kind": "ellipse", "cx": 0.7, "cy": 0.0, "rx": 0.5, "ry": 0.3}, hatch="scuff", **RFOOT))
        p.append(shape("scuffed-dip", "footing-dip-front", "texture", 1.5, True, 0.9, {"kind": "ellipse", "cx": 2.8, "cy": 0.0, "rx": 0.55, "ry": 0.17}, hatch="scuff", **RFOOT))
    for bone, x, tag in (("post-a", PA_X, "a"), ("post-b", PB_X, "b")):
        p.append(wcirc(f"post-{tag}", bone, "body", 3, 0.42, x, 0, P_R, **RPOST))
        if s in (50, 100):
            p.append(wcirc(f"cap-{tag}", bone, "body", 3.2, 0.36, x, 0, P_R + CAP_LIP, **RPOST))
            p.append(wcirc(f"cap-crown-{tag}", bone, "detail", 3.3, 0.3, x, 0, P_R * 0.6, filled=False, **RPOST))
        if s != 500:
            side = 1 if x < 0.7 else -1
            xa, xb = sorted((x + side * CL_IN, x + side * CL_OUT))
            p.append(wrect(f"clamp-{tag}-high", bone, "body", 3.4, 0.34, xa, -CL_W / 2, xb, CL_W / 2, **RPOST))
    for bone, zz in (("bar-high", 4), ("bar-low", 3.9)):
        r = rr(BAR_R, s)
        x0, x1 = (BAR_X0, BAR_X1) if s != 500 else (PA_X + P_R, PB_X - P_R)
        p.append(wrect(bone, bone, "body", zz, 0.62, x0, -r, x1, r, **RBAR))
    if s in (50, 100):
        for i, (a, b) in enumerate(GRIPS_HI):
            p.append(wrect(f"grip-bar-high-{i + 1}", "bar-high", "texture", 4.2, 0.9, a, -BAR_R, b, BAR_R, **RBAR))
    r = rr(DIP_R, s)
    for side, sz in (("r", -1), ("l", 1)):
        p.append(wrect(f"dip-bar-{side}", f"dip-bar-{side}", "body", 4, 0.6, DIP_X0, sz * DIP_Z - r, DIP_X1, sz * DIP_Z + r, **RDIP))
        if s in (50, 100):
            for a, b in GRIPS_DIP:
                p.append(wrect(f"grip-dip-{side}", f"dip-bar-{side}", "texture", 4.2, 0.9, a, sz * DIP_Z - r, b, sz * DIP_Z + r, **RDIP))
    rb = rr(BF_R, s)
    cl, ch = cross_centre(U_LOW, s), cross_centre(U_HIGH, s)
    if s != 500:
        p.append(wrect("cross-low", "bench-frame-low", "body", 3, 0.56, cl[0] - rb, -BOARD_HALF, cl[0] + rb, BOARD_HALF, **RFRAME))
    else:
        for side, sz in (("r", -1), ("l", 1)):
            p.append(wcirc(f"leg-low-{side}", "bench-frame-low", "body", 3, 0.56, cl[0], sz * LOW_Z, rb, **RFRAME))
    for side, sz in (("r", -1), ("l", 1)):
        p.append(wcirc(f"leg-high-{side}", "bench-frame-high", "body", 3, 0.56, ch[0], sz * HIGH_Z, BF_R, **RFRAME))
    p.append(wrect("foot-bar", "bench-frame-high", "body", 5.5, 0.56, ch[0] - rb, -FOOT_HALF, ch[0] + rb, FOOT_HALF, **RFRAME))
    p.append(wrect("foot-roller", "bench-frame-high", "body", 6, 0.18, ch[0] - ROLL_R, -ROLL_HALF, ch[0] + ROLL_R, ROLL_HALF, **RROLL))
    bc = board_corners(s)
    xlo, xhi = min(q[0] for q in bc), max(q[0] for q in bc)
    if s == 50:
        for n, z in zip(("r", "m", "l"), SLAT_Z):
            p.append(wrect(f"slat-{n}", "bench-board", "body", 5, 0.7, xlo, z - SLAT_W / 2, xhi, z + SLAT_W / 2, hatch="timber", **RBOARD))
        for i, (x, z, rg) in enumerate(GUM):
            p.append(wcirc(f"gum-{i + 1}", "bench-board", "texture", 5.3, 0.2, x, z, rg, **RBOARD))
        for i, (z, x0, x1) in enumerate(((-0.12, 4.40, 4.95), (0.01, 4.85, 5.45), (0.09, 4.50, 5.05))):
            p.append(wpoly(f"check-{i + 1}", "bench-board", "texture", 5.2, False, 0.35, [(x0, z), ((x0 + x1) / 2, z + 0.005), (x1, z - 0.002)], closed=False, **RBOARD))
        p.append(wrect("roller-wear", "bench-frame-high", "texture", 6.1, 0.4, ch[0] - ROLL_R, -0.11, ch[0] - ROLL_R + 0.022, 0.11, **RROLL))
    else:
        p.append(wrect("board", "bench-board", "body", 5, 0.7, xlo, -BOARD_HALF, xhi, BOARD_HALF, **RBOARD))
    return p


def hatches(s):
    """▤️ The hatches of one scale."""
    h = []
    if s in (50, 100, 200):
        h.append({"id": "concrete", "title": T("Concrete, stippled", "Beton, gepunktet"), "pattern": "dots", "angleDeg": 0, "spacingMm": {50: 0.9, 100: 0.7, 200: 0.5}[s], "penMm": 0.13})
    if s == 50:
        h.append({"id": "timber", "title": T("Greyed Robinia, grain along the board", "Vergraute Robinie, Maserung längs"), "pattern": "lines", "angleDeg": round(TH, 1), "spacingMm": 0.6, "penMm": 0.13})
        h.append({"id": "rust", "title": T("Rust at the ground line and the welds", "Rost an der Bodenlinie und den Schweißnähten"), "pattern": "stipple", "angleDeg": 0, "spacingMm": 0.4, "penMm": 0.13})
        h.append({"id": "flaking", "title": T("Flaking powder coat, zinc showing", "Abplatzende Pulverbeschichtung, Zink sichtbar"), "pattern": "cross", "angleDeg": 45, "spacingMm": 0.7, "penMm": 0.13})
        h.append({"id": "scuff", "title": T("Scuffed hollow in the impact gravel", "Ausgetretene Mulde im Fallschutzkies"), "pattern": "dots", "angleDeg": 0, "spacingMm": 0.8, "penMm": 0.13})
    return h


PENS = {50: {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": None, "ground": 0.7}, 100: {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": None, "ground": 0.5}, 200: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.35}, 500: {"body": 0.18, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.25}}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}

REP_DECISIONS = {
    50: {
        "silhouette": T("The whole station at 117 x 66 mm: six buried footings 0.70 m deep with their top 100 mm under the impact gravel, the two painted 114.3 mm pull-up posts with domed caps and four cast clamps, the high bar at 2.35 m and the low rowing bar at 1.00 m, the parallel dip bars at 1.25 m on four 48.3 mm posts, and the decline sit-up bench with three 95 mm Robinia slats, its two leg frames and the EPDM foot roller.", "Die ganze Station mit 117 x 66 mm: sechs eingegrabene Fundamente 0,70 m tief mit ihrer Oberkante 100 mm unter dem Fallschutzkies, die zwei lackierten Klimmzugpfosten 114,3 mm mit gewölbten Kappen und vier Gussschellen, die hohe Stange auf 2,35 m und die niedrige Ruderstange auf 1,00 m, der Barren auf 1,25 m auf vier Pfosten 48,3 mm und die schräge Sit-up-Bank mit drei Robinienlatten 95 mm, ihren zwei Beinrahmen und der EPDM-Fußrolle."),
        "retained": T("Cap lips, clamp blocks, the welded domed end caps of the dip bars, the 12 mm gaps between the slats; the grips worn to bare zinc, rust at the ground line and at the welds, flaking powder coat at the post feet, a sticker on post B, checks in the greyed board, trodden gum, the roller worn flat where the feet hook, the scuffed hollows in the gravel under the bars.", "Kappenränder, Schellenblöcke, die angeschweißten gewölbten Endkappen der Barrenholme, die 12-mm-Fugen zwischen den Latten; die bis aufs Zink abgegriffenen Griffzonen, Rost an der Bodenlinie und an den Schweißnähten, abplatzende Pulverbeschichtung an den Pfostenfüßen, ein Aufkleber an Pfosten B, Risse in der vergrauten Bank, festgetretener Kaugummi, die dort plattgedrückte Rolle, wo die Füße einhaken, die ausgetretenen Mulden im Kies unter den Stangen."),
        "merged": T("Bolts and nuts of the clamps and of the slat fixings into the clamp blocks and slats; the tube walls into closed tubes.", "Schrauben und Muttern der Schellen und der Lattenbefestigung in Schellenblöcke und Latten; die Rohrwände in geschlossene Rohre."),
        "omitted": T("The concrete filling of the embedded tube ends, the reinforcement and the blinding layer of the footings, the drainage layer under the impact gravel.", "Die Betonfüllung der eingebetteten Rohrenden, die Bewehrung und die Sauberkeitsschicht der Fundamente, die Dränschicht unter dem Fallschutzkies."),
        "thinElements": T("Checks in the board as open texture strokes; the cap crowns in plan as detail circles.", "Risse in der Bank als offene Texturstriche; die Kappenkuppen im Grundriss als Detailkreise."),
        "hatching": T("Concrete with dots, the board with fine lines along its slope, rust with a stipple, flaking coat cross-hatched, the scuffed gravel with dots.", "Beton mit Punkten, die Bank mit feinen Linien entlang ihrer Neigung, Rost punktiert, abplatzende Beschichtung kreuzschraffiert, der ausgetretene Kies mit Punkten."),
        "styles": T("ISO 128 pens 0.5 / 0.25 / 0.13 mm; dark moss-green posts, mid-grey zinc, the worn grips nearly white, the roller black.", "ISO-128-Stifte 0,5 / 0,25 / 0,13 mm; dunkel moosgrüne Pfosten, mittelgraues Zink, die abgegriffenen Griffzonen fast weiß, die Rolle schwarz."),
        "overlays": T("None: the athletes are loads on the bars, the board and the roller, not drawn figures.", "Keine: die Sportler sind Lasten auf den Stangen, der Bank und der Rolle, keine gezeichneten Figuren."),
    },
    100: {
        "silhouette": T("The station at 59 x 33 mm: footings, posts with caps and clamps, both pull-up bars, the dip frame and the bench with one 309 mm board.", "Die Station mit 59 x 33 mm: Fundamente, Pfosten mit Kappen und Schellen, beide Klimmzugstangen, der Barren und die Bank mit einer Liegefläche 309 mm."),
        "retained": T("Cap domes (20 mm), clamp blocks (60 x 90 mm), the worn grip zones of the bars, the rust band at the foot of the pull-up posts.", "Kappenkuppen (20 mm), Schellenblöcke (60 x 90 mm), die abgegriffenen Griffzonen der Stangen, das Rostband am Fuß der Klimmzugpfosten."),
        "merged": T("The three slats and their 12 mm gaps into one board; the 5 mm dip-bar end caps into the bar ends.", "Die drei Latten und ihre 12-mm-Fugen zu einer Liegefläche; die 5-mm-Endkappen der Barrenholme in die Holmenden."),
        "omitted": T("Weld rust, flaking coat, sticker, gum, board checks, the roller wear and the scuffed hollows: below 0.15 mm of paper or not legible at this size.", "Schweißnahtrost, abplatzende Beschichtung, Aufkleber, Kaugummi, Risse, die Rollenabnutzung und die Mulden: unter 0,15 mm auf dem Papier oder in dieser Größe nicht lesbar."),
        "thinElements": T("The cap crowns in plan as detail circles.", "Die Kappenkuppen im Grundriss als Detailkreise."),
        "hatching": T("Concrete dots 0.7 mm only.", "Nur Betonpunkte 0,7 mm."),
        "styles": T("Pens 0.35 / 0.18 / 0.13 mm.", "Stifte 0,35 / 0,18 / 0,13 mm."),
        "overlays": T("None.", "Keine."),
    },
    200: {
        "silhouette": T("The station at 29 x 17 mm: footings, flat-topped posts with their clamps, the two bars (0.17 mm), the dip frame and the bench as board, frames and roller.", "Die Station mit 29 x 17 mm: Fundamente, oben flache Pfosten mit ihren Schellen, die zwei Stangen (0,17 mm), der Barren und die Bank als Liegefläche, Rahmen und Rolle."),
        "retained": T("The clamp blocks (60 mm), all tubes at their true diameters (33.7 to 114.3 mm), the 110 mm roller.", "Die Schellenblöcke (60 mm), alle Rohre in ihrem wahren Durchmesser (33,7 bis 114,3 mm), die Rolle 110 mm."),
        "merged": T("The 20 mm cap domes into the post tops (same 2.52 m height).", "Die 20-mm-Kappenkuppen in die Pfostenköpfe (gleiche Höhe 2,52 m)."),
        "omitted": T("Every trace of use: grip wear, rust and coat damage are under the printable feature.", "Alle Gebrauchsspuren: Griffabnutzung, Rost und Beschichtungsschäden liegen unter der druckbaren Einzelheit."),
        "thinElements": T("None.", "Keine."),
        "hatching": T("Concrete dots 0.5 mm only.", "Nur Betonpunkte 0,5 mm."),
        "styles": T("Pens 0.25 / 0.13 mm.", "Stifte 0,25 / 0,13 mm."),
        "overlays": T("None.", "Keine."),
    },
    500: {
        "silhouette": T("The station at 12 x 6.6 mm: footings, the two pull-up posts with both bars running into them, the dip frame and the bench; every tube thinner than 75 mm is drawn at the 0.15 mm print floor.", "Die Station mit 12 x 6,6 mm: Fundamente, die zwei Klimmzugpfosten mit beiden Stangen, die in sie einlaufen, der Barren und die Bank; jedes Rohr unter 75 mm ist an der Druckgrenze von 0,15 mm gezeichnet."),
        "retained": T("The heights of both bars (2.35 and 1.00 m), the dip bars at 1.25 m and 0.52 m apart, the slope of the bench and the roller, the footprints of the six footings.", "Die Höhen beider Stangen (2,35 und 1,00 m), der Barren auf 1,25 m mit 0,52 m Abstand, die Neigung der Bank und die Rolle, die Grundflächen der sechs Fundamente."),
        "merged": T("The clamps (60 mm) into the posts, the two bench cross tubes into the board, thickened to 75 mm downwards so its sitting face stays at its height.", "Die Schellen (60 mm) in die Pfosten, die zwei Bankquerrohre in die Liegefläche, nach unten auf 75 mm verdickt, so bleibt die Sitzfläche auf ihrer Höhe."),
        "omitted": T("Caps, clamps and every trace of use.", "Kappen, Schellen und alle Gebrauchsspuren."),
        "thinElements": T("Bars, dip tubes and bench tubes as 75 mm bands (true 33.7 to 48.3 mm).", "Stangen, Barrenrohre und Bankrohre als Bänder von 75 mm (wahr 33,7 bis 48,3 mm)."),
        "hatching": T("None.", "Keine."),
        "styles": T("Pens 0.18 / 0.13 mm.", "Stifte 0,18 / 0,13 mm."),
        "overlays": T("None.", "Keine."),
    },
}


def representation(s):
    """🖨️ One authored representation."""
    lo, hi = RANGES[s]
    return {"status": "authored", "denominator": s, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[s], "hatches": hatches(s), "geometry": {"elevation": elevation(s), "front": front(s), "plan": plan(s)}, "decisions": REP_DECISIONS[s]}


VOL_DECISIONS = {
    50: [
        T("Body for 7.5 mm: six C25/30 footings as boxes, the pull-up posts as revolved 114.3 mm tubes 3.10 m long (0.50 m embedded) with pressed caps (2 mm lip, 20 mm dome), four cast clamps with 6 mm rounded edges, the bars as 33.7 mm cylinders butting the clamps, the four 48.3 mm dip posts mitred into the 48.3 mm dip bars with welded domed end caps in the bar profile, the bench as three 95 x 45 mm slats on two 42.4 mm cross tubes, two leg frames embedded 0.40 m, the foot bar and the 110 mm EPDM roller.", "Körper für 7,5 mm: sechs Fundamente C25/30 als Quader, die Klimmzugpfosten als gedrehte Rohre 114,3 mm, 3,10 m lang (0,50 m eingebettet), mit gepressten Kappen (2-mm-Rand, 20-mm-Kuppe), vier Gussschellen mit 6-mm-Rundung, die Stangen als Zylinder 33,7 mm stumpf an den Schellen, die vier Barrenpfosten 48,3 mm in die Barrenholme 48,3 mm eingeschnitten, mit angeschweißten gewölbten Endkappen im Holmprofil, die Bank als drei Latten 95 x 45 mm auf zwei Querrohren 42,4 mm, zwei Beinrahmen 0,40 m eingebettet, die Fußstange und die EPDM-Rolle 110 mm."),
        T("Traces of use as texture solids: rust collars at the ground line of every post and below the dip-bar welds, rust lips at the clamp welds and the bench leg weld, the grip zones of both pull-up bars and of the dip bars worn to bare zinc, two trodden gum spots on the board; flaking coat, sticker, checks and the scuffed hollows are 2D texture only.", "Gebrauchsspuren als Texturkörper: Rostkragen an der Bodenlinie jedes Pfostens und unter den Barrennähten, Rostränder an den Schellennähten und der Bankbeinnaht, die Griffzonen beider Klimmzugstangen und der Barrenholme bis aufs Zink abgegriffen, zwei festgetretene Kaugummis auf der Bank; abplatzende Beschichtung, Aufkleber, Risse und die Mulden sind nur 2D-Textur."),
        T("Thermal: references to every non-texture body solid in seven regions; structure: the tubes, slats and footings are the members, joints at the embedments, the clamps, the dip welds and the board fixings, a soil support pad under each footing, masses at the centres of every bone; the workers' hands under the lowered bar are contacts of the maintenance pose without support solids.", "Thermik: Verweise auf jeden Körper außer den Texturen in sieben Regionen; Tragwerk: Rohre, Latten und Fundamente sind die Stäbe, Gelenke an den Einbettungen, den Schellen, den Barrennähten und der Lattenbefestigung, ein Bodenauflager unter jedem Fundament, Massen in den Schwerpunkten jedes Knochens; die Hände der Arbeiter unter der abgesenkten Stange sind Kontakte der Wartungspose ohne Auflagerkörper."),
    ],
    100: [
        T("Body for 15 mm: the three slats and their 12 mm gaps merge into one 309 x 45 mm board; the 5 mm dip-bar end caps merge into plain bar ends; caps (20 mm dome) and clamps stay; clamp edges are sharp; no texture solids (rust collars and worn grips are below 1 mm thick).", "Körper für 15 mm: die drei Latten und ihre 12-mm-Fugen werden eine Liegefläche 309 x 45 mm; die 5-mm-Endkappen der Barrenholme gehen in glatten Holmenden auf; Kappen (20-mm-Kuppe) und Schellen bleiben; Schellenkanten scharf; keine Texturkörper (Rostkragen und Griffzonen sind unter 1 mm dick)."),
    ],
    200: [
        T("Body for 30 mm: the post caps merge into flat-topped posts of the same 2.52 m height; clamps (60 mm), the 33.7 mm bars and every other tube keep their true size.", "Körper für 30 mm: die Pfostenkappen gehen in oben flachen Pfosten gleicher Höhe 2,52 m auf; Schellen (60 mm), die Stangen 33,7 mm und alle anderen Rohre behalten ihre wahre Größe."),
        T("Minimal: the structure keeps one member per bone, one joint per real connection (embedments, clamps, welds, board fixings), one soil support per footing and one mass per bone, so it cannot lose a solid without losing a load path; its joints and supports grow to 60 mm.", "Minimal: das Tragwerk behält einen Stab je Knochen, ein Gelenk je wirklicher Verbindung (Einbettungen, Schellen, Schweißnähte, Lattenbefestigung), ein Bodenauflager je Fundament und eine Masse je Knochen, es kann daher keinen Körper verlieren, ohne einen Lastpfad zu verlieren; seine Gelenke und Auflager wachsen auf 60 mm."),
    ],
    500: [
        T("Body for 75 mm: the clamps are dropped and both bars run into the posts; every tube under 75 mm (bars, dip frame, low bench legs, foot bar) is modelled at the 75 mm print floor of its 2D band, except the two high bench legs, which keep their true 42.4 mm as declared slender rods (34:1) so the 338 mm between them still prints open at 1:1000; the board is 75 mm thick, grown downwards, and takes the two cross tubes in, so the legs run up into it; the 114.3 mm posts, the 110 mm roller and the footings keep their size; the body is minimal for the parts that tell the station apart.", "Körper für 75 mm: die Schellen entfallen und beide Stangen laufen in die Pfosten; jedes Rohr unter 75 mm (Stangen, Barren, niedrige Bankbeine, Fußstange) ist an der 75-mm-Druckgrenze seines 2D-Bandes modelliert, außer den zwei hohen Bankbeinen, die als deklarierte schlanke Stäbe (34:1) ihre wahren 42,4 mm behalten, damit die 338 mm zwischen ihnen auch bei 1:1000 offen drucken; die Liegefläche ist 75 mm dick, nach unten verdickt, und nimmt die zwei Querrohre auf, so laufen die Beine in sie hinein; die Pfosten 114,3 mm, die Rolle 110 mm und die Fundamente behalten ihre Größe; der Körper ist minimal für die Teile, die die Station unterscheiden."),
        T("Minimal: the structure is the same set of members, joints, soil supports and masses as at 1:200, every joint and support at 150 mm; the clamp joints now run from the post into the bar end.", "Minimal: das Tragwerk ist dieselbe Menge aus Stäben, Gelenken, Bodenauflagern und Massen wie bei 1:200, jedes Gelenk und Auflager mit 150 mm; die Schellengelenke reichen jetzt vom Pfosten in das Stangenende."),
    ],
}


def volumes():
    """🧊️ The four handcrafted volumes."""
    out = {}
    for s in SCALES:
        b = body(s)
        out[str(s)] = {"status": "authored", "denominator": s, "body": b, "thermal": thermal(b), "structure": structure(s, b), "decisions": VOL_DECISIONS[s]}
    return out


# ---------------------------------------------------------------- materials, thermal


def material(id_, en, de, eps, alpha, rho, lam, cp, e=None, strength=None, src="authored-estimate", ref="incropera-2011"):
    """🧱️ One material."""
    m = {"id": id_, "title": T(en, de), "emissivity": eps, "solarAbsorptivity": alpha}
    for k, v in (("densityKgM3", rho), ("conductivityWmK", lam), ("specificHeatJKgK", cp), ("elasticModulusPa", e), ("strengthPa", strength)):
        if v is not None:
            m[k] = v
    m["source"] = src
    m["reference"] = ref
    return m


MATERIALS = [
    material("steel-painted", "Powder-coated steel RAL 6005 over zinc, chalked and flaking", "Pulverbeschichteter Stahl RAL 6005 auf Zink, kreidend und abplatzend", 0.9, 0.84, RHO["steel"], 50, 490, 210e9, 235e6, ref="ashrae-hof-2021"),
    material("steel-galvanised", "Hot-dip galvanised steel S235, weathered dull grey zinc patina", "Feuerverzinkter Stahl S235, stumpf grau verwitterte Zinkpatina", 0.28, 0.72, RHO["steel"], 50, 490, 210e9, 235e6, ref="ashrae-hof-2021"),
    material("steel-worn", "Zinc worn bright by hands", "Von Händen blank gegriffenes Zink", 0.12, 0.6, RHO["steel"], 50, 490, None, None, "illustrative", "ashrae-hof-2021"),
    material("rust", "Rust at welds and at the ground line", "Rost an Schweißnähten und an der Bodenlinie", 0.85, 0.85, 5200, 0.6, 650, None, None, "illustrative", "incropera-2011"),
    material("robinia-weathered", "Robinia (black locust) D40, greyed and checked", "Robinie D40, vergraut und rissig", 0.9, 0.62, RHO["robinia"], 0.17, 1600, 13.5e9, 40e6, ref="en-338-2016"),
    material("epdm-worn", "EPDM cellular rubber roller, sun-aged and worn", "EPDM-Zellkautschukrolle, sonnengealtert und abgenutzt", 0.92, 0.93, RHO["epdm"], 0.06, 1900, None, None, ref="incropera-2011"),
    material("concrete", "Concrete C25/30 footing, XC2/XF1", "Betonfundament C25/30, XC2/XF1", 0.92, 0.7, RHO["concrete"], 2.0, 1000, 31e9, 2.6e6, "literature", "iso-10456-2007"),
    material("chewing-gum", "Trodden chewing gum, blackened", "Festgetretener Kaugummi, geschwärzt", 0.9, 0.85, 1100, 0.2, 1600, None, None, "illustrative", "incropera-2011"),
]


def region_mass(rid):
    """⚖️ The mass behind the area of a region, for its areal heat capacity."""
    p = PARTS
    return {
        "pull-up-posts": M["post-a"] + M["post-b"],
        "pull-up-bars": M["bar-high"] + M["bar-low"],
        "dip-frame": M["dip-post-front"] + M["dip-post-rear"] + M["dip-bar-l"] + M["dip-bar-r"],
        "bench-board": M["bench-board"],
        "bench-frame": M["bench-frame-low"] + M["bench-frame-high"] - p["roller"],
        "foot-roller": p["roller"],
        "footings": sum(M[b] for b, *_ in FOOTINGS),
    }[rid]


CP = {"pull-up-posts": 490, "pull-up-bars": 490, "dip-frame": 490, "bench-board": 1600, "bench-frame": 490, "foot-roller": 1900, "footings": 1000}


def region(id_, en, de, mat, lc, geom, svf, src="authored-estimate", ref="ashrae-hof-2021", **kw):
    """🔥️ One passive thermal region with its 1:50 outer area and areal heat capacity."""
    r = {"id": id_, "title": T(en, de), "kind": "surface", "model": "passive", "material": mat, "areaM2": round(A[id_], 4), "characteristicLengthM": lc, "geometry": geom, "skyViewFactor": svf, "layers": [], "heatCapacityJM2K": round(region_mass(id_) * CP[id_] / A[id_]), "source": src, "reference": ref}
    r.update(kw)
    return r


REGIONS = [
    region("pull-up-posts", "Pull-up posts, caps and clamps (powder coat)", "Klimmzugpfosten, Kappen und Schellen (Pulverbeschichtung)", "steel-painted", P_D, "cylinder", 0.5, ratedPowerW=0, ratedPowerBasis="passive"),
    region("pull-up-bars", "High and low pull-up bars (zinc, worn grips)", "Hohe und niedrige Klimmzugstange (Zink, abgegriffen)", "steel-galvanised", BAR_D, "cylinder", 0.55),
    region("dip-frame", "Parallel dip bars and their posts (zinc)", "Barrenholme und ihre Pfosten (Zink)", "steel-galvanised", DIP_D, "cylinder", 0.55),
    region("bench-board", "Greyed Robinia sit-up board", "Vergraute Robinien-Liegefläche", "robinia-weathered", SLAT_W, "horizontal-plate-up", 0.6, ref="en-338-2016"),
    region("bench-frame", "Bench leg frames, cross tubes and foot bar (zinc)", "Bankbeinrahmen, Querrohre und Fußstange (Zink)", "steel-galvanised", BF_D, "cylinder", 0.5),
    region("foot-roller", "EPDM foot roller", "EPDM-Fußrolle", "epdm-worn", 2 * ROLL_R, "cylinder", 0.55, ref="incropera-2011"),
    region("footings", "Six buried concrete footings", "Sechs eingegrabene Betonfundamente", "concrete", 0.5, "horizontal-plate-up", 0.0, src="literature", ref="iso-10456-2007"),
]

ENV = {"airTempC": 30, "meanRadiantTempC": 42, "skyTempC": 14, "groundTempC": 41, "relativeHumidity": 0.38, "windMS": 1.5, "windDirectionDeg": 60, "solarWM2": 850, "diffuseFraction": 0.15, "pressurePa": 101325}
SOIL_T = 17


def buried_share(rid):
    """🪨️ Share of the area of a region below the impact surface (y < 0), in contact with soil and concrete."""
    if rid == "pull-up-posts":
        b = 2 * (2 * math.pi * P_R * (0 - P_BOT) + math.pi * P_R**2)
    elif rid == "dip-frame":
        b = 4 * (2 * math.pi * DIP_R * (0 - DIP_BOT) + math.pi * DIP_R**2)
    elif rid == "bench-frame":
        b = 4 * (2 * math.pi * BF_R * (0 - B_BOT) + math.pi * BF_R**2)
    else:
        return 0.0
    return round(b / A[rid], 3)


def rb(rid, sun, extra=None, exposed_cut=0.0):
    """🌬️ The binding of one region: exposed share above ground, sunlit share, soil contact below ground and a further contact (hands, back, shoes)."""
    bur = buried_share(rid)
    if rid == "footings":
        return {"exposedFraction": 0.0, "sunlitFraction": 0.0, "contact": {"fraction": 1.0, "tempC": SOIL_T}}
    b = {"exposedFraction": round(1 - bur - exposed_cut, 3), "sunlitFraction": sun}
    if extra:
        b["contact"] = {"fraction": round(bur + extra[0], 3), "tempC": round((bur * SOIL_T + extra[0] * extra[1]) / (bur + extra[0]), 1)}
    elif bur > 0:
        b["contact"] = {"fraction": bur, "tempC": SOIL_T}
    return b


def thermal_binding(kind):
    """🏃️ Region bindings of the three named poses (open summer sun at midday, impact gravel at 41 degC, soil at 17 degC)."""
    if kind == "in-service":
        regs = {"pull-up-posts": rb("pull-up-posts", 0.32), "pull-up-bars": rb("pull-up-bars", 0.34), "dip-frame": rb("dip-frame", 0.33), "bench-board": rb("bench-board", 0.4), "bench-frame": rb("bench-frame", 0.3), "foot-roller": rb("foot-roller", 0.36), "footings": rb("footings", 0)}
    elif kind == "maintenance":
        regs = {"pull-up-posts": rb("pull-up-posts", 0.28), "pull-up-bars": rb("pull-up-bars", 0.3, (0.04, 30), 0.04), "dip-frame": rb("dip-frame", 0.33), "bench-board": rb("bench-board", 0.4), "bench-frame": rb("bench-frame", 0.3), "foot-roller": rb("foot-roller", 0.36), "footings": rb("footings", 0)}
    else:
        regs = {"pull-up-posts": rb("pull-up-posts", 0.3), "pull-up-bars": rb("pull-up-bars", 0.3, (0.05, 33), 0.05), "dip-frame": rb("dip-frame", 0.3, (0.02, 33), 0.02), "bench-board": rb("bench-board", 0.22, (0.12, 32), 0.12), "bench-frame": rb("bench-frame", 0.26), "foot-roller": rb("foot-roller", 0.3, (0.15, 30), 0.15), "footings": rb("footings", 0)}
    return {"regions": regs, "emitters": {}}


BW = 80 * G
SIT = 75 * G
HOOK = 150.0

LOADS_IN_USE = [
    {"id": "pull-up-left", "kind": "occupant", "title": T("Athlete hanging on the high bar, left hand (half of 80 kg)", "Sportler hängt an der hohen Stange, linke Hand (die Hälfte von 80 kg)"), "anchor": "grip-high-left", "fxN": 0, "fyN": r6(-BW / 2), "source": "authored-estimate", "reference": "din-33402-2-2020"},
    {"id": "pull-up-right", "kind": "occupant", "title": T("Athlete hanging on the high bar, right hand (half of 80 kg)", "Sportler hängt an der hohen Stange, rechte Hand (die Hälfte von 80 kg)"), "anchor": "grip-high-right", "fxN": 0, "fyN": r6(-BW / 2), "source": "authored-estimate", "reference": "din-33402-2-2020"},
    {"id": "dip-left", "kind": "occupant", "title": T("Second athlete supported on the dip bars, left bar (half of 80 kg)", "Zweiter Sportler im Stütz am Barren, linker Holm (die Hälfte von 80 kg)"), "anchor": "dip-left", "fxN": 0, "fyN": r6(-BW / 2), "source": "authored-estimate", "reference": "din-33402-2-2020"},
    {"id": "dip-right", "kind": "occupant", "title": T("Second athlete supported on the dip bars, right bar (half of 80 kg)", "Zweiter Sportler im Stütz am Barren, rechter Holm (die Hälfte von 80 kg)"), "anchor": "dip-right", "fxN": 0, "fyN": r6(-BW / 2), "source": "authored-estimate", "reference": "din-33402-2-2020"},
    {"id": "sit-up-hips", "kind": "occupant", "title": T("Third visitor doing sit-ups: back and hips on the board (75 kg plus the 150 N the hooked feet pull)", "Dritter Besucher bei Sit-ups: Rücken und Hüfte auf der Bank (75 kg plus die 150 N, die die eingehakten Füße ziehen)"), "anchor": "sit-up-hips", "fxN": 0, "fyN": r6(-(SIT + HOOK)), "source": "authored-estimate", "reference": "din-33402-2-2020"},
    {"id": "sit-up-feet", "kind": "occupant", "title": T("Feet hooked under the roller, pulling up 150 N", "Füße unter der Rolle eingehakt, 150 N nach oben"), "anchor": "sit-up-feet", "fxN": 0, "fyN": HOOK, "source": "authored-estimate", "reference": "din-en-16630-2015"},
]

SOILS = [f"soil-{b.replace('footing-', '')}" for b, *_ in FOOTINGS]
DROP = -0.85


def hand_loads():
    """🤲 The workers' hands as applied carrying forces: each hand pair holds half the weight of the high bar (the hands lie symmetric about its centre)."""
    half = round(round(M["bar-high"], 2) * G / 2, 1)
    return [{"id": f"carry-{k}", "kind": "equipment", "title": T(f"Worker's hands carrying half of the bar near post {k.upper()}", f"Hände des Arbeiters tragen die halbe Stange bei Pfosten {k.upper()}"), "anchor": f"hand-{k}", "fxN": 0, "fyN": half, "source": "authored-estimate"} for k in ("a", "b")]


POSES = [
    {"id": "in-service", "title": T("In service", "Im Einsatz"), "description": T("The station as it stands in the park after years of use: empty at midday in open summer sun, on its six buried footings in the loose impact gravel, the grips worn to bare zinc and rust at the ground line and the welds.", "Die Station, wie sie nach Jahren im Park steht: leer, mittags in offener Sommersonne, auf ihren sechs eingegrabenen Fundamenten im losen Fallschutzkies, die Griffzonen bis aufs Zink abgegriffen, Rost an der Bodenlinie und den Schweißnähten."), "bones": {}, "ground": "authored", "contacts": SOILS, "thermal": thermal_binding("in-service"), "loads": []},
    {"id": "maintenance", "title": T("Bar replacement", "Stangenwechsel"), "description": T("The depot crew has opened the four clamp caps of the worn high bar and lowers the 2.9 kg tube 0.85 m by hand to chest height between the posts; the replacement bar goes up the same way; posts, low bar, dip frame and bench stay.", "Der Bauhof hat die vier Schellendeckel der abgegriffenen hohen Stange geöffnet und senkt das 2,9-kg-Rohr von Hand 0,85 m auf Brusthöhe zwischen die Pfosten ab; die Ersatzstange geht denselben Weg hinauf; Pfosten, niedrige Stange, Barren und Bank bleiben."), "bones": {"bar-high": {"dx": DROP}}, "ground": "authored", "contacts": SOILS, "thermal": thermal_binding("maintenance"), "loads": hand_loads()},
    {"id": "in-use", "title": T("Three athletes", "Drei Sportler"), "description": T("An athlete of 80 kg hangs from the high bar with a shoulder-wide grip, a second one holds the support on the dip bars, a third visitor of 75 kg does decline sit-ups with the feet hooked under the roller; their bodies shade bars and board and warm the grips.", "Ein Sportler mit 80 kg hängt schulterbreit an der hohen Stange, ein zweiter hält den Stütz am Barren, ein dritter Besucher mit 75 kg macht Sit-ups auf der Schrägbank mit den Füßen unter der Rolle; ihre Körper beschatten Stangen und Bank und wärmen die Griffe."), "bones": {}, "ground": "authored", "contacts": SOILS, "thermal": thermal_binding("in-use"), "loads": LOADS_IN_USE},
    {"id": "hands-on", "key": True, "title": T("Hands on the bar", "Hände an der Stange"), "description": T("Key of the bar-change clip: the workers carry the high bar while it still lies in its open clamps.", "Schlüssel des Stangenwechsel-Clips: die Arbeiter tragen die hohe Stange, solange sie noch in ihren offenen Schellen liegt."), "bones": {}, "ground": "authored", "contacts": SOILS, "thermal": thermal_binding("in-service"), "loads": hand_loads()},
]

PU_EXP = thermal_binding("in-service")["regions"]["pull-up-bars"]["exposedFraction"]

CLIPS = [
    {
        "id": "bar-change",
        "title": T("Change the high bar", "Hohe Stange wechseln"),
        "description": T("Two workers take the worn bar in their hands, the clamp caps come off, the bar is lowered 0.85 m in 3 s, held while it is swapped, and the replacement is lifted back into the clamps.", "Zwei Arbeiter nehmen die abgegriffene Stange in die Hände, die Schellendeckel kommen ab, die Stange wird in 3 s um 0,85 m abgesenkt, gehalten, während sie getauscht wird, und die Ersatzstange wird wieder in die Schellen gehoben."),
        "durationS": 12,
        "fps": 30,
        "loop": False,
        "rootMotion": "none",
        "keys": [{"t": 0, "pose": "in-service", "ease": "smooth"}, {"t": 0.5, "pose": "hands-on", "ease": "smooth"}, {"t": 1, "pose": "hands-on", "ease": "smooth"}, {"t": 4, "pose": "maintenance", "ease": "step"}, {"t": 8, "pose": "maintenance", "ease": "smooth"}, {"t": 11, "pose": "hands-on", "ease": "step"}, {"t": 11.5, "pose": "hands-on", "ease": "smooth"}, {"t": 12, "pose": "in-service", "ease": "step"}],
        "contacts": [{"anchor": a, "fromS": 0, "toS": 12} for a in SOILS],
        "phases": [{"id": "open", "fromS": 0, "toS": 1, "title": T("Open the clamps", "Schellen öffnen")}, {"id": "lower", "fromS": 1, "toS": 4, "title": T("Lower", "Absenken")}, {"id": "swap", "fromS": 4, "toS": 8, "title": T("Swap the bar", "Stange tauschen")}, {"id": "raise", "fromS": 8, "toS": 11, "title": T("Raise the replacement", "Ersatz hochheben")}, {"id": "close", "fromS": 11, "toS": 12, "title": T("Close the clamps", "Schellen schließen")}],
        "events": [{"t": 0.5, "kind": "load-on", "title": T("Hands take the bar", "Hände nehmen die Stange"), "target": "carry-a"}, {"t": 4, "kind": "motion", "title": T("Bar at chest height", "Stange auf Brusthöhe")}, {"t": 8, "kind": "motion", "title": T("Replacement goes up", "Ersatz geht hoch")}, {"t": 11.5, "kind": "load-off", "title": T("Clamps closed, hands off", "Schellen zu, Hände weg"), "target": "carry-a"}],
        "tracks": [{"target": "region:pull-up-bars:sunlitFraction", "ease": "smooth", "keys": [{"t": 0, "v": 0.34}, {"t": 4, "v": 0.3}, {"t": 8, "v": 0.3}, {"t": 11, "v": 0.34}]}],
    },
    {
        "id": "pull-up-set",
        "title": T("Pull-ups, dips and sit-ups", "Klimmzüge, Dips und Sit-ups"),
        "description": T("One repetition every 3 s: the athlete pulls up from the dead hang (the bar load peaks at 1.5 times body weight), slows at the top and brakes on the way down; the dipper and the sit-up visitor work in their own rhythm, so the loads on the three parts never peak together.", "Eine Wiederholung alle 3 s: der Sportler zieht sich aus dem toten Hang hoch (die Stangenlast erreicht das 1,5-Fache des Körpergewichts), wird oben langsamer und bremst auf dem Weg nach unten; der Barrensportler und der Sit-up-Besucher arbeiten in eigenem Takt, daher erreichen die Lasten der drei Teile nie gleichzeitig ihr Maximum."),
        "durationS": 6,
        "fps": 30,
        "loop": True,
        "rootMotion": "none",
        "keys": [{"t": 0, "pose": "in-use", "ease": "linear"}, {"t": 6, "pose": "in-use", "ease": "linear"}],
        "contacts": [{"anchor": a, "fromS": 0, "toS": 6} for a in SOILS],
        "phases": [{"id": "pull-1", "fromS": 0, "toS": 1.2, "title": T("Pull up", "Hochziehen")}, {"id": "lower-1", "fromS": 1.2, "toS": 3, "title": T("Lower", "Ablassen")}, {"id": "pull-2", "fromS": 3, "toS": 4.2, "title": T("Pull up", "Hochziehen")}, {"id": "lower-2", "fromS": 4.2, "toS": 6, "title": T("Lower", "Ablassen")}],
        "events": [{"t": 0.4, "kind": "load-on", "title": T("Pull: peak bar load", "Zug: höchste Stangenlast"), "target": "pull-up-left"}, {"t": 3.4, "kind": "load-on", "title": T("Pull: peak bar load", "Zug: höchste Stangenlast"), "target": "pull-up-left"}],
        "tracks": [
            {"target": f"load:{lid}", "ease": "smooth", "keys": [{"t": 0, "v": 1.0}, {"t": 0.4, "v": 1.5}, {"t": 1.2, "v": 0.7}, {"t": 2.2, "v": 1.25}, {"t": 3.0, "v": 1.0}, {"t": 3.4, "v": 1.5}, {"t": 4.2, "v": 0.7}, {"t": 5.2, "v": 1.25}, {"t": 6.0, "v": 1.0}]}
            for lid in ("pull-up-left", "pull-up-right")
        ] + [
            {"target": f"load:{lid}", "ease": "smooth", "keys": [{"t": 0, "v": 1.0}, {"t": 1.0, "v": 0.8}, {"t": 2.0, "v": 1.35}, {"t": 3.0, "v": 1.0}, {"t": 4.0, "v": 0.8}, {"t": 5.0, "v": 1.35}, {"t": 6.0, "v": 1.0}]}
            for lid in ("dip-left", "dip-right")
        ] + [
            {"target": "load:sit-up-feet", "ease": "smooth", "keys": [{"t": 0, "v": 1.0}, {"t": 1.5, "v": 1.8}, {"t": 3.0, "v": 1.0}, {"t": 4.5, "v": 1.8}, {"t": 6.0, "v": 1.0}]},
            {"target": "region:pull-up-bars:sunlitFraction", "ease": "smooth", "keys": [{"t": 0, "v": 0.3}, {"t": 1.2, "v": 0.26}, {"t": 3.0, "v": 0.3}, {"t": 4.2, "v": 0.26}, {"t": 6.0, "v": 0.3}]},
        ],
    },
]


def sec(area, i, e, mat, en, de):
    """📏️ One member section."""
    return {"areaM2": float(f"{area:.6g}"), "secondMomentM4": float(f"{i:.6g}"), "extremeFibreM": r6(e), "material": mat, "title": T(en, de)}


SECTIONS = {
    "footing-a": sec(FA * FA, FA * FA**3 / 12, FA / 2, "concrete", "Footing 500 x 500 mm", "Fundament 500 x 500 mm"),
    "footing-b": sec(FA * FA, FA * FA**3 / 12, FA / 2, "concrete", "Footing 500 x 500 mm", "Fundament 500 x 500 mm"),
    "footing-dip-front": sec(DIP_FX * DIP_FZ, DIP_FZ * DIP_FX**3 / 12, DIP_FX / 2, "concrete", "Strip footing 400 x 920 mm", "Streifenfundament 400 x 920 mm"),
    "footing-dip-rear": sec(DIP_FX * DIP_FZ, DIP_FZ * DIP_FX**3 / 12, DIP_FX / 2, "concrete", "Strip footing 400 x 920 mm", "Streifenfundament 400 x 920 mm"),
    "footing-bench-low": sec(BFL_X * BFL_Z, BFL_Z * BFL_X**3 / 12, BFL_X / 2, "concrete", "Strip footing 300 x 500 mm", "Streifenfundament 300 x 500 mm"),
    "footing-bench-high": sec(BFH_X * BFH_Z, BFH_Z * BFH_X**3 / 12, BFH_X / 2, "concrete", "Strip footing 300 x 600 mm", "Streifenfundament 300 x 600 mm"),
    "post-a": sec(tube_area(P_D, P_T), tube_i(P_D, P_T), P_R, "steel-painted", "Tube o114.3 x 3.6 (EN 10219-2)", "Rohr o114,3 x 3,6 (EN 10219-2)"),
    "post-b": sec(tube_area(P_D, P_T), tube_i(P_D, P_T), P_R, "steel-painted", "Tube o114.3 x 3.6 (EN 10219-2)", "Rohr o114,3 x 3,6 (EN 10219-2)"),
    "bar-high": sec(tube_area(BAR_D, BAR_T), tube_i(BAR_D, BAR_T), BAR_R, "steel-galvanised", "Tube o33.7 x 3.2 (EN 10255 M)", "Rohr o33,7 x 3,2 (EN 10255 M)"),
    "bar-low": sec(tube_area(BAR_D, BAR_T), tube_i(BAR_D, BAR_T), BAR_R, "steel-galvanised", "Tube o33.7 x 3.2 (EN 10255 M)", "Rohr o33,7 x 3,2 (EN 10255 M)"),
    "dip-post-front": sec(2 * tube_area(DIP_D, DIP_T), 2 * tube_i(DIP_D, DIP_T), DIP_R, "steel-galvanised", "Two tubes o48.3 x 3.2 (EN 10219-2)", "Zwei Rohre o48,3 x 3,2 (EN 10219-2)"),
    "dip-post-rear": sec(2 * tube_area(DIP_D, DIP_T), 2 * tube_i(DIP_D, DIP_T), DIP_R, "steel-galvanised", "Two tubes o48.3 x 3.2 (EN 10219-2)", "Zwei Rohre o48,3 x 3,2 (EN 10219-2)"),
    "dip-bar-l": sec(tube_area(DIP_D, DIP_T), tube_i(DIP_D, DIP_T), DIP_R, "steel-galvanised", "Bar o48.3 x 3.2 (EN 10219-2)", "Holm o48,3 x 3,2 (EN 10219-2)"),
    "dip-bar-r": sec(tube_area(DIP_D, DIP_T), tube_i(DIP_D, DIP_T), DIP_R, "steel-galvanised", "Bar o48.3 x 3.2 (EN 10219-2)", "Holm o48,3 x 3,2 (EN 10219-2)"),
    "bench-frame-low": sec(2 * tube_area(BF_D, BF_T), 2 * tube_i(BF_D, BF_T), BF_R, "steel-galvanised", "Two legs o42.4 x 2.6 (EN 10219-2)", "Zwei Beine o42,4 x 2,6 (EN 10219-2)"),
    "bench-frame-high": sec(2 * tube_area(BF_D, BF_T), 2 * tube_i(BF_D, BF_T), BF_R, "steel-galvanised", "Two legs o42.4 x 2.6 (EN 10219-2)", "Zwei Beine o42,4 x 2,6 (EN 10219-2)"),
    "bench-board": sec(3 * SLAT_W * SLAT_T, 3 * SLAT_W * SLAT_T**3 / 12, SLAT_T / 2, "robinia-weathered", "Three Robinia slats 95 x 45 mm (D40, EN 338)", "Drei Robinienlatten 95 x 45 mm (D40, EN 338)"),
}

TITLES = {
    "ground": ("Reference frame at the impact surface", "Bezugsrahmen an der Fallschutzoberfläche"),
    "footing-a": ("Point footing of post A", "Punktfundament von Pfosten A"),
    "footing-b": ("Point footing of post B", "Punktfundament von Pfosten B"),
    "footing-dip-front": ("Strip footing of the front dip posts", "Streifenfundament der vorderen Barrenpfosten"),
    "footing-dip-rear": ("Strip footing of the rear dip posts", "Streifenfundament der hinteren Barrenpfosten"),
    "footing-bench-low": ("Strip footing of the low bench frame", "Streifenfundament des niedrigen Bankrahmens"),
    "footing-bench-high": ("Strip footing of the high bench frame", "Streifenfundament des hohen Bankrahmens"),
    "post-a": ("Pull-up post A with cap and clamps", "Klimmzugpfosten A mit Kappe und Schellen"),
    "post-b": ("Pull-up post B with cap and clamps", "Klimmzugpfosten B mit Kappe und Schellen"),
    "bar-high": ("High pull-up bar, lowered for replacement", "Hohe Klimmzugstange, zum Wechsel abgesenkt"),
    "bar-low": ("Low rowing bar", "Niedrige Ruderstange"),
    "dip-post-front": ("Front pair of dip posts", "Vorderes Barrenpfostenpaar"),
    "dip-post-rear": ("Rear pair of dip posts", "Hinteres Barrenpfostenpaar"),
    "dip-bar-l": ("Left dip bar", "Linker Barrenholm"),
    "dip-bar-r": ("Right dip bar", "Rechter Barrenholm"),
    "bench-frame-low": ("Low bench leg frame with cross tube", "Niedriger Bankbeinrahmen mit Querrohr"),
    "bench-frame-high": ("High bench leg frame, foot bar and roller", "Hoher Bankbeinrahmen, Fußstange und Rolle"),
    "bench-board": ("Sit-up board of three slats", "Sit-up-Liegefläche aus drei Latten"),
}

LENGTHS = {"ground": 0.1, "post-a": P_TOP - P_BOT, "post-b": P_TOP - P_BOT, "bar-high": BAR_X1 - BAR_X0, "bar-low": BAR_X1 - BAR_X0, "dip-post-front": DIP_Y - DIP_BOT, "dip-post-rear": DIP_Y - DIP_BOT, "dip-bar-l": DIP_X1 - DIP_X0, "dip-bar-r": DIP_X1 - DIP_X0, "bench-frame-low": YL - B_BOT, "bench-frame-high": FOOT_Y - B_BOT, "bench-board": B_L}
for _b, *_ in FOOTINGS:
    LENGTHS[_b] = FT - FB

JOINTS = {"bar-high": {"type": "prismatic", "axisDeg": 0, "minM": -0.9, "maxM": 0}}
MASS_REF = {"bench-board": "en-338-2016"}


def bone(id_):
    """🦴️ One bone placed from the world rest frames of BONES, with its mass at the computed centre."""
    d = BONES[id_]
    ox, oy = d["o"]
    p = d["p"]
    if p is None:
        x, y, ang = ox, oy, d["a"]
    else:
        px, py = BONES[p]["o"]
        pa = BONES[p]["a"]
        dx, dy = ox - px, oy - py
        c, s = math.cos(math.radians(-pa)), math.sin(math.radians(-pa))
        x, y, ang = dx * c - dy * s, dx * s + dy * c, d["a"] - pa
    b = {"id": id_, "parent": p, "title": T(*TITLES[id_]), "x": r6(x), "y": r6(y)}
    if d.get("z"):
        b["z"] = r6(d["z"])
    b.update({"angleDeg": r6(ang), "lengthM": r6(LENGTHS[id_]), "joint": JOINTS.get(id_, {"type": "fixed"})})
    if id_ in M:
        cu, cv = loc(id_, COM_WORLD[id_][0], COM_WORLD[id_][1])
        b["mass"] = {"kg": round(M[id_], 2), "comX": cu, "comY": cv, "distribution": "point", "source": "authored-estimate", "reference": MASS_REF.get(id_, "en-10219-2-2019" if not id_.startswith("footing") else "en-1991-1-1-2002")}
    if id_ in SECTIONS:
        b["section"] = SECTIONS[id_]
    return b


RIG = {"kind": "structural", "root": "ground", "bones": [bone(b) for b in BONES]}


def anchor(id_, bone_, x, y, z, kind, en, de, **kw):
    """🪝️ One anchor from a world rest point."""
    u, v = loc(bone_, x, y)
    a = {"id": id_, "bone": bone_, "x": u, "y": v}
    zl = z - BONES[bone_].get("z", 0.0)
    if abs(zl) > 1e-9:
        a["z"] = r6(zl)
    a.update({"kind": kind, "title": T(en, de)})
    a.update(kw)
    return a


ANCHORS = [anchor(f"soil-{b.replace('footing-', '')}", b, x, FB, 0, "support", f"Soil bearing under the {TITLES[b][0][0].lower() + TITLES[b][0][1:]}", f"Bodenpressung unter dem {TITLES[b][1]}", support="contact", normalDeg=90, footprintLengthM=sx, footprintWidthM=sz, frictionCoefficient=0.6) for b, x, sx, sz in FOOTINGS]
ANCHORS += [
    anchor("hand-a", "bar-high", 0.36, BAR_HI - BAR_R, 0, "load", "Worker's hands under the bar near post A", "Hände des Arbeiters unter der Stange bei Pfosten A"),
    anchor("hand-b", "bar-high", 1.04, BAR_HI - BAR_R, 0, "load", "Worker's hands under the bar near post B", "Hände des Arbeiters unter der Stange bei Pfosten B"),
    anchor("grip-high-left", "bar-high", 0.48, BAR_HI + BAR_R, 0, "load", "Left hand on the high bar (worn grip)", "Linke Hand an der hohen Stange (abgegriffen)"),
    anchor("grip-high-right", "bar-high", 0.92, BAR_HI + BAR_R, 0, "load", "Right hand on the high bar (worn grip)", "Rechte Hand an der hohen Stange (abgegriffen)"),
    anchor("dip-left", "dip-bar-l", 2.80, DIP_Y + DIP_R, DIP_Z, "load", "Left hand on the left dip bar", "Linke Hand am linken Barrenholm"),
    anchor("dip-right", "dip-bar-r", 2.80, DIP_Y + DIP_R, -DIP_Z, "load", "Right hand on the right dip bar", "Rechte Hand am rechten Barrenholm"),
    anchor("sit-up-hips", "bench-board", B_X0 + 0.45 * CS - SLAT_T * SN, B_Y0 + 0.45 * SN + SLAT_T * CS, 0, "load", "Back and hips on the sit-up board", "Rücken und Hüfte auf der Sit-up-Bank"),
    anchor("sit-up-feet", "bench-frame-high", XH, FOOT_Y - ROLL_R, 0, "load", "Insteps hooked under the foot roller", "Fußrücken unter der Fußrolle eingehakt"),
]

REFERENCES = [
    {"id": "din-en-16630-2015", "citation": "DIN EN 16630:2015 Permanently installed outdoor fitness equipment - Safety requirements and test methods (user loads, free height of fall, impact area, entrapment and grip requirements)."},
    {"id": "din-en-1176-1-2017", "citation": "DIN EN 1176-1:2017 Playground equipment and surfacing - Part 1: General safety requirements and test methods (grip diameter 16 to 45 mm, foundations covered or rounded, free height of fall for hanging by the hands)."},
    {"id": "din-en-1177-2018", "citation": "DIN EN 1177:2018 Impact attenuating playground surfacing - Methods of test for determination of impact attenuation (loose gravel 2/8 mm, 300 mm layer, up to 3 m critical fall height)."},
    {"id": "en-10219-2-2019", "citation": "EN 10219-2:2019 Cold formed welded steel structural hollow sections - Part 2: Tolerances, dimensions and sectional properties (CHS 114.3 x 3.6: 9.83 kg/m; 48.3 x 3.2: 3.56 kg/m; 42.4 x 2.6: 2.55 kg/m)."},
    {"id": "en-10255-2004", "citation": "EN 10255:2004+A1:2007 Non-alloy steel tubes suitable for welding and threading (medium series 33.7 x 3.2 mm)."},
    {"id": "en-iso-1461-2022", "citation": "EN ISO 1461:2022 Hot dip galvanized coatings on fabricated iron and steel articles (zinc coating 70 to 85 um on tubes)."},
    {"id": "en-338-2016", "citation": "EN 338:2016 Structural timber - Strength classes (D40: bending 40 MPa, mean modulus 13.0 GPa, mean density 700 kg/m3; Robinia commonly graded D40)."},
    {"id": "en-1991-1-1-2002", "citation": "EN 1991-1-1:2002 Eurocode 1: Actions on structures - Part 1-1: Densities, self-weight, imposed loads (normal-weight concrete 24 kN/m3, steel 77 kN/m3)."},
    {"id": "en-1992-1-1-2004", "citation": "EN 1992-1-1:2004 Eurocode 2: Design of concrete structures - Part 1-1 (C25/30: fck 25 MPa, fctm 2.6 MPa, Ecm 31 GPa)."},
    {"id": "en-1993-1-1-2005", "citation": "EN 1993-1-1:2005 Eurocode 3: Design of steel structures - Part 1-1 (S235: fy 235 MPa, E 210 GPa)."},
    {"id": "iso-10456-2007", "citation": "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (concrete 2400 kg/m3: 2.0 W/(m K), 1000 J/(kg K); steel 50 W/(m K))."},
    {"id": "din-33402-2-2020", "citation": "DIN 33402-2:2020 Ergonomics - Human body dimensions - Part 2: Values (body mass and shoulder breadth of adults)."},
    {"id": "ashrae-hof-2021", "citation": "ASHRAE Handbook - Fundamentals (2021), chapter 14 and 33: solar absorptance and emittance of surfaces (galvanised steel new 0.65 / 0.13, weathered and dirty 0.9 / 0.28; dark green paint about 0.8 to 0.9 / 0.9)."},
    {"id": "incropera-2011", "citation": "Bergman, T. L., Lavine, A. S., Incropera, F. P., DeWitt, D. P. (2011). Fundamentals of Heat and Mass Transfer, 7th ed., Wiley (emissivity of rubber, wood, concrete and oxidised steel)."},
    {"id": "fll-spielplatz-2008", "citation": "FLL (2008) Empfehlungen für Planung, Bau und Instandhaltung von Spielplätzen und Freiräumen zum Spielen, Bonn (inspection and maintenance of outdoor play and fitness equipment by the municipal depot)."},
]

SUMMARY = T(
    "A used municipal outdoor fitness station of the calisthenics type, 5.9 m long: two 114.3 mm powder-coated pull-up posts 2.52 m high with a high bar at 2.35 m and a low rowing bar at 1.00 m (33.7 mm zinc, held in cast clamps), parallel dip bars at 1.25 m and 0.52 m apart on four 48.3 mm posts, and a decline sit-up bench with three greyed Robinia slats and an EPDM foot roller, all on six buried concrete footings 0.70 m deep in loose impact gravel. Years of use wore the grips to bare zinc, rusted the posts at the ground line and the welds, flaked the green coat at the post feet, checked the board and flattened the roller where the feet hook. The depot opens the clamps and lowers the worn high bar by hand to replace it; three athletes pull up, dip and do sit-ups.",
    "Eine gebrauchte kommunale Outdoor-Fitnessstation vom Calisthenics-Typ, 5,9 m lang: zwei pulverbeschichtete Klimmzugpfosten 114,3 mm, 2,52 m hoch, mit einer hohen Stange auf 2,35 m und einer niedrigen Ruderstange auf 1,00 m (Zink 33,7 mm, in Gussschellen gehalten), ein Barren auf 1,25 m mit 0,52 m Holmabstand auf vier Pfosten 48,3 mm und eine schräge Sit-up-Bank mit drei vergrauten Robinienlatten und einer EPDM-Fußrolle, alles auf sechs eingegrabenen Betonfundamenten 0,70 m tief im losen Fallschutzkies. Jahre des Gebrauchs haben die Griffzonen bis aufs Zink abgegriffen, die Pfosten an der Bodenlinie und den Schweißnähten rosten lassen, die grüne Beschichtung an den Pfostenfüßen abplatzen lassen, die Bank rissig und die Rolle dort platt gemacht, wo die Füße einhaken. Der Bauhof öffnet die Schellen und senkt die abgegriffene hohe Stange von Hand zum Wechsel ab; drei Sportler machen Klimmzüge, Dips und Sit-ups.",
)

THERMAL_ASSUMPTIONS = [
    T("All seven regions are passive: solar gain on the sunlit share, long-wave exchange with the sky and the hot impact gravel, forced convection in 1.5 m/s of wind; the thin tubes (9 to 22 kJ/(m2 K) per outer area) follow the sun within minutes, the footings are buried and held by the soil at 17 degC.", "Alle sieben Regionen sind passiv: Solargewinn auf dem besonnten Anteil, langwelliger Austausch mit dem Himmel und dem heißen Fallschutzkies, erzwungene Konvektion bei 1,5 m/s Wind; die dünnen Rohre (9 bis 22 kJ/(m2 K) je Außenfläche) folgen der Sonne in Minuten, die Fundamente liegen im Boden und werden von ihm auf 17 degC gehalten."),
    T("Weathered surfaces differ from new ones (ASHRAE Fundamentals, authored estimates): weathered zinc absorbs 0.72 and emits 0.28 instead of about 0.65 and 0.13 when bright, so the bars get hot but radiate poorly; the chalked moss-green powder coat absorbs 0.84; the grips worn bright are lower (0.6 / 0.12); greyed Robinia 0.62; the sun-aged black EPDM roller 0.93 is the hottest touch surface of the station in summer sun.", "Verwitterte Flächen unterscheiden sich von neuen (ASHRAE Fundamentals, Schätzungen): verwittertes Zink absorbiert 0,72 und emittiert 0,28 statt etwa 0,65 und 0,13 im blanken Zustand, daher werden die Stangen heiß, strahlen aber schlecht ab; die kreidende moosgrüne Pulverbeschichtung absorbiert 0,84; die blank gegriffenen Zonen weniger (0,6 / 0,12); vergraute Robinie 0,62; die sonnengealterte schwarze EPDM-Rolle mit 0,93 ist in der Sommersonne die heißeste Berührfläche der Station."),
    T("Below the impact surface the embedded tube ends and the buried legs are in contact with soil and concrete at 17 degC (their area share per region); in use the hands warm the grips, the back the board and the shoes the roller by contact. Areas are the total outer areas of the 1:50 solids; exposure and contact belong to the poses.", "Unter der Fallschutzoberfläche stehen die eingebetteten Rohrenden und die eingegrabenen Beine mit Boden und Beton bei 17 degC in Kontakt (ihr Flächenanteil je Region); im Gebrauch wärmen die Hände die Griffe, der Rücken die Bank und die Schuhe die Rolle durch Kontakt. Flächen sind die gesamten Außenflächen der 1:50-Körper; Exposition und Kontakt gehören zu den Posen."),
    T("No element draws power: the pull-up posts state passive 0 W.", "Kein Element nimmt Leistung auf: die Klimmzugpfosten geben passiv 0 W an."),
]

STRUCTURE_ASSUMPTIONS = [
    T("In service every steel part is embedded 0.40 to 0.50 m in its own C25/30 footing (point footings 0.50 x 0.50 m under the pull-up posts, strip footings under each pair of dip posts and each bench frame, all 0.70 m deep to frost depth with the top 100 mm under the impact gravel); each footing bears on the soil as a contact support. The bars sit in cast clamps on the posts and are idealised as members fixed to post A; the parallel bars are welded to their posts.", "Im Einsatz ist jedes Stahlteil 0,40 bis 0,50 m in sein eigenes Fundament C25/30 eingebettet (Punktfundamente 0,50 x 0,50 m unter den Klimmzugpfosten, Streifenfundamente unter jedem Barrenpfostenpaar und jedem Bankrahmen, alle 0,70 m tief bis zur Frostgrenze mit der Oberkante 100 mm unter dem Fallschutzkies); jedes Fundament liegt als Kontaktauflager auf dem Boden. Die Stangen sitzen in Gussschellen an den Pfosten und sind als an Pfosten A eingespannte Stäbe idealisiert; der Barren ist an seine Pfosten geschweißt."),
    T("Use loads (DIN EN 16630 user loads, authored estimates after DIN 33402-2 body masses): a hanging athlete of 80 kg puts half the body weight on each hand of the high bar, rising to 1.5 times body weight while pulling up (kipping peaks up to 2 times are not modelled); the dipper splits 80 kg over both bars; the sit-up visitor of 75 kg presses back and hips onto the board while the hooked feet pull the roller up by 150 N, 270 N at the top of each sit-up. The 33.7 x 3.2 mm bar spans 1.19 m: at 1.18 kN in two points 0.44 m apart its bending stress stays far below the 235 MPa of S235.", "Gebrauchslasten (Nutzerlasten DIN EN 16630, Schätzungen nach Körpermassen DIN 33402-2): ein hängender Sportler mit 80 kg gibt das halbe Körpergewicht auf jede Hand an der hohen Stange, beim Hochziehen bis zum 1,5-Fachen (Kipping-Spitzen bis zum 2-Fachen sind nicht modelliert); der Barrensportler verteilt 80 kg auf beide Holme; der Sit-up-Besucher mit 75 kg drückt Rücken und Hüfte auf die Bank, während die eingehakten Füße die Rolle mit 150 N nach oben ziehen, 270 N oben bei jedem Sit-up. Die Stange 33,7 x 3,2 mm spannt 1,19 m: bei 1,18 kN in zwei Punkten im Abstand 0,44 m bleibt ihre Biegespannung weit unter den 235 MPa von S235."),
    T("In the bar-replacement pose the lowered bar hangs on two workers' hands and the rest stays on its footings: the hands are applied as their carrying forces, 14 N per worker and together the 28 N weight of the bar, because supports of the archive are springs of equal stiffness that would share the weight of the whole station between hands and footings.", "In der Pose Stangenwechsel liegt die abgesenkte Stange in den Händen zweier Arbeiter und der Rest bleibt auf seinen Fundamenten: die Hände sind als ihre Tragkräfte angesetzt, 14 N je Arbeiter und zusammen die 28 N Gewicht der Stange, weil Auflager des Archivs Federn gleicher Steifigkeit sind, die das Gewicht der ganzen Station auf Hände und Fundamente verteilen würden."),
]


def document():
    """📄️ The whole asset document."""
    xs = [x - sx / 2 for _, x, sx, _ in FOOTINGS] + [x + sx / 2 for _, x, sx, _ in FOOTINGS]
    asset = {
        "id": SLUG,
        "category": "objects",
        "title": T("Used Outdoor Fitness Station", "Gebrauchtes Outdoor-Fitnessgerät"),
        "summary": SUMMARY,
        "tags": ["used", "park", "public-space", "sport", "fitness", "calisthenics", "pull-up-bar", "parallel-bars", "sit-up-bench", "street-furniture", "galvanised-steel", "powder-coated-steel", "hardwood", "concrete", "footing"],
        "provenance": {
            "author": "Semio staffage content (used park objects wave)",
            "method": "handcrafted",
            "created": "2026-11-22",
            "references": REFERENCES,
            "notes": T(
                "Catalogue-typical municipal calisthenics station after DIN EN 16630 (type, no manufacturer): pull-up bar 2.35 m, rowing bar 1.00 m, dip bars 1.25 m and 0.52 m apart, decline sit-up bench 0.40 to 0.80 m; tubes after EN 10219-2 and EN 10255, footings to frost depth; masses from section, length and density, footings included. Nothing is measured; dimensions are literature and authored estimates, the traces of use are illustrative.",
                "Katalogtypische kommunale Calisthenics-Station nach DIN EN 16630 (Typ, kein Hersteller): Klimmzugstange 2,35 m, Ruderstange 1,00 m, Barren 1,25 m mit 0,52 m Abstand, schräge Sit-up-Bank 0,40 bis 0,80 m; Rohre nach EN 10219-2 und EN 10255, Fundamente bis zur Frostgrenze; Massen aus Querschnitt, Länge und Dichte, Fundamente eingeschlossen. Nichts ist gemessen; Abmessungen sind Literatur und Schätzungen, die Gebrauchsspuren sind illustrativ.",
            ),
        },
        "dimensions": {"heightM": r6(P_TOP + CAP_H - FB), "widthM": r6(max(xs) - min(xs)), "depthM": r6(DIP_FZ), "massKg": round(TOTAL, 1)},
        "materials": MATERIALS,
        "rig": RIG,
        "volumes": volumes(),
        "anchors": ANCHORS,
        "representations": {str(s): representation(s) for s in (500, 200, 100, 50)},
        "symbols": [],
        "poses": POSES,
        "clips": CLIPS,
        "thermal": {"environment": ENV, "passengers": [], "operating": [], "regions": REGIONS, "emitters": [], "assumptions": THERMAL_ASSUMPTIONS},
        "structure": {"gravityMS2": G, "dynamics": "quasi-static", "supports": SOILS, "assumptions": STRUCTURE_ASSUMPTIONS, "clinical": False},
        "presentation": {"defaultPose": "in-service", "defaultClip": "pull-up-set", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}


if __name__ == "__main__":
    doc = document()
    with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf-8") as f:
        json.dump(doc, f, ensure_ascii=False, indent=None, separators=(", ", ": "))
        f.write("\n")
    print("[DEBUG] masses", {k: round(v, 2) for k, v in M.items()}, "total", round(TOTAL, 1))
    print("[DEBUG] areas", {k: round(v, 4) for k, v in A.items()})
    print("[DEBUG] dims", doc["asset"]["dimensions"], "theta", round(TH, 3), "XL", round(XL, 4), "YL", round(YL, 4), "XH", round(XH, 4), "YH", round(YH, 4))
    for s in SCALES:
        v = doc["asset"]["volumes"][str(s)]
        print("[DEBUG] scale", s, "body", len(v["body"]), "thermal", len(v["thermal"]), "structure", len(v["structure"]))
