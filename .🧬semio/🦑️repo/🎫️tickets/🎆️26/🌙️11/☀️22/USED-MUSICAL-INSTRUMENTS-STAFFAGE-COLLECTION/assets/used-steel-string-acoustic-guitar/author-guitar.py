"""🎸 Authoritative author script of `used-steel-string-acoustic-guitar`: states every hand-chosen number of the used dreadnought, its rectangular hardshell case and strap, and writes the asset document `🔣️.json` next to it.

Run: `python3 author-guitar.py` (writes `🔣️.json`; `--report` prints the reference measurements used in the report).
@see ../../brief.md
@see ../../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-tools.md
"""
import json
import math
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from author_lib import (R6, add, ccw, catmull, circle, cross, decompose, dot, earclip, frame_cols, inset, loft_mesh, mm, mesh_area, mv, norm, rect, rot, rounded_rect, scl, signed_volume, solve_rig, sub, tr, world)

HERE = os.path.dirname(os.path.abspath(__file__))
T = lambda en, de: {"en": en, "de": de}

HX, HZ, RC = 0.2225, 0.5525, 0.030
SH, LIN = 0.004, 0.006
Y_FEET, Y_SPLIT, Y_TOP = 0.008, 0.120, 0.183
LID_H = Y_TOP - Y_SPLIT
KX = -0.2265
Y_FLOOR = Y_FEET + SH + LIN
G_Y, G_Z = 0.1455, -0.523
BODY_L, SCALE_L = 0.508, 0.645
NUT_S = SCALE_L * (1 - 2 ** (-14 / 12))
HEAD_DEG, HEAD_L = -14.0, 0.180
D_TAIL, D_HEEL = 0.124, 0.098
SADDLE_U, SADDLE_TOP, NUT_TOP = 0.2175, 0.0125, 0.0085
SH_U, SH_R = 0.370, 0.0508
TENSION_N = 723.0
LATCH_Z = [-0.42, -0.18, 0.18, 0.42]
HINGE_Z = [-0.30, 0.30]
FEET = [(0.165, 0.45), (0.165, -0.45), (-0.165, 0.45), (-0.165, -0.45)]
LID_OPEN, LATCH_OPEN = 95.0, -100.0
PLAYED = {"x": 1.05, "y": 0.95, "tilt": 20.0}
GRIP_Y = 0.76
CAPO_AT = (0.125, 0.080, 0.12)


def depth(u):
    """📏 Body depth of the dreadnought from the tail (124 mm) to the heel (98 mm)."""
    return D_TAIL - (D_TAIL - D_HEEL) * max(0.0, min(BODY_L, u)) / BODY_L


HALF = [(0.003, 0.050), (0.012, 0.095), (0.028, 0.135), (0.050, 0.165), (0.080, 0.186), (0.115, 0.1965), (0.150, 0.1985), (0.185, 0.1955), (0.215, 0.1860), (0.245, 0.1690), (0.270, 0.1505), (0.292, 0.1395), (0.310, 0.1370), (0.330, 0.1395), (0.360, 0.1445), (0.395, 0.1460), (0.425, 0.1430), (0.452, 0.1350), (0.475, 0.1210), (0.492, 0.1020), (0.502, 0.0800), (0.507, 0.0550), (0.508, 0.0300)]


def outline(per=4):
    """🎸 The dreadnought body outline (u from the tail, w lateral, bass side +w), counter-clockwise."""
    ctrl = [(0.0, 0.0)] + HALF + [(0.5082, 0.0)] + [(u, -w) for u, w in reversed(HALF)]
    pts = catmull(ctrl, True, per)
    pts = [(R6(min(BODY_L, max(0.0, u))), R6(w)) for u, w in pts]
    out = []
    for p in pts:
        if not out or math.hypot(p[0] - out[-1][0], p[1] - out[-1][1]) > 0.0015:
            out.append(p)
    if math.hypot(out[0][0] - out[-1][0], out[0][1] - out[-1][1]) < 0.0015:
        out.pop()
    return ccw(out)


def half_width(u):
    """↔️ Half width of the body outline at u."""
    pts = [(0.0, 0.0)] + HALF + [(BODY_L, 0.0)]
    for (a, wa), (b, wb) in zip(pts, pts[1:]):
        if a <= u <= b:
            return wa + (wb - wa) * (u - a) / (b - a) if b > a else max(wa, wb)
    return 0.0


def fret_s(n):
    """🎼 Neck-local position of fret n (from the body joint towards the nut)."""
    return NUT_S - SCALE_L * (1 - 2 ** (-n / 12))


def fb_half(s):
    """🪵 Fretboard half width at neck position s: 43 mm at the nut, 56.5 mm at fret 20."""
    return 0.0215 + (NUT_S - s) * 0.00675 / (NUT_S - fret_s(20))


def neck_depth(s):
    """📐 Neck depth below the top plane: 19 mm at the body, 15.5 mm at the nut."""
    return 0.0155 + 0.0035 * (NUT_S - s) / NUT_S


def head_to_guitar(h, v):
    """🔩 A head-local (h, v) point in guitar-local (u, v)."""
    c, s = math.cos(math.radians(HEAD_DEG)), math.sin(math.radians(HEAD_DEG))
    return (BODY_L + NUT_S + h * c - v * s, h * s + v * c)


# ---------------------------------------------------------------- rig

def strap_points():
    """🎗️ The strap fold over the strings in the case (guitar-local u, v of the strap axis, its inner face)."""
    a = (0.0, -0.062)
    b = (0.0, 0.003)
    c = (0.344, 0.0205)
    d = (0.826, 0.0098)
    return a, b, c, d


def base_bones():
    """🦴 The rig with placeholder rest angles for the strap (solved by `finish_rig`)."""
    return [
        {"id": "case", "parent": None, "title": T("Hardshell case: bottom shell with lining, compartment and neck rest", "Hartschalenkoffer: Unterschale mit Futter, Fach und Halsauflage"), "x": 0.0, "y": 0.0, "angleDeg": 0, "lengthM": 0.445, "joint": {"type": "fixed"}},
        {"id": "lid", "parent": "case", "title": T("Case lid on its two hinges", "Kofferdeckel an seinen zwei Scharnieren"), "x": KX, "y": Y_SPLIT, "angleDeg": 0, "lengthM": 0.449, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 100}},
        {"id": "handle", "parent": "case", "title": T("Carrying handle on its two pivots", "Tragegriff an seinen zwei Drehpunkten"), "x": 0.2305, "y": 0.100, "z": 0.0, "angleDeg": -90, "lengthM": 0.044, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 95}},
        *[{"id": f"latch-{i + 1}", "parent": "case", "title": T(f"Draw latch {i + 1} (lever)", f"Spannverschluss {i + 1} (Hebel)"), "x": 0.2315, "y": 0.092, "z": z, "angleDeg": 90, "lengthM": 0.040, "joint": {"type": "revolute", "minDeg": -110, "maxDeg": 0}} for i, z in enumerate(LATCH_Z)],
        {"id": "capo", "parent": "case", "title": T("Spring capo lying on the compartment lid", "Federkapodaster auf dem Fachdeckel"), "x": CAPO_AT[0] - 0.032, "y": CAPO_AT[1], "z": CAPO_AT[2], "angleDeg": 0, "lengthM": 0.064, "joint": {"type": "fixed"}},
        {"id": "lift", "parent": "case", "title": T("Carrier of the guitar: the straight way from the case to the player (kinematic slide, no part)", "Träger der Gitarre: der gerade Weg vom Koffer zum Spieler (kinematische Führung, kein Bauteil)"), "x": 0.0, "y": G_Y, "z": G_Z, "angleDeg": 0, "lengthM": 0.005, "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 1.4}},
        {"id": "guitar", "parent": "lift", "title": T("Guitar body: top, back, sides and bridge, from the end pin to the neck joint", "Gitarrenkorpus: Decke, Boden, Zargen und Steg, vom Gurtpin bis zum Halsansatz"), "x": 0.0, "y": 0.0, "angleDeg": 0, "abductionDeg": 90, "lengthM": BODY_L, "joint": {"type": "revolute", "minDeg": -10, "maxDeg": 30, "twistMinDeg": -10, "twistMaxDeg": 100}},
        {"id": "neck", "parent": "guitar", "title": T("Neck with heel and fretboard, 14 frets clear of the body", "Hals mit Halsfuß und Griffbrett, 14 Bünde frei"), "x": BODY_L, "y": 0.0, "angleDeg": 0, "lengthM": round(NUT_S, 6), "joint": {"type": "fixed"}},
        {"id": "head", "parent": "neck", "title": T("Headstock with six tuners, pitched back 14 degrees", "Kopfplatte mit sechs Mechaniken, 14 Grad zurückgeneigt"), "x": round(NUT_S, 6), "y": 0.0, "angleDeg": HEAD_DEG, "lengthM": HEAD_L, "joint": {"type": "fixed"}},
        {"id": "strap-tab", "parent": "guitar", "title": T("Strap end tab on the end pin", "Gurtende am Gurtpin"), "x": 0.0, "y": -0.062, "angleDeg": 90, "lengthM": 0.065, "joint": {"type": "revolute", "minDeg": -180, "maxDeg": 180}},
        {"id": "strap-rear", "parent": "strap-tab", "title": T("Strap, rear run", "Gurt, hinterer Lauf"), "x": 0.065, "y": 0.0, "angleDeg": 0, "lengthM": 0.344, "joint": {"type": "revolute", "minDeg": -180, "maxDeg": 180}},
        {"id": "strap-cross", "parent": "strap-rear", "title": T("Strap, run across the back to the shoulder", "Gurt, Lauf über den Rücken zur Schulter"), "x": 0.344, "y": 0.0, "angleDeg": 0, "lengthM": 0.482, "joint": {"type": "revolute", "minDeg": -180, "maxDeg": 180}},
        {"id": "strap-front", "parent": "strap-cross", "title": T("Strap, front run with the lace to the headstock", "Gurt, vorderer Lauf mit der Schnur zur Kopfplatte"), "x": 0.482, "y": 0.0024, "angleDeg": 0, "lengthM": 0.482, "joint": {"type": "revolute", "minDeg": -180, "maxDeg": 180}},
    ]


def gframe_world(frames, p):
    """🌍 World position of a guitar-local point."""
    return world(frames, "guitar", p)


def finish_rig():
    """🧮 Solves the strap rest placements so the strap lies folded over the strings in the case."""
    bones = base_bones()
    by = {b["id"]: b for b in bones}
    a, b, c, d = strap_points()
    by["strap-rear"]["lengthM"] = R6(math.hypot(c[0] - b[0], c[1] - b[1]))
    by["strap-cross"]["x"] = by["strap-rear"]["lengthM"]
    by["strap-cross"]["lengthM"] = R6(math.hypot(d[0] - c[0], d[1] - c[1]))
    by["strap-front"]["x"] = by["strap-cross"]["lengthM"]
    by["strap-front"]["lengthM"] = by["strap-cross"]["lengthM"]
    by["strap-tab"]["lengthM"] = R6(b[1] - a[1])
    by["strap-rear"]["x"] = by["strap-tab"]["lengthM"]
    rest = solve_rig(bones)
    gr, go = rest["guitar"]
    targets = {}
    gu, gv, gw = [gr[i][0] for i in range(3)], [gr[i][1] for i in range(3)], [gr[i][2] for i in range(3)]

    def gdir(du, dv):
        return norm(add(scl(gu, du), scl(gv, dv)))

    u1 = gdir(c[0] - b[0], c[1] - b[1])
    v1 = norm(cross(gw, u1))
    targets["strap-rear"] = frame_cols(u1, v1, cross(u1, v1))
    u2 = gdir(d[0] - c[0], d[1] - c[1])
    v2 = norm(cross(gw, u2))
    targets["strap-cross"] = frame_cols(u2, v2, cross(u2, v2))
    u3 = scl(u2, -1)
    targets["strap-front"] = frame_cols(u3, v2, cross(u3, v2))
    for bid in ["strap-rear", "strap-cross", "strap-front"]:
        frames = solve_rig(bones)
        pr, _ = frames[by[bid]["parent"]]
        m = mm(tr(pr), targets[bid])
        ang, abd, tw = decompose(m)
        by[bid]["angleDeg"], by[bid]["abductionDeg"], by[bid]["twistDeg"] = R6(ang), R6(abd), R6(tw)
    return bones


BONES = finish_rig()
BY = {b["id"]: b for b in BONES}
REST = solve_rig(BONES)


def lift_offsets():
    """🚚 Prismatic travel of the lift bone into the played place."""
    r, o = REST["guitar"]
    dx, dy = PLAYED["x"] - o[0], PLAYED["y"] - o[1]
    return dx, dy


LDX, LDY = lift_offsets()
BY["lift"]["joint"]["axisDeg"] = R6(math.degrees(math.atan2(LDY, LDX)))
BY["lift"]["joint"]["maxM"] = R6(math.hypot(LDX, LDY) + 0.01)
GUITAR_PLAYED = {"angleDeg": PLAYED["tilt"], "twistDeg": 90}


def strap_played(offsets):
    """🧷 Strap offsets of the played pose: the strap runs from the end pin round the right flank, across the back to the left shoulder and down to the headstock lace."""
    frames = solve_rig(BONES, offsets)
    e = world(frames, "strap-tab", [0, 0, 0])
    tie = world(frames, "head", [0.025, -0.007, 0])
    centre = [0.81, 0, -0.20]
    l01 = BY["strap-tab"]["lengthM"] + BY["strap-rear"]["lengthM"]
    l2, l3 = BY["strap-cross"]["lengthM"], BY["strap-front"]["lengthM"]
    p1 = add(e, scl(norm(sub([0.66, 1.12, -0.36], e)), l01))
    want = [0.78, 1.46, -0.04]
    best, bd = None, 9
    for i in range(-60, 61):
        for j in range(-60, 61):
            for k in range(-60, 61, 6):
                p = add(want, [i * 0.004, j * 0.004, k * 0.004])
                err = abs(math.dist(p, p1) - l2) + abs(math.dist(p, tie) - l3)
                if err < bd:
                    bd, best = err, p
    p2 = best
    out = dict(offsets)

    def frame_for(start, end, hint):
        u = norm(sub(end, start))
        v = norm(sub(hint, scl(u, dot(hint, u))))
        return frame_cols(u, v, cross(u, v))

    def away(pt):
        h = [pt[0] - centre[0], 0, pt[2] - centre[2]]
        return norm(h)

    seq = [("strap-tab", e, p1, away(add(scl(e, 0.5), scl(p1, 0.5)))), ("strap-rear", e, p1, away(add(scl(e, 0.5), scl(p1, 0.5)))), ("strap-cross", p1, p2, [-1, 0, 0]), ("strap-front", None, tie, None)]
    for bid, s, t, h in seq:
        frames = solve_rig(BONES, out)
        if bid == "strap-front":
            s = world(frames, "strap-front", [0, 0, 0])
            h = norm(add(away(add(scl(s, 0.5), scl(t, 0.5))), [0, 0.6, 0]))
        target = frame_for(s, t, h)
        pr, _ = frames[BY[bid]["parent"]]
        rest = rot(BY[bid]["angleDeg"], BY[bid].get("abductionDeg", 0), BY[bid].get("twistDeg", 0))
        m = mm(tr(rest), mm(tr(pr), target))
        a, b, tw = decompose(m)
        out[bid] = {"angleDeg": R6(a), "abductionDeg": R6(b), "twistDeg": R6(tw)}
    frames = solve_rig(BONES, out)
    tip = world(frames, "strap-front", [BY["strap-front"]["lengthM"], 0, 0])
    return out, {"p1": p1, "p2": p2, "tie": tie, "tipError": math.dist(tip, tie), "e": e}


