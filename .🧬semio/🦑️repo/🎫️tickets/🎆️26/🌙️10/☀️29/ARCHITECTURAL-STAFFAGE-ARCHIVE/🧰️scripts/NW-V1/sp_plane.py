"""london-plane-pollarded: authored species description (rig, masses, thermal, poses and the four drawings)."""
import math
from lib import *
from common import *
from reps import *


class Sp:
    pass


def palmate(cx, cy, R, rot=0.0):
    pts = []
    for i in range(10):
        a = (90 + 36 * i) * D2R
        r = R if i % 2 == 0 else R * 0.5
        pts.append((cx + r * math.cos(a), cy + r * math.sin(a)))
    return rot_pts(pts, cx, cy, rot)


def build():
    S = Sp()
    S.id = "london-plane-pollarded"
    S.folder = "london-plane-pollarded"
    S.title = T("London Plane, Pollarded", "Platane, gekappt (Kopfbaum)")
    S.summary = T("A street plane tree held as a pollard: a clear grey-cream mottled bole of 3.8 m ends in a knuckled head from which one summer of whippy shoots carries a dense upright crown of large palmate leaves.",
                  "Eine als Kopfbaum gehaltene Straßenplatane: Ein grau-cremefarben gescheckter Schaft von 3,8 m endet in einem knorrigen Kopf, aus dem die Triebe eines Sommers eine dichte, aufrechte Krone aus großen, gelappten Blättern tragen.")
    S.tags = ["tree", "deciduous", "broadleaf", "pollard", "street-tree", "wind", "transpiration"]
    S.depth_m = 6.5
    S.footprint = 1.8
    S.density = 950
    S.root_mass_factor = 1.25
    S.default_scale = 100
    S.default_pose = "summer-calm"
    S.breeze_pose = "summer-breeze"
    S.breeze_v10 = 5.5
    S.gust_peak = 14.0
    S.trunk_bone = "trunk"
    S.twig_factor = {"leaf-sun": 1.6, "leaf-shade": 1.6}
    S.cd = {}
    S.bark_extra = {"head": 1.25}

    r = Rig()
    S.rig = r
    WOOD = "plane-wood"
    r.add("root-plate", None, (0, 0), 90, 0.45, T("Root plate and flare", "Wurzelteller und Wurzelanlauf"), 1.0, 0.70, limit=0, material=WOOD).region = "trunk-bark"
    r.add("trunk", "root-plate", (0, 0.45), 90.5, 3.30, T("Clear stem", "Schaft"), 0.64, 0.58, limit=3.5, material=WOOD).region = "trunk-bark"
    r.add("head", "trunk", r.tip("trunk"), 90, 0.80, T("Pollard head", "Kopf (Kappstelle)"), 0.62, 0.78, limit=3.5, material=WOOD).region = "pollard-bark"
    knuckles = [("w2", (-0.30, 4.25), 158, 0.55), ("w1", (-0.17, 4.50), 125, 0.60), ("c", (0.0, 4.55), 92, 0.50), ("e1", (0.17, 4.50), 58, 0.60), ("e2", (0.30, 4.25), 22, 0.55)]
    kn_title = {"w2": ("Outer west knuckle", "Äußere Westknolle"), "w1": ("Inner west knuckle", "Innere Westknolle"), "c": ("Central knuckle", "Mittlere Knolle"), "e1": ("Inner east knuckle", "Innere Ostknolle"), "e2": ("Outer east knuckle", "Äußere Ostknolle")}
    for k, base, ang, L in knuckles:
        r.add("k-" + k, "head", base, ang, L, T(*kn_title[k]), 0.24, 0.20, limit=6, material=WOOD).region = "pollard-bark"
    sh = {"w2": (150, 1.6, 105, 1.8), "w1": (118, 1.8, 100, 1.6), "c": (90, 2.0, 94, 1.6), "e1": (62, 1.8, 80, 1.6), "e2": (30, 1.6, 75, 1.8)}
    sh_title = {"w2": ("outer west", "außen West"), "w1": ("inner west", "innen West"), "c": ("central", "Mitte"), "e1": ("inner east", "innen Ost"), "e2": ("outer east", "außen Ost")}
    for k, (a1, l1, a2, l2) in sh.items():
        ta, td = sh_title[k]
        a = r.add("s-%sa" % k, "k-" + k, r.tip("k-" + k), a1, l1, T("Shoot %s, lower part" % ta, "Trieb %s, unterer Teil" % td), 0.10, 0.07, limit=20, material=WOOD)
        a.region = "shoot-bark"
        b = r.add("s-%sb" % k, "s-%sa" % k, r.tip("s-%sa" % k), a2, l2, T("Shoot %s, upper part" % ta, "Trieb %s, oberer Teil" % td), 0.07, 0.03, limit=25, material=WOOD)
        b.region = "shoot-bark"
    S.levels = {"trunk": "trunk", "head": "head"}
    for k in sh:
        S.levels["k-" + k] = "limb"
        S.levels["s-%sa" % k] = "shoota"
        S.levels["s-%sb" % k] = "shootb"
    S.lean_ref = {"trunk": 0.3, "head": 0.1, "limb": 0.3, "shoota": 1.0, "shootb": 1.5, "fol": 3.0}
    S.droop_levels = {}
    S.sway_amp = {"trunk": 0.5, "head": 0.5, "limb": 0.5, "shoota": 0.55, "shootb": 0.6, "fol": 0.7}
    S.sway_lag = 0.38
    S.sway_period = 3.0
    S.gust_tau = {"trunk": 0.7, "head": 0.7, "limb": 0.6, "shoota": 0.45, "shootb": 0.35, "fol": 0.2}
    S.sway_desc_en = "A deterministic %s s sway cycle about the breeze lean (5.5 m/s at 10 m; estimated first mode of the stiff pollard stem and its whippy shoots about 0.33 Hz). Every level lags the one below it, the foliage masses swing about 3 degrees."
    S.sway_desc_de = "Ein deterministischer Schwingungszyklus von %s s um die Neigung bei Brise (5,5 m/s in 10 m Höhe; geschätzte erste Eigenform des steifen Kopfbaumstamms mit seinen biegsamen Trieben, etwa 0,33 Hz). Jede Ebene hinkt der darunter nach, die Laubmassen schwingen um etwa 3 Grad."
    S.gust_desc_en = "A single gust over 8 s from %.1f to %.1f m/s at 10 m: the wind-load tracks rise to the gust peak with the reconfigured drag law and the crown leans with a level-wise lag, whippy shoots first, the stem last."
    S.gust_desc_de = "Eine einzelne Böe über 8 s von %.1f auf %.1f m/s in 10 m Höhe: Die Windlast-Spuren steigen mit dem umgeformten Widerstandsgesetz auf die Böenspitze, die Krone neigt sich ebenenweise verzögert, zuerst die biegsamen Triebe, zuletzt der Stamm."

    S.masses = [
        Mass("fol-core", "s-ca", (0.0, 6.4), 1.55, 1.9, 0, "leaf-shade", "shade", T("shaded inner foliage", "beschattete Innenlaubmasse")),
        Mass("fol-w2", "s-w2b", (-2.4, 6.2), 1.1, 1.5, 12, "leaf-sun", "sun", T("outer west foliage", "Laubmasse außen West")),
        Mass("fol-w1", "s-w1b", (-1.25, 7.35), 1.0, 1.5, 7, "leaf-sun", "sun", T("inner west foliage", "Laubmasse innen West")),
        Mass("fol-top", "s-cb", (0.0, 8.0), 1.15, 1.0, 0, "leaf-sun", "sun", T("crown top foliage", "Laubmasse Kronenspitze")),
        Mass("fol-e1", "s-e1b", (1.3, 7.3), 1.0, 1.5, -7, "leaf-sun", "sun", T("inner east foliage", "Laubmasse innen Ost")),
        Mass("fol-e2", "s-e2b", (2.4, 6.15), 1.1, 1.5, -12, "leaf-sun", "sun", T("outer east foliage", "Laubmasse außen Ost")),
        Mass("fol-low-w", "s-w2a", (-1.6, 5.4), 1.0, 1.0, 8, "leaf-sun", "sun", T("low west foliage", "Laubmasse tief West")),
        Mass("fol-low-e", "s-e2a", (1.6, 5.4), 1.0, 0.98, -8, "leaf-sun", "sun", T("low east foliage", "Laubmasse tief Ost")),
    ]
    S.regions_def = None
    proj = math.pi / 4 * 6.7 * S.depth_m
    S.leaf_area = {"leaf-sun": 1.9 * proj, "leaf-shade": 1.5 * proj}
    S.mat_of = {"trunk-bark": "plane-bark", "pollard-bark": "plane-bark-scarred", "shoot-bark": "plane-twig", "leaf-sun": "plane-leaf-sun", "leaf-shade": "plane-leaf-shade"}
    S.tone = {"wood": 0.35, "sun": 0.6, "shade": 0.45}
    S.tone_line = 0.2
    S.sun = {"trunk-bark": 0.35, "pollard-bark": 0.45, "shoot-bark": 0.3, "leaf-sun": 0.9, "leaf-shade": 0.08}
    S.regions = [
        {"id": "trunk-bark", "title": T("Mottled bark of the clear stem", "Gescheckte Rinde des Schafts"), "kind": "bark", "model": "passive", "material": "plane-bark", "len": 0.6, "geom": "cylinder", "sky": 0.45, "hc": 67500, "ref": "wood-handbook-2021"},
        {"id": "pollard-bark", "title": T("Scarred bark of the pollard head and knuckles", "Vernarbte Rinde von Kopf und Knollen"), "kind": "bark", "model": "passive", "material": "plane-bark-scarred", "len": 0.3, "geom": "cylinder", "sky": 0.6, "hc": 81000, "ref": "wood-handbook-2021"},
        {"id": "shoot-bark", "title": T("Bark of the one-year shoots", "Rinde der einjährigen Triebe"), "kind": "stem", "model": "passive", "material": "plane-twig", "len": 0.06, "geom": "cylinder", "sky": 0.65, "hc": 30000, "ref": "wood-handbook-2021"},
        {"id": "leaf-sun", "title": T("Sun leaves of the outer crown", "Sonnenblätter der Außenkrone"), "kind": "foliage", "model": "leaf", "material": "plane-leaf-sun", "area": S.leaf_area["leaf-sun"], "len": 0.12, "geom": "vertical-plate", "sky": 0.7, "hc": 900, "gs": 0.0060, "lai": 1.9, "ref": "jones-2014"},
        {"id": "leaf-shade", "title": T("Shade leaves of the inner crown", "Schattenblätter der Innenkrone"), "kind": "foliage", "model": "leaf", "material": "plane-leaf-shade", "area": S.leaf_area["leaf-shade"], "len": 0.12, "geom": "vertical-plate", "sky": 0.3, "hc": 700, "gs": 0.0035, "lai": 1.5, "ref": "jones-2014"},
    ]
    S.emitters = [
        {"id": "transpiration-sun", "aid": "tp-sun", "region": "leaf-sun", "strength": 1.0, "diam": 2.2, "title": T("Transpiration of the sun leaves", "Transpiration der Sonnenblätter"),
         "anchors": [("fol-w2", (-2.3, 6.3)), ("fol-w1", (-1.25, 7.6)), ("fol-top", (0.05, 8.2)), ("fol-e1", (1.3, 7.5)), ("fol-e2", (2.3, 6.3)), ("fol-low-w", (-1.55, 5.6)), ("fol-low-e", (1.55, 5.6))]},
        {"id": "transpiration-shade", "aid": "tp-shade", "region": "leaf-shade", "strength": 0.7, "diam": 1.6, "title": T("Transpiration of the shade leaves", "Transpiration der Schattenblätter"),
         "anchors": [("fol-core", (-0.2, 5.0)), ("fol-core", (0.2, 5.7)), ("fol-core", (-0.1, 6.4)), ("fol-core", (0.2, 7.1)), ("fol-core", (-0.15, 7.8))]},
    ]
    S.materials = [
        {"id": "plane-bark", "title": T("Plane bark, mottled and flaking", "Platanenrinde, gescheckt und abblätternd"), "emissivity": 0.95, "solarAbsorptivity": 0.55, "densityKgM3": 650, "conductivityWmK": 0.12, "source": "authored-estimate", "reference": "wood-handbook-2021"},
        {"id": "plane-bark-scarred", "title": T("Scarred callus bark of the pollard head", "Vernarbte Kallusrinde des Kopfes"), "emissivity": 0.95, "solarAbsorptivity": 0.78, "densityKgM3": 700, "conductivityWmK": 0.13, "source": "authored-estimate", "reference": "wood-handbook-2021"},
        {"id": "plane-twig", "title": T("Green-brown bark of young shoots", "Grünbraune Rinde junger Triebe"), "emissivity": 0.95, "solarAbsorptivity": 0.6, "densityKgM3": 750, "conductivityWmK": 0.14, "source": "authored-estimate", "reference": "wood-handbook-2021"},
        {"id": "plane-leaf-sun", "title": T("Plane sun leaves", "Platanen-Sonnenblätter"), "emissivity": 0.97, "solarAbsorptivity": 0.5, "source": "literature", "reference": "jones-2014"},
        {"id": "plane-leaf-shade", "title": T("Plane shade leaves (effective, shaded by the outer crown)", "Platanen-Schattenblätter (wirksam, durch die Außenkrone beschattet)"), "emissivity": 0.97, "solarAbsorptivity": 0.25, "source": "authored-estimate", "reference": "jones-2014"},
        {"id": WOOD, "title": T("Green plane wood", "Frisches Platanenholz"), "emissivity": 0.9, "solarAbsorptivity": 0.65, "densityKgM3": 950, "conductivityWmK": 0.16, "elasticModulusPa": 7000000000, "strengthPa": 40000000, "source": "authored-estimate", "reference": "wood-handbook-2021"},
    ]
    S.references = common_refs() + [
        {"id": "bs-3998-2010", "citation": "BS 3998:2010 Tree work - Recommendations. British Standards Institution. Clause on pollarding: crown reduction to a framework of knuckles (pollard head) and regular re-cutting of the regrowth."},
    ]
    S.notes = T("Rig and drawings are authored by hand for a mature street plane of about 50 years re-pollarded every second winter: stem 3.8 m, pollard head at 3.75 to 4.55 m, five knuckles and the regrowth of one summer (about 4 m). Dimensions follow typical pruned street planes; wood properties, masses and the leaf conductance are authored estimates guided by the Wood Handbook and Jones (2014).",
                 "Rig und Zeichnungen sind von Hand für eine ausgewachsene Straßenplatane von etwa 50 Jahren entworfen, die jeden zweiten Winter neu gekappt wird: Schaft 3,8 m, Kopf auf 3,75 bis 4,55 m, fünf Knollen und der Neuaustrieb eines Sommers (etwa 4 m). Die Maße folgen typischen geschnittenen Straßenplatanen; Holzeigenschaften, Massen und Blattleitfähigkeit sind Autorenschätzungen, angelehnt an das Wood Handbook und Jones (2014).")
    S.environment = {"airTempC": 24, "meanRadiantTempC": 28, "relativeHumidity": 0.5, "windMS": 1.5, "windDirectionDeg": 0, "solarWM2": 700, "pressurePa": 101325, "diffuseFraction": 0.15}
    S.thermal_assumptions = tree_thermal_assumptions("palmate plane leaves", "palmate Platanenblätter")
    S.structure_assumptions = tree_structure_assumptions("plane", "Platane")
    S.poses = [
        {"id": "summer-calm", "title": T("Calm summer day", "Windstiller Sommertag"), "desc": T("Light air of 1.5 m/s at 10 m: the crown stands at rest, sun leaves lit, transpiration at 80 % of the daytime maximum.", "Leiser Zug von 1,5 m/s in 10 m Höhe: Die Krone steht in Ruhe, Sonnenblätter beleuchtet, Transpiration bei 80 % des Tagesmaximums."), "v10": 1.5, "sun": 1.0, "emit": 0.8},
        {"id": "summer-breeze", "title": T("Summer breeze", "Sommerbrise"), "desc": T("Gentle to moderate breeze of 5.5 m/s at 10 m (Beaufort 3 to 4): shoots and foliage lean downwind, drag by the reconfigured law F = 0.5 rho cd A v^2 (cd 0.35, Vogel-type reduction above 8 m/s).", "Schwache bis mäßige Brise von 5,5 m/s in 10 m Höhe (Beaufort 3 bis 4): Triebe und Laub neigen sich in Windrichtung, Widerstand nach dem umgeformten Gesetz F = 0,5 rho cd A v^2 (cd 0,35, Vogel-artige Minderung über 8 m/s)."), "v10": 5.5, "sun": 1.0, "emit": 0.95},
        {"id": "midday-heat", "title": T("Heat at midday", "Mittagshitze"), "desc": T("Still, hot midday: the foliage is fully sunlit but the stomata close partly under the high vapour pressure deficit, transpiration at 45 % of its maximum; the leaves run warmer than the air.", "Windstill, heißer Mittag: Das Laub ist voll besonnt, die Stomata schließen sich aber teilweise unter dem hohen Sättigungsdefizit, Transpiration bei 45 % des Maximums; die Blätter werden wärmer als die Luft."), "v10": 1.0, "sun": 1.0, "emit": 0.45},
        {"id": "autumn-leaf-fall", "title": T("Autumn, senescing leaves", "Herbst, alternde Blätter"), "desc": T("Damp autumn day: the leaves are yellowing and the stomata are mostly closed (transpiration 20 %), low sun fraction, dew on the bark and leaves. The drawn foliage stays as authored: shapes are not toggled by a pose.", "Feuchter Herbsttag: Die Blätter vergilben und die Stomata sind meist geschlossen (Transpiration 20 %), geringer Sonnenanteil, Tau auf Rinde und Blättern. Das gezeichnete Laub bleibt wie entworfen: Formen werden von einer Pose nicht ein- oder ausgeblendet."), "v10": 3.0, "sun": 0.55, "emit": 0.2, "wet": 0.3},
        {"id": "gale", "title": T("Gale with rain", "Sturm mit Regen"), "desc": T("Beaufort 9, 22 m/s at 10 m with rain: the crown leans strongly downwind, drag with the reconfiguration reduction (cd 0.35, (v/8)^-0.6 above 8 m/s), wet bark and leaves, overcast sky.", "Beaufort 9, 22 m/s in 10 m Höhe mit Regen: Die Krone neigt sich stark in Windrichtung, Widerstand mit der Umformungsminderung (cd 0,35, (v/8)^-0,6 über 8 m/s), nasse Rinde und Blätter, bedeckter Himmel."), "v10": 22.0, "sun": 0.15, "emit": 0.5, "wet": 0.6},
    ]
    S.make_reps = make_reps
    S.make_symbols = make_symbols
    return S


