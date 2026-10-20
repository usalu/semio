#!/usr/bin/env python3
"""Builds staged/ops.json for the conc-nx set from the live files: every op is an exact-anchor replacement that must match once.
usage: ops_gen.py [--root REPO_ROOT]"""
import json, os, sys, hashlib

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, *([".."] * 8)))
if "--root" in sys.argv:
    ROOT = os.path.abspath(sys.argv[sys.argv.index("--root") + 1])


def E(code):
    return chr(code) + "️"


LIB = os.path.join(E(0x1F9F0) + "framework", E(0x1F6CD) + "products", E(0x1F991) + "repo", E(0x1F528) + "modules", E(0x1F4DA) + "library")
CACHING = os.path.join(LIB, E(0x26A1) + "caching")
GRAPH = os.path.join(CACHING, E(0x1F578) + "graph")
SEAL = os.path.join(GRAPH, E(0x1F50F) + "seal")
GEN = os.path.join(SEAL, E(0x1F4CC) + "generation")
STARTUP = os.path.join(GRAPH, E(0x1F3C1) + "startup")
ORDER = os.path.join(CACHING, E(0x1F4BE) + "storage", E(0x1F512) + "enforcement", E(0x1F50C) + "nx")
NXGRAPH = os.path.join(ORDER, E(0x1F4E6) + "graph")
TS = E(0x1F7E6) + ".ts"
JSONF = E(0x1F523) + ".json"
TESTS = E(0x1F9EA) + "tests"
FIXT = E(0x1F9EB) + "fixtures"
SCHEMA = E(0x1F9EC) + "schema"
PY = E(0x1F40D) + "oracle.py"

FILES = {
    "graph": os.path.join(NXGRAPH, TS),
    "nx": os.path.join(ORDER, TS),
    "script": os.path.join(CACHING, E(0x1F680) + "bootstrap", E(0x1F4DC) + "script.ts"),
    "service": os.path.join(GRAPH, E(0x1F3C1) + "startup", E(0x1F6A6) + "service", TS),
}


def read(path):
    with open(os.path.join(ROOT, path), encoding="utf8") as handle:
        return handle.read()


ops = []
GROUP = "1"


def op(key, anchor, replacement, fresh=True):
    path = FILES[key]
    text = read(path)
    if fresh and text.count(anchor) != 1:
        sys.exit("anchor occurs %d times in %s: %r" % (text.count(anchor), path, anchor[:80]))
    ops.append({"path": path, "anchor": anchor, "replacement": replacement, "group": GROUP})


def line_of(key, start):
    text = read(FILES[key])
    first = text.index(start)
    return text[first:text.index("\n", first)]


# ---- G: graph pool ---------------------------------------------------------------------------------------------------------------
op("graph", 'import {existsSync} from "node:fs";', 'import {existsSync,renameSync as _unused} from "node:fs";'.replace(",renameSync as _unused", ""))
ops.pop()

