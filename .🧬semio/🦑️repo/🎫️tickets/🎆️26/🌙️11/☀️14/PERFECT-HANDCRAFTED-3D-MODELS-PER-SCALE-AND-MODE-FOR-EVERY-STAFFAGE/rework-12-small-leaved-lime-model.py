"""🌳 Small-leaved lime (Tilia cordata, park tree of about 60 years): the hand numbers of author 12. World x ahead (east), y up, z left (north)."""
import os
from importlib.machinery import SourceFileLoader

A = SourceFileLoader("rework12assemble", os.path.join(os.path.dirname(os.path.abspath(__file__)), "rework-12-assemble.py")).load_module()
K = A.K

t = K.Tree("small-leaved-lime", bark={"trunk": "lime-bark", "limb": "lime-bark-young", "epi": "lime-twig"}, wood="lime-wood", wood_density=740,
           regions={"trunk": "trunk-bark", "limb": "limb-bark", "epi": "epicormic-stem", "sun": "leaf-sun", "shade": "leaf-shade", "epicormic": "leaf-epicormic"},
           leaf_materials={"sun": "lime-leaf-sun", "shade": "lime-leaf-shade", "epicormic": "lime-leaf-epicormic"}, tones={"trunk": 0.3, "limb": 0.34, "epi": 0.4, "sun": 0.6, "shade": 0.44, "epicormic": 0.66})

STEM = [(0, 0, 0, 0.4), (0, 0.12, 0, 0.34), (0, 0.35, 0, 0.29), (0, 0.8, 0, 0.25), (0.01, 1.3, 0, 0.235), (0.02, 2.6, 0, 0.215), (0, 4.0, 0.02, 0.19), (-0.03, 5.2, 0.03, 0.165), (-0.04, 6.6, 0, 0.14), (0, 8.0, -0.03, 0.115), (0.04, 9.4, -0.02, 0.09), (0.05, 10.6, 0, 0.068), (0.02, 11.8, 0.02, 0.046), (0, 12.8, 0.02, 0.028), (0, 13.6, 0, 0.015), (0.02, 14.15, 0, 0.008)]
fy = K.heights(STEM)
t.axis("stem", STEM, kind="trunk", level=0, ridges=(16, 0.008), lobes=[(4, 0.1, 30, 0.22)], ground=True, bones=[
    ("root-plate", 0, fy(0.35), 0, "Root plate and collar with burr", "Wurzelteller und Wurzelhals mit Maserknolle", "Root collar", "Wurzelhals"),
    ("trunk", fy(0.35), fy(2.6), 0, "Trunk (bole)", "Stamm (Schaft)", "Bole", "Schaft"),
    ("ld1", fy(2.6), fy(5.2), 0, "Stem in the lower crown", "Stamm in der unteren Krone", "Stem", "Stamm"),
    ("ld2", fy(5.2), fy(8.0), 0, "Stem in the middle crown", "Stamm in der mittleren Krone", "Stem", "Stamm"),
    ("ld3", fy(8.0), fy(10.6), 1, "Stem in the upper crown", "Stamm in der oberen Krone", "Stem", "Stamm"),
    ("ld4", fy(10.6), fy(12.6), 2, "Leader", "Wipfeltrieb", "Leader", "Wipfeltrieb"),
    ("fol-top", fy(12.6), 1.0, 3, "Leader tip with its foliage", "Wipfelspitze mit Laub", "Leader tip", "Wipfelspitze"),
])
for n, (az, reach, r0) in enumerate([(-8, 1.2, 0.11), (82, 1.0, 0.1), (172, 1.15, 0.105), (262, 1.0, 0.095)], 1):
    d = K.direction(az, 0)
    t.axis(f"root-{n}", [(d[0] * 0.1, 0.36, d[2] * 0.1, r0), (d[0] * 0.45, 0.14, d[2] * 0.45, r0 * 0.8), (d[0] * reach * 0.7, 0.04, d[2] * reach * 0.7, r0 * 0.6), (d[0] * reach, 0.0, d[2] * reach, r0 * 0.45)], bone="root-plate", kind="trunk", level=2, aspect=1.4, ground=True, sides=(12, 8, 6, 6))

