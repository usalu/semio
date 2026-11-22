"""🪻️ wisteria-pergola: an old Chinese wisteria (Wisteria sinensis) in flower on a freestanding timber pergola, 3.6 m x 2.8 m, 3.1 m high to the top of the canopy.

The pergola is passive structure: four C24 posts, two beams, six rafters. The plant twines up beside one post, runs two leaders over the rafters and carries a
canopy of young leaves with racemes hanging between the rafters. Run: `python3 wisteria.py`. Every number below is designed here per scale.
@see ./pw16lib.py
"""
import math
from pw16lib import *  # noqa: F401,F403

ID = "wisteria-pergola"
POSTS = {"fl": (1.5, 1.0), "fr": (1.5, -1.0), "bl": (-1.5, 1.0), "br": (-1.5, -1.0)}
POST_W, POST_H = 0.14, 2.56
BEAM_Y, BEAM_D, BEAM_W, BEAM_X = 2.55, 0.20, 0.08, 1.8
RAFT_Y, RAFT_D, RAFT_W, RAFT_Z = 2.745, 0.16, 0.06, 1.4
RAFTERS = [-1.5, -0.9, -0.3, 0.3, 0.9, 1.5]
TOP = 3.10
TRUNK = (1.3, -0.82)
CANOPY_X = [-1.75, -0.875, 0.0, 0.875]
RACEMES = [(1, -1.2, -1.2), (1, -1.2, 0.15), (1, -1.2, 0.78), (2, -0.6, -0.74), (2, -0.6, 0.45), (2, -0.6, 1.2), (3, 0.0, -0.74), (3, 0.0, 0.15), (3, 0.6, -0.3), (3, 0.6, 0.78), (4, 1.2, -1.2), (4, 1.2, 0.15), (4, 1.2, 1.2)]
TIMBER, WOOD, LEAF, FLOWER = "timber-c24", "wisteria-wood", "wisteria-leaf", "wisteria-flower"

rig = Rig()
rig.add("root-plate", None, T("Paving line", "Belagslinie"), 0, 0, 0, 0, FIXED)
post_section = {"areaM2": r5(POST_W ** 2), "secondMomentM4": round(POST_W ** 4 / 12, 10), "extremeFibreM": POST_W / 2, "material": TIMBER, "title": T("C24 post 140 x 140 mm", "Pfosten C24 140 x 140 mm")}
for key, (x, z) in POSTS.items():
    rig.add(f"post-{key}", "root-plate", T(f"Post {'front' if key[0] == 'f' else 'back'} {'left' if key[1] == 'l' else 'right'}", f"Pfosten {'vorne' if key[0] == 'f' else 'hinten'} {'links' if key[1] == 'l' else 'rechts'}"), x, 0, 90, POST_H, FIXED, z=z, mass=(POST_W ** 2 * POST_H * 420, "literature", "en-338"), section=post_section)
beam_section = {"areaM2": r5(BEAM_W * BEAM_D), "secondMomentM4": round(BEAM_W * BEAM_D ** 3 / 12, 10), "extremeFibreM": BEAM_D / 2, "material": TIMBER, "title": T("C24 beam 80 x 200 mm", "Pfette C24 80 x 200 mm")}
for side, z in (("l", 1.0), ("r", -1.0)):
    rig.add(f"beam-{side}", "root-plate", T(f"Beam {'left' if side == 'l' else 'right'}", f"Pfette {'links' if side == 'l' else 'rechts'}"), -BEAM_X, BEAM_Y, 0, 2 * BEAM_X, FIXED, z=z, mass=(BEAM_W * BEAM_D * 2 * BEAM_X * 420, "literature", "en-338", (BEAM_X, BEAM_D / 2)), section=beam_section)
rig.add("rafters", "root-plate", T("Six rafters 60 x 160 mm", "Sechs Sparren 60 x 160 mm"), -1.5, RAFT_Y, 0, 3.0, FIXED, mass=(len(RAFTERS) * RAFT_W * RAFT_D * 2 * RAFT_Z * 420, "literature", "en-338", (1.5, RAFT_D / 2)))
rig.add("trunk", "root-plate", T("Twining trunk", "Windender Stamm"), TRUNK[0], 0, 90, 2.92, revolute(-1.5, 1.5), z=TRUNK[1], mass=(math.pi * 0.055 ** 2 * 2.92 * 800, "authored-estimate", "wood-handbook"))
rig.add("leader-r", "trunk", T("Leader over the right beam", "Leittrieb über der rechten Pfette"), 2.92, 0, 90, 2.9, revolute(-2, 2), z=-0.08, mass=(math.pi * 0.03 ** 2 * 2.9 * 800, "authored-estimate", "wood-handbook"))
a, b, t = orient(rig, "trunk", [0, 0, 1])
rig.add("leader-x", "trunk", T("Leader across the rafters", "Leittrieb quer über die Sparren"), 2.92, 0, a, 1.72, revolute(-2, 2), abduction=b, twist=t, mass=(math.pi * 0.03 ** 2 * 1.72 * 800, "authored-estimate", "wood-handbook"))
for i, x0 in enumerate(CANOPY_X, 1):
    rig.add_world(f"canopy-{i}", "leader-r", T(f"Canopy section {i}", f"Laubdachabschnitt {i}"), [x0, 2.95, 0], 0, 0.875, revolute(-4, 4), mass=(6.0, "authored-estimate", "valder-1995"))
for j, (c, x, z) in enumerate(RACEMES, 1):
    rig.add_world(f"raceme-{j}", f"canopy-{c}", T(f"Raceme cluster {j}", f"Traubenbüschel {j}"), [x, 2.86, z], -90, 0.32, revolute(-35, 35), mass=(0.15, "authored-estimate", "valder-1995", (0.15, 0)))


