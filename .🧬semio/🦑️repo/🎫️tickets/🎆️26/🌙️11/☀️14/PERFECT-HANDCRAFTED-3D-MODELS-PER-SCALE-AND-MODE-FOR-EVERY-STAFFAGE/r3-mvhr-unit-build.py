"""🏗️ Round 3, appliances/mvhr-unit: builds the candidate asset document (catalog file re-read, roots of `r3-mvhr-unit-roots.py`, volumes of `r3-mvhr-unit-volumes.py`) under `🗑️generated/r3-mvhr-unit/🔣️.json` and writes the volumes source `volumes/appliances/mvhr-unit.json`.

Usage: python3 r3-mvhr-unit-build.py
"""
import copy
import importlib.util
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/mvhr-unit/🔣️.json"))
OUT = os.path.join(HERE, "🗑️generated", "r3-mvhr-unit", "🔣️.json")
SOURCE = os.path.join(HERE, "volumes", "appliances", "mvhr-unit.json")
AREAS = {"casing": 1.37, "extract-fan": 0.078, "filters": 0.12}


def load(name):
    """📥 Imports a sibling script by file name."""
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(HERE, name))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def main():
    doc = json.load(open(CATALOG, encoding="utf-8"))
    cand = copy.deepcopy(doc)
    load("r3-mvhr-unit-roots.py").apply_roots(cand["asset"], AREAS)
    vols = load("r3-mvhr-unit-volumes.py").volumes()
    cand["asset"]["volumes"] = vols
    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    json.dump(cand, open(OUT, "w", encoding="utf-8"), indent=2, ensure_ascii=False)
    json.dump({"volumes": vols}, open(SOURCE, "w", encoding="utf-8"), indent=2, ensure_ascii=False)
    print("candidate", OUT)
    print("source", SOURCE)


if __name__ == "__main__":
    main()