room = line_of("graph", "async function room(")
NEW_ROOM = r'''/** 🧮️ Thrown inside one pool pass that finds no capacity for its demand; `poolWithRoom` then leaves the pool, evicts idle namespaces without holding it and runs the pass again. */
class NxGraphRoomNeededV1 extends Error{readonly current:string;readonly extra:number;readonly additional:number;constructor(current:string,extra:number,additional:number){super("Funded Nx graph cache capacity required");this.current=current;this.extra=extra;this.additional=additional;}}
/** 🧮️ Refuses inside a pool pass, before any mutation, when the roster cannot take the demand. */
function requireRoom(store:NxGraphCacheStoreV1,rows:readonly NamespaceRow[],current:string,extra:number,additional:number):void{if(!capacity(store,rows,extra+additional*(nativePolicy.maximumPhysicalBytes+policy.namespaceMetadataBytes)))throw new NxGraphRoomNeededV1(current,extra,additional);}
/** 🧮️ Thrown inside a pool pass whose namespace lease is held exclusively elsewhere; the pass is left, the lease awaited without holding the pool, and the pass run again. */
class NxGraphNamespaceBusyV1 extends Error{readonly resource:string;constructor(resource:string){super("Nx graph namespace busy");this.resource=resource;}}
/** 🔒️ Takes the shared lease of one namespace inside a pool pass without ever waiting for it. */
async function tryNamespaceLeaseV1(store:NxGraphCacheStoreV1,directory:string):Promise<ResourceLease>{
 const resource=nxGraphNamespaceResourceV1(directory),control=nxGraphOperationControlV1(store.storage,store.control);
 try{return await acquireResourceLease({directory:poolPolicies.get(store)!.leaseDirectory,resource,mode:"shared",signal:control.signal,control,timeoutMs:0});}catch(error){check(store);if(error instanceof Error&&error.message==="Resource lease timed out: "+resource)throw new NxGraphNamespaceBusyV1(resource);throw error;}
}
/** 🚪️ Runs one short roster pass under the exclusive pool. A pass that needs room is retried (at most three times) after the idle namespaces were evicted outside the pool; a pass that met a busy namespace is retried (at most sixteen times) after that lease was awaited while holding nothing. */
async function poolWithRoom<T>(store:NxGraphCacheStoreV1,body:()=>Promise<T>):Promise<T>{
 for(let evictions=0,busy=0;;){
  try{return await pool(store,body);}catch(error){
   if(error instanceof NxGraphNamespaceBusyV1){
    if(++busy>16)throw Error("Nx graph namespace stayed busy");
    const control=nxGraphOperationControlV1(store.storage,store.control),waited=await acquireResourceLease({directory:poolPolicies.get(store)!.leaseDirectory,resource:error.resource,mode:"shared",signal:control.signal,control});waited.release();continue;
   }
   if(!(error instanceof NxGraphRoomNeededV1))throw error;
   if(evictions++>=3)throw Error("Funded Nx graph cache capacity exhausted");
   await evictIdleNamespacesV1(store,error);
  }
 }
}
/** ♻️ Evicts idle namespaces until the demand fits. It holds only the queued `nx:graph-evict` gate and, one at a time, a victim's exclusive namespace lease taken without waiting (a busy namespace is skipped); the pool is held only for the two short roster reads, never across retirement or deletion, so no pool holder ever waits on another namespace's readers. */
async function evictIdleNamespacesV1(store:NxGraphCacheStoreV1,need:NxGraphRoomNeededV1):Promise<void>{
 const control=nxGraphOperationControlV1(store.storage,store.control),leaseDirectory=poolPolicies.get(store)!.leaseDirectory,demand=need.extra+need.additional*(nativePolicy.maximumPhysicalBytes+policy.namespaceMetadataBytes);
 const gate=await acquireQueuedResourceLease({directory:leaseDirectory,resource:"nx:graph-evict:"+createHash("sha256").update(store.root).digest("hex"),mode:"exclusive",owner:randomBytes(8).toString("hex"),signal:control.signal,control});
 try{
  const roster=()=>pool(store,()=>inventory(store));let rows=await roster();
  if(capacity(store,rows,demand))return;
  for(const row of rows){
   if(row.directory===need.current)continue;
   check(store);const resource=nxGraphNamespaceResourceV1(row.directory);let lease:ResourceLease;
   try{lease=await acquireResourceLease({directory:leaseDirectory,resource,mode:"exclusive",signal:control.signal,control,timeoutMs:0});}catch(error){check(store);if(error instanceof Error&&error.message==="Resource lease timed out: "+resource)continue;throw error;}
   try{
    await store.recheck();let current:string;try{current=stamp(join(row.directory,".namespace.json"));}catch{continue;}
    if(current!==row.stamp)continue;
    await bytes(row.directory,store);await retireNxGraphNativeChildV1(store,nxGraphNativeImageDirectoryV1(store.root,row.identity));rmSync(requireNativeStoragePathV1(store.root,row.directory),{recursive:true,force:true});
   }finally{lease.release();}
   await control.publish({resource,mode:"exclusive",elapsedMs:0});await control.yieldContinuation();
   rows=await roster();if(capacity(store,rows,demand))return;
  }
  throw Error("Funded Nx graph cache capacity exhausted");
 }finally{gate.release();}
}
/** 🔌️ Attaches an existing namespace without the pool: the shared namespace lease the caller holds keeps eviction away, the registered descriptor and a committed funded native shard must already exist (births only happen in a pool pass), and a namespace that is missing or only half born releases the lease through `releaseLease` and answers `undefined` so the caller takes the pool pass that births it while holding nothing. */
async function attachExistingNamespaceV1(store:NxGraphCacheStoreV1,value:NxGraphNamespaceV1,directory:string,releaseLease:()=>void):Promise<Awaited<ReturnType<typeof retainNxGraphChildV1>>|undefined>{
 const miss=()=>{releaseLease();return undefined;};
 const file=join(directory,".namespace.json"),options={directory:nxGraphNativeImageDirectoryV1(store.root,value),leaseDirectory:poolPolicies.get(store)!.leaseDirectory,filesystemBytes:nativePolicy.filesystemBytes,maximumPhysicalBytes:nativePolicy.maximumPhysicalBytes,control:nxGraphOperationLifetimeControlV1(store.storage,store.control)};
 try{const row=lstatSync(file);if(!row.isFile()||row.isSymbolicLink()||row.nlink!==1||row.size>2048||JSON.stringify(JSON.parse(readFileSync(file,"utf8")))!==JSON.stringify(value))return miss();}catch{return miss();}
 if(!existsSync(dirname(options.directory)))return miss();
 const funded=(await observeNativeStorageStateV1({policy:poolPolicies.get(store)!,control:options.control,port:store.image.port})).allocations.filter(row=>row.child?.directory===options.directory);
 if(funded.length!==1||funded[0]!.owner!==nativePolicy.fundingOwner||funded[0]!.phase!=="committed")return miss();
 const child=await retainNxGraphChildV1(store,options);
 try{const backing=lstatSync(child.image.image);if(backing.blocks*512<backing.size)throw Error("Fully allocated native graph backing required");check(store);}catch(error){await child.close().catch(()=>undefined);throw error;}
 return child;
}'''
op("graph", room, NEW_ROOM)

