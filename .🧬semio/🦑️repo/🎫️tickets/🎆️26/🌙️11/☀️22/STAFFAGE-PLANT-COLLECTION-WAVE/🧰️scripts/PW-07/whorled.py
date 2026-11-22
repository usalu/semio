"""🌲️ Whorled conifer builder (Norway spruce, European larch): a straight stem in segments with one tier per branch storey. A tier is a skirt
of revolution about the stem on its own whorl bone (wind tilts it) plus branch sprays on two sagittal spray bones (west and east, snow bends
them) and, at 1:50, two lateral sprays on the whorl bone; every scale is designed anew from the species values (1:500 one crown in two
regions, 1:200 skirts, 1:100 skirts with sprays, 1:50 skirts, sprays, tips, branchlets and texture). Physics is shared by all scales.
@see ./conifer_lib.py
"""
import math
import os
import sys

from conifer_lib import (CATALOG, fit_lateral, fit_ring, G, RHO, T, TICKET, Capsule, Ellipsoid, Marks, Revolve, Rig, SCALES, bbox_union, cantilever_slopes, dump,
                         gap_report, inside, marks_shape, outline_polyline, path_d, r3, r4, representation, section, shape_of, simplify, slice_intervals,
                         slope_at, to_elev_local, union_width, v_at, wind_force)

FEATURE = {"500": 0.075, "200": 0.03, "100": 0.015, "50": 0.0075}
PENSUM = {"500": 0.15, "200": 0.18, "100": 0.2, "50": 0.25}


class Tier:
    pass


def tier_top(t, f, sag):
    g = (1 - sag) * f + sag * f * f
    return t.h + t.top - (t.top + t.D) * g


def skirt_profile(t, n, grow=1.0, drop=0.0, under=0.25):
    """Closed (y, r) profile of a tier skirt in world heights: the top surface from the stem outwards, the hanging rim, the underside back to the stem."""
    R = t.R * grow
    fs = [i / (n - 1) for i in range(n)]
    top = [(tier_top(t, f, t.sag) - drop * f, f * R) for f in fs]
    tip = (t.h - t.D - drop - t.rim, 0.97 * R)
    bottom = [(tier_top(t, f, t.sag) - drop * f - t.rim - under * 2 * math.sin(math.pi * f) * (1 - f), f * R * 0.985) for f in reversed(fs[1:-1])]
    bottom.append((t.h + t.top - t.rim - 0.5 * under, 0.0))
    return top + [tip] + bottom


def fmt(v, de=False, nd=1):
    s = ("%." + str(nd) + "f") % v
    return s.replace(".", ",") if de else s


