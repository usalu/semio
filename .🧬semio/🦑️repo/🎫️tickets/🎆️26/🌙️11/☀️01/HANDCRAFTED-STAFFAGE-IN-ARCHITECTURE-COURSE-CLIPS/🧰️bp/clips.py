import json, sys
a = json.load(open(sys.argv[1], encoding="utf8"))["asset"]
for c in a.get("clips", []):
    print(c["id"], c.get("durationS"), c.get("loop"), json.dumps({k: v for k, v in c.items() if k not in ("id",)}, ensure_ascii=False)[:int(sys.argv[2])])
