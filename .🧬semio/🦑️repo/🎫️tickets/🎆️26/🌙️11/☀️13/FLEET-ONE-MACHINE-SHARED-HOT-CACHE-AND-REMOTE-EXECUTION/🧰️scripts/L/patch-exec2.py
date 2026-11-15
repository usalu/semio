LIB="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
RE=LIB+"🌐️remote-execution/"
def edit(path,pairs):
    s=open(path,encoding='utf8').read()
    for old,new in pairs:
        assert s.count(old)==1,(path.split('/')[-2],s.count(old),old[:90])
        s=s.replace(old,new)
    open(path,'w',encoding='utf8').write(s)

# --- server
edit(RE+'🖥️server/🟦️.ts',[
('import {runtimeServeV1} from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🌍️serve/🟦️.ts";','import {runtimeServeV1} from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🌍️serve/🟦️.ts";\nimport {FLEET_CONTENT_HEADER_V1} from "../../🌐️remote-cache/🛰️fleet/🪪️identity/🟦️.ts";\nimport {createFleetGateV1,type FleetAccessV1,type FleetGateV1} from "../../🌐️remote-cache/🛰️fleet/🪪️identity/🛂️gate/🟦️.ts";'),
('/** 🔐️ Bearer tokens: with none the service is open (loopback only), a read token allows lookups and downloads, a write token everything, a worker token the worker API. */','/** 🔐️ Operator tokens, honoured from the machine itself only once a gate with fleet identities is in place: with none the service is open (loopback only), a read token allows lookups and downloads, a write token everything, a worker token the worker API. */'),
(' tokens:RemoteExecutionTokensV1;tls?:',' tokens:RemoteExecutionTokensV1;gate?:FleetGateV1;maximumWorkers?:number;maximumRequestBytes?:number;tls?:'),
(' const open=options.tokens.read===undefined&&options.tokens.write===undefined,openWorkers=options.tokens.worker===undefined;\n if(!LOOPBACK.has(options.host)&&(open||openWorkers))throw Error("Tokens are required to listen beyond loopback");',' const gate=options.gate??createFleetGateV1({tokens:{read:options.tokens.read,write:options.tokens.write,worker:options.tokens.worker}});\n if(!LOOPBACK.has(options.host)&&gate.open)throw Error("Tokens or fleet identities are required to listen beyond loopback");\n if(WILDCARD.has(options.host))throw Error("Listen on the fleet interface, not on a wildcard address");'),
(''' const authorize=(call:{method:string;metadata:Readonly<Record<string,string>>}):void=>{
  if(open)return;
  const presented=bearer(call.metadata.authorization);
  if(!presented)throw new GrpcErrorV1(GRPC_CODES_V1.UNAUTHENTICATED,"Unauthorized");
  const writer=options.tokens.write!==undefined&&compare(options.tokens.write,presented),reader=options.tokens.read!==undefined&&compare(options.tokens.read,presented);
  if(!writer&&!reader)throw new GrpcErrorV1(GRPC_CODES_V1.UNAUTHENTICATED,"Unauthorized");
  if(!writer&&!READ_ONLY.has(call.method))throw new GrpcErrorV1(GRPC_CODES_V1.PERMISSION_DENIED,"Read-only token");
 };''',''' const authorize=async(call:{method:string;metadata:Readonly<Record<string,string>>;peer:string}):Promise<void|Readonly<{principal:string}>>=>{
  if(!gate.network(call.peer))throw new GrpcErrorV1(GRPC_CODES_V1.PERMISSION_DENIED,"Forbidden");
  const verdict=await gate.check({method:"POST",target:"/"+call.method,authorization:call.metadata.authorization,content:call.metadata[FLEET_CONTENT_HEADER_V1],peer:call.peer,roles:["submit","admin"],tokens:READ_ONLY.has(call.method)?["read","write"]:["write"],...(READ_ONLY.has(call.method)?{}:{refused:["read"]})});
  if(!verdict.ok)throw new GrpcErrorV1(verdict.status===403?GRPC_CODES_V1.PERMISSION_DENIED:GRPC_CODES_V1.UNAUTHENTICATED,verdict.status===403?"Forbidden":"Unauthorized");
  return verdict.access===undefined||verdict.access.role==="static"?undefined:{principal:verdict.access.origin};
 };'''),
(''' const allowed=(request:Request,kind:"worker"|"status"):Response|undefined=>{
  const presented=bearer(request.headers.get("authorization"));
  if(kind==="worker"){
   if(openWorkers)return undefined;
   return presented&&compare(options.tokens.worker!,presented)?undefined:text(401,"Unauthorized");
  }
  if(open&&openWorkers)return undefined;
  const matches=[options.tokens.read,options.tokens.write,options.tokens.worker].some(token=>token!==undefined&&presented!==undefined&&compare(token,presented));
  return matches?undefined:text(401,"Unauthorized");
 };''',''' const owners=new Map<string,string>(),maximumWorkers=options.maximumWorkers??256;
 const allowed=async(request:Request,peer:string|undefined,kind:"worker"|"status"):Promise<Readonly<{denied:Response}>|Readonly<{access:FleetAccessV1|undefined}>>=>{
  const url=new URL(request.url),verdict=await gate.check({method:request.method,target:url.pathname+url.search,authorization:request.headers.get("authorization"),content:request.headers.get(FLEET_CONTENT_HEADER_V1),peer,roles:kind==="worker"?["worker"]:["worker","submit","admin"],tokens:kind==="worker"?["worker"]:["read","write","worker"]});
  return verdict.ok?{access:verdict.access}:{denied:gate.respond(verdict)};
 };
 const mine=(access:FleetAccessV1|undefined,id:string):boolean=>access===undefined||access.role==="static"||owners.get(id)===access.origin;
 const jsonBody=async(request:Request,access:FleetAccessV1|undefined):Promise<unknown>=>{
  const bytes=new Uint8Array(await request.arrayBuffer());
  if(access!==undefined&&access.role!=="static"&&access.content!=="-"&&createHash("sha256").update(bytes).digest("hex")!==access.content)return undefined;
  try{return JSON.parse(new TextDecoder().decode(bytes));}catch{return undefined;}
 };'''),
('idleTimeoutSeconds:120,maximumRequestBodyBytes:64*1048576,','idleTimeoutSeconds:120,maximumRequestBodyBytes:options.maximumRequestBytes??16*1048576,'),
('  fetch:async request=>{\n   const url=new URL(request.url),path=url.pathname,method=request.method;\n   try{\n    if(path==="/healthz")return Response.json({status:"ok"});','  fetch:async(request,peer)=>{\n   const url=new URL(request.url),path=url.pathname,method=request.method;\n   if(!gate.network(peer))return text(403,"Forbidden");\n   try{\n    if(path==="/healthz")return Response.json({status:"ok"});'),
('    if(path==="/status"&&method==="GET"){const denied=allowed(request,"status");return denied??Response.json(await status());}','    if(path==="/status"&&method==="GET"){const guard=await allowed(request,peer,"status");return "denied" in guard?guard.denied:Response.json(await status());}'),
('    if(path==="/v1/fleet/load"&&method==="GET"){const denied=allowed(request,"status");if(denied)return denied;const document','    if(path==="/v1/fleet/load"&&method==="GET"){const guard=await allowed(request,peer,"status");if("denied" in guard)return guard.denied;const document'),
('''     const denied=allowed(request,"worker");if(denied)return denied;
     const body=await request.json().catch(()=>undefined);
     try{requireRemoteExecutionShapeV1("LoadReport",body,"Closed load report required");}catch(error){return text(400,(error as Error).message);}
     scheduler.acceptLoad(body as never);return Response.json({accepted:true});''','''     const guard=await allowed(request,peer,"worker");if("denied" in guard)return guard.denied;
     const body=await jsonBody(request,guard.access);
     try{requireRemoteExecutionShapeV1("LoadReport",body,"Closed load report required");}catch(error){return text(400,(error as Error).message);}
     if(guard.access!==undefined&&guard.access.role!=="static"&&(body as {origin:string}).origin!==guard.access.origin)return text(403,"Forbidden: the report names another origin");
     scheduler.acceptLoad(body as never);return Response.json({accepted:true});'''),
('''    const denied=allowed(request,"worker");if(denied)return denied;
    const [,id,verb,task]=route;
    if(!id&&method==="POST"){
     const body=await request.json().catch(()=>undefined);
     try{requireRemoteExecutionShapeV1("WorkerRegistration",body,"Closed worker registration required");}catch(error){return text(400,(error as Error).message);}
     return Response.json(scheduler.register(body));
    }
    if(id&&!verb&&!task&&method==="DELETE"){scheduler.deregister(id);return Response.json({ok:true});}
    if(id&&verb==="poll"&&method==="POST"){''','''    const guard=await allowed(request,peer,"worker");if("denied" in guard)return guard.denied;
    const [,id,verb,task]=route,access=guard.access;
    if(id&&!mine(access,id))return text(403,"Forbidden: this worker belongs to another origin");
    if(!id&&method==="POST"){
     const body=await jsonBody(request,access);
     try{requireRemoteExecutionShapeV1("WorkerRegistration",body,"Closed worker registration required");}catch(error){return text(400,(error as Error).message);}
     if(access!==undefined&&access.role!=="static"&&(body as {origin:string}).origin!==access.origin)return text(403,"Forbidden: the registration names another origin");
     const live=new Set(scheduler.status().workers.map(row=>String(row.id)));
     for(const known of[...owners.keys()])if(!live.has(known))owners.delete(known);
     if(owners.size>=maximumWorkers)return text(503,"Too many workers");
     const registered=scheduler.register(body);
     if(access!==undefined&&access.role!=="static")owners.set(registered.id,access.origin);
     return Response.json(registered);
    }
    if(id&&!verb&&!task&&method==="DELETE"){scheduler.deregister(id);owners.delete(id);return Response.json({ok:true});}
    if(id&&verb==="poll"&&method==="POST"){'''),
('''    if(id&&verb==="heartbeat"&&method==="POST"){
     const body=await request.json().catch(()=>undefined);''','''    if(id&&verb==="heartbeat"&&method==="POST"){
     const body=await jsonBody(request,access);'''),
('''    if(id&&task&&method==="POST"){
     const body=await request.json().catch(()=>undefined);''','''    if(id&&task&&method==="POST"){
     const body=await jsonBody(request,access);'''),
('const WILDCARD=new Set(["0.0.0.0","::","[::]"]),NS=','const WILDCARD=new Set(["0.0.0.0","::","[::]"]),NS='),
('   for await(const snapshot of scheduler.execute({instance,action,skipCacheLookup:request.skip_cache_lookup===true},call.signal))yield','   for await(const snapshot of scheduler.execute({instance,action,skipCacheLookup:request.skip_cache_lookup===true,...(call.principal===undefined?{}:{submittedBy:call.principal})},call.signal))yield'),
('  [`${EXEC}/Execute`]:async function*(request:ProtobufMessageV1,call:{signal:AbortSignal}){','  [`${EXEC}/Execute`]:async function*(request:ProtobufMessageV1,call:{signal:AbortSignal;principal?:string}){'),
])
print("exec part 2 ok")
