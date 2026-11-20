"""📝 Round 3 red squirrel: the corrected texts, dimensions, material and reference of the asset."""


def t(en, de):
    return {"en": en, "de": de}


SUMMARY = t(
    "An adult Eurasian red squirrel (Sciurus vulgaris, 0.30 kg, head and body 0.21 m, tail 0.19 m with its hair): two-part spine, bushy two-part tail, fore limbs with long fingers and long plantigrade hind feet; sitting upright eating a nut, a bounding run and climbing a vertical trunk (the trunk is not part of the asset: the claws are supports with a bark friction of 1.5); fur thermal model with 38.5 degC core, nose breathing and a back plume.",
    "Ein erwachsenes Europäisches Eichhörnchen (Sciurus vulgaris, 0,30 kg, Kopf-Rumpf-Länge 0,21 m, Schwanz mit Haar 0,19 m): zweiteilige Wirbelsäule, buschiger zweiteiliger Schwanz, Vorderglieder mit langen Fingern und lange Hinterfüße eines Sohlengängers; aufrecht sitzend beim Nussfressen, ein Sprunglauf und Klettern an einem senkrechten Stamm (der Stamm ist nicht Teil des Assets: die Krallen sind Auflager mit Rindenreibung 1,5); Fell-Wärmemodell mit 38,5 Grad Kern, Nasenatmung und Rückenfahne.",
)
DIMENSIONS = {"heightM": 0.209, "widthM": 0.151, "depthM": 0.068}
NEW_POSES = {"bound-support": ("bound-extension", t("Bound, hands carry", "Sprung, Hände tragen")), "bound-reach": ("bound-swing", t("Bound, hind feet reach", "Sprung, Hinterfüße greifen vor"))}
POSE_DESCRIPTIONS = {
    "sitting-eating": t(
        "Sitting on the haunches with both hind feet flat and the knees up beside the belly, the back rounded, both hands at the mouth, the head bent forward, the tail raised behind the back with its tip curling away.",
        "Auf den Keulen sitzend, beide Hinterfüße flach, die Knie neben dem Bauch hochgezogen, der Rücken rund, beide Hände am Maul, der Kopf nach vorn geneigt, der Schwanz hinter dem Rücken aufgestellt, die Spitze nach hinten gebogen.",
    ),
    "climbing": t(
        "Head up on a vertical trunk whose bark is the plane x = 0.038 m (not part of the asset): claws of both hands and both hind feet hooked in the bark as supports, the hind feet beside the pelvis with the toes up, the body along the trunk, the tail hanging down.",
        "Kopf oben an einem senkrechten Stamm, dessen Rinde die Ebene x = 0,038 m ist (nicht Teil des Assets): Krallen beider Hände und beider Hinterfüße in der Rinde als Auflager, die Hinterfüße neben dem Becken mit den Zehen nach oben, der Körper am Stamm entlang, der Schwanz hängt herab.",
    ),
    "bound-support": t(
        "Key pose: the hands planted under the shoulders carry the body, the hind legs pass forward tightly flexed.",
        "Schlüsselpose: die unter den Schultern aufgesetzten Hände tragen den Körper, die Hinterbeine schwingen eng angezogen nach vorn.",
    ),
    "bound-swing": t(
        "Key pose at the lift-off of the hands, which push off behind the shoulders: the back arches and the hind legs swing forward, tucked under the belly.",
        "Schlüsselpose beim Abheben der Hände, die hinter den Schultern abdrücken: der Rücken wölbt sich, die Hinterbeine schwingen angezogen unter dem Bauch nach vorn.",
    ),
    "bound-reach": t(
        "Key pose in the air: the elbows fold, the forearms swing forward under the chest, the hind feet reach forward for the landing.",
        "Schlüsselpose in der Luft: die Ellbogen beugen sich, die Unterarme schwingen unter der Brust nach vorn, die Hinterfüße greifen zur Landung vor.",
    ),
}
CLIP_PATCH = {
    "bound": {
        "advanceM": 0.3591,
        "keys": [{"t": 0, "pose": "bound-extension", "ease": "linear"}, {"t": 0.045, "pose": "bound-support", "ease": "linear"}, {"t": 0.09, "pose": "bound-swing", "ease": "linear"}, {"t": 0.115, "pose": "bound-reach", "ease": "linear"}, {"t": 0.14, "pose": "bound-gather", "ease": "linear"}, {"t": 0.24, "pose": "bound-push", "ease": "linear"}, {"t": 0.3, "pose": "bound-flight", "ease": "linear"}],
        "contacts": [{"anchor": "ground-hand-n", "fromS": 0, "toS": 0.09}, {"anchor": "ground-hand-f", "fromS": 0, "toS": 0.09}, {"anchor": "ground-foot-n", "fromS": 0.14, "toS": 0.24}, {"anchor": "ground-foot-f", "fromS": 0.14, "toS": 0.24}],
        "description": t(
            "A bounding run of 0.36 s per cycle (about 2.8 bounds per second, 0.36 m per bound, 1.0 m/s): the hands land and carry the body over them, the hind legs swing through, the hind feet plant under the chest and push off; two airborne phases per cycle, the tail streams behind.",
            "Ein Sprunglauf mit 0,36 s je Zyklus (etwa 2,8 Sprünge pro Sekunde, 0,36 m je Sprung, 1,0 m/s): Die Hände landen und tragen den Körper über sich hinweg, die Hinterbeine schwingen durch, die Hinterfüße setzen unter der Brust auf und drücken ab; zwei Flugphasen je Zyklus, der Schwanz weht hinterher.",
        ),
    },
    "climb": {
        "description": t(
            "Two grip changes up a vertical trunk in 2.4 s: the claws of the hands and the hind feet are hooked in the bark as supports, never fewer than three grips at a time, the hind feet stay planted while the body rises 0.05 m and the hands reach 0.05 m higher.",
            "Zwei Griffwechsel an einem senkrechten Stamm in 2,4 s: Die Krallen der Hände und Hinterfüße haken sich als Auflager in die Rinde, nie weniger als drei Griffe zugleich, die Hinterfüße bleiben stehen, während der Körper um 0,05 m steigt und die Hände 0,05 m höher greifen.",
        ),
    },
}
PENS = {}
REFERENCE = {"id": "thorington-1997", "citation": "Thorington, R. W., Darrow, K., Betts, A. D. K. (1997). Comparative myology of the forelimb of squirrels (Sciuridae). Journal of Morphology 234, 155-182; with Thorington, R. W., Heaney, L. R. (1981). Body proportions and gliding adaptations of flying squirrels. Journal of Mammalogy 62, 101-114. Limb proportions of tree squirrels: humerus and radius of nearly equal length, tibia longer than femur, hind foot about a quarter of head and body length; the bone lengths here are authored estimates guided by these proportions and by the body lengths of gurnell-1987."}
BONE = {"id": "bone", "title": t("Bone", "Knochen"), "emissivity": 0.95, "densityKgM3": 1900, "elasticModulusPa": 15000000000, "strengthPa": 130000000, "source": "authored-estimate", "reference": "thorington-1997"}

