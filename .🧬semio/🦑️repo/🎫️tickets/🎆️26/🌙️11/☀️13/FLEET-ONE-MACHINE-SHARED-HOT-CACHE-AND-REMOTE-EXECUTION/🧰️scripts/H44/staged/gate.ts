import {randomBytes} from "node:crypto";
import {mkdirSync,readFileSync,renameSync,writeFileSync} from "node:fs";
import {availableParallelism,loadavg} from "node:os";
import {join} from "node:path";
import {offloadGateV1,type OffloadGateReasonV1} from "../../🚦️admission/🟦️.ts";
import {CHECKOUT_TEST_POLICY_V1,checkoutTestProblemV1,type CheckoutTestMeasureV1,type CheckoutTestMeasuresV1,type CheckoutTestModeV1} from "../📐️policy/🟦️.ts";

/** 🧭️ Why a test file runs here or leaves: the reason of the shared offload gate (`host-wait`: the host verdict of this machine is `wait`; `local-slot`: the host admits, the file runs here), `host-load` (the host admits memory-wise but its CPU load per core says a remote run is faster), `never` and `always` of the mode, `no-idle-machine` when no other machine could take it. */
export type CheckoutTestGateReasonV1=OffloadGateReasonV1|"always"|"no-idle-machine"|"host-load";

/** ⚖️ The load verdict for one test file: leave this machine or stay. */
export type CheckoutTestLoadVerdictV1="offload"|"keep";

/** 🔥️ The CPU load of this machine per core: the one-minute load average over the logical cores (`0` where the platform has no load average). */
export function checkoutTestLoadPerCoreV1(load:readonly number[]=loadavg(),cores:number=availableParallelism()):number{
 return Math.round((load[0]??0)/Math.max(1,cores)*100)/100;
}

/** ⚖️ Decides from the CPU load per core and the measured durations of this file whether a remote run pays: at or above `perCoreAlways` the file always leaves; below `perCoreMeasured` it stays; in between it leaves unless both a local and a remote duration are known and fresh (`maximumAgeSeconds`) and the remote one is not shorter than `remoteWorthRatio` of the local one. Pure; the measured durations are the wall seconds the same file took here and on another machine.
 * @see ../🧬️schema/🔣️.json */
export function checkoutTestLoadVerdictV1(input:Readonly<{loadPerCore:number;measure?:CheckoutTestMeasureV1|undefined;now?:number}>):CheckoutTestLoadVerdictV1{
 const rule=CHECKOUT_TEST_POLICY_V1.load;
 if(input.loadPerCore>=rule.perCoreAlways)return"offload";
 if(input.loadPerCore<rule.perCoreMeasured)return"keep";
 const now=input.now??Date.now(),local=input.measure?.local,remote=input.measure?.remote,fresh=(row:{at:number}|undefined):boolean=>row!==undefined&&Math.abs(now-row.at)<=rule.maximumAgeSeconds*1000;
 return local!==undefined&&remote!==undefined&&fresh(local)&&fresh(remote)&&remote.seconds>=local.seconds*rule.remoteWorthRatio?"keep":"offload";
}

/** 📏️ Reads the measured durations of the test files of this machine (`{version:1,files:{}}` when none were recorded or the document violates its schema). */
export function readCheckoutTestMeasuresV1(root:string):CheckoutTestMeasuresV1{
 try{
  const value:unknown=JSON.parse(readFileSync(join(root,CHECKOUT_TEST_POLICY_V1.load.file),"utf8"));
  if(checkoutTestProblemV1("Measures",value)===undefined)return value as CheckoutTestMeasuresV1;
 }catch{}
 return{version:1,files:{}};
}

/** 🖊️ Records how long one file took here (`local`) or through another machine (`remote`, wall seconds including the wait), atomically, keeping the newest `maximumFiles` files; a failure only costs the next verdict its measure. */
export function recordCheckoutTestMeasureV1(root:string,file:string,side:"local"|"remote",seconds:number,now:number=Date.now()):void{
 try{
  const rule=CHECKOUT_TEST_POLICY_V1.load,current=readCheckoutTestMeasuresV1(root).files,files:Record<string,CheckoutTestMeasureV1>={...current,[file]:{...current[file],[side]:{seconds:Math.round(seconds*10)/10,at:now}}},newest=(row:CheckoutTestMeasureV1)=>Math.max(row.local?.at??0,row.remote?.at??0);
  const names=Object.keys(files).sort((left,right)=>newest(files[right]!)-newest(files[left]!)||(left<right?-1:1)).slice(0,rule.maximumFiles),kept:Record<string,CheckoutTestMeasureV1>={};
  for(const name of names.sort())kept[name]=files[name]!;
  mkdirSync(root,{recursive:true});
  const path=join(root,rule.file),temporary=path+"."+randomBytes(3).toString("hex");
  writeFileSync(temporary,JSON.stringify({version:1,files:kept}));renameSync(temporary,path);
 }catch{}
}

/** 🚪️ Whether one test file is offered to an idle machine of the fleet. In `auto` the very gate of the compile units decides first (`offloadGateV1`, the same host-capacity verdict: a free slot keeps the work only while the host does not say `wait`; a test file has one slot, always free, is always worth a round trip and has no slot queue or broker); when the host admits, the CPU load per core and the measured local and remote durations of the file decide (`load`, `host-load`: a host that has memory but is saturated in CPU is where test files time out). `always` offers it whenever the fleet has an idle machine, `never` keeps it. After a positive verdict at least one Ready machine of this platform must exist (`idle`: how many, `null` when the fleet is not asked), else the file stays and its reason is `no-idle-machine`. Asks each signal only when the one before did not decide. */
export async function checkoutTestGateV1(input:Readonly<{mode:CheckoutTestModeV1;host:()=>Promise<"admit"|"wait">;idle:()=>Promise<number|null>;load?:()=>Promise<CheckoutTestLoadVerdictV1>}>):Promise<Readonly<{offload:boolean;reason:CheckoutTestGateReasonV1}>>{
 if(input.mode==="never")return{offload:false,reason:"never"};
 let reason:CheckoutTestGateReasonV1="always";
 if(input.mode==="auto"){
  const gate=await offloadGateV1({mode:"distribute",door:false,expensive:()=>false,workerFits:async()=>false,localSlot:async()=>true,worth:()=>"worth",slotQueueMilliseconds:()=>0,host:input.host,broker:async()=>"unknown"});
  if(gate.offload)reason=gate.reason;
  else if(gate.reason==="local-slot"&&input.load!==undefined&&await input.load()==="offload")reason="host-load";
  else return{offload:false,reason:gate.reason};
 }
 const idle=await input.idle();
 return idle!==null&&idle>0?{offload:true,reason}:{offload:false,reason:"no-idle-machine"};
}
