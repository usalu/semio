"""🌳 Pollarded London plane (Platanus x hispanica, street tree of about 50 years, second summer after the cut): the hand numbers of its author. World x ahead (east), y up, z left (north).

The chisel is `rework-12-kit.py`, the carrier into the asset document `rework-12-assemble.py`.

@see ./rework/assets/vegetation--london-plane-pollarded.md
"""
import json
import os
from importlib.machinery import SourceFileLoader

A = SourceFileLoader("rework12assemble", os.path.join(os.path.dirname(os.path.abspath(__file__)), "rework-12-assemble.py")).load_module()
K = A.K
K.OUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "🗑️generated", "rework-london-plane-pollarded")

t = K.Tree("london-plane-pollarded", bark={"trunk": "plane-bark", "limb": "plane-bark-scarred", "shoot": "plane-twig"}, wood="plane-wood", wood_density=950,
           regions={"trunk": "trunk-bark", "limb": "pollard-bark", "shoot": "shoot-bark", "sun": "leaf-sun", "shade": "leaf-shade"},
           leaf_materials={"sun": "plane-leaf-sun", "shade": "plane-leaf-shade"}, tones={"trunk": 0.33, "limb": 0.35, "shoot": 0.35, "sun": 0.6, "shade": 0.45})

STEM = [(0, 0, 0, 0.5), (0, 0.15, 0, 0.42), (0, 0.45, 0, 0.365), (0, 0.9, 0, 0.335), (0.01, 1.3, 0, 0.32), (0.02, 2.4, 0, 0.305), (0.01, 3.3, 0, 0.31), (0, 3.62, 0, 0.325), (0, 3.8, 0, 0.33)]
HEAD = [(0, 3.5, 0, 0.31), (0, 3.72, 0, 0.345), (0, 3.95, 0, 0.385), (0, 4.14, 0, 0.36), (0, 4.3, 0, 0.27), (0, 4.4, 0, 0.17)]
fy = K.heights(STEM)
t.axis("stem", STEM, kind="trunk", level=0, lobes=[(5, 0.12, 20, 0.25)], ground=True, bones=[
    ("root-plate", 0, fy(0.45), 0, "Root plate and flare", "Wurzelteller und Wurzelanlauf", "Root collar", "Wurzelhals"),
    ("trunk", fy(0.45), 1.0, 0, "Clear stem", "Schaft", "Clear stem", "Schaft"),
])
t.axis("head", HEAD, kind="limb", level=0, parent="trunk", bones=[("head", 0, 1.0, 0, "Pollard head", "Kopf (Kappstelle)", "Pollard head", "Kopf")])
for n, (az, reach, r0) in enumerate([(20, 1.05, 0.13), (92, 0.9, 0.12), (164, 1.0, 0.125), (236, 0.92, 0.115), (308, 0.98, 0.12)], 1):
    d = K.direction(az, 0)
    t.axis(f"buttress-{n}", [(d[0] * 0.2, 0.42, d[2] * 0.2, r0), (d[0] * 0.5, 0.17, d[2] * 0.5, r0 * 0.85), (d[0] * reach * 0.75, 0.05, d[2] * reach * 0.75, r0 * 0.62), (d[0] * reach, 0.0, d[2] * reach, r0 * 0.45)], bone="root-plate", kind="trunk", level=2, aspect=1.5, ground=True, sides=(12, 8, 6, 6), scales=(50, 100, 200))
for n, (y, az, length, r0) in enumerate([(3.86, 95, 0.44, 0.1), (4.02, 232, 0.47, 0.11), (3.9, 318, 0.43, 0.095)], 1):
    d = K.direction(az, 8)
    o = (0.0, y, 0.0)
    t.axis(f"head-burr-{n}", [(o[0], o[1], o[2], r0 * 1.2), (d[0] * length * 0.6, y + d[1] * length * 0.6, d[2] * length * 0.6, r0 * 1.15), (d[0] * length, y + d[1] * length, d[2] * length, r0)], bone="head", kind="limb", level=2, sides=(10, 8, 6, 6), scales=(50, 100, 200))

