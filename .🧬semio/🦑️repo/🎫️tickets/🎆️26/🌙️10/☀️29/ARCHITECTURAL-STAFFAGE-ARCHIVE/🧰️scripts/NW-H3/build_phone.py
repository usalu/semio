from a_phone_geom import *
from h3asm import *
from h3check import check

aid = "standing-phone-user"
SK = dubois(BODY, 1.69)
mats = [m for m in BASE["materials"] if m["id"] in ("skin", "hair", "cotton-underwear", "cotton-sock")] + [
    material("fleece-sweatshirt", "Cotton-polyester fleece sweatshirt", "Sweatshirt aus Baumwoll-Polyester-Fleece", 0.93, 0.6, cp=1300),
    material("denim", "Denim", "Denim", 0.92, 0.75, cp=1300),
    material("canvas-sneaker", "Canvas sneaker with rubber sole", "Stoffsneaker mit Gummisohle", 0.9, 0.5, cp=1400),
    material("headphone-polymer", "Polymer and imitation-leather headphones", "Kopfhörer aus Polymer und Kunstleder", 0.9, 0.7, cp=1400),
    material("phone-glass-metal", "Glass and aluminium smartphone", "Smartphone aus Glas und Aluminium", 0.85, 0.7, cp=800),
]
cap_skin = 13000
f_h = fcl_of(0.09 + 0.34)
h_layers = [garment("tee", "Cotton T-shirt", "Baumwoll-T-Shirt", 0.09, "cotton-underwear", 0.0008, 150, f_h),
            garment("sweatshirt", "Fleece sweatshirt, hood down", "Fleece-Sweatshirt, Kapuze unten", 0.34, "fleece-sweatshirt", 0.004, 450, f_h)]
f_j = fcl_of(0.03 + 0.25)
j_layers = [garment("briefs", "Cotton briefs", "Baumwollslip", 0.03, "cotton-underwear", 0.0008, 100, f_j),
            garment("jeans", "Slim denim jeans", "Enge Jeans", 0.25, "denim", 0.0012, 440, f_j)]
f_s = 1.0
s_layers = [garment("socks", "Ankle socks", "Sneakersocken", 0.02, "cotton-sock", 0.002, 100, f_s),
            garment("sneaker", "Canvas sneaker", "Stoffsneaker", 0.04, "canvas-sneaker", 0.004, 600, f_s)]
