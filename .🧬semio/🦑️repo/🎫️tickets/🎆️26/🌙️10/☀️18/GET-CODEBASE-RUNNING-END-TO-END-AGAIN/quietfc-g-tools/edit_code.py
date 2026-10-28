"""🛠️ quietfc-g: the code edits of the staged change, applied to a root (the overlay while staged) as anchored, idempotent replacements: every anchor must occur exactly once, a second run changes nothing. Usage: python3 edit_code.py <root>"""
import sys
root=sys.argv[1]
F="🧰️framework"
HC=f"{root}/{F}/🔨️modules/🏃️process/🏗️build/🌡️host-capacity"
LIB=f"{root}/{F}/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
def edit(path,pairs):
    import os
    if not os.path.realpath(path).startswith(os.path.realpath(root)+"/"):raise SystemExit(f"refusing to edit outside the overlay root: {path}")
    text=open(path,encoding="utf8").read();changed=False
    for old,new in pairs:
        if new in text:continue
        if text.count(old)!=1:raise SystemExit(f"{path}: anchor occurs {text.count(old)}x: {old[:90]}")
        text=text.replace(old,new);changed=True
    if changed:
        temporary=path+".quietfc.tmp"
        with open(temporary,"w",encoding="utf8") as handle:handle.write(text)
        import os;os.replace(temporary,path)
    print("patched" if changed else "kept",path.split("/")[-2])