NAMES = {"e": ("East", "Ost"), "ne": ("North-east", "Nordost"), "nw": ("North-west", "Nordwest"), "w": ("West", "West"), "s": ("South", "Süd"), "c": ("Central", "Mittel")}
LIMBS = {"e": (-10, 34, 1.3, 3.96, 0.215, 1.0, 0), "ne": (62, 42, 1.1, 4.04, 0.2, 0.94, 8), "nw": (128, 32, 1.35, 3.94, 0.215, 1.03, -6), "w": (192, 40, 1.15, 4.02, 0.205, 0.97, 5), "s": (268, 36, 1.25, 3.98, 0.21, 1.0, -9), "c": (40, 80, 0.62, 4.2, 0.19, 1.0, 0)}
LEAD = {"e": (66, 3.45, 0.024, 16), "ne": (70, 3.3, 0.023, 13), "nw": (64, 3.55, 0.024, 18), "w": (69, 3.35, 0.023, 14), "s": (67, 3.5, 0.024, 15), "c": (86, 3.6, 0.025, 2)}
BROOM = [("out", 0, 42, 3.1, 0.02, 24, (50, 100)), ("r1", -40, 50, 3.3, 0.02, 20, (50, 100)), ("l1", 42, 53, 2.95, 0.0195, 18, (50, 100)), ("r2", -84, 60, 2.8, 0.018, 16, (50,)), ("l2", 88, 63, 3.3, 0.0195, 14, (50, 100)), ("in", 178, 78, 3.5, 0.021, 5, (50, 100)), ("r3", -140, 70, 3.0, 0.018, 10, (50,)), ("l3", 134, 73, 3.35, 0.0195, 8, (50, 100)), ("low", 6, 26, 2.5, 0.016, 32, (50,)), ("up", -15, 60, 3.4, 0.02, 14, (50, 100))]
BROOM_C = [("a", 15, 76, 3.35, 0.021, 6, (50, 100)), ("b", 78, 70, 3.2, 0.02, 10, (50, 100)), ("d", 140, 74, 3.3, 0.02, 8, (50, 100)), ("f", 205, 71, 3.15, 0.019, 10, (50,)), ("g", 262, 75, 3.3, 0.02, 7, (50, 100)), ("h", 322, 72, 3.2, 0.0195, 9, (50, 100))]
COARSE = [("lead", 0.36, 0.66, 0.86), ("out", 0.2, 0.64, 0.84), ("r1", 0.2, 0.6, 0.8), ("l1", 0.2, 0.6, 0), ("l2", 0.2, 0.6, 0.8), ("in", 0.22, 0.6, 0), ("up", 0.2, 0.6, 0), ("r3", 0.22, 0.58, 0.78)]
COARSE_C = [("lead", 0.36, 0.7, 0.9), ("a", 0.2, 0.6, 0.8), ("d", 0.2, 0.6, 0.8), ("g", 0.2, 0.6, 0.8)]
STUBS = [(0.78, 75, 12, 0.19, 0.06), (0.88, -80, 25, 0.17, 0.055), (0.7, 170, -20, 0.16, 0.065)]
for key, (az, el, length, y0, rk, grow, twist) in LIMBS.items():
    d = K.direction(az, el)
    d2 = K.direction(az, el + 14)
    o = (0.0, y0, 0.0)
    mid = K.add(o, K.mul(d, length * 0.55))
    end = K.add(mid, K.mul(d2, length * 0.45))
    tip = K.add(end, K.mul(d2, rk * 0.75))
    en, de = f"{NAMES[key][0]} framework limb with its knuckle", f"{NAMES[key][1]}-Gerüstast mit Knolle"
    t.axis(f"limb-{key}", [(o[0], o[1], o[2], 0.2), (K.add(o, K.mul(d, length * 0.3))[0], K.add(o, K.mul(d, length * 0.3))[1], K.add(o, K.mul(d, length * 0.3))[2], 0.165), (mid[0], mid[1], mid[2], 0.145), (K.add(mid, K.mul(d2, length * 0.25))[0], K.add(mid, K.mul(d2, length * 0.25))[1], K.add(mid, K.mul(d2, length * 0.25))[2], 0.165), (end[0], end[1], end[2], rk), (tip[0], tip[1], tip[2], rk * 0.86)],
           bones=[(f"k-{key}", 0, 1.0, 1, en, de, "Framework limb", "Gerüstast")], parent="head", kind="limb", level=1)
    for n, (at, daz, sel, slen, sr) in enumerate(STUBS, 1):
        t.branch(f"knuckle-{key}-stub-{n}", f"limb-{key}", at, az + daz, sel + (40 if key == "c" else 0), slen, sr, r1=sr * 0.8, kind="limb", level=2, collar=1.0, bone=f"k-{key}", sides=(8, 6, 6, 6), scales=(50, 100) if n < 3 else (50,))
    lel, llen, lr, lup = LEAD[key]
    sen, sde = f"{NAMES[key][0]} leading shoot", f"{NAMES[key][1]}-Leittrieb"
    t.branch(f"shoot-{key}", f"limb-{key}", 0.9, az, lel, llen, lr, r1=0.005, up=lup, kind="shoot", level=2, collar=1.0, sides=(8, 6, 6, 6), bones=[
        (f"s-{key}a", 0, 0.5, 2, f"{sen}, first summer", f"{sde}, erster Sommer", "Shoot", "Trieb"),
        (f"s-{key}b", 0.5, 1.0, 3, f"{sen}, second summer", f"{sde}, zweiter Sommer", "Shoot", "Trieb")])
    t.spray(f"leaf-{key}-lead", f"shoot-{key}", 0.4, 0.44, 0.44, (6, 7, 0.3), over=0.18, scales=(50,), bone=f"s-{key}b", flat=1.0, peak=0.4, base=0.6, group=f"{key}-top")
    t.spray(f"leaf-{key}-heart", f"shoot-{key}", 0.1, 0.5, 0.5, (5, 3, 0.18), f1=0.5, over=0.0, shade=True, scales=(50, 100), bone=f"s-{key}a", flat=1.0, peak=0.55, base=0.45, group=key)
    t.spray(f"leaf-{key}-lead", f"shoot-{key}", 0.38, 0.5, 0.5, (6, 5, 0.3), over=0.14, scales=(100,), bone=f"s-{key}b", flat=1.0, peak=0.4, base=0.6, group=f"{key}-top")
    for n, (sid, daz, sel, slen, sr, sup, scales) in enumerate(BROOM_C if key == "c" else BROOM):
        at = 0.84 + 0.03 * (n % 5)
        aid = f"shoot-{key}-{sid}"
        t.branch(aid, f"limb-{key}", at, (daz if key == "c" else az + twist + daz), sel, slen * grow, sr, r1=0.005, up=sup, turn=(6 if n % 2 else -6), kind="shoot", level=3, collar=1.0, bone=f"s-{key}a", scales=scales)
        t.spray(f"leaf-{key}-{sid}", aid, 0.17, 0.34 + 0.03 * (n % 3), 0.34 + 0.03 * (n % 3), (6, 7 + n % 2, 0.3), over=0.15, scales=(50,), bone=f"s-{key}a", flat=1.0, peak=0.45, base=0.55, group=key)
        if True:
            t.spray(f"leaf-{key}-{sid}", aid, 0.18, 0.47, 0.47, (6, 5, 0.3), over=0.12, scales=(100,), bone=f"s-{key}a", flat=1.0, peak=0.45, base=0.55, group=key)
    t.axes[f"shoot-{key}"]["keep"] = True
    coarse = COARSE_C if key == "c" else COARSE
    for sid, f0, rx200, rx500 in coarse:
        aid = f"shoot-{key}" if sid == "lead" else f"shoot-{key}-{sid}"
        bone = f"s-{key}b" if sid == "lead" else f"s-{key}a"
        group = f"{key}-top" if sid == "lead" else key
        t.spray(f"crown-{key}-{sid}", aid, f0, rx200, rx200, (6, 4, 0.2), over=0.14, scales=(200,), bone=bone, flat=1.0, peak=0.45, base=0.55, group=group)
        if rx500:
            t.spray(f"crown-{key}-{sid}", aid, f0, rx500, rx500, (6, 3, 0.2), over=0.2 if sid == "lead" else 0.12, scales=(500,), bone=bone, flat=1.0, peak=0.7 if sid == "lead" else 0.45, base=0.55, group=group)
    if key == "c":
        t.spray("crown-heart", "shoot-c", 0.04, 1.55, 1.3, (5, 3, 0.14), f1=0.62, over=0.0, shade=True, scales=(500,), bone="s-ca", flat=1.0, peak=0.5, base=0.6, group="c")
    t.spray(f"crown-{key}-heart", f"shoot-{key}", 0.08, 0.8, 0.8, (5, 3, 0.14), f1=0.5, over=0.0, shade=True, scales=(200,), bone=f"s-{key}a", flat=1.0, peak=0.55, base=0.5, group=key)

