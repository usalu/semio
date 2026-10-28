"""🧪️ Discrete-event simulation of the host capacity gate on the trace of the heaviest fleet night (`trace.py`): the same requests arrive under the OLD policy (fixed defaults, 20 % floor, 300 s ramp, 30 s lending) and under the NEW one (measured demand, scaled floor, 60 s ramp, 5 s lending, interactive priority), on a 32 GB replay and on a 16 GB profile. The decision itself is `admission_model.evaluate`, equal to the TypeScript on all 207 stored cases; the estimator is the repo oracle."""
import collections,copy,importlib.util,json,math,os,random,statistics,sys,time
HERE=os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0,HERE)
from admission_model import evaluate,floor_mb
ROOT=os.path.dirname(HERE)
_spec=importlib.util.spec_from_file_location("oracle",os.path.join(os.getcwd(),"🧰️framework","🔨️modules","🏃️process","🏗️build","🌡️host-capacity","🧪️tests","📐️estimates","🐍️oracle","🐍️.py"))
oracle=importlib.util.module_from_spec(_spec);_spec.loader.exec_module(oracle)
TRACE=os.path.join(ROOT,"🗑️generated","ram-g","trace.json")
POLICY_JSON=os.path.join(os.getcwd(),"🧰️framework","🛍️products","🦑️repo","🔨️modules","📚️library","🏗️build","🔣️policy.json")

def new_policy():
    return json.load(open(POLICY_JSON,encoding="utf8"))["hostCapacity"]

def old_policy():
    p=copy.deepcopy(new_policy())
    p["memory"].update({"floorBasisPoints":2000,"floorMinimumMegabytes":4096,"floorMaximumMegabytes":8192,"rampMilliseconds":300000})
    p["wait"]["lendAfterMilliseconds"]=30000
    return p

VARIANTS={
 "old":dict(policy=old_policy(),learn=False,human=False),
 "floor":dict(policy=None,learn=False,human=False,keep=("rampMilliseconds","lend")),
 "floor+ramp+lend":dict(policy=new_policy(),learn=False,human=False),
 "new":dict(policy=new_policy(),learn=True,human=True),
}
_noprio=copy.deepcopy(new_policy());_noprio["estimates"]["quickMilliseconds"]=0
VARIANTS["new-without-priority"]=dict(policy=_noprio,learn=True,human=False)
VARIANTS["floor"]["policy"]=copy.deepcopy(old_policy());VARIANTS["floor"]["policy"]["memory"].update({k:new_policy()["memory"][k] for k in ("floorBasisPoints","floorMinimumMegabytes","floorMaximumMegabytes")})

def load_requests(start,end):
    t=json.load(open(TRACE,encoding="utf8"))
    grants={}
    for g in t["grants"]:grants.setdefault(g["op"],[]).append(g)
    rows=[r for r in t["requests"] if start<=r["arrival"]<end]
    rows.sort(key=lambda r:r["arrival"])
    return rows,grants

