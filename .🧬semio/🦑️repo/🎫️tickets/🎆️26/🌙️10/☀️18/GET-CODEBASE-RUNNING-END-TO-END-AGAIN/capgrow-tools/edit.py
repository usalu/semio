"""🌱️ capgrow: anchored edits that let a reduced grant grow and size a compile from what its grant carries; `python3 -I edit.py <root>` applies them (idempotent, refuses on a missing anchor), so the batch can be rebuilt on top of a newer live tree."""
import json,os,sys

ROOT=sys.argv[1]
HC="🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity"
CU="🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🧮️units"
LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"

def edit(rel,pairs):
    path=os.path.join(ROOT,rel);text=open(path,encoding="utf8").read();mode=os.stat(path).st_mode&0o7777;changed=False
    for old,new in pairs:
        if new in text:continue
        if text.count(old)!=1:sys.exit(f"anchor missing or not unique in {rel}: {old[:120]!r}")
        text=text.replace(old,new);changed=True
    if changed:
        temporary=path+".capgrow.tmp"
        with open(temporary,"w",encoding="utf8") as handle:handle.write(text)
        os.chmod(temporary,mode);os.replace(temporary,path)
    print(("edited " if changed else "already ")+rel)

def edit_json(rel,change):
    path=os.path.join(ROOT,rel);value=json.load(open(path,encoding="utf8"));mode=os.stat(path).st_mode&0o7777
    before=json.dumps(value,sort_keys=True);change(value)
    if json.dumps(value,sort_keys=True)==before:print("already "+rel);return
    temporary=path+".capgrow.tmp"
    with open(temporary,"w",encoding="utf8") as handle:handle.write(json.dumps(value,indent=2,ensure_ascii=False)+"\n")
    os.chmod(temporary,mode);os.replace(temporary,path);print("edited "+rel)

