from hbuild import *
import elderly_geo

AID = 'elderly-with-cane'
c = Ctx(AID)
v1 = [dict(s) for s in c.a['geometry']['elevation']]
geo = elderly_geo.build(c)

HAT = hatch
pens = {"500": {"body": .13, "detail": None, "texture": None, "glazing": None, "ground": .13},
        "200": {"body": .18, "detail": .13, "texture": None, "glazing": None, "ground": .18},
        "100": {"body": .25, "detail": .15, "texture": .1, "glazing": None, "ground": .25},
        "50": {"body": .35, "detail": .25, "texture": .13, "glazing": .13, "ground": .35}}

# 1:50 = the v1 drawing as raw material, lod removed, material hatch added
s50 = []
for s in v1:
    s = {k: v for k, v in s.items() if k != "lod"}
    if s["id"] == "jacket": s["hatch"] = "tweed-weave"
    if s["id"] == "trousers-seat" or s["id"].startswith(("thigh", "shank")): pass
    s50.append(s)

D_ = lambda **k: {kk: tx(*vv) for kk, vv in k.items()}
reps = {
    "500": dict(shapes=geo["500"]["shapes"], pens=pens["500"], hatches=[], range=(350, 1000), decisions=D_(
        silhouette=("One composed stooped silhouette: cap and head, jacket mass with the near arm, trousers and shoes as bone-bound pieces that overlap into a single outline; the forward lean (head ahead of the feet) is the identifying feature.",
                    "Eine zusammengesetzte gebeugte Silhouette: Mütze und Kopf, Jackenmasse mit dem nahen Arm, Hose und Schuhe als knochengebundene Teile, die zu einem Umriss verschmelzen; der Vorneigewinkel (Kopf vor den Füßen) ist das Erkennungsmerkmal."),
        retained=("Head with cap peak, kyphotic back, near arm and hand to the cane, both legs and shoes, cane.", "Kopf mit Mützenschirm, Rundrücken, naher Arm und Hand am Stock, beide Beine und Schuhe, Stock."),
        merged=("Hair, skin and cap form one head piece; jacket, collar, hump and pockets are one mass; the hand merges into the forearm piece.", "Haar, Haut und Mütze bilden ein Kopfstück; Jacke, Kragen, Buckel und Taschen sind eine Masse; die Hand verschmilzt mit dem Unterarmstück."),
        omitted=("Face details, glasses, ear, fingers, thumb, collar, pockets, buttons, the far arm (0.14 mm wide, would only show as a sliver) and all texture.", "Gesichtsdetails, Brille, Ohr, Finger, Daumen, Kragen, Taschen, Knöpfe, der ferne Arm (0,14 mm breit, wäre nur ein Splitter) und jede Textur."),
        thinElements=("The cane (25 mm real) is drawn as a single 0.13 mm line, i.e. 65 mm real width: thickened 2.6 times so that the third support stays visible; it keeps a 0.2 mm gap to the legs. Ankles are not thickened (0.17 mm).", "Der Stock (25 mm real) ist eine einzelne 0,13-mm-Linie, also 65 mm real: 2,6-fach verbreitert, damit die dritte Stütze sichtbar bleibt; er hält 0,2 mm Abstand zu den Beinen. Knöchel werden nicht verdickt (0,17 mm)."),
        hatching=("None; solid tone only.", "Keine; nur Flächenton."),
        styles=("Line view: one outline pen 0.13 mm. Solid view: jacket mid tone, trousers and shoes dark, head light, so the stoop reads as a mass.", "Linienansicht: ein Konturstift 0,13 mm. Flächenansicht: Jacke mittlerer Ton, Hose und Schuhe dunkel, Kopf hell, sodass die Beugung als Masse liest."),
        overlays=("Thermal colours attach to the pieces: head piece carries face, hair and cap as a merged area-weighted value, the forearm piece merges jacket and hands (labelled merged); trousers and shoes are separate.", "Wärmefarben liegen auf den Teilen: das Kopfstück trägt Gesicht, Haar und Mütze als flächengewichteten Mischwert, das Unterarmstück Jacke und Hände (als gemischt gekennzeichnet); Hose und Schuhe sind getrennt."))),
    "200": dict(shapes=geo["200"]["shapes"], pens=pens["200"], hatches=[], range=(150, 350), decisions=D_(
        silhouette=("Main masses separated: head with hair and cap, collar, jacket, trousers seat, one piece per leg segment and arm segment, shoes; cane and crook as lines.", "Hauptmassen getrennt: Kopf mit Haar und Mütze, Kragen, Jacke, Hosensitz, je ein Stück pro Bein- und Armsegment, Schuhe; Stock und Krücke als Linien."),
        retained=("Cap peak, hair at the nape, jacket with front opening line, both arms and hands (merged into the forearm pieces), both legs, shoes with toe pieces, cane with crook.", "Mützenschirm, Haar im Nacken, Jacke mit vorderer Öffnungslinie, beide Arme und Hände (im Unterarmstück verschmolzen), beide Beine, Schuhe mit Spitzenstück, Stock mit Krücke."),
        merged=("Hand and glove-free fingers into the forearm; jacket hump into the jacket; shirt and waistcoat layers are not drawn.", "Hand und Finger im Unterarm; Jackenbuckel in der Jacke; Hemd- und Westenlage nicht gezeichnet."),
        omitted=("Face features, ear, glasses, thumb, fingers, pocket, buttons, check pattern, sole lines.", "Gesichtszüge, Ohr, Brille, Daumen, Finger, Tasche, Knöpfe, Karomuster, Sohlenlinien."),
        thinElements=("Cane and crook are 0.18 mm lines (36 mm real instead of 25 mm), kept 1.4 times thickened; crook is an open stroke.", "Stock und Krücke sind 0,18-mm-Linien (36 mm real statt 25 mm), 1,4-fach verbreitert; die Krücke ist ein offener Strich."),
        hatching=("None.", "Keine."),
        styles=("Outlines 0.18 mm, one internal edge (jacket front) 0.13 mm; solid tones: jacket mid, trousers dark, face light, cap dark.", "Konturen 0,18 mm, eine innere Kante (Jackenfront) 0,13 mm; Flächentöne: Jacke mittel, Hose dunkel, Gesicht hell, Mütze dunkel."),
        overlays=("Every thermal region has its own piece except the forearm pieces (jacket and hand-skin merged); head skin and hair are separate pieces.", "Jede Wärmeregion hat ein eigenes Stück außer den Unterarmstücken (Jacke und Hand gemischt); Gesichtshaut und Haar sind getrennte Stücke."))),
    "100": dict(shapes=geo["100"]["shapes"], pens=pens["100"], hatches=[
        HAT("tweed-weave", "Tweed weave", "Tweedgewebe", "lines", 45, 1.0, 0.07),
        HAT("far-limb", "Far side limb", "Abgewandtes Glied", "lines", 60, 0.6, 0.07)], range=(75, 150), decisions=D_(
        silhouette=("Articulated clothing: collar, tweed jacket with pocket and front line, trousers per segment, mitten hands with thumb, shoes with toe piece, hair mass and flat cap, cane with crook.", "Gegliederte Kleidung: Kragen, Tweedjacke mit Tasche und Frontlinie, Hose je Segment, Fäustlingshände mit Daumen, Schuhe mit Spitzenstück, Haarmasse und Schiebermütze, Stock mit Krücke."),
        retained=("Ear, cap seam, sole lines, both thumbs, far limbs.", "Ohr, Mützennaht, Sohlenlinien, beide Daumen, abgewandte Glieder."),
        merged=("Fingers are one mitten outline; hair fringe and cap peak are one cap piece.", "Finger sind eine Fäustlingskontur; Haarkranz und Mützenschirm sind ein Mützenstück."),
        omitted=("Eye, glasses, wrinkle lines, buttons, check lines, finger separations.", "Auge, Brille, Fältchen, Knöpfe, Karolinien, Fingertrennungen."),
        thinElements=("Cane and crook are drawn with a 0.25 mm pen, equal to the 25 mm real shaft diameter: no thickening.", "Stock und Krücke sind mit 0,25-mm-Stift gezeichnet, gleich dem realen Schaftdurchmesser von 25 mm: keine Verdickung."),
        hatching=("Tweed weave: 1.0 mm lines at 45 degrees on the jacket; far-side limbs 0.6 mm lines at 60 degrees to push the far leg and arm back.", "Tweedgewebe: 1,0-mm-Linien unter 45 Grad auf der Jacke; abgewandte Glieder 0,6-mm-Linien unter 60 Grad, damit fernes Bein und Arm zurücktreten."),
        styles=("Body pen 0.25 mm, detail 0.15 mm, texture 0.1 mm; solid tones as in 1:50 but without check lines.", "Körperstift 0,25 mm, Detail 0,15 mm, Textur 0,1 mm; Flächentöne wie 1:50, aber ohne Karolinien."),
        overlays=("All eight regions are drawn: face, hair, cap, hands, jacket, trousers, shoes, cane; the hatch is clipped inside the region shape so colour maps stay readable.", "Alle acht Regionen sind gezeichnet: Gesicht, Haar, Mütze, Hände, Jacke, Hose, Schuhe, Stock; die Schraffur liegt innerhalb der Regionsform, Farbkarten bleiben lesbar."))),
    "50": dict(shapes=s50, pens=pens["50"], hatches=[HAT("tweed-weave", "Tweed weave", "Tweedgewebe", "cross", 45, 1.4, 0.06)], range=(20, 75), decisions=D_(
        silhouette=("Fully articulated figure drawn from the original 1:50 study: profile face with ear, eye and glasses, flat cap, hair, collar, jacket with hump, pocket, buttons and check lines, trousers per segment, fingered hands, shoes with sole and toe seams, cane with grain.", "Voll gegliederte Figur nach der ursprünglichen 1:50-Studie: Gesicht im Profil mit Ohr, Auge und Brille, Schiebermütze, Haar, Kragen, Jacke mit Buckel, Tasche, Knöpfen und Karolinien, Hose je Segment, Hände mit Fingern, Schuhe mit Sohlen- und Spitzennaht, Stock mit Maserung."),
        retained=("All detail of the original drawing.", "Alle Details der ursprünglichen Zeichnung."),
        merged=("Nothing is merged; the interior outlines of adjacent same-region pieces are hidden by the depiction.", "Nichts ist verschmolzen; die Innenkonturen angrenzender Stücke derselben Region werden von der Darstellung ausgeblendet."),
        omitted=("Shirt cuff and waistcoat layers (not visible), laces.", "Hemdmanschette und Westenlage (nicht sichtbar), Schnürsenkel."),
        thinElements=("The cane is a 21 mm capsule (0.42 mm at scale) with a 25 mm crook; finger separations are 0.13 mm lines at a 6 mm finger pitch.", "Der Stock ist eine 21-mm-Kapsel (0,42 mm im Maßstab) mit 25-mm-Krücke; Fingertrennungen sind 0,13-mm-Linien bei 6 mm Fingerteilung."),
        hatching=("Cross hatch 1.4 mm at 45 degrees (pen 0.06) indicates tweed on the jacket in addition to the check lines.", "Kreuzschraffur 1,4 mm unter 45 Grad (Stift 0,06) deutet den Tweed der Jacke zusätzlich zu den Karolinien an."),
        styles=("Body 0.35, detail 0.25, texture 0.13 mm; solid tones follow the material tone.", "Körper 0,35, Detail 0,25, Textur 0,13 mm; Flächentöne folgen dem Materialton."),
        overlays=("One shape per region and bone; all regions drawn.", "Ein Stück je Region und Knochen; alle Regionen gezeichnet."))),
}

