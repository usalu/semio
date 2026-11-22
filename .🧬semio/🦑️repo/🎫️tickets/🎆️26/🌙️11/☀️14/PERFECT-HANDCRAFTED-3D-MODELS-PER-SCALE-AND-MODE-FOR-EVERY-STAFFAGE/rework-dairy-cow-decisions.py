"""📝 Dairy cow decisions: states per scale what the limb, girdle and skeleton rework modelled and why.

Usage: python3 rework-dairy-cow-decisions.py <asset document> (in place)
"""
import json
import sys

COMMON_EN = "The limbs follow the standing angles of cattle: hip joint 1.17 m above the ground, stifle 0.79 m (123 degrees), hock 0.47 m (140 degrees), shoulder joint 1.00 m, elbow 0.79 m under the withers (129 degrees), carpus 0.40 m, fetlocks 0.12 to 0.14 m; each foot has a pastern bone between cannon and cloven hoof, the hind limbs hang from the hip bones 0.30 m apart and the fore limbs from the shoulder joints 0.40 m apart."
COMMON_DE = "Die Gliedmaßen folgen den Standwinkeln des Rindes: Hüftgelenk 1,17 m über dem Boden, Knie 0,79 m (123 Grad), Sprunggelenk 0,47 m (140 Grad), Schultergelenk 1,00 m, Ellbogen 0,79 m unter dem Widerrist (129 Grad), Vorderfußwurzel 0,40 m, Fesselgelenke 0,12 bis 0,14 m; jeder Fuß hat ein Fesselbein zwischen Röhrbein und Klaue, die Hinterbeine hängen an den 0,30 m auseinander liegenden Hüftbeinen und die Vorderbeine an den 0,40 m auseinander liegenden Schultergelenken."
TEXT = {
    "50": (
        "Limbs at 1:50: the shoulder is a flat mound over the blade that runs into the triceps mass of the upper arm, the forearm is 0.18 m deep below the elbow and 0.08 m above the broad carpus, the cannon 0.062 by 0.066 m with fetlock, pastern, two dewclaws and two claws; the thigh is the flat plate of a dairy cow, the gaskin carries the Achilles tendon to the point of the hock 0.10 m above the joint, and the hook bones stand out of the loin. In the skeleton the shoulder blades reach the withers, the atlas meets the occipital condyles and the hip bones are members of their own bones.",
        "Gliedmaßen bei 1:50: Die Schulter ist ein flacher Hügel über dem Schulterblatt, der in die Trizepsmasse des Oberarms übergeht, der Unterarm ist unter dem Ellbogen 0,18 m und über der breiten Vorderfußwurzel 0,08 m tief, das Röhrbein misst 0,062 mal 0,066 m mit Fesselgelenk, Fessel, zwei Afterklauen und zwei Klauen; die Keule ist die flache Platte der Milchkuh, der Unterschenkel führt die Achillessehne zum Fersenhöcker 0,10 m über dem Gelenk, und die Hüfthöcker treten aus der Lende. Im Skelett reichen die Schulterblätter bis zum Widerrist, der Atlas trifft die Hinterhauptskondylen, und die Hüftbeine sind Stäbe ihrer eigenen Knochen.",
    ),
    "100": (
        "Limbs at 1:100: the same masses with five to seven stations each; elbow point, stifle and hook bones are below 15 mm of relief and are left to the shoulder, thigh and loin, the point of the hock is the upper end of the hind cannon, and the dewclaws are omitted.",
        "Gliedmaßen bei 1:100: dieselben Massen mit je fünf bis sieben Stationen; Ellbogenhöcker, Knie und Hüfthöcker liegen unter 15 mm Relief und gehen in Schulter, Keule und Lende auf, der Fersenhöcker ist das obere Ende des hinteren Röhrbeins, die Afterklauen entfallen.",
    ),
    "200": (
        "Limbs at 1:200: four to five stations per segment, enough for the taper of forearm and gaskin, the knee, the hock with its point and the fetlock, all at least 60 mm thick; smaller relief is not modelled.",
        "Gliedmaßen bei 1:200: vier bis fünf Stationen je Abschnitt, genug für die Verjüngung von Unterarm und Unterschenkel, Vorderfußwurzel, Sprunggelenk mit Fersenhöcker und Fesselgelenk, alles mindestens 60 mm stark; kleineres Relief wird nicht modelliert.",
    ),
    "500": (
        "Limbs at 1:500: upper arm, forearm, cannon, pastern and one cloven hoof per fore foot, thigh, gaskin, cannon with the hock, pastern and hoof per hind foot, every part at least 75 mm thick, so cannons and hooves are drawn slightly heavier than life; the chest is widened to carry the upper arms. The 1:500 skeleton has one member per bone, the pelvis as ilium wings, hip bones, ischia and pubis, and spine and tail at the printable 75 mm.",
        "Gliedmaßen bei 1:500: Oberarm, Unterarm, Röhrbein, Fessel und eine Klaue je Vorderfuß, Keule, Unterschenkel, Röhrbein mit Sprunggelenk, Fessel und Klaue je Hinterfuß, jedes Teil mindestens 75 mm stark, Röhrbeine und Klauen daher etwas kräftiger als in Natur; die Brust ist verbreitert und trägt die Oberarme. Das Skelett bei 1:500 hat einen Stab je Knochen, das Becken aus Darmbeinflügeln, Hüftbeinen, Sitzbeinen und Schambein, Wirbelsäule und Schwanz in der druckbaren Stärke von 75 mm.",
    ),
}