PLAY_OFF = {"lift": {"dx": R6(LDX), "dy": R6(LDY)}, "guitar": dict(GUITAR_PLAYED), "lid": {"angleDeg": LID_OPEN}, **{f"latch-{i + 1}": {"angleDeg": LATCH_OPEN} for i in range(4)}}
PLAY_OFF, STRAP_INFO = strap_played(PLAY_OFF)


# ---------------------------------------------------------------- masses (section x density, see report)

MASS = {
    "case": (2.73, 0.0, 0.058), "lid": (1.52, 0.2265, 0.040), "handle": (0.07, 0.030, 0.012),
    **{f"latch-{i + 1}": (0.012, 0.020, -0.004) for i in range(4)}, "capo": (0.06, 0.032, 0.008),
    "guitar": (1.25, 0.245, -0.062), "neck": (0.43, 0.150, -0.006), "head": (0.38, 0.090, -0.012),
    "strap-tab": (0.008, 0.032, 0.001), "strap-rear": (0.042, 0.172, 0.001), "strap-cross": (0.058, 0.241, 0.001), "strap-front": (0.060, 0.241, 0.001),
}
for bid, (kg, cx, cy) in MASS.items():
    BY[bid]["mass"] = {"kg": kg, "comX": cx, "comY": cy, "distribution": "point", "source": "authored-estimate", "reference": "fpl-wood-handbook" if bid in ("guitar", "neck", "head") else "case-catalogue"}
TOTAL_KG = round(sum(v[0] for v in MASS.values()), 3)


def com(frames):
    """⚖️ Centre of mass of the asset in world for a frame set."""
    s, m = [0, 0, 0], 0
    for bid, (kg, cx, cy) in MASS.items():
        p = world(frames, bid, [cx, cy, 0])
        s, m = add(s, scl(p, kg)), m + kg
    return scl(s, 1 / m)


def set_handle_z():
    """🎯 Puts the handle over the centre of mass along the case so the carried case hangs level."""
    for _ in range(3):
        c = com(solve_rig(BONES))
        BY["handle"]["z"] = R6(c[2])


set_handle_z()
REST = solve_rig(BONES)
Z_H = BY["handle"]["z"]

SECTIONS = {
    "case": (0.00192, 2.9e-06, 0.056, "tolex-ply", T("Bottom shell as an open channel 445 x 112 mm, walls 4 mm", "Unterschale als offener Kasten 445 x 112 mm, Wände 4 mm")),
    "lid": (0.00146, 7.4e-07, 0.031, "tolex-ply", T("Lid as an open channel 445 x 63 mm, walls 4 mm", "Deckel als offener Kasten 445 x 63 mm, Wände 4 mm")),
    "guitar": (0.00256, 4.6e-06, 0.062, "mahogany", T("Soundbox as a thin-walled box: top 3 mm, back 2.7 mm, sides 2.3 mm", "Klangkörper als dünnwandiger Kasten: Decke 3 mm, Boden 2,7 mm, Zargen 2,3 mm")),
    "neck": (0.00079, 3.4e-08, 0.012, "mahogany", T("Neck with fretboard and truss rod, about 49 x 22 mm", "Hals mit Griffbrett und Halsstab, etwa 49 x 22 mm")),
    "head": (0.00116, 2.0e-08, 0.0073, "mahogany", T("Headstock plate 80 x 14.5 mm", "Kopfplatte 80 x 14,5 mm")),
    "handle": (0.00048, 2.0e-08, 0.012, "handle-vinyl", T("Handle grip 22 x 24 mm with steel core", "Griff 22 x 24 mm mit Stahlkern")),
    **{s: (0.00012, 5.8e-11, 0.0012, "leather", T("Leather strap 50 x 2.4 mm", "Ledergurt 50 x 2,4 mm")) for s in ["strap-tab", "strap-rear", "strap-cross", "strap-front"]},
}
for bid, (a, i, e, mat, title) in SECTIONS.items():
    BY[bid]["section"] = {"areaM2": a, "secondMomentM4": i, "extremeFibreM": e, "material": mat, "title": title}


# ---------------------------------------------------------------- parts

class Part:
    """🧩 One part: its 2D solid shape and its 3D solid from the same numbers."""

    def __init__(self, pid, bone, role, mat, region, tone, sweep=None, prof=None, span=None, tip=None, holes=None, d3="auto", flags=None, d2=True, filled=True, regions=None, solid2d=None, layer=None):
        self.id, self.bone, self.role, self.mat, self.region, self.tone = pid, bone, role, mat, region, tone
        self.sweep, self.prof, self.span, self.tip, self.holes = sweep, prof, span, tip, holes
        self.d3, self.flags, self.d2, self.filled, self.regions, self.solid2d, self.layer = d3, flags or {}, d2, filled, regions, solid2d, layer

    def solid3(self):
        if self.d3 is None:
            return None
        base = {"id": self.id, "bone": self.bone, "material": self.mat, "role": self.role, "tone": self.tone, **self.flags}
        if self.d3 != "auto":
            return {**base, **self.d3}
        if self.sweep == "w":
            return {**base, "kind": "extrude", "plane": "xy", "profile": [list(p) for p in self.prof], **({"holes": [[list(p) for p in h] for h in self.holes]} if self.holes else {}), "from": R6(self.span[0]), "to": R6(self.span[1])}
        if self.sweep == "v":
            return {**base, "kind": "extrude", "plane": "zx", "profile": [[p[1], p[0]] for p in self.prof], **({"holes": [[[p[1], p[0]] for p in h] for h in self.holes]} if self.holes else {}), "from": R6(self.span[0]), "to": R6(self.span[1])}
        return {**base, "kind": "extrude", "plane": "yz", "profile": [list(p) for p in self.prof], "from": R6(self.span[0]), "to": R6(self.span[1])}

    def shape(self, z):
        if not self.d2:
            return None
        sh = {"id": self.id, "bone": self.bone, "role": self.role, "z": round(z, 3), "filled": self.filled and self.role != "texture", "tone": self.tone}
        if self.regions:
            sh["regions"] = self.regions
        else:
            sh["region"] = self.region
        sh["material"] = self.mat
        sh["kind"] = "poly"
        sh["points"] = [[R6(a), R6(b)] for a, b in self.prof]
        sh["closed"] = True
        if self.solid2d:
            sh["solid"] = self.solid2d
        else:
            oz = REST[self.bone][1][2] if self.sweep == "w" else 0.0
            sol = {"kind": "extruded", "sweep": self.sweep, "z0": R6(self.span[0] + oz), "z1": R6(self.span[1] + oz)}
            if self.tip:
                sol["zTip"] = [R6(self.tip[0] + oz), R6(self.tip[1] + oz)]
            sh["solid"] = sol
        return sh


def lid_u(x):
    return x - KX


def box3(size, at, cr=None, rotation=None, scale=None):
    """📦 A box solid body."""
    d = {"kind": "box", "size": [R6(s) for s in size], "transform": {"translation": [R6(a) for a in at]}}
    if cr:
        d["cornerRadius"] = cr
    if rotation:
        d["transform"]["rotation"] = rotation
    if scale:
        d["transform"]["scale"] = scale
    return d


def body_mesh(scale):
    """🎸 Soundbox polyhedron: outline walls with a chamfered back edge, depth tapering from 124 to 98 mm."""
    ol = outline(4 if scale == 50 else 3)
    ch = 0.0025
    inn = inset(ol, ch)
    a = [[u, -0.0035, w] for u, w in ol]
    b = [[u, -depth(u) + ch, w] for u, w in ol]
    c = [[u, -depth(u), w] for u, w in inn]
    verts, tris = loft_mesh([a, b, c], [ol, inn])
    return {"kind": "polyhedron", "vertices": verts, "triangles": [list(t) for t in tris]}, (verts, tris)


def neck_section(s, ring=14):
    """🦒 Neck cross-section at s: flat top under the fretboard, a C below, the heel deepening and narrowing towards the body."""
    wn = fb_half(min(s, NUT_S)) - 0.0003
    dn = neck_depth(min(s, NUT_S))
    hd = 0.092 * max(0.0, 1 - s / 0.08) ** 1.4 if s < 0.08 else 0.0
    dd = max(dn, hd)
    k = 0.45 * min(1.0, max(0.0, (hd - dn) / 0.06))
    pts = []
    for i in range(ring + 1):
        f = math.pi * i / ring
        pts.append([s, -dd * (math.sin(f) ** 0.8), wn * math.cos(f) * (1 - k * math.sin(f))])
    return pts


def neck_mesh(top=0.0, stations=None):
    """🦒 Neck and heel polyhedron along the neck bone."""
    st = stations or [0.0, 0.004, 0.01, 0.02, 0.03, 0.045, 0.06, 0.075, 0.1, 0.15, 0.2, 0.25, 0.3, NUT_S, NUT_S + 0.005]
    rings = []
    for s in st:
        r = neck_section(s)
        if top:
            r = [[p[0], p[1] if p[1] < -1e-9 or abs(p[2]) < 1e-9 else p[1] + top, p[2]] for p in r]
            r[0][1] = top
            r[-1][1] = top
        rings.append(r)
    verts, tris = loft_mesh(rings)
    return {"kind": "polyhedron", "vertices": verts, "triangles": [list(t) for t in tris]}


def whole_guitar_mesh():
    """🎸 The guitar at 1:200 as one polyhedron: tapering soundbox, neck with fretboard and the pitched headstock."""
    def sec(hw, v0, v1, n=12):
        r = min(0.004, hw * 0.5, (v1 - v0) * 0.4)
        pts = rounded_rect(0, (v0 + v1) / 2, hw, (v1 - v0) / 2, r, 2)
        return [(p[1], p[0]) for p in pts]

    rings = []
    us = [0.004, 0.012, 0.03, 0.06, 0.1, 0.15, 0.2, 0.25, 0.29, 0.31, 0.34, 0.39, 0.43, 0.46, 0.485, 0.5, 0.505]
    for u in us:
        rings.append([[u, v, w] for v, w in sec(half_width(u), -depth(u), -0.0)])
    for s in [0.0, 0.03, 0.08, 0.2, NUT_S]:
        rings.append([[BODY_L + s, v, w] for v, w in sec(fb_half(s), -neck_depth(s), 0.006)])
    for h in [0.012, 0.09, 0.178]:
        gu, gv = head_to_guitar(h, -0.00725)
        hw = 0.0215 + (0.037 - 0.0215) * min(1, h / 0.012) if h <= 0.012 else 0.037 + 0.006 * (h - 0.012) / 0.156
        rings.append([[gu, gv + v, w] for v, w in sec(hw, -0.00725, 0.00725)])
    verts, tris = loft_mesh(rings)
    return {"kind": "polyhedron", "vertices": verts, "triangles": [list(t) for t in tris]}


def capsule3(p0, p1, r0, r1=None):
    return {"kind": "capsule", "p0": [R6(x) for x in p0], "p1": [R6(x) for x in p1], "r0": r0, "r1": r0 if r1 is None else r1}


def pickguard():
    """🐢 Teardrop pickguard on the treble side of the soundhole (u, w)."""
    return ccw([(0.432, -0.040), (0.420, -0.052), (0.400, -0.061), (0.375, -0.065), (0.350, -0.062), (0.328, -0.054), (0.312, -0.042), (0.300, -0.050), (0.293, -0.068), (0.292, -0.088), (0.300, -0.108), (0.318, -0.124), (0.343, -0.132), (0.370, -0.131), (0.396, -0.122), (0.418, -0.106), (0.434, -0.086), (0.442, -0.064), (0.440, -0.050)])


WORN = (0.352, -0.094, 0.016, 0.012)
BRIDGE = ccw([(0.204, 0.076), (0.229, 0.076), (0.233, 0.05), (0.236, 0.02), (0.236, -0.02), (0.233, -0.05), (0.229, -0.076), (0.204, -0.076), (0.201, -0.04), (0.199, 0.0), (0.201, 0.04)])
HEAD_OUT = ccw([(0.0, 0.0215), (0.012, 0.037), (0.168, 0.043), (0.176, 0.041), (0.180, 0.035), (0.180, -0.035), (0.176, -0.041), (0.168, -0.043), (0.012, -0.037), (0.0, -0.0215)])
POSTS = [0.050, 0.090, 0.130]
STR_SADDLE = [0.02625, 0.01575, 0.00525, -0.00525, -0.01575, -0.02625]
STR_NUT = [0.0180, 0.0108, 0.0036, -0.0036, -0.0108, -0.0180]
STR_D = [0.00135, 0.00107, 0.00081, 0.00061, 0.00041, 0.00030]


