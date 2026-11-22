"""🌹 Shrub rose (Rosa, modern shrub rose), a 1.5 m fountain of eleven arching canes in June flower: species data, rig, four scales of drawing and volume, poses, clips, thermal and structure.

Run: `repo tools exec python -- shrub_rose.py` (writes the catalog asset and the manifest row).
@see plant_lib.py, shrub.py
"""
import math
from plant_lib import (T, r4, Rig, revolute, FIXED, mass, round_section, Ell, Cap, Rev, Mark, Scale, pens, decisions, member_capsule, member_loft, support_box, feature, overrides,
                       vadd, vsub, vscale, vlen, lerp3, leaf_region, passive_region, transpiration)
from shrub import wind_at, drag, region_bindings, load, pose, gust_clip, sway_clip, diurnal_clip, up_direction, assemble, outline_symbol

ID = "shrub-rose"
REFS = [
    ("phillips-rix-2004", "Phillips, R. and Rix, M. (2004). The Ultimate Guide to Roses. Macmillan, London. Modern shrub roses: arching canes of 1.2 to 2 m, repeat-flowering, flowers 6 to 10 cm across in clusters."),
    ("cairns-2000", "Cairns, T. (ed.) (2000). Modern Roses 11: The World Encyclopedia of Roses. Academic Press, San Diego. Classification and habit of the shrub rose class."),
    ("wood-handbook-2021", "Forest Products Laboratory (2021). Wood Handbook: Wood as an Engineering Material. General Technical Report FPL-GTR-282, USDA Forest Service. Chapter 5 (green density, modulus of elasticity and rupture; closest listed hardwood used as proxy)."),
    ("en-1991-1-4", "EN 1991-1-4:2005 Eurocode 1, Actions on structures, Part 1-4: Wind actions. Air density 1.25 kg/m3, F = 0.5 rho cd A v^2, peak velocity pressure (1 + 7 Iv) q_m."),
    ("vogel-1989", "Vogel, S. (1989). Drag and reconfiguration of broad leaves in high winds. Journal of Experimental Botany 40(8), 941-948."),
    ("jones-2014", "Jones, H. G. (2014). Plants and Microclimate, 3rd ed., Cambridge University Press. Leaf energy balance, leaf area index, boundary-layer and stomatal conductance, interception of rain by foliage."),
    ("monteith-unsworth-2013", "Monteith, J. L. and Unsworth, M. H. (2013). Principles of Environmental Physics, 4th ed., Academic Press. Penman-Monteith leaf transpiration."),
]
LEAF, BLOOM, BARK = "leaf-canopy", "rose-blooms", "cane-bark"
M_LEAF, M_BLOOM, M_BARK, M_WOOD = "rose-leaf", "rose-petal", "rose-cane", "rose-wood"
KEYS = ["c"] + [str(i) for i in range(1, 11)]
AZ = {str(i): 18 + 36 * (i - 1) for i in range(1, 11)}
JIT = [1.0, 0.97, 1.03, 0.99, 1.02, 0.96, 1.04, 0.98, 1.01, 0.95, 1.03]


def canes():
    """🌿 The eleven canes in world rest metres as three true 3D segments each (azimuth from +x towards +z, lean from the vertical): a steep base, an arching middle and a nodding tip."""
    out = {}
    for i, k in enumerate(KEYS):
        if k == "c":
            a, base, segs = 0, (0, 0, 0), [(4, 0.62), (9, 0.48), (16, 0.33)]
        else:
            a = AZ[k]
            base = (0.045 * math.cos(math.radians(a)), 0, 0.045 * math.sin(math.radians(a)))
            j = JIT[i]
            segs = [(13 * j, 0.68 * JIT[(i + 2) % 11]), (40 * j, 0.44), (84 + 6 * (j - 1) * 10, 0.3 * JIT[(i + 4) % 11])]
        pts = [base]
        dirs = []
        for lean, length in segs:
            d = (math.sin(math.radians(lean)) * math.cos(math.radians(a)), math.cos(math.radians(lean)), math.sin(math.radians(lean)) * math.sin(math.radians(a)))
            dirs.append(d)
            pts.append(vadd(pts[-1], vscale(d, length)))
        out[k] = {"a": a, "pts": pts, "dirs": dirs, "lens": [l for _, l in segs]}
    return out


C = canes()
RADII = [0.011, 0.009, 0.007]


def on_cane(k, seg, t):
    return lerp3(C[k]["pts"][seg], C[k]["pts"][seg + 1], t)


def rig():
    r = Rig()
    r.add("root-crown", None, T("Root crown with bud union", "Wurzelhals mit Veredlungsstelle"), (-0.06, 0, 0), 0, 0.12, FIXED, mass(1.6, 0.06, "authored-estimate", "wood-handbook-2021"), round_section(0.06, M_WOOD, "Solid section, d = 6 cm", "Vollquerschnitt, d = 6 cm"))
    names = [("base", "Basis"), ("arch", "Bogen"), ("tip", "Spitze")]
    masses = [0.22, 0.36, 0.28]
    for k in KEYS:
        c = C[k]
        for s in range(3):
            d = c["dirs"][s]
            h = math.hypot(d[0], d[1])
            parent = "root-crown" if s == 0 else f"cane-{k}-{s}"
            en = f"Central cane, {names[s][0]}" if k == "c" else f"Cane {k}, {names[s][0]}"
            de = f"Mittlere Rute, {names[s][1]}" if k == "c" else f"Rute {k}, {names[s][1]}"
            lim = [revolute(-4, 4), revolute(-12, 12), revolute(-25, 25)][s]
            sec = round_section(2 * RADII[s], M_WOOD, f"Solid section, d = {round(2000 * RADII[s])} mm", f"Vollquerschnitt, d = {round(2000 * RADII[s])} mm")
            m = mass(masses[s], c["lens"][s] * h / 2, "authored-estimate", "wood-handbook-2021" if s == 0 else "phillips-rix-2004", "uniform" if s == 0 else "point")
            r.add(f"cane-{k}-{s + 1}", parent, T(en, de), c["pts"][s], math.degrees(math.atan2(d[1], d[0])), c["lens"][s] * h, lim, m, sec)
    return r


