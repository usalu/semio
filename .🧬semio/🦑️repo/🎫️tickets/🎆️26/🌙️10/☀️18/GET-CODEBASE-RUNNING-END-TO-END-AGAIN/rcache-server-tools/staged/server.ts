import {createHash,timingSafeEqual} from "node:crypto";
import {mkdir,stat} from "node:fs/promises";
import {join} from "node:path";
import {parseRemoteCacheActionResultV1,validateRemoteCacheActionResultV1,type RemoteCacheValidationV1} from "../🔬️action-result/🟦️.ts";
import {acquireRemoteCacheLockV1,REMOTE_CACHE_COLLECTOR_LOCK_V1,REMOTE_CACHE_SERVER_LOCK_V1} from "../🧹️collector/🟦️.ts";
import {RemoteCacheStoreV1,type RemoteCacheKindV1,type RemoteCachePutModeV1,type RemoteCacheStoreEventV1} from "../💾️store/🟦️.ts";

/** 🔐️ Authorizes bearer tokens: with neither token the server is open, a read token allows GET and HEAD, a write token allows everything. */
export type RemoteCacheTokensV1=Readonly<{read?:string|undefined;write?:string|undefined}>;

/** 🧮️ Counts what one server did per keyspace since it started; `stale` counts action results that were dropped on a read because their blobs had left the store. */
export type RemoteCacheCountersV1=Record<RemoteCacheKindV1,{hits:number;misses:number;puts:number;conflicts:number;rejected:number;stale:number}>;

/** 📊️ The JSON document `GET /status` answers. */
export type RemoteCacheServerStatusV1=Readonly<{version:1;protocols:readonly ["bazel-http","nx-self-hosted"];entries:number;bytes:number;reservedBytes:number;maximumBytes:number;maximumEntryBytes:number;readOnly:boolean;validation:RemoteCacheValidationV1;counters:RemoteCacheCountersV1;startedAt:string}>;

/** 📜️ One answered request, published after the response head is decided. */
export type RemoteCacheRequestEventV1=Readonly<{stage:"request";method:string;path:string;status:number;milliseconds:number}>;

/** 🔌️ Everything the web-standard handler needs. */
export type RemoteCacheHandlerOptionsV1=Readonly<{store:RemoteCacheStoreV1;tokens:RemoteCacheTokensV1;defaultInstance:string;readOnly?:boolean;validation?:RemoteCacheValidationV1;maximumActionResultBytes?:number;startedAt?:Date;onRequest?:(event:RemoteCacheRequestEventV1)=>void}>;

const RESERVED=new Set(["ac","cas","v1","healthz","status"]),INSTANCE=/^[a-z0-9][a-z0-9._-]{0,63}$/u,DIGEST=/^[0-9a-f]{64}$/u,KEY=/^[a-z0-9_-]{1,128}$/u,TREE_BYTES=16*1048576,
 text=(status:number,message:string,headers:Record<string,string>={})=>new Response(message+"\n",{status,headers:{"content-type":"text/plain",...headers}}),
 digest=(value:string)=>createHash("sha256").update(value).digest(),
 row=()=>({hits:0,misses:0,puts:0,conflicts:0,rejected:0,stale:0});

/** 🚰️ A request body read through its reader, because Bun's own stream iterator is not available for request bodies: `chunks` feeds the store and `drain` discards what is left, up to a bound, so a reply sent before the body was read is not lost to a connection reset. */
export type RemoteCacheBodyV1=Readonly<{chunks:AsyncGenerator<Uint8Array>;drain:(limit:number)=>Promise<void>}>;

/** 🚰️ Wraps a request body stream. */
export function remoteCacheBodyV1(stream:ReadableStream<Uint8Array>|null):RemoteCacheBodyV1{
 if(!stream)return Object.freeze({chunks:(async function*(){})(),drain:async()=>{}});
 const reader=stream.getReader();let finished=false;
 async function* chunks():AsyncGenerator<Uint8Array>{for(;;){const {done,value}=await reader.read();if(done){finished=true;return;}yield value;}}
 const drain=async(limit:number):Promise<void>=>{
  let seen=0;
  try{while(!finished){const {done,value}=await reader.read();if(done){finished=true;return;}seen+=value.byteLength;if(seen>limit){await reader.cancel();return;}}}catch{}
 };
 return Object.freeze({chunks:chunks(),drain});
}

/** 🪪️ Compares two secrets in constant time. */
export function remoteCacheTokenMatchesV1(expected:string,presented:string):boolean{return timingSafeEqual(digest(expected),digest(presented));}

