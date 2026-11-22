"""🍇️ grapevine-row: Vitis vinifera, four cordon-trained vines on a vertical-shoot-positioned (VSP) wire trellis between two timber posts, 4.0 m x 1.8 m x 0.56 m.

Run: `python3 grapevine.py` writes `🗂️catalog/🌳️vegetation/grapevine-row/🔣️.json` and the manifest row. Every number below is designed here per scale.
@see ./pw16lib.py
@see ./design-grapevine.md
"""
import math
from pw16lib import *  # noqa: F401,F403

ID = "grapevine-row"
VINES = [-1.5, -0.5, 0.5, 1.5]
POST_X, POST_D, POST_H = 1.95, 0.10, 1.80
CORDON_Y, CORDON_D, TRUNK_D = 0.82, 0.045, 0.08
WIRES = [("fruit", 0.80, 0.0), ("catch-1l", 1.10, 0.14), ("catch-1r", 1.10, -0.14), ("catch-2l", 1.45, 0.14), ("catch-2r", 1.45, -0.14), ("top", 1.75, 0.0)]
WIRE_D = 0.0025
BUNCH_DX = [(-0.28, 0.05), (0.22, -0.05)]
REACH, DEPTH, TOP = 1.97, 0.56, 1.80
CORDON = [(-0.47, 0.51), (-0.51, 0.51), (-0.51, 0.51), (-0.51, 0.47)]

rig = Rig()
rig.add("root-plate", None, T("Row line on the ground", "Zeilenlinie am Boden"), 0, 0, 0, 0, FIXED)
post_section = {"areaM2": r5(math.pi * POST_D ** 2 / 4), "secondMomentM4": round(math.pi * POST_D ** 4 / 64, 12), "extremeFibreM": POST_D / 2, "material": "pine-post", "title": T("Round pine post, 100 mm", "Kiefern-Rundpfahl, 100 mm")}
post_kg = math.pi * POST_D ** 2 / 4 * POST_H * 560
for side, x in (("l", -POST_X), ("r", POST_X)):
    rig.add(f"post-{side}", "root-plate", T(f"End post {'left' if side == 'l' else 'right'}", f"Endpfahl {'links' if side == 'l' else 'rechts'}"), x, 0, 90, POST_H, FIXED, mass=(post_kg, "authored-estimate", "wood-handbook"), section=post_section)
rig.add_world("wires", "post-l", T("Trellis wires (fruiting, two catch-wire pairs, top)", "Spalierdrähte (Biege-, zwei Heftdrahtpaare, Kopfdraht)"), [-POST_X, 0.80, 0], 0, 2 * POST_X, FIXED, mass=(6 * 2 * POST_X * math.pi * WIRE_D ** 2 / 4 * 7850, "literature", "en-10025-2", (POST_X, 0.4)))
trunk_kg = math.pi * TRUNK_D ** 2 / 4 * 0.80 * 950
cordon_kg = math.pi * CORDON_D ** 2 / 4 * 1.0 * 950
for i, x in enumerate(VINES, 1):
    rig.add(f"trunk-{i}", "root-plate", T(f"Trunk of vine {i}", f"Stamm von Rebstock {i}"), x, 0, 90, CORDON_Y, revolute(-2, 2), mass=(trunk_kg, "authored-estimate", "keller-2015"))
    a, b = CORDON[i - 1]
    rig.add_world(f"cordon-{i}", f"trunk-{i}", T(f"Bilateral cordon of vine {i}", f"Zweiarmiger Kordon von Rebstock {i}"), [x + a, CORDON_Y, 0], 0, b - a, revolute(-2, 2), mass=(cordon_kg, "authored-estimate", "keller-2015"))
    rig.add_world(f"canopy-{i}", f"cordon-{i}", T(f"Shoot canopy of vine {i}", f"Laubwand von Rebstock {i}"), [x, 0.85, 0], 90, 0.95, revolute(-15, 15), mass=(2.4, "authored-estimate", "keller-2015"))
    for k, (dx, dz) in enumerate(BUNCH_DX):
        rig.add_world(f"bunch-{i}{'ab'[k]}", f"cordon-{i}", T(f"Grape bunch {i}{'ab'[k]}", f"Traube {i}{'ab'[k]}"), [x + dx, CORDON_Y, 0], -90, 0.23, revolute(-35, 35), mass=(0.3, "authored-estimate", "keller-2015", (0.11, 0)))

LEAF, WOOD, FRUIT, POST, WIRE = "vine-leaf", "vine-wood", "grape-berry", "pine-post", "trellis-wire"


def lobe(sc, id, vine, dx, y, z, rx, ry, rz, tone, region="leaves", rot=0):
    bone = f"canopy-{vine}"
    x = VINES[vine - 1] + dx
    x = max(-REACH + rx, min(REACH - rx, x))
    rz = min(rz, DEPTH / 2 - abs(z))
    ry = min(ry, TOP - y)
    c = rig.local(bone, [x, y, z])
    sc.ell(id, bone, c, (ry, rx, rz), rot=rot, region=region, material=LEAF, tone=tone)


def trellis(sc, wires=True):
    for side in "lr":
        sc.rod(f"post-{side}", f"post-{side}", [0, 0, 0], [POST_H, 0, 0], POST_D, region="posts", material=POST, tone=0.55)
    arm = {50: 0.012, 100: 0.014, 200: 0.016, 500: 0.016}[sc.d]
    for side in "lr":
        for h in (1.10, 1.45):
            sc.rod(f"arm-{side}-{int(h * 100)}", f"post-{side}", [h, 0, -0.165], [h, 0, 0.165], arm, region="wires", material=WIRE, tone=0.25, role="detail")
    if wires:
        for name, y, z in WIRES:
            sc.rod(f"wire-{name}", "wires", [0, y - 0.80, z], [2 * POST_X, y - 0.80, z], WIRE_D, region="wires", material=WIRE, tone=0.2, role="detail", filled=False, views=() if os.environ.get("NOWIRE") else ("elevation", "front", "plan"))


def vine_wood(sc, i, split, head):
    t = f"trunk-{i}"
    if split:
        sc.rod(f"trunk-{i}-low", t, [0, 0, 0], [0.42, 0, 0], TRUNK_D, region="vine-wood", material=WOOD, tone=0.25)
        sc.rod(f"trunk-{i}-high", t, [0.40, 0, 0], [CORDON_Y, 0, 0], TRUNK_D * 0.85, region="vine-wood", material=WOOD, tone=0.27)
    else:
        sc.rod(f"trunk-{i}", t, [0, 0, 0], [CORDON_Y, 0, 0], TRUNK_D, region="vine-wood", material=WOOD, tone=0.25)
    if head:
        sc.ell(f"head-{i}", t, [CORDON_Y, 0, 0], head, region="vine-wood", material=WOOD, tone=0.22)
    c = f"cordon-{i}"
    a, b = CORDON[i - 1]
    if split:
        sc.rod(f"cordon-{i}-l", c, [0, 0, 0], [-a, 0, 0], CORDON_D * 0.9, region="vine-wood", material=WOOD, tone=0.3)
        sc.rod(f"cordon-{i}-r", c, [-a, 0, 0], [b - a, 0, 0], CORDON_D, region="vine-wood", material=WOOD, tone=0.3)
    else:
        sc.rod(f"cordon-{i}", c, [0, 0, 0], [b - a, 0, 0], CORDON_D, region="vine-wood", material=WOOD, tone=0.3)


