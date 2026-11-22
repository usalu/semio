"""📝️ Bilingual texts of `construction-worker-with-shovel` (English first, German second)."""
from lib import tx

TITLE = tx("Construction Worker with Shovel", "Bauarbeitende Person mit Schaufel")
SUMMARY = tx(
    "A construction worker of 1.72 m and 78 kg in hi-vis vest, hard hat, leather gloves and safety boots digs a trench with a 1.21 m round-point shovel with D-grip: driving the blade into the soil, prying it loose, lifting the load and resting on the planted shovel. Digging is heavy work of about 5 to 6 met, so the body releases some 450 to 550 W while the passive shovel adds nothing to the power.",
    "Eine Bauarbeiterin bzw. ein Bauarbeiter von 1,72 m und 78 kg mit Warnweste, Schutzhelm, Lederhandschuhen und Sicherheitsstiefeln hebt mit einer 1,21 m langen Spitzschaufel mit D-Griff einen Graben aus: Blatt in den Boden stoßen, Erde lösen, die Last anheben und auf die eingesteckte Schaufel gestützt ausruhen. Graben ist Schwerarbeit von etwa 5 bis 6 met, der Körper gibt dabei rund 450 bis 550 W ab, die passive Schaufel trägt nichts zur Leistung bei.",
)
TAGS = ["human", "worker", "construction", "digging", "shovel", "hi-vis", "hard-hat", "safety-boots", "earthwork"]
NOTES = tx(
    "Rig, outlines and volumes derived from the archive figure worker-carrying-box (round 5, stature 1.741 m) without its carton, scaled uniformly by 0.988 to a stature of 1.72 m (DIN 33402-2 and ANSUR II male near the 40th percentile) with de Leva's male segment masses for 78 kg. The shovel (DIN 20127 type round-point shovel with D-grip: ash shaft 38 mm, blade 0.235 m wide and 0.27 m long, 2.4 kg) is stated per scale as revolved and extruded parts that are drawn and modelled from the same profile. In every pose the arms and the shovel joint are solved onto the D-grip in the right fist, the shaft in the left fist and the blade tip on the ground (HW-04 ik.ts on the product's own chain).",
    "Skelett, Umrisse und Volumen abgeleitet von der Archivfigur worker-carrying-box (Runde 5, Körperhöhe 1,741 m) ohne Karton, gleichmäßig um 0,988 auf eine Körperhöhe von 1,72 m skaliert (DIN 33402-2 und ANSUR II Männer, etwa 40. Perzentil) mit den männlichen Segmentmassen nach de Leva für 78 kg. Die Schaufel (Spitzschaufel mit D-Griff nach DIN 20127: Eschenstiel 38 mm, Blatt 0,235 m breit und 0,27 m lang, 2,4 kg) ist je Maßstab aus gedrehten und extrudierten Teilen beschrieben, die aus demselben Profil gezeichnet und modelliert sind. In jeder Pose sind Arme und Schaufelgelenk auf den D-Griff in der rechten Faust, den Stiel in der linken Faust und die Blattspitze am Boden gelöst (HW-04 ik.ts auf der Kette des Produkts).",
)
REFS = [
    {"id": "din-20127", "citation": "DIN 20127 (draft series DIN 20120 to 20128): Schaufeln und Spaten, Begriffe und Maße (round-point shovel with D-grip, blade about 0.23 x 0.28 m, mass about 2 to 2.5 kg)."},
    {"id": "iso-7730-2005", "citation": "ISO 7730:2005 Ergonomics of the thermal environment, analytical determination of thermal comfort (metabolic rates, Annex B)."},
]
PATTERNS = [
    {"id": "rest", "title": tx("Calm standing breathing", "Ruhige Atmung im Stand"), "rateBpm": 15, "tidalVolumeL": 0.65, "inhaleFraction": 0.38, "pauseAfterInhaleFraction": 0.04, "exhaleFraction": 0.45, "pauseAfterExhaleFraction": 0.13, "route": "nose", "source": "authored-estimate", "reference": "gupta-2010"},
    {"id": "digging", "title": tx("Digging, heavy rhythmic work", "Graben, schwere rhythmische Arbeit"), "rateBpm": 24, "tidalVolumeL": 1.6, "inhaleFraction": 0.43, "pauseAfterInhaleFraction": 0.01, "exhaleFraction": 0.52, "pauseAfterExhaleFraction": 0.04, "route": "nose-and-mouth", "source": "authored-estimate", "reference": "gupta-2010"},
    {"id": "lifting", "title": tx("Lifting the load, breath pressed", "Last anheben, Atem gepresst"), "rateBpm": 28, "tidalVolumeL": 1.7, "inhaleFraction": 0.46, "pauseAfterInhaleFraction": 0.0, "exhaleFraction": 0.54, "pauseAfterExhaleFraction": 0.0, "route": "nose-and-mouth", "source": "authored-estimate", "reference": "gupta-2010"},
    {"id": "recovering", "title": tx("Catching breath on the shovel", "Verschnaufen auf der Schaufel"), "rateBpm": 20, "tidalVolumeL": 1.1, "inhaleFraction": 0.42, "pauseAfterInhaleFraction": 0.02, "exhaleFraction": 0.5, "pauseAfterExhaleFraction": 0.06, "route": "nose-and-mouth", "source": "authored-estimate", "reference": "gupta-2010"},
]


