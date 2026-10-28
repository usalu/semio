/** 🦀️ Resolves Cargo and Go manifests, native dependency closures, mounted Rust Source and native generator preparation. */
import { createHash } from "node:crypto";
import { dirname, isAbsolute, sep, join, relative, resolve } from "node:path";
import { memoizedObservedComputationV1, memoizedFileRecord, observedSourceAnalysisV1, observedFs } from "../../../../🏗️build/👁️observation/🟨️.mjs";
import { runtimeParseTomlV1 } from "../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/📄️parse/🟦️.ts";
import {runtimeKeyV1} from "../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";
import {refuseBuildV1} from "../../../📖️texts/🟦️.ts";
const { existsSync, readdirSync } = observedFs;

/** 🏷️ Names this component as its roster entry does. */
export const component = "native-manifests";
let prepareSourceAnalysesV1, RUST_SOURCE_CACHE, RUST_INPUT_SNAPSHOTS, buildPath, targetExecutionKindV1, executionTargetV1, SCRIPT_BASENAME;

/** 🔌️ Binds this component's dependencies from the revision registry once its needs are wired. */
export function wire(registry) {
  ({ prepareSourceAnalysesV1, RUST_SOURCE_CACHE, RUST_INPUT_SNAPSHOTS } = registry.sourceAnalyses);
  ({ buildPath, SCRIPT_BASENAME } = registry.coordinates);
  ({ targetExecutionKindV1, executionTargetV1 } = registry.targetPolicy);
}

/** ♻️ Content-addressed discovery never trusts source timestamps or retains token streams. */
function cachedRustReferences(file, cache, snapshots) {
  return memoizedObservedComputationV1(snapshots,file,()=>{
  const references = observedSourceAnalysisV1(prepareSourceAnalysesV1().rust,file);
  if(cache!==RUST_SOURCE_CACHE){
    const digest=createHash("sha256").update(JSON.stringify(references)).digest("hex"),previous=cache.entries.get(digest);
    if(previous){cache.hits++;cache.entries.delete(digest);cache.entries.set(digest,previous);}
    else{cache.misses++;const bytes=Buffer.byteLength(JSON.stringify(references))+192;if(bytes<=cache.limit){while(cache.bytes+bytes>cache.limit){const oldest=cache.entries.keys().next().value;cache.bytes-=cache.entries.get(oldest).bytes;cache.entries.delete(oldest);}cache.entries.set(digest,{references,bytes});cache.bytes+=bytes;}}
  }
  return references;
  });
}

/** 📥️ Resolves Cargo entry points and compact source references without running a compiler; a `compiled` closure is what a compilation without `cfg(test)` reads, so the mounts and includes that only exist under `#[cfg(test)]` stay out of it. */
export function rustSourceFiles(entries, manifestRoot, cache = RUST_SOURCE_CACHE, snapshots = new Map(), compiled = false) {
  let closures = RUST_INPUT_SNAPSHOTS.get(snapshots);
  if (!closures) { closures = new Map(); RUST_INPUT_SNAPSHOTS.set(snapshots, closures); }
  const key = JSON.stringify([manifestRoot, entries.map(entry => resolve(entry)), compiled]);
  return [...memoizedObservedComputationV1(closures,key,()=>{
  const files = new Set(), visited = new Set();
  const literal = (value) => typeof value === "string" ? value : Array.isArray(value) ? value.map(literal).join("") : manifestRoot;
  const visit = (file, moduleBase = dirname(file)) => {
    files.add(file);
    const identity = file + "\0" + moduleBase;
    if (!existsSync(file) || visited.has(identity)) return;
    visited.add(identity);
    if (!file.endsWith(".rs")) return;
    const references = cachedRustReferences(file, cache, snapshots);
    if (references.error) throw new Error(references.error + ": " + file);
    if (compiled && references.fileTest) return;
    const skipped = compiled ? new Set(references.testIncludes ?? []) : new Set();
    references.includes.forEach((expression, index) => { if (!skipped.has(index)) visit(resolve(dirname(file), literal(expression))); });
    const scope = (modules, base, fileScope) => {
      for (const { name, path, test, modules: children } of modules) {
        if (compiled && test) continue;
        if (children) { scope(children, resolve(base, path ?? name), false); continue; }
        const candidates = path === undefined ? [join(base, name + ".rs"), join(base, name, "mod.rs")] : [resolve(fileScope ? dirname(file) : base, path)];
        if (!candidates.some(existsSync)) for (const child of candidates) files.add(child);
        for (const child of candidates) if (existsSync(child)) visit(child, path === undefined && buildPath(child).endsWith("/" + name + ".rs") ? join(dirname(child), name) : dirname(child));
      }
    };
    scope(references.modules, moduleBase, true);
  };
  for (const entry of entries) visit(resolve(entry));
  const result = [...files].sort();
  return result;
  })];
}

