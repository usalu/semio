"""📝️ Bilingual texts, breathing patterns and clips of `worker-with-jackhammer` (English first, German second)."""
from lib import tx

TITLE = tx("Worker with Jackhammer", "Arbeitende Person mit Abbruchhammer")
SUMMARY = tx(
    "A road worker of 1.80 m and 92 kg in hi-vis vest, hard hat with helmet-mounted ear defenders, leather gloves and safety boots breaks concrete with a 31 kg electric demolition hammer on a pointed chisel, both fists on the T-handle in a braced, staggered stance: breaking, pressing down, rocking the chisel out and standing ready. The 2000 W nameplate of the breaker dominates the total power; the heavy work of about 6 met adds some 750 W of body heat.",
    "Eine Straßenbauarbeiterin bzw. ein Straßenbauarbeiter von 1,80 m und 92 kg mit Warnweste, Schutzhelm mit Kapselgehörschutz, Lederhandschuhen und Sicherheitsstiefeln bricht mit einem 31 kg schweren Elektro-Abbruchhammer und Spitzmeißel Beton auf, beide Fäuste am T-Griff, in versetzter, abgestützter Stellung: Brechen, Nachdrücken, Meißel herauswippen und Bereitstehen. Die Nennleistung des Hammers von 2000 W bestimmt die Gesamtleistung; die Schwerarbeit von etwa 6 met fügt rund 750 W Körperwärme hinzu.",
)
TAGS = ["human", "worker", "construction", "road-works", "jackhammer", "power-tool", "hearing-protection", "hi-vis", "hard-hat"]
NOTES = tx(
    "Rig, outlines and volumes derived from the archive figure worker-carrying-box (round 5, stature 1.741 m) without its carton, scaled uniformly by 1.034 to a stature of 1.80 m (DIN 33402-2 and ANSUR II male, about the 60th percentile) with de Leva's male segment masses for a heavy 92 kg build. The breaker (30 kg class electric demolition hammer after EN 62841-2-6 catalogue machines: 2000 W input, T-handle 0.66 m wide, 28 mm hexagon pointed chisel, 0.96 m from handle to point) and the helmet-mounted ear defenders (EN 352-3) are stated per scale as revolved and extruded parts drawn and modelled from the same profile. In every pose both arms and the breaker joint are solved so that both fists hold the T-handle and the chisel point stands on the ground (HW-04 ik.ts on the product's own chain).",
    "Skelett, Umrisse und Volumen abgeleitet von der Archivfigur worker-carrying-box (Runde 5, Körperhöhe 1,741 m) ohne Karton, gleichmäßig um 1,034 auf eine Körperhöhe von 1,80 m skaliert (DIN 33402-2 und ANSUR II Männer, etwa 60. Perzentil) mit den männlichen Segmentmassen nach de Leva für einen kräftigen Körperbau von 92 kg. Der Hammer (Elektro-Abbruchhammer der 30-kg-Klasse nach Katalogmaschinen gemäß EN 62841-2-6: 2000 W Aufnahme, T-Griff 0,66 m breit, Sechskant-Spitzmeißel 28 mm, 0,96 m vom Griff bis zur Spitze) und der am Helm befestigte Kapselgehörschutz (EN 352-3) sind je Maßstab aus gedrehten und extrudierten Teilen beschrieben, die aus demselben Profil gezeichnet und modelliert sind. In jeder Pose sind beide Arme und das Hammergelenk so gelöst, dass beide Fäuste den T-Griff halten und die Meißelspitze am Boden steht (HW-04 ik.ts auf der Kette des Produkts).",
)
REFS = [
    {"id": "en-62841-2-6", "citation": "EN 62841-2-6:2020 Electric motor-operated hand-held tools, safety, part 2-6: hammers (demolition hammers of the 30 kg class with about 2 kW rated input, catalogue values of Makita HM1812, Hilti TE 3000-AVR, Bosch GSH 27)."},
    {"id": "en-352-3", "citation": "EN 352-3:2020 Hearing protectors, safety requirements, part 3: ear-muffs attached to a head protection."},
    {"id": "iso-7730-2005", "citation": "ISO 7730:2005 Ergonomics of the thermal environment, analytical determination of thermal comfort (metabolic rates, Annex B)."},
]
PATTERNS = [
    {"id": "rest", "title": tx("Calm standing breathing", "Ruhige Atmung im Stand"), "rateBpm": 14, "tidalVolumeL": 0.7, "inhaleFraction": 0.38, "pauseAfterInhaleFraction": 0.04, "exhaleFraction": 0.45, "pauseAfterExhaleFraction": 0.13, "route": "nose", "source": "authored-estimate", "reference": "gupta-2010"},
    {"id": "breaking", "title": tx("Breaking, heavy work", "Brechen, Schwerarbeit"), "rateBpm": 26, "tidalVolumeL": 1.8, "inhaleFraction": 0.44, "pauseAfterInhaleFraction": 0.01, "exhaleFraction": 0.52, "pauseAfterExhaleFraction": 0.03, "route": "nose-and-mouth", "source": "authored-estimate", "reference": "gupta-2010"},
    {"id": "straining", "title": tx("Straining to rock the chisel out", "Pressen beim Herauswippen"), "rateBpm": 30, "tidalVolumeL": 1.9, "inhaleFraction": 0.46, "pauseAfterInhaleFraction": 0.0, "exhaleFraction": 0.54, "pauseAfterExhaleFraction": 0.0, "route": "nose-and-mouth", "source": "authored-estimate", "reference": "gupta-2010"},
]


