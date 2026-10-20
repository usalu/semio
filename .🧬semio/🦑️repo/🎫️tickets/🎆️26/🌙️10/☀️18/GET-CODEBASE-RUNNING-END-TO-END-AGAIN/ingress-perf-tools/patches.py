"""🧾 Anchored edits of the ingress-perf set: (keyword path, old anchor, new text, label)."""
STORAGE = "framework/modules/process/artifacts/native-build/storage"
EDITS = []
NEW_FILES = []

def edit(path, old, new, label):
    EDITS.append((path, old, new, label))

edit(STORAGE + "/.ts",
 'async function storageCustodyV1(options:StorageCustodyOptionsV1):Promise<Readonly<{backing:NativeStorageBackingV1;transaction:',
 'async function storageCustodyV1(options:StorageCustodyOptionsV1):Promise<Readonly<{backing:NativeStorageBackingV1;read:<T>(control:ResourceLeaseControl,run:(state:NativeStorageStateV1)=>T|Promise<T>)=>Promise<T>;transaction:',
 "A1 custody read type")

edit(STORAGE + "/.ts",
 ' return{backing,transaction:async<T>',
 ' const receipts=createJournalReceiptV1<NativeStorageStateV1>(journal);\n'
 ' return{backing,read:async<T>(control:ResourceLeaseControl,run:(state:NativeStorageStateV1)=>T|Promise<T>)=>{checkResourceLeaseControl(control);const revision=receipts.revision(),recalled=revision===undefined?undefined:receipts.recall(revision);\n'
 '  if(recalled!==undefined){await port.recheck(backing,control);checkResourceLeaseControl(control);return run(recalled);}\n'
 '  return port.withJournal(control,async(control:ResourceLeaseControl)=>{checkResourceLeaseControl(control);await port.recheck(backing,control);checkResourceLeaseControl(control);\n'
 '   const lease=await acquireQueuedResourceLease({directory:leaseDirectory,resource:"native-storage:"+backing.identity,mode:"shared",sharedCohort:true,owner:randomBytes(16).toString("base64url"),signal:control.signal,control});\n'
 '   try{const before=receipts.revision(),{state}=await readCleanupJournal(root,capacityBytes,()=>{checkResourceLeaseControl(control);},()=>control.remainingMilliseconds()??Number.MAX_SAFE_INTEGER),after=receipts.revision(),value=await run(state);if(before!==undefined&&before===after)receipts.remember(before,state);checkResourceLeaseControl(control);return value;}finally{lease.release();}\n'
 '  });},transaction:async<T>',
 "A2 custody read implementation")

edit(STORAGE + "/.ts",
 'const namespaceProtocol=Object.freeze({...schema.definitions.NamespaceProtocol.const});',
 'import{createJournalReceiptV1}from"./@JOURNAL@/@REVISION@/@TS@";\nconst namespaceProtocol=Object.freeze({...schema.definitions.NamespaceProtocol.const});',
 "A0 journal receipt import")

edit(STORAGE + "/.ts",
 'await port.measure?.(backing,{mode:"recheck",requestBytes:0},actual);await custody.transaction(actual,state=>{',
 'await port.measure?.(backing,{mode:"recheck",requestBytes:0},actual);await custody.read(actual,state=>{',
 "A3 recheck reads")

edit(STORAGE + "/schema/.json",
 '"informationBackoffMilliseconds": 75,',
 '"informationBackoffMilliseconds": 75,\n        "recheckWindowMilliseconds": 500,\n        "recheckWindowMaximumMilliseconds": 5000,',
 "B0 image protocol recheck window")

ROSTER = STORAGE + "/image@1f34e/roster=1f4e1"
NEW_FILES.extend([
 (ROSTER + "/.ts=1f7e6", "roster.ts"),
 (ROSTER + "/tests=1f9ea/.ts=1f7e6", "roster.test.ts"),
 (ROSTER + "/tests=1f9ea/.json=1f523", "roster.fixture.json"),
])

