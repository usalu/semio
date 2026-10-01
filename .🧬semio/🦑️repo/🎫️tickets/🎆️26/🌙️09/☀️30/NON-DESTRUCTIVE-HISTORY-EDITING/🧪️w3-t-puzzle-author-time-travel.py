#!/usr/bin/env python3
"""⏪️ W3-T-PUZZLE authoring tool: writes the language-agnostic time-travel corpus of the puzzle 3d and puzzle 5d
selection leaves, `🧫️fixtures/🧫️selection-time-travel/🔣️.json`. Every case is a base document, a log of selection
gestures (one edit each), and ONE history edit of a gesture's inputs — its offset, angle, factors or targets. The
expected preview (the state right before the edited gesture plus the draft, nothing downstream) and the expected
replay (the fresh fold of the edited log) are folded here with the leaf mirror of `🧪️w3-t-puzzle-author-vectors.py`,
operation for operation the Rust arithmetic, so the Rust store law can require them byte for byte; the per-mutation
outcomes are the leaf outcomes of the edited log. The independent oracle (`🧪️w3-t-puzzle-oracle-selfcheck.py`) folds
the same logs with its own implementation, and the corpus is validated against its JSON Schema with `jsonschema`.
Usage: `🧪️w3-t-puzzle-author-time-travel.py 3d|5d`."""
import copy
import importlib.util
import json
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
SPEC = importlib.util.spec_from_file_location("author", os.path.join(HERE, "🧪️w3-t-puzzle-author-vectors.py"))
AUTHOR = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(AUTHOR)
QUARTER = math.pi / 2

