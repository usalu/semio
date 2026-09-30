#!/usr/bin/env python3
"""🐍️ W2-S-E: runs every registered handler of the layout second implementation (`📐️mutate-layout-1/🐍️.py`) standalone
against the committed camelCase fixture vectors — the repository host's `Adapter`/`Context`/`Outcome`, a plan whose fixture
table maps `shared://🧬️mutations/…` onto the subset's `🧫️fixtures/🧬️mutations/…`. Every mutate and inverse scenario must pass.

Run: .venv/bin/python 🧪️w2-s-e-layout-oracle.py
"""
import importlib.util
import os
import sys
import tempfile

REPO = "/Users/ueli/Documents/semio"
SUBSET = f"{REPO}/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any"
sys.path.insert(0, f"{REPO}/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host")


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


host = load("semio_repo_test", f"{REPO}/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host/🐍️.py")
oracle = load("layout_oracle", f"{SUBSET}/🧪️tests/📐️mutate-layout-1/🐍️.py")


def main():
    fixtures = []
    for directory, _names, files in os.walk(f"{SUBSET}/🧫️fixtures/🧬️mutations"):
        if "🔣️.json" in files:
            relative = os.path.relpath(directory, f"{SUBSET}/🧫️fixtures/🧬️mutations")
            fixtures.append({"uri": f"shared://🧬️mutations/{relative}/🔣️.json", "path": os.path.relpath(os.path.join(directory, "🔣️.json"), REPO)})
    adapter = oracle.adapter()
    passed, failed = 0, []
    with tempfile.TemporaryDirectory() as work:
        for kind in oracle.VECTORS:
            for scenario_id in (f"mutate-{kind}", f"inverse-{kind}"):
                scenario = {"id": scenario_id, "steps": []}
                context = host.Context({"workDir": work, "fixtures": fixtures, "case": "mutate-layout-1"}, scenario, "oracle", REPO)
                try:
                    adapter.handler(scenario, "oracle")(context)
                    passed += 1
                except Exception as error:
                    failed.append((scenario_id, str(error)[:300]))
    for scenario_id, error in failed:
        print(f"[w2-s-e] FAIL {scenario_id}: {error}")
    print(f"[w2-s-e] layout second implementation: {passed} passed, {len(failed)} failed")
    raise SystemExit(1 if failed else 0)


if __name__ == "__main__":
    main()