/** 🧭️ Resolves a request path to its namespace, keyspace and key, or `undefined` for every path outside the two protocols. */
export function parseRemoteCachePathV1(pathname:string,defaultInstance:string):Readonly<{namespace:string;kind:RemoteCacheKindV1;key:string}>|undefined{
 const segments=pathname.split("/").slice(1);let namespace=defaultInstance;
 if(segments.length>0&&!RESERVED.has(segments[0]!)){namespace=segments.shift()!;if(!INSTANCE.test(namespace))return undefined;}
 if((segments[0]==="ac"||segments[0]==="cas")&&segments.length===2)return DIGEST.test(segments[1]!)?{namespace,kind:segments[0],key:segments[1]!}:undefined;
 if(segments[0]==="v1"&&segments[1]==="cache"&&segments.length===3)return KEY.test(segments[2]!)?{namespace,kind:"nx",key:segments[2]!}:undefined;
 return undefined;
}

/** 🏗️ Builds the protocol handler as a pure web-standard function, so any HTTP host can serve it: Bazel's HTTP cache (`/ac/<sha256>`, `/cas/<sha256>`) and the Nx self-hosted cache API (`/v1/cache/<hash>`), each optionally under a `/<instance>` prefix. Blobs are verified against their digest; action results are validated (and, when complete, answered only while every blob they name is still stored); a read-only handler refuses every write. */
export function createRemoteCacheHandlerV1(options:RemoteCacheHandlerOptionsV1):(request:Request)=>Promise<Response>{
 const counters:RemoteCacheCountersV1={ac:row(),cas:row(),nx:row()},validation=options.validation??"complete",readOnly=options.readOnly===true,limit=options.maximumActionResultBytes??4194304,
  startedAt=(options.startedAt??new Date()).toISOString(),open=options.tokens.read===undefined&&options.tokens.write===undefined;
 const authorize=(request:Request,write:boolean):Response|undefined=>{
  if(open)return undefined;
  const header=request.headers.get("authorization"),presented=header?.startsWith("Bearer ")?header.slice(7):undefined;
  if(presented===undefined||presented==="")return text(401,"Unauthorized",{"www-authenticate":"Bearer"});
  const writer=options.tokens.write!==undefined&&remoteCacheTokenMatchesV1(options.tokens.write,presented),reader=options.tokens.read!==undefined&&remoteCacheTokenMatchesV1(options.tokens.read,presented);
  if(!writer&&!reader)return text(401,"Unauthorized",{"www-authenticate":"Bearer"});
  return write&&!writer?text(403,"Forbidden: read-only token"):undefined;
 };
 const mode=(kind:RemoteCacheKindV1):RemoteCachePutModeV1=>kind==="ac"?"replace":kind==="cas"?"content":"keep";
 const blobs=(namespace:string)=>({has:(hash:string)=>options.store.has(namespace,"cas",hash),read:(hash:string)=>options.store.readBytes(namespace,"cas",hash,TREE_BYTES)});
 const readResult=async(namespace:string,key:string,method:string):Promise<Response>=>{
  const counter=counters.ac;
  if(validation!=="complete"){
   const entry=await options.store.read(namespace,"ac",key);
   if(!entry){counter.misses++;return text(404,"Not found");}
   counter.hits++;return new Response(method==="GET"?entry.size===0?"":Bun.file(entry.path):null,{headers:{"content-type":"application/octet-stream","content-length":String(entry.size)}});
  }
  const bytes=await options.store.readBytes(namespace,"ac",key,limit);
  if(bytes===undefined){counter.misses++;return text(404,"Not found");}
  try{
   const parsed=parseRemoteCacheActionResultV1(bytes);
   for(const value of[...parsed.references,...parsed.trees])if(!options.store.has(namespace,"cas",value.hash))throw Error("blob left the store");
  }catch{await options.store.remove(namespace,"ac",key);counter.stale++;counter.misses++;return text(404,"Not found");}
  counter.hits++;return new Response((method==="GET"?bytes:null) as BodyInit|null,{headers:{"content-type":"application/octet-stream","content-length":String(bytes.byteLength)}});
 };
 const answer=async(request:Request,body:RemoteCacheBodyV1):Promise<Response>=>{
  const url=new URL(request.url),method=request.method;
  if(url.pathname==="/healthz")return method==="GET"||method==="HEAD"?new Response(method==="GET"?'{"status":"ok"}\n':null,{headers:{"content-type":"application/json"}}):text(405,"Method not allowed",{allow:"GET, HEAD"});
  if(url.pathname==="/status"){
   const denied=authorize(request,false);if(denied)return denied;
   const usage=options.store.usage(),status:RemoteCacheServerStatusV1={version:1,protocols:["bazel-http","nx-self-hosted"],entries:usage.entries,bytes:usage.bytes,reservedBytes:usage.reservedBytes,maximumBytes:usage.maximumBytes,maximumEntryBytes:usage.maximumEntryBytes,readOnly,validation,counters,startedAt};
   return Response.json(status);
  }
  const address=parseRemoteCachePathV1(url.pathname,options.defaultInstance);
  if(!address)return text(404,"Not found");
  if(method==="GET"||method==="HEAD"){
   const denied=authorize(request,false);if(denied)return denied;
   if(address.kind==="ac")return readResult(address.namespace,address.key,method);
   const entry=await options.store.read(address.namespace,address.kind,address.key),counter=counters[address.kind];
   if(!entry){counter.misses++;return text(404,"Not found");}
   counter.hits++;const headers={"content-type":"application/octet-stream","content-length":String(entry.size)};
   return new Response(method==="GET"?entry.size===0?"":Bun.file(entry.path):null,{headers});
  }
  if(method!=="PUT")return text(405,"Method not allowed",{allow:"GET, HEAD, PUT"});
  const denied=authorize(request,true);if(denied)return denied;
  if(readOnly)return text(403,"Forbidden: this cache is read-only");
  const counter=counters[address.kind],length=request.headers.get("content-length");
  if(address.kind==="nx"&&length===null)return text(411,"Content-Length required");
  const declared=length===null?undefined:Number(length);
  if(declared!==undefined&&!Number.isSafeInteger(declared))return text(400,"Content-Length invalid");
  let source:AsyncIterable<Uint8Array>=body.chunks,known=declared;
  if(address.kind==="ac"&&validation!=="off"){
   if(declared!==undefined&&declared>limit){counter.rejected++;return text(413,"Action result exceeds the maximum size");}
   const parts:Uint8Array[]=[];let size=0;
   try{for await(const chunk of body.chunks){size+=chunk.byteLength;if(size>limit){counter.rejected++;return text(413,"Action result exceeds the maximum size");}parts.push(chunk);}}catch{return text(499,"Client closed request");}
   if(declared!==undefined&&size!==declared){counter.rejected++;return text(400,"Content-Length mismatch");}
   const bytes=new Uint8Array(size);let offset=0;for(const part of parts){bytes.set(part,offset);offset+=part.byteLength;}
   const verdict=await validateRemoteCacheActionResultV1(bytes,validation,blobs(address.namespace));
   if(!verdict.valid){counter.rejected++;return text(400,`Invalid action result: ${verdict.reason}`);}
   source=(async function*(){yield bytes;})();known=size;
  }
  const outcome=await options.store.put(address.namespace,address.kind,address.key,source,{mode:mode(address.kind),verify:address.kind==="cas",declaredLength:known});
  switch(outcome){
   case "created":case "replaced":counter.puts++;return text(200,"Stored");
   case "exists":if(address.kind==="nx"){counter.conflicts++;return text(409,"Conflict: cannot overwrite an existing cache entry");}return text(200,"Stored");
   case "digest-mismatch":counter.rejected++;return text(400,"Digest mismatch");
   case "length-mismatch":counter.rejected++;return text(400,"Content-Length mismatch");
   case "too-large":counter.rejected++;return text(413,"Entry exceeds the maximum entry size");
   case "capacity":counter.rejected++;return text(507,"Insufficient storage",{"retry-after":"5"});
   case "aborted":return text(499,"Client closed request");
  }
 };
 return async request=>{
  const started=performance.now(),body=remoteCacheBodyV1(request.body),response=await answer(request,body);
  await body.drain(options.store.usage().maximumEntryBytes);
  options.onRequest?.({stage:"request",method:request.method,path:new URL(request.url).pathname,status:response.status,milliseconds:Math.round(performance.now()-started)});
  return response;
 };
}

