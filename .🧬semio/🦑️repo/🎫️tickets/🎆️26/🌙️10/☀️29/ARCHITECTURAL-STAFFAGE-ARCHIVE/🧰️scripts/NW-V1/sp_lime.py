"""small-leaved-lime: dense heart-leaved dome crown, one excurrent leader, epicormic shoots at the stem base."""
import math
from lib import *
from common import *
from reps import *
from gen import *
from sp_plane import Sp, common_refs, tree_thermal_assumptions, tree_structure_assumptions


def on(r, id, s, off=0.0):
    return r.bones[id].at(s, off)


def heart(cx, cy, R, rot=0.0):
    base = [(0, -1.0), (0.55, -0.45), (0.95, 0.2), (0.7, 0.85), (0.3, 0.85), (0, 0.45), (-0.3, 0.85), (-0.7, 0.85), (-0.95, 0.2), (-0.55, -0.45)]
    return rot_pts([(cx + R * x, cy + R * y) for x, y in base], cx, cy, rot)


def build():
    S = Sp()
    S.id = "small-leaved-lime"
    S.folder = "small-leaved-lime"
    S.title = T("Small-Leaved Lime", "Winterlinde")
    S.summary = T("A mature park lime (Tilia cordata) with a straight stem, one continuous leader and a dense, domed crown of small heart-shaped leaves; epicormic shoots sprout from the stem base.",
                  "Eine ausgewachsene Parklinde (Tilia cordata) mit geradem Stamm, durchgehender Stammverlängerung und dichter, gewölbter Krone aus kleinen herzförmigen Blättern; am Stammfuß treiben Stammaustriebe.")
    S.tags = ["tree", "deciduous", "broadleaf", "dense-crown", "epicormic", "wind", "transpiration"]
    S.depth_m = 9.0
    S.footprint = 2.0
    S.density = 750
    S.root_mass_factor = 1.25
    S.default_scale = 200
    S.default_pose = "summer-calm"
    S.breeze_pose = "summer-breeze"
    S.breeze_v10 = 5.5
    S.gust_peak = 14.0
    S.trunk_bone = "trunk"
    S.twig_factor = {"leaf-sun": 1.7, "leaf-shade": 1.7, "leaf-epicormic": 1.5}
    S.cd = {}
    S.bark_extra = {}
    r = Rig()
    S.rig = r
    W = "lime-wood"
    r.add("root-plate", None, (0, 0), 90, 0.5, T("Root plate and flare", "Wurzelteller und Wurzelanlauf"), 0.85, 0.62, limit=0, material=W).region = "trunk-bark"
    r.add("trunk", "root-plate", (0, 0.5), 90, 2.4, T("Clear stem", "Schaft"), 0.56, 0.50, limit=3.5, material=W).region = "trunk-bark"
    r.add("ld1", "trunk", r.tip("trunk"), 90.5, 3.3, T("Leader, lower part", "Stammverlängerung, unterer Teil"), 0.50, 0.36, limit=4, material=W).region = "trunk-bark"
    r.add("ld2", "ld1", r.tip("ld1"), 89, 3.4, T("Leader, middle part", "Stammverlängerung, mittlerer Teil"), 0.36, 0.22, limit=5, material=W).region = "limb-bark"
    r.add("ld3", "ld2", r.tip("ld2"), 91, 3.0, T("Leader, upper part", "Stammverlängerung, oberer Teil"), 0.22, 0.10, limit=6, material=W).region = "limb-bark"
    r.add("ld4", "ld3", r.tip("ld3"), 90, 1.4, T("Leader tip", "Wipfel"), 0.10, 0.05, limit=8, material=W).region = "limb-bark"
    L = [("lw1a", "ld1", 0.3, 140, 2.2, 0.22, 0.15, "Lower west limb", "Unterer Westast"), ("lw1b", "lw1a", None, 172, 1.8, 0.15, 0.09, "Lower west limb, outer part", "Unterer Westast, Außenteil"),
         ("le1a", "ld1", 0.5, 40, 2.2, 0.22, 0.15, "Lower east limb", "Unterer Ostast"), ("le1b", "le1a", None, 8, 1.8, 0.15, 0.09, "Lower east limb, outer part", "Unterer Ostast, Außenteil"),
         ("lw2a", "ld1", 2.5, 135, 3.0, 0.18, 0.10, "Middle west limb", "Mittlerer Westast"), ("le2a", "ld1", 2.8, 44, 3.0, 0.18, 0.10, "Middle east limb", "Mittlerer Ostast"),
         ("lw3a", "ld2", 1.6, 128, 2.6, 0.14, 0.07, "Upper west limb", "Oberer Westast"), ("le3a", "ld2", 2.1, 52, 2.4, 0.14, 0.07, "Upper east limb", "Oberer Ostast"),
         ("lw4a", "ld3", 1.0, 125, 1.8, 0.10, 0.05, "Crown west limb", "Kronenast West"), ("le4a", "ld3", 1.4, 55, 1.8, 0.10, 0.05, "Crown east limb", "Kronenast Ost")]
    for id, par, s, ang, ln, d0, d1, te, td in L:
        base = r.tip(par) if s is None else on(r, par, s)
        r.add(id, par, base, ang, ln, T(te, td), d0, d1, limit=10 if id.endswith("a") else 14, material=W).region = "limb-bark"
    r.add("epi-e", "trunk", r.bones["trunk"].to_world((0.5, -0.26)), 55, 1.3, T("Epicormic shoots, east", "Stammaustriebe, Ost"), 0.03, 0.015, limit=25, material=W).region = "epicormic-stem"
    r.add("epi-w", "trunk", r.bones["trunk"].to_world((1.5, 0.25)), 128, 0.9, T("Epicormic shoots, west", "Stammaustriebe, West"), 0.03, 0.015, limit=25, material=W).region = "epicormic-stem"
    S.levels = {"trunk": "trunk", "ld1": "trunk", "ld2": "trunk", "ld3": "upper", "ld4": "upper", "epi-e": "epi", "epi-w": "epi"}
    for id, *_ in L:
        S.levels[id] = "limb" if id.endswith("a") else "limbb"
    S.lean_ref = {"trunk": 0.25, "upper": 0.6, "limb": 0.6, "limbb": 1.0, "epi": 2.0, "fol": 3.0}
    S.droop_levels = {"limb": 1.2, "limbb": 2.0, "fol": 3.0}
    S.sway_amp = {"trunk": 0.5, "upper": 0.5, "limb": 0.55, "limbb": 0.6, "epi": 0.5, "fol": 0.65}
    S.sway_lag = 0.3
    S.sway_period = 4.0
    S.gust_tau = {"trunk": 0.8, "upper": 0.6, "limb": 0.5, "limbb": 0.4, "epi": 0.3, "fol": 0.25}
    S.sway_desc_en = "A deterministic %s s sway cycle about the breeze lean (5.5 m/s at 10 m; estimated first mode of a 14 m clamped-free stem about 0.25 Hz). Each level lags the one below it; the dense foliage masses swing about 2 to 3 degrees."
    S.sway_desc_de = "Ein deterministischer Schwingungszyklus von %s s um die Neigung bei Brise (5,5 m/s in 10 m Höhe; geschätzte erste Eigenform eines 14 m hohen eingespannten Stabs etwa 0,25 Hz). Jede Ebene hinkt der darunter nach; die dichten Laubmassen schwingen etwa 2 bis 3 Grad."
    S.gust_desc_en = "A single gust over 8 s from %.1f to %.1f m/s at 10 m: the wind-load tracks rise with the reconfigured drag law and the crown leans with a level-wise lag, outer limbs first, the stem last."
    S.gust_desc_de = "Eine einzelne Böe über 8 s von %.1f auf %.1f m/s in 10 m Höhe: Die Windlast-Spuren steigen mit dem umgeformten Widerstandsgesetz, die Krone neigt sich ebenenweise verzögert, zuerst die äußeren Äste, zuletzt der Stamm."
    S.masses = [
        Mass("fol-core-low", "ld1", (0.0, 6.6), 2.1, 2.0, 0, "leaf-shade", "shade", T("shaded lower core", "beschatteter Kern unten")),
        Mass("fol-core-high", "ld2", (0.0, 10.3), 1.6, 2.0, 0, "leaf-shade", "shade", T("shaded upper core", "beschatteter Kern oben")),
        Mass("fol-w1", "lw1b", (-3.0, 5.0), 1.7, 1.3, 0, "leaf-sun", "sun", T("lower west foliage", "Laubmasse unten West")),
        Mass("fol-e1", "le1b", (2.9, 5.1), 1.7, 1.3, 0, "leaf-sun", "sun", T("lower east foliage", "Laubmasse unten Ost")),
        Mass("fol-w2", "lw2a", (-3.0, 7.4), 1.55, 1.45, 8, "leaf-sun", "sun", T("middle west foliage", "Laubmasse Mitte West")),
        Mass("fol-e2", "le2a", (3.0, 7.7), 1.55, 1.45, -8, "leaf-sun", "sun", T("middle east foliage", "Laubmasse Mitte Ost")),
        Mass("fol-w3", "lw3a", (-2.1, 10.0), 1.4, 1.4, 10, "leaf-sun", "sun", T("upper west foliage", "Laubmasse oben West")),
        Mass("fol-e3", "le3a", (2.0, 10.4), 1.4, 1.4, -10, "leaf-sun", "sun", T("upper east foliage", "Laubmasse oben Ost")),
        Mass("fol-w4", "lw4a", (-1.1, 11.8), 1.1, 1.2, 8, "leaf-sun", "sun", T("crown west foliage", "Laubmasse Krone West")),
        Mass("fol-e4", "le4a", (1.1, 12.0), 1.1, 1.2, -8, "leaf-sun", "sun", T("crown east foliage", "Laubmasse Krone Ost")),
        Mass("fol-top", "ld4", (0.1, 12.7), 1.3, 1.5, 0, "leaf-sun", "sun", T("crown top foliage", "Laubmasse Wipfel")),
        Mass("fol-epi-e", "epi-e", (0.95, 1.95), 0.55, 0.6, -20, "leaf-epicormic", "epi", T("east epicormic leaves", "Austriebsblätter Ost")),
        Mass("fol-epi-w", "epi-w", (-0.7, 2.5), 0.5, 0.5, 15, "leaf-epicormic", "epi", T("west epicormic leaves", "Austriebsblätter West")),
    ]
    for m in S.masses:
        if m.kind != "epi":
            m.c = (m.c[0] * 0.92, m.c[1] - (0.1 if m.id == "fol-top" else 0.0))
            m.rx *= 1.12
            m.ry *= 1.1
    proj = math.pi / 4 * 9.4 * S.depth_m
    S.leaf_area = {"leaf-sun": 2.3 * proj, "leaf-shade": 2.8 * proj, "leaf-epicormic": 7.5}
    S.mat_of = {"trunk-bark": "lime-bark", "limb-bark": "lime-bark-young", "epicormic-stem": "lime-twig", "leaf-sun": "lime-leaf-sun", "leaf-shade": "lime-leaf-shade", "leaf-epicormic": "lime-leaf-epicormic"}
    S.tone = {"wood": 0.38, "sun": 0.6, "shade": 0.45, "epi": 0.5}
    S.tone_line = 0.2
    S.sun = {"trunk-bark": 0.3, "limb-bark": 0.25, "epicormic-stem": 0.1, "leaf-sun": 0.9, "leaf-shade": 0.06, "leaf-epicormic": 0.12}
    S.regions = [
        {"id": "trunk-bark", "title": T("Grey-brown ridged bark of stem and lower leader", "Graubraune Furchenrinde von Stamm und unterer Stammverlängerung"), "kind": "bark", "model": "passive", "material": "lime-bark", "len": 0.55, "geom": "cylinder", "sky": 0.4, "hc": 70000, "ref": "wood-handbook-2021"},
        {"id": "limb-bark", "title": T("Bark of upper leader and limbs", "Rinde von oberer Stammverlängerung und Ästen"), "kind": "bark", "model": "passive", "material": "lime-bark-young", "len": 0.15, "geom": "cylinder", "sky": 0.5, "hc": 40000, "ref": "wood-handbook-2021"},
        {"id": "epicormic-stem", "title": T("Stems of the epicormic shoots", "Stängel der Stammaustriebe"), "kind": "stem", "model": "passive", "material": "lime-twig", "len": 0.03, "geom": "cylinder", "sky": 0.5, "hc": 12000, "ref": "wood-handbook-2021"},
        {"id": "leaf-sun", "title": T("Sun leaves of the outer crown", "Sonnenblätter der Außenkrone"), "kind": "foliage", "model": "leaf", "material": "lime-leaf-sun", "area": S.leaf_area["leaf-sun"], "len": 0.07, "geom": "vertical-plate", "sky": 0.7, "hc": 850, "gs": 0.0045, "lai": 2.3, "ref": "jones-2014"},
        {"id": "leaf-shade", "title": T("Shade leaves of the dense inner crown", "Schattenblätter der dichten Innenkrone"), "kind": "foliage", "model": "leaf", "material": "lime-leaf-shade", "area": S.leaf_area["leaf-shade"], "len": 0.08, "geom": "vertical-plate", "sky": 0.3, "hc": 650, "gs": 0.0025, "lai": 2.8, "ref": "jones-2014"},
        {"id": "leaf-epicormic", "title": T("Large shade leaves of the epicormic shoots", "Große Schattenblätter der Stammaustriebe"), "kind": "foliage", "model": "leaf", "material": "lime-leaf-shade", "area": 7.5, "len": 0.1, "geom": "vertical-plate", "sky": 0.4, "hc": 650, "gs": 0.0030, "lai": round(7.5 / proj, 3), "ref": "jones-2014"},
    ]
    sunm = [m for m in S.masses if m.kind == "sun"]
    S.emitters = [
        {"id": "transpiration-sun", "aid": "tp-sun", "region": "leaf-sun", "strength": 1.0, "diam": 2.6, "title": T("Transpiration of the sun leaves", "Transpiration der Sonnenblätter"), "anchors": [(m.id, (m.c[0], m.c[1] + 0.2)) for m in sunm]},
        {"id": "transpiration-shade", "aid": "tp-shade", "region": "leaf-shade", "strength": 0.7, "diam": 2.2, "title": T("Transpiration of the shade leaves", "Transpiration der Schattenblätter"),
         "anchors": [("fol-core-low", (-0.3, 5.4)), ("fol-core-low", (0.3, 6.6)), ("fol-core-low", (-0.2, 7.8)), ("fol-core-high", (0.2, 9.4)), ("fol-core-high", (-0.2, 11.0))]},
        {"id": "transpiration-epicormic", "aid": "tp-epi", "region": "leaf-epicormic", "strength": 0.6, "diam": 0.9, "title": T("Transpiration of the epicormic leaves", "Transpiration der Austriebsblätter"),
         "anchors": [("fol-epi-e", (0.8, 1.7)), ("fol-epi-e", (1.05, 2.1)), ("fol-epi-e", (1.0, 1.9)), ("fol-epi-w", (-0.6, 2.35)), ("fol-epi-w", (-0.8, 2.6))]},
    ]
    S.materials = [
        {"id": "lime-bark", "title": T("Lime bark, grey-brown with long ridges", "Lindenrinde, graubraun mit langen Rippen"), "emissivity": 0.95, "solarAbsorptivity": 0.75, "densityKgM3": 600, "conductivityWmK": 0.12, "source": "authored-estimate", "reference": "wood-handbook-2021"},
        {"id": "lime-bark-young", "title": T("Smoother bark of the upper limbs", "Glattere Rinde der oberen Äste"), "emissivity": 0.95, "solarAbsorptivity": 0.65, "densityKgM3": 650, "conductivityWmK": 0.13, "source": "authored-estimate", "reference": "wood-handbook-2021"},
        {"id": "lime-twig", "title": T("Green-red bark of epicormic shoots", "Grünrote Rinde der Stammaustriebe"), "emissivity": 0.95, "solarAbsorptivity": 0.6, "densityKgM3": 700, "conductivityWmK": 0.14, "source": "authored-estimate", "reference": "wood-handbook-2021"},
        {"id": "lime-leaf-sun", "title": T("Lime sun leaves", "Linden-Sonnenblätter"), "emissivity": 0.97, "solarAbsorptivity": 0.5, "source": "literature", "reference": "jones-2014"},
        {"id": "lime-leaf-shade", "title": T("Lime shade leaves (effective)", "Linden-Schattenblätter (wirksam)"), "emissivity": 0.97, "solarAbsorptivity": 0.24, "source": "authored-estimate", "reference": "jones-2014"},
        {"id": "lime-leaf-epicormic", "title": T("Epicormic shoot leaves", "Blätter der Stammaustriebe"), "emissivity": 0.97, "solarAbsorptivity": 0.35, "source": "authored-estimate", "reference": "jones-2014"},
        {"id": W, "title": T("Green lime wood", "Frisches Lindenholz"), "emissivity": 0.9, "solarAbsorptivity": 0.65, "densityKgM3": 750, "conductivityWmK": 0.14, "elasticModulusPa": 5000000000, "strengthPa": 30000000, "source": "authored-estimate", "reference": "wood-handbook-2021"},
    ]
    S.references = common_refs()
    S.notes = T("Rig and drawings are authored by hand for a mature park lime of about 60 years: stem 2.9 m, one continuous leader to 14 m, four limb tiers and two epicormic shoot clusters on the stem. Crown size follows the order of magnitude of Tilia cordata in Pretzsch et al. (2015); wood properties, masses and conductances are authored estimates.",
                 "Rig und Zeichnungen sind von Hand für eine ausgewachsene Parklinde von etwa 60 Jahren entworfen: Stamm 2,9 m, eine durchgehende Stammverlängerung bis 14 m, vier Asttiers und zwei Austriebsbüschel am Stamm. Die Kronengröße folgt der Größenordnung von Tilia cordata bei Pretzsch et al. (2015); Holzeigenschaften, Massen und Leitfähigkeiten sind Autorenschätzungen.")
    S.environment = {"airTempC": 24, "meanRadiantTempC": 28, "relativeHumidity": 0.5, "windMS": 1.5, "windDirectionDeg": 0, "solarWM2": 700, "pressurePa": 101325, "diffuseFraction": 0.15}
    S.thermal_assumptions = tree_thermal_assumptions("", "")
    S.structure_assumptions = tree_structure_assumptions("lime", "Linde")
    S.poses = [
        {"id": "summer-calm", "title": T("Calm summer day", "Windstiller Sommertag"), "desc": T("Light air of 1.5 m/s at 10 m: the dense crown stands at rest, outer leaves lit, inner leaves in deep shade, transpiration at 80 % of the daytime maximum.", "Leiser Zug von 1,5 m/s in 10 m Höhe: Die dichte Krone steht in Ruhe, äußere Blätter beleuchtet, innere im tiefen Schatten, Transpiration bei 80 % des Tagesmaximums."), "v10": 1.5, "sun": 1.0, "emit": 0.8},
        {"id": "summer-breeze", "title": T("Summer breeze", "Sommerbrise"), "desc": T("Moderate breeze of 5.5 m/s at 10 m (Beaufort 4): limbs and foliage lean downwind; drag F = 0.5 rho cd A v^2 with cd 0.35 and a Vogel-type reduction above 8 m/s.", "Mäßige Brise von 5,5 m/s in 10 m Höhe (Beaufort 4): Äste und Laub neigen sich in Windrichtung; Widerstand F = 0,5 rho cd A v^2 mit cd 0,35 und Vogel-artiger Minderung über 8 m/s."), "v10": 5.5, "sun": 1.0, "emit": 0.95},
        {"id": "drought-stress", "title": T("Drought stress at midday", "Trockenstress am Mittag"), "desc": T("Still, hot midday after weeks without rain: stomata are largely closed (transpiration 30 % of the maximum), the sunlit leaves run well above air temperature.", "Windstill, heißer Mittag nach Wochen ohne Regen: Die Stomata sind weitgehend geschlossen (Transpiration 30 % des Maximums), die besonnten Blätter liegen weit über der Lufttemperatur."), "v10": 1.0, "sun": 1.0, "emit": 0.3},
        {"id": "autumn-leaf-fall", "title": T("Autumn, senescing leaves", "Herbst, alternde Blätter"), "desc": T("Damp autumn day with yellowing leaves, stomata mostly closed (transpiration 20 %), dew on bark and leaves. The drawn foliage stays: shapes are not toggled by a pose.", "Feuchter Herbsttag mit vergilbenden Blättern, Stomata meist geschlossen (Transpiration 20 %), Tau auf Rinde und Blättern. Das gezeichnete Laub bleibt: Formen werden von einer Pose nicht ein- oder ausgeblendet."), "v10": 3.0, "sun": 0.5, "emit": 0.2, "wet": 0.3},
        {"id": "wet-snow", "title": T("Early wet snow on the foliage", "Früher Nassschnee im Laub"), "desc": T("Wet autumn snow of 10 kg per m2 of foliage plan area (order of magnitude from EN 1991-1-3) weighs on the foliage masses and bends limbs and foliage down; calm, no sun, stomata closed.", "Nasser Herbstschnee von 10 kg je m2 Laubgrundfläche (Größenordnung nach EN 1991-1-3) lastet auf den Laubmassen und biegt Äste und Laub nach unten; windstill, keine Sonne, Stomata geschlossen."), "v10": 1.5, "sun": 0.05, "emit": 0.05, "wet": 1.0, "droop": 1.0, "extra": snow_extra(10.0)},
        {"id": "gale", "title": T("Gale with rain", "Sturm mit Regen"), "desc": T("Beaufort 9, 22 m/s at 10 m with rain: the crown leans strongly downwind, drag with the reconfiguration reduction (cd 0.35, (v/8)^-0.6 above 8 m/s), wet bark and leaves, overcast sky.", "Beaufort 9, 22 m/s in 10 m Höhe mit Regen: Die Krone neigt sich stark in Windrichtung, Widerstand mit der Umformungsminderung (cd 0,35, (v/8)^-0,6 über 8 m/s), nasse Rinde und Blätter, bedeckter Himmel."), "v10": 22.0, "sun": 0.15, "emit": 0.5, "wet": 0.6},
    ]
    S.make_reps = make_reps
    S.make_symbols = make_symbols
    return S


