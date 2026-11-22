"""📜️ The physical scenario and every bilingual text of `used-accordion`: summary, provenance, materials, thermal model, poses, clips and the design decisions of each scale.

Loaded by `author-accordion.py`; the numbers are the reference table of `../../reports/used-accordion.md`.
"""


def t(en, de):
    """🌍️ A bilingual text."""
    return {"en": en, "de": de}


REFS = [
    {"id": "hornbostel-sachs-1914", "citation": "Hornbostel, E. M. von, Sachs, C. (1914). Systematik der Musikinstrumente. Ein Versuch. Zeitschrift für Ethnologie 46, 553-590 (class 412.132: free aerophones, sets of free reeds; the accordion as a free-reed instrument with bellows)."},
    {"id": "richter-1990", "citation": "Richter, G. (1990). Akkordeon. Handbuch für Musiker und Instrumentenbauer. Leipzig: VEB Fachbuchverlag (construction of the piano accordion: treble and bass casings, reed blocks, bellows of cardboard folds with cloth and metal corners, 41-key treble keyboard, 120-bass Stradella system, masses of the assemblies)."},
    {"id": "dunkel-1996", "citation": "Dunkel, M. (1996). Akkordeon. In: Finscher, L. (ed.), Die Musik in Geschichte und Gegenwart, 2nd ed., Sachteil 1, 61-82. Kassel: Bärenreiter (history and types of the piano accordion, Klingenthal production)."},
    {"id": "hohner-bravo-iii-120", "citation": "Hohner Musikinstrumente (2024). Bravo III 120, technical data: 41 piano keys, 120 basses, 48 x 18.5 cm, 9.2 kg. https://hohner.de/en/instruments/accordions/chromatic/bravo/bravo-iii-120"},
    {"id": "thomann-accordion-listings-2024", "citation": "Thomann GmbH (2024). Catalogue data of 41-key 120-bass piano accordions: Scandalli Air III 120 42.5 x 19 x 43 cm, 9.5 kg; Scandalli Super VI 48.5 x 19.5 x 44 cm, 11.9 kg; Weltmeister Supra 120 height 482 mm, 12.1 kg; accordion gig bag 120 bass approx. 55 x 23 x 45 cm. https://www.thomann.de"},
    {"id": "kirstein-accordion-case-120", "citation": "Musikhaus Kirstein (2024). Accordion hard case 120 bass, size II: outer dimensions 56.5 x 26.5 cm footprint, plywood with tolex, plush lining. https://www.kirstein.de/en/Accordion/Accordion-Accessories/Bags-Cases/Alpenklang-Accordion-Case-120-Bass-Size-II.html"},
    {"id": "incropera-2018", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (thermal properties and emissivities of metals, polymers, wood and leather; free convection)."},
    {"id": "iso-10456-2007", "citation": "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (plywood, cardboard, rubber, plastics)."},
    {"id": "iso-9886-2004", "citation": "ISO 9886:2004 Ergonomics - Evaluation of thermal strain by physiological measurements (skin temperatures of hands and trunk, 32 to 34 degC at rest indoors)."},
]

MATERIALS = [
    {"id": "case-tolex", "title": t("Poplar plywood 9 mm covered with black PVC tolex (aged, scuffed)", "Pappelsperrholz 9 mm mit schwarzem PVC-Bezug (gealtert, abgestoßen)"), "emissivity": 0.9, "solarAbsorptivity": 0.9, "densityKgM3": 560, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "case-plush", "title": t("Wine-red velour plush on foam, worn flat where the instrument rests", "Weinroter Velours auf Schaumstoff, an den Auflagestellen plattgedrückt"), "emissivity": 0.95, "solarAbsorptivity": 0.8, "densityKgM3": 120, "conductivityWmK": 0.045, "specificHeatJKgK": 1400, "source": "authored-estimate", "reference": "iso-10456-2007"},
    {"id": "corner-steel", "title": t("Nickel-plated steel of corners, latches and lugs, plating worn to the steel at the edges", "Vernickelter Stahl der Ecken, Schlösser und Ösen, an den Kanten bis auf den Stahl abgerieben"), "emissivity": 0.3, "solarAbsorptivity": 0.5, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "source": "authored-estimate", "reference": "incropera-2018"},
    {"id": "leather", "title": t("Vegetable-tanned leather of straps and handle, darkened by sweat", "Pflanzlich gegerbtes Leder der Riemen und des Griffs, vom Schweiß nachgedunkelt"), "emissivity": 0.95, "solarAbsorptivity": 0.75, "densityKgM3": 860, "conductivityWmK": 0.16, "specificHeatJKgK": 1500, "source": "literature", "reference": "incropera-2018"},
    {"id": "celluloid", "title": t("Red pearloid celluloid on the wooden casings, yellowed and crazed", "Rotes Perlmutt-Zelluloid auf den Holzgehäusen, vergilbt und mit Haarrissen"), "emissivity": 0.93, "solarAbsorptivity": 0.65, "densityKgM3": 1400, "conductivityWmK": 0.21, "specificHeatJKgK": 1500, "source": "authored-estimate", "reference": "incropera-2018"},
    {"id": "plated-steel", "title": t("Chrome-plated steel of the treble grille and strap hooks, dulled", "Verchromter Stahl des Diskantgitters und der Riemenhaken, stumpf geworden"), "emissivity": 0.15, "solarAbsorptivity": 0.4, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "source": "authored-estimate", "reference": "incropera-2018"},
    {"id": "key-plastic", "title": t("Key and button plastic, the white keys yellowed to ivory", "Tasten- und Knopfkunststoff, die weißen Tasten elfenbeinfarben vergilbt"), "emissivity": 0.92, "solarAbsorptivity": 0.4, "densityKgM3": 1050, "conductivityWmK": 0.18, "specificHeatJKgK": 1400, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "key-bed-wood", "title": t("Beech of the key bed, ebonised", "Buche des Tastenbetts, schwarz gebeizt"), "emissivity": 0.9, "solarAbsorptivity": 0.8, "densityKgM3": 700, "conductivityWmK": 0.17, "specificHeatJKgK": 1700, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "bellows-card", "title": t("Bellows of pressboard folds with black cloth and taped corners", "Balg aus Pressspanfalten mit schwarzem Gewebe und beklebten Ecken"), "emissivity": 0.9, "solarAbsorptivity": 0.9, "densityKgM3": 650, "conductivityWmK": 0.07, "specificHeatJKgK": 1400, "source": "authored-estimate", "reference": "iso-10456-2007"},
    {"id": "rubber", "title": t("Rubber of the feet", "Gummi der Füße"), "emissivity": 0.92, "solarAbsorptivity": 0.9, "densityKgM3": 1150, "conductivityWmK": 0.16, "specificHeatJKgK": 1800, "source": "literature", "reference": "iso-10456-2007"},
]


def region(rid, title, material, area, length, geometry, layer, extra=None):
    """🌡️ One passive region with one layer."""
    r = {"id": rid, "title": title, "kind": "surface", "model": "passive", "material": material, "areaM2": area, "characteristicLengthM": length, "geometry": geometry, "skyViewFactor": 0.5, "layers": [layer], "heatCapacityJM2K": layer["heatCapacityJM2K"], "source": "authored-estimate", "reference": "incropera-2018"}
    r.update(extra or {})
    return r


def layer(lid, title, kind, material, thickness, resistance, capacity):
    """🧶️ One layer."""
    return {"id": lid, "title": title, "kind": kind, "material": material, "thicknessM": thickness, "resistanceM2KW": resistance, "heatCapacityJM2K": capacity, "source": "authored-estimate", "reference": "iso-10456-2007"}


AREAS = {"case-shell": 1.484, "case-lid": 0.563, "case-lining": 1.141, "treble-casing": 0.221, "grille": 0.056, "keyboard": 0.159, "bellows": 0.401, "bass-casing": 0.265, "bass-buttons": 0.050, "straps": 0.154}

REGIONS = [
    region("case-shell", t("Tub of the hard case, handle and fittings", "Unterteil des Hartschalenkoffers, Griff und Beschläge"), "case-tolex", AREAS["case-shell"], 0.5, "vertical-plate", layer("case-board", t("Plywood 9 mm with tolex", "Sperrholz 9 mm mit Bezug"), "timber", "case-tolex", 0.010, 0.077, 5600)),
    region("case-lid", t("Front lid, lying open on the floor", "Vorderer Deckel, offen auf dem Boden liegend"), "case-tolex", AREAS["case-lid"], 0.5, "horizontal-plate-up", layer("lid-board", t("Plywood lid tray with tolex and plush", "Deckelschale aus Sperrholz mit Bezug und Plüsch"), "timber", "case-tolex", 0.025, 0.25, 9000)),
    region("case-lining", t("Plush lining of the tub", "Plüschauskleidung des Unterteils"), "case-plush", AREAS["case-lining"], 0.45, "vertical-plate", layer("plush", t("Velour on foam 8 mm", "Velours auf Schaumstoff 8 mm"), "polymer", "case-plush", 0.008, 0.18, 1350)),
    region("treble-casing", t("Treble casing, celluloid on wood", "Diskantgehäuse, Zelluloid auf Holz"), "celluloid", AREAS["treble-casing"], 0.3, "vertical-plate", layer("treble-wall", t("Celluloid 0.8 mm on a 6 mm wooden wall", "Zelluloid 0,8 mm auf 6 mm Holzwand"), "polymer", "celluloid", 0.0068, 0.04, 6500), {"ratedPowerW": 0, "ratedPowerBasis": "passive"}),
    region("grille", t("Treble grille, chrome-plated steel over cloth", "Diskantgitter, verchromter Stahl über Stoff"), "plated-steel", AREAS["grille"], 0.2, "vertical-plate", layer("grille-sheet", t("Steel sheet 0.6 mm", "Stahlblech 0,6 mm"), "metal", "plated-steel", 0.003, 0.0001, 2200)),
    region("keyboard", t("Treble keyboard of 41 keys", "Diskanttastatur mit 41 Tasten"), "key-plastic", AREAS["keyboard"], 0.09, "vertical-plate", layer("key-tops", t("Key plastic on wooden key levers", "Tastenkunststoff auf Holzhebeln"), "polymer", "key-plastic", 0.012, 0.067, 17600)),
    region("bellows", t("Bellows, cloth over pressboard folds", "Balg, Gewebe über Pressspanfalten"), "bellows-card", AREAS["bellows"], 0.44, "vertical-plate", layer("bellows-fold", t("Pressboard 1.5 mm with cloth", "Pressspan 1,5 mm mit Gewebe"), "polymer", "bellows-card", 0.0015, 0.02, 1400)),
    region("bass-casing", t("Bass casing, celluloid on wood", "Bassgehäuse, Zelluloid auf Holz"), "celluloid", AREAS["bass-casing"], 0.3, "vertical-plate", layer("bass-wall", t("Celluloid 0.8 mm on a 6 mm wooden wall", "Zelluloid 0,8 mm auf 6 mm Holzwand"), "polymer", "celluloid", 0.0068, 0.04, 6500)),
    region("bass-buttons", t("120 bass buttons and the air button", "120 Bassknöpfe und der Luftknopf"), "key-plastic", AREAS["bass-buttons"], 0.01, "cylinder", layer("button", t("Button plastic", "Knopfkunststoff"), "polymer", "key-plastic", 0.009, 0.05, 13000)),
    region("straps", t("Leather shoulder straps, bass strap and bellows straps", "Lederne Schulterriemen, Bassriemen und Balgriemen"), "leather", AREAS["straps"], 0.045, "vertical-plate", layer("strap-leather", t("Leather 4 mm on felt", "Leder 4 mm auf Filz"), "polymer", "leather", 0.008, 0.05, 10300)),
]

ENVIRONMENT = {"airTempC": 20, "meanRadiantTempC": 20, "relativeHumidity": 0.45, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 0, "pressurePa": 101325}

THERMAL = {
    "environment": ENVIRONMENT,
    "passengers": [],
    "operating": [],
    "regions": REGIONS,
    "emitters": [],
    "assumptions": [
        t("An acoustic free-reed instrument draws no power and releases no heat of its own: every region is passive (0 W, stated on the treble casing). The air the bellows pumps through the reeds is room air and is not heated; the accordion is not blown, so there is no breath emitter.",
          "Ein akustisches Durchschlagzungeninstrument nimmt keine Leistung auf und gibt keine eigene Wärme ab: jede Region ist passiv (0 W, am Diskantgehäuse angegeben). Die Luft, die der Balg durch die Stimmzungen pumpt, ist Raumluft und wird nicht erwärmt; das Akkordeon wird nicht angeblasen, daher gibt es keinen Atemauslass."),
        t("Played, the instrument warms where it touches the player: the back of the treble casing against the chest through clothing (30 degC), the keys under the fingertips and the bass buttons (33 degC), the bass strap on the back of the left hand and the shoulder straps (32 degC), the bass foot on the thigh (30 degC; ISO 9886 skin temperatures). Stored, the case stands on a floor at 19 degC and the lid lies flat on it.",
          "Gespielt erwärmt sich das Instrument, wo es den Spieler berührt: die Rückseite des Diskantgehäuses an der Brust durch die Kleidung (30 degC), die Tasten unter den Fingerkuppen und die Bassknöpfe (33 degC), der Bassriemen auf dem linken Handrücken und die Schulterriemen (32 degC), der Bassfuß auf dem Oberschenkel (30 degC; Hauttemperaturen nach ISO 9886). Gelagert steht der Koffer auf einem 19 degC warmen Boden und der Deckel liegt flach darauf."),
        t("Aged surfaces: the crazed and yellowed celluloid keeps the emissivity of new celluloid (0.93), the worn nickel plating of the corners rises from about 0.1 to 0.3 where the steel shows, and the dulled chrome of the grille from 0.07 to 0.15 (authored estimates after Incropera 2018).",
          "Gealterte Oberflächen: das rissige und vergilbte Zelluloid behält den Emissionsgrad von neuem Zelluloid (0,93), die abgeriebene Vernickelung der Ecken steigt von etwa 0,1 auf 0,3, wo der Stahl durchscheint, und das stumpfe Chrom des Gitters von 0,07 auf 0,15 (Schätzwerte nach Incropera 2018)."),
    ],
}


def binding(contacts, exposed):
    """🌬️ The region bindings of one pose: `exposed` per region, `contacts` region -> (fraction, tempC)."""
    out = {}
    for r in REGIONS:
        b = {"exposedFraction": exposed.get(r["id"], 1.0), "sunlitFraction": 0}
        if r["id"] in contacts:
            fr, tc = contacts[r["id"]]
            b["contact"] = {"fraction": fr, "tempC": tc}
        out[r["id"]] = b
    return out


OPEN_CASE = {"case-shell": (0.08, 19), "case-lid": (0.45, 19)}
STORED_EXPOSED = {"case-shell": 0.92, "case-lid": 0.55, "case-lining": 0.7, "treble-casing": 0.85, "keyboard": 0.9, "bellows": 0.9, "bass-casing": 0.85, "bass-buttons": 0.9, "straps": 0.6}
CLOSED_EXPOSED = {"case-shell": 0.55, "case-lid": 0.5, "case-lining": 0.02, "treble-casing": 0.02, "grille": 0.02, "keyboard": 0.02, "bellows": 0.02, "bass-casing": 0.02, "bass-buttons": 0.02, "straps": 0.02}
PLAYED_CONTACTS = {**OPEN_CASE, "treble-casing": (0.12, 30), "keyboard": (0.04, 33), "bass-casing": (0.06, 30), "bass-buttons": (0.08, 33), "straps": (0.45, 32)}
PLAYED_EXPOSED = {"case-shell": 0.92, "case-lid": 0.55, "case-lining": 0.7}
DEALER = "used-accordion"


def poses(supports):
    """🧍️ The named and key poses."""
    feet = [s for s in supports if s.startswith("case-foot")]
    played_contacts = feet + ["lid-rest", "thigh", "strap-hook-right", "strap-hook-left"]
    bellows = lambda d: {f"bellows-{k}": {"dx": d} for k in range(1, 5)}
    lift = {"treble": {"dx": 0.8, "dy": 0.6}}
    loads = lambda pull: [
        {"id": "right-hand-keys", "kind": "live", "title": t("Right-hand fingers pressing three keys", "Finger der rechten Hand drücken drei Tasten"), "anchor": "keys-press", "fxN": 3, "fyN": 0, "source": "authored-estimate", "reference": "richter-1990"},
        {"id": "left-hand-bellows", "kind": "live", "title": t("Left hand pulling the bellows open at the bass strap", "Linke Hand zieht den Balg am Bassriemen auf"), "anchor": "bass-strap-hand", "fxN": pull, "fyN": 0, "source": "authored-estimate", "reference": "richter-1990"},
    ]
    return [
        {"id": "stored", "title": t("Stored at the dealer, case open", "Beim Händler gelagert, Koffer offen"), "description": t("The accordion stands in the plush of its open hard case with the bellows strapped shut; the front lid lies flat on the floor as the dealer opened it for viewing.", "Das Akkordeon steht im Plüsch seines offenen Hartschalenkoffers, der Balg mit den Balgriemen geschlossen; der Vorderdeckel liegt flach auf dem Boden, wie ihn der Händler zum Ansehen geöffnet hat."), "bones": {}, "ground": "auto", "contacts": feet + ["lid-rest"], "thermal": {"regions": binding({**OPEN_CASE, "treble-casing": (0.003, 20), "bass-casing": (0.003, 20)}, STORED_EXPOSED), "emitters": {}}, "loads": []},
        {"id": "case-closed", "title": t("Case closed on the floor", "Koffer geschlossen auf dem Boden"), "description": t("The lid is swung up and latched; the instrument sits enclosed in the plush, ready to be carried.", "Der Deckel ist hochgeklappt und verriegelt; das Instrument sitzt eingeschlossen im Plüsch, bereit zum Tragen."), "bones": {"lid": {"twistDeg": -90}}, "ground": "auto", "contacts": feet, "thermal": {"regions": binding({"case-shell": (0.08, 19), "treble-casing": (0.003, 20), "bass-casing": (0.003, 20)}, CLOSED_EXPOSED), "emitters": {}}, "loads": []},
        {"id": "carried", "title": t("Carried in the case by its handle", "Im Koffer am Griff getragen"), "description": t("One hand carries the closed case of 15.5 kg by its leather handle at knuckle height, 0.76 m above the floor; the whole weight hangs from the handle.", "Eine Hand trägt den geschlossenen Koffer von 15,5 kg am Ledergriff in Fingerknöchelhöhe, 0,76 m über dem Boden; das ganze Gewicht hängt am Griff."), "bones": {"lid": {"twistDeg": -90}}, "root": {"y": 0.215}, "ground": "authored", "contacts": ["handle-grip"], "thermal": {"regions": binding({"case-shell": (0.01, 33), "treble-casing": (0.003, 20), "bass-casing": (0.003, 20)}, {**CLOSED_EXPOSED, "case-shell": 0.99}), "emitters": {}}, "loads": []},
        {"id": "played", "title": t("Played seated", "Im Sitzen gespielt"), "description": t("Out of its case and unstrapped, the accordion rests with its bass foot on the left thigh of a seated player 0.62 m above the floor and hangs from both shoulder straps; the bellows is opened by 160 mm, the right hand presses keys and the left hand pulls at the bass strap. The empty case stays open on the floor beside the chair.", "Aus dem Koffer genommen und abgeschnallt, ruht das Akkordeon mit dem Bassfuß auf dem linken Oberschenkel eines sitzenden Spielers 0,62 m über dem Boden und hängt an beiden Schulterriemen; der Balg ist um 160 mm geöffnet, die rechte Hand drückt Tasten und die linke zieht am Bassriemen. Der leere Koffer bleibt offen neben dem Stuhl auf dem Boden."), "bones": {**lift, **bellows(0.04)}, "ground": "auto", "contacts": played_contacts, "thermal": {"regions": binding(PLAYED_CONTACTS, PLAYED_EXPOSED), "emitters": {}}, "loads": loads(15)},
        {"id": "bellows-pushed", "key": True, "title": t("Bellows pushed in", "Balg zugedrückt"), "description": t("End of the push stroke: the bellows is closed to 20 mm above its strapped length.", "Ende des Druckhubs: der Balg ist bis auf 20 mm über seine geschlossene Länge zusammengedrückt."), "bones": {**lift, **bellows(0.005)}, "ground": "auto", "contacts": played_contacts, "thermal": {"regions": binding(PLAYED_CONTACTS, PLAYED_EXPOSED), "emitters": {}}, "loads": loads(30)},
        {"id": "bellows-pulled", "key": True, "title": t("Bellows pulled open", "Balg aufgezogen"), "description": t("End of the pull stroke: the bellows is open by 300 mm at the bass end.", "Ende des Zughubs: der Balg ist am Bassende um 300 mm geöffnet."), "bones": {**lift, **bellows(0.075)}, "ground": "auto", "contacts": played_contacts, "thermal": {"regions": binding(PLAYED_CONTACTS, PLAYED_EXPOSED), "emitters": {}}, "loads": loads(30)},
    ]


def clips(supports):
    """🎞️ The carry clip and the bellows cycle."""
    feet = [s for s in supports if s.startswith("case-foot")]
    played = feet + ["lid-rest", "thigh", "strap-hook-right", "strap-hook-left"]
    return [
        {"id": "carry", "title": t("Close the case and carry it off", "Koffer schließen und forttragen"), "description": t("The lid swings up from the floor and is latched, then a hand grips the handle and lifts the case of 15.5 kg to knuckle height.", "Der Deckel schwenkt vom Boden hoch und wird verriegelt, dann greift eine Hand den Griff und hebt den Koffer von 15,5 kg auf Fingerknöchelhöhe."), "durationS": 3.0, "fps": 15, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 1.4, "pose": "case-closed", "ease": "smooth"}, {"t": 3.0, "pose": "carried", "ease": "smooth"}],
         "contacts": [*[{"anchor": f, "fromS": 0, "toS": 1.9} for f in feet], {"anchor": "lid-rest", "fromS": 0, "toS": 0.1}, {"anchor": "handle-grip", "fromS": 1.7, "toS": 3.0}],
         "phases": [{"id": "closing", "fromS": 0, "toS": 1.4, "title": t("Closing the lid", "Deckel schließen")}, {"id": "lifting", "fromS": 1.4, "toS": 3.0, "title": t("Lifting by the handle", "Am Griff anheben")}],
         "events": [{"t": 0.1, "kind": "contact-off", "title": t("The lid leaves the floor", "Der Deckel verlässt den Boden"), "target": "lid-rest"}, {"t": 1.4, "kind": "motion", "title": t("Lid latched", "Deckel verriegelt"), "target": "lid"}, {"t": 1.7, "kind": "contact-on", "title": t("Hand on the handle", "Hand am Griff"), "target": "handle-grip"}, {"t": 1.9, "kind": "contact-off", "title": t("The case leaves the floor", "Der Koffer verlässt den Boden"), "target": "case-foot-treble-front"}],
         "tracks": []},
        {"id": "bellows-cycle", "title": t("Bellows open and close, seated", "Balg auf und zu, im Sitzen"), "description": t("One bellows cycle of six seconds while the player sits: three seconds pulling the bellows open from 20 mm to 300 mm, three seconds pushing it closed again; the force of the left hand reverses at the turn.", "Ein Balgzyklus von sechs Sekunden im Sitzen: drei Sekunden Aufziehen des Balgs von 20 mm auf 300 mm, drei Sekunden Zudrücken; die Kraft der linken Hand kehrt sich am Wendepunkt um."), "durationS": 6.0, "fps": 15, "loop": True, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "bellows-pushed", "ease": "smooth"}, {"t": 3.0, "pose": "bellows-pulled", "ease": "smooth"}, {"t": 6.0, "pose": "bellows-pushed", "ease": "smooth"}],
         "contacts": [{"anchor": a, "fromS": 0, "toS": 6.0} for a in played],
         "phases": [{"id": "pull", "fromS": 0, "toS": 3.0, "title": t("Pull stroke", "Zughub")}, {"id": "push", "fromS": 3.0, "toS": 6.0, "title": t("Push stroke", "Druckhub")}],
         "events": [{"t": 3.0, "kind": "motion", "title": t("Bellows turns", "Balgwende"), "target": "bass"}],
         "tracks": [{"target": "load:left-hand-bellows", "ease": "step", "keys": [{"t": 0, "v": 1}, {"t": 3.0, "v": -1}]}]},
    ]