def thermal_assumptions(area):
    return [
        tx("Body regions sum to the DuBois area of %.3f m2 (78 kg, 172 cm); clothed regions keep the area factors of the base figure. Basal metabolism after Mifflin-St Jeor for a man of 42." % area, "Die Körperregionen ergeben die DuBois-Fläche von %s m2 (78 kg, 172 cm); bekleidete Regionen behalten die Flächenfaktoren der Ausgangsfigur. Grundumsatz nach Mifflin-St Jeor für einen 42-jährigen Mann." % ("%.3f" % area).replace(".", ",")),
        tx("Clothing is layered per region (T-shirt, work shirt, mesh vest; briefs, work trousers; socks, leather boot; hair, air gap and HDPE shell in the helmet) with garment values guided by ISO 9920.", "Die Kleidung ist je Region geschichtet (T-Shirt, Arbeitshemd, Netzweste; Unterhose, Arbeitshose; Socken, Lederstiefel; Haar, Luftspalt und HDPE-Schale im Helm) mit Werten nach Anhalt von ISO 9920."),
        tx("Metabolic rates follow the Compendium of Physical Activities (digging, spading 5.0 met; shovelling heavy loads 6 to 7 met; resting on the tool 2.0 met while recovering) and ISO 8996 class 3 (high metabolic rate, shovelling); lifting the soil does about 40 W of external work.", "Die Stoffwechselraten folgen dem Compendium of Physical Activities (Graben, Umstechen 5,0 met; schwere Lasten schaufeln 6 bis 7 met; auf das Werkzeug gestützt verschnaufen 2,0 met) und ISO 8996 Klasse 3 (hoher Umsatz, Schaufeln); das Anheben der Erde leistet etwa 40 W äußere Arbeit."),
        tx("The reference case is an outdoor trench on a mild overcast day (15 C, 60 % relative humidity, 1 m/s wind, diffuse light of 150 W/m2); shaft and blade are passive surfaces at ambient temperature.", "Der Referenzfall ist ein Graben im Freien an einem milden bedeckten Tag (15 C, 60 % relative Feuchte, 1 m/s Wind, diffuses Licht von 150 W/m2); Stiel und Blatt sind passive Oberflächen auf Umgebungstemperatur."),
    ]