const RUST_REFUSALS = [["Generated Rust modules", "macro-mount"], ["Conditional Rust mount", "conditional-mount"], ["Dynamic Rust include", "dynamic-include"]];

/** 🧾️ The Rust source set of one crate as the `rust-source-set` fact: the files a compilation of the crate reads (and the files that emit `include*!` into their consumers, whose reads no static reading of the crate can see), or the reason no static reading exists (a build script, a macro or conditional mount, a dynamic include, no entry point, an unparsable file).
 * @see ./🧬️schema/🔣️.json */
export function cargoSourceSetV1(root, workspaceRoot, facts, includeTests = true, compiled = false) {
  if (facts === null || typeof facts !== "object") return cargoSourceSetOfV1(root, workspaceRoot, facts, includeTests, compiled);
  let memo = CARGO_SOURCE_SETS_V1.get(facts);
  if (memo === undefined) CARGO_SOURCE_SETS_V1.set(facts, memo = new Map());
  return memoizedObservedComputationV1(memo, JSON.stringify([root, workspaceRoot, includeTests, compiled]), () => cargoSourceSetOfV1(root, workspaceRoot, facts, includeTests, compiled));
}

/** 🧠️ The answers of {@link cargoSourceSetV1} per fact table (one per project): the targets of a project ask for the same crates again and again, and a replayed answer hands every asker the rows it rests on without probing the tree again. */
const CARGO_SOURCE_SETS_V1 = new WeakMap();

function cargoSourceSetOfV1(root, workspaceRoot, facts, includeTests, compiled) {
  const directory = join(workspaceRoot, root), manifest = join(directory, "Cargo.toml"), unresolved = (reason, message) => ({ version: 1, root, resolved: false, reason, ...(message ? { message } : {}) });
  if (!existsSync(manifest)) return unresolved("no-manifest");
  const cargo = readToml(manifest), entries = [cargo.lib?.path ?? "src/lib.rs", "src/main.rs", ...(cargo.bin ?? []).map((target) => target.path), ...(includeTests ? [...(cargo.test ?? []), ...(cargo.bench ?? []), ...(cargo.example ?? [])].map((target) => target.path) : []), ...(cargo.package?.build === false ? [] : [typeof cargo.package?.build === "string" ? cargo.package.build : "build.rs"])].filter(Boolean).map((path) => resolve(directory, path)).filter(existsSync);
  if (cargo.package?.build !== false && existsSync(resolve(directory, typeof cargo.package?.build === "string" ? cargo.package.build : "build.rs"))) return unresolved("build-script");
  for (const source of ["src/bin", ...(includeTests ? ["tests", "examples", "benches"] : [])]) {
    if (!existsSync(join(directory, source))) continue;
    for (const entry of readdirSync(join(directory, source), { withFileTypes: true })) {
      const path = join(directory, source, entry.name, ...(entry.isDirectory() ? ["main.rs"] : []));
      if (path.endsWith(".rs") && existsSync(path)) entries.push(path);
    }
  }
  if (!entries.length) return unresolved("no-entries");
  try {
    const read = rustSourceFiles(entries, directory, RUST_SOURCE_CACHE, facts, compiled), place = (file) => `{workspaceRoot}/${buildPath(relative(workspaceRoot, file))}`;
    const emitters = read.filter((file) => file.endsWith(".rs") && existsSync(file) && cachedRustReferences(file, RUST_SOURCE_CACHE, facts).emitted > 0).map(place);
    return { version: 1, root, resolved: true, files: read.map(place), ...(emitters.length ? { emitters } : {}) };
  }
  catch (error) { const message = String(error?.message ?? error); return unresolved(RUST_REFUSALS.find(([prefix]) => message.startsWith(prefix))?.[1] ?? "unparsable", message.split(workspaceRoot + sep).join("")); }
}

/** 🧭️ Cargo owns compilation while the build graph tracks mounted sources and embedded assets; a crate without a static reading has no source list and keeps its directory and owner tree. */
export function cargoSourceInputs(root, workspaceRoot, facts, includeTests = true, compiled = false) {
  const set = cargoSourceSetV1(root, workspaceRoot, facts, includeTests, compiled);
  return set.resolved ? [...set.files] : undefined;
}