DECISION=HC+"/📏️decision/🟦️.ts"
GROW_RULE=" A grow request (`grow`, which only the holder of a grant that runs with fewer jobs than it asked for evaluates, never written to the queue) is decided after every waiting request and takes nothing from them: it waits `queue-order` while any other request waits or the queue is held behind a barrier, `ramp` while the grant still ramps up (its last reservation is not yet observed, so it cannot grow twice on the same memory), a soft stop like any request, and otherwise grows to the most jobs whose memory fits with the memory of `growSpareJobs` more jobs left free (the protocol's hysteresis margin) and whose cores fit the load room; it never shrinks a reservation, never becomes a barrier and never takes the partial path of an upgrade."
edit(DECISION,[
 ("keeps the queue position the run arrived with. */\nexport type HostWaiterV1=Readonly<{request:HostRequestV1;arrivedAtMs:number;pid:number;session?:string;weight?:number;upgrade?:boolean}>;",
  "keeps the queue position the run arrived with; `grow` (with `upgrade`) marks the request of a running grant admitted with fewer jobs than it asked for to grow its jobs now that memory freed up. */\nexport type HostWaiterV1=Readonly<{request:HostRequestV1;arrivedAtMs:number;pid:number;session?:string;weight?:number;upgrade?:boolean;grow?:boolean}>;"),
 ("and `rampSlot` marks a shrunk reservation that occupies a ramp slot while it ramps. */",
  "and `rampSlot` marks a shrunk reservation that occupies a ramp slot while it ramps; `grownAtMs` is when a grant admitted with fewer jobs than it asked for last grew its jobs. */"),
 ("arrivedAtMs?:number;upgradedAtMs?:number;phase?:",
  "arrivedAtMs?:number;upgradedAtMs?:number;grownAtMs?:number;phase?:"),
 ("waiters without a session count as holding nothing), then by arrival, then by id. */",
  "waiters without a session count as holding nothing), then by arrival, then by id; grow requests of running grants come after every other request. */"),
 ("const rank=(waiter:HostWaiterV1)=>waiter.request.priority===\"interactive\"?0:1,",
  "const rank=(waiter:HostWaiterV1)=>waiter.grow===true?2:waiter.request.priority===\"interactive\"?0:1,"),
 ("because it is small, bounded and every hard stop still applies to it. */",
  "because it is small, bounded and every hard stop still applies to it."+GROW_RULE+" */"),
 ("import {resolveLanguageV1,type OsLanguageListV1} from \"../../🗣️language/🟦️.ts\";\n",
  "import {resolveLanguageV1,type OsLanguageListV1} from \"../../🗣️language/🟦️.ts\";\n\nconst protocol=schema.definitions.Protocol.const;\n"),
 ("let blocked=false,admitted=0,own=ledgerCpu,",
  "let blocked=false,waits=false,admitted=0,own=ledgerCpu,"),
 ("  const request=waiter.request,need=(jobs:number)=>Math.min(request.fixedMegabytes+jobs*request.perJobMegabytes,cap),waited=nowMs-waiter.arrivedAtMs,quiet=request.quiet===true,",
  "  const request=waiter.request,need=(jobs:number)=>Math.min(request.fixedMegabytes+jobs*request.perJobMegabytes,cap),waited=nowMs-waiter.arrivedAtMs,quiet=request.quiet===true,grows=waiter.grow===true&&waiter.upgrade===true,"),
 ("  const refuse=(reason:HostReasonV1):void=>{decisions.push(",
  "  const refuse=(reason:HostReasonV1):void=>{if(!grows)waits=true;decisions.push("),
 ("  if(hard){refuse(hard);continue;}\n",
  "  if(hard){refuse(hard);continue;}\n  if(grows){\n   if(blocked||waits){refuse(\"queue-order\");continue;}\n   if(replaced===undefined||ramping(replaced)){refuse(\"ramp\");continue;}\n   if(soft!==undefined){refuse(soft);continue;}\n   const roomy=(jobs:number)=>{const wanted=Math.min(request.fixedMegabytes+(jobs+protocol.growSpareJobs)*request.perJobMegabytes,cap);return Math.max(0,wanted-held)<=freeObserved&&Math.max(wanted,held)<=freeBudget;};\n   let jobs=0;for(let candidate=request.preferredJobs;candidate>=request.minimumJobs;candidate--)if(roomy(candidate)&&candidate*request.cpuPerJob<=freeLoad){jobs=candidate;break;}\n   if(jobs>0)admit(jobs,jobs<request.preferredJobs?\"reduced\":\"fits\",Math.max(replaced.reservedMegabytes,need(jobs)));else refuse(roomy(request.minimumJobs)?\"load\":\"memory\");\n   continue;\n  }\n"),
 ("\"upgradedPartly\"|\"upgradeWithdrawn\"|\"grantQuiet\"|\"waiterUpgrade\"|\"reduced\"|",
  "\"upgradedPartly\"|\"upgradeWithdrawn\"|\"grantQuiet\"|\"waiterUpgrade\"|\"reduced\"|\"reducedGrows\"|\"grown\"|"),
 ("/** 📏️ Sizes the request of one named operation,",
  "/** 🪴️ How many consecutive grow attempts of one grant were admitted and the fewest jobs any of them admitted. */\nexport type HostGrowthSettleV1=Readonly<{attempts:number;jobs:number}>;\n\n/** ⏲️ The time hysteresis of a growth: a grant grows only after `growSettleAttempts` consecutive attempts (one per heartbeat) were admitted, and then to the fewest jobs any of them admitted; one refused attempt starts the count again, so memory that frees up for a moment never grows a run.\n * @see ../🧪️tests/🔼️upgrade/🌱️grow/🐍️oracle/🐍️.py */\nexport function settleHostGrowthV1(previous:HostGrowthSettleV1|undefined,decision:Pick<HostDecisionV1,\"verdict\"|\"jobs\">):Readonly<{next:HostGrowthSettleV1|undefined;jobs:number|undefined}>{\n if(decision.verdict!==\"admit\")return Object.freeze({next:undefined,jobs:undefined});\n const next=Object.freeze({attempts:(previous?.attempts??0)+1,jobs:Math.min(previous?.jobs??decision.jobs,decision.jobs)});\n return next.attempts>=protocol.growSettleAttempts?Object.freeze({next:undefined,jobs:next.jobs}):Object.freeze({next,jobs:undefined});\n}\n\n/** 🧭️ The plan a process carries, followed to the grant row as it is now: fewer jobs after a partial growth, more after a grant admitted with fewer jobs grew, the cores of the grant and the memory the jobs may hold in proportion; the same plan when the row is quiet, belongs to another grant or holds the same jobs. */\nexport function followHostGrantPlanV1(plan:HostPlanV1,grant:Pick<HostGrantV1,\"id\"|\"jobs\"|\"cpu\"|\"quiet\">):HostPlanV1{\n if(grant.id!==plan.id||grant.quiet===true||grant.jobs===plan.jobs)return plan;\n return requireHostCapacityV1<HostPlanV1>(\"Plan\",Object.freeze({...plan,jobs:grant.jobs,cpu:grant.cpu,memoryMegabytes:Math.floor(plan.memoryMegabytes*grant.jobs/plan.jobs)}));\n}\n\n/** 📏️ Sizes the request of one named operation,"),
])

