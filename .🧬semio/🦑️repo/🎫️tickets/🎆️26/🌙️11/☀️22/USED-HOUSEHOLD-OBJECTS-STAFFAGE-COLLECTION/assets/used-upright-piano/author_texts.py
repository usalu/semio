"""📝 Texts, poses, clips, thermal and structure records of the used upright piano, English first and German second.

See also author-piano.py and author_parts.py.
"""
import json

from author_geom import T, r6
import author_parts as P

SUMMARY = T(
    "A second-hand German upright piano of the 125 cm class from the 1970s, 1.50 x 0.60 x 1.25 m and about 230 kg: walnut-veneered case with a 15 mm cast-iron frame, 88 yellowed keys, three brass pedals and a vinyl bench. The lid is bleached by the sun and ringed by glasses, the fallboard scratched, one key top is missing and old note stickers sit on the name board. In the depot it stands strapped on a piano dolly, it is rolled away by two movers and tipped over a door sill, and back in a living room it is played with the fallboard open.",
    "Ein gebrauchtes deutsches Klavier der 125-cm-Klasse aus den 1970er Jahren, 1,50 x 0,60 x 1,25 m und etwa 230 kg: nussbaumfurniertes Gehäuse mit 15 mm Gussplatte, 88 vergilbte Tasten, drei Messingpedale und eine Kunstlederbank. Der Deckel ist von der Sonne ausgebleicht und von Gläsern beringt, die Tastenklappe zerkratzt, ein Tastenbelag fehlt und alte Notenaufkleber kleben auf der Namensleiste. Im Lager steht es verzurrt auf einem Klavierroller, zwei Träger rollen es weg und kippen es über eine Türschwelle, und zurück in einem Wohnzimmer wird es mit offener Tastenklappe gespielt.",
)

PROVENANCE_NOTES = T(
    "Built to the catalogue-typical West or East German upright piano of the 125 cm class of 1965-1990 (no brand), the removal firms' piano dolly and a 50 mm ratchet lashing strap; every parameter is literature or an authored estimate, nothing is measured.",
    "Gebaut nach dem katalogtypischen west- oder ostdeutschen Klavier der 125-cm-Klasse der Jahre 1965-1990 (ohne Marke), dem Klavierroller der Umzugsfirmen und einem 50-mm-Zurrgurt mit Ratsche; jeder Parameter ist Literatur oder Schätzung, nichts ist gemessen.",
)

REGION_IDS = ["case", "lid", "fallboard", "keyboard", "back", "brass-fittings", "dolly", "strap", "bench", "bench-seat"]


def binding(states, contacts=None):
    """🌬️ The region states of one pose: (exposed, sunlit) per region and the conductive contacts."""
    regions = {}
    for rid in REGION_IDS:
        e, s = states.get(rid, (1.0, 0.0))
        regions[rid] = {"exposedFraction": e, "sunlitFraction": s}
        if contacts and rid in contacts:
            regions[rid]["contact"] = {"fraction": contacts[rid][0], "tempC": contacts[rid][1]}
    return {"regions": regions, "emitters": {}}


STORED_STATE = {"case": (1.0, 0.15), "lid": (1.0, 0.4), "fallboard": (1.0, 0.3), "keyboard": (0.0, 0.0), "back": (1.0, 0.0), "brass-fittings": (1.0, 0.1), "dolly": (1.0, 0.1), "strap": (1.0, 0.3), "bench": (1.0, 0.3), "bench-seat": (1.0, 0.4)}
CARRIED_STATE = {rid: (1.0, 0.0) for rid in REGION_IDS}
CARRIED_STATE["keyboard"] = (0.0, 0.0)
USE_STATE = {"case": (1.0, 0.1), "lid": (1.0, 0.3), "fallboard": (1.0, 0.2), "keyboard": (0.15, 0.1), "back": (1.0, 0.0), "brass-fittings": (1.0, 0.0), "dolly": (1.0, 0.0), "strap": (1.0, 0.0), "bench": (1.0, 0.1), "bench-seat": (0.4, 0.0)}
SEATED = {"bench-seat": (0.6, 32.0)}
PIANIST_N = 600.0

IN_USE_BONES = {"piano-level": {"dy": r6(-P.DOLLY_TOP)}, "dolly": {"dx": P.DOLLY_PARK_DX}, "strap-drop": {"dy": r6(P.STRAP_DROP_DY)}, "strap": {"angleDeg": 90}, "fallboard": {"angleDeg": P.FALL_OPEN}, "bench-slide": {"dx": P.BENCH_USE_DX}}
LID_BONES = dict(IN_USE_BONES)
LID_BONES["lid"] = {"angleDeg": P.LID_OPEN}
CARRIED_BONES = {"dolly-roll": {"dx": P.CARRY_DX}, "dolly-tilt": {"angleDeg": P.CARRY_TILT}}


def pianist():
    """🧑‍🎤 The pianist on the bench: the share of body weight the seat carries (feet and forearms take the rest)."""
    return [{"id": "pianist", "kind": "occupant", "title": T(f"Pianist seated on the bench ({PIANIST_N:.0f} N on the seat)", f"Pianist auf der Bank sitzend ({PIANIST_N:.0f} N auf der Sitzfläche)"), "anchor": "bench-seat", "fxN": 0, "fyN": -PIANIST_N, "source": "authored-estimate", "reference": "en-12520-2015"}]


USE_CONTACTS = P.PIANO_CASTORS + P.DOLLY_CASTORS + P.BENCH_FEET


