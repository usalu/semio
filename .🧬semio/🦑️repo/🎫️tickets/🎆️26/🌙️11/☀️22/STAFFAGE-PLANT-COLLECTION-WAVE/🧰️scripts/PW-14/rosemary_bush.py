"""🌿 Rosemary (Salvia rosmarinus), an upright 1.2 m evergreen shrub of nine woody stems in spring flower: species data, rig, four scales of drawing and volume, poses, clips, thermal and structure.

Run: `repo tools exec python -- rosemary_bush.py` (writes the catalog asset and the manifest row).
@see plant_lib.py, shrub.py
"""
import math
from plant_lib import (T, r4, Rig, revolute, FIXED, mass, round_section, Ell, Cap, Rev, Mark, Scale, pens, decisions, member_capsule, member_loft, support_box, feature, overrides,
                       vadd, vsub, vscale, vlen, lerp3, leaf_region, passive_region, transpiration)
from shrub import wind_at, drag, region_bindings, load, pose, gust_clip, sway_clip, diurnal_clip, up_direction, assemble, outline_symbol

ID = "rosemary-bush"
REFS = [
    ("drew-2017", "Drew, B. T., Gonzalez-Gallegos, J. G., Xiang, C.-L., Kriebel, R., Drummond, C. P., Walker, J. B. and Sytsma, K. J. (2017). Salvia united: The greatest good for the greatest number. Taxon 66(1), 133-145 (Rosmarinus officinalis L. transferred to Salvia rosmarinus Spenn.)."),
    ("flora-europaea-1972", "Tutin, T. G. et al. (eds.) (1972). Flora Europaea, Vol. 3. Cambridge University Press. Rosmarinus officinalis: erect or decumbent shrub to 2 m; leaves linear, 10-40 mm by 1.2-3.5 mm, revolute, white-tomentose beneath; corolla 10-12 mm, pale blue."),
    ("wood-handbook-2021", "Forest Products Laboratory (2021). Wood Handbook: Wood as an Engineering Material. General Technical Report FPL-GTR-282, USDA Forest Service. Chapter 5 (green density, modulus of elasticity and rupture; closest listed hardwood used as proxy)."),
    ("en-1991-1-4", "EN 1991-1-4:2005 Eurocode 1, Actions on structures, Part 1-4: Wind actions. Air density 1.25 kg/m3, F = 0.5 rho cd A v^2, peak velocity pressure (1 + 7 Iv) q_m."),
    ("en-1991-1-3", "EN 1991-1-3:2003 Eurocode 1, Actions on structures, Part 1-3: Snow loads. Used for the order of magnitude of wet snow held by the upright shoots."),
    ("vogel-1989", "Vogel, S. (1989). Drag and reconfiguration of broad leaves in high winds. Journal of Experimental Botany 40(8), 941-948."),
    ("jones-2014", "Jones, H. G. (2014). Plants and Microclimate, 3rd ed., Cambridge University Press. Leaf energy balance, leaf area index, boundary-layer and stomatal conductance."),
    ("monteith-unsworth-2013", "Monteith, J. L. and Unsworth, M. H. (2013). Principles of Environmental Physics, 4th ed., Academic Press. Penman-Monteith leaf transpiration."),
]
LEAF, FLOWER, BARK = "leaf-shoots", "flower-clusters", "stem-bark"
M_LEAF, M_FLOWER, M_BARK, M_WOOD = "rosemary-leaf", "rosemary-flower", "rosemary-bark", "rosemary-wood"
KEYS = ["c"] + [str(i) for i in range(1, 9)]
JIT = [1.0, 0.96, 1.04, 0.98, 1.03, 0.95, 1.02, 0.99, 1.05]


def stems():
    """🪵 The nine stems in world rest metres: base, lower woody stem and leafy shoot as true 3D directions (azimuth from +x towards +z, lean from the vertical)."""
    out = {}
    for i, k in enumerate(KEYS):
        if k == "c":
            a, base, p1, l1, p2, l2 = 0, (0, 0, 0), 3, 0.40, 2, 0.745
        else:
            a = 22.5 + 45 * (i - 1)
            base = (0.03 * math.cos(math.radians(a)), 0, 0.03 * math.sin(math.radians(a)))
            p1, l1, p2, l2 = 15 * JIT[i], 0.38 * JIT[(i + 2) % 9], 17.5 * JIT[(i + 4) % 9], 0.76 * JIT[(i + 6) % 9]
        d1 = (math.sin(math.radians(p1)) * math.cos(math.radians(a)), math.cos(math.radians(p1)), math.sin(math.radians(p1)) * math.sin(math.radians(a)))
        d2 = (math.sin(math.radians(p2)) * math.cos(math.radians(a)), math.cos(math.radians(p2)), math.sin(math.radians(p2)) * math.sin(math.radians(a)))
        top1 = vadd(base, vscale(d1, l1))
        top2 = vadd(top1, vscale(d2, l2))
        out[k] = {"a": a, "base": base, "d1": d1, "d2": d2, "l1": l1, "l2": l2, "top1": top1, "top2": top2}
    return out


S = stems()


def rig():
    r = Rig()
    r.add("root-crown", None, T("Woody root crown", "Verholzter Wurzelhals"), (-0.05, 0, 0), 0, 0.1, FIXED, mass(1.2, 0.05, "authored-estimate", "wood-handbook-2021"), round_section(0.07, M_WOOD, "Solid section, d = 7 cm", "Vollquerschnitt, d = 7 cm"))
    for k in KEYS:
        s = S[k]
        en, de = ("central", "mittlerer") if k == "c" else (f"no. {k}", f"Nr. {k}")
        for bone, o, d, l, joint, m, sec, t in (
            (f"stem-{k}", s["base"], s["d1"], s["l1"], revolute(-3, 3), mass(0.35, 0.19, "authored-estimate", "wood-handbook-2021"), round_section(0.028, M_WOOD, "Solid section, d = 28 mm", "Vollquerschnitt, d = 28 mm"),
             T(f"Woody stem {en}", f"Verholzter Trieb {de}") if k != "c" else T("Central woody stem", "Mittlerer verholzter Trieb")),
            (f"shoot-{k}", s["top1"], s["d2"], s["l2"], revolute(-12, 12), mass(0.45, 0.4, "authored-estimate", "jones-2014", "point"), round_section(0.013, M_WOOD, "Solid section, d = 13 mm", "Vollquerschnitt, d = 13 mm"),
             T(f"Leafy shoot {en}", f"Beblätterter Spross {de}") if k != "c" else T("Central leafy shoot", "Mittlerer beblätterter Spross"))):
            h = math.hypot(d[0], d[1])
            r.add(bone, "root-crown" if bone.startswith("stem") else f"stem-{k}", t, o, math.degrees(math.atan2(d[1], d[0])), l * h, joint, m, sec)
    return r


