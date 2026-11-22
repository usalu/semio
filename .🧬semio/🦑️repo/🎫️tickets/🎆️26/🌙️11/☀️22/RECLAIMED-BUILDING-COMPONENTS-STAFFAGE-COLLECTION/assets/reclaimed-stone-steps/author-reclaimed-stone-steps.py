"""🪜 Authoritative chisel of the staffage asset `reclaimed-stone-steps`: five reclaimed granite block steps 150 x 350 x 1200 mm, stacked crosswise on squared-timber bearers, one lifted in a scissor stone clamp, all five reset as a flight on a stepped concrete foundation.

Every hand-chosen number is stated here; running it rewrites `🔣️.json` next to it and prints the reference measurements of the model.

@see ../../brief.md
@see ../../reports/reclaimed-stone-steps.md
"""
import json
import math
from pathlib import Path

HERE = Path(__file__).resolve().parent
G = 9.81
SCALES = ("50", "100", "200", "500")
FEATURE = {"50": 0.0075, "100": 0.015, "200": 0.03, "500": 0.075}

RHO_GRANITE = 2650.0
RHO_MORTAR = 1800.0
RHO_TIMBER = 420.0
RHO_CONCRETE = 2400.0
RHO_STEEL = 7850.0
RHO_RUBBER = 1200.0

STEP_H = 0.15
STEP_D = 0.35
STEP_L = 1.20
HALF_L = STEP_L / 2
WORN_HALF = 0.35
CHIP_TO = -0.48
CRUST_T = 0.02
CRUST_S = (0.12, 0.33)
CRUST_V = (0.02, 0.12)
PLUG_S = 0.10
PLUG_L = 0.48
PLUG_R = 0.02

BEARER = (1.30, 0.10, 0.10)
BEARER_X = 0.60
BEARER_Z = (-0.40, 0.40)
BATTEN = (0.08, 0.08, 1.20)
BATTEN_X = (0.28, 1.08)
Y_BOTTOM = 0.10
Y_BATTEN = Y_BOTTOM + STEP_H
Y_TOP = Y_BATTEN + BATTEN[1]
BOTTOM_X = {"step-1": 0.0, "step-2": 0.40, "step-3": 0.80}
TOP_XC = 0.575
TOP_W = {"step-4": (-0.25, -0.60), "step-5": (0.25, 0.60)}
TURN = {"step-4": 90.0, "step-5": -90.0}

X_F = 1.55
GOING = 0.30
RISE = STEP_H
FLIGHT = ["step-1", "step-2", "step-3", "step-4", "step-5"]
FOUND_W = 0.60
LIFT_DY = 0.60
CLAMP_HANG = 0.35
BATTEN_DROP = Y_BATTEN - Y_BOTTOM

Q_STAIR = 3.0
Q_POINT = 2000.0
PITCH = math.atan2(RISE, GOING)


def t(en, de):
    """🗣️ A bilingual text, English first."""
    return {"en": en, "de": de}


def r(v, d=5):
    """🎯 A number rounded for the document."""
    out = round(v + 0.0, d)
    return 0.0 if out == 0 else out


def rp(pts):
    """🧮 A rounded point list."""
    return [[r(a), r(b)] for a, b in pts]


def de_num(v, fmt="{:.2f}"):
    """🔢 A number in German notation."""
    return fmt.format(v).replace(".", ",")


def arc(cx, cy, rad, a0, a1, n):
    """🌙 Points of an arc, angles in degrees, both ends included."""
    return [(cx + rad * math.cos(math.radians(a0 + (a1 - a0) * k / n)), cy + rad * math.sin(math.radians(a0 + (a1 - a0) * k / n))) for k in range(n + 1)]


def circle(cx, cy, rad, n):
    """⭕ A circle as an n-gon."""
    return [(cx + rad * math.cos(2 * math.pi * k / n), cy + rad * math.sin(2 * math.pi * k / n)) for k in range(n)]


def stadium(cv, half_len, half_wid, n=6):
    """🏟️ A stadium in (v, w) with its long axis along v, centred at (cv, 0)."""
    c1, c0 = cv + half_len - half_wid, cv - half_len + half_wid
    top = [(c1 + half_wid * math.cos(-math.pi / 2 + math.pi * k / n), half_wid * math.sin(-math.pi / 2 + math.pi * k / n)) for k in range(n + 1)]
    bottom = [(c0 + half_wid * math.cos(math.pi / 2 + math.pi * k / n), half_wid * math.sin(math.pi / 2 + math.pi * k / n)) for k in range(n + 1)]
    return top + bottom


def strip(p0, p1, width):
    """📏 A straight bar of `width` between two profile points."""
    dv, dw = p1[0] - p0[0], p1[1] - p0[1]
    n = math.hypot(dv, dw)
    ov, ow = -dw / n * width / 2, dv / n * width / 2
    return [(p0[0] + ov, p0[1] + ow), (p1[0] + ov, p1[1] + ow), (p1[0] - ov, p1[1] - ow), (p0[0] - ov, p0[1] - ow)]


def signed_area(pts):
    """➰ Signed shoelace area (counter-clockwise positive)."""
    return sum(pts[i][0] * pts[(i + 1) % len(pts)][1] - pts[(i + 1) % len(pts)][0] * pts[i][1] for i in range(len(pts))) / 2


def ccw(pts):
    """🔄 The polygon counter-clockwise."""
    return pts if signed_area(pts) > 0 else list(reversed(pts))


def poly_area(pts):
    """📐 The area of a simple polygon."""
    return abs(signed_area(pts))


def poly_centroid(pts):
    """🎯 The centroid of a simple polygon."""
    a = signed_area(pts)
    cx = sum((pts[i][0] + pts[(i + 1) % len(pts)][0]) * (pts[i][0] * pts[(i + 1) % len(pts)][1] - pts[(i + 1) % len(pts)][0] * pts[i][1]) for i in range(len(pts)))
    cy = sum((pts[i][1] + pts[(i + 1) % len(pts)][1]) * (pts[i][0] * pts[(i + 1) % len(pts)][1] - pts[(i + 1) % len(pts)][0] * pts[i][1]) for i in range(len(pts)))
    return cx / (6 * a), cy / (6 * a)


ORIGIN = {}
BONES = []


def bone(id_, parent, x, y, z, title, length, joint, world):
    """🦴 Adds a bone (every rest placement is axis-parallel) and records its rest world origin."""
    e = {"id": id_, "parent": parent, "title": title, "x": r(x), "y": r(y)}
    if z:
        e["z"] = r(z)
    e.update({"angleDeg": 0, "lengthM": r(length), "joint": joint})
    BONES.append(e)
    ORIGIN[id_] = world


def prismatic(dx, dy):
    """➡️ A prismatic joint whose axis is the travel (dx, dy), from rest to the full travel."""
    return {"type": "prismatic", "axisDeg": math.degrees(math.atan2(dy, dx)), "minM": 0, "maxM": math.hypot(dx, dy)}


STEP_TARGET = {}


def build_rig():
    """🦴 Bearers as root; battens that drop onto the bearers; three bottom steps that slide into the flight; two top steps on carriages that turn into the flight; the lift, the clamp, the foundation and the walking-load carrier."""
    bone("bearers", None, 0, 0, 0, t("Squared-timber bearers 10/10 (stay on the ground)", "Kantholz-Unterleger 10/10 (bleiben am Boden)"), 1.25, {"type": "fixed"}, (0, 0, 0))
    bone("battens", "bearers", 0, Y_BATTEN, 0, t("Intermediate timbers 8/8 between the layers (drop onto the bearers when the stack is cleared)", "Zwischenhölzer 8/8 zwischen den Lagen (fallen beim Abräumen auf die Unterleger)"), 1.20, {"type": "prismatic", "axisDeg": 90, "minM": r(-BATTEN_DROP), "maxM": 0}, (0, Y_BATTEN, 0))
    for k, sid in enumerate(FLIGHT[:3]):
        x0 = BOTTOM_X[sid]
        d = (X_F + GOING * k - x0, RISE * k - Y_BOTTOM)
        STEP_TARGET[sid] = d
        bone(sid, "bearers", x0, Y_BOTTOM, 0, t(f"Block step {k + 1}, bottom layer, lying across the bearers", f"Blockstufe {k + 1}, untere Lage, quer auf den Unterlegern"), STEP_D, prismatic(*d), (x0, Y_BOTTOM, 0))
    for k, sid in ((3, "step-4"), (4, "step-5")):
        wn = TOP_W[sid][0]
        xo = X_F + GOING * k - abs(wn)
        d = (xo - TOP_XC, RISE * k - Y_TOP)
        STEP_TARGET[sid] = d
        n = k + 1
        parent = "bearers"
        if sid == "step-5":
            bone("step-5-lift", "bearers", TOP_XC, Y_TOP, 0, t("Hoist of the clamp (massless, vertical)", "Hub der Zange (masselos, senkrecht)"), 0, {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 1.0}, (TOP_XC, Y_TOP, 0))
            parent = "step-5-lift"
            bone(f"{sid}-carriage", parent, 0, 0, 0, t(f"Carriage of block step {n} to its place in the flight (massless)", f"Schlitten von Blockstufe {n} an ihren Platz in der Treppe (masselos)"), 0, prismatic(*d), (TOP_XC, Y_TOP, 0))
        else:
            bone(f"{sid}-carriage", parent, TOP_XC, Y_TOP, 0, t(f"Carriage of block step {n} to its place in the flight (massless)", f"Schlitten von Blockstufe {n} an ihren Platz in der Treppe (masselos)"), 0, prismatic(*d), (TOP_XC, Y_TOP, 0))
        bone(sid, f"{sid}-carriage", 0, 0, 0, t(f"Block step {n}, top layer, lying along the stack (turns a quarter turn into the flight)", f"Blockstufe {n}, obere Lage, längs auf dem Stapel (dreht sich um eine Vierteldrehung in die Treppe)"), STEP_L / 2, {"type": "revolute", "minDeg": 0, "maxDeg": 0}, (TOP_XC, Y_TOP, 0))
    clamp_d = (X_F + GOING * 4 + STEP_D / 2 - TOP_XC, RISE * 5 + CLAMP_HANG - (Y_TOP + STEP_H))
    STEP_TARGET["clamp"] = clamp_d
    bone("clamp", "step-5-lift", 0, STEP_H, TOP_W["step-5"][0] + (TOP_W["step-5"][1] - TOP_W["step-5"][0]) / 2, t("Scissor stone clamp with master link (hangs from the crane hook)", "Scheren-Steinzange mit Aufhängering (hängt am Kranhaken)"), 0.62, prismatic(*clamp_d), (TOP_XC, Y_TOP + STEP_H, TOP_W["step-5"][0] + (TOP_W["step-5"][1] - TOP_W["step-5"][0]) / 2))
    bone("foundation", "bearers", X_F, 0, 0, t("Stepped concrete foundation of the new flight", "Gestuftes Betonfundament der neuen Treppe"), GOING * 4 + STEP_D, {"type": "fixed"}, (X_F, 0, 0))
    bone("load-carrier", "foundation", 0, 0, 0, t("Walking-load carrier (massless, climbs the pitch line)", "Lastträger der Begehung (masselos, steigt die Lauflinie hinauf)"), 0, {"type": "prismatic", "axisDeg": math.degrees(PITCH), "minM": 0, "maxM": r(math.hypot(GOING * 4, RISE * 4), 6)}, (X_F, 0, 0))


