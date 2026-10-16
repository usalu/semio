#!/usr/bin/env python3
# usage: census.py [crate-suffix ...|--top|--all]  -> TSV rows (name target label files bytes lines embedded verdict) of the canonical source-closure counter
import json, os, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + "/w1")
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from common import *
table = crate_table()
real = json.load(open(os.path.dirname(os.path.abspath(__file__)) + "/real.json"))
sel = [a for a in sys.argv[1:] if not a.startswith("--")] or (real if "--all" in sys.argv else [])
crates = {PKG_PREFIX + c: table[c] for c in sel}
if "--top" in sys.argv:
    crates["semio-framework-os-renderer-wgpu"] = "<top>"
env = dict(os.environ, SPLIT=SPLIT, CRATES=json.dumps(crates, ensure_ascii=False))
out = subprocess.run(["bun", os.path.dirname(os.path.abspath(__file__)) + "/census.ts"], capture_output=True, text=True, env=env, cwd=GEN)
sys.stdout.write(out.stdout)
sys.stderr.write(out.stderr[-600:])
