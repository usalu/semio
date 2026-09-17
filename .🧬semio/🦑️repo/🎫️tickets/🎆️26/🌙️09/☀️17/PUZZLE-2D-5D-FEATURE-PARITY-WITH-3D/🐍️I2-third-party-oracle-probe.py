"""🕸️ Integrator I2 — runs EVERY scenario of the 2d python third-party oracle over the whole
committed fixture corpus, without nx and without the test host.

    .venv/bin/python3 '.../🐍️I2-third-party-oracle-probe.py'

Slice 2F reported that `vectors()` discovered ZERO vectors because `SCENARIOS_DIR`/`LEAF_SCHEMA`
still named the pre-relocation layout. This probe is what proves the repaired discovery really walks
the corpus: it prints the vector count each scenario saw, so a silent green is impossible.
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


class Ctx:
    def fixture(self, uri):
        return os.path.join(FIX, "🔣️.json")


def main():
    adapter = load("🧪️tests/🕸️third-party-puzzle-2d-1/🐍️.py", "third_party")
    ctx = Ctx()
    discovered = adapter.vectors(ctx)
    regions = [vector for vector in discovered if "target-region" in vector["kind"]]
    print("discovered %d committed vectors (%d of them target-region) over %d leaves" % (len(discovered), len(regions), len({vector["id"].split("/")[0] for vector in discovered})))
    if not regions:
        raise AssertionError("the 15 target-region vectors were not discovered")
    failures = 0
    for scenario, handler in (
        ("graph-cascade", adapter.graph_cascade),
        ("kind-compatibility", adapter.kind_compatibility),
        ("geometry-transforms", adapter.geometry_transforms),
        ("payload-schemas", adapter.payload_schemas),
        ("diff-reproduction", adapter.diff_reproduction),
        ("region-containment", adapter.region_containment),
    ):
        try:
            answer = handler(ctx).payload
            print("  PASS %-22s %5d checks over %4d vectors" % (answer["scenario"], answer["checked"], len(answer["vectors"])))
        except Exception as error:
            failures += 1
            print("  FAIL %-22s %s" % (scenario, str(error)[:4000]))
    print("\n%d/%d scenarios agree" % (6 - failures, 6))
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
