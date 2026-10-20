import {createHash} from "node:crypto";
import schema from "./@@SCHEMA@@/@@JSON@@" with {type:"json"};
import {incrementalMutateV1,type IncrementalEditOpV1,type IncrementalLanguageV1,type IncrementalToolV1} from "../@@TS@@";
import {validateJsonSchemaSubset} from "@@SEVEN@@/@@MODULES@@/@@SCHEMA@@/@@VALIDATOR@@/@@PACKAGES@@/@@TYPESCRIPT@@/@@TS@@";

/** 🔤 The source languages a concurrent edit can be rendered for. */
export type ConcurrentLanguageV1="ts"|"rs"|"go"|"py"|"text";
/** 🌍 A text in every language the harness speaks. */
export type ConcurrentTextV1=Readonly<{en:string;de:string}>;
/** 🪢 One step of a simulated agent: edit a file, revert its edits, run a task, run any repository command, wait for the other agents at a barrier or pause. */
export type ConcurrentStepV1=Readonly<
 {kind:"edit";file:string;op:IncrementalEditOpV1;language:ConcurrentLanguageV1}|
 {kind:"revert"}|
 {kind:"run";target:string;outputs?:readonly string[]}|
 {kind:"repo";args:readonly string[];outputs?:readonly string[];stdout?:boolean}|
 {kind:"barrier";id:string}|
 {kind:"pause";milliseconds:number}>;
/** 🎼 A simulated agent: its project group decides which other agents are related to it. */
export type ConcurrentSessionV1=Readonly<{id:string;area:string;group:string;delayMilliseconds?:number;steps:readonly ConcurrentStepV1[]}>;
/** 🎻 Agents working in one checkout copy at the same time, with the setup that precedes them and the pass criteria that judge them. */
export type ConcurrentWorkloadV1=Readonly<{id:string;title:ConcurrentTextV1;summary:ConcurrentTextV1;criteria:string;tools?:readonly IncrementalToolV1[];setup:readonly ConcurrentStepV1[];sessions:readonly ConcurrentSessionV1[]}>;
/** 🔒 A family of locks, recognised by the resource names its waits carry. */
export type ConcurrentLeaseClassV1=Readonly<{id:string;label:ConcurrentTextV1;scope:"workspace"|"project"|"host";patterns:readonly string[]}>;
type RuleHeadV1={id:string;title:ConcurrentTextV1;owners:readonly string[]};
/** 📐 One pass criterion, declared in the schema: it names the executors it accepts. */
export type ConcurrentRuleV1=Readonly<RuleHeadV1&(
 {kind:"no-failure"}|
 {kind:"serial-equal";compare:readonly ("exit"|"outputs"|"stdout")[]}|
 {kind:"max-wait";classes:readonly string[];relation:"related"|"unrelated"|"any";maximumMilliseconds:number}|
 {kind:"executes-once";sessions:readonly string[];excludeUncached:boolean}|
 {kind:"max-executed";sessions:readonly string[];maximum:number;excludeUncached:boolean})>;
/** 🗃 A named set of pass criteria. */
export type ConcurrentCriteriaV1=Readonly<{id:string;title:ConcurrentTextV1;rules:readonly ConcurrentRuleV1[]}>;
/** 🗄 The authored data: texts, lock classes, criteria and workloads. */
export type ConcurrentWorkloadSetV1=Readonly<{version:1;labels:Readonly<Record<string,ConcurrentTextV1>>;classes:readonly ConcurrentLeaseClassV1[];criteria:readonly ConcurrentCriteriaV1[];workloads:readonly ConcurrentWorkloadV1[]}>;
/** ⏳ One episode in which a session waited for a lock held by someone else; a wait is precise when its end was observed (lease trace, Bazel and Cargo wait lines) and a lower bound when it was derived from the CLI progress rows. */
export type ConcurrentWaitV1=Readonly<{class:string;resource:string;mode:"shared"|"exclusive";source:"lease"|"trace"|"bazel"|"cargo";precise:boolean;startMs:number;endMs:number;waitMs:number;holderPid:number|null;position:number|null;relation?:"related"|"unrelated"|"external";holders?:readonly string[]}>;
/** 👀 Which tasks one run executed, restored from a cache or never needed. */
export type ConcurrentObservationV1=Readonly<{executed:readonly string[];restored:readonly string[];cached:readonly string[]}>;
/** 🎚 How reliably the tasks of a run were attributed to it. */
export type ConcurrentObservedV1="exact"|"window"|"ambiguous"|"none";
/** 📇 Everything one command of one session did. */
export type ConcurrentStepResultV1=Readonly<{index:number;kind:"run"|"repo";command:readonly string[];startMs:number;endMs:number;exit:number|null;waits:readonly ConcurrentWaitV1[];observation:ConcurrentObservationV1|null;observed:ConcurrentObservedV1;outputs:string|null;stdout:string|null}>;
/** 🕰 The timeline of one session. */
export type ConcurrentSessionResultV1=Readonly<{id:string;group:string;startMs:number;endMs:number;steps:readonly ConcurrentStepResultV1[]}>;
/** 📈 The timelines of a concurrent run and of its serial reference. */
export type ConcurrentMeasurementV1=Readonly<{workload:string;tool:IncrementalToolV1;uncached:readonly string[];sessions:readonly ConcurrentSessionResultV1[];serial:readonly ConcurrentSessionResultV1[]|null}>;
/** 🏛 The judgement of one criterion. */
export type ConcurrentRuleResultV1=Readonly<{rule:string;kind:ConcurrentRuleV1["kind"];owners:readonly string[];status:"pass"|"fail"|"skip";value:number|null;limit:number|null;detail:readonly string[]}>;
/** 🏆 One judged workload run with one tool. */
export type ConcurrentRunV1=Readonly<{workload:string;tool:IncrementalToolV1;criteria:string;measurement:ConcurrentMeasurementV1;rules:readonly ConcurrentRuleResultV1[]}>;
/** 🎖 The verdicts counted per accepted executor. */
export type ConcurrentOwnerTallyV1=Readonly<{owner:string;passed:number;failed:number;skipped:number}>;
/** 🗳 All judged runs with their tally. */
export type ConcurrentReportV1=Readonly<{version:1;runs:readonly ConcurrentRunV1[];passed:number;failed:number;skipped:number;owners:readonly ConcurrentOwnerTallyV1[]}>;
/** 👥 A session as the attribution of a wait sees it: its group and the time windows in which it ran a command. */
export type ConcurrentActivityV1=Readonly<{id:string;group:string;intervals:readonly (readonly [number,number])[]}>;

