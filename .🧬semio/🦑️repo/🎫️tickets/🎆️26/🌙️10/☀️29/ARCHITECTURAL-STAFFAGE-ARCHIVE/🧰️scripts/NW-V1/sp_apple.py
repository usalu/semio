"""apple-tree: full-standard orchard apple, open vase crown, fruit load as seasonal pose and load."""
import math
from lib import *
from common import *
from reps import *
from gen import *
from sp_plane import Sp, common_refs, tree_thermal_assumptions, tree_structure_assumptions
from sp_lime import on


def ovate(cx, cy, R, rot=0.0):
    base = [(0, 1.0), (0.5, 0.5), (0.55, -0.2), (0.0, -1.0), (-0.55, -0.2), (-0.5, 0.5)]
    return rot_pts([(cx + R * x, cy + R * y) for x, y in base], cx, cy, rot)


FRUIT_AT = [(0.55, 250), (0.62, 300), (0.5, 200), (0.65, 330), (0.45, 270), (0.6, 225)]
FRUIT_R = 0.04
FRUIT_KG_FULL = 150.0


def build():
    S = Sp()
    S.id = "apple-tree"
    S.folder = "apple-tree"
    S.title = T("Apple Tree, Orchard Standard", "Apfelbaum, Hochstamm")
    S.summary = T("A full-standard orchard apple tree with a clear stem of 1.6 m and an open, spreading vase crown of arching scaffold limbs; the fruit load is carried as a seasonal pose and load, from green fruit set to the drooping harvest load.",
                  "Ein Hochstamm-Obstbaum (Apfel) mit 1,6 m Stammhöhe und offener, ausladender Vasenkrone aus gebogenen Leitästen; die Fruchtlast wird als jahreszeitliche Pose und Last geführt, vom grünen Fruchtansatz bis zur Erntelast, die die Äste nach unten biegt.")
    S.tags = ["tree", "deciduous", "orchard", "fruit-tree", "fruit-load", "wind", "transpiration"]
    S.depth_m = 6.2
    S.footprint = 1.4
    S.density = 900
    S.root_mass_factor = 1.25
    S.default_scale = 100
    S.default_pose = "summer-fruit-set"
    S.breeze_pose = "breeze-with-fruit"
    S.breeze_v10 = 5.5
    S.breeze_droop = 0.6
    S.gust_peak = 14.0
    S.trunk_bone = "trunk"
    S.twig_factor = {"leaf-sun": 1.5, "leaf-shade": 1.5}
    S.cd = {}
    S.bark_extra = {}
    r = Rig()
    S.rig = r
    W = "apple-wood"
    r.add("root-plate", None, (0, 0), 90, 0.35, T("Root plate and flare", "Wurzelteller und Wurzelanlauf"), 0.6, 0.45, limit=0, material=W).region = "trunk-bark"
    r.add("trunk", "root-plate", (0, 0.35), 90, 1.25, T("Clear stem", "Stamm"), 0.38, 0.34, limit=3.5, material=W).region = "trunk-bark"
    r.add("ld", "trunk", r.tip("trunk"), 90.5, 2.2, T("Leader, lower part", "Stammverlängerung, unterer Teil"), 0.30, 0.16, limit=5, material=W).region = "trunk-bark"
    r.add("ld-b", "ld", r.tip("ld"), 91, 1.5, T("Leader, upper part", "Stammverlängerung, oberer Teil"), 0.16, 0.07, limit=8, material=W).region = "limb-bark"
    SC = [("sc-w1a", "trunk", 1.2, 140, 1.8, 0.20, 0.13, "Lower west scaffold", "Unterer Westleitast"), ("sc-w1b", "sc-w1a", None, 168, 1.6, 0.13, 0.07, "Lower west scaffold, outer part", "Unterer Westleitast, Außenteil"),
          ("sc-e1a", "trunk", 1.25, 38, 1.9, 0.20, 0.13, "Lower east scaffold", "Unterer Ostleitast"), ("sc-e1b", "sc-e1a", None, 12, 1.6, 0.13, 0.07, "Lower east scaffold, outer part", "Unterer Ostleitast, Außenteil"),
          ("sc-w2a", "ld", 0.5, 118, 2.2, 0.17, 0.10, "Upper west scaffold", "Oberer Westleitast"), ("sc-w2b", "sc-w2a", None, 135, 1.5, 0.10, 0.05, "Upper west scaffold, outer part", "Oberer Westleitast, Außenteil"),
          ("sc-e2a", "ld", 0.7, 64, 2.3, 0.17, 0.10, "Upper east scaffold", "Oberer Ostleitast"), ("sc-e2b", "sc-e2a", None, 48, 1.5, 0.10, 0.05, "Upper east scaffold, outer part", "Oberer Ostleitast, Außenteil")]
    for id, par, s, ang, ln, d0, d1, te, td in SC:
        base = r.tip(par) if s is None else on(r, par, s)
        r.add(id, par, base, ang, ln, T(te, td), d0, d1, limit=10 if id.endswith("a") else 14, material=W).region = "limb-bark"
    SP = [("spur-w1", "sc-w1b", 0.8, 100, 0.6, "west lower", "West unten"), ("spur-e1", "sc-e1b", 0.8, 80, 0.6, "east lower", "Ost unten"), ("spur-w2", "sc-w2b", 0.7, 120, 0.55, "west upper", "West oben"), ("spur-e2", "sc-e2b", 0.7, 60, 0.55, "east upper", "Ost oben")]
    for id, par, s, ang, ln, te, td in SP:
        r.add(id, par, on(r, par, s * r.bones[par].L), ang, ln, T("Fruiting spurs, %s" % te, "Fruchtholz, %s" % td), 0.02, 0.012, limit=25, material=W).region = "spur-twig"
    S.levels = {"trunk": "trunk", "ld": "trunk", "ld-b": "upper"}
    for id, *_ in SC:
        S.levels[id] = "limb" if id.endswith("a") else "limbb"
    for id, *_ in SP:
        S.levels[id] = "spur"
    S.lean_ref = {"trunk": 0.3, "upper": 0.6, "limb": 0.8, "limbb": 1.3, "spur": 2.0, "fol": 3.5}
    S.droop_levels = {"limb": 2.0, "limbb": 3.5, "spur": 4.0, "fol": 4.5, "upper": 0.8}
    S.sway_amp = {"trunk": 0.5, "upper": 0.5, "limb": 0.55, "limbb": 0.6, "spur": 0.6, "fol": 0.7}
    S.sway_lag = 0.45
    S.sway_period = 2.4
    S.gust_tau = {"trunk": 0.6, "upper": 0.45, "limb": 0.4, "limbb": 0.35, "spur": 0.3, "fol": 0.2}
    S.sway_desc_en = "A deterministic %s s sway cycle about the breeze lean with a half-ripe fruit load (5.5 m/s at 10 m; estimated first mode of the 6 m tree about 0.4 Hz). The arching scaffolds bob in anti-phase to the stem; foliage masses swing about 3 degrees."
    S.sway_desc_de = "Ein deterministischer Schwingungszyklus von %s s um die Neigung bei Brise mit halber Fruchtlast (5,5 m/s in 10 m Höhe; geschätzte erste Eigenform des 6 m hohen Baums etwa 0,4 Hz). Die gebogenen Leitäste wippen gegenphasig zum Stamm; die Laubmassen schwingen etwa 3 Grad."
    S.gust_desc_en = "A single gust over 8 s from %.1f to %.1f m/s at 10 m with the fruit load of the breeze pose: the wind-load tracks rise with the reconfigured drag law, the scaffolds lean downwind with a level-wise lag."
    S.gust_desc_de = "Eine einzelne Böe über 8 s von %.1f auf %.1f m/s in 10 m Höhe mit der Fruchtlast der Brisenpose: Die Windlast-Spuren steigen mit dem umgeformten Widerstandsgesetz, die Leitäste neigen sich ebenenweise verzögert in Windrichtung."
    S.masses = [
        Mass("fol-core-low", "ld", (0.0, 3.7), 1.5, 1.1, 0, "leaf-shade", "shade", T("shaded lower core", "beschatteter Kern unten")),
        Mass("fol-core-up", "ld-b", (0.0, 4.7), 1.1, 0.95, 0, "leaf-shade", "shade", T("shaded upper core", "beschatteter Kern oben")),
        Mass("fol-w1", "sc-w1b", (-2.2, 3.1), 1.3, 0.95, 5, "leaf-sun", "sun", T("lower west foliage", "Laubmasse unten West")),
        Mass("fol-e1", "sc-e1b", (2.2, 3.2), 1.3, 0.9, -5, "leaf-sun", "sun", T("lower east foliage", "Laubmasse unten Ost")),
        Mass("fol-w2", "sc-w2b", (-1.7, 4.7), 1.2, 1.05, 8, "leaf-sun", "sun", T("upper west foliage", "Laubmasse oben West")),
        Mass("fol-e2", "sc-e2b", (1.65, 4.9), 1.2, 1.05, -8, "leaf-sun", "sun", T("upper east foliage", "Laubmasse oben Ost")),
        Mass("fol-top", "ld-b", (0.15, 5.4), 1.25, 0.95, 0, "leaf-sun", "sun", T("crown top foliage", "Laubmasse Kronenspitze")),
    ]
    for m in S.masses:
        m.c = (m.c[0] * 0.93, m.c[1])
        m.rx *= 1.1
        m.ry *= 1.08
    proj = math.pi / 4 * 6.9 * S.depth_m
    S.leaf_area = {"leaf-sun": 2.0 * proj, "leaf-shade": 1.8 * proj}
    S.mat_of = {"trunk-bark": "apple-bark", "limb-bark": "apple-bark-young", "spur-twig": "apple-twig", "leaf-sun": "apple-leaf-sun", "leaf-shade": "apple-leaf-shade", "fruit": "apple-fruit"}
    S.tone = {"wood": 0.38, "sun": 0.6, "shade": 0.45}
    S.tone_line = 0.2
    S.sun = {"trunk-bark": 0.4, "limb-bark": 0.3, "spur-twig": 0.3, "leaf-sun": 0.9, "leaf-shade": 0.1, "fruit": 0.55}
    S.regions = [
        {"id": "trunk-bark", "title": T("Scaly grey-brown bark of stem and leader", "Schuppige graubraune Rinde von Stamm und Stammverlängerung"), "kind": "bark", "model": "passive", "material": "apple-bark", "len": 0.35, "geom": "cylinder", "sky": 0.45, "hc": 60000, "ref": "wood-handbook-2021"},
        {"id": "limb-bark", "title": T("Bark of scaffold limbs and upper leader", "Rinde von Leitästen und oberer Stammverlängerung"), "kind": "bark", "model": "passive", "material": "apple-bark-young", "len": 0.14, "geom": "cylinder", "sky": 0.55, "hc": 40000, "ref": "wood-handbook-2021"},
        {"id": "spur-twig", "title": T("Fruiting spurs and twigs", "Fruchtholz und Zweige"), "kind": "stem", "model": "passive", "material": "apple-twig", "len": 0.02, "geom": "cylinder", "sky": 0.6, "hc": 12000, "ref": "wood-handbook-2021"},
        {"id": "leaf-sun", "title": T("Sun leaves of the outer crown", "Sonnenblätter der Außenkrone"), "kind": "foliage", "model": "leaf", "material": "apple-leaf-sun", "area": S.leaf_area["leaf-sun"], "len": 0.06, "geom": "vertical-plate", "sky": 0.7, "hc": 850, "gs": 0.0055, "lai": 2.0, "ref": "jones-2014"},
        {"id": "leaf-shade", "title": T("Shade leaves of the inner crown", "Schattenblätter der Innenkrone"), "kind": "foliage", "model": "leaf", "material": "apple-leaf-shade", "area": S.leaf_area["leaf-shade"], "len": 0.07, "geom": "vertical-plate", "sky": 0.3, "hc": 650, "gs": 0.003, "lai": 1.8, "ref": "jones-2014"},
        {"id": "fruit", "title": T("Skin of the apples", "Schale der Äpfel"), "kind": "surface", "model": "passive", "material": "apple-fruit", "area": 15.0, "len": 0.08, "geom": "sphere", "sky": 0.5, "hc": 40000, "ref": "jones-2014"},
    ]
    sunm = [m for m in S.masses if m.kind == "sun"]
    S.emitters = [
        {"id": "transpiration-sun", "aid": "tp-sun", "region": "leaf-sun", "strength": 1.0, "diam": 2.0, "title": T("Transpiration of the sun leaves", "Transpiration der Sonnenblätter"),
         "anchors": [(m.id, (m.c[0], m.c[1] + 0.15)) for m in sunm] + [("fol-w1", (S.masses[2].c[0] - 0.6, S.masses[2].c[1])), ("fol-e1", (S.masses[3].c[0] + 0.6, S.masses[3].c[1]))]},
        {"id": "transpiration-shade", "aid": "tp-shade", "region": "leaf-shade", "strength": 0.7, "diam": 1.5, "title": T("Transpiration of the shade leaves", "Transpiration der Schattenblätter"),
         "anchors": [("fol-core-low", (-0.4, 3.4)), ("fol-core-low", (0.3, 3.8)), ("fol-core-low", (0.0, 4.1)), ("fol-core-up", (-0.2, 4.6)), ("fol-core-up", (0.25, 4.9))]},
    ]
    S.materials = [
        {"id": "apple-bark", "title": T("Apple bark, scaly and cracked", "Apfelrinde, schuppig und rissig"), "emissivity": 0.95, "solarAbsorptivity": 0.7, "densityKgM3": 650, "conductivityWmK": 0.12, "source": "authored-estimate", "reference": "wood-handbook-2021"},
        {"id": "apple-bark-young", "title": T("Smoother bark of the scaffold limbs", "Glattere Rinde der Leitäste"), "emissivity": 0.95, "solarAbsorptivity": 0.62, "densityKgM3": 700, "conductivityWmK": 0.13, "source": "authored-estimate", "reference": "wood-handbook-2021"},
        {"id": "apple-twig", "title": T("Brown bark of spurs and twigs", "Braune Rinde von Fruchtholz und Zweigen"), "emissivity": 0.95, "solarAbsorptivity": 0.6, "densityKgM3": 750, "conductivityWmK": 0.14, "source": "authored-estimate", "reference": "wood-handbook-2021"},
        {"id": "apple-leaf-sun", "title": T("Apple sun leaves", "Apfel-Sonnenblätter"), "emissivity": 0.97, "solarAbsorptivity": 0.5, "source": "literature", "reference": "jones-2014"},
        {"id": "apple-leaf-shade", "title": T("Apple shade leaves (effective)", "Apfel-Schattenblätter (wirksam)"), "emissivity": 0.97, "solarAbsorptivity": 0.25, "source": "authored-estimate", "reference": "jones-2014"},
        {"id": "apple-fruit", "title": T("Apple skin (waxy, water-rich fruit)", "Apfelschale (wachsig, wasserreiche Frucht)"), "emissivity": 0.95, "solarAbsorptivity": 0.6, "densityKgM3": 840, "conductivityWmK": 0.4, "specificHeatJKgK": 3600, "source": "authored-estimate", "reference": "jones-2014"},
        {"id": W, "title": T("Green apple wood", "Frisches Apfelholz"), "emissivity": 0.9, "solarAbsorptivity": 0.65, "densityKgM3": 900, "conductivityWmK": 0.16, "elasticModulusPa": 7000000000, "strengthPa": 45000000, "source": "authored-estimate", "reference": "wood-handbook-2021"},
    ]
    S.references = common_refs()
    S.notes = T("Rig and drawings are authored by hand for a 30-year-old full-standard orchard apple on a vigorous seedling rootstock: clear stem 1.6 m, two scaffold tiers, crown about 7 m wide. A mature standard bears roughly 100 to 200 kg of apples a year; the 150 kg of the harvest pose and the fruit counts are authored estimates (apples of about 0.2 kg), not measured yields. The drawn fruit are a sample of the load, not a count.",
                 "Rig und Zeichnungen sind von Hand für einen 30-jährigen Hochstamm-Apfelbaum auf starkwüchsiger Sämlingsunterlage entworfen: Stamm 1,6 m, zwei Leitastetagen, Krone etwa 7 m breit. Ein ausgewachsener Hochstamm trägt grob 100 bis 200 kg Äpfel im Jahr; die 150 kg der Erntepose und die Fruchtzahlen sind Autorenschätzungen (Äpfel von etwa 0,2 kg), keine gemessenen Erträge. Die gezeichneten Früchte sind eine Auswahl der Last, keine Zählung.")
    S.environment = {"airTempC": 22, "meanRadiantTempC": 26, "relativeHumidity": 0.55, "windMS": 1.5, "windDirectionDeg": 0, "solarWM2": 700, "pressurePa": 101325, "diffuseFraction": 0.15}
    S.thermal_assumptions = tree_thermal_assumptions("", "") + [T("The apple skin is a passive surface of the whole fruit load (about 750 apples of 8 cm diameter, 15 m2); its areal heat capacity is that of a water-rich sphere (840 kg/m3, 3600 J/(kg K), radius/3), an authored estimate.", "Die Apfelschale ist eine passive Fläche der gesamten Fruchtlast (etwa 750 Äpfel von 8 cm Durchmesser, 15 m2); ihre flächenbezogene Wärmekapazität ist die einer wasserreichen Kugel (840 kg/m3, 3600 J/(kg K), Radius/3), eine Autorenschätzung.")]
    S.structure_assumptions = tree_structure_assumptions("apple", "Apfel")
    share = {m.id: m.area() for m in sunm}
    tot = sum(share.values())

    def fruit(total_kg):
        return fruit_extra({k: total_kg * 0.85 * v / tot for k, v in share.items()}, "apple-yield")

    S.poses = [
        {"id": "summer-fruit-set", "title": T("Summer, green fruit set", "Sommer, grüner Fruchtansatz"), "desc": T("Light air of 1.5 m/s at 10 m, after the June drop: about 25 kg of small green apples hang in the crown, the scaffolds are still nearly straight.", "Leiser Zug von 1,5 m/s in 10 m Höhe, nach dem Junifall: etwa 25 kg kleine grüne Äpfel hängen in der Krone, die Leitäste sind noch fast gerade."), "v10": 1.5, "sun": 1.0, "emit": 0.8, "droop": 0.17, "extra": fruit(25.0)},
        {"id": "breeze-with-fruit", "title": T("Breeze with half the fruit load", "Brise mit halber Fruchtlast"), "desc": T("Moderate breeze of 5.5 m/s at 10 m (Beaufort 4) in late summer with about 90 kg of apples: the scaffolds droop and lean downwind; drag by the reconfigured law (cd 0.35).", "Mäßige Brise von 5,5 m/s in 10 m Höhe (Beaufort 4) im Spätsommer mit etwa 90 kg Äpfeln: Die Leitäste hängen durch und neigen sich in Windrichtung; Widerstand nach dem umgeformten Gesetz (cd 0,35)."), "v10": 5.5, "sun": 1.0, "emit": 0.9, "droop": 0.6, "extra": fruit(90.0)},
        {"id": "harvest-laden", "title": T("Harvest, full fruit load", "Ernte, volle Fruchtlast"), "desc": T("Calm early-autumn day with the full fruit load of about 150 kg (750 apples of 0.2 kg): the scaffold limbs bow down by several degrees, the leaves begin to age and transpire at 50 %.", "Windstiller Frühherbsttag mit der vollen Fruchtlast von etwa 150 kg (750 Äpfel zu je 0,2 kg): Die Leitäste biegen sich um mehrere Grad nach unten, die Blätter beginnen zu altern und transpirieren zu 50 %."), "v10": 1.0, "sun": 0.8, "emit": 0.5, "droop": 1.0, "extra": fruit(150.0)},
        {"id": "gale-windfall", "title": T("Gale with rain and heavy fruit", "Sturm mit Regen und schwerem Fruchtbehang"), "desc": T("Beaufort 9, 22 m/s at 10 m with rain, 120 kg of fruit still on the tree: strong lean, drooping scaffolds, wet bark and leaves; the critical case for scaffold limbs and fruit loss.", "Beaufort 9, 22 m/s in 10 m Höhe mit Regen, noch 120 kg Früchte am Baum: starke Neigung, hängende Leitäste, nasse Rinde und Blätter; der kritische Fall für Leitäste und Fruchtverlust."), "v10": 22.0, "sun": 0.15, "emit": 0.5, "wet": 0.6, "droop": 0.8, "extra": fruit(120.0)},
    ]
    S.make_reps = make_reps
    S.make_symbols = make_symbols
    S.references = S.references + [{"id": "apple-yield", "citation": "General orchard practice: a mature full-standard apple tree bears roughly 100 to 200 kg of fruit per year; the value used here is an authored estimate, not a cited measurement."}]
    return S