def thermal_assumptions(area):
    return [
        tx("Body regions sum to the DuBois area of %.3f m2 (92 kg, 180 cm); clothed regions keep the area factors of the base figure. Basal metabolism after Mifflin-St Jeor for a man of 38." % area, "Die Körperregionen ergeben die DuBois-Fläche von %s m2 (92 kg, 180 cm); bekleidete Regionen behalten die Flächenfaktoren der Ausgangsfigur. Grundumsatz nach Mifflin-St Jeor für einen 38-jährigen Mann." % ("%.3f" % area).replace(".", ",")),
        tx("Clothing is layered per region as in the base figure (T-shirt, work shirt, mesh vest; briefs, work trousers; socks, leather boot; hair, air gap and HDPE shell in the helmet) with garment values guided by ISO 9920; the ear defenders are a passive ABS surface.", "Die Kleidung ist je Region wie bei der Ausgangsfigur geschichtet (T-Shirt, Arbeitshemd, Netzweste; Unterhose, Arbeitshose; Socken, Lederstiefel; Haar, Luftspalt und HDPE-Schale im Helm) mit Werten nach Anhalt von ISO 9920; der Kapselgehörschutz ist eine passive ABS-Fläche."),
        tx("Metabolic rates follow the Compendium of Physical Activities (using heavy power tools such as jackhammers 6.3 met) and ISO 8996 class 3; standing ready with the hands on the handle 1.8 met.", "Die Stoffwechselraten folgen dem Compendium of Physical Activities (Arbeiten mit schweren Elektrowerkzeugen wie Abbruchhämmern 6,3 met) und ISO 8996 Klasse 3; Bereitstehen mit den Händen am Griff 1,8 met."),
        tx("The breaker draws its 2000 W nameplate at full impact; about 350 W leave through its motor, gear and cylinder housing (the rest goes into the chisel, the concrete and the motor's cooling air). The reference case is a sunny summer road site (25 C, 45 % relative humidity, 1.5 m/s wind, 600 W/m2 sun).", "Der Hammer nimmt bei voller Schlagleistung seine Nennleistung von 2000 W auf; etwa 350 W treten über Motor-, Getriebe- und Zylindergehäuse aus (der Rest geht in Meißel, Beton und die Kühlluft des Motors). Der Referenzfall ist eine sonnige Straßenbaustelle im Sommer (25 C, 45 % relative Feuchte, 1,5 m/s Wind, 600 W/m2 Sonne)."),
    ]