def poses(f):
    """🧍 The poses: stored on the dolly, rolled away and tipped over a sill, in use, lid open."""
    kg = f["piano"]
    return [
        {"id": "stored", "title": T("Stored in the reuse depot", "Gelagert im Gebrauchtwarenlager"), "description": T(f"Emptied of sheet music and dusted, the {kg:.0f} kg piano stands on its piano dolly in the depot, lashed to it with one 50 mm ratchet strap over the lid; lid and fallboard are closed, the bench is pushed against the keyslip.", f"Von Noten geleert und abgestaubt steht das {kg:.0f} kg schwere Klavier im Lager auf seinem Klavierroller, mit einem 50-mm-Zurrgurt über den Deckel daran verzurrt; Deckel und Tastenklappe sind zu, die Bank steht an der Vorsatzleiste."), "bones": {}, "ground": "auto", "contacts": P.DOLLY_CASTORS + P.BENCH_FEET, "thermal": binding(STORED_STATE), "loads": []},
        {"id": "carried", "title": T("Rolled away by two movers, tipped over a sill", "Von zwei Trägern weggerollt, über eine Schwelle gekippt"), "description": T(f"Two movers pull the strapped piano on its dolly 0.70 m backwards by the carrying grips in the outer back posts and tip it 3 degrees over the back castors so the front castors clear a door sill of 19 mm; the bench stays behind in the depot.", f"Zwei Träger ziehen das verzurrte Klavier auf seinem Roller an den Tragegriffen der äußeren Rasten 0,70 m rückwärts und kippen es um 3 Grad über die hinteren Rollen, damit die vorderen Rollen eine 19 mm hohe Türschwelle überwinden; die Bank bleibt im Lager."), "bones": CARRIED_BONES, "ground": "authored", "contacts": P.DOLLY_BACK + P.GRIPS + P.BENCH_FEET, "thermal": binding(CARRIED_STATE), "loads": []},
        {"id": "in-use", "title": T("Played in a living room", "Gespielt im Wohnzimmer"), "description": T("Back in a living room the piano stands on its own brass castors, fallboard open, a pianist on the bench at playing distance; the dolly with the released strap is parked behind it, as the movers left it.", "Zurück in einem Wohnzimmer steht das Klavier auf seinen eigenen Messingrollen, die Tastenklappe offen, ein Pianist auf der Bank in Spielabstand; der Roller mit dem gelösten Gurt steht dahinter, wie die Träger ihn abgestellt haben."), "bones": IN_USE_BONES, "ground": "auto", "contacts": USE_CONTACTS, "thermal": binding(USE_STATE, SEATED), "loads": pianist()},
        {"id": "lid-open", "title": T("Played with the lid open", "Gespielt mit offenem Deckel"), "description": T("As in use, with the top lid opened 80 degrees on its back hinge so the sound leaves the case over the strings.", "Wie in Gebrauch, mit dem Deckel um 80 Grad am hinteren Scharnier geöffnet, damit der Klang über den Saiten aus dem Gehäuse tritt."), "bones": LID_BONES, "ground": "auto", "contacts": USE_CONTACTS, "thermal": binding(USE_STATE, SEATED), "loads": pianist()},
    ]


