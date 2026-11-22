"""🌳 Weeping willow (Salix x sepulcralis 'Chrysocoma', about 40 years): the hand numbers of author 12. World x ahead (east), y up, z left (north)."""
import os
from importlib.machinery import SourceFileLoader

A = SourceFileLoader("rework12assemble", os.path.join(os.path.dirname(os.path.abspath(__file__)), "rework-12-assemble.py")).load_module()
K = A.K

t = K.Tree("weeping-willow", bark="willow-bark", wood="willow-wood", wood_density=780, regions={"trunk": "trunk-bark", "limb": "limb-bark", "sun": "leaf-curtain"}, leaf_materials={"sun": "willow-leaf"}, tones={"trunk": 0.28, "limb": 0.33, "sun": 0.62})

STEM = [(0, 0, 0, 0.56), (0, 0.12, 0, 0.49), (0.01, 0.4, 0, 0.43), (0.04, 0.9, 0, 0.395), (0.07, 1.3, 0.01, 0.385), (0.12, 1.8, 0.02, 0.39), (0.17, 2.2, 0.02, 0.4), (0.2, 2.6, 0.02, 0.34)]
fy = K.heights(STEM)
t.axis("stem", STEM, kind="trunk", level=0, ridges=(18, 0.014), lobes=[(5, 0.12, 25, 0.3)], ground=True, bones=[
    ("root-plate", 0, fy(0.4), 0, "Root plate and collar", "Wurzelteller und Wurzelhals", "Root collar", "Wurzelhals"),
    ("trunk-1", fy(0.4), fy(1.4), 0, "Lower trunk", "Unterer Stamm", "Trunk", "Stamm"),
    ("trunk-2", fy(1.4), 1.0, 0, "Upper trunk with the fork", "Oberer Stamm mit der Gabel", "Trunk", "Stamm"),
])
for n, (az, reach, r0) in enumerate([(10, 1.7, 0.15), (82, 1.5, 0.13), (154, 1.7, 0.15), (226, 1.5, 0.13), (298, 1.6, 0.14)], 1):
    d = K.direction(az, 0)
    t.axis(f"root-{n}", [(d[0] * 0.15, 0.5, d[2] * 0.15, r0), (d[0] * 0.62, 0.2, d[2] * 0.62, r0 * 0.8), (d[0] * reach * 0.7, 0.05, d[2] * reach * 0.7, r0 * 0.6), (d[0] * reach, 0.0, d[2] * reach, r0 * 0.42)], bone="root-plate", kind="trunk", level=2, aspect=1.5, ground=True, sides=(12, 8, 6, 6))

