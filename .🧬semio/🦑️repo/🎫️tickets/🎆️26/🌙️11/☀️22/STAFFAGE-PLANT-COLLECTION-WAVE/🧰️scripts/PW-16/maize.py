"""🌽️ maize-row: Zea mays at tasseling, six plants 0.2 m apart on an earthed-up ridge, 2.5 m high; C4 leaves, ears in husk with silk, branched tassels.

Run: `python3 maize.py` writes `🗂️catalog/🌳️vegetation/maize-row/🔣️.json` and the manifest row. Every number below is designed here per scale.
@see ./pw16lib.py
"""
import math
from pw16lib import *  # noqa: F401,F403

ID = "maize-row"
PLANTS = [-0.5, -0.3, -0.1, 0.1, 0.3, 0.5]
PHI = [10, 75, -15, 105, 25, -80]
LO, HI, TASSEL = 1.2, 0.95, 0.35
LEAVES = [  # height on the stalk, chord length, chord elevation (deg), arch, blade width at the collar
    (0.32, 0.55, -5, 0.10, 0.075), (0.52, 0.70, 0, 0.13, 0.085), (0.72, 0.80, 5, 0.15, 0.09), (0.95, 0.85, 8, 0.16, 0.095),
    (1.20, 0.82, 14, 0.15, 0.092), (1.45, 0.75, 20, 0.14, 0.088), (1.70, 0.62, 28, 0.12, 0.08), (1.93, 0.48, 38, 0.10, 0.08)]
EAR_Y, EAR_L = 0.98, 0.27
STALK, LEAF, EAR, TAS, SOIL = "maize-stalk", "maize-leaf", "maize-husk", "maize-tassel", "ridge-soil"

rig = Rig()
rig.add("root-plate", None, T("Row line on the ground", "Zeilenlinie am Boden"), 0, 0, 0, 0, FIXED)
a, b, t = orient(rig, "root-plate", [0, 0, 1], up=(0, 1, 0))
rig.add("ridge", "root-plate", T("Earthed-up ridge of the row", "Angehäufelter Damm der Reihe"), 0, 0, a, 0.4, FIXED, abduction=b, twist=t)
for p, (x, phi) in enumerate(zip(PLANTS, PHI), 1):
    rig.add(f"stalk-{p}-lo", "root-plate", T(f"Lower stalk of plant {p}", f"Unterer Halm von Pflanze {p}"), x, 0, 90, LO, revolute(-8, 8), mass=(0.38, "authored-estimate", "ritchie-1993"))
    rig.add(f"stalk-{p}-hi", f"stalk-{p}-lo", T(f"Upper stalk of plant {p}", f"Oberer Halm von Pflanze {p}"), LO, 0, 0, HI, revolute(-12, 12), mass=(0.16, "authored-estimate", "ritchie-1993"))
    rig.add(f"tassel-{p}", f"stalk-{p}-hi", T(f"Tassel of plant {p}", f"Rispe von Pflanze {p}"), HI, 0, 0, TASSEL, revolute(-20, 20), mass=(0.02, "authored-estimate", "ritchie-1993"))
    for k, (h, L, e, sag, wid) in enumerate(LEAVES, 1):
        parent = f"stalk-{p}-lo" if h < LO else f"stalk-{p}-hi"
        u0 = h if h < LO else h - LO
        az = phi if k % 2 else phi + 180
        a, b, t = orient(rig, parent, direction(az, e), roll=18 if k % 2 else -18)
        rig.add(f"leaf-{p}-{k}", parent, T(f"Leaf {k} of plant {p}", f"Blatt {k} von Pflanze {p}"), u0, 0, a, L, revolute(-15, 15), abduction=b, twist=t, mass=(0.012 + 0.004 * L * 10, "authored-estimate", "ritchie-1993"))
    a, b, t = orient(rig, f"stalk-{p}-lo", direction(phi + 90, 60))
    rig.add(f"ear-{p}", f"stalk-{p}-lo", T(f"Ear in husk of plant {p}", f"Kolben in Lieschen von Pflanze {p}"), EAR_Y, 0, a, EAR_L, revolute(-60, 15), abduction=b, twist=t, mass=(0.25, "authored-estimate", "ritchie-1993", (0.13, 0)))


def leaf_profile(L, sag, th, n):
    spine = arc((0, 0), (L, 0), sag, n)
    return band(spine, lambda s: th / 2 * (1 - 0.55 * s))


def leaves(sc, keep, th, n):
    for p in range(1, 7):
        for k in keep:
            h, L, e, sag, wid = LEAVES[k - 1]
            sc.plate(f"leaf-{p}-{k}", f"leaf-{p}-{k}", leaf_profile(L, sag, th, n), -wid / 2, wid / 2, region="leaves", material=LEAF, tone=0.35 + 0.03 * ((p + k) % 5), tip=(-0.004, 0.004))


def ridge(sc, profile):
    sc.plate("ridge", "ridge", profile, -0.7, 0.7, region="ridge-soil", material=SOIL, tone=0.7)


