#!/usr/bin/env python3
"""R85: one snapshot of a worker's readiness from the store (default g1-m05): status, containment, last timings, the holder lines and the last log lines of the stages. Usage: snap.py [origin]"""
import json,sys,datetime,os,re
O=sys.argv[1] if len(sys.argv)>1 else "g1-m05";D="/Users/Shared/semio-store/v1/origins/"+O+"/"
now=datetime.datetime.now(datetime.timezone.utc)
def stamp(ms):return datetime.datetime.fromtimestamp(ms/1000,datetime.timezone.utc).strftime("%H:%M:%S")
try:
    d=json.load(open(D+"ready.json"))
    print(now.strftime("%H:%M:%S"),"UTC ready.json updated",stamp(d["updatedMs"]),"state",d["state"],"stage",d["stage"],"attempts",d["attempts"],"ready",d["ready"])
    print(" reason:",(d.get("reason") or {}).get("code"),"| refusal:",(d.get("refusal") or {}).get("code"))
    if d.get("containment"):print(" containment:",d["containment"]["en"])
    print(" tail:",[t[:140] for t in d.get("tail",[])][-2:])
except Exception as e:print("ready.json",e)
try:
    for r in json.load(open(D+"ready-timings.json"))["runs"][-5:]:print(" run",r["stage"],stamp(r["startedMs"]),stamp(r["endedMs"]),r["outcome"],"exit",r["exit"],"attempts",r["attempts"])
except Exception as e:print("timings",e)
for n in ("tools","workspace"):
    f=D+"ready-log-%s.txt"%n
    if os.path.exists(f):
        lines=open(f,errors="replace").read().splitlines()
        hold=[l for l in lines if "holds the dependency store" in l or "Stopped orphan" in l or "Verwaisten" in l]
        print(" log",n,"mtime",stamp(os.path.getmtime(f)*1000),"lines",len(lines),"| holder/reap lines:",[h[:170] for h in hold[-2:]])
        print("   last:",[l[:150] for l in lines[-2:]])
