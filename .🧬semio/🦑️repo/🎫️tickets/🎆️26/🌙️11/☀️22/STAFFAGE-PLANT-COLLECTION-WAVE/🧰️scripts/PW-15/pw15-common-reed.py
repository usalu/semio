"""🌾 PW-15, vegetation/common-reed: a flowering patch of common reed (Phragmites australis), 3 m tall: nine culms from a creeping rhizome with broad distichous leaves and dense nodding panicles, three young leafy shoots and last year's broken culms in a litter mat. Stated by hand in world space (x ahead, y up, z left, metres) per scale and built from the round-3 miscanthus plant parts (`pw15-lib.Plant`, kit `K`), solids, outlines, strips and drawings.

Reference: Flora of North America 25 (Saltonstall et al. 2003): culms 1-4 m tall, 0.5-1.5 cm thick, blades 15-40 cm x 2-4 cm; Kew GrassBase: panicle 20-50 cm long, 6-15 cm wide; Flora of New Zealand V: blades 40-60 x 1-3 cm, panicle 20-30 cm, erect or finally drooping, purplish to brownish.
Usage: repo tools exec python -- pw15-common-reed.py
See: ../../📝️PW-15.md
"""
import importlib.util
import math
from pathlib import Path
from types import SimpleNamespace

spec_ = importlib.util.spec_from_file_location("pw15_lib", Path(__file__).resolve().parent / "pw15-lib.py")
L = importlib.util.module_from_spec(spec_)
spec_.loader.exec_module(L)
K, M, t = L.K, L.M, L.t

ID = "common-reed"
HEIGHT, WIDTH, DEPTH, MASS = 3.0, 1.6, 1.4, 6.0
TARGET = {"x": WIDTH / 2, "z": DEPTH / 2, "y": HEIGHT}
CROWN = 0.08
TOP = ("2", "3")
CULM = {
    "1": ((0.25, 0.06, 0.1), -3, -1, 1.45, 0.92),
    "2": ((-0.05, 0.06, -0.15), -1, 0, 1.6, 1.0),
    "3": ((0.05, 0.06, 0.2), 1, 0, 1.6, 1.0),
    "4": ((-0.28, 0.06, 0.05), 3, 1, 1.45, 0.9),
    "5": ((0.36, 0.06, -0.26), -3, -1, 1.35, 0.85),
    "6": ((-0.4, 0.06, -0.22), 3, 1, 1.35, 0.85),
    "7": ((0.15, 0.06, 0.36), -2, -1, 1.5, 0.95),
    "8": ((-0.2, 0.06, 0.32), 2, 1, 1.42, 0.9),
    "9": ((0.0, 0.06, -0.38), 0, 0, 1.5, 0.92),
}
CULMS = list(CULM)
SHOOT = {"1": ((0.3, 0.06, -0.08), -6, 1.55), "2": ((-0.15, 0.06, 0.15), 4, 1.45), "3": ((-0.05, 0.06, -0.3), 2, 1.6)}
LATERAL = {}
for _c, (_base, *_rest) in CULM.items():
    _s = 0.06 * _base[2] / 0.38
    LATERAL.update({f"culm-{_c}a": round(_s, 3), f"culm-{_c}b": round(0.7 * _s, 3), f"panicle-{_c}": round(0.3 * _s, 3)})
for _c, (_base, *_rest) in SHOOT.items():
    LATERAL[f"shoot-{_c}"] = round(0.15 * _base[2] / 0.3, 3)

LEAF_PROFILE = [(0.0, 0.55), (0.06, 0.9), (0.2, 1.0), (0.45, 0.92), (0.68, 0.7), (0.85, 0.42), (0.95, 0.18), (1.0, 0.04)]


def nodes(count, low, high, length0, length1, width0, width1, div, end):
    out = []
    for i in range(count):
        s = i / (count - 1)
        out.append((low + (high - low) * s, 180 * (i % 2) + 17 * i, div, end - 18 * s, length0 + (length1 - length0) * s, width0 + (width1 - width0) * s))
    return out


S = {
    50: {"leaves": nodes(10, 0.3, 0.95, 0.48, 0.28, 0.022, 0.013, 48, 112), "shoot_leaves": nodes(7, 0.25, 1.0, 0.44, 0.3, 0.02, 0.014, 40, 108), "thick": 0.0007, "points": 9,
         "culm_r": (0.0055, 0.0045, 0.0032), "shoot_r": (0.0045, 0.0025), "rachis": (0.3, 0.0022, 0.0012),
         "branches": [(0.0, 0, 34, 70, 0.16), (0.08, 140, 30, 66, 0.17), (0.16, 280, 28, 62, 0.17), (0.24, 60, 26, 60, 0.16), (0.32, 200, 25, 56, 0.16), (0.4, 340, 24, 52, 0.15), (0.48, 120, 22, 48, 0.15), (0.56, 260, 20, 44, 0.14), (0.64, 40, 18, 40, 0.13), (0.72, 180, 16, 36, 0.12), (0.8, 320, 14, 30, 0.1), (0.88, 100, 12, 26, 0.1), (0.94, 240, 9, 20, 0.09), (0.04, 210, 32, 68, 0.16), (0.2, 350, 27, 61, 0.165), (0.36, 130, 24, 54, 0.155)],
         "branch_r": 0.002, "nod": 0.55,
         "dead": [(0.2, -0.35, 0.9, 14, 30), (-0.35, 0.2, 0.75, 20, 160), (0.5, 0.15, 0.6, 26, 330), (-0.1, -0.05, 0.85, 9, 250), (0.05, 0.42, 0.55, 22, 300), (-0.55, -0.05, 0.7, 18, 200), (0.38, -0.1, 0.5, 24, 300), (-0.25, -0.45, 0.65, 22, 220)],
         "dead_r": 0.0045, "mat": [(0.002, 0.66), (0.03, 0.64), (0.06, 0.56), (0.08, 0.4), (0.09, 0.2)], "strip": (0.25, 0.75, 0.32)},
    100: {"leaves": nodes(6, 0.34, 0.92, 0.5, 0.32, 0.023, 0.015, 46, 112), "shoot_leaves": nodes(4, 0.3, 1.0, 0.44, 0.34, 0.02, 0.016, 40, 108), "thick": 0.0009, "points": 8,
          "culm_r": (0.0055, 0.0045, 0.0035), "shoot_r": (0.0045, 0.003),
          "panicle": [(0.0, 0.01, 0.01), (0.06, 0.026, 0.018), (0.15, 0.03, 0.02), (0.24, 0.025, 0.018), (0.32, 0.008, 0.008)], "nod": 30,
          "dead": [(0.2, -0.35, 0.9, 14, 30), (-0.35, 0.2, 0.75, 20, 160), (-0.1, -0.05, 0.85, 9, 250), (-0.55, -0.05, 0.7, 18, 200), (0.38, -0.1, 0.5, 24, 300)],
          "dead_r": 0.0045, "mat": [(0.004, 0.66), (0.05, 0.6), (0.08, 0.4), (0.09, 0.2)], "strip": (0.25, 0.75, 0.3)},
    200: {"leaves": nodes(4, 0.42, 0.9, 0.56, 0.4, 0.026, 0.019, 48, 108), "shoot_leaves": nodes(3, 0.4, 1.0, 0.5, 0.42, 0.023, 0.02, 40, 104), "thick": 0.0015, "points": 8,
          "culm_r": (0.006, 0.005, 0.004), "shoot_r": (0.005, 0.004),
          "panicle": [(0.0, 0.016, 0.016), (0.06, 0.03, 0.02), (0.16, 0.036, 0.022), (0.25, 0.03, 0.02), (0.33, 0.014, 0.014)], "nod": 30,
          "mat": [(0.004, 0.66), (0.06, 0.56), (0.09, 0.22)], "strip": (0.1, 0.9, 0.012)},
    500: {"leaves": nodes(3, 0.5, 0.88, 0.66, 0.5, 0.03, 0.023, 48, 106), "shoot_leaves": nodes(2, 0.5, 1.0, 0.58, 0.5, 0.027, 0.023, 40, 102), "thick": 0.004, "points": 7,
          "culm_r": (0.0065, 0.0055, 0.0045), "shoot_r": (0.0055, 0.0045),
          "panicle": [(0.0, 0.04, 0.04), (0.08, 0.06, 0.042), (0.18, 0.065, 0.042), (0.28, 0.05, 0.04), (0.34, 0.04, 0.04)], "nod": 30,
          "mat": [(0.004, 0.66), (0.07, 0.56), (0.095, 0.25)], "strip": (0.1, 0.9, 0.014)},
}

