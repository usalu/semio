import {spawn,spawnSync} from "node:child_process";
import {appendFileSync,existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,statSync,writeFileSync} from "node:fs";
import {join,resolve} from "node:path";

/** 🔬️ W21 probe: replays the credential half of the fleet main test "a refusal is re-checked as soon as the credential changes" with a 100 ms sampler of the spool, the uploader lock holder (pid + command), the identity file and the spawn stamp, and writes the timeline to `🗑️generated/run-<n>.jsonl`. Usage: `bun probe.ts <runs> [subfolder of 🗑️generated]`. */
const UC="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache";
const {startRemoteCacheServerV1}=await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🖥️server/🟦️.ts");
const {drainUploadsV1,spoolLengthV1}=await import(UC+"/🛰️fleet/📤️uploader/🟦️.ts");
const {FLEET_POLICY_V1:fleet}=await import(UC+"/🛰️fleet/🪪️identity/🟦️.ts");
process.env.SEMIO_UNIT_CACHE_UPLOADER_IDLE_MS="1500";
const here=import.meta.dir,out=join(here,"🗑️generated",process.argv[3]??""),script=UC+"/📜️script.ts",counter=UC+"/🛰️fleet/🧪️tests/🔢️count/🟦️.ts",base=realpathSync(mkdtempSync(join(process.env.TMPDIR??"/tmp","w21-")));
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
type M={checkout:string;cache:string;cas:string;log:string};
function machine(name:string):M{const root=dir(name),checkout=join(root,"a","ws"),cache=join(root,"cache"),cas=join(root,"cas"),log=join(root,"compiles.jsonl");workspace(checkout);mkdirSync(cache,{recursive:true});writeFileSync(log,"");return{checkout,cache,cas,log};}
function build(m:M,environment:Record<string,string|undefined>,lane:string):Promise<number>{
 const laneRoot=join(m.cache,"l",lane);mkdirSync(laneRoot,{recursive:true});
 const env:Record<string,string|undefined>={...process.env,...Object.fromEntries(Object.keys(process.env).filter(k=>k.startsWith("SEMIO_REMOTE_CACHE_")||k.startsWith("SEMIO_FLEET_")||k==="SEMIO_WORKSPACE_ROOT"||k==="SEMIO_UNIT_CACHE_WORKSPACE_ROOT").map(k=>[k,undefined])),RUSTC_WRAPPER:process.execPath,RUSTC:script,SEMIO_UNIT_CACHE_ROOT:m.cache,SEMIO_UNIT_CACHE_LANE:lane,SEMIO_UNIT_CACHE_LANE_ROOT:laneRoot,SEMIO_UNIT_CACHE_SLOTS:"4",SEMIO_UNIT_CACHE_TOOLCHAIN:"test-toolchain",SEMIO_UNIT_CACHE_INNER:JSON.stringify([process.execPath,counter,m.log,"-","0","--","rustc"]),SEMIO_CAS_ROOT:m.cas,CARGO_TARGET_DIR:join(laneRoot,"t"),CARGO_TERM_COLOR:"never",CARGO_INCREMENTAL:"0",RUSTUP_TOOLCHAIN:"nightly-2026-07-20",...environment};
 for(const k of["RUSTC_WORKSPACE_WRAPPER","CARGO_BUILD_RUSTC_WRAPPER","CARGO_MAKEFLAGS","MAKEFLAGS","MFLAGS"])delete env[k];
 return new Promise(done=>{const c=spawn("cargo",["build","--manifest-path",join(m.checkout,"Cargo.toml"),"-p","app"],{cwd:m.checkout,env:env as NodeJS.ProcessEnv,stdio:"ignore"});c.on("close",code=>done(code??1));});
}
const remote=(url:string,token:string)=>({SEMIO_REMOTE_CACHE_ENDPOINT:url,SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:token});
const commands=new Map<number,string>();
const command=(pid:number)=>{let known=commands.get(pid);if(known===undefined){known=spawnSync("ps",["-o","command=","-p",String(pid)],{encoding:"utf8"}).stdout.trim().replace(/.*--semio-unit-cache/u,"").slice(0,120)||(pid===process.pid?"probe":"?");commands.set(pid,known);}return known;};
function sample(m:M,phase:string,t0:number){
 const state=join(m.cache,fleet.layout.state),spool=join(state,fleet.layout.spool),read=(p:string)=>{try{return readFileSync(p,"utf8");}catch{return undefined;}};
 let holder:number|undefined;try{holder=JSON.parse(read(join(state,"uploader.lock"))??"null")?.pid;}catch{}
 const jobs=existsSync(spool)?readdirSync(spool).filter(n=>n.endsWith(".json")).map(n=>{try{const j=JSON.parse(readFileSync(join(spool,n),"utf8"));return `${n.startsWith("offer")?"o":"j"}:${j.crate}:${j.deferredBy===undefined?"-":j.deferredBy.slice(-8)}:${j.notBeforeMs===undefined?"0":Math.round((j.notBeforeMs-Date.now())/100)/10}:${j.attempts??0}`;}catch{return n+":?";}}):[];
 let stamp:number|undefined;try{stamp=Math.round(statSync(join(state,fleet.layout.spawned)).mtimeMs-t0);}catch{}
 return{t:Date.now()-t0,phase,holder:holder===undefined?null:holder+" "+command(holder),identity:(read(join(state,fleet.layout.uploaderIdentity))??"").slice(-8),stamp,jobs};
}
const runs=Number(process.argv[2]??"1");
for(let run=1;run<=runs;run++){
 const server=await startRemoteCacheServerV1({directory:dir("srv"),host:"127.0.0.1",port:0,maximumBytes:2**30,maximumEntryBytes:2**28,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"w-token"}});
 const m=machine("d1"),work=dir("dwork"),file=join(out,`run-${run}.jsonl`),t0=Date.now();writeFileSync(file,"");
 let phase="build1";const timer=setInterval(()=>{appendFileSync(file,JSON.stringify(sample(m,phase,t0))+"\n");},100);
 const first=await build(m,{...remote(server.url,"wrong-token"),SEMIO_UNIT_CACHE_WORKSPACE_ROOT:work},"l1");
 phase="build2";const second=await build(m,{...remote(server.url,"w-token"),SEMIO_UNIT_CACHE_WORKSPACE_ROOT:work},"l2");
 phase="settle";process.env.SEMIO_CAS_ROOT=m.cas;
 let ok=false,calls=0,busy=0;const deadline=Date.now()+90000;
 while(Date.now()<deadline){calls++;const r=await drainUploadsV1(m.cache,{...process.env,...remote(server.url,"w-token"),SEMIO_UNIT_CACHE_WORKSPACE_ROOT:work,SEMIO_CAS_ROOT:m.cas},{idleMilliseconds:0});if(r==="busy")busy++;if(r!=="busy"&&spoolLengthV1(m.cache)===0){ok=true;break;}await new Promise(d=>setTimeout(d,150));}
 clearInterval(timer);
 const ev=join(m.cache,"x","uploader.jsonl"),uploaderEvents=existsSync(ev)?readFileSync(ev,"utf8").split("\n").filter(Boolean).map(l=>{const e=JSON.parse(l);return e.crate+" "+(e.reason??e.kind);}):[];
 const summary={run,first,second,ok,settleMs:Date.now()-t0,calls,busy,uploaderEvents};
 appendFileSync(file,JSON.stringify({summary})+"\n");console.log(JSON.stringify(summary));
 await server.stop().catch(()=>{});
 for(;;){const holder=sample(m,"after",t0).holder;if(holder===null)break;const pid=Number(holder.split(" ")[0]);try{process.kill(pid,"SIGTERM");}catch{break;}await new Promise(d=>setTimeout(d,300));}
}
rmSync(base,{recursive:true,force:true});
