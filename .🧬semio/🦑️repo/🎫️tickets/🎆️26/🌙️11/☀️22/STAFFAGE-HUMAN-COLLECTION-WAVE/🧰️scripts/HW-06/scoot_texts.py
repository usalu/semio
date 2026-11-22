"""🛵️ Poses, clips, physics and every text of `mobility-scooter-user` (English first, German second)."""
import copy
import hw06lib as L
import scoot_base as SB
import scoot_parts as SP

t = L.t
CONTACTS = ["patch-rear", "patch-front"]
PERSON_REGIONS = ["face-skin", "cap", "hand-skin", "gloves", "jacket", "trousers", "shoes"]
SCOOTER_REGIONS = ["shell", "upholstery", "tyres", "battery-box", "drive-motor"]
TURN = 360.0
LOOP_S, TURNS = 2.4, 5


def bindings(drive_level, sun=0.5, seat_contact=0.18):
    r = {
        "face-skin": {"exposedFraction": 1, "sunlitFraction": sun}, "cap": {"exposedFraction": 1, "sunlitFraction": sun},
        "hand-skin": {"exposedFraction": 1, "sunlitFraction": sun * 0.8}, "gloves": {"exposedFraction": 1, "sunlitFraction": sun * 0.8},
        "jacket": {"exposedFraction": 0.9, "sunlitFraction": sun * 0.8, "contact": {"fraction": 0.1, "tempC": 18}},
        "trousers": {"exposedFraction": 1 - seat_contact, "sunlitFraction": sun * 0.6, "contact": {"fraction": seat_contact, "tempC": 22}},
        "shoes": {"exposedFraction": 0.9, "sunlitFraction": sun * 0.5, "contact": {"fraction": 0.1, "tempC": 12}},
        "shell": {"exposedFraction": 1, "sunlitFraction": sun * 0.8}, "upholstery": {"exposedFraction": 0.5, "sunlitFraction": sun * 0.4},
        "tyres": {"exposedFraction": 1, "sunlitFraction": sun * 0.3, "contact": {"fraction": 0.01, "tempC": 12}},
        "battery-box": {"exposedFraction": 0.4, "sunlitFraction": 0.0}, "drive-motor": {"exposedFraction": 0.3, "sunlitFraction": 0.0},
    }
    return r


def pose(pid, title, desc, bones, met, breathing, drive, wheel=0.0):
    b = dict(bones); b["rear-wheel"] = {"angleDeg": wheel}; b["front-wheel"] = {"angleDeg": wheel}
    return {"id": pid, "title": title, "description": desc, "bones": b, "ground": "auto", "contacts": list(CONTACTS),
            "thermal": {"activityMet": met, "breathing": breathing, "operating": {"drive": drive}, "regions": bindings(drive), "emitters": {"body-plume": 0.6}}, "loads": []}


def poses(P):
    return [
        pose("driving", t("Driving", "Fahren"), t("Leaning forward off the backrest, both gloved hands on the handlebar grips, thumbs at the throttle lever, eyes on the path ahead; the scooter rolls at walking pace.",
                                                   "Von der Rückenlehne nach vorn geneigt, beide behandschuhten Hände an den Lenkergriffen, die Daumen am Gashebel, der Blick auf den Weg; das Elektromobil rollt in Schrittgeschwindigkeit."),
             P["driving"], 1.3, "driving", 0.3),
        pose("parked", t("Parked, hands in the lap", "Geparkt, Hände im Schoß"), t("Sitting back against the backrest with the motor off and both hands resting on the thighs.",
                                                                                     "Zurückgelehnt an der Rückenlehne, der Motor aus, beide Hände ruhen auf den Oberschenkeln."),
             P["parked"], 1.0, "rest-seated", 0.0),
        pose("greeting", t("Greeting a neighbour", "Nachbarn grüßen"), t("Stopped with the drive on standby, left hand on the handlebar, the right hand raised beside the head to wave to someone on the pavement.",
                                                                       "Angehalten, der Antrieb in Bereitschaft, die linke Hand am Lenker, die rechte neben dem Kopf erhoben, um jemandem auf dem Gehweg zuzuwinken."),
             P["greeting"], 1.3, "rest-seated", 0.05),
        pose("reaching-into-basket", t("Reaching into the basket", "In den Korb greifen"), t("Leaning forward with the left hand resting on the thigh, the right hand reaching into the front basket for a shopping bag.",
                                                                                              "Nach vorn gebeugt, die linke Hand ruht auf dem Oberschenkel, die rechte greift in den Frontkorb nach einer Einkaufstasche."),
             P["reaching-into-basket"], 1.6, "reaching", 0.0),
    ]


