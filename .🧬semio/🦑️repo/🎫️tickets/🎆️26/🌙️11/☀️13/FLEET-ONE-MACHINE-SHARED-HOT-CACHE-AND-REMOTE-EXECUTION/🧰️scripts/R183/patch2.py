# R183 part 2 (executed by patch.py): f5 ready code, f7 graph server.
edit('f5',',readyScaledTreePolicyV1,readyStallRestartV1',',readyHolderWaitOfV1,readyHolderCustodyV1,readyBackgroundReaderV1,readyForegroundBusyV1,type ReadyHolderWaitV1,type ReadyReadRecordV1,readyScaledTreePolicyV1,readyStallRestartV1')
edit('f5','"stage-code-stale"|"store-full"','"stage-code-stale"|"stage-holder-silent"|"store-full"')
edit('f5','"stalled"|"code-changed"|"code-stale"|"adopted"|"binary-missing";','"stalled"|"code-changed"|"code-stale"|"holder-silent"|"adopted"|"binary-missing";')
edit('f5','export type ReadyRunResultV1=Readonly<{containment?:ReadyContainmentResultV1;','export type ReadyRunResultV1=Readonly<{containment?:ReadyContainmentResultV1;holderSilent?:Readonly<{pid:number;command:string;seconds:number}>;')
edit('f5','const P=READY_POLICY_V1,ZERO="0".repeat(64),','''/** 🏔️ Whether the readiness loop of `origin` is busy (see `readyForegroundBusyV1`), read from its `ready.json` in `store`; a store that is not there or a file that cannot be read says no, so a machine without a readiness loop (the group server) is never held back. */
export function readyForegroundBusyOfStoreV1(store:string|undefined,origin:string,nowMs:number=Date.now()):boolean{
 if(store===undefined||store==="")return false;
 try{return readyForegroundBusyV1(JSON.parse(readFileSync(join(store,"v1","origins",origin,READY_POLICY_V1.statusName),"utf8")),nowMs);}catch{return false;}
}
const P=READY_POLICY_V1,ZERO="0".repeat(64),''')
edit('f5','let killReason:ReadyKillReasonV1="stall",cpuBefore:ReadonlyMap<number,number>=new Map();','let killReason:ReadyKillReasonV1="stall",cpuBefore:ReadonlyMap<number,number>=new Map(),inspecting=false,inspectedAt=0,silentHolder:Readonly<{pid:number;command:string;seconds:number}>|undefined;const yielded=new Set<number>();')
edit('f5','  const done=(result:ReadyRunResultV1):void=>{clearInterval(timer);','''  const readRecord=():ReadyReadRecordV1|undefined=>{try{const value=JSON.parse(readFileSync(machinePathV1(input.root,...treePolicy.readRecord),"utf8")) as Partial<ReadyReadRecordV1>;return Number.isSafeInteger(value.runner)&&Number.isFinite(value.startedMs)&&(value.reason==="request"||value.reason==="refresh")?{reason:value.reason,runner:value.runner!,read:value.read??null,startedMs:value.startedMs!,status:value.status??null,endedMs:value.endedMs??null}:undefined;}catch{return undefined;}};
  const inspect=async(named:ReadyHolderWaitV1,waiting:number):Promise<void>=>{
   const table=await readyReadTableV1(),entries=input.orphans===undefined?[]:readyRegistryEntriesV1(input.orphans.registryRoot,contain),entry=entries.filter(row=>row.pid===named.pid).sort((left,right)=>right.startedAtMs-left.startedAtMs)[0],verdict=readyHolderCustodyV1({wait:named,entry,table,nowMs:Date.now(),policy:treePolicy});
   if(killed)return;
   silentHolder=verdict.kind==="silent"?{pid:named.pid,command:named.command,seconds:Math.round(verdict.silentMs/1000)}:undefined;
   if(verdict.kind==="progressing"||verdict.kind==="gone"){stallAt=Date.now();progressAt=Date.now();}
   if(verdict.kind!=="progressing"&&verdict.kind!=="silent"||waiting<treePolicy.holderYieldMilliseconds||yielded.has(named.pid))return;
   const reader=readyBackgroundReaderV1({record:readRecord(),holder:named.pid,table,self:process.pid,policy:treePolicy});
   if(reader===undefined)return;
   yielded.add(named.pid);
   const seconds=Math.round(waiting/1000),record=readyKillRecorderV1({directory,stage:inflight.stage,owner:ownerOf(inflight.stage,inflight.startedMs),reason:"yield",seconds,say,policy:contain});
   say(readyContainmentLineV1("yielded-reader",{stage:inflight.stage,seconds,pid:named.pid,name:named.command.slice(0,80),reason:reader.reason}));
   await readyKillTreeV1({root:reader.root,known:[],signal:"SIGTERM",table,spare:new Set<number>(),record});
   setTimeout(()=>{void readyKillTreeV1({root:reader.root,known:[],signal:"SIGKILL",spare:new Set<number>(),record});},P.graceMilliseconds).unref();
  };
  const done=(result:ReadyRunResultV1):void=>{clearInterval(timer);''')