def common_refs():
    return [
        {"id": "wood-handbook-2021", "citation": "Forest Products Laboratory (2021). Wood Handbook: Wood as an Engineering Material. General Technical Report FPL-GTR-282, USDA Forest Service. Chapter 5 (mechanical properties, green wood); used as a guide, species values are authored estimates."},
        {"id": "en-1991-1-4", "citation": "EN 1991-1-4:2005 Eurocode 1, Actions on structures, Part 1-4: Wind actions. Air density 1.25 kg/m3, drag force F = 0.5 rho cd A v^2, power-law profile."},
        {"id": "vogel-1989", "citation": "Vogel, S. (1989). Drag and reconfiguration of broad leaves in high winds. Journal of Experimental Botany 40(8), 941-948."},
        {"id": "jones-2014", "citation": "Jones, H. G. (2014). Plants and Microclimate, 3rd ed., Cambridge University Press. Leaf energy balance, leaf area index, stomatal and boundary-layer conductance."},
        {"id": "monteith-unsworth-2013", "citation": "Monteith, J. L. and Unsworth, M. H. (2013). Principles of Environmental Physics, 4th ed., Academic Press. Penman-Monteith leaf transpiration."},
        {"id": "pretzsch-2015", "citation": "Pretzsch, H., Biber, P., Uhl, E. et al. (2015). Crown size and growing space requirement of common tree species in urban centres, parks, and forests. Urban Forestry and Urban Greening 14(3), 466-479."},
        {"id": "en-1991-1-3", "citation": "EN 1991-1-3:2003 Eurocode 1, Actions on structures, Part 1-3: Snow loads. Used for the order of magnitude of wet-snow accretion on foliage."},
    ]


