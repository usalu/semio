"""🌲️ Atlas cedar (Cedrus atlantica): every number of the asset `atlas-cedar`, chosen by hand after a mature park tree of about 25 m with a
massive stem that divides its strength into long ascending limbs, each carrying a wide, flat, horizontal needle plate, so that the crown is a
stack of tiered plates with sky between them under a stiff, upright leader. A plate is a flat solid of revolution about its own vertical
plate bone (with tufts on its upper face at 1:50 and 1:100), a limb is a tapered capsule on a sagittal limb bone that runs out of the rig plane
to its plate. Run: `repo tools exec python -- atlas_cedar.py`.
@see ./conifer_lib.py
@see ./whorled.py — the shared clip, pose and symbol conventions
"""
import math
import os

from conifer_lib import (CATALOG, G, RHO, T, TICKET, Capsule, Marks, Revolve, Rig, SCALES, bbox_union, cantilever_slopes, dump, fit_ring, gap_report,
                         marks_shape, outline_polyline, r3, r4, representation, section, shape_of, simplify, slice_intervals, slope_at, union_width, v_at,
                         wind_force)
from whorled import PENSUM, fmt

id = "atlas-cedar"
H = 25.0
stem = [0.0, 0.8, 6.5, 12.5, 18.0, H]
names = ["trunk-1", "trunk-2", "trunk-3", "leader"]
stem_titles = [T("Stem, lower (bare, plated bark)", "Stamm unten (kahl, Plattenborke)"), T("Stem, main fork zone", "Stamm, Zone der Starkäste"), T("Stem, upper crown", "Stamm, obere Krone"), T("Leader", "Gipfeltrieb")]
stem_limits = [2, 3, 3.5, 6]
crown_base = 4.8
wood = "cedar-wood"
green_density = 820.0
E_green = 8.0e9
k_root = 45e6
needle_area_sun, needle_area_shade = 190.0, 140.0
needle_kg_m2 = 0.35
gs_sun, gs_shade = 0.0018, 0.0012
cd = 0.4
v_calm, v_windy, v_storm = 2.0, 8.0, 22.0
sway_period = 3.4
# limb: (id, height on stem, plate centre x, plate centre y, plate centre z, plate radius, thickness, lateral scale, limb base radius)
LIMBS = [
    ("w1", 5.2, -5.6, 7.6, 1.2, 2.9, 1.0, 0.85, 0.22),
    ("e1", 5.8, 5.2, 8.3, -1.6, 2.7, 0.95, 1.1, 0.21),
    ("s1", 7.2, 0.8, 9.6, -4.6, 2.5, 0.9, 1.2, 0.18),
    ("n1", 8.0, -1.4, 10.6, 4.4, 2.6, 0.9, 1.15, 0.18),
    ("w2", 10.4, -4.4, 13.0, -1.4, 2.4, 0.85, 0.9, 0.16),
    ("e2", 11.2, 4.6, 14.2, 1.6, 2.5, 0.85, 1.05, 0.16),
    ("s2", 13.6, 1.4, 16.3, -3.2, 2.0, 0.75, 1.15, 0.13),
    ("n2", 14.6, -2.2, 17.6, 2.6, 2.1, 0.75, 1.1, 0.13),
    ("w3", 16.6, -2.9, 19.7, 0.6, 1.8, 0.7, 0.95, 0.11),
    ("e3", 17.4, 2.6, 21.0, -0.9, 1.7, 0.7, 1.05, 0.1),
    ("top", 21.2, 0.3, 23.0, 0.2, 1.3, 0.65, 1.0, 0.08),
]
SUN = {"w2", "e2", "s2", "n2", "w3", "e3", "top"}
TUFTS = {  # tufts on the upper face: (angle deg, fraction of radius, tuft radius fraction, height fraction)
    "w1": [(200, 0.55, 0.38, 0.5), (60, 0.5, 0.34, 0.45)], "e1": [(20, 0.5, 0.4, 0.5), (230, 0.45, 0.32, 0.42)], "s1": [(300, 0.5, 0.36, 0.48), (120, 0.4, 0.3, 0.4)],
    "n1": [(80, 0.52, 0.36, 0.5), (260, 0.42, 0.3, 0.4)], "w2": [(180, 0.5, 0.36, 0.5), (40, 0.42, 0.3, 0.42)], "e2": [(350, 0.5, 0.37, 0.5), (150, 0.4, 0.3, 0.4)],
    "s2": [(280, 0.45, 0.38, 0.5)], "n2": [(100, 0.45, 0.38, 0.5)], "w3": [(190, 0.4, 0.4, 0.5)], "e3": [(10, 0.4, 0.4, 0.5)], "top": [(90, 0.3, 0.45, 0.55)],
}


def r_stem(y):
    if y >= H:
        return 0.0
    main = 0.42 * ((H - y) / (H - 1.3)) ** 0.75 + 0.42 * math.exp(-y / 0.35)
    if y > 20.0:
        main = max(main, 0.03 * min(1.0, (H - y) / 1.0))
    return main


def plate_profile(cy, R, th, n, dome=0.6, under=0.4):
    """A flat, domed needle plate: upper face rising to the centre, slightly hanging rim, underside nearly flat."""
    top = [(cy + under * th * 0.2 + (1 - under * 0.2) * th * (1 - f ** 2) ** dome * 0.6 + 0.0, f * R) for f in [i / (n - 1) for i in range(n)]]
    rim = (cy - 0.25 * th, 0.97 * R)
    bottom = [(cy - under * th * (1 - (1 - f) ** 3) - 0.08, f * R * 0.95) for f in [1 - (i + 1) / (n - 1) for i in range(n - 2)]]
    return top + [rim] + bottom + [(cy - 0.08 - under * th * 0, 0.0)]