def phi_of(d):
    return math.degrees(math.atan2(d[1], d[0]))


FOLIAGE = {
    50: [(0, 0.8, 0.12), (1, 0.15, 0.15), (1, 0.5, 0.165), (1, 0.85, 0.15), (2, 0.25, 0.13), (2, 0.6, 0.12), (2, 0.92, 0.095)],
    100: [(0, 0.82, 0.13), (1, 0.3, 0.17), (1, 0.75, 0.165), (2, 0.45, 0.14), (2, 0.9, 0.105)],
    200: [(1, 0.35, 0.19), (2, 0.42, 0.15), (2, 0.88, 0.1)],
    500: [(1, 0.5, 0.24), (2, 0.7, 0.16)],
}
BLOOMS = {50: [(2, 1.0, 0), (2, 0.8, 1), (2, 0.8, -1), (1, 0.7, 1), (1, 0.45, -1)], 100: [(2, 1.0, 0), (2, 0.7, 1), (1, 0.6, -1)], 200: [(2, 1.0, 0), (1, 0.6, 1)], 500: [(2, 1.0, 0)]}
BLOOM_R = {50: (0.045, 0.034), 100: (0.05, 0.038), 200: (0.06, 0.045), 500: (0.075, 0.06)}


def scale_model(r, den, hk=1.0, ak=1.0):
    sc = Scale(den, r)
    tl, tb, tc = 0.52, 0.28, 0.36
    fine = den in (50, 100)
    sc.add(Rev("root-collar", "root-crown", (0, 0, 0), [(0, 0), (0, 0.07), (0.04, 0.065), (0.08, 0.04), (0.09, 0)], material=M_BARK, role="body", tone=tc, region=BARK, z=0.5))
    hosts = []
    for k in KEYS:
        c = C[k]
        for s in range(3):
            rr = RADII[s] if den == 50 else max(RADII[s], 0.0078) if den == 100 else RADII[s]
            p0 = c["pts"][s] if s else (c["pts"][0][0], rr, c["pts"][0][2])
            sc.add(Cap(f"cane-{k}-{s + 1}", f"cane-{k}-{s + 1}", p0, c["pts"][s + 1], rr, rr * 0.85 if s == 2 else rr, material=M_BARK, role="body" if fine else "detail", tone=tc,
                       region=BARK, slender=not fine, stroke=not fine, z=0.6))
        for j, (seg, t, rad) in enumerate(FOLIAGE[den]):
            if k == "c" and seg == 0:
                continue
            d = c["dirs"][seg]
            f = 1.0 if k == "c" else JIT[(int(k) + j) % 11]
            along = rad * (1.15 if seg == 1 else 1.05) * f * ak
            h = Ell(f"leaves-{k}-{j + 1}", f"cane-{k}-{seg + 1}", on_cane(k, seg, t), (along, rad * 0.95 * hk, rad * hk), phi=phi_of(d), material=M_LEAF, role="body", tone=tl, region=LEAF, z=1 + 0.01 * j)
            sc.add(h)
            hosts.append(h)
        ba, bb = BLOOM_R[den]
        for j, (seg, t, side) in enumerate(BLOOMS[den]):
            axis = on_cane(k, seg, t)
            az = math.radians(c["a"] + 90 * side)
            off = 0.06 if side else 0.0
            back = 0.0 if side else ba - BLOOM_R[50][0]
            centre = (axis[0] + off * math.cos(az) - back * c["dirs"][seg][0], axis[1] + bb * (0.9 if side == 0 else 0.6) + (0 if side == 0 else 0.07), axis[2] + off * math.sin(az) - back * c["dirs"][seg][2])
            if k == "c" and seg == 2 and side == 0:
                centre = (centre[0], 1.5 - bb, centre[2])
            sc.add(Ell(f"bloom-{k}-{j + 1}", f"cane-{k}-{seg + 1}", centre, (ba, bb, ba), material=M_BLOOM, role="body", tone=0.06, region=BLOOM, z=2))
    if fine:
        leaflets(sc, hosts, den)
    return sc


def leaflets(sc, hosts, den):
    """🍃 Leaf marks: small closed leaflet ovals in pinnate pairs inside the foliage masses (side and front), short midrib dashes in plan."""
    per = 3 if den == 50 else 2
    m = 0
    for h in hosts:
        a, b, c = h.r
        for j in range(per):
            f = (j + 0.5) / per - 0.5
            for view in ("elevation", "front", "plan"):
                if view == "elevation":
                    cx, cy, rx, ry = h.c[0], h.c[1], a, b
                elif view == "front":
                    cx, cy, rx, ry = h.c[2], h.c[1], c, b
                else:
                    cx, cy, rx, ry = h.c[0], h.c[2], a, c
                px, py = cx + f * 1.0 * rx, cy + (0.15 - 0.3 * abs(f)) * ry
                if view == "plan" or den == 100:
                    pts = [(px - 0.025, py - 0.012), (px + 0.025, py + 0.012)]
                    closed = False
                else:
                    pts = [(px - 0.026, py), (px - 0.01, py + 0.011), (px + 0.014, py + 0.009), (px + 0.026, py), (px + 0.012, py - 0.01), (px - 0.012, py - 0.009)]
                    closed = True
                m += 1
                sc.mark(Mark(f"leaflet-mark-{m:03d}", h.bone, view, pts, h, role="texture", closed=closed))


