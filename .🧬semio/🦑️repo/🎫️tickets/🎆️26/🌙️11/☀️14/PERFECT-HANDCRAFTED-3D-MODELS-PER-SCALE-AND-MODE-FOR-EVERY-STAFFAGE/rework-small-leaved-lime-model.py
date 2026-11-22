"""🌳 Small-leaved lime (Tilia cordata, park tree of about 60 years): the hand numbers of its author (begun by author 12 in `rework-12-small-leaved-lime-model.py`). World x ahead (east), y up, z left (north).

The chisel is `rework-12-kit.py`, the carrier into the asset document `rework-12-assemble.py`.

@see ./rework/assets/vegetation--small-leaved-lime.md
"""
import os
from importlib.machinery import SourceFileLoader

A = SourceFileLoader("rework12assemble", os.path.join(os.path.dirname(os.path.abspath(__file__)), "rework-12-assemble.py")).load_module()
K = A.K
K.OUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "🗑️generated", "rework-small-leaved-lime")

t = K.Tree("small-leaved-lime", bark={"trunk": "lime-bark", "limb": "lime-bark-young", "epi": "lime-twig"}, wood="lime-wood", wood_density=740,
           regions={"trunk": "trunk-bark", "limb": "limb-bark", "epi": "epicormic-stem", "sun": "leaf-sun", "shade": "leaf-shade", "epicormic": "leaf-epicormic"},
           leaf_materials={"sun": "lime-leaf-sun", "shade": "lime-leaf-shade", "epicormic": "lime-leaf-epicormic"}, tones={"trunk": 0.3, "limb": 0.34, "epi": 0.4, "sun": 0.6, "shade": 0.44, "epicormic": 0.66})

STEM = [(0, 0, 0, 0.4), (0, 0.12, 0, 0.34), (0, 0.35, 0, 0.29), (0, 0.8, 0, 0.25), (0.01, 1.3, 0, 0.235), (0.02, 2.6, 0, 0.215), (0, 4.0, 0, 0.19), (-0.03, 5.2, 0, 0.165), (-0.04, 6.6, 0, 0.14), (0, 8.0, 0, 0.115), (0.04, 9.4, 0, 0.09), (0.05, 10.6, 0, 0.068), (0.02, 11.8, 0, 0.046), (0, 12.8, 0, 0.028), (0, 13.6, 0, 0.015), (0.02, 14.15, 0, 0.008)]
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
for n, (az, reach, r0) in enumerate([(-8, 1.15, 0.15), (64, 1.0, 0.13), (136, 1.1, 0.14), (208, 0.95, 0.13), (280, 1.05, 0.14)], 1):
    d = K.direction(az, 0)
    t.axis(f"root-{n}", [(d[0] * 0.1, 0.42, d[2] * 0.1, r0), (d[0] * 0.45, 0.14, d[2] * 0.45, r0 * 0.8), (d[0] * reach * 0.7, 0.04, d[2] * reach * 0.7, r0 * 0.6), (d[0] * reach, 0.0, d[2] * reach, r0 * 0.45)], bone="root-plate", kind="trunk", level=2, aspect=1.4, ground=True, sides=(12, 8, 6, 6))

