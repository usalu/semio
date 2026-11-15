import sys,glob
base="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🖥️server/🟦️.ts"
s=open(base,encoding='utf8').read()
def rep(old,new,count=1):
    global s
    assert s.count(old)==count,(s.count(old),old[:80])
    s=s.replace(old,new)

rep('import {createHash,timingSafeEqual} from "node:crypto";','import {createHash,timingSafeEqual} from "node:crypto";\nimport {createFleetGateV1,type FleetAccessV1,type FleetGateV1} from "../🛰️fleet/🪪️identity/🛂️gate/🟦️.ts";\nimport {createFleetRegistryV1,createFleetVerifierV1,fleetContentHashV1,FLEET_CONTENT_HEADER_V1} from "../🛰️fleet/🪪️identity/🟦️.ts";\nimport {openProvenanceV1,provenanceFingerprintV1,type DivergenceV1,type ProvenanceV1} from "../🧾️provenance/🟦️.ts";')
rep('import {answerClaimRequestV1,createClaimTableV1,type ClaimSummaryV1,type ClaimTableV1} from "../🎟️claims/🟦️.ts";','import {answerClaimRequestV1,createClaimTableV1,parseClaimPathV1,type ClaimSummaryV1,type ClaimTableV1} from "../🎟️claims/🟦️.ts";')
rep('''/** 🔐️ Authorizes bearer tokens: with neither token the server is open, a read token allows GET and HEAD, a write token allows everything. */
export type RemoteCacheTokensV1=Readonly<{read?:string|undefined;write?:string|undefined;fleet?:import("../🛰️fleet/🔑️credential/🟦️.ts").FleetCredentialSourceV1|undefined}>;''','''/** 🔐️ Operator tokens: honoured from the machine itself only once the server runs with fleet identities, so they never cross the LAN. With neither token and no fleet the server is open (loopback only); a read token allows GET and HEAD, a write token allows everything. */
export type RemoteCacheTokensV1=Readonly<{read?:string|undefined;write?:string|undefined}>;

/** 🛰️ What the fleet identities of a server need: the group store whose `origins/` it trusts, the group, the CIDR networks allowed to talk to it, where it remembers pinned origin keys and which origin may sign as admin. */
export type RemoteCacheFleetOptionsV1=Readonly<{storeRoot:string;group:string;networks:readonly string[];pinsPath?:string;adminOrigin?:string}>;

/** 🧾️ The `fleet` part of `GET /status`: how many origins are usable, retired or in conflict and why requests were refused (counts by reason, never the identity of an attacker's key). */
export type RemoteCacheFleetStatusV1=Readonly<{origins:Readonly<Record<string,number>>;denied:Readonly<Record<string,number>>}>;''')
rep('fleet?:Readonly<{generation:number;credentialId:string;rotatedAtMs:number}>|null;claims?:ClaimSummaryV1}>;','fleet?:RemoteCacheFleetStatusV1|null;divergences?:Readonly<{count:number;recent:readonly DivergenceV1[]}>;claims?:ClaimSummaryV1}>;')
rep('mirror?:(request:Request)=>Promise<Response|undefined>;onRequest','mirror?:(request:Request)=>Promise<Response|undefined>;gate?:FleetGateV1;provenance?:ProvenanceV1;fleetStatus?:()=>RemoteCacheFleetStatusV1;onRequest')
rep('''  startedAt=(options.startedAt??new Date()).toISOString(),open=options.tokens.read===undefined&&options.tokens.write===undefined&&options.tokens.fleet===undefined;
 const authorize=(request:Request,write:boolean):Response|undefined=>{
  if(open)return undefined;
  const header=request.headers.get("authorization"),presented=header?.startsWith("Bearer ")?header.slice(7):undefined;
  if(presented===undefined||presented==="")return text(401,"Unauthorized",{"www-authenticate":"Bearer"});
  const writer=options.tokens.write!==undefined&&remoteCacheTokenMatchesV1(options.tokens.write,presented)||options.tokens.fleet?.tokens().some(token=>remoteCacheTokenMatchesV1(token,presented))===true,reader=options.tokens.read!==undefined&&remoteCacheTokenMatchesV1(options.tokens.read,presented);
  if(!writer&&!reader)return text(401,"Unauthorized",{"www-authenticate":"Bearer"});
  return write&&!writer?text(403,"Forbidden: read-only token"):undefined;
 };''','''  startedAt=(options.startedAt??new Date()).toISOString(),gate=options.gate??createFleetGateV1({tokens:{read:options.tokens.read,write:options.tokens.write}});
 const bound=(access:FleetAccessV1|undefined,bytes:Uint8Array):boolean=>access===undefined||access.role==="static"||(access.content!=="-"&&fleetContentHashV1(bytes)===access.content);
 const authenticate=async(request:Request,url:URL,peer:string|undefined,write:boolean):Promise<Readonly<{denied:Response}>|Readonly<{access:FleetAccessV1|undefined}>>=>{
  const verdict=await gate.check({method:request.method,target:url.pathname+url.search,authorization:request.headers.get("authorization"),content:request.headers.get(FLEET_CONTENT_HEADER_V1),peer,roles:write?["cache-write"]:["cache-read","cache-write","admin"],tokens:write?["write"]:["read","write"],...(write?{refused:["read"]}:{})});
  return verdict.ok?{access:verdict.access}:{denied:gate.respond(verdict)};
 };''')
