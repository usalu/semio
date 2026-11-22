"""🌲️ European larch (Larix decidua): every number of the asset `european-larch`, chosen by hand after an open-grown tree of about 30 m with
an open, irregular conical crown of eleven thin, widely spaced branch storeys through which the stem shows, horizontal branches that turn up
at their ends with long pendulous branchlets, soft light-green needles in tufts, and a thick, deeply fissured grey-brown bark. Deciduous: the
poses are calm, fresh breeze, gale and golden autumn. Run: `repo tools exec python -- european_larch.py`.
@see ./whorled.py
"""
import math
import sys

from conifer_lib import T
import whorled

id = "european-larch"
H = 30.0
stem_joints = [0.0, 0.7, 8.6, 15.4, 21.2, 26.0, H]
stem_names = ["trunk-1", "trunk-2", "trunk-3", "trunk-4", "leader"]
stem_titles = [T("Stem, lower (bare, fissured bark)", "Stamm unten (kahl, rissige Borke)"), T("Stem, lower crown", "Stamm, untere Krone"), T("Stem, middle crown", "Stamm, mittlere Krone"),
               T("Stem, upper crown", "Stamm, obere Krone"), T("Leader", "Gipfeltrieb")]
stem_limits = [2, 2.5, 3, 4, 6]
whorl_limit = 12
spray_limit = 18
crown_base = 4.4
sun_from = 15.0
wood = "larch-wood"
bark_lower, bark_upper = "larch-bark", "larch-bark-young"
needle_sun, needle_shade = "larch-needles-sun", "larch-needles-shade"
green_density = 870.0
E_green = 10.1e9
k_root = 40e6
root_plate_extra_kg = 0.0
footprint = 2.6
needle_area_sun, needle_area_shade = 110.0, 70.0
needle_kg_m2 = 0.2
needle_width = 0.001
gs_sun, gs_shade = 0.004, 0.0028
branches_per_whorl = 5
twig_factor = 1.5
cd = 0.32
v_calm, v_windy, v_storm = 2.0, 8.0, 22.0
whorl_lean, spray_lean = 6.0, 8.0
sway_period = 3.9
root_azimuths = [35, 110, 185, 250, 320]
root_len = [1.2, 0.95, 1.1, 0.85, 1.0]
root_r = [0.16, 0.13, 0.15, 0.12, 0.14]
bark_heat = (75000, 40000)
bark_upper_sky = 0.3
emitter_d = {"transpiration-sun": 2.4, "transpiration-shade": 2.4}
sun = {"bark-lower": 0.45, "bark-upper": 0.25, "needles-sun": 0.9, "needles-shade": 0.4}
transpiration_day = [(0, 0.03), (5, 0.05), (7, 0.35), (9, 0.85), (12, 1.0), (14, 0.95), (16, 0.75), (18, 0.3), (20, 0.06), (24, 0.03)]
environment = {"airTempC": 22, "meanRadiantTempC": 25, "relativeHumidity": 0.55, "windMS": 2, "windDirectionDeg": 0, "solarWM2": 700, "pressurePa": 101325, "diffuseFraction": 0.15}
words = {"name": ("European larch", "Europäischen Lärche"), "wood": ("larch", "Lärchenholz"), "midday": ("a broad midday plateau of the light-demanding, high-conductance needles", "ein breites Mittagsplateau der lichtbedürftigen, gut leitenden Nadeln")}
N_TIERS = 11
RF = [1.0, 0.9, 1.08, 0.94, 1.1, 0.92, 1.06, 0.95, 1.04, 0.97, 1.0]
LS = [0.9, 1.12, 0.88, 1.1, 0.92, 1.14, 0.9, 1.08, 0.93, 1.05, 1.0]
DX = [0.15, -0.18, 0.2, -0.12, 0.16, -0.2, 0.1, -0.12, 0.08, -0.06, 0.0]
DZ = [-0.12, 0.16, -0.08, 0.2, -0.15, 0.1, -0.12, 0.08, -0.06, 0.04, 0.0]
JIT = [28, -20, 34, -30, 18, -34, 26, -16, 30, -24, 0]
OFF = [22, -28, 16, -20, 30, -14, 24, -26, 12, -18, 10]