OLD = {"e": "e2", "ne": "e1", "nw": "w1", "w": "w2", "s": "e1", "c": "c"}
like = {}
for key, old in OLD.items():
    like[f"k-{key}"] = f"k-{old}"
    like[f"s-{key}a"] = f"s-{old}a"
    like[f"s-{key}b"] = f"s-{old}b"
cfg = {
    "like": like,
    "old_to_new": {"fol-core": "s-ca", "fol-top": "s-cb", "fol-w2": "s-wb", "fol-w1": "s-nwb", "fol-e1": "s-neb", "fol-e2": "s-eb", "fol-low-w": "s-wa", "fol-low-e": "s-ea"},
    "group_bone": dict({key: f"s-{key}a" for key in LIMBS}, **{f"{key}-top": f"s-{key}b" for key in LIMBS}),
    "leaf_density": 0.1,
    "area_trim": {"pollard-bark": 0.6},
    "footprint": 1.8,
    "emitters": {
        "transpiration-sun": {"on": ["leaf-e-lead", "leaf-ne-lead", "leaf-nw-lead", "leaf-w-lead", "leaf-s-lead", "leaf-c-g", "leaf-c-a"], "diameter": 0.6, "cls": "sun", "prefix": "tp-sun", "title": ("Transpiration of the sun leaves", "Transpiration der Sonnenblätter")},
        "transpiration-shade": {"on": ["leaf-e-heart", "leaf-ne-heart", "leaf-nw-heart", "leaf-w-heart", "leaf-s-heart"], "diameter": 0.5, "cls": "shade", "prefix": "tp-shade", "title": ("Transpiration of the shade leaves", "Transpiration der Schattenblätter")},
    },
    "scaffolds": ["limb-e", "limb-nw", "limb-w", "limb-s"],
    "symbol_extra": lambda view, shape, tree: shape,
    "habit": {
        50: ("the plane is built as a pollard grows. A flared collar of 1.0 m with five buttress roots carries a clear bole of 0.64 m at breast height that swells into the pollard head at 3.7 to 4.4 m, with three burrs. Six short framework limbs leave the head, five in a ring all round the compass and one in the middle; each rises at 32 to 42 degrees, turns up and ends in a swollen knuckle of 0.38 to 0.43 m with old cut stubs. From every knuckle the regrowth of two summers stands as a broom: one leading shoot of 3.3 to 3.6 m, bound to two rig bones (its two annual increments), and six to ten further whips of 2.5 to 3.5 m that lean out and turn upright, sixty-eight shoots in all. Every shoot carries its large palmate leaves as one lobed plume along its length, bare for the first half metre above the knuckle; the plumes in the heart of each broom are the shade leaves. Single leaves (0.2 m, 4 mm on paper) are not modelled.", "Die Platane ist gebaut, wie ein Kopfbaum wächst. Ein Wurzelhals von 1,0 m mit fünf Wurzelanläufen trägt einen astfreien Schaft von 0,64 m in Brusthöhe, der auf 3,7 bis 4,4 m in den Kopf mit drei Maserknollen anschwillt. Sechs kurze Gerüstäste verlassen den Kopf, fünf im Ring rund um die Kompassrose und einer in der Mitte; jeder steigt mit 32 bis 42 Grad, richtet sich auf und endet in einer verdickten Knolle von 0,38 bis 0,43 m mit alten Schnittstummeln. Aus jeder Knolle steht der Austrieb zweier Sommer als Besen: ein Leittrieb von 3,3 bis 3,6 m an zwei Skelettknochen (seinen zwei Jahreszuwächsen) und sechs bis zehn weitere Ruten von 2,5 bis 3,5 m, die sich nach außen neigen und aufrichten, zusammen achtundsechzig Triebe. Jeder Trieb trägt seine großen, gelappten Blätter als eine gelappte Fahne entlang seiner Länge, kahl auf dem ersten halben Meter über der Knolle; die Fahnen im Herzen jedes Besens sind die Schattenblätter. Einzelne Blätter (0,2 m, 4 mm auf dem Papier) sind nicht modelliert."),
        100: ("the same bole, head, six framework limbs with their knuckles and stubs, and of every broom the leading shoot and the five or six strongest whips, each with one leaf plume; the shade plume stays in the heart of each broom.", "Derselbe Schaft, Kopf und sechs Gerüstäste mit Knollen und Stummeln, von jedem Besen der Leittrieb und die fünf oder sechs stärksten Ruten, jede mit einer Blattfahne; die Schattenfahne bleibt im Herzen jedes Besens."),
        200: ("bole, head with burrs, the six framework limbs and the six leading shoots remain as wood; each broom of the ring is eight sunlit upright foliage masses, one per stronger whip, around a shaded heart, the central broom four over its heart.", "Schaft, Kopf mit Maserknollen, die sechs Gerüstäste und die sechs Leittriebe bleiben als Holz; jeder Besen des Rings besteht aus acht besonnten, aufrechten Laubmassen, je stärkerer Rute eine, um ein beschattetes Herz, der mittlere Besen aus vier über seinem Herz."),
        500: ("bole, head, framework limbs and leading shoots, none thinner than 240 mm, and four foliage masses per broom of the ring (top, outward and two flanks) and four of the central broom around one shaded heart over the head: the dense upright mop of a pollard in leaf.", "Schaft, Kopf, Gerüstäste und Leittriebe, keiner dünner als 240 mm, und vier Laubmassen je Besen des Rings (oben, außen und zwei Flanken) sowie vier des mittleren Besens um ein beschattetes Herz über dem Kopf: der dichte, aufrechte Schopf eines belaubten Kopfbaums."),
    },
    "drawing": {
        50: ("the bole on its buttressed collar, the head with its burrs, the six framework limbs with their knuckles, the whips of every broom and their leaf plumes as one lobed outline per broom with its sky openings.", "der Schaft auf dem Wurzelhals mit Wurzelanläufen, der Kopf mit seinen Maserknollen, die sechs Gerüstäste mit ihren Knollen, die Ruten jedes Besens und ihre Blattfahnen als ein gelappter Umriss je Besen mit seinen Himmelslücken."),
        100: ("bole, head, framework limbs and the stronger whips as tapered outlines per bone and the foliage of each broom as one lobed outline.", "Schaft, Kopf, Gerüstäste und die stärkeren Ruten als verjüngte Umrisse je Knochen und das Laub jedes Besens als ein gelappter Umriss."),
        200: ("bole, head and framework limbs, and the foliage masses of each broom, the hearts shaded.", "Schaft, Kopf und Gerüstäste sowie die Laubmassen jedes Besens, die Herzen beschattet."),
        500: ("bole, head with the limb stubs and the crown lobes of the brooms around the shaded heart.", "Schaft, Kopf mit den Astansätzen und die Kronenlappen der Besen um das beschattete Herz."),
    },
}
plain = t.mass_samples


def finer(m, scale):
    """🍃 The leaf plumes of 1:50 and 1:100 are lofted through about twice as many rings as the kit's default so that the stated relief along a shoot (seven and five leaf tiers) is carried."""
    keep = K.RING_STEP[scale]
    K.RING_STEP[scale] = keep * {50: 0.45, 100: 0.6}.get(scale, 1.0)
    try:
        return plain(m, scale)
    finally:
        K.RING_STEP[scale] = keep


t.mass_samples = finer
A.assemble(t, cfg)
PLAN = os.path.join(K.OUT, t.slug, "plan.json")
plan = json.load(open(PLAN))
for shapes in plan["shapes"].values():
    for view in ("elevation", "front", "plan"):
        for shape in shapes[view]:
            if shape["bone"].startswith("s-") and shape["region"] == "shoot-bark":
                shape["layer"] = -1000
K.write(PLAN, plan)
