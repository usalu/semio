# Anchored edits of live files. Every `old` must occur exactly once in the base text (or `new` already be present, which makes the edit idempotent).
# Groups: lease (shared writer custody), lane (lanes + unit cache in `repo cargo`), registry (coalesced registry fetch), admit (per-unit admission memo), config (.cargo/config.toml), component (optional hook).
R = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
F = "🧰️framework/🔨️modules/🏃️process"
CAP = R + "/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity"
LAUNCHER = R + "/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts"
DEPS = R + "/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts"
READY = R + "/🏃️process/🦀️cargo/📥️dependencies/🟢️ready/💾️storage/🟦️.ts"
FETCHED = R + "/🏃️process/🦀️cargo/📥️dependencies/🧾️fetched/🟦️.ts"
FETCHED_SCHEMA = R + "/🏃️process/🦀️cargo/📥️dependencies/🧾️fetched/🧬️schema/🔣️.json"
FETCHED_TEST = R + "/🏃️process/🦀️cargo/📥️dependencies/🧾️fetched/🧪️tests/🟦️.ts"
FETCHED_FIXTURE = R + "/🏃️process/🦀️cargo/📥️dependencies/🧾️fetched/🧪️tests/🔣️.json"
ADMISSION = F + "/📦️artifacts/🏗️native-build/📥️source/🎟️admission/🟦️.ts"
CONFIG = ".cargo/config.toml"
COMPONENT = F + "/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts"
UC = R + "/⚡️caching/🦀️cargo/🗃️unit-cache"
UCL = "../../../⚡️caching/🦀️cargo/🗃️unit-cache"

IMPORT_CAP = 'import{withCompilerWorkspaceWriteV1}from"../../../⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🟦️.ts";'

EDITS = {}


def edit(target, group, name, old, new):
    EDITS.setdefault(target, []).append((group, name, old, new))


# ---------------------------------------------------------------- lease: the writer custody of a compile is shared, exclusive only to recover headroom
edit(CAP + "/🟦️.ts", "lease", "custody-mode",
     'leaseDirectory:string,control:ResourceLeaseControl,request:Readonly<{version:1;wait:boolean}>,body:(control:ResourceLeaseControl)=>Promise<T>):Promise<T>{\n requireIssuedResourceControlV1(control);if(validateJsonSchemaSubset(schema.definitions.WriterRequest',
     'leaseDirectory:string,control:ResourceLeaseControl,request:Readonly<{version:1;wait:boolean;mode?:"exclusive"|"shared"}>,body:(control:ResourceLeaseControl)=>Promise<T>):Promise<T>{\n requireIssuedResourceControlV1(control);if(validateJsonSchemaSubset(schema.definitions.WriterRequest')
edit(CAP + "/🟦️.ts", "lease", "custody-lease",
     'mode:"exclusive" as const,owner:randomBytes(16).toString("base64url"),control,signal:control.signal}',
     'mode:(request.mode??"exclusive") as "exclusive"|"shared",...(request.mode==="shared"?{sharedCohort:true}:{}),owner:randomBytes(16).toString("base64url"),control,signal:control.signal}')
edit(CAP + "/🟦️.ts", "lease", "shared-write",
     '\n/** 🔏️ Requires the actual live shared compiler writer',
     '''
/** 🏪️ Admits free headroom under a SHARED writer custody, so concurrent compiles never wait for each other, and escalates to an exclusive custody only to recover missing headroom; the body then runs shared. Cargo compiles use this; maintenance keeps the exclusive custody of `withCompilerInitialHeadroomV1`. */
export async function withCompilerSharedHeadroomV1<T>(image:Pick<NativeImageStorageV1,"root"|"maximumPhysicalBytes"|"recheck">,leaseDirectory:string,control:ResourceLeaseControl,requiredBytes:number,request:Readonly<{version:1;wait:boolean}>,body:(control:ResourceLeaseControl)=>Promise<T>,recover?:()=>Promise<void>,measure?:()=>Promise<number>):Promise<T>{
 const available=measure??(async():Promise<number>=>{const fs=await statfs(image.root,{bigint:true});return Number(fs.bavail*fs.bsize);});
 const enough=await withCompilerWriterCustodyV1(image,leaseDirectory,control,{...request,mode:"shared"},async actual=>{checkResourceLeaseControl(actual);return compilerHeadroomAdmittedV1({version:1,availableBytes:await available(),requiredBytes});});
 if(!enough)await withCompilerWriterCustodyV1(image,leaseDirectory,control,{...request,mode:"exclusive"},async actual=>{checkResourceLeaseControl(actual);let availableBytes=await available();if(!compilerHeadroomAdmittedV1({version:1,availableBytes,requiredBytes})&&recover){await recover();await image.recheck();availableBytes=await available();}checkResourceLeaseControl(actual);if(!compilerHeadroomAdmittedV1({version:1,availableBytes,requiredBytes}))throw Error("Insufficient physical compiler initial headroom");});
 return withCompilerWriterCustodyV1(image,leaseDirectory,control,{...request,mode:"shared"},body);
}
/** 🎟️ Like `withCompilerWorkspaceWriteV1` for compilers that only grow private lanes and the content-addressed cache: concurrent compiles of one image hold the writer custody together. */
export function withCompilerWorkspaceSharedWriteV1<T>(storage:NativeStorageReservationV1,leaseDirectory:string,control:ResourceLeaseControl,body:(control:ResourceLeaseControl)=>Promise<T>):Promise<T>{const owner=workspaceOwner(storage,control),current=writers.getStore(),write=(actual:ResourceLeaseControl)=>withCompilerWorkspaceBorrowV1(storage,actual,phase=>withCompilerSharedHeadroomV1({root:storage.backing.root,maximumPhysicalBytes:storage.backing.maximumPhysicalBytes,recheck:storage.recheck},leaseDirectory,phase,storage.allocation.bytes,{version:1,wait:true},body,owner.recover));return current?.root===owner.root?writerOperation(current,control,write):write(control);}

/** 🔏️ Requires the actual live shared compiler writer''')