/** 🔒️ The certificate chain and private key of an optional TLS listener, as file paths. */
export type RemoteCacheTlsV1=Readonly<{certificate:string;key:string}>;

/** ⚙️ Authors one running server: where to listen, what to store, who may use it and what it checks. */
export type RemoteCacheServerOptionsV1=Readonly<{directory:string;host:string;port:number;maximumBytes:number;maximumEntryBytes:number;maximumAgeDays:number;defaultInstance:string;tokens:RemoteCacheTokensV1;readOnly?:boolean;tls?:RemoteCacheTlsV1|undefined;validation?:RemoteCacheValidationV1;maximumActionResultBytes?:number;maintenanceSeconds?:number;signal?:AbortSignal;onEvent?:(event:RemoteCacheStoreEventV1|RemoteCacheRequestEventV1|Readonly<{stage:"listening";url:string}>|Readonly<{stage:"stopped"}>)=>void}>;

/** 🛰️ A started server: its address, a live status and an orderly stop. */
export interface RemoteCacheServerV1{readonly url:string;readonly port:number;readonly store:RemoteCacheStoreV1;readonly stopped:Promise<void>;stop():Promise<void>}

/** 🚧️ Raised when another living server already owns the store directory. */
export class RemoteCacheDirectoryBusyV1 extends Error{constructor(readonly directory:string){super(`Another remote cache server uses ${directory}`);}}

