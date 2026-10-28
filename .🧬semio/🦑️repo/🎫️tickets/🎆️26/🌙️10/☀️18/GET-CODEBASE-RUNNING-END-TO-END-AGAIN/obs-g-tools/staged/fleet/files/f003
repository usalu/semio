#!/usr/bin/env python3
"""🐍️ Independent oracle of the fleet status aggregation. It shares no code with `../🟦️.ts`: it reads only the closed schema (policy and EN+DE texts) and the lease ETA policy, takes facts as JSON and produces the report as JSON.
usage: 🐍️.py summarize <facts.json> <en|de>            print the report of one facts file
       🐍️.py generate <seed> <count>                    print `count` JSON lines {id, language, facts, report} of seeded random facts
Python 3.9 compatible (no match statements)."""
import json,os,random,statistics,sys
HERE=os.path.dirname(os.path.abspath(__file__))
SCHEMA=json.load(open(os.path.join(HERE,"..","🧬️schema","🔣️.json"),encoding="utf8"))
STANDING=json.load(open(os.path.join(HERE,*([".."]*7),"🔨️modules","🏃️process","🔒️leases","📈️standing","🧬️schema","🔣️.json"),encoding="utf8"))
POLICY=SCHEMA["definitions"]["Policy"]["const"];MESSAGES=SCHEMA["definitions"]["Messages"]["const"];ETA=STANDING["definitions"]["Policy"]["const"]["eta"]
INF=float("inf")

def fill(template,values):
    out=template
    for key,value in values.items(): out=out.replace("{"+key+"}",str(value))
    return out
