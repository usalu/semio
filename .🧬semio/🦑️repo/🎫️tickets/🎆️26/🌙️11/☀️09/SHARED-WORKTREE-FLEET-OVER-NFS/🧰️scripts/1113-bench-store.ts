import {createRequire} from "node:module";
import {createHash,randomBytes} from "node:crypto";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync,appendFileSync,statSync,openSync,writeSync,closeSync,utimesSync,existsSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
// usage: bun 1113-bench-store.ts <tree-root> <scenario...>   scenarios: idle5 idle50 publish1000 big journal catchup
// <tree-root> is a temp tree that holds 🧰️framework/.../🤝️shared-worktree (the module under test).
const tree=process.argv[2]!,scenarios=process.argv.slice(3);
const module_=join(tree,"🧰️framework","🛍️products","🦑️repo","🔨️modules","🤝️shared-worktree");
const req=createRequire(import.meta.url),fsp=req("node:fs/promises");
let storeRoot="",counts:Record<string,number>={},fsyncs=0,active=false;
const ops=["stat","lstat","readFile","readdir","open","rename","unlink","mkdir","copyFile","utimes","chmod","writeFile","rmdir"];
for(const op of ops){const original=fsp[op];fsp[op]=(...args:unknown[])=>{
 if(active&&typeof args[0]==="string"&&args[0].startsWith(storeRoot))counts[op]=(counts[op]??0)+1;
 const result=original(...args);
 if(op==="open"&&active&&typeof args[0]==="string"&&args[0].startsWith(storeRoot))return (result as Promise<{sync:()=>Promise<void>}>).then(handle=>{const sync=handle.sync.bind(handle);handle.sync=()=>{fsyncs++;return sync();};return handle;});
 return result;};}
const journal=await import(join(module_,"📒️journal","🟦️.ts")),replica=await import(join(module_,"🔁️replica","🟦️.ts"));
const temp=(label:string)=>mkdtempSync(join(tmpdir(),`bench-${label}-`));
const total=()=>Object.values(counts).reduce((a,b)=>a+b,0);
const mb=(bytes:number)=>Math.round(bytes/1048576);
const rss=()=>mb(process.memoryUsage().rss);
const report=(name:string,data:Record<string,unknown>)=>console.log(JSON.stringify({scenario:name,...data}));
const options=(root:string,name:string,store:string)=>({worktree:root,storeRoot:store,originId:name,groupId:"g1",pollMilliseconds:250,scanSeconds:900,debounceMilliseconds:5,persistMilliseconds:0});
const sha=(text:string|Buffer)=>createHash("sha256").update(text).digest("hex");
async function machine(name:string,store:string,first:boolean){
 const worktree=temp(name);writeFileSync(join(worktree,".gitignore"),"ignored\n");
 const o=options(worktree,name,store);
 if(first)await replica.seedV1(o);else await replica.joinV1(o);
 return {o,replica:await replica.openReplicaV1(o)};
}
async function writeOrigins(store:string,count:number){
 const blob=temp("blob"),file=join(blob,"x");writeFileSync(file,"x");
 for(let index=0;index<count;index++){
  const origin=`o${String(index).padStart(3,"0")}`,hlc=journal.formatHlcV1({ms:Date.now()-5000,counter:index},origin);
  await journal.writeBlobV1(store,origin,sha("x"),file);
  await journal.appendEventsV1(store,origin,[{version:1,hlc,operation:"put",path:`${origin}.txt`,sha256:sha("x"),bytes:1,executable:false,base:null}]);
 }
 rmSync(blob,{recursive:true,force:true});
}

async function idle(count:number){
 const store=temp("store");storeRoot=store;
 const me=await machine("me",store,true);await writeOrigins(store,count-1);
 for(let round=0;round<3;round++)await replica.followOnceV1(me.replica);
 const rounds=48,times:number[]=[];
 counts={};fsyncs=0;active=true;
 for(let round=0;round<rounds;round++){const start=performance.now();await replica.followOnceV1(me.replica);times.push(performance.now()-start);await Bun.sleep(250);}
 active=false;
 times.sort((a,b)=>a-b);
 report(`idle${count}`,{origins:count,rounds,storeOpsPerRound:+(total()/rounds).toFixed(2),storeOpsPerSecond:+(total()/rounds*4).toFixed(1),byOp:counts,medianRoundMs:+times[rounds>>1]!.toFixed(3)});
 await replica.closeReplicaV1(me.replica);
}

async function publish1000(){
 const store=temp("store");storeRoot=store;
 const me=await machine("me",store,true);
 for(let index=0;index<1000;index++){const dir=join(me.o.worktree,`d${index%20}`);mkdirSync(dir,{recursive:true});writeFileSync(join(dir,`f${index}.txt`),`${index} ${randomBytes(512).toString("hex")}`);}
 counts={};fsyncs=0;active=true;
 const start=performance.now(),result=await replica.publishOnceV1(me.replica);
 const ms=performance.now()-start;active=false;
 report("publish1000",{files:result.published,ms:Math.round(ms),msPerFile:+(ms/1000).toFixed(3),storeOps:total(),storeOpsPerFile:+(total()/1000).toFixed(2),fsyncs,fsyncsPerFile:+(fsyncs/1000).toFixed(3),byOp:counts});
 await replica.closeReplicaV1(me.replica);
}

