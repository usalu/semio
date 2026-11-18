#!/usr/bin/env python3
"""R74 watch: one line per worker origin from <store>/v1/origins/*/ready.json. Usage: snap.py [rounds] [interval seconds]"""
import json,glob,os,sys,time,datetime
St="/Users/Shared/semio-store/v1/origins"
def once():
    now=time.time()
    print(datetime.datetime.now().strftime("%H:%M:%S"),flush=True)
    for p in sorted(glob.glob(St+"/*/ready.json")):
        o=os.path.basename(os.path.dirname(p))
        try:d=json.load(open(p))
        except Exception as e:print(o,"unreadable");continue
        r=(d.get("reason") or {}).get("code");f=(d.get("refusal") or {}).get("code")
        print("%-10s ready=%-5s %-8s %-12s try=%s age=%4ds pid=%s reason=%s refusal=%s | %s"%(o,d["ready"],d["state"],d["stage"],d["attempts"],now-d["updatedMs"]/1000,d["pid"],r,f,(d["tail"] or [""])[-1][:70]),flush=True)
rounds=int(sys.argv[1]) if len(sys.argv)>1 else 1;gap=int(sys.argv[2]) if len(sys.argv)>2 else 60
for i in range(rounds):
    once()
    if i<rounds-1:time.sleep(gap)