def build(S):
    """Builds the asset document of a whorled conifer from its species module `S` (hand-chosen values)."""
    rig = Rig()
    stem, names, wood = S.stem_joints, S.stem_names, S.wood
    rig.add("root-plate", None, (0, 0, 0), 90, stem[1], T("Root plate and root flare", "Wurzelteller und Wurzelanlauf"),
            section=section(2 * S.r_stem(stem[1]), wood, "Root collar", "Wurzelhals"))
    for i in range(1, len(stem) - 1):
        y0, y1 = stem[i], stem[i + 1]
        bid = names[i - 1]
        parent = "root-plate" if i == 1 else names[i - 2]
        title = S.stem_titles[i - 1]
        rig.add(bid, parent, (0, y0, 0), 90, y1 - y0, title, limit=S.stem_limits[i - 1], section=section(2 * S.r_stem(y0), wood, title["en"], title["de"]))
    bones = rig.bones
    top_y = stem[-1]

    def stem_bone(y):
        for i in range(1, len(stem) - 1):
            if stem[i] <= y < stem[i + 1]:
                return names[i - 1]
        return names[-1]

    tiers = []
    for k, (h, R, D, top, rim, sag, ls, dx, dz, jit) in enumerate(S.tiers()):
        t = Tier()
        t.k, t.h, t.R, t.D, t.top, t.rim, t.sag, t.ls, t.dx, t.dz, t.jit = k + 1, h, R, D, top, rim, sag, ls, dx, dz, jit
        t.id = "tier-%02d" % t.k
        t.region = "needles-sun" if h >= S.sun_from else "needles-shade"
        t.material = S.needle_sun if t.region == "needles-sun" else S.needle_shade
        t.bone = rig.add("whorl-%02d" % t.k, stem_bone(h), (0, h, 0), 90, 0.6, T("Branch whorl %d at %.1f m" % (t.k, h), "Astquirl %d in %s m" % (t.k, fmt(h, True))), limit=S.whorl_limit)
        droop = S.spray_droop(t)
        L = 1.07 * R
        dbr = S.branch_d(t)
        t.west = rig.add("spray-w-%02d" % t.k, t.bone.id, (0, h, 0), 180 + droop, L, T("Branch spray west, whorl %d" % t.k, "Astzweig West, Quirl %d" % t.k), limit=S.spray_limit,
                         section=section(dbr, wood, "Branch base", "Astbasis"))
        t.east = rig.add("spray-e-%02d" % t.k, t.bone.id, (0, h, 0), -droop, L, T("Branch spray east, whorl %d" % t.k, "Astzweig Ost, Quirl %d" % t.k), limit=S.spray_limit,
                         section=section(dbr, wood, "Branch base", "Astbasis"))
        tiers.append(t)

    # ------------------------------------------------------------------ bodies per scale ----------------------------------------------
    def stem_profile(y0, y1, n, factor=1.0):
        ys = [y0 + (y1 - y0) * i / (n - 1) for i in range(n)]
        pts = [(y, factor * S.r_stem(y)) for y in ys]
        out = [(y0, 0.0)] + [p for p in pts if p[1] > 1e-6]
        if pts[-1][1] <= 1e-6:
            out.append((y1, 0.0))
        else:
            out.append((y1, 0.0))
        return out

    def stem_solids(scale):
        out = []
        if scale == "500":
            return [Revolve("stem", bones[names[0]], 0, 0, stem_profile(0.0, S.crown_base + 1.2, 6), S.bark_lower, 0.3, region="bark-lower")]
        if scale != "200":
            out.append(Revolve("root-flare", bones["root-plate"], 0, 0, stem_profile(0.0, stem[1] + 0.05, {"100": 6, "50": 10}[scale]), S.bark_lower, 0.3, region="bark-lower"))
        n = {"200": 3, "100": 4, "50": 6}[scale]
        for i in range(1, len(stem) - 1):
            bid = names[i - 1]
            last = i == len(stem) - 2
            y0 = 0.0 if (i == 1 and scale == "200") else stem[i]
            y1 = stem[i + 1] if last else stem[i + 1] + 0.04
            prof = stem_profile(y0, y1, n + (3 if last else 0) + (2 if (i == 1 and scale == "200") else 0))
            reg, mat = ("bark-lower", S.bark_lower) if i == 1 else ("bark-upper", S.bark_upper)
            out.append(Revolve("leader" if last else "stem-%d" % i, bones[bid], 0, 0, prof, mat, 0.32 if i == 1 else 0.36, region=reg))
        if scale == "50":
            for j, az in enumerate(S.root_azimuths):
                a = az * math.pi / 180
                L, r0 = S.root_len[j], S.root_r[j]
                p0 = (0.3 * math.cos(a), r0, 0.3 * math.sin(a))
                p1 = ((0.3 + L) * math.cos(a), 0.05, (0.3 + L) * math.sin(a))
                out.append(Capsule("root-%d" % (j + 1), bones["root-plate"], p0, p1, r0, 0.05, S.bark_lower, 0.28, region="bark-lower"))
        return out

    DIAG = {"ne": 45, "nw": 135, "sw": 225, "se": 315}

    def spray_points(t, side, reach, f0=0.5, extra=0.0):
        if side in DIAG:
            az = DIAG[side] + 0.6 * S.lateral_offset(t)
        else:
            az = (180 + t.jit) if side == "w" else (0 - t.jit) if side == "e" else (90 + S.lateral_offset(t)) if side == "n" else (-90 + S.lateral_offset(t))
        a = az * math.pi / 180
        y_in = tier_top(t, f0, t.sag) - 0.25 * t.rim - 0.08
        p0 = (t.dx + f0 * t.R * math.cos(a), y_in, t.dz + f0 * t.R * math.sin(a) * t.ls)
        p1 = (t.dx + reach * t.R * math.cos(a), t.h - t.D - S.spray_tip_drop(t) - extra, t.dz + reach * t.R * math.sin(a) * t.ls)
        return p0, p1, a

    def tier_solids(scale, ref=None):
        """The tiers of one scale; a coarse tier is fitted to `ref`, the plan box of the same tier at 1:50, so every scale has the same extent."""
        out = []
        if scale == "500":
            return out
        for t in tiers:
            r0, r1 = S.spray_r(t)
            if scale == "200":
                x0, x1, z0, z1 = ref[t.id]
                cx, cz, f, k = fit_ring(x0, x1, z0, z1, t.R)
                prof = skirt_profile(t, 5, grow=f, drop=0.1, under=0.15)
                out.append(Revolve(t.id, t.bone, cx, cz, prof, t.material, S.tone(t), region=t.region, ls=k))
                continue
            if scale == "100":
                x0, x1, z0, z1 = ref[t.id]
                cz, k = fit_lateral(z0, z1, 1.02 * t.R)
                prof = skirt_profile(t, 7, grow=1.02, drop=0.04, under=0.2)
                out.append(Revolve(t.id, t.bone, t.dx, cz, prof, t.material, S.tone(t), region=t.region, ls=k))
                for side, b in (("w", t.west), ("e", t.east)):
                    ra, rb = r0 * 1.05, r1 * 1.2
                    p0, p1, a = spray_points(t, side, 1.0, 0.45)
                    target = x0 + rb if side == "w" else x1 - rb
                    reach = (target - t.dx) / (t.R * math.cos(a))
                    p0, p1, a = spray_points(t, side, reach, 0.45)
                    out.append(Capsule("%s-spray-%s" % (t.id, side), b, p0, p1, ra, rb, t.material, S.tone(t) - 0.05, region=t.region))
                for side in S.hanging_sides(t, "100"):
                    p0, p1, a = spray_points(t, side, 0.92, 0.45, S.hang_drop(t))
                    out.append(Capsule("%s-spray-%s" % (t.id, side), t.bone, p0, p1, r0, r1 * 1.1, t.material, S.tone(t) - 0.02, region=t.region))
                continue
            prof = skirt_profile(t, 10, under=0.25)
            out.append(Revolve(t.id, t.bone, t.dx, t.dz, prof, t.material, S.tone(t), region=t.region, ls=t.ls))
            for side, b in (("w", t.west), ("e", t.east)):
                p0, p1, a = spray_points(t, side, 1.06)
                out.append(Capsule("%s-spray-%s" % (t.id, side), b, p0, p1, r0, r1, t.material, S.tone(t) - 0.05, region=t.region))
                tip = S.spray_tip(t, p1, a)
                if tip is not None:
                    q1, ra, rb = tip
                    out.append(Capsule("%s-tip-%s" % (t.id, side), b, p1, q1, ra, rb, t.material, S.tone(t) - 0.08, region=t.region))
                for j, (q0, q1, ra, rb) in enumerate(S.branchlets(t, p0, p1, a)):
                    out.append(Capsule("%s-hang-%s-%d" % (t.id, side, j + 1), b, q0, q1, ra, rb, t.material, S.tone(t) + 0.04, region=t.region))
            for side in ("n", "s"):
                p0, p1, a = spray_points(t, side, 1.04)
                out.append(Capsule("%s-spray-%s" % (t.id, side), t.bone, p0, p1, r0 * 0.95, r1, t.material, S.tone(t) - 0.03, region=t.region))
            for j, side in enumerate(S.hanging_sides(t, "50")):
                p0, p1, a = spray_points(t, side, 0.95, 0.5, S.hang_drop(t) * (0.75 + 0.18 * ((j + t.k) % 3)))
                out.append(Capsule("%s-spray-%s" % (t.id, side), t.bone, p0, p1, r0 * 0.8, r1 * 0.75, t.material, S.tone(t) - 0.02, region=t.region))
        return out

    fine50 = stem_solids("50") + tier_solids("50")
    lead50 = [s for s in fine50 if s.id == "leader"][0]
    crown50 = [s for s in fine50 if s.bone.id.startswith(("whorl", "spray"))]
    ref = {}
    for t in tiers:
        own = {t.bone.id, t.west.id, t.east.id}
        bt = bbox_union([s for s in crown50 if s.bone.id in own])
        ref[t.id] = (bt[0], bt[3], bt[2], bt[5])

    def crown500():
        """The 1:500 crown: two revolves (sunlit upper, shaded lower) whose radial envelope is the union of every 1:50 tier, spray, tip and
        branchlet and of the leader, centred and stretched so that the box is the box of the 1:50 crown."""
        parts = crown50 + [lead50]
        bb = bbox_union(parts)
        cx, cz = (bb[0] + bb[3]) / 2, (bb[2] + bb[5]) / 2
        Rx, Rz = (bb[3] - bb[0]) / 2, (bb[5] - bb[2]) / 2
        lo, hi = bb[1], bb[4]
        n = 500
        env = []
        for i in range(n + 1):
            y = hi - (hi - lo) * i / n
            iv = slice_intervals(parts, y)
            w = max([max(abs(a - cx), abs(b - cx)) for a, b in iv] + [0.0])
            if lo < y < hi and S.r_stem(y) > 0:
                w = max(w, S.r_stem(y) + abs(cx) + 0.06)
            env.append((y, w))
        env[0] = (hi, 0.0)
        env = [(y, w) for y, w in env if w > 1e-6 or y == hi]
        halves = [simplify(env, 0.03 + 0.005 * i) for i in range(9)]
        half = next((h for h in halves if len(h) <= 40), halves[-1])
        split = S.sun_from - 0.3
        above = [p for p in half if p[0] > split]
        below = [p for p in half if p[0] < split]
        w_split = None
        for (y0, w0), (y1, w1) in zip(half, half[1:]):
            if y1 <= split <= y0:
                w_split = w0 + (w1 - w0) * (split - y0) / (y1 - y0 + 1e-12)
        upper = above + [(split, w_split), (split, 0.0)]
        lower = [(split, 0.0), (split, w_split)] + below + [(below[-1][0], 0.0)]
        b2 = bones[names[1]]
        Rmax = max(w for _, w in half)
        fx, fz, f, k = fit_ring(bb[0], bb[3], bb[2], bb[5], Rmax)
        upper = [(y, w * f) for y, w in upper]
        lower = [(y, w * f) for y, w in lower]
        sun = Revolve("crown-sun", b2, fx, fz, upper, S.needle_sun, 0.55, region="needles-sun", ls=k)
        shade = Revolve("crown-shade", b2, fx, fz, lower, S.needle_shade, 0.65, region="needles-shade", ls=k)
        assert sun.N == shade.N or abs(max(w for _, w in upper) - max(w for _, w in lower)) > 0, (sun.N, shade.N)
        return [sun, shade]

    # ------------------------------------------------------------------ masses, dimensions, anchors ------------------------------------
    dens = S.green_density
    for bid in ["root-plate"] + names:
        b = bones[bid]
        kg = sum(s.volume() for s in fine50 if s.bone.id == bid and s.region and s.region.startswith("bark")) * dens
        if bid == "root-plate":
            kg += S.root_plate_extra_kg
        b.mass = {"kg": round(kg, 1), "comX": r3(b.L / 2), "comY": 0, "distribution": "uniform", "source": "authored-estimate", "reference": "wood-handbook-2021"}
    needle_area = {"needles-sun": S.needle_area_sun, "needles-shade": S.needle_area_shade}
    weight = {t.id: t.R ** 2 * (t.top + t.D + t.rim) for t in tiers}
    for reg in needle_area:
        tot = sum(weight[t.id] for t in tiers if t.region == reg)
        for t in tiers:
            if t.region == reg:
                t.needle_m2 = needle_area[reg] * weight[t.id] / tot
    for t in tiers:
        t.needle_kg = t.needle_m2 * S.needle_kg_m2
        t.branch_kg = S.branches_per_whorl * math.pi / 12 * 1.07 * t.R * S.branch_d(t) ** 2 * dens * S.twig_factor
        t.kg = t.needle_kg + t.branch_kg
        t.bone.mass = {"kg": round(0.6 * t.kg, 1), "comX": r3(0.3 * (t.top - t.D)), "comY": 0, "distribution": "point", "source": "authored-estimate", "reference": "jones-2014"}
        for b in (t.west, t.east):
            b.mass = {"kg": round(0.2 * t.kg, 1), "comX": r3(b.L * 0.45), "comY": 0, "distribution": "uniform", "source": "authored-estimate", "reference": "wood-handbook-2021"}
    mass = sum(b.mass["kg"] for b in bones.values())
    bb = bbox_union(fine50)
    dims = {"heightM": r3(bb[4]), "widthM": r3(bb[3] - bb[0]), "depthM": r3(bb[5] - bb[2]), "massKg": round(mass)}

    anchors = [{"id": "base-support", "bone": "root-plate", "x": 0, "y": 0, "kind": "support", "title": T("Root plate anchorage", "Verankerung des Wurzeltellers"), "support": "fixed",
                "normalDeg": 90, "footprintLengthM": S.footprint, "footprintWidthM": S.footprint, "frictionCoefficient": 0.6}]
    outlets = {"needles-sun": [], "needles-shade": []}
    for reg in outlets:
        cands = [t for t in tiers if t.region == reg]
        step = max(1, len(cands) // 6)
        pick = cands[::step][:6]
        while len(pick) < 5:
            pick = pick + cands[: 5 - len(pick)]
        for j, t in enumerate(pick):
            ang = (j * 137.5 + 40) * math.pi / 180
            rr = 0.42 * t.R
            wx, wz = t.dx + rr * math.cos(ang), t.dz + rr * math.sin(ang) * t.ls
            u = 0.25 * (t.top - t.D)
            word = "sun" if reg == "needles-sun" else "shade"
            aid = "tp-%s-%d" % (word, j + 1)
            anchors.append({"id": aid, "bone": t.bone.id, "x": r4(u), "y": r4(-wx), "z": r4(wz), "kind": "emitter",
                            "title": T("Transpiration of the %s needles, anchor %d" % ("sunlit" if word == "sun" else "shaded", j + 1),
                                       "Transpiration der %s Nadeln, Anker %d" % ("besonnten" if word == "sun" else "beschatteten", j + 1))})
            outlets[reg].append((aid, t, (wx, t.h + u, wz)))

    # ------------------------------------------------------------------ thermal model --------------------------------------------------
    crown_plan = math.pi * (dims["widthM"] / 2) * (dims["depthM"] / 2)
    regions = [
        {"id": "bark-lower", "title": S.region_titles["bark-lower"], "kind": "bark", "model": "passive", "material": S.bark_lower,
         "areaM2": round(sum(s.lateral_area() for s in fine50 if s.region == "bark-lower"), 2), "characteristicLengthM": round(2 * S.r_stem(1.3), 2), "geometry": "cylinder",
         "skyViewFactor": 0.35, "layers": [], "heatCapacityJM2K": S.bark_heat[0], "source": "authored-estimate", "reference": "wood-handbook-2021"},
        {"id": "bark-upper", "title": S.region_titles["bark-upper"], "kind": "bark", "model": "passive", "material": S.bark_upper,
         "areaM2": round(sum(s.lateral_area() for s in fine50 if s.region == "bark-upper"), 2), "characteristicLengthM": round(2 * S.r_stem(S.crown_base + 8), 2), "geometry": "cylinder",
         "skyViewFactor": S.bark_upper_sky, "layers": [], "heatCapacityJM2K": S.bark_heat[1], "source": "authored-estimate", "reference": "wood-handbook-2021"},
    ]
    for reg, area, gs, sky in (("needles-sun", S.needle_area_sun, S.gs_sun, 0.7), ("needles-shade", S.needle_area_shade, S.gs_shade, 0.3)):
        regions.append({"id": reg, "title": S.region_titles[reg], "kind": "foliage", "model": "leaf", "material": S.needle_sun if reg == "needles-sun" else S.needle_shade, "areaM2": area,
                        "areaBasis": "exchange", "characteristicLengthM": S.needle_width, "geometry": "cylinder", "skyViewFactor": sky, "layers": [], "heatCapacityJM2K": 1100,
                        "leaf": {"stomatalConductanceMaxMS": gs, "leafAreaIndex": round(area / crown_plan, 2), "photosynthesis": "c3"}, "source": "authored-estimate", "reference": "jones-2014"})
    emitters = []
    for reg, eid in (("needles-sun", "transpiration-sun"), ("needles-shade", "transpiration-shade")):
        ids = [a for a, _, _ in outlets[reg]]
        emitters.append({"id": eid, "kind": "transpiration", "title": S.emitter_titles[eid], "anchor": ids[0], "anchors": ids, "directionDeg": 0, "diameterM": S.emitter_d[eid],
                         "region": reg, "source": "authored-estimate", "reference": "monteith-unsworth-2013"})

    # ------------------------------------------------------------------ wind, poses, clips --------------------------------------------
    dy = 0.05
    ys = [S.crown_base - 2.0 + dy * i for i in range(int((top_y - S.crown_base + 2.0) / dy) + 1)]
    widths = [(y, union_width(slice_intervals(crown50, y))) for y in ys]
    bounds = [ys[0]] + [t.h - t.D - t.rim + 0.3 for t in tiers[1:]] + [top_y + 1]
    for i, t in enumerate(tiers):
        band = [(y, w) for y, w in widths if bounds[i] <= y < bounds[i + 1]]
        t.area = sum(w * dy for _, w in band)
        t.zc = sum(w * y * dy for y, w in band) / t.area if t.area > 0 else t.h
    crown_area = sum(t.area for t in tiers)

    def loads(v10):
        out = []
        for t in tiers:
            F = wind_force(t.area, t.zc, v10, cd=S.cd)
            out.append({"id": "wind-%s" % t.bone.id, "kind": "wind", "title": T("Wind drag on branch whorl %d" % t.k, "Winddruck auf Astquirl %d" % t.k), "bone": t.bone.id,
                        "x": r3(t.zc - t.h), "y": 0, "fxN": round(F, 1), "fyN": 0, "source": "literature", "reference": "en-1991-1-4"})
        q = 0.5 * RHO * 1.0 * 2 * S.r_stem(1.3) * v_at(v10, S.crown_base / 2) ** 2
        out.append({"id": "wind-stem", "kind": "wind", "title": T("Wind drag on the bare lower stem", "Winddruck auf den kahlen unteren Stamm"), "fxN": 0, "fyN": 0,
                    "distributed": {"bone": names[0], "fromM": 0, "toM": r3(S.crown_base - stem[1]), "wxNPerM": round(q, 2), "wyNPerM": 0}, "source": "literature", "reference": "en-1991-1-4"})
        return out

    def EI(y):
        d = 2 * max(S.r_stem(y), 0.03)
        return S.E_green * math.pi * d ** 4 / 64

    def lean(v10, extra):
        forces = [wind_force(t.area, t.zc, v10, cd=S.cd) for t in tiers]
        table = cantilever_slopes([t.zc for t in tiers], forces, EI, S.k_root, top_y)
        offs, prev = {}, 0.0
        for i in range(1, len(stem) - 1):
            sl = slope_at(table, (stem[i] + stem[i + 1]) / 2)
            offs[names[i - 1]] = -(sl - prev) * 180 / math.pi
            prev = sl
        for t in tiers:
            offs[t.bone.id] = -extra * S.whorl_lean * (0.6 + 0.4 * t.h / top_y)
            offs[t.west.id] = -extra * S.spray_lean
            offs[t.east.id] = -extra * S.spray_lean
        tip = sum(slope_at(table, dy * (i + 0.5)) * dy for i in range(int(top_y / dy)))
        return offs, sum(forces), sum(f * t.zc for f, t in zip(forces, tiers)), tip

    def rnd2(d):
        return {k: {"angleDeg": round(v, 2)} for k, v in d.items() if abs(round(v, 2)) > 0}

    _, calm_F, calm_M, _ = lean(S.v_calm, 0.0)
    windy_off, windy_F, windy_M, windy_tip = lean(S.v_windy, (S.v_windy / S.v_storm) ** 2)
    storm_off, storm_F, storm_M, storm_tip = lean(S.v_storm, 1.0)
    W = S.words

    def binding(sun, wet=None):
        out = {}
        for r in regions:
            d = {"exposedFraction": 1, "sunlitFraction": round(sun[r["id"]], 3)}
            if wet is not None:
                d["wettedness"] = wet
            out[r["id"]] = d
        return out

    def kn(v):
        return v / 1000.0

    poses = [
        {"id": "calm", "title": T("Calm", "Windstille"),
         "description": T("Light air of %.0f m/s at 10 m: the stem stands vertical and the %d tiers hang at rest; the crown silhouette of %.0f m2 takes %.0f N of drag in all." % (S.v_calm, len(tiers), crown_area, calm_F),
                          "Leiser Zug von %s m/s in 10 m Höhe: Der Stamm steht senkrecht, die %d Etagen hängen in Ruhe; die Kronensilhouette von %s m2 nimmt insgesamt %s N Winddruck auf." % (fmt(S.v_calm, True, 0), len(tiers), fmt(crown_area, True, 0), fmt(calm_F, True, 0))),
         "bones": {}, "ground": "authored", "contacts": [], "thermal": {"regions": binding(S.sun), "emitters": {"transpiration-sun": 1, "transpiration-shade": 1}}, "loads": loads(S.v_calm)},
        {"id": "windy", "title": T("Fresh breeze", "Frische Brise"),
         "description": T("Beaufort 5, %.0f m/s at 10 m with the power-law profile (exponent 0.22) and cd %.2f on the silhouette of each tier with the reconfiguration reduction above 8 m/s: %.1f kN on the crown and a base moment of %.0f kN m. The stem bends as a tapered cantilever of green %s (E %.1f GPa) on a root plate of %.0f MN m per radian: the tip moves %.2f m downwind." % (S.v_windy, S.cd, kn(windy_F), kn(windy_M), W["wood"][0], S.E_green / 1e9, S.k_root / 1e6, windy_tip),
                          "Beaufort 5, %s m/s in 10 m Höhe mit Potenzprofil (Exponent 0,22) und cd %s auf die Silhouette jeder Etage mit der Formanpassung über 8 m/s: %s kN auf die Krone und ein Fußmoment von %s kNm. Der Stamm biegt sich als verjüngter Kragarm aus frischem %s (E %s GPa) auf einem Wurzelteller von %s MNm je Radiant: Die Spitze weicht %s m nach Lee aus." % (fmt(S.v_windy, True, 0), fmt(S.cd, True, 2), fmt(kn(windy_F), True), fmt(kn(windy_M), True, 0), W["wood"][1], fmt(S.E_green / 1e9, True), fmt(S.k_root / 1e6, True, 0), fmt(windy_tip, True, 2))),
         "bones": rnd2(windy_off), "ground": "authored", "contacts": [], "thermal": {"regions": binding(S.sun), "emitters": {"transpiration-sun": 1, "transpiration-shade": 1}}, "loads": loads(S.v_windy)},
        {"id": "storm", "title": T("Gale", "Sturm"),
         "description": T("Beaufort 9, %.0f m/s at 10 m: %.1f kN on the crown and a base moment of %.0f kN m (Mayhead 1973 order of magnitude for conifers); the stem tip moves %.2f m downwind, the tiers tilt to the lee and the sprays stream with the wind. Rain wets every surface." % (S.v_storm, kn(storm_F), kn(storm_M), storm_tip),
                          "Beaufort 9, %s m/s in 10 m Höhe: %s kN auf die Krone und ein Fußmoment von %s kNm (Größenordnung nach Mayhead 1973 für Nadelbäume); die Stammspitze weicht %s m nach Lee aus, die Etagen kippen nach Lee und die Zweige legen sich in den Wind. Regen benetzt alle Oberflächen." % (fmt(S.v_storm, True, 0), fmt(kn(storm_F), True), fmt(kn(storm_M), True, 0), fmt(storm_tip, True, 2))),
         "bones": rnd2(storm_off), "ground": "authored", "contacts": [], "thermal": {"regions": binding({k: v * 0.35 for k, v in S.sun.items()}, 0.4), "emitters": {"transpiration-sun": 0.5, "transpiration-shade": 0.4}},
         "loads": loads(S.v_storm)},
    ]
    poses.extend(S.extra_poses(tiers, binding, loads, fmt))

    P = S.sway_period
    depth = {}
    for bid in rig.order:
        b = bones[bid]
        depth[bid] = 0 if b.parent is None else depth[b.parent] + 1
    keys = []
    for i in range(9):
        bo = {}
        for bid, base in windy_off.items():
            amp = S.sway_amp(bid, base)
            bo[bid] = {"angleDeg": round(base + amp * math.sin(2 * math.pi * (i % 8) / 8 - 0.35 * depth[bid]), 2)}
        keys.append({"t": round(P * i / 8, 4), "pose": "windy", "ease": "smooth", "bones": bo})
    keys[-1]["bones"] = {k: dict(v) for k, v in keys[0]["bones"].items()}
    sway = {"id": "wind-sway", "title": T("Wind sway", "Windschwingung"),
            "description": T("A deterministic %.1f s sway cycle about the fresh-breeze lean (first bending mode of a %.0f m conifer, about %.2f Hz, within the range reviewed by Moore and Maguire 2004): the stem bends gently, the tiers and sprays lag behind with growing phase towards their tips." % (P, top_y, 1 / P),
                             "Ein deterministischer Schwingungszyklus von %s s um die Neigung der frischen Brise (erste Biegeeigenform eines %s m hohen Nadelbaums, etwa %s Hz, im von Moore und Maguire 2004 zusammengestellten Bereich): Der Stamm biegt sich sanft, Etagen und Zweige hinken mit zur Spitze wachsender Phase nach." % (fmt(P, True), fmt(top_y, True, 0), fmt(1 / P, True, 2))),
            "durationS": P, "fps": 30, "loop": True, "rootMotion": "none", "keys": keys, "contacts": [], "phases": [{"id": "sway", "fromS": 0, "toS": P, "title": T("Sway cycle", "Schwingungszyklus")}],
            "events": [{"t": 0, "kind": "motion", "title": T("Cycle start at the mean breeze lean", "Zyklusbeginn in der mittleren Windneigung")}],
            "tracks": [{"target": "emitter:%s" % e, "ease": "smooth", "keys": [{"t": 0, "v": 1}, {"t": round(P / 2, 3), "v": 0.92}, {"t": P, "v": 1}]} for e in ("transpiration-sun", "transpiration-shade")]}

    def gust_shape(t_):
        return math.exp(-((t_ - 2.5) / 0.9) ** 2) if t_ < 2.5 else math.exp(-((t_ - 2.5) / 1.8) ** 2)
    gkeys = []
    for t_ in [0, 1, 1.8, 2.5, 3.2, 4.2, 5.5, 8]:
        f = 0.7 * gust_shape(t_)
        gkeys.append({"t": t_, "pose": "windy", "ease": "smooth", "bones": {b: {"angleDeg": round(windy_off[b] + (storm_off[b] - windy_off[b]) * f, 2)} for b in storm_off}})
    gust = {"id": "gust", "title": T("Gust", "Windböe"),
            "description": T("A single 8 s gust on the fresh breeze: the wind-load tracks rise from 5 %% to 2.75 times the breeze load at 2.5 s (EN 1991-1-4 peak velocity pressure with a turbulence intensity of 0.25) and decay; the stem and tiers lean up to 70 %% of the way to the gale lean.",
                             "Eine einzelne Böe von 8 s auf der frischen Brise: Die Windlastspuren steigen von 5 %% auf das 2,75-fache der Brisenlast bei 2,5 s (Spitzengeschwindigkeitsdruck nach EN 1991-1-4 mit einer Turbulenzintensität von 0,25) und klingen ab; Stamm und Etagen neigen sich bis zu 70 %% des Wegs zur Sturmneigung."),
            "durationS": 8, "fps": 30, "loop": False, "rootMotion": "none", "keys": gkeys, "contacts": [],
            "phases": [{"id": "rise", "fromS": 0, "toS": 2.5, "title": T("Gust front", "Böenfront")}, {"id": "peak", "fromS": 2.5, "toS": 3.6, "title": T("Peak", "Spitze")},
                       {"id": "decay", "fromS": 3.6, "toS": 8, "title": T("Decay and recovery", "Abklingen und Erholung")}],
            "events": [{"t": 2.5, "kind": "load-on", "title": T("Peak gust load", "Böenspitze")}, {"t": 5, "kind": "load-off", "title": T("Load back near the mean", "Last wieder nahe dem Mittelwert")}],
            "tracks": [{"target": "load:" + ld["id"], "ease": "smooth", "keys": [{"t": t_, "v": round(0.05 + 2.7 * gust_shape(t_), 3)} for t_ in [0, 1.2, 2.5, 3.6, 5, 8]]} for ld in loads(S.v_windy)]}
    sun_day = [(0, 0.0), (5, 0.0), (6, 0.05), (8, 0.43), (10, 0.79), (12, 1.0), (14, 0.79), (16, 0.36), (18, 0.0), (24, 0.0)]
    tr_day = S.transpiration_day
    dtracks = [{"target": "emitter:%s" % e, "ease": "smooth", "keys": [{"t": t_, "v": v} for t_, v in tr_day]} for e in ("transpiration-sun", "transpiration-shade")]
    dtracks += [{"target": "region:%s:sunlitFraction" % r["id"], "ease": "smooth", "keys": [{"t": t_, "v": round(v * S.sun[r["id"]], 3)} for t_, v in sun_day]} for r in regions]
    diurnal = {"id": "diurnal-transpiration", "title": T("Day of transpiration", "Transpirationstag"),
               "description": T("A 24 h summer day compressed to 24 s (1 s = 1 h, 00:00 to 24:00) in calm weather: the sunlit fraction of every region and the transpiration strength follow the sun, stomata nearly closed at night and %s; needle and bark temperatures are not authored but follow from the thermal solver." % W["midday"][0],
                                "Ein Sommertag von 24 h auf 24 s gerafft (1 s = 1 h, 00:00 bis 24:00) bei Windstille: Der besonnte Anteil jeder Region und die Transpirationsstärke folgen der Sonne, nachts sind die Stomata fast geschlossen und %s; Nadel- und Rindentemperaturen werden nicht vorgegeben, sondern folgen aus dem thermischen Löser." % W["midday"][1]),
               "durationS": 24, "fps": 10, "loop": True, "rootMotion": "none", "keys": [{"t": 0, "pose": "calm", "ease": "smooth"}, {"t": 12, "pose": "calm", "ease": "smooth"}, {"t": 24, "pose": "calm", "ease": "smooth"}],
               "contacts": [], "phases": [{"id": "night", "fromS": 0, "toS": 6, "title": T("Night", "Nacht")}, {"id": "morning", "fromS": 6, "toS": 12, "title": T("Morning to noon", "Morgen bis Mittag")},
                                          {"id": "afternoon", "fromS": 12, "toS": 18, "title": T("Afternoon", "Nachmittag")}, {"id": "evening", "fromS": 18, "toS": 24, "title": T("Evening and night", "Abend und Nacht")}],
               "events": [{"t": 6, "kind": "thermal", "title": T("Sunrise", "Sonnenaufgang")}, {"t": 12, "kind": "thermal", "title": T("Solar noon, peak transpiration", "Sonnenhöchststand, maximale Transpiration")},
                          {"t": 18, "kind": "thermal", "title": T("Sunset", "Sonnenuntergang")}], "tracks": dtracks}
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

    # ------------------------------------------------------------------ representations and volumes ----------------------------------
    reps, vols, report = {}, {}, {}
    crown_parts = crown500()
    for scale in SCALES:
        body = stem_solids(scale) + (crown_parts if scale == "500" else tier_solids(scale, ref))
        shapes = {"elevation": [], "front": [], "plan": []}
        for view in shapes:
            for i, s in enumerate(body):
                z = 1.0 if (s.bone.id in names or s.bone.id == "root-plate") else 3.0 - 0.001 * i
                sh = shape_of(s, view, round(z, 3))
                if s.id == "crown-sun":
                    sh.pop("region", None)
                    sh["regions"] = ["needles-sun", "bark-upper"]
                shapes[view].append(sh)
            shapes[view].extend(texture(S, scale, view, body, tiers, bones, names))
        reps[scale] = representation(scale, shapes, S.rep_decisions(scale, len(tiers), dims, fmt))
        th = [{"id": "th-%s" % s.id, "of": s.id, "region": s.region} for s in body]
        if scale == "500":
            core = Revolve("bark-upper-core", bones[names[1]], 0, 0, stem_profile(S.crown_base + 1.0, top_y, 6), S.bark_upper, 0.36).json()
            th.append(dict(core, region="bark-upper"))
        for reg, eid in (("needles-sun", "transpiration-sun"), ("needles-shade", "transpiration-shade")):
            for j, (aid, t, p) in enumerate(outlets[reg]):
                ro = {"500": 0.3, "200": 0.26, "100": 0.24, "50": 0.22}[scale]
                e = Ellipsoid("outlet-%s-%d" % (eid.split("-")[1], j + 1), t.bone, p, (ro, 0.7 * ro, ro), t.material, 0.5).json()
                e["role"], e["tags"], e["region"] = "detail", [eid], reg
                th.append(e)
        fac = {"500": 0.56, "200": 0.58, "100": 0.6, "50": 0.62}[scale]
        nst = {"500": 3, "200": 4, "100": 5, "50": 7}[scale]
        rp = bones["root-plate"]
        st = [dict(Revolve("support-base", rp, 0, 0, [(0, 0), (0, 0.9), (0.08, 0.9), (0.08, 0)], wood, 0.25).json(), role="detail", member="root-plate", part="support", anchor="base-support"),
              dict(Revolve("member-root-plate", rp, 0, 0, stem_profile(0.02, stem[1] + 0.05, nst, factor=fac), wood, 0.4).json(), member="root-plate", part="member")]
        for i in range(1, len(stem) - 1):
            bid = names[i - 1]
            last = i == len(stem) - 2
            y0, y1 = stem[i] - 0.03, (stem[i + 1] - 0.4 if last else stem[i + 1] + 0.05)
            m = Revolve("member-%s" % bid, bones[bid], 0, 0, stem_profile(y0, y1, nst, factor=fac), wood, 0.4)
            m.slender = (y1 - y0) / (2 * m.R) >= 5.5
            st.append(dict(m.json(), member=bid, part="member"))
        rad = {"500": 0.036, "200": 0.034, "100": 0.032, "50": 0.03}[scale]
        rings = {view: [outline_polyline(s, view, 48) for s in body] for view in ("elevation", "front", "plan")}
        for t in tiers:
            for b, sign in ((t.west, -1), (t.east, 1)):
                c = spray_member(t, b, sign, rings, rad, rad if scale == "500" else 0.02, wood)
                st.append(dict(c.json(), member=b.id, part="member"))
        vols[scale] = {"status": "authored", "denominator": int(scale), "body": [s.json() for s in body], "thermal": th, "structure": st,
                       "decisions": S.volume_decisions(scale, len(body), len(tiers), fmt)}
        report[scale] = {"solids": len(body), "gaps": {v: gap_report(body, v, scale, PENSUM[scale]) for v in ("elevation", "front", "plan")}}

    asset = {"id": S.id, "category": "vegetation", "title": S.title, "summary": S.summary, "tags": S.tags,
             "provenance": {"author": "Semio staffage plant worker PW-07", "method": "handcrafted", "created": "2026-11-22", "references": S.references, "notes": S.notes},
             "dimensions": dims, "materials": S.materials, "rig": {"kind": "botanical", "root": "root-plate", "bones": rig.json()}, "volumes": vols, "anchors": anchors,
             "representations": reps, "symbols": [symbol(S, crown_parts, fine50, dims)], "poses": poses, "clips": clips,
             "thermal": {"environment": S.environment, "regions": regions, "emitters": emitters, "assumptions": S.thermal_assumptions, "operating": [], "passengers": []},
             "structure": {"gravityMS2": G, "dynamics": "quasi-static", "supports": ["base-support"], "assumptions": S.structure_assumptions, "clinical": False},
             "presentation": {"defaultPose": "calm", "defaultClip": "wind-sway", "defaultScale": 200},
             "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"], "completion": "authored"}
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}
    info = {"dims": dims, "crown_area": round(crown_area, 1), "calm": (round(calm_F), round(calm_M)), "windy": (round(windy_F), round(windy_M), round(windy_tip, 3)),
            "storm": (round(storm_F), round(storm_M), round(storm_tip, 3)), "report": report, "bones": len(rig.order),
            "tiers": [(t.id, round(t.h, 2), round(t.R, 2), round(t.area, 1), round(t.kg, 1)) for t in tiers]}
    return doc, info


