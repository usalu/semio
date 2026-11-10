"""🔮️ Third-party oracle of the ISO 16757 scene laws: the plan polygons (width x, depth z) of the body box and every space envelope, their areas, overlap and the extent, computed with shapely.
Run `/home/deck/semio/.venv/bin/python -I 🔮️oracles/🐍️.py` to regenerate `🧪️tests/🧫️fixtures/🔣️.json`; the Rust law test and any other implementation read the same file."""
import json
import pathlib

from shapely.geometry import box

CASES = [
    {"id": "valve", "body": {"width": 0.15, "depth": 0.10, "height": 0.20}, "spaces": [{"id": "installation", "min": [-0.05, -0.05, -0.05], "max": [0.20, 0.25, 0.15]}, {"id": "access", "min": [0.0, 0.0, 0.0], "max": [0.3, 0.2, 0.1]}]},
    {"id": "pump", "body": {"width": 0.4, "depth": 0.25, "height": 0.35}, "spaces": [{"id": "operation", "min": [-0.1, 0.0, -0.1], "max": [0.6, 0.5, 0.4]}]},
    {"id": "wide", "body": {"width": 2.5, "depth": 0.5, "height": 1.0}, "spaces": []},
]


def plan(lo, hi):
    return box(lo[0], lo[2], hi[0], hi[2])


def expected(case):
    body = plan([0, 0, 0], [case["body"]["width"], case["body"]["height"], case["body"]["depth"]])
    spaces = [plan(s["min"], s["max"]) for s in case["spaces"]]
    boxes = [(body.bounds[0:2], body.bounds[2:4])] + [((s["min"][0], s["min"][2]), (s["max"][0], s["max"][2])) for s in case["spaces"]]
    return {
        "bodyPlanArea": body.area,
        "spacePlanAreas": [s.area for s in spaces],
        "spaceOverlapAreas": [body.intersection(s).area for s in spaces],
        "spaceVolumes": [(s["max"][0] - s["min"][0]) * (s["max"][1] - s["min"][1]) * (s["max"][2] - s["min"][2]) for s in case["spaces"]],
        "extentX": [min(b[0][0] for b in boxes), max(b[1][0] for b in boxes)],
        "extentZ": [min(b[0][1] for b in boxes), max(b[1][1] for b in boxes)],
    }


if __name__ == "__main__":
    out = pathlib.Path(__file__).resolve().parent.parent / "🧪️tests" / "🧫️fixtures" / "🔣️.json"
    out.write_text(json.dumps([{**case, "expected": expected(case)} for case in CASES], indent=2, ensure_ascii=False) + "\n", encoding="utf8")
