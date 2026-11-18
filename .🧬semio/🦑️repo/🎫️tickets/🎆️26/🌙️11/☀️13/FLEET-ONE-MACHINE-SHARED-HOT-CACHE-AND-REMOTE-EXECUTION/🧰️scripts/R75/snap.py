import json,time,glob,os,datetime
print(datetime.datetime.utcnow().strftime("%H:%M UTC"))
tot=0
for f in sorted(glob.glob("/Users/Shared/semio-store/v1/origins/*/pool.json")):
    o=f.split("/")[-2]
    try:d=json.load(open(f))
    except Exception:print(o,"unreadable");continue
    tot+=d.get("ownersPerMinute",0)
    print("%-10s %-12s ch %2d tgt %2d run %d opm %5.1f pub %4d st %4d cm %5d rec %3d age %3ds %s"%(o,d.get("reason"),d["children"],d["target"],d["running"],d["ownersPerMinute"],d["published"],d["stored"],d.get("childMegabytes",0),d["recycled"],time.time()-d["updatedMs"]/1000,d["wave"][:6] if d.get("wave") else "-"))
print("fleet opm",round(tot,1))