COMMAND = STORAGE + "/cleanup/command/.ts"
edit(COMMAND,
 'import{nativeImageInformationV1,observeNativeImageInventoryV1,type NativeImageInventoryPolicyV1}from"../../@IMAGE@/@INFORMATION@/@TS@";',
 'import{nativeImageInformationV1,observeNativeImageInventoryV1,type NativeImageInventoryPolicyV1}from"../../@IMAGE@/@INFORMATION@/@TS@";\n'
 'import{nativeImageRosterV1,observeSharedNativeImageInventoryV1,nativeRecheckWindowV1,createNativePhysicalGateV1,rememberSettledNativeRosterV1,settledNativeRosterV1}from"../../@IMAGE@/@ROSTER@/@TS@";',
 "B1 command roster import")

edit(COMMAND,
 'const matching=await observeNativeImageInventoryV1(()=>captureOwnedCommandV1("/usr/bin/hdiutil",["info","-plist"],directory,{control,output:{',
 'const matching=await observeSharedNativeImageInventoryV1(()=>captureOwnedCommandV1("/usr/bin/hdiutil",["info","-plist"],directory,{control,output:{',
 "B2 inspect shares one capture")

edit(COMMAND,
 'observe=()=>observeNativeImageInventoryV1(()=>{verifyPersistentFacts(receiver);return captureOwnedCommandV1("/usr/bin/hdiutil",["info","-plist"],receiver.options.directory,{control,output:{maximumBytes:limits.maximumOutputBytes,maximumLines:limits.maximumOutputLines}});},information=>{const images=information.images;if(!Array.isArray(images)||images.length>limits.maximumObservedImages)throw Error("Complete bounded persistent parent kernel inventory required");',
 'observe=(after?:number)=>nativeImageRosterV1.observe({capture:()=>{verifyPersistentFacts(receiver);return captureOwnedCommandV1("/usr/bin/hdiutil",["info","-plist"],receiver.options.directory,{control,output:{maximumBytes:limits.maximumOutputBytes,maximumLines:limits.maximumOutputLines}});},policy:attempts,maximumAgeMilliseconds:nativeRecheckWindowV1(),...(after===undefined?{}:{after}),accept:information=>{const images=information.images;if(!Array.isArray(images)||images.length>limits.maximumObservedImages)throw Error("Complete bounded persistent parent kernel inventory required");',
 "B3 children roster observe")

edit(COMMAND,
 'return rows.sort((a,b)=>a.root.length-b.root.length||a.root.localeCompare(b.root));},attempts).then(rows=>{verifyPersistentFacts(receiver);return rows;}),settled=async()=>{let previous=await observe();for(let attempt=1;attempt<attempts.attempts;attempt++){const next=await observe();if(JSON.stringify(next)===JSON.stringify(previous))return next;previous=next;}throw Error("Original persistent descendant kernel roster changed");};',
 'return rows.sort((a,b)=>a.root.length-b.root.length||a.root.localeCompare(b.root));}}).then(row=>{verifyPersistentFacts(receiver);return row;}),settled=async()=>{const prior=settledNativeRosterV1(receiver);let previous=await observe();if(prior!==undefined&&JSON.stringify(previous.value)===prior)return previous.value;for(let attempt=1;attempt<attempts.attempts;attempt++){const next=await observe(previous.serial);if(JSON.stringify(next.value)===JSON.stringify(previous.value)){rememberSettledNativeRosterV1(receiver,JSON.stringify(next.value));return next.value;}previous=next;}throw Error("Original persistent descendant kernel roster changed");};',
 "B4 children settled against prior")

