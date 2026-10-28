"""🌤️ Independent Python oracle of the quiet follow-up: a run that follows a finished run of its class is forecast quiet (the earlier run left its results cached), quiet grants ignore the soft stops (raised memory pressure, load, open files, attached images; the hard stops still hold), declared envelopes size multi-worker targets (end-to-end suites) at what their workers hold, the test-worker policy counts on the admitted envelope, and a declared host profile shrinks the machine every planner sees. It replays the exact sequence of gate-g row 4c from the recorded classes (`../🧫️fixtures/📼️recorded.json`): the hot no-op sized `200% of the default from 12 earlier runs` that waited 165+ s for 7168 MB. It reuses the repo oracles of the decision and the forecast (`../../🔼️upgrade/🐍️oracle/🐍️.py`, which uses the estimates oracle, whose weighted quantile is cross-checked against numpy). Run `python3 🐍️.py` to regenerate `../🧫️fixtures/🔣️.json`, `python3 🐍️.py --check` to prove the stored vectors are what it produces."""
import importlib.util,json,math,os,random,re,sys

HERE=os.path.dirname(os.path.abspath(__file__))
TESTS=os.path.join(HERE,"..","..")
TARGET=os.path.join(HERE,"..","🧫️fixtures","🔣️.json")
RECORDED=os.path.join(HERE,"..","🧫️fixtures","📼️recorded.json")
DECISION=os.path.join(TESTS,"🧫️fixtures","🔣️.json")
WORKER_RUNNERS=[
 {"id":"bun-test","override":"SEMIO_BUN_TEST_WORKERS","baseMegabytes":128,"workerMegabytes":1024,"wanted":1,"maximum":8,"oversubscription":1},
 {"id":"vitest","override":"SEMIO_VITEST_WORKERS","baseMegabytes":1200,"workerMegabytes":100,"wanted":4,"maximum":8,"oversubscription":4},
 {"id":"cargo-test","override":"SEMIO_CARGO_TEST_THREADS","baseMegabytes":256,"workerMegabytes":192,"wanted":"cores","maximum":32,"oversubscription":4},
 {"id":"playwright","override":"SEMIO_PLAYWRIGHT_WORKERS","baseMegabytes":256,"workerMegabytes":1200,"wanted":4,"maximum":6,"oversubscription":1},
 {"id":"e2e-stacks","override":"SEMIO_E2E_STACKS","baseMegabytes":256,"workerMegabytes":1500,"wanted":3,"maximum":4,"oversubscription":1}]

def load(path,name):
    spec=importlib.util.spec_from_file_location(name,path);module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module);return module
UP=load(os.path.join(TESTS,"🔼️upgrade","🐍️oracle","🐍️.py"),"upgrade_oracle")
EST=UP.load_estimates()
rnd=lambda value:math.floor(value+0.5)

def policies():
    base=json.load(open(DECISION,encoding="utf8"))["policy"]
    new=json.loads(json.dumps(base))
    new["memory"]["quietSoftStopGrants"]=1
    new["estimates"]["quietFollowUpMilliseconds"]=1_800_000
    new["envelopes"]=[{"name":"end-to-end","pattern":"^test-e2e(?:-|$)","cpu":4,"megabytes":6144}]
    return base,new

def floor_of(policy,total):return UP.floor_mb(policy,total)