def calibrated(r, den, target):
    """📏 The along and across factors of a coarse scale's leaf masses, found by alternating bisection so that its side width and front depth equal those of 1:50 (the masses are redrawn, their envelope keeps the true extent)."""
    hk, ak = 1.0, 1.0
    for _ in range(3):
        for axis in (0, 2):
            lo, hi = 0.6, 1.6
            for _ in range(24):
                mid = (lo + hi) / 2
                a, b = scale_model(r, den, mid if axis == 2 else hk, mid if axis == 0 else ak).extent()
                if b[axis] - a[axis] > target[axis]:
                    hi = mid
                else:
                    lo = mid
            hk, ak = (lo, ak) if axis == 2 else (hk, lo)
    print(f"[DEBUG] 1:{den} across factor {hk:.4f}, along factor {ak:.4f}")
    return scale_model(r, den, hk, ak)


def regions(bark):
    return [
        {**leaf_region(LEAF, T("Pinnate leaves of the canes", "Gefiederte Blätter der Ruten"), M_LEAF, 4.2, 0.03, "vertical-plate", 0.5, 600, 0.006, 1.9, "c3", "jones-2014"), "areaBasis": "exchange"},
        {**leaf_region(BLOOM, T("Open rose blooms (petals and sepals)", "Offene Rosenblüten (Kron- und Kelchblätter)"), M_BLOOM, 0.45, 0.08, "horizontal-plate-up", 0.8, 500, 0.0008, 0.2, "c3", "phillips-rix-2004"), "areaBasis": "exchange"},
        passive_region(BARK, T("Green to brown canes with prickles", "Grüne bis braune Ruten mit Stacheln"), "stem", M_BARK, bark, 0.018, "cylinder", 0.2, 12000, "wood-handbook-2021"),
    ]


def anchors(r):
    out = [{"id": "base-support", "bone": "root-crown", "x": 0.06, "y": 0, "kind": "support", "title": T("Planting hole anchorage", "Verankerung im Pflanzloch"), "support": "fixed", "normalDeg": 90, "footprintLengthM": 0.45, "footprintWidthM": 0.45, "frictionCoefficient": 0.6}]
    for k in KEYS:
        x, y, z = r.local(f"cane-{k}-2", on_cane(k, 1, 0.5))
        out.append({"id": f"tp-leaves-{k}", "bone": f"cane-{k}-2", "x": r4(x), "y": r4(y), **({"z": r4(z)} if abs(z) > 1e-9 else {}), "kind": "emitter", "title": T(f"Transpiration of the leaves of cane {k}" if k != "c" else "Transpiration of the leaves of the central cane", f"Transpiration der Blätter von Rute {k}" if k != "c" else "Transpiration der Blätter der mittleren Rute")})
        x, y, z = r.local(f"cane-{k}-3", on_cane(k, 2, 0.95))
        out.append({"id": f"tp-blooms-{k}", "bone": f"cane-{k}-3", "x": r4(x), "y": r4(y), **({"z": r4(z)} if abs(z) > 1e-9 else {}), "kind": "emitter", "title": T(f"Transpiration of the blooms of cane {k}" if k != "c" else "Transpiration of the blooms of the central cane", f"Transpiration der Blüten von Rute {k}" if k != "c" else "Transpiration der Blüten der mittleren Rute")})
    return out


def structure(r, d):
    """🏗️ Members along the true cane segments; joints at the base-to-arch bends and point masses of the arches and tips at 1:50, joints at 1:100, members only at 1:200, and at 1:500 the root crown and the central cane as one loft."""
    f = feature(d)
    out = [support_box(r, "root-crown", "base-support", (0.18, f, 0.18), (0, -f / 2, 0), M_WOOD)]
    if d == 500:
        p = C["c"]["pts"]
        out.append(member_loft(r, "member-centre", "cane-c-1", [(0, 0, 0), p[1], p[2], p[3]], 0.04, M_WOOD, members=["root-crown", "cane-c-2", "cane-c-3"]))
    else:
        out.append(member_capsule(r, "member-root-crown", "root-crown", (-0.025, 0.035, 0), (0.025, 0.035, 0), max(0.03, 0.55 * f), M_WOOD))
    for k in KEYS:
        if d == 500 and k == "c":
            continue
        c = C[k]
        for s in range(3):
            rr = RADII[s] if d == 50 else max(RADII[s], 0.0078) if d == 100 else RADII[s]
            p0 = c["pts"][s] if s else (c["pts"][0][0], rr, c["pts"][0][2])
            out.append(member_capsule(r, f"member-cane-{k}-{s + 1}", f"cane-{k}-{s + 1}", p0, c["pts"][s + 1], rr, M_WOOD, slender=d >= 200))
    if d in (50, 100):
        for k in KEYS:
            out.append({"id": f"joint-cane-{k}-2", "kind": "ellipsoid", "bone": f"cane-{k}-2", "material": M_WOOD, "role": "detail", "tone": 0.3, "radii": [0.013] * 3, "transform": {"translation": [r4(x) for x in r.local(f"cane-{k}-2", C[k]["pts"][1])]}, "member": f"cane-{k}-2", "part": "joint"})
    if d == 50:
        for k in KEYS:
            out.append({"id": f"mass-cane-{k}-3", "kind": "ellipsoid", "bone": f"cane-{k}-3", "material": M_WOOD, "role": "detail", "tone": 0.3, "radii": [0.02] * 3, "transform": {"translation": [r4(x) for x in r.local(f"cane-{k}-3", on_cane(k, 2, 0.5))]}, "member": f"cane-{k}-3", "part": "mass"})
    return out


