"""🗿 Writes the handcrafted volumes of `humans/worker-carrying-box` (round 3) into the authoring source `volumes/humans/worker-carrying-box.json` and the candidate document `🗑️generated/r3-worker-carrying-box/🔣️.json` (the draft asset of `r3-worker-carrying-box-roots.py` with these volumes).

Usage: python3 r3-worker-carrying-box-model.py [--only 50]
Every list of every scale is stated in the `r3_worker_carrying_box_s<scale>` modules next to this file; this script only assembles and saves them.
"""
import importlib
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, T)
import r3_worker_carrying_box_kit as k

SOURCE = os.path.join(T, "volumes/humans/worker-carrying-box.json")


def scale(denominator, asset):
    module = importlib.import_module("r3_worker_carrying_box_s%d" % denominator)
    lists = module.lists(asset)
    return {"status": "authored", "denominator": denominator, "body": lists["body"], "thermal": lists["thermal"], "structure": lists["structure"], "decisions": lists["decisions"]}


def main():
    asset = k.draft()
    source = json.load(open(SOURCE, encoding="utf-8"))
    only = sys.argv[sys.argv.index("--only") + 1] if "--only" in sys.argv else None
    for denominator in (50, 100, 200, 500):
        if only and str(denominator) != only:
            continue
        fresh = scale(denominator, asset)
        source["volumes"][str(denominator)] = fresh
        print("1:%d body %d thermal %d structure %d" % (denominator, len(fresh["body"]), len(fresh["thermal"]), len(fresh["structure"])))
    tmp = SOURCE + ".r3.tmp"
    json.dump(source, open(tmp, "w", encoding="utf-8"), ensure_ascii=False)
    os.replace(tmp, SOURCE)
    document = json.load(open(k.DRAFT, encoding="utf-8"))
    document["asset"]["volumes"] = source["volumes"]
    json.dump(document, open(os.path.join(k.OUT, "🔣️.json"), "w", encoding="utf-8"), ensure_ascii=False)


main()
