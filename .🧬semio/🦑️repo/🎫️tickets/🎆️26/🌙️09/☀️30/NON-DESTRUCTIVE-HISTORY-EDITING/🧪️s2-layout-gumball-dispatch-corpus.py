"""🧫️ Independent author of the Canvas2d gumball dispatch corpus (`📐️Canvas2dHost/🧫️fixtures/🧫️gumball-dispatch`).

A second implementation of the gumball gesture algebra in numpy (hit test, model mapping, unwrapped turn angle, live
stream/commit/abort and non-live one-shot dispatch), independent of the TS overlay and its wgpu twin. It writes the
expected dispatches of every case, validates the corpus against its schema of record with `jsonschema` (draft 7, the
meta-layer schema resolved by `$id`), and with `--check` only verifies the committed corpus is current.

Usage: .venv/bin/python 🧪️s2-layout-gumball-dispatch-corpus.py [--check]
"""

import json
import pathlib
import sys

import numpy as np
from jsonschema import Draft7Validator
from referencing import Registry, Resource

REPO = pathlib.Path(__file__).resolve().parents[7]
HOST = REPO / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📐️Canvas2dHost"
CORPUS = HOST / "🧫️fixtures/🧫️gumball-dispatch/🔣️.json"
SCHEMA = HOST / "🧬️schema/🔣️gumball-dispatch/🔣️.json"
META_SCHEMA = HOST / "🧬️schema/🔣️gumball-meta/🔣️.json"

HANDLE_LENGTH = 56.0
ROTATE_RADIUS = 44.0
HIT_RADIUS = 10.0
RING_HIT_BAND = 4.0
UNIFORM_KNOB = ROTATE_RADIUS * 0.7
EPS_MOVE = 1e-9
EPS_TURN = 1e-6
TOLERANCE = 1e-9
ALL = {"moveAxes": True, "rotate": True, "scaleAxes": True, "scaleUniform": True}


def layer(pivot, ids, live, config=None, model_to_layer=None):
    gumball = {"active": True, "liveDispatch": live, "pivotLayer": list(pivot), "selectionIds": list(ids), "config": dict(config or ALL)}
    if model_to_layer is not None:
        gumball["modelToLayer"] = model_to_layer
    return {"id": "meta:gumball", "role": "meta", "gumball": gumball}


def world_to_screen(view, point):
    zoom = view["camera"]["zoom"] or 1.0
    return np.array([(point[0] - view["camera"]["x"]) * zoom + view["viewport"]["width"] * 0.5, (point[1] - view["camera"]["y"]) * zoom + view["viewport"]["height"] * 0.5])


def screen_to_world(view, point):
    zoom = view["camera"]["zoom"] or 1.0
    return np.array([(point[0] - view["viewport"]["width"] * 0.5) / zoom + view["camera"]["x"], (point[1] - view["viewport"]["height"] * 0.5) / zoom + view["camera"]["y"]])


def layer_to_model(gumball, point):
    mapping = gumball.get("modelToLayer") or {"scale": [1.0, 1.0], "offset": [0.0, 0.0]}
    return (point - np.array(mapping["offset"], dtype=float)) / np.array(mapping["scale"], dtype=float)


def knobs(pivot):
    return [
        ("scaleUniform", "scaleUniform", pivot + np.array([UNIFORM_KNOB, -UNIFORM_KNOB])),
        ("scaleY", "scaleAxes", pivot + np.array([-HANDLE_LENGTH * 0.75, HANDLE_LENGTH * 0.75])),
        ("scaleX", "scaleAxes", pivot + np.array([HANDLE_LENGTH * 0.75, HANDLE_LENGTH * 0.75])),
        ("moveY", "moveAxes", pivot + np.array([0.0, -HANDLE_LENGTH])),
        ("moveX", "moveAxes", pivot + np.array([HANDLE_LENGTH, 0.0])),
    ]


def handle_at(case, point):
    gumball = case["layer"]["gumball"]
    if not gumball["active"]:
        return None
    pivot = world_to_screen(case, gumball["pivotLayer"])
    for kind, flag, centre in knobs(pivot):
        if gumball["config"][flag] and np.hypot(*(point - centre)) <= HIT_RADIUS:
            return kind
    if gumball["config"]["rotate"] and abs(np.hypot(*(point - pivot)) - ROTATE_RADIUS) <= RING_HIT_BAND:
        return "rotate"
    return None


def verb(kind):
    return "translateSelection" if kind in ("moveX", "moveY") else "rotateSelection" if kind == "rotate" else "scaleSelection"


