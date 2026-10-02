#!/usr/bin/env python3
"""🔎️ Prints a one-line-per-vector summary of the identity, lifecycle and leaderboard vectors of the quiz product.

Run from the repository root: ``.venv/Scripts/python.exe <this file>``. Read-only; a quick way to review what the
generator committed after a change of the references.
"""

import json
import os

ROOT = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "..", "..", "..", "..", ".."))
FIXTURES = os.path.join(ROOT, "🧰️framework", "🛍️products", "❓️quiz", "🧫️fixtures")


def fixture(name):
    """📄️ One parsed fixture."""
    with open(os.path.join(FIXTURES, name, "🔣️.json"), "r", encoding="utf-8") as handle:
        return json.load(handle)


def outcome(decision):
    """⚖️ A decision as its rejection or its event types."""
    return decision.get("rejection") or "+".join(event["type"] for event in decision["events"]) or "nothing"


def main():
    """🖨️ Prints the summaries."""
    identity = fixture("🪪️identity-shapes")
    for vector in identity["handles"]:
        print("handle %-28s %s" % (vector["id"], "refused" if vector["expected"] is None else ascii(vector["expected"]["display"]) + " key " + ascii(vector["expected"]["key"])))
    print("alphabet", identity["alphabet"]["members"], len(identity["alphabet"]["folds"]), "folds")
    for vector in identity["shapes"]:
        print("shape %-30s %s" % (vector["id"], vector["expected"]))
    lifecycle = fixture("🧾️learner-lifecycle")
    for sequence in lifecycle["registrations"]:
        print("registration %-26s %s" % (sequence["id"], [outcome(step["expected"]) for step in sequence["steps"]]))
    for vector in lifecycle["quotas"]:
        print("quota %-16s %s" % (vector["id"], vector["expected"]))
    for sequence in lifecycle["learners"]:
        print("learner %-36s %s" % (sequence["id"], [outcome(step["expected"]) for step in sequence["steps"]]))
    for scenario in lifecycle["site"]["plays"]:
        print("site %-44s %s" % (scenario["id"], scenario["expected"]["awards"]))
    board = fixture("🏆️leaderboard")
    for vector in board["vectors"]:
        for caller, answer in vector["expected"]["leaderboards"].items():
            print("ranking %-26s %-10s rows %d learners %d own %s" % (vector["id"], caller, len(answer["rows"]), answer["learners"], answer.get("own", {}).get("rank")))
    for vector in board["boards"]:
        for caller, answer in vector["expected"].items():
            print("board %-20s %-10s rows %d learners %d own %s" % (vector["id"], caller, len(answer["rows"]), answer["learners"], answer.get("own", {}).get("rank")))


if __name__ == "__main__":
    main()
