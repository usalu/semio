"""🛠️ Round 3, machinery/expansion-valve: builds the candidate asset document (catalog file re-read, roots of `r3-expansion-valve-roots.py`, volumes of `r3-expansion-valve-model.py`) under `🗑️generated/r3-expansion-valve/🔣️.json`; with `--source` it also writes the volumes source `volumes/machinery/expansion-valve.json`.

Usage: python3 r3-expansion-valve-build.py [--source]
"""
import copy
import importlib.util
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/expansion-valve/🔣️.json"))
OUT = os.path.join(HERE, "🗑️generated", "r3-expansion-valve", "🔣️.json")
SOURCE = os.path.join(HERE, "volumes", "machinery", "expansion-valve.json")


def load(name):
    """📥 Imports a sibling script by file name."""
    spec = importlib.util.spec_from_file_location(name.replace("-", "_").replace(".py", ""), os.path.join(HERE, name))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def main():
    doc = json.load(open(CATALOG, encoding="utf-8"))
    cand = copy.deepcopy(doc)
    load("r3-expansion-valve-roots.py").apply_roots(cand["asset"])
    vols = load("r3-expansion-valve-model.py").volumes()
    cand["asset"]["volumes"] = vols
    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    json.dump(cand, open(OUT, "w", encoding="utf-8"), indent=2, ensure_ascii=False)
    print("candidate", OUT)
    if "--source" in sys.argv:
        json.dump({"volumes": vols}, open(SOURCE, "w", encoding="utf-8"), indent=2, ensure_ascii=False)
        print("source", SOURCE)


if __name__ == "__main__":
    main()
