"""📝️ Texts, references, materials and thermal regions of `humans/person-carrying-shopping-bags` (English first, German second)."""
from lib import t

TITLE = t("Person Carrying Shopping Bags", "Person mit Einkaufstaschen")
SUMMARY = t(
    "A man of 1.78 m in a knee-length wool overcoat and scarf walks home from the market with a full paper shopping bag in each fist, "
    "4.5 kg each (a baguette in the left, a leek in the right): the arms hang straight with the load and swing only a little, the bags "
    "clear the legs and pendulum slightly behind the step; a level walk at 4 km/h at about 3.5 met and a pause standing with the bags.",
    "Ein Mann von 1,78 m in knielangem Wollmantel und Schal geht vom Markt nach Hause, in jeder Faust eine volle Papiereinkaufstüte "
    "von je 4,5 kg (links ein Baguette, rechts eine Lauchstange): Die Arme hängen unter der Last gestreckt und schwingen nur wenig, "
    "die Tüten laufen frei neben den Beinen und pendeln leicht hinter dem Schritt her; Gehen in der Ebene mit 4 km/h bei etwa 3,5 met "
    "und eine Pause im Stand mit den Tüten.")

DIMENSIONS = {"heightM": 1.803, "widthM": 0.385, "depthM": 0.782}

BAG_BONE = {"l": t("Paper shopping bag in the left fist", "Papiertüte in der linken Faust"), "r": t("Paper shopping bag in the right fist", "Papiertüte in der rechten Faust")}
GRIP = {"l": t("Left fist round the bag handles", "Linke Faust um die Tütengriffe"), "r": t("Right fist round the bag handles", "Rechte Faust um die Tütengriffe")}

MATERIALS = [
    {"id": "kraft-paper", "title": t("Brown kraft paper, 90 g/m2", "Braunes Kraftpapier, 90 g/m2"), "emissivity": 0.93, "solarAbsorptivity": 0.7, "densityKgM3": 700, "source": "authored-estimate"},
    {"id": "paper-twist", "title": t("Twisted paper handle", "Gedrehter Papiergriff"), "emissivity": 0.93, "solarAbsorptivity": 0.7, "densityKgM3": 700, "source": "authored-estimate"},
    {"id": "bread-crust", "title": t("Baguette crust", "Baguettekruste"), "emissivity": 0.95, "solarAbsorptivity": 0.65, "source": "authored-estimate"},
    {"id": "leek", "title": t("Leek", "Lauch"), "emissivity": 0.97, "solarAbsorptivity": 0.6, "source": "authored-estimate"},
]

REFERENCES = [
    {"id": "ainsworth-2011", "citation": "Ainsworth, B. E. et al. (2011). 2011 Compendium of Physical Activities: a second update of codes and MET values. Medicine and Science in Sports and Exercise 43(8), 1575-1581 (walking 2.5 to 3.0 mph; carrying loads)."},
    {"id": "iso-11228-1", "citation": "ISO 11228-1:2021 Ergonomics, manual handling, lifting, lowering and carrying (carrying in both hands, cumulative mass)."},
]

