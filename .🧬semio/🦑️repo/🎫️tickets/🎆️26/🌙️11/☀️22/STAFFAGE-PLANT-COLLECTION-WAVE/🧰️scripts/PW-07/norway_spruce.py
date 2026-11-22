"""🌲️ Norway spruce (Picea abies): every number of the asset `norway-spruce`, chosen by hand after an open-grown tree of about 30 m with a
narrow, slightly concave cone, downswept "comb" branches whose tips turn up and whose branchlets hang, a clear stem to about 2 m in a park,
grey-brown scaly bark and a pointed leader. Run: `repo tools exec python -- norway_spruce.py`.
@see ./whorled.py
"""
import math
import sys

from conifer_lib import T
import whorled

id = "norway-spruce"
H = 30.0
stem_joints = [0.0, 0.6, 7.5, 14.5, 20.5, 25.5, H]
stem_names = ["trunk-1", "trunk-2", "trunk-3", "trunk-4", "leader"]
stem_titles = [T("Stem, lower (bare, scaly bark)", "Stamm unten (kahl, schuppige Borke)"), T("Stem, lower crown", "Stamm, untere Krone"), T("Stem, middle crown", "Stamm, mittlere Krone"),
               T("Stem, upper crown", "Stamm, obere Krone"), T("Leader", "Gipfeltrieb")]
stem_limits = [2, 2.5, 3, 4, 6]
whorl_limit = 10
spray_limit = 16
crown_base = 2.6
sun_from = 14.0
wood = "spruce-wood"
bark_lower, bark_upper = "spruce-bark", "spruce-bark-young"
needle_sun, needle_shade = "spruce-needles-sun", "spruce-needles-shade"
green_density = 800.0
E_green = 7.9e9
k_root = 25e6
root_plate_extra_kg = 0.0
footprint = 2.4
needle_area_sun, needle_area_shade = 140.0, 200.0
needle_kg_m2 = 0.45
needle_width = 0.0012
gs_sun, gs_shade = 0.0022, 0.0015
branches_per_whorl = 6
twig_factor = 1.8
cd = 0.35
v_calm, v_windy, v_storm = 2.0, 8.0, 22.0
whorl_lean, spray_lean = 5.0, 6.0
sway_period = 4.2
root_azimuths = [18, 96, 162, 232, 301]
root_len = [1.05, 0.8, 1.15, 0.9, 0.85]
root_r = [0.15, 0.12, 0.16, 0.13, 0.12]
bark_heat = (65000, 35000)
bark_upper_sky = 0.12
emitter_d = {"transpiration-sun": 2.2, "transpiration-shade": 2.6}
sun = {"bark-lower": 0.35, "bark-upper": 0.05, "needles-sun": 0.85, "needles-shade": 0.2}
transpiration_day = [(0, 0.03), (5, 0.05), (7, 0.3), (9, 0.8), (11.5, 1.0), (13.5, 0.85), (16, 0.72), (18, 0.3), (20, 0.06), (24, 0.03)]
environment = {"airTempC": 22, "meanRadiantTempC": 25, "relativeHumidity": 0.55, "windMS": 2, "windDirectionDeg": 0, "solarWM2": 700, "pressurePa": 101325, "diffuseFraction": 0.15}
words = {"name": ("Norway spruce", "Gemeinen Fichte"), "wood": ("spruce", "Fichtenholz"), "midday": ("a slight midday depression", "eine leichte Mittagsdepression")}
N_TIERS = 15
LS = [0.97, 1.05, 0.94, 1.03, 0.96, 1.06, 0.95, 1.02, 0.97, 1.04, 0.95, 1.03, 0.98, 1.02, 1.0]
DX = [0.06, -0.05, 0.04, -0.07, 0.05, -0.03, 0.06, -0.05, 0.03, -0.04, 0.04, -0.03, 0.02, -0.02, 0.0]
DZ = [-0.04, 0.06, -0.05, 0.03, -0.06, 0.05, -0.03, 0.04, -0.04, 0.03, -0.03, 0.02, -0.02, 0.01, 0.0]
JIT = [24, -18, 30, -26, 16, -30, 22, -14, 28, -22, 18, -26, 20, -16, 0]


def r_stem(y):
    if y >= H:
        return 0.0
    main = 0.30 * ((H - y) / (H - 1.3)) ** 0.85 + 0.30 * math.exp(-y / 0.25)
    if y > 25.0:
        main = max(main, 0.024 * min(1.0, (H - y) / 0.9))
    return main