class Solid:
    """🧊 A solid of one scale in the bone-local frame: a box or a prism along a bone axis (u: profile (v, w); w: profile (u, v); v: profile (w, u))."""

    def __init__(self, id_, bone_, kind, material, tone, **kw):
        self.id, self.bone, self.kind, self.material, self.tone, self.kw = id_, bone_, kind, material, tone, kw

    def doc(self):
        """📄 The JSON of the solid."""
        d = {"id": self.id, "kind": "extrude" if self.kind == "prism" else "box", "bone": self.bone, "material": self.material, "role": "body", "tone": self.tone}
        for flag in ("slender", "sheet", "envelope", "texture"):
            if self.kw.get(flag):
                d[flag] = True
        if self.kind == "box":
            d["size"] = [r(v) for v in self.kw["size"]]
            d["transform"] = {"translation": [r(v) for v in self.kw["at"]]}
        else:
            d["profile"] = rp(ccw(self.kw["profile"]))
            if self.kw.get("holes"):
                d["holes"] = [rp(ccw(h)) for h in self.kw["holes"]]
            d["plane"] = {"u": "yz", "w": "xy", "v": "zx"}[self.kw["axis"]]
            d["from"], d["to"] = r(self.kw["range"][0]), r(self.kw["range"][1])
        return d

    def volume(self):
        """⚖️ Volume in m³ (holes subtracted)."""
        if self.kind == "box":
            a, b, c = self.kw["size"]
            return a * b * c
        area = poly_area(self.kw["profile"]) - sum(poly_area(h) for h in self.kw.get("holes", []))
        return area * (self.kw["range"][1] - self.kw["range"][0])

    def centre(self):
        """🎯 The local centre (u, v, w) of the solid (holes neglected)."""
        if self.kind == "box":
            return tuple(self.kw["at"])
        a, b = poly_centroid(self.kw["profile"])
        c = (self.kw["range"][0] + self.kw["range"][1]) / 2
        return {"u": (c, a, b), "w": (a, b, c), "v": (b, c, a)}[self.kw["axis"]]

    def local_box(self):
        """📦 Local bounds ((u0, u1), (v0, v1), (w0, w1))."""
        if self.kind == "box":
            return tuple((c - s / 2, c + s / 2) for c, s in zip(self.kw["at"], self.kw["size"]))
        p = self.kw["profile"]
        a = (min(q[0] for q in p), max(q[0] for q in p))
        b = (min(q[1] for q in p), max(q[1] for q in p))
        c = tuple(self.kw["range"])
        return {"u": (c, a, b), "w": (a, b, c), "v": (b, c, a)}[self.kw["axis"]]


REGION_OF = {"granite": "step-granite", "old-mortar": "mortar-remnants", "softwood": "timber", "foundation-concrete": "foundation", "clamp-steel": "lifting-gear", "grip-rubber": "lifting-gear"}


def section(scale, variant):
    """🪨 The section (s, v) of a block step: s from the nosing (0) to the back (0.35), v from the bed (0) to the tread (0.15)."""
    d, h = STEP_D, STEP_H
    if scale == "50":
        if variant == "worn":
            return worn_section()
        if variant == "chip":
            return [(0, 0), (d, 0), (d, h), (0.04, h), (0, h - 0.03)]
        return [(0, 0), (d, 0), (d, h), *arc(0.012, h - 0.012, 0.012, 90, 180, 4)]
    if scale == "100":
        if variant == "worn":
            return [(0, 0), (d, 0), (d, h), (0.03, h), (0, h - 0.03)]
        if variant == "chip":
            return [(0, 0), (d, 0), (d, h), (0.04, h), (0, h - 0.03)]
        return [(0, 0), (d, 0), (d, h), (0.015, h), (0, h - 0.015)]
    if scale == "200":
        return [(0, 0), (d, 0), (d, h), (0.03, h), (0, h - 0.03)]
    return [(0, 0), (d, 0), (d, h), (0, h)]


def worn_section():
    """🦶 The 1:50 section of the walked middle of a tread: dished 8 mm, nosing worn round to R 35."""
    h = STEP_H
    top = [(0.26, h), (0.20, h - 0.0035), (0.14, h - 0.0075), (0.08, h - 0.008), (0.035, h - 0.008)]
    nose = arc(0.035, h - 0.043, 0.035, 90, 180, 5)[1:]
    return [(0, 0), (STEP_D, 0), (STEP_D, h)] + top + nose


def step_segments(sid, scale):
    """✂️ The length segments of a step at a scale: (id suffix, from, to, variant)."""
    if scale in ("200", "500"):
        return [("", -HALF_L, HALF_L, "end")]
    first = [("a", -HALF_L, CHIP_TO, "chip"), ("b", CHIP_TO, -WORN_HALF, "end")] if sid == "step-3" else [("a", -HALF_L, -WORN_HALF, "end")]
    return first + [("m", -WORN_HALF, WORN_HALF, "worn"), ("c", WORN_HALF, HALF_L, "end")]


def to_local(sid, s, v):
    """🔀 A section point (s, v) of step `sid` in its profile plane: (u, v) for a bottom step, (v, w) for a top step."""
    if sid in BOTTOM_X:
        return (s, v)
    wn, wb = TOP_W[sid]
    return (v, wn + math.copysign(1, wb - wn) * s)


def step_solids(sid, scale):
    """🪨 The granite segments of one step and the mortar crust on the end that was built into the house wall (the end at -z in the flight; the free end at +z carries the railing post hole)."""
    out = []
    tone = {"step-1": 0.56, "step-2": 0.6, "step-3": 0.54, "step-4": 0.58, "step-5": 0.62}[sid]
    axis = "w" if sid in BOTTOM_X else "u"
    for suf, a, b, variant in step_segments(sid, scale):
        prof = worn_section() if (scale == "50" and variant == "worn") else section(scale, variant)
        out.append(Solid(f"{sid}{'-' + suf if suf else ''}", sid, "prism", "granite", tone if variant != "worn" else tone + 0.04, axis=axis, profile=[to_local(sid, s, v) for s, v in prof], range=(a, b)))
    texture = scale in ("200", "500")
    sign = 1 if sid == "step-5" else -1
    for side in ("wall",):
        along = (sign * HALF_L, sign * (HALF_L + CRUST_T))
        along = (min(along), max(along))
        s0, s1 = CRUST_S
        v0, v1 = CRUST_V
        if sid in BOTTOM_X:
            size = (s1 - s0, v1 - v0, CRUST_T)
            at = ((s0 + s1) / 2, (v0 + v1) / 2, (along[0] + along[1]) / 2)
        else:
            w0, w1 = to_local(sid, s0, 0)[1], to_local(sid, s1, 0)[1]
            size = (CRUST_T, v1 - v0, abs(w1 - w0))
            at = ((along[0] + along[1]) / 2, (v0 + v1) / 2, (w0 + w1) / 2)
        out.append(Solid(f"{sid}-crust-{side}", sid, "box", "old-mortar", 0.84, size=size, at=at, texture=texture))
    return out


def bearer_solids(scale):
    """🪵 The two bearers 100 x 100 x 1300 mm along the stack."""
    return [Solid(f"bearer-{'ab'[i]}", "bearers", "box", "softwood", 0.42 + 0.04 * i, size=BEARER, at=(BEARER_X, BEARER[1] / 2, z)) for i, z in enumerate(BEARER_Z)]


def batten_solids(scale):
    """🪵 The two intermediate timbers 80 x 80 x 1200 mm across the bottom layer."""
    return [Solid(f"batten-{'ab'[i]}", "battens", "box", "softwood", 0.5 + 0.04 * i, size=BATTEN, at=(x, BATTEN[1] / 2, 0)) for i, x in enumerate(BATTEN_X)]


def foundation_profile():
    """🧱 The stepped section (u, v) of the foundation: a tread of 300 mm at every rise of 150 mm under steps 2 to 5."""
    pts = [(STEP_D, 0), (GOING * 4 + STEP_D, 0)]
    for k in range(4, 0, -1):
        x1 = GOING * k + STEP_D
        x0 = GOING * k + 0.05
        pts += [(x1, RISE * k), (x0, RISE * k)]
    return [p for i, p in enumerate(pts) if i == 0 or p != pts[i - 1]]


def foundation_solids(scale):
    """🧱 The stepped concrete foundation, 1.20 m wide."""
    return [Solid("foundation", "foundation", "prism", "foundation-concrete", 0.78, axis="w", profile=foundation_profile(), range=(-FOUND_W, FOUND_W))]


JAW_W = STEP_D / 2
EYE_V = 0.56
EYE_R = 0.0125
RING_BAR = 0.016
RING_HALF = 0.10


def ring_centre(rb):
    """⭕ The centre height of the master link whose hole rests on the eye pin."""
    return EYE_V - EYE_R - rb + RING_HALF


