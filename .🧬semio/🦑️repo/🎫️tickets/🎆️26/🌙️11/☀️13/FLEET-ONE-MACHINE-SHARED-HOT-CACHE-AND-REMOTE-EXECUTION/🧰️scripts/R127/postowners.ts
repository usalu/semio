import {createHash} from "node:crypto";
import {mkdirSync,rmSync,statSync,readdirSync} from "node:fs";
import {join} from "node:path";
const L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph";
const world=await import(process.env.R127_STAGE?process.env.R127_STAGE+"/world.ts":L+"/🕰️generations/🧪️tests/🏃️world/🟦️.ts");
const load=await import(process.env.R127_STAGE?process.env.R127_STAGE+"/load.mjs":L+"/🕰️generations/🔌️load/🟨️.mjs");
const rowlog=await import(L+"/🕰️generations/🗃️row-log/🟨️.mjs");
const N=Number(process.env.R127_OWNERS??4774),NODE=Number(process.env.R127_NODE_BYTES??22000),ROWS=Number(process.env.R127_ROWS??300),CAP=Number(process.env.R127_CAP_MB??0),WORK=process.env.R127_WORK!;
const profile=await import(L+"/../⏱️profile/🟨️.mjs");const perPhase:Record<string,{peak:number;first:number;last:number}>={};
const sha=(text:string)=>createHash("sha256").update(text).digest("hex");
const started=performance.now();let peak=0;const marks:[string,number,number][]=[];
const sample=()=>{const rss=Math.round(process.memoryUsage().rss/1048576);if(rss>peak)peak=rss;{const name=profile.profileCurrentPhaseV1()??"-",row=perPhase[name]??=({peak:0,first:Math.round(performance.now()-started),last:0});row.peak=Math.max(row.peak,rss);row.last=Math.round(performance.now()-started);}if(CAP&&rss>CAP){console.log(JSON.stringify({capped:true,rssMb:rss,atS:Math.round((performance.now()-started)/1000),last:marks.at(-1)}));process.exit(137);}};
setInterval(sample,50).unref();
const mark=(name:string)=>{sample();marks.push([name,Math.round(performance.now()-started),peak]);};
rmSync(WORK,{recursive:true,force:true});mkdirSync(join(WORK,"pkg"),{recursive:true});
const names=Array.from({length:N},(_,i)=>"p"+String(i).padStart(5,"0"));
const rootOf=(name:string)=>"pkg/"+name;
const entries=(count:number,salt:string)=>Array.from({length:count},(_,i)=>"{projectRoot}/src/"+salt+"/"+sha(salt+i).slice(0,24)+"/**/*.ts");
const nodeOf=(name:string)=>{
 const inputsBytes=Math.floor(NODE*0.72),perTarget=Math.floor(inputsBytes/4),targets:Record<string,unknown>={};
 for(const target of ["build","test","lint","typecheck"])targets[target]={execution:{kind:"command",command:"repo run "+name+":"+target},inputs:entries(Math.floor(perTarget/56),name+target),dependsOn:["^build"],outputs:["{projectRoot}/dist"],cache:true};
 const rest=NODE-inputsBytes;
 return {name,root:rootOf(name),projectType:"library",sourceRoot:rootOf(name)+"/src",targets,metadata:{semio:{description:"x".repeat(Math.max(0,rest-600)),tags:["a","b"]}},tags:["lib"]};
};
const dir=join(WORK,"ws");mkdirSync(dir,{recursive:true});
const mock=world.createMockHost(dir,{});
const host=mock.host as any;
if(process.env.R127_REAL_READ==="1"){const contract=await import(L+"/📜️contract/🟦️.ts");host.readGraph=contract.readBuildGraphV1;}
const EXPAND=Number(process.env.R127_EXPAND_PER_TARGET??0);
if(EXPAND>0){const pool=Array.from({length:60000},(_,i)=>"{workspaceRoot}/pkg/shared"+(i%700)+"/src/f"+i+".ts");
 host.expand=async(chunk:Record<string,any>)=>{for(const[name,project]of Object.entries(chunk)){const at=Number(name.slice(1));for(const[tn,target]of Object.entries<any>(project.targets)){const list:string[]=[];for(let k=0;k<EXPAND;k++)list.push(pool[(at*31+k*17+tn.length)%pool.length]!);target.inputs=[...target.inputs,...list];}}return chunk;};}