edit(CAP + "/🧬️schema/🔣️.json", "lease", "writer-request-mode",
     '''        "wait": {
          "type": "boolean"
        }
      }''',
     '''        "wait": {
          "type": "boolean"
        },
        "mode": {
          "enum": [
            "exclusive",
            "shared"
          ]
        }
      }''')

# ---------------------------------------------------------------- lane: `repo cargo --store=shared` runs in a lane with the unit cache
edit(LAUNCHER, "lane", "imports", IMPORT_CAP,
     'import{withCompilerWorkspaceSharedWriteV1}from"../../../⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🟦️.ts";\n'
     'import {acquireLaneV1,resolveLaneIdV1,systemAncestryV1} from "' + UCL + '/🧵️lane/🟦️.ts";\n'
     'import {collectGarbageV1} from "' + UCL + '/🧹️gc/🟦️.ts";\n'
     'import {focusFromCargoArgumentsV1,laneLanguageV1,laneMessageV1,unitCacheEnvironmentV1,unitCacheRootV1} from "' + UCL + '/🚪️launch/🟦️.ts";\n'
     'import {unitAdmissionMemoV1} from "' + UCL + '/📖️admission/🟦️.ts";')
edit(LAUNCHER, "lane", "lane-store",
     '\n/** 📍️ Selects the manifest Cargo itself would read',
     '''
/** 🧵️ One Cargo command's lane: a private target and build directory of its agent inside the issued compiler store. A lane is held by one Cargo process at a time, so Cargo's unit locks never span two processes; the first free lane of the agent is used and a busy one overflows into a sibling that fills from the shared content-addressed unit cache. Cleanup of idle lanes and cold cache objects runs here at most every ten minutes, inside the image's budget. */
export async function withRepositoryCargoLaneV1<T>(options:Readonly<{storage:NativeStorageReservationV1;cacheRoot:string;capacityBytes:number;environment:Readonly<Record<string,string|undefined>>;temporary:string;control:ResourceLeaseControl}>,body:(store:Readonly<{target:string;build:string;temporary:string;lane:Readonly<{id:string;root:string}>}>)=>Promise<T>):Promise<T>{
 requireNativeStorageReservationV1(options.storage,options.control);await options.storage.recheck();
 const root=options.storage.backing.root,language=laneLanguageV1(options.environment);requireNativeStoragePathV1(root,options.cacheRoot);mkdirSync(options.cacheRoot,{recursive:true});
 const base=resolveLaneIdV1({environment:options.environment,ancestry:systemAncestryV1}),lane=acquireLaneV1(options.cacheRoot,base);
 try{
  for(const path of[lane.paths.target,lane.paths.build])requireNativeStoragePathV1(root,path);
  if(lane.overflow>0)process.stderr.write(laneMessageV1("overflow",{base,lane:lane.paths.id},language)+"\\n");
  return await body(Object.freeze({target:lane.paths.target,build:lane.paths.build,temporary:options.temporary,lane:Object.freeze({id:lane.paths.id,root:lane.paths.base})}));
 }finally{
  lane.release();
  try{const stamp=join(options.cacheRoot,"gc.stamp");let due=true;try{due=Date.now()-statSync(stamp).mtimeMs>600000;}catch{}if(due){writeFileSync(stamp,String(Date.now()));const{applied}=collectGarbageV1(options.cacheRoot,options.capacityBytes);if(applied.lanes+applied.incremental+applied.objects>0)process.stderr.write(laneMessageV1("collected",applied,language)+"\\n");}}catch{}
 }
}

/** 📍️ Selects the manifest Cargo itself would read''')
edit(LAUNCHER, "lane", "fs-imports", 'import {copyFileSync,mkdirSync,readdirSync,rmSync} from "node:fs";',
     'import {copyFileSync,mkdirSync,readdirSync,rmSync,statSync,writeFileSync} from "node:fs";')