hair_layers = [{"id": "hair", "title": T("Long hair in a ponytail", "Langes Haar im Pferdeschwanz"), "kind": "hair", "material": "hair", "thicknessM": 0.03, "resistanceM2KW": 0.08, "vapourResistanceM2PaW": 12.8, "heatCapacityJM2K": 400, "source": "authored-estimate"}]
cap = lambda ls, base=cap_skin: base + sum(l["heatCapacityJM2K"] for l in ls)
regions = [
    body_region(FACE, "Face and neck skin", "Haut von Gesicht und Hals", "skin", "skin", SK * 0.07, 1.0, "head", 0.07, 0.07, 0.2, "sphere", 0.5, cap_skin, [], ref="dubois-1916"),
    body_region(HAIR, "Hair", "Haar", "hair", "hair", SK * 0.03, 1.0, "head", 0.04, 0.03, 0.2, "sphere", 0.8, cap(hair_layers, 8000), hair_layers),
    body_region(HAND, "Bare hands", "Unbedeckte Hände", "skin", "skin", SK * 0.05, 1.0, "hand", 0.04, 0.05, 0.1, "cylinder", 0.2, cap_skin, [], ref="dubois-1916"),
    body_region(HOOD, "T-shirt and fleece sweatshirt on trunk and arms", "T-Shirt und Fleece-Sweatshirt an Rumpf und Armen", "clothing", "fleece-sweatshirt", SK * 0.49, f_h, "trunk", 0.55, 0.49, 0.3, "vertical-plate", 0.45, cap(h_layers), h_layers),
    body_region(JEANS, "Jeans over the legs", "Jeans über den Beinen", "clothing", "denim", SK * 0.30, f_j, "leg", 0.27, 0.30, 0.13, "cylinder", 0.1, cap(j_layers), j_layers),
    body_region(SHOE, "Socks and sneakers", "Socken und Sneaker", "surface", "canvas-sneaker", SK * 0.06, f_s, "foot", 0.03, 0.06, 0.09, "horizontal-plate-up", 0.1, cap(s_layers), s_layers),
    {"id": HP, "title": T("Over-ear headphones worn around the neck", "Über dem Ohr getragene Kopfhörer um den Hals"), "kind": "surface", "model": "passive", "material": "headphone-polymer", "areaM2": 0.03, "characteristicLengthM": 0.1,
     "geometry": "cylinder", "skyViewFactor": 0.4, "layers": [], "heatCapacityJM2K": 11000, "source": "authored-estimate"},
    {"id": PH, "title": T("Smartphone, heated by its electronics", "Smartphone, von der Elektronik erwärmt"), "kind": "surface", "model": "operating", "material": "phone-glass-metal", "areaM2": 0.022, "characteristicLengthM": 0.147,
     "geometry": "vertical-plate", "skyViewFactor": 0.3, "layers": [], "heatCapacityJM2K": 6900, "operating": {"key": "phone", "heatW": 2.5}, "source": "authored-estimate"},
]
resp = {"noseEmitter": "nose-breath", "mouthEmitter": "mouth-breath", "exhaledTempC": 34, "exhaledRelativeHumidity": 0.95,
        "patterns": [
            {"id": "quiet-standing", "title": T("Quiet standing", "Ruhiges Stehen"), "rateBpm": 14, "tidalVolumeL": 0.40, "inhaleFraction": 0.38, "pauseAfterInhaleFraction": 0.04, "exhaleFraction": 0.45, "pauseAfterExhaleFraction": 0.13, "route": "nose", "source": "authored-estimate", "reference": "icrp-66-1994"},
            {"id": "typing", "title": T("Concentrated texting, slightly shallow", "Konzentriertes Tippen, leicht flach"), "rateBpm": 15, "tidalVolumeL": 0.38, "inhaleFraction": 0.40, "pauseAfterInhaleFraction": 0.05, "exhaleFraction": 0.45, "pauseAfterExhaleFraction": 0.10, "route": "nose", "source": "authored-estimate", "reference": "icrp-66-1994"},
            {"id": "talking", "title": T("Talking on the phone", "Telefonieren"), "rateBpm": 17, "tidalVolumeL": 0.55, "inhaleFraction": 0.25, "pauseAfterInhaleFraction": 0.02, "exhaleFraction": 0.68, "pauseAfterExhaleFraction": 0.05, "route": "nose-and-mouth", "source": "authored-estimate", "reference": "gupta-2010"}],
        "motion": [{"bone": "torso", "angleDeg": 0.5, "dy": 0.003}], "source": "authored-estimate", "reference": "gupta-2010"}
