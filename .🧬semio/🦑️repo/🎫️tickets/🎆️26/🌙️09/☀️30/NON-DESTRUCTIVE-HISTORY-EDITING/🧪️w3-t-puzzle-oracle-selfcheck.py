#!/usr/bin/env python3
"""🔁️ W3-T-PUZZLE self-check of the independent Python oracles (`🧊️mutate-puzzle-3d-1/🐍️.py`,
`🖐️mutate-puzzle-5d-1/🐍️.py`) against EVERY committed selection-transform vector, not only the one row per kind the
feature runs: the oracle's own apply must reproduce the committed after-snapshot exactly, and its own inverse must
restore the committed before-snapshot exactly. For puzzle 3d a THIRD-PARTY oracle (numpy rotation matrices, not
quaternions) then re-places every attracted object from its attracting object and the attraction's six parameters and
requires the committed after pose for EVERY attraction touching a moved object (re-derived or used to re-place a
follower), and every attraction of the resolved chain scene — the law that resolving the document after a move never
snaps anything back.
Finally the time-travel corpus `🧫️fixtures/🧫️selection-time-travel/🔣️.json` is validated with the THIRD-PARTY `jsonschema`
(its own schema, every snapshot against the artifact snapshot schema, every payload against its leaf schema) and the
oracle folds every edited log itself: the preview and the replay must be exactly the committed ones.
Usage: `🧪️w3-t-puzzle-oracle-selfcheck.py 3d|5d`."""
import importlib.util
import json
import math
import os
import sys
import types

import jsonschema
import numpy
import referencing

PUZZLE = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts"
SUBSET = {"3d": PUZZLE + "/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any", "5d": PUZZLE + "/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any"}
ORACLE = {"3d": "🧪️tests/🧊️mutate-puzzle-3d-1/🐍️.py", "5d": "🧪️tests/🖐️mutate-puzzle-5d-1/🐍️.py"}
LEAVES = {"3d": ("✋️drag-selection", "🔄️rotate-selection", "🔍️scale-selection"), "5d": ("✋️drag-selection2d", "🚚️drag-selection3d", "🔄️rotate-selection3d", "🔍️scale-selection3d")}


def oracle(artifact):
    """🧩️ The oracle module, imported with a stand-in for the case runner it registers itself with."""
    runner = types.ModuleType("semio_repo_test")
    runner.Adapter = type("Adapter", (), {"__init__": lambda self, *args: None, "oracle": lambda self, *args: self})
    runner.Outcome = lambda *args, **kwargs: None
    sys.modules["semio_repo_test"] = runner
    spec = importlib.util.spec_from_file_location("oracle_" + artifact, os.path.join(SUBSET[artifact], ORACLE[artifact]))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


PLACEMENT_TOLERANCE = 1e-6
"""📏️ How close a re-placed pose must land. The six parameters are Euler-style angles: at a tilt of ±90° (gimbal
lock) the derivation's `asin` near ±1 resolves the angle only to about the square root of machine precision, so the
round trip derive → place is exact to ~1e-7 there and to ~1e-15 elsewhere."""


def matrix(q):
    """🧊️ The rotation matrix of the unit quaternion `[x, y, z, w]`."""
    x, y, z, w = q
    return numpy.array([[1 - 2 * (y * y + z * z), 2 * (x * y - z * w), 2 * (x * z + y * w)], [2 * (x * y + z * w), 1 - 2 * (x * x + z * z), 2 * (y * z - x * w)], [2 * (x * z - y * w), 2 * (y * z + x * w), 1 - 2 * (x * x + y * y)]])


def axis_matrix(axis, degrees):
    """🌀️ Rodrigues' rotation matrix turning `degrees` about `axis`."""
    k = numpy.asarray(axis, dtype=float) / numpy.linalg.norm(axis)
    skew = numpy.array([[0.0, -k[2], k[1]], [k[2], 0.0, -k[0]], [-k[1], k[0], 0.0]])
    angle = math.radians(degrees)
    return numpy.eye(3) + math.sin(angle) * skew + (1 - math.cos(angle)) * skew @ skew