STRUCTURE_ASSUMPTIONS = [
    tx("Segments are point masses at the de Leva centres of mass (male table); rotational inertia and the impact forces of the hammer (about 1000 blows per minute) are neglected.", "Segmente sind Punktmassen in den Schwerpunkten nach de Leva (Männertabelle); Drehträgheit und Schlagkräfte des Hammers (etwa 1000 Schläge je Minute) werden vernachlässigt."),
    tx("The 31 kg breaker hangs on the right fist in the rigid-segment tree; while the chisel point stands on the concrete it is a ground contact with friction 0.5 that shares the weight with both boots under the equal-stiffness assumption. The left fist closes the loop on the T-handle only geometrically.", "Der 31 kg schwere Hammer hängt im starren Segmentbaum an der rechten Faust; solange die Meißelspitze auf dem Beton steht, ist sie ein Bodenkontakt mit Reibung 0,5, der das Gewicht unter der Annahme gleicher Steifigkeit mit beiden Stiefeln teilt. Die linke Faust schließt die Kette am T-Griff nur geometrisch."),
]
POSES = {
    "breaking": (tx("Breaking concrete", "Beton aufbrechen"), tx("Staggered braced stance with the left foot forward and the knees soft, trunk leaning slightly over the breaker, both fists on the T-handle at hip height and the forearms pressing down; the chisel point stands on the concrete with the tool tilted 5 degrees towards the operator.", "Versetzte, abgestützte Stellung mit dem linken Fuß vorn und weichen Knien, der Rumpf leicht über den Hammer geneigt, beide Fäuste am T-Griff auf Hüfthöhe, die Unterarme drücken nach unten; die Meißelspitze steht auf dem Beton, das Gerät 5 Grad zur bedienenden Person geneigt.")),
    "pressing-down": (tx("Pressing down", "Nachdrücken"), tx("Knees bent deeper and trunk further over the handle to put body weight on a stubborn spot; the breaker tilts 9 degrees towards the operator.", "Knie tiefer gebeugt und Rumpf weiter über dem Griff, um an einer zähen Stelle Körpergewicht aufzubringen; der Hammer neigt sich 9 Grad zur bedienenden Person.")),
    "rocking-out": (tx("Rocking the chisel out", "Meißel herauswippen"), tx("Upright and leaning back, both fists pull the T-handle towards the body so the breaker tilts 20 degrees and levers the stuck chisel out of the crack.", "Aufrecht und nach hinten gelehnt ziehen beide Fäuste den T-Griff zum Körper, sodass der Hammer um 20 Grad kippt und den festsitzenden Meißel aus dem Riss hebelt.")),
    "standing-ready": (tx("Standing ready", "Bereitstehen"), tx("Upright with the breaker standing vertically on its chisel in front of the body, both hands resting on the grips, the motor idling.", "Aufrecht, der Hammer steht senkrecht auf seinem Meißel vor dem Körper, beide Hände ruhen auf den Griffen, der Motor läuft im Leerlauf.")),
}


