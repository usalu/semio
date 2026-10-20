#!/usr/bin/env python3
"""Runs a Python apply tool in its read-only mode and prints every file it opened under the live root (audit hook). usage: trace-reads.py <out.json> <script> [args...]"""
import json, os, runpy, sys
sys.dont_write_bytecode = True
ROOT = "/Users/ueli/Documents/semio/"
out, script, args = sys.argv[1], sys.argv[2], sys.argv[3:]
seen = set()
def hook(event, a):
    if event == "open":
        p = a[0]
        if isinstance(p, bytes): p = p.decode("utf8", "replace")
        if isinstance(p, str):
            p = os.path.abspath(p)
            mode = a[1] or "r"
            if p.startswith(ROOT) and "/.cache/" not in p and "w" not in str(mode) and "a" not in str(mode) and "x" not in str(mode):
                seen.add(p[len(ROOT):])
sys.addaudithook(hook)
sys.argv = [script] + args
code = 0
try:
    runpy.run_path(script, run_name="__main__")
except SystemExit as error:
    code = error.code if isinstance(error.code, int) else (0 if error.code is None else 1)
json.dump(sorted(seen), open(out, "w"), ensure_ascii=False, indent=0)
print("traced", len(seen), "reads; exit", code)