REP_DECISIONS = {
    "50": {
        "silhouette": t(
            "Full articulation at 0.15 x 0.21 m = 3.0 x 4.2 mm: every drawn part is the outline of one mass of the 1:50 body in the frame of its bone and carries a solid hint, so side elevation, front elevation and plan are the three views of one animal in every pose. Parts that are deeper than wide (rump, chest, neck, upper arm, forearm, thigh, shank) are the prism of their side outline with the body of revolution of their lateral half width; flat parts (head, the two tail links, hind foot) add the prism of their plan outline; in each view the true outline of that view lies on top, and what is nearer to the viewer lies above.",
            "Volle Gliederung bei 0,15 x 0,21 m = 3,0 x 4,2 mm: Jedes gezeichnete Teil ist der Umriss einer Masse des Körpers 1:50 im Bezugssystem seines Knochens und trägt einen Körperhinweis, sodass Seitenansicht, Vorderansicht und Draufsicht in jeder Pose die drei Ansichten eines Tieres sind. Teile, die tiefer als breit sind (Becken, Brust, Hals, Oberarm, Unterarm, Ober- und Unterschenkel), sind das Prisma ihres Seitenumrisses mit dem Rotationskörper ihrer seitlichen Halbbreite; flache Teile (Kopf, die zwei Schwanzglieder, Hinterfuß) erhalten zusätzlich das Prisma ihres Grundrissumrisses; in jeder Ansicht liegt der dort wahre Umriss oben, und was dem Betrachter näher ist, liegt darüber.",
        ),
        "retained": t(
            "Rump, chest, neck and head, both ears, the eye, the nose pad, the brush in two links, the white underside from throat to groin in three pieces, both fore limbs in upper arm, forearm, paw and fingers, both hind limbs in thigh, shank, foot and bare sole.",
            "Becken, Brust, Hals und Kopf, beide Ohren, das Auge, der Nasenspiegel, der Schwanz in zwei Gliedern, die weiße Unterseite von der Kehle bis zur Leiste in drei Stücken, beide Vorderglieder in Oberarm, Unterarm, Pfote und Fingern, beide Hinterglieder in Ober-, Unterschenkel, Fuß und nackter Sohle.",
        ),
        "merged": t(
            "The four fingers of a hand are one tapering group, the toes are part of the foot, the lips are part of the head; whiskers, claws and single hairs are not drawn.",
            "Die vier Finger einer Hand sind eine sich verjüngende Gruppe, die Zehen gehören zum Fuß, die Lippen zum Kopf; Schnurrhaare, Krallen und einzelne Haare sind nicht gezeichnet.",
        ),
        "omitted": t(
            "The ear tuft hairs, whiskers, claws, single toes and the hair line of the tail (all under 0.1 mm on paper).",
            "Die Haare des Ohrpinsels, Schnurrhaare, Krallen, einzelne Zehen und die Haarlinie des Schwanzes (alle unter 0,1 mm auf dem Papier).",
        ),
        "thinElements": t(
            "Fingers 0.006 m, forearm 0.008 m at the wrist, hind foot 0.006 m and its sole 0.003 m thick are filled at true size (0.05-0.16 mm on paper); they read through their fill and merge with the 0.25 mm outline, nothing is thickened.",
            "Finger 0,006 m, Unterarm 0,008 m am Handgelenk, Hinterfuß 0,006 m und seine Sohle 0,003 m dick sind in wahrer Größe gefüllt (0,05-0,16 mm auf dem Papier); sie lesen sich über ihre Füllung und verschmelzen mit der 0,25-mm-Kontur, nichts ist verdickt.",
        ),
        "hatching": t(
            "None: the 0.4 mm minimum hatch spacing equals 0.02 m, a third of the body width.",
            "Keine: der Mindestabstand von 0,4 mm entspricht 0,02 m, einem Drittel der Körperbreite.",
        ),
        "styles": t(
            "0.25 mm silhouette, no detail pen; tones: tail darkest, back and head mid, far limbs lighter than near limbs, fingers and soles pale, the underside lightest, eye and nose pad near black.",
            "0,25-mm-Kontur, kein Detailstift; Töne: Schwanz am dunkelsten, Rücken und Kopf mittel, ferne Glieder heller als nahe, Finger und Sohlen blass, die Unterseite am hellsten, Auge und Nasenspiegel fast schwarz.",
        ),
        "overlays": t(
            "Every thermal region has its own shapes: dorsal fur on rump and chest, ventral fur on the three white pieces, head fur on neck, head and eye, the nose pad, the ears, the tail, arm fur on upper arm, forearm and paw, bare fingers, leg fur on thigh, shank and foot, bare soles.",
            "Jede thermische Region hat eigene Formen: Rückenfell an Becken und Brust, Bauchfell an den drei weißen Stücken, Kopffell an Hals, Kopf und Auge, der Nasenspiegel, die Ohren, der Schwanz, Armfell an Oberarm, Unterarm und Pfote, nackte Finger, Beinfell an Ober-, Unterschenkel und Fuß, nackte Sohlen.",
        ),
    },
    "100": {
        "silhouette": t(
            "Simplified articulation at 0.15 x 0.21 m = 1.5 x 2.1 mm: the outlines of the masses of the 1:100 body with half as many points as at 1:50, each with a solid hint, so the three views stay views of one animal in every pose. Trunk, neck, head, tail and the limb segments are drawn as at 1:50 (side prism, body of revolution, plan prism for the flat parts); eye, nose pad and fingers are gone.",
            "Vereinfachte Gliederung bei 0,15 x 0,21 m = 1,5 x 2,1 mm: die Umrisse der Massen des Körpers 1:100 mit halb so vielen Punkten wie bei 1:50, jeder mit Körperhinweis, sodass die drei Ansichten in jeder Pose Ansichten eines Tieres bleiben. Rumpf, Hals, Kopf, Schwanz und die Gliedabschnitte sind wie bei 1:50 gezeichnet (Seitenprisma, Rotationskörper, Grundrissprisma für die flachen Teile); Auge, Nasenspiegel und Finger entfallen.",
        ),
        "retained": t(
            "The bushy tail in two links, the ears, the rounded back, the white underside and the limbs that show whether the animal sits, bounds or climbs.",
            "Der buschige Schwanz in zwei Gliedern, die Ohren, der runde Rücken, die weiße Unterseite und die Glieder, die zeigen, ob das Tier sitzt, springt oder klettert.",
        ),
        "merged": t(
            "Paw and fingers are one bare hand (region hands-skin), the throat patch is part of the chest patch, the nose is part of the head shape (regions head-fur and nose).",
            "Pfote und Finger sind eine nackte Hand (Region hands-skin), der Kehlfleck gehört zum Brustfleck, die Nase zur Kopfform (Regionen head-fur und nose).",
        ),
        "omitted": t(
            "Eye, nose pad, single fingers and toes, whiskers, claws, ear tuft hairs.",
            "Auge, Nasenspiegel, einzelne Finger und Zehen, Schnurrhaare, Krallen, Haare des Ohrpinsels.",
        ),
        "thinElements": t(
            "Forearm, hand, shank end, foot and sole (0.003-0.011 m = 0.03-0.11 mm on paper) are filled at true size and merge with the 0.18 mm outline of their neighbours; they keep the pose readable in the model and in enlargements and are not thickened.",
            "Unterarm, Hand, Unterschenkelende, Fuß und Sohle (0,003-0,011 m = 0,03-0,11 mm auf dem Papier) sind in wahrer Größe gefüllt und verschmelzen mit der 0,18-mm-Kontur ihrer Nachbarn; sie halten die Pose im Modell und in Vergrößerungen lesbar und sind nicht verdickt.",
        ),
        "hatching": t("None.", "Keine."),
        "styles": t(
            "0.18 mm silhouette, no detail pen; tones: tail darkest, back mid, underside lightest, hands and soles pale.",
            "0,18-mm-Kontur, kein Detailstift; Töne: Schwanz am dunkelsten, Rücken mittel, Unterseite am hellsten, Hände und Sohlen blass.",
        ),
        "overlays": t(
            "Every thermal region is drawn by its own shapes except the nose, which is listed on the head shape.",
            "Jede thermische Region ist durch eigene Formen gezeichnet außer der Nase, die an der Kopfform geführt wird.",
        ),
    },
}

