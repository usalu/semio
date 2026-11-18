import json,glob,time,sys,os
store="/Users/Shared/semio-store/v1/origins"
out=sys.argv[1];n=int(sys.argv[2]);gap=int(sys.argv[3])
for i in range(n):
    rows=[];tot=0;ch=0;run=0
    for f in sorted(glob.glob(store+"/*/pool.json")):
        try:d=json.load(open(f))
        except Exception:continue
        o=d["origin"];age=int(time.time()-d["updatedMs"]/1000)
        tot+=d.get("ownersPerMinute",0) or 0;ch+=d.get("children",0) or 0;run+=d.get("running",0) or 0
        rows.append(f'{o} {d["state"]} w={(d.get("wave") or "-")[:6]} ch={d.get("children")} tgt={d.get("target")} run={d.get("running")} cl={d.get("claimed")} pub={d.get("published")} st={d.get("stored")} held={d.get("held")} att={d.get("claimAttempts")} opm={d.get("ownersPerMinute")} {d.get("reason")} age={age} start={int(d.get("startedMs",0)/1000)}')
    with open(out,"a") as h:h.write(time.strftime("%H:%M:%S",time.gmtime())+f" fleet_opm={tot:.1f} children={ch} running={run}\n"+"\n".join(rows)+"\n")
    time.sleep(gap)
