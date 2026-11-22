"""🛋️ Splits the bar stool cushion by hand at 1:50 and 1:100 into the skirt that is stapled to the seat board (seat bone) and the foam crown that sinks under load (pad bone), so the loaded poses no longer push the cushion through swivel and pan.

Usage: python3 rework-bar-stool-cushion.py [--catalog]
Without a flag it writes the source `volumes/furniture/bar-stool.json` and the candidate `🗑️generated/r3-bar-stool/🔣️.json`; with `--catalog` it writes the catalog file too (re-read immediately before).
"""
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes/furniture/bar-stool.json")
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/bar-stool/🔣️.json"
CANDIDATE = os.path.join(T, "🗑️generated/r3-bar-stool/🔣️.json")

SEAT_AXIS = {"axis": [1.0, 0.0, 0.0], "origin": [-0.698, 0.0, 0.0]}
PAD_AXIS = {"axis": [0.0, 1.0, 0.0], "origin": [0.0, -0.72, 0.0]}
SEAT_HEIGHT, PAD_HEIGHT = 0.698, 0.72
SKIRT = {
    "50": [[0.698, 0.0], [0.698, 0.175], [0.69867, 0.1775], [0.7005, 0.17933], [0.703, 0.18], [0.727, 0.18], [0.73, 0.177], [0.73, 0.0]],
    "100": [[0.698, 0.0], [0.698, 0.177], [0.701, 0.18], [0.73, 0.18], [0.73, 0.0]],
}
CROWN = {
    "50": [[0.728, 0.0], [0.728, 0.177], [0.731, 0.18], [0.73518, 0.17932], [0.74, 0.17732], [0.74414, 0.17414], [0.74732, 0.17], [0.74932, 0.16518], [0.75, 0.16], [0.751, 0.1], [0.752, 0.0]],
    "100": [[0.728, 0.0], [0.728, 0.18], [0.74, 0.17732], [0.74732, 0.17], [0.75, 0.16], [0.752, 0.0]],
}
PIPING_LIFT = 0.0027
FOAM_BASE = [[0.71, 0.0], [0.71, 0.1785], [0.7285, 0.1785], [0.7285, 0.0]]
FOAM_CROWN = [[0.7295, 0.0], [0.7295, 0.1765], [0.731, 0.1785], [0.73479, 0.17787], [0.73925, 0.17602], [0.74308, 0.17308], [0.74602, 0.16925], [0.74787, 0.16479], [0.7485, 0.16], [0.7495, 0.1], [0.7505, 0.0]]
DECISIONS = {
    "50": {
        "en": " The cushion is two solids: the skirt (cover wrapped round the board, 0.698 to 0.730 m) stays on the seat bone, the foam crown above the piping seam (0.728 to 0.752 m) rides on the pad bone and sinks into the skirt under load, 18 mm occupied and 30 mm at the proof load, so no pose pushes upholstery through the swivel or the pan; the piping lies in the groove between the two.",
        "de": " Das Polster besteht aus zwei Körpern: die Schürze (um die Platte geschlagener Bezug, 0,698 bis 0,730 m) bleibt am Sitzknochen, die Schaumkuppe über der Kedernaht (0,728 bis 0,752 m) sitzt am Polsterknochen und sinkt unter Last in die Schürze, 18 mm besetzt und 30 mm unter Prüflast, sodass keine Haltung Polster durch Drehteller oder Pfanne drückt; der Keder liegt in der Fuge zwischen beiden.",
    },
    "100": {
        "en": " The cushion keeps the split of 1:50: a 32 mm skirt on the seat bone and a 24 mm crown on the pad bone that sinks into it under load.",
        "de": " Das Polster behält die Teilung von 1:50: eine 32-mm-Schürze am Sitzknochen und eine 24-mm-Kuppe am Polsterknochen, die unter Last in sie einsinkt.",
    },
    "200": {
        "en": " The cushion stays one disc on the pad bone: its sinking of 18 to 30 mm is below the 30 mm feature, the disc then covers the upper part of the mount band, which is narrower than the cushion.",
        "de": " Das Polster bleibt eine Scheibe am Polsterknochen: sein Einsinken von 18 bis 30 mm liegt unter dem 30-mm-Merkmal, die Scheibe deckt dann den oberen Teil des schmaleren Aufnahmebands.",
    },
}


