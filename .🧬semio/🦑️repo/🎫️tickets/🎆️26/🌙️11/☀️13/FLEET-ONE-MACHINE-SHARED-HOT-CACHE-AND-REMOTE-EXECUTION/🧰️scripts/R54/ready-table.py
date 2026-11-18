#!/usr/bin/env python3
"""Per-origin readiness from the store: ready.json (the new role) and pool.json (to see which workers run new code)."""
import json,os,sys,time
store=sys.argv[1] if len(sys.argv)>1 else "/Users/Shared/semio-store"
base=store+"/v1/origins"
now=time.time()
def load(path):
    try:return json.load(open(path))
    except Exception:return None
print("%-10s %-9s %-5s %-24s %-4s %-4s %-6s %-16s %-6s %s"%("origin","state","ready","stage","try","exit","peakMB","refusal","age","pool/attest"))
for o in sorted(os.listdir(base)):
    if not os.path.isdir(base+"/"+o):continue
    r=load(base+"/"+o+"/ready.json");p=load(base+"/"+o+"/pool.json");a=load(base+"/"+o+"/attest.json")
    pool="pool %ds"%int(now-p["updatedMs"]/1000) if p else "no-pool"
    att="attest %ds"%int(now-a["wallMs"]/1000) if a and "wallMs" in a else "attest ?"
    if r is None:
        print("%-10s %-9s %-5s %-24s %-4s %-4s %-6s %-16s %-6s %s %s"%(o,"-" if not o.endswith("-server") else "server","-","-","-","-","-","-","-",pool,att));continue
    stage="%s %s/%s"%(r["stage"],r["stageIndex"],r["stageCount"]) if r["stage"] else "-"
    print("%-10s %-9s %-5s %-24s %-4s %-4s %-6s %-16s %-6s %s %s"%(o,r["state"],r["ready"],stage,r["attempts"],r["exit"],r["peakMegabytes"],(r["refusal"] or {}).get("code","-"),"%ds"%int(now-r["updatedMs"]/1000),pool,att))
