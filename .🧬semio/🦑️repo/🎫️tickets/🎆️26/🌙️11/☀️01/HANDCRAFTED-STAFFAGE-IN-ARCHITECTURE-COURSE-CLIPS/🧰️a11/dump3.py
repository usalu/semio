import json, sys
d = json.load(open(sys.argv[1], encoding="utf-8"))["asset"]
for k in ("100", "50"):
    r = d["representations"][k]
    print("==", k, r["validRange"], r["pens"], r["hatches"])
    for s in r["geometry"]["elevation"]:
        if "panel-front" in (s.get("region"), *(s.get("regions") or [])):
            print(json.dumps(s, ensure_ascii=False))
    for key in ("silhouette", "merged", "omitted", "overlays", "retained"):
        print(key, json.dumps(r["decisions"][key], ensure_ascii=False))