def tree_thermal_assumptions(leaf_en, leaf_de):
    return [
        T("Leaf regions use one-sided leaf area; the leaf area index is that area over the crown ground projection. The maximum stomatal conductance is an authored estimate for the species type, guided by Jones (2014) (typical broadleaf range 0.003 to 0.008 m/s); no published species maximum is claimed. Photosynthesis type: C3. Aperture and night closure are the engine's business, not authored here.",
          "Blattregionen verwenden die einseitige Blattfläche; der Blattflächenindex ist diese Fläche bezogen auf die Kronenprojektion am Boden. Die maximale Spaltöffnungsleitfähigkeit ist eine Autorenschätzung für den Artentyp, angelehnt an Jones (2014) (typischer Laubholzbereich 0,003 bis 0,008 m/s); es wird kein veröffentlichtes Artmaximum behauptet. Photosynthesetyp: C3. Öffnung und nächtliches Schließen sind Sache der Engine und werden hier nicht vorgegeben."),
        T("Bark, stems and shoots are passive. Their areal heat capacity is the volumetric heat capacity of green wood (about 2.7 MJ per m3 and K) times an effective depth of 1 to 3 cm (a quarter of the diameter for thin members), an authored estimate guided by the Wood Handbook. No surface temperature is authored.",
          "Rinde, Stämme und Triebe sind passiv. Ihre flächenbezogene Wärmekapazität ist die volumetrische Wärmekapazität von frischem Holz (etwa 2,7 MJ je m3 und K) mal einer wirksamen Tiefe von 1 bis 3 cm (ein Viertel des Durchmessers bei dünnen Gliedern), eine Autorenschätzung angelehnt an das Wood Handbook. Es werden keine Oberflächentemperaturen vorgegeben."),
        T("Transpiration emitters release the latent flux of their foliage region as vapour, spread over at least five anchors across the crown; the direction is the vertical measured in the frame of the first anchor's bone, since the release is diffuse. Sun and shade foliage are separate regions: the sun leaves form the outer masses, the shade leaves the inner core, and each pose authors the sunlit share of each region.",
          "Transpirationsemitter geben den latenten Strom ihrer Laubregion als Dampf ab, verteilt auf mindestens fünf Anker über die Krone; die Richtung ist die Senkrechte im Bezugssystem des Knochens des ersten Ankers, da die Abgabe diffus ist. Sonnen- und Schattenlaub sind getrennte Regionen: Die Sonnenblätter bilden die äußeren Massen, die Schattenblätter den inneren Kern, und jede Pose gibt den besonnten Anteil jeder Region vor."),
        T("Leaf shedding, leaf colour and the snow-free or fruit-free states are not geometric: a pose cannot hide shapes, so seasonal poses change the thermal binding (stomatal closure, wetness, sun) and the loads only.",
          "Laubfall, Laubfarbe und schnee- oder fruchtfreie Zustände sind nicht geometrisch: Eine Pose kann keine Formen ausblenden, saisonale Posen ändern daher nur die thermische Bindung (Stomataschluss, Nässe, Sonne) und die Lasten."),
    ]


