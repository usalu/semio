/** 🤫️ quiet16 (derived from agents16's simulator): a deterministic, graph-free simulation of the live host capacity governor for concurrent agents on a declared machine.
 * The decision (`evaluateHostCapacityV1`), the sizing chain (`sizeHostRequestV1`, `estimateHostPhasesV1`, `scaleHostRequestV1`, `rampHostRequestV1`, `forecastHostQuietV1`, `quietHostRequestV1`, `hostRequestPriorityV1`), the growth (`growHostGrantV1`, `settleHostGrowthV1` inside it), the learning (`recordHostPhasesV1`, `recordHostForecastV1`) and the messages (`formatHostCapacityV1`) are the REAL modules of `--root`; the holder loops of `🎫️gate` (door admission poll, upgrade loop, heartbeat, grower) are mirrored here because they live in closures over real timers.
 * Executing Bazel segments (`executes`, the actions of an edit or a cold build) are born: the birth of the first one waits for a quiet grant to grow (`live.birthUpgrade`, the action-birth executor point of the Bazel script), and they run as wide as Bazel's jobs, which is the plan the tool was started with (`live.inheritedUpgrade`: the grant's jobs after the door's growth; otherwise the full request's jobs) unless the births are gated by the grant's live jobs (`live.birthWidth`); width scales a segment's footprint above its start level and stretches its time (the recorded runs used 8 jobs).
 * quiet16 adds what a checkout without the inherited upgrade does (`live.inheritedUpgrade` false): a quiet grant grows only at an executor point (a graph refresh asks before the builder custody and waits for the growth) or when its tree holds more than the quiet reservation (`live.outgrown`: the holder asks in the fair queue while the run goes on); the run is learned as executed when it asked or when its peak exceeded the quiet size of its class (`live.peakExecuted`).
 * Usage: `bun sim.ts --root <checkout> --scenario hot --host h32d-fleet [--store estimates.json] [--json out.json] [--lend] [--trace]`
 * @see ../data/scenarios.json
 * @see ../../../result-quiet16.md
 * @see ../../../result-agents16.md */