VOLUME_DECISIONS = {
    "50": [
        t(
            "Body at 1:50: 30 solids on the corrected skeleton (head and body 0.21 m, humerus and forearm 0.040 m, femur 0.050 m, tibia 0.058 m, hind foot 0.059 m with the heel). Rump, chest and neck are lofts with elliptic sections and domes about their joints, so every pose stays closed; the head is a blunt wedge with a round braincase, two leaf ears turned outwards, bulging eyes and the nose pad; the brush is two flattened links 0.054 m wide whose tip curls away; the white underside is three shells 2.2 mm proud of the red coat; each limb is upper arm, forearm, paw and finger group, thigh plate, shank with calf, long flat foot and bare sole.",
            "Körper bei 1:50: 30 Körper auf dem berichtigten Skelett (Kopf-Rumpf 0,21 m, Oberarm und Unterarm 0,040 m, Oberschenkel 0,050 m, Schienbein 0,058 m, Hinterfuß 0,059 m mit Ferse). Becken, Brust und Hals sind Lofts mit elliptischen Schnitten und Kuppeln um ihre Gelenke, sodass jede Pose geschlossen bleibt; der Kopf ist ein stumpfer Keil mit runder Hirnkapsel, zwei nach außen gedrehten Blattohren, vorstehenden Augen und dem Nasenspiegel; der Schwanz besteht aus zwei abgeflachten Gliedern von 0,054 m Breite, deren Spitze wegbiegt; die weiße Unterseite sind drei Schalen, 2,2 mm über dem roten Fell; jedes Glied besteht aus Oberarm, Unterarm, Pfote und Fingergruppe, Schenkelplatte, Unterschenkel mit Wade, langem flachem Fuß und nackter Sohle.",
        ),
        t(
            "Thermal at 1:50: every fur region is the body surface as the guard-hair layer and, inside it, the body offset inwards by the guard-hair thickness as the underfur shell (back 5.6 mm, belly 4.0 mm, head 2.8 mm, ears 1.6 mm, tail 10 mm, arms 2.0 mm, legs 2.4 mm); paws and feet are too thin for an inner shell and carry the guard layer only. Bare skin is the finger groups, the soles (their upper face lies against the foot and is no surface) and a cap over the muzzle tip for nose and lips. Breath leaves through the nose pad, the body plume through a low saddle patch on the loin.",
            "Thermik bei 1:50: Jede Fellregion ist die Körperoberfläche als Deckhaarschicht und darin der um die Deckhaardicke nach innen versetzte Körper als Unterfellschale (Rücken 5,6 mm, Bauch 4,0 mm, Kopf 2,8 mm, Ohren 1,6 mm, Schwanz 10 mm, Arme 2,0 mm, Beine 2,4 mm); Pfoten und Füße sind zu dünn für eine innere Schale und tragen nur die Deckschicht. Nackte Haut sind die Fingergruppen, die Sohlen (ihre Oberseite liegt am Fuß an und ist keine Oberfläche) und eine Kappe über der Schnauzenspitze für Nase und Lippen. Der Atem tritt am Nasenspiegel aus, die Körperfahne an einem flachen Sattelfleck über der Lende.",
        ),
        t(
            "Structure at 1:50: the skeleton, not the body. Lumbar and thoracic column run dorsal in the trunk, with six rib hoops and the sternum as slender bars, two innominate bones from the iliac crest through the hip joint to the ischium, two scapulae on the rib cage, the cervical column, the skull with its zygomatic width and the mandible, the caudal vertebrae tapering to 1.8 mm; humerus, forearm (radius and ulna as one bar with the olecranon), femur and lower leg have 3.2 to 4.4 mm shafts and joint ends of 7.6 to 9.2 mm, hand and foot are flat plates; joints are spheres of joint-end size at the rig joints, two masses mark chest and head.",
            "Tragwerk bei 1:50: das Skelett, nicht der Körper. Lenden- und Brustwirbelsäule verlaufen dorsal im Rumpf, mit sechs Rippenreifen und dem Brustbein als schlanken Stäben, zwei Hüftbeinen vom Darmbeinkamm über das Hüftgelenk zum Sitzbein, zwei Schulterblättern auf dem Brustkorb, der Halswirbelsäule, dem Schädel mit seiner Jochbogenbreite und dem Unterkiefer, den Schwanzwirbeln bis 1,8 mm; Oberarm, Unterarm (Speiche und Elle als ein Stab mit dem Ellbogenhöcker), Oberschenkel und Unterschenkel haben Schäfte von 3,2 bis 4,4 mm und Gelenkenden von 7,6 bis 9,2 mm, Hand und Fuß sind flache Platten; Gelenke sind Kugeln in der Größe der Gelenkenden an den Rig-Gelenken, zwei Massen markieren Brust und Kopf.",
        ),
    ],
    "100": [
        t(
            "Body at 1:100: 24 solids authored anew with three to five own sections each. Against 1:50 the fingers are merged into the paw, the throat patch into the chest patch, eye and nose pad are gone; rump, chest, neck, head wedge, leaf ears, the two tail links, two white shells and seven masses per side (upper arm, forearm, hand, thigh, shank, foot, sole) remain, so the animal still sits, bounds and climbs.",
            "Körper bei 1:100: 24 Körper, neu modelliert mit je drei bis fünf eigenen Schnitten. Gegenüber 1:50 sind die Finger in die Pfote und der Kehlfleck in den Brustfleck aufgenommen, Auge und Nasenspiegel entfallen; es bleiben Becken, Brust, Hals, Kopfkeil, Blattohren, die zwei Schwanzglieder, zwei weiße Schalen und sieben Massen je Seite (Oberarm, Unterarm, Hand, Ober- und Unterschenkel, Fuß, Sohle), sodass das Tier weiterhin sitzt, springt und klettert.",
        ),
        t(
            "Thermal at 1:100: the two fur layers of each region are merged into the body surface (one reference per mass); the whole hand is the bare-skin region of the hands, the soles that of the feet, a cap over the muzzle tip that of the nose; the breath outlet is a small pad at the muzzle tip, the plume outlet a saddle patch of four sections.",
            "Thermik bei 1:100: Die zwei Fellschichten jeder Region sind in der Körperoberfläche zusammengefasst (ein Verweis je Masse); die ganze Hand ist die Nackthautregion der Hände, die Sohlen die der Füße, eine Kappe über der Schnauzenspitze die der Nase; der Atemauslass ist ein kleines Polster an der Schnauzenspitze, der Fahnenauslass ein Sattelfleck aus vier Schnitten.",
        ),
        t(
            "Structure at 1:100: the skeleton reduced to what a 15 mm feature can show. Skull, rib cage and pelvis are three masses of their real size; the column (lumbar, thoracic, cervical, two caudal links) and every long bone, hand and foot are slender bars at about their shaft thickness (1.3 to 4.4 mm). No joint solids: a sphere of 15 mm would be larger than the limbs, the joints are where the bars meet. Two masses mark chest and head.",
            "Tragwerk bei 1:100: das Skelett, reduziert auf das, was ein Merkmal von 15 mm zeigen kann. Schädel, Brustkorb und Becken sind drei Massen in wahrer Größe; die Wirbelsäule (Lende, Brust, Hals, zwei Schwanzglieder) und jeder Röhrenknochen, Hand und Fuß sind schlanke Stäbe in etwa ihrer Schaftdicke (1,3 bis 4,4 mm). Keine Gelenkkörper: Eine Kugel von 15 mm wäre größer als die Glieder, die Gelenke liegen dort, wo die Stäbe sich treffen. Zwei Massen markieren Brust und Kopf.",
        ),
    ],
}

