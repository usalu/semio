"""🍎 Apple tree (Malus domestica, half-standard orchard tree of about 25 years in fruit): the hand numbers of author 12. World x ahead (east), y up, z left (north)."""
import os
from importlib.machinery import SourceFileLoader

A = SourceFileLoader("rework12assemble", os.path.join(os.path.dirname(os.path.abspath(__file__)), "rework-12-assemble.py")).load_module()
K = A.K

t = K.Tree("apple-tree", bark={"trunk": "apple-bark", "limb": "apple-bark-young", "spur": "apple-twig"}, wood="apple-wood", wood_density=880,
           regions={"trunk": "trunk-bark", "limb": "limb-bark", "spur": "spur-twig", "sun": "leaf-sun", "shade": "leaf-shade"},
           leaf_materials={"sun": "apple-leaf-sun", "shade": "apple-leaf-shade"}, tones={"trunk": 0.3, "limb": 0.36, "spur": 0.4, "sun": 0.58, "shade": 0.42})

STEM = [(0, 0, 0, 0.25), (0, 0.08, 0, 0.21), (0, 0.3, 0, 0.175), (0.01, 0.8, 0, 0.152), (0.02, 1.3, 0, 0.145), (0.03, 1.7, 0, 0.15), (0, 2.4, 0.02, 0.115), (-0.05, 3.2, 0.03, 0.088), (-0.02, 4.0, 0, 0.064), (0.05, 4.8, -0.02, 0.042), (0.08, 5.4, 0, 0.024), (0.1, 5.9, 0, 0.0105)]
fy = K.heights(STEM)
t.axis("stem", STEM, kind="trunk", level=0, ridges=(12, 0.008), lobes=[(4, 0.1, 20, 0.15)], ground=True, bones=[
    ("root-plate", 0, fy(0.3), 0, "Root plate and collar", "Wurzelteller und Wurzelhals", "Root collar", "Wurzelhals"),
    ("trunk", fy(0.3), fy(1.6), 0, "Trunk (half-standard stem)", "Stamm (Halbstamm)", "Trunk", "Stamm"),
    ("ld", fy(1.6), fy(3.2), 0, "Central leader, lower part", "Mitteltrieb, unterer Teil", "Leader", "Mitteltrieb"),
    ("ld-b", fy(3.2), fy(4.8), 1, "Central leader, upper part", "Mitteltrieb, oberer Teil", "Leader", "Mitteltrieb"),
    ("fol-top", fy(4.8), 1.0, 3, "Leader tip with its foliage", "Triebspitze mit Laub", "Tip", "Spitze"),
])
for n, (az, reach, r0) in enumerate([(25, 0.75, 0.07), (115, 0.65, 0.065), (205, 0.75, 0.07), (295, 0.65, 0.065)], 1):
    d = K.direction(az, 0)
    t.axis(f"root-{n}", [(d[0] * 0.06, 0.24, d[2] * 0.06, r0), (d[0] * 0.3, 0.09, d[2] * 0.3, r0 * 0.8), (d[0] * reach * 0.7, 0.03, d[2] * reach * 0.7, r0 * 0.6), (d[0] * reach, 0.0, d[2] * reach, r0 * 0.5)], bone="root-plate", kind="trunk", level=2, aspect=1.4, ground=True, sides=(12, 8, 6, 6))