def drive_clip(P):
    keys = []
    n = 8
    for i in range(n + 1):
        tt = round(LOOP_S * i / n, 4)
        b = copy.deepcopy(P["driving"])
        ang = round(-TURN * TURNS * i / n, 3)
        b["rear-wheel"] = {"angleDeg": ang}; b["front-wheel"] = {"angleDeg": ang}
        b["head"] = {"angleDeg": round(P["driving"]["head"]["angleDeg"] + (0.8 if i % 4 == 2 else 0.0), 3)}
        keys.append({"t": tt, "pose": "driving", "ease": "linear", "bones": b})
    speed = TURNS * 2 * 3.141592653589793 * SP.R_WHEEL / LOOP_S
    return {"id": "drive-cruise", "title": t("Cruising at walking pace", "In Schrittgeschwindigkeit fahren"),
            "description": t(f"Driving straight on at {speed * 3.6:.1f} km/h (treadmill frame: the scooter stays, the ground passes): both wheel pairs roll without slip, five turns of the 250 mm wheels per loop; the drive runs at 30 % of its 400 W rating.",
                             f"Geradeausfahrt mit {speed * 3.6:.1f} km/h (Laufbandsystem: das Elektromobil steht, der Boden zieht vorbei): beide Radpaare rollen schlupffrei, fünf Umdrehungen der 250-mm-Räder je Schleife; der Antrieb läuft mit 30 % seiner Nennleistung von 400 W.".replace(f"{speed * 3.6:.1f}", f"{speed * 3.6:.1f}".replace(".", ","))),
            "durationS": LOOP_S, "fps": 30, "loop": True, "rootMotion": "none", "keys": keys,
            "contacts": [{"anchor": a, "fromS": 0, "toS": LOOP_S} for a in CONTACTS],
            "phases": [{"id": "cruise", "fromS": 0, "toS": LOOP_S, "title": t("Steady cruise", "Gleichmäßige Fahrt")}], "events": [],
            "tracks": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": LOOP_S, "v": 0}]},
                       {"target": "operating:drive", "ease": "smooth", "keys": [{"t": 0, "v": 0.3}, {"t": LOOP_S, "v": 0.3}]}]}, speed


def idle_clip():
    return {"id": "idle-breathing", "title": t("Resting on the parked scooter", "Rast auf dem geparkten Elektromobil"),
            "description": t("Sitting back with the motor off, quiet breathing at 15 per minute and a slight rise of the chest and shoulders.",
                             "Zurückgelehnt bei ausgeschaltetem Motor, ruhiges Atmen mit 15 Zügen pro Minute und leichtes Heben von Brustkorb und Schultern."),
            "durationS": 4.0, "fps": 30, "loop": True, "rootMotion": "none",
            "keys": [{"t": 0, "pose": "parked", "ease": "smooth"}, {"t": 1.7, "pose": "parked", "ease": "smooth", "bones": {"lumbar": {"angleDeg": 1.0}, "head": {"angleDeg": -1.0}}}, {"t": 4.0, "pose": "parked", "ease": "smooth"}],
            "contacts": [{"anchor": a, "fromS": 0, "toS": 4.0} for a in CONTACTS], "phases": [{"id": "rest", "fromS": 0, "toS": 4.0, "title": t("Resting", "Rasten")}], "events": [],
            "tracks": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 1}, {"t": 4.0, "v": 1}]}, {"target": "operating:drive", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 4.0, "v": 0}]}]}


WAVE = {}


