import {readFileSync} from "node:fs";
import {resolve} from "node:path";

const ready=resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready");
export const readySource=resolve(ready,"🟦️.ts");
export const readySchema=resolve(ready,"🧬️schema/🔣️.json");

/** 🧷️ Replaces one exact source anchor for the privately tested Ready patch. */
function replace(source:string,before:string,after:string):string{
 const first=source.indexOf(before);
 if(first<0||source.lastIndexOf(before)!==first)throw Error("Ready source anchor changed: "+before);
 return source.slice(0,first)+after+source.slice(first+before.length);
}

/** 🧬️ Adds a closed receipt shape shared by the timing and same-flight checkpoint. */
function schemaPatch(source:string):string{
 const schema=JSON.parse(source),defs=schema.definitions;
 if(defs.OwnedActivity!==undefined||defs.DeadlineStop!==undefined||defs.StepOverdueEvidence!==undefined||defs.TimingWatchdog!==undefined)throw Error("Ready terminal evidence schema already present");
 defs.OwnedActivity={
  description:"The latest recognized owned activity and its producer's work counter.",type:"object",additionalProperties:false,required:["resource","counter"],
  properties:{
   resource:{type:"string",pattern:"^(?:native-artifact-input-(?:stage|read|verification)|nx:source-(?:index|verification)|javascript-(?:selected-payload|preserving-source)|dependencies:(?:workspace-edges|Source-aliases|selected-postorder|sync-bindings|store-ownership:frontier))$"},
   counter:{type:"integer",minimum:1,maximum:Number.MAX_SAFE_INTEGER}
  }
 };
 defs.DeadlineStop={enum:["hard-cap","soft-cap-idle"]};
 const boundedInteger={$ref:"#/definitions/WatchdogInteger"};
 defs.StepOverdueEvidence={
  description:"The owned overdue decision: step elapsed, declared bound and computed limit in seconds, and time since other progress in milliseconds.",
  type:"object",additionalProperties:false,required:["seconds","bound","limit","quietMs"],properties:{seconds:boundedInteger,bound:boundedInteger,limit:boundedInteger,quietMs:boundedInteger}
 };
 defs.TimingWatchdog={type:"object",additionalProperties:false,minProperties:1,properties:{lastActivity:{$ref:"#/definitions/OwnedActivity"},deadline:{$ref:"#/definitions/DeadlineStop"},stepOverdue:{$ref:"#/definitions/StepOverdueEvidence"}}};
 const definitions=Object.entries(defs).slice(-4).map(([key,value])=>"    "+JSON.stringify(key)+": "+JSON.stringify(value)).join(",\n");
 source=replace(source,'    "WindowsWatchdogSample": {',definitions+',\n    "WindowsWatchdogSample": {');
 source=replace(source,'"launch": {"$ref": "#/definitions/StageLaunch"}, "stop": {"enum": ["stall", "overdue", "restart", "timeout", "cancel"]}', '"launch": {"$ref": "#/definitions/StageLaunch"}, "stop": {"$ref": "#/definitions/StageStop"}, "deadline": {"$ref": "#/definitions/DeadlineStop"}, "stepOverdue": {"$ref": "#/definitions/StepOverdueEvidence"}');
 source=replace(source,'        "windows": {"$ref": "#/definitions/WindowsWatchdogSample"}','        "windows": {"$ref": "#/definitions/WindowsWatchdogSample"},\n        "lastActivity": {"$ref": "#/definitions/OwnedActivity"}');
 return replace(source,'"endEvidence": {"enum": ["exit-receipt", "observed"]}, "signal": {"type": ["string", "null"]}', '"endEvidence": {"enum": ["exit-receipt", "observed"]}, "signal": {"type": ["string", "null"]}, "watchdog": {"$ref": "#/definitions/TimingWatchdog"}');
}