const policy=schema.definitions.Policy.const,order=(left:string,right:string)=>left<right?-1:left>right?1:0,sorted=(values:Iterable<string>)=>[...new Set(values)].sort(order);
const closed=(name:keyof typeof schema.definitions,value:unknown,message:string):void=>{if(validateJsonSchemaSubset(schema.definitions[name],value,schema).length)throw Error(message);};

/** 🧿 Names the lock class of a resource: the first class with a matching pattern wins, a resource nobody claims is `other`. */
export function concurrentClassifyV1(classes:readonly ConcurrentLeaseClassV1[],resource:string):string{
 for(const row of classes)if(row.patterns.some(pattern=>new RegExp(pattern,"u").test(resource)))return row.id;
 return "other";
}

/** 🪤 Returns a stateful filter over the stderr lines of one command that keeps only what the wait derivation needs: lock rows of the CLI, Bazel and Cargo wait lines and the line after a wait line, and drops the typed progress rows of other publishers. */
export function concurrentLineKeeperV1():(line:string)=>boolean{
 const script=new RegExp(policy.scriptRow,"u"),bazel=new RegExp(policy.bazelWait,"u"),cargo=new RegExp(policy.cargoWait,"u"),ignored=policy.ignoredPrefixes.map(prefix=>`"resource":"${prefix}`);
 let after=false;
 return line=>{
  const wait=bazel.test(line)||cargo.test(line),row=!wait&&script.test(line)&&line.includes('"activity"')&&!ignored.some(prefix=>line.includes(prefix)),keep=wait||after||row;
  after=wait;
  return keep;
 };
}

type OpenV1={key:string;resource:string;mode:"shared"|"exclusive";source:"lease"|"bazel"|"cargo";startAt:number;firstAt:number;firstElapsed:number;lastAt:number;lastElapsed:number;holderPid:number|null;position:number|null};
const activityOf=(text:string,script:RegExp):{resource:string;mode:"shared"|"exclusive";elapsedMs:number}|null=>{
 const match=script.exec(text);
 if(!match)return null;
 let row:{activity?:{resource?:unknown;mode?:unknown;elapsedMs?:unknown}};
 try{row=JSON.parse(match[1]!);}catch{return null;}
 const activity=row?.activity;
 if(!activity||typeof activity.resource!=="string"||activity.resource===""||(activity.mode!=="shared"&&activity.mode!=="exclusive")||typeof activity.elapsedMs!=="number"||!Number.isFinite(activity.elapsedMs)||activity.elapsedMs<0)return null;
 return {resource:activity.resource,mode:activity.mode,elapsedMs:activity.elapsedMs};
};

