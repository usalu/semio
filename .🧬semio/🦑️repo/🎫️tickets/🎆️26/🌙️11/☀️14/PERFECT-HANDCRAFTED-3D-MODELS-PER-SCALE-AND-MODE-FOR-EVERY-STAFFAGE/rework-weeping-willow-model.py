"""🌳 Weeping willow (Salix x sepulcralis 'Chrysocoma', about 40 years): the hand numbers of its author (begun by author 12 in `rework-12-weeping-willow-model.py`). World x ahead (east), y up, z left (north).

The chisel is `rework-12-kit.py`, the carrier into the asset document `rework-12-assemble.py`; the round-3 catalog asset the carrier starts from is read from `K.CATALOG`.

@see ./rework/assets/vegetation--weeping-willow.md
"""
import json
import math
import os
from importlib.machinery import SourceFileLoader

A = SourceFileLoader("rework12assemble", os.path.join(os.path.dirname(os.path.abspath(__file__)), "rework-12-assemble.py")).load_module()
K = A.K
G = os.path.join(os.path.dirname(os.path.abspath(__file__)), "🗑️generated", "rework-weeping-willow")
K.OUT = G
if os.path.exists(os.path.join(G, "base", "weeping-willow", "🔣️.json")):
    K.CATALOG = os.path.join(G, "base")


def solve(self):
    """🦴 The rest frames of the bones as the carrier's own `Tree.solve` finds them, and for a bone that states a `hinge` (a world axis) the `twistDeg` that lays its joint axis into that direction: a hanging tress swings with the wind about the lateral world axis whatever side of the crown it hangs on (`T/rig-lateral-rest.md`: rest = Rw(angleDeg) Rv(abductionDeg) Ru(twistDeg))."""
    base = {"o": (0.0, 0.0, 0.0), "u": (1.0, 0.0, 0.0), "v": (0.0, 1.0, 0.0), "w": (0.0, 0.0, 1.0)}
    rig = []
    for bid, b in self.bones.items():
        parent = self.frames[b["parent"]] if b["parent"] else base
        rel = K.sub(b["a"], parent["o"])
        x, y, z = (round(K.dot(rel, parent[k]), 4) for k in "uvw")
        d = K.unit(K.sub(b["b"], b["a"]))
        du, dv, dw = (K.dot(d, parent[k]) for k in "uvw")
        phi = round(math.degrees(math.asin(max(-1.0, min(1.0, dw)))), 3)
        theta = round(math.degrees(math.atan2(dv, du)), 3)
        ts, tc, ps, pc = math.sin(math.radians(theta)), math.cos(math.radians(theta)), math.sin(math.radians(phi)), math.cos(math.radians(phi))
        u1 = K.add(K.mul(parent["u"], tc), K.mul(parent["v"], ts))
        u, v, w = K.add(K.mul(u1, pc), K.mul(parent["w"], ps)), K.add(K.mul(parent["u"], -ts), K.mul(parent["v"], tc)), K.add(K.mul(u1, -ps), K.mul(parent["w"], pc))
        twist = 0.0
        if b.get("hinge"):
            twist = round(math.degrees(math.atan2(K.dot(b["hinge"], v), K.dot(b["hinge"], w))), 3)
            c, n = math.cos(math.radians(twist)), math.sin(math.radians(twist))
            v, w = K.sub(K.mul(v, c), K.mul(w, n)), K.add(K.mul(w, c), K.mul(v, n))
        o = K.add(parent["o"], K.add(K.add(K.mul(parent["u"], x), K.mul(parent["v"], y)), K.mul(parent["w"], z)))
        self.frames[bid] = {"o": o, "u": u, "v": v, "w": w}
        b.update(x=x, y=y, z=z, angleDeg=theta, abductionDeg=phi, twistDeg=twist, lengthM=round(K.norm(K.sub(b["b"], b["a"])), 4))
        rig.append(b)
    return rig


