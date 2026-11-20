"""📝 Texts of the round-3 root fixes of `humans/worker-carrying-box` (English first, German second): summary, provenance, pose and clip descriptions, structural assumptions and the decisions of the redrawn 2D representations."""

SUMMARY = {
    "en": "A construction worker in a high-visibility vest and hard hat lifts a 20 kg carton by its hand holes from the floor in a squat and carries it at hip or chest height; trunk, shoulder and elbow moments change with the lever arm of the load.",
    "de": "Eine Bauarbeiterin bzw. ein Bauarbeiter mit Warnweste und Schutzhelm hebt einen 20-kg-Karton an den Grifflöchern in der Hocke vom Boden und trägt ihn auf Hüft- oder Brusthöhe; Rumpf-, Schulter- und Ellbogenmomente ändern sich mit dem Hebelarm der Last.",
}

NOTES = {
    "en": "Round 3: one body, modelled and drawn. Stature 1.741 m (the mean of de Leva's subjects) on 30 mm boot soles under a 44 mm helmet crown; joint heights after Winter (hip 0.523 H, knee 0.282 H, ankle 0.038 H, shoulder joint 0.818 H, C7 0.8605 H) with the lumbosacral joint 45 mm behind the hip joints; shoulder joints 0.36 m apart under a biacromial breadth of 0.41 m; girths of a stocky 84 kg man after ANSUR II and DIN 33402-2. The carton is 0.40 m wide, 0.30 m deep and 0.30 m high with a hand hole in each end panel; it is two 10 kg half-bones, one per hand, because both hands coincide in the side elevation. The hands hook into the hand holes in every pose; the arm angles are solved onto them. Every 2D view is the orthographic view of that scale's 3D body.",
    "de": "Runde 3: ein Körper, modelliert und gezeichnet. Körperhöhe 1,741 m (Mittel der Probanden von de Leva) auf 30 mm Stiefelsohle unter 44 mm Helmscheitel; Gelenkhöhen nach Winter (Hüfte 0,523 H, Knie 0,282 H, Knöchel 0,038 H, Schultergelenk 0,818 H, C7 0,8605 H), das Lumbosakralgelenk 45 mm hinter den Hüftgelenken; Schultergelenke 0,36 m auseinander unter 0,41 m Schulterbreite (biakromial); Umfänge eines kräftigen 84-kg-Mannes nach ANSUR II und DIN 33402-2. Der Karton ist 0,40 m breit, 0,30 m tief und 0,30 m hoch mit einem Griffloch in jeder Stirnwand; er besteht aus zwei 10-kg-Halbknochen, je einer pro Hand, weil beide Hände in der Seitenansicht zusammenfallen. Die Hände greifen in jeder Pose in die Grifflöcher; die Armwinkel sind darauf gelöst. Jede 2D-Ansicht ist die orthografische Ansicht des 3D-Körpers ihres Maßstabs.",
}

REFERENCES = [
    {"id": "winter-2009", "citation": "Winter, D. A. (2009). Biomechanics and Motor Control of Human Movement, 4th ed., Wiley, table 4.1 and figure 4.1 (segment lengths and joint heights as fractions of stature, after Drillis and Contini 1966)."},
    {"id": "gordon-2014", "citation": "Gordon, C. C. et al. (2014). 2012 Anthropometric Survey of U.S. Army Personnel: Methods and Summary Statistics (ANSUR II). Natick TR-15/007 (male breadths, depths, girths, head, hand and foot dimensions)."},
    {"id": "din-33402-2", "citation": "DIN 33402-2:2020 Ergonomie, Körpermaße des Menschen, Teil 2: Werte."},
    {"id": "en-iso-20345", "citation": "EN ISO 20345:2022 Personal protective equipment, safety footwear (toe cap, upper height of design B ankle boots, cleated outsole)."},
    {"id": "fefco-0201", "citation": "FEFCO/ESBO International fibreboard case code, 12th ed. (2022), style 0201 regular slotted container with die-cut hand holes; double-wall board BC about 6.5 mm."},
]

