"""🗿 Writes the handcrafted volumes of `humans/standing-phone-user` (round 3) into the authoring source `volumes/humans/standing-phone-user.json`.

Usage: python3 r3-standing-phone-user-model.py [--only 50]
Every list of every scale is stated in the `r3_standing_phone_user_s<scale>` modules next to this file; this script only assembles and saves them.
"""
import importlib
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, T)
SOURCE = os.path.join(T, "volumes/humans/standing-phone-user.json")


def scale(denominator):
    module = importlib.import_module("r3_standing_phone_user_s%d" % denominator)
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
    tmp = SOURCE + ".r3.tmp"
    json.dump(source, open(tmp, "w", encoding="utf-8"), ensure_ascii=False)
    os.replace(tmp, SOURCE)


main()