/** ⌛ Derives the lock waits of one command from its stderr lines. The CLI publishes a row `[script] {"activity":{resource,mode,elapsedMs}}` about every second while it waits for a lease, so an episode lasts as long as the rows keep arriving with a growing elapsed time and its wait is the last elapsed time on the lease's own clock (a lower bound within the publication interval); a `lease-queue:` row names the holder; Bazel's `Another command (pid=N) is running` and Cargo's `Blocking waiting for file lock` wait until the next line of any other kind or the end of the command. Typed progress rows of other publishers are ignored. */
export function concurrentWaitsV1(lines:readonly Readonly<{atMs:number;line:string}>[],endMs:number,classes:readonly ConcurrentLeaseClassV1[]):ConcurrentWaitV1[]{
 const script=new RegExp(policy.scriptRow,"u"),bazel=new RegExp(policy.bazelWait,"u"),continuation=new RegExp(policy.bazelContinuation,"u"),cargo=new RegExp(policy.cargoWait,"u"),open=new Map<string,OpenV1>(),found:ConcurrentWaitV1[]=[];
 const close=(episode:OpenV1):void=>{
  open.delete(episode.key);
  if(episode.source==="lease")found.push(Object.freeze({class:concurrentClassifyV1(classes,episode.resource),resource:episode.resource,mode:episode.mode,source:"lease" as const,precise:false,startMs:episode.firstAt-episode.firstElapsed,endMs:episode.lastAt,waitMs:episode.lastElapsed,holderPid:episode.holderPid,position:episode.position}));
  else found.push(Object.freeze({class:concurrentClassifyV1(classes,episode.resource),resource:episode.resource,mode:episode.mode,source:episode.source,precise:true,startMs:episode.startAt,endMs:episode.lastAt,waitMs:episode.lastAt-episode.startAt,holderPid:episode.holderPid,position:null}));
 };
 const closeLines=(at:number):void=>{for(const episode of [...open.values()])if(episode.source!=="lease"){episode.lastAt=Math.max(at,episode.startAt);close(episode);}};
 const begin=(episode:Omit<OpenV1,"key">):OpenV1=>{const row={...episode,key:`${episode.source}\0${episode.resource}\0${episode.mode}`};open.set(row.key,row);return row;};
 for(const{atMs,line}of [...lines].sort((left,right)=>left.atMs-right.atMs)){
  const bazelMatch=bazel.exec(line),cargoMatch=cargo.exec(line);
  if(bazelMatch){
   const existing=open.get(`bazel\0${policy.bazelResource}\0exclusive`);
   if(!existing)begin({resource:policy.bazelResource,mode:"exclusive",source:"bazel",startAt:atMs,firstAt:atMs,firstElapsed:0,lastAt:atMs,lastElapsed:0,holderPid:Number(bazelMatch[1]),position:null});
   continue;
  }
  if(cargoMatch){
   const resource=policy.cargoResourcePrefix+cargoMatch[1]!;
   for(const episode of [...open.values()])if(episode.source==="cargo"&&episode.resource!==resource){episode.lastAt=Math.max(atMs,episode.startAt);close(episode);}
   if(!open.has(`cargo\0${resource}\0exclusive`))begin({resource,mode:"exclusive",source:"cargo",startAt:atMs,firstAt:atMs,firstElapsed:0,lastAt:atMs,lastElapsed:0,holderPid:null,position:null});
   continue;
  }
  if(continuation.test(line)&&[...open.values()].some(episode=>episode.source==="bazel"))continue;
  closeLines(atMs);
  const row=activityOf(line,script);
  if(!row)continue;
  if(row.resource.startsWith(policy.queuePrefix)){
   let info:{resource?:unknown;holderPid?:unknown;position?:unknown};
   try{info=JSON.parse(row.resource.slice(policy.queuePrefix.length));}catch{continue;}
   if(typeof info?.resource!=="string"||!Number.isInteger(info.holderPid)||!Number.isInteger(info.position))continue;
   const key=`lease\0${info.resource}\0${row.mode}`,current=open.get(key);
   if(current&&atMs-current.lastAt<=policy.episodeGapMilliseconds){current.holderPid=info.holderPid as number;current.position=info.position as number;}
   else{if(current)close(current);begin({resource:info.resource,mode:row.mode,source:"lease",startAt:atMs,firstAt:atMs,firstElapsed:0,lastAt:atMs,lastElapsed:0,holderPid:info.holderPid as number,position:info.position as number});}
   continue;
  }
  if(policy.ignoredPrefixes.some(prefix=>row.resource.startsWith(prefix)))continue;
  const key=`lease\0${row.resource}\0${row.mode}`,current=open.get(key);
  if(current&&row.elapsedMs>=current.lastElapsed&&atMs-current.lastAt<=policy.episodeGapMilliseconds){current.lastAt=atMs;current.lastElapsed=row.elapsedMs;}
  else{if(current)close(current);begin({resource:row.resource,mode:row.mode,source:"lease",startAt:atMs,firstAt:atMs,firstElapsed:row.elapsedMs,lastAt:atMs,lastElapsed:row.elapsedMs,holderPid:null,position:null});}
 }
 closeLines(endMs);
 for(const episode of [...open.values()])close(episode);
 return found.sort((left,right)=>left.startMs-right.startMs||order(left.resource,right.resource)||order(left.source,right.source));
}