STEM = {
    "200": [(-0.35, 0.34), (-0.25, 0.27), (-0.1, 0.215), (0.4, 0.195), (1.0, 0.185), (1.4, 0.18), (2.0, 0.15), (2.8, 0.12), (3.45, 0.09), (3.6, 0.08)],
    "100": [(-0.35, 0.36), (-0.3, 0.3), (-0.2, 0.235), (-0.05, 0.205), (0.5, 0.19), (1.1, 0.183), (1.5, 0.175), (2.1, 0.15), (2.8, 0.122), (3.4, 0.092), (3.6, 0.08)],
    "50": [(-0.35, 0.38), (-0.32, 0.33), (-0.26, 0.275), (-0.15, 0.225), (0.0, 0.2), (0.4, 0.19), (0.9, 0.186), (1.25, 0.18), (1.7, 0.16), (2.2, 0.14), (2.8, 0.12), (3.3, 0.095), (3.6, 0.082)],
}


def stem_shapes(S, sc):
    t = S.rig.bones["trunk"]
    asym = {"200": [0.0, 0.01, -0.012, 0.006], "100": [0.0, 0.008, -0.012, 0.01, -0.006], "50": [0.0, 0.008, -0.012, 0.012, -0.008, 0.006]}[sc]
    out = [Shape("stem", t, "body", cr_closed(stem_pts(t, STEM[sc], asym)), True, 0.36, 1, region="trunk-bark", material="apple-bark")]
    if sc == "200":
        out.append(Shape("stem-crack", t, "detail", cr_open(wavy(t, 0.1, 1.3, 0.06, 0.02, 1.5, n=5)), False, 0.2, 1.2))
    if sc == "100":
        for i, (s, off, ln) in enumerate([(0.1, 0.09, 0.6), (0.8, -0.08, 0.7), (1.0, 0.08, 0.5)]):
            out.append(Shape("bark-scale-%d" % (i + 1), t, "texture", cr_open(wavy(t, s, s + ln, off, 0.02, 1.3, n=6, phase=i)), False, 0.15, 1.2))
    if sc == "50":
        for i, (s, off, ln) in enumerate([(0.05, 0.12, 0.45), (0.2, -0.1, 0.5), (0.7, 0.1, 0.45), (0.75, -0.07, 0.5), (1.0, 0.0, 0.35)]):
            out.append(Shape("bark-scale-%d" % (i + 1), t, "texture", cr_open(wavy(t, s, s + ln, off, 0.02, 1.6, n=6, phase=i * 0.9)), False, 0.15, 1.2))
        for i, (s, off) in enumerate([(0.55, 0.0), (1.2, 0.08)]):
            c = t.to_world((s, off))
            out.append(Shape("bark-knot-%d" % (i + 1), t, "detail", cr_closed(ring_pts(c[0], c[1], 0.035, n=8, ry=0.055)), False, 0.2, 1.3))
    return out


