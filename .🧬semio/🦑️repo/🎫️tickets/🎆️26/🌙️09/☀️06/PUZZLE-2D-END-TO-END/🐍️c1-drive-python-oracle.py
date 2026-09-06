"""🔧 Standalone driver: stubs the repo test host facade and runs the puzzle-2d third-party adapter."""
import importlib.util
import json
import os
import sys
import traceback
import types

# 🚫️No `__pycache__` beside the adapter: a test case directory admits only its feature file and
# its taxonomy-declared adapters, and a stray cache directory is a `unknown-case-child` breach.
sys.dont_write_bytecode = True

REPO = "/Users/ueli/Documents/semio"
OWNER = "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any"


class Outcome:
    def __init__(self, projection, raw=None, diagnostics=None, artifacts=None, production_dispatch=None):
        self.projection = projection
        self.raw = raw


class Adapter:
    def __init__(self, implementation="python"):
        self.implementation = implementation
        self._h = {}

    def oracle(self, scenario, handler):
        self._h[scenario + "::oracle"] = handler
        return self

    def subject(self, scenario, handler):
        self._h[scenario + "::subject"] = handler
        return self

    def handler(self, scenario, role):
        return self._h.get(scenario + "::" + role)


class Context:
    def __init__(self):
        self.repo_root = REPO
        self.scenario = {"id": "driver", "steps": []}

    def fixture(self, uri):
        return os.path.join(REPO, OWNER, uri.split("://", 1)[1])


stub = types.ModuleType("semio_repo_test")
stub.Adapter = Adapter
stub.Outcome = Outcome
stub.Context = Context
sys.modules["semio_repo_test"] = stub

path = sys.argv[1]
spec = importlib.util.spec_from_file_location("adapter_under_test", path)
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
built = module.adapter()

failed = 0
for scenario in ["graph-cascade", "kind-compatibility", "geometry-transforms", "payload-schemas", "diff-reproduction"]:
    handler = built.handler(scenario, "oracle")
    if handler is None:
        continue
    try:
        outcome = handler(Context())
        projection = outcome.projection
        print("PASS %-22s checked=%s vectors=%s" % (scenario, projection.get("checked"), len(projection.get("vectors", []))))
    except AssertionError as error:
        failed += 1
        print("FAIL %-22s\n%s" % (scenario, error))
    except Exception:
        failed += 1
        print("ERROR %-22s\n%s" % (scenario, traceback.format_exc()))
print("driver: %d scenario(s) failed" % failed)
sys.exit(1 if failed else 0)