thermal = OrderedDict()
thermal["environment"] = {"airTempC": 12, "meanRadiantTempC": 12, "skyTempC": 0, "groundTempC": 12, "relativeHumidity": 0.7, "windMS": 0.8, "windDirectionDeg": 180, "solarWM2": 0, "diffuseFraction": 1, "pressurePa": 101325}
thermal["baseMet"] = 1.2
thermal["body"] = {"species": "human", "massKg": 58, "skinAreaM2": SK, "basalMetabolismW": mifflin("f", 58, 169, 24), "metWM2": 58.15, "coreSetPointC": 36.8, "skinSetPointC": 34.1, "sweating": True, "panting": False, "maxSkinBloodFlowLM2H": 90, "minSkinBloodFlowLM2H": 1.0, "source": "authored-estimate", "reference": "mifflin-1990"}
thermal["respiration"] = resp
thermal["operating"] = [{"id": "phone", "title": T("Smartphone state", "Zustand des Smartphones"), "levels": [{"v": 0, "title": T("Off", "Aus")}, {"v": 0.3, "title": T("Standby", "Bereitschaft")}, {"v": 0.6, "title": T("Screen on, reading and typing", "Bildschirm an, Lesen und Tippen")}, {"v": 1.0, "title": T("Call or camera in use", "Anruf oder Kamera aktiv")}]}]
thermal["passengers"] = []
thermal["regions"] = regions
thermal["emitters"] = breath_emitters(plume_region=HOOD)
thermal["assumptions"] = [
    T(f"Skin area {SK} m2 after DuBois and DuBois (1916) for 58 kg and 1.69 m; regions carry skin fractions 0.07 face and neck, 0.03 scalp, 0.05 hands, 0.49 trunk and arms, 0.30 legs, 0.06 feet (rule-of-nines proportions, authored).",
      f"Hautfläche {SK} m2 nach DuBois und DuBois (1916) für 58 kg und 1,69 m; die Regionen tragen die Hautanteile 0,07 Gesicht und Hals, 0,03 Kopfhaut, 0,05 Hände, 0,49 Rumpf und Arme, 0,30 Beine, 0,06 Füße (Neunerregel-Proportionen, geschätzt)."),
    T("Basal metabolism from the Mifflin-St Jeor equation for a 24-year-old woman of 58 kg and 169 cm; clothing in clo after the ISO 9920 tables (1 clo = 0.155 m2K/W), digits authored; vapour resistance R/(0.0165 K/Pa x 0.38).",
      "Grundumsatz aus der Mifflin-St-Jeor-Gleichung für eine 24-jährige Frau mit 58 kg und 169 cm; Kleidung in clo nach den Tabellen von ISO 9920 (1 clo = 0,155 m2K/W), Werte geschätzt; Dampfwiderstand R/(0,0165 K/Pa x 0,38)."),
    T("The smartphone releases about 2.5 W at full use (call or camera) and about 1.5 W with the screen on; both are authored estimates, not measurements. It is held in the hands, whose skin contact is not modelled.",
      "Das Smartphone gibt bei voller Nutzung (Anruf oder Kamera) etwa 2,5 W ab, bei eingeschaltetem Bildschirm etwa 1,5 W; beides sind geschätzte Werte, keine Messungen. Es liegt in den Händen, deren Hautkontakt nicht modelliert wird."),
    T("Standing at ease is about 1.2 met (ISO 7730 Annex A), phone use adds hardly any load; breathing patterns are authored and only guided by quiet adult values and Gupta et al. (2010) for talking.",
      "Entspanntes Stehen entspricht etwa 1,2 met (ISO 7730 Anhang A), das Telefonieren erhöht die Last kaum; die Atemmuster sind geschätzt und nur an ruhigen Erwachsenenwerten und Gupta et al. (2010) für das Sprechen orientiert."),
]
rid = [r["id"] for r in regions]
EXP = {HOOD: 0.92}


def tb(met, br, ph, plume):
    return binding(regions, {"body-plume": plume}, met, br, exposed=EXP, contact={SHOE: {"fraction": 0.3, "tempC": 12}}, operating={"phone": ph})


TB = {"texting": tb(1.2, "typing", 0.6, 0.6), "call": tb(1.3, "talking", 1.0, 0.6), "photo": tb(1.4, "quiet-standing", 1.0, 0.7)}
TITLES = {"texting": (T("Texting, head bowed", "Tippen mit gesenktem Kopf"), T("Feet together, weight on both legs, head tilted 40 degrees down, both hands holding the phone at chest height.", "Füße eng beieinander, Gewicht auf beiden Beinen, Kopf 40 Grad nach unten geneigt, beide Hände halten das Telefon auf Brusthöhe.")),
          "call": (T("Phone call at the ear", "Telefonat am Ohr"), T("Both feet flat and close, the far arm folded across the waist, the near hand holding the phone to the ear with the elbow forward.", "Beide Füße flach und eng, der ferne Arm vor dem Bauch verschränkt, die nahe Hand hält das Telefon am Ohr, der Ellbogen zeigt nach vorn.")),
          "photo": (T("Holding the phone up for a photo", "Telefon für ein Foto hochhalten"), T("Narrow stance, both arms nearly straight in front, phone upright at eye height.", "Enge Standposition, beide Arme fast gestreckt nach vorn, Telefon aufrecht auf Augenhöhe."))}
CON = {"texting": CONTACTS, "call": CONTACTS, "photo": CONTACTS}
poses = []
for k in ("texting", "call", "photo"):
    off, root = POSES[k]
    poses.append({"id": {"texting": "texting", "call": "phone-call", "photo": "photo-pose"}[k], "title": TITLES[k][0], "description": TITLES[k][1], "bones": off, "ground": "auto", "contacts": CON[k], "thermal": TB[k],
                  "loads": [{"id": "phone-weight", "kind": "carried", "title": T("Smartphone weight", "Gewicht des Smartphones"), "anchor": "phone-cog", "fxN": 0, "fyN": -round(PHONE * 9.80665, 3), "source": "authored-estimate"}]})