BOX_BONE = {
    "l": {"en": "Carton half hanging on the left hand", "de": "Kartonhälfte an der linken Hand"},
    "r": {"en": "Carton half hanging on the right hand", "de": "Kartonhälfte an der rechten Hand"},
}

MATERIALS = {}

POSES = {
    "squat-grip": {
        "en": "Squat with a straight back and almost straight arms; both hands are hooked into the hand holes of the 20 kg carton, which still rests on the ground in front of the toes and shares its weight with the floor.",
        "de": "Hocke mit geradem Rücken und fast gestreckten Armen; beide Hände greifen in die Grifflöcher des 20-kg-Kartons, der noch vor den Fußspitzen auf dem Boden steht und einen Teil seines Gewichts an den Boden abgibt.",
    },
    "lift-mid": {
        "en": "Half-risen, trunk flexed 50 degrees: the carton hangs on the hooked hands in front of the knees, far in front of the lumbar joint, the largest lever arm of the lift.",
        "de": "Halb aufgerichtet, Rumpf 50 Grad vorgeneigt: der Karton hängt an den eingehakten Händen vor den Knien, weit vor dem Lendengelenk, der größte Hebelarm des Hebens.",
    },
    "carrying-box-low": {
        "en": "Upright, trunk leaning back 6 degrees to balance the carton, which hangs at hip height in front of the pelvis on almost straight arms, the hands hooked into its hand holes.",
        "de": "Aufrecht, Rumpf 6 Grad zurückgelehnt als Ausgleich für den Karton, der auf Hüfthöhe vor dem Becken an fast gestreckten Armen hängt, die Hände in den Grifflöchern eingehakt.",
    },
    "carrying-box-chest": {
        "en": "Carton held close at chest height with the elbows bent and raised to the sides, the hands hooked into the hand holes from above: shorter lever arm at the lumbar joint, longer one at the shoulder and elbow.",
        "de": "Karton nah vor der Brust mit gebeugten, seitlich angehobenen Ellbogen, die Hände von oben in die Grifflöcher eingehakt: kürzerer Hebelarm am Lendengelenk, längerer an Schulter und Ellbogen.",
    },
    "setting-down": {
        "en": "Bent knees and flexed trunk while lowering the carton to a pallet; the carton hangs free on the hooked hands in front of the knees.",
        "de": "Gebeugte Knie und vorgeneigter Rumpf beim Absenken des Kartons auf eine Palette; der Karton hängt frei an den eingehakten Händen vor den Knien.",
    },
}

CLIPS = {
    "carry-walk": {
        "en": "Level walking with the 20 kg carton at hip height on almost straight arms, held a hand's breadth higher and further ahead than in the stance so that the thighs clear it, shortened stride, planted feet.",
        "de": "Gehen in der Ebene mit dem 20-kg-Karton auf Hüfthöhe an fast gestreckten Armen, eine Handbreit höher und weiter vorn gehalten als im Stand, damit die Oberschenkel frei schwingen, verkürzter Schritt, Füße gleiten nicht.",
    },
}

ASSUMPTIONS = [
    {
        "en": "Segment masses are point masses at de Leva centres of mass; hand centres are authored estimates; rotational inertia is neglected.",
        "de": "Segmentmassen sind Punktmassen in den Schwerpunkten nach de Leva; Handschwerpunkte sind Schätzwerte; Drehträgheit wird vernachlässigt.",
    },
    {
        "en": "The 20 kg carton is two 10 kg point masses at the hand holes, where the hands carry it (NIOSH load constant 23 kg); while the carton rests on the floor its base contact shares the weight with the feet under the equal-stiffness assumption.",
        "de": "Der 20-kg-Karton besteht aus zwei 10-kg-Punktmassen an den Grifflöchern, wo die Hände ihn tragen (NIOSH-Lastkonstante 23 kg); steht er am Boden, teilt sein Bodenkontakt das Gewicht unter der Annahme gleicher Steifigkeit mit den Füßen.",
    },
    {
        "en": "Both arms coincide in the side elevation: each arm carries its own half of the carton through its own half bone, and the two half bones share one frame in every pose.",
        "de": "Beide Arme fallen in der Seitenansicht zusammen: jeder Arm trägt seine Kartonhälfte über seinen eigenen Halbknochen, und beide Halbknochen teilen in jeder Pose ein Achsensystem.",
    },
]

