"""🐑 Sheep decisions: one entry per scale that states the girdles, the limb columns and the skeleton as modelled in the second round; written to the volumes source.

Usage: python3 rework-sheep-decisions.py
"""
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes", "animals", "sheep.json")
TEXTS = {
    "50": {
        "en": "Girdles and limbs at 1:50: each scapula runs from its dorsal border 50 mm beside the median plane down and out to the shoulder joint 95 mm beside it, each hip bone from the sacroiliac joint to the hip joint, and every limb hangs from there as a straight column with the digit split into pastern and claw. The ilium has a body from the sacrum to the acetabulum and a wing out to the tuber coxae, the pubis reaches both acetabula, and ribs 1 to 8 narrow towards the chest inlet so that the scapula lies outside them with the muscle layer between.",
        "de": "Gürtel und Gliedmaßen bei 1:50: Jedes Schulterblatt läuft vom Rückenrand 50 mm neben der Medianebene nach unten außen zum Schultergelenk 95 mm daneben, jedes Hüftbein vom Kreuzdarmbeingelenk zum Hüftgelenk, und jede Gliedmaße hängt dort als gerade Säule, die Zehe geteilt in Fessel und Klaue. Das Darmbein hat einen Körper vom Kreuzbein zur Hüftpfanne und einen Flügel hinaus zum Hüfthöcker, das Schambein erreicht beide Hüftpfannen, und die Rippen 1 bis 8 werden zum Brusteingang schmaler, sodass das Schulterblatt mit der Muskelschicht dazwischen außen aufliegt.",
    },
    "100": {
        "en": "Girdles at 1:100: the hip bone is one bar from the sacroiliac joint through the acetabulum to the ischial tuber, the scapula a blade from the dorsal border to the shoulder joint; the cranial ribs narrow as at 1:50 and every rib head reaches the spine.",
        "de": "Gürtel bei 1:100: Das Hüftbein ist ein Stab vom Kreuzdarmbeingelenk über die Hüftpfanne zum Sitzbeinhöcker, das Schulterblatt ein Blatt vom Rückenrand zum Schultergelenk; die vorderen Rippen werden wie bei 1:50 schmaler, und jeder Rippenkopf erreicht die Wirbelsäule.",
    },
    "200": {
        "en": "Feet and girdles at 1:200: the two claws of a foot are one horn block of the 30 mm feature that stands on the ground; the hip bone runs from the sacroiliac joint through the acetabulum to the ischial tuber and the scapula bar lies on the ribs, so the skeleton is one connected load path.",
        "de": "Füße und Gürtel bei 1:200: Die zwei Klauen eines Fußes sind ein Hornblock in der kleinsten Abmessung von 30 mm, der auf dem Boden steht; das Hüftbein läuft vom Kreuzdarmbeingelenk über die Hüftpfanne zum Sitzbeinhöcker, und der Schulterblattstab liegt auf den Rippen, sodass das Skelett ein zusammenhängender Lastpfad ist.",
    },
    "500": {
        "en": "Skeleton at 1:500: slender shafts at their true section for spine, sacrum, hip bone (sacroiliac joint, acetabulum, ischial tuber), scapula, long bones, mandible and tail; three rib pairs end on a sternum, the digits of a foot are one bar, and the horn block of the claws rides on the pastern.",
        "de": "Skelett bei 1:500: schlanke Schäfte im wahren Querschnitt für Wirbelsäule, Kreuzbein, Hüftbein (Kreuzdarmbeingelenk, Hüftpfanne, Sitzbeinhöcker), Schulterblatt, Röhrenknochen, Unterkiefer und Schwanz; drei Rippenpaare enden an einem Brustbein, die Zehen eines Fußes sind ein Stab, und der Hornblock der Klauen sitzt an der Fessel.",
    },
}


def main():
    names = {}
    exec(open(os.path.join(T, "volumes-insert.py"), encoding="utf-8").read().split("def member_end")[0], names)
    source = json.load(open(SOURCE, encoding="utf-8"))
    for scale, text in TEXTS.items():
        decisions = source["volumes"][scale]["decisions"]
        if text not in decisions:
            decisions.append(text)
    open(SOURCE, "w", encoding="utf-8").write('{"volumes":' + names["dump"](source["volumes"], 0) + "}\n")


main()