MASS_NOTES = {
    "50": {"en": " The frame mass is marked where the steel is: one marker in the pan, one at the middle of each leg and four on the ring.", "de": " Die Gestellmasse ist dort markiert, wo der Stahl ist: eine Marke in der Pfanne, je eine in der Mitte jedes Beins und vier auf dem Ring."},
    "100": {"en": " The frame mass is marked in the pan, at the middle of each leg and at four points of the ring.", "de": " Die Gestellmasse ist in der Pfanne, in der Mitte jedes Beins und an vier Punkten des Rings markiert."},
    "200": {"en": " The frame mass is marked in the mount, at the middle of each leg and at four points of the ring.", "de": " Die Gestellmasse ist in der Aufnahme, in der Mitte jedes Beins und an vier Punkten des Rings markiert."},
    "500": {"en": " The frame mass is marked at the four nodes where ring and legs meet; the cover layer colours seat and back in the thermal list.", "de": " Die Gestellmasse ist an den vier Knoten markiert, an denen Ring und Beine zusammentreffen; die Bezugsschicht färbt Sitz und Lehne in der Thermikliste."},
}


def scalar(value):
    return not isinstance(value, (dict, list))


def inline(value):
    if isinstance(value, list):
        return all(scalar(item) for item in value) or (
            all(isinstance(item, list) and all(scalar(leaf) for leaf in item) for item in value) and len(json.dumps(value)) <= 160
        )
    return False


def dump(value, depth):
    pad, inner = "  " * depth, "  " * (depth + 1)
    if isinstance(value, dict):
        if not value:
            return "{}"
        rows = [inner + json.dumps(key, ensure_ascii=False) + ": " + dump(item, depth + 1) for key, item in value.items()]
        return "{\n" + ",\n".join(rows) + "\n" + pad + "}"
    if isinstance(value, list):
        if not value:
            return "[]"
        if inline(value):
            return json.dumps(value, ensure_ascii=False)
        return "[\n" + ",\n".join(inner + dump(item, depth + 1) for item in value) + "\n" + pad + "]"
    return json.dumps(value, ensure_ascii=False)


def insert_before(solids, identifier, new):
    solids[:] = [solid for solid in solids if solid["id"] != new["id"]]
    index = next(i for i, solid in enumerate(solids) if solid["id"] == identifier)
    solids.insert(index, new)


def volumes(source):
    for scale in ("50", "100"):
        lists = source[scale]
        skirt = {"id": "cushion-skirt", "kind": "revolve", "bone": "seat", "material": "pu-leatherette", "role": "body", "tone": 0.3, "profile": SKIRT[scale], **SEAT_AXIS}
        insert_before(lists["body"], "cushion", skirt)
        for solid in lists["body"]:
            if solid["id"] == "cushion":
                solid["profile"] = CROWN[scale]
            elif solid["id"] == "piping" and solid["bone"] == "pad":
                solid["bone"] = "seat"
                solid["profile"] = [[round(a + PIPING_LIFT, 5), r] for a, r in solid["profile"]]
                solid.update(SEAT_AXIS)
        cover = "th-cushion-cover" if scale == "50" else "th-cushion"
        thermal_skirt = {"id": "th-cushion-skirt", "of": "cushion-skirt", "region": "seat-pad"}
        if scale == "50":
            thermal_skirt["layer"] = "leatherette"
            foam = {"id": "th-skirt-foam", "kind": "revolve", "bone": "seat", "material": "hr-foam", "role": "body", "tone": 0.8, "profile": FOAM_BASE, **SEAT_AXIS, "region": "seat-pad", "layer": "pad-foam"}
            insert_before(lists["thermal"], "th-cushion-foam", foam)
            for solid in lists["thermal"]:
                if solid["id"] == "th-cushion-foam":
                    solid["profile"] = FOAM_CROWN
        insert_before(lists["thermal"], cover, thermal_skirt)
        insert_before(lists["structure"], "m-cushion", {"id": "m-cushion-skirt", "of": "cushion-skirt", "member": "seat", "part": "member"})
        for solid in lists["structure"]:
            if solid["id"] == "j-pad-foam":
                solid["profile"] = [[0.722, 0.0], [0.722, 0.12], [0.732, 0.12], [0.732, 0.0]]
    for scale in ("100", "200", "500"):
        for solid in source[scale]["thermal"]:
            if solid["id"] in ("th-cushion", "th-cushion-skirt", "th-back-pad", "th-seat", "th-back"):
                solid["layer"] = "leatherette"
    for notes in (DECISIONS, MASS_NOTES):
        for scale, texts in notes.items():
            decision = source[scale]["decisions"][0]
            for language, text in texts.items():
                if text not in decision[language]:
                    decision[language] += text
    return source


