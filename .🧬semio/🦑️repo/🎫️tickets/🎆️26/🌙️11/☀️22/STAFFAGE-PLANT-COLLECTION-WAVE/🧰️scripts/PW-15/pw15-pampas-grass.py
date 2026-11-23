"""🌾 PW-15, vegetation/pampas-grass: a flowering tussock of pampas grass (Cortaderia selloana), 2.5 m with eight dense silvery plumes. The plant is stated by hand in world space (x ahead, y up, z left, metres) as tables per scale, in the row format of the round-3 miscanthus plant, and built with the round-3 miscanthus builders (`M.fine` for 1:50, 1:100 and 1:200, `M.coarse500` for 1:500) imported from their place.

Reference: Kew GrassBase (Clayton et al.): culms 100-300 cm, panicle open, ovate, dense, 25-100 cm long; AusGrass2: blades 60-200 cm x 3-15 mm, culms 1-4 m; Flora of New Zealand V (Edgar and Connor 2000): very tall stout erect tussock, blades to 2 m x 2 cm.
Usage: sh pw15-build.sh pampas-grass
See: ../../📝️PW-15.md
"""
import importlib.util
import math
import os
from pathlib import Path
from types import SimpleNamespace

spec_ = importlib.util.spec_from_file_location("pw15_lib", Path(__file__).resolve().parent / "pw15-lib.py")
L = importlib.util.module_from_spec(spec_)
spec_.loader.exec_module(L)
K, M, t = L.K, L.M, L.t

ID = "pampas-grass"
HEIGHT, WIDTH, DEPTH, MASS = 2.5, 2.2, 2.0, 20.0
EDGE = 0.006
TARGET = {"x": WIDTH / 2 + EDGE, "z": DEPTH / 2 + EDGE, "y": HEIGHT}
CROWN = 0.14
CULMS = ["1", "2", "3", "4", "5", "6", "7", "8"]
BUNDLES = ["l1", "l2", "l3", "r1", "r2", "r3"]

CULM = {
    "1": ((0.08, 0.12, 0.05), -10, -4, 1.12, 0.68),
    "2": ((0.03, 0.12, -0.07), -3, 0, 1.18, 0.72),
    "3": ((-0.03, 0.12, 0.07), 3, 0, 1.18, 0.72),
    "4": ((-0.08, 0.12, -0.04), 10, 4, 1.12, 0.68),
    "5": ((0.11, 0.12, -0.02), -16, -5, 1.02, 0.6),
    "6": ((-0.11, 0.12, 0.02), 16, 5, 1.02, 0.6),
    "7": ((0.02, 0.12, 0.12), -5, -2, 1.08, 0.64),
    "8": ((-0.02, 0.12, -0.12), 5, 2, 1.08, 0.64),
}
BUNDLE = {
    "l1": ((-0.16, 0.1, 0.03), 50, 0.6, 40, 0.85),
    "l2": ((-0.11, 0.1, -0.12), 32, 0.65, 36, 0.9),
    "l3": ((-0.05, 0.1, 0.16), 16, 0.7, 32, 0.95),
    "r1": ((0.16, 0.1, -0.03), -50, 0.6, -40, 0.85),
    "r2": ((0.11, 0.1, 0.12), -32, 0.65, -36, 0.9),
    "r3": ((0.05, 0.1, -0.16), -16, 0.7, -32, 0.95),
}
LATERAL = {
    "culm-1a": 0.12, "culm-1b": 0.1, "plume-1": 0.05, "culm-2a": -0.1, "culm-2b": -0.07, "plume-2": -0.03,
    "culm-3a": 0.1, "culm-3b": 0.07, "plume-3": 0.03, "culm-4a": -0.12, "culm-4b": -0.1, "plume-4": -0.05,
    "culm-5a": -0.05, "culm-5b": -0.04, "plume-5": -0.02, "culm-6a": 0.05, "culm-6b": 0.04, "plume-6": 0.02,
    "culm-7a": 0.3, "culm-7b": 0.2, "plume-7": 0.08, "culm-8a": -0.3, "culm-8b": -0.2, "plume-8": -0.08,
    "bl-l1a": 0.06, "bl-l1b": 0.12, "bl-l2a": -0.26, "bl-l2b": -0.42, "bl-l3a": 0.3, "bl-l3b": 0.42,
    "bl-r1a": -0.06, "bl-r1b": -0.12, "bl-r2a": 0.26, "bl-r2b": 0.42, "bl-r3a": -0.3, "bl-r3b": -0.42,
}
BLADE_PROFILE = [(0.0, 0.55), (0.1, 0.85), (0.25, 1.0), (0.5, 0.95), (0.7, 0.78), (0.85, 0.55), (0.95, 0.3), (1.0, 0.08)]

S50 = {
    "blade_half_thickness": 0.0002, "blade_points": 11,
    "main_a": [(0.25, 80, 18, 120, 0.95, 0.011), (0.42, -105, 18, 124, 1.05, 0.012), (0.6, -40, 16, 128, 1.15, 0.012), (0.78, 150, 15, 120, 1.2, 0.012), (0.92, 55, 14, 132, 1.25, 0.012)],
    "main_b": [(0.0, 35, 12, 136, 1.3, 0.012), (0.0, -55, 13, 124, 1.2, 0.012), (0.3, 105, 12, 128, 1.2, 0.012), (0.3, -120, 13, 120, 1.05, 0.011), (0.55, 170, 11, 118, 1.0, 0.011), (0.7, 10, 9, 140, 1.35, 0.012), (0.88, -80, 9, 130, 1.15, 0.011), (1.0, 60, 7, 128, 1.1, 0.011), (1.0, -20, 5, 148, 1.3, 0.012)],
    "stem_a_radius": (0.006, 0.005), "stem_b": (0.25, 0.005, 0.0035),
    "satellites": {
        "l1": [(62, 40, 0.6), (-66, 38, 0.56), (8, 18, 0.7), (-28, 28, 0.66)], "l2": [(58, 32, 0.66), (-64, 34, 0.62), (-10, 14, 0.72), (26, 22, 0.7)],
        "l3": [(66, 28, 0.7), (-60, 24, 0.68), (12, 10, 0.74), (-24, 16, 0.76)], "r1": [(-62, 40, 0.6), (66, 38, 0.56), (-8, 18, 0.7), (28, 28, 0.66)],
        "r2": [(-58, 32, 0.66), (64, 34, 0.62), (10, 14, 0.72), (-26, 22, 0.7)], "r3": [(-66, 28, 0.7), (60, 24, 0.68), (-12, 10, 0.74), (24, 16, 0.76)],
    },
    "companions": {c: [(150 - 37 * i, 10, 0.75)] for i, c in enumerate(CULMS)},
    "satellite_radius": (0.005, 0.0035),
    "satellite_leaves": [(0.35, 60, 18, 122, 0.85, 0.011), (0.55, -110, 17, 120, 0.95, 0.011), (0.72, -50, 16, 126, 1.05, 0.012), (0.88, 25, 12, 134, 1.15, 0.012), (1.0, -15, 7, 140, 1.25, 0.012)],
    "culm_a_radius": (0.0085, 0.0072), "culm_b_radius": (0.0068, 0.0045),
    "culm_a_leaves": [(0.15, 90, 14, 118, 0.8, 0.011), (0.3, -85, 13, 122, 0.85, 0.011), (0.48, 20, 12, 124, 0.75, 0.01)],
    "culm_b_leaves": [(0.05, -10, 10, 120, 0.5, 0.009)],
    "culm_twist": {c: 41 * i for i, c in enumerate(CULMS)},
    "basal": [(a, 46 + 10 * math.sin(a), 150 + 8 * math.cos(2 * a), 0.85 + 0.2 * math.sin(3 * a), 0.012) for a in range(5, 360, 20)],
    "basal_ring": 0.24,
    "mound": [(0.002, 0.34), (0.03, 0.33), (0.06, 0.3), (0.09, 0.25), (0.12, 0.16), (0.14, 0.06)],
    "stubble": [(0.06, 30, 0.3, 6, 50), (0.12, 95, 0.26, 10, 110), (0.08, 165, 0.32, 5, 190), (0.16, 230, 0.24, 12, 240), (0.1, 300, 0.28, 8, 320), (0.2, 10, 0.22, 15, 15), (0.19, 200, 0.2, 16, 190), (0.14, 130, 0.27, 9, 140)],
    "stubble_pair": (0.008, 0.02, 3), "stubble_radius": 0.0065,
    "plume": [(0.0, 0.012, 0.012), (0.08, 0.05, 0.045), (0.2, 0.07, 0.062), (0.36, 0.068, 0.06), (0.5, 0.045, 0.04), (0.6, 0.012, 0.012)],
    "plume_nod": 12,
    "plume_fan": [(0, 0, 1.0), (4, 30, 0.9), (3, 120, 0.94), (4, 210, 0.88), (3, 300, 0.92), (6, 75, 0.8), (6, 165, 0.78), (6, 255, 0.82), (5, 345, 0.8)],
    "strip": (0.25, 0.75, 0.32), "joint_radius": 0.012, "support_thickness": 0.02,
}