def bunches(sc, parts):
    for i in range(1, 5):
        for k, (dx, dz) in enumerate(BUNCH_DX):
            b = f"bunch-{i}{'ab'[k]}"
            for j, (u, v, ru, rv, rw, tone) in enumerate(parts):
                sc.ell(f"{b}-{j}", b, [u, v, dz], (ru, rv, rw), region="bunches", material=FRUIT, tone=tone)


def bands(sc, ry, rz, tone=0.36):
    """The lowest course of leaves on the shoot bases, a continuous band that rests on the cordon line from post to post."""
    for i in range(1, 5):
        lobe(sc, f"canopy-{i}-band", i, 0.0, 0.86, 0.0, 0.6, ry, 0.13, tone)


def joins(sc, rx, ry, y, rz):
    """Junction lobes where the walls of two neighbouring vines interlock (x = -1, 0, 1), so the wall has no pockets between lobes."""
    for i in range(1, 4):
        lobe(sc, f"canopy-{i}-join", i, 0.5, y, 0.0, rx, ry, min(rz, 0.16), 0.34 + 0.02 * i)


def edge_vines(sc, ry, rz, y=1.25):
    """Outer lobes of the end vines reach into the end posts (x = -1.97 and 1.97), so canopy and trellis are one body."""
    lobe(sc, "canopy-1-end", 1, -0.32, y, 0.0, 0.15, ry, rz, 0.44)
    lobe(sc, "canopy-4-end", 4, 0.32, y, 0.0, 0.15, ry, rz, 0.44)


def scale50():
    sc = Scale(rig, 50)
    trellis(sc)
    for i in range(1, 5):
        vine_wood(sc, i, True, (0.055, 0.06, 0.05))
        lobe(sc, f"canopy-{i}-core", i, 0.0, 1.25, 0.0, 0.42, 0.42, 0.28, 0.42)
        lobe(sc, f"canopy-{i}-low-a", i, -0.26, 1.04, 0.05, 0.30, 0.21, 0.24, 0.38)
        lobe(sc, f"canopy-{i}-low-b", i, 0.24, 1.00, -0.04, 0.31, 0.19, 0.25, 0.46)
        lobe(sc, f"canopy-{i}-mid-a", i, -0.22, 1.36, -0.05, 0.31, 0.24, 0.27, 0.48)
        lobe(sc, f"canopy-{i}-mid-b", i, 0.26, 1.33, 0.06, 0.30, 0.25, 0.26, 0.36)
        lobe(sc, f"canopy-{i}-top", i, 0.0, 1.60, 0.0, 0.44, 0.20, 0.23, 0.5)
        for j, (dx, y, rx, ry) in enumerate([(-0.36, 1.70, 0.10, 0.08), (-0.12, 1.72, 0.11, 0.08), (0.13, 1.71, 0.10, 0.09), (0.37, 1.69, 0.10, 0.08)]):
            lobe(sc, f"canopy-{i}-crest-{j}", i, dx, y, 0.0, rx, ry, 0.13, 0.52 - 0.03 * j)
        for j, (dx, z) in enumerate([(-0.33, 0.12), (0.0, -0.1), (0.31, 0.08)]):
            lobe(sc, f"canopy-{i}-skirt-{j}", i, dx, 0.89, z, 0.13, 0.07, 0.14, 0.34)
        for j, (u, v) in enumerate([(0.18, 0.22), (0.30, -0.18), (0.52, 0.28), (0.55, -0.05), (0.70, 0.18), (0.40, 0.02)]):
            sc.stroke(f"canopy-{i}-vein-{j}", f"canopy-{i}", [(u - 0.05, v + 0.05), (u, v), (u - 0.05, v - 0.05)], role="texture")
            sc.stroke(f"canopy-{i}-midrib-{j}", f"canopy-{i}", [(u - 0.07, v), (u + 0.02, v)], role="texture")
    edge_vines(sc, 0.36, 0.22)
    joins(sc, 0.16, 0.44, 1.27, 0.24)
    bands(sc, 0.1, 0.2)
    bunches(sc, [(0.06, 0.0, 0.06, 0.065, 0.06, 0.16), (0.13, 0.0, 0.075, 0.05, 0.05, 0.13), (0.195, 0.005, 0.035, 0.03, 0.03, 0.12)])
    return sc


def scale100():
    sc = Scale(rig, 100)
    trellis(sc)
    for i in range(1, 5):
        vine_wood(sc, i, False, (0.05, 0.058, 0.05))
        lobe(sc, f"canopy-{i}-core", i, 0.0, 1.26, 0.0, 0.43, 0.41, 0.28, 0.42)
        lobe(sc, f"canopy-{i}-low-a", i, -0.25, 1.03, 0.04, 0.31, 0.21, 0.25, 0.38)
        lobe(sc, f"canopy-{i}-low-b", i, 0.25, 1.01, -0.03, 0.31, 0.2, 0.25, 0.46)
        lobe(sc, f"canopy-{i}-mid", i, 0.0, 1.38, 0.0, 0.5, 0.25, 0.27, 0.44)
        lobe(sc, f"canopy-{i}-top", i, 0.0, 1.61, 0.0, 0.46, 0.19, 0.23, 0.5)
        for j, dx in enumerate([-0.25, 0.0, 0.25]):
            lobe(sc, f"canopy-{i}-crest-{j}", i, dx, 1.71, 0.0, 0.13, 0.09, 0.14, 0.52 - 0.04 * j)
    edge_vines(sc, 0.35, 0.22)
    joins(sc, 0.17, 0.44, 1.28, 0.25)
    bands(sc, 0.092, 0.2)
    bunches(sc, [(0.075, 0.0, 0.075, 0.062, 0.058, 0.15), (0.165, 0.003, 0.06, 0.04, 0.04, 0.12)])
    return sc


def scale200():
    sc = Scale(rig, 200)
    trellis(sc)
    for i in range(1, 5):
        vine_wood(sc, i, False, None)
        lobe(sc, f"canopy-{i}-low", i, 0.0, 1.06, 0.0, 0.54, 0.25, 0.27, 0.4)
        lobe(sc, f"canopy-{i}-mid", i, 0.0, 1.37, 0.0, 0.54, 0.26, 0.28, 0.45)
        lobe(sc, f"canopy-{i}-top", i, 0.0, 1.62, 0.0, 0.5, 0.18, 0.24, 0.5)
    edge_vines(sc, 0.34, 0.22)
    joins(sc, 0.2, 0.46, 1.3, 0.26)
    bands(sc, 0.095, 0.2)
    bunches(sc, [(0.1, 0.0, 0.1, 0.06, 0.055, 0.14)])
    return sc


