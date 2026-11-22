"""📝️ The bilingual texts of `teenager-with-headphones`."""
from lib import T

TITLE = T("Teenager with Headphones", "Jugendliche Person mit Kopfhörern")
SUMMARY = T(
    "A fifteen-year-old of 1.66 m in an oversized hoodie, jeans and sneakers, standing slouched with the shoulders rounded and the head bowed over a smartphone, wireless over-ear headphones on the head. Poses for texting, glancing up and listening with the phone lowered; clips for nodding to the beat, idle scrolling and a glance up from the screen. Total power is the body's metabolic heat plus the nameplates of phone and headphones.",
    "Eine fünfzehnjährige Person von 1,66 m in weitem Kapuzenpullover, Jeans und Sneakern, mit runden Schultern und über das Smartphone gesenktem Kopf gebeugt stehend, kabellose Over-Ear-Kopfhörer auf dem Kopf. Posen für Tippen, Aufblicken und Musikhören mit gesenktem Telefon; Clips für Nicken im Takt, beiläufiges Scrollen und einen Blick vom Bildschirm auf. Die Gesamtleistung ist die Stoffwechselwärme des Körpers plus die Nennleistungen von Telefon und Kopfhörern.")
TAGS = ["human", "teenager", "youth", "standing", "smartphone", "headphones", "music", "slouch", "street-life"]
NOTES = T(
    "Derived by hand from the archive's standing-phone-user (an adult of 1.69 m): trunk, girdles and pelvis scaled by 0.96, neck by 0.95 and head by 0.98 while legs and arms keep their length, the long-limbed proportion of the adolescent growth spurt, to a stature of 1.66 m (WHO 2007 growth reference at 15 years, median of both sexes) and 54 kg; drawings, the three solid lists and anchors of every bone scaled with it. Ponytail and the headphones around the neck were removed; the over-ear headphones are newly drawn on the head bone for every scale from one parametric design (cups 94 x 80 mm, 38 to 40 mm deep, over the ears; a padded band 32 mm wide arching over the hair), at 1:200 only the cups and at 1:500 merged into the head silhouette, so they move with the bowed head in every pose. Poses slouch the trunk 4 degrees forward with shoulders drawn forward and bend neck and head 45 degrees over the phone (forward head posture of smartphone users, Lee et al. 2015); segment masses follow the scaling of each segment normalised to 54 kg plus 0.25 kg of headphones.",
    "Von Hand aus dem Archivobjekt standing-phone-user (eine erwachsene Person von 1,69 m) abgeleitet: Rumpf, Gürtel und Becken mit 0,96, Hals mit 0,95 und Kopf mit 0,98 skaliert, während Beine und Arme ihre Länge behalten, die langgliedrige Proportion des Wachstumsschubs, auf eine Körperhöhe von 1,66 m (WHO-Wachstumsreferenz 2007 mit 15 Jahren, Median beider Geschlechter) und 54 kg; Zeichnungen, die drei Volumenlisten und die Anker jedes Knochens wurden mit ihm skaliert. Pferdeschwanz und die Kopfhörer um den Hals wurden entfernt; die Over-Ear-Kopfhörer sind für jeden Massstab aus einem parametrischen Entwurf neu am Kopfknochen gezeichnet (Muscheln 94 x 80 mm, 38 bis 40 mm tief, über den Ohren; ein gepolsterter Bügel von 32 mm Breite über dem Haar), in 1:200 nur die Muscheln und in 1:500 in den Kopfumriss eingeschlossen, sodass sie in jeder Pose mit dem gesenkten Kopf mitgehen. Die Posen neigen den Rumpf mit vorgezogenen Schultern 4 Grad vor und beugen Hals und Kopf 45 Grad über das Telefon (Kopfvorhaltung beim Smartphone-Gebrauch, Lee et al. 2015); die Segmentmassen folgen der Skalierung jedes Segments, auf 54 kg normiert, plus 0,25 kg Kopfhörer.")
NEW_REFERENCES = [
    {"id": "who-2007", "citation": "de Onis, M., Onyango, A. W., Borghi, E., Siyam, A., Nishida, C. and Siekmann, J. (2007). Development of a WHO growth reference for school-aged children and adolescents. Bulletin of the World Health Organization 85(9), 660-667; stature and body mass by age."},
    {"id": "schofield-1985", "citation": "Schofield, W. N. (1985). Predicting basal metabolic rate, new standards and review of previous work. Human Nutrition: Clinical Nutrition 39 (Suppl 1), 5-41."},
    {"id": "iec-62368-1-2018", "citation": "IEC 62368-1:2018 Audio/video, information and communication technology equipment, Part 1: Safety requirements; rated input marking of the equipment. The 2.5 W of the headphones is the catalogue-typical USB charging input (5 V, 0.5 A), an authored estimate."},
]


