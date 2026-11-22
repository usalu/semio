"""🌻 PW-15, vegetation/sunflower: a group of three flowering common sunflowers (Helianthus annuus) up to 2.5 m on a hilled soil ridge: stout bristly stems, spirally set long-petioled ovate leaves and nodding heads that face east, with a heliotropic clip. Stated by hand in world space (x ahead = east, y up, z left = north, metres) per scale and built from the round-3 miscanthus plant parts (`pw15-lib.Plant`, kit `K`), solids, outlines, strips and drawings.

Reference: Flora of North America 21 (Schilling 2006): annuals 100-300 cm, stems erect, usually hispid; leaf blades lance-ovate to ovate, 10-40 x 5-40 cm, petioles 2-20 cm; involucres 15-40 (-200+) mm diameter; rays 17-30 (-100+), laminae 25-50 mm. Heliotropism: Atamian et al. (2016), Science 353, 587-590.
Usage: repo tools exec python -- pw15-sunflower.py
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

ID = "sunflower"
MASS = 15.0
CROWN = 0.08
TARGET = {"x": 1.5, "z": 1.5, "y": 3.0}
PLANTS = {
    "1": {"base": (0.12, 0.06, 0.16), "lean": -2, "a": 1.25, "b": 1.02, "head": 0.16, "facing": -28, "disc": 0.125, "lateral": 0.04},
    "2": {"base": (-0.16, 0.06, -0.04), "lean": 3, "a": 1.15, "b": 0.95, "head": 0.15, "facing": -24, "disc": 0.11, "lateral": -0.03},
    "3": {"base": (0.07, 0.06, -0.22), "lean": -1, "a": 1.08, "b": 0.9, "head": 0.14, "facing": -32, "disc": 0.1, "lateral": -0.05},
}
LATERAL = {}
for _n, _p in PLANTS.items():
    LATERAL.update({f"stem-{_n}a": _p["lateral"], f"stem-{_n}b": round(0.5 * _p["lateral"], 3), f"head-{_n}": 0.0})

LEAF_PROFILE = [(0.0, 0.62), (0.07, 0.9), (0.2, 1.0), (0.4, 0.92), (0.6, 0.72), (0.78, 0.46), (0.92, 0.2), (1.0, 0.03)]
RAY_PROFILE = [(0.0, 0.45), (0.2, 0.95), (0.55, 1.0), (0.85, 0.7), (1.0, 0.25)]
GOLDEN = 137.5


def leaf_rows(count, low, high, l0, l1, peak):
    """🍃 The leaves of one stem from the lowest to the highest node: blade length rises to `peak` at a third of the stem and falls to the small upper leaves; petioles a third to a half of the blade."""
    rows = []
    for i in range(count):
        s = i / (count - 1)
        h = low + (high - low) * s
        length = l0 + (peak - l0) * min(s / 0.33, 1) if s < 0.33 else peak + (l1 - peak) * (s - 0.33) / 0.67
        rows.append((round(h, 3), round(length, 3), round(0.25 + 0.3 * (1 - s), 3) * length, 62 - 22 * s, 140 - 25 * s))
    return rows


S = {
    50: {"leaves": leaf_rows(16, 0.12, 0.9, 0.24, 0.12, 0.34), "keep": None, "thick": 0.0005, "points": 9, "petiole_r": 0.003, "stem_r": (0.017, 0.012, 0.008), "rays": 24, "ray": (0.048, 0.016), "ray_thick": 0.0005, "mound": [(0.002, 0.32), (0.03, 0.3), (0.06, 0.22), (0.085, 0.1), (0.09, 0.04)], "strip": (0.25, 0.75, 0.32)},
    100: {"leaves": leaf_rows(16, 0.12, 0.9, 0.24, 0.12, 0.34), "keep": None, "thick": 0.0008, "points": 7, "petiole_r": 0.0035, "stem_r": (0.017, 0.012, 0.009), "rays": 14, "ray": (0.048, 0.024), "ray_thick": 0.0009, "mound": [(0.004, 0.32), (0.05, 0.26), (0.085, 0.1), (0.09, 0.04)], "strip": (0.25, 0.75, 0.3)},
    200: {"leaves": leaf_rows(16, 0.12, 0.9, 0.24, 0.12, 0.34), "keep": [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], "thick": 0.0015, "points": 6, "petiole_r": 0.0045, "stem_r": (0.018, 0.013, 0.01), "rays": 0, "mound": [(0.004, 0.32), (0.06, 0.22), (0.09, 0.05)], "strip": (0.1, 0.9, 0.02)},
    500: {"leaves": leaf_rows(16, 0.12, 0.9, 0.24, 0.12, 0.34), "keep": [0, 2, 3, 4, 5, 6, 8, 10], "thick": 0.003, "points": 5, "petiole_r": 0.005, "stem_r": (0.019, 0.015, 0.012), "rays": 0, "mound": [(0.004, 0.32), (0.07, 0.22), (0.095, 0.06)], "strip": (0.1, 0.9, 0.025)},
}
STRIP_LEAVES = (4, 6, 8, 10)

LEAF = {"material": "sunflower-leaf", "role": "body", "tone": 0.5}
STEM = {"material": "sunflower-stem", "role": "body", "tone": 0.4}
SOIL = {"material": "sunflower-soil", "role": "body", "tone": 0.28}
RAY = {"material": "sunflower-ray", "role": "body", "tone": 0.82}
DISC = {"material": "sunflower-disc", "role": "body", "tone": 0.2}
PLANT_NS = SimpleNamespace(BUNDLES=[], CULMS=[], LATERAL=LATERAL, BLADE_PROFILE=LEAF_PROFILE, CROWN_LENGTH=CROWN)


def axis(rig, bone):
    return K.unit(K.sub(K.add(rig.tip(bone), (0.0, 0.0, LATERAL[bone])), rig.origin(bone)))


def frame(direction):
    e1 = K.unit(K.cross(direction, (0.0, 0.0, 1.0))) if abs(direction[2]) < 0.9 else (1.0, 0.0, 0.0)
    return e1, K.unit(K.cross(direction, e1))


def flat(plant, part_id, bone, points, across, rx, thick, region, look):
    """🌼 A flat sheet (ray floret) along `points` whose width lies along `across` (in the plane of the disc), not the horizontal of a leaf."""
    n = len(points)
    return plant.part(id=part_id, bone=bone, region=region, look=look, points=points, widths=[across] * n, rx=rx, ry=[thick / 2] * n, slender=False, sheet=True, width=2 * max(rx))


def head(plant, rig, n, scale):
    p = PLANTS[n]
    bone = f"head-{n}"
    top, ax = rig.origin(bone), axis(rig, bone)
    up = K.unit(K.sub(rig.origin(bone), rig.origin(f"stem-{n}b")))
    back = K.add(K.add(top, K.mul(up, 0.05)), K.mul(ax, 0.05))
    neck = K.bezier(top, K.add(top, K.mul(up, 0.07)), back, 5 if scale <= 100 else 4)
    r = {50: 0.0085, 100: 0.009, 200: 0.01, 500: 0.012}[scale]
    plant.round(f"neck-{n}", bone, neck, [r] * len(neck), "stems", STEM)
    disc = p["disc"]
    e1, _ = frame(ax)
    if scale <= 100:
        offsets = [(0.0, 0.035), (0.015, 0.075), (0.03, 0.75 * disc), (0.042, disc), (0.05, 0.85 * disc), (0.055, 0.5 * disc)]
        face = K.add(back, K.mul(ax, 0.042))
    else:
        rim = disc + S[50]["ray"][0] * 0.9
        depth = 0.06 if scale == 200 else 0.08
        offsets = [(0.0, 0.04), (0.35 * depth, 0.6 * disc), (0.7 * depth, rim), (0.85 * depth, rim * 0.92), (depth, 0.6 * disc)]
        face = K.add(back, K.mul(ax, 0.7 * depth))
    points = [K.add(back, K.mul(ax, d)) for d, _ in offsets]
    plant.part(id=f"head-{n}", bone=bone, region="heads", look=DISC, points=points, widths=[e1] * len(points), rx=[q for _, q in offsets], ry=[q for _, q in offsets], slender=False, width=2 * max(q for _, q in offsets))
    plant.stems[bone] = neck + [K.add(back, K.mul(ax, 0.03))]
    count = S[scale]["rays"]
    length, width = S[scale]["ray"] if count else (0, 0)
    for k in range(count):
        th = 2 * math.pi * (k + 0.5 * (n == "2")) / count
        e1, e2 = frame(ax)
        radial = K.add(K.mul(e1, math.cos(th)), K.mul(e2, math.sin(th)))
        start = K.add(face, K.mul(radial, disc - 0.008))
        d0 = K.unit(K.add(radial, K.mul(ax, 0.12)))
        d1 = K.unit(K.sub(radial, K.mul(ax, 0.18)))
        pts = K.arch(start, d0, d1, length * (0.92 + 0.08 * math.sin(3 * th)), 1.2, 5)
        across = K.unit(K.cross(ax, radial))
        flat(plant, f"ray-{n}-{k + 1}", bone, pts, across, [width / 2 * M.profile(RAY_PROFILE, i / 4) for i in range(5)], S[scale]["ray_thick"], "heads", RAY)


def stem(plant, rig, n, scale):
    table = S[scale]
    a, b = f"stem-{n}a", f"stem-{n}b"
    o, j, e = rig.origin(a), rig.origin(b), rig.origin(f"head-{n}")
    k = 7 if scale <= 100 else 4
    pa, pb = [K.lerp(o, j, i / (k - 1)) for i in range(k)], [K.lerp(j, e, i / (k - 1)) for i in range(k)]
    r0, r1, r2 = table["stem_r"]
    plant.round(a, a, pa, [r0 + (r1 - r0) * i / (k - 1) for i in range(k)], "stems", STEM)
    plant.round(b, b, pb, [r1 + (r2 - r1) * i / (k - 1) for i in range(k)], "stems", STEM)
    plant.stems[a], plant.stems[b] = pa, pb
    la, lb = L.arc_length(pa), L.arc_length(pb)
    leads = []
    for i, (s, length, petiole, incline, end) in enumerate(table["leaves"]):
        if table["keep"] is not None and i not in table["keep"]:
            continue
        az = GOLDEN * i + 40 * int(n)
        run = s * (la + lb)
        bone, at = (a, K.along(pa, run / la)) if run <= la else (b, K.along(pb, (run - la) / lb))
        tip = K.add(at, K.mul(K.direction(48, az), petiole))
        plant.round(f"petiole-{n}-{i + 1}", bone, [at, K.lerp(at, tip, 0.5), tip], [table["petiole_r"]] * 3, "stems", STEM)
        part = plant.leaf(f"leaf-{n}-{i + 1}", bone, tip, K.direction(incline, az), K.direction(end, az), length, 1.4, 0.82 * length, table["thick"], LEAF_PROFILE, table["points"], "leaves", LEAF, sheet=True, margin=0.0, roll=(38 if i % 2 else -38) * (1 - 0.4 * s))
        if i in STRIP_LEAVES:
            leads.append(part)
    return leads


def build(rig, scale):
    plant = L.Plant(rig, scale, {"y": 1.0})
    plant.dome("hill", S[scale]["mound"], SOIL)
    plant.parts[-1]["region"] = "soil"
    leads = []
    for n in PLANTS:
        leads += stem(plant, rig, n, scale)
        head(plant, rig, n, scale)
    for i, part in enumerate(leads):
        plant.strips.append((i + 1, part, S[scale]["strip"]))
    return plant


def bones():
    rows = []
    for n, p in PLANTS.items():
        rows.append({"id": f"stem-{n}a", "parent": "root-plate", "base": p["base"], "angle": p["lean"], "length": p["a"], "limit": 8, "kg": 1.2, "title": t(f"Stem {n}, lower", f"Stängel {n}, unten"), "section": L.circle_section(0.032, "sunflower-stem", "Solid section, d = 3.2 cm", "Vollquerschnitt, d = 3,2 cm")})
        rows.append({"id": f"stem-{n}b", "parent": f"stem-{n}a", "angle": 0, "length": p["b"], "limit": 15, "kg": 0.6, "title": t(f"Stem {n}, upper", f"Stängel {n}, oben"), "section": L.circle_section(0.022, "sunflower-stem", "Solid section, d = 2.2 cm", "Vollquerschnitt, d = 2,2 cm")})
        rows.append({"id": f"head-{n}", "parent": f"stem-{n}b", "angle": p["facing"] - 90 - p["lean"], "length": p["head"], "limit": 90, "kg": 1.2, "com": 0.7, "distribution": "point", "title": t(f"Flower head {n} with its neck", f"Blütenkorb {n} mit Hals"), "section": L.circle_section(0.017, "sunflower-stem", "Solid section, d = 1.7 cm", "Vollquerschnitt, d = 1,7 cm")})
    return rows


BONES = bones()
ROOT_KG = round(MASS - sum(r["kg"] for r in BONES), 4)


def member_radius(bone, scale):
    return {"a": 0.014, "b": 0.01}.get(bone[-1], 0.007) if bone.startswith("stem") else 0.007


def masses(scale):
    r = {50: 0.03, 100: 0.03, 200: 0.035, 500: 0.04}[scale]
    return {f"head-{n}": (1.0, r) for n in PLANTS}


AREAS = {}


def thermal(r50, anchors):
    leaf = round(r50["one_sided_leaf_m2"], 2)
    lai = round(leaf / (math.pi * DIMS["widthM"] / 2 * DIMS["depthM"] / 2), 2)
    return {
        "environment": {"airTempC": 26, "meanRadiantTempC": 30, "relativeHumidity": 0.5, "windMS": 2, "windDirectionDeg": 0, "solarWM2": 800, "pressurePa": 101325, "diffuseFraction": 0.15},
        "regions": [
            {"id": "leaves", "title": t("Leaf blades", "Blattspreiten"), "kind": "foliage", "model": "leaf", "material": "sunflower-leaf", "areaM2": leaf, "areaBasis": "exchange", "characteristicLengthM": 0.2, "geometry": "horizontal-plate-up", "skyViewFactor": 0.6, "layers": [], "heatCapacityJM2K": 900, "leaf": {"stomatalConductanceMaxMS": 0.015, "leafAreaIndex": lai, "photosynthesis": "c3"}, "source": "authored-estimate", "reference": "jones-2014"},
            {"id": "heads", "title": t("Flower heads", "Blütenkörbe"), "kind": "surface", "model": "passive", "material": "sunflower-disc", "areaM2": AREAS.get("heads", round(r50["areas"]["heads"], 2)), "characteristicLengthM": 0.25, "geometry": "vertical-plate", "skyViewFactor": 0.6, "layers": [], "heatCapacityJM2K": 30000, "source": "authored-estimate", "reference": "jones-2014"},
            {"id": "stems", "title": t("Stems, necks and petioles", "Stängel, Hälse und Blattstiele"), "kind": "bark", "model": "passive", "material": "sunflower-stem", "areaM2": AREAS.get("stems", round(r50["areas"]["stems"], 2)), "characteristicLengthM": 0.025, "geometry": "cylinder", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": 25000, "source": "authored-estimate", "reference": "jones-2014"},
            {"id": "soil", "title": t("Hilled soil", "Angehäufelte Erde"), "kind": "surface", "model": "passive", "material": "sunflower-soil", "areaM2": AREAS.get("soil", round(r50["areas"]["soil"], 2)), "characteristicLengthM": 0.6, "geometry": "horizontal-plate-up", "skyViewFactor": 0.3, "layers": [], "heatCapacityJM2K": 60000, "source": "authored-estimate", "reference": "jones-2014"},
        ],
        "emitters": [{"id": "transpiration-leaves", "kind": "transpiration", "title": t("Transpiration of the leaves (distributed over the three plants)", "Transpiration der Blätter (über die drei Pflanzen verteilt)"), "anchor": anchors[0], "anchors": anchors, "directionDeg": 90, "diameterM": 0.4, "region": "leaves", "source": "authored-estimate", "reference": "monteith-unsworth-2013"}],
        "assumptions": [
            t("Leaf regions use the one-sided area of the 1:50 blades; the leaf area index is that area over the ellipse of the plan extent. The maximum stomatal conductance (15 mm/s) is an authored estimate for a well-watered C3 crop with high stomatal density on both leaf faces, guided by Jones (2014). Aperture and night closure are the engine's business, not authored here.", "Blattregionen verwenden die einseitige Fläche der Spreiten von 1:50; der Blattflächenindex ist diese Fläche bezogen auf die Ellipse der Grundrissausdehnung. Die maximale Spaltöffnungsleitfähigkeit (15 mm/s) ist eine Autorenschätzung für eine gut wasserversorgte C3-Kulturpflanze mit dichten Spaltöffnungen auf beiden Blattseiten, angelehnt an Jones (2014). Öffnung und nächtliches Schließen sind Sache der Engine und werden hier nicht vorgegeben."),
            t("Heads, stems and the soil hill are passive. The flower head is a massive, water-rich receptacle (about 20 to 30 mm of tissue at 3.5 MJ per m3 and K), the stems are pith-filled (about 7 mm effective depth); the hilled soil is moist loam over 15 mm. No surface temperature is authored.", "Körbe, Stängel und Erdhügel sind passiv. Der Blütenkorb ist ein massiger, wasserreicher Blütenboden (etwa 20 bis 30 mm Gewebe mit 3,5 MJ je m3 und K), die Stängel sind markgefüllt (etwa 7 mm wirksame Tiefe); die angehäufelte Erde ist feuchter Lehm über 15 mm. Es werden keine Oberflächentemperaturen vorgegeben."),
            t("The transpiration emitter releases the latent flux of the leaves as vapour over twelve anchors on four middle leaves of every plant; the release is diffuse and points up.", "Der Transpirationsemitter gibt den latenten Strom der Blätter als Dampf über zwölf Anker an je vier mittleren Blättern jeder Pflanze ab; die Abgabe ist diffus und zeigt nach oben."),
        ],
        "operating": [], "passengers": [],
    }


V_CALM, V_WINDY, V_STORM = 1.5, 5.5, 15.0
LEAVES_AREA, HEAD_AREA = 0.35, 0.08
F = {k: (round(L.wind_force(LEAVES_AREA, cdl, v), 2), round(L.wind_force(HEAD_AREA, cdh, v), 2)) for k, cdl, cdh, v in [("calm", 0.6, 1.1, V_CALM), ("windy", 0.6, 1.1, V_WINDY), ("storm", 0.35, 1.0, V_STORM)]}


def loads(kind, f_leaves, f_head, source, reference, fy=False, heads_only=False):
    word = {"wind": ("Wind drag", "Winddruck"), "imposed": ("Weight of the ripening seeds", "Gewicht der reifenden Kerne")}[kind]
    out = []
    for n in PLANTS:
        if not heads_only:
            out.append({"id": f"{kind}-stem-{n}b", "kind": kind, "title": t(f"{word[0]} on the upper leaves of plant {n}", f"{word[1]} auf die oberen Blätter von Pflanze {n}"), "bone": f"stem-{n}b", "x": 0.5, "y": 0, "fxN": f_leaves, "fyN": 0, "source": source, "reference": reference})
        out.append({"id": f"{kind}-head-{n}", "kind": kind, "title": t(f"{word[0]} on flower head {n}", f"{word[1]} auf Blütenkorb {n}"), "bone": f"head-{n}", "x": 0.12, "y": 0, "fxN": 0 if fy else f_head, "fyN": -f_head if fy else 0, "source": source, "reference": reference})
    return out


def deltas(a, b, h):
    out = {}
    for n in PLANTS:
        out.update({f"stem-{n}a": a, f"stem-{n}b": b, f"head-{n}": h})
    return out


WINDY = deltas(-1.5, -3, -6)
STORM = deltas(-5, -10, -16)
RIPE = deltas(0, -3, -40)
EVENING = deltas(0, 6, 70)


def binding(leaves, heads, stems, soil, emit, wet=None):
    regions = {"leaves": {"exposedFraction": 1, "sunlitFraction": leaves}, "heads": {"exposedFraction": 1, "sunlitFraction": heads}, "stems": {"exposedFraction": 1, "sunlitFraction": stems}, "soil": {"exposedFraction": 1, "sunlitFraction": soil}}
    if wet is not None:
        for r, w in zip(regions.values(), wet):
            r["wettedness"] = w
    return {"regions": regions, "emitters": {"transpiration-leaves": emit}}


def poses():
    c, w, s = F["calm"], F["windy"], F["storm"]
    de = lambda v: L.fmt(v, "de")
    return [
        L.pose("calm", t("Flowering on a calm summer morning", "Blühend an einem windstillen Sommermorgen"), t(f"Full bloom: the three heads face east (+x) and nod about 30 degrees below the horizontal, as mature sunflowers do once they have stopped tracking the sun; light air of {V_CALM} m/s, drag {c[0]} N on the upper leaves and {c[1]} N on a head.", f"Vollblüte: Die drei Körbe schauen nach Osten (+x) und nicken etwa 30 Grad unter die Waagrechte, wie reife Sonnenblumen, die der Sonne nicht mehr folgen; leiser Zug von {de(V_CALM)} m/s, Widerstand {de(c[0])} N auf die oberen Blätter und {de(c[1])} N auf einen Korb."), {}, binding(0.55, 0.7, 0.2, 0.3, 0.8), loads("wind", *c, "literature", "en-1991-1-4")),
        L.pose("windy", t("Fresh breeze", "Frische Brise"), t(f"Beaufort 5, 8 m/s at 10 m, about {V_WINDY} m/s at head height (crop field, z0 = 0.1 m); cd 0.6 on the upper leaves (0.35 m2 per plant) and 1.1 on a head (0.08 m2): {w[0]} N and {w[1]} N. The stiff stems bend 1.5 and 3 degrees, the heavy heads swing 10 degrees in all.", f"Beaufort 5, 8 m/s in 10 m Höhe, etwa {de(V_WINDY)} m/s in Korbhöhe (Feld, z0 = 0,1 m); cd 0,6 auf die oberen Blätter (0,35 m2 je Pflanze) und 1,1 auf einen Korb (0,08 m2): {de(w[0])} N und {de(w[1])} N. Die steifen Stängel biegen sich 1,5 und 3 Grad, die schweren Körbe schwingen insgesamt 10 Grad."), WINDY, binding(0.55, 0.7, 0.2, 0.3, 1), loads("wind", *w, "literature", "en-1991-1-4")),
        L.pose("storm", t("Summer gale", "Sommersturm"), t(f"Beaufort 9, 22 m/s at 10 m, about {V_STORM} m/s at head height; the leaves fold and flutter (cd 0.35), the heads act as plates (cd 1.0): {s[0]} N on the upper leaves, {s[1]} N per head. The stems lean 15 degrees at the neck and the heads 31 degrees, close to the lodging of the crop.", f"Beaufort 9, 22 m/s in 10 m Höhe, etwa {de(V_STORM)} m/s in Korbhöhe; die Blätter falten sich und flattern (cd 0,35), die Körbe wirken als Platten (cd 1,0): {de(s[0])} N auf die oberen Blätter, {de(s[1])} N je Korb. Die Stängel neigen sich am Hals um 15 Grad, die Körbe um 31 Grad, nahe am Lagern des Bestands."), STORM, binding(0.3, 0.4, 0.1, 0.1, 0.5, (0.4, 0.4, 0.4, 0.6)), loads("wind", *s, "literature", "vogel-1989")),
        L.pose("ripening", t("Ripening seed heads", "Reifende Samenkörbe"), t("Late summer, the rays wilting: about 0.8 kg of filling seeds per head (8 N) bends the neck so that the heads hang face down, 40 degrees below the flowering pose; a common sight before harvest and a protection of the seeds against birds and sun (authored estimate after FNA 21 head sizes).", "Spätsommer, die Zungenblüten welken: etwa 0,8 kg reifende Kerne je Korb (8 N) biegen den Hals, sodass die Körbe mit dem Gesicht nach unten hängen, 40 Grad unter der Blühpose; ein gewohntes Bild vor der Ernte und ein Schutz der Kerne vor Vögeln und Sonne (Schätzung nach den Korbgrößen von FNA 21)."), RIPE, binding(0.45, 0.2, 0.25, 0.3, 0.5), loads("imposed", 0, 8, "authored-estimate", "fna-21-helianthus", fy=True, heads_only=True)),
        L.pose("evening-sun", t("Heads turned to the evening sky", "Körbe zum Abendhimmel gedreht"), t("The westernmost reach of the heliotropic swing at the onset of flowering (Atamian et al. 2016): the upper stem has bent 6 degrees and the heads have turned 70 degrees from their morning position towards the western sky; at night they swing back east. Calm air.", "Die westlichste Stellung der heliotropen Bewegung zu Blühbeginn (Atamian et al. 2016): Der obere Stängel hat sich 6 Grad gebogen und die Körbe haben sich 70 Grad aus ihrer Morgenstellung zum Westhimmel gedreht; nachts schwingen sie nach Osten zurück. Windstille."), EVENING, binding(0.4, 0.8, 0.2, 0.2, 0.4), loads("wind", *c, "literature", "en-1991-1-4")),
    ]


def clips():
    amplitude = {**{f"stem-{n}a": 0.6 for n in PLANTS}, **{f"stem-{n}b": 1.5 for n in PLANTS}, **{f"head-{n}": 3.0 for n in PLANTS}}
    phase = {b: -2 * math.pi * PLANTS[b.split("-")[1][0]]["base"][0] / 1.0 + (0.5 if b.startswith("head") else 0.25 if b.endswith("b") else 0) for b in amplitude}
    spec = {"sway_period": 2.0, "emitter": "transpiration-leaves",
            "sway_text": t("A deterministic 2 s cycle (a heavy head on a stiff stem, estimated 0.5 Hz) about the fresh-breeze lean: the stems sway by 1.5 degrees at the top, the heads nod by 3 degrees, plant by plant with a phase that travels downwind.", "Ein deterministischer Zyklus von 2 s (ein schwerer Korb auf einem steifen Stängel, geschätzt 0,5 Hz) um die Neigung der frischen Brise: Die Stängel schwingen oben um 1,5 Grad, die Körbe nicken um 3 Grad, Pflanze um Pflanze mit einer Phase, die windabwärts wandert."),
            "gust_text": t("A single 8 s gust: the wind-load tracks rise from 5 % to 2.75 times the fresh-breeze load at 2.5 s (EN 1991-1-4 peak pressure with Iv 0.25) and decay; stems and heads lean up to 70 % of the gale lean and spring back.", "Eine einzelne Böe über 8 s: Die Windlast-Spuren steigen bei 2,5 s von 5 % auf das 2,75-fache der Last der frischen Brise (Spitzendruck nach EN 1991-1-4 mit Iv 0,25) und klingen ab; Stängel und Körbe neigen sich bis zu 70 % der Sturmneigung und federn zurück."),
            "diurnal_text": t("A 24 h cycle compressed to 24 s (1 s = 1 h, 00:00 to 24:00), calm weather. The sunlit fraction of every region and the transpiration strength follow the sun, stomata nearly closed at night; leaf, head and stem temperatures are not authored but follow from the thermal solver. The heads stay facing east (see the heliotropic clip for the movement).", "Ein 24-h-Zyklus auf 24 s gerafft (1 s = 1 h, 00:00 bis 24:00) bei ruhigem Wetter. Der besonnte Anteil jeder Region und die Transpirationsstärke folgen der Sonne, nachts sind die Stomata fast geschlossen; Blatt-, Korb- und Stängeltemperaturen werden nicht vorgegeben, sondern folgen aus dem thermischen Löser. Die Körbe schauen weiter nach Osten (die Bewegung zeigt der heliotrope Clip).")}
    wind_ids = [l["id"] for l in loads("wind", 1, 1, "literature", "en-1991-1-4")]
    helio_keys = []
    for tk, share in [(0, 0.3), (3, 0.0), (6, 0.0), (9, 0.3), (12, 0.55), (15, 0.8), (18, 1.0), (21, 0.75), (24, 0.3)]:
        helio_keys.append({"t": tk, "pose": "calm", "ease": "smooth", "bones": {b: {"angleDeg": round(a * share, 2)} for b, a in EVENING.items() if a}})
    sun = [(0, 0), (6, 0), (8, 0.5), (12, 1.0), (16, 0.5), (18, 0), (24, 0)]
    helio = {"id": "heliotropic-tracking", "title": t("Heliotropic tracking", "Heliotrope Nachführung"), "description": t("A day compressed to 24 s (1 s = 1 h): the heads face east at dawn, turn with the sun through the day to 70 degrees towards the western sky at sunset (the upper stem bending 6 degrees) and swing back east during the night, ready for sunrise. This is the late, reduced form of the tracking that young sunflowers perform; it is driven by growth on the shaded side of the stem and stops as the heads mature and settle facing east (Atamian et al. 2016). The swing angle is an authored estimate.", "Ein Tag auf 24 s gerafft (1 s = 1 h): Die Körbe schauen bei Tagesanbruch nach Osten, drehen sich tagsüber mit der Sonne bis 70 Grad zum Westhimmel bei Sonnenuntergang (der obere Stängel biegt sich 6 Grad) und schwingen nachts nach Osten zurück, bereit für den Sonnenaufgang. Das ist die späte, verkleinerte Form der Nachführung junger Sonnenblumen; sie wird durch Wachstum auf der beschatteten Stängelseite angetrieben und endet, wenn die Körbe reifen und nach Osten gerichtet bleiben (Atamian et al. 2016). Der Schwenkwinkel ist eine Autorenschätzung."), "durationS": 24, "fps": 10, "loop": True, "rootMotion": "none", "keys": helio_keys, "contacts": [],
             "phases": [{"id": "return", "fromS": 0, "toS": 6, "title": t("Night return to the east", "Nächtliche Rückkehr nach Osten")}, {"id": "tracking", "fromS": 6, "toS": 18, "title": t("Tracking the sun", "Der Sonne folgen")}, {"id": "evening", "fromS": 18, "toS": 24, "title": t("Evening and night", "Abend und Nacht")}],
             "events": [{"t": 6, "kind": "motion", "title": t("Sunrise, heads facing east", "Sonnenaufgang, Körbe nach Osten")}, {"t": 18, "kind": "motion", "title": t("Sunset, heads turned furthest west", "Sonnenuntergang, Körbe am weitesten nach Westen")}],
             "tracks": [{"target": "region:heads:sunlitFraction", "ease": "smooth", "keys": [{"t": a, "v": round(0.85 * v, 3)} for a, v in sun]}, {"target": "emitter:transpiration-leaves", "ease": "smooth", "keys": [{"t": a, "v": max(0.03, v)} for a, v in sun]}]}
    return [L.sway_clip(spec, WINDY, amplitude, phase), L.gust_clip(spec, STORM, wind_ids), L.diurnal_clip(spec, {"leaves": 0.55, "heads": 0.7, "stems": 0.2, "soil": 0.3}), helio]


D = t
REP = {
    50: {"silhouette": D("At 1:50 (1 m is 20 mm) the group is drawn leaf by leaf as the true view of its 3D body: three stout stems on a soil ridge, sixteen long-petioled heart-shaped leaves on each in a spiral, and heads whose ray florets radiate around a dark disc, facing east and nodding. All three views are projections of the same body; in the side elevation the heads are seen face-on, in the front elevation edge-on.", "Bei 1:50 (1 m sind 20 mm) ist die Gruppe Blatt für Blatt als wahre Ansicht ihres 3D-Körpers gezeichnet: drei kräftige Stängel auf einem Erddamm, an jedem sechzehn langgestielte herzförmige Blätter in einer Spirale und Körbe, deren Zungenblüten um eine dunkle Scheibe strahlen, nach Osten gerichtet und nickend. Alle drei Ansichten sind Projektionen desselben Körpers; in der Seitenansicht sieht man die Körbe von vorn, in der Vorderansicht von der Kante."),
         "retained": D("Retained: every leaf and petiole, the stems in two segments, the curved necks, the receptacle with its domed disc and 24 ray florets per head, the soil hill.", "Beibehalten: jedes Blatt und jeder Blattstiel, die Stängel in zwei Abschnitten, die gebogenen Hälse, der Blütenboden mit gewölbter Scheibe und 24 Zungenblüten je Korb, der Erdhügel."),
         "merged": D("Merged: the phyllaries into the receptacle outline, the disc florets into the disc.", "Zusammengefasst: die Hüllblätter in den Umriss des Blütenbodens, die Röhrenblüten in die Scheibe."),
         "omitted": D("Omitted: stem bristles, leaf veins and serration, the spiral pattern of the disc florets.", "Weggelassen: Stängelborsten, Blattadern und Blattzähne, das Spiralmuster der Röhrenblüten."),
         "thinElements": D("Stems are strokes of their true width (0.68 to 0.32 mm), petioles 0.12 mm; leaves and rays are filled outlines, and a leaf or ray seen edge-on becomes a stroke of its projected width.", "Stängel sind Striche ihrer wahren Breite (0,68 bis 0,32 mm), Blattstiele 0,12 mm; Blätter und Zungenblüten sind gefüllte Umrisse, ein Blatt oder eine Zungenblüte in Kantenansicht wird ein Strich seiner projizierten Breite."),
         "hatching": D("No hatching.", "Keine Schraffur."),
         "styles": D("Line views: outlines of leaves, rays and discs over the stems. Solid views: leaves mid tone, rays light, discs dark, the classic sunflower contrast.", "Linienansichten: Umrisse von Blättern, Zungenblüten und Scheiben über den Stängeln. Flächenansichten: Blätter im Mittelton, Zungenblüten hell, Scheiben dunkel, der klassische Sonnenblumenkontrast."),
         "overlays": D("Every shape is bound to its thermal region (leaves, heads, stems, soil) and to the bone that carries it.", "Jede Form ist an ihre thermische Region (Blätter, Körbe, Stängel, Erde) und an den tragenden Knochen gebunden.")},
    100: {"silhouette": D("At 1:100 (1 m is 10 mm) every leaf is kept with a coarser outline; each head keeps fourteen broader rays (two florets merged into one) around the disc. All three views are projections of the 1:100 body.", "Bei 1:100 (1 m sind 10 mm) bleibt jedes Blatt mit gröberem Umriss erhalten; jeder Korb behält vierzehn breitere Zungenblüten (zwei Blüten in einer) um die Scheibe. Alle drei Ansichten sind Projektionen des Körpers von 1:100."),
          "retained": D("Retained: all leaves and petioles, stems, necks, discs, fourteen rays per head, the soil hill.", "Beibehalten: alle Blätter und Blattstiele, Stängel, Hälse, Scheiben, vierzehn Zungenblüten je Korb, der Erdhügel."),
          "merged": D("Merged: pairs of ray florets.", "Zusammengefasst: Paare von Zungenblüten."), "omitted": D("Omitted: the dome of the disc is flattened.", "Weggelassen: die Wölbung der Scheibe ist abgeflacht."),
          "thinElements": D("Stems 0.34 to 0.18 mm, petioles at the 0.1 mm minimum.", "Stängel 0,34 bis 0,18 mm, Blattstiele am Minimum von 0,1 mm."), "hatching": D("No hatching.", "Keine Schraffur."),
          "styles": D("Line views: outlines. Solid views: tones of the regions.", "Linienansichten: Umrisse. Flächenansichten: Töne der Regionen."), "overlays": D("Every shape is bound to its thermal region and bone.", "Jede Form ist an ihre thermische Region und ihren Knochen gebunden.")},
    200: {"silhouette": D("At 1:200 (1 m is 5 mm) the group keeps its twelve lower leaves per stem; each head is one disc whose rim is the ray corona. All three views are projections of the 1:200 body.", "Bei 1:200 (1 m sind 5 mm) behält die Gruppe ihre zwölf unteren Blätter je Stängel; jeder Korb ist eine Scheibe, deren Rand der Strahlenkranz ist. Alle drei Ansichten sind Projektionen des Körpers von 1:200."),
          "retained": D("Retained: twelve leaves per stem, stems, necks, three heads, the soil hill.", "Beibehalten: zwölf Blätter je Stängel, Stängel, Hälse, drei Körbe, der Erdhügel."),
          "merged": D("Merged: the ray florets into the rim of the head disc.", "Zusammengefasst: die Zungenblüten in den Rand der Korbscheibe."), "omitted": D("Omitted: the four small upper leaves of every stem.", "Weggelassen: die vier kleinen oberen Blätter jedes Stängels."),
          "thinElements": D("Stems 0.18 to 0.1 mm, petioles 0.1 mm.", "Stängel 0,18 bis 0,1 mm, Blattstiele 0,1 mm."), "hatching": D("No hatching.", "Keine Schraffur."),
          "styles": D("Line views: outlines. Solid views: tonal masses.", "Linienansichten: Umrisse. Flächenansichten: getönte Massen."), "overlays": D("Every shape is bound to its thermal region and bone.", "Jede Form ist an ihre thermische Region und ihren Knochen gebunden.")},
    500: {"silhouette": D("At 1:500 (1 m is 2 mm) three stem lines with eight large leaves each and a round head on top: the unmistakable sunflower sign even at site-plan scale. All three views are projections of the 1:500 body.", "Bei 1:500 (1 m sind 2 mm) drei Stängellinien mit je acht großen Blättern und einem runden Korb obenauf: das unverwechselbare Sonnenblumenzeichen auch im Lageplanmaßstab. Alle drei Ansichten sind Projektionen des Körpers von 1:500."),
          "retained": D("Retained: eight leaves per stem, stems, three heads of 8 cm depth, the soil hill of 95 mm.", "Beibehalten: acht Blätter je Stängel, Stängel, drei Körbe von 8 cm Tiefe, der Erdhügel von 95 mm."),
          "merged": D("Merged: rays into the head disc, the head deepened to 80 mm, the printable feature.", "Zusammengefasst: Zungenblüten in die Korbscheibe, der Korb auf 80 mm, die druckbare Einzelheit, vertieft."), "omitted": D("Omitted: half of the leaves (the smaller and the hidden ones).", "Weggelassen: die Hälfte der Blätter (die kleineren und die verdeckten)."),
          "thinElements": D("Stems and petioles at 0.1 mm.", "Stängel und Blattstiele mit 0,1 mm."), "hatching": D("No hatching.", "Keine Schraffur."),
          "styles": D("Line views: restrained outlines. Solid views: tonal masses by depth.", "Linienansichten: zurückhaltende Umrisse. Flächenansichten: getönte Massen nach Tiefe."), "overlays": D("Every shape is bound to its thermal region and bone.", "Jede Form ist an ihre thermische Region und ihren Knochen gebunden.")},
}
VOL = {
    50: [D("1:50 body: soil hill, three stems of two segments, sixteen leaves per stem at 137.5 degrees on petioles of a quarter to half their length, curved necks, receptacles with domed disc and 24 ray florets each.", "1:50-Körper: Erdhügel, drei Stängel aus zwei Abschnitten, sechzehn Blätter je Stängel im Winkel von 137,5 Grad an Stielen von einem Viertel bis zur Hälfte ihrer Länge, gebogene Hälse, Blütenböden mit gewölbter Scheibe und je 24 Zungenblüten."),
         D("Leaves (0.5 mm) and rays are declared sheets, stems, necks and petioles slender rods. Thermal by reference plus twelve transpiration strips; structure: root disc, stem rods of 28 and 20 mm, neck rods, joints, support disc, head masses.", "Blätter (0,5 mm) und Zungenblüten sind deklarierte Flächen, Stängel, Hälse und Blattstiele schlanke Stäbe. Thermik per Verweis plus zwölf Transpirationsstreifen; Tragwerk: Wurzelscheibe, Stängelstäbe von 28 und 20 mm, Halsstäbe, Gelenke, Auflagerscheibe, Korbmassen.")],
    100: [D("1:100 body: all leaves with seven stations, heads with fourteen rays.", "1:100-Körper: alle Blätter mit sieben Stationen, Körbe mit vierzehn Zungenblüten."), D("Thermal by reference plus twelve strips; structure with simplified rods.", "Thermik per Verweis plus zwölf Streifen; Tragwerk mit vereinfachten Stäben.")],
    200: [D("1:200 body: twelve leaves per stem, heads as one disc of 6 cm depth with the ray corona as rim.", "1:200-Körper: zwölf Blätter je Stängel, Körbe als eine Scheibe von 6 cm Tiefe mit dem Strahlenkranz als Rand."), D("Thermal by reference plus twelve strips; structure without joints.", "Thermik per Verweis plus zwölf Streifen; Tragwerk ohne Gelenke.")],
    500: [D("1:500 body: eight leaves per stem, heads 8 cm deep, soil hill 95 mm.", "1:500-Körper: acht Blätter je Stängel, Körbe 8 cm tief, Erdhügel 95 mm."), D("Thermal by reference plus twelve strips; structure with three-point rods.", "Thermik per Verweis plus zwölf Streifen; Tragwerk mit Dreipunktstäben.")],
}
REFERENCES = [
    {"id": "en-1991-1-4", "citation": "EN 1991-1-4:2005 Eurocode 1, Actions on structures, Part 1-4: Wind actions. Air density 1.25 kg/m3, F = 0.5 rho cd A v^2, peak velocity pressure (1 + 7 Iv) q_m."},
    {"id": "vogel-1989", "citation": "Vogel, S. (1989). Drag and reconfiguration of broad leaves in high winds. Journal of Experimental Botany 40(8), 941-948."},
    {"id": "jones-2014", "citation": "Jones, H. G. (2014). Plants and Microclimate, 3rd ed., Cambridge University Press. Leaf energy balance, leaf area index, boundary-layer conductance."},
    {"id": "monteith-unsworth-2013", "citation": "Monteith, J. L. and Unsworth, M. H. (2013). Principles of Environmental Physics, 4th ed., Academic Press. Penman-Monteith leaf transpiration."},
    {"id": "fna-21-helianthus", "citation": "Schilling, E. E. (2006). Helianthus annuus. In: Flora of North America Editorial Committee (eds.), Flora of North America 21, Magnoliophyta: Asteridae (in part): Asteraceae, part 3. Oxford University Press. Annuals 100-300 cm, stems erect, usually hispid; blades lance-ovate to ovate, 10-40 x 5-40 cm; involucres 15-40 (-200+) mm diameter; rays 17-30 (-100+), laminae 25-50 mm. https://floranorthamerica.org/Helianthus_annuus"},
    {"id": "atamian-2016", "citation": "Atamian, H. S., Creux, N. M., Brown, E. A., Garner, A. G., Blackman, B. K. and Harmer, S. L. (2016). Circadian regulation of sunflower heliotropism, floral orientation, and pollinator visits. Science 353(6299), 587-590. Young plants track the sun from east to west by day and reorient east at night; at maturity the heads stop and face east."},
]
DIMS = {}


def spec(dims):
    return {
        "id": ID, "target": TARGET, "plant_module": PLANT_NS, "crown_length": CROWN, "root_mass": ROOT_KG, "mass_reference": "fna-21-helianthus",
        "root_title": t("Root zone and hilled soil", "Wurzelraum und angehäufelte Erde"),
        "root_section": L.circle_section(0.5, "sunflower-soil", "Solid section, d = 50 cm", "Vollquerschnitt, d = 50 cm"),
        "bones": BONES, "lateral": LATERAL, "rename": {},
        "leaf_material": "sunflower-leaf", "leaf_region": "leaves", "emitter": "transpiration-leaves", "emitter_title": t("Transpiration of the leaves", "Transpiration der Blätter"),
        "stem_material": "sunflower-stem", "mat_material": "sunflower-soil", "top_material": "sunflower-disc",
        "mat": {50: [(0.002, 0.27), (0.04, 0.22), (0.07, 0.12)], 100: [(0.004, 0.27), (0.07, 0.12)], 200: [(0.004, 0.27), (0.07, 0.12)], 500: [(0.004, 0.27), (0.08, 0.12)]},
        "member_radius": member_radius, "joint_radius": {50: 0.02, 100: 0.02}, "support_radius": 0.27,
        "crown_mass_radii": {50: [0.03, 0.15, 0.15], 100: [0.03, 0.15, 0.15], 200: [0.035, 0.15, 0.15], 500: [0.04, 0.15, 0.15]},
        "masses": masses, "footprint": 0.6,
        "support_title": t("Root anchorage in the soil", "Wurzelverankerung im Boden"),
        "title": t("Sunflower", "Sonnenblume"),
        "summary": t(f"A group of three flowering common sunflowers (Helianthus annuus) {L.fmt(dims['heightM'])} m tall on a hilled soil ridge: stout bristly stems, sixteen long-petioled, heart-shaped leaves per plant set in a spiral, and nodding flower heads up to 34 cm across with golden ray florets around a dark disc, facing east.", f"Eine Gruppe aus drei blühenden Sonnenblumen (Helianthus annuus), {L.fmt(dims['heightM'], 'de')} m hoch auf einem angehäufelten Erddamm: kräftige, borstige Stängel, je Pflanze sechzehn langgestielte, herzförmige Blätter in einer Spirale und nickende Blütenkörbe bis 34 cm Durchmesser mit goldenen Zungenblüten um eine dunkle Scheibe, nach Osten gerichtet."),
        "tags": ["annual", "crop", "flower", "sunflower", "heliotropism", "wind", "transpiration"],
        "provenance": {"author": "Semio staffage vegetation content worker PW-15", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES, "notes": t("Built from the plant parts of the round-3 miscanthus clump (paths, whole parts on the bone of their node, outlines and strokes of true width). Stems 32 mm at the base, 2.1 to 2.4 m to the neck (FNA 21: 100-300 cm); blades 12 to 34 cm long and 0.82 times as wide on petioles of 4 to 19 cm (10-40 x 5-40 cm, petioles 2-20 cm); discs 20 to 25 cm, 24 rays of 4.8 cm (rays 17-30, 25-50 mm). The head bone points along the disc axis; the heads face east (+x) as mature heads do (Atamian et al. 2016), the clip `heliotropic-tracking` shows the late, reduced daily swing. Leaves and rays are declared sheets; seen edge-on they are drawn as strokes.", "Aus den Pflanzenteilen des Chinaschilf-Horstes aus Runde 3 gebaut (Bahnen, ganze Teile am Knochen ihres Knotens, Umrisse und Striche wahrer Breite). Stängel am Fuß 32 mm, 2,1 bis 2,4 m bis zum Hals (FNA 21: 100-300 cm); Spreiten 12 bis 34 cm lang und 0,82-mal so breit an Stielen von 4 bis 19 cm (10-40 x 5-40 cm, Stiele 2-20 cm); Scheiben 20 bis 25 cm, 24 Zungenblüten von 4,8 cm (17-30, 25-50 mm). Der Korbknochen zeigt entlang der Scheibenachse; die Körbe schauen wie reife Körbe nach Osten (+x) (Atamian et al. 2016), der Clip `heliotropic-tracking` zeigt die späte, verkleinerte Tagesbewegung. Blätter und Zungenblüten sind deklarierte Flächen; in Kantenansicht werden sie als Striche gezeichnet.")},
        "dimensions": dims,
        "materials": [
            {"id": "sunflower-leaf", "title": t("Sunflower leaves", "Sonnenblumenblätter"), "emissivity": 0.97, "solarAbsorptivity": 0.5, "source": "literature", "reference": "jones-2014"},
            {"id": "sunflower-stem", "title": t("Green sunflower stem (pith-filled, bristly)", "Grüner Sonnenblumenstängel (markgefüllt, borstig)"), "emissivity": 0.95, "solarAbsorptivity": 0.55, "densityKgM3": 450, "conductivityWmK": 0.2, "elasticModulusPa": 1500000000, "strengthPa": 20000000, "source": "authored-estimate"},
            {"id": "sunflower-ray", "title": t("Golden ray florets", "Goldgelbe Zungenblüten"), "emissivity": 0.95, "solarAbsorptivity": 0.4, "source": "authored-estimate", "reference": "jones-2014"},
            {"id": "sunflower-disc", "title": t("Dark disc and receptacle", "Dunkle Scheibe und Blütenboden"), "emissivity": 0.96, "solarAbsorptivity": 0.85, "densityKgM3": 700, "conductivityWmK": 0.4, "source": "authored-estimate", "reference": "jones-2014"},
            {"id": "sunflower-soil", "title": t("Moist hilled loam", "Feuchter angehäufelter Lehm"), "emissivity": 0.95, "solarAbsorptivity": 0.8, "densityKgM3": 1500, "conductivityWmK": 1.0, "source": "authored-estimate", "reference": "jones-2014"},
        ],
        "poses": poses(), "clips": clips(), "thermal": thermal,
        "structure": {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": ["base-support"], "assumptions": [t("One rigid body per evaluated pose, fixed at the root zone; wind drag acts on the upper leaves of every stem and on every head, the seed weight of the ripening pose on the heads, gravity on all bone masses. Stem sections are solid circles with the stiffness of green sunflower stem (authored estimate). Root lodging and stem buckling are not modelled.", "Ein starrer Körper je ausgewerteter Pose, im Wurzelraum eingespannt; Winddruck wirkt auf die oberen Blätter jedes Stängels und auf jeden Korb, das Kerngewicht der Reifepose auf die Körbe, Schwerkraft auf alle Knochenmassen. Stängelquerschnitte sind Vollkreise mit der Steifigkeit von frischem Sonnenblumenstängel (Schätzung). Wurzellager und Knicken des Stängels werden nicht modelliert.")], "clinical": False},
        "volume_decisions": VOL, "representation_decisions": REP,
        "symbol_lobes": (9, 0.18), "symbol_stems": [(p["base"][0], p["base"][2], 0.016) for p in PLANTS.values()],
    }


def main():
    M.TARGET, M.P = TARGET, PLANT_NS
    rig = K.Rig(L.rig_bones(spec({"heightM": 0})))
    e = M.extent(build(rig, 50))
    DIMS.update({"heightM": round(e["y+"], 2), "widthM": round(e["x+"] - e["x-"], 2), "depthM": round(e["z+"] - e["z-"], 2), "massKg": MASS})
    document, report = L.build_asset(spec(DIMS), {s: (lambda rig, s=s: build(rig, s)) for s in L.SCALES})
    print(f"[DEBUG] dimensions {DIMS} 1:50 extent {e}")
    L.write(document, report, t("The only annual crop flower among the grasses and perennials: three stout stems with spiral heart-shaped leaves and large east-facing heads that follow the sun by day and droop with ripening seed.", "Die einzige einjährige Kulturblume unter den Gräsern und Stauden: drei kräftige Stängel mit spiralig gestellten herzförmigen Blättern und großen, nach Osten gerichteten Körben, die tagsüber der Sonne folgen und mit reifenden Kernen herabhängen."))


if __name__ == "__main__":
    main()