LIMBS = [
    ("w", 2.2, 168, 76, 10.6, 0.19, -98, 10, "West limb", "Westast"), ("nw", 2.35, 118, 77, 10.0, 0.17, -96, -12, "North-west limb", "Nordwestast"),
    ("ne", 2.3, 44, 77, 10.2, 0.175, -96, 12, "North-east limb", "Nordostast"), ("e", 2.25, -14, 76, 10.5, 0.185, -98, -10, "East limb", "Ostast"),
    ("se", 2.4, -62, 77, 9.8, 0.165, -96, 12, "South-east limb", "Südostast"), ("sw", 2.35, -136, 77, 10.0, 0.17, -96, -12, "South-west limb", "Südwestast"),
    ("c", 2.5, 30, 87, 8.8, 0.16, -80, 60, "Centre limb", "Mittelast"),
]
BOUGHS = [(0.34, -24, 62, 3.4, 0.06, -76), (0.5, 52, 44, 3.9, 0.058, -70), (0.66, -50, 30, 3.7, 0.052, -60), (0.82, 44, 12, 3.2, 0.044, -46)]
TRESSES = {"limb": [(0.78, 34, 9), (0.9, -30, 9), (1.0, 0, 9)], "bough": [(0.7, 40, 9), (1.0, -8, 9)]}
like, group_bone, tress_axes = {}, {}, []
for key, y, az, el, length, r0, up, turn, en, de in LIMBS:
    old = "w" if key in ("w", "nw", "sw", "c") else "e"
    for part in "abc":
        like[f"limb-{key}-{part}"] = f"limb-{old}-{part}"
    t.branch(key, "stem", fy(y), az, el, length, r0, r1=0.014, up=up, turn=turn, wave=0.2, bend=2.6, level=1, socket=True, bones=[
        (f"limb-{key}-a", 0, 0.34, 1, f"{en}, rising part", f"{de}, aufsteigender Teil", "Limb", "Starkast"),
        (f"limb-{key}-b", 0.34, 0.68, 2, f"{en}, arching part", f"{de}, Bogenteil", "Limb", "Starkast"),
        (f"limb-{key}-c", 0.68, 1.0, 2, f"{en}, drooping end", f"{de}, hängendes Ende", "Branch", "Ast"),
    ])
    group_bone[key] = f"limb-{key}-b"
    hosts = [(key, "limb", az, en, de)]
    for n, (at, daz, bel, blen, br0, bup) in enumerate(BOUGHS, 1):
        bid = f"{key}-b{n}"
        baz = az + (daz if key in ("w", "ne", "se", "c") else -daz)
        t.branch(bid, key, at, baz, bel, blen * length / 10, br0, r1=0.012, up=bup, turn=-daz * 0.2, level=2, scales=(50, 100, 200))
        hosts.append((bid, "bough", baz, en, de))
        t.spray(f"cap-{bid}", bid, 0.15, 0.85, 0.5, (6, 4, 0.2), over=0.2, lift=0.25, scales=(50, 100), flat=0.55, group=key)
    t.spray(f"cap-{key}", key, 0.4, 1.1, 0.6, (7, 5, 0.2), over=0.2, lift=0.3, scales=(50, 100), flat=0.55, group=key)
    t.spray(f"cap-{key}", key, 0.4, 2.3, 0.75, (7, 4, 0.14), over=0.3, lift=0.4, scales=(200,), flat=0.5, group=key)
    t.spray(f"cap-{key}", key, 0.36, 2.9, 0.95, (6, 3, 0.12), over=0.4, lift=0.45, scales=(500,), flat=0.5, group=key)
    count = 0
    for host, kind, haz, hen, hde in hosts:
        for m, (at, daz, tlen) in enumerate(TRESSES[kind], 1):
            count += 1
            cid = f"curt-{key}{count}"
            like[cid] = f"curt-{old}{1 + count % 4}"
            if kind == "bough" and host.endswith("b1"):
                continue
            start = t.axes[host]["path"].at(at)[0]
            hang = (start[1] - 1.0) / 0.86
            keep = count in (3, 7)
            t.branch(cid + "-axis", host, at, haz + daz, -30, hang, 0.0115, r1=0.0098, up=-58, bend=0.6, level=3, collar=1.0, bones=[(cid, 0, 1.0, 3, f"{hen}, hanging tress {count}", f"{hde}, Hängestrang {count}", "Tress", "Strang")])
            t.axes[cid + "-axis"]["keep"] = keep
            tress_axes.append(cid + "-axis")
            t.spray(f"tress-{cid}", cid + "-axis", 0.05, 0.22, 0.21, (4, 7, 0.24), over=0.12, scales=(50, 100), flat=1.0, peak=0.3, base=0.75, group=f"{key}-hang")
            if m == len(TRESSES[kind]):
                t.spray(f"tress-{cid}", cid + "-axis", 0.04, 0.6, 0.55, (5, 4, 0.17), over=0.2, scales=(200,), flat=1.0, peak=0.3, base=0.75, group=f"{key}-hang")
            if keep:
                t.spray(f"fall-{cid}", cid + "-axis", 0.02, 1.1, 1.0, (5, 3, 0.14), over=0.25, scales=(500,), flat=1.0, peak=0.3, base=0.8, group=f"{key}-hang")
    group_bone[f"{key}-hang"] = f"limb-{key}-c"


def curtain(view, shape, tree):
    if not shape["id"].startswith("curtain-"):
        return None
    path = tree.axes[tress_axes[(int(shape["id"].split("-")[1]) * 5) % len(tress_axes)]]["path"]
    pick = (lambda p: [round(p[0], 4), round(p[1], 4)]) if view == "elevation" else (lambda p: [round(p[2], 4), round(p[1], 4)])
    return dict({k: v for k, v in shape.items() if k != "points"}, points=[pick(path.at(f)[0]) for f in (0, 0.25, 0.5, 0.75, 1)], closed=False)