edit(LAUNCHER, "lane", "admitted-type",
     'selected:RepositoryCargoIsolatedStoreV1|Readonly<{target:string;build:string}>,control:ResourceLeaseControl,beforeConsume',
     'selected:RepositoryCargoIsolatedStoreV1|Readonly<{target:string;build:string;lane?:Readonly<{id:string;root:string}>}>,control:ResourceLeaseControl,beforeConsume')
edit(LAUNCHER, "lease", "shared-writer",
     'return withCompilerWorkspaceWriteV1(store.storage,leaseDirectory,control,async()=>{\n   mark("writer","ready")',
     'return withCompilerWorkspaceSharedWriteV1(store.storage,leaseDirectory,control,async()=>{\n   mark("writer","ready")')
edit(LAUNCHER, "lane", "compiler-chain",
     'RUSTC_WRAPPER:scheduler,[ADMISSION_POLICY_V1.brokerEnvironment]:broker.endpoint',
     'RUSTC_WRAPPER:scheduler,...("lane"in selected&&selected.lane?unitCacheEnvironmentV1({cacheRoot:unitCacheRootV1(store.root),lane:selected.lane.id,laneRoot:selected.lane.root,scheduler,compiler:env.RUSTC??"rustc",toolchain:store.toolchain.digest,focus:focusFromCargoArgumentsV1(compilerArgs)}):{}),[ADMISSION_POLICY_V1.brokerEnvironment]:broker.endpoint')
edit(LAUNCHER, "lane", "run-type",
     'const run=async(selected:RepositoryCargoIsolatedStoreV1|Readonly<{target:string;build:string;temporary:string}>)',
     'const run=async(selected:RepositoryCargoIsolatedStoreV1|Readonly<{target:string;build:string;temporary:string;lane?:Readonly<{id:string;root:string}>}>)')
edit(LAUNCHER, "lane", "shared-store",
     'return request.store==="shared"?run({target:store.environment.CARGO_TARGET_DIR!,build:store.buildDirectory,temporary:store.environment.TMPDIR!}):',
     'return request.store==="shared"?withRepositoryCargoLaneV1({storage:store.storage,cacheRoot:unitCacheRootV1(store.root),capacityBytes:readNativeStorageReservationPolicyV1(store.storage,store.control).capacityBytes,environment,temporary:store.environment.TMPDIR!,control},run):')
edit(LAUNCHER, "admit", "memo-argument",
     'await admitCargoSourceBoundariesV1(graph,workspace,store.sourceBoundary,control,actualCargoSourceAdmissionPortV1(compilerEnvironment,cwd,store.storage));',
     'await admitCargoSourceBoundariesV1(graph,workspace,store.sourceBoundary,control,actualCargoSourceAdmissionPortV1(compilerEnvironment,cwd,store.storage),unitAdmissionMemoV1(unitCacheRootV1(store.root),store.toolchain.digest));')

# ---------------------------------------------------------------- admit: one memo per unit replaces the per-unit reads, cfg probe and durable rechecks while its files keep their stamps
edit(ADMISSION, "admit", "memo-type",
     'export type CargoSourceAdmissionRecheckPhaseV1="unit"|"complete";',
     'export type CargoSourceAdmissionRecheckPhaseV1="unit"|"complete";\n'
     '/** 🧠️ Remembers whole-unit admissions across invocations; a recalled unit reads no Source and takes no durable recheck. */\n'
     'export interface CargoSourceAdmissionMemoV1 {recall(unit:CargoCompilationUnitV1,policy:unknown):Readonly<{inputs:readonly CargoCompilationUnitV1["sourceInputs"][number][];cfg:readonly string[];environment:CargoCompilationUnitV1["compilerEnvironment"]}>|undefined;remember(unit:CargoCompilationUnitV1,policy:unknown,result:Readonly<{inputs:readonly CargoCompilationUnitV1["sourceInputs"][number][];cfg:readonly string[];environment:CargoCompilationUnitV1["compilerEnvironment"];probed:readonly string[];startedMs:number}>):void}')
edit(ADMISSION, "admit", "memo-parameter",
     'control:ResourceLeaseControl,port:CargoSourceAdmissionPortV1):Promise<void>{\n checkResourceLeaseControl(control);if(validateJsonSchemaSubset(schema,policy).length)throw Error("Closed ordinary Cargo Source boundary policy required");',
     'control:ResourceLeaseControl,port:CargoSourceAdmissionPortV1,memo?:CargoSourceAdmissionMemoV1):Promise<void>{\n checkResourceLeaseControl(control);if(validateJsonSchemaSubset(schema,policy).length)throw Error("Closed ordinary Cargo Source boundary policy required");')
