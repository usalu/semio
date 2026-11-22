"""📝 Labrador: decisions (en, de) of every scale for the girdles on the lateral rest placement, the oval trunk and the rib cage.

Usage: python3 rework-p2-labrador-dog-decisions.py   (repeatable: its own sentences are replaced)
"""
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, 'volumes/animals/labrador-dog.json')
MARK = ('Girdles:', 'Trunk section:', 'Rib cage:')

GIRDLE = {'en': 'Girdles: each shoulder blade rests on the rib cage 14 degrees out of the sagittal plane, from its dorsal border 35 mm beside the withers to the shoulder joint 75 mm beside the plane, and swings in its own plane; each hip bone runs from the sacroiliac joint 26 mm beside the plane down and out to the hip joint 55 mm beside it. Upper arm and thigh hang from these bones, so the limbs leave the trunk at the true breadths (shoulder joints 0.15 m, hip joints 0.11 m apart).', 'de': 'Gürtel: jedes Schulterblatt liegt 14 Grad aus der Sagittalebene geneigt auf dem Brustkorb, vom Rückenrand 35 mm neben dem Widerrist bis zum Schultergelenk 75 mm neben der Ebene, und schwingt in seiner eigenen Ebene; jedes Hüftbein läuft vom Kreuzdarmbeingelenk 26 mm neben der Ebene nach unten außen zum Hüftgelenk 55 mm daneben. Oberarm und Oberschenkel hängen an diesen Knochen, die Gliedmaßen verlassen den Rumpf also in den wahren Breiten (Schultergelenke 0,15 m, Hüftgelenke 0,11 m auseinander).'}
TRUNK = {'en': 'Trunk section: the chest is an upright oval, 0.29 m deep and 0.25 m wide at the ninth rib, 0.20 m between the shoulders and 0.21 m at the loin, so shoulder and upper arm stand outside the rib cage and the waist shows from above; the asset is 0.258 m wide across the upper arms.', 'de': 'Rumpfquerschnitt: die Brust ist ein stehendes Oval, 0,29 m tief und 0,25 m breit an der neunten Rippe, 0,20 m zwischen den Schultern und 0,21 m an der Lende, sodass Schulter und Oberarm außen am Brustkorb liegen und die Taille von oben sichtbar ist; über die Oberarme ist das Tier 0,258 m breit.'}
RIBS = {
    '50': {'en': 'Rib cage: thirteen rib pairs as curved lofts of their true section (9 x 6 mm) from the vertebra outwards and caudoventrally to the costochondral junction, each with its costal cartilage: nine sternal pairs reach the sternum, the cartilages of ribs ten to twelve form the costal arch, the thirteenth rib floats. Radius and ulna are two bones: the radius cranial under the humeral condyle, the ulna with the olecranon behind the elbow tapering to the lateral styloid process.', 'de': 'Brustkorb: dreizehn Rippenpaare als gekrümmte Lofts im wahren Querschnitt (9 x 6 mm) vom Wirbel nach außen und schwanzbauchwärts bis zur Knorpelgrenze, jede mit ihrem Rippenknorpel: neun sternale Paare erreichen das Brustbein, die Knorpel der Rippen zehn bis zwölf bilden den Rippenbogen, die dreizehnte Rippe endet frei. Speiche und Elle sind zwei Knochen: die Speiche vorn unter der Oberarmrolle, die Elle mit dem Ellbogenhöcker hinter dem Gelenk, auslaufend zum seitlichen Griffelfortsatz.'},
    '100': {'en': 'Rib cage: seven typified rib pairs (ribs 1, 3, 5, 7, 9, 11, 13) as curved rods of 15 mm on the true rib lines; the sternal ones run over their cartilage to the sternum, ribs 11 and 13 end free. Radius and ulna stay one bar.', 'de': 'Brustkorb: sieben typisierte Rippenpaare (Rippen 1, 3, 5, 7, 9, 11, 13) als gekrümmte Stäbe von 15 mm auf den wahren Rippenlinien; die sternalen laufen über ihren Knorpel zum Brustbein, die Rippen 11 und 13 enden frei. Speiche und Elle bleiben ein Stab.'},
    '200': {'en': 'Rib cage: three typified rib pairs (ribs 2, 6, 10) as curved rods of 30 mm drawn in under the skin; ribs 2 and 6 reach the sternum bar, rib 10 ends free at the costal arch.', 'de': 'Brustkorb: drei typisierte Rippenpaare (Rippen 2, 6, 10) als gekrümmte Stäbe von 30 mm, unter die Haut eingezogen; die Rippen 2 und 6 erreichen den Brustbeinstab, Rippe 10 endet frei am Rippenbogen.'},
    '500': {'en': 'Rib cage: left out at 75 mm; the trunk carries through the thoracic and lumbar bars, from which the shoulder blade bars and the pelvis bar hang.', 'de': 'Brustkorb: entfällt bei 75 mm; der Rumpf trägt über Brust- und Lendenstab, an denen die Schulterblattstäbe und der Beckenstab hängen.'},
}
STALE = ('A sternum bar joins the rib hoop',)
REWORD = [('deep barrel chest', 'deep oval chest'), ('tiefe Tonnenbrust', 'tiefe ovale Brust'), ('nine rib hoops', 'thirteen curved rib pairs with their costal cartilages'), ('neun Rippenreifen', 'dreizehn gekrümmte Rippenpaare mit ihren Rippenknorpeln'), ('five rib hoops', 'seven curved rib pairs'), ('fünf Rippenreifen', 'sieben gekrümmte Rippenpaare'), ('three rib hoops', 'three curved rib pairs'), ('drei Rippenreifen', 'drei gekrümmte Rippenpaare'), ('joins the rib hoops', 'joins the ribs'), ('verbindet die Rippenreifen', 'verbindet die Rippen')]

source = json.load(open(SOURCE, encoding='utf-8'))
for scale, volume in source['volumes'].items():
    kept = [d for d in volume['decisions'] if not d['en'].startswith(MARK) and not d['en'].startswith(STALE)]
    for decision in kept:
        for old, new in REWORD:
            decision['en'] = decision['en'].replace(old, new)
            decision['de'] = decision['de'].replace(old, new)
    volume['decisions'] = kept + [GIRDLE, TRUNK, RIBS[scale]]
json.dump(source, open(SOURCE, 'w', encoding='utf-8'), ensure_ascii=False, separators=(',', ':'))
