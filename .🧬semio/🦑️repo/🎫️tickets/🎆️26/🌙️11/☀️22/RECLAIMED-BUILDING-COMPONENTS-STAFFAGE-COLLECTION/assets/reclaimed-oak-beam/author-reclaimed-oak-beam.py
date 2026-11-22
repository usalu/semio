"""🪵 Authoritative chisel of the staffage asset `reclaimed-oak-beam`: every hand-chosen number of the reclaimed half-timber oak beam on its two bearers, written out as the asset document `🔣️.json` next to this script.

Run: `python3 author-reclaimed-oak-beam.py` (prints the reference measurements of the model).

See ../../brief.md for the asset contract and ../../reports/reclaimed-oak-beam.md for the reference table.
"""
import json
import math
from pathlib import Path

HERE = Path(__file__).resolve().parent

L = 5.20
B = 0.20
H = 0.24
HB = H / 2
YC = 0.10 + HB
SLANT = 0.012
LAP_X = 4.90
BEARER_X = (1.05, 4.15)
BEARER_H = 0.10
BEARER_W = 0.10
BEARER_L = 0.60
STUDS = (0.40, 1.65, 2.90, 4.15)
MORTISE_L = 0.14
MORTISE_W = 0.05
MORTISE_FLOOR = HB - 0.075
BRACE = (3.42, 3.56, 3.62, HB - 0.06)
PEGS = [(x, HB - 0.04) for x in STUDS] + [(3.48, HB - 0.035)]
PEG_R = 0.0125
TONGUE_PEGS = ((4.98, 0.05), (5.12, -0.05))
SLINGS = (1.20, 4.00)
BEARINGS = (0.10, 5.10)
LIFT = 1.00
REINSTALL = 2.30
RHO_OAK = 690.0
RHO_SOFT = 420.0
SPACING = 0.80
G_FLOOR = 1.2
Q_FLOOR = 2.0
FEATURE = {"50": 0.0075, "100": 0.015, "200": 0.03, "500": 0.075}


def t(en, de):
    """🗣️ A bilingual text, English first."""
    return {"en": en, "de": de}


def r(v):
    """🎯 Rounds a coordinate to a tenth of a millimetre."""
    return round(v + 0.0, 5)


def circle(cx, cy, rad, n):
    """⭕ A circle as an n-gon, counter-clockwise."""
    return [[r(cx + rad * math.cos(2 * math.pi * k / n)), r(cy + rad * math.sin(2 * math.pi * k / n))] for k in range(n)]


def ring_d(points):
    """✒️ One closed SVG subpath through the points."""
    return "M " + " L ".join(f"{p[0]:.5f} {p[1]:.5f}" for p in points) + " Z"


def mortise_spans():
    """📐 The x spans of the four stud mortises along the beam."""
    return [(x - MORTISE_L / 2, x + MORTISE_L / 2) for x in STUDS]


def core_profile(slant):
    """🪓 Elevation profile (u, v) of the 50 mm core lamina with the mortise notches cut from the top."""
    pts = [[0.0, -HB], [LAP_X, -HB], [LAP_X, HB]]
    a, f, b, fl = BRACE
    notches = [(s, e, MORTISE_FLOOR, None) for s, e in mortise_spans()] + [(a, b, fl, f)]
    for s, e, floor, inclined in sorted(notches, key=lambda n: -n[0]):
        if inclined is None:
            pts += [[e, HB], [e, floor], [s, floor], [s, HB]]
        else:
            pts += [[e, HB], [inclined, floor], [s, floor], [s, HB]]
    pts.append([slant, HB])
    return [[r(x), r(y)] for x, y in pts]


def cheek_profile(slant):
    """🪵 Elevation profile (u, v) of one 75 mm cheek lamina."""
    return [[0.0, -HB], [LAP_X, -HB], [LAP_X, HB], [slant, HB]]


def beam_outline(slant):
    """🧱 Elevation outline of the whole beam with the top half-lap at the right end."""
    return [[0.0, -HB], [L, -HB], [L, 0.0], [LAP_X, 0.0], [LAP_X, HB], [slant, HB]]


def ex(id_, bone, material, tone, profile, plane, lo, hi, holes=None, **kw):
    """🧊 An extrude solid."""
    s = {"id": id_, "kind": "extrude", "bone": bone, "material": material, "role": "body", "tone": tone, "profile": profile, "plane": plane, "from": r(lo), "to": r(hi)}
    if holes:
        s["holes"] = holes
    s.update(kw)
    return s


def box(id_, bone, material, tone, size, at, **kw):
    """📦 A box solid centred at `at` in its bone frame."""
    s = {"id": id_, "kind": "box", "bone": bone, "material": material, "role": "body", "tone": tone, "size": [r(v) for v in size], "transform": {"translation": [r(v) for v in at]}}
    s.update(kw)
    return s


def bearer_solids(prefix="", **kw):
    """🪚 The two softwood bearers, 100 x 100 x 600 mm, on their downward bones."""
    return [box(f"{prefix}bearer-{side}", f"bearer-{side}", "softwood", 0.62, [BEARER_H, BEARER_W, BEARER_L], [BEARER_H / 2, 0, 0], **kw) for side in ("left", "right")]


def body(scale):
    """🪵 The body list of one scale."""
    if scale in ("50", "100"):
        n = 24 if scale == "50" else 12
        slant = SLANT if scale == "50" else 0.0
        pegs = [circle(x, y, PEG_R, n) for x, y in PEGS]
        tongue = [circle(w, u, PEG_R, n) for u, w in TONGUE_PEGS]
        return [
            ex("cheek-right", "beam", "oak-weathered", 0.42, cheek_profile(slant), "xy", -B / 2, -MORTISE_W / 2, holes=pegs),
            ex("core", "beam", "oak-weathered", 0.42, core_profile(slant), "xy", -MORTISE_W / 2, MORTISE_W / 2),
            ex("cheek-left", "beam", "oak-weathered", 0.42, cheek_profile(slant), "xy", MORTISE_W / 2, B / 2, holes=pegs),
            ex("lap-tongue", "beam", "oak-weathered", 0.46, [[-B / 2, LAP_X], [B / 2, LAP_X], [B / 2, L], [-B / 2, L]], "zx", -HB, 0.0, holes=tongue),
            *bearer_solids(),
        ]
    if scale == "200":
        holes = [[[-MORTISE_W / 2, r(s)], [MORTISE_W / 2, r(s)], [MORTISE_W / 2, r(e)], [-MORTISE_W / 2, r(e)]] for s, e in mortise_spans() + [(BRACE[0], BRACE[2])]]
        return [
            ex("beam-lower", "beam", "oak-weathered", 0.42, [[0.0, -HB], [L, -HB], [L, 0.0], [LAP_X, 0.0], [LAP_X, MORTISE_FLOOR], [0.0, MORTISE_FLOOR]], "xy", -B / 2, B / 2),
            ex("beam-upper", "beam", "oak-weathered", 0.42, [[-B / 2, 0.0], [B / 2, 0.0], [B / 2, LAP_X], [-B / 2, LAP_X]], "zx", MORTISE_FLOOR, HB, holes=holes),
            *bearer_solids(),
        ]
    return [ex("beam", "beam", "oak-weathered", 0.42, beam_outline(0.0), "xy", -B / 2, B / 2), *bearer_solids()]


def upper_profile(top, pocket=None, soffit=(0.05, 0.04)):
    """🔺 Section (v, w) of the weathered upper-and-side region over the soffit trapezoid, optionally with a mortise pocket."""
    h, wtop = soffit
    pts = [[top, -B / 2]]
    if pocket is not None:
        pts += [[top, -MORTISE_W / 2], [pocket, -MORTISE_W / 2], [pocket, MORTISE_W / 2], [top, MORTISE_W / 2]]
    pts += [[top, B / 2], [-HB, B / 2], [-HB + h, wtop], [-HB + h, -wtop], [-HB, -B / 2]]
    return [[r(v), r(w)] for v, w in pts]


def soffit_profile(soffit=(0.05, 0.04)):
    """🔻 Section (v, w) of the soffit region: a flat trapezoid under the beam."""
    h, wtop = soffit
    return [[-HB, -B / 2], [-HB, B / 2], [-HB + h, wtop], [-HB + h, -wtop]]