edit(ADMISSION, "admit", "memo-recall",
     'await publish("cfg");await port.recheck(control,"unit");',
     'await publish("cfg");const startedMs=Date.now(),remembered=memo?.recall(unit,policy);if(remembered){unit.sourceInputs=remembered.inputs.map(input=>({...input}));unit.compilerCfg=[...remembered.cfg];unit.compilerEnvironment=remembered.environment;files+=remembered.inputs.length;readBytes+=remembered.inputs.reduce((sum,input)=>sum+input.bytes,0);await publish("complete");continue;}await port.recheck(control,"unit");')
edit(ADMISSION, "admit", "memo-probes",
     'seen=new Map<string,number>(),sourceBytes=new Map<string,Uint8Array>();let exceeded:Error|undefined;',
     'seen=new Map<string,number>(),sourceBytes=new Map<string,Uint8Array>(),probed:string[]=[];let exceeded:Error|undefined;')
edit(ADMISSION, "admit", "memo-probe-record",
     'const inputs=await readRustSourceClosureV1([unit.entry],root,cfg,async path=>{checkResourceLeaseControl(control);if(unitReads===0)await publish("read");',
     'const inputs=await readRustSourceClosureV1([unit.entry],root,cfg,async path=>{checkResourceLeaseControl(control);probed.push(path);if(unitReads===0)await publish("read");')
edit(ADMISSION, "admit", "memo-remember",
     'active lines "+lines+"/"+limits.maxActiveLines+")");await publish("complete");',
     'active lines "+lines+"/"+limits.maxActiveLines+")");memo?.remember(unit,policy,{inputs:unit.sourceInputs,cfg,environment:unit.compilerEnvironment,probed,startedMs});await publish("complete");')

# ---------------------------------------------------------------- registry: cargo runs hold the registry shared; only a coalesced per-request fetch is exclusive, and receipts stamp only the locked packages
edit(DEPS, "lease", "dependencies-import", IMPORT_CAP,
     'import{withCompilerWorkspaceSharedWriteV1,withCompilerWorkspaceWriteV1}from"../../../⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🟦️.ts";')
edit(DEPS, "registry", "fetch-shared",
     'withRepositoryCargoRegistryReadyV1({...options,requests,home,environment},()=>withCompilerWorkspaceWriteV1(storage,',
     'withRepositoryCargoRegistryReadyV1({...options,requests,home,environment},()=>withCompilerWorkspaceSharedWriteV1(storage,')
edit(DEPS, "registry", "compile-shared",
     'const result=await withCompilerWorkspaceWriteV1(held.storage,join(held.workspace,".cache/b/l/m/data"),input.control,()=>body(',
     'const result=await withCompilerWorkspaceSharedWriteV1(held.storage,join(held.workspace,".cache/b/l/m/data"),input.control,()=>body(')
edit(READY, "registry", "fetch-key",
     'lease={directory,resource,signal:control.signal,control},identity=requestIdentity(options),',
     'lease={directory,resource,signal:control.signal,control},identity=requestIdentity(options),fetchLease={directory,resource:"cargo:registry-fetch:"+options.home+":"+(identity??"-"),signal:control.signal,control},')
edit(READY, "registry", "fetch-writer",
     'const writer=await acquireQueuedResourceLease({...lease,owner:randomBytes(8).toString("base64url"),mode:"exclusive"});try{',
     'const writer=await acquireQueuedResourceLease({...fetchLease,owner:randomBytes(8).toString("base64url"),mode:"exclusive"});try{')
edit(READY, "registry", "file-stamps",
     'return row.isDirectory()&&!row.isSymbolicLink()?{dev:String(row.dev)',
     'return (row.isDirectory()||row.isFile())&&!row.isSymbolicLink()?{dev:String(row.dev)')

# receipts stamp only the locked packages' archives (one aggregate digest), so a fetch of anything else never invalidates them
edit(FETCHED, "registry", "receipt-type",
     'registry:readonly Readonly<{path:string;dev:string;ino:string;mtimeNs:string}>[]}>;',
     'registry:Readonly<{digest:string;count:number}>}>;')
