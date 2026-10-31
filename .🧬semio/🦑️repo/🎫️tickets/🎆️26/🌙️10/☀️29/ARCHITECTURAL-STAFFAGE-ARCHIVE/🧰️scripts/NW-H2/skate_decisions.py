from lib import tx
HATCHES = {50: [{"id": "knit-rib", "title": tx("Knit ribs", "Strickrippen"), "pattern": "lines", "angleDeg": 80, "spacingMm": 0.5, "penMm": 0.13}]}


def _un(den, size, reason, feature):
    return {"status": "unavailable", "denominator": den, "reason": reason, "limitingFeature": feature, "limitingSizeM": size, "alternatives": [{"kind": "scale", "denominator": 100}]}


def unavailable500():
    return _un(500, 0.012,
        tx("At 1:500 the 1.77 m rider is 3.5 mm tall and the board is a 0.012 m deck on 0.054 m wheels under a 0.085 m ride height: the deck, trucks and wheels cannot be drawn apart (0.012 m = 0.024 mm) and the rider's limbs (0.07 to 0.09 m) lie 0.04 to 0.15 m from each other, closer than the 0.15 mm minimum white gap plus line width, so arms, legs and board print as one blot that no longer shows that a person rides a skateboard.", "Bei 1:500 ist der 1,77 m große Fahrer 3,5 mm hoch und das Brett ein 0,012 m dickes Deck auf 0,054 m großen Rollen bei 0,085 m Fahrhöhe: Deck, Achsen und Rollen lassen sich nicht trennen (0,012 m = 0,024 mm), und die Gliedmaßen des Fahrers (0,07 bis 0,09 m) liegen 0,04 bis 0,15 m auseinander, enger als die Mindestlücke von 0,15 mm zuzüglich Linienbreite, sodass Arme, Beine und Brett als ein Fleck drucken, der nicht mehr zeigt, dass eine Person Skateboard fährt."),
        tx("Deck thickness and limb gaps", "Deckdicke und Gliedmaßenabstände"))


def unavailable200():
    return _un(200, 0.012,
        tx("At 1:200 the 0.012 m deck is 0.06 mm thick and the 0.054 m wheels are 0.27 mm under a deck ride height of 0.5 mm; the white gaps between wheels, trucks and deck, between the two legs and between arms and torso are 0.03 to 0.1 m (0.15 to 0.5 mm) and fall below the minimum white gap plus line width, so the board and the rider's limbs merge in print.", "Bei 1:200 ist das 0,012 m dicke Deck 0,06 mm dick und sind die 0,054 m großen Rollen 0,27 mm groß bei 0,5 mm Fahrhöhe; die weißen Lücken zwischen Rollen, Achsen und Deck, zwischen den beiden Beinen und zwischen Armen und Rumpf betragen 0,03 bis 0,1 m (0,15 bis 0,5 mm) und unterschreiten die Mindestlücke zuzüglich Linienbreite, sodass Brett und Gliedmaßen des Fahrers im Druck verschmelzen."),
        tx("Wheel, truck and deck gaps", "Lücken zwischen Rolle, Achse und Deck"))


