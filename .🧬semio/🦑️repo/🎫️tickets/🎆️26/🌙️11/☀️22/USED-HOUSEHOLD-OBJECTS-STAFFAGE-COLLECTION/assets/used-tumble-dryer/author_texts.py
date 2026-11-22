"""📚️ Texts, materials, rig, anchors, thermal regions, poses, clips and references of the used heat-pump tumble dryer.

Imported by `author-tumble-dryer.py`, which adds the per-scale representations and volumes.

@see ./author-tumble-dryer.py
"""

Y0 = 0.016
UNIT_KG = 46.5
DOOR_KG = 2.6
TANK_KG = 0.45
BLANKET_KG = 1.22
UNIT_COM = (0.29, 0.311)
DOOR_COM = (0.0, -0.02)
TANK_COM = (0.17, 0.0)
BLANKET_COM = (0.45, 0.008)
LAUNDRY_N = -119.6


def t(en, de):
    """🗣️ A bilingual text."""
    return {"en": en, "de": de}


def mat(mid, en, de, em, sa, rho, k, c, src="literature", ref="incropera-2018", **kw):
    """🧪️ One material."""
    m = {"id": mid, "title": t(en, de), "emissivity": em, "solarAbsorptivity": sa, "densityKgM3": rho, "conductivityWmK": k, "specificHeatJKgK": c}
    m.update(kw)
    m.update(source=src, reference=ref)
    return m


MATERIALS = [
    mat("enamel-steel", "Yellowed white powder-coated steel sheet DC01, 0.8 mm (aged: solar absorptance 0.32 against 0.25 new)", "Vergilbtes weiß pulverbeschichtetes Stahlblech DC01, 0,8 mm (gealtert: solarer Absorptionsgrad 0,32 statt 0,25 neu)", 0.9, 0.32, 7850, 50, 460, src="authored-estimate", ref="en-10130", elasticModulusPa=210e9, strengthPa=200e6),
    mat("top-pp", "Scratched white polypropylene top cover", "Zerkratzte weiße Polypropylen-Abdeckplatte", 0.92, 0.35, 905, 0.22, 1900, src="authored-estimate", ref="en-iso-10456"),
    mat("fascia-abs", "Sun-faded light grey ABS fascia with bleached print", "Sonnengebleichte hellgraue ABS-Blende mit verblasstem Druck", 0.9, 0.42, 1050, 0.17, 1400, src="authored-estimate", ref="en-iso-10456"),
    mat("display-glass", "Dark display window", "Dunkles Displayfenster", 0.9, 0.85, 2500, 1.0, 840),
    mat("door-abs", "Yellowed white ABS door ring", "Vergilbter weißer ABS-Türring", 0.9, 0.48, 1050, 0.17, 1400, src="authored-estimate", ref="en-iso-10456"),
    mat("door-glass", "Tinted borosilicate door bowl with lint film", "Getönte Borosilikat-Türschale mit Flusenfilm", 0.92, 0.3, 2230, 1.14, 830),
    mat("hinge-zinc", "Die-cast zinc door hinge", "Türscharnier aus Zinkdruckguss", 0.25, 0.6, 6600, 110, 390),
    mat("tank-pp", "Translucent white polypropylene condensate tank", "Durchscheinender weißer Polypropylen-Kondenswasserbehälter", 0.92, 0.3, 905, 0.22, 1900, ref="en-iso-10456"),
    mat("filter-pp", "Polypropylene fluff filter frame with PET mesh, lint in the mesh", "Flusensieb-Rahmen aus Polypropylen mit PET-Gewebe, Flusen im Gewebe", 0.92, 0.5, 600, 0.2, 1900, src="authored-estimate"),
    mat("base-pp", "Grey polypropylene base and plinth flap, kick marks", "Grauer Polypropylen-Boden und Sockelklappe mit Trittspuren", 0.92, 0.6, 905, 0.22, 1900, ref="en-iso-10456"),
    mat("foot-rubber", "Rubber-padded screw foot", "Schraubfuß mit Gummiauflage", 0.92, 0.9, 1200, 0.2, 2000),
    mat("pvc-cable", "White PVC mains cable H05VV-F 3G1.5", "Weißes PVC-Netzkabel H05VV-F 3G1,5", 0.9, 0.4, 1700, 0.3, 1000, src="authored-estimate"),
    mat("stainless-steel", "Brushed stainless-steel drum 0.6 mm", "Gebürstete Edelstahltrommel 0,6 mm", 0.35, 0.5, 7900, 15, 500),
    mat("compressor-steel", "Black painted rotary compressor shell", "Schwarz lackierter Rollkolbenverdichter-Mantel", 0.9, 0.9, 7850, 50, 460),
    mat("aluminium-fin", "Aluminium-finned evaporator and condenser with copper tubes", "Verdampfer und Verflüssiger mit Aluminiumlamellen und Kupferrohren", 0.2, 0.4, 2700, 200, 900),
    mat("motor-steel", "Drum motor and fan, steel and copper", "Trommelmotor und Gebläse, Stahl und Kupfer", 0.8, 0.7, 7850, 50, 460),
    mat("moving-blanket", "Quilted moving blanket, recycled cotton and polyester felt, 0.45 kg/m2", "Gesteppte Umzugsdecke aus Recycling-Baumwoll-Polyester-Filz, 0,45 kg/m2", 0.9, 0.75, 112, 0.045, 1340, src="authored-estimate", ref="moving-blanket-catalogue"),
    mat("packing-tape", "Polypropylene packing tape and paper price sticker", "Polypropylen-Packband und Papier-Preisaufkleber", 0.9, 0.4, 910, 0.2, 1900, src="authored-estimate"),
]

REGION_IDS = ["cabinet", "top-cover", "fascia", "tank", "door", "plinth", "cable", "blanket", "drum", "heat-pump"]
AREAS = {"cabinet": 1.63, "top-cover": 0.41, "fascia": 0.076, "tank": 0.206, "door": 0.217, "plinth": 0.52, "cable": 0.027, "blanket": 1.4, "drum": 1.289, "heat-pump": 0.49}
HIDDEN = t("Inside the closed steel cabinet behind the door and the plinth flap; through the tinted door glass only a dim reflection of the drum shows.", "Im geschlossenen Stahlgehäuse hinter Tür und Sockelklappe; durch das getönte Türglas zeigt sich nur ein schwacher Widerschein der Trommel.")


def region(rid, en, de, kind, model, material, length, geometry, sky, hc, src="authored-estimate", ref="incropera-2018", **kw):
    """🌡️ One thermal region."""
    r = {"id": rid, "title": t(en, de), "kind": kind, "model": model, "material": material, "areaM2": AREAS[rid], "characteristicLengthM": length, "geometry": geometry, "skyViewFactor": sky, "layers": [], "heatCapacityJM2K": hc}
    r.update(kw)
    r.update(source=src, reference=ref)
    return r