def thermal(scale):
    """🌡️ The thermal list of one scale: regions as solids with the internal faces declared as section faces."""
    out = []
    if scale == "50":
        out.append(ex("th-cut-end", "beam", "oak-fresh", 0.75, [[0.0, -HB], [0.0195, -HB], [0.0195, HB], [SLANT, HB]], "xy", -B / 2, B / 2, region="cut-end", sectionFaces=["+u", "+v", "-v", "+w", "-w"]))
        start = 0.0195
        cuts = sorted([(s, e, MORTISE_FLOOR) for s, e in mortise_spans()] + [(BRACE[0], 3.59, BRACE[3])])
        x = start
        k = 0
        for s, e, floor in cuts:
            out.append(ex(f"th-upper-{k:02d}", "beam", "oak-weathered", 0.42, upper_profile(HB), "yz", x, s, region="beam-faces", sectionFaces=["-v", "-u"] if k == 0 else ["-v"]))
            k += 1
            out.append(ex(f"th-upper-{k:02d}", "beam", "oak-weathered", 0.42, upper_profile(HB, floor), "yz", s, e, region="beam-faces", sectionFaces=["-v"]))
            k += 1
            x = e
        out.append(ex(f"th-upper-{k:02d}", "beam", "oak-weathered", 0.42, upper_profile(HB), "yz", x, LAP_X, region="beam-faces", sectionFaces=["-v"]))
        out.append(ex("th-tongue", "beam", "oak-weathered", 0.46, upper_profile(0.0), "yz", LAP_X, L, region="beam-faces", sectionFaces=["-v"]))
        out.append(ex("th-soffit", "beam", "oak-weathered", 0.42, soffit_profile(), "yz", start, L, region="beam-soffit", sectionFaces=["+v", "-u"]))
    else:
        e = {"100": 0.015, "200": 0.03, "500": 0.075}[scale]
        soffit = {"100": (0.05, 0.04), "200": (0.05, 0.04), "500": (0.075, 0.01)}[scale]
        out.append(box("th-cut-end", "beam", "oak-fresh", 0.75, [e, H, B], [e / 2, 0, 0], region="cut-end", sectionFaces=["+u", "+v", "-v", "+w", "-w"]))
        out.append(ex("th-upper", "beam", "oak-weathered", 0.42, upper_profile(HB, soffit=soffit), "yz", e, LAP_X, region="beam-faces", sectionFaces=["-v", "-u"]))
        out.append(ex("th-tongue", "beam", "oak-weathered", 0.46, upper_profile(0.0, soffit=soffit), "yz", LAP_X, L, region="beam-faces", sectionFaces=["-v"]))
        out.append(ex("th-soffit", "beam", "oak-weathered", 0.42, [[r(v), r(w)] for v, w in soffit_profile(soffit)], "yz", e, L, region="beam-soffit", sectionFaces=["+v", "-u"]))
    out += [{"id": f"th-bearer-{side}", "of": f"bearer-{side}", "region": "bearers"} for side in ("left", "right")]
    return out


def structure(scale):
    """🏗️ The structure list of one scale: the beam member with its lap, the two bearer members and the joints, sized by the smallest feature."""
    f = FEATURE[scale]
    slant = SLANT if scale == "50" else 0.0
    return [
        ex("beam-member", "beam", "oak-weathered", 0.5, beam_outline(slant), "xy", -B / 2, B / 2, member="beam", part="member"),
        *[box(f"bearer-{side}-member", f"bearer-{side}", "softwood", 0.62, [BEARER_H, BEARER_W, BEARER_L], [BEARER_H / 2, 0, 0], member=f"bearer-{side}", part="member") for side in ("left", "right")],
        *[box(f"joint-bearer-{side}", f"bearer-{side}", "softwood", 0.3, [f, BEARER_W, B], [0, 0, 0], member=f"bearer-{side}", part="joint") for side in ("left", "right")],
        box("joint-load-carrier", "load-carrier", "oak-weathered", 0.3, [0.1, f, B], [0, -f / 2, 0], member="load-carrier", part="joint"),
    ]


class Rand:
    """🎲 A tiny deterministic generator so the hand strokes are the same on every run."""

    def __init__(self, seed):
        self.s = seed

    def __call__(self):
        self.s = (self.s * 1103515245 + 12345) % 2147483648
        return self.s / 2147483648


def stroke(id_, bone, pts, role="texture", region="beam-faces", tone=0.3, **kw):
    """〰️ An open stroke."""
    s = {"id": id_, "bone": bone, "role": role, "z": 1.5 if role == "texture" else 1.6, "filled": False, "tone": tone, "region": region, "kind": "poly", "points": [[r(x), r(y)] for x, y in pts], "closed": False}
    s.update(kw)
    return s


def rect(id_, bone, x, y, w, h, z, tone, role="body", region=None, regions=None, material=None, **kw):
    """▭ A filled rectangle."""
    s = {"id": id_, "bone": bone, "role": role, "z": z, "filled": True, "tone": tone}
    if region:
        s["region"] = region
    if regions:
        s["regions"] = regions
    if material:
        s["material"] = material
    s.update({"kind": "rect", "x": r(x), "y": r(y), "w": r(w), "h": r(h)})
    s.update(kw)
    return s


def path(id_, bone, rings, z, tone, region=None, regions=None, material=None, role="body"):
    """🕳️ A filled path: the first ring is the outline, the further rings are see-through holes."""
    s = {"id": id_, "bone": bone, "role": role, "z": z, "filled": True, "tone": tone}
    if region:
        s["region"] = region
    if regions:
        s["regions"] = regions
    if material:
        s["material"] = material
    s.update({"kind": "path", "d": " ".join(ring_d(ring) for ring in rings)})
    return s


def poly(id_, bone, pts, z, tone, region=None, regions=None, material=None, role="body"):
    """⬠ A filled closed polygon."""
    s = {"id": id_, "bone": bone, "role": role, "z": z, "filled": True, "tone": tone}
    if region:
        s["region"] = region
    if regions:
        s["regions"] = regions
    if material:
        s["material"] = material
    s.update({"kind": "poly", "points": [[r(x), r(y)] for x, y in pts], "closed": True})
    return s


def hewing_marks(prefix, rnd, xs, bands, length, avoid):
    """🪓 Short broad-axe strokes across a hewn face: each is a shallow arc of the blade."""
    out = []
    k = 0
    for x0 in xs:
        for lo, hi in bands:
            x = x0 + (rnd() - 0.5) * 0.08
            dx = length * (0.8 + 0.4 * rnd())
            dy = dx * (0.35 + 0.25 * rnd())
            y = lo + rnd() * max(hi - lo - dy - 0.006, 0.0)
            if any(abs(x - ax) < 0.07 and abs(y - ay) < 0.05 for ax, ay in avoid):
                continue
            out.append(stroke(f"{prefix}-{k:02d}", "beam", [(x, y), (x + dx / 2, y + dy / 2 + 0.006), (x + dx, y + dy)]))
            k += 1
    return out


def check_line(id_, rnd, x0, x1, y, amp, steps, bone="beam"):
    """⚡ A drying check: a long, slightly wandering crack along the grain."""
    pts = [(x0 + (x1 - x0) * i / steps, y + (rnd() - 0.5) * 2 * amp) for i in range(steps + 1)]
    return stroke(id_, bone, pts)


def rings_front(prefix, pith, radii, box_):
    """🌳 Growth rings of the boxed-heart section seen on the end grain, clipped to the face."""
    x0, y0, x1, y1 = box_
    out = []
    for i, rad in enumerate(radii):
        runs, cur = [], []
        for k in range(97):
            a = 2 * math.pi * k / 96
            p = (pith[0] + rad * math.cos(a), pith[1] + rad * math.sin(a) * 0.94)
            if x0 + 0.004 < p[0] < x1 - 0.004 and y0 + 0.004 < p[1] < y1 - 0.004:
                cur.append(p)
            else:
                if len(cur) > 1:
                    runs.append(cur)
                cur = []
        if len(cur) > 1:
            runs.append(cur)
        for j, run in enumerate(runs):
            out.append(stroke(f"{prefix}-{i}-{j}", "beam", run))
    return out


def peg_avoid():
    """🚫 Peg hole centres the hewing marks keep clear of."""
    return PEGS


def elevation(scale):
    """👁️ Side elevation of one scale (bone-local metres)."""
    rnd = Rand({"50": 11, "100": 23, "200": 37, "500": 41}[scale])
    out = []
    if scale in ("50", "100"):
        slant = SLANT if scale == "50" else 0.0
        n = 24 if scale == "50" else 12
        rings = [beam_outline(slant)] + [circle(x, y, PEG_R, n) for x, y in PEGS]
        out.append(path("beam", "beam", rings, 1, 0.42, regions=["beam-faces", "beam-soffit", "cut-end"], material="oak-weathered"))
        if scale == "50":
            out += hewing_marks("hew", rnd, [0.18 + 0.27 * i for i in range(18)], [(-0.1, -0.05), (-0.02, 0.03), (0.05, 0.1)], 0.07, peg_avoid())
            out.append(check_line("check-01", rnd, 0.55, 2.35, 0.012, 0.004, 14))
            out.append(check_line("check-02", rnd, 3.05, 4.55, -0.035, 0.004, 12))
            out.append(stroke("mark-roman-3", "beam", [(1.56, -0.03), (1.56, 0.02)], role="detail"))
            out.append(stroke("mark-roman-3b", "beam", [(1.585, -0.03), (1.585, 0.02)], role="detail"))
            out.append(stroke("mark-roman-3c", "beam", [(1.61, -0.03), (1.61, 0.02), (1.64, 0.035)], role="detail"))
        else:
            out += hewing_marks("hew", rnd, [0.3 + 0.55 * i for i in range(9)], [(-0.08, -0.02), (0.02, 0.07)], 0.12, peg_avoid())
            out.append(check_line("check-01", rnd, 0.55, 2.35, 0.012, 0.004, 6))
    elif scale == "200":
        out.append(poly("beam", "beam", beam_outline(0.0), 1, 0.42, regions=["beam-faces", "beam-soffit", "cut-end"], material="oak-weathered"))
        out.append(stroke("check-01", "beam", [(0.55, 0.012), (2.35, 0.012)], role="detail", tone=0.35))
    else:
        out.append(poly("beam", "beam", beam_outline(0.0), 1, 0.4, regions=["beam-faces", "beam-soffit", "cut-end"], material="oak-weathered"))
    for side in ("left", "right"):
        out.append(rect(f"bearer-{side}", f"bearer-{side}", 0.0, -BEARER_W / 2, BEARER_H, BEARER_W, 1, 0.62, region="bearers", material="softwood"))
        if scale == "50":
            out.append(stroke(f"bearer-{side}-ring", f"bearer-{side}", [(0.03, -0.05), (0.045, -0.02), (0.07, 0.0), (0.1, 0.01)], region="bearers"))
            out.append(stroke(f"bearer-{side}-check", f"bearer-{side}", [(0.01, 0.012), (0.045, 0.004)], region="bearers"))
    return out