def clamp_geometry(scale):
    """🗜️ The scissor stone clamp in the clamp frame (origin on the tread centre of step 5, u along the step, w across it): jaws and pads on both faces, two crossing arms, pivot pin, links, eye pin and master link."""
    out = []
    S = "clamp-steel"
    pad_t = 0.01 if scale == "50" else 0.0
    jaw_t = {"50": 0.015, "100": 0.025, "200": 0.03, "500": 0.075}[scale]
    jaw_in = JAW_W + pad_t
    jaw_c = jaw_in + jaw_t / 2
    arm_t = {"50": 0.02, "100": 0.02, "200": 0.03}.get(scale)
    arm_wd = {"50": 0.06, "100": 0.06, "200": 0.06}.get(scale)
    top_v, top_w, eye_v = 0.42, 0.12, EYE_V
    if scale == "500":
        prof = [(-0.13, -(JAW_W + 0.075)), (-0.13, -JAW_W), (0.0, -JAW_W), (0.0, JAW_W), (-0.13, JAW_W), (-0.13, JAW_W + 0.075), (0.10, JAW_W + 0.075), (0.30, 0.16), (0.44, 0.13), (0.53, 0.06), (0.73, 0.06), (0.73, -0.06), (0.53, -0.06), (0.44, -0.13), (0.30, -0.16), (0.10, -(JAW_W + 0.075))]
        out.append(Solid("clamp-block", "clamp", "prism", S, 0.3, axis="u", profile=prof, range=(-0.08, 0.08)))
        return out
    for side, sign in (("a", -1), ("b", 1)):
        out.append(Solid(f"jaw-{side}", "clamp", "box", S, 0.26, size=(0.16, 0.12, jaw_t), at=(0, -0.07, sign * jaw_c)))
        if pad_t:
            out.append(Solid(f"pad-{side}", "clamp", "box", "grip-rubber", 0.12, size=(0.14, 0.09, pad_t), at=(0, -0.075, sign * (JAW_W + pad_t / 2))))
    p0a, p1a = (-0.04, -jaw_c), (top_v, top_w)
    p0b, p1b = (-0.04, jaw_c), (top_v, -top_w)
    ua = (-0.01 - arm_t, -0.01)
    ub = (0.01, 0.01 + arm_t)
    out.append(Solid("arm-a", "clamp", "prism", S, 0.3, axis="u", profile=strip(p0a, p1a, arm_wd), range=ua))
    out.append(Solid("arm-b", "clamp", "prism", S, 0.32, axis="u", profile=strip(p0b, p1b, arm_wd), range=ub))
    tt = jaw_c / (jaw_c + top_w)
    pv = p0a[0] + (p1a[0] - p0a[0]) * tt
    rpin = 0.015
    out.append(Solid("pivot-pin", "clamp", "prism", S, 0.22, axis="u", profile=circle(pv, 0, rpin, 16 if scale == "50" else 10), range=(-0.01 - arm_t - 0.01, 0.01 + arm_t + 0.01)))
    lt = arm_t
    out.append(Solid("link-a", "clamp", "prism", S, 0.3, axis="u", profile=strip((top_v - 0.02, top_w + 0.004), (eye_v, 0), 0.05), range=(ua[0] - lt, ua[0])))
    out.append(Solid("link-b", "clamp", "prism", S, 0.32, axis="u", profile=strip((top_v - 0.02, -top_w - 0.004), (eye_v, 0), 0.05), range=(ub[1], ub[1] + lt)))
    out.append(Solid("eye-pin", "clamp", "prism", S, 0.22, axis="u", profile=circle(eye_v, 0, 0.0125 if scale != "200" else 0.015, 16 if scale == "50" else 10), range=(ua[0] - lt - 0.008, ub[1] + lt + 0.008)))
    rb = RING_BAR if scale != "200" else 0.03
    cv = ring_centre(rb)
    ring = stadium(cv, 0.10, 0.06, 8 if scale == "50" else 6)
    if scale == "200":
        out.append(Solid("master-link", "clamp", "prism", S, 0.24, axis="u", profile=ring, range=(-0.015, 0.015)))
    else:
        hole = stadium(cv, 0.10 - rb, 0.06 - rb, 8 if scale == "50" else 6)
        out.append(Solid("master-link", "clamp", "prism", S, 0.24, axis="u", profile=ring, holes=[hole], range=(-RING_BAR / 2, RING_BAR / 2)))
    return out


def body(scale):
    """🧍 The body list of a scale."""
    out = bearer_solids(scale) + batten_solids(scale)
    for sid in FLIGHT:
        out += step_solids(sid, scale)
    return out + foundation_solids(scale) + clamp_geometry(scale)


TURNED = {"step-4", "step-5"}


def to_world(b, p):
    """🌍 A bone-local rest point in world metres (every rest frame is axis-parallel)."""
    o = ORIGIN[b]
    return (o[0] + p[0], o[1] + p[1], o[2] + p[2])


def outline(solid, view):
    """🖼️ The rest outline of a solid in a view and whether it is the exact profile: elevation in bone-local (u, v), front in world (z, y), plan in world (x, z)."""
    (u0, u1), (v0, v1), (w0, w1) = solid.local_box()
    o = ORIGIN[solid.bone]
    axis = solid.kw.get("axis") if solid.kind == "prism" else None
    if view == "elevation":
        if axis == "w":
            return [tuple(p) for p in solid.kw["profile"]], True
        return [(u0, v0), (u1, v0), (u1, v1), (u0, v1)], False
    if view == "front":
        if axis == "u":
            return [(o[2] + b, o[1] + a) for a, b in solid.kw["profile"]], True
        return [(o[2] + w0, o[1] + v0), (o[2] + w1, o[1] + v0), (o[2] + w1, o[1] + v1), (o[2] + w0, o[1] + v1)], False
    if axis == "v":
        return [(o[0] + b, o[2] + a) for a, b in solid.kw["profile"]], True
    return [(o[0] + u0, o[2] + w0), (o[0] + u1, o[2] + w0), (o[0] + u1, o[2] + w1), (o[0] + u0, o[2] + w1)], False


PLAN_LAYER = {"bearers": 0.0, "foundation": 0.05, "step-1": 0.2, "step-2": 0.2, "step-3": 0.2, "battens": 0.3, "step-4": 0.5, "step-5": 0.5, "clamp": 0.7}


def z_of(solid, index, view):
    """🪜 The painting order of a shape, valid in every pose: elevation (from -z) and front (from +x) by the nearest rest face of the solid, plan by layer (bearers, foundation, bottom steps, timbers, top steps, clamp), because the steps set into the flight lie on the foundation and lap over each other in the order of their index."""
    (u0, u1), (v0, v1), (w0, w1) = solid.local_box()
    o = ORIGIN[solid.bone]
    key = {"elevation": -(o[2] + w0), "front": o[0] + u1, "plan": PLAN_LAYER[solid.bone]}[view]
    return r(2 + key + 0.0001 * index, 4)


def path_d(rings):
    """✒️ SVG path of closed rings."""
    return " ".join("M " + " L ".join(f"{r(a)} {r(b)}" for a, b in ring) + " Z" for ring in rings)


def shapes_of(solid, view, scale, index):
    """✏️ The filled closed shape of one body solid in one view; the turned top steps carry an extruded `solid` in elevation and front so their drawing turns with them."""
    if scale in ("200", "500") and solid.material == "old-mortar":
        return []
    base = {"id": solid.id, "bone": solid.bone, "role": "body", "z": z_of(solid, index, view), "filled": True, "tone": solid.tone, "region": REGION_OF[solid.material], "material": solid.material}
    if scale in ("200", "500") and solid.material == "granite":
        del base["region"]
        base["regions"] = ["step-granite", "mortar-remnants"]
    if solid.material == "granite" and HATCHES[scale]:
        base["hatch"] = "split-granite"
    if solid.bone in TURNED and view in ("elevation", "front"):
        (u0, u1), (v0, v1), (w0, w1) = solid.local_box()
        oz = ORIGIN[solid.bone][2]
        return [{**base, "kind": "poly", "points": rp([(u0, v0), (u1, v0), (u1, v1), (u0, v1)]), "closed": True, "solid": {"kind": "extruded", "sweep": "w", "z0": r(oz + w0), "z1": r(oz + w1)}}]
    ring, exact = outline(solid, view)
    if exact and solid.kw.get("holes"):
        holes = []
        for h in solid.kw["holes"]:
            fake = Solid("h", solid.bone, "prism", solid.material, 0, axis=solid.kw["axis"], profile=h, range=solid.kw["range"])
            holes.append(outline(fake, view)[0])
        return [{**base, "kind": "path", "d": path_d([ring] + holes)}]
    return [{**base, "kind": "poly", "points": rp(ring), "closed": True}]


def stroke(id_, b, pts, role, region, tone=0.3, z=4.0):
    """〰️ An open detail or texture stroke."""
    return {"id": id_, "bone": b, "role": role, "z": z, "filled": False, "tone": tone, "region": region, "kind": "poly", "points": rp(pts), "closed": False}


def plan_point(sid, s, along):
    """📍 World plan point (x, z) of a step at section depth s and length position `along`."""
    o = ORIGIN[sid]
    if sid in BOTTOM_X:
        return (o[0] + s, o[2] + along)
    return (o[0] + along, o[2] + to_local(sid, s, 0)[1])


def plan_poly_step(sid, pts):
    """🗺️ A list of (s, along) points of a step in world plan coordinates."""
    return [plan_point(sid, s, a) for s, a in pts]


def plan_z(b, extra=0.0):
    """🪜 The paint order of a plan stroke: just above the layer of its part."""
    return r(2 + PLAN_LAYER[b] + 0.05 + extra, 4)


def detail_lines(scale, view):
    """〰️ Details of the views: in plan the edge of the worn tread, the worn nosing, the lead-filled railing post holes and the bush-hammered band behind the nosing; in elevation the bed joint marks of the end faces and the end checks of the bearers."""
    out = []
    if view == "plan" and scale in ("50", "100"):
        for sid in FLIGHT:
            covered = sid in ("step-1", "step-3")
            out.append(stroke(f"{sid}-nosing", sid, plan_poly_step(sid, [(0.035 if scale == "50" else 0.03, -WORN_HALF), (0.035 if scale == "50" else 0.03, WORN_HALF)]), "detail", "step-granite", 0.3, plan_z(sid, 0.001)))
            dish = [(0.04, -0.30), (0.24, -0.25), (0.25, 0.0), (0.24, 0.25), (0.04, 0.30)]
            if scale == "50" and not covered:
                out.append(stroke(f"{sid}-wear", sid, plan_poly_step(sid, dish), "texture", "step-granite", 0.35, plan_z(sid)))
            if scale == "50":
                for j, a in enumerate((-0.55, -0.45, 0.45, 0.55)):
                    out.append(stroke(f"{sid}-tool-{j}", sid, plan_poly_step(sid, [(0.03, a), (0.33, a)]), "texture", "step-granite", 0.35, plan_z(sid)))
            plug_along = PLUG_L if sid in BOTTOM_X else (PLUG_L if sid == "step-4" else -PLUG_L)
            cx, cz = plan_point(sid, PLUG_S, plug_along)
            if not covered or sid == "step-3":
                out.append({"id": f"{sid}-post-hole", "bone": sid, "role": "detail", "z": plan_z(sid, 0.002), "filled": True, "tone": 0.15, "region": "step-granite", "kind": "poly", "points": rp(circle(cx, cz, PLUG_R, 16 if scale == "50" else 8)), "closed": True})
    if view == "plan" and scale != "500":
        for k in range(1, 5):
            x = X_F + GOING * k + 0.05
            out.append(stroke(f"foundation-rise-{k}", "foundation", [(x, -FOUND_W), (x, FOUND_W)], "detail", "foundation", 0.3, plan_z("foundation")))
    if view == "elevation" and scale == "50":
        for sid in ("step-1",):
            out.append(stroke(f"{sid}-bed-mark", sid, [(0.06, 0.03), (0.30, 0.03)], "texture", "step-granite", 0.35, 4.0))
        out.append(stroke("bearer-check", "bearers", [(-0.03, 0.065), (0.10, 0.06), (0.30, 0.066)], "texture", "timber", 0.3, 4.0))
        out.append(stroke("foundation-joint", "foundation", [(STEP_D + 0.01, 0.075), (GOING * 4 + STEP_D - 0.01, 0.075)], "texture", "foundation", 0.35, 4.0))
    return out


