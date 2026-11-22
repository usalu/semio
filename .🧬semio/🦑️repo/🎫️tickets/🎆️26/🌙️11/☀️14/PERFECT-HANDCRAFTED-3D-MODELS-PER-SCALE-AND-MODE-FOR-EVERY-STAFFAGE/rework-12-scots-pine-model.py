"""🌲 Scots pine (Pinus sylvestris, open-grown, about 110 years): the hand numbers of author 12. World x ahead (east), y up, z left (north)."""
import os
from importlib.machinery import SourceFileLoader

A = SourceFileLoader("rework12assemble", os.path.join(os.path.dirname(os.path.abspath(__file__)), "rework-12-assemble.py")).load_module()
K = A.K

t = K.Tree("scots-pine", bark={"lower": "pine-bark-lower", "upper": "pine-bark-upper", "limb": "pine-bark-upper"}, wood="pine-wood", wood_density=860,
           regions={"lower": "bark-lower", "upper": "bark-upper", "limb": "limb-bark", "sun": "needles-sun", "shade": "needles-shade"},
           leaf_materials={"sun": "pine-needles-sun", "shade": "pine-needles-shade"}, tones={"lower": 0.3, "upper": 0.5, "limb": 0.46, "sun": 0.46, "shade": 0.34})

LOWER = [(0, 0, 0, 0.4), (0, 0.12, 0, 0.345), (0, 0.4, 0, 0.3), (0.01, 0.9, 0, 0.268), (0.02, 1.3, 0, 0.255), (0.05, 3.0, 0.01, 0.24), (0.1, 5.0, 0.02, 0.225), (0.16, 7.0, 0.02, 0.21), (0.22, 9.0, 0.01, 0.195)]
UPPER = [(0.214, 8.8, 0.01, 0.196), (0.25, 10.0, 0, 0.187), (0.3, 12.0, -0.02, 0.168), (0.3, 14.0, -0.04, 0.145), (0.22, 16.0, -0.02, 0.118), (0.1, 17.8, 0.04, 0.09), (0.02, 19.2, 0.1, 0.062), (0.08, 20.3, 0.12, 0.038), (0.2, 21.0, 0.08, 0.02), (0.32, 21.4, 0.04, 0.011)]
fl, fu = K.heights(LOWER), K.heights(UPPER)
t.axis("stem-lower", LOWER, kind="lower", level=0, ridges=(14, 0.014), lobes=[(4, 0.1, 40, 0.25)], ground=True, bones=[
    ("root-plate", 0, fl(0.4), 0, "Root plate and collar", "Wurzelteller und Wurzelhals", "Root collar", "Wurzelhals"),
    ("trunk-1", fl(0.4), fl(4.7), 0, "Lower trunk with plated bark", "Unterer Stamm mit Plattenborke", "Trunk", "Stamm"),
    ("trunk-2", fl(4.7), 1.0, 0, "Middle trunk with the branch stubs", "Mittlerer Stamm mit den Aststummeln", "Trunk", "Stamm"),
])
t.axis("stem-upper", UPPER, kind="upper", level=0, parent="trunk-2", bones=[
    ("trunk-3", 0, fu(14.5), 0, "Upper trunk with mirror bark", "Oberer Stamm mit Spiegelrinde", "Trunk", "Stamm"),
    ("leader", fu(14.5), fu(19.0), 1, "Crown stem", "Kronenstamm", "Crown stem", "Kronenstamm"),
    ("pl-top", fu(19.0), 1.0, 3, "Top shoots with the crown plate", "Wipfeltriebe mit dem Kronenteller", "Top shoot", "Wipfeltrieb"),
])
for n, (az, reach, r0) in enumerate([(20, 1.3, 0.11), (110, 1.1, 0.1), (200, 1.3, 0.11), (290, 1.1, 0.1)], 1):
    d = K.direction(az, 0)
    t.axis(f"root-{n}", [(d[0] * 0.1, 0.38, d[2] * 0.1, r0), (d[0] * 0.48, 0.14, d[2] * 0.48, r0 * 0.8), (d[0] * reach * 0.7, 0.04, d[2] * reach * 0.7, r0 * 0.6), (d[0] * reach, 0.0, d[2] * reach, r0 * 0.45)], bone="root-plate", kind="lower", level=2, aspect=1.4, ground=True, sides=(12, 8, 6, 6))
