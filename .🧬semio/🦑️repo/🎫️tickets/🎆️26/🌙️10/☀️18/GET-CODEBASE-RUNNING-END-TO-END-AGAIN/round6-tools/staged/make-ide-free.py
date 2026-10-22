#!/usr/bin/env python3
"""Copy of the ide-free set at the ticket root (`r6-ide-free-tools`, same depth: it derives the repository root from its location) whose plans map and plan are rebuilt from the tree the chain has produced by then (plans archived since staging, anchors moved by earlier sets)."""
import os, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from mk import *

T = fresh_copy(C + "/ide-free-tools", "r6-ide-free-tools", beside=True)
patch(T + "/blib.py", '        raise SystemExit("JEDIT changed nothing: " + path)\n', '        print("JEDIT: already clean, nothing to write: " + path)\n        return\n')
patch(T + "/blib.py", '    if n not in counts:\n        raise SystemExit(f"anchor count {n} not in {counts} for {path}: {old[:90]!r}")\n    _work[path] = text.replace(old, new)\n', '    if n == 0 and counts == (1,):\n        print(f"ED: anchor absent, a peer already removed it: {path[-60:]} {old[:60]!r}")\n        return\n    if n not in counts:\n        raise SystemExit(f"anchor count {n} not in {counts} for {path}: {old[:90]!r}")\n    _work[path] = text.replace(old, new)\n')
for script, modules in (("build-plans-map.py", []), ("build-plan.py", ["s_plans", "s_leftovers", "s_code", "s_guard", "s_agent_client"])):
    run = subprocess.run([sys.executable, T + "/" + script, *modules], capture_output=True, text=True, cwd=T)
    if run.returncode:
        raise SystemExit("maker: ide-free %s failed: %s" % (script, (run.stdout + run.stderr)[-800:]))
    print(script, run.stdout.strip()[-200:])
print("ide-free copy:", T)
