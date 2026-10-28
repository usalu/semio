import {countedCompilerCapacityRequestV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { scriptInvocationEnvironment, requireIssuedResourceControlInvocationV1,requireIssuedResourceControlV1, createRequestedIssuedResourceControlV1, type ScriptInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import { validateJsonSchemaSubset } from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { mkdir, mkdtemp, readFile, lstat, realpath, copyFile, writeFile } from "node:fs/promises";
import type { BunBundleRequest, BunBundleResult } from "./🟦️bun/🟦️.ts";
import { availableParallelism } from "node:os";
import { resolveLanguageV1 } from "../../../../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";
import { CATALOG_LANGUAGES_V1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import { describeCompileUnitsV1, withCompilerCapacityScopeV1, fixedCompilerCapacityRequestV1, scriptCompilerAdmissionControlV1, compilerAdmissionBudgetV1, type CompilerAdmissionControl } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { createRequire } from "node:module";
import { join, dirname, relative, resolve, isAbsolute } from "node:path";
import { createReadStream, lstatSync, openSync, writeSync, closeSync, constants } from "node:fs";
import { createHash } from "node:crypto";
import workerSchema from "./🟦️bun/🧬️schema/🔣️.json" with { type: "json" };
import { runOwnedCommand } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { buildModeEnvironment, buildModePolicy, type BuildMode } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import { runtimeBuiltinNamesV1 } from "../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";

import{withArtifactScratchV1,withArtifactDestinationWriterV1,withArtifactDestinationTaskV1,type ArtifactDestinationV1,type ArtifactDestinationContextV1}from"../📦️artifacts/💾️storage/🟦️.ts";
import{createBunBundleStreamV1}from"./🟦️bun/📤️stream/🟦️.ts";
import{readNativeCallerAuthorityV1}from"../📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import{withRepositoryCacheStoreV1}from"../💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";

/** 📦️ Describes a cancellable JavaScript artifact without exposing a compiler SDK. */
export interface JavaScriptBundle { readonly workspace: string; readonly source: string; readonly destination: ArtifactDestinationV1; readonly mode: BuildMode; readonly platform: "node" | "browser"; readonly signal: AbortSignal; readonly environment: Readonly<Record<string, string | undefined>>; readonly external?: readonly string[]; readonly runtime?: "javascript" | "runtime"; readonly invocation: ScriptInvocation; readonly capacity?: Readonly<{ parallelism: number }>; }

/** 🏗️ Starts esbuild after admission so its Go workers honor the inherited compiler budget. */
export function bundleJavaScript(options: JavaScriptBundle): Promise<void> {
 return withArtifactDestinationWriterV1(options.destination,options.invocation,async destination=>{
  const failures=validateJsonSchemaSubset(schema,Object.fromEntries(Object.entries(options).filter(([key,value])=>!["signal","environment","invocation","destination","capacity"].includes(key)&&value!==undefined)));
  if(failures.length)throw Error("Invalid JavaScript bundle: "+failures.join("; "));if(options.workspace!==destination.workspace||options.mode!==destination.mode)throw Error("Original bundle repository or mode disagrees with destination");
  const source=resolve(options.source),distance=relative(options.workspace,source),output=destination.output;if(source.length>256||output.length>252||isAbsolute(distance)||distance===".."||distance.startsWith("../")||distance.startsWith("..\\")||source===output||!relative(destination.directory,source).startsWith(".."))throw Error("Bundle Source and output custody overlap or escape");
  for(let path=source;;path=dirname(path)){const status=lstatSync(path);if(status.isSymbolicLink()||path===source&&(!status.isFile()||status.nlink!==1))throw Error("Actual physical bundle Source required");if(dirname(path)===path)break;}
  const control=createRequestedIssuedResourceControlV1(destination.control,120000,options.signal),environment={...options.environment,...destination.environment};
  if(options.runtime==="runtime"){
   await withBunBundleFiles({version:1,kind:"bun-package",mode:options.mode,entrypoint:source},{workspace:options.workspace,invocation:options.invocation,control,capacity:options.capacity,signal:control.signal,environment},destination,async files=>{
    if(files.sourceFile.byteLength+(files.sourceMapFile?.byteLength??0)>536870912)throw Error("Bundle output byte capacity exceeded");control.signal.throwIfAborted();await copyFile(files.source,output);await verifyBunFile(output,files.sourceFile,control.signal);if(files.sourceMap){await copyFile(files.sourceMap,output+".map");await verifyBunFile(output+".map",files.sourceMapFile!,control.signal);}
   });return;
  }
  const require=createRequire(join(options.workspace,"package.json")),policy=buildModePolicy(options.mode),args=[require.resolve("esbuild/bin/esbuild"),source,"--bundle",`--tree-shaking=${policy.optimize}`,"--format=esm","--target=es2022",`--platform=${options.platform}`,`--outfile=${output}`,...(policy.optimize?["--minify"]:[]),...(policy.sourceMap?["--sourcemap=external"]:[]),...[...runtimeBuiltinNamesV1(),...(options.external??[])].map(value=>`--external:${value}`)];
  const compile=async(lease:Readonly<{parallelism:number}>,control:CompilerAdmissionControl)=>{
  await runOwnedCommand(process.execPath,args,options.workspace,"javascript:esbuild",compilerAdmissionBudgetV1(control),{env:{...environment,SEMIO_COMPILER_PARALLELISM:String(lease.parallelism),GOMAXPROCS:String(lease.parallelism)},signal:control.signal});const bytes=(await lstat(output)).size+(policy.sourceMap?(await lstat(output+".map")).size:0);if(bytes>536870912)throw Error("Bundle output byte capacity exceeded");};
  if(options.capacity)await compile(options.capacity,control);else await withCompilerCapacityScopeV1({request:countedCompilerCapacityRequestV1(),directory:join(options.workspace,".cache/b/l/m/data"),control,environment},compile);
 });
}

/** 🤲️ Runs the Bun bundler under the counted slot its caller already holds, and requests the fixed `bun-bundle` slot itself otherwise; a caller that holds scratch or dependency leases must carry its slot, so no capacity is ever requested beneath them.
 * @see ../../../../../../🔨️modules/🏃️process/🔒️leases/🧭️order/🔣️policy.json */
export async function withCarriedOrCountedBunCapacity<T>(directory:string,control:CompilerAdmissionControl,carried:Readonly<{parallelism:number}>|undefined,operation:(environment:Readonly<Record<string,string|undefined>>,control:CompilerAdmissionControl)=>Promise<T>,environment:Readonly<Record<string,string|undefined>>=process.env):Promise<T>{
 if(carried===undefined)return withCountedBunCapacity(directory,control,operation,environment);
 const request=fixedCompilerCapacityRequestV1("bun-bundle");if(request.kind!=="available"||carried.parallelism<(request.minimumParallelism??1))throw Error("Bun bundler requires its counted slot");
 return operation(Object.freeze(bunSlotEnvironment(environment,carried.parallelism)),control);
}

/** 📏️ The environment of a Bun bundler child bounded to its counted slot: Bun ignores `GOMAXPROCS`, so the libuv pool is bounded as well and the slot is queued like any compile. */
function bunSlotEnvironment(environment:Readonly<Record<string,string|undefined>>,parallelism:number):Record<string,string|undefined>{
 const child:Record<string,string|undefined>={...environment,SEMIO_COMPILER_PARALLELISM:String(parallelism),GOMAXPROCS:String(parallelism),UV_THREADPOOL_SIZE:String(parallelism)};delete child.SEMIO_COMPILER_CAPACITY_HELD;return child;
}

/** 🎟️ Admits the Bun bundler (its own worker pool ignores `GOMAXPROCS`) into the fixed counted `bun-bundle` slot of the compile units policy instead of the exclusive whole host; Go-based tools such as esbuild run under the planned units. */
export function withCountedBunCapacity<T>(directory:string,control:CompilerAdmissionControl,operation:(environment:Readonly<Record<string,string|undefined>>,control:CompilerAdmissionControl)=>Promise<T>,environment:Readonly<Record<string,string|undefined>>=process.env):Promise<T>{
 requireIssuedResourceControlV1(control);const capturedEnvironment=Object.freeze({...environment});const language=resolveLanguageV1(CATALOG_LANGUAGES_V1,[],capturedEnvironment).language,started=Date.now(),cores=availableParallelism(),report=(name:"granted",values:Record<string,string|number>)=>process.stderr.write(describeCompileUnitsV1(language,name,{operation:"bun bundle",cores,...values})+"\n");
 return withCompilerCapacityScopeV1({request:fixedCompilerCapacityRequestV1("bun-bundle"),directory,control,environment:capturedEnvironment},async(facts,actualControl)=>{report("granted",{granted:facts.parallelism,seconds:Math.round((Date.now()-started)/1000)});return operation(Object.freeze(bunSlotEnvironment(capturedEnvironment,facts.parallelism)),actualControl);});
}

type BunCompileOptions = Readonly<{ workspace: string; invocation: ScriptInvocation; control: CompilerAdmissionControl; capacity?: Readonly<{ parallelism: number }>; signal?: AbortSignal; environment?: Readonly<Record<string, string | undefined>>; onLine?: (line: string) => void }>;
type BunBundleFiles = Readonly<{ source: string; sourceMap?: string; metadata: Omit<BunBundleResult, "source" | "sourceMap">; sourceFile: BunFile; sourceMapFile?: BunFile }>;
type BunFile = Readonly<{ path: string; sha256: string; byteLength: number }>;

/** 🔏️ Checks real file bytes and stable identity without creating a bulk IPC representation. */
async function verifyBunFile(path: string, expected: Readonly<{ sha256: string; byteLength: number }>, signal: AbortSignal): Promise<void> {
  signal.throwIfAborted(); const before = await lstat(path);
  if (!before.isFile() || before.size !== expected.byteLength) throw Error("Bun compiler file custody refused");
  const hash = createHash("sha256"); let size = 0;
  for await (const chunk of createReadStream(path, { signal })) { hash.update(chunk); size += chunk.byteLength; }
  const after = await lstat(path); signal.throwIfAborted();
  if (size !== expected.byteLength || hash.digest("hex") !== expected.sha256 || ["dev", "ino", "size", "mtimeMs", "ctimeMs"].some(key => before[key as keyof typeof before] !== after[key as keyof typeof after])) throw Error("Bun compiler file bytes changed or digest refused");
}

/** 🗂️ Admits one bounded worker manifest and proves both captured sources and produced files before promotion. */
export async function readBunBundleFiles(request: BunBundleRequest, response: string, signal: AbortSignal): Promise<BunBundleFiles> {
  const stat = await lstat(response); if (!stat.isFile() || stat.size > 4096 || resolve(response).length > 256) throw Error("Bun compiler manifest custody refused");
  const manifest = JSON.parse(await readFile(response, { encoding: "utf8", signal }));
  if (validateJsonSchemaSubset(workerSchema.definitions.Response, manifest).length || manifest.mode !== request.mode || manifest.kind !== request.kind || manifest.requestSha256 !== createHash("sha256").update(JSON.stringify(request)).digest("hex")) throw Error("Bun compiler manifest request binding refused");
  const root = await realpath(dirname(response)), owner = dirname(manifest.source.path);
  const admit = async (record: BunFile, name: string): Promise<string> => {
    const path = resolve(root, record.path), child = relative(root, path);
    if (isAbsolute(record.path) || child.startsWith("..") || path.length > 256 || record.path !== `${owner}/${name}` || !owner.startsWith(`bundle-${request.mode}-`) || await realpath(path) !== path) throw Error("Bun compiler output ownership refused");
    await verifyBunFile(path, record, signal); return path;
  };
  const source = await admit(manifest.source, "source.js"), sourceMap = manifest.sourceMap ? await admit(manifest.sourceMap, "source.js.map") : undefined, metadataPath = await admit(manifest.metadata, "metadata.json"), metadata = JSON.parse(await readFile(metadataPath, { encoding: "utf8", signal }));
  if (validateJsonSchemaSubset(workerSchema.definitions.Metadata, metadata).length || metadata.inputs.length !== metadata.physicalInputs.length) throw Error("Bun compiler source metadata refused");
  const paths = new Set<string>();
  for (let index = 0; index < metadata.physicalInputs.length; index++) {
    const input = metadata.physicalInputs[index], { path, ...logical } = input;
    if (paths.has(path) || await realpath(path) !== path || JSON.stringify(logical) !== JSON.stringify(metadata.inputs[index])) throw Error("Bun compiler source coverage refused");
    paths.add(path); await verifyBunFile(path, input, signal);
  }
  signal.throwIfAborted(); return { source, sourceFile: manifest.source, ...(sourceMap ? { sourceMap, sourceMapFile: manifest.sourceMap } : {}), metadata };
}

async function withBunBundleFiles<T>(request:BunBundleRequest,options:BunCompileOptions,destination:ArtifactDestinationContextV1,consume:(files:BunBundleFiles)=>Promise<T>):Promise<T>{
 const control=createRequestedIssuedResourceControlV1(destination.control,120000,options.signal),input=JSON.stringify(request),requestSha256=createHash("sha256").update(input).digest("hex"),authority=readNativeCallerAuthorityV1(options.invocation.capabilities);if(Buffer.byteLength(input)>16*1024*1024)throw Error("Bun compiler request bytes refused");const temporary=await mkdtemp(join(destination.directory,`bundle-${request.mode}-`)),output=join(destination.directory,"result.json");let file:number|undefined;const stream=createBunBundleStreamV1(requestSha256,event=>{control.signal.throwIfAborted();if(event.type==="file"){const path=join(temporary,event.name);if(path.length>256)throw Error("Bun sink path capacity exceeded");file=openSync(path,constants.O_WRONLY|constants.O_CREAT|constants.O_EXCL|(constants.O_NOFOLLOW??0),0o600);}else if(event.type==="chunk"){if(file===undefined)throw Error("Live Bun sink file required");let offset=0;while(offset<event.bytes.byteLength){const written=writeSync(file,event.bytes,offset,event.bytes.byteLength-offset);if(written<=0)throw Error("Bun sink write made no progress");offset+=written;}}else{if(file===undefined)throw Error("Live Bun sink close required");closeSync(file);file=undefined;}});
 return withCarriedOrCountedBunCapacity(join(options.workspace,".cache/b/l/m/data"),control,options.capacity,async(environment,control)=>{try{await runOwnedCommand(process.execPath,[join(import.meta.dirname,"🟦️bun/📜️script.ts"),"compile"],options.workspace,"javascript:bun-worker",compilerAdmissionBudgetV1(control),{input,signal:control.signal,stdout:"ignore",output:{maximumBytes:Math.min(738197504,authority.transport.maximumBytes),maximumLines:authority.transport.maximumLines},outputConsumer:(bytes,source)=>{if(source==="stdout")stream.push(bytes);},env:scriptInvocationEnvironment(options.invocation,buildModeEnvironment(request.mode,{...destination.environment,...environment,SEMIO_COMPILER_CAPACITY_HELD:undefined,SEMIO_BUILD_PROFILE_INPUTS:"0"})),onLine:(line,source)=>{if(source==="stderr")options.onLine?.(line);}});const records=stream.finish(),record=(name:string)=>{const value=records.find(row=>row.name===name);if(!value)throw Error("Complete Bun sink record required");return{path:relative(destination.directory,join(temporary,name)).split(/[\\/]/).join("/"),sha256:value.sha256,byteLength:value.byteLength};},manifest={version:1,requestSha256,kind:request.kind,mode:request.mode,source:record("source.js"),...(records.some(row=>row.name==="source.js.map")?{sourceMap:record("source.js.map")}:{ }),metadata:record("metadata.json")};if(Buffer.byteLength(JSON.stringify(manifest))>4096)throw Error("Bun sink manifest capacity exceeded");await writeFile(output,JSON.stringify(manifest),{flag:"wx",signal:control.signal});const files=await readBunBundleFiles(request,output,control.signal);compilerAdmissionBudgetV1(control);const result=await consume(files);control.signal.throwIfAborted();compilerAdmissionBudgetV1(control);return result;}finally{if(file!==undefined)closeSync(file);}},{...options.environment,...destination.environment});
}

/** 🧵️ Returns bounded bytes only after the actual diagnostics-owned worker and file consumer settle. */
export async function bundleVirtualJavaScript(request:BunBundleRequest,options:BunCompileOptions):Promise<BunBundleResult>{
 requireIssuedResourceControlInvocationV1(options.control,options.invocation);const authority=readNativeCallerAuthorityV1(options.invocation.capabilities);if(options.workspace!==authority.repositoryRoot||request.mode!==authority.original.buildMode)throw Error("Original bundle workspace or mode disagrees");if(options.capacity===undefined)return withCountedBunCapacity(join(options.workspace,".cache/b/l/m/data"),options.control,(slot,control)=>bundleVirtualJavaScript(request,{...options,control,capacity:{parallelism:Number(slot.SEMIO_COMPILER_PARALLELISM)}}));const phaseControl=createRequestedIssuedResourceControlV1(options.control,120000,options.signal);return withRepositoryCacheStoreV1(options.workspace,"diagnostics",options.invocation,phaseControl,(store,storePhase)=>withArtifactScratchV1({workspace:options.workspace,invocation:options.invocation,storage:store.storage,control:store.control,phaseControl:storePhase,request:{version:1,mode:request.mode,owner:"bun-worker/"+createHash("sha256").update("entrypoint" in request?request.entrypoint:request.kind).digest("hex")}},token=>withArtifactDestinationTaskV1(token,options.invocation,destination=>withBunBundleFiles(request,options,destination,async files=>({ ...files.metadata,source:await readFile(files.source,{encoding:"utf8",signal:destination.control.signal}),...(files.sourceMap?{sourceMap:await readFile(files.sourceMap,{encoding:"utf8",signal:destination.control.signal})}:{})})))));
}