UNAVAILABLE_PATCH = [
    ("0.21 m x 0.275 m = 1.05 x 1.4 mm", "0.15 m x 0.21 m = 0.75 x 1.05 mm"), ("0,21 m x 0,275 m = 1,05 x 1,4 mm", "0,15 m x 0,21 m = 0,75 x 1,05 mm"),
    ("0.016-0.024 m = 0.08-0.12 mm", "0.008-0.023 m = 0.04-0.12 mm"), ("0,016-0,024 m = 0,08-0,12 mm", "0,008-0,023 m = 0,04-0,12 mm"),
    ("0.21 m x 0.275 m = 0.42 x 0.55 mm", "0.15 m x 0.21 m = 0.30 x 0.42 mm"), ("0,21 m x 0,275 m = 0,42 x 0,55 mm", "0,15 m x 0,21 m = 0,30 x 0,42 mm"),
    ("about a quarter of a square millimetre", "about a tenth of a square millimetre"), ("etwa einen Viertel Quadratmillimeter", "etwa einen Zehntel Quadratmillimeter"),
    ("The body depth (0.09 m = 0.18 mm) is above the 0.12 mm feature width of the plot profile but below the 0.2 mm of the office laser", "The body depth (0.06 m = 0.12 mm) is at the 0.12 mm feature width of the plot profile and below the 0.2 mm of the office laser"),
    ("Die Körpertiefe (0,09 m = 0,18 mm) liegt über der Merkmalsbreite von 0,12 mm des Plot-Profils, aber unter den 0,2 mm des Büro-Lasers", "Die Körpertiefe (0,06 m = 0,12 mm) liegt auf der Merkmalsbreite von 0,12 mm des Plot-Profils und unter den 0,2 mm des Büro-Lasers"),
]