edit(COMMAND,
 'physical=(control:ResourceLeaseControl)=>mappingRead(control,async control=>{checkResourceLeaseControl(control);verifyPersistentFacts(receiver);await verifyPersistentManagedChildrenV1(receiver,control);verifyPersistentFacts(receiver);}),recheck=()=>operation(x.control,physical),',
 'strict=(control:ResourceLeaseControl)=>mappingRead(control,async control=>{checkResourceLeaseControl(control);verifyPersistentFacts(receiver);await verifyPersistentManagedChildrenV1(receiver,control);verifyPersistentFacts(receiver);}),gate=createNativePhysicalGateV1(),physical=(control:ResourceLeaseControl)=>gate.run(nativeRecheckWindowV1(),()=>{checkResourceLeaseControl(control);verifyPersistentFacts(receiver);},()=>strict(control)),recheck=()=>operation(x.control,physical),',
 "B5 windowed physical gate")

edit(COMMAND,
 'await physical(actual);await syncNativeDirectoryMetadataV1(root,path,actual);await physical(actual);}),retire:',
 'await strict(actual);await syncNativeDirectoryMetadataV1(root,path,actual);await strict(actual);}),retire:',
 "B6 metadata sync strict")
edit(COMMAND,
 'async selected=>{await physical(selected);const custody=Object.freeze({kind:"kernel-ex"',
 'async selected=>{await strict(selected);const custody=Object.freeze({kind:"kernel-ex"',
 "B7 maintenance entry strict")
edit(COMMAND,
 'if(!receiver.retirement?.durable)await physical(selected);return{acquired:true as const,value};',
 'if(!receiver.retirement?.durable)await strict(selected);return{acquired:true as const,value};',
 "B8 maintenance exit strict")
edit(COMMAND,
 'if(!receiver.retirement?.durable)await physical(actual);return result;',
 'if(!receiver.retirement?.durable)await strict(actual);return result;',
 "B9 maintenance settle strict")

JOURNAL = STORAGE + "/journal@1f4dc"
NEW_FILES.extend([
 (JOURNAL + "/revision=1f516/.ts=1f7e6", "revision.ts"),
 (JOURNAL + "/revision=1f516/tests=1f9ea/.ts=1f7e6", "revision.test.ts"),
 (JOURNAL + "/revision=1f516/tests=1f9ea/.json=1f523", "revision.fixture.json"),
])


CARGO_DEPS = "framework/products@1f6cd/repo/modules/library/process/cargo/dependencies"
READY_STORAGE = CARGO_DEPS + "/ready/storage/.ts"
edit(READY_STORAGE,
 'const protocol=schema.definitions.Protocol.const,hash=',
 'import {lstatSync,readdirSync} from "node:fs";\nimport {captureFetchedV1,fetchedHoldsV1,readFetchedReceiptV1,fetchedEntryNameV1,type FetchedPortV1,type FetchedReceiptV1} from "../../@FETCHED@/@TS@";\nconst protocol=schema.definitions.Protocol.const,hash=',
 "C1 fetched receipt import")

edit(READY_STORAGE,
 '/** 🟢️ Reuses only EX-proved first-party dependency readiness under actual retained registry SH. */',
 '/** 🧾️ Observes the registry directories of the image-resident Cargo home and the repository Source without granting any write. */\n'
 'function fetchedPortV1(options:ReadyOptions):FetchedPortV1{const root=options.storage.backing.root;return{read:async path=>(await sourceBytes(options,path,protocol.maximumFiles))??undefined,stamp:path=>{try{requireNativeStoragePathV1(root,path);const row=lstatSync(path,{bigint:true});return row.isDirectory()&&!row.isSymbolicLink()?{dev:String(row.dev),ino:String(row.ino),mtimeNs:String(row.mtimeNs)}:undefined;}catch{return undefined;}},list:path=>{try{requireNativeStoragePathV1(root,path);return readdirSync(path,{withFileTypes:true}).filter(entry=>entry.isDirectory()).map(entry=>entry.name);}catch{return undefined;}}};}\n'
 '/** 🟢️ Reuses only EX-proved first-party dependency readiness under actual retained registry SH. */',
 "C2 fetched port")

