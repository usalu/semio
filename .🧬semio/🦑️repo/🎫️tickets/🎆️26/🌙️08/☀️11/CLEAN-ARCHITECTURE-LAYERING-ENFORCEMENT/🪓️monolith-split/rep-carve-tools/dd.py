"""🔍 Unified diff of one staged file against the base (analysis only)."""
import subprocess, sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rlib import *
for rel in sys.argv[1:]:
    rel = rel if rel.startswith("🧰") else R + rel
    a = BASE + rel; b = STAGE + rel
    if not os.path.exists(a): a = "/dev/null"
    print(subprocess.run(["diff", "-u", a, b], capture_output=True, text=True).stdout)
