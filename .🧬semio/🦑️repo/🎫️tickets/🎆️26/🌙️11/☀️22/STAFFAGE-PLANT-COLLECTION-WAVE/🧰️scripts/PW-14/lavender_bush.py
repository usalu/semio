"""💜 English lavender (Lavandula angustifolia), a mature 0.6 m cushion in full flower: species data, rig, four scales of drawing and volume, poses, clips, thermal and structure.

Run: `repo tools exec python -- lavender_bush.py` (writes the catalog asset and the manifest row).
@see plant_lib.py, shrub.py
"""
import math
from plant_lib import (T, r4, Rig, revolute, FIXED, mass, round_section, Ell, Cap, Rev, Mark, Scale, representation, pens, decisions, volume, thermal_list, emitter_solid,
                       member_capsule, member_loft, support_box, write_asset, write_row, manifest_row, feature, overrides, vadd, vsub, vscale, vlen, ring, EXPORTS, leaf_region, passive_region, transpiration)
from shrub import wind_at, drag, region_bindings, load, pose, gust_clip, sway_clip, diurnal_clip, THERMAL_ASSUMPTIONS, structure_model, up_direction

ID = "lavender-bush"
REFS = [
    ("upson-andrews-2004", "Upson, T. and Andrews, S. (2004). The Genus Lavandula. Royal Botanic Gardens, Kew (Botanical Magazine Monograph). L. angustifolia: shrub to 1 m, leaves linear 2-6 cm by 1.5-6 mm, peduncles 10-30 cm, spikes 2-8 cm long, corolla violet-blue."),
    ("wood-handbook-2021", "Forest Products Laboratory (2021). Wood Handbook: Wood as an Engineering Material. General Technical Report FPL-GTR-282, USDA Forest Service. Chapter 5 (green density, modulus of elasticity and rupture; closest listed hardwood used as proxy)."),
    ("en-1991-1-4", "EN 1991-1-4:2005 Eurocode 1, Actions on structures, Part 1-4: Wind actions. Air density 1.25 kg/m3, F = 0.5 rho cd A v^2, peak velocity pressure (1 + 7 Iv) q_m."),
    ("en-1991-1-3", "EN 1991-1-3:2003 Eurocode 1, Actions on structures, Part 1-3: Snow loads. Used for the order of magnitude of a settled snow cover on the cushion."),
    ("vogel-1989", "Vogel, S. (1989). Drag and reconfiguration of broad leaves in high winds. Journal of Experimental Botany 40(8), 941-948."),
    ("jones-2014", "Jones, H. G. (2014). Plants and Microclimate, 3rd ed., Cambridge University Press. Leaf energy balance, leaf area index, boundary-layer and stomatal conductance."),
    ("monteith-unsworth-2013", "Monteith, J. L. and Unsworth, M. H. (2013). Principles of Environmental Physics, 4th ed., Academic Press. Penman-Monteith leaf transpiration."),
]

LEAF, FLOWER, BARK = "leaf-cushion", "flower-spikes", "stem-bark"
M_LEAF, M_FLOWER, M_BARK, M_WOOD = "lavender-leaf", "lavender-flower", "lavender-bark", "lavender-wood"
SECTORS = ["front", "back", "left", "right", "centre"]
NAMES = {"front": ("front", "vorderer", "vorderen"), "back": ("rear", "hinterer", "hinteren"), "left": ("left", "linker", "linken"), "right": ("right", "rechter", "rechten"), "centre": ("central", "mittlerer", "mittleren")}
JIT = [1.0, 0.93, 1.06, 0.97, 1.03, 0.95, 1.08, 0.99, 0.94, 1.05, 1.01, 0.96]


def rig():
    r = Rig()
    r.add("root-crown", None, T("Woody root crown", "Verholzter Wurzelhals"), (-0.04, 0, 0), 0, 0.08, FIXED, mass(0.45, 0.04, "authored-estimate", "wood-handbook-2021"), round_section(0.05, M_WOOD, "Solid section, d = 5 cm", "Vollquerschnitt, d = 5 cm"))
    st = revolute(-3, 3)
    cu = revolute(-6, 6)
    sp = revolute(-25, 25)
    sec = round_section(0.024, M_WOOD, "Solid section, d = 24 mm", "Vollquerschnitt, d = 24 mm")
    chains = {
        "front": ((0.015, 0, 0), (0.09, 0.12, 0), (0.17, 0.30, 0), 78, 0.24),
        "back": ((-0.015, 0, 0), (-0.09, 0.12, 0), (-0.17, 0.30, 0), 102, 0.24),
        "left": ((0, 0, 0.02), (0, 0.13, 0.02), (0, 0.30, 0.02), 90, 0.24),
        "right": ((0, 0, -0.02), (0, 0.13, -0.02), (0, 0.30, -0.02), 90, 0.24),
        "centre": ((0, 0, 0), (0, 0.17, 0), (0, 0.36, 0), 90, 0.225),
    }
    for k, (o, a, b, th, ls) in chains.items():
        en, de, dg = NAMES[k]
        d1 = vsub(a, o)
        d2 = vsub(b, a)
        r.add(f"stem-{k}", "root-crown", T(f"Woody {en} stem", f"Verholzter {de} Trieb"), o, math.degrees(math.atan2(d1[1], d1[0])), vlen(d1), st, mass(0.12, vlen(d1) / 2, "authored-estimate", "wood-handbook-2021"), sec)
        r.add(f"cushion-{k}", f"stem-{k}", T(f"{en.capitalize()} cushion sector", f"{de.capitalize()} Polstersektor"), a, math.degrees(math.atan2(d2[1], d2[0])), vlen(d2), cu, mass(0.26, vlen(d2) / 2, "authored-estimate", "jones-2014", "point"))
        so = {"left": (0, 0.30, 0.16), "right": (0, 0.30, -0.16)}.get(k, b)
        r.add(f"spikes-{k}", f"cushion-{k}", T(f"Flower stalks of the {en} sector", f"Blütenstiele des {dg} Sektors"), so, th, ls, sp, mass(0.02, ls * 0.8, "authored-estimate", "upson-andrews-2004", "point"))
    return r


SECTOR_OF = lambda a: "front" if -45 <= a < 45 else "left" if 45 <= a < 135 else "back" if a >= 135 or a < -135 else "right"


