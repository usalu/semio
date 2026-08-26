#!/usr/bin/env python3
"""🧪️ Offline replay of one case's Python oracle against its own committed plan, outside the runner.

Loads the repository's real Python host so the adapter sees the same `Adapter`/`Context`/`Outcome`
it will see under `oracle exhaustive`, feeds it the case plan `buildCasePlan` produced, and reports
per-scenario pass/fail. Used because no Rust subject host links today, so `parity` cannot be
measured and this is the only way to exercise the reference half.
"""
import importlib.util, json, os, sys, traceback

ROOT = "/Users/ueli/Documents/semio"
HOST = os.path.join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🐍️python/🐍️host.py")

spec = importlib.util.spec_from_file_location("semio_repo_test", HOST)
host = importlib.util.module_from_spec(spec)
sys.modules["semio_repo_test"] = host
spec.loader.exec_module(host)

case = json.load(open(sys.argv[1], encoding="utf-8"))
adapter_path = os.path.join(ROOT, case["caseDir"], "🐍️component.py")
aspec = importlib.util.spec_from_file_location("semio_test_adapter", adapter_path)
amod = importlib.util.module_from_spec(aspec)
aspec.loader.exec_module(amod)
adapter = amod.adapter()

plan = {"workDir": "/tmp", "fixtures": case["fixtures"], "owner": case["owner"], "case": case["case"]}
passed, failed, missing = 0, [], []
for scenario in case["scenarios"]:
    handler = adapter.handler(scenario["id"], "oracle")
    if handler is None:
        missing.append(scenario["id"]); continue
    try:
        outcome = handler(host.Context(plan, scenario, "oracle", ROOT))
        json.dumps(outcome.projection)
        passed += 1
    except Exception as error:
        failed.append((scenario["id"], "%s: %s" % (type(error).__name__, error)))
print("[replay] case=%s scenarios=%d passed=%d failed=%d unregistered=%d" % (case["case"], len(case["scenarios"]), passed, len(failed), len(missing)))
for sid, why in failed:
    print("  FAIL %s\n        %s" % (sid, why[:400]))
for sid in missing[:20]:
    print("  NO-HANDLER %s" % sid)
sys.exit(1 if failed or missing else 0)