def parts(s):
    """🧱 Every part at scale 1:s, each its own model for the smallest feature of that scale."""
    P = []
    f50, f100, f200 = s == 50, s == 100, s == 200
    outer = rounded_rect(0, 0, HZ, HX, RC, 5 if not f200 else 3)
    outer_xz = [(b, a) for a, b in outer]
    inner_xz = [(b, a) for a, b in rounded_rect(0, 0, HZ - SH, HX - SH, RC - SH, 5 if not f200 else 3)]
    lin_xz = [(b, a) for a, b in rounded_rect(0, 0, HZ - SH - LIN, HX - SH - LIN, RC - SH - LIN, 4 if not f200 else 3)]
    lid_outer = [(lid_u(a), b) for a, b in outer_xz]
    lid_inner = [(lid_u(a), b) for a, b in inner_xz]
    lid_lin = [(lid_u(a), b) for a, b in lin_xz]
    y0 = Y_FEET if f50 else 0.0
    P.append(Part("shell-bottom", "case", "body", "tolex-ply", "case-bottom", 0.22, "v", outer_xz, (y0, Y_SPLIT), d3={"kind": "extrude", "plane": "zx", "profile": [[b, a] for a, b in outer_xz], "holes": [[[b, a] for a, b in inner_xz]], "from": y0, "to": Y_SPLIT}))
    P.append(Part("shell-floor", "case", "body", "tolex-ply", "case-bottom", 0.22, "v", inner_xz, (y0, Y_FEET + SH), flags={"sheet": True}, d2=False))
    P.append(Part("shell-lid", "lid", "body", "tolex-ply", "case-lid", 0.26, "v", lid_outer, (0.0, LID_H), d3={"kind": "extrude", "plane": "zx", "profile": [[b, a] for a, b in lid_outer], "holes": [[[b, a] for a, b in lid_inner]], "from": 0.0, "to": LID_H}))
    P.append(Part("shell-lid-top", "lid", "body", "tolex-ply", "case-lid", 0.26, "v", lid_inner, (LID_H - SH, LID_H), flags={"sheet": True}, d2=False))
    if f50 or f100:
        vh = 0.008 if f50 else 0.015
        vo = [(b, a) for a, b in rounded_rect(0, 0, HZ + 0.001, HX + 0.001, RC + 0.001, 5)]
        P.append(Part("valance-bottom", "case", "detail", "nickel-steel", "case-hardware", 0.62, "v", vo, (Y_SPLIT - vh, Y_SPLIT), d3={"kind": "extrude", "plane": "zx", "profile": [[b, a] for a, b in vo], "holes": [[[b, a] for a, b in outer_xz]], "from": Y_SPLIT - vh, "to": Y_SPLIT}))
        vl = [(lid_u(a), b) for a, b in vo]
        P.append(Part("valance-lid", "lid", "detail", "nickel-steel", "case-hardware", 0.62, "v", vl, (0.0, vh), d3={"kind": "extrude", "plane": "zx", "profile": [[b, a] for a, b in vl], "holes": [[[b, a] for a, b in lid_outer]], "from": 0.0, "to": vh}))
    cs = {50: (0.034, 0.032, 0.034, 0.008), 100: (0.034, 0.032, 0.034, 0.006), 200: (0.034, 0.034, 0.034, None)}[s]
    for sx in (1, -1):
        for sz in (1, -1):
            cx, cz = sx * (HX - cs[0] / 2 + 0.0015), sz * (HZ - cs[2] / 2 + 0.0015)
            tag = f"{'f' if sx > 0 else 'r'}{'l' if sz > 0 else 'r'}"
            yb0 = Y_FEET if f50 else 0.0
            prof = rect(cx - cs[0] / 2, cz - cs[2] / 2, cx + cs[0] / 2, cz + cs[2] / 2)
            P.append(Part(f"corner-bottom-{tag}", "case", "detail", "nickel-steel", "case-hardware", 0.6, "v", prof, (yb0, yb0 + cs[1]), d3={**box3(cs[:3], (cx, yb0 + cs[1] / 2, cz), cs[3])}))
            dent = f50 and tag == "fl"
            lprof = [(lid_u(a), b) for a, b in prof]
            d3 = box3(cs[:3], (lid_u(cx), LID_H - cs[1] / 2, cz), cs[3])
            if dent:
                d3 = box3((cs[0] - 0.003, cs[1] - 0.004, cs[2] - 0.003), (lid_u(cx) - 0.0015, LID_H - cs[1] / 2 - 0.002, cz - 0.0015), cs[3], rotation=[0.0, 0.0, 0.0436, 0.999])
            P.append(Part(f"corner-lid-{tag}", "lid", "detail", "nickel-steel", "case-hardware", 0.6 if not dent else 0.52, "v", lprof, (LID_H - cs[1], LID_H), d3=d3))
    if f50:
        for i, (fx, fz) in enumerate(FEET):
            prof = circle(fx, fz, 0.0125, 16)
            P.append(Part(f"foot-{i + 1}", "case", "detail", "nickel-steel", "case-hardware", 0.55, "v", prof, (0.0, Y_FEET), d3={"kind": "revolve", "profile": [[0.0, 0.0], [0.0, 0.0125], [0.003, 0.0128], [0.006, 0.0105], [0.008, 0.006], [0.008, 0.0]], "axis": [0, 1, 0], "origin": [fx, 0.0, fz]}))
        for i, z in enumerate(LATCH_Z):
            P.append(Part(f"latch-base-{i + 1}", "case", "detail", "nickel-steel", "case-hardware", 0.66, "v", rect(HX, z - 0.02, HX + 0.009, z + 0.02), (0.090, Y_SPLIT), d3=box3((0.009, 0.030, 0.040), (HX + 0.0045, 0.105, z), 0.002)))
            P.append(Part(f"latch-lever-{i + 1}", f"latch-{i + 1}", "detail", "nickel-steel", "case-hardware", 0.72, "v", rect(0.0, -0.014, 0.040, 0.014), (-0.008, 0.0), d3=box3((0.040, 0.008, 0.028), (0.020, -0.004, 0.0), 0.0025)))
            P.append(Part(f"latch-catch-{i + 1}", "lid", "detail", "nickel-steel", "case-hardware", 0.66, "v", rect(lid_u(HX), z - 0.015, lid_u(HX) + 0.009, z + 0.015), (0.0, 0.012), d3=box3((0.009, 0.012, 0.030), (lid_u(HX) + 0.0045, 0.006, z), 0.002)))
        for i, z in enumerate(HINGE_Z):
            P.append(Part(f"hinge-case-{i + 1}", "case", "detail", "nickel-steel", "case-hardware", 0.6, "v", rect(-HX - 0.008, z - 0.03, -HX, z + 0.03), (0.098, Y_SPLIT), d3=box3((0.008, 0.022, 0.060), (-HX - 0.004, 0.109, z), 0.0015)))
            P.append(Part(f"hinge-lid-{i + 1}", "lid", "detail", "nickel-steel", "case-hardware", 0.6, "v", rect(-0.004, z - 0.03, 0.004, z + 0.03), (0.0, 0.020), d3=box3((0.008, 0.020, 0.060), (0.0, 0.010, z), 0.0015)))
        for k, dz in enumerate((-0.058, 0.058)):
            P.append(Part(f"handle-fitting-{k + 1}", "case", "detail", "nickel-steel", "case-hardware", 0.62, "v", rect(HX, Z_H + dz - 0.008, HX + 0.012, Z_H + dz + 0.008), (0.086, 0.108), d3=box3((0.012, 0.022, 0.016), (HX + 0.006, 0.097, Z_H + dz), 0.002)))
            P.append(Part(f"handle-arm-{k + 1}", "handle", "detail", "handle-vinyl", "case-handle", 0.18, "v", rect(-0.006, dz - 0.008, 0.026, dz + 0.008), (0.004, 0.014), d3=box3((0.032, 0.010, 0.016), (0.010, 0.009, dz), 0.003)))
        P.append(Part("handle-grip", "handle", "body", "handle-vinyl", "case-handle", 0.16, "v", rect(0.020, -0.066, 0.044, 0.066), (0.004, 0.026), d3={"kind": "extrude", "plane": "xy", "profile": [list(p) for p in rounded_rect(0.032, 0.015, 0.012, 0.011, 0.005, 3)], "from": -0.066, "to": 0.066}))
    else:
        hv = (-0.008, 0.026) if f100 else (-0.008, 0.026)
        hu = (-0.004, 0.044) if f100 else (-0.004, 0.044)
        P.append(Part("handle-grip", "handle", "body", "handle-vinyl", "case-handle", 0.16, "v", rect(hu[0], -0.066, hu[1], 0.066), hv, d3={"kind": "extrude", "plane": "xy", "profile": [list(p) for p in (rounded_rect((hu[0] + hu[1]) / 2, (hv[0] + hv[1]) / 2, (hu[1] - hu[0]) / 2, (hv[1] - hv[0]) / 2, 0.006, 3))], "from": -0.066, "to": 0.066}))
    P.append(Part("lining-floor", "case", "body", "plush", "case-lining", 0.42, "v", inner_xz, (Y_FEET + SH, Y_FLOOR), flags={"sheet": True}, d2=False))
    P.append(Part("lining-walls", "case", "body", "plush", "case-lining", 0.42, "v", inner_xz, (Y_FLOOR, Y_SPLIT), d3={"kind": "extrude", "plane": "zx", "profile": [[b, a] for a, b in inner_xz], "holes": [[[b, a] for a, b in lin_xz]], "from": Y_FLOOR, "to": Y_SPLIT}))
    P.append(Part("lining-lid", "lid", "body", "plush", "case-lining", 0.42, "v", lid_inner, (0.0, LID_H - SH - LIN), d3={"kind": "extrude", "plane": "zx", "profile": [[b, a] for a, b in lid_inner], "holes": [[[b, a] for a, b in lid_lin]], "from": 0.0, "to": LID_H - SH - LIN}, d2=False))
    P.append(Part("lining-ceiling", "lid", "body", "plush", "case-lining", 0.42, "v", lid_inner, (LID_H - SH - LIN, LID_H - SH), flags={"sheet": True}, d2=False))
    xin = HX - SH - LIN
    back_tail, back_heel = Y_FLOOR + 0.0035, G_Y - D_HEEL
    pad = ccw([(Y_FLOOR, -HZ + SH + LIN), (Y_FLOOR, -0.012), (back_heel + 0.0002, -0.012), (back_tail, G_Z), (back_tail, -HZ + SH + LIN)])
    P.append(Part("lining-pad", "case", "body", "plush", "case-lining", 0.4, "u", pad, (-xin, xin)))
    nb = G_Y - neck_depth(0.230 - G_Z - BODY_L)
    P.append(Part("neck-rest", "case", "body", "plush", "case-lining", 0.36, "v", rect(-0.03, 0.205, 0.03, 0.255), (Y_FLOOR, nb - 0.0002), d3=box3((0.06, nb - 0.0002 - Y_FLOOR, 0.05), (0.0, (Y_FLOOR + nb - 0.0002) / 2, 0.230), 0.004 if not f200 else None)))
    P.append(Part("compartment", "case", "body", "plush", "case-lining", 0.34, "v", rect(0.040, 0.0, xin, 0.240), (Y_FLOOR, CAPO_AT[1]), d3=box3((xin - 0.040, CAPO_AT[1] - Y_FLOOR, 0.240), ((0.040 + xin) / 2, (Y_FLOOR + CAPO_AT[1]) / 2, 0.120), 0.004 if not f200 else None)))
    if f50:
        P.append(Part("capo-bar", "capo", "detail", "capo-brass", "guitar-metal", 0.7, "v", rect(0.0, -0.008, 0.064, 0.008), (0.0, 0.012), d3=box3((0.064, 0.012, 0.016), (0.032, 0.006, 0.0), 0.003)))
        P.append(Part("capo-lever", "capo", "detail", "capo-brass", "guitar-metal", 0.76, "v", rect(0.010, -0.006, 0.058, 0.006), (0.012, 0.020), d3=box3((0.048, 0.008, 0.012), (0.034, 0.016, 0.0), 0.003)))
    elif f100:
        P.append(Part("capo-block", "capo", "detail", "capo-brass", "guitar-metal", 0.7, "v", rect(0.0, -0.0085, 0.064, 0.0085), (0.0, 0.017), d3=box3((0.064, 0.017, 0.017), (0.032, 0.0085, 0.0), 0.003)))
    if f50 or f100:
        tex = {"texture": True}
        for k, (u0, w0, u1, w1, tone) in enumerate([(0.10, -0.30, 0.19, -0.17, 0.78), (0.25, 0.05, 0.33, 0.15, 0.7), (0.30, 0.31, 0.37, 0.38, 0.86)]):
            P.append(Part(f"sticker-{k + 1}", "lid", "texture", "paper-label", "case-lid", tone, "v", rect(u0, w0, u1, w1), (LID_H, LID_H + 0.0003), flags=tex))
        P.append(Part("tolex-tear", "lid", "texture", "tolex-ply", "case-lid", 0.55, "v", ccw([(0.405, 0.47), (0.43, 0.485), (0.437, 0.53), (0.41, 0.545), (0.388, 0.52)]), (LID_H, LID_H + 0.0003), flags=tex))
    if f50:
        P.append(Part("dent-mark", "lid", "texture", "nickel-steel", "case-hardware", 0.3, "v", ccw(circle(0.433, 0.537, 0.006, 10)), (LID_H, LID_H + 0.0004), flags={"texture": True}))

    ol = outline(4 if f50 else 3)
    sh = circle(SH_U, 0.0, SH_R, 36 if f50 else 24)
    bm, _ = body_mesh(s)
    if f200:
        P.append(Part("guitar", "guitar", "body", "mahogany", "guitar-body", 0.4, "v", ol, (-D_TAIL, 0.0), tip=(-D_HEEL, 0.0), d3=whole_guitar_mesh(), regions=["guitar-body", "guitar-top"]))
        neckp = [(BODY_L, fb_half(0)), (BODY_L + NUT_S, fb_half(NUT_S))]
        hp = [head_to_guitar(h, 0)[0] for h in (0.012, 0.178)]
        np_ = ccw([(BODY_L - 0.004, fb_half(0)), (BODY_L + NUT_S, 0.0215), (hp[0], 0.037), (hp[1], 0.043), (hp[1], -0.043), (hp[0], -0.037), (BODY_L + NUT_S, -0.0215), (BODY_L - 0.004, -fb_half(0))])
        P.append(Part("neck-head", "guitar", "body", "mahogany", "guitar-body", 0.36, "v", np_, (-0.019, 0.006), d3=None, regions=["guitar-body", "guitar-rosewood", "guitar-metal"]))
    else:
        P.append(Part("guitar-box", "guitar", "body", "mahogany", "guitar-body", 0.4, "v", ol, (-D_TAIL, -0.0035), tip=(-D_HEEL, -0.0035), d3=bm))
    P.append(Part("top", "guitar", "body", "spruce-lacquer", "guitar-top", 0.82, "v", ol, (-0.0035, 0.0), holes=[ccw(sh)], flags={"sheet": True}, d2=not f200))
    P.append(Part("soundhole", "guitar", "detail", "mahogany", "guitar-body", 0.06, "v", sh, (-0.0035, -0.003), flags={"sheet": True}, d2=False))
    P.append(Part("soundhole-dark", "guitar", "detail", "mahogany", "guitar-body", 0.06, "v", sh, (-0.0035, 0.0002), d3=None))
    pg = pickguard()
    hole = ccw(circle(WORN[0], WORN[1], WORN[3], 16, WORN[2]))
    P.append(Part("pickguard", "guitar", "detail", "celluloid", "pickguard", 0.2, "v", pg, (0.0, 0.0006), holes=[hole] if not f200 else None, flags={"sheet": True}, d2=not f200))
    if not f200:
        P.append(Part("pickguard-worn-through", "guitar", "detail", "spruce-lacquer", "guitar-top", 0.9, "v", hole, (0.0006, 0.0008), d3=None))
    if f50:
        P.append(Part("worn-spot", "guitar", "texture", "spruce-bare", "guitar-top", 0.95, "v", ccw(circle(WORN[0], WORN[1], WORN[3] - 0.001, 14, WORN[2] - 0.001)), (0.0, 0.0002), flags={"texture": True}, d2=False))
        P.append(Part("rosette", "guitar", "texture", "celluloid", "guitar-top", 0.3, "v", circle(SH_U, 0.0, 0.060, 36), (0.0, 0.0003), holes=[ccw(circle(SH_U, 0.0, 0.055, 36))], flags={"texture": True}, filled=False))
        P.append(Part("arm-wear", "guitar", "texture", "spruce-bare", "guitar-top", 0.95, "v", ccw([(0.11, 0.183), (0.16, 0.191), (0.215, 0.180), (0.24, 0.166), (0.20, 0.168), (0.15, 0.176)]), (0.0, 0.0003), flags={"texture": True}))
        P.append(Part("pick-scratches", "guitar", "texture", "spruce-bare", "guitar-top", 0.9, "v", ccw([(0.265, -0.060), (0.29, -0.052), (0.30, -0.085), (0.285, -0.112), (0.265, -0.10)]), (0.0, 0.0003), flags={"texture": True}))
    if f50:
        P.append(Part("bridge", "guitar", "detail", "rosewood", "guitar-rosewood", 0.18, "v", BRIDGE, (-0.0005, 0.009)))
        P.append(Part("saddle", "guitar", "texture", "bone-white", "guitar-rosewood", 0.92, "v", rect(SADDLE_U - 0.0015, -0.036, SADDLE_U + 0.0015, 0.036), (0.009, SADDLE_TOP), flags={"texture": True}, filled=False))
        for i, w in enumerate(STR_SADDLE):
            P.append(Part(f"bridge-pin-{i + 1}", "guitar", "texture", "bone-white", "guitar-rosewood", 0.9, "v", circle(0.226, w, 0.0028, 8), (0.009, 0.0115), d3={"kind": "capsule", "p0": [0.226, 0.009, w], "p1": [0.226, 0.0105, w], "r0": 0.0028, "r1": 0.0028}, flags={"texture": True}))
        P.append(Part("end-pin", "guitar", "detail", "rosewood", "guitar-rosewood", 0.15, "v", rect(-0.010, -0.0045, 0.0, 0.0045), (-0.0665, -0.0575), d3=capsule3((-0.010, -0.062, 0.0), (0.004, -0.062, 0.0), 0.0045)))
        for i in range(6):
            r = STR_D[i] / 2
            a = (SADDLE_U, SADDLE_TOP + r, STR_SADDLE[i])
            b = (BODY_L + NUT_S, NUT_TOP + r, STR_NUT[i])
            pw = 0.028 if i < 3 else -0.028
            post = head_to_guitar(POSTS[[0, 1, 2, 2, 1, 0][i]], 0.006)
            P.append(Part(f"string-{i + 1}", "guitar", "texture", "string-metal", "guitar-metal", 0.72, "v", ccw([(a[0], a[2] - max(r, 0.0004)), (b[0], b[2] - max(r, 0.0004)), (b[0], b[2] + max(r, 0.0004)), (a[0], a[2] + max(r, 0.0004))]), (NUT_TOP, SADDLE_TOP + 0.0015), d3=capsule3(a, b, r), flags={"slender": True}, filled=False))
            P.append(Part(f"string-head-{i + 1}", "guitar", "texture", "string-metal", "guitar-metal", 0.72, "v", None, None, d3=capsule3(b, (post[0], post[1], pw), r), flags={"slender": True, "texture": True}, d2=False))
    elif f100:
        P.append(Part("bridge", "guitar", "texture", "rosewood", "guitar-rosewood", 0.18, "v", BRIDGE, (-0.0005, 0.009), flags={"texture": True}))

    if f50:
        nm = neck_mesh()
        P.append(Part("neck", "neck", "body", "mahogany", "guitar-body", 0.42, "v", ccw([(0.0, fb_half(0)), (NUT_S + 0.005, fb_half(NUT_S)), (NUT_S + 0.005, -fb_half(NUT_S)), (0.0, -fb_half(0))]), (-neck_depth(0), 0.0), tip=(-neck_depth(NUT_S), 0.0), d3=nm))
        fbp = ccw([(fret_s(20) - 0.004, fb_half(fret_s(20) - 0.004)), (NUT_S, fb_half(NUT_S)), (NUT_S, -fb_half(NUT_S)), (fret_s(20) - 0.004, -fb_half(fret_s(20) - 0.004))])
        P.append(Part("fretboard", "neck", "body", "rosewood", "guitar-rosewood", 0.16, "v", fbp, (-0.0015, 0.006)))
        for n in range(1, 21):
            fs = fret_s(n)
            hw = fb_half(fs)
            P.append(Part(f"fret-{n}", "neck", "texture", "string-metal", "guitar-metal", 0.7, "v", rect(fs - 0.0012, -hw, fs + 0.0012, hw), (0.006, 0.0072), d3=box3((0.0024, 0.0012, 2 * hw), (fs, 0.0066, 0.0)), flags={"texture": True}, filled=False))
        for n in (3, 5, 7, 9, 15):
            mid = (fret_s(n) + fret_s(n - 1)) / 2
            P.append(Part(f"inlay-{n}", "neck", "texture", "bone-white", "guitar-rosewood", 0.9, "v", circle(mid, 0.0, 0.003, 10), (0.006, 0.0062), d3={"kind": "capsule", "p0": [mid, 0.0059, 0.0], "p1": [mid, 0.0061, 0.0], "r0": 0.003, "r1": 0.003}, flags={"texture": True}))
        P.append(Part("fretboard-wear", "neck", "texture", "rosewood", "guitar-rosewood", 0.32, "v", ccw([(fret_s(5), -0.014), (fret_s(0) - 0.002, -0.012), (fret_s(0) - 0.002, 0.014), (fret_s(5), 0.016)]), (0.006, 0.0063), flags={"texture": True}))
        P.append(Part("nut", "neck", "texture", "bone-white", "guitar-rosewood", 0.92, "v", rect(NUT_S - 0.005, -0.0215, NUT_S, 0.0215), (0.006, NUT_TOP), flags={"texture": True}, filled=False))
        heel = ccw([(0.0, -neck_depth(0) + 0.001), (0.0, -0.092), (0.012, -0.0752), (0.03, -0.051), (0.05, -0.027), (0.07, -0.0165), (0.078, -neck_depth(0.078) + 0.001)])
        P.append(Part("heel", "neck", "body", "mahogany", "guitar-body", 0.38, "w", heel, (-0.026, 0.026), d3=None))
        P.append(Part("head", "head", "body", "mahogany", "guitar-body", 0.3, "v", HEAD_OUT, (-0.0145, 0.0)))
        P.append(Part("head-veneer", "head", "texture", "rosewood", "guitar-rosewood", 0.14, "v", inset(HEAD_OUT, 0.004), (0.0, 0.0003), flags={"texture": True}, d2=False))
        for k, h in enumerate(POSTS):
            for side in (1, -1):
                tag = f"{k + 1}{'b' if side > 0 else 't'}"
                P.append(Part(f"tuner-housing-{tag}", "head", "detail", "nickel-steel", "guitar-metal", 0.64, "v", rect(h - 0.011, side * 0.036 - 0.015, h + 0.011, side * 0.036 + 0.015), (-0.0265, -0.0145), d3=box3((0.022, 0.012, 0.030), (h, -0.0205, side * 0.036), 0.003)))
                P.append(Part(f"tuner-button-{tag}", "head", "detail", "nickel-steel", "guitar-metal", 0.7, "v", circle(h, side * 0.061, 0.010, 12, 0.011), (-0.024, -0.016), d3={"kind": "ellipsoid", "radii": [0.011, 0.004, 0.0101], "transform": {"translation": [h, -0.020, side * 0.0612]}}))
                P.append(Part(f"tuner-post-{tag}", "head", "texture", "nickel-steel", "guitar-metal", 0.7, "v", None, None, d3=capsule3((h, 0.0, side * 0.028), (h, 0.009, side * 0.028), 0.003), flags={"texture": True}, d2=False))
                P.append(Part(f"string-end-{tag}", "head", "texture", "string-metal", "guitar-metal", 0.6, "v", None, None, d3=capsule3((h, 0.009, side * 0.028), (h + 0.012 * side, 0.016, side * (0.028 + 0.03)), 0.0006), flags={"texture": True, "slender": True}, d2=False))
    else:
        top = 0.006 if f100 else 0.0
        if f100:
            P.append(Part("neck", "neck", "body", "mahogany", "guitar-body", 0.42, "v", ccw([(0.0, fb_half(0)), (NUT_S + 0.005, fb_half(NUT_S)), (NUT_S + 0.005, -fb_half(NUT_S)), (0.0, -fb_half(0))]), (-neck_depth(0), 0.006), tip=(-neck_depth(NUT_S), 0.006), d3=neck_mesh(top, [0.0, 0.01, 0.025, 0.045, 0.065, 0.08, 0.2, NUT_S, NUT_S + 0.005]), regions=["guitar-body", "guitar-rosewood"]))
            heel = ccw([(0.0, -neck_depth(0) + 0.001), (0.0, -0.092), (0.02, -0.064), (0.05, -0.027), (0.078, -neck_depth(0.078) + 0.001)])
            P.append(Part("heel", "neck", "body", "mahogany", "guitar-body", 0.38, "w", heel, (-0.026, 0.026), d3=None))
            hw = ccw([(0.0, 0.0215), (0.012, 0.072), (0.150, 0.075), (0.180, 0.045), (0.180, -0.045), (0.150, -0.075), (0.012, -0.072), (0.0, -0.0215)])
            P.append(Part("head", "head", "body", "mahogany", "guitar-body", 0.3, "v", hw, (-0.015, 0.0), regions=["guitar-body", "guitar-metal"]))

    strap = [("strap-tab", "tab"), ("strap-rear", "rear"), ("strap-cross", "cross"), ("strap-front", "front")]
    for bid, tag in strap:
        L = BY[bid]["lengthM"]
        P.append(Part(f"strap-{tag}", bid, "detail", "leather", "strap", 0.3, "v", rect(0.0, -0.025, L, 0.025), (0.0, 0.0024), d3=box3((L, 0.0024, 0.050), (L / 2, 0.0012, 0.0)), flags={"sheet": True}, filled=not f200))
    if f50:
        L = BY["strap-front"]["lengthM"]
        P.append(Part("strap-lace", "strap-front", "texture", "leather", "strap", 0.25, "v", None, None, d3=capsule3((L - 0.004, 0.0012, 0.0), (L + 0.042, 0.0012, 0.006), 0.001), flags={"texture": True, "slender": True}, d2=False))
    return P


