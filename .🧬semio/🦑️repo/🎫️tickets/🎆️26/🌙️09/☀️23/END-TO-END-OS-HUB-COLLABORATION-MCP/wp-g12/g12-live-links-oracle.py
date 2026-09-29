#!/usr/bin/env python3
"""🔗️ Independent oracle for the os-mcp hub-live-links fixture (`🏠️workspace/🧫️fixtures/🔗️hub-live-links.json`):
re-derives every case's expectation with Python's heapq (least recently used first) and max(), never the Rust code."""
import heapq
import json
import pathlib
import sys

FIXTURE = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧫️fixtures/🔗️hub-live-links.json")


def closes(live, opening, route, limit):
    own = [[artifact, "relinked"] for artifact, *_ in live if artifact == opening]
    same = [[artifact, "superseded"] for artifact, plugin, app, _ in live if artifact != opening and [plugin, app] == route]
    others = [(used, artifact) for artifact, plugin, app, used in live if artifact != opening and [plugin, app] != route]
    heapq.heapify(others)
    bounded = []
    while len(others) + 1 > limit:
        bounded.append([heapq.heappop(others)[1], "bounded"])
    return own + same + bounded


def session(bound):
    hub = [(used, artifact) for artifact, is_hub, used in bound if is_hub]
    if hub:
        return max(hub)[1]
    folder = [artifact for artifact, is_hub, _ in bound if not is_hub]
    return folder[0] if len(folder) == 1 else None


def main() -> int:
    fixture = json.loads(FIXTURE.read_text(encoding="utf-8"))
    failures = 0
    for case in fixture["close"]:
        got = closes(case["live"], case["opening"], case["route"], case.get("limit", fixture["limit"]))
        if got != case["expect"]:
            failures += 1
            print(f"FAIL close {case['name']}: oracle {got} fixture {case['expect']}")
    for case in fixture["session"]:
        got = session(case["bound"])
        if got != case["expect"]:
            failures += 1
            print(f"FAIL session {case['name']}: oracle {got} fixture {case['expect']}")
    total = len(fixture["close"]) + len(fixture["session"])
    print(f"hub-live-links oracle: {total - failures}/{total} cases agree")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