def build_workload(rows,grants,seed,thin=1):
    rng=random.Random(seed)
    strata={}
    for op,gs in grants.items():
        gs=sorted(gs,key=lambda g:g["peak"]);k=4;size=max(1,len(gs)//k)
        strata[op]=[gs[i*size:(i+1)*size if i<k-1 else len(gs)] for i in range(k)]
    work=[]
    for index,r in enumerate(rows):
        if thin>1 and index%thin:continue
        op=r["op"];pool=grants.get(op) or grants.get("run")
        stratum=rng.randrange(4);bucket=(strata.get(op) or strata["run"])[stratum] or pool
        g=rng.choice(bucket);duration=max(20,rng.choice(pool)["since"]);idle=rng.uniform(0.1,0.6)
        session=r["file"].split("/🗑️generated/")[-1].split("/")[0]
        jobs=max(1,rng.choice(pool)["jobs"]) if op in ("run","cargo","exec","run-many") else 1
        fixed={"show":5120,"graph":5120,"dependencies":6144,"command":2048}.get(op,1536)
        per_job=0 if op in ("show","graph","dependencies","command") else r["need"]-1536
        work.append(dict(index=index,arrival=r["arrival"],op=op,session=session,stratum=stratum,peak=g["peak"],duration=duration,idle=idle,jobs=jobs,fixed=fixed,per_job=per_job,cpu=2 if op in ("show","graph","dependencies") else 1))
    return work

def run(work,variant,total,cores,foreign_mean,foreign_sd,seed,horizon_extra=6*3600,tick=5):
    spec=VARIANTS[variant];policy=spec["policy"];rng=random.Random(seed*7+1)
    t0=min(w["arrival"] for w in work);t1=max(w["arrival"] for w in work)+horizon_extra
    pending=sorted(work,key=lambda w:w["arrival"]);waiters=[];grants=[];done=[];store={"version":1,"classes":{}}
    f=foreign_mean;phi=0.985;wait={};started={};conc=0;peak_conc=0;below=0;ticks=0;crit=0;used_integral=0.0
    now=t0;nid=0
    while now<t1 and (pending or waiters or grants):
        if int(now-t0)%30<tick:
            f=foreign_mean+phi*(f-foreign_mean)+rng.gauss(0,foreign_sd*math.sqrt(1-phi*phi));f=min(max(f,0.35*foreign_mean),2.2*foreign_mean)
        ms=int((now-t0)*1000)
        for g in list(grants):
            age=now-g["start"];idle_end=g["duration"]*g["idle"]
            if age>=g["duration"]:
                grants.remove(g)
                if spec["learn"] and g["peak_seen"]>0:
                    default=max(1,g["default"]*1);active=max(0,g["duration"]-g["lent_total"])
                    store=oracle.record({"estimates":policy["estimates"]},store,g["lineage"],{"operation":g["op"],"jobs":g["jobs"],"peakMegabytes":g["peak_seen"],"defaultMegabytes":default,"activeMilliseconds":int(active*1000),"endedAtMs":ms})
                done.append(g);continue
            if age<idle_end:g["actual"]=150
            else:g["actual"]=150+(g["peak"]-150)*min(1,(age-idle_end)/30)
            beat=policy["wait"]["heartbeatMilliseconds"]/1000;first=min(3,beat)
            if (age>=first and not g["seen"]) or age-g["last_beat"]>=beat:
                g["last_beat"]=age;g["seen"]=True;g["observed"]=int(g["actual"]);g["peak_seen"]=max(g["peak_seen"],g["observed"])
            idle_phase=age<idle_end
            lend_after=policy["wait"]["lendAfterMilliseconds"]/1000
            if idle_phase and age>=lend_after and g["lent"] is None:g["lent"]=now
            if not idle_phase and g["lent"] is not None:
                g["lent_total"]+=now-g["lent"];g["lent"]=None;g["resumed"]=ms
        ours=sum(g["actual"] for g in grants)
        available=max(0,total-f-ours)
        share=available*10000/total
        pressure="critical" if share<policy["pressure"]["availableCriticalBasisPoints"] else "warn" if share<policy["pressure"]["availableWarnBasisPoints"] else "normal"
        if pressure=="critical":crit+=tick
        used_integral+=ours*tick;ticks+=1
        while pending and pending[0]["arrival"]<=now:
            w=pending.pop(0);nid+=1
            default_need=lambda jobs,w=w:w["fixed"]+jobs*w["per_job"]
            lineage=[w["op"],f"{w['op']}:s{w['stratum']}"]
            est=oracle.estimate({"estimates":policy["estimates"]},store,lineage,ms) if spec["learn"] else {"ratioBasisPoints":10000,"quick":False,"samples":0}
            base={"id":f"r{nid:08d}","operation":w["op"],"priority":"batch","minimumJobs":1,"preferredJobs":w["jobs"],"fixedMegabytes":w["fixed"],"perJobMegabytes":w["per_job"],"cpuPerJob":w["cpu"]}
            base["priority"]=oracle.priority("batch","interactive" if (spec["human"] and w["session"].startswith("human")) else None,est["quick"]) if spec["learn"] else "batch"
            request=oracle.scale({"estimates":policy["estimates"]},base,est["ratioBasisPoints"],total-floor_mb(policy,total)) if spec["learn"] else base
            request={k:v for k,v in request.items() if k!="envelopePerJobMegabytes"}
            waiters.append({"request":request,"arrivedAtMs":ms,"pid":nid,"session":w["session"],"weight":1,"_w":w,"_lineage":lineage,"_default":default_need})
        if waiters:
            host={"version":1,"atMs":ms,"platform":"darwin","cores":cores,"totalMegabytes":total,"availableMegabytes":int(available),"pressure":pressure,"swapUsedMegabytes":None,"swapOutMegabytesPerSecond":None,"load":None,"openFiles":None,"openFilesMaximum":None,"attachedImages":None}
            ledger=[{"id":g["id"],"operation":g["op"],"jobs":g["jobs"],"cpu":g["cpu_total"],"reservedMegabytes":g["reserved"],"admittedAtMs":g["admitted_ms"],"observedMegabytes":g.get("observed"),"pid":1,"bootAtMs":0,"session":g["session"],**({"lentSinceMs":int((g["lent"]-t0)*1000)} if g["lent"] is not None else {}),**({"resumedAtMs":g["resumed"]} if g.get("resumed") else {})} for g in grants]
            result=evaluate(policy,host,ledger,[{k:v for k,v in x.items() if not k.startswith("_")} for x in waiters],ms)
            admitted_ids={d["id"]:d for d in result["decisions"] if d["verdict"]=="admit"}
            for x in list(waiters):
                d=admitted_ids.get(x["request"]["id"])
                if d is None:continue
                waiters.remove(x);w=x["_w"];wait[w["index"]]=(now-w["arrival"],w)
                grants.append({"id":x["request"]["id"],"op":w["op"],"session":w["session"],"jobs":d["jobs"],"reserved":d["reservedMegabytes"],"cpu_total":d["jobs"]*w["cpu"],"admitted_ms":ms,"start":now,"duration":w["duration"],"idle":w["idle"],"peak":w["peak"],"actual":150,"observed":None,"peak_seen":0,"seen":False,"last_beat":0,"lent":None,"lent_total":0,"lineage":x["_lineage"],"default":x["_default"](d["jobs"])})
        peak_conc=max(peak_conc,len(grants))
        now+=tick
    waits=sorted(v[0] for v in wait.values())
    def pct(p):return waits[min(len(waits)-1,int(p*len(waits)))] if waits else None
    delayed=[x for x in waits if x>=5]
    def dpct(p):return round(delayed[min(len(delayed)-1,int(p*len(delayed)))]) if delayed else 0
    return {"variant":variant,"delayed":len(delayed),"delayed_median_s":dpct(0.5),"delayed_p90_s":dpct(0.9),"delayed_p99_s":dpct(0.99),"requests":len(work),"admitted":len(wait),"unadmitted":len(work)-len(wait),"wait_total_h":round(sum(waits)/3600,1),"peak_concurrent":peak_conc,"critical_minutes":round(crit/60,1),"mean_ours_mb":int(used_integral/max(1,ticks*tick)),"span_h":round((now-t0)/3600,1),"waits":wait}

def scenario_fleet3(seed,hours=6):
    """👥️ Three agents and one developer on a 16 GB machine: every agent runs a full suite (about 4.5 GB, 15 min) now and then and hot rebuilds all the time, the developer only hot rebuilds."""
    rng=random.Random(seed);work=[];index=0;horizon=hours*3600
    def add(session,at,stratum,peak,duration,jobs,kind):
        nonlocal index
        work.append(dict(index=index,arrival=at,op="run",session=session,stratum=stratum,peak=peak,duration=duration,idle=0.1,jobs=jobs,fixed=1536,per_job=2048,cpu=1,kind=kind));index+=1
    for session in ("agent-a","agent-b","agent-c"):
        t=rng.uniform(0,600)
        while t<horizon:add(session,t,3,rng.randint(3800,5200),rng.randint(600,1100),2,"suite");t+=rng.uniform(2400,4000)
        t=rng.uniform(0,120)
        while t<horizon:add(session,t,0,rng.randint(300,700),rng.randint(20,60),1,"agent-hot");t+=rng.uniform(240,480)
    t=rng.uniform(0,120)
    while t<horizon:add("human-dev",t,0,rng.randint(300,700),rng.randint(20,60),1,"human-hot");t+=rng.uniform(300,600)
    return sorted(work,key=lambda w:w["arrival"])

def kind_report(result,work):
    by={}
    for value,w in result["waits"].values():by.setdefault(w["kind"],[]).append(value)
    return {k:{"n":len(v),"median_s":round(statistics.median(v)),"p90_s":round(sorted(v)[int(0.9*len(v))]),"max_s":round(max(v))} for k,v in sorted(by.items())}

def window(day,hour,hours):
    start=time.mktime((2026,10,day,hour,0,0,0,0,-1));return start,start+hours*3600

def aggregate(rows):
    keys=("delayed","delayed_median_s","delayed_p90_s","delayed_p99_s","wait_total_h","unadmitted","peak_concurrent","critical_minutes","mean_ours_mb","span_h")
    return {k:round(statistics.mean(r[k] for r in rows),1) for k in keys}

if __name__=="__main__":
    out={};seeds=range(1,6)
    start,end=window(24,16,16)
    rows,grants=load_requests(start,end)
    busiest=[name for name,_ in collections.Counter(r["file"].split("/🗑️generated/")[-1].split("/")[0] for r in rows).most_common(3)]
    for label,total,cores,mean,sd,only in (("replay-32GB",32768,10,13000,2500,None),("profile-16GB-3-agents",16384,8,6000,1200,busiest)):
        work=build_workload([r for r in rows if only is None or r["file"].split("/🗑️generated/")[-1].split("/")[0] in only],grants,seed=11);out[label]={}
        for variant in VARIANTS:
            runs=[]
            for seed in seeds:
                r=run(work,variant,total,cores,mean,sd,seed=seed);r.pop("waits");runs.append(r)
            out[label][variant]=aggregate(runs);print(label,variant,json.dumps(out[label][variant]))
    out["fleet-3-agents-1-human-16GB"]={}
    for variant in ("old","floor+ramp+lend","new-without-priority","new"):
        reports=[]
        for seed in seeds:
            work=scenario_fleet3(seed);r=run(work,variant,16384,8,6000,1200,seed=seed,horizon_extra=4*3600);reports.append(kind_report(r,work))
        merged={kind:{k:round(statistics.mean(rep[kind][k] for rep in reports if kind in rep),1) for k in ("n","median_s","p90_s","max_s")} for kind in reports[0]}
        out["fleet-3-agents-1-human-16GB"][variant]=merged;print("fleet3",variant,json.dumps(merged))
    json.dump(out,open(os.path.join(ROOT,"🗑️generated","ram-g","sim.json"),"w"),indent=1)