def lobe_sets(den):
    """🫧 The foliage lobes of one scale in world rest metres: (bone sector, centre, radii); every scale composes the cushion anew."""
    if den == 50:
        main = [("front", (0.19, 0.19, 0), (0.21, 0.16, 0.21)), ("back", (-0.19, 0.185, 0), (0.21, 0.155, 0.21)), ("left", (0, 0.19, 0.17), (0.21, 0.16, 0.21)), ("right", (0, 0.185, -0.17), (0.21, 0.155, 0.21)), ("centre", (0, 0.235, 0), (0.24, 0.15, 0.24)),
                ("front", (0.13, 0.17, 0.13), (0.17, 0.14, 0.17)), ("front", (0.13, 0.17, -0.13), (0.17, 0.14, 0.17)), ("back", (-0.13, 0.17, 0.13), (0.17, 0.14, 0.17)), ("back", (-0.13, 0.17, -0.13), (0.17, 0.14, 0.17))]
        tufts = []
        for k in range(18):
            a = k * 20 + 10
            rho = 0.312 * JIT[k % 12] ** 0.3
            c = (rho * math.cos(math.radians(a)), 0.12 + 0.03 * math.sin(math.radians(3 * a)), 0.95 * rho * math.sin(math.radians(a)))
            tufts.append((SECTOR_OF((a + 180) % 360 - 180), c, (0.08, 0.085 * JIT[(k + 3) % 12], 0.078)))
        for k in range(11):
            a = k * 36 + 10 if k < 10 else 0
            rho = 0.0 if k == 10 else (0.2 if k % 2 else 0.11)
            c = (rho * math.cos(math.radians(a)), 0.335 - 0.05 * (rho > 0.15) - 0.01 * (rho > 0), rho * math.sin(math.radians(a)))
            tufts.append(("centre" if rho < 0.15 else SECTOR_OF((a + 180) % 360 - 180), c, (0.065, 0.05, 0.065)))
        return main + tufts
    if den == 100:
        main = [("front", (0.188, 0.188, 0), (0.212, 0.158, 0.208)), ("back", (-0.188, 0.185, 0), (0.212, 0.155, 0.208)), ("left", (0, 0.188, 0.168), (0.208, 0.158, 0.212)), ("right", (0, 0.185, -0.168), (0.208, 0.155, 0.212)), ("centre", (0, 0.233, 0), (0.245, 0.152, 0.245)),
                ("front", (0.135, 0.165, 0.135), (0.175, 0.135, 0.175)), ("front", (0.135, 0.165, -0.135), (0.175, 0.135, 0.175)), ("back", (-0.135, 0.165, 0.135), (0.175, 0.135, 0.175)), ("back", (-0.135, 0.165, -0.135), (0.175, 0.135, 0.175))]
        tufts = []
        for k in range(8):
            a = k * 45 + 22.5
            c = (0.31 * math.cos(math.radians(a)), 0.11, 0.95 * 0.31 * math.sin(math.radians(a)))
            tufts.append((SECTOR_OF((a + 180) % 360 - 180), c, (0.08, 0.08, 0.078)))
        for k in range(4):
            a = k * 90 + 45
            tufts.append((SECTOR_OF((a + 180) % 360 - 180), (0.13 * math.cos(math.radians(a)), 0.31, 0.13 * math.sin(math.radians(a))), (0.075, 0.065, 0.075)))
        return main + tufts
    if den == 200:
        return [("front", (0.17, 0.19, 0), (0.23, 0.165, 0.25)), ("back", (-0.17, 0.185, 0), (0.23, 0.16, 0.25)), ("left", (0, 0.19, 0.15), (0.25, 0.165, 0.23)), ("right", (0, 0.185, -0.15), (0.25, 0.16, 0.23)), ("centre", (0, 0.235, 0), (0.26, 0.15, 0.26)),
                ("front", (0.2, 0.15, 0.19), (0.15, 0.12, 0.15)), ("front", (0.2, 0.15, -0.19), (0.15, 0.12, 0.15)), ("back", (-0.2, 0.15, 0.19), (0.15, 0.12, 0.15)), ("back", (-0.2, 0.15, -0.19), (0.15, 0.12, 0.15))]
    return []


def stalks(r, sector, n, radius, spike_len, spike_r, top=None):
    """🌾 The flower stalks of one sector: base points on a golden-angle disc around the sector bone origin, each leaning outwards from the plant axis, the first one along the bone axis (it carries the structural member)."""
    bone = f"spikes-{sector}"
    o, th, length = r.W[bone]
    axis = (math.cos(math.radians(th)), math.sin(math.radians(th)), 0)
    out = []
    phase = {"front": 0, "back": 90, "left": 45, "right": 135, "centre": 20}[sector]
    for i in range(n):
        if i == 0:
            base, d = o, axis
            L = length if top is None else top
        else:
            rho = radius * math.sqrt(i / (n - 1))
            a = math.radians(i * 137.508 + phase)
            base = (o[0] + rho * math.cos(a), o[1] - 0.012 * rho / radius, o[2] + rho * math.sin(a))
            hx, hz = base[0], base[2]
            hr = math.hypot(hx, hz)
            lean = math.radians(min(34, 9 + 60 * hr))
            ux, uz = (hx / hr, hz / hr) if hr > 1e-6 else (1, 0)
            d = (math.sin(lean) * ux, math.cos(lean), math.sin(lean) * uz)
            L = length * (0.78 + 0.18 * JIT[i % 12] - 0.12 * (rho / radius))
            reach = max(abs(base[0]), abs(base[2]))
            cap = 0.372 - reach
            if L * math.sin(lean) > cap:
                L = cap / max(math.sin(lean), 1e-6)
        tip = vadd(base, vscale(d, L - spike_len))
        end = vadd(base, vscale(d, L - spike_r))
        out.append((bone, base, tip, end, d))
    return out


def scale_model(r, den, dims=None):
    s = Scale(den, r)
    tone_leaf, tone_flower, tone_bark = 0.42, 0.72, 0.3
    if den == 500:
        s.add(Rev("cushion", "cushion-centre", (0, 0, 0), [(0.02, 0), (0.02, 0.30), (0.06, 0.385), (0.12, 0.4), (0.2, 0.39), (0.28, 0.34), (0.34, 0.25), (0.375, 0.14), (0.385, 0)], lateral=0.95, material=M_LEAF, role="body", tone=tone_leaf, region=LEAF))
        for k in SECTORS:
            o, th, L = r.W[f"spikes-{k}"]
            out = {"left": (0, 0.0, 0.07), "right": (0, 0, -0.07)}.get(k, (0, 0, 0))
            top = vadd(r.tip(f"spikes-{k}"), out) if k != "centre" else (0, 0.56, 0)
            top = (top[0] * 0.92, min(top[1], 0.53) if k != "centre" else top[1], top[2] * 0.92)
            s.add(Cap(f"flower-tuft-{k}", f"spikes-{k}", o, top, 0.04, 0.04, material=M_FLOWER, role="body", tone=tone_flower, region=FLOWER))
        s.add(Cap("woody-frame", "stem-centre", (0, 0.04, 0), (0, 0.29, 0), 0.04, 0.04, material=M_BARK, role="body", tone=tone_bark, region=BARK))
        return s
    lobes = lobe_sets(den)
    hosts = []
    for i, (sector, c, rad) in enumerate(lobes):
        main = i < 9
        hosts.append(Ell(f"{'lobe' if main else 'tuft'}-{i + 1:02d}", f"cushion-{sector}", c, rad, material=M_LEAF, role="body", tone=tone_leaf + (0.04 if not main else 0), region=LEAF, z=1.0 + 0.01 * i))
        s.add(hosts[-1])
    collar_r = {50: 0.045, 100: 0.046, 200: 0.05}[den]
    s.add(Rev("root-collar", "root-crown", (0, 0, 0), [(0, 0), (0, collar_r), (0.03, collar_r * 0.92), (0.055, collar_r * 0.6), (0.06, 0)], material=M_BARK, role="body", tone=tone_bark, region=BARK, z=0.5))
    stem_r = {50: 0.012, 100: 0.0125, 200: 0.016}[den]
    tips = {"front": (0.105, 0.16, 0), "back": (-0.105, 0.16, 0), "left": (0, 0.16, 0.12), "right": (0, 0.16, -0.12), "centre": (0, 0.2, 0)}
    for sector in (SECTORS if den != 200 else ["front", "back", "centre"]):
        base = r.W[f"stem-{sector}"][0]
        tip = tips[sector] if den != 200 else {"front": (0.12, 0.17, 0.04), "back": (-0.12, 0.17, -0.04), "centre": (0, 0.21, 0)}[sector]
        s.add(Cap(f"stem-{sector}", f"stem-{sector}", (base[0], stem_r, base[2]), tip, stem_r, stem_r * 0.75, material=M_BARK, role="body", tone=tone_bark, region=BARK, z=0.6))
    n, radius, spike_len, spike_r = {50: (11, 0.075, 0.055, 0.0055), 100: (5, 0.07, 0.06, 0.0085), 200: (3, 0.065, 0.075, 0.016)}[den]
    k = 0
    for sector in SECTORS:
        for j, (bone, base, tip, end, d) in enumerate(stalks(r, sector, n, radius, spike_len, spike_r, top=centre_top(r, spike_r) if sector == "centre" else None)):
            k += 1
            s.add(Cap(f"stalk-{k:02d}", bone, base, tip, 0.0015, 0.0015, material=M_FLOWER, role="detail", tone=0.5, region=FLOWER, slender=True, stroke=True, z=2.0))
            s.add(Cap(f"spike-{k:02d}", bone, tip, end, spike_r, spike_r, material=M_FLOWER, role="body", tone=tone_flower, region=FLOWER, z=2.2))
    if den in (50, 100):
        texture(s, lobes, den, hosts)
    return s


