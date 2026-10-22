/** 🔌️ The backend registry is the only place neutral code learns which backends exist: its schema and rows are valid and complete, every module and driver it names exists and exports what the interface demands, schemas, catalogue and help derive from it, and neutral files are ratcheted against naming a backend. */
import {test,expect} from "bun:test";
import Ajv from "ajv";
import {existsSync,readFileSync} from "node:fs";
import {join} from "node:path";
import schema from "../@@SCHEMA@@/@@JSON@@" with {type:"json"};
import registry from "../@@REGISTRY@@/@@JSON@@" with {type:"json"};
import policy from "@@UP@@🔣️policy.json" with {type:"json"};
import catalog from "@@UP@@@@OPERATIONS@@/🔣️catalog.json" with {type:"json"};
import conformance from "@@UP@@@@CONF@@/@@SCENARIOS@@/@@JSON@@" with {type:"json"};
import {validateJsonSchemaSubset} from "@@VALID@@";
import {buildBackendDefaultIdV1,backendMissingOperationsV1,buildBackendAlternationV1,buildBackendBootstrapV1,buildBackendNativeNamespacesV1,buildBackendIdsV1,buildBackendRowV1,buildBackendRowsV1,buildNativeSnapshotProtocolV1,loadBuildBackendV1,readBackendRegistryV1,requireBuildBackendIdV1} from "../@@TS@@";
import {renderBuildHelpV1,type BuildHelpCatalogV1} from "@@UP@@@@OPERATIONS@@/@@HELP@@/@@TS@@";

const ajv=new Ajv({strict:true,allowUnionTypes:true}),build=join(import.meta.dir,"../..");
const mutated=(change:(copy:any)=>void)=>{const copy=structuredClone(registry) as any;change(copy);return copy;};

test("the registry validates against its schema and refuses every broken row",()=>{
 expect(ajv.validateSchema(schema)).toBe(true);
 expect(ajv.compile(schema)(registry)).toBe(true);
 expect(validateJsonSchemaSubset(schema,registry,schema)).toEqual([]);
 expect(readBackendRegistryV1().map(row=>row.id)).toEqual(["nx","bazel","semio"]);
 for(const[label,change]of [["id",(c:any)=>{c.backends[0].id="Nx";}],["duplicate",(c:any)=>{c.backends[1].id="nx";}],["module",(c:any)=>{c.backends[0].module="/abs/x.ts";}],["driver",(c:any)=>{delete c.backends[0].driver;}],["kind",(c:any)=>{c.backends[0].kind="cloud";}],["model",(c:any)=>{c.backends[0].dependencyModel="some";}],["capability",(c:any)=>{c.backends[0].capabilities.teleport={support:"native"};}],["support",(c:any)=>{c.backends[0].capabilities.plan.support="magic";}],["empty",(c:any)=>{c.backends=[];}],["extra",(c:any)=>{c.backends[0].secret=1;}],["note",(c:any)=>{c.backends[0].capabilities.plan.note={en:"x"};}]] as const)
  expect(()=>readBackendRegistryV1(mutated(change)),label).toThrow();
});

test("every backend declares every capability and a claim for each scenario's capability, and every claim carries a note when the support is missing",()=>{
 const capabilities=schema.definitions.Capability.enum;
 for(const row of buildBackendRowsV1()){
  expect(Object.keys(row.capabilities).sort(),row.id).toEqual([...capabilities].sort());
  for(const[name,claim]of Object.entries(row.capabilities))if(claim!==undefined&&claim.support==="missing")expect(claim.note,`${row.id} ${name}`).toBeDefined();
  for(const[,claim]of Object.entries(row.capabilities))if(claim?.note)expect(claim.note.de).not.toBe(claim.note.en);
  expect(row.title.de.length).toBeGreaterThan(0);
 }
 for(const scenario of (conformance as any).scenarios)expect(capabilities).toContain(scenario.capability);
});