def tiers():
    out = []
    for k in range(N_TIERS):
        x = k / (N_TIERS - 1)
        h = 4.25 + 23.85 * (1 - (1 - x) ** 1.15)
        R = 0.38 + 3.32 * ((28.7 - h) / (28.7 - 4.25)) ** 0.92
        D = 0.5 + 0.25 * R
        top = 0.62 + 0.09 * R
        rim = 0.26 + 0.06 * R
        out.append((h, R, D, top, rim, 0.55, LS[k], DX[k], DZ[k], JIT[k]))
    return out


def spray_droop(t):
    return math.degrees(math.atan2(t.D + 0.5 * t.rim, 1.07 * t.R)) * 0.75


def branch_d(t):
    return 0.025 + 0.018 * t.R


def spray_r(t):
    return 0.12 + 0.06 * t.R, 0.06 + 0.02 * t.R


def spray_tip_drop(t):
    return 0.12 + 0.05 * t.R


def spray_tip(t, p1, a):
    """Picea abies: the branch end turns up out of the hanging comb."""
    if t.R < 0.9:
        return None
    L = 0.18 * t.R
    return (p1[0] + L * math.cos(a), p1[1] + 0.05 + 0.03 * t.R, p1[2] + L * math.sin(a) * t.ls), 0.85 * spray_r(t)[1], 0.05


def hanging_sides(t, scale):
    """The diagonal sprays whose hanging comb ends make the jagged lower edge of a tier."""
    if t.R < 0.7:
        return []
    return ["ne", "nw", "sw", "se"] if scale == "50" else ["nw", "se"]


def hang_drop(t):
    return 0.22 + 0.1 * t.R


def branchlets(t, p0, p1, a):
    """The comb hangs from the diagonal sprays (`hanging_sides`); no separate curtains."""
    if True:
        return []
    out = []
    for s, L in ((0.55, 0.45 + 0.1 * t.R), (0.8, 0.35 + 0.08 * t.R)):
        q0 = tuple(p0[k] + (p1[k] - p0[k]) * s for k in range(3))
        q0 = (q0[0], q0[1] - 0.05, q0[2])
        q1 = (q0[0] + 0.05 * math.cos(a), q0[1] - L, q0[2] + 0.05 * math.sin(a) * t.ls)
        out.append((q0, q1, 0.15 + 0.02 * t.R, 0.08))
    return out


def lateral_offset(t):
    return [18, -22, 14, -16, 25, -12, 20, -24, 12, -18, 22, -14, 16, -20, 10][t.k - 1]


def tone(t):
    return 0.62 if t.region == "needles-sun" else 0.72


def mark_len(t):
    return 0.22 + 0.08 * t.R


def sway_amp(bid, base):
    if bid.startswith(("whorl", "spray")):
        return 0.45 * abs(base) + 0.4
    return 0.45 * abs(base)


def extra_poses(tiers, binding, loads, fmt):
    bones = {}
    snow = []
    kg_m2 = 25.0
    for i, t in enumerate(tiers):
        R_up = tiers[i + 1].R if i + 1 < len(tiers) else 0.0
        A = math.pi * (t.R ** 2 - 0.8 * R_up ** 2)
        F = kg_m2 * 9.81 * A
        snow.append({"id": "snow-%s" % t.bone.id, "kind": "snow", "title": T("Snow on branch whorl %d" % t.k, "Schnee auf Astquirl %d" % t.k), "bone": t.bone.id, "x": 0.0, "y": 0, "fxN": 0,
                     "fyN": -round(0.6 * F, 0), "source": "authored-estimate", "reference": "en-1991-1-3"})
        for b, sgn in ((t.west, 1), (t.east, -1)):
            snow.append({"id": "snow-%s" % b.id, "kind": "snow", "title": T("Snow on the %s spray of whorl %d" % ("west" if sgn > 0 else "east", t.k), "Schnee auf dem %s Zweig von Quirl %d" % ("westlichen" if sgn > 0 else "östlichen", t.k)),
                         "bone": b.id, "x": round(0.6 * b.L, 3), "y": 0, "fxN": 0, "fyN": -round(0.2 * F, 0), "source": "authored-estimate", "reference": "en-1991-1-3"})
            bones[b.id] = {"angleDeg": round(sgn * min(14.0, 3.0 + 3.0 * t.R), 2)}
    total = sum(-l["fyN"] for l in snow) / 1000
    return [{"id": "snow", "title": T("Wet snow on the tiers", "Nassschnee auf den Etagen"),
             "description": T("An evergreen keeps its needles in winter: %.0f kg/m2 of wet snow on the plan area of every tier that the tier above leaves open (order of magnitude after EN 1991-1-3), %.1f kN in all, bends the branch sprays down by 6 to 14 degrees, so the comb hangs steeper; calm, overcast, the transpiration nearly stops." % (kg_m2, total),
                              "Ein Immergrüner behält im Winter seine Nadeln: %s kg/m2 Nassschnee auf der Grundrissfläche jeder Etage, die die darüberliegende frei lässt (Größenordnung nach EN 1991-1-3), insgesamt %s kN, biegen die Zweige um 6 bis 14 Grad herab, so dass der Kamm steiler hängt; windstill, bedeckt, die Transpiration ruht fast." % (fmt(kg_m2, True, 0), fmt(total, True))),
             "bones": bones, "ground": "authored", "contacts": [],
             "thermal": {"regions": binding({k: 0.0 for k in sun}, 0.6), "emitters": {"transpiration-sun": 0.03, "transpiration-shade": 0.03}}, "loads": loads(v_calm) + snow}]