TRUNK = {
    "50": (
        "Neck, top line and skull at 1:50: the neck is 0.58 m deep and 0.30 m broad at the shoulder and 0.28 by 0.17 m at the throat, its top line dips 0.08 m between withers and poll and the dewlap hangs from its lower line; the withers rise 0.03 m over the back without a hump, the fore chest ends 0.10 m ahead of the point of the shoulder so that only the brisket shows between the fore legs, the muzzle is 0.17 m broad and the cheek 0.19 m deep. The scapula bone rests inclined 14 degrees outwards from 0.09 m beside the median plane to the shoulder joint at 0.20 m. In the skeleton the skull has zygomatic arches and cheek teeth, each half of the mandible a ramus with condyle and coronoid process and the incisors at the symphysis, the sternum is a flat bar through the ventral ends of the eight sternal ribs and each shoulder blade ends in its glenoid.",
        "Hals, Rückenlinie und Schädel bei 1:50: Der Hals ist an der Schulter 0,58 m tief und 0,30 m breit und an der Kehle 0,28 mal 0,17 m, seine Oberlinie senkt sich zwischen Widerrist und Genick um 0,08 m, die Wamme hängt an seiner Unterlinie; der Widerrist steht 0,03 m über dem Rücken ohne Buckel, die Vorderbrust endet 0,10 m vor dem Buggelenk, sodass zwischen den Vorderbeinen nur die Brustspitze sichtbar ist, das Flotzmaul ist 0,17 m breit und die Backe 0,19 m tief. Der Schulterblattknochen ruht 14 Grad nach außen geneigt von 0,09 m neben der Medianebene bis zum Schultergelenk bei 0,20 m. Im Skelett hat der Schädel Jochbögen und Backenzähne, jede Unterkieferhälfte einen Ast mit Gelenk- und Kronfortsatz und die Schneidezähne an der Symphyse, das Brustbein ist ein flacher Stab durch die unteren Enden der acht wahren Rippen, und jedes Schulterblatt endet in seiner Gelenkpfanne.",
    ),
    "100": (
        "Neck, top line and skull at 1:100: the same neck with five stations, the withers with four, the fore chest shortened as at 1:50, muzzle and cheek at their true breadth; the skull is one loft of six stations, the mandible one member and the sternum a flat bar through the ventral ends of ribs 1, 3, 5 and 7.",
        "Hals, Rückenlinie und Schädel bei 1:100: derselbe Hals mit fünf Stationen, der Widerrist mit vier, die Vorderbrust gekürzt wie bei 1:50, Flotzmaul und Backe in wahrer Breite; der Schädel ist ein Körper aus sechs Stationen, der Unterkiefer ein Stab und das Brustbein ein flacher Stab durch die unteren Enden der Rippen 1, 3, 5 und 7.",
    ),
    "200": (
        "Neck, top line and skull at 1:200: the neck has four stations with the dip of its top line, the trunk keeps the withers 0.03 m over the back, the muzzle is 0.17 m broad; the sternum is a flat bar at least 36 mm thick under the typified ribs.",
        "Hals, Rückenlinie und Schädel bei 1:200: Der Hals hat vier Stationen mit der Senke seiner Oberlinie, der Rumpf hält den Widerrist 0,03 m über dem Rücken, das Flotzmaul ist 0,17 m breit; das Brustbein ist ein mindestens 36 mm starker flacher Stab unter den typisierten Rippen.",
    ),
    "500": (
        "Neck and head at 1:500: the neck is a loft of three stations from the shoulder (0.58 by 0.30 m) to the throat (0.29 by 0.18 m), the head one wedge with the muzzle 0.17 m broad; the coat patches are below the printable sheet thickness and exist only as thermal regions.",
        "Hals und Kopf bei 1:500: Der Hals ist ein Körper aus drei Stationen von der Schulter (0,58 mal 0,30 m) bis zur Kehle (0,29 mal 0,18 m), der Kopf ein Keil mit 0,17 m breitem Flotzmaul; die Fellflecken liegen unter der druckbaren Blechstärke und bestehen nur als thermische Regionen.",
    ),
}


def main():
    path = sys.argv[1]
    document = json.load(open(path, encoding="utf-8"))
    for scale, (en, de) in TEXT.items():
        decisions = document["asset"]["volumes"][scale]["decisions"]
        decisions.append({"en": COMMON_EN + " " + en, "de": COMMON_DE + " " + de})
        decisions.append({"en": TRUNK[scale][0], "de": TRUNK[scale][1]})
    with open(path, "w", encoding="utf-8") as handle:
        json.dump(document, handle, ensure_ascii=False, indent=2)
        handle.write("\n")


main()