def clips(lib, contacts, sol):
    b = sol["breaking"]["bones"]

    def shake(dt, dn):
        return {"torso": {"angleDeg": b["torso"]["angleDeg"] + dt}, "neck": {"angleDeg": b["neck"]["angleDeg"] + dn}}

    hammer = {
        "id": "hammering", "title": tx("Hammering", "Meißeln"),
        "description": tx("A 1.2-second loop at full impact: the recoil of the blows runs through the arms into the trunk and the head as a quick small shudder while the point eats into the concrete; feet and chisel stay put.", "Eine Schleife von 1,2 Sekunden bei voller Schlagleistung: Der Rückstoß der Schläge läuft über die Arme als schnelles kleines Zittern in Rumpf und Kopf, während sich die Spitze in den Beton frisst; Füße und Meißel bleiben stehen."),
        "durationS": 1.2, "fps": 30, "loop": True, "rootMotion": "none",
        "keys": [lib.key(0, "breaking"), lib.key(0.3, "breaking", shake(-0.8, 1.2)), lib.key(0.6, "breaking", shake(0.5, -0.8)), lib.key(0.9, "breaking", shake(-0.6, 1.0)), lib.key(1.2, "breaking")],
        "contacts": lib.span(contacts, 0, 1.2),
        "phases": [{"id": "impact", "fromS": 0, "toS": 1.2, "title": tx("Full impact", "Volle Schlagleistung")}],
        "events": [],
        "tracks": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 1}]}, {"target": "operating:breaker", "ease": "step", "keys": [{"t": 0, "v": 1}]}],
    }
    rock = {
        "id": "break-and-rock", "title": tx("Breaking and rocking out", "Brechen und Herauswippen"),
        "description": tx("A 4-second loop: break, lean in and press, release the trigger and rock the stuck chisel out by pulling the handle back, set it upright again.", "Eine Schleife von 4 Sekunden: brechen, sich hineinlehnen und nachdrücken, den Schalter loslassen und den festsitzenden Meißel durch Zurückziehen des Griffs herauswippen, wieder aufrichten."),
        "durationS": 4.0, "fps": 30, "loop": True, "rootMotion": "none",
        "keys": [lib.key(0, "breaking"), lib.key(1.2, "pressing-down"), lib.key(2.0, "breaking"), lib.key(2.9, "rocking-out"), lib.key(4.0, "breaking")],
        "contacts": lib.span(contacts, 0, 4.0),
        "phases": [{"id": "break", "fromS": 0, "toS": 2.0, "title": tx("Breaking and pressing", "Brechen und Nachdrücken")}, {"id": "rock", "fromS": 2.0, "toS": 4.0, "title": tx("Rocking out", "Herauswippen")}],
        "events": [{"t": 2.0, "kind": "operating", "title": tx("Trigger released", "Schalter losgelassen"), "target": "breaker"}, {"t": 3.6, "kind": "operating", "title": tx("Trigger pressed", "Schalter gedrückt"), "target": "breaker"}],
        "tracks": [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 6.3}, {"t": 1.2, "v": 6.5}, {"t": 2.9, "v": 6.0}, {"t": 4.0, "v": 6.3}]}, {"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 1}, {"t": 2.0, "v": 2}, {"t": 3.6, "v": 1}]}, {"target": "operating:breaker", "ease": "step", "keys": [{"t": 0, "v": 1}, {"t": 2.0, "v": 0.3}, {"t": 3.6, "v": 1}]}],
    }
    idle = {
        "id": "idle-breathing", "title": tx("Idle breathing", "Ruhiges Atmen"),
        "description": tx("A five-second loop standing ready: the chest rises and falls with calm breathing, the head settles in a slow sway, the hands stay on the grips and the motor idles.", "Eine Schleife von fünf Sekunden im Bereitstehen: Die Brust hebt und senkt sich mit ruhigem Atmen, der Kopf wiegt sich langsam, die Hände bleiben an den Griffen und der Motor läuft im Leerlauf."),
        "durationS": 5, "fps": 30, "loop": True, "rootMotion": "none",
        "keys": [lib.key(0, "standing-ready"), lib.key(1.25, "standing-ready", {"neck": {"angleDeg": sol["standing-ready"]["bones"]["neck"]["angleDeg"] + 1.5}}), lib.key(2.5, "standing-ready", {"head": {"angleDeg": sol["standing-ready"]["bones"]["head"]["angleDeg"] - 1.5}}), lib.key(3.75, "standing-ready", {"neck": {"angleDeg": sol["standing-ready"]["bones"]["neck"]["angleDeg"] + 1.0}}), lib.key(5, "standing-ready")],
        "contacts": lib.span(contacts, 0, 5),
        "phases": [{"id": "idle", "fromS": 0, "toS": 5, "title": tx("Standing ready", "Bereitstehen")}],
        "events": [],
        "tracks": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}]}, {"target": "operating:breaker", "ease": "step", "keys": [{"t": 0, "v": 0.3}]}],
    }
    return [hammer, rock, idle]