def wrap(angle):
    return float(angle - 2.0 * np.pi * np.round(angle / (2.0 * np.pi)))


def total(case, gesture, point):
    gumball, ids = case["layer"]["gumball"], gesture["ids"]
    pivot = world_to_screen(case, gumball["pivotLayer"])
    kind, start = gesture["kind"], gesture["start"]
    if kind in ("moveX", "moveY"):
        before, after = layer_to_model(gumball, screen_to_world(case, start)), layer_to_model(gumball, screen_to_world(case, point))
        dx = float(after[0] - before[0]) if kind == "moveX" else 0.0
        dy = float(after[1] - before[1]) if kind == "moveY" else 0.0
        if abs(dx) < EPS_MOVE and abs(dy) < EPS_MOVE:
            return None
        return {"action": "translateSelection", "args": {"ids": ids, "dx": dx, "dy": dy, "dz": 0}}
    if kind == "rotate":
        raw = float(np.arctan2(point[1] - pivot[1], point[0] - pivot[0]) - np.arctan2(start[1] - pivot[1], start[0] - pivot[0]))
        previous = gesture["total"]["args"]["angle"] if gesture["total"] else 0.0
        angle = previous + wrap(raw - previous)
        if abs(angle) < EPS_TURN:
            return None
        return {"action": "rotateSelection", "args": {"ids": ids, "ax": 0, "ay": 0, "az": 1, "angle": angle}}
    reach0, reach1 = float(np.hypot(*(start - pivot))), float(np.hypot(*(point - pivot)))
    ratio = reach1 / reach0 if reach0 > 1e-6 else 1.0
    if abs(ratio - 1.0) < EPS_TURN:
        return None
    sx, sy = (ratio, ratio) if kind == "scaleUniform" else (ratio, 1.0) if kind == "scaleX" else (1.0, ratio)
    return {"action": "scaleSelection", "args": {"ids": ids, "sx": sx, "sy": sy, "sz": 1}}


def increment(previous, current):
    if previous is None or previous["action"] != current["action"]:
        return current
    a, b = previous["args"], current["args"]
    if current["action"] == "translateSelection":
        dx, dy = b["dx"] - a["dx"], b["dy"] - a["dy"]
        return None if abs(dx) < EPS_MOVE and abs(dy) < EPS_MOVE else {"action": current["action"], "args": {**b, "dx": dx, "dy": dy}}
    if current["action"] == "rotateSelection":
        angle = b["angle"] - a["angle"]
        return None if abs(angle) < EPS_TURN else {"action": current["action"], "args": {**b, "angle": angle}}
    sx, sy = b["sx"] / a["sx"], b["sy"] / a["sy"]
    return None if abs(sx - 1.0) < EPS_TURN and abs(sy - 1.0) < EPS_TURN else {"action": current["action"], "args": {**b, "sx": sx, "sy": sy}}


def identity(kind, ids):
    action = verb(kind)
    if action == "translateSelection":
        return {"action": action, "args": {"ids": ids, "dx": 0.0, "dy": 0.0, "dz": 0}}
    if action == "rotateSelection":
        return {"action": action, "args": {"ids": ids, "ax": 0, "ay": 0, "az": 1, "angle": 0.0}}
    return {"action": action, "args": {"ids": ids, "sx": 1.0, "sy": 1.0, "sz": 1}}


def phased(payload, phase):
    return {"action": payload["action"], "args": {**payload["args"], "phase": phase}}


def replay(case):
    press = np.array([case["press"]["x"], case["press"]["y"]], dtype=float)
    kind = handle_at(case, press)
    dispatches = []
    if kind is None:
        return None, dispatches
    gumball = case["layer"]["gumball"]
    gesture = {"kind": kind, "start": press, "ids": list(gumball["selectionIds"]), "total": None, "streamed": False}
    live = gumball.get("liveDispatch", False)
    for event in case["events"]:
        if event["kind"] == "cancel":
            if live and gesture["streamed"]:
                dispatches.append({"action": verb(kind), "args": {"ids": gesture["ids"], "phase": "abort", "reason": event["reason"]}})
            return kind, dispatches
        point = np.array([event["x"], event["y"]], dtype=float)
        current = total(case, gesture, point)
        if event["kind"] == "move":
            if current is None:
                continue
            if live:
                step = increment(gesture["total"], current)
                if step is None:
                    continue
                dispatches.append(phased(step, "stream"))
                gesture["streamed"] = True
            gesture["total"] = current
            continue
        if live:
            tail = increment(gesture["total"], current) if current else None
            if gesture["streamed"] or tail:
                dispatches.append(phased(tail or identity(kind, gesture["ids"]), "commit"))
        elif current:
            dispatches.append(current)
        return kind, dispatches
    return kind, dispatches


