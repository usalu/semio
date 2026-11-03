import json, sys
for f in sys.argv[1:]:
    a = json.load(open(f, encoding="utf-8"))["asset"]
    for k, r in a["representations"].items():
        if r["status"] == "unavailable":
            print(a["id"], k, r.get("cause"), r["limitingSizeM"], "|", r["limitingFeature"]["en"][:90], "|", r["reason"]["en"][:120], r["alternatives"])