def fruit_positions(S, count_per_mass):
    pts = []
    for i, m in enumerate([m for m in S.masses if m.kind == "sun"]):
        for j in range(count_per_mass):
            fr, ang = FRUIT_AT[(i + j) % len(FRUIT_AT)]
            a = (ang + 12 * i) * D2R
            pts.append((m, j, m.c[0] + m.rx * 0.9 * fr * math.cos(a), m.c[1] + m.ry * 0.9 * fr * math.sin(a)))
    return pts


def extras(S, sc):
    out = []
    n = {"200": 1, "100": 2, "50": 4}[sc]
    for m, j, x, y in fruit_positions(S, n):
        out.append(Shape("%s-fruit%d" % (m.id, j + 1), S.rig.bones[m.id], "body" if sc == "200" else "detail", cr_closed(ring_pts(x, y, FRUIT_R, n=8)), True, 0.25, 9 + 0.01 * j, region="fruit", material="apple-fruit"))
    return out


def make_reps(S):
    C = dict(
        stem_bones=["root-plate", "trunk", "ld"],
        min_wood_mm={"200": 0.5, "100": 0.45, "50": 0.4}, min_stroke_wood_mm={"100": 9.0, "50": 0.4}, always_stroke=("spur-w1", "spur-e1", "spur-w2", "spur-e2"),
        mass={
            "200": dict(lobe=1.3, amp=0.09, vary=[1.0, 0.75, 1.15, 0.85, 1.05], under=0.2, pointed=0.7, n=5, rx_s=0.95, ry_s=0.96, phase=0.25, dz=0.2),
            "100": dict(lobe=0.8, amp=0.12, vary=[1.0, 0.7, 1.15, 0.8, 1.0, 0.9], under=0.18, pointed=0.6, n=6, rx_s=0.93, ry_s=0.93, phase=0.5, dz=0.3),
            "50": dict(lobe=0.48, amp=0.15, vary=[1.0, 0.65, 1.15, 0.8, 1.05, 0.7, 1.1], under=0.15, pointed=0.55, n=7, rx_s=0.93, ry_s=0.93, phase=0.3, dz=0.4),
        },
        mass_regions={"200": {"fol-w1": ["leaf-sun", "spur-twig"], "fol-e1": ["leaf-sun", "spur-twig"], "fol-w2": ["leaf-sun", "spur-twig"], "fol-e2": ["leaf-sun", "spur-twig"]}},
        glyph=ovate, glyph_kinds=("sun",), glyph_R=0.04, stems=stem_shapes, extras=extras, shapes500=shapes500, words=WORDS,
    )
    return make_reps_generic(S, C)