def onto(source, target):
    """🧭️ The rotation matrix turning unit `source` onto unit `target` (the antiparallel case about a perpendicular)."""
    source, target = numpy.asarray(source, dtype=float), numpy.asarray(target, dtype=float)
    if numpy.dot(source, target) + 1.0 < 1e-6:
        perpendicular = numpy.array([-source[1], source[0], 0.0]) if abs(source[0]) > abs(source[2]) else numpy.array([0.0, -source[2], source[1]])
        return axis_matrix(perpendicular, 180.0)
    axis = numpy.cross(source, target)
    if numpy.linalg.norm(axis) < 1e-15:
        return numpy.eye(3)
    return axis_matrix(axis, math.degrees(math.atan2(numpy.linalg.norm(axis), numpy.dot(source, target))))


def placed(attracting, port_a, port_b, attraction):
    """📐️ The attracted pose `(origin, rotation matrix)` the six parameters place against `attracting`: the compose
    connection frame (gap along the attracting port, shift and rise across it), the port alignment, then rotation
    about the port, turn and tilt about the turned rise and shift axes."""
    parent = numpy.asarray(port_a.get("direction") or [0.0, 0.0, -1.0], dtype=float)
    parent = parent / numpy.linalg.norm(parent)
    child = numpy.asarray(port_b.get("direction") or [0.0, 0.0, -1.0], dtype=float)
    child = child / numpy.linalg.norm(child)
    if numpy.linalg.norm(numpy.cross(parent, -child)) < 0.01:
        align = onto([0.0, 1.0, 0.0], [0.0, 0.0, -1.0]) if abs(parent[2]) < 0.01 else axis_matrix(numpy.cross([0.0, 0.0, 1.0], parent) if numpy.linalg.norm(numpy.cross([0.0, 0.0, 1.0], parent)) >= 1e-9 else numpy.cross([1.0, 0.0, 0.0], parent), 180.0)
    else:
        align = onto(-child, parent)
    frame = onto([0.0, 1.0, 0.0], parent)
    gap_dir, shift_dir, raise_dir = frame @ [0.0, 1.0, 0.0], frame @ [1.0, 0.0, 0.0], frame @ [0.0, 0.0, 1.0]
    rotate = axis_matrix(parent, -attraction["rotation"])
    turn = axis_matrix(rotate @ raise_dir, attraction["turn"])
    tilt = axis_matrix(rotate @ shift_dir, attraction["tilt"])
    local = align.T @ rotate.T @ turn.T @ tilt.T
    offset = numpy.asarray(attracting["origin"]) + numpy.asarray(port_a["position"]) + attraction["gap"] * gap_dir + attraction["shift"] * shift_dir + attraction["rise"] * raise_dir
    return local @ offset - numpy.asarray(port_b["position"]), local @ matrix(attracting.get("orientation") or [0.0, 0.0, 0.0, 1.0])


def reproduces(document, attraction):
    """✅️ Whether `attraction`'s parameters place its attracted object exactly where `document` holds it."""
    owner = lambda full: next((record, vortex) for record in document["objects"] for vortex in record["vortices"] if "%s:%s" % (record["id"], vortex["id"]) == full)
    (attracting, port_a), (attracted, port_b) = owner(attraction["attracting"]), owner(attraction["attracted"])
    origin, rotation = placed(attracting, port_a, port_b, attraction)
    return numpy.allclose(origin, attracted["origin"], atol=PLACEMENT_TOLERANCE) and numpy.allclose(rotation, matrix(attracted.get("orientation") or [0.0, 0.0, 0.0, 1.0]), atol=PLACEMENT_TOLERANCE)


def schemas(artifact):
    """📚️ Every local JSON Schema of the artifact subset, registered under its `$id` for `$ref` resolution."""
    found = {}
    for root, _, files in os.walk(os.path.join(SUBSET[artifact], "🧬️schema")):
        for name in files:
            if name == "🔣️.json":
                document = json.load(open(os.path.join(root, name), encoding="utf-8"))
                if isinstance(document, dict) and "$id" in document:
                    found[document["$id"]] = document
    return found


