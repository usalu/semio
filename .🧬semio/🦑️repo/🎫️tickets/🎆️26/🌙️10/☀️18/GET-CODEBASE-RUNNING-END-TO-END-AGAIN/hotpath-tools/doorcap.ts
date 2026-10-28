/** 🚪️ Graph-free cost of the door's live source capture, read-only: `bun doorcap.ts <root> current|once|staged [memo|-] [module-root]` (module-root: where the closure module is loaded from, e.g. a mirror holding a staged batch).
 * `staged` runs the staged `buildSourceClosureTextV1` exactly as the staged compiler + command call it; `current` replays `captureCompilerBuildSourceClosureV1` + `executeCapturedBuildSourceV1` step by step (base64 rows, schema identity, bound stringify, text stringify, identity again); `once` builds the same snapshot text with ONE stringify and the identity as sha256 of that text. Both then drop every reference, collect and print what the door would keep resident while it waits for the executor. Prints wall, CPU and RSS per step; writes nothing (the memo is read only when `memo` is given and the capture serves from it). */
import {createHash} from "node:crypto";
import {join,relative} from "node:path";
const [root,mode,memoArg,moduleRoot]=process.argv.slice(2) as [string,"current"|"once"|"staged",string|undefined,string|undefined];
const L=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const auth=await import(join(L,"⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts")),memoModule=await import(join(L,"⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧊️memo/🟦️.ts")),closure=await import(join(moduleRoot??root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟦️.ts")),selection=await import(join(L,"🏗️build/⌨️command/🧭️selection/🟦️.ts"));
const t0=performance.now(),c0=process.cpuUsage();let tl=t0,cl=c0;
const step=(name:string,detail:unknown="")=>{const t=performance.now(),c=process.cpuUsage(),mb=(n:number)=>Math.round(n/1048576);console.log(JSON.stringify({mode,step:name,wallMs:Math.round(t-tl),cpuMs:Math.round((c.user-cl.user+c.system-cl.system)/1000),rssMb:mb(process.memoryUsage().rss),heapMb:mb(process.memoryUsage().heapUsed),detail}));tl=t;cl=c;};
step("modules");
const entry=relative(root,selection.repositoryCommandKernelV1(["run","@teaching/architecture-course:build"]).entry).split("\\").join("/");
const memo=memoArg==="memo"&&memoModule.readNxAuthorizerMemoSwitchV1(process.env)?{file:memoModule.nxAuthorizerMemoFileV1(memoModule.nxAuthorizerMemoBaseV1(process.env),root,entry,true),served:(served:string)=>console.log(JSON.stringify({served}))}:undefined;
let captured:any=await auth.captureNxAuthorizerSourcesV1(root,entry,new AbortController().signal,{installed:true,...(memo?{memo}:{})});step("capture",{sources:captured.manifest.sources.length});
let bytes:any=auth.nxAuthorizerCapturedBytesV1(captured),imports:any=auth.nxAuthorizerCapturedImportsV1(root,captured),files:any[]=[];
for(const row of captured.manifest.sources)files.push(Object.freeze({...row,base64:Buffer.from(bytes.get(row.path)).toString("base64"),imports:mode==="staged"?Object.freeze({...imports.get(row.path)}):imports.get(row.path)}));
let snapshot:any=Object.freeze({version:1,root,entry,files:Object.freeze(files)});step("rows",{files:files.length});
let text:string,sha256:string;
if(mode==="current"){
 closure.buildSourceClosureIdentityV1(snapshot);step("identity-1");
 Buffer.byteLength(JSON.stringify(snapshot));step("bound-stringify");
 text=JSON.stringify(snapshot);step("text-stringify",{bytes:Buffer.byteLength(text)});
 sha256=closure.buildSourceClosureIdentityV1(snapshot);step("identity-2");
}else if(mode==="staged"){
 Buffer.byteLength(closure.buildSourceClosureTextV1(snapshot).text);step("bound-text");
 ({text,sha256}=closure.buildSourceClosureTextV1(snapshot));step("publication-text",{bytes:Buffer.byteLength(text)});
}else{
 text=JSON.stringify(snapshot);sha256=createHash("sha256").update(text).digest("hex");step("text-once",{bytes:Buffer.byteLength(text)});
}
step("held-while-waiting",{sha:sha256.slice(0,12)});
captured=bytes=imports=snapshot=undefined;files=[];
Bun.gc(true);await Bun.sleep(1500);step("snapshot-dropped-gc",{textKept:text.length>0});
text="";Bun.gc(true);await Bun.sleep(1500);step("text-dropped-gc");
const c=process.cpuUsage();console.log(JSON.stringify({mode,total:{cpuMs:Math.round((c.user-c0.user+c.system-c0.system)/1000)}}));
