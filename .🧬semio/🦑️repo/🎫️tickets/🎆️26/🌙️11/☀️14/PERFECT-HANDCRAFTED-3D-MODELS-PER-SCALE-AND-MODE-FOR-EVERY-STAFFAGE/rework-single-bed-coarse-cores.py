"""🛏️ single-bed thermal 1:100, 1:200, 1:500: the mattress keeps its two nested layers, as at 1:50.

Hand-chosen: the cold-foam core is the mattress part of that scale set in by the 10 mm quilted cover all round its section and at the
two outer ends (head end of the head part, foot end of the foot part); at the two inner joints the core runs through, the mattress being
one piece. The outer solid becomes the layer `quilt`, the new solid `th-mat-*-core` the layer `foam-core`. Run, then `volumes-insert.py`.
"""
import copy
import json

SOURCE = __file__.rsplit("/", 1)[0] + "/volumes/furniture/single-bed.json"
COVER = 0.01
OUTER_END_FROM = {"mat-head": 0.6, "mat-foot": 0.6, "mat-mid": 9.0}


def core(part):
    """🧽 The foam core of one mattress part."""
    solid = copy.deepcopy(part)
    outer = OUTER_END_FROM[part["id"]]
    if solid["kind"] == "loft":
        for section in solid["sections"]:
            section["rx"] = round(section["rx"] - COVER, 4)
            section["ry"] = round(section["ry"] - COVER, 4)
            if section["at"][0] > outer:
                section["at"][0] = round(section["at"][0] - COVER, 4)
    else:
        assert solid["kind"] == "box"
        size, at = solid["size"], solid["transform"]["translation"]
        size[1], size[2] = round(size[1] - 2 * COVER, 4), round(size[2] - 2 * COVER, 4)
        if outer < 1:
            size[0], at[0] = round(size[0] - COVER, 4), round(at[0] - COVER / 2, 4)
    solid.update({"id": "th-" + part["id"] + "-core", "material": "cold-foam", "tone": 0.75, "region": "mattress", "layer": "foam-core"})
    return solid


source = json.load(open(SOURCE))
for scale in ("100", "200", "500"):
    volume = source["volumes"][scale]
    assert not any(s["id"].endswith("-core") for s in volume["thermal"])
    parts = [s for s in volume["body"] if s["id"] in OUTER_END_FROM]
    assert len(parts) == 3
    for solid in volume["thermal"]:
        if solid.get("of") in OUTER_END_FROM:
            solid["layer"] = "quilt"
    volume["thermal"] = [core(p) for p in parts] + volume["thermal"]
texts = {
    "100": ("Thermal: one surface per region; the mattress keeps its two nested shells, the cover as the body surface around the cold-foam core set in by 10 mm.", "Thermik: eine Oberfläche je Region; die Matratze behält ihre zwei verschachtelten Schalen, den Bezug als Körperoberfläche um den um 10 mm zurückgesetzten Kaltschaumkern."),
    "200": ("Thermal: one surface per region; the slat slabs are the hidden region; the mattress is the cover shell around the cold-foam core set in by 10 mm.", "Thermik: eine Oberfläche je Region; die Rostplatten sind die verdeckte Region; die Matratze ist die Bezugsschale um den um 10 mm zurückgesetzten Kaltschaumkern."),
    "500": ("Thermal: panels, tray and leg are the frame surface, the mattress blocks the cover shell around three core blocks set in by 10 mm, the slatted base is one hidden plate inside the tray.", "Thermik: Platten, Wanne und Fuß sind die Rahmenoberfläche, die Matratzenblöcke die Bezugsschale um drei um 10 mm zurückgesetzte Kernblöcke, der Lattenrost ist eine verdeckte Platte in der Wanne."),
}
for scale, (en, de) in texts.items():
    decision = source["volumes"][scale]["decisions"][1]
    assert decision["en"].startswith("Thermal:") and " Structure:" in decision["en"] and " Tragwerk:" in decision["de"]
    decision["en"] = en + decision["en"][decision["en"].index(" Structure:"):]
    decision["de"] = de + decision["de"][decision["de"].index(" Tragwerk:"):]
open(SOURCE, "w").write(json.dumps(source, ensure_ascii=False, indent=1))
print("cores written")
