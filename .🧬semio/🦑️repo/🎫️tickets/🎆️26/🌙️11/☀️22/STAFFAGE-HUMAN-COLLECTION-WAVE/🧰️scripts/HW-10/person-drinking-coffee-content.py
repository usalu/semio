"""📝️ Identity, thermal scenario (hot coffee, steam), clips and per-scale decisions of person-drinking-coffee (imported by person-drinking-coffee.py)."""
from hw10lib import *

G = 9.80665
MASS = 64.0
HEAT_W = 50.0


def identity(a):
    a["title"] = text("Person Drinking Coffee", "Kaffee trinkende Person")
    a["summary"] = text(
        "A woman of 1.72 m on a cold morning with a steaming paper cup of coffee: short hair, hooded wool-blend jacket, corduroy trousers and suede sneakers. Four poses (holding the cup, blowing on the coffee, sipping, warming both hands round the cup) and two loops (a sip cycle and idle breathing) at about 1.2 met; the coffee is a hot water surface that steams, its wall follows it, and it carries no power of its own.",
        "Eine Frau von 1,72 m an einem kalten Morgen mit einem dampfenden Pappbecher Kaffee: kurzes Haar, Kapuzenjacke aus Wollmischung, Cordhose und Wildledersneaker. Vier Posen (Becher halten, auf den Kaffee pusten, Schluck nehmen, beide Hände am Becher wärmen) und zwei Schleifen (ein Trinkzyklus und ruhiges Atmen) bei etwa 1,2 met; der Kaffee ist eine heiße, dampfende Wasseroberfläche, die Becherwand folgt ihm, eine eigene Leistung hat er nicht.")
    a["tags"] = ["human", "adult", "woman", "standing", "drinking", "coffee", "steam", "cold-morning", "street-life"]
    a["presentation"] = {"defaultPose": "holding-cup", "defaultClip": "sip-cycle", "defaultScale": 100}
    prov = a["provenance"]
    prov["author"] = "Semio staffage human collection worker HW-10"
    prov["created"] = "2026-11-22"
    prov["references"] += [
        {"id": "iso-8996-2021", "citation": "ISO 8996:2021 Ergonomics of the thermal environment, determination of metabolic rate; Table A.1 metabolic rate of standing at ease 70 W/m2."},
        {"id": "incropera-2018", "citation": "Bergman, T. L., Lavine, A. S., Incropera, F. P. and DeWitt, D. P. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley; Table A.3 and A.6 thermophysical properties of paper, cardboard and water."},
        {"id": "brown-2008", "citation": "Brown, F. and Diller, K. R. (2008). Calculating the optimum temperature for serving hot beverages. Burns 34(5), 648-654; coffee is served at 70 to 85 degrees C and drunk at about 60 degrees C."}]
    prov["notes"] = text(
        "Derived from the archive's neutral standing adult: rig, outlines, 3D bodies and clothing scaled as one by 1.72/1.69 to a woman of 1.72 m barefoot (1.74 m shod) and 64 kg with the female segment fractions of de Leva (1996); the hooded sweatshirt, jeans and canvas sneakers are re-specified as a hooded wool-blend jacket over a long-sleeved top, corduroy trousers and suede sneakers for a cold morning. The 12 oz paper cup (112 mm high, 60 and 92 mm across, corrugated sleeve, coffee 11 mm below the rim, 370 g with the coffee) is a new rigid bone in the right hand; the fingers wrap round it with their own phalanges, and the four arm poses were fitted with a Nelder-Mead search on the rig (scripts in the ticket folder, cached as a design input) and checked in all three views.",
        "Abgeleitet von der neutralen stehenden erwachsenen Person des Archivs: Skelett, Umrisse, 3D-Körper und Kleidung gemeinsam um 1,72/1,69 auf eine Frau von 1,72 m barfuß (1,74 m mit Schuhen) und 64 kg skaliert, mit den weiblichen Segmentanteilen nach de Leva (1996); Kapuzensweatshirt, Jeans und Stoffsneaker sind für einen kalten Morgen als Kapuzenjacke aus Wollmischung über einem Langarmshirt, Cordhose und Wildledersneaker neu beschrieben. Der 12-oz-Pappbecher (112 mm hoch, 60 und 92 mm Durchmesser, Wellpapphülle, Kaffee 11 mm unter dem Rand, 370 g mit Kaffee) ist ein neuer starrer Knochen in der rechten Hand; die Finger umgreifen ihn mit ihren eigenen Gliedern, und die vier Armposen wurden mit einer Nelder-Mead-Suche am Skelett eingepasst (Skripte im Ticketordner, als Entwurfsgrundlage zwischengespeichert) und in allen drei Ansichten geprüft.")
    a["structure"]["assumptions"] = [
        text("Segments are point masses at the de Leva centres (female table, 64 kg); cup and coffee (370 g) are a carried load at their centre in the right hand.", "Segmente sind Punktmassen in den Schwerpunkten nach de Leva (Frauentabelle, 64 kg); Becher und Kaffee (370 g) sind eine getragene Last in ihrem Schwerpunkt in der rechten Hand."),
        text("Contacts are heel and ball of each foot; both feet stand flat at hip-joint width in every named pose and clip instant.", "Kontakte sind Ferse und Ballen jedes Fußes; beide Füße stehen in jeder benannten Pose und jedem Clip-Zeitpunkt flach in Hüftgelenkbreite.")]
    for b in a["rig"]["bones"]:
        if b["id"] == "torso":
            b["title"] = text("Thorax with jacket", "Brustkorb mit Jacke")


