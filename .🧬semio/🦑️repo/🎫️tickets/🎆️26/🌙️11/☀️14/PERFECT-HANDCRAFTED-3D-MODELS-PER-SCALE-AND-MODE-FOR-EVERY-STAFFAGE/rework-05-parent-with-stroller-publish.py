"""📦 Writes the validated candidate of `humans/parent-with-stroller` into the catalog asset and the volumes source.

Usage: python3 rework-05-parent-with-stroller-publish.py   (after the girdle and the coarse script; the candidate must validate first)
"""
import json

T = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/parent-with-stroller/🔣️.json"
SOURCE = T + "/volumes/humans/parent-with-stroller.json"
CANDIDATE = T + "/🗑️generated/rework-parent-with-stroller/pws-cand/🔣️.json"


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


document = json.load(open(CANDIDATE, encoding="utf-8"))
open(CATALOG, "w", encoding="utf-8").write(dump(document, 0) + "\n")
open(SOURCE, "w", encoding="utf-8").write('{"volumes":' + dump(document["asset"]["volumes"], 0) + "}\n")
print("written", CATALOG, SOURCE)
