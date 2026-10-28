import {cpSync,existsSync,mkdirSync,rmSync,writeFileSync,appendFileSync} from "node:fs";
import {join} from "node:path";
import {bazel,flagsFor,idleSamples,shutdown} from "./probe.ts";
const tools=import.meta.dir,S="/private/tmp/claude-501/-Users-ueli-Documents-semio/de5d5cb4-dca7-469c-b840-78655a0500ed/scratchpad/bz";
const [variant,jvm,source,key,idle="40"]=process.argv.slice(2) as [string,string,string,string,string?];
const root=join(S,"p-"+variant),pkg=join(S,"pkg-"+key);
if(!existsSync(pkg)){mkdirSync(S,{recursive:true});const code=Bun.spawnSync(["cp","-cR",join(source,key),pkg]).exitCode;if(code!==0)throw Error("clone failed");}
rmSync(root,{recursive:true,force:true});mkdirSync(root,{recursive:true});
const heap=Number(/-Xmx(\d+)m/.exec(jvm)?.[1]??2048);
const flags=flagsFor(tools,root,"s1",heap);
flags.startup=flags.startup.filter(value=>!value.startsWith("--host_jvm_args="));
const extra=jvm.split(" ").filter(Boolean).map(value=>"--host_jvm_args="+value);
const log=join(tools,"..","🗑️generated","bzsrv-g","analysis.jsonl");mkdirSync(join(tools,"..","🗑️generated","bzsrv-g"),{recursive:true});
const first=await bazel(flags,extra,pkg,["build","--nobuild",...flags.command,"//:all"]);
const second=await bazel(flags,extra,pkg,["build","--nobuild",...flags.command,"//:all"]);
const samples=await idleSamples(flags,Number(idle),10);
const row={variant,jvm,key,first:{wall:first.wall,code:first.code,peak:first.peakRssMb,end:first.endRssMb},second:{wall:second.wall,code:second.code,peak:second.peakRssMb,end:second.endRssMb},idle:samples,tail:first.log.split("\n").slice(-6).join(" | ").slice(0,300)};
appendFileSync(log,JSON.stringify(row)+"\n");console.log(JSON.stringify(row));
shutdown(flags,pkg);rmSync(root,{recursive:true,force:true});