def thermal_assumptions(area, body, stature, total):
    return [
        T(f"Skin area {area:.2f} m2 after DuBois and DuBois (1916) for a fifteen-year-old of {body} kg and {stature} m; the {total} kg of the asset add clothes, shoes, the 0.19 kg phone and the 0.25 kg headphones. Region areas are those of the adult figure scaled to this skin area.",
          f"Hautfläche {area:.2f} m2 nach DuBois und DuBois (1916) für eine fünfzehnjährige Person mit {body} kg und {stature} m; die {total} kg des Objekts enthalten Kleidung, Schuhe, das 0,19 kg schwere Telefon und die 0,25 kg schweren Kopfhörer. Die Regionsflächen sind die der erwachsenen Figur, auf diese Hautfläche skaliert."),
        T("Clothing of about 0.75 clo: T-shirt, oversized fleece hoodie, jeans, socks and sneakers after the ISO 9920 tables (1 clo = 0.155 m2K/W); the headphones cover the ears and part of the hair and are a passive surface warmed only by the head. The reference case is a mild grey afternoon at a bus stop, 15 degrees C.",
          "Kleidung von etwa 0,75 clo: T-Shirt, weiter Fleece-Kapuzenpullover, Jeans, Socken und Sneaker nach den Tabellen von ISO 9920 (1 clo = 0,155 m2K/W); die Kopfhörer bedecken die Ohren und einen Teil des Haars und sind eine passive Oberfläche, die nur der Kopf erwärmt. Der Referenzfall ist ein milder grauer Nachmittag an einer Bushaltestelle, 15 Grad C."),
        T("Activity: standing and texting 1.2 met, nodding to music 1.3 met (ISO 7730 Annex A for standing relaxed, the nodding authored); the basal rate 73 W is the mean of the Schofield (1985) equations for girls and boys of 10 to 18 years. Power: the phone states 5 W and the headphones 2.5 W of electrical input (catalogue-typical charging ratings, authored estimates).",
          "Aktivität: Stehen und Tippen 1,2 met, Nicken zur Musik 1,3 met (ISO 7730 Anhang A für entspanntes Stehen, das Nicken geschätzt); der Grundumsatz von 73 W ist das Mittel der Schofield-Gleichungen (1985) für Mädchen und Jungen von 10 bis 18 Jahren. Leistung: Das Telefon gibt 5 W und die Kopfhörer 2,5 W elektrische Aufnahme an (katalogtypische Ladeleistungen, Schätzwerte)."),
        T("Breathing patterns are authored and only guided by quiet adult values (ICRP 66): quiet standing, slightly shallow concentrated texting, and a calm listening pattern.",
          "Die Atemmuster sind geschätzt und nur an ruhigen Erwachsenenwerten (ICRP 66) orientiert: ruhiges Stehen, leicht flaches konzentriertes Tippen und ein ruhiges Muster beim Musikhören."),
    ]


POSE_TEXT = {
    "slouched-texting": {"title": T("Slouched, texting", "Gebeugt, tippend"), "description": T("Feet at hip width with soft knees, the trunk slouched 4 degrees forward with rounded shoulders, neck and head bowed 45 degrees over the phone held in both hands in front of the belly, thumbs on the screen; the headphones sit over the ears.", "Füße hüftbreit mit weichen Knien, der Rumpf 4 Grad vorgebeugt mit runden Schultern, Hals und Kopf 45 Grad über das Telefon gesenkt, das beide Hände vor dem Bauch halten, die Daumen auf dem Bildschirm; die Kopfhörer sitzen über den Ohren.")},
    "glancing-up": {"title": T("Glancing up from the phone", "Vom Telefon aufblickend"), "description": T("The same slouched stance with the phone still held in both hands, the head raised to look ahead for a moment, the shoulders still rounded.", "Dieselbe gebeugte Haltung, das Telefon noch in beiden Händen, der Kopf kurz gehoben, um nach vorn zu schauen, die Schultern noch rund.")},
    "listening-phone-down": {"title": T("Listening, phone lowered", "Musik hörend, Telefon gesenkt"), "description": T("Both arms hang loosely, the phone in the right hand by the thigh, head half bowed and the knees soft: just listening to the music in the headphones.", "Beide Arme hängen locker, das Telefon in der rechten Hand am Oberschenkel, der Kopf halb gesenkt und die Knie weich: nur der Musik in den Kopfhörern zuhörend.")},
}