for key, y, az, el, length, r0, en, de in [("w", 7.0, 170, 22, 0.85, 0.042, "Dead branch stub, west", "Toter Aststummel, West"), ("e", 8.2, -20, 16, 0.7, 0.038, "Dead branch stub, east", "Toter Aststummel, Ost")]:
    t.branch(f"stub-{key}-axis", "stem-lower", fl(y), az, el, length, r0, r1=r0 * 0.7, level=2, collar=1.3, bones=[(f"stub-{key}", 0, 1.0, 2, en, de, "Stub", "Stummel")])

LIMBS = [
    ("w1", 12.0, 175, 8, 5.6, 0.1, 18, -20, "Lowest west limb", "Unterster Westast"), ("e1", 12.8, -15, 12, 5.2, 0.095, 16, 18, "Lowest east limb", "Unterster Ostast"),
    ("n1", 13.6, 64, 14, 3.8, 0.08, 14, 14, "Lower north limb", "Unterer Nordast"), ("s1", 14.2, -116, 14, 3.6, 0.075, 14, -12, "Lower south limb", "Unterer Südast"),
    ("w2", 15.2, 150, 22, 5.0, 0.085, 12, 16, "Middle north-west limb", "Mittlerer Nordwestast"), ("e2", 16.0, -35, 24, 4.8, 0.08, 10, -16, "Middle south-east limb", "Mittlerer Südostast"),
    ("n2", 16.9, 55, 28, 3.6, 0.07, 8, 10, "Middle north-east limb", "Mittlerer Nordostast"), ("s2", 17.6, -125, 28, 3.4, 0.065, 8, -10, "Middle south-west limb", "Mittlerer Südwestast"),
    ("w3", 18.4, -170, 36, 4.0, 0.065, 0, 14, "Upper west limb", "Oberer Westast"), ("e3", 19.0, 15, 38, 3.8, 0.06, 0, -14, "Upper east limb", "Oberer Ostast"),
]
BOUGHS = [(0.38, 58, 10, 2.3, 0.036), (0.58, -54, 8, 2.1, 0.032), (0.78, 44, 12, 1.6, 0.027)]
TWIGS = [(0.45, 46, 34, 0.7, 0.0125), (0.75, -42, 38, 0.6, 0.012)]
for key, y, az, el, length, r0, up, turn, en, de in LIMBS:
    low = key.endswith("1")
    t.branch(key, "stem-upper", fu(y), az, el, length, r0, r1=0.011, up=up, turn=turn, wave=0.2 if key[0] in "wn" else -0.2, level=1, socket=True, bones=[
        (f"limb-{key}", 0, 0.55, 1, en, de, "Limb", "Starkast"),
        (f"pl-{key}", 0.55, 1.0, 2, f"{en}, outer part with its needle plates", f"{de}, Außenteil mit den Nadeltellern", "Branch", "Ast"),
    ])
    k = length / 5.0
    for n, (at, daz, bel, blen, br0) in enumerate(BOUGHS, 1):
        bid = f"{key}-b{n}"
        sign = 1 if key[0] in "we" else -1
        t.branch(bid, key, at, az + sign * daz, bel, blen * k + 0.4, br0 * k ** 0.5, r1=0.01, up=22, level=2, wave=0.08, scales=(50, 100, 200) if n == 1 else (50, 100))
        for m, (tat, tdaz, tel, tlen, tr0) in enumerate(TWIGS, 1):
            t.branch(f"{bid}-t{m}", bid, tat, az + sign * daz + tdaz, tel, tlen, tr0, r1=0.0098, up=20, level=3, scales=(50,))
            t.spray(f"pad-{bid}-t{m}", f"{bid}-t{m}", 0.3, 0.5, 0.42, (6, 2, 0.22), over=0.3, lift=0.1, shade=low and n == 1, scales=(50,), flat=0.6, group=key)
        shade = low and n == 1
        t.spray(f"pad-{bid}", bid, 0.4, 0.75 * k + 0.5, 0.6, (7, 3, 0.22), over=0.4, lift=0.2, shade=shade, scales=(50, 100), flat=0.6, peak=0.55, base=0.5, group=key)
    t.spray(f"pad-{key}-tip", key, 0.74, 1.0 * k + 0.4, 0.7, (8, 3, 0.22), over=0.55, lift=0.22, scales=(50, 100), flat=0.6, peak=0.55, base=0.5, group=key)
    t.spray(f"pad-{key}-inner", key, 0.3, 1.75 * k + 0.5, 0.85, (7, 3, 0.18), f1=0.66, over=0.0, lift=0.25, shade=low, scales=(200,), flat=0.6, peak=0.5, base=0.5, group=key)
    t.spray(f"pad-{key}-outer", key, 0.6, 2.0 * k + 0.6, 0.9, (7, 3, 0.18), over=1.3, lift=0.25, scales=(200,), flat=0.6, group=key)
    t.spray(f"plate-{key}", key, 0.3, 2.2 * k + 0.6, 1.0, (6, 3, 0.16), over=1.4, lift=0.25, shade=key in ("n1", "s1"), scales=(500,), flat=0.6, peak=0.55, base=0.5, group=key)