inv_head = 'const directory=requireNativeStoragePathV1(store.root,join(sdkRoot,key)),entry=lstatSync(directory);'
op("graph", inv_head, 'try{' + inv_head)
op("graph", 'stamp:before});}}return result;}', 'stamp:before});}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}}}return result;}')

old_acquire = line_of("graph", "try{await pool(store,async()=>{const rows=await inventory(store),existing=")
lease_call = old_acquire[old_acquire.index("lease=await acquireResourceLease("):]
new_acquire = ("try{" + lease_call +
    "const attached=await attachExistingNamespaceV1(store,value,directory,()=>{lease!.release();lease=undefined;});if(attached){child=attached;native=attached.image;}else await poolWithRoom(store,async()=>{const rows=await inventory(store),existing=rows.find(row=>row.directory===directory);if(existing&&JSON.stringify(existing.identity)!==JSON.stringify(value))throw Error(\"Nx graph namespace identity collision\");requireRoom(store,rows,directory,0,existing?0:1);lease=await tryNamespaceLeaseV1(store,directory);")
op("graph", old_acquire, new_acquire)

op("graph", "return pool(store,async()=>{const rows=await inventory(store);const row=rows.find(row=>row.directory===base);",
   "return poolWithRoom(store,async()=>{const rows=await inventory(store);const row=rows.find(row=>row.directory===base);")
op("graph", "await room(store,base,nextBytes+priorBytes,0);", "requireRoom(store,rows,base,nextBytes+priorBytes,0);")
op("graph", "return pool(store,async()=>{requireNxGraphNativeDirectoryV1(workspace,environment);const rows=await inventory(store),row=rows.find(",
   "return poolWithRoom(store,async()=>{requireNxGraphNativeDirectoryV1(workspace,environment);const rows=await inventory(store),row=rows.find(")