def timber(sc, rafter_w):
    for key in POSTS:
        sc.plate(f"post-{key}", f"post-{key}", [(0, -POST_W / 2), (POST_H, -POST_W / 2), (POST_H, POST_W / 2), (0, POST_W / 2)], -POST_W / 2, POST_W / 2, region="timber", material=TIMBER, tone=0.62)
    for side in "lr":
        sc.plate(f"beam-{side}", f"beam-{side}", [(0, 0), (2 * BEAM_X, 0), (2 * BEAM_X, BEAM_D), (0, BEAM_D)], -BEAM_W / 2, BEAM_W / 2, region="timber", material=TIMBER, tone=0.58)
    for k, x in enumerate(RAFTERS):
        u = x + 1.5
        sc.plate(f"rafter-{k + 1}", "rafters", [(u - rafter_w / 2, 0), (u + rafter_w / 2, 0), (u + rafter_w / 2, RAFT_D), (u - rafter_w / 2, RAFT_D)], -RAFT_Z, RAFT_Z, region="timber", material=TIMBER, tone=0.66)


def twined_trunk(sc, turns, n, r, d):
    """Two stems twined anticlockwise (seen from above, as Wisteria sinensis twines) around the axis of the trunk bone, as straight members between helix stations."""
    sc.rod("trunk-foot", "trunk", [0, 0, 0], [0.12, 0, 0], 2 * r + d, region="wood", material=WOOD, tone=0.26)
    for s in range(2):
        pts = [(0.1 + 2.82 * k / n, r * math.cos(2 * math.pi * turns * k / n + s * math.pi), r * math.sin(2 * math.pi * turns * k / n + s * math.pi)) for k in range(n + 1)]
        for k in range(n):
            sc.rod(f"trunk-{s}-{k}", "trunk", list(pts[k]), list(pts[k + 1]), d, region="wood", material=WOOD, tone=0.28 + 0.04 * s)


def leaders(sc, d):
    sc.rod("leader-r", "leader-r", [0, 0, 0], [2.9, 0, 0], d, region="wood", material=WOOD, tone=0.3)
    sc.rod("leader-x", "leader-x", [0, 0, 0], [1.72, 0, 0], d, region="wood", material=WOOD, tone=0.3)


def lobe(sc, id, i, x, y, z, rx, ry, rz, tone):
    bone = f"canopy-{i}"
    x = max(-BEAM_X + 0.05 + rx, min(BEAM_X - 0.05 - rx, x))
    rz = min(rz, RAFT_Z - 0.05 - abs(z))
    ry = min(ry, TOP - y)
    sc.ell(id, bone, rig.local(bone, [x, y, z]), (rx, ry, rz), region="leaves", material=LEAF, tone=tone)


def canopy(sc, layout):
    for i, x0 in enumerate(CANOPY_X, 1):
        xc = x0 + 0.4375
        for k, (dx, y, z, rx, ry, rz, tone) in enumerate(layout):
            lobe(sc, f"canopy-{i}-{k}", i, xc + dx, y, z, rx, ry, rz, tone + 0.02 * (i % 3))


def racemes(sc, parts, spread):
    for j, (c, x, z) in enumerate(RACEMES, 1):
        bone = f"raceme-{j}"
        if len(spread) > 1:
            sc.ell(f"raceme-{j}-hub", bone, [0.0, 0.0, 0.0], (0.06, 0.15, 0.15), region="racemes", material=FLOWER, tone=0.5)
        for m, (dv, dw, scale) in enumerate(spread):
            for n, (u, ru, r) in enumerate(parts):
                sc.ell(f"raceme-{j}-{m}-{n}", bone, [u * scale, dv, dw], (ru * scale, r, r), region="racemes", material=FLOWER, tone=0.78 - 0.04 * n)


CANOPY_50 = [(0.0, 2.83, 0.0, 0.5, 0.1, 1.3, 0.4), (-0.15, 2.97, -0.75, 0.42, 0.13, 0.6, 0.36), (0.15, 2.97, 0.7, 0.42, 0.13, 0.62, 0.42), (0.0, 2.98, 0.0, 0.44, 0.12, 0.55, 0.38),
             (-0.2, 3.02, 0.35, 0.22, 0.08, 0.3, 0.46), (0.2, 3.02, -0.35, 0.22, 0.08, 0.3, 0.34), (0.0, 2.95, 1.18, 0.38, 0.1, 0.17, 0.4), (0.0, 2.95, -1.18, 0.38, 0.1, 0.17, 0.44)]


def scale50():
    sc = Scale(rig, 50)
    timber(sc, RAFT_W)
    twined_trunk(sc, 2.5, 20, 0.022, 0.065)
    leaders(sc, 0.06)
    canopy(sc, CANOPY_50)
    racemes(sc, [(0.07, 0.07, 0.045), (0.16, 0.07, 0.036), (0.25, 0.06, 0.024)], [(0.0, 0.0, 1.0), (0.1, 0.11, 0.85), (-0.1, -0.11, 0.75)])
    for k in range(5):
        sc.stroke(f"post-fr-grain-{k}", "post-fr", [(0.25 + 0.45 * k, -0.03), (0.55 + 0.45 * k, -0.035)], role="texture")
    return sc


