"""Parses Rust files with rustfmt (single file, no module resolution) and reports syntax errors; usage: parsecheck.py <file>... | --all-tests"""
import subprocess, sys
from lib import *
files = [a for a in sys.argv[1:] if not a.startswith("--")]
if "--all-tests" in sys.argv:
    files = [REPO + "/" + p for p in test_sources() if "/🧪️tests/" in p or "worker-cell" in p or "outcome" in p or "sqlite" in p]
bad = 0
for f in files:
    p = subprocess.run(["rustfmt", "--edition", "2021", "--emit", "stdout", "--config", "skip_children=true", f], capture_output=True, text=True)
    err = [l for l in p.stderr.split("\n") if l.startswith("error")]
    if err:
        bad += 1
        print("PARSE ERROR", f[-70:], err[:2])
print("checked", len(files), "files,", bad, "with errors")