NAMES = {"w": ("west", "West"), "e": ("east", "Ost"), "n": ("north", "Nord"), "s": ("south", "Süd"), "a": ("north-east", "Nordost"), "b": ("south-west", "Südwest")}
TIER = {1: ("Lowest", "Unterster"), 2: ("Lower", "Unterer"), 3: ("Middle", "Mittlerer"), 4: ("Upper", "Oberer"), 5: ("Top", "Oberster")}
BRANCHES = [
    ("w1", 2.7, 178, 32, 4.6, 0.08, -46, -10), ("e1", 2.95, -6, 34, 4.5, 0.078, -46, 10), ("n1", 3.2, 58, 34, 4.3, 0.074, -44, -8), ("s1", 3.45, -120, 32, 4.3, 0.072, -44, 8), ("a1", 3.8, 25, 38, 4.5, 0.07, -42, 8), ("b1", 4.0, 205, 38, 4.5, 0.07, -42, -8),
    ("w2", 4.3, 132, 42, 4.8, 0.07, -38, 8), ("e2", 4.6, -44, 42, 4.7, 0.068, -38, -8), ("n2", 4.9, 50, 44, 4.6, 0.066, -36, 10), ("s2", 5.2, -140, 44, 4.6, 0.064, -36, -10), ("a2", 5.7, -20, 46, 4.5, 0.062, -34, 8), ("b2", 5.9, 160, 46, 4.4, 0.06, -34, -8),
    ("w3", 6.3, 195, 50, 4.2, 0.058, -30, 8), ("e3", 6.7, 15, 50, 4.1, 0.056, -30, -8), ("n3", 7.0, 62, 52, 4.0, 0.054, -28, 8), ("s3", 7.4, -118, 52, 3.9, 0.052, -28, -8), ("a3", 7.9, -50, 52, 3.6, 0.05, -26, 8), ("b3", 8.1, 130, 52, 3.5, 0.048, -26, -8),
    ("w4", 8.5, 130, 55, 3.1, 0.046, -18, 6), ("e4", 8.9, -50, 55, 3.0, 0.044, -18, -6), ("n4", 9.3, 40, 56, 2.9, 0.042, -16, 6), ("s4", 9.6, -140, 56, 2.8, 0.04, -16, -6),
    ("w5", 10.5, 175, 58, 2.3, 0.034, -12, 0), ("e5", 10.9, -5, 58, 2.2, 0.032, -12, 0), ("n5", 11.3, 60, 60, 2.0, 0.03, -10, 0),
]
BOUGHS = {1: [(0.34, 56, 12, 2.3, 0.031), (0.54, -52, 12, 2.1, 0.027), (0.74, 44, 14, 1.7, 0.022)], 2: [(0.34, -56, 14, 2.2, 0.03), (0.54, 52, 14, 2.0, 0.026), (0.74, -44, 16, 1.6, 0.022)], 3: [(0.36, 54, 16, 1.9, 0.027), (0.56, -50, 18, 1.7, 0.024), (0.76, 42, 20, 1.3, 0.02)], 4: [(0.4, -52, 20, 1.5, 0.023), (0.66, 46, 22, 1.2, 0.02)], 5: [(0.42, 50, 22, 1.1, 0.02), (0.68, -44, 24, 0.9, 0.018)]}
TWIGS = [(0.4, 50, 6, 0.95, 0.012), (0.7, -46, 8, 0.8, 0.011)]
AZ = {}
for key, y, az, el, length, r0, up, turn in BRANCHES:
    side, tier = key[0], int(key[1])
    en, de = f"{TIER[tier][0]} {NAMES[side][0]} branch", f"{TIER[tier][1]} {NAMES[side][1]}ast"
    AZ[key] = az
    bones = [(f"l{key}a", 0, 0.45, 1, en, de, "Branch", "Ast"), (f"l{key}b", 0.45, 0.8, 2, f"{en}, outer part", f"{de}, Außenteil", "Branch", "Ast"), (f"fol-{key}", 0.8, 1.0, 3, f"{en}, end shoots with foliage", f"{de}, Endtriebe mit Laub", "End shoot", "Endtrieb")] if tier == 1 else [(f"l{key}a", 0, 0.62, 1 if tier < 4 else 2, en, de, "Branch", "Ast"), (f"fol-{key}", 0.62, 1.0, 3, f"{en}, end shoots with foliage", f"{de}, Endtriebe mit Laub", "End shoot", "Endtrieb")]
    t.branch(key, "stem", fy(y), az, el, length, r0, r1=0.009, up=up, turn=turn, level=1, socket=True, bones=bones)
    for n, (at, daz, bel, blen, br0) in enumerate(BOUGHS[tier], 1):
        bid = f"{key}-b{n}"
        t.branch(bid, key, at, az + daz, bel, blen, br0, r1=0.008, up=-12, turn=-daz * 0.15, level=2, scales=(50, 100, 200) if br0 >= 0.03 else (50, 100))
        inner = n == 1 and tier <= 3
        for m, (tat, tdaz, tel, tlen, tr0) in enumerate(TWIGS, 1):
            tid = f"{bid}-t{m}"
            t.branch(tid, bid, tat, az + daz + (tdaz if (n + m) % 2 else -tdaz), tel, tlen * blen / 2.2, tr0, r1=0.008, up=-10, level=3, scales=(50,))
            t.spray(f"fol-{tid}", tid, 0.25, 0.62 * blen / 2.2 + 0.25, 0.5, (5, 2, 0.16), over=0.2, lift=0.05, drop=0.12, shade=inner, scales=(50,), flat=0.8, group=key)
        t.spray(f"fol-{bid}", bid, 0.62, 0.8 * blen / 2.2 + 0.25, 0.58, (6, 3, 0.15), over=0.3, lift=0.08, drop=0.15, shade=inner, scales=(50,), flat=0.8, group=key)
        t.spray(f"fol-{bid}", bid, 0.22, 1.3 * blen / 2.2 + 0.2, 0.75, (6, 3, 0.14), over=0.35, lift=0.1, drop=0.2, shade=inner, scales=(100,), flat=0.8, group=key)
    wide = {1: 2.1, 2: 2.0, 3: 1.75, 4: 1.4, 5: 1.05}[tier]
    t.spray(f"fol-{key}-tip", key, 0.8, 1.05 if tier < 4 else 0.8 if tier == 4 else 0.55, 0.66 if tier < 5 else 0.5, (7, 3, 0.15), over=0.4, lift=0.1, drop=0.25, scales=(50, 100), flat=0.8, group=key)
    if tier <= 3:
        t.spray(f"fol-{key}-inner", key, 0.24, wide * 0.9, 0.95, (6, 3, 0.13), f1=0.6, over=0.0, lift=0.2, shade=True, scales=(200,), flat=0.75, peak=0.5, base=0.5, group=key)
        t.spray(f"fol-{key}-outer", key, 0.54, wide, 1.0, (6, 3, 0.13), over=0.7, lift=0.2, drop=0.3, scales=(200,), flat=0.75, group=key)
    else:
        t.spray(f"fol-{key}-outer", key, 0.3, wide, 0.9, (6, 3, 0.13), over=0.6, lift=0.15, drop=0.2, scales=(200,), flat=0.75, group=key)
    t.spray(f"crown-{key}", key, 0.26, wide * 1.12, 1.25 if tier <= 3 else 1.0, (5, 3, 0.12), over=0.8, lift=0.25, drop=0.3, shade=False, scales=(500,), flat=0.75, peak=0.55, base=0.5, group=key)