NAMES = {"w": ("west", "West"), "e": ("east", "Ost"), "n": ("north", "Nord"), "s": ("south", "Süd"), "a": ("north-east", "Nordost"), "b": ("south-west", "Südwest")}
TIER = {1: ("Lowest", "Unterster"), 2: ("Lower", "Unterer"), 3: ("Middle", "Mittlerer"), 4: ("Upper", "Oberer"), 5: ("Top", "Oberster")}
BRANCHES = [
    ("w1", 2.7, 178, 32, 3.7, 0.08, -46, -10), ("e1", 2.95, -6, 34, 3.6, 0.078, -46, 10), ("n1", 3.2, 80, 34, 3.6, 0.074, -44, -8), ("s1", 3.45, -98, 32, 3.6, 0.072, -44, 8), ("a1", 3.8, 25, 38, 4.1, 0.07, -42, 8), ("b1", 4.0, 205, 38, 4.1, 0.07, -42, -8),
    ("w2", 4.3, 132, 42, 5.0, 0.074, -38, 8), ("e2", 4.6, -44, 42, 4.9, 0.072, -38, -8), ("n2", 4.9, 50, 44, 4.8, 0.07, -36, 10), ("s2", 5.2, -140, 44, 4.8, 0.068, -36, -10), ("a2", 5.7, -20, 46, 4.6, 0.064, -34, 8), ("b2", 5.9, 160, 46, 4.5, 0.062, -34, -8),
    ("w3", 6.3, 195, 48, 5.0, 0.06, -34, 8), ("e3", 6.7, 15, 48, 4.9, 0.058, -34, -8), ("n3", 7.0, 92, 52, 4.8, 0.056, -28, 8), ("s3", 7.4, -90, 52, 4.7, 0.054, -28, -8), ("a3", 7.9, -50, 50, 4.5, 0.052, -30, 8), ("b3", 8.1, 130, 50, 4.4, 0.05, -30, -8),
    ("w4", 8.5, 130, 50, 4.2, 0.048, -26, 6), ("e4", 8.9, -50, 50, 4.1, 0.046, -26, -6), ("n4", 9.3, 40, 51, 3.9, 0.044, -26, 6), ("s4", 9.6, -140, 51, 3.8, 0.042, -26, -6),
    ("w5", 10.5, 175, 52, 3.3, 0.036, -24, 0), ("e5", 10.8, -5, 52, 3.2, 0.034, -24, 0), ("n5", 11.1, 88, 53, 3.0, 0.032, -24, 0), ("s5", 11.4, -92, 53, 2.9, 0.031, -24, 0),
]
BOUGHS = {1: [(0.34, 56, 12, 2.3, 0.031), (0.54, -52, 12, 2.1, 0.027), (0.74, 44, 14, 1.7, 0.022)], 2: [(0.34, -56, 14, 2.2, 0.03), (0.54, 52, 14, 2.0, 0.026), (0.74, -44, 16, 1.6, 0.022)], 3: [(0.36, 54, 16, 2.1, 0.027), (0.56, -50, 18, 1.9, 0.024), (0.76, 42, 20, 1.5, 0.02)], 4: [(0.4, -52, 20, 1.8, 0.023), (0.66, 46, 22, 1.5, 0.02)], 5: [(0.42, 50, 22, 1.4, 0.02), (0.68, -44, 24, 1.2, 0.018)]}
TWIGS = [(0.4, 50, 6, 0.95, 0.012), (0.7, -46, 8, 0.8, 0.011)]
AZ = {}
for key, y, az, el, length, r0, up, turn in BRANCHES:
    side, tier = key[0], int(key[1])
    en, de = f"{TIER[tier][0]} {NAMES[side][0]} branch", f"{TIER[tier][1]} {NAMES[side][1]}ast"
    AZ[key] = az
    bones = [(f"l{key}a", 0, 0.45, 1, en, de, "Branch", "Ast"), (f"l{key}b", 0.45, 0.8, 2, f"{en}, outer part", f"{de}, Außenteil", "Branch", "Ast"), (f"fol-{key}", 0.8, 1.0, 3, f"{en}, end shoots with foliage", f"{de}, Endtriebe mit Laub", "End shoot", "Endtrieb")] if tier == 1 else [(f"l{key}a", 0, 0.62, 1 if tier < 4 else 2, en, de, "Branch", "Ast"), (f"fol-{key}", 0.62, 1.0, 3, f"{en}, end shoots with foliage", f"{de}, Endtriebe mit Laub", "End shoot", "Endtrieb")]
    t.branch(key, "stem", fy(y), az, el, length, r0, r1=0.009, up=up, turn=turn, level=1, socket=True, bones=bones, sides=(16, 12, 8, 8))
    for n, (at, daz, bel, blen, br0) in enumerate(BOUGHS[tier], 1):
        bid = f"{key}-b{n}"
        t.branch(bid, key, at, az + daz, bel, blen, br0, r1=0.008, up=-12, turn=-daz * 0.15, level=2, scales=(50, 100, 200) if br0 >= 0.026 else (50, 100))
        t.axes[bid]["keep"] = br0 >= 0.026
        inner = n == 1 and tier <= 3
        k = min(1.0, (blen / 2.2) ** 2)
        for m, (tat, tdaz, tel, tlen, tr0) in enumerate(TWIGS, 1):
            tid = f"{bid}-t{m}"
            t.branch(tid, bid, tat, az + daz + (tdaz if (n + m) % 2 else -tdaz), tel, tlen * blen / 2.2, tr0, r1=0.008, up=-10, level=3, scales=(50,))
            t.spray(f"fol-{tid}", tid, 0.25, 0.62 * blen / 2.2 + 0.25, 0.5, (5, 2, 0.16), over=0.2, lift=0.05 * k, drop=0.12 * k, shade=inner, scales=(50, 100), flat=0.8, group=key)
            t.spray(f"fol-{tid}", tid, 0.15, 0.62 * blen / 2.2 + 0.32, 0.54, (5, 2, 0.16), over=0.26, lift=0.05 * k, drop=0.12 * k, shade=inner, scales=(200,), flat=0.8, group=key)
        t.spray(f"fol-{bid}", bid, 0.62 if blen >= 1.6 else 0.45, 0.8 * blen / 2.2 + 0.25, 0.58, (6, 3, 0.15), over=0.3, lift=0.08 * k, drop=0.15 * k, shade=inner, scales=(50, 100), flat=0.8, group=key)
        t.spray(f"fol-{bid}", bid, 0.5 if blen >= 1.6 else 0.35, 0.8 * blen / 2.2 + 0.32, 0.62, (6, 3, 0.15), over=0.36, lift=0.08 * k, drop=0.15 * k, shade=inner, scales=(200,), flat=0.8, group=key)
        t.spray(f"fol-{bid}", bid, 0.0, 1.1 * blen / 2.2 + 0.32, 0.64, (6, 3, 0.14), over=0.4, lift=0.08 * k, drop=0.15 * k, shade=inner, scales=(500,), flat=0.8, peak=0.65, base=0.15, group=key)
    wide = {1: 2.0, 2: 2.1, 3: 1.85, 4: 1.55, 5: 1.25}[tier]
    t.spray(f"fol-{key}-tip", key, 0.8 if tier < 3 else 0.7, 1.05 if tier < 4 else 0.9 if tier == 4 else 0.8, 0.66 if tier < 5 else 0.6, (7, 3, 0.15), over=0.4, lift=0.1 if tier < 3 else 0.05, drop=0.25 if tier < 3 else 0.1, scales=(50, 100, 200, 500), flat=0.8, group=key)