DECISIONS_2D = {
    "50": {
        "silhouette": {"en": "Every view is the orthographic view of the 1:50 body in the carrying stance: side elevation with profile face, hard hat with peak and crown ridge, shirt collar, vest with hem, reflective bands and brace, cargo pocket, knee pad, boots with sole and toe sole, both arms (the far arm behind the carton) and the gloves hooked into the hand holes, the carton with its right hand hole and tape. Front view (the figure's left is on the right of the sheet): face with eyes, brows, nose and mouth, ears, the vest with zip, bands and braces over the shirt, sleeves, glove cuffs, gloves at the carton ends, trousers with pockets and knee pads, boots and the carton with its label. Plan: hard hat, shoulders in the vest, sleeves and forearms reaching down to the gloves at the carton ends, the carton lid with its tape, boot toes. Sleeves, forearms and cuffs are revolved profiles and gloves and carton extruded profiles of their bones, so the turned arms and the tilted carton stay right in every pose.", "de": "Jede Ansicht ist die orthografische Ansicht des 1:50-Körpers in der Tragehaltung: Seitenansicht mit Gesicht im Profil, Schutzhelm mit Schirm und Scheitelgrat, Hemdkragen, Weste mit Saum, Reflexstreifen und Träger, Beintasche, Knieschoner, Stiefel mit Sohle und Zehensohle, beide Arme (der ferne Arm hinter dem Karton) und die in die Grifflöcher gehakten Handschuhe, der Karton mit rechtem Griffloch und Klebeband. Vorderansicht (die linke Körperseite liegt rechts im Blatt): Gesicht mit Augen, Brauen, Nase und Mund, Ohren, die Weste mit Reißverschluss, Streifen und Trägern über dem Hemd, Ärmel, Handschuhstulpen, Handschuhe an den Kartonenden, Hose mit Taschen und Knieschonern, Stiefel und der Karton mit Etikett. Grundriss: Schutzhelm, Schultern in der Weste, Ärmel und Unterarme bis zu den Handschuhen an den Kartonenden, der Kartondeckel mit Klebeband, Stiefelspitzen. Ärmel, Unterarme und Stulpen sind Drehprofile, Handschuhe und Karton Strangprofile ihrer Knochen, sodass gedrehte Arme und der geneigte Karton in jeder Pose stimmen."},
        "retained": {"en": "Face, ears, both arms and legs, hand holes, tape, label, knee pads and pockets.", "de": "Gesicht, Ohren, beide Arme und Beine, Grifflöcher, Klebeband, Etikett, Knieschoner und Taschen."},
        "merged": {"en": "Fingers and thumb into the glove outline (they lie in the hand hole and behind the palm in every view).", "de": "Finger und Daumen in den Handschuhumriss (sie liegen in jeder Ansicht im Griffloch und hinter dem Handteller)."},
        "omitted": {"en": "Laces and helmet headband (inside the outline), cloth folds.", "de": "Schnürsenkel und Helmkopfband (innerhalb des Umrisses), Stofffalten."},
        "thinElements": {"en": "Mouth, jaw, zip and collar lines 0.13 to 0.25 mm; reflective bands drawn at their 50 mm width.", "de": "Mund-, Kiefer-, Reißverschluss- und Kragenlinien 0,13 bis 0,25 mm; Reflexstreifen in ihrer Breite von 50 mm gezeichnet."},
    },
    "100": {
        "silhouette": {"en": "Every view is the orthographic view of the 1:100 body: hard hat with peak and ridge, face with nose and ear, shirt yoke, vest with hem, bands and brace, trousers, boots with sole, revolved sleeves and forearms with glove cuffs, mitten gloves, the carton with hand hole and tape. Front view (the figure's left is on the right of the sheet) and plan show the same parts from ahead and from above.", "de": "Jede Ansicht ist die orthografische Ansicht des 1:100-Körpers: Schutzhelm mit Schirm und Grat, Gesicht mit Nase und Ohr, Hemdpasse, Weste mit Saum, Streifen und Träger, Hose, Stiefel mit Sohle, gedrehte Ärmel und Unterarme mit Handschuhstulpen, Fäustlingshandschuhe, der Karton mit Griffloch und Klebeband. Vorderansicht (die linke Körperseite liegt rechts im Blatt) und Grundriss zeigen dieselben Teile von vorn und von oben."},
        "retained": {"en": "Both arms (the far arm hatched), both legs, nose, ears, mouth and jaw lines, hand hole and tape.", "de": "Beide Arme (der ferne schraffiert), beide Beine, Nase, Ohren, Mund- und Kieferlinie, Griffloch und Klebeband."},
        "merged": {"en": "Fingers and thumb into one mitten outline; knee into the shank.", "de": "Finger und Daumen in einen Fäustlingsumriss; Knie in den Unterschenkel."},
        "omitted": {"en": "Eyes, brows, knee pads, pockets, label, laces, zip.", "de": "Augen, Brauen, Knieschoner, Taschen, Etikett, Schnürung, Reißverschluss."},
        "thinElements": {"en": "Reflective bands as 0.5 mm wide outlined strips (filled strips would leave slivers under the forearms in the chest carry); brace 0.25 mm; mouth and jaw lines 0.15 mm.", "de": "Reflexstreifen als 0,5 mm breite umrissene Streifen (gefüllte Streifen ließen beim Tragen vor der Brust Splitter unter den Unterarmen); Träger 0,25 mm; Mund- und Kieferlinie 0,15 mm."},
    },
    "200": {
        "silhouette": {"en": "Every view is the orthographic view of the 1:200 body: hard hat with peak, head, shirt collar, vest with hem and two bands, trousers, boots, revolved sleeves and forearms with cuffs, mittens and the carton block with its hand hole. Front view (the figure's left is on the right of the sheet) and plan from the same masses.", "de": "Jede Ansicht ist die orthografische Ansicht des 1:200-Körpers: Schutzhelm mit Schirm, Kopf, Hemdkragen, Weste mit Saum und zwei Streifen, Hose, Stiefel, gedrehte Ärmel und Unterarme mit Stulpen, Fäustlinge und der Kartonblock mit Griffloch. Vorderansicht (die linke Körperseite liegt rechts im Blatt) und Grundriss aus denselben Massen."},
        "retained": {"en": "Both arms and legs, helmet peak, vest bands, hand hole.", "de": "Beide Arme und Beine, Helmschirm, Weststreifen, Griffloch."},
        "merged": {"en": "Face, nose and ears into the head; hair into the head outline; shirt into the collar above the vest.", "de": "Gesicht, Nase und Ohren in den Kopf; Haar in den Kopfumriss; Hemd in den Kragen über der Weste."},
        "omitted": {"en": "Ridge, braces, pockets, knee pads, tape, label, face lines.", "de": "Grat, Träger, Taschen, Knieschoner, Klebeband, Etikett, Gesichtslinien."},
        "thinElements": {"en": "Reflective bands as 0.25 mm wide outlined strips (50 mm real).", "de": "Reflexstreifen als 0,25 mm breite umrissene Streifen (real 50 mm)."},
    },
    "500": {
        "silhouette": {"en": "Every view is the orthographic view of the 1:500 body: hard hat dome, head, shirt collar, vest trunk, seat, two-part legs with boots, revolved two-part arms with fists, the carton block. The figure with the box in front of the thighs on hanging arms reads at a glance. Front view: the figure's left is on the right of the sheet.", "de": "Jede Ansicht ist die orthografische Ansicht des 1:500-Körpers: Helmkuppel, Kopf, Hemdkragen, Westenrumpf, Gesäß, zweiteilige Beine mit Stiefeln, gedrehte zweiteilige Arme mit Fäusten, der Kartonblock. Die Figur mit dem Karton vor den Oberschenkeln an hängenden Armen ist auf einen Blick lesbar. Vorderansicht: die linke Körperseite liegt rechts im Blatt."},
        "retained": {"en": "Hard hat, collar, both arms and legs, boots, carton.", "de": "Schutzhelm, Kragen, beide Arme und Beine, Stiefel, Karton."},
        "merged": {"en": "Helmet brim into the dome; neck into the head; in the front view the face into the helmet piece and the seat behind the carton (a face strip under the brim and the seat between the thighs would be slivers under 0.2 mm in the squat); vest and bands into one trunk; gloves into fists.", "de": "Helmrand in die Kuppel; Hals in den Kopf; in der Vorderansicht das Gesicht in das Helmteil und das Gesäß hinter den Karton (ein Gesichtsstreifen unter dem Rand und das Gesäß zwischen den Oberschenkeln wären in der Hocke Splitter unter 0,2 mm); Weste und Streifen in einen Rumpf; Handschuhe in Fäuste."},
        "omitted": {"en": "Face, ears, peak, bands, cuffs, hand holes, tape.", "de": "Gesicht, Ohren, Schirm, Streifen, Stulpen, Grifflöcher, Klebeband."},
        "thinElements": {"en": "No element is thinner than 75 mm (0.15 mm on paper).", "de": "Kein Element ist dünner als 75 mm (0,15 mm auf dem Papier)."},
    },
}