STEM = {
    "200": [(-0.5, 0.52), (-0.4, 0.43), (-0.28, 0.35), (-0.1, 0.30), (0.5, 0.275), (1.5, 0.265), (2.4, 0.26), (3.3, 0.245), (4.6, 0.21), (5.9, 0.175)],
    "100": [(-0.5, 0.56), (-0.44, 0.48), (-0.36, 0.40), (-0.22, 0.325), (0.0, 0.295), (0.8, 0.282), (1.7, 0.27), (2.4, 0.262), (3.0, 0.25), (3.8, 0.23), (4.8, 0.205), (5.9, 0.175)],
    "50": [(-0.5, 0.6), (-0.46, 0.54), (-0.4, 0.47), (-0.32, 0.4), (-0.2, 0.335), (-0.05, 0.3), (0.5, 0.284), (1.2, 0.274), (1.8, 0.268), (2.4, 0.262), (2.9, 0.255), (3.5, 0.24), (4.2, 0.22), (5.0, 0.2), (5.9, 0.175)],
}


def stem_shapes(S, sc):
    r = S.rig
    t = r.bones["trunk"]
    asym = {"200": [0.0, 0.006, -0.004], "100": [0.0, 0.005, -0.006, 0.004], "50": [0.0, 0.004, -0.006, 0.005, -0.003]}[sc]
    out = [Shape("stem", t, "body", cr_closed(stem_pts(t, STEM[sc], asym)), True, 0.36, 1, region="trunk-bark", material="lime-bark")]
    if sc == "200":
        out.append(Shape("stem-fissure", t, "detail", cr_open(wavy(t, 0.4, 2.6, 0.08, 0.02, 2.0, n=6)), False, 0.2, 1.2))
    if sc == "100":
        for i, (s, off, ln) in enumerate([(0.3, 0.12, 1.2), (0.9, -0.1, 1.4), (1.6, 0.1, 1.1), (2.2, -0.1, 0.9)]):
            out.append(Shape("bark-ridge-%d" % (i + 1), t, "texture", cr_open(wavy(t, s, s + ln, off, 0.02, 1.5, n=6, phase=i)), False, 0.15, 1.2))
    if sc == "50":
        for i, (s, off, ln) in enumerate([(0.2, 0.17, 1.0), (0.3, -0.04, 1.3), (0.35, -0.2, 0.9), (1.3, 0.15, 1.1), (1.5, -0.1, 1.2), (1.45, 0.02, 0.8), (2.2, 0.12, 0.9), (2.3, -0.12, 0.8)]):
            out.append(Shape("bark-ridge-%d" % (i + 1), t, "texture", cr_open(wavy(t, s, s + ln, off, 0.018, 1.8, n=7, phase=i * 0.8)), False, 0.15, 1.2))
        for i, (s, off) in enumerate([(0.75, -0.08), (1.9, 0.16)]):
            out.append(Shape("bark-knot-%d" % (i + 1), t, "detail", cr_closed(ring_pts(t.to_world((s, off))[0], t.to_world((s, off))[1], 0.045, n=8, ry=0.07)), False, 0.2, 1.3))
    return out


