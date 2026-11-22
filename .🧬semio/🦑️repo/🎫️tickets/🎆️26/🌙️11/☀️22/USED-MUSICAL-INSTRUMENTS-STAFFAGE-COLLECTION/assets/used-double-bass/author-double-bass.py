"""🎻 Authoritative generator of the used 3/4 orchestral double bass: every hand-chosen number of the flat-back workshop bass in its opened padded cover with the bass wheel on the endpin and its bass stool, drawn in 2D and modelled in 3D per scale from one part list, written to 🔣️.json next to this file."""
import json
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from author_bass_geometry import (Mesh, apply, catmull, chain, circle, clearance, cut_at_v, ellipse, half_at, hexahedron, mat_w, r6, solve_rig, symmetric_body, to_world)  # noqa: E402

SLUG = "used-double-bass"
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}
SCALES = (50, 100, 200, 500)


def T(en, de):
    """🗣 A bilingual text, English first."""
    return {"en": en, "de": de}


LOWER = [(0.0, 0.0), (0.004, 0.06), (0.02, 0.15), (0.05, 0.225), (0.10, 0.285), (0.16, 0.322), (0.22, 0.338), (0.27, 0.340), (0.33, 0.333), (0.39, 0.313), (0.43, 0.293), (0.468, 0.278)]
CBOUT = [(0.468, 0.278), (0.49, 0.236), (0.53, 0.204), (0.575, 0.192), (0.62, 0.198), (0.66, 0.218), (0.692, 0.250)]
UPPER = [(0.692, 0.250), (0.72, 0.242), (0.77, 0.246), (0.83, 0.254), (0.88, 0.255), (0.93, 0.246), (0.98, 0.222), (1.02, 0.188), (1.055, 0.140), (1.08, 0.097), (1.10, 0.065)]
BAG_UP = [(-0.025, 0.0), (-0.022, 0.07), (-0.010, 0.16), (0.02, 0.243), (0.07, 0.307), (0.13, 0.346), (0.20, 0.362), (0.27, 0.365), (0.34, 0.358), (0.41, 0.337), (0.47, 0.307), (0.53, 0.270), (0.59, 0.254),
          (0.65, 0.266), (0.71, 0.280), (0.78, 0.278), (0.85, 0.280), (0.90, 0.280), (0.95, 0.272), (1.00, 0.246), (1.04, 0.208), (1.08, 0.155), (1.11, 0.115), (1.13, 0.092)]
BAG_LO = BAG_UP
BODY_L, SCROLL_TOP = 1.10, 1.80
RIB = 0.100
ARCH_H = 0.024
BREAK_U, BREAK_DROP = 0.80, 0.055
BAND_RIM, BACK_IN, BACK_OUT = -0.300, 0.100, 0.125
FLAP_T = 0.025
SLOT = 0.030
BRIDGE_U = 0.48
NUT_U = 1.53
FB_END = 0.63
TAIL_U = (0.035, 0.315)
TAIL_W = (-0.114, -0.172)
STRINGS = [("e", -1.5, 0.0013), ("a", -0.5, 0.0011), ("d", 0.5, 0.00095), ("g", 1.5, 0.0008)]
SLEEVE = [(1.08, 0.105, 0.135), (1.15, 0.085, 0.125), (1.30, 0.075, 0.118), (1.50, 0.078, 0.118), (1.60, 0.088, 0.125), (1.70, 0.085, 0.122), (1.79, 0.072, 0.110), (1.845, 0.050, 0.080), (1.87, 0.018, 0.030)]
SLEEVE_WC = -0.110
STOOL_X = -1.15
SEAT_Y = 0.685
ENDPIN_IN = 0.388
ROD_L, TIP_R = 0.441, 0.009
AXLE_U, WHEEL_R = -0.038, 0.0375
PLAYED_E = 0.200
LEAN = 15.0
CARRY_DEG = 72.0
TIP_X = -0.40


def w_ft(u):
    """🎼 Depth of the fingerboard surface along its centre line (linear from the end over the body to the nut)."""
    return -0.253 + 0.1 * (u - FB_END)


def fb_half(u):
    """↔ Half-width of the fingerboard (120 mm at its end, 45 mm at the nut)."""
    return 0.060 - (u - FB_END) * (0.0375 / (NUT_U - FB_END))


def w_back(u):
    """🪵 Depth of the flat back: plane in the lower two thirds, broken 10 degrees towards the neck above the upper bout."""
    return RIB if u <= BREAK_U else RIB - (u - BREAK_U) * (BREAK_DROP / (BODY_L - BREAK_U))


def arch_long(u):
    """🌉 Longitudinal factor of the arching: flatter towards both ends of the body."""
    return 0.35 + 0.65 * math.sin(math.pi * max(0.0, min(BODY_L, u)) / BODY_L) ** 0.6


def w_arch(u, s):
    """⛰ Depth of the arched spruce top at the fraction `s` of the local half-width (crown 24 mm above the rib edge)."""
    return -RIB - ARCH_H * arch_long(u) * (1 - s ** 2.2)


def bridge_w(v):
    """🌉 Depth of the bridge crown under a string at lateral position `v` (radius of the crown)."""
    return -0.2835 + 1.4 * v * v


STRING_LINE_K = None


def outline(scale):
    """🎻 Bottom-to-top half outline of the body for the scale (corners sharp at the lower and upper corners of the C-bouts)."""
    per = {50: 5, 100: 3, 200: 2, 500: 1}[scale]
    if scale == 500:
        return [(0.0, 0.0), (0.02, 0.15), (0.08, 0.27), (0.20, 0.335), (0.30, 0.338), (0.40, 0.31), (0.47, 0.278), (0.575, 0.20), (0.69, 0.25), (0.80, 0.25), (0.90, 0.252), (1.00, 0.205), (1.06, 0.13), (1.10, 0.065)]
    if scale == 200:
        return chain([LOWER[::2] + [LOWER[-1]], CBOUT[::2] + [CBOUT[-1]] if CBOUT[::2][-1] != CBOUT[-1] else CBOUT[::2], UPPER[::2] + ([UPPER[-1]] if UPPER[::2][-1] != UPPER[-1] else [])], per)
    return chain([LOWER, CBOUT, UPPER], per)


def bag_half(scale):
    """👜 The outer half outline of the padded cover, from the endpin opening to the neck sleeve."""
    per = {50: 3, 100: 2, 200: 1, 500: 1}[scale]
    pts = BAG_UP if scale != 500 else BAG_UP[::2] + [BAG_UP[-1]]
    return catmull(pts, per)


def bag_loop(scale):
    """🔁 The closed outer outline of the cover in its own frame, counter-clockwise."""
    h = bag_half(scale)
    return [(u, v) for u, v in h] + [(u, -v) for u, v in reversed(h)][1:-1]


def band_half(scale, side, thick, slot=SLOT):
    """🧵 One half of the gusset band as a simple polygon: outer cover outline and the bass rib outline, cut by the endpin slot and the neck opening; `thick` grows the band inwards at coarse scales."""
    outer = cut_at_v(bag_half(scale), slot)
    inner = cut_at_v(outline(scale), slot)
    if thick:
        inner = [(u, max(slot, v - thick)) for u, v in inner]
    poly = outer + list(reversed(inner))
    return [(u, side * v) for u, v in poly] if side > 0 else [(u, side * v) for u, v in reversed(poly)]


DELTA = None
B_WORLD = None
H_WORLD = None
Q_PIVOT = None


def rest_geometry():
    """📐 The rest tilt of the lying cover (it rests on its lower and upper bout) and the world origin of the bass and cover frames."""
    global DELTA, B_WORLD
    lo = bag_half(50)
    best = None
    for k in range(4000):
        d = math.radians(k * 0.005)
        heights = [u * math.sin(d) + v * math.cos(d) for u, v in lo]
        top = max(heights)
        cand = [lo[i] for i, h in enumerate(heights) if h > top - 1e-4]
        if any(p[0] < 0.5 for p in cand) and any(p[0] > 0.6 for p in cand):
            best = d
            break
    DELTA = math.degrees(best)
    hmax = max(u * math.sin(best) + v * math.cos(best) for u, v in lo)
    B_WORLD = (0.0, hmax, 0.0)
    return DELTA


def cover_to_world(p):
    """🌍 A point of the cover or bass frame at rest in world coordinates."""
    d = math.radians(-DELTA)
    u, v, w = p
    return (B_WORLD[0] + u * math.cos(d) - v * math.sin(d), B_WORLD[1] + u * math.sin(d) + v * math.cos(d), w)


def world_to_cover(p):
    """🧭 A world point in the rest frame of the cover and the bass."""
    d = math.radians(DELTA)
    x, y, z = p[0] - B_WORLD[0], p[1] - B_WORLD[1], p[2]
    return (x * math.cos(d) - y * math.sin(d), x * math.sin(d) + y * math.cos(d), z)


def hull_contacts():
    """🪨 The two points of the lower cover outline that rest on the floor (lower and upper bout)."""
    lo = bag_half(50)
    hs = [(cover_to_world((u, -v, 0))[1], u, v) for u, v in lo]
    a = min([h for h in hs if h[1] < 0.5])
    b = min([h for h in hs if h[1] > 0.6])
    return (a[1], -a[2]), (b[1], -b[2])