CROWN500 = [(0, 2.7), (0.8, 2.45), (1.6, 2.2), (2.5, 2.25), (3.2, 2.55), (3.55, 3.05), (3.5, 3.65), (3.1, 4.2), (2.75, 4.8), (2.3, 5.4), (1.5, 5.95), (0.7, 6.3), (0, 6.38), (-0.7, 6.3), (-1.5, 5.9), (-2.3, 5.3), (-2.8, 4.7), (-3.15, 4.15), (-3.55, 3.6), (-3.6, 3.0), (-3.25, 2.5), (-2.5, 2.2), (-1.6, 2.15), (-0.8, 2.4)]


def shapes500(S):
    t = S.rig.bones["trunk"]
    ld = S.rig.bones["ld"]
    st = [(-0.35, 0.34), (-0.25, 0.27), (-0.1, 0.215), (0.4, 0.195), (1.0, 0.185), (1.5, 0.178), (2.2, 0.15), (3.0, 0.12)]
    stem = Shape("stem", t, "body", cr_closed(stem_pts(t, st, [0.0, 0.006])), True, 0.36, 1, region="trunk-bark", material="apple-bark")
    crown = Shape("crown", ld, "body", cr_closed([(x * 0.945, y) for x, y in CROWN500]), True, 0.55, 3, regions=["leaf-sun", "leaf-shade", "limb-bark", "spur-twig", "fruit"], material="apple-leaf-sun")
    return [stem, crown]


