/** lane-g: two concurrent Cargo callers on one fixture workspace, before (shared build dir under the real whole-directory custody) and after (two pooled lanes over the shared unit cache). Each caller is its own OS process (the custody coalesces holders of one process).
 * usage: bun measure-pool.ts <custody|pool> <rounds> <out.jsonl> [crateLines]
 * Streams: samples `ps` every 400 ms (RSS of the harness subtree), aborts above 500 MB of its own RSS, kills its children on exit. */
import {spawn,spawnSync} from "node:child_process";
import {appendFileSync,mkdirSync,rmSync,writeFileSync} from "node:fs";
import {join} from "node:path";

const L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library",ROOT="/private/tmp/claude-501/-Users-ueli-Documents-semio/de5d5cb4-dca7-469c-b840-78655a0500ed/scratchpad/lg";
const isWorker=process.argv[2]==="worker",args=process.argv.slice(isWorker?3:2),mode=args[0]!;
const crates=["ca","cb"];

if(isWorker){
 const {acquireCargoBuildLeaseV1}=await import("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🟦️.ts");
 const {acquirePooledLaneV1}=await import(`${L}/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🏊️pool/🟦️.ts`);
 const {compilerLaneEnvironmentV1}=await import(`${L}/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🛤️lane/🟦️.ts`);
 const [,S,crate]=args as [string,string,string],W=join(S,"ws"),U=join(S,"u"),SHARED=join(S,"shared"),LEASES=join(S,"leases"),started=Date.now(),cargoArgs=["build","--manifest-path",join(W,"Cargo.toml"),"-p",crate],controller=new AbortController();
 let env:Record<string,string|undefined>={...process.env,CARGO_TERM_COLOR:"never",RUSTUP_TOOLCHAIN:"nightly-2026-07-20",CARGO_UNSTABLE_FINE_GRAIN_LOCKING:"true",CARGO_UNSTABLE_BUILD_DIR_NEW_LAYOUT:"true"},release=()=>{},laneId="shared";
 if(mode==="custody"){
  env={...env,CARGO_TARGET_DIR:join(SHARED,"t"),CARGO_BUILD_BUILD_DIR:join(SHARED,"b")};
  const lease=await acquireCargoBuildLeaseV1({directory:LEASES,buildDirectory:join(SHARED,"b"),targetDirectory:join(SHARED,"t"),args:cargoArgs,signal:controller.signal});release=()=>lease.release();
 }else{
  const lane=await acquirePooledLaneV1(U,{crates:[crate],quotaBytes:Number.MAX_SAFE_INTEGER});laneId=lane.paths.id;
  env=compilerLaneEnvironmentV1({environment:{...env,RUSTC:"rustc"},cacheRoot:U,lane:{id:lane.paths.id,base:lane.paths.base,target:lane.paths.target,build:lane.paths.build},toolchain:"fixture-toolchain",crates:[crate],unitCache:true}) as Record<string,string|undefined>;
  const lease=await acquireCargoBuildLeaseV1({directory:LEASES,buildDirectory:lane.paths.build,targetDirectory:lane.paths.target,args:cargoArgs,signal:controller.signal});release=()=>{try{lease.release();}finally{lane.release();}};
 }
 const acquired=Date.now();
 const run=await new Promise<{code:number;cpu:number}>(done=>{const child=spawn("/usr/bin/time",["-l","cargo",...cargoArgs],{cwd:W,env:env as NodeJS.ProcessEnv,stdio:["ignore","ignore","pipe"]});let err="";child.stderr.on("data",c=>{err+=c;});child.on("close",code=>{const u=/([0-9.]+)\s+user/u.exec(err),s=/([0-9.]+)\s+sys/u.exec(err);done({code:code??1,cpu:Number(u?.[1]??0)+Number(s?.[1]??0)});});});
 release();
 console.log(JSON.stringify({crate,lane:laneId,waitS:(acquired-started)/1000,runS:(Date.now()-acquired)/1000,wallS:(Date.now()-started)/1000,cpuS:run.cpu,code:run.code}));
 process.exit(run.code);
}