NOTES = t(
    "Derived by hand from the archive's walking-commuter (rig, anthropometry of a 1.78 m, 72 kg man after de Leva 1996 and Drillis and Contini, "
    "clothing, body volumes and outlines unchanged); the shoulder bag, its strap, its load and the watch-glance pose and clip are removed. New: two "
    "bag bones hung in the fists, 4.5 kg each as bone masses (a paper bag of 0.1 kg with 4.4 kg of groceries), every bag part handcrafted per scale "
    "as one prism that is both the 3D solid and, with its solid placement, the drawing of every view; the arms hang with the load (a third of the free "
    "arm swing, 9 degrees out from the trunk so the bags clear the legs), and each bag is solved to hang plumb with a pendulum lag of 0.35 of the arm "
    "swing (kit.ts on the library's own rig solve). Walking with 9 kg in the hands at 4 km/h is taken as 3.5 met (authored estimate from Ainsworth et al. 2011).",
    "Von Hand aus dem Archivobjekt walking-commuter abgeleitet (Skelett, Anthropometrie eines Mannes von 1,78 m und 72 kg nach de Leva 1996 und "
    "Drillis und Contini, Kleidung, Körpervolumen und Umrisse unverändert); die Umhängetasche, ihr Riemen, ihre Last sowie Pose und Clip des Blicks "
    "auf die Uhr sind entfernt. Neu: zwei Tütenknochen in den Fäusten mit je 4,5 kg Knochenmasse (eine Papiertüte von 0,1 kg mit 4,4 kg Einkäufen), "
    "jedes Tütenteil je Maßstab von Hand als Prisma gebaut, das zugleich 3D-Körper und – mit seiner Körperplatzierung – die Zeichnung jeder Ansicht "
    "ist; die Arme hängen unter der Last (ein Drittel des freien Armschwungs, 9 Grad vom Rumpf abgespreizt, damit die Tüten frei neben den Beinen "
    "laufen), und jede Tüte hängt gelöst lotrecht mit einem Pendelnachlauf von 0,35 des Armschwungs (kit.ts auf der Skelettlösung der Bibliothek). "
    "Gehen mit 9 kg in den Händen bei 4 km/h gilt als 3,5 met (Schätzwert nach Ainsworth et al. 2011).")

REGIONS = [
    {"id": "paper-bags", "title": t("Paper shopping bags", "Papiereinkaufstüten"), "kind": "surface", "model": "passive", "material": "kraft-paper", "areaM2": 0.9, "characteristicLengthM": 0.3, "geometry": "vertical-plate", "skyViewFactor": 0.35, "layers": [], "heatCapacityJM2K": 1500, "source": "authored-estimate"},
    {"id": "groceries", "title": t("Groceries showing above the rims", "Einkäufe über dem Tütenrand"), "kind": "surface", "model": "passive", "material": "bread-crust", "areaM2": 0.08, "characteristicLengthM": 0.06, "geometry": "cylinder", "skyViewFactor": 0.4, "layers": [], "heatCapacityJM2K": 6000, "source": "authored-estimate"},
]

WALK_BREATH = {"id": "walking-loaded", "title": t("Walking at 4 km/h with 9 kg in the hands", "Gehen mit 4 km/h und 9 kg in den Händen"), "rateBpm": 22, "tidalVolumeL": 1.3, "inhaleFraction": 0.45, "pauseAfterInhaleFraction": 0.02, "exhaleFraction": 0.5, "pauseAfterExhaleFraction": 0.03, "route": "nose-and-mouth", "source": "authored-estimate", "reference": "icrp-66-1994"}

THERMAL_ASSUMPTIONS = [
    t("Skin area 1.894 m2 after DuBois and DuBois (1916) for 72 kg and 1.78 m; clothing layers of the overcoat ensemble after ISO 9920 (1 clo = 0.155 m2K/W, enclosed air included), resistances referred to the outer area.",
      "Hautfläche 1,894 m2 nach DuBois und DuBois (1916) für 72 kg und 1,78 m; Kleidungsschichten des Mantelensembles nach ISO 9920 (1 clo = 0,155 m2K/W, eingeschlossene Luft inbegriffen), Widerstände auf die Außenfläche bezogen."),
    t("Cold, overcast winter street at 8 degrees Celsius and 75 % humidity; walking at 4 km/h with two 4.5 kg bags is about 3.5 met (authored estimate after Ainsworth et al. 2011: level walking about 3 met, carrying a load in the hands adds), standing with the bags about 1.5 met; ventilation 22 breaths per minute with 1.3 L.",
      "Kalte, bedeckte Winterstraße bei 8 Grad Celsius und 75 % Feuchte; Gehen mit 4 km/h und zwei Tüten zu je 4,5 kg entspricht etwa 3,5 met (Schätzwert nach Ainsworth et al. 2011: ebenes Gehen etwa 3 met, das Tragen einer Last in den Händen kommt hinzu), Stehen mit den Tüten etwa 1,5 met; Atmung 22 Atemzüge pro Minute mit 1,3 L."),
    t("Bags and groceries are passive surfaces without internal heat; the fists are partly covered by the handles.",
      "Tüten und Einkäufe sind passive Flächen ohne innere Wärme; die Fäuste sind teilweise von den Griffen bedeckt."),
]