op("graph", "await room(store,base,request.extraBytes,0);", "requireRoom(store,rows,base,request.extraBytes,0);")

# ---- N: fixed cache custody ------------------------------------------------------------------------------------------------------
op("nx", '/** 🔒️ Revalidates the actual fixed image and original cache reader or writer through the native SDK operation. */\n',
   '/** 🔒️ Revalidates the actual fixed image and original cache reader or writer through the native SDK operation. Every user holds the image resource shared: Nx\'s own cache is content addressed and concurrent writers are safe (proved against stock Nx), and per-output leases keep restores and promotions of one output exclusive, so a put no longer blocks restores. */\n')
op("nx", 'resources:[{resource:"nx:fixed-cache:"+image.root,mode}]', 'resources:[{resource:"nx:fixed-cache:"+image.root,mode:"shared"}]')

# ---- B: bootstrap generation reuse -----------------------------------------------------------------------------------------------
op("script", "    let sourceHandoff:Readonly<{environment:string;close():Promise<void>}>|undefined;\n",
   "    let sourceHandoff:Readonly<{environment:string;close():Promise<void>}>|undefined;\n    let pendingGeneration:import(\"../🕸️graph/🔏️seal/📌️generation/🟦️.ts\").NxGenerationClaimV1|undefined;\n")
op("script", 'const graphSeal = await import("../🕸️graph/🔏️seal/🟦️.ts"), workspaceData =',
   'const generationOwner = await import("../🕸️graph/🔏️seal/📌️generation/🟦️.ts"), graphSeal = await import("../🕸️graph/🔏️seal/🟦️.ts"), workspaceData =')
sealed = '        const sealed=selected||!env.SEMIO_BUILD_GRAPH_DIGEST&&!inheritedReuse&&!invocation.watch&&env.NX_DAEMON!=="true"&&await graphSeal.nxGraphSealHolds(this.root,workspaceData,actualPhase.signal,sourceMembership?.digest).catch(()=>false);\n'
op("script", sealed,
   '        const generationClaim=!selected&&env.SEMIO_BUILD_GRAPH_DIGEST&&!inheritedReuse&&!invocation.watch?await generationOwner.nxGenerationClaimV1(this.root,env).catch(()=>undefined):undefined,generation=generationClaim!==undefined&&await generationOwner.nxGenerationHoldsV1(this.root,workspaceData,generationClaim,actualPhase.signal).catch(()=>false);\n        pendingGeneration=generationClaim!==undefined&&!generation?generationClaim:undefined;\n'
   + sealed.replace("const sealed=selected||", "const sealed=selected||generation||"))
op("script", "        await Promise.all(spooling);\n",
   '        if(pendingGeneration!==undefined&&code===0&&!cancelled&&!invocation.watch){\n'
   '          const{withNxGraphAuxiliaryCapacityV1}=await import("../💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts"),generationOwner=await import("../🕸️graph/🔏️seal/📌️generation/🟦️.ts"),claim=pendingGeneration;\n'
   '          const written=await withNxGraphAuxiliaryCapacityV1(this.root,env,{version:1,extraBytes:8192},()=>generationOwner.writeNxGenerationReceiptV1(this.root,env.NX_WORKSPACE_DATA_DIRECTORY!,claim,actualPhase.signal)).catch(error=>{if(profileBootstrap)console.log(`[nx] Generation receipt refused: ${error instanceof Error?error.message:String(error)}`);return false;});\n'
   '          if(profileBootstrap)console.log(`[nx] Generation receipt written=${written}`);\n        }\n'
   "        await Promise.all(spooling);\n")

# ---- group 2-put: cache puts in bounded slots under shared compiler writer custody (needs conc-cargo group "lease") ------------------
GROUP = "2-put"
op("nx", 'import{withCompilerInitialHeadroomV1}from"../🏗️compiler/📏️capacity/🟦️.ts";',
   'import{withCompilerSharedHeadroomV1}from"../🏗️compiler/📏️capacity/🟦️.ts";\nimport{nxPutSlotCountV1,withNxPutSlotV1}from"./🎟️put-slots/🟦️.ts";\nimport{statfs}from"node:fs/promises";')