def scale100():
    sc = Scale(rig, 100)
    timber(sc, RAFT_W)
    twined_trunk(sc, 1.5, 9, 0.02, 0.07)
    leaders(sc, 0.06)
    canopy(sc, [(0.0, 2.83, 0.0, 0.5, 0.1, 1.3, 0.4), (0.0, 2.97, -0.7, 0.45, 0.13, 0.65, 0.37), (0.0, 2.97, 0.7, 0.45, 0.13, 0.65, 0.42), (0.0, 2.99, 0.0, 0.44, 0.11, 0.5, 0.39), (0.0, 2.95, 1.2, 0.36, 0.1, 0.15, 0.41), (0.0, 2.95, -1.2, 0.36, 0.1, 0.15, 0.44)])
    racemes(sc, [(0.09, 0.09, 0.044), (0.22, 0.08, 0.03)], [(0.0, 0.0, 1.0), (0.1, 0.11, 0.8)])
    return sc


def scale200():
    sc = Scale(rig, 200)
    timber(sc, RAFT_W)
    sc.rod("trunk", "trunk", [0, 0, 0], [2.92, 0, 0], 0.11, region="wood", material=WOOD, tone=0.28)
    leaders(sc, 0.06)
    canopy(sc, [(0.0, 2.84, 0.0, 0.5, 0.11, 1.3, 0.4), (0.0, 2.97, -0.68, 0.48, 0.13, 0.67, 0.38), (0.0, 2.97, 0.68, 0.48, 0.13, 0.67, 0.42)])
    racemes(sc, [(0.15, 0.15, 0.045)], [(0.0, 0.0, 1.0)])
    return sc


def scale500():
    sc = Scale(rig, 500)
    timber(sc, 0.076)
    sc.rod("trunk", "trunk", [0, 0, 0], [2.92, 0, 0], 0.11, region="wood", material=WOOD, tone=0.28)
    leaders(sc, 0.06)
    canopy(sc, [(0.0, 2.93, 0.0, 0.5, 0.17, 1.35, 0.4)])
    racemes(sc, [(0.14, 0.17, 0.045)], [(0.0, 0.0, 1.0)])
    return sc


SUPPORTS = [(f"base-{k}", f"post-{k}") for k in POSTS] + [("root-crown", "trunk")]
SCALE_BUILD = {50: scale50, 100: scale100, 200: scale200, 500: scale500}
PENS = {50: pens(0.25, 0.18, 0.1, None, 0.35), 100: pens(0.25, 0.13, None, None, 0.35), 200: pens(0.18, 0.13, None, None, 0.25), 500: pens(0.18, 0.1, None, None, 0.25)}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}

DEC = {
    50: decisions(
        T("At 1:50 (1 m is 20 mm) the pergola is 72 mm wide and 62 mm high: four square posts, two beams, six rafters seen end-on in the side elevation, the trunk of two twined stems, two leaders over the rafters, a canopy of eight overlapping leaf lobes per section and 13 clusters of three tapering racemes between the rafters.", "Bei 1:50 (1 m sind 20 mm) ist die Pergola 72 mm breit und 62 mm hoch: vier Vierkantpfosten, zwei Pfetten, sechs Sparren in der Seitenansicht im Schnitt gesehen, der Stamm aus zwei verwundenen Trieben, zwei Leittriebe über den Sparren, ein Laubdach aus acht überlappenden Blattlappen je Abschnitt und 13 Büschel aus je drei sich verjüngenden Trauben zwischen den Sparren."),
        T("Retained: the true timber sections, the anticlockwise twining of the trunk, the racemes hanging free below the rafters, the leaf mat draped through the rafters.", "Beibehalten: die wahren Holzquerschnitte, das Linkswinden des Stamms, die frei unter den Sparren hängenden Trauben, die durch die Sparren hängende Blattdecke."),
        T("Merged: pinnate leaves and twigs into lobes; the flowers of a raceme into three stacked spindles.", "Zusammengefasst: Fiederblätter und Zweige zu Lappen; die Blüten einer Traube zu drei gestapelten Spindeln."),
        T("Omitted: post bases, screws, single flowers.", "Weggelassen: Pfostenfüße, Schrauben, Einzelblüten."),
        T("Leaders (60 mm) and the twined stems (65 mm) are true members; wood grain on the near post is texture.", "Leittriebe (60 mm) und die verwundenen Triebe (65 mm) sind wahre Stäbe; die Holzmaserung am vorderen Pfosten ist Textur."),
        T("None; grain strokes on the near post.", "Keine; Maserungsstriche am vorderen Pfosten."),
        T("Light racemes against the darker leaf mat, mid-grey timber.", "Helle Trauben vor der dunkleren Blattdecke, mittelgraues Holz."),
        T("Lobes carry the leaf region, racemes the flower region, trunk and leaders the wood region, posts, beams and rafters the timber region.", "Lappen tragen die Blattregion, Trauben die Blütenregion, Stamm und Leittriebe die Holzregion, Pfosten, Pfetten und Sparren die Bauholzregion.")),
    100: decisions(
        T("At 1:100 (1 m is 10 mm) the pergola is 36 mm by 31 mm: timber frame, trunk of two stems in a coarser twist, leaders, six lobes per canopy section, 13 clusters of two racemes.", "Bei 1:100 (1 m sind 10 mm) misst die Pergola 36 mm mal 31 mm: Holzrahmen, Stamm aus zwei Trieben in gröberer Windung, Leittriebe, sechs Lappen je Laubdachabschnitt, 13 Büschel aus zwei Trauben."),
        T("Retained: rafters end-on, the hanging racemes, the twining trunk.", "Beibehalten: Sparren im Schnitt, die hängenden Trauben, der windende Stamm."),
        T("Merged: top tufts into the lobes; each raceme into two spindles.", "Zusammengefasst: obere Büschel in die Lappen; jede Traube zu zwei Spindeln."),
        T("Omitted: texture, the third raceme of a cluster.", "Weggelassen: Textur, die dritte Traube eines Büschels."),
        T("Leaders 60 mm print 0.6 mm.", "Leittriebe von 60 mm drucken mit 0,6 mm."),
        T("None.", "Keine."),
        T("As at 1:50.", "Wie bei 1:50."),
        T("As at 1:50.", "Wie bei 1:50.")),
    200: decisions(
        T("At 1:200 (1 m is 5 mm) the pergola is an 18 mm frame under a leaf mat of three lobes per section, a single trunk and one raceme per cluster.", "Bei 1:200 (1 m sind 5 mm) ist die Pergola ein Rahmen von 18 mm unter einer Blattdecke aus drei Lappen je Abschnitt, ein einzelner Stamm und eine Traube je Büschel."),
        T("Retained: posts, beams, every rafter, the raceme fringe, the trunk at its post.", "Beibehalten: Pfosten, Pfetten, jeder Sparren, der Traubenbehang, der Stamm an seinem Pfosten."),
        T("Merged: the twined stems into one trunk of 110 mm; racemes into one spindle each.", "Zusammengefasst: die verwundenen Triebe zu einem Stamm von 110 mm; Trauben zu je einer Spindel."),
        T("Omitted: twist, second raceme.", "Weggelassen: Windung, zweite Traube."),
        T("Rafters (60 mm) print 0.3 mm.", "Sparren (60 mm) drucken mit 0,3 mm."),
        T("None.", "Keine."),
        T("Three greys.", "Drei Grautöne."),
        T("As at 1:50.", "Wie bei 1:50.")),
    500: decisions(
        T("At 1:500 (1 m is 2 mm) the pergola is a 7 mm frame under one leaf slab per canopy section with a fringe of 13 racemes.", "Bei 1:500 (1 m sind 2 mm) ist die Pergola ein Rahmen von 7 mm unter einer Blattplatte je Laubdachabschnitt mit einem Behang aus 13 Trauben."),
        T("Retained: true width, depth and height, posts, beams, rafters, the trunk.", "Beibehalten: wahre Breite, Tiefe und Höhe, Pfosten, Pfetten, Sparren, der Stamm."),
        T("Merged: the canopy of a section into one slab.", "Zusammengefasst: das Laubdach eines Abschnitts zu einer Platte."),
        T("Omitted: lobes, raceme taper.", "Weggelassen: Lappen, Traubenverjüngung."),
        T("Rafters are thickened from 60 to 76 mm, the smallest printable feature (0.15 mm); leaders stay true slender members.", "Sparren sind von 60 auf 76 mm verdickt, das kleinste druckbare Merkmal (0,15 mm); Leittriebe bleiben wahre schlanke Stäbe."),
        T("None.", "Keine."),
        T("One leaf grey, light racemes.", "Ein Blattgrau, helle Trauben."),
        T("As at 1:50.", "Wie bei 1:50.")),
}