def centre_top(r, spike_r):
    o, th, L = r.W["spikes-centre"]
    return 0.6 - o[1]


def texture(s, lobes, den, hosts):
    """🍃 Leaf marks: short rising dashes of the linear grey leaves inside the main lobes (side and front) and radial dashes in plan; fewer and longer at 1:100."""
    per = 4 if den == 50 else 2
    length = 0.045 if den == 50 else 0.06
    m = 0
    for i, (sector, c, rad) in enumerate(lobes[:9]):
        bone = f"cushion-{sector}"
        for j in range(per):
            f = (j + 0.5) / per
            for view in ("elevation", "front", "plan"):
                if view == "elevation":
                    cx, cy, rx, ry = c[0], c[1], rad[0], rad[1]
                elif view == "front":
                    cx, cy, rx, ry = c[2], c[1], rad[2], rad[1]
                else:
                    cx, cy, rx, ry = c[0], c[2], rad[0], rad[2]
                if view == "plan":
                    a = 2 * math.pi * (f + 0.13 * i)
                    p = (cx + 0.45 * rx * math.cos(a), cy + 0.45 * ry * math.sin(a))
                    q = (p[0] + length * 0.7 * math.cos(a), p[1] + length * 0.7 * math.sin(a))
                else:
                    x = cx + (f - 0.5) * 1.1 * rx
                    y = cy + 0.15 * ry - 0.25 * ry * abs(f - 0.5)
                    lean = (f - 0.5) * 0.9
                    p = (x, y)
                    q = (x + length * math.sin(lean), y + length * math.cos(lean))
                m += 1
                s.mark(Mark(f"leaf-mark-{m:03d}", bone, view, [p, q], hosts[i], role="texture"))


def regions(bark_area):
    return [
        {**leaf_region(LEAF, T("Grey-green linear leaves of the cushion", "Graugrüne lineale Blätter des Polsters"), M_LEAF, 1.3, 0.004, "cylinder", 0.5, 700, 0.004, 2.7, "c3", "jones-2014"), "areaBasis": "exchange"},
        {**leaf_region(FLOWER, T("Flower spikes and their green stalks", "Blütenähren und ihre grünen Stiele"), M_FLOWER, 0.12, 0.011, "cylinder", 0.8, 600, 0.0012, 0.25, "c3", "upson-andrews-2004"), "areaBasis": "exchange"},
        passive_region(BARK, T("Woody stems and root crown", "Verholzte Triebe und Wurzelhals"), "bark", M_BARK, bark_area, 0.024, "cylinder", 0.05, 16000, "wood-handbook-2021"),
    ]


def anchors(r):
    out = [{"id": "base-support", "bone": "root-crown", "x": 0.04, "y": 0, "kind": "support", "title": T("Planting hole anchorage", "Verankerung im Pflanzloch"), "support": "fixed", "normalDeg": 90, "footprintLengthM": 0.3, "footprintWidthM": 0.3, "frictionCoefficient": 0.6}]
    lobe_centre = {"front": (0.2, 0.2, 0), "back": (-0.2, 0.2, 0), "left": (0, 0.2, 0.18), "right": (0, 0.2, -0.18), "centre": (0, 0.3, 0)}
    for k in SECTORS:
        x, y, z = r.local(f"cushion-{k}", lobe_centre[k])
        out.append({"id": f"tp-cushion-{k}", "bone": f"cushion-{k}", "x": r4(x), "y": r4(y), **({"z": r4(z)} if abs(z) > 1e-9 else {}), "kind": "emitter", "title": T(f"Transpiration of the {NAMES[k][0]} cushion sector", f"Transpiration des {NAMES[k][2]} Polstersektors")})
    for k in SECTORS:
        out.append({"id": f"tp-spikes-{k}", "bone": f"spikes-{k}", "x": 0.2, "y": 0, "kind": "emitter", "title": T(f"Transpiration of the flower spikes of the {NAMES[k][0]} sector", f"Transpiration der Blütenähren des {NAMES[k][2]} Sektors")})
    return out


