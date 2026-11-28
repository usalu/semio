import schema from"./🧬️schema/🔣️.json" with { type: "json" };
import{appendNativeImageShardEventV1,nativeImageShardCapBytesV1,nativeImageShardCommittedBytesV1,nativeImageShardFoldV1,nativeImageShardHomeLayoutV1,nativeImageShardLayoutV1,nativeImageShardWantBytesV1,readNativeImageShardLedgerV1,NATIVE_IMAGE_SHARD_V1,type NativeImageShardLayoutV1,type NativeImageShardLifecycleV1,type NativeImageShardRuleV1}from"../🧩️shard/🟦️.ts";
import{nativeImageDetachBusyV1}from"../🔁️detach/🟦️.ts";
const protocol=schema.definitions.Protocol.const,shard=NATIVE_IMAGE_SHARD_V1,align=shard.alignBytes;
type LockV1=<T>(body:()=>Promise<T>)=>Promise<T>;
/** 🪜️ Names the closed protocol of growing an image and sizing a new lane image.
 * @see ./🧬️schema/🔣️.json */
export const NATIVE_IMAGE_GROWTH_V1=protocol;
/** 🧭️ How a platform stretches an image: `online` while it is mounted (a Linux loop device), `detached` only while nothing holds it (macOS, Windows: measured, an attached image refuses with status 35 or `Resource busy`), `unsupported` elsewhere. */
export function nativeImageGrowthModeV1(platform:string):"online"|"detached"|"unsupported"{
 const key=platform==="win32"?"windows":platform;return(protocol.modes as Readonly<Record<string,"online"|"detached">>)[key]??"unsupported";
}
const alignDown=(value:number)=>Math.floor(value/align)*align;
/** 🪞️ What one image does next: nothing, grow to `targetBytes`, `wait` for a quiet moment (a detached platform and a holder), or be `refused`. */
export type NativeImageGrowthPlanV1=Readonly<{action:"none"|"grow"|"wait"|"refused";mode:"online"|"detached"|"unsupported";reason:"unsupported"|"at-cap"|"below-threshold"|"enough"|"host-short"|"budget"|"held"|"ok";targetBytes:number;deltaBytes:number}>;
/** 📈️ Decides the growth of one image from its numbers: it never shrinks (the target is above the capacity, which is at least what is used), never passes the platform cap, only starts when the peak of used and measured bytes reaches `growAtPercent` of the capacity, aims at the peak plus the rule's headroom, and is cut down to what the host can give (a share of its free bytes, the larger envelope plus the initial extent, and `hostKeepBytes` that stay free afterwards, never below the protocol's keep nor above its maximum) and to the room the commitment budget leaves; a part of the wanted growth is still growth. A detached platform whose image is held answers `wait` with the target it will take. Pure and language-agnostic.
 * @see ./🧪️tests/🐍️.py */
export function nativeImageGrowthPlanV1(platform:string,rule:Pick<NativeImageShardRuleV1,"maximumImageBytes"|"headroomPercent"|"hostFreeReservePercent">,facts:Readonly<{capacityBytes:number;usedBytes:number;measuredBytes?:number;hostFreeBytes:number;hostKeepBytes?:number;budgetRoomBytes?:number;held:boolean}>):NativeImageGrowthPlanV1{
 const mode=nativeImageGrowthModeV1(platform),no=(action:NativeImageGrowthPlanV1["action"],reason:NativeImageGrowthPlanV1["reason"],targetBytes=facts.capacityBytes):NativeImageGrowthPlanV1=>Object.freeze({action,mode,reason,targetBytes,deltaBytes:targetBytes-facts.capacityBytes});
 if(mode==="unsupported")return no("refused","unsupported");
 const cap=alignDown(nativeImageShardCapBytesV1(platform,rule));
 if(facts.capacityBytes>=cap)return no("none","at-cap");
 const peak=Math.max(facts.usedBytes,facts.measuredBytes??0);
 if(peak*100<facts.capacityBytes*protocol.growAtPercent)return no("none","below-threshold");
 const wanted=Math.min(cap,nativeImageShardWantBytesV1(rule,peak));
 if(wanted<=facts.capacityBytes)return no("none","enough");
 const keep=Math.min(protocol.hostKeepMaximumBytes,Math.max(protocol.hostKeepBytes,facts.hostKeepBytes??0)),share=Math.floor(facts.hostFreeBytes*rule.hostFreeReservePercent/100),left=facts.hostFreeBytes-keep-protocol.initialBytes;
 const allowed=alignDown(Math.max(0,Math.min(share,left))),budget=facts.budgetRoomBytes===undefined?Infinity:alignDown(Math.max(0,facts.budgetRoomBytes));
 if(allowed<align)return no("refused","host-short");
 if(budget<align)return no("refused","budget");
 const target=Math.min(wanted,facts.capacityBytes+allowed,facts.capacityBytes+budget);
 return facts.held&&mode==="detached"?no("wait","held",target):no("grow","ok",target);
}
/** 🧮️ The sizing of a new lane image from the lanes measured so far: the high percentile of their sizes (at least `floorBytes`) is the least a lane image must hold (the gate that refuses a host with too little room), the preferred size keeps the rule's own headroom around the larger of the declared need and the largest lane, so a host with room is born exactly as before while a small host that can hold the measured need no longer waits for room it does not need. `rule` is the rule to give the birth, `measuredNeedBytes` its preferred need; the rule is the given one when nothing was measured. Pure and language-agnostic.
 * @see ./🧪️tests/🐍️.py */
