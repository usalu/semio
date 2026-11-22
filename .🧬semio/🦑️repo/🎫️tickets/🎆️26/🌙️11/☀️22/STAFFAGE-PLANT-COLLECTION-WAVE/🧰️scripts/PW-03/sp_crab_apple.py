"""🍏️ crab-apple: a mature European wild crab apple (Malus sylvestris) with a short leaning stem, four low spreading limbs, a central leader, thorny spur shoots and a dense rounded crown hung with small yellow-green fruit; every number is authored here.
@see ./pw03_tree.py"""
from pw03_tree import *
from common import fruit_extra
from sp_plane import Sp, common_refs, tree_thermal_assumptions, tree_structure_assumptions

W = "crab-wood"
LIMBS = [(1, 0.86, 22, 14, 44, 40), (2, 0.9, 104, -12, 47, 43), (3, 0.95, 192, 15, 42, 38), (4, 1.0, 278, -14, 46, 41)]


def build():
    S = Sp()
    S.id = S.folder = "crab-apple"
    S.title = T("Crab Apple, Wild, with Fruit", "Holzapfel, wild, mit Früchten")
    S.summary = T("A mature European wild crab apple (Malus sylvestris): a short, slightly leaning and twisted stem of 1.3 m, four low limbs spreading almost horizontally, a central leader and thorny spur shoots under a dense, broad and rounded crown hung with small yellow-green crab apples of 3 cm, carried as a seasonal fruit load from green fruit set to the full autumn load.",
                  "Ein ausgewachsener Europäischer Holzapfel (Malus sylvestris): ein kurzer, leicht geneigter und gedrehter Stamm von 1,3 m, vier tiefe, fast waagerecht ausladende Leitäste, ein Mitteltrieb und dornige Kurztriebe unter einer dichten, breiten und runden Krone, behängt mit kleinen gelbgrünen Holzäpfeln von 3 cm, als jahreszeitliche Fruchtlast vom grünen Fruchtansatz bis zur vollen Herbstlast geführt.")
    S.tags = ["tree", "deciduous", "native", "fruit-tree", "fruit-load", "hedgerow", "wind", "transpiration"]
    S.depth_m = 6.8
    S.footprint = 1.4
    S.density = 950
    S.root_mass_factor = 1.0
    S.default_scale = 100
    S.default_pose = "autumn-fruit"
    S.breeze_pose = "fruit-breeze"
    S.breeze_v10 = 5.5
    S.breeze_droop = 0.7
    S.gust_peak = 14.0
    S.trunk_bone = "trunk"
    S.twig_factor = {"leaf-sun": 1.6, "leaf-shade": 1.6}
    S.cd = {}
    S.bark_extra = {}
    S.exchange_regions = {"fruit"}
    t = Tree(950)
    S.tree = t
    S.rig = t.rig
    t.stem("root-plate", None, [(0.0, 0.34, 0, 0), (0.35, 0.205, 0.004, -0.002)], T("Root plate and flare", "Wurzelteller und Wurzelanlauf"), 0, W, "trunk-bark", "trunk", "root")
    t.stem("trunk", "root-plate", [(0.35, 0.2, 0.004, -0.002), (0.7, 0.19, 0.025, -0.01), (1.0, 0.188, 0.05, -0.022), (1.2, 0.196, 0.068, -0.03), (1.36, 0.21, 0.082, -0.036)], T("Short leaning stem", "Kurzer geneigter Stamm"), 3.0, W, "trunk-bark", "trunk", "stem")
    t.rig.bones["root-plate"].kg *= 1.6
    for k, f, az, bend, e0, e1 in LIMBS:
        t.limb("sc-%da" % k, "trunk", ("trunk", f), az, e0, e1, 2.3, 0.11, 0.075, T("Main limb %d, inner part" % k, "Leitast %d, innerer Teil" % k), 9, W, "limb-bark", "limb", "limb")
        t.limb("sc-%db" % k, "sc-%da" % k, ("sc-%da" % k, 1.0), az + bend * 0.3, 32, 12, 2.1, 0.072, 0.03, T("Main limb %d, spreading outer part" % k, "Leitast %d, ausladender Außenteil" % k), 13, W, "limb-bark", "limbb", "limb", bend=bend)
        t.limb("spur-%d" % k, "sc-%db" % k, ("sc-%db" % k, 0.62), az + bend + 30, 68, 58, 0.55, 0.013, 0.008, T("Thorny spur shoot on limb %d" % k, "Dorniger Kurztrieb an Leitast %d" % k), 22, W, "spur-twig", "spur", "twig", n=4)
    t.limb("ld", "trunk", ("trunk", 1.0), 205, 82, 77, 4.8, 0.09, 0.03, T("Central leader", "Mitteltrieb"), 6, W, "limb-bark", "upper", "limb")
    S.levels = t.levels
    S.lean_ref = {"trunk": 0.3, "upper": 0.6, "limb": 0.8, "limbb": 1.3, "spur": 2.0, "fol": 3.5}
    S.droop_levels = {"limb": 2.0, "limbb": 3.5, "spur": 4.0, "fol": 4.5, "upper": 0.8}
    S.sway_amp = {"trunk": 0.5, "upper": 0.5, "limb": 0.55, "limbb": 0.6, "spur": 0.6, "fol": 0.7}
    S.sway_lag = 0.45
    S.sway_period = 2.4
    S.gust_tau = {"trunk": 0.6, "upper": 0.45, "limb": 0.4, "limbb": 0.35, "spur": 0.3, "fol": 0.2}
    S.sway_desc_en = "A deterministic %s s sway cycle about the breeze lean with most of the fruit load (5.5 m/s at 10 m; estimated first mode of the 7 m tree about 0.4 Hz). The spreading limbs bob in anti-phase to the stem; the foliage masses swing about 3 degrees."
    S.sway_desc_de = "Ein deterministischer Schwingungszyklus von %s s um die Neigung bei Brise mit dem Großteil der Fruchtlast (5,5 m/s in 10 m Höhe; geschätzte erste Eigenform des 7 m hohen Baums etwa 0,4 Hz). Die ausladenden Leitäste wippen gegenphasig zum Stamm; die Laubmassen schwingen etwa 3 Grad."
    S.gust_desc_en = "A single autumn gust over 8 s from %.1f to %.1f m/s at 10 m with the fruit load of the breeze pose: the wind-load tracks rise with the reconfigured drag law, the spreading limbs lean downwind with a level-wise lag."
    S.gust_desc_de = "Eine einzelne Herbstböe über 8 s von %.1f auf %.1f m/s in 10 m Höhe mit der Fruchtlast der Brisenpose: Die Windlast-Spuren steigen mit dem umgeformten Widerstandsgesetz, die ausladenden Leitäste neigen sich ebenenweise verzögert in Windrichtung."
    S.masses = []
    for k, f, az, bend, e0, e1 in LIMBS:
        a = az + bend * 0.5
        S.masses.append(t.mass("fol-%d" % k, "sc-%db" % k, (2.75 * math.cos(a * D2R) + 0.1, 3.95, 2.75 * math.sin(a * D2R)), 1.4, 1.1, "leaf-sun", "sun", T("Foliage mass over limb %d" % k, "Laubmasse über Leitast %d" % k)))
    S.masses.append(t.mass("fol-core", "ld", (0.05, 4.1, -0.05), 1.8, 1.3, "leaf-shade", "shade", T("Shaded inner crown", "Beschattete Innenkrone")))
    S.masses.append(t.mass("fol-top", "ld", (-0.1, 6.1, -0.05), 1.7, 0.75, "leaf-sun", "sun", T("Crown top", "Kronenspitze")))
    proj = math.pi / 4 * 7.3 * S.depth_m
    S.leaf_area = {"leaf-sun": 2.0 * proj, "leaf-shade": 1.9 * proj}
    S.tone = {"wood": 0.36, "sun": 0.58, "shade": 0.44}
    S.tone_line = 0.2
    S.sun = {"trunk-bark": 0.35, "limb-bark": 0.3, "spur-twig": 0.3, "leaf-sun": 0.9, "leaf-shade": 0.1, "fruit": 0.55}
    S.regions = [
        {"id": "trunk-bark", "title": T("Grey-brown bark of the stem, cracking into small plates", "Graubraune, in kleine Platten reißende Stammrinde"), "kind": "bark", "model": "passive", "material": "crab-bark", "len": 0.4, "geom": "cylinder", "sky": 0.45, "hc": 60000, "ref": "wood-handbook-2021"},
        {"id": "limb-bark", "title": T("Bark of the main limbs and the leader", "Rinde der Leitäste und des Mitteltriebs"), "kind": "bark", "model": "passive", "material": "crab-bark-young", "len": 0.14, "geom": "cylinder", "sky": 0.55, "hc": 40000, "ref": "wood-handbook-2021"},
        {"id": "spur-twig", "title": T("Thorny spur shoots", "Dornige Kurztriebe"), "kind": "stem", "model": "passive", "material": "crab-twig", "len": 0.02, "geom": "cylinder", "sky": 0.6, "hc": 12000, "ref": "wood-handbook-2021"},
        {"id": "leaf-sun", "title": T("Sun leaves of the outer crown", "Sonnenblätter der Außenkrone"), "kind": "foliage", "model": "leaf", "material": "crab-leaf-sun", "area": S.leaf_area["leaf-sun"], "len": 0.05, "geom": "vertical-plate", "sky": 0.7, "hc": 800, "gs": 0.005, "lai": 2.0, "ref": "jones-2014"},
        {"id": "leaf-shade", "title": T("Shade leaves of the inner crown", "Schattenblätter der Innenkrone"), "kind": "foliage", "model": "leaf", "material": "crab-leaf-shade", "area": S.leaf_area["leaf-shade"], "len": 0.06, "geom": "vertical-plate", "sky": 0.3, "hc": 650, "gs": 0.003, "lai": 1.9, "ref": "jones-2014"},
        {"id": "fruit", "title": T("Skin of the crab apples", "Schale der Holzäpfel"), "kind": "surface", "model": "passive", "material": "crab-fruit", "area": 7.6, "len": 0.03, "geom": "sphere", "sky": 0.5, "hc": 17000, "ref": "euforgen-2003"},
    ]
    sunm = [m for m in S.masses if m.kind == "sun"]
    S.emitters = [
        {"id": "transpiration-sun", "aid": "tp-sun", "region": "leaf-sun", "strength": 1.0, "diam": 2.0, "title": T("Transpiration of the sun leaves", "Transpiration der Sonnenblätter"), "anchors": [(m.id, (m.c[0], m.c[1] + 0.2)) for m in sunm]},
        {"id": "transpiration-shade", "aid": "tp-shade", "region": "leaf-shade", "strength": 0.7, "diam": 1.6, "title": T("Transpiration of the shade leaves", "Transpiration der Schattenblätter"), "anchors": [("fol-core", (0.05 + dx, 4.1 + dy)) for dx, dy in [(-0.8, 0.0), (0.8, 0.1), (0.0, 0.5), (-0.4, -0.4), (0.5, -0.3)]]},
    ]
    S.materials = [
        {"id": "crab-bark", "title": T("Crab apple bark, grey-brown and plated", "Holzapfelrinde, graubraun und schuppig"), "emissivity": 0.95, "solarAbsorptivity": 0.72, "densityKgM3": 650, "conductivityWmK": 0.12, "source": "authored-estimate", "reference": "wood-handbook-2021"},
        {"id": "crab-bark-young", "title": T("Smoother bark of the limbs", "Glattere Rinde der Äste"), "emissivity": 0.95, "solarAbsorptivity": 0.65, "densityKgM3": 700, "conductivityWmK": 0.13, "source": "authored-estimate", "reference": "wood-handbook-2021"},
        {"id": "crab-twig", "title": T("Dark brown bark of the spur shoots", "Dunkelbraune Rinde der Kurztriebe"), "emissivity": 0.95, "solarAbsorptivity": 0.68, "densityKgM3": 750, "conductivityWmK": 0.14, "source": "authored-estimate", "reference": "wood-handbook-2021"},
        {"id": "crab-leaf-sun", "title": T("Crab apple sun leaves", "Holzapfel-Sonnenblätter"), "emissivity": 0.97, "solarAbsorptivity": 0.5, "source": "literature", "reference": "jones-2014"},
        {"id": "crab-leaf-shade", "title": T("Crab apple shade leaves (effective)", "Holzapfel-Schattenblätter (wirksam)"), "emissivity": 0.97, "solarAbsorptivity": 0.25, "source": "authored-estimate", "reference": "jones-2014"},
        {"id": "crab-fruit", "title": T("Crab apple skin (waxy, hard, sour fruit)", "Holzapfelschale (wachsige, harte, saure Frucht)"), "emissivity": 0.95, "solarAbsorptivity": 0.55, "densityKgM3": 850, "conductivityWmK": 0.4, "specificHeatJKgK": 3600, "source": "authored-estimate", "reference": "euforgen-2003"},
        {"id": W, "title": T("Green crab apple wood (very dense)", "Frisches Holzapfelholz (sehr dicht)"), "emissivity": 0.9, "solarAbsorptivity": 0.65, "densityKgM3": 950, "conductivityWmK": 0.17, "elasticModulusPa": 7500000000, "strengthPa": 50000000, "source": "authored-estimate", "reference": "wood-handbook-2021"},
    ]
    S.references = common_refs() + [
        {"id": "euforgen-2003", "citation": "Stephan, B. R., Wagner, I. and Kleinschmit, J. (2003). EUFORGEN Technical Guidelines for genetic conservation and use for wild apple and pear (Malus sylvestris and Pyrus pyraster). International Plant Genetic Resources Institute, Rome. Malus sylvestris: small tree up to about 10 m with a broad, dense crown, often with thorny short shoots; fruit 2 to 4 cm, yellow-green, sometimes flushed red, hard and sour."},
        {"id": "crab-yield", "citation": "Authored estimate for this asset: a mature wild crab apple of this size bears about 40 kg of fruit (about 2700 crab apples of 15 g, skin about 7.6 m2) in a good year; not a measured yield."},
    ]
    S.notes = T("Rig and drawings are authored by hand for a 40-year-old wild crab apple grown free at a field edge: a short stem of 1.3 m leaning slightly to the east, four main limbs leaving at 42 to 47 degrees above the horizontal and spreading almost horizontally, a central leader, crown about 7 m high and 7.7 m wide. The fruit load of 10 to 40 kg and the fruit counts are authored estimates; the drawn fruit are a sample of the load.",
                  "Rig und Zeichnungen sind von Hand für einen 40-jährigen, frei an einem Feldrand gewachsenen Holzapfel entworfen: ein kurzer Stamm von 1,3 m, leicht nach Osten geneigt, vier Leitäste, die mit 42 bis 47 Grad über der Waagerechten abgehen und fast waagerecht ausladen, ein Mitteltrieb, Krone etwa 7 m hoch und 7,7 m breit. Die Fruchtlast von 10 bis 40 kg und die Fruchtzahlen sind Autorenschätzungen; die gezeichneten Früchte sind eine Auswahl der Last.")
    S.environment = {"airTempC": 16, "meanRadiantTempC": 19, "relativeHumidity": 0.65, "windMS": 1.5, "windDirectionDeg": 0, "solarWM2": 500, "pressurePa": 101325, "diffuseFraction": 0.25}
    S.thermal_assumptions = tree_thermal_assumptions("", "") + [T("The crab apple skin is a passive surface of the whole fruit load (about 2700 fruit of 3 cm, 7.6 m2); its areal heat capacity is that of a water-rich sphere (850 kg/m3, 3600 J/(kg K), radius/3), an authored estimate.", "Die Holzapfelschale ist eine passive Fläche der gesamten Fruchtlast (etwa 2700 Früchte von 3 cm, 7,6 m2); ihre flächenbezogene Wärmekapazität ist die einer wasserreichen Kugel (850 kg/m3, 3600 J/(kg K), Radius/3), eine Autorenschätzung.")]
    S.structure_assumptions = tree_structure_assumptions("crab apple", "Holzapfel")
    share = {m.id: m.area() for m in sunm}
    tot = sum(share.values())

    def fruit(total_kg):
        return fruit_extra({k: total_kg * v / tot for k, v in share.items()}, "crab-yield")

    S.poses = [
        {"id": "summer-green-fruit", "title": T("Summer, green fruit set", "Sommer, grüner Fruchtansatz"), "desc": T("Light air of 1.5 m/s at 10 m in July: about 10 kg of hard green crab apples hang in the crown, the limbs are hardly bent.", "Leiser Zug von 1,5 m/s in 10 m Höhe im Juli: Etwa 10 kg harte grüne Holzäpfel hängen in der Krone, die Leitäste sind kaum gebogen."), "v10": 1.5, "sun": 1.0, "emit": 0.85, "droop": 0.2, "extra": fruit(10.0)},
        {"id": "autumn-fruit", "title": T("Autumn, full fruit load", "Herbst, volle Fruchtlast"), "desc": T("A calm early-autumn day with the full load of about 40 kg (2700 yellow-green crab apples): the spreading limbs bow down by a few degrees, the leaves begin to age and transpire at 60 %.", "Ein windstiller Frühherbsttag mit der vollen Last von etwa 40 kg (2700 gelbgrüne Holzäpfel): Die ausladenden Leitäste biegen sich um einige Grad nach unten, die Blätter beginnen zu altern und transpirieren zu 60 %."), "v10": 1.0, "sun": 0.85, "emit": 0.6, "droop": 0.9, "extra": fruit(40.0)},
        {"id": "fruit-breeze", "title": T("Breeze with fruit", "Brise mit Früchten"), "desc": T("Moderate breeze of 5.5 m/s at 10 m (Beaufort 4) with about 35 kg of fruit: the limbs droop and lean downwind; drag by the reconfigured law (cd 0.35).", "Mäßige Brise von 5,5 m/s in 10 m Höhe (Beaufort 4) mit etwa 35 kg Früchten: Die Leitäste hängen durch und neigen sich in Windrichtung; Widerstand nach dem umgeformten Gesetz (cd 0,35)."), "v10": 5.5, "sun": 1.0, "emit": 0.7, "droop": 0.7, "extra": fruit(35.0)},
        {"id": "autumn-gale", "title": T("Autumn gale with windfall", "Herbststurm mit Fallobst"), "desc": T("Beaufort 9, 22 m/s at 10 m with rain: strong lean, drooping limbs, wet bark and leaves; about 15 kg of fruit have fallen, 25 kg still hang.", "Beaufort 9, 22 m/s in 10 m Höhe mit Regen: starke Neigung, hängende Leitäste, nasse Rinde und Blätter; etwa 15 kg Früchte sind gefallen, 25 kg hängen noch."), "v10": 22.0, "sun": 0.15, "emit": 0.4, "wet": 0.6, "droop": 0.6, "extra": fruit(25.0)},
    ]
    S.make_reps = skeleton_reps
    S.make_symbols = lambda S, b: []
    return S
