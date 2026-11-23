"""🔌 machinery/diesel-generator: the intake louvres of 1:200 and 1:500 are the louvres their drawings show.

The rebuilt source of 2026-11-22 01:34 had typified each louvre to eight blades of 60 x 30 mm at 200 mm pitch (1:500: five of 120 x 75 mm at 300 mm from 0.7 m, no frame). The side elevation of 1:200 draws fifteen blade lines at 100 mm pitch and 1:100 has fifteen slats; the elevation of 1:500 draws a 75 mm frame and four 75 mm blade lines at 0.9, 1.2, 1.5 and 1.8 m. Measured: `volumes-silhouette` 1:200 p95 187.5 mm, 1:500 IoU 0.891; `volumes-cross-scale` 1:200 against 1:100 p95 187.5 mm.
1:200: fifteen blades per opening at 100 mm pitch, the real 35 x 2 mm sheet at 30 degrees, declared slender as at 1:50 and 1:100 (a blade is 1.3 m long): a long thin part keeps its section, the row keeps its count.
1:200 also gets the two side doors its elevation draws (door lines 0.75 and 0.5 m by 1.7 m): 30 mm leaves on the wall skin, flush with the corner posts, with the thermal twins of 1:100. The bottom corner castings and the bottom end rails stay inside the full-length base block, which is how the 1:200 drawing draws them.
1:500: a frame of 75 mm bars around each opening and four blades of 75 x 75 mm at 300 mm pitch on the drawn lines.
One-shot on `volumes/machinery/diesel-generator.json`; publish with `volumes-insert.py`.
"""
import copy
import json
import os

SOURCE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "volumes", "machinery", "diesel-generator.json")
DECISIONS = {
    "200": {"en": "Each intake louvre keeps its fifteen blades at 100 mm pitch, as drawn and as at 1:100: 35 x 2 mm sheet blades of 1.3 m at 30 degrees, slender parts at their true section. The two side doors are 30 mm leaves on the wall skin; bottom castings and end rails stay inside the full-length base block.", "de": "Jede Ansaugjalousie behält ihre fünfzehn Lamellen im Abstand von 100 mm, wie gezeichnet und wie bei 1:100: Blechlamellen 35 x 2 mm von 1,3 m unter 30 Grad, schlanke Teile mit wahrem Querschnitt. Die zwei Seitentüren sind 30-mm-Flügel auf der Wandhaut; untere Eckbeschläge und Endschwellen bleiben im durchgehenden Bodenblock."},
    "500": {"en": "Each intake louvre is a frame of 75 mm bars with four blades of 75 x 75 mm at 300 mm pitch, the louvre the elevation draws.", "de": "Jede Ansaugjalousie ist ein Rahmen aus 75-mm-Stäben mit vier Lamellen 75 x 75 mm im Abstand von 300 mm, die Jalousie, die die Ansicht zeichnet."},
}
STALE = {
    "200": [("en", ", eight louvre blades of 60 x 30 mm per opening at 200 mm pitch.", "."), ("de", ", je Öffnung acht Lamellen 60 x 30 mm im Abstand von 200 mm.", ".")],
    "500": [("en", "five louvre blades of 120 x 75 mm per opening without frames, ", ""), ("de", "je Öffnung fünf Lamellen 120 x 75 mm ohne Rahmen, ", ""),
            ("en", "Each inlet has fifteen separate inclined blades ahead of three deep splitters.", "Each inlet has four blades ahead of three deep splitters."), ("de", "Jeder Einlass enthält fünfzehn geneigte Lamellen vor drei tiefen Kulissen.", "Jeder Einlass enthält vier Lamellen vor drei tiefen Kulissen.")],
}


def main():
    """▶️ Rewrites the blade rows of the two coarse scales and adds the frames of 1:500."""
    with open(SOURCE, encoding="utf8") as f:
        document = json.load(f)
    medium, coarse = document["volumes"]["200"], document["volumes"]["500"]

    for blade in medium["body"]:
        if blade["id"].startswith("inlet-louvre-") and blade["id"].endswith("-slat"):
            side = -1 if blade["id"].endswith("-r-slat") else 1
            blade["profile"] = [[-0.0175, -0.65], [0.0175, -0.65], [0.0175, 0.65], [-0.0175, 0.65]]
            blade["from"], blade["to"] = -0.001, 0.001
            blade["repeat"] = {"count": 15, "translation": [0.0, 0.1, 0.0]}
            blade["transform"] = {"translation": [blade["transform"]["translation"][0], 0.6, side * 1.2045], "rotation": [side * 0.25882, 0.0, 0.0, 0.96593]}
            blade["slender"] = True

    if all(solid["id"] != "door-1" for solid in medium["body"]):
        fine = document["volumes"]["100"]
        at = next(i for i, solid in enumerate(medium["body"]) if solid["id"] == "wall-end-a")
        for name in ("door-2", "door-1"):
            door = copy.deepcopy(next(solid for solid in fine["body"] if solid["id"] == name))
            door["size"][2] = 0.03
            door["transform"]["translation"][2] = -1.205
            medium["body"].insert(at, door)
            medium["thermal"].extend(copy.deepcopy(solid) for solid in fine["thermal"] if solid.get("of") == name)

    for blade in coarse["body"]:
        if blade["id"].startswith("inlet-louvre-") and blade["id"].endswith("-slat"):
            side = -1 if blade["id"].endswith("-r-slat") else 1
            blade["profile"] = [[-0.0375, -0.65], [0.0375, -0.65], [0.0375, 0.65], [-0.0375, 0.65]]
            blade["from"], blade["to"] = -0.0375, 0.0375
            blade["repeat"] = {"count": 4, "translation": [0.0, 0.3, 0.0]}
            blade["transform"] = {"translation": [blade["transform"]["translation"][0], 0.9, side * 1.1825]}
    if all(solid["id"] != "louvre-frame-r" for solid in coarse["body"]):
        at = next(i for i, solid in enumerate(coarse["body"]) if solid["id"] == "inlet-louvre-1-r-slat")
        for name, (z0, z1) in (("louvre-frame-l", (1.145, 1.22)), ("louvre-frame-r", (-1.22, -1.145))):
            frame = copy.deepcopy(next(solid for solid in medium["body"] if solid["id"] == name))
            frame["profile"] = [[0.18, 0.5], [1.58, 0.5], [1.58, 2.1], [0.18, 2.1]]
            frame["holes"] = [[[0.255, 0.575], [0.255, 2.025], [1.505, 2.025], [1.505, 0.575]]]
            frame["from"], frame["to"] = z0, z1
            coarse["body"].insert(at, frame)
            twin = copy.deepcopy(next(solid for solid in medium["thermal"] if solid.get("of") == name))
            coarse["thermal"].insert(next(i for i, solid in enumerate(coarse["thermal"]) if solid.get("of") == "inlet-louvre-1-r-slat"), twin)

    for scale, pairs in STALE.items():
        for decision in document["volumes"][scale]["decisions"]:
            for language, old, new in pairs:
                decision[language] = decision[language].replace(old, new)
    for scale, sentence in DECISIONS.items():
        decisions = document["volumes"][scale]["decisions"]
        if sentence not in decisions:
            decisions.append(sentence)
    with open(SOURCE, "w", encoding="utf8") as f:
        json.dump(document, f, ensure_ascii=False, indent=1)
    print("applied")


if __name__ == "__main__":
    main()
