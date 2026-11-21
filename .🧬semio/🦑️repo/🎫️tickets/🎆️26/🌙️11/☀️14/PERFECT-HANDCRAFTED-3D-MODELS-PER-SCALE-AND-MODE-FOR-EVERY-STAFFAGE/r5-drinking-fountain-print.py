"""⛲ Round 5 drinking fountain: the 1:500 band (1:350 to 1:1000) printed the 38 mm push button and the 28 mm goose-neck as 0.17 / 0.18 mm features at 1:1000 (below the 0.2 mm office-laser minimum). They are redrawn with the 0.18 mm pen in every view, so they print over the whole band without narrowing it (the bands 20-75-150-350-1000 stay gapless).

Usage: python3 r5-drinking-fountain-print.py [--apply]
See ./r5-brief.md (addendum), ./r3-objects-drinking-fountain.md.
"""
import sys

import r5_o02_lib as L

PEN = 0.18
SHAPES = {"button", "bubbler"}


def roots(asset):
    for view in asset["representations"]["500"]["geometry"].values():
        for s in view:
            if s["id"] in SHAPES:
                s["penMm"] = PEN
    return asset


if __name__ == "__main__":
    doc = L.catalog("drinking-fountain")
    roots(doc["asset"])
    print(L.write_candidate("drinking-fountain", doc))
    if "--apply" in sys.argv:
        print("written:", L.apply_members("drinking-fountain", doc["asset"], ["representations"]))
