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


def main():
    path = sys.argv[1]
    document = json.load(open(path, encoding="utf-8"))
    for scale, (en, de) in TEXT.items():
        decisions = document["asset"]["volumes"][scale]["decisions"]
        decisions.append({"en": COMMON_EN + " " + en, "de": COMMON_DE + " " + de})
    with open(path, "w", encoding="utf-8") as handle:
        json.dump(document, handle, ensure_ascii=False, indent=2)
        handle.write("\n")


main()
