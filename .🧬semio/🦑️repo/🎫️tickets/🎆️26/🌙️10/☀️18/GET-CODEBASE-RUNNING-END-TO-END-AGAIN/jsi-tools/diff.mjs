// 🔬 usage: NODE_PATH=<scratch node_modules> bun diff.mjs <orig-root.mjs> <new-root.mjs>: differential battery over the cacheInternals roster of the original and the split build inference module.
import { mkdtempSync, mkdirSync, writeFileSync, rmSync, realpathSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { pathToFileURL } from "node:url";

const [origPath, newPath] = process.argv.slice(2);
const orig = await import(pathToFileURL(origPath).href);
const next = await import(pathToFileURL(newPath).href);

const plain = (value) => JSON.parse(JSON.stringify(value, (key, v) => {
  if (v instanceof Map) return { "@map": [...v.entries()] };
  if (v instanceof Set) return { "@set": [...v] };
  if (typeof v === "function") return "@function";
  if (typeof v === "bigint") return `@bigint:${v}`;
  if (v === undefined) return "@undefined";
  if (v instanceof Error) return `@error:${v.message}`;
  return v;
}));
const run = async (fn) => { try { return { ok: plain(await fn()) }; } catch (error) { return { error: String(error?.message ?? error).replaceAll(/jsi-(stage|orig)/gu, "jsi-X") }; } };

let failures = 0, checks = 0, errored = 0;
const errorLabels = [];
const compare = async (label, build) => {
  const a = await run(() => build(orig)), b = await run(() => build(next));
  checks++;
  if (a.error !== undefined) { errored++; errorLabels.push(`${label} => ${a.error.slice(0, 90)}`); }
  const left = JSON.stringify(a), right = JSON.stringify(b);
  if (left !== right) { failures++; console.log(`DIFF ${label}\n  orig: ${left.slice(0, 600)}\n  next: ${right.slice(0, 600)}`); }
};

await orig.libraryBootstrap; await next.libraryBootstrap;

// roster ---------------------------------------------------------------------
const keysOf = (m) => Object.keys(m.cacheInternals).sort();
checks++;
if (JSON.stringify(keysOf(orig)) !== JSON.stringify(keysOf(next))) { failures++; console.log("DIFF cacheInternals keys", keysOf(orig).filter((k) => !keysOf(next).includes(k)), keysOf(next).filter((k) => !keysOf(orig).includes(k))); }
for (const key of keysOf(orig)) await compare(`typeof cacheInternals.${key}`, (m) => typeof Object.getOwnPropertyDescriptor(m.cacheInternals, key).value + ":" + typeof Object.getOwnPropertyDescriptor(m.cacheInternals, key).get);
const added = new Set(["currentImplementationRevisionV1", "loadCurrentImplementationV1"]);
await compare("module export keys", (m) => Object.keys(m).filter((k) => !added.has(k)).sort());
await compare("typeof module exports", (m) => Object.fromEntries(Object.entries(m).filter(([k]) => !added.has(k)).map(([k, v]) => [k, typeof v])));
await compare("default plugin", (m) => ({ name: m.default.name, pattern: m.default.createNodesV2[0], keys: Object.keys(m.default) }));
await compare("runtimeComponentClosure after bootstrap", (m) => typeof m.cacheInternals.runtimeComponentClosure);
await compare("source analyses keys", (m) => Object.keys(m.prepareSourceAnalysesV1()).sort());
await compare("source analyses registered providers", (m) => Object.values(m.prepareSourceAnalysesV1()).map((p) => ({ revisionLength: p.revision.length, idPrefix: p.id.replace(/:[a-f0-9]{64}$/u, "") })));
await compare("implementation revision shape", (m) => /^[a-f0-9]{64}$/u.test(m.cacheInternals.implementationRevision()));

// pure policy ----------------------------------------------------------------------
const names = ["build", "test", "test-unit", "check", "lint-ts", "typecheck", "format-check", "generate", "generate-x", "schema", "verify", "stdio-x", "wasm", "native-build", "package", "extension-package", "cpp-build", "graph-check", "policy-check", "artifact-check", "toolchain", "watch", "dev", "serve-x", "clean", "gc", "prune", "setup", "fuzz", "e2e", "live-x", "x-e2e", "contract-x", "generator-x", "generator-inputs-x", "deps-wasm-opt", "other", "build-doc", "watch-doc", ""];
const targets = [{}, { cache: false }, { cache: true }, { options: { command: "bun x verify" } }, { inputs: ["a"] }, { options: { command: "bun ./📜️script.ts build" } }];
for (const name of names) for (const [index, target] of targets.entries()) {
  await compare(`targetPolicy ${JSON.stringify(name)}/${index}`, (m) => m.cacheInternals.targetPolicy(name, target));
  await compare(`predicates ${JSON.stringify(name)}/${index}`, (m) => [m.cacheInternals.cacheableFamily(name), m.cacheInternals.mutatingName(name), m.cacheInternals.liveName(name), m.cacheInternals.matchesUncached(name, { uncachedExact: ["clean"], uncached: ["watch"] }), m.cacheInternals.verifyCommand(target)]);
}
for (const required of [true, false]) for (const targetsInput of [{ wasm: { options: {} } }, { build: {} }, { wasm: { dependsOn: ["x"], options: { env: { A: "1" } } } }]) await compare(`withWasmTooling ${required}`, (m) => m.cacheInternals.withWasmTooling(targetsInput, required));
for (const target of [{}, { metadata: { semio: { execution: { version: 1, kind: "source" } } } }, { metadata: { semio: { execution: { version: 1, kind: "native", scope: "test" } } } }, { metadata: { semio: { execution: { version: 1, kind: "unknown" } } } }, { metadata: { semio: { execution: { version: 1, kind: "orchestration" } } } }]) await compare(`targetExecutionKindV1 ${JSON.stringify(target)}`, (m) => m.cacheInternals.targetExecutionKindV1(target, "x:y"));
for (const [target, root, owns] of [[{ command: "bun x" }, "app", true], [{ executor: "other" }, "app", true], [{ options: { cwd: "z" } }, "app", true], [{}, "app", false], [{ command: "bun x", options: { command: "y" } }, "app", true]]) await compare(`targetWithDefaults ${JSON.stringify(target)}`, (m) => m.cacheInternals.targetWithDefaults(target, root, owns));
for (const command of ["bun x native cargo metadata y", "bun x", { options: { command: "bun x native cargo metadata y" }, configurations: { a: { commands: ["bun x native cargo metadata z", { command: "bun q" }] } } }, undefined]) await compare(`nativeLockInputs ${JSON.stringify(command)}`, (m) => m.cacheInternals.nativeLockInputs(command));
await compare("goDependencies", (m) => m.cacheInternals.goDependencies('module example.com/x\nrequire (\n\ta.com/b v1\n\tc.com/d v2 // indirect\n)\nreplace a.com/b => ../b\nrequire e.com/f v3\n'));
await compare("resolveOutputPath", (m) => ["{projectRoot}/x", "{workspaceRoot}/y", "z", "{projectRoot}/../q"].map((o) => m.cacheInternals.resolveOutputPath(o, "proj/root", "/ws")));
await compare("outputRootInputs", (m) => m.cacheInternals.outputRootInputs({ path: "a/b" }, "/ws"));
await compare("generatorContractInputs", (m) => { try { return m.cacheInternals.generatorContractInputs({ inputPatterns: ["a", "b"], outputRoots: [] }); } catch (e) { return String(e); } });
await compare("generatorContractInputs with discovery", (m) => { try { return m.cacheInternals.generatorContractInputs({ inputPatterns: ["a"], inputDiscovery: { kind: "nonexistent-kind" }, outputRoots: [] }); } catch (e) { return String(e); } });
await compare("generatorOutputCouplingInputs", (m) => m.cacheInternals.generatorOutputCouplingInputs("check-x", { inputs: [] }, { "generate-x": { outputs: ["{projectRoot}/o", "{workspaceRoot}/p"] }, "check-x": {} }, "root", "/ws"));
await compare("genericCommandFallbackInputs", (m) => m.cacheInternals.genericCommandFallbackInputs("/ws"));

// lock graph -------------------------------------------------------------------------
const lock = { lockfileVersion: 1, workspaces: { "": { name: "root" }, "pkg/a": { name: "a" } }, packages: { a: ["a@workspace:pkg/a"], left: ["left@1.0.0", "", { dependencies: { right: "^1" } }, "sha"], right: ["right@2.0.0", "", {}, "sha2"], "left/right": ["right@3.0.0", "", {}, "sha3"] } };
await compare("bunLockGraph", (m) => { const g = m.cacheInternals.bunLockGraph(lock, {}); return { nodes: g.externalNodes, deps: g.dependencies, ws: g.workspacePackages, r1: g.resolve("left", "right"), r2: g.resolveImport("pkg/a/x.ts", "left") }; });
await compare("bunLockGraph bad version", (m) => m.cacheInternals.bunLockGraph({ ...lock, lockfileVersion: 9 }));
await compare("bunLockGraph bad package", (m) => m.cacheInternals.bunLockGraph({ ...lock, packages: { x: [1] } }));
await compare("bunLockGraph patched missing", (m) => m.cacheInternals.bunLockGraph({ ...lock, patchedDependencies: { "left@1.0.0": "patches/x.patch" } }, {}));

// rust sources -------------------------------------------------------------------------
const work = realpathSync(mkdtempSync(join(tmpdir(), "jsi-diff-")));
try {
  mkdirSync(join(work, "crate/src/inner"), { recursive: true });
  writeFileSync(join(work, "crate/Cargo.toml"), '[package]\nname="c"\nversion="0.1.0"\nedition="2021"\n[lib]\npath="src/lib.rs"\n[dependencies]\nd={path="../dep"}\n');
  writeFileSync(join(work, "crate/src/lib.rs"), 'mod a;\n#[path="custom.rs"] mod b;\nmod inner { pub mod deep; }\nconst X: &str = include_str!("data.txt");\nfn f() { let _ = "mod not_a_mod;"; } // mod commented;\n');
  writeFileSync(join(work, "crate/src/a.rs"), 'pub fn a() {}\n');
  writeFileSync(join(work, "crate/src/custom.rs"), 'pub fn b() {}\n');
  writeFileSync(join(work, "crate/src/inner/deep.rs"), 'pub fn d() {}\n');
  writeFileSync(join(work, "crate/src/data.txt"), "data");
  mkdirSync(join(work, "dep/src"), { recursive: true });
  writeFileSync(join(work, "dep/Cargo.toml"), '[package]\nname="d"\nversion="0.1.0"\nedition="2021"\n');
  writeFileSync(join(work, "dep/src/lib.rs"), "");
  writeFileSync(join(work, "Cargo.toml"), '[workspace]\nmembers=["crate","dep"]\n');
  const rel = (list) => list.map((p) => p.replace(work, "<w>"));
  await compare("rustSourceFiles", (m) => rel(m.cacheInternals.rustSourceFiles([join(work, "crate/src/lib.rs")], join(work, "crate"))));
  await compare("rustSourceFiles fresh cache", (m) => { const cache = m.cacheInternals.createRustSourceCache(); const files = rel(m.cacheInternals.rustSourceFiles([join(work, "crate/src/lib.rs")], join(work, "crate"), cache, new Map())); return { files, hits: cache.hits, misses: cache.misses, bytes: cache.bytes }; });
  await compare("createRustSourceCache invalid", (m) => m.cacheInternals.createRustSourceCache(0));
  await compare("nativeDependencyRoots", (m) => m.cacheInternals.nativeDependencyRoots("crate", work, true));
  await compare("nativeDependencyRoots non-test", (m) => m.cacheInternals.nativeDependencyRoots("crate", work, false));
  await compare("nativeDependencyRoots missing manifest", (m) => m.cacheInternals.nativeDependencyRoots("nowhere", work, false));
  await compare("nativeDependencies", (m) => m.cacheInternals.nativeDependencies({ dependencies: { a: { path: "../a" }, b: "1", c: { workspace: true } }, target: { x: { "dev-dependencies": { z: { path: "../z", package: "zz" } } } } }, { dependencies: { c: { path: "../c" } } }));
  await compare("cargoTargets", async (m) => { const t = await m.cacheInternals.cargoTargets("crate", work, ["x"], m.createScriptInputCache()); return t; });
  await compare("cargoTargets none", async (m) => m.cacheInternals.cargoTargets("dep", work, undefined, m.createScriptInputCache()).then((t) => Object.keys(t)));
  await compare("componentBuildTargets", (m) => m.cacheInternals.componentBuildTargets("crate", work));
  await compare("componentCrateRoots", (m) => m.cacheInternals.componentCrateRoots(["crate/Cargo.toml", "dep/Cargo.toml", "x/�/Cargo.toml"], work));
  await compare("collectPlaygroundCatalog empty", (m) => m.cacheInternals.collectPlaygroundCatalog(["crate/Cargo.toml"], work, []));
  await compare("walkCargoToml", (m) => m.cacheInternals.walkCargoToml(work));
  await compare("walkCargoTomlControlledV1", async (m) => { let calls = 0; return { found: await m.cacheInternals.walkCargoTomlControlledV1(work, { check() {}, async advance() { calls++; } }), calls }; });
  await compare("inferenceCandidates", (m) => m.cacheInternals.inferenceCandidates(["crate/Cargo.toml", "../x", "dep/Cargo.toml", "crate/src/lib.rs", "", "a\0b", "crate//Cargo.toml"], work));
  await compare("inferenceCandidates bad root", (m) => m.cacheInternals.inferenceCandidates(["a"], "../x"));
  await compare("buildTrackedSourceFile", (m) => m.cacheInternals.buildTrackedSourceFile({ workspaceRoot: work, fileMap: { projectFileMap: { p: [{ file: "crate/Cargo.toml" }] } } }, "p", "other/Cargo.toml"));
  await compare("moduleSourceRecordCacheRootV1", (m) => (m.cacheInternals.moduleSourceRecordCacheRootV1(work) ?? "").replace(work, "<w>"));
  await compare("projectFilesToProcess", (m) => [m.cacheInternals.projectFilesToProcess({}), m.cacheInternals.projectFilesToProcess({ fileMap: { projectFileMap: { a: [] } } }), m.cacheInternals.projectFilesToProcess({ filesToProcess: { projectFileMap: { b: [] } }, fileMap: { projectFileMap: { a: [] } } })]);
  await compare("projectAssemblyAuthorityV1", (m) => m.cacheInternals.projectAssemblyAuthorityV1(work, "crate", new Map([["crate", "c"], ["dep", "d"]]), ["x"]));
  await compare("importTargetsFromSource", (m) => m.cacheInternals.importTargetsFromSource('import a from "./x.ts"; import b from "left"; import c from "node:fs"; import d from "@scope/pkg/sub";', "app/file.ts", work, { app: { root: "app" }, lib: { root: "lib" } }, new Map([["@scope/pkg", "scopeproject"]]), undefined));
  await compare("createDependenciesImplementation refusal", (m) => m.cacheInternals.createDependenciesImplementation({}, { workspaceRoot: work, projects: {} }));
  await compare("createMetadataImplementation refusal", (m) => m.cacheInternals.createMetadataImplementation({ nodes: {} }, {}, { workspaceRoot: work }));
  await compare("collectImportEdges", async (m) => { const edges = []; await m.cacheInternals.collectImportEdges(work, {}, {}, new Map(), undefined, (a, b, c) => edges.push([a, b, c]), undefined, undefined, {}); return edges; });
  await compare("relativeScriptInputs", async (m) => {
    writeFileSync(join(work, "script.ts"), 'import { x } from "./helper.ts"; import "node:fs"; export const y = x;\n');
    writeFileSync(join(work, "helper.ts"), 'export const x = 1;\n');
    return (await m.cacheInternals.relativeScriptInputs([join(work, "script.ts")], work)).sort();
  });
  await compare("relativeScriptInputs escaping", async (m) => m.cacheInternals.relativeScriptInputs([join(work, "../escape.ts")], work));
  await compare("targetScriptSources", async (m) => { const r = await m.cacheInternals.targetScriptSources({ options: { command: "bun ./script.ts x" } }, work, m.createScriptInputCache(), "."); return { closure: r.closure?.sort(), fallback: r.fallback }; });
  await compare("targetScriptClosure", async (m) => (await m.cacheInternals.targetScriptClosure({ options: { command: "bun ./script.ts x" } }, work, m.createScriptInputCache(), "."))?.sort());
  await compare("genericTargetCommandInputs", async (m) => m.cacheInternals.genericTargetCommandInputs({ options: { command: "bun ./script.ts x" } }, work, ["fallback"], m.createScriptInputCache(), "."));
  await compare("genericTargetCommandInputs fallback", async (m) => m.cacheInternals.genericTargetCommandInputs({ options: { command: "echo" } }, work, ["fallback"], m.createScriptInputCache(), "."));
  await compare("nativeTargetCommandInputs", async (m) => m.cacheInternals.nativeTargetCommandInputs({ options: { command: "bun ./script.ts x" } }, work, ["f"], m.createScriptInputCache(), "."));
  await compare("projectInputs", async (m) => m.cacheInternals.projectInputs({ name: "c", targets: { build: { options: { command: "bun ./script.ts x" } } } }, "crate", work, new Map(), m.createScriptInputCache(), "source"));
  await compare("declaredSourceInputs", async (m) => m.cacheInternals.declaredSourceInputs({ metadata: {} }, work, {}));
  await compare("playgroundSessionTargets empty", async (m) => m.cacheInternals.playgroundSessionTargets(["crate/Cargo.toml"], work, []));
  await compare("pluginSiteTargetsForCrate none", (m) => m.cacheInternals.pluginSiteTargetsForCrate("crate", [], "stage", work));
  await compare("pluginSiteTargetsForCrate", (m) => m.cacheInternals.pluginSiteTargetsForCrate("crate", [{ cratePath: "crate", variant: "v1", distDir: "d" }, { cratePath: "other", variant: "v2" }], "stage", work));
  await compare("pluginSiteTargetsForCrate invalid", (m) => m.cacheInternals.pluginSiteTargetsForCrate("crate", [{ cratePath: "crate", variant: "Bad" }], "stage", work));
  await compare("rootCommandTargets", (m) => { writeFileSync(join(work, "root-script.ts"), 'const router = new Router().register("alpha", x).register("nx", y).register("beta", z);\n'); return m.cacheInternals.rootCommandTargets(join(work, "root-script.ts")); });
  await compare("withNativePreparation none", (m) => m.cacheInternals.withNativePreparation({ name: "n", root: "dep", targets: { build: { options: {} } } }, work, {}, new Map(), new Map(), new Map()));
  await compare("withNativePreparation with generator", (m) => m.cacheInternals.withNativePreparation({ name: "n", root: "crate", targets: { build: { inputs: ["nativeSources", "^nativeSources"], options: {} }, "generate-x": {}, test: { options: {} } } }, work, { g: { ownership: "owned", target: "n:generate-x", nativeConsumers: ["crate"] } }, new Map(), new Map([["dep", "d"]]), new Map()));
  await compare("withReleasePipelinePreparation", (m) => m.cacheInternals.withReleasePipelinePreparation({ name: "r", targets: { release: { cache: false, metadata: { semio: { orchestrator: true, release: { prerequisites: ["p"], jobs: [{ root: "crate" }] } } } } } }, work, {}, [], new Map(), new Map()));
  await compare("withReleasePipelinePreparation invalid", (m) => m.cacheInternals.withReleasePipelinePreparation({ name: "r", targets: { release: { metadata: { semio: { release: { prerequisites: "x" } } } } } }, work, {}, [], new Map(), new Map()));
} finally { rmSync(work, { recursive: true, force: true }); }

if (process.env.JSI_DIFF_ERRORS) console.log(errorLabels.join("\n"));
console.log(`checks=${checks} diffs=${failures} errored-both=${errored}`);
process.exit(failures ? 1 : 0);
