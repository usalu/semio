import{machinePathV1}from"../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import {appendFileSync,mkdirSync,readdirSync,readFileSync,renameSync,statSync,writeFileSync} from "node:fs";
import {join} from "node:path";

/** 📊️ The offload counters of this machine as the load reporter reads them: units handed to another machine, units the fleet refused for lack of headroom, units that fell back to a local compile after a fault. */
export type OffloadCountersV1=Readonly<{out:number;refused:number;fellBack:number}>;
const KINDS=["out","refused","fellBack"] as const;
const directory=(root:string)=>machinePathV1(root,".cache","f","offload-counters");

/** 🔖️ Why units did not leave: the count of every decline or fall-back reason (`no-idle-machine`, `worker-policy`, `incomplete-inputs`, `infrastructure`, ...), one file per reason below `.cache/f/offload-counters/reason/`, so the sum of `refused` and `fellBack` can be told apart by cause. */
export function readOffloadReasonCountersV1(root:string):Readonly<Record<string,number>>{
 const found:Record<string,number>={};
 try{for(const name of readdirSync(join(directory(root),"reason")))try{found[name]=statSync(join(directory(root),"reason",name)).size;}catch{}}catch{}
 return Object.freeze(found);
}

/** 🔢️ The counts as the sizes of one file per kind (a byte per event, appended: POSIX makes that atomic across processes, so nothing is lost and no lock is taken). */
export function readOffloadCountersV1(root:string):OffloadCountersV1{
 const size=(name:string)=>{try{return statSync(join(directory(root),name)).size;}catch{return 0;}};
 return Object.freeze({out:size("out"),refused:size("refused"),fellBack:size("fellBack")});
}

/** ➕️ Counts events and mirrors the absolute counts into the `offload` object of `.cache/f/build-counters.json`, keeping every other field of that file (the unit cache counters). The file is written atomically from the monotonic count files, so a concurrent writer of the same file at worst shows a stale count until the next event. Never throws. */
export function bumpOffloadCountersV1(root:string,add:Partial<OffloadCountersV1>,reason?:string):void{
 try{
  const base=directory(root);
  if(reason!==undefined&&/^[a-z][a-z0-9-]{0,40}$/u.test(reason)){try{appendFileSync(join(base,"reason",reason),"1");}catch{mkdirSync(join(base,"reason"),{recursive:true});appendFileSync(join(base,"reason",reason),"1");}}
  if(KINDS.every(kind=>(add[kind]??0)<1))return;
  for(const kind of KINDS){
   const count=add[kind]??0;if(count<1)continue;
   try{appendFileSync(join(base,kind),"1".repeat(count));}catch{mkdirSync(base,{recursive:true});appendFileSync(join(base,kind),"1".repeat(count));}
  }
  const path=machinePathV1(root,".cache","f","build-counters.json");let value:Record<string,unknown>={version:1};
  try{value=JSON.parse(readFileSync(path,"utf8")) as Record<string,unknown>;}catch{}
  const temporary=path+"."+process.pid+"."+Math.random().toString(16).slice(2,8);
  writeFileSync(temporary,JSON.stringify({...value,version:1,unit:value.unit??{localHit:0,remoteHit:0,miss:0,stored:0},offload:readOffloadCountersV1(root)})+"\n");renameSync(temporary,path);
 }catch{}
}