def build():
    r = rig()
    scales = {d: scale_model(r, d) for d in (50, 100, 200, 500)}
    lo, hi = scales[50].extent()
    dims = {"heightM": r4(hi[1] - max(0, lo[1])), "widthM": r4(hi[0] - lo[0]), "depthM": r4(hi[2] - lo[2])}
    for d, sc in scales.items():
        a, b = sc.extent()
        print(f"[DEBUG] 1:{d} extent x {a[0]:.3f}..{b[0]:.3f} y {a[1]:.3f}..{b[1]:.3f} z {a[2]:.3f}..{b[2]:.3f}")
    areas = overrides(ID)
    bark_area = areas.get("stem-bark", 0.085)
    an = anchors(r)
    anchor_by = {a["id"]: a for a in an}
    total_mass = round(sum(b["mass"]["kg"] for b in r.bones), 3)
    leaf_emit = [f"tp-cushion-{k}" for k in SECTORS]
    flower_emit = [f"tp-spikes-{k}" for k in SECTORS]
    emitters = [transpiration("transpiration-cushion", T("Transpiration of the leaf cushion (distributed over the mound)", "Transpiration des Blattpolsters (über den Hügel verteilt)"), leaf_emit, LEAF, up_direction(r, "cushion-front"), 0.3),
                transpiration("transpiration-spikes", T("Transpiration of the flower spikes (distributed over the flowering crown)", "Transpiration der Blütenähren (über die Blütenkrone verteilt)"), flower_emit, FLOWER, up_direction(r, "spikes-front"), 0.25)]
    regs = regions(bark_area)
    reps, vols = {}, {}
    for d, sc in scales.items():
        f = feature(d)
        er = max(0.6 * f, {50: 0.02, 100: 0.024, 200: 0.03, 500: 0.045}[d])
        em = [emitter_solid(r, f"th-{a}", anchor_by[a], LEAF if "cushion" in a else FLOWER, "transpiration-cushion" if "cushion" in a else "transpiration-spikes", er, M_LEAF if "cushion" in a else M_FLOWER) for a in leaf_emit + flower_emit]
        vols[str(d)] = volume(d, sc.body(), thermal_list(sc, em), structure_list(r, d), VOLUME_NOTES[d])
        if d != 500:
            reps[str(d)] = representation(d, *RANGES[d], PENS[d], [], sc.geometry(), REP_DECISIONS[d])
    reps["500"] = {"status": "unavailable", "denominator": 500, "cause": "below-printable-feature",
                   "reason": T("A lavender cushion of 0.8 m by 0.6 m is 1.6 mm by 1.2 mm at 1:500: a dot. Its flower spikes (11 mm thick, 0.02 mm on paper) and the scalloped rim of the cushion cannot be told apart, so the plan symbol lavender-bush-symbol at true extent stands in.",
                               "Ein Lavendelpolster von 0,8 m mal 0,6 m misst bei 1:500 1,6 mm mal 1,2 mm: ein Punkt. Seine Blütenähren (11 mm dick, 0,02 mm auf dem Papier) und der gewellte Polsterrand sind nicht unterscheidbar; daher tritt das Grundrisssymbol lavender-bush-symbol in wahrer Ausdehnung an seine Stelle."),
                   "limitingFeature": T("Flower spike (11 mm thick)", "Blütenähre (11 mm dick)"), "limitingSizeM": 0.011, "alternatives": [{"kind": "symbol", "symbol": "lavender-bush-symbol"}, {"kind": "scale", "denominator": 200}]}
    asset = {
        "id": ID, "category": "vegetation", "title": T("English Lavender", "Echter Lavendel"),
        "summary": T(f"A mature English lavender (Lavandula angustifolia) in full July flower: a dense, grey-green cushion of linear leaves {dims['widthM']:.2f} m across and 0.38 m high on a woody root crown, crowned by about fifty-five slender flower stalks that fan outwards and carry violet spikes up to {dims['heightM']:.2f} m; the evergreen subshrub of herb gardens, gravel beds and Mediterranean plantings.",
                     f"Ein ausgewachsener Echter Lavendel (Lavandula angustifolia) in voller Juliblüte: ein dichtes, graugrünes Polster aus linealen Blättern, {dims['widthM']:.2f} m breit und 0,38 m hoch, auf einem verholzten Wurzelhals, gekrönt von etwa fünfundfünfzig schlanken Blütenstielen, die sich fächerförmig nach außen neigen und violette Ähren bis {dims['heightM']:.2f} m tragen; der immergrüne Halbstrauch der Kräutergärten, Kiesbeete und mediterranen Pflanzungen."),
        "tags": ["shrub", "subshrub", "evergreen", "herb", "flowering", "mediterranean", "wind", "transpiration"],
        "provenance": {"author": "Semio staffage plant worker PW-14", "method": "handcrafted", "created": "2026-11-22", "references": [{"id": i, "citation": c} for i, c in REFS],
                       "notes": T("Five woody stems carry five cushion sectors and five flower-stalk groups; the stalks are modelled at 1:50 one by one (eleven per sector), at 1:100 five and at 1:200 three per sector with thickened spikes. Lavender keeps its leaves in winter, so the snow pose keeps the cushion and the dry stalks of last summer; a pruned geometry is not offered because shapes cannot be toggled by a pose.",
                                  "Fünf verholzte Triebe tragen fünf Polstersektoren und fünf Blütenstielgruppen; die Stiele sind bei 1:50 einzeln (elf je Sektor), bei 1:100 zu fünft und bei 1:200 zu dritt je Sektor mit verstärkten Ähren modelliert. Lavendel behält im Winter sein Laub, daher behält die Schneepose das Polster und die trockenen Stiele des letzten Sommers; eine geschnittene Geometrie wird nicht angeboten, da Formen durch eine Pose nicht ausgeblendet werden können.")},
        "dimensions": {**dims, "massKg": total_mass},
        "materials": [
            {"id": M_LEAF, "title": T("Grey tomentose lavender leaves", "Graufilzige Lavendelblätter"), "emissivity": 0.96, "solarAbsorptivity": 0.5, "source": "authored-estimate", "reference": "jones-2014"},
            {"id": M_FLOWER, "title": T("Violet lavender spikes with green stalks", "Violette Lavendelähren mit grünen Stielen"), "emissivity": 0.95, "solarAbsorptivity": 0.72, "source": "authored-estimate", "reference": "jones-2014"},
            {"id": M_BARK, "title": T("Grey-brown shredding lavender bark", "Graubraune, faserig abblätternde Lavendelrinde"), "emissivity": 0.93, "solarAbsorptivity": 0.7, "densityKgM3": 600, "conductivityWmK": 0.12, "source": "authored-estimate", "reference": "wood-handbook-2021"},
            {"id": M_WOOD, "title": T("Green lavender wood", "Frisches Lavendelholz"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 900, "conductivityWmK": 0.16, "elasticModulusPa": 6000000000, "strengthPa": 50000000, "source": "authored-estimate", "reference": "wood-handbook-2021"},
        ],
        "rig": {"kind": "botanical", "root": "root-crown", "bones": r.bones},
        "anchors": an,
        "poses": poses(r),
        "clips": clips(r),
        "thermal": {"environment": {"airTempC": 26, "meanRadiantTempC": 30, "relativeHumidity": 0.45, "windMS": 1.5, "windDirectionDeg": 0, "solarWM2": 800, "pressurePa": 101325, "diffuseFraction": 0.15},
                    "regions": regs, "emitters": emitters, "assumptions": THERMAL_ASSUMPTIONS, "operating": [], "passengers": []},
        "structure": structure_model(("One rigid body per evaluated pose, fixed at the planting hole; wind drag acts at the centre of each cushion sector and at two thirds of each stalk group, snow on the cushion sectors, gravity on all bone masses. The low cushion carries base moments of a few N m even in a gale. Root and soil failure, stalk buckling and dynamic amplification are not modelled.",
                                 "Ein starrer Körper je ausgewerteter Pose, im Pflanzloch eingespannt; Winddruck wirkt in der Mitte jedes Polstersektors und auf zwei Dritteln jeder Stielgruppe, Schnee auf den Polstersektoren, Schwerkraft auf allen Knochenmassen. Das niedrige Polster trägt selbst im Sturm Fußmomente von wenigen N m. Wurzel- und Bodenversagen, Knicken der Stiele und dynamische Überhöhung werden nicht modelliert.")),
        "volumes": vols,
        "presentation": {"defaultPose": "calm", "defaultClip": "wind-sway", "defaultScale": 50},
        "exports": EXPORTS,
        "completion": "authored",
        "representations": reps,
        "symbols": [symbol(r, dims)],
    }
    path = write_asset(asset)
    write_row(manifest_row(asset, T("A low evergreen subshrub: a grey leaf cushion on a woody crown with a fanned crown of single flower stalks and spikes, whose stalks, not the mass, carry the wind sway.",
                                    "Niedriger immergrüner Halbstrauch: ein graues Blattpolster auf verholztem Wurzelhals mit einer Fächerkrone einzelner Blütenstiele und Ähren, deren Stiele, nicht die Masse, die Windschwingung tragen.")))
    print(path, dims, total_mass)


RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350)}
PENS = {50: pens(0.2, 0.13, 0.1, None, 0.3), 100: pens(0.18, 0.13, 0.1, None, 0.25), 200: pens(0.15, 0.13, None, None, 0.25)}


