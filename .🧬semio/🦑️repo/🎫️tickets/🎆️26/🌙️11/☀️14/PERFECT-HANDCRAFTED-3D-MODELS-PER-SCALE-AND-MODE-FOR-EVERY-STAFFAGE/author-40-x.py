"""python3 author-40-x.py <slug> <scale> <list> '<python on b>' <rule> : product check of a modified list, restores the source."""
import sys, json
sys.path.insert(0, ".")
from author_24_lib import *
slug, scale, lst, expr, rule = sys.argv[1:6]
path = f"{T}/volumes/humans/{slug}.json"
orig = open(path).read()
try:
    v = json.loads(orig)
    b = v["volumes"][scale][lst]
    exec(expr)
    v["volumes"][scale][lst] = b
    json.dump(v, open(path, "w"), ensure_ascii=False, separators=(",", ":"))
    out = run(["volumes-check.ts", f"humans/{slug}", "--scale", scale, "--product", "--rules", rule])
    print("\n".join(l[:600] for l in out.splitlines() if l.startswith(("ok", "FAIL"))))
finally:
    open(path, "w").write(orig)
