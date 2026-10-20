import {createHash,timingSafeEqual} from "node:crypto";
import {RemoteCacheStoreV1,type RemoteCacheKindV1,type RemoteCachePutModeV1,type RemoteCacheStoreEventV1} from "../💾️store/🟦️.ts";

/** 🔐️ Authorizes bearer tokens: with neither token the server is open, a read token allows GET and HEAD, a write token allows everything. */
export type RemoteCacheTokensV1=Readonly<{read?:string|undefined;write?:string|undefined}>;

/** 🧮️ Counts what one server did per keyspace since it started. */
export type RemoteCacheCountersV1=Record<RemoteCacheKindV1,{hits:number;misses:number;puts:number;conflicts:number;rejected:number}>;

/** 📊️ The JSON document `GET /status` answers. */
export type RemoteCacheServerStatusV1=Readonly<{version:1;protocols:readonly ["bazel-http","nx-self-hosted"];entries:number;bytes:number;reservedBytes:number;maximumBytes:number;maximumEntryBytes:number;counters:RemoteCacheCountersV1;startedAt:string}>;

/** 📜️ One answered request, published after the response head is decided. */
export type RemoteCacheRequestEventV1=Readonly<{stage:"request";method:string;path:string;status:number;milliseconds:number}>;

/** 🔌️ Everything the web-standard handler needs. */
export type RemoteCacheHandlerOptionsV1=Readonly<{store:RemoteCacheStoreV1;tokens:RemoteCacheTokensV1;defaultInstance:string;startedAt?:Date;onRequest?:(event:RemoteCacheRequestEventV1)=>void}>;

const RESERVED=new Set(["ac","cas","v1","healthz","status"]),INSTANCE=/^[a-z0-9][a-z0-9._-]{0,63}$/u,DIGEST=/^[0-9a-f]{64}$/u,KEY=/^[a-z0-9_-]{1,128}$/u,
 text=(status:number,message:string,headers:Record<string,string>={})=>new Response(message+"\n",{status,headers:{"content-type":"text/plain",...headers}}),
 digest=(value:string)=>createHash("sha256").update(value).digest();

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