CLIP_TEXT = {
    "nod-to-the-beat": {"title": T("Nodding to the beat", "Im Takt nicken"), "description": T("Listening with the phone lowered and nodding to music at 120 beats per minute: the head drops about 9 degrees on every beat and the knees give a little with it, one bar of four beats per loop.", "Musik hören mit gesenktem Telefon und im Takt von 120 Schlägen je Minute nicken: Der Kopf sinkt bei jedem Schlag um etwa 9 Grad und die Knie geben ein wenig mit, ein Takt mit vier Schlägen je Schleife.")},
    "idle-scrolling": {"title": T("Scrolling and swaying", "Scrollen und pendeln"), "description": T("Slouched over the phone, scrolling: the head bobs a few degrees and the trunk sways slowly while the screen stays lit.", "Über das Telefon gebeugt beim Scrollen: Der Kopf wippt einige Grad und der Rumpf pendelt langsam, während der Bildschirm leuchtet.")},
    "glance-up": {"title": T("Glance up and back", "Aufblicken und zurück"), "description": T("From texting the teenager raises the head to look ahead, holds for a second and sinks back to the screen.", "Aus dem Tippen hebt die jugendliche Person den Kopf, um nach vorn zu schauen, verharrt eine Sekunde und sinkt zurück zum Bildschirm.")},
}

