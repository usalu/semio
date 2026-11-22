"""📝 Stage 4 of the seated reader: the decisions of every scale say what the rig and skeleton rework modelled and why.

Run: `python3 rework-seated-reader-4-texts.py` appends one decision per scale to the volumes source (once).
"""
import json

from rework_03_lib import source_path

A = "humans/seated-reader"
TEXTS = {
    "50": {
        "en": "Skeleton and trunk: the trunk bends in two places, so the sweater is a waist part on the lumbar bone and a chest part on the thorax that overlap at T12. The spine is five lumbar and twelve thoracic vertebral bodies with their discs and spinous processes along lordosis and kyphosis. Each clavicle runs from the manubrium to the acromion and carries scapula, scapular spine, acromion and glenoid; each hip bone runs from the sacroiliac joint to the acetabulum and carries ilium, pubis and ischium. Metacarpals reach their knuckles, phalanges swell at the joints, the thumbs lie on the notebook, so the load path is closed from the hands to the seat bones.",
        "de": "Skelett und Rumpf: Der Rumpf beugt sich an zwei Stellen, darum ist der Pullover ein Taillenteil am Lendenknochen und ein Brustteil am Brustkorb, die sich bei Th12 überlappen. Die Wirbelsäule besteht aus fünf Lenden- und zwölf Brustwirbelkörpern mit Bandscheiben und Dornfortsätzen entlang Lordose und Kyphose. Jedes Schlüsselbein läuft vom Manubrium zum Akromion und trägt Schulterblatt, Schulterblattgräte, Akromion und Gelenkpfanne; jedes Hüftbein läuft vom Iliosakralgelenk zur Hüftpfanne und trägt Darmbein, Schambein und Sitzbein. Die Mittelhandknochen reichen bis zu den Knöcheln, die Fingerglieder verdicken sich an den Gelenken, die Daumen liegen auf dem Notizbuch, sodass der Lastpfad von den Händen bis zu den Sitzbeinen geschlossen ist.",
    },
    "100": {
        "en": "Skeleton and trunk: sweater in a waist and a chest part as at 1:50; the spine is one lumbar and one thoracic rod that meet at T12. Clavicle, scapula, acromion and glenoid ride on the clavicle bone, ilium, acetabulum, pubis and ischium on the hip bone; one thumb bone per hand lies on the notebook and the seat bones bear on the seat. The shank runs into the shoe collar.",
        "de": "Skelett und Rumpf: Pullover in Taillen- und Brustteil wie bei 1:50; die Wirbelsäule ist ein Lenden- und ein Bruststab, die sich bei Th12 treffen. Schlüsselbein, Schulterblatt, Akromion und Gelenkpfanne sitzen auf dem Schlüsselbeinknochen, Darmbein, Hüftpfanne, Schambein und Sitzbein auf dem Hüftbein; je ein Daumenknochen liegt auf dem Notizbuch, und die Sitzbeine tragen auf der Sitzfläche. Der Unterschenkel läuft in den Schuhschaft.",
    },
    "200": {
        "en": "Skeleton and trunk: sweater in a waist and a chest part (a bent trunk would open by more than the 30 mm feature otherwise); lumbar and thoracic rod; one shoulder bar per clavicle; ilium and a hip bar that runs from the hip joint down to the seat. The shank runs into the shoe.",
        "de": "Skelett und Rumpf: Pullover in Taillen- und Brustteil (ein gebeugter Rumpf klaffte sonst um mehr als das 30-mm-Merkmal); Lenden- und Bruststab; ein Schulterstab je Schlüsselbein; Darmbein und ein Hüftstab, der vom Hüftgelenk bis auf die Sitzfläche läuft. Der Unterschenkel läuft in den Schuh.",
    },
    "500": {
        "en": "Skeleton and trunk: one sweater mass on the thorax (the waist bend stays below the 75 mm feature); lumbar and thoracic rod; one bar per clavicle from sternum to shoulder and one per hip bone from the sacroiliac joint over the hip joint down to the seat. The thigh is 154 mm thick above the knee so that the strip of it that shows beside the notebook in the plan still prints.",
        "de": "Skelett und Rumpf: eine Pullovermasse am Brustkorb (die Taillenbeuge bleibt unter dem 75-mm-Merkmal); Lenden- und Bruststab; je ein Stab pro Schlüsselbein vom Brustbein zur Schulter und pro Hüftbein vom Iliosakralgelenk über das Hüftgelenk bis auf die Sitzfläche. Der Oberschenkel ist über dem Knie 154 mm dick, damit der Streifen, der im Grundriss neben dem Notizbuch sichtbar bleibt, noch druckt.",
    },
}

source = json.load(open(source_path(A), encoding="utf8"))
for scale, text in TEXTS.items():
    decisions = source["volumes"][scale]["decisions"]
    if text not in decisions:
        decisions.append(text)
json.dump(source, open(source_path(A), "w", encoding="utf8"), ensure_ascii=False, indent=1)
open(source_path(A), "a", encoding="utf8").write("\n")
print("texts written")
