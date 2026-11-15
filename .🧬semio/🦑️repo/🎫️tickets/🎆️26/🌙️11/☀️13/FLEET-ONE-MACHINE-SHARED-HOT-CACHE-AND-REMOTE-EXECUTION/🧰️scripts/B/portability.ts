import {spawn} from "node:child_process";
import {existsSync,mkdirSync,readFileSync,readdirSync} from "node:fs";
import {join} from "node:path";
import {startRemoteCacheServerV1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🖥️server/🟦️.ts";
import {drainUploadsV1,spoolLengthV1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/📤️uploader/🟦️.ts";

/** usage: bun portability.ts <scratch> <checkout A> <checkout B> <package>: two machines (cache + CAS each) at two paths against one in-process server; prints the unit events of both and the compiler spawn counts. */
const [scratch,checkoutA,checkoutB,pkg]=process.argv.slice(2) as [string,string,string,string];
const script="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts";
const counter="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/🧪️tests/🔢️count/🟦️.ts";
const server=await startRemoteCacheServerV1({directory:join(scratch,"server"),host:"127.0.0.1",port:0,maximumBytes:2**32,maximumEntryBytes:2**29,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"w-token"}});
async function machine(name:string,checkout:string,cargoHome?:string){
 const cache=join(scratch,name+"-cache"),cas=join(scratch,name+"-cas"),log=join(scratch,name+"-compiles.jsonl"),lane=join(cache,"l","lane1");mkdirSync(lane,{recursive:true});
 const env={...process.env,RUSTC_WRAPPER:process.execPath,RUSTC:script,SEMIO_UNIT_CACHE_ROOT:cache,SEMIO_UNIT_CACHE_LANE:"lane1",SEMIO_UNIT_CACHE_LANE_ROOT:lane,SEMIO_UNIT_CACHE_SLOTS:"4",SEMIO_UNIT_CACHE_BUILD_SCRIPTS:"on",SEMIO_UNIT_CACHE_INNER:JSON.stringify([process.execPath,counter,log,"-","0","--","rustc"]),SEMIO_CAS_ROOT:cas,CARGO_TARGET_DIR:join(lane,"t"),CARGO_TERM_COLOR:"never",SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w-token",...(cargoHome?{CARGO_HOME:cargoHome}:{})} as Record<string,string|undefined>;
 for(const key of["CARGO_MAKEFLAGS","MAKEFLAGS","MFLAGS","CARGO_BUILD_JOBS"])delete env[key];
 const started=Date.now(),code=await new Promise<number>(done=>{const child=spawn("cargo",["build","--offline","-p",pkg],{cwd:checkout,env:env as NodeJS.ProcessEnv,stdio:["ignore","inherit","inherit"]});child.on("close",c=>done(c??1));});
 const wall=Date.now()-started;
 while(spoolLengthV1(cache)>0||existsSync(join(cache,"fl","uploader.lock"))){await drainUploadsV1(cache,{...env,SEMIO_CAS_ROOT:cas} as Record<string,string>,{idleMilliseconds:0});await new Promise(d=>setTimeout(d,200));}
 const events=readdirSync(join(cache,"x")).flatMap(n=>readFileSync(join(cache,"x",n),"utf8").split("\n").filter(Boolean).map(l=>JSON.parse(l)));
 const kinds:Record<string,number>={};for(const e of events)kinds[e.kind]=(kinds[e.kind]??0)+1;
 const compiles=existsSync(log)?readFileSync(log,"utf8").split("\n").filter(Boolean).length:0;
 console.log(JSON.stringify({machine:name,code,wallMs:wall,kinds,rustcSpawns:compiles}));
 return events;
}
const a=await machine("A",checkoutA);
const b=await machine("B",checkoutB,process.env.B_CARGO_HOME);
const hit=new Set(b.filter(e=>e.kind==="remote-hit").map(e=>e.crate)),missed=b.filter(e=>e.kind==="miss"||e.kind==="remote-skip").map(e=>[e.kind,e.crate,e.reason]);
console.log(JSON.stringify({remoteHitsOnB:[...hit].sort(),missesOnB:missed}));
await server.stop();