/** 📼 Reads the lease trace a command appends to the file named by `SEMIO_LEASE_TRACE`: one JSON row per lease acquisition that waited, with the epoch milliseconds at which it started waiting and at which it acquired. Such waits are precise; rows of one process for one resource and mode that overlap (a queue wait and the acquisition inside it) coalesce into one wait; times are shifted by `origin` onto the time base of the run, malformed rows are ignored. */
export function concurrentTraceWaitsV1(text:string,origin:number,classes:readonly ConcurrentLeaseClassV1[]):ConcurrentWaitV1[]{
 type Row={resource:string;mode:"shared"|"exclusive";startedMs:number;acquiredMs:number;pid:number;holderPid?:number};
 const rows:Row[]=[];
 for(const line of text.split("\n")){
  if(!line.trim())continue;
  let row:unknown;
  try{row=JSON.parse(line);}catch{continue;}
  if(validateJsonSchemaSubset(schema.definitions.TraceRow,row,schema).length)continue;
  const value=row as Row;
  if(value.acquiredMs>=value.startedMs)rows.push({...value});
 }
 const merged:Row[]=[];
 for(const row of rows.sort((left,right)=>order(left.resource,right.resource)||order(left.mode,right.mode)||left.pid-right.pid||left.startedMs-right.startedMs)){
  const last=merged.at(-1);
  if(last&&last.resource===row.resource&&last.mode===row.mode&&last.pid===row.pid&&row.startedMs<=last.acquiredMs){last.acquiredMs=Math.max(last.acquiredMs,row.acquiredMs);if(last.holderPid===undefined&&row.holderPid!==undefined)last.holderPid=row.holderPid;}
  else merged.push(row);
 }
 return merged.map(value=>Object.freeze({class:concurrentClassifyV1(classes,value.resource),resource:value.resource,mode:value.mode,source:"trace" as const,precise:true,startMs:value.startedMs-origin,endMs:value.acquiredMs-origin,waitMs:value.acquiredMs-value.startedMs,holderPid:value.holderPid??null,position:null})).sort((left,right)=>left.startMs-right.startMs||order(left.resource,right.resource));
}

/** 🧶 Merges the waits derived from progress rows with the traced waits: a traced wait replaces every derived lease wait of the same resource and mode that overlaps it in time, because the trace knows when the wait ended. */
export function concurrentMergeWaitsV1(derived:readonly ConcurrentWaitV1[],traced:readonly ConcurrentWaitV1[]):ConcurrentWaitV1[]{
 const kept=derived.filter(wait=>wait.source!=="lease"||!traced.some(other=>other.resource===wait.resource&&other.mode===wait.mode&&other.startMs<=wait.endMs&&other.endMs>=wait.startMs));
 return [...kept,...traced].sort((left,right)=>left.startMs-right.startMs||order(left.resource,right.resource)||order(left.source,right.source));
}

/** 🧲 Attributes a wait to the sessions that ran a command while it lasted: waiting behind a session of another group is `unrelated`, behind a session of the same group `related`, and behind nobody of the harness `external`. */
export function concurrentRelationV1(wait:Readonly<{startMs:number;endMs:number}>,self:Readonly<{id:string;group:string}>,others:readonly ConcurrentActivityV1[]):{relation:"related"|"unrelated"|"external";holders:string[]}{
 const holders=others.filter(other=>other.id!==self.id&&other.intervals.some(([start,end])=>start<=wait.endMs&&end>=wait.startMs));
 const relation=!holders.length?"external":holders.every(other=>other.group!==self.group)?"unrelated":"related";
 return {relation,holders:sorted(holders.map(other=>other.id))};
}

/** 🪪 Annotates every wait of every session with its relation and holders, from the command windows of all sessions on one time base. */
export function concurrentAttributeV1(sessions:readonly ConcurrentSessionResultV1[]):ConcurrentSessionResultV1[]{
 const activity:ConcurrentActivityV1[]=sessions.map(session=>({id:session.id,group:session.group,intervals:session.steps.map(step=>[step.startMs,step.endMs] as const)}));
 return sessions.map(session=>Object.freeze({...session,steps:session.steps.map(step=>Object.freeze({...step,waits:step.waits.map(wait=>Object.freeze({...wait,...concurrentRelationV1(wait,session,activity)}))}))}));
}

const waitTotal=(session:ConcurrentSessionResultV1):number=>session.steps.reduce((sum,step)=>sum+step.waits.reduce((inner,wait)=>inner+wait.waitMs,0),0);
const usable=(step:ConcurrentStepResultV1):boolean=>step.observation!==null&&(step.observed==="exact"||step.observed==="window");
const chosen=(ids:readonly string[],id:string):boolean=>ids.includes("*")||ids.includes(id);
const verdict=(rule:ConcurrentRuleV1,status:"pass"|"fail"|"skip",value:number|null,limit:number|null,detail:readonly string[]):ConcurrentRuleResultV1=>Object.freeze({rule:rule.id,kind:rule.kind,owners:rule.owners,status,value,limit,detail});
const label=(session:ConcurrentSessionResultV1,step:ConcurrentStepResultV1)=>`${session.id}#${step.index}`;

