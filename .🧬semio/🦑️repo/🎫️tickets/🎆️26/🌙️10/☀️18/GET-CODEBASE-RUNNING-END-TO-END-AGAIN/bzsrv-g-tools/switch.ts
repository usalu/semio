import {mkdirSync,rmSync,appendFileSync} from "node:fs";
import {join} from "node:path";
import {bazel,flagsFor,shutdown} from "./probe.ts";
const tools=import.meta.dir,S="/private/tmp/claude-501/-Users-ueli-Documents-semio/de5d5cb4-dca7-469c-b840-78655a0500ed/scratchpad/bz",root=join(S,"switch");
const SRC="/Users/ueli/Documents/semio/.cache/b/p/m/bazel/workspaces",[first,second,secondSlot]=process.argv.slice(2) as [string,string,string];
rmSync(root,{recursive:true,force:true});mkdirSync(root,{recursive:true});
const clone=(from:string,to:string)=>{rmSync(to,{recursive:true,force:true});mkdirSync(join(to,".."),{recursive:true});if(Bun.spawnSync(["cp","-cR",from,to]).exitCode!==0)throw Error("clone "+from);};
const flags=flagsFor(tools,join(root,"p"),"s0",1536);flags.startup=flags.startup.concat(["--host_jvm_args=-XX:G1PeriodicGCInterval=10000"]);
const path=join(root,"ws","s0","cur"),rows:Record<string,unknown>[]=[];
const src=(key:string)=>key==="760c0f403769cd16"?join(S,"pkg-"+key):join(SRC,secondSlot,key);
clone(src(first),path);
const run=async(label:string)=>{const result=await bazel(flags,[],path,["build","--nobuild",...flags.command,"//:all"]);rows.push({label,wall:result.wall,code:result.code,peak:result.peakRssMb,line:/Elapsed time: [\d.]+s/.exec(result.log)?.[0]});};
await run("A cold key1");await run("A hot key1");
// in place: the package of key2 replaces key1 at the SAME path
clone(src(second),path);await run("B in-place key2 (same workspace path, hot server)");await run("B hot key2");
// restart: key2 at a DIFFERENT path
const other=join(root,"ws","s0","other");clone(src(second),other);
const t=await bazel(flags,[],other,["build","--nobuild",...flags.command,"//:all"]);rows.push({label:"C key2 at a different path (server restarts)",wall:t.wall,code:t.code,peak:t.peakRssMb,line:/Elapsed time: [\d.]+s/.exec(t.log)?.[0]});
const out=join(tools,"..","🗑️generated","bzsrv-g");appendFileSync(join(out,"switch.jsonl"),JSON.stringify(rows)+"\n");console.log(JSON.stringify(rows,null,1));
shutdown(flags,other);rmSync(root,{recursive:true,force:true});
