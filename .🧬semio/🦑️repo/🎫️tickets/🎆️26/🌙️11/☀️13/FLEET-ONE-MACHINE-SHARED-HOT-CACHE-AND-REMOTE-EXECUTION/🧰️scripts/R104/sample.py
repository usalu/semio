import json,os,sys,time,datetime
base="/Users/Shared/semio-store/v1/origins"
seconds=int(sys.argv[1]);step=int(sys.argv[2]);out=sys.argv[3]
end=time.time()+seconds
def row(o):
    try:
        d=json.load(open(f"{base}/{o}/pool.json"))
    except Exception as e:
        return {"origin":o,"error":str(e)[:40]}
    return {"origin":o,"state":d.get("state"),"wave":(d.get("wave") or "")[:8],"waves":d.get("waves"),"children":d.get("children"),"target":d.get("target"),"claimed":d.get("claimed"),"published":d.get("published"),"stored":d.get("stored"),"held":d.get("held"),"running":d.get("running"),"opm":d.get("ownersPerMinute"),"reason":d.get("reason"),"ageS":round(time.time()-d.get("updatedMs",0)/1000)}
with open(out,"a") as f:
    while True:
        now=datetime.datetime.utcnow().strftime("%H:%M:%S")
        for o in sorted(os.listdir(base)):
            if os.path.exists(f"{base}/{o}/pool.json"):
                r=row(o);r["t"]=now;f.write(json.dumps(r)+"\n");f.flush()
        if time.time()>=end:break
        time.sleep(step)