DECISIONS_3D = {
    "50": [
        {
            "en": "Body 1:50, every section stated by hand for a 7.5 mm feature: skull loft with jaw and chin, nose, lips, eyes, brows and ears, hair at the nape, neck; hard hat as dome with rim, front peak, crown ridge and headband; long-sleeve work shirt with collar under a class-2 hi-vis vest whose hem sits on the pelvis so that it follows the hips in the lift, two reflective bands, braces and zip as surface texture; trouser seat with gluteal masses, thighs with cargo pockets, knee pads, knees and shanks with the trouser leg over the boot; safety boots of shaft, vamp, 30 mm cleated sole, steel toe cap and toe sole, laces; sleeves with the deltoid mass tapering to the elbow, forearms, leather glove cuffs, gloves with palm, thenar, four three-segment fingers hooked into the hand hole and a two-segment thumb; the carton as a board sleeve of 7.5 mm double-wall board and two end panels with die-cut hand holes, tape over the lid and down the ends, a label.",
            "de": "Körper 1:50, jeder Schnitt von Hand für 7,5 mm Mindestmaß angegeben: Schädel-Loft mit Kiefer und Kinn, Nase, Lippen, Augen, Brauen und Ohren, Haar im Nacken, Hals; Schutzhelm als Kuppel mit Rand, Schirm, Scheitelgrat und Kopfband; langärmliges Arbeitshemd mit Kragen unter einer Warnweste Klasse 2, deren Saum am Becken sitzt, damit er beim Heben der Hüfte folgt, zwei Reflexstreifen, Träger und Reißverschluss als Oberflächentextur; Hosengesäß mit Gesäßmassen, Oberschenkel mit Beintaschen, Knieschoner, Knie und Unterschenkel mit dem Hosenbein über dem Stiefel; Sicherheitsstiefel aus Schaft, Blatt, 30 mm Profilsohle, Stahlkappe und Zehensohle, Schnürung; Ärmel mit der zum Ellbogen schmaler werdenden Deltamuskelmasse, Unterarme, Lederstulpen, Handschuhe mit Handteller, Daumenballen, vier dreigliedrigen Fingern, die ins Griffloch greifen, und zweigliedrigem Daumen; der Karton als Mantel aus 7,5 mm Doppelwellpappe und zwei Stirnwände mit gestanzten Grifflöchern, Klebeband über dem Deckel und die Stirnseiten hinab, ein Etikett.",
        },
        {
            "en": "Thermal 1:50: every region with its layers as nested shells of their real thickness (helmet over a hair layer and the harness air gap, shirt over air and T-shirt at the yoke, vest over shirt, T-shirt and air on the trunk, sleeves over air, gloves over lining and air, trousers over briefs and air at the seat, boots over air and work socks); the bare face as its own solid ahead of the ear line; breath outlets under the nostrils and on the lips, the body plume as a patch on the helmet crown; the carton surface as its closed box.",
            "de": "Thermik 1:50: jede Region mit ihren Schichten als verschachtelte Schalen in wahrer Dicke (Helm über Haarschicht und Luftspalt der Innenausstattung, Hemd über Luft und T-Shirt an der Passe, Weste über Hemd, T-Shirt und Luft am Rumpf, Ärmel über Luft, Handschuhe über Futter und Luft, Hose über Unterhose und Luft am Gesäß, Stiefel über Luft und Arbeitssocken); das unbedeckte Gesicht als eigener Körper vor der Ohrlinie; Atemauslässe unter den Nasenlöchern und an den Lippen, die Wärmefahne als Fleck auf dem Helmscheitel; die Kartonoberfläche als sein geschlossener Quader.",
        },
        {
            "en": "Structure 1:50: the anatomical skeleton: cranium, facial skeleton and mandible, cervical spine, thoracolumbar spine with lordosis and kyphosis and spinous processes, twelve ribs per side with the costal arch, sternum, clavicles, scapulae with spine and glenoid, sacrum, hip bones with ischial and pubic rami, femur with head, neck, trochanter and condyles, patella, tibia and fibula, talus, calcaneus, tarsus, metatarsals and toes, humerus, ulna and radius, carpus, metacarpals and phalanges hooked like the glove; joints at hip, knee, ankle, shoulder, elbow, wrist, L5/S1 and C7; masses of trunk, pelvis and thighs; the carton walls carry the 20 kg into both hand holes, with one 10 kg mass per half.",
            "de": "Tragwerk 1:50: das anatomische Skelett: Hirnschädel, Gesichtsschädel und Unterkiefer, Halswirbelsäule, Brust-Lenden-Wirbelsäule mit Lordose, Kyphose und Dornfortsätzen, zwölf Rippen je Seite mit Rippenbogen, Brustbein, Schlüsselbeine, Schulterblätter mit Gräte und Gelenkpfanne, Kreuzbein, Hüftbeine mit Sitz- und Schambeinästen, Oberschenkelknochen mit Kopf, Hals, Rollhügel und Kondylen, Kniescheibe, Schien- und Wadenbein, Sprung-, Fersen- und Fußwurzelknochen, Mittelfußknochen und Zehen, Oberarmknochen, Elle und Speiche, Handwurzel, Mittelhand und Fingerglieder, gekrümmt wie der Handschuh; Gelenke an Hüfte, Knie, Knöchel, Schulter, Ellbogen, Handgelenk, L5/S1 und C7; Massen von Rumpf, Becken und Oberschenkeln; die Kartonwände leiten die 20 kg in beide Grifflöcher, je Hälfte eine 10-kg-Masse.",
        },
    ],
}
