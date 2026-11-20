"""🏗 Round 3, furniture/office-swivel-chair: assembles the candidate asset document (corrected roots, handcrafted volumes, redrawn 2D) under 🗑️generated/r3-office-swivel-chair/ and its volumes source under volumes/adhoc/, so every tool can judge it before the catalog file is touched.

Usage: python3 r3-office-swivel-chair-build.py [--scales 50,100,200,500]
"""
import copy
import importlib.util
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/office-swivel-chair/🔣️.json")
OUT = os.path.join(HERE, "🗑️generated", "r3-office-swivel-chair")


def module(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(HERE, f"r3-office-swivel-chair-{name}.py"))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def main():
    kit = module("kit")
    body = module("body")
    parts = {name: module(name) for name in ("lists", "roots", "draw") if os.path.exists(os.path.join(HERE, f"r3-office-swivel-chair-{name}.py"))}
    document = json.load(open(CATALOG, encoding="utf-8"))
    asset = document["asset"]
    asset["rig"] = kit.rig()
    volumes = copy.deepcopy(asset["volumes"])
    scales = [s for s in (sys.argv[sys.argv.index("--scales") + 1].split(",") if "--scales" in sys.argv else ["50", "100", "200", "500"])]
    for scale in scales:
        maker = getattr(body, f"body{scale}", None)
        if maker is None:
            continue
        volumes[scale]["body"] = maker()
        if "lists" in parts:
            parts["lists"].fill(asset, volumes[scale], int(scale))
    asset["volumes"] = volumes
    if "roots" in parts:
        parts["roots"].apply(asset)
    if "draw" in parts:
        parts["draw"].apply(asset, OUT)
    os.makedirs(OUT, exist_ok=True)
    json.dump(document, open(os.path.join(OUT, "🔣️.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    os.makedirs(os.path.join(HERE, "volumes", "adhoc"), exist_ok=True)
    json.dump({"volumes": volumes}, open(os.path.join(HERE, "volumes", "adhoc", "r3-office-swivel-chair.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print("candidate", os.path.join(OUT, "🔣️.json"), {k: [len(v.get(l, [])) for l in ("body", "thermal", "structure")] for k, v in volumes.items()})


if __name__ == "__main__":
    main()
