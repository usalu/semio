/** 🏭 Resident executor: one process that keeps its lease/image custody open and runs one repo command after another inside it, with the process-wide state (environment, directory, exit code) restored after every command. */
import {createInterface} from "node:readline";

const AUTHORITY=/Undeclared original Repo build selection|Original Repo build caller required|Undeclared original Repo-to-Nx child selection|Undeclared original Nx child selection/u;

/** 🧽 Runs `body` with the request's environment patch and directory applied and restores everything afterwards; resolves the exit code the body left in `process.exitCode`. */
export async function isolateRequestV1(request,body,state=process){
 const savedEnvironment={...state.env},savedDirectory=state.cwd(),savedExit=state.exitCode;
 state.exitCode=undefined;
 try{
  for(const[key,value]of Object.entries(request.env??{})){if(value===null)delete state.env[key];else state.env[key]=value;}
  if(request.cwd)state.chdir(request.cwd);
  await body();
  return Number(state.exitCode??0);
 }finally{
  for(const key of Object.keys(state.env))if(!(key in savedEnvironment))delete state.env[key];
  for(const[key,value]of Object.entries(savedEnvironment))if(state.env[key]!==value)state.env[key]=value;
  try{state.chdir(savedDirectory);}catch{}
  state.exitCode=savedExit;
 }
}

/** 🏁 The marker an executor writes to both output streams after a command; the pool delivers every byte before it and none after. */
export const executorEndMarkerV1=id=>"\u0000semio-end:"+id+"\u0000";

function fence(state,id){
 const marker=executorEndMarkerV1(id),sink=state.console??console;
 sink.log(marker);sink.error(marker);
 return new Promise(resolve=>setImmediate(resolve));
}

/** 🏭 Serves requests from `lines` (one JSON request per line) until the input ends, the age or request limit is reached, or custody is refused. `openSession()` opens the custody once, `run(request,session,signal)` executes one command inside it. */
export async function serveExecutorV1({lines,write,openSession,run,limits,now=()=>Date.now(),state=process,onFatal=()=>undefined}){
 const born=now();let served=0,session,current;
 const emit=frame=>write(JSON.stringify(frame)+"\n");
 try{session=await openSession();}catch(error){emit({t:"fatal",reason:"custody",message:String(error?.message??error)});return{served,reason:"custody"};}
 emit({t:"ready"});
 state.on?.("SIGINT",()=>current?.abort(Object.assign(new Error("cancelled"),{code:"cancelled"})));
 const iterator=lines[Symbol.asyncIterator]();
 for(;;){
  let timer;
  const idle=limits.idleMilliseconds?new Promise(resolve=>{timer=setTimeout(()=>resolve("idle"),limits.idleMilliseconds);}):new Promise(()=>undefined);
  const next=await Promise.race([iterator.next(),idle]);
  clearTimeout(timer);
  if(next==="idle"||next.done)break;
  const line=next.value;
  if(!line.trim())continue;
  let request;
  try{request=JSON.parse(line);}catch{emit({t:"fatal",reason:"protocol",message:"request is not JSON"});break;}
  if(request.t==="stop")break;
  const controller=new AbortController();current=controller;emit({t:"started",id:request.id});
  let exit=0,refusal,refusalCode,refusalFields;
  try{
   exit=await isolateRequestV1(request,()=>session.run(control=>run(request,control,controller.signal),{signal:controller.signal}),state);
   if(controller.signal.aborted)exit=130;
  }catch(error){
   const message=String(error?.message??error);
   if(controller.signal.aborted)exit=130;
   else if(error?.name==="ServiceRefusalV1"){exit=70;refusal="service";refusalCode=error.code;refusalFields=error.fields;}
   else if(AUTHORITY.test(message)){exit=70;refusal="authority";}
   else{exit=1;state.stderr?.write?.(message+"\n");}
  }finally{current=undefined;}
  served++;
  await fence(state,request.id);
  emit({t:"done",id:request.id,exit,...(refusal?{refusal}:{}),...(refusalCode?{code:refusalCode,fields:refusalFields}:{})});
  if(refusal==="authority")break;
  if(served>=limits.maximumRequests||now()-born>=limits.maximumAgeMilliseconds)break;
 }
 try{await session.close();}catch(error){onFatal(error);}
 return{served,reason:"ended"};
}

/** 📖 Splits a readable byte stream into lines. */
export function readLinesV1(stream){return createInterface({input:stream,crlfDelay:Infinity});}
