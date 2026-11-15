import {spawn,spawnSync} from "node:child_process";
import {copyFileSync,mkdirSync,readdirSync,rmSync,writeFileSync,readFileSync} from "node:fs";
import {createServer} from "node:net";
import {join} from "node:path";
import {startRemoteCacheServerV1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🖥️server/🟦️.ts";
import {drainUploadsV1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/📤️uploader/🟦️.ts";

/** usage: bun bench.ts <scratch dir> [rounds]: wall time of one wrapper invocation (a bun process) for one rustc command line, by scenario; the compiler is a fake that copies prebuilt outputs, so the numbers are the wrapper's own cost. */
const scratch=process.argv[2]!,rounds=Number(process.argv[3]??25);
const script="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts";
rmSync(scratch,{recursive:true,force:true});mkdirSync(join(scratch,"ws/src"),{recursive:true});
writeFileSync(join(scratch,"ws/src/lib.rs"),Array.from({length:60},(_,i)=>`pub fn f${i}(x: u64) -> u64 { x.wrapping_mul(${i+3}) }`).join("\n")+"\n");
const real=join(scratch,"real");mkdirSync(real,{recursive:true});
const args=(out:string)=>["--crate-name","tiny","--edition=2021","src/lib.rs","--crate-type","lib","--emit=dep-info,metadata,link","--out-dir",out,"--error-format=json","--json=diagnostic-rendered-ansi,artifacts,future-incompat","-C","metadata=abc123","-C","extra-filename=-abc123"];
const built=spawnSync("rustc",args(real),{cwd:join(scratch,"ws"),encoding:"utf8"});if(built.status!==0)throw Error(built.stderr);
const fake=join(scratch,"fake-rustc.ts");
writeFileSync(fake,`import {copyFileSync,readdirSync} from "node:fs";import {join} from "node:path";
const a=process.argv.slice(2);const out=a[a.indexOf("--out-dir")+1]!;
if(a.includes("-vV")){process.stdout.write(${JSON.stringify(spawnSync("rustc",["-vV"],{encoding:"utf8"}).stdout)});process.exit(0);}
for(const n of readdirSync(${JSON.stringify(real)}))copyFileSync(join(${JSON.stringify(real)},n),join(out,n));
for(const n of readdirSync(${JSON.stringify(real)}))process.stderr.write(JSON.stringify({artifact:join(out,n),emit:n.endsWith(".rlib")?"link":n.endsWith(".rmeta")?"metadata":"dep-info"})+"\\n");
`);
const median=(v:number[])=>[...v].sort((a,b)=>a-b)[Math.floor(v.length/2)]!,p95=(v:number[])=>[...v].sort((a,b)=>a-b)[Math.min(v.length-1,Math.floor(v.length*0.95))]!;
type Setup={cache:string;env:Record<string,string>};
function run(setup:Setup,out:string):Promise<number>{
 mkdirSync(out,{recursive:true});
 const env={...process.env,...setup.env,RUSTC_WRAPPER:process.execPath} as Record<string,string|undefined>;for(const k of["CARGO_MAKEFLAGS","MAKEFLAGS","MFLAGS"])delete env[k];
 return new Promise(done=>{const t=performance.now(),child=spawn(process.execPath,[script,"rustc",...args(out)],{cwd:join(scratch,"ws"),env:env as NodeJS.ProcessEnv,stdio:["ignore","ignore","ignore"]});child.on("close",()=>done(performance.now()-t));});
}
function base(name:string,remote:Record<string,string>):Setup{
 const cache=join(scratch,name),lane=join(cache,"l","l1");mkdirSync(lane,{recursive:true});
 return {cache,env:{SEMIO_UNIT_CACHE_ROOT:cache,SEMIO_UNIT_CACHE_LANE:"l1",SEMIO_UNIT_CACHE_LANE_ROOT:lane,SEMIO_UNIT_CACHE_SLOTS:"4",SEMIO_UNIT_CACHE_TOOLCHAIN:"bench-toolchain",SEMIO_UNIT_CACHE_INNER:JSON.stringify([process.execPath,fake]),SEMIO_CAS_ROOT:join(scratch,name+"-cas"),RUSTC:script,...remote}};
}
const server=await startRemoteCacheServerV1({directory:join(scratch,"server"),host:"127.0.0.1",port:0,maximumBytes:2**30,maximumEntryBytes:2**28,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"w-token"}});
const on={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w-token"};
const table:Record<string,{median:number;p95:number;n:number}> = {};
const record=(name:string,values:number[])=>{table[name]={median:Math.round(median(values)*10)/10,p95:Math.round(p95(values)*10)/10,n:values.length};};

const empty=(setup:Setup)=>{rmSync(join(setup.cache,"e"),{recursive:true,force:true});rmSync(join(setup.cache,"fl","spool"),{recursive:true,force:true});};

{ // local hit, fleet off vs on
 const off=base("hit-off",{}),onSetup=base("hit-on",on);
 await run(off,join(scratch,"o1"));await run(onSetup,join(scratch,"o2"));
 const a:number[]=[],b:number[]=[];
 for(let i=0;i<rounds;i++){a.push(await run(off,join(scratch,"o1")));b.push(await run(onSetup,join(scratch,"o2")));}
 record("local hit, fleet off",a);record("local hit, fleet on",b);
}
{ // miss: compile (fake) with and without the fleet, uploads not drained (the wrapper's own cost)
 const a:number[]=[],b:number[]=[];
 for(let i=0;i<rounds;i++){
  const off=base("miss-off",{});empty(off);rmSync(join(scratch,"miss-off-cas"),{recursive:true,force:true});a.push(await run(off,join(scratch,"o3")));
  rmSync(off.cache,{recursive:true,force:true});
 }
 for(let i=0;i<rounds;i++){
  const onSetup=base("miss-on",{...on,SEMIO_REMOTE_CACHE_ACCESS:"read-only"});rmSync(onSetup.cache,{recursive:true,force:true});rmSync(join(scratch,"miss-on-cas"),{recursive:true,force:true});const s2=base("miss-on",{...on,SEMIO_REMOTE_CACHE_ACCESS:"read-only"});b.push(await run(s2,join(scratch,"o4")));
 }
 record("miss + compile (fake), fleet off",a);record("miss + compile (fake), fleet on, read-only lookup",b);
}
{ // miss with claim + enqueue (read-write): wrapper's cost to exit, uploads happen later
 const c:number[]=[];
 for(let i=0;i<rounds;i++){const s=base("miss-rw",on);rmSync(s.cache,{recursive:true,force:true});rmSync(join(scratch,"miss-rw-cas"),{recursive:true,force:true});const s2=base("miss-rw",{...on,SEMIO_UNIT_CACHE_REMOTE:"on"});
  // different unit each round so the server never has it: vary the metadata
  const t=await runVaried(s2,i);c.push(t);}
 record("miss + claim + compile (fake) + enqueue upload, fleet on",c);
}
async function runVaried(setup:Setup,i:number):Promise<number>{
 const out=join(scratch,"o5");mkdirSync(out,{recursive:true});
 const env={...process.env,...setup.env,RUSTC_WRAPPER:process.execPath} as Record<string,string|undefined>;for(const k of["CARGO_MAKEFLAGS","MAKEFLAGS","MFLAGS"])delete env[k];
 const a=args(out).map(x=>x==="metadata=abc123"?"metadata=v"+i+Date.now():x);
 return new Promise(done=>{const t=performance.now(),child=spawn(process.execPath,[script,"rustc",...a],{cwd:join(scratch,"ws"),env:env as NodeJS.ProcessEnv,stdio:["ignore","ignore","ignore"]});child.on("close",()=>done(performance.now()-t));});
}
{ // remote hit: local state wiped each round, server has the unit
 const s=base("rhit",on);await run(s,join(scratch,"o6"));
 const env={...process.env,...s.env} as Record<string,string>;
 for(let k=0;k<20;k++){const r=await drainUploadsV1(s.cache,env,{idleMilliseconds:0});if(r!=="busy")break;await new Promise(d=>setTimeout(d,100));}
 const v:number[]=[];
 for(let i=0;i<rounds;i++){empty(s);rmSync(join(scratch,"rhit-cas"),{recursive:true,force:true});v.push(await run(s,join(scratch,"o6")));}
 record("remote hit (install + restore)",v);
}
{ // unreachable: connection refused, breaker cleared each round (first invocation after the outage), then with the breaker open
 const closed=await startRemoteCacheServerV1({directory:join(scratch,"closed"),host:"127.0.0.1",port:0,maximumBytes:2**26,maximumEntryBytes:2**24,maximumAgeDays:30,defaultInstance:"main",tokens:{}});
 const url=closed.url;await closed.stop();
 const first:number[]=[],later:number[]=[];
 for(let i=0;i<rounds;i++){const s=base("down",{...on,SEMIO_REMOTE_CACHE_ENDPOINT:url});rmSync(s.cache,{recursive:true,force:true});rmSync(join(scratch,"down-cas"),{recursive:true,force:true});const s2=base("down",{...on,SEMIO_REMOTE_CACHE_ENDPOINT:url});first.push(await runVaried(s2,1000+i));later.push(await runVaried(s2,2000+i));}
 record("server down (connection refused), first unit after outage",first);record("server down, breaker open (later units)",later);
}
{ // blackhole: accepts the connection and never answers
 const sockets:import("node:net").Socket[]=[],black=createServer(socket=>{sockets.push(socket);}).listen(0,"127.0.0.1");await new Promise(d=>black.once("listening",d));
 const port=(black.address() as import("node:net").AddressInfo).port,first:number[]=[],later:number[]=[];
 for(let i=0;i<5;i++){const s=base("hole",{...on,SEMIO_REMOTE_CACHE_ENDPOINT:`http://127.0.0.1:${port}`});rmSync(s.cache,{recursive:true,force:true});rmSync(join(scratch,"hole-cas"),{recursive:true,force:true});const s2=base("hole",{...on,SEMIO_REMOTE_CACHE_ENDPOINT:`http://127.0.0.1:${port}`});first.push(await runVaried(s2,3000+i));later.push(await runVaried(s2,4000+i));}
 record("server stalls (accepts, never answers), first unit",first);record("server stalls, breaker open (later units)",later);
 for(const s of sockets)s.destroy();black.close();
}
{ // denied
 const first:number[]=[];
 for(let i=0;i<rounds;i++){const s=base("deny",{...on,SEMIO_REMOTE_CACHE_TOKEN:"wrong"});rmSync(s.cache,{recursive:true,force:true});rmSync(join(scratch,"deny-cas"),{recursive:true,force:true});const s2=base("deny",{...on,SEMIO_REMOTE_CACHE_TOKEN:"wrong"});first.push(await runVaried(s2,5000+i));}
 record("credential refused (401), first unit",first);
}
console.log(JSON.stringify(table,null,1));
await server.stop();
