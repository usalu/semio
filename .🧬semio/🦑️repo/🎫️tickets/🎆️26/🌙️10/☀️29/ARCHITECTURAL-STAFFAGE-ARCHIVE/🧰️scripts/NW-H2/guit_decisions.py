from lib import tx
HATCHES = {}


def decisions():
    d = {}
    d[500] = dict(
        silhouette=("Seated figure of eleven overlapping dark forms (head with hair, shirt, pelvis, two thighs, two shanks, two boots, two arms), the guitar as one body-and-neck form on the thigh, and the stool as a seat slab with two splayed legs.", "Sitzende Figur aus elf überlappenden dunklen Formen (Kopf mit Haar, Hemd, Becken, zwei Oberschenkel, zwei Unterschenkel, zwei Stiefel, zwei Arme), die Gitarre als eine Korpus-Hals-Form auf dem Oberschenkel und der Hocker als Sitzplatte mit zwei gespreizten Beinen."),
        retained=("Seated posture with the raised near foot on the rung, the guitar neck rising forward, the 0.56 m seat height; real height 1.48 m and width of the default pose.", "Sitzhaltung mit dem erhöhten nahen Fuß auf der Fußraste, der nach vorn ansteigende Gitarrenhals, die Sitzhöhe von 0,56 m; reale Höhe 1,48 m und Breite der Standardpose."),
        merged=("Guitar body, neck and headstock into one form; face and hair into one head; forearm and hand into one arm form; the stool rung is left out and the stool legs carry the stool region.", "Gitarrenkorpus, Hals und Kopfplatte zu einer Form; Gesicht und Haar zu einem Kopf; Unterarm und Hand zu einer Armform; die Hocker-Fußraste entfällt, die Hockerbeine tragen die Hockerregion."),
        omitted=("Face details, beard, soundhole, bridge, frets, strings, tuners, fingers, rung, cuffs and boot soles.", "Gesichtsdetails, Bart, Schallloch, Steg, Bünde, Saiten, Wirbel, Finger, Fußraste, Bündchen und Stiefelsohlen."),
        thinElements=("The guitar neck (0.045 m) is drawn 0.10 m wide and the stool legs 0.10 m wide, the seat 0.10 m thick (feature limit 0.2 mm = 0.1 m); the foot rung (0.028 m) is dropped.", "Der Gitarrenhals (0,045 m) wird 0,10 m breit und die Hockerbeine 0,10 m breit, der Sitz 0,10 m dick gezeichnet (Mindestbreite 0,2 mm = 0,1 m); die Fußraste (0,028 m) entfällt."),
        hatching=("None.", "Keine."),
        styles=("Solid dark fills with a 0.15 mm outline; guitar and stool mid-grey.", "Flächig dunkle Füllungen mit 0,15-mm-Kontur; Gitarre und Hocker mittelgrau."),
        overlays=("Merged forms list their regions: head (face, hair), forearms (sleeves, hands); the guitar form carries the guitar region, the stool forms the stool region.", "Zusammengefasste Formen nennen ihre Regionen: Kopf (Gesicht, Haar), Unterarme (Ärmel, Hände); die Gitarrenform trägt die Gitarrenregion, die Hockerformen die Hockerregion."))
    d[200] = dict(
        silhouette=("Head with separate hair cap, flannel shirt, pelvis, jeans in thigh and shank per leg, boots, sleeve and tapered hand per arm; guitar as body, neck and headstock; stool as seat, two legs and a rung.", "Kopf mit getrennter Haarkappe, Flanellhemd, Becken, Jeans je Bein in Ober- und Unterschenkel, Stiefel, je Arm Ärmel und spitz zulaufende Hand; Gitarre als Korpus, Hals und Kopfplatte; Hocker als Sitz, zwei Beine und Fußraste."),
        retained=("Tones for shirt, jeans, skin, hair, boots, spruce top, neck and pine stool; neck drawn 0.06 m wide.", "Töne für Hemd, Jeans, Haut, Haar, Stiefel, Fichtendecke, Hals und Kiefernhocker; Hals 0,06 m breit gezeichnet."),
        merged=("Fingers into the hand tip; forearm cuff into the sleeve; boot and sock.", "Finger in der Handspitze; Ärmelbündchen im Ärmel; Stiefel und Socke."),
        omitted=("Soundhole, bridge, frets, strings, tuners, beard, ear, eye, cuffs, boot sole line.", "Schallloch, Steg, Bünde, Saiten, Wirbel, Bart, Ohr, Auge, Bündchen, Sohlenlinie."),
        thinElements=("The neck is drawn 0.06 m instead of 0.045 m wide and the stool legs 0.05 m instead of 0.04 m (feature limit 0.04 m); the rung stays 0.028 m and is joined to both legs.", "Der Hals wird 0,06 m statt 0,045 m breit und die Hockerbeine 0,05 m statt 0,04 m gezeichnet (Mindestbreite 0,04 m); die Fußraste bleibt 0,028 m und ist mit beiden Beinen verbunden."),
        hatching=("None.", "Keine."),
        styles=("0.18 mm outlines; light skin, mid-grey shirt and jeans, dark boots and hair, light wood.", "0,18-mm-Konturen; helle Haut, mittelgraues Hemd und Jeans, dunkle Stiefel und Haar, helles Holz."),
        overlays=("Every region is its own form: face, hair, shirt trunk, sleeve, hand, jeans, boot, guitar and stool.", "Jede Region ist eine eigene Form: Gesicht, Haar, Hemdrumpf, Ärmel, Hand, Jeans, Stiefel, Gitarre und Hocker."))
    d[100] = dict(
        silhouette=("Clothing outlines of shirt, jeans and boots with a sole line, mitten hands, face profile with eye under a hair cap, guitar with soundhole and bridge, stool with seat, legs and rung.", "Kleidungsumrisse von Hemd, Jeans und Stiefeln mit Sohlenlinie, Fäustlingshände, Gesichtsprofil mit Auge unter einer Haarkappe, Gitarre mit Schallloch und Steg, Hocker mit Sitz, Beinen und Fußraste."),
        retained=("Soundhole, bridge, boot sole line and eye; neck 0.048 m wide.", "Schallloch, Steg, Stiefelsohlenlinie und Auge; Hals 0,048 m breit."),
        merged=("Fingers into the mitten; sock into the boot; hair and beard into one cap.", "Finger im Fäustling; Socke im Stiefel; Haar und Bart in einer Kappe."),
        omitted=("Frets, strings, tuners, ear, cuffs, shirt placket, thumb.", "Bünde, Saiten, Wirbel, Ohr, Bündchen, Hemdleiste, Daumen."),
        thinElements=("Soundhole ring is a filled form 0.10 m wide; the neck 0.048 m (0.48 mm); lines at 0.13 mm.", "Der Schalllochring ist eine Vollform von 0,10 m; der Hals 0,048 m (0,48 mm); Linien mit 0,13 mm."),
        hatching=("None.", "Keine."),
        styles=("0.20 mm outlines and 0.13 mm detail lines.", "0,20-mm-Konturen und 0,13-mm-Detaillinien."),
        overlays=("Regions map one to one onto forms; soundhole and bridge lie inside the guitar region.", "Regionen entsprechen den Formen eins zu eins; Schallloch und Steg liegen innerhalb der Gitarrenregion."))
    d[50] = dict(
        silhouette=("Full articulation: face profile with ear, eye and beard under the hair, flannel shirt with opening and pocket flap, mittens with thumbs and cuffs, boots with sole lines, guitar with frets, string, tuners, soundhole and bridge, stool with rung.", "Volle Gliederung: Gesichtsprofil mit Ohr, Auge und Bart unter dem Haar, Flanellhemd mit Öffnung und Taschenklappe, Fäustlinge mit Daumen und Bündchen, Stiefel mit Sohlenlinien, Gitarre mit Bünden, Saite, Wirbeln, Schallloch und Steg, Hocker mit Fußraste."),
        retained=("Every construction element of the instrument and the clothing.", "Jedes Konstruktionselement von Instrument und Kleidung."),
        merged=("Strings reduced to one line; fingers into the mitten.", "Saiten auf eine Linie reduziert; Finger im Fäustling."),
        omitted=("Five of six strings, rosette, fret markers, stool joinery, fabric checks (below 0.1 mm paper).", "Fünf von sechs Saiten, Rosette, Bundmarkierungen, Hocker-Verbindungen, Stoffkaros (unter 0,1 mm Papier)."),
        thinElements=("Frets are 0.13 mm lines at 0.8 mm spacing; no feature is thickened.", "Bünde sind 0,13-mm-Linien im Abstand von 0,8 mm; nichts wird verdickt."),
        hatching=("None; frets and the string carry the wood detail.", "Keine; Bünde und Saite tragen das Holzdetail."),
        styles=("0.25 mm silhouettes, 0.18 mm detail, 0.13 mm texture.", "0,25-mm-Silhouetten, 0,18-mm-Detail, 0,13-mm-Textur."),
        overlays=("Every region is its own form; detail lines lie inside their region forms.", "Jede Region ist eine eigene Form; Detaillinien liegen innerhalb ihrer Regionsformen."))
    return d