def covered(capsule, rings, margin):
    """Whether the three projections of a capsule lie inside the union of the body outlines of its scale, with `margin` metres to spare."""
    for view, own in rings.items():
        _, (x0, y0, x1, y1, r0, r1) = capsule.outline_world(view)
        for i in range(13):
            f = i / 12
            cx, cy, r = x0 + (x1 - x0) * f, y0 + (y1 - y0) * f, r0 + (r1 - r0) * f + margin
            for j in range(8):
                pt = (cx + r * math.cos(math.pi * j / 4), cy + r * math.sin(math.pi * j / 4))
                if not any(inside(pt, ring) for ring in own):
                    return False
    return True


def spray_member(t, b, sign, rings, r0, r1, wood):
    """The structural member of a spray bone: a slender capsule from the stem axis into the tier, laid inside the body of its scale (the branch
    runs within the needle skirt, not along the straight bone under it); the highest start and the longest reach that stay covered are taken."""
    starts = [t.h + t.top - 0.5 * t.rim - d for d in (0.0, 0.04, 0.08, 0.12, 0.18, 0.25)] + [t.h + 0.5 * t.top, t.h + 0.25 * t.top, t.h]
    for reach in (0.9, 0.85, 0.8, 0.75, 0.7, 0.6, 0.5, 0.4, 0.3):
        for ya in starts:
            for depth in (0.5, 0.35, 0.65, 0.2, 0.8):
                yb = tier_top(t, reach, t.sag) - depth * t.rim
                c = Capsule("member-%s" % b.id, b, (0.0, ya, 0.0), (t.dx + sign * reach * t.R, yb, t.dz), r0, r1, wood, 0.4, slender=True)
                if covered(c, rings, 0.004):
                    return c
    raise AssertionError("no covered member for %s" % b.id)


