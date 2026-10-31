import json, sys
d = json.load(open(sys.argv[1]))
only = sys.argv[2:] 
for r in d:
    a = r.get("asset") or r.get("id")
    if only and a not in only:
        continue
    print("==", a)
    for i in r.get("issues", []):
        print("  ", i.get("code"), i.get("path"), (i.get("message") or "")[:230] if isinstance(i.get("message"), str) else str(i.get("message"))[:230])