def lifted_strokes(shapes, z):
    """⬆️ Lifts every open stroke above the filled faces it lies on."""
    for s in shapes:
        if not s["filled"]:
            s["z"] = z + (0.1 if s["role"] == "detail" else 0.0)
    return shapes


def front(scale):
    """👁️ Front elevation of one scale: the half-lap end of the beam over the near bearer (asset rest metres, x lateral)."""
    return lifted_strokes(front_shapes(scale), 2.5)


def front_shapes(scale):
    """👁️ The shapes of the front elevation before their strokes are lifted."""
    out = [rect("bearer-front", "bearer-right", -BEARER_L / 2, 0.0, BEARER_L, BEARER_H, 1, 0.62, region="bearers", material="softwood")]
    y_tongue, y_mid, y_top = 0.10, YC, YC + HB
    if scale == "500":
        out.append(rect("beam-end", "beam", -B / 2, y_tongue, B, H, 2, 0.4, regions=["beam-faces", "beam-soffit", "cut-end"], material="oak-weathered"))
        return out
    out.append(rect("tongue-end", "beam", -B / 2, y_tongue, B, y_mid - y_tongue, 2, 0.5, regions=["beam-faces", "beam-soffit"], material="oak-weathered"))
    out.append(rect("shoulder-end", "beam", -B / 2, y_mid, B, y_top - y_mid, 2, 0.4, region="beam-faces", material="oak-weathered"))
    if scale == "50":
        out += rings_front("ring", (0.014, 0.214), [0.022, 0.045, 0.068, 0.092, 0.118], (-B / 2, y_tongue, B / 2, y_top))
        out.append(stroke("end-check", "beam", [(0.016, 0.222), (-0.02, 0.27), (-0.045, 0.335)]))
        out.append(stroke("bearer-front-grain", "bearer-right", [(-0.28, 0.035), (0.0, 0.04), (0.28, 0.033)], region="bearers"))
        out.append(stroke("bearer-front-grain-b", "bearer-right", [(-0.28, 0.068), (0.05, 0.064), (0.28, 0.07)], region="bearers"))
    elif scale == "100":
        out += rings_front("ring", (0.014, 0.214), [0.045, 0.092], (-B / 2, y_tongue, B / 2, y_top))
    return out


def plan(scale):
    """👁️ Plan of one scale (asset rest metres, y lateral)."""
    return lifted_strokes(plan_shapes(scale), 2.5)


def plan_shapes(scale):
    """👁️ The shapes of the plan before their strokes are lifted."""
    rnd = Rand({"50": 5, "100": 7, "200": 9, "500": 13}[scale])
    out = []
    for side, xb in zip(("left", "right"), BEARER_X):
        out.append(rect(f"bearer-{side}-plan", f"bearer-{side}", xb - BEARER_W / 2, -BEARER_L / 2, BEARER_W, BEARER_L, 1, 0.62, region="bearers", material="softwood"))
    if scale == "500":
        out.append(rect("beam-top", "beam", 0.0, -B / 2, L, B, 2, 0.4, regions=["beam-faces", "cut-end"], material="oak-weathered"))
        return out
    slant = SLANT if scale == "50" else 0.0
    out.append(rect("beam-top", "beam", slant, -B / 2, LAP_X - slant, B, 2, 0.42, regions=["beam-faces"] + ([] if scale == "50" else ["cut-end"]), material="oak-weathered"))
    lap = [[LAP_X, -B / 2], [L, -B / 2], [L, B / 2], [LAP_X, B / 2]]
    if scale in ("50", "100"):
        n = 24 if scale == "50" else 12
        out.append(path("lap-cheek", "beam", [lap] + [circle(u, w, PEG_R, n) for u, w in TONGUE_PEGS], 2, 0.55, region="beam-faces", material="oak-weathered"))
    else:
        out.append(poly("lap-cheek", "beam", lap, 2, 0.55, region="beam-faces", material="oak-weathered"))
    if scale == "50":
        out.append(rect("cut-end", "beam", 0.0, -B / 2, SLANT, B, 2, 0.8, region="cut-end", material="oak-fresh"))
    for i, (s, e) in enumerate(mortise_spans()):
        out.append(rect(f"mortise-{i + 1}", "beam", s, -MORTISE_W / 2, e - s, MORTISE_W, 3, 0.12, role="detail", region="beam-faces"))
    if scale in ("50", "100"):
        out.append(rect("mortise-brace", "beam", BRACE[0], -MORTISE_W / 2, BRACE[1] - BRACE[0], MORTISE_W, 3, 0.12, role="detail", region="beam-faces"))
        out.append(rect("mortise-brace-slope", "beam", BRACE[1], -MORTISE_W / 2, BRACE[2] - BRACE[1], MORTISE_W, 3, 0.3, role="detail", region="beam-faces"))
    else:
        out.append(rect("mortise-brace", "beam", BRACE[0], -MORTISE_W / 2, BRACE[2] - BRACE[0], MORTISE_W, 3, 0.12, role="detail", region="beam-faces"))
    if scale == "50":
        out.append(stroke("shoulder-line", "beam", [(LAP_X, -B / 2), (LAP_X, B / 2)], role="detail"))
        out.append(check_line("check-top", rnd, 0.55, 3.30, 0.045, 0.003, 18))
        out += hewing_marks("hew-top", rnd, [0.2 + 0.31 * i for i in range(15)], [(-0.085, -0.04), (0.04, 0.08)], 0.06, [(x, 0.0) for x in STUDS])
        out.append(stroke("mark-top-2", "beam", [(1.53, -0.06), (1.53, -0.035)], role="detail"))
        out.append(stroke("mark-top-2b", "beam", [(1.55, -0.06), (1.55, -0.035)], role="detail"))
    elif scale == "100":
        out.append(check_line("check-top", rnd, 0.55, 3.30, 0.045, 0.003, 8))
    return out