K.Tree.solve = solve
K.RING_STEP = {50: 0.22, 100: 0.3, 200: 0.4, 500: 0.5}
kit_build, kit_blob = K.Tree.build, K.blob
NOW = {"scale": 50}


def build(self, scale, around_leaf=None, relief_gain=None, box=None):
    """🏗️ The kit's build with the willow's statement of detail per scale: strands and caps of 0.3 to 2 m are far above the smallest feature down to 1:200, so 1:50, 1:100 and 1:200 keep the full relief and only 1:500 is smoothed."""
    NOW["scale"] = scale
    return kit_build(self, scale, around_leaf, {50: 1.0, 100: 1.0, 200: 1.0, 500: 0.6}[scale], box)


def blob(samples, around, relief, flat=0.62, shape=None, phase=0.0, fine=0.0):
    """🍃 The kit's foliage loft with as many sides as its relief needs (one for every 0.3 m of girth, 16 to 36) at 1:50, 1:100 and 1:200 alike, so the lobes of a coarser scale are the lobes of the finer one."""
    if NOW["scale"] < 500:
        around, fine = max(16, min(36, int(round(6.2832 * max(v[0] for _, v, _ in samples) / 0.3)))), 0.4
    return kit_blob(samples, around, relief, flat, shape, phase, fine)


K.Tree.build, K.blob = build, blob

t = K.Tree("weeping-willow", bark="willow-bark", wood="willow-wood", wood_density=780, regions={"trunk": "trunk-bark", "limb": "limb-bark", "sun": "leaf-curtain"}, leaf_materials={"sun": "willow-leaf"}, tones={"trunk": 0.28, "limb": 0.33, "sun": 0.62})