async function big(){
 const gib=1<<30,store=temp("store");storeRoot=store;
 const me=await machine("me",store,true),you=await machine("you",store,false);
 const file=join(me.o.worktree,"big.bin"),fd=openSync(file,"w"),chunk=randomBytes(1<<20);
 for(let index=0;index<1024;index++){chunk.writeUInt32LE(index,0);writeSync(fd,chunk);}
 closeSync(fd);utimesSync(file,1_700_000_000,1_700_000_000);
 let peak=rss();const timer=setInterval(()=>{peak=Math.max(peak,rss());},50);
 counts={};fsyncs=0;active=true;
 let start=performance.now();const published=await replica.publishOnceV1(me.replica);const publishMs=performance.now()-start;
 start=performance.now();const followed=await replica.followOnceV1(you.replica);const applyMs=performance.now()-start;
 active=false;clearInterval(timer);
 const equal=statSync(join(you.o.worktree,"big.bin")).size===gib;
 report("big",{bytes:gib,published:published.published,publishMs:Math.round(publishMs),applied:followed.applied,applyMs:Math.round(applyMs),applied_ok:equal,peakRssMb:peak,fsyncs,byOp:counts});
 for(const mode of [false,true]){
  const target=join(temp("copy"),"copy");start=performance.now();
  if(journal.copyHashedV1){const result=await journal.copyHashedV1(file,target,{clone:mode});report("big-copy",{clone:mode,ms:Math.round(performance.now()-start),stable:result.stable});}
  else{const copied=await journal.writeBlobV1(store,"me",sha("never"),file).catch(()=>false);report("big-copy",{note:"old api has no copy primitive",ms:Math.round(performance.now()-start),copied});}
 }
 await replica.closeReplicaV1(me.replica);await replica.closeReplicaV1(you.replica);
}

function writeJournal(store:string,origin:string,count:number,blobs:string[],base:number){
 const directory=join(store,"v1","origins",origin,"segments");mkdirSync(directory,{recursive:true});
 let segment=0,inSegment=0,bytes=0,lines:string[]=[],last="";
 const flush=()=>{if(lines.length===0)return;const text=lines.join("");appendFileSync(join(directory,`${String(segment).padStart(12,"0")}.jsonl`),text);bytes+=Buffer.byteLength(text);lines=[];};
 for(let index=0;index<count;index++){
  if(inSegment===4096){flush();segment++;inSegment=0;bytes=0;}
  last=journal.formatHlcV1({ms:base+Math.floor(index/1000),counter:index%1000},origin);
  const sha_=blobs[index%blobs.length]!;
  lines.push(`${JSON.stringify({version:1,hlc:last,operation:"put",path:`dir${index%200}/sub${index%37}/file-${index}.txt`,sha256:sha_,bytes:1,executable:false,base:null})}\n`);inSegment++;
 }
 flush();
 const head={version:1,segment,bytes,events:inSegment,hlc:last};
 writeFileSync(join(store,"v1","origins",origin,"head.json"),JSON.stringify(head));
 if(journal.publishFlatHeadV1)return journal.publishFlatHeadV1(store,origin,head);
}
async function blobs(store:string,count:number):Promise<string[]>{
 const dir=temp("blobs"),shas:string[]=[];
 for(let index=0;index<count;index++){const file=join(dir,`b${index}`),text=`blob ${index}\n`;writeFileSync(file,text);await journal.writeBlobV1(store,"seed",sha(text),file);shas.push(sha(text));}
 rmSync(dir,{recursive:true,force:true});return shas;
}

async function journalRead(){
 const store=temp("store");storeRoot=store;mkdirSync(join(store,"v1","blobs"),{recursive:true});
 const shas=await blobs(store,20);await writeJournal(store,"big",272_000,shas,Date.now()-600_000);
 const before=rss(),start=performance.now(),read=await journal.readEventsV1(store,"big",undefined),ms=performance.now()-start;
 report("journal",{events:read.events.length,ms:Math.round(ms),eventsPerSecond:Math.round(read.events.length/(ms/1000)),rssMb:rss(),rssBeforeMb:before});
}

async function catchup(){
 const store=temp("store");storeRoot=store;
 const me=await machine("me",store,true),shas=await blobs(store,20);
 await writeJournal(store,"big",100_000,shas,Date.now()-600_000);
 const durations:number[]=[];let peak=rss(),events=0,applied=0;
 const timer=setInterval(()=>{peak=Math.max(peak,rss());},50);
 const begin=performance.now();
 for(let guard=0;guard<2000;guard++){
  const start=performance.now(),result=await replica.followOnceV1(me.replica);
  durations.push(performance.now()-start);events+=result.events;applied+=result.applied;
  if(result.events===0&&result.queued===0)break;
 }
 clearInterval(timer);
 const sorted=[...durations].sort((a,b)=>a-b);
 report("catchup",{events,applied,calls:durations.length,totalMs:Math.round(performance.now()-begin),maxCallMs:Math.round(sorted[sorted.length-1]!),medianCallMs:Math.round(sorted[sorted.length>>1]!),peakRssMb:peak});
 await replica.closeReplicaV1(me.replica);
}

for(const scenario of scenarios){
 if(scenario==="idle5")await idle(5);
 else if(scenario==="idle50")await idle(50);
 else if(scenario==="publish1000")await publish1000();
 else if(scenario==="big")await big();
 else if(scenario==="journal")await journalRead();
 else if(scenario==="catchup")await catchup();
}