def tree_structure_assumptions(en, de):
    return [
        T("The tree is one rigid body per evaluated pose, fixed at the root plate; wind drag acts at the centre of each foliage mass and on the stem as a line load, gravity on every bone mass. The base moment follows from statics at the posed geometry; root-soil failure and dynamic amplification are not modelled. Pose lean angles are authored estimates scaled with the drag force, not a computed deflection.",
          "Der Baum ist je ausgewerteter Pose ein starrer Körper, am Wurzelteller eingespannt; Winddruck wirkt im Schwerpunkt jeder Laubmasse und am Stamm als Linienlast, Schwerkraft auf jede Knochenmasse. Das Fußmoment folgt aus der Statik der gezeigten Geometrie; Wurzel-Boden-Versagen und dynamische Überhöhung werden nicht modelliert. Die Neigungswinkel der Posen sind Autorenschätzungen proportional zur Widerstandskraft, keine berechnete Durchbiegung."),
        T("Section properties are solid circles of green wood (authored-estimate properties guided by the Wood Handbook); hollow or decayed stems are not represented.",
          "Querschnittswerte sind Vollkreise aus frischem Holz (Autorenschätzungen angelehnt an das Wood Handbook); hohle oder faule Stämme werden nicht abgebildet."),
    ]


STEM500 = [(0.0, 0.54), (0.12, 0.43), (0.3, 0.34), (0.5, 0.325), (1.5, 0.31), (3.0, 0.30), (3.95, 0.295)]
STEM200 = [(0.0, 0.57), (0.1, 0.47), (0.22, 0.38), (0.4, 0.34), (1.0, 0.32), (2.2, 0.305), (3.1, 0.30), (3.95, 0.30)]
STEM100 = [(0.0, 0.6), (0.06, 0.52), (0.16, 0.42), (0.3, 0.355), (0.55, 0.325), (1.2, 0.315), (2.0, 0.305), (2.8, 0.30), (3.4, 0.295), (3.95, 0.30)]
STEM50 = [(0.0, 0.64), (0.04, 0.58), (0.1, 0.5), (0.2, 0.42), (0.32, 0.365), (0.5, 0.335), (0.9, 0.322), (1.5, 0.314), (2.1, 0.308), (2.7, 0.30), (3.2, 0.298), (3.6, 0.30), (3.95, 0.305)]


