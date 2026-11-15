import sys,os,json
ROOT="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
def rd(p): return open(ROOT+p,encoding='utf8').read()
def wr(p,t): open(ROOT+p,'w',encoding='utf8').write(t)
def rep(t,old,new,count=1):
    assert t.count(old)>=1,("anchor missing",old[:90]); return t.replace(old,new,count)
only=set(sys.argv[1:])
def want(n): return not only or n in only

if want("client"):
    p="⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/🧬️schema/🔣️.json"
    t=rd(p)
    if "rideThrough" not in t:
        t=rep(t,'"noClaims": "noclaims",','"noClaims": "noclaims", "reachable": "reachable",')
        t=rep(t,'"headMilliseconds": 1500\n','"headMilliseconds": 1500,\n          "rideThroughMilliseconds": 12000,\n          "rideThroughInitialMilliseconds": 250,\n          "rideThroughMaximumMilliseconds": 2000\n')
        t=rep(t,'"noClaimsMilliseconds": 600000 }','"noClaimsMilliseconds": 600000, "reachableMilliseconds": 300000 }')
        wr(p,t)
    p="⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/📡️client/🟦️.ts"
    t=rd(p)
    if "reachableNow" not in t:
        old=t[t.index(' async function call('):t.index(' const json=async<T>')]
        new=''' const reachablePath=join(settings.stateDirectory,fleet.layout.reachable);
 const reachableNow=():boolean=>fresh(reachablePath,fleet.breaker.reachableMilliseconds);
 let stampedAt=0;
 const reachable=():void=>{const at=Date.now();if(at-stampedAt<5000)return;stampedAt=at;stamp(settings.stateDirectory,fleet.layout.reachable);};
 const pause=(milliseconds:number)=>new Promise<void>(done=>{const timer=setTimeout(done,milliseconds);timer.unref?.();});
 async function call(method:string,url:string,timeout:number,init:{body?:BodyInit;headers?:Record<string,string>}={}):Promise<{status:number;response:Response}|FleetFaultV1>{
  const started=Date.now(),budget=reachableNow()?fleet.timeouts.rideThroughMilliseconds:0;
  for(let attempt=0;;attempt++){
   try{
    const response=await request(url,{method,headers:headers(init.headers),signal:AbortSignal.timeout(timeout),...(init.body===undefined?{}:{body:init.body})});
    reachable();
    return {status:response.status,response};
   }catch{
    const wait=Math.min(fleet.timeouts.rideThroughMaximumMilliseconds,fleet.timeouts.rideThroughInitialMilliseconds*2**attempt)*(0.5+Math.random());
    if(Date.now()-started+wait>budget){try{rmSync(reachablePath,{force:true});}catch{}mark("down");return "down";}
    await pause(wait);
   }
  }
 }
'''
        t=t.replace(old,new,1)
        t=rep(t,'import {appendFileSync,mkdirSync,readFileSync,statSync,utimesSync,writeFileSync} from "node:fs";' if 'import {appendFileSync,mkdirSync,readFileSync,statSync,utimesSync,writeFileSync} from "node:fs";' in t else 'import {mkdirSync,readFileSync,statSync,utimesSync,writeFileSync} from "node:fs";','import {appendFileSync,mkdirSync,readFileSync,rmSync,statSync,utimesSync,writeFileSync} from "node:fs";')
        t=rep(t,'/** 📡️ The fleet cache as the unit cache uses it. Every call is bounded by a timeout and never throws:','/** 📡️ The fleet cache as the unit cache uses it. Every call is bounded by a timeout and never throws, and a server that was reachable a moment ago and is not now (a restart takes a few seconds) is retried with jittered back-off for `rideThroughMilliseconds` before the call counts as a fault:')
        wr(p,t)
    p="⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/🟦️.ts"
    t=rd(p)
    if "reasserted" not in t:
        old='void unit.client.renew(unit.key,unit.owner,fleet.claim.ttlMilliseconds).then(state=>{if(state==="lost"){gone=true;stopped=true;clearInterval(timer);}});'
        new='void unit.client.renew(unit.key,unit.owner,fleet.claim.ttlMilliseconds).then(async state=>{if(state==="lost"&&!stopped){const reasserted=await unit.client.claim(unit.key,unit.owner,fleet.claim.ttlMilliseconds);if(reasserted.state==="granted")return;gone=true;stopped=true;clearInterval(timer);}});'
        t=rep(t,old,new)
        wr(p,t)
