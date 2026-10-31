"""walking-commuter v1 -> v2 (CV-H1). Reads v1/walking-commuter.json, writes the catalog asset."""
import copy
from collections import OrderedDict
from lib import *
import wc_geom

aid = "walking-commuter"
src = json.load(open(f"v1/{aid}.json"))
a = src["asset"]
standing = next(p for p in a["poses"] if p["id"] == "standing")
R = wc_geom.build(a, standing)

# ---- 1:50: the v1 detail drawing, re-bound
hm = {"coat": "wool", "sleeve-l": "wool", "sleeve-r": "wool", "forearm-l": "wool", "forearm-r": "wool"}
R["50"] = {"status": "authored", "denominator": 50, "validRange": {"minDenominator": 20, "maxDenominator": 75},
           "pens": {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": 0.1, "ground": 0.3},
           "hatches": [{"id": "wool", "title": T("Wool overcoat hatching", "Schraffur Wollmantel"), "pattern": "lines", "angleDeg": 65, "spacingMm": 0.5, "penMm": 0.1}],
           "geometry": {"elevation": v1_to_50(a["geometry"]["elevation"], hm)}}

R["500"]["decisions"] = decisions(
    dict(silhouette="One composed mass: overcoat, scarf and head merged into a single torso silhouette (head 0.22 m = 0.44 mm), plus two leg columns and the bag; the figure is 3.6 mm tall and 0.8 mm wide.",
         retained="Overcoat hem at knee height, the head bump, the bag block at hip height, both legs with shoes (they carry the walking rhythm), exact 1.78 m height and 0.40 m width.",
         merged="Coat, scarf, neck, face, hair and both hands into one filled form listing all five regions; thigh and shank stay separate bones but touch end to end.",
         omitted="Both arms (hanging swing amplitude 0.15 m = 0.3 mm is below one pen width), neck, face details, lapels, seams, buttons, bag strap, toe joint.",
         thinElements="The bag strap (10 mm) and fingers (15 mm) are dropped, never thickened; the shoe is drawn at true size.",
         hatching="None.",
         styles="Fill reads as a mid-tone coat mass, dark shoes; line pen 0.13 mm, no detail pen; light mode: dark line on white, dark mode: inverted.",
         overlays="Thermal and structural overlays colour the merged silhouette with the area-weighted value of coat, face, hair, scarf and hands (labelled merged); legs and bag keep their own region."),
    dict(silhouette="Eine zusammengesetzte Masse: Mantel, Schal und Kopf als eine Rumpfsilhouette (Kopf 0,22 m = 0,44 mm) sowie zwei Beinsäulen und die Tasche; die Figur ist 3,6 mm hoch und 0,8 mm breit.",
         retained="Mantelsaum in Kniehöhe, Kopfbuckel, Taschenblock auf Hüfthöhe, beide Beine mit Schuhen (sie tragen den Gehrhythmus), genau 1,78 m Höhe und 0,40 m Breite.",
         merged="Mantel, Schal, Hals, Gesicht, Haar und beide Hände zu einer gefüllten Form mit allen fünf Regionen; Ober- und Unterschenkel bleiben getrennte Knochen, stoßen aber Ende an Ende.",
         omitted="Beide Arme (die Pendelamplitude von 0,15 m = 0,3 mm liegt unter einer Strichbreite), Hals, Gesichtsdetails, Revers, Nähte, Knöpfe, Taschenriemen, Zehengelenk.",
         thinElements="Der Taschenriemen (10 mm) und die Finger (15 mm) entfallen, sie werden nie verdickt; der Schuh ist in wahrer Größe gezeichnet.",
         hatching="Keine.",
         styles="Die Füllung liest sich als mittlere Mantelmasse, dunkle Schuhe; Strichstärke 0,13 mm, kein Detailstift; hell: dunkle Linie auf Weiß, dunkel: invertiert.",
         overlays="Thermische und statische Überlagerungen färben die verschmolzene Silhouette mit dem flächengewichteten Wert von Mantel, Gesicht, Haar, Schal und Händen (als verschmolzen gekennzeichnet); Beine und Tasche behalten ihre eigene Region."))
R["200"]["decisions"] = decisions(
    dict(silhouette="Main masses separated: hair cap, face, scarf, overcoat with hem and bag, both arms as sleeve plus forearm plus hand, both legs as trousers plus shoe; 8.9 mm tall.",
         retained="Elbow and knee joints (separate masses so the walk cycle reads), hand blobs as skin regions, shoe and toe piece, the coat front edge and the bag flap as the only inner lines.",
         merged="Neck into scarf and head, thumb into the hand blob, lapel and pocket into the coat, strap into one hairline.",
         omitted="Fingers, thumb, ear, eye, buttons, trouser crease, shoe sole line, weave texture.",
         thinElements="Bag strap kept as one detail hairline (0.1 mm), fingers dropped; the 10 mm scarf folds are merged into the scarf mass.",
         hatching="None; fill tones carry the material.",
         styles="Body pen 0.18 mm for masses, detail pen 0.10 mm for the coat front and the bag flap; light fills for skin, mid for coat, dark for shoes and hair.",
         overlays="Each mass carries one region (coat, trousers, shoes, scarf, hair, face, hands, bag); the two sleeves share the coat value, hands show the hand-skin value."),
    dict(silhouette="Hauptmassen getrennt: Haarkappe, Gesicht, Schal, Mantel mit Saum und Tasche, beide Arme als Ärmel mit Unterarm und Hand, beide Beine als Hose mit Schuh; 8,9 mm hoch.",
         retained="Ellbogen und Knie (getrennte Massen, damit der Gehzyklus lesbar bleibt), Handflecken als Hautregionen, Schuh und Zehenstück, Mantelvorderkante und Taschenklappe als einzige Innenlinien.",
         merged="Hals in Schal und Kopf, Daumen in die Handform, Revers und Tasche in den Mantel, Riemen zu einer Haarlinie.",
         omitted="Finger, Daumen, Ohr, Auge, Knöpfe, Bügelfalte, Sohlenlinie, Webstruktur.",
         thinElements="Der Taschenriemen bleibt als eine Detail-Haarlinie (0,1 mm), Finger entfallen; die 10 mm breiten Schalfalten gehen in der Schalmasse auf.",
         hatching="Keine; die Füllungstöne tragen das Material.",
         styles="Körperstift 0,18 mm für Massen, Detailstift 0,10 mm für Mantelvorderkante und Taschenklappe; helle Füllung für Haut, mittlere für Mantel, dunkle für Schuhe und Haar.",
         overlays="Jede Masse trägt eine Region (Mantel, Hose, Schuhe, Schal, Haar, Gesicht, Hände, Tasche); die zwei Ärmel teilen den Mantelwert, die Hände zeigen den Handhautwert."))
R["100"]["decisions"] = decisions(
    dict(silhouette="Articulated clothing: tapered sleeves with elbow caps, a coat with lapel, front edge and pocket, trousers with crease, laced shoes with a flexing forefoot; 17.8 mm tall.",
         retained="Hair mass with ear, face profile with nose and chin, scarf, mitten hands with a separate thumb, shoe sole line, bag with flap, buckle and both strap runs.",
         merged="Eyebrow, eye, mouth line, shoe laces and coat buttons are not drawn; the three finger groups are one mitten outline.",
         omitted="Eye (0.14 mm), buttons (0.16 mm), laces, weave texture, back vent, hem seam.",
         thinElements="Fingers merged into a mitten with the thumb as its own shape; strap kept as 0.1 mm lines; laces dropped.",
         hatching="Wool coat only: fine parallel lines at 65 degrees, 0.6 mm spacing, 0.10 mm pen - restrained material cue, trousers and bag stay plain tone.",
         styles="Body pen 0.2 mm, detail 0.1 mm; coat mid-grey with hatch, trousers darker, shoes dark, skin light.",
         overlays="Regions as in 1:200; coat sleeves, coat body and lapel share the coat colour, mitten and thumb the hand colour."),
    dict(silhouette="Gegliederte Kleidung: verjüngte Ärmel mit Ellbogenkappen, Mantel mit Revers, Vorderkante und Tasche, Hose mit Bügelfalte, geschnürte Schuhe mit beweglichem Vorfuß; 17,8 mm hoch.",
         retained="Haarmasse mit Ohr, Gesichtsprofil mit Nase und Kinn, Schal, Fäustlingshände mit eigenem Daumen, Sohlenlinie, Tasche mit Klappe, Schnalle und beiden Riemenläufen.",
         merged="Braue, Auge, Mundlinie, Schnürsenkel und Mantelknöpfe werden nicht gezeichnet; die drei Fingergruppen sind eine Fäustlingskontur.",
         omitted="Auge (0,14 mm), Knöpfe (0,16 mm), Schnürsenkel, Webstruktur, Rückenschlitz, Saumnaht.",
         thinElements="Finger zu einem Fäustling verschmolzen, der Daumen als eigene Form; Riemen als 0,1-mm-Linien; Schnürsenkel entfallen.",
         hatching="Nur der Wollmantel: feine Parallellinien im Winkel 65 Grad, 0,6 mm Abstand, 0,10 mm Stift - zurückhaltender Materialhinweis, Hose und Tasche bleiben flächig.",
         styles="Körperstift 0,2 mm, Detail 0,1 mm; Mantel mittelgrau mit Schraffur, Hose dunkler, Schuhe dunkel, Haut hell.",
         overlays="Regionen wie 1:200; Mantelärmel, Mantelkörper und Revers teilen die Mantelfarbe, Fäustling und Daumen die Handfarbe."))
R["50"]["decisions"] = decisions(
    dict(silhouette="Fully articulated drawing: coat with lapels, buttons and vent, sleeve cuffs, three finger separations and thumb, ear, eye, brow and mouth line, shoe with sole, laces and cuff; 35.6 mm tall.",
         retained="Everything of the v1 detail drawing (60 shapes) re-bound to the v2 regions, including the weave strokes of the coat and the laces.",
         merged="Nothing merged except the far-side details hidden behind the body.",
         omitted="Pores, individual hairs, stitch holes, fabric fibres (below 0.1 mm at this scale).",
         thinElements="Finger separations are 0.10 mm texture lines; trouser crease and laces 0.10 mm; all within the printable range of the default profile.",
         hatching="Wool coat and sleeves: parallel lines at 65 degrees, 0.5 mm spacing; the weave strokes stay as texture pen.",
         styles="Body 0.25 mm, detail 0.13 mm, texture 0.10 mm; tones as in 1:100.",
         overlays="One region per shape as authored; details inherit the region of the garment they sit on."),
    dict(silhouette="Voll gegliederte Zeichnung: Mantel mit Revers, Knöpfen und Schlitz, Ärmelmanschetten, drei Fingertrennungen und Daumen, Ohr, Auge, Braue und Mundlinie, Schuh mit Sohle, Schnürung und Hosensaum; 35,6 mm hoch.",
         retained="Alles aus der v1-Detailzeichnung (60 Formen), neu an die v2-Regionen gebunden, einschließlich Webstrichen des Mantels und Schnürsenkeln.",
         merged="Nichts verschmolzen außer den vom Körper verdeckten Details der abgewandten Seite.",
         omitted="Poren, einzelne Haare, Stichlöcher, Gewebefasern (bei diesem Maßstab unter 0,1 mm).",
         thinElements="Fingertrennungen sind 0,10-mm-Texturlinien; Bügelfalte und Schnürsenkel 0,10 mm; alles im druckbaren Bereich des Standardprofils.",
         hatching="Wollmantel und Ärmel: Parallellinien im Winkel 65 Grad, 0,5 mm Abstand; die Webstriche bleiben im Texturstift.",
         styles="Körper 0,25 mm, Detail 0,13 mm, Textur 0,10 mm; Töne wie 1:100.",
         overlays="Eine Region je Form wie gezeichnet; Details erben die Region des Kleidungsstücks, auf dem sie sitzen."))

# ---- anchors
anchors = copy.deepcopy(a["anchors"])
for an in anchors:
    if an["id"] == "mouth":
        an["title"] = T("Mouth opening", "Mundöffnung")
anchors += [
    {"id": "nostrils", "bone": "head", "x": 0.03, "y": -0.104, "kind": "emitter", "title": T("Nostrils", "Nasenöffnungen")},
    {"id": "crown", "bone": "head", "x": 0.175, "y": 0.0, "kind": "emitter", "title": T("Crown of the head", "Scheitel")},
]

# ---- thermal
SK = dubois(72, 1.78)
mats = copy.deepcopy(a["materials"])
mats += [
    {"id": "cotton-underwear", "title": T("Cotton undershirt and briefs", "Baumwollunterwäsche"), "emissivity": 0.93, "solarAbsorptivity": 0.5, "specificHeatJKgK": 1300, "source": "authored-estimate"},
    {"id": "cotton-shirt", "title": T("Cotton dress shirt", "Baumwollhemd"), "emissivity": 0.93, "solarAbsorptivity": 0.5, "specificHeatJKgK": 1300, "source": "authored-estimate"},
    {"id": "wool-suit", "title": T("Wool suit jacket", "Wollsakko"), "emissivity": 0.92, "solarAbsorptivity": 0.8, "specificHeatJKgK": 1360, "source": "authored-estimate"},
    {"id": "cotton-sock", "title": T("Cotton socks", "Baumwollsocken"), "emissivity": 0.93, "solarAbsorptivity": 0.6, "specificHeatJKgK": 1300, "source": "authored-estimate"},
]
cap_skin = 13000
f_coat = fcl_of(0.08 + 0.25 + 0.35 + 0.60)
coat_layers = [garment("tee", "Cotton undershirt", "Baumwollunterhemd", 0.08, "cotton-underwear", 0.0008, 180, f_coat),
               garment("shirt", "Dress shirt, long sleeve", "Hemd, langarm", 0.25, "cotton-shirt", 0.0012, 160, f_coat),
               garment("jacket", "Suit jacket", "Sakko", 0.35, "wool-suit", 0.003, 610, f_coat),
               garment("overcoat", "Wool overcoat", "Wollmantel", 0.60, "wool-coat", 0.006, 1200, f_coat)]
f_tr = fcl_of(0.04 + 0.24)
tr_layers = [garment("briefs", "Cotton briefs", "Baumwollslip", 0.04, "cotton-underwear", 0.0008, 100, f_tr),
             garment("trousers", "Wool trousers, straight, thick", "Wollhose, gerade, dick", 0.24, "wool-trouser", 0.0025, 480, f_tr)]
f_sc = fcl_of(0.20)
sc_layers = [garment("scarf", "Thick wool scarf", "Dicker Wollschal", 0.20, "wool-scarf", 0.01, 500, f_sc)]
f_sh = 1.0
sh_layers = [garment("socks", "Cotton socks, calf length", "Baumwollsocken, wadenlang", 0.03, "cotton-sock", 0.002, 120, f_sh),
             garment("shoe", "Leather shoe, thick sole", "Lederschuh, dicke Sohle", 0.04, "leather-shoe", 0.004, 900, f_sh)]
hair_layers = [{"id": "hair", "title": T("Scalp hair", "Kopfhaar"), "kind": "hair", "material": "hair", "thicknessM": 0.02, "resistanceM2KW": 0.06, "vapourResistanceM2PaW": 9.6,
                "heatCapacityJM2K": 300, "source": "authored-estimate"}]


def cap(layers, base=cap_skin):
    return base + sum(l["heatCapacityJM2K"] for l in layers)


regions = [
    body_region("face-skin", "Face and neck skin", "Haut von Gesicht und Hals", "skin", "skin", SK * 0.05, 1.0, "head", 0.06, 0.05, 0.2, "sphere", 0.5, cap_skin, [], ref="dubois-1916"),
    body_region("hair", "Hair", "Haar", "hair", "hair", SK * 0.024, 1.0, "head", 0.03, 0.024, 0.18, "sphere", 0.8, cap(hair_layers, 8000), hair_layers),
    body_region("hand-skin", "Bare hands", "Unbedeckte Hände", "skin", "skin", SK * 0.05, 1.0, "hand", 0.04, 0.05, 0.1, "cylinder", 0.2, cap_skin, [], ref="dubois-1916"),
    body_region("coat", "Layered clothing on torso and arms", "Geschichtete Kleidung an Rumpf und Armen", "clothing", "wool-coat", SK * 0.49, f_coat, "trunk", 0.56, 0.49, 0.3, "vertical-plate", 0.45, cap(coat_layers), coat_layers),
    body_region("scarf", "Scarf over the neck", "Schal über dem Hals", "clothing", "wool-scarf", SK * 0.021, f_sc, "head", 0.01, 0.021, 0.1, "cylinder", 0.5, cap(sc_layers), sc_layers),
    body_region("trousers", "Trousers over the legs", "Hose über den Beinen", "clothing", "wool-trouser", SK * 0.295, f_tr, "leg", 0.27, 0.295, 0.13, "cylinder", 0.1, cap(tr_layers), tr_layers),
    body_region("shoes", "Socks and shoes", "Socken und Schuhe", "surface", "leather-shoe", SK * 0.07, f_sh, "foot", 0.03, 0.07, 0.1, "horizontal-plate-up", 0.1, cap(sh_layers), sh_layers),
    {"id": "bag-leather", "title": T("Leather bag", "Ledertasche"), "kind": "surface", "model": "passive", "material": "leather-bag", "areaM2": 0.28, "characteristicLengthM": 0.25,
     "geometry": "vertical-plate", "skyViewFactor": 0.3, "layers": [], "heatCapacityJM2K": 2500, "source": "authored-estimate", "reference": "iso-9920-2007"},
]
assert abs(sum(r["body"]["metabolicShare"] for r in regions if "body" in r) - 1) < 1e-9
assert abs(sum(r["body"]["skinFraction"] for r in regions if "body" in r) - 1) < 1e-9

respiration = {
    "noseEmitter": "nose-breath", "mouthEmitter": "mouth-breath", "exhaledTempC": 34, "exhaledRelativeHumidity": 0.95,
    "patterns": [
        {"id": "rest-standing", "title": T("Quiet standing", "Ruhiges Stehen"), "rateBpm": 13, "tidalVolumeL": 0.5, "inhaleFraction": 0.38,
         "pauseAfterInhaleFraction": 0.04, "exhaleFraction": 0.45, "pauseAfterExhaleFraction": 0.13, "route": "nose", "source": "authored-estimate", "reference": "icrp-66-1994"},
        {"id": "walking", "title": T("Walking at 4 km/h", "Gehen mit 4 km/h"), "rateBpm": 20, "tidalVolumeL": 1.2, "inhaleFraction": 0.45,
         "pauseAfterInhaleFraction": 0.02, "exhaleFraction": 0.5, "pauseAfterExhaleFraction": 0.03, "route": "nose-and-mouth", "source": "authored-estimate", "reference": "icrp-66-1994"},
    ],
    "motion": [{"bone": "torso", "angleDeg": 0.5, "dy": 0.003}], "source": "authored-estimate", "reference": "gupta-2010"}

thermal = OrderedDict()
thermal["environment"] = {"airTempC": 8, "meanRadiantTempC": 8, "skyTempC": -4, "groundTempC": 8, "relativeHumidity": 0.75, "windMS": 1, "windDirectionDeg": 180,
                          "solarWM2": 0, "diffuseFraction": 1, "pressurePa": 101325}
thermal["baseMet"] = 1.2
thermal["body"] = {"species": "human", "massKg": 72, "skinAreaM2": SK, "basalMetabolismW": 80, "metWM2": 58.15, "coreSetPointC": 36.8, "skinSetPointC": 34.1,
                   "sweating": True, "panting": False, "maxSkinBloodFlowLM2H": 90, "minSkinBloodFlowLM2H": 1.0, "source": "authored-estimate", "reference": "gagge-1971"}
thermal["respiration"] = respiration
thermal["operating"] = []
thermal["passengers"] = []
thermal["regions"] = regions
thermal["emitters"] = breath_emitters(plume_region="coat")
thermal["assumptions"] = [
    T("Skin area 1.894 m2 after DuBois and DuBois (1916) for 72 kg and 1.78 m; the regions carry skin fractions 0.05 face, 0.024 scalp, 0.021 neck, 0.05 hands, 0.49 trunk and arms, 0.295 legs, 0.07 feet (Rule-of-nines proportions, authored).",
      "Hautfläche 1,894 m2 nach DuBois und DuBois (1916) für 72 kg und 1,78 m; die Regionen tragen die Hautanteile 0,05 Gesicht, 0,024 Kopfhaut, 0,021 Hals, 0,05 Hände, 0,49 Rumpf und Arme, 0,295 Beine, 0,07 Füße (Neunerregel-Proportionen, geschätzt)."),
    T("Clothing layers are garment values in clo after the ISO 9920 / ASHRAE tables (1 clo = 0.155 m2K/W, enclosed air included), but the table digits are authored estimates; resistances are referred to the outer area (intrinsic value times f_cl = 1 + 1.97 I_cl); vapour resistance R/(0.0165 K/Pa x 0.38).",
      "Kleidungsschichten sind Kleidungsstückwerte in clo nach den Tabellen von ISO 9920 / ASHRAE (1 clo = 0,155 m2K/W, eingeschlossene Luft enthalten), die Tabellenwerte selbst sind geschätzt; Widerstände sind auf die Außenfläche bezogen (Eigenwert mal f_cl = 1 + 1,97 I_cl); Dampfwiderstand R/(0,0165 K/Pa x 0,38)."),
    T("Region heat capacity = skin node 13 kJ/(m2K) (10 % of the body mass at 3.5 kJ/(kgK) over the skin area, after the Gagge two-node idea) plus the areal heat capacity of the garments.",
      "Wärmekapazität der Region = Hautknoten 13 kJ/(m2K) (10 % der Körpermasse bei 3,5 kJ/(kgK) über der Hautfläche, nach dem Zweiknotengedanken von Gagge) plus die flächenbezogene Wärmekapazität der Kleidung."),
    T("Walking at 4 km/h is about 2.8 met; breathing patterns are authored estimates guided by quiet adult values (about 0.5 L, 12-15 per minute) and ICRP 66 light-exercise ventilation (about 25 L/min).",
      "Gehen mit 4 km/h entspricht etwa 2,8 met; die Atemmuster sind geschätzt nach ruhigen Erwachsenenwerten (etwa 0,5 L, 12-15 je Minute) und der Lüftung bei leichter Belastung nach ICRP 66 (etwa 25 L/min)."),
    T("The shoulder bag is a passive leather surface of 0.28 m2 that is not part of the body model.",
      "Die Umhängetasche ist eine passive Lederfläche von 0,28 m2 und nicht Teil des Körpermodells."),
]

# ---- poses
met = {"standing": 1.2, "check-watch": 1.3}
rid = [r["id"] for r in regions]


def binding(pid):
    act = met.get(pid, 2.8)
    brk = "rest-standing" if pid in met else "walking"
    regs = {}
    for r in rid:
        regs[r] = {"exposedFraction": {"coat": 0.92, "bag-leather": 0.85, "hair": 1.0, "hand-skin": 1.0}.get(r, 1.0), "sunlitFraction": 0}
    return {"activityMet": act, "breathing": brk, "regions": regs, "emitters": {"body-plume": 1.0 if act >= 2 else 0.6}}


poses = copy.deepcopy(a["poses"])
for p in poses:
    p["thermal"] = binding(p["id"])

# ---- clips
clips = copy.deepcopy(a["clips"])
for c in clips:
    c["events"] = [e for e in c["events"] if e["kind"] not in ("inhale", "exhale")]
    c["tracks"] = [t for t in c["tracks"] if not t["target"].startswith("emitter:breath")]
    if c["id"] == "walk-cycle":
        c["tracks"] += [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 2.8}, {"t": 1.2, "v": 2.8}]},
                        {"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 1}, {"t": 1.2, "v": 1}]}]
    if c["id"] == "glance-at-watch":
        c["tracks"] += [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 4, "v": 0}]}]
    if c["id"] == "idle-breathing":
        c["phases"] = [{"id": "quiet-standing", "fromS": 0, "toS": 4, "title": T("Quiet standing at the crossing", "Ruhiges Stehen an der Kreuzung")}]
        c["tracks"] += [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 4, "v": 0}]}]
        c["description"] = T("Standing at a crossing with the resting breathing pattern (13 per minute) and slight chest motion derived from the pattern.",
                             "Warten an einer Kreuzung mit dem Ruheatemmuster (13 je Minute) und leichter, aus dem Muster abgeleiteter Brustbewegung.")

