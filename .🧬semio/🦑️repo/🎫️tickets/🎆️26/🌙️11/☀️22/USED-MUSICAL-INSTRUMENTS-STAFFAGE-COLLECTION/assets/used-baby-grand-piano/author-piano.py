#!/usr/bin/env python3
"""🎹 Authoritative author script of the staffage asset `used-baby-grand-piano` (Gebrauchter Stutzflügel).

Every hand-chosen number of the asset lives here; running the script rewrites `🔣️.json` next to it.
Frames: P is the played frame of the piano (X across the keyboard from bass -0.725 to treble +0.725,
Y up, Z along the length from the key slip -0.75 to the tail +0.75); the rest pose of the rig is the
stored pose (piano on its spine on the skid, lid closed, legs and lyre off), which is also the default
pose. Every part is one prism (or a set of prisms) in the frame of its bone; the 3D body solid and the
2D `solid` shapes of the three views are written from the same prism, so drawings and models agree.

@see ../../brief.md — the binding brief of the wave
@see ../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-tools.md — the tools
"""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "🔣️.json")
SCALES = [50, 100, 200, 500]
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}
R6 = lambda v: round(float(v), 6)


def T(en, de):
    return {"en": en, "de": de}


# ----------------------------------------------------------------------------------------------- vector maths
def mat_mul(a, b):
    return [[sum(a[i][k] * b[k][j] for k in range(3)) for j in range(3)] for i in range(3)]


def mat_vec(a, v):
    return [sum(a[i][k] * v[k] for k in range(3)) for i in range(3)]


def rot_w(d):
    c, s = math.cos(math.radians(d)), math.sin(math.radians(d))
    return [[c, -s, 0], [s, c, 0], [0, 0, 1]]


def rot_v(d):
    c, s = math.cos(math.radians(d)), math.sin(math.radians(d))
    return [[c, 0, -s], [0, 1, 0], [s, 0, c]]


def rot_u(d):
    c, s = math.cos(math.radians(d)), math.sin(math.radians(d))
    return [[1, 0, 0], [0, c, s], [0, -s, c]]


def rest_rot(angle, abduction=0.0, twist=0.0):
    return mat_mul(mat_mul(rot_w(angle), rot_v(abduction)), rot_u(twist))


def poly_area(p):
    return 0.5 * sum(p[i][0] * p[(i + 1) % len(p)][1] - p[(i + 1) % len(p)][0] * p[i][1] for i in range(len(p)))


def poly_centroid(p):
    a = poly_area(p)
    cx = sum((p[i][0] + p[(i + 1) % len(p)][0]) * (p[i][0] * p[(i + 1) % len(p)][1] - p[(i + 1) % len(p)][0] * p[i][1]) for i in range(len(p)))
    cy = sum((p[i][1] + p[(i + 1) % len(p)][1]) * (p[i][0] * p[(i + 1) % len(p)][1] - p[(i + 1) % len(p)][0] * p[i][1]) for i in range(len(p)))
    return (cx / (6 * a), cy / (6 * a))


def ccw(p):
    return p if poly_area(p) > 0 else list(reversed(p))


def dedupe(p, eps=2e-4):
    out = []
    for q in p:
        if not out or abs(q[0] - out[-1][0]) > eps or abs(q[1] - out[-1][1]) > eps:
            out.append(q)
    while len(out) > 3 and abs(out[0][0] - out[-1][0]) <= eps and abs(out[0][1] - out[-1][1]) <= eps:
        out.pop()
    changed = True
    while changed and len(out) > 3:
        changed = False
        for i in range(len(out)):
            p0, p1, p2 = out[i - 1], out[i], out[(i + 1) % len(out)]
            ax, ay, bx, by = p1[0] - p0[0], p1[1] - p0[1], p2[0] - p1[0], p2[1] - p1[1]
            if abs(ax * by - ay * bx) < 1e-9 * max(1e-9, math.hypot(ax, ay) * math.hypot(bx, by)) + 1e-12:
                out.pop(i)
                changed = True
                break
    return out


def sig(v, n=6):
    return float(f"{v:.{n}g}")


def rect(a0, b0, a1, b1):
    return [(a0, b0), (a1, b0), (a1, b1), (a0, b1)]


def circle(ca, cb, r, n=16):
    return [(ca + r * math.cos(2 * math.pi * i / n), cb + r * math.sin(2 * math.pi * i / n)) for i in range(n)]


def round_rect(a0, b0, a1, b1, r, n=3):
    pts = []
    for (cx, cy, start) in [(a1 - r, b0 + r, -90), (a1 - r, b1 - r, 0), (a0 + r, b1 - r, 90), (a0 + r, b0 + r, 180)]:
        for i in range(n + 1):
            t = math.radians(start + 90 * i / n)
            pts.append((cx + r * math.cos(t), cy + r * math.sin(t)))
    return pts


def catmull(points, per):
    out = []
    pts = [points[0]] + points + [points[-1]]
    for i in range(1, len(pts) - 2):
        p0, p1, p2, p3 = pts[i - 1], pts[i], pts[i + 1], pts[i + 2]
        for k in range(per):
            t = k / per
            t2, t3 = t * t, t * t * t
            out.append(tuple(0.5 * ((2 * p1[j]) + (-p0[j] + p2[j]) * t + (2 * p0[j] - 5 * p1[j] + 4 * p2[j] - p3[j]) * t2 + (-p0[j] + 3 * p1[j] - 3 * p2[j] + p3[j]) * t3) for j in range(2)))
    out.append(points[-1])
    return out


def offset_path(path, d):
    out = []
    n = len(path)
    for i in range(n):
        if i == 0:
            dx, dz = path[1][0] - path[0][0], path[1][1] - path[0][1]
            l = math.hypot(dx, dz)
            nx, nz = -dz / l, dx / l
            out.append((path[0][0] + nx * d, path[0][1] + nz * d))
            continue
        if i == n - 1:
            dx, dz = path[-1][0] - path[-2][0], path[-1][1] - path[-2][1]
            l = math.hypot(dx, dz)
            nx, nz = -dz / l, dx / l
            out.append((path[-1][0] + nx * d, path[-1][1] + nz * d))
            continue
        ax, az = path[i][0] - path[i - 1][0], path[i][1] - path[i - 1][1]
        bx, bz = path[i + 1][0] - path[i][0], path[i + 1][1] - path[i][1]
        la, lb = math.hypot(ax, az), math.hypot(bx, bz)
        n1 = (-az / la, ax / la)
        n2 = (-bz / lb, bx / lb)
        mx, mz = n1[0] + n2[0], n1[1] + n2[1]
        lm = math.hypot(mx, mz)
        mx, mz = mx / lm, mz / lm
        cosh = mx * n1[0] + mz * n1[1]
        out.append((path[i][0] + mx * d / cosh, path[i][1] + mz * d / cosh))
    return remove_loops(out)


def seg_cross(p, q, r, s_):
    def o(a_, b_, c_):
        return (b_[0] - a_[0]) * (c_[1] - a_[1]) - (b_[1] - a_[1]) * (c_[0] - a_[0])
    d1, d2, d3, d4 = o(r, s_, p), o(r, s_, q), o(p, q, r), o(p, q, s_)
    if (d1 * d2 < 0) and (d3 * d4 < 0):
        t = d1 / (d1 - d2)
        return (p[0] + t * (q[0] - p[0]), p[1] + t * (q[1] - p[1]))
    return None


def remove_loops(path):
    """Cuts out the local loops an inward offset leaves where the curvature radius is below the offset."""
    pts = list(path)
    changed = True
    while changed:
        changed = False
        n = len(pts)
        for i in range(n - 1):
            for j in range(i + 2, min(n - 1, i + 40)):
                x = seg_cross(pts[i], pts[i + 1], pts[j], pts[j + 1])
                if x is not None:
                    pts = pts[: i + 1] + [x] + pts[j + 1:]
                    changed = True
                    break
            if changed:
                break
    return pts


def clip_half(poly, a, b, c):
    """Keeps the part of `poly` where a*x + b*y <= c."""
    out = []
    n = len(poly)
    for i in range(n):
        p, q = poly[i], poly[(i + 1) % n]
        fp, fq = a * p[0] + b * p[1] - c, a * q[0] + b * q[1] - c
        if fp <= 0:
            out.append(p)
        if (fp < 0 < fq) or (fq < 0 < fp):
            t = fp / (fp - fq)
            out.append((p[0] + t * (q[0] - p[0]), p[1] + t * (q[1] - p[1])))
    return out


def line_in_poly(poly, p0, d):
    """The longest chord of the line p0 + t d inside `poly` as (t0, t1)."""
    ts = []
    n = len(poly)
    for i in range(n):
        a, b = poly[i], poly[(i + 1) % n]
        ex, ez = b[0] - a[0], b[1] - a[1]
        den = d[0] * ez - d[1] * ex
        if abs(den) < 1e-12:
            continue
        t = ((a[0] - p0[0]) * ez - (a[1] - p0[1]) * ex) / den
        s = ((a[0] - p0[0]) * d[1] - (a[1] - p0[1]) * d[0]) / den
        if -1e-9 <= s <= 1 + 1e-9:
            ts.append(t)
    ts.sort()
    best = None
    for i in range(0, len(ts) - 1, 2):
        if best is None or ts[i + 1] - ts[i] > best[1] - best[0]:
            best = (ts[i], ts[i + 1])
    return best


def bar(p0, p1, w):
    dx, dz = p1[0] - p0[0], p1[1] - p0[1]
    l = math.hypot(dx, dz)
    nx, nz = -dz / l * w / 2, dx / l * w / 2
    return ccw([(p0[0] + nx, p0[1] + nz), (p1[0] + nx, p1[1] + nz), (p1[0] - nx, p1[1] - nz), (p0[0] - nx, p0[1] - nz)])


def simplify(path, keep):
    """Keeps every `keep`-th interior point of a path and both ends."""
    if keep <= 1:
        return list(path)
    return [path[0]] + [p for i, p in enumerate(path[1:-1], 1) if i % keep == 0] + [path[-1]]


# ----------------------------------------------------------------------------------------------- design numbers (reference table in the report)
HALF_W = 0.725
Z_FRONT = -0.75
Z_TAIL = 0.75
Y_KEYBED_BOT = 0.625
Y_KEYBED_TOP = 0.660
Y_RIM_BOT = 0.655
Y_RIM_TOP = 0.960
Y_LID_TOP = 0.980
RIM_T = 0.070
KB_HALF = 0.6125
Y_WHITE_BOT, Y_WHITE_TOP, Y_BLACK_TOP = 0.700, 0.722, 0.734
Z_KEY_FRONT, Z_FALL_HINGE, Z_NAME_FRONT, Z_NAME_BACK = -0.738, -0.592, -0.586, -0.571
Z_CHEEK_BACK, Z_RIM_FRONT = -0.55, -0.55
Z_BELLY_FRONT, Z_BELLY_BACK, Z_KEYBED_BACK = -0.42, -0.32, -0.32
Y_SB_BOT, Y_SB_TOP = 0.760, 0.769
Y_RIB_BOT = 0.738
Y_BEAM_TOP = 0.738
Y_STR_BOT, Y_STR_TOP = 0.845, 0.851
Y_PLATE_BOT, Y_PLATE_TOP = 0.865, 0.895
Y_NAME_BOT, Y_NAME_TOP, Y_DESKRAIL_TOP = 0.736, 0.935, 0.945
Z_FLAP_FRONT, Z_FLAP_BACK = -0.60, -0.36
LEG_LEN = 0.625
STORE_A, STORE_B = 0.8175, 0.78
PLAY_C = -0.975
SKID_X0, SKID_W, SKID_L = -0.20, 0.40, 1.70

BENT_CTRL = [(0.725, -0.30), (0.715, -0.24), (0.68, -0.185), (0.62, -0.14), (0.50, -0.07), (0.35, 0.02), (0.20, 0.12), (0.05, 0.235), (-0.10, 0.37), (-0.24, 0.52), (-0.36, 0.64), (-0.46, 0.715), (-0.555, 0.744), (-0.615, 0.75), (-0.665, 0.742), (-0.70, 0.718), (-0.719, 0.682), (-0.725, 0.64)]


def side_path(per):
    """The outer case outline from the treble cheek back (Z = -0.55) round the tail to the bass cheek back."""
    curve = catmull(BENT_CTRL, per)
    return [(HALF_W, Z_RIM_FRONT)] + curve + [(-HALF_W, Z_RIM_FRONT)]


def outer_outline(per):
    return ccw([(-HALF_W, Z_FRONT), (HALF_W, Z_FRONT)] + side_path(per))


def inner_path(per, extra=0.0):
    return offset_path(side_path(per), RIM_T + extra)


def rim_ring(per):
    outer = side_path(per)
    inner = inner_path(per)
    return ccw(outer + list(reversed(inner)))


def inner_region(per, z_front, extra=0.0):
    inner = inner_path(per, extra)
    poly = ccw([(inner[-1][0], z_front)] + [(inner[0][0], z_front)] + inner[1:-1])
    return clip_half(poly, 0, -1, -z_front)


def x_at(path, z, side):
    """X of the outline `path` at length position Z on the treble (side=+1) or bass (side=-1) side."""
    best = None
    for i in range(len(path) - 1):
        a, b = path[i], path[i + 1]
        if (a[1] - z) * (b[1] - z) <= 0 and a[1] != b[1]:
            t = (z - a[1]) / (b[1] - a[1])
            x = a[0] + t * (b[0] - a[0])
            if best is None or (side > 0 and x > best) or (side < 0 and x < best):
                best = x
    return best


# ----------------------------------------------------------------------------------------------- frames of the stored and played poses
def stored_world(X, Y, Z):
    return (STORE_A - Y, X + STORE_B, Z)


def played_world(X, Y, Z):
    return (X + PLAY_C, Y, Z)


CASE_O_P = (-HALF_W, Y_RIM_BOT, 0.0)
CASE_O_STORED = stored_world(*CASE_O_P)
CASE_O_PLAYED = played_world(*CASE_O_P)


def case_local(X, Y, Z):
    return (X - CASE_O_P[0], Y - CASE_O_P[1], Z)


LID_O = case_local(-HALF_W, Y_LID_TOP, 0.0)
LID_LONG_DEG, LID_SHORT_DEG = None, None
PROP_LONG = dict(X=None, Y=0.946, Z=-0.20, L=0.80, r=0.010, phi=95.0)
PROP_SHORT = dict(X=None, Y=0.946, Z=-0.10, L=0.36, r=0.009, phi=95.0)


def lid_angle_for(prop):
    hx, hy = -HALF_W, Y_LID_TOP
    px = prop["X"] + prop["L"] * math.cos(math.radians(prop["phi"]))
    py = prop["Y"] + prop["L"] * math.sin(math.radians(prop["phi"]))
    dx, dy = px - hx, py - hy
    lo, hi = 0.0, 60.0
    for _ in range(80):
        mid = (lo + hi) / 2
        t = math.radians(mid)
        f = -dx * math.sin(t) + dy * math.cos(t) + 0.020
        if f > 0:
            lo = mid
        else:
            hi = mid
    return (lo + hi) / 2


for _prop in (PROP_LONG, PROP_SHORT):
    _prop["X"] = round(x_at(offset_path(side_path(4), RIM_T), _prop["Z"], 1) - 0.018, 4)
LID_LONG_DEG = lid_angle_for(PROP_LONG)
LID_SHORT_DEG = lid_angle_for(PROP_SHORT)

# straps: stored corners of the path round case and skid
STRAP_Z = {"a": -0.15, "b": 0.30}
STRAP_W, STRAP_T = 0.050, 0.003
A_PT = (-0.2015, 0.0015)
B_PT = (-0.164, 1.5065)
C_PT = (0.164, 1.5065)
D_PT = (0.2015, 0.0015)
UP_LEN = math.hypot(B_PT[0] - A_PT[0], B_PT[1] - A_PT[1])
UP_DEG = math.degrees(math.atan2(B_PT[1] - A_PT[1], B_PT[0] - A_PT[0]))
TOP_LEN = C_PT[0] - B_PT[0] - 0.003
DOWN_LEN = math.hypot(D_PT[0] - C_PT[0], D_PT[1] - C_PT[1])
DOWN_DEG = math.degrees(math.atan2(D_PT[1] - C_PT[1], D_PT[0] - C_PT[0]))

# legs and lyre: mounted (played) points in P, stored world placements
LEGS = {
    "leg-bass": dict(mount=(-0.655, Y_KEYBED_BOT, -0.62), stored=(-0.2005, 0.05, -0.62), deg=180.0, title=T("Bass front leg", "Vorderes Bassbein")),
    "leg-treble": dict(mount=(0.655, Y_KEYBED_BOT, -0.62), stored=(-0.2005, 0.15, -0.62), deg=180.0, title=T("Treble front leg", "Vorderes Diskantbein")),
    "leg-tail": dict(mount=(-0.42, Y_KEYBED_BOT, 0.55), stored=(0.2005, 0.05, 0.55), deg=0.0, title=T("Tail leg", "Hinteres Bein")),
}
LYRE = dict(mount=(-0.02, Y_KEYBED_BOT, -0.47), stored=(0.1930 + 0.13, 0.605, -0.47), deg=-90.0)
PEDALS = {"pedal-soft": (-0.06, T("Soft pedal (una corda, left)", "Verschiebungspedal (una corda, links)")), "pedal-sostenuto": (0.0, T("Sostenuto pedal (middle)", "Tonhaltepedal (Mitte)")), "pedal-sustain": (0.06, T("Sustain pedal (right)", "Fortepedal (rechts)"))}
PEDAL_PRESS_DEG = -5.0
CARRY_LIFT = 0.08
CARRY_AWAY = 1.20