def regions():
    """🌡️ The ten thermal regions; the nameplate of the whole machine is stated once, on the heat-pump module."""
    return [
        region("cabinet", "Steel cabinet: side, back and front panels, yellowed white", "Stahlgehäuse: Seiten-, Rück- und Frontblech, vergilbt weiß", "surface", "operating", "enamel-steel", 0.8, "vertical-plate", 0.1, 2890, operating={"key": "heat-pump", "heatW": 140}),
        region("top-cover", "Scratched top cover with price sticker", "Zerkratzte Abdeckplatte mit Preisaufkleber", "surface", "passive", "top-pp", 0.6, "horizontal-plate-up", 0.2, 5160, ref="en-iso-10456"),
        region("fascia", "Faded control fascia with programme knob and display", "Verblasste Bedienblende mit Programmwähler und Display", "surface", "operating", "fascia-abs", 0.11, "vertical-plate", 0.1, 4410, operating={"key": "controls", "heatW": 4}),
        region("tank", "Condensate tank drawer, emptied", "Kondenswasserbehälter-Schublade, geleert", "surface", "passive", "tank-pp", 0.11, "vertical-plate", 0.1, 2580),
        region("door", "Porthole door: yellowed ring, domed glass, fluff filter", "Bullaugentür: vergilbter Ring, gewölbtes Glas, Flusensieb", "surface", "operating", "door-abs", 0.46, "vertical-plate", 0.1, 7500, operating={"key": "heat-pump", "heatW": 25}),
        region("plinth", "Base, plinth flap with grille and screw feet", "Boden, Sockelklappe mit Lüftungsgitter und Schraubfüße", "surface", "passive", "base-pp", 0.15, "vertical-plate", 0.05, 6870, ref="en-iso-10456"),
        region("cable", "Mains cable coiled and taped to the back", "Netzkabel aufgerollt und an die Rückwand geklebt", "surface", "passive", "pvc-cable", 0.03, "cylinder", 0.1, 3000),
        region("blanket", "Folded moving blanket left on the floor", "Gefaltete Umzugsdecke, am Boden zurückgelassen", "surface", "passive", "moving-blanket", 0.75, "horizontal-plate-up", 0.2, 2340, ref="moving-blanket-catalogue"),
        region("drum", "Stainless drum warmed by the 55 degC process air", "Edelstahltrommel, von der 55 °C warmen Prozessluft erwärmt", "surface", "operating", "stainless-steel", 0.54, "cylinder", 0.0, 2370, hidden=HIDDEN, operating={"key": "heat-pump", "heatW": 120}, ref="en-61121"),
        region("heat-pump", "Heat-pump module in the base: rotary compressor, evaporator, condenser, drum motor and fan", "Wärmepumpenmodul im Boden: Rollkolbenverdichter, Verdampfer, Verflüssiger, Trommelmotor und Gebläse", "engine", "operating", "compressor-steel", 0.15, "cylinder", 0.0, 16000,
               hidden=HIDDEN, operating={"key": "heat-pump", "heatW": 180}, ratedPowerW=1000, ratedPowerBasis="electrical-input", ref="catalogue-typical"),
    ]


OPERATING = [
    {"id": "heat-pump", "title": t("Heat pump, drum drive and fans", "Wärmepumpe, Trommelantrieb und Gebläse"), "levels": [{"v": 0, "title": t("Off", "Aus")}, {"v": 0.25, "title": t("Cool-down: drum and fans only", "Abkühlen: nur Trommel und Gebläse")}, {"v": 1, "title": t("Drying, cotton cupboard dry (about 0.6 kW mean of the 1 kW connected load)", "Trocknen, Baumwolle schranktrocken (etwa 0,6 kW Mittel des Anschlusswerts von 1 kW)")}]},
    {"id": "controls", "title": t("Electronics and display", "Elektronik und Display"), "levels": [{"v": 0, "title": t("Unplugged", "Ausgesteckt")}, {"v": 1, "title": t("On", "An")}]},
]


def binding(hp, ctl, out_of_blanket):
    """🌬️ A thermal binding of every region and the plinth vent."""
    regs = {r: {"exposedFraction": 1, "sunlitFraction": 0} for r in REGION_IDS}
    regs["drum"] = {"exposedFraction": 0, "sunlitFraction": 0}
    regs["heat-pump"] = {"exposedFraction": 0, "sunlitFraction": 0}
    regs["blanket"] = {"exposedFraction": 0.5 if out_of_blanket else 0.3, "sunlitFraction": 0, "contact": {"fraction": 0.5, "tempC": 16}}
    return {"operating": {"heat-pump": hp, "controls": ctl}, "regions": regs, "emitters": {"plinth-vent": hp}}


FEET = ["foot-front-right", "foot-front-left", "foot-back-right", "foot-back-left"]
GRIPS = ["grip-front", "grip-back"]
IN_USE = {"blanket": {"dx": -1.0}, "lift": {"dy": -Y0}}
LAUNDRY = {"id": "laundry", "kind": "carried", "title": t("8 kg cotton at 53 % residual moisture after a 1400 rpm spin, 12.2 kg in the drum", "8 kg Baumwolle mit 53 % Restfeuchte nach 1400 U/min Schleudern, 12,2 kg in der Trommel"),
           "bone": "unit", "x": 0.28, "y": 0.455, "fxN": 0, "fyN": LAUNDRY_N, "source": "authored-estimate", "reference": "en-61121"}