export function nativeImageLaneSizingV1(rule:NativeImageShardRuleV1,root:number,samples:readonly number[]):Readonly<{rule:NativeImageShardRuleV1;measuredNeedBytes:number|undefined;needBytes:number|undefined}>{
 const sorted=samples.filter(value=>Number.isFinite(value)&&value>0).sort((a,b)=>a-b),declared=rule.roots[root];
 if(sorted.length===0||declared===undefined)return Object.freeze({rule,measuredNeedBytes:undefined,needBytes:undefined});
 const sizing=protocol.laneSizing,needBytes=Math.max(sizing.floorBytes,sorted[Math.max(0,Math.ceil(sorted.length*sizing.percentileBasisPoints/10000)-1)]!);
 const roots=rule.roots.map((row,index)=>index===root?{...row,needBytes}:row),preferred=Math.ceil(Math.max(declared.needBytes,sorted[sorted.length-1]!)*(100+rule.headroomPercent)/(100+sizing.headroomPercent));
 return Object.freeze({rule:Object.freeze({...rule,headroomPercent:sizing.headroomPercent,roots}),measuredNeedBytes:preferred,needBytes});
}
/** 💬️ Renders one closed growth message in one language. */
export function nativeImageGrowthTextV1(language:"en"|"de",key:keyof typeof protocol.messages,values:Readonly<Record<string,string|number>>):string{
 return protocol.messages[key][language].replace(/\{(\w+)\}/gu,(_,name:string)=>String(values[name]??""));
}
type RunV1=(program:string,args:readonly string[])=>Promise<string>;
const fill=(template:readonly string[],values:Readonly<Record<string,string>>):string[]=>template.map(part=>part.replace(/<([A-Za-z]+)>/gu,(_,name)=>values[name]??_));
/** 🐧️ Stretches a mounted image online (Linux): the sparse backing is extended by `extend` (a plain truncate to the new size), the loop device re-reads its capacity and the filesystem grows into it, all while it stays mounted and in use. Never shrinks: a size at or below the current one is refused. The commands are those of the protocol; nothing runs for another platform. */
export async function growNativeImageOnlineV1(given:Readonly<{run:RunV1;image:string;currentBytes:number;targetBytes:number;extend(image:string,bytes:number):void}>):Promise<void>{
 if(given.targetBytes<=given.currentBytes||given.targetBytes%shard.alignBytes!==0)throw Error("Larger aligned image size required");
 const linux=protocol.online.linux,device=(await given.run(linux.findDevice.program,fill(linux.findDevice.arguments,{image:given.image}))).split("\n")[0]?.trim();
 if(!device)throw Error("Loop device of the image required: "+given.image);
 given.extend(given.image,given.targetBytes);
 await given.run(linux.setCapacity.program,fill(linux.setCapacity.arguments,{device}));
 await given.run(linux.growFilesystem.program,fill(linux.growFilesystem.arguments,{device}));
}
/** 🧾️ What one planned growth of a shard answered. */
export type NativeImageShardGrowthOutcomeV1=Readonly<{kind:"grown";capacityBytes:number}|{kind:"none"|"wait"|"refused";reason:NativeImageGrowthPlanV1["reason"]|"busy"}>;
/** 🛗️ Grows the image of one shard by its plan while builds may run: the plan (never below what is used, within the cap, the host keep and the commitment budget) decides, a detached platform takes the exclusive nonwaiting lease of the image first (`lease` answers `undefined` while a holder has it: `wait`, nothing changes), counts the processes that keep files open on it (`holders`: any is `wait` at once, a detach would only retry for minutes) and a busy detach is `wait` as well (the image is mounted again whatever happens), an online platform stretches without a lease; the growth is logged only after it happened. @see ../📈️grow/🟦️.ts */
export async function growNativeImageShardPlannedV1(given:Readonly<{directory:string;rule:NativeImageShardRuleV1;platform:string;id:string;measure(layout:NativeImageShardLayoutV1):Readonly<{usedBytes:number;hostFreeBytes:number}>|undefined;measuredNeedBytes?:()=>number|undefined;budgetBytes?:()=>number|undefined;hostRoomBytes?:()=>number|undefined;hostKeepBytes?:()=>number|undefined;lease?(layout:NativeImageShardLayoutV1):Promise<(()=>void)|undefined>;holders?(layout:NativeImageShardLayoutV1):Promise<number>;online?(layout:NativeImageShardLayoutV1,capacityBytes:number,targetBytes:number):Promise<void>;now:()=>number;lifecycle:NativeImageShardLifecycleV1;lock:LockV1;check?:()=>void}>):Promise<NativeImageShardGrowthOutcomeV1>{
 return given.lock(async()=>{
  const live=nativeImageShardFoldV1(readNativeImageShardLedgerV1(given.directory)),record=live.get(given.id);if(record===undefined)return{kind:"none",reason:"at-cap"}as const;
  const layout=nativeImageShardLayoutV1(given.directory,record),facts=given.measure(layout);if(facts===undefined)return{kind:"none",reason:"below-threshold"}as const;
  const budget=given.budgetBytes?.(),hostRoom=given.hostRoomBytes?.(),room=budget===undefined&&hostRoom===undefined?undefined:Math.min(budget===undefined?Infinity:budget-nativeImageShardCommittedBytesV1(live,record.root),hostRoom??Infinity),measured=given.measuredNeedBytes?.(),keep=given.hostKeepBytes?.();
  const plan=nativeImageGrowthPlanV1(given.platform,given.rule,{capacityBytes:record.capacityBytes,usedBytes:facts.usedBytes,hostFreeBytes:facts.hostFreeBytes,held:false,...measured===undefined?{}:{measuredBytes:measured},...keep===undefined?{}:{hostKeepBytes:keep},...room===undefined?{}:{budgetRoomBytes:room}});
  if(plan.action!=="grow")return{kind:plan.action==="wait"?"wait":plan.action==="none"?"none":"refused",reason:plan.reason}as const;
  let release:(()=>void)|undefined;
  if(plan.mode==="detached"&&given.lease!==undefined){release=await given.lease(layout);if(release===undefined)return{kind:"wait",reason:"held"}as const;}
  try{
   given.check?.();
   if(plan.mode==="detached"&&given.holders!==undefined&&await given.holders(layout)>0)return{kind:"wait",reason:"busy"}as const;
   if(plan.mode==="online"){if(given.online===undefined)return{kind:"refused",reason:"unsupported"}as const;await given.online(layout,record.capacityBytes,plan.targetBytes);}
   else await given.lifecycle.grow(layout,plan.targetBytes,nativeImageShardHomeLayoutV1(given.directory));
  }catch(error){if(nativeImageDetachBusyV1(error))return{kind:"wait",reason:"busy"}as const;throw error;}
  finally{release?.();}
  appendNativeImageShardEventV1(given.directory,{type:"grown",id:given.id,capacityBytes:plan.targetBytes,atMs:given.now()});
  return{kind:"grown",capacityBytes:plan.targetBytes}as const;
 });
}