def outward(k):
    return 0 if k == "c" else math.cos(math.radians(AZ[k]))


def poses(r):
    rg = [LEAF, BLOOM, BARK]
    sun = {LEAF: 0.5, BLOOM: 0.8, BARK: 0.15}
    em = ["transpiration-leaves", "transpiration-blooms"]
    area = 0.16

    def winds(v10, cd, src, ref, calm=False):
        out = []
        for k in KEYS:
            mid = on_cane(k, 1, 0.5)
            v = 1.5 if calm else wind_at(v10, mid[1])
            out.append(load(f"wind-cane-{k}", "wind", T(f"Wind drag on the arch of cane {k}" if k != "c" else "Wind drag on the central cane", f"Winddruck auf den Bogen von Rute {k}" if k != "c" else "Winddruck auf die mittlere Rute"), f"cane-{k}-2", r4(r.W[f"cane-{k}-2"][2] * 0.5), 0, drag(cd, area, v), 0, src, ref))
        return out

    def lean(a1, a2, a3):
        b = {}
        for k in KEYS:
            b[f"cane-{k}-1"] = {"angleDeg": a1}
            b[f"cane-{k}-2"] = {"angleDeg": a2}
            b[f"cane-{k}-3"] = {"angleDeg": a3}
        return b

    flowering = pose("flowering", T("Calm June day in full flower", "Windstiller Junitag in voller Blüte"),
                     T("Light air of 1.5 m/s: the eleven canes arch out under the weight of their flower clusters, the blooms open and facing up; the drag on an arch is below 0.3 N.", "Leiser Zug von 1,5 m/s: die elf Ruten neigen sich unter dem Gewicht ihrer Blütenbüschel bogig nach außen, die Blüten offen und nach oben gewandt; der Widerstand auf einem Bogen liegt unter 0,3 N."),
                     {}, region_bindings(rg, sun), {e: 0.85 for e in em}, winds(0, 0.9, "literature", "en-1991-1-4", calm=True))
    windy = pose("windy", T("Fresh breeze", "Frische Brise"),
                 T("Beaufort 5, 8 m/s at 10 m, about 4.9 m/s at the arches; cd 0.7 on 0.16 m2 per cane gives about 1.7 N each. The flexible arches swing 4 degrees and the nodding tips 9 degrees downwind.",
                   "Beaufort 5, 8 m/s in 10 m Höhe, etwa 4,9 m/s an den Bögen; cd 0,7 auf 0,16 m2 je Rute ergibt etwa 1,7 N je Rute. Die biegsamen Bögen schwingen um 4 Grad, die nickenden Spitzen um 9 Grad nach Lee."),
                 lean(-1, -4, -9), region_bindings(rg, sun), {e: 1 for e in em}, winds(8, 0.7, "literature", "en-1991-1-4"))
    storm = pose("storm", T("Gale", "Sturm"),
                 T("Beaufort 9, 22 m/s at 10 m (13 to 14 m/s at the arches), cd 0.45 after the leaves fold: about 7 N per cane, the arches swing 10 degrees, the tips 22 degrees and petals start to strip.",
                   "Beaufort 9, 22 m/s in 10 m Höhe (13 bis 14 m/s an den Bögen), cd 0,45 nach dem Zusammenlegen der Blätter: etwa 7 N je Rute, die Bögen schwingen um 10 Grad, die Spitzen um 22 Grad, und die Blüten beginnen sich zu entblättern."),
                 lean(-2.5, -10, -22), region_bindings(rg, {LEAF: 0.3, BLOOM: 0.4, BARK: 0.05}, {LEAF: 0.4, BLOOM: 0.5, BARK: 0.3}), {e: 0.5 for e in em}, winds(22, 0.45, "literature", "vogel-1989"))
    droop = {}
    for k in KEYS:
        o = outward(k)
        droop[f"cane-{k}-1"] = {"angleDeg": r4(-1.5 * o)}
        droop[f"cane-{k}-2"] = {"angleDeg": r4(-6 * o) if k != "c" else -3}
        droop[f"cane-{k}-3"] = {"angleDeg": r4(-14 * o) if k != "c" else -8}
    rain_loads = [load(f"water-cane-{k}", "imposed", T(f"Rain water held by the blooms and leaves of cane {k}" if k != "c" else "Rain water held by the blooms and leaves of the central cane", f"Von Blüten und Blättern der Rute {k} gehaltenes Regenwasser" if k != "c" else "Von Blüten und Blättern der mittleren Rute gehaltenes Regenwasser"), f"cane-{k}-3", r4(r.W[f"cane-{k}-3"][2] * 0.8), 0, 0, -1.1, "authored-estimate", "jones-2014") for k in KEYS]
    rain = pose("rain-laden", T("Rain-laden after a summer shower", "Regenschwer nach einem Sommerschauer"),
                T("Each cane holds about 110 g of rain in its full blooms and on its leaves (about 10 g per bloom and 0.1 kg/m2 of leaf, authored estimate after Jones 2014): the arches sink by up to 6 degrees and the flowering tips nod by up to 14 degrees outwards; the rig turns the canes in the side plane only, so the sideways canes sink less than they would. Calm, wet.",
                  "Jede Rute hält etwa 110 g Regen in ihren gefüllten Blüten und auf ihren Blättern (etwa 10 g je Blüte und 0,1 kg/m2 Blatt, Schätzung nach Jones 2014): die Bögen sinken um bis zu 6 Grad und die blühenden Spitzen nicken um bis zu 14 Grad nach außen; das Rig dreht die Ruten nur in der Seitenebene, daher sinken die seitlich weisenden Ruten weniger, als sie es täten. Windstill, nass."),
                droop, region_bindings(rg, {LEAF: 0.05, BLOOM: 0.1, BARK: 0}, {LEAF: 1, BLOOM: 1, BARK: 0.8}), {e: 0.15 for e in em}, rain_loads)
    return [flowering, windy, storm, rain]