def drawings(asset):
    for scale in ("50", "100"):
        geometry = asset["representations"][scale]["geometry"]
        crown = [[r, round(a - PAD_HEIGHT, 5)] for a, r in reversed(CROWN[scale])]
        skirt = [[round(a - SEAT_HEIGHT, 5), -r] for a, r in SKIRT[scale]]
        for view, shapes in geometry.items():
            shapes[:] = [shape for shape in shapes if shape["id"] != "cushion-skirt" and not (shape["id"] == "piping" and view != "plan")]
            index = next(i for i, shape in enumerate(shapes) if shape["id"] == "pad")
            pad = shapes[index]
            pad["points"] = crown
            shapes.insert(index, {"id": "cushion-skirt", "bone": "seat", "role": "body", "z": 5.5, "filled": True, "tone": 0.3, "kind": "poly", "region": "seat-pad", "material": "pu-leatherette", "penMm": 0.25, "points": skirt, "closed": True, "solid": {"kind": "revolved", "axisBone": "seat", "axis": "u", "axisX": 0, "axisZ": 0, "mirror": True}})
    pairs = [
        ("seat cushion with its top round and piping seam,", "seat cushion in two parts, the skirt on the board and the crown with its top round above the piping seam (the crown sinks into the skirt under load),"),
        ("Sitzpolster mit Rundung und Kedernaht,", "Sitzpolster in zwei Teilen, die Schürze auf der Platte und die Kuppe mit Rundung über der Kedernaht (die Kuppe sinkt unter Last in die Schürze),"),
        ("cushion with a plain top round,", "cushion as skirt and crown with a plain top round,"),
        ("Polster mit einfacher Rundung,", "Polster als Schürze und Kuppe mit einfacher Rundung,"),
    ]
    for scale in ("50", "100"):
        silhouette = asset["representations"][scale]["decisions"]["silhouette"]
        for language in silhouette:
            for old, new in pairs:
                silhouette[language] = silhouette[language].replace(old, new)


def main():
    text = open(CATALOG, encoding="utf-8").read()
    document = json.loads(text)
    source = json.load(open(SOURCE, encoding="utf-8"))
    source["volumes"] = volumes(source["volumes"])
    drawings(document["asset"])
    document["asset"]["volumes"] = source["volumes"]
    open(SOURCE, "w", encoding="utf-8").write(dump(source, 0) + "\n")
    os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
    open(CANDIDATE, "w", encoding="utf-8").write(dump(document, 0) + "\n")
    print("source and candidate written")
    if "--catalog" in sys.argv:
        if open(CATALOG, encoding="utf-8").read() != text:
            raise SystemExit("catalog changed while editing; retry")
        open(CATALOG, "w", encoding="utf-8").write(dump(document, 0) + "\n")
        print("catalog written")


main()
