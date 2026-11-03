import json, sys
sys.stdout.reconfigure(encoding="utf8")
from dump import load
a = load("🌳️vegetation", sys.argv[1])
sc = sys.argv[2]
ids = sys.argv[3:]
for s in a["representations"][sc]["geometry"]["elevation"]:
    if s["id"] in ids:
        t = dict(s)
        if "d" in t: t["d"] = t["d"][:160] + "..."
        print(json.dumps(t, ensure_ascii=False))
print(json.dumps(a["representations"][sc]["hatches"], ensure_ascii=False))