def clips(r):
    base, specs, peak = {}, {}, {}
    for k in KEYS:
        base.update({f"cane-{k}-1": -1, f"cane-{k}-2": -4, f"cane-{k}-3": -9})
        ph = 0 if k == "c" else AZ[k]
        specs[f"cane-{k}-1"] = (0.4, ph, 1)
        specs[f"cane-{k}-2"] = (2.0, ph + 30, 1)
        specs[f"cane-{k}-3"] = (4.5, ph + 70, 1)
        peak.update({f"cane-{k}-1": -1.8, f"cane-{k}-2": -7.5, f"cane-{k}-3": -16})
    sway = sway_clip(r, 1.6, 16, base, specs, T("A deterministic 1.6 s cycle about the fresh-breeze lean (long flexible canes, estimated 0.6 Hz): the arches swing by 2 degrees and the flower-laden tips by 4.5 degrees with a lag, the phase travelling round the fountain.",
                                                "Ein deterministischer Zyklus von 1,6 s um die Neigung der frischen Brise (lange, biegsame Ruten, geschätzt 0,6 Hz): die Bögen schwingen um 2 Grad und die blütenschweren Spitzen nachlaufend um 4,5 Grad, die Phase wandert um die Fontäne."),
                     ["transpiration-leaves", "transpiration-blooms"])
    gust = gust_clip(None, [{"id": f"wind-cane-{k}"} for k in KEYS], peak, T("Gust", "Windböe"),
                     T("A single 8 s gust: the wind-load tracks rise from 5 % to 2.75 times the fresh-breeze load at 2.5 s (EN 1991-1-4 peak pressure with Iv 0.25) and decay; the tips lean up to 16 degrees.",
                       "Eine einzelne Böe über 8 s: die Windlast-Spuren steigen bei 2,5 s von 5 % auf das 2,75-fache der Last der frischen Brise (Spitzendruck nach EN 1991-1-4 mit Iv 0,25) und klingen ab; die Spitzen neigen sich bis zu 16 Grad."))
    day = diurnal_clip({LEAF: 0.5, BLOOM: 0.8, BARK: 0.15}, ["transpiration-leaves", "transpiration-blooms"],
                       T("A 24 h June cycle compressed to 24 s (1 s = 1 h), calm weather. The sunlit fraction of every region and the transpiration strength follow the sun; leaf, petal and cane temperatures follow from the thermal solver.",
                         "Ein 24-h-Junizyklus auf 24 s gerafft (1 s = 1 h) bei ruhigem Wetter. Der besonnte Anteil jeder Region und die Transpirationsstärke folgen der Sonne; Blatt-, Blüten- und Rutentemperaturen folgen aus dem thermischen Löser."), "flowering")
    return [sway, gust, day]


