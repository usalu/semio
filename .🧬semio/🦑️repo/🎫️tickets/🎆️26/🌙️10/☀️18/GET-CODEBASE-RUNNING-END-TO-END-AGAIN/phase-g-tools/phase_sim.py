"""🧪️ Replay model of the admission size by phase against the single class of today. Inputs are what this machine recorded: the grants of the heavy fleet night (`C/🗑️generated/ram-g/trace.json`: 234 observed `run` peaks, their jobs and durations) and the stored classes (`estimates.json`). A run is drawn as assembly -> plan -> [rebuild -> plan] -> execution; the execution peak is drawn from the observed `run` peaks below 5 GB scaled per class, the graph build from the observed peaks above 5 GB (5 to 13 GB, independent of the task), the rebuild share is swept. The same stream of runs is learned online (empty store) by (a) OLD: one class on the whole tree peak, as `recordHostOutcomeV1` does today, and (b) NEW: phases (`phases.oracle.py`, equal to the TypeScript on all stored vectors). Metrics per policy: the reservation a request asks at admission, the reservation that is held but unused while the run ramps (the `unrealized` term that blocks other admissions), and the overshoot: memory really used above the reservation held at the same second."""
import collections,importlib.util,json,math,os,random,statistics,sys
ROOT="/Users/ueli/Documents/semio"
C=os.path.join(ROOT,".🧬semio","🦑️repo","🎫️tickets","🎆️26","🌙️10","☀️18","GET-CODEBASE-RUNNING-END-TO-END-AGAIN")
OV="/private/tmp/claude-501/-Users-ueli-Documents-semio/de5d5cb4-dca7-469c-b840-78655a0500ed/scratchpad/phase-g-ov"
REL=os.path.join("🧰️framework","🔨️modules","🏃️process","🏗️build","🌡️host-capacity","🧪️tests","🪜️phases","🐍️oracle","🐍️.py")
for base in (ROOT,OV):
    if os.path.exists(os.path.join(base,REL)):ORACLE=os.path.join(base,REL);break
spec=importlib.util.spec_from_file_location("phases_oracle",ORACLE);po=importlib.util.module_from_spec(spec);spec.loader.exec_module(po)
base_oracle=po.base
POLICY={"estimates":base_oracle.POLICY,"memory":{"rampMilliseconds":60000}}
TRACE=json.load(open(os.path.join(C,"🗑️generated","ram-g","trace.json"),encoding="utf8"))
RUNS=[g for g in TRACE["grants"] if g["op"]=="run"]
EXEC_POOL=sorted(g["peak"] for g in RUNS if g["peak"]<5000)
BUILD_POOL=sorted(g["peak"] for g in RUNS if g["peak"]>=5000)
JOBS=[g["jobs"] for g in RUNS];DURATIONS=sorted(g["since"] for g in RUNS)
KINDS=[("test",1536),("check",4096),("build",2048)]
TOTAL=16384;FLOOR=2048;CAPACITY=TOTAL-FLOOR

def draw_run(rng,klass,rebuild_share):
    jobs=min(rng.choice(JOBS),4);kind,per_job=KINDS[klass%3]
    scale=[0.5,0.8,1.0,1.3,0.65,1.6][klass]
    execution=max(150,int(rng.choice(EXEC_POOL)*scale*rng.uniform(0.85,1.15)))
    assembly=max(150,int(rng.gauss(380,60)));plan=max(300,int(rng.gauss(900,200)))
    rebuild=rng.random()<rebuild_share
    build=int(rng.choice(BUILD_POOL)*rng.uniform(0.9,1.1)) if rebuild else 0
    exec_s=max(20,min(rng.choice(DURATIONS),3600));build_s=rng.choice([90,180,300,600]) if rebuild else 0
    request={"id":"aaaaaaaa1","operation":"run","priority":"batch","minimumJobs":1,"preferredJobs":jobs,"fixedMegabytes":1536,"perJobMegabytes":per_job,"cpuPerJob":1}
    return {"klass":klass,"jobs":jobs,"request":request,"A":assembly,"P":plan,"R":build,"E":execution,"rebuild":rebuild,"tA":6,"tP":8,"tR":build_s,"tP2":5 if rebuild else 0,"tE":exec_s,"lineage":["run",f"run:{kind}",f"run:{kind}:class{klass}"]}

def timeline(run):
    """The second-by-second use of the tree: [(start, end, phase, megabytes)]; the plan stretch after a build still holds the builder."""
    out=[];t=0
    for phase,length,mb in (("assembly",run["tA"],run["A"]),("plan",run["tP"],run["P"]),("rebuild",run["tR"],run["R"]),("plan",run["tP2"],run["R"]*0.97 if run["rebuild"] else 0),("execution",run["tE"],run["E"])):
        if length>0:out.append((t,t+length,phase,mb));t+=length
    return out

