#!/usr/bin/env python3
"""H46 round 2: backfill in host-capacity admission (exact-match atomic patch)."""
import os,sys,tempfile
HC="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/"
D=HC+"📏️decision/🟦️.ts"
G=HC+"🎫️gate/🟦️.ts"
edits=[]
def E(p,o,n):edits.append((p,o,n))

E(D,' for(const [at,waiter] of ordered.entries()){\n  const request=waiter.request,plan=',' for(const [at,waiter] of ordered.entries()){\n  backfilling=false;\n  const request=waiter.request,plan=')
E(D,' const headAfter=policy.wait.headOfLineMilliseconds,queuedSince=',' let barrier:{extra:number;soft:boolean}|undefined,backfilling=false;const backfillBound=hostBackfillBoundMillisecondsV1(policy,ledger);\n const headAfter=policy.wait.headOfLineMilliseconds,queuedSince=')
E(D,'const admit=(jobs:number,reason:HostReasonV1,megabytes?:number):void=>{const reserved=megabytes??need(jobs);','const admit=(jobs:number,reason:HostReasonV1,megabytes?:number):void=>{const reserved=megabytes??need(jobs);if(barrier!==undefined)barrier={...barrier,extra:barrier.extra-reserved};')
E(D,'else for(const head of heads){head.bypasses++;if(head.bypasses>=policy.wait.maximumBypasses)blocked=true;}','else if(!backfilling)for(const head of heads){head.bypasses++;if(head.bypasses>=policy.wait.maximumBypasses)blocked=true;}')
E(D,'  if(elastic&&(blocked||waits)||!(plan&&!planBarrier)&&(blocked&&!(quiet&&quietLeft>0)||reserved&&!quiet&&(active.length-(booked?1:0)>0||admitted>0))){refuse("queue-order");continue;}\n',
'''  const barred=!(plan&&!planBarrier)&&(blocked&&!(quiet&&quietLeft>0)||reserved&&!quiet&&(active.length-(booked?1:0)>0||admitted>0));
  if(elastic&&(blocked||waits)||barred&&!(barrier!==undefined&&barrier.soft&&booked===undefined&&!quiet&&!elastic&&request.priority!=="lent"&&need(request.minimumJobs)<=barrier.extra)){refuse("queue-order");continue;}
  backfilling=barred;
''')
E(D,'''    if(reservable&&(waited>=policy.wait.bypassMilliseconds||bypasses>=policy.wait.maximumBypasses)){planBarrier=planBarrier||request.operation==="graph-plan";if(drains)blocked=true;else reserved=true;}else if(drains)heads.push({bypasses});''',
'''    const hold=():void=>{const extra=drains?Math.min(drainedRoom,drainedBudget)-need(request.minimumJobs)-ledger.reduce((sum,grant)=>sum+(grant.id!==request.id&&grant.admittedAtMs>=waiter.arrivedAtMs?committedOf(grant):0),0)-decisions.reduce((sum,row)=>sum+(row.verdict==="admit"?row.reservedMegabytes:0),0):Infinity,soft=waited>=policy.wait.bypassMilliseconds&&waited<backfillBound;barrier=barrier===undefined?{extra,soft}:{extra:Math.min(barrier.extra,extra),soft:barrier.soft&&soft};};
    if(reservable&&(waited>=policy.wait.bypassMilliseconds||bypasses>=policy.wait.maximumBypasses)){planBarrier=planBarrier||request.operation==="graph-plan";hold();if(drains)blocked=true;else reserved=true;}else if(drains){hold();heads.push({bypasses});}''')