STRUCTURE_ASSUMPTIONS = [
    t("Segments are point masses at the de Leva centres (male table) plus clothing; each bag is a 4.5 kg bone mass hanging from its fist, so wrists, elbows and shoulders carry 44 N each and the spine carries the pair symmetrically (ISO 11228-1: 9 kg in two hands is a light carrying task).",
      "Segmente sind Punktmassen in den Schwerpunkten nach de Leva (Männertabelle) plus Kleidung; jede Tüte ist eine Knochenmasse von 4,5 kg an ihrer Faust, sodass Handgelenke, Ellbogen und Schultern je 44 N tragen und die Wirbelsäule das Paar symmetrisch trägt (ISO 11228-1: 9 kg in zwei Händen sind eine leichte Tragaufgabe)."),
    t("Contacts are the heel and the ball of each foot; supports follow the clip contact spans of the gait.",
      "Kontakte sind Ferse und Fußballen jedes Fußes; Auflager folgen den Kontaktspannen des Gangs."),
]

POSES = {
    "standing": (t("Standing with the bags", "Stehen mit den Tüten"), t("Upright, feet at hip width, both arms hanging straight with a full paper bag in each fist, the bags plumb beside the legs.", "Aufrecht, Füße hüftbreit, beide Arme hängen gestreckt mit je einer vollen Papiertüte in der Faust, die Tüten lotrecht neben den Beinen.")),
    "contact-left": (t("Left heel strike", "Linker Fersenkontakt"), t("Left heel strike, right foot rolling off; the arms swing only a little under the load and the bags trail the swing.", "Linker Fersenkontakt, der rechte Fuß rollt ab; die Arme schwingen unter der Last nur wenig, die Tüten pendeln hinterher.")),
    "loading-left": (t("Left loading response", "Linke Belastungsantwort"), t("Weight onto the left leg, both feet on the ground, bags close beside the legs.", "Gewicht auf das linke Bein, beide Füße am Boden, Tüten dicht neben den Beinen.")),
    "passing-left": (t("Left midstance", "Linke Standphasenmitte"), t("Standing on the left leg while the right leg passes; the bags pass the knees at a hand's breadth.", "Stand auf dem linken Bein, während das rechte vorbeischwingt; die Tüten passieren die Knie eine Handbreit entfernt.")),
    "contact-right": (t("Right heel strike", "Rechter Fersenkontakt"), t("Right heel strike, left foot rolling off; mirror of the left heel strike.", "Rechter Fersenkontakt, der linke Fuß rollt ab; Spiegelbild des linken Fersenkontakts.")),
    "loading-right": (t("Right loading response", "Rechte Belastungsantwort"), t("Weight onto the right leg, both feet on the ground.", "Gewicht auf das rechte Bein, beide Füße am Boden.")),
    "passing-right": (t("Right midstance", "Rechte Standphasenmitte"), t("Standing on the right leg while the left leg passes.", "Stand auf dem rechten Bein, während das linke vorbeischwingt.")),
}

CLIPS = {
    "walk-cycle": (t("Walking with shopping bags", "Gehen mit Einkaufstüten"), t("Level walking at about 1.1 m/s (4 km/h), 100 steps per minute, feet planted without sliding; the loaded arms swing a third as far as free arms and the bags trail them a little; about 3.5 met.", "Gehen in der Ebene mit etwa 1,1 m/s (4 km/h), 100 Schritte pro Minute, die Füße gleiten nicht; die belasteten Arme schwingen ein Drittel so weit wie freie Arme, die Tüten pendeln leicht hinterher; etwa 3,5 met.")),
    "idle-breathing": (t("Idle breathing with the bags", "Ruhiges Atmen mit den Tüten"), t("A four-second loop standing with the bags: quiet breathing and a slight sway of head and trunk; the arms stay straight under the load.", "Eine Schleife von vier Sekunden im Stand mit den Tüten: ruhiges Atmen und ein leichtes Wiegen von Kopf und Rumpf; die Arme bleiben unter der Last gestreckt.")),
}


