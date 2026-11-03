import json, sys
d = json.load(open(sys.argv[1], encoding="utf-8"))["asset"]
what = sys.argv[2]
if what == "region":
    for r in d["thermal"]["regions"]:
        if r["id"] in sys.argv[3:]:
            print(json.dumps(r, ensure_ascii=False, indent=1))
elif what == "pose":
    for p in d["poses"]:
        if p["id"] in sys.argv[3:]:
            print(json.dumps(p, ensure_ascii=False, indent=1))
elif what == "clips":
    print(json.dumps(d["clips"], ensure_ascii=False)[:3500])
elif what == "rep":
    r = d["representations"][sys.argv[3]]
    print(json.dumps({k: v for k, v in r.items() if k != "geometry"}, ensure_ascii=False, indent=1))
elif what == "misc":
    print(json.dumps({k: d[k] for k in ("structure", "exports", "completion", "presentation", "anchors")}, ensure_ascii=False, indent=1))
    print(json.dumps(d["thermal"]["emitters"], ensure_ascii=False, indent=1))
    print(json.dumps(d["thermal"]["assumptions"], ensure_ascii=False))