def phi_of(d):
    return math.degrees(math.atan2(d[1], d[0]))


def on_shoot(k, t):
    s = S[k]
    return vadd(s["top1"], vscale(s["d2"], s["l2"] * t))


def on_stem(k, t):
    s = S[k]
    return vadd(s["base"], vscale(s["d1"], s["l1"] * t))


LOBES = {
    50: [(0.1, 0.19, 0.12), (0.3, 0.23, 0.165), (0.52, 0.23, 0.17), (0.72, 0.2, 0.15), (0.88, 0.12, 0.1)],
    100: [(0.15, 0.23, 0.14), (0.42, 0.25, 0.165), (0.68, 0.21, 0.15), (0.87, 0.13, 0.1)],
    200: [(0.28, 0.33, 0.17), (0.72, 0.28, 0.15)],
}


def scale_model(r, den, hk=1.0):
    sc = Scale(den, r)
    tl, tf, tb = 0.6, 0.22, 0.32
    if den == 500:
        prof = crown_profile(r)
        sc.add(Rev("crown", "shoot-c", (0, 0, 0), prof, lateral=r4(DEPTH_RATIO[0]), material=M_LEAF, role="body", tone=tl, regions=[LEAF, FLOWER]))
        sc.add(Cap("trunk", "stem-c", (0, 0.04, 0), (0, 0.32, 0), 0.04, 0.04, material=M_BARK, role="body", tone=tb, region=BARK))
        for k in KEYS[1:]:
            s = S[k]
            sc.add(Cap(f"stem-{k}", f"stem-{k}", vadd(s["base"], (0, 0.01, 0)), lerp3(s["top1"], s["top2"], 0.6), 0.009, 0.009, material=M_BARK, role="body", tone=tb, region=BARK, slender=True, stroke=True))
        return sc
    sc.add(Rev("root-collar", "root-crown", (0, 0, 0), [(0, 0), (0, 0.055), (0.04, 0.05), (0.07, 0.035), (0.08, 0)], material=M_BARK, role="body", tone=tb, region=BARK, z=0.5))
    rs = {50: 0.014, 100: 0.0145, 200: 0.016}[den]
    hosts = []
    for k in KEYS:
        s = S[k]
        sc.add(Cap(f"stem-{k}", f"stem-{k}", (s["base"][0], rs, s["base"][2]), s["top1"], rs, rs * 0.8, material=M_BARK, role="body", tone=tb, region=BARK, z=0.6))
        sc.add(Cap(f"shoot-axis-{k}", f"shoot-{k}", s["top1"], on_shoot(k, 0.92), 0.0065, 0.0035 if den == 50 else 0.0045, material=M_BARK, role="detail", tone=tb, region=BARK, slender=True, stroke=True, z=0.7))
        phi = phi_of(s["d2"])
        for j, (t, along, horiz) in enumerate(LOBES[den]):
            f = 1.0 if k == "c" else JIT[(int(k) + j) % 9]
            h = Ell(f"lobe-{k}-{j + 1}", f"shoot-{k}", on_shoot(k, t), (along * f, horiz * hk, horiz * hk * (0.98 if j % 2 else 1.0)), phi=phi, material=M_LEAF, role="body", tone=tl, region=LEAF, z=1 + 0.01 * j)
            sc.add(h)
            hosts.append((h, k, t))
        if k != "c" and den in (50, 100):
            sc.add(Ell(f"lobe-{k}-0", f"stem-{k}", on_stem(k, 0.8), (0.11, 0.105 if den == 50 else 0.115, 0.105 if den == 50 else 0.115), phi=phi_of(s["d1"]), material=M_LEAF, role="body", tone=tl + 0.03, region=LEAF, z=0.9))
        fans = {50: [-15, 0, 15], 100: [-10, 10], 200: [0]}[den]
        size = {50: (0.09, 0.026), 100: (0.09, 0.028), 200: (0.095, 0.036)}[den]
        top = on_shoot(k, 0.93)
        for j, fa in enumerate(fans):
            ph = phi + fa
            dvec = (math.cos(math.radians(ph)), math.sin(math.radians(ph)), 0)
            c = vadd(top, vscale(dvec, size[0] * 0.55))
            if k == "c":
                c = (c[0], min(c[1], 1.2 - size[0]), c[2])
            sc.add(Ell(f"tip-{k}-{j + 1}", f"shoot-{k}", c, (size[0], size[1], size[1]), phi=ph, material=M_LEAF, role="body", tone=tl + 0.06, region=LEAF, z=1.5))
        flowers = {50: [0.42, 0.58, 0.72, 0.86], 100: [0.5, 0.78], 200: [0.62]}[den] if k != "c" else {50: [0.5, 0.7, 0.85], 100: [0.6], 200: []}[den]
        fr = {50: 0.0125, 100: 0.012, 200: 0.016}[den]
        for j, t in enumerate(flowers):
            lobe_h = horiz_at(den, t) * hk
            side = (35 if j % 2 else -35) + (0 if k != "c" else 120 * j)
            az = math.radians(s["a"] + side)
            axis = on_shoot(k, t)
            c = (axis[0] + (lobe_h * 0.86) * math.cos(az), axis[1] + 0.01 * (j % 2), axis[2] + (lobe_h * 0.86) * math.sin(az))
            sc.add(Ell(f"flower-{k}-{j + 1}", f"shoot-{k}", c, (fr, fr * 0.85, fr), material=M_FLOWER, role="body", tone=tf, region=FLOWER, z=2))
    if den in (50, 100):
        needles(sc, hosts, den)
    return sc


