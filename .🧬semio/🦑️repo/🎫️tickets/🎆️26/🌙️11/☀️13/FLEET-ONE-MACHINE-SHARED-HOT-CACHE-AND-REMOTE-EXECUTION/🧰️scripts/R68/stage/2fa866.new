#!/usr/bin/env python3
"""Independent oracle of the ownership walk reports: replays the steps of the fixture against the policy of the schema and prints, per step, whether a report and a checkpoint are due and the numbers of the report. Usage: oracle.py <schema.json> <fixture.json>"""
import json,sys,math
schema=json.load(open(sys.argv[1],encoding="utf8"));fixture=json.load(open(sys.argv[2],encoding="utf8"))
p=schema["definitions"]["Policy"]["const"]
def rnd(x):return math.floor(x+0.5)
start=0;rep_ms=0;rep_n=0;chk_ms=0;chk_n=0;out=[]
for s in fixture["steps"]:
    now=s["nowMs"];n=s["count"];row={"report":False,"checkpoint":False}
    if now-rep_ms>=p["reportMilliseconds"]:
        sec=max(0.001,(now-start)/1000);rec=max(0.001,(now-rep_ms)/1000)
        row={"report":True,"checkpoint":False,"elapsedSeconds":rnd(sec),"perSecond":rnd(n/sec),"recentPerSecond":rnd((n-rep_n)/rec)};rep_ms=now;rep_n=n
    if now-chk_ms>=p["checkpointMilliseconds"] and n-chk_n>=p["checkpointMinimumOperations"]:
        row["checkpoint"]=True;chk_ms=now;chk_n=n
    out.append(row)
print(json.dumps(out,sort_keys=True))