SCAFFOLDS = [
    ("w1", 1.55, 176, 26, 3.5, 0.068, 16, -14, 0.2, "Lower west scaffold limb", "Unterer westlicher Leitast"), ("e1", 1.68, -8, 28, 3.4, 0.066, 14, 12, -0.2, "Lower east scaffold limb", "Unterer östlicher Leitast"),
    ("n1", 1.8, 62, 30, 3.2, 0.062, 14, 14, 0.18, "Lower north scaffold limb", "Unterer nördlicher Leitast"), ("s1", 1.92, -118, 28, 3.2, 0.06, 16, -12, -0.18, "Lower south scaffold limb", "Unterer südlicher Leitast"),
    ("w2", 2.7, 128, 40, 2.6, 0.046, 10, 10, 0.14, "Upper north-west scaffold limb", "Oberer nordwestlicher Leitast"), ("e2", 2.95, -44, 42, 2.5, 0.044, 8, -10, -0.14, "Upper south-east scaffold limb", "Oberer südöstlicher Leitast"),
    ("s2", 3.2, -150, 44, 2.3, 0.04, 8, 10, 0.12, "Upper south-west scaffold limb", "Oberer südwestlicher Leitast"),
]
BOUGHS = [(0.34, 58, 24, 1.5, 0.024), (0.54, -54, 22, 1.4, 0.022), (0.74, 46, 28, 1.1, 0.019)]
SPURS = [(0.22, 80, 50, 0.24), (0.44, -90, 40, 0.22), (0.64, 70, 55, 0.2), (0.86, -60, 45, 0.18)]
APPLES = {}
like = {}
for key, y, az, el, length, r0, up, turn, wave, en, de in SCAFFOLDS:
    old = {"n1": "w1", "s1": "e1", "s2": "w2"}.get(key, key)
    like[f"sc-{key}a"], like[f"sc-{key}b"], like[f"fol-{key}"] = f"sc-{old}a", f"sc-{old}b", f"fol-{old}"
    t.branch(key, "stem", fy(y), az, el, length, r0, r1=0.0098, up=up, turn=turn, wave=wave, level=1, socket=True, bones=[
        (f"sc-{key}a", 0, 0.5, 1, en, de, "Scaffold limb", "Leitast"),
        (f"sc-{key}b", 0.5, 0.84, 2, f"{en}, outer part", f"{de}, Außenteil", "Branch", "Ast"),
        (f"fol-{key}", 0.84, 1.0, 3, f"{en}, end shoots with foliage", f"{de}, Endtriebe mit Laub", "End shoot", "Endtrieb"),
    ])
    k = length / 3.4
    APPLES[key] = []
    for n, (at, daz, bel, blen, br0) in enumerate(BOUGHS, 1):
        bid = f"{key}-b{n}"
        sign = 1 if key[0] in "we" else -1
        t.branch(bid, key, at, az + sign * daz, bel, blen * k, br0 * k ** 0.5, r1=0.0098, up=-18, level=2, wave=0.06, scales=(50, 100))
        inner = n == 1 and key.endswith("1")
        t.spray(f"fol-{bid}", bid, 0.3, 0.5 * k + 0.14, 0.38, (6, 3, 0.2), over=0.15, lift=0.1, shade=inner, scales=(50, 100), flat=0.7, group=key)
        APPLES[key].append((bid, (0.45, 0.7, 0.92)))
    for n, (at, daz, sel, slen) in enumerate(SPURS, 1):
        sid = f"{key}-spur{n}"
        t.branch(sid, key, at, az + daz, sel, slen, 0.0105, r1=0.0098, up=10, level=3, kind="spur", collar=1.0)
        t.axes[sid]["keep"] = n == 2
        t.axes[sid]["scales"] = (50, 100, 200, 500) if n == 2 else (50, 100)
        t.spray(f"fol-{sid}", sid, 0.3, 0.2, 0.18, (4, 2, 0.2), over=0.1, scales=(50,), flat=1.0, group=key)
    t.spray(f"fol-{key}-tip", key, 0.78, 0.58, 0.44, (6, 3, 0.2), over=0.25, lift=0.1, scales=(50, 100), flat=0.7, group=key)
    t.spray(f"fol-{key}-inner", key, 0.2, 0.95 * k + 0.1, 0.55, (6, 3, 0.16), f1=0.6, over=0.0, lift=0.2, shade=key.endswith("1"), scales=(200,), flat=0.6, peak=0.5, base=0.5, group=key)
    t.spray(f"fol-{key}-outer", key, 0.54, 1.1 * k + 0.15, 0.6, (6, 3, 0.16), over=0.6, lift=0.2, scales=(200,), flat=0.6, group=key)
    t.spray(f"crown-{key}", key, 0.22, 1.15 * k + 0.1, 0.72, (5, 3, 0.14), over=0.5, lift=0.25, shade=key in ("n1", "s1"), scales=(500,), flat=0.6, peak=0.55, base=0.5, group=key)
