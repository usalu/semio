"""📝️ Texts, references, materials and thermal regions of `humans/person-sweeping` (English first, German second)."""
from lib import t

TITLE = t("Person Sweeping", "Kehrende Person")
SUMMARY = t(
    "An adult of 1.69 m in a fleece sweatshirt, jeans and canvas sneakers sweeps a hallway floor with a 400 mm push broom on a 1.45 m beech handle: "
    "the right fist low on the handle, the left high, the trunk leaning into each stroke; push, lift and return as a 1.6 s loop at about 3.3 met, "
    "and a rest with the broom held upright.",
    "Eine erwachsene Person von 1,69 m in Fleece-Sweatshirt, Jeans und Canvas-Sneakern kehrt einen Flurboden mit einem 400 mm breiten Schiebebesen "
    "an einem 1,45 m langen Buchenstiel: die rechte Faust tief am Stiel, die linke hoch, der Rumpf neigt sich in jeden Strich; Schieben, Anheben und "
    "Zurückführen als Schleife von 1,6 s bei etwa 3,3 met und eine Pause mit aufrecht gehaltenem Besen.")

DIMENSIONS = {"heightM": 1.688, "widthM": 1.39, "depthM": 0.644}

MATERIALS = [
    {"id": "beech-handle", "title": t("Lacquered beech broom handle", "Lackierter Buchenstiel"), "emissivity": 0.9, "solarAbsorptivity": 0.55, "densityKgM3": 720, "conductivityWmK": 0.16, "source": "literature", "reference": "en-338-2016"},
    {"id": "beech-block", "title": t("Beech broom block", "Besenholz aus Buche"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 720, "conductivityWmK": 0.16, "source": "literature", "reference": "en-338-2016"},
    {"id": "pp-bristle", "title": t("Polypropylene bristles", "Polypropylen-Borsten"), "emissivity": 0.9, "solarAbsorptivity": 0.7, "densityKgM3": 905, "source": "authored-estimate"},
    {"id": "pp-cap", "title": t("Plastic hanging cap and handle socket", "Kunststoff-Aufhängekappe und Stielhülse"), "emissivity": 0.92, "solarAbsorptivity": 0.6, "densityKgM3": 905, "source": "authored-estimate"},
]

REFERENCES = [
    {"id": "ainsworth-2011", "citation": "Ainsworth, B. E. et al. (2011). 2011 Compendium of Physical Activities: a second update of codes and MET values. Medicine and Science in Sports and Exercise 43(8), 1575-1581 (home activities, sweeping floors)."},
    {"id": "iso-8996-2021", "citation": "ISO 8996:2021 Ergonomics of the thermal environment, determination of metabolic rate (activity classes and met values)."},
    {"id": "en-338-2016", "citation": "EN 338:2016 Structural timber, strength classes (density of hardwood class D40 to D50, beech about 700 kg/m3); used here only for the density of the beech handle and block."},
]

NOTES = t(
    "Derived by hand from the archive's standing-adult (rig, anthropometry, clothing, body volumes and outlines unchanged: 1.69 m barefoot, 59 kg, "
    "de Leva female segment table, ANSUR II and DIN 33402-2 breadths). New: a push broom (handle 1.45 m, 24 mm beech; head 400 x 60 mm with 70 mm "
    "polypropylene bristles; 1.2 kg) as two bones carried by the right fist, every part handcrafted per scale as one prism or profile of revolution "
    "that is both the 3D solid and, with its solid placement, the drawing of every view; five poses solved for the grips on the handle and the bristles "
    "on the floor by inverse kinematics on the library's own rig solve; activity 3.3 met for sweeping floors (Ainsworth et al. 2011), 1.3 met resting.",
    "Von Hand aus dem Archivobjekt standing-adult abgeleitet (Skelett, Anthropometrie, Kleidung, Körpervolumen und Umrisse unverändert: 1,69 m barfuß, "
    "59 kg, de-Leva-Segmenttabelle für Frauen, Breiten nach ANSUR II und DIN 33402-2). Neu: ein Schiebebesen (Stiel 1,45 m, 24 mm Buche; Kopf 400 x 60 mm "
    "mit 70 mm Polypropylen-Borsten; 1,2 kg) als zwei Knochen an der rechten Faust, jedes Teil je Maßstab von Hand als Prisma oder Drehprofil gebaut, das "
    "zugleich 3D-Körper und – mit seiner Körperplatzierung – die Zeichnung jeder Ansicht ist; fünf Posen, deren Griffe am Stiel und Borsten am Boden per "
    "inverser Kinematik auf der Skelettlösung der Bibliothek gelöst sind; Aktivität 3,3 met für Bodenkehren (Ainsworth et al. 2011), 1,3 met in der Pause.")

REGIONS = [
    {"id": "broom-wood", "title": t("Broom handle and block", "Besenstiel und Besenholz"), "kind": "surface", "model": "passive", "material": "beech-handle", "areaM2": 0.175, "characteristicLengthM": 0.024, "geometry": "cylinder", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": 7500, "source": "authored-estimate", "reference": "en-338-2016"},
    {"id": "broom-bristles", "title": t("Bristle pad", "Borstenfeld"), "kind": "surface", "model": "passive", "material": "pp-bristle", "areaM2": 0.075, "characteristicLengthM": 0.07, "geometry": "vertical-plate", "skyViewFactor": 0.1, "layers": [], "heatCapacityJM2K": 2000, "source": "authored-estimate"},
]

SWEEP_BREATH = {"id": "sweeping", "title": t("Sweeping, moderate effort", "Kehren, mäßige Anstrengung"), "rateBpm": 20, "tidalVolumeL": 0.95, "inhaleFraction": 0.4, "pauseAfterInhaleFraction": 0.03, "exhaleFraction": 0.5, "pauseAfterExhaleFraction": 0.07, "route": "nose-and-mouth", "source": "authored-estimate", "reference": "icrp-66-1994"}

THERMAL_ASSUMPTIONS = [
    t("Indoor hallway at 21 degrees Celsius, 50 % humidity, still air; DuBois skin area of the 1.69 m, 58 kg body; clothing after ISO 9920 (fleece sweatshirt over a T-shirt, jeans over briefs, socks and sneakers), 1 clo = 0.155 m2K/W.",
      "Innenflur bei 21 Grad Celsius, 50 % Feuchte, ruhende Luft; DuBois-Hautfläche des Körpers von 1,69 m und 58 kg; Kleidung nach ISO 9920 (Fleece-Sweatshirt über T-Shirt, Jeans über Slip, Socken und Sneaker), 1 clo = 0,155 m2K/W."),
    t("Sweeping the floor with a push broom at 3.3 met (Ainsworth et al. 2011, home activities, sweeping floors, general; ISO 8996 class 2), resting on the broom at 1.3 met; ventilation 20 breaths per minute with 0.95 L while sweeping (authored estimate after ICRP 66 for light exercise).",
      "Kehren mit dem Schiebebesen bei 3,3 met (Ainsworth et al. 2011, Haushaltstätigkeiten, Boden kehren allgemein; ISO 8996 Klasse 2), Pause auf den Besen gestützt bei 1,3 met; Atmung beim Kehren 20 Atemzüge pro Minute mit 0,95 L (Schätzwert nach ICRP 66 für leichte Arbeit)."),
    t("Broom handle, block and bristles are passive surfaces without internal heat; the bristles touch the floor at room temperature while they sweep.",
      "Besenstiel, Besenholz und Borsten sind passive Flächen ohne innere Wärme; die Borsten berühren beim Kehren den raumwarmen Boden."),
]

STRUCTURE_ASSUMPTIONS = [
    t("Segments are point masses at the de Leva centres (female table); the broom handle (0.52 kg) and head (0.68 kg) carry their own masses at their centres.",
      "Segmente sind Punktmassen in den Schwerpunkten nach de Leva (Frauentabelle); Besenstiel (0,52 kg) und Besenkopf (0,68 kg) tragen ihre eigenen Massen in ihren Schwerpunkten."),
    t("Supports are the heel and ball of each foot and, while sweeping, the bristle tips on the floor (friction 0.45); the push force reaches the floor through the fists, the handle and the head, so shoulders, elbows and the lumbar spine carry the moments of the forward-leaning stroke.",
      "Auflager sind Ferse und Ballen jedes Fußes und beim Kehren die Borstenspitzen auf dem Boden (Reibung 0,45); die Schubkraft gelangt über die Fäuste, den Stiel und den Kopf in den Boden, sodass Schultern, Ellbogen und Lendenwirbelsäule die Momente des vorgeneigten Strichs tragen."),
]

POSES = {
    "push-start": (t("Stroke start", "Strichbeginn"), t("Feet staggered at hip width, trunk leaning slightly forward, both fists on the handle close to the body, the bristles set down about one metre ahead.", "Füße hüftbreit versetzt, Rumpf leicht vorgeneigt, beide Fäuste körpernah am Stiel, die Borsten etwa einen Meter vor der Person aufgesetzt.")),
    "pushing": (t("Pushing the broom", "Besen schieben"), t("Mid-stroke: the arms push the handle forward and down, the trunk leans into the stroke, the eyes follow the bristles on the floor.", "Mitte des Strichs: Die Arme schieben den Stiel nach vorn und unten, der Rumpf neigt sich in den Strich, der Blick folgt den Borsten am Boden.")),
    "push-end": (t("Stroke end", "Strichende"), t("Arms extended, trunk leaning furthest forward, the head of the broom about 1.3 m ahead at the end of the push.", "Arme gestreckt, Rumpf am weitesten vorgeneigt, der Besenkopf am Ende des Schubs etwa 1,3 m voraus.")),
    "broom-lifted": (t("Lifting the broom", "Besen anheben"), t("The return: the fists draw the handle back and lift the bristles 50 mm off the floor so the dust stays in front.", "Das Zurückführen: Die Fäuste ziehen den Stiel zurück und heben die Borsten 50 mm vom Boden, damit der Staub vorn bleibt.")),
    "resting-on-broom": (t("Resting with the broom", "Pause mit dem Besen"), t("Upright, feet at hip width, the right fist holds the broom upright beside the body at chest height, the bristles on the floor; the left arm hangs loosely.", "Aufrecht, Füße hüftbreit, die rechte Faust hält den Besen aufrecht neben dem Körper auf Brusthöhe, die Borsten am Boden; der linke Arm hängt locker.")),
}

CLIPS = {
    "sweep-stroke": (t("Sweep stroke", "Kehrstrich"), t("A 1.6 s loop: push the broom forward with the bristles on the floor (0.9 s), lift it 50 mm and draw it back (0.7 s) while the feet stay planted; about 3.3 met with quicker breathing.", "Eine Schleife von 1,6 s: den Besen mit den Borsten am Boden nach vorn schieben (0,9 s), 50 mm anheben und zurückführen (0,7 s), während die Füße stehen bleiben; etwa 3,3 met mit schnellerer Atmung.")),
    "idle-breathing": (t("Idle breathing", "Ruhiges Atmen"), t("A five-second loop resting with the broom held upright: the chest rises and falls with quiet breathing, head and trunk sway slightly; feet and bristles stay on the floor.", "Eine Schleife von fünf Sekunden in der Pause mit aufrecht gehaltenem Besen: Die Brust hebt und senkt sich mit ruhigem Atmen, Kopf und Rumpf wiegen sich leicht; Füße und Borsten bleiben am Boden.")),
}


def _d(silhouette, retained, merged, omitted, thin, hatching, styles, overlays):
    return {"silhouette": silhouette, "retained": retained, "merged": merged, "omitted": omitted, "thinElements": thin, "hatching": hatching, "styles": styles, "overlays": overlays}


DECISIONS = {
    "50": _d(
        t("The three views are the true orthographic views of the 1:50 body pushing the broom, 34.4 mm tall: head with jaw, nose and chin under the hair cap, ear and eye, neck, hooded sweatshirt with collar and hem rib, hood and rolled edge, sleeves, forearms and rib cuffs, both fists closed round the handle with thumb and fingers, jeans seat, thighs with kneecaps, shanks, socks and sneakers; the broom with its rounded hanging cap, the 24 mm handle, the socket, the beech block with rounded corners and the bristle pad. Front view (the figure's left is on the right of the sheet): the broom head 400 mm wide ahead of the feet. Plan: the handle passes beside the right hip to the head on the floor. Every part is bound to its own bone and carried through every pose.",
          "Die drei Ansichten sind die wahren orthogonalen Ansichten des 1:50-Körpers beim Schieben des Besens, 34,4 mm hoch: Kopf mit Kiefer, Nase und Kinn unter der Haarkappe, Ohr und Auge, Hals, Kapuzen-Sweatshirt mit Kragen- und Saumbund, Kapuze und Rollkante, Ärmel, Unterarme und Bündchen, beide Fäuste mit Daumen und Fingern um den Stiel geschlossen, Jeansgesäß, Oberschenkel mit Kniescheiben, Unterschenkel, Socken und Sneaker; der Besen mit gerundeter Aufhängekappe, dem 24 mm starken Stiel, der Hülse, dem Buchenholz mit gerundeten Ecken und dem Borstenfeld. Vorderansicht (die linke Körperseite liegt rechts im Blatt): der 400 mm breite Besenkopf vor den Füßen. Grundriss: Der Stiel führt neben der rechten Hüfte zum Kopf auf dem Boden. Jedes Teil gehört zu seinem Knochen und folgt jeder Pose."),
        t("Everything of the 1:100 drawing plus the hanging cap, the rounded block corners and the finger joints of both fists.", "Alles der 1:100-Zeichnung sowie Aufhängekappe, gerundete Holzecken und die Fingerglieder beider Fäuste."),
        t("Eyelets into the laces, single hairs into three strands, lips, chin and brows into the head outline; the bristle tufts into one pad.", "Ösen in die Schnürung, einzelne Haare in drei Strähnen, Lippen, Kinn und Brauen in den Kopfumriss; die Borstenbüschel in ein Feld."),
        t("Pores, single hairs, stitching holes, fabric fibres; single bristle tufts and the screw thread of the handle.", "Poren, einzelne Haare, Stichlöcher, Gewebefasern; einzelne Borstenbüschel und das Schraubgewinde des Stiels."),
        t("Finger separations, laces and seams are 0.10 mm texture lines; the 24 mm handle is drawn at its true 0.48 mm width.", "Fingertrennungen, Schnürung und Nähte sind 0,10-mm-Texturlinien; der 24 mm starke Stiel ist in seiner wahren Breite von 0,48 mm gezeichnet."),
        t("Hoodie and sleeves: vertical knit lines at 0.6 mm; jeans: twill lines at 62 degrees, 0.5 mm; the broom is plain tone.", "Hoodie und Ärmel: senkrechte Strickstriche mit 0,6 mm; Jeans: Köperlinien unter 62 Grad, 0,5 mm; der Besen ist reiner Ton."),
        t("Body 0.25 mm, detail 0.13 mm, texture 0.10 mm; light wooden handle, mid-grey block, near-black bristles, dark caps.", "Körper 0,25 mm, Detail 0,13 mm, Textur 0,10 mm; heller Holzstiel, mittelgraues Holz, fast schwarze Borsten, dunkle Kappen."),
        t("One region per shape; handle, socket, cap and block carry the broom-wood region, the bristle pad its own. Front and plan as in the side view.", "Eine Region je Form; Stiel, Hülse, Kappe und Holz tragen die Region des Besenholzes, das Borstenfeld seine eigene. Vorderansicht und Grundriss wie in der Seitenansicht.")),
    "100": _d(
        t("The three views are the true orthographic views of the 1:100 body pushing the broom, 17.2 mm tall: head with nose under the hair cap, ear, neck, sweatshirt with the hood on the upper back, sleeves, forearms, a grouped left fist and an articulated right fist on the handle, jeans seat, thighs, shanks, one shoe form and toe box per foot; handle, socket, block and bristle pad of the broom. Inner lines: cuff, hem, pocket, collar, side seam and sole. Front view (the figure's left is on the right of the sheet): the broom head across the floor ahead of the feet. Plan: shoes, legs, broom, hands, arms, sweatshirt, hood, neck, head and hair in the order of their height.",
          "Die drei Ansichten sind die wahren orthogonalen Ansichten des 1:100-Körpers beim Schieben des Besens, 17,2 mm hoch: Kopf mit Nase unter der Haarkappe, Ohr, Hals, Sweatshirt mit der Kapuze auf dem oberen Rücken, Ärmel, Unterarme, eine zusammengefasste linke und eine bewegliche rechte Faust am Stiel, Jeansgesäß, Oberschenkel, Unterschenkel, eine Schuhform und Zehenkappe je Fuß; Stiel, Hülse, Holz und Borstenfeld des Besens. Innenlinien: Bündchen, Saum, Tasche, Kragen, Seitennaht und Sohle. Vorderansicht (die linke Körperseite liegt rechts im Blatt): der Besenkopf quer über dem Boden vor den Füßen. Grundriss: Schuhe, Beine, Besen, Hände, Arme, Sweatshirt, Kapuze, Hals, Kopf und Haar in der Reihenfolge ihrer Höhe."),
        t("Hair mass with ear, face profile with nose and chin, the fists round the handle, the socket where handle and head meet.", "Haarmasse mit Ohr, Gesichtsprofil mit Nase und Kinn, die Fäuste um den Stiel, die Hülse zwischen Stiel und Kopf."),
        t("Eye, brow and mouth are not drawn; laces into the sneaker; the left finger groups share one outline; the hanging cap into the handle end.", "Auge, Braue und Mund werden nicht gezeichnet; Schnürung im Sneaker; die linken Fingergruppen teilen einen Umriss; die Aufhängekappe im Stielende."),
        t("Eye (0.16 mm), laces, knee folds, drawstrings, stitching, block corner radii.", "Auge (0,16 mm), Schnürsenkel, Kniefalten, Kordeln, Nähte, Eckradien des Holzes."),
        t("The 24 mm handle is drawn at its true 0.24 mm width with the 0.20 mm body pen around it, so it prints as one firm line.", "Der 24 mm starke Stiel ist in wahrer Breite von 0,24 mm mit dem 0,20-mm-Körperstift gezeichnet und druckt als eine kräftige Linie."),
        t("Jeans only: twill lines at 62 degrees, 0.7 mm spacing, 0.10 mm pen; fleece and broom stay plain tone.", "Nur Jeans: Köperlinien unter 62 Grad, 0,7 mm Abstand, 0,10-mm-Stift; Fleece und Besen bleiben reiner Ton."),
        t("Body pen 0.20 mm, detail 0.10 mm; hoodie mid-grey, jeans darker with twill, sneakers light; handle light, block mid, bristles dark.", "Körperstift 0,20 mm, Detail 0,10 mm; Hoodie mittelgrau, Jeans dunkler mit Köper, Sneaker hell; Stiel hell, Holz mittel, Borsten dunkel."),
        t("Regions as at 1:50; hood and hoodie share the hoodie value; handle, socket and block the broom-wood value. Front and plan: regions as in the side view.", "Regionen wie bei 1:50; Kapuze und Hoodie teilen den Hoodie-Wert; Stiel, Hülse und Holz den Wert des Besenholzes. Vorderansicht und Grundriss: Regionen wie in der Seitenansicht.")),
    "200": _d(
        t("The three views are the true orthographic views of the 1:200 body pushing the broom, 8.6 mm tall, one mass per part: head, hair cap, neck, sweatshirt, hood, jeans seat, sleeves, forearms, fists, thighs, shanks, shoes and toe boxes; the broom as handle, block and bristle pad. Front view (the figure's left is on the right of the sheet): the broom head reads as a 2 mm bar on the floor ahead of the feet. Plan: the masses in the order of their height.",
          "Die drei Ansichten sind die wahren orthogonalen Ansichten des 1:200-Körpers beim Schieben des Besens, 8,6 mm hoch, eine Masse je Teil: Kopf, Haarkappe, Hals, Sweatshirt, Kapuze, Jeansgesäß, Ärmel, Unterarme, Fäuste, Oberschenkel, Unterschenkel, Schuhe und Zehenkappen; der Besen als Stiel, Holz und Borstenfeld. Vorderansicht (die linke Körperseite liegt rechts im Blatt): Der Besenkopf liest sich als 2 mm langer Balken auf dem Boden vor den Füßen. Grundriss: die Massen in der Reihenfolge ihrer Höhe."),
        t("The forward lean, both arms reaching to the handle, the long diagonal of the handle and the broom head on the floor.", "Die Vorneigung, beide Arme zum Stiel, die lange Diagonale des Stiels und der Besenkopf auf dem Boden."),
        t("Thumbs into the fists, hem rib into the hoodie, sock into the sneaker, socket and hanging cap into the handle.", "Daumen in die Fäuste, Saumbund ins Hoodie, Socke in den Sneaker, Hülse und Aufhängekappe in den Stiel."),
        t("Fingers, ear, eye, cuffs, laces, seams, knee folds.", "Finger, Ohr, Auge, Bündchen, Schnürung, Nähte, Kniefalten."),
        t("The handle is a 25 mm rod (0.125 mm) inside its 0.18 mm outline; block and bristles keep their true 40 and 70 mm height.", "Der Stiel ist ein 25 mm starker Stab (0,125 mm) in seinem 0,18-mm-Umriss; Holz und Borsten behalten ihre wahren Höhen von 40 und 70 mm."),
        t("None; fill tones carry the materials.", "Keine; die Fülltöne tragen die Materialien."),
        t("Body pen 0.18 mm, detail pen 0.10 mm for the pocket; light skin, shoes and handle, mid hoodie and block, dark jeans, hair and bristles.", "Körperstift 0,18 mm, Detailstift 0,10 mm für die Tasche; helle Haut, Schuhe und Stiel, mittleres Hoodie und Holz, dunkle Jeans, Haare und Borsten."),
        t("One region per mass; the broom keeps its wood and bristle regions. Front and plan: regions as in the side view.", "Eine Region je Masse; der Besen behält seine Holz- und Borstenregion. Vorderansicht und Grundriss: Regionen wie in der Seitenansicht.")),
    "500": _d(
        t("A leaning figure of 1.72 m (3.4 mm) with its broom, the true orthographic views of the 1:500 body: head with hair as one mass, neck, sweatshirt with the hood merged into the upper back, jeans seat, and per side sleeve, forearm, fist, thigh, shank, shoe and toe box; the handle as a 40 mm rod and the head as block and bristles. Front view (the figure's left is on the right of the sheet): the 400 mm head as a 0.8 mm bar on the floor. Plan: the masses in the order of their height.",
          "Eine vorgeneigte Figur von 1,72 m (3,4 mm) mit ihrem Besen, die wahren orthogonalen Ansichten des 1:500-Körpers: Kopf mit Haar als eine Masse, Hals, Sweatshirt mit im oberen Rücken aufgegangener Kapuze, Jeansgesäß und je Seite Ärmel, Unterarm, Faust, Oberschenkel, Unterschenkel, Schuh und Zehenkappe; der Stiel als 40 mm starker Stab und der Kopf als Holz und Borsten. Vorderansicht (die linke Körperseite liegt rechts im Blatt): der 400 mm breite Kopf als 0,8 mm langer Balken auf dem Boden. Grundriss: die Massen in der Reihenfolge ihrer Höhe."),
        t("The forward lean of the stroke, the diagonal of the handle and the broom head ahead of the feet, the exact stature and extent.", "Die Vorneigung des Strichs, die Diagonale des Stiels und der Besenkopf vor den Füßen, genaue Körpergröße und Ausdehnung."),
        t("Face and hair; hood into the sweatshirt; socket and cap into the handle.", "Gesicht und Haar; Kapuze ins Sweatshirt; Hülse und Kappe in den Stiel."),
        t("Sleeve cuffs, hood fold, laces, sole line, digits.", "Ärmelbündchen, Kapuzenfalte, Schnürung, Sohlenlinie, Finger."),
        t("The 24 mm handle is drawn and modelled as a 40 mm rod so it prints as a firm 0.21 mm band; the drawstrings are not drawn, never thickened.", "Der 24 mm starke Stiel wird als 40 mm starker Stab gezeichnet und modelliert, damit er als kräftiges 0,21-mm-Band druckt; die Kordeln werden nicht gezeichnet, nie verdickt."),
        t("None.", "Keine."),
        t("Mid-grey torso, darker legs, light shoes and handle, dark bristles; line pen 0.13 mm only, no detail pen; light mode dark lines on white, dark mode inverted.", "Mittelgrauer Rumpf, dunklere Beine, helle Schuhe und Stiel, dunkle Borsten; nur Strichstärke 0,13 mm, kein Detailstift; hell: dunkle Linien auf Weiß, dunkel: invertiert."),
        t("Thermal and structural overlays paint the torso shape with the hoodie value; head, hands, legs, shoes and the two broom regions keep their own value. Front and plan as in the side view.", "Thermische und statische Überlagerungen färben die Rumpfform mit dem Hoodie-Wert; Kopf, Hände, Beine, Schuhe und die zwei Besenregionen behalten ihren eigenen Wert. Vorderansicht und Grundriss wie in der Seitenansicht.")),
}

VOLUME_DECISIONS = {
    "50": [t("Broom at 1:50: the 24 mm beech handle as a solid of revolution with its rounded end under a plastic hanging cap, a socket that takes the handle into the head, the 400 x 60 x 40 mm block with 8 mm corner radii and the 384 x 50 x 72 mm bristle pad; the head turns on its swivel so the pad stands flat on the floor. Thermal: handle, cap, socket and block are the broom-wood region, the pad the bristle region. Structure: handle and block are the members of their bones, masses at their centres, the support patch under the bristles and the swivel as a joint.",
             "Besen bei 1:50: der 24 mm starke Buchenstiel als Drehkörper mit gerundetem Ende unter einer Kunststoff-Aufhängekappe, eine Hülse, die den Stiel in den Kopf führt, das 400 x 60 x 40 mm große Holz mit 8 mm Eckradien und das 384 x 50 x 72 mm große Borstenfeld; der Kopf dreht sich in seinem Gelenk, sodass das Feld flach auf dem Boden steht. Thermisch: Stiel, Kappe, Hülse und Holz sind die Region des Besenholzes, das Feld die Borstenregion. Tragwerk: Stiel und Holz sind die Stäbe ihrer Knochen, Massen in ihren Schwerpunkten, die Auflagerfläche unter den Borsten und das Gelenk als Knoten.")],
    "100": [t("Broom at 1:100: handle with a plain end, socket, block with 5 mm corner radii and the bristle pad; the hanging cap is below what the scale shows. Thermal and structure as at 1:50, the swivel joint kept.",
              "Besen bei 1:100: Stiel mit glattem Ende, Hülse, Holz mit 5 mm Eckradien und Borstenfeld; die Aufhängekappe liegt unter dem, was der Maßstab zeigt. Thermisch und Tragwerk wie bei 1:50, das Gelenk bleibt.")],
    "200": [t("Broom at 1:200: a 25 mm slender handle running into the head, block and bristle pad as plain prisms; the socket merges into the handle. Thermal by reference; structure: handle and block as members, masses and the bristle support.",
              "Besen bei 1:200: ein 25 mm starker schlanker Stiel bis in den Kopf, Holz und Borstenfeld als einfache Prismen; die Hülse geht im Stiel auf. Thermisch per Verweis; Tragwerk: Stiel und Holz als Stäbe, Massen und das Borstenauflager.")],
    "500": [t("Broom at 1:500: the handle as a 40 mm slender rod, block and bristles as two prisms of 42 and 70 mm height (the drawn handle sets the thinnest drawing of the broom bones). Thermal by reference; structure: handle and block as members and the bristle support.",
              "Besen bei 1:500: der Stiel als 40 mm starker schlanker Stab, Holz und Borsten als zwei Prismen von 42 und 70 mm Höhe (der gezeichnete Stiel bestimmt die dünnste Zeichnung der Besenknochen). Thermisch per Verweis; Tragwerk: Stiel und Holz als Stäbe und das Borstenauflager.")],
}