class Part:
    """🧱 One physical part of one scale: its 3D solid in the bone frame and its 2D shapes in the three views, from the same numbers."""

    def __init__(self, pid, bone, material, tone, region, d3, d2, role="body", views=("elevation", "front", "plan"), extra=None, body=True, thermal=True, detail=False):
        self.id, self.bone, self.material, self.tone, self.region = pid, bone, material, tone, region
        self.d3, self.d2, self.role, self.views, self.extra = d3, d2, role, views, extra or {}
        self.body, self.thermal, self.detail = body, thermal, detail

    def solid(self):
        """🧊 The volume solid."""
        s = {"id": self.id, "bone": self.bone, "material": self.material, "role": "body" if self.role in ("body", "texture-body") else self.role, "tone": self.tone}
        s.update(self.d3)
        for key in ("slender", "sheet", "texture", "envelope", "sectionFaces", "tags"):
            if key in self.extra:
                s[key] = self.extra[key]
        return s


def ext_w(profile, w0, w1, holes=None):
    """🧱 3D: an extrusion of a (u, v) profile along w."""
    d = {"kind": "extrude", "plane": "xy", "profile": [[r6(a), r6(b)] for a, b in profile], "from": r6(w0), "to": r6(w1)}
    if holes:
        d["holes"] = [[[r6(a), r6(b)] for a, b in h] for h in holes]
    return d


def ext_v(profile_uw, v0, v1):
    """🧱 3D: an extrusion of a (u, w) profile along v (plane zx takes (w, u))."""
    return {"kind": "extrude", "plane": "zx", "profile": [[r6(w), r6(u)] for u, w in profile_uw], "from": r6(v0), "to": r6(v1)}


def ext_u(profile_vw, u0, u1, holes=None):
    """🧱 3D: an extrusion of a (v, w) profile along u."""
    d = {"kind": "extrude", "plane": "yz", "profile": [[r6(a), r6(b)] for a, b in profile_vw], "from": r6(u0), "to": r6(u1)}
    if holes:
        d["holes"] = [[[r6(a), r6(b)] for a, b in h] for h in holes]
    return d


def rect(u0, u1, v0, v1):
    """⬛ A rectangle polygon."""
    return [(u0, v0), (u1, v0), (u1, v1), (u0, v1)]


def cyl_u(u0, u1, r, v=0.0, w=0.0):
    """🥫 3D: a cylinder along u."""
    return {"kind": "revolve", "profile": [[0, 0], [0, r6(r)], [r6(u1 - u0), r6(r)], [r6(u1 - u0), 0]], "axis": [1, 0, 0], "origin": [r6(u0), r6(v), r6(w)]}


def cyl_v(v0, v1, r, u=0.0, w=0.0):
    """🥫 3D: a cylinder along v (up for the stool)."""
    return {"kind": "revolve", "profile": [[0, 0], [0, r6(r)], [r6(v1 - v0), r6(r)], [r6(v1 - v0), 0]], "axis": [0, 1, 0], "origin": [r6(u), r6(v0), r6(w)]}


def capsule(p0, p1, r):
    """💊 3D: a capsule between two bone-local points."""
    return {"kind": "capsule", "p0": [r6(a) for a in p0], "p1": [r6(a) for a in p1], "r0": r6(r), "r1": r6(r)}


def box(c, size):
    """📦 3D: a box centred at `c`."""
    return {"kind": "box", "size": [r6(a) for a in size], "transform": {"translation": [r6(a) for a in c]}}


def sw(profile, w0, w1, wt=None, holes=None):
    """✏ 2D: a (u, v) profile drawn as a solid extruded along w (span in bone w, `wt` the span at the bone length)."""
    return ("w", profile, w0, w1, wt, holes)


def sv(profile_uw, v0, v1):
    """✏ 2D: a (u, w) profile drawn as a solid extruded along v."""
    return ("v", profile_uw, v0, v1)


def plain(e, f, p):
    """✏ 2D: plain shapes per view (elevation bone-local, front and plan in world rest coordinates) for parts on bones that never turn."""
    return ("plain", e, f, p)


BASS_L = 1.80


def string_span(sid, scale):
    """🎼 The two segments of one string: tailpiece to bridge and bridge to nut, each (u0, v0, w0, u1, v1, w1, r)."""
    _, k, r = [s for s in STRINGS if s[0] == sid][0]
    vb, vn, vt = 0.035 * k, 0.012 * k, 0.030 * k
    wb = bridge_w(vb) - r
    wn = w_ft(NUT_U) - 0.002 - r
    wt = TAIL_W[1] - 0.016 - r
    return (TAIL_U[1] - 0.010, vt, wt, BRIDGE_U, vb, wb, r), (BRIDGE_U, vb, wb, NUT_U, vn, wn, r)


def line_span(u0, w0, u1, w1, half):
    """📏 The w-span [z0, z1] at u = 0 and at the bone length of a part whose centre depth runs linearly from (u0, w0) to (u1, w1), `half` its half thickness."""
    b = (w1 - w0) / (u1 - u0)
    a = w0 - b * u0
    return (a - half, a + half), (a + b * BASS_L - half, a + b * BASS_L + half)


def strip(u0, v0, u1, v1, half):
    """▭ A thin closed strip polygon of half-width `half` along a segment in the (u, v) plane."""
    du, dv = u1 - u0, v1 - v0
    L = math.hypot(du, dv)
    nu, nv = -dv / L * half, du / L * half
    return [(u0 + nu, v0 + nv), (u1 + nu, v1 + nv), (u1 - nu, v1 - nv), (u0 - nu, v0 - nv)]


def fhole_parts(side, scale):
    """🎻 The f-hole of one side as eyes and slot at 1:50 (texture inlays flush with the arched top) or as one slot at 1:100."""
    up, lo = (0.590, side * 0.100), (0.365, side * 0.140)
    out = []
    name = "treble" if side > 0 else "bass"
    if scale == 50:
        for tag, (u, v), r in (("upper-eye", up, 0.0115), ("lower-eye", lo, 0.0125)):
            s = abs(v) / max(half_at(outline(50), u), 1e-3)
            w = w_arch(u, min(1, s))
            prof = circle(u, v, r, 12)
            out.append(Part(f"fhole-{name}-{tag}", "bass", "fhole-shadow", 0.06, "top", ext_w(prof, w - 0.0015, w + 0.003), sw(prof, w - 0.0015, w + 0.003), role="detail", extra={"texture": True}, thermal=False, detail=True))
        um, vm = (up[0] + lo[0]) / 2, (up[1] + lo[1]) / 2
        s = abs(vm) / half_at(outline(50), um)
        w = w_arch(um, s)
        prof = strip(lo[0] + 0.012, lo[1] - side * 0.002, up[0] - 0.010, up[1] + side * 0.002, 0.004)
        out.append(Part(f"fhole-{name}-slot", "bass", "fhole-shadow", 0.06, "top", ext_w(prof, w - 0.0015, w + 0.003), sw(prof, w - 0.0015, w + 0.003), role="detail", extra={"texture": True}, thermal=False, detail=True))
    else:
        prof = strip(lo[0], lo[1], up[0], up[1], 0.009 if scale == 100 else 0.0)
        out.append(Part(f"fhole-{name}", "bass", "fhole-shadow", 0.06, "top", None, sw(prof, -0.123, -0.121), role="detail", body=False, thermal=False, detail=True))
    return out


