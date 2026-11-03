import json, sys
d = json.load(open(sys.argv[1], encoding="utf-8"))
a = d["asset"]
for s, r in a["representations"].items():
    for view, items in r.get("geometry", {}).items():
        for it in items:
            if it.get("bone") == "wall" or it["id"] in ("valve-head",):
                x = {k: v for k, v in it.items() if k not in ("id", "title")}
                print(s, view, it["id"], json.dumps(x, ensure_ascii=False))