def build():
    r = rig()
    s50 = scale_model(r, 50)
    a, b = s50.extent()
    target = (b[0] - a[0], 0, b[2] - a[2])
    scales = {50: s50, 100: calibrated(r, 100, target), 200: calibrated(r, 200, target), 500: calibrated(r, 500, target)}
    bark = overrides(ID).get(BARK, 0.6)
    emitter_anchors = [(f"tp-leaves-{k}", LEAF, "transpiration-leaves", M_LEAF) for k in KEYS] + [(f"tp-blooms-{k}", BLOOM, "transpiration-blooms", M_BLOOM) for k in KEYS]
    spec = {
        "id": ID, "rig": r, "scales": scales, "anchors": anchors(r), "emitter_anchors": emitter_anchors, "emitter_radius": {50: 0.02, 100: 0.024, 200: 0.03, 500: 0.045},
        "structure": lambda d: structure(r, d), "volume_notes": VOLUME_NOTES, "drawn": (50, 100, 200, 500),
        "ranges": {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)},
        "pens": {50: pens(0.2, 0.13, 0.1, None, 0.3), 100: pens(0.18, 0.13, 0.1, None, 0.25), 200: pens(0.15, 0.13, None, None, 0.25), 500: pens(0.15, 0.13, None, None, 0.25)},
        "decisions": REP_DECISIONS, "title": T("Shrub Rose", "Strauchrose"),
        "summary": lambda dims: T(f"A modern shrub rose (Rosa) in June flower: eleven green-brown canes rise steeply from the grafted root crown, arch outwards and nod at their tips, forming a fountain {dims['widthM']:.2f} m across and {dims['heightM']:.2f} m high; clusters of open blooms 9 cm across sit on the arches and tips among the pinnate leaves. The repeat-flowering rose of borders, front gardens and parks.",
                                  f"Eine moderne Strauchrose (Rosa) in Juniblüte: elf grünbraune Ruten steigen steil aus dem veredelten Wurzelhals auf, neigen sich bogig nach außen und nicken an den Spitzen; so entsteht eine Fontäne von {dims['widthM']:.2f} m Breite und {dims['heightM']:.2f} m Höhe; Büschel offener Blüten von 9 cm Durchmesser sitzen auf den Bögen und Spitzen zwischen den gefiederten Blättern. Die öfterblühende Rose der Rabatten, Vorgärten und Parks."),
        "tags": ["shrub", "deciduous", "flowering", "rose", "arching", "wind", "transpiration"],
        "refs": REFS,
        "notes": T("Eleven canes of three bones each (steep base, arch, nodding tip) carry the foliage masses and the bloom clusters, so wind, gust and rain move the arches and tips with their own stiffness. The rig is planar: canes that arch sideways sway and sink only in the side plane. Hips, autumn colour and a winter-bare geometry are not offered because shapes cannot be toggled by a pose.",
                   "Elf Ruten aus je drei Knochen (steile Basis, Bogen, nickende Spitze) tragen die Laubmassen und Blütenbüschel, sodass Wind, Böe und Regen Bögen und Spitzen mit ihrer eigenen Steifigkeit bewegen. Das Rig ist eben: seitlich bogige Ruten schwingen und sinken nur in der Seitenebene. Hagebutten, Herbstfärbung und eine winterkahle Geometrie werden nicht angeboten, da Formen durch eine Pose nicht ausgeblendet werden können."),
        "materials": [
            {"id": M_LEAF, "title": T("Glossy dark green rose leaflets", "Glänzende dunkelgrüne Rosenfiederblättchen"), "emissivity": 0.96, "solarAbsorptivity": 0.6, "source": "authored-estimate", "reference": "jones-2014"},
            {"id": M_BLOOM, "title": T("Deep pink rose petals", "Tiefrosa Rosenblütenblätter"), "emissivity": 0.95, "solarAbsorptivity": 0.6, "source": "authored-estimate", "reference": "jones-2014"},
            {"id": M_BARK, "title": T("Green-brown cane rind with prickles", "Grünbraune Rutenrinde mit Stacheln"), "emissivity": 0.94, "solarAbsorptivity": 0.7, "densityKgM3": 700, "conductivityWmK": 0.14, "source": "authored-estimate", "reference": "wood-handbook-2021"},
            {"id": M_WOOD, "title": T("Green rose cane wood", "Frisches Rosenholz"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 850, "conductivityWmK": 0.15, "elasticModulusPa": 4000000000, "strengthPa": 40000000, "source": "authored-estimate", "reference": "wood-handbook-2021"},
        ],
        "poses": poses(r), "clips": clips(r),
        "environment": {"airTempC": 24, "meanRadiantTempC": 28, "relativeHumidity": 0.55, "windMS": 1.5, "windDirectionDeg": 0, "solarWM2": 800, "pressurePa": 101325, "diffuseFraction": 0.15},
        "regions": regions(bark),
        "emitters": [transpiration("transpiration-leaves", T("Transpiration of the leaves (distributed over the arches)", "Transpiration der Blätter (über die Bögen verteilt)"), [f"tp-leaves-{k}" for k in KEYS], LEAF, up_direction(r, "cane-c-2"), 0.35),
                     transpiration("transpiration-blooms", T("Transpiration of the blooms (distributed over the flowering tips)", "Transpiration der Blüten (über die blühenden Spitzen verteilt)"), [f"tp-blooms-{k}" for k in KEYS], BLOOM, up_direction(r, "cane-c-3"), 0.2)],
        "structure_text": ("One rigid body per evaluated pose, fixed at the planting hole; wind drag acts at mid-length of each arch, rain water at four fifths of each nodding tip, gravity on all bone masses. The canes are idealised as solid round sections of 22, 18 and 14 mm. Root and soil failure, cane breakage, large deflection and dynamic amplification are not modelled.",
                           "Ein starrer Körper je ausgewerteter Pose, im Pflanzloch eingespannt; Winddruck wirkt in der halben Länge jedes Bogens, Regenwasser auf vier Fünfteln jeder nickenden Spitze, Schwerkraft auf allen Knochenmassen. Die Ruten sind als runde Vollquerschnitte von 22, 18 und 14 mm idealisiert. Wurzel- und Bodenversagen, Rutenbruch, große Verformungen und dynamische Überhöhung werden nicht modelliert."),
        "symbols": lambda dims: [outline_symbol(r, "root-crown", dims, [(-0.08, 0), (-0.08, 0.25), (-0.72, 0.55), (-1, 0.62), (-0.98, 0.78), (-0.8, 0.95), (-0.55, 1.08), (-0.3, 1.2), (-0.1, 1.5), (0.1, 1.48), (0.3, 1.2), (0.55, 1.08), (0.8, 0.95), (0.98, 0.78), (1, 0.62), (0.72, 0.55), (0.08, 0.25), (0.08, 0)],
                                                  "shrub-rose-symbol", T("Plan symbol with true fountain extent", "Grundrisssymbol mit wahrer Fontänenausdehnung"),
                                                  T("A plan sign for planting plans: the lobed outline of the arching canes at their true extent with the root crown as a small circle; side and front show the fountain of arches on its crown.",
                                                    "Ein Grundrisszeichen für Pflanzpläne: der gelappte Umriss der bogigen Ruten in wahrer Ausdehnung mit dem Wurzelhals als kleinem Kreis; Seiten- und Vorderansicht zeigen die Fontäne der Bögen über ihrem Wurzelhals."),
                                                  T(f"A conventional simplification of the shrub drawn at its true extent in model metres in all three views ({dims['widthM']} m wide, {dims['depthM']} m deep and {dims['heightM']} m high): the outline is the fountain of canes, the circle the root crown. It is a symbol, never used silently for a pictorial scale.",
                                                    f"Eine konventionelle Vereinfachung des Strauchs in seiner wahren Ausdehnung in Modellmetern in allen drei Ansichten ({dims['widthM']} m breit, {dims['depthM']} m tief und {dims['heightM']} m hoch): der Umriss ist die Fontäne der Ruten, der Kreis der Wurzelhals. Es ist ein Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet."), heart=0.07)],
        "distinction": T("A deciduous flowering shrub of eleven arching three-part canes with bloom clusters on the nodding tips, whose rain-laden pose sinks the arches under held water.",
                         "Sommergrüner Blütenstrauch aus elf bogigen dreiteiligen Ruten mit Blütenbüscheln an den nickenden Spitzen, dessen regenschwere Pose die Bögen unter gehaltenem Wasser sinken lässt."),
    }
    assemble(spec)


