"""📝️ Bilingual texts, breathing patterns and clips of `surveyor-with-total-station` (English first, German second)."""
from lib import tx

TITLE = tx("Surveyor with Total Station", "Vermessende Person mit Tachymeter")
SUMMARY = tx(
    "A surveyor of 1.66 m and 63 kg in hi-vis vest, hard hat and safety boots stands behind a total station on an aluminium tripod and sights a target through the eyepiece, a hand on each drive knob; she straightens to read the display and waits with a hand on the carrying handle. Light standing work of about 1.5 to 1.8 met: roughly 160 to 190 W of body heat plus the 7 W nameplate of the battery-powered instrument.",
    "Eine vermessende Person von 1,66 m und 63 kg mit Warnweste, Schutzhelm und Sicherheitsstiefeln steht hinter einem Tachymeter auf einem Aluminiumstativ und visiert durch das Okular ein Ziel an, je eine Hand an einem Feintrieb; zum Ablesen der Anzeige richtet sie sich auf und wartet mit einer Hand am Tragegriff. Leichte Arbeit im Stehen von etwa 1,5 bis 1,8 met: rund 160 bis 190 W Körperwärme und die 7 W Nennleistung des akkubetriebenen Instruments.",
)
TAGS = ["human", "adult", "surveyor", "construction", "total-station", "tripod", "measuring", "hi-vis", "hard-hat"]
NOTES = tx(
    "Rig, outlines and volumes derived from the archive figure worker-carrying-box (round 5, stature 1.741 m) without carton, knee pads and cargo pockets, scaled uniformly by 0.953 to a stature of 1.66 m (DIN 33402-2 and ANSUR II women, about the 70th percentile) with de Leva's female segment masses for 63 kg. Tripod (aluminium, head at 1.32 m, shoes on a 0.52 m circle, 5.0 kg) and total station (5.3 kg, trunnion axis 0.19 m above the tribrach) follow ISO 12858-2 and catalogue instruments; they are fixed bones placed from the pelvis, the legs stand alike in every pose, so the three tripod shoes stay on the ground. Trunk, neck, head and arms are solved so that the right eye meets the eyepiece and the fingers the drive knobs and keyboard (HW-04 ik.ts on the product's own chain).",
    "Skelett, Umrisse und Volumen abgeleitet von der Archivfigur worker-carrying-box (Runde 5, Körperhöhe 1,741 m) ohne Karton, Knieschoner und Beintaschen, gleichmäßig um 0,953 auf eine Körperhöhe von 1,66 m skaliert (DIN 33402-2 und ANSUR II Frauen, etwa 70. Perzentil) mit den weiblichen Segmentmassen nach de Leva für 63 kg. Stativ (Aluminium, Kopf auf 1,32 m, Schuhe auf einem Kreis von 0,52 m, 5,0 kg) und Tachymeter (5,3 kg, Kippachse 0,19 m über dem Dreifuß) folgen ISO 12858-2 und Katalogeräten; sie sind feste, vom Becken aus platzierte Knochen, die Beine stehen in jeder Pose gleich, sodass die drei Stativschuhe am Boden bleiben. Rumpf, Hals, Kopf und Arme sind so gelöst, dass das rechte Auge das Okular und die Finger Feintriebe und Tastatur treffen (HW-04 ik.ts auf der Kette des Produkts).",
)
REFS = [
    {"id": "iso-12858-2", "citation": "ISO 12858-2:2020 Optics and optical instruments, ancillary devices for geodetic instruments, part 2: tripods (head height, leg spread, stability)."},
    {"id": "iso-17123-3", "citation": "ISO 17123-3:2001 Optics and optical instruments, field procedures for testing geodetic and surveying instruments, part 3: theodolites (total stations as tested instruments; typical instrument mass 5 to 6 kg, battery 7.4 V about 3 to 7 W)."},
    {"id": "iso-7730-2005", "citation": "ISO 7730:2005 Ergonomics of the thermal environment, analytical determination of thermal comfort (metabolic rates, Annex B)."},
]
PATTERNS = [
    {"id": "rest", "title": tx("Calm standing breathing", "Ruhige Atmung im Stand"), "rateBpm": 14, "tidalVolumeL": 0.5, "inhaleFraction": 0.38, "pauseAfterInhaleFraction": 0.04, "exhaleFraction": 0.45, "pauseAfterExhaleFraction": 0.13, "route": "nose", "source": "authored-estimate", "reference": "gupta-2010"},
    {"id": "light-work", "title": tx("Light standing work", "Leichte Arbeit im Stand"), "rateBpm": 15, "tidalVolumeL": 0.6, "inhaleFraction": 0.4, "pauseAfterInhaleFraction": 0.03, "exhaleFraction": 0.47, "pauseAfterExhaleFraction": 0.1, "route": "nose", "source": "authored-estimate", "reference": "gupta-2010"},
    {"id": "concentrating", "title": tx("Held breath while sighting", "Angehaltener Atem beim Anzielen"), "rateBpm": 11, "tidalVolumeL": 0.55, "inhaleFraction": 0.3, "pauseAfterInhaleFraction": 0.15, "exhaleFraction": 0.4, "pauseAfterExhaleFraction": 0.15, "route": "nose", "source": "authored-estimate", "reference": "gupta-2010"},
]


