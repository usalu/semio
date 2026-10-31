from lib import tx
HATCHES = {50: [{"id": "straw-weave", "title": tx("Straw weave", "Strohgeflecht"), "pattern": "lines", "angleDeg": 35, "spacingMm": 0.5, "penMm": 0.13}]}


def _un(den, size, reason, feature):
    return {"status": "unavailable", "denominator": den, "reason": reason, "limitingFeature": feature, "limitingSizeM": size, "alternatives": [{"kind": "scale", "denominator": 100}]}


def unavailable500():
    return _un(500, 0.04,
        tx("At 1:500 the 1.16 m high kneeling figure is 2.3 mm tall: the folded limbs, the gloved hand holding a 0.04 m wide trowel and the 0.04 m kneeling pad are below the 0.2 mm feature limit, and arms, thigh and torso lie 0.03 to 0.12 m apart, closer than the minimum white gap of 0.15 mm plus line width, so they print as one blot.", "Bei 1:500 ist die 1,16 m hohe kniende Figur 2,3 mm groß: die gefalteten Gliedmaßen, die behandschuhte Hand mit der 0,04 m breiten Schaufel und das 0,04 m dicke Kniekissen liegen unter der Mindestbreite von 0,2 mm, und Arme, Oberschenkel und Rumpf liegen 0,03 bis 0,12 m auseinander, enger als die Mindestlücke von 0,15 mm zuzüglich Linienbreite, sodass sie als ein Fleck drucken."),
        tx("Trowel width and arm-to-thigh gap", "Schaufelbreite und Abstand zwischen Arm und Oberschenkel"))


def unavailable200():
    return _un(200, 0.04,
        tx("At 1:200 the 0.04 m trowel, the 0.04 m thick hat brim and the pad are 0.2 mm wide or less, and the limbs of the leaning figure lie 0.03 to 0.10 m apart (0.15 to 0.5 mm): the gaps between arms, thigh and torso fall below the 0.15 mm minimum white gap plus line width and the hat brim falls below the feature limit, so no drawing keeps them apart without merging the working arm into the body.", "Bei 1:200 sind die 0,04 m breite Schaufel, die 0,04 m dicke Hutkrempe und das Kissen höchstens 0,2 mm breit, und die Gliedmaßen der vorgebeugten Figur liegen 0,03 bis 0,10 m (0,15 bis 0,5 mm) auseinander: Die Abstände zwischen Armen, Oberschenkel und Rumpf liegen unter der Mindestlücke von 0,15 mm zuzüglich Linienbreite und die Hutkrempe unter der Mindestbreite, sodass keine Zeichnung sie getrennt hält, ohne den arbeitenden Arm mit dem Körper zu verschmelzen."),
        tx("Hat brim thickness and arm gaps", "Hutkrempendicke und Armabstände"))


