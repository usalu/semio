"""📥 Inserts the parent module's private `use` lines that a test file mentions right after its `use super::*;` (companion roots re-export the public API, so unaliased `crate::` lines are dropped).
usage: apply_imports.py <parent-file> <test-file>"""
import os, re, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from ttcedit import replace
out = subprocess.run([sys.executable, os.path.join(os.path.dirname(os.path.abspath(__file__)), "useimports.py"), sys.argv[1], sys.argv[2]], capture_output=True, text=True, check=True).stdout
lines = [l for l in out.split("\n") if l and (not l.startswith("use crate::") or " as " in l)]
if lines:
    replace(sys.argv[2], "use super::*;\n", "use super::*;\n" + "\n".join(lines) + "\n")
else:
    print("no imports for", sys.argv[2])
