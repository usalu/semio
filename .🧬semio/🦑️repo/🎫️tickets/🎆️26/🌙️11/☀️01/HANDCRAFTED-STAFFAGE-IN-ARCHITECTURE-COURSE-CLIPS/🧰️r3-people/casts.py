import json
import sys

sys.stdout.reconfigure(encoding="utf8")
V = sys.argv[1]
S = sys.argv[2]
d = json.load(open(V, encoding="utf8"))
cat = json.load(open(S, encoding="utf8"))
kinds = {e["id"]: e["category"] for e in cat["entries"]}
for k, v in d["casts"].items():
    if kinds.get(v["asset"]) in ("humans", "animals"):
        print(k, v["asset"], v.get("pose") or "", v.get("clip") or "", v.get("view"), v.get("viewpoint"), sep="|")