# ---- decision: types, quiet grants avoid soft stops, envelopes in the sizing
edit(f"{HC}/📏️decision/🟦️.ts",[
 ("maximumRamping:number;maximumQuiet:number}>;swap:","maximumRamping:number;maximumQuiet:number;quietSoftStopGrants:number}>;swap:"),
 ("quietMinimumSamples:number;quietMaximumMegabytes:number}>;","quietMinimumSamples:number;quietMaximumMegabytes:number;quietFollowUpMilliseconds:number}>;"),
 ("operations:readonly HostOperationV1[];estimates:HostEstimatePolicyV1}>;","operations:readonly HostOperationV1[];estimates:HostEstimatePolicyV1;envelopes:readonly HostEnvelopeV1[];classCeilings:readonly HostCeilingV1[]}>;"),
 ("/** 🧭️ How one operation sizes its request:","/** 🧰️ The resources a target holds at its peak when it runs several workers of its own (end-to-end suites): a target whose name matches `pattern` is sized at least at `cpu` cores and `megabytes` per job, whatever kind the worker pool classifies it as (`Envelope` of the schema). */\nexport type HostEnvelopeV1=Readonly<{name:string;pattern:string;cpu:number;megabytes:number}>;\n/** 🧢️ The most a class of runs may ever ask for (`Ceiling` of the schema): a declared bound on the class key that holds until the class measured enough runs to be capped by its own peaks. */\nexport type HostCeilingV1=Readonly<{name:string;pattern:string;megabytes:number}>;\n/** 🧭️ How one operation sizes its request:"),
 ("  const request=waiter.request,need=(jobs:number)=>Math.min(request.fixedMegabytes+jobs*request.perJobMegabytes,cap),waited=nowMs-waiter.arrivedAtMs,quiet=request.quiet===true;",
  "  const request=waiter.request,need=(jobs:number)=>Math.min(request.fixedMegabytes+jobs*request.perJobMegabytes,cap),waited=nowMs-waiter.arrivedAtMs,quiet=request.quiet===true,avoids=quiet&&softLeft>0,pause=avoids?undefined:soft;"),
 ("let blocked=false,admitted=0,own=ledgerCpu,","let blocked=false,admitted=0,own=ledgerCpu,softLeft=policy.memory.quietSoftStopGrants-active.filter(grant=>grant.quiet===true).length,"),
 ("if(quiet)quietLeft--;else for(const head of heads)","if(quiet){quietLeft--;softLeft--;}else for(const head of heads)"),
 ("  if(request.rampSlot===true&&freeRampers>=policy.memory.maximumRamping){refuse(\"ramp\");continue;}","  if(quiet&&quietLeft<=0){refuse(\"quiet-limit\");continue;}\n  if(request.rampSlot===true&&freeRampers>=policy.memory.maximumRamping){refuse(\"ramp\");continue;}"),
 ("|\"queue-order\"|\"ramp\";","|\"queue-order\"|\"ramp\"|\"quiet-limit\";"),
 ("  if(soft===undefined)for(let candidate=request.preferredJobs;candidate>=request.minimumJobs;candidate--)if(fits(candidate)&&candidate*request.cpuPerJob<=freeLoad){jobs=candidate;break;}\n  if(jobs===0&&soft===undefined&&memoryFits&&own-(replaced?.cpu??0)+request.minimumJobs*request.cpuPerJob<=ceiling)jobs=request.minimumJobs;",
  "  if(pause===undefined)for(let candidate=request.preferredJobs;candidate>=request.minimumJobs;candidate--)if(fits(candidate)&&(avoids||candidate*request.cpuPerJob<=freeLoad)){jobs=candidate;break;}\n  if(jobs===0&&pause===undefined&&memoryFits&&(avoids||own-(replaced?.cpu??0)+request.minimumJobs*request.cpuPerJob<=ceiling))jobs=request.minimumJobs;"),
 ("const reason:HostReasonV1=!memoryFits?\"memory\":soft??\"load\";","const reason:HostReasonV1=!memoryFits?\"memory\":pause??\"load\";"),
 ("export function sizeHostRequestV1(policy:HostCapacityPolicyV1,input:Readonly<{id:string;operation:string;preferredJobs?:number;kinds?:readonly HostActionKindV1[]}>):HostRequestV1|undefined{",
  "export function sizeHostRequestV1(policy:HostCapacityPolicyV1,input:Readonly<{id:string;operation:string;preferredJobs?:number;kinds?:readonly HostActionKindV1[];targets?:readonly string[]}>):HostRequestV1|undefined{"),
 (" const kinds=[...new Set<HostActionKindV1>([...operation.kind?[operation.kind]:input.kinds??[],\"build\"])],rows=kinds.map(kind=>policy.kinds[kind]);",
  " const kinds=[...new Set<HostActionKindV1>([...operation.kind?[operation.kind]:input.kinds??[],\"build\"])],rows:HostResourceV1[]=[...kinds.map(kind=>policy.kinds[kind]),...policy.envelopes.filter(row=>(input.targets??[]).some(target=>new RegExp(row.pattern,\"u\").test(target)))];"),
])
# ---- forecast: the follow-up rule
edit(f"{HC}/🌦️forecast/🟦️.ts",[
 ("the share of quiet runs among the recent ones, in basis points. */\nexport type HostForecastV1=Readonly<{quiet:boolean;megabytes:number;rampMilliseconds:number;samples:number;shareBasisPoints:number}>;",
  "the share of quiet runs among the recent ones, in basis points; `followUpMilliseconds` (only when the follow-up rule alone made the forecast quiet) is the time since the earlier run of the class finished. */\nexport type HostForecastV1=Readonly<{quiet:boolean;megabytes:number;rampMilliseconds:number;samples:number;shareBasisPoints:number;followUpMilliseconds?:number}>;"),
 ("/** 🔮️ Forecasts whether the next run of a class executes nothing: it does when enough decayed quiet runs exist",
  "/** 🔮️ Forecasts whether the next run of a class executes nothing: it does when the earlier run of the class finished at most `quietFollowUpMilliseconds` ago (a run that ended well left its results cached, so the run that follows it is a replay unless the world changed in between: the follow-up rule, which also starts a class that has no quiet history yet; a wrong forecast costs nothing because the upgrade protects, a missed one costs the whole queue wait) or when enough decayed quiet runs exist"),
 (" const share=total===0?0:Math.floor(quietWeight*10000/total+0.5),newest=(rows:typeof quiet)=>rows.reduce((latest,row)=>Math.max(latest,row.atMs),-1);\n const estimate=estimateHostRatioV1(policy,store,[classes.quiet],nowMs),megabytes=Math.floor(estimate.ratioBasisPoints*rules.quietReferenceMegabytes/10000+0.5);\n return Object.freeze({quiet:quietWeight>=rules.quietMinimumSamples*UNIT&&share>=rules.quietShareBasisPoints&&newest(quiet)>newest(busy)&&megabytes<=rules.quietMaximumMegabytes,megabytes,rampMilliseconds:estimate.rampMilliseconds,samples:estimate.samples,shareBasisPoints:share});",
  " const share=total===0?0:Math.floor(quietWeight*10000/total+0.5),newest=(rows:typeof quiet)=>rows.reduce((latest,row)=>Math.max(latest,row.atMs),-1);\n const estimate=estimateHostRatioV1(policy,store,[classes.quiet],nowMs),megabytes=Math.floor(estimate.ratioBasisPoints*rules.quietReferenceMegabytes/10000+0.5);\n const history=quietWeight>=rules.quietMinimumSamples*UNIT&&share>=rules.quietShareBasisPoints&&newest(quiet)>newest(busy),since=nowMs-Math.max(newest(quiet),newest(busy)),followUp=!history&&rules.quietFollowUpMilliseconds>0&&Math.max(newest(quiet),newest(busy))>=0&&since>=0&&since<=rules.quietFollowUpMilliseconds,quiet_=(history||followUp)&&megabytes<=rules.quietMaximumMegabytes;\n return Object.freeze({quiet:quiet_,megabytes,rampMilliseconds:estimate.rampMilliseconds,samples:estimate.samples,shareBasisPoints:share,...quiet_&&followUp?{followUpMilliseconds:since}:{}});"),
])
# ---- library door: targets of the ask, envelope sizing, the follow-up message
edit(f"{LIB}/🌡️host-capacity/🟦️.ts",[
 ("export type BuildCapacityAskV1=Readonly<{operation:string;preferredJobs:number;kinds:readonly HostActionKindV1[];lineage:readonly string[]}>;",
  "export type BuildCapacityAskV1=Readonly<{operation:string;preferredJobs:number;kinds:readonly HostActionKindV1[];targets:readonly string[];lineage:readonly string[]}>;"),
 ("function requestedKinds(args:readonly string[]):HostActionKindV1[]{\n const worker=readWorkerPolicyV1((buildPolicy as unknown as {workers:unknown}).workers),named=args[0]===\"run\"?[args[1]?.split(\":\")[1]]:(option(args,\"targets\",\"t\")??option(args,\"target\")??\"\").split(\",\");\n return named.filter((target):target is string=>typeof target===\"string\"&&target.length>0).map(target=>workerActionKindV1(worker,target,false));\n}",
  "function requestedTargets(args:readonly string[]):string[]{\n const named=args[0]===\"run\"?[args[1]?.split(\":\")[1]]:(option(args,\"targets\",\"t\")??option(args,\"target\")??\"\").split(\",\");\n return named.filter((target):target is string=>typeof target===\"string\"&&target.length>0);\n}\nfunction requestedKinds(targets:readonly string[]):HostActionKindV1[]{\n const worker=readWorkerPolicyV1((buildPolicy as unknown as {workers:unknown}).workers);\n return targets.map(target=>workerActionKindV1(worker,target,false));\n}"),
 (" const kinds=command===\"cargo\"?[]:requestedKinds(head);\n return Object.freeze({operation:command,preferredJobs:preferred,kinds:Object.freeze(kinds),lineage:buildCapacityLineageV1(args,kinds)});",
  " const targets=command===\"cargo\"?[]:requestedTargets(head),kinds=requestedKinds(targets);\n return Object.freeze({operation:command,preferredJobs:preferred,kinds:Object.freeze(kinds),targets:Object.freeze(targets),lineage:buildCapacityLineageV1(args,kinds)});"),
 ("operation:ask.operation,preferredJobs:ask.preferredJobs,kinds:ask.kinds});\n if(!base)return body({args,handle:undefined});","operation:ask.operation,preferredJobs:ask.preferredJobs,kinds:ask.kinds,targets:ask.targets});\n if(!base)return body({args,handle:undefined});"),
])
edit(f"{HC}/📏️decision/🟦️.ts",[
 ('name:"waiting"|"admitted"|"estimated"|"phased"|"quiet"|"upgrading"','name:"waiting"|"admitted"|"estimated"|"phased"|"quiet"|"quietFollowUp"|"upgrading"'),
])
edit(f"{LIB}/🌡️host-capacity/🟦️.ts",[
 (' if(quiet)write(formatHostCapacityV1(resolveHostCapacityLanguageV1(environment),"quiet",{operation:ask.operation,samples:forecast!.samples,percent:Math.round(forecast!.shareBasisPoints/100),reserved:admission.fixedMegabytes,full:request.fixedMegabytes+request.preferredJobs*request.perJobMegabytes}));',
  ' if(quiet)write(forecast!.followUpMilliseconds===undefined?formatHostCapacityV1(resolveHostCapacityLanguageV1(environment),"quiet",{operation:ask.operation,samples:forecast!.samples,percent:Math.round(forecast!.shareBasisPoints/100),reserved:admission.fixedMegabytes,full:request.fixedMegabytes+request.preferredJobs*request.perJobMegabytes}):formatHostCapacityV1(resolveHostCapacityLanguageV1(environment),"quietFollowUp",{operation:ask.operation,seconds:Math.round(forecast!.followUpMilliseconds/1000),reserved:admission.fixedMegabytes,full:request.fixedMegabytes+request.preferredJobs*request.perJobMegabytes}));'),
])
# ---- the declared host profile: index, sampler, library door, worker policy, test workers, Bazel server plan, graph farm
edit(f"{HC}/🟦️.ts",[('export * from "./📡️sampling/🟦️.ts";\n','export * from "./📡️sampling/🟦️.ts";\nexport * from "./🪞️profile/🟦️.ts";\n')])
edit(f"{HC}/📡️sampling/🟦️.ts",[
 ('import {formatHostCapacityV1,hostCapacityErrorV1,type HostCapacityPolicyV1,type HostSampleV1} from "../📏️decision/🟦️.ts";','import {formatHostCapacityV1,hostCapacityErrorV1,type HostCapacityPolicyV1,type HostSampleV1} from "../📏️decision/🟦️.ts";\nimport {hostProfileV1} from "../🪞️profile/🟦️.ts";'),
 (' const real=Math.floor(totalmem()/MEGABYTE),simulated=simulatedHostTotalV1(environment),total=simulated===null?real:Math.min(real,simulated);\n const counters=await readHostCountersV1(probe),cores=await readHostCoresV1(probe,cpus().length||1),',
  ' const counters=await readHostCountersV1(probe),realCores=await readHostCoresV1(probe,cpus().length||1),{totalMegabytes:total,cores}=hostProfileV1(environment,{totalMegabytes:Math.floor(totalmem()/MEGABYTE),cores:realCores,declared:false}),'),
])
edit(f"{LIB}/🌡️host-capacity/🟦️.ts",[
 (" const policy=ports.policy??BUILD_HOST_CAPACITY_POLICY_V1,total=ports.totalMegabytes??Math.floor(totalmem()/1048576);"," const policy=ports.policy??BUILD_HOST_CAPACITY_POLICY_V1,total=ports.totalMegabytes??hostProfileV1(environment).totalMegabytes;"),
 (" const total=Math.floor(totalmem()/1048576),capacity="," const total=hostProfileV1(environment).totalMegabytes,capacity="),
 ("import {createHostPhaseResizerV1,","import {hostProfileV1,createHostPhaseResizerV1,"),
])
edit(f"{LIB}/👷️workers/🟦️.ts",[
 ('import schema from "./🧬️schema/🔣️.json" with {type:"json"};','import {hostProfileV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🪞️profile/🟦️.ts";\nimport schema from "./🧬️schema/🔣️.json" with {type:"json"};'),
 ("export function currentWorkerHostV1():WorkerHostV1{return Object.freeze({cores:availableParallelism(),memoryBytes:totalmem()});}","export function currentWorkerHostV1(environment:Readonly<Record<string,string|undefined>>=process.env):WorkerHostV1{const host=hostProfileV1(environment,{totalMegabytes:Math.floor(totalmem()/1048576),cores:availableParallelism(),declared:false});return Object.freeze({cores:host.cores,memoryBytes:host.totalMegabytes*1048576});}"),
])
edit(f"{LIB}/🔧️tools/🏗️bazel/📜️script.ts",[
 (" return bazelServerOverrideV1(planBazelServersV1(host??{cores:availableParallelism(),memoryMegabytes:Math.floor(totalmem()/1048576)},"," return bazelServerOverrideV1(planBazelServersV1(host??(declared=>({cores:declared.cores,memoryMegabytes:declared.totalMegabytes}))(hostProfileV1(process.env,{totalMegabytes:Math.floor(totalmem()/1048576),cores:availableParallelism(),declared:false})),"),
 ('import {BUILD_HOST_CAPACITY_POLICY_V1,createCapacityControlV1,upgradeBuildCapacityV1} from "../../🌡️host-capacity/🟦️.ts";','import {BUILD_HOST_CAPACITY_POLICY_V1,createCapacityControlV1,upgradeBuildCapacityV1} from "../../🌡️host-capacity/🟦️.ts";\nimport {hostProfileV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🪞️profile/🟦️.ts";'),
])
edit(f"{HC}/../../🧪️testing/👷️workers/🟦️.ts",[
 ("host:Readonly<{cores:number;totalMegabytes:number}>={cores:availableParallelism(),totalMegabytes:Math.floor(totalmem()/1048576)}):TestWorkerFactsV1{","host:Readonly<{cores:number;totalMegabytes:number}>=hostProfileV1(environment,{cores:availableParallelism(),totalMegabytes:Math.floor(totalmem()/1048576),declared:false})):TestWorkerFactsV1{"),
 ('import{requireHostCapacityV1,resolveHostCapacityLanguageV1,hostCapacityErrorV1,type HostPlanV1}from"../../🏗️build/🌡️host-capacity/📏️decision/🟦️.ts";','import{requireHostCapacityV1,resolveHostCapacityLanguageV1,hostCapacityErrorV1,type HostPlanV1}from"../../🏗️build/🌡️host-capacity/📏️decision/🟦️.ts";\nimport{hostProfileV1}from"../../🏗️build/🌡️host-capacity/🪞️profile/🟦️.ts";'),
])
# ---- library fixtures and tests that state the old behaviour
import json as _json
def fixture_targets(path):
    text=open(path,encoding="utf8").read();data=_json.loads(text)
    if _json.dumps(data,indent=1,ensure_ascii=False)+"\n"!=text and _json.dumps(data,indent=1,ensure_ascii=False)!=text:raise SystemExit("fixture formatting is not reproducible: "+path)
    changed=False
    for row in data["asks"]:
        expect=row["expect"]
        if expect is None or "targets" in expect:continue
        args=row["args"];head=args[:args.index("--")] if "--" in args else args
        if expect["operation"]=="cargo" or expect["operation"]=="show":names=[]
        elif head[0]=="run":names=[head[1].split(":")[1]]
        else:
            value=None
            for index,argument in enumerate(head):
                for flag in ("--targets","--target","-t"):
                    if argument==flag and index+1<len(head):value=head[index+1]
                    elif argument.startswith(flag+"="):value=argument[len(flag)+1:]
            names=[n for n in (value or "").split(",") if n]
        rebuilt={}
        for key,item in expect.items():
            rebuilt[key]=item
            if key=="kinds":rebuilt["targets"]=names
        row["expect"]=rebuilt;changed=True
    if changed:
        out=_json.dumps(data,indent=1,ensure_ascii=False)+("\n" if text.endswith("\n") else "")
        open(path,"w",encoding="utf8").write(out)
    print("patched" if changed else "kept","ask targets")