def readings(rows,step=1):
    """One reading per second tagged with its phase (the gate asks at 15 s and at every boundary; at 1 s the model sees the true peaks, the test of the gate covers the sparse sampling)."""
    out=[]
    for start,end,phase,mb in rows:
        for at in range(start,end,max(1,step)):out.append([at*1000+500,int(mb),phase])
        out.append([end*1000,int(mb),phase])
    return out

def phased_run(run,rows):
    transitions=[];last=None
    for start,end,phase,mb in rows:
        if phase!=last:transitions.append({"phase":phase,"atMilliseconds":start*1000});last=phase
    end=rows[-1][1]*1000
    segs=po.segments(readings(rows),transitions[1:] if transitions and transitions[0]["phase"]=="assembly" else transitions,end)
    peak=max(s["peakMegabytes"] for s in segs)
    near=min(s["peakAtMilliseconds"] for s in segs if s["peakMegabytes"]*10000>=peak*9500)
    return {"jobs":run["jobs"],"peakMegabytes":peak,"activeMilliseconds":end,"peakAtMilliseconds":near,"phases":segs}

def need(request,jobs):
    return request["fixedMegabytes"]+jobs*request["perJobMegabytes"]

def simulate(rebuild_share,seed,count=900,classes=6,knobs=None):
    policy=json.loads(json.dumps(POLICY))
    if knobs:policy["estimates"].update(knobs)
    rng=random.Random(seed)
    runs=[draw_run(rng,rng.randrange(classes),rebuild_share) for _ in range(count)]
    stores={"old":{"version":1,"classes":{}},"new":{"version":1,"classes":{}}}
    metric={name:collections.defaultdict(list) for name in stores}
    now=1_900_000_000_000
    for run in runs:
        now+=120_000
        rows=timeline(run);total_peak=max(mb for _,_,_,mb in rows);covered_peak=max(run["A"],run["P"],run["E"])
        first_near=min(at for at,mb,_ in readings(rows) if mb*10000>=total_peak*9500)
        req=run["request"];default=need(req,run["jobs"])
        for name in ("old","new"):
            store=stores[name]
            if name=="old":
                est=base_oracle.estimate(policy,store,run["lineage"],now)
                scaled=base_oracle.scale(policy,req,est["ratioBasisPoints"],CAPACITY)
            else:
                est=po.estimate(policy,store,run["lineage"],req,now)
                scaled=po.scale_floor(policy,req,est["ratioBasisPoints"],CAPACITY,est["floorMegabytes"])
            ask=need(scaled,run["jobs"]);held=ask;window_start=0;window=est["rampMilliseconds"]/1000
            unreal=0.0;over_exec=0.0;over_build=0.0;over_ramp=0.0;reserved_s=0.0;used_s=0.0;rebuild_held=0;seen_build=False
            for start,end,phase,mb in rows:
                if name=="new":
                    change=po.resize(policy,req,est,CAPACITY,phase,{"jobs":run["jobs"],"reservedMegabytes":held,"activeMilliseconds":start*1000})
                    if change:held=change["reservedMegabytes"];window_start=start;window=change["rampMilliseconds"]/1000
                if phase=="rebuild":rebuild_held=held;seen_build=True
                for second in range(start,end):
                    if second<window_start+window:unreal+=max(0,held-mb)
                    reserved_s+=held;used_s+=mb
                    if mb>held:
                        if phase=="rebuild":over_build+=mb-held
                        elif not (phase=="plan" and seen_build):
                            over_exec+=mb-held
                            if second<window_start+window:over_ramp+=mb-held
            m=metric[name]
            for key,value in (("ask",ask),("rebuild",run["rebuild"]),("peak",covered_peak),("unreal",unreal),("over_exec",over_exec),("over_build",over_build),("over_ramp",over_ramp),("reserved_s",reserved_s),("used_s",used_s),("rebuild_held",rebuild_held),("rebuild_peak",run["R"])):m[key].append(value)
            if name=="old":
                stores[name]=base_oracle.record(policy,store,run["lineage"],{"operation":"run","jobs":run["jobs"],"peakMegabytes":total_peak,"defaultMegabytes":default,"activeMilliseconds":rows[-1][1]*1000,"peakAtMilliseconds":first_near,"endedAtMs":now})
            else:
                stores[name]=po.record(policy,store,run["lineage"],"run",default,{**phased_run(run,rows),"endedAtMs":now})
    return metric

