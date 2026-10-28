"""📌 [pinpub] rewrites the stale 📌️pinned-publication laws onto today's pin contract; usage: edit_pinned.py <root>  (edits <root>'s test file in place, anchored on the import lines and test names; exit 1 = anchor lost)."""
import sys
ROOT = sys.argv[1]
REL = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/📌️pinned-publication/🟦️.ts"
path = f"{ROOT}/{REL}"
s = open(path, encoding="utf8").read()

def swap(old, new):
    global s
    if s.count(old) != 1: sys.exit(f"anchor lost ({s.count(old)}): {old[:90]}")
    s = s.replace(old, new)

swap('import {createHash} from "node:crypto";\n', 'import {createHash,randomUUID} from "node:crypto";\nimport {gunzipSync} from "node:zlib";\n')
swap('import {buildGraphPathV1,loadBuildGraphV1,readBuildGraphV1} from "../../../🕸️graph/🟦️.ts";\n',
     'import {buildGraphPathV1,generationHostV1,loadBuildGraphV1,readBuildGraphV1} from "../../../🕸️graph/🟦️.ts";\n'
     'import * as leanInventory from "../../../📥️inputs/🪶️lean-inventory/🟨️.mjs";\n'
     'import {encodeGraphRecordsV1} from "../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📇️records/🟨️.mjs";\n'
     'import artifactPolicy from "../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/🔣️policy.json";\n')
swap('const require=createRequire(import.meta.url),Ajv=require("ajv/dist/2020").default;\n',
     'const require=createRequire(import.meta.url),Ajv=require("ajv/dist/2020").default,sha256=(bytes:string|Uint8Array)=>createHash("sha256").update(bytes).digest("hex");\n'
     '/** ⏳️ The Original budget of a law that opens native images: every opening accepts a 10 s cleanup reserve before effects (`reserveMilliseconds` of the native storage schema), so the 20 s default leaves a loaded host no time for boot recovery, publication and the read. */\n'
     'const pinnedBudgetMillisecondsV1=120000,pinnedTimeoutMillisecondsV1=300000,withPinnedInvocationV1=(body:(invocation:any)=>Promise<void>)=>withGraphTestInvocationV1(body,pinnedBudgetMillisecondsV1);\n'
     '/** 🧯️ A refusal inside an issued scope also fails that scope (consumer frames are fail-closed: `enrollConsumer` records the failure, `settleConsumers` rethrows it), so a refused read is asserted inside and around its own Original. */\n'
     'const refusedScopeV1=(message:string|RegExp,body:(original:any)=>Promise<void>)=>expect(withPinnedInvocationV1(body)).rejects.toThrow(message);\n'
     '/** 📌️ Hands the body a binder of the process pin and restores `SEMIO_BUILD_GRAPH_PATH`/`SEMIO_BUILD_GRAPH_DIGEST` afterwards. */\n'
     'async function withPinEnvironmentV1(body:(bind:(pin:{path:string;digest:string})=>void)=>Promise<void>):Promise<void>{const previous={path:process.env.SEMIO_BUILD_GRAPH_PATH,digest:process.env.SEMIO_BUILD_GRAPH_DIGEST};try{await body(pin=>{process.env.SEMIO_BUILD_GRAPH_PATH=pin.path;process.env.SEMIO_BUILD_GRAPH_DIGEST=pin.digest;});}finally{if(previous.path===undefined)delete process.env.SEMIO_BUILD_GRAPH_PATH;else process.env.SEMIO_BUILD_GRAPH_PATH=previous.path;if(previous.digest===undefined)delete process.env.SEMIO_BUILD_GRAPH_DIGEST;else process.env.SEMIO_BUILD_GRAPH_DIGEST=previous.digest;}}\n'
     '/** 🔎️ Independent oracle of a retained snapshot: parses the record manifest, checks every gzip record against its name and joins the graph with `node:zlib`, without the production decoder. */\n'
     'async function decodeSnapshotOracleV1(path:string):Promise<unknown>{const manifest=JSON.parse(await readFile(path,"utf8")),record=async(digest:string)=>{const packed=await readFile(join(dirname(path),digest+".gz"));expect(sha256(packed)).toBe(digest);return JSON.parse(gunzipSync(packed).toString("utf8"));},projects:Record<string,unknown>={},dependencies:unknown[]=[];for(const[name,digest]of manifest.projects)projects[name]=await record(digest);for(const digest of manifest.dependencies)dependencies.push(...await record(digest));let externalNodes:Record<string,unknown>|undefined;if(manifest.externalNodes){externalNodes={};for(const digest of manifest.externalNodes)Object.assign(externalNodes,await record(digest));}return {...manifest.graph,projects,dependencies,...(externalNodes?{externalNodes}:{})};}\n')

