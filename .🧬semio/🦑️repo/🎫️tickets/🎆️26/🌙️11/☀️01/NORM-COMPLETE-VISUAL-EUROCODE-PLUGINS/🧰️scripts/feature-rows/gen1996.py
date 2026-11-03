"""Derives the EN 1996 fixture `detail-extent-fail` from `detail-rich-pass` (hand-edited leaves only; run with --write): lintel and chase protrude past the wall (5.4.extent, 8.6.chase-extent), wall-b is thin and tall (5.5.1.slenderness) and bears on an oversized area (6.1.3.area). Usage: gen1996.py [--write]."""
import copy, json, pathlib, sys

root = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🚦️check-outcome")
doc = copy.deepcopy(json.load(open(root / "detail-rich-pass/🔣️.json", encoding="utf8")))
wall_a, wall_b = doc["walls"]
wall_a["lintels"][0]["positionM"] = 4.2
doc["chases"][0]["yM"] = 2.0
wall_b["thicknessM"] = wall_b["unitWidthM"] = 0.115
wall_b["heightM"] = 6.0
wall_b["supportSides"] = 2
for case in wall_b["loadCases"]:
    for c in case["concentrated"]:
        c["bearingAreaM2"] = 0.5
text = json.dumps(doc, indent=1, ensure_ascii=False) + "\n"
if "--write" in sys.argv:
    (root / "detail-extent-fail").mkdir(exist_ok=True)
    (root / "detail-extent-fail/🔣️.json").write_text(text, encoding="utf8")
else:
    print(text[:200])
