"""🏗️ Builds the round 3 forklift: candidate asset document under 🗑️generated/r3-forklift and the volumes source; `--apply` writes the changed root members into the catalog file (re-read right before, member by member, nothing else touched); `--view` also writes the candidate as an ad-hoc asset (`🗑️generated/r3-forklift/forklift-r3/🔣️.json` with `volumes/adhoc/forklift-r3.json`) so the r3 tools can render it before the catalog is written."""
import copy
import importlib.util
import json
import os
import re
import sys

T = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/forklift/🔣️.json"
MEMBERS = ["summary", "provenance", "dimensions", "rig", "anchors", "representations", "poses", "clips", "thermal"]
SCALES = [int(s) for s in os.environ.get("R3_SCALES", "50,100,200,500").split(",")]


def load(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(T, f"r3-forklift-{name}.py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def member(text, key):
    found = re.search(r'\n  "' + key + r'"\s*:\s*', text)
    depth, index, quoted = 0, found.end(), False
    while True:
        char = text[index]
        if quoted:
            if char == "\\":
                index += 1
            elif char == '"':
                quoted = False
        elif char == '"':
            quoted = True
        elif char in "{[":
            depth += 1
        elif char in "}]":
            depth -= 1
            if depth == 0:
                return found.end(), index + 1
        index += 1


def dumped(value):
    lines = json.dumps(value, indent=1, ensure_ascii=False).split("\n")
    return "\n".join(line if i == 0 else "  " + line for i, line in enumerate(lines))


def main():
    K = load("kit")
    roots = load("roots")
    texts = json.load(open(os.path.join(T, "r3-forklift-texts.json"), encoding="utf-8"))
    scales = {}
    for denominator in SCALES:
        parts = load(f"s{denominator}")
        scales[str(denominator)] = (parts.build(K.Scale(denominator), K), parts.DECISIONS)
    document = json.load(open(CATALOG, encoding="utf-8"))
    asset = copy.deepcopy(document["asset"])
    roots.apply(asset, K, {"shapes": {scale: built.shapes() for scale, (built, _) in scales.items()}, "decisions": texts["decisions"]})
    volumes = copy.deepcopy(asset["volumes"])
    for scale, (built, decisions) in scales.items():
        volumes[scale] = built.volume(decisions)
    asset["volumes"] = volumes
    out = os.path.join(T, "🗑️generated", "r3-forklift")
    os.makedirs(out, exist_ok=True)
    json.dump({**document, "asset": asset}, open(os.path.join(out, "🔣️.json"), "w", encoding="utf-8"), indent=1, ensure_ascii=False)
    json.dump({"volumes": volumes}, open(os.path.join(T, "volumes", "machinery", "forklift.json"), "w", encoding="utf-8"), indent=1, ensure_ascii=False)
    if "--view" in sys.argv:
        view = os.path.join(out, "forklift-r3")
        os.makedirs(view, exist_ok=True)
        json.dump({**document, "asset": asset}, open(os.path.join(view, "🔣️.json"), "w", encoding="utf-8"), indent=1, ensure_ascii=False)
        os.makedirs(os.path.join(T, "volumes", "adhoc"), exist_ok=True)
        json.dump({"volumes": volumes}, open(os.path.join(T, "volumes", "adhoc", "forklift-r3.json"), "w", encoding="utf-8"), indent=1, ensure_ascii=False)
    for scale, (built, _) in scales.items():
        shapes = built.shapes()
        print(scale, "body", len(built.body), "thermal", len(built.thermal()), "structure", len(built.structure), "2D", {view: len(items) for view, items in shapes.items()})
    if "--apply" in sys.argv:
        text = open(CATALOG, encoding="utf-8").read()
        for key in MEMBERS:
            start, end = member(text, key)
            text = text[:start] + dumped(asset[key]) + text[end:]
        open(CATALOG, "w", encoding="utf-8").write(text)
        print("catalog members written:", ", ".join(MEMBERS))


main()