def summarize(metric,warm=150):
    out={}
    for name,m in metric.items():
        idx=range(warm,len(m["ask"]));cov=[i for i in idx if not m["rebuild"][i]];reb=[i for i in idx if m["rebuild"][i]]
        row={
         "covered runs: ask at admission, mean MB":round(statistics.mean(m["ask"][i] for i in cov)),
         "covered runs: ask at admission, p90 MB":round(sorted(m["ask"][i] for i in cov)[int(0.9*len(cov))]),
         "covered runs: reserved above what they use, mean MB":round(statistics.mean(max(0,m["ask"][i]-m["peak"][i]) for i in cov)),
         "covered runs: unrealized reservation while ramping, GB*s":round(statistics.mean(m["unreal"][i] for i in cov)/1024),
         "covered runs: ask below their own peak (under-reserved), %":round(100*sum(1 for i in cov if m["ask"][i]<m["peak"][i])/len(cov),1),
         "all runs: memory used above the reservation outside the build, GB*s per run":round(sum(m["over_exec"][i] for i in idx)/len(idx)/1024,1),
         "all runs: memory used above the reservation while the reservation is what the ledger counts (ramp window), GB*s per run":round(sum(m["over_ramp"][i] for i in idx)/len(idx)/1024,1),
         "all runs: reserved GB*s per used GB*s":round(sum(m["reserved_s"][i] for i in idx)/sum(m["used_s"][i] for i in idx),2),
         "all runs: memory used above the reservation, build included, GB*s per run":round(sum(m["over_exec"][i]+m["over_build"][i] for i in idx)/len(idx)/1024,1),
        }
        if reb:
            row["rebuilding runs: ask at admission, mean MB"]=round(statistics.mean(m["ask"][i] for i in reb))
            row["rebuilding runs: reservation held during the build, mean MB"]=round(statistics.mean(m["rebuild_held"][i] for i in reb))
            row["rebuilding runs: build above its reservation, mean MB*s"]=round(statistics.mean(m["over_build"][i] for i in reb))
            row["rebuilding runs: build peak above its reservation, %"]=round(100*sum(1 for i in reb if m["rebuild_peak"][i]>m["rebuild_held"][i])/len(reb),1)
        out[name]=row
    return out

def mean_rows(rows):
    return {name:{k:round(statistics.mean(r[name][k] for r in rows if k in r[name]),1) for k in rows[0][name]} for name in ("old","new")}

def one(job):
    share,seed,knobs=job
    return summarize(simulate(share,seed,knobs=knobs))

def run_all(seeds=range(1,6)):
    import multiprocessing
    jobs=[(share,seed,None) for share in (0.0,0.1,0.25,0.5) for seed in seeds]+[(0.25,seed,{"quantileBasisPoints":q,"marginBasisPoints":m}) for q,m in ((9500,2500),(9800,4000),(9900,5000)) for seed in seeds]
    with multiprocessing.Pool(3) as pool:rows=pool.map(one,jobs)
    table={};iso={}
    for share in (0.0,0.1,0.25,0.5):table[share]=mean_rows([r for j,r in zip(jobs,rows) if j[0]==share and j[2] is None])
    iso["q9000+2500"]=table[0.25]
    for q,m in ((9500,2500),(9800,4000),(9900,5000)):iso[f"q{q}+{m}"]=mean_rows([r for j,r in zip(jobs,rows) if j[2] and j[2]["quantileBasisPoints"]==q])
    return table,iso

if __name__=="__main__":
    table,iso=run_all()
    out=os.path.join(C,"🗑️generated","phase-g");os.makedirs(out,exist_ok=True)
    json.dump({"share":table,"iso":iso},open(os.path.join(out,"sim.json"),"w"),indent=1)
    lines=[]
    for share,row in table.items():
        lines.append(f"## rebuild share {share:.2f} (900 runs x 5 seeds, first 150 runs of each stream = learning, not counted)")
        for key in row["old"]:lines.append(f"{key} | OLD {row['old'][key]} | NEW {row['new'].get(key)}")
    lines.append("## iso-risk sweep at rebuild share 0.25 (quantile bp + margin bp of the policy)")
    for label,row in iso.items():
        for key in ("covered runs: ask at admission, mean MB","covered runs: ask below their own peak (under-reserved), %","all runs: memory used above the reservation while the reservation is what the ledger counts (ramp window), GB*s per run","all runs: memory used above the reservation, build included, GB*s per run","rebuilding runs: build peak above its reservation, %"):
            lines.append(f"{label} | {key} | OLD {row['old'].get(key)} | NEW {row['new'].get(key)}")
    open(os.path.join(out,"sim.txt"),"w").write("\n".join(lines)+"\n");print("\n".join(lines))