STRUCTURE_ASSUMPTIONS = [
    tx("Segments are point masses at the de Leva centres of mass (male table); rotational inertia is neglected.", "Segmente sind Punktmassen in den Schwerpunkten nach de Leva (Männertabelle); Drehträgheit wird vernachlässigt."),
    tx("The 2.4 kg shovel is carried by the right fist on the D-grip; the left fist on the shaft closes the loop only geometrically, so in the rigid-segment tree its share runs through the right arm.", "Die 2,4-kg-Schaufel wird von der rechten Faust am D-Griff getragen; die linke Faust am Stiel schließt die Kette nur geometrisch, im starren Segmentbaum läuft ihr Anteil daher über den rechten Arm."),
    tx("While the blade tip stands in the soil it is a ground contact with friction 0.7; the soil reaction then shares the load with both feet under the equal-stiffness assumption.", "Solange die Blattspitze im Boden steht, ist sie ein Bodenkontakt mit Reibung 0,7; die Bodenreaktion teilt sich die Last dann unter der Annahme gleicher Steifigkeit mit beiden Füßen."),
    tx("Lifting the soil is a carried load of 7 kg (about 4.5 litres of moist soil, 1.6 t/m3) at the blade centre, not drawn as a separate body.", "Das Anheben der Erde ist eine getragene Last von 7 kg (etwa 4,5 Liter feuchte Erde, 1,6 t/m3) in der Blattmitte, nicht als eigener Körper gezeichnet."),
]
POSES = {
    "driving-blade": (tx("Driving the blade in", "Blatt einstechen"), tx("Left foot forward, hips and trunk bent over the shovel, the right fist on the D-grip at waist height pushes the shaft down at 62 degrees while the left fist guides it near the middle; the blade tip has just entered the soil.", "Linker Fuß vorn, Hüfte und Rumpf über die Schaufel gebeugt, die rechte Faust am D-Griff auf Hüfthöhe drückt den Stiel unter 62 Grad nach unten, die linke Faust führt ihn etwa in der Mitte; die Blattspitze ist gerade in den Boden eingedrungen.")),
    "prying-soil": (tx("Prying the soil loose", "Erde lösen"), tx("The blade stays in the soil while the right fist pulls the D-grip down and back to the thigh, the shaft levering over the left fist at 48 degrees; knees bent, weight on the front leg.", "Das Blatt bleibt im Boden, während die rechte Faust den D-Griff nach unten und hinten zum Oberschenkel zieht und der Stiel unter 48 Grad über der linken Faust hebelt; Knie gebeugt, Gewicht auf dem vorderen Bein.")),
    "lifting-soil": (tx("Lifting the load", "Last anheben"), tx("Knees bent and back straight, the loaded shovel is raised almost level at hip height, the left fist close to the blade carries most of the soil, the right fist holds the D-grip at the hip.", "Knie gebeugt und Rücken gerade, die beladene Schaufel wird fast waagerecht auf Hüfthöhe angehoben, die linke Faust nahe am Blatt trägt den größten Teil der Erde, die rechte Faust hält den D-Griff an der Hüfte.")),
    "resting-on-shovel": (tx("Resting on the shovel", "Auf die Schaufel gestützt"), tx("Standing upright, the shovel planted almost vertically just outside the right foot, both fists stacked on the D-grip; a breather between two loads.", "Aufrecht stehend, die Schaufel fast senkrecht knapp außen neben dem rechten Fuß eingesteckt, beide Fäuste übereinander auf dem D-Griff; eine Atempause zwischen zwei Ladungen.")),
}