cfg = {
    "like": like,
    "leaf_density": 0.35,
    "group_bone": group_bone,
    "symbol_extra": curtain,
    "emitters": {"transpiration-curtain": {"on": ["cap-w", "cap-nw", "cap-ne", "cap-e", "cap-se", "cap-sw", "cap-c", "cap-w-b1", "cap-e-b1", "cap-ne-b1"], "diameter": 0.8, "cls": "sun", "prefix": "tp-curtain", "title": ("Transpiration of the hanging foliage", "Transpiration des Hängelaubs")}},
    "scaffolds": ["w", "ne", "e", "sw"],
    "habit": {
        50: ("the willow is built as it grows. A collar of 1.1 m with five buttress roots carries a short, deeply furrowed, slightly leaning trunk of 0.77 m at breast height that forks at 2.3 m into seven limbs. Six of them rise at about 57 degrees all round, arch over at 8 to 9 m and droop at their ends 6 to 7 m from the trunk, the seventh rises through the middle and closes the dome at 10.5 m; each gives off four arching boughs. From the outer limbs and boughs sixty-three tresses of withies hang 4 to 8 m down to about a metre above the ground, each a bone of its own. The narrow leaves follow the wood: a flat foliage cap along the arch of every limb and bough, and one slender hanging strand around every tress, with air between the strands, so the crown is a dome over a ring of separate tresses and never a closed curtain.", "Die Weide ist gebaut, wie sie wächst. Ein Wurzelhals von 1,1 m mit fünf Wurzelanläufen trägt einen kurzen, tief gefurchten, leicht geneigten Stamm von 0,77 m in Brusthöhe, der sich in 2,3 m in sieben Starkäste gabelt. Sechs davon steigen rundum mit etwa 57 Grad, bogen in 8 bis 9 m über und hängen an ihren Enden 6 bis 7 m vom Stamm entfernt, der siebte steigt durch die Mitte und schließt die Kuppel in 10,5 m; jeder trägt vier überbogene Äste. Von den äußeren Starkästen und Ästen hängen dreiundsechzig Stränge aus Ruten 4 bis 8 m herab bis etwa einen Meter über den Boden, jeder ein eigener Knochen. Die schmalen Blätter folgen dem Holz: eine flache Laubkappe entlang des Bogens jedes Starkastes und Astes und ein schlanker hängender Strang um jeden Rutenstrang, mit Luft zwischen den Strängen, die Krone ist so eine Kuppel über einem Ring getrennter Stränge und nie ein geschlossener Vorhang."),
        100: ("the same trunk, seven limbs, their boughs and sixty-three tresses; the withies are kept at their true thickness as slender solids and are not drawn, every tress keeps its hanging strand and every limb and bough its cap.", "Derselbe Stamm, sieben Starkäste, ihre Äste und dreiundsechzig Stränge; die Ruten bleiben in wahrer Dicke als schlanke Körper erhalten und werden nicht gezeichnet, jeder Strang behält seinen Laubstrang und jeder Starkast und Ast seine Kappe."),
        200: ("trunk, limbs and boughs remain as wood; each limb system is one foliage cap along its arch and four hanging falls, one for every group of tresses.", "Stamm, Starkäste und Äste bleiben als Holz; jedes Astsystem ist eine Laubkappe entlang seines Bogens und vier hängende Fälle, einer für jede Stranggruppe."),
        500: ("trunk and limbs, none thinner than 240 mm, one cap per limb and two broad hanging falls per limb system with gaps between them.", "Stamm und Starkäste, keiner dünner als 240 mm, je Starkast eine Kappe und zwei breite hängende Fälle je Astsystem mit Lücken dazwischen."),
    },
    "drawing": {
        50: ("furrowed trunk, seven arching limbs with boughs and withies, the foliage caps of every limb system as one outline and its hanging strands as another with their gaps.", "gefurchter Stamm, sieben überbogene Starkäste mit Ästen und Ruten, die Laubkappen jedes Astsystems als ein Umriss und seine hängenden Stränge als ein weiterer mit ihren Lücken."),
        100: ("trunk, limbs and boughs, caps and hanging strands per limb system.", "Stamm, Starkäste und Äste, Kappen und hängende Stränge je Astsystem."),
        200: ("trunk, limbs, one cap and four hanging falls per limb system.", "Stamm, Starkäste, eine Kappe und vier hängende Fälle je Astsystem."),
        500: ("trunk, limb stubs, caps and hanging falls.", "Stamm, Astansätze, Kappen und hängende Fälle."),
    },
}
A.assemble(t, cfg)