def structure_list(r, d):
    """🏗️ The structural idealisation per scale: one member per bone at 1:50 and 1:100 (with the cushion forks as joints at 1:50), the front, rear and central chains as one loft each at 1:200, and at 1:500 the central frame loft holding root crown, central stem, cushion and stalks."""
    f = feature(d)
    tips = {"front": (0.105, 0.16, 0), "back": (-0.105, 0.16, 0), "left": (0, 0.16, 0.12), "right": (0, 0.16, -0.12), "centre": (0, 0.2, 0)}
    tops = {"front": (0.17, 0.30, 0), "back": (-0.17, 0.30, 0), "left": (0, 0.30, 0.16), "right": (0, 0.30, -0.16), "centre": (0, 0.36, 0)}
    out = [support_box(r, "root-crown", "base-support", (0.15 if d == 500 else 0.12, f, 0.15 if d == 500 else 0.12), (0, -f / 2, 0), M_WOOD)]

    def stalk_end(k, share=0.85):
        so = r.W[f"spikes-{k}"][0]
        return vadd(so, vscale(vsub(r.tip(f"spikes-{k}"), so), share))

    def chain(k):
        o = r.W[f"stem-{k}"][0]
        return [(o[0], 0.0, o[2]), r.tip(f"stem-{k}"), r.tip(f"cushion-{k}"), stalk_end(k, 1.0)]

    if d in (50, 100):
        out.append(member_capsule(r, "member-root-crown", "root-crown", (-0.03, 0.022, 0), (0.03, 0.022, 0), max(0.022, 0.55 * f), M_WOOD))
        for k in SECTORS:
            o = r.W[f"stem-{k}"][0]
            rs = max(0.011, 0.55 * f)
            out.append(member_capsule(r, f"member-stem-{k}", f"stem-{k}", (o[0], rs, o[2]), tips[k], rs, M_WOOD))
            out.append(member_capsule(r, f"member-cushion-{k}", f"cushion-{k}", tips[k], tops[k], max(0.008, 0.55 * f), M_WOOD))
            out.append(member_capsule(r, f"member-spikes-{k}", f"spikes-{k}", r.W[f"spikes-{k}"][0], stalk_end(k), 0.0035 if d == 50 else 0.005, M_WOOD, slender=True))
        if d == 50:
            for k in SECTORS:
                out.append({"id": f"joint-cushion-{k}", "kind": "ellipsoid", "bone": f"cushion-{k}", "material": M_WOOD, "role": "detail", "tone": 0.3, "radii": [0.009] * 3, "transform": {"translation": [r4(x) for x in r.local(f"cushion-{k}", tips[k])]}, "member": f"cushion-{k}", "part": "joint"})
        return out
    if d == 200:
        out.append(member_capsule(r, "member-root-crown", "root-crown", (-0.03, 0.022, 0), (0.03, 0.022, 0), 0.022, M_WOOD))
        for k in ("front", "back", "centre"):
            out.append(member_loft(r, f"member-chain-{k}", f"stem-{k}", chain(k), 0.016, M_WOOD, members=[f"cushion-{k}", f"spikes-{k}"]))
        for k in ("left", "right"):
            o = r.W[f"stem-{k}"][0]
            out.append(member_capsule(r, f"member-stem-{k}", f"stem-{k}", (o[0], 0.0165, o[2]), tips[k], 0.0165, M_WOOD))
            out.append(member_capsule(r, f"member-cushion-{k}", f"cushion-{k}", tips[k], tops[k], 0.0165, M_WOOD))
            out.append(member_capsule(r, f"member-spikes-{k}", f"spikes-{k}", r.W[f"spikes-{k}"][0], stalk_end(k), 0.005, M_WOOD, slender=True))
        return out
    out.append(member_loft(r, "member-frame", "stem-centre", [(0, 0.0, 0), (0, 0.33, 0), (0, 0.53, 0)], 0.04, M_WOOD, members=["root-crown", "cushion-centre", "spikes-centre"]))
    for k in ("front", "back"):
        out.append(member_loft(r, f"member-chain-{k}", f"stem-{k}", chain(k), 0.04, M_WOOD, members=[f"cushion-{k}", f"spikes-{k}"]))
    for k in ("left", "right"):
        out.append(member_capsule(r, f"member-sector-{k}", f"stem-{k}", (0, 0.06, 0), vadd(tops[k], (0, 0.02, 0)), 0.04, M_WOOD, members=[f"cushion-{k}"]))
        so = r.W[f"spikes-{k}"][0]
        out.append(member_capsule(r, f"member-spikes-{k}", f"spikes-{k}", so, vadd(so, (0, 0.17, 0)), 0.006, M_WOOD, slender=True))
    return out