def clips():
    """🎞️ The clips: carry (stored to carried), the pedals played in use, the lid opened and closed."""
    roll = {"dolly-roll": {"dx": P.CARRY_DX}}
    carry = {
        "id": "carry", "title": T("Roll the strapped piano away", "Das verzurrte Klavier wegrollen"),
        "description": T("Two movers take hold of the carrying grips, roll the piano on its dolly 0.70 m backwards and tip it over the back castors to clear a door sill; the bench stays where it was.", "Zwei Träger fassen die Tragegriffe, rollen das Klavier auf seinem Roller 0,70 m rückwärts und kippen es über die hinteren Rollen über eine Türschwelle; die Bank bleibt stehen."),
        "durationS": 4.4, "fps": 25, "loop": False, "rootMotion": "none",
        "keys": [{"t": 0, "pose": "stored", "ease": "linear"}, {"t": 1.0, "pose": "stored", "ease": "smooth"}, {"t": 3.4, "pose": "stored", "ease": "smooth", "bones": roll}, {"t": 4.4, "pose": "carried", "ease": "linear"}],
        "contacts": [{"anchor": a, "fromS": 0, "toS": 4.4} for a in P.DOLLY_BACK + P.BENCH_FEET] + [{"anchor": a, "fromS": 0, "toS": 3.4} for a in P.DOLLY_CASTORS if a not in P.DOLLY_BACK] + [{"anchor": g, "fromS": 1.0, "toS": 4.4} for g in P.GRIPS],
        "phases": [{"id": "take-hold", "fromS": 0, "toS": 1.0, "title": T("Taking hold of the grips", "Griffe fassen")}, {"id": "roll", "fromS": 1.0, "toS": 3.4, "title": T("Rolling backwards", "Rückwärts rollen")}, {"id": "tip", "fromS": 3.4, "toS": 4.4, "title": T("Tipping over the sill", "Über die Schwelle kippen")}],
        "events": [{"t": 1.0, "kind": "contact-on", "title": T("Movers take hold of the back posts", "Träger fassen die Rasten"), "target": "grip-back-l"}, {"t": 3.4, "kind": "contact-off", "title": T("Front castors lift off", "Vordere Rollen heben ab"), "target": "dolly-castor-front-l"}],
        "tracks": [],
    }
    down = P.PEDAL_DOWN
    pedal = {
        "id": "pedal", "title": T("Play the pedals", "Die Pedale treten"),
        "description": T("In the living room the pianist presses the right (sustain) pedal for a phrase and releases it, then the left (soft) pedal; each tread goes down 14 mm at its tip.", "Im Wohnzimmer tritt der Pianist für eine Phrase das rechte (Forte-)Pedal und lässt es los, dann das linke (Piano-)Pedal; jede Trittfläche geht an der Spitze 14 mm hinunter."),
        "durationS": 4.0, "fps": 25, "loop": True, "rootMotion": "none",
        "keys": [
            {"t": 0, "pose": "in-use", "ease": "out"},
            {"t": 0.3, "pose": "in-use", "ease": "linear", "bones": {"pedal-sustain": {"angleDeg": down}}},
            {"t": 1.5, "pose": "in-use", "ease": "in", "bones": {"pedal-sustain": {"angleDeg": down}}},
            {"t": 1.8, "pose": "in-use", "ease": "linear"},
            {"t": 2.2, "pose": "in-use", "ease": "out"},
            {"t": 2.5, "pose": "in-use", "ease": "linear", "bones": {"pedal-soft": {"angleDeg": down}}},
            {"t": 3.4, "pose": "in-use", "ease": "in", "bones": {"pedal-soft": {"angleDeg": down}}},
            {"t": 3.7, "pose": "in-use", "ease": "linear"},
        ],
        "contacts": [{"anchor": a, "fromS": 0, "toS": 4.0} for a in USE_CONTACTS],
        "phases": [{"id": "sustain", "fromS": 0, "toS": 2.0, "title": T("Sustain pedal", "Fortepedal")}, {"id": "soft", "fromS": 2.0, "toS": 4.0, "title": T("Soft pedal", "Pianopedal")}],
        "events": [{"t": 0.3, "kind": "motion", "title": T("Sustain pedal down: dampers lifted", "Fortepedal unten: Dämpfer abgehoben"), "target": "pedal-sustain"}, {"t": 2.5, "kind": "motion", "title": T("Soft pedal down: hammers moved closer", "Pianopedal unten: Hämmer näher an den Saiten"), "target": "pedal-soft"}],
        "tracks": [],
    }
    lid = {
        "id": "open-lid", "title": T("Open and close the lid", "Den Deckel öffnen und schließen"),
        "description": T("The pianist lifts the top lid on its back hinge to 80 degrees for a fuller sound, plays, and lowers it again.", "Der Pianist hebt den Deckel am hinteren Scharnier auf 80 Grad für einen volleren Klang, spielt und senkt ihn wieder."),
        "durationS": 5.0, "fps": 25, "loop": True, "rootMotion": "none",
        "keys": [{"t": 0, "pose": "in-use", "ease": "smooth"}, {"t": 1.5, "pose": "lid-open", "ease": "linear"}, {"t": 3.5, "pose": "lid-open", "ease": "smooth"}, {"t": 4.6, "pose": "in-use", "ease": "linear"}],
        "contacts": [{"anchor": a, "fromS": 0, "toS": 5.0} for a in USE_CONTACTS],
        "phases": [{"id": "lift", "fromS": 0, "toS": 1.5, "title": T("Lifting the lid", "Deckel anheben")}, {"id": "play", "fromS": 1.5, "toS": 3.5, "title": T("Playing", "Spielen")}, {"id": "lower", "fromS": 3.5, "toS": 5.0, "title": T("Lowering the lid", "Deckel senken")}],
        "events": [{"t": 1.5, "kind": "motion", "title": T("Lid open", "Deckel offen"), "target": "lid"}, {"t": 4.6, "kind": "motion", "title": T("Lid closed", "Deckel zu"), "target": "lid"}],
        "tracks": [],
    }
    return [carry, pedal, lid]


