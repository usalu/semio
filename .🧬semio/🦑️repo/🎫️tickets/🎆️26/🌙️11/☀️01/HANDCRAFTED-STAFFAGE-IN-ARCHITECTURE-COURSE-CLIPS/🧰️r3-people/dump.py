import json
import sys

sys.stdout.reconfigure(encoding="utf8")
a = json.load(open(sys.argv[1], encoding="utf8"))["asset"]
what = sys.argv[2]
arg = sys.argv[3] if len(sys.argv) > 3 else None
if what == "pose":
    p = [x for x in a["poses"] if x["id"] == arg][0]
    print(json.dumps(p, ensure_ascii=False, indent=1))
elif what == "anchors":
    print(json.dumps(a["anchors"], ensure_ascii=False, indent=None))
elif what == "structure":
    print(json.dumps(a["structure"], ensure_ascii=False, indent=1))
    print(json.dumps(a["dimensions"]))
elif what == "bones":
    for b in a["rig"]["bones"]:
        print({k: v for k, v in b.items() if k != "title"})
elif what == "clip":
    c = [x for x in a["clips"] if x["id"] == arg][0]
    print(json.dumps(c, ensure_ascii=False, indent=None))
elif what == "top":
    for k in ("provenance", "presentation", "exports", "tags", "summary", "title", "materials"):
        print(k, json.dumps(a[k], ensure_ascii=False)[:1500])
elif what == "decisions":
    print(json.dumps(a["representations"][arg]["decisions"], ensure_ascii=False, indent=1))
