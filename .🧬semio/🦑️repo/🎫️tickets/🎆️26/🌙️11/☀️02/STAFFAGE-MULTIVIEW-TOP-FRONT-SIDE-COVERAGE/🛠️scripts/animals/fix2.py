import json, math, re, sys
def dump(f, d):
    open(f, "w", encoding="utf-8", newline="\n").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")
limb = re.compile(r"^(thigh|shank|metatarsus|cannon|humerus|forearm|metacarpus|tarsus|scapula|tail|neck|lumbar|thorax|udder|hock|knee)")
compact = re.compile(r"eye|nose|nostril|muzzle|pupil|spot|cheek|horn|tooth|nail|claw|lip")
for f in sys.argv[1:]:
    doc = json.load(open(f, encoding="utf-8-sig")); a = doc["asset"]; n = 0
    for k, r in a["representations"].items():
        if r["status"] != "authored": continue
        for s in r["geometry"]["plan"]:
            if s["kind"] == "capsule" and not s.get("followTip") and limb.match(s["bone"]) and not compact.search(s["id"]):
                s["followTip"] = True; n += 1
    print(a["id"], n); dump(f, doc)