def stalks(sc, split, d_lo, d_hi):
    for p in range(1, 7):
        lo, hi = f"stalk-{p}-lo", f"stalk-{p}-hi"
        if split:
            sc.rod(f"{lo}-base", lo, [0, 0, 0], [0.5, 0, 0], d_lo * 1.1, region="stalks", material=STALK, tone=0.5)
            sc.rod(f"{lo}-mid", lo, [0.48, 0, 0], [LO + 0.02, 0, 0], d_lo, region="stalks", material=STALK, tone=0.52)
        else:
            sc.rod(f"{lo}-stalk", lo, [0, 0, 0], [LO + 0.02, 0, 0], d_lo, region="stalks", material=STALK, tone=0.5)
        sc.rod(f"{hi}-stalk", hi, [0, 0, 0], [HI + 0.01, 0, 0], d_hi, region="stalks", material=STALK, tone=0.55)


def ears(sc, parts):
    for p in range(1, 7):
        for j, (u0, u1, d, region) in enumerate(parts):
            sc.rod(f"ear-{p}-{j}", f"ear-{p}", [u0, 0, 0], [u1, 0, 0], d, region=region, material=EAR if region == "ears" else STALK, tone=0.3 if j == 1 else 0.42)


def tassels_branched(sc, branches, d_axis, d_branch):
    for p in range(1, 7):
        b = f"tassel-{p}"
        sc.rod(f"{b}-spike", b, [0, 0, 0], [TASSEL, 0, 0], d_axis, region="tassels", material=TAS, tone=0.62)
        for j, (u, du, dv, dw) in enumerate(branches):
            sc.rod(f"{b}-branch-{j}", b, [u, 0, 0], [u + du, dv, dw], d_branch, region="tassels", material=TAS, tone=0.66)


RIDGE_50 = [(-0.21, 0.0), (-0.16, 0.05), (-0.09, 0.09), (0.0, 0.1), (0.09, 0.09), (0.16, 0.05), (0.21, 0.0)]
BRANCHES_50 = [(0.06, 0.12, 0.10, 0.03), (0.07, 0.13, -0.1, -0.03), (0.09, 0.12, 0.02, 0.1), (0.10, 0.12, -0.02, -0.1), (0.12, 0.1, 0.07, -0.07), (0.13, 0.1, -0.07, 0.07)]


def scale50():
    sc = Scale(rig, 50)
    ridge(sc, RIDGE_50)
    stalks(sc, True, 0.028, 0.02)
    for p in range(1, 7):
        for k, (h, *_r) in enumerate(LEAVES, 1):
            if k % 2 == 0:
                bone, u = (f"stalk-{p}-lo", h) if h < LO else (f"stalk-{p}-hi", h - LO)
                sc.ell(f"node-{p}-{k}", bone, [u, 0, 0], (0.012, 0.019, 0.019), region="stalks", material=STALK, tone=0.45)
        for j, (v, w) in enumerate([(0.05, 0.0), (-0.035, 0.04), (-0.035, -0.04)]):
            sc.rod(f"brace-{p}-{j}", f"stalk-{p}-lo", [0.09, 0, 0], [0.006, v, w], 0.009, region="stalks", material=STALK, tone=0.48)
    leaves(sc, range(1, 9), 0.011, 11)
    ears(sc, [(0.0, 0.05, 0.02, "stalks"), (0.04, 0.22, 0.052, "ears"), (0.21, 0.26, 0.028, "ears"), (0.25, EAR_L, 0.012, "ears")])
    tassels_branched(sc, BRANCHES_50, 0.008, 0.0045)
    return sc


def scale100():
    sc = Scale(rig, 100)
    ridge(sc, [(-0.21, 0.0), (-0.12, 0.08), (0.0, 0.1), (0.12, 0.08), (0.21, 0.0)])
    stalks(sc, False, 0.028, 0.02)
    leaves(sc, range(1, 9), 0.013, 8)
    ears(sc, [(0.0, 0.05, 0.022, "stalks"), (0.04, 0.23, 0.054, "ears"), (0.22, EAR_L, 0.026, "ears")])
    tassels_branched(sc, BRANCHES_50[:4], 0.009, 0.005)
    return sc


def scale200():
    sc = Scale(rig, 200)
    ridge(sc, [(-0.21, 0.0), (-0.1, 0.09), (0.1, 0.09), (0.21, 0.0)])
    stalks(sc, False, 0.03, 0.022)
    leaves(sc, range(1, 9), 0.016, 6)
    ears(sc, [(0.02, EAR_L, 0.055, "ears")])
    tassels_branched(sc, [(0.06, 0.13, 0.09, 0.0), (0.08, 0.13, -0.09, 0.0)], 0.01, 0.006)
    return sc


def scale500():
    sc = Scale(rig, 500)
    ridge(sc, [(-0.21, 0.0), (0.0, 0.1), (0.21, 0.0)])
    stalks(sc, False, 0.03, 0.024)
    leaves(sc, range(1, 9), 0.02, 4)
    ears(sc, [(0.04, EAR_L - 0.02, 0.076, "ears")])
    for p in range(1, 7):
        sc.rod(f"tassel-{p}", f"tassel-{p}", [0, 0, 0], [TASSEL, 0, 0], 0.08, region="tassels", material=TAS, tone=0.62)
    return sc


SUPPORTS = [(f"root-{p}", f"stalk-{p}-lo") for p in range(1, 7)]
SCALE_BUILD = {50: scale50, 100: scale100, 200: scale200, 500: scale500}
PENS = {50: pens(0.25, 0.18, 0.1, None, 0.35), 100: pens(0.2, 0.13, None, None, 0.35), 200: pens(0.18, 0.13, None, None, 0.25), 500: pens(0.18, 0.1, None, None, 0.25)}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}
TP_LEAVES = [(p, k) for p in range(1, 7) for k in (3, 6)]

