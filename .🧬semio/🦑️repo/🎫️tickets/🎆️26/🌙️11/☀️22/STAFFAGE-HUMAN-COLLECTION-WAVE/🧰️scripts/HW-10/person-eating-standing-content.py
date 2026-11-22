"""📝️ Identity, thermal scenario, clips and per-scale decisions of person-eating-standing (imported by person-eating-standing.py)."""
from hw10lib import *

G = 9.80665


def identity(a):
    a["title"] = text("Person Eating Standing", "Stehend essende Person")
    a["summary"] = text(
        "A man of 1.78 m on his lunch break, eating a baguette sandwich standing, a bottle of water in the other hand: hooded cotton sweatshirt, chinos and leather sneakers, long hair tied back. Three poses (taking a bite, chewing with the sandwich lowered, drinking from the bottle) and two loops (bite and chew, idle breathing) at about 1.3 met; sandwich (250 g) and bottle (520 g) are passive loads in the hands.",
        "Ein Mann von 1,78 m in der Mittagspause, der stehend ein Baguette-Sandwich isst, eine Wasserflasche in der anderen Hand: Kapuzensweatshirt aus Baumwolle, Chinohose und Ledersneaker, lange Haare zurückgebunden. Drei Posen (Abbeißen, Kauen mit gesenktem Sandwich, Trinken aus der Flasche) und zwei Schleifen (Abbeißen und Kauen, ruhiges Atmen) bei etwa 1,3 met; Sandwich (250 g) und Flasche (520 g) sind passive Lasten in den Händen.")
    a["tags"] = ["human", "adult", "man", "standing", "eating", "drinking", "sandwich", "water-bottle", "lunch-break", "street-life"]
    a["presentation"] = {"defaultPose": "taking-a-bite", "defaultClip": "bite-and-chew", "defaultScale": 100}
    prov = a["provenance"]
    prov["author"] = "Semio staffage human collection worker HW-10"
    prov["created"] = "2026-11-22"
    refs = [r for r in prov["references"] if r["id"] != "lee-2015"]
    refs += [{"id": "ainsworth-2011", "citation": "Ainsworth, B. E. et al. (2011). 2011 Compendium of Physical Activities: a second update of codes and MET values. Medicine and Science in Sports and Exercise 43(8), 1575-1581; code 13030 eating, sitting, 1.5 MET."},
             {"id": "iso-8996-2021", "citation": "ISO 8996:2021 Ergonomics of the thermal environment, determination of metabolic rate; Table A.1 metabolic rate of standing at ease 70 W/m2."},
             {"id": "po-2011", "citation": "Po, J. M. C. et al. (2011). Time-frequency analysis of chewing activity in the natural environment. Journal of Dental Research 90(10), 1206-1210; natural chewing frequency of about 1.5 Hz."}]
    prov["references"] = refs
    prov["notes"] = text(
        "Derived from the archive's standing smartphone user: rig, outlines, 3D bodies and clothing scaled as one by 1.78/1.69 to a man of 1.78 m barefoot (1.80 m shod), segment masses re-distributed after the male table of de Leva (1996) for 75 kg, centres of mass of the long limb segments at the male fractions; the headphones and the phone are removed, the hooded sweatshirt, chinos and sneakers are re-specified as cotton, cotton twill and leather. The baguette sandwich (210 mm long, 66 mm round, filling 12 mm) in the right hand and the 0.5 L water bottle (205 mm, 65 mm round) in the left hand are new rigid bones; their grips and the three arm poses were fitted with a Nelder-Mead search on the evaluated rig (scripts in the ticket folder, cached as a design input) and checked in all three views. Breadths, depths and girths stay those of the slim template, so the build is a slim man (BMI 23.7).",
        "Abgeleitet von der stehenden Smartphone-Nutzerin des Archivs: Skelett, Umrisse, 3D-Körper und Kleidung gemeinsam um 1,78/1,69 auf einen Mann von 1,78 m barfuß (1,80 m mit Schuhen) skaliert, die Segmentmassen nach der Männertabelle von de Leva (1996) für 75 kg neu verteilt, die Schwerpunkte der langen Gliedmaßensegmente nach den männlichen Anteilen; Kopfhörer und Telefon sind entfernt, Kapuzensweatshirt, Chinohose und Sneaker als Baumwolle, Baumwollköper und Leder neu beschrieben. Das Baguette-Sandwich (210 mm lang, 66 mm rund, Belag 12 mm) in der rechten Hand und die 0,5-l-Wasserflasche (205 mm, 65 mm rund) in der linken Hand sind neue starre Knochen; ihre Griffe und die drei Armposen wurden mit einer Nelder-Mead-Suche am ausgewerteten Skelett eingepasst (Skripte im Ticketordner, als Entwurfsgrundlage zwischengespeichert) und in allen drei Ansichten geprüft. Breiten, Tiefen und Umfänge bleiben die der schlanken Vorlage, die Statur ist ein schlanker Mann (BMI 23,7).")
    a["structure"]["assumptions"] = [
        text("Segments are point masses at the de Leva centres (male table, 75 kg); the 250 g sandwich and the 520 g bottle are carried loads at their own centres in the hands.", "Segmente sind Punktmassen in den Schwerpunkten nach de Leva (Männertabelle, 75 kg); das 250 g schwere Sandwich und die 520 g schwere Flasche sind getragene Lasten in ihren eigenen Schwerpunkten in den Händen."),
        text("Contacts are heel and ball of each foot; both feet stand flat at hip-joint width in every named pose and clip instant.", "Kontakte sind Ferse und Ballen jedes Fußes; beide Füße stehen in jeder benannten Pose und jedem Clip-Zeitpunkt flach in Hüftgelenkbreite.")]
    for b in a["rig"]["bones"]:
        if b["id"] == "torso":
            b["title"] = text("Torso with sweatshirt", "Rumpf mit Sweatshirt")
        if b["id"] == "ponytail":
            b["title"] = text("Tied-back hair from the hair tie", "Zurückgebundenes Haar ab dem Haargummi")
    for an in a["anchors"]:
        if an["id"] == "grip-r":
            an["title"] = text("Right thumb on the sandwich", "Rechter Daumen am Sandwich")