def bass_parts(scale):
    """🎻 The bass itself at one scale: body, top, neck, head, fingerboard, bridge, tailpiece, strings, endpin with its wheel clamp, and the traces of use."""
    P = []
    half = outline(scale)
    f = FEATURE[scale]
    loop = [(u, v) for u, v in half] + [(u, -v) for u, v in reversed(half)][:-1]
    if scale in (50, 100, 200):
        slab = symmetric_body(half, [(1.0, lambda u, s: w_back(u)), (1.0, lambda u, s: -RIB)]).solid()
        P.append(Part("body", "bass", "varnished-maple", 0.42, "back-ribs", slab, sw(loop, -RIB, RIB)))
    else:
        P.append(Part("body", "bass", "varnished-maple", 0.42, ["back-ribs", "top"], ext_w(loop, -0.112, RIB), sw(loop, -0.112, RIB)))
    if scale == 50:
        top = symmetric_body(half, [(1.0, lambda u, s: -RIB)] + [(s, lambda u, s: w_arch(u, s)) for s in (0.85, 0.6, 0.3)]).solid()
        top_w, top_x = -0.122, {}
    elif scale == 100:
        top = symmetric_body(half, [(1.0, lambda u, s: -RIB), (0.55, lambda u, s: w_arch(u, s))]).solid()
        top_w, top_x = -0.122, {}
    else:
        top_w, top_x = -0.112, {"sheet": True}
        top = ext_w(loop, top_w, -RIB)
    if scale != 500:
        P.append(Part("top", "bass", "varnished-spruce", 0.55, "top", top, sw(loop, top_w, -RIB), extra=top_x))
    fb_u0, fb_u1 = FB_END, NUT_U
    fbt = 0.020
    fb = hexahedron([(fb_u0, -fb_half(fb_u0), w_ft(fb_u0) + fbt), (fb_u1, -fb_half(fb_u1), w_ft(fb_u1) + fbt), (fb_u1, fb_half(fb_u1), w_ft(fb_u1) + fbt), (fb_u0, fb_half(fb_u0), w_ft(fb_u0) + fbt),
                     (fb_u0, -fb_half(fb_u0), w_ft(fb_u0)), (fb_u1, -fb_half(fb_u1), w_ft(fb_u1)), (fb_u1, fb_half(fb_u1), w_ft(fb_u1)), (fb_u0, fb_half(fb_u0), w_ft(fb_u0))])
    fb_prof = [(fb_u0, -fb_half(fb_u0)), (fb_u1, -fb_half(fb_u1)), (fb_u1, fb_half(fb_u1)), (fb_u0, fb_half(fb_u0))]
    (z0, z1), zt = line_span(fb_u0, w_ft(fb_u0) + fbt / 2, fb_u1, w_ft(fb_u1) + fbt / 2, fbt / 2)
    if scale in (50, 100):
        P.append(Part("fingerboard", "bass", "ebony", 0.12, "ebony", fb.solid(), sw(fb_prof, z0, z1, zt)))
    if scale == 50:
        P.append(Part("nut", "bass", "ebony", 0.10, "ebony", box((NUT_U + 0.004, 0, w_ft(NUT_U) + 0.0035), (0.008, 0.050, 0.011)), sw(rect(NUT_U, NUT_U + 0.008, -0.025, 0.025), w_ft(NUT_U) - 0.002, w_ft(NUT_U) + 0.009)))
    if scale == 50:
        neck = [(1.085, 0.045, -0.180, 0.040), (1.10, 0.044, -0.186, 0.040), (1.13, 0.042, -0.183, -0.010), (1.17, 0.040, -0.179, -0.060), (1.22, 0.037, -0.174, -0.090), (1.35, 0.033, -0.161, -0.098), (1.53, 0.027, -0.143, -0.092)]
    elif scale == 100:
        neck = [(1.085, 0.045, -0.182, 0.040), (1.13, 0.042, -0.183, -0.010), (1.22, 0.037, -0.174, -0.090), (1.53, 0.027, -0.143, -0.092)]
    if scale in (50, 100):
        secs = [{"at": [r6(u), 0, r6((a + b) / 2)], "rx": r6(rx), "ry": r6((b - a) / 2), "exponent": 2.5, "rotation": [0.5, 0.5, 0.5, 0.5]} for u, rx, a, b in neck]
        heel = [n for n in neck if n[0] <= 1.17] if scale == 50 else neck[:2]
        shaft = [n for n in neck if n[0] >= heel[-1][0]]
        hp = [(u, -rx) for u, rx, _, _ in heel] + [(u, rx) for u, rx, _, _ in reversed(heel)]
        sp = [(u, -rx) for u, rx, _, _ in shaft] + [(u, rx) for u, rx, _, _ in reversed(shaft)]
        a0, b0, a1, b1 = shaft[0][2], shaft[0][3], shaft[-1][2], shaft[-1][3]
        (z0, z1), zt = line_span(shaft[0][0], (a0 + b0) / 2, shaft[-1][0], (a1 + b1) / 2, max(b0 - a0, b1 - a1) / 2)
        P.append(Part("neck", "bass", "worn-maple", 0.46, "neck-scroll", {"kind": "loft", "sections": secs, "capStart": True, "capEnd": True}, sw(sp, z0, z1, zt)))
        P.append(Part("neck-heel", "bass", "worn-maple", 0.46, "neck-scroll", None, sw(hp, min(a for _, _, a, _ in heel), max(b for _, _, _, b in heel)), body=False))
    else:
        top0, top1 = w_ft(1.10), w_ft(NUT_U)
        hw = 0.040 if scale == 200 else 0.0375
        dep = 0.075 if scale == 200 else 0.080
        heel = [(1.06, -hw, top0), (NUT_U, -hw, top1), (NUT_U, hw, top1), (1.06, hw, top0)]
        mesh = hexahedron([(a, b, c + dep) for a, b, c in heel] + heel)
        (z0, z1), zt = line_span(1.06, top0 + dep / 2, NUT_U, top1 + dep / 2, dep / 2)
        P.append(Part("neck", "bass", "worn-maple", 0.40, ["neck-scroll", "ebony"], mesh.solid(), sw(rect(1.06, NUT_U, -hw, hw), z0, z1, zt)))
    if scale == 50:
        cheek = rect(NUT_U, 1.715, -0.168, -0.092)
        for side, name in ((1, "treble"), (-1, "bass")):
            P.append(Part(f"pegbox-cheek-{name}", "bass", "worn-maple", 0.44, "neck-scroll", ext_v(cheek, *sorted((side * 0.019, side * 0.031))), sv(cheek, *sorted((side * 0.019, side * 0.031)))))
        back = rect(NUT_U, 1.715, -0.104, -0.092)
        P.append(Part("pegbox-back", "bass", "worn-maple", 0.44, "neck-scroll", ext_v(back, -0.019, 0.019), sv(back, -0.019, 0.019)))
        floor = rect(NUT_U, NUT_U + 0.016, -0.168, -0.104)
        P.append(Part("pegbox-floor", "bass", "worn-maple", 0.44, "neck-scroll", ext_v(floor, -0.019, 0.019), sv(floor, -0.019, 0.019)))
    elif scale == 100:
        head = rect(NUT_U, 1.715, -0.168, -0.092)
        P.append(Part("pegbox", "bass", "worn-maple", 0.44, "neck-scroll", ext_v(head, -0.031, 0.031), sv(head, -0.031, 0.031)))
    elif scale == 200:
        head = rect(NUT_U, 1.715, -0.170, -0.075)
        P.append(Part("head", "bass", "worn-maple", 0.42, ["neck-scroll", "fittings"], ext_v(head, -0.055, 0.055), sv(head, -0.055, 0.055)))
    if scale in (50, 100, 200):
        sc = circle(1.752, -0.128, 0.048, {50: 20, 100: 14, 200: 10}[scale])
        P.append(Part("scroll", "bass", "worn-maple", 0.40, "neck-scroll", ext_v(sc, -0.033, 0.033), sv(sc, -0.033, 0.033)))
    else:
        head = [(NUT_U, -0.170), (1.71, -0.172), (1.80, -0.14), (1.80, -0.095), (1.71, -0.080), (NUT_U, -0.090)]
        P.append(Part("head", "bass", "worn-maple", 0.40, ["neck-scroll", "fittings"], ext_v(head, -0.0375, 0.0375), sv(head, -0.0375, 0.0375)))
    if scale == 50:
        for side, name in ((1, "treble"), (-1, "bass")):
            blk = rect(1.555, 1.680, *sorted((side * 0.031, side * 0.047)))
            P.append(Part(f"tuner-plate-{name}", "bass", "brass", 0.62, "fittings", ext_w(blk, -0.158, -0.112), sw(blk, -0.158, -0.112)))
            for k, u in enumerate((1.585, 1.650)):
                key = rect(u - 0.006, u + 0.006, *sorted((side * 0.034, side * 0.056)))
                P.append(Part(f"tuner-key-{name}-{k}", "bass", "brass", 0.66, "fittings", ext_w(key, -0.112, -0.065), sw(key, -0.112, -0.065)))
    elif scale == 100:
        for side, name in ((1, "treble"), (-1, "bass")):
            blk = rect(1.555, 1.680, *sorted((side * 0.031, side * 0.056)))
            P.append(Part(f"tuners-{name}", "bass", "brass", 0.62, "fittings", ext_w(blk, -0.158, -0.065), sw(blk, -0.158, -0.065)))
    if scale in (50, 100):
        holes = [circle(0, -0.235, 0.011, 10), ellipse(0.040, -0.200, 0.014, 0.0055, 10), ellipse(-0.040, -0.200, 0.014, 0.0055, 10)] if scale == 50 else None
        br = [(-0.092, -0.1212), (-0.058, -0.1212), (-0.050, -0.140), (-0.030, -0.158), (0.0, -0.164), (0.030, -0.158), (0.050, -0.140), (0.058, -0.1212), (0.092, -0.1212), (0.090, -0.150), (0.078, -0.185), (0.072, -0.205),
              (0.080, -0.222), (0.076, -0.245), (0.068, -0.270), (0.040, -0.2795), (0.0, -0.2835), (-0.040, -0.2795), (-0.068, -0.270), (-0.076, -0.245), (-0.080, -0.222), (-0.072, -0.205), (-0.078, -0.185), (-0.090, -0.150)]
        if scale == 100:
            br = [(-0.092, -0.1212), (-0.058, -0.1212), (-0.030, -0.158), (0.030, -0.158), (0.058, -0.1212), (0.092, -0.1212), (0.078, -0.185), (0.080, -0.222), (0.068, -0.270), (0.0, -0.2835), (-0.068, -0.270), (-0.080, -0.222), (-0.078, -0.185)]
        bu0, bu1 = BRIDGE_U - 0.009, BRIDGE_U + 0.009
        P.append(Part("bridge", "bass", "bridge-maple", 0.78, "bridge", ext_u(br, bu0, bu1, holes), sw(rect(bu0, bu1, -0.092, 0.092), -0.2835, -0.1212)))
    elif scale == 200:
        br = [(-0.092, -0.112), (0.092, -0.112), (0.075, -0.2835), (-0.075, -0.2835)]
        P.append(Part("bridge", "bass", "bridge-maple", 0.78, ["bridge", "strings"], ext_u(br, BRIDGE_U - 0.015, BRIDGE_U + 0.015), sw(rect(BRIDGE_U - 0.015, BRIDGE_U + 0.015, -0.092, 0.092), -0.2835, -0.112)))
    else:
        ridge = [(0.30, -0.112), (BRIDGE_U - 0.06, -0.2835), (BRIDGE_U + 0.06, -0.2835), (1.06, -0.20), (1.06, -0.112)]
        P.append(Part("string-ridge", "bass", "ebony", 0.20, ["bridge", "ebony", "strings"], ext_v(ridge, -0.0375, 0.0375), sv(ridge, -0.0375, 0.0375)))
    if scale in (50, 100, 200):
        t0, t1 = TAIL_U
        tt = 0.016 if scale != 200 else 0.030
        hw0, hw1 = 0.028, 0.056
        wb0, wb1 = TAIL_W
        tp = hexahedron([(t0, -hw0, wb0), (t1, -hw1, wb1), (t1, hw1, wb1), (t0, hw0, wb0), (t0, -hw0, wb0 - tt), (t1, -hw1, wb1 - tt), (t1, hw1, wb1 - tt), (t0, hw0, wb0 - tt)])
        (z0, z1), zt = line_span(t0, wb0 - tt / 2, t1, wb1 - tt / 2, tt / 2)
        P.append(Part("tailpiece", "bass", "ebony", 0.14, "ebony", tp.solid(), sw([(t0, -hw0), (t1, -hw1), (t1, hw1), (t0, hw0)], z0, z1, zt)))
    if scale == 50:
        P.append(Part("saddle", "bass", "ebony", 0.12, "ebony", box((0.005, 0, -0.1095), (0.018, 0.080, 0.025)), sw(rect(-0.004, 0.014, -0.040, 0.040), -0.122, -0.097)))
        tw = capsule((0.040, 0, -0.124), (-0.012, 0, -0.020), 0.0028)
        P.append(Part("tail-wire", "bass", "steel", 0.30, "fittings", tw, sw(strip(0.040, 0, -0.012, 0, 0.0028), -0.124, -0.020), extra={"slender": True}))
    if scale in (50, 100):
        for sid, _, r in STRINGS:
            for seg, (u0, v0, w0, u1, v1, w1, rr) in zip(("tail", "main"), string_span(sid, scale)):
                (z0, z1), zt = line_span(u0, w0, u1, w1, rr)
                d2 = sw(strip(u0, v0, u1, v1, rr), z0, z1, zt) if scale == 50 else ("line", [(u0, v0), (u1, v1)], z0, z1, zt)
                P.append(Part(f"string-{sid}-{seg}", "bass", "string-steel", 0.20, "strings", capsule((u0, v0, w0), (u1, v1, w1), rr), d2, role="detail", extra={"slender": True}))
    if scale != 500:
        sock_u0 = -0.025 if scale in (50, 100) else -0.03
        P.append(Part("endpin-socket", "bass", "ebony", 0.16, "fittings", cyl_u(sock_u0, 0.002, 0.020), sw(rect(sock_u0, 0.002, -0.020, 0.020), -0.020, 0.020)))
    if scale == 50:
        P.append(Part("socket-screw", "bass", "steel", 0.30, "fittings", box((-0.013, 0.027, 0), (0.010, 0.016, 0.010)), sw(rect(-0.018, -0.008, 0.019, 0.035), -0.005, 0.005)))
    if scale in (50, 100):
        P.append(Part("wheel-clamp", "bass", "steel", 0.34, "fittings", box((-0.036, 0, 0), (0.028, 0.026, 0.048)), sw(rect(-0.050, -0.022, -0.013, 0.013), -0.024, 0.024)))
    elif scale == 200:
        P.append(Part("wheel-clamp", "bass", "steel", 0.34, "fittings", box((-0.045, 0, 0), (0.030, 0.030, 0.048)), sw(rect(-0.060, -0.030, -0.015, 0.015), -0.024, 0.024)))
    return P