def _d(silhouette, retained, merged, omitted, thin, hatching, styles, overlays):
    return {"silhouette": silhouette, "retained": retained, "merged": merged, "omitted": omitted, "thinElements": thin, "hatching": hatching, "styles": styles, "overlays": overlays}


DECISIONS = {
    "50": _d(
        t("Every shape is the orthographic outline of the 1:50 model standing with the bags, bound to its bone. Side view from the right: overcoat with collar and near pocket, scarf, head with hair, ear and eye, the near arm hanging straight to the fist round the twisted paper handles, the bag pinched at its folded rim, bulging below, the leek standing out of it, trousers and shoes; 36 mm tall. Front view (the figure's left is on the right of the sheet): head and face, scarf, coat with buttons and pockets, both arms a little out from the coat, the bags beside the legs with the baguette on the figure's left and the leek on its right. Plan: the bags as two blocks beside the coat, the groceries inside them.",
          "Jede Form ist der orthogonale Umriss des 1:50-Modells im Stand mit den Tüten, an ihren Knochen gebunden. Seitenansicht von rechts: Mantel mit Kragen und naher Tasche, Schal, Kopf mit Haar, Ohr und Auge, der nahe Arm hängt gestreckt bis zur Faust um die gedrehten Papiergriffe, die Tüte am umgeschlagenen Rand eingeschnürt und darunter gebaucht, mit herausragendem Lauch, Hose und Schuhe; 36 mm hoch. Vorderansicht (die linke Körperseite liegt rechts im Blatt): Kopf und Gesicht, Schal, Mantel mit Knöpfen und Taschen, beide Arme etwas vom Mantel abgespreizt, die Tüten neben den Beinen, das Baguette links der Figur, der Lauch rechts. Grundriss: die Tüten als zwei Blöcke neben dem Mantel, die Einkäufe darin."),
        t("Lapels, buttons, pocket flaps, fingers and thumbs round the handles, both handles of each bag, the folded rim, baguette tip and leek leaves.", "Revers, Knöpfe, Taschenklappen, Finger und Daumen um die Griffe, beide Griffe jeder Tüte, der umgeschlagene Rand, Baguettespitze und Lauchblätter."),
        t("Nose, brow, chin and lips into the head outline; the twisted handle fibres into one strip each; the groceries below the rim into the bag body.", "Nase, Braue, Kinn und Lippen in den Kopfumriss; die gedrehten Griffasern in je einen Streifen; die Einkäufe unter dem Rand in den Tütenkörper."),
        t("Socks, the shop print on the bags, the gusset folds and the bag bottoms in plan (hidden under the groceries).", "Socken, der Ladenaufdruck der Tüten, die Seitenfalten und die Tütenböden im Grundriss (unter den Einkäufen verborgen)."),
        t("The handles are 7 mm strips (0.14 mm) inside their outline; mouth, brows, coat front edge, back vent and seams are detail strokes.", "Die Griffe sind 7 mm breite Streifen (0,14 mm) in ihrem Umriss; Mund, Brauen, Mantelkante, Rückenschlitz und Nähte sind Detailstriche."),
        t("Wool coat, collar and sleeves: parallel lines at 65 degrees, 0.5 mm spacing; trousers, scarf, bags and groceries stay plain tone.", "Wollmantel, Kragen und Ärmel: Parallelen unter 65 Grad, 0,5 mm Abstand; Hose, Schal, Tüten und Einkäufe bleiben reiner Ton."),
        t("Body 0.25 mm, detail 0.13 mm, texture 0.10 mm; kraft paper mid-light, rim and handles a step darker, baguette light, leek dark-tipped.", "Körper 0,25 mm, Detail 0,13 mm, Textur 0,10 mm; Kraftpapier mittelhell, Rand und Griffe eine Stufe dunkler, Baguette hell, Lauch mit dunklen Blättern."),
        t("One region per shape; bag body, rim and handles carry the paper-bag region, baguette and leek the groceries region. Front and plan as in the side view.", "Eine Region je Form; Tütenkörper, Rand und Griffe tragen die Region der Papiertüten, Baguette und Lauch die Region der Einkäufe. Vorderansicht und Grundriss wie in der Seitenansicht.")),
    "100": _d(
        t("Every shape is the orthographic outline of the 1:100 model standing with the bags. Side view from the right: overcoat with collar, scarf, head with hair and ear, the near arm hanging to its fist, the pinched and bulging bag with both handles and the leek, trousers and shoes; 18 mm tall. Front view (the figure's left is on the right of the sheet): head, scarf, coat, both arms and fists, the two bags beside the legs with baguette and leek. Plan: shoes, legs, coat, sleeves, bags with their groceries, scarf, head under the hair.",
          "Jede Form ist der orthogonale Umriss des 1:100-Modells im Stand mit den Tüten. Seitenansicht von rechts: Mantel mit Kragen, Schal, Kopf mit Haar und Ohr, der nahe Arm hängt bis zur Faust, die eingeschnürte und gebauchte Tüte mit beiden Griffen und dem Lauch, Hose und Schuhe; 18 mm hoch. Vorderansicht (die linke Körperseite liegt rechts im Blatt): Kopf, Schal, Mantel, beide Arme und Fäuste, die zwei Tüten neben den Beinen mit Baguette und Lauch. Grundriss: Schuhe, Beine, Mantel, Ärmel, Tüten mit ihren Einkäufen, Schal, Kopf unter dem Haar."),
        t("Lapels, collar, ears, the thumbs, both handles of each bag, baguette and leek with its leaves.", "Revers, Kragen, Ohren, die Daumen, beide Griffe jeder Tüte, Baguette und Lauch mit seinen Blättern."),
        t("Fingers into the fists, the folded rim into the bag outline, sole and heel into the shoe.", "Finger in die Fäuste, der umgeschlagene Rand in den Tütenumriss, Sohle und Absatz in den Schuh."),
        t("Buttons, pockets, eyes, mouth, the baguette tip and the bag print.", "Knöpfe, Taschen, Augen, Mund, die Baguettespitze und der Tütenaufdruck."),
        t("The 7 mm handles are drawn at true width (0.07 mm) inside the 0.2 mm body pen, so each reads as one firm line; the coat front edge is a detail stroke.", "Die 7 mm breiten Griffe sind in wahrer Breite (0,07 mm) im 0,2-mm-Körperstift gezeichnet und lesen sich je als eine kräftige Linie; die Mantelkante ist ein Detailstrich."),
        t("Wool coat only: fine parallel lines at 65 degrees, 0.6 mm spacing, 0.10 mm pen; bags plain tone.", "Nur der Wollmantel: feine Parallelen unter 65 Grad, 0,6 mm Abstand, 0,10-mm-Stift; Tüten reiner Ton."),
        t("Body pen 0.2 mm, detail 0.1 mm; coat mid-grey with hatch, trousers darker, shoes dark, bags mid-light, handles darker.", "Körperstift 0,2 mm, Detail 0,1 mm; Mantel mittelgrau mit Schraffur, Hose dunkler, Schuhe dunkel, Tüten mittelhell, Griffe dunkler."),
        t("Regions as at 1:50: paper bags and groceries as two regions. Front and plan: regions as in the side view.", "Regionen wie bei 1:50: Papiertüten und Einkäufe als zwei Regionen. Vorderansicht und Grundriss: Regionen wie in der Seitenansicht.")),
    "200": _d(
        t("Every shape is the orthographic outline of the 1:200 model standing with the bags. Side view from the right: coat, scarf ring, head with hair, the hanging near arm and fist, the bag as a pinched block with one handle form and the leek, trousers and shoes; 9 mm tall. Front view (the figure's left is on the right of the sheet): head, scarf, coat, both arms and the two bags beside the legs. Plan: shoes, legs, coat, sleeves, bags, scarf and head.",
          "Jede Form ist der orthogonale Umriss des 1:200-Modells im Stand mit den Tüten. Seitenansicht von rechts: Mantel, Schalring, Kopf mit Haar, der hängende nahe Arm mit Faust, die Tüte als eingeschnürter Block mit einer Griffform und dem Lauch, Hose und Schuhe; 9 mm hoch. Vorderansicht (die linke Körperseite liegt rechts im Blatt): Kopf, Schal, Mantel, beide Arme und die zwei Tüten neben den Beinen. Grundriss: Schuhe, Beine, Mantel, Ärmel, Tüten, Schal und Kopf."),
        t("Both bags, their handles and the groceries standing out of them, so the load reads.", "Beide Tüten, ihre Griffe und die herausragenden Einkäufe, damit die Last lesbar ist."),
        t("Lapels and collar into the coat, nose and ears into the head, fingers into the fists, the rim into the bag, the two handles of a bag into one tapered form (two 12 mm strips 44 mm apart would leave a gap below the print minimum).", "Revers und Kragen in den Mantel, Nase und Ohren in den Kopf, Finger in die Fäuste, der Rand in die Tüte, die zwei Griffe einer Tüte in eine verjüngte Form (zwei 12 mm breite Streifen im Abstand von 44 mm ließen eine Lücke unter dem Druckminimum)."),
        t("Buttons, pockets, face details, the baguette tip.", "Knöpfe, Taschen, Gesichtsdetails, die Baguettespitze."),
        t("The handle form tapers from the 24 mm fist to the 120 mm rim and is 60 mm thick, so it prints as one mass.", "Die Griffform verjüngt sich von 24 mm an der Faust auf 120 mm am Rand und ist 60 mm dick, damit sie als eine Masse druckt."),
        t("None; fill tones carry the material.", "Keine; die Fülltöne tragen das Material."),
        t("Body pen 0.18 mm for masses, detail pen 0.10 mm for the coat front; light fills for skin and bags, mid for coat, dark for shoes and hair.", "Körperstift 0,18 mm für Massen, Detailstift 0,10 mm für die Mantelkante; helle Füllungen für Haut und Tüten, mittel für den Mantel, dunkel für Schuhe und Haar."),
        t("Every filled shape carries the region of the garment, body part, bag or groceries it depicts in all three views.", "Jede gefüllte Form trägt in allen drei Ansichten die Region des Kleidungsstücks, Körperteils, der Tüte oder der Einkäufe, die sie zeigt.")),
    "500": _d(
        t("Every shape is the orthographic outline of the 1:500 model standing with the bags: coat, scarf, head with hair, sleeves, fists, legs, one shoe per foot and a bag block on each side with one handle form and the groceries; 3.6 mm tall. Front view: the figure's left is on the right of the sheet, the bags widen the figure to 0.78 m. Plan: shoes, legs, coat, sleeves, bags, scarf and head as a few masses.",
          "Jede Form ist der orthogonale Umriss des 1:500-Modells im Stand mit den Tüten: Mantel, Schal, Kopf mit Haar, Ärmel, Fäuste, Beine, ein Schuh je Fuß und je Seite ein Tütenblock mit einer Griffform und den Einkäufen; 3,6 mm hoch. Vorderansicht: Die linke Körperseite liegt rechts im Blatt, die Tüten verbreitern die Figur auf 0,78 m. Grundriss: Schuhe, Beine, Mantel, Ärmel, Tüten, Schal und Kopf als wenige Massen."),
        t("The two bags hanging from the straight arms keep the figure recognisable as someone carrying shopping.", "Die zwei Tüten an den gestreckten Armen machen die Figur als einkaufstragende Person erkennbar."),
        t("The two handles of a bag into one tapered form between rim and fist; forefoot and heel into one shoe; fingers into the fists.", "Die zwei Griffe einer Tüte in eine verjüngte Form zwischen Rand und Faust; Vorfuß und Ferse in einen Schuh; Finger in die Fäuste."),
        t("Lapels, collar, buttons, pockets, face, the bag pinch and the leek leaves; in the side view the body of the far bag, which the near bag covers up to a sliver that would not print.", "Revers, Kragen, Knöpfe, Taschen, Gesicht, die Einschnürung der Tüte und die Lauchblätter; in der Seitenansicht der Körper der fernen Tüte, den die nahe Tüte bis auf einen nicht druckbaren Streifen verdeckt."),
        t("The handle form is 60 mm thick so it prints; the groceries are drawn at their true 60 to 70 mm diameter.", "Die Griffform ist 60 mm dick, damit sie druckt; die Einkäufe sind in ihrem wahren Durchmesser von 60 bis 70 mm gezeichnet."),
        t("None.", "Keine."),
        t("Mid-tone coat mass, light bags, dark shoes; line pen 0.13 mm, no detail pen; light mode: dark line on white, dark mode: inverted.", "Mittlerer Mantelton, helle Tüten, dunkle Schuhe; Strichstärke 0,13 mm, kein Detailstift; hell: dunkle Linie auf Weiß, dunkel: invertiert."),
        t("Every filled shape carries the region it depicts in all three views, the bags and groceries their own two regions.", "Jede gefüllte Form trägt in allen drei Ansichten die Region, die sie zeigt, Tüten und Einkäufe ihre eigenen zwei Regionen.")),
}