DECISIONS = {
    "50": {"silhouette": tx("The breaker stands in front of the figure: T-handle with rubber grips and two posts, glass-fibre motor housing, cast cylinder, chuck and pointed hexagon chisel; the ear-defender cups and their arms sit at the helmet rim.", "Der Hammer steht vor der Figur: T-Griff mit Gummigriffen und zwei Stützen, Motorgehäuse aus Glasfaser, Gusszylinder, Futter und Sechskant-Spitzmeißel; die Gehörschutzkapseln und ihre Bügel sitzen am Helmrand."),
           "retained": tx("both grips, handle posts, chuck collar, chisel point, ear-cup arms", "beide Griffe, Griffstützen, Futterring, Meißelspitze, Kapselbügel"),
           "omitted": tx("cable, switch lever, vibration dampers, vents", "Kabel, Schalthebel, Schwingungsdämpfer, Lüftungsschlitze")},
    "100": {"silhouette": tx("Breaker as T-handle, motor housing, cylinder with chuck and chisel; ear cups with their arms.", "Hammer aus T-Griff, Motorgehäuse, Zylinder mit Futter und Meißel; Gehörschutzkapseln mit Bügeln."),
            "merged": tx("grips into the T-bar, handle posts into the motor housing, chuck into the cylinder", "Griffe in den T-Balken, Griffstützen in das Motorgehäuse, Futter in den Zylinder"),
            "omitted": tx("cable, switch, dampers", "Kabel, Schalter, Dämpfer")},
    "200": {"silhouette": tx("Breaker as T-bar, motor block, cylinder and chisel; ear cups as two small ovals at the helmet rim.", "Hammer als T-Balken, Motorblock, Zylinder und Meißel; Gehörschutzkapseln als zwei kleine Ovale am Helmrand."),
            "merged": tx("ear-cup arms into the cups", "Kapselbügel in die Kapseln"),
            "thinElements": tx("chisel drawn at 40 mm (0.2 mm), its real 28 to 32 mm widened within the outline tolerance", "Meißel mit 40 mm gezeichnet (0,2 mm), seine realen 28 bis 32 mm innerhalb der Umrisstoleranz verbreitert")},
    "500": {"silhouette": tx("Breaker as T-bar, one tapering body and the chisel bar between the hands and the ground: the stance over the machine reads at a glance.", "Hammer als T-Balken, ein sich verjüngender Körper und der Meißelstab zwischen den Händen und dem Boden: Die Haltung über der Maschine ist auf einen Blick lesbar."),
            "merged": tx("ear defenders into the helmet piece; motor, cylinder and chuck into one body", "Gehörschutz in das Helmteil; Motor, Zylinder und Futter in einen Körper"),
            "thinElements": tx("chisel drawn 70 mm wide (0.14 mm with the outline), its real 28 mm thickened", "Meißel 70 mm breit gezeichnet (0,14 mm mit dem Umriss), seine realen 28 mm verdickt")},
}
VOLUME = {
    "50": tx("Breaker and ear defenders 1:50: T-bar, two rubber grips and handle posts as extruded bars, the motor housing as an extruded block of 0.18 x 0.22 m, the cast cylinder, chuck and chisel as lofts of their stations; ear cups as ellipsoids over the ears with capsule arms to the helmet rim. Thermal: breaker housing (operating, 2000 W nameplate, 350 W released at full impact), chisel and grips as passive regions, ear defenders passive. Structure: one member per part on the breaker bone.", "Hammer und Gehörschutz 1:50: T-Balken, zwei Gummigriffe und Griffstützen als extrudierte Stäbe, das Motorgehäuse als extrudierter Block von 0,18 x 0,22 m, Gusszylinder, Futter und Meißel als Lofts ihrer Stationen; Gehörschutzkapseln als Ellipsoide über den Ohren mit Kapselbügeln zum Helmrand. Thermik: Hammergehäuse (Betrieb, 2000 W Nennleistung, 350 W Abgabe bei voller Schlagleistung), Meißel und Griffe als passive Regionen, Gehörschutz passiv. Tragwerk: ein Glied je Teil am Hammerknochen."),
    "100": tx("Breaker and ear defenders 1:100: T-bar, motor block, cylinder with chuck, chisel; ear cups with arms.", "Hammer und Gehörschutz 1:100: T-Balken, Motorblock, Zylinder mit Futter, Meißel; Gehörschutzkapseln mit Bügeln."),
    "200": tx("Breaker and ear defenders 1:200: T-bar, motor block, cylinder and a slender chisel; the ear cups as two ellipsoids.", "Hammer und Gehörschutz 1:200: T-Balken, Motorblock, Zylinder und ein schlanker Meißel; die Gehörschutzkapseln als zwei Ellipsoide."),
    "500": tx("Breaker 1:500: T-bar, one tapering body and a slender chisel bar; the ear defenders are below the 75 mm feature and belong to the helmet piece of the drawing.", "Hammer 1:500: T-Balken, ein sich verjüngender Körper und ein schlanker Meißelstab; der Gehörschutz liegt unter dem Mindestmaß von 75 mm und gehört zum Helmteil der Zeichnung."),
}