/** 🛑 The source list an input precision rule may derive a key from: a crate that has no compiled entry point contributes none, a crate whose mounts cannot be read statically refuses, because its directory alone misses the files a `#[path]` mount reaches outside of it (a stale cache hit after an edit of a mounted owner file).
 * @see ./🧬️schema/🔣️.json */
export function cargoSourcesOrRefuseV1(root, workspaceRoot, facts, includeTests = true, compiled = false) {
  const set = cargoSourceSetV1(root, workspaceRoot, facts, includeTests, compiled);
  if (set.resolved) return [...set.files];
  if (set.reason === "no-entries" || set.reason === "no-manifest") return undefined;
  throw Object.assign(refuseBuildV1("refused", "graph.rust-sources-unresolved", { root, reason: set.message ? set.reason + ": " + set.message : set.reason }), { set });
}

let tomlParsing;

/** 📏️ The tooling TOML boundary exposes only repository-owned plain records, memoized per parser version (resolved on first use). */
export function readToml(path) {
  tomlParsing ??= { namespace: `toml-parse-v1:${runtimeKeyV1()}`, parse: (source) => runtimeParseTomlV1(source) };
  return memoizedFileRecord(tomlParsing.namespace, path, (bytes) => {
    const source=bytes.toString("utf8");
    try{return tomlParsing.parse(source);}catch(cause){
      const where=cause?.position??cause?.errors?.[0]?.position,integer=value=>Number.isInteger(value)&&value>=0?value:null,line=integer(where?.line-1),tomlObservation={version:1,path:resolve(path),namespace:tomlParsing.namespace,sha256:createHash("sha256").update(bytes).digest("hex"),bytes:bytes.byteLength,line,column:integer(where?.column-1),position:integer(where?.offset),sourceLine:(source.split(/\r?\n/u)[line??0]??"").slice(0,512)};
      const error=Object.assign(new Error("Observed TOML parse refused: "+JSON.stringify(tomlObservation),{cause}),{code:"BUILD_SOURCE_UNPARSEABLE",source:tomlObservation.path,sha256:tomlObservation.sha256});error.tomlObservation=tomlObservation;throw error;
    }
  });
}

/** 🔎️ Resolves path dependencies, including aliases, target tables and workspace inheritance. */
export function nativeDependencies(manifest, workspace) {
  const result = [];
  for (const table of [manifest, ...Object.values(manifest.target ?? {})]) {
    for (const kind of ["dependencies", "build-dependencies", "dev-dependencies"]) {
      for (const [alias, declaration] of Object.entries(table[kind] ?? {})) {
        if (typeof declaration !== "object") continue;
        const inherited = declaration.workspace === true;
        const dependency = inherited ? workspace.dependencies?.[alias] : declaration;
        if (dependency?.path) result.push({ name: dependency.package ?? alias, path: dependency.path, workspace: inherited, kind });
      }
    }
  }
  return result;
}

/** 🔨️ Resolves each crate's own Cargo workspace before interpreting inherited dependency paths. */
export function nativeWorkspaceV1(directory,workspaceRoot,manifest=readToml(join(directory,"Cargo.toml"))){
 let root=manifest.package?.workspace?resolve(directory,manifest.package.workspace):directory;
 for(;;){
  const local=relative(workspaceRoot,root);if(isAbsolute(local)||local===".."||local.startsWith(".."+sep))throw Error(`Native workspace escapes repository: ${directory}`);
  const path=join(root,"Cargo.toml");if(existsSync(path)){const workspace=readToml(path).workspace;if(workspace)return {root,declaration:workspace};}
  if(root===workspaceRoot)return {root,declaration:{}};
  root=dirname(root);
 }
}

const NATIVE_DIRECTORY_CLOSURES = new WeakMap();

/** 🧶️ The relative spelling of every crate directory a graph construction met, one string instance per directory: the closure of each of thousands of roots lists most of the crates, and a spelling per list entry cost hundreds of megabytes. */
const NATIVE_DIRECTORY_NAMES = new WeakMap();

/** 🛠️ Resolves Cargo's local compilation inputs, admitting development dependencies only at the selected test root. Every non-root crate's
 * closure is shared across roots of one graph construction; a cyclic path graph falls back to one visited-set traversal. */
