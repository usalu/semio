"""📝️ Decision texts shared by the three HW-04 construction people: the body drawing and volume of each scale (from the base figure, without the carton) joined with what the person's own equipment adds."""
from lib import tx

BODY = {
    "50": {
        "silhouette": tx("Every view is the orthographic view of the 1:50 body: side elevation with profile face, hard hat with peak and crown ridge, shirt collar, vest with hem, reflective bands and brace, boots with sole and toe sole, both arms with gloves. Front view (the figure's left is on the right of the sheet): face with eyes, brows, nose and mouth, ears, vest with zip, bands and braces, sleeves, glove cuffs, trousers, boots. Plan: hard hat, shoulders in the vest, sleeves, forearms and gloves, boot toes. Sleeves, forearms and cuffs are revolved and gloves extruded profiles of their bones, so turned arms stay right in every pose.", "Jede Ansicht ist die orthografische Ansicht des 1:50-Körpers: Seitenansicht mit Profilgesicht, Schutzhelm mit Schirm und Scheitelgrat, Hemdkragen, Weste mit Saum, Reflexstreifen und Träger, Stiefel mit Sohle und Zehensohle, beide Arme mit Handschuhen. Vorderansicht (die linke Körperseite liegt rechts im Blatt): Gesicht mit Augen, Brauen, Nase und Mund, Ohren, Weste mit Reißverschluss, Streifen und Trägern, Ärmel, Handschuhstulpen, Hose, Stiefel. Grundriss: Schutzhelm, Schultern in der Weste, Ärmel, Unterarme und Handschuhe, Stiefelspitzen. Ärmel, Unterarme und Stulpen sind gedrehte, Handschuhe extrudierte Profile ihrer Knochen, sodass gedrehte Arme in jeder Pose stimmen."),
        "retained": tx("Face, ears, both arms and legs with gloves and boots", "Gesicht, Ohren, beide Arme und Beine mit Handschuhen und Stiefeln"),
        "merged": tx("Fingers and thumb into the glove outline (they wrap the grip in every view)", "Finger und Daumen in den Handschuhumriss (sie umfassen in jeder Ansicht den Griff)"),
        "omitted": tx("Laces and helmet headband (inside the outline), cloth folds", "Schnürung und Helmkopfband (innerhalb des Umrisses), Stofffalten"),
        "thinElements": tx("Mouth, jaw, zip and collar lines 0.13 to 0.25 mm; reflective bands drawn at their 50 mm width", "Mund-, Kiefer-, Reißverschluss- und Kragenlinien 0,13 bis 0,25 mm; Reflexstreifen in ihrer Breite von 50 mm gezeichnet"),
        "hatching": tx("None on the body", "Keine am Körper"),
        "styles": tx("Body 0.35, detail 0.25, texture 0.13 mm", "Körper 0,35, Detail 0,25, Textur 0,13 mm"),
        "overlays": tx("One shape per region and bone. Front and plan: every filled shape carries the region of the garment, body part or equipment it depicts, so thermal, structural and transport overlays read in all three views", "Eine Form je Region und Knochen. Vorderansicht und Grundriss: Jede gefüllte Form trägt die Region des Kleidungsstücks, Körperteils oder Geräts, das sie zeigt, sodass thermische, statische und Transport-Überlagerungen in allen drei Ansichten lesbar sind"),
    },
    "100": {
        "silhouette": tx("Every view is the orthographic view of the 1:100 body: hard hat with peak and ridge, face with nose and ear, shirt yoke, vest with hem, bands and brace, trousers, boots with sole, revolved sleeves and forearms with glove cuffs, mitten gloves. Front view (the figure's left is on the right of the sheet) and plan show the same parts from ahead and from above.", "Jede Ansicht ist die orthografische Ansicht des 1:100-Körpers: Schutzhelm mit Schirm und Grat, Gesicht mit Nase und Ohr, Hemdpasse, Weste mit Saum, Streifen und Träger, Hose, Stiefel mit Sohle, gedrehte Ärmel und Unterarme mit Handschuhstulpen, Fäustlingshandschuhe. Vorderansicht (die linke Körperseite liegt rechts im Blatt) und Grundriss zeigen dieselben Teile von vorn und von oben."),
        "retained": tx("Both arms (the far arm hatched), both legs, nose, ears, mouth and jaw lines", "Beide Arme (der ferne schraffiert), beide Beine, Nase, Ohren, Mund- und Kieferlinie"),
        "merged": tx("Fingers and thumb into one mitten outline; knee into the shank", "Finger und Daumen in einen Fäustlingsumriss; Knie in den Unterschenkel"),
        "omitted": tx("Eyes, brows, laces, zip", "Augen, Brauen, Schnürung, Reißverschluss"),
        "thinElements": tx("Reflective bands as 0.5 mm wide outlined strips; brace 0.25 mm; mouth and jaw lines 0.15 mm", "Reflexstreifen als 0,5 mm breite umrandete Streifen; Träger 0,25 mm; Mund- und Kieferlinie 0,15 mm"),
        "hatching": tx("Far-side arm and leg: 0.6 mm lines at 60 degrees", "Abgewandter Arm und abgewandtes Bein: 0,6-mm-Linien unter 60 Grad"),
        "styles": tx("Body 0.25, detail 0.15, texture 0.1 mm; vest light, bands dark, boots black", "Körper 0,25, Detail 0,15, Textur 0,1 mm; Weste hell, Streifen dunkel, Stiefel schwarz"),
        "overlays": tx("Every region is drawn as its own shapes; details inherit the region of the garment they sit on. Front and plan: regions as in the side view", "Jede Region ist mit eigenen Formen gezeichnet; Details übernehmen die Region des Kleidungsstücks, auf dem sie sitzen. Vorderansicht und Grundriss: Regionen wie in der Seitenansicht"),
    },
    "200": {
        "silhouette": tx("Every view is the orthographic view of the 1:200 body: hard hat with peak, head, shirt collar, vest with hem and two bands, trousers, boots, revolved sleeves and forearms with cuffs, mittens. Front view (the figure's left is on the right of the sheet) and plan from the same masses.", "Jede Ansicht ist die orthografische Ansicht des 1:200-Körpers: Schutzhelm mit Schirm, Kopf, Hemdkragen, Weste mit Saum und zwei Streifen, Hose, Stiefel, gedrehte Ärmel und Unterarme mit Stulpen, Fäustlinge. Vorderansicht (die linke Körperseite liegt rechts im Blatt) und Grundriss aus denselben Massen."),
        "retained": tx("Both arms and legs, helmet peak, vest bands", "Beide Arme und Beine, Helmschirm, Westenstreifen"),
        "merged": tx("Face, nose and ears into the head; hair into the head outline; shirt into the collar above the vest", "Gesicht, Nase und Ohren in den Kopf; Haar in den Kopfumriss; Hemd in den Kragen über der Weste"),
        "omitted": tx("Ridge, braces, face lines", "Grat, Träger, Gesichtslinien"),
        "thinElements": tx("Reflective bands as 0.25 mm wide outlined strips (50 mm real)", "Reflexstreifen als 0,25 mm breite umrandete Streifen (50 mm real)"),
        "hatching": tx("None", "Keine"),
        "styles": tx("Outlines 0.18 mm, bands 0.13 mm; vest light tone so it separates from the shirt", "Umrisse 0,18 mm, Streifen 0,13 mm; Weste in hellem Ton, damit sie sich vom Hemd abhebt"),
        "overlays": tx("Every region except the merged hair has its own piece (hair shares the head piece). Front and plan: regions as in the side view", "Jede Region außer dem verschmolzenen Haar hat ein eigenes Teil (das Haar teilt sich das Kopfteil). Vorderansicht und Grundriss: Regionen wie in der Seitenansicht"),
    },
    "500": {
        "silhouette": tx("Every view is the orthographic view of the 1:500 body: hard hat dome, head, shirt collar, vest trunk, seat, two-part legs with boots, revolved two-part arms with fists. Front view: the figure's left is on the right of the sheet.", "Jede Ansicht ist die orthografische Ansicht des 1:500-Körpers: Helmkuppel, Kopf, Hemdkragen, Westenrumpf, Gesäß, zweiteilige Beine mit Stiefeln, gedrehte zweiteilige Arme mit Fäusten. Vorderansicht: Die linke Körperseite liegt rechts im Blatt."),
        "retained": tx("Hard hat, collar, both arms and legs, boots", "Schutzhelm, Kragen, beide Arme und Beine, Stiefel"),
        "merged": tx("Helmet brim into the dome; neck into the head; vest and bands into one trunk; gloves into fists", "Helmrand in die Kuppel; Hals in den Kopf; Weste und Streifen in einen Rumpf; Handschuhe in Fäuste"),
        "omitted": tx("Face, ears, peak, bands, cuffs", "Gesicht, Ohren, Schirm, Streifen, Stulpen"),
        "thinElements": tx("No body part is drawn thinner than 75 mm (0.15 mm on paper)", "Kein Körperteil ist dünner als 75 mm (0,15 mm auf dem Papier) gezeichnet"),
        "hatching": tx("None", "Keine"),
        "styles": tx("One 0.13 mm outline; solid view: vest light, trousers and boots dark", "Eine Umrisslinie von 0,13 mm; Flächendarstellung: Weste hell, Hose und Stiefel dunkel"),
        "overlays": tx("Thermal colours attach to pieces; the torso piece merges shirt and vest, the head piece face, hair and helmet. Front and plan: merged shapes list all regions they show", "Thermische Farben hängen an den Teilen; das Rumpfteil verschmilzt Hemd und Weste, das Kopfteil Gesicht, Haar und Helm. Vorderansicht und Grundriss: Verschmolzene Formen nennen alle Regionen, die sie zeigen"),
    },
}