def materials(a):
    rename_material(a, "fleece-sweatshirt", "wool-jacket")
    rename_material(a, "denim", "corduroy")
    rename_material(a, "canvas-sneaker", "suede-sneaker")
    for m in a["materials"]:
        if m["id"] == "wool-jacket":
            m["title"] = text("Wool-blend jacket cloth", "Jackenstoff aus Wollmischung")
            m["solarAbsorptivity"] = 0.7
        if m["id"] == "corduroy":
            m["title"] = text("Cotton corduroy", "Baumwollcord")
            m["solarAbsorptivity"] = 0.7
        if m["id"] == "suede-sneaker":
            m["title"] = text("Suede sneaker with rubber sole", "Wildledersneaker mit Gummisohle")
            m["emissivity"] = 0.95
    a["materials"] += [
        {"id": "paper-cup", "title": text("Coated paperboard cup", "Beschichteter Pappbecher"), "emissivity": 0.9, "solarAbsorptivity": 0.25, "densityKgM3": 800, "specificHeatJKgK": 1400, "source": "authored-estimate", "reference": "incropera-2018"},
        {"id": "cardboard-sleeve", "title": text("Corrugated cardboard sleeve", "Hülle aus Wellpappe"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 150, "specificHeatJKgK": 1300, "source": "authored-estimate", "reference": "incropera-2018"},
        {"id": "coffee", "title": text("Black coffee", "Schwarzer Kaffee"), "emissivity": 0.96, "solarAbsorptivity": 0.95, "densityKgM3": 1000, "specificHeatJKgK": 4180, "source": "authored-estimate", "reference": "incropera-2018"}]


def thermal(a):
    t = a["thermal"]
    t["environment"] = {"airTempC": 7, "meanRadiantTempC": 6, "skyTempC": -6, "groundTempC": 6, "relativeHumidity": 0.75, "windMS": 0.5, "windDirectionDeg": 180, "solarWM2": 0, "diffuseFraction": 1, "pressurePa": 101325}
    t["baseMet"] = 1.2
    skin = 0.007184 * MASS ** 0.425 * 172 ** 0.725
    t["body"].update({"massKg": MASS, "skinAreaM2": round(skin, 3), "basalMetabolismW": round((10 * MASS + 6.25 * 172 - 5 * 45 - 161) * 4184 / 86400, 1)})
    rename_region(a, "hoodie", "jacket")
    rename_region(a, "jeans", "trousers")
    rename_layer(a, "trousers", "jeans", "corduroy")
    rename_layer(a, "jacket", "sweatshirt", "jacket")
    rename_layer(a, "jacket", "tee", "top")
    regs = {r["id"]: r for r in t["regions"]}
    regs["jacket"]["title"] = text("Long-sleeved top and hooded wool-blend jacket on trunk and arms", "Langarmshirt und Kapuzenjacke aus Wollmischung an Rumpf und Armen")
    regs["jacket"]["layers"][0].update({"title": text("Long-sleeved cotton top", "Langarmshirt aus Baumwolle"), "resistanceM2KW": 0.0279, "vapourResistanceM2PaW": 4.5})
    regs["jacket"]["layers"][1].update({"title": text("Hooded wool-blend jacket, hood down", "Kapuzenjacke aus Wollmischung, Kapuze unten"), "material": "wool-jacket", "thicknessM": 0.006, "resistanceM2KW": 0.0899, "vapourResistanceM2PaW": 14.3, "heatCapacityJM2K": 700})
    regs["trousers"]["title"] = text("Corduroy trousers over the legs", "Cordhose über den Beinen")
    regs["trousers"]["layers"][1].update({"title": text("Cotton corduroy trousers", "Cordhose aus Baumwolle"), "material": "corduroy", "thicknessM": 0.002, "resistanceM2KW": 0.0527, "vapourResistanceM2PaW": 8.4})
    regs["sneakers"]["title"] = text("Socks and suede sneakers", "Socken und Wildledersneaker")
    regs["sneakers"]["layers"][1].update({"title": text("Suede sneaker", "Wildledersneaker"), "material": "suede-sneaker", "resistanceM2KW": 0.0078, "vapourResistanceM2PaW": 3.2})
    regs["hair"]["layers"][0]["title"] = text("Short hair", "Kurzes Haar")
    t["operating"] = [{"id": "coffee", "title": text("Coffee in the cup", "Kaffee im Becher"), "levels": [{"v": 0, "title": text("Empty or gone cold", "Leer oder kalt geworden")}, {"v": 0.6, "title": text("Warm, after a quarter of an hour", "Warm, nach einer Viertelstunde")}, {"v": 1, "title": text("Freshly poured", "Frisch eingeschenkt")}]}]
    t["regions"] += [
        {"id": "coffee", "title": text("Surface of the hot coffee", "Oberfläche des heißen Kaffees"), "kind": "water", "model": "operating", "material": "coffee", "areaM2": 0.0058, "characteristicLengthM": 0.086, "geometry": "horizontal-plate-up", "skyViewFactor": 0.6,
         "layers": [{"id": "coffee-skin", "title": text("Coffee near the surface", "Kaffee an der Oberfläche"), "kind": "polymer", "material": "coffee", "thicknessM": 0.01, "resistanceM2KW": 0.0017, "heatCapacityJM2K": 41800, "source": "authored-estimate", "reference": "incropera-2018"}],
         "heatCapacityJM2K": 41800, "operating": {"key": "coffee", "heatW": HEAT_W}, "ratedPowerW": 0, "ratedPowerBasis": "passive", "source": "authored-estimate", "reference": "incropera-2018"},
        {"id": "cup", "title": text("Paper cup wall with its sleeve", "Wand des Pappbechers mit Hülle"), "kind": "surface", "model": "coupled", "material": "paper-cup", "areaM2": 0.034, "characteristicLengthM": 0.076, "geometry": "cylinder", "skyViewFactor": 0.4,
         "layers": [{"id": "cup-wall", "title": text("Coated paperboard 0.5 mm with the corrugated sleeve over 45 % of the wall, averaged", "Beschichteter Karton 0,5 mm mit der Wellpapphülle über 45 % der Wand, gemittelt"), "kind": "insulation", "material": "paper-cup", "thicknessM": 0.0015, "resistanceM2KW": 0.0205, "heatCapacityJM2K": 740, "source": "authored-estimate", "reference": "incropera-2018"}],
         "heatCapacityJM2K": 740, "coupled": {"region": "coffee", "wetFraction": 0.9, "filmM2KW": 0.004}, "source": "authored-estimate", "reference": "incropera-2018"}]
    t["emitters"].append({"id": "coffee-steam", "kind": "vent", "title": text("Steam rising from the coffee", "Dampf über dem Kaffee"), "anchor": "coffee-surface", "directionDeg": 0, "diameterM": 0.08, "operatingKey": "coffee", "temperatureRiseK": 30, "vapourKgPerKg": 0.06, "region": "coffee", "source": "authored-estimate", "reference": "incropera-2018"})
    t["assumptions"] = [
        text(f"Skin area {t['body']['skinAreaM2']} m2 after DuBois and DuBois (1916) for 64 kg and 1.72 m; regions carry skin fractions 0.07 face and neck, 0.03 scalp, 0.05 hands, 0.49 trunk and arms, 0.30 legs, 0.06 feet (rule-of-nines proportions, authored).",
             f"Hautfläche {str(t['body']['skinAreaM2']).replace('.', ',')} m2 nach DuBois und DuBois (1916) für 64 kg und 1,72 m; die Regionen tragen die Hautanteile 0,07 Gesicht und Hals, 0,03 Kopfhaut, 0,05 Hände, 0,49 Rumpf und Arme, 0,30 Beine, 0,06 Füße (Neunerregel-Proportionen, geschätzt)."),
        text("Basal metabolism from the Mifflin-St Jeor equation for a 45-year-old woman of 64 kg and 172 cm. Clothing for a cold morning after the ISO 9920 tables (1 clo = 0.155 m2K/W): long-sleeved top and wool-blend jacket 0.76 clo on trunk and arms, briefs and corduroy 0.37 clo, socks and sneakers 0.07 clo, digits authored; vapour resistance R/(0.0165 K/Pa x 0.38). The reference surroundings are a still, overcast morning at 7 degrees C.",
             "Grundumsatz aus der Mifflin-St-Jeor-Gleichung für eine 45-jährige Frau mit 64 kg und 172 cm. Kleidung für einen kalten Morgen nach den Tabellen von ISO 9920 (1 clo = 0,155 m2K/W): Langarmshirt und Jacke aus Wollmischung 0,76 clo an Rumpf und Armen, Unterhose und Cordhose 0,37 clo, Socken und Sneaker 0,07 clo, Werte geschätzt; Dampfwiderstand R/(0,0165 K/Pa x 0,38). Die Referenzumgebung ist ein windstiller, bedeckter Morgen bei 7 Grad C."),
        text("Standing at ease is 1.2 met (ISO 8996 Table A.1, 70 W/m2); lifting the cup and blowing are taken as 1.3 met (authored). Breathing patterns are authored and guided by quiet adult values (ICRP 66); blowing is a long exhalation through pursed lips.",
             "Entspanntes Stehen sind 1,2 met (ISO 8996 Tabelle A.1, 70 W/m2); das Heben des Bechers und das Pusten werden mit 1,3 met angesetzt (geschätzt). Die Atemmuster sind geschätzt und an ruhigen Erwachsenenwerten (ICRP 66) orientiert; Pusten ist ein langes Ausatmen durch gespitzte Lippen."),
        text(f"The coffee is a water surface of 0.0058 m2 that the operating level holds warm: the {HEAT_W:.0f} W at level 1 stand for the enthalpy 330 g of freshly poured coffee give off while they cool by about 2.2 K per minute in the open cup on a cold morning (330 g x 4.18 kJ/(kg K) x 2.2 K / 60 s = 50 W, evaporation from the open surface taking the largest share), which holds the surface near 65 degrees C, the drinking range of Brown and Diller (2008); it is released heat, not a nameplate, so the cup counts 0 W of power. The surface evaporates and feeds the steam vent; the paper wall is coupled to the coffee below the fill line through the paper and the averaged sleeve; the hand contact is not modelled.",
             f"Der Kaffee ist eine Wasseroberfläche von 0,0058 m2, die die Betriebsstufe warm hält: Die {HEAT_W:.0f} W bei Stufe 1 stehen für die Enthalpie, die 330 g frisch eingeschenkter Kaffee abgeben, während sie im offenen Becher an einem kalten Morgen um etwa 2,2 K pro Minute abkühlen (330 g x 4,18 kJ/(kg K) x 2,2 K / 60 s = 50 W, den größten Teil trägt die Verdunstung von der offenen Oberfläche), was die Oberfläche nahe 65 Grad C hält, im Trinkbereich nach Brown und Diller (2008); sie ist abgegebene Wärme, keine Nennleistung, der Becher zählt also 0 W Leistung. Die Oberfläche verdunstet und speist den Dampfauslass; die Pappwand ist unterhalb der Füllhöhe über Papier und gemittelte Hülle an den Kaffee gekoppelt; der Handkontakt wird nicht modelliert.")]
    resp = t["respiration"]
    resp["patterns"] = [
        {"id": "quiet-standing", "title": text("Quiet standing", "Ruhiges Stehen"), "rateBpm": 14, "tidalVolumeL": 0.45, "inhaleFraction": 0.38, "pauseAfterInhaleFraction": 0.04, "exhaleFraction": 0.45, "pauseAfterExhaleFraction": 0.13, "route": "nose", "source": "authored-estimate", "reference": "icrp-66-1994"},
        {"id": "blowing", "title": text("Blowing on the coffee", "Auf den Kaffee pusten"), "rateBpm": 10, "tidalVolumeL": 0.8, "inhaleFraction": 0.25, "pauseAfterInhaleFraction": 0.05, "exhaleFraction": 0.6, "pauseAfterExhaleFraction": 0.1, "route": "mouth", "source": "authored-estimate", "reference": "gupta-2010"},
        {"id": "sipping", "title": text("Holding the breath for a sip", "Atem anhalten für einen Schluck"), "rateBpm": 12, "tidalVolumeL": 0.4, "inhaleFraction": 0.3, "pauseAfterInhaleFraction": 0.25, "exhaleFraction": 0.35, "pauseAfterExhaleFraction": 0.1, "route": "nose", "source": "authored-estimate", "reference": "icrp-66-1994"}]
    if not any(r["id"] == "icrp-66-1994" for r in a["provenance"]["references"]):
        a["provenance"]["references"].append({"id": "icrp-66-1994", "citation": "ICRP (1994). Human respiratory tract model for radiological protection. ICRP Publication 66, Annals of the ICRP 24(1-3); reference ventilation rates by activity (used as guidance, values authored)."})


POSES = {
    "holding-cup": (text("Holding the cup", "Becher halten"), text("Feet at hip width, head level; the right elbow is bent about 90 degrees and the fingers wrap round the cup, which stands upright in front of the waist and steams; the left arm hangs relaxed.", "Füße hüftbreit, Kopf gerade; der rechte Ellbogen ist etwa 90 Grad gebeugt und die Finger umfassen den Becher, der aufrecht vor der Taille steht und dampft; der linke Arm hängt locker."), 1.2, "quiet-standing", 0.0),
    "blowing-on-coffee": (text("Blowing on the coffee", "Auf den Kaffee pusten"), text("The cup is raised in front of the chin, 9 cm ahead and 7 cm below the lips, the head bowed over it; the breath goes through pursed lips across the coffee.", "Der Becher ist vor das Kinn gehoben, 9 cm vor und 7 cm unter den Lippen, der Kopf darüber gebeugt; der Atem geht durch gespitzte Lippen über den Kaffee."), 1.3, "blowing", 0.0),
    "sipping": (text("Sipping", "Schluck nehmen"), text("The right elbow is raised forward and the hand tilts the cup about 37 degrees towards the face with its rim at the lower lip; the head tips back slightly.", "Der rechte Ellbogen ist nach vorn gehoben und die Hand kippt den Becher um etwa 37 Grad zum Gesicht, der Rand an der Unterlippe; der Kopf neigt sich leicht zurück."), 1.3, "sipping", 0.0),
    "warming-hands": (text("Warming the hands on the cup", "Hände am Becher wärmen"), text("Shoulders drawn up against the cold, both hands wrap round the cup close in front of the chest, the head bowed towards the steam.", "Die Schultern gegen die Kälte hochgezogen, beide Hände umfassen den Becher nahe vor der Brust, der Kopf zum Dampf geneigt."), 1.2, "quiet-standing", 0.0),
}


def bind_poses(a):
    for p in a["poses"]:
        title, desc, met, breath, _ = POSES[p["id"]]
        p.pop("key", None)
        p.update({"title": title, "description": desc, "ground": "auto", "contacts": ["heel-l", "ball-l", "heel-r", "ball-r"]})
        p["thermal"] = {"activityMet": met, "breathing": breath,
                        "regions": region_binding(a, {"jacket": 0.95, "hand-skin": 0.55 if p["id"] == "warming-hands" else 0.8, "cup": 0.65 if p["id"] == "warming-hands" else 0.8},
                                                  {"sneakers": {"contact": {"fraction": 0.3, "tempC": 6}}, "coffee": {"wettedness": 1}}),
                        "emitters": {"body-plume": 0.6, "coffee-steam": 1}, "operating": {"coffee": 1}}
        p["loads"] = [{"id": "cup-weight", "kind": "carried", "title": text("Weight of cup and coffee", "Gewicht von Becher und Kaffee"), "anchor": "cup-cog", "fxN": 0, "fyN": round(-0.37 * G, 3), "source": "authored-estimate"}]


def clips(a):
    feet = lambda d: [{"anchor": x, "fromS": 0, "toS": d} for x in ("heel-l", "ball-l", "heel-r", "ball-r")]
    sip = {"id": "sip-cycle", "title": text("Sip cycle", "Trinkzyklus"),
           "description": text("A ten-second loop: the cup steams in front of the waist, rises to the chin, she blows across the coffee twice, tilts it to the lips for a sip, lowers it again and swallows; the feet stay planted and the steam follows the cup.",
                               "Eine Schleife von zehn Sekunden: Der Becher dampft vor der Taille, steigt zum Kinn, sie pustet zweimal über den Kaffee, kippt ihn für einen Schluck an die Lippen, senkt ihn wieder und schluckt; die Füße bleiben stehen und der Dampf folgt dem Becher."),
           "durationS": 10, "fps": 30, "loop": True, "rootMotion": "none",
           "keys": [{"t": 0, "pose": "holding-cup", "ease": "smooth"}, {"t": 1.6, "pose": "holding-cup", "ease": "smooth"}, {"t": 3.0, "pose": "blowing-on-coffee", "ease": "smooth"}, {"t": 4.8, "pose": "blowing-on-coffee", "ease": "smooth"},
                    {"t": 5.8, "pose": "sipping", "ease": "smooth"}, {"t": 6.8, "pose": "sipping", "ease": "smooth"}, {"t": 8.2, "pose": "holding-cup", "ease": "smooth"}, {"t": 10, "pose": "holding-cup", "ease": "smooth"}],
           "contacts": feet(10),
           "phases": [{"id": "hold", "fromS": 0, "toS": 1.6, "title": text("Holding", "Halten")}, {"id": "raise", "fromS": 1.6, "toS": 3.0, "title": text("Raising the cup", "Becher heben")}, {"id": "blow", "fromS": 3.0, "toS": 4.8, "title": text("Blowing to cool", "Kühl pusten")},
                      {"id": "sip", "fromS": 4.8, "toS": 6.8, "title": text("Sipping", "Trinken")}, {"id": "lower", "fromS": 6.8, "toS": 10, "title": text("Lowering and swallowing", "Senken und Schlucken")}],
           "events": [{"t": 6.0, "kind": "thermal", "title": text("Hot coffee at the lips", "Heißer Kaffee an den Lippen"), "target": "coffee"}],
           "tracks": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 3.0, "v": 1}, {"t": 4.8, "v": 2}, {"t": 7.4, "v": 0}, {"t": 10, "v": 0}]},
                      {"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 1.2}, {"t": 3.0, "v": 1.3}, {"t": 6.8, "v": 1.3}, {"t": 10, "v": 1.2}]},
                      {"target": "operating:coffee", "ease": "linear", "keys": [{"t": 0, "v": 1}, {"t": 10, "v": 1}]}]}
    idle = {"id": "idle-breathing", "title": text("Idle breathing", "Ruhiges Atmen"),
            "description": text("A five-second loop with both hands round the cup: the chest rises and falls with quiet breathing, the bowed head settles in a small slow sway while the coffee steams; the feet stay planted.",
                                "Eine Schleife von fünf Sekunden mit beiden Händen am Becher: Die Brust hebt und senkt sich mit ruhigem Atmen, der geneigte Kopf wiegt sich klein und langsam, während der Kaffee dampft; die Füße bleiben stehen."),
            "durationS": 5, "fps": 30, "loop": True, "rootMotion": "none",
            "keys": [{"t": 0, "pose": "warming-hands", "ease": "smooth"}, {"t": 1.25, "pose": "warming-hands", "ease": "smooth", "bones": {"head": {"angleDeg": -6.5}, "torso": {"angleDeg": -1.8}}},
                     {"t": 2.5, "pose": "warming-hands", "ease": "smooth", "bones": {"head": {"angleDeg": -8.5}, "torso": {"angleDeg": -1.2}}}, {"t": 3.75, "pose": "warming-hands", "ease": "smooth", "bones": {"head": {"angleDeg": -7}, "torso": {"angleDeg": -1.7}}},
                     {"t": 5, "pose": "warming-hands", "ease": "smooth"}],
            "contacts": feet(5), "phases": [{"id": "idle", "fromS": 0, "toS": 5, "title": text("Quiet standing", "Ruhiges Stehen")}], "events": [],
            "tracks": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 5, "v": 0}]}, {"target": "operating:coffee", "ease": "linear", "keys": [{"t": 0, "v": 1}, {"t": 5, "v": 1}]}]}
    a["clips"] = [sip, idle]


def decisions(a):
    import importlib.util
    spec = importlib.util.spec_from_file_location("dec", f"{H}/person-drinking-coffee-decisions.py")
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