title = T("Norway Spruce", "Gemeine Fichte")
summary = T("A mature, open-grown Norway spruce (Picea abies) of 30 m: a straight, slender stem with grey-brown scaly bark and a pointed leader, carrying a narrow, dense cone of fifteen branch tiers from about 2.3 m upwards; the downswept branches turn up at their tips and let their branchlets hang like a comb.",
            "Eine ausgewachsene, frei stehende Gemeine Fichte (Picea abies) von 30 m: ein gerader, schlanker Stamm mit graubrauner, schuppiger Borke und spitzem Gipfeltrieb, der einen schmalen, dichten Kegel aus fünfzehn Astetagen ab etwa 2,3 m trägt; die abwärts geschwungenen Äste biegen an den Spitzen nach oben und lassen ihre Zweige kammartig hängen.")
tags = ["tree", "evergreen", "conifer", "conical-crown", "wind", "snow", "transpiration"]
distinction = T("A narrow, dense cone of fifteen drooping comb tiers on a straight 30 m stem, the tallest and slimmest of the conifers, with 340 m² of needles against 150 m² for the Scots pine.",
                "Ein schmaler, dichter Kegel aus fünfzehn hängenden Kammetagen an einem geraden 30-m-Stamm, der höchste und schlankste der Nadelbäume, mit 340 m² Nadeln gegenüber 150 m² bei der Waldkiefer.")
notes = T("Authored by hand after a 90-year-old open-grown spruce: the crown starts at about 2.3 m, the stem is 0.60 m thick at breast height. Masses are the volume of every drawn wooden solid of the 1:50 model times green density (Picea abies, Wood Handbook spruce proxy) plus needle and branch estimates per tier; the root system below ground is not part of the asset. The 2D views of every scale are the exact orthographic projections of that scale's 3D body.",
          "Von Hand nach einer 90-jährigen, frei stehenden Fichte erstellt: die Krone beginnt in etwa 2,3 m, der Stamm ist in Brusthöhe 0,60 m dick. Massen sind das Volumen jedes gezeichneten Holzkörpers des 1:50-Modells mal Frischholzdichte (Picea abies, Fichten-Proxy des Wood Handbook) plus Nadel- und Astschätzungen je Etage; das Wurzelsystem unter dem Boden gehört nicht zum Asset. Die 2D-Ansichten jedes Maßstabs sind die exakten Parallelprojektionen des 3D-Körpers dieses Maßstabs.")