export function nativeDependencyRoots(root, workspaceRoot, tests = false, cache = new Map(), closures = new Map()) {
  const key = `${root}\0${tests}`;
  return memoizedObservedComputationV1(closures,key,()=>{
  let names = NATIVE_DIRECTORY_NAMES.get(closures);
  if (!names) NATIVE_DIRECTORY_NAMES.set(closures, names = new Map());
  const spelled = (path) => { let name = names.get(path); if (name === undefined) names.set(path, name = buildPath(relative(workspaceRoot, path)) || "."); return name; };
  const read = path => memoizedObservedComputationV1(cache,path,()=>readToml(path));
  const children = function* (directory, includeTests) {
    const manifestPath = join(directory, "Cargo.toml");
    if (!existsSync(manifestPath)) { const error = new Error(`Native dependency has no Cargo manifest: ${manifestPath}`); error.code = "NATIVE_INPUT_ADMISSION"; error.manifest=manifestPath; throw error; }
    const manifest = read(manifestPath);
    let workspace = directory;
    if (manifest.package?.workspace) workspace = resolve(directory, manifest.package.workspace);
    else while (workspace !== workspaceRoot && !read(join(workspace, "Cargo.toml"))?.workspace) {
      workspace = dirname(workspace);
      while (workspace !== workspaceRoot && !existsSync(join(workspace, "Cargo.toml"))) workspace = dirname(workspace);
      if (buildPath(relative(workspaceRoot, workspace)).startsWith("../")) throw new Error(`Native workspace escapes repository: ${directory}`);
    }
    if (buildPath(relative(workspaceRoot, workspace)).startsWith("../")) throw new Error(`Native workspace escapes repository: ${directory}`);
    const authority = existsSync(join(workspace, "Cargo.toml")) ? read(join(workspace, "Cargo.toml")).workspace ?? {} : {};
    for (const dependency of nativeDependencies(manifest, authority)) {
      if (dependency.kind === "dev-dependencies" && !includeTests) continue;
      const child = resolve(dependency.workspace ? workspace : directory, dependency.path);
      if (buildPath(relative(workspaceRoot, child)).startsWith("../")) throw new Error(`Native dependency escapes repository: ${child}`);
      yield child;
    }
  };
  let shared = NATIVE_DIRECTORY_CLOSURES.get(closures);
  if (!shared) NATIVE_DIRECTORY_CLOSURES.set(closures, shared = new Map());
  const closure = (directory, includeTests, active) => {
    if (active.has(directory)) return null;
    const compute=()=>{
    active.add(directory);
    const members = new Set([directory]);
    for (const child of children(directory, includeTests)) {
      const reached = closure(child, false, active);
      if (reached === null) return null;
      for (const member of reached) members.add(member);
    }
    active.delete(directory);
    return [...members];
    };
    return includeTests?compute():memoizedObservedComputationV1(shared,directory,compute);
  };
  let visited = closure(resolve(workspaceRoot, root), tests, new Set());
  if (visited === null) {
    visited = new Set();
    const visit = (directory, includeTests) => {
      if (visited.has(directory)) return;
      visited.add(directory);
      for (const child of children(directory, includeTests)) visit(child, false);
    };
    visit(resolve(workspaceRoot, root), tests);
  }
  const result = [...visited].map(spelled).sort();
  return result;
  });
}

/** 🧬️ Selects declared generators across Cargo's compilation closure without scheduling native dependencies twice. */
export function nativePreparation(root, workspaceRoot, contracts, tests = false, cache = new Map(), closures = new Map()) {
  const roots = new Set(nativeDependencyRoots(root, workspaceRoot, tests, cache, closures));
  return Object.values(contracts).filter((contract) => {
    if (!contract.nativeConsumers?.some((path) => roots.has(path))) return false;
    if (contract.ownership !== "owned" || !contract.target) throw new Error(`Native prerequisite needs an owned generator: ${root}`);
    return true;
  }).sort((a, b) => a.target.localeCompare(b.target));
}