def thermal_assumptions(area):
    return [
        tx("Body regions sum to the DuBois area of %.3f m2 (63 kg, 166 cm); clothed regions keep the area factors of the base figure. Basal metabolism after Mifflin-St Jeor for a woman of 31." % area, "Die Körperregionen ergeben die DuBois-Fläche von %s m2 (63 kg, 166 cm); bekleidete Regionen behalten die Flächenfaktoren der Ausgangsfigur. Grundumsatz nach Mifflin-St Jeor für eine 31-jährige Frau." % ("%.3f" % area).replace(".", ",")),
        tx("Clothing is layered per region as in the base figure (T-shirt, work shirt, mesh vest; briefs, work trousers; socks, leather boot; hair, air gap and HDPE shell in the helmet) with garment values guided by ISO 9920.", "Die Kleidung ist je Region wie bei der Ausgangsfigur geschichtet (T-Shirt, Arbeitshemd, Netzweste; Unterhose, Arbeitshose; Socken, Lederstiefel; Haar, Luftspalt und HDPE-Schale im Helm) mit Werten nach Anhalt von ISO 9920."),
        tx("Metabolic rates: standing at ease 1.5 met, light standing work at the instrument 1.7 to 1.8 met (ISO 7730 Annex B, standing light activity 1.6 met; Compendium of Physical Activities, surveying in the field 2.0 to 2.5 met while walking between stations).", "Stoffwechselraten: entspanntes Stehen 1,5 met, leichte Arbeit am Instrument im Stehen 1,7 bis 1,8 met (ISO 7730 Anhang B, leichte Tätigkeit im Stehen 1,6 met; Compendium of Physical Activities, Vermessung im Gelände 2,0 bis 2,5 met beim Gehen zwischen den Standpunkten)."),
        tx("The reference case is a cool, breezy site in spring (12 C, 70 % relative humidity, 2 m/s wind, 120 W/m2 mostly diffuse light). The instrument draws its 7 W nameplate from its battery while measuring with distance meter and motor drives and releases about 6 W at its housing; the tripod is a passive aluminium surface.", "Der Referenzfall ist eine kühle, windige Baustelle im Frühling (12 C, 70 % relative Feuchte, 2 m/s Wind, 120 W/m2 überwiegend diffuses Licht). Das Instrument zieht beim Messen mit Distanzmesser und Motorantrieben seine 7 W Nennleistung aus dem Akku und gibt etwa 6 W am Gehäuse ab; das Stativ ist eine passive Aluminiumfläche."),
    ]