const LOOPBACK=new Set(["127.0.0.1","localhost","::1","[::1]"]);

/** 🚀️ Opens the store and serves the handler with Bun, over TLS when a certificate is given; refuses to listen beyond loopback without a token, refuses a directory another server owns, keeps the store in step with outside collectors, and stops when `signal` aborts. */
export async function startRemoteCacheServerV1(options:RemoteCacheServerOptionsV1):Promise<RemoteCacheServerV1>{
 if(!LOOPBACK.has(options.host)&&options.tokens.read===undefined&&options.tokens.write===undefined)throw Error("A token is required to listen beyond loopback");
 if(!INSTANCE.test(options.defaultInstance)||RESERVED.has(options.defaultInstance))throw Error("Valid default instance required");
 if(options.tls){for(const path of[options.tls.certificate,options.tls.key])if(!(await stat(path).catch(()=>undefined))?.isFile())throw Error(`TLS file required: ${path}`);}
 const root=join(options.directory,"v1");await mkdir(root,{recursive:true});
 const owner=await acquireRemoteCacheLockV1(root,REMOTE_CACHE_SERVER_LOCK_V1);
 if(owner.held)throw new RemoteCacheDirectoryBusyV1(options.directory);
 let store:RemoteCacheStoreV1|undefined,timer:ReturnType<typeof setInterval>|undefined;
 try{
  store=await RemoteCacheStoreV1.open({directory:options.directory,maximumBytes:options.maximumBytes,maximumEntryBytes:options.maximumEntryBytes,maximumAgeMilliseconds:options.maximumAgeDays*86400000,...(options.signal?{signal:options.signal}:{}),...(options.onEvent?{onEvent:options.onEvent}:{})});
  const opened=store,handler=createRemoteCacheHandlerV1({store:opened,tokens:options.tokens,defaultInstance:options.defaultInstance,...(options.readOnly===undefined?{}:{readOnly:options.readOnly}),...(options.validation===undefined?{}:{validation:options.validation}),...(options.maximumActionResultBytes===undefined?{}:{maximumActionResultBytes:options.maximumActionResultBytes}),...(options.onEvent?{onRequest:options.onEvent}:{})});
  const server=Bun.serve({hostname:options.host,port:options.port,idleTimeout:60,maxRequestBodySize:options.maximumEntryBytes*2,...(options.tls?{tls:{cert:Bun.file(options.tls.certificate),key:Bun.file(options.tls.key)}}:{}),fetch:request=>handler(request)});
  const host=options.host.includes(":")&&!options.host.startsWith("[")?`[${options.host}]`:options.host,port=server.port??options.port,url=`${options.tls?"https":"http"}://${host}:${port}`;
  let finish!:()=>void;const stopped=new Promise<void>(resolve=>{finish=resolve;});let closing:Promise<void>|undefined;
  const stop=():Promise<void>=>closing??=(async()=>{if(timer)clearInterval(timer);await server.stop(true);await opened.close();await owner.release();options.onEvent?.({stage:"stopped"});finish();})();
  const maintenance=(options.maintenanceSeconds??0)*1000;
  if(maintenance>0){
   let running=false;
   timer=setInterval(()=>{
    if(running)return;running=true;
    void(async()=>{const lock=await acquireRemoteCacheLockV1(root,REMOTE_CACHE_COLLECTOR_LOCK_V1);if(lock.held)return;try{await opened.refresh();}finally{await lock.release();}})().catch(()=>{}).finally(()=>{running=false;});
   },maintenance);
   timer.unref();
  }
  if(options.signal){if(options.signal.aborted)await stop();else options.signal.addEventListener("abort",()=>{void stop();},{once:true});}
  options.onEvent?.({stage:"listening",url});
  return Object.freeze({url,port,store:opened,stopped,stop});
 }catch(error){
  await store?.close().catch(()=>{});await owner.release();throw error;
 }
}
