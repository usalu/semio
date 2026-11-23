#!/usr/bin/env python3
"""🔮️ Runs all 68 forward and inverse scenarios of `🎥️mutate-shooting-1/🐍️.py` through the native Python test host."""
from __future__ import annotations

import importlib.util
import json
import os
import subprocess
import sys
import types
from pathlib import Path

TICKET_DIR = Path(__file__).resolve().parent
REPO_ROOT = TICKET_DIR.parents[6]
HOST = REPO_ROOT / "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host/🐍️.py"
SUBSET_DIR = REPO_ROOT / "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any"
ADAPTER = SUBSET_DIR / "🧪️tests/🎥️mutate-shooting-1/🐍️.py"
SHARED_ROOT = SUBSET_DIR / "🧪️tests/🧫️fixtures"


def load_adapter_vectors(adapter_path: Path):
    spec = importlib.util.spec_from_file_location("shooting_python_oracle", str(adapter_path))
    assert spec and spec.loader
    module = importlib.util.module_from_spec(spec)
    stub = types.ModuleType("semio_repo_test")

    class _Stub:
        def __init__(self, *args, **kwargs):
            pass

        def __getattr__(self, _name):
            return lambda *args, **kwargs: self

    stub.Adapter = _Stub  # type: ignore[attr-defined]
    stub.Context = _Stub  # type: ignore[attr-defined]
    stub.Outcome = _Stub  # type: ignore[attr-defined]
    sys.modules["semio_repo_test"] = stub
    spec.loader.exec_module(module)
    return module


def main() -> int:
    module = load_adapter_vectors(ADAPTER)
    inputs: list[dict[str, str]] = []
    scenarios: list[dict[str, object]] = []
    seen_uris: set[str] = set()

    def add_input(uri: str) -> None:
        if uri in seen_uris:
            return
        seen_uris.add(uri)
        assert uri.startswith("shared://"), f"unexpected fixture scheme: {uri}"
        rel_path = SHARED_ROOT / uri[len("shared://") :]
        assert rel_path.is_file(), f"missing fixture file: {rel_path}"
        inputs.append({"uri": uri, "path": os.path.relpath(rel_path, REPO_ROOT)})

    add_input(module._BASE_URI)
    for kind, (dirname, fixture, _tag) in module.VECTORS.items():
        root = f"shared://🧬️mutations/{dirname}/{fixture}"
        add_input(f"{root}/🦠️mutation/🔣️.json")
        add_input(f"{root}/📸️snapshot/➡️after/🔣️.json")
        scenarios.append({"id": f"mutate-{kind}", "level": "exhaustive", "steps": []})
        scenarios.append({"id": f"inverse-{kind}", "level": "exhaustive", "steps": []})

    work_dir = TICKET_DIR / "🗑️generated/python-oracle-work"
    artifact_dir = work_dir / "artifacts"
    work_dir.mkdir(parents=True, exist_ok=True)
    artifact_dir.mkdir(parents=True, exist_ok=True)
    plan_path = work_dir / "plan.json"
    out_path = work_dir / "results.jsonl"
    plan = {
        "workDir": str(work_dir),
        "outputDir": str(work_dir / "out"),
        "artifactDir": str(artifact_dir),
        "owner": os.path.relpath(SUBSET_DIR, REPO_ROOT),
        "case": "mutate-shooting-1",
        "implementation": "python",
        "role": "oracle",
        "target": {"artifactKind": "s.shooting.shooting", "standard": "1", "subset": "any"},
        "inputs": inputs,
        "scenarios": scenarios,
    }
    plan_path.write_text(json.dumps(plan, ensure_ascii=False), encoding="utf-8")
    result = subprocess.run(
        [sys.executable, str(HOST), "--plan", str(plan_path), "--out", str(out_path), "--adapter", str(ADAPTER)],
        cwd=str(REPO_ROOT),
        capture_output=True,
        text=True,
    )
    if result.stdout.strip():
        print(result.stdout.strip())
    if result.stderr.strip():
        print(result.stderr.strip(), file=sys.stderr)
    passed = failed = errored = 0
    for line in out_path.read_text(encoding="utf-8").splitlines():
        if not line.strip():
            continue
        rec = json.loads(line)
        if rec["status"] == "passed":
            passed += 1
        elif rec["status"] == "failed":
            failed += 1
            print(f"FAILED {rec['scenario']}: {rec.get('diagnostics')}", file=sys.stderr)
        else:
            errored += 1
            print(f"ERRORED {rec['scenario']}: {rec.get('diagnostics')}", file=sys.stderr)
    total = passed + failed + errored
    print(f"Python oracle summary: {passed}/{total} passed, {failed} failed, {errored} errored (exit={result.returncode})")
    return 0 if result.returncode == 0 and failed == 0 and errored == 0 and passed == 68 else 1


if __name__ == "__main__":
    raise SystemExit(main())
