import sys,hashlib
B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
R=B+"🌐️remote-execution/"
def edit(path,pairs):
    s=open(path,encoding="utf-8").read();before=hashlib.sha256(s.encode()).hexdigest()[:12]
    for old,new in pairs:
        if s.count(old)!=1: sys.exit(f"anchor count {s.count(old)} in {path}: {old[:70]!r}")
        s=s.replace(old,new)
    open(path,"w",encoding="utf-8").write(s)
    print(path.replace(B,""),before,"->",hashlib.sha256(s.encode()).hexdigest()[:12])

fields=["cancelledQueued","cancelledExecuting","workersReused","sweepStalls","queueWaitSamples","queueWaitP50Milliseconds","queueWaitP90Milliseconds","queueWaitMaxMilliseconds"]
schema_req='        "offloadRejected"\n      ],\n      "properties": {\n        "queued": {'
schema_props='        "offloadRejected": {\n          "type": "integer",\n          "minimum": 0\n        }\n      }\n    },\n    "WorkerRow": {'
edit(B+"🏃️execution/🧬️schema/🌐️remote-execution.json",[
 (schema_req,'        "offloadRejected",\n'+",\n".join(f'        "{f}"' for f in fields)+'\n      ],\n      "properties": {\n        "queued": {'),
 (schema_props,'        "offloadRejected": {\n          "type": "integer",\n          "minimum": 0\n        },\n'+",\n".join(f'        "{f}": {{\n          "type": "integer",\n          "minimum": 0\n        }}' for f in fields)+'\n      }\n    },\n    "WorkerRow": {'),
])

S=R+"🗓️scheduler/🟦️.ts"
edit(S,[
 ('"retried"|"cancelled"|"worker-joined"','"retried"|"worker-joined"'),
 ('refused:0,offloadRejected:0};\n','refused:0,offloadRejected:0,cancelledQueued:0,cancelledExecuting:0,workersReused:0,sweepStalls:0};\n private readonly waits:number[]=[];\n private lastTick=0;\n'),
 ('  if(every>0){this.timer=setInterval(()=>{this.sweep();},every);this.timer.unref();}\n }\n',
  '  this.lastTick=this.clock();\n  if(every>0){this.timer=setInterval(()=>{this.tick(every);},every);this.timer.unref();}\n }\n\n /** 🫀️ One beat of the sweep timer: when the timer came back much later than its interval, this process was stalled (the machine is starved), the heartbeats of the workers wait unread in the socket queue and their last-seen marks are stale through no fault of theirs, so that beat judges no worker and only counts the stall; the next beat judges on fresh marks. */\n tick(every:number):void{const now=this.clock(),stalled=now-this.lastTick>every*5;this.lastTick=now;if(stalled)this.counters.sweepStalls++;this.sweep(stalled);}\n'),
 (' register(registration:RemoteExecutionWorkerRegistrationV1):RemoteExecutionWorkerRegisteredV1{\n  const id=randomUUID();\n',
  ' register(registration:RemoteExecutionWorkerRegistrationV1):RemoteExecutionWorkerRegisteredV1{\n  const known=[...this.workers.values()].find(worker=>worker.registration.name===registration.name&&worker.registration.origin===registration.origin);\n  if(known){known.registration=registration;known.lastSeen=this.clock();this.counters.workersReused++;this.dispatch();return this.registered(known.id);}\n  const id=randomUUID();\n'),
 ('  return {version:1,id,heartbeatSeconds:this.options.heartbeatSeconds,pollSeconds:this.options.pollSeconds,leaseSeconds:this.options.leaseSeconds,cache:this.options.cache,language:this.options.language};\n }\n',
  '  return this.registered(id);\n }\n\n /** 🪪️ The answer to a registration: a machine that registers again under the same name and origin (its heartbeat or its poll was told "unknown", or two of its loops raced) keeps its one entry and its id instead of leaving an orphan entry that nothing heartbeats and the lease then reports as lost. */\n private registered(id:string):RemoteExecutionWorkerRegisteredV1{\n  return {version:1,id,heartbeatSeconds:this.options.heartbeatSeconds,pollSeconds:this.options.pollSeconds,leaseSeconds:this.options.leaseSeconds,cache:this.options.cache,language:this.options.language};\n }\n'),
 ('  this.emit("cancelled",this.label(operation));\n','  if(operation.stage==="EXECUTING")this.counters.cancelledExecuting++;else this.counters.cancelledQueued++;\n'),
 ('const plan=planDispatchV1(this.queue.map(operation=>this.view(operation)),','const plan=planDispatchV1(this.queue.filter(operation=>operation.lingerTimer===undefined).map(operation=>this.view(operation)),'),
 ('   operation.stage="EXECUTING";operation.worker=worker.id;operation.startedAt=this.clock();','   const waited=this.clock()-operation.queuedAt;this.waits.push(waited);if(this.waits.length>512)this.waits.shift();\n   operation.stage="EXECUTING";operation.worker=worker.id;operation.startedAt=this.clock();'),
 (' sweep():void{\n  const now=this.clock(),lease=this.options.leaseSeconds*1000;\n  for(const worker of [...this.workers.values()])if(',' sweep(grace=false):void{\n  const now=this.clock(),lease=this.options.leaseSeconds*1000;\n  for(const worker of grace?[]:[...this.workers.values()])if('),
 ('  return {counters:{...this.counters,queued:this.queue.length,executing:running},','  return {counters:{...this.counters,...this.waitStats(),queued:this.queue.length,executing:running},'),
 (' /** 🔢️ How the workers stand for a unit of `requester` right now',' /** 📏️ How long the actions that were placed waited in the queue: the last 512 assignments, as the number of samples and the median, the 90th percentile and the maximum in milliseconds (the measure the queue limit of the requesters is sized from). */\n private waitStats():Readonly<{queueWaitSamples:number;queueWaitP50Milliseconds:number;queueWaitP90Milliseconds:number;queueWaitMaxMilliseconds:number}>{\n  const sorted=[...this.waits].sort((left,right)=>left-right),at=(share:number):number=>sorted.length===0?0:sorted[Math.min(sorted.length-1,Math.floor(sorted.length*share))]!;\n  return {queueWaitSamples:sorted.length,queueWaitP50Milliseconds:at(0.5),queueWaitP90Milliseconds:at(0.9),queueWaitMaxMilliseconds:sorted.at(-1)??0};\n }\n\n /** 🔢️ How the workers stand for a unit of `requester` right now'),
])

