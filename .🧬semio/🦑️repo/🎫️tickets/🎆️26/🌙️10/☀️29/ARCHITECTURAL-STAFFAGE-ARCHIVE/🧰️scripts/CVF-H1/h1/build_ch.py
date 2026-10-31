"""running-child v1 -> v2 (CV-H1)."""
import copy
from lib import *
import ch_geom

aid = "running-child"
src = json.load(open(f"v1/{aid}.json"))
a = src["asset"]
R = ch_geom.build(a, next(p for p in a["poses"] if p["id"] == "standing-ready"))

rm = {}
for s in a["geometry"]["elevation"]:
    if s.get("region") == "limb-skin":
        i = s["id"]
        rm[i] = "leg-skin" if (i.startswith("thigh") or i.startswith("shank")) else "arm-skin"
R["50"] = {"status": "authored", "denominator": 50, "validRange": {"minDenominator": 20, "maxDenominator": 75},
           "pens": {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": 0.1, "ground": 0.3}, "hatches": [],
           "geometry": {"elevation": v1_to_50(a["geometry"]["elevation"], None, region_map=rm)}}

R["500"]["decisions"] = decisions(
    dict(silhouette="One composed mass: T-shirt, neck, head and backpack as a single torso silhouette with the pack bulge behind the shoulders, the near arm in two bars, plus two leg columns with shoes; 2.3 mm tall, 0.6 mm wide.",
         retained="The big child head (5.8 heads to the body), the backpack bulge, shorts and bare legs as two columns, shoes at true size, exact 1.15 m height; the legs carry the sprint rhythm.",
         merged="T-shirt, neck, face, hair and backpack in one filled form (four regions listed); shorts and bare thigh in the thigh shape, bare shin and sock in the shank shape; hand into the forearm shape.",
         omitted="The far arm (hidden behind the body in most phases), hands as separate shapes, collar, straps, ear, face, forefoot joint.",
         thinElements="Child shins (45 mm), arms (56 mm) and shoes (40-60 mm) are thinner than any printable filled feature (0.2 mm = 100 mm), so they are drawn as single pen strokes of 0.14 mm (70 mm line width) along their bone axes; fingers are dropped. Thighs (100 mm) stay filled, widened by 10 %.",
         hatching="None.",
         styles="Mid-grey torso mass, lighter thighs, shins, arms and shoes as pen strokes of 0.14 mm; body pen 0.13 mm; no detail pen.",
         overlays="The torso form shows the area-weighted value of T-shirt, face, hair and pack (merged); thigh and shank forms show shorts with leg skin and leg skin with socks (merged)."),
    dict(silhouette="Eine zusammengesetzte Masse: T-Shirt, Hals, Kopf und Rucksack als eine Rumpfsilhouette mit der Rucksackwölbung hinter den Schultern, der nahe Arm in zwei Balken, dazu zwei Beinsäulen mit Schuhen; 2,3 mm hoch, 0,6 mm breit.",
         retained="Der große Kinderkopf (5,8 Kopfhöhen), die Rucksackwölbung, der nahe Arm mit Ellbogen (der Armschwung beim Rennen), Shorts und nackte Beine als zwei Säulen, Schuhe in wahrer Größe, genau 1,15 m Höhe; die Beine tragen den Sprintrhythmus.",
         merged="T-Shirt, Hals, Gesicht, Haar und Rucksack in einer gefüllten Form (vier Regionen aufgelistet); Shorts und nackter Oberschenkel in der Oberschenkelform, nackter Unterschenkel und Socke in der Unterschenkelform; Hand in der Unterarmform.",
         omitted="Der ferne Arm (in den meisten Phasen vom Körper verdeckt), Hände als eigene Formen, Kragen, Riemen, Ohr, Gesicht, Vorfußgelenk.",
         thinElements="Die 45 mm dicken Kinderschienbeine, die 56 mm dicken Arme und die 40-60 mm hohen Schuhe sind dünner als jede druckbare gefüllte Einzelheit (0,2 mm = 100 mm); sie werden als einzelne Federstriche von 0,14 mm (70 mm Linienbreite) entlang ihrer Knochenachsen gezeichnet; Finger entfallen. Oberschenkel (100 mm) bleiben gefüllt, um 10 % verbreitert.",
         hatching="Keine.",
         styles="Mittelgrauer Rumpf, hellere Oberschenkel, Unterschenkel, Arme und Schuhe als Federstriche von 0,14 mm; Körperstift 0,13 mm; kein Detailstift.",
         overlays="Die Rumpfform zeigt den flächengewichteten Wert von T-Shirt, Gesicht, Haar und Rucksack (verschmolzen); Oberschenkel- und Unterschenkelform zeigen Shorts mit Beinhaut und Beinhaut mit Socken (verschmolzen)."))
R["200"]["decisions"] = decisions(
    dict(silhouette="Separated masses: hair, face, collar, T-shirt with hem, backpack with one strap line, both bare arms with sleeve caps, shorts, bare thighs and shins, shoes; 5.8 mm tall.",
         retained="Elbow and knee flexion, hands as skin blobs (the running arm swing), sleeve caps and shorts as separate cloth masses, forefoot piece so the toe-off stays above ground.",
         merged="Neck into collar and head, thumb into the hand blob, sock into the shin.",
         omitted="Fingers, ear, eye, shoe stripe, T-shirt print, pack pocket, sock line.",
         thinElements="Backpack strap is the only hairline (0.1 mm); fingers dropped; arms drawn at true 55 mm width, which stays above the minimum feature.",
         hatching="None.",
         styles="Body pen 0.18 mm, detail 0.10 mm for the strap and the hem; skin light, T-shirt mid, shorts darker, shoes dark.",
         overlays="Arm skin, leg skin, T-shirt, shorts, shoes, pack, face and hair each carry one region."),
    dict(silhouette="Getrennte Massen: Haar, Gesicht, Kragen, T-Shirt mit Saum, Rucksack mit einer Riemenlinie, beide nackten Arme mit Ärmelkappen, Shorts, nackte Ober- und Unterschenkel, Schuhe; 5,8 mm hoch.",
         retained="Ellbogen- und Kniebeugung, Hände als Hautflecken (der Armschwung beim Rennen), Ärmelkappen und Shorts als eigene Stoffmassen, Vorfußstück, damit der Abdruck über dem Boden bleibt.",
         merged="Hals in Kragen und Kopf, Daumen in die Handform, Socke in den Unterschenkel.",
         omitted="Finger, Ohr, Auge, Schuhstreifen, T-Shirt-Aufdruck, Rucksacktasche, Sockenlinie.",
         thinElements="Der Rucksackriemen ist die einzige Haarlinie (0,1 mm); Finger entfallen; die Arme sind in wahrer Breite von 55 mm gezeichnet, was über dem Mindestmaß bleibt.",
         hatching="Keine.",
         styles="Körperstift 0,18 mm, Detailstift 0,10 mm für Riemen und Saum; Haut hell, T-Shirt mittel, Shorts dunkler, Schuhe dunkel.",
         overlays="Armhaut, Beinhaut, T-Shirt, Shorts, Schuhe, Rucksack, Gesicht und Haar tragen je eine Region."))
R["100"]["decisions"] = decisions(
    dict(silhouette="Articulated child: short sleeves over bare arms with mitten hands and thumb, T-shirt with print and hem line, shorts, bare knees, socks and sneakers with stripe and sole line, backpack with strap and pocket line, short hair with fringe and ear; 11.5 mm tall.",
         retained="Face profile with nose and ear, T-shirt print block, sock cuff line, shoe stripe, both backpack strap and pocket.",
         merged="Eye, brow and mouth line are not drawn; finger groups form one mitten.",
         omitted="Eye (0.1 mm), shoe laces, hair strands, finger lines.",
         thinElements="Fingers merged into a mitten with its own thumb; sock cuff and pocket as 0.1 mm lines.",
         hatching="None: bare skin and cotton stay plain tone so that thermal colours stay readable on the small figure.",
         styles="Body pen 0.2 mm, detail 0.1 mm; skin light, T-shirt mid, shorts darker, shoes and hair dark.",
         overlays="As in 1:200; the print block and hem share the T-shirt colour, the cuff line the sock colour."),
    dict(silhouette="Gegliedertes Kind: kurze Ärmel über nackten Armen mit Fäustlingshänden und Daumen, T-Shirt mit Aufdruck und Saumlinie, Shorts, nackte Knie, Socken und Turnschuhe mit Streifen und Sohlenlinie, Rucksack mit Riemen und Taschenlinie, kurzes Haar mit Ohr; 11,5 mm hoch.",
         retained="Gesichtsprofil mit Nase und Ohr, Aufdruckblock, Sockenbündchenlinie, Schuhstreifen, Rucksackriemen und Tasche.",
         merged="Auge, Braue und Mundlinie werden nicht gezeichnet; die Fingergruppen bilden einen Fäustling.",
         omitted="Auge (0,1 mm), Schnürsenkel, Haarsträhnen, Fingerlinien.",
         thinElements="Finger zu einem Fäustling mit eigenem Daumen verschmolzen; Sockenbündchen und Tasche als 0,1-mm-Linien.",
         hatching="Keine: nackte Haut und Baumwolle bleiben flächig, damit die thermischen Farben auf der kleinen Figur lesbar bleiben.",
         styles="Körperstift 0,2 mm, Detail 0,1 mm; Haut hell, T-Shirt mittel, Shorts dunkler, Schuhe und Haar dunkel.",
         overlays="Wie 1:200; Aufdruck und Saum teilen die T-Shirt-Farbe, die Bündchenlinie die Sockenfarbe."))
R["50"]["decisions"] = decisions(
    dict(silhouette="Fully articulated child with every v1 detail: three finger separations and thumb, ear, eye, mouth line, hair fringe, T-shirt print and hem, sock line, shoe stripe and sole; 23 mm tall.",
         retained="All 47 v1 shapes, re-bound to v2 regions (arm skin and leg skin are now separate regions).", merged="Nothing merged.",
         omitted="Individual hairs, stitching, fabric weave.", thinElements="Finger lines and fringe are 0.10 mm texture strokes.", hatching="None.",
         styles="Body 0.25 mm, detail 0.13 mm, texture 0.10 mm.", overlays="One region per shape as authored."),
    dict(silhouette="Voll gegliedertes Kind mit allen v1-Details: drei Fingertrennungen und Daumen, Ohr, Auge, Mundlinie, Haarfransen, T-Shirt-Aufdruck und Saum, Sockenlinie, Schuhstreifen und Sohle; 23 mm hoch.",
         retained="Alle 47 v1-Formen, neu an die v2-Regionen gebunden (Armhaut und Beinhaut sind jetzt getrennte Regionen).", merged="Nichts verschmolzen.",
         omitted="Einzelne Haare, Stiche, Gewebe.", thinElements="Fingerlinien und Fransen sind 0,10-mm-Texturstriche.", hatching="Keine.",
         styles="Körper 0,25 mm, Detail 0,13 mm, Textur 0,10 mm.", overlays="Eine Region je Form wie gezeichnet."))

anchors = copy.deepcopy(a["anchors"])
for an in anchors:
    if an["id"] == "mouth":
        an["title"] = T("Mouth opening", "Mundöffnung")
anchors += [{"id": "nostrils", "bone": "head", "x": 0.0255, "y": -0.0875, "kind": "emitter", "title": T("Nostrils", "Nasenöffnungen")},
            {"id": "crown", "bone": "head", "x": 0.145, "y": 0.0, "kind": "emitter", "title": T("Crown of the head", "Scheitel")}]

BODY_KG = 20.7
SK = dubois(BODY_KG, 1.15)
mats = copy.deepcopy(a["materials"])
mats += [{"id": "cotton-underwear", "title": T("Cotton briefs", "Baumwollslip"), "emissivity": 0.93, "solarAbsorptivity": 0.5, "specificHeatJKgK": 1300, "source": "authored-estimate"},
         {"id": "cotton-short", "title": T("Cotton shorts", "Baumwollshorts"), "emissivity": 0.93, "solarAbsorptivity": 0.6, "specificHeatJKgK": 1300, "source": "authored-estimate"},
         {"id": "cotton-sock", "title": T("Cotton ankle socks", "Baumwoll-Sneakersocken"), "emissivity": 0.93, "solarAbsorptivity": 0.6, "specificHeatJKgK": 1300, "source": "authored-estimate"}]
capS = 13000
f_ts = fcl_of(0.04 + 0.08)
ts_l = [garment("briefs", "Cotton briefs", "Baumwollslip", 0.04, "cotton-underwear", 0.0006, 70, f_ts), garment("tshirt", "Cotton T-shirt", "Baumwoll-T-Shirt", 0.08, "cotton-jersey", 0.0008, 150, f_ts)]
f_sh = fcl_of(0.08)
sh_l = [garment("shorts", "Cotton walking shorts", "Baumwoll-Bermudas", 0.08, "cotton-short", 0.0009, 200, f_sh)]
so_l = [garment("socks", "Ankle socks", "Sneakersocken", 0.02, "cotton-sock", 0.0015, 90, 1.0)]
shoe_l = [garment("sneaker", "Canvas sneaker, thin sole", "Canvas-Turnschuh, dünne Sohle", 0.02, "canvas-sneaker", 0.003, 450, 1.0)]
hair_l = [{"id": "hair", "title": T("Short scalp hair", "Kurzes Kopfhaar"), "kind": "hair", "material": "hair", "thicknessM": 0.012, "resistanceM2KW": 0.035, "vapourResistanceM2PaW": 5.6, "heatCapacityJM2K": 200, "source": "authored-estimate"}]
cap = lambda L, b=capS: b + sum(l["heatCapacityJM2K"] for l in L)
regions = [
    body_region("face-skin", "Face and neck skin", "Haut von Gesicht und Hals", "skin", "skin", SK * 0.05, 1.0, "head", 0.09, 0.05, 0.16, "sphere", 0.5, capS, [], ref="dubois-1916"),
    body_region("hair", "Short hair", "Kurzes Haar", "hair", "hair", SK * 0.04, 1.0, "head", 0.02, 0.04, 0.15, "sphere", 0.8, cap(hair_l, 8000), hair_l),
    body_region("arm-skin", "Bare arms and hands", "Nackte Arme und Hände", "skin", "skin", SK * 0.10, 1.0, "arm", 0.06, 0.10, 0.06, "cylinder", 0.3, capS, [], ref="dubois-1916"),
    body_region("leg-skin", "Bare legs", "Nackte Beine", "skin", "skin", SK * 0.21, 1.0, "leg", 0.17, 0.21, 0.07, "cylinder", 0.15, capS, [], ref="dubois-1916"),
    body_region("tshirt", "Briefs and T-shirt on trunk and upper arms", "Slip und T-Shirt an Rumpf und Oberarmen", "clothing", "cotton-jersey", SK * 0.37, f_ts, "trunk", 0.54, 0.37, 0.2, "vertical-plate", 0.45, cap(ts_l), ts_l),
    body_region("shorts", "Shorts over the hips and thighs", "Shorts über Hüfte und Oberschenkeln", "clothing", "cotton-short", SK * 0.12, f_sh, "leg", 0.09, 0.12, 0.1, "cylinder", 0.15, cap(sh_l), sh_l),
    body_region("socks", "Ankle socks", "Sneakersocken", "clothing", "cotton-sock", SK * 0.03, 1.0, "foot", 0.01, 0.03, 0.05, "cylinder", 0.1, cap(so_l), so_l),
    body_region("shoes", "Sneakers", "Turnschuhe", "surface", "canvas-sneaker", SK * 0.08, 1.0, "foot", 0.02, 0.08, 0.1, "horizontal-plate-up", 0.1, cap(shoe_l), shoe_l),
    {"id": "backpack", "title": T("Small backpack", "Kleiner Rucksack"), "kind": "surface", "model": "passive", "material": "nylon-pack", "areaM2": 0.12, "characteristicLengthM": 0.2, "geometry": "vertical-plate", "skyViewFactor": 0.4, "layers": [], "heatCapacityJM2K": 1500, "source": "authored-estimate", "reference": "iso-9920-2007"},
]
assert abs(sum(r["body"]["metabolicShare"] for r in regions if "body" in r) - 1) < 1e-9 and abs(sum(r["body"]["skinFraction"] for r in regions if "body" in r) - 1) < 1e-9
mk = lambda i, en, de, r, v, p1, p2, e, p3, route: {"id": i, "title": T(en, de), "rateBpm": r, "tidalVolumeL": v, "inhaleFraction": p1, "pauseAfterInhaleFraction": p2, "exhaleFraction": e, "pauseAfterExhaleFraction": p3, "route": route, "source": "authored-estimate", "reference": "rowland-2005"}
respiration = {"noseEmitter": "nose-breath", "mouthEmitter": "mouth-breath", "exhaledTempC": 34, "exhaledRelativeHumidity": 0.95,
               "patterns": [mk("rest-child", "Child at rest, standing", "Kind in Ruhe, stehend", 22, 0.17, 0.38, 0.04, 0.45, 0.13, "nose"),
                            mk("running-child", "Child sprinting", "Kind beim Sprint", 45, 0.65, 0.47, 0.0, 0.5, 0.03, "nose-and-mouth"),
                            mk("jump-effort", "Child jumping from a crouch", "Kind beim Absprung aus der Hocke", 38, 0.55, 0.46, 0.0, 0.5, 0.04, "nose-and-mouth")],
               "motion": [{"bone": "torso", "angleDeg": 0.7, "dy": 0.002}], "source": "authored-estimate", "reference": "rowland-2005"}
thermal = OrderedDict()
thermal["environment"] = {"airTempC": 18, "meanRadiantTempC": 20, "skyTempC": 8, "groundTempC": 28, "relativeHumidity": 0.55, "windMS": 0.5, "windDirectionDeg": 180, "solarWM2": 400, "diffuseFraction": 0.25, "pressurePa": 101325}
thermal["baseMet"] = 1.4
thermal["body"] = {"species": "human", "massKg": BODY_KG, "skinAreaM2": SK, "basalMetabolismW": 47, "metWM2": 58.15, "coreSetPointC": 37.0, "skinSetPointC": 34.1, "sweating": True, "panting": False,
                   "maxSkinBloodFlowLM2H": 90, "minSkinBloodFlowLM2H": 1.0, "source": "authored-estimate", "reference": "schofield-1985"}
thermal["respiration"] = respiration
thermal["operating"] = []
thermal["passengers"] = []
thermal["regions"] = regions
thermal["emitters"] = breath_emitters(plume_region="tshirt", plume_d=0.15, noseD=0.0075, mouthD=0.0098)
thermal["assumptions"] = [
    T("Skin area 0.83 m2 after DuBois and DuBois (1916) for a seven-year-old of 20.7 kg body mass and 1.15 m; the 22.1 kg of the asset adds T-shirt, shorts, shoes and 1.0 kg backpack contents. Skin fractions authored for a child (larger head): face 0.05, scalp 0.04, arms 0.10, legs 0.21, trunk and upper arms 0.37, shorts area 0.12, feet 0.11.",
      "Hautfläche 0,83 m2 nach DuBois und DuBois (1916) für ein siebenjähriges Kind mit 20,7 kg Körpermasse und 1,15 m; die 22,1 kg des Objekts enthalten T-Shirt, Shorts, Schuhe und 1,0 kg Rucksackinhalt. Hautanteile für ein Kind (größerer Kopf) geschätzt: Gesicht 0,05, Kopfhaut 0,04, Arme 0,10, Beine 0,21, Rumpf und Oberarme 0,37, Shorts-Fläche 0,12, Füße 0,11."),
    T("Summer clothing: briefs 0.04, T-shirt 0.08, walking shorts 0.08, ankle socks 0.02 and thin-soled shoes 0.02 clo (ISO 9920 / ASHRAE garment tables, authored estimates); resistances are referred to the outer area.",
      "Sommerkleidung: Slip 0,04, T-Shirt 0,08, Bermudas 0,08, Sneakersocken 0,02 und dünnsohlige Schuhe 0,02 clo (Kleidungstabellen ISO 9920 / ASHRAE, Schätzwerte); Widerstände sind auf die Außenfläche bezogen."),
    T("Child breathing: about 8 mL/kg at rest (0.17 L at 22 per minute), sprinting 45 per minute with 0.65 L (about 29 L/min, about 1.4 L/min per kg), jumping 38 per minute with 0.55 L; all authored estimates guided by general paediatric exercise physiology (Rowland 2005), not measured data.",
      "Kinderatmung: etwa 8 mL/kg in Ruhe (0,17 L bei 22 je Minute), Sprint 45 je Minute mit 0,65 L (etwa 29 L/min, etwa 1,4 L/min je kg), Springen 38 je Minute mit 0,55 L; alles Schätzwerte nach allgemeiner Kinder-Belastungsphysiologie (Rowland 2005), keine Messwerte."),
    T("Running is 6 met in this model (a child's sprint, scaled by body surface); the child's resting basal rate 47 W follows the Schofield equation for boys of 3-10 years (guided).",
      "Rennen entspricht in diesem Modell 6 met (Kindersprint, nach Körperoberfläche skaliert); der Grundumsatz 47 W folgt der Schofield-Gleichung für Jungen von 3-10 Jahren (Richtwert)."),
]
rid = [r["id"] for r in regions]
spec = {"standing-ready": (1.4, "rest-child"), "mid-stride": (6, "running-child"), "airborne": (6, "running-child"), "toe-off": (6, "running-child"), "landing": (6, "running-child"),
        "crouch": (3, "jump-effort"), "jump-takeoff": (7, "jump-effort"), "jump-apex": (4, "jump-effort"), "flight-1": (6, "running-child"), "flight-2": (6, "running-child"), "flight-3": (6, "running-child")}
poses = copy.deepcopy(a["poses"])
for p in poses:
    met, br = spec[p["id"]]
    regs = {}
    folded = p["id"] in ("crouch", "jump-takeoff")
    for r in rid:
        b = {"exposedFraction": 1.0, "sunlitFraction": 0.6}
        if r == "tshirt": b["exposedFraction"] = 0.85
        if r == "leg-skin" and folded: b["exposedFraction"] = 0.85
        if r == "shoes" and p["contacts"]:
            b["contact"] = {"fraction": 0.3 if len(p["contacts"]) >= 4 else 0.15, "tempC": 28}
        if r == "backpack": b["sunlitFraction"] = 0.4
        regs[r] = b
    p["thermal"] = {"activityMet": met, "breathing": br, "regions": regs, "emitters": {"body-plume": 0.6 if met < 2 else 1.0}}

clips = copy.deepcopy(a["clips"])
for c in clips:
    c["events"] = [e for e in c["events"] if e["kind"] not in ("inhale", "exhale")]
    c["tracks"] = [t for t in c["tracks"] if not t["target"].startswith("emitter:breath")]
    if c["id"] == "sprint-cycle":
        c["tracks"] += [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 6}, {"t": 0.52, "v": 6}]},
                        {"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 1}, {"t": 0.52, "v": 1}]}]
    else:
        c["tracks"] += [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 0.5, "v": 2}, {"t": 2.0979, "v": 1}]}]