def horiz_at(den, t):
    best = min(LOBES[den], key=lambda q: abs(q[0] - t))
    return best[2]


def needles(sc, hosts, den):
    """🌲 Needle marks: two or three short dashes per lobe slanting up and out like the linear rosemary leaves on their shoot (side and front), short ticks in plan."""
    per = 3 if den == 50 else 2
    length = 0.05 if den == 50 else 0.07
    m = 0
    for h, k, t in hosts:
        a, b, c = h.r
        for j in range(per):
            f = (j + 0.5) / per - 0.5
            for view in ("elevation", "front", "plan"):
                if view == "elevation":
                    cx, cy, rx, ry = h.c[0], h.c[1], b, a
                elif view == "front":
                    cx, cy, rx, ry = h.c[2], h.c[1], c, a
                else:
                    cx, cy, rx, ry = h.c[0], h.c[2], b, c
                if view == "plan":
                    ang = math.radians(S[k]["a"] + 120 * j + 40)
                    p = (cx + 0.4 * rx * math.cos(ang), cy + 0.4 * ry * math.sin(ang))
                    q = (p[0] + 0.6 * length * math.cos(ang), p[1] + 0.6 * length * math.sin(ang))
                else:
                    p = (cx + f * 1.1 * rx, cy - 0.2 * ry + 0.25 * ry * j / per)
                    q = (p[0] + length * 0.5 * (1 if f >= 0 else -1), p[1] + length * 0.85)
                m += 1
                sc.mark(Mark(f"needle-mark-{m:03d}", h.bone, view, [p, q], h, role="texture"))


DEPTH_RATIO = [1.0]


def crown_profile(r):
    """🏺 The 1:500 crown: at every height the widest horizontal reach of the 1:50 lobes about the axis, so the revolved crown is the envelope of the modelled shoots."""
    lobes = [p for p in scale_model(r, 50).prims if isinstance(p, Ell) and p.region == LEAF]
    xs = max(p.extent(r)[1][0] for p in lobes) - min(p.extent(r)[0][0] for p in lobes)
    zs = max(p.extent(r)[1][2] for p in lobes) - min(p.extent(r)[0][2] for p in lobes)
    DEPTH_RATIO[0] = zs / xs
    bottom = min(p.extent(r)[0][1] for p in lobes)
    top = max(p.extent(r)[1][1] for p in lobes)
    prof = [(r4(bottom + 0.01), 0)]
    n = 14
    for i in range(n + 1):
        h = bottom + 0.01 + (top - 0.01 - bottom - 0.01) * i / n
        best = 0.0
        for p in lobes:
            a, b, c = p.r
            ph = math.radians(p.phi)
            ey = math.sqrt((a * math.sin(ph)) ** 2 + (b * math.cos(ph)) ** 2)
            if abs(h - p.c[1]) >= ey:
                continue
            k = math.sqrt(1 - ((h - p.c[1]) / ey) ** 2)
            ex = math.sqrt((a * math.cos(ph)) ** 2 + (b * math.sin(ph)) ** 2) * k
            best = max(best, abs(p.c[0]) + ex)
        prof.append((r4(h), r4(max(best, 0.02))))
    prof.append((r4(top), 0))
    return prof


def regions(bark_area):
    return [
        {**leaf_region(LEAF, T("Needle-like evergreen leaves of the shoots", "Nadelartige immergrüne Blätter der Sprosse"), M_LEAF, 2.4, 0.003, "cylinder", 0.5, 800, 0.005, 4.3, "c3", "flora-europaea-1972"), "areaBasis": "exchange"},
        {**leaf_region(FLOWER, T("Pale blue flower clusters in the leaf axils", "Hellblaue Blütenbüschel in den Blattachseln"), M_FLOWER, 0.05, 0.01, "sphere", 0.7, 500, 0.001, 0.09, "c3", "flora-europaea-1972"), "areaBasis": "exchange"},
        passive_region(BARK, T("Grey shredding bark of the woody stems", "Graue, faserig abblätternde Rinde der verholzten Triebe"), "bark", M_BARK, bark_area, 0.028, "cylinder", 0.1, 19000, "wood-handbook-2021"),
    ]


def anchors(r):
    out = [{"id": "base-support", "bone": "root-crown", "x": 0.05, "y": 0, "kind": "support", "title": T("Planting hole anchorage", "Verankerung im Pflanzloch"), "support": "fixed", "normalDeg": 90, "footprintLengthM": 0.4, "footprintWidthM": 0.4, "frictionCoefficient": 0.6}]
    for k in KEYS:
        x, y, z = r.local(f"shoot-{k}", on_shoot(k, 0.5))
        out.append({"id": f"tp-shoot-{k}", "bone": f"shoot-{k}", "x": r4(x), "y": r4(y), **({"z": r4(z)} if abs(z) > 1e-9 else {}), "kind": "emitter", "title": T(f"Transpiration of shoot {k}" if k != "c" else "Transpiration of the central shoot", f"Transpiration des Sprosses {k}" if k != "c" else "Transpiration des mittleren Sprosses")})
    for k in KEYS[1:]:
        x, y, z = r.local(f"shoot-{k}", on_shoot(k, 0.7))
        out.append({"id": f"tp-flower-{k}", "bone": f"shoot-{k}", "x": r4(x), "y": r4(y), **({"z": r4(z)} if abs(z) > 1e-9 else {}), "kind": "emitter", "title": T(f"Transpiration of the flowers on shoot {k}", f"Transpiration der Blüten am Spross {k}")})
    return out