LEAF = {"material": "reed-leaf", "role": "body", "tone": 0.5}
STEM = {"material": "reed-culm", "role": "body", "tone": 0.38}
LITTER = {"material": "reed-litter", "role": "body", "tone": 0.3}
PANICLE = {"material": "reed-panicle", "role": "detail", "tone": 0.62}
PLANT = SimpleNamespace(BUNDLES=[], CULMS=CULMS, LATERAL=LATERAL, BLADE_PROFILE=LEAF_PROFILE, CROWN_LENGTH=CROWN)


def azimuth_of(v):
    return math.degrees(math.atan2(v[2], v[0]))


def axis(rig, bone):
    tip = K.add(rig.tip(bone), (0.0, 0.0, LATERAL[bone]))
    return K.unit(K.sub(tip, rig.origin(bone)))


def culm_paths(rig, c, n):
    a, b, p = f"culm-{c}a", f"culm-{c}b", f"panicle-{c}"
    o, j, e = rig.origin(a), rig.origin(b), rig.origin(p)
    return [K.lerp(o, j, i / (n - 1)) for i in range(n)], [K.lerp(j, e, i / (n - 1)) for i in range(n)]


def on_culm(path_a, path_b, s):
    """📍 The point, tangent and bone at the share s of the whole culm (lower and upper segment together)."""
    la, lb = L.arc_length(path_a), L.arc_length(path_b)
    run = s * (la + lb)
    if run <= la:
        return K.along(path_a, run / la), K.unit(K.sub(path_a[-1], path_a[0])), "a"
    return K.along(path_b, (run - la) / lb), K.unit(K.sub(path_b[-1], path_b[0])), "b"


def leaf_rows(plant, prefix, bone_of, path_a, path_b, lean, rows, scale, sheet_profile, region=True):
    made = []
    for i, (s, dphi, div, end, length, width) in enumerate(rows):
        at, tangent, seg = on_culm(path_a, path_b, s)
        azimuth = lean + dphi
        d0 = K.unit(K.add(K.mul(tangent, math.cos(math.radians(div))), K.mul(K.direction(90, azimuth), math.sin(math.radians(div)))))
        d1 = K.direction(end, azimuth)
        made.append(plant.leaf(f"{prefix}-{i + 1}", bone_of(seg), at, d0, d1, length, 1.6, width, S[scale]["thick"], sheet_profile, S[scale]["points"], "leaves", LEAF, reach=i < 2, margin=0.012))
    return made


