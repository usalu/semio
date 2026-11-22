"""🍻 Authoritative author script of the staffage asset `used-beer-garden-set`: states every hand-chosen number and writes `🔣️.json` next to it.

Run: `python3 author-beer-garden-set.py` (writes the candidate document; the measured report numbers come from `author-measure.py`).

@see ./author-geometry.py — rest frames, prism parts, solids and shapes
@see ../../brief.md — the brief of the wave
"""
import json
import math
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import importlib

geo = importlib.import_module("author-geometry")
part, rect, shape, body_solid, rest_frames, FEATURE = geo.part, geo.rect, geo.shape, geo.body_solid, geo.rest_frames, geo.FEATURE
r6 = geo.r6

HERE = os.path.dirname(os.path.abspath(__file__))


def T(en, de):
    """🗣 A bilingual text."""
    return {"en": en, "de": de}


L_TOP = 1.10
TABLE_Y = 0.63
TABLE_TOP = 0.770
TABLE_HINGE = 0.715
T_ZT = 0.80
BENCH_HINGE = 0.435
B_ZT = 0.85
B_IN = 0.305
B_W = 0.25
T_PIVOT = (0.730, 0.535, 0.027, 0.785)
B_PIVOT = (0.450, 0.635, 0.015, 0.835)
THETA_T = math.degrees(math.atan2(T_PIVOT[3] - T_PIVOT[1], T_PIVOT[0] - T_PIVOT[2]))
L_BRACE_T = math.hypot(T_PIVOT[3] - T_PIVOT[1], T_PIVOT[0] - T_PIVOT[2])
THETA_B = math.degrees(math.atan2(B_PIVOT[3] - B_PIVOT[1], B_PIVOT[0] - B_PIVOT[2]))
L_BRACE_B = math.hypot(B_PIVOT[3] - B_PIVOT[1], B_PIVOT[0] - B_PIVOT[2])