WORDS = {
    "s500_sil": ("The apple tree is composed as one closed, wide and low vase-shaped crown with a flat, slightly scalloped underside and two soft shoulders, above a short straight stem of 1.6 m; 1 m is 2 mm, the crown is 14 mm wide.", "Der Apfelbaum ist als eine geschlossene, breite und niedrige Vasenkrone mit flacher, leicht gewellter Unterseite und zwei weichen Schultern über einem kurzen geraden Stamm von 1,6 m komponiert; 1 m sind 2 mm, die Krone ist 14 mm breit."),
    "s500_ret": ("Retained: the stem with root flare and the crown outline with its low side lobes; the stem runs on into the crown for a clean join.", "Beibehalten: der Stamm mit Wurzelanlauf und der Kronenumriss mit seinen tiefen Seitenlappen; der Stamm läuft für einen sauberen Anschluss in die Krone weiter."),
    "s500_mer": ("Merged into the crown path: five sun foliage masses, both shade cores, the scaffold limbs, the fruiting spurs and the fruit.", "Im Kronenpfad vereint: fünf Sonnenlaubmassen, beide Schattenkerne, die Leitäste, das Fruchtholz und die Früchte."),
    "s500_omi": ("Omitted: single scaffolds, spurs, leaves and every apple (8 cm is 0.16 mm); no internal edge.", "Weggelassen: einzelne Leitäste, Fruchtholz, Blätter und jeder Apfel (8 cm sind 0,16 mm); keine innere Kante."),
    "s500_thin": ("Scaffolds and spurs are not thickened; they are merged into the filled crown, so the outline stays within the real outline tolerance of the 1:50 drawing.", "Leitäste und Fruchtholz werden nicht verdickt, sondern in die gefüllte Krone eingeschmolzen, so dass der Umriss innerhalb der realen Umrisstoleranz der 1:50-Zeichnung bleibt."),
    "s500_ovl": ("Overlays map onto: trunk-bark (stem) and, merged in the crown path, leaf-sun, leaf-shade, limb-bark, spur-twig and fruit; merged shapes show the area-weighted value of their regions, labelled as merged.", "Überlagerungen bilden sich ab auf: trunk-bark (Stamm) sowie, im Kronenpfad vereint, leaf-sun, leaf-shade, limb-bark, spur-twig und fruit; vereinte Formen zeigen den flächengewichteten Wert ihrer Regionen, als vereint gekennzeichnet."),
    "s200_sil": ("At 1:200 (1 m is 5 mm) the apple is separated into its main masses: stem with root flare, the scaffold limbs of two tiers and seven foliage masses (five sun masses, two shade cores) as scalloped outlines with an open, spreading vase form; the apples appear as seven separate dots.", "Bei 1:200 (1 m sind 5 mm) wird der Apfelbaum in seine Hauptmassen aufgeteilt: Stamm mit Wurzelanlauf, die Leitäste zweier Etagen und sieben Laubmassen (fünf Sonnenmassen, zwei Schattenkerne) als gewellte Umrisse in offener, ausladender Vasenform; die Äpfel erscheinen als sieben einzelne Punkte."),
    "s200_ret": ("Retained: stem, leader and the scaffold limbs thicker than 0.5 mm on paper, one crack line, and one sample apple per sun mass at the real fruit size (0.4 mm).", "Beibehalten: Stamm, Stammverlängerung und die Leitäste, die auf dem Papier dicker als 0,5 mm sind, eine Risslinie und ein Beispielapfel je Sonnenmasse in der wahren Fruchtgröße (0,4 mm)."),
    "s200_mer": ("Merged into the four outer sun masses (as merged regions): the fruiting spurs; leaf clusters and single leaves into the mass scallops.", "In den vier äußeren Sonnenmassen (als vereinte Regionen) vereint: das Fruchtholz; Blattbüschel und einzelne Blätter in die Wellen der Massen."),
    "s200_omi": ("Omitted: twigs, bark scales, knots, leaf marks and all other apples.", "Weggelassen: Zweige, Rindenschuppen, Astknoten, Blattzeichen und alle übrigen Äpfel."),
    "s200_thin": ("Limbs thinner than 0.5 mm on paper are not drawn as filled outlines; the drawn ones keep their real width and run into the masses.", "Äste, die auf dem Papier dünner als 0,5 mm wären, werden nicht als gefüllte Umrisse gezeichnet; die gezeichneten behalten ihre reale Breite und münden in die Massen."),
    "edge200": ("one bark crack line on the stem.", "eine Rindenrisslinie am Stamm."),
    "s200_ovl": ("Every thermal region keeps a shape: stem, limbs, sun and shade masses, the fruit dots; spur-twig is a merged region of the outer masses.", "Jede thermische Region behält eine Form: Stamm, Äste, Sonnen- und Schattenmassen, die Fruchtpunkte; spur-twig ist eine vereinte Region der äußeren Massen."),
    "s100_sil": ("At 1:100 (1 m is 10 mm) the masses get finer lobes and the vase structure appears: the scaffold tiers reaching into the foliage, four fruiting-spur strokes, two leaf-cluster arcs per mass, the scaly stem and fourteen sample apples (0.8 mm).", "Bei 1:100 (1 m sind 10 mm) erhalten die Massen feinere Lappen und die Vasenstruktur erscheint: die Leitastetagen, die ins Laub reichen, vier Fruchtholzstriche, zwei Blattbüschelbögen je Masse, der schuppige Stamm und vierzehn Beispieläpfel (0,8 mm)."),
    "s100_ret": ("Retained: seven foliage masses, stem, leader, all scaffold sections, the spurs and fourteen apples as filled dots.", "Beibehalten: sieben Laubmassen, Stamm, Stammverlängerung, alle Leitastabschnitte, das Fruchtholz und vierzehn Äpfel als gefüllte Punkte."),
    "s100_mer": ("Single leaves and twig ends are merged into lobed mass outlines; bark scales into three strokes.", "Einzelne Blätter und Zweigenden sind in gelappte Massenumrisse eingeschmolzen; Rindenschuppen in drei Striche."),
    "s100_omi": ("Omitted: individual leaves, fine twigs and knots; most apples (only fourteen of about 750 are drawn); the fine 1:50 leaf marks are replaced by two arcs per mass.", "Weggelassen: einzelne Blätter, feine Zweige und Astknoten; die meisten Äpfel (nur vierzehn von etwa 750 sind gezeichnet); die feinen 1:50-Blattzeichen sind durch zwei Bögen je Masse ersetzt."),
    "s100_thin": ("Thin strokes are at least 0.1 mm: scales and spurs use the texture and detail pens, scaffolds keep their real section width.", "Dünne Striche sind mindestens 0,1 mm: Schuppen und Fruchtholz mit Textur- und Detailstift, Leitäste in realer Querschnittsbreite."),
    "s100_ovl": ("Every region keeps its shapes; the shade hatch is a pure drawing hatch and does not stand for temperature; the apple dots carry the fruit region.", "Jede Region behält ihre Formen; die Schattenschraffur ist eine reine Zeichenschraffur und steht nicht für Temperatur; die Apfelpunkte tragen die Region fruit."),
    "s50_sil": ("At 1:50 (1 m is 20 mm) the apple tree is drawn in full: scaly stem with cracks and two knots, tapered leader and scaffolds, fruiting spurs, finely lobed foliage masses with leaf-cluster arcs and ovate leaf marks, and twenty-eight sample apples of the real size (1.6 mm).", "Bei 1:50 (1 m sind 20 mm) wird der Apfelbaum vollständig gezeichnet: schuppiger Stamm mit Rissen und zwei Astknoten, verjüngte Stammverlängerung und Leitäste, Fruchtholz, fein gelappte Laubmassen mit Blattbüschelbögen und eiförmigen Blattzeichen und achtundzwanzig Beispieläpfel in wahrer Größe (1,6 mm)."),
    "s50_ret": ("Retained: every woody member of the rig, every foliage mass; the fruit are a sample of the load, not a count.", "Beibehalten: jedes Holzglied des Rigs, jede Laubmasse; die Früchte sind eine Auswahl der Last, keine Zählung."),
    "s50_mer": ("Only the plug overlaps at the bone joints close the seams between tapered woody parts; bark scales are merged into five strokes.", "Nur die Überlappungen an den Knochengelenken schließen die Nähte zwischen verjüngten Holzteilen; Rindenschuppen sind in fünf Striche zusammengefasst."),
    "s50_omi": ("Omitted: the several thousand single leaves (1.2 to 1.6 mm), buds, blossom scars and the remaining apples.", "Weggelassen: die mehreren tausend einzelnen Blätter (1,2 bis 1,6 mm), Knospen, Blütennarben und die übrigen Äpfel."),
    "s50_thin": ("Spur ends of 0.012 m stay as 0.13 mm strokes; scales and arcs are 0.13 mm strokes and the leaf marks 0.18 mm outlines, all above the pen minimum.", "Fruchtholzenden von 0,012 m bleiben 0,13-mm-Striche; Schuppen und Bögen sind 0,13-mm-Striche und die Blattzeichen 0,18-mm-Umrisse, alle über dem Stiftminimum."),
    "s50_ovl": ("Each thermal region has its own shapes (the apples carry the fruit region); scales, arcs and leaf marks are strokes without a region and follow their bone.", "Jede thermische Region hat eigene Formen (die Äpfel tragen die Region fruit); Schuppen, Bögen und Blattzeichen sind Striche ohne Region und folgen ihrem Knochen."),
}