POSES = [
    {"id": "stored", "title": t("Stored on a moving blanket", "Auf einer Umzugsdecke gelagert"),
     "description": t("Taken out of a flat clearance, the condensate tank emptied, the fluff filters brushed out, the mains cable coiled and taped to the back and the door taped shut: the dryer stands on its four screw feet on a moving blanket folded four times in the hall of the second-hand department store.",
                      "Aus einer Haushaltsauflösung geholt, Kondenswasserbehälter geleert, Flusensiebe ausgebürstet, Netzkabel aufgerollt und an die Rückwand geklebt, Tür zugeklebt: der Trockner steht auf seinen vier Schraubfüßen auf einer viermal gefalteten Umzugsdecke in der Halle des Gebrauchtwarenkaufhauses."),
     "bones": {}, "ground": "authored", "contacts": ["blanket-floor"], "thermal": binding(0, 0, False), "loads": []},
    {"id": "carried", "title": t("Carried by two people", "Von zwei Personen getragen"),
     "description": t("Two people take the 49.6 kg dryer under the front and back edges of its base, lift it 0.30 m and tilt it back 8 degrees so the door stays shut against the tape; the blanket stays on the floor.",
                      "Zwei Personen fassen den 49,6 kg schweren Trockner unter der Vorder- und Hinterkante des Bodens, heben ihn 0,30 m an und neigen ihn 8 Grad nach hinten, damit die Tür am Klebeband geschlossen bleibt; die Decke bleibt am Boden."),
     "bones": {"lift": {"dy": 0.3}, "unit": {"angleDeg": 8}}, "ground": "authored", "contacts": ["blanket-floor"] + GRIPS, "thermal": binding(0, 0, False), "loads": []},
    {"id": "in-use", "title": t("Drying in a utility room", "Trocknend im Hauswirtschaftsraum"),
     "description": t("Back in a home beside the washing machine, levelled on its feet and plugged in: the heat pump dries 8 kg of cotton at about 55 degC process air in a closed loop, the condensate runs into the tank, the cabinet and the door warm up, and the compressor fan blows warm air out of the plinth grille.",
                      "Wieder in einer Wohnung neben der Waschmaschine, auf seinen Füßen ausgerichtet und eingesteckt: die Wärmepumpe trocknet 8 kg Baumwolle mit etwa 55 °C warmer Prozessluft im geschlossenen Kreis, das Kondensat läuft in den Behälter, Gehäuse und Tür erwärmen sich und das Verdichtergebläse bläst warme Luft aus dem Sockelgitter."),
     "bones": IN_USE, "ground": "authored", "contacts": ["blanket-floor"] + FEET, "thermal": binding(1, 1, True), "loads": [LAUNDRY]},
    {"id": "door-open", "title": t("Door open, fluff filter pulled", "Tür offen, Flusensieb gezogen"),
     "description": t("After the cycle the door swings open by 100 degrees on its right-hand hinge and the fluff filter is pulled 0.12 m up out of the aperture to brush off the lint.",
                      "Nach dem Programm schwenkt die Tür am rechten Scharnier um 100 Grad auf und das Flusensieb wird 0,12 m aus der Einfüllöffnung gezogen, um die Flusen abzustreifen."),
     "bones": dict(IN_USE, **{"door": {"twistDeg": -100}, "lint-filter": {"dy": 0.12}}), "ground": "authored", "contacts": ["blanket-floor"] + FEET, "thermal": binding(0, 1, True), "loads": []},
    {"id": "tank-out", "title": t("Condensate tank pulled out", "Kondenswasserbehälter herausgezogen"),
     "description": t("The condensate tank drawer at the top left of the fascia is pulled 0.28 m out to its stop to pour the 4.8 litres of condensate away.",
                      "Die Schublade des Kondenswasserbehälters oben links in der Blende ist 0,28 m bis zum Anschlag herausgezogen, um die 4,8 Liter Kondensat auszugießen."),
     "bones": dict(IN_USE, **{"tank": {"dx": 0.28}}), "ground": "authored", "contacts": ["blanket-floor"] + FEET, "thermal": binding(0, 1, True), "loads": []},
    {"id": "loaded-key", "key": True, "title": t("Loaded, before start", "Beladen, vor dem Start"),
     "description": t("Key of the drying cycle: door shut on 8 kg of damp cotton, machine switched on, heat pump still off.", "Schlüssel des Trockenprogramms: Tür über 8 kg feuchter Baumwolle geschlossen, Gerät eingeschaltet, Wärmepumpe noch aus."),
     "bones": IN_USE, "ground": "authored", "contacts": ["blanket-floor"] + FEET, "thermal": binding(0, 1, True), "loads": [LAUNDRY]},
    {"id": "finished-key", "key": True, "title": t("Finished and unloaded", "Fertig und entladen"),
     "description": t("Key of the after-cycle care: door shut, drum empty, heat pump off.", "Schlüssel der Pflege nach dem Programm: Tür zu, Trommel leer, Wärmepumpe aus."),
     "bones": IN_USE, "ground": "authored", "contacts": ["blanket-floor"] + FEET, "thermal": binding(0, 1, True), "loads": []},
]

USE_CONTACTS = ["blanket-floor"] + FEET


def spans(anchors, a, b):
    """⏱️ Contact spans of anchors."""
    return [{"anchor": x, "fromS": a, "toS": b} for x in anchors]