def endpin_parts(scale):
    """📍 The steel endpin rod with its rubber stopper (sliding in the socket) and the twin wheels of the bass wheel on their axle."""
    P = []
    f = FEATURE[scale]
    if scale in (50, 100):
        P.append(Part("endpin-rod", "endpin", "steel", 0.30, "fittings", cyl_u(0.0, ROD_L, 0.005), sw(rect(0.0, ROD_L, -0.005, 0.005), -0.005, 0.005), extra={"slender": True}))
        P.append(Part("endpin-stopper", "endpin", "rubber", 0.06, "wheels", {"kind": "ellipsoid", "radii": [TIP_R, TIP_R, TIP_R], "transform": {"translation": [ROD_L + 0.0, 0, 0]}},
                      sw(circle(ROD_L, 0, TIP_R, 10), -TIP_R, TIP_R)))
        wr, wn = WHEEL_R, {50: 24, 100: 16}[scale]
        for side, name in ((1, "left"), (-1, "right")):
            lo, hi = sorted((side * 0.024, side * 0.046))
            prof = circle(0, 0, wr, wn)
            P.append(Part(f"wheel-{name}", "wheel", "rubber", 0.10, "wheels", ext_w(prof, lo, hi), sw(prof, lo, hi)))
    else:
        rl = max(0.0075, f / 2) if scale == 200 else 0.005
        P.append(Part("endpin-rod", "endpin", "steel", 0.30, "fittings", cyl_u(0.0, ROD_L + TIP_R, 0.005), sw(rect(0.0, ROD_L + TIP_R, -0.005, 0.005), -0.005, 0.005), extra={"slender": True}))
        wr = WHEEL_R if scale == 200 else 0.0375
        prof = circle(0, 0, wr, 10)
        wlo, whi = (-0.046, 0.046)
        P.append(Part("wheel-pair", "wheel", "rubber", 0.10, ["wheels", "fittings"], ext_w(prof, wlo, whi), sw(prof, wlo, whi)))
    return P


def cover_parts(scale):
    """👜 The padded cover: back panel, the two halves of the gusset band, the neck sleeve, the hinge tape, handles and straps, and the open front panel lying on the floor."""
    P = []
    f = FEATURE[scale]
    loop = bag_loop(scale)
    bo = BACK_OUT + (0.010 if scale == 500 else 0.0)
    P.append(Part("cover-back", "cover", "padded-nylon", 0.22, "cover", ext_w(loop, BACK_IN, bo), sw(loop, BACK_IN, bo), extra={"sheet": True} if bo - BACK_IN < f else {}))
    thick = {50: 0, 100: 0, 200: 0.008, 500: 0.052}[scale]
    for side, name in ((1, "upper"), (-1, "lower")):
        poly = band_half(scale, side, thick, 0.08 if scale == 500 else SLOT)
        if scale == 500:
            poly = [(u, v) for u, v in poly if 0.06 <= u <= 1.04]
        P.append(Part(f"cover-band-{name}", "cover", "padded-nylon", 0.26, "cover", ext_w(poly, BAND_RIM, BACK_IN), sw(poly, BAND_RIM, BACK_IN), views=("elevation", "front") if (scale == 500 and side < 0) else ("elevation", "front", "plan")))
    sl = SLEEVE if scale in (50, 100) else [SLEEVE[0], SLEEVE[2], SLEEVE[4], SLEEVE[6], SLEEVE[-1]]
    if scale == 500:
        sl = [(1.04, 0.15, 0.135), SLEEVE[3], SLEEVE[6], SLEEVE[-2]]
    secs = [{"at": [r6(u), 0, SLEEVE_WC], "rx": r6(rx), "ry": r6(ry), "exponent": 2.5, "rotation": [0.5, 0.5, 0.5, 0.5]} for u, rx, ry in sl]
    prof = [(u, -rx) for u, rx, _ in sl] + [(u, rx) for u, rx, _ in reversed(sl)]
    ry0 = max(ry for _, _, ry in sl)
    P.append(Part("cover-sleeve", "cover", "padded-nylon", 0.24, "cover", {"kind": "loft", "sections": secs, "capStart": True, "capEnd": True}, sw(prof, SLEEVE_WC - ry0, SLEEVE_WC + ry0)))
    (a1, b1), (a2, b2) = hull_contacts()
    th = max(0.012, f)
    for k, uc in enumerate((a1, a2)):
        p0 = world_to_cover((cover_to_world((uc, b1 if k == 0 else b2, 0))[0], 0, 0))
        du = 0.06 if scale != 500 else 0.08
        c0 = cover_to_world((uc - du, 0, 0))[0]
        c1 = cover_to_world((uc + du, 0, 0))[0]
        corners = [world_to_cover((c0, 0.0, 0)), world_to_cover((c1, 0.0, 0)), world_to_cover((c1, th, 0)), world_to_cover((c0, th, 0))]
        prof = [(p[0], p[1]) for p in corners]
        w0, w1 = -0.325, -0.325 + max(FLAP_T, f)
        P.append(Part(f"cover-hinge-{k}", "cover", "padded-nylon", 0.30, "cover", ext_w(prof, w0, w1), sw(prof, w0, w1)))
    if scale in (50, 100):
        hu0, hu1 = 0.62, 0.80
        g0, g1 = half_at(bag_half(scale), hu0), half_at(bag_half(scale), hu1)
        hump = [(hu0, g0 - 0.006), (hu0 + 0.03, g0 + 0.030), (hu1 - 0.03, g1 + 0.030), (hu1, g1 - 0.006), (hu1 - 0.045, g1 + 0.014), (hu0 + 0.045, g0 + 0.014)] if scale == 50 else \
            [(hu0, g0 - 0.006), (hu0 + 0.03, g0 + 0.030), (hu1 - 0.03, g1 + 0.030), (hu1, g1 - 0.006)]
        P.append(Part("cover-handle", "cover", "webbing", 0.18, "cover", ext_w(hump, -0.120, -0.080), sw(hump, -0.120, -0.080)))
        for side, name in ((1, "upper"), (-1, "lower")):
            st = rect(0.30, 0.95, *sorted((side * 0.10, side * 0.15)))
            P.append(Part(f"cover-strap-{name}", "cover", "webbing", 0.18, "cover", ext_w(st, BACK_OUT, BACK_OUT + 0.016), sw(st, BACK_OUT, BACK_OUT + 0.016), views=("front", "plan")))
        gu0, gu1 = 1.30, 1.44
        grip = [(gu0, 0.070), (gu0 + 0.025, 0.105), (gu1 - 0.025, 0.105), (gu1, 0.070), (gu1 - 0.035, 0.088), (gu0 + 0.035, 0.088)] if scale == 50 else [(gu0, 0.070), (gu0 + 0.025, 0.105), (gu1 - 0.025, 0.105), (gu1, 0.070)]
        P.append(Part("cover-grip", "cover", "webbing", 0.18, "cover", ext_w(grip, SLEEVE_WC - 0.020, SLEEVE_WC + 0.020), sw(grip, SLEEVE_WC - 0.020, SLEEVE_WC + 0.020)))
    return P


