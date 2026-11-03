#!/usr/bin/env python3
"""🎚️ Language-agnostic boundary test of the shared utilisation rule: the committed cases are checked against the Python helper `within_limit`/`limit_status` of the oracle tolerance module (the Rust twin `limit_status` reads the same cases in the compliance unit tests). Usage: python3 -I 🐍️.py"""
import importlib.util
import json
import pathlib

here = pathlib.Path(__file__).resolve().parent
cases = json.loads((here / "🔣️.json").read_text())
path = here.parents[2] / "🔮️oracles" / "🏃️harness" / "📏️tolerance" / "🐍️.py"
spec = importlib.util.spec_from_file_location("norm_limit_rule", path)
rule = importlib.util.module_from_spec(spec)
spec.loader.exec_module(rule)
assert rule.UTILIZATION_TOLERANCE == cases["tolerance"]
for case in cases["cases"]:
    assert rule.limit_status(case["utilization"]) == case["status"], case["id"]
print(json.dumps({"boundaryCases": len(cases["cases"])}))