HATCHES = {
    "50": [{"id": "split-granite", "title": t("Weathered split and bush-hammered granite", "Verwitterter gespaltener und gestockter Granit"), "pattern": "stipple", "angleDeg": 0, "spacingMm": 0.8, "penMm": 0.1}],
    "100": [{"id": "split-granite", "title": t("Weathered split and bush-hammered granite", "Verwitterter gespaltener und gestockter Granit"), "pattern": "stipple", "angleDeg": 0, "spacingMm": 0.6, "penMm": 0.1}],
    "200": [],
    "500": [],
}
PENS = {
    "50": {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": None, "ground": 0.5},
    "100": {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": None, "ground": 0.35},
    "200": {"body": 0.18, "detail": 0.1, "texture": None, "glazing": None, "ground": 0.25},
    "500": {"body": 0.13, "detail": None, "texture": None, "glazing": None, "ground": 0.18},
}
RANGES = {"50": (20, 75), "100": (75, 150), "200": (150, 350), "500": (350, 1000)}


def geometry(scale):
    """🗺️ The three views of a scale, drawn from the same numbers as its solids."""
    out = {}
    solids = body(scale)
    for view in ("elevation", "front", "plan"):
        shapes = []
        for i, s in enumerate(solids):
            shapes += shapes_of(s, view, scale, i)
        out[view] = shapes + detail_lines(scale, view)
    return out


def mass_by_bone():
    """⚖️ Mass and local centre of every bone from the 1:50 solids (volume x density)."""
    rho = {m["id"]: m["densityKgM3"] for m in MATERIALS}
    acc = {}
    for s in body("50"):
        m = s.volume() * rho[s.material]
        c = s.centre()
        a = acc.setdefault(s.bone, [0.0, 0.0, 0.0, 0.0])
        a[0] += m
        a[1] += m * c[0]
        a[2] += m * c[1]
        a[3] += m * c[2]
    return {b: (m, mu / m, mv / m, mw / m) for b, (m, mu, mv, mw) in acc.items()}


MATERIALS = [
    {"id": "granite", "title": t("Weathered grey granite, split bed faces, bush-hammered risers, worn treads", "Verwitterter grauer Granit, gespaltene Lagerflächen, gestockte Ansichtsflächen, abgetretene Trittflächen"), "emissivity": 0.9, "solarAbsorptivity": 0.55, "densityKgM3": RHO_GRANITE, "conductivityWmK": 2.8, "specificHeatJKgK": 1000, "elasticModulusPa": 50e9, "strengthPa": 12e6, "source": "literature", "reference": "en-iso-10456"},
    {"id": "old-mortar", "title": t("Old lime-cement mortar remnants of the wall seating", "Alte Kalkzementmörtelreste des Wandauflagers"), "emissivity": 0.93, "solarAbsorptivity": 0.6, "densityKgM3": RHO_MORTAR, "conductivityWmK": 1.0, "specificHeatJKgK": 1000, "source": "literature", "reference": "en-iso-10456"},
    {"id": "softwood", "title": t("Weathered softwood C24", "Verwittertes Nadelholz C24"), "emissivity": 0.9, "solarAbsorptivity": 0.7, "densityKgM3": RHO_TIMBER, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11e9, "strengthPa": 24e6, "source": "literature", "reference": "en-338"},
    {"id": "foundation-concrete", "title": t("New plain concrete C20/25 of the foundation, light grey", "Neuer unbewehrter Beton C20/25 des Fundaments, hellgrau"), "emissivity": 0.93, "solarAbsorptivity": 0.6, "densityKgM3": RHO_CONCRETE, "conductivityWmK": 2.0, "specificHeatJKgK": 1000, "elasticModulusPa": 30e9, "strengthPa": 20e6, "source": "literature", "reference": "en-1992-1-1"},
    {"id": "clamp-steel", "title": t("Painted steel of the stone clamp", "Lackierter Stahl der Steinzange"), "emissivity": 0.85, "solarAbsorptivity": 0.55, "densityKgM3": RHO_STEEL, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210e9, "strengthPa": 355e6, "source": "literature", "reference": "en-10025-2"},
    {"id": "grip-rubber", "title": t("Rubber grip pads of the clamp jaws", "Gummigreifbeläge der Zangenbacken"), "emissivity": 0.92, "solarAbsorptivity": 0.9, "densityKgM3": RHO_RUBBER, "conductivityWmK": 0.17, "specificHeatJKgK": 1400, "source": "literature", "reference": "en-iso-10456"},
]

SOURCES = {"bearers": ("literature", "en-338"), "battens": ("literature", "en-338"), "foundation": ("literature", "en-1991-1-1"), "clamp": ("authored-estimate", "en-13155")}
STEP_SECTION = {"areaM2": r(STEP_D * STEP_H), "secondMomentM4": r(STEP_D * STEP_H ** 3 / 12, 9), "extremeFibreM": STEP_H / 2, "material": "granite", "title": t("Granite block step 150 x 350 mm", "Granit-Blockstufe 150 x 350 mm")}
SECTIONS = {
    "bearers": {"areaM2": 0.01, "secondMomentM4": 8.333e-06, "extremeFibreM": 0.05, "material": "softwood", "title": t("Squared timber 100 x 100 mm, C24", "Kantholz 100 x 100 mm, C24")},
    "battens": {"areaM2": 0.0064, "secondMomentM4": 3.413e-06, "extremeFibreM": 0.04, "material": "softwood", "title": t("Squared timber 80 x 80 mm, C24", "Kantholz 80 x 80 mm, C24")},
    "foundation": {"areaM2": 0.36, "secondMomentM4": 0.0108, "extremeFibreM": 0.3, "material": "foundation-concrete", "title": t("Stepped foundation, 1.20 m wide, C20/25", "Gestuftes Fundament, 1,20 m breit, C20/25")},
    "clamp": {"areaM2": 0.0012, "secondMomentM4": 3.6e-07, "extremeFibreM": 0.03, "material": "clamp-steel", "title": t("Scissor arm, flat bar 60 x 20 mm", "Scherenarm, Flachstahl 60 x 20 mm")},
}


def finish_rig():
    """🧷 Writes masses and sections onto the bones."""
    masses = mass_by_bone()
    for b in BONES:
        if b["id"] in masses:
            m, cu, cv, _ = masses[b["id"]]
            src = SOURCES.get(b["id"], ("literature", "en-1936"))
            b["mass"] = {"kg": r(m, 3), "comX": r(cu, 4), "comY": r(cv, 4), "distribution": "point", "source": src[0], "reference": src[1]}
            b["section"] = STEP_SECTION if b["id"].startswith("step-") else SECTIONS[b["id"]]
    return masses


HOOK_V = ring_centre(RING_BAR) + RING_HALF - RING_BAR - 0.005


def anchors():
    """📍 Ground contacts of the bearers and the foundation, the crane hook in the master link and the walking-load point."""
    found_l = GOING * 4 + STEP_D
    return [
        {"id": "bearer-a-ground", "bone": "bearers", "x": BEARER_X, "y": 0, "z": BEARER_Z[0], "kind": "contact", "title": t("Right bearer on the yard gravel", "Rechter Unterleger auf dem Hofschotter"), "support": "contact", "normalDeg": 90, "footprintLengthM": BEARER[0], "footprintWidthM": BEARER[2], "frictionCoefficient": 0.5},
        {"id": "bearer-b-ground", "bone": "bearers", "x": BEARER_X, "y": 0, "z": BEARER_Z[1], "kind": "contact", "title": t("Left bearer on the yard gravel", "Linker Unterleger auf dem Hofschotter"), "support": "contact", "normalDeg": 90, "footprintLengthM": BEARER[0], "footprintWidthM": BEARER[2], "frictionCoefficient": 0.5},
        {"id": "foundation-ground-front", "bone": "foundation", "x": r(STEP_D + (found_l - STEP_D) / 4), "y": 0, "kind": "contact", "title": t("Foundation on the formation, lower half", "Fundament auf dem Planum, untere Hälfte"), "support": "contact", "normalDeg": 90, "footprintLengthM": r((found_l - STEP_D) / 2), "footprintWidthM": 2 * FOUND_W, "frictionCoefficient": 0.6},
        {"id": "foundation-ground-rear", "bone": "foundation", "x": r(STEP_D + 3 * (found_l - STEP_D) / 4), "y": 0, "kind": "contact", "title": t("Foundation on the formation, upper half", "Fundament auf dem Planum, obere Hälfte"), "support": "contact", "normalDeg": 90, "footprintLengthM": r((found_l - STEP_D) / 2), "footprintWidthM": 2 * FOUND_W, "frictionCoefficient": 0.6},
        {"id": "hook", "bone": "clamp", "x": 0, "y": r(HOOK_V), "kind": "load", "title": t("Crane hook in the master link", "Kranhaken im Aufhängering")},
        {"id": "load-point", "bone": "load-carrier", "x": 0, "y": RISE, "kind": "load", "title": t("Foot of a walker on the pitch line", "Fuß einer gehenden Person auf der Lauflinie")},
    ]


CONTACTS = ["bearer-a-ground", "bearer-b-ground", "foundation-ground-front", "foundation-ground-rear"]
REGIONS = ["step-granite", "mortar-remnants", "timber", "foundation", "lifting-gear"]
BINDINGS = {
    "stored": {"step-granite": (0.9, 0.35), "mortar-remnants": (1, 0.3), "timber": (0.8, 0.2), "foundation": (0.95, 0.5), "lifting-gear": (1, 0.5)},
    "lifted": {"step-granite": (0.92, 0.38), "mortar-remnants": (1, 0.3), "timber": (0.8, 0.2), "foundation": (0.95, 0.5), "lifting-gear": (1, 0.5)},
    "reinstalled": {"step-granite": (0.7, 0.45), "mortar-remnants": (0.4, 0.1), "timber": (0.9, 0.3), "foundation": (0.3, 0.1), "lifting-gear": (1, 0.5)},
}


def binding(kind):
    """🌬️ The thermal binding of a pose: every region with its exposed and sunlit share."""
    return {"regions": {k: {"exposedFraction": e, "sunlitFraction": s} for k, (e, s) in BINDINGS[kind].items()}, "emitters": {}}


def reinstalled_bones():
    """🧭 The offsets that set the five steps into the flight, drop the timbers onto the bearers and hang the released clamp over the top step."""
    out = {"battens": {"dy": r(-BATTEN_DROP)}}
    for sid in FLIGHT[:3]:
        dx, dy = STEP_TARGET[sid]
        out[sid] = {"dx": r(dx), "dy": r(dy)}
    for sid in ("step-4", "step-5"):
        dx, dy = STEP_TARGET[sid]
        out[f"{sid}-carriage"] = {"dx": r(dx), "dy": r(dy)}
        out[sid] = {"abductionDeg": TURN[sid]}
    dx, dy = STEP_TARGET["clamp"]
    out["clamp"] = {"dx": r(dx), "dy": r(dy)}
    return out


def carrier_at(k):
    """🚶 The offset of the walking-load carrier that puts the foot on tread k (0..4, at its nosing)."""
    return {"dx": r(GOING * k), "dy": r(RISE * k)}


def poses(hook_n, clamp_n):
    """📥 Stored, lifted in the clamp, reset as a flight, reset with a point load on the top tread."""
    re = reinstalled_bones()
    stair = {"id": "stair-imposed", "kind": "live", "title": t("Imposed stair load 3.6 kN/m (3.0 kN/m² over 1.20 m width, horizontal projection)", "Nutzlast der Treppe 3,6 kN/m (3,0 kN/m² über 1,20 m Breite, Grundrissprojektion)"), "fxN": 0, "fyN": 0, "distributed": {"bone": "foundation", "fromM": 0, "toM": r(GOING * 4 + STEP_D), "wxNPerM": 0, "wyNPerM": r(-Q_STAIR * STEP_L * 1000)}, "source": "literature", "reference": "en-1991-1-1"}
    hang = {"id": "hook-hold", "kind": "carried", "title": t("Hook force holding the released clamp", "Hakenkraft, die die gelöste Zange hält"), "anchor": "hook", "fxN": 0, "fyN": r(clamp_n, 1), "source": "authored-estimate", "reference": "en-13155"}
    point = {"id": "walker", "kind": "live", "title": t("Concentrated load 2 kN on the top tread", "Einzellast 2 kN auf der obersten Trittstufe"), "anchor": "load-point", "fxN": 0, "fyN": -Q_POINT, "source": "literature", "reference": "en-1991-1-1"}
    kn = f"{hook_n / 1000:.2f}"
    return [
        {"id": "stored", "title": t("Stored crosswise on bearers", "Kreuzweise auf Unterlegern gelagert"), "description": t("Three block steps lie side by side across two squared-timber bearers, two more lie along the stack on two intermediate timbers; the scissor clamp is set on step 5, ready to lift. Beside the stack waits the stepped foundation of the new flight.", "Drei Blockstufen liegen nebeneinander quer auf zwei Kanthölzern, zwei weitere längs auf zwei Zwischenhölzern; die Scherenzange sitzt hebebereit auf Stufe 5. Neben dem Stapel wartet das gestufte Fundament der neuen Treppe."), "bones": {}, "ground": "authored", "contacts": CONTACTS, "thermal": binding("stored"), "loads": []},
        {"id": "lifted", "title": t("Step 5 lifted in the stone clamp", "Stufe 5 in der Steinzange gehoben"), "description": t(f"Step 5 hangs 0.60 m above the stack in the scissor clamp; the hook carries step, mortar remnants and clamp ({kn} kN); bearers, timbers and the other four steps stay where they lie.", f"Stufe 5 hängt 0,60 m über dem Stapel in der Scherenzange; der Haken trägt Stufe, Mörtelreste und Zange ({kn.replace('.', ',')} kN); Unterleger, Zwischenhölzer und die übrigen vier Stufen bleiben liegen."), "bones": {"step-5-lift": {"dy": LIFT_DY}}, "ground": "authored", "contacts": CONTACTS, "thermal": binding("lifted"), "loads": [{"id": "hook-lift", "kind": "carried", "title": t("Hook force of the crane", "Hakenkraft des Krans"), "anchor": "hook", "fxN": 0, "fyN": r(hook_n, 1), "source": "authored-estimate", "reference": "en-13155"}]},
        {"id": "reinstalled", "title": t("Reset as a flight on the foundation", "Als Treppe auf dem Fundament versetzt"), "description": t("The five steps form a flight of 150 mm rise and 300 mm going (2 r + g = 600 mm), each lapping 50 mm over the one below, on the stepped foundation; the released clamp hangs from the hook over the top step, the timbers lie on the bearers. The flight carries the imposed stair load of 3.0 kN/m².", "Die fünf Stufen bilden eine Treppe mit 150 mm Steigung und 300 mm Auftritt (2 s + a = 600 mm), jede 50 mm auf der unteren aufliegend, auf dem gestuften Fundament; die gelöste Zange hängt über der obersten Stufe am Haken, die Zwischenhölzer liegen auf den Unterlegern. Die Treppe trägt die Nutzlast von 3,0 kN/m²."), "bones": re, "ground": "authored", "contacts": CONTACTS, "thermal": binding("reinstalled"), "loads": [stair, hang]},
        {"id": "reinstalled-point-load", "title": t("Reset flight, 2 kN on the top tread", "Versetzte Treppe, 2 kN auf der obersten Stufe"), "description": t("The reset flight under a concentrated load of 2 kN on the nosing of the top tread.", "Die versetzte Treppe unter einer Einzellast von 2 kN an der Vorderkante der obersten Trittstufe."), "bones": {**re, "load-carrier": carrier_at(4)}, "ground": "authored", "contacts": CONTACTS, "thermal": binding("reinstalled"), "loads": [point, hang]},
    ]


def clips():
    """🎞️ `lift` (the hook takes the weight of step 5, then hoists it 0.60 m) and `climb` (a walker's 2 kN climbs the reset flight tread by tread)."""
    climb_keys = []
    for k in range(5):
        climb_keys.append({"t": 1.6 * k, "pose": "reinstalled-point-load", "ease": "smooth", "bones": {"load-carrier": carrier_at(k)}})
    return [
        {"id": "lift", "title": t("Lift in the stone clamp", "Heben in der Steinzange"), "description": t("The hook takes up the weight of step 5 within one second, then hoists it 0.60 m off the stack; the stack stays on its bearers.", "Der Haken übernimmt in einer Sekunde das Gewicht von Stufe 5 und hebt sie dann 0,60 m vom Stapel; der Stapel bleibt auf seinen Unterlegern."), "durationS": 5, "fps": 30, "loop": False, "rootMotion": "none", "keys": [{"t": 0, "pose": "lifted", "ease": "smooth", "bones": {"step-5-lift": {"dy": 0}}}, {"t": 1, "pose": "lifted", "ease": "smooth", "bones": {"step-5-lift": {"dy": 0}}}, {"t": 5, "pose": "lifted", "ease": "smooth"}], "contacts": [], "phases": [{"id": "take-up", "fromS": 0, "toS": 1, "title": t("Take up the weight", "Gewicht übernehmen")}, {"id": "hoist", "fromS": 1, "toS": 5, "title": t("Hoist", "Anheben")}], "events": [{"t": 1, "kind": "load-on", "title": t("The hook carries the whole step", "Der Haken trägt die ganze Stufe"), "target": "hook-lift"}, {"t": 1, "kind": "motion", "title": t("Step 5 leaves the timbers", "Stufe 5 verlässt die Zwischenhölzer")}], "tracks": [{"target": "load:hook-lift", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 1, "v": 1}, {"t": 5, "v": 1}]}]},
        {"id": "climb", "title": t("A walker climbs the reset flight", "Eine Person steigt die versetzte Treppe hinauf"), "description": t("A walker's 2 kN steps from the bottom tread to the top tread, one tread every 1.6 s, along the pitch line through the nosings.", "Die 2 kN einer gehenden Person steigen von der untersten zur obersten Trittstufe, eine Stufe alle 1,6 s, entlang der Lauflinie durch die Vorderkanten."), "durationS": 6.4, "fps": 30, "loop": False, "rootMotion": "none", "keys": climb_keys, "contacts": [], "phases": [{"id": "climb", "fromS": 0, "toS": 6.4, "title": t("Climb", "Steigen")}], "events": [{"t": 0, "kind": "load-on", "title": t("Foot on the bottom tread", "Fuß auf der untersten Stufe"), "target": "walker"}, {"t": 6.4, "kind": "motion", "title": t("Foot on the top tread", "Fuß auf der obersten Stufe")}], "tracks": []},
    ]