import {readFileSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";

const argv=process.argv.slice(2),flag=(name:string,fallback?:string)=>{const at=argv.indexOf("--"+name);return at<0?fallback:argv[at+1];},has=(name:string)=>argv.includes("--"+name);
const here=dirname(fileURLToPath(import.meta.url)),root=flag("root")!,scenarioName=flag("scenario","hot")!,hostName=flag("host","h32d-fleet")!,trace=has("trace");
const data=JSON.parse(readFileSync(join(here,"..","data","scenarios.json"),"utf8")),store0=JSON.parse(readFileSync(flag("store",join(here,"..","data","estimates-1450.json"))!,"utf8"));
const hc=await import(join(root,"🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🟦️.ts"));
const policy=hc.readHostCapacityPolicyV1(JSON.parse(readFileSync(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json"),"utf8")).hostCapacity);
const bazel=await import(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧵️sessions/🟦️.ts"));
const source=(rel:string)=>readFileSync(join(root,rel),"utf8"),scriptSource=source("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts"),gateSource=source("🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🎫️gate/🟦️.ts"),doorSource=source("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🟦️.ts");
const live={upgradeAllJobs:gateSource.includes("minimumJobs:full!.preferredJobs"),measuredRamp:doorSource.includes("rampHostRequestV1(base,scaled,estimate,measured)"),elasticFarm:doorSource.includes("elastic:true"),farmFree:source("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🟨️.mjs").includes('if (event === "disabled") pool.release();'),labels:doorSource.includes("label:ask.lineage"),behind:typeof hc.hostBlockersV1==="function",reducedLine:source("🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🔼️upgrade/🟦️.ts").includes("upgradedReduced"),birthUpgrade:scriptSource.includes("await upgradeBuildCapacityV1(signal);"),birthWidth:scriptSource.includes("await width.enter(socket,signal)"),inheritedUpgrade:gateSource.includes(".requestHostCapacityUpgradeV1(environment,options.control)"),outgrown:gateSource.includes("outgrown"),peakExecuted:doorSource.includes("run.peakMegabytes>forecast.megabytes")};
const scenario=data.scenarios[scenarioName],host=data.hosts[hostName],TICK=100,newest=Math.max(...Object.values(store0.classes as Record<string,{samples:{atMs:number}[]}>).flatMap(row=>row.samples.map(sample=>sample.atMs)));
const START=newest+60000,total=host.declaredMegabytes,floor=hc.hostFloorMegabytesV1(policy,{totalMegabytes:total}),capacity=Math.max(0,total-floor);
const slotsPlanned=bazel.planBazelServersV1({cores:host.declaredCores,memoryMegabytes:total},{floorBasisPoints:policy.memory.floorBasisPoints,floorMinimumMegabytes:policy.memory.floorMinimumMegabytes,floorMaximumMegabytes:policy.memory.floorMaximumMegabytes}).slots;

/** 🎲️ A seeded generator, so every run of the simulation draws the same poll jitter. */
function seeded(seed:number):()=>number{return()=>{seed=(seed+0x6D2B79F5)|0;let value=Math.imul(seed^(seed>>>15),1|seed);value=(value+Math.imul(value^(value>>>7),61|value))^value;return((value^(value>>>14))>>>0)/4294967296;};}
const random=seeded(1027);

type Segment={kind:"door"|"served"|"plan"|"refresh"|"bazel";seconds:number;fp:number;rss:number;cpu:number;peakAt?:number;farmAfter?:number;executes?:boolean};
type Run={agent:Agent;label:string;spec:{target:string;lineage:string[];kinds:string[];segments:Segment[]};id:string;pid:number;arrivedAt:number;admittedAt?:number;endedAt?:number;base:any;request:any;growth:any;admission:any;quiet:boolean;estimate:any;segment:number;progress:number;level:number;waiting:"admit"|"upgrade"|"custody"|"slot"|undefined;waitSince:number;queued:number;upgradeWait:number;custodyWait:number;slotWait:number;slot?:number;peak:number;marks:[number,number][];executed:boolean;farm?:Farm;farmAsked:boolean;nextPoll:number;nextUpgradePoll:number;lastProgress:number;nextBeat:number;nextGrow:number;settled:any;rssPeak:number;fpPeak:number;jobsAtStart?:number;pausing:boolean;asked:boolean;quietSize:number;growWait:number};
type Farm={refusedAt?:number;id:string;pid:number;request:any;arrivedAt:number;admittedAt?:number;nextPoll:number;lastProgress:number;owner:Run};
type Agent={offset?:number;name:string;language:"en"|"de";steps:string[];step:number;run?:Run;readyAt:number;session:string;done:boolean;lines:string[]};

let now=START,store=store0,pids=1000,ids=0,custody:Run|undefined;
const grants=new Map<string,any>(),holders=new Map<string,Run|Farm>(),waiters:any[]=[],custodyQueue:Run[]=[],slots:(Run|undefined)[]=[],idleServers=new Set<number>(),events:any[]=[],timeline:any[]=[],lines:any[]=[];
const prime:Agent={name:"P",language:"en",steps:scenario.prime??[],step:0,readyAt:START,session:"f00000000000.s",done:false,lines:[]};
const agents:Agent[]=[prime,...scenario.agents.map((row:any,index:number)=>({name:row.name,language:row.language,steps:row.steps,step:0,readyAt:Infinity,session:`a${index}0000000000.s`,done:false,lines:[],offset:(row.delay??index*scenario.stagger)*1000}))];
let T0=Infinity;
const jitter=()=>policy.wait.pollMilliseconds*(1-policy.wait.jitterBasisPoints/10000+random()*2*policy.wait.jitterBasisPoints/10000);
const id=()=>`sim${String(++ids).padStart(19,"0")}`;

/** 📈️ The footprint and RSS of a run's tree right now: a waiting run holds its door level, a running segment ramps from the previous level to its peak and holds. */
/** 📏️ The share of its full width an executing Bazel segment runs with now (1 for every other segment). */
function widthOf(run:Run,segment:Segment):number{
 if(segment.kind!=="bazel"||segment.executes!==true)return 1;
 const grant=grants.get(run.id),preferred=run.growth.preferredJobs,jobs=live.birthWidth?Math.min(grant?.jobs??preferred,preferred):live.inheritedUpgrade?Math.min(run.jobsAtStart??preferred,preferred):preferred;
 return Math.max(1,jobs)/preferred;
}
function treeOf(run:Run):{fp:number;rss:number}{
 if(run.admittedAt===undefined||run.endedAt!==undefined)return{fp:0,rss:0};
 const segment=run.spec.segments[run.segment];if(!segment)return{fp:0,rss:0};
 const previous=run.segment===0?{fp:120,rss:250}:run.spec.segments[run.segment-1]!;
 if(run.waiting!==undefined)return{fp:Math.max(previous.fp*(run.segment===0?1:0.6),200),rss:Math.max(previous.rss*0.8,400)};
 const share=Math.min(1,run.progress/((segment.peakAt??segment.seconds)*1000));
 const width=widthOf(run,segment);
 return{fp:Math.round(previous.fp+(segment.fp-previous.fp)*share*width),rss:Math.round(previous.rss+(segment.rss-previous.rss)*share*width)};
}
const graphChildOf=(run:Run)=>{const segment=run.spec.segments[run.segment];return segment?.kind==="refresh"&&run.waiting===undefined?Math.max(0,treeOf(run).fp-300):0;};
const ours=()=>[...holders.values()].filter((row):row is Run=>"agent" in row);

/** 🖥️ The host sample the gate would read now: the real reducer over counters built from the declared machine, the foreign memory and our trees. */
function sample():any{
 const used=host.foreignMegabytes+ours().reduce((sum,run)=>sum+treeOf(run).fp,0)+idleServers.size*data.bazel.idleServerMegabytes,free=Math.max(0,host.realMegabytes-used);
 const load=host.foreignLoad+ours().reduce((sum,run)=>sum+(run.waiting===undefined?run.spec.segments[run.segment]?.cpu??0:0),0);
 return hc.reduceHostSampleV1(policy,{platform:"darwin",cores:host.declaredCores,totalMegabytes:total,atMs:now,load},{availableBytes:free*1048576,compressedBytes:0,constrainedBytes:null,swapUsedBytes:0,swappedOutBytes:0,pressureLevel:"normal",stallPercent:null,openFiles:null,openFilesMaximum:null,attachedImages:null});
}

/** 🗣️ Writes one product line in the agent's language and keeps it for the verdict. */
function say(agent:Agent|undefined,line:string,kind:string,extra:any={}):void{const row={t:Math.round((now-START)/100)/10,agent:agent?.name??"-",kind,line,...extra};lines.push(row);agent?.lines.push(line);if(trace)console.log(`[DEBUG] ${row.t.toFixed(1).padStart(7)} ${row.agent} ${line}`);}

/** ⏳️ The waiting line of the door, exactly as `🎫️gate` renders it (`waitingLine`). */
function waitingLine(language:"en"|"de",request:any,sampled:any,evaluation:any,running:number,seconds:number):string{
 const behindText=live.behind?hc.describeHostBlockersV1(language,hc.hostBlockersV1(policy,[...grants.values()],waiters,evaluation,request.id,now)):"";
 const position=evaluation.decisions.findIndex((row:any)=>row.id===request.id),mine=evaluation.decisions[position],before=evaluation.decisions.slice(0,position),spare=evaluation.headroomMegabytes-before.reduce((sum:number,row:any)=>sum+row.reservedMegabytes,0);
 return hc.formatHostCapacityV1(language,"waiting",{seconds,operation:request.operation,reason:hc.describeHostReasonV1(language,mine.reason),need:Math.min(request.fixedMegabytes+request.minimumJobs*request.perJobMegabytes,capacity),spare,floor:evaluation.floorMegabytes,load:sampled.load===null?"n/a":sampled.load,running,ahead:position,behind:behindText});
}
/** 🧾️ Who a waiter stands behind at this evaluation: every live grant and every request decided before it, by label. */
function behind(evaluation:any,request:any):string[]{const position=evaluation.decisions.findIndex((row:any)=>row.id===request.id);return[...[...grants.values()].filter(grant=>grant.id!==request.id).map(grant=>labelOf(grant.id)),...evaluation.decisions.slice(0,position).map((row:any)=>labelOf(row.id))];}
const labelOf=(grantId:string)=>{const holder=holders.get(grantId)??pending.get(grantId);return holder===undefined?grantId:"agent" in holder?`${holder.agent.name}:${holder.label}`:`${holder.owner.agent.name}:farm`;};
const pending=new Map<string,Run|Farm>(),records:any[]=[];

/** 🚪️ Starts the next run of an agent: sizes it with the real chain of `withBuildCapacityV1` and joins the door queue. */
function startRun(agent:Agent):void{
 const label=agent.steps[agent.step]!,spec=data.runs[label],runId=id(),targetName=spec.target.split(":")[1];
 const sized=hc.sizeHostRequestV1(policy,{id:runId,operation:"run",preferredJobs:8,kinds:spec.kinds,targets:[targetName]}),base=live.labels?{...sized,label:spec.target}:sized,estimate=hc.estimateHostPhasesV1(policy,store,spec.lineage,base,now);
 const scaled=hc.scaleHostRequestV1(policy,{...base,priority:hc.hostRequestPriorityV1(base.priority,"agent",estimate)},estimate.ratioBasisPoints,capacity,estimate.floorMegabytes);
 const measured=live.measuredRamp&&hc.hostClassBoundV1(policy,store,spec.lineage,now).source==="measured";
 let request=hc.rampHostRequestV1(base,scaled,estimate,measured);
 let growth=estimate.growRatioBasisPoints<estimate.ratioBasisPoints?{...hc.rampHostRequestV1(base,hc.scaleHostRequestV1(policy,{...base,priority:request.priority},estimate.growRatioBasisPoints,capacity,estimate.floorMegabytes),estimate,measured)}:request;
 const forecast=hc.forecastHostQuietV1(policy,store,spec.lineage[spec.lineage.length-1],now),quiet=forecast.quiet===true&&forecast.megabytes<growth.fixedMegabytes+growth.preferredJobs*growth.perJobMegabytes,admission=quiet?hc.quietHostRequestV1(growth,forecast):request;
 const run:Run={agent,label,spec,id:runId,pid:++pids,arrivedAt:now,base,request,growth,admission,quiet,estimate,segment:0,progress:0,level:0,waiting:"admit",waitSince:now,queued:0,upgradeWait:0,custodyWait:0,slotWait:0,peak:0,marks:[],executed:false,farmAsked:false,nextPoll:now,nextUpgradePoll:Infinity,lastProgress:-Infinity,nextBeat:Infinity,nextGrow:Infinity,settled:undefined,rssPeak:0,fpPeak:0,pausing:false,asked:false,quietSize:forecast.megabytes,growWait:0};
 agent.run=run;pending.set(runId,run);
 if(estimate.samples>0)say(agent,hc.formatHostCapacityV1(agent.language,"estimated",{operation:"run",percent:Math.round(estimate.ratioBasisPoints/100),samples:estimate.samples}),"estimated");
 if(quiet)say(agent,hc.formatHostCapacityV1(agent.language,forecast.followUpMilliseconds===undefined?"quiet":"quietFollowUp",{operation:"run",samples:forecast.samples,percent:Math.round(forecast.shareBasisPoints/100),seconds:Math.round((forecast.followUpMilliseconds??0)/1000),reserved:admission.fixedMegabytes,full:growth.fixedMegabytes+growth.preferredJobs*growth.perJobMegabytes}),"quiet");
 waiters.push(hc.requireHostCapacityV1("Waiter",{request:admission,arrivedAtMs:now,pid:run.pid,session:agent.session,weight:1}));
 events.push({t:now-START,agent:agent.name,run:label,event:"arrive",reserved:admission.fixedMegabytes+admission.minimumJobs*admission.perJobMegabytes,full:growth.fixedMegabytes+growth.preferredJobs*growth.perJobMegabytes,quiet});
}

/** 🎫️ One door poll of a run (or a farm request): the real decision over the whole ledger and queue; admits or prints the waiting line every `progressMilliseconds`. */
function doorPoll(holder:Run|Farm):void{
 const request="agent" in holder?holder.admission:holder.request,agent="agent" in holder?holder.agent:holder.owner.agent,sampled=sample(),evaluation=hc.evaluateHostCapacityV1(policy,sampled,[...grants.values()],waiters,now),mine=evaluation.decisions.find((row:any)=>row.id===request.id);
 const arrived="agent" in holder?holder.arrivedAt:holder.arrivedAt;
 if(mine.verdict==="admit"){
  const grant=hc.requireHostCapacityV1("Grant",{id:request.id,operation:request.operation,jobs:mine.jobs,cpu:mine.jobs*request.cpuPerJob,reservedMegabytes:mine.reservedMegabytes,admittedAtMs:now,observedMegabytes:null,pid:holder.pid,bootAtMs:1793050960000,..."agent" in holder||live.elasticFarm?{session:agent.session}:{},...request.label===undefined?{}:{label:request.label},...request.rampMilliseconds===undefined?{}:{rampMilliseconds:request.rampMilliseconds},...request.rampSlot===undefined?{}:{rampSlot:request.rampSlot},...request.quiet===true?{quiet:true,arrivedAtMs:arrived}:{}});
  grants.set(request.id,grant);holders.set(request.id,holder);pending.delete(request.id);waiters.splice(waiters.findIndex(row=>row.request.id===request.id),1);holder.admittedAt=now;
  const seconds=Math.round((now-arrived)/1000);
  say(agent,mine.reason==="override"?hc.formatHostCapacityV1(agent.language,"override",{operation:request.operation,seconds,reason:hc.describeHostReasonV1(agent.language,hc.softHostReasonV1(policy,sampled)??"load")}):mine.reason==="reduced"?hc.formatHostCapacityV1(agent.language,"agent" in holder&&holder.growth.perJobMegabytes>0?"reducedGrows":"reduced",{operation:request.operation,jobs:mine.jobs,preferred:request.preferredJobs}):hc.formatHostCapacityV1(agent.language,"admitted",{operation:request.operation,jobs:mine.jobs,reserved:mine.reservedMegabytes,available:sampled.availableMegabytes,floor:evaluation.floorMegabytes}),"admitted",{who:"agent" in holder?holder.label:"farm",queued:seconds});
  events.push({t:now-START,agent:agent.name,run:"agent" in holder?holder.label:"farm",event:"admit",jobs:mine.jobs,reserved:mine.reservedMegabytes,reason:mine.reason,waited:seconds});
  if("agent" in holder){holder.queued+=now-arrived;holder.waiting=undefined;holder.nextBeat=now+Math.min(3000,policy.wait.heartbeatMilliseconds);holder.nextGrow=now+policy.wait.heartbeatMilliseconds;if(holder.quiet&&live.inheritedUpgrade){holder.pausing=true;holder.asked=true;holder.nextUpgradePoll=now+data.doorUpgradeDelaySeconds*1000;}}
  else{holder.nextPoll=Infinity;if(live.farmFree)holder.refusedAt=now+data.farm.refuseSeconds*1000;}
  return;
 }
 if(now-holder.lastProgress>=policy.wait.progressMilliseconds){holder.lastProgress=now;say(agent,waitingLine(agent.language,request,sampled,evaluation,grants.size,Math.round((now-arrived)/1000)),"waiting",{who:"agent" in holder?holder.label:"farm",reason:mine.reason,behind:behind(evaluation,request)});}
 holder.nextPoll=now+jitter();
}

/** 🔼️ One poll of the upgrade loop of a quiet grant (`upgradeHostGrantV1`): the run is paused until its full request fits; mirrors the publish and the EN/DE lines. */
function upgradePoll(run:Run):void{
 const grant=grants.get(run.id),request=live.upgradeAllJobs?{...run.growth,minimumJobs:run.growth.preferredJobs}:run.growth,record=hc.requireHostCapacityV1("Waiter",{request,arrivedAtMs:grant.arrivedAtMs??grant.admittedAtMs,pid:run.pid,upgrade:true,session:run.agent.session,weight:1});
 if(!waiters.some(row=>row.request.id===request.id&&row.upgrade===true)){waiters.push(record);if(run.pausing)run.waiting="upgrade";run.waitSince=now;}
 const sampled=sample(),evaluation=hc.evaluateHostCapacityV1(policy,sampled,[...grants.values()],waiters,now),mine=evaluation.decisions.find((row:any)=>row.id===request.id);
 if(mine.verdict==="admit"){
  const{quiet:_,...rest}=grant;
  grants.set(run.id,hc.requireHostCapacityV1("Grant",{...rest,jobs:mine.jobs,cpu:mine.jobs*request.cpuPerJob,reservedMegabytes:mine.reservedMegabytes,resumedAtMs:now,upgradedAtMs:now,...request.rampMilliseconds===undefined?{}:{rampMilliseconds:request.rampMilliseconds},...request.rampSlot===undefined?{}:{rampSlot:request.rampSlot}}));
  waiters.splice(waiters.findIndex(row=>row.request.id===request.id&&row.upgrade===true),1);run.executed=true;
  const wanted=Math.min(request.fixedMegabytes+(live.reducedLine?request.preferredJobs:request.minimumJobs)*request.perJobMegabytes,capacity),seconds=Math.round((now-run.waitSince)/1000);
  say(run.agent,live.reducedLine&&mine.reason!=="override"&&mine.jobs<request.preferredJobs?hc.formatHostCapacityV1(run.agent.language,"upgradedReduced",{operation:"run",from:grant.reservedMegabytes,to:mine.reservedMegabytes,jobs:mine.jobs,preferred:request.preferredJobs,seconds,available:sampled.availableMegabytes,floor:evaluation.floorMegabytes}):mine.reservedMegabytes<wanted?hc.formatHostCapacityV1(run.agent.language,"upgradedPartly",{operation:"run",from:grant.reservedMegabytes,to:mine.reservedMegabytes,wanted,seconds,available:sampled.availableMegabytes,floor:evaluation.floorMegabytes,jobs:mine.jobs,preferred:request.preferredJobs}):hc.formatHostCapacityV1(run.agent.language,"upgraded",{operation:"run",from:grant.reservedMegabytes,to:mine.reservedMegabytes,seconds,jobs:mine.jobs,available:sampled.availableMegabytes,floor:evaluation.floorMegabytes}),"upgraded");
  events.push({t:now-START,agent:run.agent.name,run:run.label,event:"upgrade",jobs:mine.jobs,reserved:mine.reservedMegabytes,reason:mine.reason,waited:seconds});
  if(run.pausing)run.upgradeWait+=now-run.waitSince;else run.growWait+=now-run.waitSince;run.waiting=undefined;run.nextUpgradePoll=Infinity;return;
 }
 if(now-run.lastProgress>=policy.wait.progressMilliseconds){
  run.lastProgress=now;const position=evaluation.decisions.findIndex((row:any)=>row.id===request.id),spare=evaluation.headroomMegabytes-evaluation.decisions.slice(0,position).reduce((sum:number,row:any)=>sum+row.reservedMegabytes,0);
  say(run.agent,hc.formatHostCapacityV1(run.agent.language,"upgrading",{operation:"run",seconds:Math.round((now-run.waitSince)/1000),from:grant.reservedMegabytes,to:Math.min(request.fixedMegabytes+request.minimumJobs*request.perJobMegabytes,capacity),reason:hc.describeHostReasonV1(run.agent.language,mine.reason),spare,floor:evaluation.floorMegabytes,running:live.behind?grants.size-1:grants.size,ahead:position,behind:live.behind?hc.describeHostBlockersV1(run.agent.language,hc.hostBlockersV1(policy,[...grants.values()],waiters,evaluation,request.id,now)):""}),"upgrading",{who:run.label,reason:mine.reason,behind:behind(evaluation,request)});
 }
 run.nextUpgradePoll=now+jitter();
}

/** 🌱️ One grow attempt of a reduced grant through the REAL `growHostGrantV1` over an in-memory ledger. */
async function growAttempt(run:Run):Promise<void>{
 const sampled=sample(),ledger={takeLock:()=>true,releaseLock:()=>{},holdsLock:()=>true,readGrants:()=>[...grants.values()],readWaiters:()=>waiters,queuePath:()=>"",sample:async()=>sampled};
 const before=grants.get(run.id).jobs,jobs=await hc.growHostGrantV1({ledger,directory:"",policy,full:run.growth,state:()=>grants.get(run.id),publish:(next:any)=>{grants.set(run.id,next);},now:()=>now,pid:run.pid,language:run.agent.language,write:(line:string)=>say(run.agent,line,"grown"),settled:()=>run.settled,settle:(next:any)=>{run.settled=next;}});
 if(jobs!==undefined)events.push({t:now-START,agent:run.agent.name,run:run.label,event:"grow",from:before,jobs,reserved:grants.get(run.id).reservedMegabytes});
}

/** 💓️ The heartbeat of a holder: what its process tree holds now (the farm grant observes the graph child it runs in, as `observeTreeMegabytesV1(process.pid)` does). */
function beat(holder:Run|Farm):void{
 const grant=grants.get(holder.id);if(!grant)return;
 const observed="agent" in holder?treeOf(holder).fp:live.elasticFarm?0:graphChildOf(holder.owner);
 grants.set(holder.id,{...grant,observedMegabytes:observed});
 if("agent" in holder){holder.peak=Math.max(holder.peak,observed);holder.marks.push([now-holder.admittedAt!,observed]);if(live.outgrown&&grant.quiet===true&&observed>grant.reservedMegabytes&&!holder.asked){holder.asked=true;holder.nextUpgradePoll=now;}}
}

/** 🏁️ Ends a run: learns like the door's `learn` (real record functions), gives the grant and the slot back. */
function finish(run:Run):void{
 run.endedAt=now;const grant=grants.get(run.id),jobs=grant.jobs,active=now-run.admittedAt!,near=run.marks.find(([,megabytes])=>megabytes*10000>=run.peak*9500);
 const defaultMegabytes=Math.max(1,run.base.fixedMegabytes+jobs*run.base.perJobMegabytes),hostRun={jobs,peakMegabytes:run.peak,activeMilliseconds:active,peakAtMilliseconds:Math.min(near?.[0]??active,active),endedAtMs:now};
 if(run.peak>0&&hc.learnableHostRunV1("ok",Math.max(1,Math.floor(run.peak*10000/defaultMegabytes+0.5)),run.estimate)){const next=hc.recordHostPhasesV1(policy,store,run.spec.lineage,"run",defaultMegabytes,{...hostRun,keepPeak:true});store=hc.recordHostForecastV1(policy,next,run.spec.lineage[run.spec.lineage.length-1],{executed:run.executed||run.asked||live.peakExecuted&&run.peak>run.quietSize,...hostRun});}
 grants.delete(run.id);holders.delete(run.id);{const at=waiters.findIndex(row=>row.request.id===run.id&&row.upgrade===true);if(at>=0){waiters.splice(at,1);run.growWait+=now-run.waitSince;}}
 if(run.slot!==undefined){slots[run.slot]=undefined;idleServers.add(run.slot);}
 if(run.farm){grants.delete(run.farm.id);holders.delete(run.farm.id);pending.delete(run.farm.id);const at=waiters.findIndex(row=>row.request.id===run.farm!.id);if(at>=0)waiters.splice(at,1);}
 events.push({t:now-START,agent:run.agent.name,run:run.label,event:"end",wall:Math.round((now-run.arrivedAt)/100)/10});
 records.push({agent:run.agent.name,language:run.agent.language,run:run.label,target:run.spec.target,arrive:run.arrivedAt,admitted:run.admittedAt,ended:now,door:run.queued,upgrade:run.upgradeWait,grow:run.growWait,custody:run.custodyWait,slot:run.slotWait,wall:now-run.arrivedAt,alone:run.spec.segments.reduce((sum,row)=>sum+row.seconds*1000,0),jobs:run.jobsAtStart??grant.jobs,reserved:grant.reservedMegabytes,quiet:run.quiet,fpPeak:run.fpPeak,rssPeak:run.rssPeak,learnedPeak:run.peak});
 run.agent.run=undefined;run.agent.step++;run.agent.readyAt=now;
}

/** ⏩️ Advances a running run by one tick: waits for the builder custody or a Bazel slot at a segment start, asks for the farm, moves through its segments. */
function advance(run:Run):void{
 const segment=run.spec.segments[run.segment]!;
 if(!live.inheritedUpgrade&&run.progress===0&&segment.kind==="refresh"&&grants.get(run.id)?.quiet===true&&!run.pausing){run.pausing=true;run.asked=true;run.waiting="upgrade";run.waitSince=now;run.nextUpgradePoll=now;return;}
 if(run.progress===0&&segment.kind==="refresh"&&grants.get(run.id)?.quiet!==true)run.asked=true;
 if(run.progress===0&&segment.kind==="refresh"&&custody!==run){
  if(custody===undefined&&(custodyQueue.length===0||custodyQueue[0]===run)){custodyQueue.shift();custody=run;if(run.waiting==="custody"){run.custodyWait+=now-run.waitSince;run.waiting=undefined;}}
  else{if(run.waiting!=="custody"){run.waiting="custody";run.waitSince=now;custodyQueue.push(run);say(run.agent,`graph: a peer builds the graph; this run waits as its front door alone`,"graph-wait");}return;}
 }
 if(run.progress===0&&segment.kind==="bazel"&&run.slot===undefined){
  const sampled=sample(),idle=sampled.availableMegabytes-floor-[...grants.values()].reduce((sum,grant)=>sum+Math.max(0,grant.reservedMegabytes-(grant.observedMegabytes??0)),0);
  let at=-1;for(let index=0;index<Math.max(slotsPlanned,slots.length);index++)if(slots[index]===undefined&&(index<slotsPlanned||idleServers.has(index))){at=index;break;}
  if(at<0&&idle>=4096&&slots.length<12)at=Math.max(slots.length,slotsPlanned);
  if(at<0){if(run.waiting!=="slot"){run.waiting="slot";run.waitSince=now;}return;}
  if(run.waiting==="slot"){run.slotWait+=now-run.waitSince;run.waiting=undefined;}
  slots[at]=run;run.slot=at;idleServers.delete(at);run.jobsAtStart=grants.get(run.id).jobs;
 }
 if(segment.kind==="refresh"&&segment.farmAfter!==undefined&&!run.farmAsked&&run.progress>=segment.farmAfter*1000){
  run.farmAsked=true;const sized=hc.sizeHostRequestV1(policy,{id:id(),operation:"exec",preferredJobs:data.farm.preferredJobs,kinds:["build"]}),request=live.elasticFarm?{...sized,elastic:true,...live.labels?{label:"graph-farm"}:{}}:sized;
  const farm:Farm={id:request.id,pid:run.pid+100000,request,arrivedAt:now,nextPoll:now,lastProgress:-Infinity,owner:run};run.farm=farm;pending.set(farm.id,farm);
  waiters.push(hc.requireHostCapacityV1("Waiter",{request,arrivedAtMs:now,pid:farm.pid,...live.elasticFarm?{session:run.agent.session,weight:1}:{}}));
  events.push({t:now-START,agent:run.agent.name,run:"farm",event:"arrive",reserved:request.fixedMegabytes+request.perJobMegabytes});
 }
 if(!live.inheritedUpgrade&&live.birthUpgrade&&segment.kind==="bazel"&&segment.executes===true&&run.progress===0){if(grants.get(run.id)?.quiet===true&&!run.pausing){run.pausing=true;run.asked=true;run.waiting="upgrade";run.waitSince=now;run.nextUpgradePoll=now;return;}run.asked=true;}
 run.progress+=TICK*widthOf(run,segment);
 if(run.progress>=segment.seconds*1000){
  if(segment.kind==="refresh"){custody=undefined;if(run.farm){grants.delete(run.farm.id);holders.delete(run.farm.id);pending.delete(run.farm.id);const at=waiters.findIndex(row=>row.request.id===run.farm!.id);if(at>=0)waiters.splice(at,1);events.push({t:now-START,agent:run.agent.name,run:"farm",event:"release"});}}
  run.segment++;run.progress=0;
  if(run.segment>=run.spec.segments.length)finish(run);
 }
}

let maxExcess={t:0,excess:0},peakCommitted={t:0,committed:0,real:0},peakReal={t:0,committed:0,real:0},minFree={t:0,free:Infinity},maxRss={t:0,rss:0};
/** 📊️ Samples the ledger and the real trees once per second for the verdict. */
function observe():void{
 const sampled=sample(),evaluation=hc.evaluateHostCapacityV1(policy,sampled,[...grants.values()],[],now),committed=[...grants.values()].reduce((sum,grant)=>{const ramping=now-(grant.resumedAtMs??grant.admittedAtMs)<(grant.rampMilliseconds??policy.memory.rampMilliseconds);return sum+(ramping?Math.max(grant.reservedMegabytes,grant.observedMegabytes??0):grant.observedMegabytes??grant.reservedMegabytes);},0),reserved=[...grants.values()].reduce((sum,grant)=>sum+grant.reservedMegabytes,0);
 const trees=ours().reduce((sum,run)=>sum+treeOf(run).fp,0),servers=idleServers.size*data.bazel.idleServerMegabytes,real=trees+servers,rss=ours().reduce((sum,run)=>sum+treeOf(run).rss,0),free=host.realMegabytes-host.foreignMegabytes-real;
 for(const run of ours()){const tree=treeOf(run);run.fpPeak=Math.max(run.fpPeak,tree.fp);run.rssPeak=Math.max(run.rssPeak,tree.rss);}
 if(trees-committed>maxExcess.excess)maxExcess={t:now-START,excess:trees-committed};
 if(committed>peakCommitted.committed)peakCommitted={t:now-START,committed,real};
 if(real>peakReal.real)peakReal={t:now-START,committed,real};
 if(free<minFree.free)minFree={t:now-START,free};
 if(rss>maxRss.rss)maxRss={t:now-START,rss};
 timeline.push({t:Math.round((now-START)/1000),available:sampled.availableMegabytes,headroom:evaluation.headroomMegabytes,reserved,committed,trees,servers,rss,free,grants:[...grants.values()].map(grant=>`${labelOf(grant.id)}:${grant.jobs}j/${grant.reservedMegabytes}${grant.quiet?"q":""}`),waiting:waiters.map(row=>`${labelOf(row.request.id)}${row.upgrade?"^":""}`)});
}

const lend=has("lend");
for(let guard=0;guard<36000*10;guard++){
 for(const agent of agents)if(!agent.done&&agent.run===undefined&&now>=agent.readyAt){if(agent.step>=agent.steps.length)agent.done=true;else startRun(agent);}
 if(prime.done&&T0===Infinity){T0=now+scenario.gapSeconds*1000;for(const agent of agents.slice(1))agent.readyAt=T0+agent.offset!;}
 if(agents.every(agent=>agent.done))break;
 for(const holder of [...pending.values()])if(now>=holder.nextPoll)doorPoll(holder);
 for(const run of ours()){
  if(now>=run.nextBeat){beat(run);run.nextBeat=now+policy.wait.heartbeatMilliseconds;}
  if(now>=run.nextUpgradePoll)upgradePoll(run);
  if(run.waiting==="upgrade")continue;
  if(run.farm&&holders.has(run.farm.id)&&(now-run.farm.admittedAt!)%policy.wait.heartbeatMilliseconds<TICK)beat(run.farm);
  if(run.farm?.refusedAt!==undefined&&now>=run.farm.refusedAt&&holders.has(run.farm.id)){grants.delete(run.farm.id);holders.delete(run.farm.id);events.push({t:now-START,agent:run.agent.name,run:"farm",event:"refused"});}
  if(now>=run.nextGrow){run.nextGrow=now+policy.wait.heartbeatMilliseconds;const grant=grants.get(run.id);if(run.growth.perJobMegabytes>0&&grant&&grant.quiet!==true&&grant.lentSinceMs===undefined&&grant.jobs<run.growth.preferredJobs)await growAttempt(run);}
  if(lend&&(run.waiting==="custody"||run.waiting==="slot")&&now-run.waitSince>=policy.wait.lendAfterMilliseconds){const grant=grants.get(run.id);if(grant.lentSinceMs===undefined)grants.set(run.id,{...grant,lentSinceMs:now});}
  else if(lend){const grant=grants.get(run.id);if(grant?.lentSinceMs!==undefined){const{lentSinceMs:_,...rest}=grant;grants.set(run.id,{...rest,resumedAtMs:now});}}
  if(holders.has(run.id))advance(run);
 }
 if((now-START)%1000===0)observe();
 now+=TICK;
}

const verdict:any={T0:T0-START,scenario:scenarioName,host:hostName,declared:{totalMegabytes:total,cores:host.declaredCores,floorMegabytes:floor,capacityMegabytes:capacity,bazelSlotsPlanned:slotsPlanned},makespanSeconds:Math.round((now-T0)/1000),peakCommitted,peakReal,minFree,maxRss,events,lines,timeline,store:has("keep-store")?store:undefined};
const scenarioRecords=records.filter(row=>row.agent!=="P"),scenarioLines=lines.filter(row=>row.agent!=="P"&&row.agent!=="-"),sec=(ms:number)=>Math.round(ms/100)/10;
const waitRows=scenarioLines.filter(row=>row.kind==="waiting"||row.kind==="upgrading"),named=waitRows.filter(row=>/; (behind|hinter) \S/u.test(row.line));
const starved=waitRows.filter(row=>row.behind.length===0&&Number(/(\d+)\s?s\b/u.exec(row.line)?.[1]??0)>=60);
const ratio=scenarioRecords.filter(rec=>(rec.door+rec.upgrade)>=120000&&(rec.door+rec.upgrade)>2*rec.alone);
const wrong=scenarioLines.filter(row=>{const agent=agents.find(a=>a.name===row.agent)!;return agent.language==="de"?/\b(waiting|admitted|reservation grown|sized at)\b/u.test(row.line):/\b(wartet|zugelassen|vergroessert|bemessen)\b/u.test(row.line);});
const summary={T0:Math.round((T0-START)/1000),makespan:sec(now-T0),runs:scenarioRecords.map(rec=>({agent:rec.agent,lang:rec.language,run:rec.run,start:sec(rec.arrive-T0),door:sec(rec.door),upgrade:sec(rec.upgrade),grow:sec(rec.grow),custody:sec(rec.custody),slot:sec(rec.slot),wall:sec(rec.wall),alone:sec(rec.alone),jobs:rec.jobs,reserved:rec.reserved,quiet:rec.quiet,fp:rec.fpPeak,rss:rec.rssPeak})),order:events.filter(row=>row.event==="admit"&&row.agent!=="P"&&row.t>=T0-START).map(row=>`${row.agent}:${row.run}@${sec(row.t-(T0-START))}s`),upgrades:events.filter(row=>row.event==="upgrade"&&row.agent!=="P"&&row.t>=T0-START).map(row=>`${row.agent}:${row.run}@${sec(row.t-(T0-START))}s+${row.waited}s ${row.reason} ${row.jobs}j/${row.reserved}`),grows:events.filter(row=>row.event==="grow"&&row.t>=T0-START).length,farm:events.filter(row=>row.run==="farm").map(row=>`${row.agent}:${row.event}@${sec(row.t-(T0-START))}s${row.waited!==undefined?"+"+row.waited+"s":""}${row.jobs!==undefined?" "+row.jobs+"j/"+row.reserved:""}`),waitRows:waitRows.length,waitRowsNamingBlocker:named.length,waitReasons:Object.fromEntries([...new Set(waitRows.map(row=>row.reason))].map(reason=>[reason,waitRows.filter(row=>row.reason===reason).length])),starvedRows:starved.length,queueOver2x:ratio.map(rec=>`${rec.agent}:${rec.run}`),wrongLanguageRows:wrong.length,languages:[...new Set(scenarioLines.map(row=>agents.find(a=>a.name===row.agent)!.language))],peakCommitted:{t:sec(peakCommitted.t-(T0-START)),committed:peakCommitted.committed,real:peakCommitted.real},peakReal:{t:sec(peakReal.t-(T0-START)),committed:peakReal.committed,real:peakReal.real},minFree:{t:sec(minFree.t-(T0-START)),free:minFree.free,freePercent:Math.round(minFree.free*1000/host.realMegabytes)/10},maxRss,maxExcess:{t:sec(maxExcess.t-(T0-START)),excess:maxExcess.excess},floorMegabytes:floor};
verdict.summary=summary;
const out=flag("json");if(out)writeFileSync(out,JSON.stringify(verdict,null,1));
console.log(JSON.stringify({scenario:scenarioName,host:hostName,live,...summary},null,has("pretty")?1:0));