rep(' const answer=async(request:Request,body:RemoteCacheBodyV1):Promise<Response>=>{',' const answer=async(request:Request,body:RemoteCacheBodyV1,peer:string|undefined):Promise<Response>=>{')
rep('''   const denied=authorize(request,false);if(denied)return denied;
   const usage=options.store.usage(),status:RemoteCacheServerStatusV1={version:1,protocols:["bazel-http","nx-self-hosted"],entries:usage.entries,bytes:usage.bytes,reservedBytes:usage.reservedBytes,maximumBytes:usage.maximumBytes,maximumEntryBytes:usage.maximumEntryBytes,readOnly,validation,counters,startedAt,fleet:options.tokens.fleet?.describe()??null,claims:claims.summary()};''','''   const guard=await authenticate(request,url,peer,false);if("denied" in guard)return guard.denied;
   const usage=options.store.usage(),status:RemoteCacheServerStatusV1={version:1,protocols:["bazel-http","nx-self-hosted"],entries:usage.entries,bytes:usage.bytes,reservedBytes:usage.reservedBytes,maximumBytes:usage.maximumBytes,maximumEntryBytes:usage.maximumEntryBytes,readOnly,validation,counters,startedAt,fleet:options.fleetStatus?.()??null,divergences:options.provenance?.summary()??{count:0,recent:[]},claims:claims.summary()};''')
rep('''  const claimed=await answerClaimRequestV1({request,url,table:claims,defaultInstance:options.defaultInstance,authorize:write=>authorize(request,write),readOnly,has:(namespace,kind,key)=>options.store.has(namespace,kind,key),readBody:async limit=>{const parts:Uint8Array[]=[];let size=0;try{for await(const chunk of body.chunks){size+=chunk.byteLength;if(size>limit)return undefined;parts.push(chunk);}}catch{return undefined;}const bytes=new Uint8Array(size);let offset=0;for(const part of parts){bytes.set(part,offset);offset+=part.byteLength;}return bytes;}});''','''  const claimAddress=parseClaimPathV1(url.pathname,options.defaultInstance);
  let claimAccess:FleetAccessV1|undefined;
  if(claimAddress!==undefined){const guard=await authenticate(request,url,peer,claimAddress.action!=="await");if("denied" in guard)return guard.denied;claimAccess=guard.access;}
  const claimed=await answerClaimRequestV1({request,url,table:claims,defaultInstance:options.defaultInstance,authorize:()=>undefined,...(claimAccess===undefined||claimAccess.role==="static"?{}:{scope:claimAccess.origin}),readOnly,has:(namespace,kind,key)=>options.store.has(namespace,kind,key),readBody:async limit=>{const parts:Uint8Array[]=[];let size=0;try{for await(const chunk of body.chunks){size+=chunk.byteLength;if(size>limit)return undefined;parts.push(chunk);}}catch{return undefined;}const bytes=new Uint8Array(size);let offset=0;for(const part of parts){bytes.set(part,offset);offset+=part.byteLength;}return bound(claimAccess,bytes)?bytes:undefined;}});''')