BONES = [
    {"id": "ground", "parent": None, "title": T("Ground frame", "Bezugsrahmen"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"}},
    {"id": "table-lift", "parent": "ground", "title": T("Table lift (carried by two workers)", "Tischhub (von zwei Arbeitern getragen)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -0.70, "maxM": 0.65}},
    {"id": "table", "parent": "table-lift", "title": T("Table top with battens, hinge housings and catches", "Tischplatte mit Leisten, Klappbeschlägen und Rasten"), "x": 0, "y": TABLE_Y, "angleDeg": 0, "lengthM": 0.25, "joint": {"type": "revolute", "minDeg": -180, "maxDeg": 180}},
]


def trestle_bones(prefix, parent, x, y, zt, length, pivot, theta, lbrace, title_en, title_de, xoff=0.0):
    """🦵 A folding trestle pair and its two struts. Both turn about the world x axis (twisted frames: u down, v = -z, w = x): the trestle about its top bar in the hinge housings, the strut about its pin in the catch under the board, from where it hooks behind the foot bar."""
    out = []
    py, pz, _, _ = pivot
    base_y = TABLE_Y if prefix == "table" else BENCH_HINGE
    for side, sign in (("r", -1), ("l", 1)):
        tid = f"{prefix}-trestle-{side}"
        side_en, side_de = ("right", "rechtes") if side == "r" else ("left", "linkes")
        out.append({"id": tid, "parent": parent, "title": T(f"{title_en} trestle, {side_en} end", f"{title_de}bock, {side_de} Ende"), "x": x, "y": y, "z": sign * zt, "angleDeg": -90, "twistDeg": 90, "lengthM": length, "joint": {"type": "revolute", "minDeg": -90 if side == "r" else 0, "maxDeg": 0 if side == "r" else 90}})
        out.append({"id": f"{prefix}-pin-{side}", "parent": parent, "title": T(f"{title_en} strut pin in the catch under the board, {side_en} end", f"{title_de}spreizenbolzen in der Raste unter dem Brett, {side_de} Ende"), "x": r6(x + sign * xoff), "y": r6(py - base_y), "z": sign * pz, "angleDeg": -90, "twistDeg": 90, "lengthM": 0, "joint": {"type": "fixed"}})
        out.append({"id": f"{prefix}-brace-{side}", "parent": f"{prefix}-pin-{side}", "title": T(f"{title_en} folding strut, {side_en} end", f"{title_de}spreize, {side_de} Ende"), "x": 0, "y": 0, "angleDeg": r6(theta if side == "r" else -theta), "lengthM": r6(lbrace), "joint": {"type": "revolute", "minDeg": r6(-(90 + theta)) if side == "r" else 0, "maxDeg": 0 if side == "r" else r6(90 + theta)}})
    return out


BONES += trestle_bones("table", "table", 0, TABLE_HINGE - TABLE_Y, T_ZT, TABLE_HINGE, T_PIVOT, THETA_T, L_BRACE_T, "Table", "Tisch", xoff=0.010)
for name, x0, en, de in (("bench-front", B_IN, "Front bench", "Vordere Bank"), ("bench-rear", -B_IN - B_W, "Rear bench", "Hintere Bank")):
    BONES.append({"id": f"{name}-slide", "parent": "ground", "title": T(f"{en}, carried sideways", f"{de}, seitlich getragen"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 0, "minM": -0.35, "maxM": 0.35}})
    BONES.append({"id": name, "parent": f"{name}-slide", "title": T(f"{en}: seat board with battens, hinge housings and catches", f"{de}: Sitzbrett mit Leisten, Klappbeschlägen und Rasten"), "x": x0, "y": BENCH_HINGE, "angleDeg": 0, "lengthM": B_W, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -0.40, "maxM": 0.20}})
    BONES += trestle_bones(name, name, B_W / 2, 0, B_ZT, BENCH_HINGE, B_PIVOT, THETA_B, L_BRACE_B, en, de.replace("Vordere Bank", "Vorderer Bank").replace("Hintere Bank", "Hinterer Bank"))

FRAMES = rest_frames(BONES)
BONE_IDS = [b["id"] for b in BONES]

M_WEATHERED = "spruce-weathered"
M_FRESH = "spruce-fresh"
M_PAINT = "steel-painted"
M_GALV = "steel-galvanised"
M_RUST = "steel-rusted"
M_PE = "polyethylene-black"


def worn_board(u0, u1, v0, v1, top=0.006, bottom=0.002):
    """🪵 End section of a weathered board: the top arrises abraded by years of forearms and sleeves, the lower ones only eased."""
    return [(u0 + bottom, v0), (u1 - bottom, v0), (u1, v0 + bottom), (u1, v1 - top), (u1 - top * 0.5, v1 - top * 0.25), (u1 - top, v1), (u0 + top, v1), (u0 + top * 0.5, v1 - top * 0.25), (u0, v1 - top), (u0, v0 + bottom)]


def trestle_parts(prefix, bone, half_top, up_in, up_out, foot, cap, length, scale, bent=None, bowed=False, cap_missing=None, thick=0.0075):
    """🦿 One trestle frame of 30 x 15 mm rectangular tube at one scale: the painted inverted U (top bar and uprights) and the rusted foot (stubs and foot bar) with its PE caps; `bent` kinks the right upright outward, `bowed` lifts the foot bar 18 mm in the middle."""
    rust_top = length - 0.06
    bar = length - 0.03
    if bent:
        upper = [(-0.015, -half_top), (-0.015, half_top), (rust_top, half_top), (rust_top, up_in), (0.015, up_in), (0.015, -up_in), (bent[0], -up_in), (rust_top, -up_in - bent[1]), (rust_top, -up_out - bent[1]), (bent[0], -up_out)]
        rust = [(rust_top, -up_out - bent[1]), (rust_top, -up_in - bent[1]), (bar, -up_in - bent[1]), (bar, up_in), (rust_top, up_in), (rust_top, up_out), (bar, up_out), (bar, foot), (length, foot), (length, -foot), (bar, -foot), (bar, -up_out - bent[1])]
    else:
        upper = [(-0.015, -half_top), (-0.015, half_top), (rust_top, half_top), (rust_top, up_in), (0.015, up_in), (0.015, -up_in), (rust_top, -up_in), (rust_top, -half_top)]
        if bowed:
            rust = [(rust_top, -up_out), (rust_top, -up_in), (bar, -up_in), (bar, -0.12), (bar - 0.018, -0.04), (bar - 0.018, 0.04), (bar, 0.12), (bar, up_in), (rust_top, up_in), (rust_top, up_out), (bar, up_out), (bar, foot), (length, foot), (length, 0.12), (length - 0.018, 0.04), (length - 0.018, -0.04), (length, -0.12), (length, -foot), (bar, -foot), (bar, -up_out)]
        else:
            rust = [(rust_top, -up_out), (rust_top, -up_in), (bar, -up_in), (bar, up_in), (rust_top, up_in), (rust_top, up_out), (bar, up_out), (bar, foot), (length, foot), (length, -foot), (bar, -foot), (bar, -up_out)]
    if up_out != half_top and not bent:
        upper = [(-0.015, -half_top), (-0.015, half_top), (0.015, half_top), (0.015, up_out), (rust_top, up_out), (rust_top, up_in), (0.015, up_in), (0.015, -up_in), (rust_top, -up_in), (rust_top, -up_out), (0.015, -up_out), (0.015, -half_top)]
    parts = [
        part(f"{prefix}-frame", bone, "sv", upper, -thick, thick, M_PAINT, "painted-steel", 0.34, fill=0.28, hatch="flaking"),
        part(f"{prefix}-foot", bone, "sv", rust, -thick, thick, M_RUST, "feet", 0.2, fill=0.28, hatch="rust"),
    ]
    if cap:
        for side, sign in (("r", -1), ("l", 1)):
            if cap_missing == side:
                continue
            parts.append(part(f"{prefix}-cap-{side}", bone, "sv", rect(length - 0.032, length, sign * foot, sign * (foot + cap)), -0.0095, 0.0095, M_PE, "feet", 0.08, fill=0.55))
    return parts


def brace_part(prefix, bone, length, w=0.0075):
    """🥢 The folding strut of 15 x 15 x 1.5 mm square tube, pinned in its catch under the board and hooked behind the foot bar."""
    return part(f"{prefix}-strut", bone, "sv", rect(0, length, -w, w), -0.0075, 0.0075, M_PAINT, "painted-steel", 0.3, fill=0.36, slender=True)


def table_parts(scale):
    """🍽 The table (top, battens, housings, catches, two trestles, two struts) at one scale."""
    p = []
    vb, vt = TABLE_TOP - 0.025 - TABLE_Y, TABLE_TOP - TABLE_Y
    if scale in ("50", "100"):
        prof = worn_board(-0.25, 0.25, vb, vt) if scale == "50" else worn_board(-0.25, 0.25, vb, vt, 0.004, 0.0)
        if scale == "100":
            prof = [(-0.25, vb), (0.25, vb), (0.25, vt - 0.004), (0.246, vt), (-0.246, vt), (-0.25, vt - 0.004)]
        p.append(part("table-top", "table", "sw", prof, -L_TOP, L_TOP, M_WEATHERED, "weathered-timber", 0.62, hatch="grain"))
        for side, s in (("r", -1), ("l", 1)):
            p.append(part(f"table-batten-{side}", "table", "sw", rect(s * 0.205, s * 0.227, 0.070, vb), -1.08, 1.08, M_WEATHERED, "weathered-timber", 0.5))
        for end, e in (("r", -1), ("l", 1)):
            for side, s in (("r", -1), ("l", 1)):
                p.append(part(f"table-housing-{end}{side}", "table", "sw", rect(s * 0.19, s * 0.205, 0.080, vb), e * T_ZT - 0.02, e * T_ZT + 0.02, M_GALV, "galvanised-fittings", 0.42, fill=0.4))
            if scale in ("50", "100"):
                zc = e * T_PIVOT[1]
                p.append(part(f"table-catch-{end}", "table", "sw", rect(-0.028, 0.028, vb - (0.0075 if scale == "50" else 0.015), vb), zc - 0.02, zc + 0.02, M_GALV, "galvanised-fittings", 0.42, fill=0.5))
    elif scale == "200":
        prof = [(-0.25, vb - 0.005), (0.25, vb - 0.005), (0.25, vt), (-0.25, vt)]
        p.append(part("table-top", "table", "sw", prof, -L_TOP, L_TOP, M_WEATHERED, "weathered-timber", 0.62))
        for side, s in (("r", -1), ("l", 1)):
            p.append(part(f"table-batten-{side}", "table", "sw", rect(s * 0.19, s * 0.227, 0.070, vb - 0.005), -1.08, 1.08, M_WEATHERED, "weathered-timber", 0.5))
        for end, e in (("r", -1), ("l", 1)):
            p.append(part(f"table-housing-{end}", "table", "sw", rect(-0.19, 0.19, 0.085, vb - 0.005), e * T_ZT - 0.015, e * T_ZT + 0.015, M_GALV, "galvanised-fittings", 0.42, fill=0.4))
    else:
        p.append(part("table-top", "table", "sw", rect(-0.25, 0.25, 0.065, vt), -L_TOP, L_TOP, M_WEATHERED, "weathered-timber", 0.62))
        for end, e in (("r", -1), ("l", 1)):
            for side, s in (("r", -1), ("l", 1)):
                p.append(part(f"table-housing-{end}{side}", "table", "sw", rect(s * 0.175, s * 0.25, -0.010, 0.065), e * T_ZT - 0.0375, e * T_ZT + 0.0375, M_GALV, "galvanised-fittings", 0.42, fill=0.05))
    for end in ("r", "l"):
        bone = f"table-trestle-{end}"
        if scale == "50":
            p += trestle_parts(f"table-trestle-{end}", bone, 0.19, 0.16, 0.19, 0.20, 0.02, TABLE_HINGE, scale, bowed=(end == "l"))
            p.append(brace_part(f"table-brace-{end}", f"table-brace-{end}", L_BRACE_T))
        elif scale == "100":
            p += trestle_parts(f"table-trestle-{end}", bone, 0.19, 0.16, 0.19, 0.20, 0.02, TABLE_HINGE, scale, bowed=(end == "l"))
            p.append(brace_part(f"table-brace-{end}", f"table-brace-{end}", L_BRACE_T))
        elif scale == "200":
            p += trestle_parts(f"table-trestle-{end}", bone, 0.19, 0.16, 0.19, 0.22, 0, TABLE_HINGE, scale, thick=0.015)
            p.append(brace_part(f"table-brace-{end}", f"table-brace-{end}", L_BRACE_T))
        else:
            for side, s in (("r", -1), ("l", 1)):
                p.append(part(f"table-trestle-{end}-leg-{side}", bone, "sv", rect(0.020, TABLE_HINGE - 0.075, s * 0.175, s * 0.25), -0.0375, 0.0375, M_PAINT, "painted-steel", 0.34, fill=0.05))
            p.append(part(f"table-trestle-{end}-foot", bone, "sv", rect(TABLE_HINGE - 0.075, TABLE_HINGE, -0.25, 0.25), -0.0375, 0.0375, M_RUST, "feet", 0.2, fill=0.05))
            p.append(brace_part(f"table-brace-{end}", f"table-brace-{end}", L_BRACE_T))
    return p


def bench_parts(name, scale):
    """🪑 One bench (seat board, battens, housings, catches, two trestles, two struts) at one scale."""
    p = []
    fresh = name == "bench-front"
    mat, reg, tone = (M_FRESH, "fresh-timber", 0.82) if fresh else (M_WEATHERED, "weathered-timber", 0.6)
    if scale in ("50", "100"):
        if fresh or scale == "100":
            prof = rect(0, B_W, 0.030, 0.055) if fresh else [(0, 0.030), (B_W, 0.030), (B_W, 0.051), (B_W - 0.004, 0.055), (0.004, 0.055), (0, 0.051)]
        else:
            prof = worn_board(0, B_W, 0.030, 0.055)
        p.append(part(f"{name}-seat", name, "sw", prof, -L_TOP, L_TOP, mat, reg, tone, hatch="grain"))
        for side, (u0, u1) in (("i", (0.015, 0.037)), ("o", (0.213, 0.235))):
            p.append(part(f"{name}-batten-{side}", name, "sw", rect(u0, u1, -0.010, 0.030), -1.08, 1.08, M_WEATHERED, "weathered-timber", 0.5))
        for end, e in (("r", -1), ("l", 1)):
            for side, (u0, u1) in (("i", (0.037, 0.052)), ("o", (0.198, 0.213))):
                p.append(part(f"{name}-housing-{end}{side}", name, "sw", rect(u0, u1, -0.005, 0.030), e * B_ZT - 0.02, e * B_ZT + 0.02, M_GALV, "galvanised-fittings", 0.42, fill=0.4))
            if scale in ("50", "100"):
                zc = e * B_PIVOT[1]
                p.append(part(f"{name}-catch-{end}", name, "sw", rect(0.10, 0.15, 0.0225 if scale == "50" else 0.015, 0.030), zc - 0.02, zc + 0.02, M_GALV, "galvanised-fittings", 0.42, fill=0.5))
    elif scale == "200":
        p.append(part(f"{name}-seat", name, "sw", rect(0, B_W, 0.025, 0.055), -L_TOP, L_TOP, mat, reg, tone))
        for side, (u0, u1) in (("i", (0.015, 0.052)), ("o", (0.198, 0.235))):
            p.append(part(f"{name}-batten-{side}", name, "sw", rect(u0, u1, -0.010, 0.025), -1.08, 1.08, M_WEATHERED, "weathered-timber", 0.5))
        for end, e in (("r", -1), ("l", 1)):
            p.append(part(f"{name}-housing-{end}", name, "sw", rect(0.052, 0.198, -0.005, 0.025), e * B_ZT - 0.015, e * B_ZT + 0.015, M_GALV, "galvanised-fittings", 0.42, fill=0.4))
    else:
        inner = (-0.055, B_W) if fresh else (0, B_W + 0.055)
        p.append(part(f"{name}-seat", name, "sw", rect(inner[0], inner[1], -0.020, 0.055), -L_TOP, L_TOP, mat, reg, tone))
    for end in ("r", "l"):
        bone = f"{name}-trestle-{end}"
        bent = (0.20, 0.012) if (name == "bench-rear" and end == "r" and scale in ("50", "100")) else None
        missing = "l" if (name == "bench-rear" and end == "l") else None
        if scale in ("50", "100"):
            p += trestle_parts(bone, bone, 0.073, 0.043, 0.073, 0.105, 0.02, BENCH_HINGE, scale, bent=bent, cap_missing=missing)
            p.append(brace_part(f"{name}-brace-{end}", f"{name}-brace-{end}", L_BRACE_B))
        elif scale == "200":
            p += trestle_parts(bone, bone, 0.073, 0.043, 0.073, 0.125, 0, BENCH_HINGE, scale, thick=0.015)
            p.append(brace_part(f"{name}-brace-{end}", f"{name}-brace-{end}", L_BRACE_B))
        else:
            reach = (-0.18, 0.125) if fresh else (-0.125, 0.18)
            lo, hi = (-0.0125, 0.0625) if end == "r" else (-0.0625, 0.0125)
            p.append(part(f"{bone}-frame", bone, "sv", rect(0.020, BENCH_HINGE - 0.075, reach[0], reach[1]), lo, hi, M_PAINT, "painted-steel", 0.34, fill=0.05, envelope=True))
            p.append(part(f"{bone}-foot", bone, "sv", rect(BENCH_HINGE - 0.075, BENCH_HINGE, reach[0], reach[1]), lo, hi, M_RUST, "feet", 0.2, fill=0.05))
    return p


def parts_of(scale):
    """📋 All parts of the set at one scale."""
    return table_parts(scale) + bench_parts("bench-front", scale) + bench_parts("bench-rear", scale)




DENSITY = {M_WEATHERED: 450, M_FRESH: 450, M_PAINT: 7850, M_GALV: 7850, M_RUST: 7850, M_PE: 950}

MATERIALS = [
    {"id": M_WEATHERED, "title": T("Spruce, weathered grey (EN 338 C24)", "Fichte, grau verwittert (EN 338 C24)"), "emissivity": 0.92, "solarAbsorptivity": 0.68, "densityKgM3": 450, "conductivityWmK": 0.12, "specificHeatJKgK": 1600, "elasticModulusPa": 11000000000.0, "strengthPa": 24000000.0, "source": "authored-estimate", "reference": "weathered-wood"},
    {"id": M_FRESH, "title": T("Spruce, freshly planed replacement board", "Fichte, frisch gehobeltes Ersatzbrett"), "emissivity": 0.90, "solarAbsorptivity": 0.45, "densityKgM3": 450, "conductivityWmK": 0.12, "specificHeatJKgK": 1600, "elasticModulusPa": 11000000000.0, "strengthPa": 24000000.0, "source": "literature", "reference": "en-338-2016"},
    {"id": M_PAINT, "title": T("Steel tube, green enamel, flaking", "Stahlrohr, grün lackiert, abblätternd"), "emissivity": 0.90, "solarAbsorptivity": 0.80, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210000000000.0, "strengthPa": 235000000.0, "source": "literature", "reference": "en-10219-2"},
    {"id": M_GALV, "title": T("Galvanised pressed steel, weathered", "Verzinkter Pressstahl, bewittert"), "emissivity": 0.28, "solarAbsorptivity": 0.62, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210000000000.0, "strengthPa": 235000000.0, "source": "authored-estimate", "reference": "incropera-2018"},
    {"id": M_RUST, "title": T("Steel tube ends, paint worn off, rusted", "Stahlrohrenden, Lack abgerieben, verrostet"), "emissivity": 0.85, "solarAbsorptivity": 0.78, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210000000000.0, "strengthPa": 235000000.0, "source": "authored-estimate", "reference": "incropera-2018"},
    {"id": M_PE, "title": T("Black polyethylene foot cap", "Schwarze Polyethylen-Fußkappe"), "emissivity": 0.92, "solarAbsorptivity": 0.90, "densityKgM3": 950, "conductivityWmK": 0.40, "specificHeatJKgK": 1900, "elasticModulusPa": 900000000.0, "source": "literature", "reference": "iso-10456-2007"},
]

TRESTLES = [b["id"] for b in BONES if "-trestle-" in b["id"]]
BRACES = [b["id"] for b in BONES if "-brace-" in b["id"]]
BENCHES = ["bench-front", "bench-rear"]
BONE = {b["id"]: b for b in BONES}
VT = TABLE_TOP - TABLE_Y
VB = VT - 0.025


def bone_masses():
    """⚖️ Mass and centre of every bone from the 1:50 prisms (volume x density x the fill of hollow tubes and pressed housings)."""
    acc = {}
    for p in parts_of("50"):
        m = geo.prism_volume(p) * DENSITY[p["material"]] * p.get("fill", 1.0)
        c = geo.prism_centroid(p)
        a = acc.setdefault(p["bone"], [0.0, 0.0, 0.0])
        a[0] += m
        a[1] += m * c[0]
        a[2] += m * c[1]
    return {b: (a[0], a[1] / a[0], a[2] / a[0]) for b, a in acc.items()}


MASSES = bone_masses()
MASS_TOTAL = sum(m for m, _, _ in MASSES.values())
SOURCE_MASS = {"table": "wood-handbook-2021", "bench-front": "wood-handbook-2021", "bench-rear": "wood-handbook-2021"}

SECTIONS = {
    "table": {"areaM2": 0.01448, "secondMomentM4": 3.079e-06, "extremeFibreM": 0.0527, "material": M_WEATHERED, "title": T("Top 500 x 25 with two battens 22 x 45", "Platte 500 x 25 mit zwei Leisten 22 x 45")},
    "bench-front": {"areaM2": 0.00801, "secondMomentM4": 2.011e-06, "extremeFibreM": 0.0454, "material": M_FRESH, "title": T("Seat 250 x 25 with two battens 22 x 40", "Sitz 250 x 25 mit zwei Leisten 22 x 40")},
    "bench-rear": {"areaM2": 0.00801, "secondMomentM4": 2.011e-06, "extremeFibreM": 0.0454, "material": M_WEATHERED, "title": T("Seat 250 x 25 with two battens 22 x 40", "Sitz 250 x 25 mit zwei Leisten 22 x 40")},
}
for t in TRESTLES:
    SECTIONS[t] = {"areaM2": 0.000126, "secondMomentM4": 1.4067e-08, "extremeFibreM": 0.015, "material": M_PAINT, "title": T("Rectangular hollow section 30 x 15 x 1.5 (EN 10219)", "Rechteckhohlprofil 30 x 15 x 1,5 (EN 10219)")}
for b in BRACES:
    SECTIONS[b] = {"areaM2": 0.000081, "secondMomentM4": 2.4908e-09, "extremeFibreM": 0.0075, "material": M_PAINT, "title": T("Square hollow section 15 x 15 x 1.5 (EN 10219)", "Quadrathohlprofil 15 x 15 x 1,5 (EN 10219)")}
for b in BONES:
    if b["id"] in MASSES:
        m, cu, cv = MASSES[b["id"]]
        b["mass"] = {"kg": round(m, 3), "comX": r6(cu), "comY": r6(cv), "distribution": "point", "source": "authored-estimate", "reference": SOURCE_MASS.get(b["id"], "en-10219-2")}
    if b["id"] in SECTIONS:
        b["section"] = SECTIONS[b["id"]]


def anchor(aid, bone, x, y, z, kind, en, de, **extra):
    """📍 One anchor in the local frame of its bone."""
    a = {"id": aid, "bone": bone, "x": r6(x), "y": r6(y), "z": r6(z), "kind": kind, "title": T(en, de)}
    a.update(extra)
    return a


ANCHORS = []
FEET = []
for t in TRESTLES:
    length = BONE[t]["lengthM"]
    reach = 0.15 if t.startswith("table") else 0.07
    for side, s in (("r", -1), ("l", 1)):
        aid = f"{t}-foot-{side}"
        FEET.append(aid)
        ANCHORS.append(anchor(aid, t, length, 0, s * reach, "contact", f"Foot of the {t.replace('-', ' ')}, {'right' if side == 'r' else 'left'} cap", f"Fuß von {t}, {'rechte' if side == 'r' else 'linke'} Kappe", support="contact", normalDeg=90, footprintLengthM=0.015, footprintWidthM=0.10, frictionCoefficient=0.5))
TABLE_REST = []
for i, (u, z) in enumerate(((-0.216, -1.0), (0.216, -1.0), (-0.216, 1.0), (0.216, 1.0)), 1):
    TABLE_REST.append(f"table-rest-{i}")
    ANCHORS.append(anchor(f"table-rest-{i}", "table", u, 0.070, z, "contact", f"Batten end {i} of the folded table on the floor", f"Leistenende {i} des zusammengeklappten Tischs am Boden", support="contact", normalDeg=90, footprintLengthM=0.022, footprintWidthM=0.10, frictionCoefficient=0.5))
TABLE_FACE = []
for i, (u, z) in enumerate(((-0.2, -1.0), (0.2, -1.0), (-0.2, 1.0), (0.2, 1.0)), 1):
    TABLE_FACE.append(f"table-face-{i}")
    ANCHORS.append(anchor(f"table-face-{i}", "table", u, VT, z, "contact", f"Top face {i} of the table turned over onto the benches", f"Plattenfläche {i} des auf die Bänke gedrehten Tischs", support="contact", normalDeg=90, footprintLengthM=0.10, footprintWidthM=0.10, frictionCoefficient=0.5))
BENCH_REST = {}
LIFT = {}
for name in BENCHES:
    BENCH_REST[name] = []
    for side, z in (("r", -1.0), ("l", 1.0)):
        aid = f"{name}-rest-{side}"
        BENCH_REST[name].append(aid)
        ANCHORS.append(anchor(aid, name, B_W / 2, -0.010, z, "contact", f"Battens of the folded {name.replace('-', ' ')} on the stack, {'right' if side == 'r' else 'left'} end", f"Leisten der zusammengeklappten {name} auf dem Stapel, {'rechtes' if side == 'r' else 'linkes'} Ende", support="contact", normalDeg=90, footprintLengthM=0.20, footprintWidthM=0.10, frictionCoefficient=0.5))
    LIFT[name] = []
    for side, z in (("r", -1.08), ("l", 1.08)):
        aid = f"{name}-hand-{side}"
        LIFT[name].append(aid)
        ANCHORS.append(anchor(aid, name, B_W / 2, 0.055, z, "support", f"Hand of the worker carrying the {name.replace('-', ' ')}, {'right' if side == 'r' else 'left'} end", f"Hand des Arbeiters, der die {name} trägt, {'rechtes' if side == 'r' else 'linkes'} Ende", support="pin"))
LIFT["table"] = []
for side, z in (("r", -1.08), ("l", 1.08)):
    aid = f"table-hand-{side}"
    LIFT["table"].append(aid)
    ANCHORS.append(anchor(aid, "table", 0, VT, z, "support", f"Hands of the worker at the {'right' if side == 'r' else 'left'} table end", f"Hände des Arbeiters am {'rechten' if side == 'r' else 'linken'} Tischende", support="pin"))
SEATS = []
PLACES = []
for name, sign in (("bench-front", 1), ("bench-rear", -1)):
    for i, z in enumerate((-0.70, 0.0, 0.70), 1):
        sid = f"{name}-seat-{i}"
        SEATS.append(sid)
        ANCHORS.append(anchor(sid, name, B_W / 2, 0.055, z, "load", f"Sitting point {i} on the {name.replace('-', ' ')}", f"Sitzpunkt {i} auf der {name}"))
        pid = f"table-place-{name.split('-')[1]}-{i}"
        PLACES.append(pid)
        ANCHORS.append(anchor(pid, "table", sign * 0.15, VT, z, "load", f"Place setting {i} on the {name.split('-')[1]} side of the table", f"Gedeck {i} auf der {'vorderen' if sign > 0 else 'hinteren'} Tischseite"))

REGIONS_IDS = ["weathered-timber", "fresh-timber", "painted-steel", "galvanised-fittings", "feet"]


EXPOSED = {"weathered-timber": 0.58, "fresh-timber": 0.47, "painted-steel": 0.98, "galvanised-fittings": 0.57, "feet": 0.85}


def binding(sun, contact=None, wet=None):
    """🌤 The thermal binding of a pose: exposed and sunlit share and contact of every region."""
    regions = {}
    for r in REGIONS_IDS:
        b = {"exposedFraction": EXPOSED[r], "sunlitFraction": sun[r]}
        if contact and r in contact:
            b["contact"] = {"fraction": contact[r][0], "tempC": contact[r][1]}
        if wet and r in wet:
            b["wettedness"] = wet[r]
        regions[r] = b
    return {"regions": regions, "emitters": {}}


SUN_SERVICE = {"weathered-timber": 0.42, "fresh-timber": 0.45, "painted-steel": 0.35, "galvanised-fittings": 0.1, "feet": 0.3}
SUN_USE = {"weathered-timber": 0.30, "fresh-timber": 0.25, "painted-steel": 0.2, "galvanised-fittings": 0.05, "feet": 0.2}
SUN_DEPOT = {r: 0.0 for r in REGIONS_IDS}

FOLD = {"table-trestle-r": {"angleDeg": -90}, "table-trestle-l": {"angleDeg": 90}, "table-brace-r": {"angleDeg": r6(-90 - THETA_T)}, "table-brace-l": {"angleDeg": r6(90 + THETA_T)}}
for name in BENCHES:
    FOLD[f"{name}-trestle-r"] = {"angleDeg": -90}
    FOLD[f"{name}-trestle-l"] = {"angleDeg": 90}
    FOLD[f"{name}-brace-r"] = {"angleDeg": r6(-90 - THETA_B)}
    FOLD[f"{name}-brace-l"] = {"angleDeg": r6(90 + THETA_B)}


def fold(prefix, state=True):
    """🧷 The fold offsets of `prefix`: all folded (`True`) or the trestles swung down with the struts still flat under the board (`"struts"`)."""
    return {k: v for k, v in FOLD.items() if k.startswith(prefix + "-") and (state is True or "-brace-" in k)}


STACK_LIFT = 0.070 - (BENCH_HINGE - 0.010)
STACK_SHIFT = {"bench-front": r6(0.126 - B_W / 2 - B_IN), "bench-rear": r6(-0.126 + B_W / 2 + B_IN)}
MAINT_SHIFT = {"bench-front": r6(0.155 - B_W / 2 - B_IN), "bench-rear": r6(-0.155 + B_W / 2 + B_IN)}


def bones_of(table_lift=0.0, table_flip=0.0, table_folded=False, benches=None):
    """🎚 Bone offsets of a configuration: table lift and flip, folded table, and per bench (shift, lift, folded)."""
    out = {}
    if table_lift:
        out["table-lift"] = {"dy": r6(table_lift)}
    if table_flip:
        out["table"] = {"angleDeg": table_flip}
    if table_folded:
        out.update(fold("table", table_folded))
    for name, (shift, lift, folded) in (benches or {}).items():
        if shift:
            out[f"{name}-slide"] = {"dx": r6(shift)}
        if lift:
            out[name] = {"dy": r6(lift)}
        if folded:
            out.update(fold(name, folded))
    return out


STORED_BENCHES = {n: (STACK_SHIFT[n], STACK_LIFT, True) for n in BENCHES}
G = 9.80665
OCCUPANT_N = round(75 * G, 1)
PLACE_N = round(2.3 * G + 40, 1)


def pose(pid, en, de, den, dde, bones, contacts, thermal, loads=None, key=False):
    """🧍 One pose."""
    p = {"id": pid, "title": T(en, de), "description": T(den, dde)}
    if key:
        p["key"] = True
    p.update({"bones": bones, "ground": "authored", "contacts": contacts, "thermal": thermal, "loads": loads or []})
    return p


IN_USE_LOADS = [{"id": f"occupant-{i + 1}", "kind": "occupant", "title": T(f"Seated adult {i + 1} (75 kg)", f"Sitzender Erwachsener {i + 1} (75 kg)"), "anchor": a, "fxN": 0, "fyN": -OCCUPANT_N, "source": "authored-estimate", "reference": "en-1991-1-1"} for i, a in enumerate(SEATS)]
IN_USE_LOADS += [{"id": f"place-{i + 1}", "kind": "live", "title": T(f"Place setting {i + 1}: a full one-litre mug (2.3 kg) and resting forearms (40 N)", f"Gedeck {i + 1}: ein voller Maßkrug (2,3 kg) und aufgestützte Unterarme (40 N)"), "anchor": a, "fxN": 0, "fyN": -PLACE_N, "source": "authored-estimate", "reference": "en-581-3"} for i, a in enumerate(PLACES)]

USE_CONTACT = {"weathered-timber": (0.10, 31), "fresh-timber": (0.21, 31)}

POSES = [
    pose("in-service", "In service in the beer garden", "Im Biergarten aufgestellt", "Table and both benches unfolded on the water-bound gravel of the park beer garden, empty; the left table trestle stands on its bowed foot bar, the rear bench on its kinked right upright.", "Tisch und beide Bänke aufgeklappt auf der wassergebundenen Decke des Parkbiergartens, leer; der linke Tischbock steht auf seinem durchgebogenen Fußrohr, die hintere Bank auf ihrem geknickten rechten Stiel.", {}, FEET, binding(SUN_SERVICE)),
    pose("in-use", "Six guests at the table", "Sechs Gäste am Tisch", "Three adults on each bench, six full litre mugs and resting forearms on the top; the guests cover and shade the seats.", "Je drei Erwachsene auf jeder Bank, sechs volle Maßkrüge und aufgestützte Unterarme auf der Platte; die Gäste bedecken und beschatten die Sitze.", {}, FEET, binding(SUN_USE, USE_CONTACT), IN_USE_LOADS),
    pose("maintenance", "Turned over for repair in the depot", "Zur Reparatur im Bauhof umgedreht", "Depot workshop: both benches pushed together, the table turned over onto them top-down with its trestles in the air, so the bowed foot bar can be straightened and the struts' catches checked; a worker presses on the foot bar.", "Bauhofwerkstatt: beide Bänke zusammengeschoben, der Tisch kopfüber daraufgelegt, die Böcke in der Luft, damit das durchgebogene Fußrohr gerichtet und die Rasten der Spreizen geprüft werden können; ein Arbeiter drückt auf das Fußrohr.", bones_of(table_flip=180, benches={n: (MAINT_SHIFT[n], 0, False) for n in BENCHES}), [f for f in FEET if f.startswith("bench")] + TABLE_FACE, binding(SUN_DEPOT, {"weathered-timber": (0.12, 18), "fresh-timber": (0.20, 18)}), [{"id": "straightening", "kind": "imposed", "title": T("Worker pressing the bowed foot bar straight (300 N)", "Arbeiter drückt das durchgebogene Fußrohr gerade (300 N)"), "bone": "table-trestle-l", "x": 0.70, "y": 0, "fxN": 0, "fyN": -300, "source": "authored-estimate"}]),
    pose("stored", "Folded and stacked", "Zusammengeklappt und gestapelt", "End of the season: the table folded and lying on its battens, both benches folded and laid on it side by side; the set is 135 mm high.", "Saisonende: der Tisch zusammengeklappt auf seinen Leisten liegend, beide Bänke zusammengeklappt nebeneinander daraufgelegt; die Garnitur ist 135 mm hoch.", bones_of(table_lift=-0.70, table_folded=True, benches=STORED_BENCHES), TABLE_REST + BENCH_REST["bench-front"] + BENCH_REST["bench-rear"], binding({"weathered-timber": 0.2, "fresh-timber": 0.3, "painted-steel": 0.0, "galvanised-fittings": 0.0, "feet": 0.0}, {"weathered-timber": (0.12, 16)})),
]

HELD = {"weathered-timber": 0.4, "fresh-timber": 0.45, "painted-steel": 0.3, "galvanised-fittings": 0.1, "feet": 0.3}
KEYS = [
    pose("lifted-key", "Table lifted", "Tisch angehoben", "Two workers hold the table 0.6 m up at its ends.", "Zwei Arbeiter halten den Tisch an den Enden 0,6 m hoch.", bones_of(table_lift=0.60), [f for f in FEET if f.startswith("bench")] + LIFT["table"], binding(SUN_DEPOT), key=True),
    pose("benches-together-key", "Benches pushed together", "Bänke zusammengeschoben", "Table still held up, the benches pushed together under it.", "Tisch noch hochgehalten, die Bänke darunter zusammengeschoben.", bones_of(table_lift=0.60, benches={n: (MAINT_SHIFT[n], 0, False) for n in BENCHES}), [f for f in FEET if f.startswith("bench")] + LIFT["table"], binding(SUN_DEPOT), key=True),
    pose("turned-key", "Table turned over in the air", "Tisch in der Luft umgedreht", "The held table is turned over about its long axis, trestles up.", "Der gehaltene Tisch ist um seine Längsachse umgedreht, Böcke nach oben.", bones_of(table_lift=0.60, table_flip=180, benches={n: (MAINT_SHIFT[n], 0, False) for n in BENCHES}), [f for f in FEET if f.startswith("bench")] + LIFT["table"], binding(SUN_DEPOT), key=True),
]


def unfold_keys():
    """🪜 The in-between key poses of unpacking the stack: each bench lifted off, carried to its place, unfolded and set down, then the table."""
    keys = []
    state = {n: [STACK_SHIFT[n], STACK_LIFT, True] for n in BENCHES}
    table = {"lift": -0.70, "folded": True}
    contacts_rest = {n: BENCH_REST[n] for n in BENCHES}
    done = []

    def snapshot(kid, en, de, holding):
        bones = bones_of(table_lift=table["lift"], table_folded=table["folded"], benches={n: tuple(v) for n, v in state.items()})
        contacts = (TABLE_REST if table["lift"] < -0.5 else []) + [a for n in BENCHES if n not in done and n not in holding for a in contacts_rest[n]] + [f for n in done for f in FEET if f.startswith(n)] + [a for h in holding for a in LIFT[h]]
        keys.append(pose(kid, en, de, en + ".", de + ".", bones, contacts, binding(HELD), key=True))

    for name, en, de in (("bench-front", "front bench", "vordere Bank"), ("bench-rear", "rear bench", "hintere Bank")):
        state[name] = [STACK_SHIFT[name], STACK_LIFT + 0.10, True]
        snapshot(f"{name}-off-key", f"The {en} lifted off the stack", f"Die {de} vom Stapel gehoben", [name])
        state[name] = [0, 0.05, True]
        snapshot(f"{name}-carried-key", f"The {en} carried to its place, still folded", f"Die {de} an ihren Platz getragen, noch zusammengeklappt", [name])
        state[name] = [0, 0.05, "struts"]
        snapshot(f"{name}-opened-key", f"The {en}'s trestles swung down in the hands", f"Die Böcke der {de} in den Händen heruntergeklappt", [name])
        state[name] = [0, 0.05, False]
        snapshot(f"{name}-locked-key", f"The {en}'s struts hooked behind the foot bars", f"Die Spreizen der {de} hinter den Fußrohren eingehängt", [name])
        state[name] = [0, 0, False]
        done.append(name)
    table["lift"] = 0.05
    snapshot("table-raised-key", "The folded table raised", "Der zusammengeklappte Tisch angehoben", ["table"])
    table["folded"] = "struts"
    snapshot("table-opened-key", "The table's trestles swung down in the hands", "Die Tischböcke in den Händen heruntergeklappt", ["table"])
    table["folded"] = False
    snapshot("table-locked-key", "The table's struts hooked behind the foot bars", "Die Tischspreizen hinter den Fußrohren eingehängt", ["table"])
    return keys


UNFOLD_KEYS = unfold_keys()
POSES += KEYS + UNFOLD_KEYS


def spans(timeline):
    """🦶 Contact spans from the contacts of consecutive keys: an anchor is in contact from a key on whose pose it is listed to the next key on whose pose it is not."""
    pose_of = {p["id"]: p for p in POSES}
    out = []
    anchors = sorted({a for _, pid in timeline for a in pose_of[pid]["contacts"]}, key=lambda a: [x["id"] for x in ANCHORS].index(a))
    for a in anchors:
        start = None
        for i, (t, pid) in enumerate(timeline):
            on = a in pose_of[pid]["contacts"]
            if on and start is None:
                start = t
            if not on and start is not None:
                out.append({"anchor": a, "fromS": start, "toS": timeline[i - 1][0] if timeline[i - 1][0] > start else t})
                start = None
        if start is not None:
            out.append({"anchor": a, "fromS": start, "toS": timeline[-1][0]})
    return out


def clip(cid, en, de, den, dde, timeline, phases, events, tracks, ease="linear"):
    """🎬 A clip over key poses."""
    return {"id": cid, "title": T(en, de), "description": T(den, dde), "durationS": timeline[-1][0], "fps": 25, "loop": False, "rootMotion": "none", "keys": [{"t": t, "pose": p, "ease": ease} for t, p in timeline], "contacts": spans(timeline), "phases": phases, "events": events, "tracks": tracks}


MAINT_TIMELINE = [(0, "in-service"), (1.5, "lifted-key"), (3.0, "benches-together-key"), (4.5, "turned-key"), (6.0, "maintenance"), (7.0, "maintenance")]
UNFOLD_TIMELINE = [(0, "stored")] + [(round(1.2 + 1.2 * i, 1), k["id"]) for i, k in enumerate(UNFOLD_KEYS)] + [(round(1.2 + 1.2 * len(UNFOLD_KEYS), 1), "in-service"), (round(2.2 + 1.2 * len(UNFOLD_KEYS), 1), "in-service")]
UNFOLD_END = UNFOLD_TIMELINE[-1][0]
UNFOLD_SET = UNFOLD_TIMELINE[-2][0]
SIT_TIMELINE = [(0, "in-service"), (6.0, "in-use")]

CLIPS = [
    clip("unfold", "Unpacking and unfolding the set", "Garnitur auspacken und aufklappen", "From the stack: each bench is lifted off, carried to its place, its trestles swung down, the struts swung down from their catches and hooked behind the foot bars, and set on its feet; then the table is raised, unfolded the same way and set down between them.", "Vom Stapel: jede Bank wird abgehoben, an ihren Platz getragen, ihre Böcke heruntergeklappt, die Spreizen aus ihren Rasten heruntergeschwenkt und hinter die Fußrohre gehängt, dann auf die Füße gestellt; danach wird der Tisch angehoben, ebenso aufgeklappt und zwischen ihnen abgesetzt.", UNFOLD_TIMELINE,
         [{"id": "front-bench", "fromS": 0, "toS": UNFOLD_TIMELINE[5][0], "title": T("Front bench", "Vordere Bank")}, {"id": "rear-bench", "fromS": UNFOLD_TIMELINE[5][0], "toS": UNFOLD_TIMELINE[9][0], "title": T("Rear bench", "Hintere Bank")}, {"id": "table", "fromS": UNFOLD_TIMELINE[9][0], "toS": UNFOLD_END, "title": T("Table", "Tisch")}],
         [{"t": UNFOLD_TIMELINE[4][0], "kind": "motion", "title": T("Front bench struts hook behind the foot bars", "Spreizen der vorderen Bank greifen hinter die Fußrohre")}, {"t": UNFOLD_TIMELINE[8][0], "kind": "motion", "title": T("Rear bench struts hook behind the foot bars", "Spreizen der hinteren Bank greifen hinter die Fußrohre")}, {"t": UNFOLD_TIMELINE[12][0], "kind": "motion", "title": T("Table struts hook behind the foot bars", "Tischspreizen greifen hinter die Fußrohre")}, {"t": UNFOLD_SET, "kind": "contact-on", "title": T("Table set down on its four feet", "Tisch auf seine vier Füße gestellt")}],
         [{"target": "region:weathered-timber:sunlitFraction", "ease": "linear", "keys": [{"t": 0, "v": 0.2}, {"t": UNFOLD_SET, "v": 0.42}, {"t": UNFOLD_END, "v": 0.42}]}]),
    clip("maintenance", "Turning the table over for repair", "Tisch zur Reparatur umdrehen", "In the depot workshop two workers lift the table 0.6 m, the benches are pushed together under it, the table is turned over about its long axis in the air and laid top-down on the benches, trestles up, ready to straighten the bowed foot bar.", "In der Bauhofwerkstatt heben zwei Arbeiter den Tisch 0,6 m an, die Bänke werden darunter zusammengeschoben, der Tisch wird in der Luft um seine Längsachse gedreht und kopfüber auf die Bänke gelegt, Böcke nach oben, bereit zum Richten des durchgebogenen Fußrohrs.", MAINT_TIMELINE,
         [{"id": "lift", "fromS": 0, "toS": 1.5, "title": T("Lift", "Anheben")}, {"id": "push", "fromS": 1.5, "toS": 3.0, "title": T("Benches together", "Bänke zusammen")}, {"id": "turn", "fromS": 3.0, "toS": 4.5, "title": T("Turn over", "Umdrehen")}, {"id": "lay-down", "fromS": 4.5, "toS": 7.0, "title": T("Lay down top-down", "Kopfüber ablegen")}],
         [{"t": 0, "kind": "contact-off", "title": T("Table feet leave the floor", "Tischfüße verlassen den Boden")}, {"t": 6.0, "kind": "contact-on", "title": T("Table top rests on the benches", "Tischplatte liegt auf den Bänken")}, {"t": 6.0, "kind": "load-on", "title": T("Worker presses the bowed foot bar", "Arbeiter drückt auf das durchgebogene Fußrohr"), "target": "straightening"}],
         [{"target": "load:straightening", "ease": "linear", "keys": [{"t": 0, "v": 0}, {"t": 6.0, "v": 0}, {"t": 7.0, "v": 1}]}]),
    clip("guests-sit-down", "Six guests sit down", "Sechs Gäste setzen sich", "Afternoon in the beer garden: six adults sit down one after another, three on each bench, and put their mugs on the table; the seats are covered and shaded as they fill.", "Nachmittag im Biergarten: sechs Erwachsene setzen sich nacheinander, je drei auf jede Bank, und stellen ihre Krüge auf den Tisch; die Sitze werden beim Füllen bedeckt und beschattet.", SIT_TIMELINE,
         [{"id": "filling", "fromS": 0, "toS": 6.0, "title": T("Filling up", "Auffüllen")}],
         [{"t": round(0.6 + 0.8 * i + 0.5, 2), "kind": "load-on", "title": T(f"Guest {i + 1} sits down", f"Gast {i + 1} setzt sich"), "target": f"occupant-{i + 1}"} for i in range(6)],
         [{"target": f"load:occupant-{i + 1}", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": round(0.6 + 0.8 * i, 2), "v": 0}, {"t": round(1.1 + 0.8 * i, 2), "v": 1}, {"t": 6.0, "v": 1}]} for i in range(6)] + [{"target": f"load:place-{i + 1}", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": round(1.1 + 0.8 * i, 2), "v": 0}, {"t": round(1.6 + 0.8 * i, 2), "v": 1}, {"t": 6.0, "v": 1}]} for i in range(6)]
         + [{"target": "region:fresh-timber:sunlitFraction", "ease": "linear", "keys": [{"t": 0, "v": 0.45}, {"t": 6.0, "v": 0.25}]}, {"target": "region:weathered-timber:sunlitFraction", "ease": "linear", "keys": [{"t": 0, "v": 0.42}, {"t": 6.0, "v": 0.30}]}]),
]

PENS = {
    "50": {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": 0.18, "ground": 0.7},
    "100": {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": 0.13, "ground": 0.5},
    "200": {"body": 0.25, "detail": 0.13, "texture": None, "glazing": 0.13, "ground": 0.35},
    "500": {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25},
}
RANGES = {"50": (20, 75), "100": (75, 150), "200": (150, 350), "500": (350, 1000)}


def hatch(hid, en, de, pattern, angle, spacing, pen=0.13):
    """▤ One hatch."""
    return {"id": hid, "title": T(en, de), "pattern": pattern, "angleDeg": angle, "spacingMm": spacing, "penMm": pen}


HATCHES = {
    "50": [hatch("grain-plan", "Spruce grain along the boards in plan", "Fichtenmaserung längs der Bretter im Grundriss", "lines", 90, 0.6), hatch("grain-front", "Spruce grain along the boards in the long view", "Fichtenmaserung längs der Bretter in der Längsansicht", "lines", 0, 0.6), hatch("flaking", "Flaking green enamel", "Abblätternder grüner Lack", "stipple", 0, 0.8), hatch("rust", "Rust at the worn tube ends", "Rost an den abgeriebenen Rohrenden", "cross", 45, 0.5)],
    "100": [hatch("grain-plan", "Spruce grain along the boards in plan", "Fichtenmaserung längs der Bretter im Grundriss", "lines", 90, 0.8), hatch("grain-front", "Spruce grain along the boards in the long view", "Fichtenmaserung längs der Bretter in der Längsansicht", "lines", 0, 0.8), hatch("rust", "Rust at the worn tube ends", "Rost an den abgeriebenen Rohrenden", "stipple", 0, 0.6)],
    "200": [hatch("grain-plan", "Spruce grain along the boards in plan", "Fichtenmaserung längs der Bretter im Grundriss", "lines", 90, 1.0)],
    "500": [],
}
Z_ORDER = {"leg": 3, "top": 6, "seat": 6, "batten": 5, "housing": 5.5, "catch": 5.4, "frame": 3, "foot": 3.1, "cap": 3.2, "strut": 2.5}


def z_of(pid):
    """🔢 Paint order of a part by its kind."""
    for k, v in Z_ORDER.items():
        if f"-{k}" in pid:
            return v
    return 4


def view_hatch(p, view, scale):
    """▤ The hatch of a part in one viewpoint at one scale."""
    ids = {h["id"] for h in HATCHES[scale]}
    if p.get("hatch") == "grain":
        h = {"plan": "grain-plan", "front": "grain-front"}.get(view)
    else:
        h = p.get("hatch")
    return h if h in ids else None


def stroke(sid, bone, role, pts, closed=False, z=7):
    """〰 A detail or texture stroke, drawn on top of the filled parts it lies in."""
    return {"id": sid, "bone": bone, "role": role, "z": z, "filled": False, "tone": 0.15, "kind": "poly", "points": [[r6(a), r6(b)] for a, b in pts], "closed": closed}


def circle(cx, cy, r, n=16):
    """⭕ A polygonal circle."""
    return [(cx + r * math.cos(2 * math.pi * i / n), cy + r * math.sin(2 * math.pi * i / n)) for i in range(n)]


def details(scale):
    """✒️ The traces of use drawn as strokes at one scale: glue joints, checks, the end split, carved initials, beer rings and chewing gum."""
    el, fr, pl = [], [], []
    if scale in ("50", "100", "200"):
        role = "detail"
        for i, x in enumerate((-0.0833, 0.0833)):
            pl.append(stroke(f"table-glue-{i + 1}", "table", role, [(x, -L_TOP + 0.004), (x, L_TOP - 0.004)]))
    if scale in ("50", "100"):
        for i, x in enumerate((-0.0833, 0.0833)):
            el.append(stroke(f"table-glue-end-{i + 1}", "table", "detail", [(x, VB + 0.001), (x, VT - 0.001)]))
        fr.append(stroke("table-end-split", "table", "detail", [(-1.098, 0.757), (-1.075, 0.760), (-1.055, 0.756), (-1.03, 0.759)]))
        for i, (cx, cz) in enumerate(((0.15, -0.70), (0.13, 0.05), (-0.14, 0.66))):
            pl.append(stroke(f"table-beer-ring-{i + 1}", "table", "texture", circle(cx, cz, 0.045), closed=True))
    if scale == "50":
        pl += [stroke("table-check-1", "table", "texture", [(0.02, 0.60), (0.024, 0.80), (0.021, 1.05)]), stroke("table-check-2", "table", "texture", [(-0.17, -1.00), (-0.166, -0.80), (-0.169, -0.55)]), stroke("table-check-3", "table", "texture", [(0.205, -0.30), (0.208, -0.05)])]
        pl += [stroke("table-initials-a", "table", "texture", [(-0.10, 0.20), (-0.07, 0.24), (-0.04, 0.20)]), stroke("table-initials-a-bar", "table", "texture", [(-0.085, 0.215), (-0.055, 0.215)]), stroke("table-initials-k", "table", "texture", [(-0.02, 0.20), (-0.02, 0.24)]), stroke("table-initials-k-arms", "table", "texture", [(0.01, 0.24), (-0.02, 0.22), (0.01, 0.20)]), stroke("table-initials-heart", "table", "texture", [(0.05, 0.215), (0.035, 0.232), (0.045, 0.242), (0.05, 0.236), (0.055, 0.242), (0.065, 0.232)], closed=True)]
        pl.append(stroke("table-burn", "table", "texture", circle(-0.20, 0.95, 0.008, 10), closed=True))
        pl += [stroke("bench-rear-check-1", "bench-rear", "texture", [(-0.43, -0.90), (-0.427, -0.55), (-0.43, -0.20)]), stroke("bench-rear-check-2", "bench-rear", "texture", [(-0.36, 0.30), (-0.362, 0.62), (-0.359, 1.00)])]
        for i, z in enumerate((0.28, 0.33, 0.52)):
            fr.append(stroke(f"table-gum-{i + 1}", "table", "texture", circle(z, 0.7495, 0.0035, 10), closed=True))
    return el, fr, pl


PEN_OF = {"50": {"board": 0.35, "tube": 0.25, "fitting": 0.18}, "100": {"board": 0.25, "tube": 0.18, "fitting": 0.18}}


def pen_kind(pid):
    """🖋 The pen class of a part: boards carry the silhouette, tubes the secondary line, fittings the finest."""
    if any(k in pid for k in ("-top", "-seat", "-batten")):
        return "board"
    if any(k in pid for k in ("-housing", "-catch", "-cap")):
        return "fitting"
    return "tube"


def representation(scale):
    """🖨 The authored representation of one scale."""
    ps = parts_of(scale)
    el, fr, pl = details(scale)
    geom = {"elevation": [], "front": [], "plan": []}
    for p in ps:
        for view in ("elevation", "front", "plan"):
            geom[view].append(shape(p, FRAMES, z_of(p["id"]), view_hatch(p, view, scale), PEN_OF.get(scale, {}).get(pen_kind(p["id"]))))
    geom["elevation"] += el
    geom["front"] += fr
    geom["plan"] += pl
    lo, hi = RANGES[scale]
    return {"status": "authored", "denominator": int(scale), "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[scale], "hatches": HATCHES[scale], "geometry": geom, "decisions": REP_DECISIONS[scale]}


def volume(scale):
    """🧊 The authored volume of one scale: body, thermal and structure lists."""
    ps = parts_of(scale)
    f = FEATURE[scale]
    body = [body_solid(p) for p in ps]
    thermal = [{"id": f"th-{p['id']}", "of": p["id"], "region": p["region"]} for p in ps]
    structure = []
    have = set()
    for p in ps:
        pid = p["id"]
        if "-housing-" in pid:
            trestle = pid.rsplit("-housing-", 1)[0] + "-trestle-" + pid.rsplit("-housing-", 1)[1][0]
            structure.append({"id": f"j-{pid}", "of": pid, "member": trestle, "part": "joint"})
        elif "-catch-" in pid:
            structure.append({"id": f"j-{pid}", "of": pid, "member": pid.replace("-catch-", "-brace-"), "part": "joint"})
        else:
            structure.append({"id": f"m-{pid}", "of": pid, "member": p["bone"], "part": "member"})
            have.add(p["bone"])
    for b in BRACES:
        if b not in have:
            structure.append({"id": f"m-{b}", "kind": "extrude", "bone": b, "material": M_PAINT, "role": "detail", "tone": 0.3, "profile": [[-0.0075, 0], [0.0075, 0], [0.0075, r6(BONE[b]["lengthM"])], [-0.0075, r6(BONE[b]["lengthM"])]], "plane": "zx", "from": -0.0075, "to": 0.0075, "slender": True, "member": b, "part": "member"})
    for b, (m, cu, cv) in MASSES.items():
        structure.append({"id": f"mass-{b}", "kind": "ellipsoid", "bone": b, "material": M_RUST, "role": "detail", "tone": 0.3, "radii": [f / 2, f / 2, f / 2], "transform": {"translation": [r6(cu), r6(cv), 0]}, "member": b, "part": "mass"})
    return {"status": "authored", "denominator": int(scale), "body": body, "thermal": thermal, "structure": structure, "decisions": VOL_DECISIONS[scale]}


def region_area(region):
    """🧾 Outer area of a region: the outer area of its 1:50 solids as `author-measure.ts` measures it (flood from outside, faces where its own solids meet left out)."""
    return MEASURED_AREA[region]


MEASURED_AREA = {"weathered-timber": 4.781, "fresh-timber": 1.223, "painted-steel": 0.823, "galvanised-fittings": 0.094, "feet": 0.21}


def region(rid, en, de, material, length, geometry, sky, layer, cap, ref):
    """🌡 One passive thermal region with its single layer."""
    return {"id": rid, "title": T(en, de), "kind": "surface", "model": "passive", "material": material, "areaM2": region_area(rid), "characteristicLengthM": length, "geometry": geometry, "skyViewFactor": sky, "layers": [layer], "heatCapacityJM2K": cap, "ratedPowerW": 0, "ratedPowerBasis": "passive", "source": "authored-estimate", "reference": ref}


def layer(lid, en, de, kind, material, t, k, cap, ref):
    """🧶 One layer of a region."""
    return {"id": lid, "title": T(en, de), "kind": kind, "material": material, "thicknessM": t, "resistanceM2KW": round(t / k, 6), "heatCapacityJM2K": cap, "source": "literature", "reference": ref}


CAP_TIMBER = round(450 * 1600 * min(0.025, math.sqrt(0.12 / (450 * 1600) * 3600)))
CAP_TUBE = round(7850 * 460 * 0.0015)
CAP_HOUSING = round(7850 * 460 * 0.0025)


def thermal_model():
    """♨️ The passive thermal model: weathered and fresh spruce, painted, galvanised and rusted steel in the summer sun of a beer garden."""
    regions = [
        region("weathered-timber", "Weathered grey spruce: table top, battens and rear bench seat", "Grau verwitterte Fichte: Tischplatte, Leisten und Sitz der hinteren Bank", M_WEATHERED, 0.5, "horizontal-plate-up", 0.85, layer("spruce-board", "Spruce board 25 mm", "Fichtenbrett 25 mm", "timber", M_WEATHERED, 0.025, 0.12, CAP_TIMBER, "iso-10456-2007"), CAP_TIMBER, "weathered-wood"),
        region("fresh-timber", "Freshly planed replacement seat board of the front bench", "Frisch gehobeltes Ersatzsitzbrett der vorderen Bank", M_FRESH, 0.25, "horizontal-plate-up", 0.9, layer("spruce-board-fresh", "Spruce board 25 mm, new", "Fichtenbrett 25 mm, neu", "timber", M_FRESH, 0.025, 0.12, CAP_TIMBER, "iso-10456-2007"), CAP_TIMBER, "en-338-2016"),
        region("painted-steel", "Green-enamelled tube trestles and struts, paint flaking", "Grün lackierte Rohrböcke und Spreizen, Lack blättert ab", M_PAINT, 0.03, "vertical-plate", 0.5, layer("tube-wall", "Tube wall 1.5 mm", "Rohrwand 1,5 mm", "metal", M_PAINT, 0.0015, 50, CAP_TUBE, "iso-10456-2007"), CAP_TUBE, "en-10219-2"),
        region("galvanised-fittings", "Galvanised hinge housings and strut catches under the boards", "Verzinkte Klappbeschläge und Spreizenrasten unter den Brettern", M_GALV, 0.035, "horizontal-plate-down", 0.15, layer("pressed-steel", "Pressed steel 2.5 mm", "Pressstahl 2,5 mm", "metal", M_GALV, 0.0025, 50, CAP_HOUSING, "iso-10456-2007"), CAP_HOUSING, "incropera-2018"),
        region("feet", "Rusted foot bars, tube ends and PE caps on the gravel", "Verrostete Fußrohre, Rohrenden und PE-Kappen auf dem Kies", M_RUST, 0.03, "vertical-plate", 0.45, layer("foot-tube-wall", "Tube wall 1.5 mm, rusted", "Rohrwand 1,5 mm, verrostet", "metal", M_RUST, 0.0015, 50, CAP_TUBE, "iso-10456-2007"), CAP_TUBE, "incropera-2018"),
    ]
    return {
        "environment": {"airTempC": 28, "meanRadiantTempC": 32, "skyTempC": 14, "groundTempC": 36, "relativeHumidity": 0.45, "windMS": 1.2, "windDirectionDeg": 90, "solarWM2": 780, "diffuseFraction": 0.15, "pressurePa": 101325},
        "referenceEnvironment": "summer-hot-sun",
        "passengers": [],
        "operating": [],
        "regions": regions,
        "emitters": [],
        "assumptions": [
            T("Passive surfaces only: absorbed sun, long-wave exchange with sky and gravel, convection and conduction into the 25 mm boards and the 1.5 mm tube walls; no emitters, no heat sources, 0 W rated power stated explicitly on every region.", "Nur passive Oberflächen: absorbierte Sonne, langwelliger Austausch mit Himmel und Kies, Konvektion und Leitung in die 25-mm-Bretter und die 1,5-mm-Rohrwände; keine Emitter, keine Wärmequellen, 0 W Nennleistung an jeder Region ausdrücklich angegeben."),
            T("Weathering changes the optics: the greyed spruce absorbs about 0.68 of the sun against 0.45 for the freshly planed replacement board (UV-degraded lignin and grey surface cells), so the old rear seat and the table top run several kelvin warmer in the sun than the new front seat; flaking green enamel 0.80, weathered zinc 0.62 with a low emittance of 0.28 that keeps the housings hot, rust 0.78 (authored-estimates after the Wood Handbook chapter on weathering and Incropera's tables of surface properties).", "Die Verwitterung ändert die Optik: Die vergraute Fichte absorbiert etwa 0,68 der Sonne gegenüber 0,45 beim frisch gehobelten Ersatzbrett (UV-abgebautes Lignin, graue Oberflächenzellen), so dass der alte hintere Sitz und die Tischplatte in der Sonne einige Kelvin wärmer werden als der neue vordere Sitz; abblätternder grüner Lack 0,80, bewittertes Zink 0,62 mit niedrigem Emissionsgrad 0,28, der die Beschläge heiß hält, Rost 0,78 (Schätzwerte nach dem Wood Handbook, Kapitel Verwitterung, und Incroperas Tabellen der Oberflächeneigenschaften)."),
            T("Areal heat capacity rho*cp*min(thickness, sqrt(alpha*3600 s)): spruce 25 mm takes part almost fully within one hour, the tube walls with their 1.5 mm.", "Flächenbezogene Wärmekapazität rho*cp*min(Dicke, sqrt(alpha*3600 s)): Fichte 25 mm nimmt binnen einer Stunde fast ganz teil, die Rohrwände mit ihren 1,5 mm."),
            T("Seated guests cover part of the seats and the top with clothed bodies at 31 degC (ISO 7730 clothing surface range) and shade them; sunlit shares are scenario data of each pose and clip.", "Sitzende Gäste bedecken einen Teil der Sitze und der Platte mit bekleideten Körpern bei 31 degC (Bekleidungsoberflächen nach ISO 7730) und beschatten sie; besonnte Anteile sind Szenariodaten jeder Pose und jedes Clips."),
            T("Maintenance and stored poses stand in the depot hall: no sun.", "Wartungs- und Lagerpose stehen in der Bauhofhalle: keine Sonne."),
        ],
    }


STRUCTURE = {
    "gravityMS2": G,
    "dynamics": "quasi-static",
    "supports": [],
    "assumptions": [
        T("The supports belong to the pose: in service and in use the set is solved as one rigid assembly on the twelve foot-bar contacts of its six trestles (table trestles 0.40 m across, benches 0.25 m), friction 0.5 on water-bound gravel; the struts lock each trestle against folding, so trestle, strut and board act as one frame. Carried, the hands of the workers are pins at the board ends.", "Die Auflager gehören zur Pose: aufgestellt und in Benutzung wird die Garnitur als ein starrer Verband auf den zwölf Fußrohrkontakten ihrer sechs Böcke gelöst (Tischböcke 0,40 m breit, Bänke 0,25 m), Reibung 0,5 auf wassergebundener Decke; die Spreizen sperren jeden Bock gegen Einklappen, so dass Bock, Spreize und Brett als ein Rahmen wirken. Getragen sind die Hände der Arbeiter Gelenke an den Brettenden."),
        T("Occupant loads 75 kg per seated adult at the three sitting points of each bench; place settings 2.3 kg mug plus 40 N of resting forearms; the EN 581-2 and EN 581-3 static proof loads for outdoor seating and tables (seat about 1300 to 1600 N, table edge vertical about 1000 N) are the order of magnitude the boards and trestles are designed for.", "Personenlasten 75 kg je sitzendem Erwachsenen an den drei Sitzpunkten jeder Bank; Gedecke 2,3 kg Krug plus 40 N aufgestützte Unterarme; die statischen Prüflasten der EN 581-2 und EN 581-3 für Gartensitzmöbel und -tische (Sitz etwa 1300 bis 1600 N, Tischkante senkrecht etwa 1000 N) sind die Größenordnung, für die Bretter und Böcke ausgelegt sind."),
        T("In the maintenance pose the turned-over table rests on the two benches at four top-face contacts and the benches on their eight feet; in the stored pose the folded table lies on its four batten ends and each folded bench on two batten ends on the table. The rigid-assembly solve takes these contacts as the reactions of the whole stack; the share of the bench weight passed through the table to the floor is not separated.", "In der Wartungspose liegt der umgedrehte Tisch mit vier Plattenkontakten auf den zwei Bänken und die Bänke stehen auf ihren acht Füßen; in der Lagerpose liegt der zusammengeklappte Tisch auf seinen vier Leistenenden und jede zusammengeklappte Bank mit zwei Leistenenden auf dem Tisch. Die Starrkörperlösung nimmt diese Kontakte als Reaktionen des ganzen Stapels; der durch den Tisch zum Boden geleitete Anteil des Bankgewichts wird nicht getrennt."),
        T("Masses from the modelled 1:50 sections: spruce 450 kg/m3 (EN 338 C24 at outdoor moisture), steel 7850 kg/m3 with the fill of the 30 x 15 x 1.5 and 15 x 15 x 1.5 mm hollow sections and of the pressed 2.5 mm housings, PE caps 950 kg/m3; together 42.0 kg, in the range of 41 to 43 kg of catalogue-typical 220 x 50 cm sets.", "Massen aus den modellierten 1:50-Querschnitten: Fichte 450 kg/m3 (EN 338 C24 bei Außenfeuchte), Stahl 7850 kg/m3 mit der Füllung der Hohlprofile 30 x 15 x 1,5 und 15 x 15 x 1,5 mm und der gepressten 2,5-mm-Beschläge, PE-Kappen 950 kg/m3; zusammen 42,0 kg, im Bereich von 41 bis 43 kg katalogtypischer Garnituren 220 x 50 cm."),
    ],
    "clinical": False,
}

REFERENCES = [
    {"id": "en-581-1", "citation": "EN 581-1:2017 Outdoor furniture - Seating and tables for camping, domestic and contract use - Part 1: General safety requirements."},
    {"id": "en-581-2", "citation": "EN 581-2:2015 Outdoor furniture - Seating and tables for camping, domestic and contract use - Part 2: Mechanical safety requirements and test methods for seating (static seat and back loads, durability, stability)."},
    {"id": "en-581-3", "citation": "EN 581-3:2017 Outdoor furniture - Seating and tables for camping, domestic and contract use - Part 3: Mechanical safety requirements and test methods for tables (vertical static load on the top and the edge, stability)."},
    {"id": "en-1728-2012", "citation": "EN 1728:2012 Furniture - Seating - Test methods for the determination of strength and durability."},
    {"id": "en-16139-2013", "citation": "EN 16139:2013 Furniture - Strength, durability and safety - Requirements for non-domestic seating (level 2 for severe public use such as beer gardens)."},
    {"id": "en-338-2016", "citation": "EN 338:2016 Structural timber - Strength classes (C24: bending 24 MPa, E0,mean 11 GPa, mean density 420 kg/m3 at 12 percent moisture)."},
    {"id": "en-10219-2", "citation": "EN 10219-2:2019 Cold formed welded steel structural hollow sections - Part 2: Tolerances, dimensions and sectional properties (rectangular 30 x 15 x 1.5 and square 15 x 15 x 1.5 mm)."},
    {"id": "en-1991-1-1", "citation": "EN 1991-1-1:2002 Eurocode 1: Actions on structures - Part 1-1: General actions - Densities, self-weight, imposed loads for buildings (category C1 areas with tables, 3.0 kN/m2)."},
    {"id": "iso-10456-2007", "citation": "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (conductivity, density and specific heat of softwood, steel and polyethylene)."},
    {"id": "iso-7730-2005", "citation": "ISO 7730:2005 Ergonomics of the thermal environment - Analytical determination and interpretation of thermal comfort (clothing surface temperatures of seated persons around 28 to 32 degC)."},
    {"id": "incropera-2018", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley, Appendix A tables of total emissivity and solar absorptivity (galvanised and oxidised steel, paints)."},
    {"id": "wood-handbook-2021", "citation": "Forest Products Laboratory (2021). Wood Handbook: Wood as an Engineering Material, FPL-GTR-282, USDA Forest Service (density and moisture of spruce; chapter 16 on weathering: greying, checking and erosion of exposed softwood surfaces)."},
    {"id": "weathered-wood", "citation": "Williams, R. S. (2005). Weathering of wood. In Rowell, R. M. (ed.), Handbook of Wood Chemistry and Wood Composites, CRC Press, ch. 7 (UV degradation of lignin, greying, surface checks and erosion of about 6 mm per century on softwood); solar absorptances 0.45 fresh and 0.68 grey are authored-estimates."},
    {"id": "festzeltgarnitur-catalogues", "citation": "Catalogue-typical data of German DIY and rental listings of folding beer tent sets (Festzeltgarnitur / Bierzeltgarnitur) 2024-2026: table 220 x 50 cm, 75-77 cm high; two benches 220 x 25 cm, 47-50 cm high; spruce or fir boards on folding tubular steel trestles with locking struts; set weight about 41-43 kg. Type data, no product."},
]


REP_DECISIONS = {}
VOL_DECISIONS = {}


def build():
    """📄 The whole asset document."""
    asset = {
        "id": "used-beer-garden-set",
        "category": "furniture",
        "title": T("Used Beer Garden Set", "Gebrauchte Bierzeltgarnitur"),
        "summary": T(
            "A folding beer garden set after many seasons in a park beer garden: a 2.20 x 0.50 m spruce table 0.77 m high and two 2.20 x 0.25 m benches 0.49 m high on folding 30 x 15 mm tube trestles with locking struts. The table top and the rear bench are greyed, checked and carved, a beer ring and a burn on the top, chewing gum under its edge; the front bench carries a freshly planed replacement seat board; the left table trestle stands on a foot bar bowed 18 mm, the rear bench on a kinked upright and one missing foot cap; the green enamel flakes and the tube ends have rusted. End elevation, long side and plan; in service, with six guests, turned over for repair in the depot, folded and stacked.",
            "Eine Bierzeltgarnitur nach vielen Saisons in einem Parkbiergarten: ein Fichtentisch 2,20 x 0,50 m, 0,77 m hoch, und zwei Bänke 2,20 x 0,25 m, 0,49 m hoch, auf klappbaren Rohrböcken 30 x 15 mm mit Sperrspreizen. Tischplatte und hintere Bank sind vergraut, rissig und mit eingeritzten Buchstaben, ein Bierrand und eine Brandstelle auf der Platte, Kaugummi unter ihrer Kante; die vordere Bank trägt ein frisch gehobeltes Ersatzsitzbrett; der linke Tischbock steht auf einem um 18 mm durchgebogenen Fußrohr, die hintere Bank auf einem geknickten Stiel und mit einer fehlenden Fußkappe; der grüne Lack blättert, die Rohrenden sind verrostet. Stirnansicht, Längsseite und Grundriss; aufgestellt, mit sechs Gästen, zur Reparatur im Bauhof umgedreht, zusammengeklappt und gestapelt."),
        "tags": ["used", "park", "public-space", "dining", "seating", "beer-garden", "folding", "bench", "table", "timber", "spruce", "steel", "tube"],
        "provenance": {"author": "Semio staffage content (used park objects wave, row 06)", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES, "notes": T(
            "Side elevation = end view across the set (x across, z along the 2.20 m boards). Construction after the catalogue-typical German folding beer tent set: 25 mm glued spruce boards on two 22 x 45 (table) or 22 x 40 (bench) battens, at each end a folding trestle of 30 x 15 x 1.5 mm rectangular tube (top bar pinned in two pressed steel housings, two uprights, foot bar with PE caps) and a 15 x 15 mm strut pinned in a catch under the board and hooked behind the foot bar. Traces of use modelled as geometry where they exceed the smallest feature (worn arrises, the bowed foot bar, the kinked upright, the missing cap, the replacement board, rusted tube ends) and drawn as strokes below it (glue joints, checks, the end split, carved initials, beer rings, a cigarette burn, chewing gum). Every parameter is literature or authored-estimate; nothing is measured.",
            "Seitenansicht = Stirnansicht quer über die Garnitur (x quer, z entlang der 2,20-m-Bretter). Konstruktion nach der katalogtypischen deutschen Bierzeltgarnitur: 25-mm-verleimte Fichtenbretter auf zwei Leisten 22 x 45 (Tisch) bzw. 22 x 40 (Bank), an jedem Ende ein Klappbock aus Rechteckrohr 30 x 15 x 1,5 mm (oberes Rohr in zwei gepressten Stahlbeschlägen gelagert, zwei Stiele, Fußrohr mit PE-Kappen) und eine Spreize 15 x 15 mm, in einer Raste unter dem Brett gelagert und hinter das Fußrohr gehängt. Gebrauchsspuren als Geometrie, wo sie die kleinste Einzelheit überschreiten (abgeriebene Kanten, das durchgebogene Fußrohr, der geknickte Stiel, die fehlende Kappe, das Ersatzbrett, verrostete Rohrenden), darunter als Striche (Leimfugen, Risse, der Endriss, eingeritzte Buchstaben, Bierränder, eine Zigarettenbrandstelle, Kaugummi). Jeder Parameter ist Literatur oder Schätzwert; nichts ist gemessen.")},
        "dimensions": {"heightM": TABLE_TOP, "widthM": round(2 * (B_IN + B_W), 3), "depthM": 2 * L_TOP, "massKg": round(MASS_TOTAL, 1)},
        "materials": MATERIALS,
        "rig": {"kind": "mechanical", "root": "ground", "bones": BONES},
        "anchors": ANCHORS,
        "representations": {s: representation(s) for s in ("50", "100", "200", "500")},
        "symbols": [],
        "poses": POSES,
        "clips": CLIPS,
        "thermal": thermal_model(),
        "structure": STRUCTURE,
        "volumes": {s: volume(s) for s in ("50", "100", "200", "500")},
        "presentation": {"defaultPose": "in-service", "defaultClip": "unfold", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}


def rep(sil, ret, mer, omi, thin, hat, sty, ovl):
    """📝 The eight decisions of one representation."""
    return {"silhouette": T(*sil), "retained": T(*ret), "merged": T(*mer), "omitted": T(*omi), "thinElements": T(*thin), "hatching": T(*hat), "styles": T(*sty), "overlays": T(*ovl)}


OVERLAY = ("Every filled shape carries one region (weathered-timber, fresh-timber, painted-steel, galvanised-fittings, feet); strokes carry none.", "Jede gefüllte Form trägt genau eine Region (weathered-timber, fresh-timber, painted-steel, galvanised-fittings, feet); Striche keine.")
STYLE = ("Grey weathered boards 0.6-0.62, battens 0.5, the fresh replacement board light 0.82, green enamel 0.34, struts 0.3, zinc 0.42, rusted feet 0.2, PE caps near black.", "Graue verwitterte Bretter 0,6-0,62, Leisten 0,5, das frische Ersatzbrett hell 0,82, grüner Lack 0,34, Spreizen 0,3, Zink 0,42, verrostete Füße 0,2, PE-Kappen fast schwarz.")
REP_DECISIONS.update({
    "50": rep(("Beer garden set 22 x 15 mm on paper at 1:50, every part as a prism of its true section placed by its bone, so end view, long side and plan are one model: the end view shows the table board with its worn arrises over the trestle frame of 30 x 15 mm tube, the strut, the two benches with their trestles; the long side the boards, battens and the six trestles edge-on with their struts; the plan the three boards with the trestles reaching out under them.", "Bierzeltgarnitur 22 x 15 mm auf dem Papier bei 1:50, jedes Teil als Prisma seines wahren Querschnitts von seinem Knochen platziert, so dass Stirnansicht, Längsseite und Grundriss ein Modell sind: die Stirnansicht zeigt das Tischbrett mit abgeriebenen Kanten über dem Bockrahmen aus Rohr 30 x 15 mm, die Spreize, die zwei Bänke mit ihren Böcken; die Längsseite Bretter, Leisten und die sechs Böcke hochkant mit Spreizen; der Grundriss die drei Bretter mit den darunter vortretenden Böcken."),
               ("Worn top arrises (6 mm) of the old boards, the sharp-edged fresh front seat, battens, pressed hinge housings, strut catches, painted inverted U and rusted foot of every trestle, PE caps (one missing on the rear bench), the bowed left table foot bar (18 mm), the kinked right upright of the rear bench (12 mm out), struts.", "Abgeriebene obere Kanten (6 mm) der alten Bretter, der scharfkantige frische vordere Sitz, Leisten, gepresste Klappbeschläge, Spreizenrasten, lackiertes umgedrehtes U und verrosteter Fuß jedes Bocks, PE-Kappen (eine fehlt an der hinteren Bank), das durchgebogene linke Tischfußrohr (18 mm), der um 12 mm ausgeknickte rechte Stiel der hinteren Bank, Spreizen."),
               ("Glued board strips of each top are one board with their glue joints as strokes; the oval sections of real legs are the rectangular hollow sections they are made of.", "Die verleimten Lamellen jeder Platte sind ein Brett mit Leimfugen als Strichen; die Rohre sind die Rechteckhohlprofile, aus denen sie bestehen."),
               ("Screws and rivets (5 mm heads), the spring of the strut catch, the wall of the hollow tubes.", "Schrauben und Niete (5-mm-Köpfe), die Feder der Spreizenraste, die Wand der Hohlrohre."),
               ("Traces below 7.5 mm are strokes inside the parts: glue joints, the split at the right table end, surface checks, carved initials and a heart, three beer rings, a cigarette burn on the top, chewing gum under the top edge, checks on the rear seat.", "Spuren unter 7,5 mm sind Striche innerhalb der Teile: Leimfugen, der Riss am rechten Tischende, Oberflächenrisse, eingeritzte Initialen und ein Herz, drei Bierränder, eine Zigarettenbrandstelle auf der Platte, Kaugummi unter der Plattenkante, Risse im hinteren Sitz."),
               ("Grain lines every 0.6 mm along the boards in plan and long side (end grain unhatched), stipple on the flaking enamel, cross hatch on the rusted tube ends.", "Maserungslinien alle 0,6 mm längs der Bretter in Grundriss und Längsseite (Hirnholz ohne Schraffur), Punkte auf dem abblätternden Lack, Kreuzschraffur auf den verrosteten Rohrenden."), STYLE, OVERLAY),
    "100": rep(("Set 22 x 15 mm halved: boards with 4 mm arrises, battens, housings, trestle frames of 30 x 15 mm tube with their rusted feet, caps and struts.", "Garnitur 22 x 15 mm halbiert: Bretter mit 4-mm-Kanten, Leisten, Beschläge, Bockrahmen aus Rohr 30 x 15 mm mit verrosteten Füßen, Kappen und Spreizen."),
                ("The bowed foot bar, the kinked upright, the missing cap, the fresh replacement board, the rust at the feet.", "Das durchgebogene Fußrohr, der geknickte Stiel, die fehlende Kappe, das frische Ersatzbrett, der Rost an den Füßen."),
                ("Board arrises simplified to one chamfer; the 7.5 mm strut catches thickened to 15 mm around the strut pins.", "Brettkanten zu einer Fase vereinfacht; die 7,5-mm-Spreizenrasten um die Spreizenbolzen auf 15 mm verdickt."),
                ("Checks, initials, gum and burn (below 15 mm); screws.", "Risse, Initialen, Kaugummi und Brandstelle (unter 15 mm); Schrauben."),
                ("Glue joints, the end split and the beer rings stay as strokes; tubes and struts at their true 15 mm.", "Leimfugen, der Endriss und die Bierränder bleiben als Striche; Rohre und Spreizen mit ihren wahren 15 mm."),
                ("Grain every 0.8 mm along the boards, stipple on the rusted feet.", "Maserung alle 0,8 mm längs der Bretter, Punkte auf den verrosteten Füßen."), STYLE, OVERLAY),
    "200": rep(("Set 11 x 7.5 mm: each top a 30 mm board over two 37 mm battens, each trestle a frame of 30 x 30 mm members with its rusted foot band, the hinge housings as one 30 mm block between the battens.", "Garnitur 11 x 7,5 mm: jede Platte ein 30-mm-Brett über zwei 37-mm-Leisten, jeder Bock ein Rahmen aus 30 x 30 mm mit seinem verrosteten Fußband, die Klappbeschläge als ein 30-mm-Block zwischen den Leisten."),
                ("Table and seat heights, the board widths, the trestle frames in their places, the fresh front seat in its own tone.", "Tisch- und Sitzhöhe, die Brettbreiten, die Bockrahmen an ihrem Ort, der frische vordere Sitz in eigenem Ton."),
                ("Boards thickened from 25 to 30 mm, tubes from 15 to 30 mm thick, caps into the foot bars, the two housings of each trestle into one block.", "Bretter von 25 auf 30 mm, Rohre von 15 auf 30 mm verdickt, Kappen in die Fußrohre, die zwei Beschläge jedes Bocks zu einem Block."),
                ("Strut catches, the 18 mm bow of the foot bar and the 12 mm kink of the upright, every stroke trace except the glue joints.", "Spreizenrasten, die 18-mm-Durchbiegung des Fußrohrs und der 12-mm-Knick des Stiels, jede Strichspur außer den Leimfugen."),
                ("Every member at least 30 mm except the 15 mm struts, which keep their true section as slender bars.", "Jedes Glied mindestens 30 mm außer den 15-mm-Spreizen, die als schlanke Stäbe ihren wahren Querschnitt behalten."),
                ("Grain every 1.0 mm along the boards in plan only.", "Maserung alle 1,0 mm längs der Bretter nur im Grundriss."), STYLE, OVERLAY),
    "500": rep(("Set sign 4.4 x 2.2 mm: the table as one 75 mm slab of top and battens, each bench as one 75 mm slab, each table trestle as two 75 mm legs on a 75 mm rusted foot band with the strut in the open frame, each bench trestle as one 75 mm envelope plate on its foot band.", "Garniturzeichen 4,4 x 2,2 mm: der Tisch als eine 75-mm-Scheibe aus Platte und Leisten, jede Bank als eine 75-mm-Scheibe, jeder Tischbock als zwei 75-mm-Beine auf einem 75-mm-Rostfußband mit der Spreize im offenen Rahmen, jeder Bankbock als eine 75-mm-Hüllscheibe auf seinem Fußband."),
                ("Overall size, table and seat height, the three boards side by side, the six trestles.", "Gesamtgröße, Tisch- und Sitzhöhe, die drei Bretter nebeneinander, die sechs Böcke."),
                ("Top, battens and housings into the table slab; seat and battens into the bench slabs, which reach 55 mm in to the table edge so the 55 mm gap between them closes on paper; each table trestle frame into two 75 mm legs and a foot band reaching 30 mm out to the table edge, each bench trestle frame into one envelope plate reaching 55 mm in to it (its opening would be under the 0.15 mm gap); the bench trestles 25 mm out so their plates meet the table plates in the long view.", "Platte, Leisten und Beschläge zur Tischscheibe; Sitz und Leisten zu den Bankscheiben, die 55 mm bis an die Tischkante heranreichen, so dass die 55-mm-Lücke auf dem Papier schließt; jeder Tischbockrahmen zu zwei 75-mm-Beinen und einem Fußband, die 30 mm bis an die Tischkante reichen, jeder Bankbockrahmen zu einer Hüllscheibe, die 55 mm an sie heranreicht (ihre Öffnung läge unter der 0,15-mm-Lücke); die Bankböcke 25 mm nach außen, so dass ihre Scheiben in der Längsansicht an die Tischscheiben stoßen."),
                ("Bench struts, caps, every trace of use (all below 75 mm).", "Bankspreizen, Kappen, jede Gebrauchsspur (alle unter 75 mm)."),
                ("Every part at least 75 mm except the table struts, which keep their true 15 mm section as slender bars.", "Jedes Teil mindestens 75 mm außer den Tischspreizen, die als schlanke Stäbe ihren wahren 15-mm-Querschnitt behalten."), ("None.", "Keine."), STYLE, OVERLAY),
})


def vol(body, thermal, structure):
    """📝 The three decisions of one volume."""
    return [T(*body), T(*thermal), T(*structure)]


VOL_DECISIONS.update({
    "50": vol(("Body for 7.5 mm: every board, batten, housing, catch, trestle frame, rusted foot, cap and strut as an extrusion of its true section on its bone, identical with the 2D prisms; worn arrises in the board profiles, the bowed foot bar, the kinked upright, the missing cap and the fresh replacement board are geometry.", "Körper für 7,5 mm: jedes Brett, jede Leiste, jeder Beschlag, jede Raste, jeder Bockrahmen, jeder verrostete Fuß, jede Kappe und jede Spreize als Extrusion ihres wahren Querschnitts auf ihrem Knochen, identisch mit den 2D-Prismen; abgeriebene Kanten in den Brettprofilen, das durchgebogene Fußrohr, der geknickte Stiel, die fehlende Kappe und das frische Ersatzbrett sind Geometrie."),
              ("Thermal: every body solid is the surface of its region (references), five regions with one layer each; the hollow tubes are modelled as their outer envelope (their bore is closed at the ends), no hidden region, no emitter.", "Thermik: jeder Körper ist die Oberfläche seiner Region (Verweise), fünf Regionen mit je einer Schicht; die Hohlrohre sind als ihre Außenhülle modelliert (die Bohrung ist an den Enden geschlossen), keine verdeckte Region, kein Emitter."),
              ("Structure: boards, battens, frames, feet, caps and struts are the members of their bones; the hinge housings are the joints of the trestles, the catches the joints of the struts; the supports are the foot contacts of the pose, so the list has no support solid; a mass marker at the centre of every bone.", "Tragwerk: Bretter, Leisten, Rahmen, Füße, Kappen und Spreizen sind die Stäbe ihrer Knochen; die Klappbeschläge sind die Gelenke der Böcke, die Rasten die Gelenke der Spreizen; die Auflager sind die Fußkontakte der Pose, die Liste hat daher keinen Auflagerkörper; eine Massenmarke im Schwerpunkt jedes Knochens.")),
    "100": vol(("Body for 15 mm: boards with one 4 mm chamfer, battens, housings, trestle frames and feet of 30 x 15 mm, caps and struts at their true section; the 7.5 mm catches thickened to 15 mm around the strut pins, the bowed bar and the kinked upright stay (18 and 12 mm).", "Körper für 15 mm: Bretter mit einer 4-mm-Fase, Leisten, Beschläge, Bockrahmen und Füße 30 x 15 mm, Kappen und Spreizen in wahrem Querschnitt; die 7,5-mm-Rasten um die Spreizenbolzen auf 15 mm verdickt, das durchgebogene Rohr und der geknickte Stiel bleiben (18 und 12 mm)."),
               ("Thermal: references to the body, five regions.", "Thermik: Verweise auf den Körper, fünf Regionen."),
               ("Structure: as 1:50; the catches are the joints of the struts at the board.", "Tragwerk: wie 1:50; die Rasten sind die Gelenke der Spreizen am Brett.")),
    "200": vol(("Body for 30 mm: 30 mm boards over 37 mm battens, one 30 mm housing block per trestle, trestle frames of 30 x 30 mm with the caps merged into the foot bars, the struts as slender 15 mm bars; catches, bow and kink left out.", "Körper für 30 mm: 30-mm-Bretter über 37-mm-Leisten, ein 30-mm-Beschlagblock je Bock, Bockrahmen 30 x 30 mm mit in die Fußrohre gezogenen Kappen, die Spreizen als schlanke 15-mm-Stäbe; Rasten, Durchbiegung und Knick entfallen."),
               ("Thermal: references to the body, five regions.", "Thermik: Verweise auf den Körper, fünf Regionen."),
               ("Structure: boards, battens, frames and the slender struts as members, housings as joints, masses; the feet stand on the ground.", "Tragwerk: Bretter, Leisten, Rahmen und die schlanken Spreizen als Stäbe, Beschläge als Gelenke, Massen; die Füße stehen auf dem Boden.")),
    "500": vol(("Body for 75 mm: the table as one 75 mm slab with four 75 mm housing blocks under it, each bench as one 75 mm slab reaching 55 mm in to the table edge, each table trestle as two 75 mm painted legs on a 75 mm rusted foot band with its slender 15 mm strut, each bench trestle as a 75 mm envelope plate on its foot band; the bench trestles sit 25 mm further out to meet the table trestles in the long view.", "Körper für 75 mm: der Tisch als eine 75-mm-Scheibe mit vier 75-mm-Beschlagblöcken darunter, jede Bank als eine 75-mm-Scheibe, die 55 mm bis an die Tischkante reicht, jeder Tischbock als zwei lackierte 75-mm-Beine auf einem 75-mm-Rostfußband mit seiner schlanken 15-mm-Spreize, jeder Bankbock als 75-mm-Hüllscheibe auf seinem Fußband; die Bankböcke sitzen 25 mm weiter außen, um in der Längsansicht an die Tischböcke zu stoßen."),
               ("Thermal: references to the body, five regions; at this scale only the sum of the areas is compared.", "Thermik: Verweise auf den Körper, fünf Regionen; in diesem Maßstab wird nur die Summe der Flächen verglichen."),
               ("Structure: slabs, legs and foot bands as members, housing blocks as joints, the struts as slender 15 mm members, masses; the foot bands stand on the ground.", "Tragwerk: Scheiben, Beine und Fußbänder als Stäbe, Beschlagblöcke als Gelenke, die Spreizen als schlanke 15-mm-Stäbe, Massen; die Fußbänder stehen auf dem Boden.")),
})


if __name__ == "__main__":
    doc = build()
    with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf-8") as fh:
        json.dump(doc, fh, ensure_ascii=False, indent=1)
        fh.write("\n")
    print("[DEBUG] written", os.path.getsize(os.path.join(HERE, "🔣️.json")), "bytes; mass", round(MASS_TOTAL, 2))