blocks = {}
def block(name):
    start = s.find(f'test("{name}"')
    if start < 0 or s.count(f'test("{name}"') != 1: sys.exit("test anchor lost: " + name)
    end = s.find("\ntest(", start + 1)
    return start, (len(s) if end < 0 else end + 1)
def replace_test(name, text):
    global s
    a, b = block(name)
    s = s[:a] + text.rstrip("\n") + "\n" + s[b:]

replace_test("nested consumers reuse the exact pinned authority and reject a changed publication", r'''test("nested consumers reuse the exact pinned authority and refuse a changed publication",async()=>{
 const root=process.cwd(),directory=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"pin-")),portable=graph(),changed=JSON.stringify({...portable,version:2}),identity="Original retained Graph snapshot identity required",read=(original:any,phase:any)=>loadBuildGraphV1(root,original,phase,{selection:{kind:"all"},reuse:true}),publish=(original:any,body:(pin:{path:string;digest:string},phase:any)=>Promise<void>)=>withRepositoryBuildGraphArtifactV1(root,portable,original,createIssuedResourceControlV1(original,"graph:fixture",0),body);
 try{await writeFile(join(directory,"changed.json"),changed);await withPinEnvironmentV1(async bind=>{
  await withPinnedInvocationV1(original=>publish(original,async(pin,phase)=>{bind(pin);expect(await read(original,phase)).toEqual(portable);expect(sha256(await readFile(pin.path))).toBe(pin.digest);expect(pin.path).toBe(join(dirname(pin.path),pin.digest+".json"));expect(await decodeSnapshotOracleV1(pin.path)).toEqual(portable);}));
  for(const moved of [(_pin:{path:string;digest:string})=>({path:join(directory,"changed.json"),digest:sha256(changed)}),(pin:{path:string;digest:string})=>({path:pin.path,digest:sha256(changed)}),(pin:{path:string;digest:string})=>({path:join(dirname(pin.path),sha256(changed)+".json"),digest:pin.digest})])
   await refusedScopeV1(identity,original=>publish(original,async(pin,phase)=>{bind(moved(pin));await expect(read(original,phase)).rejects.toThrow(identity);}));
 });}finally{await rm(directory,{recursive:true,force:true});}
},pinnedTimeoutMillisecondsV1);
''')

name6 = "each pinned publication receives a distinct Nx cache and never forces an older graph"
a, b = block(name6)
t6 = s[a:b]
for old, new in (("await withGraphTestInvocationV1(original=>withRepositoryNxStorageV1(", "await withPinnedInvocationV1(original=>withRepositoryNxStorageV1("), ("},25000);", "},pinnedTimeoutMillisecondsV1);")):
    if t6.count(old) != 1: sys.exit("t6 anchor lost: " + old)
    t6 = t6.replace(old, new)
s = s[:a] + t6 + s[b:]

replace_test("public pinned graph reuse preserves ordinary nested metadata and execution arrays", r'''test("public pinned graph reuse preserves ordinary nested metadata and execution arrays",async()=>{
 const root=process.cwd(),portable=graph(),patch=require("fast-json-patch");
 expect(patch.applyPatch(portable,fixture.compactExpansion.ordinaryPatch,true).newDocument).toBe(portable);
 await withPinEnvironmentV1(bind=>withPinnedInvocationV1(original=>withRepositoryBuildGraphArtifactV1(root,portable,original,createIssuedResourceControlV1(original,"graph:fixture",0),async(pin,phase)=>{bind(pin);const result=await loadBuildGraphV1(root,original,phase,{selection:{kind:"all"},reuse:true});expect(new Ajv({strict:false}).compile(schema)(result)).toBe(true);expect(result).toEqual(portable);expect(await decodeSnapshotOracleV1(pin.path)).toEqual(result);expect((await encodeGraphRecordsV1(result,{policy:artifactPolicy.records as never,maximumBytes:artifactPolicy.maximumSnapshotBytes})).digest).toBe(pin.digest);})));
},pinnedTimeoutMillisecondsV1);
''')

