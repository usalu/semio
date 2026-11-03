import hashlib
import json
import sys

out = sys.argv[1]

text_lf = '{\n  "a": 1\n}\n'
texts = [
    {"name": "lf", "text": text_lf},
    {"name": "crlf", "text": text_lf.replace("\n", "\r\n")},
    {"name": "unicode", "text": "{\"title\": \"Büro, neutral\"}\n"},
]
for item in texts:
    item["revision"] = "sha256:" + hashlib.sha256(item["text"].replace("\r\n", "\n").encode("utf-8")).hexdigest()

canonical = [
    {"name": "sorted keys and integral floats", "source": '{"b":1.0,"a":[true,null,"ü"],"e":-0.5}', "text": '{"a":[true,null,"ü"],"b":1,"e":-0.5}'},
    {"name": "small and long numbers", "source": '{"c":1e-7,"d":22.000000000000014,"f":0.1,"g":1.5e300,"h":100}', "text": '{"c":1e-7,"d":22.000000000000014,"f":0.1,"g":1.5e+300,"h":100}'},
    {"name": "nesting", "source": '{"z":{"y":[{"b":2,"a":1}]},"x":""}', "text": '{"x":"","z":{"y":[{"a":1,"b":2}]}}'},
]

preset = {
    "id": "office-neutral",
    "title": {"en": "Office, neutral", "de": "Büro, neutral"},
    "environment": {"airC": 22.0, "radiantC": 22, "relativeHumidity": 0.5, "airSpeedMS": 0.1},
}
preset_text = '{"environment":{"airC":22,"airSpeedMS":0.1,"radiantC":22,"relativeHumidity":0.5},"id":"office-neutral","title":{"de":"Büro, neutral","en":"Office, neutral"}}'
preset_vector = {"source": json.dumps(preset, ensure_ascii=False), "text": preset_text, "revision": "sha256:" + hashlib.sha256(preset_text.encode("utf-8")).hexdigest()}


def base(**extra):
    cast = {"asset": "walking-commuter", "viewpoint": "elevation", "view": "dark-line", "theme": "dark", "scale": 50, "pose": "standing"}
    cast.update(extra)
    return cast


def without(cast, *keys):
    return {key: value for key, value in cast.items() if key not in keys}


clip = {"clip": "walk-cycle", "fps": 15}
thermal = {"view": "thermal", "environment": "winter-overcast"}

valid = [
    {"name": "a still pose", "cast": base()},
    {"name": "a clip with its clock", "cast": without(base(), "pose") | clip},
    {"name": "a thermal pose in an environment", "cast": base(**thermal)},
    {"name": "a thermal transient", "cast": without(base(view="thermal"), "pose") | {"pose": "standing", "fps": 2, "timeline": [{"t": 0, "environment": "office-neutral", "ease": "step"}, {"t": 4, "environment": "summer-hot-sun", "ease": "linear"}]}},
    {"name": "every authored scale", "cast": base(scale=500)},
    {"name": "a mirrored side elevation", "cast": base(mirror=True)},
    {"name": "transport with its start", "cast": without(base(**thermal), "pose") | clip | {"transport": ["pathlines"], "transportAgeS": 3}},
]

invalid = [
    {"name": "missing scale", "cast": without(base(), "scale"), "issue": "scale: is required"},
    {"name": "a derived custom scale", "cast": base(scale=75), "issue": "scale: is required"},
    {"name": "a scale as text", "cast": base(scale="50"), "issue": "scale: is required"},
    {"name": "a clip without fps", "cast": without(base(), "pose") | {"clip": "walk-cycle"}, "issue": "fps: is required for a clip or a timeline"},
    {"name": "a timeline without fps", "cast": base(view="thermal", timeline=[{"t": 0, "environment": "office-neutral", "ease": "step"}]), "issue": "fps: is required for a clip or a timeline"},
    {"name": "a timeline key without ease", "cast": base(view="thermal", fps=2, timeline=[{"t": 0, "environment": "office-neutral"}]), "issue": "ease: is required"},
    {"name": "thermal without environment or timeline", "cast": base(view="thermal"), "issue": "the thermal view needs an environment or a timeline"},
    {"name": "transport without its start", "cast": without(base(**thermal), "pose") | clip | {"transport": ["pathlines"]}, "issue": "need transportAgeS"},
    {"name": "transport outside the thermal view", "cast": without(base(), "pose") | clip | {"transport": ["pathlines"], "transportAgeS": 3}, "issue": "exist for the thermal view only"},
    {"name": "no view direction", "cast": without(base(), "viewpoint"), "issue": "viewpoint: is required"},
]

