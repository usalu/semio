"""🚶 Rework 05, parent carrying a toddler: pose `walk-6` swings the far upper arm 1 degree ahead instead of hanging exactly plumb (0 degrees), as `ARM_SWING` of `rework-05-parent-carrying-toddler-roots.py` now states.

With the arm exactly plumb the extruded glove shape `glove-thenar-l` is seen exactly edge-on in the front view of `walk-carrying` at 0.75 s and the product's polygon union fails on its coincident faces (check CLI: "the union of 12 polygon(s) with area cannot be built"). One-shot on the candidate `🗑️generated/r3-parent-carrying-toddler/🔣️.json`; publish with `rework-05-parent-carrying-toddler-publish.py`.
"""
import json
import os

CANDIDATE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "🗑️generated", "r3-parent-carrying-toddler", "🔣️.json")


def main():
    """▶️ Sets the two angles of the far arm in `walk-6`."""
    with open(CANDIDATE, encoding="utf8") as f:
        document = json.load(f)
    bones = next(p for p in document["asset"]["poses"] if p["id"] == "walk-6")["bones"]
    if bones["upper-arm-l"] == {"angleDeg": 1}:
        raise SystemExit("already applied")
    if bones["upper-arm-l"] != {"angleDeg": 0} or bones["forearm-l"] != {"angleDeg": 14}:
        raise SystemExit("walk-6 holds other angles than expected")
    bones["upper-arm-l"] = {"angleDeg": 1}
    bones["forearm-l"] = {"angleDeg": 14.8}
    with open(CANDIDATE, "w", encoding="utf8") as f:
        f.write(json.dumps(document, ensure_ascii=False) + "\n")
    print("applied")


if __name__ == "__main__":
    main()