def culms_and_shoots(plant, rig, scale, sheet_profile, rows_key, shoot_key):
    table = S[scale]
    n = 7 if scale <= 100 else 5
    leads = []
    for c in CULMS:
        path_a, path_b = culm_paths(rig, c, n)
        ra, rj, rb = table["culm_r"]
        plant.round(f"culm-{c}a", f"culm-{c}a", path_a, [ra + (rj - ra) * i / (n - 1) for i in range(n)], "culms", STEM)
        plant.round(f"culm-{c}b", f"culm-{c}b", path_b, [rj + (rb - rj) * i / (n - 1) for i in range(n)], "culms", STEM)
        plant.stems[f"culm-{c}a"], plant.stems[f"culm-{c}b"] = path_a, path_b
        lean = azimuth_of(K.sub(path_b[-1], path_a[0])) + 37 * int(c)
        made = leaf_rows(plant, f"leaf-{c}", lambda seg, c=c: f"culm-{c}{seg}", path_a, path_b, lean, table[rows_key], scale, sheet_profile)
        leads.append(made[len(made) // 2])
    for c, (base, angle, length) in SHOOT.items():
        bone = f"shoot-{c}"
        o, tip = rig.origin(bone), K.add(rig.tip(bone), (0.0, 0.0, LATERAL[bone]))
        path = [K.lerp(o, tip, i / (n - 1)) for i in range(n)]
        r0, r1 = table["shoot_r"]
        plant.round(bone, bone, path, [r0 + (r1 - r0) * i / (n - 1) for i in range(n)], "culms", STEM)
        plant.stems[bone] = path
        half = len(path) // 2
        made = leaf_rows(plant, f"leaf-s{c}", lambda seg, bone=bone: bone, path[: half + 1], path[half:], 90 * int(c), table[shoot_key], scale, sheet_profile)
        leads.append(made[len(made) // 2])
    return leads


def dome_height(rows, radius):
    """📏 The height of the litter dome at a radius (its rows are height and radius, the radius falling with height)."""
    for (u0, r0), (u1, r1) in zip(rows, rows[1:]):
        if r1 <= radius <= r0:
            return u0 + (u1 - u0) * (r0 - radius) / (r0 - r1)
    return rows[-1][0] if radius < rows[-1][1] else rows[0][0]


def mat(plant, scale):
    plant.dome("litter", S[scale]["mat"], LITTER)


def dead_culms(plant, scale):
    for i, (x, z, h, tilt, az) in enumerate(S[scale].get("dead", [])):
        foot = (x, dome_height(S[scale]["mat"], math.hypot(x, z)) - 0.004, z)
        tip = K.add(foot, K.mul(K.direction(tilt, az), h))
        r = S[scale]["dead_r"]
        plant.part(id=f"dead-culm-{i + 1}", bone="root-plate", region="culms", look=LITTER, capsule=(foot, tip, r), slender=True, width=2 * r)


def panicle_mass(plant, rig, c, scale, fit):
    table = S[scale]
    p = f"panicle-{c}"
    base, ax = rig.origin(p), axis(rig, p)
    lean = azimuth_of(K.sub(base, rig.origin(f"culm-{c}a")))
    rows = table["panicle"]
    length = rows[-1][0] * (fit if c in TOP else 1.0)
    nod = K.unit(K.add(K.mul(ax, math.cos(math.radians(table["nod"]))), K.mul(K.direction(90, lean), math.sin(math.radians(table["nod"])))))
    points = K.arch(base, ax, nod, length, 1.3, 7)
    plant.stems[p] = [K.lerp(points[0], K.along(points, 0.5), i / 3) for i in range(4)]
    side = K.unit(K.cross(ax, (0.0, 0.0, 1.0)))
    for i, (opening, around, share) in enumerate(table.get("fan", [(0, 0, 1.0)])):
        radial = K.add(K.mul(side, math.cos(math.radians(around))), K.mul((0.0, 0.0, 1.0), math.sin(math.radians(around))))
        d0 = K.unit(K.add(K.mul(ax, math.cos(math.radians(opening))), K.mul(radial, math.sin(math.radians(opening)))))
        d1 = K.unit(K.add(nod, K.mul(radial, 1.6 * math.sin(math.radians(opening)))))
        path = points if i == 0 else K.arch(base, d0, d1, length * share, 1.3, 7)
        plant.mass(p if i == 0 else f"{p}-{i + 1}", p, path, M.section_table([(r[0] / rows[-1][0], r[1], r[2]) for r in rows], 7), lean, "panicles", PANICLE)


def panicle_branches(plant, rig, c, fit):
    """🌾 1:50: the panicle axis and sixteen hairy branches that leave it at 9 to 34 degrees and nod to the side of the culm's lean."""
    table = S[50]
    p = f"panicle-{c}"
    base, ax = rig.origin(p), axis(rig, p)
    lean = azimuth_of(K.sub(base, rig.origin(f"culm-{c}a")))
    length, r0, r1 = table["rachis"]
    length *= fit if c in TOP else 1.0
    rachis = K.arch(base, ax, K.unit(K.add(ax, K.mul(K.direction(90, lean), 0.5))), length, 1.5, 5)
    plant.round(f"rachis-{c}", p, rachis, [r0 + (r1 - r0) * i / 4 for i in range(5)], "panicles", PANICLE)
    plant.stems[p] = rachis
    side = K.unit(K.cross(ax, (0.0, 0.0, 1.0)))
    for i, (s, around, opening, end, blen) in enumerate(table["branches"]):
        radial = K.add(K.mul(side, math.cos(math.radians(around))), K.mul((0.0, 0.0, 1.0), math.sin(math.radians(around))))
        d0 = K.unit(K.add(K.mul(ax, math.cos(math.radians(opening))), K.mul(radial, math.sin(math.radians(opening)))))
        out = K.unit(K.add(K.mul((radial[0], 0.0, radial[2]), 1 - table["nod"]), K.mul(K.direction(90, lean), table["nod"])))
        d1 = K.unit(K.add(K.mul(out, math.sin(math.radians(end))), K.mul(K.UP, math.cos(math.radians(end)))))
        pts = K.arch(K.along(rachis, s), d0, d1, blen * (fit if c in TOP else 1.0), 1.6, 6)
        plant.round(f"branch-{c}-{i + 1}", p, pts, [table["branch_r"] * (1 - 0.5 * q / 5) for q in range(6)], "panicles", PANICLE)


def make(scale):
    def run(rig):
        fit = 1.0
        for _ in range(8):
            plant = build(rig, scale, fit)
            fit *= 1 + (TARGET["y"] - M.extent(plant)["y+"]) / 0.35
        return build(rig, scale, fit)
    return run


def build(rig, scale, fit):
    plant = L.Plant(rig, scale, {"y": fit})
    mat(plant, scale)
    dead_culms(plant, scale)
    leads = culms_and_shoots(plant, rig, scale, LEAF_PROFILE, "leaves", "shoot_leaves")
    for c in CULMS:
        if scale == 50:
            panicle_branches(plant, rig, c, fit)
        else:
            panicle_mass(plant, rig, c, scale, fit)
    for i, part in enumerate(leads):
        plant.strips.append((i + 1, part, S[scale]["strip"]))
    return plant


def bones():
    rows = []
    for c, (base, angle, angle_b, la, lb) in CULM.items():
        rows.append({"id": f"culm-{c}a", "parent": "root-plate", "base": base, "angle": angle, "length": la, "limit": 12, "kg": 0.16, "title": t(f"Culm {c}, lower", f"Halm {c}, unten"), "section": L.circle_section(0.011, "reed-culm", "Solid-equivalent section, d = 1.1 cm", "Vollquerschnitt-Äquivalent, d = 1,1 cm")})
        rows.append({"id": f"culm-{c}b", "parent": f"culm-{c}a", "angle": angle_b, "length": lb, "limit": 22, "kg": 0.08, "title": t(f"Culm {c}, upper", f"Halm {c}, oben"), "section": L.circle_section(0.008, "reed-culm", "Solid-equivalent section, d = 8 mm", "Vollquerschnitt-Äquivalent, d = 8 mm")})
        rows.append({"id": f"panicle-{c}", "parent": f"culm-{c}b", "angle": 0, "length": 0.32, "limit": 30, "kg": 0.03, "com": 0.4, "distribution": "point", "title": t(f"Panicle {c}", f"Rispe {c}")})
    for c, (base, angle, length) in SHOOT.items():
        rows.append({"id": f"shoot-{c}", "parent": "root-plate", "base": base, "angle": angle, "length": length, "limit": 20, "kg": 0.12, "title": t(f"Young leafy shoot {c}", f"Junger Blatttrieb {c}"), "section": L.circle_section(0.009, "reed-culm", "Solid-equivalent section, d = 9 mm", "Vollquerschnitt-Äquivalent, d = 9 mm")})
    return rows


BONES = bones()
ROOT_KG = round(MASS - sum(r["kg"] for r in BONES), 4)


def member_radius(bone, scale):
    if bone.startswith("culm"):
        return 0.005 if bone.endswith("a") else 0.0036
    if bone.startswith("panicle"):
        return 0.002
    return 0.004


def masses(scale):
    if scale == 500:
        return {}
    r = {50: 0.012, 100: 0.012, 200: 0.018}[scale]
    return {f"panicle-{c}": (0.6, r) for c in CULMS}


AREAS = {}


def thermal(r50, anchors):
    leaf = round(r50["one_sided_leaf_m2"], 2)
    lai = round(leaf / (math.pi * DIMS["widthM"] / 2 * DIMS["depthM"] / 2), 2)
    return {
        "environment": {"airTempC": 24, "meanRadiantTempC": 27, "relativeHumidity": 0.65, "windMS": 2, "windDirectionDeg": 0, "solarWM2": 750, "pressurePa": 101325, "diffuseFraction": 0.15},
        "regions": [
            {"id": "leaves", "title": t("Leaf blades", "Blattspreiten"), "kind": "foliage", "model": "leaf", "material": "reed-leaf", "areaM2": leaf, "areaBasis": "exchange", "characteristicLengthM": 0.025, "geometry": "vertical-plate", "skyViewFactor": 0.55, "layers": [], "heatCapacityJM2K": 600, "leaf": {"stomatalConductanceMaxMS": 0.01, "leafAreaIndex": lai, "photosynthesis": "c3"}, "source": "authored-estimate", "reference": "jones-2014"},
            {"id": "panicles", "title": t("Panicles", "Rispen"), "kind": "surface", "model": "passive", "material": "reed-panicle", "areaM2": AREAS.get("panicles", round(r50["areas"]["panicles"], 2)), "characteristicLengthM": 0.08, "geometry": "cylinder", "skyViewFactor": 0.8, "layers": [], "heatCapacityJM2K": 400, "source": "authored-estimate", "reference": "jones-2014"},
            {"id": "culms", "title": t("Culms and litter", "Halme und Streu"), "kind": "bark", "model": "passive", "material": "reed-culm", "areaM2": AREAS.get("culms", round(r50["areas"]["culms"], 2)), "characteristicLengthM": 0.01, "geometry": "cylinder", "skyViewFactor": 0.45, "layers": [], "heatCapacityJM2K": 3000, "source": "authored-estimate", "reference": "jones-2014"},
        ],
        "emitters": [{"id": "transpiration-leaves", "kind": "transpiration", "title": t("Transpiration of the leaves (distributed over the patch)", "Transpiration der Blätter (über den Bestand verteilt)"), "anchor": anchors[0], "anchors": anchors, "directionDeg": 90, "diameterM": 0.5, "region": "leaves", "source": "authored-estimate", "reference": "monteith-unsworth-2013"}],
        "assumptions": [
            t("Leaf regions use the one-sided area of the 1:50 leaves; the leaf area index is that area over the ellipse of the plan extent (a sparse stand edge, far below the leaf area index of 4 to 8 of a closed reed bed). The maximum stomatal conductance (10 mm/s) is an authored estimate for a well-watered C3 wetland grass, guided by Jones (2014). Aperture and night closure are the engine's business, not authored here.", "Blattregionen verwenden die einseitige Fläche der Blätter von 1:50; der Blattflächenindex ist diese Fläche bezogen auf die Ellipse der Grundrissausdehnung (ein lockerer Bestandsrand, weit unter dem Blattflächenindex von 4 bis 8 eines geschlossenen Röhrichts). Die maximale Spaltöffnungsleitfähigkeit (10 mm/s) ist eine Autorenschätzung für ein gut wasserversorgtes C3-Sumpfgras, angelehnt an Jones (2014). Öffnung und nächtliches Schließen sind Sache der Engine und werden hier nicht vorgegeben."),
            t("Culms, litter and panicles are passive; the culm region's areal heat capacity is that of green culm tissue (about 3 MJ per m3 and K) times an effective depth of 1 mm (hollow culm wall). The roots stand in wet ground; the soil and water surface below are not part of the asset. No surface temperature is authored.", "Halme, Streu und Rispen sind passiv; die flächenbezogene Wärmekapazität der Halmregion ist die von frischem Halmgewebe (etwa 3 MJ je m3 und K) mal einer wirksamen Tiefe von 1 mm (hohle Halmwand). Die Wurzeln stehen in nassem Boden; Boden und Wasseroberfläche darunter gehören nicht zum Asset. Es werden keine Oberflächentemperaturen vorgegeben."),
            t("The transpiration emitter releases the latent flux of the leaves as vapour over twelve anchors on a middle leaf of every culm and young shoot; the release is diffuse and points up.", "Der Transpirationsemitter gibt den latenten Strom der Blätter als Dampf über zwölf Anker an einem mittleren Blatt jedes Halms und jungen Triebs ab; die Abgabe ist diffus und zeigt nach oben."),
        ],
        "operating": [], "passengers": [],
    }


V_CALM, V_WINDY, V_STORM = 1.5, 6.0, 16.0
CULM_AREA, PANICLE_AREA, SHOOT_AREA = 0.09, 0.03, 0.07
F = {k: tuple(round(L.wind_force(a, cd, v), 2) for a in (CULM_AREA, PANICLE_AREA, SHOOT_AREA)) for k, cd, v in [("calm", 0.6, V_CALM), ("windy", 0.6, V_WINDY), ("storm", 0.35, V_STORM)]}


def loads(kind, f_culm, f_panicle, f_shoot, source, reference, fy=False):
    word = {"wind": ("Wind drag", "Winddruck"), "snow": ("Snow", "Schnee")}[kind]
    out = []
    def row(bone, en, de, f, x):
        out.append({"id": f"{kind}-{bone}", "kind": kind, "title": t(f"{word[0]} on {en}", f"{word[1]} auf {de}"), "bone": bone, "x": x, "y": 0, "fxN": 0 if fy else f, "fyN": -f if fy else 0, "source": source, "reference": reference})
    for c in CULMS:
        row(f"culm-{c}b", f"the leaves of culm {c}", f"die Blätter von Halm {c}", f_culm, 0.4)
        row(f"panicle-{c}", f"panicle {c}", f"Rispe {c}", f_panicle, 0.15)
    for c in SHOOT:
        row(f"shoot-{c}", f"young shoot {c}", f"jungen Trieb {c}", f_shoot, 1.0)
    return out


def deltas(a, b, p, s):
    out = {}
    for c in CULMS:
        out.update({f"culm-{c}a": a, f"culm-{c}b": b, f"panicle-{c}": p})
    for c in SHOOT:
        out[f"shoot-{c}"] = s
    return out


WINDY = deltas(-3, -7, -10, -5)
STORM = deltas(-7, -16, -24, -14)
SNOW = {**{f"culm-{c}a": (-3 if CULM[c][0][0] > 0 else 3) for c in CULMS}, **{f"culm-{c}b": (-8 if CULM[c][0][0] > 0 else 8) for c in CULMS}, **{f"panicle-{c}": (-14 if CULM[c][0][0] > 0 else 14) for c in CULMS}, **{f"shoot-{c}": (-9 if SHOOT[c][0][0] > 0 else 9) for c in SHOOT}}


def binding(leaves, panicles, culms, emit, wet=None):
    regions = {"leaves": {"exposedFraction": 1, "sunlitFraction": leaves}, "panicles": {"exposedFraction": 1, "sunlitFraction": panicles}, "culms": {"exposedFraction": 1, "sunlitFraction": culms}}
    if wet is not None:
        for r, w in zip(regions.values(), wet):
            r["wettedness"] = w
    return {"regions": regions, "emitters": {"transpiration-leaves": emit}}


def poses():
    c, w, s = F["calm"], F["windy"], F["storm"]
    de = lambda v: L.fmt(v, "de")
    return [
        L.pose("calm", t("Calm late-summer day", "Windstiller Spätsommertag"), t(f"Light air of {V_CALM} m/s over the reed: culms stand upright, the panicles nod to the side of their lean; drag is {c[0]} N on the leaves of a culm and {c[1]} N per panicle.", f"Leiser Zug von {de(V_CALM)} m/s über dem Schilf: Die Halme stehen aufrecht, die Rispen nicken zur Seite ihrer Neigung; der Widerstand beträgt {de(c[0])} N auf die Blätter eines Halms und {de(c[1])} N je Rispe."), {}, binding(0.55, 0.85, 0.2, 0.8), loads("wind", *c, "literature", "en-1991-1-4")),
        L.pose("windy", t("Fresh breeze", "Frische Brise"), t(f"Beaufort 5, 8 m/s at 10 m, about {V_WINDY} m/s at the panicles (open water, z0 = 0.01 m); cd 0.6 on the leaves of a culm (0.09 m2), the panicle (0.03 m2) and a young shoot (0.07 m2): {w[0]}, {w[1]} and {w[2]} N. The slender culms bend 3 and 7 degrees, the panicles stream 20 degrees and the leaves turn into the wind like flags.", f"Beaufort 5, 8 m/s in 10 m Höhe, etwa {de(V_WINDY)} m/s an den Rispen (offenes Wasser, z0 = 0,01 m); cd 0,6 auf die Blätter eines Halms (0,09 m2), die Rispe (0,03 m2) und einen jungen Trieb (0,07 m2): {de(w[0])}, {de(w[1])} und {de(w[2])} N. Die schlanken Halme biegen sich 3 und 7 Grad, die Rispen wehen 20 Grad, die Blätter drehen sich wie Fahnen in den Wind."), WINDY, binding(0.55, 0.85, 0.2, 1), loads("wind", *w, "literature", "en-1991-1-4")),
        L.pose("storm", t("Gale", "Sturm"), t(f"Beaufort 9, 22 m/s at 10 m, about {V_STORM} m/s at the panicles; the leaves fold along the culm (cd 0.35): {s[0]} N per culm, {s[1]} N per panicle, {s[2]} N per young shoot. The culms lodge up to 47 degrees at the panicle and spring back when the wind falls (reed bends rather than breaks).", f"Beaufort 9, 22 m/s in 10 m Höhe, etwa {de(V_STORM)} m/s an den Rispen; die Blätter legen sich längs an den Halm (cd 0,35): {de(s[0])} N je Halm, {de(s[1])} N je Rispe, {de(s[2])} N je jungem Trieb. Die Halme legen sich an der Rispe bis 47 Grad um und richten sich bei nachlassendem Wind wieder auf (Schilf biegt sich, statt zu brechen)."), STORM, binding(0.3, 0.45, 0.1, 0.5, (0.4, 0.4, 0.4)), loads("wind", *s, "literature", "vogel-1989")),
        L.pose("snow", t("Early wet snow", "Früher Nassschnee"), t("An early wet snowfall in late autumn, before the leaves are shed: about 0.4 kg of snow on the leaves of every culm (4 N), 0.1 kg on every panicle (1 N) and 0.3 kg on every young shoot bow the culms outward by up to 25 degrees at the panicle; calm air (order of magnitude after EN 1991-1-3, authored estimate).", "Ein früher Nassschneefall im Spätherbst, bevor die Blätter abfallen: etwa 0,4 kg Schnee auf den Blättern jedes Halms (4 N), 0,1 kg auf jeder Rispe (1 N) und 0,3 kg auf jedem jungen Trieb biegen die Halme an der Rispe bis 25 Grad nach außen; Windstille (Größenordnung nach EN 1991-1-3, Schätzung)."), SNOW, binding(0, 0, 0, 0, (0.5, 0.3, 0.3)), loads("snow", 4, 1, 3, "authored-estimate", "en-1991-1-3", fy=True)),
    ]


def clips():
    amplitude = {**{f"culm-{c}a": 1.2 for c in CULMS}, **{f"culm-{c}b": 3.0 for c in CULMS}, **{f"panicle-{c}": 5.0 for c in CULMS}, **{f"shoot-{c}": 3.5 for c in SHOOT}}
    base = {**{f"culm-{c}": CULM[c][0] for c in CULMS}, **{f"shoot-{c}": SHOOT[c][0] for c in SHOOT}}
    def key(b):
        return b if b.startswith("shoot") else "culm-" + b.split("-")[1].rstrip("ab")
    phase = {b: -2 * math.pi * base[key(b)][0] / 1.2 + (0.5 if b.startswith("panicle") else 0.25 if b.endswith("b") else 0) for b in amplitude}
    spec = {"sway_period": 1.8, "emitter": "transpiration-leaves",
            "sway_text": t("A deterministic 1.8 s cycle (slender hollow culms, estimated 0.55 Hz) about the fresh-breeze lean: the culms whip by 3 degrees at the upper segment, the panicles by 5 degrees, with a phase that travels downwind through the patch like a wave over a reed bed.", "Ein deterministischer Zyklus von 1,8 s (schlanke hohle Halme, geschätzt 0,55 Hz) um die Neigung der frischen Brise: Die Halme peitschen am oberen Abschnitt um 3 Grad, die Rispen um 5 Grad, mit einer Phase, die windabwärts wie eine Welle über ein Röhricht durch den Bestand läuft."),
            "gust_text": t("A single 8 s gust: the wind-load tracks rise from 5 % to 2.75 times the fresh-breeze load at 2.5 s (EN 1991-1-4 peak pressure with Iv 0.25) and decay; culms and panicles lean up to 70 % of the gale lean and spring back.", "Eine einzelne Böe über 8 s: Die Windlast-Spuren steigen bei 2,5 s von 5 % auf das 2,75-fache der Last der frischen Brise (Spitzendruck nach EN 1991-1-4 mit Iv 0,25) und klingen ab; Halme und Rispen neigen sich bis zu 70 % der Sturmneigung und federn zurück."),
            "diurnal_text": t("A 24 h cycle compressed to 24 s (1 s = 1 h, 00:00 to 24:00), calm weather. The sunlit fraction of every region and the transpiration strength follow the sun, stomata nearly closed at night; a reed stand with wet roots transpires strongly all day. Leaf and culm temperatures are not authored but follow from the thermal solver.", "Ein 24-h-Zyklus auf 24 s gerafft (1 s = 1 h, 00:00 bis 24:00) bei ruhigem Wetter. Der besonnte Anteil jeder Region und die Transpirationsstärke folgen der Sonne, nachts sind die Stomata fast geschlossen; ein Schilfbestand mit nassen Wurzeln verdunstet den ganzen Tag stark. Blatt- und Halmtemperaturen werden nicht vorgegeben, sondern folgen aus dem thermischen Löser.")}
    wind_ids = [l["id"] for l in loads("wind", 1, 1, 1, "literature", "en-1991-1-4")]
    return [L.sway_clip(spec, WINDY, amplitude, phase), L.gust_clip(spec, STORM, wind_ids), L.diurnal_clip(spec, {"leaves": 0.55, "panicles": 0.85, "culms": 0.2})]


D = t
REP = {
    50: {"silhouette": D("At 1:50 (1 m is 20 mm) the patch is drawn leaf by leaf as the true view of its 3D body: nine tall culms with long two-ranked leaves that spread and droop at their tips, dense panicles of hairy branches nodding to one side, three young leafy shoots and last year's broken culms over the litter mat. All three views are projections of the same body.", "Bei 1:50 (1 m sind 20 mm) ist der Bestand Blatt für Blatt als wahre Ansicht seines 3D-Körpers gezeichnet: neun hohe Halme mit langen, zweizeiligen Blättern, die abstehen und an der Spitze überhängen, dichte Rispen aus behaarten Ästen, die zu einer Seite nicken, drei junge Blatttriebe und vorjährige abgeknickte Halme über der Streuschicht. Alle drei Ansichten sind Projektionen desselben Körpers."),
         "retained": D("Retained: every leaf blade (ten per culm, seven per young shoot), the culms in two segments, the panicle axis and sixteen branches per panicle, eight old culms and the litter mat.", "Beibehalten: jede Blattspreite (zehn je Halm, sieben je jungem Trieb), die Halme in zwei Abschnitten, die Rispenachse und sechzehn Äste je Rispe, acht alte Halme und die Streuschicht."),
         "merged": D("Merged: leaf sheaths into the culm, spikelets and their silky hairs into the branch strokes.", "Zusammengefasst: Blattscheiden mit dem Halm, Ährchen und ihre seidigen Haare in die Aststriche."),
         "omitted": D("Omitted: nodes, ligule hairs, the shed lower leaves and the rhizome below ground.", "Weggelassen: Knoten, Blatthäutchenhaare, die abgeworfenen unteren Blätter und das Rhizom im Boden."),
         "thinElements": D("Culms, leaves and branches are strokes of their true width (culms 0.22 to 0.13 mm, leaves 0.44 to 0.26 mm, branches 0.1 mm); nothing is thinner than 0.1 mm.", "Halme, Blätter und Äste sind Striche ihrer wahren Breite (Halme 0,22 bis 0,13 mm, Blätter 0,44 bis 0,26 mm, Äste 0,1 mm); nichts ist dünner als 0,1 mm."),
         "hatching": D("No hatching.", "Keine Schraffur."),
         "styles": D("Line views: strokes over the filled litter mat. Solid views: leaves mid tone, panicles light, culms dark.", "Linienansichten: Striche über der gefüllten Streuschicht. Flächenansichten: Blätter im Mittelton, Rispen hell, Halme dunkel."),
         "overlays": D("Every shape is bound to its thermal region (leaves, panicles, culms) and to the bone that carries it.", "Jede Form ist an ihre thermische Region (Blätter, Rispen, Halme) und an den tragenden Knochen gebunden.")},
    100: {"silhouette": D("At 1:100 (1 m is 10 mm) every culm keeps six leaves standing for its ten, each panicle is one dense nodding plume. All three views are projections of the 1:100 body.", "Bei 1:100 (1 m sind 10 mm) behält jeder Halm sechs Blätter, die für seine zehn stehen, jede Rispe ist ein dichter nickender Wedel. Alle drei Ansichten sind Projektionen des Körpers von 1:100."),
          "retained": D("Retained: nine culms in two segments, six leaves per culm and four per young shoot, nine panicles, five old culms, the litter mat.", "Beibehalten: neun Halme in zwei Abschnitten, sechs Blätter je Halm und vier je jungem Trieb, neun Rispen, fünf alte Halme, die Streuschicht."),
          "merged": D("Merged: neighbouring leaves into one, panicle branches into the plume.", "Zusammengefasst: benachbarte Blätter zu einem, Rispenäste in den Wedel."),
          "omitted": D("Omitted: three of the old culms, single branches.", "Weggelassen: drei der alten Halme, einzelne Äste."),
          "thinElements": D("Culm and leaf strokes 0.23 to 0.1 mm.", "Halm- und Blattstriche 0,23 bis 0,1 mm."),
          "hatching": D("No hatching.", "Keine Schraffur."), "styles": D("Line views: strokes over the outlined mat and plumes. Solid views: the same shapes in the tone of their region.", "Linienansichten: Striche über der umrissenen Matte und den Wedeln. Flächenansichten: dieselben Formen im Ton ihrer Region."),
          "overlays": D("Every shape is bound to its thermal region and bone.", "Jede Form ist an ihre thermische Region und ihren Knochen gebunden.")},
    200: {"silhouette": D("At 1:200 (1 m is 5 mm) each culm carries four long leaves and one panicle body; the patch reads as a stand of upright culms with flag leaves. All three views are projections of the 1:200 body.", "Bei 1:200 (1 m sind 5 mm) trägt jeder Halm vier lange Blätter und einen Rispenkörper; der Bestand liest sich als Gruppe aufrechter Halme mit Fahnenblättern. Alle drei Ansichten sind Projektionen des Körpers von 1:200."),
          "retained": D("Retained: nine culms, four leaves per culm, three per young shoot, nine panicles, the litter mat.", "Beibehalten: neun Halme, vier Blätter je Halm, drei je jungem Trieb, neun Rispen, die Streuschicht."),
          "merged": D("Merged: two or three leaves into each drawn leaf, the panicle into one body, old culms into the litter mat.", "Zusammengefasst: zwei oder drei Blätter in jedes gezeichnete Blatt, die Rispe in einen Körper, alte Halme in die Streuschicht."),
          "omitted": D("Omitted: single branches and old culms (below 0.1 mm on paper).", "Weggelassen: einzelne Äste und alte Halme (unter 0,1 mm auf dem Papier)."),
          "thinElements": D("Culms and leaves at the 0.1 to 0.13 mm pens.", "Halme und Blätter mit den Stiften von 0,1 bis 0,13 mm."),
          "hatching": D("No hatching.", "Keine Schraffur."), "styles": D("Line views: strokes over the outlined mat and panicles. Solid views: tonal masses under the strokes.", "Linienansichten: Striche über der umrissenen Matte und den Rispen. Flächenansichten: getönte Massen unter den Strichen."),
          "overlays": D("Every shape is bound to its thermal region and bone.", "Jede Form ist an ihre thermische Region und ihren Knochen gebunden.")},
    500: {"silhouette": D("At 1:500 (1 m is 2 mm) the patch is nine culm lines with three leaves each and a panicle body on top, over a low mat: the vertical, flag-leaved reed against the fountain of a tussock grass. All three views are projections of the 1:500 body.", "Bei 1:500 (1 m sind 2 mm) ist der Bestand aus neun Halmlinien mit je drei Blättern und einem Rispenkörper darüber, über einer niedrigen Matte: das senkrechte, fahnenblättrige Schilf im Gegensatz zur Fontäne eines Horstgrases. Alle drei Ansichten sind Projektionen des Körpers von 1:500."),
          "retained": D("Retained: nine culms, three leaves per culm, two per young shoot, nine panicles, the mat.", "Beibehalten: neun Halme, drei Blätter je Halm, zwei je jungem Trieb, neun Rispen, die Matte."),
          "merged": D("Merged: three or four leaves into each drawn leaf; the mat thickened to 95 mm and the panicles to 80 mm, the printable feature.", "Zusammengefasst: drei oder vier Blätter in jedes gezeichnete Blatt; die Matte auf 95 mm und die Rispen auf 80 mm, die druckbare Einzelheit, verdickt."),
          "omitted": D("Omitted: everything below 75 mm that is not a slender culm or leaf.", "Weggelassen: alles unter 75 mm, was kein schlanker Halm oder Blatt ist."),
          "thinElements": D("Culm and leaf lines at 0.1 mm.", "Halm- und Blattlinien mit 0,1 mm."),
          "hatching": D("No hatching.", "Keine Schraffur."), "styles": D("Line views: restrained outlines. Solid views: tonal masses by depth.", "Linienansichten: zurückhaltende Umrisse. Flächenansichten: getönte Massen nach Tiefe."),
          "overlays": D("Every shape is bound to its thermal region and bone.", "Jede Form ist an ihre thermische Region und ihren Knochen gebunden.")},
}
VOL = {
    50: [D("1:50 body: litter mat, eight broken old culms, nine culms of two straight segments, ten two-ranked leaf blades per culm placed at its nodes from 30 to 95 % of its height, three young shoots of seven leaves, every panicle as an axis with sixteen hairy branches.", "1:50-Körper: Streuschicht, acht abgeknickte alte Halme, neun Halme aus zwei geraden Abschnitten, zehn zweizeilige Blattspreiten je Halm an seinen Knoten von 30 bis 95 % seiner Höhe, drei junge Triebe mit sieben Blättern, jede Rispe als Achse mit sechzehn behaarten Ästen."),
         D("Leaves, culms and branches are slender (at least 20 times as long as wide); every leaf is one solid on the bone of its node, so no pose opens a gap. Thermal by reference plus twelve transpiration strips; structure: rhizome disc, culm rods of 10 and 7 mm, panicle axes, joints, support disc and lumped masses.", "Blätter, Halme und Äste sind schlank (mindestens 20-mal so lang wie breit); jedes Blatt ist ein Körper am Knochen seines Knotens, sodass keine Pose eine Lücke öffnet. Thermik per Verweis plus zwölf Transpirationsstreifen; Tragwerk: Rhizomscheibe, Halmstäbe von 10 und 7 mm, Rispenachsen, Gelenke, Auflagerscheibe und konzentrierte Massen.")],
    100: [D("1:100 body: mat, five old culms, nine culms with six leaves, three young shoots with four, one panicle body each.", "1:100-Körper: Matte, fünf alte Halme, neun Halme mit sechs Blättern, drei junge Triebe mit vier, je ein Rispenkörper."), D("Thermal by reference plus twelve strips; structure with simplified rods.", "Thermik per Verweis plus zwölf Streifen; Tragwerk mit vereinfachten Stäben.")],
    200: [D("1:200 body: mat, nine culms with four leaves of 2 to 2.6 cm, shoots with three, one panicle body each.", "1:200-Körper: Matte, neun Halme mit vier Blättern von 2 bis 2,6 cm, Triebe mit drei, je ein Rispenkörper."), D("Thermal by reference plus twelve strips; structure without joints.", "Thermik per Verweis plus zwölf Streifen; Tragwerk ohne Gelenke.")],
    500: [D("1:500 body: mat of 95 mm, nine culms with three leaves, shoots with two, panicle bodies of 80 mm.", "1:500-Körper: Matte von 95 mm, neun Halme mit drei Blättern, Triebe mit zwei, Rispenkörper von 80 mm."), D("Thermal by reference plus twelve strips; structure with three-point rods.", "Thermik per Verweis plus zwölf Streifen; Tragwerk mit Dreipunktstäben.")],
}

REFERENCES = [
    {"id": "en-1991-1-4", "citation": "EN 1991-1-4:2005 Eurocode 1, Actions on structures, Part 1-4: Wind actions. Air density 1.25 kg/m3, F = 0.5 rho cd A v^2, peak velocity pressure (1 + 7 Iv) q_m."},
    {"id": "en-1991-1-3", "citation": "EN 1991-1-3:2003 Eurocode 1, Actions on structures, Part 1-3: Snow loads. Used for the order of magnitude of snow accretion on foliage."},
    {"id": "vogel-1989", "citation": "Vogel, S. (1989). Drag and reconfiguration of broad leaves in high winds. Journal of Experimental Botany 40(8), 941-948."},
    {"id": "jones-2014", "citation": "Jones, H. G. (2014). Plants and Microclimate, 3rd ed., Cambridge University Press. Leaf energy balance, leaf area index, boundary-layer conductance."},
    {"id": "monteith-unsworth-2013", "citation": "Monteith, J. L. and Unsworth, M. H. (2013). Principles of Environmental Physics, 4th ed., Academic Press. Penman-Monteith leaf transpiration."},
    {"id": "fna-25-phragmites", "citation": "Saltonstall, K., Peterson, P. M. and Soreng, R. J. (2003). Phragmites australis. In: Flora of North America Editorial Committee (eds.), Flora of North America 25, Magnoliophyta: Commelinidae (in part): Poaceae, part 2. Oxford University Press. Culms 1-4 m tall, 0.5-1.5 cm thick; blades 15-40 cm long, 2-4 cm wide. https://floranorthamerica.org/Phragmites_australis"},
    {"id": "kew-grassbase-phragmites", "citation": "Clayton, W. D., Vorontsova, M. S., Harman, K. T. and Williamson, H. (2006 onwards). GrassBase - The Online World Grass Flora, Royal Botanic Gardens, Kew: Phragmites australis. Culms 150-600 cm; leaf blades 20-60 cm x 8-32 mm; panicle 20-50 cm long, 6-15 cm wide."},
    {"id": "edgar-connor-2000", "citation": "Edgar, E. and Connor, H. E. (2000). Flora of New Zealand, Volume V: Gramineae. Manaaki Whenua Press, Lincoln. Phragmites australis: 1.5-3 m; leaf blade 40-60 x 1-3 cm; panicle 20-30 cm, erect or finally drooping, purplish to brownish."},
]


def spec():
    return {
        "id": ID, "target": TARGET, "plant_module": PLANT, "crown_length": CROWN, "root_mass": ROOT_KG, "mass_reference": "jones-2014",
        "root_title": t("Rhizome mat and litter", "Rhizomgeflecht und Streu"),
        "root_section": L.circle_section(1.2, "reed-litter", "Solid section, d = 1.2 m", "Vollquerschnitt, d = 1,2 m"),
        "bones": BONES, "lateral": LATERAL, "rename": {},
        "leaf_material": "reed-leaf", "leaf_region": "leaves", "emitter": "transpiration-leaves", "emitter_title": t("Transpiration of the leaves", "Transpiration der Blätter"),
        "stem_material": "reed-culm", "mat_material": "reed-litter", "top_material": "reed-panicle",
        "mat": {50: [(0.002, 0.6), (0.06, 0.5), (0.08, 0.3)], 100: [(0.004, 0.6), (0.06, 0.5), (0.08, 0.3)], 200: [(0.004, 0.6), (0.06, 0.5), (0.08, 0.3)], 500: [(0.004, 0.6), (0.06, 0.5), (0.085, 0.3)]},
        "member_radius": member_radius, "joint_radius": {50: 0.01, 100: 0.009}, "support_radius": 0.6,
        "crown_mass_radii": {50: [0.03, 0.35, 0.35], 100: [0.03, 0.35, 0.35], 200: [0.035, 0.35, 0.35], 500: [0.04, 0.3, 0.3]},
        "masses": masses, "footprint": 1.2,
        "support_title": t("Rhizome anchorage", "Rhizomverankerung"),
        "title": t("Common Reed", "Schilfrohr"),
        "summary": t(f"A flowering patch of common reed (Phragmites australis) 3 m tall, {L.fmt(DIMS['widthM'])} m wide and {L.fmt(DIMS['depthM'])} m deep at the edge of a reed bed: nine slender culms rising from a creeping rhizome, each with ten long, two-ranked, grey-green leaves that spread and droop at the tip, dense purplish-brown panicles nodding to one side, three young leafy shoots and last year's broken culms in the litter.", f"Ein blühender Schilfbestand (Phragmites australis) am Rand eines Röhrichts, 3 m hoch, {L.fmt(DIMS['widthM'], 'de')} m breit und {L.fmt(DIMS['depthM'], 'de')} m tief: neun schlanke Halme aus einem kriechenden Rhizom, jeder mit zehn langen, zweizeiligen, graugrünen Blättern, die abstehen und an der Spitze überhängen, dichte, violettbraune, einseitig nickende Rispen, drei junge Blatttriebe und vorjährige abgeknickte Halme in der Streu."),
        "tags": ["grass", "perennial", "wetland", "reed", "panicles", "wind", "transpiration"],
        "provenance": {"author": "Semio staffage vegetation content worker PW-15", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES, "notes": t("Built from the plant parts of the round-3 miscanthus clump (paths inside the envelope of `dimensions`, whole leaves on the bone of their node, strokes of true width), but as a stand instead of a tussock: culms stand apart on the rhizome, leaves are two-ranked along the culm instead of basal. Culms 11 mm thick at the base and 2.4 to 2.6 m to the panicle (FNA 25: 1-4 m, 0.5-1.5 cm), blades 28 to 48 cm long and 1.3 to 2.2 cm wide (FNA 15-40 x 2-4 cm, Kew 20-60 cm x 8-32 mm; kept at least 20 times as long as wide so each blade is a slender stroke of true width), panicles 30 cm (20-50 cm). The lower culm has shed its leaves (only sheaths remain) as in late summer. The rig is the side view of the three-dimensional culms: every joint hinges about a lateral axis and carries its lateral place as `z`.", "Aus den Pflanzenteilen des Chinaschilf-Horstes aus Runde 3 gebaut (Bahnen innerhalb der Hülle von `dimensions`, ganze Blätter am Knochen ihres Knotens, Striche wahrer Breite), aber als Bestand statt als Horst: Die Halme stehen einzeln auf dem Rhizom, die Blätter sitzen zweizeilig am Halm statt grundständig. Halme am Fuß 11 mm dick und 2,4 bis 2,6 m bis zur Rispe (FNA 25: 1-4 m, 0,5-1,5 cm), Spreiten 28 bis 48 cm lang und 1,3 bis 2,2 cm breit (FNA 15-40 x 2-4 cm, Kew 20-60 cm x 8-32 mm; mindestens 20-mal so lang wie breit gehalten, damit jede Spreite ein schlanker Strich wahrer Breite ist), Rispen 30 cm (20-50 cm). Der untere Halm hat seine Blätter wie im Spätsommer abgeworfen (nur die Scheiden bleiben). Das Skelett ist die Seitenansicht der dreidimensionalen Halme: Jedes Gelenk dreht um eine Querachse und trägt seine seitliche Lage als `z`.")},
        "dimensions": DIMS,
        "materials": [
            {"id": "reed-leaf", "title": t("Reed leaf blades", "Schilfblätter"), "emissivity": 0.97, "solarAbsorptivity": 0.5, "source": "literature", "reference": "jones-2014"},
            {"id": "reed-culm", "title": t("Green reed culm (hollow, solid-section equivalent)", "Grüner Schilfhalm (hohl, Vollquerschnitt-Äquivalent)"), "emissivity": 0.9, "solarAbsorptivity": 0.55, "densityKgM3": 250, "conductivityWmK": 0.1, "elasticModulusPa": 5000000000, "strengthPa": 40000000, "source": "authored-estimate"},
            {"id": "reed-litter", "title": t("Litter of reed leaves and old culms", "Streu aus Schilfblättern und alten Halmen"), "emissivity": 0.93, "solarAbsorptivity": 0.7, "densityKgM3": 150, "conductivityWmK": 0.07, "source": "authored-estimate", "reference": "jones-2014"},
            {"id": "reed-panicle", "title": t("Purplish-brown panicles with silky hairs", "Violettbraune Rispen mit seidigen Haaren"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "source": "authored-estimate", "reference": "jones-2014"},
        ],
        "poses": poses(), "clips": clips(), "thermal": thermal,
        "structure": {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": ["base-support"], "assumptions": [t("One rigid body per evaluated pose, fixed at the rhizome mat; wind drag acts on the leaves of every upper culm segment, every panicle and young shoot, gravity on all bone masses. The hollow culm is replaced by the solid section of equal bending stiffness with the modulus of green reed culm (authored estimate). Dynamic amplification and rhizome pull-out are not modelled.", "Ein starrer Körper je ausgewerteter Pose, am Rhizomgeflecht eingespannt; Winddruck wirkt auf die Blätter jedes oberen Halmabschnitts, jede Rispe und jeden jungen Trieb, Schwerkraft auf alle Knochenmassen. Der hohle Halm ist durch den Vollquerschnitt gleicher Biegesteifigkeit mit dem Modul von frischem Schilfhalm ersetzt (Schätzung). Dynamische Überhöhung und Ausreißen des Rhizoms werden nicht modelliert.")], "clinical": False},
        "volume_decisions": VOL, "representation_decisions": REP,
        "symbol_lobes": (17, 0.12), "symbol_stems": [(CULM[c][0][0], CULM[c][0][2], 0.0055) for c in CULMS],
    }


DIMS = {"heightM": HEIGHT, "widthM": WIDTH, "depthM": DEPTH, "massKg": MASS}


def main():
    """🏁 Builds the patch; `dimensions` take the width and depth of the 1:50 body (leaves stop at the envelope, culms stand inside it)."""
    M.TARGET, M.P = TARGET, PLANT
    rig = K.Rig(L.rig_bones({**spec(), "bones": BONES}))
    e = M.extent(make(50)(rig))
    DIMS.update({"widthM": round(e["x+"] - e["x-"], 2), "depthM": round(e["z+"] - e["z-"], 2)})
    print(f"[DEBUG] dimensions {DIMS}")
    document, report = L.build_asset(spec(), {s: make(s) for s in L.SCALES})
    L.write(document, report, t("A vertical wetland grass: nine tall, separate culms with broad two-ranked flag leaves and dense nodding purplish panicles; it whips quickly in the wind and lodges in a gale, unlike the fountain of a tussock grass.", "Ein senkrechtes Sumpfgras: neun hohe, einzeln stehende Halme mit breiten, zweizeiligen Fahnenblättern und dichten, nickenden, violetten Rispen; es peitscht schnell im Wind und legt sich im Sturm um, anders als die Fontäne eines Horstgrases."))


if __name__ == "__main__":
    main()