/** 🎲 Judges a measurement with the criteria of its workload. `no-failure`: every step exits 0. `serial-equal`: every step's exit code, output digest and stdout digest equal the serial reference. `max-wait`: no wait of a class and relation exceeds the limit. `executes-once`: no task executes in more than one session. `max-executed`: no run executes more tasks than allowed. Criteria that need task observations are skipped when a run could not be observed and nothing disproves them. */
export function concurrentEvaluateV1(criteria:ConcurrentCriteriaV1,measurement:ConcurrentMeasurementV1):ConcurrentRuleResultV1[]{
 closed("Criteria",criteria,"Closed concurrent criteria required");closed("Measurement",measurement,"Closed concurrent measurement required");
 const uncached=new Set(measurement.uncached),out:ConcurrentRuleResultV1[]=[];
 for(const rule of criteria.rules){
  if(rule.kind==="no-failure"){
   const bad=measurement.sessions.flatMap(session=>session.steps.filter(step=>step.exit!==0).map(step=>`${label(session,step)} exit ${step.exit}`));
   out.push(verdict(rule,bad.length?"fail":"pass",bad.length,0,bad));
  }else if(rule.kind==="serial-equal"){
   if(measurement.serial===null){out.push(verdict(rule,"skip",null,0,[]));continue;}
   const bad:string[]=[];
   for(const session of measurement.sessions){
    const other=measurement.serial.find(row=>row.id===session.id);
    if(!other){bad.push(`${session.id}: no serial session`);continue;}
    for(const step of session.steps){
     const twin=other.steps.find(row=>row.index===step.index);
     if(!twin){bad.push(`${label(session,step)}: no serial step`);continue;}
     for(const key of rule.compare)if(step[key]!==twin[key])bad.push(`${label(session,step)} ${key}: ${step[key]} != ${twin[key]}`);
    }
   }
   out.push(verdict(rule,bad.length?"fail":"pass",bad.length,0,bad));
  }else if(rule.kind==="max-wait"){
   const hits=measurement.sessions.flatMap(session=>session.steps.flatMap(step=>step.waits.filter(wait=>(rule.classes.includes("*")||rule.classes.includes(wait.class))&&(rule.relation==="any"||(rule.relation==="related"?wait.relation==="related":wait.relation==="unrelated"||wait.relation==="external"))).map(wait=>({session,step,wait}))));
   const note=(hit:typeof hits[number])=>`${hit.wait.class} ${hit.wait.resource}`,tail=(hit:typeof hits[number])=>`(${hit.wait.relation??"unknown"}${hit.wait.holders?.length?": "+hit.wait.holders.join(","):""})`;
   const bad=hits.filter(hit=>hit.wait.waitMs>rule.maximumMilliseconds).map(hit=>`${label(hit.session,hit.step)} ${note(hit)} ${hit.wait.waitMs} ms ${tail(hit)}`),open=hits.filter(hit=>!hit.wait.precise&&hit.wait.waitMs<=rule.maximumMilliseconds).map(hit=>`${label(hit.session,hit.step)} ${note(hit)} unproven, at least ${hit.wait.waitMs} ms ${tail(hit)}`);
   out.push(verdict(rule,bad.length?"fail":open.length?"skip":"pass",hits.reduce((most,hit)=>Math.max(most,hit.wait.waitMs),0),rule.maximumMilliseconds,[...bad,...open]));
  }else{
   const runs=measurement.sessions.filter(session=>chosen(rule.sessions,session.id)).flatMap(session=>session.steps.filter(step=>step.kind==="run").map(step=>({session,step}))),seen=runs.filter(row=>usable(row.step)),missing=runs.filter(row=>!usable(row.step)).map(row=>`unobserved ${label(row.session,row.step)}`);
   const skip=!seen.length||missing.length;
   if(rule.kind==="executes-once"){
    const owners=new Map<string,Set<string>>();
    for(const{session,step}of seen)for(const task of step.observation!.executed){if(rule.excludeUncached&&uncached.has(task))continue;owners.set(task,new Set([...owners.get(task)??[],session.id]));}
    const bad=[...owners].filter(([,who])=>who.size>1).map(([task,who])=>`${task}: ${sorted(who).join(", ")}`).sort(order);
    out.push(verdict(rule,bad.length?"fail":skip?"skip":"pass",bad.length,0,[...bad,...missing]));
   }else{
    const counts=seen.map(({session,step})=>({text:label(session,step),count:step.observation!.executed.filter(task=>!(rule.excludeUncached&&uncached.has(task))).length})),bad=counts.filter(row=>row.count>rule.maximum).map(row=>`${row.text}: ${row.count} executed`);
    out.push(verdict(rule,bad.length?"fail":skip?"skip":"pass",counts.reduce((most,row)=>Math.max(most,row.count),0),rule.maximum,[...bad,...missing]));
   }
  }
 }
 return out;
}