def poses(r):
    rg = [LEAF, FLOWER, BARK]
    sun = {LEAF: 0.55, FLOWER: 0.85, BARK: 0.03}
    em = ["transpiration-cushion", "transpiration-spikes"]
    cush_area, stalk_area = 0.046, 0.0127
    lobe_x = {k: r4(r.W[f"cushion-{k}"][2] * 0.6) for k in SECTORS}

    def winds(v10, cd_c, cd_s, src, ref):
        out = []
        for k in SECTORS:
            vc, vs = wind_at(v10, 0.22), wind_at(v10, 0.45)
            out.append(load(f"wind-cushion-{k}", "wind", T(f"Wind drag on the {NAMES[k][0]} cushion sector", f"Winddruck auf den {NAMES[k][2]} Polstersektor"), f"cushion-{k}", lobe_x[k], 0, drag(cd_c, cush_area, vc), 0, src, ref))
            out.append(load(f"wind-spikes-{k}", "wind", T(f"Wind drag on the flower stalks of the {NAMES[k][0]} sector", f"Winddruck auf die Blütenstiele des {NAMES[k][2]} Sektors"), f"spikes-{k}", 0.16, 0, drag(cd_s, stalk_area, vs), 0, src, ref))
        return out

    def lean(stem, cushion, spikes):
        b = {}
        for k in SECTORS:
            b[f"stem-{k}"] = {"angleDeg": stem}
            b[f"cushion-{k}"] = {"angleDeg": cushion}
            b[f"spikes-{k}"] = {"angleDeg": spikes}
        return b

    calm = pose("calm", T("Calm July day in full flower", "Windstiller Julitag in voller Blüte"),
                T("Light air of 1.5 m/s: the cushion and the fanned stalks stand still; the drag on a cushion sector is below 0.1 N.", "Leiser Zug von 1,5 m/s: Polster und gefächerte Stiele stehen still; der Widerstand auf einem Polstersektor liegt unter 0,1 N."),
                {}, region_bindings(rg, sun), {e: 0.85 for e in em}, winds(1.5 / wind_at(1, 0.3), 0.8, 1.0, "literature", "en-1991-1-4"))
    windy = pose("windy", T("Fresh breeze", "Frische Brise"),
                 T("Beaufort 5, 8 m/s at 10 m, which is 3.7 m/s at the cushion and 4.2 m/s at the spikes; cd 0.6 on 0.046 m2 per sector and 1.0 on 0.013 m2 per stalk group. The stalks bend 9 degrees downwind, the cushion 2 degrees.",
                   "Beaufort 5, 8 m/s in 10 m Höhe, das sind 3,7 m/s am Polster und 4,2 m/s an den Ähren; cd 0,6 auf 0,046 m2 je Sektor und 1,0 auf 0,013 m2 je Stielgruppe. Die Stiele biegen sich um 9 Grad nach Lee, das Polster um 2 Grad."),
                 lean(-0.8, -2, -9), region_bindings(rg, sun), {e: 1 for e in em}, winds(8, 0.6, 1.0, "literature", "en-1991-1-4"))
    storm = pose("storm", T("Gale", "Sturm"),
                 T("Beaufort 9, 22 m/s at 10 m (10 to 12 m/s at the plant), cd 0.45 on the cushion after leaf reconfiguration and 0.8 on the stalks: the stalks lay over by 20 degrees, the cushion by 4 degrees; the base moment stays below 10 N m.",
                   "Beaufort 9, 22 m/s in 10 m Höhe (10 bis 12 m/s an der Pflanze), cd 0,45 am Polster nach Blattumformung und 0,8 an den Stielen: die Stiele legen sich um 20 Grad um, das Polster um 4 Grad; das Fußmoment bleibt unter 10 N m."),
                 lean(-1.6, -4, -20), region_bindings(rg, {LEAF: 0.3, FLOWER: 0.4, BARK: 0}, {LEAF: 0.4, FLOWER: 0.4, BARK: 0.3}), {e: 0.5 for e in em}, winds(22, 0.45, 0.8, "literature", "vogel-1989"))
    snow_b = {"cushion-front": {"angleDeg": -4}, "cushion-back": {"angleDeg": 4}, "spikes-front": {"angleDeg": -15}, "spikes-back": {"angleDeg": 15}, "spikes-centre": {"angleDeg": -5}, "spikes-left": {"angleDeg": 6}, "spikes-right": {"angleDeg": -6}}
    snow_loads = [load(f"snow-cushion-{k}", "snow", T(f"Snow on the {NAMES[k][0]} cushion sector", f"Schnee auf dem {NAMES[k][2]} Polstersektor"), f"cushion-{k}", lobe_x[k], 0, 0, -29, "authored-estimate", "en-1991-1-3") for k in SECTORS]
    snow = pose("snow", T("Snow cap in late winter", "Schneehaube im Spätwinter"),
                T("A settled snow cover of 0.3 kN/m2 on the 0.48 m2 plan area of the evergreen cushion, 29 N per sector (EN 1991-1-3 order of magnitude, authored estimate); the dry stalks of last summer are pressed outwards by up to 15 degrees, the cushion sectors sag 4 degrees; calm.",
                  "Eine gesetzte Schneedecke von 0,3 kN/m2 auf der 0,48 m2 großen Grundfläche des immergrünen Polsters, 29 N je Sektor (Größenordnung nach EN 1991-1-3, Schätzung); die trockenen Stiele des letzten Sommers werden um bis zu 15 Grad nach außen gedrückt, die Polstersektoren sinken um 4 Grad; Windstille."),
                snow_b, region_bindings(rg, {LEAF: 0, FLOWER: 0.1, BARK: 0}, {LEAF: 0.5, FLOWER: 0.3, BARK: 0.3}), {e: 0 for e in em}, snow_loads)
    return [calm, windy, storm, snow]


def clips(r):
    windy_base = {}
    for k in SECTORS:
        windy_base.update({f"stem-{k}": -0.8, f"cushion-{k}": -2, f"spikes-{k}": -9})
    ph = {"front": 0, "back": 70, "left": 140, "right": 210, "centre": 280}
    specs = {}
    for k in SECTORS:
        specs[f"stem-{k}"] = (0.3, ph[k], 1)
        specs[f"cushion-{k}"] = (1.0, ph[k] + 20, 1)
        specs[f"spikes-{k}"] = (4.0, ph[k] + 40, 2)
    sway = sway_clip(r, 1.2, 12, windy_base, specs, T("A deterministic 1.2 s cycle about the fresh-breeze lean: the cushion sectors rock by 1 degree once per cycle (estimated 0.8 Hz), the light flower stalks whip by 4 degrees twice per cycle (1.7 Hz) with a phase that travels round the plant.",
                                                     "Ein deterministischer Zyklus von 1,2 s um die Neigung der frischen Brise: die Polstersektoren wiegen sich einmal je Zyklus um 1 Grad (geschätzt 0,8 Hz), die leichten Blütenstiele peitschen zweimal je Zyklus um 4 Grad (1,7 Hz) mit einer um die Pflanze wandernden Phase."),
                      ["transpiration-cushion", "transpiration-spikes"])
    wind_ids = [{"id": f"wind-cushion-{k}"} for k in SECTORS] + [{"id": f"wind-spikes-{k}"} for k in SECTORS]
    peak = {}
    for k in SECTORS:
        peak.update({f"stem-{k}": -1.1, f"cushion-{k}": -2.8, f"spikes-{k}": -12})
    gust = gust_clip(None, wind_ids, peak, T("Gust", "Windböe"),
                     T("A single 8 s gust: the wind-load tracks rise from 5 % to 2.75 times the fresh-breeze load at 2.5 s (EN 1991-1-4 peak pressure with Iv 0.25) and decay; the stalks lean up to 21 degrees from upright, the cushion up to 5 degrees.",
                       "Eine einzelne Böe über 8 s: die Windlast-Spuren steigen bei 2,5 s von 5 % auf das 2,75-fache der Last der frischen Brise (Spitzendruck nach EN 1991-1-4 mit Iv 0,25) und klingen ab; die Stiele neigen sich bis zu 21 Grad aus der Senkrechten, das Polster bis zu 5 Grad."))
    day = diurnal_clip({LEAF: 0.55, FLOWER: 0.85, BARK: 0.03}, ["transpiration-cushion", "transpiration-spikes"],
                       T("A 24 h July cycle compressed to 24 s (1 s = 1 h), calm weather. The sunlit fraction of every region and the transpiration strength follow the sun; leaf, spike and bark temperatures are not authored but follow from the thermal solver.",
                         "Ein 24-h-Julizyklus auf 24 s gerafft (1 s = 1 h) bei ruhigem Wetter. Der besonnte Anteil jeder Region und die Transpirationsstärke folgen der Sonne; Blatt-, Ähren- und Rindentemperaturen werden nicht vorgegeben, sondern folgen aus dem thermischen Löser."))
    return [sway, gust, day]