if want("offload"):
    p="🏗️build/🌐️remote-execution/🚀️offload/🧬️schema/🔣️.json"
    t=rd(p)
    if "rideThroughMilliseconds" not in t:
        t=rep(t,'            "requestMilliseconds"\n          ],','            "requestMilliseconds",\n            "rideThroughMilliseconds"\n          ],')
        t=rep(t,'      "requestMilliseconds": 20000\n    },','      "requestMilliseconds": 20000,\n      "rideThroughMilliseconds": 20000\n    },')
        t=rep(t,'            "requestMilliseconds": {\n              "type": "integer",\n              "minimum": 1\n            }\n','            "requestMilliseconds": {\n              "type": "integer",\n              "minimum": 1\n            },\n            "rideThroughMilliseconds": {\n              "type": "integer",\n              "minimum": 1\n            }\n')
        wr(p,t)
    p="🏗️build/🌐️remote-execution/🚀️offload/🔌️client/🟦️.ts"
    t=rd(p)
    if "transient" not in t:
        t=rep(t,'Readonly<{kind:"declined";reason:OffloadReasonV1;detail:string}>','Readonly<{kind:"declined";reason:OffloadReasonV1;detail:string;transient?:true}>')
        t=rep(t,'const declined=(reason:OffloadReasonV1,detail:string):OffloadExecutionV1=>({kind:"declined",reason,detail});','const declined=(reason:OffloadReasonV1,detail:string,transient=false):OffloadExecutionV1=>({kind:"declined",reason,detail,...(transient?{transient:true as const}:{})});')
        t=rep(t,'declined("infrastructure","no response")','declined("infrastructure","no response",true)')
        t=rep(t,'"infrastructure",`${error.code} ${error.message}`);','"infrastructure",`${error.code} ${error.message}`,error.code===GRPC_CODES_V1.UNAVAILABLE);')
        t=rep(t,'return declined("infrastructure",String((error as Error)?.message??error));','return declined("infrastructure",String((error as Error)?.message??error),true);')
        wr(p,t)
    p="🏗️build/🌐️remote-execution/🚀️offload/🟦️.ts"
    t=rd(p)
    if "executeOffloadRidingThroughV1" not in t:
        t=rep(t,'import {executeOffloadV1,readOffloadOutputsV1,writeOffloadOutputsV1} from "./🔌️client/🟦️.ts";','import {executeOffloadRidingThroughV1,readOffloadOutputsV1,writeOffloadOutputsV1} from "./🔌️client/🟦️.ts";')
        t=rep(t,'try{execution=await executeOffloadV1({','try{execution=await executeOffloadRidingThroughV1({')
        t=rep(t,'signal:cancelled});}','signal:cancelled},policy.timeouts.rideThroughMilliseconds);}')
        wr(p,t)
    p="🏗️build/🌐️remote-execution/🚀️offload/🔌️client/🟦️.ts"
    t=rd(p)
    if "executeOffloadRidingThroughV1" not in t:
        t+=open(os.path.dirname(os.path.abspath(__file__))+"/riding.ts.frag",encoding='utf8').read()
        wr(p,t)
    print("offload schema properties need rideThroughMilliseconds definition: check")