def flap_profile(scale):
    """🪟 The front panel outline in its own frame: along the hinge and away from it (= the height of the closed panel)."""
    pts = []
    for u, v in bag_loop(scale):
        x, y, _ = cover_to_world((u, v, 0))
        pts.append((x - H_WORLD[0], y))
    return pts


def flap_parts(scale):
    """🪟 The unzipped front panel of the cover, folded down onto the floor."""
    f = FEATURE[scale]
    prof = flap_profile(scale)
    t = 0.035 if scale == 500 else FLAP_T
    return [Part("cover-flap", "cover-flap", "padded-nylon", 0.34, "cover", ext_w(prof, 0.0, t), sw(prof, 0.0, t), extra={"sheet": True} if t < f else {})]


STOOL_LEG_AZ = (45, 135, 225, 315)


def stool_parts(scale):
    """🪑 The bass stool: four-star base with glides, telescopic column with clamp, footrest ring on struts, round padded seat; the star base becomes one low disc and the column thickens at 1:500."""
    P = []
    f = FEATURE[scale]
    X0 = STOOL_X
    rf, hf = 0.262, 0.022
    rh, hh = 0.030, 0.110

    def vc(pid, bone, mat, tone, region, y0, y1, r, extra=None, views=("elevation", "front", "plan")):
        oy = SEAT_Y if bone == "stool-seat" else 0.0
        e = {"kind": "rect", "x": r6(-r), "y": r6(y0 - oy), "w": r6(2 * r), "h": r6(y1 - y0)}
        fr = {"kind": "rect", "x": r6(-r), "y": r6(y0), "w": r6(2 * r), "h": r6(y1 - y0)}
        pl = {"kind": "ellipse", "cx": r6(X0), "cy": 0, "rx": r6(r), "ry": r6(r)}
        P.append(Part(pid, bone, mat, tone, region, cyl_v(y0 - oy, y1 - oy, r), plain(e, fr, pl), views=views, extra=extra))

    def tube(pid, bone, mat, tone, region, a, b, r, extra=None, views=("elevation", "front", "plan")):
        (ax, ay, az), (bx, by, bz) = a, b
        e = {"kind": "capsule", "x0": r6(ax), "y0": r6(ay), "x1": r6(bx), "y1": r6(by), "r0": r6(r), "r1": r6(r)}
        fr = {"kind": "capsule", "x0": r6(az), "y0": r6(ay), "x1": r6(bz), "y1": r6(by), "r0": r6(r), "r1": r6(r)}
        pl = {"kind": "capsule", "x0": r6(X0 + ax), "y0": r6(az), "x1": r6(X0 + bx), "y1": r6(bz), "r0": r6(r), "r1": r6(r)}
        P.append(Part(pid, bone, mat, tone, region, capsule(a, b, r), plain(e, fr, pl), views=views, extra=extra))

    if scale == 500:
        vc("stool-base", "stool", "chrome-steel", 0.62, "stool-frame", 0.0, 0.075, r6((rf + 0.016) * math.cos(math.radians(45))))
        vc("stool-column", "stool", "chrome-steel", 0.72, "stool-frame", 0.075, SEAT_Y - 0.015, 0.0375)
    else:
        legr = 0.011 if scale in (50, 100) else 0.015
        for k, az in enumerate(STOOL_LEG_AZ):
            ca, sa = math.cos(math.radians(az)), math.sin(math.radians(az))
            tube(f"stool-leg-{k}", "stool", "chrome-steel", 0.70, "stool-frame", (rh * ca, hh, rh * sa), (rf * ca, hf + 0.004, rf * sa), legr)
            gr, gh = (0.016, 0.016) if scale in (50, 100) else (0.016, 0.03)
            P.append(Part(f"stool-glide-{k}", "stool", "rubber", 0.08, "stool-frame", cyl_v(0.0, gh, gr, u=rf * ca, w=rf * sa),
                          plain({"kind": "rect", "x": r6(rf * ca - gr), "y": 0, "w": r6(2 * gr), "h": r6(gh)}, {"kind": "rect", "x": r6(rf * sa - gr), "y": 0, "w": r6(2 * gr), "h": r6(gh)},
                                {"kind": "ellipse", "cx": r6(X0 + rf * ca), "cy": r6(rf * sa), "rx": gr, "ry": gr})))
        vc("stool-hub", "stool", "chrome-steel", 0.66, "stool-frame", 0.08, 0.135, 0.035)
        vc("stool-column", "stool", "chrome-steel", 0.72, "stool-frame", 0.135, 0.620, 0.020)
        vc("stool-clamp", "stool", "chrome-steel", 0.60, "stool-frame", 0.595, 0.625, 0.026)
    if scale == 50:
        P.append(Part("stool-knob", "stool", "black-plastic", 0.10, "stool-frame", box((0.045, 0.61, 0), (0.040, 0.015, 0.015)),
                      plain({"kind": "rect", "x": 0.025, "y": 0.6025, "w": 0.040, "h": 0.015}, {"kind": "rect", "x": -0.0075, "y": 0.6025, "w": 0.015, "h": 0.015}, {"kind": "rect", "x": r6(X0 + 0.025), "y": -0.0075, "w": 0.040, "h": 0.015})))
    ringR, rr, ringy = 0.180, 0.007, 0.330
    nseg = {50: 12, 100: 10, 200: 8, 500: 8}[scale]
    tor = {"kind": "revolve", "profile": [[r6(rr * math.sin(2 * math.pi * k / nseg)), r6(ringR + rr * math.cos(2 * math.pi * k / nseg))] for k in range(nseg)] + [[0, r6(ringR + rr)]], "axis": [0, 1, 0], "origin": [0, ringy, 0]}
    ro, ri = ringR + rr, ringR - rr
    ring_plan = {"kind": "path", "d": f"M {r6(X0 - ro)} 0 A {r6(ro)} {r6(ro)} 0 1 0 {r6(X0 + ro)} 0 A {r6(ro)} {r6(ro)} 0 1 0 {r6(X0 - ro)} 0 Z M {r6(X0 - ri)} 0 A {r6(ri)} {r6(ri)} 0 1 1 {r6(X0 + ri)} 0 A {r6(ri)} {r6(ri)} 0 1 1 {r6(X0 - ri)} 0 Z"}
    bar = {"kind": "rect", "x": r6(-ro), "y": r6(ringy - rr), "w": r6(2 * ro), "h": r6(2 * rr)}
    if scale == 500:
        bar = {"kind": "poly", "closed": False, "open": True, "points": [[r6(-ringR), ringy], [r6(ringR), ringy]]}
        ring_plan = {"kind": "ellipse", "cx": r6(X0), "cy": 0, "rx": ringR, "ry": ringR, "open": True}
    P.append(Part("stool-footring", "stool", "chrome-steel", 0.68, "stool-frame", tor, plain(dict(bar), dict(bar), ring_plan), extra={"slender": True}))
    if scale != 500:
        vc("stool-ring-collar", "stool", "chrome-steel", 0.62, "stool-frame", 0.315, 0.345, 0.026)
    for k, az in enumerate(STOOL_LEG_AZ):
        ca, sa = math.cos(math.radians(az)), math.sin(math.radians(az))
        tube(f"stool-strut-{k}", "stool", "chrome-steel", 0.66, "stool-frame", (0.018 * ca, ringy, 0.018 * sa), (0.184 * ca, ringy, 0.184 * sa), 0.004, extra={"slender": True})
        if scale == 500:
            a0, b0 = (0.018 * ca, 0.184 * ca), (0.018 * sa, 0.184 * sa)
            P[-1].d2 = plain({"kind": "poly", "closed": False, "open": True, "points": [[r6(a0[0]), ringy], [r6(a0[1]), ringy]]}, {"kind": "poly", "closed": False, "open": True, "points": [[r6(b0[0]), ringy], [r6(b0[1]), ringy]]},
                             {"kind": "poly", "closed": False, "open": True, "points": [[r6(X0 + a0[0]), r6(b0[0])], [r6(X0 + a0[1]), r6(b0[1])]]})
    if scale != 500:
        vc("stool-inner-column", "stool-seat", "chrome-steel", 0.76, "stool-frame", 0.600, SEAT_Y, 0.016)
    if scale == 50:
        vc("stool-seat-bracket", "stool-seat", "chrome-steel", 0.56, "stool-frame", SEAT_Y - 0.017, SEAT_Y, 0.040)
        vc("stool-seat-board", "stool-seat", "plywood", 0.50, "stool-seat", SEAT_Y, SEAT_Y + 0.018, 0.170)
        prof = [[0, 0], [0, 0.172], [0.006, 0.175], [0.030, 0.175], [0.040, 0.168], [0.042, 0.150], [0.042, 0]]
        side = [[-0.175, 0.018], [0.175, 0.018], [0.175, 0.048], [0.168, 0.058], [0.15, 0.060], [-0.15, 0.060], [-0.168, 0.058], [-0.175, 0.048]]
        P.append(Part("stool-seat-pad", "stool-seat", "worn-vinyl", 0.14, "stool-seat", {"kind": "revolve", "profile": prof, "axis": [0, 1, 0], "origin": [0, 0.018, 0]},
                      plain({"kind": "poly", "closed": True, "points": side}, {"kind": "poly", "closed": True, "points": [[a, r6(b + SEAT_Y)] for a, b in side]},
                            {"kind": "ellipse", "cx": r6(X0), "cy": 0, "rx": 0.175, "ry": 0.175})))
    elif scale == 500:
        vc("stool-seat-pad", "stool-seat", "worn-vinyl", 0.14, ["stool-seat", "stool-frame"], SEAT_Y - 0.015, SEAT_Y + 0.060, 0.175)
    else:
        vc("stool-seat-pad", "stool-seat", "worn-vinyl", 0.14, "stool-seat", SEAT_Y, SEAT_Y + 0.060, 0.175)
    return P