def build():
    rig = Rig()
    rig.add("root-plate", None, (0, 0, 0), 90, stem[1], T("Root plate and root flare", "Wurzelteller und Wurzelanlauf"), section=section(2 * r_stem(stem[1]), wood, "Root collar", "Wurzelhals"))
    for i in range(1, len(stem) - 1):
        parent = "root-plate" if i == 1 else names[i - 2]
        rig.add(names[i - 1], parent, (0, stem[i], 0), 90, stem[i + 1] - stem[i], stem_titles[i - 1], limit=stem_limits[i - 1], section=section(2 * r_stem(stem[i]), wood, stem_titles[i - 1]["en"], stem_titles[i - 1]["de"]))
    bones = rig.bones

    def stem_bone(y):
        for i in range(1, len(stem) - 1):
            if stem[i] <= y < stem[i + 1]:
                return names[i - 1]
        return names[-1]

    class L:
        pass
    limbs = []
    side_word = {"w": ("west", "West"), "e": ("east", "Ost"), "s": ("south", "Süd"), "n": ("north", "Nord"), "t": ("top", "Wipfel")}
    for (lid, h0, px, py, pz, R, th, ls, rb) in LIMBS:
        l = L()
        l.id, l.h0, l.px, l.py, l.pz, l.R, l.th, l.ls, l.rb = lid, h0, px, py, pz, R, th, ls, rb
        l.region = "needles-sun" if lid in SUN else "needles-shade"
        l.material = "cedar-needles-sun" if lid in SUN else "cedar-needles-shade"
        theta = math.degrees(math.atan2(py - th * 0.3 - h0, px))
        Lp = math.hypot(px, py - th * 0.3 - h0)
        sw = side_word[lid[0]]
        l.limb = rig.add("limb-%s" % lid, stem_bone(h0), (0, h0, 0), theta, Lp, T("Limb %s to plate %s" % (sw[0], lid), "Starkast %s zur Platte %s" % (sw[1], lid)), limit=6,
                         section=section(2 * rb, wood, "Limb", "Starkast"))
        tip = l.limb.tip()
        l.plate = rig.add("plate-%s" % lid, l.limb.id, (tip[0], tip[1], pz), 90, 0.5, T("Needle plate %s" % lid, "Nadelplatte %s" % lid), limit=10)
        l.drop = l.plate.o[1]
        limbs.append(l)

    def stem_profile(y0, y1, n, factor=1.0):
        ys = [y0 + (y1 - y0) * i / (n - 1) for i in range(n)]
        pts = [(y, factor * r_stem(y)) for y in ys]
        return [(y0, 0.0)] + [p for p in pts if p[1] > 1e-6] + [(y1, 0.0)]

    def stem_solids(scale):
        out = []
        if scale == "500":
            out.append(Revolve("stem", bones["trunk-1"], 0, 0, stem_profile(0.0, stem[2] + 0.04, 5), "cedar-bark", 0.3, region="bark-lower"))
            out.append(Revolve("stem-upper", bones["trunk-2"], 0, 0, stem_profile(stem[2], H, 6), "cedar-bark-upper", 0.34, region="bark-upper"))
            return out
        if scale != "200":
            out.append(Revolve("root-flare", bones["root-plate"], 0, 0, stem_profile(0.0, stem[1] + 0.05, {"100": 6, "50": 10}[scale]), "cedar-bark", 0.3, region="bark-lower"))
        n = {"200": 3, "100": 4, "50": 6}[scale]
        for i in range(1, len(stem) - 1):
            last = i == len(stem) - 2
            y0 = 0.0 if (i == 1 and scale == "200") else stem[i]
            y1 = stem[i + 1] if last else stem[i + 1] + 0.04
            reg, mat = ("bark-lower", "cedar-bark") if i == 1 else ("bark-upper", "cedar-bark-upper")
            out.append(Revolve("leader" if last else "stem-%d" % i, bones[names[i - 1]], 0, 0, stem_profile(y0, y1, n + (3 if last else 0)), mat, 0.32 if i == 1 else 0.36, region=reg))
        if scale == "50":
            for j, (az, Lr, r0) in enumerate([(30, 1.4, 0.2), (105, 1.1, 0.17), (170, 1.5, 0.21), (240, 1.0, 0.16), (305, 1.2, 0.18), (355, 0.9, 0.15)]):
                a = az * math.pi / 180
                out.append(Capsule("root-%d" % (j + 1), bones["root-plate"], (0.4 * math.cos(a), r0, 0.4 * math.sin(a)), ((0.4 + Lr) * math.cos(a), 0.06, (0.4 + Lr) * math.sin(a)), r0, 0.06, "cedar-bark", 0.28, region="bark-lower"))
        return out

    def limb_solid(l, scale):
        fac = {"500": 1.15, "200": 1.05, "100": 1.0, "50": 1.0}[scale]
        r1 = max(l.rb * 0.45, {"500": 0.06, "200": 0.04, "100": 0.03, "50": 0.025}[scale])
        p1 = (l.px, l.drop - 0.05, l.pz)
        return Capsule("limb-%s" % l.id, l.limb, l.limb.o, p1, l.rb * fac, r1, "cedar-bark-upper", 0.38, region="limb-bark")

    def plate_solids(l, scale, ref=None):
        out = []
        cy = l.plate.o[1]
        if scale == "50":
            out.append(Revolve("plate-%s" % l.id, l.plate, l.px, l.pz, plate_profile(cy, l.R, l.th, 11), l.material, 0.6 if l.region == "needles-sun" else 0.7, region=l.region, ls=l.ls))
            for j, (ang, fr, rr, hf) in enumerate(TUFTS[l.id]):
                a = ang * math.pi / 180
                tx, tz = l.px + fr * l.R * math.cos(a), l.pz + fr * l.R * math.sin(a) * l.ls
                prof = plate_profile(cy + hf * l.th, rr * l.R, 0.55 * l.th, 7, dome=0.8, under=0.2)
                out.append(Revolve("tuft-%s-%d" % (l.id, j + 1), l.plate, tx, tz, prof, l.material, 0.55 if l.region == "needles-sun" else 0.65, region=l.region))
            return out
        x0, x1, z0, z1, ytop = ref[l.id]
        n = {"100": 8, "200": 6, "500": 5}[scale]
        cx, cz, f, k = fit_ring(x0, x1, z0, z1, l.R)
        extra = {"100": 0.12, "200": 0.22, "500": 0.3}[scale]
        out.append(Revolve("plate-%s" % l.id, l.plate, cx, cz, plate_profile(cy, l.R * f, l.th + extra, n), l.material, 0.6 if l.region == "needles-sun" else 0.7, region=l.region, ls=k))
        if scale == "100":
            ang, fr, rr, hf = TUFTS[l.id][0]
            a = ang * math.pi / 180
            tx, tz = l.px + fr * l.R * math.cos(a), l.pz + fr * l.R * math.sin(a) * l.ls
            out.append(Revolve("tuft-%s-1" % l.id, l.plate, tx, tz, plate_profile(cy + hf * l.th, rr * l.R * 1.05, 0.6 * l.th, 6, dome=0.8, under=0.2), l.material, 0.55 if l.region == "needles-sun" else 0.65, region=l.region))
        return out

    fine50 = stem_solids("50") + [limb_solid(l, "50") for l in limbs] + [s for l in limbs for s in plate_solids(l, "50")]
    ref = {}
    for l in limbs:
        b = bbox_union([s for s in fine50 if s.bone.id == l.plate.id])
        ref[l.id] = (b[0], b[3], b[2], b[5], b[4])
    bb = bbox_union(fine50)
    dims = {"heightM": r3(bb[4]), "widthM": r3(bb[3] - bb[0]), "depthM": r3(bb[5] - bb[2])}

    # ------------------------------------------------------------------ masses ------------------------------------------------------
    for bid in ["root-plate"] + names:
        b = bones[bid]
        kg = sum(s.volume() for s in fine50 if s.bone.id == bid) * green_density
        b.mass = {"kg": round(kg, 1), "comX": r3(b.L / 2), "comY": 0, "distribution": "uniform", "source": "authored-estimate", "reference": "wood-handbook-2021"}
    area = {"needles-sun": needle_area_sun, "needles-shade": needle_area_shade}
    w = {l.id: l.R ** 2 * l.ls for l in limbs}
    for l in limbs:
        l.needle_m2 = area[l.region] * w[l.id] / sum(w[m.id] for m in limbs if m.region == l.region)
        l.kg = l.needle_m2 * needle_kg_m2 + 0.6 * math.pi * l.R ** 2 * l.ls * 0.012 * green_density
        l.plate.mass = {"kg": round(l.kg, 1), "comX": r3(0.25), "comY": 0, "distribution": "point", "source": "authored-estimate", "reference": "jones-2014"}
        lk = limb_solid(l, "50").volume() * green_density
        l.limb.mass = {"kg": round(lk, 1), "comX": r3(l.limb.L * 0.42), "comY": 0, "distribution": "uniform", "source": "authored-estimate", "reference": "wood-handbook-2021"}
    dims["massKg"] = round(sum(b.mass["kg"] for b in bones.values()))

    anchors = [{"id": "base-support", "bone": "root-plate", "x": 0, "y": 0, "kind": "support", "title": T("Root plate anchorage", "Verankerung des Wurzeltellers"), "support": "fixed",
                "normalDeg": 90, "footprintLengthM": 3.0, "footprintWidthM": 3.0, "frictionCoefficient": 0.6}]
    outlets = {"needles-sun": [], "needles-shade": []}
    for l in limbs:
        word = "sun" if l.region == "needles-sun" else "shade"
        j = len(outlets[l.region]) + 1
        aid = "tp-%s-%d" % (word, j)
        u = 0.15 * l.th
        anchors.append({"id": aid, "bone": l.plate.id, "x": r4(u), "y": 0, "kind": "emitter",
                        "title": T("Transpiration of the %s plates, anchor %d" % ("sunlit" if word == "sun" else "lower", j), "Transpiration der %s Platten, Anker %d" % ("besonnten" if word == "sun" else "unteren", j))})
        outlets[l.region].append((aid, l, (l.px, l.plate.o[1] + u, l.pz)))
    crown_plan = math.pi * dims["widthM"] / 2 * dims["depthM"] / 2
    regions = [
        {"id": "bark-lower", "title": T("Lower stem and root flare, dark plated bark", "Unterer Stamm und Wurzelanlauf, dunkle Plattenborke"), "kind": "bark", "model": "passive", "material": "cedar-bark",
         "areaM2": round(sum(s.lateral_area() for s in fine50 if s.region == "bark-lower"), 2), "characteristicLengthM": round(2 * r_stem(1.3), 2), "geometry": "cylinder", "skyViewFactor": 0.35,
         "layers": [], "heatCapacityJM2K": 75000, "source": "authored-estimate", "reference": "wood-handbook-2021"},
        {"id": "bark-upper", "title": T("Upper stem and leader", "Oberer Stamm und Gipfeltrieb"), "kind": "bark", "model": "passive", "material": "cedar-bark-upper",
         "areaM2": round(sum(s.lateral_area() for s in fine50 if s.region == "bark-upper"), 2), "characteristicLengthM": round(2 * r_stem(12), 2), "geometry": "cylinder", "skyViewFactor": 0.3,
         "layers": [], "heatCapacityJM2K": 50000, "source": "authored-estimate", "reference": "wood-handbook-2021"},
        {"id": "limb-bark", "title": T("Bark of the ascending limbs", "Rinde der aufsteigenden Starkäste"), "kind": "bark", "model": "passive", "material": "cedar-bark-upper",
         "areaM2": round(sum(s.lateral_area() for s in fine50 if s.region == "limb-bark"), 2), "characteristicLengthM": 0.3, "geometry": "cylinder", "skyViewFactor": 0.3,
         "layers": [], "heatCapacityJM2K": 35000, "source": "authored-estimate", "reference": "wood-handbook-2021"},
    ]
    for reg, ar, gs, sky in (("needles-sun", needle_area_sun, gs_sun, 0.75), ("needles-shade", needle_area_shade, gs_shade, 0.4)):
        regions.append({"id": reg, "title": T("Needles of the sunlit upper plates" if reg == "needles-sun" else "Needles of the lower plates", "Nadeln der besonnten oberen Platten" if reg == "needles-sun" else "Nadeln der unteren Platten"),
                        "kind": "foliage", "model": "leaf", "material": "cedar-needles-sun" if reg == "needles-sun" else "cedar-needles-shade", "areaM2": ar, "areaBasis": "exchange",
                        "characteristicLengthM": 0.0015, "geometry": "horizontal-plate-up", "skyViewFactor": sky, "layers": [], "heatCapacityJM2K": 1200,
                        "leaf": {"stomatalConductanceMaxMS": gs, "leafAreaIndex": round(ar / crown_plan, 2), "photosynthesis": "c3"}, "source": "authored-estimate", "reference": "jones-2014"})
    emitters = []
    for reg, eid in (("needles-sun", "transpiration-sun"), ("needles-shade", "transpiration-shade")):
        ids = [a for a, _, _ in outlets[reg]]
        emitters.append({"id": eid, "kind": "transpiration", "title": T("Transpiration of the %s plates (one anchor per plate)" % ("sunlit upper" if reg == "needles-sun" else "lower"), "Transpiration der %s Platten (ein Anker je Platte)" % ("besonnten oberen" if reg == "needles-sun" else "unteren")),
                         "anchor": ids[0], "anchors": ids, "directionDeg": 0, "diameterM": 3.0, "region": reg, "source": "authored-estimate", "reference": "monteith-unsworth-2013"})

    # ------------------------------------------------------------------ wind, poses, clips --------------------------------------------
    for l in limbs:
        ps = [s for s in fine50 if s.bone.id == l.plate.id]
        ys = [l.plate.o[1] - l.th + 0.02 * i for i in range(int(2.5 * l.th / 0.02))]
        l.area = sum(union_width(slice_intervals(ps, y)) * 0.02 for y in ys)
        l.zc = l.plate.o[1]
    sun_frac = {"bark-lower": 0.4, "bark-upper": 0.15, "limb-bark": 0.25, "needles-sun": 0.9, "needles-shade": 0.3}

    def loads(v10):
        out = []
        for l in limbs:
            F = wind_force(l.area, l.zc, v10, cd=cd)
            out.append({"id": "wind-%s" % l.plate.id, "kind": "wind", "title": T("Wind drag on needle plate %s" % l.id, "Winddruck auf Nadelplatte %s" % l.id), "bone": l.plate.id, "x": 0.2, "y": 0,
                        "fxN": round(F, 1), "fyN": 0, "source": "literature", "reference": "en-1991-1-4"})
        q = 0.5 * RHO * 1.0 * 2 * r_stem(1.3) * v_at(v10, crown_base / 2) ** 2
        out.append({"id": "wind-stem", "kind": "wind", "title": T("Wind drag on the bare lower stem", "Winddruck auf den kahlen unteren Stamm"), "fxN": 0, "fyN": 0,
                    "distributed": {"bone": "trunk-1", "fromM": 0, "toM": r3(crown_base - stem[1]), "wxNPerM": round(q, 2), "wyNPerM": 0}, "source": "literature", "reference": "en-1991-1-4"})
        return out

    def EI(y):
        d = 2 * max(r_stem(y), 0.04)
        return E_green * math.pi * d ** 4 / 64

    def lean(v10, extra):
        forces = [wind_force(l.area, l.zc, v10, cd=cd) for l in limbs]
        table = cantilever_slopes([l.zc for l in limbs], forces, EI, k_root, H)
        offs, prev = {}, 0.0
        for i in range(1, len(stem) - 1):
            sl = slope_at(table, (stem[i] + stem[i + 1]) / 2)
            offs[names[i - 1]] = -(sl - prev) * 180 / math.pi
            prev = sl
        for l in limbs:
            offs[l.limb.id] = -extra * 3.0
            offs[l.plate.id] = -extra * 5.0
        tip = sum(slope_at(table, 0.05 * (i + 0.5)) * 0.05 for i in range(int(H / 0.05)))
        return offs, sum(forces), sum(f * l.zc for f, l in zip(forces, limbs)), tip

    def binding(sun, wet=None):
        out = {}
        for r in regions:
            d = {"exposedFraction": 1, "sunlitFraction": round(sun[r["id"]], 3)}
            if wet is not None:
                d["wettedness"] = wet
            out[r["id"]] = d
        return out

    def rnd2(d):
        return {k: {"angleDeg": round(v, 2)} for k, v in d.items() if abs(round(v, 2)) > 0}
    _, calm_F, calm_M, _ = lean(v_calm, 0)
    windy_off, windy_F, windy_M, windy_tip = lean(v_windy, (v_windy / v_storm) ** 2)
    storm_off, storm_F, storm_M, storm_tip = lean(v_storm, 1.0)
    crown_area = sum(l.area for l in limbs)
    snow, sbones = [], {}
    for l in limbs:
        A = math.pi * l.R ** 2 * l.ls
        F = 20 * 9.81 * A
        snow.append({"id": "snow-%s" % l.plate.id, "kind": "snow", "title": T("Snow on needle plate %s" % l.id, "Schnee auf Nadelplatte %s" % l.id), "bone": l.plate.id, "x": 0.3, "y": 0, "fxN": 0, "fyN": -round(F),
                     "source": "authored-estimate", "reference": "en-1991-1-3"})
        sgn = 1 if math.cos(l.limb.theta * math.pi / 180) < 0 else -1
        if l.id != "top":
            sbones[l.limb.id] = {"angleDeg": round(sgn * (1.5 + 0.5 * l.R), 2)}
            sbones[l.plate.id] = {"angleDeg": round(sgn * (2.0 + 1.0 * l.R), 2)}
    snow_kn = sum(-s["fyN"] for s in snow) / 1000
    poses = [
        {"id": "calm", "title": T("Calm", "Windstille"),
         "description": T("Light air of 2 m/s at 10 m: the stem stands vertical and the %d plates lie level on their limbs; %.0f N of drag in all on %.0f m2 of plate silhouette." % (len(limbs), calm_F, crown_area),
                          "Leiser Zug von 2 m/s in 10 m Höhe: Der Stamm steht senkrecht und die %d Platten liegen waagerecht auf ihren Starkästen; insgesamt %s N Winddruck auf %s m2 Plattensilhouette." % (len(limbs), fmt(calm_F, True, 0), fmt(crown_area, True, 0))),
         "bones": {}, "ground": "authored", "contacts": [], "thermal": {"regions": binding(sun_frac), "emitters": {"transpiration-sun": 1, "transpiration-shade": 1}}, "loads": loads(v_calm)},
        {"id": "windy", "title": T("Fresh breeze", "Frische Brise"),
         "description": T("Beaufort 5, 8 m/s at 10 m with the power-law profile and cd 0.4 on the side silhouette of each plate (reduced above 8 m/s): %.1f kN on the crown, a base moment of %.0f kN m; the massive stem (E 8 GPa) on a root plate of 45 MN m per radian leans %.2f m at the top, the plates rock a few degrees." % (windy_F / 1000, windy_M / 1000, windy_tip),
                          "Beaufort 5, 8 m/s in 10 m Höhe mit Potenzprofil und cd 0,4 auf die Seitensilhouette jeder Platte (über 8 m/s verringert): %s kN auf die Krone, ein Fußmoment von %s kNm; der mächtige Stamm (E 8 GPa) auf einem Wurzelteller von 45 MNm je Radiant neigt sich oben um %s m, die Platten schaukeln um wenige Grad." % (fmt(windy_F / 1000, True), fmt(windy_M / 1000, True, 0), fmt(windy_tip, True, 2))),
         "bones": rnd2(windy_off), "ground": "authored", "contacts": [], "thermal": {"regions": binding(sun_frac), "emitters": {"transpiration-sun": 1, "transpiration-shade": 1}}, "loads": loads(v_windy)},
        {"id": "storm", "title": T("Gale", "Sturm"),
         "description": T("Beaufort 9, 22 m/s at 10 m: %.1f kN on the wide plates and a base moment of %.0f kN m; the top moves %.2f m and the plates tilt to the lee. Rain wets every surface." % (storm_F / 1000, storm_M / 1000, storm_tip),
                          "Beaufort 9, 22 m/s in 10 m Höhe: %s kN auf die breiten Platten und ein Fußmoment von %s kNm; die Spitze weicht %s m aus und die Platten kippen nach Lee. Regen benetzt alle Oberflächen." % (fmt(storm_F / 1000, True), fmt(storm_M / 1000, True, 0), fmt(storm_tip, True, 2))),
         "bones": rnd2(storm_off), "ground": "authored", "contacts": [], "thermal": {"regions": binding({k: 0.35 * v for k, v in sun_frac.items()}, 0.4), "emitters": {"transpiration-sun": 0.5, "transpiration-shade": 0.4}},
         "loads": loads(v_storm)},
        {"id": "snow", "title": T("Snow on the plates", "Schnee auf den Platten"),
         "description": T("Wet snow of 20 kg/m2 on the plan area of every plate (order of magnitude after EN 1991-1-3), %.1f kN in all: the flat plates of a cedar catch snow like shelves, the limbs bend down by 2 to 3 degrees and the plates tip outwards; calm, overcast, transpiration nearly at rest. Snow break of such limbs is the known weakness of cedars in parks." % snow_kn,
                          "Nassschnee von 20 kg/m2 auf der Grundrissfläche jeder Platte (Größenordnung nach EN 1991-1-3), insgesamt %s kN: die flachen Platten einer Zeder fangen Schnee wie Regalböden, die Starkäste biegen sich um 2 bis 3 Grad herab und die Platten kippen nach außen; windstill, bedeckt, die Transpiration ruht fast. Schneebruch solcher Äste ist die bekannte Schwäche der Zedern in Parks." % fmt(snow_kn, True)),
         "bones": sbones, "ground": "authored", "contacts": [], "thermal": {"regions": binding({k: 0.0 for k in sun_frac}, 0.6), "emitters": {"transpiration-sun": 0.03, "transpiration-shade": 0.03}},
         "loads": loads(v_calm) + snow},
    ]
    depth = {}
    for bid in rig.order:
        depth[bid] = 0 if bones[bid].parent is None else depth[bones[bid].parent] + 1
    P = sway_period
    keys = []
    for i in range(9):
        bo = {b: {"angleDeg": round(base + (0.45 * abs(base) + (0.5 if b.startswith("plate") else 0.2 if b.startswith("limb") else 0)) * math.sin(2 * math.pi * (i % 8) / 8 - 0.35 * depth[b]), 2)} for b, base in windy_off.items()}
        keys.append({"t": round(P * i / 8, 4), "pose": "windy", "ease": "smooth", "bones": bo})
    keys[-1]["bones"] = {k: dict(v) for k, v in keys[0]["bones"].items()}
    sway = {"id": "wind-sway", "title": T("Wind sway", "Windschwingung"),
            "description": T("A deterministic %.1f s sway cycle about the fresh-breeze lean (first bending mode of a 25 m tree with a heavy, wide crown, about %.2f Hz, in the range reviewed by Moore and Maguire 2004): the stem bends gently, limbs and plates lag behind and rock." % (P, 1 / P),
                             "Ein deterministischer Schwingungszyklus von %s s um die Neigung der frischen Brise (erste Biegeeigenform eines 25 m hohen Baums mit schwerer, breiter Krone, etwa %s Hz, im von Moore und Maguire 2004 zusammengestellten Bereich): Der Stamm biegt sich sanft, Starkäste und Platten hinken nach und schaukeln." % (fmt(P, True), fmt(1 / P, True, 2))),
            "durationS": P, "fps": 30, "loop": True, "rootMotion": "none", "keys": keys, "contacts": [], "phases": [{"id": "sway", "fromS": 0, "toS": P, "title": T("Sway cycle", "Schwingungszyklus")}],
            "events": [{"t": 0, "kind": "motion", "title": T("Cycle start at the mean breeze lean", "Zyklusbeginn in der mittleren Windneigung")}],
            "tracks": [{"target": "emitter:%s" % e, "ease": "smooth", "keys": [{"t": 0, "v": 1}, {"t": round(P / 2, 3), "v": 0.92}, {"t": P, "v": 1}]} for e in ("transpiration-sun", "transpiration-shade")]}

    def gs_(t_):
        return math.exp(-((t_ - 2.5) / 0.9) ** 2) if t_ < 2.5 else math.exp(-((t_ - 2.5) / 1.8) ** 2)
    gust = {"id": "gust", "title": T("Gust", "Windböe"),
            "description": T("A single 8 s gust on the fresh breeze: the wind-load tracks rise from 5 % to 2.75 times the breeze load at 2.5 s (EN 1991-1-4 peak velocity pressure, turbulence intensity 0.25) and decay; stem, limbs and plates lean up to 70 % of the way to the gale lean.",
                             "Eine einzelne Böe von 8 s auf der frischen Brise: Die Windlastspuren steigen von 5 % auf das 2,75-fache der Brisenlast bei 2,5 s (Spitzengeschwindigkeitsdruck nach EN 1991-1-4, Turbulenzintensität 0,25) und klingen ab; Stamm, Starkäste und Platten neigen sich bis zu 70 % des Wegs zur Sturmneigung."),
            "durationS": 8, "fps": 30, "loop": False, "rootMotion": "none",
            "keys": [{"t": t_, "pose": "windy", "ease": "smooth", "bones": {b: {"angleDeg": round(windy_off[b] + (storm_off[b] - windy_off[b]) * 0.7 * gs_(t_), 2)} for b in storm_off}} for t_ in [0, 1, 1.8, 2.5, 3.2, 4.2, 5.5, 8]],
            "contacts": [], "phases": [{"id": "rise", "fromS": 0, "toS": 2.5, "title": T("Gust front", "Böenfront")}, {"id": "peak", "fromS": 2.5, "toS": 3.6, "title": T("Peak", "Spitze")}, {"id": "decay", "fromS": 3.6, "toS": 8, "title": T("Decay and recovery", "Abklingen und Erholung")}],
            "events": [{"t": 2.5, "kind": "load-on", "title": T("Peak gust load", "Böenspitze")}, {"t": 5, "kind": "load-off", "title": T("Load back near the mean", "Last wieder nahe dem Mittelwert")}],
            "tracks": [{"target": "load:" + ld["id"], "ease": "smooth", "keys": [{"t": t_, "v": round(0.05 + 2.7 * gs_(t_), 3)} for t_ in [0, 1.2, 2.5, 3.6, 5, 8]]} for ld in loads(v_windy)]}
    sun_day = [(0, 0.0), (5, 0.0), (6, 0.05), (8, 0.43), (10, 0.79), (12, 1.0), (14, 0.79), (16, 0.36), (18, 0.0), (24, 0.0)]
    tr_day = [(0, 0.02), (5, 0.04), (7, 0.4), (9, 0.95), (11, 0.85), (13, 0.6), (15, 0.65), (17, 0.5), (19, 0.1), (24, 0.02)]
    diurnal = {"id": "diurnal-transpiration", "title": T("Day of transpiration", "Transpirationstag"),
               "description": T("A hot, dry summer day of 24 h compressed to 24 s (1 s = 1 h) in calm weather: the sunlit fraction of every region follows the sun; the drought-adapted cedar transpires most in the morning and closes its stomata at midday (a deep midday depression), nearly closed at night; temperatures follow from the thermal solver.",
                                "Ein heißer, trockener Sommertag von 24 h auf 24 s gerafft (1 s = 1 h) bei Windstille: Der besonnte Anteil jeder Region folgt der Sonne; die an Trockenheit angepasste Zeder transpiriert am meisten am Morgen und schließt mittags ihre Spaltöffnungen (eine tiefe Mittagsdepression), nachts fast geschlossen; die Temperaturen folgen aus dem thermischen Löser."),
               "durationS": 24, "fps": 10, "loop": True, "rootMotion": "none", "keys": [{"t": 0, "pose": "calm", "ease": "smooth"}, {"t": 12, "pose": "calm", "ease": "smooth"}, {"t": 24, "pose": "calm", "ease": "smooth"}], "contacts": [],
               "phases": [{"id": "night", "fromS": 0, "toS": 6, "title": T("Night", "Nacht")}, {"id": "morning", "fromS": 6, "toS": 12, "title": T("Morning to noon", "Morgen bis Mittag")}, {"id": "afternoon", "fromS": 12, "toS": 18, "title": T("Afternoon", "Nachmittag")}, {"id": "evening", "fromS": 18, "toS": 24, "title": T("Evening and night", "Abend und Nacht")}],
               "events": [{"t": 6, "kind": "thermal", "title": T("Sunrise", "Sonnenaufgang")}, {"t": 12, "kind": "thermal", "title": T("Solar noon, stomata half closed", "Sonnenhöchststand, Spaltöffnungen halb geschlossen")}, {"t": 18, "kind": "thermal", "title": T("Sunset", "Sonnenuntergang")}],
               "tracks": [{"target": "emitter:%s" % e, "ease": "smooth", "keys": [{"t": t_, "v": v} for t_, v in tr_day]} for e in ("transpiration-sun", "transpiration-shade")] +
                         [{"target": "region:%s:sunlitFraction" % r["id"], "ease": "smooth", "keys": [{"t": t_, "v": round(v * sun_frac[r["id"]], 3)} for t_, v in sun_day]} for r in regions]}
    clips = [sway, gust, diurnal]
    need = {}
    for p in poses:
        for b, o in p["bones"].items():
            need[b] = max(need.get(b, 0), abs(o["angleDeg"]))
    for c in clips:
        for k in c["keys"]:
            for b, o in k.get("bones", {}).items():
                need[b] = max(need.get(b, 0), abs(o["angleDeg"]))
    for b in bones.values():
        if b.limit is not None:
            b.limit = max(b.limit, math.ceil(need.get(b.id, 0) * 1.25 + 0.5))

    # ------------------------------------------------------------------ scales -------------------------------------------------------
    reps, vols, report = {}, {}, {}
    for scale in SCALES:
        body = stem_solids(scale) + [limb_solid(l, scale) for l in limbs] + [s for l in limbs for s in plate_solids(l, scale, ref)]
        shapes = {"elevation": [], "front": [], "plan": []}
        for view in shapes:
            for i, s in enumerate(body):
                z = 1.0 if s.bone.id in names or s.bone.id == "root-plate" else 1.5 if s.bone.id.startswith("limb") else 2.0 + 0.001 * i
                shapes[view].append(shape_of(s, view, round(z, 3)))
            shapes[view].extend(texture(scale, view, body, limbs))
        reps[scale] = representation(scale, shapes, rep_decisions(scale, dims))
        th = [{"id": "th-%s" % s.id, "of": s.id, "region": s.region} for s in body]
        ro = {"500": 0.3, "200": 0.26, "100": 0.24, "50": 0.22}[scale]
        from conifer_lib import Ellipsoid
        for reg, eid in (("needles-sun", "transpiration-sun"), ("needles-shade", "transpiration-shade")):
            for j, (aid, l, p) in enumerate(outlets[reg]):
                e = Ellipsoid("outlet-%s-%d" % (eid.split("-")[1], j + 1), l.plate, p, (ro, 0.5 * ro, ro), l.material, 0.5).json()
                e["role"], e["tags"], e["region"] = "detail", [eid], reg
                th.append(e)
        fac = {"500": 0.56, "200": 0.58, "100": 0.6, "50": 0.62}[scale]
        nst = {"500": 3, "200": 4, "100": 5, "50": 7}[scale]
        rp = bones["root-plate"]
        st = [dict(Revolve("support-base", rp, 0, 0, [(0, 0), (0, 1.1), (0.08, 1.1), (0.08, 0)], wood, 0.25).json(), role="detail", member="root-plate", part="support", anchor="base-support"),
              dict(Revolve("member-root-plate", rp, 0, 0, stem_profile(0.02, stem[1] + 0.05, nst, fac), wood, 0.4).json(), member="root-plate", part="member")]
        for i in range(1, len(stem) - 1):
            last = i == len(stem) - 2
            y0, y1 = stem[i] - 0.03, (stem[i + 1] - 0.4 if last else stem[i + 1] + 0.05)
            m = Revolve("member-%s" % names[i - 1], bones[names[i - 1]], 0, 0, stem_profile(y0, y1, nst, fac), wood, 0.4)
            m.slender = (y1 - y0) / (2 * m.R) >= 5.5
            st.append(dict(m.json(), member=names[i - 1], part="member"))
        for l in limbs:
            c = Capsule("member-%s" % l.limb.id, l.limb, l.limb.o, (l.px * 0.97, l.drop - 0.05, l.pz * 0.97), l.rb * fac, max(0.03, l.rb * 0.3 * fac), wood, 0.4, slender=True)
            st.append(dict(c.json(), member=l.limb.id, part="member"))
        vols[scale] = {"status": "authored", "denominator": int(scale), "body": [s.json() for s in body], "thermal": th, "structure": st, "decisions": volume_decisions(scale, len(body))}
        report[scale] = {"solids": len(body), "gaps": {v: len(gap_report(body, v, scale, PENSUM[scale])) for v in ("elevation", "front", "plan")}}
    asset = {"id": id, "category": "vegetation", "title": T("Atlas Cedar", "Atlaszeder"),
             "summary": T("A mature Atlas cedar (Cedrus atlantica) of 25 m in a park: a massive, dark-barked stem that divides its strength into ten long ascending limbs, each carrying a wide, flat, horizontal plate of blue-green needles, so that the broad crown is a stack of tiered plates with sky between them under a stiff, upright leader.",
                          "Eine ausgewachsene Atlaszeder (Cedrus atlantica) von 25 m in einem Park: ein mächtiger Stamm mit dunkler Borke, der seine Kraft in zehn lange, aufsteigende Starkäste teilt, von denen jeder eine breite, flache, waagerechte Platte aus blaugrünen Nadeln trägt, so dass die breite Krone ein Stapel geschichteter Platten mit Himmel dazwischen unter einem steifen, aufrechten Gipfeltrieb ist."),
             "tags": ["tree", "evergreen", "conifer", "tiered-crown", "wind", "snow", "transpiration"],
             "provenance": {"author": "Semio staffage plant worker PW-07", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES,
                            "notes": T("Authored by hand after a 120-year-old park cedar: the stem is 0.84 m thick at breast height and the crown starts in plates from about 5 m. Masses are the volume of every drawn wooden solid of the 1:50 model times green density (Wood Handbook cedar proxy) plus needle and twig estimates per plate; the root system below ground is not part of the asset. The 2D views of every scale are the exact orthographic projections of that scale's 3D body.",
                                       "Von Hand nach einer 120-jährigen Parkzeder erstellt: der Stamm ist in Brusthöhe 0,84 m dick, die Krone beginnt mit Platten ab etwa 5 m. Massen sind das Volumen jedes gezeichneten Holzkörpers des 1:50-Modells mal Frischholzdichte (Zedern-Proxy des Wood Handbook) plus Nadel- und Zweigschätzungen je Platte; das Wurzelsystem unter dem Boden gehört nicht zum Asset. Die 2D-Ansichten jedes Maßstabs sind die exakten Parallelprojektionen des 3D-Körpers dieses Maßstabs.")},
             "dimensions": dims, "materials": MATERIALS, "rig": {"kind": "botanical", "root": "root-plate", "bones": rig.json()}, "volumes": vols, "anchors": anchors, "representations": reps,
             "symbols": [symbol(dims, [s for s in stem_solids("500")] + [limb_solid(l, "500") for l in limbs] + [s for l in limbs for s in plate_solids(l, "500", ref)])], "poses": poses, "clips": clips,
             "thermal": {"environment": {"airTempC": 28, "meanRadiantTempC": 31, "relativeHumidity": 0.35, "windMS": 2, "windDirectionDeg": 0, "solarWM2": 850, "pressurePa": 101325, "diffuseFraction": 0.12},
                         "regions": regions, "emitters": emitters, "assumptions": THERMAL_ASSUMPTIONS, "operating": [], "passengers": []},
             "structure": {"gravityMS2": G, "dynamics": "quasi-static", "supports": ["base-support"], "assumptions": STRUCTURE_ASSUMPTIONS, "clinical": False},
             "presentation": {"defaultPose": "calm", "defaultClip": "wind-sway", "defaultScale": 200},
             "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"], "completion": "authored"}
    info = {"dims": dims, "crown_area": round(crown_area, 1), "windy": (round(windy_F), round(windy_M), round(windy_tip, 3)), "storm": (round(storm_F), round(storm_M), round(storm_tip, 3)), "snow_kN": round(snow_kn, 1), "report": report, "bones": len(rig.order)}
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}, info


