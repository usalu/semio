"""🚪️ The staged patches of the gate (phase marks, the holder's segments and re-sizing) and of its contract."""
F="🧰️framework"
HC=f"{F}/🔨️modules/🏃️process/🏗️build/🌡️host-capacity"
GATE=f"{HC}/🎫️gate/🟦️.ts"
CONTRACT=f"{HC}/🎫️gate/📋️contract/🟦️.ts"

def apply(root,replace,read,write):
    out=[]
    out.append(replace(root,GATE,'import {mkdirSync,readdirSync,readFileSync,renameSync,rmSync,statSync,unlinkSync,utimesSync,writeFileSync} from "node:fs";','import {appendFileSync,existsSync,mkdirSync,readdirSync,readFileSync,renameSync,rmSync,statSync,unlinkSync,utimesSync,writeFileSync} from "node:fs";'))
    out.append(replace(root,GATE,'import {currentHostProbeV1,observeTreeMegabytesV1,readHostSampleV1,type HostSampleRecordV1} from "../📡️sampling/🟦️.ts";\n','import {currentHostProbeV1,observeTreeMegabytesV1,readHostSampleV1,type HostSampleRecordV1} from "../📡️sampling/🟦️.ts";\nimport {HOST_PHASES_V1,hostPhaseSegmentsV1,type HostPhaseTransitionV1,type HostPhaseV1} from "../🪜️phases/🟦️.ts";\n'))
    out.append(replace(root,GATE,'const queuePath=(directory:string,arrivedAtMs:number,id:string)=>',
'''const phasesPath=(directory:string,id:string)=>join(directory,"grants",`${id}${protocol.phasesSuffix}`);
/** 🚏️ Lets any process of a run say which phase the run enters (assembly, plan, rebuild, execution): one line `<phase> <milliseconds>` appended to the grant's phase file, which the holder reads within a second; false when the environment carries no live grant, the switch of a nested invocation is off (an inherited grant belongs to the invocation that holds it) or the phase is unknown. */
export function markHostCapacityPhaseV1(environment:Readonly<Record<string,string|undefined>>,phase:HostPhaseV1,now:()=>number=Date.now):boolean{
 const id=environment[protocol.grantEnvironment];
 if(!id||environment[protocol.marksEnvironment]==="off"||!(HOST_PHASES_V1 as readonly string[]).includes(phase)||!new RegExp(schema.definitions.Grant.properties.id.pattern,"u").test(id))return false;
 try{const directory=hostCapacityDirectoryV1(environment);if(!existsSync(grantPath(directory,id)))return false;appendFileSync(phasesPath(directory,id),`${phase} ${now()}\\n`);return true;}catch{return false;}
}
const queuePath=(directory:string,arrivedAtMs:number,id:string)=>'''))
    out.append(replace(root,GATE,'plan,inherited:true,adopted:false,bypassed:false,environment:Object.freeze({[protocol.grantEnvironment]:grant.id,[protocol.planEnvironment]:JSON.stringify(plan)})}));','plan,inherited:true,adopted:false,bypassed:false,environment:Object.freeze({[protocol.grantEnvironment]:grant.id,[protocol.planEnvironment]:JSON.stringify(plan),[protocol.marksEnvironment]:"off"})}));'))
    out.append(replace(root,GATE,'plan,inherited:false,adopted,bypassed:false,environment:Object.freeze({[protocol.grantEnvironment]:held.id,[protocol.planEnvironment]:JSON.stringify(plan)})}));','plan,inherited:false,adopted,bypassed:false,environment:Object.freeze({[protocol.grantEnvironment]:held.id,[protocol.planEnvironment]:JSON.stringify(plan),[protocol.marksEnvironment]:"on"})}));'))
    out.append(replace(root,GATE,'failure:unknown,failed=false,marks:(readonly [number,number])[]=[];','failure:unknown,failed=false,marks:(readonly [number,number,HostPhaseV1|undefined])[]=[],phase:HostPhaseV1|undefined,transitions:HostPhaseTransitionV1[]=[],markLines=0,markBytes=0;'))
    out.append(replace(root,GATE,'try{unlinkSync(attachedPath(directory,held.id));}catch{}};','try{unlinkSync(attachedPath(directory,held.id));}catch{}try{unlinkSync(phasesPath(directory,held.id));}catch{}};'))
    out.append(replace(root,GATE,
'''const heartbeat=()=>{void observeAll().then(observed=>{if(holding&&observed!==null){peak=Math.max(peak,observed);marks.push([activeAt(now()),observed]);''',
'''const heartbeat=(tag:HostPhaseV1|undefined=phase)=>{void observeAll().then(observed=>{if(holding&&observed!==null){peak=Math.max(peak,observed);marks.push([activeAt(now()),observed,tag]);'''))
    out.append(replace(root,GATE,
' const first=setTimeout(heartbeat,Math.min(3000,policy.wait.heartbeatMilliseconds)),timer=setInterval(heartbeat,policy.wait.heartbeatMilliseconds);\n',
''' const enter=(next:HostPhaseV1):void=>{
  const at=now(),active=activeAt(at);heartbeat(phase);transitions.push({phase:next,atMilliseconds:active});phase=next;
  let resized:ReturnType<NonNullable<HostCapacityOptionsV1["resize"]>>;try{resized=options.resize?.(next,{jobs:held.jobs,reservedMegabytes:state.reservedMegabytes,activeMilliseconds:active});}catch{resized=undefined;}
  publish({...state,phase:next,...resized===undefined?{}:{reservedMegabytes:resized.reservedMegabytes,rampMilliseconds:resized.rampMilliseconds,resumedAtMs:at}});
 };
 const readPhases=():void=>{
  const file=phasesPath(directory,held.id);let size=0;try{size=statSync(file).size;}catch{return;}
  if(size===markBytes||size>protocol.maximumPhaseMarkBytes)return;
  markBytes=size;
  const rows=readFileSync(file,"utf8").split("\\n").filter(row=>row!=="");
  for(const row of rows.slice(markLines)){
   markLines++;const next=row.split(" ")[0] as HostPhaseV1;
   if((HOST_PHASES_V1 as readonly string[]).includes(next)&&next!==phase&&transitions.length<protocol.maximumPhaseTransitions)enter(next);
  }
 };
 const first=setTimeout(()=>heartbeat(),Math.min(3000,policy.wait.heartbeatMilliseconds)),timer=setInterval(()=>heartbeat(),policy.wait.heartbeatMilliseconds);
'''))
    out.append(replace(root,GATE,
'''  try{
   const at=now(),episode=(options.waited??scriptProcessWaitEpisodeV1)(at),lent=state.lentSinceMs!==undefined;''',
'''  try{
   readPhases();
   const at=now(),episode=(options.waited??scriptProcessWaitEpisodeV1)(at),lent=state.lentSinceMs!==undefined;'''))
    out.append(replace(root,GATE,
'''const ended=now(),active=activeAt(ended),near=marks.find(([,megabytes])=>megabytes*10000>=peak*protocol.peakNearBasisPoints);try{await options.learn(Object.freeze({jobs:held.jobs,peakMegabytes:peak,activeMilliseconds:active,peakAtMilliseconds:Math.min(near?.[0]??active,active),endedAtMs:ended,status:''',
'''const ended=now(),active=activeAt(ended),near=marks.find(([,megabytes])=>megabytes*10000>=peak*protocol.peakNearBasisPoints);readPhases();const phases=hostPhaseSegmentsV1(marks,transitions,active);try{await options.learn(Object.freeze({jobs:held.jobs,peakMegabytes:peak,activeMilliseconds:active,peakAtMilliseconds:Math.min(near?.[0]??active,active),endedAtMs:ended,...phases.length===0?{}:{phases},status:'''))
    out.append(replace(root,GATE,"An adopted grant carries the plan its owner admitted. */","An adopted grant carries the plan its owner admitted. A run that marks its phases (`markHostCapacityPhaseV1`) is cut into phase segments by the readings it was observed at, and its reservation follows the `resize` port at each boundary; the segments go to `learn`. */"))
    out.append(replace(root,CONTRACT,'import type {HostSampleRecordV1} from "../../📡️sampling/🟦️.ts";\n','import type {HostSampleRecordV1} from "../../📡️sampling/🟦️.ts";\nimport type {HostPhaseResizeV1,HostPhaseSegmentV1,HostPhaseV1} from "../../🪜️phases/🟦️.ts";\n'))
    out.append(replace(root,CONTRACT,'endedAtMs:number;status:HostRunStatusV1}>;','endedAtMs:number;status:HostRunStatusV1;phases?:readonly HostPhaseSegmentV1[]}>;'))
    out.append(replace(root,CONTRACT,'and whether its body completed, failed or was cancelled. */','and whether its body completed, failed or was cancelled; `phases` are the segments of a run that marked its phases. */'))
    out.append(replace(root,CONTRACT,'learn?:(run:HostRunV1)=>void|Promise<void>}>;','learn?:(run:HostRunV1)=>void|Promise<void>;resize?:(phase:HostPhaseV1,context:Readonly<{jobs:number;reservedMegabytes:number;activeMilliseconds:number}>)=>HostPhaseResizeV1|undefined}>;'))
    return out