/** 🏗️ Attaches generation to native leaves in the outer task graph, including transitive Cargo consumers. */
export function withNativePreparation(project, workspaceRoot, contracts, cache = new Map(), projectsByRoot = new Map(), closures = new Map()) {
  const nativeRoot = project.metadata?.nativeRoot ?? project.root, manifest = join(workspaceRoot, nativeRoot, "Cargo.toml");
  if (!existsSync(manifest) || !readToml(manifest).package) return project;
  const generatorTargets = new Set(Object.values(contracts).flatMap((contract) => [contract.target, contract.previewTarget, contract.checkTarget]));
  const plans = new Map();
  for (const [name, target] of Object.entries(project.targets)) {
    const execution = targetExecutionKindV1(target);
    if (execution === "source" || execution === undefined && !/^(?:build|check|lint|test|wasm|native|component|describe|extension-package|package|font-tool|bench)(?:-|$)/.test(name) || generatorTargets.has(`${project.name}:${name}`)) continue;
    const tests = execution === "native" ? target.metadata.semio.execution.scope === "test" : execution === "orchestration" ? false : /^(?:test|bench)(?:-|$)/.test(name);
    if (!plans.has(tests)) {
      try { plans.set(tests, nativePreparation(nativeRoot, workspaceRoot, contracts, tests, cache, closures)); }
      catch (error) {
        if (error.code !== "NATIVE_INPUT_ADMISSION") throw error;
        const admission = "native-input-admission";
        project.metadata = { ...project.metadata, nativeInputAdmission: { manifest: buildPath(relative(workspaceRoot, manifest)), diagnostic: error.message } };
        project.targets[admission] = executionTargetV1("source", { executor: "repo:script", cache: false, options: { cwd: ".", command: `repo ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/📜️script.ts native-input-check --manifest ${JSON.stringify(buildPath(relative(workspaceRoot, manifest)))}` } });
        plans.set(tests, { admission });
      }
    }
    if (plans.get(tests).admission) { const name = `${project.name}:${plans.get(tests).admission}`; target.dependsOn = [...(target.dependsOn ?? []), name]; continue; }
    const selected = plans.get(tests);
    if (projectsByRoot.size && target.inputs?.some((input) => input === "^nativeSources" || input === "^nativeTestSources")) {
      const dependencies = nativeDependencyRoots(nativeRoot, workspaceRoot, tests, cache, closures).filter((root) => root !== nativeRoot).map((root) => {
        const owner = projectsByRoot.get(root);
        if (!owner){const error=new Error(`Native dependency has no build project owner: ${root} (required by ${project.name}:${name}; candidates=${projectsByRoot.size})`);error.code="BUILD_SOURCE_AUTHORITY";throw error;}
        return owner;
      }).sort();
      target.inputs = target.inputs.flatMap((input) => input === "^nativeSources" || input === "^nativeTestSources" ? dependencies.length ? [{ input: "nativeSources", projects: dependencies }] : [] : [input]);
    }
    if (!selected.length) continue;
    target.dependsOn = [...(target.dependsOn ?? []), ...selected.map((contract) => contract.target).filter((name) => !target.dependsOn?.includes(name))];
    target.inputs = [...(target.inputs ?? [tests ? "default" : "production", tests ? "^default" : "^production"]), { dependentTasksOutputFiles: "**/*", transitive: true }];
  }
  return project;
}

/** 🐹️ Reads module identities and replacement paths without executing Go during graph construction. */
export function goDependencies(text) {
  const result = { module: "", requires: [], replacements: [] };
  let block;
  for (const source of text.split("\n")) {
    const tokens = source.replace(/\/\/.*$/, "").trim().match(/"[^"]*"|\S+/g)?.map((value) => value.replace(/^"|"$/g, "")) ?? [];
    if (!tokens.length) continue;
    if (tokens[0] === ")") { block = undefined; continue; }
    if (tokens[1] === "(") { block = tokens[0]; continue; }
    const kind = block ?? tokens.shift();
    if (kind === "module") result.module = tokens[0];
    if (kind === "require") result.requires.push(tokens[0]);
    if (kind === "replace" && tokens.includes("=>")) result.replacements.push({ module: tokens[0], path: tokens[tokens.indexOf("=>") + 1] });
  }
  return result;
}

/** 🗺️ Resolves a language's source owner for package directories and their sibling domain sources. */
export function goManifest(root, workspaceRoot) {
  const owner = root.includes("/📦️packages/") ? root.split("/📦️packages/")[0] : root;
  const script = join(workspaceRoot, root, SCRIPT_BASENAME);
  const goScript = root.includes("🐹️go") || (existsSync(script) && observedSourceAnalysisV1(prepareSourceAnalysesV1().script,script).go);
  const local = join(workspaceRoot, root, "go.mod");
  if (existsSync(local)) return local;
  const manifest = join(workspaceRoot, owner, "go.mod");
  return goScript && existsSync(manifest) ? manifest : undefined;
}