def thermal(regions):
    """🌡️ The thermal record: unheated depot hall, ten passive regions, no element draws power."""
    return {
        "environment": {"airTempC": 13, "meanRadiantTempC": 12, "relativeHumidity": 0.65, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 250, "diffuseFraction": 0.3, "pressurePa": 101325},
        "operating": [],
        "regions": regions,
        "emitters": [],
        "passengers": [],
        "assumptions": [
            T("Passive wood and metal: veneered chipboard P2 (680 kg/m3, 0.14 W/mK, 1700 J/kgK after EN 312 and ISO 10456), beech, spruce, cast iron, brass, plywood, needle felt, polyester webbing and a vinyl foam cushion; areal heat capacity rho c d of each layer.", "Passives Holz und Metall: furnierte Spanplatte P2 (680 kg/m3, 0,14 W/mK, 1700 J/kgK nach EN 312 und ISO 10456), Buche, Fichte, Grauguss, Messing, Sperrholz, Nadelfilz, Polyestergurt und ein Schaumpolster mit Kunstleder; flächenbezogene Wärmekapazität rho c d jeder Schicht."),
            T("Aged surfaces differ from new ones: the lid veneer bleached by decades of window sun is taken with a solar absorptance of 0.52 against about 0.72 for fresh dark walnut, the dulled case lacquer with 0.68, the yellowed key tops with 0.38 against about 0.25 for new white acrylic, the raw spruce of the back darkened to 0.55 against about 0.40, tarnished brass with an emissivity of 0.35 against about 0.05 polished (authored estimates from the colour; wood emissivity 0.9 and oxidised brass 0.22-0.6 after Incropera Table A.11).", "Gealterte Oberflächen unterscheiden sich von neuen: das von Jahrzehnten Fenstersonne ausgebleichte Deckelfurnier wird mit einem solaren Absorptionsgrad von 0,52 statt etwa 0,72 für frisches dunkles Nussbaum angesetzt, der stumpfe Gehäuselack mit 0,68, die vergilbten Tastenbeläge mit 0,38 statt etwa 0,25 für neues weißes Acryl, die nachgedunkelte rohe Fichte der Rückseite mit 0,55 statt etwa 0,40, angelaufenes Messing mit einem Emissionsgrad von 0,35 statt etwa 0,05 poliert (Schätzwerte aus der Farbe; Emissionsgrad von Holz 0,9 und oxidiertem Messing 0,22-0,6 nach Incropera Tabelle A.11)."),
            T("Reference case: unheated reuse depot in late autumn, 13 degC air, 12 degC surfaces, a 250 W/m2 sun patch through the open roller door on the lid and the fallboard; the iron frame, strings, pin block and action lie in the closed case and are no region; the keys are a hidden region under the closed fallboard.", "Referenzfall: unbeheiztes Gebrauchtwarenlager im Spätherbst, 13 degC Luft, 12 degC Umgebungsflächen, ein Sonnenfleck von 250 W/m2 durch das offene Rolltor auf Deckel und Tastenklappe; Gussplatte, Saiten, Stimmstock und Mechanik liegen im geschlossenen Gehäuse und sind keine Region; die Tasten sind eine verdeckte Region unter der geschlossenen Klappe."),
            T("No element draws power: the case region states 0 W passive explicitly.", "Kein Element nimmt Leistung auf: die Gehäuseregion gibt ausdrücklich 0 W passiv an."),
        ],
    }


def structure(f):
    """🏗️ The structure record: masses, load paths per pose, string tension and tipping."""
    m = f["mass"]
    case_kg = m["case"][0]
    return {
        "gravityMS2": P.G,
        "dynamics": "quasi-static",
        "supports": [],
        "assumptions": [
            T(f"Masses from the 1:50 solids (section x density): case with back, iron frame (7200 kg/m3), strings, pin block, action and keys {case_kg:.1f} kg, lid {m['lid'][0]:.1f} kg, fallboard {m['fallboard'][0]:.1f} kg, three brass pedals {3 * m['pedal-mute'][0]:.1f} kg: piano {f['piano']:.1f} kg (catalogue 210-250 kg); dolly {m['dolly'][0]:.1f} kg, strap {m['strap'][0]:.2f} kg, bench {m['bench'][0]:.1f} kg; total {f['total']:.1f} kg.", f"Massen aus den 1:50-Körpern (Querschnitt x Dichte): Gehäuse mit Rasten, Gussplatte (7200 kg/m3), Saiten, Stimmstock, Mechanik und Tasten {case_kg:.1f} kg, Deckel {m['lid'][0]:.1f} kg, Tastenklappe {m['fallboard'][0]:.1f} kg, drei Messingpedale {3 * m['pedal-mute'][0]:.1f} kg: Klavier {f['piano']:.1f} kg (Katalog 210-250 kg); Roller {m['dolly'][0]:.1f} kg, Gurt {m['strap'][0]:.2f} kg, Bank {m['bench'][0]:.1f} kg; gesamt {f['total']:.1f} kg."),
            T(f"Load paths: stored, the four piano castors stand on the needle felt and the dolly carries everything on its four castors; carried, the back dolly castors and the two movers' grips in the outer back posts (pins at 0.75 m) carry the tipped piano; in use, the four brass castors stand on the floor, the bench carries the seated pianist ({PIANIST_N:.0f} N, EN 12520 seat static load 1300 N as the strength level). The product solves the whole asset as one rigid body on all contacts of a pose, so the shares between the piano, the parked dolly and the bench are the solver's elastic split, not separate equilibria.", f"Lastpfade: gelagert stehen die vier Klavierrollen auf dem Nadelfilz und der Roller trägt alles auf seinen vier Rollen; getragen tragen die hinteren Rollerrollen und die Griffe der beiden Träger in den äußeren Rasten (Gelenkpunkte in 0,75 m) das gekippte Klavier; in Gebrauch stehen die vier Messingrollen auf dem Boden, die Bank trägt den sitzenden Pianisten ({PIANIST_N:.0f} N, Sitz-Standlast 1300 N nach EN 12520 als Festigkeitsstufe). Das Produkt löst das ganze Asset als einen starren Körper auf allen Kontakten einer Pose, die Aufteilung zwischen Klavier, abgestelltem Roller und Bank ist daher die elastische Verteilung des Lösers, keine getrennten Gleichgewichte."),
            T("Internal load path: the strings pull with about 18-20 t on the cast-iron frame (Fletcher and Rossing), which carries that tension in itself and stands with its weight on the back posts and the bottom beam; the case boards (chipboard P2 25 mm, f_m 11 MPa) carry only the self-weight of fronts, lid and keyboard into the side panels, the cheek blocks join keybed and side panels.", "Innerer Lastpfad: die Saiten ziehen mit etwa 18-20 t an der Gussplatte (Fletcher und Rossing), die diesen Zug in sich trägt und mit ihrem Gewicht auf den Rasten und dem Unterbalken steht; die Gehäuseplatten (Spanplatte P2 25 mm, f_m 11 MPa) tragen nur das Eigengewicht von Füllungen, Deckel und Klaviatur in die Seitenwände, die Backen verbinden Klaviaturboden und Seitenwände."),
            T(f"Tipping (EN 14749 as frame): the piano's centre of mass lies {f['comx']:.3f} m from the back and {f['comy']:.2f} m above the castors, the front castors {P.CASTOR_X[1]:.2f} m from the back; a horizontal push of about {f['tip']:.0f} N at the top edge tips it forwards, which is why movers keep it close to the wall side and why it is never left standing on two castors.", f"Kippen (EN 14749 als Rahmen): der Schwerpunkt des Klaviers liegt {f['comx']:.3f} m vor der Rückseite und {f['comy']:.2f} m über den Rollen, die vorderen Rollen {P.CASTOR_X[1]:.2f} m vor der Rückseite; ein waagrechter Stoß von etwa {f['tip']:.0f} N an der Oberkante kippt es nach vorn, deshalb halten Träger es auf der Wandseite und lassen es nie auf zwei Rollen stehen."),
        ],
        "clinical": False,
    }