CASES = {
    "3d": [
        ("drag-offset", "The chain root's drag is edited from (0, 0, 1) to (0, 0, 3): the followers it carries and every downstream turn and drag replay onto the higher pose.", [{"mutation": "dragSelection", "targets": ["object-a"], "offset": [0.0, 0.0, 1.0]}, {"mutation": "rotateSelection", "targets": ["object-b"], "axis": [0.0, 0.0, 1.0], "angle": QUARTER}, {"mutation": "dragSelection", "targets": ["object-d"], "offset": [1.0, 0.0, 0.0]}], 0, {"mutation": "dragSelection", "targets": ["object-a"], "offset": [0.0, 0.0, 3.0]}),
        ("drag-targets", "The chain root's drag is re-targeted to the middle link: `object-a` stays, `object-b` moves and its crossing attractions re-derive, and the downstream gestures replay on that.", [{"mutation": "dragSelection", "targets": ["object-a"], "offset": [0.0, 0.0, 1.0]}, {"mutation": "rotateSelection", "targets": ["object-b"], "axis": [0.0, 0.0, 1.0], "angle": QUARTER}, {"mutation": "dragSelection", "targets": ["object-d"], "offset": [1.0, 0.0, 0.0]}], 0, {"mutation": "dragSelection", "targets": ["object-b"], "offset": [0.0, 0.0, 1.0]}),
        ("drag-no-op", "The drag is edited to a zero offset: the edited gesture reports a Warning-level no-op and the downstream gestures replay as if it never moved anything.", [{"mutation": "dragSelection", "targets": ["object-a"], "offset": [0.0, 0.0, 1.0]}, {"mutation": "dragSelection", "targets": ["object-d"], "offset": [1.0, 0.0, 0.0]}], 0, {"mutation": "dragSelection", "targets": ["object-a"], "offset": [0.0, 0.0, 0.0]}),
        ("rotate-angle", "The chain root's quarter turn is edited to an eighth turn; the downstream drag of the middle link replays from the new pose.", [{"mutation": "rotateSelection", "targets": ["object-a"], "axis": [0.0, 0.0, 1.0], "angle": QUARTER}, {"mutation": "dragSelection", "targets": ["object-b"], "offset": [0.0, 1.0, 0.0]}], 0, {"mutation": "rotateSelection", "targets": ["object-a"], "axis": [0.0, 0.0, 1.0], "angle": QUARTER / 2}),
        ("rotate-targets", "The turn is re-targeted to the root AND the locked `object-c`: the edited gesture reports the lock as Warning-level `mutation.partial` and turns the rest.", [{"mutation": "rotateSelection", "targets": ["object-a"], "axis": [0.0, 0.0, 1.0], "angle": QUARTER}, {"mutation": "dragSelection", "targets": ["object-b"], "offset": [0.0, 1.0, 0.0]}], 0, {"mutation": "rotateSelection", "targets": ["object-a", "object-c"], "axis": [0.0, 0.0, 1.0], "angle": QUARTER}),
        ("scale-factors", "The root's scaling is edited from 2 to (0.5, 1, 1); scaling moves no pose, so only the scale replays and the downstream drag lands as before.", [{"mutation": "scaleSelection", "targets": ["object-a"], "factors": [2.0, 2.0, 2.0]}, {"mutation": "dragSelection", "targets": ["object-a"], "offset": [1.0, 0.0, 0.0]}], 0, {"mutation": "scaleSelection", "targets": ["object-a"], "factors": [0.5, 1.0, 1.0]}),
        ("scale-targets", "The scaling is re-targeted from the root to `object-d`.", [{"mutation": "scaleSelection", "targets": ["object-a"], "factors": [2.0, 2.0, 2.0]}, {"mutation": "dragSelection", "targets": ["object-a"], "offset": [1.0, 0.0, 0.0]}], 0, {"mutation": "scaleSelection", "targets": ["object-d"], "factors": [2.0, 2.0, 2.0]}),
        ("downstream-missing", "The edited drag is re-targeted to a ghost: it reports Error-level `mutation.target-missing`, which blocks finalizing, and the downstream drag still replays.", [{"mutation": "dragSelection", "targets": ["object-a"], "offset": [0.0, 0.0, 1.0]}, {"mutation": "dragSelection", "targets": ["object-d"], "offset": [1.0, 0.0, 0.0]}], 0, {"mutation": "dragSelection", "targets": ["object-ghost"], "offset": [0.0, 0.0, 1.0]}),
    ],
    "5d": [
        ("drag2d-offset", "A board drag is edited from (5, -2.5) to (10, 0); the downstream world drag replays its board pin on top.", [{"mutation": "dragSelection2d", "targets": ["part-a", "part-b"], "dx": 5.0, "dy": -2.5}, {"mutation": "dragSelection3d", "targets": ["part-a"], "offset": [1.0, 0.0, 0.0]}], 0, {"mutation": "dragSelection2d", "targets": ["part-a", "part-b"], "dx": 10.0, "dy": 0.0}),
        ("drag2d-targets", "The board drag is re-targeted to `part-b` alone.", [{"mutation": "dragSelection2d", "targets": ["part-a", "part-b"], "dx": 5.0, "dy": -2.5}, {"mutation": "dragSelection3d", "targets": ["part-a"], "offset": [1.0, 0.0, 0.0]}], 0, {"mutation": "dragSelection2d", "targets": ["part-b"], "dx": 5.0, "dy": -2.5}),
        ("drag3d-offset", "A world drag is edited from (1, 0, 0) to (0, 2, 0); its board pin moves with it and the downstream turn replays.", [{"mutation": "dragSelection3d", "targets": ["part-a", "volume-1"], "offset": [1.0, 0.0, 0.0]}, {"mutation": "rotateSelection3d", "targets": ["part-a"], "axis": [0.0, 0.0, 1.0], "angle": QUARTER}], 0, {"mutation": "dragSelection3d", "targets": ["part-a", "volume-1"], "offset": [0.0, 2.0, 0.0]}),
        ("rotate3d-angle", "A world turn is edited from a quarter to a half turn; the downstream scaling replays.", [{"mutation": "rotateSelection3d", "targets": ["part-a", "volume-1"], "axis": [0.0, 0.0, 1.0], "angle": QUARTER}, {"mutation": "scaleSelection3d", "targets": ["part-a"], "factors": [2.0, 1.0, 1.0]}], 0, {"mutation": "rotateSelection3d", "targets": ["part-a", "volume-1"], "axis": [0.0, 0.0, 1.0], "angle": math.pi}),
        ("rotate3d-targets", "The world turn is re-targeted to `part-b` and the locked `part-c`: the lock is a Warning-level `mutation.partial`.", [{"mutation": "rotateSelection3d", "targets": ["part-a", "volume-1"], "axis": [0.0, 0.0, 1.0], "angle": QUARTER}, {"mutation": "scaleSelection3d", "targets": ["part-a"], "factors": [2.0, 1.0, 1.0]}], 0, {"mutation": "rotateSelection3d", "targets": ["part-b", "part-c"], "axis": [0.0, 0.0, 1.0], "angle": QUARTER}),
        ("scale3d-factors", "A world scaling is edited from (2, 1, 1) to (1, 1, 3); the downstream drag replays.", [{"mutation": "scaleSelection3d", "targets": ["part-a", "volume-1"], "factors": [2.0, 1.0, 1.0]}, {"mutation": "dragSelection3d", "targets": ["part-a"], "offset": [0.0, 0.0, 1.0]}], 0, {"mutation": "scaleSelection3d", "targets": ["part-a", "volume-1"], "factors": [1.0, 1.0, 3.0]}),
        ("scale3d-targets", "The world scaling is re-targeted to `volume-1` alone.", [{"mutation": "scaleSelection3d", "targets": ["part-a", "volume-1"], "factors": [2.0, 1.0, 1.0]}, {"mutation": "dragSelection3d", "targets": ["part-a"], "offset": [0.0, 0.0, 1.0]}], 0, {"mutation": "scaleSelection3d", "targets": ["volume-1"], "factors": [2.0, 1.0, 1.0]}),
        ("downstream-missing", "The edited world drag is re-targeted to a ghost: Error-level `mutation.target-missing` blocks finalizing, and the downstream turn still replays.", [{"mutation": "dragSelection3d", "targets": ["part-a"], "offset": [1.0, 0.0, 0.0]}, {"mutation": "rotateSelection3d", "targets": ["part-a"], "axis": [0.0, 0.0, 1.0], "angle": QUARTER}], 0, {"mutation": "dragSelection3d", "targets": ["part-ghost"], "offset": [1.0, 0.0, 0.0]}),
    ],
}
LEVELS = ("info", "warning", "error", "fatal")


