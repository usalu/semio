import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, "..", "🧰️FW"))
from atomic_json import write_json_atomic  # noqa: E402

ROOT = os.path.abspath(os.path.join(HERE, *([".."] * 8), "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog"))


def label(name):
    return {"en": name, "de": name}


SECTIONS = {
    os.path.join("🛋️furniture", "bookshelf"): {"front": {"plane": 0.4, "label": label("A-A")}},
    os.path.join("🛋️furniture", "three-seat-sofa"): {"front": {"plane": 1.0, "label": label("A-A")}},
    os.path.join("🧱️components", "curtain-wall-bay"): {"front": {"plane": 0.7, "label": label("A-A")}, "plan": {"plane": 2.075, "label": label("B-B")}},
    os.path.join("🧱️components", "masonry-wall-with-lintel"): {"front": {"plane": 1.25, "label": label("A-A")}, "plan": {"plane": 1.5, "label": label("B-B")}},
    os.path.join("🧱️components", "sliding-door"): {"plan": {"plane": 1.0, "label": label("A-A")}},
}

for folder, sections in SECTIONS.items():
    path = os.path.join(ROOT, folder, "🔣️.json")
    with open(path, encoding="utf8") as handle:
        document = json.load(handle)
    count = 0
    for key, rep in document["asset"]["representations"].items():
        if rep["status"] != "authored":
            continue
        rebuilt = {}
        for name, value in rep.items():
            if name == "sections":
                continue
            rebuilt[name] = value
            if name == "geometry":
                rebuilt["sections"] = sections
        document["asset"]["representations"][key] = rebuilt
        count += 1
    write_json_atomic(path, document)
    print(folder, count)