# ---------------------------------------------------------------- thermal and structure lists

HIDDEN = {"case-lining", "guitar-top", "guitar-body", "guitar-rosewood", "guitar-metal", "pickguard", "strap"}


def thermal_list(s, P):
    """🌡️ Thermal solids: references for the outer case surfaces, own solids for the regions inside the closed case."""
    out = []
    for p in P:
        sol = p.solid3()
        if sol is None or (sol.get("texture") and not sol.get("slender")):
            continue
        if p.region in HIDDEN:
            out.append({**sol, "id": "t-" + p.id, "region": p.region})
        else:
            out.append({"id": "t-" + p.id, "of": p.id, "region": p.region})
    if s != 50:
        for i in range(6):
            r = max(STR_D[i] / 2, 0.0004)
            out.append({**capsule3((SADDLE_U, SADDLE_TOP + r, STR_SADDLE[i]), (BODY_L + NUT_S, NUT_TOP + r, STR_NUT[i]), r), "id": f"t-string-{i + 1}", "bone": "guitar", "material": "string-metal", "role": "texture", "tone": 0.7, "slender": True, "region": "guitar-metal"})
        if s == 100:
            for side in (1, -1):
                out.append({**box3((0.11, 0.015, 0.015), (0.09, -0.022, side * 0.06)), "id": f"t-tuner-row-{'b' if side > 0 else 't'}", "bone": "head", "material": "nickel-steel", "role": "detail", "tone": 0.64, "region": "guitar-metal"})
        fbp = [[w, u] for u, w in ccw([(fret_s(20) - 0.004, fb_half(fret_s(20) - 0.004)), (NUT_S, fb_half(NUT_S)), (NUT_S, -fb_half(NUT_S)), (fret_s(20) - 0.004, -fb_half(fret_s(20) - 0.004))])]
        out.append({"id": "t-fretboard-layer", "kind": "extrude", "plane": "zx", "profile": fbp, "from": 0.0, "to": 0.006, "bone": "neck", "material": "rosewood", "role": "body", "tone": 0.16, "region": "guitar-rosewood", "layer": "fretboard"})
    return out