def representation(scale):
    """🖼️ The authored representation of one preset scale."""
    pens = {
        "50": {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": 0.13, "ground": 0.5},
        "100": {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": 0.1, "ground": 0.35},
        "200": {"body": 0.18, "detail": 0.1, "texture": None, "glazing": 0.1, "ground": 0.25},
        "500": {"body": 0.13, "detail": None, "texture": None, "glazing": None, "ground": 0.18},
    }[scale]
    ranges = {"50": (20, 75), "100": (75, 150), "200": (150, 350), "500": (350, 1000)}[scale]
    return {
        "status": "authored",
        "denominator": int(scale),
        "validRange": {"minDenominator": ranges[0], "maxDenominator": ranges[1]},
        "pens": pens,
        "hatches": [],
        "geometry": {"elevation": elevation(scale), "front": front(scale), "plan": plan(scale)},
        "decisions": REP_DECISIONS[scale],
    }


REP_DECISIONS = {
    "50": {
        "silhouette": t(
            "Side elevation of the 240 mm deep, 5.20 m long hewn oak beam (4.8 x 104 mm on paper) lying on two 100 x 100 mm softwood bearers: the oblique saw cut of the dismantling at the left end (12 mm out of square), the top half-lap at the right end (300 mm long, 120 mm deep), and the five see-through peg holes of 25 mm through the old mortises. Front view: the lap end, the 200 x 120 mm tongue under the set-back shoulder, with the growth rings of the boxed-heart section and a radial drying check, over the 600 mm bearer. Plan: the hewn top with four stud mortises 50 x 140 mm and the brace mortise with its sloping end, the pale strip of the fresh cut, the lap cheek with its two vertical peg holes, and the bearers projecting 200 mm on each side.",
            "Seitenansicht des 240 mm hohen, 5,20 m langen beilgehauenen Eichenbalkens (4,8 x 104 mm auf Papier) auf zwei Kanthölzern 100 x 100 mm aus Nadelholz: der schräge Trennschnitt des Rückbaus am linken Ende (12 mm aus dem Winkel), das obere Blatt am rechten Ende (300 mm lang, 120 mm tief) und die fünf durchgehenden Holznagellöcher von 25 mm durch die alten Zapfenlöcher. Vorderansicht: das Blattende, die Zunge 200 x 120 mm unter der zurückgesetzten Brust, mit den Jahrringen des Kernholzquerschnitts und einem radialen Schwindriss, über dem 600 mm langen Kantholz. Draufsicht: die gehauene Oberseite mit vier Ständerzapfenlöchern 50 x 140 mm und dem Strebenzapfenloch mit schrägem Ende, dem hellen Streifen des frischen Schnitts, der Blattfläche mit ihren zwei senkrechten Nagellöchern und den Kanthölzern, die beidseitig 200 mm vorstehen."),
        "retained": t(
            "Every trace of use at its true place: the four stud mortises at 1.25 m spacing (x = 0.40, 1.65, 2.90, 4.15 m), the brace mortise at 3.42 to 3.62 m, the five 25 mm peg holes 40 mm below the top (0.5 mm on paper), the two lap pegs, the lap shoulder at 4.90 m, the oblique cut end, axe marks of the broad axe, two drying checks along the grain and the carpenter's mark III beside the second mortise.",
            "Jede Gebrauchsspur an ihrer wahren Stelle: die vier Ständerzapfenlöcher im Abstand von 1,25 m (x = 0,40, 1,65, 2,90, 4,15 m), das Strebenzapfenloch bei 3,42 bis 3,62 m, die fünf Holznagellöcher von 25 mm 40 mm unter der Oberkante (0,5 mm auf Papier), die zwei Blattnägel, die Blattbrust bei 4,90 m, das schräge Schnittende, Hiebspuren des Breitbeils, zwei Schwindrisse längs der Faser und das Abbundzeichen III neben dem zweiten Zapfenloch."),
        "merged": t(
            "The hewn side faces, top and soffit are one beam outline in the side elevation; the lap cheek and the end grain of the tongue are one face in plan and front view.",
            "Die gehauenen Seitenflächen, Oberseite und Unterseite bilden in der Seitenansicht eine Balkenkontur; Blattfläche und Hirnholz der Zunge sind in Draufsicht und Vorderansicht je eine Fläche."),
        "omitted": t(
            "The mortises are hidden in the side elevation (no hidden lines), the horizontal peg holes are not visible in plan and front view, the undulation of the hewn faces (a few millimetres) stays inside the outline pen.",
            "Die Zapfenlöcher sind in der Seitenansicht verdeckt (keine verdeckten Kanten), die waagrechten Nagellöcher sind in Draufsicht und Vorderansicht nicht sichtbar, die Welligkeit der gehauenen Flächen (wenige Millimeter) bleibt innerhalb der Konturfeder."),
        "thinElements": t(
            "Axe marks, drying checks and growth rings are 0.1 mm texture strokes; the carpenter's marks and the lap shoulder in plan are 0.18 mm detail strokes.",
            "Hiebspuren, Schwindrisse und Jahrringe sind Texturstriche von 0,1 mm; Abbundzeichen und Blattbrust in der Draufsicht sind Detailstriche von 0,18 mm."),
        "hatching": t("No hatch: the views are not cuts; the axe marks and rings stand for the surface.", "Keine Schraffur: die Ansichten sind keine Schnitte; Hiebspuren und Jahrringe stehen für die Oberfläche."),
        "styles": t("Outline 0.35 mm, details 0.18 mm, texture 0.1 mm; mortises are dark filled pockets, the fresh cut is the palest tone.", "Kontur 0,35 mm, Details 0,18 mm, Textur 0,1 mm; Zapfenlöcher sind dunkel gefüllte Taschen, der frische Schnitt ist der hellste Ton."),
        "overlays": t("The beam, each bearer and each mortise are their own cells; the peg holes are holes of the beam cell.", "Balken, jedes Kantholz und jedes Zapfenloch sind eigene Zellen; die Nagellöcher sind Löcher der Balkenzelle."),
    },
    "100": {
        "silhouette": t(
            "The beam (2.4 x 52 mm) on its two bearers with the half-lap step at the right end and the five peg holes as 0.25 mm dots in the side elevation. Front view: tongue and shoulder of the lap end over the bearer with two growth rings. Plan: the hewn top with the five mortises as dark pockets (0.5 x 1.4 mm), the lap cheek with its two peg holes and the bearers.",
            "Der Balken (2,4 x 52 mm) auf seinen zwei Kanthölzern mit der Blattstufe am rechten Ende und den fünf Nagellöchern als Punkte von 0,25 mm in der Seitenansicht. Vorderansicht: Zunge und Brust des Blattendes über dem Kantholz mit zwei Jahrringen. Draufsicht: die gehauene Oberseite mit den fünf Zapfenlöchern als dunkle Taschen (0,5 x 1,4 mm), der Blattfläche mit ihren zwei Nagellöchern und den Kanthölzern."),
        "retained": t("Mortises, peg holes, lap and bearers at their true places and sizes; a sparser row of axe marks and one drying check.", "Zapfenlöcher, Nagellöcher, Blatt und Kanthölzer an ihrer wahren Stelle und Größe; eine lockerere Reihe von Hiebspuren und ein Schwindriss."),
        "merged": t("The oblique cut end (12 mm, 0.12 mm on paper) is drawn square and its fresh face is merged into the beam cell.", "Das schräge Schnittende (12 mm, 0,12 mm auf Papier) ist rechtwinklig gezeichnet und seine frische Fläche in die Balkenzelle aufgenommen."),
        "omitted": t("The carpenter's marks, the outer growth rings, the grain of the bearers and the lap shoulder line in plan.", "Die Abbundzeichen, die äußeren Jahrringe, die Faser der Kanthölzer und die Linie der Blattbrust in der Draufsicht."),
        "thinElements": t("Axe marks and the drying check are 0.1 mm texture strokes.", "Hiebspuren und Schwindriss sind Texturstriche von 0,1 mm."),
        "hatching": t("No hatch.", "Keine Schraffur."),
        "styles": t("Outline 0.25 mm, details 0.13 mm, texture 0.1 mm.", "Kontur 0,25 mm, Details 0,13 mm, Textur 0,1 mm."),
        "overlays": t("Beam, bearers and mortises are their own cells.", "Balken, Kanthölzer und Zapfenlöcher sind eigene Zellen."),
    },
    "200": {
        "silhouette": t(
            "The beam as one outline (1.2 x 26 mm) with the half-lap step (1.5 x 0.6 mm) on two bearers (0.5 x 0.5 mm). Front view: tongue and shoulder over the bearer. Plan: the top with the five mortises (0.25 x 0.7 mm) and the lap cheek.",
            "Der Balken als eine Kontur (1,2 x 26 mm) mit der Blattstufe (1,5 x 0,6 mm) auf zwei Kanthölzern (0,5 x 0,5 mm). Vorderansicht: Zunge und Brust über dem Kantholz. Draufsicht: die Oberseite mit den fünf Zapfenlöchern (0,25 x 0,7 mm) und der Blattfläche."),
        "retained": t("Lap, mortises, bearers and one drying check as a detail line.", "Blatt, Zapfenlöcher, Kanthölzer und ein Schwindriss als Detaillinie."),
        "merged": t("The sloping end of the brace mortise is part of one pocket; the cut end is square.", "Das schräge Ende des Strebenzapfenlochs gehört zu einer Tasche; das Schnittende ist rechtwinklig."),
        "omitted": t("Peg holes (25 mm, 0.125 mm on paper), axe marks and growth rings.", "Nagellöcher (25 mm, 0,125 mm auf Papier), Hiebspuren und Jahrringe."),
        "thinElements": t("The drying check is a 0.1 mm detail line.", "Der Schwindriss ist eine Detaillinie von 0,1 mm."),
        "hatching": t("No hatch.", "Keine Schraffur."),
        "styles": t("Outline 0.18 mm, details 0.1 mm.", "Kontur 0,18 mm, Details 0,1 mm."),
        "overlays": t("Beam, bearers and mortises are their own cells.", "Balken, Kanthölzer und Zapfenlöcher sind eigene Zellen."),
    },
    "500": {
        "silhouette": t(
            "The beam as one outline (0.48 x 10.4 mm) with the half-lap step at the right end (0.6 x 0.24 mm) on two bearers (0.2 x 0.2 mm). Front view: the end of the beam over the bearer. Plan: the beam top over the two bearers.",
            "Der Balken als eine Kontur (0,48 x 10,4 mm) mit der Blattstufe am rechten Ende (0,6 x 0,24 mm) auf zwei Kanthölzern (0,2 x 0,2 mm). Vorderansicht: das Balkenende über dem Kantholz. Draufsicht: die Balkenoberseite über den zwei Kanthölzern."),
        "retained": t("Length, depth, the half-lap step and the bearers with their 100 mm clearance under the beam, which is what a sling or a fork passes through.", "Länge, Höhe, die Blattstufe und die Kanthölzer mit ihren 100 mm Abstand unter dem Balken, durch den ein Gurt oder eine Gabel greift."),
        "merged": t("All faces of the beam are one cell; in front view tongue and shoulder are one end face.", "Alle Balkenflächen sind eine Zelle; in der Vorderansicht sind Zunge und Brust eine Stirnfläche."),
        "omitted": t("Mortises (50 mm, 0.1 mm on paper), peg holes, checks and marks.", "Zapfenlöcher (50 mm, 0,1 mm auf Papier), Nagellöcher, Risse und Zeichen."),
        "thinElements": t("None: the bearers stay 0.2 mm squares, closed by the 0.13 mm outline pen.", "Keine: die Kanthölzer bleiben Quadrate von 0,2 mm, geschlossen durch die Konturfeder von 0,13 mm."),
        "hatching": t("No hatch.", "Keine Schraffur."),
        "styles": t("Outline 0.13 mm.", "Kontur 0,13 mm."),
        "overlays": t("Beam and bearers are their own cells.", "Balken und Kanthölzer sind eigene Zellen."),
    },
}

VOL_DECISIONS = {
    "50": [
        t("Body: the hewn oak beam is built of three laminae across its width because the kernel has no boolean subtraction: two 75 mm cheeks (elevation extrudes with the five 25 mm peg holes as true through holes, 24-gons) and a 50 mm core whose elevation profile carries the four stud mortises (140 mm long, 75 mm deep) and the brace mortise with its sloping end (60 mm deep), so every mortise is a real open pocket and every peg hole is see-through where it crosses its mortise; the seams of the core lie exactly on the mortise cheeks. The tongue of the top half-lap (300 x 200 x 120 mm) is a plan extrude with its two vertical 25 mm peg holes; its seam under the shoulder is the shoulder line carried down. The left end is the oblique saw cut of the dismantling, 12 mm out of square.",
          "Körper: der gehauene Eichenbalken besteht über die Breite aus drei Lamellen, weil der Kern keine boolesche Subtraktion kennt: zwei Wangen von 75 mm (Ansichtsextrusionen mit den fünf Nagellöchern von 25 mm als echte Durchgangslöcher, 24-Ecke) und ein Kern von 50 mm, dessen Ansichtsprofil die vier Ständerzapfenlöcher (140 mm lang, 75 mm tief) und das Strebenzapfenloch mit schrägem Ende (60 mm tief) trägt, so dass jedes Zapfenloch eine echte offene Tasche und jedes Nagelloch dort durchsichtig ist, wo es sein Zapfenloch kreuzt; die Fugen des Kerns liegen genau auf den Wangen der Zapfenlöcher. Die Zunge des oberen Blatts (300 x 200 x 120 mm) ist eine Grundrissextrusion mit ihren zwei senkrechten Nagellöchern von 25 mm; ihre Fuge unter der Brust ist die nach unten verlängerte Brustlinie. Das linke Ende ist der schräge Trennschnitt des Rückbaus, 12 mm aus dem Winkel."),
        t("Bearers: two 100 x 100 x 600 mm softwood squared timbers on their own bones under the beam at 1.05 and 4.15 m (about a fifth of the length from each end); they stay on the ground when the beam is lifted or reinstalled.",
          "Kanthölzer: zwei Nadelholz-Kanthölzer 100 x 100 x 600 mm auf eigenen Knochen unter dem Balken bei 1,05 und 4,15 m (etwa ein Fünftel der Länge von jedem Ende); sie bleiben am Boden, wenn der Balken angehoben oder wieder eingebaut wird."),
        t("Thermal: the section is shared between the weathered faces (top, sides, mortise walls, lap faces and the end grain of the tongue) and a flat soffit trapezoid, so every internal face lies within 45 degrees of the section normal and is declared a section face; the mortise pockets stay open in the thermal list (their walls count), the peg holes are closed (2 % of the face area). The fresh cut is its own slab of 7.5 to 19.5 mm with its oblique face as the only counted face.",
          "Thermisch: der Querschnitt ist auf die verwitterten Flächen (Oberseite, Seiten, Zapfenlochwände, Blattflächen und Hirnholz der Zunge) und ein flaches Trapez der Unterseite aufgeteilt, so dass jede innere Fläche innerhalb von 45 Grad zur Schnittnormalen liegt und als Schnittfläche erklärt ist; die Zapfenlochtaschen bleiben in der thermischen Liste offen (ihre Wände zählen), die Nagellöcher sind geschlossen (2 % der Flächen). Der frische Schnitt ist eine eigene Scheibe von 7,5 bis 19,5 mm, deren schräge Fläche als einzige zählt."),
        t("Structure: the beam member is the true 200 x 240 mm section with the half-lap step, the bearers are compression members along their downward bones, joints of one smallest feature sit at the beam-bearer interfaces and under the load carrier; there are no permanent supports, every pose names its own contacts (bearers on the ground, sling points, wall bearings).",
          "Tragwerk: das Balkenglied ist der wahre Querschnitt 200 x 240 mm mit der Blattstufe, die Kanthölzer sind Druckglieder entlang ihrer nach unten zeigenden Knochen, Gelenke von einer kleinsten Einzelheit sitzen an den Fugen Balken-Kantholz und unter dem Lastträger; es gibt keine ständigen Auflager, jede Pose nennt ihre eigenen Kontakte (Kanthölzer am Boden, Anschlagpunkte, Wandauflager)."),
    ],
    "100": [
        t("Body: the same three laminae and tongue as at 1:50, generalised for the 15 mm feature: the oblique cut (12 mm) is square, the peg holes (25 mm, still above the feature) are 12-gons.",
          "Körper: dieselben drei Lamellen und die Zunge wie bei 1:50, verallgemeinert für die kleinste Einzelheit von 15 mm: der schräge Schnitt (12 mm) ist rechtwinklig, die Nagellöcher (25 mm, noch über der Einzelheit) sind 12-Ecke."),
        t("Thermal: the mortise walls are merged into the top face (pockets closed, 4 % less area, within the 30 % of this scale); the fresh cut is a 15 mm slab.",
          "Thermisch: die Zapfenlochwände sind in die Oberseite aufgenommen (Taschen geschlossen, 4 % weniger Fläche, innerhalb der 30 % dieses Maßstabs); der frische Schnitt ist eine Scheibe von 15 mm."),
        t("Minimal: the structure is already one beam member, two bearer members and three joints; at 1:100 only the oblique end of the member is squared and the joints grow to 15 mm.", "Minimal: das Tragwerk ist bereits ein Balkenglied, zwei Kantholzglieder und drei Gelenke; bei 1:100 wird nur das schräge Ende des Glieds rechtwinklig und die Gelenke wachsen auf 15 mm."),
    ],
    "200": [
        t("Body: two solids for the beam: a lower block with the lap step and an upper 75 mm layer as a plan extrude whose holes are the five mortises (the brace mortise as one 200 mm pocket); the peg holes (25 mm) are below the 30 mm feature and are left out.",
          "Körper: zwei Körper für den Balken: ein unterer Block mit der Blattstufe und eine obere Schicht von 75 mm als Grundrissextrusion, deren Löcher die fünf Zapfenlöcher sind (das Strebenzapfenloch als eine Tasche von 200 mm); die Nagellöcher (25 mm) liegen unter der Einzelheit von 30 mm und entfallen."),
        t("Minimal: the thermal list is already four region solids and the two bearers (as at 1:100, the cut-end slab 30 mm thick); the structure is one beam member with the lap step, two bearer members and three joints of 30 mm.", "Minimal: die thermische Liste besteht bereits aus vier Regionskörpern und den zwei Kanthölzern (wie bei 1:100, die Schnittscheibe 30 mm dick); das Tragwerk ist ein Balkenglied mit der Blattstufe, zwei Kantholzglieder und drei Gelenke von 30 mm."),
    ],
    "500": [
        t("Body: one beam solid with the half-lap step (120 x 300 mm, above the 75 mm feature) and the two bearers (100 mm); mortises and peg holes are below the feature.",
          "Körper: ein Balkenkörper mit der Blattstufe (120 x 300 mm, über der Einzelheit von 75 mm) und die zwei Kanthölzer (100 mm); Zapfenlöcher und Nagellöcher liegen unter der Einzelheit."),
        t("Minimal: the thermal list is already four region solids and the two bearers (cut-end slab 75 mm, a 75 mm deep soffit wedge); the structure is one beam member with the lap step, two bearer members and three joints of 75 mm.", "Minimal: die thermische Liste besteht bereits aus vier Regionskörpern und den zwei Kanthölzern (Schnittscheibe 75 mm, ein 75 mm hoher Keil der Unterseite); das Tragwerk ist ein Balkenglied mit der Blattstufe, zwei Kantholzglieder und drei Gelenke von 75 mm."),
    ],
}


def poly_area(pts):
    """📏 Shoelace area of a polygon."""
    return abs(sum(pts[i][0] * pts[(i + 1) % len(pts)][1] - pts[(i + 1) % len(pts)][0] * pts[i][1] for i in range(len(pts)))) / 2


def beam_volume():
    """⚖️ Net volume of the 1:50 beam solids: cheeks with holes, notched core, tongue with holes."""
    hole = math.pi * PEG_R ** 2
    cheek = poly_area(cheek_profile(SLANT)) - len(PEGS) * hole
    core = poly_area(core_profile(SLANT))
    tongue = (L - LAP_X) * B - 2 * hole
    return 2 * cheek * (B / 2 - MORTISE_W / 2) + core * MORTISE_W + tongue * HB


V_BEAM = beam_volume()
M_BEAM = round(V_BEAM * RHO_OAK, 2)
M_BEARER = round(BEARER_H * BEARER_W * BEARER_L * RHO_SOFT, 2)
M_TOTAL = round(M_BEAM + 2 * M_BEARER, 2)
I_BEAM = B * H ** 3 / 12


def region_binding(faces, soffit, cut, bearers):
    """🌤️ The thermal binding of the four regions."""
    return {"regions": {"beam-faces": faces, "beam-soffit": soffit, "cut-end": cut, "bearers": bearers}, "emitters": {}}


YARD = region_binding({"exposedFraction": 1, "sunlitFraction": 0.65}, {"exposedFraction": 1, "sunlitFraction": 0}, {"exposedFraction": 1, "sunlitFraction": 0.3}, {"exposedFraction": 1, "sunlitFraction": 0.4, "contact": {"fraction": 0.23, "tempC": 18}})
LIFTED = region_binding({"exposedFraction": 1, "sunlitFraction": 0.65}, {"exposedFraction": 1, "sunlitFraction": 0.05}, {"exposedFraction": 1, "sunlitFraction": 0.3}, {"exposedFraction": 1, "sunlitFraction": 0.4, "contact": {"fraction": 0.23, "tempC": 18}})
INDOOR = region_binding({"exposedFraction": 1, "sunlitFraction": 0, "contact": {"fraction": 0.31, "tempC": 20}}, {"exposedFraction": 1, "sunlitFraction": 0}, {"exposedFraction": 1, "sunlitFraction": 0, "contact": {"fraction": 1, "tempC": 16}}, {"exposedFraction": 1, "sunlitFraction": 0.4, "contact": {"fraction": 0.23, "tempC": 18}})


def bearers_down(dy):
    """⬇️ Offsets that keep both bearers on the ground while the beam above them rises by `dy`."""
    return {"bearer-left": {"dy": -dy}, "bearer-right": {"dy": -dy}}


DEAD = {"id": "floor-dead", "kind": "imposed", "title": t("Floor dead load 0.96 kN/m (1.2 kN/m² x 0.80 m)", "Ausbaulast 0,96 kN/m (1,2 kN/m² x 0,80 m)"), "fxN": 0, "fyN": 0, "distributed": {"bone": "beam", "fromM": 0, "toM": L, "wxNPerM": 0, "wyNPerM": -G_FLOOR * SPACING * 1000}, "source": "authored-estimate", "reference": "en-1991-1-1"}
LIVE = {"id": "floor-imposed", "kind": "live", "title": t("Imposed floor load 1.6 kN/m (2.0 kN/m² x 0.80 m)", "Nutzlast 1,6 kN/m (2,0 kN/m² x 0,80 m)"), "fxN": 0, "fyN": 0, "distributed": {"bone": "beam", "fromM": 0, "toM": L, "wxNPerM": 0, "wyNPerM": -Q_FLOOR * SPACING * 1000}, "source": "literature", "reference": "en-1991-1-1"}
POINT = {"id": "point-load", "kind": "live", "title": t("Concentrated load 2 kN", "Einzellast 2 kN"), "anchor": "load-point", "fxN": 0, "fyN": -2000, "source": "literature", "reference": "en-1991-1-1"}

POSES = [
    {"id": "stored", "title": t("Stored on bearers", "Gelagert auf Kanthölzern"), "description": t("The reclaimed beam lies in the yard of the component exchange on two squared-timber bearers, mortise face up, under its own weight.", "Der gebrauchte Balken liegt im Lager der Bauteilbörse auf zwei Kanthölzern, die Zapfenlochseite oben, unter Eigengewicht."), "bones": {}, "ground": "authored", "contacts": ["ground-left", "ground-right"], "thermal": YARD, "loads": []},
    {"id": "lifted", "title": t("Lifted on a two-leg sling", "Angehoben am Zweistranggehänge"), "description": t("The beam hangs 1.0 m above the bearers from two round slings choked at 1.20 and 4.00 m, the legs 45 degrees from the vertical to the crane hook; the bearers stay on the ground.", "Der Balken hängt 1,0 m über den Kanthölzern an zwei geschnürten Rundschlingen bei 1,20 und 4,00 m, die Stränge 45 Grad gegen die Senkrechte zum Kranhaken; die Kanthölzer bleiben am Boden."), "bones": bearers_down(LIFT), "root": {"y": LIFT}, "ground": "authored", "contacts": ["sling-left", "sling-right"], "thermal": LIFTED, "loads": []},
    {"id": "reinstalled", "title": t("Reinstalled as a ceiling beam", "Wieder eingebaut als Deckenbalken"), "description": t("Back in service as a simply supported ceiling beam with its soffit at 2.40 m, 200 mm bearing in the walls at each end (span 5.00 m), carrying a floor of 1.2 kN/m² dead and 2.0 kN/m² imposed load over 0.80 m beam spacing.", "Wieder im Dienst als frei aufliegender Deckenbalken mit Unterkante bei 2,40 m, je 200 mm Auflager in den Wänden (Stützweite 5,00 m), mit einer Decke von 1,2 kN/m² Ausbau- und 2,0 kN/m² Nutzlast bei 0,80 m Balkenabstand."), "bones": bearers_down(REINSTALL), "root": {"y": REINSTALL}, "ground": "authored", "contacts": ["bearing-left", "bearing-right"], "thermal": INDOOR, "loads": [DEAD, LIVE]},
    {"id": "reinstalled-point-load", "title": t("Reinstalled, point load at midspan", "Wieder eingebaut, Einzellast in Feldmitte"), "description": t("The reinstalled ceiling beam under its floor dead load and a 2 kN concentrated load at midspan.", "Der wieder eingebaute Deckenbalken unter Ausbaulast und einer Einzellast von 2 kN in Feldmitte."), "bones": {**bearers_down(REINSTALL), "load-carrier": {"dx": 2.5}}, "root": {"y": REINSTALL}, "ground": "authored", "contacts": ["bearing-left", "bearing-right"], "thermal": INDOOR, "loads": [DEAD, POINT]},
]

CLIPS = [
    {"id": "lift", "title": t("Lift off the bearers", "Abheben von den Kanthölzern"), "description": t("The crane takes up the two-leg sling and raises the beam 1.0 m clear of its bearers, which stay where they lie.", "Der Kran strafft das Zweistranggehänge und hebt den Balken 1,0 m über seine Kanthölzer, die liegen bleiben."), "durationS": 6, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 6, "pose": "lifted", "ease": "smooth"}],
     "contacts": [{"anchor": "ground-left", "fromS": 0, "toS": 0.3}, {"anchor": "ground-right", "fromS": 0, "toS": 0.3}, {"anchor": "sling-left", "fromS": 0.3, "toS": 6}, {"anchor": "sling-right", "fromS": 0.3, "toS": 6}],
     "phases": [{"id": "take-up", "fromS": 0, "toS": 0.3, "title": t("Sling takes the load", "Gehänge übernimmt die Last")}, {"id": "hoist", "fromS": 0.3, "toS": 6, "title": t("Hoist", "Heben")}],
     "events": [{"t": 0.3, "kind": "contact-off", "title": t("Beam leaves the bearers", "Balken verlässt die Kanthölzer"), "target": "ground-left"}, {"t": 6, "kind": "motion", "title": t("Beam hangs 1.0 m clear", "Balken hängt 1,0 m frei")}], "tracks": []},
    {"id": "moving-point-load", "title": t("Reinstalled, moving point load", "Wieder eingebaut, wandernde Einzellast"), "description": t("In the reinstalled ceiling the 2 kN load walks from the left bearing to the right bearing over the floor dead load.", "In der wieder eingebauten Decke wandert die Einzellast von 2 kN über der Ausbaulast vom linken zum rechten Auflager."), "durationS": 6, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "reinstalled-point-load", "ease": "smooth", "bones": {"load-carrier": {"dx": 0}}}, {"t": 6, "pose": "reinstalled-point-load", "ease": "smooth", "bones": {"load-carrier": {"dx": 5.0}}}],
     "contacts": [{"anchor": "bearing-left", "fromS": 0, "toS": 6}, {"anchor": "bearing-right", "fromS": 0, "toS": 6}],
     "phases": [{"id": "walk", "fromS": 0, "toS": 6, "title": t("Walk across", "Überqueren")}],
     "events": [{"t": 0, "kind": "motion", "title": t("Load at the left bearing", "Last am linken Auflager")}, {"t": 3, "kind": "motion", "title": t("Load at midspan", "Last in Feldmitte")}], "tracks": []},
]