def materials(a):
    for m in a["materials"]:
        if m["id"] == "cotton-sweatshirt":
            m["title"] = text("Cotton sweatshirt jersey", "Baumwoll-Sweatshirtstoff")
        if m["id"] == "cotton-twill":
            m["title"] = text("Cotton twill (chinos)", "Baumwollköper (Chinohose)")
            m["solarAbsorptivity"] = 0.55
        if m["id"] == "leather-sneaker":
            m["title"] = text("Leather sneaker with rubber sole", "Ledersneaker mit Gummisohle")
            m["emissivity"] = 0.95
            m["solarAbsorptivity"] = 0.65
    a["materials"] += [
        {"id": "bread-roll", "title": text("Baguette crust and crumb", "Baguettekruste und -krume"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 250, "specificHeatJKgK": 2500, "source": "authored-estimate", "reference": "incropera-2018"},
        {"id": "sandwich-filling", "title": text("Ham, cheese and lettuce", "Schinken, Käse und Salat"), "emissivity": 0.95, "solarAbsorptivity": 0.7, "densityKgM3": 1000, "specificHeatJKgK": 3300, "source": "authored-estimate", "reference": "incropera-2018"},
        {"id": "pet-bottle", "title": text("PET bottle filled with water", "PET-Flasche mit Wasser"), "emissivity": 0.94, "solarAbsorptivity": 0.15, "source": "authored-estimate", "reference": "incropera-2018"}]
    refs = a["provenance"]["references"]
    if not any(r["id"] == "incropera-2018" for r in refs):
        refs.append({"id": "incropera-2018", "citation": "Bergman, T. L., Lavine, A. S., Incropera, F. P. and DeWitt, D. P. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley; Table A.3 thermophysical properties of common materials."})


def thermal(a):
    t = a["thermal"]
    t["environment"] = {"airTempC": 17, "meanRadiantTempC": 18, "skyTempC": 6, "groundTempC": 19, "relativeHumidity": 0.6, "windMS": 0.6, "windDirectionDeg": 180, "solarWM2": 0, "diffuseFraction": 1, "pressurePa": 101325}
    t["baseMet"] = 1.3
    body = t["body"]
    skin = 0.007184 * MASS_ ** 0.425 * (178) ** 0.725
    body.update({"massKg": MASS_, "skinAreaM2": round(skin, 3), "basalMetabolismW": round((10 * MASS_ + 6.25 * 178 - 5 * 30 + 5) * 4184 / 86400, 1), "source": "authored-estimate", "reference": "mifflin-1990"})
    resp = t["respiration"]
    resp["patterns"] = [
        {"id": "quiet-standing", "title": text("Quiet standing", "Ruhiges Stehen"), "rateBpm": 13, "tidalVolumeL": 0.5, "inhaleFraction": 0.38, "pauseAfterInhaleFraction": 0.04, "exhaleFraction": 0.45, "pauseAfterExhaleFraction": 0.13, "route": "nose", "source": "authored-estimate", "reference": "icrp-66-1994"},
        {"id": "chewing", "title": text("Breathing through the nose while chewing", "Nasenatmung beim Kauen"), "rateBpm": 15, "tidalVolumeL": 0.45, "inhaleFraction": 0.36, "pauseAfterInhaleFraction": 0.03, "exhaleFraction": 0.47, "pauseAfterExhaleFraction": 0.14, "route": "nose", "source": "authored-estimate", "reference": "icrp-66-1994"}]
    resp["motion"] = [{"bone": "torso", "angleDeg": 0.5, "dy": 0.0032}]
    regs = {r["id"]: r for r in t["regions"]}
    regs["hoodie"]["title"] = text("T-shirt and cotton sweatshirt on trunk and arms", "T-Shirt und Baumwoll-Sweatshirt an Rumpf und Armen")
    regs["hoodie"]["layers"][1].update({"id": "sweatshirt", "title": text("Cotton sweatshirt, hood down", "Baumwoll-Sweatshirt, Kapuze unten"), "material": "cotton-sweatshirt", "thicknessM": 0.003, "resistanceM2KW": 0.0527, "vapourResistanceM2PaW": 8.4})
    regs["chinos"]["title"] = text("Chinos over the legs", "Chinohose über den Beinen")
    rename_layer(a, "chinos", "jeans", "chinos")
    regs["chinos"]["layers"][1].update({"id": "chinos", "title": text("Cotton twill chinos", "Chinohose aus Baumwollköper"), "material": "cotton-twill", "thicknessM": 0.0009, "resistanceM2KW": 0.0388, "vapourResistanceM2PaW": 6.2})
    regs["sneakers"]["title"] = text("Socks and leather sneakers", "Socken und Ledersneaker")
    regs["sneakers"]["layers"][1].update({"id": "sneaker", "title": text("Leather sneaker", "Ledersneaker"), "material": "leather-sneaker", "resistanceM2KW": 0.0078, "vapourResistanceM2PaW": 3.2})
    regs["hair"]["layers"][0]["title"] = text("Long hair tied back", "Lange Haare, zurückgebunden")
    t["regions"].append({"id": "sandwich", "title": text("Baguette sandwich", "Baguette-Sandwich"), "kind": "surface", "model": "passive", "material": "bread-roll", "areaM2": 0.046, "characteristicLengthM": 0.066, "geometry": "cylinder", "skyViewFactor": 0.4, "layers": [],
                         "heatCapacityJM2K": 11000, "ratedPowerW": 0, "ratedPowerBasis": "passive", "source": "authored-estimate", "reference": "incropera-2018"})
    t["regions"].append({"id": "water-bottle", "title": text("Water bottle, 0.5 L", "Wasserflasche, 0,5 l"), "kind": "surface", "model": "passive", "material": "pet-bottle", "areaM2": 0.044, "characteristicLengthM": 0.065, "geometry": "cylinder", "skyViewFactor": 0.4, "layers": [],
                         "heatCapacityJM2K": 47000, "ratedPowerW": 0, "ratedPowerBasis": "passive", "source": "authored-estimate", "reference": "incropera-2018"})
    t["assumptions"] = [
        text(f"Skin area {body['skinAreaM2']} m2 after DuBois and DuBois (1916) for {MASS_:.0f} kg and 1.78 m; regions carry skin fractions 0.07 face and neck, 0.03 scalp, 0.05 hands, 0.49 trunk and arms, 0.30 legs, 0.06 feet (rule-of-nines proportions, authored).",
             f"Hautfläche {str(body['skinAreaM2']).replace('.', ',')} m2 nach DuBois und DuBois (1916) für {MASS_:.0f} kg und 1,78 m; die Regionen tragen die Hautanteile 0,07 Gesicht und Hals, 0,03 Kopfhaut, 0,05 Hände, 0,49 Rumpf und Arme, 0,30 Beine, 0,06 Füße (Neunerregel-Proportionen, geschätzt)."),
        text("Basal metabolism from the Mifflin-St Jeor equation for a 30-year-old man of 75 kg and 178 cm; clothing in clo after the ISO 9920 tables (1 clo = 0.155 m2K/W): T-shirt and sweatshirt 0.44 clo, briefs and chinos 0.28 clo, socks and sneakers 0.07 clo, digits authored; vapour resistance R/(0.0165 K/Pa x 0.38).",
             "Grundumsatz aus der Mifflin-St-Jeor-Gleichung für einen 30-jährigen Mann mit 75 kg und 178 cm; Kleidung in clo nach den Tabellen von ISO 9920 (1 clo = 0,155 m2K/W): T-Shirt und Sweatshirt 0,44 clo, Unterhose und Chinohose 0,28 clo, Socken und Sneaker 0,07 clo, Werte geschätzt; Dampfwiderstand R/(0,0165 K/Pa x 0,38)."),
        text("Standing at ease is 1.2 met (ISO 8996 Table A.1, 70 W/m2); eating while standing is taken as 1.3 met, between standing at ease and the 1.5 MET that the Ainsworth compendium gives for eating while sitting (code 13030), because the arm is lifted to the mouth and the jaw works; the values are authored, not measured.",
             "Entspanntes Stehen entspricht 1,2 met (ISO 8996 Tabelle A.1, 70 W/m2); Essen im Stehen wird mit 1,3 met angesetzt, zwischen entspanntem Stehen und den 1,5 MET, die das Ainsworth-Kompendium für Essen im Sitzen nennt (Code 13030), weil der Arm zum Mund gehoben wird und der Kiefer arbeitet; die Werte sind geschätzt, nicht gemessen."),
        text("The sandwich is a passive body at ambient temperature: bread of 250 kg/m3 and 2.5 kJ/(kg K) over a mean depth of 17 mm gives about 11 kJ/(m2 K); it has no power of its own and its contact with the fingers is not modelled. The water bottle is passive as well: 0.5 kg of water behind a 0.3 mm PET wall over 0.044 m2 is about 47 kJ/(m2 K), so it follows the air only slowly. While chewing, breathing runs through the nose (mouth closed).",
             "Das Sandwich ist ein passiver Körper auf Umgebungstemperatur: Brot mit 250 kg/m3 und 2,5 kJ/(kg K) über eine mittlere Tiefe von 17 mm ergibt etwa 11 kJ/(m2 K); es hat keine eigene Leistung, und sein Kontakt mit den Fingern wird nicht modelliert. Auch die Wasserflasche ist passiv: 0,5 kg Wasser hinter einer 0,3 mm dicken PET-Wand auf 0,044 m2 ergeben etwa 47 kJ/(m2 K), sie folgt der Luft also nur langsam. Beim Kauen läuft die Atmung durch die Nase (Mund geschlossen).")]


MASS_ = 75.0
POSE_TEXT = {
    "taking-a-bite": (text("Taking a bite", "Abbeißen"),
                      text("Feet at hip width, weight on both legs; the right elbow is raised forward and out, the forearm folded so that the right hand brings the end of the sandwich to the open mouth, the roll pointing from the front right towards the face; the head leans 10 degrees into the bite, the left hand carries the bottle upright beside the thigh.",
                           "Füße hüftbreit, Gewicht auf beiden Beinen; der rechte Ellbogen ist nach vorn und außen gehoben, der Unterarm so gebeugt, dass die rechte Hand das Ende des Sandwiches an den geöffneten Mund führt, das Brötchen zeigt von vorn rechts zum Gesicht; der Kopf neigt sich 10 Grad in den Biss, die linke Hand trägt die Flasche aufrecht neben dem Oberschenkel.")),
    "chewing": (text("Chewing, sandwich lowered", "Kauen, Sandwich gesenkt"),
                text("Feet at hip width, head level, looking ahead while chewing; the right elbow is bent about 90 degrees and holds the sandwich in front of the waist pointing forward and to the left, the left hand carries the bottle upright beside the thigh.",
                     "Füße hüftbreit, Kopf gerade, beim Kauen geradeaus blickend; der rechte Ellbogen ist etwa 90 Grad gebeugt und hält das Sandwich vor der Taille nach vorn links gerichtet, die linke Hand trägt die Flasche aufrecht neben dem Oberschenkel.")),
    "drinking-from-bottle": (text("Drinking from the bottle", "Aus der Flasche trinken"),
                             text("Feet at hip width, the head tipped back about 15 degrees; the left elbow is raised forward and the hand tilts the bottle with its mouth at the lips and its bottom up, the right hand keeps the sandwich lowered in front of the waist.",
                                  "Füße hüftbreit, der Kopf etwa 15 Grad zurückgeneigt; der linke Ellbogen ist nach vorn gehoben und die Hand kippt die Flasche mit der Öffnung an den Lippen und dem Boden nach oben, die rechte Hand hält das Sandwich gesenkt vor der Taille.")),
}
POSE_THERMAL = {"taking-a-bite": (1.3, "chewing", 0.75, 0.9), "chewing": (1.3, "chewing", 0.85, 0.95), "drinking-from-bottle": (1.3, "quiet-standing", 0.85, 0.9)}


def bind_poses(a):
    for p in a["poses"]:
        title, desc = POSE_TEXT[p["id"]]
        met, breath, sandwich_exposed, hoodie = POSE_THERMAL[p["id"]]
        p.pop("key", None)
        p.update({"title": title, "description": desc, "ground": "auto", "contacts": ["heel-l", "ball-l", "heel-r", "ball-r"]})
        p["thermal"] = {"activityMet": met, "breathing": breath,
                        "regions": region_binding(a, {"hoodie": hoodie, "sandwich": sandwich_exposed, "water-bottle": 0.8, "hand-skin": 0.75},
                                                  {"sneakers": {"contact": {"fraction": 0.3, "tempC": 19}}}),
                        "emitters": {"body-plume": 0.6}, "operating": {}}
        p["loads"] = [{"id": "sandwich-weight", "kind": "carried", "title": text("Sandwich weight", "Gewicht des Sandwiches"), "anchor": "sandwich-cog", "fxN": 0, "fyN": round(-0.25 * G, 3), "source": "authored-estimate"},
                      {"id": "bottle-weight", "kind": "carried", "title": text("Weight of the filled bottle", "Gewicht der gefüllten Flasche"), "anchor": "bottle-cog", "fxN": 0, "fyN": round(-0.52 * G, 3), "source": "authored-estimate"}]


def clips(a, chew_head):
    feet = lambda d: [{"anchor": x, "fromS": 0, "toS": d} for x in ("heel-l", "ball-l", "heel-r", "ball-r")]
    chew = []
    t = 5.6
    i = 0
    while t < 9.5:
        chew.append({"t": round(t, 3), "pose": "chewing", "ease": "smooth", "bones": {"head": {"angleDeg": chew_head + (1.6 if i % 2 == 0 else -0.4)}, "neck": {"angleDeg": -1 + (0.5 if i % 2 == 0 else 0)}}})
        t += 0.333
        i += 1
    bite = {
        "id": "bite-and-chew", "title": text("Bite and chew", "Abbeißen und Kauen"),
        "description": text("A ten-second loop: chewing with the sandwich lowered, the right arm lifts the sandwich to the mouth, the head leans into the bite, the arm lowers it again and the head nods slightly with the jaw at about 1.5 chews per second; the feet stay planted.",
                            "Eine Schleife von zehn Sekunden: Kauen mit gesenktem Sandwich, der rechte Arm hebt das Sandwich zum Mund, der Kopf neigt sich in den Biss, der Arm senkt es wieder und der Kopf nickt leicht mit dem Kiefer bei etwa 1,5 Kaubewegungen pro Sekunde; die Füße bleiben stehen."),
        "durationS": 10, "fps": 30, "loop": True, "rootMotion": "none",
        "keys": [{"t": 0, "pose": "chewing", "ease": "smooth"}, {"t": 1.8, "pose": "chewing", "ease": "smooth"}, {"t": 3.2, "pose": "taking-a-bite", "ease": "smooth"}, {"t": 3.9, "pose": "taking-a-bite", "ease": "smooth", "bones": {"neck": {"angleDeg": -9}, "head": {"angleDeg": -5}}},
                 {"t": 4.3, "pose": "taking-a-bite", "ease": "smooth", "bones": {"neck": {"angleDeg": -4}, "head": {"angleDeg": 1}}}, {"t": 5.2, "pose": "chewing", "ease": "smooth"}] + chew + [{"t": 10, "pose": "chewing", "ease": "smooth"}],
        "contacts": feet(10),
        "phases": [{"id": "hold", "fromS": 0, "toS": 1.8, "title": text("Holding and swallowing", "Halten und Schlucken")}, {"id": "raise", "fromS": 1.8, "toS": 3.9, "title": text("Raising the sandwich to the mouth", "Sandwich zum Mund führen")},
                   {"id": "bite", "fromS": 3.9, "toS": 4.3, "title": text("Biting off", "Abbeißen")}, {"id": "chew", "fromS": 4.3, "toS": 10, "title": text("Lowering and chewing", "Senken und Kauen")}],
        "events": [{"t": 3.9, "kind": "motion", "title": text("Teeth close on the bread", "Die Zähne schließen sich um das Brot"), "target": "sandwich"}],
        "tracks": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 1}, {"t": 10, "v": 1}]}, {"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 1.3}, {"t": 3.2, "v": 1.4}, {"t": 5.2, "v": 1.3}, {"t": 10, "v": 1.3}]}]}
    idle = {
        "id": "idle-breathing", "title": text("Idle breathing", "Ruhiges Atmen"),
        "description": text("A five-second loop between two bites, sandwich lowered and bottle at the side: the chest rises and falls with quiet breathing through the nose, the head settles in a small slow sway; the feet stay planted.",
                            "Eine Schleife von fünf Sekunden zwischen zwei Bissen, Sandwich gesenkt und Flasche an der Seite: Der Brustkorb hebt und senkt sich mit ruhiger Nasenatmung, der Kopf wiegt sich leicht und langsam; die Füße bleiben stehen."),
        "durationS": 5, "fps": 30, "loop": True, "rootMotion": "none",
        "keys": [{"t": 0, "pose": "chewing", "ease": "smooth"}, {"t": 1.25, "pose": "chewing", "ease": "smooth", "bones": {"head": {"angleDeg": chew_head + 1.5}, "neck": {"angleDeg": -0.5}}},
                 {"t": 2.5, "pose": "chewing", "ease": "smooth", "bones": {"head": {"angleDeg": chew_head - 0.5}, "neck": {"angleDeg": -1.5}}}, {"t": 3.75, "pose": "chewing", "ease": "smooth", "bones": {"head": {"angleDeg": chew_head + 1}, "neck": {"angleDeg": -1}}}, {"t": 5, "pose": "chewing", "ease": "smooth"}],
        "contacts": feet(5), "phases": [{"id": "rest", "fromS": 0, "toS": 5, "title": text("Breathing at rest", "Atmen in Ruhe")}], "events": [],
        "tracks": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 5, "v": 0}]}, {"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 1.2}, {"t": 5, "v": 1.2}]}]}
    a["clips"] = [bite, idle]


GIRDLE = text("Shoulder and pelvic girdle are bones of the rig: each clavicle runs from the sternoclavicular joint beside the manubrium out, back and up to the acromioclavicular joint and carries clavicle, scapula, scapular spine, acromion and glenoid; each hip bone runs from the sacroiliac joint down, forward and out to the hip joint and carries ilium, pubic and ischial ramus. Arms and thighs hang from them at the true breadths of this man (shoulder joints 0.322 m, hip joints 0.179 m apart), so a shrug or a hip hike moves the limb with its girdle.",
              "Schulter- und Beckengürtel sind Knochen des Skeletts: Jedes Schlüsselbein läuft vom Sternoklavikulargelenk neben dem Manubrium nach außen, hinten und oben zum Akromioklavikulargelenk und trägt Schlüsselbein, Schulterblatt, Schulterblattgräte, Akromion und Gelenkpfanne; jedes Hüftbein läuft vom Iliosakralgelenk nach unten, vorn und außen zum Hüftgelenk und trägt Darmbein, Scham- und Sitzbeinast. Arme und Oberschenkel hängen in den wahren Breiten dieses Mannes daran (Schultergelenke 0,322 m, Hüftgelenke 0,179 m auseinander), sodass ein Schulterheben oder Hüftheben die Gliedmaße mit ihrem Gürtel bewegt.")
SLEEVE = text("The upper sleeves follow an arm of 0.27 m girth under the sweatshirt: 0.114 m wide at the deltoid, 0.097 m at mid arm and 0.092 m at the elbow, the same taper at every scale and in the revolved sleeve profile of the drawings.",
              "Die Oberärmel folgen einem Arm von 0,27 m Umfang unter dem Sweatshirt: 0,114 m breit am Deltamuskel, 0,097 m in Armmitte und 0,092 m am Ellbogen, dieselbe Verjüngung in jedem Maßstab und im gedrehten Ärmelprofil der Zeichnungen.")

DECISIONS = {
    "50": {
        "silhouette": text("The three views are the orthographic views of the 1:50 body of a man of 1.78 m taking a bite, 36 mm tall: head with jaw, nose and chin under the cap of tied-back hair, the hair tail on the nape, ear and eye, neck, hooded sweatshirt with hem rib, pocket, collar and the hood on the upper back with its rolled edge, sleeves, forearms and rib cuffs, both hands as palm, fingers and thumb, the right hand round the baguette sandwich whose end touches the lips, chinos seat, thighs with kneecaps, shanks, socks, sneakers with sole, toe cap and toe sole. The roll is drawn with its rounded ends and the filling of ham, cheese and lettuce as a wavy band that stands 4 mm proud of the crust. Front view (the figure's left is on the right of the sheet): the raised right elbow, the roll foreshortened towards the face. Plan: the roll crossing in front of the face from the right hand to the mouth, the filling showing at its sides. Sleeves, forearms and the roll are drawn as solids of revolution about their bone, hands, filling and the hair tail as plates, so every part follows the three-dimensional arm poses.",
                           "Die drei Ansichten sind die orthogonalen Ansichten des 1:50-Körpers eines Mannes von 1,78 m beim Abbeißen, 36 mm hoch: Kopf mit Kiefer, Nase und Kinn unter der Kappe zurückgebundener Haare, der Haarschwanz im Nacken, Ohr und Auge, Hals, Kapuzensweatshirt mit Saumbund, Tasche, Kragen und der Kapuze auf dem oberen Rücken mit ihrem Rollrand, Ärmel, Unterarme und Bündchen, beide Hände als Handfläche, Finger und Daumen, die rechte Hand um das Baguette-Sandwich, dessen Ende die Lippen berührt, Gesäß der Chinohose, Oberschenkel mit Kniescheiben, Unterschenkel, Socken, Sneaker mit Sohle, Zehenkappe und Zehensohle. Das Brötchen ist mit seinen gerundeten Enden gezeichnet, der Belag aus Schinken, Käse und Salat als welliges Band, das 4 mm über die Kruste vorsteht. Vorderansicht (die linke Körperseite liegt rechts im Blatt): der gehobene rechte Ellbogen, das zum Gesicht hin verkürzte Brötchen. Grundriss: das Brötchen quer vor dem Gesicht von der rechten Hand zum Mund, der Belag an seinen Seiten sichtbar. Ärmel, Unterarme und das Brötchen sind als Drehkörper um ihren Knochen gezeichnet, Hände, Belag und Haarschwanz als Platten, sodass jedes Teil den räumlichen Armposen folgt."),
        "retained": text("Everything of the 1:100 drawing plus eye, brow, mouth, the hair tie, the fingers as their own shape, socks, toe caps, laces and the wavy filling of the sandwich as its own shape.", "Alles aus der 1:100-Zeichnung und zusätzlich Auge, Braue, Mund, Haargummi, die Finger als eigene Form, Socken, Zehenkappen, Schnürung und der wellige Belag des Sandwiches als eigene Form."),
        "merged": text("The four fingers of a hand are one shape; eyelets merge into the lace strokes; hair strands are three strokes; lips, chin and brows merge into the head outline; ham, cheese and lettuce are one band.", "Die vier Finger einer Hand sind eine Form; Ösen gehen in den Schnürstrichen auf; Haarsträhnen sind drei Striche; Lippen, Kinn und Brauen gehen im Kopfumriss auf; Schinken, Käse und Salat sind ein Band."),
        "omitted": text("Pores, individual hairs, stitching holes, fabric fibres, crumbs, the scoring of the crust and the bite mark.", "Poren, einzelne Haare, Stichlöcher, Stofffasern, Krümel, die Einschnitte der Kruste und die Bissspur."),
        "thinElements": text("Laces and seams are 0.10 mm texture lines; the 12 mm filling is drawn at its true thickness (0.24 mm on paper) and never thickened.", "Schnürung und Nähte sind 0,10-mm-Texturlinien; der 12 mm dicke Belag ist in seiner wahren Dicke gezeichnet (0,24 mm auf dem Papier) und nie verdickt."),
        "hatching": text("Sweatshirt and sleeves: vertical knit lines at 0.6 mm; chinos: twill lines at 62 degrees, 0.5 mm.", "Sweatshirt und Ärmel: senkrechte Strickstriche mit 0,6 mm; Chinohose: Köperlinien unter 62 Grad mit 0,5 mm."),
        "styles": text("Body 0.25 mm, detail 0.13 mm, texture 0.10 mm; skin and bread light, sweatshirt mid-grey, chinos light-mid, filling dark, sneaker soles darker.", "Körper 0,25 mm, Detail 0,13 mm, Textur 0,10 mm; Haut und Brot hell, Sweatshirt mittelgrau, Chinohose hell-mittel, Belag dunkel, Sneakersohlen dunkler."),
        "overlays": text("One region per shape; details inherit the region of the garment they sit on, roll and filling carry the sandwich value. Front and plan: every filled shape carries the region of the garment, body part or food it depicts, so thermal, structural and transport overlays read in all three views.", "Eine Region je Form; Details übernehmen die Region des Kleidungsstücks, auf dem sie sitzen, Brötchen und Belag tragen den Wert des Sandwiches. Vorderansicht und Grundriss: Jede gefüllte Form trägt die Region des Kleidungsstücks, Körperteils oder Essens, das sie zeigt, sodass Thermik-, Tragwerks- und Transportüberlagerungen in allen drei Ansichten lesbar sind."),
    },
    "100": {
        "silhouette": text("The three views are the orthographic views of the 1:100 body taking a bite, 18 mm tall: head with nose under the hair cap, hair tail, ear, neck, sweatshirt with the hood on the upper back, sleeves, forearms, each hand as palm, finger slab and thumb, the sandwich as one roll with a seam line for the filling, chinos seat, thighs, shanks, one shoe form and a toe box per foot. Inner lines: hem, pocket, collar, side seam, sole and the filling seam. Front view (the figure's left is on the right of the sheet): hair down to the hairline, ears, the raised right elbow and the roll at the mouth, hem, collar and pocket lines. Plan: shoes, legs, the roll in front of the face, hands, head, hair and hair tail in the order of their height; no inner lines.",
                           "Die drei Ansichten sind die orthogonalen Ansichten des 1:100-Körpers beim Abbeißen, 18 mm hoch: Kopf mit Nase unter der Haarkappe, Haarschwanz, Ohr, Hals, Sweatshirt mit der Kapuze auf dem oberen Rücken, Ärmel, Unterarme, jede Hand als Handfläche, Fingerplatte und Daumen, das Sandwich als ein Brötchen mit einer Nahtlinie für den Belag, Gesäß der Chinohose, Oberschenkel, Unterschenkel, je Fuß eine Schuhform und eine Zehenkappe. Innenlinien: Saum, Tasche, Kragen, Seitennaht, Sohle und die Belagnaht. Vorderansicht (die linke Körperseite liegt rechts im Blatt): Haar bis zum Haaransatz, Ohren, der gehobene rechte Ellbogen und das Brötchen am Mund, Saum-, Kragen- und Taschenlinien. Grundriss: Schuhe, Beine, das Brötchen vor dem Gesicht, Hände, Kopf, Haar und Haarschwanz in der Reihenfolge ihrer Höhe; keine Innenlinien."),
        "retained": text("Hair cap with hair tail and ear, face profile with nose and chin, hands with finger slab and thumb, the roll at its true size with the filling as one seam line.", "Haarkappe mit Haarschwanz und Ohr, Gesichtsprofil mit Nase und Kinn, Hände mit Fingerplatte und Daumen, das Brötchen in wahrer Größe mit dem Belag als eine Nahtlinie."),
        "merged": text("Eye, brow and mouth are not drawn; laces and eyelets merge into the sneaker; the four fingers form one slab; cuffs merge into the forearms; the filling merges into the roll, marked by its seam.", "Auge, Braue und Mund werden nicht gezeichnet; Schnürung und Ösen gehen im Sneaker auf; die vier Finger bilden eine Platte; Bündchen gehen in den Unterarmen auf; der Belag geht im Brötchen auf, markiert durch seine Naht."),
        "omitted": text("Eye (0.16 mm), laces, knee folds, drawstrings, stitching, hair tie, the 4 mm overhang of the filling.", "Auge (0,16 mm), Schnürung, Kniefalten, Kordeln, Nähte, Haargummi, der 4 mm Überstand des Belags."),
        "thinElements": text("The filling seam is a 0.10 mm detail line inside the roll; nothing is thickened.", "Die Belagnaht ist eine 0,10-mm-Detaillinie im Brötchen; nichts wird verdickt."),
        "hatching": text("Chinos only: fine parallel twill lines at 62 degrees, 0.7 mm spacing, 0.10 mm pen; the sweatshirt stays plain tone.", "Nur Chinohose: feine parallele Köperlinien unter 62 Grad, 0,7 mm Abstand, 0,10-mm-Stift; das Sweatshirt bleibt reiner Ton."),
        "styles": text("Body pen 0.20 mm, detail 0.10 mm; sweatshirt mid-grey, chinos light-mid with twill, sneakers light, skin and bread light.", "Körperstift 0,20 mm, Detail 0,10 mm; Sweatshirt mittelgrau, Chinohose hell-mittel mit Köper, Sneaker hell, Haut und Brot hell."),
        "overlays": text("Regions as at 1:200; hood and sweatshirt share the sweatshirt value, the roll carries the sandwich value. Front and plan: regions as in the side view; details inherit the region of the garment or food they sit on.", "Regionen wie bei 1:200; Kapuze und Sweatshirt teilen den Sweatshirt-Wert, das Brötchen trägt den Wert des Sandwiches. Vorderansicht und Grundriss: Regionen wie in der Seitenansicht; Details übernehmen die Region des Kleidungsstücks oder Essens, auf dem sie sitzen."),
    },
    "200": {
        "silhouette": text("The three views are the orthographic views of the 1:200 body taking a bite, 9 mm tall, one mass per part: head, hair cap, hair tail, neck, sweatshirt, hood, chinos seat, sleeves, forearms, one mass per hand, the sandwich as one roll at the mouth, thighs, shanks, shoes and toe boxes; the pocket is the only inner line. Front view (the figure's left is on the right of the sheet): the same masses, the raised right elbow; the legs keep a gap of more than 30 mm real size. Plan: the masses in the order of their height.",
                           "Die drei Ansichten sind die orthogonalen Ansichten des 1:200-Körpers beim Abbeißen, 9 mm hoch, eine Masse je Teil: Kopf, Haarkappe, Haarschwanz, Hals, Sweatshirt, Kapuze, Gesäß der Chinohose, Ärmel, Unterarme, eine Masse je Hand, das Sandwich als ein Brötchen am Mund, Oberschenkel, Unterschenkel, Schuhe und Zehenkappen; die Tasche ist die einzige Innenlinie. Vorderansicht (die linke Körperseite liegt rechts im Blatt): dieselben Massen, der gehobene rechte Ellbogen; die Beine halten eine Lücke von mehr als 30 mm wahrer Größe. Grundriss: die Massen in der Reihenfolge ihrer Höhe."),
        "retained": text("Elbows and knees as separate masses so that the pose changes read, the hood on the upper back, the hair tail, the roll at the mouth (1.05 by 0.33 mm), the kangaroo pocket as the only inner line.", "Ellbogen und Knie als getrennte Massen, damit die Posenwechsel lesbar bleiben, die Kapuze auf dem oberen Rücken, der Haarschwanz, das Brötchen am Mund (1,05 mal 0,33 mm), die Känguru-Tasche als einzige Innenlinie."),
        "merged": text("Thumb and fingers into the hand, hem rib into the sweatshirt, sock into the sneaker, hair tie into the hair tail, filling into the roll.", "Daumen und Finger in die Hand, Saumbund in das Sweatshirt, Socke in den Sneaker, Haargummi in den Haarschwanz, Belag in das Brötchen."),
        "omitted": text("Ear, nose, eye, cuffs, laces, seams, knee folds, the filling seam.", "Ohr, Nase, Auge, Bündchen, Schnürung, Nähte, Kniefalten, die Belagnaht."),
        "thinElements": text("None below the print minimum; the roll is drawn at its true 66 mm (0.33 mm on paper).", "Keine unter dem Druckminimum; das Brötchen ist mit seinen wahren 66 mm gezeichnet (0,33 mm auf dem Papier)."),
        "hatching": text("None; fill tones carry the materials.", "Keine; die Füllungstöne tragen die Materialien."),
        "styles": text("Body pen 0.18 mm, detail pen 0.10 mm for the pocket; light tones for skin, bread and shoes, mid for the sweatshirt and chinos, dark for hair.", "Körperstift 0,18 mm, Detailstift 0,10 mm für die Tasche; helle Töne für Haut, Brot und Schuhe, mittlere für Sweatshirt und Chinohose, dunkle für das Haar."),
        "overlays": text("One region per mass; both sleeves share the sweatshirt value, hair tail and hair cap share the hair value, the roll carries the sandwich value. Front and plan: regions as in the side view.", "Eine Region je Masse; beide Ärmel teilen den Sweatshirt-Wert, Haarschwanz und Haarkappe teilen den Haar-Wert, das Brötchen trägt den Wert des Sandwiches. Vorderansicht und Grundriss: Regionen wie in der Seitenansicht."),
    },
    "500": {
        "silhouette": text("A figure of 1.78 m (3.6 mm) with the right forearm raised to the face, the orthographic views of the 1:500 body: head with hair as one mass, neck, sweatshirt with the hood merged into the upper back, chinos seat, and per side sleeve, forearm, hand, thigh, shank, shoe and toe box. Front view (the figure's left is on the right of the sheet): the same masses, the right hand at the mouth. Plan: the masses in the order of their height.",
                           "Eine Figur von 1,78 m (3,6 mm) mit zum Gesicht gehobenem rechtem Unterarm, die orthogonalen Ansichten des 1:500-Körpers: Kopf mit Haar als eine Masse, Hals, Sweatshirt mit im oberen Rücken aufgegangener Kapuze, Gesäß der Chinohose und je Seite Ärmel, Unterarm, Hand, Oberschenkel, Unterschenkel, Schuh und Zehenkappe. Vorderansicht (die linke Körperseite liegt rechts im Blatt): dieselben Massen, die rechte Hand am Mund. Grundriss: die Massen in der Reihenfolge ihrer Höhe."),
        "retained": text("The raised right forearm with the hand at the mouth, which carries the pose, the hanging left arm and the exact stature and extent.", "Der gehobene rechte Unterarm mit der Hand am Mund, der die Pose trägt, der hängende linke Arm sowie genaue Körpergröße und Ausdehnung."),
        "merged": text("Face and hair; hair tail into the hair; hood into the sweatshirt shape; filling into the roll.", "Gesicht und Haar; Haarschwanz im Haar; Kapuze in der Sweatshirtform; Belag in das Brötchen."),
        "omitted": text("Sleeve cuffs, hood fold, laces, sole line, thumbs and fingers, the hair tail as a shape of its own, cap and label of the bottle.", "Ärmelbündchen, Kapuzenfalte, Schnürung, Sohlenlinie, Daumen und Finger, der Haarschwanz als eigene Form, Deckel und Etikett der Flasche."),
        "thinElements": text("Roll (66 mm) and bottle (65 mm) are drawn at the floor size of 75 mm of this scale, as the 3D model of the scale keeps them, so that they print beside the hands; the hands are drawn 45 mm thick.", "Brötchen (66 mm) und Flasche (65 mm) sind in der Mindestgröße von 75 mm dieses Maßstabs gezeichnet, wie das 3D-Modell des Maßstabs sie behält, damit sie neben den Händen drucken; die Hände sind 45 mm dick gezeichnet."),
        "hatching": text("None.", "Keine."),
        "styles": text("Mid-grey torso, lighter legs, light shoes; line pen 0.13 mm only, no detail pen; light mode dark lines on white, dark mode inverted.", "Mittelgrauer Rumpf, hellere Beine, helle Schuhe; nur Strichstärke 0,13 mm, kein Detailstift; hell: dunkle Linien auf Weiß, dunkel: invertiert."),
        "overlays": text("One region per mass: roll and bottle carry the sandwich and the bottle; head, sweatshirt, legs and shoes keep their own value. Front and plan: shapes list the regions they show.", "Eine Region je Masse: Brötchen und Flasche tragen Sandwich und Flasche; Kopf, Sweatshirt, Beine und Schuhe behalten ihren eigenen Wert. Vorderansicht und Grundriss: Formen nennen die Regionen, die sie zeigen."),
    },
}

BODY_50 = text("Body at 1:50 in the biting grip: every form is a loft through hand-stated cross-sections. Head with jaw and chin, nose, lips, brows, eyes and ears; the tied-back hair as a cap 8 mm over the skull that ends at the hairline, the hair tie and the hair tail on the nape; neck; hooded sweatshirt as one garment with shoulder slope, hem rib, collar rib, kangaroo pocket, drawstrings and the hood on the upper back with its rolled edge; sleeves with the deltoid mass, forearms and rib cuffs; each hand with palm, thenar, a thumb of three segments and four fingers of three segments; the baguette sandwich as a roll of revolution 210 mm long and 68 mm round with rounded ends, through which the filling passes as a slab 12 mm thick with a wavy lettuce edge, 4 mm proud of the crust on both sides; chinos with seat, gluteal masses, thighs, kneecaps and calves; socks; leather sneakers with rubber sole, toe cap and laces.",
               "Körper bei 1:50 im Beißgriff: Jede Form ist ein Loft durch von Hand gesetzte Querschnitte. Kopf mit Kiefer und Kinn, Nase, Lippen, Brauen, Augen und Ohren; das zurückgebundene Haar als Kappe 8 mm über dem Schädel, die am Haaransatz endet, das Haargummi und der Haarschwanz im Nacken; Hals; Kapuzensweatshirt als ein Kleidungsstück mit Schulterschräge, Saumbund, Kragenbund, Kängurutasche, Kordeln und der Kapuze auf dem oberen Rücken mit ihrem Rollrand; Ärmel mit Deltamasse, Unterarme und Bündchen; jede Hand mit Handfläche, Daumenballen, einem Daumen aus drei Gliedern und vier Fingern aus je drei Gliedern; das Baguette-Sandwich als Drehkörper von 210 mm Länge und 68 mm Durchmesser mit gerundeten Enden, durch das der Belag als 12 mm dicke Platte mit welligem Salatrand läuft, beidseitig 4 mm über die Kruste vorstehend; Chinohose mit Gesäß, Gesäßmuskeln, Oberschenkeln, Kniescheiben und Waden; Socken; Ledersneaker mit Gummisohle, Zehenkappe und Schnürung.")
THERMAL_50 = text("Thermal at 1:50: the outer garment surfaces, hair, hands and the sandwich are the body solids themselves. Inside them lie the real inner layers: the T-shirt as a shell 4 mm under the sweatshirt on the trunk and as short sleeves on the upper arms only, the briefs 1.2 mm under the twill on the pelvis only, the socks 4 mm inside the sneaker uppers and visible at the ankles. The face is its own solid, the skull ahead of the hairline, so that the scalp counts as hair and not as bare skin. Roll and filling carry the sandwich region. Breath leaves from a patch under the nose tip and one on the lips, the body plume from a patch on the crown.",
                  "Thermik bei 1:50: Die äußeren Kleidungsflächen, Haar, Hände und das Sandwich sind die Körperformen selbst. Darin liegen die wirklichen inneren Schichten: das T-Shirt als Schale 4 mm unter dem Sweatshirt am Rumpf und als kurze Ärmel nur an den Oberarmen, die Unterhose 1,2 mm unter dem Köper nur am Becken, die Socken 4 mm innerhalb der Sneakerschäfte und an den Knöcheln sichtbar. Das Gesicht ist ein eigener Körper, der Schädel vor dem Haaransatz, sodass die Kopfhaut als Haar und nicht als nackte Haut zählt. Brötchen und Belag tragen die Region des Sandwiches. Der Atem tritt aus einem Fleck unter der Nasenspitze und einem auf den Lippen aus, die Körperfahne aus einem Fleck auf dem Scheitel.")
BODY_100 = text("Body at 1:100, drawn anew with fewer and longer sections: head with nose and ears under the hair cap, without eyes, brows and lips; the hair tail as one form without its tie; neck; the sweatshirt as one form in which hem rib and collar rib are merged, the hood on the upper back without its rolled edge, no pocket and no drawstrings; sleeves, forearms with the cuff merged; each hand as palm, one slab for the four fingers and a thumb of two segments; the sandwich as one roll of revolution, the 12 mm filling below the 15 mm feature of this scale merged into it; chinos seat with the gluteal mass merged, thighs with the knee merged, shanks with the calf; one shoe form per bone with the sole merged, no socks and no laces.",
                "Körper bei 1:100, neu gezeichnet mit weniger und längeren Querschnitten: Kopf mit Nase und Ohren unter der Haarkappe, ohne Augen, Brauen und Lippen; der Haarschwanz als eine Form ohne Haargummi; Hals; das Sweatshirt als eine Form, in der Saum- und Kragenbund aufgehen, die Kapuze auf dem oberen Rücken ohne Rollrand, keine Tasche und keine Kordeln; Ärmel, Unterarme mit eingeschlossenem Bündchen; jede Hand als Handfläche, eine Platte für die vier Finger und ein Daumen aus zwei Gliedern; das Sandwich als ein Drehkörper, der 12 mm dicke Belag unter dem kleinsten Merkmal von 15 mm dieses Maßstabs geht darin auf; Gesäß der Chinohose mit eingeschlossenem Gesäßmuskel, Oberschenkel mit eingeschlossenem Knie, Unterschenkel mit der Wade; eine Schuhform je Knochen mit eingeschlossener Sohle, keine Socken und keine Schnürung.")
THERMAL_100 = text("Thermal at 1:100: every region lies on the body forms of this scale; the layers are merged into the outer garment surface (the inner shells of T-shirt, briefs and socks are thinner than the smallest feature). The face is its own solid ahead of the hairline; hair cap and hair tail carry the hair, the roll the sandwich; breath leaves under the nose and at the lips, the body plume from a patch on the crown.",
                   "Thermik bei 1:100: Jede Region liegt auf den Körperformen dieses Maßstabs; die Schichten gehen in der äußeren Kleidungsfläche auf (die inneren Schalen von T-Shirt, Unterhose und Socken sind dünner als das kleinste Merkmal). Das Gesicht ist ein eigener Körper vor dem Haaransatz; Haarkappe und Haarschwanz tragen das Haar, das Brötchen das Sandwich; der Atem tritt unter der Nase und an den Lippen aus, die Körperfahne aus einem Fleck auf dem Scheitel.")
BODY_200 = text("Body at 1:200, one mass per part: head without nose and ears, a fuller hair cap, the hair tail as a mass of five sections, neck, sweatshirt with the hood as a second mass on the upper back, chinos seat, sleeves, forearms, one mass per hand for palm, fingers and thumb, the sandwich as a roll of revolution of its true size, thighs, shanks, a shoe and a toe box per foot.",
                "Körper bei 1:200, eine Masse je Teil: Kopf ohne Nase und Ohren, eine vollere Haarkappe, der Haarschwanz als Masse aus fünf Querschnitten, Hals, Sweatshirt mit der Kapuze als zweiter Masse auf dem oberen Rücken, Gesäß der Chinohose, Ärmel, Unterarme, eine Masse je Hand für Handfläche, Finger und Daumen, das Sandwich als Drehkörper in wahrer Größe, Oberschenkel, Unterschenkel, ein Schuh und eine Zehenkappe je Fuß.")
THERMAL_200 = text("Thermal at 1:200: every region lies on the masses of this scale, layers merged into the garment surface; the face is the head ahead of the hairline; the roll carries the sandwich; the three outlets are patches of 30 mm.", "Thermik bei 1:200: Jede Region liegt auf den Massen dieses Maßstabs, die Schichten gehen in der Kleidungsfläche auf; das Gesicht ist der Kopf vor dem Haaransatz; das Brötchen trägt das Sandwich; die drei Auslässe sind Flecken von 30 mm.")
BODY_500 = text("Body at 1:500, the fewest masses that read as a person with a hand raised to the mouth: head, hair cap with the hair tail merged into it, neck, sweatshirt with the hood merged into the upper back, chinos seat, and per side sleeve, forearm, hand, thigh, shank, shoe and toe box. Roll and bottle are kept at the 75 mm floor of the scale as bodies of revolution in the hands.",
                "Körper bei 1:500, die wenigsten Massen, die als Person mit zum Mund gehobener Hand lesbar sind: Kopf, Haarkappe mit darin aufgegangenem Haarschwanz, Hals, Sweatshirt mit im oberen Rücken aufgegangener Kapuze, Gesäß der Chinohose und je Seite Ärmel, Unterarm, Hand, Oberschenkel, Unterschenkel, Schuh und Zehenkappe. Brötchen und Flasche bleiben als Drehkörper in der Mindestgröße von 75 mm dieses Maßstabs in den Händen.")
THERMAL_500 = text("Thermal at 1:500: every region lies on the masses of this scale, layers merged; the face is the head ahead of the hairline. Roll and bottle carry their regions on their 75 mm bodies; the three outlets are patches of 75 mm.",
                   "Thermik bei 1:500: Jede Region liegt auf den Massen dieses Maßstabs, die Schichten sind zusammengefasst; das Gesicht ist der Kopf vor dem Haaransatz. Brötchen und Flasche tragen ihre Regionen auf ihren 75-mm-Körpern; die drei Auslässe sind Flecken von 75 mm.")
VOLUME_DECISIONS = {"50": (BODY_50, THERMAL_50), "100": (BODY_100, THERMAL_100), "200": (BODY_200, THERMAL_200), "500": (BODY_500, THERMAL_500)}


def replace_terms(t):
    rules = [("texting grip", "biting grip"), ("phone", "sandwich"), ("Telefon", "Sandwich"), ("Tippgriff", "Beißgriff"), ("denim", "twill"), ("Denim", "Köper"), ("Jeans", "Chinohose"), ("jeans", "chinos")]
    for a_, b_ in rules:
        t = t.replace(a_, b_)
    return t


BOTTLE = {"50": text(" The 0.5 L water bottle in the left hand is drawn as a body of revolution with shoulder, neck, label band and cap.", " Die 0,5-l-Wasserflasche in der linken Hand ist als Drehkörper mit Schulter, Hals, Etikettband und Deckel gezeichnet."),
          "100": text(" The water bottle in the left hand with its cap.", " Die Wasserflasche in der linken Hand mit ihrem Deckel."),
          "200": text(" The water bottle in the left hand as one mass.", " Die Wasserflasche in der linken Hand als eine Masse."),
          "500": text(" The roll and the water bottle in the left hand as small blocks of 75 mm.", " Das Brötchen und die Wasserflasche in der linken Hand als kleine Blöcke von 75 mm.")}
BOTTLE_VOL = {sc: text("The 0.5 L PET water bottle in the left hand is a body of revolution (205 mm, 65 mm round, cap included)" + (" at the floor of 75 mm" if sc == "500" else "") + "; it carries the bottle region and its own member.",
                       "Die 0,5-l-PET-Wasserflasche in der linken Hand ist ein Drehkörper (205 mm, 65 mm rund, mit Deckel)" + (" in der Mindestgröße von 75 mm" if sc == "500" else "") + "; sie trägt die Region der Flasche und ihr eigenes Bauteil.") for sc in SCALES}


def decisions(a):
    for sc in SCALES:
        dec = copy.deepcopy(DECISIONS[sc])
        dec["silhouette"] = {k: dec["silhouette"][k] + BOTTLE[sc][k] for k in ("en", "de")}
        a["representations"][sc]["decisions"] = dec
        old = a["volumes"][sc]["decisions"]
        body, therm = VOLUME_DECISIONS[sc]
        rest = []
        for d in old[2:]:
            if d["en"].startswith("Shoulder and pelvic girdle"):
                rest.append(GIRDLE)
            elif d["en"].startswith("The upper sleeves"):
                rest.append(SLEEVE)
            else:
                rest.append({"en": replace_terms(d["en"]), "de": replace_terms(d["de"])})
        a["volumes"][sc]["decisions"] = [body, therm] + rest + [BOTTLE_VOL[sc]]
    for r in a["representations"].values():
        for h in r.get("hatches", []):
            if h["id"] == "denim":
                h["id"] = "twill"
                h["title"] = text("Chino twill lines", "Köperlinien der Chinohose")
        for v in VIEWS:
            for sh in r["geometry"][v]:
                if sh.get("hatch") == "denim":
                    sh["hatch"] = "twill"
                if sh.get("region") == "chinos" or sh.get("regions") == ["chinos"]:
                    sh["tone"] = round(min(0.62, sh["tone"] + 0.3), 2)