# ---- provenance
prov = copy.deepcopy(a["provenance"])
have = {r["id"] for r in prov["references"]}
for k in ("gagge-1971", "mifflin-1990", "icrp-66-1994", "tartarini-2020"):
    if k not in have:
        prov["references"].append({"id": k, "citation": REFS[k]})
prov["author"] = "Semio staffage content worker CV-H1"
prov["notes"] = T(a["provenance"]["notes"]["en"] + " Version 2: four individually drawn scale representations (1:500, 1:200, 1:100, 1:50), layered clothing regions, DuBois skin area, two-node physiology and derived breathing.",
                  a["provenance"]["notes"]["de"] + " Version 2: vier einzeln gezeichnete Maßstabsdarstellungen (1:500, 1:200, 1:100, 1:50), geschichtete Kleidungsregionen, DuBois-Hautfläche, Zweiknotenphysiologie und abgeleitete Atmung.")

asset = OrderedDict()
for k in ("id", "category", "title", "summary", "tags"):
    asset[k] = a[k]
asset["provenance"] = prov
asset["dimensions"] = a["dimensions"]
asset["materials"] = mats
asset["rig"] = a["rig"]
asset["anchors"] = anchors
asset["representations"] = OrderedDict((k, R[k]) for k in ("500", "200", "100", "50"))
asset["symbols"] = [position_symbol("pelvis", "walking-direction-marker",
                                    ("Walking person marker", "Gehende Person als Marke"),
                                    ("Position dot with a direction arrow for crowded overview plans at 1:500 and smaller.", "Positionspunkt mit Richtungspfeil für belebte Übersichtspläne ab 1:500."),
                                    "The dot marks the true standing position of the person (insertion point at the model origin); its 0.9 mm diameter and the arrow length are paper sizes and carry no body dimension.",
                                    "Der Punkt markiert die wahre Standposition der Person (Einfügepunkt im Modellursprung); sein Durchmesser von 0,9 mm und die Pfeillänge sind Papiergrößen und tragen kein Körpermaß.")]
asset["poses"] = poses
asset["clips"] = clips
asset["thermal"] = thermal
asset["structure"] = a["structure"]
pres = dict(a["presentation"]); pres["defaultScale"] = 100
asset["presentation"] = OrderedDict([("defaultPose", pres["defaultPose"]), ("defaultClip", pres["defaultClip"]), ("defaultScale", 100), ("viewpoints", pres["viewpoints"])])
asset["exports"] = a["exports"]
asset["completion"] = "authored"
doc = OrderedDict([("$schema", src["$schema"]), ("version", "semio.staffage/v2"), ("asset", asset)])
save(aid, doc)
print("wrote", aid, {k: len(v["geometry"]["elevation"]) for k, v in R.items()})