put_old = line_of("nx", " return withCompilerInitialHeadroomV1(")
op("nx", put_old,
   ' return withNxPutSlotV1({root:scope.root,leaseDirectory:scope.leaseDirectory,control:scope.control,slots:nxPutSlotCountV1(scope.environment)},async slot=>{const measure=async()=>{const space=await statfs(scope.root,{bigint:true});return Math.max(0,Number(space.bavail*space.bsize)-await slot.others()*maximum);};return withCompilerSharedHeadroomV1({root:scope.root,maximumPhysicalBytes:repositoryStrictStorageV1(scope.repositoryRoot).capacityBytes,recheck:scope.recheck},scope.leaseDirectory,scope.control,requiredBytes,{version:1,wait:true},body,undefined,measure);});')
op("nx", "/** 📏️ Measures native cache-copy demand under exact output readers before queueing for its serialized writer behind concurrent invocations. */\n",
   "/** 📏️ Measures native cache-copy demand under exact output readers, then runs the physical copy in one of a few put slots under the shared compiler writer custody: headroom is the free space minus the maximum artifact charge of every other slot in use, so concurrent puts and compiles of other agents no longer queue behind one another. */\n")

# ---- group 2-daemon: bounded daemons, per-namespace reuse, idle shutdown ----------------------------------------------------------
GROUP = "2-daemon"
op("service", 'export const NX_DAEMON_SERVICE_BIRTH_ENV_V1="SEMIO_NX_DAEMON_SERVICE_BIRTH";',
   'import {acquireNxDaemonSlotV1,nxDaemonIdleV1,nxDaemonLimitsV1} from "../🚏️admission/🟦️.ts";\nexport const NX_DAEMON_SERVICE_BIRTH_ENV_V1="SEMIO_NX_DAEMON_SERVICE_BIRTH";')
op("service", "let release!:()=>void;const released=new Promise<void>(accept=>{release=accept;});",
   'const limits=nxDaemonLimitsV1(process.env),kind=birth.watcherIdentity===null?"finite" as const:"watch" as const,slot=await acquireNxDaemonSlotV1(repositoryLeaseDirectoryV1(birth.repositoryRoot),birth.repositoryRoot,kind,limits[kind],new AbortController().signal);\n let release!:()=>void;const released=new Promise<void>(accept=>{release=accept;});')
op("service", "}));}finally{release();}\n}", "}));}finally{release();slot.release();}\n}")
op("service", "closing:Promise<void>|undefined,stopped=false;\n", "closing:Promise<void>|undefined,stopped=false;\n    const activity={open:0,last:Date.now()};\n")
op("service", 'async request(run){if(!context.live)throw Error("Original daemon root released");const id=randomBytes(16).toString("hex");lifetime.record({type:"request-open",id});try{return await run();}finally{lifetime.record({type:"request-close",id});finish();}}',
   'async request(run){if(!context.live)throw Error("Original daemon root released");const id=randomBytes(16).toString("hex");lifetime.record({type:"request-open",id});activity.open++;activity.last=Date.now();try{return await run();}finally{activity.open--;activity.last=Date.now();lifetime.record({type:"request-close",id});finish();}}')
op("service", 'const cancel=()=>drain("Original daemon service interrupted");original.control.signal.addEventListener("abort",cancel,{once:true});',
   'const cancel=()=>drain("Original daemon service interrupted");original.control.signal.addEventListener("abort",cancel,{once:true});\n    const idleTimer=setInterval(()=>{if(nxDaemonIdleV1({kind,openRequests:activity.open,lastActivity:activity.last,now:Date.now(),idleMilliseconds:limits.idleMilliseconds}))drain(`Idle for ${limits.idleMilliseconds}ms`);},limits.checkMilliseconds);idleTimer.unref();')