# ------------------------------------------------------------------ physical scenario v2
def clo(x): return x*CLO
AIR0 = 0.161*CLO


def clothed(prefix, garments, area_note=""):
    ls = [layer(prefix+"-air", "air-gap", "still-air", AIR0, 0, "Air enclosed under the clothing", "Unter der Kleidung eingeschlossene Luft", 0.004)]
    for gid, kind, m, iclu, cap, en, de, th in garments:
        ls.append(layer(prefix+"-"+gid, kind, m, iclu*0.835*CLO, cap, en, de, th))
    return ls


MATS = [mat("still-air", "Still air", "Ruhende Luft", 0.9), mat("cotton-flannel", "Cotton flannel shirt", "Baumwoll-Flanellhemd", 0.9, 0.5), mat("wool-knit", "Wool waistcoat", "Wollweste", 0.9, 0.6),
        mat("cap-tweed", "Tweed cap", "Tweedmütze", 0.92, 0.7), mat("long-johns", "Cotton long underwear", "Lange Baumwollunterwäsche", 0.9, 0.5), mat("wool-sock", "Wool socks", "Wollsocken", 0.9, 0.6)]
AT = 1.0
regions = [
    dict(id="face-skin", en="Face and neck skin", de="Haut von Gesicht und Hals", kind="skin", model="body", material="skin", area=0.099, L=0.2, geom="sphere", svf=0.6, layers=[], seg="head", ms=0.09, sf=0.055, ex=1.0, su=0.5),
    dict(id="hair", en="White hair at nape and temples", de="Weißes Haar an Nacken und Schläfen", kind="hair", model="body", material="hair", area=0.027, L=0.18, geom="sphere", svf=0.6, layers=[layer("hair-pelt", "hair", "hair", 0.04, 300, "Hair and trapped air", "Haar und eingeschlossene Luft", 0.01)], seg="head", ms=0.01, sf=0.015, ex=1.0, su=0.45),
    dict(id="cap", en="Flat cap over the scalp", de="Schiebermütze über der Kopfhaut", kind="clothing", model="body", material="cap-tweed", area=0.059, L=0.18, geom="sphere", svf=0.8, layers=[layer("cap-hair", "hair", "hair", 0.03, 250, "Hair under the cap", "Haar unter der Mütze", 0.008)] + [layer("cap-air", "air-gap", "still-air", 0.02, 0, "Air under the cap", "Luft unter der Mütze", 0.006), layer("cap-tweed", "clothing", "cap-tweed", 0.035, 900, "Tweed cap", "Tweedmütze", 0.003)], seg="head", ms=0.05, sf=0.03, ex=1.0, su=0.6),
    dict(id="hand-skin", en="Bare hands", de="Unbedeckte Hände", kind="skin", model="body", material="skin", area=0.09, L=0.05, geom="cylinder", svf=0.4, layers=[], seg="hand", ms=0.03, sf=0.05, ex=1.0, su=0.35),
    dict(id="jacket", en="Shirt, waistcoat and tweed jacket on trunk and arms", de="Hemd, Weste und Tweedjacke an Rumpf und Armen", kind="clothing", model="body", material="tweed", area=0.996, L=0.3, geom="cylinder", svf=0.5,
         layers=clothed("jk", [("shirt", "clothing", "cotton-flannel", 0.25, 700, "Long-sleeve flannel shirt", "Langarm-Flanellhemd", 0.0012), ("waistcoat", "clothing", "wool-knit", 0.20, 800, "Thin wool waistcoat", "Dünne Wollweste", 0.003), ("tweed", "clothing", "tweed", 0.35, 1200, "Tweed jacket", "Tweedjacke", 0.004)]),
         seg="trunk", ms=0.50, sf=0.45, ex=0.9, su=0.42),
    dict(id="trousers", en="Long underwear and wool trousers on thighs and shanks", de="Lange Unterhose und Wollhose an Ober- und Unterschenkeln", kind="clothing", model="body", material="wool-trouser", area=0.677, L=0.14, geom="cylinder", svf=0.45,
         layers=clothed("tr", [("underwear", "clothing", "long-johns", 0.15, 400, "Long cotton underwear", "Lange Baumwollunterhose", 0.001), ("wool", "clothing", "wool-trouser", 0.24, 900, "Wool trousers", "Wollhose", 0.003)]), seg="leg", ms=0.27, sf=0.33, ex=0.88, su=0.4),
    dict(id="shoes", en="Socks and leather shoes", de="Socken und Lederschuhe", kind="clothing", model="body", material="leather-shoe", area=0.126, L=0.1, geom="cylinder", svf=0.25,
         layers=[layer("sh-sock", "clothing", "wool-sock", 0.012, 300, "Wool socks", "Wollsocken", 0.002), layer("sh-air", "air-gap", "still-air", 0.012, 0, "Air in the shoe", "Luft im Schuh", 0.004), layer("sh-leather", "shell", "leather-shoe", 0.025, 1500, "Leather upper and sole", "Lederschaft und Sohle", 0.004)], seg="foot", ms=0.05, sf=0.07, ex=1.0, su=0.25),
    dict(id="cane-wood", en="Wooden cane", de="Gehstock", kind="surface", model="passive", material="ash-wood", area=0.07, L=0.025, geom="cylinder", svf=0.5, layers=[], cap=1800, ex=1.0, su=0.5),
]
for r in regions:
    r.setdefault("ref", "iso-9920-2007")