def rep_decisions(scale):
    """📝️ The design decisions of one representation."""
    if scale == 50:
        return {
            "silhouette": t("A used 120-bass piano accordion standing in its open hard case, 0.50 m long, 0.27 m deep and 0.545 m high with the handle, the front lid lying flat on the floor: seen from the open front (side elevation) the tub frame with its steel corners and latches, the wine-red lining behind the instrument, and the instrument itself with the keyboard at the treble end, the grille, the four bellows segments with their folds, the bass box with the button rows and the bass strap; front: the bass end of the case with an old dealer sticker, the lid as a strip on the floor; plan: the case top with handle and latches and the open lid with its plush face.",
                            "Ein gebrauchtes 120-Bass-Pianoakkordeon im offenen Hartschalenkoffer, 0,50 m lang, 0,27 m tief und mit Griff 0,545 m hoch, der Vorderdeckel flach auf dem Boden: von der offenen Vorderseite (Seitenansicht) der Rahmen des Unterteils mit Stahlecken und Schlössern, das weinrote Futter hinter dem Instrument und das Instrument selbst mit der Tastatur am Diskantende, dem Gitter, den vier Balgabschnitten mit ihren Falten, dem Bassteil mit den Knopfreihen und dem Bassriemen; Vorderansicht: das Bassende des Koffers mit einem alten Händleraufkleber, der Deckel als Streifen auf dem Boden; Draufsicht: der Kofferdeckel mit Griff und Schlössern und der offene Deckel mit seiner Plüschseite."),
            "retained": t("Case boards 10 mm, lining 8 mm, lid 25 mm, corner caps 30 mm, latches, handle on two lugs; treble box 105 x 455 x 190 mm with grille 73 x 365 mm, key bed, 24 white and 17 black keys at 17.7 mm pitch, two strap hooks, two feet; bellows closed 120 mm in four segments; bass box 140 mm with 120 buttons of 9 mm in six rows, air button, bass strap, two feet; two bellows straps.",
                         "Kofferbretter 10 mm, Futter 8 mm, Deckel 25 mm, Eckkappen 30 mm, Schlösser, Griff auf zwei Ösen; Diskantteil 105 x 455 x 190 mm mit Gitter 73 x 365 mm, Tastenbett, 24 weißen und 17 schwarzen Tasten im Raster 17,7 mm, zwei Riemenhaken, zwei Füßen; Balg geschlossen 120 mm in vier Abschnitten; Bassteil 140 mm mit 120 Knöpfen von 9 mm in sechs Reihen, Luftknopf, Bassriemen, zwei Füßen; zwei Balgriemen."),
            "merged": t("Each bellows segment carries five folds; the 120 buttons are six row strips in elevation and plan and single dots only in the front view; the white keys are one strip with their joints as strokes.", "Jeder Balgabschnitt trägt fünf Falten; die 120 Knöpfe sind in Seitenansicht und Draufsicht sechs Reihenstreifen und nur in der Vorderansicht einzelne Punkte; die weißen Tasten sind ein Streifen mit ihren Fugen als Strichen."),
            "omitted": t("Register switches, reed blocks, the bellows valves inside, screws and the cloth behind the grille.", "Registerschalter, Stimmstöcke, die Ventile im Balg, Schrauben und der Stoff hinter dem Gitter."),
            "thinElements": t("The 3 mm grille and the 3 mm bellows straps are drawn as filled strips relying on the body pen; key joints, bellows folds, crazing and scuffs are detail strokes.", "Das 3 mm dicke Gitter und die 3 mm dicken Balgriemen sind gefüllte Streifen, die die Körperlinie trägt; Tastenfugen, Balgfalten, Haarrisse und Schrammen sind Detailstriche."),
            "hatching": t("None; the grille bars are three detail strokes.", "Keine; die Gitterstäbe sind drei Detailstriche."),
            "styles": t("Tolex 0.22, lining 0.42, celluloid 0.38, grille 0.78, white keys 0.86 (yellowed), black keys 0.08, bellows 0.13 with folds 0.62 and worn corners 0.5, leather 0.3, steel 0.62; body pen 0.25 mm, detail 0.13 mm.", "Bezug 0,22, Futter 0,42, Zelluloid 0,38, Gitter 0,78, weiße Tasten 0,86 (vergilbt), schwarze Tasten 0,08, Balg 0,13 mit Falten 0,62 und abgeriebenen Ecken 0,5, Leder 0,3, Stahl 0,62; Körperstift 0,25 mm, Detail 0,13 mm."),
            "overlays": t("Traces of use: worn bellows corners, crazing on the bass casing, scuffs on the tub edges and an old dealer sticker on the bass end of the case.", "Gebrauchsspuren: abgeriebene Balgecken, Haarrisse im Bassgehäuse, Schrammen an den Kofferkanten und ein alter Händleraufkleber am Bassende des Koffers."),
        }
    if scale == 100:
        return {
            "silhouette": t("The same open case and accordion at 1:100, 5 mm high on paper: tub frame, lining, lid strip; the accordion as treble box with keyboard and grille, four bellows segments, bass box with button field and bass strap.", "Derselbe offene Koffer mit Akkordeon bei 1:100, auf Papier 5 mm hoch: Rahmen des Unterteils, Futter, Deckelstreifen; das Akkordeon als Diskantteil mit Tastatur und Gitter, vier Balgabschnitte, Bassteil mit Knopffeld und Bassriemen."),
            "retained": t("Case boards and lining at their true 10 and 8 mm, the lid, the handle; keyboard as key bed with white keys and the black-key ridge, grille, bellows segments with three folds each, bass button field, bass strap.", "Kofferbretter und Futter mit ihren wirklichen 10 und 8 mm, der Deckel, der Griff; Tastatur als Tastenbett mit weißen Tasten und der Leiste der schwarzen Tasten, Gitter, Balgabschnitte mit je drei Falten, Bassknopffeld, Bassriemen."),
            "merged": t("Black keys into one ridge, the 120 buttons into one field, feet into the casings.", "Schwarze Tasten zu einer Leiste, die 120 Knöpfe zu einem Feld, Füße in die Gehäuse."),
            "omitted": t("Corner caps, latches, strap hooks, shoulder and bellows straps, key joints, worn corners, crazing and the sticker, all below 15 mm.", "Eckkappen, Schlösser, Riemenhaken, Schulter- und Balgriemen, Tastenfugen, abgeriebene Ecken, Haarrisse und Aufkleber, alle unter 15 mm."),
            "thinElements": t("Boards, lining and grille keep their thickness as thin filled strips carried by the body pen.", "Bretter, Futter und Gitter behalten ihre Dicke als dünne gefüllte Streifen, die die Körperlinie trägt."),
            "hatching": t("None.", "Keine."),
            "styles": t("Tones as at 1:50; body pen 0.18 mm, detail 0.1 mm for the folds.", "Töne wie bei 1:50; Körperstift 0,18 mm, Detail 0,1 mm für die Falten."),
            "overlays": t("None.", "Keine."),
        }
    return {
        "silhouette": t("The open case with the accordion at 1:200, 2.7 mm high: a frame of 18 mm generalised boards around the instrument mass, the lid strip, the handle arch.", "Der offene Koffer mit Akkordeon bei 1:200, 2,7 mm hoch: ein Rahmen aus auf 18 mm verallgemeinerten Brettern um die Masse des Instruments, der Deckelstreifen, der Griffbogen."),
        "retained": t("Case frame, lining wall, lid, handle; treble box with keyboard step and grille, four bellows segments, bass box with the end block of buttons and strap.", "Kofferrahmen, Futterwand, Deckel, Griff; Diskantteil mit Tastaturstufe und Gitter, vier Balgabschnitte, Bassteil mit dem Endblock aus Knöpfen und Riemen."),
        "merged": t("Board and lining into one 18 mm frame, handle lugs and grip into one arch, feet into the casings.", "Brett und Futter zu einem 18-mm-Rahmen, Griffösen und Griff zu einem Bogen, Füße in die Gehäuse."),
        "omitted": t("Folds, fittings, straps other than the bass strap and every trace of use.", "Falten, Beschläge, Riemen außer dem Bassriemen und alle Gebrauchsspuren."),
        "thinElements": t("The grille stays a filled strip carried by the body pen.", "Das Gitter bleibt ein gefüllter Streifen, den die Körperlinie trägt."),
        "hatching": t("None.", "Keine."),
        "styles": t("Tones as at 1:50; body pen 0.13 mm.", "Töne wie bei 1:50; Körperstift 0,13 mm."),
        "overlays": t("None.", "Keine."),
    }