def structure_list(s):
    """🏗️ The load path: shells as channels, the soundbox as a box girder, neck, headstock and strap runs as members, hinges, pivots, the lining pads and the feet as joints, two mass markers."""
    f = {50: 0.0075, 100: 0.015, 200: 0.030}[s]
    out = []

    def m(i, bone, part, geo, member=None, members=None, **kw):
        d = {"id": i, "bone": bone, "material": kw.pop("material", "tolex-ply"), "role": "body", "tone": 0.5, **geo, **kw, "member": member or bone, "part": part}
        if members:
            d["members"] = members
        out.append(d)

    y0 = Y_FEET if s == 50 else 0.0
    u = [[-HX, y0], [HX, y0], [HX, Y_SPLIT], [HX - SH, Y_SPLIT], [HX - SH, y0 + SH], [-HX + SH, y0 + SH], [-HX + SH, Y_SPLIT], [-HX, Y_SPLIT]]
    m("s-case", "case", "member", {"kind": "extrude", "plane": "xy", "profile": [[R6(a), R6(b)] for a, b in u], "from": -HZ, "to": HZ}, members=[f"latch-{i + 1}" for i in range(4)] if s != 50 else None)
    ul = [[0.004, LID_H], [0.004, 0.0], [0.004 + SH, 0.0], [0.004 + SH, LID_H - SH], [0.449 - SH, LID_H - SH], [0.449 - SH, 0.0], [0.449, 0.0], [0.449, LID_H]]
    m("s-lid", "lid", "member", {"kind": "extrude", "plane": "xy", "profile": [[R6(a), R6(b)] for a, b in ul], "from": -HZ, "to": HZ})
    hj = max(0.008, f)
    for i, z in enumerate(HINGE_Z):
        m(f"s-hinge-{i + 1}", "lid", "joint", box3((hj, 0.042, 0.060), (0.004 - hj / 2, 0.0, z)), material="nickel-steel")
    if s == 50:
        for i, z in enumerate(LATCH_Z):
            m(f"s-latch-{i + 1}", f"latch-{i + 1}", "member", box3((0.040, 0.008, 0.028), (0.020, -0.004, 0.0), 0.0025), material="nickel-steel")
            m(f"s-latch-pivot-{i + 1}", "case", "joint", box3((0.009, 0.030, 0.040), (HX + 0.0045, 0.105, z)), member=f"latch-{i + 1}", material="nickel-steel")
        for k, (fx, fz) in enumerate(FEET):
            m(f"s-foot-{k + 1}", "case", "joint", box3((0.024, Y_FEET, 0.024), (fx, Y_FEET / 2, fz)), material="nickel-steel")
    hv = (0.004, 0.026) if s == 50 else ((-0.008, 0.026) if s == 100 else (-0.008, 0.026))
    m("s-handle", "handle", "member", {"kind": "extrude", "plane": "xy", "profile": [list(p) for p in rounded_rect(0.032 if s == 50 else 0.020, (hv[0] + hv[1]) / 2, 0.012 if s == 50 else 0.024, (hv[1] - hv[0]) / 2, 0.004, 2)], "from": -0.066, "to": 0.066}, material="handle-vinyl")
    if s == 50:
        for k, dz in enumerate((-0.058, 0.058)):
            m(f"s-handle-pivot-{k + 1}", "case", "joint", box3((0.012, 0.022, 0.016), (HX + 0.006, 0.097, Z_H + dz)), member="handle", material="nickel-steel")
            m(f"s-handle-arm-{k + 1}", "handle", "joint", box3((0.032, 0.010, 0.016), (0.010, 0.009, dz)), member="handle", material="handle-vinyl")
    if s == 200:
        xin = HX - SH - LIN
        m("s-capo-rest", "case", "member", box3((xin - 0.040, CAPO_AT[1] + 0.020 - y0 - SH, 0.240), ((0.040 + xin) / 2, (y0 + SH + CAPO_AT[1] + 0.020) / 2, 0.120)), member="capo", material="plush")
    else:
        m("s-capo", "capo", "member", box3((0.064, 0.012 if s == 50 else 0.017, 0.016 if s == 50 else 0.017), (0.032, 0.006 if s == 50 else 0.0085, 0.0)), material="capo-brass")
        m("s-capo-rest", "case", "joint", box3((0.10, CAPO_AT[1] - (y0 + SH), 0.10), (CAPO_AT[0], (y0 + SH + CAPO_AT[1]) / 2, CAPO_AT[2])), member="capo", material="plush")
    gb = [[-D_TAIL, 0.09], [-D_TAIL, -0.09], [0.0, -0.09], [0.0, 0.09]]
    gh = [[-D_TAIL + 0.003, 0.087], [-D_TAIL + 0.003, -0.087], [-0.003, -0.087], [-0.003, 0.087]]
    m("s-guitar", "guitar", "member", {"kind": "extrude", "plane": "yz", "profile": gb, "holes": [gh], "from": 0.015, "to": 0.497}, material="mahogany")
    jy0 = y0 + SH
    jt = G_Y - D_TAIL + 0.001
    if jt - jy0 < f:
        jy0 = jt - f - 0.0005
    m("s-guitar-rest", "case", "joint", box3((0.16, jt - jy0, 0.46), (0.0, (jy0 + jt) / 2, -0.275)), member="guitar", material="plush")
    nbm = 0.1455 - 0.0175
    m("s-neck-rest", "case", "joint", box3((0.04, nbm + 0.001 - (y0 + SH), 0.05), (0.0, (y0 + SH + nbm + 0.001) / 2, 0.230)), member="neck", material="plush")
    if s == 200:
        tip = head_to_guitar(HEAD_L, -0.007)
        tu, tv = tip[0] - BODY_L, tip[1]
        m("s-neck", "neck", "member", {"kind": "extrude", "plane": "xy", "profile": [[0.0, -0.020], [R6(tu), R6(tv - 0.012)], [R6(tu), R6(tv + 0.012)], [0.0, 0.004]], "from": -0.012, "to": 0.012}, members=["head"], material="mahogany", slender=True)
    else:
        nw = 0.024
        m("s-neck", "neck", "member", {"kind": "extrude", "plane": "yz", "profile": [[-0.0175, -nw], [-0.0175, nw], [0.006, nw], [0.006, -nw]], "from": 0.0, "to": NUT_S}, material="mahogany")
        m("s-head", "head", "member", {"kind": "extrude", "plane": "zx", "profile": [[w, h] for h, w in HEAD_OUT], "from": -0.015 if s == 100 else -0.0145, "to": 0.0}, material="mahogany")
    m("s-end-pin", "guitar", "joint", box3((0.035, D_TAIL, 0.08), (0.0075, -D_TAIL / 2, 0.0)), member="strap-tab", material="rosewood")
    for bid in ["strap-tab", "strap-rear", "strap-cross", "strap-front"]:
        L = BY[bid]["lengthM"]
        m("s-" + bid, bid, "member", box3((L, 0.0024, 0.050), (L / 2, 0.0012, 0.0)), material="leather", sheet=True)
    mr = max(0.008, f / 2 + 0.001)
    cc = com(REST)
    m("s-mass-case", "case", "mass", {"kind": "ellipsoid", "radii": [mr, mr, mr], "transform": {"translation": [0.0, 0.07, 0.0]}}, material="nickel-steel")
    gc = [0.30, -0.05, 0.0]
    m("s-mass-guitar", "guitar", "mass", {"kind": "ellipsoid", "radii": [mr, mr, mr], "transform": {"translation": gc}}, material="nickel-steel")
    return out


# ---------------------------------------------------------------- materials and thermal regions

def mat(i, en, de, eps, alpha, rho, lam, cp, src, ref, e=None, strength=None):
    d = {"id": i, "title": T(en, de), "emissivity": eps, "solarAbsorptivity": alpha, "densityKgM3": rho, "conductivityWmK": lam, "specificHeatJKgK": cp}
    if e:
        d["elasticModulusPa"] = e
    if strength:
        d["strengthPa"] = strength
    d["source"], d["reference"] = src, ref
    return d


MATERIALS = [
    mat("tolex-ply", "Tolex-covered plywood shell, scuffed (4 mm laminate: 3.5 mm poplar plywood, 0.5 mm PVC)", "Tolexbezogene Sperrholzschale, abgestoßen (4 mm Verbund: 3,5 mm Pappelsperrholz, 0,5 mm PVC)", 0.92, 0.88, 690, 0.15, 1550, "literature", "iso-10456-2007", 6.0e9, 2.5e7),
    mat("nickel-steel", "Nickel-plated steel hardware, tarnished", "Vernickelte Stahlbeschläge, angelaufen", 0.18, 0.5, 7850, 50, 470, "literature", "incropera-2018", 2.0e11, 3.0e8),
    mat("handle-vinyl", "Vinyl handle grip over a steel core, worn shiny", "Vinylgriff über Stahlkern, blank gegriffen", 0.9, 0.85, 1400, 0.2, 1300, "authored-estimate", "iso-10456-2007", 3.0e9),
    mat("plush", "Plush pile lining over polyurethane foam, faded", "Plüschfutter über PU-Schaum, verblasst", 0.95, 0.7, 70, 0.045, 1400, "literature", "iso-10456-2007"),
    mat("paper-label", "Old paper stickers, yellowed", "Alte Papieraufkleber, vergilbt", 0.92, 0.4, 800, 0.12, 1300, "authored-estimate", "iso-10456-2007"),
    mat("spruce-lacquer", "Spruce top under aged nitrocellulose lacquer (ambered)", "Fichtendecke unter gealtertem Nitrolack (nachgedunkelt)", 0.91, 0.55, 430, 0.12, 1600, "literature", "fpl-wood-handbook", 1.08e10, 6.5e7),
    mat("spruce-bare", "Spruce worn bare of lacquer (pick and arm wear)", "Lackfrei abgespielte Fichte (Plektrum- und Armabrieb)", 0.9, 0.45, 430, 0.12, 1600, "literature", "fpl-wood-handbook"),
    mat("mahogany", "Mahogany back, sides and neck under aged lacquer", "Mahagoni für Boden, Zargen und Hals unter gealtertem Lack", 0.9, 0.72, 560, 0.15, 1600, "literature", "fpl-wood-handbook", 1.03e10, 7.4e7),
    mat("rosewood", "Rosewood fretboard, bridge and head veneer, oiled", "Palisander für Griffbrett, Steg und Kopfplattenfurnier, geölt", 0.9, 0.85, 850, 0.2, 1500, "literature", "fpl-wood-handbook", 1.2e10, 1.0e8),
    mat("celluloid", "Tortoise celluloid pickguard", "Schildpatt-Zelluloid-Schlagbrett", 0.92, 0.85, 1350, 0.21, 1400, "literature", "incropera-2018"),
    mat("bone-white", "Bone saddle, nut, bridge pins and dots", "Knochensattel, Stegeinlage, Stegpins und Punkte", 0.9, 0.35, 1900, 0.4, 1300, "authored-estimate", "incropera-2018"),
    mat("string-metal", "Phosphor-bronze wound and plain steel strings, nickel-silver frets", "Phosphorbronze-umsponnene und blanke Stahlsaiten, Neusilberbünde", 0.2, 0.55, 8300, 60, 420, "literature", "incropera-2018", 2.0e11),
    mat("capo-brass", "Nickel-plated brass capo with rubber pad", "Vernickelter Messingkapodaster mit Gummiauflage", 0.2, 0.5, 8500, 110, 380, "literature", "incropera-2018", 1.0e11),
    mat("leather", "Leather strap, sweat-darkened, with a lace end", "Ledergurt, schweißdunkel, mit Schnurende", 0.93, 0.8, 900, 0.16, 1500, "literature", "iso-10456-2007", 1.5e8, 2.0e7),
]

INSIDE = T("Inside the closed hardshell case in the stored default pose: it shows only when the lid is open or the guitar is played.", "Im geschlossenen Hartschalenkoffer in der gelagerten Standardpose: sichtbar nur bei offenem Deckel oder beim Spielen.")


def region(i, en, de, m, area, lc, geom, svf, layers, hc, hidden=False, power=False):
    d = {"id": i, "title": T(en, de), "kind": "surface", "model": "passive", "material": m, "areaM2": area, "characteristicLengthM": lc, "geometry": geom, "skyViewFactor": svf, "layers": layers, "heatCapacityJM2K": hc}
    if power:
        d["ratedPowerW"], d["ratedPowerBasis"] = 0, "passive"
    if hidden:
        d["hidden"] = INSIDE
    d["source"], d["reference"] = "authored-estimate", "incropera-2018"
    return d


def layer(i, en, de, kind, m, t, lam, rho, cp, ref):
    return {"id": i, "title": T(en, de), "kind": kind, "material": m, "thicknessM": t, "resistanceM2KW": R6(t / lam), "heatCapacityJM2K": round(t * rho * cp, 1), "source": "literature", "reference": ref}


AREAS = json.load(open(os.path.join(HERE, "areas.json"))) if os.path.exists(os.path.join(HERE, "areas.json")) else {}
A = lambda k, d: AREAS.get(k, d)
REGIONS = [
    region("case-bottom", "Bottom shell, tolex outside", "Unterschale, Tolex außen", "tolex-ply", A("case-bottom", 0.83), 0.445, "horizontal-plate-down", 0.0, [layer("shell", "Tolex-covered plywood 4 mm", "Tolexbezogenes Sperrholz 4 mm", "timber", "tolex-ply", 0.004, 0.15, 690, 1550, "iso-10456-2007")], 4278, power=True),
    region("case-lid", "Lid shell with stickers, tolex outside", "Deckelschale mit Aufklebern, Tolex außen", "tolex-ply", A("case-lid", 0.68), 0.445, "horizontal-plate-up", 0.9, [layer("shell", "Tolex-covered plywood 4 mm", "Tolexbezogenes Sperrholz 4 mm", "timber", "tolex-ply", 0.004, 0.15, 690, 1550, "iso-10456-2007")], 4278),
    region("case-hardware", "Latches, hinges, valances, corner caps and feet", "Verschlüsse, Scharniere, Randschienen, Eckkappen und Füße", "nickel-steel", A("case-hardware", 0.12), 0.03, "vertical-plate", 0.4, [], 7400),
    region("case-handle", "Carrying handle", "Tragegriff", "handle-vinyl", A("case-handle", 0.012), 0.024, "cylinder", 0.5, [], 9000),
    region("case-lining", "Plush lining, compartment and neck rest", "Plüschfutter, Fach und Halsauflage", "plush", A("case-lining", 2.0), 0.2, "horizontal-plate-up", 0.0, [layer("plush", "Plush over foam 6 mm", "Plüsch über Schaum 6 mm", "insulation", "plush", 0.006, 0.045, 70, 1400, "iso-10456-2007")], 588, hidden=True),
    region("guitar-top", "Spruce top, both faces", "Fichtendecke, beide Seiten", "spruce-lacquer", A("guitar-top", 0.33), 0.4, "vertical-plate", 0.5, [layer("top", "Spruce top 3.5 mm with lacquer", "Fichtendecke 3,5 mm mit Lack", "timber", "spruce-lacquer", 0.0035, 0.12, 430, 1600, "fpl-wood-handbook")], 2408, hidden=True),
    region("guitar-body", "Back, sides, neck and headstock (closed soundbox)", "Boden, Zargen, Hals und Kopf (geschlossener Klangkörper)", "mahogany", A("guitar-body", 0.55), 0.4, "vertical-plate", 0.5, [], 2800, hidden=True),
    region("guitar-rosewood", "Fretboard, bridge and end pin", "Griffbrett, Steg und Gurtpin", "rosewood", A("guitar-rosewood", 0.06), 0.05, "horizontal-plate-up", 0.5, [layer("fretboard", "Rosewood fretboard 6 mm", "Palisandergriffbrett 6 mm", "timber", "rosewood", 0.006, 0.2, 850, 1500, "fpl-wood-handbook")], 7650, hidden=True),
    region("guitar-metal", "Strings, frets, tuners and capo", "Saiten, Bünde, Mechaniken und Kapodaster", "string-metal", A("guitar-metal", 0.05), 0.01, "cylinder", 0.5, [], 3500, hidden=True),
    region("pickguard", "Pickguard, worn through under the pick", "Schlagbrett, unter dem Plektrum durchgespielt", "celluloid", A("pickguard", 0.018), 0.12, "vertical-plate", 0.5, [], 945, hidden=True),
    region("strap", "Leather strap", "Ledergurt", "leather", A("strap", 0.28), 0.05, "vertical-plate", 0.3, [], 3240, hidden=True),
]
RIDS = [r["id"] for r in REGIONS]


# ---------------------------------------------------------------- anchors, loads, poses, clips

def anc(i, bone, x, y, z, kind, en, de, **kw):
    d = {"id": i, "bone": bone, "x": R6(x), "y": R6(y), "z": R6(z), "kind": kind, "title": T(en, de)}
    d.update(kw)
    return d


ANCHORS = [
    *[anc(f"foot-{['front-left', 'front-right', 'rear-left', 'rear-right'][k]}", "case", fx, 0.0, fz, "contact", f"Case foot {['front left', 'front right', 'rear left', 'rear right'][k]}", f"Kofferfuß {['vorn links', 'vorn rechts', 'hinten links', 'hinten rechts'][k]}", support="contact", footprintLengthM=0.025, footprintWidthM=0.025, frictionCoefficient=0.6) for k, (fx, fz) in enumerate(FEET)],
    *[anc(f"rear-edge-{s}", "case", -HX - 0.0015, Y_FEET, z, "contact", f"Rear bottom corner cap, {s}", f"Hintere untere Eckkappe, {'links' if s == 'left' else 'rechts'}", support="contact", footprintLengthM=0.02, footprintWidthM=0.03, frictionCoefficient=0.5) for s, z in (("left", HZ - 0.016), ("right", -HZ + 0.016))],
    anc("handle-grip", "handle", 0.032, 0.015, 0.0, "contact", "Hand on the handle grip", "Hand am Tragegriff", support="pin", footprintLengthM=0.09, footprintWidthM=0.03),
    anc("strap-shoulder", "strap-cross", BY["strap-cross"]["lengthM"], 0.0012, 0.0, "contact", "Strap over the left shoulder", "Gurt über der linken Schulter", support="pin", footprintLengthM=0.06, footprintWidthM=0.05),
    anc("belly-rest", "guitar", 0.27, -0.117, 0.0, "contact", "Back of the guitar against the player's belly", "Gitarrenboden am Bauch des Spielers", support="contact", normalDeg=0, footprintLengthM=0.12, footprintWidthM=0.1, frictionCoefficient=0.5),
    anc("string-bridge", "guitar", SADDLE_U, SADDLE_TOP, 0.0, "load", "String pull at the saddle", "Saitenzug am Stegsattel"),
    anc("string-nut", "neck", NUT_S, NUT_TOP, 0.0, "load", "String pull at the nut", "Saitenzug am Sattel"),
    anc("strum-point", "guitar", 0.445, 0.011, 0.0, "load", "Pick strum on the strings", "Plektrumanschlag auf den Saiten"),
    anc("forearm-rest", "guitar", 0.17, 0.0, 0.19, "load", "Right forearm on the lower bout", "Rechter Unterarm auf der unteren Zarge"),
    anc("fret-hand", "neck", fret_s(2) + 0.012, 0.009, 0.0, "load", "Fretting hand at the second fret", "Greifhand am zweiten Bund"),
]