regions[0]["src"] = "authored-estimate"; regions[0]["ref"] = "dubois-1916"
regions[-1]["ref"] = "ashrae-2021"

env = {"airTempC": 10, "meanRadiantTempC": 10, "skyTempC": 3, "groundTempC": 9, "relativeHumidity": 0.8, "windMS": 1.5, "windDirectionDeg": 180, "solarWM2": 0, "diffuseFraction": 1, "pressurePa": 101325}
body = {"species": "human", "massKg": 70, "skinAreaM2": 1.8, "basalMetabolismW": 67.7, "metWM2": 58.15, "coreSetPointC": 36.5, "skinSetPointC": 33.6, "sweating": True, "panting": False,
        "maxSkinBloodFlowLM2H": 63, "minSkinBloodFlowLM2H": 6.3, "source": "authored-estimate", "reference": "gagge-1971"}
resp = {"noseEmitter": "breath-nose", "mouthEmitter": "breath-mouth", "exhaledTempC": 32.5, "exhaledRelativeHumidity": 0.95,
        "patterns": [pattern("quiet", "Quiet standing", "Ruhiges Stehen", 16, 0.55, (0.38, 0.04, 0.45, 0.13), "nose"),
                     pattern("slow-walking", "Slow walking with a cane", "Langsames Gehen mit Stock", 19, 0.8, (0.42, 0.02, 0.52, 0.04), "nose")],
        "motion": [{"bone": "torso", "angleDeg": 0.8, "dy": 0.004}], "source": "authored-estimate", "reference": "gupta-2010"}
