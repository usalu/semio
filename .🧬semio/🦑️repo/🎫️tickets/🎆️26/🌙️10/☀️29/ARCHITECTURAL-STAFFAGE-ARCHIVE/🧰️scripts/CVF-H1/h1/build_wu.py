"""wheelchair-user v1 -> v2 (CV-H1)."""
import copy
from lib import *
import wu_geom

aid = "wheelchair-user"
src = json.load(open(f"v1/{aid}.json"))
a = src["asset"]
R = wu_geom.build(a, next(p for p in a["poses"] if p["id"] == "seated-level"))

R["50"] = {"status": "authored", "denominator": 50, "validRange": {"minDenominator": 20, "maxDenominator": 75},
           "pens": {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": 0.1, "ground": 0.3}, "hatches": [],
           "geometry": {"elevation": v1_to_50(a["geometry"]["elevation"])}}

R["500"]["decisions"] = decisions(
    dict(silhouette="Person and chair as one composed seated mass: hoodie, hood, face and beanie as one torso silhouette, near arm as sleeve plus forearm-with-glove, thigh, shin and shoe columns, and the chair reduced to what reads at 3 mm: one ring for each rear wheel, a disc for the castor, three frame strokes; 2.9 mm tall, 2.2 mm long.",
         retained="The big rear wheel ring (0.61 m) that identifies the chair, the castor, the upright back with push handle, the footplate, the seated posture with knees at 90 degrees, exact 1.45 m height and 1.09 m length.",
         merged="Tyre, rim and push rim into one ring line listing tyres and frame metal; hoodie, face and beanie into one form; seat rail with cushion into one stroke; forearm, glove and bare fingertips into one shape.",
         omitted="Wheel spokes (12 spokes of 3 mm cannot be printed and would turn the ring into a disc), the hub, brake lever, castor fork and housing, back sling, far arm and far leg (coincident with the near ones), shoe toe joint.",
         thinElements="Tubes of 24 mm and the 23 mm tyre are drawn as single pen strokes (0.13 mm = 65 mm line width) instead of outlines; the ring is placed at the tyre centre line (0.298 m) so that the 0.61 m wheel stays within 12 mm; nothing is drawn as a fill thinner than the pen.",
         hatching="None.",
         styles="Dark ring and strokes for the chair, mid-grey hoodie mass, light jeans, dark shoes; pen 0.13 mm; no detail pen; ground line 0.18 mm.",
         overlays="The torso form shows the area-weighted value of hoodie, face and beanie (merged); the ring shows tyres and frame metal merged; cushion is merged into the seat stroke; jeans, shoes and castor have their own colour. The wheel rotation is not visible at this scale."),
    dict(silhouette="Person und Stuhl als eine zusammengesetzte Sitzmasse: Hoodie, Kapuze, Gesicht und Mütze als eine Rumpfsilhouette, nahe Arm als Ärmel plus Unterarm mit Handschuh, Ober-, Unterschenkel und Schuh als Säulen, der Stuhl auf das reduziert, was bei 3 mm lesbar bleibt: ein Ring je Hinterrad, eine Scheibe für die Lenkrolle, drei Rahmenstriche; 2,9 mm hoch, 2,2 mm lang.",
         retained="Der große Hinterradring (0,61 m), der den Stuhl kennzeichnet, die Lenkrolle, die aufrechte Lehne mit Schiebegriff, die Fußplatte, die Sitzhaltung mit Knien bei 90 Grad, genau 1,45 m Höhe und 1,09 m Länge.",
         merged="Reifen, Felge und Greifreifen zu einer Ringlinie mit Reifen und Rahmenmetall; Hoodie, Gesicht und Mütze in einer Form; Sitzrohr mit Kissen in einem Strich; Unterarm, Handschuh und freie Fingerspitzen in einer Form.",
         omitted="Radspeichen (12 Speichen von 3 mm sind nicht druckbar und würden den Ring zur Scheibe machen), die Nabe, Bremshebel, Rollengabel und -gehäuse, Rückenbespannung, ferner Arm und fernes Bein (deckungsgleich mit den nahen), Schuhzehengelenk.",
         thinElements="Rohre von 24 mm und der 23 mm breite Reifen werden als einzelne Federstriche (0,13 mm = 65 mm Linienbreite) statt als Umriss gezeichnet; der Ring liegt auf der Reifenmittellinie (0,298 m), damit das 0,61 m große Rad innerhalb von 12 mm bleibt; nichts wird als Füllung dünner als die Feder gezeichnet.",
         hatching="Keine.",
         styles="Dunkler Ring und dunkle Striche für den Stuhl, mittelgrauer Hoodie, helle Jeans, dunkle Schuhe; Stift 0,13 mm; kein Detailstift; Bodenlinie 0,18 mm.",
         overlays="Die Rumpfform zeigt den flächengewichteten Wert von Hoodie, Gesicht und Mütze (verschmolzen); der Ring zeigt Reifen und Rahmenmetall verschmolzen; das Kissen ist in den Sitzstrich eingeschmolzen; Jeans, Schuhe und Lenkrolle haben eigene Farben. Die Raddrehung ist bei diesem Maßstab nicht sichtbar."))
R["200"]["decisions"] = decisions(
    dict(silhouette="Separated masses: beanie, face, hood, hoodie, both arms (sleeve, forearm, glove blob), jeans seat, thighs, shins, shoes; chair as wheel ring with six spokes and hub, castor disc, seven frame strokes and the cushion; 7.3 mm tall.",
         retained="Wheel rotation (six spokes at 60 degrees so that a quarter turn is visible), castor with one spoke line, push handle, footplate, cushion as the only filled chair part.",
         merged="Tyre and push rim in one ring (they are 20 mm apart = 0.1 mm); glove and bare fingertips in one blob; neck into hood and head; back sling into one stroke.",
         omitted="Six of the twelve spokes, the hand rim as its own ring, brake lever, castor housing, fingers, ear, eye.",
         thinElements="Tubes drawn as 0.15 mm strokes (30 mm) instead of 24 mm outlines, spokes as 0.1 mm lines (20 mm) instead of 3 mm; the six remaining spokes keep a 60-degree pitch whose wheel-turn symmetry is 60 degrees, not the real 30 degrees.",
         hatching="None.",
         styles="Body pen 0.18 mm for the wheel ring and masses, detail 0.1 mm for spokes and the castor spoke, tube strokes 0.15 mm; chair dark, cushion mid-grey.",
         overlays="Ring = tyres, spokes and hub = frame metal, castor = tyres, cushion = cushion; the two sleeves share the hoodie value, the glove blob lists gloves and hand skin."),
    dict(silhouette="Getrennte Massen: Mütze, Gesicht, Kapuze, Hoodie, beide Arme (Ärmel, Unterarm, Handschuhfleck), Jeans am Gesäß, Ober-, Unterschenkel, Schuhe; Stuhl als Radring mit sechs Speichen und Nabe, Lenkrollenscheibe, sieben Rahmenstriche und das Kissen; 7,3 mm hoch.",
         retained="Raddrehung (sechs Speichen im Abstand von 60 Grad, damit eine Vierteldrehung sichtbar wird), Lenkrolle mit einer Speichenlinie, Schiebegriff, Fußplatte, Kissen als einziges gefülltes Stuhlteil.",
         merged="Reifen und Greifreifen in einem Ring (sie liegen 20 mm = 0,1 mm auseinander); Handschuh und freie Fingerspitzen in einem Fleck; Hals in Kapuze und Kopf; Rückenbespannung in einem Strich.",
         omitted="Sechs der zwölf Speichen, der Greifreifen als eigener Ring, Bremshebel, Rollengehäuse, Finger, Ohr, Auge.",
         thinElements="Rohre als 0,15-mm-Striche (30 mm) statt 24-mm-Umrisse, Speichen als 0,1-mm-Linien (20 mm) statt 3 mm; die sechs verbleibenden Speichen haben 60 Grad Teilung, die Drehsymmetrie ist 60 Grad statt der echten 30 Grad.",
         hatching="Keine.",
         styles="Körperstift 0,18 mm für Radring und Massen, Detail 0,1 mm für Speichen und Rollenspeiche, Rohrstriche 0,15 mm; Stuhl dunkel, Kissen mittelgrau.",
         overlays="Ring = Reifen, Speichen und Nabe = Rahmenmetall, Lenkrolle = Reifen, Kissen = Kissen; die zwei Ärmel teilen den Hoodie-Wert, der Handschuhfleck führt Handschuhe und Handhaut."))
R["100"]["decisions"] = decisions(
    dict(silhouette="Deliberately drawn chair: tyre as a 23 mm filled ring, push rim as a ring line inside it, twelve spokes and hub, castor with hub and spoke, tube frame as capsules, footplate, cushion, back sling and brake lever; person with hoodie, pocket, beanie with cuff, mitten gloves with thumb, jeans, shoes; 14.5 mm tall.",
         retained="All twelve spokes (the wheel turns 240 degrees per push, so the spoke pattern shows the rotation), push rim, castor housing, brake lever, hoodie pocket.",
         merged="Glove and bare fingertips in one mitten (regions listed); eye, brow and mouth line not drawn.",
         omitted="Footplate heel loop, hoodie seam, eye, ear detail.",
         thinElements="Tubes keep their true 24 mm diameter as capsules with a 0.1 mm outline (34 mm line); spokes (3 mm) are thickened to 10 mm lines, within tolerance; the push rim (14 mm) is drawn as a 0.1 mm ring line at 0.262 m, 10 mm inside its true centre to keep a 0.17 mm gap to the tyre.",
         hatching="None.",
         styles="Body 0.2 mm, detail 0.1 mm; tyre dark fill, frame mid-grey, hoodie mid, jeans light, glove darker.",
         overlays="Each chair part has its own region (tyres, frame metal, cushion); the hand rim shows the frame metal value."),
    dict(silhouette="Bewusst gezeichneter Stuhl: Reifen als 23 mm breiter gefüllter Ring, Greifreifen als Ringlinie darin, zwölf Speichen und Nabe, Lenkrolle mit Nabe und Speiche, Rohrrahmen als Kapseln, Fußplatte, Kissen, Rückenbespannung und Bremshebel; Person mit Hoodie, Tasche, Mütze mit Umschlag, Fäustlingshandschuhen mit Daumen, Jeans, Schuhen; 14,5 mm hoch.",
         retained="Alle zwölf Speichen (das Rad dreht sich 240 Grad je Schub, das Speichenbild zeigt die Drehung), Greifreifen, Rollengehäuse, Bremshebel, Hoodietasche.",
         merged="Handschuh und freie Fingerspitzen in einem Fäustling (Regionen aufgelistet); Auge, Braue und Mundlinie nicht gezeichnet.",
         omitted="Fersenschlaufe der Fußplatte, Hoodienaht, Auge, Ohrdetail.",
         thinElements="Rohre behalten ihren echten Durchmesser von 24 mm als Kapseln mit 0,1-mm-Umriss (34 mm Linie); Speichen (3 mm) werden auf 10-mm-Linien verdickt, innerhalb der Toleranz; der Greifreifen (14 mm) ist als 0,1-mm-Ringlinie bei 0,262 m gezeichnet, 10 mm innerhalb seiner echten Mitte, damit 0,17 mm Abstand zum Reifen bleiben.",
         hatching="Keine.",
         styles="Körper 0,2 mm, Detail 0,1 mm; Reifen dunkel gefüllt, Rahmen mittelgrau, Hoodie mittel, Jeans hell, Handschuh dunkler.",
         overlays="Jedes Stuhlteil hat eine eigene Region (Reifen, Rahmenmetall, Kissen); der Greifreifen zeigt den Wert des Rahmenmetalls."))
R["50"]["decisions"] = decisions(
    dict(silhouette="Complete drawing of person and chair with every v1 detail: tyre, rim and push rim as separate rings, twelve 3 mm spokes, hub, castor spokes, tube frame as capsules, footplate heel loop, back sling, brake lever; glove fingers and bare fingertips, beanie cuff, hoodie pocket and seam; 29 mm tall.",
         retained="All 75 v1 shapes (re-bound to v2 regions).", merged="Nothing merged.",
         omitted="Spoke nipples, tyre tread, stitching.", thinElements="Spokes at 0.13 mm detail pen (v1 hairlines), fingertip skin as its own shape; the 3 mm spoke is drawn wider than real, within the line weight.",
         hatching="None.", styles="Body 0.25 mm, detail 0.13 mm, texture 0.10 mm.", overlays="One region per shape as authored."),
    dict(silhouette="Vollständige Zeichnung von Person und Stuhl mit allen v1-Details: Reifen, Felge und Greifreifen als getrennte Ringe, zwölf 3-mm-Speichen, Nabe, Rollenspeichen, Rohrrahmen als Kapseln, Fersenschlaufe der Fußplatte, Rückenbespannung, Bremshebel; Handschuhfinger und freie Fingerspitzen, Mützenumschlag, Hoodietasche und -naht; 29 mm hoch.",
         retained="Alle 75 v1-Formen (neu an die v2-Regionen gebunden).", merged="Nichts verschmolzen.",
         omitted="Speichennippel, Reifenprofil, Stiche.", thinElements="Speichen im Detailstift 0,13 mm (v1-Haarlinien), Fingerspitzenhaut als eigene Form; die 3-mm-Speiche wird breiter als echt gezeichnet, im Rahmen der Strichstärke.",
         hatching="Keine.", styles="Körper 0,25 mm, Detail 0,13 mm, Textur 0,10 mm.", overlays="Eine Region je Form wie gezeichnet."))

anchors = copy.deepcopy(a["anchors"])
for an in anchors:
    if an["id"] == "mouth":
        an["title"] = T("Mouth opening", "Mundöffnung")
anchors += [{"id": "nostrils", "bone": "head", "x": 0.0294, "y": -0.1018, "kind": "emitter", "title": T("Nostrils", "Nasenöffnungen")},
            {"id": "crown", "bone": "head", "x": 0.17, "y": 0.0, "kind": "emitter", "title": T("Crown of the head", "Scheitel")}]

SK = dubois(75, 1.75)
mats = copy.deepcopy(a["materials"])
mats += [{"id": "cotton-underwear", "title": T("Cotton T-shirt", "Baumwoll-T-Shirt"), "emissivity": 0.93, "solarAbsorptivity": 0.5, "specificHeatJKgK": 1300, "source": "authored-estimate"},
         {"id": "cotton-sock", "title": T("Cotton socks", "Baumwollsocken"), "emissivity": 0.93, "solarAbsorptivity": 0.6, "specificHeatJKgK": 1300, "source": "authored-estimate"}]
capS = 13000
f_ho = fcl_of(0.08 + 0.25 + 0.34)
ho_l = [garment("tee", "Cotton T-shirt", "Baumwoll-T-Shirt", 0.08, "cotton-underwear", 0.0008, 180, f_ho), garment("fleece", "Light fleece layer", "Leichte Fleeceschicht", 0.25, "fleece", 0.004, 350, f_ho),
        garment("hoodie", "Hooded sweatshirt (hood ring at the neck)", "Kapuzenpullover (Kapuzenring am Hals)", 0.34, "fleece", 0.005, 600, f_ho)]
f_je = fcl_of(0.04 + 0.24)
je_l = [garment("briefs", "Cotton briefs", "Baumwollslip", 0.04, "cotton-underwear", 0.0006, 70, f_je), garment("jeans", "Straight denim jeans (thick trousers)", "Gerade Jeans (dicke Hose)", 0.24, "denim", 0.0012, 500, f_je)]
bean_l = [{"id": "beanie", "title": T("Knitted beanie over hair", "Strickmütze über dem Haar"), "kind": "clothing", "material": "knit-wool", "thicknessM": 0.004, "resistanceM2KW": round(0.2 * CLO * 1.1, 5), "vapourResistanceM2PaW": round(0.2 * CLO * 1.1 / VAP, 1), "heatCapacityJM2K": 400, "source": "authored-estimate", "reference": "iso-9920-2007"}]
glv_l = [garment("gloves", "Lined leather gloves", "Gefütterte Lederhandschuhe", 0.25, "leather-glove", 0.003, 400, 1.2)]
sh_l = [garment("socks", "Cotton socks, calf length", "Baumwollsocken, wadenlang", 0.03, "cotton-sock", 0.002, 120, 1.0), garment("sneaker", "Canvas sneaker, thin sole", "Canvas-Turnschuh, dünne Sohle", 0.02, "canvas-sneaker", 0.003, 500, 1.0)]
cap = lambda L, b=capS: b + sum(l["heatCapacityJM2K"] for l in L)
regions = [
    body_region("face-skin", "Face skin", "Gesichtshaut", "skin", "skin", SK * 0.05, 1.0, "head", 0.07, 0.05, 0.18, "sphere", 0.5, capS, [], ref="dubois-1916"),
    body_region("beanie", "Beanie over scalp and ears", "Mütze über Kopfhaut und Ohren", "clothing", "knit-wool", SK * 0.035, 1.1, "head", 0.03, 0.035, 0.18, "sphere", 0.8, cap(bean_l), bean_l),
    body_region("hand-skin", "Bare fingertips", "Freie Fingerspitzen", "skin", "skin", SK * 0.01, 1.0, "hand", 0.01, 0.01, 0.02, "cylinder", 0.2, capS, [], ref="dubois-1916"),
    body_region("gloves", "Gloves over the hands", "Handschuhe über den Händen", "clothing", "leather-glove", SK * 0.04, 1.2, "hand", 0.03, 0.04, 0.09, "cylinder", 0.2, cap(glv_l), glv_l),
    body_region("hoodie", "Layered clothing on torso, neck and arms", "Geschichtete Kleidung an Rumpf, Hals und Armen", "clothing", "fleece", SK * 0.50, f_ho, "trunk", 0.57, 0.50, 0.3, "vertical-plate", 0.4, cap(ho_l), ho_l),
    body_region("jeans", "Jeans over the legs", "Jeans über den Beinen", "clothing", "denim", SK * 0.295, f_je, "leg", 0.25, 0.295, 0.13, "cylinder", 0.1, cap(je_l), je_l),
    body_region("shoes", "Socks and sneakers", "Socken und Turnschuhe", "surface", "canvas-sneaker", SK * 0.07, 1.0, "foot", 0.04, 0.07, 0.1, "horizontal-plate-up", 0.1, cap(sh_l), sh_l),
    {"id": "frame-metal", "title": T("Aluminium chair frame and rims", "Aluminiumrahmen und Felgen"), "kind": "surface", "model": "passive", "material": "aluminium", "areaM2": 0.55, "characteristicLengthM": 0.03, "geometry": "cylinder", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": 2500, "source": "authored-estimate", "reference": "cooper-2014"},
    {"id": "tyres", "title": T("Pneumatic tyres and castor tyres", "Luftreifen und Lenkrollenreifen"), "kind": "tyre", "model": "passive", "material": "rubber", "areaM2": 0.35, "characteristicLengthM": 0.6, "geometry": "cylinder", "skyViewFactor": 0.4, "layers": [], "heatCapacityJM2K": 6000, "source": "authored-estimate", "reference": "cooper-2014"},
    {"id": "cushion", "title": T("Seat cushion and back sling", "Sitzkissen und Rückenbespannung"), "kind": "surface", "model": "passive", "material": "foam", "areaM2": 0.4, "characteristicLengthM": 0.4, "geometry": "horizontal-plate-up", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": 3000, "source": "authored-estimate", "reference": "cooper-2014"},
]
assert abs(sum(r["body"]["metabolicShare"] for r in regions if "body" in r) - 1) < 1e-9 and abs(sum(r["body"]["skinFraction"] for r in regions if "body" in r) - 1) < 1e-9
mk = lambda i, en, de, r, v, p1, p2, e, p3, route, ref="cooper-2014": {"id": i, "title": T(en, de), "rateBpm": r, "tidalVolumeL": v, "inhaleFraction": p1, "pauseAfterInhaleFraction": p2, "exhaleFraction": e, "pauseAfterExhaleFraction": p3, "route": route, "source": "authored-estimate", "reference": ref}
respiration = {"noseEmitter": "nose-breath", "mouthEmitter": "mouth-breath", "exhaledTempC": 34, "exhaledRelativeHumidity": 0.95,
               "patterns": [mk("rest-seated", "Seated at rest", "Ruhiges Sitzen", 14, 0.5, 0.38, 0.04, 0.45, 0.13, "nose", "icrp-66-1994"),
                            mk("light-seated", "Hands on the rims, starting to push", "Hände an den Reifen, Antrieb beginnt", 17, 0.7, 0.42, 0.03, 0.48, 0.07, "nose"),
                            mk("propulsion", "Hand-rim propulsion on the level, 1.2 m/s", "Greifreifenantrieb in der Ebene, 1,2 m/s", 22, 1.1, 0.46, 0.01, 0.50, 0.03, "nose-and-mouth"),
                            mk("ramp-effort", "Propulsion up a 1:12 ramp", "Antrieb auf einer Rampe 1:12", 26, 1.3, 0.47, 0.0, 0.51, 0.02, "nose-and-mouth")],
               "motion": [{"bone": "torso", "angleDeg": 0.5, "dy": 0.003}], "source": "authored-estimate", "reference": "gupta-2010"}
for p in respiration["patterns"]:
    tot = p["inhaleFraction"] + p["pauseAfterInhaleFraction"] + p["exhaleFraction"] + p["pauseAfterExhaleFraction"]
    p["pauseAfterExhaleFraction"] = round(p["pauseAfterExhaleFraction"] + 1 - tot, 4)
thermal = OrderedDict()
thermal["environment"] = {"airTempC": 10, "meanRadiantTempC": 10, "skyTempC": -2, "groundTempC": 10, "relativeHumidity": 0.7, "windMS": 1, "windDirectionDeg": 180, "solarWM2": 0, "diffuseFraction": 1, "pressurePa": 101325}
thermal["baseMet"] = 1.2
thermal["body"] = {"species": "human", "massKg": 75, "skinAreaM2": SK, "basalMetabolismW": 83, "metWM2": 58.15, "coreSetPointC": 36.8, "skinSetPointC": 34.1, "sweating": True, "panting": False,
                   "maxSkinBloodFlowLM2H": 90, "minSkinBloodFlowLM2H": 1.0, "source": "authored-estimate", "reference": "gagge-1971"}
thermal["respiration"] = respiration
thermal["operating"] = []
thermal["passengers"] = []
thermal["regions"] = regions
thermal["emitters"] = breath_emitters(plume_region="hoodie", plume_d=0.25)
thermal["assumptions"] = [
    T("Skin area 1.927 m2 after DuBois and DuBois (1916) for 75 kg and 1.75 m; skin fractions authored: face 0.05, scalp 0.035, bare fingertips 0.01, gloved hands 0.04, torso, neck and arms 0.50, legs 0.295, feet 0.07. No impaired thermoregulation is modelled (the model is not clinical).",
      "Hautfläche 1,927 m2 nach DuBois und DuBois (1916) für 75 kg und 1,75 m; Hautanteile geschätzt: Gesicht 0,05, Kopfhaut 0,035, freie Fingerspitzen 0,01, behandschuhte Hände 0,04, Rumpf, Hals und Arme 0,50, Beine 0,295, Füße 0,07. Eine gestörte Thermoregulation wird nicht modelliert (das Modell ist nicht klinisch)."),
    T("Garment values are authored estimates after the ISO 9920 / ASHRAE tables (T-shirt 0.08, light fleece 0.25, hooded sweatshirt 0.34, thick trousers 0.24 and briefs 0.04 clo, beanie and gloves authored 0.2 and 0.25 clo); resistances are referred to the outer area (times f_cl = 1 + 1.97 I_cl).",
      "Kleidungswerte sind Schätzwerte nach den Tabellen von ISO 9920 / ASHRAE (T-Shirt 0,08, leichtes Fleece 0,25, Kapuzenpullover 0,34, dicke Hose 0,24 und Slip 0,04 clo, Mütze und Handschuhe geschätzt 0,2 und 0,25 clo); Widerstände sind auf die Außenfläche bezogen (mal f_cl = 1 + 1,97 I_cl)."),
    T("The cushion is a passive foam surface that conducts into the jeans and the hoodie at the contact fractions of each pose (surface about 24 degrees C after sitting); frame, tyres and cushion are passive regions that stay near the ambient and radiant temperatures.",
      "Das Kissen ist eine passive Schaumfläche, die an den Kontaktanteilen jeder Haltung in Jeans und Hoodie leitet (Oberfläche nach dem Sitzen etwa 24 Grad C); Rahmen, Reifen und Kissen sind passive Regionen nahe Umgebungs- und Strahlungstemperatur."),
    T("Hand-rim propulsion at 1.1 to 1.3 m/s is about 2.5 to 3 met (guided by Cooper and Cooper); breathing patterns are authored estimates: 22 per minute with 1.1 L (about 24 L/min) on the level, 26 per minute with 1.3 L (about 34 L/min) on the ramp.",
      "Greifreifenantrieb bei 1,1 bis 1,3 m/s entspricht etwa 2,5 bis 3 met (nach Cooper und Cooper); die Atemmuster sind Schätzwerte: 22 je Minute mit 1,1 L (etwa 24 L/min) in der Ebene, 26 je Minute mit 1,3 L (etwa 34 L/min) auf der Rampe."),
]
spec = {"seated-level": (1.2, "rest-seated"), "hands-on-rims": (1.8, "light-seated"), "push-end": (2.6, "propulsion"), "ramp-ascent": (3.2, "ramp-effort")}
poses = copy.deepcopy(a["poses"])
for p in poses:
    met, br = spec[p["id"]]
    regs = {}
    for r in regions:
        rid = r["id"]
        b = {"exposedFraction": 1.0, "sunlitFraction": 0}
        if rid == "jeans": b["exposedFraction"] = 0.8; b["contact"] = {"fraction": 0.2, "tempC": 24}
        if rid == "hoodie": b["exposedFraction"] = 0.92; b["contact"] = {"fraction": 0.08, "tempC": 22}
        if rid == "cushion": b["exposedFraction"] = 0.6
        if rid == "tyres": b["contact"] = {"fraction": 0.02, "tempC": 10}
        regs[rid] = b
    p["thermal"] = {"activityMet": met, "breathing": br, "regions": regs, "emitters": {"body-plume": 0.6 if met < 2 else 1.0}}
clips = copy.deepcopy(a["clips"])
for c in clips:
    c["events"] = [e for e in c["events"] if e["kind"] not in ("inhale", "exhale")]
    c["tracks"] = [t for t in c["tracks"] if not t["target"].startswith("emitter:breath")]
    if c["id"] == "push-cycle":
        c["tracks"].append({"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 1}, {"t": 1.0, "v": 2}, {"t": 4, "v": 2}]})
    if c["id"] == "ramp-roll":
        c["tracks"].append({"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 3}, {"t": 2, "v": 3}]})
    if c["id"] == "rest-in-chair":
        c["tracks"].append({"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 5, "v": 0}]})
        c["phases"] = [{"id": "quiet-sitting", "fromS": 0, "toS": 5, "title": T("Sitting quietly", "Ruhiges Sitzen")}]
        c["description"] = T("Sitting still in the chair with the resting breathing pattern (14 per minute) and slight chest motion derived from the pattern.", "Ruhiges Sitzen im Stuhl mit dem Ruheatemmuster (14 je Minute) und leichter, aus dem Muster abgeleiteter Brustbewegung.")
prov = copy.deepcopy(a["provenance"])
have = {r["id"] for r in prov["references"]}
for k in ("gagge-1971", "icrp-66-1994", "tartarini-2020", "gupta-2010", "dubois-1916", "iso-9920-2007"):
    if k not in have and k in REFS:
        prov["references"].append({"id": k, "citation": REFS[k]})
prov["author"] = "Semio staffage content worker CV-H1"
prov["notes"] = T(a["provenance"]["notes"]["en"] + " Version 2: four individually drawn scale representations with deliberate decisions for the thin wheel elements, layered clothing regions, passive chair regions, two-node physiology and derived breathing patterns for rest, propulsion and ramp effort.",
                  a["provenance"]["notes"]["de"] + " Version 2: vier einzeln gezeichnete Maßstabsdarstellungen mit bewussten Entscheidungen für die dünnen Radelemente, geschichtete Kleidungsregionen, passive Stuhlregionen, Zweiknotenphysiologie und abgeleitete Atemmuster für Ruhe, Antrieb und Rampenanstieg.")
pres = OrderedDict([("defaultPose", a["presentation"]["defaultPose"]), ("defaultClip", a["presentation"]["defaultClip"]), ("defaultScale", 100), ("viewpoints", a["presentation"]["viewpoints"])])
syms = [position_symbol("frame", "wheelchair-position-marker", ("Wheelchair user marker", "Rollstuhlnutzende Person als Marke"),
                        ("Position dot with a direction arrow for crowded overview plans; the dot marks the rear axle.", "Positionspunkt mit Richtungspfeil für belebte Übersichtspläne; der Punkt markiert die Hinterachse."),
                        "The dot marks the true position of the rear axle; its 0.9 mm diameter and the arrow are paper sizes and carry no dimension. A turning circle of 1.5 m is not implied.",
                        "Der Punkt markiert die wahre Position der Hinterachse; sein Durchmesser von 0,9 mm und der Pfeil sind Papiergrößen ohne Maßbedeutung. Ein Wendekreis von 1,5 m ist damit nicht gemeint.")]
finish(aid, src, a, R, dict(provenance=prov, materials=mats, anchors=anchors, symbols=syms, poses=poses, clips=clips, thermal=thermal, presentation=pres))
