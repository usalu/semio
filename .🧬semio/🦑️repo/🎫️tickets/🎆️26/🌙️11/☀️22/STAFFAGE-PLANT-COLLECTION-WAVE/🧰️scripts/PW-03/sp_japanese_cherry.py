"""🌸️ japanese-cherry: a mature Prunus serrulata 'Kanzan' in full bloom, top-grafted standard with a funnel-shaped crown of five ascending scaffolds and a central upright; every number is authored here.
@see ./pw03_tree.py"""
from pw03_tree import *
from common import fruit_extra, snow_extra
from sp_plane import Sp, common_refs, tree_thermal_assumptions, tree_structure_assumptions

W = "cherry-wood"
BLOSSOM_KG = 24.0
SCAFFOLDS = [(1, 0.88, 12, 10), (2, 0.92, 84, -8), (3, 0.95, 158, 12), (4, 0.97, 226, -10), (5, 1.0, 298, 8)]


def build():
    S = Sp()
    S.id = S.folder = "japanese-cherry"
    S.title = T("Japanese Cherry 'Kanzan', in Bloom", "Japanische Blütenkirsche 'Kanzan', in Blüte")
    S.summary = T("A mature ornamental Japanese cherry (Prunus serrulata 'Kanzan') in full bloom: a top-grafted standard with a clear stem of 1.9 m and the graft swelling below the fork, five scaffolds rising steeply in a funnel and arching out into a broad, flat-topped vase crown hung with clusters of double pink flowers among the young bronze leaves.",
                  "Eine ausgewachsene Japanische Blütenkirsche (Prunus serrulata 'Kanzan') in voller Blüte: ein kopfveredelter Hochstamm mit 1,9 m Stammhöhe und der Veredelungswulst unter der Verzweigung, fünf Leitäste, die steil trichterförmig aufsteigen und in eine breite, flach gewölbte Vasenkrone überbiegen, behängt mit Büscheln gefüllter rosa Blüten zwischen dem jungen bronzefarbenen Laub.")
    S.tags = ["tree", "deciduous", "ornamental", "flowering-tree", "blossom", "street-tree", "wind", "transpiration"]
    S.depth_m = 7.9
    S.footprint = 1.6
    S.density = 880
    S.root_mass_factor = 1.0
    S.default_scale = 100
    S.default_pose = "full-bloom"
    S.breeze_pose = "bloom-breeze"
    S.breeze_v10 = 5.5
    S.breeze_droop = 0.15
    S.gust_peak = 14.0
    S.trunk_bone = "trunk"
    S.twig_factor = {"leaf-sun": 1.6, "leaf-shade": 1.6}
    S.cd = {}
    S.bark_extra = {}
    S.exchange_regions = {"blossom"}
    t = Tree(880)
    S.tree = t
    S.rig = t.rig
    t.stem("root-plate", None, [(0.0, 0.31, 0, 0), (0.35, 0.205, 0.003, 0)], T("Root plate and root flare", "Wurzelteller und Wurzelanlauf"), 0, W, "trunk-bark", "trunk", "root")
    t.stem("trunk", "root-plate", [(0.35, 0.2, 0.003, 0), (0.8, 0.186, 0.006, 0), (1.3, 0.18, 0.01, 0.002), (1.65, 0.184, 0.013, 0.003), (1.8, 0.205, 0.015, 0.003), (1.95, 0.19, 0.016, 0.003)], T("Clear stem with graft union", "Stamm mit Veredelungsstelle"), 3.0, W, "trunk-bark", "trunk", "stem")
    t.rig.bones["root-plate"].kg *= 1.6
    for k, f, az, bend in SCAFFOLDS:
        t.limb("sc-%da" % k, "trunk", ("trunk", f), az, 62, 57, 3.0, 0.115, 0.085, T("Scaffold %d, ascending part" % k, "Leitast %d, aufsteigender Teil" % k), 8, W, "limb-bark", "limb", "limb")
        t.limb("sc-%db" % k, "sc-%da" % k, ("sc-%da" % k, 1.0), az + bend * 0.3, 52, 34, 2.6, 0.082, 0.034, T("Scaffold %d, arching outer part" % k, "Leitast %d, überbiegender Außenteil" % k), 12, W, "limb-bark", "limbb", "limb", bend=bend)
    t.limb("up-c", "trunk", ("trunk", 1.0), 30, 86, 79, 5.4, 0.075, 0.026, T("Central upright limb", "Mittlerer aufrechter Ast"), 6, W, "limb-bark", "upper", "limb")
    t.limb("up-1", "sc-1a", ("sc-1a", 0.55), 44, 76, 68, 2.4, 0.05, 0.022, T("Upright shoot on scaffold 1", "Aufrechter Trieb an Leitast 1"), 12, W, "limb-bark", "upper", "limb")
    t.limb("up-3", "sc-3a", ("sc-3a", 0.55), 186, 76, 68, 2.4, 0.05, 0.022, T("Upright shoot on scaffold 3", "Aufrechter Trieb an Leitast 3"), 12, W, "limb-bark", "upper", "limb")
    S.levels = t.levels
    S.lean_ref = {"trunk": 0.3, "upper": 0.7, "limb": 0.8, "limbb": 1.3, "fol": 3.2}
    S.droop_levels = {"limb": 1.2, "limbb": 2.5, "fol": 3.0, "upper": 0.5}
    S.sway_amp = {"trunk": 0.5, "upper": 0.55, "limb": 0.55, "limbb": 0.6, "fol": 0.7}
    S.sway_lag = 0.45
    S.sway_period = 2.6
    S.gust_tau = {"trunk": 0.6, "upper": 0.45, "limb": 0.4, "limbb": 0.35, "fol": 0.2}
    S.sway_desc_en = "A deterministic %s s sway cycle about the breeze lean in full bloom (5.5 m/s at 10 m; estimated first mode of the 8 m tree about 0.38 Hz). The arching scaffold ends bob behind the stem with a level-wise lag; the blossom-laden foliage masses swing about 3 degrees."
    S.sway_desc_de = "Ein deterministischer Schwingungszyklus von %s s um die Neigung bei Brise in voller Blüte (5,5 m/s in 10 m Höhe; geschätzte erste Eigenform des 8 m hohen Baums etwa 0,38 Hz). Die überbiegenden Leitastenden wippen ebenenweise verzögert hinter dem Stamm; die blütenbeladenen Laubmassen schwingen etwa 3 Grad."
    S.gust_desc_en = "A single spring gust over 8 s from %.1f to %.1f m/s at 10 m in full bloom: the wind-load tracks rise with the reconfigured drag law, the scaffolds lean downwind with a level-wise lag."
    S.gust_desc_de = "Eine einzelne Frühjahrsböe über 8 s von %.1f auf %.1f m/s in 10 m Höhe in voller Blüte: Die Windlast-Spuren steigen mit dem umgeformten Widerstandsgesetz, die Leitäste neigen sich ebenenweise verzögert in Windrichtung."
    S.masses = []
    for k, f, az, bend in SCAFFOLDS:
        a = az + bend * 0.6
        S.masses.append(t.mass("fol-%d" % k, "sc-%db" % k, (2.85 * math.cos(a * D2R), 6.05, 2.85 * math.sin(a * D2R)), 1.35, 1.0, "leaf-sun", "sun", T("Blossom and leaf mass over scaffold %d" % k, "Blüten- und Laubmasse über Leitast %d" % k)))
    S.masses.append(t.mass("fol-core", "up-c", (0.35, 5.2, 0.2), 1.7, 1.2, "leaf-shade", "shade", T("Shaded inner crown", "Beschattete Innenkrone")))
    S.masses.append(t.mass("fol-top", "up-c", (0.45, 7.4, 0.26), 1.7, 0.6, "leaf-sun", "sun", T("Flat crown top", "Flache Kronenoberseite")))
    proj = math.pi / 4 * 8.1 * S.depth_m
    S.leaf_area = {"leaf-sun": 1.0 * proj, "leaf-shade": 0.6 * proj}
    S.tone = {"wood": 0.36, "sun": 0.62, "shade": 0.45}
    S.tone_line = 0.2
    S.sun = {"trunk-bark": 0.35, "limb-bark": 0.3, "leaf-sun": 0.85, "leaf-shade": 0.1, "blossom": 0.8}
    S.regions = [
        {"id": "trunk-bark", "title": T("Glossy chestnut-brown stem bark with horizontal lenticel bands", "Glänzend kastanienbraune Stammrinde mit waagerechten Lentizellenbändern"), "kind": "bark", "model": "passive", "material": "cherry-bark", "len": 0.38, "geom": "cylinder", "sky": 0.45, "hc": 60000, "ref": "wood-handbook-2021"},
        {"id": "limb-bark", "title": T("Smoother bark of the scaffolds and uprights", "Glattere Rinde der Leitäste und aufrechten Äste"), "kind": "bark", "model": "passive", "material": "cherry-bark-young", "len": 0.12, "geom": "cylinder", "sky": 0.5, "hc": 35000, "ref": "wood-handbook-2021"},
        {"id": "leaf-sun", "title": T("Young bronze leaves of the outer crown", "Junge bronzefarbene Blätter der Außenkrone"), "kind": "foliage", "model": "leaf", "material": "cherry-leaf-sun", "area": S.leaf_area["leaf-sun"], "len": 0.09, "geom": "vertical-plate", "sky": 0.7, "hc": 600, "gs": 0.004, "lai": 1.0, "ref": "jones-2014"},
        {"id": "leaf-shade", "title": T("Young shade leaves of the inner crown", "Junge Schattenblätter der Innenkrone"), "kind": "foliage", "model": "leaf", "material": "cherry-leaf-shade", "area": S.leaf_area["leaf-shade"], "len": 0.1, "geom": "vertical-plate", "sky": 0.3, "hc": 550, "gs": 0.0025, "lai": 0.6, "ref": "jones-2014"},
        {"id": "blossom", "title": T("Petals of the double pink flower clusters", "Blütenblätter der gefüllten rosa Blütenbüschel"), "kind": "surface", "model": "passive", "material": "cherry-blossom", "area": 120.0, "len": 0.055, "geom": "sphere", "sky": 0.6, "hc": 1100, "ref": "kuitert-1999"},
    ]
    sunm = [m for m in S.masses if m.kind == "sun"]
    S.emitters = [
        {"id": "transpiration-sun", "aid": "tp-sun", "region": "leaf-sun", "strength": 1.0, "diam": 2.0, "title": T("Transpiration of the young sun leaves", "Transpiration der jungen Sonnenblätter"), "anchors": [(m.id, (m.c[0], m.c[1] + 0.2)) for m in sunm]},
        {"id": "transpiration-shade", "aid": "tp-shade", "region": "leaf-shade", "strength": 0.6, "diam": 1.6, "title": T("Transpiration of the shade leaves", "Transpiration der Schattenblätter"), "anchors": [("fol-core", (0.35 + dx, 5.2 + dy)) for dx, dy in [(-0.8, 0.0), (0.8, 0.1), (0.0, 0.5), (-0.4, -0.4), (0.5, -0.3)]]},
    ]
    S.materials = [
        {"id": "cherry-bark", "title": T("Cherry bark, glossy with lenticel bands", "Kirschrinde, glänzend mit Lentizellenbändern"), "emissivity": 0.94, "solarAbsorptivity": 0.74, "densityKgM3": 650, "conductivityWmK": 0.12, "source": "authored-estimate", "reference": "wood-handbook-2021"},
        {"id": "cherry-bark-young", "title": T("Smooth reddish-brown bark of the limbs", "Glatte rotbraune Rinde der Äste"), "emissivity": 0.94, "solarAbsorptivity": 0.7, "densityKgM3": 700, "conductivityWmK": 0.13, "source": "authored-estimate", "reference": "wood-handbook-2021"},
        {"id": "cherry-leaf-sun", "title": T("Young bronze cherry leaves", "Junge bronzefarbene Kirschblätter"), "emissivity": 0.97, "solarAbsorptivity": 0.55, "source": "authored-estimate", "reference": "jones-2014"},
        {"id": "cherry-leaf-shade", "title": T("Young cherry shade leaves (effective)", "Junge Kirsch-Schattenblätter (wirksam)"), "emissivity": 0.97, "solarAbsorptivity": 0.28, "source": "authored-estimate", "reference": "jones-2014"},
        {"id": "cherry-blossom", "title": T("Pink petals, thin and water-rich", "Rosa Blütenblätter, dünn und wasserreich"), "emissivity": 0.95, "solarAbsorptivity": 0.35, "densityKgM3": 900, "conductivityWmK": 0.4, "specificHeatJKgK": 3800, "source": "authored-estimate", "reference": "kuitert-1999"},
        {"id": W, "title": T("Green cherry wood", "Frisches Kirschholz"), "emissivity": 0.9, "solarAbsorptivity": 0.65, "densityKgM3": 880, "conductivityWmK": 0.16, "elasticModulusPa": 9000000000, "strengthPa": 55000000, "source": "authored-estimate", "reference": "wood-handbook-2021"},
    ]
    S.references = common_refs() + [
        {"id": "kuitert-1999", "citation": "Kuitert, W. (1999). Japanese Flowering Cherries. Timber Press, Portland. Cultivar 'Kanzan' (syn. 'Sekiyama'): vigorous, stiffly ascending vase habit spreading with age to about 8 to 12 m, double pink flowers of about 5 to 6 cm in clusters of 2 to 5 opening with the bronze young leaves in April."},
        {"id": "cherry-blossom-load", "citation": "Authored estimate for this asset: about 12 000 double flowers of about 2 g fresh mass (24 kg) and a petal exchange area of about 120 m2 in full bloom; not a measured count."},
    ]
    S.notes = T("Rig and drawings are authored by hand for a 30-year-old 'Kanzan' grafted at 1.8 m on a wild-cherry stem: clear stem 1.9 m with the graft swelling, five scaffolds leaving at 28 to 33 degrees from the vertical and arching out to about 35 degrees above the horizontal, a central upright and two upright shoots, crown about 8 m high and 8 m wide. The asset is the flowering state: the blossom and the half-expanded young leaves are drawn in every pose; the poses differ in wind, wetness, snow and blossom load. Flower and leaf quantities are authored estimates.",
                  "Rig und Zeichnungen sind von Hand für eine 30-jährige 'Kanzan', in 1,8 m Höhe auf einen Vogelkirschenstamm veredelt, entworfen: Stamm 1,9 m mit Veredelungswulst, fünf Leitäste, die mit 28 bis 33 Grad gegen die Senkrechte abgehen und bis etwa 35 Grad über der Waagerechten überbiegen, ein mittlerer aufrechter Ast und zwei aufrechte Triebe, Krone etwa 8 m hoch und 8 m breit. Das Asset ist der Blühzustand: Blüten und das halb entfaltete junge Laub sind in jeder Pose gezeichnet; die Posen unterscheiden sich in Wind, Nässe, Schnee und Blütenlast. Blüten- und Blattmengen sind Autorenschätzungen.")
    S.environment = {"airTempC": 15, "meanRadiantTempC": 18, "relativeHumidity": 0.6, "windMS": 1.5, "windDirectionDeg": 0, "solarWM2": 600, "pressurePa": 101325, "diffuseFraction": 0.25}
    S.thermal_assumptions = tree_thermal_assumptions("", "") + [T("The petals of the flower clusters are one passive surface with the exchange area of about 12 000 double flowers (120 m2, both faces of the petal layers); their areal heat capacity is that of a 0.3 mm water-rich sheet, an authored estimate. Petal transpiration is neglected.", "Die Blütenblätter der Büschel sind eine passive Fläche mit der Austauschfläche von etwa 12 000 gefüllten Blüten (120 m2, beide Seiten der Blütenblattlagen); ihre flächenbezogene Wärmekapazität ist die einer 0,3 mm dünnen, wasserreichen Schicht, eine Autorenschätzung. Die Transpiration der Blütenblätter wird vernachlässigt.")]
    S.structure_assumptions = tree_structure_assumptions("cherry", "Kirsche")
    share = {m.id: m.area() for m in sunm}
    tot = sum(share.values())

    def blossom(total_kg):
        return fruit_extra({k: total_kg * v / tot for k, v in share.items()}, "cherry-blossom-load")

    def both(*fs):
        return lambda S: [x for f in fs for x in f(S)]

    S.poses = [
        {"id": "full-bloom", "title": T("Full bloom, still air", "Vollblüte, Windstille"), "desc": T("A mild April day with a light air of 1.5 m/s at 10 m: the crown is in full bloom with about 24 kg of double flowers and half-expanded bronze leaves; the scaffolds hardly move.", "Ein milder Apriltag mit leisem Zug von 1,5 m/s in 10 m Höhe: Die Krone steht in voller Blüte mit etwa 24 kg gefüllter Blüten und halb entfaltetem bronzefarbenem Laub; die Leitäste bewegen sich kaum."), "v10": 1.5, "sun": 1.0, "emit": 0.6, "droop": 0.1, "extra": blossom(BLOSSOM_KG)},
        {"id": "bloom-breeze", "title": T("Breeze in full bloom", "Brise in voller Blüte"), "desc": T("Moderate breeze of 5.5 m/s at 10 m (Beaufort 4) in full bloom: the arching scaffold ends lean downwind, the first petals drift; drag by the reconfigured law (cd 0.35).", "Mäßige Brise von 5,5 m/s in 10 m Höhe (Beaufort 4) in voller Blüte: Die überbiegenden Leitastenden neigen sich in Windrichtung, die ersten Blütenblätter treiben davon; Widerstand nach dem umgeformten Gesetz (cd 0,35)."), "v10": 5.5, "sun": 1.0, "emit": 0.7, "droop": 0.15, "extra": blossom(BLOSSOM_KG)},
        {"id": "spring-gale", "title": T("Spring gale with petal fall", "Frühjahrssturm mit Blütenfall"), "desc": T("Beaufort 8 to 9, 20 m/s at 10 m with showers: strong lean of the whole funnel crown, wet bark and petals; about 10 kg of flowers are already torn off, 14 kg remain.", "Beaufort 8 bis 9, 20 m/s in 10 m Höhe mit Schauern: starke Neigung der ganzen Trichterkrone, nasse Rinde und Blüten; etwa 10 kg Blüten sind schon abgerissen, 14 kg hängen noch."), "v10": 20.0, "sun": 0.2, "emit": 0.3, "wet": 0.5, "droop": 0.15, "extra": blossom(14.0)},
        {"id": "april-snow", "title": T("Late wet snow on the blossom", "Später Nassschnee auf der Blüte"), "desc": T("A cold April night after a late snowfall: about 7 kg/m2 of wet snow lies on the blossom-laden foliage masses and bends the arching scaffold ends down; the leaves hardly transpire. Wet snow on a crown in bloom is a known cause of limb breakage in flowering cherries.", "Eine kalte Aprilnacht nach spätem Schneefall: Etwa 7 kg/m2 Nassschnee liegen auf den blütenbeladenen Laubmassen und biegen die überbiegenden Leitastenden nach unten; die Blätter transpirieren kaum. Nassschnee auf einer blühenden Krone ist eine bekannte Ursache für Astbruch bei Blütenkirschen."), "v10": 1.0, "sun": 0.25, "emit": 0.1, "wet": 1.0, "droop": 0.9, "extra": both(blossom(BLOSSOM_KG), snow_extra(7.0))},
    ]
    S.make_reps = skeleton_reps
    S.make_symbols = lambda S, b: []
    return S
