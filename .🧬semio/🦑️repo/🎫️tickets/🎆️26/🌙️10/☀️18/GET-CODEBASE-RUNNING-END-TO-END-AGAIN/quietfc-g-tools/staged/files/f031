"""🪜️ Independent Python oracle of the phased estimates: the cut of a run's readings into phase segments, recording by phase (the graph build leaves the run's own classes), the estimate and the reservation by phase, the re-sizing of a grant at a phase boundary and the compute slots of a rebuild; it builds on the oracle of the estimates (whose weighted quantile is checked against numpy) and writes the shared vectors for every language. Run `python3 🐍️.py` to regenerate `../🧫️fixtures/🔣️.json`; `python3 🐍️.py --check` verifies the stored vectors."""
import importlib.util,json,math,os,random,sys

HERE=os.path.dirname(os.path.abspath(__file__))
_spec=importlib.util.spec_from_file_location("estimates_oracle",os.path.join(HERE,"..","..","📐️estimates","🐍️oracle","🐍️.py"))
base=importlib.util.module_from_spec(_spec);_spec.loader.exec_module(base)
MINIMUM_RAMP=60000
NEAR=9500
PHASES=["assembly","plan","rebuild","execution"]

def _round(x):
    return math.floor(x+0.5)

def clamp(low,value,high):
    return max(low,min(high,value))

def segments(readings,transitions,end,near=NEAR):
    if not transitions:return []
    bounds=[{"phase":"assembly","start":0}]
    for row in transitions:
        if row["phase"]!=bounds[-1]["phase"]:bounds.append({"phase":row["phase"],"start":max(bounds[-1]["start"],row["atMilliseconds"])})
    rows=[{"phase":b["phase"],"start":b["start"],"end":max(b["start"],bounds[i+1]["start"] if i+1<len(bounds) else end),"readings":[]} for i,b in enumerate(bounds)]
    for at,megabytes,tag in readings:
        phase=tag if tag is not None else "assembly"
        owners=[r for r in rows if r["phase"]==phase and r["start"]<=at]
        owner=owners[-1] if owners else next((r for r in rows if r["phase"]==phase),None)
        if owner is not None:owner["readings"].append((at,megabytes))
    out=[]
    for r in rows:
        peak=max([m for _,m in r["readings"]],default=0)
        close=[a for a,m in r["readings"] if m*10000>=peak*near]
        out.append({"phase":r["phase"],"startMilliseconds":r["start"],"activeMilliseconds":r["end"]-r["start"],"peakMegabytes":peak,"peakAtMilliseconds":0 if peak==0 else min(close)})
    return out

def rebuilt(segs,at):
    return sum(max(0,min(at,s["startMilliseconds"]+s["activeMilliseconds"])-s["startMilliseconds"]) if s["phase"]=="rebuild" else 0 for s in segs)

def strongest(segs,phase):
    rows=sorted([s for i,s in enumerate(segs) if s["phase"]==phase and s["peakMegabytes"]>0 and not (phase=="plan" and i>0 and segs[i-1]["phase"]=="rebuild")],key=lambda s:(-s["peakMegabytes"],s["startMilliseconds"]))
    return rows[0] if rows else None

def phase_class(phase):
    return "phase:"+phase

def record(policy,store,lineage,operation,default,run):
    phases=run.get("phases") or []
    if not phases:
        return base.record(policy,store,lineage,{"operation":operation,"jobs":run["jobs"],"peakMegabytes":run["peakMegabytes"],"defaultMegabytes":default,"activeMilliseconds":run["activeMilliseconds"],"peakAtMilliseconds":run["peakAtMilliseconds"],"endedAtMs":run["endedAtMs"]})
    reference=policy["estimates"]["phaseReferenceMegabytes"]
    own=run["activeMilliseconds"]-rebuilt(phases,run["activeMilliseconds"])
    nxt=store
    def put(classes,seg,default_,active):
        nonlocal nxt
        peak_at=max(0,seg["peakAtMilliseconds"]-rebuilt(phases,seg["peakAtMilliseconds"]))
        nxt=base.record(policy,nxt,classes,{"operation":operation,"jobs":run["jobs"],"peakMegabytes":seg["peakMegabytes"],"defaultMegabytes":default_,"activeMilliseconds":active(peak_at),"peakAtMilliseconds":peak_at,"endedAtMs":run["endedAtMs"]})
    execution=strongest(phases,"execution") or strongest(phases,"plan")
    if execution:put(lineage,execution,default,lambda peak_at:own)
    for phase in ("assembly","plan"):
        seg=strongest(phases,phase)
        if seg:put([phase_class(phase)],seg,reference,lambda peak_at,seg=seg:max(seg["activeMilliseconds"],peak_at))
    build=strongest(phases,"rebuild")
    if build:
        nxt=base.record(policy,nxt,[phase_class("rebuild")],{"operation":operation,"jobs":run["jobs"],"peakMegabytes":build["peakMegabytes"],"defaultMegabytes":reference,"activeMilliseconds":build["activeMilliseconds"],"peakAtMilliseconds":min(build["activeMilliseconds"],max(0,build["peakAtMilliseconds"]-build["startMilliseconds"])),"endedAtMs":run["endedAtMs"]})
    return nxt

