"""📦 Dairy cow publish: writes the built candidate document into the catalog asset file and the volumes source in the catalog's own layout.

Usage: python3 rework-dairy-cow-publish.py [--keep-2d]
The catalog file must still be the frozen copy or the last publish of this script (it is re-read right before the write); `--keep-2d` keeps the `representations` that are in the catalog file (drawn after the first publish).
"""
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/dairy-cow/🔣️.json"
SOURCE = os.path.join(HERE, "volumes", "animals", "dairy-cow.json")
GENERATED = os.path.join(HERE, "🗑️generated", "rework-dairy-cow")
CANDIDATE = os.path.join(HERE, "🗑️generated", "r3-dairy-cow", "🔣️.json")


def scalar(value):
    return not isinstance(value, (dict, list))


def inline(value):
    if isinstance(value, list):
        return all(scalar(item) for item in value) or (all(isinstance(item, list) and all(scalar(leaf) for leaf in item) for item in value) and len(json.dumps(value)) <= 160)
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


def main():
    keep = "--keep-2d" in sys.argv
    candidate = json.load(open(CANDIDATE, encoding="utf-8"))
    current_text = open(CATALOG, encoding="utf-8").read()
    current = json.loads(current_text)
    frozen = json.load(open(os.path.join(GENERATED, "catalog-before.json"), encoding="utf-8"))
    last_path = os.path.join(GENERATED, "catalog-published.json")
    last = json.load(open(last_path, encoding="utf-8")) if os.path.exists(last_path) else None

    def same(a, b):
        return {k: v for k, v in a["asset"].items() if k != "representations"} == {k: v for k, v in b["asset"].items() if k != "representations"}

    if not same(current, frozen) and not (last and same(current, last)):
        raise SystemExit("catalog asset was changed by someone else; merge by hand")
    if keep:
        candidate["asset"]["representations"] = current["asset"]["representations"]
    text = dump(candidate, 0) + "\n"
    if open(CATALOG, encoding="utf-8").read() != current_text:
        raise SystemExit("catalog asset changed while publishing; retry")
    open(CATALOG, "w", encoding="utf-8").write(text)
    open(last_path, "w", encoding="utf-8").write(text)
    open(SOURCE, "w", encoding="utf-8").write(dump({"volumes": candidate["asset"]["volumes"]}, 0) + "\n")
    print("[DEBUG] published", len(text), "bytes; layout round trip of the previous file:", dump(current, 0) + "\n" == current_text)


main()