VOLUME = {
    "50": tx("Body 1:50 of the base figure for a 7.5 mm feature, scaled to this person: skull with jaw, chin, nose, lips, eyes, brows and ears, hair at the nape, neck; hard hat as dome with rim, peak, ridge and headband; work shirt with collar under a class-2 hi-vis vest with bands, braces and zip as texture; trouser seat with gluteal masses, thighs, knees and shanks over safety boots of shaft, vamp, cleated sole, toe cap and laces; sleeves with the deltoid, forearms, glove cuffs, gloves with palm, thenar, three-segment fingers and thumb. Thermal 1:50: every region with its layers as nested shells of real thickness; breath outlets at nostrils and lips, the plume patch on the helmet crown. Structure 1:50: the anatomical skeleton with 17 joints and segment mass markers.", "Körper 1:50 der Ausgangsfigur für 7,5 mm Mindestmaß, auf diese Person skaliert: Schädel mit Kiefer, Kinn, Nase, Lippen, Augen, Brauen und Ohren, Haar im Nacken, Hals; Schutzhelm als Kuppel mit Rand, Schirm, Grat und Kopfband; Arbeitshemd mit Kragen unter einer Warnweste Klasse 2 mit Streifen, Trägern und Reißverschluss als Textur; Hosengesäß mit Gesäßmassen, Oberschenkel, Knie und Unterschenkel über Sicherheitsstiefeln aus Schaft, Blatt, Profilsohle, Zehenkappe und Schnürung; Ärmel mit Deltamuskel, Unterarme, Handschuhstulpen, Handschuhe mit Handteller, Daumenballen, dreigliedrigen Fingern und Daumen. Thermik 1:50: jede Region mit ihren Schichten als verschachtelte Schalen in wahrer Dicke; Atemauslässe an Nase und Lippen, die Fahnenfläche auf dem Helmscheitel. Tragwerk 1:50: das anatomische Skelett mit 17 Gelenken und Segmentmassen."),
    "100": tx("Body 1:100 of the base figure for a 15 mm feature, scaled to this person: one skull loft with nose, chin and ears, neck, hair; helmet as dome, rim, peak and ridge; vest over shirt yoke and collar, bands and braces as texture; trouser seat, thigh, knee and shank; boots as shaft, vamp, sole and toe cap; sleeves, forearms, glove cuffs, mitten gloves with one finger block and a thumb. Thermal 1:100: one surface per region by reference to the body solids. Structure 1:100: skeleton with rib hoops, metatarsal fan and finger row.", "Körper 1:100 der Ausgangsfigur für 15 mm Mindestmaß, auf diese Person skaliert: ein Schädel-Loft mit Nase, Kinn und Ohren, Hals, Haar; Helm als Kuppel, Rand, Schirm und Grat; Weste über Hemdpasse und Kragen, Streifen und Träger als Textur; Hosengesäß, Oberschenkel, Knie und Unterschenkel; Stiefel aus Schaft, Blatt, Sohle und Zehenkappe; Ärmel, Unterarme, Stulpen, Fäustlinge mit einem Fingerblock und Daumen. Thermik 1:100: eine Fläche je Region als Verweis auf die Körpervolumen. Tragwerk 1:100: Skelett mit Rippenreifen, Mittelfußfächer und Fingerreihe."),
    "200": tx("Body 1:200 of the base figure for a 30 mm feature, scaled to this person: one mass per part, helmet with peak, collar above the vest, two-part limbs with mittens and boots. Thermal 1:200: one surface per region, breath and plume outlets of 30 mm. Structure 1:200: a skeleton of 30 mm bones with rib hoops, shoulder and pelvic girdles and ball joints.", "Körper 1:200 der Ausgangsfigur für 30 mm Mindestmaß, auf diese Person skaliert: eine Masse je Teil, Helm mit Schirm, Kragen über der Weste, zweiteilige Gliedmaßen mit Fäustlingen und Stiefeln. Thermik 1:200: eine Fläche je Region, Atem- und Fahnenauslässe von 30 mm. Tragwerk 1:200: ein Skelett aus 30-mm-Knochen mit Rippenreifen, Schulter- und Beckengürtel und Kugelgelenken."),
    "500": tx("Body 1:500 of the base figure for a 75 mm feature, scaled to this person: head with neck under the helmet dome, one trunk mass in the vest, collar ring, seat, two-part legs with boots, two-part arms with fists. Thermal 1:500: one surface per region and outlets of 75 mm. Structure 1:500: a stick skeleton of 75 mm bars.", "Körper 1:500 der Ausgangsfigur für 75 mm Mindestmaß, auf diese Person skaliert: Kopf mit Hals unter der Helmkuppel, eine Rumpfmasse in der Weste, Kragenring, Gesäß, zweiteilige Beine mit Stiefeln, zweiteilige Arme mit Fäusten. Thermik 1:500: eine Fläche je Region und Auslässe von 75 mm. Tragwerk 1:500: ein Stabskelett aus 75-mm-Stäben."),
}


def join(a, b, sep_en="; ", sep_de="; "):
    return tx(a["en"] + sep_en + b["en"], a["de"] + sep_de + b["de"])


def decisions(scale, add):
    """The decisions of one scale: the body text with the person's additions per field (`add[field]` = text appended)."""
    out = {}
    for k, v in BODY[scale].items():
        if k in add:
            out[k] = join(v, add[k], " " if k == "silhouette" else "; ", " " if k == "silhouette" else "; ")
        else:
            out[k] = v
        if not out[k]["en"].endswith("."):
            out[k] = tx(out[k]["en"] + ".", out[k]["de"] + ".")
    return out


def volume_decisions(base, scale, add):
    """The base figure's volume decisions without the carton, the shared body text first, then the person's equipment."""
    keep = [d for d in base if not any(w in d["en"].lower() for w in ("carton", "box"))]
    return [VOLUME[scale]] + keep + [add]