def extras(S, sc):
    return []


C = None


def make_reps(S):
    global C
    C = dict(
        stem_bones=["root-plate", "trunk", "ld1"],
        min_wood_mm={"200": 0.5, "100": 0.45, "50": 0.4}, min_stroke_wood_mm={"100": 1.0, "50": 0.4},
        mass={
            "200": dict(lobe=1.5, amp=0.08, vary=[1.0, 0.75, 1.15, 0.85, 1.05], under=0.12, pointed=0.7, n=5, rx_s=0.97, ry_s=0.97, phase=0.25, dz=0.2, epi_lobe=0.6),
            "100": dict(lobe=0.95, amp=0.11, vary=[1.0, 0.7, 1.15, 0.8, 1.0, 0.9], under=0.1, pointed=0.6, n=6, rx_s=0.96, ry_s=0.96, phase=0.5, dz=0.3, epi_lobe=0.6),
            "50": dict(lobe=0.55, amp=0.14, vary=[1.0, 0.65, 1.15, 0.8, 1.05, 0.7, 1.1], under=0.08, pointed=0.55, n=7, rx_s=0.96, ry_s=0.96, phase=0.3, dz=0.4, epi_lobe=0.6),
        },
        always_stroke=("epi-e", "epi-w"), glyph=heart, glyph_kinds=("sun", "epi"), glyph_R=0.05, stems=stem_shapes, extras=extras, shapes500=shapes500, words=WORDS,
    )
    return make_reps_generic(S, C)