PLAY_CASE_DX = CASE_O_PLAYED[0] - CASE_O_STORED[0]
PLAY_CASE_DY = CASE_O_PLAYED[1] - CASE_O_STORED[1]


def chain_offsets(stored_xy, stored_deg, mount_local, played_rel_deg):
    """Slide (case u), lift (case v) and revolute offsets that carry a part from its stored world place to its mount."""
    ox, oy = CASE_O_STORED[0], CASE_O_STORED[1]
    su = stored_xy[1] - oy
    sv = -(stored_xy[0] - ox)
    rest_rel = stored_deg - 90.0
    return dict(rest=(su, sv), rest_rel=rest_rel, du=mount_local[0] - su, dv=mount_local[1] - sv, dang=played_rel_deg - rest_rel)


LEG_CHAIN = {k: chain_offsets(v["stored"], v["deg"], case_local(*v["mount"]), -90.0) for k, v in LEGS.items()}
LYRE_CHAIN = chain_offsets(LYRE["stored"], LYRE["deg"], case_local(*LYRE["mount"]), -90.0)


# ----------------------------------------------------------------------------------------------- rig
def bone(id, parent, title, x, y, angle, length, joint, z=None, abduction=None, twist=None, mass=None, section=None):
    b = {"id": id, "parent": parent, "title": title, "x": R6(x), "y": R6(y), "angleDeg": R6(angle), "lengthM": R6(length), "joint": joint}
    if z is not None:
        b["z"] = R6(z)
    if abduction is not None:
        b["abductionDeg"] = R6(abduction)
    if twist is not None:
        b["twistDeg"] = R6(twist)
    if mass is not None:
        b["mass"] = mass
    if section is not None:
        b["section"] = section
    return b


def build_rig():
    bones = [
        bone("floor", None, T("Depot floor frame (massless, fixed root)", "Bezug zum Lagerboden (masselos, feste Wurzel)"), 0, 0, 0, 0.04, {"type": "fixed"}),
        bone("skid-slide", "floor", T("Skid carried along the floor (movers)", "Brett längs des Bodens getragen (Möbelpacker)"), SKID_X0, 0, 0, 0.02, {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 1.5}),
        bone("skid-lift", "skid-slide", T("Skid lifted by its hand slots", "Brett an den Griffschlitzen angehoben"), 0, 0, 0, 0.02, {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.2}),
        bone("skid", "skid-lift", T("Padded piano skid (Flügelbrett) with battens", "Gepolstertes Flügelbrett mit Querleisten"), 0, 0, 0, SKID_W, {"type": "fixed"}),
        bone("case-slide", "skid", T("Case moved off the skid (setting up)", "Gehäuse vom Brett genommen (Aufstellen)"), CASE_O_STORED[0] - SKID_X0, CASE_O_STORED[1], 0, 0.02, {"type": "prismatic", "axisDeg": 0, "minM": -2.0, "maxM": 0}),
        bone("case-lift", "case-slide", T("Case raised to playing height", "Gehäuse auf Spielhöhe gehoben"), 0, 0, 0, 0.02, {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.7}),
        bone("case", "case-lift", T("Case: rim, keybed, keyboard, action, plate, strings, soundboard and beams (tipped about the spine edge)", "Gehäuse: Rast, Klaviaturboden, Tastatur, Mechanik, Platte, Saiten, Resonanzboden und Rastenbalken (um die Rastkante gekippt)"), 0, 0, 90, 2 * HALF_W, {"type": "revolute", "minDeg": -90, "maxDeg": 0}),
        bone("lid", "case", T("Main lid on three spine hinges", "Hauptdeckel an drei Rastscharnieren"), LID_O[0], LID_O[1], 0, 2 * HALF_W, {"type": "revolute", "minDeg": 0, "maxDeg": 40}),
        bone("lid-flap", "lid", T("Front lid flap on its piano hinge", "Vorderdeckel am Klavierband"), 0, 0, 0, 0.05, {"type": "revolute", "twistMinDeg": -180, "twistMaxDeg": 0}, z=Z_FLAP_BACK),
    ]
    for key, prop, title in [("prop-long", PROP_LONG, T("Long lid prop", "Langer Deckelstab")), ("prop-short", PROP_SHORT, T("Short lid prop", "Kurzer Deckelstab"))]:
        loc = case_local(prop["X"], prop["Y"], prop["Z"])
        bones.append(bone(key, "case", title, loc[0], loc[1], 180, prop["L"], {"type": "revolute", "minDeg": -100, "maxDeg": 0}, z=prop["Z"]))
    fl = case_local(-KB_HALF, 0.744, Z_FALL_HINGE)
    bones.append(bone("fallboard", "case", T("Fallboard (key cover) on its back hinge", "Tastenklappe an ihrem hinteren Scharnier"), fl[0], fl[1], 0, 0.05, {"type": "revolute", "twistMinDeg": -95, "twistMaxDeg": 0}, z=Z_FALL_HINGE))
    dk = case_local(-0.32, 0.9525, -0.56)
    bones.append(bone("desk", "case", T("Music desk on its front hinge", "Notenpult an seinem vorderen Scharnier"), dk[0], dk[1], 0, 0.05, {"type": "revolute", "twistMinDeg": 0, "twistMaxDeg": 80}, z=-0.56))
    for key, leg in LEGS.items():
        ch = LEG_CHAIN[key]
        lim_u = sorted([0.0, ch["du"], -CARRY_LIFT])
        lim_v = sorted([0.0, ch["dv"], CARRY_AWAY])
        bones.append(bone(f"{key}-slide", "case", T(f"{leg['title']['en']}: slide along the case", f"{leg['title']['de']}: Verschiebung am Gehäuse"), ch["rest"][0], ch["rest"][1], 0, 0.02, {"type": "prismatic", "axisDeg": 0, "minM": R6(lim_u[0] - 0.01), "maxM": R6(lim_u[-1] + 0.01)}, z=leg["mount"][2]))
        bones.append(bone(f"{key}-lift", f"{key}-slide", T(f"{leg['title']['en']}: lift across the case", f"{leg['title']['de']}: Hub quer zum Gehäuse"), 0, 0, 0, 0.02, {"type": "prismatic", "axisDeg": 90, "minM": R6(lim_v[0] - 0.01), "maxM": R6(lim_v[-1] + 0.01)}))
        lo, hi = sorted([0.0, ch["dang"]])
        bones.append(bone(key, f"{key}-lift", leg["title"], 0, 0, ch["rest_rel"], LEG_LEN, {"type": "revolute", "minDeg": R6(lo), "maxDeg": R6(hi)}))
    ch = LYRE_CHAIN
    lim_u = sorted([0.0, ch["du"], -CARRY_LIFT])
    lim_v = sorted([0.0, ch["dv"], CARRY_AWAY])
    bones.append(bone("lyre-slide", "case", T("Pedal lyre: slide along the case", "Pedallyra: Verschiebung am Gehäuse"), ch["rest"][0], ch["rest"][1], 0, 0.02, {"type": "prismatic", "axisDeg": 0, "minM": R6(lim_u[0] - 0.01), "maxM": R6(lim_u[-1] + 0.01)}, z=LYRE["mount"][2]))
    bones.append(bone("lyre-lift", "lyre-slide", T("Pedal lyre: lift across the case", "Pedallyra: Hub quer zum Gehäuse"), 0, 0, 0, 0.02, {"type": "prismatic", "axisDeg": 90, "minM": R6(lim_v[0] - 0.01), "maxM": R6(lim_v[-1] + 0.01)}))
    lo, hi = sorted([0.0, ch["dang"]])
    bones.append(bone("lyre", "lyre-lift", T("Pedal lyre with pedal box and braces", "Pedallyra mit Pedalkasten und Streben"), 0, 0, ch["rest_rel"], 0.605, {"type": "revolute", "minDeg": R6(lo), "maxDeg": R6(hi)}))
    for key, (xoff, title) in PEDALS.items():
        bones.append(bone(key, "lyre", title, 0.565, xoff, 90, 0.05, {"type": "revolute", "minDeg": -6, "maxDeg": 0}, abduction=-90))
    for s in ["a", "b"]:
        zs = STRAP_Z[s]
        bones.append(bone(f"strap-{s}-up", "skid", T(f"Strap {s.upper()}: rising run past the lid", f"Gurt {s.upper()}: aufsteigender Lauf am Deckel"), A_PT[0] - SKID_X0, A_PT[1], UP_DEG, 0.05, {"type": "revolute", "minDeg": 0, "maxDeg": 95}, z=zs))
        bones.append(bone(f"strap-{s}-top", f"strap-{s}-up", T(f"Strap {s.upper()}: run over the treble side with ratchet", f"Gurt {s.upper()}: Lauf über die Diskantseite mit Ratsche"), UP_LEN + 0.003 * math.cos(math.radians(UP_DEG)), -0.003 * math.sin(math.radians(UP_DEG)), -UP_DEG, 0.05, {"type": "revolute", "minDeg": -95, "maxDeg": 0}))
        bones.append(bone(f"strap-{s}-down", f"strap-{s}-top", T(f"Strap {s.upper()}: falling run past the underside", f"Gurt {s.upper()}: absteigender Lauf an der Unterseite"), TOP_LEN, 0, DOWN_DEG, 0.05, {"type": "revolute", "minDeg": 0, "maxDeg": 95}))
    return bones


def solve(bones, offsets=None):
    offsets = offsets or {}
    world = {}
    by = {b["id"]: b for b in bones}
    for b in bones:
        off = offsets.get(b["id"], {})
        Rr = rest_rot(b["angleDeg"], b.get("abductionDeg", 0), b.get("twistDeg", 0))
        Ro = rest_rot(off.get("angleDeg", 0), off.get("abductionDeg", 0), off.get("twistDeg", 0))
        x = b["x"] + off.get("dx", 0)
        y = b["y"] + off.get("dy", 0)
        pos = [x, y, b.get("z", 0.0)]
        if b["parent"] is None:
            o = pos
            R = mat_mul(Rr, Ro)
        else:
            po, pR = world[b["parent"]]
            d = mat_vec(pR, pos)
            o = [po[i] + d[i] for i in range(3)]
            R = mat_mul(mat_mul(pR, Rr), Ro)
        world[b["id"]] = (o, R)
    return world


def to_world(world, bone_id, p):
    o, R = world[bone_id]
    d = mat_vec(R, p)
    return [o[i] + d[i] for i in range(3)]


# ----------------------------------------------------------------------------------------------- materials and densities
MATERIALS = [
    dict(id="case-lacquer", title=T("Laminated beech rim and case parts under aged black polyester lacquer (crazed)", "Schichtverleimte Buchenrast und Gehäuseteile unter gealtertem schwarzem Polyesterlack (craqueliert)"), emissivity=0.91, solarAbsorptivity=0.93, densityKgM3=720, conductivityWmK=0.17, specificHeatJKgK=1600, elasticModulusPa=1.4e10, strengthPa=9.0e7, source="literature", reference="iso-10456-2007"),
    dict(id="lid-lacquer", title=T("Lid board (poplar core, beech crossband) under aged black lacquer", "Deckelplatte (Pappelkern, Buchenabsperrung) unter gealtertem schwarzem Lack"), emissivity=0.91, solarAbsorptivity=0.93, densityKgM3=560, conductivityWmK=0.13, specificHeatJKgK=1600, elasticModulusPa=9.0e9, source="literature", reference="iso-10456-2007"),
    dict(id="key-ivorine", title=T("Yellowed acrylic key tops on spruce key bodies", "Vergilbte Acryl-Tastenbeläge auf Fichten-Tastenhebeln"), emissivity=0.90, solarAbsorptivity=0.40, densityKgM3=520, conductivityWmK=0.15, specificHeatJKgK=1500, source="authored-estimate", reference="iso-10456-2007"),
    dict(id="key-ebonite", title=T("Black key caps (phenolic, worn matt)", "Schwarze Obertasten (Phenolharz, mattgespielt)"), emissivity=0.92, solarAbsorptivity=0.92, densityKgM3=1300, conductivityWmK=0.25, specificHeatJKgK=1400, source="authored-estimate", reference="iso-10456-2007"),
    dict(id="action-felt", title=T("Action and key frame: hornbeam and maple parts, felt, buckskin, lead key weights and brass flanges", "Mechanik und Tastenrahmen: Hainbuche- und Ahornteile, Filz, Hirschleder, Bleigewichte und Messingflansche"), emissivity=0.90, solarAbsorptivity=0.70, densityKgM3=600, conductivityWmK=0.08, specificHeatJKgK=1500, source="authored-estimate", reference="askenfelt-1990"),
    dict(id="cast-iron", title=T("Grey cast-iron plate, bronze lacquer worn at the pins", "Graugussplatte, Bronzelack an den Stiften abgegriffen"), emissivity=0.80, solarAbsorptivity=0.60, densityKgM3=7200, conductivityWmK=50, specificHeatJKgK=460, elasticModulusPa=1.1e11, strengthPa=2.0e8, source="literature", reference="incropera-2018"),
    dict(id="string-steel", title=T("Music wire and copper-wound bass strings (layer of 230 strings)", "Klaviersaitendraht und kupferumsponnene Basssaiten (Lage von 230 Saiten)"), emissivity=0.30, solarAbsorptivity=0.50, densityKgM3=1300, conductivityWmK=45, specificHeatJKgK=470, source="authored-estimate", reference="fletcher-rossing-1998"),
    dict(id="spruce", title=T("Spruce soundboard and ribs, thin shellac, darkened", "Fichtenresonanzboden und Rippen, dünn schellackiert, nachgedunkelt"), emissivity=0.90, solarAbsorptivity=0.55, densityKgM3=450, conductivityWmK=0.12, specificHeatJKgK=1700, elasticModulusPa=1.1e10, source="literature", reference="fletcher-rossing-1998"),
    dict(id="beech-raw", title=T("Raw beech and spruce: keybed underside, beams, belly rail", "Rohe Buche und Fichte: Klaviaturboden unten, Rastenbalken, Anhängestock"), emissivity=0.90, solarAbsorptivity=0.60, densityKgM3=700, conductivityWmK=0.17, specificHeatJKgK=1600, elasticModulusPa=1.4e10, strengthPa=9.0e7, source="literature", reference="iso-10456-2007"),
    dict(id="brass", title=T("Tarnished cast brass: castors, ferrules, pedals, hinges", "Angelaufenes Messing: Rollen, Beinschuhe, Pedale, Scharniere"), emissivity=0.35, solarAbsorptivity=0.60, densityKgM3=8500, conductivityWmK=110, specificHeatJKgK=380, elasticModulusPa=1.0e11, source="literature", reference="incropera-2018"),
    dict(id="plywood", title=T("Birch plywood skid board and battens", "Birkensperrholz von Brett und Querleisten"), emissivity=0.90, solarAbsorptivity=0.60, densityKgM3=680, conductivityWmK=0.17, specificHeatJKgK=1600, elasticModulusPa=1.0e10, strengthPa=4.0e7, source="literature", reference="iso-10456-2007"),
    dict(id="carpet", title=T("Grey needle-felt carpet padding, worn", "Graue Nadelfilzpolsterung, abgenutzt"), emissivity=0.92, solarAbsorptivity=0.80, densityKgM3=250, conductivityWmK=0.06, specificHeatJKgK=1300, source="authored-estimate", reference="iso-10456-2007"),
    dict(id="webbing", title=T("Polyester ratchet-strap webbing with zinc-plated ratchet", "Polyester-Spanngurt mit verzinkter Ratsche"), emissivity=0.88, solarAbsorptivity=0.70, densityKgM3=1380, conductivityWmK=0.20, specificHeatJKgK=1300, source="authored-estimate", reference="iso-10456-2007"),
    dict(id="worn-finish", title=T("Lacquer worn through to grey primer and wood; dealer's paper sticker", "Bis auf grauen Grund und Holz durchgegriffener Lack; Papieraufkleber des Händlers"), emissivity=0.90, solarAbsorptivity=0.65, densityKgM3=700, conductivityWmK=0.15, specificHeatJKgK=1500, source="authored-estimate", reference="iso-10456-2007"),
]
DENSITY = {m["id"]: m["densityKgM3"] for m in MATERIALS}
TONE = {"case-lacquer": 0.12, "lid-lacquer": 0.10, "key-ivorine": 0.86, "key-ebonite": 0.08, "action-felt": 0.55, "cast-iron": 0.58, "string-steel": 0.7, "spruce": 0.78, "beech-raw": 0.68, "brass": 0.52, "plywood": 0.66, "carpet": 0.42, "webbing": 0.5, "worn-finish": 0.6}


