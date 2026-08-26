#!/usr/bin/env python3
"""🧪️ Scratch driver for the four w16 semio-structural Python oracles.

Loads one case's `🐍️component.py` outside the test host (a stub `semio_repo_test` module stands in
for the host's `Adapter`/`Context`/`Outcome`/`digest`) and exercises it directly against the
committed artifacts and specification vectors, so the implementation can be debugged without a
five-minute coordinator run. Ticket 26/08/23/END-TO-END-TESTING-REFACTOR.
"""

import hashlib
import importlib.util
import json
import os
import sys
import types

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "..", "..", "..", ".."))
SEMIO = os.path.join(REPO, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio")


def install_stub():
    module = types.ModuleType("semio_repo_test")

    class Outcome:
        def __init__(self, projection, raw=None, diagnostics=None):
            self.projection = projection
            self.raw = raw
            self.diagnostics = diagnostics or []

    class Context:
        def __init__(self, scenario):
            self.scenario = scenario

        def fixture(self, uri):
            assert uri.startswith("asset://"), uri
            return os.path.join(SEMIO, uri[len("asset://") :])

        def fixture_bytes(self, uri):
            with open(self.fixture(uri), "rb") as handle:
                return handle.read()

    class Adapter:
        def __init__(self, implementation="python"):
            self.implementation = implementation
            self.handlers = {}

        def oracle(self, scenario, handler):
            self.handlers[scenario] = handler
            return self

        def subject(self, scenario, handler):
            return self

    module.Outcome = Outcome
    module.Context = Context
    module.Adapter = Adapter
    module.digest = lambda payload: hashlib.sha256(payload or b"").hexdigest()[:32]
    sys.modules["semio_repo_test"] = module
    return module


def load(case):
    install_stub()
    path = os.path.join(SEMIO, "🧪️tests", case, "🐍️component.py")
    spec = importlib.util.spec_from_file_location("adapter_" + case.replace("-", "_"), path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def scenario(scenario_id, steps):
    return {"id": scenario_id, "name": scenario_id, "level": "exhaustive", "mode": "differential", "seed": "0", "steps": steps}


def main():
    case = sys.argv[1]
    module = load(case)
    stub = sys.modules["semio_repo_test"]
    payload = json.load(open(os.path.join(os.path.dirname(__file__), case + ".json"), encoding="utf-8"))
    failures = []

    ctx = stub.Context(scenario("identity-round-trip", [{"keyword": "Given", "text": "x"}]))
    try:
        outcome = module.identity_round_trip(ctx)
        print("identity-round-trip OK", json.dumps({key: value for key, value in outcome.projection.items() if key != "document"}))
    except Exception as error:  # noqa: BLE001
        failures.append(("identity-round-trip", error))
        print("identity-round-trip FAIL", error)

    for kind, entry in payload["rows"].items():
        steps = [{"keyword": "When", "text": "apply", "docString": json.dumps(entry)}]
        for name, handler in (("mutate", module.mutate), ("inverse", module.inverse)):
            try:
                handler(stub.Context(scenario("%s-%s" % (name, kind), steps)))
                print("%s-%s OK" % (name, kind))
            except Exception as error:  # noqa: BLE001
                failures.append(("%s-%s" % (name, kind), error))
                print("%s-%s FAIL %s" % (name, kind, error))

    for kind, paths in payload["vectors"].items():
        text = " ".join("asset://" + path for path in paths)
        steps = [{"keyword": "Given", "text": text}]
        try:
            module.spec_vector(stub.Context(scenario("spec-vector-" + kind, steps)))
            print("spec-vector-%s OK" % kind)
        except Exception as error:  # noqa: BLE001
            failures.append(("spec-vector-" + kind, error))
            print("spec-vector-%s FAIL %s" % (kind, error))

    print("\n%d failure(s)" % len(failures))
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
