import json, sys
d = json.load(open(sys.argv[1], encoding="utf-8"))["asset"]
for k in ("200", "100", "50"):
    r = d["representations"][k]
    print("==", k, r["validRange"], r["pens"])
    for s in r["geometry"]["elevation"]:
        if "panel-front" in (s.get("region"), *(s.get("regions") or [])) or s["id"] in ("radiator-body",):
            print(json.dumps(s, ensure_ascii=False))
    print(json.dumps(r["decisions"], ensure_ascii=False, indent=1)[:6000])
    print(json.dumps(r["hatches"], ensure_ascii=False))
    break