DEC = {
    50: decisions(
        T("At 1:50 (1 m is 20 mm) the row stands 50 mm high on its ridge: six stalks with brace roots and nodes, eight arching strap leaves per plant in alternating ranks, one ear in husk with its silk at 1 m and a tassel of a spike and six branches.", "Bei 1:50 (1 m sind 20 mm) steht die Reihe 50 mm hoch auf ihrem Damm: sechs Halme mit Stützwurzeln und Knoten, acht überhängende Bandblätter je Pflanze in wechselnden Zeilen, ein Kolben in Lieschen mit seinen Narbenfäden in 1 m Höhe und eine Rispe aus einer Ähre und sechs Ästen."),
        T("Retained: true leaf count, arch, taper and ranking; the brace roots, stalk nodes, the ear shank, husk tip and silk; the tassel branches.", "Beibehalten: wahre Blattzahl, Bogen, Verjüngung und Zeilenstellung; die Stützwurzeln, Halmknoten, der Kolbenstiel, die Lieschenspitze und die Narbenfäden; die Rispenäste."),
        T("Merged: leaf sheaths into the stalk; the wavy blade margin into a straight taper.", "Zusammengefasst: Blattscheiden in den Halm; der gewellte Blattrand in eine gerade Verjüngung."),
        T("Omitted: the leaf midrib line, ligules and the small leaves below 0.3 m.", "Weggelassen: die Mittelrippenlinie, Blatthäutchen und die kleinen Blätter unter 0,3 m."),
        T("Tassel branches (4.5 mm) are true members; the leaves are plates of their true width seen at their true angle.", "Rispenäste (4,5 mm) sind wahre Stäbe; die Blätter sind Platten ihrer wahren Breite in ihrem wahren Winkel."),
        T("None.", "Keine."),
        T("Leaves in five greys so the ranks separate; dark ears; light tassels and ridge.", "Blätter in fünf Grautönen, damit sich die Zeilen trennen; dunkle Kolben; helle Rispen und Damm."),
        T("Leaves carry the leaf region, stalks, nodes and brace roots the stalk region, ears the husk region, tassels and ridge their own.", "Blätter tragen die Blattregion, Halme, Knoten und Stützwurzeln die Halmregion, Kolben die Lieschenregion, Rispen und Damm ihre eigenen.")),
    100: decisions(
        T("At 1:100 (1 m is 10 mm) the row is 25 mm high: six stalks, eight leaves per plant with a coarser arch, the ear with its husk tip and a tassel of four branches.", "Bei 1:100 (1 m sind 10 mm) ist die Reihe 25 mm hoch: sechs Halme, acht Blätter je Pflanze mit gröberem Bogen, der Kolben mit seiner Lieschenspitze und eine Rispe aus vier Ästen."),
        T("Retained: all leaves, the ear angle, the tassel silhouette, the ridge.", "Beibehalten: alle Blätter, der Kolbenwinkel, der Umriss der Rispe, der Damm."),
        T("Merged: the two stalk courses into one tube per bone; silk into the husk tip.", "Zusammengefasst: die zwei Halmabschnitte zu einem Rohr je Knochen; Narbenfäden in die Lieschenspitze."),
        T("Omitted: nodes, brace roots, two tassel branches.", "Weggelassen: Knoten, Stützwurzeln, zwei Rispenäste."),
        T("Stalks of 20 to 28 mm print at 0.2 to 0.28 mm plus pen.", "Halme von 20 bis 28 mm drucken mit 0,2 bis 0,28 mm plus Stift."),
        T("None.", "Keine."),
        T("Five leaf greys, dark ears.", "Fünf Blattgrautöne, dunkle Kolben."),
        T("As at 1:50.", "Wie bei 1:50.")),
    200: decisions(
        T("At 1:200 (1 m is 5 mm) the row is 12.5 mm high: stalks, eight leaves per plant drawn as arcs of five chords, one ear tube and a two-branched tassel.", "Bei 1:200 (1 m sind 5 mm) ist die Reihe 12,5 mm hoch: Halme, acht Blätter je Pflanze als Bögen aus fünf Sehnen, ein Kolbenrohr und eine zweiästige Rispe."),
        T("Retained: the ranked leaf fountain, the ear, the tassel tips, true height and spread.", "Beibehalten: der gestaffelte Blattbrunnen, der Kolben, die Rispenspitzen, wahre Höhe und Ausladung."),
        T("Merged: ear, shank and husk tip into one tube.", "Zusammengefasst: Kolben, Stiel und Lieschenspitze zu einem Rohr."),
        T("Omitted: four tassel branches, the leaf taper of the band.", "Weggelassen: vier Rispenäste, die Verjüngung des Blattbandes."),
        T("Leaves keep their true width (0.09 m is 0.45 mm); the band of an edge-on leaf is 16 mm thick.", "Blätter behalten ihre wahre Breite (0,09 m sind 0,45 mm); das Band eines hochkant gesehenen Blattes ist 16 mm dick."),
        T("None.", "Keine."),
        T("Five leaf greys.", "Fünf Blattgrautöne."),
        T("As at 1:50.", "Wie bei 1:50.")),
    500: decisions(
        T("At 1:500 (1 m is 2 mm) the row is a 5 mm tuft: six stalks with eight leaves each drawn as arcs of three chords, an ear and a spindle tassel.", "Bei 1:500 (1 m sind 2 mm) ist die Reihe ein Schopf von 5 mm: sechs Halme mit je acht Blättern als Bögen aus drei Sehnen, ein Kolben und eine spindelförmige Rispe."),
        T("Retained: the leaf count, the true spread and height.", "Beibehalten: die Blattzahl, die wahre Ausladung und Höhe."),
        T("Merged: the tassel into one spindle, the ear with its husk into one 76 mm tube.", "Zusammengefasst: die Rispe zu einer Spindel, der Kolben mit seinen Lieschen zu einem Rohr von 76 mm."),
        T("Omitted: arch detail, tassel branches.", "Weggelassen: Bogendetails, Rispenäste."),
        T("Stalks are true 24 to 30 mm members (0.05 mm plus pen); the ear is thickened to the 75 mm feature.", "Halme sind wahre Stäbe von 24 bis 30 mm (0,05 mm plus Stift); der Kolben ist auf das Merkmal von 75 mm verdickt."),
        T("None.", "Keine."),
        T("Leaves in five greys, the ridge light.", "Blätter in fünf Grautönen, der Damm hell."),
        T("Leaves carry the leaf region; stalks, ears, tassels and ridge their own.", "Blätter tragen die Blattregion; Halme, Kolben, Rispen und Damm ihre eigenen.")),
}