def r_stem(y):
    if y >= H:
        return 0.0
    main = 0.31 * ((H - y) / (H - 1.3)) ** 0.82 + 0.33 * math.exp(-y / 0.3)
    if y > 25.0:
        main = max(main, 0.024 * min(1.0, (H - y) / 0.9))
    return main


def tiers():
    out = []
    for k in range(N_TIERS):
        x = k / (N_TIERS - 1)
        h = 6.1 + 22.4 * (1 - (1 - x) ** 1.1)
        R = (0.5 + 3.6 * ((29.2 - h) / (29.2 - 6.1)) ** 0.85) * RF[k]
        D = 0.18 + 0.1 * R
        top = 0.32 + 0.06 * R
        rim = 0.16 + 0.04 * R
        out.append((h, R, D, top, rim, 0.25, LS[k], DX[k], DZ[k], JIT[k]))
    return out


def spray_droop(t):
    return math.degrees(math.atan2(t.D + 0.5 * t.rim, 1.07 * t.R)) * 0.6


def branch_d(t):
    return 0.03 + 0.02 * t.R


def spray_r(t):
    return 0.08 + 0.035 * t.R, 0.04 + 0.01 * t.R


def spray_tip_drop(t):
    return 0.05 + 0.03 * t.R


def spray_tip(t, p1, a):
    """Larix decidua: the long branch end sweeps up."""
    if t.R < 0.8:
        return None
    L = 0.2 * t.R
    return (p1[0] + L * math.cos(a), p1[1] + 0.15 + 0.1 * t.R, p1[2] + L * math.sin(a) * t.ls), 0.85 * spray_r(t)[1], 0.05


def hanging_sides(t, scale):
    """The diagonal shoots whose long pendulous branchlets hang below the thin tier."""
    if t.R < 0.9:
        return []
    return ["ne", "nw", "sw", "se"] if scale == "50" else ["ne", "sw"]


def hang_drop(t):
    return 0.45 + 0.22 * t.R


def branchlets(t, p0, p1, a):
    return []


def lateral_offset(t):
    return OFF[t.k - 1]


def tone(t):
    return 0.45 if t.region == "needles-sun" else 0.55


def mark_len(t):
    return 0.18 + 0.05 * t.R


def sway_amp(bid, base):
    if bid.startswith(("whorl", "spray")):
        return 0.5 * abs(base) + 0.6
    return 0.45 * abs(base)


def extra_poses(tiers, binding, loads, fmt):
    autumn_sun = dict(sun)
    return [{"id": "autumn", "title": T("Golden autumn", "Goldener Herbst"),
             "description": T("Late October before needle fall: the needles have turned golden yellow and their stomata are closing, so the transpiration falls to 15 % of the summer value; calm, low sun. The geometry is that of the summer crown (the needles are still on the tree); the winter-bare crown is not drawn.",
                              "Ende Oktober vor dem Nadelfall: Die Nadeln sind goldgelb geworden und ihre Spaltöffnungen schließen sich, so dass die Transpiration auf 15 % des Sommerwerts fällt; windstill, tiefe Sonne. Die Geometrie ist die der Sommerkrone (die Nadeln hängen noch am Baum); die winterkahle Krone wird nicht gezeichnet."),
             "bones": {}, "ground": "authored", "contacts": [],
             "thermal": {"regions": binding({k: 0.6 * v for k, v in autumn_sun.items()}), "emitters": {"transpiration-sun": 0.15, "transpiration-shade": 0.15}}, "loads": loads(v_calm)}]


title = T("European Larch", "Europäische Lärche")
summary = T("A mature, open-grown European larch (Larix decidua) of 30 m: a straight stem with thick, deeply fissured grey-brown bark, bare to about 4.5 m, under an open, irregular cone of eleven thin branch storeys through which the stem shows; the horizontal branches sweep up at their ends and let long branchlets hang, clothed in soft, light-green needles that turn gold and fall in autumn.",
            "Eine ausgewachsene, frei stehende Europäische Lärche (Larix decidua) von 30 m: ein gerader Stamm mit dicker, tief rissiger graubrauner Borke, bis etwa 4,5 m kahl, unter einem lichten, unregelmäßigen Kegel aus elf dünnen Astetagen, durch die der Stamm scheint; die waagerechten Äste schwingen an den Enden auf und lassen lange Zweige hängen, besetzt mit weichen, hellgrünen Nadeln, die sich im Herbst golden färben und fallen.")