emitters = [
    {"id": "breath-nose", "kind": "breath", "title": tx("Exhaled breath from the nostrils", "Ausatemluft aus den Nasenlöchern"), "anchor": "nose", "directionDeg": -150, "diameterM": 0.0095, "source": "authored-estimate", "reference": "gupta-2010"},
    {"id": "breath-mouth", "kind": "breath", "title": tx("Exhaled breath from the mouth", "Ausatemluft aus dem Mund"), "anchor": "mouth", "directionDeg": -100, "diameterM": 0.0124, "source": "authored-estimate", "reference": "gupta-2010"},
    {"id": "plume", "kind": "plume", "title": tx("Thermal plume above the body", "Thermische Fahne über dem Körper"), "anchor": "crown", "directionDeg": 0, "diameterM": 0.2, "region": "jacket", "source": "authored-estimate", "reference": "ashrae-2021"},
]

FEET = {"l": ("heel-l", "ball-l"), "r": ("heel-r", "ball-r")}
def pose_rule(p):
    met = p["thermal"]["activityMet"]
    contacts = p["contacts"]
    n = sum(1 for s in "lr" if any(x in contacts for x in FEET[s]))
    ct = {}
    if n: ct["shoes"] = {"fraction": round(0.175*n, 3), "tempC": 9}
    if "cane-tip" in contacts: ct["cane-wood"] = {"fraction": 0.004, "tempC": 9}
    ct["hand-skin"] = {"fraction": 0.15, "tempC": 10}
    return {"met": met, "work": 0, "pat": "quiet" if met <= 1.4 else "slow-walking", "contact": ct, "em": {"plume": round(min(1, 0.3+0.2*met), 2)}}