VOLUME_DECISIONS = {
    50: [T("Body at 1:50: the ridge as a prism of its rounded section, stalks as tubes in two courses with node collars and three brace roots, 48 leaves as tapered arching plates (polyhedra whose width runs from the collar to a 8 mm tip), ears as shank, ear, husk tip and silk, tassels as a spike and six branches.", "Körper bei 1:50: der Damm als Prisma seines gerundeten Querschnitts, Halme als Rohre in zwei Abschnitten mit Knotenwülsten und drei Stützwurzeln, 48 Blätter als verjüngte, gebogene Platten (Polyeder, deren Breite vom Blattgrund auf eine Spitze von 8 mm abnimmt), Kolben aus Stiel, Kolben, Lieschenspitze und Narbenfäden, Rispen aus einer Ähre und sechs Ästen."),
         T("Thermal: every body solid by reference in its region, one transpiration outlet on a leaf of plant 3. Structure: one member per bone by reference (every tube its own), a support block at each stalk foot.", "Thermik: jeder Körper per Verweis in seiner Region, ein Transpirationsauslass an einem Blatt von Pflanze 3. Tragwerk: ein Stab je Knochen per Verweis (jedes Rohr ein eigener), ein Auflagerblock an jedem Halmfuß.")],
    100: [T("Body at 1:100: one tube per stalk bone, eight coarser leaves per plant, ear of three parts, four tassel branches.", "Körper bei 1:100: ein Rohr je Halmknochen, acht gröbere Blätter je Pflanze, Kolben aus drei Teilen, vier Rispenäste."), T("Thermal and structure by reference.", "Thermik und Tragwerk per Verweis.")],
    200: [T("Body at 1:200: eight leaves per plant of five chords and 16 mm band, one ear tube, two tassel branches.", "Körper bei 1:200: acht Blätter je Pflanze aus fünf Sehnen und 16 mm Band, ein Kolbenrohr, zwei Rispenäste."), T("Thermal and structure by reference.", "Thermik und Tragwerk per Verweis.")],
    500: [T("Body at 1:500: eight leaves per plant of three chords and 20 mm band, ear tube of 76 mm, spindle tassels as 80 mm tubes.", "Körper bei 1:500: acht Blätter je Pflanze aus drei Sehnen und 20 mm Band, Kolbenrohr von 76 mm, Rispen als Rohre von 80 mm."), T("Thermal and structure by reference; supports of 80 mm.", "Thermik und Tragwerk per Verweis; Auflager von 80 mm.")],
}

REFS = {
    "ritchie-1993": "Ritchie, S. W., Hanway, J. J. and Benson, G. O. (1993). How a Corn Plant Develops. Special Report 48, Iowa State University of Science and Technology, Ames. Leaf number, plant height, tasseling (VT) and silking (R1), ear position.",
    "fao-ecocrop-maize": "FAO (2017). Crop Information: Maize. FAO Land and Water Division, Rome. Plant density 5 to 8 plants per m2 at 0.75 m row spacing, height 1.5 to 3 m.",
    "jones-2014": "Jones, H. G. (2014). Plants and Microclimate, 3rd ed., Cambridge University Press. C4 photosynthesis, leaf energy balance, boundary-layer and stomatal conductance.",
    "monteith-unsworth-2013": "Monteith, J. L. and Unsworth, M. H. (2013). Principles of Environmental Physics, 4th ed., Academic Press. Penman-Monteith leaf transpiration.",
    "en-1991-1-4": "EN 1991-1-4:2005 Eurocode 1, Actions on structures, Part 1-4: Wind actions. Dynamic pressure q = 0.5 rho v^2 with air density 1.25 kg/m3, force F = cf q A, peak velocity pressure (1 + 7 Iv) q_m.",
    "hillel-1998": "Hillel, D. (1998). Environmental Soil Physics. Academic Press, San Diego. Bulk density, heat capacity and thermal conductivity of moist loam.",
}