test("the policy default, the catalogue and the help derive from the registry",()=>{
 expect(buildBackendIdsV1()).toContain(buildBackendDefaultIdV1());
 expect(buildBackendRowsV1().filter(row=>row.default===true).length).toBe(1);
 expect((policy as Record<string,unknown>).defaultTool).toBeUndefined();
 const names=new Set((catalog as any).operations.map((row:any)=>row.name));
 for(const row of buildBackendRowsV1())for(const operation of Object.keys(row.operations))expect(names.has(operation),`${row.id} ${operation}`).toBe(true);
 for(const operation of (catalog as any).operations)expect(operation.implementations,operation.name).toBeUndefined();
 for(const flag of (catalog as any).flags)if(flag.valueName?.includes("nx")||flag.valueName?.includes("bazel"))throw Error(`flag ${flag.name} names a backend in its value`);
 const alternation=buildBackendAlternationV1();
 expect(alternation).toBe(buildBackendIdsV1().join("|"));
 for(const language of (catalog as any).languages){
  const text=renderBuildHelpV1(catalog as unknown as BuildHelpCatalogV1,{},language);
  expect(text).toContain(`repo [--tool=${alternation}]`);expect(text).not.toContain("{backends}");
  expect(renderBuildHelpV1(catalog as unknown as BuildHelpCatalogV1,{operation:"tools"},language)).toContain(`tools setup [${alternation}|all]`);
 }
 expect(backendMissingOperationsV1("bazel")).toEqual(["inputs"]);expect(backendMissingOperationsV1("semio")).toEqual([]);
 expect(()=>buildBackendRowV1("buck2")).toThrow("Unknown build tool: buck2 (nx|bazel|semio)");
 expect(()=>requireBuildBackendIdV1("")).toThrow("Unbekanntes Build-Werkzeug");
});

test("the native-snapshot protocol is declared by exactly one backend and carries its variable and environment",()=>{
 const owners=buildBackendRowsV1().filter(row=>row.nativeSnapshot);
 expect(owners.length).toBe(1);
 expect(buildNativeSnapshotProtocolV1()).toEqual(owners[0]!.nativeSnapshot);
 expect(buildNativeSnapshotProtocolV1()!.variable).toMatch(/^SEMIO_[A-Z0-9_]+$/u);
});

test("nested native launches and cache proofs are declared per backend, never by neutral code",async()=>{
 const rows=buildBackendRowsV1(),namespaces=buildBackendNativeNamespacesV1();
 expect(namespaces).toEqual(rows.flatMap(row=>row.nativeCaller?[row.nativeCaller.namespace]:[]));
 expect(new Set(namespaces).size).toBe(namespaces.length);
 for(const row of rows.filter(candidate=>candidate.nativeCaller)){
  expect(existsSync(join(build,"@@BACKENDS@@",row.nativeCaller!.schema)),row.id).toBe(true);
  const launch=await buildBackendBootstrapV1(["tools","setup",row.id],row.id);
  if(launch)expect(launch.namespace).toBe(row.nativeCaller!.namespace);
 }
 for(const row of rows){
  expect(row.cacheProof,row.id).toBeDefined();
  if(row.cacheProof?.logMarker)expect(()=>new RegExp(row.cacheProof!.logMarker!,"u")).not.toThrow();
  for(const source of Object.values(row.cacheProof?.spawnMarker??{}))expect(()=>new RegExp(source,"u")).not.toThrow();
 }
 expect(readBackendRegistryV1({version:1,backends:rows.map(row=>({...row,cacheProof:{logMarker:"("}}))}).length).toBe(rows.length);
});

test("every module and driver a row names exists, and the in-house backend loads with its own id",async()=>{
 for(const row of buildBackendRowsV1())for(const path of [row.module,row.driver])expect(existsSync(join(build,"@@BACKENDS@@",path)),`${row.id} ${path}`).toBe(true);
 const semio=await loadBuildBackendV1("semio");
 expect(semio.id).toBe("semio");
 for(const method of ["setup","reset","prepareStorage","cache","remoteCache","execute"] as const)expect(typeof semio[method],method).toBe("function");
 expect((await semio.cache()).tool).toBe("semio");
 expect((await semio.remoteCache()).tool).toBe("semio");
 expect(await loadBuildBackendV1("semio")).toBe(semio);
 await expect(loadBuildBackendV1("buck2")).rejects.toThrow("Unknown build tool");
 expect(await buildBackendBootstrapV1(["run","a:b"],"semio")).toBeUndefined();
 expect(await buildBackendBootstrapV1(["tools","setup","semio"],"nx")).toBeUndefined();
 await expect(buildBackendBootstrapV1(["tools","setup","nobody"],"nx")).rejects.toThrow("Unknown build tool");
});

