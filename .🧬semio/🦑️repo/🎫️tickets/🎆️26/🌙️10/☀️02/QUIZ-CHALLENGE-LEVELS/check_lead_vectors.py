#!/usr/bin/env python3
"""🛫️ Checks that the regenerated fixtures parse and lists the clock-lead vectors: the instants and the learner sequences named after the lead.

Run from the repository root: ``PYTHONIOENCODING=utf-8 .venv/Scripts/python.exe <this file>``.
"""

import json

ROOT = "🧰️framework/🛍️products/❓️quiz/🧫️fixtures/"
rules = json.load(open(ROOT + "⛰️challenge-rules/🔣️.json", encoding="utf-8"))
for vector in rules["instants"]:
    print("instant", vector["id"], vector["at"], vector["floor"], vector["now"], "->", vector["expected"])
lifecycle = json.load(open(ROOT + "🧾️learner-lifecycle/🔣️.json", encoding="utf-8"))
print("learners", len(lifecycle["learners"]))
for sequence in lifecycle["learners"]:
    if "ahead" in sequence["id"] or sequence["id"] in ("the-clock", "the-last-instant"):
        print(sequence["id"], [step["expected"].get("rejection") or [event["type"] for event in step["expected"]["events"]] for step in sequence["steps"]])