tags = ["tree", "deciduous", "conifer", "open-crown", "wind", "autumn", "transpiration"]
distinction = T("The only deciduous conifer of the collection: an open, airy cone of eleven thin, widely spaced storeys with the stem showing between them, pendulous branchlets and 180 m² of soft needles with the highest stomatal conductance of the conifers.",
                "Der einzige sommergrüne Nadelbaum der Sammlung: ein lichter, luftiger Kegel aus elf dünnen, weit gestellten Etagen mit dem Stamm dazwischen, hängenden Zweigen und 180 m² weichen Nadeln mit der höchsten Spaltöffnungsleitfähigkeit der Nadelbäume.")
notes = T("Authored by hand after a 100-year-old open-grown larch: clear stem to about 4.5 m, 0.62 m thick at breast height. Masses are the volume of every drawn wooden solid of the 1:50 model times green density (Larix decidua, Wood Handbook western larch proxy) plus needle and branch estimates per storey; the root system below ground is not part of the asset. The winter-bare state cannot be drawn with the same shapes; the autumn pose keeps the summer geometry. The 2D views of every scale are the exact orthographic projections of that scale's 3D body.",
          "Von Hand nach einer 100-jährigen, frei stehenden Lärche erstellt: astfreier Stamm bis etwa 4,5 m, in Brusthöhe 0,62 m dick. Massen sind das Volumen jedes gezeichneten Holzkörpers des 1:50-Modells mal Frischholzdichte (Larix decidua, Proxy Westamerikanische Lärche des Wood Handbook) plus Nadel- und Astschätzungen je Etage; das Wurzelsystem unter dem Boden gehört nicht zum Asset. Der winterkahle Zustand lässt sich mit denselben Formen nicht zeichnen; die Herbstpose behält die Sommergeometrie. Die 2D-Ansichten jedes Maßstabs sind die exakten Parallelprojektionen des 3D-Körpers dieses Maßstabs.")