fixture_targets(f"{LIB}/🌡️host-capacity/🧪️tests/🧫️fixtures/🔣️.json")
edit(f"{LIB}/🌡️host-capacity/🧪️tests/🔼️upgrade/🟦️.ts",[
 (' const second=holdMachine(directory);await sleep(40);\n let started=false;\n const waiting=run(directory,async()=>{started=true;});\n await sleep(300);expect(started,"after a busy run the forecast is not quiet, the full grant queues at the door").toBe(false);\n second.release();await waiting;await second.done;expect(started).toBe(true);',
  ' const second=holdMachine(directory);await sleep(40);\n let started=false;\n const waiting=run(directory,async granted=>{started=true;expect(granted.handle!.reservedMegabytes,"after a busy run the next run of the class follows it: the small grant answers at once").toBeLessThan(4096);await sleep(120);});\n await sleep(300);expect(started,"the follow-up forecast does not queue behind the full machine").toBe(true);\n second.release();await waiting;await second.done;'),
])
# ---- the sample file is shared by every process of the machine: a sample is reused only by a process that declares the same machine
edit(f"{HC}/📡️sampling/🟦️.ts",[
 ('export type HostSampleRecordV1=Readonly<{host:HostSampleV1;swappedOutMegabytes:number|null}>;','export type HostSampleRecordV1=Readonly<{host:HostSampleV1;swappedOutMegabytes:number|null;declared?:string}>;'),
])
edit(f"{HC}/🎫️gate/🟦️.ts",[
 ('import {currentHostProbeV1,observeTreeMegabytesV1,readHostSampleV1,type HostSampleRecordV1} from "../📡️sampling/🟦️.ts";','import {currentHostProbeV1,observeTreeMegabytesV1,readHostSampleV1,type HostSampleRecordV1} from "../📡️sampling/🟦️.ts";\nimport {hostDeclarationKeyV1} from "../🪞️profile/🟦️.ts";'),
 ('async function currentSample(directory:string,policy:HostCapacityPolicyV1,control:ResourceLeaseControl,ports:HostCapacityPortsV1):Promise<HostSampleV1>{\n const now=ports.now??Date.now,path=join(directory,"sample.json"),prior=readJson(path) as HostSampleRecordV1|undefined,cached=isHostCapacityV1("Host",prior?.host)?prior!.host:undefined;\n if(cached&&now()>=cached.atMs&&now()-cached.atMs<=policy.wait.sampleMaximumAgeMilliseconds)return cached;',
  'async function currentSample(directory:string,policy:HostCapacityPolicyV1,control:ResourceLeaseControl,ports:HostCapacityPortsV1,environment:Readonly<Record<string,string|undefined>>=process.env):Promise<HostSampleV1>{\n const now=ports.now??Date.now,path=join(directory,"sample.json"),prior=readJson(path) as HostSampleRecordV1|undefined,cached=isHostCapacityV1("Host",prior?.host)?prior!.host:undefined,declared=hostDeclarationKeyV1(environment);\n if(cached&&(prior!.declared??"")===declared&&now()>=cached.atMs&&now()-cached.atMs<=policy.wait.sampleMaximumAgeMilliseconds)return cached;'),
 ('readHostSampleV1(policy,currentHostProbeV1(control.signal),previous,now));\n requireHostCapacityV1<HostSampleV1>("Host",record.host);\n try{writeJson(path,record);}catch{}',
  'readHostSampleV1(policy,currentHostProbeV1(control.signal),previous,now,environment));\n requireHostCapacityV1<HostSampleV1>("Host",record.host);\n try{writeJson(path,{...record,declared});}catch{}'),
 ('const host=await currentSample(directory,policy,options.control,options);\n   try{utimesSync','const host=await currentSample(directory,policy,options.control,options,environment);\n   try{utimesSync'),
 ('sample:()=>currentSample(directory,policy,options.control,options)}),directory,policy,request:','sample:()=>currentSample(directory,policy,options.control,options,environment)}),directory,policy,request:'),
 ('options:HostCapacityPortsV1&Readonly<{directory:string;policy:HostCapacityPolicyV1;control:ResourceLeaseControl}>):Promise<HostCapacityStatusV1>{','options:HostCapacityPortsV1&Readonly<{directory:string;policy:HostCapacityPolicyV1;control:ResourceLeaseControl;environment?:Readonly<Record<string,string|undefined>>}>):Promise<HostCapacityStatusV1>{'),
 ('host=await currentSample(directory,options.policy,options.control,options),grants=readGrants(directory,now(),options.policy)','host=await currentSample(directory,options.policy,options.control,options,options.environment),grants=readGrants(directory,now(),options.policy)'),
])
# ---- the repo oracle of the upgrade/forecast law follows the new rules (flags OFF reproduce every stored vector)
edit(f"{HC}/🧪️tests/🔼️upgrade/🐍️oracle/🐍️.py",[
 ('        r=waiter["request"];quiet=r.get("quiet") is True\n','        r=waiter["request"];quiet=r.get("quiet") is True;avoids=quiet and state["soft_left"]>0;pause=None if avoids else soft\n'),
 ('"quiet_left":policy["memory"]["maximumQuiet"]-len([g for g in active if g.get("quiet") is True]),','"quiet_left":policy["memory"]["maximumQuiet"]-len([g for g in active if g.get("quiet") is True]),"soft_left":policy["memory"].get("quietSoftStopGrants",0)-len([g for g in active if g.get("quiet") is True]),'),
 ('            if quiet:state["quiet_left"]-=1\n','            if quiet:state["quiet_left"]-=1;state["soft_left"]-=1\n'),
 ('        if r.get("rampSlot") is True and free_rampers>=policy["memory"]["maximumRamping"]:refuse("ramp");continue\n','        if quiet and state["quiet_left"]<=0:\n            count("quietLimit");refuse("quiet-limit");continue\n        if r.get("rampSlot") is True and free_rampers>=policy["memory"]["maximumRamping"]:refuse("ramp");continue\n'),
 ('        if soft is None:\n            for candidate in range(r["preferredJobs"],r["minimumJobs"]-1,-1):\n                if fits(candidate) and candidate*r["cpuPerJob"]<=free_load:jobs=candidate;break\n        if jobs==0 and soft is None and memory_fits and state["own"]-(replaced["cpu"] if replaced else 0)+r["minimumJobs"]*r["cpuPerJob"]<=ceiling:jobs=r["minimumJobs"]',
  '        if pause is None:\n            for candidate in range(r["preferredJobs"],r["minimumJobs"]-1,-1):\n                if fits(candidate) and (avoids or candidate*r["cpuPerJob"]<=free_load):jobs=candidate;break\n        if jobs==0 and pause is None and memory_fits and (avoids or state["own"]-(replaced["cpu"] if replaced else 0)+r["minimumJobs"]*r["cpuPerJob"]<=ceiling):jobs=r["minimumJobs"]'),
 ('reason="memory" if not memory_fits else (soft or "load")','reason="memory" if not memory_fits else (pause or "load")'),
 ('    estimate=estimates.estimate(policy,store,[quiet_key],now);megabytes=math.floor(estimate["ratioBasisPoints"]*e["quietReferenceMegabytes"]/10000+0.5)\n    return {"quiet":wq>=e["quietMinimumSamples"]*UNIT and share>=e["quietShareBasisPoints"] and newest(quiet)>newest(busy) and megabytes<=e["quietMaximumMegabytes"],"megabytes":megabytes,"rampMilliseconds":estimate["rampMilliseconds"],"samples":estimate["samples"],"shareBasisPoints":share}',
  '    estimate=estimates.estimate(policy,store,[quiet_key],now);megabytes=math.floor(estimate["ratioBasisPoints"]*e["quietReferenceMegabytes"]/10000+0.5)\n    history=wq>=e["quietMinimumSamples"]*UNIT and share>=e["quietShareBasisPoints"] and newest(quiet)>newest(busy)\n    latest=max(newest(quiet),newest(busy));since=now-latest\n    follow_up=(not history) and e.get("quietFollowUpMilliseconds",0)>0 and latest>=0 and since>=0 and since<=e["quietFollowUpMilliseconds"]\n    verdict=(history or follow_up) and megabytes<=e["quietMaximumMegabytes"]\n    result={"quiet":verdict,"megabytes":megabytes,"rampMilliseconds":estimate["rampMilliseconds"],"samples":estimate["samples"],"shareBasisPoints":share}\n    if verdict and follow_up:result["followUpMilliseconds"]=since\n    return result'),
])
# ---- the build guide names the follow-up forecast, the soft stops, the envelopes and the declared machine (EN + DE)
edit(f"{root}/{F}/🛍️products/🦑️repo/📓️build-guide.md",[
 ("   - `SEMIO_HOST_CAPACITY=off` is the escape hatch; wrap ad-hoc heavy commands in `repo capacity run --memory=<MB> -- <command>`.\n",
  "   - **Quiet follow-up** (staged, quietfc-g): the run that follows a finished run of its class within 30 minutes is forecast quiet even when the class has little quiet history, because the finished run left its results cached (`[capacity] run: forecast quiet because the earlier run of its class finished 90s ago and left its results cached: …`); a quiet grant ignores the soft stops (raised memory pressure, load, open files, attached images) and the hard stops (critical pressure, swap, memory) still hold it. A class that learned 200% of its default (7168 MB) no longer waits for the full grant on a hot no-op.\n   - **Envelopes**: the policy declares what a multi-worker target holds (`end-to-end`: 4 cores, 6144 MB for targets named `test-e2e…`), so the admitted job carries it and the end-to-end stack planner counts on it.\n   - **Declared machine**: `SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES` (e.g. `16384`) and `SEMIO_HOST_CAPACITY_CORES` (e.g. `8`) make a larger host plan as a smaller one in the gate, the worker policy, the Bazel server plan and the test runners; a declaration only shrinks, a malformed value means the real host, and the shared sample file is reused only by processes declaring the same machine.\n   - `SEMIO_HOST_CAPACITY=off` is the escape hatch; wrap ad-hoc heavy commands in `repo capacity run --memory=<MB> -- <command>`.\n"),
 ("   - `SEMIO_HOST_CAPACITY=off` ist die Notausstiegsluke; schwere Ad-hoc-Befehle in `repo capacity run --memory=<MB> -- <Befehl>` einpacken.\n",
  "   - **Ruhige Anschlusszusage** (vorgemerkt, quietfc-g): der Lauf, der innerhalb von 30 Minuten auf einen beendeten Lauf seiner Klasse folgt, gilt auch bei wenig ruhiger Vorgeschichte als ruhig, weil der beendete Lauf seine Ergebnisse zwischengespeichert hat (`[capacity] run: ruhig prognostiziert, weil der frühere Lauf seiner Klasse vor 90 s endete und seine Ergebnisse zwischengespeichert hinterließ: …`); eine ruhige Zusage ignoriert die weichen Sperren (erhöhter Speicherdruck, Last, offene Dateien, eingehängte Images), die harten (kritischer Druck, Swap, Speicher) halten sie weiter. Eine Klasse, die 200 % ihres Standards (7168 MB) gelernt hat, wartet bei einem heißen No-op nicht mehr auf die volle Zusage.\n   - **Hüllen**: die Richtlinie erklärt, was ein Ziel mit mehreren Arbeitern hält (`end-to-end`: 4 Kerne, 6144 MB für Ziele namens `test-e2e…`); der zugelassene Auftrag trägt das, und die Planung der Ende-zu-Ende-Stacks rechnet damit.\n   - **Erklärter Rechner**: `SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES` (z. B. `16384`) und `SEMIO_HOST_CAPACITY_CORES` (z. B. `8`) lassen einen größeren Rechner im Tor, in der Arbeiter-Richtlinie, im Bazel-Serverplan und in den Testläufern wie einen kleineren planen; eine Erklärung verkleinert nur, ein fehlerhafter Wert heißt der echte Rechner, und die gemeinsame Messdatei nutzen nur Prozesse mit derselben Erklärung gemeinsam.\n   - `SEMIO_HOST_CAPACITY=off` ist die Notausstiegsluke; schwere Ad-hoc-Befehle in `repo capacity run --memory=<MB> -- <Befehl>` einpacken.\n"),
])
# ---- FIX-OF-RED (g5): a class never asks for more than its measured peak or its declared ceiling; the absolute peak is kept in the sample
edit(f"{HC}/📐️estimates/🟦️.ts",[
 ("export type HostEstimateSampleV1=Readonly<{atMs:number;ratioBasisPoints:number;activeMilliseconds:number;peakAtMilliseconds?:number}>;",
  "export type HostEstimateSampleV1=Readonly<{atMs:number;ratioBasisPoints:number;activeMilliseconds:number;peakAtMilliseconds?:number;peakMegabytes?:number}>;"),
 ("export type HostOutcomeV1=Readonly<{operation:string;jobs:number;peakMegabytes:number;defaultMegabytes:number;activeMilliseconds:number;peakAtMilliseconds:number;endedAtMs:number}>;",
  "export type HostOutcomeV1=Readonly<{operation:string;jobs:number;peakMegabytes:number;defaultMegabytes:number;activeMilliseconds:number;peakAtMilliseconds:number;endedAtMs:number;keepPeak?:boolean}>;"),
 ('function quantile(samples:readonly HostEstimateSampleV1[],weight:readonly number[],field:"ratioBasisPoints"|"activeMilliseconds"|"peakAtMilliseconds",basisPoints:number):number{\n const value=(sample:HostEstimateSampleV1):number=>field==="peakAtMilliseconds"?sample.peakAtMilliseconds??sample.activeMilliseconds:sample[field];',
  'function quantile(samples:readonly HostEstimateSampleV1[],weight:readonly number[],field:"ratioBasisPoints"|"activeMilliseconds"|"peakAtMilliseconds"|"peakMegabytes",basisPoints:number):number{\n const value=(sample:HostEstimateSampleV1):number=>field==="peakAtMilliseconds"?sample.peakAtMilliseconds??sample.activeMilliseconds:field==="peakMegabytes"?sample.peakMegabytes??0:sample[field];'),
 ("  rows.push({atMs:outcome.endedAtMs,ratioBasisPoints:ratio,activeMilliseconds:outcome.activeMilliseconds,peakAtMilliseconds:Math.min(outcome.peakAtMilliseconds,outcome.activeMilliseconds)});",
  "  rows.push({atMs:outcome.endedAtMs,ratioBasisPoints:ratio,activeMilliseconds:outcome.activeMilliseconds,peakAtMilliseconds:Math.min(outcome.peakAtMilliseconds,outcome.activeMilliseconds),...outcome.keepPeak===true?{peakMegabytes:Math.max(1,round(outcome.peakMegabytes))}:{}});"),
 ("/** ✍️ Records one finished run in every class of its lineage,",
  "/** 🧢️ The most a class may ask for by declaration: the smallest `megabytes` of the ceilings whose pattern matches the class key, undefined when none does. */\nexport function hostClassCeilingV1(policy:HostCapacityPolicyV1,key:string):number|undefined{\n const rows=policy.classCeilings.filter(row=>new RegExp(row.pattern,\"u\").test(key));\n return rows.length===0?undefined:Math.min(...rows.map(row=>row.megabytes));\n}\n\n/** 📏️ The most a class has measurably needed: the decayed quantile of the absolute peaks its runs kept plus the margin, from the most specific class of the lineage that kept any, and only once `minimumSamples` effective runs back it (a thinner class is bounded by its declared ceiling alone); this bound does not depend on the number of jobs a run was granted, which the learned ratio does. */\nexport function measuredHostPeakV1(policy:HostCapacityPolicyV1,store:HostEstimateStoreV1,lineage:readonly string[],nowMs:number):number|undefined{\n const rules=policy.estimates;\n for(let index=lineage.length-1;index>=0;index--){\n  const rows=(store.classes[lineage[index]!]?.samples??[]).filter(sample=>sample.peakMegabytes!==undefined);\n  if(rows.length===0)continue;\n  const weight=weights(policy,rows,nowMs);\n  return weight.reduce((sum,row)=>sum+row,0)/UNIT<rules.minimumSamples?undefined:round(quantile(rows,weight,\"peakMegabytes\",rules.quantileBasisPoints)*(DEFAULT_BASIS_POINTS+rules.marginBasisPoints)/DEFAULT_BASIS_POINTS);\n }\n return undefined;\n}\n\n/** ✍️ Records one finished run in every class of its lineage,"),
])
edit(f"{HC}/🪜️phases/🟦️.ts",[
 ("import {estimateHostRatioV1,hostSampleQuantileV1,hostSampleWeightsV1,recordHostOutcomeV1,scaleHostRequestV1,","import {estimateHostRatioV1,hostClassCeilingV1,hostSampleQuantileV1,hostSampleWeightsV1,measuredHostPeakV1,recordHostOutcomeV1,scaleHostRequestV1,"),
 ("export type HostPhasedRunV1=Readonly<{jobs:number;peakMegabytes:number;activeMilliseconds:number;peakAtMilliseconds:number;endedAtMs:number;phases?:readonly HostPhaseSegmentV1[]}>;",
  "export type HostPhasedRunV1=Readonly<{jobs:number;peakMegabytes:number;activeMilliseconds:number;peakAtMilliseconds:number;endedAtMs:number;phases?:readonly HostPhaseSegmentV1[];keepPeak?:boolean}>;"),
 ("return recordHostOutcomeV1(policy,store,lineage,{operation,jobs:run.jobs,peakMegabytes:run.peakMegabytes,defaultMegabytes,activeMilliseconds:run.activeMilliseconds,peakAtMilliseconds:run.peakAtMilliseconds,endedAtMs:run.endedAtMs});",
  "return recordHostOutcomeV1(policy,store,lineage,{operation,jobs:run.jobs,peakMegabytes:run.peakMegabytes,defaultMegabytes,activeMilliseconds:run.activeMilliseconds,peakAtMilliseconds:run.peakAtMilliseconds,endedAtMs:run.endedAtMs,...run.keepPeak===true?{keepPeak:true}:{}});"),
 ("const record=(classes:readonly string[],segment:HostPhaseSegmentV1,default_:number,active:(peakAt:number)=>number)=>{const peakAt=Math.max(0,segment.peakAtMilliseconds-rebuilt(phases,segment.peakAtMilliseconds));next=recordHostOutcomeV1(policy,next,classes,{operation,jobs:run.jobs,peakMegabytes:segment.peakMegabytes,defaultMegabytes:default_,activeMilliseconds:active(peakAt),peakAtMilliseconds:peakAt,endedAtMs:run.endedAtMs});};",
  "const record=(classes:readonly string[],segment:HostPhaseSegmentV1,default_:number,active:(peakAt:number)=>number,keepPeak:boolean=false)=>{const peakAt=Math.max(0,segment.peakAtMilliseconds-rebuilt(phases,segment.peakAtMilliseconds));next=recordHostOutcomeV1(policy,next,classes,{operation,jobs:run.jobs,peakMegabytes:segment.peakMegabytes,defaultMegabytes:default_,activeMilliseconds:active(peakAt),peakAtMilliseconds:peakAt,endedAtMs:run.endedAtMs,...keepPeak?{keepPeak:true}:{}});};"),
 ("if(execution)record(lineage,execution,defaultMegabytes,()=>own);","if(execution)record(lineage,execution,defaultMegabytes,()=>own,run.keepPeak===true);"),
 (" const execution=estimateHostRatioV1(policy,store,lineage,nowMs),prior=base.fixedMegabytes+base.minimumJobs*base.perJobMegabytes,preferred=base.fixedMegabytes+base.preferredJobs*base.perJobMegabytes;",
  " const learnedExecution=estimateHostRatioV1(policy,store,lineage,nowMs),prior=base.fixedMegabytes+base.minimumJobs*base.perJobMegabytes,preferred=base.fixedMegabytes+base.preferredJobs*base.perJobMegabytes;\n const bounds=[hostClassCeilingV1(policy,lineage[lineage.length-1]!),measuredHostPeakV1(policy,store,lineage,nowMs)].filter((row):row is number=>row!==undefined),bound=bounds.length===0?undefined:Math.min(...bounds),execution:HostEstimateV1=bound===undefined||preferred<=0?learnedExecution:Object.freeze({...learnedExecution,ratioBasisPoints:Math.max(1,Math.min(learnedExecution.ratioBasisPoints,Math.floor(bound*SCALE/preferred)))});"),
])
edit(f"{LIB}/🌡️host-capacity/🟦️.ts",[
 ("const next=recordHostPhasesV1(policy,store,ask.lineage,ask.operation,defaultMegabytes,run);","const next=recordHostPhasesV1(policy,store,ask.lineage,ask.operation,defaultMegabytes,{...run,keepPeak:true});"),
])
