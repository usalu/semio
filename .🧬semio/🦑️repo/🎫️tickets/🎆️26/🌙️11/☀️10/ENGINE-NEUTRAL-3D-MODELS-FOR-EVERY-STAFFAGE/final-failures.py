import json, sys
r = json.load(open(sys.argv[1], encoding="utf8"))
flt = sys.argv[2] if len(sys.argv) > 2 else ""
lim = int(sys.argv[3]) if len(sys.argv) > 3 else 1500
for f in r["testResults"]:
    if flt not in f["name"]: continue
    if f["status"] == "failed" and not f["assertionResults"]:
        print("FILE", f["name"].split("/🧪️tests/")[-1], f.get("message", "")[:lim])
    for t in f["assertionResults"]:
        if t["status"] == "failed":
            print("TEST", f["name"].split("/🧪️tests/")[-1], "|", t["fullName"][:200])
            print("   ", "\n    ".join(m[:lim] for m in t["failureMessages"][:1]))
