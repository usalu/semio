from lib import tx
HATCHES = {}


def unavailable(den, size, reason, feature):
    return {"status": "unavailable", "denominator": den, "reason": reason, "limitingFeature": feature, "limitingSizeM": size, "alternatives": [{"kind": "scale", "denominator": 100}]}


def unavailable500():
    return unavailable(500, 0.07,
        tx("At 1:500 the 0.87 m toddler is 1.7 mm high and its arms, boots and mittens (0.06 to 0.08 m) are below the 0.2 mm feature limit; merging the two bodies into one form would lose the carried second person that this asset is about.", "Bei 1:500 ist das 0,87 m große Kleinkind 1,7 mm hoch, seine Arme, Stiefel und Fäustlinge (0,06 bis 0,08 m) liegen unter der Mindestbreite von 0,2 mm; ein Zusammenfassen beider Körper zu einer Form würde die getragene zweite Person verlieren, um die es bei diesem Asset geht."),
        tx("Toddler arm and boot width", "Breite von Arm und Stiefel des Kleinkinds"))


def unavailable200():
    return unavailable(200, 0.075,
        tx("At 1:200 the two bodies overlap along the hip, chest and arms: the gaps between the toddler's head, thighs and boots and the parent's coat, arm and thigh are 0.07 to 0.15 m, below the 0.15 mm (0.03 m) minimum white gap plus line width, so they print as blots; the drawing cannot keep both persons readable without merging them.", "Bei 1:200 überlagern sich die beiden Körper an Hüfte, Brust und Armen: Die Abstände zwischen Kopf, Oberschenkeln und Stiefeln des Kleinkinds und Mantel, Arm und Oberschenkel der Eltern betragen 0,07 bis 0,15 m und liegen unter der Mindestlücke von 0,15 mm (0,03 m) zuzüglich Linienbreite, sie drucken daher als Flecken; ohne Zusammenfassen bleiben beide Personen nicht lesbar."),
        tx("Gaps between the carried toddler and the parent", "Abstände zwischen getragenem Kleinkind und Elternteil"))


def decisions():
    d = {}
    d[100] = dict(
        silhouette=("Parent as head with beanie and face profile, coat with a separate skirt (the scarf is part of the coat form), sleeve and a forearm-with-glove form per arm, jeans legs in thigh and shank and boots; the toddler as padded trunk, head with hat, near leg with boot and near arm with mitten reaching to the parent's neck.", "Elternteil aus Kopf mit Mütze und Gesichtsprofil, Mantel mit getrenntem Schoß (der Schal gehört zur Mantelform), Ärmel und einer Unterarm-mit-Handschuh-Form je Arm, Jeansbeinen aus Ober- und Unterschenkel und Stiefeln; das Kleinkind als gepolsterter Rumpf, Kopf mit Mütze, nahes Bein mit Stiefel und naher Arm mit Fäustling bis zum Hals der Eltern."),
        retained=("The carried second body with its three contacts (seat on the forearm, arm round the neck, legs in front of the parent); coat skirt, both boots; real parent height 1.73 m.", "Der getragene zweite Körper mit seinen drei Kontakten (Sitz auf dem Unterarm, Arm um den Hals, Beine vor den Eltern); Mantelschoß, beide Stiefel; reale Elternhöhe 1,73 m."),
        merged=("Scarf into the coat form; glove into the forearm form; mitten into the toddler's arm form; pelvis into coat skirt and thighs. The toddler's far leg and far arm are hidden behind the near ones and are not drawn.", "Schal in der Mantelform; Handschuh in der Unterarmform; Fäustling in der Armform des Kleinkinds; Becken in Mantelschoß und Oberschenkeln. Das ferne Bein und der ferne Arm des Kleinkinds sind hinter den nahen verdeckt und werden nicht gezeichnet."),
        omitted=("Eyes, ear, mouth, seams, laces, buttons, hat pompom, thumbs.", "Augen, Ohr, Mund, Nähte, Schnürung, Knöpfe, Mützenbommel, Daumen."),
        thinElements=("The toddler's arm is 0.07 m (0.7 mm); the glove end tapers like the real hand; nothing is thickened.", "Der Arm des Kleinkinds ist 0,07 m (0,7 mm) breit; das Handschuhende läuft wie die echte Hand spitz zu; nichts wird verdickt."),
        hatching=("None; tones carry the materials.", "Keine; Töne tragen die Materialien."),
        styles=("0.20 mm outlines; tones separate the parent's coat, jeans, boots and skin from the toddler's overall.", "0,20-mm-Konturen; Töne trennen Mantel, Jeans, Stiefel und Haut der Eltern vom Overall des Kleinkinds."),
        overlays=("Merged forms list their regions: coat (coat, scarf), forearm forms (sleeve, glove), toddler arm (overall, mitten); every other region is its own form and the toddler's regions sit on the toddler's forms.", "Zusammengefasste Formen nennen ihre Regionen: Mantel (Mantel, Schal), Unterarmformen (Ärmel, Handschuh), Arm des Kleinkinds (Overall, Fäustling); jede andere Region ist eine eigene Form, die Regionen des Kleinkinds liegen auf dessen Formen."))
    d[50] = dict(
        silhouette=("Parent with face profile, ear and eye under the beanie, coat, separate scarf and skirt, sleeves and forearm-with-glove forms, jeans and boots; toddler with padded body, face profile, hat, near leg with rubber boot and near arm with mitten.", "Elternteil mit Gesichtsprofil, Ohr und Auge unter der Mütze, Mantel, getrenntem Schal und Schoß, Ärmeln und Unterarm-mit-Handschuh-Formen, Jeans und Stiefeln; Kleinkind mit gepolstertem Körper, Gesichtsprofil, Mütze, nahem Bein mit Gummistiefel und nahem Arm mit Fäustling."),
        retained=("The scarf as its own form and the eye and ear lines; real parent height 1.73 m.", "Der Schal als eigene Form sowie Augen- und Ohrlinie; reale Elternhöhe 1,73 m."),
        merged=("Glove into the forearm form, mitten into the toddler's arm form; pelvis into coat skirt and thighs; the toddler's far limbs are hidden behind the near ones.", "Handschuh in der Unterarmform, Fäustling in der Armform des Kleinkinds; Becken in Mantelschoß und Oberschenkeln; die fernen Gliedmaßen des Kleinkinds sind hinter den nahen verdeckt."),
        omitted=("Eyelashes, mouth, stitching, fabric folds, laces (below 0.1 mm paper).", "Wimpern, Mund, Nähte, Stofffalten, Schnürung (unter 0,1 mm Papier)."),
        thinElements=("No feature is thickened; the eye is 0.012 m wide (0.24 mm).", "Nichts wird verdickt; das Auge ist 0,012 m breit (0,24 mm)."),
        hatching=("None.", "Keine."),
        styles=("0.25 mm silhouettes, 0.18 mm detail lines.", "0,25-mm-Silhouetten, 0,18-mm-Detaillinien."),
        overlays=("Every region is its own form or part of a named merged form.", "Jede Region ist eine eigene Form oder Teil einer benannten zusammengefassten Form."))
    return d