TEXTS={
 "en":{"reducedGrows":"[capacity] admitted {operation} with {jobs} of {preferred} job(s): the host cannot carry more right now; the run grows to more jobs once memory frees up",
       "grown":"[capacity] {operation}: grown from {from} to {jobs} of {preferred} job(s) after {seconds}s, reservation {before} MB -> {reserved} MB: memory freed up and nothing of ours waits ({available} MB available, {floor} MB floor); work that starts from now on uses the new jobs"},
 "de":{"reducedGrows":"[capacity] {operation} mit {jobs} von {preferred} Job(s) zugelassen: der Rechner traegt gerade nicht mehr; der Lauf waechst auf mehr Jobs, sobald Speicher frei wird",
       "grown":"[capacity] {operation}: nach {seconds} s von {from} auf {jobs} von {preferred} Job(s) gewachsen, Reservierung {before} MB -> {reserved} MB: Speicher ist frei geworden und nichts von uns wartet ({available} MB verfuegbar, Untergrenze {floor} MB); Arbeit, die ab jetzt startet, nutzt die neuen Jobs"},
}
def host_schema(value):
    d=value["definitions"];d["Protocol"]["const"]["growSpareJobs"]=1;d["Protocol"]["const"]["growSettleAttempts"]=2
    d["Waiter"]["properties"]["grow"]={"type":"boolean"}
    d["Grant"]["properties"]["grownAtMs"]={"type":"integer","minimum":0}
    message=d["Message"]
    for name in ("reducedGrows","grown"):
        if name not in message["required"]:message["required"].append(name)
        message["properties"][name]={"type":"string"}
    for language,texts in TEXTS.items():d["Messages"]["const"][language].update(texts)
edit_json(HC+"/🧬️schema/🔣️.json",host_schema)