# ----------------------------------------------------------------------------------------------- parts: one prism in the frame of its bone, written as a 3D solid and as 2D solid shapes
class Part:
    def __init__(self, id, bone, plane, prof, lo, hi, mat, region, role="body", holes=None, taper=None, flags=None, repeat=None, views=("elevation", "front", "plan"), hatch=None, rank=None, tone=None, member=False, tip=None, pen=None):
        self.id, self.bone, self.plane = id, bone, plane
        self.prof = dedupe([tuple(p) for p in prof])
        self.lo, self.hi = lo, hi
        self.mat, self.region, self.role = mat, region, role
        self.holes = [dedupe(h) for h in (holes or [])]
        self.taper, self.flags, self.repeat = taper, dict(flags or {}), repeat
        self.views, self.hatch, self.rank = views, hatch or {}, rank or {}
        self.tone = TONE[mat] if tone is None else tone
        self.member, self.tip, self.pen = member, tip, pen

    def volume(self):
        a = abs(poly_area(self.prof)) - sum(abs(poly_area(h)) for h in self.holes)
        k = 1.0
        if self.taper:
            s0, s1 = self.taper
            k = (s0 * s0 + s0 * s1 + s1 * s1) / 3
        n = self.repeat["count"] if self.repeat else 1
        return a * abs(self.hi - self.lo) * k * n

    def local_points(self, both=True):
        pts = []
        copies = self.repeat["count"] if self.repeat else 1
        step = self.repeat.get("translation", [0, 0, 0]) if self.repeat else [0, 0, 0]
        for c in range(copies):
            for (a, b) in self.prof:
                for s in ([self.lo, self.hi] if both else [self.lo]):
                    if self.plane == "uv":
                        p = [a, b, s]
                    elif self.plane == "uw":
                        p = [a, s, b]
                    else:
                        p = [s, a, b]
                    pts.append([p[i] + c * step[i] for i in range(3)])
        return pts

    def centroid_local(self):
        ca, cb = poly_centroid(ccw(self.prof))
        cs = (self.lo + self.hi) / 2
        if self.plane == "uv":
            p = [ca, cb, cs]
        elif self.plane == "uw":
            p = [ca, cs, cb]
        else:
            p = [cs, ca, cb]
        if self.repeat:
            n = self.repeat["count"]
            step = self.repeat.get("translation", [0, 0, 0])
            p = [p[i] + step[i] * (n - 1) / 2 for i in range(3)]
        return p

    def solid3d(self):
        if self.plane == "uv":
            plane, prof, holes = "xy", self.prof, self.holes
        elif self.plane == "uw":
            plane, prof, holes = "zx", [(b, a) for (a, b) in self.prof], [[(b, a) for (a, b) in h] for h in self.holes]
        else:
            plane, prof, holes = "yz", self.prof, self.holes
        prof = ccw(prof)
        s = {"id": self.id, "kind": "extrude", "bone": self.bone, "material": self.mat, "role": self.role, "tone": R6(self.tone), "profile": [[R6(a), R6(b)] for a, b in prof], "plane": plane, "from": R6(min(self.lo, self.hi)), "to": R6(max(self.lo, self.hi))}
        if holes:
            s["holes"] = [[[R6(a), R6(b)] for a, b in ccw(h)] for h in holes]
        if self.taper:
            s["taperTo"] = [R6(self.taper[0]), R6(self.taper[1])]
        for k, v in self.flags.items():
            s[k] = v
        if self.repeat:
            s["repeat"] = {"count": self.repeat["count"], "translation": [R6(v) for v in self.repeat["translation"]]}
        return s

    def rings2d(self):
        copies = self.repeat["count"] if self.repeat else 1
        step = self.repeat.get("translation", [0, 0, 0]) if self.repeat else [0, 0, 0]
        rings = []
        for c in range(copies):
            if self.plane == "uv":
                d = (step[0] * c, step[1] * c)
            elif self.plane == "uw":
                d = (step[0] * c, step[2] * c)
            else:
                d = (step[1] * c, step[2] * c)
            rings.append([(a + d[0], b + d[1]) for a, b in self.prof])
        return rings

    def shape2d(self, view, oz):
        if self.plane == "uv":
            solid = {"kind": "extruded", "sweep": "w", "z0": R6(min(self.lo, self.hi) + oz), "z1": R6(max(self.lo, self.hi) + oz)}
            if self.tip:
                solid["zTip"] = [R6(self.tip[0] + oz), R6(self.tip[1] + oz)]
        elif self.plane == "uw":
            solid = {"kind": "extruded", "sweep": "v", "z0": R6(min(self.lo, self.hi)), "z1": R6(max(self.lo, self.hi))}
        else:
            solid = {"kind": "extruded", "sweep": "u", "z0": R6(min(self.lo, self.hi)), "z1": R6(max(self.lo, self.hi))}
        rings = self.rings2d()
        sh = {"id": self.id, "bone": self.bone, "role": self.role, "z": 0, "filled": True, "tone": R6(self.tone), "region": self.region, "material": self.mat}
        if len(rings) == 1:
            sh.update({"kind": "poly", "points": [[R6(a), R6(b)] for a, b in ccw(rings[0])], "closed": True})
        else:
            sh.update({"kind": "path", "d": " ".join("M " + " L ".join(f"{R6(a)} {R6(b)}" for a, b in ccw(r)) + " Z" for r in rings)})
        if self.pen is not None:
            sh["penMm"] = self.pen
        if self.hatch.get(view):
            sh["hatch"] = self.hatch[view]
        sh["solid"] = solid
        return sh


class Frame:
    """🧭 Maps P-frame coordinates of a part onto the local frame of a bone that rests parallel to P."""

    def __init__(self, bone, ox, oy, oz=0.0):
        self.bone, self.ox, self.oy, self.oz = bone, ox, oy, oz

    def plan(self, id, polyXZ, ylo, yhi, mat, region, **kw):
        return Part(id, self.bone, "uw", [(x - self.ox, z - self.oz) for x, z in polyXZ], ylo - self.oy, yhi - self.oy, mat, region, holes=[[(x - self.ox, z - self.oz) for x, z in h] for h in kw.pop("holesXZ", [])], **kw)

    def side(self, id, polyXY, zlo, zhi, mat, region, **kw):
        return Part(id, self.bone, "uv", [(x - self.ox, y - self.oy) for x, y in polyXY], zlo - self.oz, zhi - self.oz, mat, region, **kw)

    def end(self, id, polyYZ, xlo, xhi, mat, region, **kw):
        return Part(id, self.bone, "vw", [(y - self.oy, z - self.oz) for y, z in polyYZ], xlo - self.ox, xhi - self.ox, mat, region, **kw)


CASE = Frame("case", -HALF_W, Y_RIM_BOT, 0.0)
LIDF = Frame("lid", -HALF_W, Y_LID_TOP, 0.0)
FLAPF = Frame("lid-flap", -HALF_W, Y_LID_TOP, Z_FLAP_BACK)
FALLF = Frame("fallboard", -KB_HALF, 0.744, Z_FALL_HINGE)
DESKF = Frame("desk", -0.32, 0.9525, -0.56)


def black_keys():
    ww = 2 * KB_HALF / 52
    names = ["A", "B"] + ["C", "D", "E", "F", "G", "A", "B"] * 7 + ["C"]
    shift = {"C": -0.15, "D": 0.15, "F": -0.2, "G": 0.0, "A": 0.2}
    keys = []
    for i, n in enumerate(names[:-1]):
        if n in shift:
            xb = -KB_HALF + (i + 1) * ww + shift[n] * ww
            keys.append((n, xb))
    return ww, keys


def octave_patterns():
    ww, keys = black_keys()
    first = keys[0]
    rest = keys[1:]
    pattern = {}
    for n, xb in rest:
        pattern.setdefault(n, []).append(xb)
    return ww, first, pattern


def case_parts(scale):
    per = {50: 4, 100: 2, 200: 1, 500: 1}[scale]
    parts = []
    hatch_lac = {50: "crazed-lacquer", 100: "crazed-lacquer"}.get(scale)
    rim_poly = rim_ring(per) if scale != 500 else rim_ring(1)
    rim_t = RIM_T if scale != 500 else 0.075
    if scale == 500:
        outer = side_path(1)
        inner = offset_path(outer, rim_t)
        rim_poly = ccw(outer + list(reversed(inner)))
    parts.append(CASE.plan("rim", rim_poly, Y_RIM_BOT, Y_RIM_TOP, "case-lacquer", "case", member=True, hatch={"front": hatch_lac, "plan": hatch_lac} if hatch_lac else None))
    if scale in (50, 100):
        parts.append(CASE.plan("keybed", rect(-HALF_W, Z_FRONT, HALF_W, Z_KEYBED_BACK), Y_KEYBED_BOT, Y_KEYBED_TOP, "beech-raw", "bracing", member=True))
        for s, x0, x1 in [("bass", -HALF_W, -KB_HALF), ("treble", KB_HALF, HALF_W)]:
            r = 0.06 if scale == 50 else 0.05
            prof = [(Y_KEYBED_TOP, Z_FRONT), (0.80 - r, Z_FRONT)] + [(0.80 - r + r * math.sin(math.radians(a)), Z_FRONT + r - r * math.cos(math.radians(a))) for a in (30, 60)] + [(0.80, Z_FRONT + r), (0.80, Z_CHEEK_BACK), (Y_KEYBED_TOP, Z_CHEEK_BACK)]
            parts.append(CASE.end(f"cheek-{s}", prof, x0, x1, "case-lacquer", "case"))
            parts.append(CASE.side(f"end-block-{s}", rect(x0, 0.80, x1, Y_RIM_TOP), Z_FLAP_FRONT, Z_CHEEK_BACK, "case-lacquer", "case"))
        parts.append(CASE.side("key-slip", rect(-KB_HALF, Y_KEYBED_TOP, KB_HALF, Y_WHITE_BOT), Z_FRONT, Z_KEY_FRONT, "case-lacquer", "case"))
        parts.append(CASE.plan("keyframe", rect(-KB_HALF, Z_KEY_FRONT + 0.003, KB_HALF, Z_BELLY_FRONT), Y_KEYBED_TOP, Y_WHITE_BOT, "action-felt", "keys"))
        ww = 2 * KB_HALF / 52
        if scale == 50:
            parts.append(CASE.plan("white-keys", rect(-KB_HALF + 0.0005, Z_KEY_FRONT, -KB_HALF + ww - 0.0005, Z_NAME_BACK), Y_WHITE_BOT, Y_WHITE_TOP, "key-ivorine", "keys", repeat={"count": 52, "translation": [ww, 0, 0]}, hatch={"plan": "key-joints"}))
            wwi, first, pattern = octave_patterns()
            bw = 0.0135
            n0, x0 = first
            parts.append(CASE.plan("black-key-a0", rect(x0 - bw / 2, -0.682, x0 + bw / 2, Z_NAME_BACK), Y_WHITE_TOP, Y_BLACK_TOP, "key-ebonite", "keys"))
            for n in ["C", "D", "F", "G", "A"]:
                xs = pattern[n]
                parts.append(CASE.plan(f"black-keys-{n.lower()}", rect(xs[0] - bw / 2, -0.682, xs[0] + bw / 2, Z_NAME_BACK), Y_WHITE_TOP, Y_BLACK_TOP, "key-ebonite", "keys", repeat={"count": len(xs), "translation": [xs[1] - xs[0], 0, 0]}))
        else:
            parts.append(CASE.plan("white-keys", rect(-KB_HALF, Z_KEY_FRONT, KB_HALF, Z_NAME_BACK), Y_WHITE_BOT, Y_WHITE_TOP, "key-ivorine", "keys", hatch={"plan": "key-joints-100"}))
            parts.append(CASE.plan("black-keys", rect(-KB_HALF + 0.03, -0.682, KB_HALF - 0.03, Z_NAME_BACK), Y_WHITE_TOP, Y_WHITE_TOP + 0.0015, "key-ebonite", "keys", flags={"texture": True}, role="texture", hatch={"plan": "black-key-row"}))
        parts.append(CASE.side("nameboard", rect(-KB_HALF, Y_NAME_BOT, KB_HALF, Y_NAME_TOP), Z_NAME_FRONT, Z_NAME_BACK, "case-lacquer", "case"))
        parts.append(CASE.plan("desk-rail", rect(-KB_HALF, Z_NAME_FRONT, KB_HALF, Z_BELLY_FRONT), Y_NAME_TOP, Y_DESKRAIL_TOP, "case-lacquer", "case"))
        parts.append(CASE.plan("action", rect(-0.60, Z_NAME_BACK, 0.60, Z_BELLY_FRONT), Y_WHITE_BOT, 0.88, "action-felt", "keys"))
        parts.append(CASE.plan("belly-rail", rect(-HALF_W + RIM_T, Z_BELLY_FRONT, HALF_W - RIM_T, Z_BELLY_BACK), Y_KEYBED_TOP, Y_PLATE_BOT, "beech-raw", "bracing", member=True))
    else:
        r = 0.06
        front = [(-HALF_W, Y_KEYBED_BOT), (HALF_W, Y_KEYBED_BOT), (HALF_W, 0.80), (KB_HALF, 0.80), (KB_HALF, Y_WHITE_TOP), (-KB_HALF, Y_WHITE_TOP), (-KB_HALF, 0.80), (-HALF_W, 0.80)]
        parts.append(CASE.side("keyboard-end", front, Z_FRONT, Z_CHEEK_BACK, "case-lacquer", "case", member=True))
        parts.append(CASE.plan("keybed", rect(-HALF_W, Z_CHEEK_BACK, HALF_W, Z_KEYBED_BACK), Y_KEYBED_BOT, Y_KEYBED_TOP, "beech-raw", "bracing", member=True))
        parts.append(CASE.plan("keys", rect(-KB_HALF, Z_KEY_FRONT, KB_HALF, Z_CHEEK_BACK), Y_WHITE_TOP, Y_BLACK_TOP if scale == 200 else Y_WHITE_TOP + 0.003, "key-ivorine", "keys", flags={"texture": True} if scale == 500 else None, role="texture" if scale == 500 else "body"))
        hi = Y_DESKRAIL_TOP
        for s, x0, x1 in [("bass", -HALF_W, -KB_HALF), ("treble", KB_HALF, HALF_W)]:
            parts.append(CASE.side(f"end-block-{s}", rect(x0, 0.80, x1, Y_RIM_TOP), Z_FLAP_FRONT, Z_CHEEK_BACK, "case-lacquer", "case"))
        parts.append(CASE.plan("front-body", rect(-KB_HALF, Z_NAME_FRONT, KB_HALF, Z_BELLY_BACK), Y_WHITE_TOP, hi, "case-lacquer", "case"))
    sb_front = Z_BELLY_BACK
    sbpoly = inner_region(per if scale != 500 else 1, sb_front)
    parts.append(CASE.plan("soundboard", sbpoly, Y_SB_BOT, Y_SB_TOP, "spruce", "soundboard", flags={"sheet": True}))
    if scale == 50:
        d = (math.cos(math.radians(-35)), math.sin(math.radians(-35)))
        n = (-d[1], d[0])
        ribs = []
        for k in range(-6, 8):
            p0 = (0.0 + n[0] * 0.105 * k, 0.20 + n[1] * 0.105 * k)
            ch = line_in_poly(sbpoly, p0, d)
            if ch and ch[1] - ch[0] > 0.25:
                a = (p0[0] + d[0] * (ch[0] + 0.01), p0[1] + d[1] * (ch[0] + 0.01))
                b = (p0[0] + d[0] * (ch[1] - 0.01), p0[1] + d[1] * (ch[1] - 0.01))
                ribs.append((a, b))
        for i, (a, b) in enumerate(ribs):
            parts.append(CASE.plan(f"rib-{i + 1}", bar(a, b, 0.025), Y_RIB_BOT, Y_SB_BOT, "spruce", "bracing"))
    beams = [((-0.48, Z_BELLY_BACK), (-0.44, 0.70)), ((0.02, Z_BELLY_BACK), (-0.30, 0.66)), ((0.50, Z_BELLY_BACK), (0.02, 0.40))]
    inner = inner_region(per if scale != 500 else 1, sb_front - 0.05)
    bw = {50: 0.05, 100: 0.05, 200: 0.05, 500: 0.075}[scale]
    bh_bot = Y_RIM_BOT if scale != 500 else Y_RIM_BOT
    for i, (a, b) in enumerate(beams):
        d = (b[0] - a[0], b[1] - a[1])
        l = math.hypot(*d)
        d = (d[0] / l, d[1] / l)
        ch = line_in_poly(inner, a, d)
        e = (a[0] + d[0] * ch[1], a[1] + d[1] * ch[1])
        st = (a[0] - d[0] * 0.0, a[1])
        parts.append(CASE.plan(f"beam-{i + 1}", bar(st, e, bw), bh_bot, Y_BEAM_TOP if scale != 500 else Y_SB_BOT, "beech-raw", "bracing", member=True))
    tb = 0.14 if scale != 500 else 0.15
    parts.append(CASE.plan("tail-leg-block", rect(-0.42 - tb / 2, 0.55 - tb / 2, -0.42 + tb / 2, 0.55 + tb / 2), Y_KEYBED_BOT, Y_RIM_BOT, "beech-raw", "bracing"))
    plate_out = inner_region(per if scale != 500 else 1, -0.43, 0.012)
    holes = []
    if scale in (50, 100):
        core = inner_region(per, -0.27, 0.012 + 0.085)
        for (lo, hi) in [(-1.0, -0.28), (-0.22, 0.14), (0.20, 1.0)]:
            h = clip_half(clip_half(core, 1, 0, hi), -1, 0, -lo)
            h = dedupe(h, 0.006)
            if len(h) >= 3 and abs(poly_area(h)) > 0.01:
                holes.append(ccw(h))
    if scale == 500:
        parts.append(CASE.plan("plate", plate_out, Y_PLATE_BOT, Y_PLATE_BOT + 0.020, "cast-iron", "plate", flags={"sheet": True}))
    else:
        parts.append(CASE.plan("plate", plate_out, Y_PLATE_BOT, Y_PLATE_TOP, "cast-iron", "plate", holesXZ=holes))
    strings = inner_region(per if scale != 500 else 1, -0.40, 0.04)
    parts.append(CASE.plan("strings", strings, Y_STR_BOT, Y_STR_TOP, "string-steel", "strings", flags={"sheet": True}, hatch={"plan": "strings" if scale in (50, 100) else None}))
    if scale in (50, 100):
        for key, prop in [("long", PROP_LONG), ("short", PROP_SHORT)]:
            xin = x_at(inner_path(per), prop["Z"], 1)
            parts.append(CASE.plan(f"prop-block-{key}", rect(prop["X"] - 0.012, prop["Z"] - 0.022, xin + 0.004, prop["Z"] + 0.022), 0.925, 0.958, "case-lacquer", "case"))
    parts.append(CASE.plan("wear-cheek", rect(0.625, -0.685, 0.715, -0.62), 0.80, 0.8012, "worn-finish", "worn-finish", role="texture", flags={"texture": True}, pen=0.2))
    if scale in (50, 100):
        parts.append(CASE.side("wear-key-slip", rect(-0.20, 0.665, 0.20, 0.695), Z_FRONT - 0.0012, Z_FRONT, "worn-finish", "worn-finish", role="texture", flags={"texture": True}, pen=0.2))
    if scale == 50:
        parts.append(CASE.end("dealer-sticker", rect(0.80, -0.50, 0.86, -0.42), HALF_W, HALF_W + 0.0012, "worn-finish", "worn-finish", role="texture", flags={"texture": True}, pen=0.2))
        parts.append(CASE.side("key-lock", rect(-0.012, 0.670, 0.012, 0.692), Z_FRONT - 0.003, Z_FRONT, "brass", "brass", role="detail"))
    return parts