TOP = [(19.3, 240, 28, 2.4, 0.038), (19.6, 120, 30, 2.2, 0.036), (20.1, -60, 32, 2.0, 0.032), (20.4, 150, 34, 1.8, 0.03), (20.6, 210, 34, 1.7, 0.028), (20.9, 30, 36, 1.5, 0.024)]
for n, (y, az, el, length, r0) in enumerate(TOP, 1):
    bid = f"top-b{n}"
    t.branch(bid, "stem-upper", fu(y), az, el, length, r0, r1=0.01, up=10, level=2, scales=(50, 100, 200) if n < 3 else (50, 100))
    t.spray(f"pad-{bid}", bid, 0.4, 0.95, 0.62, (7, 3, 0.22), over=0.4, lift=0.2, scales=(50, 100), flat=0.6, peak=0.55, base=0.5, group="top")
t.spray("pad-top", "stem-upper", fu(20.5), 0.9, 0.8, (7, 3, 0.2), over=0.35, lift=0.0, scales=(50, 100), bone="pl-top", flat=1.0, group="top")
t.mass("plate-top", "pl-top", [(-2.0, 20.5, 0.9, 1.5, 0.8), (-0.6, 21.0, 0.2, 2.2, 1.0), (0.9, 21.1, -0.3, 2.2, 1.0), (2.2, 20.6, -0.9, 1.5, 0.8)], (7, 3, 0.17), scales=(200, 500), flat=0.6, group="top")