def symbol(r, dims):
    """🔣 The true-extent plan sign of the planting plan with the matching side and front outlines (the alternative of the unavailable 1:500 drawing)."""
    w, h, dz = dims["widthM"] / 2, dims["heightM"], dims["depthM"] / 2
    jag = [1.0, 0.94, 1.0, 0.95, 0.99, 0.93]
    plan = ring(0, 0, w, dz, 24, jitter=[1.0, 0.95, 0.99, 0.94])
    side = [(-w, 0.0), (-w * 0.98, 0.17), (-w * 0.82, 0.3), (-w * 0.6, 0.36)]
    for i in range(9):
        x = -w * 0.55 + i * w * 1.1 / 8
        side.append((x, h * (0.9 if i % 2 else 1.0) * (1 - 0.1 * abs(x) / w)))
    side += [(w * 0.6, 0.36), (w * 0.82, 0.3), (w * 0.98, 0.17), (w, 0.0)]
    front = [(z * dz / w, y) for z, y in side]
    sx = max(abs(y) for _, y in side)
    side = [(x, y * h / sx) for x, y in side]
    front = [(x, y * h / sx) for x, y in front]

    def shp(id, view, pts, role="body", closed=True, z=2):
        loc = [r.local("root-crown", (p[0], p[1], 0))[:2] for p in pts] if view == "elevation" else pts
        return {"id": id, "bone": "root-crown", "role": role, "z": z, "filled": False, "tone": 0.5, "kind": "poly", "points": [[r4(x), r4(y)] for x, y in loc], "closed": closed}
    geo = {"elevation": [shp("crown", "elevation", side)], "front": [shp("crown", "front", front)],
           "plan": [shp("crown", "plan", plan), shp("heart", "plan", ring(0, 0, 0.05, 0.05, 12), role="detail", z=3)]}
    return {"id": "lavender-bush-symbol", "title": T("Plan symbol with true cushion extent", "Grundrisssymbol mit wahrer Polsterausdehnung"),
            "description": T("A plan sign for planting plans: the lobed outline of the cushion at its true extent with the woody crown as a small circle; side and front show the cushion with its flower fringe. The replacement for the unavailable 1:500 drawing.",
                             "Ein Grundrisszeichen für Pflanzpläne: der gelappte Umriss des Polsters in wahrer Ausdehnung mit dem verholzten Wurzelhals als kleinem Kreis; Seiten- und Vorderansicht zeigen das Polster mit seinem Blütensaum. Der Ersatz für die nicht verfügbare Zeichnung 1:500."),
            "semantics": "true-extent", "units": "model-m", "denominators": [500, 200], "pens": pens(0.18, 0.13, None, None, 0.25), "hatches": [], "geometry": geo,
            "dimensionalNote": T(f"A conventional simplification of the plant drawn at its true extent in model metres in all three views ({dims['widthM']} m wide, {dims['depthM']} m deep and {dims['heightM']} m high): the outline is the cushion with its flower fringe, the circle the woody crown. It is a symbol, never used silently for a pictorial scale.",
                                 f"Eine konventionelle Vereinfachung der Pflanze in ihrer wahren Ausdehnung in Modellmetern in allen drei Ansichten ({dims['widthM']} m breit, {dims['depthM']} m tief und {dims['heightM']} m hoch): der Umriss ist das Polster mit seinem Blütensaum, der Kreis der verholzte Wurzelhals. Es ist ein Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet.")}