def texture(S, scale, view, body, tiers, bones, names):
    """Texture strokes of one view: branchlet hatching inside every tier skirt (1:50, 1:100), bark plates on the bare stem (1:50) and radial
    needle strokes in the plan rings (1:50); a stroke stays 0.5 mm of paper inside its host and away from every other outline."""
    if scale in ("500", "200"):
        return []
    den = int(scale)
    outlines = {s.id: outline_polyline(s, view, 48) for s in body}
    M = Marks(view, outlines, 0.5 * den / 1000)
    strokes = {}
    by_id = {s.id: s for s in body}
    for t in reversed(tiers):
        s = by_id[t.id]
        if view in ("elevation", "front"):
            c = t.dx if view == "elevation" else t.dz
            k = 1.0 if view == "elevation" else t.ls
            n = {"100": 5, "50": 11}[scale]
            for i in range(n):
                f = -0.86 + 1.72 * (i + 0.5 * (t.k % 2)) / n
                x = c + f * t.R * k
                y0 = tier_top(t, abs(f), t.sag) - 0.18 - 0.1 * t.rim
                L = S.mark_len(t) * (1.0 if scale == "50" else 1.3)
                bend = 0.04 * (1 if f > 0 else -1)
                st = [(x, y0), (x + bend, y0 - 0.5 * L), (x + 1.6 * bend, y0 - L)]
                if M.offer(t.id, t.id, st):
                    strokes.setdefault(t.id, []).append(st)
        elif scale == "50":
            for j in range(14):
                a = 2 * math.pi * (j + 0.37 * t.k) / 14
                r0, r1 = 0.62 * t.R, 0.86 * t.R
                st = [(t.dx + r0 * math.cos(a), t.dz + r0 * math.sin(a) * t.ls), (t.dx + r1 * math.cos(a + 0.05), t.dz + r1 * math.sin(a + 0.05) * t.ls)]
                if M.offer(t.id, t.id, st):
                    strokes.setdefault(t.id, []).append(st)
    if scale == "50" and view in ("elevation", "front"):
        host = "stem-1"
        s = by_id[host]
        for i in range(18):
            y = 0.9 + (S.crown_base - 1.6) * (i % 6) / 6 + 0.15 * (i // 6)
            x = -0.15 + 0.15 * (i // 6)
            st = [(x, y), (x + 0.015, y + 0.22), (x - 0.01, y + 0.42)]
            if M.offer(host, host, st):
                strokes.setdefault(host, []).append(st)
    out = []
    for host, ss in strokes.items():
        s = by_id[host]
        out.append(marks_shape("%s-texture" % host, s.bone, view, ss, 3.0, s.tone, region=s.region, material=s.material))
    return out


def symbol(S, crown_parts, fine50, dims):
    """The plan symbol at true extent: the crown outline of the 1:500 crown and the stem section in all three views."""
    sun, shade = crown_parts
    rp_shapes = {}
    env = sun.env + shade.env
    env = sorted(env, key=lambda p: -p[0])
    oy = sun.bone.o[1]
    right = [(sun.cx + r, oy + a) for a, r in env]
    left = [(sun.cx - r, oy + a) for a, r in reversed(env)]
    ring_e = simplify(right, 0.05) + simplify(left, 0.05)
    right_f = [(sun.cz + r * sun.ls, oy + a) for a, r in env]
    left_f = [(sun.cz - r * sun.ls, oy + a) for a, r in reversed(env)]
    ring_f = simplify(right_f, 0.05) + simplify(left_f, 0.05)
    R = max(r for _, r in env)
    ring_p = [(sun.cx + R * math.cos(2 * math.pi * i / 48), sun.cz + R * sun.ls * math.sin(2 * math.pi * i / 48)) for i in range(48)]
    r0, rb = S.r_stem(0.0), S.r_stem(S.crown_base)
    stem_e = [(-r0, 0), (-rb, S.crown_base), (rb, S.crown_base), (r0, 0)]
    rs = S.r_stem(1.3)

    def poly(id, pts, filled, tone):
        return {"id": id, "bone": "root-plate", "role": "body", "z": 2 if not filled else 3, "filled": filled, "tone": tone, "kind": "poly", "points": [[r4(x), r4(y)] for x, y in pts], "closed": True}
    geo = {"elevation": [poly("crown", ring_e, False, 0.5), poly("stem", stem_e, True, 0.3)],
           "front": [poly("crown", ring_f, False, 0.5), poly("stem", stem_e, True, 0.3)],
           "plan": [poly("crown", ring_p, False, 0.5), {"id": "stem", "bone": "root-plate", "role": "body", "z": 3, "filled": True, "tone": 0.3, "kind": "ellipse", "cx": 0, "cy": 0, "rx": r4(rs), "ry": r4(rs)}]}
    W = S.words
    return {"id": "%s-symbol" % S.id, "title": T("Plan symbol with true crown extent", "Grundrisssymbol mit wahrer Kronenausdehnung"),
            "description": T("A conventional sign of the %s for site and landscape plans: the outline of the true crown extent and the stem section. It is a separate symbol and is never used silently for a pictorial scale." % W["name"][0],
                             "Ein konventionelles Zeichen der %s für Lage- und Freiraumpläne: der Umriss der wahren Kronenausdehnung und der Stammquerschnitt. Es ist ein eigenes Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet." % W["name"][1]),
            "semantics": "true-extent", "units": "model-m", "denominators": [500, 200, 100], "pens": {"body": 0.18, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.25}, "hatches": [],
            "geometry": geo,
            "dimensionalNote": T("Drawn at true extent in model metres in all three views (%.2f m wide, %.2f m deep and %.2f m high): the outline is the crown, the stem is drawn at its real position and thickness at breast height in plan. It is a symbol, never used silently for a pictorial scale." % (dims["widthM"], dims["depthM"], dims["heightM"]),
                                 "In wahrer Ausdehnung in Modellmetern in allen drei Ansichten gezeichnet (%s m breit, %s m tief und %s m hoch): Der Umriss ist die Krone, der Stamm liegt in wirklicher Lage und im Grundriss in seiner Stärke auf Brusthöhe. Es ist ein Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet." % (fmt(dims["widthM"], True, 2), fmt(dims["depthM"], True, 2), fmt(dims["heightM"], True, 2)))}


def write(S, doc):
    """Writes the catalog file and the manifest row, or with `--candidate` on the command line only `🗑️generated/PW-07/<id>/🔣️.json`."""
    if "--candidate" in sys.argv:
        path = os.path.join(TICKET, "🗑️generated", "PW-07", S.id, "🔣️.json")
        os.makedirs(os.path.dirname(path), exist_ok=True)
        dump(path, doc)
        return path
    path = os.path.join(CATALOG, S.id, "🔣️.json")
    dump(path, doc)
    a = doc["asset"]
    row = {"id": a["id"], "category": "vegetation", "title": a["title"], "heightM": a["dimensions"]["heightM"], "widthM": a["dimensions"]["widthM"], "powerW": 0,
           "poses": [p["id"] for p in a["poses"] if not p.get("key")], "clips": [c["id"] for c in a["clips"]],
           "representations": {k: a["representations"][k]["status"] for k in ["50", "100", "200", "500"]}, "symbols": [s["id"] for s in a["symbols"]],
           "analytics": ["thermal", "transport", "structural"], "exports": a["exports"], "distinction": S.distinction, "completion": "authored"}
    dump(os.path.join(TICKET, "📥️manifest-rows", "%s.json" % a["id"]), row)
    return path
