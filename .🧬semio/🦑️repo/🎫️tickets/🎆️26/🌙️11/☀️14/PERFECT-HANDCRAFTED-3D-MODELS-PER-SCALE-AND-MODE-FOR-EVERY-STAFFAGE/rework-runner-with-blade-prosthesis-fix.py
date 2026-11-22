"""🩹 runner-with-blade-prosthesis: the forefoot of the shoe at 1:500 gets the printable 52 mm, and the declared areas of the shorts (waist to mid-thigh over both legs) and of the bare face, ears and neck follow the modelled surfaces.

Usage: python3 rework-runner-with-blade-prosthesis-fix.py     writes the volumes source and the `thermal` member of the catalog file
"""
import copy
import importlib.util
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(T, *([".."] * 7)))
SLUG = "runner-with-blade-prosthesis"
CATALOG = os.path.join(REPO, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "👤️humans", SLUG, "🔣️.json")
SOURCE = os.path.join(T, "volumes", "humans", SLUG + ".json")
spec = importlib.util.spec_from_file_location("rig_lateral", os.path.join(T, "rig-lateral-standing-adult.py"))
rig = importlib.util.module_from_spec(spec)
spec.loader.exec_module(rig)

AREAS = {"shorts": 0.374, "face-skin": 0.1}
TOE_HALF_HEIGHT = 0.026

source = json.load(open(SOURCE, encoding="utf8"))
for name in ("body", "thermal"):
    for solid in source["volumes"]["500"][name]:
        if solid["id"].endswith("shoe-toe-l") and "sections" in solid:
            for section in solid["sections"]:
                if section["ry"] == 0.0256:
                    section["ry"] = TOE_HALF_HEIGHT
                if section["at"][1] == 0.0256:
                    section["at"][1] = TOE_HALF_HEIGHT
json.dump(source, open(SOURCE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
open(SOURCE, "a", encoding="utf8").write("\n")
text = open(CATALOG, encoding="utf8").read()
document = json.loads(text)
before = copy.deepcopy(document["asset"]["thermal"])
for region in document["asset"]["thermal"]["regions"]:
    if region["id"] in AREAS:
        region["areaM2"] = AREAS[region["id"]]
open(CATALOG, "w", encoding="utf8").write(rig.splice(text, "thermal", before, document["asset"]["thermal"]))
