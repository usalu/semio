"""📝 Red squirrel: decisions texts of the structure lists and of the coarse scales after the girdle and rib-cage rebuild.

Usage: python3 rework-red-squirrel-decisions.py
"""
import json

SOURCE = "volumes/animals/red-squirrel.json"
TEXTS = {
    ("50", 2): {
        "en": "Structure at 1:50: the skeleton, not the body. The column is beaded vertebra by vertebra (two caudal and three sacral in the pelvis, seven lumbar of 6.6 mm, twelve thoracic of 3.9 mm, seven cervical with a wide atlas) and carries a crest of spinous processes; the sacrum widens into the alae that meet the ilia. Twelve ribs per side leave the column at the vertebrae, bow out to a half-width of 5.5 to 16.2 mm and slope back and down: seven reach the sternum (manubrium, sternebrae, spatulate xiphoid), three join the costal arch, two end free. Each hip bone is a bone of the rig from the sacroiliac joint to the hip joint and carries ilium, ischium and the pubic bow that closes the ring at the symphysis; each scapula is a bone of the rig from its dorsal border to the shoulder joint, with spine and acromion; the collarbone runs sideways from the manubrium to the acromion. Skull with its zygomatic width, mandible, 23 caudal vertebrae, long bones with 3.2 to 4.4 mm shafts and joint ends of 7.6 to 9.2 mm, hands and feet as plates with their phalanges; joints are spheres at the rig joints, two masses mark chest and head.",
        "de": "Tragwerk bei 1:50: das Skelett, nicht der Körper. Die Wirbelsäule ist Wirbel für Wirbel gegliedert (zwei Schwanz- und drei Kreuzwirbel im Becken, sieben Lendenwirbel von 6,6 mm, zwölf Brustwirbel von 3,9 mm, sieben Halswirbel mit breitem Atlas) und trägt einen Kamm aus Dornfortsätzen; das Kreuzbein verbreitert sich zu den Flügeln, an denen die Darmbeine anliegen. Zwölf Rippen je Seite gehen an den Wirbeln ab, wölben sich auf 5,5 bis 16,2 mm Halbbreite aus und fallen nach hinten unten: sieben erreichen das Brustbein (Handgriff, Brustbeinstücke, spatelförmiger Schwertfortsatz), drei schließen an den Rippenbogen an, zwei enden frei. Jedes Hüftbein ist ein Knochen des Rigs vom Kreuzdarmbeingelenk zum Hüftgelenk und trägt Darmbein, Sitzbein und den Schambeinbogen, der den Ring an der Symphyse schließt; jedes Schulterblatt ist ein Knochen des Rigs vom Rückenrand zum Schultergelenk, mit Gräte und Schulterhöhe; das Schlüsselbein läuft seitlich vom Handgriff zur Schulterhöhe. Schädel mit Jochbogenbreite, Unterkiefer, 23 Schwanzwirbel, Röhrenknochen mit 3,2 bis 4,4 mm Schaft und 7,6 bis 9,2 mm Gelenkenden, Hände und Füße als Platten mit ihren Fingergliedern; Gelenke sind Kugeln an den Rig-Gelenken, zwei Massen markieren Brust und Kopf.",
    },
    ("100", 2): {
        "en": "Structure at 1:100: the skeleton reduced to what a 15 mm feature can show. Skull and sacrum are masses of their real size; the column, the six tail links and every long bone, hand and foot are slender bars; five ribs per side (3, 5, 7, 9, 11) and the sternum are bars of 1.4 to 1.9 mm, each scapula one bar from its dorsal border to the shoulder joint, each hip bone one bar from the iliac crest through the hip joint to the ischium, each collarbone a rod of 0.8 mm. No phalanges, no single caudal vertebrae and no joint solids: the joints are where the bars meet. Two masses mark chest and head.",
        "de": "Tragwerk bei 1:100: das Skelett, auf das reduziert, was ein Merkmal von 15 mm zeigen kann. Schädel und Kreuzbein sind Massen in wahrer Größe; die Wirbelsäule, die sechs Schwanzglieder und jeder Röhrenknochen, Hand und Fuß sind schlanke Stäbe; fünf Rippen je Seite (3, 5, 7, 9, 11) und das Brustbein sind Stäbe von 1,4 bis 1,9 mm, jedes Schulterblatt ein Stab vom Rückenrand zum Schultergelenk, jedes Hüftbein ein Stab vom Darmbeinkamm über das Hüftgelenk zum Sitzbein, jedes Schlüsselbein ein Stab von 0,8 mm. Keine Fingerglieder, keine einzelnen Schwanzwirbel und keine Gelenkkörper: Die Gelenke liegen dort, wo die Stäbe sich treffen. Zwei Massen markieren Brust und Kopf.",
    },
    ("100", 3): {
        "en": "Corneas, nose pad and nostril recess stay as small solids because they carry the thermal regions of eyes and nose. Fingers, claws, pupils and lids of 1:50 are not carried: the hand is a mitten that holds the fingers, the sole runs to the toe tips.",
        "de": "Hornhäute, Nasenspiegel und Nasenloch bleiben als kleine Körper, weil sie die thermischen Regionen Augen und Nase tragen. Finger, Krallen, Pupillen und Lider von 1:50 werden nicht übernommen: Die Hand ist ein Fäustling, der die Finger enthält, die Sohle reicht bis zu den Zehenspitzen.",
    },
    ("200", 1): {
        "en": "Thermal at 1:200: one surface per region; the sole carries the region of the bare foot, the mitten that of the hand; nose, breath outlet and the plume saddle on the loin are own solids. Structure at 1:200: skull and sacrum as masses; the column, six tail links, three ribs per side (5, 8, 11), sternum, scapulae, hip bones, collarbones and every long bone as slender bars; two masses.",
        "de": "Thermik bei 1:200: eine Oberfläche je Region; die Sohle trägt die Region des nackten Fußes, der Fäustling die der Hand; Nase, Atemauslass und der Fahnensattel auf der Lende sind eigene Körper. Tragwerk bei 1:200: Schädel und Kreuzbein als Massen; Wirbelsäule, sechs Schwanzglieder, drei Rippen je Seite (5, 8, 11), Brustbein, Schulterblätter, Hüftbeine, Schlüsselbeine und jeder Röhrenknochen als schlanke Stäbe; zwei Massen.",
    },
    ("200", 2): {
        "en": "Corneas and nose pad stay as small solids because they carry the thermal regions of eyes and nose. Fingers, claws, pupils, lids, muzzle pads and the nostril recess of 1:50 are not carried: the hand is a mitten, the sole runs to the toe tips.",
        "de": "Hornhäute und Nasenspiegel bleiben als kleine Körper, weil sie die thermischen Regionen Augen und Nase tragen. Finger, Krallen, Pupillen, Lider, Schnauzenpolster und das Nasenloch von 1:50 werden nicht übernommen: Die Hand ist ein Fäustling, die Sohle reicht bis zu den Zehenspitzen.",
    },
    ("500", 1): {
        "en": "Thermal at 1:500: one surface per region; the mitten carries the region of the hand, the sole that of the foot. Structure at 1:500: skull and sacrum as masses; the dorsal column, six tail links, two ribs per side (5, 9), sternum, scapulae, hip bones, collarbones and the long bones as slender bars; two masses.",
        "de": "Thermik bei 1:500: eine Oberfläche je Region; der Fäustling trägt die Region der Hand, die Sohle die des Fußes. Tragwerk bei 1:500: Schädel und Kreuzbein als Massen; die dorsale Wirbelsäule, sechs Schwanzglieder, zwei Rippen je Seite (5, 9), Brustbein, Schulterblätter, Hüftbeine, Schlüsselbeine und die Röhrenknochen als schlanke Stäbe; zwei Massen.",
    },
}
TEXTS[("500", 2)] = TEXTS[("200", 2)]


def main():
    text = open(SOURCE, encoding="utf-8").read()
    document = json.loads(text)
    for (scale, index), value in TEXTS.items():
        document["volumes"][scale]["decisions"][index] = value
    scope = {"json": json}
    exec("def scalar(value)" + open("rework-red-squirrel-girdles.py", encoding="utf-8").read().split("def scalar(value)")[1].split("def member_end")[0], scope)
    open(SOURCE, "w", encoding="utf-8").write('{"volumes": ' + scope["dump"](document["volumes"], 0) + "}\n")
    print("decisions written")


main()
