"""🗿 Writes the round-5 handcrafted volumes of `humans/standing-adult` into the authoring source `volumes/humans/standing-adult.json`.

Usage: python3 r5-standing-adult-model.py [--only 50]
Every list of every scale is stated in the module named in `MODULES`; this script only assembles and saves them.
"""
import importlib
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, T)
SOURCE = os.path.join(T, "volumes/humans/standing-adult.json")
MODULES = {50: "r5_standing_adult_s50", 100: "r5_standing_adult_s100", 200: "r5_standing_adult_s200", 500: "r5_standing_adult_s500"}


def scale(denominator):
    name = MODULES[denominator]
    if not os.path.exists(os.path.join(T, name + ".py")):
        name = "r3_standing_adult_s%d" % denominator
    module = importlib.import_module(name)
    return {"status": "authored", "denominator": denominator, "body": module.body(), "thermal": module.thermal(), "structure": module.structure(), "decisions": module.decisions()}


def main():
    source = json.load(open(SOURCE, encoding="utf-8"))
    only = sys.argv[sys.argv.index("--only") + 1] if "--only" in sys.argv else None
    for denominator in (50, 100, 200, 500):
        if only and str(denominator) != only:
            continue
        fresh = scale(denominator)
        source["volumes"][str(denominator)] = fresh
        print("1:%d body %d thermal %d structure %d" % (denominator, len(fresh["body"]), len(fresh["thermal"]), len(fresh["structure"])))
    tmp = SOURCE + ".r5.tmp"
    json.dump(source, open(tmp, "w", encoding="utf-8"), ensure_ascii=False)
    os.replace(tmp, SOURCE)


main()
