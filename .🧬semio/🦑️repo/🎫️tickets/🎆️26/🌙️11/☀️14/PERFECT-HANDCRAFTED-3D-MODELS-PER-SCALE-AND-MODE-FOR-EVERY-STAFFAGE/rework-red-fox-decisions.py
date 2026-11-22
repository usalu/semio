"""📝 Red fox: decisions texts for the girdles on lateral rest placement, the two-part shoulder and the closed load path.

Usage: python3 rework-red-fox-decisions.py   (edits the volumes source in place, idempotent)
"""
import json
import os
import re

SOURCE = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'volumes/animals/red-fox.json')

BODY = {
    'en': ' The shoulder is two overlapping rounded masses per side, the scapular muscles on the scapula bone (it lies on the rib cage, 12° out of the sagittal plane) and the upper arm on the humerus, so the shoulder stays closed when the scapula swings.',
    'de': ' Die Schulter besteht je Seite aus zwei überlappenden gerundeten Massen, der Schulterblattmuskulatur am Schulterblattknochen (er liegt um 12° aus der Sagittalebene geneigt auf dem Brustkorb) und dem Oberarm am Humerus; so bleibt die Schulter geschlossen, wenn das Schulterblatt schwingt.',
}
STRUCTURE = {
    '50': {
        'en': ' Girdles on their true lateral placement: each scapula is a blade with spine and acromion on its own bone from the dorsal border 24 mm beside the median plane to the shoulder joint 44 mm beside it, hung from the third rib by the serratus sling (a soft-tissue tie, the only load path between trunk and fore limb, as a canid has no collarbone); each hip bone (ilium wing, acetabulum, ischium with tuber, pubis and ischial arch around the obturator foramen) sits on its own bone from the sacroiliac joint 18 mm to the hip joint 36 mm beside the plane, both halves meeting in the pelvic symphysis. Rib heads reach the vertebral bodies, the rib cage is 98 mm broad, the thoracic spinous processes rise to 36 mm at the withers and lean back to the anticlinal eleventh, and two caudal vertebrae in the croup join sacrum and tail.',
        'de': ' Gürtel in ihrer wahren seitlichen Lage: jedes Schulterblatt ist ein Blatt mit Gräte und Acromion an seinem eigenen Knochen, vom Rückenrand 24 mm neben der Medianebene bis zum Schultergelenk 44 mm daneben, an der dritten Rippe aufgehängt durch die Serratus-Schlinge (ein Weichteilzugband, der einzige Lastpfad zwischen Rumpf und Vordergliedmaße, da Hundeartige kein Schlüsselbein haben); jedes Hüftbein (Darmbeinschaufel, Hüftpfanne, Sitzbein mit Höcker, Schambein und Sitzbeinbogen um das Hüftloch) sitzt an seinem eigenen Knochen vom Kreuzdarmbeingelenk 18 mm bis zum Hüftgelenk 36 mm neben der Ebene, beide Hälften treffen sich in der Beckenfuge. Die Rippenköpfe reichen an die Wirbelkörper, der Brustkorb ist 98 mm breit, die Dornfortsätze der Brustwirbel steigen am Widerrist auf 36 mm und neigen sich bis zum antiklinalen elften nach hinten, und zwei Schwanzwirbel in der Kruppe verbinden Kreuzbein und Rute.',
    },
    '100': {
        'en': ' Girdles on their true lateral placement: the scapula blade lies on its own bone 12° out of the sagittal plane and hangs from the third rib by the serratus sling; the pelvis is the sacrum and one hip bone per side from the iliac crest to the ischial tuber, each on its own bone from the sacroiliac joint to the hip joint; rib heads reach the spine, the rib cage is 98 mm broad and a caudal root joins sacrum and tail.',
        'de': ' Gürtel in ihrer wahren seitlichen Lage: das Schulterblatt liegt an seinem eigenen Knochen um 12° aus der Sagittalebene geneigt und hängt über die Serratus-Schlinge an der dritten Rippe; das Becken besteht aus dem Kreuzbein und je Seite einem Hüftbein vom Darmbeinkamm bis zum Sitzbeinhöcker, jedes an seinem eigenen Knochen vom Kreuzdarmbeingelenk bis zum Hüftgelenk; die Rippenköpfe reichen an die Wirbelsäule, der Brustkorb ist 98 mm breit, und eine Schwanzwurzel verbindet Kreuzbein und Rute.',
    },
    '200': {
        'en': ' The pelvis is one block from the iliac crests under the croup to the hip joints that also stands for both hip bones (their rig bones run from the sacroiliac joint to the hip joint inside it); the scapulae lie on their own bones 12° out of the sagittal plane; rib heads reach the spine and the rib cage is 98 mm broad.',
        'de': ' Das Becken ist ein Block von den Darmbeinkämmen unter der Kruppe bis zu den Hüftgelenken, der auch für beide Hüftbeine steht (deren Rig-Knochen laufen darin vom Kreuzdarmbeingelenk zum Hüftgelenk); die Schulterblätter liegen an ihren eigenen Knochen um 12° aus der Sagittalebene geneigt; die Rippenköpfe reichen an die Wirbelsäule, der Brustkorb ist 98 mm breit.',
    },
}
STRUCTURE['500'] = STRUCTURE['200']
OLD = [
    ('en', 'Shoulder blade and upper arm are one rounded mass per side that rises a few millimetres out of the chest, the thigh', 'The thigh'),
    ('en', 'shoulder blade and upper arm as one rounded shoulder mass per side; ', ''),
    ('de', 'Schulterblatt und Oberarm sind je Seite eine gerundete Masse, die wenige Millimeter aus der Brust tritt, die Keule', 'Die Keule'),
    ('de', 'Schulterblatt und Oberarm als eine gerundete Schultermasse je Seite; ', ''),
]


def main():
    source = json.load(open(SOURCE, encoding='utf-8'))
    for scale, volume in source['volumes'].items():
        decisions = volume['decisions']
        structure = next((d for d in decisions if re.search('skeleton|structure', d['en'][:160], re.I)), decisions[-1])
        for language in ('en', 'de'):
            if scale != '500':
                for where, old, new in OLD:
                    if where == language:
                        decisions[0][language] = decisions[0][language].replace(old, new)
                if BODY[language] not in decisions[0][language]:
                    decisions[0][language] += BODY[language]
            if STRUCTURE[scale][language] not in structure[language]:
                structure[language] += STRUCTURE[scale][language]
        print(scale, 'structure decision starts:', structure['en'][:70], '| de shoulder:', re.findall('Schulterblatt und Oberarm[^.;]*', decisions[0]['de']))
    json.dump(source, open(SOURCE, 'w', encoding='utf-8'), ensure_ascii=False, separators=(',', ':'))


main()
