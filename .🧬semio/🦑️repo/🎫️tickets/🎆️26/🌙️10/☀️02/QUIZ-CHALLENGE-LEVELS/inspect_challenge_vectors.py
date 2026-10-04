#!/usr/bin/env python3
"""🔎️ Prints digests of the regenerated challenge vectors, for a human look at what the reference committed.

Run from the repository root: ``.venv/Scripts/python.exe <this file>``.
"""

import json
import os

FIXTURES = os.path.join("🧰️framework", "🛍️products", "❓️quiz", "🧫️fixtures")


def load(case):
    """📂️ One fixture."""
    with open(os.path.join(FIXTURES, case, "🔣️.json"), encoding="utf-8") as handle:
        return json.load(handle)


lifecycle = load("🧾️learner-lifecycle")
for sequence in lifecycle["learners"][-5:]:
    print("[DEBUG] sequence", sequence["id"])
    for step in sequence["steps"]:
        expected = step["expected"]
        print("[DEBUG]   ", step["command"]["type"], step["command"].get("challenge", ""), step["command"].get("task", ""), expected.get("rejection") or [(event["type"], event.get("at")) for event in expected["events"]])
    views = sequence["views"]["expected"]
    print("[DEBUG]   best", views["learner"]["best"], "total", views["learner"]["total"])
    for run, view in views["runs"].items():
        print("[DEBUG]   run", run[:6], view["status"], view["sheet"]["challenge"], "hints" in view and view["hints"], view.get("opened"), view.get("result", {}).get("score"), view.get("result", {}).get("points"))
board = load("🏆️leaderboard")
vector = next(vector for vector in board["vectors"] if vector["id"] == "points-across-challenges")
for identifier, asked in vector["expected"]["leaderboards"].items():
    rows = asked["nobody"]["rows"]
    print("[DEBUG] board", identifier, [(row["rank"], row["total"], {quiz: (best["challenge"], best["points"]) for quiz, best in row["best"].items()}, row["reachedAt"] - 1790000000000) for row in rows])
crowd = load("📊️crowd-view")
for vector in crowd["vectors"][-4:]:
    print("[DEBUG] crowd", vector["id"], vector["expected"]["scores"])
    for task in vector["expected"]["tasks"]:
        print("[DEBUG]   ", task["task"], task.get("dimension", ""), task["scores"], [(item["item"], item.get("counts") or item.get("places")) for item in task["items"]])
site = lifecycle["site"]
for play in site["plays"][:5]:
    print("[DEBUG] site", play["id"], play["runs"][0]["challenge"], play["expected"])