def tension_loads(frames):
    """🎻 The 0.72 kN of six strings as an equal and opposite pair at saddle and nut along the string line."""
    a = world(frames, "guitar", [SADDLE_U, SADDLE_TOP, 0])
    b = world(frames, "neck", [NUT_S, NUT_TOP, 0])
    d = norm(sub(b, a))
    f = scl(d, TENSION_N)
    mk = lambda i, an, v, en, de: {"id": i, "kind": "imposed", "title": T(en, de), "anchor": an, "fxN": round(v[0], 2), "fyN": round(v[1], 2), "fzN": round(v[2], 2), "source": "literature", "reference": "string-tension"}
    return [mk("strings-at-saddle", "string-bridge", f, "String tension 0.72 kN pulling the saddle towards the nut", "Saitenzug 0,72 kN zieht den Stegsattel zum Sattel"), mk("strings-at-nut", "string-nut", scl(f, -1), "String tension 0.72 kN pressing the neck towards the bridge", "Saitenzug 0,72 kN drückt den Hals zum Steg")]


def play_loads(frames):
    """✋ Forearm, pick and fretting hand of the played pose."""
    r, _ = frames["guitar"]
    w = [r[i][2] for i in range(3)]
    v = [r[i][1] for i in range(3)]
    pick = add(scl(w, -3.5), scl(v, -1.0))
    mk = lambda i, an, f, en, de: {"id": i, "kind": "live", "title": T(en, de), "anchor": an, "fxN": round(f[0], 2), "fyN": round(f[1], 2), "fzN": round(f[2], 2), "source": "authored-estimate", "reference": "playing-forces"}
    return [mk("forearm", "forearm-rest", [0, -12.0, 0], "Weight of the right forearm on the lower bout, 12 N", "Gewicht des rechten Unterarms auf der unteren Zarge, 12 N"), mk("strum-pick", "strum-point", pick, "Pick force of a downstroke, 3.6 N", "Plektrumkraft eines Abschlags, 3,6 N"), mk("fretting-hand", "fret-hand", [0, 4.0, 0], "Fretting hand holding the neck up, 4 N", "Greifhand hält den Hals, 4 N")]


def binding(exp, contact=None):
    contact = contact or {}
    regs = {}
    for rid in RIDS:
        b = {"exposedFraction": exp.get(rid, 1.0), "sunlitFraction": 0.0}
        if rid in contact:
            b["contact"] = {"fraction": contact[rid][0], "tempC": contact[rid][1]}
        regs[rid] = b
    return {"regions": regs, "emitters": {}}


CLOSED = {r: 0.0 for r in HIDDEN}
OPEN = {"case-lining": 0.85, "guitar-body": 0.65, "guitar-top": 1.0, "guitar-rosewood": 1.0, "guitar-metal": 1.0, "pickguard": 1.0, "strap": 0.9}
FEET_IDS = ["foot-front-left", "foot-front-right", "foot-rear-left", "foot-rear-right"]


def carried_root(handle_deg=90.0):
    """🧳 Root placement that hangs the case under the hand with its centre of mass below the grip."""
    off = {"handle": {"angleDeg": handle_deg}}
    fr = solve_rig(BONES, off)
    c = com(fr)
    g = world(fr, "handle", [0.032, 0.015, 0])
    th = -90 - math.degrees(math.atan2(c[1] - g[1], c[0] - g[0]))
    rr = rot(th)
    gw = mv(rr, g)
    return {"x": R6(0.0 - gw[0]), "y": R6(GRIP_Y - gw[1]), "angleDeg": R6(th)}


def tilt_root(deg):
    """↗️ Root placement that tilts the case about its rear bottom corner caps."""
    p = [-HX - 0.0015, Y_FEET, 0]
    q = mv(rot(deg), p)
    return {"x": R6(p[0] - q[0]), "y": R6(-q[1]), "angleDeg": deg}


def pose(i, en, de, den, dde, bones, root, contacts, therm, extra_loads=(), key=False):
    fr = solve_rig(BONES, bones, root)
    d = {"id": i, "title": T(en, de), "description": T(den, dde)}
    if key:
        d["key"] = True
    d.update({"bones": bones, "root": root, "ground": "authored", "contacts": contacts, "thermal": therm, "loads": tension_loads(fr) + list(extra_loads(fr) if callable(extra_loads) else extra_loads)})
    return d


OPEN_BONES = {"lid": {"angleDeg": LID_OPEN}, **{f"latch-{i + 1}": {"angleDeg": LATCH_OPEN} for i in range(4)}}
ZERO = {"x": 0.0, "y": 0.0, "angleDeg": 0.0}
CAR = carried_root()
POSES = [
    pose("stored", "Stored in the closed case", "Im geschlossenen Koffer gelagert", "The guitar lies in its rectangular hardshell case on the depot floor, latches shut, the strap folded over the strings, the capo on the compartment lid; strings tuned to pitch.", "Die Gitarre liegt im rechteckigen Hartschalenkoffer auf dem Lagerboden, Verschlüsse zu, der Gurt über den Saiten gefaltet, der Kapodaster auf dem Fachdeckel; Saiten auf Stimmung.", {}, ZERO, FEET_IDS, binding(CLOSED)),
    pose("case-open", "Case open", "Koffer offen", "Latches flipped and the lid open at 95 degrees on its hinge stop: the guitar on the plush, the worn pickguard and the folded strap in view.", "Verschlüsse geöffnet und der Deckel 95 Grad offen am Scharnieranschlag: die Gitarre auf dem Plüsch, das abgespielte Schlagbrett und der gefaltete Gurt sichtbar.", dict(OPEN_BONES), ZERO, FEET_IDS, binding(OPEN, {"guitar-body": (0.3, 20.0)})),
    pose("carried", "Carried by the handle", "Am Griff getragen", "The closed case hangs from one hand at the grip, 0.76 m above the floor, its centre of mass under the handle; the hand is a pin support.", "Der geschlossene Koffer hängt an einer Hand am Griff, 0,76 m über dem Boden, der Schwerpunkt unter dem Griff; die Hand ist ein Gelenklager.", {"handle": {"angleDeg": 90}}, CAR, ["handle-grip"], binding(CLOSED, {"case-handle": (0.6, 33.0)})),
    pose("played", "Played standing with the strap", "Im Stehen mit Gurt gespielt", "Out of the open case and on its strap: top facing the listener, neck raised 20 degrees, the strap from the end pin round the right flank and over the left shoulder to the lace on the headstock; forearm, pick and fretting hand as loads.", "Aus dem offenen Koffer am Gurt: Decke zum Zuhörer, Hals um 20 Grad angehoben, der Gurt vom Gurtpin um die rechte Flanke und über die linke Schulter zur Schnur an der Kopfplatte; Unterarm, Plektrum und Greifhand als Lasten.", dict(PLAY_OFF), ZERO, FEET_IDS + ["strap-shoulder", "belly-rest"], binding(OPEN, {"guitar-body": (0.12, 33.0), "guitar-top": (0.04, 33.0), "guitar-metal": (0.06, 33.0), "strap": (0.3, 32.0)}), play_loads),
    pose("carry-grip", "Hand on the handle", "Hand am Griff", "Key of the carry clip: the handle raised, the case still flat.", "Schlüssel des Trageclips: der Griff aufgestellt, der Koffer noch flach.", {"handle": {"angleDeg": 90}}, ZERO, FEET_IDS + ["handle-grip"], binding(CLOSED, {"case-handle": (0.6, 33.0)}), key=True),
    *[pose(f"carry-tilt-{d}", f"Tilted {d} degrees", f"Um {d} Grad gekippt", f"Key of the carry clip: the case tips up about its rear corner caps, {d} degrees.", f"Schlüssel des Trageclips: der Koffer kippt um die hinteren Eckkappen, {d} Grad.", {"handle": {"angleDeg": 90}}, tilt_root(d), ["rear-edge-left", "rear-edge-right", "handle-grip"], binding(CLOSED, {"case-handle": (0.6, 33.0)}), key=True) for d in (15, 30, 45)],
    pose("latches-open", "Latches open", "Verschlüsse offen", "Key of the case opening clip: the four levers flipped, the lid still down.", "Schlüssel des Öffnungsclips: die vier Hebel aufgeklappt, der Deckel noch zu.", {f"latch-{i + 1}": {"angleDeg": LATCH_OPEN} for i in range(4)}, ZERO, FEET_IDS, binding(CLOSED), key=True),
]


def key(t, p, ease="smooth", bones=None):
    d = {"t": t, "pose": p, "ease": ease}
    if bones:
        d["bones"] = bones
    return d


CLIPS = [
    {"id": "carry", "title": T("Pick up and carry", "Aufnehmen und tragen"), "description": T("The hand raises the handle, tips the case up about its rear corner caps and lifts it off to hang at the side.", "Die Hand stellt den Griff auf, kippt den Koffer um die hinteren Eckkappen und hebt ihn ab, bis er seitlich hängt."), "durationS": 3.2, "fps": 25, "loop": False, "rootMotion": "none",
     "keys": [key(0, "stored"), key(0.5, "carry-grip"), key(0.9, "carry-tilt-15"), key(1.3, "carry-tilt-30"), key(1.7, "carry-tilt-45"), key(2.6, "carried"), key(3.2, "carried")],
     "contacts": [{"anchor": "foot-front-left", "fromS": 0, "toS": 0.6}, {"anchor": "foot-front-right", "fromS": 0, "toS": 0.6}, {"anchor": "foot-rear-left", "fromS": 0, "toS": 0.75}, {"anchor": "foot-rear-right", "fromS": 0, "toS": 0.75}, {"anchor": "rear-edge-left", "fromS": 0.7, "toS": 1.9}, {"anchor": "rear-edge-right", "fromS": 0.7, "toS": 1.9}, {"anchor": "handle-grip", "fromS": 0.35, "toS": 3.2}],
     "phases": [{"id": "grip", "fromS": 0, "toS": 0.5, "title": T("Grip", "Greifen")}, {"id": "tilt", "fromS": 0.5, "toS": 1.7, "title": T("Tip up", "Aufkippen")}, {"id": "lift", "fromS": 1.7, "toS": 2.6, "title": T("Lift off", "Abheben")}, {"id": "carry", "fromS": 2.6, "toS": 3.2, "title": T("Carry", "Tragen")}],
     "events": [{"t": 0.35, "kind": "contact-on", "title": T("Hand closes on the grip", "Hand umfasst den Griff"), "target": "handle-grip"}, {"t": 1.9, "kind": "contact-off", "title": T("Case leaves the floor", "Koffer verlässt den Boden"), "target": "rear-edge-left"}], "tracks": []},
    {"id": "case-opening", "title": T("Open the case", "Koffer öffnen"), "description": T("The four draw latches flip open one after another, then the lid swings up to its 95 degree stop.", "Die vier Spannverschlüsse springen nacheinander auf, dann schwenkt der Deckel bis zum Anschlag bei 95 Grad."), "durationS": 4.0, "fps": 25, "loop": False, "rootMotion": "none",
     "keys": [key(0, "stored"), key(0.3, "stored", bones={"latch-1": {"angleDeg": LATCH_OPEN}}), key(0.6, "stored", bones={"latch-1": {"angleDeg": LATCH_OPEN}, "latch-2": {"angleDeg": LATCH_OPEN}}), key(0.9, "stored", bones={"latch-1": {"angleDeg": LATCH_OPEN}, "latch-2": {"angleDeg": LATCH_OPEN}, "latch-3": {"angleDeg": LATCH_OPEN}}), key(1.2, "latches-open"), key(3.5, "case-open"), key(4.0, "case-open")],
     "contacts": [{"anchor": a, "fromS": 0, "toS": 4.0} for a in FEET_IDS], "phases": [{"id": "latches", "fromS": 0, "toS": 1.2, "title": T("Latches", "Verschlüsse")}, {"id": "lid", "fromS": 1.2, "toS": 4.0, "title": T("Lid", "Deckel")}],
     "events": [{"t": 0.3 * (i + 1), "kind": "motion", "title": T(f"Latch {i + 1} springs open", f"Verschluss {i + 1} springt auf"), "target": f"latch-{i + 1}"} for i in range(4)] + [{"t": 3.5, "kind": "motion", "title": T("Lid on its stop", "Deckel am Anschlag"), "target": "lid"}], "tracks": []},
]


def strum_clip():
    """🎶 Four downstrokes in two seconds: the pick force pulses and the guitar rocks on its strap."""
    keys = []
    for k in range(9):
        t = round(k * 0.25, 3)
        g = {"angleDeg": PLAYED["tilt"] - (0.4 if k % 2 == 0 else 0.0), "twistDeg": 91.2 if k % 2 == 0 else 90.0}
        keys.append(key(t, "played", bones={"guitar": g}))
    pulses = []
    for k in range(4):
        t0 = k * 0.5
        pulses += [{"t": t0, "v": 1.0}, {"t": round(t0 + 0.08, 3), "v": 0.0}, {"t": round(t0 + 0.42, 3), "v": 0.0}]
    pulses.append({"t": 2.0, "v": 1.0})
    return {"id": "strumming", "title": T("Strumming standing", "Schlagen im Stehen"), "description": T("Four downstrokes per two seconds (120 beats per minute): the pick force pulses at the soundhole and the guitar rocks 1.2 degrees on the strap with every stroke.", "Vier Abschläge in zwei Sekunden (120 Schläge pro Minute): die Plektrumkraft pulsiert am Schallloch und die Gitarre wiegt sich bei jedem Schlag 1,2 Grad am Gurt."), "durationS": 2.0, "fps": 25, "loop": True, "rootMotion": "none", "keys": keys,
            "contacts": [{"anchor": a, "fromS": 0, "toS": 2.0} for a in FEET_IDS + ["strap-shoulder", "belly-rest"]], "phases": [{"id": f"beat-{k + 1}", "fromS": k * 0.5, "toS": (k + 1) * 0.5, "title": T(f"Beat {k + 1}", f"Schlag {k + 1}")} for k in range(4)],
            "events": [{"t": k * 0.5, "kind": "load-on", "title": T("Downstroke", "Abschlag"), "target": "strum-pick"} for k in range(4)], "tracks": [{"target": "load:strum-pick", "ease": "linear", "keys": pulses}]}