for n, (y, az, el, length, r0) in enumerate([(11.9, 30, 38, 1.9, 0.022), (12.3, 210, 40, 1.7, 0.02), (12.7, 120, 42, 1.4, 0.018), (13.1, -60, 44, 1.15, 0.016)], 1):
    t.branch(f"top-b{n}", "stem", fy(y), az, el, length, r0, r1=0.008, up=-20, level=2, scales=(50, 100))
    t.spray(f"fol-top-b{n}", f"top-b{n}", 0.3, 0.6, 0.46, (5, 3, 0.16), over=0.2, drop=0.1, scales=(50, 100), flat=0.8, group="top")
    t.spray(f"fol-top-b{n}", f"top-b{n}", 0.05, 0.68, 0.5, (5, 3, 0.16), over=0.26, drop=0.1, scales=(200,), flat=0.8, group="top")
t.spray("fol-top-shoot", "stem", fy(13.2), 0.5, 0.48, (6, 3, 0.15), over=0.25, scales=(50, 100, 200), bone="fol-top", flat=1.0, group="top")
t.spray("fol-top-inner", "stem", fy(9.3), 2.0, 2.0, (7, 3, 0.14), f1=fy(10.7), over=0.0, shade=True, scales=(50, 100, 200), bone="ld3", flat=1.0, group="top")
t.spray("fol-top-shoot", "stem", fy(11.6), 1.05, 1.0, (6, 3, 0.13), over=0.35, scales=(500,), bone="fol-top", flat=1.0, group="top")