def decisions():
    d = {}
    d[100] = dict(
        silhouette=("Clothing outlines: shirt with leaning back, trousers of the near leg with a knee patch ellipse, clog with strap, a forearm form per arm that runs on into the glove, hat with crown, brim and band, face profile with eye, trowel blade, pad, trug with rim and weeds.", "Kleidungsumrisse: Hemd mit geneigtem Rücken, Hose des nahen Beins mit Kniepolster-Ellipse, Clog mit Riemen, je Arm eine Unterarmform, die in den Handschuh übergeht, Hut mit Kopf, Krempe und Band, Gesichtsprofil mit Auge, Schaufelblatt, Kissen, Trog mit Rand und Unkraut."),
        retained=("Knee patch, clog strap, hat band, trug rim, eye and the rolled sleeve edge where the upper arm ends; real height 1.16 m and width 1.84 m of the weeding pose.", "Kniepolster, Clog-Riemen, Hutband, Trogrand, Auge und die hochgekrempelte Ärmelkante am Ende des Oberarms; reale Höhe 1,16 m und Breite 1,84 m der Jätepose."),
        merged=("Bare forearm and glove into one forearm form; fingers and thumb into the glove end; the trowel handle is hidden in the glove; the pelvis is covered by shirt hem and thigh. The far leg is hidden behind the near leg and not drawn.", "Nackter Unterarm und Handschuh zu einer Unterarmform; Finger und Daumen im Handschuhende; der Schaufelgriff steckt im Handschuh; das Becken ist von Hemdsaum und Oberschenkel verdeckt. Das ferne Bein ist hinter dem nahen verdeckt und wird nicht gezeichnet."),
        omitted=("Ear, mouth, placket, finger lines, trowel ferrule and handle, far clog.", "Ohr, Mund, Knopfleiste, Fingerlinien, Schaufelzwinge und -griff, ferner Clog."),
        thinElements=("The trowel blade is drawn 0.06 m wide (0.6 mm) with a blunt start inside the glove; the hat band is a 0.13 mm line; nothing is thickened.", "Das Schaufelblatt ist 0,06 m breit (0,6 mm) mit stumpfem Anfang im Handschuh gezeichnet; das Hutband ist eine 0,13-mm-Linie; nichts wird verdickt."),
        hatching=("None; tones carry the materials.", "Keine; Töne tragen die Materialien."),
        styles=("0.20 mm outlines, 0.13 mm detail lines; light shirt and hat, mid-grey trousers, dark clog.", "0,20-mm-Konturen, 0,13-mm-Detaillinien; helles Hemd und heller Hut, mittelgraue Hose, dunkler Clog."),
        overlays=("The forearm form lists forearm skin and glove regions; every other region is its own form; the knee patch and strap lines lie inside their regions.", "Die Unterarmform nennt die Regionen Unterarmhaut und Handschuh; jede andere Region ist eine eigene Form; Kniepolster- und Riemenlinien liegen innerhalb ihrer Regionen."))
    d[50] = dict(
        silhouette=("Full articulation of the near leg, both arms in sleeve, bare forearm and glove with an integrated thumb, linen shirt, face profile with eye under a woven hat with crown, brim and straw hatching, trowel blade, pad and trug with weeds and rim.", "Volle Gliederung des nahen Beins, beider Arme in Ärmel, nacktem Unterarm und Handschuh mit integriertem Daumen, Leinenhemd, Gesichtsprofil mit Auge unter einem Strohhut mit Kopf, Krempe und Strohschraffur, Schaufelblatt, Kissen und Trog mit Unkraut und Rand."),
        retained=("Knee patch, clog strap, the glove as its own form over the bare forearm, the straw weave hatching and the trug rim.", "Kniepolster, Clog-Riemen, der Handschuh als eigene Form über dem nackten Unterarm, die Strohgeflecht-Schraffur und der Trogrand."),
        merged=("The sock is part of the clog form; fingers and thumb are one glove outline; the pelvis is covered by shirt hem and thigh; the far leg is hidden behind the near one.", "Die Socke ist Teil der Clog-Form; Finger und Daumen sind eine Handschuhkontur; das Becken ist von Hemdsaum und Oberschenkel verdeckt; das ferne Bein ist hinter dem nahen verdeckt."),
        omitted=("Ear, mouth, placket, finger lines, trowel handle and ferrule, eyelashes, stitching (below 0.1 mm paper).", "Ohr, Mund, Knopfleiste, Fingerlinien, Schaufelgriff und -zwinge, Wimpern, Nähte (unter 0,1 mm Papier)."),
        thinElements=("The hat brim is 0.04 m thick (0.8 mm); the trowel blade 0.06 m wide; no feature is thickened.", "Die Hutkrempe ist 0,04 m dick (0,8 mm); das Schaufelblatt 0,06 m breit; nichts wird verdickt."),
        hatching=("Straw weave hatch on the hat crown (0.5 mm spacing, 0.13 mm pen).", "Strohgeflecht-Schraffur auf dem Hutkopf (0,5 mm Abstand, 0,13 mm Stift)."),
        styles=("0.25 mm silhouettes, 0.18 mm detail, 0.13 mm texture.", "0,25-mm-Silhouetten, 0,18-mm-Detail, 0,13-mm-Textur."),
        overlays=("Every region is its own form; detail lines lie inside the form of their region.", "Jede Region ist eine eigene Form; Detaillinien liegen innerhalb der Form ihrer Region."))
    return d