const NEUTRAL_CLEAN=["🔌️backends/🟦️.ts","⚙️operations/🟦️.ts","🌐️remote-cache/⌨️command/🟦️.ts","🌐️remote-cache/🟦️.ts","🔧️tools/💾️storage/🟦️.ts","📥️inputs/🎯️selected/📦️snapshot/🟦️.ts","📥️inputs/🎯️selected/🦀️rust/🟦️.ts","⚙️operations/📖️help/🟦️.ts","⚙️operations/🔣️catalog.json","🧬️schema/🔣️.json","🏃️execution/🧬️schema/🌐️remote-cache.json","⚙️operations/🧬️schema/🔣️.json","📏️incremental/⌨️command/🟦️.ts","📏️incremental/🤝️concurrent/⌨️command/🟦️.ts","📐️conformance/🟦️.ts","📐️conformance/🏃️runner/🟦️.ts","📐️conformance/⌨️command/🟦️.ts","🧮️task-model/🟦️.ts","🧭️invocation/🟦️.ts","📥️caller/🟦️.ts","📥️caller/📤️orchestration/🟨️.mjs","📥️caller/📤️child/🟦️.ts","🧾️ci-report/🟦️.ts","🧾️ci-report/📜️script.ts","🌐️remote-cache/💾️store/🟦️.ts","🌐️remote-cache/🖥️server/🟦️.ts","🕸️graph/📥️inference/🧩️component-targets/🟨️.mjs","🚀️bootstrap/⌨️cli/🟦️.ts"];
const KNOWN_LEAKS:Record<string,number>={"📜️script.ts":1,"🏃️execution/🔒️hermetic/🚪️entry/🟦️.ts":1,"📏️incremental/🟦️.ts":2,"📏️incremental/🏃️runner/🟦️.ts":3,"📏️incremental/👁️observers/🟦️.ts":2,"📏️incremental/🤝️concurrent/🟦️.ts":4,"📏️incremental/🤝️concurrent/🏃️runner/🟦️.ts":3,"🪞️parity/🟦️.ts":16,"🪞️parity/🏃️collect/🟦️.ts":3};
const literal=/(["'`])(nx|bazel)\1|\b(?:tool|selected\.tool)\s*[!=]==\s*["'](nx|bazel)["']/gu;
test("neutral files never name a backend: the cleaned ones stay clean and the remaining ones may not grow",()=>{
 for(const path of NEUTRAL_CLEAN){
  const file=join(build,path);
  if(!existsSync(file))continue;
  const text=readFileSync(file,"utf8"),hits=[...text.matchAll(literal)].map(match=>match[0]);
  expect(hits,path).toEqual([]);
 }
 for(const[path,limit]of Object.entries(KNOWN_LEAKS)){
  const file=join(build,path);
  if(!existsSync(file))continue;
  expect([...readFileSync(file,"utf8").matchAll(literal)].length,path).toBeLessThanOrEqual(limit);
 }
});

test("schemas that name a build tool accept every registered id and nothing else",()=>{
 const ids=buildBackendIdsV1(),pattern=/^[a-z][a-z0-9-]{0,31}$/u;
 for(const id of ids)expect(pattern.test(id)).toBe(true);
 for(const bad of ["","Nx","a b","1x","x".repeat(40)])expect(pattern.test(bad)).toBe(false);
 for(const path of ["🧬️schema/🔣️.json","🧬️schema/📥️authoring.json","🧬️schema/📥️caller.json","🏃️execution/🧬️schema/🌐️remote-cache.json","📏️incremental/🧬️schema/🔣️.json","🪞️parity/🧬️schema/🔣️.json","🧾️ci-report/🧬️schema/🔣️.json"]){
  const file=join(build,path);
  if(!existsSync(file))continue;
  const text=readFileSync(file,"utf8");
  expect(text,path).not.toMatch(/"enum":\s*\[\s*"nx",\s*"bazel"\s*\]/u);expect(text,path).not.toContain("^(nx|bazel)$");
 }
});