def details(scale):
    """🖋 Detail and texture strokes of one scale: purfling, rosin dust, worn varnish, the repaired crack, fingerboard tape, the scroll spiral, zip lines, scuffs and the taped repair of the cover."""
    out = {"elevation": [], "front": [], "plan": []}

    def add(views, sid, bone, geo, role, region, tone, z, filled=False, solid=None, material=None):
        for v in views:
            s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "region": region}
            if material:
                s["material"] = material
            s.update(geo)
            if solid:
                s["solid"] = solid
            out[v].append(s)

    def poly(pts, closed=True):
        return {"kind": "poly", "closed": closed, "points": [[r6(a), r6(b)] for a, b in pts]}

    def wsol(w0, w1, wt=None):
        d = {"kind": "extruded", "sweep": "w", "z0": r6(w0), "z1": r6(w1)}
        if wt:
            d["zTip"] = [r6(wt[0]), r6(wt[1])]
        return d

    if scale in (50, 100):
        half = outline(scale)
        ins = 0.008
        pur = [(max(u, ins), max(0.0, v - ins)) for u, v in half if v > ins and u < BODY_L - ins]
        pur = pur + [(u, -v) for u, v in reversed(pur)]
        add(("elevation",), "purfling", "bass", poly(pur), "detail", "top", 0.3, 7.9, solid=wsol(-0.1225, -0.1215))
        rosin = [(0.505, -0.060), (0.615, -0.050), (0.62, 0.045), (0.505, 0.062)]
        add(("elevation",), "rosin-dust", "bass", poly(rosin), "texture" if scale == 50 else "detail", "top", 0.88, 7.95, filled=True, solid=wsol(-0.1226, -0.1218))
        wear = [(0.80, 0.215), (0.86, 0.226), (0.93, 0.218), (0.95, 0.238), (0.88, 0.252), (0.80, 0.250)]
        add(("elevation",), "worn-varnish-upper-bout", "bass", poly(wear), "texture" if scale == 50 else "detail", "top", 0.70, 7.95, filled=True, solid=wsol(-0.1226, -0.1218))
    if scale == 50:
        add(("elevation",), "crack-repair", "bass", poly([(0.10, -0.150), (0.31, -0.165)], closed=False), "detail", "top", 0.2, 7.96, solid=wsol(-0.1226, -0.1218))
        add(("elevation",), "edge-chip", "bass", poly([(0.215, 0.330), (0.245, 0.333), (0.240, 0.339), (0.218, 0.337)]), "texture", "top", 0.78, 7.96, filled=True, solid=wsol(-0.1226, -0.1218))
        for k, u in enumerate((1.005, 1.13)):
            add(("elevation",), f"fingerboard-tape-{k}", "bass", poly([(u, -fb_half(u)), (u, fb_half(u))], closed=False), "texture", "ebony", 0.9, 8.6, solid=wsol(w_ft(u) - 0.001, w_ft(u)))
        add(("elevation",), "string-wear-groove", "bass", poly([(0.70, -0.035), (0.70, 0.035)], closed=False), "texture", "ebony", 0.35, 8.6, solid=wsol(w_ft(0.70) - 0.001, w_ft(0.70)))
        add(("elevation", "front", "plan"), "wheel-hub-left", "wheel", poly(circle(0, 0, 0.012, 12)), "detail", "wheels", 0.5, 9.0, solid=wsol(0.0455, 0.0465))
        bag = bag_half(50)
        add(("elevation",), "cover-tape-repair", "cover", poly([(-0.020, -0.05), (0.030, -0.05), (0.030, -0.12), (-0.012, -0.12)]), "texture", "cover", 0.62, 6.9, filled=True, solid=wsol(BAND_RIM - 0.001, BAND_RIM))
        for k, (u, side) in enumerate(((0.02, 1), (1.00, 1), (0.30, -1))):
            vv = side * (half_at(bag, u) - 0.012)
            add(("elevation",), f"cover-scuff-{k}", "cover", poly(circle(u, vv, 0.010, 8)), "texture", "cover", 0.55, 6.8, filled=True, solid=wsol(BAND_RIM - 0.001, BAND_RIM))
        fprof = flap_profile(50)
        cx = sum(p[0] for p in fprof) / len(fprof)
        cy = sum(p[1] for p in fprof) / len(fprof)
        zipl = [(cx + (x - cx) * 0.95, cy + (y - cy) * 0.93) for x, y in fprof[::2]]
        add(("plan",), "flap-zip-line", "cover-flap", poly(zipl), "detail", "cover", 0.3, 5.0, solid=wsol(-0.325 + FLAP_T, -0.325 + FLAP_T + 0.001))
        add(("plan",), "flap-faded-patch", "cover-flap", poly(ellipse(cx + 0.15, cy + 0.08, 0.12, 0.07, 10)), "texture", "cover", 0.5, 5.01, filled=True, solid=wsol(-0.325 + FLAP_T, -0.325 + FLAP_T + 0.001))
        add(("plan",), "stool-seat-tear", "stool-seat", poly([(STOOL_X - 0.06, 0.03), (STOOL_X + 0.05, -0.02), (STOOL_X + 0.06, 0.01), (STOOL_X - 0.05, 0.05)]), "texture", "stool-seat", 0.6, 9.0, filled=True)
        add(("elevation",), "stool-column-wear", "stool", poly([(-0.020, 0.40), (0.020, 0.40)], closed=False), "texture", "stool-frame", 0.4, 6.0)
    if scale in (50, 100):
        add(("plan",), "cover-zip-plan", "cover", poly([(0.05, 0.0), (1.05, 0.0)], closed=False), "detail", "cover", 0.3, 9.5, solid=wsol(BAND_RIM, BAND_RIM + 0.002))
    return out


OUTLINED_500 = {"cover-flap", "cover-back", "cover-band-upper", "cover-band-lower", "cover-hinge-0", "cover-hinge-1", "stool-seat-pad", "string-ridge"}


def all_parts(scale):
    """🗂 The complete part list of one scale."""
    P = bass_parts(scale) + endpin_parts(scale) + cover_parts(scale) + flap_parts(scale) + stool_parts(scale)
    if scale == 500:
        for p in P:
            if p.id in OUTLINED_500:
                p.extra["outline2d"] = True
    if scale in (50, 100):
        P += fhole_parts(1, scale) + fhole_parts(-1, scale)
    return P


REST_FRAMES = None


def solid_points(d):
    """📍 Sample points of a solid in its bone frame (for bounds and draw depths)."""
    k = d["kind"]
    tr = d.get("transform", {}).get("translation", [0, 0, 0])
    if k == "polyhedron":
        return [tuple(p) for p in d["vertices"]]
    if k == "box":
        sx, sy, sz = d["size"]
        return [(tr[0] + a * sx / 2, tr[1] + b * sy / 2, tr[2] + c * sz / 2) for a in (-1, 1) for b in (-1, 1) for c in (-1, 1)]
    if k == "ellipsoid":
        rx, ry, rz = d["radii"]
        return [(tr[0] + a * rx, tr[1] + b * ry, tr[2] + c * rz) for a in (-1, 1) for b in (-1, 1) for c in (-1, 1)]
    if k == "capsule":
        r = d["r0"]
        return [(p[0] + a * r, p[1] + b * r, p[2] + c * r) for p in (d["p0"], d["p1"]) for a in (-1, 1) for b in (-1, 1) for c in (-1, 1)]
    if k == "extrude":
        out = []
        for a, b in d["profile"]:
            for t in (d["from"], d["to"]):
                out.append({"xy": (a, b, t), "yz": (t, a, b), "zx": (b, t, a)}[d["plane"]])
        return out
    if k == "revolve":
        ax, o = d["axis"], d["origin"]
        out = []
        for a, r in d["profile"]:
            for th in range(0, 360, 30):
                c, s = r * math.cos(math.radians(th)), r * math.sin(math.radians(th))
                if ax == [1, 0, 0]:
                    out.append((o[0] + a, o[1] + c, o[2] + s))
                else:
                    out.append((o[0] + c, o[1] + a, o[2] + s))
        return out
    if k == "loft":
        out = []
        for sec in d["sections"]:
            u, v, w = sec["at"]
            for th in range(0, 360, 30):
                out.append((u, v + sec["rx"] * math.cos(math.radians(th)), w + sec["ry"] * math.sin(math.radians(th))))
        return out
    raise ValueError(k)


def part_points(p):
    """📍 Sample points of a part in its bone frame, from its 3D solid or its 2D profile."""
    if p.d3 is not None:
        return solid_points(p.d3)
    kind = p.d2[0]
    if kind == "w":
        return [(a, b, c) for a, b in p.d2[1] for c in (p.d2[2], p.d2[3])]
    return []


def world_bounds(p):
    """📦 World bounds of a part at rest."""
    pts = [to_world(REST_FRAMES, p.bone, q) for q in part_points(p)]
    if not pts:
        return (0, 0, 0), (0, 0, 0)
    return tuple(min(q[i] for q in pts) for i in range(3)), tuple(max(q[i] for q in pts) for i in range(3))