STRUCTURE_ASSUMPTIONS = [
    tx("Segments are point masses at the de Leva centres of mass (female table); rotational inertia is neglected.", "Segmente sind Punktmassen in den Schwerpunkten nach de Leva (Frauentabelle); Drehträgheit wird vernachlässigt."),
    tx("Tripod (5.0 kg) and instrument (5.3 kg) stand on three ground contacts; in the rigid-segment tree they hang on fixed bones placed from the pelvis, so the equal-stiffness distribution lets the tripod shoes and the boots share the total weight. The fingers on the drive knobs press with less than 5 N and are not modelled as loads.", "Stativ (5,0 kg) und Instrument (5,3 kg) stehen auf drei Bodenkontakten; im starren Segmentbaum hängen sie an festen, vom Becken aus platzierten Knochen, sodass die Verteilung gleicher Steifigkeit Stativschuhe und Stiefel das Gesamtgewicht teilen lässt. Die Finger an den Feintrieben drücken mit weniger als 5 N und sind nicht als Lasten modelliert."),
]
POSES = {
    "sighting": (tx("Sighting through the eyepiece", "Durch das Okular anzielen"), tx("Bent forward from the hips over the instrument, the right eye at the eyepiece, the right fingers on the vertical drive knob and the left fingers on the horizontal drive below; feet apart between the rear tripod legs.", "Aus der Hüfte über das Instrument gebeugt, das rechte Auge am Okular, die rechten Finger am vertikalen Feintrieb und die linken Finger am horizontalen Feintrieb darunter; die Füße zwischen den hinteren Stativbeinen.")),
    "reading-display": (tx("Reading the display", "Anzeige ablesen"), tx("Half upright, head bowed to the display on the back of the instrument, the right index finger on the keyboard, the left arm hanging.", "Halb aufgerichtet, den Kopf zur Anzeige an der Rückseite des Instruments geneigt, der rechte Zeigefinger auf der Tastatur, der linke Arm hängt.")),
    "standing-by": (tx("Standing by with a hand on the handle", "Bereitstehen mit der Hand am Griff"), tx("Upright behind the tripod, the right hand resting on the carrying handle of the instrument, the left arm hanging, waiting for the rod person to move to the next point.", "Aufrecht hinter dem Stativ, die rechte Hand auf dem Tragegriff des Instruments, der linke Arm hängt, wartend, bis die Person mit dem Reflektorstab zum nächsten Punkt geht.")),
}


