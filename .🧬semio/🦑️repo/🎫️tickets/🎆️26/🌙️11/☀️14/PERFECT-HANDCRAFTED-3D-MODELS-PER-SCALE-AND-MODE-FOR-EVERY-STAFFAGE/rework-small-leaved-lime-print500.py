"""🖨️ Small-leaved lime, 1:500 side elevation: the far-side branch `ln3a` shows only a wedge beside the stem (95 mm = 0.185 mm on paper, below the 0.2 mm that print); its underside is drawn 30 mm lower at mid length so the visible wedge prints. One-shot on the built candidate, after `rework-small-leaved-lime-round.sh` and before `rework-small-leaved-lime-publish.py`: `python3 rework-small-leaved-lime-print500.py`."""
import json
import os

CANDIDATE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "🗑️generated", "rework-small-leaved-lime", "small-leaved-lime", "🔣️.json")
OLD, NEW = "L 3.0331 0.2152", "L 3.0331 0.2452"


def main():
    """▶️ Redraws the one point and writes the candidate back."""
    with open(CANDIDATE, encoding="utf8") as f:
        document = json.load(f)
    shape = next(s for s in document["asset"]["representations"]["500"]["geometry"]["elevation"] if s["id"] == "ln3a")
    if NEW in shape["d"]:
        raise SystemExit("already applied")
    if OLD not in shape["d"]:
        raise SystemExit("the candidate was rebuilt with other numbers: measure again")
    shape["d"] = shape["d"].replace(OLD, NEW)
    with open(CANDIDATE, "w", encoding="utf8") as f:
        json.dump(document, f, ensure_ascii=False)
    print("applied")


if __name__ == "__main__":
    main()