def shapes_of(p, rank):
    """✏ The 2D shapes of a part in every view it is drawn in."""
    lo, hi = world_bounds(p)
    depth = {"elevation": 5 - lo[2] + rank * 1e-4, "front": 5 + hi[0] + rank * 1e-4, "plan": 5 + hi[1] + rank * 1e-4}
    out = {}
    for view in p.views:
        role = "texture" if p.role == "texture-body" else p.role
        s = {"id": p.id, "bone": p.bone, "role": role, "z": round(depth[view], 4), "filled": True, "tone": p.tone, "material": p.material}
        if isinstance(p.region, list):
            s["regions"] = p.region
        else:
            s["region"] = p.region
        kind = p.d2[0]
        if kind == "plain":
            geo = {"elevation": p.d2[1], "front": p.d2[2], "plan": p.d2[3]}[view]
            s.update(geo)
            if geo.get("open"):
                s["filled"] = False
                del s["open"]
        elif kind == "line":
            _, prof, w0, w1, wt = p.d2
            s.update({"kind": "poly", "closed": False, "filled": False, "points": [[r6(a), r6(b)] for a, b in prof]})
            sol = {"kind": "extruded", "sweep": "w", "z0": r6(w0), "z1": r6(w1)}
            if wt:
                sol["zTip"] = [r6(wt[0]), r6(wt[1])]
            s["solid"] = sol
        elif kind == "w":
            _, prof, w0, w1, wt, holes = p.d2
            oz = REST_FRAMES[p.bone][0][2]
            if holes:
                s["kind"] = "path"
                s["d"] = " ".join("M " + " L ".join(f"{r6(a)} {r6(b)}" for a, b in ring) + " Z" for ring in [prof] + holes)
            else:
                s.update({"kind": "poly", "closed": True, "points": [[r6(a), r6(b)] for a, b in prof]})
            sol = {"kind": "extruded", "sweep": "w", "z0": r6(oz + w0), "z1": r6(oz + w1)}
            if wt:
                sol["zTip"] = [r6(oz + wt[0]), r6(oz + wt[1])]
            s["solid"] = sol
        else:
            _, prof, v0, v1 = p.d2
            s.update({"kind": "poly", "closed": True, "points": [[r6(a), r6(b)] for a, b in prof]})
            s["solid"] = {"kind": "extruded", "sweep": "v", "z0": r6(v0), "z1": r6(v1)}
        if p.extra.get("outline2d"):
            s["filled"] = False
        out[view] = s
    return out


exec(open(os.path.join(HERE, "author-double-bass-texts.py"), encoding="utf8").read())


def representation(scale):
    """🖨 The authored representation of one scale: part shapes, detail strokes, pens, hatches and decisions."""
    geo = {"elevation": [], "front": [], "plan": []}
    for rank, part in enumerate(all_parts(scale)):
        for view, s in shapes_of(part, rank).items():
            geo[view].append(s)
    for view, lst in details(scale).items():
        geo[view] += lst
    pens = {50: {"body": 0.35, "detail": 0.18, "texture": 0.10, "glazing": None, "ground": 0.50}, 100: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.35},
            200: {"body": 0.18, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.25}, 500: {"body": 0.13, "detail": None, "texture": None, "glazing": None, "ground": 0.18}}[scale]
    for view in geo:
        geo[view] = [s for s in geo[view] if pens[s["role"]] is not None]
    rng = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}[scale]
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": rng[0], "maxDenominator": rng[1]}, "pens": pens, "hatches": [], "geometry": geo, "decisions": DECISIONS_2D[scale]}


def thermal_list(scale, parts):
    """🌡 The thermal list of one scale: every non-texture body solid by reference, and the strings as slender thermal solids where the body list has none."""
    out = []
    if scale == 500:
        loop = [(u, v) for u, v in outline(500)] + [(u, -v) for u, v in reversed(outline(500))][:-1]
        out.append(dict(ext_w(loop, -0.112, -RIB), id="th-top", bone="bass", material="varnished-spruce", role="body", tone=0.55, sheet=True, region="top"))
    for p in parts:
        if not p.body or not p.thermal or p.d3 is None:
            continue
        region = p.region if isinstance(p.region, str) else p.region[0]
        out.append({"id": f"th-{p.id}", "of": p.id, "region": region})
    if scale in (200, 500):
        for sid, _, r in STRINGS:
            _, (u0, v0, w0, u1, v1, w1, rr) = string_span(sid, scale)
            out.append(dict(capsule((u0, v0, w0), (u1, v1, w1), rr), id=f"th-string-{sid}", bone="bass", material="string-steel", role="detail", tone=0.2, slender=True, region="strings"))
    if scale in (200, 500):
        out.append(dict(box((0.20, 0, -0.18), (0.075, 0.075, 0.075)) if scale == 500 else box((0.20, 0, -0.14), (0.30, 0.10, 0.03)), id="th-tailpiece-ebony", bone="bass", material="ebony", role="body", tone=0.14, region="ebony"))
    return out


def mass_ball(pid, bone, c, mat, scale, member, fit=0.03):
    """⚖ A centre-of-mass marker, at most `fit` in radius where the part is thinner (never under the feature)."""
    r = max(fit, FEATURE[scale])
    return dict(box(c, (2 * r, 2 * r, 2 * r)) if False else {"kind": "ellipsoid", "radii": [r, r, r], "transform": {"translation": [r6(a) for a in c]}}, id=pid, bone=bone, material=mat, role="detail", tone=0.5, member=member, part="mass")


def structure_list(scale, parts):
    """🏗 The structural idealisation of one scale: the bass body as a box beam with the neck, the bridge as a post on it, the endpin rod, the cover panels, the stool frame, their joints and centres of mass."""
    f = FEATURE[scale]
    names = {p.id for p in parts}
    out = []
    bw = max(f, 0.02)
    out.append(dict(box((0.555, 0, -0.016), (1.05, 0.20, 0.21)), id="member-bass-body", bone="bass", material="varnished-maple", role="body", tone=0.4, member="bass", part="member"))
    nw = max(0.05, f)
    out.append(dict(box((1.315, 0, -0.135), (0.43, nw, max(0.065, f))), id="member-bass-neck", bone="bass", material="worn-maple", role="body", tone=0.4, member="bass", part="member"))
    out.append(dict(box((1.09, 0, -0.10), (max(0.05, f), max(0.06, f), max(0.08, f))), id="joint-neck-block", bone="bass", material="worn-maple", role="detail", tone=0.5, member="bass", part="joint"))
    if "bridge" in names:
        out.append({"id": "member-bass-bridge", "of": "bridge", "member": "bass", "part": "member"})
    out.append({"id": "member-endpin", "of": "endpin-rod", "member": "endpin", "part": "member"})
    out.append(dict(box((-0.012, 0, 0), (max(0.03, f), max(0.04, f), max(0.04, f))), id="joint-endpin-socket", bone="bass", material="steel", role="detail", tone=0.5, member="endpin", part="joint"))
    out.append(dict(box((AXLE_U, 0, 0), (max(0.02, f), max(0.02, f), max(0.092, f))), id="joint-wheel-axle", bone="bass", material="steel", role="detail", tone=0.5, member="wheel", part="joint"))
    out.append({"id": "member-cover-back", "of": "cover-back", "member": "cover", "part": "member"})
    out.append({"id": "member-cover-band-lower", "of": "cover-band-lower", "member": "cover", "part": "member"})
    out.append({"id": "member-cover-band-upper", "of": "cover-band-upper", "member": "cover", "part": "member"})
    out.append({"id": "member-cover-flap", "of": "cover-flap", "member": "cover-flap", "part": "member"})
    out.append({"id": "joint-cover-hinge-0", "of": "cover-hinge-0", "member": "cover-flap", "part": "joint"})
    for k in range(4):
        if f"stool-leg-{k}" in names:
            out.append({"id": f"member-stool-leg-{k}", "of": f"stool-leg-{k}", "member": "stool", "part": "member"})
    if "stool-base" in names:
        out.append({"id": "member-stool-base", "of": "stool-base", "member": "stool", "part": "member"})
    out.append({"id": "member-stool-column", "of": "stool-column", "member": "stool", "part": "member"})
    if "stool-hub" in names:
        out.append({"id": "joint-stool-hub", "of": "stool-hub", "member": "stool", "part": "joint"})
    if "stool-inner-column" in names:
        out.append({"id": "member-stool-inner-column", "of": "stool-inner-column", "member": "stool-seat", "part": "member"})
    out.append({"id": "member-stool-seat", "of": "stool-seat-board" if "stool-seat-board" in names else "stool-seat-pad", "member": "stool-seat", "part": "member"})
    out.append(mass_ball("mass-bass", "bass", (MASS["bass"][1], MASS["bass"][2], -0.03), "varnished-maple", scale, "bass"))
    out.append(mass_ball("mass-cover", "cover", (0.55, 0.0, -0.05), "padded-nylon", scale, "cover"))
    out.append(mass_ball("mass-stool", "stool", (0.0, 0.25, 0.0), "chrome-steel", scale, "stool", 0.019))
    out.append(mass_ball("mass-stool-seat", "stool-seat", (0.0, 0.03, 0.0), "plywood", scale, "stool-seat"))
    fp = flap_profile(50)
    out.append(mass_ball("mass-cover-flap", "cover-flap", (sum(p[0] for p in fp) / len(fp), sum(p[1] for p in fp) / len(fp), FLAP_T / 2), "padded-nylon", scale, "cover-flap", FLAP_T / 2))
    return out


def volume(scale):
    """🧊 The authored volume of one scale."""
    parts = all_parts(scale)
    body = [p.solid() for p in parts if p.body and p.d3 is not None]
    return {"status": "authored", "denominator": scale, "body": body, "thermal": thermal_list(scale, parts), "structure": structure_list(scale, parts), "decisions": DECISIONS_3D[scale]}