REFERENCES = [
    {"id": "en-338", "citation": "EN 338:2016 Structural timber, strength classes, Table 1 (C24: fm,k 24 N/mm², E0,mean 11.0 kN/mm², rho mean 420 kg/m³) and Table 2 (D30: fm,k 30 N/mm², E0,mean 11.0 kN/mm², rho k 530 kg/m³)."},
    {"id": "din-4074-5", "citation": "DIN 4074-5:2008 Strength grading of wood, part 5: sawn hardwood (visual grade LS 10); EN 1912:2012 assigns LS 10 oak to D30. Reclaimed timber has to be regraded before reuse."},
    {"id": "din-68364", "citation": "DIN 68364:2003-05 Properties of wood species, density, modulus of elasticity and strength: European oak (Quercus robur, Q. petraea) mean density 690 kg/m³ at 12 % moisture content."},
    {"id": "en-1991-1-1", "citation": "EN 1991-1-1:2002 Eurocode 1, Part 1-1, Table 6.1 and 6.2 (category A floors 2.0 kN/m², concentrated load 2.0 kN) and Annex A (densities of building materials for the floor dead load)."},
    {"id": "en-1995-1-1", "citation": "EN 1995-1-1:2004 Eurocode 5, 6.1.5 and 7.2 (bearing and deflection of timber beams; effective span between the centres of the bearings)."},
    {"id": "en-iso-10456", "citation": "EN ISO 10456:2007 Building materials and products, hygrothermal properties, Table 3: timber 700 kg/m³ lambda 0.18 W/mK, timber 450 kg/m³ lambda 0.12 to 0.13 W/mK, specific heat 1600 J/kgK."},
    {"id": "en-iso-13786", "citation": "EN ISO 13786:2017 Thermal performance of building components, dynamic thermal characteristics: periodic penetration depth delta = sqrt(lambda T / (pi rho c)); half of it is taken as the effective storage depth of an exposed face."},
    {"id": "emissivity-literature", "citation": "ASHRAE Handbook Fundamentals (2021), chapter 4, thermal radiation properties of building materials (wood emissivity about 0.9; solar absorptance of weathered and fresh timber taken as rounded authored estimates guided by this chapter)."},
    {"id": "gerner-fachwerk", "citation": "Gerner, M.: Fachwerk. Entwicklung, Gefüge, Instandsetzung. Deutsche Verlags-Anstalt, Stuttgart 2007: oak sections of plates and beams 18/20 to 22/26 cm, tenons one third of the member width, oak pegs 25 to 30 mm, lap joints of half the member depth."},
    {"id": "dguv-209-013", "citation": "DGUV Information 209-013 Anschläger (2021) and EN 1492-2:2000+A1:2008 round slings: inclination angle of a two-leg sling at most 60 degrees from the vertical, choke hitch."},
    {"id": "din-iso-128-50", "citation": "DIN ISO 128-50:2002 Technical drawings, general principles of presentation, part 50: basic conventions for areas on sections and cuts (hatching)."},
    {"id": "din-1356-1", "citation": "DIN 1356-1:1995 Building drawings, part 1: types of drawings, contents and basic principles of presentation (scales, line groups)."},
]