MATERIALS = [
    {"id": LEAF, "title": T("Maize leaf blade", "Maisblattspreite"), "emissivity": 0.96, "solarAbsorptivity": 0.5, "source": "authored-estimate", "reference": "jones-2014"},
    {"id": STALK, "title": T("Green maize stalk with pith", "Grüner Maishalm mit Mark"), "emissivity": 0.95, "solarAbsorptivity": 0.55, "densityKgM3": 800, "conductivityWmK": 0.35, "specificHeatJKgK": 3500, "elasticModulusPa": 1500000000, "strengthPa": 20000000, "source": "authored-estimate", "reference": "ritchie-1993"},
    {"id": EAR, "title": T("Green husk leaves around the ear", "Grüne Lieschenblätter um den Kolben"), "emissivity": 0.95, "solarAbsorptivity": 0.5, "densityKgM3": 900, "conductivityWmK": 0.4, "specificHeatJKgK": 3600, "source": "authored-estimate", "reference": "ritchie-1993"},
    {"id": TAS, "title": T("Tassel spikelets with anthers", "Rispenährchen mit Staubbeuteln"), "emissivity": 0.94, "solarAbsorptivity": 0.45, "densityKgM3": 400, "source": "authored-estimate", "reference": "ritchie-1993"},
    {"id": SOIL, "title": T("Moist loam of the ridge", "Feuchter Lehm des Damms"), "emissivity": 0.95, "solarAbsorptivity": 0.75, "densityKgM3": 1350, "conductivityWmK": 1.0, "specificHeatJKgK": 1500, "source": "literature", "reference": "hillel-1998"},
]

REGIONS = [
    leaf_region("leaves", T("Maize leaves", "Maisblätter"), LEAF, 3.6, 0.08, "horizontal-plate-up", 0.6, 500, 0.0055, 1.6, "c4", "jones-2014"),
    passive_region("stalks", T("Stalks, nodes and brace roots", "Halme, Knoten und Stützwurzeln"), "stem", STALK, 1.1, 0.025, "cylinder", 0.4, 22000, "ritchie-1993"),
    passive_region("ears", T("Ears in husk", "Kolben in Lieschen"), "surface", EAR, 0.25, 0.05, "cylinder", 0.3, 45000, "ritchie-1993"),
    passive_region("tassels", T("Tassels", "Rispen"), "surface", TAS, 0.12, 0.005, "cylinder", 0.8, 3000, "ritchie-1993"),
    passive_region("ridge-soil", T("Ridge soil", "Dammerde"), "surface", SOIL, 1.32, 0.3, "horizontal-plate-up", 0.7, 100000, "hillel-1998"),
]
REGIONS[0]["areaBasis"] = "exchange"

ANCHORS = [anchors_support(f"stalk-{p}-lo", T(f"Root crown of plant {p} in the ridge", f"Wurzelhals von Pflanze {p} im Damm"), 0.05, 0.05, id=f"root-{p}") for p in range(1, 7)]
TP = []
for p, k in TP_LEAVES:
    aid = f"tp-{p}-{k}"
    TP.append(aid)
    ANCHORS.append(emitter_anchor(aid, f"leaf-{p}-{k}", LEAVES[k - 1][1] * 0.45, 0, T(f"Transpiration of leaf {k} of plant {p}", f"Transpiration von Blatt {k} der Pflanze {p}")))
EMITTERS = [transpiration("transpiration-leaves", T("Transpiration of the leaves (distributed over twelve anchors)", "Transpiration der Blätter (über zwölf Anker verteilt)"), TP, "leaves", 0, 0.4)]


def loads(v, extra=None):
    out = []
    for p in range(1, 7):
        out.append(wind_load(f"wind-foliage-{p}", T(f"Wind drag on the leaves of plant {p}", f"Winddruck auf die Blätter von Pflanze {p}"), f"stalk-{p}-hi", 0.3, 0, 0.6, 0.4, v))
        out.append(wind_load(f"wind-tassel-{p}", T(f"Wind drag on the tassel of plant {p}", f"Winddruck auf die Rispe von Pflanze {p}"), f"tassel-{p}", 0.2, 0, 1.0, 0.03, v))
    return out + (extra or [])


def binding(leaf_sun, emitter, ear_sun=0.15):
    return bind({"leaves": (1, leaf_sun), "stalks": (1, 0.2), "ears": (1, ear_sun), "tassels": (1, 0.85), "ridge-soil": (1, 0.3)}, {"transpiration-leaves": emitter})


def offsets(lean, flutter, ear=0.0):
    out = {}
    for p in range(1, 7):
        out[f"stalk-{p}-lo"] = {"angleDeg": r5(lean * 0.35)}
        out[f"stalk-{p}-hi"] = {"angleDeg": r5(lean * 0.65)}
        out[f"tassel-{p}"] = {"angleDeg": r5(lean * 1.4)}
        for k in range(1, 9):
            out[f"leaf-{p}-{k}"] = {"angleDeg": r5(flutter * (1 if (p + k) % 2 else -0.7))}
        if ear:
            out[f"ear-{p}"] = {"angleDeg": ear}
    return out