def rig_bones():
    """🦴 The rig: floor, stool and seat, the tilting cover with its hinged front panel, and the bass on its slide, lift and tilt helpers with endpin and wheel."""
    qx, qy = Q_PIVOT
    bx, by, _ = B_WORLD
    hx, hy, hz = world_to_cover(H_WORLD)
    return [
        {"id": "floor", "parent": None, "title": T("Depot floor frame (massless, fixed root)", "Lagerboden-Bezug (masselos, feste Wurzel)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0.02, "joint": {"type": "fixed"}},
        {"id": "stool", "parent": "floor", "title": T("Bass stool base: four-star foot, column, clamp and footrest ring (stays where it stands)", "Kontrabasshocker-Gestell: Vierstern-Fuß, Säule, Klemme und Fußring (bleibt stehen)"),
         "x": STOOL_X, "y": 0, "angleDeg": 0, "lengthM": 0.262, "joint": {"type": "fixed"}, "mass": MASS_ENTRY["stool"], "section": SECTIONS["stool"]},
        {"id": "stool-seat", "parent": "stool", "title": T("Round padded seat on the telescopic inner column (height adjustable)", "Runder gepolsterter Sitz auf der Teleskop-Innensäule (höhenverstellbar)"),
         "x": 0, "y": SEAT_Y, "angleDeg": 0, "lengthM": 0.175, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -0.06, "maxM": 0.05}, "mass": MASS_ENTRY["stool-seat"]},
        {"id": "cover-tilt", "parent": "floor", "title": T("Tilt of the cover onto the bass wheel (pivot of the roll-up)", "Kippen der Hülle auf das Kontrabassrad (Drehpunkt des Aufrichtens)"),
         "x": r6(qx), "y": r6(qy), "angleDeg": 0, "lengthM": 0.02, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 85}},
        {"id": "cover", "parent": "cover-tilt", "title": T("Padded cover: back panel, gusset band and neck sleeve", "Gepolsterte Hülle: Rückenteil, Seitenband und Halsschlauch"),
         "x": r6(bx - qx), "y": r6(by - qy), "angleDeg": r6(-DELTA), "lengthM": 1.87, "joint": {"type": "fixed"}, "mass": MASS_ENTRY["cover"]},
        {"id": "cover-flap", "parent": "cover", "title": T("Unzipped front panel of the cover, hinged along the floor edge of the band", "Aufgezippte Vorderseite der Hülle, an der Bodenkante des Seitenbands angelenkt"),
         "x": r6(hx), "y": r6(hy), "z": r6(hz), "angleDeg": r6(DELTA), "twistDeg": 90, "lengthM": 1.13, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 0, "twistMinDeg": -92, "twistMaxDeg": 0}, "mass": MASS_ENTRY["cover-flap"]},
        {"id": "bass-slide", "parent": "floor", "title": T("Horizontal travel of the bass when it is taken out to be played", "Waagrechter Weg des Basses, wenn er zum Spielen herausgenommen wird"),
         "x": r6(qx), "y": r6(qy), "angleDeg": 0, "lengthM": 0.02, "joint": {"type": "prismatic", "axisDeg": 0, "minM": -3.0, "maxM": 3.0}},
        {"id": "bass-lift", "parent": "bass-slide", "title": T("Vertical travel of the bass", "Senkrechter Weg des Basses"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0.02, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -3.0, "maxM": 3.0}},
        {"id": "bass-tilt", "parent": "bass-lift", "title": T("Tilt of the bass from lying to rolled or upright", "Kippen des Basses vom Liegen zum Rollen oder Aufrechtstehen"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0.02,
         "joint": {"type": "revolute", "minDeg": -5, "maxDeg": 130}},
        {"id": "bass", "parent": "bass-tilt", "title": T("Double bass: body, neck, head, fingerboard, bridge, tailpiece and strings (turns about its own axis to face forward when played)", "Kontrabass: Korpus, Hals, Kopf, Griffbrett, Steg, Saitenhalter und Saiten (dreht sich beim Spielen um die eigene Achse nach vorn)"),
         "x": r6(bx - qx), "y": r6(by - qy), "angleDeg": r6(-DELTA), "lengthM": BASS_L, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 0, "twistMinDeg": 0, "twistMaxDeg": 95}, "mass": MASS_ENTRY["bass"], "section": SECTIONS["bass"]},
        {"id": "endpin", "parent": "bass", "title": T("Steel endpin with rubber stopper, sliding out of the socket", "Stahlstachel mit Gummistopper, aus der Buchse ausziehbar"),
         "x": ENDPIN_IN, "y": 0, "angleDeg": 180, "lengthM": ROD_L, "joint": {"type": "prismatic", "axisDeg": 180, "minM": 0, "maxM": 0.40}, "mass": MASS_ENTRY["endpin"], "section": SECTIONS["endpin"]},
        {"id": "wheel", "parent": "bass", "title": T("Twin wheels of the bass wheel on their axle in the clamp", "Zwillingsrollen des Kontrabassrads auf ihrer Achse in der Klemme"),
         "x": AXLE_U, "y": 0, "angleDeg": 0, "lengthM": 0.03, "joint": {"type": "revolute", "minDeg": -1440, "maxDeg": 1440}, "mass": MASS_ENTRY["wheel"]},
    ]


def find_pivot(bones):
    """🎯 The pivot of the roll-up: on the family of points that turn the cover so the wheels end on the floor, the one nearest the lower bout whose sweep keeps every point of cover, sleeve and wheels on or above the floor."""
    global Q_PIVOT
    phi = CARRY_DEG + DELTA
    pts = [cover_to_world((u, v, 0)) for u, v in bag_loop(50)] + [cover_to_world((u, s * rx, 0)) for u, rx, _ in SLEEVE for s in (-1, 1)]
    wheel = [cover_to_world((AXLE_U + WHEEL_R * math.cos(a), WHEEL_R * math.sin(a), 0)) for a in [2 * math.pi * k / 24 for k in range(24)]]
    axle = cover_to_world((AXLE_U, 0, 0))
    c, s = math.cos(math.radians(phi)), math.sin(math.radians(phi))
    best = None
    for k in range(-200, 120):
        qx = k * 0.0025
        qy = (WHEEL_R - s * (axle[0] - qx) - c * axle[1]) / (1 - c)
        worst, wworst = 1e9, 1e9
        for t in [i / 60 for i in range(61)]:
            a = math.radians(phi * t)
            ca, sa = math.cos(a), math.sin(a)
            worst = min([worst] + [qy + sa * (x - qx) + ca * (y - qy) for x, y, _ in pts])
            wworst = min([wworst] + [qy + sa * (x - qx) + ca * (y - qy) for x, y, _ in wheel])
        if worst >= -0.001 and wworst >= -0.0005 and (best is None or qx > best[1]):
            best = (worst, qx, qy)
    Q_PIVOT = (best[1], best[2])
    return best


def played_offsets(angle_extra=0.0):
    """🎻 Offsets of the playing pose: the bass stands upright on its extended endpin, leaning 15 degrees back towards the player, its top turned forward."""
    bones = rig_bones()
    tilt = 90 + LEAN + DELTA + angle_extra
    base = {"bass-tilt": {"angleDeg": tilt}, "bass": {"twistDeg": 90}, "endpin": {"dx": -PLAYED_E}}
    fr = solve_rig(bones, base)
    tip = to_world(fr, "endpin", (ROD_L + TIP_R, 0, 0))
    base["bass-slide"] = {"dx": r6(TIP_X - tip[0])}
    base["bass-lift"] = {"dy": r6(-tip[1])}
    return base


def carried_offsets(frac=1.0):
    """🛞 Offsets of the rolled pose: cover and bass turned together about the pivot until the wheels stand on the floor, the front panel zipped shut."""
    phi = (CARRY_DEG + DELTA) * frac
    return {"cover-tilt": {"angleDeg": r6(phi)}, "bass-tilt": {"angleDeg": r6(phi)}, "cover-flap": {"twistDeg": -90.0}}


def main():
    """📄 Writes the asset document."""
    global REST_FRAMES, H_WORLD
    rest_geometry()
    (a1, b1), _ = hull_contacts()
    hw = cover_to_world((a1, b1, 0))
    H_WORLD = (0.0, 0.0, -0.325)
    REST_FRAMES = {}
    global Q_PIVOT
    Q_PIVOT = (0.0, 0.0)
    best = find_pivot(None)
    REST_FRAMES = solve_rig(rig_bones(), {})
    global DIMENSIONS
    lo, hi = [1e9] * 3, [-1e9] * 3
    for part in all_parts(50):
        if part.d3 is None or part.extra.get("texture"):
            continue
        a, b = world_bounds(part)
        lo = [min(lo[i], a[i]) for i in range(3)]
        hi = [max(hi[i], b[i]) for i in range(3)]
    DIMENSIONS = {"heightM": r6(round(hi[1] - max(0.0, lo[1]), 3)), "widthM": r6(round(hi[0] - lo[0], 3)), "depthM": r6(round(hi[2] - lo[2], 3)), "massKg": r6(round(M_TOTAL, 2))}
    print(f"[DEBUG] bounds lo {lo} hi {hi} dims {DIMENSIONS}")
    doc = document()
    with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf8") as fh:
        json.dump(doc, fh, ensure_ascii=False, indent=1)
        fh.write("\n")
    fr = solve_rig(rig_bones(), played_offsets())
    print(f"[DEBUG] delta {DELTA:.3f} deg, B {B_WORLD}, pivot {Q_PIVOT}, roll-up lowest {best[0]:.4f} m, clearance bag-bass {clearance(outline(50), bag_half(50)):.4f} m")
    print(f"[DEBUG] played tip {to_world(fr, 'endpin', (ROD_L + TIP_R, 0, 0))} scroll {to_world(fr, 'bass', (SCROLL_TOP, 0, -0.128))} offsets {played_offsets()}")


if __name__ == "__main__":
    main()
