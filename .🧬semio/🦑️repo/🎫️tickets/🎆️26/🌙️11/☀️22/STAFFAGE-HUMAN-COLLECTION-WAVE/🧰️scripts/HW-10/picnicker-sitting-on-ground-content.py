"""📝️ Identity, thermal scenario (sunny park, blanket on grass), clips and per-scale decisions of picnicker-sitting-on-ground."""
from hw10lib import *

G = 9.80665
MASS = 68.0


def identity(a):
    a["title"] = text("Picnicker Sitting on the Ground", "Picknickende Person auf dem Boden")
    a["summary"] = text(
        "A bearded man of 1.70 m at a picnic in the park, sitting cross-legged on a gingham blanket of 1.4 by 1.2 m and eating an apple: check shirt, jeans and leather boots. Three poses (biting the apple cross-legged, resting it on the knee, leaning back on one hand with the knees drawn up) and two loops (eating the apple, idle breathing) at 1.0 to 1.3 met in spring sun; the blanket carries him on the grass.",
        "Ein bärtiger Mann von 1,70 m beim Picknick im Park, der im Schneidersitz auf einer karierten Decke von 1,4 mal 1,2 m sitzt und einen Apfel isst: kariertes Hemd, Jeans und Lederstiefel. Drei Posen (im Schneidersitz in den Apfel beißen, den Apfel auf dem Knie ablegen, auf eine Hand zurückgelehnt mit angezogenen Knien) und zwei Schleifen (Apfel essen, ruhiges Atmen) bei 1,0 bis 1,3 met in der Frühlingssonne; die Decke trägt ihn auf dem Gras.")
    a["tags"] = ["human", "adult", "man", "sitting", "sitting-on-ground", "cross-legged", "picnic", "eating", "apple", "blanket", "park"]
    a["presentation"] = {"defaultPose": "cross-legged-eating", "defaultClip": "eating-an-apple", "defaultScale": 100}
    prov = a["provenance"]
    prov["author"] = "Semio staffage human collection worker HW-10"
    prov["created"] = "2026-11-22"
    prov["references"] = [r for r in prov["references"] if r["id"] != "guitar-spec"] + [
        {"id": "iso-8996-2021", "citation": "ISO 8996:2021 Ergonomics of the thermal environment, determination of metabolic rate; Table A.1 sitting at ease 58 W/m2 (1.0 met)."},
        {"id": "icrp-66-1994", "citation": "ICRP (1994). Human respiratory tract model for radiological protection. ICRP Publication 66, Annals of the ICRP 24(1-3); reference ventilation rates by activity (guidance, values authored)."}]
    prov["notes"] = text(
        "Derived from the archive's seated guitarist: rig, outlines, 3D bodies and clothing scaled as one by 1.70/1.76 to a man of 1.70 m and 68 kg (male segment fractions of de Leva 1996 kept); guitar and stool are removed, the root of the rig is now the picnic blanket on the ground and the hip-lift joint lowers the pelvis until the seat bones rest on it. The cross-legged, resting and leaning-back postures were fitted bone by bone with a Nelder-Mead search on the rig (knees out to 0.31 m either side, ankles under the opposite knee, scripts in the ticket folder, cached as a design input); the 76 mm apple is a new rigid bone in the right hand held by curled phalanges.",
        "Abgeleitet vom sitzenden Gitarristen des Archivs: Skelett, Umrisse, 3D-Körper und Kleidung gemeinsam um 1,70/1,76 auf einen Mann von 1,70 m und 68 kg skaliert (männliche Segmentanteile nach de Leva 1996 beibehalten); Gitarre und Hocker sind entfernt, die Wurzel des Skeletts ist nun die Picknickdecke auf dem Boden, und das Hüfthubgelenk senkt das Becken, bis die Sitzbeinhöcker darauf ruhen. Schneidersitz, Ruhe- und Rücklehnhaltung wurden Knochen für Knochen mit einer Nelder-Mead-Suche am Skelett eingepasst (Knie bis 0,31 m zu jeder Seite, Knöchel unter dem anderen Knie, Skripte im Ticketordner, als Entwurfsgrundlage zwischengespeichert); der 76 mm große Apfel ist ein neuer starrer Knochen in der rechten Hand, gehalten von gebeugten Fingergliedern.")
    a["structure"]["assumptions"] = [
        text("Segments are point masses at the de Leva centres (male table, 68 kg); the 180 g apple is a carried load in the right hand, the 0.9 kg blanket lies on the grass.", "Segmente sind Punktmassen in den Schwerpunkten nach de Leva (Männertabelle, 68 kg); der 180 g schwere Apfel ist eine getragene Last in der rechten Hand, die 0,9 kg schwere Decke liegt auf dem Gras."),
        text("Sitting cross-legged, the body rests on the seat bones and the outer edges of both feet on the blanket; leaning back, on the seat bones, both flat boots and the left palm. The blanket is the unilateral support on the ground in every pose.", "Im Schneidersitz ruht der Körper auf den Sitzbeinhöckern und den Außenkanten beider Füße auf der Decke; zurückgelehnt auf den Sitzbeinhöckern, beiden flachen Stiefeln und der linken Handfläche. Die Decke ist in jeder Pose das einseitige Auflager auf dem Boden.")]
    bone(a, "hip-lift")["title"] = text("Hip height above the blanket", "Hüfthöhe über der Decke")
    bone(a, "hip-shift")["title"] = text("Hip shift on the blanket", "Hüftverschiebung auf der Decke")
    for an in a["anchors"]:
        if an["id"] == "seat":
            an["title"] = text("Seat bones on the blanket", "Sitzbeinhöcker auf der Decke")


