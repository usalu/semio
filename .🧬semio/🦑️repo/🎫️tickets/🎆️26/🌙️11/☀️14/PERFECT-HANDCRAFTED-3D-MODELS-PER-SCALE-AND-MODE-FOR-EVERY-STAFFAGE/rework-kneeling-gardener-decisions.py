"""📝 kneeling-gardener: the decisions of every scale say what the lumbar joint and the neckline changed. Usage: python3 rework-kneeling-gardener-decisions.py (idempotent; publish with volumes-insert.py)."""
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes", "humans", "kneeling-gardener.json")
BODY = {
    "en": " The back bends in two places: the shirt is a waist part on the lumbar spine and a chest part on the rib cage that overlap at the thoracolumbar joint, so the back rounds when the gardener leans forward and stays closed. The neckline slopes from the nape down to above the jugular notch and leaves the front of the neck free.",
    "de": " Der Rücken beugt sich an zwei Stellen: Das Hemd besteht aus einem Taillenteil an der Lendenwirbelsäule und einem Brustteil am Brustkorb, die sich am Übergang von Brust- zu Lendenwirbelsäule überlappen, sodass der Rücken beim Vorbeugen rund wird und geschlossen bleibt. Der Halsausschnitt fällt vom Nacken bis über die Drosselgrube ab und lässt den vorderen Hals frei.",
}
STRUCTURE = {
    "en": " The spine is two members, lumbar and thoracic, that meet in a joint at T12/L1; the trunk mass is carried as abdomen on the lumbar member and as chest on the rib cage.",
    "de": " Die Wirbelsäule besteht aus zwei Stäben, Lenden- und Brustwirbelsäule, die sich in einem Gelenk bei Th12/L1 treffen; die Rumpfmasse liegt als Bauch am Lendenstab und als Brust am Brustkorb.",
}
source = json.load(open(SOURCE, encoding="utf-8"))
for scale in source["volumes"].values():
    for index, text in ((0, BODY), (2, STRUCTURE)):
        for language in ("en", "de"):
            if text[language].strip() not in scale["decisions"][index][language]:
                scale["decisions"][index][language] += text[language]
json.dump(source, open(SOURCE, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
open(SOURCE, "a", encoding="utf-8").write("\n")