E(D,'/** 🚫️ Whether a request can never run on this machine, whatever else runs:','''/** 🪂️ How long a request that holds the line (it waited `wait.bypassMilliseconds`, the barrier of `decideHostCapacityV1`) may be backfilled before it reserves for good: twice the median wait the live grants measured between arrival and admission (a host whose requests wait for minutes does not freeze its small requests after thirty seconds), at least `wait.bypassMilliseconds` (the measured waits of fewer than three grants say nothing: exactly that) and at most `wait.agingMilliseconds` (from then on the request heads the line and drains the rest, see the starvation ticket). While it is backfillable a smaller request that fits the room right now runs ahead of it when its reservation fits beside what the holder needs once everything running has ended (and the grants admitted since the holder arrived stay counted), so a backfilled request can never delay the holder beyond the end of the grants that already ran when it arrived.
 * @see decideHostCapacityV1
 * @see ../🧪️tests/🪂️backfill/🟦️.ts */
export function hostBackfillBoundMillisecondsV1(policy:HostCapacityPolicyV1,ledger:readonly Pick<HostGrantV1,"admittedAtMs"|"arrivedAtMs">[]):number{
 const waits=ledger.flatMap(grant=>grant.arrivedAtMs!==undefined&&grant.admittedAtMs>=grant.arrivedAtMs?[grant.admittedAtMs-grant.arrivedAtMs]:[]).sort((left,right)=>left-right);
 if(waits.length<3)return policy.wait.bypassMilliseconds;
 return Math.min(policy.wait.agingMilliseconds,Math.max(policy.wait.bypassMilliseconds,2*waits[Math.floor(waits.length/2)]!));
}

/** 🚫️ Whether a request can never run on this machine, whatever else runs:''')
E(G,'...request.quiet===true?{quiet:true,arrivedAtMs}:{}','...request.quiet===true?{quiet:true}:{},arrivedAtMs')


# texts (regex, idempotent): the waiting reason names the reservation and the backfill, EN and DE
import re
R="/Users/ueli/Documents/semio/🧰️framework/"
TEXTFILES=[HC+"🧬️schema/🔣️.json",HC+"🧪️tests/🚪️gate/🟦️.ts",R+"🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📏️incremental/🏎️bench/🧪️tests/🧫️fixtures/🔣️.json",R+"🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚢️fleet-status/🧪️tests/🧫️fixtures/🔣️.json",R+"🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/📐️booking/🔣️.json",R+"🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/📐️booking/🟦️.ts"]
TEXTS=[(re.compile(r"an earlier request is still waiting(?! and holds)"),"an earlier request is still waiting and holds the next reservation; smaller requests that fit beside it run first"),(re.compile(r"eine frühere Anfrage wartet noch(?! und hält)"),"eine frühere Anfrage wartet noch und hält die nächste Reservierung; kleinere Anfragen, die daneben passen, laufen zuerst")]
for p in TEXTFILES:
    if not os.path.exists(p):print("MISSING",p[-60:]);sys.exit(2)
by={}
for p,o,n in edits:by.setdefault(p,[]).append((o,n))
out={}
for p,rows in by.items():
    s=open(p,encoding="utf8").read()
    for o,n in rows:
        if s.count(o)!=1:print("MISMATCH",p[-40:],o[:90].replace("\n","|"),s.count(o));sys.exit(2)
        s=s.replace(o,n)
    out[p]=s

for p in TEXTFILES:
    t=open(p,encoding="utf8").read();u=t
    for rx,new in TEXTS:u=rx.sub(new,u)
    if u!=t:out[p]=u
NEW=HC+"🧪️tests/🪂️backfill/"
here=os.path.dirname(os.path.abspath(__file__))+"/staged/"
for src,dst in[("🟦️.ts","🟦️.ts"),("🐍️.py","🐍️.py"),("🧫️fixtures/🔣️.json","🧫️fixtures/🔣️.json")]:
    out[NEW+dst]=open(here+src,encoding="utf8").read()
if "--check" in sys.argv:print("ok");sys.exit(0)
if "--scratch" in sys.argv:
    d=sys.argv[sys.argv.index("--scratch")+1];os.makedirs(d,exist_ok=True)
    for i,s in enumerate(out.values()):open(os.path.join(d,str(i)+".txt"),"w",encoding="utf8").write(s)
    sys.exit(0)
for p,s in out.items():
    os.makedirs(os.path.dirname(p),exist_ok=True);fd,t=tempfile.mkstemp(dir=os.path.dirname(p),prefix=".h46-");os.write(fd,s.encode());os.close(fd);os.chmod(t,os.stat(p).st_mode if os.path.exists(p) else 0o644);os.rename(t,p);print("wrote",p[-50:])