replace_test("public pinned graph expands only compact tokens with shared bounded content reads", r'''test("public pinned graph reuse delivers compact tokens verbatim and the generation host expands each closure with one verified read",async()=>{
 const root=process.cwd(),rows=fixture.compactExpansion,patch=require("fast-json-patch"),registry=readBuildOwnerRegistryV1(fixture.ownerAdmission.registry),scope=randomUUID().replaceAll("-",""),relocate=(file:string)=>scope+"/"+file,tokens:Record<string,string>={},written:string[]=[];
 const contracts:Record<string,any>=Object.fromEntries(Object.entries(rows.contracts).map(([kind,contract]:[string,any])=>[kind,kind==="source"?{...contract,files:contract.files.map(relocate)}:{...contract,manifest:relocate(contract.manifest)}]));
 for(const[kind,contract]of Object.entries(contracts)){const contractSchema=kind==="source"?require("../../../📥️inputs/🧮️digest/🔣️schema.json"):require("../../../📥️inputs/🔒️lock/🔣️schema.json").$defs.CargoLockInventory;expect(new Ajv({strict:false}).compile(contractSchema)(contract)).toBe(true);tokens[kind]=(kind==="source"?"SEMIO_SOURCE_CLOSURE_":"SEMIO_CARGO_LOCK_")+sha256(JSON.stringify(contract));}
 const located=(value:unknown)=>{let text=JSON.stringify(value).replaceAll('"$source"',JSON.stringify(tokens.source)).replaceAll('"$lock"',JSON.stringify(tokens.lock));for(const file of [...rows.contracts.source.files,rows.contracts.lock.manifest])text=text.replaceAll(JSON.stringify("{workspaceRoot}/"+file),JSON.stringify("{workspaceRoot}/"+relocate(file)));return JSON.parse(text);};
 const portable=graph(),expected=graph(),batch=graph(),batchContracts=Array.from({length:rows.batch.count},(_,index)=>({version:1,files:[relocate(rows.batch.filePrefix+index+".ts")]})),corrupt={version:1,files:[relocate("app/corrupt.ts")]};
 expect(patch.applyPatch(portable,located(rows.inputPatch),true).newDocument).toBe(portable);expect(patch.applyPatch(expected,located(rows.expectedPatch),true).newDocument).toBe(expected);expect(new Ajv({strict:false}).compile(schema)(expected)).toBe(true);batch.projects.app!.targets.build!.inputs=batchContracts.map(contract=>({env:"SEMIO_SOURCE_CLOSURE_"+sha256(JSON.stringify(contract))}));
 const reads=spyOn(leanInventory,"readInventoryRecordLeanV1"),closureReads=(storeRoot:string)=>reads.mock.calls.filter(([directory])=>String(directory).startsWith(storeRoot)).length;
 const write=async(storeRoot:string,kind:string,contract:unknown,bytes=JSON.stringify(contract))=>{const directory=join(storeRoot,kind==="lock"?"build-lock-closures":"build-input-closures"),path=join(directory,sha256(JSON.stringify(contract))+".json");await mkdir(directory,{recursive:true});written.push(path);await writeFile(path,bytes);};
 try{await withPinEnvironmentV1(async bind=>{
  await withPinnedInvocationV1(original=>withRepositoryBuildGraphArtifactV1(root,portable,original,createIssuedResourceControlV1(original,"graph:fixture",0),async(pin,phase)=>{
   const storeRoot=dirname(dirname(pin.path)),host=generationHostV1(root,original,phase,registry);bind(pin);reads.mockClear();
   const delivered=await loadBuildGraphV1(root,original,phase,{selection:{kind:"all"},reuse:true});expect(delivered).toEqual(portable);expect(delivered).not.toEqual(expected);expect(await decodeSnapshotOracleV1(pin.path)).toEqual(delivered);expect(closureReads(storeRoot)).toBe(0);
   for(const[kind,contract]of Object.entries(contracts))await write(storeRoot,kind,contract);
   expect(await host.expand(structuredClone(portable.projects),phase)).toEqual(expected.projects);expect(closureReads(storeRoot)).toBe(Object.keys(contracts).length);
   for(const contract of batchContracts)await write(storeRoot,"source",contract);reads.mockClear();
   expect((await host.expand(structuredClone(batch.projects),phase) as any).app.targets.build.inputs).toEqual(batchContracts.map(contract=>"{workspaceRoot}/"+contract.files[0]));expect(closureReads(storeRoot)).toBe(rows.batch.count);
  }));
  await refusedScopeV1(rows.digestError,original=>withRepositoryBuildGraphArtifactV1(root,portable,original,createIssuedResourceControlV1(original,"graph:fixture",0),async(pin,phase)=>{const projects=structuredClone(portable.projects);await write(dirname(dirname(pin.path)),"source",corrupt,"{}");(projects.app!.targets.build!.inputs as unknown[]).push({env:"SEMIO_SOURCE_CLOSURE_"+sha256(JSON.stringify(corrupt))});await expect(generationHostV1(root,original,phase,registry).expand(projects,phase)).rejects.toThrow(rows.digestError);}));
 });}finally{reads.mockRestore();await Promise.all(written.map(path=>rm(path,{force:true})));}
},pinnedTimeoutMillisecondsV1);
''')

