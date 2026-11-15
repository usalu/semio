// 🧪️ Deterministic offload proof: a standalone cargo workspace outside the worktree, compiled by real cargo through the repository's rustc wrapper (`unitCacheEnvironmentV1`, the environment a launcher gives cargo), first on this machine alone, then with offload forced on, then with `auto` under simulated local pressure; per unit where it ran, round trip and bytes; the offloaded outputs are compared byte for byte with a local compile of the same units.
// usage: bun proof.ts --mode scratch|live --scratch <dir> [--code live|stage --stage <stage root>] [--agents 3] [--layers 4 --width 6 --iterations 700000] [--phases always,auto] [--jobs 6] [--out <file.json>]
import {copyFileSync,cpSync,existsSync,mkdirSync,readFileSync,readdirSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {createHash} from "node:crypto";
import {createServer} from "node:net";
import {join,resolve} from "node:path";
import {spawn,spawnSync} from "node:child_process";

const LIVE="/Users/Shared/semio",LIB=`${LIVE}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`,BUILD=`${LIB}/🏗️build`;
const arg=(name:string,fallback?:string):string|undefined=>{const index=process.argv.indexOf("--"+name);return index>=0?process.argv[index+1]:fallback;};
const mode=arg("mode","scratch")!,scratch=resolve(arg("scratch")!),codeMode=arg("code","live")!,stageRoot=arg("stage"),agentsWanted=Number(arg("agents","3")),jobs=Number(arg("jobs","6"));
const layers=arg("layers","4")!,width=arg("width","6")!,iterations=arg("iterations","700000")!,phases=(arg("phases","always,auto")!).split(",").filter(Boolean);
const outFile=arg("out",join(scratch,"proof-result.json"))!,helper=join(import.meta.dir,"mkworkspace.py");
mkdirSync(scratch,{recursive:true});
if(realpathSync(scratch).startsWith(LIVE+"/"))throw Error("scratch must be outside the worktree");
const codeBase=codeMode==="stage"?stageRoot!:LIVE;
const sidecar=`${codeBase}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🟦️.ts`;
const script=`${codeBase}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts`;
const bun=process.execPath,TOOLCHAIN="nightly-2026-07-20";
const log=(text:string):void=>{process.stderr.write(`[proof ${new Date().toISOString().slice(11,19)}] ${text}\n`);};
const sha=(path:string):string=>createHash("sha256").update(readFileSync(path)).digest("hex");

const {startRemoteCacheServerV1}=await import(`${BUILD}/🌐️remote-cache/🖥️server/🟦️.ts`);
const {publishFleetCacheFactsV1}=await import(`${BUILD}/🌐️remote-cache/🛰️fleet/📣️publication/🟦️.ts`);
const {syncFleetIdentityV1}=await import(`${BUILD}/🌐️remote-cache/🛰️fleet/🪪️identity/🟦️.ts`);
const {refreshFleetCacheFactsV1,remoteCacheFetchV1}=await import(`${BUILD}/🌐️remote-cache/🛰️fleet/🟦️.ts`);
const {writeHostViewV1,eventuallyV1}=await import(`${BUILD}/🌐️remote-execution/🧪️tests/🧰️cluster/🟦️.ts`);
const launch=await import(`${LIB}/⚡️caching/🦀️cargo/🗃️unit-cache/🚪️launch/🟦️.ts`);
const drain=await import(`${LIB}/⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/📤️uploader/🟦️.ts`);

const store=join(scratch,"store"),children:ReturnType<typeof Bun.spawn>[]=[],NETWORK="127.0.0.0/8";
let cache:any,ports:number[]=[];
const freePort=async():Promise<number>=>{for(;;){const port=21000+Math.floor(Math.random()*20000);if(await new Promise<boolean>(done=>{const server=createServer();server.once("error",()=>{done(false);});server.listen(port,"127.0.0.1",()=>{server.close(()=>{done(true);});});}))return port;}};
const rustEnv=(extra:Record<string,string|undefined>={}):Record<string,string>=>Object.fromEntries(Object.entries({...process.env,RUSTUP_TOOLCHAIN:TOOLCHAIN,...extra}).filter((row):row is [string,string]=>row[1]!==undefined));
const rustc=spawnSync("rustup",["which","rustc","--toolchain",TOOLCHAIN],{encoding:"utf8"}).stdout.trim(),versionText=spawnSync(rustc,["-vV"],{encoding:"utf8"}).stdout;
if(!rustc||!versionText.includes("host:"))throw Error("the pinned toolchain "+TOOLCHAIN+" is not installed");

async function joinFleet(root:string,origin:string):Promise<string>{
 mkdirSync(join(root,".cache","f"),{recursive:true});mkdirSync(join(store,"v1","origins",origin),{recursive:true});
 writeFileSync(join(root,".cache","f","machine.json"),JSON.stringify({version:1,originId:origin,groupId:"g1",storeRoot:store,key:"0".repeat(32)}));
 const synced=await syncFleetIdentityV1({root});if(synced.error!==undefined)throw Error(synced.error);
 return root;
}
const factsPath=join(store,"v1","fleet","cache.json"),facts=():any=>{try{return JSON.parse(readFileSync(factsPath,"utf8"));}catch{return undefined;}};
const logs=join(scratch,"logs");mkdirSync(logs,{recursive:true});
function startRole(role:"scheduler"|"agent",environment:Record<string,string>,label:string){
 const child=Bun.spawn([bun,sidecar,role],{stdout:"ignore",stderr:Bun.file(join(logs,`${label}.log`)),cwd:scratch,env:{...environment,SEMIO_FLEET_PARENT_PID:String(process.pid)}});children.push(child);return child;
}
async function stopAll():Promise<void>{for(const child of children)try{child.kill("SIGTERM");}catch{}await Bun.sleep(500);for(const child of children)try{child.kill("SIGKILL");}catch{}await cache?.stop().catch(()=>{});}

let requesterRoot="",requesterOrigin="";const agentOrigins:string[]=[];
const schedulerApi=():string=>{
 const facts_=JSON.parse(readFileSync(join(requesterRoot,".cache","f","cache-facts.json"),"utf8")),exec=new URL(facts_.facts?.execution?.endpoint??facts_.execution?.endpoint??facts()?.execution?.endpoint);
 return `http://${exec.hostname}:${mode==="scratch"?ports[1]:8981}`;
};
const fleetTable=async():Promise<any>=>{
 const signed=remoteCacheFetchV1({SEMIO_WORKSPACE_ROOT:requesterRoot},{root:requesterRoot,audience:"execution",roleOf:()=>"submit"});
 return (await signed(`${schedulerApi()}/v1/fleet/load`)).json();
};

if(mode==="scratch"){
 const server=await joinFleet(join(scratch,"server"),"g1-server");
 ports=[await freePort(),await freePort(),await freePort()];
 const idle=join(scratch,"idle-host.json");writeHostViewV1(idle,{atMs:Date.now()+86400000});
 const base=(root:string,origin:string):Record<string,string>=>({...rustEnv(),SEMIO_FLEET_ROOT:root,SEMIO_WORKSPACE_ROOT:root,SEMIO_FLEET_ORIGIN:origin,SEMIO_FLEET_GROUP:"g1",SEMIO_FLEET_STORE_ROOT:store,SEMIO_FLEET_NETWORK:NETWORK,SEMIO_CAS_ROOT:join(root,"cas"),SEMIO_REMOTE_EXECUTION_PORTS:`${ports[0]},${ports[1]}`,SEMIO_REMOTE_EXECUTION_HOST_VIEW:idle,SEMIO_REMOTE_CACHE_FLEET_PORT:String(ports[2])});
 cache=await startRemoteCacheServerV1({directory:join(scratch,"cache"),host:"127.0.0.1",port:0,maximumBytes:1<<30,maximumEntryBytes:1<<28,maximumAgeDays:30,defaultInstance:"main",tokens:{},fleet:{storeRoot:store,group:"g1",networks:[NETWORK],registryTtlMilliseconds:200}});
 await publishFleetCacheFactsV1({admin:{role:"admin",originId:"g1-server"},storeRoot:store,directory:join(scratch,"cache-data"),group:"g1",instance:"main",port:Number(new URL(cache.url).port),endpoints:[cache.url]});
 await refreshFleetCacheFactsV1({root:server,force:true});
 startRole("scheduler",base(server,"g1-server"),"scheduler");
 await eventuallyV1(()=>facts()?.execution!==undefined,60000,"the scheduler publishes its endpoint");
 for(let index=1;index<=agentsWanted;index++){const origin=`g1-a${String(index).padStart(2,"0")}`,root=await joinFleet(join(scratch,origin),origin);await refreshFleetCacheFactsV1({root,force:true});startRole("agent",base(root,origin),origin);agentOrigins.push(origin);}
 requesterOrigin="g1-r01";requesterRoot=await joinFleet(join(scratch,"requester"),requesterOrigin);await refreshFleetCacheFactsV1({root:requesterRoot,force:true});
 await eventuallyV1(async()=>{const table=await fleetTable();return agentOrigins.every(origin=>table.machines?.some((row:any)=>row.origin===origin&&row.worker.state==="idle"&&row.headroom===true));},120000,"all agents report an idle worker");
 log(`scratch fleet up: scheduler, ${agentsWanted} agents, signed cache ${cache.url}`);
}else{
 requesterRoot=LIVE;requesterOrigin=JSON.parse(readFileSync(join(LIVE,".cache","f","machine.json"),"utf8")).originId;
 const table=await fleetTable();agentOrigins.push(...(table.machines??[]).filter((row:any)=>row.origin!==requesterOrigin&&row.worker.state!=="absent").map((row:any)=>row.origin));
 log(`live fleet: ${(table.machines??[]).length} machine(s) report, workers: ${agentOrigins.join(",")}`);
}

const ws=join(scratch,"ws"),target=join(scratch,"target"),outputs=join(scratch,"out");
function cargoEnv(input:{label:string;remote:boolean;offload:"never"|"always"|"auto";extra?:Record<string,string>}):Record<string,string>{
 const cacheRoot=join(scratch,"uc-"+input.label);
 const environment=rustEnv({CARGO_TARGET_DIR:target,CARGO_INCREMENTAL:"0",CARGO_TERM_COLOR:"never",RUSTC:rustc,SEMIO_CAS_ROOT:join(scratch,"cas-"+input.label),...input.remote?{}:{SEMIO_UNIT_CACHE_REMOTE:"off"}});
 const unit=launch.unitCacheEnvironmentV1({cacheRoot,lane:"z",laneRoot:join(scratch,"lane-"+input.label),compiler:rustc,toolchain:versionText,focus:[],slots:jobs,workspaceRoot:requesterRoot,environment});
 if(codeMode==="stage")unit.RUSTC_WRAPPER=launch.ensureUnitCacheShimV1(cacheRoot,bun,script);
 mkdirSync(join(scratch,"cas-"+input.label),{recursive:true});
 return {...environment,...unit,SEMIO_OFFLOAD:input.offload,...input.extra};
}
async function cargo(input:{label:string;remote:boolean;offload:"never"|"always"|"auto";extra?:Record<string,string>}):Promise<{status:number;wallMs:number;tail:string}>{
 const started=Date.now(),child=spawn("cargo",["build","--offline","-j",String(jobs),"--manifest-path",join(ws,"Cargo.toml")],{cwd:ws,env:cargoEnv(input),stdio:["ignore","pipe","pipe"]});
 const text:Buffer[]=[];child.stdout.on("data",c=>text.push(c));child.stderr.on("data",c=>text.push(c));
 const status=await new Promise<number>(done=>child.on("close",code=>done(code??1)));
 writeFileSync(join(logs,`cargo-${input.label}.log`),Buffer.concat(text));
 return {status,wallMs:Date.now()-started,tail:Buffer.concat(text).toString().split("\n").slice(-4).join("\n")};
}
function makeWorkspace(salt:string):void{
 const run=spawnSync("python3",["-I",helper,ws,salt,"--iterations",iterations,"--layers",layers,"--width",width],{encoding:"utf8"});
 if(run.status!==0)throw Error(run.stderr);
}
function collectArtifacts(destination:string):Map<string,string>{
 const found=new Map<string,string>(),deps=join(target,"debug","deps");
 rmSync(destination,{recursive:true,force:true});mkdirSync(destination,{recursive:true});
 for(const name of readdirSync(deps))if(/^libpz.*\.(rlib|rmeta)$/u.test(name)){copyFileSync(join(deps,name),join(destination,name));found.set(name,sha(join(destination,name)));}
 return found;
}
const eventsOf=(cacheRoot:string):any[]=>{const directory=join(cacheRoot,"x");return existsSync(directory)?readdirSync(directory).filter(name=>name.endsWith(".jsonl")).flatMap(name=>readFileSync(join(directory,name),"utf8").split("\n").filter(Boolean).map(line=>{try{return JSON.parse(line);}catch{return undefined;}}).filter(Boolean)):[];};
type Unit={crate:string;where:string;roundTripMs:number|null;bytesShipped:number|null;bytesUploaded:number|null;bytesReturned:number|null;reason:string|null;detail:string|null};
function unitTable(cacheRoot:string):Unit[]{
 const events=eventsOf(cacheRoot),rows=new Map<string,Unit>();
 const row=(crate:string):Unit=>{let found=rows.get(crate);if(!found){found={crate,where:"local",roundTripMs:null,bytesShipped:null,bytesUploaded:null,bytesReturned:null,reason:null,detail:null};rows.set(crate,found);}return found;};
 for(const event of events){
  if(event.kind==="miss"&&typeof event.crate==="string"&&event.crate.startsWith("pz")){const unit=row(event.crate);const match=/offloaded:(\S+)/u.exec(event.reason??"");if(match)unit.where=match[1]!;}
  if(event.kind==="remote-hit"&&String(event.crate).startsWith("pz"))row(event.crate).where="fleet-cache";
  if(event.kind==="offload-run"){const unit=row(event.crate);unit.where=event.origin??unit.where;unit.roundTripMs=Math.round(event.milliseconds);unit.bytesShipped=event.bytesShipped??null;unit.bytesUploaded=event.bytesUploaded??null;unit.bytesReturned=event.bytesReturned??null;unit.detail=event.detail??null;}
  if(event.kind==="offload-declined"||event.kind==="offload-local-fallback"){const unit=row(event.crate);unit.reason=event.reason??event.kind;unit.detail=event.detail??null;}
 }
 return [...rows.values()].sort((a,b)=>a.crate<b.crate?-1:1);
}
const median=(values:number[]):number|null=>{const sorted=[...values].sort((a,b)=>a-b);return sorted.length===0?null:sorted[Math.floor(sorted.length/2)]!;};

const result:any={mode,code:codeMode,requesterOrigin,agents:agentOrigins,workspace:{layers:Number(layers),width:Number(width),units:Number(layers)*Number(width)+1,iterations:Number(iterations)},phases:[],startedAt:new Date().toISOString()};
try{
 for(const phase of phases){
  const salt=`${phase}-${Date.now()}`;
  log(`phase ${phase}: workspace salt ${salt}`);
  makeWorkspace(salt);
  rmSync(target,{recursive:true,force:true});
  const ref=await cargo({label:"ref-"+phase,remote:false,offload:"never"});
  if(ref.status!==0)throw Error("the local reference build failed:\n"+ref.tail);
  const refFiles=collectArtifacts(join(outputs,"ref-"+phase));
  log(`  local reference: ${ref.wallMs} ms, ${refFiles.size} artifacts`);
  rmSync(target,{recursive:true,force:true});
  const extra:Record<string,string>=phase==="auto"?{SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES:"2048"}:phase==="calm"?{SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES:"65536"}:{};
  const label="off-"+phase;
  if(phase==="auto"||phase==="calm"){mkdirSync(join(scratch,"uc-"+label),{recursive:true});cpSync(join(scratch,"uc-ref-"+phase,"c"),join(scratch,"uc-"+label,"c"),{recursive:true});}
  const before=await fleetTable().catch(()=>undefined);
  const off=await cargo({label,remote:true,offload:phase==="calm"?"auto":phase as "always"|"auto",extra});
  const gotFiles=collectArtifacts(join(outputs,"got-"+phase));
  const drained=await drain.drainUploadsV1(join(scratch,"uc-"+label),cargoEnv({label,remote:true,offload:"never"}),{idleMilliseconds:300}).catch((e:Error)=>({error:e.message}));
  const after=await fleetTable().catch(()=>undefined);
  const units=unitTable(join(scratch,"uc-"+label)),compared=[...refFiles].map(([name,hash])=>({name,equal:gotFiles.get(name)===hash,ref:hash.slice(0,16),got:(gotFiles.get(name)??"missing").slice(0,16)}));
  const offloaded=units.filter(u=>u.where!=="local"&&u.where!=="fleet-cache");
  const inOf=(table:any):Record<string,number>=>Object.fromEntries((table?.machines??[]).map((m:any)=>[m.origin,m.offloadedIn??0]));
  result.phases.push({phase,salt,reference:{status:ref.status,wallMs:ref.wallMs,artifacts:refFiles.size},offload:{status:off.status,wallMs:off.wallMs,tail:off.status===0?undefined:off.tail,artifacts:gotFiles.size},units,offloadedUnits:offloaded.length,roundTripMedianMs:median(offloaded.map(u=>u.roundTripMs).filter((v):v is number=>v!==null)),perOrigin:Object.fromEntries([...new Set(offloaded.map(u=>u.where))].map(origin=>[origin,offloaded.filter(u=>u.where===origin).length])),declines:Object.fromEntries([...new Set(units.map(u=>u.reason).filter(Boolean))].map(reason=>[reason!,units.filter(u=>u.reason===reason).length])),bytes:{shipped:offloaded.reduce((s,u)=>s+(u.bytesShipped??0),0),uploaded:offloaded.reduce((s,u)=>s+(u.bytesUploaded??0),0),returned:offloaded.reduce((s,u)=>s+(u.bytesReturned??0),0)},compared,identical:compared.filter(c=>c.equal).length,different:compared.filter(c=>!c.equal).map(c=>c.name),drained,schedulerOffloadedIn:Object.fromEntries(Object.entries(inOf(after)).map(([origin,count])=>[origin,count-(inOf(before)[origin]??0)]).filter(([,count])=>(count as number)!==0))});
  log(`  ${phase}: exit ${off.status}, ${off.wallMs} ms, offloaded ${offloaded.length}/${units.length}, identical ${compared.filter(c=>c.equal).length}/${compared.length}`);
 }
}finally{
 writeFileSync(outFile,JSON.stringify(result,null,1));
 await stopAll();
}
log("result written to "+outFile);
process.exit(0);