edit('f5','const full=result.detached?result:{...result,containment:{reaped:[...reaped],survivors:[...left.values()]}};','const full=result.detached?result:{...result,containment:{reaped:[...reaped],survivors:[...left.values()]},...silentHolder!==undefined&&result.stalled?{holderSilent:silentHolder}:{}};')
edit('f5','seenPhase=readyStagePhaseV1(fed.lines,phase,treePolicy);phaseOffset=fed.offset;','seenPhase=readyStagePhaseV1(fed.lines,phase,treePolicy);phaseOffset=fed.offset;if(!killed&&!inspecting&&waiting>0&&Date.now()-inspectedAt>=treePolicy.holderCheckMilliseconds){const named=readyHolderWaitOfV1(recent,treePolicy);if(named!==undefined){inspecting=true;inspectedAt=Date.now();void inspect(named,waiting).catch(()=>undefined).finally(()=>{inspecting=false;});}}')
edit('f5','const byCode=result.codeChanged!==undefined,stale=result.codeStale===true,','const byCode=result.codeChanged!==undefined,stale=result.codeStale===true,silent=result.codeChanged===undefined?result.holderSilent:undefined,')
edit('f5','stalledReason=stale?"code-stale":byCode?"code-changed":"stalled";','stalledReason=stale?"code-stale":byCode?"code-changed":silent!==undefined?"holder-silent":"stalled";')
edit('f5','     else ports.say(`stage ${row.name} showed nothing new for ','''     else if(silent!==undefined)ports.say(`stage ${row.name} waited for process ${silent.pid} (${silent.command}) that holds the dependency store but has not renewed its registration for ${silent.seconds} s, it may be blocked; its process tree was stopped, the stage starts again in ${Math.round(pause/1000)} s / Stufe ${row.name} wartete auf Prozess ${silent.pid} (${silent.command}), der den Abhängigkeitsspeicher hält, aber seine Anmeldung seit ${silent.seconds} s nicht erneuert hat und womöglich blockiert ist; ihr Prozessbaum wurde beendet, sie startet in ${Math.round(pause/1000)} s neu`);
     else ports.say(`stage ${row.name} showed nothing new for ''')
edit('f5',':readyRefusalV1("stage-stalled",{stage:row.name,minutes,seconds:',':silent!==undefined?readyRefusalV1("stage-holder-silent",{stage:row.name,minutes,pid:silent.pid,command:silent.command,silent:silent.seconds,seconds:Math.round(pause/1000),count:stallCount,maximum:READY_TREE_POLICY_V1.stallRestartMaximum}):readyRefusalV1("stage-stalled",{stage:row.name,minutes,seconds:')

edit('f7','import {measureReadTreeV1,memoryOverrunV1} from "./🔬️stall/🌳️tree/🟦️.ts";','import {measureReadTreeV1,memoryOverrunV1} from "./🔬️stall/🌳️tree/🟦️.ts";\nimport {readyForegroundBusyOfStoreV1} from "../🚦️ready/🟦️.ts";')
edit('f7','async mayServe(){return(await readRemoteExecutionHostViewV1(environment)).host.totalMegabytes>=policy.serveTotalMegabytes;},','async mayServe(){return(await readRemoteExecutionHostViewV1(environment)).host.totalMegabytes>=policy.serveTotalMegabytes&&!readyForegroundBusyOfStoreV1(store,origin);},')
edit('f7','async mayRefresh(){const key=','async mayRefresh(){if(readyForegroundBusyOfStoreV1(store,origin))return false;const key=')
edit('f7','A machine without a fleet cache waits and looks again. */','A machine without a fleet cache waits and looks again. While the readiness loop of the machine is busy (its `ready.json` says not ready and running, waiting or backing off) it neither serves requests nor refreshes: readiness is the foreground, a background read holds the dependency store for hours, and a read still running when a stage queues behind it is stopped by the stage runner (`yield`). */')
print("patched",d)