def perimeter(pts):
    """📏 The perimeter of a closed polygon."""
    return sum(math.dist(pts[i], pts[(i + 1) % len(pts)]) for i in range(len(pts)))


def surface(solid):
    """🧴 The whole surface of a solid in m² (prism: two caps and the mantle, holes included)."""
    if solid.kind == "box":
        a, b, c = solid.kw["size"]
        return 2 * (a * b + b * c + c * a)
    length = solid.kw["range"][1] - solid.kw["range"][0]
    caps = 2 * (poly_area(solid.kw["profile"]) - sum(poly_area(h) for h in solid.kw.get("holes", [])))
    return caps + length * (perimeter(solid.kw["profile"]) + sum(perimeter(h) for h in solid.kw.get("holes", [])))


def hand_areas():
    """📐 Hand areas per region: every face of the real parts, including faces that touch a part of another material (mortar on stone, stone on timber, the foundation bed), less the cuts between the length segments of one step and the faces where the clamp parts bear on each other (pads on jaws, pins in arms, links and ring, arm ends in the jaws), which are no faces of the real parts."""
    out = {k: 0.0 for k in ("step-granite", "mortar-remnants", "timber", "foundation", "lifting-gear")}
    solids = body("50")
    for s in solids:
        out[REGION_OF[s.material]] += surface(s)
    for sid in FLIGHT:
        segs = [x for x in solids if x.bone == sid and x.material == "granite"]
        for k in range(len(segs) - 1):
            out["step-granite"] -= 2 * min(poly_area(segs[k].kw["profile"]), poly_area(segs[k + 1].kw["profile"]))
    pads = 2 * 2 * 0.14 * 0.09
    pins = 2 * math.pi * 0.015 * 0.04 + 2 * math.pi * EYE_R * (0.04 + RING_BAR)
    arm_ends = 2 * 0.035 * 2 * (0.06 + 0.02)
    links = 2 * 2 * 0.03 * 0.05
    out["lifting-gear"] -= pads + pins + arm_ends + links
    return {k: r(v, 3) for k, v in out.items()}


AREAS = {}


