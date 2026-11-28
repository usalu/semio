import json,sys,time,os
out=sys.argv[1];n=int(sys.argv[2]);p=os.path.expanduser("~/Library/Caches/semio/host-capacity/warden/state.json")
with open(out,"w") as f:
    for _ in range(n):
        try:
            d=json.load(open(p));h=d["host"];f.write(json.dumps({"atMs":d["atMs"],"level":d["level"],"trigger":d.get("trigger"),**h})+"\n");f.flush()
        except Exception: pass
        time.sleep(1)