def materials(a):
    for m in a["materials"]:
        if m["id"] == "flannel":
            m["title"] = text("Brushed cotton check shirt", "Kariertes Hemd aus angerauter Baumwolle")
    a["materials"] += [
        {"id": "wool-blanket", "title": text("Wool picnic blanket with a water-proof back", "Woll-Picknickdecke mit wasserdichter Rückseite"), "emissivity": 0.92, "solarAbsorptivity": 0.6, "densityKgM3": 150, "specificHeatJKgK": 1360, "source": "authored-estimate", "reference": "incropera-2011"},
        {"id": "apple", "title": text("Apple", "Apfel"), "emissivity": 0.95, "solarAbsorptivity": 0.65, "densityKgM3": 840, "specificHeatJKgK": 3600, "source": "authored-estimate", "reference": "incropera-2011"}]


def thermal(a):
    t = a["thermal"]
    t["environment"] = {"airTempC": 22, "meanRadiantTempC": 24, "skyTempC": 4, "groundTempC": 19, "relativeHumidity": 0.5, "windMS": 1.0, "windDirectionDeg": 180, "solarWM2": 650, "diffuseFraction": 0.25, "pressurePa": 101325}
    skin = 0.007184 * MASS ** 0.425 * 170 ** 0.725
    t["body"].update({"massKg": MASS, "skinAreaM2": round(skin, 3), "basalMetabolismW": round((10 * MASS + 6.25 * 170 - 5 * 28 + 5) * 4184 / 86400, 1), "source": "authored-estimate", "reference": "mifflin-1990"})
    regs = {r["id"]: r for r in t["regions"]}
    regs["shirt-torso"]["title"] = text("Check shirt over T-shirt, trunk", "Kariertes Hemd über T-Shirt, Rumpf")
    regs["shirt-torso"]["layers"][2].update({"title": text("Brushed cotton check shirt (0.25 clo)", "Kariertes Baumwollhemd (0,25 clo)"), "resistanceM2KW": 0.0388})
    regs["shirt-sleeves"]["title"] = text("Check shirt sleeves, rolled to the forearm", "Hemdärmel, bis zum Unterarm")
    regs["shirt-sleeves"]["layers"][1].update({"title": text("Check shirt sleeve (0.25 clo)", "Hemdärmel (0,25 clo)"), "resistanceM2KW": 0.0388})
    t["regions"] += [
        {"id": "blanket", "title": text("Picnic blanket on the grass", "Picknickdecke auf dem Gras"), "kind": "surface", "model": "passive", "material": "wool-blanket", "areaM2": 1.68, "characteristicLengthM": 1.2, "geometry": "horizontal-plate-up", "skyViewFactor": 0.85,
         "layers": [], "heatCapacityJM2K": 1200, "ratedPowerW": 0, "ratedPowerBasis": "passive", "source": "authored-estimate", "reference": "incropera-2011"},
        {"id": "apple", "title": text("Apple", "Apfel"), "kind": "surface", "model": "passive", "material": "apple", "areaM2": 0.018, "characteristicLengthM": 0.076, "geometry": "sphere", "skyViewFactor": 0.5, "layers": [],
         "heatCapacityJM2K": 38000, "ratedPowerW": 0, "ratedPowerBasis": "passive", "source": "authored-estimate", "reference": "incropera-2011"}]
    resp = t["respiration"]
    resp["patterns"] = [resp["patterns"][0], {"id": "chewing", "title": text("Breathing through the nose while chewing", "Nasenatmung beim Kauen"), "rateBpm": 15, "tidalVolumeL": 0.45, "inhaleFraction": 0.36, "pauseAfterInhaleFraction": 0.03, "exhaleFraction": 0.47, "pauseAfterExhaleFraction": 0.14, "route": "nose", "source": "authored-estimate", "reference": "icrp-66-1994"}]
    t["assumptions"] = [
        text(f"DuBois skin area {t['body']['skinAreaM2']} m2 for 1.70 m and 68 kg; basal metabolism after Mifflin-St Jeor for a 28-year-old man. A check shirt over a T-shirt (0.34 clo trunk, 0.25 clo sleeves), jeans (0.26 clo), leather boots with socks, short hair and beard; garment values are authored estimates guided by ISO 9920 (1 clo = 0.155 m2K/W), layer resistances include the enclosed air.",
             f"DuBois-Hautfläche {str(t['body']['skinAreaM2']).replace('.', ',')} m2 für 1,70 m und 68 kg; Grundumsatz nach Mifflin-St Jeor für einen 28-jährigen Mann. Ein kariertes Hemd über T-Shirt (0,34 clo Rumpf, 0,25 clo Ärmel), Jeans (0,26 clo), Lederstiefel mit Socken, kurzes Haar und Bart; die Kleidungswerte sind Autorenschätzungen nach ISO 9920 (1 clo = 0,155 m2K/W), die Schichtwiderstände enthalten die eingeschlossene Luft."),
        text("Sitting at ease on the ground is 1.0 met (ISO 8996 Table A.1, 58 W/m2); eating, with the arm lifting the apple and the jaw at work, is taken as 1.3 met, below the 1.5 MET of the Ainsworth compendium for eating while sitting (code 13030), all authored. The reference case is a sunny spring afternoon (22 degrees C, 650 W/m2 sun, 25 % diffuse); the sunlit fractions of the poses are authored for sun from ahead and above.",
             "Entspanntes Sitzen auf dem Boden sind 1,0 met (ISO 8996 Tabelle A.1, 58 W/m2); Essen, bei dem der Arm den Apfel hebt und der Kiefer arbeitet, wird mit 1,3 met angesetzt, unter den 1,5 MET des Ainsworth-Kompendiums für Essen im Sitzen (Code 13030), alles geschätzt. Der Referenzfall ist ein sonniger Frühlingsnachmittag (22 Grad C, 650 W/m2 Sonne, 25 % diffus); die besonnten Anteile der Posen sind für Sonne von vorn oben geschätzt."),
        text("Blanket and apple are passive: the blanket (wool on a water-proof back, about 1.2 kJ/(m2 K)) lies on grass at 19 degrees C and warms in the sun; the seat and the legs touch it, the apple (180 g, about 38 kJ/(m2 K)) stays near the air temperature. Neither has power of its own.",
             "Decke und Apfel sind passiv: Die Decke (Wolle auf wasserdichter Rückseite, etwa 1,2 kJ/(m2 K)) liegt auf Gras von 19 Grad C und erwärmt sich in der Sonne; Gesäß und Beine berühren sie, der Apfel (180 g, etwa 38 kJ/(m2 K)) bleibt nahe der Lufttemperatur. Keiner von beiden hat eine eigene Leistung.")]