edit(FETCHED, "registry", "stamps",
     '''/** 📸️ Stamps the registry cache and extraction directories and every directory directly below them. */
export function fetchedRegistryStampsV1(home:string,port:FetchedPortV1):FetchedReceiptV1["registry"]|undefined{
 const rows:Array<FetchedReceiptV1["registry"][number]>=[];
 for(const parent of protocol.registryDirectories){
  const listed=port.list(underHome(home,parent));if(listed===undefined)continue;const names=[...listed].sort();
  for(const path of[parent,...names.map(name=>posix.join(parent,name))]){const stamp=port.stamp(underHome(home,path));if(!stamp)return undefined;rows.push(Object.freeze({path,...stamp}));}
 }
 return rows.length>protocol.maximumRegistryEntries?undefined:Object.freeze(rows);
}''',
     '''/** 📦️ The `name-version` of every registry package a lockfile pins, `undefined` for a lockfile that cannot be read. */
export function fetchedPackagesV1(text:string):string[]|undefined{
 let document:{package?:Array<{name?:unknown;version?:unknown;source?:unknown}>};try{document=Bun.TOML.parse(text) as typeof document;}catch{return undefined;}
 if(!Array.isArray(document.package))return undefined;
 const rows:string[]=[];
 for(const row of document.package){if(row.source===undefined)continue;if(typeof row.name!=="string"||typeof row.version!=="string"||/[\\\\/]/u.test(row.name+row.version))return undefined;rows.push(row.name+"-"+row.version);}
 return[...new Set(rows)].sort();
}

/** 📸️ Stamps the downloaded archive of every locked package, in whichever registry index directory holds it, and folds the stamps into one digest. A package that was not fetched yields `undefined`; adding, replacing or removing any other package never changes the digest. */
export function fetchedRegistryStampsV1(home:string,port:FetchedPortV1,packages:readonly string[]):FetchedReceiptV1["registry"]|undefined{
 const listed=port.list(underHome(home,protocol.registryCache));if(listed===undefined||packages.length===0||packages.length>protocol.maximumPackages)return undefined;
 const indexes=[...listed].sort(),rows:string[][]=[];
 for(const name of packages){let found=false;for(const index of indexes){const path=posix.join(protocol.registryCache,index,name+".crate"),stamp=port.stamp(underHome(home,path));if(stamp){rows.push([path,stamp.dev,stamp.ino,stamp.mtimeNs]);found=true;break;}}if(!found)return undefined;}
 return Object.freeze({digest:digestOf(Buffer.from(JSON.stringify(rows))),count:rows.length});
}''')
edit(FETCHED, "registry", "capture",
     ''' const locks:Array<FetchedReceiptV1["locks"][number]>=[];
 for(const path of[...paths].sort()){const bytes=await port.read(path);if(!bytes||bytes.byteLength>protocol.maximumLockBytes||classifyFetchedLockV1(Buffer.from(bytes).toString("utf8"))!=="registry")return undefined;locks.push(Object.freeze({path,digest:digestOf(bytes),bytes:bytes.byteLength}));}
 const registry=fetchedRegistryStampsV1(input.home,port);if(!registry?.length)return undefined;''',
     ''' const locks:Array<FetchedReceiptV1["locks"][number]>=[],packages=new Set<string>();
 for(const path of[...paths].sort()){const bytes=await port.read(path);if(!bytes||bytes.byteLength>protocol.maximumLockBytes)return undefined;const text=Buffer.from(bytes).toString("utf8"),named=fetchedPackagesV1(text);if(classifyFetchedLockV1(text)!=="registry"||!named)return undefined;for(const name of named)packages.add(name);locks.push(Object.freeze({path,digest:digestOf(bytes),bytes:bytes.byteLength}));}
 const registry=fetchedRegistryStampsV1(input.home,port,[...packages].sort());if(!registry)return undefined;''')
edit(FETCHED, "registry", "holds",
     ''' for(const lock of receipt.locks){const bytes=await port.read(lock.path);if(!bytes||bytes.byteLength!==lock.bytes||digestOf(bytes)!==lock.digest)return false;}
 const now=fetchedRegistryStampsV1(home,port);
 return now!==undefined&&now.length===receipt.registry.length&&now.every((row,index)=>{const kept=receipt.registry[index]!;return row.path===kept.path&&row.dev===kept.dev&&row.ino===kept.ino&&row.mtimeNs===kept.mtimeNs;});''',
     ''' const packages=new Set<string>();
 for(const lock of receipt.locks){const bytes=await port.read(lock.path);if(!bytes||bytes.byteLength!==lock.bytes||digestOf(bytes)!==lock.digest)return false;const named=fetchedPackagesV1(Buffer.from(bytes).toString("utf8"));if(!named)return false;for(const name of named)packages.add(name);}
 const now=fetchedRegistryStampsV1(home,port,[...packages].sort());
 return now!==undefined&&now.count===receipt.registry.count&&now.digest===receipt.registry.digest;''')
edit(FETCHED_SCHEMA, "registry", "protocol",
     '''        "maximumRegistryEntries": 512,
        "maximumReceiptBytes": 262144,
        "registryDirectories": ["registry/cache", "registry/src"]''',
     '''        "maximumPackages": 8192,
        "maximumReceiptBytes": 262144,
        "registryCache": "registry/cache"''')