def text(language,key,values=None): return fill(MESSAGES[key][language],values or {})
def duration(language,seconds):
    if seconds<120: return text(language,"unit.s",{"n":seconds})
    if seconds<7200: return text(language,"unit.min",{"n":int(seconds/60+0.5)})
    tenths=int(seconds/360+0.5);return text(language,"unit.h",{"n":"%d.%d"%(tenths//10,tenths%10)})
def gib(value):
    tenths=(value*10+2**29)//2**30;return "%d.%d"%(tenths//10,tenths%10)
def short(resource):
    limit=POLICY["runs"]["resourceCharacters"]
    return resource if len(resource)<=limit else "…"+resource[-(limit-1):]
def median_seconds(durations):
    rows=[x for x in (durations or [])][-ETA["maximumSamples"]:]
    if len(rows)<ETA["minimumSamples"]: return None
    return int(statistics.median(sorted(rows))/1000+0.5)
def starts(text_,prefixes): return any(text_.startswith(p) for p in prefixes)

def lease_estimates(facts):
    out={};leases=facts.get("leases")
    if not leases: return out
    by={}
    for w in leases["waiters"]: by.setdefault(w["resource"],[]).append(w)
    for resource,ws in by.items():
        times=[(h.get("standing") or {}).get("etaMinutes") for h in leases["holders"] if h["resource"]==resource]
        if not times: remaining=0
        elif any(t is None for t in times): remaining=None
        else: remaining=max(t*60 for t in times)
        ordered=sorted(ws,key=lambda w:(w["position"] if "position" in w else INF,-w["waitedSeconds"],w["pid"]))
        ahead=0
        for w in ordered:
            if not w["blockers"]: out[(w["pid"],resource)]=0
            elif remaining is None or ahead is None: out[(w["pid"],resource)]=None
            else: out[(w["pid"],resource)]=remaining+ahead
            service=median_seconds(facts.get("history",{}).get(w["historyKey"])) if "historyKey" in w else None
            ahead=None if ahead is None or service is None else ahead+service
    return out

def capacity_estimates(facts):
    out={};cap=facts.get("capacity")
    if not cap: return out
    events=[]
    for g in cap["grants"]:
        typical=median_seconds(facts.get("activeMilliseconds",{}).get(g["operation"]))
        if typical is not None: events.append((max(0,typical-g["heldSeconds"]),g["reservedMegabytes"]))
    events.sort()
    free=cap["headroomMegabytes"];now=0;unknown=False
    for w in sorted(cap["waiters"],key=lambda w:(w["position"],w["id"])):
        if unknown: out[w["id"]]=None;continue
        need=min(w["needMegabytes"],cap["capacityMegabytes"]) if cap["capacityMegabytes"]>0 else w["needMegabytes"]
        while events and events[0][0]<=now: free+=events.pop(0)[1]
        while free<need and events:
            t,mb=events.pop(0);now=max(now,t);free+=mb
            while events and events[0][0]<=now: free+=events.pop(0)[1]
        if free<need: unknown=True;out[w["id"]]=None;continue
        out[w["id"]]=now;free-=need
        typical=median_seconds(facts.get("activeMilliseconds",{}).get(w["operation"]))
        if typical is not None: events.append((now+typical,need));events.sort()
    return out

def runs_of(facts,lw,cw):
    leases=facts.get("leases") or {"holders":[],"waiters":[],"cycles":[]};cap=facts.get("capacity") or {"grants":[],"waiters":[]}
    H,W,G,M=leases["holders"],leases["waiters"],cap["grants"],cap["waiters"]
    known=set(r["pid"] for r in H+W+G+M);parent={}
    for h in H:
        s=h.get("standing")
        if s is None or h["pid"] in parent: continue
        up=s.get("parentPid")
        if up is None and s.get("chain"): up=s["chain"][-1]
        if up is not None: parent[h["pid"]]=up
    def root(pid):
        at=pid
        for _ in range(POLICY["runs"]["parentWalk"]):
            up=parent.get(at)
            if up is None or up==at or up not in known: break
            at=up
        return at
    groups={}
    def group(pid):
        r=root(pid);return groups.setdefault(r,{"pids":set(),"H":[],"W":[],"G":[],"M":[]})
    for kind,rows in (("H",H),("W",W),("G",G),("M",M)):
        for r in rows:
            g=group(r["pid"]);g["pids"].add(r["pid"]);g[kind].append(r)
    sd=sorted([h["standing"] for h in H if h.get("standing") and "implementationDigest" in h["standing"] and "implementationAgeSeconds" in h["standing"]],key=lambda s:(s["implementationAgeSeconds"],s["implementationDigest"]))
    newest=sd[0]["implementationDigest"] if sd else None
    runs=[]
    for r,g in groups.items():
        def mine(rows): return sorted(rows,key=lambda x:(0 if x["pid"]==r else 1,x["pid"]))
        rows=mine(g["H"])+mine(g["W"])+mine(g["G"])+mine(g["M"])
        command=next((x["command"] for x in rows if x.get("command")),None);session=next((x["session"] for x in rows if x.get("session")),None);agent=next((x["agent"] for x in rows if x.get("agent")),None)
        grant=sorted(g["G"],key=lambda x:(-x["heldSeconds"],x["pid"]))[0] if g["G"] else None
        def horder(x): return (0 if x["mode"]=="exclusive" else 1,-x["heldSeconds"],x["pid"])
        standing_row=sorted([x for x in g["H"] if x.get("standing") is not None],key=horder)
        standing_row=standing_row[0] if standing_row else None
        detail_row=sorted([x for x in g["H"] if (x.get("standing") or {}).get("phase") is not None],key=horder)
        detail=detail_row[0]["standing"] if detail_row else None
        waits=[("lease",x["pid"],x["waitedSeconds"],short(x["resource"]),lw.get((x["pid"],x["resource"])),x.get("priority")) for x in g["W"]]+[("capacity",x["pid"],x["waitedSeconds"],x["operation"],cw.get(x["id"]),None) for x in g["M"]]
        waits.sort(key=lambda w:(-w[2],w[1],w[0]))
        ages=[x["heldSeconds"] for x in g["H"]]+[x["waitedSeconds"] for x in g["W"]]+[x["heldSeconds"] for x in g["G"]]+[x["waitedSeconds"] for x in g["M"]]
        obs=[x for x in g["G"] if x["observedMegabytes"] is not None]
        admitted=None if not g["G"] else sum(x["reservedMegabytes"] for x in g["G"]);observed=None if not obs else sum(x["observedMegabytes"] for x in obs)
        run={"pid":r,"pids":sorted(g["pids"]),"state":"running" if not waits else ("waiting" if len(g["H"])+len(g["G"])==0 else "blocked"),"ageSeconds":max(ages) if ages else 0}
        if agent is not None: run["agent"]=agent
        if session is not None: run["session"]=session
        if command is not None: run["command"]=command[:POLICY["runs"]["commandCharacters"]]
        if grant: run["operation"]=grant["operation"]
        phase=detail.get("phase") if detail else (grant.get("phase") if grant else None)
        if phase is not None: run["phase"]=phase
        if detail and "done" in detail: run["done"]=detail["done"]
        if detail and "total" in detail: run["total"]=detail["total"]
        chosen=(standing_row["standing"] if standing_row else None) or detail
        if chosen and "etaMinutes" in chosen: run["etaSeconds"]=chosen["etaMinutes"]*60
        if chosen and chosen.get("etaOverdue"): run["etaOverdue"]=True
        priority=(chosen or {}).get("priority")
        if priority is None: priority=next((w[5] for w in waits if w[5] is not None),None)
        if priority is not None: run["priority"]=priority
        if standing_row:
            s=standing_row["standing"];digest=s.get("implementationDigest")
            impl={"state":s["state"],"freshness":"unknown" if digest is None else ("current" if digest==newest else "older")}
            if digest is not None: impl["digest"]=digest[:POLICY["digestCharacters"]]
            if "implementationAgeSeconds" in s: impl["ageSeconds"]=s["implementationAgeSeconds"]
            run["implementation"]=impl
        run["memory"]={"admittedMegabytes":admitted,"observedMegabytes":observed,"deltaMegabytes":None if admitted is None or observed is None else observed-admitted,"jobs":sum(x["jobs"] for x in g["G"])}
        if waits: run["waitingFor"]={"kind":waits[0][0],"subject":waits[0][3],"waitedSeconds":waits[0][2],"estimatedWaitSeconds":waits[0][4]}
        ex={}
        for x in sorted([x for x in g["H"] if x["mode"]=="exclusive"],key=lambda x:(-x["heldSeconds"],x["resource"])):
            if x["resource"] not in ex: ex[x["resource"]]=short(x["resource"])
        run["exclusive"]=list(ex.values())[:POLICY["runs"]["exclusiveShown"]]
        if any(x.get("lent") is True for x in g["G"]): run["lent"]=True
        runs.append(run)
    order={"running":0,"blocked":1,"waiting":2}
    runs.sort(key=lambda x:(order[x["state"]],-x["ageSeconds"],x["pid"]))
    return runs

def natural_key(slot):
    import re
    m=re.match(r"^([a-z]*)(\d+)$",slot)
    return (0,m.group(1),int(m.group(2)),slot) if m else (1,"",0,slot)

def summarize(facts,language):
    lw=lease_estimates(facts);cw=capacity_estimates(facts);runs=runs_of(facts,lw,cw)
    leases=facts.get("leases");cap=facts.get("capacity");at=facts["atMs"]
    grants=cap["grants"] if cap else [];obs=[g for g in grants if g["observedMegabytes"] is not None]
    host=facts.get("host")
    memory={"totalMegabytes":host["totalMegabytes"] if host else 0,"availableMegabytes":host["availableMegabytes"] if host else 0,"floorMegabytes":cap["floorMegabytes"] if cap else 0,"spareMegabytes":cap["headroomMegabytes"] if cap else 0,"pressure":host["pressure"] if host else "normal","swapUsedMegabytes":host["swapUsedMegabytes"] if host else None,"load":host["load"] if host else None,"cores":host["cores"] if host else None,"admittedMegabytes":sum(g["reservedMegabytes"] for g in grants),"observedMegabytes":sum(g["observedMegabytes"] for g in obs),"unobservedGrants":len(grants)-len(obs),"overReservedMegabytes":sum(max(0,g["reservedMegabytes"]-g["observedMegabytes"]) for g in obs),"underReservedMegabytes":sum(max(0,g["observedMegabytes"]-g["reservedMegabytes"]) for g in obs),"grants":len(grants),"waiters":len(cap["waiters"]) if cap else 0}
    by={}
    for w in (leases["waiters"] if leases else []): by.setdefault(w["resource"],[]).append(w)
    queues=[]
    for resource,ws in by.items():
        queues.append((-max(w["waitedSeconds"] for w in ws),resource,sorted(ws,key=lambda w:(w["position"] if "position" in w else INF,-w["waitedSeconds"],w["pid"]))))
    queues.sort(key=lambda q:(q[0],q[1]))
    shown=queues[:POLICY["queues"]["maximumLeaseQueues"]];lq=[]
    for _,resource,ws in shown:
        holders=sorted([h for h in leases["holders"] if h["resource"]==resource],key=lambda h:(-h["heldSeconds"],h["pid"]))
        hs=[]
        for h in holders:
            row={"pid":h["pid"],"mode":h["mode"],"heldSeconds":h["heldSeconds"],"etaSeconds":None if (h.get("standing") or {}).get("etaMinutes") is None else h["standing"]["etaMinutes"]*60}
            if h.get("standing") is not None: row["state"]=h["standing"]["state"]
            hs.append(row)
        wl=[]
        for w in ws:
            row={"pid":w["pid"],"mode":w["mode"],"position":w.get("position"),"waitedSeconds":w["waitedSeconds"],"estimatedWaitSeconds":lw.get((w["pid"],resource))}
            if "priority" in w: row["priority"]=w["priority"]
            wl.append(row)
        lq.append({"resource":short(resource),"class":ws[0]["class"],"holders":hs,"waiters":wl})
    H=leases["holders"] if leases else [];W=leases["waiters"] if leases else []
    g=POLICY["graph"]
    builders=sorted([h for h in H if h["mode"]=="exclusive" and starts(h["resource"],g["builderResources"])],key=lambda h:(-h["heldSeconds"],h["pid"]))
    waiting=sorted([w for w in W if starts(w["resource"],g["builderResources"]+g["claimResources"]+g["slotResources"])],key=lambda w:(-w["waitedSeconds"],w["pid"]))
    builder=None
    if builders:
        b=builders[0];s=b.get("standing") or {}
        builder={"pid":b["pid"],"heldSeconds":b["heldSeconds"],"state":s.get("state","legacy"),"etaSeconds":None if s.get("etaMinutes") is None else s["etaMinutes"]*60}
        for key in ("phase","done","total"):
            if key in s: builder[key]=s[key]
    graph={"builder":builder,"computers":len(set(h["pid"] for h in H if starts(h["resource"],g["claimResources"]))),"slots":len(set((h["pid"],h["resource"]) for h in H if starts(h["resource"],g["slotResources"]))),"waiting":len(set(w["pid"] for w in waiting)),"longestWaitSeconds":waiting[0]["waitedSeconds"] if waiting else 0,"estimatedWaitSeconds":lw.get((waiting[0]["pid"],waiting[0]["resource"])) if waiting else None}
    bz=facts.get("bazel") or {"slotPrefix":"","slots":[]};prefix=bz["slotPrefix"];holders={}
    for h in H:
        if prefix and h["resource"].startswith(prefix) and h["mode"]=="exclusive": holders[h["resource"][len(prefix):]]=h["pid"]
    servers=[]
    for s in sorted(bz["slots"],key=lambda s:natural_key(s["slot"])):
        row={"slot":s["slot"],"live":s["pid"] is not None,"held":s["slot"] in holders,"pid":s["pid"],"holderPid":holders.get(s["slot"]),"treeMegabytes":s["treeMegabytes"]}
        for key in ("key","usedAt","agent"):
            if key in s: row[key]=s[key]
        servers.append(row)
    st=facts.get("storage")
    images=[]
    for im in sorted(st["images"] if st else [],key=lambda im:im["id"]):
        row=dict(im);row["usedBasisPoints"]=None if im["usedBytes"] is None or im["capacityBytes"] is None or im["capacityBytes"]==0 else im["usedBytes"]*10000//im["capacityBytes"];images.append(row)
    sealed=[]
    for e in (facts.get("sealed") or {"entries":[]})["entries"]:
        row={"entry":e["entry"],"digest":e["digest"][:POLICY["digestCharacters"]],"files":e["files"],"bytes":e["bytes"],"promotedAtMs":e["promotedAtMs"],"ageSeconds":max(0,(at-e["promotedAtMs"])//1000),"fresh":e["fresh"],"changed":e["changed"]}
        if "blocked" in e: row["blocked"]=e["blocked"]
        if "last" in e: row["last"]=e["last"]
        sealed.append(row)
    sealed.sort(key=lambda r:r["entry"])
    gen=facts.get("generation");generation=None
    if gen:
        v=gen["verified"]
        generation={"number":gen["number"],"built":gen["built"],"owners":gen["owners"],"implementation":gen["implementation"][:POLICY["digestCharacters"]],"publishedAtMs":gen["publishedAtMs"],"ageSeconds":None if gen["publishedAtMs"] is None else max(0,(at-gen["publishedAtMs"])//1000),"verified":None if v is None else {"atMs":v["atMs"],"ageSeconds":max(0,(at-v["atMs"])//1000),"covered":v["covered"],"count":v["count"],"basisPoints":0 if v["count"]==0 else v["covered"]*10000//v["count"]}}
    findings=[]
    def say(id_,sev,subject,values): findings.append({"id":id_,"severity":sev,"subject":subject,"values":values,"text":text(language,"finding."+id_,values)})
    for c in (leases["cycles"] if leases else []): say("cycle","alert",c,{"subject":c})
    seen=set()
    for h in sorted(H,key=lambda h:(h["pid"],h["resource"])):
        s=h.get("standing");state=s["state"] if s else None;key=(h["pid"],state)
        if state in ("stuck","stale") and key not in seen:
            seen.add(key)
            if state=="stuck": say("holder-stuck","alert","pid %d"%h["pid"],{"pid":h["pid"],"resource":short(h["resource"]),"minutes":(s.get("silentSeconds",0)+30)//60})
            else: say("holder-stale","warn","pid %d"%h["pid"],{"pid":h["pid"],"resource":short(h["resource"]),"ids":", ".join(s.get("staleIds",[]))})
    if host and host["pressure"]!="normal": say("memory-pressure","alert" if host["pressure"]=="critical" else "warn","host",{"pressure":host["pressure"]})
    for g_ in sorted(grants,key=lambda g_:(g_["pid"],g_["id"])):
        if g_["observedMegabytes"] is not None and g_["reservedMegabytes"]>0 and g_["observedMegabytes"]*10000>g_["reservedMegabytes"]*POLICY["findings"]["underReservedBasisPoints"]: say("under-reserved","warn","pid %d"%g_["pid"],{"pid":g_["pid"],"observed":g_["observedMegabytes"],"reserved":g_["reservedMegabytes"]})
    for w in sorted(cap["waiters"] if cap else [],key=lambda w:w["position"]):
        if w["waitedSeconds"]>=POLICY["findings"]["capacityWaitSeconds"]: say("capacity-wait","warn","pid %d"%w["pid"],{"pid":w["pid"],"waited":duration(language,w["waitedSeconds"]),"operation":w["operation"],"need":w["needMegabytes"]})
    waited={}
    for w in W:
        k=(w["pid"],w["resource"])
        if w["waitedSeconds"]>=POLICY["findings"]["leaseWaitSeconds"] and (k not in waited or waited[k]["waitedSeconds"]<w["waitedSeconds"]): waited[k]=w
    for w in sorted(waited.values(),key=lambda w:(-w["waitedSeconds"],w["pid"])): say("lease-wait","warn","pid %d"%w["pid"],{"pid":w["pid"],"waited":duration(language,w["waitedSeconds"]),"resource":short(w["resource"])})
    if st:
        if st["hostFreeBytes"] is not None and st["hostFreeBytes"]<st["hostFreeFloorBytes"]: say("disk-low","alert","host",{"free":gib(st["hostFreeBytes"]),"floor":gib(st["hostFreeFloorBytes"])})
        if st["machine"]["overBytes"]>0: say("disk-over-budget","warn","machine",{"over":gib(st["machine"]["overBytes"])})
        for im in images:
            if im["usedBasisPoints"] is not None and im["usedBasisPoints"]>=POLICY["findings"]["imageFullBasisPoints"]: say("image-full","warn",im["id"],{"id":im["id"],"percent":im["usedBasisPoints"]//100})
    for e in sealed:
        if "blocked" in e: say("sealed-blocked","alert",e["entry"],{"entry":e["entry"],"first":e["blocked"].get("first",str(e["blocked"]["rows"]))})
        elif e["fresh"] is False: say("sealed-drifted","info",e["entry"],{"entry":e["entry"],"changed":e["changed"]})
    for s in facts["sources"]:
        if not s["ok"]: say("source-failed","warn",s["id"],{"section":s["id"],"reason":s.get("error","?")})
    sev={"alert":0,"warn":1,"info":2}
    findings.sort(key=lambda f:(sev[f["severity"]],f["id"],f["subject"]))
    count=lambda state:len([r for r in runs if r["state"]==state])
    return {"version":1,"atMs":at,"workspace":facts["workspace"],
     "summary":{"running":count("running"),"blocked":count("blocked"),"waiting":count("waiting"),"alerts":len([f for f in findings if f["severity"]=="alert"]),"warnings":len([f for f in findings if f["severity"]=="warn"])},
     "memory":memory,"runs":runs[:POLICY["runs"]["maximumRows"]],
     "queues":{"graph":graph,"capacity":{"floorMegabytes":cap["floorMegabytes"] if cap else 0,"spareMegabytes":cap["headroomMegabytes"] if cap else 0,"grants":len(grants),"waiters":[{"id":w["id"],"pid":w["pid"],"operation":w["operation"],"needMegabytes":w["needMegabytes"],"waitedSeconds":w["waitedSeconds"],"reason":w["reason"],"reasonText":w["reasonText"],"position":w["position"],"estimatedWaitSeconds":cw.get(w["id"])} for w in sorted(cap["waiters"] if cap else [],key=lambda w:w["position"])]},"leases":lq,"leasesOmitted":max(0,len(queues)-len(shown))},
     "bazel":{"slots":len(servers),"live":len([s for s in servers if s["live"]]),"held":len([s for s in servers if s["held"]]),"rssMegabytes":sum(s["treeMegabytes"] or 0 for s in servers),"servers":servers},
     "storage":{"hostFreeBytes":st["hostFreeBytes"] if st else None,"hostFreeFloorBytes":st["hostFreeFloorBytes"] if st else 0,"machine":st["machine"] if st else {"totalBytes":0,"effectiveTotalBytes":0,"usedBytes":0,"footprintBytes":0,"overBytes":0,"evictBytes":0,"blockedBytes":0,"source":"default"},"overAreas":sorted(st["overAreas"]) if st else [],"images":images},
     "sealed":{"mode":(facts.get("sealed") or {}).get("mode","live"),"drifted":len([e for e in sealed if e["fresh"] is False]),"blocked":len([e for e in sealed if "blocked" in e]),"entries":sealed},
     "generation":generation,"findings":findings[:POLICY["findings"]["maximum"]],"sources":facts["sources"]}

def generate(seed,count):
    rnd=random.Random(seed);out=[]
    ops=["run","cargo","show","graph","test"];resources=["build-graph-state:/w:selected:7","repository-graph-facts:/w:shard:7","bazel-slot-s0","bazel-slot-s1","nx:daemon-slot:1","graph-claim:/w:tok1","graph-claim-slot:/w:0","cargo-preparation:/w","rustup:home"]
    for index in range(count):
        language=rnd.choice(["en","de"]);pids=rnd.sample(range(100,400),rnd.randint(2,9));at=1_793_000_000_000+rnd.randint(0,10**9)
        def standing(pid):
            if rnd.random()<0.2: return None
            s={"state":rnd.choice(["progressing","progressing","silent","stuck","stale","legacy"])}
            if rnd.random()<0.8: s["priority"]=rnd.choice(["interactive","agent","background"])
            if rnd.random()<0.7: s["phase"]=rnd.choice(["inference","execution","loading","cargo:workspace"])
            if rnd.random()<0.5: s["total"]=rnd.randint(1,500);s["done"]=rnd.randint(0,s["total"])
            if rnd.random()<0.4: s["etaMinutes"]=rnd.randint(1,90)
            elif rnd.random()<0.2: s["etaOverdue"]=True
            if rnd.random()<0.9: s["implementationDigest"]=rnd.choice(["a"*16,"b"*16,"c"*16]);s["implementationAgeSeconds"]=rnd.randint(1,5000)
            if rnd.random()<0.4: s["parentPid"]=rnd.choice(pids)
            elif rnd.random()<0.2: s["chain"]=[rnd.choice(pids),rnd.choice(pids)]
            if s["state"]=="stuck": s["silentSeconds"]=rnd.randint(1200,7200)
            if s["state"]=="stale": s["staleIds"]=rnd.sample(["a","b","c"],rnd.randint(1,2))
            return s
        holders=[];waiters=[];identity={}
        states={pid:standing(pid) for pid in pids}
        for pid in pids:
            identity[pid]={}
            if rnd.random()<0.8: identity[pid]["command"]=rnd.choice(["bun repo run x","/Users/u/w/cache","hash123"])*rnd.randint(1,4)
            if rnd.random()<0.7: identity[pid]["session"]="aa.d"
            if rnd.random()<0.6: identity[pid]["agent"]=rnd.choice(["default","codex","ueli"])
        for pid in pids:
            for _ in range(rnd.randint(0,3)):
                h={"pid":pid,"resource":rnd.choice(resources),"mode":rnd.choice(["shared","exclusive"]),"class":rnd.choice(["graph","image","tool-singleton"]),"heldSeconds":rnd.randint(0,9000)}
                h.update(identity[pid])
                s=states[pid]
                if s is not None: h["standing"]=dict(s)
                holders.append(h)
            if rnd.random()<0.35:
                w={"pid":pid,"resource":rnd.choice(resources),"mode":rnd.choice(["shared","exclusive"]),"class":"graph","waitedSeconds":rnd.randint(0,3000),"blockers":rnd.sample(["1/q","2/3"],rnd.randint(0,2))}
                w.update(identity[pid])
                if rnd.random()<0.8: w["position"]=rnd.randint(1,6)
                if rnd.random()<0.4: w["priority"]=rnd.choice(["interactive","agent","background"])
                if rnd.random()<0.8: w["historyKey"]=rnd.choice(["k1","k2","k3"])
                waiters.append(w)
        # keep per resource at most one exclusive holder shape irrelevant; the aggregation is defined for any input
        grants=[];cwait=[]
        for pid in rnd.sample(pids,rnd.randint(0,len(pids))):
            if rnd.random()<0.6:
                g={"id":"g%d"%pid,"operation":rnd.choice(ops),"pid":pid,"jobs":rnd.randint(1,4),"reservedMegabytes":rnd.randint(500,9000),"observedMegabytes":None if rnd.random()<0.3 else rnd.randint(100,14000),"heldSeconds":rnd.randint(0,3000)}
                if rnd.random()<0.3: g["phase"]=rnd.choice(["plan","execution"])
                if rnd.random()<0.2: g["lent"]=True
                if rnd.random()<0.5: g["agent"]=rnd.choice(["default","codex"])
                grants.append(g)
            else:
                cwait.append({"id":"w%d"%pid,"operation":rnd.choice(ops),"pid":pid,"needMegabytes":rnd.randint(500,12000),"waitedSeconds":rnd.randint(0,2500),"reason":rnd.choice(["memory","queue-order","load"]),"reasonText":"why","position":0})
        rnd.shuffle(cwait)
        for position,w in enumerate(cwait,1): w["position"]=position
        total=16384;floor=2048
        facts={"version":1,"atMs":at,"workspace":"/w",
         "host":{"platform":"darwin","cores":rnd.choice([None,8,10]),"totalMegabytes":total,"availableMegabytes":rnd.randint(500,9000),"pressure":rnd.choice(["normal","normal","warn","critical"]),"swapUsedMegabytes":rnd.choice([None,0,4000]),"load":rnd.choice([None,3.5,22.25])},
         "leases":{"holders":holders,"waiters":waiters,"cycles":rnd.sample(["1/q > 2/q","3/q > 4/q"],rnd.randint(0,1))},
         "capacity":{"floorMegabytes":floor,"headroomMegabytes":rnd.randint(-3000,6000),"capacityMegabytes":total-floor,"grants":grants,"waiters":cwait},
         "activeMilliseconds":{op:[rnd.randint(2000,900000) for _ in range(rnd.randint(0,40))] for op in ops if rnd.random()<0.8},
         "history":{k:[rnd.randint(1000,2_000_000) for _ in range(rnd.randint(0,5))] for k in ["k1","k2","k3"]},
         "bazel":{"slotPrefix":"bazel-slot-","slots":[dict({"slot":"s%d"%i,"pid":rnd.choice([None,rnd.randint(1000,2000)]),"treeMegabytes":rnd.choice([None,300,800])},**({"key":"k%d"%i} if rnd.random()<0.5 else {}),**({"usedAt":at-rnd.randint(0,10**6)} if rnd.random()<0.5 else {})) for i in rnd.sample(range(0,12),rnd.randint(0,6))]},
         "storage":{"hostFreeBytes":rnd.choice([None,rnd.randint(5,90)*2**30]),"hostFreeFloorBytes":15*2**30,"images":[{"id":name,"kind":"partition","state":"mounted","usedBytes":rnd.randint(0,8*2**30),"capacityBytes":rnd.choice([None,8*2**30,2**33]),"budgetBytes":rnd.choice([None,2**33]),"source":"statfs"} for name in rnd.sample(["tools","graph","products","rust","nx"],rnd.randint(0,4))],"overAreas":rnd.sample(["go","playwright","cargo"],rnd.randint(0,2)),"machine":{"totalBytes":40*2**30,"effectiveTotalBytes":40*2**30,"usedBytes":rnd.randint(1,90)*2**30,"footprintBytes":rnd.randint(1,120)*2**30,"overBytes":rnd.choice([0,0,3*2**30]),"evictBytes":rnd.choice([0,2**30]),"blockedBytes":0,"source":rnd.choice(["default","file","argument","environment"])}},
         "sealed":{"mode":rnd.choice(["sealed","live","promote"]),"entries":[dict({"entry":"e%d"%i,"digest":"d"*64,"files":rnd.randint(100,2000),"bytes":rnd.randint(10**6,2*10**7),"promotedAtMs":at-rnd.randint(0,10**8),"fresh":rnd.choice([True,False,None]),"changed":rnd.randint(0,5)},**({"blocked":{"atMs":at-5,"rows":rnd.randint(1,4)}} if rnd.random()<0.2 else {}),**({"last":{"atMs":at-9,"added":1,"removed":0,"changed":2}} if rnd.random()<0.5 else {})) for i in range(rnd.randint(0,3))]},
         "generation":rnd.choice([None,{"number":rnd.randint(1,900),"built":{"kind":rnd.choice(["cold","delta","refresh"]),"dirty":rnd.randint(0,50)},"owners":4019,"implementation":"f"*64,"publishedAtMs":rnd.choice([None,at-rnd.randint(0,10**7)]),"verified":rnd.choice([None,{"atMs":at-rnd.randint(0,10**6),"covered":rnd.randint(0,4019),"count":4019}])}]),
         "sources":[{"id":name,"ok":rnd.random()>0.1,"milliseconds":rnd.randint(0,300),**({"error":"boom"} if rnd.random()<0.2 else {})} for name in ["leases","capacity","bazel","storage","sealed","generation"]]}
        for s in facts["sources"]:
            if s["ok"]: s.pop("error",None)
        if facts["generation"] is None: del facts["generation"]
        out.append({"id":"seed%d-%d"%(seed,index),"language":language,"facts":facts,"report":summarize(facts,language)})
    return out

if __name__=="__main__":
    if sys.argv[1]=="summarize":
        print(json.dumps(summarize(json.load(open(sys.argv[2],encoding="utf8")),sys.argv[3]),ensure_ascii=False))
    elif sys.argv[1]=="generate":
        for row in generate(int(sys.argv[2]),int(sys.argv[3])): print(json.dumps(row,ensure_ascii=False))
    else: raise SystemExit(__doc__)