PB = {p["id"]: p for p in poses}
tx, cl = PB["texting"]["bones"], PB["phone-call"]["bones"]
legs = {b: dict(tx[b]) for b in tx if b.split("-")[0] in ("thigh", "shank", "foot", "toe")}


def ov(**kw):
    d = {}
    for b, v in kw.items():
        b = b.replace("_", "-")
        d[b] = {"angleDeg": round(tx.get(b, {}).get("angleDeg", 0) + v, 4)}
    return d


def mid(b):
    return {"angleDeg": round((tx.get(b, {}).get("angleDeg", 0) + cl.get(b, {}).get("angleDeg", 0)) / 2, 4)}


clips = []
c1 = {"id": "scroll-and-sway", "title": T("Scrolling and swaying", "Scrollen und Wiegen"), "description": T("A six-second loop: the head dips and lifts with the text, the thumb scrolls, the torso shifts slightly; feet stay planted.", "Eine Schleife von sechs Sekunden: Der Kopf senkt und hebt sich mit dem Text, der Daumen scrollt, der Oberkörper wiegt leicht; die Füße bleiben stehen."),
      "durationS": 6, "fps": 30, "loop": True, "rootMotion": "none", "keys": [
          {"t": 0, "pose": "texting", "ease": "smooth"},
          {"t": 1.5, "pose": "texting", "ease": "smooth", "bones": ov(head=-3, neck=-2, hand_l=5, torso=-1)},
          {"t": 3, "pose": "texting", "ease": "smooth", "bones": ov(head=3, neck=1, torso=2, hand_l=-2)},
          {"t": 4.5, "pose": "texting", "ease": "smooth", "bones": ov(head=-2, neck=-1, hand_l=4, torso=-0.5)},
          {"t": 6, "pose": "texting", "ease": "smooth"}],
      "contacts": [{"anchor": c, "fromS": 0, "toS": 6} for c in CON["texting"]],
      "phases": [{"id": "reading", "fromS": 0, "toS": 3, "title": T("Reading and scrolling", "Lesen und Scrollen")}, {"id": "typing", "fromS": 3, "toS": 6, "title": T("Typing a reply", "Antwort tippen")}],
      "events": clip_events([(3.0, "operating", "Screen brightens while typing", "Bildschirm wird beim Tippen heller", "phone")]),
      "tracks": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 3, "v": 1}, {"t": 6, "v": 1}]},
                 {"target": "operating:phone", "ease": "smooth", "keys": [{"t": 0, "v": 0.6}, {"t": 3, "v": 0.8}, {"t": 6, "v": 0.6}]}]}