def lid_parts(scale):
    per = {50: 4, 100: 2, 200: 1, 500: 1}[scale]
    out = outer_outline(per)
    main = clip_half(out, 0, -1, -Z_FLAP_BACK)
    t = 0.020 if scale != 500 else 0.024
    hatch = {50: "crazed-lacquer", 100: "crazed-lacquer"}.get(scale)
    parts = [LIDF.plan("lid", main, Y_LID_TOP - t, Y_LID_TOP, "lid-lacquer", "lid", member=True, flags={"sheet": True} if scale in (200, 500) else None, hatch={"plan": hatch, "front": hatch} if hatch else None)]
    flap = rect(-HALF_W, Z_FLAP_FRONT, HALF_W, Z_FLAP_BACK)
    if scale == 50:
        flap = [(-HALF_W, Z_FLAP_FRONT), (HALF_W - 0.016, Z_FLAP_FRONT), (HALF_W, Z_FLAP_FRONT + 0.013), (HALF_W, Z_FLAP_BACK), (-HALF_W, Z_FLAP_BACK)]
    tf = 0.0118 if scale in (200, 500) else t
    parts.append(FLAPF.plan("lid-flap", flap, Y_LID_TOP - tf, Y_LID_TOP, "lid-lacquer", "lid", flags={"sheet": True} if scale in (200, 500) else None, hatch={"plan": hatch} if hatch else None))
    if scale == 50:
        for i, z in enumerate([-0.25, 0.15, 0.55]):
            parts.append(LIDF.side(f"lid-hinge-{i + 1}", rect(-HALF_W, Y_LID_TOP, -HALF_W + 0.022, Y_LID_TOP + 0.008), z - 0.035, z + 0.035, "brass", "brass", role="detail"))
    parts.append(LIDF.plan("wear-lid-edge", rect(0.60, -0.26, 0.68, -0.06), Y_LID_TOP, Y_LID_TOP + 0.0012, "worn-finish", "worn-finish", role="texture", flags={"texture": True}, pen=0.2))
    return parts


def fall_desk_parts(scale):
    parts = []
    if scale == 500:
        return parts
    th = 0.012 if scale == 50 else 0.015
    if scale == 200:
        th = 0.030
    if scale != 200:
        parts.append(FALLF.plan("fallboard", rect(-KB_HALF + 0.0003, Z_FALL_HINGE - 0.150, KB_HALF - 0.0003, Z_FALL_HINGE), 0.744 - th / 2, 0.744 + th / 2, "case-lacquer", "case"))
    dt = 0.014 if scale != 200 else 0.030
    parts.append(DESKF.plan("desk", rect(-0.32, -0.56, 0.32, -0.29), 0.9525 - dt / 2 if scale != 200 else 0.9455, 0.9525 + dt / 2 if scale != 200 else 0.9595, "case-lacquer", "case", flags=None))
    return parts


def prop_parts(scale):
    parts = []
    for key, prop in [("prop-long", PROP_LONG), ("prop-short", PROP_SHORT)]:
        r = prop["r"]
        if scale == 500 and key == "prop-short":
            continue
        if scale == 200 and key == "prop-short":
            r = 0.015
        slender = scale in (200, 500) and key == "prop-long"
        n = 12 if scale == 50 else 8
        parts.append(Part(key, key, "vw", circle(0, 0, r, n), 0.0, prop["L"], "case-lacquer", "case", flags={"slender": True} if slender else None, member=True))
    return parts


def leg_parts(scale, key):
    parts = []
    oz = 0.0
    if scale == 50:
        parts.append(Part(f"{key}-collar", key, "vw", round_rect(-0.05, -0.05, 0.05, 0.05, 0.008, 2), 0.0, 0.065, "case-lacquer", "legs-lyre", member=True))
        h0, h1, u0, u1 = 0.0425, 0.030, 0.065, 0.555
        parts.append(leg_shaft(key, h0, h1, u0, u1, member=True))
        parts.append(Part(f"{key}-ferrule", key, "vw", rect(-0.032, -0.032, 0.032, 0.032), 0.555, 0.578, "brass", "brass"))
        parts.append(Part(f"{key}-horn", key, "vw", rect(-0.016, -0.016, 0.016, 0.016), 0.578, 0.600, "brass", "brass"))
        parts.append(Part(f"{key}-wheel", key, "uw", circle(0.600, 0.0, 0.025, 16), -0.011, 0.011, "brass", "brass"))
    elif scale == 100:
        parts.append(Part(f"{key}-collar", key, "vw", rect(-0.05, -0.05, 0.05, 0.05), 0.0, 0.065, "case-lacquer", "legs-lyre", member=True))
        parts.append(leg_shaft(key, 0.0425, 0.030, 0.065, 0.555, member=True))
        parts.append(Part(f"{key}-ferrule", key, "vw", rect(-0.032, -0.032, 0.032, 0.032), 0.555, 0.576, "brass", "brass"))
        parts.append(Part(f"{key}-castor", key, "uw", circle(0.600, 0.0, 0.025, 10), -0.016, 0.016, "brass", "brass"))
    elif scale == 200:
        parts.append(leg_shaft(key, 0.048, 0.032, 0.0, 0.575, member=True, mat="case-lacquer"))
        parts.append(Part(f"{key}-castor", key, "vw", rect(-0.02, -0.02, 0.02, 0.02), 0.575, LEG_LEN, "brass", "brass"))
    else:
        parts.append(leg_shaft(key, 0.048, 0.0385, 0.0, LEG_LEN, member=True, mat="case-lacquer", region="legs-lyre"))
    return parts


class ShaftPart(Part):
    """🦵 A square leg shaft tapering linearly along its bone: a frustum in 3D, a trapezoid swept with `zTip` in 2D."""

    def __init__(self, id, bone, h0, h1, u0, u1, mat, region, member):
        slope = (h1 - h0) / (u1 - u0)
        a0 = h0 - slope * u0
        aL = h0 + slope * (LEG_LEN - u0)
        super().__init__(id, bone, "uv", [(u0, -h0), (u1, -h1), (u1, h1), (u0, h0)], -a0, a0, mat, region, member=member)
        self.tip = (-aL, aL)
        self.h0, self.h1, self.u0, self.u1 = h0, h1, u0, u1

    def volume(self):
        return (self.u1 - self.u0) * 4 * (self.h0 ** 2 + self.h0 * self.h1 + self.h1 ** 2) / 3

    def centroid_local(self):
        a, b, L = self.h0 ** 2, self.h1 ** 2, self.u1 - self.u0
        t = (a + 2 * self.h0 * self.h1 + 3 * b) / (4 * (a + self.h0 * self.h1 + b))
        return [self.u0 + t * L, 0.0, 0.0]

    def local_points(self, both=True):
        out = []
        for u, h in [(self.u0, self.h0), (self.u1, self.h1)]:
            for sv in (-1, 1):
                for sw in (-1, 1):
                    out.append([u, sv * h, sw * h])
        return out

    def solid3d(self):
        return {"id": self.id, "kind": "extrude", "bone": self.bone, "material": self.mat, "role": self.role, "tone": R6(self.tone), "profile": [[R6(a), R6(b)] for a, b in ccw(rect(-self.h0, -self.h0, self.h0, self.h0))], "plane": "yz", "from": R6(self.u0), "to": R6(self.u1), "taperTo": [1, R6(self.h1 / self.h0)]}


def leg_shaft(key, h0, h1, u0, u1, member=False, mat="case-lacquer", region="legs-lyre"):
    return ShaftPart(f"{key}-shaft", key, h0, h1, u0, u1, mat, region, member)


def lyre_parts(scale):
    parts = []
    if scale in (50, 100):
        parts.append(Part("lyre-top-bar", "lyre", "uv", rect(0.0, -0.13, 0.05, 0.13), -0.03, 0.03, "case-lacquer", "legs-lyre", member=True))
        for s in (-1, 1):
            if scale == 50:
                outer = [(0.05, 0.065 * s), (0.15, 0.092 * s), (0.30, 0.104 * s), (0.45, 0.094 * s), (0.53, 0.078 * s)]
            else:
                outer = [(0.05, 0.065 * s), (0.30, 0.104 * s), (0.53, 0.078 * s)]
            inner = [(u, v - 0.032 * s) for u, v in outer]
            parts.append(Part(f"lyre-arm-{'treble' if s > 0 else 'bass'}", "lyre", "uv", ccw(outer + list(reversed(inner))), -0.016, 0.016, "case-lacquer", "legs-lyre", member=True))
        parts.append(Part("pedal-box", "lyre", "uv", rect(0.53, -0.11, 0.605, 0.11), -0.05, 0.05, "case-lacquer", "legs-lyre"))
        for s in (-1, 1):
            a, b = (0.565, 0.035), (0.0, 0.145)
            parts.append(Part(f"lyre-brace-{'treble' if s > 0 else 'bass'}", "lyre", "uw", bar(a, b, 0.014), 0.07 * s - 0.007, 0.07 * s + 0.007, "brass", "brass", flags={"slender": True}))
    elif scale == 200:
        prof = [(0.0, -0.13), (0.05, -0.13), (0.05, -0.10), (0.53, -0.10), (0.53, -0.11), (0.605, -0.11), (0.605, 0.11), (0.53, 0.11), (0.53, 0.10), (0.05, 0.10), (0.05, 0.13), (0.0, 0.13)]
        parts.append(Part("lyre-frame", "lyre", "uv", prof, -0.03, 0.03, "case-lacquer", "legs-lyre", member=True))
        parts.append(Part("pedal-box", "lyre", "uv", rect(0.53, -0.11, 0.605, 0.11), -0.05, 0.05, "case-lacquer", "legs-lyre"))
        parts.append(Part("pedals", "lyre", "uw", rect(0.525, -0.17, 0.57, -0.05), -0.09, 0.09, "brass", "brass"))
        parts.append(Part("lyre-brace-pair", "lyre", "uw", bar((0.565, 0.035), (0.0, 0.145), 0.014), -0.077, 0.077, "case-lacquer", "legs-lyre", flags={"slender": True}))
    else:
        prof = [(0.0, -0.13), (0.075, -0.13), (0.075, -0.0375), (0.53, -0.0375), (0.53, -0.11), (0.605, -0.11), (0.605, 0.11), (0.53, 0.11), (0.53, 0.0375), (0.075, 0.0375), (0.075, 0.13), (0.0, 0.13)]
        parts.append(Part("lyre-frame", "lyre", "uv", prof, -0.0375, 0.0375, "case-lacquer", "legs-lyre", member=True))
        parts.append(Part("pedals", "lyre", "uw", rect(0.53, -0.17, 0.605, -0.0375), -0.0375, 0.0375, "brass", "brass"))
    return parts


def pedal_parts(scale):
    parts = []
    if scale not in (50, 100):
        return parts
    for key in PEDALS:
        th = 0.012 if scale == 50 else 0.015
        if scale == 50:
            prof = [(0.0, -th / 2), (0.11, -th / 2), (0.165, -0.004), (0.172, 0.004), (0.165, 0.010), (0.11, th / 2), (0.0, th / 2)]
        else:
            prof = [(0.0, -th / 2), (0.17, -th / 2), (0.17, th / 2), (0.0, th / 2)]
        parts.append(Part(key, key, "uv", prof, -0.016, 0.016, "brass", "brass"))
    return parts


def skid_parts(scale):
    parts = []
    hz = SKID_L / 2
    slots = [rect(0.14, s * 0.80 - 0.0175, 0.26, s * 0.80 + 0.0175) for s in (-1, 1)]
    if scale in (50, 100):
        parts.append(Part("skid-board", "skid", "uw", rect(0.0, -hz, SKID_W, hz), 0.020, 0.045 if scale == 50 else 0.055, "plywood", "skid", holes=slots, member=True))
        if scale == 50:
            parts.append(Part("skid-carpet", "skid", "uw", rect(0.0, -hz, SKID_W, hz), 0.045, 0.055, "carpet", "skid", holes=slots, hatch={"plan": "carpet"}))
        for s in (-1, 1):
            parts.append(Part(f"skid-batten-{'tail' if s > 0 else 'key'}", "skid", "uw", rect(0.01, s * 0.75 - 0.03, SKID_W - 0.01, s * 0.75 + 0.03), 0.0, 0.020, "plywood", "skid", member=True))
    else:
        parts.append(Part("skid-board", "skid", "uw", rect(0.0, -hz, SKID_W, hz), 0.020 if scale == 200 else 0.0, 0.055, "plywood", "skid", member=True))
        if scale == 200:
            for s in (-1, 1):
                parts.append(Part(f"skid-batten-{'tail' if s > 0 else 'key'}", "skid", "uw", rect(0.01, s * 0.75 - 0.03, SKID_W - 0.01, s * 0.75 + 0.03), 0.0, 0.020, "plywood", "skid", member=True))
    for s in ["a", "b"]:
        zs = STRAP_Z[s]
        parts.append(Part(f"strap-{s}-under", "skid", "uw", rect(A_PT[0] - SKID_X0, zs - STRAP_W / 2, D_PT[0] - SKID_X0, zs + STRAP_W / 2), 0.0, STRAP_T, "webbing", "straps", flags={"slender": True}))
    return parts


def strap_parts(scale):
    parts = []
    for s in ["a", "b"]:
        for seg, L in [("up", UP_LEN), ("top", TOP_LEN), ("down", DOWN_LEN)]:
            parts.append(Part(f"strap-{s}-{seg}", f"strap-{s}-{seg}", "uv", rect(0.0, -STRAP_T / 2, L, STRAP_T / 2), -STRAP_W / 2, STRAP_W / 2, "webbing", "straps", flags={"slender": True}))
        if scale in (50, 100):
            parts.append(Part(f"strap-{s}-ratchet", f"strap-{s}-top", "uv", rect(0.10, STRAP_T / 2, 0.20, STRAP_T / 2 + 0.028), -0.032, 0.032, "webbing", "straps", role="detail"))
    return parts


def all_parts(scale):
    parts = case_parts(scale) + lid_parts(scale) + fall_desk_parts(scale) + prop_parts(scale) + lyre_parts(scale) + pedal_parts(scale) + skid_parts(scale) + strap_parts(scale)
    for key in LEGS:
        parts += leg_parts(scale, key)
    for p in parts:
        while PENS[scale][p.role] is None:
            p.role = {"texture": "detail", "detail": "body"}[p.role]
    return parts


# ----------------------------------------------------------------------------------------------- drawing order of the case parts (bottom to top) in the front elevation and the plan
import fnmatch

FRONT_ORDER = ["lid", "lid-hinge-*", "wear-lid-edge", "lid-flap", "prop-long", "prop-short", "desk", "plate", "plate-opening-*", "strings", "action", "nameboard", "desk-rail", "front-body", "prop-block-*", "belly-rail", "soundboard", "rib-*", "beam-*", "tail-leg-block", "fallboard", "black-key*", "keys", "white-keys", "keyframe", "key-slip", "key-lock", "wear-key-slip", "cheek-*", "wear-cheek", "end-block-*", "keyboard-end", "rim", "dealer-sticker", "keybed"]
PLAN_ORDER = ["tail-leg-block", "beam-*", "rib-*", "soundboard", "keybed", "keyframe", "key-slip", "key-lock", "wear-key-slip", "white-keys", "keys", "black-key*", "fallboard", "cheek-*", "wear-cheek", "keyboard-end", "strings", "belly-rail", "action", "plate", "plate-opening-*", "front-body", "nameboard", "desk-rail", "desk", "prop-short", "prop-long", "prop-block-*", "end-block-*", "rim", "dealer-sticker", "lid", "lid-hinge-*", "wear-lid-edge", "lid-flap"]
BANDS = {"front": (-0.11, 0.185), "plan": (0.62, 1.482)}