/** 🎰 Tallies the rule results of all runs and per accepted executor: a failed rule fails every executor it names, a skipped rule leaves it unproven. */
export function concurrentReportV1(runs:readonly ConcurrentRunV1[]):ConcurrentReportV1{
 let passed=0,failed=0,skipped=0;const owners=new Map<string,{passed:number;failed:number;skipped:number}>();
 for(const run of runs)for(const rule of run.rules){
  const key=rule.status==="pass"?"passed":rule.status==="fail"?"failed":"skipped";
  if(key==="passed")passed++;else if(key==="failed")failed++;else skipped++;
  for(const owner of rule.owners){const tally=owners.get(owner)??{passed:0,failed:0,skipped:0};tally[key]++;owners.set(owner,tally);}
 }
 const report={version:1 as const,runs,passed,failed,skipped,owners:[...owners].sort((left,right)=>order(left[0],right[0])).map(([owner,tally])=>({owner,...tally}))};
 closed("Report",report,"Closed concurrent report required");
 return Object.freeze(report);
}

const number=(nonce:string)=>Number.parseInt(nonce.slice(0,6),36)%1000003;
const goTemplates:Readonly<Partial<Record<IncrementalEditOpV1,(nonce:string)=>string>>>=Object.freeze({
 comment:nonce=>`// [concurrent] ${nonce}\n`,"private-body":nonce=>`var _ = ${number(nonce)}\n`,"public-api":nonce=>`const Concurrent${nonce.toUpperCase()} = ${number(nonce)}\n`
});

/** ✍ Renders one edit of a source text: Go has its own templates, every other language and `whitespace` are the incremental harness's. */
export function concurrentMutateV1(text:string,op:IncrementalEditOpV1,language:ConcurrentLanguageV1,nonce:string):string{
 if(language!=="go"||op==="whitespace")return incrementalMutateV1(text,op,language==="go"?"text":language as IncrementalLanguageV1,nonce);
 if(!new RegExp(policy.nonce,"u").test(nonce))throw Error("Closed concurrent nonce required");
 const render=goTemplates[op];
 if(!render)throw Error(`Concurrent mutation ${op} is unsupported for go`);
 return (text.endsWith("\n")||text===""?text:text+"\n")+render(nonce);
}

/** 🔑 Digests files and directory contents deterministically: sha256 over the sorted parts, each as path, byte length and bytes. */
export function concurrentDigestV1(parts:readonly Readonly<{path:string;bytes:Uint8Array|string}>[]):string{
 const hash=createHash("sha256");
 for(const part of [...parts].sort((left,right)=>order(left.path,right.path))){
  const bytes=typeof part.bytes==="string"?Buffer.from(part.bytes,"utf8"):part.bytes;
  hash.update(`${part.path}\0${bytes.byteLength}\0`);hash.update(bytes);hash.update("\0");
 }
 return hash.digest("hex");
}

/** 🛰 Reads the run record Nx stores after every run as the observation of one session, or refuses to attribute it: concurrent Nx runs share one record, so a record counts only when its own start and end lie inside the command's window. */
export function concurrentNxFitV1(record:unknown,ids:readonly string[],window:Readonly<{startMs:number;endMs:number}>,concurrent:boolean):{observation:ConcurrentObservationV1|null;observed:"exact"|"window"|"ambiguous"}{
 const tasks=(record as {tasks?:unknown})?.tasks,known=new Set(ids);
 if(!Array.isArray(tasks))return {observation:null,observed:"ambiguous"};
 const rows=(tasks as {taskId?:unknown;cacheStatus?:unknown}[]).filter(task=>typeof task?.taskId==="string"&&typeof task.cacheStatus==="string"&&known.has(task.taskId));
 if(!rows.length)return {observation:null,observed:"ambiguous"};
 const stamp=(value:unknown):number=>typeof value==="number"?value:typeof value==="string"?Date.parse(value):Number.NaN,run=(record as {run?:{startTime?:unknown;endTime?:unknown}}).run,start=stamp(run?.startTime),end=stamp(run?.endTime);
 let observed:"exact"|"window"|"ambiguous";
 if(Number.isFinite(start)&&Number.isFinite(end))observed=start>=window.startMs-policy.nxSkewMilliseconds&&end<=window.endMs+policy.nxSkewMilliseconds?(concurrent?"window":"exact"):"ambiguous";
 else observed=concurrent?"ambiguous":"exact";
 if(observed==="ambiguous")return {observation:null,observed};
 const executed=rows.filter(task=>task.cacheStatus==="cache-miss").map(task=>task.taskId as string),restored=rows.filter(task=>task.cacheStatus!=="cache-miss").map(task=>task.taskId as string);
 return {observation:Object.freeze({executed:sorted(executed),restored:sorted(restored),cached:[]}),observed};
}