def structure(r, d):
    """🏗️ Members along the true stem and shoot lines; joints at the stem forks and point masses of the shoots at 1:50, joints at 1:100, members only at 1:200, and at 1:500 one column for root crown, central stem and shoot."""
    f = feature(d)
    out = [support_box(r, "root-crown", "base-support", (0.16 if d == 500 else 0.14, f, 0.16 if d == 500 else 0.14), (0, -f / 2, 0), M_WOOD)]
    if d == 500:
        out.append(member_capsule(r, "member-centre", "stem-c", (0, 0.04, 0), on_shoot("c", 0.95), 0.04, M_WOOD, members=["root-crown", "shoot-c"]))
    else:
        out.append(member_capsule(r, "member-root-crown", "root-crown", (-0.02, 0.03, 0), (0.02, 0.03, 0), max(0.025, 0.55 * f), M_WOOD))
    for k in KEYS:
        if d == 500 and k == "c":
            continue
        s = S[k]
        rs = 0.014 if d < 200 else (0.016 if d == 200 else 0.012)
        out.append(member_capsule(r, f"member-stem-{k}", f"stem-{k}", (s["base"][0], rs, s["base"][2]), s["top1"], rs, M_WOOD, slender=d == 500))
        out.append(member_capsule(r, f"member-shoot-{k}", f"shoot-{k}", s["top1"], on_shoot(k, 0.9), 0.0065, M_WOOD, slender=True))
    if d in (50, 100):
        for k in KEYS:
            out.append({"id": f"joint-shoot-{k}", "kind": "ellipsoid", "bone": f"shoot-{k}", "material": M_WOOD, "role": "detail", "tone": 0.3, "radii": [0.012] * 3, "transform": {"translation": [r4(x) for x in r.local(f"shoot-{k}", S[k]["top1"])]}, "member": f"shoot-{k}", "part": "joint"})
    if d == 50:
        for k in KEYS:
            out.append({"id": f"mass-shoot-{k}", "kind": "ellipsoid", "bone": f"shoot-{k}", "material": M_WOOD, "role": "detail", "tone": 0.3, "radii": [0.02] * 3, "transform": {"translation": [r4(x) for x in r.local(f"shoot-{k}", on_shoot(k, 0.45))]}, "member": f"shoot-{k}", "part": "mass"})
    return out


def poses(r):
    rg = [LEAF, FLOWER, BARK]
    sun = {LEAF: 0.5, FLOWER: 0.7, BARK: 0.1}
    em = ["transpiration-shoots", "transpiration-flowers"]
    area = 0.09

    def winds(v10, cd, src, ref, calm=False):
        out = []
        for k in KEYS:
            mid = on_shoot(k, 0.5)
            v = 1.5 if calm else wind_at(v10, mid[1])
            x = r4(r.W[f"shoot-{k}"][2] * 0.5)
            out.append(load(f"wind-shoot-{k}", "wind", T(f"Wind drag on shoot {k}" if k != "c" else "Wind drag on the central shoot", f"Winddruck auf Spross {k}" if k != "c" else "Winddruck auf den mittleren Spross"), f"shoot-{k}", x, 0, drag(cd, area, v), 0, src, ref))
        return out

    def lean(stem, shoot):
        b = {}
        for k in KEYS:
            b[f"stem-{k}"] = {"angleDeg": stem}
            b[f"shoot-{k}"] = {"angleDeg": shoot}
        return b

    calm = pose("calm", T("Calm spring morning in flower", "Windstiller Frühlingsmorgen in Blüte"),
                T("Light air of 1.5 m/s: the nine upright shoots stand still, studded with pale blue flowers; the drag on a shoot is below 0.15 N.", "Leiser Zug von 1,5 m/s: die neun aufrechten Sprosse stehen still, mit hellblauen Blüten besetzt; der Widerstand auf einem Spross liegt unter 0,15 N."),
                {}, region_bindings(rg, sun), {e: 0.8 for e in em}, winds(0, 0.9, "literature", "en-1991-1-4", calm=True))
    windy = pose("windy", T("Fresh breeze", "Frische Brise"),
                 T("Beaufort 5, 8 m/s at 10 m, 4.4 to 4.8 m/s at the shoots; cd 0.7 on 0.09 m2 per shoot gives about 0.8 N each. The woody shoots lean 4 degrees downwind, the stems 1 degree.",
                   "Beaufort 5, 8 m/s in 10 m Höhe, 4,4 bis 4,8 m/s an den Sprossen; cd 0,7 auf 0,09 m2 je Spross ergibt etwa 0,8 N je Spross. Die verholzten Sprosse neigen sich um 4 Grad nach Lee, die Triebe um 1 Grad."),
                 lean(-1, -4), region_bindings(rg, sun), {e: 1 for e in em}, winds(8, 0.7, "literature", "en-1991-1-4"))
    storm = pose("storm", T("Gale", "Sturm"),
                 T("Beaufort 9, 22 m/s at 10 m (12 to 13 m/s at the shoots), cd 0.5 after reconfiguration of the needle sprays: about 4.5 N per shoot, the shoots lay over by 9 degrees and the base moment reaches about 40 N m.",
                   "Beaufort 9, 22 m/s in 10 m Höhe (12 bis 13 m/s an den Sprossen), cd 0,5 nach Umformung der Nadelzweige: etwa 4,5 N je Spross, die Sprosse legen sich um 9 Grad um und das Fußmoment erreicht etwa 40 N m."),
                 lean(-2, -9), region_bindings(rg, {LEAF: 0.25, FLOWER: 0.3, BARK: 0}, {LEAF: 0.4, FLOWER: 0.4, BARK: 0.3}), {e: 0.5 for e in em}, winds(22, 0.5, "literature", "vogel-1989"))
    splay = {}
    for k in KEYS:
        c = 0 if k == "c" else math.cos(math.radians(S[k]["a"]))
        splay[f"shoot-{k}"] = {"angleDeg": r4(-8 * c)}
        splay[f"stem-{k}"] = {"angleDeg": r4(-1.5 * c)}
    snow_loads = [load(f"snow-shoot-{k}", "snow", T(f"Wet snow on shoot {k}" if k != "c" else "Wet snow on the central shoot", f"Nassschnee auf Spross {k}" if k != "c" else "Nassschnee auf dem mittleren Spross"), f"shoot-{k}", r4(r.W[f"shoot-{k}"][2] * 0.7), 0, 0, -24, "authored-estimate", "en-1991-1-3") for k in KEYS]
    snow = pose("snow", T("Wet snow splaying the shoots", "Nassschnee spreizt die Sprosse"),
                T("Wet snow of 0.4 kN/m2 on the 0.55 m2 crown plan, 24 N per shoot (EN 1991-1-3 order of magnitude, authored estimate), opens the upright crown: the shoots splay outwards by up to 8 degrees, the stems by 1.5 degrees; the rig turns them in the side plane only, so the splay shows fully in the side elevation and in the forward and rearward shoots. Calm.",
                  "Nassschnee von 0,4 kN/m2 auf der 0,55 m2 großen Kronengrundfläche, 24 N je Spross (Größenordnung nach EN 1991-1-3, Schätzung), öffnet die aufrechte Krone: die Sprosse spreizen sich um bis zu 8 Grad nach außen, die Triebe um 1,5 Grad; das Rig dreht sie nur in der Seitenebene, daher zeigt sich die Spreizung voll in der Seitenansicht und an den nach vorn und hinten weisenden Sprossen. Windstille."),
                splay, region_bindings(rg, {LEAF: 0.05, FLOWER: 0, BARK: 0}, {LEAF: 0.6, FLOWER: 0.6, BARK: 0.4}), {e: 0.05 for e in em}, snow_loads)
    return [calm, windy, storm, snow]