rep('''   const denied=authorize(request,true);if(denied)return denied;
   if(readOnly)return text(403,"Forbidden: this cache is read-only");
   const parts:Uint8Array[]=[];let size=0;
   try{for await(const chunk of body.chunks){size+=chunk.byteLength;if(size>1048576)return text(413,"Claim exceeds the maximum size");parts.push(chunk);}}catch{return text(499,"Client closed request");}
   let claim''','''   const guard=await authenticate(request,url,peer,true);if("denied" in guard)return guard.denied;
   if(readOnly)return text(403,"Forbidden: this cache is read-only");
   const parts:Uint8Array[]=[];let size=0;
   try{for await(const chunk of body.chunks){size+=chunk.byteLength;if(size>1048576)return text(413,"Claim exceeds the maximum size");parts.push(chunk);}}catch{return text(499,"Client closed request");}
   if(!bound(guard.access,Buffer.concat(parts)))return text(400,"Content hash required and must match");
   let claim''')
rep('''  if(method==="GET"||method==="HEAD"){
   const denied=authorize(request,false);if(denied)return denied;
   if(address.kind==="ac")''','''  if(method==="GET"||method==="HEAD"){
   const guard=await authenticate(request,url,peer,false);if("denied" in guard)return guard.denied;
   if(address.kind==="ac")''')
rep('''  const denied=authorize(request,true);if(denied)return denied;
  if(readOnly)return text(403,"Forbidden: this cache is read-only");
  const counter=counters[address.kind]''','''  const guard=await authenticate(request,url,peer,true);if("denied" in guard)return guard.denied;
  const access=guard.access,signed=access!==undefined&&access.role!=="static";
  if(readOnly)return text(403,"Forbidden: this cache is read-only");
  const counter=counters[address.kind]''')
rep('''   const bytes=new Uint8Array(size);let offset=0;for(const part of parts){bytes.set(part,offset);offset+=part.byteLength;}
   const verdict=await validateRemoteCacheActionResultV1(bytes,validation,blobs(address.namespace));
   if(!verdict.valid){counter.rejected++;return text(400,`Invalid action result: ${verdict.reason}`);}
   source=(async function*(){yield bytes;})();known=size;
  }
  const outcome=await options.store.put(''','''   const bytes=new Uint8Array(size);let offset=0;for(const part of parts){bytes.set(part,offset);offset+=part.byteLength;}
   if(!bound(access,bytes)){counter.rejected++;return text(400,"Content hash required and must match");}
   const verdict=await validateRemoteCacheActionResultV1(bytes,validation,blobs(address.namespace));
   if(!verdict.valid){counter.rejected++;return text(400,`Invalid action result: ${verdict.reason}`);}
   if(signed&&options.provenance!==undefined){
    const parsed=parseRemoteCacheActionResultV1(bytes);fingerprint=provenanceFingerprintV1([...parsed.references,...parsed.trees].map(row=>row.hash));
    const held=await options.store.readBytes(address.namespace,"ac",address.key,limit),before=options.provenance.of(address.namespace,"ac",address.key);
    if(held!==undefined&&before?.origin!==access!.origin){
     let kept=before?.fingerprint;
     if(kept===undefined)try{const old=parseRemoteCacheActionResultV1(held);kept=provenanceFingerprintV1([...old.references,...old.trees].map(row=>row.hash));}catch{kept=undefined;}
     if(kept!==undefined&&kept!==fingerprint){
      options.provenance.diverged({namespace:address.namespace,kind:"ac",key:address.key,keptOrigin:before?.origin??"unknown",keptSha256:kept,refusedOrigin:access!.origin,refusedSha256:fingerprint});
      counter.conflicts++;return text(409,"Conflict: another origin stored a different result for this key");
     }
    }
    prior=before;
   }
   source=(async function*(){yield bytes;})();known=size;
  }
  let nxHash:ReturnType<typeof createHash>|undefined;
  if(address.kind==="nx"&&signed&&access!.content!=="-"){const hash=nxHash=createHash("sha256"),chunks=body.chunks;source=(async function*(){for await(const chunk of chunks){hash.update(chunk);yield chunk;}})();}
  const outcome=await options.store.put(''')