TOP = [(3.7, 100, 34, 1.3, 0.022), (4.2, -20, 36, 1.2, 0.02), (4.7, 200, 38, 1.0, 0.018), (5.1, 60, 40, 0.8, 0.016)]
for n, (y, az, el, length, r0) in enumerate(TOP, 1):
    bid = f"top-b{n}"
    t.branch(bid, "stem", fy(y), az, el, length, r0, r1=0.0098, up=-10, level=2, scales=(50, 100))
    t.spray(f"fol-{bid}", bid, 0.3, 0.45, 0.34, (5, 3, 0.2), over=0.15, lift=0.1, scales=(50, 100), flat=0.7, group="top")
t.spray("fol-top-shoot", "stem", fy(5.2), 0.34, 0.32, (5, 3, 0.2), over=0.15, scales=(50, 100), bone="fol-top", flat=1.0, group="top")
t.spray("crown-top", "stem", fy(3.6), 1.0, 0.95, (6, 4, 0.16), over=0.15, scales=(200, 500), bone="ld-b", flat=1.0, peak=0.45, base=0.6, group="top")


def apples(scale, tree):
    """🍎 The fruit: apples of 74 mm hanging under the boughs at the stated fractions; from 1:200 on one cluster per bough, at 1:500 one per scaffold limb."""
    out = []
    for key, rows in APPLES.items():
        for bid, fractions in rows:
            bone = tree.bone_at(key, 0.5) if False else tree.axes[bid]["bone"]
            path = tree.axes[bid]["path"]
            if scale <= 100:
                for i, f in enumerate(fractions, 1):
                    p = path.at(f)[0]
                    side = 0.05 if i % 2 else -0.05
                    mesh = K.sphere((p[0] + side, p[1] - 0.075, p[2] - side), 0.037, 8 if scale == 50 else 6)
                    out.append({"solid": tree.solid(f"apple-{bid}-{i}", bone, mesh, "apple-fruit", 0.7, role="detail", digits=4), "region": "fruit", "bone": bone})
            elif scale == 200 or bid.endswith("b2"):
                p = path.at(0.7)[0]
                r = 0.06 if scale == 200 else 0.14
                out.append({"solid": tree.solid(f"apples-{bid}", bone, K.sphere((p[0], p[1] - r - 0.04, p[2]), r, 6), "apple-fruit", 0.7, role="detail", digits=3), "region": "fruit", "bone": bone})
    return out


def apple_shapes(scale, tree):
    groups = {}
    for item in apples(scale, tree):
        groups.setdefault(item["bone"], []).append(item["solid"]["id"])
    return [{"id": f"fruit-{bone}", "bone": bone, "role": "detail", "filled": True, "tone": 0.7, "region": "fruit", "material": "apple-fruit", "solids": ids, "cellF": 0.25, "tolF": 0.3, "layer": 1000} for bone, ids in groups.items()]