def clips(r):
    base, specs, peak = {}, {}, {}
    for i, k in enumerate(KEYS):
        base.update({f"stem-{k}": -1, f"shoot-{k}": -4})
        ph = 0 if k == "c" else S[k]["a"] * 0.8
        specs[f"stem-{k}"] = (0.35, ph, 1)
        specs[f"shoot-{k}"] = (1.6, ph + 25, 1)
        peak.update({f"stem-{k}": -1.6, f"shoot-{k}": -7})
    sway = sway_clip(r, 1.0, 12, base, specs, T("A deterministic 1 s cycle about the fresh-breeze lean (stiff woody shoots, estimated 1 Hz): every shoot rocks by 1.6 degrees with a phase that travels round the crown, the stems by 0.35 degrees.",
                                                "Ein deterministischer Zyklus von 1 s um die Neigung der frischen Brise (steife verholzte Sprosse, geschätzt 1 Hz): jeder Spross wiegt sich um 1,6 Grad mit einer um die Krone wandernden Phase, die Triebe um 0,35 Grad."),
                     ["transpiration-shoots", "transpiration-flowers"])
    gust = gust_clip(None, [{"id": f"wind-shoot-{k}"} for k in KEYS], peak, T("Gust", "Windböe"),
                     T("A single 8 s gust: the wind-load tracks rise from 5 % to 2.75 times the fresh-breeze load at 2.5 s (EN 1991-1-4 peak pressure with Iv 0.25) and decay; the shoots lean up to 7 degrees.",
                       "Eine einzelne Böe über 8 s: die Windlast-Spuren steigen bei 2,5 s von 5 % auf das 2,75-fache der Last der frischen Brise (Spitzendruck nach EN 1991-1-4 mit Iv 0,25) und klingen ab; die Sprosse neigen sich bis zu 7 Grad."))
    day = diurnal_clip({LEAF: 0.5, FLOWER: 0.7, BARK: 0.1}, ["transpiration-shoots", "transpiration-flowers"],
                       T("A 24 h spring cycle compressed to 24 s (1 s = 1 h), calm weather. The sunlit fraction of every region and the transpiration strength follow the sun; leaf, flower and bark temperatures follow from the thermal solver.",
                         "Ein 24-h-Frühlingszyklus auf 24 s gerafft (1 s = 1 h) bei ruhigem Wetter. Der besonnte Anteil jeder Region und die Transpirationsstärke folgen der Sonne; Blatt-, Blüten- und Rindentemperaturen folgen aus dem thermischen Löser."))
    return [sway, gust, day]


def extra_thermal(r):
    return {500: [{"id": "th-flower-share", "kind": "ellipsoid", "bone": "shoot-c", "material": M_FLOWER, "role": "detail", "tone": 0.3, "radii": [0.3, 0.15, 0.28], "transform": {"translation": [r4(x) for x in r.local("shoot-c", (0, 0.86, 0))]}, "region": FLOWER}]}


def calibrated(r, den, target):
    """📏 The horizontal lobe factor of a coarse scale found by bisection so that its width equals the 1:50 width (the coarse lobes are redrawn, their envelope keeps the true extent)."""
    lo, hi = 0.8, 1.2
    for _ in range(30):
        mid = (lo + hi) / 2
        a, b = scale_model(r, den, mid).extent()
        if (b[0] - a[0]) + (b[2] - a[2]) > target:
            hi = mid
        else:
            lo = mid
    print(f"[DEBUG] 1:{den} horizontal factor {lo:.4f}")
    return scale_model(r, den, lo)