VOLUME = {
    50: [t("Body 1:50: the case tub of five 10 mm boards with 8 mm plush lining on floor, back and ends, eight steel corner caps of 30 mm and two latches; the 25 mm lid lies on the floor with two caps at its far edge; the handle is a leather capsule of 18 x 25 mm on two lugs. The accordion: rounded treble and bass boxes of 105 and 140 mm, the 3 mm grille as a declared sheet, the key bed with 24 white keys (repeat at 17.7 mm) and 17 black keys in seven groups, shoulder straps of 8 mm, strap hooks, four rubber feet, 120 bass buttons as six revolved rows of 20, the air button, the bass strap as a C-shaped extrusion, the two bellows straps as texture solids 3 mm thick. The bellows is four telescoping segments of 110 mm; closed they overlap and the outer one hides the inner, open each shows its five folds.",
           "Körper 1:50: das Kofferunterteil aus fünf 10-mm-Brettern mit 8 mm Plüschfutter an Boden, Rückwand und Enden, acht Stahleckkappen von 30 mm und zwei Schlössern; der 25-mm-Deckel liegt auf dem Boden mit zwei Kappen an der vorderen Kante; der Griff ist eine Lederkapsel von 18 x 25 mm auf zwei Ösen. Das Akkordeon: gerundete Diskant- und Bassgehäuse von 105 und 140 mm, das 3-mm-Gitter als erklärte Platte, das Tastenbett mit 24 weißen Tasten (Wiederholung im Raster 17,7 mm) und 17 schwarzen Tasten in sieben Gruppen, Schulterriemen von 8 mm, Riemenhaken, vier Gummifüße, 120 Bassknöpfe als sechs gedrehte Reihen zu 20, der Luftknopf, der Bassriemen als C-förmige Extrusion, die zwei Balgriemen als 3 mm dicke Texturkörper. Der Balg besteht aus vier teleskopierenden Abschnitten von 110 mm; geschlossen überlappen sie und der äußere verdeckt den inneren, geöffnet zeigt jeder seine fünf Falten."),
         t("Thermal 1:50: every body solid is a surface of its region by reference. Structure 1:50: the boards, lid, handle, casings and bellows segments are the members of their bones, the piano hinge a slender joint, the seat of the instrument on the case floor and the bellows frames are joints, one support pad per anchor and one mass marker per bone.",
           "Thermik 1:50: jeder Körper ist per Verweis Oberfläche seiner Region. Struktur 1:50: Bretter, Deckel, Griff, Gehäuse und Balgabschnitte sind die Stäbe ihrer Knochen, das Klavierband ein schlankes Gelenk, der Sitz des Instruments auf dem Kofferboden und die Balgrahmen sind Gelenke, ein Auflagerklotz je Anker und eine Massenmarke je Knochen.")],
    100: [t("Body 1:100, drawn anew: boards and lining keep their true 10 and 8 mm as declared sheets; corner caps, latches, hooks, shoulder and bellows straps are below 15 mm and left out; the casings stand on the lining without feet; the keyboard is one extrusion of key bed, white keys and a continuous black-key ridge; the 120 buttons are one field block 10 mm proud of the bass end; the handle keeps lugs and grip.",
            "Körper 1:100, neu gezeichnet: Bretter und Futter behalten ihre wirklichen 10 und 8 mm als erklärte Platten; Eckkappen, Schlösser, Haken, Schulter- und Balgriemen liegen unter 15 mm und entfallen; die Gehäuse stehen ohne Füße auf dem Futter; die Tastatur ist eine Extrusion aus Tastenbett, weißen Tasten und durchgehender Leiste der schwarzen Tasten; die 120 Knöpfe sind ein Feldblock, der 10 mm über das Bassende steht; der Griff behält Ösen und Griffstück."),
          t("Thermal and structure 1:100 as at 1:50 on these solids.", "Thermik und Struktur 1:100 wie bei 1:50 auf diesen Körpern.")],
    200: [t("Body 1:200, drawn anew: the case is a frame of 18 mm generalised boards (board and lining merged), the back wall one 18 mm block of board and lining, the handle one arch 30 mm wide; the accordion keeps its treble box with the keyboard step, the grille sheet, the four bellows segments, the bass box with the button block and the bass strap.",
            "Körper 1:200, neu gezeichnet: der Koffer ist ein Rahmen aus auf 18 mm verallgemeinerten Brettern (Brett und Futter verschmolzen), die Rückwand ein 18-mm-Block aus Brett und Futter, der Griff ein 30 mm breiter Bogen; das Akkordeon behält sein Diskantteil mit Tastaturstufe, die Gitterplatte, die vier Balgabschnitte, das Bassteil mit Knopfblock und Bassriemen."),
          t("Thermal and structure 1:200 on these solids; the merged back block carries the lining region.", "Thermik und Struktur 1:200 auf diesen Körpern; der verschmolzene Rückwandblock trägt die Futterregion.")],
}