RIPE = [{"id": f"ripening-{p}", "kind": "carried", "title": T(f"Grain fill of the ear of plant {p}", f"Kornfüllung des Kolbens von Pflanze {p}"), "bone": f"ear-{p}", "x": 0.13, "y": 0, "fxN": 0, "fyN": -1.96, "source": "authored-estimate", "reference": "ritchie-1993"} for p in range(1, 7)]
POSES = [
    make_pose("tasseling-calm", T("Calm day at tasseling", "Windstiller Tag zur Rispenblüte"), T("Light air of 1.5 m/s in July: the stalks stand upright, the leaves fan out in their alternating ranks, the tassels shed pollen above the silks of the ears.", "Leiser Zug von 1,5 m/s im Juli: die Halme stehen aufrecht, die Blätter fächern sich in ihren wechselnden Zeilen, die Rispen stäuben über den Narbenfäden der Kolben."), {}, binding(0.6, 0.9), loads(1.5)),
    make_pose("windy", T("Fresh breeze", "Frische Brise"), T("7 m/s at tassel height (Beaufort 4): about 7 N on the leaves of each plant (cf 0.6 on 0.4 m2); the stalks bend 3 degrees, the tassels swing further and the leaves flutter by 4 degrees.", "7 m/s in Rispenhöhe (Beaufort 4): etwa 7 N auf die Blätter jeder Pflanze (cf 0,6 auf 0,4 m2); die Halme biegen sich 3 Grad, die Rispen schwingen weiter und die Blätter flattern um 4 Grad."), offsets(-3, 4), binding(0.6, 1.0), loads(7)),
    make_pose("storm", T("Summer storm", "Sommergewitter"), T("18 m/s gusts under a dark sky: about 50 N per plant, the stalks bend 10 degrees towards lodging, the tassels whip and the leaves stream; the stomata close partly.", "Böen von 18 m/s unter dunklem Himmel: etwa 50 N je Pflanze, die Halme biegen sich 10 Grad zum Lagern, die Rispen peitschen und die Blätter flattern; die Spaltöffnungen schließen sich teilweise."), offsets(-10, 12), binding(0.1, 0.4, 0.05), loads(18)),
    make_pose("ripe-ears", T("Ripe ears in late summer", "Reife Kolben im Spätsommer"), T("Calm September day at dent stage: grain fill has added 0.2 kg to every ear (carried load of 1.96 N), the ears tilt 25 degrees down from the stalk, the lower leaves senesce and transpiration falls to 60 %.", "Windstiller Septembertag zur Zahnreife: die Kornfüllung hat jedem Kolben 0,2 kg zugefügt (getragene Last 1,96 N), die Kolben neigen sich 25 Grad vom Halm nach unten, die unteren Blätter vergilben und die Transpiration sinkt auf 60 %."), offsets(0, 0, -25), binding(0.65, 0.6, 0.3), loads(1.5, RIPE)),
]


def clips():
    base = {k: v["angleDeg"] for k, v in offsets(-3, 4).items()}
    specs = {}
    for p in range(1, 7):
        specs[f"stalk-{p}-lo"] = (0.6, 40 * p, 1)
        specs[f"stalk-{p}-hi"] = (1.5, 40 * p + 30, 1)
        specs[f"tassel-{p}"] = (4.0, 40 * p + 70, 1)
        for k in range(1, 9):
            specs[f"leaf-{p}-{k}"] = (3.0, 40 * p + 25 * k, 1 + k % 2)
    return [
        wind_sway_clip(rig, "windy", 2.0, base, specs, T("A deterministic 2.0 s cycle (slender stalks, estimated 0.5 Hz) about the fresh-breeze lean: the stalks rock by 2 degrees, the tassels by 4 and the leaves flutter at one and two times the stalk frequency with a phase that travels down the row.", "Ein deterministischer Zyklus von 2,0 s (schlanke Halme, geschätzt 0,5 Hz) um die Neigung der frischen Brise: die Halme wiegen um 2 Grad, die Rispen um 4 und die Blätter flattern mit der ein- und zweifachen Halmfrequenz mit einer Phase, die die Reihe entlangläuft."), "transpiration-leaves"),
        gust_clip("windy", lambda s: offsets(-10 * s, 12 * s), [ld["id"] for ld in loads(7)], T("A single 8 s gust: the wind-load tracks rise from 5 % to 2.75 times the fresh-breeze load at 2.5 s (EN 1991-1-4 peak pressure with Iv 0.25) and decay; the stalks bend up to 70 % of the storm lean and spring back, the tassels overshoot.", "Eine einzelne Böe über 8 s: die Windlast-Spuren steigen bei 2,5 s von 5 % auf das 2,75-fache der Last der frischen Brise (Spitzendruck nach EN 1991-1-4 mit Iv 0,25) und klingen ab; die Halme biegen sich bis zu 70 % der Sturmneigung und federn zurück, die Rispen schwingen nach.")),
        diurnal_clip("tasseling-calm", "transpiration-leaves", [(0, 0.02), (5, 0.03), (7, 0.3), (9, 0.75), (12, 1), (15, 0.85), (18, 0.3), (20, 0.04), (24, 0.02)], (("leaves", 0.6), ("stalks", 0.2), ("ears", 0.15), ("tassels", 0.85), ("ridge-soil", 0.3)),
                     T("A 24 h cycle compressed to 24 s (1 s = 1 h) in calm July weather: the sunlit fraction of every region and the transpiration strength follow the sun; the C4 leaves keep transpiring into the afternoon and close at night; temperatures follow from the thermal solver.", "Ein 24-h-Zyklus auf 24 s gerafft (1 s = 1 h) bei ruhigem Juliwetter: der besonnte Anteil jeder Region und die Transpirationsstärke folgen der Sonne; die C4-Blätter transpirieren bis in den Nachmittag und schließen nachts; die Temperaturen folgen aus dem thermischen Löser.")),
    ]