def decisions():
    d = {}
    d[100] = dict(
        silhouette=("Clothing outlines: hoodie with a bunched hood, trousers per leg in thigh and shank, one-piece skate shoes, a sleeve and a forearm-with-mitten form per arm, face profile under a beanie, and the board as a deck on two trucks above two wheels.", "Kleidungsumrisse: Hoodie mit Kapuze, Hose je Bein in Ober- und Unterschenkel, einteilige Skateschuhe, je Arm eine Ärmel- und eine Unterarm-mit-Fäustling-Form, Gesichtsprofil unter einer Mütze, dazu das Brett als Deck auf zwei Achsen über zwei Rollen."),
        retained=("Crouched stance with the feet over the trucks, hood, beanie, real wheel diameter 0.054 m and deck length 0.81 m with kicked nose and tail.", "Geduckte Haltung mit den Füßen über den Achsen, Kapuze, Mütze, realer Raddurchmesser 0,054 m und Decklänge 0,81 m mit hochgezogenen Enden."),
        merged=("Forearm, glove and fingers form one arm piece; sole and upper form one shoe; trucks hang below the deck with the wheels half covered by them; the pelvis is covered by hoodie hem and thighs.", "Unterarm, Handschuh und Finger bilden ein Armteil; Sohle und Schaft bilden einen Schuh; die Achsen hängen unter dem Deck und verdecken die Rollen teilweise; das Becken ist von Hoodiesaum und Oberschenkeln verdeckt."),
        omitted=("Eye, ear, mouth, thumbs, drawstrings, pocket, seams, laces, grip tape, baseplates, bearings, trouser cuffs.", "Auge, Ohr, Mund, Daumen, Kordeln, Tasche, Nähte, Schnürung, Griptape, Grundplatten, Lager, Hosenumschläge."),
        thinElements=("The deck is drawn at its real 0.012 m (0.12 mm) with a 0.2 mm outline pen; the shoes reach 0.016 m below the deck top so they cross the deck outline instead of running parallel to it.", "Das Deck ist mit seiner realen Dicke von 0,012 m (0,12 mm) und 0,2-mm-Kontur gezeichnet; die Schuhe reichen 0,016 m unter die Deckoberkante, damit sie die Deckkontur kreuzen statt parallel zu ihr zu laufen."),
        hatching=("None; tones carry the materials.", "Keine; Töne tragen die Materialien."),
        styles=("0.20 mm outlines; light skin and shoes, mid-grey hoodie, dark trousers and beanie, white wheels.", "0,20-mm-Konturen; helle Haut und Schuhe, mittelgrauer Hoodie, dunkle Hose und Mütze, weiße Rollen."),
        overlays=("Forearm forms list sleeve and hand regions; every other region is its own form: face, beanie, hoodie, trousers, shoes, deck, trucks, wheels.", "Unterarmformen nennen die Regionen Ärmel und Hand; jede andere Region ist eine eigene Form: Gesicht, Mütze, Hoodie, Hose, Schuhe, Deck, Achsen, Rollen."))
    d[50] = dict(
        silhouette=("As at 1:100 with longer sleeves, a sole line and trouser cuffs, the eye in the face profile and a ribbed beanie: hoodie with hood, trousers, one-piece shoes, arms with mitten ends, face profile, deck, trucks and wheels.", "Wie bei 1:100 mit längeren Ärmeln, Sohlenlinie und Hosenumschlägen, dem Auge im Gesichtsprofil und einer gerippten Mütze: Hoodie mit Kapuze, Hose, einteilige Schuhe, Arme mit Fäustlingsenden, Gesichtsprofil, Deck, Achsen und Rollen."),
        retained=("Cuffs, sole lines and the eye as true details inside their forms.", "Umschläge, Sohlenlinien und das Auge als echte Details innerhalb ihrer Formen."),
        merged=("Forearm and mitten into one form; sole into the shoe; pelvis into hoodie hem and thighs.", "Unterarm und Fäustling zu einer Form; Sohle im Schuh; Becken in Hoodiesaum und Oberschenkeln."),
        omitted=("Ear, mouth, fingers, thumbs, drawstrings, pocket, laces, grip tape, baseplates, bearings, seams (below 0.1 mm paper or too close to other outlines to print apart).", "Ohr, Mund, Finger, Daumen, Kordeln, Tasche, Schnürung, Griptape, Grundplatten, Lager, Nähte (unter 0,1 mm Papier oder zu dicht an anderen Konturen, um getrennt zu drucken)."),
        thinElements=("Details are 0.13 to 0.18 mm lines inside large forms; nothing is thickened.", "Details sind 0,13- bis 0,18-mm-Linien innerhalb großer Formen; nichts wird verdickt."),
        hatching=("Knit rib hatch on the beanie (0.5 mm spacing, 0.13 mm pen).", "Strickrippen-Schraffur auf der Mütze (0,5 mm Abstand, 0,13 mm Stift)."),
        styles=("0.25 mm silhouettes, 0.18 mm detail, 0.13 mm texture.", "0,25-mm-Silhouetten, 0,18-mm-Detail, 0,13-mm-Textur."),
        overlays=("Same mapping as at 1:100; detail lines lie inside their region forms.", "Gleiche Zuordnung wie bei 1:100; Detaillinien liegen innerhalb ihrer Regionsformen."))
    return d