def build():
    r = rig()
    s50 = scale_model(r, 50)
    a, b = s50.extent()
    target = (b[0] - a[0]) + (b[2] - a[2])
    scales = {50: s50, 100: calibrated(r, 100, target), 200: calibrated(r, 200, target), 500: scale_model(r, 500)}
    bark = overrides(ID).get(BARK, 0.6)
    an = anchors(r)
    emitter_anchors = [(f"tp-shoot-{k}", LEAF, "transpiration-shoots", M_LEAF) for k in KEYS] + [(f"tp-flower-{k}", FLOWER, "transpiration-flowers", M_FLOWER) for k in KEYS[1:]]
    spec = {
        "id": ID, "rig": r, "scales": scales, "anchors": an, "emitter_anchors": emitter_anchors, "emitter_radius": {50: 0.02, 100: 0.024, 200: 0.03, 500: 0.045},
        "extra_thermal": extra_thermal(r), "structure": lambda d: structure(r, d), "volume_notes": VOLUME_NOTES, "drawn": (50, 100, 200, 500),
        "ranges": {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)},
        "pens": {50: pens(0.2, 0.13, 0.1, None, 0.3), 100: pens(0.18, 0.13, 0.1, None, 0.25), 200: pens(0.15, 0.13, None, None, 0.25), 500: pens(0.15, 0.13, None, None, 0.25)},
        "decisions": REP_DECISIONS, "title": T("Rosemary", "Rosmarin"),
        "summary": lambda dims: T(f"An upright rosemary (Salvia rosmarinus) in spring flower: nine grey, shredding woody stems rise from a short root crown and carry dense columns of dark, needle-like leaves to {dims['heightM']:.2f} m; the crown is {dims['widthM']:.2f} m across, studded with pale blue flower clusters, with the bare woody legs of an older plant visible below 0.2 m. The evergreen kitchen herb and hedge plant of Mediterranean gardens.",
                                  f"Ein aufrechter Rosmarin (Salvia rosmarinus) in Frühlingsblüte: neun graue, faserig abblätternde verholzte Triebe steigen aus einem kurzen Wurzelhals auf und tragen dichte Säulen dunkler, nadelartiger Blätter bis {dims['heightM']:.2f} m; die Krone ist {dims['widthM']:.2f} m breit und mit hellblauen Blütenbüscheln besetzt, unter 0,2 m sind die kahlen verholzten Beine einer älteren Pflanze zu sehen. Das immergrüne Küchenkraut und die Heckenpflanze mediterraner Gärten."),
        "tags": ["shrub", "evergreen", "herb", "flowering", "mediterranean", "upright", "wind", "transpiration"],
        "refs": REFS,
        "notes": T("Nine stems, each a woody lower stem and a leafy shoot, fan out from the root crown; each shoot carries a column of foliage lobes, a fan of upright shoot tips and the flower clusters. The rig is planar: shoots that lean sideways sway and splay only in the side plane. A pruned or winter-bare geometry is not offered because rosemary is evergreen and shapes cannot be toggled by a pose.",
                   "Neun Triebe, jeder aus einem verholzten unteren Trieb und einem beblätterten Spross, fächern sich aus dem Wurzelhals auf; jeder Spross trägt eine Säule aus Laublappen, einen Fächer aufrechter Triebspitzen und die Blütenbüschel. Das Rig ist eben: seitlich geneigte Sprosse schwingen und spreizen sich nur in der Seitenebene. Eine geschnittene oder winterkahle Geometrie wird nicht angeboten, da Rosmarin immergrün ist und Formen durch eine Pose nicht ausgeblendet werden können."),
        "materials": [
            {"id": M_LEAF, "title": T("Dark green revolute rosemary leaves", "Dunkelgrüne, eingerollte Rosmarinblätter"), "emissivity": 0.96, "solarAbsorptivity": 0.68, "source": "authored-estimate", "reference": "jones-2014"},
            {"id": M_FLOWER, "title": T("Pale blue rosemary flowers", "Hellblaue Rosmarinblüten"), "emissivity": 0.95, "solarAbsorptivity": 0.55, "source": "authored-estimate", "reference": "jones-2014"},
            {"id": M_BARK, "title": T("Grey shredding rosemary bark", "Graue, faserige Rosmarinrinde"), "emissivity": 0.93, "solarAbsorptivity": 0.65, "densityKgM3": 600, "conductivityWmK": 0.12, "source": "authored-estimate", "reference": "wood-handbook-2021"},
            {"id": M_WOOD, "title": T("Green rosemary wood", "Frisches Rosmarinholz"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 950, "conductivityWmK": 0.17, "elasticModulusPa": 7000000000, "strengthPa": 60000000, "source": "authored-estimate", "reference": "wood-handbook-2021"},
        ],
        "poses": poses(r), "clips": clips(r),
        "environment": {"airTempC": 18, "meanRadiantTempC": 22, "relativeHumidity": 0.55, "windMS": 1.5, "windDirectionDeg": 0, "solarWM2": 750, "pressurePa": 101325, "diffuseFraction": 0.2},
        "regions": regions(bark),
        "emitters": [transpiration("transpiration-shoots", T("Transpiration of the leafy shoots (distributed over the crown)", "Transpiration der beblätterten Sprosse (über die Krone verteilt)"), [f"tp-shoot-{k}" for k in KEYS], LEAF, up_direction(r, "shoot-c"), 0.3),
                     transpiration("transpiration-flowers", T("Transpiration of the flower clusters (distributed over the outer shoots)", "Transpiration der Blütenbüschel (über die äußeren Sprosse verteilt)"), [f"tp-flower-{k}" for k in KEYS[1:]], FLOWER, up_direction(r, "shoot-1"), 0.2)],
        "structure_text": ("One rigid body per evaluated pose, fixed at the planting hole; wind drag acts at mid-length of each shoot, wet snow at seven tenths of each shoot, gravity on all bone masses. The base moment stays below 0.1 kN m even in a gale. Root and soil failure, branch breakage and dynamic amplification are not modelled.",
                           "Ein starrer Körper je ausgewerteter Pose, im Pflanzloch eingespannt; Winddruck wirkt in der halben Länge jedes Sprosses, Nassschnee auf sieben Zehnteln jedes Sprosses, Schwerkraft auf allen Knochenmassen. Das Fußmoment bleibt selbst im Sturm unter 0,1 kN m. Wurzel- und Bodenversagen, Astbruch und dynamische Überhöhung werden nicht modelliert."),
        "symbols": lambda dims: [outline_symbol(r, "root-crown", dims, [(-0.12, 0), (-0.12, 0.17), (-0.93, 0.2), (-1, 0.45), (-0.95, 0.75), (-0.8, 0.98), (-0.55, 1.1), (-0.3, 1.17), (-0.1, 1.2), (0.1, 1.19), (0.32, 1.15), (0.55, 1.1), (0.8, 0.98), (0.95, 0.75), (1, 0.45), (0.93, 0.2), (0.12, 0.17), (0.12, 0)],
                                                  "rosemary-bush-symbol", T("Plan symbol with true crown extent", "Grundrisssymbol mit wahrer Kronenausdehnung"),
                                                  T("A plan sign for planting plans: the lobed crown outline of the upright shrub at its true extent with the stem base as a small circle; side and front show the upright crown on its short woody legs.",
                                                    "Ein Grundrisszeichen für Pflanzpläne: der gelappte Kronenumriss des aufrechten Strauchs in wahrer Ausdehnung mit dem Stammfuß als kleinem Kreis; Seiten- und Vorderansicht zeigen die aufrechte Krone auf ihren kurzen verholzten Beinen."),
                                                  T(f"A conventional simplification of the shrub drawn at its true extent in model metres in all three views ({dims['widthM']} m wide, {dims['depthM']} m deep and {dims['heightM']} m high): the outline is the crown with the woody base, the circle the stem base. It is a symbol, never used silently for a pictorial scale.",
                                                    f"Eine konventionelle Vereinfachung des Strauchs in seiner wahren Ausdehnung in Modellmetern in allen drei Ansichten ({dims['widthM']} m breit, {dims['depthM']} m tief und {dims['heightM']} m hoch): der Umriss ist die Krone mit dem verholzten Fuß, der Kreis der Stammfuß. Es ist ein Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet."), heart=0.06)],
        "distinction": T("An upright evergreen herb shrub of nine woody stems with columnar needle-leaved shoots and axillary flowers, whose stiff shoots sway little and splay under wet snow.",
                         "Aufrechter immergrüner Kräuterstrauch aus neun verholzten Trieben mit säulenförmigen, nadelblättrigen Sprossen und Blüten in den Blattachseln, dessen steife Sprosse wenig schwingen und sich unter Nassschnee spreizen."),
    }
    assemble(spec)


