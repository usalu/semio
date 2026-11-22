"""🌳 Multi-stem silver birch (Betula pendula, three stems from one stool, about 30 years): the hand numbers of author 12. World x ahead (east), y up, z left (north)."""
import os
from importlib.machinery import SourceFileLoader

A = SourceFileLoader("rework12assemble", os.path.join(os.path.dirname(os.path.abspath(__file__)), "rework-12-assemble.py")).load_module()
K = A.K

t = K.Tree("multi-stem-birch", bark={"trunk": "birch-bark", "base": "birch-bark-base", "limb": "birch-bark"}, wood="birch-wood", wood_density=880,
           regions={"trunk": "stem-bark", "base": "stem-bark", "limb": "branch-bark", "sun": "leaf-crown"}, leaf_materials={"sun": "birch-leaf"}, tones={"trunk": 0.82, "base": 0.28, "limb": 0.5, "sun": 0.6})

t.axis("stool", [(0, 0, 0, 0.36), (0, 0.1, 0, 0.33), (0, 0.3, 0, 0.27), (0, 0.5, 0, 0.17)], kind="base", level=0, ground=True, lobes=[(3, 0.14, 10, 0.3)], sides=(30, 20, 14, 10), bones=[("root-plate", 0, 1.0, 0, "Root plate and stool of the three stems", "Wurzelteller und Stock der drei Stämme", "Stool", "Stock")])
STEMS = {
    "l": ([(-0.16, 0, -0.05, 0.12), (-0.3, 0.6, -0.08, 0.1), (-0.45, 1.3, -0.1, 0.09), (-1.0, 4.0, -0.2, 0.075), (-1.6, 7.0, -0.3, 0.055), (-1.9, 9.3, -0.3, 0.036), (-2.1, 10.8, -0.25, 0.02), (-2.3, 11.7, -0.2, 0.0098)], "West stem", "Weststamm"),
    "c": ([(0.05, 0, 0.12, 0.13), (0.05, 0.6, 0.16, 0.105), (0.02, 1.3, 0.2, 0.095), (-0.05, 4.0, 0.3, 0.08), (0, 7.0, 0.35, 0.06), (0.1, 9.5, 0.25, 0.04), (0.15, 11.5, 0.15, 0.022), (0.2, 12.6, 0.1, 0.012), (0.3, 13.0, 0.05, 0.0098)], "Centre stem", "Mittelstamm"),
    "r": ([(0.16, 0, -0.08, 0.115), (0.32, 0.6, -0.14, 0.095), (0.5, 1.3, -0.2, 0.085), (1.2, 4.0, -0.5, 0.07), (1.9, 6.8, -0.8, 0.052), (2.3, 9.0, -0.9, 0.034), (2.5, 10.4, -0.9, 0.02), (2.7, 11.2, -0.85, 0.0098)], "East stem", "Oststamm"),
}
BRANCHES = {
    "l": [(2.9, 205, 50, 3.0, 0.03), (3.9, 55, 52, 3.1, 0.029), (4.9, 150, 54, 3.2, 0.028), (5.8, 245, 54, 3.1, 0.026), (6.7, 30, 56, 2.9, 0.024), (7.6, 170, 56, 2.7, 0.022), (8.5, 300, 58, 2.4, 0.02), (9.3, 120, 58, 2.1, 0.019)],
    "c": [(3.2, 62, 50, 3.1, 0.031), (4.2, 300, 52, 3.2, 0.03), (5.2, 160, 54, 3.3, 0.029), (6.2, 20, 54, 3.2, 0.027), (7.2, 230, 56, 3.0, 0.025), (8.2, 130, 56, 2.8, 0.023), (9.2, 340, 58, 2.5, 0.021), (10.2, 200, 58, 2.2, 0.019)],
    "r": [(2.7, -25, 50, 3.0, 0.03), (3.7, 244, 52, 3.1, 0.028), (4.7, 40, 54, 3.2, 0.027), (5.6, 310, 54, 3.0, 0.025), (6.5, 130, 56, 2.8, 0.023), (7.4, 0, 56, 2.6, 0.022), (8.3, 220, 58, 2.3, 0.02), (9.0, 60, 58, 2.0, 0.019)],
}
WHIPS = [(0.34, 75, 1.2), (0.5, -70, 1.5), (0.64, 60, 1.6), (0.78, -55, 1.5), (0.9, 40, 1.3)]
like, group_bone = {}, {}
for s, (stations, en, de) in STEMS.items():
    f = K.heights(stations)
    top = stations[-1][1]
    t.axis(f"stem-{s}", stations, kind="trunk", level=0, parent="root-plate", ground=True, sides=(24, 14, 10, 8), bones=[
        (f"stem-{s}-1", 0, f(5.2), 0, f"{en}, lower part", f"{de}, unterer Teil", "Stem", "Stamm"),
        (f"stem-{s}-2", f(5.2), f(top - 2.2), 1, f"{en}, upper part", f"{de}, oberer Teil", "Stem", "Stamm"),
        (f"fol-{s}-top", f(top - 2.2), 1.0, 3, f"{en}, nodding tip with its foliage", f"{de}, nickende Spitze mit Laub", "Tip", "Spitze"),
    ])
    for n, (y, az, el, length, r0) in enumerate(BRANCHES[s], 1):
        bid = f"br-{s}-{n}"
        like[bid], like[f"fol-{s}-{n}"] = f"br-{s}-{'a' if n < 5 else 'b'}", f"fol-{s}-{'a' if n < 5 else 'b'}"
        t.branch(bid + "-axis", f"stem-{s}", f(y), az, el, length, r0, r1=0.0098, up=-128, bend=1.7, level=2, socket=True, bones=[
            (bid, 0, 0.62, 2, f"{en}, branch {n}", f"{de}, Ast {n}", "Branch", "Ast"),
            (f"fol-{s}-{n}", 0.62, 1.0, 3, f"{en}, branch {n}, hanging end", f"{de}, Ast {n}, hängendes Ende", "Hanging end", "Hängeende"),
        ])
        t.axes[bid + "-axis"]["keep"] = n in (2, 5, 8)
        group_bone[f"{s}{n}"] = bid
        for k, (at, daz, wlen) in enumerate(WHIPS, 1):
            wid = f"wh-{s}{n}{k}"
            t.branch(wid, bid + "-axis", at, az + daz, -34, wlen * length / 3.2, 0.0102, r1=0.0096, up=-50, level=3, collar=1.0, scales=(50,))
            t.spray(f"fol-{wid}", wid, 0.08, 0.17, 0.16, (4, 4, 0.24), over=0.1, scales=(50, 100), flat=1.0, peak=0.4, base=0.6, group=f"{s}{n}")
        t.spray(f"fol-{bid}", bid + "-axis", 0.45, 0.2, 0.18, (4, 5, 0.24), over=0.12, scales=(50, 100), flat=1.0, peak=0.4, base=0.6, group=f"{s}{n}")
        t.spray(f"veil-{bid}", bid + "-axis", 0.3, 0.95 * length / 3.2, 0.9, (6, 3, 0.2), f1=0.85, over=0.0, lift=-0.7, scales=(200,), flat=1.0, peak=0.5, base=0.55, group=f"{s}{n}")
        if n in (2, 5, 8):
            t.spray(f"crown-{bid}", bid + "-axis", 0.2, 1.7 * length / 3.2, 1.4, (5, 3, 0.16), f1=0.85, over=0.0, lift=-0.8, scales=(500,), flat=1.0, peak=0.5, base=0.55, group=f"{s}{n}")
    t.spray(f"fol-{s}-tip", f"stem-{s}", f(top - 1.9), 0.24, 0.22, (4, 4, 0.22), over=0.12, drop=0.25, scales=(50, 100), bone=f"fol-{s}-top", flat=1.0, group=f"{s}-top")
    t.spray(f"fol-{s}-tip", f"stem-{s}", f(top - 2.4), 0.6, 0.6, (5, 3, 0.16), over=0.15, drop=0.2, scales=(200, 500), bone=f"fol-{s}-top", flat=1.0, group=f"{s}-top")
    group_bone[f"{s}-top"] = f"fol-{s}-top"