MATERIALS = [
    {"id": "oak-weathered", "title": t("Old oak, weathered hewn faces (D30)", "Alteiche, verwitterte gehauene Flächen (D30)"), "emissivity": 0.9, "solarAbsorptivity": 0.72, "densityKgM3": RHO_OAK, "conductivityWmK": 0.18, "specificHeatJKgK": 1600, "elasticModulusPa": 11.0e9, "strengthPa": 30.0e6, "source": "literature", "reference": "din-68364"},
    {"id": "oak-fresh", "title": t("Oak, fresh saw cut of the dismantling", "Eiche, frischer Trennschnitt des Rückbaus"), "emissivity": 0.9, "solarAbsorptivity": 0.5, "densityKgM3": RHO_OAK, "conductivityWmK": 0.18, "specificHeatJKgK": 1600, "elasticModulusPa": 11.0e9, "strengthPa": 30.0e6, "source": "authored-estimate", "reference": "emissivity-literature"},
    {"id": "softwood", "title": t("Softwood bearer C24, weathered", "Nadelholz-Kantholz C24, verwittert"), "emissivity": 0.9, "solarAbsorptivity": 0.65, "densityKgM3": RHO_SOFT, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11.0e9, "strengthPa": 24.0e6, "source": "literature", "reference": "en-338"},
]