edit(READY_STORAGE,
 'const consume=async()=>{const reader=await acquireQueuedResourceLease({...lease,owner:randomBytes(8).toString("base64url"),mode:"shared",sharedCohort:true});try{await storage.recheck();const receipt=identity&&file?await readReady(options,file,identity):undefined;if(receipt){const value=await withRegistryReadScopeV1(options,body);if(!await readyInputs(options,receipt))throw Error("Original Cargo ready inputs changed during consumption");await storage.recheck();return{hit:true as const,value};}return{hit:false as const};}finally{reader.release();}};',
 'const fetchedFile=identity?requireNativeStoragePathV1(storage.backing.root,join(namespace,fetchedEntryNameV1(Number.parseInt(identity.slice(0,8),16)%protocol.slots))):undefined,fetchedPort=fetchedPortV1(options);\n'
 ' const readFetched=async():Promise<FetchedReceiptV1|undefined>=>{if(!identity||!fetchedFile)return;const bytes=await readPhysicalCargoSourceV1(fetchedFile,{expectedBytes:null,version:1,root:storage.backing.root,maximumBytes:protocol.maximumReceiptBytes,remainingFiles:1},options.control);if(!bytes)return;const receipt=readFetchedReceiptV1(Buffer.from(bytes).toString("utf8"),identity);return receipt&&await fetchedHoldsV1(receipt,options.home,fetchedPort)?receipt:undefined;};\n'
 ' const consume=async()=>{const reader=await acquireQueuedResourceLease({...lease,owner:randomBytes(8).toString("base64url"),mode:"shared",sharedCohort:true});try{await storage.recheck();const receipt=identity&&file?await readReady(options,file,identity):undefined,fetched=receipt?undefined:await readFetched();if(receipt||fetched){const value=await withRegistryReadScopeV1(options,body);if(receipt&&!await readyInputs(options,receipt))throw Error("Original Cargo ready inputs changed during consumption");if(fetched&&!await fetchedHoldsV1(fetched,options.home,fetchedPort))throw Error("Original Cargo fetched registry changed during consumption");await storage.recheck();return{hit:true as const,value};}return{hit:false as const};}finally{reader.release();}};',
 "C3 shared hit on fetched receipt")

edit(READY_STORAGE,
 'let claimed:ReadyReceipt|undefined;const writer=',
 'let claimed:ReadyReceipt|undefined,fetchedClaim:FetchedReceiptV1|undefined;const writer=',
 "C4 fetched claim")

edit(READY_STORAGE,
 'if(!claimed){await fetch();if(identity&&file){',
 'if(!claimed){fetchedClaim=await readFetched();if(!fetchedClaim)await fetch();if(!fetchedClaim&&identity&&file){',
 "C5 skip fetch under a valid fetched receipt")

edit(READY_STORAGE,
 'if(!claimed)throw Error("Original Cargo ready publication changed");}}}\n }finally{writer.release();}',
 'if(!claimed)throw Error("Original Cargo ready publication changed");}}\n  if(!claimed&&!fetchedClaim&&identity&&fetchedFile){let manifests:string[]|undefined;try{manifests=options.requests.map(row=>ownedSource(options.workspace,row.manifest));}catch(error){if(!(error instanceof UnsupportedReadySource))throw error;}const captured=manifests&&await captureFetchedV1({request:identity,manifests,home:options.home},fetchedPort);if(captured){await publishReady(options,namespace,fetchedFile,JSON.stringify(captured)+"\\n");fetchedClaim=await readFetched();}}}\n }finally{writer.release();}',
 "C6 publish fetched receipt")

