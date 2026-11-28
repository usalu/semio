/** 📇️ The default view of `repo fleet status`: the detail report folded to what an agent acts on, derived from it alone (nothing is read again), so the summary is always derivable from the detail. Totals per window, per-origin counters only where anomalous, findings grouped by id with counts and subjects, workers that need a look as rows and the healthy ones as one list. The closed schema `Summary` is validated before it is printed.
 * @see ./🧬️schema/🔣️.json `Summary`, `Selection` and the `summary` policy */
export type FleetSummaryV1=Readonly<{version:1;view:"summary";atMs:number;workspace:string}&Record<string,any>>;
const SUMMARY=policy.summary,scalar=(value:unknown):boolean=>value===null||typeof value!=="object",positive=(values:readonly number[]):boolean=>values.some(value=>value>0);
const rateRank=(left:readonly [string,readonly number[]],right:readonly [string,readonly number[]]):number=>right[1][0]!-left[1][0]!||right[1][right[1].length-1]!-left[1][left[1].length-1]!||compare(left[0],right[0]);
const cut=(text:string,limit:number):string=>Array.from(text).length>limit?Array.from(text).slice(0,limit-1).join("")+"…":text;
const runWho=(run:Row):string=>(run.agent??run.session?.slice(run.session.indexOf(".")+1)??"-").slice(0,policy.runs.whoCharacters);
const runWhat=(run:Row):string=>run.operation??(commandName(run.command)!=="-"?commandName(run.command):run.exclusive?.[0]?.split(":")[0]?.slice(0,policy.runs.whatCharacters)??"-");

