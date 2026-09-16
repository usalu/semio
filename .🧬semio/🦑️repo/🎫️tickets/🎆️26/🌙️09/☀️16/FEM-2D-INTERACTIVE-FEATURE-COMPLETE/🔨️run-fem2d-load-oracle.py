#!/usr/bin/env python3
"""🔨️ Ticket driver for the fem2d LOAD subset's differential oracle.

Runs the committed Python twin (`🪆️subsets/🏋️load/🧪️tests/🏋️mutate-fem2d-1-load/🐍️.py`) through the
repository's own Python test host, over the `@id-reject` rows its feature file declares — the same
host the coordinator drives, so a green run here is a green run there. The plan is READ from the
feature: rows, fixture URIs and directory names are never retyped.

    uv run python 🔨️run-fem2d-load-oracle.py            # every @id-reject row
    uv run python 🔨️run-fem2d-load-oracle.py combination # only rows whose id contains "combination"
"""

import importlib.util
import json
import os
import re
import subprocess
import sys

sys.dont_write_bytecode = True

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "..", "..", ".."))
HOST = os.path.join(REPO, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host/🐍️.py")
CASE = os.path.join(REPO, "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🏋️mutate-fem2d-1-load")
SHARED = os.path.join(REPO, "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧫️fixtures")
WORK = os.path.join(os.path.dirname(__file__), "🗑️generated", "fem2d-load-oracle")


def load(path, name):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


def oracle_module():
    """🔮️ The committed oracle, imported with the repository's own test host bound as
    `semio_repo_test` exactly as the host itself binds it."""
    host = load(HOST, "semio_repo_test")
    sys.modules["semio_repo_test"] = host
    return load(os.path.join(CASE, "🐍️.py"), "fem2d_load_oracle")


def reject_rows():
    """📋️ The `@id-reject` Examples table of the committed feature, as `(id, dir, fixture)`."""
    with open(os.path.join(CASE, "🥒️.feature"), encoding="utf-8") as handle:
        text = handle.read()
    block = text.split("@id-reject", 1)[1]
    steps = [line.strip() for line in block.splitlines() if line.strip().startswith(("Given ", "And "))]
    uris = [token for line in steps for token in line.split() if token.startswith("shared://")]
    rows = []
    for line in block.splitlines():
        cells = [cell.strip() for cell in line.strip().strip("|").split("|")] if line.strip().startswith("|") else []
        if len(cells) == 3 and cells[0] != "id":
            rows.append(tuple(cells))
    return uris, rows


def plan_for(uris, rows):
    fixtures, scenarios = {}, []
    for identifier, directory, fixture in rows:
        expanded = [uri.replace("<dir>", directory).replace("<fixture>", fixture) for uri in uris]
        for uri in expanded:
            fixtures[uri] = os.path.join(SHARED, uri[len("shared://"):])
        scenarios.append({"id": "reject-%s" % identifier, "level": "exhaustive", "steps": [{"text": " ".join(expanded), "docString": ""}]})
    return {
        "owner": os.path.relpath(os.path.join(CASE, "..", ".."), REPO),
        "case": "🏋️mutate-fem2d-1-load",
        "implementation": "python",
        "role": "oracle",
        "workDir": WORK,
        "outputDir": os.path.join(WORK, "out"),
        "artifactDir": os.path.join(WORK, "📦️artifacts"),
        "platform": sys.platform,
        "baselineSha": "",
        "featureHash": "",
        "target": {"artifact": "s.fem.fem2d", "standard": "1", "subset": "load"},
        "fixtures": [{"uri": uri, "path": path} for uri, path in sorted(fixtures.items())],
        "scenarios": scenarios,
    }


def run(needle):
    oracle_module()
    os.makedirs(WORK, exist_ok=True)
    uris, rows = reject_rows()
    selected = [row for row in rows if needle in row[0]]
    if not selected:
        raise SystemExit("no @id-reject row matches %r" % needle)
    plan = plan_for(uris, selected)
    plan_path = os.path.join(WORK, "📋️plan.json")
    with open(plan_path, "w", encoding="utf-8") as handle:
        json.dump(plan, handle, ensure_ascii=False)
    results_path = os.path.join(WORK, "📤️results.jsonl")
    environment = dict(os.environ, PYTHONDONTWRITEBYTECODE="1")
    completed = subprocess.run([sys.executable, HOST, "--plan", plan_path, "--out", results_path, "--adapter", os.path.join(CASE, "🐍️.py")], capture_output=True, text=True, env=environment)
    if completed.stdout.strip():
        print(completed.stdout.strip())
    if completed.stderr.strip():
        print(completed.stderr.strip(), file=sys.stderr)
    passed = failed = 0
    with open(results_path, encoding="utf-8") as handle:
        for line in handle:
            result = json.loads(line)
            if result["status"] == "passed":
                passed += 1
                print("  PASS %-40s %8.1f ms" % (result["scenario"], result["durationMs"]))
            else:
                failed += 1
                print("  FAIL %-40s %s" % (result["scenario"], (result["diagnostics"] or [{}])[0].get("message", "")))
    print("[run] %d passed, %d failed (host exit %d)" % (passed, failed, completed.returncode))
    return 0 if failed == 0 and completed.returncode == 0 else 1


if __name__ == "__main__":
    sys.exit(run(sys.argv[1] if len(sys.argv) > 1 else ""))