def clips(lib, contacts, sol):
    d = 3.6
    tip = "blade-tip"
    dig = {
        "id": "dig-cycle", "title": tx("Digging cycle", "Grabzyklus"),
        "description": tx("A 3.6-second loop of one shovelful: drive the blade in, pry the soil loose, lift the loaded shovel and swing it back to the start; the feet stay planted, the blade tip touches the ground while it is in the soil.", "Eine Schleife von 3,6 Sekunden für eine Schaufel voll: Blatt einstechen, Erde lösen, die beladene Schaufel anheben und zurück zum Anfang schwingen; die Füße bleiben stehen, die Blattspitze berührt den Boden, solange sie in der Erde steckt."),
        "durationS": d, "fps": 30, "loop": True, "rootMotion": "none",
        "keys": [lib.key(0, "driving-blade"), lib.key(0.8, "driving-blade"), lib.key(1.6, "prying-soil"), lib.key(2.6, "lifting-soil"), lib.key(d, "driving-blade")],
        "contacts": lib.span([c for c in contacts if c != tip], 0, d) + [{"anchor": tip, "fromS": 0, "toS": 1.9}, {"anchor": tip, "fromS": 3.4, "toS": d}],
        "phases": [{"id": "drive", "fromS": 0, "toS": 0.8, "title": tx("Driving the blade in", "Blatt einstechen")}, {"id": "pry", "fromS": 0.8, "toS": 1.6, "title": tx("Prying", "Lösen")}, {"id": "lift", "fromS": 1.6, "toS": 2.6, "title": tx("Lifting the load", "Last anheben")}, {"id": "return", "fromS": 2.6, "toS": d, "title": tx("Tossing and returning", "Abwerfen und zurück")}],
        "events": [{"t": 1.9, "kind": "contact-off", "title": tx("Blade leaves the soil", "Blatt verlässt den Boden"), "target": tip}, {"t": 1.9, "kind": "load-on", "title": tx("Soil on the blade", "Erde auf dem Blatt"), "target": "shovel-cog"}, {"t": 2.9, "kind": "load-off", "title": tx("Soil tossed", "Erde abgeworfen"), "target": "shovel-cog"}, {"t": 3.4, "kind": "contact-on", "title": tx("Blade touches the ground", "Blatt berührt den Boden"), "target": tip}],
        "tracks": [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 5.5}, {"t": 1.6, "v": 6.0}, {"t": 2.6, "v": 6.5}, {"t": d, "v": 5.5}]}, {"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 1}, {"t": 1.6, "v": 2}, {"t": 2.9, "v": 1}]}],
    }
    idle = {
        "id": "idle-breathing", "title": tx("Catching breath", "Verschnaufen"),
        "description": tx("A four-second loop resting on the shovel: deep, quick breaths lift the chest and shoulders, the head drops and rises slightly; hands on the D-grip, the blade planted.", "Eine Schleife von vier Sekunden auf die Schaufel gestützt: tiefe, schnelle Atemzüge heben Brust und Schultern, der Kopf senkt und hebt sich leicht; Hände am D-Griff, das Blatt eingesteckt."),
        "durationS": 4, "fps": 30, "loop": True, "rootMotion": "none",
        "keys": [lib.key(0, "resting-on-shovel"), lib.key(1.0, "resting-on-shovel", {"neck": {"angleDeg": sol["resting-on-shovel"]["bones"]["neck"]["angleDeg"] + 3}, "head": {"angleDeg": sol["resting-on-shovel"]["bones"]["head"]["angleDeg"] + 2}}), lib.key(2.0, "resting-on-shovel"), lib.key(3.0, "resting-on-shovel", {"neck": {"angleDeg": sol["resting-on-shovel"]["bones"]["neck"]["angleDeg"] + 2}}), lib.key(4, "resting-on-shovel")],
        "contacts": lib.span(contacts, 0, 4),
        "phases": [{"id": "rest", "fromS": 0, "toS": 4, "title": tx("Recovering", "Erholen")}],
        "events": [],
        "tracks": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 3}]}],
    }
    return [dig, idle]