UPGRADE=HC+"/🔼️upgrade/🟦️.ts"
edit(UPGRADE,[
 ("import {describeHostReasonV1,evaluateHostCapacityV1,formatHostCapacityV1,isHostCapacityV1,requireHostCapacityV1,type HostCapacityPolicyV1,type HostGrantV1,type HostPlanV1,type HostReasonV1,type HostRequestV1,type HostSampleV1,type HostWaiterV1} from \"../📏️decision/🟦️.ts\";\nimport {hostCapacityDirectoryV1,hostUpgradePathV1,readHostCapacityPlanV1} from \"../🎫️gate/🟦️.ts\";",
  "import {describeHostReasonV1,evaluateHostCapacityV1,formatHostCapacityV1,isHostCapacityV1,requireHostCapacityV1,settleHostGrowthV1,type HostCapacityPolicyV1,type HostGrantV1,type HostGrowthSettleV1,type HostReasonV1,type HostRequestV1,type HostSampleV1,type HostWaiterV1} from \"../📏️decision/🟦️.ts\";\nimport {hostCapacityDirectoryV1,hostUpgradePathV1,readHostCapacityPlanV1,readLiveHostCapacityPlanV1} from \"../🎫️gate/🟦️.ts\";"),
 ("/** 🧩️ Lets the process that waited for an upgrade take over what it was granted: when the grant row holds fewer jobs than the plan the process carries (a partial growth: the memory that was free carries fewer jobs at the learned footprint of one), the plan in its environment and the job count variable shrink to the granted jobs (the cores of the grant, the memory the jobs may hold in proportion), so the build tool it starts afterwards, its `--jobs`, local resources and workers, plans with what it was granted and never swaps the host. A plan is never raised; answers whether one was changed. */\nexport function adoptUpgradedHostPlanV1(environment:Record<string,string|undefined>):boolean{\n const id=environment[protocol.grantEnvironment],plan=readHostCapacityPlanV1(environment);\n if(!id||!plan||plan.id!==id)return false;\n let row:unknown;try{row=JSON.parse(readFileSync(join(hostCapacityDirectoryV1(environment),\"grants\",`${id}.json`),\"utf8\"));}catch{return false;}\n if(!isHostCapacityV1(\"Grant\",row))return false;\n const grant=row as HostGrantV1;\n if(grant.quiet===true||grant.jobs>=plan.jobs)return false;\n environment[protocol.planEnvironment]=JSON.stringify(requireHostCapacityV1<HostPlanV1>(\"Plan\",{...plan,jobs:grant.jobs,cpu:grant.cpu,memoryMegabytes:Math.floor(plan.memoryMegabytes*grant.jobs/plan.jobs)}));\n if(environment.SEMIO_BUILD_PARALLEL!==undefined)environment.SEMIO_BUILD_PARALLEL=String(grant.jobs);\n return true;\n}",
  "/** 🧩️ Lets a process of the run take over what its grant holds now: when the grant row holds other jobs than the plan the process carries (fewer after a partial growth, where the memory that was free carries fewer jobs at the learned footprint of one; more after a grant admitted with fewer jobs grew because memory freed up), the plan in its environment and the job count variable follow the granted jobs (the cores of the grant, the memory the jobs may hold in proportion), so the build tool it starts afterwards, its `--jobs`, local resources and workers, plans with what it was granted: never more, so it never swaps the host, and no fewer, so freed memory is used. Answers whether the plan changed. */\nexport function adoptUpgradedHostPlanV1(environment:Record<string,string|undefined>):boolean{\n const id=environment[protocol.grantEnvironment],plan=readHostCapacityPlanV1(environment);\n if(!id||!plan||plan.id!==id)return false;\n const next=readLiveHostCapacityPlanV1(environment);\n if(next===undefined||next===plan||next.jobs===plan.jobs)return false;\n environment[protocol.planEnvironment]=JSON.stringify(next);\n if(environment.SEMIO_BUILD_PARALLEL!==undefined)environment.SEMIO_BUILD_PARALLEL=String(next.jobs);\n return true;\n}"),
 ("/** 🔼️ Grows a held quiet grant into its full request in the fair queue:",
  "/** 🌿️ What the holder of a grant admitted with fewer jobs than it asked for hands to one grow attempt: the upgrade context without the asker, the full request it may grow into and the time hysteresis it carries between attempts. */\nexport type HostGrowContextV1=Omit<HostUpgradeContextV1,\"request\"|\"control\"|\"requested\"|\"stopped\"|\"session\">&Readonly<{full:HostRequestV1;settled:()=>HostGrowthSettleV1|undefined;settle:(next:HostGrowthSettleV1|undefined)=>void}>;\n\n/** 🌱️ One grow attempt of a running grant that holds fewer jobs than its full request: under the gate lock it decides a grow request (`grow`, never written to the queue, so no other run ever waits for it) in the world of the live grants and the queue; the decision lets it grow only while nothing else of ours waits, after its last reservation was observed (the ramp) and with the memory of the protocol's spare jobs left free, and `settleHostGrowthV1` grows it only after consecutive admitted attempts, to the fewest jobs they admitted. A growth rewrites the grant row in one atomic replace with more jobs and cores, a reservation that never shrinks and a restarted ramp, and says so in the language of the run; answers the jobs it grew to, else undefined. */\nexport async function growHostGrantV1(context:HostGrowContextV1):Promise<number|undefined>{\n const {ledger,directory,policy,full,now,pid,language,write}=context,host=await ledger.sample();\n if(!ledger.takeLock(directory,now(),policy,pid))return undefined;\n try{\n  const at=now(),state=context.state();\n  if(state.quiet===true||state.lentSinceMs!==undefined||state.jobs>=full.preferredJobs)return undefined;\n  const request=requireHostCapacityV1<HostRequestV1>(\"Request\",{...full,minimumJobs:state.jobs+1}),grants=ledger.readGrants(directory,at,policy),waiters=ledger.readWaiters(directory,at,policy).filter(row=>row.request.id!==request.id);\n  const evaluation=evaluateHostCapacityV1(policy,host,grants,[...waiters,requireHostCapacityV1<HostWaiterV1>(\"Waiter\",{request,arrivedAtMs:state.arrivedAtMs??state.admittedAtMs,pid,upgrade:true,grow:true})],at),mine=evaluation.decisions.find(row=>row.id===request.id)!,settled=settleHostGrowthV1(context.settled(),mine);\n  context.settle(settled.next);\n  if(settled.jobs===undefined)return undefined;\n  const reserved=Math.max(state.reservedMegabytes,Math.min(full.fixedMegabytes+settled.jobs*full.perJobMegabytes,Math.max(0,host.totalMegabytes-evaluation.floorMegabytes)));\n  context.publish(requireHostCapacityV1<HostGrantV1>(\"Grant\",{...state,jobs:settled.jobs,cpu:settled.jobs*full.cpuPerJob,reservedMegabytes:reserved,resumedAtMs:at,grownAtMs:at}));\n  write(formatHostCapacityV1(language,\"grown\",{operation:full.operation,from:state.jobs,jobs:settled.jobs,preferred:full.preferredJobs,before:state.reservedMegabytes,reserved,seconds:Math.round((at-state.admittedAtMs)/1000),available:host.availableMegabytes,floor:evaluation.floorMegabytes}));\n  return settled.jobs;\n }finally{rmSync(join(directory,\"lock\"),{recursive:true,force:true});}\n}\n\n/** 🔼️ Grows a held quiet grant into its full request in the fair queue:"),
])