CLIPS.append(strum_clip())


# ---------------------------------------------------------------- representations and volumes

PENS = {50: {"body": 0.25, "detail": 0.2, "texture": 0.13, "glazing": None, "ground": 0.35}, 100: {"body": 0.2, "detail": 0.2, "texture": 0.1, "glazing": None, "ground": 0.25}, 200: {"body": 0.2, "detail": 0.2, "texture": None, "glazing": None, "ground": 0.25}}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350)}

DEC2D = {
    50: dict(
        silhouette=T("Every part is one extruded or revolved solid profile drawn in all three views, so the closed case reads in the side elevation as its 445 x 183 mm end with feet, valance band, latches and handle, in the front elevation as the 1105 mm long side with four latches and the handle, and in plan as the lid with dented corner caps and stickers; the guitar, the strap and the lining are drawn beneath and appear when the lid opens or the guitar is played.", "Jedes Teil ist ein extrudiertes oder rotiertes Körperprofil, das in allen drei Ansichten gezeichnet wird: Der geschlossene Koffer zeigt sich in der Seitenansicht als 445 x 183 mm Stirnseite mit Füßen, Randschiene, Verschlüssen und Griff, in der Vorderansicht als 1105 mm lange Seite mit vier Verschlüssen und Griff und im Grundriss als Deckel mit eingedellten Eckkappen und Aufklebern; Gitarre, Gurt und Futter liegen darunter und erscheinen bei offenem Deckel oder beim Spielen."),
        retained=T("Shells, valances, eight corner caps, four feet, four latches with catches, two hinges, the handle with its fittings, the lining with compartment, neck rest and body pad, the capo; the soundbox with tapering depth, top with soundhole and rosette, the pickguard with its worn-through hole, bridge, saddle, pins, end pin, six strings, neck with heel, fretboard with twenty frets and dots, nut, headstock with six tuners; the four strap runs.", "Schalen, Randschienen, acht Eckkappen, vier Füße, vier Verschlüsse mit Gegenstücken, zwei Scharniere, der Griff mit Beschlägen, das Futter mit Fach, Halsauflage und Korpuspolster, der Kapodaster; der Klangkörper mit abnehmender Tiefe, die Decke mit Schallloch und Rosette, das Schlagbrett mit durchgespieltem Loch, Steg, Stegeinlage, Pins, Gurtpin, sechs Saiten, Hals mit Halsfuß, Griffbrett mit zwanzig Bünden und Punkten, Sattel, Kopfplatte mit sechs Mechaniken; die vier Gurtläufe."),
        merged=T("Bracing, kerfed linings and the truss rod are inside the closed soundbox and neck and not drawn.", "Beleistung, Reifchen und Halsstab liegen im geschlossenen Korpus und Hals und werden nicht gezeichnet."),
        omitted=T("Tuner shafts, string wraps at the posts and the lace knot are below the drawing at 1:50 and only modelled.", "Mechanikwellen, Saitenwicklungen an den Wirbeln und der Schnurknoten liegen unter der Zeichnung bei 1:50 und sind nur modelliert."),
        thinElements=T("Strings, frets, saddle, nut and rosette are open texture outlines at the texture pen; the strap is a filled 2.4 mm band.", "Saiten, Bünde, Stegeinlage, Sattel und Rosette sind offene Texturumrisse mit dem Texturstift; der Gurt ist ein gefülltes 2,4-mm-Band."),
        hatching=T("No hatches: wood, tolex and metal are told apart by tone.", "Keine Schraffuren: Holz, Tolex und Metall unterscheiden sich durch den Ton."),
        styles=T("Dark tolex, light ambered spruce, mid mahogany, dark rosewood, bright tarnished nickel; the traces of use in their own tones: the worn-through spot in the pickguard and the bare arm wear on the lower bout, the scratched area beside the pickguard, the dented front left lid corner, the torn tolex and three old stickers.", "Dunkles Tolex, helle nachgedunkelte Fichte, mittleres Mahagoni, dunkles Palisander, helles angelaufenes Nickel; die Gebrauchsspuren in eigenen Tönen: das durchgespielte Loch im Schlagbrett und der blanke Armabrieb an der unteren Zarge, die zerkratzte Fläche neben dem Schlagbrett, die eingedellte vordere linke Deckelecke, das eingerissene Tolex und drei alte Aufkleber."),
        overlays=T("No overlays.", "Keine Überlagerungen.")),
    100: dict(
        silhouette=T("The same case and guitar for a 15 mm feature: the feet are merged into a bottom shell that stands on the floor, the handle is one grip block, latches, hinges and valances are left out, the corner caps stay.", "Derselbe Koffer und dieselbe Gitarre für 15 mm kleinste Einzelheit: die Füße sind in eine auf dem Boden stehende Unterschale eingezogen, der Griff ist ein Griffblock, Verschlüsse, Scharniere und Randschienen entfallen, die Eckkappen bleiben."),
        retained=T("Shells, corner caps, handle, lining with compartment, neck rest and pad, capo, soundbox, top with soundhole, pickguard with its worn hole, bridge, neck with heel and fretboard, the headstock with the tuner rows, the strap, the stickers and the torn tolex.", "Schalen, Eckkappen, Griff, Futter mit Fach, Halsauflage und Polster, Kapodaster, Klangkörper, Decke mit Schallloch, Schlagbrett mit Loch, Steg, Hals mit Fuß und Griffbrett, Kopfplatte mit den Mechanikreihen, Gurt, Aufkleber und Tolexriss."),
        merged=T("Fretboard into the neck, tuners into a widened headstock plate, the capo into one block.", "Griffbrett in den Hals, Mechaniken in eine verbreiterte Kopfplatte, Kapodaster in einen Block."),
        omitted=T("Latches, hinges, valances, feet, frets, dots, strings, saddle, pins, end pin and rosette.", "Verschlüsse, Scharniere, Randschienen, Füße, Bünde, Punkte, Saiten, Stegeinlage, Pins, Gurtpin und Rosette."),
        thinElements=T("The bridge is a texture; the strap a filled band.", "Der Steg ist Textur; der Gurt ein gefülltes Band."),
        hatching=T("No hatches.", "Keine Schraffuren."),
        styles=T("Tones as at 1:50.", "Töne wie bei 1:50."),
        overlays=T("No overlays.", "Keine Überlagerungen.")),
    200: dict(
        silhouette=T("For a 30 mm feature the case is its two shells with the corner caps and a 30 mm grip; the guitar is one outline of body, neck and head with the soundhole.", "Für 30 mm kleinste Einzelheit ist der Koffer seine zwei Schalen mit Eckkappen und einem 30-mm-Griff; die Gitarre ist ein Umriss aus Korpus, Hals und Kopf mit Schallloch."),
        retained=T("Shells, corner caps, handle, lining with compartment and neck rest, the guitar outline, soundhole, strap runs.", "Schalen, Eckkappen, Griff, Futter mit Fach und Halsauflage, Gitarrenumriss, Schallloch, Gurtläufe."),
        merged=T("Top, pickguard, fretboard and tuners into the guitar outline (its shapes list the merged regions).", "Decke, Schlagbrett, Griffbrett und Mechaniken in den Gitarrenumriss (die Formen nennen die zusammengefassten Regionen)."),
        omitted=T("All hardware but the corner caps, the capo, the bridge, stickers and every texture.", "Alle Beschläge außer den Eckkappen, der Kapodaster, der Steg, Aufkleber und jede Textur."),
        thinElements=T("The strap runs are open outlines, too thin to print filled at 1:350.", "Die Gurtläufe sind offene Umrisse, zu dünn, um bei 1:350 gefüllt zu drucken."),
        hatching=T("No hatches.", "Keine Schraffuren."),
        styles=T("Three tones: case, guitar, lining.", "Drei Töne: Koffer, Gitarre, Futter."),
        overlays=T("No overlays.", "Keine Überlagerungen.")),
}

DEC3D = {
    50: [T("Body: every part is the solid of its 2D profile or a handcrafted mesh with the same outline: the bottom shell and lid as hollow tolex trays 4 mm thick with plywood floor and top, 6 mm plush lining walls and floors, the body pad as a wedge under the tapering back, the neck rest block and the compartment; nickel valance bands 8 mm high, eight 34 mm corner caps (the front left lid cap dented and tilted), four domed feet, four draw latches with catches, two hinges with knuckles, the handle with fittings, arms and a rounded grip.", "Body: jedes Teil ist der Körper seines 2D-Profils oder ein handgefertigtes Netz mit demselben Umriss: Unterschale und Deckel als hohle Tolexschalen mit 4 mm Wand und Sperrholzboden und -decke, 6 mm Plüschwände und -böden, das Korpuspolster als Keil unter dem abfallenden Boden, Halsauflage und Fach; vernickelte Randschienen 8 mm hoch, acht 34-mm-Eckkappen (die vordere linke Deckelkappe eingedellt und verkippt), vier gewölbte Füße, vier Spannverschlüsse mit Gegenstücken, zwei Scharniere mit Gelenkrollen, der Griff mit Beschlägen, Bügeln und gerundetem Griffstück."),
         T("Guitar: the soundbox as a polyhedron of the dreadnought outline (lower bout 397 mm, waist 274 mm, upper bout 292 mm, 508 mm long) with depth from 124 to 98 mm and a chamfered back edge, the 3.5 mm spruce top as a sheet with the 101.6 mm soundhole cut through it, the 0.6 mm pickguard sheet with its worn-through hole over a bare spot, the belly bridge, the neck and heel as a swept C-section, the 7.5 mm fretboard, twenty fret wires, dots, nut, the headstock pitched back 14 degrees with six sealed tuners, posts and cut string ends, the six strings as slender capsules of their gauges, the end pin; the strap as four 2.4 mm leather sheets folded over the strings with a lace.", "Gitarre: der Klangkörper als Polyeder des Dreadnought-Umrisses (unterer Bug 397 mm, Taille 274 mm, oberer Bug 292 mm, 508 mm lang) mit 124 bis 98 mm Tiefe und gefaster Bodenkante, die 3,5-mm-Fichtendecke als Platte mit dem durchgehenden 101,6-mm-Schallloch, das 0,6-mm-Schlagbrett mit durchgespieltem Loch über einer blanken Stelle, der Bauchsteg, Hals und Halsfuß als gezogener C-Querschnitt, das 7,5-mm-Griffbrett, zwanzig Bunddrähte, Punkte, Sattel, die um 14 Grad geneigte Kopfplatte mit sechs gekapselten Mechaniken, Wirbeln und abgeschnittenen Saitenenden, die sechs Saiten als schlanke Kapseln ihrer Stärken, der Gurtpin; der Gurt als vier 2,4-mm-Lederplatten über den Saiten gefaltet, mit Schnur."),
         T("Thermal: the outer case surfaces reuse the body solids; the regions inside the closed case are their own solids, as hidden regions must be. Structure: both shells as open channels, latch levers, handle, capo, soundbox box girder, neck and headstock members and the strap runs, with hinges, latch bases, pivots, end pin, lining pads and feet as joints and two mass markers.", "Thermik: die äußeren Kofferflächen verwenden die Body-Körper; die Regionen im geschlossenen Koffer sind eigene Körper, wie verdeckte Regionen es verlangen. Struktur: beide Schalen als offene Kästen, Verschlusshebel, Griff, Kapodaster, Korpus als Kastenträger, Hals- und Kopfstäbe und die Gurtläufe, mit Scharnieren, Verschlussbasen, Drehpunkten, Gurtpin, Futterpolstern und Füßen als Gelenke und zwei Schwerpunktmarken.")],
    100: [T("Own model for a 15 mm feature: the bottom shell stands on the floor with the feet merged into a 12 mm floor; valances, latches, hinges and feet are left out; the handle is one 48 x 34 mm block; corner caps with a 6 mm radius; the capo one 17 mm block; the neck mesh carries the fretboard on top and fewer stations; the headstock is a 15 mm plate widened over the tuner rows; frets, strings, dots, nut, pins and end pin are left out of the body; the bridge is a texture; strap sheets as at 1:50.", "Eigenes Modell für 15 mm kleinste Einzelheit: die Unterschale steht mit in einen 12-mm-Boden eingezogenen Füßen auf dem Boden; Randschienen, Verschlüsse, Scharniere und Füße entfallen; der Griff ist ein 48 x 34 mm Block; Eckkappen mit 6 mm Radius; der Kapodaster ein 17-mm-Block; das Halsnetz trägt das Griffbrett oben und hat weniger Stationen; die Kopfplatte ist eine 15-mm-Platte, über die Mechanikreihen verbreitert; Bünde, Saiten, Punkte, Sattel, Pins und Gurtpin entfallen im Body; der Steg ist Textur; Gurtplatten wie bei 1:50."),
         T("Thermal keeps the six strings as slender solids and the fretboard as its 6 mm layer; structure merges the latch levers into the bottom channel and thickens hinge and pad joints to the feature.", "Thermik behält die sechs Saiten als schlanke Körper und das Griffbrett als seine 6-mm-Schicht; die Struktur fasst die Verschlusshebel in den unteren Kasten und verdickt Scharnier- und Polstergelenke auf die Einzelheit.")],
    200: [T("Own model for a 30 mm feature: the two shells, corner caps without radius, a 30 mm grip block, the lining without the lid ceiling detail, the guitar as one polyhedron of soundbox, neck with fretboard and the pitched headstock, the top and the pickguard as sheets, the strap as sheets; the capo, tuners, bridge and all hardware but the corner caps are left out.", "Eigenes Modell für 30 mm kleinste Einzelheit: die zwei Schalen, Eckkappen ohne Radius, ein 30-mm-Griffblock, das Futter, die Gitarre als ein Polyeder aus Klangkörper, Hals mit Griffbrett und geneigter Kopfplatte, Decke und Schlagbrett als Platten, der Gurt als Platten; Kapodaster, Mechaniken, Steg und alle Beschläge außer den Eckkappen entfallen."),
         T("Structure: the neck and headstock are one slender member that spans the head bone, the capo bone is spanned by the compartment block, the latch bones by the bottom channel.", "Struktur: Hals und Kopfplatte sind ein schlanker Stab, der den Kopfknochen mitnimmt, der Kapodasterknochen wird vom Fachblock und die Verschlussknochen vom unteren Kasten mitgenommen.")],
}


GUITAR_BONES = {"lift", "guitar", "neck", "head", "strap-tab", "strap-rear", "strap-cross", "strap-front"}
PLAN_GROUP = {"valance-bottom": 0, "lining-walls": 2, "lining-pad": 3, "compartment": 3, "neck-rest": 3, "capo-bar": 3, "capo-lever": 3, "capo-block": 3, "valance-lid": 5, "shell-lid": 6}


def rest_box(p, frames=None):
    """📦 World box of a part's 2D prism at rest (or in the given frames)."""
    r, o = (frames or REST)[p.bone]
    pts = []
    for a, b in p.prof:
        for c in p.span:
            loc = [a, c, b] if p.sweep == "v" else ([a, b, c] if p.sweep == "w" else [c, a, b])
            pts.append(add(o, mv(r, loc)))
    return [min(q[i] for q in pts) for i in range(3)], [max(q[i] for q in pts) for i in range(3)]