W=R+"👷️worker/🟦️.ts"
edit(W,[
 (' const register=async():Promise<void>=>{\n  for(let delay=250;',' let registering:Promise<void>|undefined;\n const registerOnce=async():Promise<void>=>{\n  for(let delay=250;'),
 (' };\n await register();\n if(!registered||!content)return;',' };\n const register=():Promise<void>=>{registering??=registerOnce().finally(()=>{registering=undefined;});return registering;};\n await register();\n if(!registered||!content)return;'),
 (' const heartbeat=setInterval(()=>{\n  const id=registered!.id;\n  void call<RemoteExecutionHeartbeatAnswerV1>("POST",`/v1/workers/${id}/heartbeat`,{version:1,running:[...running.keys()]},options.signal,10000).then(answer=>{if(answer){if(!answer.known)void register();cancel(answer.cancel);}}).catch(()=>{});\n },registered.heartbeatSeconds*1000);',
  ' let beating=false,beatLost=false;\n const heartbeat=setInterval(()=>{\n  if(beating)return;\n  beating=true;const id=registered!.id;\n  void call<RemoteExecutionHeartbeatAnswerV1>("POST",`/v1/workers/${id}/heartbeat`,{version:1,running:[...running.keys()]},options.signal,10000).then(answer=>{beatLost=false;if(answer){if(!answer.known)void register();cancel(answer.cancel);}}).catch(error=>{if(!beatLost&&!options.signal.aborted)say("reconnecting",`heartbeat lost: ${(error as Error).message}`);beatLost=true;}).finally(()=>{beating=false;});\n },registered.heartbeatSeconds*1000);'),
])

K=R+"🛰️sidecar/🗓️scheduler/🟦️.ts"
edit(K,[
 ('if(event.stage!=="submitted"&&event.stage!=="completed"&&event.stage!=="assigned")say','if(event.stage!=="submitted"&&event.stage!=="completed"&&event.stage!=="assigned")say'),
 (' await publish();\n try{\n  while(!context.signal.aborted){',' await publish();\n let lastSummary="";\n try{\n  while(!context.signal.aborted){'),
 ('   await publish();\n  }\n }finally{','   await publish();\n   const counters=service.scheduler.status().counters,summary=`cancelled ${counters.cancelled} (before start ${counters.cancelledQueued}, running ${counters.cancelledExecuting}), workers re-registered ${counters.workersReused}, sweep stalls ${counters.sweepStalls}, queue wait p50/p90/max ${counters.queueWaitP50Milliseconds}/${counters.queueWaitP90Milliseconds}/${counters.queueWaitMaxMilliseconds} ms over ${counters.queueWaitSamples} assignments`;\n   if(summary!==lastSummary){lastSummary=summary;say(`scheduler summary: ${summary}`);}\n  }\n }finally{'),
])