edit(FETCHED_SCHEMA, "registry", "stamp-definition",
     '''    "Stamp": {
      "type": "object",
      "additionalProperties": false,
      "required": ["path", "dev", "ino", "mtimeNs"],
      "properties": {
        "path": {"type": "string", "minLength": 1, "maxLength": 256},
        "dev": {"type": "string", "pattern": "^[0-9]+$"},
        "ino": {"type": "string", "pattern": "^[0-9]+$"},
        "mtimeNs": {"type": "string", "pattern": "^[0-9]+$"}
      }
    },
''',
     "")
edit(FETCHED_SCHEMA, "registry", "receipt-registry",
     '''"registry": {"type": "array", "maxItems": 512, "items": {"$ref": "#/definitions/Stamp"}}''',
     '''"registry": {
          "type": "object",
          "additionalProperties": false,
          "required": ["digest", "count"],
          "properties": {
            "digest": {"type": "string", "pattern": "^[a-f0-9]{64}$"},
            "count": {"type": "integer", "minimum": 1, "maximum": 8192}
          }
        }''')

# ================================================================ group 2: shared build script runs (`buildscript`) and broker-visible cache hits (`broker`)
BROKER = F + "/📦️artifacts/🏗️native-build/🎟️admission/🟦️.ts"
BROKER_SCHEMA = F + "/📦️artifacts/🏗️native-build/🎟️admission/🧬️schema/🔣️.json"
STORE_COMPILER = R + "/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts"

edit(LAUNCHER, "buildscript", "features",
     'focus:focusFromCargoArgumentsV1(compilerArgs)})',
     'focus:focusFromCargoArgumentsV1(compilerArgs),features:{buildScripts:true}})')

edit(BROKER, "broker", "record-type",
     'userMs: number; sysMs: number; exit: number }>;',
     'userMs: number; sysMs: number; exit: number; cache?: Readonly<{ key: string; recorded: boolean }> | null }>;')
edit(BROKER, "broker", "request-types",
     '/** 📒️ One recorded observation of a unit key',
     """/** 📦️ The measurements recorded when a cached unit was first built (`ProvenanceV1`). */
export type CachedProvenanceV1 = Readonly<{ peakRssBytes: number; userMs: number; sysMs: number; wallMs: number; rmetaOffsetMs: number | null }>;

/** 📣️ A wrapper's report that a unit was served from the unit cache instead of compiled (`CachedRequestV1`): the admission facts, the cache key and the recorded measurements. */
export type CachedRequestV1 = Readonly<Omit<AdmitRequestV1, "op"> & { op: "cached"; cacheKey: string; provenance: CachedProvenanceV1 | null }>;

/** 🔭️ A wrapper's question for the finished record of the unit its scheduler just reported (`RecallRequestV1`). */
export type RecallRequestV1 = Readonly<{ v: 1; op: "recall"; pass: string; crate: string; outDir: string; extraFilename: string }>;

/** 📒️ One recorded observation of a unit key""")
edit(BROKER, "broker", "handler",
     'const message = JSON.parse(line) as { op?: string };\n',
     """const message = JSON.parse(line) as { op?: string };
        if (message.op === "cached" && !entry) {
          const request = admitDefinition<CachedRequestV1>("CachedRequestV1", message), pass = passes.get(request.pass);
          if (!pass) throw Error(`Unknown build pass ${request.pass}`);
          const at = now(), recorded = request.provenance;
          completed.push(Object.freeze({ key: unitKeyV1(request, pass.profile), pass: request.pass, profile: pass.profile, target: request.target, crate: request.crate, package: request.package, version: request.version, crateTypes: request.crateTypes, primary: request.primary, artifact: unitArtifactV1(request), dependencies: request.externs.map((path) => { const extern = externArtifactsV1(path); return { artifacts: extern.candidates, edge: extern.edge }; }), buildScriptOutDir: request.buildScriptOutDir, predictedBytes: 0, requestMs: at, admittedMs: at, startMs: at, endMs: at, rmetaMs: recorded?.rmetaOffsetMs == null ? null : at, peakRssBytes: recorded?.peakRssBytes ?? 0, userMs: recorded?.userMs ?? 0, sysMs: recorded?.sysMs ?? 0, exit: 0, cache: Object.freeze({ key: request.cacheKey, recorded: recorded !== null }) }));
          finished = true;
          lastActivityMs = at;
          verdict(socket, true);
          return;
        }
        if (message.op === "recall" && !entry) {
          const request = admitDefinition<RecallRequestV1>("RecallRequestV1", message), artifact = unitArtifactV1(request);
          const record = [...completed].reverse().find((row) => row.pass === request.pass && row.exit === 0 && !row.cache && row.artifact === artifact) ?? null;
          if (!socket.destroyed) socket.write(`${JSON.stringify({ v: 1, op: "unit", record })}\\n`);
          finished = true;
          return;
        }
""")
edit(BROKER, "broker", "observations",
     'return units.filter((unit) => unit.exit === 0).map((unit) =>',
     'return units.filter((unit) => unit.exit === 0 && !unit.cache).map((unit) =>')
