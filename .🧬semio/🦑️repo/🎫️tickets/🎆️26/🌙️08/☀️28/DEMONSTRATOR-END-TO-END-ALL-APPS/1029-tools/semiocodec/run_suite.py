#!/usr/bin/env python3
"""Local harness: runs one mutate-semio-* Python adapter's identity handlers against the committed fixtures with a stub of the repo test host.

usage: run_suite.py <path to the suite 🐍️.py> [handler-substring ...]
"""
import importlib.util
import json
import pathlib
import sys
import traceback

sys.path.insert(0, str(pathlib.Path(__file__).parent))
from run_suite_stub import Context  # noqa: E402  (registers the semio_repo_test stub)

suite = pathlib.Path(sys.argv[1]).resolve()
subset = suite.parents[2]
needles = sys.argv[2:] or ["identity", "round", "carrier"]
spec = importlib.util.spec_from_file_location("suite_under_test", suite)
loaded = importlib.util.module_from_spec(spec)
sys.modules["suite_under_test"] = loaded
spec.loader.exec_module(loaded)
adapter = loaded.adapter()
failed = 0
ran = 0
for key, handler in adapter._handlers.items():
    if not any(needle in key for needle in needles):
        continue
    ran += 1
    try:
        feature = next(iter(suite.parent.glob("*.feature")), None)
        steps = feature.read_text(encoding="utf-8").split("\n") if feature else []
        outcome = handler(Context(subset, key.split("::")[0], steps))
        print("PASS", key, json.dumps(outcome.projection, ensure_ascii=False, default=str)[:160])
    except Exception:
        failed += 1
        print("FAIL", key)
        traceback.print_exc(limit=3)
print("handlers run: %d, failed: %d" % (ran, failed))
sys.exit(1 if failed or not ran else 0)
