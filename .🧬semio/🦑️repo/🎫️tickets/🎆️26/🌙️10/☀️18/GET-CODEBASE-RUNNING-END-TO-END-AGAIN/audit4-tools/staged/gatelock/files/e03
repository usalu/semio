import {randomBytes} from "node:crypto";
import {utimesSync} from "node:fs";
import {decideHostHandoverV1,describeHostHandoverRefusalV1,formatHostCapacityV1,isCurrentHostGrantV1,isHostCapacityV1,requireHostCapacityV1,type HostGrantV1,type HostHandoverV1,type HostPlanV1,type HostRequestV1} from "../../📏️decision/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};
import type {HostCapacityOptionsV1,HostLedgerPrimitivesV1,HostProcessIdentityV1} from "../📋️contract/🟦️.ts";

/** 🪪️ The identity of the parent process, read from the system process table when no test replaces it. */
const parentIdentityV1=async():Promise<HostProcessIdentityV1|undefined>=>{const row=(await import("../../../../🪪️agent-sessions/🧭️ancestry/🟦️.ts")).systemAncestryV1()[0];return row?{pid:row.pid,started:row.started}:undefined;};
/** 🆕️ The start time of this process, read the same way the parent identity of its replacement is. */
const selfStartedV1=async():Promise<string|undefined>=>(await import("../../../../🪪️agent-sessions/🧭️ancestry/🟦️.ts")).systemAncestryV1(process.platform,process.pid)[0]?.started;

/** 🤝️ Takes over the grant of the command this process replaces, or answers undefined and says why (the request then queues like any other); the identity of the parent is read from the system process table before the gate lock is taken, so a slow listing can never outlive the lock's staleness bound and let a waiter steal it: the offered grant must be current, carry the one-time token, be fresh, have been offered by this process's own parent and be for the same operation, and the takeover rewrites the row under the gate lock, so two claimants never both hold it. */
export async function claimHostGrantHandoverV1(ledger:HostLedgerPrimitivesV1,options:HostCapacityOptionsV1,directory:string,offered:string,request:HostRequestV1,language:"en"|"de",now:()=>number,pid:number):Promise<HostGrantV1|undefined>{
 const {protocol,takeLock,releaseLock,holdsLock,grantPath,readJson,writeJson,alive,bootAt,modified,delay,checkResourceLeaseControl}=ledger,policy=options.policy,environment=options.environment??process.env,write=options.write??(line=>process.stderr.write(line+"\n")),identifier=new RegExp(schema.definitions.Grant.properties.id.pattern,"u");
 try{delete (environment as Record<string,string|undefined>)[protocol.handoverEnvironment];}catch{}
 const parent=await(options.parent??parentIdentityV1)().catch(()=>undefined);
 while(!takeLock(directory,now(),policy,pid)){
  checkResourceLeaseControl(options.control);
  try{await delay(Math.max(1,policy.wait.pollMilliseconds*(0.75+Math.random()*0.5)),undefined,{signal:options.control.signal});}catch(error){throw options.control.signal.aborted?options.control.signal.reason:error;}
 }
 try{
  const id=offered.slice(0,offered.indexOf(".")),path=identifier.test(id)?grantPath(directory,id):undefined,at=now(),row=path===undefined?undefined:readJson(path),grant=isHostCapacityV1("Grant",row)?row as HostGrantV1:null;
  const current=path!==undefined&&grant!==null&&isCurrentHostGrantV1({grant,expectedId:id,pidAlive:alive(grant.pid),nowMs:at,bootAtMs:bootAt(directory,at),modifiedAtMs:modified(path)},policy);
  const verdict=decideHostHandoverV1({grant,grantCurrent:current,offered,operation:request.operation,parent:parent??null,nowMs:at});
  if(!verdict.adopt){write(formatHostCapacityV1(language,"handoverRefused",{reason:describeHostHandoverRefusalV1(language,verdict.reason)}));return undefined;}
  const{handover:_,...rest}=grant!,taken=requireHostCapacityV1<HostGrantV1>("Grant",{...rest,pid,bootAtMs:bootAt(directory,at)});
  if(!holdsLock(directory,pid))return undefined;
  writeJson(path!,taken);
  write(formatHostCapacityV1(language,"adopted",{operation:taken.operation,jobs:taken.jobs,reserved:taken.reservedMegabytes}));
  return taken;
 }finally{releaseLock(directory,pid);}
}

/** 🅿️ Keeps the grant of a body that failed in a way a restart cures: the row stays in the ledger, still heartbeating and still counted, so nobody is admitted into the gap; the restart wrapper offers it to the process that replaces this one ({@link claimHostGrantHandoverV1}) and settles it afterwards, and a grant nobody offers within the declared bound is given back. The restart custody registry of the churn module is the one place the wrapper looks. */
export async function parkHostGrantV1(ledger:HostLedgerPrimitivesV1,input:Readonly<{options:HostCapacityOptionsV1;path:string;state:HostGrantV1;plan:HostPlanV1;pid:number;now:()=>number;language:"en"|"de"}>):Promise<void>{
 const {protocol,readJson,writeJson,remove}=ledger,churn=await import("../../../../../📁️filesystem/🧾️observation/🔁️churn/🟦️.ts"),{lentSinceMs:_,...base}=input.state,policy=input.options.policy,write=input.options.write??(line=>process.stderr.write(line+"\n"));
 writeJson(input.path,base);
 let live=true,unregister=()=>{};
 const touch=setInterval(()=>{try{utimesSync(input.path,new Date(),new Date());}catch{}},policy.wait.heartbeatMilliseconds),release=():void=>{
  if(!live)return;live=false;clearInterval(touch);clearTimeout(expiry);process.removeListener("exit",release);unregister();
  const row=readJson(input.path) as {pid?:unknown}|undefined;if(row===undefined||row.pid===input.pid)remove(input.path);
 },expiry=setTimeout(release,protocol.parkMaximumMilliseconds);
 touch.unref();expiry.unref();process.once("exit",release);
 unregister=churn.registerRestartCustodyV1({name:"host-capacity",settle:release,offer:async()=>{
  if(!live)return{};
  const started=await(input.options.self??selfStartedV1)().catch(()=>undefined);if(started===undefined)return{};
  const offer:HostHandoverV1={token:randomBytes(16).toString("base64url"),fromPid:input.pid,fromStarted:started,offeredAtMs:input.now()};
  writeJson(input.path,{...base,handover:offer});
  write(formatHostCapacityV1(input.language,"handover",{operation:base.operation,jobs:base.jobs,reserved:base.reservedMegabytes}));
  return{[protocol.handoverEnvironment]:`${base.id}.${offer.token}`,[protocol.planEnvironment]:JSON.stringify(input.plan)};
 }});
}