VOLUME_DECISIONS = {
    "50": [t("Bags at 1:50: each a kraft-paper body as one prism of its side outline (pinched to 60 mm at the rim, bulging to 180 mm, 320 mm wide, 315 mm tall), the folded rim band, two twisted handles of 7 mm as arch strips up into the fist; a baguette (60 mm) with its tip in the left bag and a leek (40 mm) with its leaves in the right. Thermal: body, rim and handles are the paper-bag region, baguette and leek the groceries region. Structure: body and handles are the members of the bag bone, the 4.5 kg mass at the centre of the groceries, the grip as a joint.",
             "Tüten bei 1:50: jede ein Kraftpapierkörper als ein Prisma ihres Seitenumrisses (am Rand auf 60 mm eingeschnürt, auf 180 mm gebaucht, 320 mm breit, 315 mm hoch), der umgeschlagene Randstreifen, zwei gedrehte Griffe von 7 mm als Bogenstreifen bis in die Faust; ein Baguette (60 mm) mit seiner Spitze in der linken Tüte und ein Lauch (40 mm) mit seinen Blättern in der rechten. Thermisch: Körper, Rand und Griffe sind die Region der Papiertüten, Baguette und Lauch die Region der Einkäufe. Tragwerk: Körper und Griffe sind die Stäbe des Tütenknochens, die 4,5 kg Masse im Schwerpunkt der Einkäufe, der Griff als Gelenk.")],
    "100": [t("Bags at 1:100: body with a simpler side outline, two 7 mm handles, baguette, leek and leek leaves; the rim band is merged into the body. Thermal and structure as at 1:50.",
              "Tüten bei 1:100: Körper mit einfacherem Seitenumriss, zwei 7 mm Griffe, Baguette, Lauch und Lauchblätter; der Randstreifen geht im Körper auf. Thermisch und Tragwerk wie bei 1:50.")],
    "200": [t("Bags at 1:200: the body as a pinched block, both handles merged into one tapered form of 60 mm, baguette and leek with a single leaf form. Thermal by reference; structure: body and handles as members, the bag masses.",
              "Tüten bei 1:200: der Körper als eingeschnürter Block, beide Griffe in eine verjüngte Form von 60 mm zusammengefasst, Baguette und Lauch mit einer einzigen Blattform. Thermisch per Verweis; Tragwerk: Körper und Griffe als Stäbe, die Tütenmassen.")],
    "500": [t("Bags at 1:500: the body as a block of 320 x 180 x 330 mm, both handles merged into one tapered form of 60 mm between rim and fist, baguette and leek as 70 and 60 mm rods. Thermal by reference; structure: body and handle form as members, the bag masses.",
              "Tüten bei 1:500: der Körper als Block von 320 x 180 x 330 mm, beide Griffe in eine verjüngte Form von 60 mm zwischen Rand und Faust zusammengefasst, Baguette und Lauch als Stäbe von 70 und 60 mm. Thermisch per Verweis; Tragwerk: Körper und Griffform als Stäbe, die Tütenmassen.")],
}
