"""🔎 Prints rig, poses, clips and a per-scale summary of the solids of components/external-venetian-blind."""
import json, sys
from collections import Counter
F = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/external-venetian-blind/🔣️.json"
a = json.load(open(F))["asset"]
mode = sys.argv[1] if len(sys.argv) > 1 else "summary"
if mode == "summary":
    print(list(a.keys()))
    for k in ("dimensions", "summary", "notes"):
        print(k, json.dumps(a.get(k), ensure_ascii=False)[:1500])
    print("rig", json.dumps(a["rig"], ensure_ascii=False)[:5000])
    for p in a["poses"]:
        print("pose", json.dumps(p, ensure_ascii=False)[:500])
    for c in a.get("clips", []):
        print("clip", json.dumps(c, ensure_ascii=False)[:400])
    print("pres", json.dumps(a["presentation"], ensure_ascii=False)[:400])
    for s, v in a["volumes"].items():
        print("==", s, {k: (len(x) if isinstance(x, list) else type(x).__name__) for k, x in v.items()})
        for l in ("body", "thermal", "structure"):
            x = v.get(l)
            if isinstance(x, list):
                print(" ", l, Counter((e.get("kind"), e.get("bone")) for e in x).most_common(14))
                print("   ids", [e.get("id") for e in x][:30])
            else:
                print(" ", l, json.dumps(x, ensure_ascii=False)[:800])
else:
    s, l = sys.argv[1], sys.argv[2]
    pat = sys.argv[3] if len(sys.argv) > 3 else ""
    x = a["volumes"][s][l]
    x = x if isinstance(x, list) else [x]
    n = 0
    for e in x:
        if pat in json.dumps(e.get("id", "")):
            print(json.dumps(e, ensure_ascii=False)[:int(sys.argv[4]) if len(sys.argv) > 4 else 700])
            n += 1
            if n > 40: break