REP_DECISIONS = {
    50: decisions(
        silhouette=("At 1:50 the rose is 35 mm wide and 30 mm high: eleven canes drawn at their true section rise from the root crown, arch and nod; seven leaf masses per outer cane follow the arch and 55 open blooms sit on the arches and tips, so the fountain reads with its flowers.",
                    "Bei 1:50 ist die Rose 35 mm breit und 30 mm hoch: elf Ruten in wahrem Querschnitt steigen aus dem Wurzelhals auf, neigen sich bogig und nicken; sieben Laubmassen je äußerer Rute folgen dem Bogen und 55 offene Blüten sitzen auf Bögen und Spitzen, sodass die Fontäne mit ihren Blüten liest."),
        retained=("Retained: every cane segment, the bud union, all blooms, the leaf masses along the arches, the bare base of the canes.", "Beibehalten: jedes Rutensegment, die Veredlungsstelle, alle Blüten, die Laubmassen entlang der Bögen, der kahle Fuß der Ruten."),
        merged=("Merged: the pinnate leaves (5 to 7 leaflets of 3 to 5 cm) into masses; the petals of a bloom into one cup.", "Zusammengefasst: die gefiederten Blätter (5 bis 7 Fiederblättchen von 3 bis 5 cm) zu Massen; die Kronblätter einer Blüte zu einem Becher."),
        omitted=("Omitted: prickles (5 to 8 mm), buds, side shoots inside the masses.", "Weggelassen: Stacheln (5 bis 8 mm), Knospen, Seitentriebe im Inneren der Massen."),
        thinElements=("Canes of 22, 18 and 14 mm are drawn at their true width (0.44 to 0.28 mm).", "Ruten von 22, 18 und 14 mm sind in wahrer Breite gezeichnet (0,44 bis 0,28 mm)."),
        hatching=("No hatching; three small leaflet ovals per leaf mass (0.1 mm texture pen).", "Keine Schraffur; drei kleine Fiederblättchen-Ovale je Laubmasse (Texturstift 0,1 mm)."),
        styles=("Light line: outlines with leaflets. Dark solid: mid-grey foliage, nearly white blooms, grey canes.", "Helle Linie: Umrisse mit Fiederblättchen. Dunkel flächig: mittelgraues Laub, fast weiße Blüten, graue Ruten."),
        overlays=("Thermal: leaf masses carry leaf-canopy, blooms rose-blooms, canes and crown cane-bark. Structure: one member per cane segment, the bends as joints and the tip masses.", "Thermik: Laubmassen tragen leaf-canopy, Blüten rose-blooms, Ruten und Wurzelhals cane-bark. Tragwerk: ein Stab je Rutensegment, die Knicke als Gelenke und die Spitzenmassen.")),
    100: decisions(
        silhouette=("Five leaf masses per outer cane and three blooms per cane keep the arching fountain at 17 mm width.", "Fünf Laubmassen je äußerer Rute und drei Blüten je Rute erhalten die bogige Fontäne bei 17 mm Breite."),
        retained=("Retained: all canes, 33 blooms, the open base.", "Beibehalten: alle Ruten, 33 Blüten, der offene Fuß."),
        merged=("Merged: seven leaf masses of a cane into five, five blooms into three.", "Zusammengefasst: sieben Laubmassen einer Rute zu fünf, fünf Blüten zu drei."),
        omitted=("Omitted: the smallest blooms on the arches.", "Weggelassen: die kleinsten Blüten auf den Bögen."),
        thinElements=("The tip segments are thickened from 14 to 15.6 mm to reach the 15 mm feature.", "Die Spitzensegmente sind von 14 auf 15,6 mm verstärkt, um das Merkmal von 15 mm zu erreichen."),
        hatching=("No hatching; two leaflet dashes per mass.", "Keine Schraffur; zwei Fiederblättchenstriche je Masse."),
        styles=("As at 1:50 with fewer marks.", "Wie bei 1:50 mit weniger Zeichen."),
        overlays=("Thermal regions as at 1:50; members and bend joints.", "Thermische Regionen wie bei 1:50; Stäbe und Knickgelenke.")),
    200: decisions(
        silhouette=("At 1:200 (8.7 by 7.5 mm) every cane carries three leaf masses (arch, tip, nodding end) and two bloom clusters; the canes are 0.13 mm lines.", "Bei 1:200 (8,7 mal 7,5 mm) trägt jede Rute drei Laubmassen (Bogen, Spitze, nickendes Ende) und zwei Blütenbüschel; die Ruten sind Linien von 0,13 mm."),
        retained=("Retained: the arching fountain, the open base, bloom clusters on every cane.", "Beibehalten: die bogige Fontäne, der offene Fuß, Blütenbüschel an jeder Rute."),
        merged=("Merged: the leaf masses of the arch into one, those of the tip into two; the blooms of a tip into one cluster.", "Zusammengefasst: die Laubmassen des Bogens zu einer, die der Spitze zu zwei; die Blüten einer Spitze zu einem Büschel."),
        omitted=("Omitted: leaflet texture, the base leaf masses.", "Weggelassen: Fiederblättchentextur, die unteren Laubmassen."),
        thinElements=("Canes (22 to 14 mm) are slender lines at the 0.13 mm pen; bloom clusters are 120 mm across.", "Ruten (22 bis 14 mm) sind schlanke Linien mit dem Stift von 0,13 mm; Blütenbüschel sind 120 mm breit."),
        hatching=("No hatching.", "Keine Schraffur."),
        styles=("Light line: outlines. Dark solid: grey masses with light bloom dots.", "Helle Linie: Umrisse. Dunkel flächig: graue Massen mit hellen Blütenpunkten."),
        overlays=("Thermal regions as at 1:100; one member per bone.", "Thermische Regionen wie bei 1:100; ein Stab je Knochen.")),
    500: decisions(
        silhouette=("At 1:500 (3.5 by 3 mm) each cane is an arch mass and a tip mass with one bloom cluster at its tip; the canes are lines.", "Bei 1:500 (3,5 mal 3 mm) ist jede Rute eine Bogen- und eine Spitzenmasse mit einem Blütenbüschel an der Spitze; die Ruten sind Linien."),
        retained=("Retained: the eleven arches of the fountain and the open base.", "Beibehalten: die elf Bögen der Fontäne und der offene Fuß."),
        merged=("Merged: the leaf masses of a cane into two, all its blooms into one cluster.", "Zusammengefasst: die Laubmassen einer Rute zu zwei, alle ihre Blüten zu einem Büschel."),
        omitted=("Omitted: separate tips and texture.", "Weggelassen: getrennte Spitzen und Textur."),
        thinElements=("Canes stay slender lines at the 0.13 mm pen; bloom clusters are 150 mm across, two features.", "Ruten bleiben schlanke Linien mit dem Stift von 0,13 mm; Blütenbüschel sind 150 mm breit, zwei Merkmale."),
        hatching=("No hatching.", "Keine Schraffur."),
        styles=("Light line: outlines. Dark solid: eleven grey arches with light dots.", "Helle Linie: Umrisse. Dunkel flächig: elf graue Bögen mit hellen Punkten."),
        overlays=("Thermal: masses leaf-canopy, clusters rose-blooms, canes cane-bark.", "Thermik: Massen leaf-canopy, Büschel rose-blooms, Ruten cane-bark.")),
}

