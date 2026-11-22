"""🛋️ Authoritative author script of the staffage asset `used-corner-sofa`: states every hand-chosen measure of the second-hand L-shaped corner sofa (2.60 x 1.90 m, two parts, worn cushions, unscrewed feet in a bag, two moving blankets) and writes the asset document `🔣️.json` next to it.

Run: `python3 author-used-corner-sofa.py` (no third-party modules).

@see ../../brief.md — the author brief of the ticket
@see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts — the typed contract
"""
import importlib.util
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("geometry", os.path.join(HERE, "author-geometry.py"))
G = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(G)
r5, pts5 = G.r5, G.pts5

SCALES = (50, 100, 200, 500)
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}
BANDS = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}


def T(en, de):
    """🗣️ A bilingual text, English first."""
    return {"en": en, "de": de}


TA = 0.008
TB = 0.016
FOOT_H = 0.10
FRONT = -0.475
BACK = 0.475
CH_FRONT = -1.425
ARM_W = 0.18
ARM_H = 0.52
BASE_H = 0.20
SEAT_T = 0.14
MAIN_L = 1.65
CH_X0 = 1.80
CH_W = 0.95
CH_SEAT_W = 0.77
CH_Z = -0.475
CH_SLIDE = -0.15
BL_A = {"x0": -0.125, "len": 1.80, "z": -0.50, "wid": 2.00, "t": TA}
BL_B = {"x0": 1.825, "len": 0.90, "z": -0.475, "wid": 2.00, "t": TB}
BL_A_MOVE = -1.90
BL_B_MOVE = 1.10
TOTE = {"x": 0.80, "z": -1.00, "lx": 0.26, "lz": 0.14, "h": 0.16, "wall": 0.001, "bottom": 0.002}
FEET_PITCH = 0.0605
SAG = {"seat-1": 0.035, "seat-2": 0.015, "chaise-seat": 0.025}
DIP = {"back-1": 0.03, "back-2": 0.02, "chaise-back": 0.025}
BACK_LEAN = 15.0
BACK_A = 0.278
BACK_B = 0.085
SINK = {"seat-1": -0.055, "seat-2": -0.045, "chaise-seat": -0.05, "back-1": -0.012, "back-2": -0.01, "chaise-back": -0.012}
LIFT = {"main": 0.25, "chaise": 0.22, "tilt": 4.0}

# ---------------------------------------------------------------- rig frames

FOOT_TARGETS = [(0.06, 0.415), (0.06, -0.415), (1.59, 0.415), (1.59, -0.415), (1.71, 0.415), (1.71, -1.365), (2.54, 0.415), (2.54, -1.365)]
FOOT_SLOTS = [(TOTE["x"] + i * FEET_PITCH, TOTE["z"] + j * FEET_PITCH) for j in (0.5, -0.5) for i in (-1.5, -0.5, 0.5, 1.5)]
FOOT_Y0 = TA + TOTE["bottom"]


def foot_path(k):
    """🦶️ Rest point B, travel L, prismatic axis angle and path twist of foot k (tote slot to its socket under the frame in use)."""
    bx, bz = FOOT_SLOTS[k]
    tx, tz = FOOT_TARGETS[k]
    d = [tx - bx, 0.0 - FOOT_Y0, tz - bz]
    length = math.sqrt(sum(c * c for c in d))
    dn = [c / length for c in d]
    theta = math.degrees(math.acos(dn[0]))
    sin_t = math.sin(math.radians(theta))
    tau = math.degrees(math.atan2(-dn[2] / sin_t, dn[1] / sin_t))
    return {"B": (bx, FOOT_Y0, bz), "T": (tx, 0.0, tz), "L": length, "theta": round(theta, 6), "tau": round(tau, 6)}


PATHS = [foot_path(k) for k in range(8)]


def check_paths():
    """✅️ Proves that every foot rests upright (frame Rw(90)) and slides exactly onto its socket."""
    for p in PATHS:
        rp = G.ru(p["tau"])
        rf = G.mul(G.rw(90), G.rv(p["tau"]))
        world = G.mul(rp, rf)
        target = G.rw(90)
        assert all(abs(world[i][j] - target[i][j]) < 1e-9 for i in range(3) for j in range(3)), "foot frame"
        u, v = G.column(rp, 0), G.column(rp, 1)
        c, s = math.cos(math.radians(p["theta"])), math.sin(math.radians(p["theta"]))
        end = [p["B"][i] + p["L"] * (c * u[i] + s * v[i]) for i in range(3)]
        assert all(abs(end[i] - p["T"][i]) < 1e-5 for i in range(3)), ("foot target", end, p["T"])


check_paths()


class Frame:
    """📐️ The rest frame of a bone in world metres: origin and rotation (columns u, v, w)."""

    def __init__(self, origin, rot=None):
        self.o = origin
        self.r = rot or [[1.0, 0, 0], [0, 1.0, 0], [0, 0, 1.0]]

    def world(self, p):
        d = G.apply(self.r, p)
        return (self.o[0] + d[0], self.o[1] + d[1], self.o[2] + d[2])


MAIN_O = (0.0, TA, 0.0)
CH_O = (CH_X0, TB, CH_Z)
FR = {
    "ground": Frame((0.0, 0.0, 0.0)),
    "blanket-main": Frame((BL_A["x0"], 0.0, BL_A["z"])),
    "blanket-chaise": Frame((BL_B["x0"], 0.0, BL_B["z"])),
    "tote": Frame((TOTE["x"], TA, TOTE["z"])),
    "main": Frame(MAIN_O),
    "chaise": Frame(CH_O),
}
SEAT_Z = {}
BACK_Z = {}

# ------------------------------------------------------------- back geometry


def frame_profile(scale):
    """🪵️ The back frame section (v from the base bottom, w lateral) of one scale: leaning front face, rolled top, vertical back."""
    lean = math.tan(math.radians(6))
    seg = {50: 6, 100: 3, 200: 1}.get(scale)
    if seg is None:
        return [(0.20, 0.30), (0.57, 0.30 + 0.37 * lean), (0.62, 0.39), (0.62, 0.475), (0.20, 0.475)]
    wf = 0.30 + 0.37 * lean
    pts = [(0.20, 0.30), (0.57, wf)]
    pts += [(0.57 + 0.05 * math.sin(math.radians(a)), wf + 0.05 - 0.05 * math.cos(math.radians(a))) for a in [90 * k / seg for k in range(1, seg + 1)]]
    pts += [(0.62 - 0.03 + 0.03 * math.cos(math.radians(a)), 0.445 + 0.03 * math.sin(math.radians(a))) for a in [90 * k / seg for k in range(0, seg + 1)]]
    pts += [(0.20, 0.475)]
    out = []
    for p in pts:
        if not out or math.hypot(p[0] - out[-1][0], p[1] - out[-1][1]) > 1e-6:
            out.append(p)
    return out


def back_section_local(a, b, n=4, count=64):
    """🧸️ A leaning back-cushion section about its own centre in (v, w)."""
    c, s = math.cos(math.radians(BACK_LEAN)), math.sin(math.radians(BACK_LEAN))
    return [(x * c - y * s, x * s + y * c) for x, y in G.superellipse(a, b, n, count)]


def fit_back():
    """📏️ Places the full back cushion section: bottom on the deck (v 0.20), back against the frame face."""
    frame = [(p[0], p[1]) for p in frame_profile(50)]
    sec = back_section_local(BACK_A, BACK_B)
    vmin = min(p[0] for p in sec)
    vc = 0.20 - vmin
    lo, hi = -0.3, 0.5
    for _ in range(60):
        mid = (lo + hi) / 2
        if any(G.inside((vc + p[0], mid + p[1]), frame) for p in sec):
            hi = mid
        else:
            lo = mid
    wc = lo - 0.001
    pts = [(vc + p[0], wc + p[1]) for p in sec]
    seat_back = min(p[1] for p in pts if p[0] < 0.20 + SEAT_T + 0.01) - 0.002
    top = max(p[0] for p in pts)
    return {"vc": vc, "wc": wc, "seat_back": seat_back, "top": top}


BACKFIT = fit_back()
SEAT_W0 = FRONT
SEAT_W1 = BACKFIT["seat_back"]
SEAT_WC = (SEAT_W0 + SEAT_W1) / 2
CH_SEAT_W0 = CH_FRONT
CH_SEAT_WC = (CH_SEAT_W0 + SEAT_W1) / 2

# ------------------------------------------------------------------- pieces


class Piece:
    """🧩️ One body solid with its three drawn views (elevation in bone u, v; front in world z, y; plan in world x, z)."""

    def __init__(self, pid, bone, region, material, tone, solid, views, z):
        self.id, self.bone, self.region, self.material, self.tone = pid, bone, region, material, tone
        self.solid, self.views, self.z = solid, views, z


def solid_base(pid, bone, material, tone, role="body", **flags):
    """🧱️ The common fields of a body solid."""
    d = {"id": pid, "bone": bone, "material": material, "role": role, "tone": tone}
    d.update({k: v for k, v in flags.items() if v})
    return d


def rbox(pid, bone, region, material, tone, center, size, r, z, sheet=False, seg=4):
    """⬜️ A rounded box (axis-aligned in its bone) with its rounded-rectangle views."""
    fr = FR[bone]
    s = solid_base(pid, bone, material, tone, sheet=sheet)
    s.update({"kind": "box", "size": [r5(c) for c in size]})
    if r > 0:
        s["cornerRadius"] = r5(r)
    s["transform"] = {"translation": [r5(c) for c in center]}
    corners = [fr.world((center[0] + i * size[0] / 2, center[1] + j * size[1] / 2, center[2] + k * size[2] / 2)) for i in (-1, 1) for j in (-1, 1) for k in (-1, 1)]
    lo = [min(c[i] for c in corners) for i in range(3)]
    hi = [max(c[i] for c in corners) for i in range(3)]
    elo = (center[0] - size[0] / 2, center[1] - size[1] / 2)
    ehi = (center[0] + size[0] / 2, center[1] + size[1] / 2)
    views = {
        "elevation": G.rrect(elo[0], elo[1], ehi[0], ehi[1], r, seg),
        "front": G.rrect(lo[2], lo[1], hi[2], hi[1], r, seg),
        "plan": G.rrect(lo[0], lo[2], hi[0], hi[2], r, seg),
    }
    return Piece(pid, bone, region, material, tone, s, views, z)


def extrude_u(pid, bone, region, material, tone, profile, u0, u1, z):
    """🪵️ A prism of a (v, w) profile along the bone axis u (plane `yz`)."""
    fr = FR[bone]
    s = solid_base(pid, bone, material, tone)
    s.update({"kind": "extrude", "plane": "yz", "profile": pts5(profile), "from": r5(u0), "to": r5(u1)})
    vs = [p[0] for p in profile]
    ws = [p[1] for p in profile]
    views = {
        "elevation": [(u0, min(vs)), (u1, min(vs)), (u1, max(vs)), (u0, max(vs))],
        "front": [(fr.o[2] + p[1], fr.o[1] + p[0]) for p in profile],
        "plan": [(fr.o[0] + u0, fr.o[2] + min(ws)), (fr.o[0] + u1, fr.o[2] + min(ws)), (fr.o[0] + u1, fr.o[2] + max(ws)), (fr.o[0] + u0, fr.o[2] + max(ws))],
    }
    return Piece(pid, bone, region, material, tone, s, views, z)


def loft_u(pid, bone, region, material, tone, sections, z):
    """🧸️ A loft along the bone axis u: elevation and plan from the extents of every section, front from their hull."""
    fr = FR[bone]
    s = solid_base(pid, bone, material, tone)
    s.update({"kind": "loft", "sections": [{"at": [r5(c) for c in sc["at"]], "rx": r5(sc["rx"]), "ry": r5(sc["ry"]), "exponent": sc["exponent"], "rotation": [round(q, 6) for q in sc["rotation"]]} for sc in sections], "capStart": True, "capEnd": True})
    top, bottom, left, right, cloud = [], [], [], [], []
    for sc in s["sections"]:
        pts = G.section_points(sc, 72)
        u = sc["at"][0]
        top.append((u, max(p[1] for p in pts)))
        bottom.append((u, min(p[1] for p in pts)))
        right.append((fr.o[0] + u, fr.o[2] + max(p[2] for p in pts)))
        left.append((fr.o[0] + u, fr.o[2] + min(p[2] for p in pts)))
        cloud += [(fr.o[2] + p[2], fr.o[1] + p[1]) for p in pts]
    views = {"elevation": bottom + top[::-1], "front": G.hull(cloud), "plan": left + right[::-1]}
    return Piece(pid, bone, region, material, tone, s, views, z)


def revolve_foot(pid, bone, profile, z):
    """🦶️ A turned foot about its own upright bone axis u."""
    fr = FR[bone]
    s = solid_base(pid, bone, "beech-lacquered", 0.35)
    s.update({"kind": "revolve", "profile": pts5(profile), "axis": [1, 0, 0], "origin": [0, 0, 0]})
    rim = [p for p in profile if p[1] > 0]
    side = [(p[0], p[1]) for p in rim]
    elev = side + [(p[0], -p[1]) for p in side[::-1]]
    front = [(fr.o[2] + p[1], fr.o[1] + p[0]) for p in side] + [(fr.o[2] - p[1], fr.o[1] + p[0]) for p in side[::-1]]
    rmax = max(p[1] for p in profile)
    views = {"elevation": elev, "front": front, "plan": ("ellipse", fr.o[0], fr.o[2], rmax)}
    return Piece(pid, bone, "feet", "beech-lacquered", 0.35, s, views, z)


def box_foot(pid, bone, z):
    """🦶️ A foot drawn and modelled at the 1:500 feature: a 75 mm block."""
    p = rbox(pid, bone, "feet", "beech-lacquered", 0.35, (FOOT_H / 2, 0.0, 0.0), (FOOT_H, 0.075, 0.075), 0.0, z)
    return p


