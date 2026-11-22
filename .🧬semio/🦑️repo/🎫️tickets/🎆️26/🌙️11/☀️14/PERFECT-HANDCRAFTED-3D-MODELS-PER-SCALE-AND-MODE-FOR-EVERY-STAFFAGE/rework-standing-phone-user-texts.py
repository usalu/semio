"""📝 Decisions of `humans/standing-phone-user` for the girdle bones, the coarse skeleton lists and the taken-in sleeves.

Usage: python3 rework-standing-phone-user-texts.py
"""
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes", "humans", "standing-phone-user.json")

GIRDLE = {
    "en": "Shoulder and pelvic girdle are bones of the rig: each clavicle runs from the sternoclavicular joint beside the manubrium out, back and up to the acromioclavicular joint and carries clavicle, scapula, scapular spine, acromion and glenoid; each hip bone runs from the sacroiliac joint down, forward and out to the hip joint and carries ilium, pubic and ischial ramus. Arms and thighs hang from them at the true breadths (shoulder joints 0.306 m, hip joints 0.170 m apart), so a shrug or a hip hike moves the limb with its girdle.",
    "de": "Schulter- und Beckengürtel sind Knochen des Rigs: Jedes Schlüsselbein läuft vom Sternoklavikulargelenk neben dem Manubrium nach außen, hinten und oben zum Akromioklavikulargelenk und trägt Schlüsselbein, Schulterblatt, Schulterblattgräte, Akromion und Gelenkpfanne; jedes Hüftbein läuft vom Iliosakralgelenk nach unten, vorn und außen zum Hüftgelenk und trägt Darmbein, Scham- und Sitzbeinast. Arme und Oberschenkel hängen in den wahren Breiten daran (Schultergelenke 0,306 m, Hüftgelenke 0,170 m auseinander), sodass Schulterheben oder Beckenkippen die Gliedmaße mit ihrem Gürtel bewegt.",
}
FINE = {
    "en": "The load path is closed where the bones meet in reality: the posterior ilium reaches the sacrum at the sacroiliac joint, the teeth of mandible and maxilla are in occlusion, and the pad of the little finger under the phone is a joint solid, because the phone rests on soft tissue and not on bone.",
    "de": "Der Lastpfad ist dort geschlossen, wo die Knochen wirklich aufeinandertreffen: Das hintere Darmbein erreicht das Kreuzbein im Iliosakralgelenk, die Zähne von Unter- und Oberkiefer stehen in Okklusion, und die Kuppe des kleinen Fingers unter dem Telefon ist ein Gelenkkörper, weil das Telefon auf Weichgewebe und nicht auf Knochen liegt.",
}
MIDDLE = {
    "en": "At 1:100 the posterior ilium joins hip bone and sacrum, the carpus joins radius and metacarpus, and the sock is a body of its own between jeans hem and shoe so that the foot stays attached to the shank in every pose.",
    "de": "Bei 1:100 verbindet das hintere Darmbein Hüftbein und Kreuzbein, die Handwurzel verbindet Speiche und Mittelhand, und die Socke ist ein eigener Körper zwischen Jeanssaum und Schuh, damit der Fuß in jeder Haltung am Unterschenkel bleibt.",
}
COARSE = {
    "en": "Structure at 1:200: skull, cervical and thoracolumbar spine with their curvature, three rib hoops each side, sternum, one bar per shoulder girdle on the clavicle bone from the manubrium to the acromion, sacrum and hip bones on the hip-bone bones. The long bones keep their true shaft and are declared slender: femur with head, neck and condyles, tibia and fibula, humerus with its head down to the elbow, ulna and radius, the radius reaching the carpus. Parts under 30 mm (trochanter, patella, malleolus, tubercle, olecranon, radial head) are left out; elbow and carpus are one ball each at the joint, 44 mm and 31 mm, where humerus, ulna and radius or radius and hand meet. Hand, foot and toes are one plate each; the phone is a block in the hands.",
    "de": "Tragwerk bei 1:200: Schädel, Hals- und Brust-Lenden-Wirbelsäule mit ihrer Krümmung, drei Rippenbögen je Seite, Brustbein, ein Stab je Schultergürtel auf dem Schlüsselbeinknochen vom Manubrium zum Akromion, Kreuzbein und Hüftbeine auf den Hüftbeinknochen. Die langen Knochen behalten ihren wahren Schaft und sind als schlank erklärt: Oberschenkelknochen mit Kopf, Hals und Kondylen, Schien- und Wadenbein, Oberarmknochen mit Kopf bis zum Ellbogen, Elle und Speiche, die Speiche bis zur Handwurzel. Teile unter 30 mm (Rollhügel, Kniescheibe, Knöchel, Höcker, Ellenhaken, Speichenkopf) entfallen; Ellbogen und Handwurzel sind je eine Kugel am Gelenk, 44 mm und 31 mm, wo Oberarmknochen, Elle und Speiche beziehungsweise Speiche und Hand zusammentreffen. Hand, Fuß und Zehen sind je eine Platte; das Telefon ist ein Block in den Händen.",
}
COARSEST = {
    "en": "Structure at 1:500: compact bones are masses of 75 mm, long bones keep their true shaft and are declared slender. Skull, cervical spine and spine as masses; two open rib hoops each side and the sternum up to the manubrium as slender bars; each clavicle a slender bar from the manubrium to the acromioclavicular joint with the scapula as one mass behind the rib cage on the same bone; sacrum and the two hip bones as masses from iliac crest to ischium. Per leg the proximal femur (head, neck, trochanter) as one mass at the hip joint, the femoral shaft, one mass for the knee, tibia and fibula, foot and toes; per arm humerus from the shoulder joint to the elbow, ulna and radius from elbow to carpus, and the hand bones along the grip; the phone as a block in the hands; masses of pelvis, trunk and thighs.",
    "de": "Tragwerk bei 1:500: Gedrungene Knochen sind Massen von 75 mm, lange Knochen behalten ihren wahren Schaft und sind als schlank erklärt. Schädel, Halswirbelsäule und Wirbelsäule als Massen; zwei offene Rippenbögen je Seite und das Brustbein bis zum Manubrium als schlanke Stäbe; jedes Schlüsselbein ein schlanker Stab vom Manubrium zum Akromioklavikulargelenk mit dem Schulterblatt als einer Masse hinter dem Brustkorb auf demselben Knochen; Kreuzbein und die beiden Hüftbeine als Massen vom Darmbeinkamm zum Sitzbein. Je Bein der proximale Oberschenkelknochen (Kopf, Hals, Rollhügel) als eine Masse am Hüftgelenk, der Schaft, eine Masse für das Knie, Schien- und Wadenbein, Fuß und Zehen; je Arm der Oberarmknochen vom Schultergelenk zum Ellbogen, Elle und Speiche vom Ellbogen zur Handwurzel und die Handknochen entlang des Griffs; das Telefon als Block in den Händen; Massen von Becken, Rumpf und Oberschenkeln.",
}
SLEEVE = {
    "en": "The upper sleeves follow a slim arm of 0.26 m girth under fleece: 0.108 m wide at the deltoid, 0.092 m at mid arm and 0.087 m at the elbow, the same taper at every scale and in the revolved sleeve profile of the drawings.",
    "de": "Die Oberärmel folgen einem schlanken Arm von 0,26 m Umfang unter Fleece: 0,108 m breit am Deltamuskel, 0,092 m in Armmitte und 0,087 m am Ellbogen, dieselbe Verjüngung in jedem Maßstab und im gedrehten Ärmelprofil der Zeichnungen.",
}


def main():
    source = json.load(open(SOURCE, encoding="utf8"))
    volumes = source["volumes"]
    volumes["50"]["decisions"] = volumes["50"]["decisions"][:3] + [GIRDLE, FINE, SLEEVE]
    volumes["100"]["decisions"] = volumes["100"]["decisions"][:3] + [GIRDLE, MIDDLE, SLEEVE]
    volumes["200"]["decisions"] = volumes["200"]["decisions"][:2] + [COARSE, GIRDLE, SLEEVE]
    volumes["500"]["decisions"] = volumes["500"]["decisions"][:2] + [COARSEST, GIRDLE, SLEEVE]
    json.dump(source, open(SOURCE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
    open(SOURCE, "a", encoding="utf8").write("\n")


if __name__ == "__main__":
    main()
