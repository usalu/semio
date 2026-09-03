#!/usr/bin/env python3
"""🔍️ E1 helper: standalone runner for `mutate-shooting-1/🐍️.py`'s shape — one SHARED before
fixture plus per-kind mutation/after fixtures (`VECTORS[kind] = (dir, fixture, wire_tag)`), unlike
layout's per-kind-triad shape. Builds a hand-built plan.json and runs it through the repository's own
dependency-free python host, exactly as `🔍️e1-run-python-oracle.py` does for the simpler shape.
"""
from __future__ import annotations

import importlib.util
import json
import os
import subprocess
import sys
import types

REPO_ROOT = "/Users/ueli/Documents/semio"
HOST = os.path.join(REPO_ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🐍️python/🐍️.py")


def load_module(adapter_path: str):
    spec = importlib.util.spec_from_file_location("e1_target_adapter", adapter_path)
    module = importlib.util.module_from_spec(spec)
    stub = types.ModuleType("semio_repo_test")

    class _Stub:
        def __init__(self, *a, **k):
            pass

        def __getattr__(self, name):
            return lambda *a, **k: self

    stub.Adapter = _Stub
    stub.Context = _Stub
    stub.Outcome = _Stub
    sys.modules["semio_repo_test"] = stub
    spec.loader.exec_module(module)
    return module


def main(argv):
    owner_dir, adapter_path, case_id, out_path = argv[1:5]
    module = load_module(adapter_path)
    vectors = module.VECTORS
    base_uri = module._BASE_URI

    fixtures = []
    scenarios = []

    def add_fixture(uri):
        rel = uri[len("asset://"):]
        path = os.path.join(owner_dir, rel)
        assert os.path.isfile(path), f"missing fixture on disk: {path}"
        fixtures.append({"uri": uri, "path": os.path.relpath(path, REPO_ROOT)})

    add_fixture(base_uri)
    for kind, (dirname, fixture, _tag) in vectors.items():
        root = f"asset://🧬️schema/🧬️mutations/{dirname}/🧪️tests/{fixture}"
        add_fixture(f"{root}/🦠️mutation/🔣️.json")
        add_fixture(f"{root}/📸️snapshot/➡️after/🔣️.json")
        scenarios.append({"id": f"mutate-{kind}", "level": "exhaustive"})
        scenarios.append({"id": f"inverse-{kind}", "level": "exhaustive"})

    work_dir = os.path.join(REPO_ROOT, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION/🗑️generated", f"e1-{case_id}-work")
    output_dir = os.path.join(work_dir, "out")
    os.makedirs(work_dir, exist_ok=True)
    os.makedirs(output_dir, exist_ok=True)

    plan = {
        "workDir": work_dir,
        "outputDir": output_dir,
        "owner": os.path.relpath(owner_dir, REPO_ROOT),
        "case": case_id,
        "implementation": "python",
        "role": "oracle",
        "baselineSha": "",
        "featureHash": "",
        "platform": "",
        "fixtures": fixtures,
        "scenarios": scenarios,
    }
    plan_path = os.path.join(work_dir, "plan.json")
    with open(plan_path, "w", encoding="utf-8") as f:
        json.dump(plan, f, ensure_ascii=False)

    result = subprocess.run(
        [sys.executable, HOST, "--plan", plan_path, "--out", out_path, "--adapter", adapter_path],
        cwd=REPO_ROOT,
        capture_output=True,
        text=True,
    )
    print(result.stdout)
    print(result.stderr, file=sys.stderr)
    print(f"host exit code: {result.returncode}")

    passed = failed = errored = 0
    with open(out_path, encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            rec = json.loads(line)
            status = rec["status"]
            if status == "passed":
                passed += 1
            elif status == "failed":
                failed += 1
                print(f"FAILED {rec['scenario']}: {rec['diagnostics']}")
            else:
                errored += 1
                print(f"ERRORED {rec['scenario']}: {rec['diagnostics']}")
    total = passed + failed + errored
    print(f"summary: {passed}/{total} passed, {failed} failed, {errored} errored")
    return 0 if result.returncode == 0 else 1


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
