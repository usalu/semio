"""Rewrites the touch-session fixture rows for the long press at the deadline (menu while the finger is down), the tap step and the wake deadlines. Idempotent. Run: /home/deck/semio/.venv/bin/python -I touch-session-fixture.py"""
import json, pathlib

ROOT = pathlib.Path("/home/deck/semio/🧰️framework/🔨️modules/🕹️interaction/👆️gesture/🧪️tests/🧫️fixtures/🖐️touch-session.json")
data = json.loads(ROOT.read_text(encoding="utf-8"))

data["why"] = (
    "The touch session law every viewport host obeys: raw contacts with a clock in, host steps out (the primary press is deferred until the finger has decided, a held finger is an additive press that asks for its long press at the deadline while it is still down, a decided tap says `tap`, two fingers are the camera). "
    "`deadlines` (when present) is the wake time `nextDeadlineMs` after each event, `null` when nothing waits on the clock. `orbit` rows pin the pinch-to-orbit law against the TypeScript `applyPinchToOrbit` (values produced by that function). Language-agnostic: a TypeScript or other twin replays the same rows."
)

def fix(steps):
    if steps == ["press", "release"]:
        return ["press", "release", "tap"]
    if steps == ["press", "release", "doubleTap"]:
        return ["press", "release", "tap", "doubleTap"]
    return steps

for case in data["session"]:
    case["steps"] = [fix(s) for s in case["steps"]]

def row(name, events, steps, deadlines=None, **extra):
    case = {"name": name, **extra, "events": events, "steps": steps}
    if deadlines is not None:
        case["deadlines"] = deadlines
    return case

def ev(kind, t, pid=None, x=None, y=None):
    e = {"type": kind, "t": t}
    if pid is not None:
        e["pointerId"] = pid
    if x is not None:
        e["x"] = x
        e["y"] = y
    return e

by = {c["name"]: c for c in data["session"]}
c = by["a finger held past the long press becomes an additive press, released in place it asks for the menu"]
c["name"] = "a finger held past the long press asks for the menu at the deadline while it is still down, the lift only releases"
c["steps"] = [[], [], ["press+", "longPress"], ["release+"]]
c = by["a long press that is dragged selects a rectangle and asks for no menu"]
c["name"] = "a long press that is dragged afterwards selects a rectangle, the menu was asked for at the deadline"
c["steps"] = [[], ["press+", "longPress"], ["drag+"], ["release+"]]
c = by["a long press with a jittering finger fires on the first jitter"]
c["steps"] = [[], ["press+", "longPress"], ["release+"]]
c = by["a long hold lifted before any tick still counts"]
c["steps"] = [[], ["press+", "longPress", "release+"]]

names = {c["name"] for c in data["session"]}
fresh = [
    row(
        "a still finger names its wake time and the long press fires on it",
        [ev("down", 100, 1, 50, 50), ev("tick", 549), ev("tick", 550), ev("up", 700, 1)],
        [[], [], ["press+", "longPress"], ["release+"]],
        [550, 550, None, None],
    ),
    row(
        "a finger that left its slop names no wake time",
        [ev("down", 0, 1, 0, 0), ev("move", 10, 1, 40, 0), ev("up", 20, 1)],
        [[], ["press", "drag"], ["release"]],
        [450, None, None],
    ),
    row(
        "a lift before the deadline cancels the wake",
        [ev("down", 0, 1, 10, 10), ev("up", 100, 1)],
        [[], ["press", "release", "tap"]],
        [450, None],
    ),
    row(
        "a second finger cancels the wake of the first",
        [ev("down", 0, 1, 100, 100), ev("down", 10, 2, 200, 100), ev("up", 20, 2), ev("up", 30, 1)],
        [[], [], [], ["cameraEnd"]],
        [450, None, None, None],
    ),
    row(
        "a finger on a device where a long press is off never wakes the host",
        [ev("down", 0, 1, 10, 10), ev("tick", 1000), ev("up", 1100, 1)],
        [[], [], ["press", "release", "tap"]],
        [None, None, None],
        longPress=False,
    ),
]
data["session"] += [f for f in fresh if f["name"] not in names]
ROOT.write_text(json.dumps(data, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
print("rows", len(data["session"]))