GATE=HC+"/🎫️gate/🟦️.ts"
edit(GATE,[
 ("import {decideHostHandoverV1,describeHostHandoverRefusalV1,describeHostReasonV1,evaluateHostCapacityV1,formatHostCapacityV1,",
  "import {decideHostHandoverV1,describeHostHandoverRefusalV1,describeHostReasonV1,evaluateHostCapacityV1,followHostGrantPlanV1,formatHostCapacityV1,"),
 ("type HostEvaluationV1,type HostGrantV1,type HostHandoverV1,",
  "type HostEvaluationV1,type HostGrantV1,type HostGrowthSettleV1,type HostHandoverV1,"),
 (" return requireHostCapacityV1<HostPlanV1>(\"Plan\",value);\n}\n",
  " return requireHostCapacityV1<HostPlanV1>(\"Plan\",value);\n}\n\n/** 🔭️ Reads the plan an enclosing grant carried into the environment, followed to the grant row as it is now (`followHostGrantPlanV1`): a process started before its run grew or partly grew sees the jobs, cores and job memory the grant holds now; the carried plan when the row cannot be read, undefined when the process runs under no grant. */\nexport function readLiveHostCapacityPlanV1(environment:Readonly<Record<string,string|undefined>>):HostPlanV1|undefined{\n const plan=readHostCapacityPlanV1(environment);\n if(plan===undefined)return undefined;\n let row:unknown;try{row=readJson(grantPath(hostCapacityDirectoryV1(environment),plan.id));}catch{return plan;}\n return isHostCapacityV1(\"Grant\",row)?followHostGrantPlanV1(plan,row as HostGrantV1):plan;\n}\n"),
 ("      else if(mine.reason===\"reduced\")write(formatHostCapacityV1(language,\"reduced\",{",
  "      else if(mine.reason===\"reduced\")write(formatHostCapacityV1(language,options.upgrade!==undefined&&options.upgrade.perJobMegabytes>0?\"reducedGrows\":\"reduced\",{"),
 (" const full=options.upgrade,quiet=held.quiet===true&&full!==undefined,upgradeFile=hostUpgradePathV1(directory,held.id);\n let upgrading=false,",
  " const full=options.upgrade,quiet=held.quiet===true&&full!==undefined,growable=full!==undefined&&!adopted&&full.perJobMegabytes>0,upgradeFile=hostUpgradePathV1(directory,held.id);\n let growing=false,settled:HostGrowthSettleV1|undefined,upgrading=false,"),
 ("options.resize?.(next,{jobs:held.jobs,reservedMegabytes:state.reservedMegabytes,",
  "options.resize?.(next,{jobs:state.jobs,reservedMegabytes:state.reservedMegabytes,"),
 (" first.unref();timer.unref();lender.unref();watcher?.unref();\n",
  " const grower=growable?setInterval(()=>{\n  if(growing||upgrading||stopped||!holding||state.quiet===true||state.lentSinceMs!==undefined||state.jobs>=full!.preferredJobs)return;\n  growing=true;\n  void import(\"../🔼️upgrade/🟦️.ts\").then(module=>module.growHostGrantV1({ledger:Object.freeze({...custody(),readGrants,readWaiters,queuePath,sample:()=>currentSample(directory,policy,options.control,options,environment)}),directory,policy,full:full!,state:()=>state,publish,now,pid,language:resolveHostCapacityLanguageV1(environment),write:options.write??(line=>process.stderr.write(line+\"\\n\")),settled:()=>settled,settle:next=>{settled=next;}})).catch(()=>undefined).finally(()=>{growing=false;});\n },policy.wait.heartbeatMilliseconds):undefined;\n first.unref();timer.unref();lender.unref();watcher?.unref();grower?.unref();\n"),
 ("  return await body(Object.freeze({id:held.id,jobs:quiet?full!.preferredJobs:held.jobs,cpu:quiet?full!.preferredJobs*full!.cpuPerJob:held.cpu,",
  "  return await body(Object.freeze({id:held.id,jobs:quiet||growable?full!.preferredJobs:held.jobs,cpu:quiet||growable?full!.preferredJobs*full!.cpuPerJob:held.cpu,"),
 ("  stopped=true;clearTimeout(first);clearInterval(timer);clearInterval(lender);clearInterval(watcher);",
  "  stopped=true;clearTimeout(first);clearInterval(timer);clearInterval(lender);clearInterval(watcher);clearInterval(grower);"),
])