cfg = {
    "like": {"limb-n1": "limb-w1", "limb-s1": "limb-e1", "limb-n2": "limb-w2", "limb-s2": "limb-e2", "pl-n1": "pl-w1", "pl-s1": "pl-e1", "pl-n2": "pl-w2", "pl-s2": "pl-e2"},
    "droop_poses": ["snow"],
    "leaf_density": 0.9,
    "group_bone": dict({key: f"pl-{key}" for key, *_ in LIMBS}, top="pl-top"),
    "stem_kind": "lower",
    "emitters": {
        "transpiration-sun": {"on": ["pad-w1-tip", "pad-e1-tip", "pad-w2-tip", "pad-e2-tip", "pad-top"], "diameter": 0.9, "cls": "sun", "prefix": "tp-sun", "title": ("Transpiration of the sun needles", "Transpiration der Sonnennadeln")},
        "transpiration-shade": {"on": ["pad-w1-b1", "pad-e1-b1", "pad-n1-b1", "pad-s1-b1", "pad-w1-b1", "pad-e1-b1"][:4] + [], "diameter": 0.7, "cls": "shade", "prefix": "tp-shade", "title": ("Transpiration of the shade needles", "Transpiration der Schattennadeln")},
    },
    "scaffolds": ["w1", "e1", "w2", "e2"],
    "habit": {
        50: ("the pine is built as it grows. A collar of 0.8 m with four root spurs carries a slightly leaning stem of 0.51 m at breast height, clad in thick grey plates to 9 m and in thin orange mirror bark above, bare of living branches to 12 m with two dead stubs at 7 and 8.2 m. Ten twisted limbs leave the upper stem between 12 and 19 m, the lowest nearly level and up to 5.6 m long, each with three flat boughs whose twigs turn up like candles; the stem itself bends over into the flat top at 21.4 m. The needles sit in tufts at the shoot ends only: one flat-bottomed, rounded tuft over every twig, bough end and limb end, so the crown is tiers of separate needle clouds with open sky between them, sun needles on top and outside, shade needles on the innermost plates of the lowest limbs.", "Die Kiefer ist gebaut, wie sie wächst. Ein Wurzelhals von 0,8 m mit vier Wurzelanläufen trägt einen leicht geneigten Stamm von 0,51 m in Brusthöhe, bis 9 m mit dicker grauer Plattenborke, darüber mit dünner orangefarbener Spiegelrinde, astfrei bis 12 m mit zwei toten Stummeln in 7 und 8,2 m. Zehn gedrehte Starkäste verlassen den oberen Stamm zwischen 12 und 19 m, die untersten fast waagrecht und bis 5,6 m lang, jeder mit drei flachen Ästen, deren Zweige wie Kerzen aufgerichtet sind; der Stamm selbst biegt in 21,4 m in den flachen Wipfel um. Die Nadeln sitzen nur in Büscheln an den Triebenden: ein unten flaches, gerundetes Büschel über jedem Zweig, Ast- und Starkastende, die Krone besteht so aus Etagen getrennter Nadelwolken mit offenem Himmel dazwischen, Sonnennadeln oben und außen, Schattennadeln an den innersten Tellern der untersten Starkäste."),
        100: ("the same two-barked stem with its stubs, ten limbs and their boughs down to 37 mm; the candle twigs are gone, every bough end and limb end keeps its needle plate.", "Derselbe zweirindige Stamm mit seinen Stummeln, zehn Starkäste und ihre Äste bis 37 mm; die Kerzenzweige entfallen, jedes Ast- und Starkastende behält seinen Nadelteller."),
        200: ("stem with stubs, limbs and their first boughs remain as wood; every limb is an inner and an outer needle plate, the top one plate.", "Stamm mit Stummeln, Starkäste und ihre ersten Äste bleiben als Holz; jeder Starkast ist ein innerer und ein äußerer Nadelteller, der Wipfel ein Teller."),
        500: ("stem and limbs, none thinner than 240 mm, and one needle plate per limb plus the top plate: the bare stem under the tiered umbrella crown.", "Stamm und Starkäste, keiner dünner als 240 mm, und je Starkast ein Nadelteller sowie der Wipfelteller: der kahle Stamm unter der gestuften Schirmkrone."),
    },
    "drawing": {
        50: ("plated lower and smooth upper stem with two stubs, ten limbs with boughs and candle twigs, and the needle plates of every limb as one flat lobed outline.", "plattiger unterer und glatter oberer Stamm mit zwei Stummeln, zehn Starkäste mit Ästen und Kerzenzweigen und die Nadelteller jedes Starkastes als ein flacher gelappter Umriss."),
        100: ("stem, stubs, limbs and boughs per bone and the needle plates of each limb.", "Stamm, Stummel, Starkäste und Äste je Knochen und die Nadelteller jedes Starkastes."),
        200: ("stem, stubs, limbs and two needle plates per limb.", "Stamm, Stummel, Starkäste und zwei Nadelteller je Starkast."),
        500: ("bare stem, limb stubs and eleven needle plates.", "kahler Stamm, Astansätze und elf Nadelteller."),
    },
}
cfg["emitters"]["transpiration-shade"]["on"] = ["pad-w1-b1", "pad-e1-b1", "pad-n1-b1", "pad-s1-b1"]
A.assemble(t, cfg)