def symbol(ext):
    (x0, x1), (_, y1), (z0, z1) = ext
    plan = [{"id": "row", "bone": "root-plate", "role": "body", "z": 2, "filled": False, "tone": 0.5, "kind": "ellipse", "cx": r5((x0 + x1) / 2), "cy": r5((z0 + z1) / 2), "rx": r5((x1 - x0) / 2), "ry": r5((z1 - z0) / 2)},
            {"id": "ridge", "bone": "root-plate", "role": "detail", "z": 3, "filled": False, "tone": 0.6, "kind": "poly", "points": [[-0.7, -0.21], [0.7, -0.21], [0.7, 0.21], [-0.7, 0.21]], "closed": True}]
    plan += [{"id": f"stalk-{p}", "bone": "root-plate", "role": "body", "z": 4, "filled": True, "tone": 0.3, "kind": "ellipse", "cx": x, "cy": 0, "rx": 0.03, "ry": 0.03} for p, x in enumerate(PLANTS, 1)]
    elev = [{"id": "crop", "bone": "root-plate", "role": "body", "z": 2, "filled": False, "tone": 0.5, "kind": "poly", "points": [[r5(x0), 0], [r5(x1), 0], [r5(x1), 1.0], [0.6, 2.2], [0, r5(y1)], [-0.6, 2.2], [r5(x0), 1.0]], "closed": True}]
    front = [{"id": "crop", "bone": "root-plate", "role": "body", "z": 2, "filled": False, "tone": 0.5, "kind": "poly", "points": [[r5(z0), 0], [r5(z1), 0], [r5(z1), 1.0], [0.4, 2.2], [0, r5(y1)], [-0.4, 2.2], [r5(z0), 1.0]], "closed": True}]
    return [{"id": "maize-row-symbol", "title": T("Plan symbol of the maize row", "Grundrisssymbol der Maisreihe"), "description": T("A sign for planting and landscape plans: the crown outline of the six plants, the ridge and the stalk positions; a separate symbol, never used silently for a pictorial scale.", "Ein Zeichen für Pflanz- und Freiraumpläne: der Kronenumriss der sechs Pflanzen, der Damm und die Halmpositionen; ein eigenes Symbol, nie stillschweigend für einen Bildmaßstab verwendet."),
             "semantics": "true-extent", "units": "model-m", "denominators": [500, 200, 100], "pens": pens(0.18, 0.13, None, None, 0.25), "hatches": [], "geometry": {"elevation": elev, "front": front, "plan": plan},
             "dimensionalNote": T("Drawn at true extent in model metres in all three views: the outline is the leaf spread of the row, the ridge and stalks at their real positions.", "In wahrer Ausdehnung in Modellmetern in allen drei Ansichten gezeichnet: der Umriss ist die Blattausladung der Reihe, Damm und Halme in wirklicher Lage.")}]