POSES = {
    "cross-legged-eating": (text("Biting the apple cross-legged", "Im Schneidersitz in den Apfel beißen"), text("Sitting cross-legged on the blanket, the pelvis rolled back on the seat bones and the trunk upright; the knees fall out to about 0.31 m either side, each ankle lies under the opposite knee on the outer edge of the foot. The right hand lifts the apple to the mouth, the left hand rests on the left knee.", "Im Schneidersitz auf der Decke, das Becken auf den Sitzbeinhöckern zurückgerollt und der Rumpf aufrecht; die Knie fallen etwa 0,31 m zu jeder Seite, jeder Knöchel liegt auf der Außenkante des Fußes unter dem anderen Knie. Die rechte Hand hebt den Apfel zum Mund, die linke Hand ruht auf dem linken Knie."), 1.3, "chewing", ["blanket-ground", "seat", "ankle-l", "ankle-r"]),
    "cross-legged-resting": (text("Resting with the apple on the knee", "Ruhen mit dem Apfel auf dem Knie"), text("The same cross-legged seat with the trunk leaning a little forward and the head bowed; the right hand rests the apple on the right knee, the left hand on the left knee.", "Derselbe Schneidersitz mit leicht vorgeneigtem Rumpf und gesenktem Kopf; die rechte Hand legt den Apfel auf dem rechten Knie ab, die linke Hand liegt auf dem linken Knie."), 1.0, "rest", ["blanket-ground", "seat", "ankle-l", "ankle-r"]),
    "leaning-back": (text("Leaning back on one hand", "Auf eine Hand zurückgelehnt"), text("Sitting with the knees drawn up and both boots flat on the blanket; the trunk leans back onto the straight left arm, whose palm lies on the blanket behind the hip, the right hand holds the apple over the right knee.", "Sitzend mit angezogenen Knien und beiden Stiefeln flach auf der Decke; der Rumpf lehnt sich auf den gestreckten linken Arm zurück, dessen Handfläche hinter der Hüfte auf der Decke liegt, die rechte Hand hält den Apfel über dem rechten Knie."), 1.0, "rest", ["blanket-ground", "seat", "heel-l", "contact-l", "heel-r", "contact-r", "palm-l"]),
}