for n, (y, az, el, length, r0) in enumerate([(11.7, 30, 44, 1.5, 0.022), (12.2, 200, 46, 1.3, 0.02), (12.7, 130, 48, 1.1, 0.018), (13.2, -50, 50, 0.9, 0.016)], 1):
    t.branch(f"top-b{n}", "stem", fy(y), az, el, length, r0, r1=0.008, up=-20, level=2, scales=(50, 100))
    t.spray(f"fol-top-b{n}", f"top-b{n}", 0.3, 0.5, 0.42, (5, 3, 0.16), over=0.2, drop=0.1, scales=(50, 100), flat=0.8, group="top")
t.spray("fol-top-shoot", "stem", fy(13.2), 0.5, 0.48, (6, 3, 0.15), over=0.25, scales=(50, 100), bone="fol-top", flat=1.0, group="top")
t.spray("fol-top-shoot", "stem", fy(11.6), 1.05, 1.0, (6, 3, 0.13), over=0.35, scales=(200, 500), bone="fol-top", flat=1.0, group="top")
t.spray("crown-core", "stem", fy(4.2), 1.5, 1.4, (6, 4, 0.12), f1=fy(10.4), over=0.0, shade=True, scales=(500,), bone="ld2", flat=1.0, peak=0.4, base=0.7, group="core")

