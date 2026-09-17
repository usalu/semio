"""🎯️ Slice 2F — runs both target-region oracles over the committed fixture corpus without nx.

    .venv/bin/python3 '.../🐍️2F-region-oracle-probe.py'

Half one replays every committed `target-region` vector through the artifact's own Python SECOND
IMPLEMENTATION (`🧪️tests/◻️mutate-puzzle-2d-1/🐍️.py`): forward, the committed after-snapshot member
by member, and the full inverse law. Half two runs the shapely THIRD-PARTY oracle
(`🧪️tests/🕸️third-party-puzzle-2d-1/🐍️.py`'s `region-containment` scenario) over the same vectors.
Both halves import the adapters by path behind a `semio_repo_test` shim, so neither needs the test
host, a plan file or a compiled subject.
"""

import importlib.util
import json
import os
import sys
import types

ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any"
FIX = os.path.join(ROOT, "🧫️fixtures/🧬️mutations")

shim = types.ModuleType("semio_repo_test")


class Adapter:
    def __init__(self, name):
        self.name = name

    def oracle(self, *args, **kwargs):
        return self


class Outcome:
    def __init__(self, payload, raw=None):
        self.payload, self.raw = payload, raw


shim.Adapter, shim.Outcome = Adapter, Outcome
sys.modules["semio_repo_test"] = shim


def load(relative, name):
    spec = importlib.util.spec_from_file_location(name, os.path.join(ROOT, relative))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


VECTORS = [
    ("create-target-region", "🌍create-target-region/🌍️appends-region-2", "applied"),
    ("create-target-region", "🌍create-target-region/🌍️paints-a-tower-footprint", "applied"),
    ("create-target-region", "🌍create-target-region/🚫️rejects-a-region-id-the-board-already-holds", "refused"),
    ("delete-target-region", "🪦delete-target-region/🪦️removes-region-1", "applied"),
    ("delete-target-region", "🪦delete-target-region/🚫️rejects-deleting-a-region-the-board-never-held", "refused"),
    ("move-target-region", "🚀move-target-region/🚀️slides-region-1", "applied"),
    ("move-target-region", "🚀move-target-region/🚫️rejects-moving-a-region-the-board-never-held", "refused"),
    ("resize-target-region", "📐resize-target-region/📐️widens-region-1", "applied"),
    ("resize-target-region", "📐resize-target-region/🚫️rejects-resizing-a-region-the-board-never-held", "refused"),
    ("edit-target-region-label", "🖋️edit-target-region-label/🖋️renames-region-1", "applied"),
    ("edit-target-region-label", "🖋️edit-target-region-label/🚫️rejects-renaming-a-region-the-board-never-held", "refused"),
    ("change-target-region-hidden", "🙈change-target-region-hidden/🙈️hides-region-1", "applied"),
    ("change-target-region-hidden", "🙈change-target-region-hidden/🚫️rejects-hiding-a-region-the-board-never-held", "refused"),
    ("change-target-region-locked", "🔏change-target-region-locked/🔏️locks-region-1", "applied"),
    ("change-target-region-locked", "🔏change-target-region-locked/🚫️rejects-locking-a-region-the-board-never-held", "refused"),
]


def leaf(vector, *parts):
    return json.load(open(os.path.join(FIX, vector, *parts), encoding="utf-8"))


def second_implementation():
    reference = load("🧪️tests/◻️mutate-puzzle-2d-1/🐍️.py", "reference")
    failures = 0
    for kind, vector, verdict in VECTORS:
        try:
            before = leaf(vector, "📸️snapshot/⬅️before/🔣️.json")
            after = leaf(vector, "📸️snapshot/➡️after/🔣️.json")
            outcome = leaf(vector, "🎯️outcome/🔣️.json")
            payload = reference.payload_of(leaf(vector, "🦠️mutation/🔣️.json"), kind)
            reference.validate(before, "probe-%s" % kind)
            reference.validate(after, "probe-%s" % kind)
            if verdict == "refused":
                assert outcome["status"] == "rejected", outcome
                sentinel = os.path.join(FIX, vector, "🔺️diff/🚫️.absent")
                assert os.path.exists(sentinel) and os.path.getsize(sentinel) == 0, "contract D6 sentinel"
                try:
                    reference.apply_mutation(before, kind, payload)
                except AssertionError:
                    reference.equals_committed(kind, before, after)
                    print("  refused  %s" % vector)
                    continue
                raise AssertionError("accepted a vector the committed outcome declares rejected")
            applied = reference.apply_mutation(before, kind, payload)
            reference.validate(applied, "probe-%s" % kind)
            reference.equals_committed(kind, applied, after)
            reference.observable(kind, before, applied, reference.declares_no_op(outcome))
            current = applied
            for step_kind, step_payload in reference.inverse_mutation(before, kind, payload):
                current = reference.apply_mutation(current, step_kind, step_payload)
            reference.restores(kind, current, before)
            print("  applied  %s" % vector)
        except Exception as error:
            failures += 1
            print("  FAILED   %s\n           %s" % (vector, error))
    print("\n  %d/%d vectors agree with the python second implementation" % (len(VECTORS) - failures, len(VECTORS)))
    print("  KINDS = %d  SPEC_VECTORS = %d" % (len(reference.KINDS), len(reference.SPEC_VECTORS)))
    return failures


class Ctx:
    def fixture(self, uri):
        return os.path.join(FIX, "🔣️.json")


def third_party():
    adapter = load("🧪️tests/🕸️third-party-puzzle-2d-1/🐍️.py", "third_party")
    answer = adapter.region_containment(Ctx()).payload
    print("  scenario %r: %d checks over %d vectors" % (answer["scenario"], answer["checked"], len(answer["vectors"])))
    return 0


if __name__ == "__main__":
    print("🐍️ second implementation (mutate-puzzle-2d-1)")
    broken = second_implementation()
    print("\n🕸️ third-party shapely oracle (third-party-puzzle-2d-1 / region-containment)")
    broken += third_party()
    sys.exit(1 if broken else 0)
