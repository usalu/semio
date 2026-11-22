"""🗒️ Per-scale drawing and volume decisions of picnicker-sitting-on-ground."""
from hw10lib import *

D = {
    "50": dict(
        silhouette=text("Each view is the orthographic view of the 1:50 body sitting cross-legged on the blanket: seen from the right the folded legs with the near knee raised, the upright trunk and the apple at the mouth; seen from ahead the knees fanned out to both sides and the crossed shanks; from above the diamond of the crossed legs, the arms and the head, all over the gingham square of the blanket.", "Jede Ansicht ist die orthogonale Ansicht des 1:50-Körpers im Schneidersitz auf der Decke: von rechts die gefalteten Beine mit dem nahen Knie oben, der aufrechte Rumpf und der Apfel am Mund; von vorn die zu beiden Seiten ausgestellten Knie und die gekreuzten Unterschenkel; von oben die Raute der gekreuzten Beine, Arme und Kopf, alles über dem karierten Quadrat der Decke."),
        retained=text("Every part of the body down to 7.5 mm; pelvis and trunk as flat outlines of their bone, limbs as revolved radius profiles, the apple as a body of revolution with the dimple at its top, the blanket as its true 6 mm edge in the elevations and its full outline in the plan.", "Jedes Körperteil bis 7,5 mm; Becken und Rumpf als flache Umrisse ihres Knochens, Gliedmaßen als gedrehte Radiusprofile, der Apfel als Drehkörper mit der Mulde oben, die Decke als ihre wahre Kante von 6 mm in den Ansichten und mit ihrem vollen Umriss im Grundriss."),
        merged=text("Fingers, thumb and palm into one hand outline; nose into the head; heel and sole into one sole shape per boot.", "Finger, Daumen und Handfläche in einen Handumriss; Nase in den Kopf; Absatz und Sohle in eine Sohlenform je Stiefel."),
        omitted=text("Buttons, the stalk of the apple, the fringe of the blanket, the boot shaft under the jeans; the far eye, brow and ear in the side view.", "Knöpfe, der Stiel des Apfels, die Fransen der Decke, der Stiefelschaft unter der Jeans; das ferne Auge, Braue und Ohr in der Seitenansicht."),
        thinElements=text("The 6 mm blanket is drawn at its true thickness (0.12 mm on paper) inside its outline; the placket is two 0.13 mm lines; nothing is thickened.", "Die 6 mm dicke Decke ist in ihrer wahren Dicke gezeichnet (0,12 mm auf dem Papier); die Knopfleiste sind zwei 0,13-mm-Linien; nichts wird verdickt."),
        hatching=text("The blanket in the plan carries a gingham cross hatch at 1.2 mm; tones separate skin, shirt, denim, leather and apple.", "Die Decke trägt im Grundriss eine Vichy-Kreuzschraffur mit 1,2 mm; Töne trennen Haut, Hemd, Jeans, Leder und Apfel."),
        styles=text("0.25 mm outlines, 0.18 mm detail (eyes, brows, mouth, pockets), 0.13 mm texture (placket, pocket flaps), the hatch at 0.10 mm.", "0,25-mm-Umrisse, 0,18-mm-Detail (Augen, Brauen, Mund, Taschen), 0,13-mm-Textur (Knopfleiste, Taschenklappen), die Schraffur mit 0,10 mm."),
        overlays=text("The layer order of a view is the order of the parts along its viewing direction in the pose; the blanket lies under everything; details lie on the mass that carries them.", "Die Ebenenfolge einer Ansicht ist die Reihenfolge der Teile entlang ihrer Blickrichtung in der Pose; die Decke liegt unter allem; Details liegen auf der Masse, die sie trägt.")),
    "100": dict(
        silhouette=text("The orthographic views of the 1:100 body cross-legged on the blanket: head with hair and beard, trunk with collar, sleeves, forearms, hands, thighs, shanks, boots, the apple at the mouth and the blanket.", "Die orthogonalen Ansichten des 1:100-Körpers im Schneidersitz auf der Decke: Kopf mit Haar und Bart, Rumpf mit Kragen, Ärmel, Unterarme, Hände, Oberschenkel, Unterschenkel, Stiefel, der Apfel am Mund und die Decke."),
        retained=text("The crossed legs, the apple and the hands, the beard and the collar, the gingham check of the blanket.", "Die gekreuzten Beine, der Apfel und die Hände, Bart und Kragen, das Karo der Decke."),
        merged=text("Eyes, brows and mouth into the head; pockets into the shirt; toe caps into the boots.", "Augen, Brauen und Mund in den Kopf; Taschen in das Hemd; Zehenkappen in die Stiefel."),
        omitted=text("Buttons, placket, face details, the dimple of the apple.", "Knöpfe, Knopfleiste, Gesichtsdetails, die Mulde des Apfels."),
        thinElements=text("The blanket edge is 0.06 mm thick inside a 0.20 mm outline; nothing is thickened.", "Die Deckenkante ist 0,06 mm dick in einem Umriss von 0,20 mm; nichts wird verdickt."),
        hatching=text("Gingham cross hatch on the blanket in the plan at 0.8 mm.", "Vichy-Kreuzschraffur der Decke im Grundriss mit 0,8 mm."),
        styles=text("0.20 mm outlines, 0.13 mm detail, the hatch at 0.10 mm.", "0,20-mm-Umrisse, 0,13-mm-Detail, die Schraffur mit 0,10 mm."),
        overlays=text("Layer order along the viewing direction of each view; the blanket under everything.", "Ebenenfolge entlang der Blickrichtung jeder Ansicht; die Decke unter allem.")),
    "200": dict(
        silhouette=text("The orthographic views of the 1:200 body: one shape per segment, the trunk, the head with hair, sleeves, forearms and fists, thighs, shanks and boots, the apple and the blanket.", "Die orthogonalen Ansichten des 1:200-Körpers: eine Form je Segment, Rumpf, Kopf mit Haar, Ärmel, Unterarme und Fäuste, Oberschenkel, Unterschenkel und Stiefel, der Apfel und die Decke."),
        retained=text("The cross-legged posture, the hand at the mouth with the apple (0.38 mm) and the blanket as the ground of the figure.", "Die Haltung im Schneidersitz, die Hand mit dem Apfel am Mund (0,38 mm) und die Decke als Grund der Figur."),
        merged=text("Collar into the shirt, nose, ears and beard into the head, hands into fists, toe caps into the boots.", "Kragen in das Hemd, Nase, Ohren und Bart in den Kopf, Hände in Fäuste, Zehenkappen in die Stiefel."),
        omitted=text("Every detail of face and clothing, the check of the blanket.", "Jedes Detail von Gesicht und Kleidung, das Karo der Decke."),
        thinElements=text("The blanket is a 0.03 mm band inside its outline; nothing is thickened.", "Die Decke ist ein Band von 0,03 mm in ihrem Umriss; nichts wird verdickt."),
        hatching=text("None.", "Keine."),
        styles=text("0.18 mm outlines.", "0,18-mm-Umrisse."),
        overlays=text("Layer order along the viewing direction of each view.", "Ebenenfolge entlang der Blickrichtung jeder Ansicht.")),
    "500": dict(
        silhouette=text("The orthographic views of the 1:500 body: trunk, head with hair, arms with hands, legs with boots, the apple at the floor size of 75 mm and the blanket as a thin slab of 2.8 by 2.4 mm.", "Die orthogonalen Ansichten des 1:500-Körpers: Rumpf, Kopf mit Haar, Arme mit Händen, Beine mit Stiefeln, der Apfel in der Mindestgröße von 75 mm und die Decke als dünne Platte von 2,8 mal 2,4 mm."),
        retained=text("A figure sitting cross-legged on a blanket, readable at a glance.", "Eine im Schneidersitz auf einer Decke sitzende Figur, auf einen Blick lesbar."),
        merged=text("Neck into the head, every garment detail into its mass.", "Hals in den Kopf, jedes Kleidungsdetail in seine Masse."),
        omitted=text("Fingers, face, collar, the check of the blanket.", "Finger, Gesicht, Kragen, das Karo der Decke."),
        thinElements=text("Hands are drawn at 75 mm; the apple keeps the floor size of 75 mm (its true 76 mm); the blanket stays at its true thickness inside a 0.15 mm outline.", "Hände sind mit 75 mm gezeichnet; der Apfel behält die Mindestgröße von 75 mm (wahr 76 mm); die Decke bleibt in wahrer Dicke in einem Umriss von 0,15 mm."),
        hatching=text("None.", "Keine."),
        styles=text("0.15 mm outlines.", "0,15-mm-Umrisse."),
        overlays=text("Layer order along the viewing direction of each view.", "Ebenenfolge entlang der Blickrichtung jeder Ansicht.")),
}
EXTRA = {
    "50": text("Blanket and apple at 1:50: the blanket is a sheet 1.4 by 1.2 m and 6 mm thick lying on the ground, the root of the rig, so it never moves; the apple is a body of revolution 76 mm across with the dimple at its top, held by the curled fingers of the right hand.", "Decke und Apfel bei 1:50: Die Decke ist eine Platte von 1,4 mal 1,2 m und 6 mm Dicke auf dem Boden, die Wurzel des Skeletts, und bewegt sich nie; der Apfel ist ein Drehkörper von 76 mm Durchmesser mit der Mulde oben, gehalten von den gebeugten Fingern der rechten Hand."),
    "100": text("Blanket and apple at 1:100: the blanket as a declared sheet, the apple as a body of revolution with its dimple.", "Decke und Apfel bei 1:100: die Decke als deklarierte Platte, der Apfel als Drehkörper mit seiner Mulde."),
    "200": text("Blanket and apple at 1:200: the blanket as a declared sheet, the apple as a plain body of revolution.", "Decke und Apfel bei 1:200: die Decke als deklarierte Platte, der Apfel als einfacher Drehkörper."),
    "500": text("Blanket and apple at 1:500: the blanket as a declared sheet, the apple as a ball of 75 mm, the floor size of the scale.", "Decke und Apfel bei 1:500: die Decke als deklarierte Platte, der Apfel als Kugel von 75 mm, der Mindestgröße des Maßstabs."),
}


def swap(t):
    for a_, b_ in [("a man of 1.76 m seated on a 24 inch stool", "a man of 1.70 m sitting cross-legged on a picnic blanket"), ("ein Mann von 1,76 m auf einem 24-Zoll-Hocker", "ein Mann von 1,70 m im Schneidersitz auf einer Picknickdecke"),
                   ("; guitar and stool by reference", ""), ("; Gitarre und Hocker per Verweis", ""), (" of the player", ""), ("flannel", "check"), ("Flanell", "Karo")]:
        t = t.replace(a_, b_)
    return t


def apply(a):
    for sc in SCALES:
        a["representations"][sc]["decisions"] = D[sc]
        keep = [d for d in a["volumes"][sc]["decisions"] if not d["en"].startswith(("Guitar", "Stool", "The fretting hand", "Head and hands"))]
        a["volumes"][sc]["decisions"] = [{"en": swap(d["en"]), "de": swap(d["de"])} for d in keep] + [EXTRA[sc]]