/** 🧮️ Folds a detail report to the summary; `limit` bounds the run rows. */
export function summarizeFleetReportV1(report:FleetReportV1,limit:number=20):FleetSummaryV1{
 const memory={...report.memory} as Row;delete memory.warden;
 const runs={count:report.runs.length,rows:(report.runs as Row[]).slice(0,limit).map(run=>({pid:run.pid,state:run.state,who:runWho(run),what:runWhat(run),...run.phase===undefined?{}:{phase:cut(run.phase,SUMMARY.phaseCharacters)},...run.done===undefined?{}:{done:run.done},...run.total===undefined?{}:{total:run.total},...run.etaSeconds===undefined?{}:{etaSeconds:run.etaSeconds},ageSeconds:run.ageSeconds,...run.priority===undefined?{}:{priority:run.priority},...run.waitingFor===undefined?{}:{waiting:{kind:run.waitingFor.kind,waitedSeconds:run.waitingFor.waitedSeconds}}}))};
 const graph=report.queues.graph,queues={graph:{builder:graph.builder,computers:graph.computers,slots:graph.slots,waiting:graph.waiting,longestWaitSeconds:graph.longestWaitSeconds},capacity:{grants:report.queues.capacity.grants,waiters:report.queues.capacity.waiters.length,floorMegabytes:report.queues.capacity.floorMegabytes,spareMegabytes:report.queues.capacity.spareMegabytes},leases:(report.queues.leases as Row[]).map(queue=>({resource:queue.resource,class:queue.class,holders:queue.holders.length,waiters:queue.waiters.length,longestWaitSeconds:Math.max(0,...queue.waiters.map((waiter:Row)=>waiter.waitedSeconds))})),leasesOmitted:report.queues.leasesOmitted};
 const bazel={slots:report.bazel.slots,live:report.bazel.live,held:report.bazel.held,rssMegabytes:report.bazel.rssMegabytes};
 const storage={hostFreeBytes:report.storage.hostFreeBytes,hostFreeFloorBytes:report.storage.hostFreeFloorBytes,overBytes:report.storage.machine.overBytes,fullImages:(report.storage.images as Row[]).filter(image=>image.usedBasisPoints!==null&&image.usedBasisPoints>=policy.findings.imageShownBasisPoints).map(image=>({id:image.id,usedBasisPoints:image.usedBasisPoints}))};
 const sealed={mode:report.sealed.mode,entries:report.sealed.entries.length,drifted:report.sealed.drifted,blocked:report.sealed.blocked};
 const generation=report.generation===null?null:{number:report.generation.number,ageSeconds:report.generation.ageSeconds,owners:report.generation.owners,dirty:report.generation.built.dirty,verifiedBasisPoints:report.generation.verified?.basisPoints??null};
 const groups=new Map<string,{id:string;severity:string;subjects:string[];count:number;text:string}>();
 for(const finding of report.findings){const key=`${finding.severity}\0${finding.id}`,group=groups.get(key)??{id:finding.id,severity:finding.severity,subjects:[],count:0,text:finding.text};group.count++;if(!group.subjects.includes(finding.subject))group.subjects.push(finding.subject);groups.set(key,group);}
 const findings=[...groups.values()].map(group=>({id:group.id,severity:group.severity,count:group.count,subjects:group.subjects.slice(0,SUMMARY.maximumSubjects),more:Math.max(0,group.subjects.length-SUMMARY.maximumSubjects),text:cut(group.text,SUMMARY.textCharacters)}));
 const failed=report.sources.filter(row=>!row.ok).map(row=>({id:row.id,...row.error===undefined?{}:{error:cut(row.error,SUMMARY.textCharacters)}})),sources={count:report.sources.length,milliseconds:report.sources.reduce((sum,row)=>sum+row.milliseconds,0),failed};
 const result:Record<string,any>={version:1,view:"summary",atMs:report.atMs,workspace:report.workspace,summary:report.summary,memory,runs,queues,bazel,storage,sealed,generation};
 if(report.scheduler!==undefined){const scheduler=report.scheduler as Row;result.scheduler={running:scheduler.running,queued:scheduler.queued,reservedMegabytes:scheduler.reservedMegabytes,budgetMegabytes:scheduler.budgetMegabytes,sharedTickets:scheduler.sharedTickets,longestWaitSeconds:scheduler.longestWaitSeconds};}
 if(report.fleet!==undefined){
  const fleet=report.fleet as Row,machines=fleet.machines as Row[],attention=machines.filter(machine=>machine.pressure!=="normal"||machine.worker==="refusing"||!machine.headroom||machine.why!==undefined||fleet.treeCode!==undefined&&machine.code!==undefined&&machine.code!==fleet.treeCode).map(machine=>({origin:machine.origin,worker:machine.worker,pressure:machine.pressure,headroom:machine.headroom,...machine.why===undefined?{}:{why:machine.why.code},...fleet.treeCode===undefined||machine.code===undefined||machine.code===fleet.treeCode?{}:{code:machine.code}}));
  result.fleet={machines:machines.length,queued:fleet.queued,executing:fleet.executing,offloadedIn:fleet.offloadedIn,offloadedOut:fleet.offloadedOut,refused:fleet.refused,declined:fleet.declined,retried:fleet.retried,cacheHit:fleet.cacheHit,cacheMiss:fleet.cacheMiss,...fleet.treeCode===undefined?{}:{treeCode:fleet.treeCode},...fleet.cache===undefined?{}:{cache:fleet.cache},...fleet.claims===undefined?{}:{claims:fleet.claims},...fleet.keys===undefined?{}:{keys:{origins:fleet.keys.origins,units:fleet.keys.units,differing:fleet.keys.diffs.length,dependent:fleet.keys.diffs.filter((row:Row)=>row.sameSource).length}},attention};
 }
 if(report.workers!==undefined){
  const rows=report.workers.rows as Row[],needs=(row:Row):boolean=>row.liveness!=="ready"||row.refusal!==null||row.tree!==undefined&&row.tree!==null&&row.tree.verdict!=="ok"||row.memory!==null&&!row.memory.absent&&row.memory.level!=="calm";
  result.workers={counts:report.workers.counts,rows:rows.filter(needs).map(row=>({origin:row.origin,liveness:row.liveness,stage:row.stage,stageIndex:row.stageIndex,stageCount:row.stageCount,attempts:row.attempts,recordAgeSeconds:row.recordAgeSeconds,refusal:row.refusal===null?null:row.refusal.code,tree:row.tree===undefined||row.tree===null?null:row.tree.verdict,level:row.memory===null||row.memory.absent?null:row.memory.level,availableMegabytes:row.memory===null||row.memory.absent?null:row.memory.availableMegabytes,swapUsedMegabytes:row.memory===null||row.memory.absent?null:row.memory.swapUsedMegabytes,failed:row.recent===undefined||row.recent===null?null:row.recent.failed})),healthy:rows.filter(row=>!needs(row)).map(row=>row.origin)};
 }
 if(report.cas!==undefined)result.cas=report.cas;
 if(report.retention!==undefined)result.retention=report.retention;
 if(report.offload!==undefined){const offload=report.offload as Row;result.offload={local:offload.local,fleet:offload.fleet,out:offload.out,refused:offload.refused,fellBack:offload.fellBack,deadline:offload.deadline,lost:offload.lost,silent:offload.silent,reasons:Object.fromEntries(Object.entries(offload.reasons as Record<string,number>).filter(([,count])=>count>0).sort((left,right)=>right[1]-left[1]||compare(left[0],right[0]))),...offload.spool===undefined?{}:{spool:offload.spool}};}
 if(report.rates!==undefined){
  const rates=report.rates as Row,anomalous=new Set<string>(SUMMARY.anomalousMetrics),totals=Object.entries(rates.fleet as Record<string,number[]>).filter(([,values])=>positive(values)).sort(rateRank),origins:Record<string,Record<string,number[]>>={};
  let omitted=totals.length-Math.min(totals.length,SUMMARY.maximumFleetMetrics);
  for(const [origin,metrics] of Object.entries(rates.origins as Record<string,Record<string,number[]>>)){
   const rows=Object.entries(metrics).filter(([id,values])=>anomalous.has(id)&&(values[0]??0)>0).sort(rateRank);
   if(rows.length===0)continue;
   origins[origin]=Object.fromEntries(rows.slice(0,SUMMARY.maximumOriginMetrics));omitted+=Math.max(0,rows.length-SUMMARY.maximumOriginMetrics);
  }
  result.rates={windowsSeconds:rates.windowsSeconds,fleet:Object.fromEntries(totals.slice(0,SUMMARY.maximumFleetMetrics)),origins,omitted};
 }
 result.findings=findings;result.sources=sources;
 return Object.freeze(result) as FleetSummaryV1;
}