def thermal():
    """🌡️ Five passive surface regions of real materials on a mild sunny day."""
    reg = lambda id_, title, material, char, geom, sky, cap, ref: {"id": id_, "title": title, "kind": "surface", "model": "passive", "material": material, "areaM2": AREAS[id_], "characteristicLengthM": char, "geometry": geom, "skyViewFactor": sky, "layers": [], "heatCapacityJM2K": cap, "source": "authored-estimate", "reference": ref}
    return {
        "environment": {"airTempC": 20, "meanRadiantTempC": 20, "skyTempC": 8, "groundTempC": 18, "relativeHumidity": 0.5, "windMS": 1, "windDirectionDeg": 0, "solarWM2": 500, "diffuseFraction": 0.2, "pressurePa": 101325},
        "operating": [],
        "regions": [
            reg("step-granite", t("Granite faces of the five steps (worn treads, bush-hammered risers, split beds and ends)", "Granitflächen der fünf Stufen (abgetretene Trittflächen, gestockte Ansichten, gespaltene Lager und Köpfe)"), "granite", 0.35, "horizontal-plate-up", 0.6, 198750, "en-iso-13786"),
            reg("mortar-remnants", t("Old mortar crusts on the wall-side end faces", "Alte Mörtelkrusten an den wandseitigen Kopfseiten"), "old-mortar", 0.2, "vertical-plate", 0.3, 36000, "en-iso-10456"),
            reg("timber", t("Weathered bearers and intermediate timbers", "Verwitterte Unterleger und Zwischenhölzer"), "softwood", 0.1, "vertical-plate", 0.4, 24200, "en-iso-13786"),
            reg("foundation", t("Stepped concrete foundation", "Gestuftes Betonfundament"), "foundation-concrete", 0.3, "horizontal-plate-up", 0.8, 181700, "en-iso-13786"),
            reg("lifting-gear", t("Stone clamp and master link", "Steinzange und Aufhängering"), "clamp-steel", 0.05, "vertical-plate", 0.6, 26500, "en-iso-10456"),
        ],
        "emitters": [],
        "assumptions": [
            t("Passive granite stores heat in about 75 mm, half the step height: the 24 h penetration depth is 170 mm for lambda 2.8 W/mK, rho 2650 kg/m³ and c 1000 J/kgK (EN ISO 13786), half of it is more than half the 150 mm step, so half the step is taken (198.75 kJ/m²K).", "Passiver Granit speichert Wärme in etwa 75 mm, der halben Stufenhöhe: die 24-h-Eindringtiefe beträgt 170 mm für Lambda 2,8 W/mK, Rho 2650 kg/m³ und c 1000 J/kgK (EN ISO 13786); ihre Hälfte übersteigt die halbe Stufe von 150 mm, daher gilt diese (198,75 kJ/m²K)."),
            t("Weathered grey granite absorbs about 0.55 of the sun against about 0.45 for a freshly tooled face; the worn treads are polished but soiled, so the one weathered value is kept for all faces. Weathered softwood 0.7 against 0.6 new. Authored estimates guided by the ASHRAE Handbook, not measurements.", "Verwitterter grauer Granit absorbiert etwa 0,55 der Sonne gegen etwa 0,45 bei frisch bearbeiteter Fläche; die abgetretenen Trittflächen sind poliert, aber verschmutzt, daher gilt der eine Verwitterungswert für alle Flächen. Verwittertes Nadelholz 0,7 gegen 0,6 neu. Schätzungen nach dem ASHRAE Handbook, keine Messungen."),
            t("The mortar remnants are crusts of 20 mm: their heat capacity is their whole thickness (36 kJ/m²K).", "Die Mörtelreste sind Krusten von 20 mm: ihre Wärmekapazität ist ihre ganze Dicke (36 kJ/m²K)."),
            t("Exposed shares follow the pose: in the stack the layers shade each other; in the flight the beds, the lapped backs and the foundation treads are covered by the steps.", "Die freien Anteile folgen der Stellung: im Stapel verschatten sich die Lagen; in der Treppe sind Lager, überdeckte Rückseiten und Fundamentstufen von den Stufen bedeckt."),
            t("Region areas are the whole surfaces of the real parts, including the faces where a part bears on a part of another material (crust on stone, stone on timber, the foundation on its bed); the cuts between the length segments of a step and the contacts between clamp parts are not faces.", "Die Flächen der Regionen sind die ganzen Oberflächen der realen Teile, einschließlich der Flächen, mit denen ein Teil auf einem Teil aus anderem Material aufliegt (Kruste auf Stein, Stein auf Holz, Fundament auf seinem Bett); die Schnitte zwischen den Längenabschnitten einer Stufe und die Berührungen der Zangenteile sind keine Flächen."),
            t("Nothing is heated or operated: every element is passive with no nameplate power.", "Nichts wird beheizt oder betrieben: jedes Element ist passiv ohne Nennleistung."),
        ],
        "passengers": [],
    }


def structure():
    """🏗️ Rigid-body statics on the ground contacts of bearers and foundation, the hook as an applied force."""
    return {"gravityMS2": G, "dynamics": "quasi-static", "supports": CONTACTS, "assumptions": [
        t("Stored: the bottom layer bears on the two 100 x 100 mm bearers (1.30 m strips on the yard), the top layer on two 80 x 80 mm timbers across the bottom steps; stone never touches stone.", "Gelagert: die untere Lage liegt auf den zwei Kanthölzern 100 x 100 mm (Streifen von 1,30 m auf dem Hof), die obere Lage auf zwei Hölzern 80 x 80 mm quer über den unteren Stufen; Stein berührt nie Stein."),
        t("Lifted: the hook force equals the weight of step 5 with its mortar crusts and of the clamp (volume times density). The scissor clamp is a non-fixed load lifting attachment to EN 13155; it holds by friction of its rubber pads on riser and back face, the pinching force growing with the load.", "Gehoben: die Hakenkraft ist das Gewicht von Stufe 5 mit ihren Mörtelkrusten und der Zange (Volumen mal Dichte). Die Scherenzange ist ein loses Lastaufnahmemittel nach EN 13155; sie hält durch Reibung ihrer Gummibeläge an Ansichts- und Rückseite, die Klemmkraft wächst mit der Last."),
        t("Reset: the steps of 150 mm rise and 300 mm going lap 50 mm over each other and bear fully on the stepped foundation of C20/25 in a mortar bed, so they act as blocks in compression, not as beams; the bottom step stands on its frost-proof strip below the ground line (not modelled). Imposed load 3.0 kN/m² and 2.0 kN (EN 1991-1-1 Table 6.2, stairs).", "Versetzt: die Stufen mit 150 mm Steigung und 300 mm Auftritt überdecken sich 50 mm und liegen im Mörtelbett vollflächig auf dem gestuften Fundament C20/25, sie wirken als Druckblöcke, nicht als Balken; die unterste Stufe steht auf ihrem frostfreien Streifen unter der Geländelinie (nicht modelliert). Nutzlast 3,0 kN/m² und 2,0 kN (EN 1991-1-1 Tabelle 6.2, Treppen)."),
        t("The solver treats the whole asset as one rigid body on its ground contacts: stack and foundation share the self-weight by their lever arms; the steps carried over to the flight load the foundation contacts, the timbers left behind the bearers.", "Der Löser behandelt das ganze Asset als einen starren Körper auf seinen Bodenkontakten: Stapel und Fundament teilen das Eigengewicht nach Hebelarmen; die in die Treppe versetzten Stufen belasten die Fundamentkontakte, die zurückgebliebenen Hölzer die Unterleger."),
        t("Granite: apparent density 2650 kg/m³ (EN 1936), E about 50 GPa, flexural strength about 12 MPa (EN 12372), authored typical values for a German grey granite.", "Granit: Rohdichte 2650 kg/m³ (EN 1936), E etwa 50 GPa, Biegefestigkeit etwa 12 MPa (EN 12372), typische Schätzwerte für einen deutschen grauen Granit."),
    ], "clinical": False}


def ref(id_, solid, **label):
    """🔗 A reference solid of the thermal or structure list."""
    return {"id": id_, "of": solid, **label}


def thermal_list(scale):
    """🌡️ Every body solid is its own thermal surface, bound to the region of its material."""
    return [ref(f"th-{s.id}", s.id, region=REGION_OF[s.material]) for s in body(scale)]


def structure_list(scale):
    """🏗️ Members on steps, timber, foundation and clamp; supports at the four ground contacts; the walking-load joint; masses at the centres of the steps and the foundation."""
    f = FEATURE[scale]
    out = []
    masses = mass_by_bone()
    for s in body(scale):
        if s.material in ("old-mortar", "grip-rubber"):
            continue
        part = "joint" if s.id in ("pivot-pin", "eye-pin") else "member"
        out.append(ref(f"st-{s.id}", s.id, member=s.bone, part=part))
    for i, z in enumerate(BEARER_Z):
        out.append({"id": f"support-bearer-{'ab'[i]}", "kind": "box", "bone": "bearers", "material": "softwood", "role": "body", "tone": 0.2, "size": [BEARER[0], r(f), BEARER[2]], "transform": {"translation": [BEARER_X, r(f / 2), z]}, "member": "bearers", "part": "support", "anchor": f"bearer-{'ab'[i]}-ground"})
    fl = GOING * 4 + STEP_D
    for i, (name, k) in enumerate((("front", 0.25), ("rear", 0.75))):
        out.append({"id": f"support-foundation-{name}", "kind": "box", "bone": "foundation", "material": "foundation-concrete", "role": "body", "tone": 0.2, "size": [r((fl - STEP_D) / 2), r(f), 2 * FOUND_W], "transform": {"translation": [r(STEP_D + (fl - STEP_D) * k), r(f / 2), 0]}, "member": "foundation", "part": "support", "anchor": f"foundation-ground-{name}"})
    out.append({"id": "joint-load-carrier", "kind": "box", "bone": "load-carrier", "material": "foundation-concrete", "role": "body", "tone": 0.3, "size": [0.10, r(f), 0.10], "transform": {"translation": [r(STEP_D + 0.10), r(f / 2), 0]}, "member": "load-carrier", "part": "joint"})
    rad = max(f / 2, 0.02)
    for b in FLIGHT + ["foundation"]:
        m, cu, cv, cw = masses[b]
        out.append({"id": f"mass-{b}", "kind": "ellipsoid", "bone": b, "material": "granite" if b.startswith("step") else "foundation-concrete", "role": "body", "tone": 0.5, "radii": [rad, rad, rad], "transform": {"translation": [r(cu, 4), r(cv, 4), r(cw, 4)]}, "member": b, "part": "mass"})
    return out