DECISIONS = {
    "500": {
        "silhouette": T("The views of the 1:500 model, 3.3 mm tall: one silhouette per body part with the bowed head, the hoodie mass and the arms bent to the phone.", "Die Ansichten des 1:500-Modells, 3,3 mm hoch: ein Umriss je Körperteil mit gesenktem Kopf, der Masse des Kapuzenpullovers und den zum Telefon gebeugten Armen."),
        "retained": T("The slouch and the bowed head, which make the figure read as a person absorbed in a phone.", "Die gebeugte Haltung und der gesenkte Kopf, durch die die Figur als in das Telefon vertiefte Person lesbar wird."),
        "merged": T("Hair, face and headphones into one head silhouette; hood into the trunk; hands with the phone.", "Haar, Gesicht und Kopfhörer in einen Kopfumriss; Kapuze in den Rumpf; Hände mit dem Telefon."),
        "omitted": T("The headphone band and cups as separate parts (40 mm, below the 75 mm feature), laces, seams, face.", "Bügel und Muscheln der Kopfhörer als eigene Teile (40 mm, unter dem Merkmal von 75 mm), Schnürsenkel, Nähte, Gesicht."),
        "thinElements": T("Phone and fingers lie below the feature and stay merged into the hands.", "Telefon und Finger liegen unter dem Merkmal und bleiben in den Händen."),
        "hatching": T("None.", "Keine."),
        "styles": T("Body pen 0.13 mm; no detail pen.", "Körperstift 0,13 mm; kein Detailstift."),
        "overlays": T("The head silhouette lists face, hair and headphones as one merged form; the thermal model keeps the headphones as two cups at the ears.", "Der Kopfumriss führt Gesicht, Haar und Kopfhörer als eine zusammengefasste Form; das Thermikmodell behält die Kopfhörer als zwei Muscheln an den Ohren."),
    },
    "200": {
        "silhouette": T("The views of the 1:200 model, 8.3 mm tall: head with short hair and the two headphone cups, hoodie, sleeves, jeans legs, sneakers, hands with the phone.", "Die Ansichten des 1:200-Modells, 8,3 mm hoch: Kopf mit kurzem Haar und den beiden Kopfhörermuscheln, Kapuzenpullover, Ärmel, Jeansbeine, Sneaker, Hände mit dem Telefon."),
        "retained": T("The dark cups over the ears, which read as headphones even without the band, and the forward head.", "Die dunklen Muscheln über den Ohren, die auch ohne Bügel als Kopfhörer lesbar sind, und der vorgeschobene Kopf."),
        "merged": T("Cushion into the cup, hood into the trunk, fingers into the hands.", "Polster in die Muschel, Kapuze in den Rumpf, Finger in die Hände."),
        "omitted": T("The headband (16 mm, below the 30 mm feature), drawstrings, face.", "Der Kopfbügel (16 mm, unter dem Merkmal von 30 mm), Kordeln, Gesicht."),
        "thinElements": T("None: every part is a filled shape at its true size.", "Keine: Jeder Teil ist eine gefüllte Form in wahrer Grösse."),
        "hatching": T("None.", "Keine."),
        "styles": T("Body pen 0.18 mm, detail 0.1 mm; dark headphones and hair, mid hoodie, dark jeans, light sneakers.", "Körperstift 0,18 mm, Detail 0,1 mm; dunkle Kopfhörer und Haare, mittlerer Kapuzenpullover, dunkle Jeans, helle Sneaker."),
        "overlays": T("One region per shape: face skin, hair, hands, hoodie, jeans, sneakers, headphones, phone.", "Eine Region je Form: Gesichtshaut, Haar, Hände, Kapuzenpullover, Jeans, Sneaker, Kopfhörer, Telefon."),
    },
    "100": {
        "silhouette": T("The views of the 1:100 model, 16.6 mm tall: head with short hair under the headband, the cups over the ears, hood and hoodie, sleeves with cuffs, jeans, sneakers, hands holding the phone.", "Die Ansichten des 1:100-Modells, 16,6 mm hoch: Kopf mit kurzem Haar unter dem Kopfbügel, die Muscheln über den Ohren, Kapuze und Kapuzenpullover, Ärmel mit Bündchen, Jeans, Sneaker, Hände mit dem Telefon."),
        "retained": T("The headband arching over the hair, the cups, the slouch and the hood lying on the shoulders.", "Der über das Haar gespannte Kopfbügel, die Muscheln, die gebeugte Haltung und die auf den Schultern liegende Kapuze."),
        "merged": T("Cushion into the cup, fingers into the hands, laces into the shoe.", "Polster in die Muschel, Finger in die Hände, Schnürsenkel in den Schuh."),
        "omitted": T("Face lines, hair strands, seams.", "Gesichtslinien, Haarsträhnen, Nähte."),
        "thinElements": T("The 16 mm band is drawn at its true width as a filled arch; drawstrings and hems are detail strokes.", "Der 16 mm breite Bügel ist in wahrer Breite als gefüllter Bogen gezeichnet; Kordeln und Säume sind Detailstriche."),
        "hatching": T("None.", "Keine."),
        "styles": T("Body pen 0.2 mm, detail 0.1 mm; tones as at 1:50.", "Körperstift 0,2 mm, Detail 0,1 mm; Tonwerte wie in 1:50."),
        "overlays": T("One region per shape, as at 1:50.", "Eine Region je Form, wie in 1:50."),
    },
    "50": {
        "silhouette": T("The slouched teenager as the true orthographic views of its 1:50 model, 33 mm tall: short hair under the padded headband, cups over the ears, face in profile, hood, oversized hoodie with pocket and drawstrings, sleeves with cuffs, jeans with seams, sneakers with laces, both hands with fingers and thumbs on the phone.", "Die gebeugte jugendliche Person als wahre orthogonale Ansichten ihres 1:50-Modells, 33 mm hoch: kurzes Haar unter dem gepolsterten Kopfbügel, Muscheln über den Ohren, Gesicht im Profil, Kapuze, weiter Kapuzenpullover mit Tasche und Kordeln, Ärmel mit Bündchen, Jeans mit Nähten, Sneaker mit Schnürsenkeln, beide Hände mit Fingern und Daumen am Telefon."),
        "retained": T("One shape per part with the same id, tone, region and material in every view; the headphones are one cup per side and one band, each an extruded solid placed with the head, so they stay right when the head bows.", "Eine Form je Teil mit derselben Kennung, demselben Tonwert, derselben Region und demselben Material in jeder Ansicht; die Kopfhörer sind je Seite eine Muschel und ein Bügel, jeweils ein extrudierter Körper am Kopf, sodass sie beim Senken des Kopfes richtig bleiben."),
        "merged": T("Ear cushion into the cup; the far cup lies behind the near one in the side elevation.", "Ohrpolster in die Muschel; die ferne Muschel liegt in der Seitenansicht hinter der nahen."),
        "omitted": T("The slider of the band and the buttons of the cups, single hairs, fabric texture.", "Der Schieber des Bügels und die Tasten der Muscheln, einzelne Haare, Gewebestruktur."),
        "thinElements": T("Drawstrings, seams and laces are detail and texture strokes; the 14 mm band is a filled arch at its true width.", "Kordeln, Nähte und Schnürsenkel sind Detail- und Texturstriche; der 14 mm breite Bügel ist ein gefüllter Bogen in wahrer Breite."),
        "hatching": T("None.", "Keine."),
        "styles": T("Body 0.25 mm, detail 0.13 mm, texture 0.10 mm; headphones nearly black, hair dark, hoodie mid grey, jeans dark, sneakers light.", "Körper 0,25 mm, Detail 0,13 mm, Textur 0,10 mm; Kopfhörer fast schwarz, Haar dunkel, Kapuzenpullover mittelgrau, Jeans dunkel, Sneaker hell."),
        "overlays": T("One region per shape: face skin, hair, hands, hoodie, jeans, sneakers, headphones, phone.", "Eine Region je Form: Gesichtshaut, Haar, Hände, Kapuzenpullover, Jeans, Sneaker, Kopfhörer, Telefon."),
    },
}