CROWN500 = [(0, 4.7), (1.0, 4.35), (2.0, 3.95), (3.0, 3.75), (3.9, 4.1), (4.5, 4.8), (4.75, 5.6), (4.65, 6.5), (4.75, 7.4), (4.55, 8.4), (4.1, 9.4), (3.6, 10.5), (3.0, 11.5), (2.3, 12.6), (1.4, 13.5), (0.5, 14.15), (-0.4, 14.1), (-1.2, 13.5), (-2.0, 12.6), (-2.7, 11.5), (-3.3, 10.5), (-3.9, 9.4), (-4.4, 8.4), (-4.7, 7.4), (-4.6, 6.5), (-4.7, 5.6), (-4.45, 4.8), (-3.85, 4.1), (-3.0, 3.8), (-2.0, 4.0), (-1.0, 4.4)]


def shapes500(S):
    r = S.rig
    t, ld1 = r.bones["trunk"], r.bones["ld1"]
    stem_st = [(-0.5, 0.5), (-0.38, 0.4), (-0.22, 0.32), (0.1, 0.285), (1.5, 0.268), (2.4, 0.26), (3.3, 0.24), (4.1, 0.22), (4.9, 0.2)]
    stem = Shape("stem", t, "body", cr_closed(stem_pts(t, stem_st, [0.0, 0.004])), True, 0.36, 1, region="trunk-bark", material="lime-bark")
    crown = Shape("crown", ld1, "body", cr_closed([(x * 0.97, y) for x, y in CROWN500]), True, 0.55, 3, regions=["leaf-sun", "leaf-shade", "limb-bark"], material="lime-leaf-sun")
    epi = Shape("epicormic", t, "body", cr_closed([(0.2, 0.95), (0.7, 1.35), (1.25, 1.85), (1.3, 2.35), (0.95, 2.45), (0.55, 2.1), (0.3, 1.55)]), True, 0.5, 2, regions=["leaf-epicormic", "epicormic-stem"], material="lime-leaf-epicormic")
    return [stem, crown, epi]