def sized_ask(policy,store,lineage,now,total,operation="run",jobs=1,kinds=("build",),mode=None):
    """The request a run of this class asks for at the door, scaled by what its class learned (a port of `sizeHostRequestV1` + `scaleHostRequestV1`)."""
    op=next(row for row in policy["operations"] if row["name"]==operation)
    rows=[policy["kinds"][kind] for kind in sorted(set(kinds)|{"build"})]
    per_job=max(row["megabytes"] for row in rows);cpu=max(row["cpu"] for row in rows)
    base={"fixedMegabytes":op["fixedMegabytes"],"perJobMegabytes":per_job,"minimumJobs":1,"preferredJobs":jobs,"cpuPerJob":cpu}
    estimate=EST.estimate(policy,store,lineage,now);ratio=estimate["ratioBasisPoints"]
    preferred=base["fixedMegabytes"]+jobs*per_job
    if mode and preferred>0:
        bound,source=EST.class_bound(policy,store,lineage,now);grown=max(1,min(ratio,bound*10000//preferred))
        ratio=grown if mode=="grow" or not (source=="fallback" and estimate["samples"]==0) else ratio
    capacity=max(0,total-floor_of(policy,total));rules=policy["estimates"]
    scaled_per_job=rnd(base["perJobMegabytes"]*ratio/10000);fixed=rnd(base["fixedMegabytes"]*ratio/10000)
    fixed+=max(0,rules["minimumMegabytes"]-(fixed+base["minimumJobs"]*scaled_per_job))
    ceiling=capacity*rules["maximumShareBasisPoints"]//10000;need=fixed+base["minimumJobs"]*scaled_per_job
    if need>ceiling:fixed=fixed*ceiling//need;scaled_per_job=scaled_per_job*ceiling//need
    return {"ratioBasisPoints":ratio,"samples":estimate["samples"],"fixedMegabytes":fixed,"perJobMegabytes":scaled_per_job,"needMegabytes":fixed+base["minimumJobs"]*scaled_per_job,"defaultMegabytes":base["fixedMegabytes"]+jobs*per_job}

def ident(rng):return UP.ident(rng)

COURSE="run:build:@teaching/architecture-course:build"
def bound_vectors(rng,policy):
    """Full requests of classes that are declared, measured or neither: the bound of `hostClassBoundV1` caps the learned ratio, so a class never asks for more than its measured peak (declared ceiling, generic fallback) whatever 200 % of its default says."""
    now=1_800_000_000_000;cases=[];sources={"measured":0,"declared":0,"fallback":0};binding=0;cold=0
    for index in range(144):
        leaf=rng.choice([COURSE,"run:build:@semio-tech/repo-dashboard-rs:build","run:test:@semio-tech/print-rs:test"]);lineage=["run","run:"+leaf.split(":")[1],leaf]
        classes={}
        for key in lineage:
            rows=[]
            for _ in range(rng.choice([0,2,3,6,12])):
                row={"atMs":now-rng.randint(0,5*86_400_000),"ratioBasisPoints":rng.choice([6000,12000,19000,20000,25000]),"activeMilliseconds":rng.choice([9000,200000,900000]),"peakAtMilliseconds":rng.choice([1000,100000,400000])}
                if rng.random()<0.6:row["peakMegabytes"]=rng.choice([900,2400,4600,4800,6300,9800,14000])
                rows.append(row)
            rows.sort(key=lambda row:-row["atMs"])
            if rows:classes[key]={"samples":rows}
        if index%12==0:classes={}
        declared=[{"name":"course-build","pattern":"^run:build:@teaching/architecture-course:build$","megabytes":12288}] if rng.random()<0.5 else []
        fallback=rng.choice([4096,8192,12288]);total=rng.choice([16384,32768,61440]);jobs=rng.choice([1,2,4,8,16])
        local=json.loads(json.dumps(policy));local["estimates"]["fallbackCeilingMegabytes"]=fallback;local["classCeilings"]=declared
        store={"version":1,"classes":classes}
        bound,source=EST.class_bound(local,store,lineage,now);sources[source]+=1
        free=sized_ask(local,store,lineage,now,total,jobs=jobs);first=sized_ask(local,store,lineage,now,total,jobs=jobs,mode="initial");grown=sized_ask(local,store,lineage,now,total,jobs=jobs,mode="grow")
        full=lambda row:row["fixedMegabytes"]+jobs*row["perJobMegabytes"]
        binding+=full(grown)<full(free);cold+=free["samples"]==0 and full(first)==full(free) and full(grown)<full(free)
        cases.append({"lineage":lineage,"nowMs":now,"totalMegabytes":total,"jobs":jobs,"fallbackCeilingMegabytes":fallback,"classCeilings":declared,"store":store,"expect":{"source":source,"bound":bound,"initial":{"ratioBasisPoints":first["ratioBasisPoints"],"fullMegabytes":full(first)},"grow":{"ratioBasisPoints":grown["ratioBasisPoints"],"fixedMegabytes":grown["fixedMegabytes"],"perJobMegabytes":grown["perJobMegabytes"],"fullMegabytes":full(grown)}}})
    assert all(sources.values()) and binding>=20 and cold>=5,(sources,binding,cold)
    return {"cases":cases,"sources":sources,"binding":binding,"cold":cold}

def host_row(total,available,pressure,load,cores=10,now=1_800_000_000_000):
    return {"version":1,"atMs":now,"platform":"darwin","cores":cores,"totalMegabytes":total,"availableMegabytes":available,"pressure":pressure,"swapUsedMegabytes":0,"swapOutMegabytesPerSecond":0,"load":load,"openFiles":8000,"openFilesMaximum":245760,"attachedImages":10}

def request_row(identifier,fixed,per_job=0,quiet=False):
    row={"id":identifier,"operation":"run","priority":"batch","minimumJobs":1,"preferredJobs":1,"fixedMegabytes":fixed,"perJobMegabytes":per_job,"cpuPerJob":1}
    if quiet:row.update({"rampSlot":False,"quiet":True,"rampMilliseconds":60000})
    return row

def replay_4c():
    """Gate-g row 4c, step by step, on the recorded classes and the policy of the build library at that time (the follow-up knobs off = the old behaviour, on = the new)."""
    recorded=json.load(open(RECORDED,encoding="utf8"));leaf=recorded["leaf"];lineage=recorded["lineage"];host=recorded["host"]
    old=json.loads(json.dumps(recorded["policy"]));old["memory"]["quietSoftStopGrants"]=0;old["estimates"]["quietFollowUpMilliseconds"]=0;old["envelopes"]=[]
    new=json.loads(json.dumps(recorded["policy"]));new["memory"]["quietSoftStopGrants"]=1;new["estimates"]["quietFollowUpMilliseconds"]=1_800_000;new["envelopes"]=[]
    store={"version":1,"classes":{key:{"samples":rows} for key,rows in recorded["classes"].items()}}
    last=max(row["atMs"] for row in store["classes"][leaf+"~busy"]["samples"])
    steps=[];total=host["totalMegabytes"]
    def forecast_row(policy,at,store_):
        value=UP.forecast(policy,EST,store_,leaf,at);return value
    now=last+90_000
    ask=sized_ask(old,store,lineage,now,total)
    def world(queue,policy_name,pressure,images):
        row=host_row(total,host["availableMegabytes"],pressure,host["load"],host["cores"],now);row["attachedImages"]=images
        return {"policy":policy_name,"host":row,"ledger":[],"queue":queue,"nowMs":now}
    full=request_row("aaaaaaaa1",ask["fixedMegabytes"],ask["perJobMegabytes"])
    steps.append({"name":"run 2 at the door, before the change","nowMs":now,"ask":ask,"forecast":forecast_row(old,now,store)})
    steps.append({"name":"run 2 at the door, follow-up rule","nowMs":now,"ask":ask,"forecast":forecast_row(new,now,store)})
    quiet_fc=forecast_row(new,now,store);quiet=UP.quiet_request(full,quiet_fc)
    worlds=[]
    for name,policy_name,policy,pressure,images,queue in (
        ("full request, raised pressure (the bench2-g log)","old",old,"warn",10,[{"request":full,"arrivedAtMs":now,"pid":4242}]),
        ("quiet request, the soft stop holds it","old",old,"warn",10,[{"request":quiet,"arrivedAtMs":now,"pid":4242}]),
        ("quiet request, the soft stop is ignored","new",new,"warn",10,[{"request":quiet,"arrivedAtMs":now,"pid":4242}]),
        ("full request, raised pressure, new policy","new",new,"warn",10,[{"request":full,"arrivedAtMs":now,"pid":4242}]),
        ("quiet request, attached images at the maximum, the soft stop holds it","old",old,"normal",32,[{"request":quiet,"arrivedAtMs":now,"pid":4242}]),
        ("quiet request, attached images at the maximum, ignored","new",new,"normal",32,[{"request":quiet,"arrivedAtMs":now,"pid":4242}]),
        ("quiet request, critical pressure is a hard stop","new",new,"critical",10,[{"request":quiet,"arrivedAtMs":now,"pid":4242}])):
        w=world(queue,policy_name,pressure,images);w["name"]=name;w["expect"]=UP.evaluate(policy,w["host"],w["ledger"],w["queue"],now);worlds.append(w)
    # the chain goes on: run 2 finishes quiet, runs 3 and 4 follow
    session=json.loads(json.dumps(store));t=now;sequence=[]
    for index,(executed,peak) in enumerate(((False,1180),(False,1210),(True,2790),(False,1190))):
        run={"executed":executed,"peakMegabytes":peak,"activeMilliseconds":9000,"peakAtMilliseconds":4000,"endedAtMs":t+20_000}
        session=UP.record_forecast(new,EST,session,leaf,run);t=run["endedAtMs"]+60_000
        sequence.append({"name":f"run {index+2} {'executes' if executed else 'executes nothing'}","run":run,"nextNowMs":t,"expect":forecast_row(new,t,session)})
    gap=forecast_row(new,t+1_800_000+1,session)
    return {"leaf":leaf,"lineage":lineage,"store":store,"host":host_row(total,host["availableMegabytes"],"warn",host["load"],host["cores"],now),"steps":steps,"worlds":worlds,"sequence":sequence,"afterWindow":{"nowMs":t+1_800_000+1,"expect":gap},"finalStore":session}

def forecast_vectors(rng,new):
    cases=UP.stores_for_forecast(rng,new,EST)
    return cases

def decision_vectors(rng,off,on):
    worlds=[];changed=0
    for index in range(220):
        world=UP.random_world(rng,off,index);world["name"]=f"followup-{index}"
        world["expect"]=UP.evaluate(on,world["host"],world["ledger"],world["queue"],world["nowMs"])
        plain=UP.evaluate(off,world["host"],world["ledger"],world["queue"],world["nowMs"])
        if plain["decisions"]!=world["expect"]["decisions"]:changed+=1
        worlds.append(world)
    return worlds,changed

def envelope_vectors(off,on):
    table={row["id"]:row for row in WORKER_RUNNERS}
    def plan_workers(runner_id,cores,host_cores,memory):
        runner=table[runner_id];each=max(1,runner["workerMegabytes"]);base=runner["baseMegabytes"]
        asked=host_cores if runner["wanted"]=="cores" else runner["wanted"];wanted=max(1,min(asked,runner["maximum"]))
        memory_cap=(memory-base)//each if memory>=base+each else 0;core_cap=cores*runner["oversubscription"]
        workers=max(1,min(wanted,memory_cap,core_cap))
        bound="memory" if memory_cap<wanted and memory_cap<=core_cap else "cores" if core_cap<wanted else ("maximum" if asked>runner["maximum"] else "wanted")
        return {"runner":runner_id,"workers":workers,"bound":bound}
    cases=[]
    for name,policy,operation,kinds,targets,jobs in (
        ("e2e run, no envelope declared",off,"run",["test"],["test-e2e"],1),
        ("e2e run, envelope declared",on,"run",["test"],["test-e2e"],1),
        ("e2e dev variant, envelope declared",on,"run",["test"],["test-e2e-dev"],1),
        ("unit tests are not end-to-end",on,"run",["test"],["test"],1),
        ("journeys are not end-to-end",on,"run",["test"],["test-journeys"],1),
        ("run-many with one e2e target",on,"run-many",["build","test"],["build","test-e2e"],4),
        ("show is a fixed block, no envelope",on,"show",[],["test-e2e"],1),
        ("two jobs of an e2e target",on,"run",["test"],["test-e2e"],2)):
        op=next(row for row in policy["operations"] if row["name"]==operation)
        if op["shape"]=="fixed":
            request={"id":"aaaaaaaa1","operation":operation,"priority":op["priority"],"minimumJobs":1,"preferredJobs":1,"fixedMegabytes":op["fixedMegabytes"],"perJobMegabytes":0,"cpuPerJob":op["cpu"]}
        else:
            rows=[policy["kinds"][kind] for kind in sorted(set(([op["kind"]] if "kind" in op else kinds))|{"build"})]
            rows+=[{"cpu":env["cpu"],"megabytes":env["megabytes"]} for env in policy["envelopes"] if any(re.search(env["pattern"],target) for target in targets)]
            request={"id":"aaaaaaaa1","operation":operation,"priority":op["priority"],"minimumJobs":1,"preferredJobs":max(1,min(jobs,1024)),"fixedMegabytes":op["fixedMegabytes"],"perJobMegabytes":max(row["megabytes"] for row in rows),"cpuPerJob":max(row["cpu"] for row in rows)}
        granted_jobs=2 if name.startswith("two jobs") else 1
        plan={"jobs":granted_jobs,"cpu":granted_jobs*request["cpuPerJob"],"memoryMegabytes":granted_jobs*request["perJobMegabytes"],"fixedMegabytes":request["fixedMegabytes"]}
        facts={"cores":max(1,plan["cpu"]//plan["jobs"]),"memoryMegabytes":plan["memoryMegabytes"]//plan["jobs"] if plan["memoryMegabytes"]>0 else plan["fixedMegabytes"]}
        runners=[plan_workers(id_,facts["cores"],10,facts["memoryMegabytes"]) for id_ in ("e2e-stacks","playwright","vitest")]
        cases.append({"name":name,"policy":"on" if policy is on else "off","operation":operation,"kinds":kinds,"targets":targets,"jobs":jobs,"grantedJobs":granted_jobs,"expect":{"request":request,"plan":plan,"facts":facts,"workers":runners}})
    return cases

def profile_vectors():
    real={"totalMegabytes":32768,"cores":10}
    def number(text,digits):return int(text) if text is not None and re.fullmatch(r"[1-9][0-9]{%d,8}"%(digits-1),text) else None
    cases=[]
    for name,environment in (("nothing declared",{}),("total only",{"SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES":"16384"}),("cores only",{"SEMIO_HOST_CAPACITY_CORES":"4"}),
        ("both",{"SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES":"16384","SEMIO_HOST_CAPACITY_CORES":"8"}),("a declaration only shrinks the memory",{"SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES":"99999"}),
        ("a declaration only shrinks the cores",{"SEMIO_HOST_CAPACITY_CORES":"64"}),("fewer than four digits is the real host",{"SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES":"999"}),("a leading zero is the real host",{"SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES":"016384"}),
        ("text is the real host",{"SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES":"16 GB","SEMIO_HOST_CAPACITY_CORES":"many"}),("zero cores is the real host",{"SEMIO_HOST_CAPACITY_CORES":"0"}),("empty values are the real host",{"SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES":"","SEMIO_HOST_CAPACITY_CORES":""}),
        ("one malformed value does not void the other",{"SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES":"abc","SEMIO_HOST_CAPACITY_CORES":"2"})):
        total=number(environment.get("SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES"),4);cores=number(environment.get("SEMIO_HOST_CAPACITY_CORES"),1)
        key="" if total is None and cores is None else "%s/%s"%(total if total is not None else "-",cores if cores is not None else "-")
        cases.append({"name":name,"environment":environment,"real":real,"expect":{"totalMegabytes":min(real["totalMegabytes"],total) if total is not None else real["totalMegabytes"],"cores":min(real["cores"],cores) if cores is not None else real["cores"],"declared":total is not None or cores is not None},"key":key})
    return cases

def generate():
    off,on=policies();UP.COUNTS.clear();rng=random.Random(20261026)
    replay=replay_4c()
    forecasts=[]
    for item in forecast_vectors(rng,on):forecasts.append(item)
    decisions,changed=decision_vectors(rng,off,on)
    # sanity facts the law relies on
    first=replay["steps"][0];second=replay["steps"][1]
    assert first["ask"]["ratioBasisPoints"]==20000 and first["ask"]["samples"]==12 and first["ask"]["needMegabytes"]==7168,first
    assert first["forecast"]["quiet"] is False and second["forecast"]["quiet"] is True and "followUpMilliseconds" in second["forecast"]
    return {"policy":{"replay":{"old":{"quietSoftStopGrants":0,"quietFollowUpMilliseconds":0,"envelopes":[]},"new":{"quietSoftStopGrants":1,"quietFollowUpMilliseconds":1_800_000,"envelopes":[]}},"off":{"quietSoftStopGrants":0,"quietFollowUpMilliseconds":0,"envelopes":[]},"on":{"quietSoftStopGrants":1,"quietFollowUpMilliseconds":1_800_000,"envelopes":on["envelopes"]}},"replay":replay,"bounds":bound_vectors(random.Random(20261027),on),"forecasts":forecasts,"decisions":decisions,"decisionsChangedByTheFlag":changed,"workerPolicy":{"version":1,"hostShareBasisPoints":5000,"runners":WORKER_RUNNERS},"envelopes":envelope_vectors(off,on),"profiles":profile_vectors(),"forecastCoverage":{"quiet":sum(1 for item in forecasts if item["expect"]["quiet"]),"followUp":sum(1 for item in forecasts if "followUpMilliseconds" in item["expect"]),"busy":sum(1 for item in forecasts if not item["expect"]["quiet"])}}

def dump(value):return json.dumps(value,separators=(",",":"),ensure_ascii=False)+"\n"

if __name__=="__main__":
    value=generate();text=dump(value)
    if sys.argv[1:]==["--check"]:
        stored=open(TARGET,encoding="utf8").read()
        print("vectors equal the oracle" if stored==text else "STALE vectors");sys.exit(0 if stored==text else 1)
    with open(TARGET,"w",encoding="utf8") as handle:handle.write(text)
    print("wrote",len(value["decisions"]),"decisions,",len(value["forecasts"]),"forecasts,",len(value["envelopes"]),"envelope cases,",len(value["profiles"]),"profiles; flag changed",value["decisionsChangedByTheFlag"],"decisions;",value["forecastCoverage"])