def point(x, y):
    return {"x": x, "y": y}


def move(x, y):
    return {"kind": "move", "x": x, "y": y}


def release(x, y):
    return {"kind": "release", "x": x, "y": y}


def cancel(reason):
    return {"kind": "cancel", "reason": reason}


VIEW = {"viewport": {"width": 800, "height": 600}, "camera": {"x": 0, "y": 0, "zoom": 1}}
PIVOT = (100, 50)
P = (500, 350)
FEM = {"scale": [20, -20], "offset": [40, 40]}


def case(name, gumball, press, events, view=None):
    return {"name": name, **(view or VIEW), "layer": gumball, "press": press, "events": events}


CASES = [
    case("a live x drag streams every move as its increment and commits the release tail", layer(PIVOT, ["frame-1"], True), point(P[0] + 56, P[1]), [move(570, 350), move(580, 360), release(590, 355)]),
    case("a live y drag on a zoomed, panned camera divides the screen motion by the zoom", layer((-20, 10), ["frame-1", "frame-2"], True), point(400 + (-20 - 5) * 2, 300 + (10 - 7) * 2 - 56), [move(350, 230), move(352, 210), release(352, 190)], view={"viewport": {"width": 800, "height": 600}, "camera": {"x": 5, "y": 7, "zoom": 2}}),
    case("a live drag through a scaled, flipped model map reports model units", layer((80, 120), ["n1"], True, model_to_layer=FEM), point(400 + 80 + 56, 300 + 120), [move(556, 420), release(576, 420)]),
    case("a live turn streams angle increments about the pivot", layer(PIVOT, ["frame-1"], True), point(P[0] + 44, P[1]), [move(P[0] + 40, P[1] + 18), move(P[0] + 20, P[1] + 39), release(P[0], P[1] + 44)]),
    case("a live turn past the far side keeps its angle continuous instead of jumping a full turn", layer(PIVOT, ["frame-1"], True), point(P[0] + 44, P[1]), [move(P[0], P[1] - 44), move(P[0] - 44, P[1] - 1), move(P[0] - 44, P[1] + 1), move(P[0], P[1] + 44), release(P[0] + 30, P[1] + 32)]),
    case("a live uniform scaling multiplies its increments to the release factor", layer(PIVOT, ["frame-1"], True), point(P[0] + UNIFORM_KNOB, P[1] - UNIFORM_KNOB), [move(P[0] + 45, P[1] - 45), move(P[0] + 61.6, P[1] - 61.6), release(P[0] + 15.4, P[1] - 15.4)]),
    case("a live x scaling changes only sx", layer(PIVOT, ["frame-1"], True), point(P[0] + 42, P[1] + 42), [move(P[0] + 84, P[1] + 84), release(P[0] + 63, P[1] + 63)]),
    case("a live y scaling changes only sy", layer(PIVOT, ["frame-1"], True), point(P[0] - 42, P[1] + 42), [move(P[0] - 21, P[1] + 21), release(P[0] - 21, P[1] + 21)]),
    case("a live press released where it began dispatches nothing", layer(PIVOT, ["frame-1"], True), point(P[0] + 56, P[1]), [release(P[0] + 56, P[1])]),
    case("a live release without a streamed move commits its whole motion as the tail", layer(PIVOT, ["frame-1"], True), point(P[0] + 56, P[1]), [release(P[0] + 76, P[1] + 9)]),
    case("a live drag returning to its press point streams nothing for that move and commits the identity", layer(PIVOT, ["frame-1"], True), point(P[0] + 56, P[1]), [move(P[0] + 66, P[1]), move(P[0] + 56, P[1]), move(P[0] + 61, P[1]), release(P[0] + 61, P[1])]),
    case("a live cancel after a streamed move aborts the open transaction", layer(PIVOT, ["frame-1"], True), point(P[0] + 56, P[1]), [move(P[0] + 70, P[1]), cancel("captureLost")]),
    case("a live window blur after a streamed turn aborts with its reason", layer(PIVOT, ["frame-1"], True), point(P[0] + 44, P[1]), [move(P[0] + 30, P[1] + 32), cancel("blur")]),
    case("a live cancel before any streamed move dispatches nothing", layer(PIVOT, ["frame-1"], True), point(P[0] + 56, P[1]), [cancel("captureLost")]),
    case("a non-live drag previews locally and dispatches ONE one-shot pose delta on release", layer(PIVOT, ["frame-1"], False), point(P[0] + 56, P[1]), [move(570, 350), move(580, 360), release(590, 355)]),
    case("a non-live turn past the far side releases one continuous angle", layer(PIVOT, ["frame-1"], False), point(P[0] + 44, P[1]), [move(P[0], P[1] - 44), move(P[0] - 44, P[1] - 1), move(P[0] - 44, P[1] + 1), release(P[0], P[1] + 44)]),
    case("a non-live cancel dispatches nothing", layer(PIVOT, ["frame-1"], False), point(P[0] + 42, P[1] + 42), [move(P[0] + 84, P[1] + 84), cancel("captureLost")]),
    case("a non-live release where the press began dispatches nothing", layer(PIVOT, ["frame-1"], False), point(P[0] + 56, P[1]), [move(P[0] + 80, P[1]), release(P[0] + 56, P[1])]),
    case("the uniform knob sits on the turn ring and wins the press", layer(PIVOT, ["frame-1"], True), point(P[0] + 31, P[1] - 30), [release(P[0] + 31, P[1] - 30)]),
    case("a press on the ring band between knobs turns", layer(PIVOT, ["frame-1"], True), point(P[0] - 20, P[1] - 40), [release(P[0] - 20, P[1] - 40)]),
    case("a press beside every handle hits nothing and dispatches nothing", layer(PIVOT, ["frame-1"], True), point(P[0] + 20, P[1] + 5), [move(P[0] + 60, P[1] + 5), release(P[0] + 80, P[1] + 5)]),
    case("a disabled turn ring hits nothing", layer(PIVOT, ["frame-1"], True, config={**ALL, "rotate": False}), point(P[0], P[1] + 44), [move(P[0] + 44, P[1]), release(P[0] + 44, P[1])]),
    case("disabled move axes leave their knobs dead", layer(PIVOT, ["frame-1"], True, config={**ALL, "moveAxes": False}), point(P[0] + 56, P[1]), [release(P[0] + 80, P[1])]),
]