edit(BROKER, "broker", "os-import",
     'import { tmpdir, totalmem } from "node:os";',
     'import { availableParallelism, tmpdir, totalmem } from "node:os";')
edit(BROKER, "broker", "wrapper-chain",
     '/** 🪜️ Recorded observations of a finished build',
     """/** 🗃️ The compiler chain of a build whose environment names a unit cache (`SEMIO_UNIT_CACHE_ROOT` and the wrapper script): Cargo runs the cache's wrapper, which serves a unit from the cache or falls through to this driver's scheduler, and reports every served unit to the broker so the unit list, critical path and provenance stay complete. Empty when no cache is named. */
export function compilerWrapperEnvironmentV1(scheduler: string, laneRoot: string, environment: Readonly<Record<string, string | undefined>>): Record<string, string> {
  const root = environment.SEMIO_UNIT_CACHE_ROOT, script = environment.SEMIO_UNIT_CACHE_SCRIPT;
  if (!root || !script || environment.SEMIO_UNIT_CACHE === "off") return {};
  return { RUSTC_WRAPPER: process.execPath, RUSTC: script, SEMIO_UNIT_CACHE_ROOT: root, SEMIO_UNIT_CACHE_LANE: "g", SEMIO_UNIT_CACHE_LANE_ROOT: laneRoot, SEMIO_UNIT_CACHE_INNER: JSON.stringify([scheduler, environment.RUSTC ?? "rustc"]), SEMIO_UNIT_CACHE_SLOTS: environment.SEMIO_UNIT_CACHE_SLOTS ?? String(availableParallelism()), SEMIO_UNIT_CACHE_FOCUS: "", SEMIO_UNIT_CACHE_BUILD_SCRIPTS: "on", SEMIO_UNIT_CACHE_BROKER_REPLAY: "on", ...(environment.SEMIO_UNIT_CACHE_TOOLCHAIN ? { SEMIO_UNIT_CACHE_TOOLCHAIN: environment.SEMIO_UNIT_CACHE_TOOLCHAIN } : {}) };
}

/** 🪜️ Recorded observations of a finished build""")
edit(BROKER_SCHEMA, "broker", "definitions",
     '    "VerdictV1": {',
     """    "ProvenanceV1": {
      "type": "object",
      "additionalProperties": false,
      "required": ["peakRssBytes", "userMs", "sysMs", "wallMs", "rmetaOffsetMs"],
      "properties": {
        "peakRssBytes": {"type": "integer", "minimum": 0},
        "userMs": {"type": "integer", "minimum": 0},
        "sysMs": {"type": "integer", "minimum": 0},
        "wallMs": {"type": "integer", "minimum": 0},
        "rmetaOffsetMs": {"anyOf": [{"type": "integer", "minimum": 0}, {"type": "null"}]}
      }
    },
    "CachedRequestV1": {
      "type": "object",
      "additionalProperties": false,
      "required": ["v", "op", "pass", "pid", "crate", "package", "version", "target", "crateTypes", "outDir", "extraFilename", "source", "manifestDir", "primary", "buildScriptOutDir", "externs", "cacheKey", "provenance"],
      "properties": {
        "v": {"const": 1},
        "op": {"const": "cached"},
        "pass": {"type": "string", "pattern": "^[A-Za-z0-9_.:-]{1,128}$"},
        "pid": {"type": "integer", "minimum": 1},
        "crate": {"type": "string", "pattern": "^[A-Za-z0-9_]{1,128}$"},
        "package": {"type": "string", "maxLength": 128},
        "version": {"type": "string", "maxLength": 128},
        "target": {"type": "string", "pattern": "^[A-Za-z0-9_.-]{1,128}$"},
        "crateTypes": {"type": "array", "minItems": 1, "items": {"enum": ["lib", "rlib", "dylib", "cdylib", "staticlib", "proc-macro", "bin"]}},
        "outDir": {"$ref": "#/definitions/Path"},
        "extraFilename": {"type": "string", "maxLength": 128},
        "source": {"$ref": "#/definitions/Path"},
        "manifestDir": {"$ref": "#/definitions/Path"},
        "primary": {"type": "boolean"},
        "buildScriptOutDir": {"anyOf": [{"$ref": "#/definitions/Path"}, {"type": "null"}]},
        "externs": {"type": "array", "items": {"$ref": "#/definitions/Path"}},
        "cacheKey": {"type": "string", "pattern": "^[a-f0-9]{64}$"},
        "provenance": {"anyOf": [{"$ref": "#/definitions/ProvenanceV1"}, {"type": "null"}]}
      }
    },
    "RecallRequestV1": {
      "type": "object",
      "additionalProperties": false,
      "required": ["v", "op", "pass", "crate", "outDir", "extraFilename"],
      "properties": {
        "v": {"const": 1},
        "op": {"const": "recall"},
        "pass": {"type": "string", "pattern": "^[A-Za-z0-9_.:-]{1,128}$"},
        "crate": {"type": "string", "pattern": "^[A-Za-z0-9_]{1,128}$"},
        "outDir": {"$ref": "#/definitions/Path"},
        "extraFilename": {"type": "string", "maxLength": 128}
      }
    },
    "VerdictV1": {""")