references = [
    {"id": "wood-handbook-2021", "citation": "Forest Products Laboratory (2021). Wood Handbook: Wood as an Engineering Material. General Technical Report FPL-GTR-282, USDA Forest Service. Chapter 5 (green density, modulus of elasticity and rupture; white and red spruce as proxy for Picea abies)."},
    {"id": "en-1991-1-4", "citation": "EN 1991-1-4:2005 Eurocode 1, Actions on structures, Part 1-4: Wind actions. Air density 1.25 kg/m3, F = 0.5 rho cd A v^2, peak velocity pressure (1 + 7 Iv) q_m."},
    {"id": "en-1991-1-3", "citation": "EN 1991-1-3:2003 Eurocode 1, Actions on structures, Part 1-3: Snow loads. Used for the order of magnitude of wet snow intercepted by evergreen crowns."},
    {"id": "mayhead-1973", "citation": "Mayhead, G. J. (1973). Some drag coefficients for British forest trees derived from wind tunnel studies. Agricultural Meteorology 12, 123-130."},
    {"id": "moore-maguire-2004", "citation": "Moore, J. R. and Maguire, D. A. (2004). Natural sway frequencies and damping ratios of trees: concepts, review and synthesis of previous studies. Trees 18, 195-203."},
    {"id": "jones-2014", "citation": "Jones, H. G. (2014). Plants and Microclimate, 3rd ed., Cambridge University Press. Leaf energy balance, leaf area index, boundary-layer conductance."},
    {"id": "monteith-unsworth-2013", "citation": "Monteith, J. L. and Unsworth, M. H. (2013). Principles of Environmental Physics, 4th ed., Academic Press. Penman-Monteith leaf transpiration."},
    {"id": "farjon-2010", "citation": "Farjon, A. (2010). A Handbook of the World's Conifers. Brill, Leiden. Picea abies: habit, height, crown form and branching."},
    {"id": "caudullo-2016-picea", "citation": "Caudullo, G., Tinner, W. and de Rigo, D. (2016). Picea abies in Europe: distribution, habitat, usage and threats. In: San-Miguel-Ayanz, J. et al. (eds.), European Atlas of Forest Tree Species. Publications Office of the European Union, Luxembourg."},
]
materials = [
    {"id": "spruce-bark", "title": T("Grey-brown scaly spruce bark", "Graubraune, schuppige Fichtenborke"), "emissivity": 0.95, "solarAbsorptivity": 0.78, "densityKgM3": 480, "conductivityWmK": 0.12, "source": "authored-estimate", "reference": "jones-2014"},
    {"id": "spruce-bark-young", "title": T("Thin reddish-brown bark of the upper stem", "Dünne rotbraune Rinde des oberen Stamms"), "emissivity": 0.94, "solarAbsorptivity": 0.7, "densityKgM3": 450, "conductivityWmK": 0.11, "source": "authored-estimate", "reference": "jones-2014"},
    {"id": "spruce-needles-sun", "title": T("Spruce needles, sunlit upper tiers", "Fichtennadeln, besonnte obere Etagen"), "emissivity": 0.97, "solarAbsorptivity": 0.55, "source": "literature", "reference": "jones-2014"},
    {"id": "spruce-needles-shade", "title": T("Spruce needles, shaded lower tiers", "Fichtennadeln, beschattete untere Etagen"), "emissivity": 0.97, "solarAbsorptivity": 0.3, "source": "authored-estimate", "reference": "jones-2014"},
    {"id": "spruce-wood", "title": T("Green spruce wood", "Frisches Fichtenholz"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 800, "conductivityWmK": 0.14, "elasticModulusPa": 7.9e9, "strengthPa": 3.5e7, "source": "literature", "reference": "wood-handbook-2021"},
]
region_titles = {"bark-lower": T("Lower stem and root flare, scaly bark", "Unterer Stamm und Wurzelanlauf, schuppige Borke"), "bark-upper": T("Stem inside the crown and leader", "Stamm in der Krone und Gipfeltrieb"),
                 "needles-sun": T("Needles of the sunlit upper tiers", "Nadeln der besonnten oberen Etagen"), "needles-shade": T("Needles of the shaded lower tiers", "Nadeln der beschatteten unteren Etagen")}
emitter_titles = {"transpiration-sun": T("Transpiration of the sun needles (distributed over the upper crown)", "Transpiration der Sonnennadeln (über die obere Krone verteilt)"),
                  "transpiration-shade": T("Transpiration of the shade needles (distributed over the lower crown)", "Transpiration der Schattennadeln (über die untere Krone verteilt)")}
thermal_assumptions = [
    T("Needle regions use the needle exchange area; the leaf area index is that area over the crown ground projection. The maximum stomatal conductance (2.2 mm/s sunlit, 1.5 mm/s shaded) is an authored estimate for a boreal conifer guided by Jones (2014); photosynthesis type C3. Aperture and night closure are the engine's business, not authored here.",
      "Nadelregionen verwenden die Austauschfläche der Nadeln; der Blattflächenindex ist diese Fläche bezogen auf die Kronenprojektion am Boden. Die maximale Spaltöffnungsleitfähigkeit (2,2 mm/s besonnt, 1,5 mm/s beschattet) ist eine Autorenschätzung für einen borealen Nadelbaum, angelehnt an Jones (2014); Photosynthesetyp C3. Öffnung und nächtliches Schließen sind Sache der Engine und werden hier nicht vorgegeben."),
    T("Bark is passive. Its areal heat capacity is the volumetric heat capacity of green wood (about 2.7 MJ per m3 and K) times an effective depth of 1 to 2.5 cm, an authored estimate guided by the Wood Handbook; the bark areas are the lateral areas of the 1:50 stem, flare and root solids. No surface temperature is authored.",
      "Die Rinde ist passiv. Ihre flächenbezogene Wärmekapazität ist die volumetrische Wärmekapazität von frischem Holz (etwa 2,7 MJ je m3 und K) mal einer wirksamen Tiefe von 1 bis 2,5 cm, eine Autorenschätzung angelehnt an das Wood Handbook; die Rindenflächen sind die Mantelflächen der 1:50-Körper von Stamm, Anlauf und Wurzeln. Es werden keine Oberflächentemperaturen vorgegeben."),
    T("Transpiration emitters release the latent flux of their needle region as vapour, spread over six anchors across the upper or the lower crown; the direction is the vertical of the whorl bone of the first anchor, since the release is diffuse. The sun needles are the tiers from 14 m upwards.",
      "Transpirationsemitter geben den latenten Strom ihrer Nadelregion als Dampf ab, verteilt auf sechs Anker über die obere oder untere Krone; die Richtung ist die Senkrechte des Quirlknochens des ersten Ankers, da die Abgabe diffus ist. Die Sonnennadeln sind die Etagen ab 14 m."),
]
structure_assumptions = [
    T("One rigid body per evaluated pose, fixed at the root plate. Wind drag acts at the centroid of the silhouette band of every tier and as a line load on the bare lower stem; snow acts on the whorls and sprays. Sections are solid circles of green spruce wood; the stem lean of the wind poses is the deflection of a tapered cantilever (E 7.9 GPa) on a root plate of 25 MN m per radian. Dynamic amplification, branch shedding and root-soil failure are not modelled.",
      "Ein starrer Körper je ausgewerteter Pose, am Wurzelteller eingespannt. Winddruck wirkt im Schwerpunkt des Silhouettenbands jeder Etage und als Linienlast am kahlen unteren Stamm; Schnee wirkt auf Quirle und Zweige. Querschnitte sind Vollkreise aus frischem Fichtenholz; die Stammneigung der Windposen ist die Durchbiegung eines verjüngten Kragarms (E 7,9 GPa) auf einem Wurzelteller von 25 MNm je Radiant. Dynamische Überhöhung, Astbruch und Wurzel-Boden-Versagen werden nicht modelliert."),
]


def rep_decisions(scale, n, dims, fmt):
    h, w = dims["heightM"], dims["widthM"]
    if scale == "500":
        return {
            "silhouette": ("At 1:500 (1 m is 2 mm, the tree %s mm high) the spruce is one narrow, serrated cone on a short stem: the crown path is the true side view of the 1:500 crown solid, whose radial envelope steps out at every one of the %d tiers, and it ends in the pointed leader." % (fmt(h * 2, False, 0), n),
                           "Bei 1:500 (1 m sind 2 mm, der Baum %s mm hoch) ist die Fichte ein schmaler, gesägter Kegel auf kurzem Stamm: der Kronenpfad ist die wahre Seitenansicht des 1:500-Kronenkörpers, dessen Hüllradius an jeder der %d Etagen vorspringt, und endet im spitzen Gipfeltrieb." % (fmt(h * 2, True, 0), n)),
            "retained": ("Retained: the clear stem with its flare, the serrated outline of the tiers, the concave spire and the overall width of %s m." % fmt(w, False, 2), "Beibehalten: der freie Stamm mit Anlauf, der gesägte Umriss der Etagen, die eingezogene Spitze und die Gesamtbreite von %s m." % fmt(w, True, 2)),
            "merged": ("Merged: all tiers, sprays, branch tips and hanging branchlets into two crown solids (sunlit above 13.7 m, shaded below) drawn as one crown path with two outlines; the upper stem and the leader into the crown.", "Vereinigt: alle Etagen, Zweige, Astspitzen und hängenden Zweiglein zu zwei Kronenkörpern (besonnt über 13,7 m, beschattet darunter), gezeichnet als ein Kronenpfad mit zwei Umrissen; der obere Stamm und der Gipfeltrieb in die Krone."),
            "omitted": ("Omitted: single sprays, the surface roots, bark and needle texture.", "Weggelassen: einzelne Zweige, die Wurzelanläufe, Borken- und Nadeltextur."),
            "thinElements": ("No thin part is thickened: the leader is the sharp tip of the crown path, the branch tips are inside the envelope.", "Kein dünnes Teil wird verdickt: der Gipfeltrieb ist die scharfe Spitze des Kronenpfads, die Astspitzen liegen in der Hülle."),
            "hatching": ("No hatching; outlines 0.15 mm, ground line 0.25 mm.", "Keine Schraffur; Umrisse 0,15 mm, Bodenlinie 0,25 mm."),
            "styles": ("Light line: the two outlines. Dark solid: the crown reads as one dark cone, the shaded lower part darker, the stem lighter.", "Helle Linie: die zwei Umrisse. Dunkle Fläche: die Krone liest sich als ein dunkler Kegel, der beschattete untere Teil dunkler, der Stamm heller."),
            "overlays": ("Thermal: bark-lower on the stem path, needles-sun and the hidden bark-upper merged in the upper crown, needles-shade in the lower crown. Structure: the stem members, the spray members and the root support carry the colours.", "Thermisch: bark-lower auf dem Stammpfad, needles-sun und die verdeckte bark-upper vereint in der oberen Krone, needles-shade in der unteren Krone. Tragwerk: die Stammglieder, die Zweigglieder und das Wurzelauflager tragen die Farben."),
        }
    if scale == "200":
        return {
            "silhouette": ("At 1:200 (1 m is 5 mm) the %d tiers are separate drooping skirts layered from the stem outwards, each the true view of its 1:200 skirt solid, stacked into the narrow cone; the stem shows below 2.3 m." % n,
                           "Bei 1:200 (1 m sind 5 mm) sind die %d Etagen einzelne hängende Schürzen, vom Stamm aus geschichtet, jede die wahre Ansicht ihres 1:200-Schürzenkörpers, zum schmalen Kegel gestapelt; unter 2,3 m zeigt sich der Stamm." % n),
            "retained": ("Retained: every tier with its hanging rim, the slight eccentricity and ellipticity of each tier, the stem with its flare and the leader.", "Beibehalten: jede Etage mit hängendem Saum, die leichte Außermittigkeit und Elliptizität jeder Etage, der Stamm mit Anlauf und der Gipfeltrieb."),
            "merged": ("Merged: the sprays, upturned tips and branchlets of each tier into its skirt (6 %% wider and 10 cm deeper than at 1:50); the root flare into the lower stem.", "Vereinigt: Zweige, aufgebogene Spitzen und Zweiglein jeder Etage in ihre Schürze (6 %% breiter und 10 cm tiefer als bei 1:50); der Wurzelanlauf in den unteren Stamm."),
            "omitted": ("Omitted: surface roots, bark and needle texture.", "Weggelassen: Wurzelanläufe, Borken- und Nadeltextur."),
            "thinElements": ("The leader keeps its true taper; nothing is thickened.", "Der Gipfeltrieb behält seine wahre Verjüngung; nichts wird verdickt."),
            "hatching": ("No hatching; outlines 0.18 mm.", "Keine Schraffur; Umrisse 0,18 mm."),
            "styles": ("Light line: the layered skirt outlines read as tiers. Dark solid: shaded lower tiers darker than the sunlit upper ones.", "Helle Linie: die geschichteten Schürzenumrisse lesen sich als Etagen. Dunkle Fläche: die beschatteten unteren Etagen dunkler als die besonnten oberen."),
            "overlays": ("Thermal: each skirt carries its region (needles-sun from 14 m), the stem bark-lower and bark-upper. Structure: stem segments and spray members.", "Thermisch: jede Schürze trägt ihre Region (needles-sun ab 14 m), der Stamm bark-lower und bark-upper. Tragwerk: Stammabschnitte und Zweigglieder."),
        }
    if scale == "100":
        return {
            "silhouette": ("At 1:100 (1 m is 10 mm) each of the %d tiers is a skirt with its west and east sprays reaching beyond the rim, so the cone outline is ragged at every tier; side and front differ because every tier is slightly elliptic and off-centre." % n,
                           "Bei 1:100 (1 m sind 10 mm) ist jede der %d Etagen eine Schürze, deren westlicher und östlicher Zweig über den Saum hinausreichen, so dass der Kegelumriss an jeder Etage ausfranst; Seiten- und Vorderansicht unterscheiden sich, weil jede Etage leicht elliptisch und außermittig ist." % n),
            "retained": ("Retained: skirts, the two sagittal sprays per tier, the root flare, stem segments and leader, and five branchlet hatch strokes per tier.", "Beibehalten: Schürzen, die zwei Zweige in der Rig-Ebene je Etage, Wurzelanlauf, Stammabschnitte und Gipfeltrieb sowie fünf Zweigleinstriche je Etage."),
            "merged": ("Merged: the upturned tips and hanging branchlets into the sprays (9 %% reach, thicker ends); the lateral sprays into the skirts.", "Vereinigt: aufgebogene Spitzen und hängende Zweiglein in die Zweige (9 %% Reichweite, dickere Enden); die seitlichen Zweige in die Schürzen."),
            "omitted": ("Omitted: surface roots and bark texture.", "Weggelassen: Wurzelanläufe und Borkentextur."),
            "thinElements": ("Sprays keep their true section; strokes stay 0.5 mm inside their tier and away from every other outline.", "Zweige behalten ihren wahren Querschnitt; Striche bleiben 0,5 mm innerhalb ihrer Etage und fern von jedem anderen Umriss."),
            "hatching": ("No hatch pattern; short hanging texture strokes of 0.1 mm suggest the comb branchlets.", "Kein Schraffurmuster; kurze hängende Texturstriche von 0,1 mm deuten die Kammzweige an."),
            "styles": ("Light line: outlines and comb strokes. Dark solid: tiers layered, shade tiers darker.", "Helle Linie: Umrisse und Kammstriche. Dunkle Fläche: Etagen geschichtet, Schattenetagen dunkler."),
            "overlays": ("Thermal: skirts and sprays in their needle region, stem regions on the stem. Structure: stem and spray members coloured.", "Thermisch: Schürzen und Zweige in ihrer Nadelregion, Stammregionen am Stamm. Tragwerk: Stamm- und Zweigglieder eingefärbt."),
        }
    return {
        "silhouette": ("At 1:50 (1 m is 20 mm) every tier is the true view of its skirt, four sprays (west and east on their own bones, north and south on the whorl), the upturned branch tips and, on the lower tiers, the hanging branchlet curtains; the stem stands on its flare with five surface roots.",
                       "Bei 1:50 (1 m sind 20 mm) ist jede Etage die wahre Ansicht ihrer Schürze, von vier Zweigen (West und Ost an eigenen Knochen, Nord und Süd am Quirl), den aufgebogenen Astspitzen und an den unteren Etagen den hängenden Zweigleinvorhängen; der Stamm steht auf seinem Anlauf mit fünf Wurzelanläufen."),
        "retained": ("Retained: everything modelled at 1:50, plus comb hatching in every tier, scaly bark strokes on the bare stem and radial needle strokes in the plan rings.", "Beibehalten: alles bei 1:50 Modellierte, dazu Kammschraffur in jeder Etage, schuppige Borkenstriche am kahlen Stamm und radiale Nadelstriche in den Grundrissringen."),
        "merged": ("Merged: single needles and side shoots into the sprays and skirts.", "Vereinigt: einzelne Nadeln und Seitentriebe in Zweige und Schürzen."),
        "omitted": ("Omitted: cones (they hang in the top tiers and are below 0.2 mm in plan) and dead lower stubs.", "Weggelassen: Zapfen (sie hängen in den obersten Etagen und sind im Grundriss unter 0,2 mm) und tote untere Stummel."),
        "thinElements": ("Upturned tips (r 4.5 cm at the end) and branchlets keep their true size; texture strokes keep 0.5 mm from every outline so no gap below the print minimum is created.", "Aufgebogene Spitzen (r 4,5 cm am Ende) und Zweiglein behalten ihre wahre Größe; Texturstriche halten 0,5 mm Abstand zu jedem Umriss, damit kein Spalt unter dem Druckminimum entsteht."),
        "hatching": ("No hatch pattern; texture strokes of 0.13 mm.", "Kein Schraffurmuster; Texturstriche von 0,13 mm."),
        "styles": ("Light line: outlines, comb and bark strokes. Dark solid: layered tiers, sprays slightly lighter than their skirt, shade tiers darker.", "Helle Linie: Umrisse, Kamm- und Borkenstriche. Dunkle Fläche: geschichtete Etagen, Zweige etwas heller als ihre Schürze, Schattenetagen dunkler."),
        "overlays": ("Thermal: every needle part in its region, flare, roots and lower stem bark-lower, upper stem and leader bark-upper. Structure: stem, root plate and every spray member coloured, the support under the root plate.", "Thermisch: jedes Nadelteil in seiner Region, Anlauf, Wurzeln und unterer Stamm bark-lower, oberer Stamm und Gipfeltrieb bark-upper. Tragwerk: Stamm, Wurzelteller und jedes Zweigglied eingefärbt, das Auflager unter dem Wurzelteller."),
    }


def volume_decisions(scale, nbody, n, fmt):
    body = {"500": ("Body at 1:500 (smallest feature 75 mm): the stem with its flare up into the crown and the crown as two solids of revolution (sunlit and shaded) whose stepped envelope is the union of the 1:50 crown; %d solids." % nbody,
                    "Körper bei 1:500 (kleinstes Merkmal 75 mm): der Stamm mit Anlauf bis in die Krone und die Krone als zwei Drehkörper (besonnt und beschattet), deren gestufte Hülle die Vereinigung der 1:50-Krone ist; %d Körper." % nbody),
            "200": ("Body at 1:200 (30 mm): five stem solids and %d skirts of revolution, each 6 %% wider than at 1:50 so that it holds its sprays; %d solids." % (n, nbody),
                    "Körper bei 1:200 (30 mm): fünf Stammkörper und %d Drehschürzen, jede 6 %% breiter als bei 1:50, so dass sie ihre Zweige enthält; %d Körper." % (n, nbody)),
            "100": ("Body at 1:100 (15 mm): flare, stem segments, leader, %d skirts and two tapered spray capsules per tier on the spray bones; %d solids." % (n, nbody),
                    "Körper bei 1:100 (15 mm): Anlauf, Stammabschnitte, Gipfeltrieb, %d Schürzen und zwei verjüngte Zweigkapseln je Etage an den Zweigknochen; %d Körper." % (n, nbody)),
            "50": ("Body at 1:50 (7.5 mm): flare with five surface roots, stem segments and leader, %d skirts of revolution, four sprays per tier, upturned tips and hanging branchlet curtains; %d solids." % (n, nbody),
                   "Körper bei 1:50 (7,5 mm): Anlauf mit fünf Wurzelanläufen, Stammabschnitte und Gipfeltrieb, %d Drehschürzen, vier Zweige je Etage, aufgebogene Spitzen und hängende Zweigleinvorhänge; %d Körper." % (n, nbody))}[scale]
    th = {"500": ("Thermal at 1:500: the stem and the two crown solids by reference; the bark of the upper stem is a capsule hidden inside the crown; twelve transpiration outlets.", "Thermisch bei 1:500: Stamm und die zwei Kronenkörper per Verweis; die Rinde des oberen Stamms ist eine in der Krone verborgene Kapsel; zwölf Transpirationsauslässe."),
          "200": ("Thermal at 1:200: every body solid by reference in its region; twelve transpiration outlets at the emitter anchors.", "Thermisch bei 1:200: jeder Körper per Verweis in seiner Region; zwölf Transpirationsauslässe an den Emitterankern."),
          "100": ("Thermal at 1:100: every body solid by reference; twelve outlets.", "Thermisch bei 1:100: jeder Körper per Verweis; zwölf Auslässe."),
          "50": ("Thermal at 1:50: every body solid by reference, so the bark areas of the regions are the lateral areas of these solids; twelve outlets.", "Thermisch bei 1:50: jeder Körper per Verweis, so dass die Rindenflächen der Regionen die Mantelflächen dieser Körper sind; zwölf Auslässe.")}[scale]
    st = {"500": ("Structure at 1:500: root support disc, root-plate and stem members at 56 % of the stem radius in three stations, and one slender member along every spray bone inside the crown.", "Tragwerk bei 1:500: Wurzelauflagerscheibe, Wurzelteller- und Stammglieder mit 56 % des Stammradius in drei Stationen und ein schlankes Glied längs jedes Zweigknochens in der Krone."),
          "200": ("Structure at 1:200: support disc, stem members at 58 % of the radius in four stations, slender spray members.", "Tragwerk bei 1:200: Auflagerscheibe, Stammglieder mit 58 % des Radius in vier Stationen, schlanke Zweigglieder."),
          "100": ("Structure at 1:100: support disc, stem members at 60 % in five stations, tapered slender spray members.", "Tragwerk bei 1:100: Auflagerscheibe, Stammglieder mit 60 % in fünf Stationen, verjüngte schlanke Zweigglieder."),
          "50": ("Structure at 1:50: support disc, stem members at 62 % in seven stations, tapered slender spray members inside every spray.", "Tragwerk bei 1:50: Auflagerscheibe, Stammglieder mit 62 % in sieben Stationen, verjüngte schlanke Zweigglieder in jedem Zweig.")}[scale]
    return [T(*body), T(*th), T(*st)]


if __name__ == "__main__":
    S = sys.modules[__name__]
    tl = tiers()
    for k in range(len(tl) - 1):
        h, R, D, top, rim = tl[k][:5]
        h2, R2, D2, top2, rim2 = tl[k + 1][:5]
        ov = (h + top) - (h2 - D2 - rim2)
        assert ov > 0.15, (k, ov)
    doc, info = whorled.build(S)
    path = whorled.write(S, doc)
    import json
    print(json.dumps({k: v for k, v in info.items() if k != "report"}, indent=0)[:3000])
    for sc, r in info["report"].items():
        print(sc, r["solids"], {v: len(g) for v, g in r["gaps"].items()}, [g[:3] for v, gs in r["gaps"].items() for g in gs[:4]])
    print(path)
