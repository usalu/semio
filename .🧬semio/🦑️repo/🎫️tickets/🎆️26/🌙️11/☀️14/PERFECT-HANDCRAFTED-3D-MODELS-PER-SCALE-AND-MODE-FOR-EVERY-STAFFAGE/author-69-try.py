"""python3 author-69-try.py <slug> '<python using V (volumes dict) >' [rules]: product check (failing tokens only, all scales) of a temporarily edited source; restores the source."""
import sys, json, subprocess
sys.path.insert(0, ".")
from author_69_lib import T
slug, expr = sys.argv[1], sys.argv[2]
rules = sys.argv[3] if len(sys.argv) > 3 else None
path = f"{T}/volumes/humans/{slug}.json"
orig = open(path).read()
try:
    v = json.loads(orig)
    V = v["volumes"]
    exec(expr)
    json.dump(v, open(path, "w"), ensure_ascii=False, separators=(",", ":"))
    import os
    cmd = os.environ["CMD"].split() if os.environ.get("CMD") else [f"{T}/author-69-chk.sh", f"humans/{slug}"] + (["--rules", rules] if rules else [])
    out = subprocess.run(cmd, capture_output=True, text=True).stdout
    print("\n".join(l[:900] for l in out.splitlines() if not l.startswith("[check]")))
finally:
    open(path, "w").write(orig)