/** 🚦️ Stages only the existing owned log, checkpoint, stop and timing paths. */
function sourcePatch(source:string):string{
 const insert=`type ReadyOwnedActivityV1=Readonly<{resource:string;counter:number}>;
type ReadyDeadlineStopV1="hard-cap"|"soft-cap-idle";
type ReadyStepOverdueEvidenceV1=Readonly<{seconds:number;bound:number;limit:number;quietMs:number}>;
type ReadyStopEvidenceV1=Readonly<{deadline?:ReadyDeadlineStopV1;stepOverdue?:ReadyStepOverdueEvidenceV1}>;
type ReadyTimingWatchdogV1=ReadyStopEvidenceV1&Readonly<{lastActivity?:ReadyOwnedActivityV1}>;
`;
 source=replace(source,"export type ReadyTimingV1=Readonly<{",insert+"export type ReadyTimingV1=Readonly<{watchdog?:ReadyTimingWatchdogV1;");
 source=replace(source,"export type ReadyRunResultV1=Readonly<{","export type ReadyRunResultV1=Readonly<{watchdog?:ReadyTimingWatchdogV1;");
 source=replace(source,"attempts:null,digest});","attempts:null,digest,...timingWatchdogV1(result.watchdog)});");
 source=replace(source,"...physical===undefined?{}:{endEvidence:physical.endEvidence,signal:result.signal??null}});","...physical===undefined?{}:{endEvidence:physical.endEvidence,signal:result.signal??null},...timingWatchdogV1(result.watchdog)});");
 source=replace(source,"const readyTerminalOutcomeV1=", `const timingWatchdogV1=(watchdog:ReadyTimingWatchdogV1|undefined)=>watchdog===undefined?{}:{watchdog};
const readyTimingWatchdogV1=(lastActivity:ReadyOwnedActivityV1|undefined,evidence:ReadyStopEvidenceV1)=>timingWatchdogV1(lastActivity===undefined&&evidence.deadline===undefined&&evidence.stepOverdue===undefined?undefined:{...lastActivity===undefined?{}:{lastActivity},...evidence});
const readyTerminalOutcomeV1=`);
 source=replace(source,"windows?:ReadyWindowsWatchdogSampleV1;","windows?:ReadyWindowsWatchdogSampleV1;lastActivity?:ReadyOwnedActivityV1;");
 source=replace(source,"stop?:ReadyStageStopV1}>;","stop?:ReadyStageStopV1}>&ReadyStopEvidenceV1;");
 source=replace(source,"const readyStoppedForStallV1=", `const readyValidStepEvidenceV1=(step:ReadyStepOverdueEvidenceV1|undefined):step is ReadyStepOverdueEvidenceV1=>step!==undefined&&!validateJsonSchemaSubset(schema.definitions.StepOverdueEvidence,step,schema).length&&step.seconds>=step.limit&&step.limit>=step.bound;
const readyRecordedStopEvidenceV1=(flight:ReadyInflightV1,stop:ReadyStageStopV1|undefined):ReadyStopEvidenceV1=>{
 if(stop==="timeout"&&flight.deadline!==undefined&&schema.definitions.DeadlineStop.enum.includes(flight.deadline))return{deadline:flight.deadline};
 if(stop==="overdue"&&readyValidStepEvidenceV1(flight.stepOverdue))return{stepOverdue:flight.stepOverdue};
 return{};
};
const readyStoppedForStallV1=`);
 source=replace(source,"/** 🗂️ Recovers structured tool refusals", String.raw`const readyActivityResourceV1=new RegExp(schema.definitions.OwnedActivity.properties.resource.pattern,"u");
const readyActivityCounterV1=/^([A-Za-z-]+(?::[A-Za-z-]+)*):([1-9][0-9]{0,15})(?:\/[1-9][0-9]{0,3}:canonical:[0-9]{1,4}:changed:[0-9]{1,4})?$/u;

/** 📍️ Retains the latest productive counter from the bounded batch of complete stage log lines. */
function readyOwnedActivityV1(lines:readonly string[],last:ReadyOwnedActivityV1|undefined):ReadyOwnedActivityV1|undefined{
 for(const line of lines){
  if(line.length>P.refusalLineCharacters||!line.startsWith("[script] {"))continue;
  let row:{stage?:string;activity?:{resource?:string;mode?:string;elapsedMs?:number}}|null;
  try{row=JSON.parse(line.slice(9));}catch{continue;}
  if(row?.stage!=="running"||typeof row.activity?.resource!=="string"||(row.activity.mode!=="shared"&&row.activity.mode!=="exclusive")||
   typeof row.activity.elapsedMs!=="number"||!Number.isFinite(row.activity.elapsedMs)||row.activity.elapsedMs<0)continue;
  const matched=readyActivityCounterV1.exec(row.activity.resource);
  if(matched===null||!readyActivityResourceV1.test(matched[1]!))continue;
  const resource=matched[1]!,counter=Number(matched[2]);
  if(Number.isSafeInteger(counter)&&(last?.resource!==resource||counter>last.counter))last={resource,counter};
 }
 return last;
}

/** 🗂️ Recovers structured tool refusals`);
 source=replace(source,"  let stop:ReadyStageStopV1|undefined;","  let stop:ReadyStageStopV1|undefined,evidenceAtStop:ReadyStopEvidenceV1={};");
 source=replace(source,"    stop=readyRecordedStageStopV1(held);","    stop=readyRecordedStageStopV1(held);\n    evidenceAtStop=readyRecordedStopEvidenceV1(held,stop);");
 source=replace(source,"return {...failure,timedOut:stop", "return {...failure,...readyTimingWatchdogV1(observed?.lastActivity,evidenceAtStop),timedOut:stop");
 source=replace(source,"  const stop=readyRecordedStageStopV1(inflight);","  const stop=readyRecordedStageStopV1(inflight);\n  let evidenceAtStop=readyRecordedStopEvidenceV1(inflight,stop);");
 source=replace(source,"cpuBefore:ReadonlyMap<number,number>=new Map(prior?.cpu)","lastActivity=prior?.lastActivity,cpuBefore:ReadonlyMap<number,number>=new Map(prior?.cpu)");
 source=replace(source,"   phase=readyStagePhaseV1(fed.lines,phase,treePolicy);","   lastActivity=readyOwnedActivityV1(fed.lines,lastActivity);\n   phase=readyStagePhaseV1(fed.lines,phase,treePolicy);");
 source=replace(source,"...windowsBefore===undefined?{}:{windows:windowsBefore}};","...windowsBefore===undefined?{}:{windows:windowsBefore},...lastActivity===undefined?{}:{lastActivity}};");
 source=replace(source,"  const kill=(reason:ReadyStageStopV1):void=>{\n   if(killed||!watching)return;","  const kill=(reason:ReadyStageStopV1,evidence:ReadyStopEvidenceV1={}):boolean=>{\n   if(killed||!watching)return false;\n   checkpoint(true);");
 source=replace(source,`   if(held===undefined||!sameReadyFlightV1(held,inflight))return;
   if(held.launch!==undefined)try{writeReadyLocalRecordV1(inflightFile,{...held,stop:reason});}catch(error){failWatch(error);return;}
   killed=true;killReason=reason;void settle().catch(failWatch);`,`   if(held===undefined||!sameReadyFlightV1(held,inflight))return false;
   if(held.launch!==undefined)try{writeReadyLocalRecordV1(inflightFile,{...held,stop:reason,...evidence});}catch(error){failWatch(error);return false;}
   killed=true;killReason=reason;evidenceAtStop=evidence;void settle().catch(failWatch);return true;`);
 source=replace(source,"result={...result,physicalEnd:readyPhysicalEndV1(inflight,Date.now(),endedMs)};","if(!killed&&owns())lastActivity=readyOwnedActivityV1(logSinceV1(inflight.log,phaseOffset).lines,lastActivity);\n   result={...result,...readyTimingWatchdogV1(lastActivity,evidenceAtStop),physicalEnd:readyPhysicalEndV1(inflight,Date.now(),endedMs)};");
 source=replace(source,"phaseOffset=fed.offset;if(!killed&&!inspecting", "phaseOffset=fed.offset;if(!killed)lastActivity=readyOwnedActivityV1(fed.lines,lastActivity);if(!killed&&!inspecting");
 source=replace(source,'overdueStep={...watched.step,limit:readyStepOverdueSecondsV1(watched.step.bound,treePolicy),quiet:Math.round(watched.quietMs/1000)};stalled=true;kill("overdue");',`
     const {seconds,bound}=watched.step,limit=readyStepOverdueSecondsV1(bound,treePolicy),stepOverdue={seconds,bound,limit,quietMs:watched.quietMs};
     if(kill("overdue",readyValidStepEvidenceV1(stepOverdue)?{stepOverdue}:{})){overdueStep={...watched.step,limit,quiet:Math.round(watched.quietMs/1000)};stalled=true;}
    `);
 return replace(source,'timedOut=true;kill("timeout");','timedOut=true;kill("timeout",{deadline:verdict==="timeout"?"hard-cap":"soft-cap-idle"});');
}

/** 🧪️ Hooks the two normal module paths without changing workspace or wrapper files. */
export function stageReadyTerminalEvidenceV1():void{
 Bun.plugin({
  name:"ready-terminal-evidence",
  setup(build){
   build.onLoad({filter:/🚦️ready[/\\](?:🟦️\.ts|🧬️schema[/\\]🔣️\.json)$/u},({path})=>{
    if(path===readySource)return{contents:sourcePatch(readFileSync(path,"utf8")),loader:"ts"};
    if(path===readySchema)return{exports:{default:JSON.parse(schemaPatch(readFileSync(path,"utf8")))},loader:"object"};
    return undefined;
   });
  }
 });
}

export function proposedReadyTerminalEvidenceV1():ReadonlyMap<string,string>{
 return new Map([[readySource,sourcePatch(readFileSync(readySource,"utf8"))],[readySchema,schemaPatch(readFileSync(readySchema,"utf8"))]]);
}

if(process.env.SEMIO_PRIVATE_READY_EVIDENCE==="1")stageReadyTerminalEvidenceV1();