VOLUME_DECISIONS = {
    50: [T("Body at 1:50: posts, beams and rafters as true prisms, the trunk as two stems twined in two and a half turns (40 straight members), two leaders, eight leaf lobes per canopy section with a draped lower mat that fills the rafter bays, 39 racemes of three stacked spindles.", "Körper bei 1:50: Pfosten, Pfetten und Sparren als wahre Prismen, der Stamm als zwei in zweieinhalb Windungen verwundene Triebe (40 gerade Stäbe), zwei Leittriebe, acht Blattlappen je Laubdachabschnitt mit einer herabhängenden unteren Decke, die die Sparrenfelder füllt, 39 Trauben aus drei gestapelten Spindeln."),
         T("Thermal: every body solid by reference in its region, one transpiration outlet in canopy section 2. Structure: one member per bone and per drawn bar by reference, a support block at each post foot and at the root crown.", "Thermik: jeder Körper per Verweis in seiner Region, ein Transpirationsauslass im Laubdachabschnitt 2. Tragwerk: ein Stab je Knochen und je gezeichnetem Stab per Verweis, ein Auflagerblock an jedem Pfostenfuß und am Wurzelhals.")],
    100: [T("Body at 1:100: trunk of two stems in one and a half turns, six lobes per section, 26 racemes of two spindles.", "Körper bei 1:100: Stamm aus zwei Trieben in eineinhalb Windungen, sechs Lappen je Abschnitt, 26 Trauben aus zwei Spindeln."), T("Thermal and structure by reference.", "Thermik und Tragwerk per Verweis.")],
    200: [T("Body at 1:200: one trunk tube, three lobes per section, 13 single racemes.", "Körper bei 1:200: ein Stammrohr, drei Lappen je Abschnitt, 13 einzelne Trauben."), T("Thermal and structure by reference.", "Thermik und Tragwerk per Verweis.")],
    500: [T("Minimal: the structure of 1:500 is one member per bone and per drawn bar and one support per anchor, as at 1:200; fewer would leave a drawn post, rafter or raceme without its member.", "Minimal: das Tragwerk von 1:500 ist ein Stab je Knochen und je gezeichnetem Stab und ein Auflager je Anker, wie bei 1:200; weniger ließe einen gezeichneten Pfosten, Sparren oder eine Traube ohne ihren Stab."),
          T("Body at 1:500: one leaf slab per section, rafters of 76 mm, 13 racemes of 90 mm.", "Körper bei 1:500: eine Blattplatte je Abschnitt, Sparren von 76 mm, 13 Trauben von 90 mm."), T("Thermal and structure by reference; supports of 80 mm.", "Thermik und Tragwerk per Verweis; Auflager von 80 mm.")],
}

