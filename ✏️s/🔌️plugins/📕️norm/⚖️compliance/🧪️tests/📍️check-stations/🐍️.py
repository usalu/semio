#!/usr/bin/env python3
"""🛂️ Independent oracle of the check-station law: reads the committed fixture and verifies utilization == max(stations) (at least 1 on failure) and the position bounds. Usage: python3 -I 🐍️.py"""
import json
import pathlib

root = pathlib.Path(__file__).resolve().parent.parent
report = json.loads((root / "🧫️fixtures" / "🔬️check-report" / "🔣️.json").read_text())
seen = 0
for check in report["checks"]:
    stations = check["stations"]
    if not stations:
        continue
    seen += 1
    top = max(s["utilization"] for s in stations)
    assert abs(check["utilization"] - (max(top, 1.0) if check["status"] == "Fail" else top)) < 1e-12, check["id"]
    for s in stations:
        assert 1 <= len(s["at"]) <= 3 and s["utilization"] >= 0
        assert len(s["at"]) > 1 or 0.0 <= s["at"][0] <= 1.0
assert seen > 0
print(json.dumps({"checksWithStations": seen}))