REP_DECISIONS = {
    50: decisions(
        silhouette=("At 1:50 the shrub is 18 mm wide and 24 mm high: nine shoot columns of five foliage lobes each, a lower lobe on every outer stem and a fan of three upright shoot tips per shoot give the feathery, upright crown; the bare woody legs show below 0.2 m.",
                    "Bei 1:50 ist der Strauch 18 mm breit und 24 mm hoch: neun Sprosssäulen aus je fünf Laublappen, ein unterer Lappen an jedem äußeren Trieb und ein Fächer aus drei aufrechten Triebspitzen je Spross ergeben die fedrige, aufrechte Krone; unter 0,2 m zeigen sich die kahlen verholzten Beine."),
        retained=("Retained: all nine stems and shoots, the root collar, 35 flower clusters on the outer faces of the shoots.", "Beibehalten: alle neun Triebe und Sprosse, der Wurzelhals, 35 Blütenbüschel an den Außenseiten der Sprosse."),
        merged=("Merged: the single linear leaves (1 to 4 cm) into lobes and shoot tips; the flowers of an axil into one cluster.", "Zusammengefasst: die einzelnen linealen Blätter (1 bis 4 cm) zu Lappen und Triebspitzen; die Blüten einer Achsel zu einem Büschel."),
        omitted=("Omitted: single leaves and corollas, the side twigs inside the crown.", "Weggelassen: Einzelblätter und Blütenkronen, die Seitenzweige im Inneren der Krone."),
        thinElements=("The shoot axes inside the crown (13 to 7 mm) are 0.13 mm lines; the stems are drawn at their true 28 mm.", "Die Sprossachsen im Inneren der Krone (13 bis 7 mm) sind Linien von 0,13 mm; die Triebe sind in wahrer Stärke von 28 mm gezeichnet."),
        hatching=("No hatching; three needle dashes per lobe (0.1 mm texture pen).", "Keine Schraffur; drei Nadelstriche je Lappen (Texturstift 0,1 mm)."),
        styles=("Light line: lobe, tip and flower outlines with needle dashes. Dark solid: a dark crown with lighter flowers and grey stems.", "Helle Linie: Umrisse von Lappen, Spitzen und Blüten mit Nadelstrichen. Dunkel flächig: eine dunkle Krone mit helleren Blüten und grauen Trieben."),
        overlays=("Thermal: lobes and tips carry leaf-shoots, flowers flower-clusters, stems, shoot axes and collar stem-bark. Structure: members along the nine stems and shoots, the forks as joints and the shoot masses.", "Thermik: Lappen und Spitzen tragen leaf-shoots, Blüten flower-clusters, Triebe, Sprossachsen und Wurzelhals stem-bark. Tragwerk: Stäbe entlang der neun Triebe und Sprosse, die Gabeln als Gelenke und die Sprossmassen.")),
    100: decisions(
        silhouette=("Four lobes per shoot, a lower lobe per outer stem and two shoot tips per shoot keep the upright, feathery outline at 9 mm width.", "Vier Lappen je Spross, ein unterer Lappen je äußerem Trieb und zwei Triebspitzen je Spross erhalten den aufrechten, fedrigen Umriss bei 9 mm Breite."),
        retained=("Retained: nine stems and shoots, 17 flower clusters, the bare legs.", "Beibehalten: neun Triebe und Sprosse, 17 Blütenbüschel, die kahlen Beine."),
        merged=("Merged: five lobes of a shoot into four, three tips into two, four flower clusters into two.", "Zusammengefasst: fünf Lappen eines Sprosses zu vier, drei Spitzen zu zwei, vier Blütenbüschel zu zwei."),
        omitted=("Omitted: half of the flower clusters.", "Weggelassen: die Hälfte der Blütenbüschel."),
        thinElements=("Shoot axes as 0.13 mm lines, stems at their true section.", "Sprossachsen als Linien von 0,13 mm, Triebe im wahren Querschnitt."),
        hatching=("No hatching; two needle dashes per lobe.", "Keine Schraffur; zwei Nadelstriche je Lappen."),
        styles=("As at 1:50 with fewer marks.", "Wie bei 1:50 mit weniger Zeichen."),
        overlays=("Thermal regions as at 1:50; members and fork joints.", "Thermische Regionen wie bei 1:50; Stäbe und Gabelgelenke.")),
    200: decisions(
        silhouette=("At 1:200 (4.5 by 6 mm) every shoot is two large lobes and one shoot tip, so the crown reads as nine upright columns on the woody base.", "Bei 1:200 (4,5 mal 6 mm) ist jeder Spross zwei große Lappen und eine Triebspitze, sodass die Krone als neun aufrechte Säulen auf dem verholzten Fuß liest."),
        retained=("Retained: the nine columns, the bare legs, one flower cluster per outer shoot.", "Beibehalten: die neun Säulen, die kahlen Beine, ein Blütenbüschel je äußerem Spross."),
        merged=("Merged: the lobes of a shoot into two, the lower stem lobes into the columns.", "Zusammengefasst: die Lappen eines Sprosses zu zwei, die unteren Triebslappen in die Säulen."),
        omitted=("Omitted: needle texture.", "Weggelassen: Nadeltextur."),
        thinElements=("Flower clusters thickened to 32 mm (0.16 mm), stems to 32 mm.", "Blütenbüschel auf 32 mm (0,16 mm) verstärkt, Triebe auf 32 mm."),
        hatching=("No hatching.", "Keine Schraffur."),
        styles=("Light line: outlines. Dark solid: a dark columnar crown.", "Helle Linie: Umrisse. Dunkel flächig: eine dunkle säulenförmige Krone."),
        overlays=("Thermal regions as at 1:100; members without joints.", "Thermische Regionen wie bei 1:100; Stäbe ohne Gelenke.")),
    500: decisions(
        silhouette=("At 1:500 (1.8 by 2.4 mm) the crown is one revolved envelope of the modelled shoots on one trunk leg.", "Bei 1:500 (1,8 mal 2,4 mm) ist die Krone eine Rotationshülle der modellierten Sprosse auf einem Stammbein."),
        retained=("Retained: the upright ovoid crown and the open woody foot.", "Beibehalten: die aufrechte eiförmige Krone und der offene verholzte Fuß."),
        merged=("Merged: shoots, tips and flowers into the crown shape (leaf-shoots and flower-clusters merged, area weighted); nine stems into one 80 mm leg.", "Zusammengefasst: Sprosse, Spitzen und Blüten in die Kronenform (leaf-shoots und flower-clusters zusammengefasst, flächengewichtet); neun Triebe zu einem Bein von 80 mm."),
        omitted=("Omitted: the columns of the shoots and the flowers as separate forms.", "Weggelassen: die Säulen der Sprosse und die Blüten als eigene Formen."),
        thinElements=("The legs (28 mm) are merged into one 80 mm leg at the 75 mm feature.", "Die Beine (28 mm) sind zu einem Bein von 80 mm im Merkmal von 75 mm zusammengefasst."),
        hatching=("No hatching.", "Keine Schraffur."),
        styles=("Light line: one closed outline. Dark solid: one dark mass on a lighter leg.", "Helle Linie: ein geschlossener Umriss. Dunkel flächig: eine dunkle Masse auf einem helleren Bein."),
        overlays=("Thermal: the crown shows the area-weighted value of leaf-shoots and flower-clusters (labelled as merged), the leg stem-bark.", "Thermik: die Krone zeigt den flächengewichteten Wert von leaf-shoots und flower-clusters (als zusammengefasst beschriftet), das Bein stem-bark.")),
}