op("service", 'finally{context.live=false;ownedDaemon=undefined;original.control.signal.removeEventListener("abort",cancel);}',
   'finally{clearInterval(idleTimer);context.live=false;ownedDaemon=undefined;original.control.signal.removeEventListener("abort",cancel);}')
op("script", "    if(bind)Object.assign(env,(await bind(toolingIdentity,env.SEMIO_BUILD_GRAPH_DIGEST??null,implementationIdentity)).environment);\n",
   "    if(bind)Object.assign(env,(await bind(toolingIdentity,env.SEMIO_BUILD_GRAPH_DIGEST??null,implementationIdentity)).environment);\n"
   "    if(env.NX_DAEMON===undefined&&!invocation.watch&&invocation.args[0]!==\"watch\"){const admission=await import(\"../🕸️graph/🏁️startup/🚏️admission/🟦️.ts\").then(owner=>owner.nxDaemonAdmissionV1(this.root,env,actualPhase.signal)).catch(()=>({enabled:false,reason:\"refused\"}));env.NX_DAEMON=admission.enabled?\"true\":\"false\";if(profileBootstrap)console.log(`[nx] Daemon ${admission.enabled?\"enabled\":\"off\"} (${admission.reason})`);}\n")

new = {
    "generation/module.ts": os.path.join(GEN, TS),
    "generation/schema/schema.json": os.path.join(GEN, SCHEMA, JSONF),
    "generation/tests/tests.ts": os.path.join(GEN, TESTS, TS),
    "generation/tests/fixtures_data.json": os.path.join(GEN, TESTS, FIXT, JSONF),
    "generation/tests/oracle.py": os.path.join(GEN, TESTS, PY),
    "putslots/module.ts": os.path.join(ORDER, E(0x1F39F) + "put-slots", TS),
    "putslots/schema.json": os.path.join(ORDER, E(0x1F39F) + "put-slots", SCHEMA, JSONF),
    "putslots/tests.ts": os.path.join(ORDER, E(0x1F39F) + "put-slots", TESTS, TS),
    "putslots/fixtures_data.json": os.path.join(ORDER, E(0x1F39F) + "put-slots", TESTS, FIXT, JSONF),
    "putslots/oracle.py": os.path.join(ORDER, E(0x1F39F) + "put-slots", TESTS, PY),
    "daemon/module.ts": os.path.join(STARTUP, E(0x1F68F) + "admission", TS),
    "daemon/schema.json": os.path.join(STARTUP, E(0x1F68F) + "admission", SCHEMA, JSONF),
    "daemon/tests.ts": os.path.join(STARTUP, E(0x1F68F) + "admission", TESTS, TS),
    "daemon/fixtures_data.json": os.path.join(STARTUP, E(0x1F68F) + "admission", TESTS, FIXT, JSONF),
    "daemon/oracle.py": os.path.join(STARTUP, E(0x1F68F) + "admission", TESTS, PY),
    "poolorder/tests.ts": os.path.join(NXGRAPH, TESTS, E(0x1F513) + "pool-order", TS),
    "poolorder/fixtures_data.json": os.path.join(NXGRAPH, TESTS, E(0x1F513) + "pool-order", FIXT, JSONF),
    "poolorder/oracle.py": os.path.join(NXGRAPH, TESTS, E(0x1F513) + "pool-order", PY),
}
for extra, target in json.load(open(os.path.join(HERE, "staged", "new.json"))).items() if os.path.exists(os.path.join(HERE, "staged", "new.json")) else []:
    new[extra] = target
bases = {path: hashlib.sha256(read(path).encode()).hexdigest() for path in sorted(set(row["path"] for row in ops))}
os.makedirs(os.path.join(HERE, "staged"), exist_ok=True)
groups = {name: ("2-put" if name.startswith("putslots/") else "2-daemon" if name.startswith("daemon/") else "1") for name in new}
json.dump({"root": ROOT, "ops": ops, "bases": bases, "new": new, "groups": groups}, open(os.path.join(HERE, "staged", "ops.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
print("ops %d, new %d, files %d" % (len(ops), len(new), len(bases)))