def clips(lib, contacts, sol):
    d = 8.0
    whole = lib.span(contacts, 0, d)
    measuring = {
        "id": "measuring-cycle", "title": tx("Measuring a point", "Einen Punkt messen"),
        "description": tx("An 8-second loop: from standing by she bends to the eyepiece, holds the sight while turning the drive knobs, straightens to read and key in the reading and returns to standing by; feet and tripod stay planted.", "Eine Schleife von 8 Sekunden: Aus dem Bereitstehen beugt sie sich zum Okular, hält die Zielung und dreht dabei die Feintriebe, richtet sich zum Ablesen und Eintippen auf und kehrt ins Bereitstehen zurück; Füße und Stativ bleiben stehen."),
        "durationS": d, "fps": 30, "loop": True, "rootMotion": "none",
        "keys": [lib.key(0, "standing-by"), lib.key(1.6, "sighting"), lib.key(3.6, "sighting", {"forearm-r": {**sol["sighting"]["bones"]["forearm-r"], "twistDeg": sol["sighting"]["bones"]["forearm-r"].get("twistDeg", 0) + 6}}), lib.key(4.4, "sighting"), lib.key(5.6, "reading-display"), lib.key(6.8, "reading-display"), lib.key(d, "standing-by")],
        "contacts": whole,
        "phases": [{"id": "approach", "fromS": 0, "toS": 1.6, "title": tx("Bending to the eyepiece", "Zum Okular beugen")}, {"id": "sight", "fromS": 1.6, "toS": 4.4, "title": tx("Sighting and fine pointing", "Anzielen und Feineinstellen")}, {"id": "read", "fromS": 4.4, "toS": 6.8, "title": tx("Reading and keying in", "Ablesen und Eintippen")}, {"id": "return", "fromS": 6.8, "toS": d, "title": tx("Standing by", "Bereitstehen")}],
        "events": [{"t": 4.4, "kind": "operating", "title": tx("Distance measurement triggered", "Distanzmessung ausgelöst"), "target": "instrument"}],
        "tracks": [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 1.5}, {"t": 1.6, "v": 1.8}, {"t": 5.6, "v": 1.7}, {"t": d, "v": 1.5}]}, {"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 1.6, "v": 2}, {"t": 4.4, "v": 1}, {"t": 6.8, "v": 0}]}, {"target": "operating:instrument", "ease": "step", "keys": [{"t": 0, "v": 0.5}, {"t": 4.4, "v": 1}, {"t": 6.8, "v": 0.5}]}],
    }
    idle = {
        "id": "idle-breathing", "title": tx("Idle breathing", "Ruhiges Atmen"),
        "description": tx("A five-second loop standing by: the chest rises and falls with calm breathing, the head and the free arm settle in a slow sway; the hand stays on the handle.", "Eine Schleife von fünf Sekunden im Bereitstehen: Die Brust hebt und senkt sich mit ruhigem Atmen, Kopf und freier Arm wiegen sich langsam; die Hand bleibt am Griff."),
        "durationS": 5, "fps": 30, "loop": True, "rootMotion": "none",
        "keys": [lib.key(0, "standing-by"), lib.key(1.25, "standing-by", {"torso": {"angleDeg": -0.6}, "head": {"angleDeg": 2.2}, "upper-arm-l": {"angleDeg": 6, "abductionDeg": 3}}), lib.key(2.5, "standing-by", {"torso": {"angleDeg": 0.4}, "head": {"angleDeg": 0.6}, "forearm-l": {"angleDeg": 15}}), lib.key(3.75, "standing-by", {"torso": {"angleDeg": -0.5}, "head": {"angleDeg": 1.8}, "upper-arm-l": {"angleDeg": 2, "abductionDeg": 3}}), lib.key(5, "standing-by")],
        "contacts": lib.span(contacts, 0, 5),
        "phases": [{"id": "idle", "fromS": 0, "toS": 5, "title": tx("Standing by", "Bereitstehen")}],
        "events": [],
        "tracks": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}]}, {"target": "operating:instrument", "ease": "step", "keys": [{"t": 0, "v": 0.5}]}],
    }
    return [measuring, idle]