def shorten(rows, k, i):
    return [tuple(round(v * k, 3) if j == i else v for j, v in enumerate(r)) for r in rows]


for _key in ("main_a", "main_b", "satellite_leaves", "culm_a_leaves"):
    S50[_key] = shorten(S50[_key], 0.8, 4)

def widen(rows, width):
    return [(*r[:-1], width) for r in rows]


S100 = {
    **S50,
    "blade_half_thickness": 0.0002, "blade_points": 7,
    **{_key: widen(S50[_key], 0.0165) for _key in ("main_a", "main_b", "satellite_leaves", "culm_a_leaves", "culm_b_leaves", "basal")},
    "mound": [(0.004, 0.34), (0.04, 0.32), (0.08, 0.27), (0.12, 0.17), (0.14, 0.06)],
    "stubble": S50["stubble"][:6], "stubble_radius": 0.009,
    "plume_fan": [(0, 0, 1.0), (4, 40, 0.9), (4, 160, 0.88), (4, 280, 0.9), (6, 100, 0.8), (6, 220, 0.8)],
    "strip": (0.25, 0.75, 0.3), "joint_radius": 0.011,
}
S100.pop("stubble_pair")

S200 = {
    **S100,
    "blade_half_thickness": 0.0002, "blade_points": 6,
    "companions": {c: [] for c in CULMS},
    "main_a": widen([S50["main_a"][1], S50["main_a"][4]], 0.033), "main_b": widen([S50["main_b"][0], S50["main_b"][2], S50["main_b"][5], S50["main_b"][8]], 0.033),
    "satellite_leaves": widen([S50["satellite_leaves"][1], S50["satellite_leaves"][4]], 0.033), "basal": widen(S50["basal"][::2], 0.033),
    "culm_a_leaves": widen(S50["culm_a_leaves"][1:2], 0.033), "culm_b_leaves": [],
    "stubble": [],
    "mound": [(0.004, 0.34), (0.05, 0.31), (0.1, 0.22), (0.145, 0.07)],
    "plume": [(0.0, 0.018, 0.018), (0.08, 0.06, 0.055), (0.22, 0.078, 0.07), (0.38, 0.072, 0.066), (0.52, 0.048, 0.044), (0.6, 0.016, 0.016)],
    "plume_fan": [(0, 0, 1.0), (4, 70, 0.88), (4, 200, 0.9), (5, 310, 0.86)],
    "strip": (0.1, 0.95, 0.01),
}

S500 = {
    "mound": [(0.004, 0.34), (0.07, 0.32), (0.12, 0.22), (0.15, 0.08)],
    "lobe_a": [(0.0, 0.05, 0.04), (0.5, 0.07, 0.045), (1.0, 0.08, 0.045)],
    "lobe_b": [(0.0, 0.08, 0.045), (0.3, 0.085, 0.045), (0.6, 0.07, 0.04), (0.85, 0.045, 0.039), (1.0, 0.01, 0.01)],
    "lobe_b_droop": 34,
    "laterals": {
        "l1": [(66, 40, 136, 1.2), (-62, 36, 132, 1.15), (-20, 24, 128, 1.3)], "l2": [(60, 33, 132, 1.25), (-64, 30, 130, 1.2), (16, 18, 124, 1.35)],
        "l3": [(62, 26, 128, 1.3), (-58, 22, 126, 1.25), (-14, 12, 118, 1.4)], "r1": [(-66, 40, 136, 1.2), (62, 36, 132, 1.15), (20, 24, 128, 1.3)],
        "r2": [(-60, 33, 132, 1.25), (64, 30, 130, 1.2), (-16, 18, 124, 1.35)], "r3": [(-62, 26, 128, 1.3), (58, 22, 126, 1.25), (14, 12, 118, 1.4)],
    },
    "lateral_section": [(0.0, 0.04, 0.04), (0.3, 0.075, 0.045), (0.6, 0.068, 0.042), (0.85, 0.045, 0.039), (1.0, 0.01, 0.01)],
    "inner": {c: [(150 - 45 * i, 10, 75, 1.2), (-40 - 45 * i, 14, 90, 1.1)] for i, c in enumerate(CULMS)},
    "inner_section": [(0.0, 0.04, 0.04), (0.4, 0.07, 0.045), (0.75, 0.055, 0.04), (1.0, 0.01, 0.01)],
    "culm_radius": 0.008,
    "plume": [(0.0, 0.03, 0.03), (0.1, 0.07, 0.06), (0.3, 0.085, 0.07), (0.5, 0.06, 0.05), (0.6, 0.02, 0.02)],
    "shoot_axis_radius": 0.005, "plume_nod": 12, "strip": (0.1, 0.9, 0.012), "support_thickness": 0.075,
}

S500["laterals"] = {n: shorten(v, 0.85, 3) for n, v in S500["laterals"].items()}

PLANT = SimpleNamespace(BUNDLES=BUNDLES, CULMS=CULMS, LATERAL=LATERAL, BLADE_PROFILE=BLADE_PROFILE, S50=S50, S100=S100, S200=S200, S500=S500, CROWN_LENGTH=CROWN)


def bones():
    rows = []
    for c, (base, angle, angle_b, la, lb) in CULM.items():
        rows.append({"id": f"culm-{c}a", "parent": "root-plate", "base": base, "angle": angle, "length": la, "limit": 10, "kg": 0.35, "title": t(f"Flowering culm {c}, lower", f"Blütenhalm {c}, unten"), "section": L.circle_section(0.015, "pampas-culm", "Solid section, d = 1.5 cm", "Vollquerschnitt, d = 1,5 cm")})
        rows.append({"id": f"culm-{c}b", "parent": f"culm-{c}a", "angle": angle_b, "length": lb, "limit": 18, "kg": 0.15, "title": t(f"Flowering culm {c}, upper", f"Blütenhalm {c}, oben"), "section": L.circle_section(0.011, "pampas-culm", "Solid section, d = 1.1 cm", "Vollquerschnitt, d = 1,1 cm")})
        rows.append({"id": f"plume-{c}", "parent": f"culm-{c}b", "angle": 0, "length": 0.6, "limit": 22, "kg": 0.14, "com": 0.4, "distribution": "point", "title": t(f"Plume {c}", f"Blütenwedel {c}")})
    names = {"l": ("left", "links"), "r": ("right", "rechts")}
    for b, (base, angle, la, angle_b, lb) in BUNDLE.items():
        en, de = names[b[0]]
        rows.append({"id": f"bl-{b}a", "parent": "root-plate", "base": base, "angle": angle, "length": la, "limit": 12, "kg": 0.6, "title": t(f"Leaf bundle {b[1]} {en}, rising", f"Blattbündel {b[1]} {de}, aufsteigend")})
        rows.append({"id": f"bl-{b}b", "parent": f"bl-{b}a", "angle": angle_b - angle, "length": lb, "limit": 26, "kg": 0.56, "title": t(f"Leaf bundle {b[1]} {en}, arching", f"Blattbündel {b[1]} {de}, überhängend")})
    return rows


