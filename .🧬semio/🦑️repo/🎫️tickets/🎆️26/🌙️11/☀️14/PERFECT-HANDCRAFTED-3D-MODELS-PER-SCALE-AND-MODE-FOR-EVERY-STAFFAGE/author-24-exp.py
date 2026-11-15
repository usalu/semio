"""Experiment helper: python3 author-24-exp.py <slug> <scale> <python expr on body list b> ; runs kit-thermal on the modified source and restores it."""
import sys, json, shutil, subprocess
sys.path.insert(0, ".")
from author_24_lib import *
slug, scale, expr = sys.argv[1], sys.argv[2], sys.argv[3]
path = f"{T}/volumes/humans/{slug}.json"
orig = open(path).read()
try:
    v = json.loads(orig)
    b = v["volumes"][scale]["body"]
    exec(expr)
    v["volumes"][scale]["body"] = b
    json.dump(v, open(path, "w"), ensure_ascii=False, separators=(",", ":"))
    out = run(["kit-thermal.ts", f"humans/{slug}", "--scale", scale])
    print("\n".join(l[:200] for l in out.splitlines() if l[:1] != " " and "decisions" not in l)[:3000])
finally:
    open(path, "w").write(orig)
