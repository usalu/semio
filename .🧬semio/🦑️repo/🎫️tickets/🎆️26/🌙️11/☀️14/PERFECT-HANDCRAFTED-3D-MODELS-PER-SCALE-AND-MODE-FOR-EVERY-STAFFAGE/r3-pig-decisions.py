"""📝 Brings the drawing decisions of animals/pig in line with the round-3 redraw: paper sizes of the new outlines, the dewclaws in the hoof outline, the eyes hidden under the lop ears in plan, the 1:500 leg width. Applied to 🗑️generated/r3-pig/representations.json (before r3-pig-apply.py) so that every run of r3-pig-run.sh carries it.

Usage: python3 r3-pig-decisions.py
"""
import json

P = "🗑️generated/r3-pig/representations.json"
data = json.load(open(P, encoding="utf-8"))
r = data["representations"]
SIZE = {"50": (("21 × 15 mm", "25 × 14 mm"),), "100": (("10.6 × 7.6 mm", "12.7 × 6.8 mm"), ("10,6 × 7,6 mm", "12,7 × 6,8 mm")), "200": (("5.3 × 3.8 mm", "6.4 × 3.4 mm"), ("5,3 × 3,8 mm", "6,4 × 3,4 mm")), "500": (("2.1 × 1.5 mm", "2.5 × 1.4 mm"), ("2,1 × 1,5 mm", "2,5 × 1,4 mm"))}
for scale, pairs in SIZE.items():
    s = r[scale]["decisions"]["silhouette"]
    for lang in ("en", "de"):
        for old, new in pairs:
            s[lang] = s[lang].replace(old, new)
r["50"]["decisions"]["omitted"] = {"en": "Nostril holes and eyelashes are not drawn; the dewclaws are part of the hoof outline; in plan the eyes lie hidden under the lop ears and are not drawn; the head is 6 mm long.", "de": "Nüstern und Wimpern sind nicht gezeichnet; die Afterklauen gehören zum Umriss der Klaue; im Grundriss liegen die Augen verdeckt unter den Schlappohren und sind nicht gezeichnet; der Kopf ist 6 mm lang."}
r["500"]["decisions"]["thinElements"] = {"en": "Legs (real 0.05 to 0.09 m = 0.1 to 0.18 mm) are drawn 0.092 m wide (0.18 mm), so that every leg, the far ones half hidden behind the near ones included, keeps the 0.2 mm printable feature where it shows; the 0.13 mm outline adds the rest.", "de": "Beine (real 0,05 bis 0,09 m = 0,1 bis 0,18 mm) sind 0,092 m breit gezeichnet (0,18 mm), damit jedes Bein, auch die halb hinter den nahen verdeckten fernen, dort, wo es sichtbar ist, die druckbare Mindestgröße von 0,2 mm hält; die 0,13-mm-Kontur ergänzt den Rest."}
json.dump(data, open(P, "w", encoding="utf-8"), ensure_ascii=False)
print("decisions updated")