/** ✅️ Checks a summary against its own closed schema; the empty list means valid. */
export function validateFleetSummaryV1(summary:unknown):readonly string[]{
 return validateJsonSchemaSubset(schema.definitions.Summary as never,summary,schema as never).map(error=>String(typeof error==="string"?error:(error as {message?:string}).message??JSON.stringify(error)));
}

/** ✅️ Checks a selection of detail sections against its own closed schema; the empty list means valid. */
export function validateFleetSelectionV1(selection:unknown):readonly string[]{
 return validateJsonSchemaSubset(schema.definitions.Selection as never,selection,schema as never).map(error=>String(typeof error==="string"?error:(error as {message?:string}).message??JSON.stringify(error)));
}

/** 🎯️ The origins a detail report knows (workers, machines, counters), sorted. */
export function fleetOriginsV1(report:FleetReportV1):readonly string[]{
 return [...new Set([...(report.workers?.rows as Row[]|undefined??[]).map(row=>row.origin as string),...(report.fleet?.machines as Row[]|undefined??[]).map(row=>row.origin as string),...Object.keys(report.rates?.origins??{})])].sort(natural);
}

/** 🎯️ Keeps of a detail report only what concerns one origin (its worker, machine, counters and findings; the host and fleet-wide parts stay). An unknown origin is refused with the known ones in both languages. */
export function filterFleetReportV1(report:FleetReportV1,origin:string):FleetReportV1{
 const known=fleetOriginsV1(report);
 if(!known.includes(origin))throw Error(fleetTextV1("en","error.origin",{origin,list:known.join(", ")})+"\n"+fleetTextV1("de","error.origin",{origin,list:known.join(", ")}));
 const rates=report.rates===undefined?undefined:{...report.rates,origins:Object.fromEntries(Object.entries(report.rates.origins as Row).filter(([name])=>name===origin))};
 return Object.freeze({...report,...report.workers===undefined?{}:{workers:{...report.workers,rows:(report.workers.rows as Row[]).filter(row=>row.origin===origin)}},...report.fleet===undefined?{}:{fleet:{...report.fleet,machines:(report.fleet.machines as Row[]).filter(row=>row.origin===origin)}},...rates===undefined?{}:{rates},findings:report.findings.filter(finding=>finding.subject===origin||finding.values.origin===origin)}) as FleetReportV1;
}

/** 🗂️ The named top level sections of a detail report as one document, with the origin when one was asked for. */
export function selectFleetSectionsV1(report:FleetReportV1,sections:readonly string[],origin?:string):Row{
 const picked:Record<string,unknown>={};
 for(const name of sections){const value=(report as Row)[name];if(value!==undefined)picked[name]=value;}
 return {version:1,view:"detail",atMs:report.atMs,workspace:report.workspace,...origin===undefined?{}:{origin},sections:picked};
}

/** 🖨️ Renders JSON for reading: objects expand down to `depth` levels, an object or array of plain values stays on one line when it is short. */
export function renderFleetJsonV1(value:unknown,depth:number=3,width:number=200):string{
 const flat=(item:unknown):boolean=>scalar(item)||(Array.isArray(item)?item:Object.values(item as object)).every(entry=>scalar(entry)||Array.isArray(entry)&&entry.every(scalar));
 const walk=(item:unknown,level:number,pad:string):string=>{
  const one=JSON.stringify(item);
  if(scalar(item)||level>=depth||flat(item)&&one.length<=width)return one;
  const inner=pad+" ",rows=Array.isArray(item)?item.map(entry=>inner+walk(entry,level+1,inner)):Object.entries(item as object).map(([key,entry])=>inner+JSON.stringify(key)+": "+walk(entry,level+1,inner));
  return (Array.isArray(item)?"[":"{")+"\n"+rows.join(",\n")+"\n"+pad+(Array.isArray(item)?"]":"}");
 };
 return walk(value,0,"");
}