prov = copy.deepcopy(a["provenance"])
have = {r["id"] for r in prov["references"]}
extra = dict(REFS)
extra["rowland-2005"] = "Rowland, T. W. (2005). Children's Exercise Physiology, 2nd ed. Human Kinetics; general paediatric exercise and ventilation background (values authored, not taken from tables)."
extra["schofield-1985"] = "Schofield, W. N. (1985). Predicting basal metabolic rate, new standards and review of previous work. Human Nutrition: Clinical Nutrition 39 (Suppl 1), 5-41; used as guidance for the child's basal rate."
for k in ("gagge-1971", "tartarini-2020", "rowland-2005", "schofield-1985", "gupta-2010", "dubois-1916", "iso-9920-2007"):
    if k not in have and k in extra:
        prov["references"].append({"id": k, "citation": extra[k]})
prov["author"] = "Semio staffage content worker CV-H1"
prov["notes"] = T(a["provenance"]["notes"]["en"] + " Version 2: four individually drawn scale representations, split arm and leg skin regions, summer clothing layers, child two-node physiology and age-specific breathing patterns.",
                  a["provenance"]["notes"]["de"] + " Version 2: vier einzeln gezeichnete Maßstabsdarstellungen, getrennte Arm- und Beinhautregionen, Sommerkleidungsschichten, Zweiknotenphysiologie des Kindes und altersgerechte Atemmuster.")
dims = dict(a["dimensions"]); dims["widthM"] = 0.29
pres = OrderedDict([("defaultPose", a["presentation"]["defaultPose"]), ("defaultClip", a["presentation"]["defaultClip"]), ("defaultScale", 100), ("viewpoints", a["presentation"]["viewpoints"])])
syms = [position_symbol("pelvis", "running-direction-marker", ("Running child marker", "Rennendes Kind als Marke"),
                        ("Position dot with a direction arrow for crowded overview plans at 1:500 and smaller.", "Positionspunkt mit Richtungspfeil für belebte Übersichtspläne ab 1:500."),
                        "The dot marks the true standing position; its 0.9 mm diameter and the arrow are paper sizes and carry no body dimension.", "Der Punkt markiert die wahre Standposition; sein Durchmesser von 0,9 mm und der Pfeil sind Papiergrößen und tragen kein Körpermaß.")]
finish(aid, src, a, R, dict(provenance=prov, dimensions=dims, materials=mats, anchors=anchors, symbols=syms, poses=poses, clips=clips, thermal=thermal, presentation=pres))