TEXT = {
    "summary": t("A second-hand 120-bass piano accordion of the student and amateur class (41 treble keys, 120 Stradella basses, 10.2 kg, 0.48 m high, 0.42 m long with the bellows closed) standing in its open plywood hard case of 5.3 kg with the lid on the floor, as a used-instrument dealer shows it: yellowed keys, crazed celluloid, worn bellows corners, a scuffed case with an old sticker. Carried in the closed case by its handle, played seated on the left thigh with the bellows opening and closing.",
                 "Ein gebrauchtes 120-Bass-Pianoakkordeon der Schüler- und Amateurklasse (41 Diskanttasten, 120 Stradella-Bässe, 10,2 kg, 0,48 m hoch, mit geschlossenem Balg 0,42 m lang) im offenen Hartschalenkoffer aus Sperrholz von 5,3 kg, der Deckel auf dem Boden, wie ein Gebrauchtinstrumentenhändler es zeigt: vergilbte Tasten, rissiges Zelluloid, abgeriebene Balgecken, ein abgestoßener Koffer mit altem Aufkleber. Im geschlossenen Koffer am Griff getragen, im Sitzen auf dem linken Oberschenkel gespielt, mit sich öffnendem und schließendem Balg."),
    "provenance": {"author": "Semio staffage content (used instruments wave)", "method": "handcrafted", "created": "2026-11-22", "references": REFS,
                   "notes": t("Type of the 41/120/III piano accordion as built in Klingenthal and elsewhere from the 1960s (Hornbostel-Sachs 412.132; Richter 1990): no brand, no logo. Dimensions rounded from catalogue data of the class (48 x 18.5 to 48.5 x 19.5 cm, 42.5 to 44 cm long closed, 9.2 to 12.4 kg): treble keyboard of 24 white keys at 17.7 mm (424.8 mm), casings 455 mm high and 190 mm deep, treble box 105 mm, bellows 120 mm closed and up to 440 mm open, bass box 140 mm with 120 buttons of 9 mm. Masses: treble 5.6 kg (casing, keyboard, four reed sets), bellows 0.7 kg, bass 3.9 kg. Hard case after the dealer class: 0.50 x 0.27 x 0.51 m of 9 mm poplar plywood with tolex and plush, 5.3 kg. All values are authored estimates or literature, none measured.",
                             "Typ des Pianoakkordeons 41/120/III, wie es ab den 1960er-Jahren in Klingenthal und anderswo gebaut wurde (Hornbostel-Sachs 412.132; Richter 1990): keine Marke, kein Logo. Maße gerundet aus Katalogdaten der Klasse (48 x 18,5 bis 48,5 x 19,5 cm, geschlossen 42,5 bis 44 cm lang, 9,2 bis 12,4 kg): Diskanttastatur aus 24 weißen Tasten im Raster 17,7 mm (424,8 mm), Gehäuse 455 mm hoch und 190 mm tief, Diskantteil 105 mm, Balg geschlossen 120 mm und offen bis 440 mm, Bassteil 140 mm mit 120 Knöpfen von 9 mm. Massen: Diskant 5,6 kg (Gehäuse, Tastatur, vier Stimmzungensätze), Balg 0,7 kg, Bass 3,9 kg. Hartschalenkoffer nach der Händlerklasse: 0,50 x 0,27 x 0,51 m aus 9-mm-Pappelsperrholz mit Bezug und Plüsch, 5,3 kg. Alle Werte sind Schätzwerte oder Literatur, keiner ist gemessen.")},
    "materials": MATERIALS,
    "thermal": THERMAL,
    "poses": poses,
    "clips": clips,
    "representation": {s: rep_decisions(s) for s in (50, 100, 200)},
    "volume": VOLUME,
    "structureAssumptions": [
        t("Stored, the case stands on four corner contacts and the open lid on its far edge; the instrument of 10.2 kg rests on the plush of the case floor. Carried, the whole 15.5 kg hangs from the handle grip. Played, the instrument rests on the left thigh (contact) and hangs from the two shoulder-strap hooks (pins), the right hand presses keys with about 3 N and the left hand works the bellows with 15 to 30 N at the bass strap, reversing with the stroke.",
          "Gelagert steht der Koffer auf vier Eckkontakten und der offene Deckel auf seiner vorderen Kante; das Instrument von 10,2 kg ruht auf dem Plüsch des Kofferbodens. Getragen hängen die ganzen 15,5 kg am Griff. Gespielt ruht das Instrument auf dem linken Oberschenkel (Kontakt) und hängt an den zwei Haken der Schulterriemen (Gelenke), die rechte Hand drückt Tasten mit etwa 3 N und die linke bewegt den Balg mit 15 bis 30 N am Bassriemen, mit dem Hub wechselnd."),
        t("The bellows is idealised as four rigid segments on prismatic joints along the bellows axis; the real bellows carries the hand force through the air pressure inside it, which this quasi-static model replaces by the joints.",
          "Der Balg ist als vier starre Abschnitte auf Schubgelenken entlang der Balgachse idealisiert; der wirkliche Balg trägt die Handkraft über den Luftdruck in seinem Inneren, den dieses quasistatische Modell durch die Gelenke ersetzt."),
    ],
    "unavailable500": {"status": "unavailable", "denominator": 500, "cause": "below-printable-feature", "reason": t("At 1:500 the open case is 1.0 by 1.1 mm on paper and the accordion inside it under 1 mm: below 3 mm the instrument cannot be told from a box.", "Bei 1:500 ist der offene Koffer auf Papier 1,0 mal 1,1 mm groß und das Akkordeon darin unter 1 mm: unter 3 mm ist das Instrument nicht von einer Kiste zu unterscheiden."), "limitingFeature": t("Overall height of 0.545 m", "Gesamthöhe von 0,545 m"), "limitingSizeM": 0.545, "alternatives": [{"kind": "scale", "denominator": 200}, {"kind": "scale", "denominator": 100}]},
}