CLIPS = [
    {"id": "carry", "title": t("Carry out of the store", "Aus dem Kaufhaus tragen"),
     "description": t("The two carriers take hold under the base, lift the dryer off the blanket and tilt it back 8 degrees as they set off; the blanket stays behind.", "Die zwei Träger fassen unter den Boden, heben den Trockner von der Decke und neigen ihn beim Losgehen um 8 Grad nach hinten; die Decke bleibt liegen."),
     "durationS": 6.0, "fps": 25, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 1.0, "pose": "stored", "ease": "smooth"}, {"t": 3.0, "pose": "carried", "ease": "smooth", "bones": {"unit": {"angleDeg": 0}}},
              {"t": 5.0, "pose": "carried", "ease": "smooth"}, {"t": 6.0, "pose": "carried", "ease": "smooth"}],
     "contacts": spans(["blanket-floor"], 0, 6.0) + spans(GRIPS, 1.0, 6.0),
     "phases": [{"id": "take-hold", "fromS": 0, "toS": 1.0, "title": t("Taking hold", "Anfassen")}, {"id": "lift", "fromS": 1.0, "toS": 3.0, "title": t("Lifting off the blanket", "Von der Decke heben")},
                {"id": "tilt", "fromS": 3.0, "toS": 5.0, "title": t("Tilting back", "Nach hinten neigen")}, {"id": "carry", "fromS": 5.0, "toS": 6.0, "title": t("Carrying", "Tragen")}],
     "events": [{"t": 1.0, "kind": "contact-off", "title": t("Feet leave the blanket", "Füße verlassen die Decke"), "target": "unit"}, {"t": 5.0, "kind": "motion", "title": t("Dryer tilted and carried", "Trockner geneigt und getragen"), "target": "unit"}],
     "tracks": []},
    {"id": "drying-cycle", "title": t("Drying cycle with warm drum", "Trockenprogramm mit warmer Trommel"),
     "description": t("Cotton cupboard dry, 8 kg, 160 minutes: the electronics start, the drum turns and the compressor starts after one minute, drum, door and cabinet warm up over the first 25 minutes and hold while the heat pump condenses about 4.2 litres of water, the last 20 minutes cool the laundry with the drum and fans only.",
                      "Baumwolle schranktrocken, 8 kg, 160 Minuten: die Elektronik startet, die Trommel dreht sich und nach einer Minute läuft der Verdichter an, Trommel, Tür und Gehäuse erwärmen sich in den ersten 25 Minuten und halten, während die Wärmepumpe etwa 4,2 Liter Wasser auskondensiert, die letzten 20 Minuten kühlen die Wäsche nur mit Trommel und Gebläse ab."),
     "durationS": 9600, "fps": 0.02, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "loaded-key", "ease": "step"}, {"t": 9600, "pose": "loaded-key", "ease": "step"}],
     "contacts": spans(USE_CONTACTS, 0, 9600),
     "phases": [{"id": "start", "fromS": 0, "toS": 60, "title": t("Start", "Start")}, {"id": "heat-up", "fromS": 60, "toS": 1500, "title": t("Heat-up", "Aufheizen")},
                {"id": "drying", "fromS": 1500, "toS": 8400, "title": t("Drying", "Trocknen")}, {"id": "cool-down", "fromS": 8400, "toS": 9600, "title": t("Cool-down", "Abkühlen")}],
     "events": [{"t": 60, "kind": "operating", "title": t("Compressor starts", "Verdichter startet"), "target": "heat-pump"}, {"t": 8400, "kind": "operating", "title": t("Compressor stops, cool-down", "Verdichter stoppt, Abkühlen"), "target": "heat-pump"},
                {"t": 9600, "kind": "operating", "title": t("Cycle ends", "Programmende"), "target": "heat-pump"}],
     "tracks": [{"target": "operating:controls", "ease": "step", "keys": [{"t": 0, "v": 1}, {"t": 9600, "v": 1}]},
                {"target": "operating:heat-pump", "ease": "linear", "keys": [{"t": 0, "v": 0}, {"t": 60, "v": 0}, {"t": 1500, "v": 1}, {"t": 8400, "v": 1}, {"t": 8460, "v": 0.25}, {"t": 9540, "v": 0.25}, {"t": 9600, "v": 0}]},
                {"target": "emitter:plinth-vent", "ease": "linear", "keys": [{"t": 0, "v": 0}, {"t": 60, "v": 0}, {"t": 300, "v": 1}, {"t": 8400, "v": 1}, {"t": 8460, "v": 0}, {"t": 9600, "v": 0}]}]},
    {"id": "empty-tank", "title": t("Empty the condensate tank", "Kondenswasserbehälter leeren"),
     "description": t("After the cycle the tank drawer is pulled 0.28 m out, held while the condensate is poured away and pushed back in.", "Nach dem Programm wird die Behälterschublade 0,28 m herausgezogen, gehalten, während das Kondensat ausgegossen wird, und wieder eingeschoben."),
     "durationS": 8.0, "fps": 25, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "finished-key", "ease": "smooth"}, {"t": 2.5, "pose": "tank-out", "ease": "smooth"}, {"t": 5.5, "pose": "tank-out", "ease": "smooth"}, {"t": 8.0, "pose": "finished-key", "ease": "smooth"}],
     "contacts": spans(USE_CONTACTS, 0, 8.0),
     "phases": [{"id": "pull", "fromS": 0, "toS": 2.5, "title": t("Pulling out", "Herausziehen")}, {"id": "empty", "fromS": 2.5, "toS": 5.5, "title": t("Emptying", "Ausgießen")}, {"id": "push", "fromS": 5.5, "toS": 8.0, "title": t("Pushing back", "Einschieben")}],
     "events": [{"t": 2.5, "kind": "motion", "title": t("Tank at its stop", "Behälter am Anschlag"), "target": "tank"}, {"t": 8.0, "kind": "motion", "title": t("Tank clicks in", "Behälter rastet ein"), "target": "tank"}],
     "tracks": []},
    {"id": "clean-filter", "title": t("Open the door and clean the fluff filter", "Tür öffnen und Flusensieb reinigen"),
     "description": t("The door swings open by 100 degrees, the fluff filter is pulled up out of the aperture, brushed off, put back and the door is shut again.", "Die Tür schwenkt um 100 Grad auf, das Flusensieb wird aus der Einfüllöffnung gezogen, abgestreift, wieder eingesetzt und die Tür wieder geschlossen."),
     "durationS": 10.0, "fps": 25, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "finished-key", "ease": "smooth"}, {"t": 2.0, "pose": "door-open", "ease": "smooth", "bones": {"lint-filter": {"dy": 0}}}, {"t": 4.0, "pose": "door-open", "ease": "smooth"},
              {"t": 6.0, "pose": "door-open", "ease": "smooth"}, {"t": 8.0, "pose": "door-open", "ease": "smooth", "bones": {"lint-filter": {"dy": 0}}}, {"t": 10.0, "pose": "finished-key", "ease": "smooth"}],
     "contacts": spans(USE_CONTACTS, 0, 10.0),
     "phases": [{"id": "open", "fromS": 0, "toS": 2.0, "title": t("Opening the door", "Tür öffnen")}, {"id": "pull", "fromS": 2.0, "toS": 4.0, "title": t("Pulling the filter", "Sieb ziehen")},
                {"id": "brush", "fromS": 4.0, "toS": 6.0, "title": t("Brushing off the lint", "Flusen abstreifen")}, {"id": "close", "fromS": 6.0, "toS": 10.0, "title": t("Refitting and closing", "Einsetzen und schließen")}],
     "events": [{"t": 2.0, "kind": "motion", "title": t("Door fully open", "Tür ganz offen"), "target": "door"}, {"t": 10.0, "kind": "motion", "title": t("Door latched", "Tür eingerastet"), "target": "door"}],
     "tracks": []},
]


