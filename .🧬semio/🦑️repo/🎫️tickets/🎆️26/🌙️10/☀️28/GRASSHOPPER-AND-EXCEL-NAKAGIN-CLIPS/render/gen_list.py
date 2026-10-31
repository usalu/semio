import json,sys
sys.stdout.reconfigure(encoding="utf-8")
m=json.load(open(sys.argv[1],encoding="utf-8"))
for v in m["videos"]:
    if v["id"].startswith(("bim_","optimization_")):
        s=v["chapters"][0]["segments"][0]
        print(s["script"],s["scene"],v["output"])
