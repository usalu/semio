import json,sys
d=json.load(open(sys.argv[1],encoding="utf8"))
for k,v in d["result"].items():
    p=d["pinned"][k]
    for f,h in v["sha256"].items():
        print(k,f,"SAME" if p["sha256"].get(f)==h else f"OLD {p['sha256'].get(f)} NEW {h}")
