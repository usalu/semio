import {spawn} from "node:child_process";
import {existsSync,mkdirSync,readFileSync,readdirSync} from "node:fs";
import {join} from "node:path";
import {fleetRemoteEnvironmentV1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🚪️launch/🟦️.ts";
import {drainUploadsV1,spoolLengthV1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/📤️uploader/🟦️.ts";
/** usage: bun real-fleet.ts <scratch> <checkout A> <checkout B> <package>: the two scratch machines talk to the REAL fleet cache resolved from this machine's facts (the token is never printed). */
const [scratch,a,b,pkg]=process.argv.slice(2) as [string,string,string,string];
const script="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts";
const remote=fleetRemoteEnvironmentV1({...process.env,SEMIO_REMOTE_CACHE_ENDPOINT:undefined,SEMIO_REMOTE_CACHE_TOKEN:undefined,SEMIO_REMOTE_CACHE_ACCESS:undefined} as Record<string,string|undefined>,"/Users/Shared/semio");
console.log(JSON.stringify({resolved:Object.keys(remote),endpoint:remote.SEMIO_REMOTE_CACHE_ENDPOINT,instance:remote.SEMIO_REMOTE_CACHE_INSTANCE,access:remote.SEMIO_REMOTE_CACHE_ACCESS,hasToken:remote.SEMIO_REMOTE_CACHE_TOKEN!==undefined}));
async function machine(name:string,checkout:string){
 const cache=join(scratch,name+"-cache"),cas=join(scratch,name+"-cas"),lane=join(cache,"l","lane1");mkdirSync(lane,{recursive:true});
 const env={...process.env,...remote,RUSTC_WRAPPER:process.execPath,RUSTC:script,SEMIO_UNIT_CACHE_ROOT:cache,SEMIO_UNIT_CACHE_LANE:"lane1",SEMIO_UNIT_CACHE_LANE_ROOT:lane,SEMIO_UNIT_CACHE_SLOTS:"4",SEMIO_UNIT_CACHE_BUILD_SCRIPTS:"on",SEMIO_UNIT_CACHE_INNER:'["rustc"]',SEMIO_CAS_ROOT:cas,CARGO_TARGET_DIR:join(lane,"t"),CARGO_TERM_COLOR:"never"} as Record<string,string|undefined>;
 for(const k of["CARGO_MAKEFLAGS","MAKEFLAGS","MFLAGS","CARGO_BUILD_JOBS"])delete env[k];
 const code=await new Promise<number>(done=>{const c=spawn("cargo",["build","--offline","-p",pkg],{cwd:checkout,env:env as NodeJS.ProcessEnv,stdio:["ignore","ignore","ignore"]});c.on("close",x=>done(x??1));});
 while(spoolLengthV1(cache)>0||existsSync(join(cache,"fl","uploader.lock"))){await drainUploadsV1(cache,env as Record<string,string>,{idleMilliseconds:0});await new Promise(d=>setTimeout(d,200));}
 const events=readdirSync(join(cache,"x")).flatMap(n=>readFileSync(join(cache,"x",n),"utf8").split("\n").filter(Boolean).map(l=>JSON.parse(l)));
 const kinds:Record<string,number>={};for(const e of events)kinds[e.kind]=(kinds[e.kind]??0)+1;
 console.log(JSON.stringify({machine:name,code,kinds,skips:events.filter(e=>e.kind==="remote-skip").map(e=>e.reason).slice(0,3)}));
}
await machine("A",a);await machine("B",b);