VOLUME_DECISIONS = {
    "50": [T("Body 1:50: the slim adolescent body of the phone-user model with a 4 % shorter trunk, short hair, the hoodie, jeans, sneakers and both hands with fingers on the phone; headphones as two elliptic cups 94 x 80 x 38 mm over the ears and a band 32 mm wide and 14 mm thick extruded along the head's normal in the shape of an arch over the hair.", "Körper 1:50: der schlanke jugendliche Körper des Telefonmodells mit 4 % kürzerem Rumpf, kurzem Haar, Kapuzenpullover, Jeans, Sneakern und beiden Händen mit Fingern am Telefon; Kopfhörer als zwei elliptische Muscheln von 94 x 80 x 38 mm über den Ohren und ein Bügel von 32 mm Breite und 14 mm Dicke, als Bogen über dem Haar entlang der Kopfnormalen extrudiert."),
           T("Thermal 1:50: the regions of the body on their solids; the headphones are one passive surface over cups and band, warmed by the head they cover.", "Thermik 1:50: die Regionen des Körpers auf ihren Volumen; die Kopfhörer sind eine passive Oberfläche über Muscheln und Bügel, erwärmt vom Kopf, den sie bedecken."),
           T("Structure 1:50: the skeleton of the phone-user model at the scaled trunk; the headphones are a mass on the head bone, not a member.", "Tragwerk 1:50: das Skelett des Telefonmodells mit skaliertem Rumpf; die Kopfhörer sind eine Masse am Kopfknochen, kein Glied.")],
    "100": [T("Body 1:100: the same figure for a 15 mm feature; cups 40 mm deep and the band 16 mm thick.", "Körper 1:100: dieselbe Figur für ein Merkmal von 15 mm; Muscheln 40 mm tief und Bügel 16 mm dick."),
            T("Thermal 1:100: one surface per region; headphones on cups and band.", "Thermik 1:100: eine Oberfläche je Region; Kopfhörer auf Muscheln und Bügel."),
            T("Structure 1:100: the skeleton of 1:100 at the scaled trunk.", "Tragwerk 1:100: das Skelett von 1:100 mit skaliertem Rumpf.")],
    "200": [T("Body 1:200: masses of at least 30 mm; the headphones are the two 40 mm cups, the 16 mm band is left out.", "Körper 1:200: Massen von mindestens 30 mm; die Kopfhörer sind die beiden 40 mm tiefen Muscheln, der 16 mm dicke Bügel entfällt."),
            T("Thermal 1:200: one surface per region; headphones on the cups.", "Thermik 1:200: eine Oberfläche je Region; Kopfhörer auf den Muscheln."),
            T("Structure 1:200: the members of 30 mm of the scaled skeleton.", "Tragwerk 1:200: die Glieder von 30 mm des skalierten Skeletts.")],
    "500": [T("Body 1:500: the fewest masses of the figure; the headphones lie below the 75 mm feature and are part of the head mass.", "Körper 1:500: die wenigsten Massen der Figur; die Kopfhörer liegen unter dem Merkmal von 75 mm und gehören zur Kopfmasse."),
            T("Thermal 1:500: one surface per region; the headphones keep two 75 mm cups at the ears in the thermal model only.", "Thermik 1:500: eine Oberfläche je Region; die Kopfhörer behalten nur im Thermikmodell zwei Muscheln von 75 mm an den Ohren."),
            T("Structure 1:500: the members of 1:200 at the printable thickness of this scale.", "Tragwerk 1:500: die Glieder von 1:200 in der druckbaren Stärke dieses Massstabs.")],
}

STRUCTURE_ASSUMPTIONS = [
    T("Segments are point masses at the scaled de Leva centres; the 190 g phone is a load at its own centre and the 250 g headphones are part of the head mass, which together with the 45 degree bow raises the moment at the neck.", "Segmente sind Punktmassen in den skalierten Schwerpunkten nach de Leva; das 190 g schwere Telefon ist eine Last in seinem eigenen Schwerpunkt und die 250 g schweren Kopfhörer gehören zur Kopfmasse, was zusammen mit der Beugung um 45 Grad das Moment am Hals erhöht."),
    T("Contacts are heel and ball of each foot; both feet stand flat at hip-joint width in every named pose.", "Kontakte sind Ferse und Ballen jedes Fußes; beide Füße stehen in jeder benannten Pose flach in Hüftgelenkbreite."),
]