def scale500():
    sc = Scale(rig, 500)
    trellis(sc)
    for i in range(1, 5):
        vine_wood(sc, i, False, None)
        lobe(sc, f"canopy-{i}", i, 0.0, 1.30, 0.0, 0.56, 0.50, 0.28, 0.42)
    edge_vines(sc, 0.32, 0.2, y=1.28)
    joins(sc, 0.22, 0.45, 1.3, 0.26)
    bands(sc, 0.1, 0.2)
    bunches(sc, [(0.1, 0.0, 0.1, 0.05, 0.045, 0.14)])
    return sc


def outlets(sc):
    r, u = {50: (0.03, 0.42), 100: (0.035, 0.44), 200: (0.04, 0.46), 500: (0.045, 0.48)}[sc.d]
    return [{"id": "outlet-transpiration-leaves", "kind": "ellipsoid", "bone": "canopy-2", "material": LEAF, "role": "detail", "tone": 0.5, "radii": [r, r, r], "transform": {"translation": [u, 0, 0], "rotation": [0, 0, 0, 1]}, "region": "leaves", "tags": ["transpiration-leaves"]}]


SUPPORTS = [("base-support-l", "post-l"), ("base-support-r", "post-r")] + [(f"root-{i}", f"trunk-{i}") for i in range(1, 5)]


def supports(sc):
    a = {50: 0.06, 100: 0.07, 200: 0.075, 500: 0.08}[sc.d]
    return [{"id": f"support-{aid}", "kind": "box", "bone": bone, "material": POST if bone.startswith("post") else WOOD, "role": "body", "tone": 0.6, "size": [a, a, a], "transform": {"translation": [a / 2 - 0.02, 0, 0]}, "member": bone, "part": "support", "anchor": aid} for aid, bone in SUPPORTS]


SCALE_BUILD = {50: scale50, 100: scale100, 200: scale200, 500: scale500}
PENS = {50: pens(0.25, 0.18, 0.1, None, 0.35), 100: pens(0.25, 0.13, None, None, 0.35), 200: pens(0.18, 0.13, None, None, 0.25), 500: pens(0.18, 0.1, None, None, 0.25)}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}

DEC = {
    50: decisions(
        T("At 1:50 (1 m is 20 mm) the row is 80 mm long and 36 mm high: two round posts, six wires, four vines with their trunk heads, bilateral cordons and a leaf wall of six overlapping lobes per vine with a crest of four leaf clusters and a skirt of three, eight bunches below the cordon.", "Bei 1:50 (1 m sind 20 mm) ist die Zeile 80 mm lang und 36 mm hoch: zwei Rundpfähle, sechs Drähte, vier Rebstöcke mit Stammkopf, zweiarmigem Kordon und einer Laubwand aus sechs überlappenden Lappen je Stock mit einem Kamm aus vier Blattbüscheln und einem Saum aus drei, acht Trauben unter dem Kordon."),
        T("Retained: trunk taper, the knotted head, both cordon arms, every wire at its height and side, bunch shoulders and tips, the ragged hedged crest at 1.8 m.", "Beibehalten: Stammverjüngung, der knorrige Kopf, beide Kordonarme, jeder Draht in seiner Höhe und Seite, Traubenschultern und -spitzen, der gestutzte, ausgefranste Kamm bei 1,8 m."),
        T("Merged: shoots and leaves into lobes of the leaf wall; berries into three ellipsoids per bunch.", "Zusammengefasst: Triebe und Blätter zu Lappen der Laubwand; Beeren zu drei Ellipsoiden je Traube."),
        T("Omitted: tendrils, single berries, staples and wire tensioners.", "Weggelassen: Ranken, einzelne Beeren, Krampen und Drahtspanner."),
        T("Wires (2.5 mm, 0.05 mm on paper) are detail strokes at their true height, hung from steel cross-arms on the posts; the post is a true 100 mm member.", "Drähte (2,5 mm, 0,05 mm auf Papier) sind Detailstriche in wahrer Höhe, an Stahlauslegern der Pfähle aufgehängt; der Pfahl ist ein wahrer Stab von 100 mm."),
        T("None; palmate leaf marks are texture strokes inside the wall.", "Keine; handförmige Blattzeichen sind Texturstriche in der Wand."),
        T("Leaf lobes in four greys so the wall reads as depth; dark bunches; light posts.", "Blattlappen in vier Grautönen, damit die Wand räumlich wirkt; dunkle Trauben; helle Pfähle."),
        T("Lobes carry the leaf region, trunks and cordons vine wood, bunches the berry region, posts and wires their own regions.", "Lappen tragen die Blattregion, Stämme und Kordons das Rebholz, Trauben die Beerenregion, Pfähle und Drähte ihre eigenen Regionen.")),
    100: decisions(
        T("At 1:100 (1 m is 10 mm) the row is 40 mm by 18 mm: posts, wires, four trunks with heads and single cordons, five lobes per vine with a crest of three clusters, eight two-part bunches.", "Bei 1:100 (1 m sind 10 mm) misst die Zeile 40 mm mal 18 mm: Pfähle, Drähte, vier Stämme mit Kopf und einfachem Kordon, fünf Lappen je Stock mit einem Kamm aus drei Büscheln, acht zweiteilige Trauben."),
        T("Retained: the hedged top, the open fruit zone with its bunches, the trunk heads.", "Beibehalten: die gestutzte Oberkante, die offene Traubenzone mit ihren Trauben, die Stammköpfe."),
        T("Merged: the two middle lobes into one, the skirt into the lower lobes, the trunk into one stem.", "Zusammengefasst: die beiden mittleren Lappen zu einem, der Saum in die unteren Lappen, der Stamm zu einem Schaft."),
        T("Omitted: texture strokes, the bunch tips.", "Weggelassen: Texturstriche, die Traubenspitzen."),
        T("Wires stay detail strokes; cordons are true 45 mm members.", "Drähte bleiben Detailstriche; Kordons sind wahre Stäbe von 45 mm."),
        T("None.", "Keine."),
        T("Three greys in the wall, dark bunches.", "Drei Grautöne in der Wand, dunkle Trauben."),
        T("As at 1:50: every part carries its region.", "Wie bei 1:50: jedes Teil trägt seine Region.")),
    200: decisions(
        T("At 1:200 (1 m is 5 mm) the row is a 20 mm by 9 mm band: three stacked lobes per vine form the leaf wall, the trunks, cordons and posts are members, one bunch per bone.", "Bei 1:200 (1 m sind 5 mm) ist die Zeile ein Band von 20 mm mal 9 mm: drei gestapelte Lappen je Stock bilden die Laubwand, Stämme, Kordons und Pfähle sind Stäbe, eine Traube je Knochen."),
        T("Retained: the open fruit zone below the wall, the bunch rhythm, the posts.", "Beibehalten: die offene Traubenzone unter der Wand, der Traubenrhythmus, die Pfähle."),
        T("Merged: all lobes of a vine into three horizontal tiers; the trunk head into the trunk.", "Zusammengefasst: alle Lappen eines Stocks zu drei liegenden Lagen; der Stammkopf in den Stamm."),
        T("Omitted: crest clusters, bunch shoulders.", "Weggelassen: Kammbüschel, Traubenschultern."),
        T("Wires remain as fine strokes inside the wall; they show only between wall and posts.", "Drähte bleiben feine Striche in der Wand; sichtbar sind sie nur zwischen Wand und Pfählen."),
        T("None.", "Keine."),
        T("Three greys per tier.", "Drei Grautöne je Lage."),
        T("Tiers carry the leaf region; wood, berries, posts and wires their own.", "Lagen tragen die Blattregion; Holz, Beeren, Pfähle und Drähte ihre eigenen.")),
    500: decisions(
        T("At 1:500 (1 m is 2 mm) the row is an 8 mm by 3.6 mm hedge: one lobe per vine merges into a continuous leaf wall between the two posts, over four trunks and a row of bunches.", "Bei 1:500 (1 m sind 2 mm) ist die Zeile eine Hecke von 8 mm mal 3,6 mm: ein Lappen je Stock verschmilzt zu einer durchgehenden Laubwand zwischen den beiden Pfählen, über vier Stämmen und einer Reihe von Trauben."),
        T("Retained: true length, height and depth, the trunk legs below the wall, the posts.", "Beibehalten: wahre Länge, Höhe und Tiefe, die Stammbeine unter der Wand, die Pfähle."),
        T("Merged: the whole canopy of a vine into one lobe.", "Zusammengefasst: die ganze Laubwand eines Stocks zu einem Lappen."),
        T("Omitted: tiers, crest, bunch shape.", "Weggelassen: Lagen, Kamm, Traubenform."),
        T("The 80 mm trunk prints at 0.16 mm plus pen; wires are 0.1 mm strokes hidden in the wall.", "Der Stamm von 80 mm druckt mit 0,16 mm plus Stift; Drähte sind Striche von 0,1 mm in der Wand."),
        T("None.", "Keine."),
        T("One wall grey, darker bunches.", "Ein Wandgrau, dunklere Trauben."),
        T("The wall carries the leaf region.", "Die Wand trägt die Blattregion.")),
}