const owners0=names.map(name=>({name,root:rootOf(name),package:"@syn/"+name,uses:[],broken:false}));
const envParts=world.environmentParts(owners0),envDigest=world.environmentDigest(owners0);
host.discoverPart=async(spec:any,signal:any,control:any,ownerDone:any)=>{
 mark("compute-begin");
 const log=rowlog.createRowLogV1({path:join(WORK,"rl.rows")});
 const part:any={owners:[],edgeOnly:[],carried:[],edges:[],externalNodes:{},parseUnavailable:[],rows:[],env:{digest:envDigest,rows:[],parts:envParts},replaceTags:[],derived:[],phases:{}};
 const wanted=spec.cold?null:new Set<string>(spec.owners);
 for(let i=0;i<N;i++){
  if(wanted&&!wanted.has(names[i]!))continue;
  const name=names[i]!,root=rootOf(name),rows=Array.from({length:ROWS},(_,k)=>({kind:"file",path:"/Users/ueli/Documents/semio/"+root+"/src/f"+k+".ts",fact:{size:1000+k,mtimeMs:1.7e12+k,ctimeMs:1.7e12+k,ino:i*1000+k,sha256:sha(name+k)}}));
  part.owners.push({name,root,config:root+"/project.json",node:nodeOf(name),nodeRows:log.append(rows as any)});
  for(let e=1;e<=8;e++)part.edges.push({source:name,sourceFile:root+"/src/f0.ts",target:names[(i+e*37)%N],type:"static"});
  ownerDone?.();
  if(i%200===199)(globalThis as any).Bun?.gc(true);
 }
 (globalThis as any).Bun?.gc(true);mark("owners-made");
 part.edges=part.edges;part.rows=[...(function*(){for(const owner of part.owners)yield* owner.nodeRows;})()];
 mark("rows-spread");
 part.dispose=()=>log.close();
 return part;
};
let phase=-1;
const origClock=host.clock;
host.clock=(control:any)=>{const clock=origClock(control);return {...clock,begin:(p:string,t:number)=>{mark("begin:"+p);return clock.begin(p,t);},end:async(p:string,d?:any)=>{mark("end:"+p);return clock.end(p,d);}};};
const out:any={owners:N,nodeBytes:NODE,rowsPerOwner:ROWS,capMb:CAP||null};
try{
 const result=await load.loadGenerationGraphV1(host,{});
 mark("load-done");
 const g=result.graph;
 out.projects=Object.keys(g.projects).length;
 const K=Number(process.env.R127_DELTA??0);
 if(K>0){
  out.first={peakRssMb:peak,seconds:Math.round((performance.now()-started)/100)/10};
  (globalThis as any).Bun?.gc(true);
  const dirty=new Set(names.filter((_,i)=>i%Math.floor(N/K)===0).slice(0,K));
  const baseStore=host.storeOptions.bind(host);
  host.storeOptions=(store:any,control:any)=>{const o=baseStore(store,control);return {...o,holds:async(rows:any[])=>rows.every(row=>{const m=/pkg\/(p\d+)\//u.exec(row.path);return !(m&&dirty.has(m[1]!));})};};
  const t0=performance.now();peak=0;marks.length=0;const mark0=marks.length;
  const again=await load.loadGenerationGraphV1(host,{});
  out.delta={k:dirty.size,seconds:Math.round((performance.now()-t0)/100)/10,peakRssMb:peak,projects:Object.keys(again.graph.projects).length,stats:again.stats};
 }out.dependencies=g.dependencies.length;out.stats=result.stats;
}catch(error:any){out.error=String(error?.message??error).slice(0,300);}
sample();
const size=(path:string):number=>{let total=0;try{for(const e of readdirSync(path,{withFileTypes:true})){const p=join(path,e.name);total+=e.isDirectory()?size(p):statSync(p).size;}}catch{}return total;};
out.peakRssMb=peak;out.seconds=Math.round((performance.now()-started)/100)/10;out.storeMb=Math.round(size(join(dir,".gen"))/1048576);out.marks=marks;out.perPhase=perPhase;out.said=mock.said.map((s:any)=>s.text).slice(-12);
console.log(JSON.stringify(out));
rmSync(WORK,{recursive:true,force:true});
process.exit(0);
