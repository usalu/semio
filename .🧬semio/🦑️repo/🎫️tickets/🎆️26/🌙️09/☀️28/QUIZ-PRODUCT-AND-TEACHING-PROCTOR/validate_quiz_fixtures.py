#!/usr/bin/env python3
"""🧬️ Validates every schema-typed document of the quiz product's shared vectors with python-jsonschema.

Run from the repository root: ``.venv/Scripts/python.exe <this file>``. The rows are read from the
table of ``🧪️tests/🧬️schema-conformance/🥒️.feature``, so this report and the conformance oracle judge
exactly the same documents. Prints one Markdown row per table row and exits non-zero on any violation.
"""

import importlib.util
import json
import os
import sys

sys.dont_write_bytecode = True
ROOT =os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "..", "..", "..", "..", ".."))
QUIZ = os.path.join(ROOT, "🧰️framework", "🛍️products", "❓️quiz")
CASE = os.path.join(QUIZ, "🧪️tests", "🧬️schema-conformance")
HOST = os.path.join(ROOT, "🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "🧪️test", "🖥️host", "🐍️.py")


def load(path, name):
    """📦️ Loads one Python file as a module."""
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


sys.modules["semio_repo_test"] = load(HOST, "semio_repo_test")
CONFORMANCE = load(os.path.join(CASE, "🐍️.py"), "conformance")


def table():
    """📋️ The feature's data table as dictionaries."""
    rows = []
    with open(os.path.join(CASE, "🥒️.feature"), "r", encoding="utf-8") as handle:
        for line in handle:
            if line.strip().startswith("|"):
                rows.append([cell.strip() for cell in line.strip().strip("|").split("|")])
    return [dict(zip(rows[0], row)) for row in rows[1:]]


def fixture(uri):
    """🧫️ The parsed fixture a ``shared://`` URI names."""
    with open(os.path.join(QUIZ, "🧫️fixtures", uri[len("shared://") :]), "r", encoding="utf-8") as handle:
        return json.load(handle)


failures = 0
covered = set()
print("| row | fixture | definition | documents | violations |")
print("|---|---|---|---|---|")
for row in table():
    covered.add(row["fixture"])
    documents = CONFORMANCE.matches(fixture(row["fixture"]), row["pointer"])
    broken = [(path, CONFORMANCE.violations(row["definition"], document)) for path, document in documents]
    broken = [(path, lines) for path, lines in broken if lines]
    failures += len(broken) + (0 if documents else 1)
    print("| %s | %s | %s | %d | %s |" % (row["id"], row["fixture"][len("shared://") :], row["definition"], len(documents), "; ".join("%s %s" % (path, lines) for path, lines in broken) or "0"))
rejected = fixture("shared://🧬️schema-conformance/🔣️.json")["rejected"]
for vector in rejected:
    lines = CONFORMANCE.violations(vector["definition"], vector["document"])
    if not any(line.endswith(": " + vector["violates"]) for line in lines):
        failures += 1
        print("| rejected/%s | expected %s | %s |" % (vector["id"], vector["violates"], lines))
print("| rejected | 🧬️schema-conformance/🔣️.json | Quiz/Catalog | %d | each breaks its named rule |" % len(rejected))
GENERATED = ["✅️answer-validation", "🃏️sheet-assembly", "🏅️badge-rules", "🏆️leaderboard", "📏️sorting-concordance", "🔀️matching-concordance", "🕸️profile-similarity", "🧾️learner-lifecycle"]
uncovered = sorted(name for name in GENERATED if "shared://%s/🔣️.json" % name not in covered)
print("\n[DEBUG] uncovered fixtures: %s" % (uncovered or "none"))
print("[DEBUG] failures: %d" % failures)
sys.exit(1 if failures or uncovered else 0)