def rep(sil, ret, mer, omi, thin, hat, sty, ove):
    """🧾 One set of representation decisions."""
    return {"silhouette": sil, "retained": ret, "merged": mer, "omitted": omi, "thinElements": thin, "hatching": hat, "styles": sty, "overlays": ove}


NO_HATCH = T("None; veneer, beech, spruce, brass, felt, webbing and vinyl are told apart by tone.", "Keine; Furnier, Buche, Fichte, Messing, Filz, Gurtband und Kunstleder werden durch Ton unterschieden.")
OVERLAYS = T("Fallboard, lid, pedals and strap are drawn on their own bones and their front and plan shapes are swept from their side profiles, so an opened lid, a raised fallboard, a pressed pedal and the laid-down strap read right in every view; works, keys and pedals are drawn behind the side panel.", "Tastenklappe, Deckel, Pedale und Gurt sind auf eigenen Knochen gezeichnet und ihre Vorder- und Draufsichtformen aus ihren Seitenprofilen gezogen, so lesen ein geöffneter Deckel, eine hochgeklappte Tastenklappe, ein getretenes Pedal und der abgelegte Gurt in jeder Ansicht richtig; Innenleben, Tasten und Pedale liegen hinter der Seitenwand.")

REP_DECISIONS = {
    "50": rep(
        T("Piano 12 x 25 mm in the side view on the 2.3 mm dolly: right side panel full depth below the keyboard with the rounded step at the cheek blocks and the narrow upper side, the lid with its worn front edge, the orange strap band and ratchet over the side, the bench in front. Front: sides, bottom board, knee board with three pedal slots and brass pedals, keybed, cheek blocks, keyslip, fallboard lip, name board, upper panel, lid edge, the strap round it all. Plan: lid with ring marks and the bleached field, cheek tops, name board, fallboard, bench cushion.", "Klavier 12 x 25 mm in der Seitenansicht auf dem 2,3 mm hohen Roller: rechte Seitenwand in voller Tiefe unter der Klaviatur mit gerundeter Stufe an den Backen und schmaler oberer Seite, der Deckel mit abgegriffener Vorderkante, das orange Gurtband mit Ratsche über der Seite, die Bank davor. Vorderansicht: Seiten, Bodenbrett, Untertür mit drei Pedalschlitzen und Messingpedalen, Klaviaturboden, Backen, Vorsatzleiste, Klappenleiste, Namensleiste, Oberfüllung, Deckelkante, der Gurt um alles. Grundriss: Deckel mit Ringen und ausgebleichtem Feld, Backenoberseiten, Namensleiste, Tastenklappe, Bankpolster."),
        T("True board thicknesses (25 mm sides and lid, 20 mm fronts, 18 mm fallboard), the 1.22 m keyboard with 52 naturals and 36 sharps at the true octave pattern, the missing key top on E4, the three brass pedals and castors, the 75 mm dolly castors, the 50 mm webbing at 1.5 mm with the ratchet, the sagging bench cushion, the chipped veneer on the right side panel, scuffs, the water stain, three ring marks, scratches and a burn mark on the lid, note stickers on the name board, the price label, the holes of removed candle holders.", "Wahre Plattenstärken (Seiten und Deckel 25 mm, Füllungen 20 mm, Klappe 18 mm), die 1,22 m breite Klaviatur mit 52 Unter- und 36 Obertasten im wahren Oktavmuster, der fehlende Tastenbelag auf e1, die drei Messingpedale und -rollen, die 75-mm-Rollerrollen, das 50 mm breite und 1,5 mm starke Gurtband mit Ratsche, das durchgesessene Bankpolster, das abgeplatzte Furnier an der rechten Seitenwand, Scheuerstellen, der Wasserfleck, drei Ringe, Kratzer und eine Brandstelle auf dem Deckel, Notenaufkleber auf der Namensleiste, das Preisschild, die Löcher abgeschraubter Kerzenhalter."),
        T("Key gaps of 1 mm are detail lines on one key block; the felt, hammers and dampers of the action are one envelope; the moulded frame of the upper panel is a detail outline.", "Tastenfugen von 1 mm sind Detaillinien auf einem Tastenblock; Filz, Hämmer und Dämpfer der Mechanik sind eine Hüllform; die Profilrahmen der Oberfüllung sind Detailumrisse."),
        T("Screws, hinges except the lid hinge line, the music desk, the tuning pins and the strings drawn singly.", "Schrauben, Scharniere außer der Deckelscharnierlinie, das Notenpult, die Stimmwirbel und einzeln gezeichnete Saiten."),
        T("The 1.5 mm webbing (0.03 mm), the 7 mm felt and the 9 mm soundboard are drawn at true size and read through their outlines; the 1.5 mm fallboard gaps are not drawn.", "Das 1,5 mm starke Gurtband (0,03 mm), der 7-mm-Filz und der 9-mm-Resonanzboden sind in wahrer Größe gezeichnet und wirken durch ihre Konturen; die 1,5-mm-Fugen der Klappe sind nicht gezeichnet."),
        NO_HATCH,
        T("Case veneer 0.32-0.36, bleached lid 0.45, beech 0.34-0.4, keys 0.92, sharps 0.05, brass 0.75-0.8, felt 0.3, plywood 0.55, webbing 0.6, ratchet 0.2, bench 0.35, cushion 0.25; traces of use as 0.13 mm texture strokes.", "Gehäusefurnier 0,32-0,36, ausgebleichter Deckel 0,45, Buche 0,34-0,4, Tasten 0,92, Obertasten 0,05, Messing 0,75-0,8, Filz 0,3, Sperrholz 0,55, Gurtband 0,6, Ratsche 0,2, Bank 0,35, Polster 0,25; Gebrauchsspuren als 0,13-mm-Texturstriche."),
        OVERLAYS,
    ),
    "100": rep(
        T("Piano 6 x 12.5 mm on the dolly: side panel with three-point corners, lid, strap band and ratchet, bench; front with the pedals, cheek blocks, keyslip, fallboard and upper panel; plan with lid, cheeks, fallboard and bench.", "Klavier 6 x 12,5 mm auf dem Roller: Seitenwand mit dreipunktigen Ecken, Deckel, Gurtband und Ratsche, Bank; Vorderansicht mit Pedalen, Backen, Vorsatzleiste, Tastenklappe und Oberfüllung; Grundriss mit Deckel, Backen, Klappe und Bank."),
        T("Board thicknesses, the pedals and pedal slots, castor blocks, the strap and ratchet, two ring marks, the price label, the sharps as lines in plan.", "Plattenstärken, Pedale und Pedalschlitze, Rollenblöcke, Gurt und Ratsche, zwei Ringe, das Preisschild, die Obertasten als Linien im Grundriss."),
        T("Each castor (wheel and housing) is one block; the sharps (10.5 mm) are merged into the key block and drawn as lines; the lid edge rounding, the chip in the side and the cushion sag (12 mm) are below 15 mm and merged.", "Jede Rolle (Rad und Gehäuse) ist ein Block; die Obertasten (10,5 mm) gehen im Tastenblock auf und sind als Linien gezeichnet; die Kantenrundung des Deckels, die Abplatzung der Seite und die Sitzmulde (12 mm) liegen unter 15 mm und gehen auf."),
        T("Key gaps, scratches, scuffs, stickers, candle holder holes, the burn mark, the bleached field outline and all grain lines but one.", "Tastenfugen, Kratzer, Scheuerstellen, Aufkleber, Kerzenhalterlöcher, die Brandstelle, die Umrisslinie des ausgebleichten Felds und alle Maserlinien bis auf eine."),
        T("Webbing, felt and soundboard at true size, read by outline.", "Gurtband, Filz und Resonanzboden in wahrer Größe, über die Kontur lesbar."),
        NO_HATCH,
        T("Same tones as 1:50, body pen 0.25 mm.", "Gleiche Töne wie 1:50, Körperstift 0,25 mm."),
        OVERLAYS,
    ),
    "200": rep(
        T("Piano 3 x 6.3 mm on the dolly: side panel with chamfered corners, the 30 mm lid, strap band and ratchet, the bench as legs, seat block and cushion; front with the sides, the pedal group, keybed, cheek blocks, keyslip, fallboard and upper panel; plan with lid, cheeks and fallboard.", "Klavier 3 x 6,3 mm auf dem Roller: Seitenwand mit gefasten Ecken, der 30-mm-Deckel, Gurtband und Ratsche, die Bank als Beine, Sitzblock und Polster; Vorderansicht mit Seiten, Pedalgruppe, Klaviaturboden, Backen, Vorsatzleiste, Klappe und Oberfüllung; Grundriss mit Deckel, Backen und Klappe."),
        T("The keyboard height, the step of the side panel, the cheek blocks, the knee recess with the pedals, the castors, the strap.", "Die Klaviaturhöhe, die Stufe der Seitenwand, die Backen, die Kniemulde mit den Pedalen, die Rollen, der Gurt."),
        T("Sides, lid, upper panel and knee board are drawn and modelled 30 mm; the three pedals are one group; posts and beams are one back frame; iron frame, strings and pin block are one block; keys and sharps one block; dolly board and felt one 30 mm board; bench rails and seat board one block.", "Seiten, Deckel, Oberfüllung und Untertür sind mit 30 mm gezeichnet und modelliert; die drei Pedale sind eine Gruppe; Rasten und Balken ein Rückenrahmen; Gussplatte, Saiten und Stimmstock ein Block; Tasten und Obertasten ein Block; Rollerplatte und Filz eine 30-mm-Platte; Bankzargen und Sitzbrett ein Block."),
        T("Pedal slots, the leg taper of the bench, all traces of use, ring marks and labels: below a readable line at this scale.", "Pedalschlitze, die Verjüngung der Bankbeine, alle Gebrauchsspuren, Ringe und Schilder: in diesem Maßstab unter einer lesbaren Linie."),
        T("The strap (0.0075 mm) and the soundboard are carried by the pen.", "Der Gurt (0,0075 mm) und der Resonanzboden werden vom Stift getragen."),
        NO_HATCH,
        T("Body pen 0.25 mm, tones as 1:50.", "Körperstift 0,25 mm, Töne wie 1:50."),
        OVERLAYS,
    ),
    "500": rep(
        T("Piano 1.2 x 2.5 mm on the dolly line: the side with its step, the 75 mm lid, the strap as one line; front with the two side-and-cheek blocks, the case between, keyslip, fallboard block and pedal block; plan with lid, side blocks and fallboard; the bench as frame and cushion.", "Klavier 1,2 x 2,5 mm auf der Rollerlinie: die Seite mit ihrer Stufe, der 75-mm-Deckel, der Gurt als eine Linie; Vorderansicht mit den beiden Seiten-und-Backen-Blöcken, dem Gehäuse dazwischen, Vorsatzleiste, Klappenblock und Pedalblock; Grundriss mit Deckel, Seitenblöcken und Klappe; die Bank als Gestell und Polster."),
        T("Overall 1.50 x 0.60 x 1.25 m on a 0.113 m dolly, the step at keyboard height, the knee recess with the pedals, the lid, the fallboard.", "Gesamtmaß 1,50 x 0,60 x 1,25 m auf einem 0,113 m hohen Roller, die Stufe in Klaviaturhöhe, die Kniemulde mit den Pedalen, der Deckel, die Tastenklappe."),
        T("Side panel and cheek block into one 140 mm block per side; fronts, bottom, keybed, keyslip, name board and works into one case mass; the lid takes the top beam in (75 mm); fallboard and keys into one 75 mm block; the pedals into one block; dolly board and castors into one block; bench legs and rails into a frame.", "Seitenwand und Backe in einen 140-mm-Block je Seite; Füllungen, Boden, Klaviaturboden, Vorsatzleiste, Namensleiste und Innenleben in eine Gehäusemasse; der Deckel nimmt den oberen Balken auf (75 mm); Klappe und Tasten in einen 75-mm-Block; die Pedale in einen Block; Rollerplatte und Rollen in einen Block; Bankbeine und Zargen in ein Gestell."),
        T("Castors, ratchet, cheek rounding, pedal slots and all traces of use.", "Rollen, Ratsche, Backenrundung, Pedalschlitze und alle Gebrauchsspuren."),
        T("The strap is a 0.1 mm line, the true 50 mm of the webbing.", "Der Gurt ist eine 0,1-mm-Linie, die wahren 50 mm des Gurtbands."),
        NO_HATCH,
        T("Body pen 0.25 mm, tones as 1:50.", "Körperstift 0,25 mm, Töne wie 1:50."),
        OVERLAYS,
    ),
}