def tote_walls(scale):
    """👜️ The open woven bag: hollow walls (plane `zx`, along v) and its bottom sheet."""
    hx, hz, w = TOTE["lx"] / 2, TOTE["lz"] / 2, TOTE["wall"]
    s = solid_base("bag-walls", "tote", "pp-woven", 0.5)
    outer = [(-hz, -hx), (hz, -hx), (hz, hx), (-hz, hx)]
    inner = [(-hz + w, -hx + w), (-hz + w, hx - w), (hz - w, hx - w), (hz - w, -hx + w)]
    s.update({"kind": "extrude", "plane": "zx", "profile": pts5(outer), "holes": [pts5(inner)], "from": 0.0, "to": r5(TOTE["h"])})
    fr = FR["tote"]
    views = {
        "elevation": [(-hx, 0.0), (hx, 0.0), (hx, TOTE["h"]), (-hx, TOTE["h"])],
        "front": [(fr.o[2] - hz, fr.o[1]), (fr.o[2] + hz, fr.o[1]), (fr.o[2] + hz, fr.o[1] + TOTE["h"]), (fr.o[2] - hz, fr.o[1] + TOTE["h"])],
        "plan": ("ring", [(fr.o[0] - hx, fr.o[2] - hz), (fr.o[0] + hx, fr.o[2] - hz), (fr.o[0] + hx, fr.o[2] + hz), (fr.o[0] - hx, fr.o[2] + hz)], [(fr.o[0] - hx + w, fr.o[2] - hz + w), (fr.o[0] - hx + w, fr.o[2] + hz - w), (fr.o[0] + hx - w, fr.o[2] + hz - w), (fr.o[0] + hx - w, fr.o[2] - hz + w)]),
    }
    walls = Piece("bag-walls", "tote", "bag", "pp-woven", 0.5, s, views, 1.6)
    bottom = rbox("bag-bottom", "tote", "bag", "pp-woven", 0.45, (0.0, TOTE["bottom"] / 2, 0.0), (TOTE["lx"] - 2 * w, TOTE["bottom"], TOTE["lz"] - 2 * w), 0.0, 0.6, sheet=True)
    return [bottom, walls]


def seat_sections(bone, length, w0, w1, sag, scale):
    """🪑️ Loft stations of a seat cushion: end roundings and a sag dip that the scale can still show."""
    re = 0.03
    n = 5
    half_t = SEAT_T / 2
    wc = (w0 + w1) / 2 - (SEAT_Z[bone])
    ry0 = (w1 - w0) / 2
    ends = {50: [0, 0.12, 0.35, 0.65, 1.0], 100: [0, 0.4, 1.0], 200: [0, 1.0]}[scale]
    inner = {50: 9, 100: 5, 200: 3}[scale]
    if sag < FEATURE[scale]:
        sag = 0.0
        inner = 2
    stations = []
    for f in ends:
        d = re * f
        shrink = re - math.sqrt(max(re * re - (re - d) ** 2, 0.0))
        stations.append((d, shrink, 0.0))
    for k in range(1, inner - 1):
        f = k / (inner - 1)
        stations.append((re + f * (length - 2 * re), 0.0, sag * math.sin(math.pi * f) ** 2))
    for f in reversed(ends):
        d = re * f
        shrink = re - math.sqrt(max(re * re - (re - d) ** 2, 0.0))
        stations.append((length - d, shrink, 0.0))
    out = []
    for u, shrink, dip in stations:
        rx = half_t - dip / 2 - shrink
        out.append({"at": [u, half_t - dip / 2, wc], "rx": rx, "ry": ry0 - shrink, "exponent": n, "rotation": G.LIMB})
    return out


def back_sections(bone, length, dip, scale):
    """🧸️ Loft stations of a slumped back cushion: leaning 15 degrees, end roundings, the top dipped where the fibre fill has migrated."""
    re = 0.03
    ends = {50: [0, 0.12, 0.35, 0.65, 1.0], 100: [0, 0.4, 1.0], 200: [0, 1.0]}[scale]
    inner = {50: 9, 100: 5, 200: 3}[scale]
    if dip < FEATURE[scale]:
        dip = 0.0
        inner = 2
    c, s = math.cos(math.radians(BACK_LEAN)), math.sin(math.radians(BACK_LEAN))
    stations = []
    for f in ends:
        d = re * f
        stations.append((d, re - math.sqrt(max(re * re - (re - d) ** 2, 0.0)), 0.0))
    for k in range(1, inner - 1):
        f = k / (inner - 1)
        stations.append((re + f * (length - 2 * re), 0.0, dip * math.sin(math.pi * f) ** 2))
    for f in reversed(ends):
        d = re * f
        stations.append((length - d, re - math.sqrt(max(re * re - (re - d) ** 2, 0.0)), 0.0))
    out = []
    for u, shrink, dd in stations:
        a = BACK_A - dd / 2 - shrink
        b = BACK_B - shrink
        sec = back_section_local(a, b)
        vmin = min(p[0] for p in sec)
        vc = -vmin + shrink
        wc = BACKFIT["wc"] - dd / 2 * s - BACK_Z[bone]
        out.append({"at": [u, vc, wc], "rx": a, "ry": b, "exponent": 4, "rotation": G.lean_quat(BACK_LEAN)})
    return out


def back_block(pid, bone, length, z):
    """🧸️ The 1:500 back cushion: one leaning slab along u with the height of the rounded cushion, leaning against the 1:500 frame block."""
    c, s = math.cos(math.radians(BACK_LEAN)), math.sin(math.radians(BACK_LEAN))
    height = BACKFIT["top"] - 0.20
    b = BACK_B - 0.01
    a = (height / 2 - b * s) / c
    corners = [(x * c - y * s, x * s + y * c) for x, y in [(a, b), (-a, b), (-a, -b), (a, -b)]]
    dense = [(corners[i - 1][0] + (corners[i][0] - corners[i - 1][0]) * t / 20, corners[i - 1][1] + (corners[i][1] - corners[i - 1][1]) * t / 20) for i in range(4) for t in range(20)]
    vmin = min(p[0] for p in corners)
    frame = frame_profile(500)
    lo, hi = -0.3, 0.5
    for _ in range(60):
        mid = (lo + hi) / 2
        if any(G.inside((0.20 - vmin + p[0], mid + p[1]), frame) for p in dense):
            hi = mid
        else:
            lo = mid
    prof = [(p[0] - vmin, lo - 0.02 - BACK_Z[bone] + p[1]) for p in corners]
    return extrude_u(pid, bone, "back-cushions", "fabric-worn", 0.62, prof, 0.0, length, z)


# ----------------------------------------------------------------- the body


def body(scale):
    """🛋️ Every body piece of one scale, each with its three views."""
    pieces = []
    bl_r = {50: 0.0, 100: 0.0, 200: 0.0, 500: 0.0}[scale]
    ta, tb = (0.02, 0.02) if scale == 500 else (TA, TB)
    pieces.append(rbox("blanket-main", "blanket-main", "blankets", "moving-blanket", 0.28, (BL_A["len"] / 2, ta / 2, 0.0), (BL_A["len"], ta, BL_A["wid"]), bl_r, 0.2, sheet=True))
    pieces.append(rbox("blanket-chaise", "blanket-chaise", "blankets", "moving-blanket", 0.3, (BL_B["len"] / 2, tb / 2, 0.0), (BL_B["len"], tb, BL_B["wid"]), {50: 0.008, 100: 0.0075, 200: 0.0, 500: 0.0}[scale], 0.2, sheet=True))
    pieces += tote_walls(scale)
    for k in range(8):
        bone = f"foot-{k + 1}"
        if scale == 500:
            pieces.append(box_foot(f"foot-{k + 1}", bone, 1.0))
        else:
            prof = {
                50: [(0, 0), (0, 0.0215), (0.004, 0.0225), (0.082, 0.0262), (0.084, 0.0285), (0.092, 0.0285), (0.094, 0.03), (0.10, 0.03), (0.10, 0)],
                100: [(0, 0), (0, 0.022), (0.09, 0.027), (0.10, 0.03), (0.10, 0)],
                200: [(0, 0), (0, 0.024), (0.10, 0.029), (0.10, 0)],
            }[scale]
            pieces.append(revolve_foot(f"foot-{k + 1}", bone, prof, 1.0))
    arm_r = {50: 0.04, 100: 0.035, 200: 0.03, 500: 0.0}[scale]
    base_r = {50: 0.012, 100: 0.01, 200: 0.0, 500: 0.0}[scale]
    pieces.append(rbox("arm-main", "main", "arms", "fabric-faded", 0.82, (ARM_W / 2, ARM_H / 2, 0.0), (ARM_W, ARM_H, BACK - FRONT), arm_r, 3.0))
    pieces.append(rbox("base-main", "main", "frame-cover", "fabric-worn", 0.45, ((ARM_W + MAIN_L) / 2, BASE_H / 2, 0.0), (MAIN_L - ARM_W, BASE_H, BACK - FRONT), base_r, 2.0))
    pieces.append(extrude_u("frame-main", "main", "frame-cover", "fabric-worn", 0.55, frame_profile(scale), ARM_W, MAIN_L, 2.5))
    pieces.append(rbox("arm-chaise", "chaise", "arms", "fabric-faded", 0.82, (CH_SEAT_W + ARM_W / 2, ARM_H / 2, (BACK + CH_FRONT) / 2 - CH_Z), (ARM_W, ARM_H, BACK - CH_FRONT), arm_r, 3.0))
    pieces.append(rbox("base-chaise", "chaise", "frame-cover", "fabric-worn", 0.45, (CH_SEAT_W / 2, BASE_H / 2, (BACK + CH_FRONT) / 2 - CH_Z), (CH_SEAT_W, BASE_H, BACK - CH_FRONT), base_r, 2.0))
    pieces.append(extrude_u("frame-chaise", "chaise", "frame-cover", "fabric-worn", 0.55, [(p[0], p[1] - CH_Z) for p in frame_profile(scale)], 0.0, CH_SEAT_W, 2.5))
    for bone, length, w0 in (("seat-1", 0.735, SEAT_W0), ("seat-2", 0.735, SEAT_W0), ("chaise-seat", CH_SEAT_W, CH_SEAT_W0)):
        if scale == 500:
            wc = (w0 + SEAT_W1) / 2 - SEAT_Z[bone]
            pieces.append(rbox(bone, bone, "seat-cushions", "fabric-worn", 0.72, (length / 2, SEAT_T / 2, wc), (length, SEAT_T, SEAT_W1 - w0), 0.0, 5.0))
        else:
            pieces.append(loft_u(bone, bone, "seat-cushions", "fabric-worn", 0.72, seat_sections(bone, length, w0, SEAT_W1, SAG[bone], scale), 5.0))
    for bone, length in (("back-1", 0.735), ("back-2", 0.735), ("chaise-back", CH_SEAT_W)):
        if scale == 500:
            pieces.append(back_block(bone, bone, length, 4.0))
        else:
            pieces.append(loft_u(bone, bone, "back-cushions", "fabric-worn", 0.62, back_sections(bone, length, DIP[bone], scale), 4.0))
    return pieces


SEAT_Z.update({"seat-1": SEAT_WC, "seat-2": SEAT_WC, "chaise-seat": CH_SEAT_WC})
BACK_Z.update({"back-1": BACKFIT["wc"], "back-2": BACKFIT["wc"], "chaise-back": BACKFIT["wc"]})
FR["seat-1"] = Frame((MAIN_O[0] + ARM_W, MAIN_O[1] + BASE_H, SEAT_WC))
FR["seat-2"] = Frame((MAIN_O[0] + ARM_W + 0.735, MAIN_O[1] + BASE_H, SEAT_WC))
FR["back-1"] = Frame((MAIN_O[0] + ARM_W, MAIN_O[1] + BASE_H, BACKFIT["wc"]))
FR["back-2"] = Frame((MAIN_O[0] + ARM_W + 0.735, MAIN_O[1] + BASE_H, BACKFIT["wc"]))
FR["chaise-seat"] = Frame((CH_X0, TB + BASE_H, CH_SEAT_WC))
FR["chaise-back"] = Frame((CH_X0, TB + BASE_H, BACKFIT["wc"]))
for _k, _p in enumerate(PATHS):
    FR[f"foot-{_k + 1}"] = Frame(_p["B"], G.rw(90))

# ------------------------------------------------------------------ drawing

ROLE_Z = {"body": 0.0, "detail": 0.3, "texture": 0.4}


def shape(pid, bone, role, z, tone, geom, region=None, material=None, filled=True, hatch=None):
    """✏️ One 2D shape of a view."""
    d = {"id": pid, "bone": bone, "role": role, "z": round(z, 2), "filled": filled, "tone": tone}
    d.update(geom)
    if region:
        d["region"] = region
    if material:
        d["material"] = material
    if hatch:
        d["hatch"] = hatch
    return d


def poly(points, tol=0.0002):
    """🔺️ A closed polygon geometry, simplified within 0.2 mm."""
    pts = G.simplify(points, tol) if len(points) > 8 else list(points)
    return {"kind": "poly", "points": pts5(pts), "closed": True}


def piece_shapes(piece, view, hatch=None):
    """🖊️ The filled body shape of a piece in one view."""
    g = piece.views[view]
    if isinstance(g, tuple) and g[0] == "ellipse":
        geom = {"kind": "ellipse", "cx": r5(g[1]), "cy": r5(g[2]), "rx": r5(g[3]), "ry": r5(g[3])}
    elif isinstance(g, tuple) and g[0] == "ring":
        d = "M " + " L ".join(f"{r5(x)} {r5(y)}" for x, y in g[1]) + " Z M " + " L ".join(f"{r5(x)} {r5(y)}" for x, y in g[2]) + " Z"
        geom = {"kind": "path", "d": d}
    else:
        geom = poly(g)
    return shape(piece.id, piece.bone, "body", piece.z, piece.tone, geom, piece.region, piece.material, True, hatch)


def path(segments):
    """〰️ An open path geometry of polylines."""
    return {"kind": "path", "d": " ".join("M " + " L ".join(f"{r5(x)} {r5(y)}" for x, y in seg) for seg in segments)}