def time_travel(artifact, module):
    """⏪️ The corpus checks: schema-valid throughout, and the oracle's own fold of every edited log reproduces the
    committed preview and replay exactly."""
    known = schemas(artifact)
    registry = referencing.Registry().with_resources((identifier, referencing.Resource.from_contents(document, default_specification=referencing.jsonschema.DRAFT7)) for identifier, document in known.items())
    corpus = json.load(open(os.path.join(SUBSET[artifact], "🧫️fixtures", "🧫️selection-time-travel", "🔣️.json"), encoding="utf-8"))
    by_tag = {document["properties"]["mutation"]["const"]: document for document in known.values() if document.get("properties", {}).get("mutation", {}).get("const")}
    errors = sorted(jsonschema.Draft7Validator(known["https://json.schemas.assets.semio-tech.com/s/puzzle/%s/selection-time-travel.json" % ("puzzle3d" if artifact == "3d" else "puzzle5d")], registry=registry).iter_errors(corpus), key=str)
    failures = ["corpus schema: %s" % error.message for error in errors[:5]]
    for case in corpus["cases"]:
        for payload in case["log"] + [case["edit"]["replacement"]]:
            failures += ["%s: payload %s" % (case["id"], error.message) for error in jsonschema.Draft7Validator(by_tag[payload["mutation"]], registry=registry).iter_errors(payload)]
        kind_of_tag = lambda tag: "".join("-" + character.lower() if character.isupper() else character for character in tag)
        fold = lambda scene, payload: module.apply_mutation(scene, kind_of_tag(payload["mutation"]), {key: value for key, value in payload.items() if key != "mutation"})
        at, replacement = case["edit"]["at"], case["edit"]["replacement"]
        preview = case["base"]
        for payload in case["log"][:at] + [replacement]:
            preview = fold(preview, payload)
        replayed = case["base"]
        for index, payload in enumerate(case["log"]):
            replayed = fold(replayed, replacement if index == at else payload)
        if preview != case["preview"]:
            failures.append("%s: the oracle's preview differs" % case["id"])
        if replayed != case["replayed"]:
            failures.append("%s: the oracle's replay differs" % case["id"])
    return len(corpus["cases"]), failures


def kind_of(leaf):
    return next(leaf[at:] for at, character in enumerate(leaf) if character.isascii() and character.isalpha())


def main():
    artifact = sys.argv[1]
    module = oracle(artifact)
    checked, failures, placements = 0, [], [0]
    for leaf in LEAVES[artifact]:
        root = os.path.join(SUBSET[artifact], "🧫️fixtures", "🧬️mutations", leaf)
        for case in sorted(os.listdir(root)):
            read = lambda *parts: json.load(open(os.path.join(root, case, *parts), encoding="utf-8"))
            outcome = read("🎯️outcome", "🔣️.json")
            if outcome["status"] == "rejected":
                continue
            kind = kind_of(leaf)
            before, after = read("📸️snapshot", "⬅️before", "🔣️.json"), read("📸️snapshot", "➡️after", "🔣️.json")
            payload = {key: value for key, value in read("🦠️mutation", "🔣️.json").items() if key != "mutation"}
            try:
                applied = module.apply_mutation(before, kind, payload)
                if applied != after:
                    raise AssertionError("after differs: %s" % json.dumps({member: applied[member] for member in applied if applied[member] != after.get(member)})[:600])
                current = applied
                for step_kind, step_payload in module.inverse_mutation(before, kind, payload):
                    current = module.apply_mutation(current, step_kind, step_payload)
                if current != before:
                    raise AssertionError("inverse did not restore before")
                if artifact == "3d":
                    moved = {record["id"] for record, held in zip(after["objects"], before["objects"]) if (record["origin"], record.get("orientation")) != (held["origin"], held.get("orientation"))}
                    owner = lambda full: full.split(":")[0]
                    for held in after["attractions"]:
                        if (owner(held["attracting"]) in moved or owner(held["attracted"]) in moved) and not reproduces(after, held):
                            raise AssertionError("third-party placement: %s does not reproduce its attracted pose" % held["id"])
                        placements[0] += owner(held["attracting"]) in moved or owner(held["attracted"]) in moved
                    if any(held["id"] == "attraction-ab" for held in before["attractions"]) and not all(reproduces(before, held) for held in before["attractions"]):
                        raise AssertionError("third-party placement: the chain scene is not resolved")
                checked += 1
            except AssertionError as error:
                failures.append("%s/%s: %s" % (leaf, case, error))
    cases, travel = time_travel(artifact, module)
    failures += travel
    print("checked", checked, "attractions touching a moved object placed by numpy", placements[0], "time-travel cases", cases, "failures", len(failures))
    for failure in failures:
        print("  FAIL", failure)
    sys.exit(1 if failures else 0)


if __name__ == "__main__":
    main()
