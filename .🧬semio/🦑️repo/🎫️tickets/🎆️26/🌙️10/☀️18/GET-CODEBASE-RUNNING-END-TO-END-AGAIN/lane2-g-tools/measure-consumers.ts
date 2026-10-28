/** lane2-g: two concurrent artifact consumers (each its own OS process) on one fixture workspace, before (shared build directory under the real whole-directory custody, provenance written into the shared ledger) and after (two pooled lanes over the shared unit cache, provenance published by the retention rule). Each consumer: take custody|lane, cargo build with JSON messages, write the receipt with the real writer, retain|nothing, release.
 * usage: bun measure-consumers.ts <custody|lane> <rounds> <out.jsonl> [crateLines]
 * Streams: samples `ps` every 400 ms (RSS of the harness subtree), aborts above 500 MB of its own RSS, removes its scratch on exit. */
import {spawn,spawnSync} from "node:child_process";
import {appendFileSync,mkdirSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {join} from "node:path";

const R="/Users/ueli/Documents/semio",L=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library",N=R+"/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build",ROOT="/private/tmp/claude-501/-Users-ueli-Documents-semio/de5d5cb4-dca7-469c-b840-78655a0500ed/scratchpad/l2";
const isWorker=process.argv[2]==="worker",args=process.argv.slice(isWorker?3:2),mode=args[0]!;
const crates=["ca","cb"];

if(isWorker){
 const {acquireCargoBuildLeaseV1}=await import(N+"/🔒️lease/🟦️.ts");
 const {writeCompletedCargoInvocationProvenanceV1,cargoProvenancePhysicalControlV1}=await import(N+"/🟦️.ts");
 const {CurrentPhysicalOwnerV1}=await import(R+"/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️current/🟦️.ts");
 const {acquirePooledLaneV1}=await import(`${L}/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🏊️pool/🟦️.ts`);
 const {retainLaneOutputsV1}=await import(`${L}/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/📤️retention/🟦️.ts`);
 const {compilerLaneEnvironmentV1}=await import(`${L}/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🛤️lane/🟦️.ts`);
 const [,S,crate]=args as [string,string,string],W=join(S,"ws"),U=join(S,"u"),SHARED=join(S,"shared"),LEASES=join(S,"leases"),started=Date.now(),manifest=join(W,"Cargo.toml"),cargoArgs=["build","--locked","--manifest-path",manifest,"-p",crate],controller=new AbortController();
 let env:Record<string,string|undefined>={PATH:process.env.PATH,HOME:process.env.HOME,CARGO_HOME:join(S,"home"),CARGO_TERM_COLOR:"never",RUSTUP_TOOLCHAIN:"nightly-2026-07-20",CARGO_UNSTABLE_FINE_GRAIN_LOCKING:"true",CARGO_UNSTABLE_BUILD_DIR_NEW_LAYOUT:"true"},release=()=>{},laneId="shared",build=join(SHARED,"b"),target=join(SHARED,"t"),retain=async()=>0;
 if(mode==="custody"){
  env={...env,CARGO_TARGET_DIR:target,CARGO_BUILD_BUILD_DIR:build};
  const lease=await acquireCargoBuildLeaseV1({directory:LEASES,buildDirectory:build,targetDirectory:target,args:cargoArgs,signal:controller.signal});release=()=>lease.release();
 }else{
  const lane=await acquirePooledLaneV1(U,{crates:[crate],quotaBytes:Number.MAX_SAFE_INTEGER});laneId=lane.paths.id;build=lane.paths.build;target=lane.paths.target;
  env=compilerLaneEnvironmentV1({environment:{...env,RUSTC:"rustc"},cacheRoot:U,lane:{id:lane.paths.id,base:lane.paths.base,target:lane.paths.target,build:lane.paths.build},toolchain:"fixture-toolchain",crates:[crate],unitCache:true}) as Record<string,string|undefined>;
  const lease=await acquireCargoBuildLeaseV1({directory:LEASES,buildDirectory:build,targetDirectory:target,args:cargoArgs,signal:controller.signal});
  release=()=>{try{lease.release();}finally{lane.release();}};
  retain=async()=>{const t=Date.now();await retainLaneOutputsV1({lane:build,shared:join(SHARED,"b"),laneId:lane.paths.id,language:"en"});return(Date.now()-t)/1000;};
 }
 const acquired=Date.now();
 const run=await new Promise<{code:number;cpu:number;out:string}>(done=>{const child=spawn("/usr/bin/time",["-l","cargo",...cargoArgs,"--message-format=json-render-diagnostics"],{cwd:W,env:env as NodeJS.ProcessEnv,stdio:["ignore","pipe","pipe"]});let err="",out="";child.stdout.on("data",c=>{out+=c;});child.stderr.on("data",c=>{err+=c;});child.on("close",code=>{const u=/([0-9.]+)\s+user/u.exec(err),s=/([0-9.]+)\s+sys/u.exec(err);done({code:code??1,cpu:Number(u?.[1]??0)+Number(s?.[1]??0),out});});});
 const built=Date.now();
 const units:any[]=[],buildScripts:any[]=[];for(const line of run.out.split("\n")){if(!line.startsWith("{"))continue;const m=JSON.parse(line);if(m.reason==="build-script-executed")buildScripts.push(m);if(m.reason==="compiler-artifact")units.push({message:m,evidence:{version:1,kind:"discovery",paths:[],producer:null}});}
 await writeCompletedCargoInvocationProvenanceV1(join(S,"receipts","cargo-unit-provenance-"+crate+"-"+Date.now()+".json"),{manifest,cwd:W,command:"cargo",args:cargoArgs,buildDirectory:build,builtAtMs:started,status:run.code,cancelled:false,units,buildScripts},new Map(),join(S,"home"),()=>new CurrentPhysicalOwnerV1(W,cargoProvenancePhysicalControlV1({cancelled:()=>false,remainingMs:()=>600000,onProgress:()=>undefined})));
 const provenanced=Date.now(),retainS=await retain();
 release();
 console.log(JSON.stringify({crate,lane:laneId,waitS:(acquired-started)/1000,buildS:(built-acquired)/1000,provenanceS:(provenanced-built)/1000,retainS,wallS:(Date.now()-started)/1000,cpuS:run.cpu,code:run.code}));
 process.exit(run.code);
}

const [,roundsText,out,linesText]=args as [string,string,string,string|undefined],rounds=Number(roundsText),lines=Number(linesText??2500);
const S=join(ROOT,mode+process.pid),W=join(S,"ws"),U=join(S,"u"),SHARED=join(S,"shared"),LEASES=join(S,"leases");
mkdirSync(W,{recursive:true});mkdirSync(LEASES,{recursive:true});mkdirSync(join(SHARED,"t"),{recursive:true});mkdirSync(join(SHARED,"b"),{recursive:true});
const body=(salt:number)=>Array.from({length:lines},(_,i)=>`pub fn f${i}(x: u64) -> u64 { let mut v: Vec<u64> = (0..16).map(|k| k * ${i + 1} + x).collect(); for j in 0..v.len() { v[j] = v[j].wrapping_mul(${salt + i}).rotate_left((j % 7) as u32); } match x % 5 { 0 => v.iter().sum(), 1 => v.iter().max().copied().unwrap_or(0), 2 => v.iter().min().copied().unwrap_or(1), _ => v.len() as u64 } }`).join("\n")+"\n";
writeFileSync(join(W,"Cargo.toml"),`[workspace]\nresolver = "2"\nmembers = [${crates.map(c=>`"${c}"`).join(",")}]\n[profile.dev]\ndebug = 0\n`);
for(const c of crates){mkdirSync(join(W,c,"src"),{recursive:true});writeFileSync(join(W,c,"Cargo.toml"),`[package]\nname = "${c}"\nversion = "0.1.0"\nedition = "2021"\n`);writeFileSync(join(W,c,"src/lib.rs"),body(1));}
spawnSync("cargo",["generate-lockfile","--manifest-path",join(W,"Cargo.toml")],{cwd:W,env:{PATH:process.env.PATH,HOME:process.env.HOME,CARGO_HOME:join(S,"home"),RUSTUP_TOOLCHAIN:"nightly-2026-07-20"},encoding:"utf8"});
const edit=(c:string,salt:number)=>writeFileSync(join(W,c,"src/lib.rs"),body(salt));
let peakTree=0;const sample=setInterval(()=>{if(process.memoryUsage().rss/1048576>500){console.error("own RSS above 500 MB, abort");process.exit(3);}const rows=spawnSync("ps",["-Ao","pid=,ppid=,rss="],{encoding:"utf8"}).stdout.trim().split("\n").map(l=>l.trim().split(/\s+/).map(Number)),kids=new Map<number,number[]>();for(const [pid,ppid] of rows)kids.set(ppid!,[...(kids.get(ppid!)??[]),pid!]);let total=0;const walk=(pid:number)=>{for(const k of kids.get(pid)??[]){total+=rows.find(r=>r[0]===k)![2]!;walk(k);}};walk(process.pid);peakTree=Math.max(peakTree,total/1024);},400);
const du=(path:string)=>Number(spawnSync("du",["-sk",path],{encoding:"utf8"}).stdout.split("\t")[0])/1024;
const pair=(round:number)=>Promise.all(crates.map(crate=>new Promise<any>(done=>{const child=spawn(process.execPath,[import.meta.filename,"worker",mode,S,crate],{stdio:["ignore","pipe","inherit"]});let text="";child.stdout.on("data",c=>{text+=c;});child.on("close",code=>done({round,...JSON.parse(text.trim().split("\n").at(-1)||"{}"),exit:code}));})));
const log=(row:object)=>{appendFileSync(out,JSON.stringify({mode,pid:process.pid,lines,load:Number(spawnSync("sysctl",["-n","vm.loadavg"],{encoding:"utf8"}).stdout.trim().split(/\s+/)[1]),...row})+"\n");console.log(JSON.stringify(row));};
try{
 for(const c of crates){edit(c,1);}
 log({kind:"warm",rows:await pair(0)});
 for(let round=1;round<=rounds;round++){
  for(const c of crates)edit(c,round+1);
  peakTree=0;const started=Date.now(),rows=await pair(round);
  log({kind:"pair",round,wallS:(Date.now()-started)/1000,peakTreeRssMb:Math.round(peakTree),rows,diskMb:Math.round(du(mode==="custody"?SHARED:join(U,"l")))});
 }
 log({kind:"ledger",files:Number(spawnSync("sh",["-c",`ls ${join(SHARED,"b","semio-cargo-provenance")} 2>/dev/null | wc -l`],{encoding:"utf8"}).stdout.trim()),receiptsDir:Number(spawnSync("sh",["-c",`ls ${join(S,"receipts")} | wc -l`],{encoding:"utf8"}).stdout.trim())});
}finally{clearInterval(sample);rmSync(S,{recursive:true,force:true});}
