#!/usr/bin/env python3
"""📐️ Measures how far the TypeScript subject lies from the Python oracle in the last harness run of the two animation cases.

Run from the repository root after a ``parity`` run: ``.venv/Scripts/python.exe <this file>``. It reads the
projections the harness cached under ``.🧬semio/🦑️repo/⚡️cache/tests/results`` and prints, per scenario, how many
numbers were compared and the largest absolute difference — the margin under the 1e-9 of ``pets-float-v1``.
"""

import json
import os
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "..", "..", "..", "..", ".."))
RESULTS = os.path.join(ROOT, ".🧬semio", "🦑️repo", "⚡️cache", "tests", "results")
CASES = ["🎞️animation-sampling", "🪀️spring-settling"]


def numbers(oracle, subject, found):
    """🔢️ Collects every pair of numbers at the same place of two projections; anything else must be equal."""
    if isinstance(oracle, bool) or isinstance(subject, bool) or isinstance(oracle, str) or isinstance(subject, str):
        if oracle != subject:
            raise AssertionError("%r differs from %r" % (oracle, subject))
    elif isinstance(oracle, (int, float)) and isinstance(subject, (int, float)):
        found.append(abs(oracle - subject))
    elif isinstance(oracle, list) and isinstance(subject, list) and len(oracle) == len(subject):
        for left, right in zip(oracle, subject):
            numbers(left, right, found)
    elif isinstance(oracle, dict) and isinstance(subject, dict) and oracle.keys() == subject.keys():
        for key in oracle:
            numbers(oracle[key], subject[key], found)
    else:
        raise AssertionError("%r differs in shape from %r" % (oracle, subject))


def main():
    """🧾️ Prints one line per scenario."""
    for case in CASES:
        oracle_directory = next(os.path.join(RESULTS, name) for name in sorted(os.listdir(RESULTS)) if name.endswith(case + "-oracle-python"))
        subject_directory = next(os.path.join(RESULTS, name) for name in sorted(os.listdir(RESULTS)) if name.endswith(case + "-subject-typescript"))
        for name in sorted(os.listdir(oracle_directory)):
            if not name.endswith(".oracle.projection.json"):
                continue
            scenario = name[: -len(".oracle.projection.json")]
            with open(os.path.join(oracle_directory, name), "r", encoding="utf-8") as handle:
                oracle = json.load(handle)
            with open(os.path.join(subject_directory, scenario + ".subject.projection.json"), "r", encoding="utf-8") as handle:
                subject = json.load(handle)
            found = []
            numbers(oracle, subject, found)
            sys.stdout.write("%s/%s: %d numbers, largest difference %.3e\n" % (case, scenario, len(found), max(found)))


if __name__ == "__main__":
    main()