/** 🏗️ Builds the protocol handler as a pure web-standard function, so any HTTP host can serve it: Bazel's HTTP cache (`/ac/<sha256>`, `/cas/<sha256>`) and the Nx self-hosted cache API (`/v1/cache/<hash>`), each optionally under a `/<instance>` prefix. */
export function createRemoteCacheHandlerV1(options:RemoteCacheHandlerOptionsV1):(request:Request)=>Promise<Response>{
 const counters:RemoteCacheCountersV1={ac:{hits:0,misses:0,puts:0,conflicts:0,rejected:0},cas:{hits:0,misses:0,puts:0,conflicts:0,rejected:0},nx:{hits:0,misses:0,puts:0,conflicts:0,rejected:0}},
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
 const answer=async(request:Request,body:RemoteCacheBodyV1):Promise<Response>=>{
  const url=new URL(request.url),method=request.method;
  if(url.pathname==="/healthz")return method==="GET"||method==="HEAD"?new Response(method==="GET"?'{"status":"ok"}\n':null,{headers:{"content-type":"application/json"}}):text(405,"Method not allowed",{allow:"GET, HEAD"});
  if(url.pathname==="/status"){
   const denied=authorize(request,false);if(denied)return denied;
   const usage=options.store.usage(),status:RemoteCacheServerStatusV1={version:1,protocols:["bazel-http","nx-self-hosted"],entries:usage.entries,bytes:usage.bytes,reservedBytes:usage.reservedBytes,maximumBytes:usage.maximumBytes,maximumEntryBytes:usage.maximumEntryBytes,counters,startedAt};
   return Response.json(status);
  }
  const address=parseRemoteCachePathV1(url.pathname,options.defaultInstance);
  if(!address)return text(404,"Not found");
  if(method==="GET"||method==="HEAD"){
   const denied=authorize(request,false);if(denied)return denied;
   const entry=options.store.read(address.namespace,address.kind,address.key),row=counters[address.kind];
   if(!entry){row.misses++;return text(404,"Not found");}
   row.hits++;const headers={"content-type":"application/octet-stream","content-length":String(entry.size)};
   return new Response(method==="GET"?entry.size===0?"":Bun.file(entry.path):null,{headers});
  }
  if(method!=="PUT")return text(405,"Method not allowed",{allow:"GET, HEAD, PUT"});
  const denied=authorize(request,true);if(denied)return denied;
  const row=counters[address.kind],length=request.headers.get("content-length");
  if(address.kind==="nx"&&length===null)return text(411,"Content-Length required");
  const declared=length===null?undefined:Number(length);
  if(declared!==undefined&&!Number.isSafeInteger(declared))return text(400,"Content-Length invalid");
  const outcome=await options.store.put(address.namespace,address.kind,address.key,body.chunks,{mode:mode(address.kind),verify:address.kind==="cas",declaredLength:declared});
  switch(outcome){
   case "created":case "replaced":row.puts++;return text(200,"Stored");
   case "exists":if(address.kind==="nx"){row.conflicts++;return text(409,"Conflict: cannot overwrite an existing cache entry");}return text(200,"Stored");
   case "digest-mismatch":row.rejected++;return text(400,"Digest mismatch");
   case "length-mismatch":row.rejected++;return text(400,"Content-Length mismatch");
   case "too-large":row.rejected++;return text(413,"Entry exceeds the maximum entry size");
   case "capacity":row.rejected++;return text(507,"Insufficient storage",{"retry-after":"5"});
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

/** ⚙️ Authors one running server: where to listen, what to store and who may use it. */
export type RemoteCacheServerOptionsV1=Readonly<{directory:string;host:string;port:number;maximumBytes:number;maximumEntryBytes:number;maximumAgeDays:number;defaultInstance:string;tokens:RemoteCacheTokensV1;signal?:AbortSignal;onEvent?:(event:RemoteCacheStoreEventV1|RemoteCacheRequestEventV1|Readonly<{stage:"listening";url:string}>|Readonly<{stage:"stopped"}>)=>void}>;

/** 🛰️ A started server: its address, a live status and an orderly stop. */
export interface RemoteCacheServerV1{readonly url:string;readonly port:number;readonly store:RemoteCacheStoreV1;readonly stopped:Promise<void>;stop():Promise<void>}

const LOOPBACK=new Set(["127.0.0.1","localhost","::1","[::1]"]);

/** 🚀️ Opens the store and serves the handler with Bun; refuses to listen beyond loopback without a token, and stops when `signal` aborts. */
export async function startRemoteCacheServerV1(options:RemoteCacheServerOptionsV1):Promise<RemoteCacheServerV1>{
 if(!LOOPBACK.has(options.host)&&options.tokens.read===undefined&&options.tokens.write===undefined)throw Error("A token is required to listen beyond loopback");
 if(!INSTANCE.test(options.defaultInstance)||RESERVED.has(options.defaultInstance))throw Error("Valid default instance required");
 const store=await RemoteCacheStoreV1.open({directory:options.directory,maximumBytes:options.maximumBytes,maximumEntryBytes:options.maximumEntryBytes,maximumAgeMilliseconds:options.maximumAgeDays*86400000,...(options.signal?{signal:options.signal}:{}),...(options.onEvent?{onEvent:options.onEvent}:{})});
 const handler=createRemoteCacheHandlerV1({store,tokens:options.tokens,defaultInstance:options.defaultInstance,...(options.onEvent?{onRequest:options.onEvent}:{})});
 const server=Bun.serve({hostname:options.host,port:options.port,idleTimeout:60,maxRequestBodySize:options.maximumEntryBytes*2,fetch:request=>handler(request)});
 const host=options.host.includes(":")&&!options.host.startsWith("[")?`[${options.host}]`:options.host,port=server.port??options.port,url=`http://${host}:${port}`;
 let finish!:()=>void;const stopped=new Promise<void>(resolve=>{finish=resolve;});let closing:Promise<void>|undefined;
 const stop=():Promise<void>=>closing??=(async()=>{await server.stop(true);await store.close();options.onEvent?.({stage:"stopped"});finish();})();
 if(options.signal){if(options.signal.aborted)await stop();else options.signal.addEventListener("abort",()=>{void stop();},{once:true});}
 options.onEvent?.({stage:"listening",url});
 return Object.freeze({url,port,store,stopped,stop});
}
