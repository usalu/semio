/** 👁️ Inference observation law over `./🧫️fixtures/🔣️.json`: a persisted inference result is reused exactly while every filesystem fact the
 * computation observed still holds, identical rewrites stay hits, any observed change is a miss, and a hit always equals a fresh recomputation.
 * Oracles: Ajv validates every persisted record against `../🧬️schema/🔣️.json`; fast-glob independently lists the probed directory. */
import { expect, test, spyOn, beforeAll } from "bun:test";
import * as rawFs from "node:fs";
import { createHash } from "node:crypto";
import { brotliCompressSync, brotliDecompressSync } from "node:zlib";
import { createRequire } from "node:module";
import ts from "typescript";
import { mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, dirname, relative, resolve } from "node:path";
import Ajv from "ajv";
import fastGlob from "fast-glob";

type Scenario = { id: string; mutation: { op: "none" | "write" | "replace-directory-with-file"; path?: string; content?: string }; expected: "hit" | "miss" };
const fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json", import.meta.url), "utf8")) as { schemaVersion: number; files: Record<string, string>; scenarios: Scenario[];corruption:{value:string;computations:number};races:{before:string;after:string;entry:string;directory:string;conflict:string;text:string;ports:string[]};semantic:{parentValidation:{projects:number;expectedReads:number};provider:{id:string;revision:string;nextRevision:string};before:string;bodyChanged:string;importsChanged:string;expected:string[];changedExpected:string[];cacheMaximumBytes:number;oversizedCharacters:number;malformedFactPatches:unknown[][];nested:{path:string;before:string;after:string;missing:string};replayError:string} };
const validate = new Ajv({ strict: true }).compile(JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json", import.meta.url), "utf8")));
const { observedFs, observeInference, inferenceHeaderLine, inferenceObservationBlockPathV1, readInferenceHeaderV1 } = await import(new URL("../🟨️.mjs", import.meta.url).href) as typeof import("../🟨️.mjs");
beforeAll(async()=>{const root=process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir();mkdirSync(root,{recursive:true});await observeInference({cachePath:join(root,"observation-test-load.json"),key:"observation-test-load"},()=>undefined);});

test("Rust and native closure memo hits retain every receiving scope's source facts",async()=>{
 const {closureReplay:row}=JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json",import.meta.url),"utf8")),parent=process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),root=mkdtempSync(join(parent,"closure-"));
 const api=await import("../🟨️.mjs"),plugin=await import("../../🕸️graph/📥️inference/🟨️.mjs");await plugin.libraryBootstrap;
 try{
  for(const[path,content]of Object.entries(row.files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),content as string);}
  const snapshots=new Map(),rust=plugin.cacheInternals.createRustSourceCache(),closure=(entries:string[])=>api.captureObservedSourceInputsV1(()=>plugin.cacheInternals.rustSourceFiles(entries.map(path=>join(root,path)),root,rust,snapshots),undefined,false);
  const first=await closure(row.entries),same=await closure(row.entries),leaf=await closure(row.leafEntries);
  expect(first.value.map((path:string)=>relative(root,path).replaceAll("\\","/"))).toEqual(row.rustExpected);
  expect(first.value).toEqual((await fastGlob("src/*.rs",{cwd:root,absolute:true})).sort());expect(same.observations).toEqual(first.observations);
  expect(leaf.observations.some((fact:any)=>fact.path===join(root,"src/leaf.rs")&&fact.kind==="analysis")).toBe(true);
  const files=new Map(),closures=new Map(),native=()=>api.captureObservedSourceInputsV1(()=>plugin.cacheInternals.nativeDependencyRoots("a",root,false,files,closures),undefined,false);
  const before=await native(),repeated=await native();expect(before.value).toEqual(row.nativeExpected);expect(repeated.observations).toEqual(before.observations);
  writeFileSync(join(root,row.manifest),row.mutation);
  const changed=await native();expect(changed.value).toEqual(row.nativeChanged);expect(await api.observationsHold(before.observations)).toBe(false);expect(await api.observationsHold(changed.observations)).toBe(true);
  const oracle=createRequire(import.meta.url)("smol-toml").parse(row.mutation);expect(oracle.dependencies.c.path).toBe("../"+row.nativeChanged.at(-1));
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("invocation memo replays complete current observations into every receiving scope",async()=>{
 const {invocationMemo:row}=JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
 const parent=process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),root=mkdtempSync(join(parent,"memo-")),path=join(root,row.path),cache=new Map(),counts:number[]=[];
 const api=await import("../🟨️.mjs");let calls=0;
 try{
  writeFileSync(path,row.before);
  const compute=()=>api.memoizedObservedComputationV1(cache,path,()=>{calls++;return JSON.parse(api.observedFs.readFileSync(path,"utf8"));});
  const first=await api.captureObservedSourceInputsV1(compute,undefined,false);counts.push(calls);
  const second=await api.captureObservedSourceInputsV1(compute,undefined,false);counts.push(calls);
  expect(first.value).toEqual(JSON.parse(row.before));{const canonical=(rows:readonly{kind:string;path?:string}[])=>[...rows].sort((a,b)=>(a.kind+"\0"+(a.path??"")).localeCompare(b.kind+"\0"+(b.path??"")));expect(canonical(second.observations)).toEqual(canonical(first.observations));}
  writeFileSync(path,row.after);
  const changed=await api.captureObservedSourceInputsV1(compute,undefined,false);counts.push(calls);
  expect(changed.value).toEqual(JSON.parse(row.after));expect(counts).toEqual(row.expectedCalls);
  expect(await api.observationsHold(first.observations)).toBe(false);expect(await api.observationsHold(changed.observations)).toBe(true);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("project observations await asynchronous JSON computation before admission",async()=>{
 const {invocationMemo:row}=JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
 const parent=process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),root=mkdtempSync(join(parent,"async-")),path=join(root,row.path),api=await import("../🟨️.mjs");
 try{
  writeFileSync(path,JSON.stringify(row.projectResult));
  const compute=()=>api.observeProjectInferenceV1({cachePath:join(root,"image.json"),key:"async-project",owner:root},async()=>JSON.parse(await api.observedFs.readFile(path,"utf8")));
  const first=await api.captureObservedSourceInputsV1(compute,undefined,false),second=await api.captureObservedSourceInputsV1(compute,undefined,false);
  expect(first.value).toEqual(row.projectResult);expect(second.value).toEqual(row.projectResult);{const canonical=(rows:readonly{kind:string;path?:string}[])=>[...rows].sort((a,b)=>(a.kind+"\0"+(a.path??"")).localeCompare(b.kind+"\0"+(b.path??"")));expect(canonical(second.observations)).toEqual(canonical(first.observations));}
  expect(api.projectInferenceCountsV1()[root]).toBe(1);
 }finally{rmSync(root,{recursive:true,force:true});}
});

/** 🧪️ One isolated workspace whose files are older than the racy window, so the first result is persisted. */
function workspace(): string {
  const parent = process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir();
  mkdirSync(parent, { recursive: true });
  const root = mkdtempSync(join(realpathSync(parent), "io-"));
  for (const [path, content] of Object.entries(fixture.files)) { mkdirSync(join(root, path, ".."), { recursive: true }); writeFileSync(join(root, path), content); }
  Bun.sleepSync(2200);
  return root;
}

/** 🧱️ The rows of one persisted record, decoded independently of production: the header names content-addressed blocks (brotli JSON arrays named by the sha256 of their bytes) that precede the header's inline rows; the production expansion must agree row for row. */
function recordHeader(cachePath: string): { digest: string; resultSha256: string; blocks: string[]; authorityObservations: unknown[]; observations: any[] } {
  const line = JSON.parse(inferenceHeaderLine(cachePath)!) as { digest: string; resultSha256: string; blocks: string[]; authorityObservations: unknown[]; observations: any[] };
  const blocked = line.blocks.flatMap((name) => { const bytes = brotliDecompressSync(readFileSync(inferenceObservationBlockPathV1(dirname(resolve(cachePath)), name))); expect(createHash("sha256").update(bytes).digest("hex")).toBe(name); return JSON.parse(bytes.toString("utf8")) as any[]; });
  const expanded = { ...line, observations: [...blocked, ...line.observations] };
  expect(expanded.observations).toEqual(readInferenceHeaderV1(cachePath).observations);
  return expanded;
}

/** 🔍️ The probe program: one read, one absence, one listing and one type probe, all through the observed filesystem. */
function probe(root: string): Readonly<Record<string, unknown>> {
  return { one: observedFs.readFileSync(join(root, "a/one.txt"), "utf8"), missing: observedFs.existsSync(join(root, "c/missing.txt")), listing: observedFs.readdirSync(join(root, "a")).sort(), bIsDirectory: observedFs.statSync(join(root, "b")).isDirectory() };
}

test("fixture is closed and versioned", () => {
  expect(fixture.schemaVersion).toBe(1);
  expect(new Set(fixture.scenarios.map((row) => row.id)).size).toBe(fixture.scenarios.length);
});

for (const scenario of fixture.scenarios) test(`observed inference reuse: ${scenario.id}`, async () => {
  const root = workspace(), cachePath = join(root, ".cache", "inference.json");
  let computations = 0;
  const compute = (): Readonly<Record<string, unknown>> => { computations++; return probe(root); };
  const first = await observeInference({ cachePath, key: ["law", scenario.id] }, compute);
  expect(computations).toBe(1);
  const header = JSON.parse(inferenceHeaderLine(cachePath)!);
  expect(validate(header), JSON.stringify(validate.errors)).toBe(true);
  expect(first.listing).toEqual((await fastGlob("*", { cwd: join(root, "a"), onlyFiles: false })).sort());
  if (scenario.mutation.op === "write") { mkdirSync(join(root, scenario.mutation.path!, ".."), { recursive: true }); writeFileSync(join(root, scenario.mutation.path!), scenario.mutation.content!); }
  if (scenario.mutation.op === "replace-directory-with-file") { rmSync(join(root, scenario.mutation.path!), { recursive: true }); writeFileSync(join(root, scenario.mutation.path!), "file"); }
  const second = await observeInference({ cachePath, key: ["law", scenario.id] }, compute);
  expect(computations).toBe(scenario.expected === "hit" ? 1 : 2);
  expect(second).toEqual(probe(root));
  rmSync(root, { recursive: true, force: true });
}, 20_000);

test("a result that is not exactly JSON-representable is never persisted", async () => {
  const root = workspace(), cachePath = join(root, ".cache", "inexact.json");
  let computations = 0;
  for (let round = 0; round < 2; round++) await observeInference({ cachePath, key: ["inexact"] }, () => { computations++; return { value: undefined, file: observedFs.readFileSync(join(root, "a/two.txt"), "utf8") }; });
  expect(computations).toBe(2);
  rmSync(root, { recursive: true, force: true });
}, 20_000);

const { memoizedFileRecord, observationsHold, observeDependencies } = await import(new URL("../🟨️.mjs", import.meta.url).href) as typeof import("../🟨️.mjs");

for (const scenario of fixture.scenarios) test(`non-blocking validation agrees with reuse: ${scenario.id}`, async () => {
  const root = workspace(), cachePath = join(root, ".cache", "inference.json");
  await observeInference({ cachePath, key: ["hold", scenario.id] }, () => probe(root));
  const header = recordHeader(cachePath);
  expect(header.observations.length, "the record holds the probe's facts").toBeGreaterThan(0);
  if (scenario.mutation.op === "write") { mkdirSync(join(root, scenario.mutation.path!, ".."), { recursive: true }); writeFileSync(join(root, scenario.mutation.path!), scenario.mutation.content!); }
  if (scenario.mutation.op === "replace-directory-with-file") { rmSync(join(root, scenario.mutation.path!), { recursive: true }); writeFileSync(join(root, scenario.mutation.path!), "file"); }
  expect(await observationsHold(header.observations), scenario.id).toBe(scenario.expected === "hit");
  rmSync(root, { recursive: true, force: true });
}, 20_000);

test("a per-file analysis is memoized across inference misses by content digest and recomputed when the bytes change", async () => {
  const root = workspace(), cachePath = join(root, ".cache", "memo.json"), file = join(root, "a/one.txt");
  let analyses = 0;
  const analyse = (bytes: Buffer): Readonly<{ length: number; upper: string }> => { analyses++; return { length: bytes.byteLength, upper: bytes.toString("utf8").toUpperCase() }; };
  const run = (key: string) => observeInference({ cachePath, key: ["memo", key] }, () => ({ fact: memoizedFileRecord("law-analysis-v1", file, analyse) }));
  const first = await run("one");
  const second = await run("two");
  expect(second).toEqual(first);
  expect(analyses).toBe(1);
  const header = recordHeader(cachePath) as { observations: { kind: string; path: string }[] };
  expect(header.observations.some((row) => row.kind === "file" && row.path === file)).toBe(true);
  writeFileSync(file, "changed bytes\n");
  Bun.sleepSync(2200);
  const third = await run("three");
  expect(analyses).toBe(2);
  expect(third).toEqual({ fact: analyse(Buffer.from("changed bytes\n")) });
  rmSync(root, { recursive: true, force: true });
}, 30_000);

test("a dependency receipt names the persisted inference records this process used, and is withdrawn when an input is racy", async () => {
  const root = workspace(), cachePath = join(root, ".cache", "nodes.json"), receiptPath = join(root, ".cache", "nodes.receipt.json");
  await observeInference({ cachePath, key: ["receipt"] }, () => probe(root));
  await observeDependencies({ receiptPath }, () => observedFs.readFileSync(join(root, "a/two.txt"), "utf8"));
  const receipt = JSON.parse(readFileSync(receiptPath, "utf8")) as { pid: number; inference: [string, string][]; observations: { path: string }[] };
  expect(receipt.pid).toBe(process.pid);
  expect(receipt.inference.some(([path]) => path === cachePath)).toBe(true);
  expect(receipt.observations.some((row) => row.path === join(root, "a/two.txt"))).toBe(true);
  writeFileSync(join(root, "a/two.txt"), "fresh\n");
  await observeDependencies({ receiptPath }, () => observedFs.readFileSync(join(root, "a/two.txt"), "utf8"));
  expect(() => readFileSync(receiptPath)).toThrow();
  rmSync(root, { recursive: true, force: true });
}, 30_000);

test("persisted inference body corruption requires original source recomputation",async()=>{const root=workspace(),cachePath=join(root,"cache.json");let computations=0;try{const compute=()=>{computations++;return probe(root);};const before=await observeInference({cachePath,key:"corruption"},compute),bytes=brotliDecompressSync(readFileSync(cachePath)).toString("utf8"),split=bytes.indexOf("\n");writeFileSync(cachePath,brotliCompressSync(Buffer.from(bytes.slice(0,split+1)+JSON.stringify({...before,one:fixture.corruption.value}))));expect(await observeInference({cachePath,key:"corruption"},compute)).toEqual(before);expect(computations).toBe(fixture.corruption.computations);}finally{rmSync(root,{recursive:true,force:true});}});

const {captureObservedSourceInputsV1,observeProjectInferenceV1,observedTypeScriptHost}=await import(new URL("../🟨️.mjs",import.meta.url).href) as typeof import("../🟨️.mjs");
const digest=(value:string|Buffer)=>createHash("sha256").update(value).digest("hex");

for(const typed of [false,true])test(`consumed directory listing is observed once: ${typed?"typed":"names"}`,async()=>{
 const root=workspace(),directory=join(root,"a"),original=rawFs.readdirSync;let reads=0;
 const mocked=spyOn(rawFs,"readdirSync").mockImplementation(((path:any,options:any)=>{const value=original(path,options);if(String(path)===directory&&++reads===1)writeFileSync(join(directory,fixture.races.entry),"new");return value;}) as typeof rawFs.readdirSync);
 try{const actual=await captureObservedSourceInputsV1(()=>typed?observedFs.readdirSync(directory,{withFileTypes:true}):observedFs.readdirSync(directory)),names=actual.value.map((entry:any)=>typeof entry==="string"?entry:entry.name).sort(),row=actual.observations.find((row:any)=>row.path===directory&&row.kind===(typed?"readdir":"readdirNames"));expect(reads).toBe(1);expect(names).not.toContain(fixture.races.entry);expect(row?.fact).toBe(digest(JSON.stringify(typed?actual.value.map((entry:any)=>[entry.name,entry.isFile()?"file":"other"]).sort():names)));expect(await observationsHold(actual.observations)).toBe(false);expect((await fastGlob("*",{cwd:directory,onlyFiles:false}))).toContain(fixture.races.entry);}finally{mocked.mockRestore();rmSync(root,{recursive:true,force:true});}
},20000);

for(const port of fixture.races.ports)test(`consumed TypeScript host result owns authority: ${port}`,async()=>{
 const root=workspace(),path=port==="getDirectories"?root:join(root,"a/one.txt");let returned:any;
 try{const sys={...ts.sys,[port]:(...args:any[])=>{if(port==="readFile"){writeFileSync(path,fixture.races.after);return returned=ts.sys.readFile(path);}if(port==="fileExists"||port==="directoryExists"){returned=false;return returned;}if(port==="realpath")return returned=join(root,"b");mkdirSync(join(root,fixture.races.directory));return returned=ts.sys.getDirectories(root);}},actual=await captureObservedSourceInputsV1(()=>{const host=observedTypeScriptHost(sys);return (host as any)[port](path);}),kind=({readFile:"typescriptText",fileExists:"fileExists",directoryExists:"directoryExists",realpath:"typescriptRealpath",getDirectories:"directories"} as Record<string,string>)[port],row=actual.observations.find((row:any)=>row.path===path&&row.kind===kind);expect(actual.value).toEqual(returned);expect(row?.fact).toEqual(port==="readFile"?digest(returned):port==="getDirectories"?digest(JSON.stringify(returned.sort())):returned);if(port==="readFile"||port==="getDirectories")expect(await observationsHold(actual.observations)).toBe(true);}finally{rmSync(root,{recursive:true,force:true});}
},20000);

test("consumed TypeScript BOM text agrees with the independent compiler host",async()=>{
 const root=workspace(),path=join(root,"a/one.txt");
 try{for(const encoding of ["utf8","utf16le","utf16be"]){const body=Buffer.from(fixture.races.text,encoding==="utf8"?"utf8":"utf16le"),bytes=encoding==="utf8"?Buffer.concat([Buffer.from([239,187,191]),body]):Buffer.concat([Buffer.from([255,254]),body]);if(encoding==="utf16be")bytes.swap16();writeFileSync(path,bytes);const actual=await captureObservedSourceInputsV1(()=>observedTypeScriptHost(ts.sys).readFile(path));expect(actual.value).toBe(ts.sys.readFile(path));expect(actual.value).toBe(fixture.races.text);expect(await observationsHold(actual.observations)).toBe(true);}}finally{rmSync(root,{recursive:true,force:true});}
},20000);

test("conflicting repeated actual reads are retained and inference is refused",async()=>{
 const root=workspace(),path=join(root,"a/one.txt"),cachePath=join(root,"conflict.json");let failed:any[]=[];
 try{await expect(captureObservedSourceInputsV1(()=>observeInference({cachePath,key:"conflict"},()=>{const first=observedFs.readFileSync(path,"utf8");writeFileSync(path,fixture.races.after);const second=observedFs.readFileSync(path,"utf8");return {first,second};}),(rows:any[])=>{failed=rows;})).rejects.toThrow(fixture.races.conflict);const rows=failed.filter(row=>row.kind==="file"&&row.path===path);expect(rows).toHaveLength(2);expect(new Set(rows.map(row=>row.fact.sha256))).toEqual(new Set([digest(fixture.files["a/one.txt"]!),digest(fixture.races.after)]));expect(await observationsHold(rows,64,new Set([join(root,"a")]))).toBe(false);expect(rawFs.existsSync(cachePath)).toBe(false);}finally{rmSync(root,{recursive:true,force:true});}
},20000);

test("captures original facts inherited from cached project inference",async()=>{
 const root=workspace(),path=join(root,"a/one.txt"),cachePath=join(root,"project.json");let computations=0;
 try{const run=()=>observeProjectInferenceV1({cachePath,key:"cached-project",owner:root},()=>{computations++;return probe(root);});await run();const actual=await captureObservedSourceInputsV1(run);expect(computations).toBe(1);expect(actual.value).toEqual(probe(root));expect(actual.observations.some(row=>row.kind==="file"&&row.path===path)).toBe(true);expect(await observationsHold(actual.observations)).toBe(true);const header=JSON.parse(inferenceHeaderLine(cachePath)!);expect(validate(header),JSON.stringify(validate.errors)).toBe(true);}finally{rmSync(root,{recursive:true,force:true});}
},20000);

test("conflicting cached project facts cannot overwrite a parent's consumed read",async()=>{
 const root=workspace(),path=join(root,"a/one.txt"),cachePath=join(root,"project.json");let failed:any[]=[];
 try{const run=()=>observeProjectInferenceV1({cachePath,key:"cached-conflict",owner:root},()=>probe(root));await run();await expect(captureObservedSourceInputsV1(()=>observeInference({cachePath:join(root,"parent.json"),key:"parent"},async()=>{writeFileSync(path,fixture.races.after);observedFs.readFileSync(path,"utf8");writeFileSync(path,fixture.files["a/one.txt"]!);return run();}),(rows:any[])=>{failed=rows;})).rejects.toThrow(fixture.races.conflict);expect(new Set(failed.filter(row=>row.kind==="file"&&row.path===path).map(row=>row.fact.sha256)).size).toBe(2);}finally{rmSync(root,{recursive:true,force:true});}
},20000);

test("failed reads retain their original captured error facts",async()=>{
 const root=workspace(),path=join(root,"absent.txt");let failed:any[]=[];
 try{await expect(captureObservedSourceInputsV1(()=>observedFs.readFileSync(path),(rows:any[])=>{failed=rows;})).rejects.toThrow();expect(failed).toContainEqual({kind:"file",path,fact:{error:"ENOENT"}});expect(await observationsHold(failed)).toBe(true);}finally{rmSync(root,{recursive:true,force:true});}
},20000);

test("cancelled observation validation refuses before probing",async()=>{
 const controller=new AbortController();controller.abort(Error("original cancelled"));await expect(observationsHold([],64,new Set(),new Set(),controller.signal)).rejects.toThrow("original cancelled");
});

test("neutral changed bytes report the original stale fact without admitting a mixed image",async()=>{
 const root=workspace(),path=join(root,"a/one.txt"),changed:any[]=[];
 try{const captured=await captureObservedSourceInputsV1(()=>observedFs.readFileSync(path,"utf8")),rows=captured.observations.filter(row=>row.path===path);writeFileSync(path,fixture.races.after);const original=rows[0];if(!original||original.kind!=="file"||!("sha256" in original.fact))throw Error("Original physical Source fact required");expect(digest(readFileSync(path))).not.toBe(original.fact.sha256);expect(await observationsHold(rows,64,new Set(),new Set(),undefined,(row:any)=>changed.push(row))).toBe(false);expect(changed).toEqual(rows);}finally{rmSync(root,{recursive:true,force:true});}
},20000);

const {registerSourceAnalysisV1,observedSourceAnalysisV1,observedSourceAnalysisAsyncV1,sourceObservationStatsV1}=await import(new URL("../🟨️.mjs",import.meta.url).href) as typeof import("../🟨️.mjs");
const {commandSourceImports}=await import(new URL("../../../⚡️caching/📥️inference/🟨️.mjs",import.meta.url).href) as typeof import("../../../⚡️caching/📥️inference/🟨️.mjs");
const semanticParser=(path:string,bytes:Buffer)=>commandSourceImports(path,bytes.toString("utf8"));

for(const asynchronous of [false,true])test(`semantic source analysis admits body edits and refuses changed imports: ${asynchronous?"async":"sync"}`,async()=>{
 const root=workspace(),path=join(root,"source.ts"),provider=registerSourceAnalysisV1({id:fixture.semantic.provider.id,revision:fixture.semantic.provider.revision,analyze:semanticParser});writeFileSync(path,fixture.semantic.before);
 try{const captured=await captureObservedSourceInputsV1(()=>asynchronous?observedSourceAnalysisAsyncV1(provider,path):observedSourceAnalysisV1(provider,path)),rows=captured.observations.filter(row=>row.path===path),row=rows[0]!;if(row.kind!=="analysis")throw Error("Original registered parser observation required");expect(Object.isFrozen(provider)).toBe(true);expect(provider.id).toBe(fixture.semantic.provider.id);expect(captured.value).toEqual(ts.preProcessFile(fixture.semantic.before,true,true).importedFiles.map(item=>item.fileName));expect(captured.value).toEqual(fixture.semantic.expected);expect(rows.map(item=>item.kind)).toEqual(["analysis"]);expect(row.fact.resultSha256).toBe(digest(JSON.stringify(captured.value)));expect(validate({digest:"a".repeat(64),resultSha256:"b".repeat(64),observations:rows,authorityObservations:[]}),JSON.stringify(validate.errors)).toBe(true);writeFileSync(path,fixture.semantic.before);expect(await observationsHold(rows)).toBe(true);writeFileSync(path,fixture.semantic.bodyChanged);expect(await observationsHold(rows)).toBe(true);writeFileSync(path,fixture.semantic.importsChanged);expect(await observationsHold(rows)).toBe(false);expect(semanticParser(path,readFileSync(path))).toEqual(fixture.semantic.changedExpected);}finally{rmSync(root,{recursive:true,force:true});}
},20000);

test("semantic source analysis keeps repeated equivalent results without hiding raw consumers",async()=>{
 const root=workspace(),path=join(root,"source.ts"),provider=registerSourceAnalysisV1({id:fixture.semantic.provider.id,revision:fixture.semantic.provider.revision,analyze:semanticParser});writeFileSync(path,fixture.semantic.before);
 try{const captured=await captureObservedSourceInputsV1(()=>observeInference({cachePath:join(root,"semantic.json"),key:"body-rewrite"},()=>{const before=observedSourceAnalysisV1(provider,path);writeFileSync(path,fixture.semantic.bodyChanged);return {before,after:observedSourceAnalysisV1(provider,path)};}));expect(captured.value).toEqual({before:fixture.semantic.expected,after:fixture.semantic.expected});const semantic=captured.observations.filter(row=>row.kind==="analysis"&&row.path===path);expect(semantic).toHaveLength(2);expect(await observationsHold(semantic)).toBe(true);writeFileSync(path,fixture.semantic.before);const mixed=await captureObservedSourceInputsV1(()=>{observedFs.readFileSync(path);writeFileSync(path,fixture.semantic.bodyChanged);return observedSourceAnalysisV1(provider,path);});expect(mixed.observations.filter(row=>row.path===path).map(row=>row.kind).sort()).toEqual(["analysis","file"]);expect(await observationsHold(mixed.observations)).toBe(false);}finally{rmSync(root,{recursive:true,force:true});}
},20000);

test("semantic source analysis cache hits retain nested consumed probes",async()=>{
 const root=workspace(),path=join(root,"source.ts"),configuration=join(root,fixture.semantic.nested.path),missing=join(root,fixture.semantic.nested.missing),cachePath=join(root,"nested.json");let analyses=0;
 writeFileSync(path,fixture.semantic.before);writeFileSync(configuration,fixture.semantic.nested.before);Bun.sleepSync(2200);
 const provider=registerSourceAnalysisV1({id:fixture.semantic.provider.id+":nested",revision:fixture.semantic.provider.revision,analyze:(source:string,bytes:Buffer)=>{analyses++;observedFs.readFileSync(configuration,"utf8");observedFs.existsSync(missing);return semanticParser(source,bytes);}}),run=(key:string)=>observeInference({cachePath,key},()=>observedSourceAnalysisV1(provider,path));
 try{await run("one");const captured=await captureObservedSourceInputsV1(()=>run("two"));expect(analyses).toBe(1);expect(captured.value).toEqual(fixture.semantic.expected);expect(captured.observations.some(row=>row.kind==="file"&&row.path===configuration)).toBe(true);expect(captured.observations.some(row=>row.kind==="exists"&&row.path===missing&&row.fact===false)).toBe(true);const row=captured.observations.find(row=>row.kind==="analysis")!;expect(row.fact.nestedObservations).toHaveLength(2);expect(await observationsHold(captured.observations)).toBe(true);writeFileSync(configuration,fixture.semantic.nested.after);expect(await observationsHold(captured.observations)).toBe(false);await run("three");expect(analyses).toBe(2);}finally{rmSync(root,{recursive:true,force:true});}
},20000);

test("semantic source analysis refuses unknown producers and ambiguous revision declarations",async()=>{
 const root=workspace(),path=join(root,"source.ts"),id=fixture.semantic.provider.id+":identity",provider=registerSourceAnalysisV1({id,revision:fixture.semantic.provider.revision,analyze:semanticParser});writeFileSync(path,fixture.semantic.before);
 try{const captured=await captureObservedSourceInputsV1(()=>observedSourceAnalysisV1(provider,path)),rows=captured.observations.filter(row=>row.kind==="analysis");expect(await observationsHold(rows.map(row=>({...row,fact:{...row.fact,provider:id+":unknown"}})))).toBe(false);for(const vector of fixture.semantic.malformedFactPatches){const fact=(await import("fast-json-patch")).applyPatch(structuredClone(rows[0]!.fact),vector as any,true).newDocument,invalid=[{...rows[0],fact}],authority=JSON.parse(readFileSync(new URL("../../🧬️schema/📥️authority.json",import.meta.url),"utf8")),Envelope=(await import("ajv/dist/2020")).default;expect(new Envelope({strict:false}).compile(authority)({version:3,module:{implementation:"a".repeat(64),publication:"b".repeat(64)},membership:"a".repeat(64),graph:"b".repeat(64),observations:invalid})).toBe(true);expect(validate({digest:"a".repeat(64),resultSha256:"b".repeat(64),authorityObservations:[],observations:invalid})).toBe(false);expect(await observationsHold(invalid)).toBe(false);}expect(()=>registerSourceAnalysisV1({id,revision:fixture.semantic.provider.revision,analyze:()=>[]})).toThrow();registerSourceAnalysisV1({id,revision:fixture.semantic.provider.nextRevision,analyze:semanticParser});expect(await observationsHold(rows)).toBe(false);expect(()=>observedSourceAnalysisV1(provider,path)).toThrow();expect(validate({digest:"a".repeat(64),resultSha256:"b".repeat(64),authorityObservations:[],observations:rows.map(row=>({...row,fact:{...row.fact,revision:"invalid"}}))})).toBe(false);}finally{rmSync(root,{recursive:true,force:true});}
},20000);

test("semantic source analysis replays current parsers and fails closed on parser errors",async()=>{
 const root=workspace(),path=join(root,"source.ts");let calls=0;writeFileSync(path,fixture.semantic.before);
 const provider=registerSourceAnalysisV1({id:fixture.semantic.provider.id+":replay",revision:fixture.semantic.provider.revision,analyze:(source:string,bytes:Buffer)=>{calls++;if(bytes.toString("utf8")===fixture.semantic.importsChanged)throw Error(fixture.semantic.replayError);return semanticParser(source,bytes);}});
 try{const captured=await captureObservedSourceInputsV1(()=>observedSourceAnalysisV1(provider,path));expect(calls).toBe(1);expect(await observationsHold(captured.observations)).toBe(true);expect(calls).toBe(1);writeFileSync(path,fixture.semantic.bodyChanged);expect(await observationsHold(captured.observations)).toBe(true);expect(calls).toBe(2);writeFileSync(path,fixture.semantic.importsChanged);expect(await observationsHold(captured.observations)).toBe(false);expect(calls).toBe(3);let failed:any[]=[];await expect(captureObservedSourceInputsV1(()=>observedSourceAnalysisV1(provider,path),(rows:any[])=>{failed=rows;})).rejects.toThrow(fixture.semantic.replayError);const raw=failed.filter(row=>row.kind==="file"&&row.path===path);expect(raw).toHaveLength(1);expect(await observationsHold(raw)).toBe(true);writeFileSync(path,fixture.semantic.bodyChanged);expect(await observationsHold(raw)).toBe(false);expect(sourceObservationStatsV1().captureObservations).toBe(0);}finally{rmSync(root,{recursive:true,force:true});}
},20000);

test("semantic source replay cache bounds JSON images and isolates returned values",async()=>{
 const root=workspace(),path=join(root,"source.ts"),shared={imports:[...fixture.semantic.expected]};let small=0,large=0;writeFileSync(path,fixture.semantic.before);
 const provider=registerSourceAnalysisV1({id:fixture.semantic.provider.id+":isolation",revision:fixture.semantic.provider.revision,analyze:()=>{small++;return shared;}}),oversized=registerSourceAnalysisV1({id:fixture.semantic.provider.id+":oversized",revision:fixture.semantic.provider.revision,analyze:()=>{large++;return "x".repeat(fixture.semantic.oversizedCharacters);}});
 try{const first=observedSourceAnalysisV1(provider,path);if(typeof first!=="object"||first===null||!("imports" in first)||!Array.isArray(first.imports))throw Error("Original parser import image required");first.imports.push("./mutated.js");expect(observedSourceAnalysisV1(provider,path)).toEqual(shared);expect(small).toBe(1);for(let round=0;round<2;round++){const value=observedSourceAnalysisV1(oversized,path);if(typeof value!=="string")throw Error("Original parser string image required");expect(value.length).toBe(fixture.semantic.oversizedCharacters);}expect(large).toBe(2);expect(sourceObservationStatsV1().analysisBytes).toBeLessThanOrEqual(fixture.semantic.cacheMaximumBytes);}finally{rmSync(root,{recursive:true,force:true});}
},20000);

test("semantic source analysis trusts an identical stamp without reading and judges a moved stamp by the current parser; a source inside the racy window is never persisted",async()=>{
 const root=workspace(),path=join(root,"source.ts"),provider=registerSourceAnalysisV1({id:fixture.semantic.provider.id,revision:fixture.semantic.provider.revision,analyze:semanticParser});writeFileSync(path,fixture.semantic.before);
 try{const captured=await captureObservedSourceInputsV1(()=>observedSourceAnalysisV1(provider,path)),stamp=rawFs.statSync(path),original=rawFs.statSync,readOriginal=rawFs.readFileSync;let reads=0;writeFileSync(path,fixture.semantic.importsChanged);
  const stat=spyOn(rawFs,"statSync").mockImplementation(((file:any,...args:any[])=>String(file)===path?stamp:(original as any)(file,...args)) as typeof rawFs.statSync),read=spyOn(rawFs,"readFileSync").mockImplementation(((file:any,...args:any[])=>{if(String(file)===path)reads++;return (readOriginal as any)(file,...args);}) as typeof rawFs.readFileSync);
  try{expect(await observationsHold(captured.observations)).toBe(true);expect(reads).toBe(0);}finally{stat.mockRestore();read.mockRestore();}
  expect(await observationsHold(captured.observations)).toBe(false);
  const cachePath=join(root,"racy.json");let calls=0;writeFileSync(path,fixture.semantic.before);
  const run=()=>observeInference({cachePath,key:"racy"},()=>{calls++;return observedSourceAnalysisV1(provider,path);});
  await run();await run();expect(calls).toBe(2);expect(rawFs.existsSync(cachePath)).toBe(false);
  Bun.sleepSync(2200);await run();await run();expect(calls).toBe(3);expect(rawFs.existsSync(cachePath)).toBe(true);
 }finally{rmSync(root,{recursive:true,force:true});}
},30000);

test("semantic parent validation shares each provider independently across project images",async()=>{
 const root=workspace(),path=join(root,"source.ts"),command=registerSourceAnalysisV1({id:fixture.semantic.provider.id+":parent-command",revision:fixture.semantic.provider.revision,analyze:semanticParser}),module=registerSourceAnalysisV1({id:fixture.semantic.provider.id+":parent-module",revision:fixture.semantic.provider.revision,analyze:(source:string,bytes:Buffer)=>({imports:semanticParser(source,bytes)})});writeFileSync(path,fixture.semantic.before);Bun.sleepSync(2200);
 const projects=Array.from({length:fixture.semantic.parentValidation.projects},(_,index)=>()=>observeProjectInferenceV1({cachePath:join(root,`project-${index}.json`),key:index,owner:root},()=>({command:observedSourceAnalysisV1(command,path),module:observedSourceAnalysisV1(module,path)})));
 try{for(const project of projects)await project();let reads=0;const original=rawFs.readFileSync,mocked=spyOn(rawFs,"readFileSync").mockImplementation(((file:any,...args:any[])=>{if(String(file)===path)reads++;return (original as any)(file,...args);}) as typeof rawFs.readFileSync);
  try{const parent=async(key:string)=>{reads=0;return observeInference({cachePath:join(root,key+".json"),key},async()=>{const values=[];for(const project of projects)values.push(await project());return {values,validationReads:reads};});},oracle=ts.preProcessFile(fixture.semantic.before,true,true).importedFiles.map(item=>item.fileName),expected=Array.from({length:fixture.semantic.parentValidation.projects},()=>({command:oracle,module:{imports:oracle}}));
   const unchanged=await parent("provider-sharing-unchanged");expect(unchanged.values).toEqual(expected);expect(unchanged.validationReads).toBe(0);
   writeFileSync(path,fixture.semantic.bodyChanged);
   const result=await parent("provider-sharing");expect(result.values).toEqual(expected);expect(result.validationReads).toBe(fixture.semantic.parentValidation.expectedReads);}finally{mocked.mockRestore();}
 }finally{rmSync(root,{recursive:true,force:true});}
},20000);


test("observed TOML receives literal astral paths and refuses surrogate escapes with exact original source context",async()=>{
 const corpus=JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json",import.meta.url),"utf8")).tomlReceiving,parser=createRequire(import.meta.url)("smol-toml"),schema=JSON.parse(readFileSync(new URL("../../🧬️schema/📥️toml-failure.json",import.meta.url),"utf8")),admit=new Ajv({strict:true}).compile(schema),parent=process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),root=mkdtempSync(join(parent,"toml-")),path=join(root,"Cargo.toml"),plugin=await import("../../🕸️graph/📥️inference/🟨️.mjs"),api=await import("../🟨️.mjs");await plugin.libraryBootstrap;
 try{for(const row of corpus.cases){writeFileSync(path,row.source);let independent:any,builtin:any,error:any,receipt:any;try{independent=parser.parse(row.source);}catch{}try{builtin=Bun.TOML.parse(row.source);}catch{}try{receipt=await api.captureObservedSourceInputsV1(()=>plugin.cacheInternals.nativeDependencyRoots(".",root,false,new Map(),new Map()),undefined,false);}catch(value){error=value;}
  expect(Boolean(independent),row.name).toBe(row.smolAccepted);const bunDiverges=/\\U[0-9A-Fa-f]{8}|\\u[dD][89a-fA-F][0-9A-Fa-f]{2}/u.test(row.source);if(!bunDiverges)expect(Boolean(builtin),row.name).toBe(row.accepted);expect(error===undefined,row.name).toBe(row.accepted);
  if(row.accepted){expect(independent.lib.path).toBe(row.path);if(!bunDiverges)expect(builtin.lib.path).toBe(row.path);expect(receipt.observations.some((fact:any)=>fact.kind==="file"&&fact.path===path)).toBe(true);expect(await api.observationsHold(receipt.observations)).toBe(true);}else{expect(error.message).toContain("Observed TOML parse refused");expect(admit(error.tomlObservation)).toBe(true);expect(error.tomlObservation.path).toBe(path);expect(error.tomlObservation.sha256).toBe(createHash("sha256").update(row.source).digest("hex"));expect(error.tomlObservation.bytes).toBe(Buffer.byteLength(row.source));expect(error.tomlObservation.sourceLine).toBe(row.source.split("\n")[error.tomlObservation.line??0]);expect(error.cause.message).toMatch(/0xD800 - 0xDFFF|Unicode scalar value/u);}
 }
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("semantic parser handles retain exact evaluated inference ownership",async()=>{
 const row=JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json",import.meta.url),"utf8")).semantic.evaluations,root=workspace(),path=join(root,"source.ts"),owner=join(root,"owner.ts"),module=await import("../../🕸️graph/📥️inference/🟨️.mjs"),api=await import("../🟨️.mjs");await module.libraryBootstrap;
 try{
  writeFileSync(path,row.source);writeFileSync(join(root,"leaf.ts"),row.leaf);writeFileSync(owner,row.ownerBefore);
  const identities:string[]=[];for(const value of row.owners){const identity=module.cacheInternals.sourceAnalysisOwnerIdentityV1(value.url,value.revision),oracle=Buffer.from(await crypto.subtle.digest("SHA-256",Buffer.from(JSON.stringify([value.url,value.revision])))).toString("hex");expect(identity).toBe(oracle);expect(module.cacheInternals.sourceAnalysisOwnerIdentityV1(value.url,value.revision)).toBe(identity);identities.push(identity);}expect(new Set(identities).size).toBe(row.owners.length);
  const providers=row.owners.map((value:any,index:number)=>api.registerSourceAnalysisV1({id:"fixture:evaluated-imports:v1:"+identities[index],revision:value.revision,analyze:semanticParser}));for(const provider of providers){expect(provider.id.length).toBeLessThanOrEqual(row.providerMaximumCharacters);expect(Object.isFrozen(provider)).toBe(true);}
  const oracle=ts.preProcessFile(row.source,true,true).importedFiles.map(item=>item.fileName);expect(oracle).toEqual(row.expectedImports);
  const memo=new Map();let checkpoints=0;const checkpoint=async()=>{checkpoints++;await Promise.resolve();},run=(provider:any)=>api.captureObservedSourceInputsV1(()=>api.memoizedObservedAsyncComputationV1(memo,provider,async()=>{api.observedFs.readFileSync(owner);return api.observedSourceAnalysisV1(provider,path);},checkpoint),undefined,false);
  const first=await run(providers[0]),second=await run(providers[1]),again=await run(providers[0]);for(const [provider,value] of [[providers[0],first],[providers[1],second],[providers[0],again]]){expect(value.value).toEqual(oracle);expect(value.observations.some((fact:any)=>fact.kind==="file"&&fact.path===owner)).toBe(true);expect(value.observations.some((fact:any)=>fact.kind==="analysis"&&fact.path===path&&fact.fact.provider===provider.id)).toBe(true);expect(await api.observationsHold(value.observations)).toBe(true);}
  expect(checkpoints).toBeGreaterThan(0);expect(again.observations).toEqual(first.observations);expect(()=>api.observedSourceAnalysisV1({...providers[0]},path)).toThrow("Current registered source analysis provider required");writeFileSync(owner,row.ownerAfter);expect(await api.observationsHold(first.observations)).toBe(false);expect(await api.observationsHold(second.observations)).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("semantic Source observation declaration roster matches actual runtime exports",async()=>{
 const row=JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json",import.meta.url),"utf8")).semantic.declarationExports,source=ts.createSourceFile("observation.d.mts",readFileSync(new URL("../🟨️.d.mts",import.meta.url),"utf8"),ts.ScriptTarget.Latest,true),declared:string[]=[];
 for(const statement of source.statements){if(!ts.canHaveModifiers(statement)||!ts.getModifiers(statement)?.some(modifier=>modifier.kind===ts.SyntaxKind.ExportKeyword))continue;if(ts.isFunctionDeclaration(statement)&&statement.name)declared.push(statement.name.text);if(ts.isVariableStatement(statement))for(const declaration of statement.declarationList.declarations)if(ts.isIdentifier(declaration.name))declared.push(declaration.name.text);}
 expect(declared.sort()).toEqual(row);expect(Object.keys(await import("../🟨️.mjs")).sort()).toEqual(row);
});

for(const row of JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json",import.meta.url),"utf8")).semantic.settlement.cases)test("semantic settlement retains original cancellation: "+row.id,async()=>{
 const input=JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json",import.meta.url),"utf8")).semantic.settlement,api=await import("../🟨️.mjs"),parent=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!parent)throw Error("Original Source law artifact directory required");const root=mkdtempSync(join(parent,"settle-")),path=join(root,"source.txt"),controller=new AbortController();writeFileSync(path,input.before);let computations=0,rounds=0;
 try{const result=api.settleObservedComputationV1(async()=>{computations++;await Promise.resolve();if(row.phase==="compute"){controller.abort(Error(row.reason));return "completed";}api.observedFs.readFileSync(path);writeFileSync(path,input.after);api.observedFs.readFileSync(path);return "completed";},{signal:controller.signal,limit:1,onRound:async()=>{rounds++;await Promise.resolve();controller.abort(Error(row.reason));}});await expect(result).rejects.toThrow(row.reason);expect(computations).toBe(1);expect(rounds).toBe(row.phase==="round"?1:0);
 const oracle=Bun.spawnSync(["node","--input-type=module","-e",'const c=new AbortController();try{await Promise.resolve();c.abort(Error(process.argv[1]));c.signal.throwIfAborted();console.log("completed")}catch(e){console.log(e.message)}',row.reason],{stdout:"pipe",stderr:"pipe"});expect(oracle.exitCode).toBe(0);expect(oracle.stdout.toString().trim()).toBe(row.reason);
 }finally{rmSync(root,{recursive:true,force:true});}
});