def details(scale, pieces):
    """🧵️ The detail and texture strokes of one scale, all inside the filled outlines: piping, seams, slump creases, the faded band of the arms, the coffee ring, the price sticker."""
    by = {p.id: p for p in pieces}
    out = {"elevation": [], "front": [], "plan": []}
    if scale == 500:
        return out
    inset = {50: 0.012, 100: 0.016, 200: 0.0}[scale]
    for sid in ("seat-1", "seat-2", "chaise-seat"):
        p = by[sid]
        secs = p.solid["sections"]
        if scale in (50, 100):
            topline = []
            for sc in secs[2:-2] if scale == 50 else secs[1:-1]:
                topline.append((sc["at"][0], sc["at"][1] + sc["rx"] - inset))
            out["elevation"].append(shape(f"piping-{sid}", sid, "detail", 5.5, 0.3, path([topline, [(secs[2 if scale == 50 else 1]["at"][0], inset), (secs[-3 if scale == 50 else -2]["at"][0], inset)]]), p.region))
            fr = FR[sid]
            u0, u1 = fr.o[0] + secs[1]["at"][0] + inset, fr.o[0] + secs[-2]["at"][0] - inset
            ry = secs[len(secs) // 2]["ry"]
            out["plan"].append(shape(f"piping-{sid}", sid, "detail", 5.5, 0.3, {"kind": "poly", "points": pts5(G.rrect(u0, fr.o[2] - ry + inset, u1, fr.o[2] + ry - inset, 0.03, 3)), "closed": True}, p.region, filled=False))
        if scale == 50 and SAG[sid] >= 0.02:
            fr = FR[sid]
            length = secs[-1]["at"][0]
            out["plan"].append(shape(f"sag-{sid}", sid, "texture", 5.6, 0.4, {"kind": "ellipse", "cx": r5(fr.o[0] + length / 2), "cy": r5(fr.o[2] - 0.04), "rx": r5(length * 0.28), "ry": 0.15}, p.region, filled=False))
    for bid in ("back-1", "back-2", "chaise-back"):
        p = by[bid]
        if scale == 50:
            secs = p.solid["sections"]
            crease = []
            for sc in secs[2:-2]:
                pts = G.section_points(sc, 72)
                vmin = min(q[1] for q in pts)
                vmax = max(q[1] for q in pts)
                crease.append((sc["at"][0], vmin + 0.42 * (vmax - vmin)))
            out["elevation"].append(shape(f"crease-{bid}", bid, "texture", 4.5, 0.4, path([crease]), p.region))
    if scale in (50, 100):
        for aid, bone in (("arm-main", "main"), ("arm-chaise", "chaise")):
            p = by[aid]
            e = p.views["elevation"]
            u0, u1 = min(q[0] for q in e) + 0.03, max(q[0] for q in e) - 0.03
            band = ARM_H - 0.11
            wave = [(u0 + (u1 - u0) * k / 6, band + (0.006 if k % 2 else -0.006)) for k in range(7)]
            out["elevation"].append(shape(f"fade-{aid}", bone, "texture", 3.5, 0.5, path([wave]), p.region))
            pl = p.views["plan"]
            x0, x1 = min(q[0] for q in pl), max(q[0] for q in pl)
            z0, z1 = min(q[1] for q in pl), max(q[1] for q in pl)
            out["plan"].append(shape(f"fade-{aid}", bone, "texture", 3.5, 0.5, path([[(x0 + 0.05, z0 + 0.06), (x0 + 0.05, z1 - 0.06)], [(x1 - 0.05, z0 + 0.06), (x1 - 0.05, z1 - 0.06)]]), p.region))
        fr = FR["main"]
        out["elevation"].append(shape("seam-base-main", "main", "detail", 2.2, 0.3, path([[(ARM_W + 0.015, BASE_H - 0.02), (MAIN_L - 0.015, BASE_H - 0.02)]]), "frame-cover"))
        out["elevation"].append(shape("seam-base-chaise", "chaise", "detail", 2.2, 0.3, path([[(0.015, BASE_H - 0.02), (CH_SEAT_W - 0.015, BASE_H - 0.02)]]), "frame-cover"))
        p = by["arm-chaise"]
        f = p.views["front"]
        z0, z1 = min(q[0] for q in f), max(q[0] for q in f)
        out["front"].append(shape("fade-arm-chaise", "chaise", "texture", 3.5, 0.5, path([[(z0 + 0.05 + (z1 - z0 - 0.1) * k / 10, ARM_H - 0.11 + TB + (0.006 if k % 2 else -0.006)) for k in range(11)]]), "arms"))
        out["front"].append(shape("seam-arm-chaise", "chaise", "detail", 3.4, 0.3, path([[(z0 + 0.03, TB + 0.03), (z1 - 0.03, TB + 0.03)]]), "arms"))
    if scale == 50:
        out["plan"].append(shape("coffee-ring", "chaise", "texture", 3.6, 0.3, {"kind": "ellipse", "cx": r5(CH_X0 + CH_SEAT_W + ARM_W / 2), "cy": -0.82, "rx": 0.036, "ry": 0.036}, "arms", filled=False))
        out["elevation"].append(shape("price-sticker", "chaise", "detail", 2.3, 0.2, {"kind": "poly", "points": pts5([(0.30, 0.07), (0.38, 0.07), (0.38, 0.12), (0.30, 0.12)]), "closed": True}, "frame-cover", filled=False))
        out["elevation"].append(shape("bag-hem", "tote", "detail", 1.7, 0.3, path([[(-TOTE["lx"] / 2 + 0.004, TOTE["h"] - 0.012), (TOTE["lx"] / 2 - 0.004, TOTE["h"] - 0.012)]]), "bag"))
        for k in range(8):
            bone = f"foot-{k + 1}"
            out["elevation"].append(shape(f"ring-{bone}", bone, "detail", 1.5, 0.3, path([[(0.084, -0.0285), (0.084, 0.0285)], [(0.092, -0.0285), (0.092, 0.0285)]]), "feet"))
    return out


def drawings(scale, pieces):
    """🗺️ The three views of one scale: one filled shape per body piece and the detail strokes."""
    hatch = "quilting" if scale in (50, 100) else None
    geo = {"elevation": [], "front": [], "plan": []}
    for p in pieces:
        for view in geo:
            geo[view].append(piece_shapes(p, view, hatch if (view == "plan" and p.region == "blankets") else None))
    for view, extra in details(scale, pieces).items():
        geo[view] += extra
    return geo

# ---------------------------------------------------------------- materials

MATERIALS = [
    {"id": "fabric-worn", "title": T("Worn polyester flat-weave cover, pilled", "Abgenutzter Polyester-Flachgewebebezug, pillig"), "emissivity": 0.9, "solarAbsorptivity": 0.62, "densityKgM3": 400, "conductivityWmK": 0.05, "specificHeatJKgK": 1300, "source": "authored-estimate", "reference": "ashrae-2021-fundamentals"},
    {"id": "fabric-faded", "title": T("Sun-faded cover fabric of the arms", "Sonnengebleichter Bezugsstoff der Armlehnen"), "emissivity": 0.9, "solarAbsorptivity": 0.48, "densityKgM3": 400, "conductivityWmK": 0.05, "specificHeatJKgK": 1300, "source": "authored-estimate", "reference": "ashrae-2021-fundamentals"},
    {"id": "pu-foam", "title": T("PU foam RG 35, aged", "PU-Schaum RG 35, gealtert"), "emissivity": 0.9, "solarAbsorptivity": 0.5, "densityKgM3": 35, "conductivityWmK": 0.04, "specificHeatJKgK": 1400, "elasticModulusPa": 12000.0, "strengthPa": 4000.0, "source": "authored-estimate", "reference": "iso-3386-1"},
    {"id": "fibre-fill", "title": T("Polyester fibre fill of the back cushions", "Polyester-Faserfüllung der Rückenkissen"), "emissivity": 0.9, "solarAbsorptivity": 0.5, "densityKgM3": 22, "conductivityWmK": 0.045, "specificHeatJKgK": 1300, "source": "authored-estimate", "reference": "iso-10456-2007"},
    {"id": "chipboard-p2", "title": T("Chipboard P2 (EN 312)", "Spanplatte P2 (EN 312)"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 650, "conductivityWmK": 0.14, "specificHeatJKgK": 1700, "elasticModulusPa": 1600000000.0, "strengthPa": 11000000.0, "source": "literature", "reference": "en-312-2010"},
    {"id": "spruce-c24", "title": T("Spruce batten, strength class C24", "Fichtenleiste, Festigkeitsklasse C24"), "emissivity": 0.9, "solarAbsorptivity": 0.55, "densityKgM3": 420, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11000000000.0, "strengthPa": 24000000.0, "source": "literature", "reference": "en-338-2016"},
    {"id": "spring-steel", "title": T("Zigzag (Nosag) spring wire 4 mm", "Wellenfederdraht (Nosag) 4 mm"), "emissivity": 0.3, "solarAbsorptivity": 0.6, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 206000000000.0, "strengthPa": 1600000000.0, "source": "literature", "reference": "en-10270-1-2011"},
    {"id": "beech-lacquered", "title": T("Turned beech, scratched lacquer", "Gedrechselte Buche, zerkratzter Lack"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 720, "conductivityWmK": 0.18, "specificHeatJKgK": 1600, "elasticModulusPa": 14000000000.0, "strengthPa": 100000000.0, "source": "literature", "reference": "wood-handbook-2021"},
    {"id": "moving-blanket", "title": T("Quilted moving blanket, cotton-polyester with non-woven fill", "Gesteppte Möbeldecke, Baumwoll-Polyester mit Vliesfüllung"), "emissivity": 0.9, "solarAbsorptivity": 0.8, "densityKgM3": 90, "conductivityWmK": 0.045, "specificHeatJKgK": 1300, "source": "authored-estimate", "reference": "iso-10456-2007"},
    {"id": "hardboard", "title": T("Hardboard HB 3.2 mm (EN 622-2)", "Hartfaserplatte HB 3,2 mm (EN 622-2)"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 850, "conductivityWmK": 0.17, "specificHeatJKgK": 1700, "elasticModulusPa": 3500000000.0, "strengthPa": 30000000.0, "source": "literature", "reference": "en-622-2-2004"},
    {"id": "pp-woven", "title": T("Woven polypropylene carrier bag", "Gewebte Polypropylen-Tragetasche"), "emissivity": 0.9, "solarAbsorptivity": 0.5, "densityKgM3": 900, "conductivityWmK": 0.22, "specificHeatJKgK": 1700, "source": "literature", "reference": "iso-10456-2007"},
]
DENSITY = {m["id"]: m["densityKgM3"] for m in MATERIALS}

# ------------------------------------------------------------------ thermal

REGION_DEFS = [
    ("seat-cushions", T("Seat cushions, worn and sagging", "Sitzkissen, abgenutzt und durchgesessen"), "fabric-worn", 0.57, "horizontal-plate-up", 0.8, ("foam-core", T("Aged PU foam core 0.14 m under the cover", "Gealterter PU-Schaumkern 0,14 m unter dem Bezug"), "insulation", "pu-foam", 0.14, 0.04), "authored-estimate"),
    ("back-cushions", T("Loose back cushions, slumped", "Lose Rückenkissen, zusammengesackt"), "fabric-worn", 0.5, "vertical-plate", 0.5, ("fibre-core", T("Polyester fibre fill 0.17 m", "Polyester-Faserfüllung 0,17 m"), "insulation", "fibre-fill", 0.17, 0.045), "authored-estimate"),
    ("arms", T("Box arms with sun-faded tops", "Kastenarmlehnen mit sonnengebleichten Oberseiten"), "fabric-faded", 0.52, "vertical-plate", 0.5, ("arm-padding", T("PU padding 30 mm over the chipboard panel", "PU-Polsterung 30 mm über der Spanplatte"), "insulation", "pu-foam", 0.03, 0.04), "authored-estimate"),
    ("frame-cover", T("Upholstered base and back frame", "Gepolsterter Sockel und Rückenrahmen"), "fabric-worn", 0.4, "vertical-plate", 0.5, ("frame-board", T("Chipboard P2 16 mm under the cover", "Spanplatte P2 16 mm unter dem Bezug"), "timber", "chipboard-p2", 0.016, 0.14), "literature"),
    ("feet", T("Unscrewed beech feet", "Abgeschraubte Buchenfüße"), "beech-lacquered", 0.05, "cylinder", 0.3, ("beech-foot", T("Beech, half diameter 25 mm", "Buche, halber Durchmesser 25 mm"), "timber", "beech-lacquered", 0.025, 0.18), "literature"),
    ("blankets", T("Two quilted moving blankets", "Zwei gesteppte Möbeldecken"), "moving-blanket", 1.8, "horizontal-plate-up", 0.6, ("blanket-quilt", T("Quilted blanket 8 mm", "Gesteppte Decke 8 mm"), "insulation", "moving-blanket", 0.008, 0.045), "authored-estimate"),
    ("bag", T("Carrier bag holding the feet", "Tragetasche mit den Füßen"), "pp-woven", 0.17, "vertical-plate", 0.4, ("bag-film", T("Woven PP 1 mm", "PP-Gewebe 1 mm"), "polymer", "pp-woven", 0.001, 0.22), "literature"),
]
SPECIFIC = {m["id"]: (m["densityKgM3"], m["conductivityWmK"], m["specificHeatJKgK"]) for m in MATERIALS}


def heat_capacity(material, thickness):
    """🔥️ Areal heat capacity of a layer, limited to the thermal penetration depth of one hour sqrt(alpha t)."""
    rho, k, c = SPECIFIC[material]
    depth = min(thickness, math.sqrt(k / (rho * c) * 3600))
    return round(rho * c * depth + 520)


def regions(areas):
    """🌡️ The thermal regions with their single layer; areas are the outer areas of the 1:50 model."""
    out = []
    for rid, title, material, length, geometry, sky, layer, src in REGION_DEFS:
        lid, ltitle, lkind, lmat, thick, k = layer
        hc = heat_capacity(lmat, thick)
        reg = {"id": rid, "title": title, "kind": "surface", "model": "passive", "material": material, "areaM2": areas[rid], "characteristicLengthM": length, "geometry": geometry, "skyViewFactor": sky,
               "layers": [{"id": lid, "title": ltitle, "kind": lkind, "material": lmat, "thicknessM": thick, "resistanceM2KW": round(thick / k, 6), "heatCapacityJM2K": hc, "source": src, "reference": "iso-10456-2007"}],
               "heatCapacityJM2K": hc, "source": "authored-estimate", "reference": "iso-10456-2007"}
        if rid == "frame-cover":
            reg["ratedPowerW"] = 0
            reg["ratedPowerBasis"] = "passive"
        out.append(reg)
    return out


def thermal_list(pieces):
    """🌡️ One thermal solid per body piece: the body surface is the region surface (every region has a single layer)."""
    return [{"id": f"th-{p.id}", "of": p.id, "region": p.region} for p in pieces]

# ---------------------------------------------------------------- structure


def sbox(sid, bone, material, center, size, member, part, anchor=None, role="body", tone=0.3, **flags):
    """🏗️ A structural box solid in its bone frame."""
    d = solid_base(sid, bone, material, tone, role, **flags)
    d.update({"kind": "box", "size": [r5(c) for c in size], "transform": {"translation": [r5(c) for c in center]}, "member": member, "part": part})
    if anchor:
        d["anchor"] = anchor
    return d


def scap(sid, bone, p0, p1, r, member):
    """🌀️ A zigzag spring idealised as a slender wire capsule."""
    d = solid_base(sid, bone, "spring-steel", 0.25, slender=True)
    d.update({"kind": "capsule", "p0": [r5(c) for c in p0], "p1": [r5(c) for c in p1], "r0": r, "r1": r, "member": member, "part": "member"})
    return d


def springs(prefix, bone, positions, w0, w1, v, sagging, along_u=False, u0=0.0, u1=0.0):
    """🌀️ Nosag springs at the deck: straight, or bent down 20 mm at mid span where the seat above is worn."""
    out = []
    for i, (q, sag) in enumerate(zip(positions, sagging)):
        if along_u:
            a, b, m = (u0, v, q), (u1, v, q), ((u0 + u1) / 2, v - sag, q)
        else:
            a, b, m = (q, v, w0), (q, v, w1), (q, v - sag, (w0 + w1) / 2)
        if sag > 0:
            out.append(scap(f"{prefix}-{i + 1}a", bone, a, m, 0.002, bone))
            out.append(scap(f"{prefix}-{i + 1}b", bone, m, b, 0.002, bone))
        else:
            out.append(scap(f"{prefix}-{i + 1}", bone, a, b, 0.002, bone))
    return out


def part_frame(scale, bone):
    """🪵️ The timber frame members of one part (main or chaise) at one scale, in its bone frame."""
    f = FEATURE[scale]
    out = []
    if bone == "main":
        u0, u1, wc, half = ARM_W - 0.012, MAIN_L - 0.01, 0.0, 0.475
        arm_u, arm_w, arm_len = 0.0, 0.0, 0.95
        end_u = MAIN_L - 0.019
        seat_springs = [0.27, 0.45, 0.63, 0.81, 1.02, 1.20, 1.38, 1.56]
        sag = [0.0, 0.02, 0.02, 0.0, 0.0, 0.0, 0.0, 0.0]
        cross = [(0.915, 0.0, 0.894)]
    else:
        u0, u1, wc, half = 0.01, CH_SEAT_W + 0.012, 0.0, 0.95
        arm_u, arm_w, arm_len = CH_SEAT_W, 0.0, 1.90
        end_u = 0.019
        seat_springs = [-0.85, -0.68, -0.51, -0.34, -0.17, 0.0, 0.17]
        sag = [0.0, 0.0, 0.015, 0.015, 0.015, 0.0, 0.0]
        cross = [((u0 + u1) / 2, -0.255, u1 - u0)]
    rail_w = half - 0.019
    side = bone == "chaise"
    if scale in (50, 100):
        out.append(sbox(f"rail-front-{bone}", bone, "chipboard-p2", ((u0 + u1) / 2, 0.1025, wc - rail_w), (u1 - u0, 0.195, 0.018), bone, "member"))
        out.append(sbox(f"rail-back-{bone}", bone, "chipboard-p2", ((u0 + u1) / 2, 0.1025, wc + rail_w), (u1 - u0, 0.195, 0.018), bone, "member"))
        out.append(sbox(f"rail-end-{bone}", bone, "chipboard-p2", (end_u, 0.1025, wc), (0.018, 0.195, 2 * rail_w - 0.018), bone, "member"))
        if scale == 50:
            for i, (cu, cw, cl) in enumerate(cross):
                size = (0.044, 0.07, 2 * rail_w - 0.018) if bone == "main" else (cl, 0.07, 0.044)
                out.append(sbox(f"rail-cross-{bone}-{i + 1}", bone, "spruce-c24", (cu, 0.16, cw), size, bone, "member"))
            if bone == "main":
                out += springs("spring-main", bone, seat_springs, -rail_w + 0.004, rail_w - 0.004, 0.198, sag)
            else:
                out += springs("spring-chaise", bone, seat_springs, 0, 0, 0.198, sag, True, 0.024, CH_SEAT_W + 0.014)
        else:
            pick = [0, 2, 5, 7] if bone == "main" else [0, 3, 6]
            out += springs(f"spring-{bone}", bone, [seat_springs[i] for i in pick], -rail_w + 0.004, rail_w - 0.004, 0.198, [sag[i] if sag[i] >= f else 0.0 for i in pick], side, 0.024, CH_SEAT_W + 0.014)
    elif scale == 200:
        out.append(sbox(f"rail-front-{bone}", bone, "chipboard-p2", ((u0 + u1) / 2, 0.1025, wc - rail_w), (u1 - u0, 0.195, 0.018), bone, "member", slender=True))
        out.append(sbox(f"rail-back-{bone}", bone, "chipboard-p2", ((u0 + u1) / 2, 0.1025, wc + rail_w), (u1 - u0, 0.195, 0.018), bone, "member", slender=True))
        if bone == "main":
            out.append(sbox(f"rail-end-{bone}", bone, "chipboard-p2", (end_u - 0.006, 0.1025, wc), (0.03, 0.195, 2 * rail_w - 0.018), bone, "member"))
        else:
            out.append(sbox(f"rail-end-{bone}", bone, "chipboard-p2", (end_u, 0.1025, wc), (0.018, 0.195, 2 * rail_w - 0.018), bone, "member", slender=True))
        pick = [1, 6] if bone == "main" else [1, 5]
        out += springs(f"spring-{bone}", bone, [seat_springs[i] for i in pick], -rail_w + 0.004, rail_w - 0.004, 0.198, [0.0, 0.0], side, 0.024, CH_SEAT_W + 0.014)
    else:
        out.append(sbox(f"rail-front-{bone}", bone, "chipboard-p2", ((u0 + u1) / 2, 0.1025, wc - rail_w), (u1 - u0, 0.195, 0.018), bone, "member", slender=True))
        out.append(sbox(f"rail-back-{bone}", bone, "chipboard-p2", ((u0 + u1) / 2, 0.1025, wc + rail_w), (u1 - u0, 0.195, 0.018), bone, "member", slender=True))
        du = 0.075
        out.append(sbox(f"deck-{bone}", bone, "chipboard-p2", ((u0 + u1) / 2, 0.2 - du / 2, wc), (u1 - u0 - 0.02, du, 2 * rail_w - 0.018), bone, "member"))
    au0 = arm_u + 0.012
    au1 = arm_u + ARM_W - 0.012
    aw = arm_len / 2 - 0.015
    awc = 0.0
    if scale in (50, 100):
        out.append(sbox(f"arm-panel-out-{bone}", bone, "hardboard", ((au0 + 0.0016 if bone == "main" else au1 - 0.0016), 0.2525, awc), (0.0032, 0.495, 2 * aw), bone, "member", sheet=True))
        out.append(sbox(f"arm-panel-in-{bone}", bone, "hardboard", ((au1 - 0.0016 if bone == "main" else au0 + 0.0016), 0.2525, awc), (0.0032, 0.495, 2 * aw), bone, "member", sheet=True))
        out.append(sbox(f"arm-top-{bone}", bone, "chipboard-p2", ((au0 + au1) / 2, 0.492, awc), (au1 - au0, 0.016, 2 * aw), bone, "member"))
        out.append(sbox(f"arm-bottom-{bone}", bone, "spruce-c24", ((au0 + au1) / 2, 0.027, awc), (au1 - au0 - 0.0064, 0.044, 2 * aw), bone, "member"))
        posts = [-aw + 0.022, aw - 0.022] + ([0.0] if bone == "chaise" else [])
        if scale == 100:
            posts = posts[:2]
        for i, pw in enumerate(posts):
            out.append(sbox(f"arm-post-{bone}-{i + 1}", bone, "spruce-c24", ((au0 + au1) / 2, 0.2665, pw), (au1 - au0 - 0.0064, 0.435, 0.044), bone, "member"))
    else:
        out.append(sbox(f"arm-block-{bone}", bone, "spruce-c24", ((au0 + au1) / 2, 0.2525, awc), (au1 - au0, 0.495, 2 * aw), bone, "member"))
    bu0, bu1 = (ARM_W, MAIN_L) if bone == "main" else (0.0, CH_SEAT_W)
    bw = 0.0 if bone == "main" else -CH_Z
    if scale in (50, 100):
        out.append(sbox(f"back-rail-low-{bone}", bone, "spruce-c24", ((bu0 + bu1) / 2, 0.222, 0.41 + bw), (bu1 - bu0, 0.044, 0.10), bone, "member"))
        out.append(sbox(f"back-rail-top-{bone}", bone, "spruce-c24", ((bu0 + bu1) / 2, 0.572, 0.425 + bw), (bu1 - bu0, 0.044, 0.07), bone, "member"))
        ups = [bu0 + 0.022, (bu0 + bu1) / 2, bu1 - 0.022] if scale == 50 else [bu0 + 0.022, bu1 - 0.022]
        for i, uu in enumerate(ups):
            out.append(sbox(f"back-upright-{bone}-{i + 1}", bone, "spruce-c24", (uu, 0.397, 0.425 + bw), (0.044, 0.306, 0.07), bone, "member"))
        if scale == 50:
            out.append(sbox(f"back-panel-{bone}", bone, "chipboard-p2", ((bu0 + bu1) / 2, 0.4025, 0.465 + bw), (bu1 - bu0 - 0.02, 0.395, 0.01), bone, "member"))
    elif scale == 200:
        out.append(sbox(f"back-rail-low-{bone}", bone, "spruce-c24", ((bu0 + bu1) / 2, 0.222, 0.41 + bw), (bu1 - bu0, 0.044, 0.10), bone, "member", slender=(bu1 - bu0) / 0.10 >= 5))
        out.append(sbox(f"back-rail-top-{bone}", bone, "spruce-c24", ((bu0 + bu1) / 2, 0.572, 0.425 + bw), (bu1 - bu0, 0.044, 0.07), bone, "member", slender=(bu1 - bu0) / 0.07 >= 5))
        for i, uu in enumerate([bu0 + 0.022, bu1 - 0.022]):
            out.append(sbox(f"back-upright-{bone}-{i + 1}", bone, "spruce-c24", (uu, 0.397, 0.425 + bw), (0.044, 0.306, 0.07), bone, "member"))
    else:
        out.append(sbox(f"back-panel-{bone}", bone, "spruce-c24", ((bu0 + bu1) / 2, 0.40, 0.4325 + bw), (bu1 - bu0, 0.40, 0.075), bone, "member"))
    return out


def part_joints(scale, bone):
    """🔩️ The joints of one part: arm to base, back frame to base and the open sofa connector at the part joint."""
    f = FEATURE[scale]
    j = max(f, 0.03)
    if bone == "main":
        arm_side, conn_u, bw = ARM_W - 0.006, MAIN_L - 0.0055, 0.0
    else:
        arm_side, conn_u, bw = CH_SEAT_W + 0.006, 0.0055, -CH_Z
    out = [
        sbox(f"joint-arm-{bone}", bone, "spruce-c24", (arm_side, 0.10, -0.40 if bone == "main" else -0.875), (j, max(0.10, j), j), bone, "joint", role="detail", tone=0.25),
        sbox(f"joint-back-{bone}", bone, "spruce-c24", (((ARM_W + MAIN_L) / 2) if bone == "main" else CH_SEAT_W / 2, 0.20, 0.41 + bw), (j, j, j), bone, "joint", role="detail", tone=0.25),
    ]
    if scale == 50:
        out.append(sbox(f"joint-connector-{bone}", bone, "spring-steel", (conn_u + (0.0045 if bone == "main" else -0.0045), 0.11, 0.30 + bw), (0.0025, 0.06, 0.14), bone, "joint", role="detail", tone=0.2, sheet=True))
    return out


def cushion_members(scale, pieces):
    """🧽️ The foam slab of every cushion bone, on its own bone: what carries an occupant down to the springs."""
    out = []
    by = {p.id: p for p in pieces}
    for bone, length, w0 in (("seat-1", 0.735, SEAT_W0), ("seat-2", 0.735, SEAT_W0), ("chaise-seat", CH_SEAT_W, CH_SEAT_W0)):
        depth = SEAT_W1 - w0
        out.append(sbox(f"slab-{bone}", bone, "pu-foam", (length / 2, 0.06, 0.0), (length - 0.04, 0.12, depth - 0.04), bone, "member", tone=0.35))
    for bone, length in (("back-1", 0.735), ("back-2", 0.735), ("chaise-back", CH_SEAT_W)):
        c, s = math.cos(math.radians(BACK_LEAN)), math.sin(math.radians(BACK_LEAN))
        a, b = BACK_A - 0.03, BACK_B - 0.025
        corners = [(x * c - y * s, x * s + y * c) for x, y in [(a, b), (-a, b), (-a, -b), (a, -b)]]
        vmin = min(p[0] for p in corners)
        prof = [(p[0] - vmin, p[1] + BACKFIT["wc"] - BACK_Z[bone]) for p in corners]
        d = solid_base(f"slab-{bone}", bone, "fibre-fill", 0.35)
        d.update({"kind": "extrude", "plane": "yz", "profile": pts5(prof), "from": 0.02, "to": r5(length - 0.02), "member": bone, "part": "member"})
        out.append(d)
    return out


def feet_members(pieces):
    """🦶️ Every foot is its own member: the turned body solid."""
    return [{"id": f"m-foot-{k + 1}", "of": f"foot-{k + 1}", "member": f"foot-{k + 1}", "part": "member"} for k in range(8)]

# ------------------------------------------------------------------ anchors

REST = {"main": [(0.06, 0.43), (0.06, -0.43), (1.59, 0.43), (1.59, -0.43)], "chaise": [(0.06, 0.90), (0.89, 0.90), (0.06, -0.90), (0.89, -0.90)]}
GRIP = {"main": [(0.03, 0.35), (0.03, -0.35), (1.62, 0.35), (1.62, -0.35)], "chaise": [(0.15, 0.92), (0.62, 0.92), (0.15, -0.92), (0.62, -0.92)]}
REST_IDS = [f"{b}-rest-{i + 1}" for b in ("main", "chaise") for i in range(4)]
GRIP_IDS = [f"{b}-grip-{i + 1}" for b in ("main", "chaise") for i in range(4)]
FOOT_IDS = [f"foot-{k + 1}-floor" for k in range(8)]
LOAD_POINTS = {"seat-1": (0.3675, 0.12), "seat-2": (0.3675, 0.12), "chaise-seat": (0.385, 0.35)}


def anchors():
    """📍️ Rest contacts on the blankets, carrying grips, floor contacts of the feet and the three sitting points."""
    out = []
    for bone in ("main", "chaise"):
        for i, (u, w) in enumerate(REST[bone]):
            out.append({"id": f"{bone}-rest-{i + 1}", "bone": bone, "x": u, "y": 0, "z": w, "kind": "contact", "title": T(f"Base corner {i + 1} of the {'two-seat part' if bone == 'main' else 'chaise'} on its blanket", f"Sockelecke {i + 1} des {'Zweisitzerteils' if bone == 'main' else 'Longchairs'} auf seiner Decke"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.06, "footprintWidthM": 0.06, "frictionCoefficient": 0.5})
    for bone in ("main", "chaise"):
        for i, (u, w) in enumerate(GRIP[bone]):
            out.append({"id": f"{bone}-grip-{i + 1}", "bone": bone, "x": u, "y": 0, "z": w, "kind": "support", "title": T(f"Hand grip {i + 1} under the {'two-seat part' if bone == 'main' else 'chaise'}", f"Handgriff {i + 1} unter dem {'Zweisitzerteil' if bone == 'main' else 'Longchair'}"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.09, "footprintWidthM": 0.08, "frictionCoefficient": 0.8})
    for k in range(8):
        out.append({"id": f"foot-{k + 1}-floor", "bone": f"foot-{k + 1}", "x": 0, "y": 0, "z": 0, "kind": "contact", "title": T(f"Foot {k + 1} on the floor", f"Fuß {k + 1} auf dem Boden"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.043, "footprintWidthM": 0.043, "frictionCoefficient": 0.4})
    for i, bone in enumerate(("seat-1", "seat-2", "chaise-seat")):
        u, w = LOAD_POINTS[bone]
        out.append({"id": f"seat-load-{i + 1}", "bone": bone, "x": u, "y": r5(SEAT_T - SAG[bone]), "z": w if bone == "chaise-seat" else 0.12, "kind": "load", "title": T(f"Sitting point {i + 1}", f"Sitzpunkt {i + 1}")})
    return out


# ------------------------------------------------------------------- masses


def volume_of(solid):
    """📦️ Approximate volume of a solid for the mass budget (zigzag springs count 2.2 times their span of wire)."""
    k = solid["kind"]
    if k == "box":
        return solid["size"][0] * solid["size"][1] * solid["size"][2]
    if k == "capsule":
        length = math.dist(solid["p0"], solid["p1"])
        return math.pi * solid["r0"] ** 2 * length * 2.2
    if k == "extrude":
        a = abs(G.area(solid["profile"])) - sum(abs(G.area(h)) for h in solid.get("holes", []))
        return a * (solid["to"] - solid["from"])
    if k == "revolve":
        pr = solid["profile"]
        return sum(math.pi * (pr[i][1] ** 2 + pr[i][1] * pr[i + 1][1] + pr[i + 1][1] ** 2) / 3 * abs(pr[i + 1][0] - pr[i][0]) for i in range(len(pr) - 1))
    if k == "loft":
        secs = solid["sections"]

        def sec_area(s):
            n = s["exponent"]
            return 4 * s["rx"] * s["ry"] * math.gamma(1 + 1 / n) ** 2 / math.gamma(1 + 2 / n)

        return sum((sec_area(secs[i]) + sec_area(secs[i + 1])) / 2 * (secs[i + 1]["at"][0] - secs[i]["at"][0]) for i in range(len(secs) - 1))
    raise ValueError(k)


def centre_of(solid):
    """🎯️ Bone-local centre of a solid for the mass budget."""
    k = solid["kind"]
    if k == "box":
        return solid["transform"]["translation"]
    if k == "capsule":
        return [(a + b) / 2 for a, b in zip(solid["p0"], solid["p1"])]
    if k == "extrude":
        pr = solid["profile"]
        m = [sum(p[0] for p in pr) / len(pr), sum(p[1] for p in pr) / len(pr)]
        mid = (solid["from"] + solid["to"]) / 2
        return {"yz": [mid, m[0], m[1]], "zx": [m[1], mid, m[0]], "xy": [m[0], m[1], mid]}[solid["plane"]]
    if k == "revolve":
        return [max(p[0] for p in solid["profile"]) / 2, 0.0, 0.0]
    if k == "loft":
        secs = solid["sections"]
        return [sum(s["at"][i] for s in secs) / len(secs) for i in range(3)]
    raise ValueError(k)


def surface_of(piece):
    """🧮️ Outer surface of a body piece (rounded boxes, prisms, lofts, revolves, the open bag) for areas and cover masses."""
    s = piece.solid
    k = s["kind"]
    if k == "box":
        a, b, c = s["size"]
        r = s.get("cornerRadius", 0.0)
        return 2 * ((a - 2 * r) * (b - 2 * r) + (b - 2 * r) * (c - 2 * r) + (c - 2 * r) * (a - 2 * r)) + 2 * math.pi * r * ((a - 2 * r) + (b - 2 * r) + (c - 2 * r)) + 4 * math.pi * r * r
    if k == "extrude":
        pr = s["profile"]
        per = sum(math.dist(pr[i - 1], pr[i]) for i in range(len(pr)))
        length = s["to"] - s["from"]
        if s.get("holes"):
            h = s["holes"][0]
            hper = sum(math.dist(h[i - 1], h[i]) for i in range(len(h)))
            ring = abs(G.area(pr)) - abs(G.area(h))
            return per * length + hper * length + ring
        return per * length + 2 * abs(G.area(pr))
    if k == "loft":
        secs = s["sections"]
        pers = []
        for sc in secs:
            pts = G.section_points(sc, 72)
            pers.append(sum(math.dist(pts[i - 1], pts[i]) for i in range(len(pts))))
        side = sum((pers[i] + pers[i + 1]) / 2 * (secs[i + 1]["at"][0] - secs[i]["at"][0]) for i in range(len(secs) - 1))

        def sec_area(q):
            n = q["exponent"]
            return 4 * q["rx"] * q["ry"] * math.gamma(1 + 1 / n) ** 2 / math.gamma(1 + 2 / n)

        return side + sec_area(secs[0]) + sec_area(secs[-1])
    if k == "revolve":
        pr = s["profile"]
        return sum(math.pi * (pr[i][1] + pr[i + 1][1]) * math.dist(pr[i], pr[i + 1]) for i in range(len(pr) - 1))
    raise ValueError(k)


COVER_KG_M2 = 0.40
PADDING_KG_M3 = 21.0


def masses():
    """⚖️ Bone masses from the 1:50 members (section x length x density), foam and fibre volumes and the cover fabric, with their centres."""
    pieces = body(50)
    by = {p.id: p for p in pieces}
    out = {}
    for bone in ("main", "chaise"):
        parts = [(volume_of(m) * DENSITY[m["material"]], centre_of(m)) for m in part_frame(50, bone) + [j for j in part_joints(50, bone) if j["material"] != "spring-steel"]]
        own = [p for p in pieces if p.bone == bone]
        for p in own:
            vol = volume_of(p.solid)
            members_in = sum(volume_of(m) for m in part_frame(50, bone)) * vol / sum(volume_of(q.solid) for q in own)
            parts.append((max(vol - members_in, 0.0) * PADDING_KG_M3 + surface_of(p) * COVER_KG_M2, centre_of(p.solid)))
        out[bone] = parts
    for bone in ("seat-1", "seat-2", "chaise-seat"):
        p = by[bone]
        out[bone] = [(volume_of(p.solid) * DENSITY["pu-foam"] + surface_of(p) * COVER_KG_M2, centre_of(p.solid))]
    for bone in ("back-1", "back-2", "chaise-back"):
        p = by[bone]
        out[bone] = [(volume_of(p.solid) * DENSITY["fibre-fill"] + surface_of(p) * COVER_KG_M2, centre_of(p.solid))]
    for k in range(8):
        p = by[f"foot-{k + 1}"]
        out[p.bone] = [(volume_of(p.solid) * DENSITY["beech-lacquered"] + 0.025, centre_of(p.solid))]
    for bid in ("blanket-main", "blanket-chaise"):
        p = by[bid]
        out[bid] = [(volume_of(p.solid) * DENSITY["moving-blanket"], centre_of(p.solid))]
    out["tote"] = [(sum(volume_of(by[i].solid) for i in ("bag-walls", "bag-bottom")) * DENSITY["pp-woven"], [0.0, 0.03, 0.0])]
    result = {}
    for bone, parts in out.items():
        kg = sum(m for m, _ in parts)
        com = [sum(m * c[i] for m, c in parts) / kg for i in range(3)]
        result[bone] = (kg, com)
    return result


MASSES = masses()


def mass_markers(scale):
    """🟣️ A compact marker at the centre of mass of every bone with mass, the feature in diameter."""
    rr = max(FEATURE[scale] / 2, 0.02)
    out = []
    for bone, (kg, com) in MASSES.items():
        r = FEATURE[scale] / 2 if bone.startswith("blanket") else rr
        d = solid_base(f"mass-{bone}", bone, {"main": "chipboard-p2", "chaise": "chipboard-p2", "tote": "pp-woven"}.get(bone, "pu-foam" if "seat" in bone else "fibre-fill" if "back" in bone else "beech-lacquered" if bone.startswith("foot") else "moving-blanket"), 0.3, "detail")
        d.update({"kind": "ellipsoid", "radii": [r, r, r], "transform": {"translation": [r5(c) for c in com]}, "member": bone, "part": "mass"})
        out.append(d)
    return out


def structure_list(scale, pieces):
    """🏗️ The structural idealisation of one scale: frames, springs, cushion slabs, feet, joints, supports and masses."""
    return part_frame(scale, "main") + part_frame(scale, "chaise") + part_joints(scale, "main") + part_joints(scale, "chaise") + cushion_members(scale, pieces) + feet_members(pieces) + mass_markers(scale)

# ---------------------------------------------------------------------- rig


def segment_mass(bone, reference):
    """⚖️ The authored point mass of a bone at its computed centre."""
    kg, com = MASSES[bone]
    return {"kg": round(kg, 3), "comX": r5(com[0]), "comY": r5(com[1]), "distribution": "point", "source": "authored-estimate", "reference": reference}


def rig():
    """🦴️ Floor reference, two blankets with the bag, the two parts on lift and slide joints, the five loose cushions, and eight feet that each travel on a straight path from the bag to their socket."""
    b = [
        {"id": "ground", "parent": None, "title": T("Floor reference", "Bodenbezug"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"}},
        {"id": "blanket-main", "parent": "ground", "title": T("Moving blanket under the two-seat part", "Möbeldecke unter dem Zweisitzerteil"), "x": BL_A["x0"], "y": 0, "z": BL_A["z"], "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 0, "minM": -2.0, "maxM": 0}, "mass": segment_mass("blanket-main", "semio-used-household")},
        {"id": "tote", "parent": "blanket-main", "title": T("Carrier bag with the unscrewed feet", "Tragetasche mit den abgeschraubten Füßen"), "x": r5(TOTE["x"] - BL_A["x0"]), "y": TA, "z": r5(TOTE["z"] - BL_A["z"]), "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"}, "mass": segment_mass("tote", "semio-used-household")},
        {"id": "blanket-chaise", "parent": "ground", "title": T("Folded moving blanket under the chaise", "Gefaltete Möbeldecke unter dem Longchair"), "x": BL_B["x0"], "y": 0, "z": BL_B["z"], "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 1.5}, "mass": segment_mass("blanket-chaise", "semio-used-household")},
        {"id": "main-lift", "parent": "ground", "title": T("Lift of the two-seat part", "Hub des Zweisitzerteils"), "x": 0, "y": TA, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.4}},
        {"id": "main", "parent": "main-lift", "title": T("Two-seat part: base, arm and back frame", "Zweisitzerteil: Sockel, Armlehne und Rückenrahmen"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": MAIN_L, "joint": {"type": "revolute", "minDeg": -8, "maxDeg": 8}, "mass": segment_mass("main", "en-312-2010")},
        {"id": "seat-1", "parent": "main", "title": T("Seat cushion 1 at the arm, sagged", "Sitzkissen 1 an der Armlehne, durchgesessen"), "x": ARM_W, "y": BASE_H, "z": r5(SEAT_WC), "angleDeg": 0, "lengthM": 0.735, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -0.08, "maxM": 0}, "mass": segment_mass("seat-1", "iso-3386-1")},
        {"id": "seat-2", "parent": "main", "title": T("Seat cushion 2", "Sitzkissen 2"), "x": ARM_W + 0.735, "y": BASE_H, "z": r5(SEAT_WC), "angleDeg": 0, "lengthM": 0.735, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -0.08, "maxM": 0}, "mass": segment_mass("seat-2", "iso-3386-1")},
        {"id": "back-1", "parent": "main", "title": T("Back cushion 1, slumped", "Rückenkissen 1, zusammengesackt"), "x": ARM_W, "y": BASE_H, "z": r5(BACKFIT["wc"]), "angleDeg": 0, "lengthM": 0.735, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -0.04, "maxM": 0}, "mass": segment_mass("back-1", "iso-10456-2007")},
        {"id": "back-2", "parent": "main", "title": T("Back cushion 2", "Rückenkissen 2"), "x": ARM_W + 0.735, "y": BASE_H, "z": r5(BACKFIT["wc"]), "angleDeg": 0, "lengthM": 0.735, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -0.04, "maxM": 0}, "mass": segment_mass("back-2", "iso-10456-2007")},
        {"id": "chaise-slide", "parent": "ground", "title": T("Slide of the chaise onto the two-seat part", "Verschiebung des Longchairs an das Zweisitzerteil"), "x": CH_X0, "y": TB, "z": CH_Z, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 0, "minM": -0.2, "maxM": 0}},
        {"id": "chaise-lift", "parent": "chaise-slide", "title": T("Lift of the chaise", "Hub des Longchairs"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.4}},
        {"id": "chaise", "parent": "chaise-lift", "title": T("Chaise part: base, outer arm and back frame", "Longchair: Sockel, äußere Armlehne und Rückenrahmen"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": CH_W, "joint": {"type": "fixed"}, "mass": segment_mass("chaise", "en-312-2010")},
        {"id": "chaise-seat", "parent": "chaise", "title": T("Chaise seat cushion, sagged", "Longchair-Sitzkissen, durchgesessen"), "x": 0, "y": BASE_H, "z": r5(CH_SEAT_WC - CH_Z), "angleDeg": 0, "lengthM": CH_SEAT_W, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -0.08, "maxM": 0}, "mass": segment_mass("chaise-seat", "iso-3386-1")},
        {"id": "chaise-back", "parent": "chaise", "title": T("Chaise back cushion, slumped", "Longchair-Rückenkissen, zusammengesackt"), "x": 0, "y": BASE_H, "z": r5(BACKFIT["wc"] - CH_Z), "angleDeg": 0, "lengthM": CH_SEAT_W, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -0.04, "maxM": 0}, "mass": segment_mass("chaise-back", "iso-10456-2007")},
    ]
    for k, p in enumerate(PATHS):
        b.append({"id": f"foot-path-{k + 1}", "parent": "ground", "title": T(f"Path of foot {k + 1} from the bag to its socket", f"Weg von Fuß {k + 1} aus der Tasche zu seiner Aufnahme"), "x": r5(p["B"][0]), "y": r5(p["B"][1]), "z": r5(p["B"][2]), "angleDeg": 0, "twistDeg": p["tau"], "lengthM": 0, "joint": {"type": "fixed"}})
        b.append({"id": f"foot-{k + 1}", "parent": f"foot-path-{k + 1}", "title": T(f"Foot {k + 1}, unscrewed", f"Fuß {k + 1}, abgeschraubt"), "x": 0, "y": 0, "angleDeg": 90, "abductionDeg": p["tau"], "lengthM": FOOT_H, "joint": {"type": "prismatic", "axisDeg": p["theta"], "minM": 0, "maxM": round(p["L"] + 0.001, 4)}, "mass": segment_mass(f"foot-{k + 1}", "wood-handbook-2021")})
    return {"kind": "structural", "root": "ground", "bones": b}


# -------------------------------------------------------------------- poses

AREA_KEYS = ["seat-cushions", "back-cushions", "arms", "frame-cover", "feet", "blankets", "bag"]


def binding(areas, sun=None, contact=None):
    """🌬️ A thermal binding of every region."""
    sun = sun or {}
    contact = contact or {}
    regs = {}
    for rid in AREA_KEYS:
        b = {"exposedFraction": round(1 - contact.get(rid, (0, 0))[0], 3), "sunlitFraction": sun.get(rid, 0.0)}
        if rid in contact:
            b["contact"] = {"fraction": contact[rid][0], "tempC": contact[rid][1]}
        regs[rid] = b
    return {"regions": regs, "emitters": {}}


def use_offsets(seated):
    """🛋️ The bone offsets of the reassembled sofa in a home: parts on their feet and joined, blankets and bag put aside, cushions sunk under the seated occupants."""
    o = {"main-lift": {"dy": r5(FOOT_H - TA)}, "chaise-slide": {"dx": CH_SLIDE}, "chaise-lift": {"dy": r5(FOOT_H - TB)}, "blanket-main": {"dx": BL_A_MOVE}, "blanket-chaise": {"dx": BL_B_MOVE}}
    for k, p in enumerate(PATHS):
        c, s = math.cos(math.radians(p["theta"])), math.sin(math.radians(p["theta"]))
        o[f"foot-{k + 1}"] = {"dx": p["L"] * c, "dy": p["L"] * s}
    for bone in seated:
        o[bone] = {"dy": SINK[bone]}
        back = {"seat-1": "back-1", "seat-2": "back-2", "chaise-seat": "chaise-back"}[bone]
        o[back] = {"dy": SINK[back]}
    return o


def occupant(i):
    """🧍️ A seated adult of 75 kg on sitting point i (EN 1991-1-1 category A occupancy, EN 12520 domestic seating)."""
    return {"id": f"occupant-{i}", "kind": "occupant", "title": T(f"Seated adult {i} (75 kg)", f"Sitzende erwachsene Person {i} (75 kg)"), "anchor": f"seat-load-{i}", "fxN": 0, "fyN": -735.5, "source": "authored-estimate", "reference": "en-1991-1-1-2002"}


def poses(areas):
    """📥️ Stored on the blankets (default), carried in two parts, reassembled and in use with three occupants, the empty assembled sofa and the key poses of the clips."""
    stored_contact = {"blankets": (0.5, 19.0), "frame-cover": (round(((MAIN_L - ARM_W) * 0.95 + CH_SEAT_W * 1.9) / areas["frame-cover"], 3), 19.0), "arms": (round(ARM_W * (0.95 + 1.9) / areas["arms"], 3), 19.0), "bag": (round(TOTE["lx"] * TOTE["lz"] / areas["bag"], 3), 19.0), "feet": (0.07, 19.0)}
    carried_contact = {"blankets": (0.5, 19.0), "bag": stored_contact["bag"], "feet": (0.07, 19.0)}
    use_contact = {"blankets": (0.5, 20.0), "bag": stored_contact["bag"], "feet": (0.07, 20.0)}
    sun = {"arms": 0.3, "seat-cushions": 0.15, "back-cushions": 0.1, "frame-cover": 0.05}

    def seated(n):
        c = dict(use_contact)
        c["seat-cushions"] = (round(0.14 * n / areas["seat-cushions"], 3), 30.0)
        c["back-cushions"] = (round(0.08 * n / areas["back-cushions"], 3), 30.0)
        return c

    feet_contacts = FOOT_IDS
    return [
        {"id": "stored", "title": T("Stored on moving blankets", "Auf Möbeldecken eingelagert"), "description": T("At the flat clearance depot: the two parts stand 0.15 m apart on two moving blankets, the eight feet unscrewed and standing in a carrier bag on the blanket; no sun.", "Im Lager der Haushaltsauflösung: die beiden Teile stehen 0,15 m auseinander auf zwei Möbeldecken, die acht Füße abgeschraubt in einer Tragetasche auf der Decke; keine Sonne."), "bones": {}, "ground": "authored", "contacts": REST_IDS, "thermal": binding(areas, {}, stored_contact), "loads": []},
        {"id": "carried", "title": T("Carried in two parts", "In zwei Teilen getragen"), "description": T("Four movers lift both parts at their grips under the base: the two-seat part 0.25 m up and tilted 4 degrees with the arm end lower, the chaise 0.22 m up and level; blankets and bag stay on the floor.", "Vier Träger heben beide Teile an den Griffen unter dem Sockel: das Zweisitzerteil 0,25 m hoch und 4 Grad geneigt mit dem Armlehnenende tiefer, den Longchair 0,22 m hoch und waagrecht; Decken und Tasche bleiben am Boden."), "bones": {"main-lift": {"dy": LIFT["main"]}, "main": {"angleDeg": LIFT["tilt"]}, "chaise-lift": {"dy": LIFT["chaise"]}}, "ground": "authored", "contacts": GRIP_IDS, "thermal": binding(areas, {}, carried_contact), "loads": []},
        {"id": "in-use", "title": T("In use with three occupants", "In Gebrauch mit drei Personen"), "description": T("Reassembled in a living room on its eight feet, the chaise hooked onto the two-seat part; three adults sit on the worn seat at the arm, the second seat and the chaise; a window sun patch lies on the faded arms.", "In einem Wohnzimmer auf seinen acht Füßen wieder aufgebaut, der Longchair am Zweisitzerteil eingehängt; drei Erwachsene sitzen auf dem durchgesessenen Platz an der Armlehne, dem zweiten Platz und dem Longchair; ein Sonnenfleck vom Fenster liegt auf den ausgebleichten Armlehnen."), "bones": use_offsets(("seat-1", "seat-2", "chaise-seat")), "ground": "authored", "contacts": feet_contacts, "thermal": binding(areas, sun, seated(3)), "loads": [occupant(1), occupant(2), occupant(3)]},
        {"id": "in-use-empty", "title": T("Reassembled, empty", "Wieder aufgebaut, leer"), "description": T("The sofa on its feet in the living room, nobody seated: the sag of the seat at the arm and the slumped back cushions show unloaded.", "Das Sofa auf seinen Füßen im Wohnzimmer, niemand sitzt: die Mulde im Platz an der Armlehne und die zusammengesackten Rückenkissen zeigen sich unbelastet."), "bones": use_offsets(()), "ground": "authored", "contacts": feet_contacts, "thermal": binding(areas, sun, use_contact), "loads": []},
        {"id": "lift-off-key", "key": True, "title": T("Lift-off", "Abheben"), "description": T("The movers have taken hold: both parts 30 mm above their blankets.", "Die Träger haben zugefasst: beide Teile 30 mm über ihren Decken."), "bones": {"main-lift": {"dy": 0.03}, "chaise-lift": {"dy": 0.03}}, "ground": "authored", "contacts": GRIP_IDS, "thermal": binding(areas, {}, carried_contact), "loads": []},
        {"id": "one-seated-key", "key": True, "title": T("First occupant seated", "Erste Person sitzt"), "description": T("One adult on the worn seat at the arm.", "Eine erwachsene Person auf dem durchgesessenen Platz an der Armlehne."), "bones": use_offsets(("seat-1",)), "ground": "authored", "contacts": feet_contacts, "thermal": binding(areas, sun, seated(1)), "loads": [occupant(1)]},
        {"id": "two-seated-key", "key": True, "title": T("Second occupant on the chaise", "Zweite Person auf dem Longchair"), "description": T("A second adult sits on the chaise.", "Eine zweite erwachsene Person setzt sich auf den Longchair."), "bones": use_offsets(("seat-1", "chaise-seat")), "ground": "authored", "contacts": feet_contacts, "thermal": binding(areas, sun, seated(2)), "loads": [occupant(1), occupant(3)]},
    ]


def clips():
    """🎞️ `carry`: the parts are lifted off their blankets; `sit-down-three`: three people sit down one after another and the worn cushions sink."""
    carry = {"id": "carry", "title": T("Carry away", "Abtragen"), "description": T("The movers take hold under both parts, lift them off the blankets and carry them, the two-seat part tilting 4 degrees as its arm end is lowered first.", "Die Träger fassen unter beide Teile, heben sie von den Decken und tragen sie, das Zweisitzerteil neigt sich um 4 Grad, weil sein Armlehnenende zuerst gesenkt wird."), "durationS": 3.0, "fps": 25, "loop": False, "rootMotion": "none",
             "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 0.8, "pose": "lift-off-key", "ease": "smooth"}, {"t": 2.2, "pose": "carried", "ease": "smooth"}, {"t": 3.0, "pose": "carried", "ease": "smooth"}],
             "contacts": [{"anchor": a, "fromS": 0, "toS": 0.8} for a in REST_IDS] + [{"anchor": a, "fromS": 0.6, "toS": 3.0} for a in GRIP_IDS],
             "phases": [{"id": "take-hold", "fromS": 0, "toS": 0.8, "title": T("Take hold under the base", "Unter den Sockel fassen")}, {"id": "lift", "fromS": 0.8, "toS": 2.2, "title": T("Lift and tilt", "Anheben und neigen")}, {"id": "carry", "fromS": 2.2, "toS": 3.0, "title": T("Carry", "Tragen")}],
             "events": [{"t": 0.8, "kind": "contact-off", "title": T("Both parts leave their blankets", "Beide Teile verlassen ihre Decken")}, {"t": 2.2, "kind": "motion", "title": T("Carrying height reached", "Traghöhe erreicht")}], "tracks": []}
    sit = {"id": "sit-down-three", "title": T("Three people sit down", "Drei Personen setzen sich"), "description": T("One adult drops onto the worn seat at the arm, a second onto the chaise, a third onto the second seat; each cushion sinks into its tired foam.", "Eine erwachsene Person lässt sich auf den durchgesessenen Platz an der Armlehne fallen, eine zweite auf den Longchair, eine dritte auf den zweiten Platz; jedes Kissen sinkt in seinen müden Schaum."), "durationS": 3.6, "fps": 25, "loop": False, "rootMotion": "none",
           "keys": [{"t": 0, "pose": "in-use-empty", "ease": "smooth"}, {"t": 0.9, "pose": "one-seated-key", "ease": "smooth"}, {"t": 1.8, "pose": "two-seated-key", "ease": "smooth"}, {"t": 2.7, "pose": "in-use", "ease": "smooth"}, {"t": 3.6, "pose": "in-use", "ease": "smooth"}],
           "contacts": [{"anchor": a, "fromS": 0, "toS": 3.6} for a in FOOT_IDS],
           "phases": [{"id": "first", "fromS": 0, "toS": 0.9, "title": T("First occupant", "Erste Person")}, {"id": "second", "fromS": 0.9, "toS": 1.8, "title": T("Second occupant", "Zweite Person")}, {"id": "third", "fromS": 1.8, "toS": 3.6, "title": T("Third occupant and settling", "Dritte Person und Einsinken")}],
           "events": [{"t": 0.9, "kind": "load-on", "title": T("First occupant seated", "Erste Person sitzt"), "target": "occupant-1"}, {"t": 1.8, "kind": "load-on", "title": T("Second occupant seated", "Zweite Person sitzt"), "target": "occupant-3"}, {"t": 2.7, "kind": "load-on", "title": T("Third occupant seated", "Dritte Person sitzt"), "target": "occupant-2"}], "tracks": []}
    return [carry, sit]

# ---------------------------------------------------------- representations

PENS = {50: {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": 0.18, "ground": 0.7}, 100: {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": 0.13, "ground": 0.5}, 200: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": 0.13, "ground": 0.35}, 500: {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25}}
HATCHES = {50: [{"id": "quilting", "title": T("Quilting of the moving blankets", "Steppung der Möbeldecken"), "pattern": "cross", "angleDeg": 45, "spacingMm": 1.6, "penMm": 0.13}], 100: [{"id": "quilting", "title": T("Quilting of the moving blankets", "Steppung der Möbeldecken"), "pattern": "cross", "angleDeg": 45, "spacingMm": 1.2, "penMm": 0.1}], 200: [], 500: []}

REP_DECISIONS = {
    50: {
        "silhouette": T("Corner sofa set 57.5 x 15.3 mm (stored on its blankets) drawn from the 1:50 body: the two-seat part with its box arm, base, rolled back frame, two sagging seat cushions and two slumped back cushions; the chaise with its long outer arm, base, back frame, the 1.52 m seat cushion and its back cushion; two moving blankets and the carrier bag with the eight unscrewed feet. Front view from the chaise end, plan as an L of two parts 0.15 m apart.", "Ecksofa-Satz 57,5 x 15,3 mm (auf seinen Decken eingelagert) aus dem 1:50-Körper gezeichnet: das Zweisitzerteil mit Kastenarmlehne, Sockel, gerolltem Rückenrahmen, zwei durchgesessenen Sitzkissen und zwei zusammengesackten Rückenkissen; der Longchair mit langer äußerer Armlehne, Sockel, Rückenrahmen, dem 1,52 m langen Sitzkissen und seinem Rückenkissen; zwei Möbeldecken und die Tragetasche mit den acht abgeschraubten Füßen. Vorderansicht vom Longchair-Ende, Grundriss als L aus zwei Teilen mit 0,15 m Abstand."),
        "retained": T("The sag of the seat at the arm (35 mm) and of the chaise (25 mm), the 15 mm dip of the second seat, the 15 degree slump and the dipped tops of the back cushions, the 40 mm arm roundings, the leaning and rolled back frame, the turned feet with their ring, the 8 and 16 mm blankets.", "Die Mulde im Platz an der Armlehne (35 mm) und im Longchair (25 mm), die 15-mm-Mulde des zweiten Platzes, das 15-Grad-Zusammensacken und die eingedellten Oberkanten der Rückenkissen, die 40-mm-Rundungen der Armlehnen, der geneigte und gerollte Rückenrahmen, die gedrechselten Füße mit Ring, die 8- und 16-mm-Decken."),
        "merged": T("Cover, padding and boards of each upholstered part are one outline; piping is a stroke.", "Bezug, Polsterung und Platten jedes Polsterteils sind ein Umriss; Keder ist ein Strich."),
        "omitted": T("Zips, stitching of the quilting (drawn as the hatch), the threaded bolts of the feet, the folded bag handles.", "Reißverschlüsse, Steppnähte (als Schraffur gezeichnet), die Gewindebolzen der Füße, die eingeschlagenen Taschengriffe."),
        "thinElements": T("Blankets 8 and 16 mm and the 1 mm bag walls at true thickness (sheets).", "Decken 8 und 16 mm und die 1-mm-Taschenwände in wahrer Dicke (Flächen)."),
        "hatching": T("Plan: cross hatch of the quilted blankets, 80 mm diamonds.", "Grundriss: Kreuzschraffur der gesteppten Decken, 80-mm-Rauten."),
        "styles": T("Faded arms light (0.82), seat cushions 0.72, back cushions 0.62, frame 0.45 to 0.55, feet 0.35, blankets dark (0.28 to 0.3).", "Ausgebleichte Armlehnen hell (0,82), Sitzkissen 0,72, Rückenkissen 0,62, Rahmen 0,45 bis 0,55, Füße 0,35, Decken dunkel (0,28 bis 0,3)."),
        "overlays": T("Traces of use as strokes: piping following the sag, slump creases, the fading boundary of the arms, the coffee ring on the chaise arm, the sag contour in plan, the depot price sticker on the chaise front.", "Gebrauchsspuren als Striche: Keder entlang der Mulde, Knickfalten, die Ausbleichgrenze der Armlehnen, der Kaffeering auf der Longchair-Armlehne, die Muldenkontur im Grundriss, das Preisetikett des Lagers an der Longchair-Front."),
    },
    100: {
        "silhouette": T("The set 28.8 x 7.7 mm from the 1:100 body: both parts with arms, bases, back frames and the five cushions with simpler roundings, blankets, bag and feet.", "Der Satz 28,8 x 7,7 mm aus dem 1:100-Körper: beide Teile mit Armlehnen, Sockeln, Rückenrahmen und den fünf Kissen mit einfacheren Rundungen, Decken, Tasche und Füße."),
        "retained": T("Sag of the seat at the arm and of the chaise, slumped back cushions with their dips, arm roundings, rolled back frame, tapered feet.", "Mulde des Platzes an der Armlehne und des Longchairs, zusammengesackte Rückenkissen mit Delle, Armlehnenrundungen, gerollter Rückenrahmen, konische Füße."),
        "merged": T("Roundings in two steps, the second seat's 15 mm dip lost below the 15 mm feature.", "Rundungen in zwei Stufen, die 15-mm-Mulde des zweiten Platzes geht unter dem 15-mm-Merkmal auf."),
        "omitted": T("Foot ring, slump creases, coffee ring, price sticker, bag hem.", "Fußring, Knickfalten, Kaffeering, Preisetikett, Taschensaum."),
        "thinElements": T("Blankets and bag walls stay at true thickness as sheets.", "Decken und Taschenwände bleiben als Flächen in wahrer Dicke."),
        "hatching": T("Plan: cross hatch of the blankets.", "Grundriss: Kreuzschraffur der Decken."),
        "styles": T("Tones as at 1:50.", "Töne wie bei 1:50."),
        "overlays": T("Piping and the fading boundary of the arms as strokes.", "Keder und die Ausbleichgrenze der Armlehnen als Striche."),
    },
    200: {
        "silhouette": T("The set 14.4 x 3.8 mm from the 1:200 body: arms with a 30 mm rounding, bases, chamfered back frames, cushions in few stations, blankets, bag and cylindrical feet.", "Der Satz 14,4 x 3,8 mm aus dem 1:200-Körper: Armlehnen mit 30-mm-Rundung, Sockel, abgeschrägte Rückenrahmen, Kissen in wenigen Stationen, Decken, Tasche und zylindrische Füße."),
        "retained": T("The 35 mm sag at the arm, the slump of the back cushions, the cushion division, the gap between the parts.", "Die 35-mm-Mulde an der Armlehne, das Zusammensacken der Rückenkissen, die Kissenteilung, der Abstand der Teile."),
        "merged": T("Roundings under 30 mm become one chamfer step; the chaise sag and the dips of the back cushions below 30 mm are straight.", "Rundungen unter 30 mm werden eine Fasenstufe; die Longchair-Mulde und die Dellen der Rückenkissen unter 30 mm sind gerade."),
        "omitted": T("All strokes of use; hatch.", "Alle Gebrauchsspuren als Striche; Schraffur."),
        "thinElements": T("Blankets and bag walls as sheets, feet 48 to 58 mm at true width.", "Decken und Taschenwände als Flächen, Füße 48 bis 58 mm in wahrer Breite."),
        "hatching": T("None.", "Keine."),
        "styles": T("Fabric tones 0.45 to 0.82, blankets 0.28.", "Stofftöne 0,45 bis 0,82, Decken 0,28."),
        "overlays": T("Each part carries its region.", "Jedes Teil trägt seine Region."),
    },
    500: {
        "silhouette": T("One composed sign 5.8 x 1.5 mm from the 1:500 body: blocks of arms, bases, back frames and cushions, the blankets, the bag and 75 mm feet.", "Ein zusammengesetztes Zeichen 5,8 x 1,5 mm aus dem 1:500-Körper: Blöcke der Armlehnen, Sockel, Rückenrahmen und Kissen, die Decken, die Tasche und 75-mm-Füße."),
        "retained": T("Arm tops at 0.52 m above the base bottom, back tops, the leaning back cushions, the two parts and the L.", "Armlehnenoberkanten 0,52 m über der Sockelunterkante, Rückenoberkanten, die geneigten Rückenkissen, die zwei Teile und das L."),
        "merged": T("Cushions are plain blocks, the back frame a chamfered block.", "Kissen sind einfache Blöcke, der Rückenrahmen ein gefaster Block."),
        "omitted": T("Sags, dips, roundings, all strokes.", "Mulden, Dellen, Rundungen, alle Striche."),
        "thinElements": T("Feet (43 to 60 mm) are drawn at the 75 mm feature; blankets and bag walls stay sheets.", "Füße (43 bis 60 mm) werden im 75-mm-Merkmal gezeichnet; Decken und Taschenwände bleiben Flächen."),
        "hatching": T("None.", "Keine."),
        "styles": T("Arms 0.82, cushions 0.62 to 0.72, frame 0.45 to 0.55, blankets 0.28.", "Armlehnen 0,82, Kissen 0,62 bis 0,72, Rahmen 0,45 bis 0,55, Decken 0,28."),
        "overlays": T("Each block carries its region.", "Jeder Block trägt seine Region."),
    },
}

VOL_DECISIONS = {
    50: [T("Body, authored for 7.5 mm: two-seat part (box arm 0.18 x 0.52 x 0.95 m rounded 40 mm, base 1.47 x 0.20 x 0.95 m, back frame with a 6 degree front face and rolled top to 0.62 m, two seat cushion lofts of 13 stations with sags of 35 and 15 mm, two back cushion lofts leaning 15 degrees with dips of 30 and 20 mm), chaise (outer arm 1.90 m, base, back frame, a 0.77 x 1.52 m seat cushion with a 25 mm sag, back cushion), two quilted blankets (8 mm, folded 16 mm) and the open carrier bag (1 mm walls, 2 mm bottom) with eight turned feet standing in it.", "Körper für 7,5 mm: Zweisitzerteil (Kastenarmlehne 0,18 x 0,52 x 0,95 m mit 40 mm Rundung, Sockel 1,47 x 0,20 x 0,95 m, Rückenrahmen mit 6 Grad geneigter Front und gerollter Oberkante bis 0,62 m, zwei Sitzkissen-Lofts aus 13 Stationen mit 35 und 15 mm Mulde, zwei um 15 Grad geneigte Rückenkissen-Lofts mit 30 und 20 mm Delle), Longchair (äußere Armlehne 1,90 m, Sockel, Rückenrahmen, ein 0,77 x 1,52 m Sitzkissen mit 25 mm Mulde, Rückenkissen), zwei gesteppte Decken (8 mm, gefaltet 16 mm) und die offene Tragetasche (1 mm Wände, 2 mm Boden) mit acht gedrechselten Füßen darin."),
          T("Thermal: every body solid is the surface of its region (one layer each: foam core, fibre fill, arm padding, chipboard, beech, quilt, PP film). Structure: chipboard base rails 18 x 195 mm, end panels, a spruce cross rail, eight and seven Nosag springs (three under the worn seat and three under the chaise bent 15 to 20 mm), hardboard arm panels on spruce posts and rails, back frame rails, uprights and panel, foam and fibre slabs of the cushions, eight foot members, arm, back and connector joints, a pad at every rest, grip and foot anchor, mass markers.", "Thermik: jeder Körper ist die Oberfläche seiner Region (je eine Schicht: Schaumkern, Faserfüllung, Armpolster, Spanplatte, Buche, Steppdecke, PP-Gewebe). Tragwerk: Spanplatten-Sockelholme 18 x 195 mm, Stirnplatten, ein Fichten-Querholm, acht und sieben Wellenfedern (drei unter dem durchgesessenen Platz und drei unter dem Longchair um 15 bis 20 mm durchgebogen), Hartfaser-Armwangen auf Fichtenpfosten und -holmen, Rückenrahmenholme, -stiele und -platte, Schaum- und Faserplatten der Kissen, acht Fußglieder, Arm-, Rücken- und Verbinderknoten, ein Auflager an jedem Lager-, Griff- und Fußanker, Massenmarken.")],
    100: [T("Body, authored for 15 mm: the same parts with roundings in two steps (arm 35 mm, base 10 mm), cushion lofts of 7 stations; the 15 mm dip of the second seat is not shown; feet tapered in one step.", "Körper für 15 mm: dieselben Teile mit Rundungen in zwei Stufen (Armlehne 35 mm, Sockel 10 mm), Kissen-Lofts aus 7 Stationen; die 15-mm-Mulde des zweiten Platzes entfällt; Füße in einer Stufe konisch."),
           T("Thermal: the body surfaces per region. Structure: base rails and end panels, two straight springs per seat (the 20 mm bend kept under the worn seat), arm panels, top board, bottom rail and two posts, back rails and two uprights, cushion slabs, feet, joints, supports, masses; the back panel and the cross rail are left out.", "Thermik: die Körperflächen je Region. Tragwerk: Sockelholme und Stirnplatten, zwei gerade Federn je Platz (die 20-mm-Biegung unter dem durchgesessenen Platz bleibt), Armwangen, Deckbrett, Unterholm und zwei Pfosten, Rückenholme und zwei Stiele, Kissenplatten, Füße, Knoten, Auflager, Massen; Rückenplatte und Querholm entfallen.")],
    200: [T("Body, authored for 30 mm: arms rounded 30 mm, sharp bases, back frame with chamfers, cushion lofts of 5 stations keeping only the 35 mm sag at the arm and the leaning of the back cushions, cylindrical feet.", "Körper für 30 mm: Armlehnen mit 30 mm Rundung, scharfe Sockel, Rückenrahmen mit Fasen, Kissen-Lofts aus 5 Stationen, die nur die 35-mm-Mulde an der Armlehne und die Neigung der Rückenkissen behalten, zylindrische Füße."),
           T("Thermal: the body surfaces per region. Structure: base rails at their true 18 mm section declared slender, a 30 mm end panel on the two-seat part, two springs per part, one arm block per arm, slender back rails and two uprights, cushion slabs, feet, joints, supports, masses.", "Thermik: die Körperflächen je Region. Tragwerk: Sockelholme im wahren 18-mm-Querschnitt als schlank erklärt, eine 30-mm-Stirnplatte am Zweisitzerteil, zwei Federn je Teil, ein Armblock je Armlehne, schlanke Rückenholme und zwei Stiele, Kissenplatten, Füße, Knoten, Auflager, Massen.")],
    500: [T("Body, authored for 75 mm: sharp arm and base blocks, a chamfered back frame block, seat cushions as blocks, back cushions as leaning slabs, 75 mm foot blocks standing in the bag, blankets as sheets. The model is minimal: nothing smaller can be left out without losing the parts of the sofa.", "Körper für 75 mm: scharfe Armlehnen- und Sockelblöcke, ein gefaster Rückenrahmenblock, Sitzkissen als Blöcke, Rückenkissen als geneigte Platten, 75-mm-Fußblöcke in der Tasche, Decken als Flächen. Das Modell ist minimal: nichts Kleineres kann weggelassen werden, ohne die Teile des Sofas zu verlieren."),
           T("Thermal: the body surfaces per region. Structure: slender base rails, a 75 mm deck plate per part, one arm block per arm, one back panel per part, cushion slabs, feet, 75 mm joints, supports and masses.", "Thermik: die Körperflächen je Region. Tragwerk: schlanke Sockelholme, eine 75-mm-Deckplatte je Teil, ein Armblock je Armlehne, eine Rückenplatte je Teil, Kissenplatten, Füße, 75-mm-Knoten, Auflager und Massen.")],
}


def areas_of():
    """📐️ The outer area of every region on the 1:50 model: the surfaces of its solids less the faces two solids of the same region share (the back frames standing on their bases)."""
    pieces = body(50)
    out = {rid: 0.0 for rid in AREA_KEYS}
    for p in pieces:
        out[p.region] += surface_of(p)
    out["frame-cover"] -= 2 * 0.175 * ((MAIN_L - ARM_W) + CH_SEAT_W)
    return {k: round(v, 3) for k, v in out.items()}


# ----------------------------------------------------------------- document

REFERENCES = [
    {"id": "din-en-1728-2012", "citation": "DIN EN 1728:2012-10 Furniture - Seating - Test methods for the determination of strength and durability (static seat and back loads, seat front edge test)."},
    {"id": "din-en-12520-2015", "citation": "DIN EN 12520:2016-02 (EN 12520:2015) Furniture - Strength, durability and safety - Requirements for domestic seating (test loads applied with EN 1728)."},
    {"id": "en-1991-1-1-2002", "citation": "EN 1991-1-1:2002 Eurocode 1: Actions on structures - Part 1-1: General actions - Densities, self-weight, imposed loads for buildings (category A residential floors; a seated adult taken as 75 kg)."},
    {"id": "din-18011-2023", "citation": "DIN 18011:2023-05 Stellflächen, Abstände und Bewegungsflächen im Wohnungsbau (placement areas of seating furniture in dwellings)."},
    {"id": "en-312-2010", "citation": "EN 312:2010 Particleboards - Specifications (type P2, boards for interior fitments including furniture, for use in dry conditions; bending strength 11 N/mm2 and modulus 1600 N/mm2 at 13-20 mm)."},
    {"id": "en-338-2016", "citation": "EN 338:2016 Structural timber - Strength classes (C24: mean density 420 kg/m3, E0,mean 11 kN/mm2, fm,k 24 N/mm2)."},
    {"id": "en-622-2-2004", "citation": "EN 622-2:2004 Fibreboards - Specifications - Part 2: Requirements for hardboards (HB, about 850-1000 kg/m3)."},
    {"id": "en-10270-1-2011", "citation": "EN 10270-1:2011 Steel wire for mechanical springs - Part 1: Patented cold drawn unalloyed spring steel wire (grade SM, tensile strength about 1600 N/mm2 at 4 mm)."},
    {"id": "iso-10456-2007", "citation": "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (timber, particleboard, fibreboard, plastics, textiles, flexible foams)."},
    {"id": "iso-3386-1", "citation": "ISO 3386-1:1986 Polymeric materials, cellular flexible - Determination of stress-strain characteristic in compression - Part 1: Low-density materials (compression stress value at 40 % of 3-5 kPa for upholstery foams around 35 kg/m3)."},
    {"id": "ral-gz-430", "citation": "RAL-GZ 430 Möbel - Gütesicherung (German furniture quality mark: upholstery durability and foam density requirements for seating)."},
    {"id": "wood-handbook-2021", "citation": "Forest Products Laboratory (2021). Wood Handbook: Wood as an Engineering Material. General Technical Report FPL-GTR-282, USDA Forest Service (density and properties of beech)."},
    {"id": "ashrae-2021-fundamentals", "citation": "ASHRAE Handbook - Fundamentals (2021), chapter 33 Physical properties of materials (emissivity and solar absorptance of surfaces); sun-faded fabric taken 0.14 below the unfaded cover as an authored estimate."},
    {"id": "incropera-2018", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (thermal penetration depth, radiative properties)."},
    {"id": "semio-used-household", "citation": "Semio staffage, ticket used-household-objects: catalogue-typical data of German mid-range corner sofas with a chaise (2.50-2.80 x 1.60-2.00 m, seat height 0.42-0.46 m, seat depth 0.55-0.60 m, 90-130 kg) and of quilted moving blankets (180 x 200 cm, 6-10 mm); authored estimates, not taken from one datasheet."},
]


def document():
    """📄️ The whole asset document."""
    areas = areas_of()
    pieces_by = {s: body(s) for s in SCALES}
    total = round(sum(kg for kg, _ in MASSES.values()), 2)
    reps, vols = {}, {}
    for s in SCALES:
        pieces = pieces_by[s]
        lo, hi = BANDS[s]
        reps[str(s)] = {"status": "authored", "denominator": s, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[s], "hatches": HATCHES[s], "geometry": drawings(s, pieces), "decisions": REP_DECISIONS[s]}
        vols[str(s)] = {"status": "authored", "denominator": s, "body": [p.solid for p in pieces], "thermal": thermal_list(pieces), "structure": structure_list(s, pieces), "decisions": VOL_DECISIONS[s]}
    xs, ys, zs = [], [], []
    for p in pieces_by[50]:
        fr = FR[p.bone]
        for view, g in p.views.items():
            pass
    asset = {
        "id": "used-corner-sofa",
        "category": "furniture",
        "title": T("Used Corner Sofa", "Gebrauchtes Ecksofa"),
        "summary": T("A second-hand L-shaped fabric corner sofa, 2.60 x 1.90 m, from a flat clearance: a two-seat part with a box arm and a chaise with a long outer arm, loose back cushions slumped, the seat at the arm and the chaise sagged, the arms bleached by the sun. Stored in two parts on two moving blankets with its eight feet unscrewed in a bag, carried in two parts by four movers, and reassembled in a living room with three people seated.", "Ein gebrauchtes L-förmiges Stoff-Ecksofa, 2,60 x 1,90 m, aus einer Haushaltsauflösung: ein Zweisitzerteil mit Kastenarmlehne und ein Longchair mit langer äußerer Armlehne, die losen Rückenkissen zusammengesackt, der Platz an der Armlehne und der Longchair durchgesessen, die Armlehnen von der Sonne ausgebleicht. In zwei Teilen auf zwei Möbeldecken eingelagert, die acht Füße abgeschraubt in einer Tasche, von vier Trägern in zwei Teilen getragen und in einem Wohnzimmer mit drei sitzenden Personen wieder aufgebaut."),
        "tags": ["used", "household", "second-hand", "living", "sofa", "corner-sofa", "chaise", "seating", "upholstery", "foam", "fabric", "chipboard", "moving-blanket", "two-part"],
        "provenance": {"author": "Semio staffage content (used household objects wave)", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES, "notes": T(
            "A typical German mid-range corner sofa of the 2010s, 2.60 x 1.90 x 0.85 m in use: two-seat part 1.65 x 0.95 m (box arm 0.18 m, two seats of 0.735 m) and chaise 0.95 x 1.90 m (seat 0.77 x 1.52 m, outer arm 0.18 m), joined by sofa connectors. Base of 18 mm chipboard P2 rails on spruce battens with Nosag springs, arms of hardboard on spruce posts padded with foam, back frame of spruce and chipboard, loose seat cushions of 0.14 m RG 35 foam and fibre-filled back cushions, eight turned beech feet of 0.10 m. Seat height 0.44 m, seat depth 0.55 m, arm height 0.62 m. Traces of use: seat at the arm sagged 35 mm over three tired springs, second seat 15 mm, chaise 25 mm, back cushions slumped to 15 degrees with dipped tops, arm tops faded, a coffee ring, a depot price sticker. Masses follow from the member sections, foam and fibre volumes and the cover fabric (0.40 kg/m2).",
            "Ein typisches deutsches Ecksofa der mittleren Preisklasse aus den 2010er Jahren, 2,60 x 1,90 x 0,85 m im Gebrauch: Zweisitzerteil 1,65 x 0,95 m (Kastenarmlehne 0,18 m, zwei Plätze zu 0,735 m) und Longchair 0,95 x 1,90 m (Sitz 0,77 x 1,52 m, äußere Armlehne 0,18 m), mit Sofaverbindern verbunden. Sockel aus 18-mm-Spanplattenholmen P2 auf Fichtenleisten mit Wellenfedern, Armlehnen aus Hartfaser auf Fichtenpfosten mit Schaumpolsterung, Rückenrahmen aus Fichte und Spanplatte, lose Sitzkissen aus 0,14 m RG-35-Schaum und faserfüllte Rückenkissen, acht gedrechselte Buchenfüße von 0,10 m. Sitzhöhe 0,44 m, Sitztiefe 0,55 m, Armlehnenhöhe 0,62 m. Gebrauchsspuren: Platz an der Armlehne 35 mm über drei müden Federn durchgesessen, zweiter Platz 15 mm, Longchair 25 mm, Rückenkissen auf 15 Grad zusammengesackt mit eingedellten Oberkanten, Armlehnenoberseiten ausgebleicht, ein Kaffeering, ein Preisetikett des Lagers. Die Massen folgen aus den Querschnitten der Glieder, den Schaum- und Faservolumen und dem Bezugsstoff (0,40 kg/m2).")},
        "dimensions": {"heightM": 0.0, "widthM": 0.0, "depthM": 0.0, "massKg": total},
        "materials": MATERIALS,
        "rig": rig(),
        "anchors": anchors(),
        "representations": reps,
        "symbols": [],
        "poses": poses(areas),
        "clips": clips(),
        "thermal": {"environment": {"airTempC": 21, "meanRadiantTempC": 20, "relativeHumidity": 0.5, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 350, "diffuseFraction": 0.15, "pressurePa": 101325}, "operating": [], "regions": regions(areas), "emitters": [], "passengers": [],
                    "assumptions": [T("Passive upholstery and storage aids: absorbed sun, long-wave exchange, convection and contact conduction; the foam core (0.14 m, 3.5 m2K/W) and the fibre fill (0.17 m, 3.8 m2K/W) carry the resistance, so the covers follow air and sun within minutes; heat capacities from the thermal penetration depth of one hour plus 0.52 kJ/m2K of cover.", "Passive Polster und Lagerhilfen: absorbierte Sonne, Langwellenaustausch, Konvektion und Kontaktleitung; Schaumkern (0,14 m, 3,5 m2K/W) und Faserfüllung (0,17 m, 3,8 m2K/W) tragen den Widerstand, die Bezüge folgen Luft und Sonne binnen Minuten; Wärmekapazitäten aus der Eindringtiefe einer Stunde plus 0,52 kJ/m2K Bezug."),
                                    T("Aged surfaces: the sun-faded arm fabric absorbs 0.48 of the solar radiation against 0.62 of the unfaded cover (authored estimate after ASHRAE surface data); the worn foam is 10 % softer than new (authored).", "Gealterte Oberflächen: der ausgebleichte Armlehnenstoff absorbiert 0,48 der Sonnenstrahlung gegen 0,62 beim nicht gebleichten Bezug (Schätzwert nach ASHRAE-Oberflächendaten); der abgenutzte Schaum ist 10 % weicher als neu (Schätzwert)."),
                                    T("Reference case: a living room at 21 degC with a south window sun patch of 350 W/m2; in the depot (stored, carried) no sun reaches the sofa. Contact fractions: blankets and bag lie on the floor, seated adults cover 0.14 m2 of seat and 0.08 m2 of back each at 30 degC (ISO 7730 clothing surface).", "Referenzfall: ein Wohnzimmer mit 21 degC und einem Südfenster-Sonnenfleck von 350 W/m2; im Lager (eingelagert, getragen) erreicht keine Sonne das Sofa. Kontaktanteile: Decken und Tasche liegen auf dem Boden, sitzende Erwachsene bedecken je 0,14 m2 Sitz und 0,08 m2 Rücken mit 30 degC (Bekleidungsoberfläche nach ISO 7730)."),
                                    T("Power: furniture without any electrical element, stated as passive 0 W on the frame cover.", "Leistung: Möbel ohne elektrisches Element, als passiv 0 W am Rahmenbezug angegeben.")]},
        "structure": {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": [], "clinical": False, "assumptions": [
            T("Stored: each part rests with its base corners on its blanket (eight contacts); carried: the hands of four movers hold each part at four grips under the base ends (support contacts); in use: the eight feet stand on the floor and carry the frames, the cushions sink on prismatic joints into the deck.", "Eingelagert: jedes Teil liegt mit den Sockelecken auf seiner Decke (acht Kontakte); getragen: die Hände von vier Trägern halten jedes Teil an vier Griffen unter den Sockelenden (Stützkontakte); in Gebrauch: die acht Füße stehen auf dem Boden und tragen die Rahmen, die Kissen sinken auf Schiebegelenken in das Deck."),
            T("Load path in use: occupant 75 kg (735.5 N, EN 1991-1-1 category A, EN 12520 domestic seating) on the foam slab, Nosag springs spanning the base rails (chipboard P2 18 x 195 mm, EN 312), rails into the arm posts and end panels, feet of turned beech to the floor. The worn springs under seat 1 are bent 20 mm; foam sink 55 mm at 735 N on the tired RG 35 core (ISO 3386-1, authored).", "Lastpfad in Gebrauch: Person 75 kg (735,5 N, EN 1991-1-1 Kategorie A, EN 12520 Wohnsitzmöbel) auf der Schaumplatte, Wellenfedern zwischen den Sockelholmen (Spanplatte P2 18 x 195 mm, EN 312), Holme in die Armpfosten und Stirnplatten, gedrechselte Buchenfüße zum Boden. Die müden Federn unter Platz 1 sind 20 mm durchgebogen; Schaumeinsinken 55 mm bei 735 N im müden RG-35-Kern (ISO 3386-1, Schätzwert)."),
            T("The solver treats the asset as one rigid body: in the carried pose the weight of the blankets, the bag and the feet that stay on the floor (6.6 kg, 7 % of the total) is counted on the movers' grips.", "Der Löser behandelt das Objekt als einen starren Körper: in der getragenen Pose wird das Gewicht der am Boden bleibenden Decken, Tasche und Füße (6,6 kg, 7 % der Gesamtmasse) auf die Griffe der Träger gerechnet.")]},
        "volumes": vols,
        "presentation": {"defaultPose": "stored", "defaultClip": "carry", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    bounds = envelope(pieces_by[50])
    asset["dimensions"].update({"heightM": round(bounds[1][1] - bounds[0][1], 3), "widthM": round(bounds[1][0] - bounds[0][0], 3), "depthM": round(bounds[1][2] - bounds[0][2], 3)})
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}


def envelope(pieces):
    """📦️ World bounds of the rest pose from the plan and front views of every piece."""
    lo, hi = [1e9] * 3, [-1e9] * 3
    for p in pieces:
        for view, axes in (("plan", (0, 2)), ("front", (2, 1))):
            g = p.views[view]
            if isinstance(g, tuple) and g[0] == "ellipse":
                pts = [(g[1] - g[3], g[2] - g[3]), (g[1] + g[3], g[2] + g[3])]
            elif isinstance(g, tuple) and g[0] == "ring":
                pts = g[1]
            else:
                pts = g
            for q in pts:
                for i, ax in enumerate(axes):
                    lo[ax] = min(lo[ax], q[i])
                    hi[ax] = max(hi[ax], q[i])
    return lo, hi


if __name__ == "__main__":
    doc = document()
    with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf-8") as fh:
        json.dump(doc, fh, ensure_ascii=False, indent=1)
        fh.write("\n")
    a = doc["asset"]
    print("[DEBUG] dimensions", a["dimensions"], "areas", {r["id"]: r["areaM2"] for r in a["thermal"]["regions"]})
    print("[DEBUG] masses", {k: round(v[0], 2) for k, v in MASSES.items()})
    print("[DEBUG] backfit", BACKFIT)