EPI = {"e": [(0.3, -12, 66, 1.3, 0.012), (0.2, 34, 50, 0.9, 0.0105), (0.36, -58, 54, 1.0, 0.0105), (0.16, -30, 40, 0.7, 0.0105), (0.26, 70, 58, 1.1, 0.0105)], "w": [(0.28, 170, 66, 1.25, 0.012), (0.34, 214, 52, 1.0, 0.0105), (0.18, 132, 50, 0.85, 0.0105), (0.14, 190, 40, 0.7, 0.0105), (0.24, 250, 58, 1.05, 0.0105)]}
for side, rows in EPI.items():
    for n, (y, az, el, length, r0) in enumerate(rows, 1):
        eid = f"epi-{side}" if n == 1 else f"epi-{side}-{n}"
        en, de = f"Epicormic shoots at the {NAMES[side][0]} of the collar", f"Stockausschläge an der {NAMES[side][1]}seite des Wurzelhalses"
        if n == 1:
            t.branch(eid + "-axis", "stem", fy(y), az, el, length, r0, r1=0.008, up=22, level=3, kind="epi", collar=1.0, bones=[(f"epi-{side}", 0, 0.55, 3, en, de, "Shoot", "Trieb"), (f"fol-epi-{side}", 0.55, 1.0, 3, f"{en}, leafy part", f"{de}, belaubter Teil", "Shoot", "Trieb")])
            t.axes[eid + "-axis"]["keep"] = True
        else:
            t.branch(eid, "stem", fy(y), az, el, length, r0, r1=0.008, up=22, level=3, kind="epi", collar=1.0, bone=f"epi-{side}", scales=(50, 100) if n <= 3 else (50,))
            t.axes[eid]["keep"] = n <= 3
        axis = eid + "-axis" if n == 1 else eid
        t.spray(f"fol-{eid}" if n > 1 else f"fol-{eid}-1", axis, 0.3, 0.21, 0.2, (5, 2, 0.2), over=0.08, cls="epicormic", scales=(50,), bone=f"fol-epi-{side}", flat=1.0, group=f"epi-{side}")
    t.spray(f"fol-epi-{side}-all", f"epi-{side}-axis", 0.0, 0.46, 0.36, (5, 2, 0.14), over=0.1, peak=0.45, base=0.75, cls="epicormic", scales=(100, 200, 500), bone=f"fol-epi-{side}", flat=1.0, group=f"epi-{side}")

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
    "group_bone": dict({key: (f"l{key}b" if key.endswith("1") else f"fol-{key}") for key, *_ in BRANCHES}, **{"top": "fol-top", "epi-e": "fol-epi-e", "epi-w": "fol-epi-w"}),
    "leaf_density": 0.4,
    "peaks": {"summer-breeze": 1.5, "autumn-leaf-fall": 0.8, "wet-snow": 5.0, "gale": 9.0, "breeze-sway": 2.5, "gust": 5.0},
    "emitters": {
        "transpiration-sun": {"on": ["fol-w1-tip", "fol-e1-tip", "fol-n1-tip", "fol-s1-tip", "fol-w2-tip", "fol-e2-tip", "fol-w3-tip", "fol-e3-tip", "fol-top-shoot"], "diameter": 1.1, "cls": "sun", "prefix": "tp-sun", "title": ("Transpiration of the sun leaves", "Transpiration der Sonnenblätter")},
        "transpiration-shade": {"on": ["fol-w1-b1", "fol-e1-b1", "fol-n1-b1", "fol-s1-b1", "fol-w2-b1"], "diameter": 0.8, "cls": "shade", "prefix": "tp-shade", "title": ("Transpiration of the shade leaves", "Transpiration der Schattenblätter")},
        "transpiration-epicormic": {"on": ["fol-epi-e-1", "fol-epi-e-2", "fol-epi-e-3", "fol-epi-w-1", "fol-epi-w-2"], "diameter": 0.4, "cls": "epicormic", "prefix": "tp-epi", "title": ("Transpiration of the epicormic leaves", "Transpiration der Blätter der Stockausschläge")},
    },
    "scaffolds": ["w1", "e1", "n1", "s1"],
    "area_trim": {"trunk-bark": 0.9, "limb-bark": 1.07, "epicormic-stem": 1.0},
    "habit": {
        50: ("the lime is built as it grows. A collar of 0.8 m with five root spurs and a burr carries a bole of 0.47 m at breast height, clear to 2.6 m, that stays one straight stem to the tip at 14.1 m. Twenty-six branches leave it in five whorl-like tiers, each tier turned against the one below; they rise at 32 to 54 degrees and arch over, the lowest ending below the horizontal, as lime branches do. Every branch gives off two or three boughs alternately, every bough two drooping twigs, and each twig, bough end and branch end carries one hanging foliage mass; the innermost boughs carry the shade leaves. Ten thin epicormic shoots of 0.7 to 1.3 m stand as a brush around the collar, each with its own small leaf mass.", "Die Linde ist gebaut, wie sie wächst. Ein Wurzelhals von 0,8 m mit fünf Wurzelanläufen und Maserknolle trägt einen Schaft von 0,47 m in Brusthöhe, astfrei bis 2,6 m, der als ein gerader Stamm bis zur Spitze in 14,1 m durchläuft. Sechsundzwanzig Äste verlassen ihn in fünf quirlähnlichen Etagen, jede gegen die untere verdreht; sie steigen mit 32 bis 54 Grad und bogen über, die untersten enden unter der Waagrechten, wie Lindenäste es tun. Jeder Ast trägt wechselständig zwei oder drei Nebenäste, jeder Nebenast zwei hängende Zweige, und jeder Zweig, jedes Nebenast- und Astende trägt eine hängende Laubmasse; die innersten Nebenäste tragen die Schattenblätter. Zehn dünne Stockausschläge von 0,7 bis 1,3 m stehen als Büschel rund um den Wurzelhals, jeder mit eigener kleiner Laubmasse."),
        100: ("the same stem and twenty-six arching branches with their boughs down to 37 mm; the twigs of 24 mm are below what prints and are gone, but their hanging foliage masses stay where the twigs carried them, with one mass per bough end and branch end; the brush of epicormic shoots is three shoots per side in one leaf mass that reaches down to the collar.", "Derselbe Stamm und sechsundzwanzig überbogene Äste mit ihren Nebenästen bis 37 mm; die Zweige von 24 mm liegen unter dem Druckbaren und entfallen, ihre hängenden Laubmassen bleiben aber dort, wo die Zweige sie trugen, dazu je eine Masse am Nebenast- und Astende; das Büschel der Stockausschläge sind je Seite drei Triebe in einer Laubmasse, die bis zum Wurzelhals hinabreicht."),
        200: ("stem and branches remain as wood, with the boughs from 52 mm drawn at the 85 mm that still print; thinner boughs and all twigs are gone. The foliage is kept as at 1:100, one hanging mass per twig, bough end and branch end with a coarser surface, because a mass of 0.5 to 1 m is 2.5 to 5 mm on paper and the sky between the tiers is what makes the crown a lime; the innermost boughs stay shaded, and one leading epicormic shoot per side stands for its group.", "Stamm und Äste bleiben als Holz, die Nebenäste ab 52 mm mit den 85 mm, die noch drucken; dünnere Nebenäste und alle Zweige entfallen. Das Laub bleibt wie bei 1:100, je Zweig, Nebenast- und Astende eine hängende Masse mit gröberer Oberfläche, denn eine Masse von 0,5 bis 1 m misst auf dem Papier 2,5 bis 5 mm, und der Himmel zwischen den Etagen macht die Krone zur Linde; die innersten Nebenäste bleiben beschattet, und je Seite steht ein führender Stockausschlag für seine Gruppe."),
        500: ("stem and branches, none thinner than 220 mm; every bough carries one hanging foliage mass that stands for its twigs (1 to 2.6 m wide, 2 to 5 mm on paper), the innermost shaded, and every branch end one; one epicormic shoot per side with one leaf mass at the collar. The outline is that of 1:200 with its tiers, not a stack of plates.", "Stamm und Äste, keiner dünner als 220 mm; jeder Nebenast trägt eine hängende Laubmasse, die für seine Zweige steht (1 bis 2,6 m breit, 2 bis 5 mm auf dem Papier), die innersten beschattet, und jedes Astende eine; je Seite ein Stockausschlag mit einer Laubmasse am Wurzelhals. Der Umriss ist der von 1:200 mit seinen Etagen, kein Stapel von Tellern."),
    },
    "drawing": {
        50: ("bole on its collar with the epicormic shoots, the straight stem, twenty-six arching branches with boughs and twigs, and the hanging leaf masses of every branch as one lobed outline with its openings.", "Schaft auf dem Wurzelhals mit den Stockausschlägen, der gerade Stamm, sechsundzwanzig überbogene Äste mit Nebenästen und Zweigen und die hängenden Laubmassen jedes Astes als ein gelappter Umriss mit seinen Lücken."),
        100: ("bole, stem, branches and boughs as tapered outlines per bone and the foliage of each bone as one lobed outline.", "Schaft, Stamm, Äste und Nebenäste als verjüngte Umrisse je Knochen und das Laub jedes Knochens als ein gelappter Umriss."),
        200: ("bole, stem and branches, and the foliage of each branch system as one lobed outline with its openings, the inner masses shaded.", "Schaft, Stamm und Äste sowie das Laub jedes Astsystems als ein gelappter Umriss mit seinen Lücken, die inneren Massen beschattet."),
        500: ("stem, branches and one lobed foliage outline per branch system, the inner masses shaded.", "Stamm, Äste und ein gelappter Laubumriss je Astsystem, die inneren Massen beschattet."),
    },
}
if os.environ.get("LIME_FOLDS"):
    import math
    for scale in (50, 100, 200, 500):
        for m in t.masses:
            if scale not in m["scales"]:
                continue
            q = t.mass_samples(m, scale)
            worst = 0.0
            for i in range(1, len(q)):
                d = K.norm(K.sub(q[i][0], q[i - 1][0]))
                c = max(-1.0, min(1.0, K.dot(K.unit(q[i][2]), K.unit(q[i - 1][2]))))
                worst = max(worst, max(q[i][1][0], q[i][1][1]) * math.acos(c) / max(d, 1e-9))
            if worst > 0.6:
                print(f"[DEBUG] fold {scale} {m['id']} {worst:.2f}")
    raise SystemExit(0)
if os.environ.get("LIME_DIFF"):
    D = SourceFileLoader("limediff", os.path.join(os.path.dirname(os.path.abspath(__file__)), "rework-small-leaved-lime-diff.py")).load_module()
    for pair in os.environ["LIME_DIFF"].split(","):
        coarse, fine = map(int, pair.split(":"))
        D.compare(t, fine, coarse, os.path.join(K.OUT, f"diff-{coarse}-{fine}.png"))
    raise SystemExit(0)
A.assemble(t, cfg)
