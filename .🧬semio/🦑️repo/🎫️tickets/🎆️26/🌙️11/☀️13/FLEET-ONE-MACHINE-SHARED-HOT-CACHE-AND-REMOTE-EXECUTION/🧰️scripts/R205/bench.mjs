import {mkdtempSync,mkdirSync,rmSync,readFileSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {randomBytes,createHash} from "node:crypto";
const R="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const U=R+"⚡️caching/🦀️cargo/🗃️unit-cache/";
const scratch=mkdtempSync("/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R205/b-");
process.env.SEMIO_CAS_ROOT=join(scratch,"cas");
for(const k of Object.keys(process.env))if(k.startsWith("SEMIO_REMOTE_CACHE_")||k.startsWith("SEMIO_FLEET_"))delete process.env[k];
const {startRemoteCacheServerV1}=await import(R+"🏗️build/🌐️remote-cache/🖥️server/🟦️.ts");
const {createFleetClientV1,readFleetSettingsV1}=await import(U+"🛰️fleet/📡️client/🟦️.ts");
const {installFleetUnitV1}=await import(U+"🛰️fleet/📥️adopt/🟦️.ts");
const {fetchFleetUnitV1}=await import(U+"🛰️fleet/🟦️.ts");
const {packUnitV1}=await import(U+"🛰️fleet/📦️wire/🟦️.ts");
const N=Number(process.env.N??120),W=Number(process.env.W??4),LAT=Number(process.env.LAT??0),SCALE=Number(process.env.SCALE??0.25);
const server=await startRemoteCacheServerV1({directory:join(scratch,"srv"),host:"127.0.0.1",port:0,maximumBytes:2**31,maximumEntryBytes:2**28,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"w"}});
const env={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w"};
const calls={};let latency=0;
const counting=async(url,init)=>{const m=(init?.method??"GET"),p=new URL(url).pathname;const kind=m+" "+(p.includes("/cas/")?"cas":p.includes("/ac/")?"ac":p.includes("slots")?"slots":"other");calls[kind]=(calls[kind]??0)+1;if(latency)await new Promise(r=>setTimeout(r,latency));return fetch(url,init);};
const settings=readFleetSettingsV1(join(scratch,"u"),env);
const client=createFleetClientV1(settings,{fetch:counting});
const platform={version:1,os:"darwin",arch:"arm64",host:"aarch64-apple-darwin",rustc:"a".repeat(64),toolchain:"t",linker:"d",cpu:""};
const sizes=[1184165,10214479,54841222,182286596];
const rnd=(i)=>{const x=Math.sin(i*12.9898)*43758.5453;return x-Math.floor(x);};
const units=[];let total=0;
for(let i=0;i<N;i++){
 const u=rnd(i);const base=u<.5?sizes[0]*(0.2+u):u<.9?sizes[1]*(0.1+u/2):u<.99?sizes[2]*(u-.5):sizes[3]*0.5;
 const rlib=Math.max(20000,Math.min(Math.floor(base*SCALE),24e6)),rmeta=Math.max(5000,Math.floor(rlib/6));
 const files=new Map();const mk=(n)=>{const b=randomBytes(Math.min(n,4096));const buf=Buffer.alloc(n);for(let o=0;o<n;o+=b.length)b.copy(buf,o);buf.writeUInt32LE(i*7919+n,0);const h=createHash("sha256").update(buf).digest("hex");files.set(h,buf);return {h,n};};
 const a=mk(rlib),b=mk(rmeta),d=mk(300);total+=rlib+rmeta+300;
 const key=createHash("sha256").update("k"+i).digest("hex"),slot=createHash("sha256").update("s"+i).digest("hex"),local=createHash("sha256").update("l"+i).digest("hex");
 const manifest={version:1,key,localKey:local,slot,crate:"c"+i,platform,createdMs:1,costMs:1,dependencies:[],environment:[],outputs:[{name:`libc${i}.rlib`,sha256:a.h,bytes:a.n,executable:false},{name:`libc${i}.rmeta`,sha256:b.h,bytes:b.n,executable:false},{name:`c${i}.d`,sha256:d.h,bytes:d.n,executable:false,template:true}],stdout:null,stderr:null};
 units.push({key,slot,local,manifest,files});
}
for(const u of units){
 // the .d output is a template: its bytes must be valid utf-8 text
 const d=u.manifest.outputs[2];const text=Buffer.from("deps: x.rs\n");const h=createHash("sha256").update(text).digest("hex");u.files.delete(d.sha256);u.files.set(h,text);d.sha256=h;d.bytes=text.length;
 const pack=packUnitV1(u.manifest,u.files);
 for(const [h,b] of pack.blobs)await client.putBlob(h,b);
 const r=await client.putResult(u.key,pack.result,u.slot);if(r!==true)throw Error("put "+r);
}
for(const k of Object.keys(calls))delete calls[k];
latency=LAT;
const root=join(scratch,"u");mkdirSync(root,{recursive:true});
const t0=performance.now();let next=0,hits=0;
const mode=process.env.MODE??"fetch";
await Promise.all(Array.from({length:W},async()=>{for(;;){const i=next++;if(i>=units.length)return;const u=units[i];
 const target={client,root,localKey:u.local,key:u.key,slot:u.slot,platform,roots:[],unit:undefined};
 const r=mode==="install"?await installFleetUnitV1(target):await fetchFleetUnitV1({...target,workspace:undefined});
 if(r==="hit")hits++;}}));
const wall=(performance.now()-t0)/1000;
console.log(JSON.stringify({N,W,LAT,mode,hits,MB:+(total/1e6).toFixed(0),wallS:+wall.toFixed(2),MBps:+(total/1e6/wall).toFixed(1),perUnitMs:+(wall*1000*W/N).toFixed(0),calls}));
await server.stop();rmSync(scratch,{recursive:true,force:true});