def bind_poses(a):
    for p in a["poses"]:
        title, desc, met, breath, contacts = POSES[p["id"]]
        p.update({"title": title, "description": desc, "ground": "auto", "contacts": contacts})
        sun = {"face": 0.5, "hair": 0.6, "shirt-torso": 0.45, "shirt-sleeves": 0.45, "hands": 0.4, "jeans": 0.5, "boots": 0.5, "blanket": 0.65, "apple": 0.4}
        p["thermal"] = {"activityMet": met, "breathing": breath,
                        "regions": region_binding(a, {"jeans": 0.8, "boots": 0.85, "hands": 0.75, "apple": 0.85, "blanket": 0.75},
                                                  {**{k: {"sunlitFraction": v} for k, v in sun.items()},
                                                   "jeans": {"sunlitFraction": 0.5, "contact": {"fraction": 0.2, "tempC": 24}}, "boots": {"sunlitFraction": 0.5, "contact": {"fraction": 0.1, "tempC": 24}},
                                                   "blanket": {"sunlitFraction": 0.65, "contact": {"fraction": 1.0, "tempC": 19}}}),
                        "emitters": {"plume": 0.5}}
        p["loads"] = [{"id": "apple-weight", "kind": "carried", "title": text("Weight of the apple", "Gewicht des Apfels"), "anchor": "apple-cog", "fxN": 0, "fyN": round(-0.18 * G, 3), "source": "authored-estimate"}]