def view_orders(P):
    """🎨 Authored paint order per view at rest, farthest first: the case shells hide what lies inside them, the guitar keeps its own stacking (soundbox, top, pickguard, bridge, fretboard, frets, strings, strap)."""
    items = [p for p in P if p.d2]
    boxes = {p.id: rest_box(p) for p in items}
    gb = [boxes[p.id] for p in items if p.bone in GUITAR_BONES]
    gmin = [min(b[0][i] for b in gb) for i in range(3)]
    gmax = [max(b[1][i] for b in gb) for i in range(3)]
    stack = {p.id: k for k, p in enumerate(q for q in items if q.bone in GUITAR_BONES)}
    played = solve_rig(BONES, PLAY_OFF)
    pbox = {p.id: rest_box(p, played) for p in items if p.bone in GUITAR_BONES}

    def key(view, p):
        lo, hi = boxes[p.id]
        if p.bone in GUITAR_BONES:
            base = {"elevation": -gmin[2], "front": gmax[0], "plan": 4.0}[view]
            inner = {"elevation": -pbox[p.id][0][2], "front": pbox[p.id][1][0], "plan": stack[p.id]}[view]
            return (base, inner, stack[p.id])
        if view == "elevation":
            return (-lo[2], 0, 0)
        if view == "front":
            return (hi[0], 0, 0)
        if p.bone == "lid":
            g = PLAN_GROUP.get(p.id, 7)
        elif p.bone == "capo":
            g = 3
        else:
            g = PLAN_GROUP.get(p.id, 1)
        return (g, hi[1], 0)

    out = {}
    for view in ("elevation", "front", "plan"):
        ordered = sorted(items, key=lambda p: key(view, p))
        out[view] = [p.shape(1 + k * 0.1) for k, p in enumerate(ordered)]
    return out


def build_scale(s):
    P = parts(s)
    body = [p.solid3() for p in P if p.solid3() is not None]
    rep = {"status": "authored", "denominator": s, "validRange": {"minDenominator": RANGES[s][0], "maxDenominator": RANGES[s][1]}, "pens": PENS[s], "hatches": [], "geometry": view_orders(P), "decisions": DEC2D[s]}
    vol = {"status": "authored", "denominator": s, "body": body, "thermal": thermal_list(s, P), "structure": structure_list(s), "decisions": DEC3D[s]}
    return rep, vol


REFS = [
    {"id": "hornbostel-sachs", "citation": "von Hornbostel, E. M.; Sachs, C. (1914). Systematik der Musikinstrumente. Ein Versuch. Zeitschrift für Ethnologie 46, 553-590: class 321.322 necked box lute (guitar family), sounded with the fingers or a plectrum (suffixes -5 and -6)."},
    {"id": "dreadnought-spec", "citation": "Fourteen-fret dreadnought steel-string pattern (introduced 1934), published maker specification of the D-18/D-28 models: body length 20 in (508 mm), lower bout 15 5/8 in (397 mm), upper bout 11 1/2 in (292 mm), depth 4 7/8 in (124 mm) at the tail and about 3 7/8 in (98 mm) at the heel, scale 25.4 in (645 mm), nut width 1 11/16 in (43 mm), headstock pitch about 14 degrees, mass about 2 kg; waist about 274 mm read from the outline."},
    {"id": "cumpiano-natelson", "citation": "Cumpiano, W. R.; Natelson, J. D. (1993). Guitarmaking: Tradition and Technology. Chronicle Books, San Francisco: steel-string plates (top 2.8-3.2 mm, back and sides 2.3-2.7 mm), neck and heel geometry, fret placement, bridge and saddle heights."},
    {"id": "gruhn-carter", "citation": "Gruhn, G.; Carter, W. (2010). Gruhn's Guide to Vintage Guitars, 3rd ed. Backbeat Books: dreadnought dimensions, rectangular plywood hardshell cases of the 1950s to 1970s, typical wear of used instruments (pick wear, arm wear, buckle rash)."},
    {"id": "string-tension", "citation": "D'Addario & Co. String tension specifications: phosphor-bronze light set .012-.053 at standard pitch and 25.5 in scale, total about 163 lbf (0.72 kN); gauges 0.30 to 1.35 mm."},
    {"id": "fpl-wood-handbook", "citation": "Forest Products Laboratory (2010). Wood Handbook - Wood as an Engineering Material, FPL-GTR-190: Sitka spruce 425 kg/m3 and E_L 10.8 GPa, mahogany about 550 kg/m3 and 10.3 GPa, rosewood 800-900 kg/m3 at 12 % moisture content."},
    {"id": "iso-10456-2007", "citation": "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values: plywood 500-700 kg/m3 and 0.13-0.17 W/(m K), PVC, leather and textile foams."},
    {"id": "incropera-2018", "citation": "Incropera, F. P.; DeWitt, D. P.; Bergman, T. L.; Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley: emissivities of lacquered wood (0.9), tarnished nickel (0.1-0.2), polymers."},
    {"id": "din-33402-2", "citation": "DIN 33402-2:2020 Ergonomics - Human body dimensions - Part 2: Values: standing shoulder height about 1.45 m and grip (knuckle) height about 0.76 m of adult men, used for the strap and the carried case."},
    {"id": "case-catalogue", "citation": "Catalogue-typical rectangular plywood hardshell case for dreadnought guitars (used-instrument dealer listings): outside 1105 x 445 x 183 mm, tolex over 3.5 mm poplar plywood, plush lining with accessory compartment and neck rest, four draw latches, two hinges, about 4.4 kg (authored estimate)."},
    {"id": "playing-forces", "citation": "Authored estimates of playing forces: forearm resting on the lower bout about 10-15 N, a strummed downstroke 2-5 N, a fretting hand holding the neck a few newtons net (squeeze of 10-20 N internal to the hand and the neck)."},
]


def document():
    reps, vols = {}, {}
    for s in (50, 100, 200):
        reps[str(s)], vols[str(s)] = build_scale(s)
    reps["500"] = {"status": "unavailable", "denominator": 500, "cause": "below-printable-feature", "reason": T("At 1:500 the whole case is 2.2 mm long and 0.37 mm high on paper: under the 3 mm below which a component is not drawn.", "Bei 1:500 ist der ganze Koffer auf dem Papier 2,2 mm lang und 0,37 mm hoch: unter den 3 mm, unter denen ein Bauteil nicht gezeichnet wird."), "limitingFeature": T("Overall length of the case, 1.105 m", "Gesamtlänge des Koffers, 1,105 m"), "limitingSizeM": 1.105, "alternatives": [{"kind": "scale", "denominator": 200}]}
    vols["500"] = {"status": "unavailable", "denominator": 500}
    asset = {
        "id": "used-steel-string-acoustic-guitar", "category": "objects",
        "title": T("Used Steel-String Acoustic Guitar", "Gebrauchte Westerngitarre"),
        "summary": T("A used fourteen-fret dreadnought steel-string guitar, 1.04 m long, spruce top on mahogany back and sides, in its rectangular tolex-covered plywood hardshell case of 1105 x 445 x 183 mm with dented corner caps, torn tolex and old stickers; the pickguard is worn through under the pick, the lower bout bare from the forearm, the cut string ends stick out at the tuners and a spring capo lies on the compartment lid. Stored closed on the floor, carried by the handle, the case opened and played standing on its leather strap.", "Eine gebrauchte Dreadnought-Westerngitarre mit vierzehn Bünden frei, 1,04 m lang, Fichtendecke auf Mahagoniboden und -zargen, in ihrem rechteckigen tolexbezogenen Sperrholz-Hartschalenkoffer von 1105 x 445 x 183 mm mit eingedellten Eckkappen, eingerissenem Tolex und alten Aufklebern; das Schlagbrett ist unter dem Plektrum durchgespielt, die untere Zarge vom Unterarm blank, die abgeschnittenen Saitenenden stehen an den Mechaniken ab und ein Federkapodaster liegt auf dem Fachdeckel. Geschlossen auf dem Boden gelagert, am Griff getragen, geöffnet und im Stehen am Ledergurt gespielt."),
        "tags": ["used", "instrument", "music", "second-hand", "strings", "guitar", "acoustic-guitar", "steel-string", "dreadnought", "hardshell-case", "spruce", "mahogany", "rosewood", "plywood", "tolex", "leather", "chordophone"],
        "provenance": {"author": "Semio staffage content (used instruments wave, row 03)", "method": "handcrafted", "created": "2026-11-22", "references": REFS, "notes": T("Side elevation from the case's end (the case lies with its length lateral), front elevation along its handle side, plan from above. The guitar is a dreadnought type of the 1960s to 1980s without maker marks. Masses from plate sections and densities: guitar 2.06 kg (soundbox 1.25, neck 0.43, head with tuners 0.38), case 4.4 kg, strap 0.17 kg, capo 0.06 kg. Nothing is measured.", "Seitenansicht von der Stirnseite des Koffers (der Koffer liegt mit seiner Länge quer), Vorderansicht entlang der Griffseite, Grundriss von oben. Die Gitarre ist ein Dreadnought-Typ der 1960er bis 1980er Jahre ohne Herstellerzeichen. Massen aus Plattenquerschnitten und Dichten: Gitarre 2,06 kg (Korpus 1,25, Hals 0,43, Kopf mit Mechaniken 0,38), Koffer 4,4 kg, Gurt 0,17 kg, Kapodaster 0,06 kg. Nichts ist gemessen.")},
        "dimensions": {"heightM": Y_TOP, "widthM": 0.0, "depthM": 0.0, "massKg": TOTAL_KG},
        "materials": MATERIALS,
        "rig": {"kind": "mechanical", "root": "case", "bones": BONES},
        "anchors": ANCHORS,
        "representations": reps,
        "symbols": [],
        "poses": POSES,
        "clips": CLIPS,
        "thermal": {"environment": {"airTempC": 20, "meanRadiantTempC": 20, "relativeHumidity": 0.45, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 1, "pressurePa": 101325, "skyTempC": 20, "groundTempC": 19}, "operating": [], "passengers": [], "regions": REGIONS, "emitters": [],
                    "assumptions": [T("A passive object in the still indoor air of a dealer's depot: radiation and convection of the tolex, plywood, wood, celluloid, leather and metal surfaces, no internal heat and no electrical rating (0 W, passive).", "Ein passives Objekt in der ruhigen Raumluft eines Händlerlagers: Strahlung und Konvektion der Tolex-, Sperrholz-, Holz-, Zelluloid-, Leder- und Metallflächen, keine innere Wärme und keine elektrische Leistung (0 W, passiv)."),
                                    T("Closed, the case insulates the guitar: its regions are declared inside the closed case and the stored and carried poses expose them to no room air; the area of a thin plate counts both faces, because the soundbox air communicates with the room through the 101.6 mm soundhole.", "Geschlossen isoliert der Koffer die Gitarre: ihre Regionen sind als im geschlossenen Koffer deklariert und die gelagerte und die getragene Pose setzen sie keiner Raumluft aus; die Fläche einer dünnen Platte zählt beide Seiten, weil die Luft im Korpus durch das 101,6-mm-Schallloch mit dem Raum verbunden ist."),
                                    T("Played, the hands, the forearm, the belly and the shoulder warm the neck, the lower bout, the back and the strap by contact at 32-33 degrees C; aged lacquer absorbs more than new (0.55 instead of about 0.45 on spruce).", "Beim Spielen erwärmen Hände, Unterarm, Bauch und Schulter Hals, untere Zarge, Boden und Gurt durch Kontakt mit 32-33 Grad C; gealterter Lack absorbiert mehr als neuer (0,55 statt etwa 0,45 auf Fichte).")]},
        "structure": {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": [], "clinical": False,
                      "assumptions": [T("Each pose names its supports: four case feet on the floor when stored and open, the hand at the handle grip as a pin when carried, the strap at the left shoulder as a pin and the belly against the back when played (the open case still stands on its feet).", "Jede Pose nennt ihre Auflager: vier Kofferfüße auf dem Boden gelagert und offen, die Hand am Griff als Gelenk beim Tragen, der Gurt an der linken Schulter als Gelenk und der Bauch am Boden beim Spielen (der offene Koffer steht weiter auf seinen Füßen)."),
                                      T("The 0.72 kN of six light-gauge strings act as an equal and opposite pair at the saddle and the nut: no reaction changes, but the neck is in compression and the bridge and neck joint carry the moment of the strings above the top.", "Die 0,72 kN von sechs leichten Saiten wirken als gleiches und entgegengesetztes Paar an Stegsattel und Sattel: keine Reaktion ändert sich, aber der Hals ist gedrückt und Steg und Halsansatz tragen das Moment der Saiten über der Decke."),
                                      T("The asset is solved as one rigid body: in the played pose the case on the floor and the guitar on the strap share the reactions by stiffness, so the shoulder carries the guitar weight only approximately.", "Das Asset wird als ein starrer Körper gelöst: in der gespielten Pose teilen sich der Koffer auf dem Boden und die Gitarre am Gurt die Reaktionen nach Steifigkeit, die Schulter trägt das Gitarrengewicht daher nur näherungsweise.")]},
        "volumes": vols,
        "presentation": {"defaultPose": "stored", "defaultClip": "strumming", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    ext = extents(vols["50"]["body"])
    asset["dimensions"]["widthM"] = round(ext[0], 3)
    asset["dimensions"]["depthM"] = round(ext[2], 3)
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}


def extents(body):
    """📦 Rough world extents of the rest body (x, y, z) from the case parts' profiles."""
    xs, zs = [], []
    for sol in body:
        bone = sol["bone"]
        if bone not in ("case", "lid", "handle") and not bone.startswith("latch"):
            continue
        r, o = REST[bone]
        pts = []
        if sol["kind"] == "extrude" and sol["plane"] == "zx":
            pts = [[p[1], (sol["from"] + sol["to"]) / 2, p[0]] for p in sol["profile"]]
        elif sol["kind"] == "extrude" and sol["plane"] == "xy":
            pts = [[p[0], p[1], z] for p in sol["profile"] for z in (sol["from"], sol["to"])]
        elif sol["kind"] == "box":
            c, sz = sol["transform"]["translation"], sol["size"]
            pts = [[c[0] + a * sz[0] / 2, c[1], c[2] + b * sz[2] / 2] for a in (-1, 1) for b in (-1, 1)]
        for p in pts:
            w = add(o, mv(r, p))
            xs.append(w[0])
            zs.append(w[2])
    return (max(xs) - min(xs), Y_TOP, max(zs) - min(zs))


if __name__ == "__main__":
    doc = document()
    with open(os.path.join(HERE, "🔣️.json"), "w") as fh:
        json.dump(doc, fh, ensure_ascii=False, separators=(",", ":"))
    a = doc["asset"]
    print("[DEBUG] dims", a["dimensions"], "handle z", Z_H, "carried", CAR, "strap tip error", round(STRAP_INFO["tipError"], 4))
    print("[DEBUG] strap played", {k: v for k, v in PLAY_OFF.items() if k.startswith("strap")})