name9 = "neutral graph reset selects owned publication files and preserves storage receipts and pinned facts"
a, b = block(name9)
t9 = s[a:b]
old9 = '''  const source=await readFile(new URL("../../../📜️script.ts",import.meta.url),"utf8");expect(source).toContain("invalidateBuildGraphV1(root,invocation,phaseControl)");expect(source).not.toContain('rm(join(root,".🧬semio/🦑️repo/⚡️cache/build")');\n'''
new9 = '''  const runtime=await readFile(new URL("../../../⌨️command/⚙️runtime/🟦️.ts",import.meta.url),"utf8"),backends=await import("../../../🔌️backends/🟦️.ts"),registryBase=new URL("../../../🔌️backends/",import.meta.url),raw='rm(join(root,".🧬semio/🦑️repo/⚡️cache/build")';expect(runtime).toContain("loadBuildBackendV1(selected.tool)).reset({root,invocation,phase:phaseControl,");expect(runtime).not.toContain(raw);
  for(const id of backends.buildBackendIdsV1()){const url=new URL(backends.buildBackendRowV1(id).module,registryBase),source=await readFile(url,"utf8"),start=source.indexOf("async reset("),end=source.indexOf("\\n async ",start+1),reset=source.slice(start,end<0?undefined:end),hops=await Promise.all([...reset.matchAll(/import\\("([^"]+)"\\)/gu)].map(match=>readFile(new URL(match[1]!,url),"utf8")));expect(start).toBeGreaterThan(-1);expect({id,invalidates:reset.includes("invalidateBuildGraphV1(")||hops.some(hop=>hop.includes("await invalidateBuildGraphV1(root,invocation,phaseControl"))}).toEqual({id,invalidates:true});expect(source).not.toContain(raw);}
'''
if t9.count(old9) != 1: sys.exit("t9 anchor lost")
s = s[:a] + t9.replace(old9, new9) + s[b:]

replace_test("public pinned graph reuse requires the exact closed all or task admission", r'''test("public pinned graph reuse requires the exact closed all or task admission",async()=>{
 const corpus=fixture.pinnedSelection,equal=nxOracleRequireV1("lodash/isEqual"),root=process.cwd(),refusal="Original selected build graph admission required";
 await withPinEnvironmentV1(async bind=>{for(const row of corpus.cases){const portable=graph();if(row.admitted)portable.admission={selection:row.admitted as any,registry:corpus.registry,closure:corpus.closure};else delete portable.admission;expect(equal(row.admitted,row.requested)).toBe(row.accepted);
  const scope=(original:any)=>withRepositoryBuildGraphArtifactV1(root,portable,original,createIssuedResourceControlV1(original,"graph:fixture",0),async(pin,phase)=>{bind(pin);const read=loadBuildGraphV1(root,original,phase,{selection:row.requested as any,reuse:true});if(row.accepted)expect(await read).toEqual(portable);else await expect(read).rejects.toThrow(refusal);});
  if(row.accepted)await withPinnedInvocationV1(scope);else await refusedScopeV1(refusal,scope);
 }});
},pinnedTimeoutMillisecondsV1);
''')