def clips(a):
    seated = lambda d, ids: [{"anchor": x, "fromS": 0, "toS": d} for x in ids]
    head = pose(a, "cross-legged-resting")["bones"].get("head", {}).get("angleDeg", -6)
    chew = []
    t, i = 5.2, 0
    while t < 8.7:
        chew.append({"t": round(t, 3), "pose": "cross-legged-resting", "ease": "smooth", "bones": {"head": {"angleDeg": head + (1.6 if i % 2 == 0 else -0.4)}}})
        t += 0.333
        i += 1
    eat = {"id": "eating-an-apple", "title": text("Eating an apple", "Einen Apfel essen"),
           "description": text("A nine-second loop sitting cross-legged: the apple rests on the right knee, rises to the mouth, he bites, lowers it to the knee and chews with small nods at about 1.5 chews per second; seat and feet stay on the blanket.",
                               "Eine Schleife von neun Sekunden im Schneidersitz: Der Apfel ruht auf dem rechten Knie, steigt zum Mund, er beißt ab, senkt ihn auf das Knie und kaut mit kleinen Nickbewegungen bei etwa 1,5 Kaubewegungen pro Sekunde; Gesäß und Füße bleiben auf der Decke."),
           "durationS": 9, "fps": 30, "loop": True, "rootMotion": "none",
           "keys": [{"t": 0, "pose": "cross-legged-resting", "ease": "smooth"}, {"t": 1.2, "pose": "cross-legged-resting", "ease": "smooth"}, {"t": 2.6, "pose": "cross-legged-eating", "ease": "smooth"},
                    {"t": 3.2, "pose": "cross-legged-eating", "ease": "smooth", "bones": {"head": {"angleDeg": -2}, "neck": {"angleDeg": -7}}}, {"t": 3.6, "pose": "cross-legged-eating", "ease": "smooth"}, {"t": 4.8, "pose": "cross-legged-resting", "ease": "smooth"}] + chew + [{"t": 9, "pose": "cross-legged-resting", "ease": "smooth"}],
           "contacts": seated(9, ["blanket-ground", "seat", "ankle-l", "ankle-r"]),
           "phases": [{"id": "rest", "fromS": 0, "toS": 1.2, "title": text("Apple on the knee", "Apfel auf dem Knie")}, {"id": "raise", "fromS": 1.2, "toS": 3.2, "title": text("Raising the apple", "Apfel heben")},
                      {"id": "bite", "fromS": 3.2, "toS": 3.6, "title": text("Biting", "Abbeißen")}, {"id": "chew", "fromS": 3.6, "toS": 9, "title": text("Lowering and chewing", "Senken und Kauen")}],
           "events": [{"t": 3.2, "kind": "motion", "title": text("Teeth bite into the apple", "Die Zähne beißen in den Apfel"), "target": "apple"}],
           "tracks": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 1}, {"t": 9, "v": 1}]}, {"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 1.1}, {"t": 2.6, "v": 1.3}, {"t": 4.8, "v": 1.2}, {"t": 9, "v": 1.1}]}]}
    idle = {"id": "idle-breathing", "title": text("Idle breathing", "Ruhiges Atmen"),
            "description": text("A five-second loop sitting cross-legged with the apple resting on the knee: the chest rises and falls with quiet breathing, the head settles in a small slow nod; seat and ankles stay on the blanket.",
                                "Eine Schleife von fünf Sekunden im Schneidersitz, der Apfel ruht auf dem Knie: Die Brust hebt und senkt sich mit ruhigem Atmen, der Kopf nickt klein und langsam; Gesäß und Knöchel bleiben auf der Decke."),
            "durationS": 5, "fps": 30, "loop": True, "rootMotion": "none",
            "keys": [{"t": 0, "pose": "cross-legged-resting", "ease": "smooth"}, {"t": 1.25, "pose": "cross-legged-resting", "ease": "smooth", "bones": {"head": {"angleDeg": -4.5}}}, {"t": 2.5, "pose": "cross-legged-resting", "ease": "smooth", "bones": {"head": {"angleDeg": -7}}},
                     {"t": 3.75, "pose": "cross-legged-resting", "ease": "smooth", "bones": {"head": {"angleDeg": -5}}}, {"t": 5, "pose": "cross-legged-resting", "ease": "smooth"}],
            "contacts": seated(5, ["blanket-ground", "seat", "ankle-l", "ankle-r"]), "phases": [{"id": "rest", "fromS": 0, "toS": 5, "title": text("Resting in the sun", "Ruhen in der Sonne")}], "events": [],
            "tracks": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 5, "v": 0}]}, {"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 1.0}, {"t": 5, "v": 1.0}]}]}
    a["clips"] = [eat, idle]


def decisions(a):
    import importlib.util
    spec = importlib.util.spec_from_file_location("dec", f"{H}/picnicker-sitting-on-ground-decisions.py")
    dec = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(dec)
    dec.apply(a)


def apply(a):
    identity(a)
    materials(a)
    thermal(a)
    bind_poses(a)
    clips(a)
    decisions(a)
