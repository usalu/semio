import os
import subprocess
import sys
import time
import unittest

RUNS = int(sys.argv[1])
TESTS = sys.argv[2:] or ["test_an_interrupt_mid_export_kills_the_process_tree_and_leaves_the_cache_untouched", "test_a_grandchild_of_the_command_dies_with_it", "test_a_failing_progress_callback_cancels_like_an_interrupt"]
LOAD = int(os.environ.get("FK_LOAD", "24"))
BURN = "while True: pass"

import importlib

module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🧪️tests.🐍️")
burners = [subprocess.Popen([sys.executable, "-c", BURN]) for _ in range(LOAD)]
failed = 0
try:
    for run in range(RUNS):
        for name in TESTS:
            suite = unittest.TestSuite([module.Cancellation(name)])
            result = unittest.TestResult()
            started = time.monotonic()
            suite.run(result)
            bad = result.failures + result.errors
            failed += bool(bad)
            print(f"run {run} {name[:40]} {'FAIL' if bad else 'ok'} {time.monotonic() - started:.1f}s", flush=True)
            for _, trace in bad:
                print(trace[-1500:], flush=True)
finally:
    for burner in burners:
        burner.kill()
print(f"RESULT runs={RUNS} tests={len(TESTS)} failures={failed}", flush=True)
