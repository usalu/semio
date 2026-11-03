import json
import sys

d = json.load(open(sys.argv[1], encoding="utf-8"))
a = d["asset"]
print("asset keys", [k for k in a])
for k, v in a.items():
    if k in ("representations",):
        for s, r in v.items():
            print("rep", s, r.get("status"))
            g = r.get("geometry", {})
            for view, items in g.items():
                for it in items:
                    print("   ", s, view, it["id"], it.get("bone"), it.get("role"), it.get("region") or it.get("regions"), it.get("kind"))
            for kk in r:
                if kk not in ("geometry", "status", "pens", "reason", "alternatives", "limitingFeature", "validRange", "denominator", "limitingSizeM", "hatches"):
                    print("   rep-key", s, kk)
    elif k not in ("provenance", "title", "summary", "materials", "rig"):
        print("KEY", k, json.dumps(v, ensure_ascii=False)[:1800])