edit(READY_STORAGE,
 'if(claimed&&!await readyInputs(options,claimed))throw Error("Original Cargo ready inputs changed before consumption");const value=await withRegistryReadScopeV1(options,body);if(claimed&&!await readyInputs(options,claimed))throw Error("Original Cargo ready inputs changed during consumption");await storage.recheck();return value;',
 'if(claimed&&!await readyInputs(options,claimed))throw Error("Original Cargo ready inputs changed before consumption");if(fetchedClaim&&!await fetchedHoldsV1(fetchedClaim,options.home,fetchedPort))throw Error("Original Cargo fetched registry changed before consumption");const value=await withRegistryReadScopeV1(options,body);if(claimed&&!await readyInputs(options,claimed))throw Error("Original Cargo ready inputs changed during consumption");if(fetchedClaim&&!await fetchedHoldsV1(fetchedClaim,options.home,fetchedPort))throw Error("Original Cargo fetched registry changed during consumption");await storage.recheck();return value;',
 "C7 fetched claim held through consumption")

FETCHED = CARGO_DEPS + "/fetched=1f9fe"
NEW_FILES.extend([
 (FETCHED + "/.ts=1f7e6", "fetched.ts"),
 (FETCHED + "/schema=1f9ec/.json=1f523", "fetched.schema.json"),
 (FETCHED + "/tests=1f9ea/.ts=1f7e6", "fetched.test.ts"),
 (FETCHED + "/tests=1f9ea/.json=1f523", "fetched.fixture.json"),
])

ADMISSION = "framework/modules/process/artifacts/native-build/admission/.ts"
edit(ADMISSION,
 'import { fileURLToPath } from "node:url";',
 'import { fileURLToPath } from "node:url";\nimport { captureSchedulerReceiptV1, readSchedulerReceiptV1, schedulerIdentityV1, schedulerReceiptFileV1, schedulerReceiptHoldsV1, writeSchedulerReceiptV1 } from "./@SCHEDULER@/@TS@";',
 "D1 scheduler receipt import")
edit(ADMISSION,
 'examples?: boolean; control: CompilerAdmissionControl;',
 'examples?: boolean; toolchain?: string; control: CompilerAdmissionControl;',
 "D2 toolchain option")
edit(ADMISSION,
 'const graph=await captureCargoCompilationGraphV1({workspaceRoot:options.cwd,cwd:options.cwd,program:"cargo",prefix:[],args:["build","--locked","--release","--manifest-path",manifest,"--bins"',
 'const receiptFile=join(options.targetDirectory,schedulerReceiptFileV1()),identity=options.toolchain&&!options.examples?schedulerIdentityV1(options.toolchain,options.environment):undefined,held=identity?schedulerReceiptHoldsV1(readSchedulerReceiptV1(receiptFile),identity,manifest):undefined;if(held)return held;\n  const graph=await captureCargoCompilationGraphV1({workspaceRoot:options.cwd,cwd:options.cwd,program:"cargo",prefix:[],args:["build","--locked","--release","--manifest-path",manifest,"--bins"',
 "D3 receipt hit skips planning, admission and cargo")
edit(ADMISSION,
 '    return executables;\n  } finally { if (deadline) clearTimeout(deadline);',
 '    if (identity) { const receipt = captureSchedulerReceiptV1(identity, manifest, executables); if (receipt) writeSchedulerReceiptV1(receiptFile, receipt); }\n    return executables;\n  } finally { if (deadline) clearTimeout(deadline);',
 "D4 receipt publish")
edit("framework/products@1f6cd/repo/modules/library/process/cargo/launcher/.ts",
 'buildDirectory:store.buildDirectory,environment:store.environment,control:cpuControl,storage:store.storage,sourceBoundary:store.sourceBoundary}',
 'buildDirectory:store.buildDirectory,environment:store.environment,toolchain:store.toolchain.digest,control:cpuControl,storage:store.storage,sourceBoundary:store.sourceBoundary}',
 "D5 launcher passes toolchain digest")
SCHED = "framework/modules/process/artifacts/native-build/admission/scheduler=1f516"
NEW_FILES.extend([
 (SCHED + "/.ts=1f7e6", "scheduler.ts"),
 (SCHED + "/schema=1f9ec/.json=1f523", "scheduler.schema.json"),
 (SCHED + "/tests=1f9ea/.ts=1f7e6", "scheduler.test.ts"),
 (SCHED + "/tests=1f9ea/.json=1f523", "scheduler.fixture.json"),
])