if want("scheduler"):
    p="🏗️build/🏃️execution/🧬️schema/🌐️remote-execution.json"
    t=rd(p)
    if '"resumed"' not in t:
        i=t.index('"WorkerRegistration": {')
        j=t.index('"properties": {',i)+len('"properties": {')
        t=t[:j]+'''
        "resumed": {
          "description": "The tasks a worker is still running when it joins again after the scheduler lost its state (a restart): the scheduler adopts them instead of running the same actions a second time.",
          "type": "array",
          "maxItems": 512,
          "items": {
            "type": "object",
            "additionalProperties": false,
            "required": ["task", "instance", "action"],
            "properties": {
              "task": { "type": "string", "pattern": "^[0-9a-f-]{36}\\\\.[0-9]{1,3}$" },
              "instance": { "type": "string", "maxLength": 64 },
              "action": { "type": "object", "additionalProperties": false, "required": ["hash", "size"], "properties": { "hash": { "type": "string", "pattern": "^[a-f0-9]{64}$" }, "size": { "type": "integer", "minimum": 0 } } }
            }
          }
        },'''+t[j:]
        wr(p,t)
        json.loads(t)
    p="🏗️build/🌐️remote-execution/🟦️.ts"
    t=rd(p)
    if "resumed" not in t:
        i=t.index('export type RemoteExecutionWorkerRegistrationV1=Readonly<{')
        j=t.index('}>',i)
        t=t[:j]+';resumed?:readonly Readonly<{task:string;instance:string;action:RemoteExecutionDigestV1}>[]'+t[j:]
        wr(p,t)
    p="🏗️build/🌐️remote-execution/👷️worker/🟦️.ts"
    t=rd(p)
    if "resumable" not in t:
        t=rep(t,' const running=new Map<string,AbortController>();\n',' const running=new Map<string,AbortController>(),resumable=new Map<string,Readonly<{instance:string;action:{hash:string;size:number}}>>();\n')
        t=rep(t,'origin,capacity:options.capacity??','origin,...resumable.size>0?{resumed:[...resumable].map(([task,row])=>({task,instance:row.instance,action:row.action}))}:{},capacity:options.capacity??')
        t=rep(t,'for(let attempt=0;attempt<5;attempt++){','for(let attempt=0;attempt<8;attempt++){')
        t=rep(t,'catch(error){if(attempt===4)say("reconnecting",`report lost: ${(error as Error).message}`);else await runtimeSleepV1(200*2**attempt);}','catch(error){if(attempt===7)say("reconnecting",`report lost: ${(error as Error).message}`);else{if(/Scheduler answered 404/u.test((error as Error).message))await register();await runtimeSleepV1(Math.min(3000,200*2**attempt));}}')
        t=rep(t,'  running.delete(task.id);publish();wake?.();','  running.delete(task.id);resumable.delete(task.id);publish();wake?.();')
        t=rep(t,'const controller=new AbortController();running.set(answer.task.id,controller);publish();','const controller=new AbortController();running.set(answer.task.id,controller);resumable.set(answer.task.id,{instance:answer.task.instance,action:{hash:answer.task.action.hash,size:answer.task.action.size}});publish();')
        wr(p,t)
    p="🏗️build/🌐️remote-execution/🗓️scheduler/🟦️.ts"
    t=rd(p)
    if "adopted" not in t:
        t=rep(t,'cancelRequested:boolean;lingerTimer?:ReturnType<typeof setTimeout>;','cancelRequested:boolean;adopted?:true;lingerTimer?:ReturnType<typeof setTimeout>;')
        old='  this.workers.set(id,{id,registration,lastSeen:this.clock(),running:new Set(),cancel:new Set(),draining:false,refusedUntil:0,refused:0});\n'
        new=old+open(os.path.dirname(os.path.abspath(__file__))+'/adopt.frag',encoding='utf8').read()
        t=rep(t,old,new)
        t=rep(t,'   if(operation.stage==="COMPLETED")return;\n   if(operation.offload&&!placementIdleElsewhereV1','   if(operation.stage!=="CACHE_CHECK")return;\n   if(operation.offload&&!placementIdleElsewhereV1')
        t=rep(t,' private requeue(operation:Operation,reason:string,failure:boolean):void{\n  const name=operation.worker??"";\n',' private requeue(operation:Operation,reason:string,failure:boolean):void{\n  const name=operation.worker??"";\n  if(operation.adopted===true)return this.finish(operation,{status:FAILED_BACK(reason),actionResult:null,cached:false,worker:name});\n')
        wr(p,t)
print("done",only or "all")