VOL_DECISIONS = {
    "50": [
        T("Body 1:50 is the piano as built: two 25 mm veneered side panels in full depth below the keyboard with the rounded step at the cheek blocks (the right one with a chip of veneer at 0.42 m), beech cheek blocks and keybed, the keyslip, the knee board with three pedal slots, the bottom board on four brass castors, the name board, the upper panel, the lid with its worn front edge, the fallboard with the lip that covers the key fronts; behind: two beams and four spruce posts with the 9 mm soundboard; inside: the 16 mm string plane, the pin block, the cast-iron frame of 15 mm with its four openings, the action envelope, the 1.22 m key block with 36 sharps (five per octave repeated seven times and B flat 0); three brass pedals on a pedal rail. The dolly is a 21 mm plywood board with 7 mm felt on four 75 mm castors, the strap a 50 x 1.5 mm loop under the board and over the lid with its ratchet, the bench four tapered legs, rails, a seat board and a cushion sagging 12 mm.", "Body 1:50 ist das Klavier wie gebaut: zwei 25-mm-Seitenwände, furniert, unter der Klaviatur in voller Tiefe mit gerundeter Stufe an den Backen (die rechte mit abgeplatztem Furnier in 0,42 m Höhe), Backen und Klaviaturboden aus Buche, die Vorsatzleiste, die Untertür mit drei Pedalschlitzen, das Bodenbrett auf vier Messingrollen, die Namensleiste, die Oberfüllung, der Deckel mit abgegriffener Vorderkante, die Tastenklappe mit der Leiste über den Tastenfronten; hinten: zwei Balken und vier Rasten aus Fichte mit dem 9-mm-Resonanzboden; innen: die 16 mm starke Saitenebene, der Stimmstock, die 15 mm starke Gussplatte mit vier Öffnungen, die Hüllform der Mechanik, der 1,22 m breite Tastenblock mit 36 Obertasten (fünf je Oktave siebenmal wiederholt und b0); drei Messingpedale auf einer Pedalleiste. Der Roller ist eine 21-mm-Sperrholzplatte mit 7 mm Filz auf vier 75-mm-Rollen, der Gurt eine 50 x 1,5 mm Schlaufe unter der Platte und über den Deckel mit Ratsche, die Bank vier verjüngte Beine, Zargen, ein Sitzbrett und ein um 12 mm durchgesessenes Polster."),
        T("Thermal 1:50: the case, lid, fallboard, back, brass, dolly, strap and bench regions reference their body solids; the knee board is an own solid without the pedal slots (felt-lined in reality), so the closed case is no outer area; the keys are the hidden region, own solids inside the case under the fallboard; frame, strings, pin block and action belong to no region. Structure 1:50: every board, beam, post, the iron frame and the keys are members of the case bone, the cheek blocks its joints, lid, fallboard, pedals, dolly, strap and bench members of their own bones, mass markers at the computed centres; the supports are the contacts of each pose.", "Thermik 1:50: die Regionen Gehäuse, Deckel, Klappe, Rückseite, Messing, Roller, Gurt und Bank verweisen auf ihre Körper; die Untertür ist ein eigener Körper ohne die Pedalschlitze (in Wirklichkeit filzgefüttert), damit das geschlossene Gehäuse keine Außenfläche ist; die Tasten sind die verdeckte Region, eigene Körper im Gehäuse unter der Klappe; Gussplatte, Saiten, Stimmstock und Mechanik gehören zu keiner Region. Struktur 1:50: jede Platte, jeder Balken, jede Raste, die Gussplatte und die Tasten sind Glieder des Gehäuseknochens, die Backen seine Gelenke, Deckel, Klappe, Pedale, Roller, Gurt und Bank Glieder ihrer eigenen Knochen, Massenmarken in den berechneten Schwerpunkten; die Auflager sind die Kontakte jeder Pose."),
    ],
    "100": [
        T("Body 1:100 keeps the true boards, the posts, the iron frame with its openings and the three pedals with simplified treads; the side corners have three points, the chip and the lid edge rounding are below 15 mm and merged, the 36 sharps are merged into the key block, each castor of piano and dolly is one block, the cushion is flat.", "Body 1:100 behält die wahren Platten, die Rasten, die Gussplatte mit ihren Öffnungen und die drei Pedale mit vereinfachten Trittflächen; die Ecken der Seiten haben drei Punkte, Abplatzung und Kantenrundung des Deckels liegen unter 15 mm und gehen auf, die 36 Obertasten gehen im Tastenblock auf, jede Rolle von Klavier und Roller ist ein Block, das Polster ist eben."),
        T("Thermal 1:100 as 1:50 without the sharps; structure 1:100: boards, frame, keys and pedals as members, cheek blocks as joints, mass markers.", "Thermik 1:100 wie 1:50 ohne die Obertasten; Struktur 1:100: Platten, Gussplatte, Tasten und Pedale als Glieder, Backen als Gelenke, Massenmarken."),
    ],
    "200": [
        T("Body 1:200: side panels, lid, upper panel and knee board are 30 mm (one smallest feature), the side corners chamfered; posts and beams are one back frame with three openings, frame, strings and pin block one 31 mm works block, keys and sharps one block, the three pedals one 30 mm group block on the middle pedal; the dolly is one 30 mm board on four castor blocks; the bench keeps its four legs, the rails and the seat board are one block under the cushion.", "Body 1:200: Seitenwände, Deckel, Oberfüllung und Untertür sind 30 mm stark (ein kleinstes Merkmal), die Ecken der Seiten gefast; Rasten und Balken sind ein Rückenrahmen mit drei Öffnungen, Gussplatte, Saiten und Stimmstock ein 31-mm-Block, Tasten und Obertasten ein Block, die drei Pedale ein 30-mm-Gruppenblock am mittleren Pedal; der Roller ist eine 30-mm-Platte auf vier Rollenblöcken; die Bank behält ihre vier Beine, Zargen und Sitzbrett sind ein Block unter dem Polster."),
        T("Thermal 1:200 references the boards, the back frame, the pedal group and the bench, the knee board and the keys are own solids; structure 1:200: boards, back frame, works and keys as case members, cheek blocks as joints, mass markers.", "Thermik 1:200 verweist auf Platten, Rückenrahmen, Pedalgruppe und Bank, Untertür und Tasten sind eigene Körper; Struktur 1:200: Platten, Rückenrahmen, Innenleben und Tasten als Gehäuseglieder, Backen als Gelenke, Massenmarken."),
    ],
    "500": [
        T("Body 1:500: each side panel with its cheek block is one 140 mm block with the full side profile, the case between them one mass (fronts, bottom, keybed, keyslip, name board and works), the back one 90 mm block, the lid 75 mm, fallboard and keys one 75 mm block on its hinge, the pedals one block in the knee recess; the dolly is one block, the strap four slender bands at their true 50 mm, the bench a frame of two leg panels and a rail with the cushion; castors and ratchet are below 75 mm.", "Body 1:500: jede Seitenwand mit ihrer Backe ist ein 140-mm-Block mit dem vollen Seitenprofil, das Gehäuse dazwischen eine Masse (Füllungen, Boden, Klaviaturboden, Vorsatzleiste, Namensleiste und Innenleben), die Rückseite ein 90-mm-Block, der Deckel 75 mm, Klappe und Tasten ein 75-mm-Block am Scharnier, die Pedale ein Block in der Kniemulde; der Roller ist ein Block, der Gurt vier schlanke Bänder in ihren wahren 50 mm, die Bank ein Gestell aus zwei Beinplatten und einer Zarge mit dem Polster; Rollen und Ratsche liegen unter 75 mm."),
        T("Thermal 1:500 references the blocks with their regions, the keys are an own hidden solid inside the case mass; structure 1:500: the blocks as members of their bones, mass markers.", "Thermik 1:500 verweist auf die Blöcke mit ihren Regionen, die Tasten sind ein eigener verdeckter Körper in der Gehäusemasse; Struktur 1:500: die Blöcke als Glieder ihrer Knochen, Massenmarken."),
    ],
}