/** 🚏 Lists every file an unrelated agent edits that another agent's targets declare as an input: such a workload cannot be judged by group, so it is refused before it runs. */
export function concurrentIndependenceIssuesV1(rows:readonly Readonly<{id:string;group:string;edits:readonly string[];touches:readonly string[]}>[]):string[]{
 const found:string[]=[];
 for(const editor of rows)for(const reader of rows){
  if(editor.id===reader.id||editor.group===reader.group)continue;
  for(const file of editor.edits)if(reader.touches.includes(file))found.push(`${editor.id} edits ${file}, an input of the targets of ${reader.id}, but ${editor.group} and ${reader.group} are declared unrelated`);
 }
 return found.sort(order);
}

const stepFiles=(steps:readonly ConcurrentStepV1[]):string[]=>steps.flatMap(step=>step.kind==="edit"?[step.file]:[]);

/** 🛂 Admits the authored workload data and lists every inconsistency: duplicate ids, an unknown criteria set or lock class or session, a barrier mentioned twice in one session, an edit without a template, a file edited by two owners. */
export function concurrentWorkloadIssuesV1(value:unknown):string[]{
 const issues=validateJsonSchemaSubset(schema.definitions.WorkloadSet,value,schema).map(String);
 if(issues.length)return issues;
 const set=value as ConcurrentWorkloadSetV1,found:string[]=[],unique=(kind:string,ids:readonly string[])=>{const seen=new Set<string>();for(const id of ids){if(seen.has(id))found.push(`duplicate ${kind} ${id}`);seen.add(id);}};
 const classes=new Set(set.classes.map(row=>row.id)),criteria=new Map(set.criteria.map(row=>[row.id,row]));
 unique("class",set.classes.map(row=>row.id));unique("criteria",set.criteria.map(row=>row.id));unique("workload",set.workloads.map(row=>row.id));
 for(const row of set.classes)for(const pattern of row.patterns)try{new RegExp(pattern,"u");}catch{found.push(`${row.id}: invalid pattern ${pattern}`);}
 for(const row of set.criteria){
  unique(`rule of ${row.id}`,row.rules.map(rule=>rule.id));
  for(const rule of row.rules)if(rule.kind==="max-wait")for(const name of rule.classes)if(name!=="*"&&!classes.has(name))found.push(`${row.id}/${rule.id}: unknown lock class ${name}`);
 }
 for(const workload of set.workloads){
  const sessions=new Set(workload.sessions.map(row=>row.id)),owners=new Map<string,string>();
  unique(`session of ${workload.id}`,workload.sessions.map(row=>row.id));
  const rules=criteria.get(workload.criteria);
  if(!rules)found.push(`${workload.id}: unknown criteria ${workload.criteria}`);
  else for(const rule of rules.rules)if("sessions" in rule)for(const name of rule.sessions)if(name!=="*"&&!sessions.has(name))found.push(`${workload.id}/${rule.id}: unknown session ${name}`);
  const all=[{id:"setup",steps:workload.setup},...workload.sessions];
  for(const owner of all){
   const barriers=owner.steps.flatMap(step=>step.kind==="barrier"?[step.id]:[]);
   unique(`barrier in ${workload.id}/${owner.id}`,barriers);
   for(const step of owner.steps)if(step.kind==="edit"&&step.op!=="whitespace")try{concurrentMutateV1("",step.op,step.language,"aaaaaa");}catch(error){found.push(`${workload.id}/${owner.id}: ${(error as Error).message}`);}
   for(const file of stepFiles(owner.steps)){const holder=owners.get(file);if(holder!==undefined&&holder!==owner.id)found.push(`${workload.id}: ${file} is edited by ${holder} and ${owner.id}`);owners.set(file,owner.id);}
  }
 }
 return found;
}

/** 🎟 Selects workloads by id (`all` selects every one, an unknown id is refused) and keeps the first `sessions` sessions of each. */
export function concurrentSelectV1(set:ConcurrentWorkloadSetV1,ids:readonly string[]|"all",sessions:number|undefined):ConcurrentWorkloadV1[]{
 const unknown=ids==="all"?[]:ids.filter(id=>!set.workloads.some(row=>row.id===id));
 if(unknown.length)throw Error(`Unknown workload: ${unknown.join(", ")} (${set.workloads.map(row=>row.id).join("|")})`);
 const picked=ids==="all"?[...set.workloads]:ids.map(id=>set.workloads.find(row=>row.id===id)!),limit=sessions??policy.defaultSessions;
 if(!Number.isInteger(limit)||limit<1||limit>policy.maximumSessions)throw Error(`Sessions must be between 1 and ${policy.maximumSessions}`);
 return picked.map(row=>Object.freeze({...row,sessions:row.sessions.slice(0,limit)}));
}

