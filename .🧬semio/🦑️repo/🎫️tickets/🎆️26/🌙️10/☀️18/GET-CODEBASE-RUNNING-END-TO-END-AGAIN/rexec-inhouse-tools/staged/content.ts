import type {RemoteExecutionDigestV1} from "../🟦️.ts";

/** 🗄️ Where the protocol's blobs and action results live: the shared content-addressed store. Scheduler and workers use this interface only, so a store can be swapped without touching either. */
export interface RemoteExecutionContentV1{
 has(instance:string,hash:string,signal?:AbortSignal):Promise<boolean>;
 missing(instance:string,digests:readonly RemoteExecutionDigestV1[],signal?:AbortSignal):Promise<RemoteExecutionDigestV1[]>;
 read(instance:string,digest:RemoteExecutionDigestV1,signal?:AbortSignal):Promise<Uint8Array|undefined>;
 stream(instance:string,digest:RemoteExecutionDigestV1,signal?:AbortSignal):Promise<AsyncGenerator<Uint8Array>|undefined>;
 write(instance:string,digest:RemoteExecutionDigestV1,source:Uint8Array|AsyncIterable<Uint8Array>,signal?:AbortSignal):Promise<void>;
 readActionResult(instance:string,hash:string,signal?:AbortSignal):Promise<Uint8Array|undefined>;
 writeActionResult(instance:string,hash:string,bytes:Uint8Array,signal?:AbortSignal):Promise<void>;
}

/** 🚧️ The store refused or could not take the request; `code` is a gRPC status code. */
export class RemoteExecutionContentErrorV1 extends Error{constructor(readonly code:number,message:string){super(message);}}

/** ⚙️ How to reach the shared store over its HTTP protocol (`/<instance>/cas/<sha256>`, `/<instance>/ac/<sha256>`, bearer token). */
export type RemoteExecutionHttpContentOptionsV1=Readonly<{endpoint:string;token?:string|undefined;concurrency?:number;timeoutSeconds?:number;retries?:number}>;

const EMPTY="e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",INSTANCE=/^[a-z0-9][a-z0-9._-]{0,63}$/u,TRANSIENT=new Set([408,429,500,502,503,504]);

async function* chunksOf(body:ReadableStream<Uint8Array>):AsyncGenerator<Uint8Array>{
 const reader=body.getReader();
 try{for(;;){const {done,value}=await reader.read();if(done)return;yield value;}}finally{await reader.cancel().catch(()=>{});}
}