def area_hand():
    """📐 Hand areas of the real object per region (m²)."""
    sides = 2 * (LAP_X * H + (L - LAP_X) * HB - len(PEGS) * math.pi * PEG_R ** 2)
    top = (LAP_X - SLANT) * B
    stud = 2 * MORTISE_L * 0.075 + 2 * MORTISE_W * 0.075
    brace_trap = ((BRACE[2] - BRACE[0]) + (BRACE[1] - BRACE[0])) / 2 * (HB - BRACE[3])
    brace = 2 * brace_trap + MORTISE_W * (HB - BRACE[3]) + MORTISE_W * math.hypot(BRACE[2] - BRACE[1], HB - BRACE[3]) + MORTISE_W * (BRACE[1] - BRACE[0]) - MORTISE_W * (BRACE[2] - BRACE[0])
    lap = (L - LAP_X) * B + 2 * B * HB
    faces = sides + top + len(STUDS) * stud + brace + lap
    soffit = L * B
    cut = B * math.hypot(H, SLANT)
    bearers = 2 * (2 * BEARER_H * BEARER_W + 2 * BEARER_H * BEARER_L + 2 * BEARER_W * BEARER_L)
    return {"beam-faces": faces, "beam-soffit": soffit, "cut-end": cut, "bearers": bearers}


AREAS = area_hand()