COMPILER = "framework/products@1f6cd/repo/modules/library/caching/storage/enforcement/compiler/.ts"
edit(COMPILER,
 'import {randomBytes} from "node:crypto";',
 'import {randomBytes} from "node:crypto";\nimport {AsyncLocalStorage} from "node:async_hooks";',
 "E1 async hooks import")
edit(COMPILER,
 'body:(store:RepositoryCompilerStoreV1,control:ResourceLeaseControl)=>Promise<T>):Promise<T>{checkResourceLeaseControl(control);return receiveScriptProcessInvocation(',
 'body:(store:RepositoryCompilerStoreV1,control:ResourceLeaseControl)=>Promise<T>):Promise<T>{checkResourceLeaseControl(control);\n'
 ' const retained=retainedCompilerStores.getStore()?.get(retainedCompilerKeyV1(workspace,mode));\n'
 ' if(retained?.live){let reusable=true;try{requireIssuedResourceControlInvocationV1(control,retained.invocation);}catch{reusable=false;}\n'
 '  if(reusable)return withRequestedIssuedResourceControlV1(retained.control,Math.max(0,Math.floor(control.remainingMilliseconds()??0)),async actual=>{await retained.store.storage.recheck();const value=await body(retained.store,actual);await retained.store.storage.recheck();return value;},control.signal);}\n'
 ' return receiveScriptProcessInvocation(',
 "E2 reuse a retained compiler store inside a custody session")
edit(COMPILER,
 'type CompilerStoreOwner={',
 'type RetainedCompilerStoreV1={readonly store:RepositoryCompilerStoreV1;readonly control:ResourceLeaseControl;readonly invocation:ScriptInvocation;live:boolean};\n'
 'const retainedCompilerStores=new AsyncLocalStorage<ReadonlyMap<string,RetainedCompilerStoreV1>>(),retainedCompilerKeyV1=(workspace:string,mode:BuildMode)=>workspace+"\\0"+mode;\n'
 '/** 🧵️ Enters the compiler store once and lets every `withRepositoryCompilerStoreV1` call of the same invocation, workspace and mode inside `body` reuse it with its own deadline and cancellation, until `body` settles. */\n'
 'export function withRetainedRepositoryCompilerStoreV1<T>(workspace:string,mode:BuildMode,control:ResourceLeaseControl,environment:Readonly<Record<string,string|undefined>>,body:(control:ResourceLeaseControl)=>Promise<T>):Promise<T>{\n'
 ' return withRepositoryCompilerStoreV1(workspace,mode,control,environment,(store,actual)=>{const owner=compilerStoreOwners.get(store);if(!owner?.live)throw Error("Actual retained compiler constructor required");const entry:RetainedCompilerStoreV1={store,control:actual,invocation:owner.invocation,live:true},map=new Map(retainedCompilerStores.getStore());map.set(retainedCompilerKeyV1(workspace,mode),entry);return retainedCompilerStores.run(map,async()=>{try{return await body(actual);}finally{entry.live=false;}});});\n'
 '}\n'
 'type CompilerStoreOwner={',
 "E3 retained compiler store scope")

SESSION = "framework/products@1f6cd/repo/modules/library/caching/storage/enforcement/session=1f9f5"
NEW_FILES.extend([
 (SESSION + "/.ts=1f7e6", "session.ts"),
 (SESSION + "/pool=1f3ca/.ts=1f7e6", "pool.ts"),
 (SESSION + "/pool=1f3ca/tests=1f9ea/.ts=1f7e6", "pool.test.ts"),
 (SESSION + "/pool=1f3ca/tests=1f9ea/.json=1f523", "pool.fixture.json"),
 ("framework/products@1f6cd/repo/modules/library/process/cargo/launcher/session=1f9f5/.ts=1f7e6", "cargo-session.ts"),
])