/** 🌐️ The store of the repository's own cache server (and any server speaking Bazel's HTTP cache protocol), as `RemoteExecutionContentV1`. Retries transient failures, bounds parallel existence checks, sends the declared length of every blob and streams large blobs in both directions. */
export function httpRemoteExecutionContentV1(options:RemoteExecutionHttpContentOptionsV1):RemoteExecutionContentV1{
 const base=options.endpoint.replace(/\/+$/u,""),limit=options.concurrency??32,retries=options.retries??3,timeout=(options.timeoutSeconds??60)*1000,headers=(extra:Record<string,string>={}):Record<string,string>=>({...(options.token?{authorization:`Bearer ${options.token}`}:{}),...extra});
 const url=(instance:string,kind:"ac"|"cas",hash:string):string=>{
  if(instance!==""&&!INSTANCE.test(instance))throw new RemoteExecutionContentErrorV1(3,`Valid instance name required: ${instance}`);
  return `${base}${instance===""?"":"/"+instance}/${kind}/${hash}`;
 };
 const send=async(method:string,target:string,signal:AbortSignal|undefined,init:Partial<RequestInit>={},retry=true):Promise<Response>=>{
  let last:unknown;
  for(let attempt=0;attempt<=(retry?retries:0);attempt++){
   try{
    const response=await fetch(target,{method,...init,headers:{...headers(),...(init.headers as Record<string,string>|undefined)},signal:signal?AbortSignal.any([signal,AbortSignal.timeout(timeout)]):AbortSignal.timeout(timeout)});
    if(TRANSIENT.has(response.status)&&attempt<retries){await response.body?.cancel();await Bun.sleep(50*2**attempt);continue;}
    return response;
   }catch(error){
    if(signal?.aborted)throw error;
    last=error;if(attempt<retries)await Bun.sleep(50*2**attempt);
   }
  }
  throw new RemoteExecutionContentErrorV1(14,`Content store unreachable: ${base}: ${last instanceof Error?last.message:String(last)}`);
 };
 const refuse=(response:Response,what:string):never=>{
  const code=response.status===401?16:response.status===403?7:response.status===507?8:response.status===413?8:response.status===400?3:14;
  throw new RemoteExecutionContentErrorV1(code,`Content store answered ${response.status} for ${what}`);
 };
 const content:RemoteExecutionContentV1={
  async has(instance,hash,signal){
   if(hash===EMPTY)return true;
   const response=await send("HEAD",url(instance,"cas",hash),signal);await response.body?.cancel();
   if(response.status===200)return true;
   if(response.status===404)return false;
   return refuse(response,`cas/${hash}`);
  },
  async missing(instance,digests,signal){
   const out:RemoteExecutionDigestV1[]=new Array(digests.length),found:boolean[]=new Array(digests.length);let next=0;
   await Promise.all(Array.from({length:Math.min(limit,digests.length)},async()=>{for(;;){const index=next++;if(index>=digests.length)return;found[index]=await content.has(instance,digests[index]!.hash,signal);}}));
   let count=0;
   digests.forEach((digest,index)=>{if(!found[index])out[count++]=digest;});
   return out.slice(0,count);
  },
  async read(instance,digest,signal){
   if(digest.size===0&&digest.hash===EMPTY)return new Uint8Array(0);
   const response=await send("GET",url(instance,"cas",digest.hash),signal);
   if(response.status===404){await response.body?.cancel();return undefined;}
   if(response.status!==200)return refuse(response,`cas/${digest.hash}`);
   return new Uint8Array(await response.arrayBuffer());
  },
  async stream(instance,digest,signal){
   if(digest.size===0&&digest.hash===EMPTY)return (async function*(){})();
   const response=await send("GET",url(instance,"cas",digest.hash),signal);
   if(response.status===404){await response.body?.cancel();return undefined;}
   if(response.status!==200)return refuse(response,`cas/${digest.hash}`);
   return response.body?chunksOf(response.body):(async function*(){})();
  },
  async write(instance,digest,source,signal){
   if(digest.size===0&&digest.hash===EMPTY)return;
   const target=url(instance,"cas",digest.hash),request={"content-length":String(digest.size),"content-type":"application/octet-stream"};
   if(source instanceof Uint8Array){const response=await send("PUT",target,signal,{body:source as unknown as BodyInit,headers:request});await response.body?.cancel();if(response.status!==200)refuse(response,`cas/${digest.hash}`);return;}
   const iterator=source[Symbol.asyncIterator](),body=new ReadableStream<Uint8Array>({async pull(controller){const next=await iterator.next();if(next.done)controller.close();else controller.enqueue(next.value);},cancel(){void iterator.return?.();}});
   const response=await send("PUT",target,signal,{body,headers:request,duplex:"half"} as RequestInit,false);await response.body?.cancel();
   if(response.status!==200)refuse(response,`cas/${digest.hash}`);
  },
  async readActionResult(instance,hash,signal){
   const response=await send("GET",url(instance,"ac",hash),signal);
   if(response.status===404){await response.body?.cancel();return undefined;}
   if(response.status!==200)return refuse(response,`ac/${hash}`);
   return new Uint8Array(await response.arrayBuffer());
  },
  async writeActionResult(instance,hash,bytes,signal){
   const response=await send("PUT",url(instance,"ac",hash),signal,{body:bytes as unknown as BodyInit,headers:{"content-length":String(bytes.length),"content-type":"application/octet-stream"}});await response.body?.cancel();
   if(response.status!==200)refuse(response,`ac/${hash}`);
  },
 };
 return Object.freeze(content);
}
