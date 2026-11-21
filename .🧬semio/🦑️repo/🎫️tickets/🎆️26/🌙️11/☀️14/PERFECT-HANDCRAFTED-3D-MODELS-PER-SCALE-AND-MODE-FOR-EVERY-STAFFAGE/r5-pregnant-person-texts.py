"""📝 Writes the round-5 changes of `humans/pregnant-person` into the bilingual decisions of the source: the knit falling from the bust onto the abdomen (1:50 to 1:200), the 17 vertebrae (1:50), the Catmull-Rom limbs (1:100, 1:200) and the shoulder joints at the real biacromial breadth. Each sentence is appended once (idempotent).

Usage: python3 r5-pregnant-person-texts.py
@see ./r5-brief.md
"""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(HERE, "volumes", "humans", "pregnant-person.json")
ADD = {
    "50": [(0, " The knit falls from the bust apex in a straight tent onto the upper curve of the abdomen (belly-drape, sweater flanks followed just inside), so no ledge or crease runs under the bust; the shoulder joints sit 0.32 m apart, a biacromial breadth of 0.355 m.", " Der Strick fällt von der Brustspitze als gerades Zelt auf die obere Wölbung des Bauchs (belly-drape, knapp innerhalb der Pulloverflanken), sodass unter der Brust keine Stufe und keine Falte entsteht; die Schultergelenke liegen 0,32 m auseinander, eine Schulterbreite von 0,355 m."), (2, " Round 5: the thoracolumbar spine is its seventeen vertebral bodies L5 to T1 (lumbar ones 1.35 times as high) on the curve of the lordosis and kyphosis, each with its spinous process (lumbar short and level, thoracic long and steep).", " Runde 5: die Brust- und Lendenwirbelsäule besteht aus ihren siebzehn Wirbelkörpern L5 bis T1 (Lendenwirbel 1,35-mal so hoch) auf der Kurve von Lordose und Kyphose, jeder mit seinem Dornfortsatz (lumbal kurz und waagrecht, thorakal lang und steil).")],
    "100": [(0, " The knit tent from the bust onto the abdomen is kept in four slices; the limb and abdomen stations run through Catmull-Rom sections, so thigh, calf and ankle are continuous curves, not facets.", " Das Strickzelt von der Brust auf den Bauch bleibt mit vier Schnitten erhalten; Glieder- und Bauchstationen laufen durch Catmull-Rom-Schnitte, sodass Oberschenkel, Wade und Knöchel stetige Kurven sind, keine Facetten.")],
    "200": [(0, " The knit tent from the bust onto the abdomen in three slices; legs and arms through smoothed stations.", " Das Strickzelt von der Brust auf den Bauch mit drei Schnitten; Beine und Arme durch geglättete Stationen.")],
}
doc = json.load(open(SOURCE, encoding="utf-8"))
structure = doc["volumes"]["50"]["decisions"][2]
structure["en"] = structure["en"].replace(" and eight spinous processes", "")
structure["de"] = structure["de"].replace(" und acht Dornfortsätzen", "")
for scale, items in ADD.items():
    decisions = doc["volumes"][scale]["decisions"]
    for index, en, de in items:
        if en.strip() not in decisions[index]["en"]:
            decisions[index]["en"] += en
            decisions[index]["de"] += de
open(SOURCE, "w", encoding="utf-8").write(json.dumps(doc) + "\n")
print("decisions updated")