def units_schema(value):
    d=value["definitions"];p=d["Policy"]["const"]
    p["unitQuantileBasisPoints"]=7500;p["minimumUnitMegabytes"]=256;p["minimumUnitSamples"]=8
    host=d["Host"]
    for name in ("grantedMegabytes","unitMegabytes"):
        if name not in host["required"]:host["required"].append(name)
    host["properties"]["grantedMegabytes"]={"type":["integer","null"],"minimum":0,"maximum":16777216}
    host["properties"]["unitMegabytes"]={"type":"integer","minimum":1,"maximum":1048576}
    decision=d["Decision"]
    if "budgetMegabytes" not in decision["required"]:decision["required"].append("budgetMegabytes")
    decision["properties"]["budgetMegabytes"]={"type":["integer","null"],"minimum":1,"maximum":16777216}
    messages=d["Messages"]["const"]
    messages["en"]["reasons"]["granted"]="the machine admission granted fewer jobs and its memory carries no more compile units"
    messages["de"]["reasons"]["granted"]="die Maschinen-Zulassung hat weniger Jobs erteilt und ihr Speicher traegt nicht mehr Kompiliereinheiten"
edit_json(CU+"/🧬️schema/🔣️.json",units_schema)

UNITS=CU+"/🟦️.ts"
edit(UNITS,[
 ("/** 🌡️ The live host facts one plan reads; `null` marks a fact the platform does not offer (no limit from it). */\nexport type CompileUnitsHostV1=Readonly<{cores:number;availableMegabytes:number|null;pressure:\"normal\"|\"warn\"|\"critical\"|null;grantedJobs:number|null}>;",
  "/** 🌡️ The live host facts one plan reads; `null` marks a fact the platform does not offer (no limit from it): the jobs the enclosing machine admission holds now and the memory its jobs may hold (`grantedMegabytes`), and the measured footprint of one compile unit (`unitMegabytes`, `compileUnitMegabytesV1`). */\nexport type CompileUnitsHostV1=Readonly<{cores:number;availableMegabytes:number|null;pressure:\"normal\"|\"warn\"|\"critical\"|null;grantedJobs:number|null;grantedMegabytes:number|null;unitMegabytes:number}>;"),
 ("/** ⚖️ The planned units: ask for up to `preferred` cores, start as soon as `minimum` are free. */\nexport type CompileUnitsDecisionV1=Readonly<{preferred:number;minimum:number;reason:CompileUnitsReasonV1}>;",
  "/** ⚖️ The planned units: ask for up to `preferred` cores, start as soon as `minimum` are free; `budgetMegabytes` is the memory the compile may hold under its machine admission (null without one), which bounds the memory-admitting compiler broker. */\nexport type CompileUnitsDecisionV1=Readonly<{preferred:number;minimum:number;reason:CompileUnitsReasonV1;budgetMegabytes:number|null}>;"),
 ("import {readHostCapacityPlanV1}from\"../../🌡️host-capacity/🎫️gate/🟦️.ts\";",
  "import {readLiveHostCapacityPlanV1}from\"../../🌡️host-capacity/🎫️gate/🟦️.ts\";"),
 ("/** 🔨️ Plans counted compile capacity from the live host: one compile may use `shareBasisPoints` of the cores (so at least two run side by side), never more than its work can use, than the machine admission granted, than the free memory carries (`megabytesPerUnit` each above the reserve), one core under critical pressure and half under raised pressure; it starts once `minimumShareBasisPoints` of that are free.",
  "/** 📏️ The measured footprint of one compile unit: the `unitQuantileBasisPoints` quantile (nearest rank) of the newest peak of every unit the compiler broker recorded, never below `minimumUnitMegabytes`; the authored `megabytesPerUnit` until `minimumUnitSamples` units were measured.\n * @see ./🧪️tests/🐍️oracle/🐍️.py */\nexport function compileUnitMegabytesV1(history:ReadonlyMap<string,readonly Readonly<{peakRssBytes:number}>[]>=new Map()):number{\n const peaks=[...history.values()].filter(rows=>rows.length>0).map(rows=>rows[rows.length-1]!.peakRssBytes).sort((left,right)=>left-right);\n if(peaks.length<policy.minimumUnitSamples)return policy.megabytesPerUnit;\n return Math.max(policy.minimumUnitMegabytes,Math.ceil(peaks[Math.max(0,Math.ceil(peaks.length*policy.unitQuantileBasisPoints/10000)-1)]!/MEGABYTE));\n}\n\n/** 🔨️ Plans counted compile capacity from the live host: one compile may use `shareBasisPoints` of the cores (so at least two run side by side), never more than its work can use, than the machine admission carries (its granted jobs, or more when the memory its jobs may hold carries more compile units at the measured footprint of one: a compile is one action that runs several units), than the free memory carries (`unitMegabytes` each above the reserve), one core under critical pressure and half under raised pressure; it starts once `minimumShareBasisPoints` of that are free. Under a machine admission the compile may hold the memory of its grant, and never less than its planned units need at the measured footprint (`budgetMegabytes`)."),
 (" if(host.grantedJobs!==null)limit(host.grantedJobs,\"granted\");\n if(host.availableMegabytes!==null)limit(Math.floor(Math.max(0,host.availableMegabytes-policy.reserveMegabytes)/policy.megabytesPerUnit),\"memory\");",
  " if(host.grantedJobs!==null)limit(Math.max(host.grantedJobs,host.grantedMegabytes===null?0:Math.floor(host.grantedMegabytes/host.unitMegabytes)),\"granted\");\n if(host.availableMegabytes!==null)limit(Math.floor(Math.max(0,host.availableMegabytes-policy.reserveMegabytes)/host.unitMegabytes),\"memory\");"),
 ("{preferred:units,minimum:Math.max(1,Math.ceil(units*policy.minimumShareBasisPoints/10000)),reason}",
  "{preferred:units,minimum:Math.max(1,Math.ceil(units*policy.minimumShareBasisPoints/10000)),reason,budgetMegabytes:host.grantedMegabytes===null?null:Math.max(1,host.grantedMegabytes,units*host.unitMegabytes)}"),
 ("/** 🔌️ Reads the live host facts: cores, free memory and memory pressure of the platform probe plus the jobs the enclosing machine admission granted. A counter that cannot be read stays `null`, no fact, no limit. */\nexport async function readCompileUnitsHostV1(options:Readonly<{environment?:Readonly<Record<string,string|undefined>>;signal?:AbortSignal;probe?:HostProbeV1;cores?:number}>={}):Promise<CompileUnitsHostV1>{\n const environment=options.environment??process.env,plan=readHostCapacityPlanV1(environment);",
  "/** 🔌️ Reads the live host facts: cores, free memory and memory pressure of the platform probe, the jobs and job memory the enclosing machine admission holds now (its grant row, so a run that grew after this process started is seen grown) and the unit footprint measured in the compiler broker's `history`. A counter that cannot be read stays `null`, no fact, no limit. */\nexport async function readCompileUnitsHostV1(options:Readonly<{environment?:Readonly<Record<string,string|undefined>>;signal?:AbortSignal;probe?:HostProbeV1;cores?:number;history?:ReadonlyMap<string,readonly Readonly<{peakRssBytes:number}>[]>}>={}):Promise<CompileUnitsHostV1>{\n const environment=options.environment??process.env,plan=readLiveHostCapacityPlanV1(environment);"),
 ("pressure:counters?.pressureLevel??null,grantedJobs:plan?.jobs??null});",
  "pressure:counters?.pressureLevel??null,grantedJobs:plan?.jobs??null,grantedMegabytes:plan?.memoryMegabytes??null,unitMegabytes:compileUnitMegabytesV1(options.history)});"),
 ("export async function compileUnitsForHostV1(options:Readonly<{environment?:Readonly<Record<string,string|undefined>>;signal?:AbortSignal;needed?:number;probe?:HostProbeV1;cores?:number}>={})",
  "export async function compileUnitsForHostV1(options:Readonly<{environment?:Readonly<Record<string,string|undefined>>;signal?:AbortSignal;needed?:number;probe?:HostProbeV1;cores?:number;history?:ReadonlyMap<string,readonly Readonly<{peakRssBytes:number}>[]>}>={})"),
])