def anchors():
    """📍️ Blanket on the floor, four screw feet, two grips, the plinth vent."""
    A = [{"id": "blanket-floor", "bone": "blanket", "x": 0.45, "y": 0, "z": 0, "kind": "contact", "title": t("Moving blanket on the floor", "Umzugsdecke auf dem Boden"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.9, "footprintWidthM": 0.75, "frictionCoefficient": 0.5}]
    names = {"foot-front-right": ("Front right screw foot", "Schraubfuß vorne rechts"), "foot-front-left": ("Front left screw foot", "Schraubfuß vorne links"), "foot-back-right": ("Back right screw foot", "Schraubfuß hinten rechts"), "foot-back-left": ("Back left screw foot", "Schraubfuß hinten links")}
    for nm, (x, z) in zip(FEET, [(0.22, -0.25), (0.22, 0.25), (-0.27, -0.25), (-0.27, 0.25)]):
        A.append({"id": nm, "bone": "unit", "x": round(x + 0.32, 4), "y": 0, "z": z, "kind": "contact", "title": t(*names[nm]), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.04, "footprintWidthM": 0.04, "frictionCoefficient": 0.6})
    A.append({"id": "grip-front", "bone": "unit", "x": 0.595, "y": 0.015, "z": 0, "kind": "support", "title": t("Front carrier's hands under the front edge of the base", "Hände des vorderen Trägers unter der Vorderkante des Bodens"), "support": "pin", "footprintLengthM": 0.08, "footprintWidthM": 0.5})
    A.append({"id": "grip-back", "bone": "unit", "x": 0.0, "y": 0.015, "z": 0, "kind": "support", "title": t("Back carrier's hands under the back edge of the base", "Hände des hinteren Trägers unter der Hinterkante des Bodens"), "support": "pin", "footprintLengthM": 0.08, "footprintWidthM": 0.5})
    A.append({"id": "vent-outlet", "bone": "unit", "x": 0.598, "y": 0.075, "z": 0, "kind": "emitter", "title": t("Grille of the plinth flap", "Lüftungsgitter der Sockelklappe")})
    return A


def rig():
    """🦴️ Ground, blanket that stays behind, lift and tilt of the dryer, porthole door on its vertical hinge, tank drawer, fluff filter."""
    return {"kind": "mechanical", "root": "ground", "bones": [
        {"id": "ground", "parent": None, "title": t("Floor reference", "Bodenbezug"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"}},
        {"id": "blanket", "parent": "ground", "title": t("Folded moving blanket (stays behind)", "Gefaltete Umzugsdecke (bleibt zurück)"), "x": -0.45, "y": 0, "angleDeg": 0, "lengthM": 0.9, "joint": {"type": "prismatic", "axisDeg": 0, "minM": -1.2, "maxM": 0},
         "mass": {"kg": BLANKET_KG, "comX": BLANKET_COM[0], "comY": BLANKET_COM[1], "distribution": "point", "source": "authored-estimate", "reference": "moving-blanket-catalogue"}},
        {"id": "lift", "parent": "ground", "title": t("Lift of the dryer off the blanket", "Anheben des Trockners von der Decke"), "x": -0.32, "y": Y0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -Y0, "maxM": 0.5}},
        {"id": "unit", "parent": "lift", "title": t("Dryer cabinet, tilt about the back edge of the base", "Trocknergehäuse, Neigung um die Hinterkante des Bodens"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0.64, "joint": {"type": "revolute", "minDeg": -2, "maxDeg": 15},
         "mass": {"kg": UNIT_KG, "comX": UNIT_COM[0], "comY": UNIT_COM[1], "distribution": "point", "source": "authored-estimate", "reference": "catalogue-typical"},
         "section": {"areaM2": 0.001264, "secondMomentM4": 6.57e-5, "extremeFibreM": 0.395, "material": "enamel-steel", "title": t("Two 0.8 mm steel side panels 790 mm high as webs", "Zwei 0,8-mm-Stahlseitenbleche von 790 mm Höhe als Stege")}},
        {"id": "door", "parent": "unit", "title": t("Porthole door on its right-hand vertical hinge", "Bullaugentür an ihrem rechten senkrechten Scharnier"), "x": 0.595, "y": 0.455, "z": -0.235, "angleDeg": 90, "lengthM": 0.23,
         "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 0, "twistMinDeg": -110, "twistMaxDeg": 0},
         "mass": {"kg": DOOR_KG, "comX": DOOR_COM[0], "comY": DOOR_COM[1], "distribution": "point", "source": "authored-estimate", "reference": "catalogue-typical"}},
        {"id": "tank", "parent": "unit", "title": t("Condensate tank drawer", "Kondenswasserbehälter-Schublade"), "x": 0.27, "y": 0.775, "z": 0.2, "angleDeg": 0, "lengthM": 0.328, "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 0.3},
         "mass": {"kg": TANK_KG, "comX": TANK_COM[0], "comY": TANK_COM[1], "distribution": "point", "source": "authored-estimate", "reference": "catalogue-typical"}},
        {"id": "lint-filter", "parent": "unit", "title": t("Fluff filter in the door aperture (its 80 g counted with the cabinet)", "Flusensieb in der Einfüllöffnung (seine 80 g beim Gehäuse gezählt)"), "x": 0.562, "y": 0.287, "angleDeg": 0, "lengthM": 0.02, "joint": {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.15}},
    ]}


REFS = [
    {"id": "catalogue-typical", "citation": "Data sheets of freestanding 8 kg heat-pump condenser tumble dryers of German mid-range brands, 2012 to 2020 generation, taken as a type: 598 x 636 to 652 x 842 to 850 mm (W x D x H, door included, feet screwed in), 45 to 56 kg, 112 l stainless drum, porthole door of about 460 mm with the fluff filter in the aperture, condensate tank of 4.5 to 5 l in the top left of the fascia, second filter in front of the heat exchanger behind the plinth flap, connected load 0.8 to 1.1 kW, 1.5 to 1.8 kWh and 150 to 170 minutes for 8 kg cotton cupboard dry; values rounded to the class, not those of one product."},
    {"id": "en-61121", "citation": "EN 61121:2013 (IEC 61121:2012) Tumble dryers for household use, methods for measuring the performance: rated capacity, cotton cupboard-dry programme, initial moisture content 60 % (8 kg load carries 4.8 kg water before drying, about 53 % after a 1400 rpm spin)."},
    {"id": "eu-392-2012", "citation": "Commission Delegated Regulation (EU) No 392/2012 energy labelling of household tumble driers (energy class from the weighted annual energy, condensation efficiency class)."},
    {"id": "eu-932-2012", "citation": "Commission Regulation (EU) No 932/2012 ecodesign requirements for household tumble driers."},
    {"id": "en-60335-2-11", "citation": "EN 60335-2-11:2010 Household and similar electrical appliances, safety, particular requirements for tumble dryers (surface temperature limits of accessible parts)."},
    {"id": "din-18011", "citation": "DIN 18011:2006 Stellflächen, Abstände und Bewegungsflächen im Wohnungsbau: Waschmaschine und Wäschetrockner je 60 x 60 cm Stellfläche, 90 cm Bewegungsfläche davor."},
    {"id": "en-10130", "citation": "EN 10130:2006 Cold rolled low carbon steel flat products for cold forming (DC01, 0.6 to 0.8 mm cabinet sheet, density 7850 kg/m3, E 210 GPa, yield 140 to 280 MPa)."},
    {"id": "en-iso-10456", "citation": "EN ISO 10456:2007 Building materials and products, hygrothermal properties (polypropylene 0.22 W/mK, 1900 J/kgK; ABS 0.17 W/mK; steel 50 W/mK, 450 J/kgK; textile felts 0.04 to 0.05 W/mK)."},
    {"id": "incropera-2018", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (emissivities of painted metal, plastics, glass and stainless steel; solar absorptance of white and yellowed paints)."},
    {"id": "dguv-208-033", "citation": "DGUV Information 208-033 (2018) Muskel-Skelett-Belastungen, Lastenhandhabung: loads above 25 kg carried by two persons, grip under the lower edge, short distances."},
    {"id": "moving-blanket-catalogue", "citation": "Removal trade moving blankets 1.8 x 1.5 m of quilted recycled cotton and polyester felt, about 0.45 kg/m2 and 4 mm per layer (1.2 kg each), folded four times under appliances."},
    {"id": "din-1356-1", "citation": "DIN 1356-1:1995 Building drawings, part 1: types of drawings, contents and basic principles of presentation (line groups)."},
    {"id": "din-iso-128-50", "citation": "DIN ISO 128-50:2002 Technical drawings, basic conventions for areas on cuts and sections (textile quilting hatch)."},
]

PENS = {50: {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": 0.13, "ground": 0.5},
        100: {"body": 0.25, "detail": 0.13, "texture": 0.13, "glazing": 0.13, "ground": 0.35},
        200: {"body": 0.25, "detail": 0.13, "texture": 0.13, "glazing": 0.13, "ground": 0.25}}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350)}
HATCHES = {50: [{"id": "quilt", "title": t("Quilted blanket, stitching lines", "Gesteppte Decke, Steppnähte"), "pattern": "cross", "angleDeg": 45, "spacingMm": 1.2, "penMm": 0.1}], 100: [], 200: []}

DECISIONS = {
    50: {
        "silhouette": t("Side elevation 18 x 17.3 mm on paper: the yellowed steel side panel with a dent, scuffs and the outline of a peeled-off label, the door ring standing 35 mm proud, the programme knob, the screw feet, the cable coil taped to the back and the folded blanket below. Front elevation 15 x 17.3 mm: porthole door with dark glass and lint film, the tape across the free edge of the door, the faded fascia with knob, display and buttons, the tank drawer, the plinth flap with grille and kick marks. Plan 18 x 15 mm: top cover with price sticker, scratches, a cup ring and adhesive residue, the door and knob in front, the cable coil behind and the quilted blanket around.",
                            "Seitenansicht 18 x 17,3 mm auf Papier: das vergilbte Stahlseitenblech mit Delle, Schrammen und dem Umriss eines abgezogenen Etiketts, der 35 mm vorstehende Türring, der Programmwähler, die Schraubfüße, die an die Rückwand geklebte Kabelrolle und unten die gefaltete Decke. Vorderansicht 15 x 17,3 mm: Bullaugentür mit dunklem Glas und Flusenfilm, das Klebeband über der freien Türkante, die verblasste Blende mit Wähler, Display und Tasten, die Behälterschublade, die Sockelklappe mit Gitter und Trittspuren. Grundriss 18 x 15 mm: Abdeckplatte mit Preisaufkleber, Kratzern, einem Tassenrand und Kleberesten, davor Tür und Wähler, dahinter die Kabelrolle und ringsum die gesteppte Decke."),
        "retained": t("Blanket, four screw feet with pad and stem, base, plinth flap, both side panels, back panel and front panel with the 360 mm aperture as 0.8 mm sheets, top cover, fascia, knob, display, tank front and tank body, door ring, domed glass bowl, hinge block, fluff filter, cable coil.",
                          "Decke, vier Schraubfüße mit Teller und Schaft, Boden, Sockelklappe, beide Seitenbleche, Rückwand und Frontblech mit der 360-mm-Öffnung als 0,8-mm-Bleche, Abdeckplatte, Blende, Wähler, Display, Behälterfront und Behälter, Türring, gewölbte Glasschale, Scharnierblock, Flusensieb, Kabelrolle."),
        "merged": t("Drum, heat-pump module and motor are hidden thermal solids; the cable is one coil; the hinge pins are in the hinge block.", "Trommel, Wärmepumpenmodul und Motor sind verdeckte Thermokörper; das Kabel ist eine Rolle; die Scharnierstifte stecken im Scharnierblock."),
        "omitted": t("Screws, door latch, seals, the second filter behind the plinth flap and the inner wiring.", "Schrauben, Türschloss, Dichtungen, das zweite Sieb hinter der Sockelklappe und die innere Verdrahtung."),
        "thinElements": t("The 0.8 mm steel panels and the display window are declared sheets; the front view draws the side panels as the frame of the front.", "Die 0,8-mm-Stahlbleche und das Displayfenster sind erklärte Bleche; die Vorderansicht zeichnet die Seitenbleche als Rahmen der Front."),
        "hatching": t("Cross quilting on the blanket in all three views (DIN ISO 128-50).", "Kreuzsteppung auf der Decke in allen drei Ansichten (DIN ISO 128-50)."),
        "styles": t("Outline 0.35 mm, details 0.18 mm, texture 0.13 mm; the yellowed cabinet and the white top light, the faded fascia and the door ring mid light, the glass, feet and display dark, the blanket mid grey.", "Kontur 0,35 mm, Details 0,18 mm, Textur 0,13 mm; vergilbtes Gehäuse und weiße Abdeckplatte hell, verblasste Blende und Türring mittelhell, Glas, Füße und Display dunkel, die Decke mittelgrau."),
        "overlays": t("Every part is its own cell so that the thermal view colours cabinet, door, fascia, tank, plinth, cable and blanket apart.", "Jedes Teil ist eine eigene Zelle, damit die Thermoansicht Gehäuse, Tür, Blende, Behälter, Sockel, Kabel und Decke getrennt einfärbt."),
    },
    100: {
        "silhouette": t("Side elevation 9 x 8.7 mm: side panel, door ring, knob, feet, cable coil and blanket. Front 7.5 x 8.7 mm: door with glass and tape, fascia with knob, tank drawer, plinth flap with two grille slots. Plan 9 x 7.5 mm: top cover with sticker and two scratches, door, knob, cable coil and blanket.",
                            "Seitenansicht 9 x 8,7 mm: Seitenblech, Türring, Wähler, Füße, Kabelrolle und Decke. Vorderansicht 7,5 x 8,7 mm: Tür mit Glas und Klebeband, Blende mit Wähler, Behälterschublade, Sockelklappe mit zwei Gitterschlitzen. Grundriss 9 x 7,5 mm: Abdeckplatte mit Aufkleber und zwei Kratzern, Tür, Wähler, Kabelrolle und Decke."),
        "retained": t("Blanket, feet as plain cylinders, base, plinth flap thickened to 15 mm, sheet panels, top cover, fascia, knob as a disc, tank front and body, door ring and glass, hinge block, fluff filter, cable coil.", "Decke, Füße als einfache Zylinder, Boden, Sockelklappe auf 15 mm verstärkt, Blechwände, Abdeckplatte, Blende, Wähler als Scheibe, Behälterfront und Behälter, Türring und Glas, Scharnierblock, Flusensieb, Kabelrolle."),
        "merged": t("Ring profile without its rounded rim, glass bowl as a cone, motor into the heat-exchanger block.", "Ringprofil ohne gerundeten Rand, Glasschale als Kegel, Motor in den Wärmeübertragerblock."),
        "omitted": t("Display, buttons, knob ticks, door grip recess, dent, scuffs, label residue, cup ring and lint film.", "Display, Tasten, Wählerstriche, Griffmulde der Tür, Delle, Schrammen, Etikettreste, Tassenrand und Flusenfilm."),
        "thinElements": t("The steel panels stay declared sheets.", "Die Stahlbleche bleiben erklärte Bleche."),
        "hatching": t("No hatch at this scale.", "Keine Schraffur in diesem Maßstab."),
        "styles": t("Outline 0.25 mm, details 0.13 mm.", "Kontur 0,25 mm, Details 0,13 mm."),
        "overlays": t("One cell per part as at 1:50.", "Eine Zelle je Teil wie bei 1:50."),
    },
    200: {
        "silhouette": t("Side elevation 4.5 x 4.3 mm: the cabinet block on its 40 mm base, the door ring, the cable disc and the blanket strip. Front 3.8 x 4.3 mm: door ring with glass, fascia and tank drawer, plinth flap. Plan 4.5 x 3.8 mm: top cover, door, cable disc and blanket.",
                            "Seitenansicht 4,5 x 4,3 mm: der Gehäuseblock auf seinem 40-mm-Boden, der Türring, die Kabelscheibe und der Deckenstreifen. Vorderansicht 3,8 x 4,3 mm: Türring mit Glas, Blende und Behälterschublade, Sockelklappe. Grundriss 4,5 x 3,8 mm: Abdeckplatte, Tür, Kabelscheibe und Decke."),
        "retained": t("Blanket, base with the feet in it, plinth flap of 30 mm, sheet panels, top cover thickened to 30 mm, fascia, tank as one block, door ring and glass, cable disc.", "Decke, Boden mit den Füßen darin, Sockelklappe von 30 mm, Blechwände, Abdeckplatte auf 30 mm verstärkt, Blende, Behälter als ein Block, Türring und Glas, Kabelscheibe."),
        "merged": t("Feet into the base, tank front and body into one block, hinge into the ring, cable coil into a disc, the heat-pump module into one block.", "Füße in den Boden, Behälterfront und Behälter in einen Block, Scharnier in den Ring, Kabelrolle in eine Scheibe, das Wärmepumpenmodul in einen Block."),
        "omitted": t("Knob (22 mm), fluff filter (20 mm), all detail strokes.", "Wähler (22 mm), Flusensieb (20 mm), alle Detailstriche."),
        "thinElements": t("Steel panels and the 16 mm blanket are declared sheets; the top cover is drawn 30 mm instead of 20 mm.", "Stahlbleche und die 16-mm-Decke sind erklärte Bleche; die Abdeckplatte ist mit 30 statt 20 mm gezeichnet."),
        "hatching": t("No hatch.", "Keine Schraffur."),
        "styles": t("Outline 0.25 mm only.", "Nur Kontur 0,25 mm."),
        "overlays": t("One cell per mass.", "Eine Zelle je Masse."),
    },
}

VOL_DECISIONS = {
    50: [t("Body: every part as it is: the blanket, four screw feet revolved with pad and stem, the polypropylene base and plinth flap, the cabinet as 0.8 mm sheets with the 360 mm aperture cut into the front panel, top cover, fascia, knob, display, the tank drawer front and body, the door ring revolved with its rounded rim, the domed glass bowl, the hinge block, the fluff filter and the taped cable coil.",
           "Körper: jedes Teil, wie es ist: die Decke, vier gedrehte Schraubfüße mit Teller und Schaft, Polypropylen-Boden und Sockelklappe, das Gehäuse als 0,8-mm-Bleche mit der 360-mm-Öffnung im Frontblech, Abdeckplatte, Blende, Wähler, Display, Behälterfront und Behälter, der gedrehte Türring mit gerundetem Rand, die gewölbte Glasschale, der Scharnierblock, das Flusensieb und die geklebte Kabelrolle."),
         t("Thermal: every body solid is a thermal solid of its region; drum and heat-pump module are hidden solids in the closed cabinet; the plinth vent is an outlet patch on the flap. Structure: the side panels and the base are the members of the cabinet, the feet its joints on the blanket, the hinge block joins door and side panel, a rail joins tank and left panel, the blanket pad is the floor support.",
           "Thermik: jeder Körper ist ein Thermokörper seiner Region; Trommel und Wärmepumpenmodul sind verdeckte Körper im geschlossenen Gehäuse; das Sockelgitter ist ein Austrittsfeld auf der Klappe. Tragwerk: Seitenbleche und Boden sind die Glieder des Gehäuses, die Füße seine Knoten auf der Decke, der Scharnierblock verbindet Tür und Seitenblech, eine Schiene Behälter und linkes Blech, das Deckenpolster ist das Bodenauflager.")],
    100: [t("Body: a new model for the 15 mm feature: feet as plain cylinders, the plinth flap thickened to 15 mm, the knob a disc, the door ring without its rim radius, the glass bowl a cone; display and knob ticks are left out.",
            "Körper: ein neues Modell für das Merkmal von 15 mm: Füße als einfache Zylinder, die Sockelklappe auf 15 mm verstärkt, der Wähler eine Scheibe, der Türring ohne Randradius, die Glasschale ein Kegel; Display und Wählerstriche entfallen."),
          t("Thermal and structure as at 1:50 on the solids of this scale; the drum is a plain cylinder and the motor is merged into the heat-exchanger block.", "Thermik und Tragwerk wie bei 1:50 auf den Körpern dieses Maßstabs; die Trommel ist ein einfacher Zylinder und der Motor ist in den Wärmeübertragerblock einbezogen.")],
    200: [t("Body: a new abstraction for the 30 mm feature: the feet are merged into a 40 mm base, the tank is one block, the hinge is in the ring, the cable coil a 30 mm disc; knob and fluff filter are below the feature and left out; the top cover is thickened from 20 to 30 mm so it prints to 1:350; panels and blanket are declared sheets.",
            "Körper: eine neue Abstraktion für das Merkmal von 30 mm: die Füße sind in einen 40-mm-Boden einbezogen, der Behälter ist ein Block, das Scharnier steckt im Ring, die Kabelrolle ist eine 30-mm-Scheibe; Wähler und Flusensieb liegen unter dem Merkmal und entfallen; die Abdeckplatte ist von 20 auf 30 mm verstärkt, damit sie bis 1:350 druckt; Bleche und Decke sind erklärte Bleche."),
          t("Structure: side panels and base are the cabinet members with joint pads at the feet; the heat-pump module is one hidden thermal block.", "Tragwerk: Seitenbleche und Boden sind die Gehäuseglieder mit Knotenklötzen an den Füßen; das Wärmepumpenmodul ist ein verdeckter Thermoblock.")],
}

UNAVAILABLE_500 = {"status": "unavailable", "denominator": 500, "cause": "below-printable-feature",
                   "reason": t("At 1:500 the stored dryer is 1.8 mm by 1.7 mm on paper, under the 3 mm a recognisable drawing needs, and its fascia (110 mm) and door ring rim (35 mm) are 0.22 and 0.07 mm, at or below the 0.2 mm office-laser minimum; draw it at 1:200 or larger.",
                               "Bei 1:500 ist der gelagerte Trockner 1,8 mm mal 1,7 mm groß auf Papier, unter den 3 mm, die eine erkennbare Zeichnung braucht, und seine Blende (110 mm) und der Türringrand (35 mm) messen 0,22 und 0,07 mm, an oder unter dem Mindestmerkmal von 0,2 mm des Büro-Laserdrucks; bei 1:200 oder größer zeichnen."),
                   "limitingFeature": t("Fascia of 110 mm", "Blende von 110 mm"), "limitingSizeM": 0.11,
                   "alternatives": [{"kind": "scale", "denominator": 200}, {"kind": "scale", "denominator": 100}]}


def asset_shell():
    """🧺️ Every member of the asset except representations and volumes."""
    dryer = UNIT_KG + DOOR_KG + TANK_KG
    return {
        "id": "used-tumble-dryer", "category": "appliances",
        "title": t("Used Heat-Pump Tumble Dryer", "Gebrauchter Wärmepumpentrockner"),
        "summary": t("A second-hand 8 kg heat-pump condenser tumble dryer of 598 x 640 x 850 mm and 49.6 kg as a second-hand department store offers it: yellowed cabinet with a dent and scuffs, sun-faded fascia, scratched top with a price sticker, door taped shut, cable coiled and taped to the back, standing on a folded moving blanket; carried by two people under its base, back in a utility room it dries at about 0.6 kW mean of its 1 kW connected load with a warm drum, empties its condensate tank and gets its fluff filter cleaned.",
                     "Ein gebrauchter 8-kg-Wärmepumpen-Kondenstrockner von 598 x 640 x 850 mm und 49,6 kg, wie ihn ein Gebrauchtwarenkaufhaus anbietet: vergilbtes Gehäuse mit Delle und Schrammen, sonnengebleichte Blende, zerkratzte Abdeckplatte mit Preisaufkleber, Tür zugeklebt, Kabel aufgerollt und an die Rückwand geklebt, auf einer gefalteten Umzugsdecke stehend; von zwei Personen unter dem Boden getragen, trocknet er wieder im Hauswirtschaftsraum mit etwa 0,6 kW Mittel seines Anschlusswerts von 1 kW bei warmer Trommel, leert seinen Kondenswasserbehälter und bekommt sein Flusensieb gereinigt."),
        "tags": ["used", "household", "second-hand", "utility", "appliance", "tumble-dryer", "heat-pump", "laundry", "electric", "internal-gain", "steel", "polypropylene", "moving-blanket"],
        "provenance": {"author": "Semio staffage household authors (used household objects wave, author 23)", "method": "handcrafted", "created": "2026-11-22", "references": REFS,
                       "notes": t("Dryer 598 mm wide, 850 mm high on its feet, 640 mm deep with the door, %.2f kg (cabinet %.1f, door %.1f, tank %.2f); moving blanket 0.90 x 0.75 x 0.016 m, %.2f kg; dimensions, masses and ratings are class-typical authored estimates, nothing is measured." % (dryer, UNIT_KG, DOOR_KG, TANK_KG, BLANKET_KG),
                                  "Trockner 598 mm breit, 850 mm hoch auf seinen Füßen, 640 mm tief mit Tür, %s kg (Gehäuse %s, Tür %s, Behälter %s); Umzugsdecke 0,90 x 0,75 x 0,016 m, %s kg; Abmessungen, Massen und Leistungen sind klassentypische Schätzwerte, nichts ist gemessen." % tuple(("%.2f" % v if i in (0, 3, 4) else "%.1f" % v).replace(".", ",") for i, v in enumerate((dryer, UNIT_KG, DOOR_KG, TANK_KG, BLANKET_KG))))},
        "dimensions": {"heightM": round(Y0 + 0.85, 4), "widthM": 0.9, "depthM": 0.75, "massKg": round(dryer + BLANKET_KG, 3)},
        "materials": MATERIALS, "rig": rig(), "anchors": anchors(), "symbols": [], "poses": POSES, "clips": CLIPS,
        "thermal": {"environment": {"airTempC": 18, "meanRadiantTempC": 18, "relativeHumidity": 0.55, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 0, "pressurePa": 101325},
                    "passengers": [], "operating": OPERATING, "regions": regions(),
                    "emitters": [{"id": "plinth-vent", "kind": "fan", "title": t("Warm compressor cooling air out of the plinth grille", "Warme Verdichter-Kühlluft aus dem Sockelgitter"), "anchor": "vent-outlet", "directionDeg": 0, "diameterM": 0.1, "operatingKey": "heat-pump", "volumeFlowM3S": 0.02, "temperatureRiseK": 8, "source": "authored-estimate", "reference": "incropera-2018"}],
                    "assumptions": [t("A utility room or store hall at 18 degC: stored every surface is at room temperature; drying, the closed process-air loop runs at about 55 degC in the drum, the cabinet releases about 140 W, the drum 120 W into the cabinet, the door 25 W, the heat-pump module 180 W through the plinth grille, the electronics 4 W; the rest of the mean 0.6 kW leaves with the 4.2 litres of warm condensate and the warm laundry.",
                                      "Ein Hauswirtschaftsraum oder eine Kaufhaushalle bei 18 °C: gelagert haben alle Flächen Raumtemperatur; beim Trocknen läuft der geschlossene Prozessluftkreis mit etwa 55 °C in der Trommel, das Gehäuse gibt etwa 140 W ab, die Trommel 120 W in das Gehäuse, die Tür 25 W, das Wärmepumpenmodul 180 W durch das Sockelgitter, die Elektronik 4 W; der Rest der mittleren 0,6 kW geht mit den 4,2 Litern warmen Kondensats und der warmen Wäsche."),
                                    t("The nameplate states the connected electrical load of the whole machine once, 1 kW on the heat-pump module (compressor, drum motor, fans and electronics); a heat pump counts its electrical input, not the heat it moves. Aged surfaces: the yellowed steel absorbs 0.32 of the sun instead of 0.25, the faded fascia 0.42, the yellowed door ring 0.48 (authored estimates after Incropera).",
                                      "Das Typenschild nennt den elektrischen Anschlusswert der ganzen Maschine einmal, 1 kW am Wärmepumpenmodul (Verdichter, Trommelmotor, Gebläse und Elektronik); eine Wärmepumpe zählt ihre elektrische Aufnahme, nicht die bewegte Wärme. Gealterte Flächen: das vergilbte Blech absorbiert 0,32 der Sonne statt 0,25, die verblasste Blende 0,42, der vergilbte Türring 0,48 (Schätzwerte nach Incropera).")]},
        "structure": {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": ["blanket-floor"],
                      "assumptions": [t("Stored, the 49.6 kg dryer stands with its four screw feet on the folded blanket, which lies on the floor; the 0.8 mm steel side panels (DC01, EN 10130) carry the top, the fascia, the drum bearings and the door hinge down to the polypropylene base, which holds the heat-pump module and sits on the feet.",
                                        "Gelagert steht der 49,6 kg schwere Trockner mit vier Schraubfüßen auf der gefalteten Decke, die auf dem Boden liegt; die 0,8-mm-Stahlseitenbleche (DC01, EN 10130) tragen Abdeckplatte, Blende, Trommellager und Türscharnier hinunter zum Polypropylen-Boden, der das Wärmepumpenmodul hält und auf den Füßen steht."),
                                      t("Carried, two people hold the base under its front and back edges (DGUV 208-033: above 25 kg two persons), each about 24 kg; in use the four feet carry the dryer and 12.2 kg of damp laundry. Only the blanket is a permanent support; poses name the feet or grips they use, and the solver treats the asset as one rigid assembly, so the blanket left on the floor shares the reactions with grips or feet in proportion to stiffness.",
                                        "Getragen halten zwei Personen den Boden unter Vorder- und Hinterkante (DGUV 208-033: über 25 kg zwei Personen), je etwa 24 kg; in Gebrauch tragen die vier Füße den Trockner und 12,2 kg feuchte Wäsche. Nur die Decke ist ein ständiges Auflager; die Posen nennen die Füße oder Griffe, die sie nutzen, und der Löser behandelt das Asset als eine starre Baugruppe, die am Boden zurückgelassene Decke teilt daher die Reaktionen mit Griffen oder Füßen nach Steifigkeit.")],
                      "clinical": False},
        "presentation": {"defaultPose": "stored", "defaultClip": "drying-cycle", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
