#!/usr/bin/env python3
"""Copy of the ide-free set at the ticket root (`r6-ide-free-tools`, same depth: it derives the repository root from its location) whose plans map and plan are rebuilt from the tree the chain has produced by then (plans archived since staging, anchors moved by earlier sets)."""
import os, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from mk import *

T = fresh_copy(C + "/ide-free-tools", "r6-ide-free-tools", beside=True)
for script in ("build-plans-map.py", "build-plan.py"):
    run = subprocess.run([sys.executable, T + "/" + script], capture_output=True, text=True, cwd=T)
    if run.returncode:
        raise SystemExit("maker: ide-free %s failed: %s" % (script, (run.stdout + run.stderr)[-800:]))
    print(script, run.stdout.strip()[-200:])
print("ide-free copy:", T)