WORDS = {
    "s500_sil": ("The small-leaved lime is composed as one closed, smoothly domed crown that is widest at about 6.5 m and tapers into a blunt top, with a low lobed underside above a straight stem; a small separate leaf tuft marks the epicormic shoots on the stem. 1 m is 2 mm.", "Die Winterlinde ist als eine geschlossene, glatt gewölbte Krone komponiert, die bei etwa 6,5 m am breitesten ist und in einen stumpfen Wipfel ausläuft, mit tiefer gelappter Unterseite über einem geraden Stamm; ein kleines abgesetztes Laubbüschel zeigt die Stammaustriebe. 1 m sind 2 mm."),
    "s500_ret": ("Retained: the stem with its root flare, the crown outline with two low side lobes, and one tuft for the epicormic shoots (both clusters merged).", "Beibehalten: der Stamm mit Wurzelanlauf, der Kronenumriss mit zwei tiefen Seitenlappen und ein Büschel für die Stammaustriebe (beide Büschel vereint)."),
    "s500_mer": ("Merged into the crown path: ten sun foliage masses, both shade cores, the leader and all limbs.", "Im Kronenpfad vereint: zehn Sonnenlaubmassen, beide Schattenkerne, die Stammverlängerung und alle Äste."),
    "s500_omi": ("Omitted: single limbs, twigs, bark ridges and leaf marks; they are far below 0.1 mm line width at this scale. No internal edge.", "Weggelassen: einzelne Äste, Zweige, Rindenrippen und Blattzeichen; sie liegen bei diesem Maßstab weit unter 0,1 mm Strichbreite. Keine innere Kante."),
    "s500_thin": ("Limbs and twigs are not thickened; they are merged into the filled crown, so the outline stays within the real outline tolerance of the 1:50 drawing.", "Äste und Zweige werden nicht verdickt, sondern in die gefüllte Krone eingeschmolzen, so dass der Umriss innerhalb der realen Umrisstoleranz der 1:50-Zeichnung bleibt."),
    "s500_ovl": ("Overlays map onto: trunk-bark (stem), the epicormic tuft (leaf-epicormic and epicormic-stem, merged) and, merged in the crown path, leaf-sun, leaf-shade and limb-bark; merged shapes show the area-weighted value of their regions, labelled as merged.", "Überlagerungen bilden sich ab auf: trunk-bark (Stamm), das Austriebsbüschel (leaf-epicormic und epicormic-stem, vereint) sowie, im Kronenpfad vereint, leaf-sun, leaf-shade und limb-bark; vereinte Formen zeigen den flächengewichteten Wert ihrer Regionen, als vereint gekennzeichnet."),
    "s200_sil": ("At 1:200 (1 m is 5 mm) the lime is separated into its main masses: stem with root flare, the limbs of the four tiers and thirteen dense foliage masses (ten sun masses, two shade cores, the epicormic leaves) as smoothly scalloped outlines that overlap into one dome.", "Bei 1:200 (1 m sind 5 mm) wird die Linde in ihre Hauptmassen aufgeteilt: Stamm mit Wurzelanlauf, die Äste der vier Tiers und dreizehn dichte Laubmassen (zehn Sonnenmassen, zwei Schattenkerne, die Austriebsblätter) als glatt gewellte Umrisse, die sich zu einer Kuppel überlappen."),
    "s200_ret": ("Retained: stem, the two epicormic tufts, the thicker limbs and the leader as tapered outlines where they stay visible between the masses.", "Beibehalten: Stamm, die beiden Austriebsbüschel, die stärkeren Äste und die Stammverlängerung als verjüngte Umrisse, wo sie zwischen den Massen sichtbar bleiben."),
    "s200_mer": ("Merged into the mass outlines: twigs, leaf clusters and the heart-shaped leaves; thin limbs of the crown tier stay under the masses.", "In den Massenumrissen vereint: Zweige, Blattbüschel und die herzförmigen Blätter; dünne Äste der Kronenstufe bleiben unter den Massen."),
    "s200_omi": ("Omitted: twigs, bark ridges, knots and leaf marks and all texture strokes.", "Weggelassen: Zweige, Rindenrippen, Astknoten und Blattzeichen sowie alle Texturstriche."),
    "s200_thin": ("Limbs thinner than 0.5 mm on paper are not drawn as filled outlines; the drawn ones keep their real width and run into the masses (joined, no gap).", "Äste, die auf dem Papier dünner als 0,5 mm wären, werden nicht als gefüllte Umrisse gezeichnet; die gezeichneten behalten ihre reale Breite und münden in die Massen (verbunden, kein Spalt)."),
    "edge200": ("one bark fissure line on the stem.", "eine Rindenrisslinie am Stamm."),
    "s200_ovl": ("Every thermal region keeps its own shapes (stem, limbs, sun, shade and epicormic masses); overlays follow the bone-bound masses, so sway and loads act on the drawn shapes.", "Jede thermische Region behält ihre eigenen Formen (Stamm, Äste, Sonnen-, Schatten- und Austriebsmassen); Überlagerungen folgen den knochengebundenen Massen, so dass Schwingung und Lasten an den gezeichneten Formen wirken."),
    "s100_sil": ("At 1:100 (1 m is 10 mm) the masses get finer lobes and the structure appears: the ridged stem with four ridge strokes, the limbs of all tiers reaching into the masses, two leaf-cluster arcs per mass, and the epicormic shoots with their leaf tufts.", "Bei 1:100 (1 m sind 10 mm) erhalten die Massen feinere Lappen und die Struktur erscheint: der gefurchte Stamm mit vier Rippenstrichen, die Äste aller Tiers, die in die Massen reichen, zwei Blattbüschelbögen je Masse und die Stammaustriebe mit ihren Blattbüscheln."),
    "s100_ret": ("Retained: thirteen foliage masses, stem, leader, all limbs thicker than 0.45 mm on paper and the epicormic stems as thin strokes.", "Beibehalten: dreizehn Laubmassen, Stamm, Stammverlängerung, alle Äste über 0,45 mm Papierbreite und die Austriebsstängel als dünne Striche."),
    "s100_mer": ("Single leaves and twig ends are merged into lobed mass outlines; bark relief into four ridge strokes.", "Einzelne Blätter und Zweigenden sind in gelappte Massenumrisse eingeschmolzen; das Rindenrelief in vier Rippenstriche."),
    "s100_omi": ("Omitted: individual leaves, fine twig tufts and knots; the fine 1:50 leaf marks are replaced by two arcs per mass.", "Weggelassen: einzelne Blätter, feine Zweigbüschel und Astknoten; die feinen 1:50-Blattzeichen sind durch zwei Bögen je Masse ersetzt."),
    "s100_thin": ("Thin strokes are at least 0.1 mm: ridges and fissures use the texture pen, limbs keep their real section width.", "Dünne Striche sind mindestens 0,1 mm: Rippen und Risse mit dem Texturstift, Äste in realer Querschnittsbreite."),
    "s100_ovl": ("Every region keeps its shapes; the shade hatch is a pure drawing hatch and does not stand for temperature.", "Jede Region behält ihre Formen; die Schattenschraffur ist eine reine Zeichenschraffur und steht nicht für Temperatur."),
    "s50_sil": ("At 1:50 (1 m is 20 mm) the lime is drawn in full: the ridged stem with eight ridge strokes and two knots, the tapered leader and limbs, the epicormic shoots, and finely lobed foliage masses with leaf-cluster arcs and heart-shaped leaf marks.", "Bei 1:50 (1 m sind 20 mm) wird die Linde vollständig gezeichnet: der gefurchte Stamm mit acht Rippenstrichen und zwei Astknoten, die verjüngte Stammverlängerung und Äste, die Stammaustriebe und fein gelappte Laubmassen mit Blattbüschelbögen und herzförmigen Blattzeichen."),
    "s50_ret": ("Retained: every woody member of the rig that is at least 0.4 mm wide on paper, every foliage mass; the leaf marks show the largest few leaves per mass at the real leaf size (about 0.08 m).", "Beibehalten: jedes Holzglied des Rigs, das auf dem Papier mindestens 0,4 mm breit ist, jede Laubmasse; die Blattzeichen zeigen die größten Blätter je Masse in der wahren Blattgröße (etwa 0,08 m)."),
    "s50_mer": ("Only the plug overlaps at the bone joints close the seams between tapered woody parts; the fine ridge relief is merged into eight strokes.", "Nur die Überlappungen an den Knochengelenken schließen die Nähte zwischen verjüngten Holzteilen; das feine Rippenrelief ist in acht Striche zusammengefasst."),
    "s50_omi": ("Omitted: the many thousand single leaves (1.6 mm on paper), buds and third-order twigs.", "Weggelassen: die vielen tausend einzelnen Blätter (1,6 mm auf dem Papier), Knospen und Zweige dritter Ordnung."),
    "s50_thin": ("Limb tips of 0.05 m (1 mm) stay filled; ridges and arcs are 0.13 mm strokes and the leaf marks 0.18 mm outlines, all above the pen minimum.", "Asttriebe von 0,05 m (1 mm) bleiben gefüllt; Rippen und Bögen sind 0,13-mm-Striche und die Blattzeichen 0,18-mm-Umrisse, alle über dem Stiftminimum."),
    "s50_ovl": ("Each thermal region has its own shapes; ridges, knots, arcs and leaf marks are strokes without a region and follow their bone.", "Jede thermische Region hat eigene Formen; Rippen, Knoten, Bögen und Blattzeichen sind Striche ohne Region und folgen ihrem Knochen."),
}