REFS = {
    "valder-1995": "Valder, P. (1995). Wisterias: A Comprehensive Guide. Florilegium, Balmain, and Timber Press, Portland. Wisteria sinensis twines anticlockwise, racemes 15-30 cm opening all at once before the leaves, pinnate leaves of 7-13 leaflets.",
    "flora-of-china-10": "Wei, Z. and Pedley, L. (2010). Wisteria. In: Wu, Z. Y., Raven, P. H. and Hong, D. Y. (eds.), Flora of China 10 (Fabaceae), 181-183. Science Press, Beijing, and Missouri Botanical Garden Press, St. Louis.",
    "jones-2014": "Jones, H. G. (2014). Plants and Microclimate, 3rd ed., Cambridge University Press. Leaf energy balance, leaf area index, boundary-layer and stomatal conductance.",
    "monteith-unsworth-2013": "Monteith, J. L. and Unsworth, M. H. (2013). Principles of Environmental Physics, 4th ed., Academic Press. Penman-Monteith leaf transpiration.",
    "en-338": "EN 338:2016 Structural timber, strength classes (C24: mean density 420 kg/m3, E0,mean 11 GPa, f_m,k 24 MPa).",
    "en-1991-1-4": "EN 1991-1-4:2005 Eurocode 1, Actions on structures, Part 1-4: Wind actions. Dynamic pressure q = 0.5 rho v^2 with air density 1.25 kg/m3, force F = cf q A, force coefficient of rectangular sections.",
    "wood-handbook": "Forest Products Laboratory (2021). Wood Handbook, Wood as an Engineering Material, General Technical Report FPL-GTR-282. Green density, specific heat and conductivity of wood.",
}