def order_index(order, id):
    for i, pat in enumerate(order):
        if fnmatch.fnmatch(id, pat):
            return i
    return None


def nearness(view, pts, id):
    if view == "elevation":
        return -min(p[2] for p in pts)
    order = FRONT_ORDER if view == "front" else PLAN_ORDER
    i = order_index(order, id)
    if i is not None:
        lo, hi = BANDS[view]
        return lo + (hi - lo) * (i + 0.5) / len(order)
    return max(p[0] for p in pts) if view == "front" else max(p[1] for p in pts)


PENS = {50: {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": None, "ground": 0.7}, 100: {"body": 0.35, "detail": 0.2, "texture": 0.13, "glazing": None, "ground": 0.5}, 200: {"body": 0.25, "detail": 0.2, "texture": None, "glazing": None, "ground": 0.35}, 500: {"body": 0.2, "detail": None, "texture": None, "glazing": None, "ground": 0.25}}
HATCHES = {
    "crazed-lacquer": {50: dict(pattern="stipple", angleDeg=0, spacingMm=0.9, penMm=0.1), 100: dict(pattern="stipple", angleDeg=0, spacingMm=1.2, penMm=0.1), "title": T("Crazed lacquer", "Craquelierter Lack")},
    "key-joints": {50: dict(pattern="lines", angleDeg=90, spacingMm=0.47, penMm=0.1), "title": T("Key joints", "Tastenfugen")},
    "key-joints-100": {100: dict(pattern="lines", angleDeg=90, spacingMm=0.47, penMm=0.1), "title": T("Key joints, every second joint", "Tastenfugen, jede zweite Fuge")},
    "black-key-row": {100: dict(pattern="lines", angleDeg=90, spacingMm=0.42, penMm=0.15), "title": T("Row of black keys", "Reihe der Obertasten")},
    "strings": {50: dict(pattern="lines", angleDeg=90, spacingMm=0.45, penMm=0.1), 100: dict(pattern="lines", angleDeg=90, spacingMm=0.5, penMm=0.1), "title": T("Strings", "Saiten")},
    "carpet": {50: dict(pattern="stipple", angleDeg=0, spacingMm=0.6, penMm=0.1), "title": T("Needle-felt carpet", "Nadelfilz")},
}


def details2d(scale):
    """✍️ Flat drawing-only strokes of the side elevation on bones that turn only about the lateral axis."""
    out = []
    if scale != 50:
        return out
    ww = 2 * KB_HALF / 52
    segs = []
    for i in range(1, 52):
        u = -KB_HALF + i * ww - CASE.ox
        segs.append(f"M {R6(u)} {R6(Y_WHITE_BOT - CASE.oy + 0.002)} L {R6(u)} {R6(Y_WHITE_TOP - CASE.oy - 0.001)}")
    out.append(("white-key-joints", "white-keys", {"id": "white-key-joints", "bone": "case", "role": "texture", "z": 0, "filled": False, "tone": 0.3, "region": "keys", "material": "key-ivorine", "kind": "path", "d": " ".join(segs)}))
    scr = []
    for (u0, v0, u1, v1) in [(0.66, 0.70, 0.70, 0.74), (0.645, 0.735, 0.69, 0.765), (0.675, 0.676, 0.705, 0.70)]:
        scr.append(f"M {R6(u0 - CASE.ox)} {R6(v0 - CASE.oy)} L {R6(u1 - CASE.ox)} {R6(v1 - CASE.oy)}")
    out.append(("cheek-scratches", "cheek-treble", {"id": "cheek-scratches", "bone": "case", "role": "texture", "z": 0, "filled": False, "tone": 0.6, "region": "worn-finish", "material": "worn-finish", "kind": "path", "d": " ".join(scr)}))
    return out


def plate_openings(scale, parts):
    plate = next(p for p in parts if p.id == "plate")
    out = []
    for i, h in enumerate(plate.holes):
        out.append(Part(f"plate-opening-{i + 1}", "case", "uw", h, plate.hi - 0.001, plate.hi, "string-steel", "strings", role="texture", hatch={"plan": "strings"}, views=("plan",)))
    return out


def build_shapes(scale, parts, world):
    views = {"elevation": [], "front": [], "plan": []}
    extra = plate_openings(scale, parts) if scale in (50, 100) else []
    host_near = {}
    for p in parts + extra:
        oz = world[p.bone][0][2]
        pts = [to_world(world, p.bone, q) for q in p.local_points()]
        for view in p.views:
            sh = p.shape2d(view, oz)
            if sh.get("hatch") and scale not in HATCHES[sh["hatch"]]:
                sh.pop("hatch")
            near = nearness(view, pts, p.id)
            host_near[(view, p.id)] = near
            views[view].append((near, sh))
    for (id, host, sh) in details2d(scale):
        views["elevation"].append((host_near[("elevation", host)] + 1e-4, sh))
    out = {}
    for view, items in views.items():
        items.sort(key=lambda t: t[0])
        shapes = []
        for i, (near, sh) in enumerate(items):
            sh["z"] = round(1 + 0.01 * i, 4)
            shapes.append(sh)
        out[view] = shapes
    used = sorted({sh["hatch"] for v in out.values() for sh in v if sh.get("hatch")})
    hatches = [dict(id=h, title=HATCHES[h]["title"], **HATCHES[h][scale]) for h in used]
    return out, hatches


# ----------------------------------------------------------------------------------------------- volumes
def js_of(scale):
    return {50: 0.02, 100: 0.025, 200: 0.03, 500: 0.075}[scale]


def box(id, bone, center, size, mat, role="detail", tone=0.5, **kw):
    s = {"id": id, "kind": "box", "bone": bone, "material": mat, "role": role, "tone": tone, "size": [R6(v) for v in size], "transform": {"translation": [R6(v) for v in center]}}
    s.update(kw)
    return s


def build_volume(scale, parts, masses):
    body = [p.solid3d() for p in parts]
    hidden = {r["id"] for r in REGIONS if r.get("hidden")}
    thermal = []
    for p in parts:
        if p.region in hidden:
            t = p.solid3d()
            t["id"] = f"th-{p.id}"
            t["region"] = p.region
            thermal.append(t)
        else:
            thermal.append({"id": f"th-{p.id}", "of": p.id, "region": p.region})
    structure = [{"id": f"st-{p.id}", "of": p.id, "member": p.bone, "part": "member"} for p in parts if p.member]
    js = js_of(scale)
    structure.append(box("joint-case-skid", "case", [0, js / 2, 0], [js, js, js], "case-lacquer", member="case", part="joint"))
    structure.append(box("joint-lid-hinge", "lid", [js / 2, -js / 2, 0], [js, js, js], "brass", member="lid", part="joint"))
    for key, prop in [("prop-long", PROP_LONG), ("prop-short", PROP_SHORT)]:
        loc = case_local(prop["X"], prop["Y"], prop["Z"])
        xin = x_at(offset_path(side_path(4), RIM_T if scale != 500 else 0.075), prop["Z"], 1) - CASE.ox
        u0, u1 = loc[0] - js / 2, xin + 0.01
        structure.append(box(f"joint-{key}", "case", [(u0 + u1) / 2, loc[1], prop["Z"]], [u1 - u0, js, js], "case-lacquer", member=key, part="joint"))
    if scale == 500:
        structure.append({"id": "st-prop-short", "kind": "extrude", "bone": "prop-short", "material": "case-lacquer", "role": "body", "tone": 0.12, "profile": [[R6(a), R6(b)] for a, b in ccw(circle(0, 0, PROP_SHORT["r"], 8))], "plane": "yz", "from": 0, "to": R6(PROP_SHORT["L"]), "slender": True, "member": "prop-short", "part": "member"})
    for key in LEGS:
        structure.append(box(f"joint-{key}", key, [js / 2, 0, 0], [js, js, js], "brass", member=key, part="joint"))
    structure.append(box("joint-lyre", "lyre", [js / 2, 0, 0], [js, js, js], "brass", member="lyre", part="joint"))
    ms = {50: 0.03, 100: 0.035, 200: 0.04, 500: 0.075}[scale]
    for b in ["case", "lid", "skid", "lyre"] + list(LEGS):
        m = dict(masses[b])
        own = [p for p in parts if p.bone == b]
        inside = any(all(min(q[i] for q in p.local_points()) - 0.002 <= c <= max(q[i] for q in p.local_points()) + 0.002 for i, c in enumerate((m["comX"], m["comY"], m["comZ"]))) for p in own)
        if not inside and own:
            heavy = max(own, key=lambda p: p.volume() * DENSITY[p.mat])
            m["comX"], m["comY"], m["comZ"] = heavy.centroid_local()
        structure.append({"id": f"mass-{b}", "kind": "ellipsoid", "bone": b, "material": "cast-iron" if b == "case" else "case-lacquer", "role": "detail", "tone": 0.5, "radii": [ms / 2] * 3, "transform": {"translation": [R6(m["comX"]), R6(m["comY"]), R6(m["comZ"])]}, "member": b, "part": "mass"})
    return body, thermal, structure


VOLUME_DECISIONS = {
    50: [T("Body: the real baby grand built part by part in the stored pose: the 70 mm laminated rim as a U ring of the true bentside, the 35 mm keybed, two cheek blocks with rounded front, end blocks, key slip with lock and worn patch, 52 white keys as one repeated key with 1 mm joints, 36 black keys in six repeated rows, nameboard, desk rail, action and key frame blocks, belly rail and pin block, a 9 mm soundboard with seven ribs, three beams and the tail leg block, the 30 mm cast-iron plate with three openings over the string layer, two prop blocks; the 20 mm main lid with three brass hinges, the front flap with its chipped corner, the closed fallboard, the folded music desk, two props folded under the lid; three legs of collar, tapered shaft, brass ferrule, castor horn and wheel lying beside the skid, the lyre with curved arms, pedal box, three pedals and two brass braces standing against the keybed; the padded skid of board, carpet and two battens with hand slots; two ratchet straps of three runs each and their run under the board.", "Körper: der echte Stutzflügel Teil für Teil in der Lagerpose gebaut: die 70 mm starke schichtverleimte Rast als U-Ring der wahren Biegung, der 35-mm-Klaviaturboden, zwei Backen mit gerundeter Front, Endklötze, Vorsatzleiste mit Schloss und abgegriffener Stelle, 52 weiße Tasten als eine wiederholte Taste mit 1-mm-Fugen, 36 Obertasten in sechs wiederholten Reihen, Namensbrett, Pultleiste, Mechanik- und Tastenrahmenblock, Anhängestock und Stimmstock, ein 9-mm-Resonanzboden mit sieben Rippen, drei Rastenbalken und der Klotz des hinteren Beins, die 30-mm-Gussplatte mit drei Öffnungen über der Saitenlage, zwei Stabklötze; der 20-mm-Hauptdeckel mit drei Messingscharnieren, der Vorderdeckel mit abgeplatzter Ecke, die geschlossene Tastenklappe, das umgelegte Notenpult, zwei unter dem Deckel liegende Stäbe; drei Beine aus Kragen, verjüngtem Schaft, Messingschuh, Rollengabel und Rolle neben dem Brett, die Lyra mit geschwungenen Armen, Pedalkasten, drei Pedalen und zwei Messingstreben am Klaviaturboden; das gepolsterte Brett aus Platte, Teppich und zwei Querleisten mit Griffschlitzen; zwei Ratschengurte mit je drei Läufen und dem Lauf unter dem Brett."),
         T("Thermal: every region is the skin of its body solids by reference; plate and strings are hidden inside the closed case and count their whole surface; the worn patches and the dealer's sticker are texture solids of their own region.", "Thermik: jede Region ist die Haut ihrer Körper als Verweis; Platte und Saiten liegen verborgen im geschlossenen Gehäuse und zählen ihre ganze Fläche; die abgegriffenen Stellen und der Händleraufkleber sind Texturkörper ihrer eigenen Region."),
         T("Structure: rim, keybed, belly rail and beams are the members of the case, the lid board, both props, the leg shafts and collars, the lyre arms and top bar, the skid board and battens are members of their bones; joints at the spine on the skid, the lid hinge, the prop pivots, the leg tops and the lyre top; mass markers at the centres of mass of case, lid, skid, lyre and legs.", "Tragwerk: Rast, Klaviaturboden, Anhängestock und Rastenbalken sind die Stäbe des Gehäuses, Deckelplatte, beide Stäbe, Beinschäfte und Kragen, Lyraarme und Querholz, Brett und Querleisten sind Stäbe ihrer Knochen; Knoten an der Rast auf dem Brett, am Deckelscharnier, an den Stabdrehpunkten, an den Beinköpfen und am Lyrakopf; Massenmarken in den Schwerpunkten von Gehäuse, Deckel, Brett, Lyra und Beinen.")],
    100: [T("Body: individually modelled for 15 mm: the white keys become one block with the joints as a hatch, the black keys a texture row, carpet and board one 35 mm padded board, the ribs, hinges, lock, sticker and key slip patch are left out, the castors become one cylinder, lyre arms and pedals are drawn with fewer points and 15 mm pedals; every other part keeps its true section.", "Körper: einzeln für 15 mm modelliert: die weißen Tasten werden ein Block mit den Fugen als Schraffur, die Obertasten eine Texturreihe, Teppich und Brett ein 35-mm-Polsterbrett, Rippen, Scharniere, Schloss, Aufkleber und Vorsatzleisten-Fleck entfallen, die Rollen werden je ein Zylinder, Lyraarme und Pedale mit weniger Punkten und 15-mm-Pedalen; jedes andere Teil behält seinen wahren Querschnitt."),
          T("Thermal: regions by reference to the body; plate and strings hidden.", "Thermik: Regionen als Verweis auf den Körper; Platte und Saiten verborgen."),
          T("Structure: as at 1:50 without the separate key parts; joints and masses enlarged to 20 mm markers.", "Tragwerk: wie 1:50 ohne die einzelnen Tastenteile; Knoten und Massen als 20-mm-Marken.")],
    200: [T("Body: for 30 mm the keyboard end becomes one end block (cheeks, key slip and keybed front in their true outline) with the key row as a 12 mm deep layer, the action, nameboard and desk rail one front body, the fallboard is merged into it, the desk is 30 mm thick, the short prop 30 mm round, the long prop keeps its 20 mm section as a slender rod, the legs are one tapered shaft with a brass castor block, the lyre one frame with a pedal block and one slender brace pair; lid, flap, plate, strings and soundboard keep their gauge as declared sheets.", "Körper: für 30 mm wird das Klaviaturende ein Endblock (Backen, Vorsatzleiste und Klaviaturbodenfront in wahrem Umriss) mit der Tastenreihe als 12 mm tiefe Lage, Mechanik, Namensbrett und Pultleiste ein Frontkörper, die Tastenklappe geht darin auf, das Pult ist 30 mm stark, der kurze Stab 30 mm rund, der lange Stab behält seinen 20-mm-Querschnitt als schlanker Stab, die Beine sind je ein verjüngter Schaft mit Messingrollenblock, die Lyra ein Rahmen mit Pedalblock und einem schlanken Strebenpaar; Deckel, Vorderdeckel, Platte, Saiten und Resonanzboden behalten ihre Stärke als erklärte Platten."),
          T("Thermal: regions by reference; only the sum of the areas is compared at this scale.", "Thermik: Regionen als Verweis; in diesem Maßstab wird nur die Flächensumme verglichen."),
          T("Structure: rim, keyboard end, keybed, belly rail and beams for the case, lid sheet, slender long prop, short prop, leg shafts, lyre frame, skid board and battens; 30 mm joints and mass markers.", "Tragwerk: Rast, Klaviaturende, Klaviaturboden, Anhängestock und Rastenbalken für das Gehäuse, Deckelplatte, schlanker langer Stab, kurzer Stab, Beinschäfte, Lyrarahmen, Brett und Querleisten; 30-mm-Knoten und Massenmarken.")],
    500: [T("Body: for 75 mm the rim is 75 mm thick, the keyboard end, front body, keybed and three 75 mm beams are the masses of the case, the lid and flap keep their 24 mm sheet, plate, strings and soundboard are declared sheets inside, the keys a texture layer, the legs one tapered shaft to the floor 96 to 77 mm, the lyre one 75 mm frame with a pedal block, the long prop a slender rod, the short prop, fallboard and desk are left out (under the feature, inside the case or merged into the front body), the skid one 55 mm board; straps stay slender at their true section.", "Körper: für 75 mm ist die Rast 75 mm stark, Klaviaturende, Frontkörper, Klaviaturboden und drei 75-mm-Balken sind die Massen des Gehäuses, Deckel und Vorderdeckel behalten ihre 24-mm-Platte, Platte, Saiten und Resonanzboden sind erklärte Platten im Inneren, die Tasten eine Texturlage, die Beine je ein verjüngter Schaft bis zum Boden von 96 auf 77 mm, die Lyra ein 75-mm-Rahmen mit Pedalblock, der lange Stab ein schlanker Stab, kurzer Stab, Tastenklappe und Pult entfallen (unter dem Merkmal, im Gehäuse oder im Frontkörper aufgegangen), das Brett eine 55-mm-Platte; die Gurte bleiben schlank in wahrem Querschnitt."),
          T("Thermal: regions by reference; only the sum of the areas is compared.", "Thermik: Regionen als Verweis; nur die Flächensumme wird verglichen."),
          T("Structure: the members of the case masses, lid sheet, slender props (the short prop as a structure rod only), leg shafts, lyre frame and skid board; 75 mm joints and mass markers.", "Tragwerk: die Stäbe der Gehäusemassen, Deckelplatte, schlanke Stäbe (der kurze Stab nur als Tragwerksstab), Beinschäfte, Lyrarahmen und Brett; 75-mm-Knoten und Massenmarken.")],
}


# ----------------------------------------------------------------------------------------------- masses and sections of the bones (from the 1:50 parts: volume x density)
SECTIONS = {
    "case": dict(areaM2=2 * RIM_T * (Y_RIM_TOP - Y_RIM_BOT), secondMomentM4=2 * RIM_T * (Y_RIM_TOP - Y_RIM_BOT) ** 3 / 12, extremeFibreM=(Y_RIM_TOP - Y_RIM_BOT) / 2, material="case-lacquer", title=T("Two rim walls 70 x 305 mm (laminated beech)", "Zwei Rastwände 70 x 305 mm (Buche schichtverleimt)")),
    "lid": dict(areaM2=1.40 * 0.020, secondMomentM4=1.40 * 0.020 ** 3 / 12, extremeFibreM=0.010, material="lid-lacquer", title=T("Lid board 1.40 m x 20 mm", "Deckelplatte 1,40 m x 20 mm")),
    "skid": dict(areaM2=SKID_W * 0.025, secondMomentM4=SKID_W * 0.025 ** 3 / 12, extremeFibreM=0.0125, material="plywood", title=T("Birch plywood board 400 x 25 mm", "Birkensperrholzplatte 400 x 25 mm")),
    "prop-long": dict(areaM2=math.pi * 0.010 ** 2, secondMomentM4=math.pi * 0.020 ** 4 / 64, extremeFibreM=0.010, material="case-lacquer", title=T("Round beech stick 20 mm", "Runder Buchenstab 20 mm")),
    "prop-short": dict(areaM2=math.pi * 0.009 ** 2, secondMomentM4=math.pi * 0.018 ** 4 / 64, extremeFibreM=0.009, material="case-lacquer", title=T("Round beech stick 18 mm", "Runder Buchenstab 18 mm")),
    "lyre": dict(areaM2=2 * 0.032 * 0.032, secondMomentM4=2 * 0.032 * 0.032 ** 3 / 12, extremeFibreM=0.016, material="case-lacquer", title=T("Two lyre arms 32 x 32 mm", "Zwei Lyraarme 32 x 32 mm")),
}
for _k in LEGS:
    SECTIONS[_k] = dict(areaM2=0.072 ** 2, secondMomentM4=0.072 ** 4 / 12, extremeFibreM=0.036, material="case-lacquer", title=T("Square beech leg tapering 85 to 60 mm", "Quadratisches Buchenbein, verjüngt von 85 auf 60 mm"))


def bone_masses(parts):
    acc = {}
    for p in parts:
        m = p.volume() * DENSITY[p.mat]
        c = p.centroid_local()
        a = acc.setdefault(p.bone, [0.0, 0.0, 0.0, 0.0])
        a[0] += m
        for i in range(3):
            a[i + 1] += m * c[i]
    out = {}
    for b, (m, x, y, z) in acc.items():
        if m > 0:
            out[b] = dict(kg=m, comX=x / m, comY=y / m, comZ=z / m)
    return out


# ----------------------------------------------------------------------------------------------- anchors
def anchors():
    out = []
    for i, (u, w) in enumerate([(0.02, -0.75), (0.38, -0.75), (0.02, 0.75), (0.38, 0.75)]):
        out.append({"id": f"batten-{['key-lid', 'key-bottom', 'tail-lid', 'tail-bottom'][i]}", "bone": "skid", "x": u, "y": 0.0, "z": w, "kind": "contact", "support": "contact", "footprintLengthM": 0.04, "footprintWidthM": 0.06, "frictionCoefficient": 0.5, "title": T(["Batten end under the keyboard end, lid side", "Batten end under the keyboard end, underside side", "Batten end under the tail end, lid side", "Batten end under the tail end, underside side"][i], ["Leistenende unter dem Klaviaturende, Deckelseite", "Leistenende unter dem Klaviaturende, Bodenseite", "Leistenende unter dem Schwanzende, Deckelseite", "Leistenende unter dem Schwanzende, Bodenseite"][i])})
    for i, (u, w) in enumerate([(0.14, -0.80), (0.26, -0.80), (0.14, 0.80), (0.26, 0.80)]):
        out.append({"id": f"grip-{i + 1}", "bone": "skid", "x": u, "y": 0.0325, "z": w, "kind": "support", "support": "contact", "normalDeg": 90, "footprintLengthM": 0.06, "footprintWidthM": 0.035, "frictionCoefficient": 0.8, "title": T(f"Mover's hand in the {'keyboard' if w < 0 else 'tail'}-end slot, {'lid' if u < 0.2 else 'underside'} side", f"Hand eines Möbelpackers im Griffschlitz am {'Klaviatur' if w < 0 else 'Schwanz'}ende, {'Deckel' if u < 0.2 else 'Boden'}seite")})
    for k, leg in LEGS.items():
        out.append({"id": f"{k}-castor", "bone": k, "x": LEG_LEN, "y": 0.0, "z": 0.0, "kind": "contact", "support": "contact", "footprintLengthM": 0.012, "footprintWidthM": 0.022, "frictionCoefficient": 0.05, "title": T(f"{leg['title']['en']}: castor on the floor", f"{leg['title']['de']}: Rolle auf dem Boden")})
    out.append({"id": "leg-bass-resting", "bone": "leg-bass", "x": 0.03, "y": 0.05, "z": 0.0, "kind": "contact", "support": "contact", "footprintLengthM": 0.065, "footprintWidthM": 0.10, "frictionCoefficient": 0.5, "title": T("Bass leg lying on the floor beside the skid (collar face)", "Bassbein liegt neben dem Brett am Boden (Kragenfläche)")})
    out.append({"id": "leg-tail-resting", "bone": "leg-tail", "x": 0.03, "y": -0.05, "z": 0.0, "kind": "contact", "support": "contact", "footprintLengthM": 0.065, "footprintWidthM": 0.10, "frictionCoefficient": 0.5, "title": T("Tail leg lying on the floor beside the skid (collar face)", "Hinteres Bein liegt neben dem Brett am Boden (Kragenfläche)")})
    out.append({"id": "lyre-standing", "bone": "lyre", "x": 0.605, "y": 0.0, "z": 0.0, "kind": "contact", "support": "contact", "footprintLengthM": 0.10, "footprintWidthM": 0.22, "frictionCoefficient": 0.5, "title": T("Pedal box of the removed lyre standing on the floor", "Pedalkasten der abgenommenen Lyra steht am Boden")})
    for key, X in [("bass", -0.30), ("treble", 0.25)]:
        c = case_local(X, Y_WHITE_TOP, -0.70)
        out.append({"id": f"hand-{key}", "bone": "case", "x": R6(c[0]), "y": R6(c[1]), "z": -0.70, "kind": "load", "title": T(f"{'Left' if key == 'bass' else 'Right'} hand on the keys", f"{'Linke' if key == 'bass' else 'Rechte'} Hand auf den Tasten")})
    out.append({"id": "sustain-foot", "bone": "pedal-sustain", "x": 0.15, "y": 0.006, "z": 0.0, "kind": "load", "title": T("Right foot on the sustain pedal", "Rechter Fuß auf dem Fortepedal")})
    out.append({"id": "music-on-desk", "bone": "desk", "x": 0.32, "y": 0.007, "z": 0.13, "kind": "load", "title": T("Sheet music on the desk", "Noten auf dem Pult")})
    return out


# ----------------------------------------------------------------------------------------------- thermal model
REGIONS = [
    dict(id="case", title=T("Case: rim, cheeks, nameboard, desk rail, fallboard and desk in crazed black lacquer", "Gehäuse: Rast, Backen, Namensbrett, Pultleiste, Tastenklappe und Pult in craqueliertem schwarzem Lack"), material="case-lacquer", geometry="vertical-plate", L=0.30, C=0.065 * 720 * 1600, layer=("lacquer-film", T("Polyester lacquer film 0.3 mm, crazed", "Polyesterlackfilm 0,3 mm, craqueliert"), "polymer", 0.0015, 450), power=True),
    dict(id="lid", title=T("Lid and front flap, black lacquer worn at the lifting edge", "Deckel und Vorderdeckel, schwarzer Lack, an der Hebekante abgegriffen"), material="lid-lacquer", geometry="horizontal-plate-up", L=1.40, C=0.020 * 560 * 1600, layer=("lacquer-film", T("Polyester lacquer film 0.3 mm", "Polyesterlackfilm 0,3 mm"), "polymer", 0.0015, 450)),
    dict(id="keys", title=T("Keys, key frame and action (yellowed key tops)", "Tasten, Tastenrahmen und Mechanik (vergilbte Tastenbeläge)"), material="key-ivorine", geometry="horizontal-plate-up", L=0.15, C=0.022 * 520 * 1500, layer=("key-top", T("Acrylic key top 2 mm, yellowed", "Acryl-Tastenbelag 2 mm, vergilbt"), "polymer", 0.010, 2800)),
    dict(id="plate", title=T("Cast-iron plate with bronze lacquer", "Gussplatte mit Bronzelack"), material="cast-iron", geometry="horizontal-plate-up", L=1.20, C=0.030 * 7200 * 460, layer=("bronze-lacquer", T("Bronze lacquer", "Bronzelack"), "metal", 0.0002, 100), hidden=T("Inside the closed case under the lid in the stored pose; seen only through the open lid when played.", "In der Lagerpose im geschlossenen Gehäuse unter dem Deckel; nur bei offenem Deckel im Spiel zu sehen.")),
    dict(id="strings", title=T("Strings (music wire and copper-wound bass strings)", "Saiten (Klaviersaitendraht und kupferumsponnene Basssaiten)"), material="string-steel", geometry="horizontal-plate-up", L=1.00, C=0.006 * 1300 * 470, layer=("wire", T("Steel and copper wire", "Stahl- und Kupferdraht"), "metal", 0.0001, 50), hidden=T("Between plate and soundboard inside the closed case.", "Zwischen Platte und Resonanzboden im geschlossenen Gehäuse.")),
    dict(id="soundboard", title=T("Spruce soundboard, underside and top", "Fichtenresonanzboden, Unter- und Oberseite"), material="spruce", geometry="horizontal-plate-down", L=1.00, C=0.009 * 450 * 1700, layer=("shellac", T("Thin shellac, darkened", "Dünner Schellack, nachgedunkelt"), "timber", 0.0005, 80)),
    dict(id="bracing", title=T("Bracing in raw wood: keybed, belly rail, beams, ribs and leg block", "Verstrebung in rohem Holz: Klaviaturboden, Anhängestock, Rastenbalken, Rippen und Beinklotz"), material="beech-raw", geometry="horizontal-plate-down", L=0.30, C=0.035 * 700 * 1600, layer=("raw-wood", T("Raw beech, dusty", "Rohe Buche, verstaubt"), "timber", 0.001, 200)),
    dict(id="legs-lyre", title=T("Legs and pedal lyre in black lacquer", "Beine und Pedallyra in schwarzem Lack"), material="case-lacquer", geometry="vertical-plate", L=0.08, C=0.036 * 720 * 1600, layer=("lacquer-film", T("Polyester lacquer film 0.3 mm", "Polyesterlackfilm 0,3 mm"), "polymer", 0.0015, 450)),
    dict(id="brass", title=T("Tarnished brass: castors, ferrules, pedals, hinges, braces and lock", "Angelaufenes Messing: Rollen, Beinschuhe, Pedale, Scharniere, Streben und Schloss"), material="brass", geometry="cylinder", L=0.03, C=0.005 * 8500 * 380, layer=("tarnish", T("Tarnish film", "Anlaufschicht"), "metal", 0.00005, 10)),
    dict(id="skid", title=T("Skid: plywood board with grey carpet padding and battens", "Brett: Sperrholzplatte mit grauer Teppichpolsterung und Querleisten"), material="carpet", geometry="horizontal-plate-up", L=0.40, C=0.025 * 680 * 1600 + 0.010 * 250 * 1300, layer=("carpet", T("Needle-felt carpet 10 mm", "Nadelfilz 10 mm"), "polymer", 0.17, 3250)),
    dict(id="straps", title=T("Two ratchet straps (polyester webbing)", "Zwei Ratschengurte (Polyestergewebe)"), material="webbing", geometry="vertical-plate", L=0.05, C=0.003 * 1380 * 1300, layer=("webbing", T("Woven polyester 3 mm", "Polyestergewebe 3 mm"), "polymer", 0.015, 5400)),
    dict(id="worn-finish", title=T("Worn-through lacquer patches and the dealer's sticker", "Durchgegriffene Lackstellen und der Händleraufkleber"), material="worn-finish", geometry="vertical-plate", L=0.06, C=0.002 * 700 * 1500, layer=("primer", T("Exposed grey primer and wood", "Freiliegender grauer Grund und Holz"), "timber", 0.004, 2100)),
]
AREA = {}


def regions():
    out = []
    for r in REGIONS:
        lid, ltitle, lkind, lres, lcap = r["layer"]
        reg = {"id": r["id"], "title": r["title"], "kind": "surface", "model": "passive", "material": r["material"], "areaM2": R6(AREA.get(r["id"], 1.0)), "characteristicLengthM": r["L"], "geometry": r["geometry"], "skyViewFactor": 0.5, "layers": [{"id": lid, "title": ltitle, "kind": lkind, "material": r["material"], "resistanceM2KW": lres, "heatCapacityJM2K": lcap, "source": "authored-estimate", "reference": "iso-10456-2007"}], "heatCapacityJM2K": R6(r["C"]), "source": "authored-estimate", "reference": "incropera-2018"}
        if r.get("power"):
            reg["ratedPowerW"] = 0
            reg["ratedPowerBasis"] = "passive"
        if r.get("hidden"):
            reg["hidden"] = r["hidden"]
        out.append(reg)
    return out


def binding(contact_skid=True):
    regs = {r["id"]: {"exposedFraction": 1, "sunlitFraction": 0} for r in REGIONS}
    if contact_skid:
        regs["skid"] = {"exposedFraction": 1, "sunlitFraction": 0, "contact": {"fraction": 0.02, "tempC": 15}}
    return {"regions": regs, "emitters": {}}


ENVIRONMENT = {"airTempC": 16, "meanRadiantTempC": 16, "skyTempC": 16, "groundTempC": 15, "relativeHumidity": 0.55, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 1, "pressurePa": 101325}


# ----------------------------------------------------------------------------------------------- poses and clips
def played_bones(lid_deg, prop_long, prop_short, flap=-180.0, desk=75.0, fall=-90.0):
    b = {"case-slide": {"dx": R6(PLAY_CASE_DX)}, "case-lift": {"dy": R6(PLAY_CASE_DY)}, "case": {"angleDeg": -90}, "lid": {"angleDeg": R6(lid_deg)}, "lid-flap": {"twistDeg": flap}, "prop-long": {"angleDeg": prop_long}, "prop-short": {"angleDeg": prop_short}, "fallboard": {"twistDeg": fall}, "desk": {"twistDeg": desk}}
    for k, ch in list(LEG_CHAIN.items()) + [("lyre", LYRE_CHAIN)]:
        b[f"{k}-slide"] = {"dx": R6(ch["du"])}
        b[f"{k}-lift"] = {"dy": R6(ch["dv"])}
        if abs(ch["dang"]) > 1e-9:
            b[k] = {"angleDeg": R6(ch["dang"])}
    for s in ["a", "b"]:
        b[f"strap-{s}-up"] = {"angleDeg": R6(180 - UP_DEG)}
        b[f"strap-{s}-top"] = {"angleDeg": R6(UP_DEG - 180)}
        b[f"strap-{s}-down"] = {"angleDeg": R6(-DOWN_DEG)}
    return b


def carried_bones(away):
    b = {"skid-lift": {"dy": CARRY_LIFT}}
    if away:
        b["skid-slide"] = {"dx": CARRY_AWAY}
    for k in list(LEGS) + ["lyre"]:
        b[f"{k}-slide"] = {"dx": -CARRY_LIFT}
        if away:
            b[f"{k}-lift"] = {"dy": CARRY_AWAY}
    return b


BATTENS = ["batten-key-lid", "batten-key-bottom", "batten-tail-lid", "batten-tail-bottom"]
GRIPS = ["grip-1", "grip-2", "grip-3", "grip-4"]
RESTING = ["lyre-standing", "leg-bass-resting", "leg-tail-resting"]
CASTORS = [f"{k}-castor" for k in LEGS]


def hand_loads():
    return [{"id": "left-hand", "kind": "live", "title": T("Left hand playing (forte chord)", "Linke Hand spielt (Forte-Akkord)"), "anchor": "hand-bass", "fxN": 0, "fyN": -15, "source": "authored-estimate", "reference": "askenfelt-1990"}, {"id": "right-hand", "kind": "live", "title": T("Right hand playing", "Rechte Hand spielt"), "anchor": "hand-treble", "fxN": 0, "fyN": -12, "source": "authored-estimate", "reference": "askenfelt-1990"}, {"id": "sheet-music", "kind": "imposed", "title": T("Sheet music on the desk (0.3 kg)", "Noten auf dem Pult (0,3 kg)"), "anchor": "music-on-desk", "fxN": 0, "fyN": -2.9, "source": "illustrative"}]


def poses():
    P = []
    P.append({"id": "stored", "title": T("Stored on the skid", "Auf dem Brett gelagert"), "description": T("At the dealer's depot: the baby grand lies on its straight spine on the padded skid, lid, flap, fallboard and desk closed and the props folded, held by two ratchet straps; the three legs lie beside the skid and the pedal lyre stands against the keybed.", "Im Lager des Händlers: der Stutzflügel liegt auf seiner geraden Rast auf dem gepolsterten Brett, Deckel, Vorderdeckel, Tastenklappe und Pult geschlossen und die Stäbe umgelegt, gehalten von zwei Ratschengurten; die drei Beine liegen neben dem Brett, die Pedallyra lehnt am Klaviaturboden."), "bones": {}, "ground": "auto", "contacts": BATTENS + RESTING, "thermal": binding(), "loads": []})
    P.append({"id": "carried", "title": T("Carried on the skid", "Auf dem Brett getragen"), "description": T("Four movers lift the strapped piano on its skid by the hand slots 80 mm off the floor; each hand carries about 0.7 kN; legs and lyre stay on the floor.", "Vier Möbelpacker heben den verzurrten Flügel auf seinem Brett an den Griffschlitzen 80 mm vom Boden; jede Hand trägt etwa 0,7 kN; Beine und Lyra bleiben am Boden."), "bones": carried_bones(False), "ground": "authored", "contacts": GRIPS + RESTING, "thermal": binding(False), "loads": []})
    P.append({"id": "carried-away", "title": T("Carried away", "Weggetragen"), "description": T("The movers have carried the piano 1.2 m on its skid.", "Die Möbelpacker haben den Flügel auf dem Brett 1,2 m weit getragen."), "key": True, "bones": carried_bones(True), "ground": "authored", "contacts": GRIPS + RESTING, "thermal": binding(False), "loads": []})
    P.append({"id": "played", "title": T("Played, lid on the long prop", "Gespielt, Deckel auf dem langen Stab"), "description": T("Set up on its three legs with the lyre fitted beside the empty skid, flap folded back, lid on the long prop at 32 degrees, fallboard open, music desk up; a pianist on a bench (not part of the asset) plays: both hands and the sheet music are loads.", "Auf seinen drei Beinen mit eingesetzter Lyra neben dem leeren Brett aufgestellt, Vorderdeckel zurückgeklappt, Deckel auf dem langen Stab bei 32 Grad, Tastenklappe offen, Notenpult aufgestellt; ein Pianist auf einer Bank (nicht Teil des Assets) spielt: beide Hände und die Noten sind Lasten."), "bones": played_bones(LID_LONG_DEG, -85, 0), "ground": "auto", "contacts": CASTORS + BATTENS, "thermal": binding(), "loads": hand_loads()})
    P.append({"id": "played-short-prop", "title": T("Played, lid on the short prop", "Gespielt, Deckel auf dem kurzen Stab"), "description": T("As played, the lid on the short prop at 17 degrees for chamber music in a small room.", "Wie gespielt, der Deckel auf dem kurzen Stab bei 17 Grad für Kammermusik im kleinen Raum."), "bones": played_bones(LID_SHORT_DEG, 0, -85), "ground": "auto", "contacts": CASTORS + BATTENS, "thermal": binding(), "loads": hand_loads()})
    P.append({"id": "played-lid-closed", "title": T("Played, lid closed", "Gespielt, Deckel geschlossen"), "description": T("Lid closed with the flap folded back over it, fallboard open and desk up: practising quietly.", "Deckel geschlossen, Vorderdeckel darauf zurückgeklappt, Tastenklappe offen und Pult aufgestellt: leises Üben."), "bones": played_bones(0, 0, 0), "ground": "auto", "contacts": CASTORS + BATTENS, "thermal": binding(), "loads": hand_loads()})
    P.append({"id": "lid-raised", "title": T("Lid held up", "Deckel gehalten"), "description": T("The lid is held 6 degrees above the long-prop position while the prop is still folded.", "Der Deckel wird 6 Grad über der Stellung des langen Stabs gehalten, der Stab liegt noch."), "key": True, "bones": played_bones(LID_LONG_DEG + 6, 0, 0), "ground": "auto", "contacts": CASTORS + BATTENS, "thermal": binding(), "loads": []})
    P.append({"id": "lid-raised-prop-up", "title": T("Prop raised under the held lid", "Stab unter dem gehaltenen Deckel aufgestellt"), "description": T("The long prop stands; the lid is still held above it.", "Der lange Stab steht; der Deckel wird noch darüber gehalten."), "key": True, "bones": played_bones(LID_LONG_DEG + 6, -85, 0), "ground": "auto", "contacts": CASTORS + BATTENS, "thermal": binding(), "loads": []})
    pd = played_bones(LID_LONG_DEG, -85, 0)
    pd["pedal-sustain"] = {"angleDeg": PEDAL_PRESS_DEG}
    P.append({"id": "sustain-down", "title": T("Sustain pedal down", "Fortepedal getreten"), "description": T("The right foot presses the sustain pedal 13 mm down at the toe with 45 N.", "Der rechte Fuß tritt das Fortepedal an der Spitze 13 mm mit 45 N."), "key": True, "bones": pd, "ground": "auto", "contacts": CASTORS + BATTENS, "thermal": binding(), "loads": hand_loads() + [{"id": "sustain-foot-force", "kind": "live", "title": T("Right foot on the sustain pedal", "Rechter Fuß auf dem Fortepedal"), "anchor": "sustain-foot", "fxN": 0, "fyN": -45, "source": "authored-estimate", "reference": "askenfelt-1990"}]})
    return P


def clips():
    allc = lambda ids, a, b: [{"anchor": i, "fromS": a, "toS": b} for i in ids]
    return [
        {"id": "carry", "title": T("Carry on the skid", "Auf dem Brett tragen"), "description": T("Four movers take the strapped piano by the hand slots of the skid, lift it 80 mm in 1.5 s and carry it 1.2 m; the legs and the lyre stay where they are.", "Vier Möbelpacker greifen den verzurrten Flügel an den Griffschlitzen des Bretts, heben ihn in 1,5 s 80 mm an und tragen ihn 1,2 m weit; Beine und Lyra bleiben liegen."), "durationS": 6, "fps": 10, "loop": False, "rootMotion": "none", "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 1.5, "pose": "carried", "ease": "smooth"}, {"t": 6, "pose": "carried-away", "ease": "smooth"}], "contacts": allc(BATTENS, 0, 0.6) + allc(GRIPS, 0.4, 6) + allc(RESTING, 0, 6), "phases": [{"id": "lift", "fromS": 0, "toS": 1.5, "title": T("Lifting", "Anheben")}, {"id": "carry", "fromS": 1.5, "toS": 6, "title": T("Carrying", "Tragen")}], "events": [{"t": 0.5, "kind": "contact-off", "title": T("Battens leave the floor", "Leisten verlassen den Boden"), "target": "skid"}, {"t": 1.5, "kind": "motion", "title": T("Movers walk off", "Möbelpacker gehen los"), "target": "skid"}], "tracks": []},
        {"id": "lid-opening", "title": T("Opening the lid onto the long prop", "Deckel auf den langen Stab öffnen"), "description": T("With the flap already folded back the lid is lifted 38 degrees in 2 s, the long prop is raised in 1 s and the lid is lowered onto it in 1.5 s.", "Bei schon zurückgeklapptem Vorderdeckel wird der Deckel in 2 s um 38 Grad gehoben, der lange Stab in 1 s aufgestellt und der Deckel in 1,5 s darauf abgesenkt."), "durationS": 4.5, "fps": 20, "loop": False, "rootMotion": "none", "keys": [{"t": 0, "pose": "played-lid-closed", "ease": "smooth"}, {"t": 2, "pose": "lid-raised", "ease": "smooth"}, {"t": 3, "pose": "lid-raised-prop-up", "ease": "smooth"}, {"t": 4.5, "pose": "played", "ease": "smooth"}], "contacts": allc(CASTORS + BATTENS, 0, 4.5), "phases": [{"id": "lift-lid", "fromS": 0, "toS": 2, "title": T("Lifting the lid", "Deckel heben")}, {"id": "raise-prop", "fromS": 2, "toS": 3, "title": T("Raising the prop", "Stab aufstellen")}, {"id": "lower-lid", "fromS": 3, "toS": 4.5, "title": T("Lowering onto the prop", "Auf den Stab absenken")}], "events": [{"t": 4.5, "kind": "contact-on", "title": T("Lid rests in the prop cup", "Deckel liegt im Stabnapf"), "target": "prop-long"}], "tracks": []},
        {"id": "sustain-pedal", "title": T("Sustain pedal", "Fortepedal"), "description": T("The right foot presses the sustain pedal for 1.2 s and releases it: a 2.4 s pedalling cycle with the 45 N foot force.", "Der rechte Fuß tritt das Fortepedal 1,2 s lang und lässt es los: ein Pedalzyklus von 2,4 s mit 45 N Fußkraft."), "durationS": 2.4, "fps": 25, "loop": True, "rootMotion": "none", "keys": [{"t": 0, "pose": "played", "ease": "smooth"}, {"t": 0.5, "pose": "sustain-down", "ease": "smooth"}, {"t": 1.7, "pose": "sustain-down", "ease": "smooth"}, {"t": 2.2, "pose": "played", "ease": "smooth"}], "contacts": allc(CASTORS + BATTENS, 0, 2.4), "phases": [{"id": "press", "fromS": 0, "toS": 0.5, "title": T("Pressing", "Treten")}, {"id": "hold", "fromS": 0.5, "toS": 1.7, "title": T("Dampers raised", "Dämpfer gehoben")}, {"id": "release", "fromS": 1.7, "toS": 2.4, "title": T("Releasing", "Loslassen")}], "events": [{"t": 0.25, "kind": "load-on", "title": T("Foot presses", "Fuß tritt"), "target": "sustain-foot-force"}, {"t": 1.95, "kind": "load-off", "title": T("Foot releases", "Fuß lässt los"), "target": "sustain-foot-force"}], "tracks": []},
    ]


# ----------------------------------------------------------------------------------------------- representations: decisions per scale
REP_DECISIONS = {
    50: dict(
        silhouette=T("Stored: the side elevation looks at the keyboard end of the piano standing on its spine on the skid (7 x 30 mm): key slip, 52 key fronts with their joints, black keys, cheek blocks, the closed fallboard, nameboard and desk rail, the lid edge and flap, with the lyre against the keybed, the legs lying beside the board and the straps running over the top; the front elevation shows the underside of the case standing up, 34 x 30 mm: rim, keybed, soundboard with seven ribs, three beams, the tail leg block, the lyre and the legs; the plan shows the treble side edge with the strap runs over it. Played, the same shapes give the keyboard view, the bentside view and the true plan of the grand with the lid on its prop.", "Gelagert: die Seitenansicht blickt auf das Klaviaturende des auf seiner Rast auf dem Brett stehenden Flügels (7 x 30 mm): Vorsatzleiste, 52 Tastenfronten mit ihren Fugen, Obertasten, Backen, die geschlossene Tastenklappe, Namensbrett und Pultleiste, Deckelkante und Vorderdeckel, dazu die Lyra am Klaviaturboden, die neben dem Brett liegenden Beine und die über die Oberkante laufenden Gurte; die Vorderansicht zeigt die aufgestellte Unterseite des Gehäuses, 34 x 30 mm: Rast, Klaviaturboden, Resonanzboden mit sieben Rippen, drei Balken, den Klotz des hinteren Beins, Lyra und Beine; die Draufsicht zeigt die Diskantkante mit den Gurtläufen darüber. Gespielt ergeben dieselben Formen die Klaviaturansicht, die Ansicht der Biegung und den wahren Grundriss des Flügels mit dem Deckel auf dem Stab."),
        retained=T("Every part of the 1:50 body at its true size and place, drawn as the solid of its own prism so that all poses project right: rim, cheeks, keys, action, plate with its openings, strings, soundboard, ribs, beams, lid, flap, hinges, props, fallboard, desk, legs with ferrules and castors, lyre with pedals and braces, skid with slots and battens, straps with ratchets, the worn patches, the chipped flap corner and the dealer's sticker.", "Jedes Teil des Körpers 1:50 in wahrer Größe und Lage, als Körper seines eigenen Prismas gezeichnet, damit jede Pose richtig projiziert: Rast, Backen, Tasten, Mechanik, Platte mit ihren Öffnungen, Saiten, Resonanzboden, Rippen, Balken, Deckel, Vorderdeckel, Scharniere, Stäbe, Tastenklappe, Pult, Beine mit Schuhen und Rollen, Lyra mit Pedalen und Streben, Brett mit Schlitzen und Leisten, Gurte mit Ratschen, die abgegriffenen Stellen, die abgeplatzte Vorderdeckelecke und der Händleraufkleber."),
        merged=T("The 230 strings are one string layer; tuning pins, dampers, hammers and the action parts are one action block; the long and bass bridges lie hidden under the plate and are not drawn.", "Die 230 Saiten sind eine Saitenlage; Stimmwirbel, Dämpfer, Hämmer und die Mechanikteile sind ein Mechanikblock; Steg und Basssteg liegen verborgen unter der Platte und sind nicht gezeichnet."),
        omitted=T("Hidden parts are covered by the nearer ones in each view; screws, the hitch pins, the felt strips and the lacquer cracks themselves are below the print profile.", "Verdeckte Teile werden in jeder Ansicht von den näheren verdeckt; Schrauben, Anhängestifte, Filzstreifen und die Lackrisse selbst liegen unter dem Druckprofil."),
        thinElements=T("Straps (3 mm) and the lyre braces are slender solids drawn at their true width; the key joints are 0.13 mm texture strokes in the elevation and a 0.47 mm hatch in the plan.", "Gurte (3 mm) und Lyrastreben sind schlanke Körper in wahrer Breite; die Tastenfugen sind 0,13-mm-Texturstriche in der Ansicht und eine 0,47-mm-Schraffur im Grundriss."),
        hatching=T("Stipple of crazed lacquer on rim and lid, lines along the strings in the plate openings, key joints on the keys, stipple of the carpet on the skid.", "Punktraster des craquelierten Lacks auf Rast und Deckel, Linien längs der Saiten in den Plattenöffnungen, Tastenfugen auf den Tasten, Punktraster des Teppichs auf dem Brett."),
        styles=T("Black lacquer in dark tones, yellowed keys light, cast iron and brass in middle greys, raw wood light grey, carpet and straps middle grey; worn patches lighter than the lacquer around them.", "Schwarzer Lack in dunklen Tönen, vergilbte Tasten hell, Guss und Messing in mittleren Grautönen, rohes Holz hellgrau, Teppich und Gurte mittelgrau; abgegriffene Stellen heller als der Lack um sie."),
        overlays=T("Thermal regions and structural members are painted on the same shapes; no annotations.", "Thermische Regionen und Tragglieder werden auf dieselben Formen gemalt; keine Beschriftungen.")),
    100: dict(
        silhouette=T("The same three views at 15 x 15 mm (stored end view) and 17 x 15 mm: the keyboard end with cheeks, key block and black-key row, the underside with rim, keybed, soundboard, beams, lyre and legs, the plan of the treble edge; played, the plan of the grand with the open lid.", "Dieselben drei Ansichten mit 15 x 15 mm (gelagerte Stirnansicht) und 17 x 15 mm: das Klaviaturende mit Backen, Tastenblock und Obertastenreihe, die Unterseite mit Rast, Klaviaturboden, Resonanzboden, Balken, Lyra und Beinen, der Grundriss der Diskantkante; gespielt der Grundriss des Flügels mit offenem Deckel."),
        retained=T("Rim, keybed, cheeks, end blocks, key slip, key block, nameboard, desk rail, action, belly rail, soundboard, three beams, plate with openings, strings, lid, flap, both props, fallboard, desk, legs, lyre with pedals, skid, straps with ratchets.", "Rast, Klaviaturboden, Backen, Endklötze, Vorsatzleiste, Tastenblock, Namensbrett, Pultleiste, Mechanik, Anhängestock, Resonanzboden, drei Balken, Platte mit Öffnungen, Saiten, Deckel, Vorderdeckel, beide Stäbe, Tastenklappe, Pult, Beine, Lyra mit Pedalen, Brett, Gurte mit Ratschen."),
        merged=T("White keys are one block with every second joint as hatch, black keys one row, board and carpet one padded board, the castor of each leg one wheel.", "Weiße Tasten sind ein Block mit jeder zweiten Fuge als Schraffur, Obertasten eine Reihe, Brett und Teppich ein Polsterbrett, die Rolle jedes Beins ein Rad."),
        omitted=T("Ribs, hinges, lock, key slip patch, sticker, scratches and the chip of the flap corner are below 15 mm.", "Rippen, Scharniere, Schloss, Fleck der Vorsatzleiste, Aufkleber, Kratzer und die Ecke des Vorderdeckels liegen unter 15 mm."),
        thinElements=T("Straps and braces stay slender at their true width; the 12 mm pedals are drawn 15 mm.", "Gurte und Streben bleiben schlank in wahrer Breite; die 12-mm-Pedale sind 15 mm gezeichnet."),
        hatching=T("Coarser crazing stipple on rim and lid, string lines in the plate openings, key joints and black-key row as hatches.", "Gröberes Craquelé-Punktraster auf Rast und Deckel, Saitenlinien in den Plattenöffnungen, Tastenfugen und Obertastenreihe als Schraffuren."),
        styles=T("Tones as at 1:50.", "Töne wie 1:50."),
        overlays=T("Thermal regions and members on the same shapes.", "Thermische Regionen und Tragglieder auf denselben Formen.")),
    200: dict(
        silhouette=T("At 1:200 the stored piano is 3.3 x 7.5 mm end-on and 8.5 x 7.5 mm from the underside: a keyboard end block, the rim, keybed and beams, the legs as tapered bars, the lyre frame and the skid; played, the grand outline with lid and props.", "Bei 1:200 ist der gelagerte Flügel stirnseitig 3,3 x 7,5 mm und von unten 8,5 x 7,5 mm: ein Klaviaturendblock, Rast, Klaviaturboden und Balken, die Beine als verjüngte Stäbe, der Lyrarahmen und das Brett; gespielt der Flügelumriss mit Deckel und Stäben."),
        retained=T("Rim with its true bentside, keyboard end, keybed, three beams, tail leg block, soundboard, plate, strings, lid, flap, both props, desk, legs with castor blocks, lyre frame, pedal block, braces, skid board and battens, straps.", "Rast mit wahrer Biegung, Klaviaturende, Klaviaturboden, drei Balken, Klotz des hinteren Beins, Resonanzboden, Platte, Saiten, Deckel, Vorderdeckel, beide Stäbe, Pult, Beine mit Rollenblöcken, Lyrarahmen, Pedalblock, Streben, Brett und Querleisten, Gurte."),
        merged=T("Cheeks, key slip and keybed front are one keyboard end block, keys one layer, action, nameboard and desk rail one front body with the fallboard, the three pedals one block.", "Backen, Vorsatzleiste und Klaviaturbodenfront sind ein Klaviaturendblock, die Tasten eine Lage, Mechanik, Namensbrett und Pultleiste mit der Tastenklappe ein Frontkörper, die drei Pedale ein Block."),
        omitted=T("Ribs, plate openings, hinges, ratchets, carpet and all wear marks except the worn cheek and lid edge are below 30 mm.", "Rippen, Plattenöffnungen, Scharniere, Ratschen, Teppich und alle Gebrauchsspuren außer der abgegriffenen Backe und Deckelkante liegen unter 30 mm."),
        thinElements=T("Lid, flap, plate, strings and soundboard are thin sheets at their true gauge; the long prop and the braces are slender.", "Deckel, Vorderdeckel, Platte, Saiten und Resonanzboden sind dünne Platten in wahrer Stärke; der lange Stab und die Streben sind schlank."),
        hatching=T("No hatching: the scale shows tone only.", "Keine Schraffur: der Maßstab zeigt nur Ton."),
        styles=T("Tones as at 1:50.", "Töne wie 1:50."),
        overlays=T("Thermal regions and members on the same shapes.", "Thermische Regionen und Tragglieder auf denselben Formen.")),
    500: dict(
        silhouette=T("At 1:500 the stored piano is 1.3 x 3.0 mm end-on and 3.4 x 3.0 mm from the underside: one rim, the keyboard end, the beams, three leg bars, the lyre and the board; played, the grand outline 2.9 x 3.0 mm with the raised lid.", "Bei 1:500 ist der gelagerte Flügel stirnseitig 1,3 x 3,0 mm und von unten 3,4 x 3,0 mm: eine Rast, das Klaviaturende, die Balken, drei Beinstäbe, die Lyra und das Brett; gespielt der Flügelumriss 2,9 x 3,0 mm mit gehobenem Deckel."),
        retained=T("Rim (75 mm), keyboard end, front body, keybed, three beams, tail leg block, soundboard, plate and strings as sheets, lid, flap, long prop, three legs, lyre frame with pedal block, skid, straps.", "Rast (75 mm), Klaviaturende, Frontkörper, Klaviaturboden, drei Balken, Klotz des hinteren Beins, Resonanzboden, Platte und Saiten als Platten, Deckel, Vorderdeckel, langer Stab, drei Beine, Lyrarahmen mit Pedalblock, Brett, Gurte."),
        merged=T("Board, carpet and battens are one board, each leg one tapered bar to the floor, the lyre one frame.", "Brett, Teppich und Leisten sind eine Platte, jedes Bein ein verjüngter Stab bis zum Boden, die Lyra ein Rahmen."),
        omitted=T("Short prop, fallboard, desk, castors, ratchets and the small wear marks are below 75 mm or inside the case.", "Kurzer Stab, Tastenklappe, Pult, Rollen, Ratschen und die kleinen Gebrauchsspuren liegen unter 75 mm oder im Gehäuse."),
        thinElements=T("The lid is a 24 mm sheet, the straps and the long prop are slender bars drawn at their true width with the 0.2 mm pen.", "Der Deckel ist eine 24-mm-Platte, Gurte und langer Stab sind schlanke Stäbe in wahrer Breite mit dem 0,2-mm-Stift."),
        hatching=T("No hatching.", "Keine Schraffur."),
        styles=T("Tones as at 1:50.", "Töne wie 1:50."),
        overlays=T("Thermal regions and members on the same shapes.", "Thermische Regionen und Tragglieder auf denselben Formen.")),
}
VALID = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}

REFERENCES = [
    {"id": "hornbostel-sachs-1914", "citation": "von Hornbostel, E. M.; Sachs, C. (1914). Systematik der Musikinstrumente. Zeitschrift für Ethnologie 46, 553-590: the piano is 314.122-4-8, a board zither with resonator box sounded by hammers with a keyboard."},
    {"id": "fletcher-rossing-1998", "citation": "Fletcher, N. H.; Rossing, T. D. (1998). The Physics of Musical Instruments, 2nd ed., Springer, ch. 12 The Piano (spruce soundboard 8-10 mm with ribs, cast-iron plate, string tension of about 20 kN per note group and 160-200 kN in all)."},
    {"id": "askenfelt-1990", "citation": "Askenfelt, A. (ed.) (1990). Five Lectures on the Acoustics of the Piano. Royal Swedish Academy of Music No. 64 (grand action, key dip about 10 mm, static touch weight about 0.5 N, playing forces of 10-50 N, damper pedal)."},
    {"id": "conklin-1996", "citation": "Conklin, H. A. (1996). Design and tone in the mechanoacoustic piano. Part I: Piano hammers and tonal effects; Part II: Piano structure. J. Acoust. Soc. Am. 99(6), 3286-3296 and 100(2), 695-708 (rim, beams, plate and total string load)."},
    {"id": "baby-grand-catalogues", "citation": "Maker catalogue data of the 150 cm baby grand class (Stutzflügel): Yamaha GB1K 151 x 146 x 99 cm, 263 kg; Yamaha GC1 161 x 149 x 101 cm, 290 kg; key top about 0.72 m, octave span 164-165 mm, 88 keys over 1.22 m; three legs and a three-pedal lyre (catalogue-typical values, no brand drawn)."},
    {"id": "en-12195-2", "citation": "EN 12195-2:2000 Load restraint assemblies on road vehicles - Safety - Part 2: Web lashing made from man-made fibres (polyester ratchet straps, 50 mm webbing)."},
    {"id": "piano-skid-practice", "citation": "Removal trade practice for grand pianos: the instrument is turned onto its straight (bass) side on a padded piano board (skid, Flügelbrett) of about 1.6-1.8 x 0.35-0.45 m with hand slots, legs and lyre removed, and strapped (authored-estimate from mover's equipment catalogues)."},
    {"id": "iso-10456-2007", "citation": "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (timber, plywood, metals, polymers)."},
    {"id": "incropera-2018", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (emissivity of lacquer, wood, cast iron, oxidised brass and steel)."},
]


# ----------------------------------------------------------------------------------------------- document
def build():
    bones = build_rig()
    parts50 = all_parts(50)
    masses = bone_masses(parts50)
    for b in bones:
        m = masses.get(b["id"])
        if m:
            b["mass"] = {"kg": R6(m["kg"]), "comX": R6(m["comX"]), "comY": R6(m["comY"]), "distribution": "point", "source": "authored-estimate", "reference": "baby-grand-catalogues" if b["id"] not in ("skid",) and not b["id"].startswith("strap") else "piano-skid-practice"}
        if b["id"] in SECTIONS:
            sec = SECTIONS[b["id"]]
            b["section"] = {k: (sig(v) if isinstance(v, float) else v) for k, v in sec.items()}
    world = solve(bones)
    reps, vols = {}, {}
    for scale in SCALES:
        parts = all_parts(scale)
        geometry, hatches = build_shapes(scale, parts, world)
        lo, hi = VALID[scale]
        reps[str(scale)] = {"status": "authored", "denominator": scale, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[scale], "hatches": hatches, "geometry": geometry, "decisions": REP_DECISIONS[scale]}
        body, thermal, structure = build_volume(scale, parts, masses)
        vols[str(scale)] = {"status": "authored", "denominator": scale, "body": body, "thermal": thermal, "structure": structure, "decisions": VOLUME_DECISIONS[scale]}
    pts = []
    for p in parts50:
        pts += [to_world(world, p.bone, q) for q in p.local_points()]
    xs, ys, zs = [p[0] for p in pts], [p[1] for p in pts], [p[2] for p in pts]
    total = sum(m["kg"] for m in masses.values())
    dims = {"heightM": R6(max(ys) - min(0, min(ys))), "widthM": R6(max(xs) - min(xs)), "depthM": R6(max(zs) - min(zs)), "massKg": R6(total)}
    piano = sum(m["kg"] for k, m in masses.items() if not (k.startswith("skid") or k.startswith("strap")))
    asset = {
        "id": "used-baby-grand-piano",
        "category": "objects",
        "title": T("Used Baby Grand Piano", "Gebrauchter Stutzflügel"),
        "summary": T(f"A second-hand black-lacquer baby grand, 1.50 m long, 1.45 m wide and 0.99 m high with the lid closed, {piano:.0f} kg, with crazed lacquer, yellowed keys, a chipped flap corner and lacquer worn through where the hands rest. At the dealer it lies on its spine on a padded skid, strapped, with the legs and the lyre beside it; it is carried by four movers, and set up it is played with the lid on the long or the short prop, the sustain pedal working.", f"Ein gebrauchter schwarz lackierter Stutzflügel, 1,50 m lang, 1,45 m breit und bei geschlossenem Deckel 0,99 m hoch, {piano:.0f} kg, mit craqueliertem Lack, vergilbten Tasten, abgeplatzter Vorderdeckelecke und durchgegriffenem Lack, wo die Hände aufliegen. Beim Händler liegt er auf seiner Rast verzurrt auf einem gepolsterten Brett, Beine und Lyra daneben; vier Möbelpacker tragen ihn, und aufgestellt wird er mit dem Deckel auf dem langen oder kurzen Stab gespielt, das Fortepedal arbeitet."),
        "tags": ["used", "instrument", "music", "second-hand", "keyboard", "piano", "grand-piano", "baby-grand", "chordophone", "lacquer", "cast-iron", "spruce", "brass", "skid", "strapped"],
        "provenance": {"author": "Semio staffage content (used instruments wave)", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES, "notes": T("Type: the 150 cm baby grand (Stutzflügel) of the post-war DDR/BRD workshop and Japanese series classes, drawn as a type without brand or logo. Played frame: X across the keyboard (bass to treble), Y up, Z from the key slip to the tail; asset axes: x ahead = across the keyboard, z left = along the piano, so the side elevation is the pianist's view of the keyboard, the front elevation the view of the bentside and the plan the grand outline. Rest and default pose are the stored state on the skid; the case is tipped about its spine edge by -90 degrees and lifted by two prismatic carriers into the played state, the legs and the lyre are carried from their stored places to their mounts by slide, lift and revolute chains, and the strap runs fold flat onto the floor. Masses are the 1:50 prism volumes times the densities of the materials; strings, action and key frame carry effective densities for their wire, lead weights and flanges.", "Typ: der 150-cm-Stutzflügel der DDR/BRD-Werkstattklassen der Nachkriegszeit und der japanischen Serienklassen, als Typ ohne Marke oder Logo gezeichnet. Spielrahmen: X quer zur Tastatur (Bass bis Diskant), Y nach oben, Z von der Vorsatzleiste zum Schwanz; Asset-Achsen: x vorn = quer zur Tastatur, z links = längs des Flügels, so dass die Seitenansicht der Blick des Pianisten auf die Tastatur ist, die Vorderansicht der Blick auf die Biegung und der Grundriss der Flügelumriss. Ruhe- und Standardpose ist der Lagerzustand auf dem Brett; das Gehäuse wird um seine Rastkante um -90 Grad gekippt und von zwei Schubgelenken in den Spielzustand gehoben, Beine und Lyra werden von Schub-, Hub- und Drehketten von ihren Lagerplätzen an ihre Befestigungen gebracht, und die Gurtläufe legen sich flach auf den Boden. Massen sind die Prismenvolumen 1:50 mal die Dichten der Werkstoffe; Saiten, Mechanik und Tastenrahmen tragen wirksame Dichten für Draht, Bleigewichte und Flansche.")},
        "dimensions": dims,
        "materials": MATERIALS,
        "rig": {"kind": "mechanical", "root": "floor", "bones": bones},
        "anchors": anchors(),
        "representations": reps,
        "symbols": [],
        "poses": poses(),
        "clips": clips(),
        "thermal": {"environment": ENVIRONMENT, "operating": [], "passengers": [], "regions": regions(), "emitters": [], "assumptions": [T("Acoustic instrument without power: every surface is passive lacquer, wood, cast iron, brass, carpet or webbing in a depot at 16 degC without sun; the case region states its 0 W explicitly.", "Akustisches Instrument ohne Leistung: jede Fläche ist passiver Lack, Holz, Guss, Messing, Teppich oder Gewebe in einem Lager bei 16 °C ohne Sonne; die Gehäuseregion nennt ihre 0 W ausdrücklich."), T("Aged lacquer: crazed polyester lacquer keeps the emissivity of new lacquer (0.91) but its film is cracked; where it is worn through the grey primer and wood are their own region.", "Gealterter Lack: craquelierter Polyesterlack behält den Emissionsgrad neuen Lacks (0,91), sein Film ist aber gerissen; wo er durchgegriffen ist, sind grauer Grund und Holz eine eigene Region."), T("Plate and strings lie inside the closed case in the stored pose and count their whole surface (hidden regions).", "Platte und Saiten liegen in der Lagerpose im geschlossenen Gehäuse und zählen ihre ganze Fläche (verborgene Regionen).")]},
        "structure": {"gravityMS2": 9.81, "dynamics": "quasi-static", "supports": [], "assumptions": [T("No support is permanent: each pose names its contacts. Stored, the piano rests with its spine on the carpet of the skid, which stands on the four ends of its two battens; the lying legs and the standing lyre rest on the floor. Carried, four hands under the slot ends of the skid carry the piano and skid (about 0.7 kN each). Played, the three castors carry the piano and the empty skid lies on its battens.", "Kein Auflager ist dauerhaft: jede Pose nennt ihre Kontakte. Gelagert ruht der Flügel mit seiner Rast auf dem Teppich des Bretts, das auf den vier Enden seiner zwei Leisten steht; die liegenden Beine und die stehende Lyra ruhen am Boden. Getragen tragen vier Hände unter den Schlitzenden Flügel und Brett (je etwa 0,7 kN). Gespielt tragen die drei Rollen den Flügel, und das leere Brett liegt auf seinen Leisten."), T("The string tension of about 180 kN is an internal, self-equilibrated load between the cast-iron plate and the rim; it causes no support reaction and is not applied as a load.", "Der Saitenzug von etwa 180 kN ist eine innere, im Gleichgewicht stehende Last zwischen Gussplatte und Rast; er erzeugt keine Auflagerkraft und wird nicht als Last aufgebracht."), T("The solver treats the whole asset as one rigid body on its contacts; the straps are pretensioned restraints, not members.", "Der Löser behandelt das ganze Asset als einen starren Körper auf seinen Kontakten; die Gurte sind vorgespannte Sicherungen, keine Tragglieder.")], "clinical": False},
        "volumes": vols,
        "presentation": {"defaultPose": "stored", "defaultClip": "lid-opening", "defaultScale": 100},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}


def load_areas():
    f = os.path.join(HERE, "areas.json")
    if os.path.exists(f):
        AREA.update(json.load(open(f)))


if __name__ == "__main__":
    load_areas()
    doc = build()
    with open(OUT, "w", encoding="utf-8") as fh:
        json.dump(doc, fh, ensure_ascii=False, indent=1)
    a = doc["asset"]
    print("dimensions", a["dimensions"], "bones", len(a["rig"]["bones"]), {k: len(v["body"]) for k, v in a["volumes"].items()}, {k: sum(len(g) for g in v["geometry"].values()) for k, v in a["representations"].items()})