DECISIONS = {
    "50": {"silhouette": tx("The aluminium tripod (head, three telescopic legs with clamps and shoes) and the total station (tribrach, alidade with display, two standards, telescope with eyepiece, drive knob and carrying handle) stand in front of her; legs are revolved, the instrument parts extruded profiles of their fixed bones.", "Das Aluminiumstativ (Kopf, drei Teleskopbeine mit Klemmen und Schuhen) und das Tachymeter (Dreifuß, Alhidade mit Anzeige, zwei Stützen, Fernrohr mit Okular, Feintrieb und Tragegriff) stehen vor ihr; die Beine sind gedreht, die Instrumententeile extrudierte Profile ihrer festen Knochen."),
           "retained": tx("tripod head, upper and lower leg tubes and shoes, eyepiece, drive knob, display, handle", "Stativkopf, obere und untere Beinrohre und Schuhe, Okular, Feintrieb, Anzeige, Griff"),
           "omitted": tx("knee pads and cargo pockets (she wears plain work trousers), leg clamps, keyboard keys, objective lens", "Knieschoner und Beintaschen (sie trägt einfache Arbeitshosen), Beinklemmen, Tastaturtasten, Objektivlinse"),
           "thinElements": tx("lower leg tubes at their true 28 mm", "untere Beinrohre in ihren wahren 28 mm")},
    "100": {"silhouette": tx("Tripod head, upper and lower leg tubes and shoes; the instrument as tribrach, alidade, standards and telescope.", "Stativkopf, obere und untere Beinrohre und Schuhe; das Instrument als Dreifuß, Alhidade, Stützen und Fernrohr."),
            "merged": tx("eyepiece and drive knob into the telescope; display into the alidade", "Okular und Feintrieb in das Fernrohr; Anzeige in die Alhidade"),
            "omitted": tx("knee pads and cargo pockets (plain trousers), handle", "Knieschoner und Beintaschen (einfache Hose), Griff")},
    "200": {"silhouette": tx("Tripod as head and three legs of one section; the instrument as one body with the telescope across.", "Stativ als Kopf und drei Beine mit einem Querschnitt; das Instrument als ein Körper mit quer liegendem Fernrohr."),
            "merged": tx("upper and lower leg tubes into one leg; tribrach, alidade and standards into one instrument body", "obere und untere Beinrohre in ein Bein; Dreifuß, Alhidade und Stützen in einen Instrumentenkörper"),
            "thinElements": tx("tripod legs drawn 40 mm wide (0.2 mm), their real 28 to 34 mm thickened within the outline tolerance", "Stativbeine 40 mm breit gezeichnet (0,2 mm), ihre realen 28 bis 34 mm innerhalb der Umrisstoleranz verdickt")},
    "500": {"silhouette": tx("Tripod as three leg bars under a head block, the instrument as one block: the tripod reads at a glance in front of the figure.", "Stativ als drei Beinstäbe unter einem Kopfblock, das Instrument als ein Block: Das Stativ ist vor der Figur auf einen Blick lesbar."),
            "merged": tx("whole instrument with telescope into one block", "das ganze Instrument mit Fernrohr in einen Block"),
            "thinElements": tx("tripod legs drawn 70 mm wide (0.14 mm with the outline), their real 30 mm thickened", "Stativbeine 70 mm breit gezeichnet (0,14 mm mit dem Umriss), ihre realen 30 mm verdickt")},
}
VOLUME = {
    "50": tx("Tripod and total station 1:50: tripod head disc, three legs as an upper tube of 44 mm and a slender lower tube of 28 mm with a steel shoe on its own fixed bone each (pointed into the ground on a 0.52 m circle); tribrach, alidade with display, two standards, telescope with eyepiece, drive knob and carrying handle as extruded blocks. Thermal: tripod and instrument regions; structure: one member per tripod bone and the instrument.", "Stativ und Tachymeter 1:50: Stativkopfscheibe, drei Beine aus einem oberen Rohr von 44 mm und einem schlanken unteren Rohr von 28 mm mit Stahlschuh auf je einem eigenen festen Knochen (auf einem Kreis von 0,52 m in den Boden gesteckt); Dreifuß, Alhidade mit Anzeige, zwei Stützen, Fernrohr mit Okular, Feintrieb und Tragegriff als extrudierte Blöcke. Thermik: Regionen Stativ und Instrument; Tragwerk: ein Glied je Stativknochen und das Instrument."),
    "100": tx("Tripod and total station 1:100: head disc, upper and lower leg tubes with shoes; tribrach, alidade, standards and telescope as blocks for a 15 mm feature.", "Stativ und Tachymeter 1:100: Kopfscheibe, obere und untere Beinrohre mit Schuhen; Dreifuß, Alhidade, Stützen und Fernrohr als Blöcke für 15 mm Mindestmaß."),
    "200": tx("Tripod and total station 1:200: head and three slender legs of 40 mm; one instrument block with the telescope across.", "Stativ und Tachymeter 1:200: Kopf und drei schlanke Beine von 40 mm; ein Instrumentenblock mit quer liegendem Fernrohr."),
    "500": tx("Tripod and total station 1:500: head and three slender leg bars; the instrument as one block of 0.17 x 0.23 x 0.30 m.", "Stativ und Tachymeter 1:500: Kopf und drei schlanke Beinstäbe; das Instrument als ein Block von 0,17 x 0,23 x 0,30 m."),
}