def texture(scale, view, body, limbs):
    """Layered tuft strokes in every plate (1:50 short horizontal dashes in side and front, radial strokes in plan; 1:100 fewer), bark plates on the bare stem (1:50)."""
    if scale in ("500", "200"):
        return []
    den = int(scale)
    outlines = {s.id: outline_polyline(s, view, 48) for s in body}
    M = Marks(view, outlines, 0.5 * den / 1000)
    by = {s.id: s for s in body}
    strokes = {}
    for l in sorted(limbs, key=lambda m: -m.plate.o[1]):
        host = "plate-%s" % l.id
        s = by[host]
        cy = l.plate.o[1]
        if view in ("elevation", "front"):
            c = l.px if view == "elevation" else l.pz
            k = 1.0 if view == "elevation" else l.ls
            n = {"100": 4, "50": 9}[scale]
            for i in range(n):
                f = -0.8 + 1.6 * (i + 0.5) / n
                for row, dy in enumerate((0.05 * l.th, -0.1 * l.th)):
                    x = c + f * l.R * k + 0.1 * row
                    st = [(x - 0.16, cy + dy - 0.02), (x, cy + dy), (x + 0.16, cy + dy - 0.03)]
                    if M.offer(host, host, st):
                        strokes.setdefault(host, []).append(st)
        elif scale == "50":
            for j in range(12):
                a = 2 * math.pi * (j + 0.3) / 12
                st = [(l.px + 0.55 * l.R * math.cos(a), l.pz + 0.55 * l.R * math.sin(a) * l.ls), (l.px + 0.82 * l.R * math.cos(a + 0.06), l.pz + 0.82 * l.R * math.sin(a + 0.06) * l.ls)]
                if M.offer(host, host, st):
                    strokes.setdefault(host, []).append(st)
    if scale == "50" and view in ("elevation", "front"):
        host = "stem-1"
        for i in range(24):
            y = 1.0 + 0.55 * (i % 6) + 0.2 * (i // 6)
            x = -0.3 + 0.2 * (i // 6)
            st = [(x, y), (x + 0.03, y + 0.18), (x - 0.02, y + 0.32)]
            if M.offer(host, host, st):
                strokes.setdefault(host, []).append(st)
    return [marks_shape("%s-texture" % h, by[h].bone, view, ss, 3.0, by[h].tone, region=by[h].region, material=by[h].material) for h, ss in strokes.items()]


def symbol(dims, parts):
    """Plan symbol at true extent: the 1:500 outline of every plate as one lobed crown line (union per height in side and front, the plate circles' hull in plan) and the stem."""
    def side(view):
        ys = [0.5 + i * 0.1 for i in range(int((dims["heightM"] - 0.5) / 0.1) + 1)]
        right, left = [], []
        for y in ys:
            iv = slice_intervals(parts, y, view)
            if iv:
                right.append((max(b for _, b in iv), y))
                left.append((min(a for a, _ in iv), y))
        return simplify(right, 0.08) + list(reversed(simplify(left, 0.08)))
    pts = []
    for s in parts:
        pts += outline_polyline(s, "plan", 24)
    pts.sort()

    def turn(o, a, b):
        return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])
    lo, hi = [], []
    for p in pts:
        while len(lo) >= 2 and turn(lo[-2], lo[-1], p) <= 0:
            lo.pop()
        lo.append(p)
    for p in reversed(pts):
        while len(hi) >= 2 and turn(hi[-2], hi[-1], p) <= 0:
            hi.pop()
        hi.append(p)
    hull = lo[:-1] + hi[:-1]
    r0, rb = r_stem(0), r_stem(crown_base)
    stem_pts = [(-r0, 0), (-rb, crown_base), (rb, crown_base), (r0, 0)]

    def poly(i, p, filled, tone):
        return {"id": i, "bone": "root-plate", "role": "body", "z": 3 if filled else 2, "filled": filled, "tone": tone, "kind": "poly", "points": [[r4(x), r4(y)] for x, y in p], "closed": True}
    rs = r_stem(1.3)
    return {"id": "atlas-cedar-symbol", "title": T("Plan symbol with true crown extent", "Grundrisssymbol mit wahrer Kronenausdehnung"),
            "description": T("A conventional sign of the Atlas cedar for site and landscape plans: the outline of the true crown extent and the stem section. It is a separate symbol and is never used silently for a pictorial scale.",
                             "Ein konventionelles Zeichen der Atlaszeder für Lage- und Freiraumpläne: der Umriss der wahren Kronenausdehnung und der Stammquerschnitt. Es ist ein eigenes Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet."),
            "semantics": "true-extent", "units": "model-m", "denominators": [500, 200, 100], "pens": {"body": 0.18, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.25}, "hatches": [],
            "geometry": {"elevation": [poly("crown", side("elevation"), False, 0.5), poly("stem", stem_pts, True, 0.3)], "front": [poly("crown", side("front"), False, 0.5), poly("stem", stem_pts, True, 0.3)],
                         "plan": [poly("crown", hull, False, 0.5), {"id": "stem", "bone": "root-plate", "role": "body", "z": 3, "filled": True, "tone": 0.3, "kind": "ellipse", "cx": 0, "cy": 0, "rx": r4(rs), "ry": r4(rs)}]},
            "dimensionalNote": T("Drawn at true extent in model metres in all three views (%.2f m wide, %.2f m deep and %.2f m high): the outline is the crown of plates, the stem is drawn at its real position and thickness. It is a symbol, never used silently for a pictorial scale." % (dims["widthM"], dims["depthM"], dims["heightM"]),
                                 "In wahrer Ausdehnung in Modellmetern in allen drei Ansichten gezeichnet (%s m breit, %s m tief und %s m hoch): Der Umriss ist die Plattenkrone, der Stamm liegt in wirklicher Lage und Stärke. Es ist ein Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet." % (fmt(dims["widthM"], True, 2), fmt(dims["depthM"], True, 2), fmt(dims["heightM"], True, 2)))}


