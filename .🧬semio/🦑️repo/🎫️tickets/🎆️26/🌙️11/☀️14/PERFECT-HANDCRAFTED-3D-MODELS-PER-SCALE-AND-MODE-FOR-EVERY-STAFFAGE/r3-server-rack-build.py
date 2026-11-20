"""🏗 Round 3, appliances/server-rack: assembles the candidate asset document (corrected roots, handcrafted volumes, redrawn 2D) under 🗑️generated/r3-server-rack/ and its volumes source under volumes/adhoc/r3-server-rack.json, so every tool can judge the candidate before the catalog file is touched.

Usage: python3 r3-server-rack-build.py
"""
import importlib.util
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/server-rack/🔣️.json"))
OUT = os.path.join(HERE, "🗑️generated", "r3-server-rack")


def module(name):
    """📦 Loads one of the sibling scripts."""
    spec = importlib.util.spec_from_file_location("r3_server_rack_" + name, os.path.join(HERE, f"r3-server-rack-{name}.py"))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def candidate():
    """🧩 The catalog document with every round 3 change applied in memory."""
    kit, body, lists, roots, draw = (module(name) for name in ("kit", "body", "lists", "roots", "draw"))
    document = json.load(open(CATALOG, encoding="utf-8"))
    asset = document["asset"]
    asset["rig"] = kit.rig(asset["rig"])
    roots.apply(asset)
    for scale in ("50", "100", "200", "500"):
        solids = getattr(body, "body" + scale)()
        asset["volumes"][scale] = {"status": "authored", "denominator": int(scale), "body": solids, "thermal": lists.thermal(scale, solids), "structure": lists.structure(scale, solids), "decisions": lists.decisions(scale)}
    draw.apply(asset)
    return document


def main():
    document = candidate()
    os.makedirs(OUT, exist_ok=True)
    json.dump(document, open(os.path.join(OUT, "🔣️.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    os.makedirs(os.path.join(HERE, "volumes", "adhoc"), exist_ok=True)
    volumes = document["asset"]["volumes"]
    json.dump({"volumes": volumes}, open(os.path.join(HERE, "volumes", "adhoc", "r3-server-rack.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print("candidate", os.path.join(OUT, "🔣️.json"), {k: [sum(s.get("repeat", {}).get("count", 1) for s in v[l]) for l in ("body", "thermal", "structure")] for k, v in volumes.items()})


if __name__ == "__main__":
    main()