BONES = bones()
ROOT_KG = round(MASS - sum(r["kg"] for r in BONES), 4)


def member_radius(bone, scale):
    if bone.startswith("culm"):
        return 0.0075 if bone.endswith("a") else 0.0055
    if bone.startswith("plume"):
        return 0.003 if scale <= 200 else 0.004
    return (0.0045 if bone.endswith("a") else 0.0035) if scale <= 100 else (0.0055 if bone.endswith("a") else 0.0045)


def masses(scale):
    if scale == 500:
        return {}
    r = {50: 0.014, 100: 0.014, 200: 0.02}[scale]
    return {f"plume-{c}": (0.45, r) for c in CULMS}


def thermal(r50, anchors):
    leaf = round(r50["one_sided_leaf_m2"], 2)
    lai = round(leaf / (math.pi * TARGET["x"] * TARGET["z"]), 2)
    return {
        "environment": {"airTempC": 22, "meanRadiantTempC": 25, "relativeHumidity": 0.55, "windMS": 2, "windDirectionDeg": 0, "solarWM2": 700, "pressurePa": 101325, "diffuseFraction": 0.15},
        "regions": [
            {"id": "blades", "title": t("Leaf blades", "Blattspreiten"), "kind": "foliage", "model": "leaf", "material": "pampas-blade", "areaM2": leaf, "areaBasis": "exchange", "characteristicLengthM": 0.012, "geometry": "vertical-plate", "skyViewFactor": 0.6, "layers": [], "heatCapacityJM2K": 700, "leaf": {"stomatalConductanceMaxMS": 0.005, "leafAreaIndex": lai, "photosynthesis": "c3"}, "source": "authored-estimate", "reference": "jones-2014"},
            {"id": "plumes", "title": t("Silvery plumes", "Silbrige Blütenwedel"), "kind": "surface", "model": "passive", "material": "pampas-plume", "areaM2": AREAS.get("plumes", round(r50["areas"]["plumes"], 2)), "characteristicLengthM": 0.13, "geometry": "cylinder", "skyViewFactor": 0.8, "layers": [], "heatCapacityJM2K": 500, "source": "authored-estimate", "reference": "jones-2014"},
            {"id": "culms", "title": t("Culms and tussock base", "Halme und Horstbasis"), "kind": "bark", "model": "passive", "material": "pampas-culm-mat", "areaM2": AREAS.get("culms", round(r50["areas"]["culms"], 2)), "characteristicLengthM": 0.015, "geometry": "cylinder", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": 9000, "source": "authored-estimate", "reference": "jones-2014"},
        ],
        "emitters": [{"id": "transpiration-blades", "kind": "transpiration", "title": t("Transpiration of the blades (distributed over the tussock)", "Transpiration der Blätter (über den Horst verteilt)"), "anchor": anchors[0], "anchors": anchors, "directionDeg": 90, "diameterM": 0.6, "region": "blades", "source": "authored-estimate", "reference": "monteith-unsworth-2013"}],
        "assumptions": [
            t("Leaf regions use the one-sided area of the 1:50 blades; the leaf area index is that area over the ellipse of the plan extent. The maximum stomatal conductance (5 mm/s) is an authored estimate for a tough, xeromorphic C3 tussock grass, guided by Jones (2014); Cortaderia is a C3 grass of the subfamily Danthonioideae. Aperture and night closure are the engine's business, not authored here.", "Blattregionen verwenden die einseitige Fläche der Blätter von 1:50; der Blattflächenindex ist diese Fläche bezogen auf die Ellipse der Grundrissausdehnung. Die maximale Spaltöffnungsleitfähigkeit (5 mm/s) ist eine Autorenschätzung für ein derbes, xeromorphes C3-Horstgras, angelehnt an Jones (2014); Cortaderia ist ein C3-Gras der Unterfamilie Danthonioideae. Öffnung und nächtliches Schließen sind Sache der Engine und werden hier nicht vorgegeben."),
            t("Culms, the tussock base and the plumes are passive. The areal heat capacity of the culm region is that of green plant tissue (about 3 MJ per m3 and K) times an effective depth of 3 mm, raised for the dense tussock base of old leaf sheaths; the plumes are light, open hair surfaces. No surface temperature is authored.", "Halme, Horstbasis und Blütenwedel sind passiv. Die flächenbezogene Wärmekapazität der Halmregion ist die von frischem Pflanzengewebe (etwa 3 MJ je m3 und K) mal einer wirksamen Tiefe von 3 mm, erhöht für die dichte Horstbasis aus alten Blattscheiden; die Wedel sind leichte, offene Haarflächen. Es werden keine Oberflächentemperaturen vorgegeben."),
            t("The transpiration emitter releases the latent flux of the blades as vapour over twelve anchors on the leading leaves of the six leaf bundles; the release is diffuse and points up.", "Der Transpirationsemitter gibt den latenten Strom der Blätter als Dampf über zwölf Anker an den führenden Blättern der sechs Blattbündel ab; die Abgabe ist diffus und zeigt nach oben."),
        ],
        "operating": [], "passengers": [],
    }


AREAS = {"plumes": 2.06}


def loads(kind, scale_bundle, scale_plume, source, reference, fy=False):
    out = []
    for b in BUNDLES:
        en = "left" if b[0] == "l" else "right"
        de = "links" if b[0] == "l" else "rechts"
        out.append({"id": f"{kind}-bl-{b}b", "kind": kind, "title": t(f"{'Wind drag' if kind == 'wind' else 'Snow'} on leaf bundle {b[1]} {en}, arching", f"{'Winddruck' if kind == 'wind' else 'Schnee'} auf Blattbündel {b[1]} {de}, überhängend"), "bone": f"bl-{b}b", "x": 0.4, "y": 0, "fxN": 0 if fy else scale_bundle, "fyN": -scale_bundle if fy else 0, "source": source, "reference": reference})
    for c in CULMS:
        out.append({"id": f"{kind}-plume-{c}", "kind": kind, "title": t(f"{'Wind drag' if kind == 'wind' else 'Snow'} on plume {c}", f"{'Winddruck' if kind == 'wind' else 'Schnee'} auf Blütenwedel {c}"), "bone": f"plume-{c}", "x": 0.3, "y": 0, "fxN": 0 if fy else scale_plume, "fyN": -scale_plume if fy else 0, "source": source, "reference": reference})
    return out


BUNDLE_AREA, PLUME_AREA = 0.14, 0.084
V_WINDY, V_STORM, V_CALM = 5.4, 14.5, 1.5
F = {k: (round(L.wind_force(BUNDLE_AREA, cd, v), 2), round(L.wind_force(PLUME_AREA, cdp, v), 2)) for k, cd, cdp, v in [("calm", 0.5, 0.6, V_CALM), ("windy", 0.5, 0.6, V_WINDY), ("storm", 0.3, 0.4, V_STORM)]}


def deltas(a, b, plume, bla, blb):
    out = {}
    for c in CULMS:
        out.update({f"culm-{c}a": a, f"culm-{c}b": b, f"plume-{c}": plume})
    for n in BUNDLES:
        out.update({f"bl-{n}a": bla, f"bl-{n}b": blb})
    return out


WINDY = deltas(-3, -5, -7, -2, -5)
STORM = deltas(-6, -12, -16, -4, -12)
SNOW = {**{f"culm-{c}a": (-2 if CULM[c][1] < 0 else 2) for c in CULMS}, **{f"culm-{c}b": (-5 if CULM[c][1] < 0 else 5) for c in CULMS}, **{f"plume-{c}": (-6 if CULM[c][1] < 0 else 6) for c in CULMS}, **{f"bl-{n}b": (14 if n[0] == "l" else -14) for n in BUNDLES}}


def binding(blades, plumes, culms, emit, wet=None):
    regions = {"blades": {"exposedFraction": 1, "sunlitFraction": blades}, "plumes": {"exposedFraction": 1, "sunlitFraction": plumes}, "culms": {"exposedFraction": 1, "sunlitFraction": culms}}
    if wet is not None:
        for r, w in zip(regions.values(), wet):
            r["wettedness"] = w
    return {"regions": regions, "emitters": {"transpiration-blades": emit}}


def poses():
    calm, windy, storm = F["calm"], F["windy"], F["storm"]
    return [
        L.pose("calm", t("Calm late-summer day", "Windstiller Spätsommertag"), t(f"Light air of {V_CALM} m/s: the blades fountain freely from the tussock and the eight plumes stand upright above them; drag is {calm[0]} N per leaf bundle and {calm[1]} N per plume.", f"Leiser Zug von {L.fmt(V_CALM, 'de')} m/s: Die Blätter fallen frei als Fontäne aus dem Horst, die acht Blütenwedel stehen aufrecht darüber; der Widerstand beträgt {L.fmt(calm[0], 'de')} N je Blattbündel und {L.fmt(calm[1], 'de')} N je Wedel."), {"plume-1": -0.5, "plume-4": 0.5}, binding(0.6, 0.85, 0.15, 0.8), loads("wind", calm[0], calm[1], "literature", "en-1991-1-4")),
        L.pose("windy", t("Fresh breeze", "Frische Brise"), t(f"Beaufort 5, 8 m/s at 10 m, about {V_WINDY} m/s at plume height (logarithmic profile, z0 = 0.05 m); cd 0.5 on the porous leaf bundles (0.14 m2 each) and 0.6 on the dense plumes (0.6 x 0.14 m): {windy[0]} N per bundle, {windy[1]} N per plume. The stiff culms bend 3 to 5 degrees, the plumes lean 15 degrees in all and the blade tips stream to the lee.", f"Beaufort 5, 8 m/s in 10 m Höhe, etwa {L.fmt(V_WINDY, 'de')} m/s in Wedelhöhe (logarithmisches Profil, z0 = 0,05 m); cd 0,5 auf die durchlässigen Blattbündel (je 0,14 m2) und 0,6 auf die dichten Wedel (0,6 x 0,14 m): {L.fmt(windy[0], 'de')} N je Bündel, {L.fmt(windy[1], 'de')} N je Wedel. Die steifen Halme biegen sich 3 bis 5 Grad, die Wedel neigen sich insgesamt 15 Grad, die Blattspitzen strömen nach Lee."), WINDY, binding(0.6, 0.85, 0.15, 1), loads("wind", windy[0], windy[1], "literature", "en-1991-1-4")),
        L.pose("storm", t("Gale", "Sturm"), t(f"Beaufort 9, 22 m/s at 10 m, about {V_STORM} m/s at plume height; the blades streamline (cd 0.3) and the plumes comb out (cd 0.4): {storm[0]} N per bundle, {storm[1]} N per plume. The culms lodge up to 34 degrees at the plume, the leaf fountain is blown flat to the lee.", f"Beaufort 9, 22 m/s in 10 m Höhe, etwa {L.fmt(V_STORM, 'de')} m/s in Wedelhöhe; die Blätter legen sich an (cd 0,3), die Wedel werden ausgekämmt (cd 0,4): {L.fmt(storm[0], 'de')} N je Bündel, {L.fmt(storm[1], 'de')} N je Wedel. Die Halme legen sich am Wedel bis 34 Grad um, die Blattfontäne wird nach Lee flachgedrückt."), STORM, binding(0.35, 0.45, 0.05, 0.5, (0.4, 0.4, 0.4)), loads("wind", storm[0], storm[1], "literature", "vogel-1989")),
        L.pose("snow", t("Wet snow on the winter tussock", "Nassschnee auf dem Winterhorst"), t("The evergreen blades and the plumes that stand through winter catch wet snow: about 2 kg per leaf bundle (20 N) and 0.3 kg per plume (3 N) bow the arching blades down and outward by 14 degrees and the culms outward by up to 13 degrees at the plume; calm air (order of magnitude after EN 1991-1-3, authored estimate).", "Die wintergrünen Blätter und die über den Winter stehenden Wedel fangen Nassschnee: etwa 2 kg je Blattbündel (20 N) und 0,3 kg je Wedel (3 N) drücken die überhängenden Blätter um 14 Grad nach unten und außen und die Halme am Wedel bis 13 Grad nach außen; Windstille (Größenordnung nach EN 1991-1-3, Schätzung)."), SNOW, binding(0, 0, 0, 0, (0.5, 0.3, 0.3)), loads("snow", 20, 3, "authored-estimate", "en-1991-1-3", fy=True)),
    ]


def clips():
    amplitude = {**{f"culm-{c}a": 0.8 for c in CULMS}, **{f"culm-{c}b": 1.6 for c in CULMS}, **{f"plume-{c}": 3.5 for c in CULMS}, **{f"bl-{n}a": 1.0 for n in BUNDLES}, **{f"bl-{n}b": 3.5 for n in BUNDLES}}
    base = {**{f"culm-{c}": CULM[c][0] for c in CULMS}, **{f"bl-{n}": BUNDLE[n][0] for n in BUNDLES}}
    phase = {b: -2 * math.pi * base[b[:-1] if b[-1] in "ab" else "culm-" + b.split("-")[1]][0] / 1.6 + (0.4 if b.startswith("plume") else 0.2 if b.endswith("b") else 0) for b in amplitude}
    spec = {"sway_period": 2.6, "emitter": "transpiration-blades",
            "sway_text": t("A deterministic 2.6 s cycle (stiff culms of a large tussock, estimated 0.38 Hz) about the fresh-breeze lean: the culms bend gently, the heavy plumes swing by 3.5 degrees and the blade tips by 3.5 degrees with a phase that travels downwind through the tussock.", "Ein deterministischer Zyklus von 2,6 s (steife Halme eines großen Horstes, geschätzt 0,38 Hz) um die Neigung der frischen Brise: Die Halme biegen sich sanft, die schweren Wedel schwingen 3,5 Grad und die Blattspitzen 3,5 Grad mit einer Phase, die windabwärts durch den Horst wandert."),
            "gust_text": t("A single 8 s gust: the wind-load tracks rise from 5 % to 2.75 times the fresh-breeze load at 2.5 s (EN 1991-1-4 peak pressure with Iv 0.25) and decay; culms, plumes and blades lean up to 70 % of the gale lean and spring back.", "Eine einzelne Böe über 8 s: Die Windlast-Spuren steigen bei 2,5 s von 5 % auf das 2,75-fache der Last der frischen Brise (Spitzendruck nach EN 1991-1-4 mit Iv 0,25) und klingen ab; Halme, Wedel und Blätter neigen sich bis zu 70 % der Sturmneigung und federn zurück."),
            "diurnal_text": t("A 24 h cycle compressed to 24 s (1 s = 1 h, 00:00 to 24:00), calm weather. The sunlit fraction of every region and the transpiration strength follow the sun, stomata nearly closed at night; leaf and culm temperatures are not authored but follow from the thermal solver.", "Ein 24-h-Zyklus auf 24 s gerafft (1 s = 1 h, 00:00 bis 24:00) bei ruhigem Wetter. Der besonnte Anteil jeder Region und die Transpirationsstärke folgen der Sonne, nachts sind die Stomata fast geschlossen; Blatt- und Halmtemperaturen werden nicht vorgegeben, sondern folgen aus dem thermischen Löser.")}
    wind_ids = [l["id"] for l in loads("wind", 1, 1, "literature", "en-1991-1-4")]
    return [L.sway_clip(spec, WINDY, amplitude, phase), L.gust_clip(spec, STORM, wind_ids), L.diurnal_clip(spec, {"blades": 0.6, "plumes": 0.85, "culms": 0.15})]


def D(en, de):
    return t(en, de)


REP = {
    50: {"silhouette": D("At 1:50 (1 m is 20 mm) the tussock is drawn leaf by leaf as the true view of its 3D body: a dense fountain of long, narrow, arching blades from a broad crown, and eight stout culms that carry dense, upright, silvery plumes well above the foliage. Side elevation, front elevation and plan are projections of the same body.", "Bei 1:50 (1 m sind 20 mm) ist der Horst Blatt für Blatt als wahre Ansicht seines 3D-Körpers gezeichnet: eine dichte Fontäne langer, schmaler, bogig überhängender Blätter aus einer breiten Horstbasis und acht kräftige Halme, die dichte, aufrechte, silbrige Blütenwedel weit über das Laub tragen. Seitenansicht, Vorderansicht und Grundriss sind Projektionen desselben Körpers."),
         "retained": D("Retained: every blade of the six leaf bundles and their four side shoots, eighteen basal blades, the eight culms in two segments with their sheathing leaves, each plume as a main body and eight silky lobes, the tussock mound with eight pairs of old culm stubs.", "Beibehalten: jedes Blatt der sechs Blattbündel und ihrer vier Seitentriebe, achtzehn Grundblätter, die acht Halme in zwei Abschnitten mit ihren Scheidenblättern, jeder Wedel als Hauptkörper mit acht seidigen Lappen, der Horsthügel mit acht Paaren alter Halmstoppeln."),
         "merged": D("Merged: the thousands of spikelets of a plume into its lobes, leaf sheaths into the stems they clasp, old dead leaves into the mound.", "Zusammengefasst: die tausenden Ährchen eines Wedels in seine Lappen, Blattscheiden mit den Halmen, die sie umfassen, alte tote Blätter in den Hügel."),
         "omitted": D("Omitted: the serrated leaf margins, the midrib line, nodes and single spikelets, all below 0.15 mm on paper.", "Weggelassen: die gesägten Blattränder, die Mittelrippe, Knoten und einzelne Ährchen, alle unter 0,15 mm auf dem Papier."),
         "thinElements": D("Each blade, stem and culm is a filled outline, the true projection of its solid (blades 11 to 12 mm, 0.22 to 0.24 mm on paper; culms 17 mm, 0.34 mm; a blade seen edge-on is 0.4 mm thin); the outline pen of 0.2 mm carries the thin ones, so nothing prints below 0.2 mm.", "Jedes Blatt, jeder Trieb und Halm ist ein gefüllter Umriss, die wahre Projektion seines Körpers (Blätter 11 bis 12 mm, 0,22 bis 0,24 mm auf dem Papier; Halme 17 mm, 0,34 mm; ein von der Kante gesehenes Blatt ist 0,4 mm dünn); der Umrissstift von 0,2 mm trägt die dünnen Teile, nichts druckt unter 0,2 mm."),
         "hatching": D("No hatching; the overlap of the blade outlines carries the volume.", "Keine Schraffur; die Überlagerung der Blattumrisse trägt das Volumen."),
         "styles": D("Line views: every part as a paper-filled outline, the nearer over the farther, so the fountain reads with hidden lines. Solid views: the same outlines in the tone of their region, plumes light, culms dark.", "Linienansichten: jedes Teil als papiergefüllter Umriss, das nähere über dem ferneren, sodass die Fontäne mit verdeckten Linien lesbar ist. Flächenansichten: dieselben Umrisse im Ton ihrer Region, Wedel hell, Halme dunkel."),
         "overlays": D("Every outline is bound to its thermal region (blades, plumes, culms) and to the bone that carries it, so the thermal and structural views colour the same drawing.", "Jeder Umriss ist an seine thermische Region (Blätter, Wedel, Halme) und an den tragenden Knochen gebunden, sodass Thermik- und Tragwerksansicht dieselbe Zeichnung einfärben.")},
    100: {"silhouette": D("At 1:100 (1 m is 10 mm) the fountain keeps every blade of the 1:50 plant, each typified at 17 mm, just above the smallest feature of the scale (15 mm; the species reaches 20 mm), with seven stations instead of eleven, so the arcs become polylines that still read as curves at 10 mm per metre; the shoot axes are straight. The outline of the tussock is that of 1:50. Each plume is a fan of six silky lobes on its culm. All three views are projections of the 1:100 body.", "Bei 1:100 (1 m sind 10 mm) behält die Fontäne jedes Blatt der Pflanze von 1:50, jedes auf 17 mm typisiert, knapp über dem kleinsten Merkmal des Maßstabs (15 mm; die Art erreicht 20 mm), mit sieben statt elf Stationen, sodass die Bögen zu Polylinien werden, die bei 10 mm je Meter noch als Kurven lesbar sind; die Triebachsen sind gerade. Der Umriss des Horstes ist der von 1:50. Jeder Wedel ist ein Fächer aus sechs seidigen Lappen auf seinem Halm. Alle drei Ansichten sind Projektionen des Körpers von 1:100."),
          "retained": D("Retained: all blades of the six leaf bundles, their side shoots and the basal skirt, the eight culms with their leaves and companion shoots, six plume lobes per culm, the mound with six single stubs.", "Beibehalten: alle Blätter der sechs Blattbündel, ihrer Seitentriebe und des Grundblattrocks, die acht Halme mit ihren Blättern und Begleittrieben, sechs Wedellappen je Halm, der Hügel mit sechs einzelnen Stoppeln."),
          "merged": D("Merged: stub pairs into single stubs, nine plume lobes into six.", "Zusammengefasst: Stoppelpaare zu einzelnen Stoppeln, neun Wedellappen zu sechs."),
          "omitted": D("Omitted: two of the eight stub pairs, the fine curvature between the stations, nodes and spikelets.", "Weggelassen: zwei der acht Stoppelpaare, die feine Krümmung zwischen den Stationen, Knoten und Ährchen."),
          "thinElements": D("Blades 17 mm (0.17 mm on paper), culms 9 to 17 mm, shoot axes 7 to 12 mm, all as filled outlines with the 0.2 mm pen; nothing prints below 0.2 mm.", "Blätter 17 mm (0,17 mm auf dem Papier), Halme 9 bis 17 mm, Triebachsen 7 bis 12 mm, alle als gefüllte Umrisse mit dem Stift von 0,2 mm; nichts druckt unter 0,2 mm."),
          "hatching": D("No hatching.", "Keine Schraffur."),
          "styles": D("Line views: paper-filled outlines of blades, culms, mound and plume lobes, the nearer over the farther. Solid views: the same shapes in the tone of their region.", "Linienansichten: papiergefüllte Umrisse von Blättern, Halmen, Hügel und Wedellappen, das Nähere über dem Ferneren. Flächenansichten: dieselben Formen im Ton ihrer Region."),
          "overlays": D("Every shape is bound to its thermal region and to the bone that carries it. The range ends at 1:125, where the narrowest plume lobe still prints at 0.2 mm.", "Jede Form ist an ihre thermische Region und an den tragenden Knochen gebunden. Der Bereich endet bei 1:125, wo der schmalste Wedellappen noch mit 0,2 mm druckt.")},
    200: {"silhouette": D("At 1:200 (1 m is 5 mm) a single blade is far below the smallest feature (30 mm), so the fountain is typified: ribbons of 34 mm stand for two to three leaves each and follow the paths of leaves of the 1:50 plant, so the outline of the fountain stays that of the finer scales; the inner companion shoots and stubs are dropped, and every plume is a body of four lobes. All three views are projections of the 1:200 body.", "Bei 1:200 (1 m sind 5 mm) liegt ein einzelnes Blatt weit unter dem kleinsten Merkmal (30 mm), die Fontäne ist daher typisiert: Bänder von 34 mm stehen je für zwei bis drei Blätter und folgen den Bahnen von Blättern der Pflanze von 1:50, sodass der Umriss der Fontäne der der feineren Maßstäbe bleibt; die inneren Begleittriebe und Stoppeln entfallen, und jeder Wedel ist ein Körper aus vier Lappen. Alle drei Ansichten sind Projektionen des Körpers von 1:200."),
          "retained": D("Retained: per leaf bundle a main shoot with two and four ribbons and four side shoots with two ribbons each, nine basal ribbons, eight culms with one ribbon each, eight plumes of four lobes, the mound.", "Beibehalten: je Blattbündel ein Haupttrieb mit zwei und vier Bändern und vier Seitentriebe mit je zwei Bändern, neun Grundbänder, acht Halme mit je einem Band, acht Wedel aus vier Lappen, der Hügel."),
          "merged": D("Merged: two to three leaves into one ribbon of 34 mm, the plume into four lobes, stubble into the mound.", "Zusammengefasst: zwei bis drei Blätter zu einem Band von 34 mm, der Wedel in vier Lappen, Stoppeln in den Hügel."),
          "omitted": D("Omitted: companion shoots, upper culm leaves and stubs; they lie inside the fountain.", "Weggelassen: Begleittriebe, obere Halmblätter und Stoppeln; sie liegen innerhalb der Fontäne."),
          "thinElements": D("Ribbons of 34 mm (0.17 mm on paper) and culms of 9 to 17 mm as filled outlines with the 0.2 mm pen.", "Bänder von 34 mm (0,17 mm auf dem Papier) und Halme von 9 bis 17 mm als gefüllte Umrisse mit dem Stift von 0,2 mm."),
          "hatching": D("No hatching.", "Keine Schraffur."),
          "styles": D("Line views: paper-filled outlines of ribbons, culms, mound and plumes. Solid views: the same shapes as tonal masses ordered by depth.", "Linienansichten: papiergefüllte Umrisse von Bändern, Halmen, Hügel und Wedeln. Flächenansichten: dieselben Formen als getönte Massen, nach Tiefe geordnet."),
          "overlays": D("Every shape is bound to its thermal region and to the bone that carries it.", "Jede Form ist an ihre thermische Region und an den tragenden Knochen gebunden.")},
    500: {"silhouette": D("At 1:500 (1 m is 2 mm) the tussock is a mop of pointed arching tongues that rise from the broad base and hang out on all sides, with eight plume bodies on fine culm lines above it; the tongues follow their bones, so the tussock still leans in the wind. All three views are projections of the 1:500 body.", "Bei 1:500 (1 m sind 2 mm) ist der Horst ein Schopf spitzer, bogiger Zungen, die aus der breiten Basis aufsteigen und ringsum überhängen, mit acht Wedelkörpern auf feinen Halmlinien darüber; die Zungen folgen ihren Knochen, sodass sich der Horst weiterhin im Wind neigt. Alle drei Ansichten sind Projektionen des Körpers von 1:500."),
          "retained": D("Retained: a rising and an arching tongue per leaf bundle, eighteen side tongues, sixteen upright tongues, eight culm lines, eight plumes and the mound.", "Beibehalten: eine aufsteigende und eine überhängende Zunge je Blattbündel, achtzehn Seitenzungen, sechzehn aufrechte Zungen, acht Halmlinien, acht Wedel und der Hügel."),
          "merged": D("Merged: all leaves of a shoot into its tongues, stubble into the mound, every plume into one body.", "Zusammengefasst: alle Blätter eines Triebs in dessen Zungen, Stoppeln in den Hügel, jeder Wedel in einen Körper."),
          "omitted": D("Omitted: single leaves, shoots and plume lobes; they are far below 0.1 mm on paper.", "Weggelassen: einzelne Blätter, Triebe und Wedellappen; sie liegen weit unter 0,1 mm auf dem Papier."),
          "thinElements": D("The culms are filled outlines of their 16 mm body, carried by the 0.2 mm outline pen; tongues are 80 to 90 mm thick, 0.16 to 0.18 mm on paper.", "Die Halme sind gefüllte Umrisse ihres 16 mm starken Körpers, getragen vom Umrissstift von 0,2 mm; die Zungen sind 80 bis 90 mm dick, 0,16 bis 0,18 mm auf dem Papier."),
          "hatching": D("No hatching.", "Keine Schraffur."),
          "styles": D("Line views: restrained outlines of tongues, plumes and mound. Solid views: tonal masses ordered by depth.", "Linienansichten: zurückhaltende Umrisse von Zungen, Wedeln und Hügel. Flächenansichten: getönte Massen, nach Tiefe geordnet."),
          "overlays": D("Every shape is bound to its thermal region and to the bone that carries it.", "Jede Form ist an ihre thermische Region und an den tragenden Knochen gebunden.")},
}

VOL = {
    50: [D("1:50 body: tussock mound with eight pairs of stubs, eighteen basal blades; per leaf bundle a main shoot (stem on both bones, five and nine leaves at its nodes) and four side shoots of five leaves; per culm two straight segments with three and one sheathing leaves, one companion shoot of five leaves and a plume of a main body and eight silky lobes.", "1:50-Körper: Horsthügel mit acht Stoppelpaaren, achtzehn Grundblätter; je Blattbündel ein Haupttrieb (Stängel über beide Knochen, fünf und neun Blätter an seinen Knoten) und vier Seitentriebe mit je fünf Blättern; je Halm zwei gerade Abschnitte mit drei und einem Scheidenblatt, ein Begleittrieb mit fünf Blättern und ein Wedel aus Hauptkörper und acht seidigen Lappen."),
         D("Every leaf is one whole solid on one bone fixed at its node on the bone axis, so no pose opens a gap; paths that would leave the envelope of `dimensions` are shortened to staggered lengths, the outermost leaves end on it. Every part is a closed triangle mesh (`polyhedron`) through its stated sections: blades as rhombic sheets 0.4 mm thick, stems and culms as octagonal prisms, plume lobes with sixteen sides, the mound with thirty-two; each drawing is the outline of the projected mesh, so the three views and the volume are the same polygons.", "Jedes Blatt ist ein ganzer Körper an einem Knochen und an seinem Knoten auf der Knochenachse befestigt, sodass keine Pose eine Lücke öffnet; Bahnen, die die Hülle von `dimensions` verlassen würden, sind auf gestaffelte Längen gekürzt, die äußersten Blätter enden auf ihr. Jedes Teil ist ein geschlossenes Dreiecksnetz (`polyhedron`) durch seine angegebenen Querschnitte: Blätter als rhombische Platten von 0,4 mm Dicke, Triebe und Halme als achteckige Prismen, Wedellappen mit sechzehn Seiten, der Hügel mit zweiunddreißig; jede Zeichnung ist der Umriss des projizierten Netzes, sodass die drei Ansichten und das Volumen dieselben Polygone sind."),
         D("Thermal: every body part by reference (blades, plumes, culms) and twelve transpiration strips on the leading leaves; structure: rhizome disc, culm rods of 15 and 11 mm, shoot axes, plume axes, joints, support disc and lumped masses.", "Thermik: jeder Körperteil per Verweis (Blätter, Wedel, Halme) und zwölf Transpirationsstreifen auf den führenden Blättern; Tragwerk: Rhizomscheibe, Halmstäbe von 15 und 11 mm, Triebachsen, Wedelachsen, Gelenke, Auflagerscheibe und konzentrierte Massen.")],
    100: [D("1:100 body: the 1:50 plant with seven stations per blade, blades of 17 mm (the smallest feature is 15 mm), straight shoot axes, single stubs, plumes of six lobes; the outline is that of 1:50.", "1:100-Körper: die Pflanze von 1:50 mit sieben Stationen je Blatt, Blättern von 17 mm (das kleinste Merkmal ist 15 mm), geraden Triebachsen, einzelnen Stoppeln, Wedeln aus sechs Lappen; der Umriss ist der von 1:50."),
          D("Thermal by reference plus twelve strips; structure with simplified rods and larger joints.", "Thermik per Verweis plus zwölf Streifen; Tragwerk mit vereinfachten Stäben und größeren Gelenken.")],
    200: [D("1:200 body: ribbons of 34 mm with six stations, each for two to three leaves (the smallest feature is 30 mm), straight shoot axes, one ribbon per culm, no companion shoots or stubs, plumes of four lobes.", "1:200-Körper: Bänder von 34 mm mit sechs Stationen, je für zwei bis drei Blätter (das kleinste Merkmal ist 30 mm), gerade Triebachsen, ein Band je Halm, keine Begleittriebe oder Stoppeln, Wedel aus vier Lappen."),
          D("Thermal by reference plus twelve strips; structure without joints.", "Thermik per Verweis plus zwölf Streifen; Tragwerk ohne Gelenke.")],
    500: [D("1:500 body: mound, per bundle a rising and an arching tongue and three side tongues, per culm two straight segments of 16 mm, two upright tongues and one plume body; every tongue is drawn as the outline of its own body.", "1:500-Körper: Hügel, je Bündel eine aufsteigende und eine überhängende Zunge und drei Seitenzungen, je Halm zwei gerade Segmente von 16 mm, zwei aufrechte Zungen und ein Wedelkörper; jede Zunge ist als Umriss ihres eigenen Körpers gezeichnet."),
          D("Thermal by reference plus twelve strips; structure with three-point rods.", "Thermik per Verweis plus zwölf Streifen; Tragwerk mit Dreipunktstäben.")],
}

REFERENCES = [
    {"id": "en-1991-1-4", "citation": "EN 1991-1-4:2005 Eurocode 1, Actions on structures, Part 1-4: Wind actions. Air density 1.25 kg/m3, F = 0.5 rho cd A v^2, peak velocity pressure (1 + 7 Iv) q_m."},
    {"id": "en-1991-1-3", "citation": "EN 1991-1-3:2003 Eurocode 1, Actions on structures, Part 1-3: Snow loads. Used for the order of magnitude of snow accretion on foliage."},
    {"id": "vogel-1989", "citation": "Vogel, S. (1989). Drag and reconfiguration of broad leaves in high winds. Journal of Experimental Botany 40(8), 941-948."},
    {"id": "jones-2014", "citation": "Jones, H. G. (2014). Plants and Microclimate, 3rd ed., Cambridge University Press. Leaf energy balance, leaf area index, boundary-layer conductance."},
    {"id": "monteith-unsworth-2013", "citation": "Monteith, J. L. and Unsworth, M. H. (2013). Principles of Environmental Physics, 4th ed., Academic Press. Penman-Monteith leaf transpiration."},
    {"id": "kew-grassbase-cortaderia", "citation": "Clayton, W. D., Vorontsova, M. S., Harman, K. T. and Williamson, H. (2006 onwards). GrassBase - The Online World Grass Flora, Royal Botanic Gardens, Kew: Cortaderia selloana. Culms 100-300 cm long; panicle open, ovate, dense, 25-100 cm long. https://www.kew.org/data/grasses-db/www/imp02552.htm"},
    {"id": "ausgrass-cortaderia", "citation": "AusGrass2, Grasses of Australia: Cortaderia selloana. Culms 100-400 cm tall; leaf blades 60-200 cm long, 3-15 mm wide; panicle 25-100 cm. https://ausgrass2.myspecies.info/node/640"},
    {"id": "edgar-connor-2000", "citation": "Edgar, E. and Connor, H. E. (2000). Flora of New Zealand, Volume V: Gramineae. Manaaki Whenua Press, Lincoln. Cortaderia selloana: very tall, stout, erect tussock with sharp leaves; blades to 2 m x 2 cm."},
]


def spec():
    return {
        "id": ID, "target": TARGET, "plant_module": PLANT, "crown_length": CROWN, "root_mass": ROOT_KG, "mass_reference": "jones-2014",
        "root_title": t("Tussock base and rhizome mat", "Horstbasis und Rhizomfilz"),
        "root_section": L.circle_section(0.6, "pampas-culm-mat", "Solid section, d = 60 cm", "Vollquerschnitt, d = 60 cm"),
        "bones": BONES, "lateral": LATERAL,
        "rename": {"miscanthus-blade": "pampas-blade", "miscanthus-culm": "pampas-culm", "miscanthus-culm-mat": "pampas-culm-mat", "miscanthus-plume": "pampas-plume"},
        "leaf_material": "pampas-blade", "leaf_region": "blades", "emitter": "transpiration-blades", "emitter_title": t("Transpiration of the blades", "Transpiration der Blätter"),
        "stem_material": "pampas-culm", "mat_material": "pampas-culm-mat", "top_material": "pampas-plume",
        "mat": {50: [(0.002, 0.3), (0.05, 0.28), (0.1, 0.19), (0.125, 0.13)], 100: [(0.004, 0.3), (0.06, 0.26), (0.1, 0.19), (0.125, 0.13)], 200: [(0.004, 0.3), (0.1, 0.19), (0.125, 0.13)], 500: [(0.004, 0.3), (0.1, 0.2), (0.13, 0.15)]},
        "member_radius": member_radius, "joint_radius": {50: 0.012, 100: 0.011}, "support_radius": 0.3,
        "crown_mass_radii": {50: [0.05, 0.12, 0.12], 100: [0.05, 0.11, 0.11], 200: [0.05, 0.1, 0.1], 500: [0.05, 0.09, 0.09]},
        "masses": masses, "footprint": 0.68,
        "support_title": t("Tussock and rhizome anchorage", "Verankerung von Horst und Rhizom"),
        "title": t("Pampas Grass", "Pampasgras"),
        "summary": t("A flowering tussock of pampas grass (Cortaderia selloana) 2.5 m tall, 2.2 m wide and 2 m deep: a broad crown of old leaf bases and stubs, a dense fountain of long, narrow, sharp-edged arching blades, and eight stout culms that rise well above the foliage and end in dense, upright, silvery plumes 60 cm long.", "Ein blühender Horst Pampasgras (Cortaderia selloana), 2,5 m hoch, 2,2 m breit und 2 m tief: eine breite Horstbasis aus alten Blattgründen und Stoppeln, eine dichte Fontäne langer, schmaler, scharfkantiger, bogig überhängender Blätter und acht kräftige Halme, die weit über das Laub aufsteigen und in dichten, aufrechten, silbrigen Blütenwedeln von 60 cm Länge enden."),
        "tags": ["grass", "perennial", "tussock", "plumes", "ornamental", "wind", "transpiration"],
        "provenance": {"author": "Semio staffage vegetation content worker PW-15", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES, "notes": t("Modelled as a real tussock in the construction of the round-3 miscanthus clump: six leaf bundles on an outer ring, each with a main shoot whose leaves sit at its nodes and four side shoots, eighteen basal blades and eight flowering culms on an inner ring. Blades 0.85 to 1.35 m long and 11 to 12 mm wide (AusGrass2: 60-200 cm x 3-15 mm), culms 17 mm thick at the base and 2 to 2.5 m tall to the plume tip (Kew GrassBase: culms 100-300 cm), plumes 60 cm long and 14 cm wide (panicle 25-100 cm, dense). The rig is the side view of the three-dimensional shoots: every joint hinges about a lateral axis and carries its lateral place as `z`. Female plants have the fuller, silkier plumes; the plumes stand through winter, so the snow pose keeps them.", "Als wirklicher Horst in der Bauweise des Chinaschilf-Horstes aus Runde 3 modelliert: sechs Blattbündel auf einem äußeren Ring, jedes mit einem Haupttrieb, dessen Blätter an seinen Knoten sitzen, und vier Seitentrieben, achtzehn Grundblätter und acht Blütenhalme auf einem inneren Ring. Blätter 0,85 bis 1,35 m lang und 11 bis 12 mm breit (AusGrass2: 60-200 cm x 3-15 mm), Halme am Fuß 17 mm dick und bis zur Wedelspitze 2 bis 2,5 m hoch (Kew GrassBase: Halme 100-300 cm), Wedel 60 cm lang und 14 cm breit (Rispe 25-100 cm, dicht). Das Skelett ist die Seitenansicht der dreidimensionalen Triebe: Jedes Gelenk dreht um eine Querachse und trägt seine seitliche Lage als `z`. Weibliche Pflanzen tragen die volleren, seidigeren Wedel; die Wedel stehen über den Winter, daher behält die Schneepose sie.")},
        "dimensions": {"heightM": HEIGHT, "widthM": WIDTH, "depthM": DEPTH, "massKg": MASS},
        "materials": [
            {"id": "pampas-blade", "title": t("Pampas grass blades", "Pampasgras-Blätter"), "emissivity": 0.97, "solarAbsorptivity": 0.55, "source": "literature", "reference": "jones-2014"},
            {"id": "pampas-culm-mat", "title": t("Tussock base of old leaf sheaths", "Horstbasis aus alten Blattscheiden"), "emissivity": 0.93, "solarAbsorptivity": 0.65, "densityKgM3": 320, "conductivityWmK": 0.08, "source": "authored-estimate", "reference": "jones-2014"},
            {"id": "pampas-plume", "title": t("Silvery plumes", "Silbrige Blütenwedel"), "emissivity": 0.9, "solarAbsorptivity": 0.3, "source": "authored-estimate", "reference": "jones-2014"},
            {"id": "pampas-culm", "title": t("Green pampas grass culm (pith-filled, solid-section equivalent)", "Grüner Pampasgrashalm (markgefüllt, Vollquerschnitt-Äquivalent)"), "emissivity": 0.9, "solarAbsorptivity": 0.55, "densityKgM3": 380, "conductivityWmK": 0.1, "elasticModulusPa": 3000000000, "strengthPa": 35000000, "source": "authored-estimate"},
        ],
        "poses": poses(), "clips": clips(), "thermal": thermal,
        "structure": {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": ["base-support"], "assumptions": [t("One rigid body per evaluated pose, fixed at the tussock base; wind drag acts on every leaf bundle and plume, snow on the same parts, gravity on all bone masses. Culm sections are solid circles with the stiffness of green pampas culm tissue (authored estimate, pith-filled culm replaced by an equivalent solid). Dynamic amplification and uprooting are not modelled.", "Ein starrer Körper je ausgewerteter Pose, an der Horstbasis eingespannt; Winddruck wirkt auf jedes Blattbündel und jeden Wedel, Schnee auf dieselben Teile, Schwerkraft auf alle Knochenmassen. Halmquerschnitte sind Vollkreise mit der Steifigkeit von frischem Pampasgrashalm (Schätzung, markgefüllter Halm durch ein äquivalentes Vollprofil ersetzt). Dynamische Überhöhung und Entwurzeln werden nicht modelliert.")], "clinical": False},
        "volume_decisions": VOL, "representation_decisions": REP,
        "ranges": {50: (20, 75), 100: (75, 125), 200: (125, 350), 500: (350, 1000)},
        "symbol_lobes": (25, 0.2), "symbol_stems": [(0.0, 0.0, 0.3)],
    }


TRIM = {50: 1.02, 100: 1.02, 200: 1.02}
TURN = {50: {"blade-l2s3-4": 5.0}, 100: {}, 200: {}}
if os.environ.get("PW15_TRIM"):
    TRIM.update({int(k): float(v) for k, v in (item.split(":") for item in os.environ["PW15_TRIM"].split(","))})
if os.environ.get("PW15_TURN"):
    for item in os.environ["PW15_TURN"].split(","):
        scale_, part_, degrees_ = item.split(":")
        TURN[int(scale_)][part_] = float(degrees_)


def trimmed(table, k):
    """🎚️ The table with every blade width times k (within 2 % of the stated width). The catalog-silhouette guard fills enclosed gaps of up to 256 raster cells and closes hairline slits on the body raster only, so a gap between crossing blades of exactly 257 cells counts as a stray boundary although drawing and body are the same polygons (measured: 62 mm at 1:50); the factor of a scale is chosen so that no gap of its three views sits on that threshold. `PW15_TRIM=50:1.01,100:0.99` in the environment tries other factors."""
    return {**table, **{key: [(*r[:-1], round(r[-1] * k, 6)) for r in table[key]] for key in ("main_a", "main_b", "satellite_leaves", "culm_a_leaves", "culm_b_leaves", "basal")}}


def turned(plant, turns):
    """🧭 Turns single blades about the vertical through their node by a few degrees (hand corrections of the stated plant, for the same reason as `trimmed`: a blade that closes a gap of the guard's raster by one cell is moved off it). `PW15_TURN=50:blade-c3t1-3:4` in the environment tries others."""
    for part in plant.parts:
        degrees = turns.get(part["id"])
        if not degrees:
            continue
        c, r = math.cos(math.radians(degrees)), math.sin(math.radians(degrees))
        o = part["points"][0]
        spin = lambda v: (v[0] * c - v[2] * r, v[1], v[0] * r + v[2] * c)
        part["points"] = [K.add(o, spin(K.sub(p, o))) for p in part["points"]]
        part["widths"] = [spin(w) for w in part["widths"]]
    missing = set(turns) - {part["id"] for part in plant.parts}
    assert not missing, f"no such parts to turn: {missing}"
    return plant


SETTLE = M.settle


def straight(path, diameter):
    """📏 Coarser than 1:50 every shoot axis is below the smallest feature and is a straight slender rod (the chord of its 1:50 path); its leaves sit at their nodes on that chord."""
    M.BOWS.append(0.0)
    return [K.lerp(path[0], path[-1], i / (len(path) - 1)) for i in range(len(path))]


def fine(scale, table):
    def run(rig):
        M.settle = SETTLE if scale == 50 else straight
        return turned(M.fine(rig, scale, table, FIT), TURN[scale])
    return run


def coarse(rig):
    """🌾 1:500: the round-3 tongues without their `envelope` flag: every tongue is drawn as the outline of its own body, so the scale is compared with 1:200 body against body like its drawings, not hull against hull."""
    plant = M.coarse500(rig, S500, FIT)
    for part in plant.parts:
        part.pop("envelope", None)
    return plant


BUILDERS = {50: fine(50, trimmed(S50, TRIM[50])), 100: fine(100, trimmed(S100, TRIM[100])), 200: fine(200, trimmed(S200, TRIM[200])), 500: coarse}
FIT = {"y": 1.0, "sheaf": 1.0}


def fitted(maker):
    """📏 Fits the lengths of the two upright plumes to the declared height (the round-3 `build` loop)."""
    def run(rig):
        FIT["y"] = 1.0
        for _ in range(8):
            plant = maker(rig)
            FIT["y"] *= 1 + (TARGET["y"] - L.extent(plant)["y+"]) / 0.6
        return maker(rig)
    return run


def main():
    s = spec()
    M.TARGET, M.P = TARGET, PLANT
    document, report = L.build_asset(s, {k: fitted(v) for k, v in BUILDERS.items()})
    L.write(document, report, t("The tallest grass of the catalog: a stout evergreen tussock with eight dense, upright, silvery plumes high above a fountain of sharp blades; heavy plumes swing slowly and hold snow in winter.", "Das höchste Gras des Katalogs: ein kräftiger, wintergrüner Horst mit acht dichten, aufrechten, silbrigen Wedeln hoch über einer Fontäne scharfer Blätter; die schweren Wedel schwingen langsam und halten im Winter Schnee."))


if __name__ == "__main__":
    main()