name11 = "nested graph handoff validates ancestor pins before fresh source admission under original control"
a, b = block(name11)
t11 = s[a:b]
edits11 = (
    ('const equal=nxOracleRequireV1("lodash/isEqual"),root=process.cwd(),directory=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"nh-")),previous={path:process.env.SEMIO_BUILD_GRAPH_PATH,digest:process.env.SEMIO_BUILD_GRAPH_DIGEST},validCaller=',
     'const equal=nxOracleRequireV1("lodash/isEqual"),root=process.cwd(),pins=new Map<string,{path:string;digest:string}>(),validCaller='),
    (' try{for(const protocol of',
     ' await withPinEnvironmentV1(async bind=>{\n await withPinnedInvocationV1(async original=>{for(const row of fixture.nestedGraphHandoff.cases){const key=JSON.stringify(row.pin);if(!pins.has(key))await withRepositoryBuildGraphArtifactV1(root,{...graph(),admission:{selection:row.pin as any,registry:"a".repeat(64),closure:"b".repeat(64)}},original,createIssuedResourceControlV1(original,"graph:fixture",0),async pin=>{pins.set(key,{path:pin.path,digest:pin.digest});});}});\n for(const protocol of'),
    (',portable={...graph(),admission:{selection:row.pin,registry:"a".repeat(64),closure:"b".repeat(64)}},source=JSON.stringify(portable),path=join(directory,tool+"-"+protocol.namespace+"-"+row.id+".json");expect(validCaller(authored)).toBe(true);await writeFile(path,"corrupt"in row?source+" ":source);process.env.SEMIO_BUILD_GRAPH_PATH=path;process.env.SEMIO_BUILD_GRAPH_DIGEST=createHash("sha256").update(source).digest("hex");',
     ',pin=pins.get(JSON.stringify(row.pin))!;expect(validCaller(authored)).toBe(true);bind("corrupt"in row?{path:pin.path,digest:sha256(pin.digest)}:pin);'),
    ('request=createNativeBuildCallerCapabilitiesV1(root,["run","app:test"],"nx",process.env),started=performance.now();', 'request=createNativeBuildCallerCapabilitiesV1(root,["run","app:test"],"nx",process.env);'),
    ('  await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"nested-graph-handoff-test"', '  let refusal:unknown;const outcome=await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"nested-graph-handoff-test"'),
    ('if("refused"in row)await expect(pending).rejects.toThrow();else expect(await pending).toBeDefined();', 'if("refused"in row){refusal=await pending.then(()=>undefined,(error:unknown)=>error);expect(refusal).toBeInstanceOf(Error);}else expect(await pending).toBeDefined();'),
    ('}finally{observer.mockRestore();}\n  });\n', '}finally{observer.mockRestore();}\n  }).then(()=>undefined,(error:unknown)=>error);expect(outcome===undefined||"refused"in row&&outcome===refusal).toBe(true);\n'),
    ('{version:1,owner:"nested-graph-handoff-test",maximumElapsedMilliseconds:20000}', '{version:1,owner:"nested-graph-handoff-test",maximumElapsedMilliseconds:pinnedBudgetMillisecondsV1}'),
    ('if(options.reuse){await new Promise(accept=>setTimeout(accept,expiry.progressDelayMilliseconds));progressFinished=true;return loadBuildGraphV1(workspace,invocation,actualPhase,options);}fresh++;return graph();});})).rejects.toThrow();expect(progressFinished).toBe(true);expect(fresh).toBe(0);expect(performance.now()-started).toBeGreaterThanOrEqual(expiry.maximumElapsedMilliseconds);',
     'if(options.reuse){await new Promise(accept=>setTimeout(accept,expiry.progressDelayMilliseconds));progressFinished=true;return loadBuildGraphV1(workspace,invocation,actualPhase,options);}fresh++;return graph();});})).rejects.toThrow(/^(?:Finite Original physical cleanup reserve required before effects|Original Script (?:process )?deadline (?:refused|exhausted))$/u);expect(progressFinished).toBe(false);expect(fresh).toBe(0);'),
    (' }finally{if(previous.path===undefined)delete process.env.SEMIO_BUILD_GRAPH_PATH;else process.env.SEMIO_BUILD_GRAPH_PATH=previous.path;if(previous.digest===undefined)delete process.env.SEMIO_BUILD_GRAPH_DIGEST;else process.env.SEMIO_BUILD_GRAPH_DIGEST=previous.digest;await rm(directory,{recursive:true,force:true});}\n});',
     ' });\n},pinnedTimeoutMillisecondsV1);'),
)
for old, new in edits11:
    if t11.count(old) != 1: sys.exit(f"t11 anchor lost ({t11.count(old)}): {old[:80]}")
    t11 = t11.replace(old, new)
s = s[:a] + t11 + s[b:]
open(path, "w", encoding="utf8").write(s)
print("edited", REL[-60:])