def representation(scale):
    """🖼️ The authored representation of one preset scale."""
    lo, hi = RANGES[scale]
    return {"status": "authored", "denominator": int(scale), "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[scale], "hatches": HATCHES[scale], "geometry": geometry(scale), "decisions": REP_DECISIONS[scale]}


REP_DECISIONS = {
    "50": {
        "silhouette": t("Side elevation of the yard: the three bottom steps seen on their end faces with the real section, 150 x 350 mm, the worn round nosing at the front and the 50 mm gaps between them, on the end of a 100 x 100 mm bearer; above, the two top steps on their 80 x 80 mm timbers seen along their 1.20 m length, with the scissor clamp on step 5 and its master link; to the right the stepped foundation of the new flight, four rises of 150 mm on treads of 300 mm. Front view: the sections of the two top steps with the crossing arms, jaws and pads of the clamp, the foundation back face behind. Plan: the top steps lying along the stack over the crosswise bottom layer, bearers and timbers projecting, the lead-filled railing post holes and the worn treads.", "Seitenansicht des Lagers: die drei unteren Stufen von ihren Kopfseiten mit dem wahren Querschnitt 150 x 350 mm, der rund abgetretenen Vorderkante und den Fugen von 50 mm, auf dem Ende eines Kantholzes 100 x 100 mm; darüber die zwei oberen Stufen auf ihren Hölzern 80 x 80 mm in ihrer Länge von 1,20 m, mit der Scherenzange auf Stufe 5 und ihrem Aufhängering; rechts das gestufte Fundament der neuen Treppe, vier Steigungen von 150 mm auf Auftritten von 300 mm. Vorderansicht: die Querschnitte der oberen Stufen mit gekreuzten Armen, Backen und Belägen der Zange, dahinter die Rückseite des Fundaments. Draufsicht: die oberen Stufen längs über der quer liegenden unteren Lage, Unterleger und Hölzer stehen vor, die bleigefüllten Geländerlöcher und die abgetretenen Trittflächen."),
        "retained": t("Every trace of use: the tread of each step dished 8 mm over its walked middle 700 mm and its nosing worn round to R 35 there, sharp R 12 arrises over the unwalked ends, the chipped nosing at one end of step 3 (40 x 30 mm), the 20 mm crust of old mortar on the end face that was built into the house wall, the 40 mm railing post holes 100 mm behind the nosing and 120 mm from one end, filled with lead.", "Jede Gebrauchsspur: die Trittfläche jeder Stufe auf den begangenen mittleren 700 mm 8 mm tief ausgetreten und die Vorderkante dort auf R 35 rund abgetreten, scharfe Kanten R 12 an den unbegangenen Enden, die abgeschlagene Vorderkante an einem Ende von Stufe 3 (40 x 30 mm), die 20 mm dicke Kruste alten Mörtels an der Kopfseite, die in der Hauswand steckte, die 40-mm-Geländerlöcher 100 mm hinter der Vorderkante und 120 mm vom einen Ende, mit Blei vergossen."),
        "merged": t("The three length segments of a step (end, worn middle, end) are one outline in elevation and plan where they are seen together; the master link is one ring.", "Die drei Längenabschnitte einer Stufe (Ende, abgetretene Mitte, Ende) sind in Ansicht und Draufsicht eine Kontur, wo sie zusammen gesehen werden; der Aufhängering ist ein Ring."),
        "omitted": t("No hidden lines: the dished tread behind the end faces and the beds under the top layer are not shown; the bush-hammered texture of the risers is the stipple of the granite.", "Keine verdeckten Kanten: die ausgetretene Trittfläche hinter den Kopfseiten und die Lager unter der oberen Lage sind nicht gezeigt; die gestockte Oberfläche der Ansichtsflächen ist die Punktierung des Granits."),
        "thinElements": t("The edge of the wear and the tooling bands are 0.1 mm texture strokes, the worn nosing line 0.18 mm detail; the lead plugs are dark 0.8 mm dots.", "Der Rand der Abnutzung und die Bearbeitungsstreifen sind Texturstriche von 0,1 mm, die abgetretene Vorderkante ein Detailstrich von 0,18 mm; die Bleiplomben sind dunkle Punkte von 0,8 mm."),
        "hatching": t("Granite carries a stipple of 0.8 mm spacing for the split and bush-hammered faces; timber, concrete and steel are tones.", "Granit trägt eine Punktierung von 0,8 mm Abstand für die gespaltenen und gestockten Flächen; Holz, Beton und Stahl sind Töne."),
        "styles": t("Outline 0.35 mm, details 0.18 mm, texture 0.1 mm; the worn middle of each step is a shade lighter.", "Kontur 0,35 mm, Details 0,18 mm, Textur 0,1 mm; die abgetretene Mitte jeder Stufe ist eine Spur heller."),
        "overlays": t("Each step segment, crust, timber, the foundation and each clamp part are their own cells on their own bones.", "Jeder Stufenabschnitt, jede Kruste, jedes Holz, das Fundament und jedes Zangenteil sind eigene Zellen auf eigenen Knochen."),
    },
    "100": {
        "silhouette": t("The stack (6 mm high on paper) with the step sections, the worn nosings as 30 mm chamfers, the bearers and timbers, the clamp with arms, jaws and ring, and the stepped foundation beside it.", "Der Stapel (6 mm hoch auf Papier) mit den Stufenquerschnitten, den abgetretenen Vorderkanten als Fasen von 30 mm, Unterlegern und Hölzern, der Zange mit Armen, Backen und Ring und daneben das gestufte Fundament."),
        "retained": t("Worn and chipped nosings, the mortar crusts (20 mm), the railing post holes and the gaps of the stack at their true places.", "Abgetretene und abgeschlagene Vorderkanten, die Mörtelkrusten (20 mm), die Geländerlöcher und die Fugen des Stapels an ihrer wahren Stelle."),
        "merged": t("The 8 mm dish of the tread (0.08 mm on paper) is merged into a straight tread; the rubber pads into the jaws.", "Die 8 mm tiefe Mulde der Trittfläche (0,08 mm auf Papier) ist in eine gerade Trittfläche aufgegangen; die Gummibeläge in die Backen."),
        "omitted": t("Wear marks and tooling bands, bearer checks.", "Abnutzungsspuren und Bearbeitungsstreifen, Risse der Unterleger."),
        "thinElements": t("The nosing line is a 0.13 mm detail stroke.", "Die Vorderkantenlinie ist ein Detailstrich von 0,13 mm."),
        "hatching": t("Granite stipple of 0.6 mm spacing.", "Granitpunktierung von 0,6 mm Abstand."),
        "styles": t("Outline 0.25 mm, details 0.13 mm, texture 0.1 mm.", "Kontur 0,25 mm, Details 0,13 mm, Textur 0,1 mm."),
        "overlays": t("Steps, crusts, timbers, foundation and clamp parts are their own cells.", "Stufen, Krusten, Hölzer, Fundament und Zangenteile sind eigene Zellen."),
    },
    "200": {
        "silhouette": t("Each step one block with a 30 mm worn nosing chamfer, bearers, timbers, the clamp as jaws, arms and a plain ring, the stepped foundation.", "Jede Stufe ein Block mit einer Fase der abgetretenen Vorderkante von 30 mm, Unterleger, Hölzer, die Zange als Backen, Arme und ein voller Ring, das gestufte Fundament."),
        "retained": t("Stack, gaps, the clamp form and the stepped foundation.", "Stapel, Fugen, die Form der Zange und das gestufte Fundament."),
        "merged": t("The three length segments of a step are one block; the hole of the master link is closed.", "Die drei Längenabschnitte einer Stufe sind ein Block; das Loch des Aufhängerings ist geschlossen."),
        "omitted": t("Railing post holes and the chip; the mortar crusts (20 mm, below the 30 mm feature) are texture solids in 3D and belong to the cell of their step in the drawing.", "Geländerlöcher und die Abplatzung; die Mörtelkrusten (20 mm, unter der Einzelheit von 30 mm) sind in 3D Texturkörper und gehören in der Zeichnung zur Zelle ihrer Stufe."),
        "thinElements": t("None.", "Keine."),
        "hatching": t("No hatch.", "Keine Schraffur."),
        "styles": t("Outline 0.18 mm.", "Kontur 0,18 mm."),
        "overlays": t("Steps, timbers, foundation and clamp parts are their own cells.", "Stufen, Hölzer, Fundament und Zangenteile sind eigene Zellen."),
    },
    "500": {
        "silhouette": t("Five rectangular blocks (0.3 x 0.7 mm on paper) on the bearers and timbers, the clamp as one gripping block, the stepped foundation.", "Fünf rechteckige Blöcke (0,3 x 0,7 mm auf Papier) auf Unterlegern und Hölzern, die Zange als ein greifender Block, das gestufte Fundament."),
        "retained": t("The crosswise layers and the clearance under the bottom layer, the stepped foundation with its four rises.", "Die kreuzweisen Lagen und der Abstand unter der unteren Lage, das gestufte Fundament mit seinen vier Steigungen."),
        "merged": t("The clamp is one block; nosings are square.", "Die Zange ist ein Block; Vorderkanten sind rechtwinklig."),
        "omitted": t("All traces below 75 mm: wear, chip, holes, the ring opening; the 20 mm mortar crusts are texture solids in 3D and belong to the cell of their step in the drawing.", "Alle Spuren unter 75 mm: Abnutzung, Abplatzung, Löcher, die Ringöffnung; die Mörtelkrusten von 20 mm sind in 3D Texturkörper und gehören in der Zeichnung zur Zelle ihrer Stufe."),
        "thinElements": t("None.", "Keine."),
        "hatching": t("No hatch.", "Keine Schraffur."),
        "styles": t("Outline 0.13 mm.", "Kontur 0,13 mm."),
        "overlays": t("Steps, timbers, foundation and clamp are their own cells.", "Stufen, Hölzer, Fundament und Zange sind eigene Zellen."),
    },
}

VOL_DECISIONS = {
    "50": [
        t("Body: each granite block step is three extrusions along its length because the kernel has no boolean subtraction: two end segments of 250 mm with the sharp section (R 12 nosing) and the walked middle of 700 mm whose section carries the 8 mm dish of the tread and the nosing worn round to R 35; step 3 has a 120 mm end segment with the chipped nosing (40 x 30 mm). The end face that was built into the house wall carries a 20 mm mortar crust over the rear 210 mm; the free end carries the lead-filled railing post hole. The bottom steps lie across the bearers (sections in the paper plane), the top steps along the stack on their own bones about whose normal they turn a quarter turn into the flight.", "Körper: jede Granit-Blockstufe besteht aus drei Extrusionen entlang ihrer Länge, weil der Kern keine boolesche Subtraktion kennt: zwei Endabschnitte von 250 mm mit dem scharfen Querschnitt (Vorderkante R 12) und die begangene Mitte von 700 mm, deren Querschnitt die 8 mm tiefe Mulde der Trittfläche und die auf R 35 abgetretene Vorderkante trägt; Stufe 3 hat einen Endabschnitt von 120 mm mit der abgeschlagenen Vorderkante (40 x 30 mm). Die Kopfseite, die in der Hauswand steckte, trägt über die hinteren 210 mm eine Mörtelkruste von 20 mm; das freie Ende trägt das bleigefüllte Geländerloch. Die unteren Stufen liegen quer auf den Unterlegern (Querschnitte in der Zeichenebene), die oberen längs auf eigenen Knochen, um deren Normale sie sich mit einer Vierteldrehung in die Treppe drehen."),
        t("Clamp: jaws 160 x 120 x 15 mm with 10 mm rubber pads on riser and back face of step 5, two crossing flat-bar arms 60 x 20 mm on a 30 mm pivot pin, two links to the 25 mm eye pin and a master link of 16 mm bar; storage aids: two 100 x 100 x 1300 mm bearers and two 80 x 80 x 1200 mm intermediate timbers on their own bones; the stepped foundation of the new flight stands beside the stack.", "Zange: Backen 160 x 120 x 15 mm mit Gummibelägen von 10 mm an Ansichts- und Rückseite von Stufe 5, zwei gekreuzte Flachstahlarme 60 x 20 mm auf einem Drehbolzen von 30 mm, zwei Laschen zum Ösenbolzen von 25 mm und ein Aufhängering aus Rundstahl 16 mm; Lagerhilfen: zwei Kanthölzer 100 x 100 x 1300 mm und zwei Zwischenhölzer 80 x 80 x 1200 mm auf eigenen Knochen; das gestufte Fundament der neuen Treppe steht neben dem Stapel."),
        t("Thermal: every body solid is its own thermal surface bound to the region of its material; the faces where solids touch are not outer area.", "Thermisch: jeder Körper ist seine eigene thermische Fläche in der Region seines Materials; Berührflächen der Körper sind keine Außenfläche."),
        t("Structure: the step segments, timbers, foundation and clamp parts are members of their bones, the pins joints; supports of one smallest feature lie under the bearers and the foundation at their ground contacts; mass markers at the centres of the steps and the foundation.", "Tragwerk: die Stufenabschnitte, Hölzer, das Fundament und die Zangenteile sind Glieder ihrer Knochen, die Bolzen Gelenke; Auflager von einer kleinsten Einzelheit liegen unter Unterlegern und Fundament an ihren Bodenkontakten; Massenmarken in den Schwerpunkten der Stufen und des Fundaments."),
    ],
    "100": [
        t("Body: the same segments generalised for the 15 mm feature: the dish (8 mm) is merged into a straight tread with a 30 mm chamfer as the worn nosing, the sharp ends a 15 mm chamfer; the rubber pads (10 mm) are merged into 25 mm jaws; crusts stay (20 mm).", "Körper: dieselben Abschnitte verallgemeinert für die Einzelheit von 15 mm: die Mulde (8 mm) geht in eine gerade Trittfläche mit einer Fase von 30 mm als abgetretener Vorderkante auf, die scharfen Enden haben eine Fase von 15 mm; die Gummibeläge (10 mm) gehen in Backen von 25 mm auf; die Krusten bleiben (20 mm)."),
        t("Minimal: the thermal list follows the body solid by solid; the structure keeps one member per part and the supports grow to 15 mm.", "Minimal: die thermische Liste folgt dem Körper Stück für Stück; das Tragwerk behält ein Glied je Teil und die Auflager wachsen auf 15 mm."),
    ],
    "200": [
        t("Body: each step one block with a 30 mm nosing chamfer; the mortar crusts (20 mm) become texture; the clamp has 30 mm arms and jaws and a closed master link.", "Körper: jede Stufe ein Block mit einer Fase von 30 mm an der Vorderkante; die Mörtelkrusten (20 mm) werden Textur; die Zange hat Arme und Backen von 30 mm und einen geschlossenen Aufhängering."),
        t("Minimal: thermal and structure follow the body solid by solid with 30 mm supports.", "Minimal: Thermik und Tragwerk folgen dem Körper Stück für Stück mit Auflagern von 30 mm."),
    ],
    "500": [
        t("Body: five rectangular blocks, bearers (100 mm) and timbers (80 mm) above the 75 mm feature, the clamp as one block that grips step 5 with 75 mm legs, the stepped foundation.", "Körper: fünf rechteckige Blöcke, Unterleger (100 mm) und Hölzer (80 mm) über der Einzelheit von 75 mm, die Zange als ein Block, der Stufe 5 mit Schenkeln von 75 mm greift, das gestufte Fundament."),
        t("Minimal: thermal and structure follow the body solid by solid with 75 mm supports.", "Minimal: Thermik und Tragwerk folgen dem Körper Stück für Stück mit Auflagern von 75 mm."),
    ],
}

REFERENCES = [
    {"id": "din-18065", "citation": "DIN 18065:2020-08 Stairs in buildings, terminology, measuring rules, main dimensions: step rule 2 r + g = 590 to 650 mm; outdoor block steps of 150 mm rise and 300 mm going give 600 mm."},
    {"id": "din-18332", "citation": "DIN 18332:2019-09 VOB Part C, natural stone works (ATV): block steps (Blockstufen) of solid natural stone laid on a foundation with an overlap of at least 30 mm on the step below, mortar bed; usual sections 15/35 and 16/35 cm, lengths 1.00 to 1.50 m."},
    {"id": "en-1936", "citation": "EN 1936:2006 Natural stone test methods, determination of real density and apparent density: German grey granites (Lausitz, Fichtelgebirge) about 2600 to 2700 kg/m³; 2650 kg/m³ taken."},
    {"id": "en-12372", "citation": "EN 12372:2006 Natural stone test methods, determination of flexural strength under concentrated load: granite typically 10 to 20 MPa; 12 MPa taken as a conservative estimate for weathered reclaimed stone."},
    {"id": "en-1991-1-1", "citation": "EN 1991-1-1:2002 with DIN EN 1991-1-1/NA:2010 Table 6.1DE: stairs qk 3.0 kN/m², Qk 2.0 kN; Annex A: plain concrete 24 kN/m³."},
    {"id": "en-1992-1-1", "citation": "EN 1992-1-1:2004 Eurocode 2, Table 3.1: concrete C20/25, fck 20 MPa, Ecm 30 GPa."},
    {"id": "en-338", "citation": "EN 338:2016 Structural timber, strength classes, Table 1: C24 fm,k 24 N/mm², E0,mean 11.0 kN/mm², rho mean 420 kg/m³."},
    {"id": "en-13155", "citation": "EN 13155:2020 Cranes, safety, non-fixed load lifting attachments: clamps that hold by friction; the clamping force follows from the load through the scissor linkage."},
    {"id": "en-10025-2", "citation": "EN 10025-2:2019 Hot rolled products of structural steels, S355: fy 355 MPa, E 210 GPa, rho 7850 kg/m³."},
    {"id": "en-iso-10456", "citation": "EN ISO 10456:2007 Building materials and products, hygrothermal properties, Table 3: granite lambda 2.8 W/mK, c 1000 J/kgK; mortar 1800 kg/m³ lambda 1.0 W/mK; rubber 1200 kg/m³ lambda 0.17 W/mK; timber 450 kg/m³ lambda 0.13 W/mK."},
    {"id": "en-iso-13786", "citation": "EN ISO 13786:2017 Thermal performance of building components, dynamic thermal characteristics: periodic penetration depth delta = sqrt(lambda T / (pi rho c)); half of it is taken as the effective storage depth of an exposed face."},
    {"id": "ashrae-fundamentals", "citation": "ASHRAE Handbook Fundamentals (2021), chapter 4: emissivity of stone and concrete about 0.9 to 0.93, solar absorptance of weathered versus fresh surfaces taken as rounded authored estimates guided by this chapter."},
    {"id": "dguv-209-013", "citation": "DGUV Information 209-013 Anschläger (2021): lifting of natural stone elements with clamps, set the clamp in the centre of gravity, clear the area under the load."},
    {"id": "din-iso-128-50", "citation": "DIN ISO 128-50:2002 Technical drawings, general principles of presentation, part 50: basic conventions for areas on sections and cuts (hatching)."},
    {"id": "din-1356-1", "citation": "DIN 1356-1:1995 Building drawings, part 1: types of drawings, contents and basic principles of presentation (scales, line groups)."},
]


def bounds_of(scale):
    """📏 World rest bounds of the body of a scale."""
    lo, hi = [1e9] * 3, [-1e9] * 3
    for s in body(scale):
        (u0, u1), (v0, v1), (w0, w1) = s.local_box()
        for p in ((u0, v0, w0), (u1, v1, w1)):
            q = to_world(s.bone, p)
            for i in range(3):
                lo[i], hi[i] = min(lo[i], q[i]), max(hi[i], q[i])
    return lo, hi


def document():
    """📄 The whole asset document."""
    BONES.clear()
    ORIGIN.clear()
    build_rig()
    masses = finish_rig()
    AREAS.update(hand_areas())
    total = sum(m[0] for m in masses.values())
    m5 = masses["step-5"][0]
    mc = masses["clamp"][0]
    lo, hi = bounds_of("50")
    step_kg = sum(masses[s][0] for s in FLIGHT)
    asset = {
        "id": "reclaimed-stone-steps",
        "category": "components",
        "title": t("Reclaimed Stone Block Steps", "Gebrauchte Blockstufen"),
        "summary": t(
            "Five solid granite block steps of 150 x 350 x 1200 mm from a dismantled outdoor stair, as a component exchange stores them crosswise on squared-timber bearers: treads dished and nosings worn round over the walked middle, bush-hammered risers, mortar crusts on the ends that were built into the house wall, lead-filled railing post holes and one chipped nosing; one step lifted in a scissor stone clamp, and all five reset as a flight of 150/300 mm on a stepped concrete foundation.",
            "Fünf massive Granit-Blockstufen 150 x 350 x 1200 mm aus einer rückgebauten Außentreppe, wie eine Bauteilbörse sie kreuzweise auf Kanthölzern lagert: Trittflächen in der begangenen Mitte ausgetreten und Vorderkanten rund abgetreten, gestockte Ansichtsflächen, Mörtelkrusten an den Köpfen, die in der Hauswand steckten, bleigefüllte Geländerlöcher und eine abgeschlagene Vorderkante; eine Stufe in der Scheren-Steinzange gehoben, und alle fünf als Treppe 15/30 auf einem gestuften Betonfundament wieder versetzt."),
        "tags": ["component", "reclaimed", "stone", "granite", "natural-stone", "stair", "block-step", "landscape", "structural"],
        "provenance": {"author": "Semio staffage component authors", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES, "notes": t(
            f"Five steps of 150 x 350 x 1200 mm, {step_kg / 5:.1f} kg each on average after wear and with crusts, {step_kg:.0f} kg together at 2650 kg/m³; flight of 150 mm rise and 300 mm going with 50 mm overlap; foundation {masses['foundation'][0]:.0f} kg of C20/25. The wear depth, chip and post-hole positions are authored estimates of a typical 19th to early 20th century outdoor stair, not a measured specimen.",
            f"Fünf Stufen 150 x 350 x 1200 mm, im Mittel je {de_num(step_kg / 5, '{:.1f}')} kg nach Abnutzung und mit Krusten, zusammen {step_kg:.0f} kg bei 2650 kg/m³; Treppe mit 150 mm Steigung und 300 mm Auftritt bei 50 mm Überdeckung; Fundament {masses['foundation'][0]:.0f} kg aus C20/25. Abnutzungstiefe, Abplatzung und Lage der Geländerlöcher sind Schätzungen einer typischen Außentreppe des 19. bis frühen 20. Jahrhunderts, kein gemessenes Stück.")},
        "dimensions": {"heightM": r(hi[1] - lo[1], 4), "widthM": r(hi[0] - lo[0], 4), "depthM": r(hi[2] - lo[2], 4), "massKg": r(total, 2)},
        "materials": MATERIALS,
        "rig": {"kind": "structural", "root": "bearers", "bones": BONES},
        "anchors": anchors(),
        "representations": {s: representation(s) for s in SCALES},
        "symbols": [],
        "poses": poses((m5 + mc) * G, mc * G),
        "clips": clips(),
        "thermal": thermal(),
        "structure": structure(),
        "volumes": {s: {"status": "authored", "denominator": int(s), "body": [x.doc() for x in body(s)], "thermal": thermal_list(s), "structure": structure_list(s), "decisions": VOL_DECISIONS[s]} for s in SCALES},
        "presentation": {"defaultPose": "stored", "defaultClip": "lift", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}, masses, (lo, hi)


if __name__ == "__main__":
    doc, masses, (lo, hi) = document()
    (HERE / "🔣️.json").write_text(json.dumps(doc, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    for b, (m, *_c) in masses.items():
        print(f"[DEBUG] mass {b} {m:.2f} kg")
    print(f"[DEBUG] total {sum(m[0] for m in masses.values()):.2f} kg; bounds x {lo[0]:.3f}..{hi[0]:.3f} y {lo[1]:.3f}..{hi[1]:.3f} z {lo[2]:.3f}..{hi[2]:.3f}")
    print(f"[DEBUG] targets {STEP_TARGET}")
    print(f"[DEBUG] areas {AREAS}")
