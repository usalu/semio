import {spawn,spawnSync} from "node:child_process";
import {appendFileSync,existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,statSync,writeFileSync} from "node:fs";
import {join} from "node:path";

/** 🏁️ W21 race probe: replays the fleet main test "two machines start the same cold build at the same time" (two scratch machines, one server, `core` delayed 2.5 s) and samples both machines' uploader lock file (holder pid, alive, command), spool and spawn stamp every 100 ms until both locks are gone and both spools are empty (90 s bound); timeline in `🗑️generated/race/run-<n>.jsonl`. Usage: `bun race.ts <runs>`. */
const UC="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache";
const {startRemoteCacheServerV1}=await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🖥️server/🟦️.ts");
const {FLEET_POLICY_V1:fleet}=await import(UC+"/🛰️fleet/🪪️identity/🟦️.ts");
process.env.SEMIO_UNIT_CACHE_UPLOADER_IDLE_MS="1500";
const out=join(import.meta.dir,"🗑️generated","race"),script=UC+"/📜️script.ts",counter=UC+"/🛰️fleet/🧪️tests/🔢️count/🟦️.ts",base=realpathSync(mkdtempSync(join(process.env.TMPDIR??"/tmp","w21r-")));
mkdirSync(out,{recursive:true});
const dir=(label:string)=>realpathSync(mkdtempSync(join(base,label+"-")));
function crate(root:string,name:string,manifest:string,source:string,extra:Record<string,string>={}){const d=join(root,name);mkdirSync(join(d,"src"),{recursive:true});writeFileSync(join(d,"Cargo.toml"),manifest);writeFileSync(join(d,"src",source.includes("fn main()")?"main.rs":"lib.rs"),source);for(const [f,t] of Object.entries(extra))writeFileSync(join(d,f),t);}
function workspace(root:string){
 mkdirSync(root,{recursive:true});
 writeFileSync(join(root,"Cargo.toml"),`cargo-features = ["trim-paths"]\n[workspace]\nresolver = "2"\nmembers = ["core","mac","leaf1","leaf2","app"]\n[profile.dev]\ndebug = 0\ntrim-paths = "object"\n`);
 const manifest=(name:string,deps:string,extra="")=>`[package]\nname = "${name}"\nversion = "0.1.0"\nedition = "2021"\n${extra}[dependencies]\n${deps}`;
 crate(root,"core",manifest("core",""),"pub fn id() -> u32 { 1 }\n");
 crate(root,"mac",manifest("mac","","[lib]\nproc-macro = true\n"),"use proc_macro::TokenStream;\n#[proc_macro]\npub fn five(_: TokenStream) -> TokenStream { \"5\".parse().unwrap() }\n");
 for(const [i,name] of["leaf1","leaf2"].entries())crate(root,name,manifest(name,"core = { path = \"../core\" }\nmac = { path = \"../mac\" }\n","build = \"build.rs\"\n"),"pub fn f() -> u32 { core::id() + mac::five!() + env!(\"LEAF_ID\").parse::<u32>().unwrap() }\n",{"build.rs":`fn main() { println!("cargo:rustc-env=LEAF_ID=${i+1}"); }\n`});
 crate(root,"app",manifest("app","leaf1 = { path = \"../leaf1\" }\nleaf2 = { path = \"../leaf2\" }\n"),"fn main() { println!(\"{}\", leaf1::f() + leaf2::f()); }\n");
}
type M={name:string;checkout:string;cache:string;cas:string;log:string};
function machine(name:string,depth:string):M{const root=dir(name),checkout=join(root,depth,"ws"),cache=join(root,"cache"),cas=join(root,"cas"),log=join(root,"compiles.jsonl");workspace(checkout);mkdirSync(cache,{recursive:true});writeFileSync(log,"");return{name,checkout,cache,cas,log};}
function build(m:M,environment:Record<string,string|undefined>):Promise<number>{
 const laneRoot=join(m.cache,"l","l1");mkdirSync(laneRoot,{recursive:true});
 const env:Record<string,string|undefined>={...process.env,...Object.fromEntries(Object.keys(process.env).filter(k=>k.startsWith("SEMIO_REMOTE_CACHE_")||k.startsWith("SEMIO_FLEET_")||k==="SEMIO_WORKSPACE_ROOT"||k==="SEMIO_UNIT_CACHE_WORKSPACE_ROOT").map(k=>[k,undefined])),RUSTC_WRAPPER:process.execPath,RUSTC:script,SEMIO_UNIT_CACHE_ROOT:m.cache,SEMIO_UNIT_CACHE_LANE:"l1",SEMIO_UNIT_CACHE_LANE_ROOT:laneRoot,SEMIO_UNIT_CACHE_SLOTS:"4",SEMIO_UNIT_CACHE_TOOLCHAIN:"test-toolchain",SEMIO_UNIT_CACHE_INNER:JSON.stringify([process.execPath,counter,m.log,"core","2500","--","rustc"]),SEMIO_CAS_ROOT:m.cas,CARGO_TARGET_DIR:join(laneRoot,"t"),CARGO_TERM_COLOR:"never",CARGO_INCREMENTAL:"0",RUSTUP_TOOLCHAIN:"nightly-2026-07-20",...environment};
 for(const k of["RUSTC_WORKSPACE_WRAPPER","CARGO_BUILD_RUSTC_WRAPPER","CARGO_MAKEFLAGS","MAKEFLAGS","MFLAGS"])delete env[k];
 return new Promise(done=>{const c=spawn("cargo",["build","--manifest-path",join(m.checkout,"Cargo.toml"),"-p","app"],{cwd:m.checkout,env:env as NodeJS.ProcessEnv,stdio:"ignore"});c.on("close",code=>done(code??1));});
}
const alive=(pid:number)=>{try{process.kill(pid,0);return true;}catch{return false;}};
const commands=new Map<number,string>();
const command=(pid:number)=>{let known=commands.get(pid);if(known===undefined){known=spawnSync("ps",["-o","stat=,command=","-p",String(pid)],{encoding:"utf8"}).stdout.trim().replace(/\/\S*\/cache/u,"<root>").slice(0,90)||"gone";commands.set(pid,known);}return known;};
function sample(m:M){
 const state=join(m.cache,fleet.layout.state),spool=join(state,fleet.layout.spool),lock=join(state,"uploader.lock");
 let holder:string|null=null;if(existsSync(lock)){let pid=-1;try{pid=JSON.parse(readFileSync(lock,"utf8")).pid;}catch{}holder=`${pid} ${alive(pid)?"alive":"dead"} ${command(pid)} age ${(()=>{try{return Math.round(Date.now()-statSync(lock).mtimeMs);}catch{return -1;}})()}`;}
 const graves=existsSync(state)?readdirSync(state).filter(n=>n.includes(".dead-")).length:0;
 const jobs=existsSync(spool)?readdirSync(spool).filter(n=>n.endsWith(".json")).map(n=>{try{const j=JSON.parse(readFileSync(join(spool,n),"utf8"));return `${n.startsWith("offer")?"o":"j"}:${j.crate}:${j.notBeforeMs===undefined?"0":Math.round((j.notBeforeMs-Date.now())/100)/10}:${j.attempts??0}`;}catch{return n;}}):[];
 return{holder,graves,jobs};
}
const runs=Number(process.argv[2]??"1");
for(let run=1;run<=runs;run++){
 const server=await startRemoteCacheServerV1({directory:dir("srv"),host:"127.0.0.1",port:0,maximumBytes:2**30,maximumEntryBytes:2**28,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"w-token"}});
 const env={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w-token"};
 const m1=machine("r1","a"),m2=machine("r2","b/deeper"),file=join(out,`run-${run}.jsonl`),t0=Date.now();writeFileSync(file,"");
 let phase="build";const timer=setInterval(()=>{appendFileSync(file,JSON.stringify({t:Date.now()-t0,phase,m1:sample(m1),m2:sample(m2)})+"\n");},100);
 const codes=await Promise.all([build(m1,env),build(m2,env)]);
 phase="wait";const builtMs=Date.now()-t0;let ok=false;const deadline=Date.now()+90000;
 while(Date.now()<deadline){const a=sample(m1),b=sample(m2);if(!a.holder&&!b.holder&&a.jobs.length===0&&b.jobs.length===0){ok=true;break;}await new Promise(d=>setTimeout(d,200));}
 clearInterval(timer);
 const tail=(m:M)=>{const ev=join(m.cache,"x","uploader.jsonl");return existsSync(ev)?readFileSync(ev,"utf8").split("\n").filter(Boolean).slice(-8).map(l=>{const e=JSON.parse(l);return e.crate+" "+(e.reason??e.kind);}):[];};
 const summary={run,codes,ok,builtMs,settleMs:Date.now()-t0,m1:sample(m1),m2:sample(m2),m1Events:tail(m1),m2Events:tail(m2)};
 appendFileSync(file,JSON.stringify({summary})+"\n");console.log(JSON.stringify(summary));
 await server.stop().catch(()=>{});
 for(const m of[m1,m2])for(let i=0;i<20;i++){const h=sample(m).holder;if(!h)break;const pid=Number(h.split(" ")[0]);if(!alive(pid))break;try{process.kill(pid,"SIGTERM");}catch{}await new Promise(d=>setTimeout(d,300));}
}
rmSync(base,{recursive:true,force:true});