MATERIALS = [
    {"id": TIMBER, "title": T("Structural timber C24", "Bauholz C24"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 420, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11000000000, "strengthPa": 24000000, "source": "literature", "reference": "en-338"},
    {"id": WOOD, "title": T("Wisteria wood with smooth grey bark", "Glyzinienholz mit glatter grauer Rinde"), "emissivity": 0.93, "solarAbsorptivity": 0.65, "densityKgM3": 800, "conductivityWmK": 0.18, "specificHeatJKgK": 2500, "source": "authored-estimate", "reference": "wood-handbook"},
    {"id": LEAF, "title": T("Young pinnate wisteria leaves", "Junge gefiederte Glyzinienblätter"), "emissivity": 0.96, "solarAbsorptivity": 0.45, "source": "authored-estimate", "reference": "jones-2014"},
    {"id": FLOWER, "title": T("Lilac-blue pea flowers", "Lilablaue Schmetterlingsblüten"), "emissivity": 0.95, "solarAbsorptivity": 0.5, "densityKgM3": 300, "source": "authored-estimate", "reference": "valder-1995"},
]

REGIONS = [
    leaf_region("leaves", T("Wisteria leaves", "Glyzinienblätter"), LEAF, 18.0, 0.05, "horizontal-plate-up", 0.7, 400, 0.004, 1.8, "c3", "valder-1995"),
    passive_region("racemes", T("Racemes in flower", "Blühende Trauben"), "surface", FLOWER, 3.8, 0.08, "cylinder", 0.3, 2000, "valder-1995"),
    passive_region("wood", T("Trunk and leaders", "Stamm und Leittriebe"), "bark", WOOD, 2.0, 0.07, "cylinder", 0.4, 30000, "wood-handbook"),
    passive_region("timber", T("Posts, beams and rafters", "Pfosten, Pfetten und Sparren"), "surface", TIMBER, 18.0, 0.14, "vertical-plate", 0.5, 7000, "en-338"),
]
REGIONS[0]["areaBasis"] = "exchange"

ANCHORS = [anchors_support(f"post-{k}", T(f"Foot of post {k}", f"Fuß von Pfosten {k}"), 0.14, 0.14, id=f"base-{k}") for k in POSTS]
ANCHORS.append(anchors_support("trunk", T("Root crown of the wisteria", "Wurzelhals der Glyzinie"), 0.12, 0.12, id="root-crown"))
TP = []
for i in range(1, 5):
    for k, u in enumerate((0.25, 0.65)):
        aid = f"tp-{i}{'ab'[k]}"
        TP.append(aid)
        ANCHORS.append(emitter_anchor(aid, f"canopy-{i}", u, 0.08, T(f"Transpiration of canopy section {i}, {'first' if k == 0 else 'second'} half", f"Transpiration des Laubdachabschnitts {i}, {'erste' if k == 0 else 'zweite'} Hälfte")))
EMITTERS = [transpiration("transpiration-leaves", T("Transpiration of the leaf canopy (distributed over eight anchors)", "Transpiration des Laubdachs (über acht Anker verteilt)"), TP, "leaves", 0, 0.9)]


def loads(v, extra=None):
    out = [wind_load(f"wind-canopy-{i}", T(f"Wind drag on canopy section {i}", f"Winddruck auf Laubdachabschnitt {i}"), f"canopy-{i}", 0.44, 0, 1.0, 0.35, v) for i in range(1, 5)]
    out += [wind_load(f"wind-raceme-{j}", T(f"Wind drag on raceme cluster {j}", f"Winddruck auf Traubenbüschel {j}"), f"raceme-{j}", 0.15, 0, 1.2, 0.03, v) for j in range(1, len(RACEMES) + 1)]
    out += [wind_load(f"wind-post-{k}", T(f"Wind drag on post {k}", f"Winddruck auf Pfosten {k}"), f"post-{k}", 1.3, 0, 2.0, POST_W * POST_H, v) for k in POSTS]
    return out + (extra or [])


def binding(leaf_sun, emitter, flower_sun=0.3):
    return bind({"leaves": (1, leaf_sun), "racemes": (1, flower_sun), "wood": (1, 0.2), "timber": (1, 0.45)}, {"transpiration-leaves": emitter})


def offsets(lean, swing, trunk=0.0):
    out = {f"canopy-{i}": {"angleDeg": r5(lean * (0.8 + 0.1 * i))} for i in range(1, 5)}
    out.update({f"raceme-{j}": {"angleDeg": r5(swing * (0.85 + 0.03 * (j % 5)))} for j in range(1, len(RACEMES) + 1)})
    if trunk:
        out["trunk"] = {"angleDeg": trunk}
    return out


POSES = [
    make_pose("flowering-calm", T("Calm day in full flower", "Windstiller Tag in voller Blüte"), T("A still day in May: the racemes hang straight between the rafters, the young leaves are half grown. Light air of 1 m/s.", "Ein stiller Tag im Mai: die Trauben hängen gerade zwischen den Sparren, die jungen Blätter sind halb entfaltet. Leiser Zug von 1 m/s."), {}, binding(0.7, 0.85), loads(1.0)),
    make_pose("breezy", T("Fresh breeze", "Frische Brise"), T("7 m/s at roof height (Beaufort 4): the racemes swing 12 degrees downwind, the canopy lifts by a degree; about 11 N per canopy section and 1.1 N per raceme cluster.", "7 m/s in Dachhöhe (Beaufort 4): die Trauben schwingen 12 Grad nach Lee, das Laubdach hebt sich um ein Grad; etwa 11 N je Laubdachabschnitt und 1,1 N je Traubenbüschel."), offsets(1.0, 12), binding(0.7, 1.0), loads(7)),
    make_pose("storm", T("Spring storm", "Frühjahrssturm"), T("20 m/s gusts under cloud: the racemes stream out at 30 degrees, the canopy lifts 3 degrees and the trunk yields a fraction; the timber frame takes about 90 N per canopy section and 160 N per post.", "Böen von 20 m/s bei Bewölkung: die Trauben wehen unter 30 Grad aus, das Laubdach hebt sich 3 Grad und der Stamm gibt ein wenig nach; der Holzrahmen nimmt etwa 90 N je Laubdachabschnitt und 160 N je Pfosten auf."), offsets(3.0, 30, 0.3), binding(0.15, 0.4, 0.05), loads(20)),
    make_pose("flowering-night", T("Mild night in flower", "Milde Blütennacht"), T("A calm May night without sun: stomata nearly closed, the flowers scent the still air.", "Eine ruhige Mainacht ohne Sonne: die Spaltöffnungen sind fast geschlossen, die Blüten duften in der stillen Luft."), {}, binding(0.0, 0.05, 0.0), loads(0.5)),
]


def clips():
    base = {k: v["angleDeg"] for k, v in offsets(1.0, 12).items()}
    specs = {f"canopy-{i}": (0.8, 60 * i, 1) for i in range(1, 5)}
    specs.update({f"raceme-{j}": (7.0, 28 * j, 1) for j in range(1, len(RACEMES) + 1)})
    return [
        wind_sway_clip(rig, "breezy", 2.4, base, specs, T("A deterministic 2.4 s cycle (racemes as pendulums of about 0.3 m, slowed by drag, estimated 0.42 Hz) about the breeze lean: the racemes swing by 7 degrees with phases spread over the roof, the canopy breathes by under a degree.", "Ein deterministischer Zyklus von 2,4 s (Trauben als Pendel von etwa 0,3 m, durch den Widerstand gebremst, geschätzt 0,42 Hz) um die Neigung der Brise: die Trauben schwingen um 7 Grad mit über das Dach verteilten Phasen, das Laubdach atmet um weniger als ein Grad."), "transpiration-leaves"),
        gust_clip("breezy", lambda s: offsets(3.0 * s, 30 * s, 0.3 * s), [ld["id"] for ld in loads(7)], T("A single 8 s gust: the wind-load tracks rise from 5 % to 2.75 times the breeze load at 2.5 s (EN 1991-1-4 peak pressure with Iv 0.25) and decay; the racemes stream out to 70 % of the storm swing and fall back.", "Eine einzelne Böe über 8 s: die Windlast-Spuren steigen bei 2,5 s von 5 % auf das 2,75-fache der Last der Brise (Spitzendruck nach EN 1991-1-4 mit Iv 0,25) und klingen ab; die Trauben wehen bis zu 70 % der Sturmauslenkung aus und fallen zurück.")),
        diurnal_clip("flowering-calm", "transpiration-leaves", [(0, 0.04), (5, 0.05), (7, 0.3), (9, 0.75), (12, 1), (15, 0.8), (18, 0.3), (20, 0.06), (24, 0.04)], (("leaves", 0.7), ("racemes", 0.3), ("wood", 0.2), ("timber", 0.45)),
                     T("A 24 h day in May compressed to 24 s (1 s = 1 h): the sunlit fraction of the canopy, the racemes, the wood and the frame and the transpiration strength follow the sun; temperatures follow from the thermal solver.", "Ein 24-h-Tag im Mai auf 24 s gerafft (1 s = 1 h): der besonnte Anteil von Laubdach, Trauben, Holz und Rahmen und die Transpirationsstärke folgen der Sonne; die Temperaturen folgen aus dem thermischen Löser.")),
    ]


def symbol(ext):
    (x0, x1), (_, y1), (z0, z1) = ext
    plan = [{"id": "canopy", "bone": "root-plate", "role": "body", "z": 2, "filled": False, "tone": 0.5, "kind": "poly", "points": [[r5(x0), r5(z0)], [r5(x1), r5(z0)], [r5(x1), r5(z1)], [r5(x0), r5(z1)]], "closed": True}]
    plan += [{"id": f"post-{k}", "bone": "root-plate", "role": "body", "z": 3, "filled": True, "tone": 0.3, "kind": "poly", "points": [[x - 0.07, z - 0.07], [x + 0.07, z - 0.07], [x + 0.07, z + 0.07], [x - 0.07, z + 0.07]], "closed": True} for k, (x, z) in POSTS.items()]
    plan.append({"id": "trunk", "bone": "root-plate", "role": "detail", "z": 4, "filled": True, "tone": 0.2, "kind": "ellipse", "cx": TRUNK[0], "cy": TRUNK[1], "rx": 0.055, "ry": 0.055})
    elev = [{"id": "frame", "bone": "root-plate", "role": "body", "z": 2, "filled": False, "tone": 0.5, "kind": "poly", "points": [[r5(x0), 2.55], [r5(x1), 2.55], [r5(x1), r5(y1)], [r5(x0), r5(y1)]], "closed": True},
            {"id": "post-f", "bone": "root-plate", "role": "body", "z": 3, "filled": False, "tone": 0.3, "kind": "poly", "points": [[1.43, 0], [1.57, 0], [1.57, 2.55], [1.43, 2.55]], "closed": True},
            {"id": "post-b", "bone": "root-plate", "role": "body", "z": 3, "filled": False, "tone": 0.3, "kind": "poly", "points": [[-1.57, 0], [-1.43, 0], [-1.43, 2.55], [-1.57, 2.55]], "closed": True}]
    front = [{"id": "frame", "bone": "root-plate", "role": "body", "z": 2, "filled": False, "tone": 0.5, "kind": "poly", "points": [[r5(z0), 2.55], [r5(z1), 2.55], [r5(z1), r5(y1)], [r5(z0), r5(y1)]], "closed": True},
             {"id": "post-l", "bone": "root-plate", "role": "body", "z": 3, "filled": False, "tone": 0.3, "kind": "poly", "points": [[0.93, 0], [1.07, 0], [1.07, 2.55], [0.93, 2.55]], "closed": True},
             {"id": "post-r", "bone": "root-plate", "role": "body", "z": 3, "filled": False, "tone": 0.3, "kind": "poly", "points": [[-1.07, 0], [-0.93, 0], [-0.93, 2.55], [-1.07, 2.55]], "closed": True}]
    return [{"id": "wisteria-pergola-symbol", "title": T("Plan symbol of the pergola", "Grundrisssymbol der Pergola"), "description": T("A sign for garden and landscape plans: the roof outline 3.6 m by 2.8 m, the four posts and the trunk; a separate symbol, never used silently for a pictorial scale.", "Ein Zeichen für Garten- und Freiraumpläne: der Dachumriss von 3,6 m mal 2,8 m, die vier Pfosten und der Stamm; ein eigenes Symbol, nie stillschweigend für einen Bildmaßstab verwendet."),
             "semantics": "true-extent", "units": "model-m", "denominators": [500, 200, 100], "pens": pens(0.18, 0.13, None, None, 0.25), "hatches": [], "geometry": {"elevation": elev, "front": front, "plan": plan},
             "dimensionalNote": T("Drawn at true extent in model metres in all three views: the roof outline is the extent of beams, rafters and canopy, posts and trunk at their real positions and sections.", "In wahrer Ausdehnung in Modellmetern in allen drei Ansichten gezeichnet: der Dachumriss ist die Ausdehnung von Pfetten, Sparren und Laubdach, Pfosten und Stamm in wirklicher Lage und Stärke.")}]


def main():
    reps, vols = {}, {}
    for d in SCALES:
        sc = SCALE_BUILD[d]()
        thermal, structure = thermal_and_structure(sc, [outlet(d, "transpiration-leaves", "canopy-2", 0.35, LEAF, "leaves")], support_blocks(d, SUPPORTS, lambda bone: TIMBER if bone.startswith("post") else WOOD))
        reps[str(d)] = rep(sc, *RANGES[d], PENS[d], DEC[d])
        vols[str(d)] = {"status": "authored", "denominator": d, "body": sc.body, "thermal": thermal, "structure": structure, "decisions": VOLUME_DECISIONS[d]}
        ext = [r5(hi - lo) for lo, hi in sc.extent]
        box = sc.extent
        print(f"[DEBUG] 1:{d} extent {ext} lo {[round(lo, 3) for lo, _ in sc.extent]} solids {len(sc.body)}")
    mass = round(sum(b["mass"]["kg"] for b in rig.bones if "mass" in b), 2)
    asset = {
        "id": ID, "category": "vegetation", "rig": {"kind": "botanical", "root": "root-plate", "bones": rig.bones}, "anchors": ANCHORS,
        "title": T("Wisteria on Pergola", "Glyzinie an der Pergola"),
        "summary": T("An old Chinese wisteria (Wisteria sinensis) in full flower on a freestanding timber pergola 3.6 m by 2.8 m: four C24 posts of 140 mm, two beams of 80 by 200 mm at 2.55 m and six rafters of 60 by 160 mm. The trunk of two stems twines anticlockwise up beside the front right post, two leaders run over the rafters, and a mat of young pinnate leaves carries 13 clusters of lilac racemes that hang between the rafters. The pergola is passive structure; plant and pergola are one asset.",
                     "Eine alte Chinesische Glyzinie (Wisteria sinensis) in voller Blüte an einer freistehenden Holzpergola von 3,6 m mal 2,8 m: vier Pfosten C24 von 140 mm, zwei Pfetten von 80 mal 200 mm in 2,55 m Höhe und sechs Sparren von 60 mal 160 mm. Der Stamm aus zwei Trieben windet sich links herum neben dem vorderen rechten Pfosten empor, zwei Leittriebe laufen über die Sparren, und eine Decke junger Fiederblätter trägt 13 Büschel lilafarbener Trauben, die zwischen den Sparren hängen. Die Pergola ist passive Struktur; Pflanze und Pergola sind ein Asset."),
        "tags": ["wisteria", "climber", "pergola", "timber", "flowering", "deciduous", "wind", "transpiration"],
        "provenance": {"author": "Semio staffage plant worker PW-16", "method": "handcrafted", "created": "2026-11-22", "references": [{"id": k, "citation": v} for k, v in REFS.items()],
                       "notes": T("Habit after Valder (1995) and the Flora of China: anticlockwise twining, racemes of 15 to 30 cm that open together before the leaves are full grown. Timber after EN 338 (C24, 420 kg/m3). Plant masses (green wood 800 kg/m3, 6 kg of twigs and leaves per canopy section, 0.15 kg per raceme cluster), leaf area (18 m2) and conductance are authored estimates guided by these sources and Jones (2014).",
                                  "Wuchs nach Valder (1995) und der Flora of China: Linkswinden, Trauben von 15 bis 30 cm, die sich gemeinsam öffnen, bevor die Blätter ausgewachsen sind. Holz nach EN 338 (C24, 420 kg/m3). Pflanzenmassen (frisches Holz 800 kg/m3, 6 kg Zweige und Blätter je Laubdachabschnitt, 0,15 kg je Traubenbüschel), Blattfläche (18 m2) und Leitfähigkeit sind Autorenschätzungen angelehnt an diese Quellen und Jones (2014).")},
        "dimensions": {"heightM": ext[1], "widthM": ext[0], "depthM": ext[2], "massKg": mass},
        "materials": MATERIALS, "representations": reps, "symbols": symbol(box), "poses": POSES, "clips": clips(),
        "thermal": {"environment": {"airTempC": 20, "meanRadiantTempC": 23, "relativeHumidity": 0.55, "windMS": 1.0, "windDirectionDeg": 0, "solarWM2": 700, "diffuseFraction": 0.2, "pressurePa": 101325}, "passengers": [], "operating": [], "regions": REGIONS, "emitters": EMITTERS,
                    "assumptions": [T("The leaf region uses the one-sided area of the young leaves in May (18 m2); its leaf area index of 1.8 refers to the roof of 10 m2. The maximum stomatal conductance of 4 mm/s is an authored estimate for a C3 climber with expanding leaves (Jones 2014).", "Die Blattregion verwendet die einseitige Fläche der jungen Blätter im Mai (18 m2); ihr Blattflächenindex von 1,8 bezieht sich auf das Dach von 10 m2. Die maximale Spaltöffnungsleitfähigkeit von 4 mm/s ist eine Autorenschätzung für einen C3-Kletterer mit sich entfaltenden Blättern (Jones 2014)."),
                                    T("Racemes, wood and timber are passive regions: areal heat capacities from volumetric heat capacity times an effective depth (petals 1 mm, wisteria wood 2.0 MJ/(m3 K) x 15 mm, C24 0.67 MJ/(m3 K) x 10 mm). No surface temperature is authored.", "Trauben, Holz und Bauholz sind passive Regionen: flächenbezogene Wärmekapazitäten aus volumetrischer Wärmekapazität mal wirksamer Tiefe (Blütenblätter 1 mm, Glyzinienholz 2,0 MJ/(m3 K) x 15 mm, C24 0,67 MJ/(m3 K) x 10 mm). Es werden keine Oberflächentemperaturen vorgegeben.")]},
        "structure": {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": [a for a, _ in SUPPORTS], "assumptions": [T("One rigid body per evaluated pose, fixed at the four post feet and at the root crown. Wind drag acts on each canopy section (cf 1.0 on 0.35 m2), each raceme cluster (cf 1.2 on 0.03 m2) and each post (cf 2.0 for a square section on 0.36 m2), EN 1991-1-4 form; gravity on every bone mass. The pergola is C24 timber (posts 140 x 140 mm, beams 80 x 200 mm); joints are idealised as rigid, dynamic amplification is not modelled.", "Ein starrer Körper je ausgewerteter Pose, an den vier Pfostenfüßen und am Wurzelhals eingespannt. Winddruck wirkt auf jeden Laubdachabschnitt (cf 1,0 auf 0,35 m2), jedes Traubenbüschel (cf 1,2 auf 0,03 m2) und jeden Pfosten (cf 2,0 für einen Vierkantquerschnitt auf 0,36 m2), Form nach EN 1991-1-4; Schwerkraft auf jede Knochenmasse. Die Pergola ist Bauholz C24 (Pfosten 140 x 140 mm, Pfetten 80 x 200 mm); Anschlüsse sind starr idealisiert, dynamische Überhöhung wird nicht modelliert.")], "clinical": False},
        "volumes": vols,
        "presentation": {"defaultPose": "flowering-calm", "defaultClip": "wind-sway", "defaultScale": 100},
        "exports": EXPORTS, "completion": "authored",
    }
    print(write(asset, ID))
    manifest_row(asset, T("An old Chinese wisteria (Wisteria sinensis) in full flower on a freestanding C24 timber pergola, 3.6 m by 2.8 m and 3.1 m high: a twined trunk beside one post, leaders over six rafters and lilac racemes hanging between them. The pergola is passive structure.", "Eine alte Chinesische Glyzinie (Wisteria sinensis) in voller Blüte an einer freistehenden Holzpergola aus C24, 3,6 m mal 2,8 m und 3,1 m hoch: ein verwundener Stamm neben einem Pfosten, Leittriebe über sechs Sparren und lilafarbene Trauben, die zwischen ihnen hängen. Die Pergola ist passive Struktur."))


if __name__ == "__main__":
    main()
