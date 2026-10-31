"""seated-reader v1 -> v2 (CV-H1)."""
import copy
from lib import *
import rd_geom

aid = "seated-reader"
src = json.load(open(f"v1/{aid}.json"))
a = src["asset"]
R = rd_geom.build(a, next(p for p in a["poses"] if p["id"] == "seated-upright"))

rm = {"bench-seat": "bench", "bench-slat-1": "bench", "bench-leg-front": "bench", "bench-leg-rear": "bench", "bench-back": "bench", "bench-back-slat": "bench", "bench-back-slat-2": "bench",
      "cup-body": "cup", "cup-lid": "cup", "cup-sleeve": "cup"}
hm = {"sweater": "knit", "sleeve-l": "knit", "sleeve-r": "knit", "forearm-l": "knit", "forearm-r": "knit"}
R["50"] = {"status": "authored", "denominator": 50, "validRange": {"minDenominator": 20, "maxDenominator": 75},
           "pens": {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": 0.1, "ground": 0.3},
           "hatches": [{"id": "knit", "title": T("Knitted sweater hatching", "Schraffur Strickpullover"), "pattern": "lines", "angleDeg": 90, "spacingMm": 0.5, "penMm": 0.1}],
           "geometry": {"elevation": v1_to_50(a["geometry"]["elevation"], hm, region_map=rm)}}

R["500"]["decisions"] = decisions(
    dict(silhouette="Seated mass: sweater, cowl, head and bun as one torso silhouette leaning slightly forward, thighs and shins as three tapered columns, one bench outline with the back, two legs and the cup; 2.7 mm tall, bench width 0.43 m.",
         retained="The seated posture (hip angle, knees, flat feet), the hair bun that tips the head to 1.34 m, the near arm reaching to the notebook, the bench back as a leaning line, exact 1.34 m height.",
         merged="Sweater, face, hair and cowl in one form (regions listed); hand and forearm in one shape; bench seat, back, legs and the paper cup in one bench outline.",
         omitted="The far arm, the page and the neck, hands as separate shapes, shoe toe joint, bench slats and the cup lid.",
         thinElements="The 16 mm notebook is kept as a 30 mm bar (pen width 0.075 m on top); bench legs 50 mm are widened to 60 mm, the seat slab stays 40 mm; slats, page and cup lid are dropped.",
         hatching="None.",
         styles="Mid-grey sweater mass, lighter jeans, dark shoes and bench; pen 0.13 mm; no detail pen.",
         overlays="The torso form shows the area-weighted value of sweater, face and hair (merged); jeans, shoes, notebook and bench have their own colour; the cup is merged into the bench."),
    dict(silhouette="Sitzmasse: Pullover, Rollkragen, Kopf und Dutt als eine leicht vorgeneigte Rumpfsilhouette, Ober- und Unterschenkel als drei verjüngte Säulen, eine Bankkontur mit Lehne, zwei Beinen und dem Becher; 2,7 mm hoch, Bankbreite 0,43 m.",
         retained="Die Sitzhaltung (Hüftwinkel, Knie, flache Füße), der Haardutt, der den Kopf auf 1,34 m hebt, der nahe Arm bis zum Notizbuch, die Bankrückenlehne als geneigte Linie, genau 1,34 m Höhe.",
         merged="Pullover, Gesicht, Haar und Kragen in einer Form (Regionen aufgelistet); Hand und Unterarm in einer Form; Sitz, Lehne, Beine der Bank und der Pappbecher in einer Bankkontur.",
         omitted="Der ferne Arm, die Seite und der Hals, Hände als eigene Formen, Zehengelenk, Bankleisten und der Becherdeckel.",
         thinElements="Das 16 mm dicke Notizbuch bleibt als 30-mm-Balken (dazu die Strichbreite 0,075 m); die 50 mm breiten Bankbeine werden auf 60 mm verbreitert, die Sitzbohle bleibt 40 mm; Leisten, Seite und Becherdeckel entfallen.",
         hatching="Keine.",
         styles="Mittelgrauer Pullover, hellere Jeans, dunkle Schuhe und Bank; Stift 0,13 mm; kein Detailstift.",
         overlays="Die Rumpfform zeigt den flächengewichteten Wert von Pullover, Gesicht und Haar (verschmolzen); Jeans, Schuhe, Notizbuch und Bank haben eigene Farben; der Becher ist in die Bank eingeschmolzen."))
R["200"]["decisions"] = decisions(
    dict(silhouette="Separated masses: bun, hair, face, cowl, sweater, both arms (sleeve, forearm, hand), jeans seat, thighs, shins, shoes; bench as seat slab, two legs and back; notebook and cup as own shapes; 6.7 mm tall.",
         retained="Elbow and knee bends, hands as skin-coloured blobs on the notebook, the notebook as a bar with its page line (so the page-turn reads), the cup with its lid.",
         merged="Neck into cowl and head, thumb into hand, sweater rib and sleeve cuffs into the masses.",
         omitted="Fingers, ear, eye, slats, shoe sole line, cup sleeve.",
         thinElements="Notebook cover drawn 30 mm thick instead of 16 mm; page as one 0.1 mm detail line; bench legs 50 mm kept.",
         hatching="None.",
         styles="Body pen 0.18 mm, detail pen 0.10 mm for the page line only; tones: sweater mid, jeans light, bench mid, shoes dark.",
         overlays="One region per mass; both sleeves share the sweater value, the bench slab shows the seat temperature at the contact."),
    dict(silhouette="Getrennte Massen: Dutt, Haar, Gesicht, Kragen, Pullover, beide Arme (Ärmel, Unterarm, Hand), Jeans am Gesäß, Ober- und Unterschenkel, Schuhe; Bank als Sitzbohle, zwei Beine und Lehne; Notizbuch und Becher als eigene Formen; 6,7 mm hoch.",
         retained="Ellbogen- und Kniebeugen, Hände als hautfarbene Flecken am Notizbuch, das Notizbuch als Balken mit Seitenlinie (damit das Umblättern lesbar bleibt), der Becher mit Deckel.",
         merged="Hals in Kragen und Kopf, Daumen in die Hand, Pullover-Rippe und Ärmelbündchen in die Massen.",
         omitted="Finger, Ohr, Auge, Leisten, Sohlenlinie, Becherhülse.",
         thinElements="Notizbuchdeckel 30 mm statt 16 mm dick gezeichnet; die Seite als eine 0,1-mm-Detaillinie; Bankbeine mit 50 mm unverändert.",
         hatching="Keine.",
         styles="Körperstift 0,18 mm, Detailstift 0,10 mm nur für die Seitenlinie; Töne: Pullover mittel, Jeans hell, Bank mittel, Schuhe dunkel.",
         overlays="Eine Region je Masse; beide Ärmel teilen den Pullover-Wert, die Bankbohle zeigt die Sitztemperatur am Kontakt."))
R["100"]["decisions"] = decisions(
    dict(silhouette="Articulated seated figure: knitted sweater with cowl and rib, jeans with seam, sneakers with sole line and flexing forefoot, mitten hands with thumb on a bound notebook, hair with bun and tie; 13.4 mm tall.",
         retained="Face profile with nose and ear, bench slats and the back slat, notebook with edge line and page, paper cup with sleeve line.",
         merged="Eye, brow and mouth line are not drawn; the finger groups are one mitten outline.",
         omitted="Eye (0.14 mm), shoe laces, cup rim, knit texture strokes.",
         thinElements="Notebook 28 mm thick (real 16 mm) with a 0.1 mm edge line; slats as 0.1 mm lines; fingers merged.",
         hatching="Sweater only: vertical lines (the knit wales), 0.6 mm spacing, 0.10 mm pen; jeans and bench stay plain.",
         styles="Body 0.2 mm, detail 0.1 mm; jeans light, sweater mid with hatch, shoes dark.",
         overlays="Regions as in 1:200 with the cup and the notebook edge sharing their region colours."),
    dict(silhouette="Gegliederte sitzende Figur: Strickpullover mit Rollkragen und Bündchen, Jeans mit Naht, Turnschuhe mit Sohlenlinie und beweglichem Vorfuß, Fäustlingshände mit Daumen am gebundenen Notizbuch, Haar mit Dutt und Haargummi; 13,4 mm hoch.",
         retained="Gesichtsprofil mit Nase und Ohr, Bankleisten und Lehnenleiste, Notizbuch mit Kantenlinie und Seite, Pappbecher mit Hülsenlinie.",
         merged="Auge, Braue und Mundlinie werden nicht gezeichnet; die Fingergruppen sind eine Fäustlingskontur.",
         omitted="Auge (0,14 mm), Schnürsenkel, Becherrand, Strickstruktur.",
         thinElements="Notizbuch 28 mm dick (real 16 mm) mit 0,1-mm-Kantenlinie; Leisten als 0,1-mm-Linien; Finger verschmolzen.",
         hatching="Nur der Pullover: senkrechte Linien (die Maschenstäbchen), 0,6 mm Abstand, 0,10 mm Stift; Jeans und Bank bleiben flächig.",
         styles="Körper 0,2 mm, Detail 0,1 mm; Jeans hell, Pullover mittel mit Schraffur, Schuhe dunkel.",
         overlays="Regionen wie 1:200, Becher und Notizbuchkante teilen die Farben ihrer Regionen."))
R["50"]["decisions"] = decisions(
    dict(silhouette="Fully articulated drawing with every v1 detail: ribbed knit strokes, jeans seams, finger separations, ear, eye, shoe laces, bench slats, notebook pages with ruling; 26.8 mm tall.",
         retained="All 56 v1 shapes, re-bound to v2 regions (bench and cup now carry their own regions).",
         merged="Nothing merged.", omitted="Individual hairs, stitching, paper fibres.",
         thinElements="Page ruling and finger lines are 0.10 mm texture strokes; the page sheet is a 0.13 mm detail line.",
         hatching="Sweater and sleeves: vertical lines at 0.5 mm spacing (knit wales).",
         styles="Body 0.25 mm, detail 0.13 mm, texture 0.10 mm.", overlays="One region per shape as authored."),
    dict(silhouette="Voll gegliederte Zeichnung mit allen v1-Details: gerippte Strickstriche, Jeansnähte, Fingertrennungen, Ohr, Auge, Schnürsenkel, Bankleisten, Notizbuchseiten mit Lineatur; 26,8 mm hoch.",
         retained="Alle 56 v1-Formen, neu an die v2-Regionen gebunden (Bank und Becher tragen jetzt eigene Regionen).",
         merged="Nichts verschmolzen.", omitted="Einzelne Haare, Stiche, Papierfasern.",
         thinElements="Seitenlineatur und Fingerlinien sind 0,10-mm-Texturstriche; die Seite ist eine 0,13-mm-Detaillinie.",
         hatching="Pullover und Ärmel: senkrechte Linien im Abstand 0,5 mm (Maschenstäbchen).",
         styles="Körper 0,25 mm, Detail 0,13 mm, Textur 0,10 mm.", overlays="Eine Region je Form wie gezeichnet."))

anchors = copy.deepcopy(a["anchors"])
anchors += [{"id": "nostrils", "bone": "head", "x": 0.0284, "y": -0.0983, "kind": "emitter", "title": T("Nostrils", "Nasenöffnungen")},
            {"id": "crown", "bone": "head", "x": 0.165, "y": 0.0, "kind": "emitter", "title": T("Crown of the head", "Scheitel")}]
for an in anchors:
    if an["id"] == "mouth":
        an["title"] = T("Mouth opening", "Mundöffnung")

SK = dubois(61, 1.68)
mats = copy.deepcopy(a["materials"])
mats += [{"id": "cotton-underwear", "title": T("Cotton underwear and T-shirt", "Baumwollwäsche und T-Shirt"), "emissivity": 0.93, "solarAbsorptivity": 0.5, "specificHeatJKgK": 1300, "source": "authored-estimate"},
         {"id": "cotton-sock", "title": T("Cotton ankle socks", "Baumwoll-Sneakersocken"), "emissivity": 0.93, "solarAbsorptivity": 0.6, "specificHeatJKgK": 1300, "source": "authored-estimate"}]
capS = 13000
f_sw = fcl_of(0.04 + 0.08 + 0.35)
sw_l = [garment("underwear", "Bra, panties", "BH, Slip", 0.04, "cotton-underwear", 0.0006, 80, f_sw),
        garment("tee", "Cotton T-shirt", "Baumwoll-T-Shirt", 0.08, "cotton-underwear", 0.0008, 180, f_sw),
        garment("sweater", "Thick knitted sweater with cowl", "Dicker Strickpullover mit Rollkragen", 0.35, "knit-wool", 0.006, 700, f_sw)]
f_je = fcl_of(0.24)
je_l = [garment("jeans", "Straight denim jeans (thick trousers)", "Gerade Jeans (dicke Hose)", 0.24, "denim", 0.0012, 500, f_je)]
f_sh = 1.0
sh_l = [garment("socks", "Ankle socks", "Sneakersocken", 0.02, "cotton-sock", 0.0015, 90, f_sh),
        garment("sneaker", "Canvas sneaker, thin sole", "Canvas-Turnschuh, dünne Sohle", 0.02, "canvas-sneaker", 0.003, 500, f_sh)]
hair_l = [{"id": "hair", "title": T("Scalp hair with bun", "Kopfhaar mit Dutt"), "kind": "hair", "material": "hair", "thicknessM": 0.025, "resistanceM2KW": 0.07, "vapourResistanceM2PaW": 11.2, "heatCapacityJM2K": 400, "source": "authored-estimate"}]
cap = lambda L, b=capS: b + sum(l["heatCapacityJM2K"] for l in L)
regions = [
    body_region("face-skin", "Face and neck skin", "Haut von Gesicht und Hals", "skin", "skin", SK * 0.05, 1.0, "head", 0.06, 0.05, 0.2, "sphere", 0.5, capS, [], ref="dubois-1916"),
    body_region("hair", "Hair and bun", "Haar und Dutt", "hair", "hair", SK * 0.024, 1.0, "head", 0.03, 0.024, 0.18, "sphere", 0.8, cap(hair_l, 8000), hair_l),
    body_region("hand-skin", "Bare hands", "Unbedeckte Hände", "skin", "skin", SK * 0.05, 1.0, "hand", 0.04, 0.05, 0.1, "cylinder", 0.2, capS, [], ref="dubois-1916"),
    body_region("sweater", "Layered clothing on torso, neck and arms", "Geschichtete Kleidung an Rumpf, Hals und Armen", "clothing", "knit-wool", SK * 0.511, f_sw, "trunk", 0.57, 0.511, 0.3, "vertical-plate", 0.45, cap(sw_l), sw_l),
    body_region("jeans", "Jeans over the legs", "Jeans über den Beinen", "clothing", "denim", SK * 0.295, f_je, "leg", 0.27, 0.295, 0.13, "cylinder", 0.15, cap(je_l), je_l),
    body_region("shoes", "Socks and sneakers", "Socken und Turnschuhe", "surface", "canvas-sneaker", SK * 0.07, f_sh, "foot", 0.03, 0.07, 0.1, "horizontal-plate-up", 0.1, cap(sh_l), sh_l),
    {"id": "notebook", "title": T("Notebook", "Notizbuch"), "kind": "surface", "model": "passive", "material": "paper", "areaM2": 0.12, "characteristicLengthM": 0.2, "geometry": "vertical-plate", "skyViewFactor": 0.4, "layers": [], "heatCapacityJM2K": 1500, "source": "authored-estimate", "reference": "iso-9920-2007"},
    {"id": "bench", "title": T("Park bench fragment", "Parkbankausschnitt"), "kind": "surface", "model": "passive", "material": "wood-bench", "areaM2": 0.9, "characteristicLengthM": 0.5, "geometry": "horizontal-plate-up", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": 9000, "source": "authored-estimate", "reference": "iso-9920-2007"},
    {"id": "cup", "title": T("Paper cup", "Pappbecher"), "kind": "surface", "model": "passive", "material": "paper-cup", "areaM2": 0.035, "characteristicLengthM": 0.1, "geometry": "cylinder", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": 1200, "source": "authored-estimate", "reference": "iso-9920-2007"},
]
assert abs(sum(r["body"]["metabolicShare"] for r in regions if "body" in r) - 1) < 1e-9 and abs(sum(r["body"]["skinFraction"] for r in regions if "body" in r) - 1) < 1e-9
pat = lambda i, t, r, v, p1, e, p2, route, en, de, ref="icrp-66-1994": {"id": i, "title": T(en, de), "rateBpm": r, "tidalVolumeL": v, "inhaleFraction": p1, "pauseAfterInhaleFraction": 0.04 if e < 0.46 else 0.02, "exhaleFraction": e, "pauseAfterExhaleFraction": p2, "route": route, "source": "authored-estimate", "reference": ref}
respiration = {"noseEmitter": "nose-breath", "mouthEmitter": "mouth-breath", "exhaledTempC": 34, "exhaledRelativeHumidity": 0.95,
               "patterns": [pat("reading-rest", None, 14, 0.45, 0.38, 0.45, 0.13, "nose", "Seated quiet reading", "Ruhiges Sitzen und Lesen"),
                            pat("light-activity", None, 20, 0.8, 0.44, 0.5, 0.04, "nose-and-mouth", "Light activity: leaning, rising", "Leichte Aktivität: Vorbeugen, Aufstehen"),
                            pat("standing-rest", None, 15, 0.5, 0.4, 0.47, 0.09, "nose", "Quiet standing", "Ruhiges Stehen")],
               "motion": [{"bone": "torso", "angleDeg": 0.5, "dy": 0.003}], "source": "authored-estimate", "reference": "gupta-2010"}
for p in respiration["patterns"]:
    tot = p["inhaleFraction"] + p["pauseAfterInhaleFraction"] + p["exhaleFraction"] + p["pauseAfterExhaleFraction"]
    p["pauseAfterExhaleFraction"] = round(p["pauseAfterExhaleFraction"] + 1 - tot, 4)

thermal = OrderedDict()
thermal["environment"] = {"airTempC": 14, "meanRadiantTempC": 16, "skyTempC": 6, "groundTempC": 14, "relativeHumidity": 0.6, "windMS": 0.3, "windDirectionDeg": 180, "solarWM2": 150, "diffuseFraction": 0.3, "pressurePa": 101325}
thermal["baseMet"] = 1.0
thermal["body"] = {"species": "human", "massKg": 61, "skinAreaM2": SK, "basalMetabolismW": 65, "metWM2": 58.15, "coreSetPointC": 36.8, "skinSetPointC": 34.1, "sweating": True, "panting": False,
                   "maxSkinBloodFlowLM2H": 90, "minSkinBloodFlowLM2H": 1.0, "source": "authored-estimate", "reference": "gagge-1971"}
thermal["respiration"] = respiration
thermal["operating"] = []
thermal["passengers"] = []
thermal["regions"] = regions
thermal["emitters"] = breath_emitters(plume_region="sweater", plume_d=0.22, noseD=0.009, mouthD=0.0118)
thermal["assumptions"] = [
    T("Skin area 1.692 m2 after DuBois and DuBois (1916) for 61 kg and 1.68 m; skin fractions authored (rule-of-nines proportions): face 0.05, scalp 0.024, hands 0.05, torso, neck and arms 0.511, legs 0.295, feet 0.07.",
      "Hautfläche 1,692 m2 nach DuBois und DuBois (1916) für 61 kg und 1,68 m; Hautanteile geschätzt (Neunerregel): Gesicht 0,05, Kopfhaut 0,024, Hände 0,05, Rumpf, Hals und Arme 0,511, Beine 0,295, Füße 0,07."),
    T("Garment clo values follow the ISO 9920 / ASHRAE tables as authored estimates (underwear 0.04, T-shirt 0.08, thick sweater 0.35, thick trousers 0.24, ankle socks 0.02, thin-soled shoes 0.02 clo); resistances are referred to the outer area (times f_cl = 1 + 1.97 I_cl).",
      "Die clo-Werte der Kleidungsstücke folgen den Tabellen von ISO 9920 / ASHRAE als Schätzwerte (Unterwäsche 0,04, T-Shirt 0,08, dicker Pullover 0,35, dicke Hose 0,24, Sneakersocken 0,02, dünnsohlige Schuhe 0,02 clo); Widerstände sind auf die Außenfläche bezogen (mal f_cl = 1 + 1,97 I_cl)."),
    T("Seated reading is 1.0 met; the bench is a passive timber surface that conducts into the jeans and the sweater at the contact fractions of each pose (seat 15 degrees C).",
      "Ruhiges Sitzen und Lesen sind 1,0 met; die Bank ist eine passive Holzfläche, die an den Kontaktanteilen jeder Haltung in Jeans und Pullover leitet (Sitz 15 Grad C)."),
    T("Breathing patterns are authored estimates guided by quiet adult values (about 0.45 L at 14 per minute for a woman) and ICRP 66 light-activity ventilation; the cup is passive (no hot-drink heat release modelled).",
      "Atemmuster sind geschätzt nach ruhigen Erwachsenenwerten (etwa 0,45 L bei 14 je Minute für eine Frau) und der Lüftung bei leichter Aktivität nach ICRP 66; der Becher ist passiv (keine Wärmeabgabe eines Heißgetränks modelliert)."),
]

base_sun = {"face-skin": 0.7, "hair": 0.7, "hand-skin": 0.6, "sweater": 0.6, "jeans": 0.5, "shoes": 0.5, "notebook": 0.7, "bench": 0.6, "cup": 0.6}
spec = {  # pose: (met, breathing, jeans exposed/contact, sweater exposed/contact, bench exposed)
    "seated-upright": (1.0, "reading-rest", (0.85, 0.15), (1.0, 0.0), 0.8),
    "reclined-reading": (1.0, "reading-rest", (0.85, 0.15), (0.86, 0.14), 0.75),
    "reaching-for-cup": (1.2, "reading-rest", (0.85, 0.15), (1.0, 0.0), 0.8),
    "leaning-forward": (1.6, "light-activity", (0.92, 0.08), (1.0, 0.0), 0.88),
    "rising": (2.2, "light-activity", (1.0, 0.0), (1.0, 0.0), 1.0),
    "standing": (1.4, "standing-rest", (1.0, 0.0), (1.0, 0.0), 1.0),
}
poses = copy.deepcopy(a["poses"])
for p in poses:
    met, br, (je, jc), (sw, sc_), be = spec[p["id"]]
    regs = {}
    for r in regions:
        rid = r["id"]
        b = {"exposedFraction": 1.0, "sunlitFraction": base_sun[rid]}
        if rid == "jeans":
            b["exposedFraction"] = je
            if jc: b["contact"] = {"fraction": jc, "tempC": 15}
        if rid == "sweater":
            b["exposedFraction"] = sw
            if sc_: b["contact"] = {"fraction": sc_, "tempC": 15}
        if rid == "bench":
            b["exposedFraction"] = be
        regs[rid] = b
    p["thermal"] = {"activityMet": met, "breathing": br, "regions": regs, "emitters": {"body-plume": 0.6 if met < 1.5 else 1.0}}

clips = copy.deepcopy(a["clips"])
for c in clips:
    c["events"] = [e for e in c["events"] if e["kind"] not in ("inhale", "exhale")]
    c["tracks"] = [t for t in c["tracks"] if not t["target"].startswith("emitter:breath")]
    if c["id"] == "page-turn":
        c["tracks"].append({"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 3.6, "v": 0}]})
    if c["id"] == "stand-up":
        c["tracks"].append({"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 1.0, "v": 1}, {"t": 3.2, "v": 2}, {"t": 4, "v": 2}]})
    if c["id"] == "lean-back":
        c["tracks"].append({"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 5, "v": 0}]})

prov = copy.deepcopy(a["provenance"])
have = {r["id"] for r in prov["references"]}
for k in ("gagge-1971", "mifflin-1990", "icrp-66-1994", "tartarini-2020", "gupta-2010"):
    if k not in have:
        prov["references"].append({"id": k, "citation": REFS[k]} if k in REFS else None)
prov["references"] = [r for r in prov["references"] if r]
prov["author"] = "Semio staffage content worker CV-H1"
prov["notes"] = T(a["provenance"]["notes"]["en"] + " Version 2: four individually drawn scale representations, layered clothing regions, bench and cup as own passive regions, DuBois skin area, two-node physiology and derived breathing.",
                  a["provenance"]["notes"]["de"] + " Version 2: vier einzeln gezeichnete Maßstabsdarstellungen, geschichtete Kleidungsregionen, Bank und Becher als eigene passive Regionen, DuBois-Hautfläche, Zweiknotenphysiologie und abgeleitete Atmung.")
pres = OrderedDict([("defaultPose", a["presentation"]["defaultPose"]), ("defaultClip", a["presentation"]["defaultClip"]), ("defaultScale", 100), ("viewpoints", a["presentation"]["viewpoints"])])
syms = [position_symbol("seat", "seated-person-marker", ("Seated person marker", "Sitzende Person als Marke"),
                        ("Position dot for crowded overview plans; no direction arrow because a seated person does not travel.", "Positionspunkt für belebte Übersichtspläne; ohne Richtungspfeil, da eine sitzende Person nicht läuft."),
                        "The dot marks the true position of the bench; its 0.9 mm diameter is a paper size and carries no body dimension.", "Der Punkt markiert die wahre Position der Bank; sein Durchmesser von 0,9 mm ist eine Papiergröße und trägt kein Körpermaß.", arrow=False)]
finish(aid, src, a, R, dict(provenance=prov, materials=mats, anchors=anchors, symbols=syms, poses=poses, clips=clips, thermal=thermal, presentation=pres))