VOLUME_NOTES = {
    50: [("Body: 53 foliage lobes and 27 shoot tips as ellipsoids along the true lines of the nine shoots, 35 flower clusters, nine stems (28 mm), nine slender shoot axes and the root collar; every outline drawn is the exact projection of these solids.", "Körper: 53 Laublappen und 27 Triebspitzen als Ellipsoide entlang der wahren Linien der neun Sprosse, 35 Blütenbüschel, neun Triebe (28 mm), neun schlanke Sprossachsen und der Wurzelhals; jeder gezeichnete Umriss ist die exakte Projektion dieser Körper."),
         ("Thermal: references of every body solid plus 17 transpiration markers. Structure: members along the stems and shoots, nine fork joints, nine shoot masses, the support slab.", "Thermik: Verweise auf jeden Körper und 17 Transpirationsmarken. Tragwerk: Stäbe entlang der Triebe und Sprosse, neun Gabelgelenke, neun Sprossmassen, die Auflagerplatte.")],
    100: [("Body: 44 lobes, 18 tips, 17 flower clusters, stems, shoot axes and collar.", "Körper: 44 Lappen, 18 Spitzen, 17 Blütenbüschel, Triebe, Sprossachsen und Wurzelhals."),
          ("Structure: members and fork joints, no masses.", "Tragwerk: Stäbe und Gabelgelenke, keine Massen.")],
    200: [("Body: 18 lobes, nine tips, eight flower clusters of 32 mm, stems of 32 mm, slender shoot axes, the collar.", "Körper: 18 Lappen, neun Spitzen, acht Blütenbüschel von 32 mm, Triebe von 32 mm, schlanke Sprossachsen, der Wurzelhals."),
          ("Structure: one member per bone, the support slab.", "Tragwerk: ein Stab je Knochen, die Auflagerplatte.")],
    500: [("Body: the crown as one solid of revolution enveloping the 1:50 shoots, one 80 mm trunk leg and eight slender stems inside the crown.", "Körper: die Krone als ein Rotationskörper, der die Sprosse von 1:50 umhüllt, ein Stammbein von 80 mm und acht schlanke Triebe im Inneren der Krone."),
          ("Thermal: the flower clusters as one inner share of the crown. Structure: one column for root crown, central stem and shoot, slender members for the other stems and shoots.", "Thermik: die Blütenbüschel als ein innerer Anteil der Krone. Tragwerk: eine Säule für Wurzelhals, mittleren Trieb und Spross, schlanke Stäbe für die übrigen Triebe und Sprosse.")],
}

if __name__ == "__main__":
    build()
