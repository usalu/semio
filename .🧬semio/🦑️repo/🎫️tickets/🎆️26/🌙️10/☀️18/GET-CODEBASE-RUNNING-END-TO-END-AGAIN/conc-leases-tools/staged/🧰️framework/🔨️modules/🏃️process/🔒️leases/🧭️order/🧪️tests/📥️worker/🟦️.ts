import {acquireQueuedResourceLease,acquireResourceLease,LeaseOrderErrorV1,withResourceLeases,type ResourceLease} from "../../../🟦️.ts";

const spec=JSON.parse(process.argv[2]!) as Record<string,any>;
const task=new AbortController().signal;
const send=(line:string):void=>{process.stdout.write(line+"\n");};
const reader=Bun.stdin.stream().getReader(),decoder=new TextDecoder();
let buffered="";
async function line():Promise<string>{
 for(;;){
  const at=buffered.indexOf("\n");
  if(at>=0){const text=buffered.slice(0,at);buffered=buffered.slice(at+1);return text;}
  const chunk=await reader.read();if(chunk.done)return "";
  buffered+=decoder.decode(chunk.value);
 }
}
const take=(resource:string,mode:"shared"|"exclusive",queued:boolean,timeoutMs?:number):Promise<ResourceLease>=>{
 const options={directory:spec.directory,resource,mode,signal:task,...timeoutMs===undefined?{}:{timeoutMs}};
 return queued?acquireQueuedResourceLease({...options,owner:"w"+process.pid,...mode==="shared"?{sharedCohort:true}:{}}):acquireResourceLease(options);
};
const failure=(error:unknown)=>error instanceof LeaseOrderErrorV1?{ok:false,code:error.code,kind:error.violation.kind,message:error.message}:{ok:false,code:"OTHER",message:String(error)};

if(spec.mode==="hold"){
 const held=await take(spec.resource,spec.access,spec.queued===true);send("HELD");
 await line();held.release();send("DONE");
}else if(spec.mode==="wait"){
 const started=Date.now();
 try{const held=await take(spec.resource,spec.access,spec.queued===true,spec.timeoutMs);send("ACQUIRED "+(Date.now()-started));held.release();}catch(error){send("FAILED "+JSON.stringify(failure(error)));}
}else if(spec.mode==="chain"){
 const first=await take(spec.first.resource,spec.first.access,spec.queued===true);send("HELD");
 await line();
 const started=Date.now();let second:ResourceLease|undefined,result:Record<string,unknown>;
 try{second=await take(spec.second.resource,spec.second.access,spec.queued===true);result={ok:true,elapsedMs:Date.now()-started};}catch(error){result={...failure(error),elapsedMs:Date.now()-started};}
 send("RESULT "+JSON.stringify(result));
 if(!result.ok)first.release();
 await line();second?.release();if(result.ok)first.release();send("DONE");
}else if(spec.mode==="probe"){
 try{const held=await take(spec.resource,spec.access,false,spec.timeoutMs);send("RESULT "+JSON.stringify({ok:true}));held.release();}catch(error){send("RESULT "+JSON.stringify(failure(error)));}
}else if(spec.mode==="readers"){
 const end=Date.now()+spec.durationMs;let count=0;
 await new Promise(resolve=>setTimeout(resolve,spec.offsetMs));
 send("READY");
 while(Date.now()<end){await withResourceLeases({directory:spec.directory,signal:new AbortController().signal,resources:[{resource:spec.resource,mode:"shared"}]},()=>new Promise<void>(resolve=>setTimeout(resolve,spec.holdMs)));count++;}
 send("DONE "+count);
}
process.exit(0);