c2 = {"id": "answer-call", "title": T("Answering a call", "Anruf annehmen"), "description": T("Non-looping, four seconds: the head lifts, the near arm raises the phone from chest height to the ear while the far arm folds across the waist; the stance of the legs is held.", "Ohne Wiederholung, vier Sekunden: Der Kopf hebt sich, der nahe Arm führt das Telefon von der Brust zum Ohr, während der ferne Arm vor dem Bauch ruht; die Beinstellung bleibt."),
      "durationS": 4, "fps": 30, "loop": False, "rootMotion": "none", "keys": [
          {"t": 0, "pose": "texting", "ease": "smooth"},
          {"t": 0.8, "pose": "texting", "ease": "smooth", "bones": {"neck": {"angleDeg": -6}, "head": {"angleDeg": -8}, "torso": {"angleDeg": -1}}},
          {"t": 2.2, "pose": "phone-call", "ease": "smooth", "bones": {**legs, "arm-r": mid("arm-r"), "forearm-r": mid("forearm-r"), "hand-r": mid("hand-r"), "arm-l": mid("arm-l"), "forearm-l": mid("forearm-l"), "hand-l": mid("hand-l"), "neck": {"angleDeg": -4}, "head": {"angleDeg": 0}, "torso": {"angleDeg": -1}}},
          {"t": 3.2, "pose": "phone-call", "ease": "smooth", "bones": legs},
          {"t": 4, "pose": "phone-call", "ease": "smooth", "bones": legs}],
      "contacts": [{"anchor": c, "fromS": 0, "toS": 4} for c in CON["texting"]],
      "phases": [{"id": "ring", "fromS": 0, "toS": 0.8, "title": T("Phone rings", "Telefon klingelt")}, {"id": "raise", "fromS": 0.8, "toS": 3.2, "title": T("Raising the phone to the ear", "Telefon zum Ohr führen")}, {"id": "talk", "fromS": 3.2, "toS": 4, "title": T("Talking", "Sprechen")}],
      "events": clip_events([(0.8, "operating", "Phone starts ringing", "Telefon beginnt zu klingeln", "phone"), (3.2, "thermal", "Phone against the ear", "Telefon am Ohr", "phone")]),
      "tracks": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 1}, {"t": 3.2, "v": 2}, {"t": 4, "v": 2}]},
                 {"target": "operating:phone", "ease": "smooth", "keys": [{"t": 0, "v": 0.6}, {"t": 2.2, "v": 1.0}, {"t": 4, "v": 1.0}]},
                 {"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 1.2}, {"t": 4, "v": 1.3}]}]}
clips = [c1, c2]

R = {}
P = lambda b, d, t, g: {"body": b, "detail": d, "texture": t, "glazing": None, "ground": g}
R["500"] = {"status": "authored", "denominator": 500, "validRange": {"minDenominator": 350, "maxDenominator": 1000}, "pens": P(0.13, None, None, 0.18), "hatches": [], "geometry": {"elevation": rep_500()},
    "decisions": decisions(
    dict(silhouette="A slightly stooped profile of 1.67 m (3.3 mm): head with hair and ponytail as one mass, torso with hood and headphones as one mass, the near arm as three tapered links reaching forward, two leg columns with shoes.",
         retained="The bowed head, the forward-reaching arm that carries the pose, the ponytail that sets the back extent, the step of the feet and the exact stature and extent.",
         merged="Face and hair; hoodie, headphones and phone in one torso shape listing all three regions; hand and phone remain in the hand blob.",
         omitted="The far arm (it hides behind the near arm in profile), neck, sleeve cuffs, hood fold, laces, sole line.",
         thinElements="The 10 mm phone and the 4 mm headphone band are not drawn, never thickened; the 60 mm ponytail is kept because it sets the back extent.",
         hatching="None.", styles="Mid-grey torso, darker legs, light shoes; line pen 0.13 mm only, no detail pen; light mode dark lines on white, dark mode inverted.",
         overlays="Thermal and structural overlays paint the torso shape with the area-weighted value of hoodie, headphones and phone (labelled merged); head, hand, legs and shoes keep their own value."),
    dict(silhouette="Ein leicht gebeugtes Profil von 1,67 m (3,3 mm): Kopf mit Haar und Pferdeschwanz als eine Masse, Rumpf mit Kapuze und Kopfhörern als eine Masse, der nahe Arm als drei verjüngte Glieder nach vorn, zwei Beinsäulen mit Schuhen.",
         retained="Der gesenkte Kopf, der nach vorn greifende Arm, der die Pose trägt, der Pferdeschwanz, der die hintere Ausdehnung bestimmt, die Schrittstellung der Füße sowie genaue Körpergröße und Ausdehnung.",
         merged="Gesicht und Haar; Sweatshirt, Kopfhörer und Telefon in einer Rumpfform mit allen drei Regionen; Hand und Telefon bleiben im Handfleck.",
         omitted="Der ferne Arm (er verschwindet im Profil hinter dem nahen), Hals, Ärmelbündchen, Kapuzenfalte, Schnürung, Sohlenlinie.",
         thinElements="Das 10 mm dicke Telefon und der 4 mm breite Kopfhörerbügel werden nicht gezeichnet, nie verdickt; der 60 mm lange Pferdeschwanz bleibt, weil er die hintere Ausdehnung bestimmt.",
         hatching="Keine.", styles="Mittelgrauer Rumpf, dunklere Beine, helle Schuhe; nur Strichstärke 0,13 mm, kein Detailstift; hell: dunkle Linien auf Weiß, dunkel: invertiert.",
         overlays="Thermische und statische Überlagerungen färben die Rumpfform mit dem flächengewichteten Wert von Sweatshirt, Kopfhörern und Telefon (als verschmolzen gekennzeichnet); Kopf, Hand, Beine und Schuhe behalten ihren eigenen Wert."))}
R["200"] = {"status": "authored", "denominator": 200, "validRange": {"minDenominator": 150, "maxDenominator": 350}, "pens": P(0.18, 0.10, None, 0.2), "hatches": [], "geometry": {"elevation": rep_200()},
    "decisions": decisions(
    dict(silhouette="Main masses separated: face, hair cap and ponytail, neck, hoodie with the bunched hood, both arms as sleeve, forearm and hand, both legs as jeans plus sneaker; 8.4 mm tall.",
         retained="Elbows and knees as separate masses so the pose changes read, the hood bump behind the neck, the headphones as a dark spot at the neck, the kangaroo pocket as the only inner line.",
         merged="Thumb into the hand, hem rib into the hoodie, sock into the sneaker, hair tie into the ponytail.",
         omitted="Fingers, ear, eye, cuffs, laces, seams, knee folds.",
         thinElements="The phone is one 0.18 mm stroke over its true 148 mm length instead of a 0.05 mm slab; fingers dropped.",
         hatching="None; fill tones carry the materials.",
         styles="Body pen 0.18 mm, detail pen 0.10 mm for the pocket; light tones for skin and shoes, mid for hoodie, dark for jeans, hair and headphones.",
         overlays="One region per mass; both sleeves share the hoodie value, the ponytail and the hair cap share the hair value, the stroke carries the phone value."),
    dict(silhouette="Hauptmassen getrennt: Gesicht, Haarkappe und Pferdeschwanz, Hals, Sweatshirt mit der gebauschten Kapuze, beide Arme als Ärmel, Unterarm und Hand, beide Beine als Jeans mit Sneaker; 8,4 mm hoch.",
         retained="Ellbogen und Knie als getrennte Massen, damit die Posenwechsel lesbar bleiben, der Kapuzenwulst hinter dem Nacken, die Kopfhörer als dunkler Fleck am Hals, die Känguru-Tasche als einzige Innenlinie.",
         merged="Daumen in die Hand, Saumbund in das Sweatshirt, Socke in den Sneaker, Haargummi in den Pferdeschwanz.",
         omitted="Finger, Ohr, Auge, Bündchen, Schnürung, Nähte, Knie-Falten.",
         thinElements="Das Telefon ist ein Strich von 0,18 mm über seine wahre Länge von 148 mm statt einer 0,05 mm dicken Platte; Finger entfallen.",
         hatching="Keine; die Füllungstöne tragen die Materialien.",
         styles="Körperstift 0,18 mm, Detailstift 0,10 mm für die Tasche; helle Töne für Haut und Schuhe, mittlere für das Sweatshirt, dunkle für Jeans, Haar und Kopfhörer.",
         overlays="Eine Region je Masse; beide Ärmel teilen den Sweatshirt-Wert, Pferdeschwanz und Haarkappe den Haarwert, der Strich trägt den Telefonwert."))}
R["100"] = {"status": "authored", "denominator": 100, "validRange": {"minDenominator": 75, "maxDenominator": 150}, "pens": P(0.2, 0.1, None, 0.25),
    "hatches": [{"id": "denim", "title": T("Denim twill lines", "Denim-Köperlinien"), "pattern": "lines", "angleDeg": 62, "spacingMm": 0.7, "penMm": 0.1}], "geometry": {"elevation": rep_100()},
    "decisions": decisions(
    dict(silhouette="Articulated clothing: sleeves with cuff lines, hoodie with hem rib, front pocket and bunched hood, jeans with a side seam, sneakers with sole lines and toe boxes, mitten hands with thumb; 16.7 mm tall.",
         retained="Hair mass with ponytail and ear, face profile with nose and chin, headphone cup and band at the neck, mitten hand with thumb, the phone as a stroke between the hands.",
         merged="Eye, brow and mouth are not drawn; laces and eyelets merge into the sneaker; the three finger groups form one mitten.",
         omitted="Eye (0.16 mm), laces, knee folds, drawstrings, stitching.",
         thinElements="The phone stroke (0.2 mm, true length 148 mm) replaces the 10 mm slab; the headphone band is a 0.1 mm line.",
         hatching="Jeans only: fine parallel twill lines at 62 degrees, 0.7 mm spacing, 0.10 mm pen; the fleece stays plain tone.",
         styles="Body pen 0.20 mm, detail 0.10 mm; hoodie mid-grey, jeans darker with twill, sneakers light, skin light.",
         overlays="Regions as at 1:200; headphone cup and band carry the headphone value, hood and hoodie share the hoodie value."),
    dict(silhouette="Gegliederte Kleidung: Ärmel mit Bündchenlinien, Sweatshirt mit Saumbund, Vordertasche und gebauschter Kapuze, Jeans mit Seitennaht, Sneaker mit Sohlenlinien und Zehenkappen, Fäustlingshände mit Daumen; 16,7 mm hoch.",
         retained="Haarmasse mit Pferdeschwanz und Ohr, Gesichtsprofil mit Nase und Kinn, Kopfhörerschale und Bügel am Hals, Fäustlingshand mit Daumen, das Telefon als Strich zwischen den Händen.",
         merged="Auge, Braue und Mund werden nicht gezeichnet; Schnürung und Ösen gehen im Sneaker auf; die drei Fingergruppen bilden einen Fäustling.",
         omitted="Auge (0,16 mm), Schnürsenkel, Knie-Falten, Kordeln, Nähte.",
         thinElements="Der Telefonstrich (0,2 mm, wahre Länge 148 mm) ersetzt die 10 mm dicke Platte; der Kopfhörerbügel ist eine 0,1-mm-Linie.",
         hatching="Nur die Jeans: feine Köperlinien im Winkel 62 Grad, 0,7 mm Abstand, 0,10 mm Stift; das Fleece bleibt flächig.",
         styles="Körperstift 0,20 mm, Detail 0,10 mm; Sweatshirt mittelgrau, Jeans dunkler mit Köper, Sneaker hell, Haut hell.",
         overlays="Regionen wie bei 1:200; Kopfhörerschale und Bügel tragen den Kopfhörerwert, Kapuze und Sweatshirt den Sweatshirt-Wert."))}
R["50"] = {"status": "authored", "denominator": 50, "validRange": {"minDenominator": 20, "maxDenominator": 75}, "pens": P(0.25, 0.13, 0.1, 0.3),
    "hatches": [{"id": "denim", "title": T("Denim twill lines", "Denim-Köperlinien"), "pattern": "lines", "angleDeg": 62, "spacingMm": 0.5, "penMm": 0.1},
                {"id": "knit", "title": T("Fleece knit lines", "Fleece-Strickstriche"), "pattern": "lines", "angleDeg": 90, "spacingMm": 0.6, "penMm": 0.1}], "geometry": {"elevation": rep_50()},
    "decisions": decisions(
    dict(silhouette="Fully articulated drawing: rib cuffs and hem, kangaroo pocket, drawstrings, collar rib, hood fold, jeans with side seams, knee fold and hems, sneakers with sole, toe sole and three lace crossings, finger separations, face with eye, brow and mouth; 33.4 mm tall.",
         retained="Everything of the 1:100 drawing plus the small construction details that are still wider than 0.1 mm at this scale.",
         merged="Eyelets merge into the lace strokes; hair strands are four strokes instead of single hairs.",
         omitted="Pores, individual hairs, stitching holes, fabric fibres, camera lens of the phone.",
         thinElements="Finger separations, laces and seams are 0.10 mm texture lines; the 10 mm phone is a real 0.2 mm rounded slab with a 0.13 mm screen line.",
         hatching="Hoodie and sleeves: vertical knit lines at 0.6 mm; jeans: twill lines at 62 degrees, 0.5 mm.",
         styles="Body 0.25 mm, detail 0.13 mm, texture 0.10 mm; tones as at 1:100, sneaker soles darker.",
         overlays="One region per shape; details inherit the region of the garment they sit on, the phone slab carries the phone value."),
    dict(silhouette="Voll gegliederte Zeichnung: Bündchen und Saumbund, Känguru-Tasche, Kordeln, Kragenbund, Kapuzenfalte, Jeans mit Seitennähten, Knie-Falte und Säumen, Sneaker mit Sohle, Zehensohle und drei Schnürkreuzungen, Fingertrennungen, Gesicht mit Auge, Braue und Mund; 33,4 mm hoch.",
         retained="Alles aus der Zeichnung 1:100 und die kleinen Konstruktionsdetails, die bei diesem Maßstab noch breiter als 0,1 mm sind.",
         merged="Ösen gehen in den Schnürstrichen auf; Haarsträhnen sind vier Striche statt einzelner Haare.",
         omitted="Poren, einzelne Haare, Stichlöcher, Gewebefasern, die Kameralinse des Telefons.",
         thinElements="Fingertrennungen, Schnürsenkel und Nähte sind 0,10-mm-Texturlinien; das 10 mm dicke Telefon ist eine echte, gerundete Platte von 0,2 mm mit einer 0,13-mm-Bildschirmlinie.",
         hatching="Sweatshirt und Ärmel: senkrechte Strickstriche mit 0,6 mm; Jeans: Köperlinien im Winkel 62 Grad, 0,5 mm.",
         styles="Körper 0,25 mm, Detail 0,13 mm, Textur 0,10 mm; Töne wie bei 1:100, Sneakersohlen dunkler.",
         overlays="Eine Region je Form; Details erben die Region des Kleidungsstücks, auf dem sie sitzen, die Telefonplatte trägt den Telefonwert."))}

W0 = fk(a, {"bones": poses[0]["bones"], "root": {"y": ground_y(a, poses[0]["bones"], poses[0]["contacts"])}})
asset = {
    "id": aid, "category": "humans",
    "title": T("Standing Phone User", "Stehende Smartphone-Nutzerin"),
    "summary": T("A 24-year-old woman of 1.69 m in a hooded sweatshirt, slim jeans and canvas sneakers, standing at a bus stop with her head bowed over a smartphone; over-ear headphones around the neck, a ponytail, weight on one leg. Poses for texting, a call at the ear and a photo.",
                 "Eine 24-jährige Frau von 1,69 m in Kapuzenpullover, enger Jeans und Stoffsneakern, die an einer Bushaltestelle mit gesenktem Kopf auf ein Smartphone schaut; Kopfhörer um den Hals, Pferdeschwanz, Gewicht auf einem Bein. Posen für Tippen, Telefonat am Ohr und Foto."),
    "tags": ["human", "adult", "woman", "standing", "smartphone", "headphones", "text-neck", "waiting"],
    "provenance": {"author": "Semio staffage content worker NW-H3", "method": "handcrafted", "created": "2026-10-29",
                   "references": refs(["de-leva-1996", "winter-2009", "drillis-contini-1966", "dubois-1916", "iso-9920-2007", "iso-7730-2005", "ashrae-2021", "gupta-2010", "gagge-1971", "mifflin-1990", "icrp-66-1994"]),
                   "notes": T("Rig scaled from the archive's human template to 1.69 m (de Leva female segment fractions); outlines, clothing, poses and clips handcrafted. The head-down posture follows the sagittal profile of a person looking at a hand-held phone; all thermal and respiratory parameters other than the cited equations are authored estimates.",
                              "Skelett aus der Menschenvorlage des Archivs auf 1,69 m skaliert (de Leva, weibliche Segmentanteile); Umrisse, Kleidung, Posen und Clips von Hand gezeichnet. Die Kopf-nach-unten-Haltung folgt dem Seitenprofil einer Person, die auf ein Handy schaut; alle thermischen und respiratorischen Parameter außer den zitierten Gleichungen sind Schätzwerte.")},
    "dimensions": {"heightM": 1.67, "widthM": 0.64, "depthM": 0.4, "massKg": MASS},
    "materials": mats, "rig": rig, "anchors": anchors, "representations": OrderedDict((k, R[k]) for k in ("500", "200", "100", "50")), "symbols": [],
    "poses": poses, "clips": clips, "thermal": thermal,
    "structure": {"gravityMS2": 9.80665, "dynamics": "inertial", "supports": [], "assumptions": [
        T("Segments are point masses at the de Leva centres (female table); the 190 g phone is a load at its own centre.", "Segmente sind Punktmassen in den Schwerpunkten nach de Leva (Frauentabelle); das 190 g schwere Telefon ist eine Last in seinem eigenen Schwerpunkt."),
        T("Contacts are heel and ball of each foot; both feet stand flat and close together in every named pose.", "Kontakte sind Ferse und Ballen jedes Fußes; beide Füße stehen in jeder benannten Pose flach und eng beieinander.")], "clinical": False},
    "presentation": {"defaultPose": "texting", "defaultClip": "scroll-and-sway", "defaultScale": 100, "viewpoints": ["elevation"]},
    "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component"], "completion": "authored"}
asset["rig"]["bones"][0]
errs = check(asset)
if "--write" in sys.argv:
    write_asset(aid, "humans", asset)
    print("written")
