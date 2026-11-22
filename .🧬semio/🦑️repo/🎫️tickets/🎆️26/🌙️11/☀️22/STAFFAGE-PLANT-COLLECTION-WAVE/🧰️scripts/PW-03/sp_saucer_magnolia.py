"""🌷️ saucer-magnolia: a mature multi-stem Magnolia × soulangeana in late bloom, four smooth grey stems from one root crown, spreading low branches and upright goblet flowers; every number is authored here.
@see ./pw03_tree.py"""
from pw03_tree import *
from common import fruit_extra, snow_extra
from sp_plane import Sp, common_refs, tree_thermal_assumptions, tree_structure_assumptions

W = "magnolia-wood"
FLOWER_KG = 35.0
STEMS = [(1, 25, 8, 1), (2, 118, -10, 1), (3, 205, 10, 1), (4, 295, -8, 1)]


def stems_wind(v10):
    def f(S):
        out = []
        for k in (2, 3, 4):
            b = S.rig.bones["s%da" % k]
            wx = 0.5 * 1.25 * 1.0 * b.d0 * (v10 * (max(b.L, 1.0) / 10) ** 0.22) ** 2
            out.append({"id": "wind-stem-%d" % k, "kind": "wind", "title": T("Drag on stem %d" % k, "Widerstand an Stamm %d" % k), "fxN": 0, "fyN": 0, "distributed": {"bone": b.id, "fromM": 0, "toM": round(b.L, 3), "wxNPerM": round(wx, 2), "wyNPerM": 0}, "source": "literature", "reference": "en-1991-1-4"})
        return out
    return f


