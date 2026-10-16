#!/usr/bin/env python3
# usage: census.py [crate ...|--top]  -> TSV rows (name target label files bytes lines embedded verdict)
import json, os, subprocess, sys
from common import *
carved = json.load(open(os.path.dirname(os.path.abspath(__file__)) + "/carved.json"))
sel = [a for a in sys.argv[1:] if not a.startswith("--")] or carved
crates = {PKG_PREFIX + c: CRATES[c] for c in sel}
if "--top" in sys.argv:
    crates["semio-framework-os-renderer-wgpu"] = "<top>"
env = dict(os.environ, SPLIT=SPLIT, CRATES=json.dumps(crates, ensure_ascii=False))
out = subprocess.run(["bun", os.path.dirname(os.path.abspath(__file__)) + "/census-wp1.ts"], capture_output=True, text=True, env=env, cwd=GEN)
sys.stdout.write(out.stdout)
sys.stderr.write(out.stderr[-600:])