REP_DECISIONS = {
    50: decisions(
        silhouette=("The cushion is composed of nine overlapping leaf lobes with twelve rim tufts and seven crown tufts, so its outline is scalloped like a clipped-back lavender; above it 55 single flower stalks fan outwards and end in violet spikes 55 mm long. At 1:50 the plant is 16 mm wide and 12 mm high.",
                    "Das Polster ist aus neun überlappenden Blattlappen mit achtzehn Randbüscheln und elf Kronenbüscheln komponiert, sodass sein Umriss gewellt ist wie bei einem zurückgeschnittenen Lavendel; darüber neigen sich 55 einzelne Blütenstiele fächerförmig nach außen und enden in 55 mm langen violetten Ähren. Bei 1:50 ist die Pflanze 16 mm breit und 12 mm hoch."),
        retained=("Retained: every flower stalk and spike, the lobes and tufts of the cushion, the five woody stems and the root collar (hidden by the cushion in the side and front views).", "Beibehalten: jeder Blütenstiel und jede Ähre, die Lappen und Büschel des Polsters, die fünf verholzten Triebe und der Wurzelhals (in Seiten- und Vorderansicht vom Polster verdeckt)."),
        merged=("Merged: the single linear leaves (2 to 5 cm) into lobes and tufts; the whorls of flowers of a spike into one spike outline.", "Zusammengefasst: die einzelnen linealen Blätter (2 bis 5 cm) zu Lappen und Büscheln; die Blütenquirle einer Ähre zu einem Ährenumriss."),
        omitted=("Omitted: single leaves, the bracts and the corolla lips (1 cm, 0.2 mm on paper).", "Weggelassen: Einzelblätter, die Tragblätter und die Kronlippen (1 cm, 0,2 mm auf dem Papier)."),
        thinElements=("The 3 mm stalks are drawn as single 0.13 mm lines (6.5 mm in the model), the 11 mm spikes at their true width (0.22 mm).", "Die 3 mm dünnen Stiele werden als einfache Linien von 0,13 mm gezeichnet (6,5 mm im Modell), die 11 mm dicken Ähren in wahrer Breite (0,22 mm)."),
        hatching=("No hatching; four short leaf dashes per lobe (0.1 mm texture pen) give the linear leaves.", "Keine Schraffur; vier kurze Blattstriche je Lappen (Texturstift 0,1 mm) geben die linealen Blätter wieder."),
        styles=("Light line: outlines of lobes, spikes and the stalk lines with the leaf dashes. Dark solid: the cushion reads as a mid-grey mound, the spikes darker; both read at 16 mm.", "Helle Linie: Umrisse von Lappen und Ähren und die Stiellinien mit den Blattstrichen. Dunkel flächig: das Polster liest als mittelgrauer Hügel, die Ähren dunkler; beides liest bei 16 mm."),
        overlays=("Thermal: lobes and tufts carry leaf-cushion, spikes flower-spikes, stems and collar stem-bark. Structure: the members of the root crown, the five stems, five cushion sectors and five stalk groups.", "Thermik: Lappen und Büschel tragen leaf-cushion, Ähren flower-spikes, Triebe und Wurzelhals stem-bark. Tragwerk: die Stäbe von Wurzelhals, fünf Trieben, fünf Polstersektoren und fünf Stielgruppen.")),
    100: decisions(
        silhouette=("Nine lobes with eight rim and four crown tufts give a calmer scalloped mound 8 mm wide; five stalks per sector (25) fan out with spikes thickened to 17 mm so each still prints as a dash.", "Neun Lappen mit acht Rand- und vier Kronenbüscheln ergeben einen ruhigeren gewellten Hügel von 8 mm Breite; fünf Stiele je Sektor (25) fächern sich auf, die Ähren auf 17 mm verstärkt, damit jede noch als Strich druckt."),
        retained=("Retained: the fan of stalks per sector, the scalloped rim, the woody stems.", "Beibehalten: der Stielfächer je Sektor, der gewellte Rand, die verholzten Triebe."),
        merged=("Merged: eleven stalks of a sector into five; small rim tufts into eight larger ones.", "Zusammengefasst: elf Stiele eines Sektors zu fünf; kleine Randbüschel zu acht größeren."),
        omitted=("Omitted: six of eleven stalks per sector, the smallest tufts.", "Weggelassen: sechs von elf Stielen je Sektor, die kleinsten Büschel."),
        thinElements=("Stalks stay 0.13 mm lines; spikes are thickened from 11 to 17 mm (0.17 mm) to stay above the print feature together with the pen.", "Stiele bleiben Linien von 0,13 mm; Ähren werden von 11 auf 17 mm (0,17 mm) verstärkt, um mit dem Stift über dem Druckmerkmal zu bleiben."),
        hatching=("No hatching; two leaf dashes per main lobe.", "Keine Schraffur; zwei Blattstriche je Hauptlappen."),
        styles=("Light line and dark solid as at 1:50 with fewer marks; the mound stays one mid-grey mass.", "Helle Linie und dunkel flächig wie bei 1:50 mit weniger Zeichen; der Hügel bleibt eine mittelgraue Masse."),
        overlays=("Thermal regions as at 1:50 on fewer shapes; the structural members are those of 1:50 at the 1:100 section.", "Thermische Regionen wie bei 1:50 auf weniger Formen; die Tragwerksstäbe sind die von 1:50 im Querschnitt von 1:100.")),
    200: decisions(
        silhouette=("At 1:200 the plant is 4 mm by 3 mm: nine large lobes give the domed cushion and fifteen thickened spikes on single stalks a bristling flower fringe.", "Bei 1:200 misst die Pflanze 4 mm mal 3 mm: neun große Lappen ergeben das gewölbte Polster und fünfzehn verstärkte Ähren an einzelnen Stielen einen borstigen Blütensaum."),
        retained=("Retained: the dome, the fringe of spikes, three stems inside the cushion.", "Beibehalten: die Kuppel, der Ährensaum, drei Triebe im Polster."),
        merged=("Merged: the tufts into the lobes; five stems into three; eleven stalks of a sector into three.", "Zusammengefasst: die Büschel in die Lappen; fünf Triebe zu drei; elf Stiele eines Sektors zu drei."),
        omitted=("Omitted: leaf texture and the scalloping below 40 mm.", "Weggelassen: Blatttextur und die Wellung unter 40 mm."),
        thinElements=("Spikes thickened to 32 mm (0.16 mm), stalks drawn as 0.13 mm lines.", "Ähren auf 32 mm (0,16 mm) verstärkt, Stiele als Linien von 0,13 mm gezeichnet."),
        hatching=("No hatching.", "Keine Schraffur."),
        styles=("Light line: outlines only. Dark solid: one grey dome with a darker fringe.", "Helle Linie: nur Umrisse. Dunkel flächig: eine graue Kuppel mit dunklerem Saum."),
        overlays=("The thermal view paints the lobes as leaf-cushion, the spikes as flower-spikes and the stems as stem-bark; structure as at 1:100.", "Die thermische Ansicht färbt die Lappen als leaf-cushion, die Ähren als flower-spikes und die Triebe als stem-bark; Tragwerk wie bei 1:100.")),
}

VOLUME_NOTES = {
    50: [("Body: nine leaf lobes, twenty-nine rim and crown tufts as ellipsoids, five woody stems and the root collar, and 55 flower stalks (3 mm, slender) with 11 mm spikes; every drawn outline is the exact projection of these solids.",
          "Körper: neun Blattlappen, neunundzwanzig Rand- und Kronenbüschel als Ellipsoide, fünf verholzte Triebe und der Wurzelhals sowie 55 Blütenstiele (3 mm, schlank) mit 11 mm dicken Ähren; jeder gezeichnete Umriss ist die exakte Projektion dieser Körper."),
         ("Thermal: every body solid carries its region by reference; ten small ellipsoids mark the transpiration anchors. Structure: one member per bone along the woody frame, joints at the five cushion forks, the support slab under the root crown.",
          "Thermik: jeder Körper trägt seine Region als Verweis; zehn kleine Ellipsoide markieren die Transpirationsanker. Tragwerk: ein Stab je Knochen entlang des Holzgerüsts, Gelenke an den fünf Polstergabeln, die Auflagerplatte unter dem Wurzelhals.")],
    100: [("Body: nine lobes, twelve tufts, five stems, the collar and 25 stalks with 17 mm spikes; texture marks exist only in 2D.", "Körper: neun Lappen, zwölf Büschel, fünf Triebe, der Wurzelhals und 25 Stiele mit 17 mm dicken Ähren; Texturzeichen gibt es nur in 2D."),
          ("Structure: one member per bone at the 1:100 section, no separate joints.", "Tragwerk: ein Stab je Knochen im Querschnitt von 1:100, keine eigenen Gelenke.")],
    200: [("Body: nine large lobes, three stems, the collar and fifteen stalks with 32 mm spikes.", "Körper: neun große Lappen, drei Triebe, der Wurzelhals und fünfzehn Stiele mit 32 mm dicken Ähren."),
          ("Structure: the front, rear and central chains of stem, cushion sector and stalk group as one loft each, the lateral sectors as three members each, the root crown and the support slab.", "Tragwerk: die vordere, hintere und mittlere Kette aus Trieb, Polstersektor und Stielgruppe als je ein Loft, die seitlichen Sektoren als je drei Stäbe, der Wurzelhals und die Auflagerplatte.")],
    500: [("Digital model only (the 1:500 drawing is unavailable): the cushion as one solid of revolution, the flowers as five 80 mm tufts, one per stalk group, the woody frame as one 80 mm column.", "Nur digitales Modell (die Zeichnung 1:500 ist nicht verfügbar): das Polster als ein Rotationskörper, die Blüten als fünf Büschel von 80 mm, eines je Stielgruppe, das Holzgerüst als eine Säule von 80 mm."),
          ("Structure: the frame loft spans root crown, central stem, cushion and stalks; the front and rear chains are one loft each, the lateral sectors one member and one slender stalk member each.", "Tragwerk: das Gerüst-Loft überspannt Wurzelhals, mittleren Trieb, Polster und Stiele; die vordere und hintere Kette sind je ein Loft, die seitlichen Sektoren je ein Stab und ein schlanker Stielstab.")],
}

if __name__ == "__main__":
    build()