const [,roundsText,out,linesText]=args as [string,string,string,string|undefined],rounds=Number(roundsText),lines=Number(linesText??2500);
const S=join(ROOT,mode+process.pid),W=join(S,"ws"),U=join(S,"u"),SHARED=join(S,"shared"),LEASES=join(S,"leases");
mkdirSync(W,{recursive:true});mkdirSync(LEASES,{recursive:true});mkdirSync(join(SHARED,"t"),{recursive:true});mkdirSync(join(SHARED,"b"),{recursive:true});
const body=(salt:number)=>Array.from({length:lines},(_,i)=>`pub fn f${i}(x: u64) -> u64 { let mut v: Vec<u64> = (0..16).map(|k| k * ${i + 1} + x).collect(); for j in 0..v.len() { v[j] = v[j].wrapping_mul(${salt + i}).rotate_left((j % 7) as u32); } match x % 5 { 0 => v.iter().sum(), 1 => v.iter().max().copied().unwrap_or(0), 2 => v.iter().min().copied().unwrap_or(1), _ => v.len() as u64 } }`).join("\n")+"\n";
writeFileSync(join(W,"Cargo.toml"),`[workspace]\nresolver = "2"\nmembers = [${crates.map(c=>`"${c}"`).join(",")}]\n[profile.dev]\ndebug = 0\n`);
for(const c of crates){mkdirSync(join(W,c,"src"),{recursive:true});writeFileSync(join(W,c,"Cargo.toml"),`[package]\nname = "${c}"\nversion = "0.1.0"\nedition = "2021"\n`);writeFileSync(join(W,c,"src/lib.rs"),body(1));}
const edit=(c:string,salt:number)=>writeFileSync(join(W,c,"src/lib.rs"),body(salt));
let peakTree=0;const sample=setInterval(()=>{if(process.memoryUsage().rss/1048576>500){console.error("own RSS above 500 MB, abort");process.exit(3);}const rows=spawnSync("ps",["-Ao","pid=,ppid=,rss="],{encoding:"utf8"}).stdout.trim().split("\n").map(l=>l.trim().split(/\s+/).map(Number)),kids=new Map<number,number[]>();for(const [pid,ppid] of rows)kids.set(ppid!,[...(kids.get(ppid!)??[]),pid!]);let total=0;const walk=(pid:number)=>{for(const k of kids.get(pid)??[]){total+=rows.find(r=>r[0]===k)![2]!;walk(k);}};walk(process.pid);peakTree=Math.max(peakTree,total/1024);},400);
const du=(path:string)=>Number(spawnSync("du",["-sk",path],{encoding:"utf8"}).stdout.split("\t")[0])/1024;
const pair=(round:number)=>Promise.all(crates.map(crate=>new Promise<any>(done=>{const child=spawn(process.execPath,[import.meta.filename,"worker",mode,S,crate],{stdio:["ignore","pipe","inherit"]});let text="";child.stdout.on("data",c=>{text+=c;});child.on("close",code=>done({round,...JSON.parse(text.trim().split("\n").at(-1)||"{}"),exit:code}));})));
const log=(row:object)=>{appendFileSync(out,JSON.stringify({mode,pid:process.pid,lines,...row})+"\n");console.log(JSON.stringify(row));};
try{
 for(const c of crates){edit(c,1);}
 log({kind:"warm",rows:await pair(0)});
 for(let round=1;round<=rounds;round++){
  for(const c of crates)edit(c,round+1);
  peakTree=0;const started=Date.now(),rows=await pair(round);
  log({kind:"pair",round,wallS:(Date.now()-started)/1000,peakTreeRssMb:Math.round(peakTree),rows,diskMb:Math.round(du(mode==="custody"?SHARED:join(U,"l")))});
 }
 log({kind:"disk",sharedMb:Math.round(du(SHARED)),lanesMb:Math.round(du(join(U,"l"))),objectsMb:Math.round(du(join(U,"o"))),lanes:mode==="pool"?spawnSync("ls",[join(U,"l")],{encoding:"utf8"}).stdout.trim().split("\n").length:0});
}finally{clearInterval(sample);rmSync(S,{recursive:true,force:true});}
