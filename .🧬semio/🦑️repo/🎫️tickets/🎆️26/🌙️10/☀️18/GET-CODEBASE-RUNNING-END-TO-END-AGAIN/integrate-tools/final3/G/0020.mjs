/** 🧭️ Tracks the literal import closure of command scripts and the packages they consume. */
import { advanceScriptInvocation } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { bindBuildCommandV1 } from "../../../../🏗️build/📤️arguments/🟨️.mjs";
import { checkResourceLeaseControl } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import { checkScriptInvocation, requireIssuedResourceControlInvocationV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { createRequire, isBuiltin } from "node:module";
import { createHash } from "node:crypto";
import { dirname, isAbsolute, join, relative, resolve, sep } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { memoizedObservedAsyncComputationV1, memoizedObservedComputationV1, observedSourceAnalysisV1, observedRequireResolve, observedTypeScriptHost, observedFs } from "../../../../🏗️build/👁️observation/🟨️.mjs";
import { readCargoExecutionContractV1, selectCargoExecutionOperationV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️execution/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
const { existsSync, statSync } = observedFs;

/** 🏷️ Names this component as its roster entry does. */
export const component = "script-inputs";
let buildPath, readPhysicalSource, prepareSourceAnalysesV1, LIBRARY_ROOT, POLICY, readToml, readBunLockGraph, SCRIPT_PROVIDER_LOADER_MODULE, SCRIPT_PROVIDER_DESIGNATION, SCRIPT_PROVIDER_DESIGNATION_SCHEMA;

/** 🔌️ Binds this component's dependencies from the revision registry once its needs are wired. */
export function wire(registry) {
  ({ buildPath, readPhysicalSource, LIBRARY_ROOT, POLICY, SCRIPT_PROVIDER_LOADER_MODULE, SCRIPT_PROVIDER_DESIGNATION, SCRIPT_PROVIDER_DESIGNATION_SCHEMA } = registry.coordinates);
  ({ prepareSourceAnalysesV1 } = registry.sourceAnalyses);
  ({ readToml } = registry.nativeManifests);
  ({ readBunLockGraph } = registry.bunLock);
}

/** 🗂️ Shares source reads and command closures only within one graph construction. */
export function createScriptInputCache(context={}) {
  if(context.invocation){requireIssuedResourceControlInvocationV1(context.phaseControl,context.invocation);checkResourceLeaseControl(context.phaseControl);}
  return { files: new Map(), closures: new Map(), commandDependencies: new Map(), modules:new Set(), resources:new Set(),generatedImports:new Map(),declared:new Map(),paths:new Map(),compiledNames:new WeakMap(),moduleSets:new WeakMap(),context,providerDesignation:context.scriptProviderDesignation };
}

/** 🔌️ Names the declared provider modules a script loads at run time, which no literal import reaches: the designation, its schema and every designated module. */
export function scriptProviderImportsV1(cache) {
  const designation = cache.providerDesignation ?? join(LIBRARY_ROOT, SCRIPT_PROVIDER_DESIGNATION), schema = join(LIBRARY_ROOT, SCRIPT_PROVIDER_DESIGNATION_SCHEMA);
  const declared = JSON.parse(readPhysicalSource(designation).toString("utf8"));
  if (validateJsonSchemaSubset(JSON.parse(readPhysicalSource(schema).toString("utf8")), declared).length) throw Error("Complete repository script provider declaration required");
  return [{ path: designation, source: false }, { path: schema, source: false }, ...Object.values(declared.providers).map(module => ({ path: resolve(dirname(designation), module), source: true }))];
}

/** 🧿️ Keeps a logical import's interpretation separate from its physical file identity. */
export function buildImportResourceV1(containing,specifier){
 if(!specifier.startsWith("."))return {specifier,suffix:"",source:true};
 const url=new URL(specifier,pathToFileURL(containing)),query=url.searchParams;
 return {specifier:fileURLToPath(url),suffix:url.search+url.hash,source:!query.has("raw")&&(!query.has("url")||query.has("worker")||query.has("sharedworker"))};
}

/** 📦️ Resolves a package specifier to the workspace module it names, so imports through `@scope/package` subpaths join a command closure like relative ones; installed registry packages stay covered by their locked external dependency. */
function workspacePackageModuleV1(containing, specifier, workspaceRoot) {
  if (specifier.startsWith(".") || isAbsolute(specifier) || isBuiltin(specifier) || /^[a-z][a-z0-9+.-]*:/u.test(specifier)) return undefined;
  let resolved;
  try { resolved = observedRequireResolve(containing, specifier); }
  catch (error) { if (error?.code === "MODULE_NOT_FOUND" || error?.code === "ERR_PACKAGE_PATH_NOT_EXPORTED") return undefined; throw error; }
  if (!isAbsolute(resolved)) return undefined;
  const local = relative(workspaceRoot, resolved);
  return local === "" || local.startsWith("..") || isAbsolute(local) || local.split(sep).includes("node_modules") ? undefined : resolved;
}

/** 🧪️ Names the literal module Source a target declares as an input (a test file it runs) so that file's import closure joins the target's closure. */
function declaredModuleInputV1(input, workspaceRoot, projectRoot) {
  if (typeof input !== "string" || input.startsWith("!") || /[*?{}[\]]/u.test(input.replace(/^\{(?:projectRoot|workspaceRoot)\}/u, "")) || !/\.[cm]?[jt]sx?$/u.test(input) || !/^\{(?:projectRoot|workspaceRoot)\}\//u.test(input)) return undefined;
  const path = resolve(workspaceRoot, input.replace("{projectRoot}", projectRoot).replace("{workspaceRoot}", "."));
  return !relative(workspaceRoot, path).startsWith("..") && existsSync(path) ? path : undefined;
}

/** 🔗️ Resolves a literal `new URL("…", import.meta.url)` reference to the workspace file it names, which is how a script points at the test files it runs and the data it reads. */
function workspaceUrlTargetV1(containing, literal, workspaceRoot) {
  let url;
  try { url = new URL(literal, pathToFileURL(containing)); } catch { return undefined; }
  if (url.protocol !== "file:") return undefined;
  const path = fileURLToPath(url), local = relative(workspaceRoot, path);
  if (local === "" || local.startsWith("..") || isAbsolute(local) || local.split(sep).includes("node_modules") || !existsSync(path) || !statSync(path).isFile()) return undefined;
  return { path, source: /\.[cm]?[jt]sx?$/u.test(path) };
}

/** 🔑️ Names a memo entry by the digest of its ordered parts, so a long path list is hashed once and never retained as a key string. */
function memoDigestV1(...parts) {
  const hash = createHash("sha256");
  for (const part of parts) {
    if (Array.isArray(part)) { hash.update("["); for (const item of part) hash.update(typeof item === "string" ? item : "\u0001").update("\0"); hash.update("]"); }
    else hash.update(String(part)).update("\0");
  }
  return hash.digest("hex");
}

/** 📛️ The workspace-relative name of one closure file, shared by every closure of one graph construction instead of re-created for each. */
function closurePathNameV1(cache, workspaceRoot, file) {
  let names = cache.paths.get(workspaceRoot);
  if (!names) cache.paths.set(workspaceRoot, names = new Map());
  let name = names.get(file);
  if (name === undefined) names.set(file, name = `{workspaceRoot}/${buildPath(relative(workspaceRoot, file))}`);
  return name;
}

/** 🧮️ Resolves the literal module inputs of one target list through one observed memo per distinct list: the command inputs that every generated Cargo target repeats verbatim are probed once, and every receiving project still replays the same existence facts. */
/** 🧮️ True when a target input names a module the entry closure already compiled: adding it as a closure entry changes nothing, and its existence is already implied by the file fact the closure read. */
function coveredModuleInputV1(input, projectRoot, covered) {
  if (typeof input !== "string") return false;
  if (input.startsWith("{workspaceRoot}/")) return covered.has(input);
  if (input.startsWith("{projectRoot}/")) return covered.has(projectRoot === "." ? `{workspaceRoot}/${input.slice(14)}` : `{workspaceRoot}/${projectRoot}/${input.slice(14)}`);
  return false;
}

function declaredModuleInputsV1(inputs, workspaceRoot, projectRoot, cache, covered) {
  if (!Array.isArray(inputs) || !inputs.length) return [];
  const pending = covered ? inputs.filter(input => !coveredModuleInputV1(input, projectRoot, covered)) : inputs;
  if (!pending.length) return [];
  return memoizedObservedComputationV1(cache.declared, memoDigestV1(workspaceRoot, projectRoot, pending), () => pending.flatMap(input => { const declared = declaredModuleInputV1(input, workspaceRoot, projectRoot); return declared ? [declared] : []; }));
}

/** 🏭️ The generated-output targets that the modules of one closure import: the absolute module set is derived once per distinct closure instead of once per target. */
function generatedTargetsV1(cache, closure, workspaceRoot) {
  let modules = cache.moduleSets.get(closure);
  if (!modules) cache.moduleSets.set(closure, modules = new Set(closure.map(path => resolve(workspaceRoot, path.slice("{workspaceRoot}/".length)))));
  return [...new Set([...cache.generatedImports].filter(([path]) => modules.has(path)).flatMap(([, rows]) => rows.map(row => row.target)))].sort();
}

/** 🧭️ Tracks literal relative command imports through the existing TypeScript tooling boundary. */
export async function relativeScriptInputs(entries, workspaceRoot, cache = createScriptInputCache(), requiredSources=false) {
  const roots = [...new Set(entries.map(entry => resolve(entry)))].sort(), key = memoDigestV1(resolve(workspaceRoot), roots, requiredSources);
  let steps=0;const checkpoint=(force=false)=>yieldInferenceV1(cache.context,"build:command-source-imports",++steps,force);
  return memoizedObservedAsyncComputationV1(cache.closures,key,async()=>{
  const compiler = createRequire(import.meta.url)("typescript"), host = observedTypeScriptHost(compiler.sys), files = new Set(),compiled=new Set();
  const visit = async(path,source=true) => {
    await checkpoint();
    if (closurePathNameV1(cache, workspaceRoot, path).startsWith("{workspaceRoot}/../")) throw new Error(`Command import escapes workspace: ${path}`);
    const seen=files.has(path);
    files.add(path);
    if(requiredSources){const admitted=cache.physical??=new Set();if(!admitted.has(path)){readPhysicalSource(path);admitted.add(path);}}
    else if (!existsSync(path)) return;
    if(!source||!/\.[cm]?[jt]sx?$/u.test(path)){cache.resources.add(path);if(!seen)readPhysicalSource(path);return;}
    if(compiled.has(path))return;
    compiled.add(path);cache.modules.add(path);
    const loadsProviders = path === join(LIBRARY_ROOT, SCRIPT_PROVIDER_LOADER_MODULE);
    const imports = await memoizedObservedAsyncComputationV1(cache.files,JSON.stringify([path,requiredSources,loadsProviders?cache.providerDesignation??null:undefined]),async()=>{
      const imports = loadsProviders ? scriptProviderImportsV1(cache) : [];
      for (const entry of observedSourceAnalysisV1(prepareSourceAnalysesV1().command,path)) {
        await checkpoint();
        const resource=buildImportResourceV1(path,entry);
        const produced=cache.generatedOutput?.(resource.specifier);if(produced){const rows=cache.generatedImports.get(path)??[];if(!rows.some(row=>row.target===produced.target&&row.output===produced.output))rows.push(produced);cache.generatedImports.set(path,rows);continue;}
        let resolved;
        try { resolved = observedRequireResolve(path, resource.specifier); }
        catch (error) {
          if (error.code !== "MODULE_NOT_FOUND") throw error;
          resolved = compiler.resolveModuleName(resource.specifier, path, { moduleResolution: compiler.ModuleResolutionKind.Bundler, allowJs: true, resolveJsonModule: true }, host).resolvedModule?.resolvedFileName;
          if (!resolved) {if(requiredSources||resource.suffix||!/\.[cm]?[jt]sx?$/u.test(resource.specifier))throw error;continue;}
          if (/\.d\.[cm]?ts$/.test(resolved)) throw error;
        }
        imports.push({path:resolved,source:resource.source});
      }
      const references = observedSourceAnalysisV1(prepareSourceAnalysesV1().references,path);
      for (const specifier of references.runtime) {
        await checkpoint();
        const member = workspacePackageModuleV1(path, specifier, workspaceRoot);
        if (member) imports.push({path:member,source:true});
      }
      for (const literal of references.urls) {
        await checkpoint();
        const target = workspaceUrlTargetV1(path, literal, workspaceRoot);
        if (target) imports.push(target);
      }
      return imports;
    },checkpoint);
    for (const imported of imports) await visit(imported.path,imported.source);
  };
  for (const entry of roots) await visit(entry);
  const result = [...files].map((file) => closurePathNameV1(cache, workspaceRoot, file)).sort();
  cache.compiledNames.set(result, new Set([...compiled].map((file) => closurePathNameV1(cache, workspaceRoot, file))));
  return result;
  },checkpoint);
}

/** 🔧️ Native leaves hash their compiler contract and exact command implementation, independently of UI selection. */
export async function nativeCommandInputs(workspaceRoot, scripts) {
  const script = join(LIBRARY_ROOT, "⚡️caching/🦀️cargo/📜️script.ts");
  const cargo = POLICY.toolchains.cargo, javascript = POLICY.toolchains.javascript;
  return [
    ...await relativeScriptInputs([script], workspaceRoot, scripts),
    ...[...javascript.files.filter((file) => !["package.json", "bun.lock"].includes(file)), ...cargo.files].map((file) => `{workspaceRoot}/${file}`),
    { externalDependencies: ["@iarna/toml"] },
    ...cargo.environment.map((env) => ({ env })),
    ...[...javascript.commands, ...cargo.commands].map((runtime) => ({ runtime })),
    { runtime: 'node -p "process.platform.concat(process.arch)"' },
  ];
}

/** 🧭️ Captures outer scripts and explicitly declared child scripts in their own working directories. */
export async function targetScriptSources(target, workspaceRoot, scripts, projectRoot = ".",nativeManifest) {
 const entries=new Set();let fallback=false;
 const retain=bound=>{if(bound.script){readPhysicalSource(bound.script);entries.add(bound.script);}else fallback=true;};
 const local=path=>{const value=buildPath(relative(workspaceRoot,path));if(value===".."||value.startsWith("../")||isAbsolute(value))throw Error("Declared command Source escapes workspace");return value||".";};
 for(const options of targetCommandRecipes(target)){
  const commands=typeof options.command==="string"?[options.command]:options.commands??[];if(!commands.length)fallback=true;
  for(const operation of commands){
   const command=typeof operation==="string"?operation:operation.command,arguments_=options.forwardAllArgs!==false&&(typeof operation==="string"||operation.forwardAllArgs!==false)?options.args??[]:[],bound=bindBuildCommandV1(command,workspaceRoot,options.cwd??".",projectRoot,arguments_);retain(bound);
   const owner=options.nativeOwnerCommand,selected=owner?bindBuildCommandV1([owner.program,...owner.arguments].map(value=>JSON.stringify(value)).join(" "),workspaceRoot,owner.workingDirectory,projectRoot,arguments_):bound;if(owner)retain(selected);
   const explicitManifest=owner?.manifest??nativeManifest,manifest=resolve(workspaceRoot,explicitManifest??join(projectRoot,"Cargo.toml"));local(manifest);if(explicitManifest===undefined&&!existsSync(manifest))continue;
   const cargo=readToml(manifest),reference=cargo.package?.metadata?.semio?.["cargo-execution"];if(reference===undefined)continue;if(typeof reference!=="string"||!reference||isAbsolute(reference))throw Error("Relative Cargo execution declaration required");
   const declarationPath=resolve(dirname(manifest),reference);local(declarationPath);readPhysicalSource(manifest);const declaration=readCargoExecutionContractV1("declaration",JSON.parse(readPhysicalSource(declarationPath).toString("utf8")));entries.add(manifest);entries.add(declarationPath);
   const selectedOperation=selectCargoExecutionOperationV1(declaration,{program:selected.program,cwd:owner?.workingDirectory??options.cwd??".",arguments:selected.arguments},workspaceRoot);if(!selectedOperation?.continuation)continue;
   const child=selectedOperation.continuation,childBound=bindBuildCommandV1([child.program,...child.arguments].map(value=>JSON.stringify(value)).join(" "),workspaceRoot,child.cwd,projectRoot,[]);if(!childBound.script)throw Error("Declared Cargo continuation requires an explicit script Source");retain(childBound);
  }
 }
 const baseSize=entries.size,base=baseSize?await relativeScriptInputs([...entries],workspaceRoot,scripts,true):undefined;
 for(const declared of declaredModuleInputsV1(target.inputs,workspaceRoot,projectRoot,scripts,base&&scripts.compiledNames.get(base)))entries.add(declared);
 const closure=entries.size===baseSize?base:entries.size?await relativeScriptInputs([...entries],workspaceRoot,scripts,true):undefined,generatedDependencies=scripts.generatedImports.size&&closure?generatedTargetsV1(scripts,closure,workspaceRoot):[];return {closure,fallback,...(generatedDependencies.length?{generatedDependencies}:{})};
}

/** ⚙️ Keeps configured recipe inheritance identical for command and lock source discovery. */
function targetCommandRecipes(target) {
  return [target.options ?? {}, ...Object.values(target.configurations ?? {}).map(options => ({ ...target.options, ...options }))];
}

/** 🗂️ Keeps configured executable imports in the target's invariant source closure. */
export async function targetScriptClosure(target, workspaceRoot, scripts, projectRoot = ".") {
  return (await targetScriptSources(target,workspaceRoot,scripts,projectRoot)).closure;
}

/** 🔐️ `cargo metadata --locked` validates the shared lock against every workspace manifest, so its replay must hash all of them. */
export function nativeLockInputs(command) {
  if(command && typeof command === "object")return [...new Set(targetCommandRecipes(command).flatMap(options=>(typeof options.command==="string"?[options.command]:options.commands??[]).flatMap(operation=>nativeLockInputs(typeof operation==="string"?operation:operation.command))))];
  return typeof command === "string" && command.includes(" native cargo metadata ") ? ["{workspaceRoot}/**/Cargo.toml", "{workspaceRoot}/**/Cargo.lock"] : [];
}

/** 🧭️ Adds the selected native target's local router and executable import closure. */
export async function nativeTargetCommandInputs(target, workspaceRoot, fallback, scripts, projectRoot = ".") {
  fallback??=await nativeCommandInputs(workspaceRoot,scripts);
  const closure = await targetScriptClosure(target, workspaceRoot, scripts, projectRoot);
  return closure ? [...new Set([...closure, ...fallback])] : fallback;
}

/** 🎯️ Hashes any script-backed target's own command closure, exactly, instead of the whole router. */
export async function genericTargetCommandInputs(target, workspaceRoot, fallback, scripts = createScriptInputCache(), projectRoot = ".") {
  const sources = await targetScriptSources(target, workspaceRoot, scripts, projectRoot),closure=sources.closure;
  if (!closure) return [...fallback, "{workspaceRoot}/package.json", "{workspaceRoot}/bun.lock"];
  const key=memoDigestV1(closure,sources.fallback?fallback:null);
  let index=0;const checkpoint=(force=false)=>yieldInferenceV1(scripts.context,"build:command-source-packages",++index,force);
  return memoizedObservedAsyncComputationV1(scripts.commandDependencies,key,async()=>{
  const packages = new Set(), graph = existsSync(join(workspaceRoot, "bun.lock")) ? readBunLockGraph(workspaceRoot) : undefined;
  for (const input of closure) {
    await checkpoint();
    const file = resolve(workspaceRoot, input.slice("{workspaceRoot}/".length));
    if (!existsSync(file)) continue;
    for (const specifier of observedSourceAnalysisV1(prepareSourceAnalysesV1().module,file).imports) {
      if (specifier.startsWith(".") || specifier.startsWith("/") || isBuiltin(specifier) || specifier.startsWith("bun:")) continue;
      const name = specifier.startsWith("@") ? specifier.split("/").slice(0, 2).join("/") : specifier.split("/")[0];
      if (!graph) { packages.add(name); continue; }
      const key = graph.resolveImport(buildPath(relative(workspaceRoot, file)), name);
      if (key && graph.externalNodes[`npm:${key}`]) packages.add(`npm:${key}`);
    }
  }
  const inputs = [...new Set([...closure,...(sources.fallback?[...fallback,"{workspaceRoot}/package.json","{workspaceRoot}/bun.lock"]:[])]), { externalDependencies: [...packages].sort() }];
  return inputs;
  },checkpoint);
}

/** 🪢️ The previous blanket router contract, kept only as a correctness fallback for a command naming no script. */
export function genericCommandFallbackInputs(workspaceRoot) {
  const runner = buildPath(relative(workspaceRoot, LIBRARY_ROOT));
  return [
    "{workspaceRoot}/📜️script.ts",
    `{workspaceRoot}/${runner}/**/*.{ts,tsx,js,mjs,cjs,json}`,
    `{workspaceRoot}/${runner}/🟨️.mjs`,
    `{workspaceRoot}/${runner}/⚡️caching/**/*`,
  ];
}

/** ⏳️ Delegates bounded inference work to the caller's original progress and continuation control; durable progress is published at most every 200 ms so its cost never scales with the number of inferred facts. */
export async function yieldInferenceV1(context,phase,index,force=false){
 if(!context.invocation)return;
 const last=yieldInferenceV1.last??=new WeakMap(),now=performance.now();
 if(!force&&now-(last.get(context)??-Infinity)<200){context.invocation.control.signal.throwIfAborted();return;}
 last.set(context,now);
 requireIssuedResourceControlInvocationV1(context.phaseControl,context.invocation);checkScriptInvocation(context.invocation);checkResourceLeaseControl(context.phaseControl);await advanceScriptInvocation(context.invocation,phase,"running");checkResourceLeaseControl(context.phaseControl);
}