def regions():
    """🌡️ The four thermal regions."""
    return [
        {"id": "beam-faces", "title": t("Weathered faces of the beam (top, sides, mortise walls, lap faces)", "Verwitterte Balkenflächen (Oberseite, Seiten, Zapfenlochwände, Blattflächen)"), "kind": "surface", "model": "passive", "material": "oak-weathered", "areaM2": round(AREAS["beam-faces"], 3), "characteristicLengthM": 0.24, "geometry": "vertical-plate", "skyViewFactor": 0.6, "layers": [], "heatCapacityJM2K": 37000, "source": "authored-estimate", "reference": "en-iso-13786"},
        {"id": "beam-soffit", "title": t("Underside of the beam", "Balkenunterseite"), "kind": "surface", "model": "passive", "material": "oak-weathered", "areaM2": round(AREAS["beam-soffit"], 3), "characteristicLengthM": 0.2, "geometry": "horizontal-plate-down", "skyViewFactor": 0.05, "layers": [], "heatCapacityJM2K": 37000, "source": "authored-estimate", "reference": "en-iso-13786"},
        {"id": "cut-end", "title": t("Fresh saw cut of the dismantling", "Frischer Trennschnitt des Rückbaus"), "kind": "surface", "model": "passive", "material": "oak-fresh", "areaM2": round(AREAS["cut-end"], 4), "characteristicLengthM": 0.24, "geometry": "vertical-plate", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": 37000, "source": "authored-estimate", "reference": "en-iso-13786"},
        {"id": "bearers", "title": t("Softwood bearers", "Kanthölzer aus Nadelholz"), "kind": "surface", "model": "passive", "material": "softwood", "areaM2": round(AREAS["bearers"], 3), "characteristicLengthM": 0.1, "geometry": "vertical-plate", "skyViewFactor": 0.4, "layers": [], "heatCapacityJM2K": 24500, "source": "authored-estimate", "reference": "en-iso-13786"},
    ]


def anchors():
    """🪝 Contacts on the ground, sling points, wall bearings and the load point."""
    out = []
    for side in ("left", "right"):
        out.append({"id": f"ground-{side}", "bone": f"bearer-{side}", "x": BEARER_H, "y": 0, "kind": "contact", "title": t(f"Bearer {side} on the ground", "Kantholz " + ("links" if side == "left" else "rechts") + " auf dem Boden"), "support": "contact", "footprintLengthM": BEARER_W, "footprintWidthM": BEARER_L, "frictionCoefficient": 0.5})
    out.append({"id": "sling-left", "bone": "beam", "x": SLINGS[0], "y": HB, "kind": "support", "title": t("Left sling point (choke hitch, leg 45 degrees)", "Linker Anschlagpunkt (Schnürgang, Strang 45 Grad)"), "support": "roller", "normalDeg": 45})
    out.append({"id": "sling-right", "bone": "beam", "x": SLINGS[1], "y": HB, "kind": "support", "title": t("Right sling point (choke hitch, leg 45 degrees)", "Rechter Anschlagpunkt (Schnürgang, Strang 45 Grad)"), "support": "roller", "normalDeg": 135})
    out.append({"id": "bearing-left", "bone": "beam", "x": BEARINGS[0], "y": -HB, "kind": "support", "title": t("Left wall bearing, pin", "Linkes Wandauflager, Festlager"), "support": "pin", "footprintLengthM": 0.2, "footprintWidthM": B})
    out.append({"id": "bearing-right", "bone": "beam", "x": BEARINGS[1], "y": -HB, "kind": "support", "title": t("Right wall bearing, roller", "Rechtes Wandauflager, Rollenlager"), "support": "roller", "normalDeg": 90, "footprintLengthM": 0.2, "footprintWidthM": B})
    out.append({"id": "load-point", "bone": "load-carrier", "x": 0, "y": 0, "kind": "load", "title": t("Concentrated load", "Einzellast")})
    return out


def rig():
    """🦴 Beam as root, two bearers that can stay behind, a load carrier."""
    bearers = []
    for side, xb in zip(("left", "right"), BEARER_X):
        bearers.append({"id": f"bearer-{side}", "parent": "beam", "title": t(f"Bearer {side} (stays on the ground when the beam is lifted)", "Kantholz " + ("links" if side == "left" else "rechts") + " (bleibt am Boden, wenn der Balken angehoben wird)"), "x": xb, "y": -HB, "angleDeg": -90, "lengthM": BEARER_H, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -3.0, "maxM": 0}, "mass": {"kg": M_BEARER, "comX": BEARER_H / 2, "comY": 0, "distribution": "uniform", "source": "literature", "reference": "en-338"}, "section": {"areaM2": round(BEARER_W * BEARER_L, 4), "secondMomentM4": round(BEARER_L * BEARER_W ** 3 / 12, 8), "extremeFibreM": BEARER_W / 2, "material": "softwood", "title": t("C24 100 x 100 mm, 600 mm long", "C24 100 x 100 mm, 600 mm lang")}})
    return {"kind": "structural", "root": "beam", "bones": [
        {"id": "beam", "parent": None, "title": t("Reclaimed oak beam", "Gebrauchter Eichenbalken"), "x": 0, "y": YC, "angleDeg": 0, "lengthM": L, "joint": {"type": "fixed"}, "mass": {"kg": M_BEAM, "comX": L / 2, "comY": 0, "distribution": "uniform", "source": "literature", "reference": "din-68364"}, "section": {"areaM2": round(B * H, 4), "secondMomentM4": round(I_BEAM, 9), "extremeFibreM": HB, "material": "oak-weathered", "title": t("Oak D30 200 x 240 mm", "Eiche D30 200 x 240 mm")}},
        *bearers,
        {"id": "load-carrier", "parent": "beam", "title": t("Load carrier (massless, moves the point load)", "Lastträger (masselos, bewegt die Einzellast)"), "x": BEARINGS[0], "y": HB, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": BEARINGS[1] - BEARINGS[0]}},
    ]}


def document():
    """📄 The whole asset document."""
    asset = {
        "id": "reclaimed-oak-beam",
        "category": "components",
        "title": t("Reclaimed Oak Beam", "Gebrauchter Eichenbalken"),
        "summary": t(
            "A hewn oak beam of 200 x 240 mm and 5.20 m from a dismantled half-timbered house, as a component exchange stores it on two squared-timber bearers: four stud mortises and a brace mortise on its top with their 25 mm peg holes, a half-lap at one end and the oblique saw cut of the dismantling at the other; lifted on a two-leg sling and reinstalled as a simply supported ceiling beam of 5.00 m span.",
            "Ein beilgehauener Eichenbalken 200 x 240 mm, 5,20 m lang, aus einem rückgebauten Fachwerkhaus, wie ihn eine Bauteilbörse auf zwei Kanthölzern lagert: vier Ständerzapfenlöcher und ein Strebenzapfenloch auf der Oberseite mit ihren Holznagellöchern von 25 mm, ein Blatt am einen Ende und der schräge Trennschnitt des Rückbaus am anderen; angehoben am Zweistranggehänge und wieder eingebaut als frei aufliegender Deckenbalken mit 5,00 m Stützweite."),
        "tags": ["component", "reclaimed", "timber", "oak", "hardwood", "beam", "half-timbered", "structural"],
        "provenance": {"author": "Semio staffage component authors", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES, "notes": t(
            "Section 200 x 240 mm, length 5.20 m, net volume %.4f m³ after mortises, peg holes, lap and cut, mass %.1f kg at 690 kg/m³; bearers 100 x 100 x 600 mm C24, %.2f kg each. Reinstalled span 5.00 m between the centres of the 200 mm wall bearings, beam spacing 0.80 m. All trace positions are authored estimates of a typical plate (Rähm) of a 17th to 19th century timber frame." % (V_BEAM, M_BEAM, M_BEARER),
            "Querschnitt 200 x 240 mm, Länge 5,20 m, Nettovolumen %s m³ nach Zapfenlöchern, Nagellöchern, Blatt und Schnitt, Masse %s kg bei 690 kg/m³; Kanthölzer 100 x 100 x 600 mm C24, je %s kg. Stützweite im Wiedereinbau 5,00 m zwischen den Mitten der 200-mm-Wandauflager, Balkenabstand 0,80 m. Alle Lagen der Spuren sind Schätzungen eines typischen Rähms eines Fachwerks des 17. bis 19. Jahrhunderts." % (f"{V_BEAM:.4f}".replace(".", ","), f"{M_BEAM:.1f}".replace(".", ","), f"{M_BEARER:.2f}".replace(".", ",")))},
        "dimensions": {"heightM": round(0.10 + H, 3), "widthM": L, "depthM": BEARER_L, "massKg": M_TOTAL},
        "materials": MATERIALS,
        "rig": rig(),
        "anchors": anchors(),
        "representations": {s: representation(s) for s in ("50", "100", "200", "500")},
        "symbols": [],
        "poses": POSES,
        "clips": CLIPS,
        "thermal": {"environment": {"airTempC": 20, "meanRadiantTempC": 20, "skyTempC": 8, "groundTempC": 18, "relativeHumidity": 0.5, "windMS": 1, "windDirectionDeg": 0, "solarWM2": 500, "diffuseFraction": 0.2, "pressurePa": 101325}, "operating": [], "regions": regions(), "emitters": [], "assumptions": [
            t("Passive oak faces store heat in an effective depth of about 34 mm (half the 24 h penetration depth of 67 mm for lambda 0.18 W/mK, rho 690 kg/m³, c 1600 J/kgK, EN ISO 13786), softwood bearers in about 36 mm.", "Passive Eichenflächen speichern Wärme in einer wirksamen Tiefe von etwa 34 mm (die Hälfte der 24-h-Eindringtiefe von 67 mm für Lambda 0,18 W/mK, Rho 690 kg/m³, c 1600 J/kgK, EN ISO 13786), die Kanthölzer in etwa 36 mm."),
            t("Weathered and soiled old oak absorbs more sunlight (0.72) than the bright fresh saw cut of the dismantling (0.50); both emit 0.9. Values are authored estimates.", "Verwitterte und verschmutzte Alteiche absorbiert mehr Sonnenlicht (0,72) als der helle frische Trennschnitt des Rückbaus (0,50); beide emittieren 0,9. Die Werte sind Schätzungen."),
            t("Region areas are the total outer areas of the real beam: the weathered region holds top, sides, the walls of the five mortises, the lap cheek, the lap shoulder and the end grain of the tongue; the bores of the seven peg holes (about 0.08 m²) are left out.", "Die Flächen der Regionen sind die gesamten Außenflächen des realen Balkens: die verwitterte Region umfasst Oberseite, Seiten, die Wände der fünf Zapfenlöcher, Blattfläche, Blattbrust und das Hirnholz der Zunge; die Bohrungen der sieben Nagellöcher (etwa 0,08 m²) sind nicht enthalten."),
            t("Stored and lifted the beam stands in the open yard on a mild sunny day; reinstalled it is indoors without sun, its top partly under the floor boards and its cut end in the wall pocket.", "Gelagert und angehoben liegt der Balken im offenen Lager an einem milden sonnigen Tag; wieder eingebaut ist er innen ohne Sonne, seine Oberseite teilweise unter den Dielen und sein Schnittende in der Wandtasche."),
        ], "passengers": []},
        "structure": {"gravityMS2": 9.81, "dynamics": "quasi-static", "supports": [], "assumptions": [
            t("Old oak graded D30 (DIN 4074-5 LS 10, EN 1912): E0,mean 11.0 GPa, fm,k 30 MPa; gross section 200 x 240 mm, A 0.048 m², I 2.304e-4 m⁴. The section loss at a mortise (50 x 75 mm, 8 %) and at the half-lap (half the depth over 300 mm, inside the wall bearing) is stated, not modelled in the stress.", "Alteiche sortiert als D30 (DIN 4074-5 LS 10, EN 1912): E0,mean 11,0 GPa, fm,k 30 MPa; Bruttoquerschnitt 200 x 240 mm, A 0,048 m², I 2,304e-4 m⁴. Die Schwächung an einem Zapfenloch (50 x 75 mm, 8 %) und am Blatt (halbe Höhe über 300 mm, im Wandauflager) ist angegeben, aber in der Spannung nicht abgebildet."),
            t("Each pose names its own contacts: stored, the beam bears through the bearers on the ground; lifted, it hangs from two sling points whose legs lean 45 degrees, so the beam carries 0.83 kN compression between them; reinstalled, it is a simply supported beam (pin left, roller right) of 5.00 m span.", "Jede Pose nennt ihre eigenen Kontakte: gelagert trägt der Balken über die Kanthölzer auf den Boden ab; angehoben hängt er an zwei Anschlagpunkten, deren Stränge 45 Grad geneigt sind, so dass der Balken dazwischen 0,83 kN Druck erhält; wieder eingebaut ist er ein frei aufliegender Einfeldträger (Festlager links, Rollenlager rechts) mit 5,00 m Stützweite."),
            t("The whole asset is one rigid body for the reactions: when the beam is lifted or reinstalled, the two bearers (5.0 kg, 3 % of the mass) left on the ground are carried by the active supports through the rig; they add 25 N to each reaction.", "Das ganze Asset ist für die Reaktionen ein starrer Körper: ist der Balken angehoben oder eingebaut, werden die zwei am Boden gebliebenen Kanthölzer (5,0 kg, 3 % der Masse) über das Skelett von den aktiven Auflagern getragen; sie erhöhen jede Reaktion um 25 N."),
            t("Shear and axial deformation are neglected in the deflection.", "Schub- und Längsverformung werden in der Durchbiegung vernachlässigt."),
        ], "clinical": False},
        "volumes": {s: {"status": "authored", "denominator": int(s), "body": body(s), "thermal": thermal(s), "structure": structure(s), "decisions": VOL_DECISIONS[s]} for s in ("50", "100", "200", "500")},
        "presentation": {"defaultPose": "stored", "defaultClip": "lift", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}


if __name__ == "__main__":
    doc = document()
    (HERE / "🔣️.json").write_text(json.dumps(doc, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    span = BEARINGS[1] - BEARINGS[0]
    w = (G_FLOOR + Q_FLOOR) * SPACING * 1000 + M_BEAM * 9.81 / L
    m = w * span ** 2 / 8
    print(f"[DEBUG] beam volume {V_BEAM:.5f} m3, mass {M_BEAM} kg; bearers {M_BEARER} kg each; total {M_TOTAL} kg")
    print(f"[DEBUG] areas " + ", ".join(f"{k} {v:.4f}" for k, v in AREAS.items()))
    print(f"[DEBUG] reinstalled w {w:.0f} N/m, M {m / 1000:.2f} kNm, sigma {m / (I_BEAM / HB) / 1e6:.2f} MPa, deflection {5 * w * span ** 4 / (384 * 11e9 * I_BEAM) * 1000:.1f} mm")
