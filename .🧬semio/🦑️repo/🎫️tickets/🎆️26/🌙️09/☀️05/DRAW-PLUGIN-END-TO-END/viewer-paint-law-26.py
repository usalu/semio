"""🧪️ Independently checks the language-neutral Viewer camera evidence using exact fractions."""
from fractions import Fraction
import json
from pathlib import Path

fixture = json.loads(Path(__file__).with_name("viewer-paint-law-26.json").read_text(encoding="utf-8"))
for case in fixture["cases"]:
    before = fixture["before"]
    scales = [Fraction(case[axis], before[axis]) for axis in ["pixels", "width", "height"]]
    assert (case["bitmap"] == before["bitmap"] and scales[0] >= Fraction(27, 25) and min(scales[1:]) >= Fraction(21, 20)) == case["zoom"], case
print(f"{len(fixture['cases'])} neutral zoom cases")