cfg = {
    "like": like,
    "leaf_density": 0.5,
    "group_bone": dict({key: f"sc-{key}b" for key, *_ in SCAFFOLDS}, top="ld-b"),
    "extras": apples,
    "extra_shapes": apple_shapes,
    "emitters": {
        "transpiration-sun": {"on": ["fol-w1-tip", "fol-e1-tip", "fol-n1-tip", "fol-s1-tip", "fol-w2-tip", "fol-e2-tip", "fol-s2-tip"], "diameter": 0.5, "cls": "sun", "prefix": "tp-sun", "title": ("Transpiration of the sun leaves", "Transpiration der Sonnenblätter")},
        "transpiration-shade": {"on": ["fol-w1-b1", "fol-e1-b1", "fol-n1-b1", "fol-s1-b1"], "diameter": 0.4, "cls": "shade", "prefix": "tp-shade", "title": ("Transpiration of the shade leaves", "Transpiration der Schattenblätter")},
    },
    "scaffolds": ["w1", "e1", "n1", "s1"],
    "habit": {
        50: ("the apple tree is built as it is grown. A collar of 0.5 m with four root spurs carries a half-standard stem of 0.29 m, clear to 1.5 m, that continues as a modified central leader to 5.9 m. Four scaffold limbs leave it between 1.55 and 1.9 m at 26 to 30 degrees, crooked from years of pruning, three more between 2.7 and 3.2 m at about 42 degrees; each carries three short fruiting boughs that sag under the crop and four stubby spurs. The leaves sit in tufts: one low mass on every bough, limb end and spur, shade leaves on the innermost boughs, with the open centre of an orchard crown between the limbs. Sixty-three apples of 74 mm hang under the boughs.", "Der Apfelbaum ist gebaut, wie er erzogen wird. Ein Wurzelhals von 0,5 m mit vier Wurzelanläufen trägt einen Halbstamm von 0,29 m, astfrei bis 1,5 m, der als abgeleiteter Mitteltrieb bis 5,9 m weiterläuft. Vier Leitäste verlassen ihn zwischen 1,55 und 1,9 m mit 26 bis 30 Grad, vom jahrelangen Schnitt gekniet, drei weitere zwischen 2,7 und 3,2 m mit etwa 42 Grad; jeder trägt drei kurze Fruchtäste, die unter dem Behang durchhängen, und vier kurze Fruchtspieße. Die Blätter sitzen in Büscheln: eine niedrige Masse an jedem Fruchtast, Leitastende und Fruchtspieß, Schattenblätter an den innersten Fruchtästen, mit der offenen Mitte einer Obstbaumkrone zwischen den Leitästen. Dreiundsechzig Äpfel von 74 mm hängen unter den Fruchtästen."),
        100: ("the same stem, seven scaffold limbs, their fruiting boughs and spurs; the spur tufts are gone, every bough and limb end keeps its leaf mass and every apple hangs in its place.", "Derselbe Stamm, sieben Leitäste, ihre Fruchtäste und Fruchtspieße; die Büschel der Spieße entfallen, jeder Fruchtast und jedes Leitastende behält seine Laubmasse, und jeder Apfel hängt an seinem Platz."),
        200: ("stem and scaffold limbs with one spur each; each limb is an inner and an outer leaf mass, and the apples of each bough are one cluster.", "Stamm und Leitäste mit je einem Fruchtspieß; jeder Leitast ist eine innere und eine äußere Laubmasse, und die Äpfel jedes Fruchtastes sind ein Behang."),
        500: ("stem and scaffold limbs, none thinner than 240 mm, one leaf mass per limb around the top mass, and one fruit cluster per limb.", "Stamm und Leitäste, keiner dünner als 240 mm, je Leitast eine Laubmasse um die Wipfelmasse und je Leitast ein Fruchtbehang."),
    },
    "drawing": {
        50: ("stem on its collar, seven crooked scaffold limbs with fruiting boughs and spurs, the leaf tufts of every limb as one outline with its openings, and the apples in front.", "Stamm auf dem Wurzelhals, sieben gekniete Leitäste mit Fruchtästen und Fruchtspießen, die Laubbüschel jedes Leitastes als ein Umriss mit seinen Lücken und die Äpfel davor."),
        100: ("stem, scaffold limbs, boughs, the leaf masses of each limb and the apples.", "Stamm, Leitäste, Fruchtäste, die Laubmassen jedes Leitastes und die Äpfel."),
        200: ("stem, scaffold limbs, two leaf masses per limb and the fruit clusters.", "Stamm, Leitäste, zwei Laubmassen je Leitast und die Fruchtbehänge."),
        500: ("stem, limb stubs, crown lobes and fruit clusters.", "Stamm, Astansätze, Kronenlappen und Fruchtbehänge."),
    },
}
A.assemble(t, cfg)