EPI = {"e": [(0.3, -18, 62, 1.9, 0.014), (0.22, 32, 56, 1.6, 0.012), (0.4, -62, 58, 1.5, 0.012)], "w": [(0.28, 166, 60, 1.8, 0.014), (0.36, 206, 56, 1.6, 0.012), (0.2, 128, 58, 1.4, 0.012)]}
for side, rows in EPI.items():
    for n, (y, az, el, length, r0) in enumerate(rows, 1):
        eid = f"epi-{side}" if n == 1 else f"epi-{side}-{n}"
        en, de = f"Epicormic shoots at the {NAMES[side][0]} of the collar", f"Stockausschläge an der {NAMES[side][1]}seite des Wurzelhalses"
        if n == 1:
            t.branch(eid + "-axis", "stem", fy(y), az, el, length, r0, r1=0.008, up=22, level=3, kind="epi", collar=1.0, bones=[(f"epi-{side}", 0, 0.55, 3, en, de, "Shoot", "Trieb"), (f"fol-epi-{side}", 0.55, 1.0, 3, f"{en}, leafy part", f"{de}, belaubter Teil", "Shoot", "Trieb")])
            t.axes[eid + "-axis"]["keep"] = True
        else:
            t.branch(eid, "stem", fy(y), az, el, length, r0, r1=0.008, up=22, level=3, kind="epi", collar=1.0, bone=f"epi-{side}", scales=(50, 100))
        axis = eid + "-axis" if n == 1 else eid
        t.spray(f"fol-{eid}", axis, 0.4, 0.17, 0.17, (4, 2, 0.16), over=0.1, cls="epicormic", scales=(50, 100), bone=f"fol-epi-{side}", flat=1.0, group=f"epi-{side}")
    t.spray(f"fol-epi-{side}-all", f"epi-{side}-axis", 0.3, 0.34, 0.32, (4, 2, 0.14), over=0.2, cls="epicormic", scales=(200, 500), bone=f"fol-epi-{side}", flat=1.0, group=f"epi-{side}")

like = {}
for key, *_ in BRANCHES:
    side, tier = key[0], int(key[1])
    old_side = {"w": "w", "e": "e", "n": "w", "s": "e", "a": "e", "b": "w"}[side]
    old_tier = min(tier, 4)
    like[f"l{key}a"] = f"l{old_side}{old_tier}a"
    like[f"l{key}b"] = f"l{old_side}1b"
    like[f"fol-{key}"] = f"fol-{old_side}{old_tier}"
