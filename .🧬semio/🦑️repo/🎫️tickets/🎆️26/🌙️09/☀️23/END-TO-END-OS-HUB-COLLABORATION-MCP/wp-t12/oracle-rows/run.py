#!/usr/bin/env python3
"""🔮️ Runs the Python oracle role of every scenario in a `dump.ts` JSON through the real host `Context`, fixtures
resolved by the platform's own `resolveFixtures` (carried in the dump).
`SEMIO_ADAPTERS` points at a scratch root holding patched adapters (falls back to the repo's), `SEMIO_FIXTURE_ROOT`
at a scratch root holding patched fixtures.
Usage: run.py <scenarios.json> [--level quick]"""
import importlib.util
import json
import os
import pathlib
import re
import sys

R = pathlib.Path("/Users/ueli/Documents/semio")
A = pathlib.Path(os.environ.get("SEMIO_ADAPTERS", str(R)))
spec = importlib.util.spec_from_file_location("semio_repo_test", R / "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host/🐍️.py")
host = importlib.util.module_from_spec(spec)
sys.modules["semio_repo_test"] = host
spec.loader.exec_module(host)
URI = re.compile(r"\b(shared|local|asset|schema):\/\/([^\s\"'`,;)\]]+)")
level = sys.argv[sys.argv.index("--level") + 1] if "--level" in sys.argv else None
for number, entry in enumerate(json.load(open(sys.argv[1]))):
    case_dir = pathlib.Path(entry["file"]).parent
    source = A / case_dir / "🐍️.py" if (A / case_dir / "🐍️.py").exists() else R / case_dir / "🐍️.py"
    sys.path.insert(0, str(R / case_dir))
    host._prioritize_local_source_paths([str(R / path) for path in entry.get("localSources", [])])
    module_spec = importlib.util.spec_from_file_location("case_%d" % number, source)
    module = importlib.util.module_from_spec(module_spec)
    module_spec.loader.exec_module(module)
    adapter = module.adapter()
    owner = R / str(case_dir).split("/🧪️tests/")[0]
    counts = {}
    for scenario in entry["scenarios"]:
        handler = adapter.handler(scenario, "oracle")
        verdict = None
        if handler is None:
            verdict = "unregistered"
        elif level and scenario.get("level") != level:
            verdict = "skipped"
        else:
            texts = [step["text"] for step in scenario["steps"]] + [step.get("docString") or "" for step in scenario["steps"]] + [cell for step in scenario["steps"] for row in (step.get("dataTable") or []) for cell in row]
            fixtures = scenario.get("fixtures")
            if fixtures is None:
                fixtures = [{"uri": match.group(0), "path": str((owner / "🧫️fixtures" / match.group(2)).relative_to(R))} for text in texts for match in URI.finditer(text) if match.group(1) == "shared"]
            context = host.Context({"workDir": str(pathlib.Path(sys.argv[1]).parent / "work"), "fixtures": fixtures, "case": str(case_dir)}, scenario, "oracle", os.environ.get("SEMIO_FIXTURE_ROOT", str(R)))
            try:
                handler(context)
                verdict = "passed"
            except Exception as error:
                verdict = "failed"
                print("FAIL", scenario["id"], type(error).__name__, str(error)[:240])
        counts[verdict] = counts.get(verdict, 0) + 1
    print(case_dir.name, counts)