def look_clip():
    return {"id": "stop-and-wave", "title": t("Stopping to wave", "Anhalten und winken"),
            "description": t("Stopped with the drive on standby: he sits up, lifts the right hand from the grip and waves twice to a neighbour, then takes the handlebar with both hands again.",
                             "Angehalten, der Antrieb in Bereitschaft: Er richtet sich auf, nimmt die rechte Hand vom Griff, winkt einem Nachbarn zweimal zu und fasst den Lenker dann wieder mit beiden Händen."),
            "durationS": 5.0, "fps": 30, "loop": False, "rootMotion": "none",
            "keys": [{"t": 0, "pose": "driving", "ease": "smooth"}, {"t": 1.4, "pose": "greeting", "ease": "smooth"}, {"t": 1.8, "pose": "greeting", "ease": "smooth", "bones": WAVE},
                     {"t": 2.2, "pose": "greeting", "ease": "smooth"}, {"t": 2.6, "pose": "greeting", "ease": "smooth", "bones": WAVE}, {"t": 3.0, "pose": "greeting", "ease": "smooth"},
                     {"t": 4.2, "pose": "driving", "ease": "smooth"}, {"t": 5.0, "pose": "driving", "ease": "smooth"}],
            "contacts": [{"anchor": a, "fromS": 0, "toS": 5.0} for a in CONTACTS],
            "phases": [{"id": "raise", "fromS": 0, "toS": 1.4, "title": t("Raising the hand", "Hand heben")}, {"id": "wave", "fromS": 1.4, "toS": 3.0, "title": t("Waving", "Winken")}, {"id": "return", "fromS": 3.0, "toS": 5.0, "title": t("Back to the handlebar", "Zurück an den Lenker")}],
            "events": [{"t": 1.8, "kind": "motion", "title": t("First wave", "Erstes Winken")}, {"t": 2.6, "kind": "motion", "title": t("Second wave", "Zweites Winken")}],
            "tracks": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 5.0, "v": 0}]}, {"target": "operating:drive", "ease": "smooth", "keys": [{"t": 0, "v": 0.05}, {"t": 5.0, "v": 0.05}]},
                       {"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 1.2}, {"t": 1.4, "v": 1.4}, {"t": 4.2, "v": 1.2}]}]}


def basket_clip():
    return {"id": "reach-into-basket", "title": t("Taking a bag from the basket", "Tasche aus dem Korb nehmen"),
            "description": t("Parked: he leans forward with the left hand braced on his thigh, reaches into the front basket with the right hand, then sits back with his hands in the lap.",
                             "Geparkt: Er beugt sich nach vorn, stützt die linke Hand auf den Oberschenkel, greift mit der rechten in den Frontkorb und lehnt sich dann mit den Händen im Schoß zurück."),
            "durationS": 6.0, "fps": 30, "loop": False, "rootMotion": "none",
            "keys": [{"t": 0, "pose": "parked", "ease": "smooth"}, {"t": 2.0, "pose": "reaching-into-basket", "ease": "smooth"}, {"t": 3.4, "pose": "reaching-into-basket", "ease": "smooth"}, {"t": 5.4, "pose": "parked", "ease": "smooth"}, {"t": 6.0, "pose": "parked", "ease": "smooth"}],
            "contacts": [{"anchor": a, "fromS": 0, "toS": 6.0} for a in CONTACTS],
            "phases": [{"id": "lean", "fromS": 0, "toS": 2.0, "title": t("Leaning forward", "Vorbeugen")}, {"id": "grasp", "fromS": 2.0, "toS": 3.4, "title": t("Grasping the bag", "Tasche fassen")}, {"id": "sit-back", "fromS": 3.4, "toS": 6.0, "title": t("Sitting back", "Zurücklehnen")}],
            "events": [{"t": 3.0, "kind": "motion", "title": t("Hand in the basket", "Hand im Korb")}],
            "tracks": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 1}, {"t": 1.0, "v": 2}, {"t": 5.0, "v": 1}, {"t": 6.0, "v": 1}]}, {"target": "operating:drive", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 6.0, "v": 0}]},
                       {"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 1.0}, {"t": 2.0, "v": 1.6}, {"t": 3.4, "v": 1.6}, {"t": 6.0, "v": 1.0}]}]}


MATERIALS = [
    ("abs-shell", t("Painted ABS body panels (burgundy)", "Lackierte ABS-Verkleidung (bordeauxrot)"), 0.9, 0.7, "literature", "ashrae-2021"),
    ("vinyl", t("Black vinyl upholstery", "Schwarzer Kunstlederbezug"), 0.9, 0.88, "authored-estimate", None),
    ("pu-tyre", t("Puncture-proof polyurethane tyre", "Pannensicherer Polyurethanreifen"), 0.93, 0.9, "literature", "ashrae-2021"),
    ("battery-case", t("Black polypropylene battery case", "Schwarzes Akkugehäuse aus Polypropylen"), 0.9, 0.88, "authored-estimate", None),
    ("motor-alloy", t("Die-cast aluminium transaxle housing, painted", "Lackiertes Aluminium-Druckgussgehäuse der Antriebsachse"), 0.85, 0.6, "authored-estimate", None),
    ("aluminium-rim", t("Silver-painted wheel rim", "Silbern lackierte Felge"), 0.6, 0.45, "authored-estimate", None),
    ("steel-tube", t("Powder-coated steel tube", "Pulverbeschichtetes Stahlrohr"), 0.9, 0.8, "literature", "ashrae-2021"),
    ("steel-wire", t("Coated steel wire basket", "Beschichteter Drahtkorb"), 0.88, 0.55, "authored-estimate", None),
    ("rubber-grip", t("Rubber handlebar grip", "Gummigriff"), 0.93, 0.9, "authored-estimate", None),
    ("lamp-lens", t("Polycarbonate headlamp lens", "Scheinwerferscheibe aus Polycarbonat"), 0.9, 0.2, "authored-estimate", None),
]

REFERENCES = [
    {"id": "din-33402-2", "citation": "DIN 33402-2:2020-12 Ergonomie - Körpermaße des Menschen - Teil 2: Werte; body height of men aged 61 to 65 (P50 1.74 m)."},
    {"id": "sorkin-1999", "citation": "Sorkin, J. D., Muller, D. C. and Andres, R. (1999). Longitudinal change in height of men and women: implications for interpretation of the body mass index. American Journal of Epidemiology 150(9), 969-977."},
    {"id": "en-12184-2022", "citation": "EN 12184:2022 Electrically powered wheelchairs, scooters and their chargers - Requirements and test methods; class B scooters for indoor and outdoor use."},
    {"id": "scooter-catalogues", "citation": "Manufacturer data sheets of mid-size four-wheel mobility scooters (6 km/h, 24 V transaxle 350-450 W, 2 x 12 V 36 Ah AGM, 70-90 kg with batteries, length 1.2-1.35 m, width 0.58-0.62 m, seat height 0.53-0.62 m); authored-estimate."},
    {"id": "ainsworth-2011", "citation": "Ainsworth, B. E. et al. (2011). 2011 Compendium of Physical Activities: a second update of codes and MET values. Medicine and Science in Sports and Exercise 43(8), 1575-1581; riding in a car or truck 1.3 MET, sitting quietly 1.3 MET."},
    {"id": "wong-2008", "citation": "Wong, J. Y. (2008). Theory of Ground Vehicles, 4th ed., Wiley; rolling resistance of solid tyres."},
]


def thermal(asset):
    th = asset["thermal"]
    skin = 0.007184 * SB.BODY ** 0.425 * (SB.STATURE * 100) ** 0.725
    bmr = (10 * SB.BODY + 6.25 * SB.STATURE * 100 - 5 * 78 + 5) * 4184 / 86400
    th["body"].update({"massKg": SB.BODY, "skinAreaM2": round(skin, 3), "basalMetabolismW": round(bmr, 1), "coreSetPointC": 36.7, "skinSetPointC": 33.9, "maxSkinBloodFlowLM2H": 70})
    th["operating"] = [{"id": "drive", "title": t("Drive power", "Antriebsleistung"), "levels": [{"v": 0, "title": t("Off", "Aus")}, {"v": 0.05, "title": t("Standby", "Bereitschaft")},
                                                                                               {"v": 0.3, "title": t("Cruising at 6 km/h", "Fahrt mit 6 km/h")}, {"v": 1, "title": t("Rated 400 W (ramp, kerb)", "Nennleistung 400 W (Rampe, Bordstein)")}]}]
    r = th["respiration"]
    r["patterns"] = [
        {"id": "rest-seated", "title": t("Seated at rest", "Ruhiges Sitzen"), "rateBpm": 15, "tidalVolumeL": 0.5, "inhaleFraction": 0.38, "pauseAfterInhaleFraction": 0.04, "exhaleFraction": 0.45, "pauseAfterExhaleFraction": 0.13, "route": "nose", "source": "authored-estimate", "reference": "icrp-66-1994"},
        {"id": "driving", "title": t("Driving, seated light arm work", "Fahren, leichte Armarbeit im Sitzen"), "rateBpm": 16, "tidalVolumeL": 0.58, "inhaleFraction": 0.4, "pauseAfterInhaleFraction": 0.03, "exhaleFraction": 0.47, "pauseAfterExhaleFraction": 0.1, "route": "nose", "source": "authored-estimate", "reference": "icrp-66-1994"},
        {"id": "reaching", "title": t("Leaning and reaching", "Vorbeugen und Greifen"), "rateBpm": 19, "tidalVolumeL": 0.7, "inhaleFraction": 0.42, "pauseAfterInhaleFraction": 0.03, "exhaleFraction": 0.48, "pauseAfterExhaleFraction": 0.07, "route": "nose-and-mouth", "source": "authored-estimate", "reference": "icrp-66-1994"}]
    R = {x["id"]: x for x in th["regions"]}
    R["cap"]["title"] = t("Knitted cap over scalp and ears", "Strickmütze über Kopfhaut und Ohren")
    R["cap"]["layers"][0]["title"] = t("Knitted wool cap", "Strickmütze aus Wolle")
    R["gloves"]["title"] = t("Knitted gloves over the hands", "Strickhandschuhe über den Händen")
    R["gloves"]["layers"][0]["title"] = t("Fingerless knitted gloves, lined", "Gefütterte fingerlose Strickhandschuhe")
    R["hand-skin"]["title"] = t("Bare fingertips", "Freie Fingerspitzen")
    j = R["jacket"]; j["title"] = t("Layered clothing on torso, neck and arms", "Geschichtete Kleidung an Rumpf, Hals und Armen")
    lay = {x["id"]: x for x in j["layers"]}
    lay["shirt"]["title"] = t("Flannel shirt and vest", "Flanellhemd und Unterhemd")
    lay["shirt"]["resistanceM2KW"] = round(0.2 * 0.155 * 1.2, 5)
    lay["fleece"]["title"] = t("Light fleece layer", "Leichte Fleeceschicht")
    lay["jacket"].update({"title": t("Hooded quilted jacket", "Gesteppte Kapuzenjacke"), "resistanceM2KW": round(0.42 * 0.155 * 1.2, 5)})
    tr = R["trousers"]; tr["title"] = t("Corduroy trousers over the legs", "Cordhose über den Beinen")
    tl = {x["id"]: x for x in tr["layers"]}
    tl["trousers"]["title"] = t("Corduroy trousers", "Cordhose")
    tl["briefs"]["title"] = t("Long johns", "Lange Unterhose")
    tl["briefs"]["resistanceM2KW"] = round(0.1 * 0.155 * 1.087, 5)
    sh = R["shoes"]; sh["title"] = t("Socks and leather walking shoes", "Socken und Lederhalbschuhe")
    sl = {x["id"]: x for x in sh["layers"]}
    sl["shoe"]["title"] = t("Leather walking shoe with velcro straps", "Lederhalbschuh mit Klettverschluss")
    sl["shoe"]["resistanceM2KW"] = round(0.04 * 0.155, 5)
    def region(rid, title, model, material, area, geometry, **kw):
        out = {"id": rid, "title": title, "kind": kw.pop("kind", "surface"), "model": model, "material": material, "areaM2": area, "characteristicLengthM": kw.pop("L", 0.3), "geometry": geometry,
               "skyViewFactor": kw.pop("sky", 0.5), "layers": [], "heatCapacityJM2K": kw.pop("cap", 5000), "source": "authored-estimate", "reference": "scooter-catalogues"}
        out.update(kw); return out
    th["regions"] += [
        region("shell", t("Body panels, deck, tiller and basket", "Verkleidung, Trittfläche, Lenksäule und Korb"), "passive", "abs-shell", 3.3, "horizontal-plate-up", cap=4000),
        region("upholstery", t("Seat, backrest, armrests and grips", "Sitz, Rückenlehne, Armlehnen und Griffe"), "passive", "vinyl", 1.3, "vertical-plate", cap=6000, sky=0.3),
        region("tyres", t("Four solid tyres 250 mm", "Vier Vollgummireifen 250 mm"), "passive", "pu-tyre", 0.65, "cylinder", kind="tyre", L=0.25, cap=8000, sky=0.4),
        region("battery-box", t("Battery box under the seat (2 x 12 V 36 Ah)", "Akkukasten unter dem Sitz (2 x 12 V 36 Ah)"), "operating", "battery-case", 0.5, "vertical-plate", cap=20000, sky=0.1,
               operating={"key": "drive", "heatW": 12}, reference="en-12184-2022"),
    ]
    th["regions"].append({"id": "drive-motor", "title": t("24 V transaxle drive motor with gearbox", "24-V-Antriebsmotor mit Getriebe an der Hinterachse"), "kind": "engine", "model": "operating", "material": "motor-alloy",
                          "areaM2": 0.21, "characteristicLengthM": 0.13, "geometry": "cylinder", "skyViewFactor": 0.1, "layers": [], "heatCapacityJM2K": 18000,
                          "operating": {"key": "drive", "heatW": 90}, "ratedPowerW": 400, "ratedPowerBasis": "rated-output", "source": "authored-estimate", "reference": "en-12184-2022"})
    th["passengers"] = []
    th["assumptions"] = [
        t(f"Skin area {skin:.3f} m2 after DuBois and DuBois (1916) for {SB.BODY:.0f} kg and {SB.STATURE} m; basal metabolism {bmr:.0f} W after Mifflin-St Jeor (1990) for a man of 78; skin fractions as for the adult figure he is derived from; an older body regulates less: lower set points and a maximum skin blood flow of 70 instead of 90 L/(m2 h) (Kenney and Munce 2003, authored). No impaired thermoregulation beyond age is modelled (the model is not clinical).",
          f"Hautfläche {skin:.3f} m2 nach DuBois und DuBois (1916) für {SB.BODY:.0f} kg und {SB.STATURE} m; Grundumsatz {bmr:.0f} W nach Mifflin-St Jeor (1990) für einen 78-jährigen Mann; Hautanteile wie bei der Erwachsenenfigur, von der er abgeleitet ist; ein älterer Körper regelt schwächer: niedrigere Sollwerte und eine maximale Hautdurchblutung von 70 statt 90 L/(m2 h) (Kenney und Munce 2003, geschätzt). Eine über das Alter hinaus gestörte Thermoregulation wird nicht modelliert (das Modell ist nicht klinisch).".replace(f"{skin:.3f}", f"{skin:.3f}".replace(".", ",")).replace(f"{SB.STATURE} m", f"{SB.STATURE} m".replace(".", ","))),
        t("Garment values are authored estimates after the ISO 9920 tables (flannel shirt with vest 0.20, light fleece 0.25, hooded quilted jacket 0.42, corduroy trousers 0.24, long johns 0.10, leather shoes 0.04 clo, knitted cap and gloves 0.2 and 0.25 clo); resistances are referred to the outer area.",
          "Kleidungswerte sind Schätzwerte nach den Tabellen von ISO 9920 (Flanellhemd mit Unterhemd 0,20, leichtes Fleece 0,25, gesteppte Kapuzenjacke 0,42, Cordhose 0,24, lange Unterhose 0,10, Lederschuhe 0,04 clo, Strickmütze und Handschuhe 0,2 und 0,25 clo); Widerstände sind auf die Außenfläche bezogen."),
        t("Driving a scooter is seated light arm work of 1.3 met (ISO 8996; the Compendium of Physical Activities gives riding in a car 1.3 MET). The drive nameplate is 400 W rated output of the 24 V transaxle motor (catalogue-typical for mid-size class-B scooters after EN 12184); at the 30 % cruise level the motor and gearbox release up to 90 W at full rating and the batteries 12 W, so the transaxle housing is the warm part of the thermal view; body panels, seat and tyres are passive.",
          "Das Fahren eines Elektromobils ist leichte Armarbeit im Sitzen mit 1,3 met (ISO 8996; das Compendium of Physical Activities nennt Autofahren mit 1,3 MET). Das Typenschild des Antriebs ist 400 W Nennabgabeleistung des 24-V-Achsmotors (katalogtypisch für mittelgroße Elektromobile der Klasse B nach EN 12184); bei Fahrt mit 30 % geben Motor und Getriebe bis zu 90 W bei Nennleistung ab und die Akkus 12 W, daher ist das Achsgehäuse in der Thermoansicht der warme Teil; Verkleidung, Sitz und Reifen sind passiv."),
    ]


def finish(asset, P):
    for m in MATERIALS:
        e = {"id": m[0], "title": m[1], "emissivity": m[2], "solarAbsorptivity": m[3], "source": m[4]}
        if m[5]: e["reference"] = m[5]
        asset["materials"].append(e)
    for m in asset["materials"]:
        if m["id"] == "fleece": m["title"] = t("Fleece and quilted nylon", "Fleece und gesteppter Nylon")
        if m["id"] == "denim": m["title"] = t("Corduroy", "Cord")
        if m["id"] == "canvas-sneaker": m["title"] = t("Leather walking shoe", "Lederhalbschuh")
        if m["id"] == "leather-glove": m["title"] = t("Knitted wool glove", "Strickhandschuh aus Wolle")
    asset["anchors"] += [
        {"id": "patch-rear", "bone": "chassis", "x": 0, "y": -SB.AXLE, "kind": "contact", "title": t("Rear tyre contact patches", "Aufstandsflächen der Hinterreifen"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.04, "footprintWidthM": 0.07, "frictionCoefficient": 0.8},
        {"id": "patch-front", "bone": "chassis", "x": SP.WHEELBASE, "y": -SB.AXLE, "kind": "contact", "title": t("Front tyre contact patches", "Aufstandsflächen der Vorderreifen"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.04, "footprintWidthM": 0.07, "frictionCoefficient": 0.8},
        {"id": "grip-l", "bone": "hand-l", "x": round(0.082 * SB.S, 4), "y": round(-0.026 * SB.S, 4), "kind": "attachment", "title": t("Left hand on the handlebar grip", "Linke Hand am Lenkergriff")},
        {"id": "grip-r", "bone": "hand-r", "x": round(0.082 * SB.S, 4), "y": round(-0.026 * SB.S, 4), "kind": "attachment", "title": t("Right hand on the handlebar grip and throttle lever", "Rechte Hand am Lenkergriff und Gashebel")},
    ]
    asset["poses"] = poses(P)
    dc, speed = drive_clip(P)
    g = P["greeting"]
    WAVE.update({"hand-r": dict(g["hand-r"], angleDeg=round(g["hand-r"]["angleDeg"] + 25, 3)), "forearm-r": dict(g["forearm-r"], angleDeg=round(max(0, g["forearm-r"]["angleDeg"] - 12), 3))})
    asset["clips"] = [dc, idle_clip(), look_clip(), basket_clip()]
    thermal(asset)
    asset["structure"]["assumptions"] = [
        t("The four tyre patches are the ground contacts, lumped into a rear and a front patch; the person is rigidly attached to the scooter through seat and footboard and the weight is shared according to the elastic-support model of the solver (about two thirds on the rear axle under the seat and the batteries).",
          "Die vier Reifenaufstandsflächen sind die Bodenkontakte, zu einer hinteren und einer vorderen zusammengefasst; die Person ist über Sitz und Trittbrett starr mit dem Elektromobil verbunden, und das Gewicht verteilt sich nach dem Elastizitätsmodell des Lösers (etwa zwei Drittel auf der Hinterachse unter Sitz und Akkus)."),
        t("The scooter stays in place while the ground advances (treadmill frame); the traction force of the transaxle and the rolling resistance are internal and not modelled as external loads. Scooter 74 kg (chassis with seat, tiller and 23 kg of batteries 66 kg, wheel pairs 4.2 and 3.8 kg) and rider 76 kg.",
          "Das Elektromobil bleibt stehen, während der Boden vorbeiläuft (Laufbandsystem); Zugkraft der Antriebsachse und Rollwiderstand sind innere Kräfte und nicht als äußere Lasten modelliert. Elektromobil 74 kg (Fahrgestell mit Sitz, Lenksäule und 23 kg Akkus 66 kg, Radpaare 4,2 und 3,8 kg) und Fahrer 76 kg."),
    ]
    asset["structure"]["supports"] = []
    pr = asset["provenance"]
    pr.update({"author": "Semio staffage human worker HW-06", "created": "2026-11-22", "method": "handcrafted"})
    pr["references"] = pr["references"] + REFERENCES + [{"id": "kenney-munce-2003", "citation": "Kenney, W. L. and Munce, T. A. (2003). Invited review: aging and human temperature regulation. Journal of Applied Physiology 95(6), 2598-2603."}]
    pr["notes"] = [
        t(f"A man of 78, {SB.STATURE:.2f} m and {SB.BODY:.0f} kg: stature after DIN 33402-2 (men 61 to 65, P50 1.74 m) minus the age loss of stature after 65 (Sorkin et al. 1999). He is derived from the wheelchair user's handcrafted seated body: every person bone, 2D shape of the four scales and 3D solid is scaled by {SB.S:.4f} (cross-sections that sat exactly at the smallest feature of a scale keep it) and set on the scooter so that his buttocks rest on the seat cushion 0.56 m above the ground and his soles on the footboard at 0.155 m; segment masses are the de Leva (1996) male fractions of that figure times {SB.BODY / SB.BODY_OLD:.3f}. He wears a knitted cap, fingerless knitted gloves, a hooded quilted jacket over fleece and flannel, corduroy trousers and leather walking shoes.",
          f"Ein 78-jähriger Mann von {SB.STATURE:.2f} m und {SB.BODY:.0f} kg: Körperhöhe nach DIN 33402-2 (Männer 61 bis 65, P50 1,74 m) abzüglich des Altersverlusts nach 65 (Sorkin et al. 1999). Er ist vom handgearbeiteten Sitzkörper der rollstuhlnutzenden Person abgeleitet: Jeder Personenknochen, jede 2D-Form der vier Maßstäbe und jeder 3D-Körper ist mit {SB.S:.4f} skaliert (Querschnitte, die genau auf dem kleinsten Merkmal eines Maßstabs lagen, behalten es) und so auf das Elektromobil gesetzt, dass sein Gesäß 0,56 m über dem Boden auf dem Sitzpolster und seine Sohlen auf dem Trittbrett in 0,155 m Höhe liegen; die Segmentmassen sind die männlichen Anteile nach de Leva (1996) dieser Figur mal {SB.BODY / SB.BODY_OLD:.3f}. Er trägt eine Strickmütze, fingerlose Strickhandschuhe, eine gesteppte Kapuzenjacke über Fleece und Flanell, eine Cordhose und Lederhalbschuhe.".replace(f"{SB.S:.4f}", f"{SB.S:.4f}".replace(".", ",")).replace(f"{SB.BODY / SB.BODY_OLD:.3f}", f"{SB.BODY / SB.BODY_OLD:.3f}".replace(".", ",")).replace(f"{SB.STATURE:.2f} m", f"{SB.STATURE:.2f} m".replace(".", ","))),
        t("The scooter is a handcrafted mid-size four-wheel model after catalogue values and EN 12184 class B: wheelbase 0.92 m, four 250 mm (10 inch) puncture-proof tyres on rear and front wheel pairs that turn with the distance rolled, a 24 V transaxle with a 400 W rated-output motor between the rear wheels, two 12 V 36 Ah batteries in a box under the seat, a burgundy ABS body with rear shroud, footboard, front nose with fenders, bumper and headlamp, a tiller with console and handlebar raked back to the rider, a wire basket in front of it, a captain's seat on a post with a 12-degree backrest and flip-up armrests; 1.30 m long, 0.60 m wide, 74 kg. Every pose solves both arms in three dimensions so that the gloved hands close round the grips (or reach into the basket).",
          "Das Elektromobil ist ein handgearbeitetes mittelgroßes vierrädriges Modell nach Katalogwerten und EN 12184 Klasse B: Radstand 0,92 m, vier pannensichere 250-mm-Reifen (10 Zoll) an hinterem und vorderem Radpaar, die sich mit der gerollten Strecke drehen, eine 24-V-Antriebsachse mit einem Motor von 400 W Nennabgabe zwischen den Hinterrädern, zwei 12-V-36-Ah-Akkus in einem Kasten unter dem Sitz, eine bordeauxrote ABS-Verkleidung mit Heckhaube, Trittbrett, Bugnase mit Kotflügeln, Stoßfänger und Scheinwerfer, eine zum Fahrer zurückgeneigte Lenksäule mit Bedienpult und Lenker, davor ein Drahtkorb, ein Kapitänssitz auf einer Säule mit 12 Grad geneigter Rückenlehne und hochklappbaren Armlehnen; 1,30 m lang, 0,60 m breit, 74 kg. Jede Haltung löst beide Arme dreidimensional, so dass sich die behandschuhten Hände um die Griffe schließen (oder in den Korb greifen)."),
    ]
    asset["presentation"] = {"defaultPose": "driving", "defaultClip": "drive-cruise", "defaultScale": 100}
    asset["id"] = "mobility-scooter-user"
    asset["title"] = t("Mobility Scooter User", "Person auf einem Elektromobil")
    asset["summary"] = t("A 78-year-old man in a knitted cap and quilted jacket drives a four-wheel mobility scooter with a front basket: the scooter's chassis is the root, both wheel pairs roll with the distance, the 400 W transaxle motor and the batteries under the seat warm up when driving; with cruising, a shoulder check, reaching into the basket and a rest with the motor off.",
                         "Ein 78-jähriger Mann mit Strickmütze und Steppjacke fährt ein vierrädriges Elektromobil mit Frontkorb: Das Fahrgestell ist die Wurzel des Modells, beide Radpaare rollen mit der Strecke, der 400-W-Achsmotor und die Akkus unter dem Sitz erwärmen sich beim Fahren; mit Fahrt, Schulterblick, Griff in den Korb und Rast bei ausgeschaltetem Motor.")
    asset["tags"] = ["human", "adult", "elderly", "seated", "mobility-scooter", "mobility-aid", "vehicle", "male", "powered"]
    by = {b["id"]: b for b in asset["rig"]["bones"]}
    by["torso"]["title"] = t("Thorax with quilted jacket", "Brustkorb mit Steppjacke")
    by["head"]["title"] = t("Head with knitted cap", "Kopf mit Strickmütze")
    for s in ("l", "r"):
        by[f"hand-{s}"]["title"] = t(f"{'Left' if s == 'l' else 'Right'} hand in a fingerless glove", f"{'Linke' if s == 'l' else 'Rechte'} Hand im fingerlosen Handschuh")
        by[f"foot-{s}"]["title"] = t(f"{'Left' if s == 'l' else 'Right'} foot on the footboard", f"{'Linker' if s == 'l' else 'Rechter'} Fuß auf dem Trittbrett")
    sym = asset["symbols"][0]
    sym["id"] = "scooter-position-marker"
    sym["title"] = t("Mobility scooter marker", "Elektromobil als Marke")
    sym["description"] = t("Position dot with a direction arrow for crowded overview plans; the dot marks the rear axle.", "Positionspunkt mit Richtungspfeil für belebte Übersichtspläne; der Punkt markiert die Hinterachse.")
    for view in sym["geometry"].values():
        for s in view: s["bone"] = "chassis"
    import scoot_decisions as D
    D.apply(asset)
    return speed
