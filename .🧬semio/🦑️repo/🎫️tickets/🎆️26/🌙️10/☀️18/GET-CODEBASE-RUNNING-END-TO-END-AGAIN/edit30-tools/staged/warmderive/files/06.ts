/** ♨️ The warm deriver: a detached process that answers the memo's derivation requests with the planner's own deriver and keeps the TypeScript programs it built between requests, so a one-line edit of a course module re-asks its program question in a fraction of a second instead of building a program from nothing in a fresh child. It runs exactly the implementation of the run that started it (every file of that run's captured closure is byte-identical on disk before and after it loads), serves only requests carrying that run's identity, answers one request at a time and exits when idle. Any failure of the warm path falls back to the derivation child.
 * @see ../🟦️.ts the derivation request and answer files
 * @see ../../../../../🕸️graph/📥️inference/🧭️script-inputs/♨️reuse/🟨️.mjs the reuse store */
import {spawn} from "node:child_process";
import {createHash} from "node:crypto";
import {copyFileSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {connect,createServer,type Server,type Socket} from "node:net";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {GRAPH_MEMO_POLICY_V1,type MemoRowV1} from "../../🟦️.ts";
import type {MemoDeriverV1} from "../🟦️.ts";
import type {TypeScriptProgramReuseV1} from "../../../../../🕸️graph/📥️inference/🧭️script-inputs/♨️reuse/🟨️.mjs";

const policy=GRAPH_MEMO_POLICY_V1.warm;
/** 🧾️ One file of a captured closure. */
export type WarmClosureV1=Readonly<{root:string;sources:readonly Readonly<{path:string;sha256:string}>[]}>;
/** 🔌️ What the memo asks of the warm path: an answer to a written request file (true when the warm deriver wrote the answer next to it), and a start after a cold derivation. */
export type WarmDeriverPortV1=Readonly<{ask:(request:string,signal?:AbortSignal)=>Promise<boolean>;start:(request:string)=>void}>;

/** 📍️ The endpoint of the warm deriver of one checkout and implementation: a Unix socket in the temporary directory, a named pipe on Windows. */
export function warmDeriverEndpointV1(root:string,identity:string,platform:NodeJS.Platform=process.platform,temporary:string=tmpdir()):string{
 const name=policy.prefix+createHash("sha256").update(root+"\0"+identity).digest("hex").slice(0,16);
 return platform==="win32"?"\\\\.\\pipe\\"+name:join(temporary,name+".sock");
}

/** 🔥️ Wraps a deriver so its program questions reuse the previous program: before a question asked before, every row of the previous answer except the text of the program's own source files must still hold (`holds`, the planner's observation check), and only then is the store armed; the rows handed back are the previous rows that held plus the new ones, so the answer never claims fewer dependencies than it has. */
export function warmProgramDeriverV1(base:MemoDeriverV1,store:TypeScriptProgramReuseV1,memory:Map<string,Readonly<{rows:readonly MemoRowV1[];sources:ReadonlySet<string>}>>,holds:(rows:readonly MemoRowV1[],signal?:AbortSignal)=>Promise<boolean>):MemoDeriverV1{
 return async(row,signal)=>{
  const previous=memory.get(row.path),guard=previous?.rows.filter(fact=>!(fact.kind==="typescriptText"&&previous.sources.has(fact.path)))??[];
  const armed=previous!==undefined&&await holds(guard,signal);
  memory.delete(row.path);store.begin(armed);
  let answer:Awaited<ReturnType<MemoDeriverV1>>,done:ReturnType<TypeScriptProgramReuseV1["end"]>;try{answer=await base(row,signal);}finally{done=store.end();}
  const identity=(fact:MemoRowV1)=>fact.kind+"\u0000"+fact.path,seen=new Set(answer.rows.map(identity)),rows=armed?[...answer.rows,...guard.filter(fact=>!seen.has(identity(fact)))]:[...answer.rows];
  memory.set(row.path,{rows,sources:done.sources});
  return{fact:answer.fact,rows};
 };
}

/** 🛎️ Serves derivation requests on an endpoint, one at a time: a request of another identity is refused, `answer` writes the answer file, the server closes itself after `idleMilliseconds` without a request; a live server on the endpoint makes this one refuse to start, a stale socket file is replaced. */
export async function serveWarmDeriverV1(input:Readonly<{endpoint:string;identity:string;idleMilliseconds:number;answer:(request:string,signal:AbortSignal)=>Promise<void>;onClose?:()=>void}>):Promise<Readonly<{close:()=>Promise<void>}>>{
 let queue=Promise.resolve(),timer:ReturnType<typeof setTimeout>|undefined,closed=false;
 const server:Server=createServer(socket=>{
  let text="";socket.setEncoding("utf8");
  socket.on("data",chunk=>{text+=chunk;if(text.length>policy.maximumMessageBytes){socket.destroy();return;}const end=text.indexOf("\n");if(end<0)return;let message:{version?:unknown;identity?:unknown;request?:unknown};try{message=JSON.parse(text.slice(0,end));}catch{socket.destroy();return;}
   if(message.version!==1||typeof message.request!=="string"){socket.destroy();return;}
   if(message.identity!==input.identity){socket.end(JSON.stringify({version:1,refused:"identity"})+"\n");return;}
   const abort=new AbortController();socket.once("close",()=>abort.abort());rearm();
   queue=queue.then(async()=>{if(abort.signal.aborted)return;let reply:object;try{await input.answer(message.request as string,abort.signal);reply={version:1,answered:true};}catch{reply={version:1,answered:false};}if(!socket.destroyed)socket.end(JSON.stringify(reply)+"\n");rearm();});
  });
  socket.on("error",()=>{});
 });
 const close=()=>new Promise<void>(done=>{if(closed){done();return;}closed=true;if(timer)clearTimeout(timer);server.close(()=>{input.onClose?.();done();});});
 const rearm=()=>{if(timer)clearTimeout(timer);timer=setTimeout(()=>{void queue.then(close);},input.idleMilliseconds);timer.unref?.();};
 const listen=()=>new Promise<void>((done,refuse)=>{server.once("error",refuse);server.listen(input.endpoint,()=>{server.off("error",refuse);done();});});
 try{await listen();}catch(error){
  if((error as NodeJS.ErrnoException).code!=="EADDRINUSE"||process.platform==="win32")throw error;
  if(await warmEndpointAliveV1(input.endpoint))throw Object.assign(Error("A warm deriver already serves this endpoint / Ein warmer Ableiter bedient diesen Endpunkt bereits"),{code:"EADDRINUSE"});
  rmSync(input.endpoint,{force:true});await listen();
 }
 rearm();
 return{close};
}

/** 📡️ Tells whether a warm deriver listens on an endpoint. */
export function warmEndpointAliveV1(endpoint:string):Promise<boolean>{return new Promise<boolean>(done=>{const probe=connect(endpoint);probe.once("connect",()=>{probe.destroy();done(true);});probe.once("error",()=>done(false));});}

/** 📮️ Asks the warm deriver of an endpoint to answer a written request file: false when no server listens, it refuses the identity, fails, or does not answer within `answerMilliseconds`; never throws. */
export function askWarmDeriverV1(input:Readonly<{endpoint:string;identity:string;request:string;connectMilliseconds:number;answerMilliseconds:number;signal?:AbortSignal}>):Promise<boolean>{
 return new Promise<boolean>(done=>{
  let settled=false,text="";const socket:Socket=connect(input.endpoint);socket.setEncoding("utf8");
  const finish=(value:boolean)=>{if(settled)return;settled=true;clearTimeout(connecting);clearTimeout(answering);input.signal?.removeEventListener("abort",abort);socket.destroy();done(value);},abort=()=>finish(false);
  const connecting=setTimeout(()=>finish(false),input.connectMilliseconds),answering=setTimeout(()=>finish(false),input.connectMilliseconds+input.answerMilliseconds);
  input.signal?.addEventListener("abort",abort,{once:true});if(input.signal?.aborted){abort();return;}
  socket.once("connect",()=>{clearTimeout(connecting);socket.write(JSON.stringify({version:1,identity:input.identity,request:input.request})+"\n");});
  socket.on("data",chunk=>{text+=chunk;const end=text.indexOf("\n");if(end<0)return;try{finish((JSON.parse(text.slice(0,end)) as {answered?:unknown}).answered===true);}catch{finish(false);}});
  socket.once("error",()=>finish(false));socket.once("close",()=>finish(false));
 });
}

/** ✅️ Tells whether every file of a captured closure has, on disk, the bytes the closure names. */
export function warmClosureHoldsV1(closure:WarmClosureV1):boolean{
 for(const row of closure.sources){let bytes:Buffer;try{bytes=readFileSync(join(closure.root,row.path));}catch{return false;}if(createHash("sha256").update(bytes).digest("hex")!==row.sha256)return false;}
 return true;
}

/** 🧭️ The warm port of one run: its endpoint by checkout and identity; `start` writes the run's captured closure and a copy of the request (to warm the programs) into a private directory and starts the deriver detached, unless the run holds no captured closure. */
export function warmDeriverPortV1(root:string,identity:string,closure:WarmClosureV1|undefined,execPath:string=process.execPath,main:string=fileURLToPath(new URL("./⌨️main/🟦️.ts",import.meta.url))):WarmDeriverPortV1{
 const endpoint=warmDeriverEndpointV1(root,identity);
 return{
  ask:(request,signal)=>askWarmDeriverV1({endpoint,identity,request,connectMilliseconds:policy.connectMilliseconds,answerMilliseconds:policy.answerMilliseconds,...(signal?{signal}:{})}),
  start:request=>{
   if(!closure)return;
   try{
    const directory=mkdtempSync(join(tmpdir(),policy.prefix)),closureFile=join(directory,"closure.json"),warmup=join(directory,"request.json");
    writeFileSync(closureFile,JSON.stringify(closure),{mode:0o600});copyFileSync(request,warmup);
    spawn(execPath,[main,"--root",root,"--identity",identity,"--endpoint",endpoint,"--directory",directory],{cwd:root,stdio:"ignore",detached:true,windowsHide:true,env:Object.fromEntries(Object.entries(process.env).filter(([name])=>name!==GRAPH_MEMO_POLICY_V1.derive))}).unref();
   }catch{}
  },
 };
}

/** 🗂️ The captured closure of the running implementation, when this process runs one. */
export function capturedWarmClosureV1():WarmClosureV1|undefined{
 const selection=(globalThis as {__semioCapturedSelectionV1?:{physical?:{root?:unknown;sources?:unknown}}}).__semioCapturedSelectionV1?.physical;
 return selection&&typeof selection.root==="string"&&Array.isArray(selection.sources)?{root:selection.root,sources:selection.sources as WarmClosureV1["sources"]}:undefined;
}
