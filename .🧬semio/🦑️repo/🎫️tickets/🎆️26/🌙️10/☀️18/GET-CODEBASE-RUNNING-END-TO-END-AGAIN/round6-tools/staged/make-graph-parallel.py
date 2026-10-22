#!/usr/bin/env python3
"""Copy of the graph-parallel set whose op list is re-derived from the tree the chain has produced by then (ops_gen.py --root), so the owner's staged files stay untouched."""
import os, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from mk import *

T = fresh_copy(C + "/graph-parallel-tools", "graph-parallel")
run = subprocess.run([sys.executable, T + "/ops_gen.py", "--root", repo_root()], capture_output=True, text=True, cwd=T)
if run.returncode:
    raise SystemExit("maker: graph-parallel ops_gen.py failed: " + (run.stdout + run.stderr)[-800:])
print("graph-parallel copy:", T, run.stdout.strip()[-120:])