LAUNCHER=LIB+"/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts"
edit(LAUNCHER,[
 (" * the host capacity gate, a jobserver of exactly the admitted cores and the memory-admitting rustc broker sized to the same share of physical memory.\n",
  " * the host capacity gate, a jobserver of exactly the admitted cores and the memory-admitting rustc broker sized to the same share of physical memory; a planned compile\n * under a machine admission takes the cores the grant's memory carries at the unit footprint the broker measured, and the broker never holds more than that memory.\n"),
 ("mark(\"capacity\",\"begin\");const units=request.parallelism===\"planned\"?await compileUnitsForHostV1({environment:env,signal:control.signal}):undefined,",
  "mark(\"capacity\",\"begin\");const history=await readUnitHistoryV1(unitHistoryPathV1(join(store.root,\"components\"))),units=request.parallelism===\"planned\"?await compileUnitsForHostV1({environment:env,signal:control.signal,history}):undefined,"),
 ("const history=await readUnitHistoryV1(unitHistoryPathV1(join(store.root,\"components\"))),jobserverDirectory=",
  "const jobserverDirectory="),
 ("physicalBytes:admissionMemoryShareV1(admissionMemoryBytesV1(),tokens,availableParallelism())",
  "physicalBytes:Math.min(admissionMemoryShareV1(admissionMemoryBytesV1(),tokens,availableParallelism()),units?.decision.budgetMegabytes==null?Infinity:ADMISSION_POLICY_V1.reserveBytes+units.decision.budgetMegabytes*1048576)"),
])

POLICY=LIB+"/🏗️build/🔣️policy.json"

edit(GATE,[
 ("const carried=readHostCapacityPlanV1(environment),plan=carried?.id===grant.id?carried:",
  "const carried=readHostCapacityPlanV1(environment),plan=carried?.id===grant.id?followHostGrantPlanV1(carried,grant):"),
])