def build():
    S = Sp()
    S.id = S.folder = "saucer-magnolia"
    S.title = T("Saucer Magnolia, Multi-Stem, in Bloom", "Tulpen-Magnolie, mehrstämmig, in Blüte")
    S.summary = T("A mature saucer magnolia (Magnolia × soulangeana) of four smooth grey stems from one root crown, low and wider than high, in late bloom: upright goblet flowers of 14 cm, white inside and flushed purple-pink outside, stand on the branch tips while the first leaves unfold.",
                  "Eine ausgewachsene Tulpen-Magnolie (Magnolia × soulangeana) aus vier glatten grauen Stämmen aus einem Wurzelstock, niedrig und breiter als hoch, in später Blüte: aufrechte, kelchförmige Blüten von 14 cm, innen weiß und außen purpurrosa überhaucht, stehen auf den Zweigspitzen, während sich die ersten Blätter entfalten.")
    S.tags = ["tree", "large-shrub", "deciduous", "ornamental", "flowering-tree", "multi-stem", "blossom", "wind", "transpiration"]
    S.depth_m = 6.6
    S.footprint = 1.4
    S.density = 850
    S.root_mass_factor = 1.0
    S.default_scale = 100
    S.default_pose = "late-bloom"
    S.breeze_pose = "bloom-breeze"
    S.breeze_v10 = 5.5
    S.breeze_droop = 0.15
    S.gust_peak = 14.0
    S.trunk_bone = "s1a"
    S.twig_factor = {"leaf-sun": 1.6, "leaf-shade": 1.6}
    S.cd = {}
    S.bark_extra = {}
    S.exchange_regions = {"flower"}
    t = Tree(850)
    S.tree = t
    S.rig = t.rig
    t.stem("root-plate", None, [(0.0, 0.5, 0, 0), (0.3, 0.34, 0, 0)], T("Root crown of the four stems", "Wurzelstock der vier Stämme"), 0, W, "stem-bark", "trunk", "root")
    t.rig.bones["root-plate"].kg *= 1.8
    for k, az, bend, side in STEMS:
        base = (0.13 * math.cos(az * D2R), 0.22, 0.13 * math.sin(az * D2R))
        t.limb("s%da" % k, "root-plate", base, az, 74, 70, 2.3, 0.105, 0.085, T("Stem %d, lower part" % k, "Stamm %d, unterer Teil" % k), 4, W, "stem-bark", "stem", "stem")
        t.limb("s%db" % k, "s%da" % k, ("s%da" % k, 1.0), az + bend * 0.3, 66, 52, 2.2, 0.08, 0.036, T("Stem %d, upper part" % k, "Stamm %d, oberer Teil" % k), 9, W, "limb-bark", "limb", "limb", bend=bend)
        t.limb("br-%d" % k, "s%da" % k, ("s%da" % k, 0.42), az + 45 * side, 30, 12, 2.1, 0.055, 0.022, T("Low spreading branch on stem %d" % k, "Tiefer ausladender Ast an Stamm %d" % k), 12, W, "limb-bark", "limbb", "limb", bend=-12 * side)
    S.levels = t.levels
    S.lean_ref = {"trunk": 0.3, "stem": 0.55, "limb": 1.0, "limbb": 1.4, "fol": 3.2}
    S.droop_levels = {"limb": 1.2, "limbb": 2.4, "fol": 3.0}
    S.sway_amp = {"trunk": 0.5, "stem": 0.5, "limb": 0.6, "limbb": 0.65, "fol": 0.7}
    S.sway_lag = 0.5
    S.sway_period = 2.2
    S.gust_tau = {"trunk": 0.6, "stem": 0.5, "limb": 0.4, "limbb": 0.35, "fol": 0.2}
    S.sway_desc_en = "A deterministic %s s sway cycle about the breeze lean in late bloom (5.5 m/s at 10 m; estimated first mode of the 6 m multi-stem tree about 0.45 Hz). The four stems and their low branches swing with a level-wise lag; the flowering foliage masses swing about 3 degrees."
    S.sway_desc_de = "Ein deterministischer Schwingungszyklus von %s s um die Neigung bei Brise in später Blüte (5,5 m/s in 10 m Höhe; geschätzte erste Eigenform des 6 m hohen mehrstämmigen Baums etwa 0,45 Hz). Die vier Stämme und ihre tiefen Äste schwingen ebenenweise verzögert; die blühenden Laubmassen schwingen etwa 3 Grad."
    S.gust_desc_en = "A single spring gust over 8 s from %.1f to %.1f m/s at 10 m in late bloom: the wind-load tracks rise with the reconfigured drag law, stems and branches lean downwind with a level-wise lag."
    S.gust_desc_de = "Eine einzelne Frühjahrsböe über 8 s von %.1f auf %.1f m/s in 10 m Höhe in später Blüte: Die Windlast-Spuren steigen mit dem umgeformten Widerstandsgesetz, Stämme und Äste neigen sich ebenenweise verzögert in Windrichtung."
    S.masses = []
    for k, az, bend, side in STEMS:
        a = az + bend * 0.5
        S.masses.append(t.mass("fol-%d" % k, "s%db" % k, (2.0 * math.cos(a * D2R), 4.55, 2.0 * math.sin(a * D2R)), 1.25, 1.05, "leaf-sun", "sun", T("Upper flowering mass over stem %d" % k, "Obere Blühmasse über Stamm %d" % k)))
        b = az + 32 * side
        S.masses.append(t.mass("fol-l%d" % k, "br-%d" % k, (2.15 * math.cos(b * D2R), 2.2, 2.15 * math.sin(b * D2R)), 1.2, 0.85, "leaf-sun", "sun", T("Low flowering mass on branch %d" % k, "Tiefe Blühmasse an Ast %d" % k)))
    S.masses.append(t.mass("fol-core", "s1b", (0.15, 3.5, 0.1), 1.7, 1.1, "leaf-shade", "shade", T("Shaded inner crown", "Beschattete Innenkrone")))
    proj = math.pi / 4 * 6.8 * S.depth_m
    S.leaf_area = {"leaf-sun": 0.75 * proj, "leaf-shade": 0.45 * proj}
    S.tone = {"wood": 0.42, "sun": 0.6, "shade": 0.45}
    S.tone_line = 0.2
    S.sun = {"stem-bark": 0.35, "limb-bark": 0.35, "leaf-sun": 0.85, "leaf-shade": 0.1, "flower": 0.9}
    S.regions = [
        {"id": "stem-bark", "title": T("Smooth silver-grey bark of the four stems and the root crown", "Glatte silbergraue Rinde der vier Stämme und des Wurzelstocks"), "kind": "bark", "model": "passive", "material": "magnolia-bark", "len": 0.2, "geom": "cylinder", "sky": 0.5, "hc": 45000, "ref": "wood-handbook-2021"},
        {"id": "limb-bark", "title": T("Smooth grey bark of the upper stems and branches", "Glatte graue Rinde der oberen Stämme und Äste"), "kind": "bark", "model": "passive", "material": "magnolia-bark-young", "len": 0.1, "geom": "cylinder", "sky": 0.55, "hc": 30000, "ref": "wood-handbook-2021"},
        {"id": "leaf-sun", "title": T("Unfolding young leaves of the outer crown", "Sich entfaltende junge Blätter der Außenkrone"), "kind": "foliage", "model": "leaf", "material": "magnolia-leaf-sun", "area": S.leaf_area["leaf-sun"], "len": 0.12, "geom": "vertical-plate", "sky": 0.7, "hc": 700, "gs": 0.0035, "lai": 0.75, "ref": "jones-2014"},
        {"id": "leaf-shade", "title": T("Young shade leaves of the inner crown", "Junge Schattenblätter der Innenkrone"), "kind": "foliage", "model": "leaf", "material": "magnolia-leaf-shade", "area": S.leaf_area["leaf-shade"], "len": 0.13, "geom": "vertical-plate", "sky": 0.3, "hc": 650, "gs": 0.002, "lai": 0.45, "ref": "jones-2014"},
        {"id": "flower", "title": T("Tepals of the goblet flowers", "Blütenhüllblätter der Kelchblüten"), "kind": "surface", "model": "passive", "material": "magnolia-tepal", "area": 108.0, "len": 0.1, "geom": "vertical-plate", "sky": 0.6, "hc": 2400, "ref": "callaway-1994"},
    ]
    sunm = [m for m in S.masses if m.kind == "sun"]
    S.emitters = [
        {"id": "transpiration-sun", "aid": "tp-sun", "region": "leaf-sun", "strength": 1.0, "diam": 1.8, "title": T("Transpiration of the young sun leaves", "Transpiration der jungen Sonnenblätter"), "anchors": [(m.id, (m.c[0], m.c[1] + 0.2)) for m in sunm]},
        {"id": "transpiration-shade", "aid": "tp-shade", "region": "leaf-shade", "strength": 0.6, "diam": 1.4, "title": T("Transpiration of the shade leaves", "Transpiration der Schattenblätter"), "anchors": [("fol-core", (0.15 + dx, 3.5 + dy)) for dx, dy in [(-0.8, 0.0), (0.8, 0.1), (0.0, 0.5), (-0.4, -0.4), (0.5, -0.3)]]},
    ]
    S.materials = [
        {"id": "magnolia-bark", "title": T("Magnolia bark, smooth silver-grey", "Magnolienrinde, glatt silbergrau"), "emissivity": 0.94, "solarAbsorptivity": 0.6, "densityKgM3": 600, "conductivityWmK": 0.12, "source": "authored-estimate", "reference": "wood-handbook-2021"},
        {"id": "magnolia-bark-young", "title": T("Smooth grey bark of the branches", "Glatte graue Rinde der Äste"), "emissivity": 0.94, "solarAbsorptivity": 0.62, "densityKgM3": 650, "conductivityWmK": 0.13, "source": "authored-estimate", "reference": "wood-handbook-2021"},
        {"id": "magnolia-leaf-sun", "title": T("Unfolding magnolia leaves", "Sich entfaltende Magnolienblätter"), "emissivity": 0.97, "solarAbsorptivity": 0.5, "source": "authored-estimate", "reference": "jones-2014"},
        {"id": "magnolia-leaf-shade", "title": T("Young magnolia shade leaves (effective)", "Junge Magnolien-Schattenblätter (wirksam)"), "emissivity": 0.97, "solarAbsorptivity": 0.26, "source": "authored-estimate", "reference": "jones-2014"},
        {"id": "magnolia-tepal", "title": T("Fleshy tepals, white inside and purple-pink outside", "Fleischige Blütenhüllblätter, innen weiß und außen purpurrosa"), "emissivity": 0.95, "solarAbsorptivity": 0.38, "densityKgM3": 950, "conductivityWmK": 0.45, "specificHeatJKgK": 3900, "source": "authored-estimate", "reference": "callaway-1994"},
        {"id": W, "title": T("Green magnolia wood", "Frisches Magnolienholz"), "emissivity": 0.9, "solarAbsorptivity": 0.62, "densityKgM3": 850, "conductivityWmK": 0.15, "elasticModulusPa": 10000000000, "strengthPa": 50000000, "source": "authored-estimate", "reference": "wood-handbook-2021"},
    ]
    S.references = common_refs() + [
        {"id": "callaway-1994", "citation": "Callaway, D. J. (1994). The World of Magnolias. Timber Press, Portland. Magnolia x soulangeana: a large multi-stemmed shrub or small tree of about 6 to 9 m, usually wider than high, with smooth grey bark; goblet-shaped flowers of nine tepals, white inside and flushed rose-purple outside, opening before and with the young leaves in early spring; leaves obovate, 10 to 15 cm."},
        {"id": "magnolia-flower-load", "citation": "Authored estimate for this asset: about 1000 open flowers of about 35 g fresh mass (35 kg) with nine tepals of about 60 cm2 each side, an exchange area of about 108 m2; not a measured count."},
    ]
    S.notes = T("Rig and drawings are authored by hand for a 35-year-old saucer magnolia grown as a multi-stem specimen on a lawn: four stems from one root crown leaning out at 16 to 20 degrees from the vertical, each with a low spreading branch, crown about 6 m high and 6.5 m wide. The asset is the late flowering state: the upright flowers and the half-unfolded leaves are drawn in every pose; the poses differ in wind, wetness, snow and flower load. Flower and leaf quantities are authored estimates.",
                  "Rig und Zeichnungen sind von Hand für eine 35-jährige Tulpen-Magnolie entworfen, die als mehrstämmiger Solitär auf einem Rasen gewachsen ist: vier Stämme aus einem Wurzelstock, 16 bis 20 Grad gegen die Senkrechte nach außen geneigt, jeder mit einem tiefen ausladenden Ast, Krone etwa 6 m hoch und 6,5 m breit. Das Asset ist der späte Blühzustand: Die aufrechten Blüten und die halb entfalteten Blätter sind in jeder Pose gezeichnet; die Posen unterscheiden sich in Wind, Nässe, Schnee und Blütenlast. Blüten- und Blattmengen sind Autorenschätzungen.")
    S.environment = {"airTempC": 14, "meanRadiantTempC": 17, "relativeHumidity": 0.6, "windMS": 1.5, "windDirectionDeg": 0, "solarWM2": 580, "pressurePa": 101325, "diffuseFraction": 0.25}
    S.thermal_assumptions = tree_thermal_assumptions("", "") + [T("The tepals of the open flowers are one passive surface with the exchange area of about 1000 flowers (108 m2, both faces); their areal heat capacity is that of a 0.6 mm fleshy, water-rich tepal, an authored estimate. Flower transpiration is neglected.", "Die Blütenhüllblätter der offenen Blüten sind eine passive Fläche mit der Austauschfläche von etwa 1000 Blüten (108 m2, beide Seiten); ihre flächenbezogene Wärmekapazität ist die eines 0,6 mm dicken, fleischigen, wasserreichen Hüllblatts, eine Autorenschätzung. Die Transpiration der Blüten wird vernachlässigt.")]
    S.structure_assumptions = tree_structure_assumptions("magnolia", "Magnolie") + [T("The four stems are fixed together at the root crown; the wind drag on the lower parts of stems 2 to 4 is a line load of its own, like that of stem 1.", "Die vier Stämme sind im Wurzelstock gemeinsam eingespannt; der Winddruck auf die unteren Teile der Stämme 2 bis 4 ist eine eigene Linienlast wie der von Stamm 1.")]
    share = {m.id: m.area() for m in sunm}
    tot = sum(share.values())

    def flowers(total_kg):
        return fruit_extra({k: total_kg * v / tot for k, v in share.items()}, "magnolia-flower-load")

    def both(*fs):
        return lambda S: [x for f in fs for x in f(S)]

    S.poses = [
        {"id": "late-bloom", "title": T("Late bloom, still air", "Späte Blüte, Windstille"), "desc": T("A mild April day with a light air of 1.5 m/s at 10 m: about 1000 goblet flowers (35 kg) stand open on the branch tips, the leaves are half unfolded and transpire at about half their summer rate.", "Ein milder Apriltag mit leisem Zug von 1,5 m/s in 10 m Höhe: Etwa 1000 Kelchblüten (35 kg) stehen offen auf den Zweigspitzen, die Blätter sind halb entfaltet und transpirieren mit etwa der Hälfte ihrer Sommerrate."), "v10": 1.5, "sun": 1.0, "emit": 0.5, "droop": 0.1, "extra": both(flowers(FLOWER_KG), stems_wind(1.5))},
        {"id": "bloom-breeze", "title": T("Breeze in late bloom", "Brise in später Blüte"), "desc": T("Moderate breeze of 5.5 m/s at 10 m (Beaufort 4): the upper stems and the low branches lean downwind, the flowers rock on their short stalks; drag by the reconfigured law (cd 0.35).", "Mäßige Brise von 5,5 m/s in 10 m Höhe (Beaufort 4): Die oberen Stämme und die tiefen Äste neigen sich in Windrichtung, die Blüten schaukeln auf ihren kurzen Stielen; Widerstand nach dem umgeformten Gesetz (cd 0,35)."), "v10": 5.5, "sun": 1.0, "emit": 0.6, "droop": 0.15, "extra": both(flowers(FLOWER_KG), stems_wind(5.5))},
        {"id": "spring-gale", "title": T("Spring gale with tepal fall", "Frühjahrssturm mit Blütenblattfall"), "desc": T("Beaufort 8 to 9, 20 m/s at 10 m with showers: strong lean of the whole crown, wet bark, leaves and tepals; about 15 kg of flowers are shattered, 20 kg remain.", "Beaufort 8 bis 9, 20 m/s in 10 m Höhe mit Schauern: starke Neigung der ganzen Krone, nasse Rinde, Blätter und Hüllblätter; etwa 15 kg Blüten sind zerschlagen, 20 kg bleiben."), "v10": 20.0, "sun": 0.2, "emit": 0.3, "wet": 0.5, "droop": 0.15, "extra": both(flowers(20.0), stems_wind(20.0))},
        {"id": "wet-snow", "title": T("Wet snow on the flowering crown", "Nassschnee auf der blühenden Krone"), "desc": T("A cold night after a late snowfall: about 8 kg/m2 of wet snow lies on the flowering masses and presses the low branches down; the leaves hardly transpire. Late snow on the brittle, spreading branches of a flowering magnolia is a known cause of breakage.", "Eine kalte Nacht nach spätem Schneefall: Etwa 8 kg/m2 Nassschnee liegen auf den Blühmassen und drücken die tiefen Äste nach unten; die Blätter transpirieren kaum. Später Schnee auf den spröden, ausladenden Ästen einer blühenden Magnolie ist eine bekannte Ursache für Bruch."), "v10": 1.0, "sun": 0.25, "emit": 0.1, "wet": 1.0, "droop": 0.9, "extra": both(flowers(FLOWER_KG), snow_extra(8.0), stems_wind(1.0))},
    ]
    S.make_reps = skeleton_reps
    S.make_symbols = lambda S, b: []
    return S