VOLUME_DECISIONS = {
    50: [T("Body at 1:50: posts and trunks as true cylinders, six wires as slender 2.5 mm rods, per vine six leaf lobes, four crest and three skirt clusters, three ellipsoids per bunch; the end lobes reach into the posts so the row is one body.", "Körper bei 1:50: Pfähle und Stämme als wahre Zylinder, sechs Drähte als schlanke Stäbe von 2,5 mm, je Stock sechs Blattlappen, vier Kamm- und drei Saumbüschel, drei Ellipsoide je Traube; die Endlappen reichen in die Pfähle, so dass die Zeile ein Körper ist."),
         T("Thermal: every body solid by reference in its region, one transpiration outlet in the canopy of vine 2. Structure: one member per bone by reference, a support block at each post foot and each trunk foot.", "Thermik: jeder Körper per Verweis in seiner Region, ein Transpirationsauslass in der Laubwand von Stock 2. Tragwerk: ein Stab je Knochen per Verweis, ein Auflagerblock an jedem Pfahl- und Stammfuß.")],
    100: [T("Body at 1:100: five lobes and three crest clusters per vine, two ellipsoids per bunch, trunk and cordon as single cylinders, wires kept.", "Körper bei 1:100: fünf Lappen und drei Kammbüschel je Stock, zwei Ellipsoide je Traube, Stamm und Kordon als einzelne Zylinder, Drähte beibehalten."),
          T("Thermal and structure as at 1:50, by reference.", "Thermik und Tragwerk wie bei 1:50, per Verweis.")],
    200: [T("Minimal: the structure of 1:200 already is one member per bone and per drawn bar and one support per anchor, the least the structural views need; it keeps the solids of 1:100.", "Minimal: das Tragwerk von 1:200 ist bereits ein Stab je Knochen und je gezeichnetem Stab und ein Auflager je Anker, das Mindeste, das die Tragwerksansichten brauchen; es behält die Körper von 1:100."),
          T("Body at 1:200: three tiers per vine, one ellipsoid per bunch, no trunk head.", "Körper bei 1:200: drei Lagen je Stock, ein Ellipsoid je Traube, kein Stammkopf."),
          T("Thermal and structure by reference; supports at the smallest feature of the scale.", "Thermik und Tragwerk per Verweis; Auflager im kleinsten Merkmal des Maßstabs.")],
    500: [T("Minimal: the structure of 1:500 is one member per bone and per drawn bar and one support per anchor, as at 1:200; fewer would leave a drawn wire or bone without its member.", "Minimal: das Tragwerk von 1:500 ist ein Stab je Knochen und je gezeichnetem Stab und ein Auflager je Anker, wie bei 1:200; weniger ließe einen gezeichneten Draht oder Knochen ohne seinen Stab."),
          T("Body at 1:500: one lobe per vine (0.56 m deep), bunches of 90 mm, trunks of 80 mm, the 2.5 mm wires as slender rods.", "Körper bei 1:500: ein Lappen je Stock (0,56 m tief), Trauben von 90 mm, Stämme von 80 mm, die Drähte von 2,5 mm als schlanke Stäbe."),
          T("Thermal and structure by reference; supports of 80 mm.", "Thermik und Tragwerk per Verweis; Auflager von 80 mm.")],
}

REFS = {
    "keller-2015": "Keller, M. (2015). The Science of Grapevines: Anatomy and Physiology, 2nd ed., Academic Press, London. Cordon and VSP training, shoot and leaf area per vine, cluster mass, stomatal behaviour.",
    "smart-robinson-1991": "Smart, R. and Robinson, M. (1991). Sunlight into Wine: A Handbook for Winegrape Canopy Management. Winetitles, Adelaide. VSP canopy dimensions (wall 0.3-0.6 m thick, about 1 m tall above the fruiting wire), wire heights.",
    "jones-2014": "Jones, H. G. (2014). Plants and Microclimate, 3rd ed., Cambridge University Press. Leaf energy balance, leaf area index, boundary-layer and stomatal conductance.",
    "monteith-unsworth-2013": "Monteith, J. L. and Unsworth, M. H. (2013). Principles of Environmental Physics, 4th ed., Academic Press. Penman-Monteith leaf transpiration.",
    "en-1991-1-4": "EN 1991-1-4:2005 Eurocode 1, Actions on structures, Part 1-4: Wind actions. Dynamic pressure q = 0.5 rho v^2 with air density 1.25 kg/m3, force F = cf q A, peak velocity pressure (1 + 7 Iv) q_m.",
    "wood-handbook": "Forest Products Laboratory (2021). Wood Handbook, Wood as an Engineering Material, General Technical Report FPL-GTR-282. Green density, specific heat and conductivity of wood.",
    "en-10025-2": "EN 10025-2:2019 Hot rolled products of structural steels, non-alloy steels (density 7850 kg/m3, E 210 GPa); used for the galvanised trellis wire.",
}