def fold(artifact, scene, payload):
    """🧮️ One leaf on `scene`: `(next scene, worst level or None, message codes in order)`."""
    result, outcome = (AUTHOR.outcome_3d if artifact == "3d" else AUTHOR.outcome_5d)(copy.deepcopy(scene), payload)
    if outcome["status"] == "rejected":
        return scene, "error", [outcome["code"]]
    messages = outcome.get("messages", [])
    worst = max((message["level"] for message in messages), key=LEVELS.index, default=None)
    return result[0], worst, [message["code"] for message in messages]


def case(artifact, base, identifier, description, log, at, replacement):
    """⏪️ One corpus case: the preview and the replay of `log` with gesture `at` edited to `replacement`."""
    preview = copy.deepcopy(base)
    for payload in log[:at]:
        preview = fold(artifact, preview, payload)[0]
    preview = fold(artifact, preview, replacement)[0]
    replayed, outcomes = copy.deepcopy(base), []
    for index, payload in enumerate(log):
        replayed, worst, codes = fold(artifact, replayed, replacement if index == at else payload)
        if index >= at:
            outcomes.append({"at": index, "worst": worst, "codes": codes})
    return {"id": identifier, "description": description, "base": base, "log": log, "edit": {"at": at, "replacement": replacement}, "preview": preview, "replayed": replayed, "outcomes": outcomes, "blocksFinalize": any(outcome["worst"] in ("error", "fatal") for outcome in outcomes)}


def main():
    artifact = sys.argv[1]
    base = AUTHOR.chain_scene_3d() if artifact == "3d" else AUTHOR.scene_5d()
    corpus = {
        "$schema": "../../🧬️schema/🔣️selection-time-travel/🔣️.json",
        "artifact": "puzzle.%s" % artifact,
        "description": "Time travel over the %s selection leaves: each case edits ONE recorded gesture's inputs; `preview` is the state right before it plus the draft (nothing downstream), `replayed` the fresh fold of the edited log, `outcomes` the per-mutation outcomes of the Report replay from the edited gesture on." % ("puzzle 3d" if artifact == "3d" else "puzzle 5d"),
        "cases": [case(artifact, base, *entry) for entry in CASES[artifact]],
    }
    path = os.path.join(AUTHOR.SUBSET[artifact], "🧫️fixtures", "🧫️selection-time-travel", "🔣️.json")
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(json.dumps(corpus, indent=2, ensure_ascii=False, sort_keys=True) + "\n")
    for entry in corpus["cases"]:
        print(entry["id"], [(outcome["at"], outcome["worst"], outcome["codes"]) for outcome in entry["outcomes"]], "blocks" if entry["blocksFinalize"] else "clean")


if __name__ == "__main__":
    main()
