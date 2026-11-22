"""📝 Decisions of the domestic cat volumes: what the trunk noses, the closed ribcage and the pelvis of this pass model and why."""
import json

SOURCE = "volumes/animals/domestic-cat.json"
BODY = {
    "en": " Every trunk loft ends at its spinal joint in a blunt half ellipsoid about 60 mm long that lies inside its neighbour at rest, so the sitting, bowing and stretching spine bends without a flat cap or a step.",
    "de": " Jeder Rumpfloft endet an seinem Wirbelgelenk in einem stumpfen Halbellipsoid von etwa 60 mm Länge, das in Ruhe im Nachbarn liegt, sodass die sitzende, sich verbeugende und sich streckende Wirbelsäule ohne flache Kappe und ohne Stufe biegt.",
}
STRUCTURE = {
    "50": {
        "en": " The rib heads sit on the vertebrae; ribs one to eight reach the sternum through their costal cartilages, ribs nine to thirteen join the costal arch. The alae of the sacrum are 38 mm wide and carry the ilia in the sacroiliac joints, the pubis runs from acetabulum to acetabulum through the symphysis, and the ear cartilages are rooted on the skull, so the load path from skull to paws is closed.",
        "de": " Die Rippenköpfe sitzen an den Wirbeln; die Rippen eins bis acht erreichen das Brustbein über ihre Rippenknorpel, die Rippen neun bis dreizehn schließen an den Rippenbogen an. Die Kreuzbeinflügel sind 38 mm breit und tragen die Darmbeine in den Kreuzdarmbeingelenken, das Schambein läuft von Hüftpfanne zu Hüftpfanne durch die Symphyse, und die Ohrknorpel wurzeln am Schädel, sodass der Lastpfad vom Schädel bis zu den Pfoten geschlossen ist.",
    },
    "100": {
        "en": " The pelvis is sacrum, two hip bones of ilium and ischium each and the pubic bar at 15 mm, meeting in the sacroiliac joints, the acetabula and the symphysis; the sacrum carries the first caudal rod. Each of the four rib hoops per side runs from its vertebra to the sternum, the last one into the hoop before it as the costal arch.",
        "de": " Das Becken besteht aus Kreuzbein, zwei Hüftbeinen aus je Darmbein und Sitzbein und der Schambeinspange in 15 mm, verbunden in den Kreuzdarmbeingelenken, den Hüftpfannen und der Symphyse; das Kreuzbein trägt den ersten Schwanzwirbelstab. Jeder der vier Rippenbögen je Seite läuft von seinem Wirbel zum Brustbein, der letzte als Rippenbogen in den Bogen davor.",
    },
    "200": {
        "en": " The three rib hoops per side close the ribcage: the cranial and the middle one run from the spine to the sternum, the caudal one joins the middle one as the costal arch.",
        "de": " Die drei Rippenbögen je Seite schließen den Brustkorb: der vordere und der mittlere laufen von der Wirbelsäule zum Brustbein, der hintere schließt als Rippenbogen an den mittleren an.",
    },
    "500": {
        "en": " Both rib hoops per side run from the spine to the sternum, so the ribcage is closed.",
        "de": " Beide Rippenbögen je Seite laufen von der Wirbelsäule zum Brustbein, sodass der Brustkorb geschlossen ist.",
    },
}
document = json.load(open(SOURCE))
for scale, volume in document["volumes"].items():
    body, structure = volume["decisions"][0], volume["decisions"][2]
    for language in ("en", "de"):
        if BODY[language].strip() not in body[language]:
            body[language] += BODY[language]
        if STRUCTURE[scale][language].strip() not in structure[language]:
            structure[language] += STRUCTURE[scale][language]
medium = document["volumes"]["100"]["decisions"][2]
medium["en"] = medium["en"].replace("the pelvis with sacrum as one plate, seven rib hoops per side for thirteen ribs", "the pelvis, four rib hoops per side for thirteen ribs")
medium["de"] = medium["de"].replace("das Becken mit Kreuzbein als eine Platte, sieben Rippenbögen je Seite für dreizehn Rippen", "das Becken, vier Rippenbögen je Seite für dreizehn Rippen")
json.dump(document, open(SOURCE, "w"), ensure_ascii=False, indent=1)