def make_reps(S):
    r = S.rig
    trunk, head = r.bones["trunk"], r.bones["head"]
    bk = {m.id: m for m in S.masses}
    S.shapes = {}
    reps = {}

    #region 500
    stem = Shape("stem", trunk, "body", cr_closed(stem_pts(trunk, [(s - 0.45, w) for s, w in STEM500], [0.0, 0.004])), True, 0.35, 1, region="trunk-bark", material="plane-bark")
    knob = [(-0.30, 3.70), (-0.38, 4.05), (-0.85, 4.35), (-0.88, 4.62), (-0.45, 4.6), (-0.2, 4.75), (0.2, 4.75), (0.45, 4.6), (0.88, 4.62), (0.85, 4.35), (0.38, 4.05), (0.30, 3.70)]
    head500 = Shape("head", head, "body", cr_closed(knob), True, 0.3, 2, region="pollard-bark", material="plane-bark-scarred")
    crown_pts = [(0.0, 5.15), (0.9, 5.0), (1.6, 4.78), (2.3, 4.95), (2.95, 5.35), (3.3, 5.95), (3.35, 6.6), (3.1, 7.3), (2.6, 7.75), (2.05, 8.05), (1.6, 8.5), (1.0, 8.85), (0.2, 8.95), (-0.5, 8.85), (-1.1, 8.6), (-1.6, 8.3), (-2.2, 7.95), (-2.85, 7.5), (-3.3, 6.9), (-3.38, 6.2), (-3.2, 5.55), (-2.7, 5.05), (-2.0, 4.8), (-1.3, 4.9), (-0.6, 5.1)]
    crown = Shape("crown", head, "body", cr_closed(crown_pts), True, 0.55, 3, regions=["leaf-sun", "leaf-shade", "shoot-bark"], material="plane-leaf-sun")
    S.shapes["500"] = [stem, head500, crown]
    reps["500"] = rep_json(S, "500", S.shapes["500"], {
        "silhouette": ("The pollarded plane is composed as one closed, slightly lobed oval crown with a flat, deeply hanging underside above a clearly separate knuckled head and a straight stem; 1 m is 2 mm, so the head width of 1.8 m reads as a 3.6 mm swelling.", "Die gekappte Platane ist als eine geschlossene, leicht gelappte ovale Krone mit flacher, tief hängender Unterseite über einem deutlich abgesetzten knorrigen Kopf und einem geraden Stamm komponiert; 1 m sind 2 mm, die Kopfbreite von 1,8 m liest sich daher als 3,6 mm breite Verdickung."),
        "retained": ("Retained: the stem with its root flare, the knuckled head (five knuckles merged to one lobed outline) and the crown outline with its two low hanging side lobes.", "Beibehalten: der Stamm mit Wurzelanlauf, der knorrige Kopf (fünf Knollen zu einem gelappten Umriss vereint) und der Kronenumriss mit seinen zwei tief hängenden Seitenlappen."),
        "merged": ("Merged into the crown path: seven sun foliage masses, the shade core and all shoots; the five knuckles are merged into the head outline.", "Im Kronenpfad vereint: sieben Sonnenlaubmassen, der Schattenkern und alle Triebe; die fünf Knollen sind im Kopfumriss vereint."),
        "omitted": ("Omitted: single shoots, leaves, bark patches and scars; they are far below 0.1 mm line width at this scale. There is no internal edge.", "Weggelassen: einzelne Triebe, Blätter, Rindenschuppen und Narben; sie liegen bei diesem Maßstab weit unter 0,1 mm Strichbreite. Es gibt keine innere Kante."),
        "thinElements": ("Shoots, leaf stalks and scars are not thickened: they are merged into the filled crown or dropped, so the outline stays within the real outline tolerance of the 1:50 drawing; the crown keeps a 0.4 m (0.8 mm) clear space above the head.", "Triebe, Blattstiele und Narben werden nicht verdickt, sondern in die gefüllte Krone eingeschmolzen oder weggelassen, so dass der Umriss innerhalb der realen Umrisstoleranz der 1:50-Zeichnung bleibt; die Krone hält 0,4 m (0,8 mm) Freiraum über dem Kopf."),
        "hatching": ("No hatching; the pen is 0.15 mm for outlines and 0.25 mm for the ground line.", "Keine Schraffur; der Stift ist 0,15 mm für Umrisse und 0,25 mm für die Bodenlinie."),
        "styles": ("Light line: closed outlines only. Dark solid: the crown reads as one dark mass, the head darker than the lighter mottled stem.", "Helle Linie: nur geschlossene Umrisse. Dunkle Fläche: die Krone liest sich als eine dunkle Masse, der Kopf dunkler als der hellere gescheckte Stamm."),
        "overlays": ("Overlays map onto: trunk-bark (stem), pollard-bark (head) and, merged in the crown path, leaf-sun, leaf-shade and shoot-bark; the merged crown shows the area-weighted value of its regions, labelled as merged.", "Überlagerungen bilden sich ab auf: trunk-bark (Stamm), pollard-bark (Kopf) sowie, im Kronenpfad vereint, leaf-sun, leaf-shade und shoot-bark; die vereinte Krone zeigt den flächengewichteten Wert ihrer Regionen, als vereint gekennzeichnet."),
    })
    #endregion 500

    #region 200
    sh = []
    sh.append(Shape("stem", trunk, "body", cr_closed(stem_pts(trunk, [(s - 0.45, w) for s, w in STEM200], [0.0, 0.006, -0.004])), True, 0.35, 1, region="trunk-bark", material="plane-bark"))
    sh.append(Shape("stem-fissure", trunk, "detail", cr_open(wavy(trunk, 0.5, 2.9, 0.1, 0.02, 2.0, n=6)), False, 0.2, 1.2))
    sh.append(tapered(S, head, "200", 5, [1.0, 1.08, 1.22, 1.2, 1.05], 2, id="head"))
    for k in ["w2", "w1", "c", "e1", "e2"]:
        b = r.bones["k-" + k]
        sh.append(tapered(S, b, "200", 4, [0.8, 0.75, 0.9, 1.1], 2.1, id="knuckle-" + k))
        a = r.bones["s-%sa" % k]
        if k in ("w2", "e2"):
            sh.append(tapered(S, a, "200", 4, [1.0, 0.95, 0.9, 0.95], 2.2, id="shoot-%s" % k))
    for m in S.masses:
        shp, _ = mass_shape(S, m, "200", lobe=1.35, amp=0.09, vary=[1.0, 0.7, 1.2, 0.85, 1.1], z=3 + (0 if m.kind == "shade" else 0.3), phase=0.2, under=0.15, pointed=0.7, n=5, rx_s=0.95, ry_s=0.97)
        sh.append(shp)
    S.shapes["200"] = sh
    reps["200"] = rep_json(S, "200", sh, {
        "silhouette": ("At 1:200 (1 m is 5 mm) the plane is separated into its main masses: the bole, the knuckled head with five distinct knuckles and eight foliage masses (seven sun masses, the shade core) as separate scalloped outlines that overlap into one dome.", "Bei 1:200 (1 m sind 5 mm) wird die Platane in ihre Hauptmassen aufgeteilt: Stamm, knorriger Kopf mit fünf erkennbaren Knollen und acht Laubmassen (sieben Sonnenmassen, der Schattenkern) als getrennte, gewellte Umrisse, die sich zu einer Kuppel überlappen."),
        "retained": ("Retained: stem with flare, head, five knuckles, the two outer lower shoot sections that connect knuckles and foliage (the three inner ones run under the masses), and one bark fissure line.", "Beibehalten: Stamm mit Anlauf, Kopf, fünf Knollen, die zwei äußeren unteren Triebabschnitte, die Knollen und Laub verbinden (die drei inneren laufen unter den Massen), und eine Rindenrisslinie."),
        "merged": ("Merged into the foliage outlines: the upper shoot sections, leaf stalks and leaves; the palmate leaf lobes into the scallops of the mass outlines.", "In den Laubumrissen vereint: die oberen Triebabschnitte, Blattstiele und Blätter; die Lappen der Blätter in den Wellen der Massenumrisse."),
        "omitted": ("Omitted: bark patches, knuckle scars, twigs and leaf marks and all texture strokes.", "Weggelassen: Rindenschuppen, Knollennarben, Zweige und Blattzeichen sowie alle Texturstriche."),
        "thinElements": ("Shoots at 0.07 to 0.10 m are drawn at real width (0.35 to 0.5 mm) as tapered filled outlines that run into the knuckle and into the mass outline (joined, no gap); the upper shoots stay under the masses.", "Triebe von 0,07 bis 0,10 m werden in realer Breite (0,35 bis 0,5 mm) als verjüngte gefüllte Umrisse gezeichnet, die in Knolle und Massenumriss münden (verbunden, kein Spalt); die oberen Triebe bleiben unter den Massen."),
        "hatching": ("No hatching. One internal edge line (0.13 mm): the bark fissure of the stem.", "Keine Schraffur. Eine innere Kante (0,13 mm): der Rindenriss des Stamms."),
        "styles": ("Light line: outlines 0.18 mm, one 0.13 mm detail pen. Dark solid: sun and shade foliage in two tones, the head darker than the stem.", "Helle Linie: Umrisse 0,18 mm, ein Detailstift 0,13 mm. Dunkle Fläche: Sonnen- und Schattenlaub in zwei Tönen, der Kopf dunkler als der Stamm."),
        "overlays": ("Every thermal region keeps its own shapes: stem, head and knuckles (pollard-bark), shoots (shoot-bark) and the foliage masses; sway and loads act on the bone-bound shapes.", "Jede thermische Region behält ihre eigenen Formen: Stamm, Kopf und Knollen (pollard-bark), Triebe (shoot-bark) und die Laubmassen; Schwingung und Lasten wirken an den knochengebundenen Formen."),
    })
    #endregion 200

    #region 100
    sh = []
    sh.append(Shape("stem", trunk, "body", cr_closed(stem_pts(trunk, [(s - 0.45, w) for s, w in STEM100], [0.0, 0.005, -0.006, 0.003])), True, 0.35, 1, region="trunk-bark", material="plane-bark"))
    patches = [(0.9, 0.0, 0.16, 0.26, 10), (1.7, -0.1, 0.14, 0.3, -8), (2.3, 0.09, 0.15, 0.24, 14), (3.0, -0.06, 0.13, 0.28, -5), (3.55, 0.07, 0.12, 0.2, 8)]
    for i, (s, off, w, h, rt) in enumerate(patches):
        pts = [(s - 0.4 * h + 0.0, off - w), (s + 0.1 * h, off - 1.0 * w), (s + 0.5 * h, off - 0.3 * w), (s + 0.4 * h, off + 0.8 * w), (s - 0.1 * h, off + 1.0 * w), (s - 0.5 * h, off + 0.2 * w)]
        pts = [trunk.to_world((x - 0.45, y)) for x, y in pts]
        sh.append(Shape("bark-patch-%d" % (i + 1), trunk, "texture", cr_closed(pts), False, 0.15, 1.1))
    sh.append(tapered(S, head, "100", 6, [1.0, 1.1, 1.25, 1.22, 1.1, 1.0], 2, id="head"))
    for k in ["w2", "w1", "c", "e1", "e2"]:
        b = r.bones["k-" + k]
        sh.append(tapered(S, b, "100", 5, [0.95, 0.88, 0.95, 1.15, 1.35], 2.1, id="knuckle-" + k))
        a = r.bones["s-%sa" % k]
        sh.append(tapered(S, a, "100", 5, [1.0, 0.95, 1.0, 0.92, 0.95], 2.2, id="shoot-%s-a" % k))
        b2 = r.bones["s-%sb" % k]
        sh.append(tapered(S, b2, "100", 5, [1.0, 0.95, 0.9, 0.9, 0.95], 2.3, id="shoot-%s-b" % k))
    arcs = [[(0.62, 0.62, 200, 330, 0), (0.35, 0.4, 30, 150, 0)], [(0.6, 0.6, 20, 130, 0), (0.38, 0.4, 210, 320, 0)], [(0.6, 0.62, 250, 350, 0), (0.38, 0.4, 100, 200, 0)]]
    for i, m in enumerate(S.masses):
        hatch = "shade-lines" if m.kind == "shade" else None
        shp, _ = mass_shape(S, m, "100", lobe=0.85, amp=0.12, vary=[1.0, 0.65, 1.15, 0.8, 1.0, 0.9], z=3 + 0.4 * i, phase=0.5 * i, under=0.1, pointed=0.6, n=6, hatch=hatch, rx_s=0.9, ry_s=0.9)
        sh.append(shp)
        sh += mass_arcs(S, m, "100", arcs[i % 3], "detail", 3.2 + 0.4 * i)
    S.shapes["100"] = sh
    reps["100"] = rep_json(S, "100", sh, {
        "silhouette": ("At 1:100 (1 m is 10 mm) the masses of 1:200 are shrunk into separate leafy clusters with finer lobes so that the pollard framework shows through: stem with five bark patches, head, knuckles and both shoot sections run visibly into the foliage masses.", "Bei 1:100 (1 m sind 10 mm) werden die Massen von 1:200 zu getrennten Blattbüscheln mit feineren Lappen geschrumpft, so dass das Kopfbaumgerüst durchscheint: Stamm mit fünf Rindenschuppen, Kopf, Knollen und beide Triebabschnitte laufen sichtbar in die Laubmassen."),
        "retained": ("Retained: eight foliage clusters, stem, head, five knuckles with swollen fists, ten shoot sections (two per knuckle) and bark indication by five patches.", "Beibehalten: acht Laubbüschel, Stamm, Kopf, fünf Knollen mit verdickten Enden, zehn Triebabschnitte (zwei je Knolle) und Rindenandeutung durch fünf Schuppen."),
        "merged": ("Single leaves and twig ends are merged into lobed cluster outlines; the palmate leaf margins into the scallops; knuckle scars into the head outline.", "Einzelne Blätter und Zweigenden sind in gelappte Büschelumrisse eingeschmolzen; die Blattränder in die Wellen; Knollennarben in den Kopfumriss."),
        "omitted": ("Omitted: individual leaves, lateral twigs, knuckle scar rings and the fine flaking of the bark; the fine 1:50 leaf marks are replaced by two cluster arcs per mass.", "Weggelassen: einzelne Blätter, Seitenzweige, Narbenringe der Knollen und das feine Abblättern der Rinde; die feinen 1:50-Blattzeichen sind durch zwei Büschelbögen je Masse ersetzt."),
        "thinElements": ("Shoot sections are drawn at real width (0.3 to 0.7 mm) as filled tapered outlines; bark patches and cluster arcs are single strokes of 0.1 to 0.13 mm.", "Triebabschnitte werden in realer Breite (0,3 bis 0,7 mm) als gefüllte verjüngte Umrisse gezeichnet; Rindenschuppen und Büschelbögen sind einzelne Striche von 0,1 bis 0,13 mm."),
        "hatching": ("Restrained: only the shade core carries a thin 45 degree line hatch (0.6 mm spacing, above the profile minimum); no other hatching.", "Zurückhaltend: nur der Schattenkern trägt eine feine 45-Grad-Schraffur (0,6 mm Abstand, über dem Profilminimum); sonst keine Schraffur."),
        "styles": ("Light line: 0.2 mm outlines, 0.13 mm detail, 0.1 mm texture. Dark solid: tonal clusters, detail strokes stay visible on them.", "Helle Linie: Umrisse 0,2 mm, Detail 0,13 mm, Textur 0,1 mm. Dunkle Fläche: getönte Büschel, Detailstriche bleiben darauf sichtbar."),
        "overlays": ("Every region keeps its shapes (clusters, stem, head, shoots); the shade hatch is a pure drawing hatch and does not stand for temperature.", "Jede Region behält ihre Formen (Büschel, Stamm, Kopf, Triebe); die Schattenschraffur ist eine reine Zeichenschraffur und steht nicht für Temperatur."),
    })
    #endregion 100

    #region 50
    sh = []
    sh.append(Shape("stem", trunk, "body", cr_closed(stem_pts(trunk, [(s - 0.45, w) for s, w in STEM50], [0.0, 0.004, -0.006, 0.005, -0.003])), True, 0.33, 1, region="trunk-bark", material="plane-bark"))
    sh.append(stroke(S, "root-ridge-w", trunk, [trunk.to_world((-0.45 + 0.05, -0.38)), trunk.to_world((-0.45 + 0.2, -0.36)), trunk.to_world((-0.45 + 0.42, -0.325))], "detail", 1.1))
    sh.append(stroke(S, "root-ridge-e", trunk, [trunk.to_world((-0.45 + 0.06, 0.4)), trunk.to_world((-0.45 + 0.22, 0.37)), trunk.to_world((-0.45 + 0.44, 0.33))], "detail", 1.1))
    patches = [(0.85, 0.05, 0.12, 0.2, 0), (0.95, -0.17, 0.09, 0.16, 0), (1.35, 0.12, 0.1, 0.22, 0), (1.55, -0.12, 0.11, 0.2, 0), (1.95, 0.0, 0.09, 0.18, 0), (2.25, 0.15, 0.08, 0.15, 0), (2.5, -0.1, 0.1, 0.2, 0), (2.95, 0.1, 0.09, 0.17, 0), (3.15, -0.13, 0.08, 0.16, 0), (3.55, 0.05, 0.1, 0.2, 0)]
    for i, (s, off, w, h, rt) in enumerate(patches):
        pts = [(s - 0.45 - 0.4 * h, off - 0.9 * w), (s - 0.45, off - 1.1 * w), (s - 0.45 + 0.5 * h, off - 0.5 * w), (s - 0.45 + 0.45 * h, off + 0.4 * w), (s - 0.45 + 0.1 * h, off + 1.1 * w), (s - 0.45 - 0.35 * h, off + 0.8 * w), (s - 0.45 - 0.55 * h, off + 0.1 * w)]
        sh.append(Shape("bark-patch-%d" % (i + 1), trunk, "texture", cr_closed([trunk.to_world(p) for p in pts]), False, 0.15, 1.2))
    sh.append(tapered(S, head, "50", 8, [1.0, 1.06, 1.16, 1.22, 1.2, 1.12, 1.04, 1.0], 2, id="head"))
    for i, (s, off, w) in enumerate([(0.2, 0.08, 0.14), (0.45, -0.12, 0.1), (0.6, 0.1, 0.12)]):
        sh.append(stroke(S, "head-scar-%d" % (i + 1), head, wavy(head, s, s + 0.16, off, 0.03, 1.0, n=5), "texture", 2.05))
    for k in ["w2", "w1", "c", "e1", "e2"]:
        b = r.bones["k-" + k]
        sh.append(tapered(S, b, "50", 7, [0.95, 0.9, 0.88, 0.95, 1.1, 1.3, 1.4], 2.1, id="knuckle-" + k))
        ring = [b.to_world((0.8 * b.L, -0.11)), b.to_world((0.82 * b.L, 0.0)), b.to_world((0.8 * b.L, 0.11))]
        sh.append(stroke(S, "knuckle-scar-" + k, b, ring, "texture", 2.15))
        a = r.bones["s-%sa" % k]
        sh.append(tapered(S, a, "50", 6, [1.0, 0.96, 1.02, 0.95, 0.97, 0.94], 2.2, id="shoot-%s-a" % k))
        b2 = r.bones["s-%sb" % k]
        sh.append(tapered(S, b2, "50", 6, [1.0, 0.95, 0.9, 0.95, 0.9, 0.95], 2.3, id="shoot-%s-b" % k))
    arcs50 = [[(0.7, 0.7, 190, 330, 0), (0.5, 0.5, 20, 150, 0), (0.28, 0.3, 230, 340, 0)], [(0.7, 0.7, 10, 130, 0), (0.5, 0.52, 200, 320, 0), (0.28, 0.3, 60, 170, 0)], [(0.68, 0.7, 240, 350, 0), (0.48, 0.5, 90, 200, 0), (0.28, 0.3, 300, 380, 0)]]
    glyph_at = [(0.72, 200), (0.74, 330), (0.7, 100), (0.74, 20)]
    for i, m in enumerate(S.masses):
        hatch = "shade-lines" if m.kind == "shade" else None
        shp, _ = mass_shape(S, m, "50", lobe=0.5, amp=0.15, vary=[1.0, 0.6, 1.2, 0.8, 1.05, 0.7, 1.1], z=3 + 0.5 * i, phase=0.3 * i + 0.1, under=0.08, pointed=0.55, n=7, hatch=hatch, rx_s=0.9, ry_s=0.9)
        sh.append(shp)
        sh += mass_arcs(S, m, "50", arcs50[i % 3], "texture", 3.2 + 0.5 * i)
        if m.kind == "sun":
            for j, (fr, ang) in enumerate(glyph_at[: 2 + i % 3]):
                a = (ang + 25 * i) * D2R
                cx = m.c[0] + m.rx * 0.9 * fr * 0.9 * math.cos(a)
                cy = m.c[1] + m.ry * 0.9 * fr * 0.9 * math.sin(a)
                sh.append(Shape("%s-leaf%d" % (m.id, j + 1), r.bones[m.id], "detail", cr_closed(palmate(cx, cy, 0.11, 15 * (i + j) - 40)), False, 0.2, 3.3 + 0.5 * i))
    S.shapes["50"] = sh
    reps["50"] = rep_json(S, "50", sh, {
        "silhouette": ("At 1:50 (1 m is 20 mm) the pollard is drawn in full: mottled stem with root ridges and ten flaking patches, the knuckled head with scar marks, five knuckles with swollen fists and callus rings, ten tapered shoot sections and eight clustered foliage masses with fine lobes, leaf-cluster arcs and palmate leaf marks.", "Bei 1:50 (1 m sind 20 mm) wird der Kopfbaum vollständig gezeichnet: gescheckter Stamm mit Wurzelrippen und zehn abblätternden Schuppen, knorriger Kopf mit Narbenzeichen, fünf Knollen mit verdickten Enden und Kallusringen, zehn verjüngte Triebabschnitte und acht Laubbüschel mit feinen Lappen, Büschelbögen und gelappten Blattzeichen."),
        "retained": ("Retained: every woody member of the rig and every foliage mass; the leaf marks show only the three to four largest leaves per sun mass, drawn at the real leaf size of 0.2 m.", "Beibehalten: jedes Holzglied des Rigs und jede Laubmasse; die Blattzeichen zeigen nur die drei bis vier größten Blätter je Sonnenmasse, in der wahren Blattgröße von 0,2 m."),
        "merged": ("Only the plug overlaps at the bone joints close the seams between tapered woody parts; the flaking of the bark is merged into ten patches.", "Nur die Überlappungen an den Knochengelenken schließen die Nähte zwischen verjüngten Holzteilen; das Abblättern der Rinde ist in zehn Schuppen zusammengefasst."),
        "omitted": ("Omitted: the several thousand single leaves, stalks and buds (leaves are 4 mm on paper) and the third-order twigs.", "Weggelassen: die mehreren tausend einzelnen Blätter, Stiele und Knospen (Blätter sind 4 mm auf dem Papier) und die Zweige dritter Ordnung."),
        "thinElements": ("Shoot ends of 0.03 m (0.6 mm) stay filled; bark patches, scars and arcs are 0.13 mm strokes and the leaf marks 0.18 mm outlines, all above the pen minimum.", "Triebenden von 0,03 m (0,6 mm) bleiben gefüllt; Rindenschuppen, Narben und Bögen sind 0,13-mm-Striche und die Blattzeichen 0,18-mm-Umrisse, alle über dem Stiftminimum."),
        "hatching": ("A thin 45 degree line hatch (0.8 mm spacing) marks the shaded inner foliage; bark and leaves are drawn with strokes.", "Eine feine 45-Grad-Schraffur (0,8 mm Abstand) kennzeichnet das beschattete Innenlaub; Rinde und Blätter werden mit Strichen gezeichnet."),
        "styles": ("Light line: 0.25 mm outlines, 0.18 mm detail, 0.13 mm texture. Dark solid: tonal fills separate the pale mottled stem, the dark scarred head and the sun and shade foliage.", "Helle Linie: Umrisse 0,25 mm, Detail 0,18 mm, Textur 0,13 mm. Dunkle Fläche: getönte Füllungen trennen den hellen gescheckten Stamm, den dunklen vernarbten Kopf sowie Sonnen- und Schattenlaub."),
        "overlays": ("Each thermal region has its own shapes; bark patches, scars, arcs and leaf marks are strokes without a region and follow their bone.", "Jede thermische Region hat eigene Formen; Rindenschuppen, Narben, Bögen und Blattzeichen sind Striche ohne Region und folgen ihrem Knochen."),
    })
    #endregion 50

    bounds = world_bbox(S.shapes["50"])
    return reps, bounds