OPENING = "framework/products@1f6cd/repo/modules/library/caching/storage/enforcement/budget/opening"
edit(OPENING + "/parents/schema/.json",
 '"maximumPathCharacters": 256',
 '"maximumPathCharacters": 256,\n        "leaseRetirementIntervalMilliseconds": 15000,\n        "leaseRetirementStamp": ".retired"',
 "F0 lease retirement protocol")
edit(OPENING + "/.ts",
 'import {lstatSync,mkdirSync} from "node:fs";',
 'import {lstatSync,mkdirSync,writeFileSync} from "node:fs";',
 "F1 fs imports")
edit(OPENING + "/.ts",
 '/** 🧽️ Opens the lease image and opportunistically retires idle leases only when no other invocation holds it, because skipped retirement loses nothing. */',
 '/** 🧽️ A retirement stamp younger than the declared interval means idle leases were swept recently; a missing, linked or future stamp never is. */\n'
 'export function leaseRetirementFreshV1(stamp:string,now:number=Date.now()):boolean{try{const row=lstatSync(stamp);const age=now-Math.floor(row.mtimeMs);return row.isFile()&&!row.isSymbolicLink()&&age>=0&&age<=parentSchema.definitions.Protocol.const.leaseRetirementIntervalMilliseconds;}catch{return false;}}\n'
 '/** 🧽️ Records that idle leases were swept now; a failed write only costs the next opener another sweep. */\n'
 'export function touchLeaseRetirementV1(stamp:string):void{try{writeFileSync(stamp,"");}catch{}}\n'
 '/** 🧽️ Opens the lease image and opportunistically retires idle leases only when no other invocation holds it, because skipped retirement loses nothing. */',
 "F2 stamp helpers")
edit(OPENING + "/.ts",
 'requireNativeStoragePathV1(image.root,directory);await image.withExclusiveMaintenance(',
 'requireNativeStoragePathV1(image.root,directory);const stamp=requireNativeStoragePathV1(image.root,join(directory,parentSchema.definitions.Protocol.const.leaseRetirementStamp));if(leaseRetirementFreshV1(stamp))return image;await image.withExclusiveMaintenance(',
 "F3 fresh stamp skips the sweep, its second open and its close")
edit(OPENING + "/.ts",
 '});await image.close();image=await open();await image.recheck();return image;',
 '});await image.close();image=await open();await image.recheck();touchLeaseRetirementV1(stamp);return image;',
 "F4 stamp after the sweep")
NEW_FILES.extend([
 (OPENING + "/tests=1f9ea/stamp=1f9f9/.ts=1f7e6", "stamp.test.ts"),
 (OPENING + "/tests=1f9ea/stamp=1f9f9/.json=1f523", "stamp.fixture.json"),
])

LAUNCHER = "framework/products@1f6cd/repo/modules/library/process/cargo/launcher/.ts"
edit(LAUNCHER,
 'import {randomBytes} from "node:crypto";',
 'import {randomBytes} from "node:crypto";\nimport {withNativeRecheckWindowV1} from "../../../../../../../@ROSTERPATH@";',
 "G1 launcher window import")
edit(LAUNCHER,
 'requireIssuedResourceControlInvocationV1(phaseControl,invocation);return withIssuedResourceOperationV1(phaseControl,async()=>{request=Object.freeze(',
 'requireIssuedResourceControlInvocationV1(phaseControl,invocation);return withIssuedResourceOperationV1(phaseControl,()=>withNativeRecheckWindowV1(async()=>{request=Object.freeze(',
 "G2 open the recheck window")
edit(LAUNCHER,
 'withRepositoryCargoIsolatedStoreV1({storage:store.storage,base:dirname(store.buildDirectory),control},run);\n });\n });\n}',
 'withRepositoryCargoIsolatedStoreV1({storage:store.storage,base:dirname(store.buildDirectory),control},run);\n });\n }));\n}',
 "G3 close the recheck window")