STEM = [(0, 0, 0, 0.56), (0, 0.12, 0, 0.49), (0.01, 0.4, 0, 0.43), (0.04, 0.9, 0, 0.395), (0.07, 1.3, 0, 0.385), (0.12, 1.8, 0, 0.39), (0.17, 2.2, 0, 0.4), (0.2, 2.6, 0, 0.34)]
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
    ("w", 2.2, 168, 73, 10.6, 0.19, -110, 10, "West limb", "Westast"), ("nw", 2.35, 118, 77, 10.0, 0.17, -108, -12, "North-west limb", "Nordwestast"),
    ("ne", 2.3, 44, 77, 10.2, 0.175, -108, 12, "North-east limb", "Nordostast"), ("e", 2.25, -14, 73, 10.5, 0.185, -110, -10, "East limb", "Ostast"),
    ("se", 2.4, -62, 77, 9.8, 0.165, -108, 12, "South-east limb", "Südostast"), ("sw", 2.35, -136, 77, 10.0, 0.17, -108, -12, "South-west limb", "Südwestast"),
    ("c", 2.5, 30, 87, 9.7, 0.16, -80, 60, "Centre limb", "Mittelast"),
]
BOUGHS = [(0.34, -24, 62, 3.4, 0.06, -76), (0.5, 52, 44, 3.9, 0.058, -70), (0.66, -50, 30, 3.7, 0.052, -60), (0.82, 44, 12, 3.2, 0.044, -46)]
TRESSES = {"limb": [(0.78, 34, 9), (0.9, -30, 9), (1.0, 0, 9)], "bough": [(0.7, 40, 9), (1.0, -8, 9)]}
like, group_bone, tress_axes = {}, {}, []
for key, y, az, el, length, r0, up, turn, en, de in LIMBS:
    old = "w" if key in ("w", "nw", "sw", "c") else "e"
    for part in "abc":
        like[f"limb-{key}-{part}"] = None
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
        t.branch(bid, key, at, baz, bel, blen * length / 10, br0, r1=0.012, up=bup, turn=-daz * 0.2, level=2)
        t.axes[bid]["slim"] = True
        hosts.append((bid, "bough", baz, en, de))
        t.spray(f"cap-{bid}", bid, 0.15, 0.85, 0.5, (6, 4, 0.2), over=0.5, lift=0.25, scales=(50, 100, 200), flat=0.55, group=f"cap-{bid}")
        group_bone[f"cap-{bid}"] = t.bone_at(key, at)
    t.spray(f"cap-{key}", key, 0.4, 1.1, 1.0 if key == "c" else 0.78, (7, 5, 0.2), over=0.6, lift=0.3, drop=0.35, scales=(50, 100, 200), flat=0.6, group=f"cap-{key}")
    group_bone[f"cap-{key}"] = t.bone_at(key, 0.7)
    t.spray(f"cap-{key}", key, 0.4, 2.3, 0.75, (7, 4, 0.14), over=0.3, lift=0.4, scales=(500,), flat=0.5, group=key)
    count = 0
    for host, kind, haz, hen, hde in hosts:
        for m, (at, daz, tlen) in enumerate(TRESSES[kind], 1):
            count += 1
            cid = f"curt-{key}{count}"
            like[cid] = f"curt-{old}{1 + count % 4}"
            if kind == "bough" and host.endswith("b1"):
                continue
            start = t.axes[host]["path"].at(at)[0]
            hang = (start[1] - 0.9) / 0.965
            keep = False
            t.branch(cid + "-axis", host, at, haz + daz, -58, hang, 0.0115, r1=0.0098, up=-31, bend=0.5, level=3, collar=1.0, bones=[(cid, 0, 1.0, 3, f"{hen}, hanging tress {count}", f"{hde}, Hängestrang {count}", "Tress", "Strang")])
            t.axes[cid + "-axis"]["keep"] = keep
            hinge = t.bones[cid]
            hinge.update(b=(hinge["b"][0], hinge["b"][1], hinge["a"][2]), hinge=(0.0, 0.0, 1.0))
            group_bone[cid] = cid
            tip = t.axes[cid + "-axis"]["path"].at(1.0)[0]
            print(f"  tress {cid} on {host} from y {start[1]:.2f} hangs {hang:.2f} m to x {tip[0]:.2f} z {tip[2]:.2f}{' kept at 1:500' if keep else ''}")
            tress_axes.append(cid + "-axis")
            t.spray(f"tress-{cid}", cid + "-axis", 0.0, 0.31, 0.17, (5, 8, 0.24), over=0.12, flat=1.0, peak=0.34, base=0.7, group=cid, scales=(50, 100, 200, 500))


def curtain(view, shape, tree):
    if not shape["id"].startswith("curtain-"):
        return None
    path = tree.axes[tress_axes[(int(shape["id"].split("-")[1]) * 5) % len(tress_axes)]]["path"]
    pick = (lambda p: [round(p[0], 4), round(p[1], 4)]) if view == "elevation" else (lambda p: [round(p[2], 4), round(p[1], 4)])
    return dict({k: v for k, v in shape.items() if k != "points"}, points=[pick(path.at(f)[0]) for f in (0, 0.25, 0.5, 0.75, 1)], closed=False)


cfg = {
    "like": like,
    "leaf_density": 0.35,
    "area_trim": {"trunk-bark": 0.735},
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


def settle(folder):
    """📐 What the carrier leaves to the willow: the `twistDeg` of the tress bones enters the rig."""
    pre = os.path.join(folder, "pre.json")
    doc = json.load(open(pre, encoding="utf-8"))
    for bone in doc["asset"]["rig"]["bones"]:
        twist = t.bones[bone["id"]].get("twistDeg", 0.0)
        if abs(twist) > 1e-9:
            entry = {}
            for k, v in bone.items():
                entry[k] = v
                if k == ("abductionDeg" if "abductionDeg" in bone else "angleDeg"):
                    entry["twistDeg"] = twist
            bone.clear()
            bone.update(entry)
    K.write(pre, doc)


settle(os.path.join(G, "weeping-willow"))