def make_symbols(S, bounds):
    root = S.rig.bones["root-plate"]
    W, D = 9.4, S.depth_m
    radii = [1.0, 0.98, 0.96, 0.99, 1.0, 0.97, 0.98, 1.0, 0.96, 0.99, 0.97, 0.98]
    crown = envelope(0, 0, W / 2, D / 2, radii, k=17, amp=0.04, phase=0.3, n=4)
    plan = [Shape("crown-extent", root, "body", cr_closed(crown), False, 0.5, 1), Shape("stem", root, "body", cr_closed(ring_pts(0, 0, 0.28, n=12)), True, 0.3, 3)]
    for i in range(4):
        a = (30 + 90 * i) * D2R
        plan.append(Shape("limb-%d" % (i + 1), root, "detail", cr_open([(0.3 * math.cos(a), 0.3 * math.sin(a)), (2.4 * math.cos(a), 2.4 * math.sin(a)), (3.6 * math.cos(a + 0.1), 3.6 * math.sin(a + 0.1))]), False, 0.3, 2))
    nominal = [Shape("sign-circle", root, "body", cr_closed(ring_pts(0, 0, 1.8, n=12)), False, 0.5, 1), Shape("sign-dot", root, "body", cr_closed(ring_pts(0, 0, 0.35, n=8)), True, 0.3, 2)]
    for s_ in plan + nominal:
        s_.frame = IDENT
    return [
        {"id": "plan-crown", "title": T("Plan symbol with true crown extent", "Grundrisssymbol mit wahrer Kronenausdehnung"), "description": T("A conventional plan sign for site and landscape plans: the true plan outline of the crown, four limb directions and the stem section. It is a separate symbol and is never used silently for a pictorial scale.", "Ein konventionelles Grundrisszeichen für Lage- und Freiraumpläne: der wahre Grundrissumriss der Krone, vier Astrichtungen und der Stammquerschnitt. Es ist ein eigenes Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet."),
         "semantics": "true-extent", "units": "model-m", "denominators": [500, 200, 100], "pens": PENS["100"], "hatches": [], "geometry": {"elevation": [], "plan": [s.json() for s in plan]},
         "dimensionalNote": T("The outline is the true plan extent of the crown in model metres (%.1f m by %.1f m) and the filled circle the true stem section of 0.56 m; the four strokes only indicate limb directions. At 1:500 the stem measures 1.1 mm on paper." % (W, D), "Der Umriss ist die wahre Kronenausdehnung im Grundriss in Modellmetern (%.1f m mal %.1f m) und der gefüllte Kreis der wahre Stammquerschnitt von 0,56 m; die vier Striche deuten nur Astrichtungen an. Bei 1:500 misst der Stamm 1,1 mm auf dem Papier." % (W, D))},
        {"id": "tree-sign", "title": T("Nominal tree sign with trunk centre", "Nominelles Baumzeichen mit Stammmitte"), "description": T("The conventional circle-and-dot tree sign of small-scale site plans: the dot marks the trunk centre, the circle is a fixed paper size.", "Das übliche Kreis-und-Punkt-Baumzeichen kleinmaßstäblicher Lagepläne: Der Punkt markiert die Stammmitte, der Kreis hat eine feste Papiergröße."),
         "semantics": "nominal", "units": "paper-mm", "denominators": [500, 200], "pens": PENS["500"], "hatches": [], "geometry": {"elevation": [], "plan": [s.json() for s in nominal]},
         "dimensionalNote": T("A fixed 3.6 mm circle with a 0.7 mm centre dot in paper millimetres; its size carries no dimension. Use the true-extent plan symbol where the crown width matters.", "Ein fester Kreis von 3,6 mm mit 0,7 mm Mittelpunkt in Papiermillimetern; seine Größe trägt kein Maß. Wo die Kronenbreite zählt, ist das Grundrisssymbol mit wahrer Ausdehnung zu verwenden.")},
    ]