const text=(set:ConcurrentWorkloadSetV1,key:string,language:"en"|"de"):string=>{const row=set.labels[key];if(!row)throw Error(`Unknown concurrent label ${key}`);return row[language];};
const grid=(head:readonly string[],body:readonly (readonly string[])[],left:readonly number[]):string[]=>{
 const widths=head.map((cell,column)=>Math.max(cell.length,...body.map(row=>row[column]!.length)));
 return [head,...body].map(cells=>cells.map((cell,column)=>left.includes(column)?cell.padEnd(widths[column]!):cell.padStart(widths[column]!)).join("  ").trimEnd());
};

/** 🖼 Renders a report in the chosen language: per run the sessions with wall time and wait time, the lock waits per class, the criteria with their verdicts and offenders, and the acceptance per executor. */
export function renderConcurrentReportV1(report:ConcurrentReportV1,set:ConcurrentWorkloadSetV1,language:"en"|"de"):string{
 const t=(key:string)=>text(set,key,language),out:string[]=[t("title")];
 for(const run of report.runs){
  const{measurement}=run,classLabel=(id:string)=>set.classes.find(row=>row.id===id)?.label[language]??id;
  out.push("",`${t("workload")}: ${run.workload} / ${run.tool} / ${t("criteria")}: ${run.criteria}`);
  const sessions=measurement.sessions.map(session=>{
   const runs=session.steps.filter(step=>usable(step)),executed=runs.reduce((sum,step)=>sum+step.observation!.executed.length,0),restored=runs.reduce((sum,step)=>sum+step.observation!.restored.length,0);
   return [session.id,session.group,String(session.steps.length),String(session.endMs-session.startMs),String(waitTotal(session)),String(executed),String(restored)];
  });
  out.push(...grid([t("session"),t("group"),t("steps"),t("wall"),t("wait"),t("executed"),t("restored")],sessions,[0,1]));
  const byKey=new Map<string,{session:string;lease:string;count:number;total:number;maximum:number;unrelated:number}>();
  for(const session of measurement.sessions)for(const wait of session.steps.flatMap(step=>step.waits)){
   const key=`${session.id}\0${wait.class}`,row=byKey.get(key)??{session:session.id,lease:wait.class,count:0,total:0,maximum:0,unrelated:0};
   row.count++;row.total+=wait.waitMs;row.maximum=Math.max(row.maximum,wait.waitMs);if(wait.relation!=="related")row.unrelated++;
   byKey.set(key,row);
  }
  if(byKey.size){
   out.push("",t("leases"));
   out.push(...grid([t("session"),t("lease"),t("count"),t("total"),t("maximum"),t("unrelated")],[...byKey.values()].sort((left,right)=>order(left.session,right.session)||order(left.lease,right.lease)).map(row=>[row.session,classLabel(row.lease),String(row.count),String(row.total),String(row.maximum),String(row.unrelated)]),[0,1]));
  }
  const rules=criteriaOf(set,run.criteria);
  out.push("",t("rules"));
  out.push(...grid([t("rule"),t("status"),t("value"),t("limit"),t("owner")],run.rules.map(rule=>[rules?.rules.find(row=>row.id===rule.rule)?.title[language]??rule.rule,t(rule.status),rule.value===null?"-":String(rule.value),rule.limit===null?"-":String(rule.limit),rule.owners.join(",")]),[0,1,4]));
  for(const rule of run.rules)if(rule.status!=="pass")for(const line of rule.detail)out.push(`  ${rule.rule}: ${line}`);
 }
 out.push("",`${t("summary")}: ${t("pass")} ${report.passed}, ${t("fail")} ${report.failed}, ${t("skip")} ${report.skipped}`);
 if(report.owners.length){
  out.push("",t("acceptance"));
  for(const row of report.owners)out.push(`  ${row.owner}: ${row.failed?t("fail"):row.skipped?t("unproven"):t("pass")} (${t("pass")} ${row.passed}, ${t("fail")} ${row.failed}, ${t("skip")} ${row.skipped})`);
 }
 return out.join("\n");
}
const criteriaOf=(set:ConcurrentWorkloadSetV1,id:string)=>set.criteria.find(row=>row.id===id);

/** 🪧 Renders the workload list in the chosen language: id, sessions with their groups, steps per session and title. */
export function renderConcurrentWorkloadsV1(set:ConcurrentWorkloadSetV1,language:"en"|"de"):string{
 const t=(key:string)=>text(set,key,language),body=set.workloads.map(row=>[row.id,row.criteria,String(row.sessions.length),row.sessions.map(session=>`${session.id}[${session.group}]`).join(" "),row.title[language]]);
 return grid([t("workload"),t("criteria"),t("sessions"),t("group"),t("title")],body,[0,1,3,4]).join("\n");
}

/** 📛 The label of a wait's class in a language, for plans and tests. */
export function concurrentClassLabelV1(set:ConcurrentWorkloadSetV1,id:string,language:"en"|"de"):string{return set.classes.find(row=>row.id===id)?.label[language]??id;}