MATERIALS = [
    {"id": LEAF, "title": T("Grapevine leaf", "Rebblatt"), "emissivity": 0.96, "solarAbsorptivity": 0.5, "source": "authored-estimate", "reference": "jones-2014"},
    {"id": WOOD, "title": T("Old vine wood with shaggy bark", "Altes Rebholz mit faseriger Borke"), "emissivity": 0.93, "solarAbsorptivity": 0.7, "densityKgM3": 950, "conductivityWmK": 0.2, "specificHeatJKgK": 2500, "source": "authored-estimate", "reference": "wood-handbook"},
    {"id": FRUIT, "title": T("Grape berries, bloomy skin", "Weinbeeren mit Duftfilm"), "emissivity": 0.95, "solarAbsorptivity": 0.8, "densityKgM3": 1080, "conductivityWmK": 0.5, "specificHeatJKgK": 3600, "source": "authored-estimate", "reference": "keller-2015"},
    {"id": POST, "title": T("Pressure-treated round pine post", "Kesseldruckimprägnierter Kiefernpfahl"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 560, "conductivityWmK": 0.14, "specificHeatJKgK": 1700, "elasticModulusPa": 9000000000, "strengthPa": 20000000, "source": "authored-estimate", "reference": "wood-handbook"},
    {"id": WIRE, "title": T("Galvanised trellis wire 2.5 mm", "Verzinkter Spalierdraht 2,5 mm"), "emissivity": 0.3, "solarAbsorptivity": 0.45, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210000000000, "strengthPa": 400000000, "source": "literature", "reference": "en-10025-2"},
]

REGIONS = [
    leaf_region("leaves", T("Vine leaves", "Rebblätter"), LEAF, 12.0, 0.12, "vertical-plate", 0.45, 500, 0.006, 5.4, "c3", "keller-2015"),
    passive_region("vine-wood", T("Trunks and cordons", "Stämme und Kordons"), "bark", WOOD, 1.4, 0.06, "cylinder", 0.4, 30000, "wood-handbook"),
    passive_region("bunches", T("Grape bunches", "Trauben"), "surface", FRUIT, 0.55, 0.1, "sphere", 0.2, 50000, "keller-2015"),
    passive_region("posts", T("Round pine posts", "Kiefern-Rundpfähle"), "surface", POST, 1.15, 0.1, "cylinder", 0.5, 12000, "wood-handbook"),
    passive_region("wires", T("Trellis wires", "Spalierdrähte"), "surface", WIRE, 0.235, 0.0025, "cylinder", 0.6, 2300, "en-10025-2"),
]
for r in REGIONS:
    if r["id"] == "leaves":
        r["areaBasis"] = "exchange"

ANCHORS = [anchors_support("post-l", T("Left post in the ground", "Linker Pfahl im Boden"), 0.1, 0.1, id="base-support-l"), anchors_support("post-r", T("Right post in the ground", "Rechter Pfahl im Boden"), 0.1, 0.1, id="base-support-r")]
ANCHORS += [anchors_support(f"trunk-{i}", T(f"Root crown of vine {i}", f"Wurzelhals von Rebstock {i}"), 0.08, 0.08, id=f"root-{i}") for i in range(1, 5)]
TP = []
for i in range(1, 5):
    for k, u in enumerate((0.3, 0.72)):
        aid = f"tp-{i}{'lu'[k]}"
        TP.append(aid)
        ANCHORS.append(emitter_anchor(aid, f"canopy-{i}", u, 0, T(f"Transpiration of the {'lower' if k == 0 else 'upper'} leaf wall of vine {i}", f"Transpiration der {'unteren' if k == 0 else 'oberen'} Laubwand von Rebstock {i}")))
EMITTERS = [transpiration("transpiration-leaves", T("Transpiration of the leaf wall (distributed over eight anchors)", "Transpiration der Laubwand (über acht Anker verteilt)"), TP, "leaves", 0, 0.6)]

WIND = {"canopy": (0.9, 0.5), "bunch": (1.0, 0.018)}


def loads(v, extra=None):
    out = []
    for i in range(1, 5):
        out.append(wind_load(f"wind-canopy-{i}", T(f"Wind drag on the leaf wall of vine {i}", f"Winddruck auf die Laubwand von Rebstock {i}"), f"canopy-{i}", 0.48, 0, *WIND["canopy"], v))
        for k in "ab":
            out.append(wind_load(f"wind-bunch-{i}{k}", T(f"Wind drag on bunch {i}{k}", f"Winddruck auf Traube {i}{k}"), f"bunch-{i}{k}", 0.11, 0, *WIND["bunch"], v))
    return out + (extra or [])


def binding(leaf_sun, emitter, fruit_sun=0.2):
    return bind({"leaves": (1, leaf_sun), "vine-wood": (1, 0.15), "bunches": (1, fruit_sun), "posts": (1, 0.5), "wires": (1, 0.6)}, {"transpiration-leaves": emitter})


def offsets(canopy, bunch, trunk=0.0):
    out = {}
    for i in range(1, 5):
        out[f"canopy-{i}"] = {"angleDeg": r5(canopy * (1 + 0.08 * (i - 2.5)))}
        if trunk:
            out[f"trunk-{i}"] = {"angleDeg": trunk}
        for k in "ab":
            out[f"bunch-{i}{k}"] = {"angleDeg": r5(bunch)}
    return out


def pose(id, title, desc, bones, thermal, loads_):
    return {"id": id, "title": title, "description": desc, "bones": bones, "ground": "authored", "contacts": [], "thermal": thermal, "loads": loads_}


RIPE = [{"id": f"ripening-{i}{k}", "kind": "carried", "title": T(f"Ripening gain of bunch {i}{k} (sugar and water)", f"Reifezuwachs der Traube {i}{k} (Zucker und Wasser)"), "bone": f"bunch-{i}{k}", "x": 0.11, "y": 0, "fxN": 0, "fyN": -1.96, "source": "authored-estimate", "reference": "keller-2015"} for i in range(1, 5) for k in "ab"]
POSES = [
    pose("summer-calm", T("Calm summer day", "Windstiller Sommertag"), T("Light air of 1.5 m/s along the row; the leaf wall stands upright, the green bunches hang straight below the cordon. Drag is about half a newton per vine.", "Leiser Zug von 1,5 m/s längs der Zeile; die Laubwand steht aufrecht, die grünen Trauben hängen gerade unter dem Kordon. Der Widerstand beträgt etwa ein halbes Newton je Stock."), {}, binding(0.55, 0.85), loads(1.5)),
    pose("breezy", T("Fresh breeze", "Frische Brise"), T("6 m/s at canopy height (Beaufort 4 to 5): about 10 N per leaf wall (cf 0.9 on 0.5 m2 of the wall section); the walls lean 3 degrees downwind, the bunches swing 6 degrees.", "6 m/s in Laubwandhöhe (Beaufort 4 bis 5): etwa 10 N je Laubwand (cf 0,9 auf 0,5 m2 Wandquerschnitt); die Wände neigen sich 3 Grad nach Lee, die Trauben schwingen 6 Grad."), offsets(-3, 6), binding(0.55, 1.0), loads(6)),
    pose("storm", T("Storm", "Sturm"), T("20 m/s gusts (Beaufort 8 to 9) under overcast sky: about 110 N per leaf wall; the walls lean 9 degrees, the trunks yield half a degree, the bunches swing 18 degrees; stomata partly closed.", "Böen von 20 m/s (Beaufort 8 bis 9) bei bedecktem Himmel: etwa 110 N je Laubwand; die Wände neigen sich 9 Grad, die Stämme geben einen halben Grad nach, die Trauben schwingen 18 Grad; Spaltöffnungen teilweise geschlossen."), offsets(-9, 18, -0.5), binding(0.15, 0.4, 0.05), loads(20)),
    pose("ripe-harvest", T("Ripe bunches before harvest", "Reife Trauben vor der Lese"), T("Calm late-summer day after leaf removal in the fruit zone: every bunch has gained 0.2 kg of sugar and water since veraison (carried load of 1.96 N), the bunches are sun-exposed and the older leaves transpire less.", "Windstiller Spätsommertag nach dem Entblättern der Traubenzone: jede Traube hat seit dem Farbumschlag 0,2 kg Zucker und Wasser zugelegt (getragene Last 1,96 N), die Trauben liegen in der Sonne und die älteren Blätter transpirieren weniger."), {}, binding(0.6, 0.65, 0.5), loads(1.5, RIPE)),
]


def build_clips():
    base = {k: v["angleDeg"] for k, v in offsets(-3, 6).items()}
    specs = {}
    for i in range(1, 5):
        specs[f"canopy-{i}"] = (2.2, 50 * i, 1)
        for j, k in enumerate("ab"):
            specs[f"bunch-{i}{k}"] = (4.5, 50 * i + 70 + 40 * j, 1)
    keys = sway(rig.bones, "breezy", 2.6, base, specs, n=12)
    sway_clip = {"id": "wind-sway", "title": T("Wind sway", "Windschwingung"), "description": T("A deterministic 2.6 s cycle (leaf walls on flexible shoots, estimated 0.38 Hz) about the fresh-breeze lean: the walls rock by 2 degrees with a phase that travels down the row, the bunches swing by 4.5 degrees behind them.", "Ein deterministischer Zyklus von 2,6 s (Laubwände auf biegsamen Trieben, geschätzt 0,38 Hz) um die Neigung der frischen Brise: die Wände wiegen um 2 Grad mit einer Phase, die die Zeile entlangläuft, die Trauben schwingen 4,5 Grad hinterher."),
                 "durationS": 2.6, "fps": 30, "loop": True, "rootMotion": "none", "keys": keys, "contacts": [], "phases": [{"id": "sway", "fromS": 0, "toS": 2.6, "title": T("Sway cycle", "Schwingungszyklus")}], "events": [{"t": 0, "kind": "motion", "title": T("Cycle start at the mean lean", "Zyklusbeginn in mittlerer Neigung")}], "tracks": [{"target": "emitter:transpiration-leaves", "ease": "smooth", "keys": [{"t": 0, "v": 1}, {"t": 1.3, "v": 0.92}, {"t": 2.6, "v": 1}]}]}
    profile = [(0, 0.05), (1.2, 0.45), (2.5, 2.75), (3.6, 1.4), (5.0, 0.5), (8.0, 0.05)]
    gust_keys = []
    for t, f in profile:
        lean = min(1.0, f / 2.75) * 0.7
        gust_keys.append({"t": t, "pose": "breezy", "ease": "smooth", "bones": offsets(-9 * lean, 18 * lean, -0.5 * lean)})
    tracks = [{"target": f"load:{ld['id']}", "ease": "smooth", "keys": [{"t": t, "v": f} for t, f in profile]} for ld in loads(6)]
    gust = {"id": "gust", "title": T("Gust", "Windböe"), "description": T("A single 8 s gust: the wind-load tracks rise from 5 % to 2.75 times the fresh-breeze load at 2.5 s (EN 1991-1-4 peak pressure with Iv 0.25) and decay; the leaf walls and bunches lean up to 70 % of the storm lean and spring back.", "Eine einzelne Böe über 8 s: die Windlast-Spuren steigen bei 2,5 s von 5 % auf das 2,75-fache der Last der frischen Brise (Spitzendruck nach EN 1991-1-4 mit Iv 0,25) und klingen ab; Laubwände und Trauben neigen sich bis zu 70 % der Sturmneigung und federn zurück."),
            "durationS": 8, "fps": 30, "loop": False, "rootMotion": "none", "keys": gust_keys, "contacts": [], "phases": [{"id": "rise", "fromS": 0, "toS": 2.5, "title": T("Gust front", "Böenfront")}, {"id": "peak", "fromS": 2.5, "toS": 3.6, "title": T("Peak", "Spitze")}, {"id": "decay", "fromS": 3.6, "toS": 8, "title": T("Decay and recovery", "Abklingen und Erholung")}],
            "events": [{"t": 2.5, "kind": "load-on", "title": T("Peak gust load", "Böenspitze")}, {"t": 5, "kind": "load-off", "title": T("Load below the mean again", "Last wieder unter dem Mittelwert")}], "tracks": tracks}
    sun = [(0, 0), (6, 0), (8, 0.43), (10, 0.79), (12, 1), (14, 0.79), (16, 0.36), (18, 0), (24, 0)]
    diurnal = {"id": "diurnal-transpiration", "title": T("Day of transpiration", "Transpirationstag"), "description": T("A 24 h cycle compressed to 24 s (1 s = 1 h) in calm weather: the sunlit fraction of every region and the transpiration strength follow the sun, the stomata nearly close at night and after the midday peak; leaf, berry and wood temperatures follow from the thermal solver.", "Ein 24-h-Zyklus auf 24 s gerafft (1 s = 1 h) bei ruhigem Wetter: der besonnte Anteil jeder Region und die Transpirationsstärke folgen der Sonne, die Spaltöffnungen schließen sich nachts und nach der Mittagsspitze fast; Blatt-, Beeren- und Holztemperaturen folgen aus dem thermischen Löser."),
               "durationS": 24, "fps": 10, "loop": True, "rootMotion": "none", "keys": [{"t": 0, "pose": "summer-calm", "ease": "smooth"}, {"t": 12, "pose": "summer-calm", "ease": "smooth"}, {"t": 24, "pose": "summer-calm", "ease": "smooth"}], "contacts": [],
               "phases": [{"id": "night", "fromS": 0, "toS": 6, "title": T("Night", "Nacht")}, {"id": "morning", "fromS": 6, "toS": 12, "title": T("Morning to noon", "Morgen bis Mittag")}, {"id": "afternoon", "fromS": 12, "toS": 18, "title": T("Afternoon", "Nachmittag")}, {"id": "evening", "fromS": 18, "toS": 24, "title": T("Evening and night", "Abend und Nacht")}],
               "events": [{"t": 6, "kind": "thermal", "title": T("Sunrise", "Sonnenaufgang")}, {"t": 12, "kind": "thermal", "title": T("Solar noon, peak transpiration", "Sonnenhöchststand, maximale Transpiration")}, {"t": 18, "kind": "thermal", "title": T("Sunset", "Sonnenuntergang")}],
               "tracks": [{"target": "emitter:transpiration-leaves", "ease": "smooth", "keys": [{"t": t, "v": v} for t, v in [(0, 0.03), (5, 0.05), (7, 0.35), (9, 0.8), (12, 1), (15, 0.75), (18, 0.3), (20, 0.06), (24, 0.03)]]}]
               + [{"target": f"region:{rid}:sunlitFraction", "ease": "smooth", "keys": [{"t": t, "v": r5(v * peak)} for t, v in sun]} for rid, peak in (("leaves", 0.55), ("vine-wood", 0.15), ("bunches", 0.2), ("posts", 0.5), ("wires", 0.6))]}
    return [sway_clip, gust, diurnal]


def symbol():
    plan = [{"id": "row", "bone": "root-plate", "role": "body", "z": 2, "filled": False, "tone": 0.5, "kind": "poly", "points": [[-2.0, -0.28], [2.0, -0.28], [2.0, 0.28], [-2.0, 0.28]], "closed": True},
            {"id": "post-l", "bone": "root-plate", "role": "body", "z": 3, "filled": True, "tone": 0.3, "kind": "ellipse", "cx": -1.95, "cy": 0, "rx": 0.05, "ry": 0.05},
            {"id": "post-r", "bone": "root-plate", "role": "body", "z": 3, "filled": True, "tone": 0.3, "kind": "ellipse", "cx": 1.95, "cy": 0, "rx": 0.05, "ry": 0.05},
            {"id": "wire", "bone": "root-plate", "role": "detail", "z": 4, "filled": False, "tone": 0.3, "kind": "poly", "points": [[-1.95, 0], [1.95, 0]], "closed": False}]
    plan += [{"id": f"vine-{i}", "bone": "root-plate", "role": "detail", "z": 5, "filled": True, "tone": 0.2, "kind": "ellipse", "cx": x, "cy": 0, "rx": 0.04, "ry": 0.04} for i, x in enumerate(VINES, 1)]
    elev = [{"id": "wall", "bone": "root-plate", "role": "body", "z": 2, "filled": False, "tone": 0.5, "kind": "poly", "points": [[-2.0, 0.8], [2.0, 0.8], [2.0, 1.8], [-2.0, 1.8]], "closed": True},
            {"id": "post-l", "bone": "root-plate", "role": "body", "z": 3, "filled": False, "tone": 0.3, "kind": "poly", "points": [[-2.0, 0], [-1.9, 0], [-1.9, 1.8], [-2.0, 1.8]], "closed": True},
            {"id": "post-r", "bone": "root-plate", "role": "body", "z": 3, "filled": False, "tone": 0.3, "kind": "poly", "points": [[1.9, 0], [2.0, 0], [2.0, 1.8], [1.9, 1.8]], "closed": True}]
    front = [{"id": "wall", "bone": "root-plate", "role": "body", "z": 2, "filled": False, "tone": 0.5, "kind": "poly", "points": [[-0.28, 0.8], [0.28, 0.8], [0.28, 1.8], [-0.28, 1.8]], "closed": True},
             {"id": "post", "bone": "root-plate", "role": "body", "z": 3, "filled": False, "tone": 0.3, "kind": "poly", "points": [[-0.05, 0], [0.05, 0], [0.05, 1.8], [-0.05, 1.8]], "closed": True}]
    return [{"id": "grapevine-row-symbol", "title": T("Plan symbol of the vine row", "Grundrisssymbol der Rebzeile"), "description": T("A sign for planting plans: the row band 4.0 m by 0.56 m with the two end posts, the fruiting wire and the four vine positions; a separate symbol, never used silently for a pictorial scale.", "Ein Zeichen für Pflanzpläne: das Zeilenband von 4,0 m mal 0,56 m mit den beiden Endpfählen, dem Biegedraht und den vier Stockpositionen; ein eigenes Symbol, nie stillschweigend für einen Bildmaßstab verwendet."),
             "semantics": "true-extent", "units": "model-m", "denominators": [500, 200, 100], "pens": pens(0.18, 0.13, None, None, 0.25), "hatches": [], "geometry": {"elevation": elev, "front": front, "plan": plan},
             "dimensionalNote": T("Drawn at true extent in model metres in all three views (4.0 m long, 0.56 m deep, 1.8 m high): the band is the leaf wall, posts and vine stocks at their real positions and thickness.", "In wahrer Ausdehnung in Modellmetern in allen drei Ansichten gezeichnet (4,0 m lang, 0,56 m tief, 1,8 m hoch): das Band ist die Laubwand, Pfähle und Rebstöcke in wirklicher Lage und Stärke.")}]


def main():
    reps, vols = {}, {}
    ext = None
    for d in SCALES:
        sc = SCALE_BUILD[d]()
        thermal, structure = thermal_and_structure(sc, outlets(sc), supports(sc))
        reps[str(d)] = rep(sc, *RANGES[d], PENS[d], DEC[d])
        vols[str(d)] = {"status": "authored", "denominator": d, "body": sc.body, "thermal": thermal, "structure": structure, "decisions": VOLUME_DECISIONS[d]}
        e = [round(hi - lo, 4) for lo, hi in sc.extent]
        print(f"[DEBUG] 1:{d} extent x {e[0]} y {e[1]} z {e[2]} lo {[round(lo, 3) for lo, _ in sc.extent]} solids {len(sc.body)}")
        ext = [r5(x) for x in e]
    mass = round(sum(b["mass"]["kg"] for b in rig.bones if "mass" in b), 2)
    asset = {
        "id": ID, "category": "vegetation", "rig": {"kind": "botanical", "root": "root-plate", "bones": rig.bones}, "anchors": ANCHORS,
        "title": T("Grapevine Row", "Rebzeile"),
        "summary": T("A 4 m segment of a vineyard row of grapevine (Vitis vinifera) trained as bilateral cordons on a vertical-shoot-positioned wire trellis: two round pine posts 1.8 m high, a fruiting wire at 0.8 m, two pairs of catch wires and a top wire. Four vines 1.0 m apart carry a hedged leaf wall 0.56 m thick from the cordon to 1.8 m and two bunches each in the open fruit zone. Trellis and vines are one asset.",
                     "Ein Abschnitt von 4 m einer Weinbergzeile aus Weinreben (Vitis vinifera), als zweiarmige Kordons an einem Drahtrahmen mit senkrechter Triebführung erzogen: zwei Kiefern-Rundpfähle von 1,8 m Höhe, ein Biegedraht in 0,8 m, zwei Heftdrahtpaare und ein Kopfdraht. Vier Rebstöcke im Abstand von 1,0 m tragen eine gestutzte Laubwand von 0,56 m Dicke vom Kordon bis 1,8 m und je zwei Trauben in der offenen Traubenzone. Drahtrahmen und Reben sind ein Asset."),
        "tags": ["grapevine", "vineyard", "climber", "trellis", "crop", "fruit", "deciduous", "wind", "transpiration"],
        "provenance": {"author": "Semio staffage plant worker PW-16", "method": "handcrafted", "created": "2026-11-22", "references": [{"id": k, "citation": v} for k, v in REFS.items()],
                       "notes": T("Row geometry after the VSP descriptions of Smart and Robinson (1991) and Keller (2015): fruiting wire 0.8 m, canopy wall to 1.8 m and 0.5 to 0.6 m thick, 1.0 m vine spacing. Masses: green vine wood 950 kg/m3, pine posts 560 kg/m3 (authored estimates guided by the Wood Handbook), 2.4 kg of shoots and leaves per vine, 0.3 kg per green bunch. Leaf conductance and leaf area are authored estimates guided by Keller (2015) and Jones (2014).",
                                  "Zeilengeometrie nach den Beschreibungen der senkrechten Triebführung bei Smart und Robinson (1991) und Keller (2015): Biegedraht 0,8 m, Laubwand bis 1,8 m und 0,5 bis 0,6 m dick, Stockabstand 1,0 m. Massen: frisches Rebholz 950 kg/m3, Kiefernpfähle 560 kg/m3 (Autorenschätzungen angelehnt an das Wood Handbook), 2,4 kg Triebe und Blätter je Stock, 0,3 kg je grüner Traube. Blattleitfähigkeit und Blattfläche sind Autorenschätzungen angelehnt an Keller (2015) und Jones (2014).")},
        "dimensions": {"heightM": ext[1], "widthM": ext[0], "depthM": ext[2], "massKg": mass},
        "materials": MATERIALS,
        "representations": reps,
        "symbols": symbol(),
        "poses": POSES,
        "clips": build_clips(),
        "thermal": {"environment": {"airTempC": 25, "meanRadiantTempC": 28, "relativeHumidity": 0.5, "windMS": 1.5, "windDirectionDeg": 0, "solarWM2": 750, "diffuseFraction": 0.15, "pressurePa": 101325}, "passengers": [], "operating": [], "regions": REGIONS, "emitters": EMITTERS,
                    "assumptions": [T("The leaf region uses the one-sided leaf area of the four vines (3 m2 each); its leaf area index of 5.4 is that area over the ground projection of the wall (4.0 m by 0.56 m). The maximum stomatal conductance of 6 mm/s is an authored estimate for a well-watered C3 grapevine, guided by Keller (2015) and Jones (2014).", "Die Blattregion verwendet die einseitige Blattfläche der vier Stöcke (je 3 m2); ihr Blattflächenindex von 5,4 ist diese Fläche bezogen auf die Grundprojektion der Wand (4,0 m mal 0,56 m). Die maximale Spaltöffnungsleitfähigkeit von 6 mm/s ist eine Autorenschätzung für eine gut versorgte C3-Rebe, angelehnt an Keller (2015) und Jones (2014)."),
                                    T("Wood, berries, posts and wires are passive regions: areal heat capacities from volumetric heat capacity times an effective depth (vine wood 2.4 MJ/(m3 K) x 12 mm, berries 3.9 MJ/(m3 K) x 13 mm, pine 1.0 MJ/(m3 K) x 12 mm, steel 3.6 MJ/(m3 K) x 0.6 mm). No surface temperature is authored.", "Holz, Beeren, Pfähle und Drähte sind passive Regionen: flächenbezogene Wärmekapazitäten aus volumetrischer Wärmekapazität mal wirksamer Tiefe (Rebholz 2,4 MJ/(m3 K) x 12 mm, Beeren 3,9 MJ/(m3 K) x 13 mm, Kiefer 1,0 MJ/(m3 K) x 12 mm, Stahl 3,6 MJ/(m3 K) x 0,6 mm). Es werden keine Oberflächentemperaturen vorgegeben.")]},
        "structure": {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": [a for a, _ in SUPPORTS], "assumptions": [T("One rigid body per evaluated pose, fixed at the two posts and at the four root crowns. Wind drag acts on each leaf wall (cf 0.9 on 0.5 m2, the wall section the wind along the row meets) and on each bunch (cf 1.0 on 0.018 m2), EN 1991-1-4 form; gravity on every bone mass. The posts are round pine of 100 mm; wire pretension and dynamic amplification are not modelled.", "Ein starrer Körper je ausgewerteter Pose, an den beiden Pfählen und den vier Wurzelhälsen eingespannt. Winddruck wirkt auf jede Laubwand (cf 0,9 auf 0,5 m2, den Wandquerschnitt, auf den Wind längs der Zeile trifft) und auf jede Traube (cf 1,0 auf 0,018 m2), Form nach EN 1991-1-4; Schwerkraft auf jede Knochenmasse. Die Pfähle sind Kiefernrundholz von 100 mm; Drahtvorspannung und dynamische Überhöhung werden nicht modelliert.")], "clinical": False},
        "volumes": vols,
        "presentation": {"defaultPose": "summer-calm", "defaultClip": "wind-sway", "defaultScale": 100},
        "exports": EXPORTS, "completion": "authored",
    }
    print(write(asset, ID))
    manifest_row(asset, T("A vineyard row of four cordon-trained grapevines (Vitis vinifera) on a vertical-shoot-positioned wire trellis between two pine posts, 4.0 m long and 1.8 m high: a hedged leaf wall 0.56 m thick above the fruiting wire and two bunches per vine in the open fruit zone. Trellis and vines are one asset.",
                          "Eine Weinbergzeile aus vier kordonerzogenen Weinreben (Vitis vinifera) an einem Drahtrahmen mit senkrechter Triebführung zwischen zwei Kiefernpfählen, 4,0 m lang und 1,8 m hoch: eine gestutzte Laubwand von 0,56 m Dicke über dem Biegedraht und je zwei Trauben in der offenen Traubenzone. Drahtrahmen und Reben sind ein Asset."))


if __name__ == "__main__":
    main()