def make_symbols(S, bounds):
    root = S.rig.bones["root-plate"]
    W, D = 6.9, S.depth_m
    radii = [1.0, 0.96, 0.99, 0.95, 1.0, 0.97, 0.98, 0.94, 1.0, 0.96, 0.98, 0.97]
    crown = envelope(0, 0, W / 2, D / 2, radii, k=11, amp=0.055, phase=0.1, n=4)
    plan = [Shape("crown-extent", root, "body", cr_closed(crown), False, 0.5, 1), Shape("stem", root, "body", cr_closed(ring_pts(0, 0, 0.19, n=12)), True, 0.3, 3)]
    for i in range(4):
        a = (45 + 90 * i) * D2R
        plan.append(Shape("scaffold-%d" % (i + 1), root, "detail", cr_open([(0.2 * math.cos(a), 0.2 * math.sin(a)), (1.6 * math.cos(a), 1.6 * math.sin(a)), (2.8 * math.cos(a + 0.12), 2.8 * math.sin(a + 0.12))]), False, 0.3, 2))
    nominal = [Shape("sign-circle", root, "body", cr_closed(ring_pts(0, 0, 1.6, n=12)), False, 0.5, 1), Shape("sign-dot", root, "body", cr_closed(ring_pts(0, 0, 0.35, n=8)), True, 0.3, 2), Shape("sign-fruit", root, "body", cr_closed(ring_pts(0, 0, 0.9, n=12)), False, 0.3, 3)]
    for s_ in plan + nominal:
        s_.frame = IDENT
    return [
        {"id": "plan-crown", "title": T("Plan symbol with true crown extent", "Grundrisssymbol mit wahrer Kronenausdehnung"), "description": T("A conventional plan sign for orchard and site plans: the true plan outline of the crown, four scaffold directions and the stem section. It is a separate symbol and is never used silently for a pictorial scale.", "Ein konventionelles Grundrisszeichen für Obstwiesen- und Lagepläne: der wahre Grundrissumriss der Krone, vier Leitastrichtungen und der Stammquerschnitt. Es ist ein eigenes Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet."),
         "semantics": "true-extent", "units": "model-m", "denominators": [500, 200, 100], "pens": PENS["100"], "hatches": [], "geometry": {"elevation": [], "plan": [s.json() for s in plan]},
         "dimensionalNote": T("The outline is the true plan extent of the crown in model metres (%.1f m by %.1f m) and the filled circle the true stem section of 0.38 m; the four strokes only indicate scaffold directions. Orchard spacing (usually 8 to 10 m for standards) is not part of the symbol." % (W, D), "Der Umriss ist die wahre Kronenausdehnung im Grundriss in Modellmetern (%.1f m mal %.1f m) und der gefüllte Kreis der wahre Stammquerschnitt von 0,38 m; die vier Striche deuten nur Leitastrichtungen an. Der Pflanzabstand im Obstgarten (bei Hochstämmen meist 8 bis 10 m) gehört nicht zum Symbol." % (W, D))},
        {"id": "tree-sign", "title": T("Nominal fruit tree sign with trunk centre", "Nominelles Obstbaumzeichen mit Stammmitte"), "description": T("A circle with a centre dot and an inner ring as the conventional sign of a fruit tree in small-scale plans; the circle is a fixed paper size.", "Ein Kreis mit Mittelpunkt und Innenring als übliches Zeichen eines Obstbaums in kleinmaßstäblichen Plänen; der Kreis hat eine feste Papiergröße."),
         "semantics": "nominal", "units": "paper-mm", "denominators": [500, 200], "pens": PENS["500"], "hatches": [], "geometry": {"elevation": [], "plan": [s.json() for s in nominal]},
         "dimensionalNote": T("A fixed 3.2 mm circle with a 1.8 mm inner ring and a 0.7 mm centre dot in paper millimetres; its size carries no dimension. Use the true-extent plan symbol where the crown width matters.", "Ein fester Kreis von 3,2 mm mit 1,8 mm Innenring und 0,7 mm Mittelpunkt in Papiermillimetern; seine Größe trägt kein Maß. Wo die Kronenbreite zählt, ist das Grundrisssymbol mit wahrer Ausdehnung zu verwenden.")},
    ]