VOLUME_NOTES = {
    50: [("Body: 76 leaf masses as ellipsoids along the true lines of the eleven canes, 55 blooms, 33 cane segments at 22, 18 and 14 mm and the bud union; every drawn outline is the exact projection of these solids.", "Körper: 76 Laubmassen als Ellipsoide entlang der wahren Linien der elf Ruten, 55 Blüten, 33 Rutensegmente von 22, 18 und 14 mm und die Veredlungsstelle; jeder gezeichnete Umriss ist die exakte Projektion dieser Körper."),
         ("Structure: one member per cane segment, the eleven base bends as joints, the masses of the flowering tips, the support slab.", "Tragwerk: ein Stab je Rutensegment, die elf unteren Knicke als Gelenke, die Massen der blühenden Spitzen, die Auflagerplatte.")],
    100: [("Body: 54 leaf masses, 33 blooms, canes with the tips at 15.6 mm.", "Körper: 54 Laubmassen, 33 Blüten, Ruten mit Spitzen von 15,6 mm."),
          ("Structure: members and bend joints, no masses.", "Tragwerk: Stäbe und Knickgelenke, keine Massen.")],
    200: [("Body: 33 leaf masses, 22 bloom clusters, slender canes at their true section.", "Körper: 33 Laubmassen, 22 Blütenbüschel, schlanke Ruten im wahren Querschnitt."),
          ("Structure: slender members per cane segment, the support slab.", "Tragwerk: schlanke Stäbe je Rutensegment, die Auflagerplatte.")],
    500: [("Body: 22 leaf masses (arch and tip of every cane), eleven bloom clusters of 150 mm, slender canes.", "Körper: 22 Laubmassen (Bogen und Spitze jeder Rute), elf Blütenbüschel von 150 mm, schlanke Ruten."),
          ("Structure: the root crown and the central cane as one loft, slender members for the other canes.", "Tragwerk: Wurzelhals und mittlere Rute als ein Loft, schlanke Stäbe für die übrigen Ruten.")],
}

if __name__ == "__main__":
    build()