edit(COMPONENT, "broker", "import",
     'import { ADMISSION_POLICY_V1, admissionMemoryBytesV1, buildSchedulerV1,',
     'import { ADMISSION_POLICY_V1, admissionMemoryBytesV1, buildSchedulerV1, compilerWrapperEnvironmentV1,')
edit(COMPONENT, "broker", "job-site",
     'RUSTC_WRAPPER: scheduler, [ADMISSION_POLICY_V1.brokerEnvironment]: broker.endpoint, [ADMISSION_POLICY_V1.passEnvironment]: job.id,',
     'RUSTC_WRAPPER: scheduler, ...compilerWrapperEnvironmentV1(scheduler, generation.root, options.environment), [ADMISSION_POLICY_V1.brokerEnvironment]: broker.endpoint, [ADMISSION_POLICY_V1.passEnvironment]: job.id,')
edit(COMPONENT, "broker", "pass-site",
     'RUSTC_WRAPPER: scheduler, [ADMISSION_POLICY_V1.brokerEnvironment]: broker.endpoint, [ADMISSION_POLICY_V1.passEnvironment]: pass.id,',
     'RUSTC_WRAPPER: scheduler, ...compilerWrapperEnvironmentV1(scheduler, generation.root, options.environment), [ADMISSION_POLICY_V1.brokerEnvironment]: broker.endpoint, [ADMISSION_POLICY_V1.passEnvironment]: pass.id,')
edit(STORE_COMPILER, "broker", "store-environment-import",
     'import {repositoryCompilerLaunchEnvironmentV1} from "./🚪️ingress/🟦️.ts";',
     'import {repositoryCompilerLaunchEnvironmentV1} from "./🚪️ingress/🟦️.ts";\nimport {UNIT_CACHE_SCRIPT_V1,unitCacheRootV1} from "../../../🦀️cargo/🗃️unit-cache/🚪️launch/🟦️.ts";')
edit(STORE_COMPILER, "broker", "store-environment",
     'CARGO_BUILD_BUILD_DIR:buildDirectory,TMPDIR:temporary,',
     'CARGO_BUILD_BUILD_DIR:buildDirectory,SEMIO_UNIT_CACHE_ROOT:unitCacheRootV1(opened.root),SEMIO_UNIT_CACHE_SCRIPT:UNIT_CACHE_SCRIPT_V1,TMPDIR:temporary,')

# ---------------------------------------------------------------- config: the declared store equals the store in use
edit(CONFIG, "config", "paths",
     'target-dir = ".🧬semio/🦑️repo/⚡️cache/rust/fixed/m/cargo/shared/target"\nbuild-dir = ".🧬semio/🦑️repo/⚡️cache/rust/fixed/m/cargo/shared/build"',
     'target-dir = ".🧬semio/🦑️repo/⚡️cache/rust/fixed/m/u/l/d/t"\nbuild-dir = ".🧬semio/🦑️repo/⚡️cache/rust/fixed/m/u/l/d/b"')
edit(CONFIG, "config", "comment",
     '# ⚡️ Repository launchers open the declared fixed Rust backing before compilation. Static paths select its\n# shared namespace; managed prototype and release facades retain their explicitly issued store paths.\n# `fine-grain-locking` serializes compilation units; `checksum-freshness` compares actual source contents.',
     '# ⚡️ Repository launchers open the declared fixed Rust backing before compilation. Every agent compiles in its own lane\n# (`m/u/l/<lane>/{t,b}`) and shares results only through the content-addressed unit cache (`m/u`); the static paths below name the\n# default lane `d`, which an unattributed command uses. No two Cargo processes share a build directory, so Cargo\'s unit locks\n# cannot form a cycle between processes; `fine-grain-locking` stays for the passes of one driver generation.\n# `checksum-freshness` compares actual source contents.')

REWRITES = {FETCHED_TEST: "registry", FETCHED_FIXTURE: "registry"}
GROUPS = ["lease", "lane", "admit", "registry", "config", "buildscript", "broker"]
GROUPS_1 = ["lease", "lane", "admit", "registry", "config"]
GROUPS_2 = ["buildscript", "broker"]