references = [
    {"id": "wood-handbook-2021", "citation": "Forest Products Laboratory (2021). Wood Handbook: Wood as an Engineering Material. General Technical Report FPL-GTR-282, USDA Forest Service. Chapter 5 (green density, modulus of elasticity and rupture; western larch as proxy for Larix decidua)."},
    {"id": "en-1991-1-4", "citation": "EN 1991-1-4:2005 Eurocode 1, Actions on structures, Part 1-4: Wind actions. Air density 1.25 kg/m3, F = 0.5 rho cd A v^2, peak velocity pressure (1 + 7 Iv) q_m."},
    {"id": "mayhead-1973", "citation": "Mayhead, G. J. (1973). Some drag coefficients for British forest trees derived from wind tunnel studies. Agricultural Meteorology 12, 123-130."},
    {"id": "moore-maguire-2004", "citation": "Moore, J. R. and Maguire, D. A. (2004). Natural sway frequencies and damping ratios of trees: concepts, review and synthesis of previous studies. Trees 18, 195-203."},
    {"id": "jones-2014", "citation": "Jones, H. G. (2014). Plants and Microclimate, 3rd ed., Cambridge University Press. Leaf energy balance, leaf area index, boundary-layer conductance."},
    {"id": "monteith-unsworth-2013", "citation": "Monteith, J. L. and Unsworth, M. H. (2013). Principles of Environmental Physics, 4th ed., Academic Press. Penman-Monteith leaf transpiration."},
    {"id": "farjon-2010", "citation": "Farjon, A. (2010). A Handbook of the World's Conifers. Brill, Leiden. Larix decidua: habit, height, crown form, branching and deciduous needles."},
    {"id": "da-ronch-2016-larix", "citation": "Da Ronch, F., Caudullo, G., Tinner, W. and de Rigo, D. (2016). Larix decidua and other larches in Europe: distribution, habitat, usage and threats. In: San-Miguel-Ayanz, J. et al. (eds.), European Atlas of Forest Tree Species. Publications Office of the European Union, Luxembourg."},
]
materials = [
    {"id": "larch-bark", "title": T("Thick, deeply fissured larch bark", "Dicke, tief rissige Lärchenborke"), "emissivity": 0.95, "solarAbsorptivity": 0.8, "densityKgM3": 500, "conductivityWmK": 0.12, "source": "authored-estimate", "reference": "jones-2014"},
    {"id": "larch-bark-young", "title": T("Thinner scaly bark of the upper stem", "Dünnere schuppige Rinde des oberen Stamms"), "emissivity": 0.94, "solarAbsorptivity": 0.72, "densityKgM3": 460, "conductivityWmK": 0.11, "source": "authored-estimate", "reference": "jones-2014"},
    {"id": "larch-needles-sun", "title": T("Soft larch needles in tufts, sunlit storeys", "Weiche Lärchennadeln in Büscheln, besonnte Etagen"), "emissivity": 0.97, "solarAbsorptivity": 0.5, "source": "literature", "reference": "jones-2014"},
    {"id": "larch-needles-shade", "title": T("Soft larch needles in tufts, lower storeys", "Weiche Lärchennadeln in Büscheln, untere Etagen"), "emissivity": 0.97, "solarAbsorptivity": 0.38, "source": "authored-estimate", "reference": "jones-2014"},
    {"id": "larch-wood", "title": T("Green larch wood", "Frisches Lärchenholz"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 870, "conductivityWmK": 0.15, "elasticModulusPa": 1.01e10, "strengthPa": 5.3e7, "source": "literature", "reference": "wood-handbook-2021"},
]
region_titles = {"bark-lower": T("Lower stem and root flare, fissured bark", "Unterer Stamm und Wurzelanlauf, rissige Borke"), "bark-upper": T("Stem between the storeys and leader", "Stamm zwischen den Etagen und Gipfeltrieb"),
                 "needles-sun": T("Needles of the sunlit upper storeys", "Nadeln der besonnten oberen Etagen"), "needles-shade": T("Needles of the lower storeys", "Nadeln der unteren Etagen")}
emitter_titles = {"transpiration-sun": T("Transpiration of the upper needles (distributed over the upper crown)", "Transpiration der oberen Nadeln (über die obere Krone verteilt)"),
                  "transpiration-shade": T("Transpiration of the lower needles (distributed over the lower crown)", "Transpiration der unteren Nadeln (über die untere Krone verteilt)")}
thermal_assumptions = [
    T("Needle regions use the needle exchange area; the leaf area index is that area over the crown ground projection. The maximum stomatal conductance (4.0 mm/s sunlit, 2.8 mm/s lower) is an authored estimate for a deciduous, light-demanding conifer guided by Jones (2014), above the evergreen conifers; photosynthesis type C3. Aperture and night closure are the engine's business, not authored here; autumn senescence is the reduced emitter strength of the autumn pose.",
      "Nadelregionen verwenden die Austauschfläche der Nadeln; der Blattflächenindex ist diese Fläche bezogen auf die Kronenprojektion am Boden. Die maximale Spaltöffnungsleitfähigkeit (4,0 mm/s besonnt, 2,8 mm/s unten) ist eine Autorenschätzung für einen sommergrünen, lichtbedürftigen Nadelbaum, angelehnt an Jones (2014), über den immergrünen Nadelbäumen; Photosynthesetyp C3. Öffnung und nächtliches Schließen sind Sache der Engine; die herbstliche Seneszenz ist die verringerte Emitterstärke der Herbstpose."),
    T("Bark is passive. Its areal heat capacity is the volumetric heat capacity of green wood (about 2.7 MJ per m3 and K) times an effective depth of 1.5 to 3 cm (thick larch bark), an authored estimate guided by the Wood Handbook; the bark areas are the lateral areas of the 1:50 stem, flare and root solids. The upper stem is partly sunlit through the open crown.",
      "Die Rinde ist passiv. Ihre flächenbezogene Wärmekapazität ist die volumetrische Wärmekapazität von frischem Holz (etwa 2,7 MJ je m3 und K) mal einer wirksamen Tiefe von 1,5 bis 3 cm (dicke Lärchenborke), eine Autorenschätzung angelehnt an das Wood Handbook; die Rindenflächen sind die Mantelflächen der 1:50-Körper von Stamm, Anlauf und Wurzeln. Der obere Stamm ist durch die lichte Krone teilweise besonnt."),
    T("Transpiration emitters release the latent flux of their needle region as vapour, spread over six anchors across the upper or the lower crown; the direction is the vertical of the whorl bone of the first anchor. The upper needles are the storeys from 15 m upwards.",
      "Transpirationsemitter geben den latenten Strom ihrer Nadelregion als Dampf ab, verteilt auf sechs Anker über die obere oder untere Krone; die Richtung ist die Senkrechte des Quirlknochens des ersten Ankers. Die oberen Nadeln sind die Etagen ab 15 m."),
]
structure_assumptions = [
    T("One rigid body per evaluated pose, fixed at the root plate. Wind drag acts at the centroid of the silhouette band of every storey (cd 0.32 for the open crown, reduced above 8 m/s by reconfiguration) and as a line load on the bare lower stem. Sections are solid circles of green larch wood; the stem lean of the wind poses is the deflection of a tapered cantilever (E 10.1 GPa) on a heart-root plate of 40 MN m per radian. Dynamic amplification and root-soil failure are not modelled.",
      "Ein starrer Körper je ausgewerteter Pose, am Wurzelteller eingespannt. Winddruck wirkt im Schwerpunkt des Silhouettenbands jeder Etage (cd 0,32 für die lichte Krone, über 8 m/s durch Formanpassung verringert) und als Linienlast am kahlen unteren Stamm. Querschnitte sind Vollkreise aus frischem Lärchenholz; die Stammneigung der Windposen ist die Durchbiegung eines verjüngten Kragarms (E 10,1 GPa) auf einem Herzwurzelteller von 40 MNm je Radiant. Dynamische Überhöhung und Wurzel-Boden-Versagen werden nicht modelliert."),
]


def rep_decisions(scale, n, dims, fmt):
    h, w = dims["heightM"], dims["widthM"]
    if scale == "500":
        return {
            "silhouette": ("At 1:500 (1 m is 2 mm, the tree %s mm high) the larch is one open, spiky cone on a tall bare stem: the crown path is the true side view of the 1:500 crown solid, whose envelope steps in to the stem between the %d storeys, so the airy layering still reads." % (fmt(h * 2, False, 0), n),
                           "Bei 1:500 (1 m sind 2 mm, der Baum %s mm hoch) ist die Lärche ein lichter, zackiger Kegel auf hohem kahlem Stamm: der Kronenpfad ist die wahre Seitenansicht des 1:500-Kronenkörpers, dessen Hülle zwischen den %d Etagen bis an den Stamm einspringt, so dass die luftige Schichtung lesbar bleibt." % (fmt(h * 2, True, 0), n)),
            "retained": ("Retained: the bare stem to 4.5 m, the notches between the storeys, the upswept spire and the width of %s m." % fmt(w, False, 2), "Beibehalten: der kahle Stamm bis 4,5 m, die Einschnitte zwischen den Etagen, die aufstrebende Spitze und die Breite von %s m." % fmt(w, True, 2)),
            "merged": ("Merged: storeys, branch ends and pendulous branchlets into two crown solids (upper from 14.7 m, lower) drawn as one crown path; the stem inside the crown into the crown.", "Vereinigt: Etagen, Astenden und hängende Zweige zu zwei Kronenkörpern (oben ab 14,7 m, unten), gezeichnet als ein Kronenpfad; der Stamm in der Krone in die Krone."),
            "omitted": ("Omitted: single branches, the surface roots, bark fissures and needle tufts.", "Weggelassen: einzelne Äste, die Wurzelanläufe, Borkenrisse und Nadelbüschel."),
            "thinElements": ("The thin stem between the storeys is kept inside the crown path, never drawn thinner than the stem.", "Der dünne Stamm zwischen den Etagen bleibt im Kronenpfad und wird nie dünner als der Stamm gezeichnet."),
            "hatching": ("No hatching; outlines 0.15 mm, ground line 0.25 mm.", "Keine Schraffur; Umrisse 0,15 mm, Bodenlinie 0,25 mm."),
            "styles": ("Light line: two outlines. Dark solid: a light crown (the needles are light green), the lower part a little darker.", "Helle Linie: zwei Umrisse. Dunkle Fläche: eine helle Krone (die Nadeln sind hellgrün), der untere Teil etwas dunkler."),
            "overlays": ("Thermal: bark-lower on the stem, needles-sun with the hidden bark-upper in the upper crown, needles-shade below. Structure: stem, spray members and root support.", "Thermisch: bark-lower am Stamm, needles-sun mit der verdeckten bark-upper in der oberen Krone, needles-shade darunter. Tragwerk: Stamm, Zweigglieder und Wurzelauflager."),
        }
    if scale == "200":
        return {
            "silhouette": ("At 1:200 (1 m is 5 mm) the %d storeys are thin, flat skirts with sky and stem between them, each the true view of its 1:200 skirt solid; the open, irregular cone is built of their offsets and different widths." % n,
                           "Bei 1:200 (1 m sind 5 mm) sind die %d Etagen dünne, flache Schürzen mit Himmel und Stamm dazwischen, jede die wahre Ansicht ihres 1:200-Schürzenkörpers; der lichte, unregelmäßige Kegel entsteht aus ihren Versätzen und Breiten." % n),
            "retained": ("Retained: every storey with its own width, offset and ellipticity, the visible stem in five parts and the leader.", "Beibehalten: jede Etage mit eigener Breite, eigenem Versatz und eigener Elliptizität, der sichtbare Stamm in fünf Teilen und der Gipfeltrieb."),
            "merged": ("Merged: the upswept ends and the pendulous branchlets into each skirt (fitted to the plan box of the 1:50 storey, 10 cm deeper); the root flare into the lower stem.", "Vereinigt: aufschwingende Enden und hängende Zweige in jede Schürze (an den Grundrisskasten der 1:50-Etage angepasst, 10 cm tiefer); der Wurzelanlauf in den unteren Stamm."),
            "omitted": ("Omitted: surface roots, bark and needle texture.", "Weggelassen: Wurzelanläufe, Borken- und Nadeltextur."),
            "thinElements": ("The leader keeps its true taper; nothing is thickened.", "Der Gipfeltrieb behält seine wahre Verjüngung; nichts wird verdickt."),
            "hatching": ("No hatching; outlines 0.18 mm.", "Keine Schraffur; Umrisse 0,18 mm."),
            "styles": ("Light line: thin skirts layered on the stem. Dark solid: light storeys over a darker stem.", "Helle Linie: dünne Schürzen am Stamm geschichtet. Dunkle Fläche: helle Etagen über einem dunkleren Stamm."),
            "overlays": ("Thermal: each skirt in its needle region, the stem in bark-lower and bark-upper. Structure: stem segments and spray members.", "Thermisch: jede Schürze in ihrer Nadelregion, der Stamm in bark-lower und bark-upper. Tragwerk: Stammabschnitte und Zweigglieder."),
        }
    if scale == "100":
        return {
            "silhouette": ("At 1:100 (1 m is 10 mm) each of the %d storeys is a thin skirt with its west and east branches reaching beyond it and two pendulous shoots hanging below, so every storey ends in ragged, drooping fringes; side and front differ because each storey is elliptic and off-centre." % n,
                           "Bei 1:100 (1 m sind 10 mm) ist jede der %d Etagen eine dünne Schürze, deren West- und Ostast über sie hinausreichen und unter der zwei hängende Triebe herabfallen, so dass jede Etage in zerzausten, hängenden Fransen endet; Seiten- und Vorderansicht unterscheiden sich, weil jede Etage elliptisch und außermittig ist." % n),
            "retained": ("Retained: skirts, two branches in the rig plane and two hanging shoots per storey, root flare, stem segments, leader, and short strokes of needle tufts.", "Beibehalten: Schürzen, je Etage zwei Äste in der Rig-Ebene und zwei hängende Triebe, Wurzelanlauf, Stammabschnitte, Gipfeltrieb und kurze Striche der Nadelbüschel."),
            "merged": ("Merged: the upswept branch ends into the branches; the lateral branches into the skirts.", "Vereinigt: die aufschwingenden Astenden in die Äste; die seitlichen Äste in die Schürzen."),
            "omitted": ("Omitted: surface roots, bark fissures and cones.", "Weggelassen: Wurzelanläufe, Borkenrisse und Zapfen."),
            "thinElements": ("Branches keep their true section; strokes stay 0.5 mm inside their storey and away from every other outline.", "Äste behalten ihren wahren Querschnitt; Striche bleiben 0,5 mm innerhalb ihrer Etage und fern von jedem anderen Umriss."),
            "hatching": ("No hatch pattern; texture strokes of 0.1 mm.", "Kein Schraffurmuster; Texturstriche von 0,1 mm."),
            "styles": ("Light line: outlines and tuft strokes. Dark solid: light storeys, the branches slightly darker.", "Helle Linie: Umrisse und Büschelstriche. Dunkle Fläche: helle Etagen, die Äste etwas dunkler."),
            "overlays": ("Thermal: skirts, branches and shoots in their needle region, the stem in its bark regions. Structure: stem and spray members.", "Thermisch: Schürzen, Äste und Triebe in ihrer Nadelregion, der Stamm in seinen Rindenregionen. Tragwerk: Stamm- und Zweigglieder."),
        }
    return {
        "silhouette": ("At 1:50 (1 m is 20 mm) every storey is the true view of its thin skirt, four branches (west and east on their own bones, north and south on the whorl) with upswept ends and four pendulous shoots that hang far below; the stem shows between the storeys and stands on its flare with five surface roots.",
                       "Bei 1:50 (1 m sind 20 mm) ist jede Etage die wahre Ansicht ihrer dünnen Schürze, von vier Ästen (West und Ost an eigenen Knochen, Nord und Süd am Quirl) mit aufschwingenden Enden und vier weit herabhängenden Trieben; der Stamm zeigt sich zwischen den Etagen und steht auf seinem Anlauf mit fünf Wurzelanläufen."),
        "retained": ("Retained: everything modelled at 1:50, plus tuft strokes in every storey, bark fissure strokes on the bare stem and radial needle strokes in the plan rings.", "Beibehalten: alles bei 1:50 Modellierte, dazu Büschelstriche in jeder Etage, Borkenrissstriche am kahlen Stamm und radiale Nadelstriche in den Grundrissringen."),
        "merged": ("Merged: single needle tufts and short shoots into skirts and branches.", "Vereinigt: einzelne Nadelbüschel und Kurztriebe in Schürzen und Äste."),
        "omitted": ("Omitted: the small upright cones (about 3 cm, 0.6 mm at 1:50, lost in the tufts).", "Weggelassen: die kleinen aufrechten Zapfen (etwa 3 cm, 0,6 mm bei 1:50, in den Büscheln verloren)."),
        "thinElements": ("Upswept ends (r 5 cm) and pendulous shoots keep their true size; texture strokes keep 0.5 mm from every outline.", "Aufschwingende Enden (r 5 cm) und hängende Triebe behalten ihre wahre Größe; Texturstriche halten 0,5 mm Abstand zu jedem Umriss."),
        "hatching": ("No hatch pattern; texture strokes of 0.13 mm.", "Kein Schraffurmuster; Texturstriche von 0,13 mm."),
        "styles": ("Light line: outlines, tuft and bark strokes. Dark solid: light storeys over the dark fissured stem.", "Helle Linie: Umrisse, Büschel- und Borkenstriche. Dunkle Fläche: helle Etagen über dem dunklen, rissigen Stamm."),
        "overlays": ("Thermal: every needle part in its region, flare, roots and lower stem bark-lower, upper stem and leader bark-upper. Structure: stem, root plate and every spray member, the support under the root plate.", "Thermisch: jedes Nadelteil in seiner Region, Anlauf, Wurzeln und unterer Stamm bark-lower, oberer Stamm und Gipfeltrieb bark-upper. Tragwerk: Stamm, Wurzelteller und jedes Zweigglied, das Auflager unter dem Wurzelteller."),
    }


def volume_decisions(scale, nbody, n, fmt):
    body = {"500": ("Body at 1:500 (smallest feature 75 mm): the stem with its flare up into the crown and the crown as two solids of revolution whose stepped envelope is the union of the 1:50 crown, never thinner than the stem; %d solids." % nbody,
                    "Körper bei 1:500 (kleinstes Merkmal 75 mm): der Stamm mit Anlauf bis in die Krone und die Krone als zwei Drehkörper, deren gestufte Hülle die Vereinigung der 1:50-Krone ist und nie dünner als der Stamm; %d Körper." % nbody),
            "200": ("Body at 1:200 (30 mm): five stem solids and %d thin skirts fitted to the plan box of their 1:50 storey; %d solids." % (n, nbody), "Körper bei 1:200 (30 mm): fünf Stammkörper und %d dünne Schürzen, an den Grundrisskasten ihrer 1:50-Etage angepasst; %d Körper." % (n, nbody)),
            "100": ("Body at 1:100 (15 mm): flare, stem segments, leader, %d skirts, two branch capsules and two hanging shoots per storey; %d solids." % (n, nbody), "Körper bei 1:100 (15 mm): Anlauf, Stammabschnitte, Gipfeltrieb, %d Schürzen, zwei Astkapseln und zwei hängende Triebe je Etage; %d Körper." % (n, nbody)),
            "50": ("Body at 1:50 (7.5 mm): flare with five surface roots, stem segments and leader, %d thin skirts, four branches with upswept ends and four pendulous shoots per storey; %d solids." % (n, nbody),
                   "Körper bei 1:50 (7,5 mm): Anlauf mit fünf Wurzelanläufen, Stammabschnitte und Gipfeltrieb, %d dünne Schürzen, vier Äste mit aufschwingenden Enden und vier hängende Triebe je Etage; %d Körper." % (n, nbody))}[scale]
    th = {"500": ("Thermal at 1:500: stem and the two crown solids by reference; the bark of the upper stem is a capsule inside the crown; twelve transpiration outlets.", "Thermisch bei 1:500: Stamm und die zwei Kronenkörper per Verweis; die Rinde des oberen Stamms ist eine Kapsel in der Krone; zwölf Transpirationsauslässe."),
          "200": ("Thermal at 1:200: every body solid by reference; twelve outlets.", "Thermisch bei 1:200: jeder Körper per Verweis; zwölf Auslässe."),
          "100": ("Thermal at 1:100: every body solid by reference; twelve outlets.", "Thermisch bei 1:100: jeder Körper per Verweis; zwölf Auslässe."),
          "50": ("Thermal at 1:50: every body solid by reference, so the bark areas are the lateral areas of these solids; twelve outlets.", "Thermisch bei 1:50: jeder Körper per Verweis, so dass die Rindenflächen die Mantelflächen dieser Körper sind; zwölf Auslässe.")}[scale]
    st = {"500": ("Structure at 1:500: root support disc, root-plate and stem members at 56 % of the stem radius, slender spray members inside the crown.", "Tragwerk bei 1:500: Wurzelauflagerscheibe, Wurzelteller- und Stammglieder mit 56 % des Stammradius, schlanke Zweigglieder in der Krone."),
          "200": ("Structure at 1:200: support disc, stem members at 58 % in four stations, slender spray members.", "Tragwerk bei 1:200: Auflagerscheibe, Stammglieder mit 58 % in vier Stationen, schlanke Zweigglieder."),
          "100": ("Structure at 1:100: support disc, stem members at 60 % in five stations, tapered spray members.", "Tragwerk bei 1:100: Auflagerscheibe, Stammglieder mit 60 % in fünf Stationen, verjüngte Zweigglieder."),
          "50": ("Structure at 1:50: support disc, stem members at 62 % in seven stations, tapered spray members inside every branch.", "Tragwerk bei 1:50: Auflagerscheibe, Stammglieder mit 62 % in sieben Stationen, verjüngte Zweigglieder in jedem Ast.")}[scale]
    return [T(*body), T(*th), T(*st)]


if __name__ == "__main__":
    S = sys.modules[__name__]
    tl = tiers()
    for k in range(len(tl) - 1):
        h, R, D, top, rim = tl[k][:5]
        h2, R2, D2, top2, rim2 = tl[k + 1][:5]
        gap = (h2 - D2 - rim2) - (h + top)
        assert gap > 0.5 or gap < -0.15, (k, gap)
    doc, info = whorled.build(S)
    path = whorled.write(S, doc)
    print({k: v for k, v in info.items() if k not in ("report", "tiers")})
    print(info["tiers"])
    for sc, r in info["report"].items():
        print(sc, r["solids"], {v: len(g) for v, g in r["gaps"].items()})
    print(path)