def estimate_phase(policy,store,phase,prior,now):
    rules=policy["estimates"]
    samples=store["classes"].get(phase_class(phase),{"samples":[]})["samples"]
    weight=base.weights(policy,samples,now)
    total=sum(weight)
    if not samples or total==0:return None
    effective=total/1e6
    trust=min(1.0,effective/rules["minimumSamples"])
    margin=10000+rules["marginBasisPoints"]
    learned=_round(base.weighted_quantile(samples,weight,"ratioBasisPoints",rules["quantileBasisPoints"])*margin*rules["phaseReferenceMegabytes"]/100000000)
    late=_round(base.weighted_quantile(samples,weight,"peakAtMilliseconds",rules["quantileBasisPoints"])*margin/10000)
    return {"megabytes":max(rules["minimumMegabytes"],_round(trust*learned+(1-trust)*prior)),"rampMilliseconds":clamp(MINIMUM_RAMP,_round(trust*late+(1-trust)*rules["defaultRampMilliseconds"]),rules["maximumRampMilliseconds"]),"samples":_round(effective)}

def estimate(policy,store,lineage,req,now):
    execution=base.estimate(policy,store,lineage,now)
    prior=req["fixedMegabytes"]+req["minimumJobs"]*req["perJobMegabytes"]
    preferred=req["fixedMegabytes"]+req["preferredJobs"]*req["perJobMegabytes"]
    bound,source=base.class_bound(policy,store,lineage,now)
    grow=execution["ratioBasisPoints"] if preferred<=0 else max(1,min(execution["ratioBasisPoints"],bound*10000//preferred))
    if not (source=="fallback" and execution["samples"]==0):execution={**execution,"ratioBasisPoints":grow}
    learned={}
    for phase in ("assembly","plan","rebuild"):
        size=estimate_phase(policy,store,phase,policy["estimates"]["rebuildPriorMegabytes"] if phase=="rebuild" else prior,now)
        if size:learned[phase]={"megabytes":size["megabytes"],"rampMilliseconds":size["rampMilliseconds"]}
    scaled={"megabytes":_round(preferred*execution["ratioBasisPoints"]/10000),"rampMilliseconds":execution["rampMilliseconds"]}
    floor=max(learned.get("assembly",{"megabytes":0})["megabytes"],learned.get("plan",{"megabytes":0})["megabytes"])
    decisive=None
    for phase in ("assembly","plan"):
        row=learned.get(phase)
        if row and (decisive is None or row["megabytes"]>decisive["megabytes"]):decisive=row
    ramp=decisive["rampMilliseconds"] if decisive and decisive["megabytes"]>scaled["megabytes"] else execution["rampMilliseconds"]
    phases={k:learned[k] for k in ("assembly","plan","rebuild") if k in learned}
    phases["execution"]=scaled
    return {**execution,"growRatioBasisPoints":grow,"rampMilliseconds":ramp,"floorMegabytes":floor,"phases":phases}

def resize(policy,req,est,capacity,phase,context):
    rules=policy["estimates"]
    execution=base.scale(policy,req,est["ratioBasisPoints"],capacity)
    def ramp(ms):return clamp(MINIMUM_RAMP,ms,rules["maximumRampMilliseconds"])
    if phase=="rebuild":
        build=est["phases"].get("rebuild")
        wanted=0 if build is None else min(capacity,build["megabytes"])
        if build is not None and wanted>context["reservedMegabytes"]:return {"reservedMegabytes":wanted,"rampMilliseconds":ramp(build["rampMilliseconds"])}
        return None
    order=["assembly","plan","execution"]
    rest=[]
    for name in order[order.index(phase):]:
        if name=="execution":mb=execution["fixedMegabytes"]+context["jobs"]*execution["perJobMegabytes"]
        else:mb=est["phases"].get(name,{"megabytes":0})["megabytes"]
        rest.append({"megabytes":mb,"rampMilliseconds":est["phases"].get(name,{"rampMilliseconds":est["rampMilliseconds"]})["rampMilliseconds"]})
    decisive=rest[0]
    for row in rest:
        if row["megabytes"]>0 and row["megabytes"]>=decisive["megabytes"]:decisive=row
    wanted=min(context["reservedMegabytes"],decisive["megabytes"])
    if wanted<context["reservedMegabytes"]:return {"reservedMegabytes":wanted,"rampMilliseconds":ramp(decisive["rampMilliseconds"]-context["activeMilliseconds"])}
    return None

def scale_floor(policy,request,ratio,capacity,floor):
    rules=policy["estimates"]
    per_job=_round(request["perJobMegabytes"]*ratio/10000)
    fixed=_round(request["fixedMegabytes"]*ratio/10000)
    fixed+=max(0,max(rules["minimumMegabytes"],floor)-(fixed+request["minimumJobs"]*per_job))
    ceiling=capacity*rules["maximumShareBasisPoints"]//10000
    need=fixed+request["minimumJobs"]*per_job
    if need>ceiling:
        fixed=fixed*ceiling//need
        per_job=per_job*ceiling//need
    return {**request,"fixedMegabytes":fixed,"perJobMegabytes":per_job,"envelopePerJobMegabytes":request["perJobMegabytes"]}

def rebuild_slots(policy,store,total,floor,now):
    rules=policy["estimates"]
    size=estimate_phase(policy,store,"rebuild",rules["rebuildPriorMegabytes"],now)
    size=size["megabytes"] if size else rules["rebuildPriorMegabytes"]
    return clamp(1,max(0,total-floor)*rules["rebuildShareBasisPoints"]//10000//size,rules["maximumRebuildSlots"])


EXEC_POOL=[108,135,214,231,261,292,359,477,640,734,992,1189,1297,1596,1808,1920,2320,2622,2895,3369,3712,4049,4433,4743]
BUILD_POOL=[5049,5210,5610,5845,5992,6210,6699,7485,9044,9494,10560,11845]
DURATION_POOL=[44,106,174,341,531,735,1070,1564,1905,2737,3600,3600]
KINDS=[("test",1536),("check",4096),("build",2048)]
REPLAY_CAPACITY=14336

def timeline(run):
    out=[];t=0
    for phase,length,mb in (("assembly",6,run["A"]),("plan",8,run["P"]),("rebuild",run["tR"],run["R"]),("plan",5 if run["rebuild"] else 0,int(run["R"]*0.97) if run["rebuild"] else 0),("execution",run["tE"],run["E"])):
        if length>0:out.append((t,t+length,phase,mb));t+=length
    return out

def replay(seed,count,share):
    """The recorded night as a stream: a covered or rebuilding run per step, learned by the single class (whole tree peak) and by phases; the asks of both are the vectors."""
    rng=random.Random(seed);policy={"estimates":base.POLICY,"memory":{"rampMilliseconds":MINIMUM_RAMP}}
    stores={"old":{"version":1,"classes":{}},"new":{"version":1,"classes":{}}}
    runs=[];now=1_900_000_000_000
    for index in range(count):
        klass=rng.randrange(6);kind,per_job=KINDS[klass%3];jobs=rng.choice([1,1,1,1,2,3,4])
        rebuild=rng.random()<share
        run={"A":max(150,int(rng.gauss(380,60))),"P":max(300,int(rng.gauss(900,200))),"E":max(150,int(rng.choice(EXEC_POOL)*[0.5,0.8,1.0,1.3,0.65,1.6][klass]*rng.uniform(0.85,1.15))),"R":int(rng.choice(BUILD_POOL)*rng.uniform(0.9,1.1)) if rebuild else 0,"rebuild":rebuild,"tR":rng.choice([90,180,300,600]) if rebuild else 0,"tE":max(20,rng.choice(DURATION_POOL))}
        request={"id":"aaaaaaaa1","operation":"run","priority":"batch","minimumJobs":1,"preferredJobs":jobs,"fixedMegabytes":1536,"perJobMegabytes":per_job,"cpuPerJob":1}
        lineage=["run",f"run:{kind}",f"run:{kind}:class{klass}"];default=request["fixedMegabytes"]+jobs*per_job
        rows=timeline(run);now+=120_000
        readings=[]
        for start,end,phase,mb in rows:
            readings+=[[at*1000+500,mb,phase] for at in range(start,end)]+[[end*1000,mb,phase]]
        transitions=[];last=None
        for start,end,phase,mb in rows:
            if phase!=last:transitions.append({"phase":phase,"atMilliseconds":start*1000});last=phase
        end=rows[-1][1]*1000;segs=segments(readings,transitions[1:],end)
        total=max(s["peakMegabytes"] for s in segs);near=min(a for a,m,_ in readings if m*10000>=total*9500)
        old=base.estimate(policy,stores["old"],lineage,now);askOld_request=base.scale(policy,request,old["ratioBasisPoints"],REPLAY_CAPACITY)
        new=estimate(policy,stores["new"],lineage,request,now);askNew_request=scale_floor(policy,request,new["ratioBasisPoints"],REPLAY_CAPACITY,new["floorMegabytes"])
        need=lambda req:req["fixedMegabytes"]+jobs*req["perJobMegabytes"]
        stores["old"]=base.record(policy,stores["old"],lineage,{"operation":"run","jobs":jobs,"peakMegabytes":total,"defaultMegabytes":default,"activeMilliseconds":end,"peakAtMilliseconds":near,"endedAtMs":now})
        phased={"jobs":jobs,"peakMegabytes":total,"activeMilliseconds":end,"peakAtMilliseconds":near,"endedAtMs":now,"phases":segs}
        stores["new"]=record(policy,stores["new"],lineage,"run",default,phased)
        runs.append({"request":request,"lineage":lineage,"jobs":jobs,"defaultMegabytes":default,"nowMs":now,"phased":phased,"coveredPeak":max(run["A"],run["P"],run["E"]),"rebuild":rebuild,"buildPeak":run["R"],"expect":{"askOld":need(askOld_request),"askNew":need(askNew_request)}})
    return runs

def random_run(rng,now,with_phases=True,rebuild=None):
    jobs=rng.choice([1,2,4])
    if not with_phases:
        return {"jobs":jobs,"peakMegabytes":rng.choice([300,900,1700,3500,6000,9000]),"activeMilliseconds":rng.choice([8000,60000,300000]),"peakAtMilliseconds":rng.choice([500,8000,40000]),"endedAtMs":now-rng.randrange(0,6*86400000)}
    rebuild=rng.random()<0.4 if rebuild is None else rebuild
    t=0;transitions=[];readings=[]
    def stretch(phase,length,peak):
        nonlocal t
        transitions.append({"phase":phase,"atMilliseconds":t})
        for step in range(rng.choice([1,2,3])):
            readings.append([t+int(length*(step+1)/4),max(1,int(peak*rng.uniform(0.5,1.0))),phase])
        t+=length
    first=rng.choice([4000,9000]);readings.append([1000,rng.randint(200,600),None]);t=first
    stretch("plan",rng.choice([3000,8000]),rng.choice([700,1400,2200]))
    if rebuild:stretch("rebuild",rng.choice([60000,240000]),rng.choice([5500,9000,13000]))
    stretch("plan",rng.choice([1000,3000]),rng.choice([700,1400]))
    stretch("execution",rng.choice([20000,90000,400000]),rng.choice([400,900,1700,3200]))
    transitions=[row for row in transitions]
    end=t
    segs=segments(readings,transitions,end)
    peak=max(s["peakMegabytes"] for s in segs)
    return {"jobs":jobs,"peakMegabytes":peak,"activeMilliseconds":end,"peakAtMilliseconds":min(s["peakAtMilliseconds"] for s in segs if s["peakMegabytes"]==peak),"endedAtMs":now-rng.randrange(0,6*86400000),"phases":segs},{"readings":readings,"transitions":transitions,"end":end}

def generate():
    policy={"estimates":base.POLICY,"memory":{"rampMilliseconds":MINIMUM_RAMP}}
    rng=random.Random(20261026)
    out={"policy":base.POLICY,"segmentCases":[],"recordCases":[],"estimateCases":[],"resizeCases":[],"slotCases":[],"floorCases":[],"replay":[]}
    for index in range(60):
        run,raw=random_run(rng,1_000_000_000)
        out["segmentCases"].append({"name":f"segments-{index}","readings":raw["readings"],"transitions":raw["transitions"],"end":raw["end"],"expect":run["phases"]})
    out["segmentCases"].append({"name":"unmarked","readings":[[1000,500,None]],"transitions":[],"end":9000,"expect":[]})
    out["segmentCases"].append({"name":"late closing reading","readings":[[1000,300,None],[5000,400,"plan"],[20000,9000,"plan"],[20500,100,"execution"]],"transitions":[{"phase":"plan","atMilliseconds":4000},{"phase":"execution","atMilliseconds":20200}],"end":30000,"expect":segments([[1000,300,None],[5000,400,"plan"],[20000,9000,"plan"],[20500,100,"execution"]],[{"phase":"plan","atMilliseconds":4000},{"phase":"execution","atMilliseconds":20200}],30000)})
    lineages=[["run","run:test","run:test:@a/b:test"],["run","run:build","run:build:@a/c:build"],["show","show:projects"],["cargo","cargo:check"]]
    requests=[{"id":"aaaaaaaa1","operation":"run","priority":"batch","minimumJobs":1,"preferredJobs":4,"fixedMegabytes":1536,"perJobMegabytes":2048,"cpuPerJob":1},{"id":"aaaaaaaa2","operation":"show","priority":"batch","minimumJobs":1,"preferredJobs":1,"fixedMegabytes":5120,"perJobMegabytes":0,"cpuPerJob":2}]
    for index in range(36):
        lineage=rng.choice(lineages);req=rng.choice(requests);now=1_000_000_000+rng.randrange(0,10*86400000)
        store={"version":1,"classes":{}};history=[]
        for step in range(rng.choice([0,1,2,4,8,12])):
            marked=rng.random()<0.85
            if marked:run,_=random_run(rng,now)
            else:run=random_run(rng,now,False)
            default=req["fixedMegabytes"]+run["jobs"]*req["perJobMegabytes"]
            chosen=rng.choice([lineage,lineage[:max(1,len(lineage)-1)]])
            store=record(policy,store,chosen,req["operation"],default,run)
            history.append({"lineage":chosen,"operation":req["operation"],"defaultMegabytes":default,"run":run})
        out["recordCases"].append({"name":f"record-{index}","history":history,"expect":store})
        est=estimate(policy,store,lineage,req,now)
        out["estimateCases"].append({"name":f"estimate-{index}","lineage":lineage,"request":req,"nowMs":now,"expect":est})
        for phase in PHASES:
            capacity=rng.choice([28000,12000]);held=rng.choice([est["phases"]["execution"]["megabytes"],9000,2000])
            context={"jobs":rng.choice([1,2,4]),"reservedMegabytes":held,"activeMilliseconds":rng.choice([0,3000,40000,500000])}
            out["resizeCases"].append({"request":req,"estimate":est,"capacityMegabytes":capacity,"phase":phase,"context":context,"expect":resize(policy,req,est,capacity,phase,context)})
        for floor_mb in (0,rng.choice([800,2500,5000])):
            capacity=rng.choice([28000,12000]);ratio=est["ratioBasisPoints"]
            out["floorCases"].append({"request":req,"ratioBasisPoints":ratio,"capacityMegabytes":capacity,"floorMegabytes":floor_mb,"expect":scale_floor(policy,req,ratio,capacity,floor_mb)})
        for total,floor in ((32768,4096),(16384,2048)):
            out["slotCases"].append({"totalMegabytes":total,"floorMegabytes":floor,"nowMs":now,"expect":rebuild_slots(policy,store,total,floor,now)})
    out["replay"]=replay(7,300,0.25)
    return out

if __name__=="__main__":
    target=os.path.join(HERE,"..","🧫️fixtures","🔣️.json")
    value=generate()
    if sys.argv[1:]==["--check"]:
        stored=json.load(open(target,encoding="utf8"))
        assert stored==json.loads(json.dumps(value)),"stored vectors differ from the oracle"
        print("vectors equal the oracle")
    else:
        os.makedirs(os.path.dirname(target),exist_ok=True)
        with open(target,"w",encoding="utf8") as handle:
            json.dump(value,handle,separators=(",",":"),ensure_ascii=False);handle.write("\n")
        print("wrote",{k:len(v) for k,v in value.items() if isinstance(v,list)})