def make_symbols(S, bounds):
    root = S.rig.bones["root-plate"]
    W, D = 6.7, S.depth_m
    radii = [1.0, 0.97, 0.93, 0.98, 1.0, 0.95, 0.96, 1.0, 0.94, 0.98, 0.93, 0.97]
    crown = envelope(0, 0, W / 2, D / 2, radii, k=13, amp=0.045, phase=0.2, n=4)
    stem = ring_pts(0, 0, 0.31, n=12)
    knuckles = []
    for i in range(5):
        a = (90 + 72 * i) * D2R
        knuckles.append(Shape("knuckle-%d" % (i + 1), root, "detail", cr_closed(ring_pts(0.52 * math.cos(a), 0.52 * math.sin(a), 0.13, n=8)), True, 0.3, 3))
    plan = [Shape("crown-extent", root, "body", cr_closed(crown), False, 0.5, 1), Shape("head-ring", root, "detail", cr_closed(ring_pts(0, 0, 0.45, n=12)), False, 0.3, 2), Shape("stem", root, "body", cr_closed(stem), True, 0.3, 4)] + knuckles
    nominal = [Shape("sign-circle", root, "body", cr_closed(ring_pts(0, 0, 1.6, n=12)), False, 0.5, 1), Shape("sign-dot", root, "body", cr_closed(ring_pts(0, 0, 0.35, n=8)), True, 0.3, 2)]
    for s_ in plan + nominal:
        s_.frame = IDENT
    return [
        {"id": "plan-crown", "title": T("Plan symbol with true crown extent", "Grundrisssymbol mit wahrer Kronenausdehnung"), "description": T("A conventional plan sign for site and landscape plans: the true plan outline of the crown, the pollard head ring with its five knuckles and the stem section. It is a separate symbol and is never used silently for a pictorial scale.", "Ein konventionelles Grundrisszeichen für Lage- und Freiraumpläne: der wahre Grundrissumriss der Krone, der Kopfring mit seinen fünf Knollen und der Stammquerschnitt. Es ist ein eigenes Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet."),
         "semantics": "true-extent", "units": "model-m", "denominators": [500, 200, 100], "pens": PENS["100"], "hatches": [], "geometry": {"elevation": [], "plan": [s.json() for s in plan]},
         "dimensionalNote": T("The outer outline is the true plan extent of the crown in model metres (%.1f m by %.1f m); the inner ring is the pollard head (0.9 m), the small circles the five knuckles and the filled circle the true stem section of 0.62 m. At 1:500 the stem measures 1.2 mm on paper." % (W, D), "Der äußere Umriss ist die wahre Kronenausdehnung im Grundriss in Modellmetern (%.1f m mal %.1f m); der innere Ring ist der Kopf (0,9 m), die kleinen Kreise sind die fünf Knollen und der gefüllte Kreis der wahre Stammquerschnitt von 0,62 m. Bei 1:500 misst der Stamm 1,2 mm auf dem Papier." % (W, D))},
        {"id": "tree-sign", "title": T("Nominal tree sign with trunk centre", "Nominelles Baumzeichen mit Stammmitte"), "description": T("The conventional circle-and-dot tree sign of small-scale site plans: the dot marks the trunk centre, the circle is a fixed paper size.", "Das übliche Kreis-und-Punkt-Baumzeichen kleinmaßstäblicher Lagepläne: Der Punkt markiert die Stammmitte, der Kreis hat eine feste Papiergröße."),
         "semantics": "nominal", "units": "paper-mm", "denominators": [500, 200], "pens": PENS["500"], "hatches": [], "geometry": {"elevation": [], "plan": [{**s.json(), "d": s.json()["d"]} for s in nominal]},
         "dimensionalNote": T("A fixed 3.2 mm circle with a 0.7 mm centre dot in paper millimetres; its size carries no dimension. Use the true-extent plan symbol where the crown width matters.", "Ein fester Kreis von 3,2 mm mit 0,7 mm Mittelpunkt in Papiermillimetern; seine Größe trägt kein Maß. Wo die Kronenbreite zählt, ist das Grundrisssymbol mit wahrer Ausdehnung zu verwenden.")},
    ]