provenance = {
    "version": 1,
    "asset": {"id": "walking-commuter", "category": "humans", "revision": "sha256:" + "ab" * 32},
    "view": {"style": "thermal", "viewpoint": "elevation", "theme": "dark", "mirror": False},
    "scale": {"denominator": 50},
    "state": {"kind": "clip", "id": "walk-cycle"},
    "clock": {"fps": 15, "durationS": 1.2, "loop": True, "offsetS": 0, "frames": 18},
    "scenario": {"kind": "thermal", "environment": {"id": "winter-overcast", "revision": "sha256:" + "cd" * 32}, "spanK": 25, "transport": []},
}
index = {
    "version": 1,
    "cast": "walker",
    "asset": "walking-commuter",
    "view": "thermal",
    "viewpoint": "elevation",
    "frames": ["0000.svg"],
    "fps": 15,
    "durationS": 1.2,
    "loop": True,
    "mmPerMetre": 20,
    "originMm": [10, 20],
    "boundsM": {"minX": -0.5, "minY": 0, "maxX": 0.5, "maxY": 1.8},
    "heightM": 1.8,
    "provenance": provenance,
}
timeline_scenario = {"kind": "thermal", "timeline": [{"t": 0, "id": "office-neutral", "ease": "step", "revision": "sha256:" + "ef" * 32}], "spanK": None, "transport": ["parcels"]}
index_cases = {
    "valid": [
        {"name": "a thermal clip", "index": index},
        {"name": "a timeline scenario", "index": {**index, "provenance": {**provenance, "scenario": timeline_scenario}}},
        {"name": "a structural scenario", "index": {**index, "view": "forces", "provenance": {**provenance, "view": {**provenance["view"], "style": "forces"}, "scenario": {"kind": "structural", "dynamics": "quasi-static", "gravityMS2": 9.81}}}},
        {"name": "a geometry scenario", "index": {**index, "view": "dark-line", "provenance": {**provenance, "view": {**provenance["view"], "style": "dark-line"}, "scenario": {"kind": "geometry"}}}},
    ],
    "invalid": [
        {"name": "no provenance", "remove": "provenance"},
        {"name": "no scale", "remove": "provenance/scale"},
        {"name": "no asset revision", "remove": "provenance/asset/revision"},
        {"name": "a revision that is no hash", "set": {"path": "provenance/asset/revision", "value": "sha256:abc"}},
        {"name": "a derived scale", "set": {"path": "provenance/scale/denominator", "value": 75}},
        {"name": "no state", "remove": "provenance/state"},
        {"name": "an unknown view style", "set": {"path": "provenance/view/style", "value": "sepia"}},
        {"name": "no clock offset", "remove": "provenance/clock/offsetS"},
        {"name": "no scenario", "remove": "provenance/scenario"},
        {"name": "a thermal scenario without environment", "remove": "provenance/scenario/environment"},
        {"name": "an unknown scenario kind", "set": {"path": "provenance/scenario/kind", "value": "weather"}},
    ],
}

document = {
    "description": "Language-agnostic vectors of the cast provenance contract (T/r3-contract.md): the revision of a catalog file (sha256 of its text with CRLF turned into LF), canonical JSON of environment presets, valid and invalid cast entries (every missing combination is refused), and valid and invalid cast indexes with their provenance record. TypeScript and Python read this one file.",
    "revisions": texts,
    "canonical": canonical,
    "preset": preset_vector,
    "validCasts": valid,
    "invalidCasts": invalid,
    "indexes": index_cases,
}
with open(out, "w", encoding="utf-8", newline="\n") as handle:
    json.dump(document, handle, ensure_ascii=False, indent=2)
    handle.write("\n")