def main():
    reps, vols = {}, {}
    for d in SCALES:
        sc = SCALE_BUILD[d]()
        thermal, structure = thermal_and_structure(sc, [outlet(d, "transpiration-leaves", "leaf-3-4", 0.4, LEAF, "leaves")], support_blocks(d, SUPPORTS, lambda bone: STALK))
        reps[str(d)] = rep(sc, *RANGES[d], PENS[d], DEC[d])
        vols[str(d)] = {"status": "authored", "denominator": d, "body": sc.body, "thermal": thermal, "structure": structure, "decisions": VOLUME_DECISIONS[d]}
        ext = [r5(hi - lo) for lo, hi in sc.extent]
        box = sc.extent
        print(f"[DEBUG] 1:{d} extent {ext} lo {[round(lo, 3) for lo, _ in sc.extent]} solids {len(sc.body)}")
    mass = round(sum(b["mass"]["kg"] for b in rig.bones if "mass" in b), 2)
    asset = {
        "id": ID, "category": "vegetation", "rig": {"kind": "botanical", "root": "root-plate", "bones": rig.bones}, "anchors": ANCHORS,
        "title": T("Maize Row", "Maisreihe"),
        "summary": T("A 1 m piece of a field row of maize (Zea mays) at tasseling: six plants 0.2 m apart on an earthed-up ridge, 2.5 m high to the tassel tips. Every plant has eight arching strap leaves in alternating ranks (the leaf planes of neighbouring plants turned against each other, as in a sown row), an ear in its husk with silk at 1 m and a branched tassel. A C4 crop.",
                     "Ein Stück von 1 m einer Feldreihe Mais (Zea mays) zur Rispenblüte: sechs Pflanzen im Abstand von 0,2 m auf einem angehäufelten Damm, 2,5 m hoch bis zu den Rispenspitzen. Jede Pflanze hat acht überhängende Bandblätter in wechselnden Zeilen (die Blattebenen benachbarter Pflanzen gegeneinander gedreht, wie in einer gesäten Reihe), einen Kolben in Lieschen mit Narbenfäden in 1 m Höhe und eine verzweigte Rispe. Eine C4-Kultur."),
        "tags": ["maize", "corn", "crop", "grass", "row", "c4", "fruit", "wind", "transpiration"],
        "provenance": {"author": "Semio staffage plant worker PW-16", "method": "handcrafted", "created": "2026-11-22", "references": [{"id": k, "citation": v} for k, v in REFS.items()],
                       "notes": T("Plant height, leaf number and ear height follow Ritchie et al. (1993) for a mid-season hybrid at VT/R1; the in-row spacing of 0.2 m matches 6.7 plants per m2 at 0.75 m rows (FAO). Masses (about 1 kg fresh per plant at tasseling), leaf conductance (C4, 5.5 mm/s) and leaf area (0.6 m2 per plant) are authored estimates guided by these sources and Jones (2014). The ridge is ground: it carries no mass.",
                                  "Pflanzenhöhe, Blattzahl und Kolbenhöhe folgen Ritchie et al. (1993) für eine mittelfrühe Hybride zu VT/R1; der Abstand in der Reihe von 0,2 m entspricht 6,7 Pflanzen je m2 bei 0,75 m Reihenweite (FAO). Massen (etwa 1 kg Frischmasse je Pflanze zur Rispenblüte), Blattleitfähigkeit (C4, 5,5 mm/s) und Blattfläche (0,6 m2 je Pflanze) sind Autorenschätzungen angelehnt an diese Quellen und Jones (2014). Der Damm ist Boden: er trägt keine Masse.")},
        "dimensions": {"heightM": ext[1], "widthM": ext[0], "depthM": ext[2], "massKg": mass},
        "materials": MATERIALS, "representations": reps, "symbols": symbol(box), "poses": POSES, "clips": clips(),
        "thermal": {"environment": {"airTempC": 27, "meanRadiantTempC": 30, "relativeHumidity": 0.5, "windMS": 1.5, "windDirectionDeg": 0, "solarWM2": 800, "diffuseFraction": 0.15, "pressurePa": 101325}, "passengers": [], "operating": [], "regions": REGIONS, "emitters": EMITTERS,
                    "assumptions": [T("The leaf region uses the one-sided area of the 48 blades (0.6 m2 per plant); its leaf area index of 1.6 refers to the ground projection of the leaf spread (about 2.3 m2). The maximum stomatal conductance of 5.5 mm/s is an authored estimate for a well-watered C4 crop, lower than in C3 leaves (Jones 2014).", "Die Blattregion verwendet die einseitige Fläche der 48 Spreiten (0,6 m2 je Pflanze); ihr Blattflächenindex von 1,6 bezieht sich auf die Grundprojektion der Blattausladung (etwa 2,3 m2). Die maximale Spaltöffnungsleitfähigkeit von 5,5 mm/s ist eine Autorenschätzung für eine gut versorgte C4-Kultur, niedriger als bei C3-Blättern (Jones 2014)."),
                                    T("Stalks, ears, tassels and ridge are passive: areal heat capacities from volumetric heat capacity times an effective depth (stalk 2.8 MJ/(m3 K) x 8 mm, ear in husk 3.2 MJ/(m3 K) x 14 mm, tassel 1 mm, moist loam 2.0 MJ/(m3 K) x 50 mm). No surface temperature is authored.", "Halme, Kolben, Rispen und Damm sind passiv: flächenbezogene Wärmekapazitäten aus volumetrischer Wärmekapazität mal wirksamer Tiefe (Halm 2,8 MJ/(m3 K) x 8 mm, Kolben in Lieschen 3,2 MJ/(m3 K) x 14 mm, Rispe 1 mm, feuchter Lehm 2,0 MJ/(m3 K) x 50 mm). Es werden keine Oberflächentemperaturen vorgegeben.")]},
        "structure": {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": [a for a, _ in SUPPORTS], "assumptions": [T("One rigid body per evaluated pose, each stalk fixed at its root crown in the ridge. Wind drag acts on the leaves of each plant (cf 0.6 on 0.4 m2, lumped on the upper stalk) and on each tassel (cf 1.0 on 0.03 m2), EN 1991-1-4 form; gravity on every bone mass. The stalk is a solid green tube (E 1.5 GPa, authored estimate); lodging, root pull-out and dynamic amplification are not modelled.", "Ein starrer Körper je ausgewerteter Pose, jeder Halm am Wurzelhals im Damm eingespannt. Winddruck wirkt auf die Blätter jeder Pflanze (cf 0,6 auf 0,4 m2, am oberen Halm zusammengefasst) und auf jede Rispe (cf 1,0 auf 0,03 m2), Form nach EN 1991-1-4; Schwerkraft auf jede Knochenmasse. Der Halm ist ein volles grünes Rohr (E 1,5 GPa, Schätzung); Lagern, Ausreißen der Wurzel und dynamische Überhöhung werden nicht modelliert.")], "clinical": False},
        "volumes": vols,
        "presentation": {"defaultPose": "tasseling-calm", "defaultClip": "wind-sway", "defaultScale": 50},
        "exports": EXPORTS, "completion": "authored",
    }
    print(write(asset, ID))
    manifest_row(asset, T("A field row of six maize plants (Zea mays) at tasseling on an earthed-up ridge, 2.5 m high: arching strap leaves in alternating ranks, ears in husk with silk and branched tassels. A C4 crop.", "Eine Feldreihe aus sechs Maispflanzen (Zea mays) zur Rispenblüte auf einem angehäufelten Damm, 2,5 m hoch: überhängende Bandblätter in wechselnden Zeilen, Kolben in Lieschen mit Narbenfäden und verzweigte Rispen. Eine C4-Kultur."))


if __name__ == "__main__":
    main()