cfg = {
    "like": like,
    "droop_poses": ["ice"],
    "leaf_density": 0.3,
    "group_bone": group_bone,
    "footprint": 1.0,
    "stems": [["stem-l-1", "stem-l-2"], ["stem-c-1", "stem-c-2"], ["stem-r-1", "stem-r-2"]],
    "emitters": {"transpiration-crown": {"on": ["fol-wh-l23", "fol-wh-l53", "fol-wh-l83", "fol-wh-c23", "fol-wh-c53", "fol-wh-c83", "fol-wh-r23", "fol-wh-r53", "fol-wh-r83"], "diameter": 0.3, "cls": "sun", "prefix": "tp-crown", "title": ("Transpiration of the crown leaves", "Transpiration der Kronenblätter")}},
    "scaffolds": ["stem-l", "stem-c", "stem-r", "br-c-3-axis"],
    "habit": {
        50: ("the birch is built as it grows. Three stems of 0.17 to 0.19 m at breast height rise from one dark, fissured stool of 0.7 m, lean apart and straighten, the centre one to 13 m, each ending in a nodding tip. Every stem carries eight thin branches from 2.7 m upwards that leave it at 50 to 58 degrees, arch over and hang down with their last third, and from every branch five whips of 19 mm hang 1.2 to 1.6 m. The small leaves hang along these whips and branch ends: one strand of foliage only 0.35 m thick per whip, so the crown stays a loose veil of 147 separate strands through which the white stems show.", "Die Birke ist gebaut, wie sie wächst. Drei Stämme von 0,17 bis 0,19 m in Brusthöhe steigen aus einem dunklen, rissigen Stock von 0,7 m, neigen sich auseinander und richten sich auf, der mittlere bis 13 m, jeder mit nickender Spitze. Jeder Stamm trägt ab 2,7 m acht dünne Äste, die ihn mit 50 bis 58 Grad verlassen, überbogen und mit dem letzten Drittel herabhängen, und von jedem Ast hängen fünf Hängezweige von 19 mm 1,2 bis 1,6 m herab. Die kleinen Blätter hängen an diesen Hängezweigen und Astenden: je Hängezweig ein nur 0,35 m dicker Laubstrang, die Krone bleibt so ein lockerer Schleier aus 147 getrennten Strängen, durch den die weißen Stämme sichtbar sind."),
        100: ("the same stool, three stems and twenty-four arching branches; the whips are gone as wood (thinner than the 37 mm that still print), wood thinner than that is kept at its true thickness as slender solids, and every strand of foliage hangs in its place.", "Derselbe Stock, drei Stämme und vierundzwanzig überbogene Äste; die Hängezweige entfallen als Holz (dünner als die 37 mm, die noch drucken), dünneres Holz bleibt in wahrer Dicke als schlanke Körper erhalten, und jeder Laubstrang hängt an seinem Platz."),
        200: ("stool and stems; the branches stay as slender solids of true thickness except three per stem, and the strands of each branch are merged into one hanging veil.", "Stock und Stämme; die Äste bleiben schlanke Körper in wahrer Dicke bis auf drei je Stamm, und die Stränge jedes Astes sind zu einem hängenden Schleier verschmolzen."),
        500: ("stool and stems, three thickened branches per stem, and their foliage as one hanging mass each plus the three tips: the light, open crown of three stems.", "Stock und Stämme, drei verdickte Äste je Stamm und ihr Laub als je eine hängende Masse sowie die drei Spitzen: die lichte, offene Krone aus drei Stämmen."),
    },
    "drawing": {
        50: ("dark stool, three white stems, twenty-four arching branches with their hanging whips, and the leaf strands of every branch as one outline with its openings.", "dunkler Stock, drei weiße Stämme, vierundzwanzig überbogene Äste mit ihren Hängezweigen und die Laubstränge jedes Astes als ein Umriss mit seinen Lücken."),
        100: ("stool, stems and the leaf strands of every branch.", "Stock, Stämme und die Laubstränge jedes Astes."),
        200: ("stool, stems and one hanging veil per branch.", "Stock, Stämme und ein hängender Schleier je Ast."),
        500: ("stool, three stems and the hanging crown masses.", "Stock, drei Stämme und die hängenden Kronenmassen."),
    },
}
A.assemble(t, cfg)