def corpus():
    cases = []
    for entry in CASES:
        handle, dispatches = replay(entry)
        cases.append({**entry, "expect": {"handle": handle, "dispatches": dispatches}})
    return {"tolerance": TOLERANCE, "cases": cases}


def validate(document):
    meta = json.loads(META_SCHEMA.read_text(encoding="utf-8"))
    schema = json.loads(SCHEMA.read_text(encoding="utf-8"))
    registry = Registry().with_resource(meta["$id"], Resource.from_contents(meta))
    validator = Draft7Validator(schema, registry=registry)
    errors = sorted(validator.iter_errors(document), key=lambda error: list(error.path))
    if errors:
        raise SystemExit("corpus invalid: " + "; ".join(f"{list(error.path)}: {error.message}" for error in errors[:5]))
    broken = json.loads(json.dumps(document))
    del broken["cases"][0]["layer"]["gumball"]["pivotLayer"]
    if not list(validator.iter_errors(broken)):
        raise SystemExit("the schema accepts a gumball without its pivot")
    stray = json.loads(json.dumps(document))
    stray["cases"][0]["layer"]["gumball"]["space"] = "fem2d"
    if not list(validator.iter_errors(stray)):
        raise SystemExit("the schema accepts an undeclared gumball field")


def render(document):
    return json.dumps(document, ensure_ascii=False, indent=2) + "\n"


def main():
    document = corpus()
    validate(document)
    text = render(document)
    handles = sum(1 for entry in document["cases"] if entry["expect"]["handle"])
    dispatched = sum(len(entry["expect"]["dispatches"]) for entry in document["cases"])
    if "--check" in sys.argv:
        current = CORPUS.read_text(encoding="utf-8") if CORPUS.exists() else ""
        if current != text:
            raise SystemExit(f"{CORPUS} is stale: re-run without --check")
        print(f"corpus current: {len(document['cases'])} cases, {handles} pressed handles, {dispatched} dispatches")
        return
    CORPUS.parent.mkdir(parents=True, exist_ok=True)
    CORPUS.write_text(text, encoding="utf-8")
    print(f"wrote {CORPUS.relative_to(REPO)}: {len(document['cases'])} cases, {handles} pressed handles, {dispatched} dispatches")


if __name__ == "__main__":
    main()
