#!/usr/bin/env python3
"""🔍️ Ticket tool: holds the TypeScript subject's projections of the gesture case to the Python oracle's, as `parity` will once the Rust twin exists.

Run from the repository root after `oracle` and `subject --implementation typescript` of the repo test harness for
``--case "👆️gesture-recognition"``: ``.venv/Scripts/python.exe <this file>``. It reads the newest projections the
harness left under ``.🧬semio/🦑️repo/⚡️cache/tests/results`` and compares them scenario by scenario under the rule
of the profile ``pets-float-v1`` (structural identity, numbers within 1e-9), printing one line per scenario.
"""

import glob
import json
import math
import os
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "..", "..", "..", "..", ".."))
RESULTS = os.path.join(ROOT, ".🧬semio", "🦑️repo", "⚡️cache", "tests", "results")


def same(left, right, where):
    """⚖️ The first place two projections differ, or ``None``."""
    if isinstance(left, bool) or isinstance(right, bool):
        return None if left == right else where
    if isinstance(left, (int, float)) and isinstance(right, (int, float)):
        return None if math.isclose(left, right, rel_tol=0, abs_tol=1e-9) else where
    if isinstance(left, list) and isinstance(right, list):
        if len(left) != len(right):
            return where + " (lengths %d and %d)" % (len(left), len(right))
        return next((found for index, (one, other) in enumerate(zip(left, right)) for found in [same(one, other, "%s/%d" % (where, index))] if found), None)
    if isinstance(left, dict) and isinstance(right, dict):
        if list(left) != list(right):
            return where + " (keys)"
        return next((found for key in left for found in [same(left[key], right[key], "%s/%s" % (where, key))] if found), None)
    return None if left == right else where


if __name__ == "__main__":
    oracle = max(glob.glob(os.path.join(RESULTS, "*👆️gesture-recognition-oracle-python")), key=os.path.getmtime)
    subject = max(glob.glob(os.path.join(RESULTS, "*👆️gesture-recognition-subject-typescript")), key=os.path.getmtime)
    failures = 0
    for path in sorted(glob.glob(os.path.join(oracle, "*.oracle.projection.json"))):
        scenario = os.path.basename(path)[: -len(".oracle.projection.json")]
        other = os.path.join(subject, scenario + ".subject.projection.json")
        if not os.path.exists(other):
            sys.stdout.buffer.write(("%-14s no subject projection\n" % scenario).encode("utf-8"))
            failures += 1
            continue
        with open(path, encoding="utf-8") as left, open(other, encoding="utf-8") as right:
            difference = same(json.load(left), json.load(right), scenario)
        failures += 1 if difference else 0
        sys.stdout.buffer.write(("%-14s %s\n" % (scenario, "equal" if difference is None else "differs at " + difference)).encode("utf-8"))
    sys.exit(1 if failures else 0)