spec = dict(materials=MATS, refs=[GAGGE, GUPTA], new_anchors=[
    dict(id="nose", bone="head", wx=0.272, wy=1.466, en="Nostrils", de="Nasenlöcher"), dict(id="crown", bone="head", wx=0.172, wy=1.645, en="Crown of the head", de="Scheitel")],
    env=env, body=body, respiration=resp, emitters=emitters, regions=regions, baseMet=1.2, emit_default={"plume": 0.6}, pose_rule=pose_rule,
    clip_tracks={"cane-gait": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 1}]}, {"target": "emitter:plume", "ease": "smooth", "keys": [{"t": 0, "v": 0.7}, {"t": 1.7, "v": 0.7}]}],
                 "rest-on-cane": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 1.6, "v": 1}, {"t": 4.2, "v": 0}]}, {"target": "emitter:plume", "ease": "smooth", "keys": [{"t": 0, "v": 0.5}, {"t": 3, "v": 0.7}, {"t": 6, "v": 0.45}]}]},
    assumptions=[
        tx("Body regions sum to the DuBois area of 1.80 m2 (70 kg, 170 cm); clothed regions carry the clothing area factor 1 + 0.28 clo. The cane is an additional passive surface.", "Die Körperregionen ergeben die DuBois-Fläche von 1,80 m2 (70 kg, 170 cm); bekleidete Regionen tragen den Bekleidungsflächenfaktor 1 + 0,28 clo. Der Stock ist eine zusätzliche passive Fläche."),
        tx("Clothing is layered per region (shirt, waistcoat, tweed jacket; long underwear, wool trousers; socks, leather shoe) with garment values guided by ISO 9920 (sum of garments x 0.835 plus an enclosed air layer of 0.161 clo, 1 clo = 0.155 m2K/W); vapour resistance follows R / (16.5e-3 x 0.38).", "Die Kleidung ist je Region geschichtet (Hemd, Weste, Tweedjacke; lange Unterhose, Wollhose; Socken, Lederschuh) mit Bekleidungswerten nach Anhalt von ISO 9920 (Summe der Kleidungsstücke x 0,835 plus eingeschlossene Luftschicht 0,161 clo, 1 clo = 0,155 m2K/W); der Dampfwiderstand folgt R / (16,5e-3 x 0,38)."),
        tx("Slow walking at 2 km/h is 2.0 met (ISO 8996); the core set point of 36.5 C, a skin set point of 33.6 C and a reduced maximum skin blood flow of 63 L/(m2 h) are authored estimates for a man of 74 years and are not clinical values.", "Langsames Gehen mit 2 km/h entspricht 2,0 met (ISO 8996); der Kernsollwert 36,5 C, der Hautsollwert 33,6 C und der verringerte maximale Hautdurchfluss von 63 L/(m2 h) sind Autorenschätzungen für einen 74-jährigen Mann, keine klinischen Werte."),
        tx("Breathing patterns are derived from the metabolic rate (ventilation about 25 times the oxygen uptake): 8.8 L/min quiet, 15 L/min walking; the nose carries all flow at these volumes. Exhaled air at 32.5 C is cooled in the nose at 10 C ambient.", "Atemmuster folgen aus dem Stoffwechsel (Ventilation etwa das 25-Fache der Sauerstoffaufnahme): 8,8 L/min in Ruhe, 15 L/min beim Gehen; die Nase führt bei diesen Volumina den gesamten Strom. Ausatemluft von 32,5 C ist bei 10 C Umgebung in der Nase abgekühlt."),
        tx("The reference case is a cold overcast morning (no direct sun); sunlit fractions are stored for use with other environments.", "Der Referenzfall ist ein kalter bedeckter Morgen (keine direkte Sonne); Besonnungsanteile sind für andere Umgebungen hinterlegt."),
    ])
doc = finish(c, spec, reps)
a = doc["asset"]
a["summary"] = c.a["summary"]
print(write(doc, AID))