DECISIONS = {
    "50": {"silhouette": tx("The shovel in the fists: polypropylene D-grip as bar and two legs, socket, ash shaft, steel blade socket and the round-point blade with its tread edge; shaft and sockets are revolved, the grip and the blade extruded profiles of the shovel bone, so the blade is edge-on in the side view and face-on from ahead whenever it points lateral.", "Die Schaufel in den Fäusten: D-Griff aus Polypropylen als Steg und zwei Schenkel, Hülse, Eschenstiel, Stahltülle und das Spitzblatt mit Trittkante; Stiel und Tüllen sind gedreht, Griff und Blatt extrudierte Profile des Schaufelknochens, sodass das Blatt in der Seitenansicht hochkant und von vorn flächig erscheint, sobald es quer steht."),
           "retained": tx("D-grip legs, grip socket, blade socket, knee pads and cargo pockets", "D-Griff-Schenkel, Griffhülse, Blatttülle, Knieschoner und Beintaschen"),
           "thinElements": tx("blade drawn at its 11 mm dished depth, not at its 2.5 mm sheet", "Blatt in seiner Wölbungstiefe von 11 mm gezeichnet, nicht in seiner Blechstärke von 2,5 mm")},
    "100": {"silhouette": tx("Shovel as D-grip, shaft, blade socket and blade.", "Schaufel aus D-Griff, Stiel, Blatttülle und Blatt."),
            "merged": tx("D-grip bar, legs and socket into one grip outline", "Griffsteg, Schenkel und Hülse in einen Griffumriss")},
    "200": {"silhouette": tx("Shovel as grip bar, one shaft and the blade.", "Schaufel als Griffsteg, ein Stiel und das Blatt."),
            "merged": tx("sockets into the shaft", "Tüllen in den Stiel"),
            "thinElements": tx("shaft drawn at 40 mm (0.2 mm), its real 38 mm widened within the outline tolerance", "Stiel mit 40 mm gezeichnet (0,2 mm), seine realen 38 mm innerhalb der Umrisstoleranz verbreitert")},
    "500": {"silhouette": tx("Shovel as a shaft bar and the blade: digging reads at a glance from the stooped figure and the long diagonal.", "Schaufel als Stielstab und Blatt: Das Graben ist an der gebeugten Figur und der langen Diagonale auf einen Blick lesbar."),
            "merged": tx("D-grip into the shaft end", "D-Griff in das Stielende"),
            "thinElements": tx("shaft drawn 76 mm wide (0.15 mm), its real 38 mm thickened to the 75 mm feature", "Stiel 76 mm breit gezeichnet (0,15 mm), seine realen 38 mm auf das Mindestmaß von 75 mm verdickt")},
}
VOLUME = {
    "50": tx("Shovel 1:50: D-grip as an extruded bar and two legs, grip socket, slender ash shaft of 38 mm, blade socket as a loft and the round-point blade as an 11 mm extruded plate (its dished envelope) on the shovel bone. Thermal: shaft and grip (ash, polypropylene) and blade (steel) as passive regions. Structure: one member per part on the shovel bone.", "Schaufel 1:50: D-Griff als extrudierter Steg und zwei Schenkel, Griffhülse, schlanker Eschenstiel von 38 mm, Blatttülle als Loft und das Spitzblatt als extrudierte Platte von 11 mm (seine gewölbte Hülle) am Schaufelknochen. Thermik: Stiel und Griff (Esche, Polypropylen) und Blatt (Stahl) als passive Regionen. Tragwerk: ein Glied je Teil am Schaufelknochen."),
    "100": tx("Shovel 1:100: grip, slender shaft, blade socket and blade plate.", "Schaufel 1:100: Griff, schlanker Stiel, Blatttülle und Blattplatte."),
    "200": tx("Shovel 1:200: grip bar, 40 mm shaft and blade plate declared as a sheet.", "Schaufel 1:200: Griffsteg, Stiel von 40 mm und als Blech erklärte Blattplatte."),
    "500": tx("Shovel 1:500: shaft bar at the 76 mm feature and blade sheet.", "Schaufel 1:500: Stielstab im Mindestmaß von 76 mm und Blattblech."),
}