REFERENCES = [
    {"id": "wood-handbook-2021", "citation": "Forest Products Laboratory (2021). Wood Handbook: Wood as an Engineering Material. General Technical Report FPL-GTR-282, USDA Forest Service. Chapter 5 (green density, modulus of elasticity and rupture; a medium-density softwood used as proxy for Cedrus atlantica, for which the handbook gives no values)."},
    {"id": "en-1991-1-4", "citation": "EN 1991-1-4:2005 Eurocode 1, Actions on structures, Part 1-4: Wind actions. Air density 1.25 kg/m3, F = 0.5 rho cd A v^2, peak velocity pressure (1 + 7 Iv) q_m."},
    {"id": "en-1991-1-3", "citation": "EN 1991-1-3:2003 Eurocode 1, Actions on structures, Part 1-3: Snow loads. Used for the order of magnitude of wet snow on flat needle plates."},
    {"id": "mayhead-1973", "citation": "Mayhead, G. J. (1973). Some drag coefficients for British forest trees derived from wind tunnel studies. Agricultural Meteorology 12, 123-130."},
    {"id": "moore-maguire-2004", "citation": "Moore, J. R. and Maguire, D. A. (2004). Natural sway frequencies and damping ratios of trees: concepts, review and synthesis of previous studies. Trees 18, 195-203."},
    {"id": "jones-2014", "citation": "Jones, H. G. (2014). Plants and Microclimate, 3rd ed., Cambridge University Press. Leaf energy balance, leaf area index, boundary-layer conductance."},
    {"id": "monteith-unsworth-2013", "citation": "Monteith, J. L. and Unsworth, M. H. (2013). Principles of Environmental Physics, 4th ed., Academic Press. Penman-Monteith leaf transpiration."},
    {"id": "farjon-2010", "citation": "Farjon, A. (2010). A Handbook of the World's Conifers. Brill, Leiden. Cedrus atlantica: habit, height, tiered crown of horizontal plates, needles in rosettes on short shoots."},
]
MATERIALS = [
    {"id": "cedar-bark", "title": T("Dark grey, plated cedar bark", "Dunkelgraue Plattenborke der Zeder"), "emissivity": 0.95, "solarAbsorptivity": 0.82, "densityKgM3": 520, "conductivityWmK": 0.12, "source": "authored-estimate", "reference": "jones-2014"},
    {"id": "cedar-bark-upper", "title": T("Smoother grey bark of limbs and upper stem", "Glattere graue Rinde der Starkäste und des oberen Stamms"), "emissivity": 0.94, "solarAbsorptivity": 0.75, "densityKgM3": 480, "conductivityWmK": 0.11, "source": "authored-estimate", "reference": "jones-2014"},
    {"id": "cedar-needles-sun", "title": T("Blue-green cedar needles in rosettes, sunlit plates", "Blaugrüne Zedernnadeln in Büscheln, besonnte Platten"), "emissivity": 0.97, "solarAbsorptivity": 0.48, "source": "literature", "reference": "jones-2014"},
    {"id": "cedar-needles-shade", "title": T("Cedar needles of the lower plates", "Zedernnadeln der unteren Platten"), "emissivity": 0.97, "solarAbsorptivity": 0.32, "source": "authored-estimate", "reference": "jones-2014"},
    {"id": "cedar-wood", "title": T("Green cedar wood", "Frisches Zedernholz"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 820, "conductivityWmK": 0.13, "elasticModulusPa": 8.0e9, "strengthPa": 4.2e7, "source": "authored-estimate", "reference": "wood-handbook-2021"},
]
THERMAL_ASSUMPTIONS = [
    T("Needle regions use the needle exchange area; the leaf area index is that area over the crown ground projection. The maximum stomatal conductance (1.8 mm/s sunlit, 1.2 mm/s lower) is an authored estimate for a drought-adapted Mediterranean mountain conifer guided by Jones (2014), the lowest of the conifers; photosynthesis type C3. The reference environment is a hot, dry summer day (28 degC, 35 % relative humidity, 850 W/m2).",
      "Nadelregionen verwenden die Austauschfläche der Nadeln; der Blattflächenindex ist diese Fläche bezogen auf die Kronenprojektion am Boden. Die maximale Spaltöffnungsleitfähigkeit (1,8 mm/s besonnt, 1,2 mm/s unten) ist eine Autorenschätzung für einen trockenheitsangepassten mediterranen Gebirgsnadelbaum, angelehnt an Jones (2014), die niedrigste der Nadelbäume; Photosynthesetyp C3. Die Bezugsumgebung ist ein heißer, trockener Sommertag (28 °C, 35 % relative Feuchte, 850 W/m2)."),
    T("Bark is passive, with the areal heat capacity of green wood over an effective depth of 1.5 to 3 cm (authored estimate after the Wood Handbook); the bark areas are the lateral areas of the 1:50 stem, flare, root and limb solids. The plates face the sky (horizontal plates).",
      "Die Rinde ist passiv, mit der flächenbezogenen Wärmekapazität von frischem Holz über eine wirksame Tiefe von 1,5 bis 3 cm (Autorenschätzung nach dem Wood Handbook); die Rindenflächen sind die Mantelflächen der 1:50-Körper von Stamm, Anlauf, Wurzeln und Starkästen. Die Platten blicken zum Himmel (waagerechte Platten)."),
    T("Transpiration emitters release the latent flux of their needle region as vapour from one anchor at the centre of every plate (seven upper, four lower plates); the direction is the vertical of the plate bone.",
      "Transpirationsemitter geben den latenten Strom ihrer Nadelregion als Dampf aus einem Anker in der Mitte jeder Platte ab (sieben obere, vier untere Platten); die Richtung ist die Senkrechte des Plattenknochens."),
]
STRUCTURE_ASSUMPTIONS = [
    T("One rigid body per evaluated pose, fixed at the root plate. Wind drag acts at the centre of every plate on its side silhouette (cd 0.4, reduced above 8 m/s) and as a line load on the bare lower stem; snow acts on the plan area of every plate. Stem and limbs are solid circles of green wood; the stem lean of the wind poses is the deflection of a tapered cantilever (E 8 GPa) on a root plate of 45 MN m per radian. Dynamic amplification, limb breakage and root-soil failure are not modelled.",
      "Ein starrer Körper je ausgewerteter Pose, am Wurzelteller eingespannt. Winddruck wirkt in der Mitte jeder Platte auf ihre Seitensilhouette (cd 0,4, über 8 m/s verringert) und als Linienlast am kahlen unteren Stamm; Schnee wirkt auf die Grundrissfläche jeder Platte. Stamm und Starkäste sind Vollkreise aus frischem Holz; die Stammneigung der Windposen ist die Durchbiegung eines verjüngten Kragarms (E 8 GPa) auf einem Wurzelteller von 45 MNm je Radiant. Dynamische Überhöhung, Astbruch und Wurzel-Boden-Versagen werden nicht modelliert."),
]


def rep_decisions(scale, dims):
    w = dims["widthM"]
    D = {
        "500": {"silhouette": ("At 1:500 (1 m is 2 mm) the cedar keeps its essence: a massive stem and eleven flat plates in tiers with sky between them, each plate one flat lens, the limbs as tapered lines to the plates; %s m wide." % fmt(w, False, 1), "Bei 1:500 (1 m sind 2 mm) behält die Zeder ihr Wesen: ein mächtiger Stamm und elf flache Platten in Etagen mit Himmel dazwischen, jede Platte eine flache Linse, die Starkäste als verjüngte Linien zu den Platten; %s m breit." % fmt(w, True, 1)),
                "retained": ("Retained: every plate at its true height, width and offset, every limb, the stem with its flare; the tiers are the identity of the species, so they are not merged into one crown.", "Beibehalten: jede Platte in wahrer Höhe, Breite und Lage, jeder Starkast, der Stamm mit Anlauf; die Etagen sind die Identität der Art und werden darum nicht zu einer Krone vereinigt."),
                "merged": ("Merged: the tufts of each plate into the plate (30 cm thicker than at 1:50); stem segments into a lower and an upper stem.", "Vereinigt: die Büschel jeder Platte in die Platte (30 cm dicker als bei 1:50); Stammabschnitte in einen unteren und einen oberen Stamm."),
                "omitted": ("Omitted: surface roots, cones, bark and needle texture.", "Weggelassen: Wurzelanläufe, Zapfen, Borken- und Nadeltextur."),
                "thinElements": ("Limbs end at least 12 cm thick, at their true path; nothing thinner than 0.2 mm is drawn.", "Starkäste enden mindestens 12 cm dick auf ihrem wahren Weg; nichts unter 0,2 mm wird gezeichnet."),
                "hatching": ("No hatching; outlines 0.15 mm.", "Keine Schraffur; Umrisse 0,15 mm."),
                "styles": ("Light line: plate lenses and limb lines. Dark solid: dark plates, lighter stem and limbs.", "Helle Linie: Plattenlinsen und Astlinien. Dunkle Fläche: dunkle Platten, hellerer Stamm und Starkäste."),
                "overlays": ("Thermal: plates in needles-sun or needles-shade, limbs limb-bark, stem bark-lower and bark-upper. Structure: stem, limb members and root support.", "Thermisch: Platten in needles-sun oder needles-shade, Starkäste limb-bark, Stamm bark-lower und bark-upper. Tragwerk: Stamm, Astglieder und Wurzelauflager.")},
        "200": {"silhouette": ("At 1:200 (1 m is 5 mm) eleven plates on their limbs, each a flat domed lens fitted to the plan box of its 1:50 plate, layered by depth; the stem in four parts.", "Bei 1:200 (1 m sind 5 mm) elf Platten auf ihren Starkästen, jede eine flache gewölbte Linse, an den Grundrisskasten ihrer 1:50-Platte angepasst, nach Tiefe geschichtet; der Stamm in vier Teilen."),
                "retained": ("Retained: plates, limbs, stem segments, leader.", "Beibehalten: Platten, Starkäste, Stammabschnitte, Gipfeltrieb."),
                "merged": ("Merged: the tufts into the plates (22 cm thicker); the root flare into the lower stem.", "Vereinigt: die Büschel in die Platten (22 cm dicker); der Wurzelanlauf in den unteren Stamm."),
                "omitted": ("Omitted: surface roots and texture.", "Weggelassen: Wurzelanläufe und Textur."),
                "thinElements": ("Limbs keep their true taper (ends at least 4 cm in radius).", "Starkäste behalten ihre wahre Verjüngung (Enden mindestens 4 cm Radius)."),
                "hatching": ("No hatching; outlines 0.18 mm.", "Keine Schraffur; Umrisse 0,18 mm."),
                "styles": ("Light line: layered plate outlines. Dark solid: lower plates darker.", "Helle Linie: geschichtete Plattenumrisse. Dunkle Fläche: untere Platten dunkler."),
                "overlays": ("Thermal and structure as at 1:500, per part.", "Thermisch und Tragwerk wie bei 1:500, je Teil.")},
        "100": {"silhouette": ("At 1:100 (1 m is 10 mm) each plate is a flat lens with one tuft rising from its upper face, so the plate tops are lumpy; limbs, root flare, stem and leader are drawn as they are modelled.", "Bei 1:100 (1 m sind 10 mm) ist jede Platte eine flache Linse mit einem Büschel auf ihrer Oberseite, so dass die Plattenoberseiten knollig sind; Starkäste, Wurzelanlauf, Stamm und Gipfeltrieb sind gezeichnet wie modelliert."),
                "retained": ("Retained: plates with one tuft each, limbs, flare, stem, leader and four pairs of foliage-layer strokes per plate.", "Beibehalten: Platten mit je einem Büschel, Starkäste, Anlauf, Stamm, Gipfeltrieb und vier Paare Laubschichtstriche je Platte."),
                "merged": ("Merged: the second tuft of the larger plates into the plate (12 cm thicker).", "Vereinigt: das zweite Büschel der größeren Platten in die Platte (12 cm dicker)."),
                "omitted": ("Omitted: surface roots, cones, bark texture.", "Weggelassen: Wurzelanläufe, Zapfen, Borkentextur."),
                "thinElements": ("Strokes stay 0.5 mm inside their plate and away from every other outline.", "Striche bleiben 0,5 mm innerhalb ihrer Platte und fern von jedem anderen Umriss."),
                "hatching": ("No hatch pattern; texture strokes of 0.1 mm.", "Kein Schraffurmuster; Texturstriche von 0,1 mm."),
                "styles": ("Light line: outlines and layer strokes. Dark solid: plates over lighter limbs.", "Helle Linie: Umrisse und Schichtstriche. Dunkle Fläche: Platten über helleren Starkästen."),
                "overlays": ("Thermal: plates and tufts in their needle region, limbs limb-bark, stem bark regions. Structure: stem and limb members.", "Thermisch: Platten und Büschel in ihrer Nadelregion, Starkäste limb-bark, Stamm Rindenregionen. Tragwerk: Stamm- und Astglieder.")},
        "50": {"silhouette": ("At 1:50 (1 m is 20 mm) every plate is a flat lens with one or two tufts on its upper face, carried by a tapered limb that leaves the stem at its own height and direction; six surface roots spread from the flare.", "Bei 1:50 (1 m sind 20 mm) ist jede Platte eine flache Linse mit einem oder zwei Büscheln auf ihrer Oberseite, getragen von einem verjüngten Starkast, der den Stamm in eigener Höhe und Richtung verlässt; sechs Wurzelanläufe spreizen sich vom Anlauf."),
               "retained": ("Retained: everything modelled at 1:50, plus foliage-layer dashes in every plate, bark plate strokes on the bare stem and radial needle strokes in plan.", "Beibehalten: alles bei 1:50 Modellierte, dazu Laubschichtstriche in jeder Platte, Borkenplattenstriche am kahlen Stamm und radiale Nadelstriche im Grundriss."),
               "merged": ("Merged: needle rosettes and short shoots into plates and tufts.", "Vereinigt: Nadelbüschel und Kurztriebe in Platten und Büschel."),
               "omitted": ("Omitted: the upright barrel-shaped cones (about 7 cm, they sit inside the plates).", "Weggelassen: die aufrechten tonnenförmigen Zapfen (etwa 7 cm, sie sitzen in den Platten)."),
               "thinElements": ("Limbs end at 2.5 cm radius at their true path; texture strokes keep 0.5 mm from every outline.", "Starkäste enden mit 2,5 cm Radius auf ihrem wahren Weg; Texturstriche halten 0,5 mm Abstand zu jedem Umriss."),
               "hatching": ("No hatch pattern; texture strokes of 0.13 mm.", "Kein Schraffurmuster; Texturstriche von 0,13 mm."),
               "styles": ("Light line: outlines, layer dashes and bark strokes. Dark solid: dark blue-green plates, grey limbs and stem.", "Helle Linie: Umrisse, Schichtstriche und Borkenstriche. Dunkle Fläche: dunkle blaugrüne Platten, graue Starkäste und Stamm."),
               "overlays": ("Thermal: every plate and tuft in its needle region, limbs limb-bark, flare, roots and lower stem bark-lower, upper stem and leader bark-upper. Structure: stem, root plate and every limb member, the support under the root plate.", "Thermisch: jede Platte und jedes Büschel in seiner Nadelregion, Starkäste limb-bark, Anlauf, Wurzeln und unterer Stamm bark-lower, oberer Stamm und Gipfeltrieb bark-upper. Tragwerk: Stamm, Wurzelteller und jedes Astglied, das Auflager unter dem Wurzelteller.")},
    }[scale]
    return D


def volume_decisions(scale, nbody):
    body = {"500": ("Body at 1:500 (75 mm): lower and upper stem, eleven limbs at least 12 cm thick and eleven plates of revolution fitted to their 1:50 plan box; %d solids." % nbody, "Körper bei 1:500 (75 mm): unterer und oberer Stamm, elf Starkäste mit mindestens 12 cm Dicke und elf Drehplatten, an ihren 1:50-Grundrisskasten angepasst; %d Körper." % nbody),
            "200": ("Body at 1:200 (30 mm): stem in four parts, limbs, fitted plates with the tufts merged in; %d solids." % nbody, "Körper bei 1:200 (30 mm): Stamm in vier Teilen, Starkäste, angepasste Platten mit eingeschmolzenen Büscheln; %d Körper." % nbody),
            "100": ("Body at 1:100 (15 mm): flare, stem, leader, limbs, plates and one tuft per plate; %d solids." % nbody, "Körper bei 1:100 (15 mm): Anlauf, Stamm, Gipfeltrieb, Starkäste, Platten und ein Büschel je Platte; %d Körper." % nbody),
            "50": ("Body at 1:50 (7.5 mm): flare with six surface roots, stem segments, leader, eleven tapered limbs, eleven plates of revolution with their tufts; %d solids." % nbody, "Körper bei 1:50 (7,5 mm): Anlauf mit sechs Wurzelanläufen, Stammabschnitte, Gipfeltrieb, elf verjüngte Starkäste, elf Drehplatten mit ihren Büscheln; %d Körper." % nbody)}[scale]
    return [T(*body), T("Thermal at 1:%s: every body solid by reference in its region; one transpiration outlet at the centre of every plate." % scale, "Thermisch bei 1:%s: jeder Körper per Verweis in seiner Region; ein Transpirationsauslass in der Mitte jeder Platte." % scale),
            T("Structure at 1:%s: root support disc, stem members at %d %% of the stem radius and a slender member along every limb to its plate." % (scale, {"500": 56, "200": 58, "100": 60, "50": 62}[scale]),
              "Tragwerk bei 1:%s: Wurzelauflagerscheibe, Stammglieder mit %d %% des Stammradius und ein schlankes Glied längs jedes Starkasts bis zu seiner Platte." % (scale, {"500": 56, "200": 58, "100": 60, "50": 62}[scale]))]


if __name__ == "__main__":
    doc, info = build()
    path = os.path.join(CATALOG, id, "🔣️.json")
    dump(path, doc)
    a = doc["asset"]
    row = {"id": id, "category": "vegetation", "title": a["title"], "heightM": a["dimensions"]["heightM"], "widthM": a["dimensions"]["widthM"], "powerW": 0, "poses": [p["id"] for p in a["poses"]],
           "clips": [c["id"] for c in a["clips"]], "representations": {k: a["representations"][k]["status"] for k in ["50", "100", "200", "500"]}, "symbols": [s["id"] for s in a["symbols"]],
           "analytics": ["thermal", "transport", "structural"], "exports": a["exports"],
           "distinction": T("A broad crown of eleven flat, horizontal needle plates in tiers on long ascending limbs, with sky between the plates: the widest and only plate-tiered conifer, drought-adapted, with the lowest stomatal conductance of the conifers.",
                            "Eine breite Krone aus elf flachen, waagerechten Nadelplatten in Etagen auf langen, aufsteigenden Starkästen, mit Himmel zwischen den Platten: der breiteste und einzige plattengestufte Nadelbaum, trockenheitsangepasst, mit der niedrigsten Spaltöffnungsleitfähigkeit der Nadelbäume."),
           "completion": "authored"}
    dump(os.path.join(TICKET, "📥️manifest-rows", "%s.json" % id), row)
    print(info)
    print(path)
