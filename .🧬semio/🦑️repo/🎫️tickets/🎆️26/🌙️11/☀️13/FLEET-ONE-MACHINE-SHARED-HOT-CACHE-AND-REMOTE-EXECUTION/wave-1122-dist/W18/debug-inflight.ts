/** 🔬️ W18 debug: replays the in-flight holder entry scenario against the real collector and prints the holder summary. */
import {mkdirSync,mkdtempSync,writeFileSync,existsSync,utimesSync} from "node:fs";
import {join} from "node:path";
import {createHash} from "node:crypto";
const L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗄️cas/";
const {collectCasV1}=await import(L+"🧹️collector/🟦️.ts"),{registerCasHolderSyncV1}=await import(L+"🫂️holders/🟦️.ts"),{createCasStoreV1}=await import(L+"🗃️store/🟦️.ts"),{CAS_POLICY_V1}=await import(L+"🧬️schema/🟦️.ts"),{createCasGuardV1}=await import(L+"🚦️guard/🟦️.ts");
const base=process.argv[2]!,MIB=1048576,sha=(t:string)=>createHash("sha256").update(t).digest("hex");
const root=mkdtempSync(join(base,"s-")),dir=mkdtempSync(join(base,"r-")),clock={value:2_000_000_000};
const policy={...CAS_POLICY_V1,capBytes:MIB,holderProtectedShare:0,minAgeMs:0,areas:[],touchGranularityMs:0,checkEvery:1,progressMs:0},wide={...policy,capBytes:1024*1024*MIB};
const store=createCasStoreV1({root,now:()=>clock.value,policy:wide});
const hashes:string[]=[];for(let i=0;i<7;i++){clock.value=2_000_000_000+i*1000;const b=Buffer.alloc(MIB,i);b.writeUInt32BE(i,0);const h=(await store.put(b)).hash;utimesSync(store.objectPath(h),new Date(1_000_000+i*1000),new Date(1_000_000+i*1000));hashes.push(h);}
const entry=(name:string,d:string[])=>{const key=sha(name),p=join(dir,"e",key.slice(0,2),key);mkdirSync(p,{recursive:true});const f=join(p,"000000000000001-aaaaaa.json");writeFileSync(f,JSON.stringify({version:1,key,outputs:d.map((x,i)=>({name:"o"+i,sha256:x,bytes:MIB,executable:false})),stderr:null,stdout:null,createdMs:1}));return f;};
const old=[entry("e0",[hashes[0]!,hashes[1]!]),entry("e1",[hashes[2]!,hashes[3]!]),entry("e2",[hashes[4]!,hashes[5]!])];
(await import("node:fs")).linkSync(store.objectPath(hashes[4]!),join(dir,"workspace-link"));registerCasHolderSyncV1(root,policy,{kind:"unit-cache",root:dir,indexes:[{directory:join(dir,"e"),references:["outputs.*.sha256","stdout","stderr"]}],evictionLog:join(dir,"y","ev.jsonl")},clock.value);clock.value+=60_000;
let fresh:any;const stages:string[]=[];
const result=await collectCasV1({root,policy,now:()=>clock.value,progress:async (e:any)=>{stages.push(e.stage);if(e.stage!=="scan"||fresh)return;const b=Buffer.alloc(MIB,7);b.writeUInt32BE(9999,0);const h=(await store.put(b)).hash;fresh={h,f:entry("fresh",[h])};}});
console.log(JSON.stringify({status:result.status,totals:result.totals,holders:result.holders,byReason:result.byReason,stages:[...new Set(stages)],fresh:fresh&&existsSync(fresh.f),old:old.map(existsSync)}));
