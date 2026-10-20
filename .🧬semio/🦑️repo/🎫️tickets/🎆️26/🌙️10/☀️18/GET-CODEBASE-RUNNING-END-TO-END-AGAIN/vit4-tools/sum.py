import json,sys
for l in open(sys.argv[1],encoding="utf-8"):
    r=json.loads(l); print(r["rc"],r["secs"],r["files"],"|",r["tests"],"|",r["file"][-70:])