cfg = {
    "like": like,
    "droop_poses": ["wet-snow"],
    "group_bone": dict({key: (f"l{key}b" if key.endswith("1") else f"fol-{key}") for key, *_ in BRANCHES}, **{"top": "fol-top", "core": "ld2", "epi-e": "fol-epi-e", "epi-w": "fol-epi-w"}),
    "leaf_density": 0.4,
    "emitters": {
        "transpiration-sun": {"on": ["fol-w1-tip", "fol-e1-tip", "fol-n1-tip", "fol-s1-tip", "fol-w2-tip", "fol-e2-tip", "fol-w3-tip", "fol-e3-tip", "fol-top-shoot"], "diameter": 1.1, "cls": "sun", "prefix": "tp-sun", "title": ("Transpiration of the sun leaves", "Transpiration der Sonnenblätter")},
        "transpiration-shade": {"on": ["fol-w1-b1", "fol-e1-b1", "fol-n1-b1", "fol-s1-b1", "fol-w2-b1"], "diameter": 0.8, "cls": "shade", "prefix": "tp-shade", "title": ("Transpiration of the shade leaves", "Transpiration der Schattenblätter")},
        "transpiration-epicormic": {"on": ["fol-epi-e", "fol-epi-e-2", "fol-epi-e-3", "fol-epi-w", "fol-epi-w-2"], "diameter": 0.4, "cls": "epicormic", "prefix": "tp-epi", "title": ("Transpiration of the epicormic leaves", "Transpiration der Blätter der Stockausschläge")},
    },
    "scaffolds": ["w1", "e1", "n1", "s1"],
    "habit": {
        50: ("the lime is built as it grows. A collar of 0.8 m with four root spurs and a burr carries a bole of 0.47 m at breast height, clear to 2.6 m, that stays one straight stem to the tip at 14.1 m. Twenty-five branches leave it in five whorl-like tiers, each tier turned against the one below; they rise at 32 to 60 degrees and arch over, the lowest ending below the horizontal, as lime branches do. Every branch gives off two or three boughs alternately, every bough two drooping twigs, and each twig, bough end and branch end carries one hanging foliage mass; the innermost boughs carry the shade leaves. Six epicormic shoots stand at the collar with their own small leaf masses.", "Die Linde ist gebaut, wie sie wächst. Ein Wurzelhals von 0,8 m mit vier Wurzelanläufen und Maserknolle trägt einen Schaft von 0,47 m in Brusthöhe, astfrei bis 2,6 m, der als ein gerader Stamm bis zur Spitze in 14,1 m durchläuft. Fünfundzwanzig Äste verlassen ihn in fünf quirlähnlichen Etagen, jede gegen die untere verdreht; sie steigen mit 32 bis 60 Grad und bogen über, die untersten enden unter der Waagrechten, wie Lindenäste es tun. Jeder Ast trägt wechselständig zwei oder drei Nebenäste, jeder Nebenast zwei hängende Zweige, und jeder Zweig, jedes Nebenast- und Astende trägt eine hängende Laubmasse; die innersten Nebenäste tragen die Schattenblätter. Sechs Stockausschläge stehen am Wurzelhals mit eigenen kleinen Laubmassen."),
        100: ("the same stem and twenty-five arching branches with their boughs down to 37 mm; the twigs are gone, each bough carries one hanging foliage mass and each branch end one, and the epicormic shoots keep their leaf masses.", "Derselbe Stamm und fünfundzwanzig überbogene Äste mit ihren Nebenästen bis 37 mm; die Zweige entfallen, jeder Nebenast trägt eine hängende Laubmasse und jedes Astende eine, die Stockausschläge behalten ihre Laubmassen."),
        200: ("stem, branches and the strongest boughs remain as wood; each branch of the three lower tiers is a shaded inner and a sunlit outer foliage mass, each upper branch one mass, and one leading epicormic shoot per side stands for its group.", "Stamm, Äste und die stärksten Nebenäste bleiben als Holz; jeder Ast der drei unteren Etagen ist eine beschattete innere und eine besonnte äußere Laubmasse, jeder obere Ast eine Masse, und je Seite steht ein führender Stockausschlag für seine Gruppe."),
        500: ("stem and branches, none thinner than 240 mm, one foliage mass per branch around a shaded core along the stem, and one epicormic shoot per side: the dense ovoid crown with its tiers.", "Stamm und Äste, keiner dünner als 240 mm, je Ast eine Laubmasse um einen beschatteten Kern entlang des Stammes und je Seite ein Stockausschlag: die dichte eiförmige Krone mit ihren Etagen."),
    },
    "drawing": {
        50: ("bole on its collar with the epicormic shoots, the straight stem, twenty-five arching branches with boughs and twigs, and the hanging leaf masses of every branch as one lobed outline with its openings.", "Schaft auf dem Wurzelhals mit den Stockausschlägen, der gerade Stamm, fünfundzwanzig überbogene Äste mit Nebenästen und Zweigen und die hängenden Laubmassen jedes Astes als ein gelappter Umriss mit seinen Lücken."),
        100: ("bole, stem, branches and boughs as tapered outlines per bone and the foliage of each bone as one lobed outline.", "Schaft, Stamm, Äste und Nebenäste als verjüngte Umrisse je Knochen und das Laub jedes Knochens als ein gelappter Umriss."),
        200: ("bole, stem and branches, and one or two foliage masses per branch, the inner ones shaded.", "Schaft, Stamm und Äste sowie eine oder zwei Laubmassen je Ast, die inneren beschattet."),
        500: ("stem, branch stubs and the crown lobes around the shaded core.", "Stamm, Astansätze und die Kronenlappen um den beschatteten Kern."),
    },
}
A.assemble(t, cfg)