rep('''  let source:AsyncIterable<Uint8Array>=body.chunks,known=declared;''','''  let source:AsyncIterable<Uint8Array>=body.chunks,known=declared,fingerprint:string|undefined,prior:ReturnType<ProvenanceV1["of"]>;''')
rep('''  switch(outcome){
   case "created":case "replaced":counter.puts++;if(address.kind!=="cas")claims.stored(address.namespace,address.kind,address.key);return text(200,"Stored");''','''  if(nxHash!==undefined&&(outcome==="created"||outcome==="replaced")&&nxHash.digest("hex")!==access!.content){await options.store.remove(address.namespace,"nx",address.key);counter.rejected++;return text(400,"Content hash mismatch");}
  if(signed&&options.provenance!==undefined&&(outcome==="created"||outcome==="replaced")&&(prior===undefined||prior.origin===access!.origin||outcome==="created"))options.provenance.record(address.namespace,address.kind,address.key,access!.origin,fingerprint);
  switch(outcome){
   case "created":case "replaced":counter.puts++;if(address.kind!=="cas")claims.stored(address.namespace,address.kind,address.key);return text(200,"Stored");''')
rep(''' return async request=>{
  const started=performance.now(),body=remoteCacheBodyV1(request.body),response=await answer(request,body);''',''' return async(request,peer)=>{
  if(!gate.network(peer))return text(403,"Forbidden");
  const started=performance.now(),body=remoteCacheBodyV1(request.body),response=await answer(request,body,peer);''')
rep('''export function createRemoteCacheHandlerV1(options:RemoteCacheHandlerOptionsV1):(request:Request)=>Promise<Response>{''','''export function createRemoteCacheHandlerV1(options:RemoteCacheHandlerOptionsV1):(request:Request,peer?:string)=>Promise<Response>{''')
rep('export type RemoteCacheServerOptionsV1=Readonly<{mirror?','export type RemoteCacheServerOptionsV1=Readonly<{fleet?:RemoteCacheFleetOptionsV1;allowWildcard?:boolean;mirror?')
rep(''' if(!LOOPBACK.has(options.host)&&options.tokens.read===undefined&&options.tokens.write===undefined&&options.tokens.fleet===undefined)throw Error("A token is required to listen beyond loopback");''',''' const registry=options.fleet===undefined?undefined:createFleetRegistryV1({storeRoot:options.fleet.storeRoot,group:options.fleet.group,pinsPath:options.fleet.pinsPath??join(options.directory,"fleet","pins.json")});
 const verifier=registry===undefined||options.fleet===undefined?undefined:createFleetVerifierV1({registry,audience:"cache",adminOrigin:options.fleet.adminOrigin??`${options.fleet.group}-server`});
 const gate=createFleetGateV1({verifier,networks:options.fleet?.networks,tokens:{read:options.tokens.read,write:options.tokens.write}});
 if(!LOOPBACK.has(options.host)&&gate.open)throw Error("A token or fleet identities are required to listen beyond loopback");
 if(WILDCARD.has(options.host)&&options.allowWildcard!==true)throw Error("Listen on the fleet interface, not on a wildcard address");''')
rep('''const LOOPBACK=new Set(["127.0.0.1","localhost","::1","[::1]"]);''','''const LOOPBACK=new Set(["127.0.0.1","localhost","::1","[::1]"]),WILDCARD=new Set(["0.0.0.0","::","[::]"]);''')
rep(''' const table=createClaimTableV1();
 try{''',''' const table=createClaimTableV1();
 const provenance=await openProvenanceV1(options.directory);
 await registry?.refresh(true);
 const fleetStatus=():RemoteCacheFleetStatusV1=>{const origins:Record<string,number>={};for(const row of Object.values(registry?.origins()??{}))origins[row.status]=(origins[row.status]??0)+1;return {origins,denied:verifier?.denied()??{}};};
 try{''')
rep('''const opened=store,handler=createRemoteCacheHandlerV1({store:opened,claims:table,''','''const opened=store,handler=createRemoteCacheHandlerV1({store:opened,claims:table,gate,provenance,...(registry===undefined?{}:{fleetStatus}),''')
rep('fetch:request=>handler(request)});','fetch:(request,peer)=>handler(request,peer)});')
rep('''const stop=():Promise<void>=>closing??=(async()=>{if(timer)clearInterval(timer);table.close();await server.stop(true);await opened.close();await owner.release();''','''const stop=():Promise<void>=>closing??=(async()=>{if(timer)clearInterval(timer);table.close();await server.stop(true);await opened.close();await provenance.flush();await owner.release();''')
open(base,'w',encoding='utf8').write(s)
print("patched")