/** 🖨️ Renders the summary as compact text lines in the chosen language; no section is longer than a handful of lines. */
export function formatFleetSummaryV1(language:FleetLanguageV1,summary:FleetSummaryV1):readonly string[]{
 const say=(key:string,values:Readonly<Record<string,string|number>>={})=>fleetTextV1(language,key,values),duration=(seconds:number)=>fleetDurationV1(language,seconds),lines:string[]=[],memory=summary.memory as Row,counts=summary.summary as Row;
 lines.push(say("header",{time:clock(summary.atMs),running:counts.running,blocked:counts.blocked,waiting:counts.waiting,alerts:counts.alerts,warnings:counts.warnings}));
 lines.push(memory.sampleAgeSeconds===null?say("memoryNone",{admitted:memory.admittedMegabytes,observed:memory.observedMegabytes,grants:memory.grants}):say("memory",{age:duration(memory.sampleAgeSeconds),admitted:memory.admittedMegabytes,observed:memory.observedMegabytes,grants:memory.grants,available:memory.availableMegabytes,total:memory.totalMegabytes,spare:memory.spareMegabytes,floor:memory.floorMegabytes,pressure:memory.pressure,swap:memory.swapUsedMegabytes??"n/a",load:memory.load??"n/a"}));
 const runs=summary.runs as Row;
 if(runs.count===0)lines.push(say("runsNone"));
 else{lines.push(say("runsHeader",{count:runs.count,shown:runs.rows.length}));for(const run of runs.rows as Row[])lines.push(say("summary.run",{pid:run.pid,state:say(`state.${run.state}`),who:run.who,what:run.what,progress:run.phase===undefined?"":say("progress",{phase:run.phase,counter:counter(run)}),age:duration(run.ageSeconds),waiting:run.waiting===undefined?"":say("summary.run.waiting",{kind:run.waiting.kind,waited:duration(run.waiting.waitedSeconds)})}));}
 const queues=summary.queues as Row;
 lines.push(say("summary.queues",{graph:queues.graph.waiting,longest:duration(queues.graph.longestWaitSeconds),builder:queues.graph.builder===null?"-":`pid ${queues.graph.builder.pid}${queues.graph.builder.phase===undefined?"":" "+queues.graph.builder.phase+counter(queues.graph.builder)}`,capacity:queues.capacity.waiters,spare:queues.capacity.spareMegabytes,leases:queues.leases.length+queues.leasesOmitted}));
 for(const queue of queues.leases as Row[])lines.push(say("summary.lease",{resource:queue.resource,holders:queue.holders,waiters:queue.waiters,longest:duration(queue.longestWaitSeconds)}));
 const scheduler=summary.scheduler as Row|undefined;
 if(scheduler!==undefined)lines.push(say("scheduler",{running:scheduler.running,queued:scheduler.queued,reserved:scheduler.reservedMegabytes,budget:scheduler.budgetMegabytes,shared:scheduler.sharedTickets,longest:scheduler.longestWaitSeconds>0?say("scheduler.longest",{waited:duration(scheduler.longestWaitSeconds)}):""}));
 const fleet=summary.fleet as Row|undefined;
 if(fleet!==undefined){
  lines.push(say("fleet",fleet as never));
  if(fleet.cache!==undefined)lines.push(say("fleet.cache",fleet.cache));
  if(fleet.claims!==undefined)lines.push(say("fleet.claims",{active:fleet.claims.active,waiting:fleet.claims.waiting,oldest:duration(fleet.claims.oldestSeconds)}));
  if(fleet.keys!==undefined&&fleet.keys.differing>0)lines.push(say("summary.keys",fleet.keys));
  for(const machine of fleet.attention as Row[])lines.push(say("summary.machine",{origin:machine.origin,worker:machine.worker,pressure:machine.pressure,headroom:machine.headroom?"":say("fleet.noheadroom"),why:machine.why===undefined||machine.why==="no-headroom"?"":" ("+machine.why+")",code:machine.code===undefined?"":" code "+machine.code}));
 }
 const workers=summary.workers as Row|undefined;
 if(workers!==undefined){
  lines.push(say("workers",workers.counts));
  for(const row of workers.rows as Row[])lines.push(say("summary.worker",{origin:row.origin,state:say(`workers.state.${row.liveness}`),stage:row.stage??"-",index:row.stageIndex,count:row.stageCount,attempts:row.attempts,record:row.recordAgeSeconds===null?"-":duration(row.recordAgeSeconds),detail:((parts:string[])=>parts.length===0?"":"; "+parts.join("; "))([row.refusal,row.tree===null?"":say("summary.tree",{verdict:say(`workers.tree.verdict.${row.tree}`)}),row.level===null?"":say("summary.level",{level:say(`workers.level.${row.level}`),available:row.availableMegabytes,swap:row.swapUsedMegabytes??"?"}),row.failed?say("summary.failed",{count:row.failed}):""].filter((part):part is string=>part!==null&&part!==""))}));
  if(workers.healthy.length>0)lines.push(say("summary.healthy",{list:workers.healthy.join(" ")}));
 }
 const cas=summary.cas as Row|undefined;
 if(cas!==undefined)lines.push(say("cas",{used:cas.usedBytes===null?"n/a":gib(cas.usedBytes),effective:gib(cas.effectiveCapBytes),percent:cas.usedBasisPoints===null?"?":Math.floor(cas.usedBasisPoints/100),cap:gib(cas.capBytes),high:gib(cas.highWatermarkBytes),low:gib(cas.lowWatermarkBytes)}));
 const offload=summary.offload as Row|undefined;
 if(offload!==undefined){
  lines.push(say("offload",offload as never));
  const reasons=Object.entries(offload.reasons as Record<string,number>).slice(0,SUMMARY.maximumReasonsShown).map(([name,count])=>`${name} ${count}`).join(", ");
  if(reasons!=="")lines.push(say("summary.reasons",{list:reasons}));
 }
 const rates=summary.rates as Row|undefined;
 if(rates!==undefined&&(Object.keys(rates.fleet).length>0||Object.keys(rates.origins).length>0)){
  lines.push(say("rates",{windows:(rates.windowsSeconds as number[]).map(duration).join(" / ")}));
  for(const [id,values] of Object.entries(rates.fleet as Record<string,number[]>))lines.push(say("summary.rate",{label:metricLabelV1(id,language),values:values.join(" / ")}));
  for(const [origin,metrics] of Object.entries(rates.origins as Record<string,Record<string,number[]>>))lines.push(say("summary.rate.origin",{origin,list:Object.entries(metrics).map(([id,values])=>`${id} ${values.join("/")}`).join(", ")}));
  if(rates.omitted>0)lines.push(say("summary.rate.omitted",{count:rates.omitted}));
 }
 const bazel=summary.bazel as Row,storage=summary.storage as Row,sealed=summary.sealed as Row,generation=summary.generation as Row|null;
 lines.push(say("bazel",{live:bazel.live,rss:bazel.rssMegabytes,held:bazel.held,slots:bazel.slots}));
 lines.push(say("summary.storage",{free:storage.hostFreeBytes===null?"n/a":gib(storage.hostFreeBytes),floor:gib(storage.hostFreeFloorBytes),over:gib(storage.overBytes),images:(storage.fullImages as Row[]).length===0?"":"; "+(storage.fullImages as Row[]).map(image=>`${image.id} ${Math.floor(image.usedBasisPoints/100)} %`).join(", ")}));
 lines.push(say("sealed",{mode:sealed.mode,count:sealed.entries,drifted:sealed.drifted,blocked:sealed.blocked}));
 lines.push(generation===null?say("generationNone"):say("summary.generation",{number:generation.number,age:generation.ageSeconds===null?"?":duration(generation.ageSeconds),owners:generation.owners,dirty:generation.dirty,percent:generation.verifiedBasisPoints===null?"?":Math.floor(generation.verifiedBasisPoints/100)}));
 if(summary.findings.length>0){
  lines.push(say("findingsHeader"));
  for(const finding of summary.findings as Row[])lines.push(finding.count===1?say("finding",{severity:say(`severity.${finding.severity}`),text:finding.text}):say("summary.finding",{severity:say(`severity.${finding.severity}`),count:finding.count,text:finding.text,subjects:finding.subjects.join(", ")+(finding.more>0?` +${finding.more}`:"")}));
 }
 const sources=summary.sources as Row;
 lines.push(sources.failed.length===0?say("sources",{count:sources.count,milliseconds:sources.milliseconds}):say("sourcesFailed",{failed:sources.failed.length,count:sources.count,list:sources.failed.map((row:Row)=>row.id).join(", ")}));
 lines.push(say("summary.hint"));
 return lines;
}

