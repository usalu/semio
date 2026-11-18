#!/usr/bin/env python3
"""R49 evidence: per origin pool status from the store (puller counts, last error) and the publisher record of the server wave (owners/min, records taken per producing host) against the ~31 owners/min of the server alone."""
import json,glob,os,time
S=os.environ.get("SEMIO_FLEET_STORE_ROOT","/Users/Shared/semio-store")
rows=[]
for f in sorted(glob.glob(S+"/v1/origins/*/pool.json")):
    try:d=json.load(open(f))
    except Exception as e:print(f,"unreadable",e);continue
    rows.append(d)
    age=int(time.time()-d["updatedMs"]/1000)
    print("%-10s %-9s %-8s host=%s wave=%s units=%d claimed=%d published=%d stored=%d held=%d failed=%d running=%d slots=%d age=%ds err=%s"%(d["origin"],d["role"],d["state"],d["host"],(d["wave"] or "-")[:12],d["units"],d["claimed"],d["published"],d["stored"],d["held"],d["failed"],d["running"],d["slots"],age,d["lastError"]))
    if d["role"]=="publisher":print("   wave seconds",d.get("waveSeconds"),"owners/min",d.get("ownersPerMinute"),"(server alone: ~31)","records taken per producing host",d.get("consumedByHost"))
have={r["origin"] for r in rows}
print("origins without a pool.json:",sorted(set(os.listdir(S+"/v1/origins"))-have))
