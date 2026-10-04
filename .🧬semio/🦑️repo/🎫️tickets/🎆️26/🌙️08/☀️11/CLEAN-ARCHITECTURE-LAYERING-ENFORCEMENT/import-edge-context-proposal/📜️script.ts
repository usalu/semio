import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, realpathSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";

const workspace = resolve(import.meta.dir, "../../../../../../../.."), ticket = dirname(import.meta.dir);
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library", caching = `${library}/⚡️caching`, inference = `${caching}/📥️inference`;
const output = join(ticket, "🗑️generated/import-edge-context");
mkdirSync(output, { recursive: true });
const hash = (value: string) => createHash("sha256").update(value).digest("hex");
const string = { type: "string" }, strings = { type: "array", uniqueItems: true, items: string };
const object = (properties: object) => ({ type: "object", additionalProperties: false, required: Object.keys(properties), properties });
const phaseSchema = object({ file: string, projects: { type: "object", additionalProperties: string }, packages: { type: "object", additionalProperties: string }, locked: { type: "object", additionalProperties: string }, targets: strings });
const schema = { $schema: "http://json-schema.org/draft-07/schema#", ...object({ version: { const: 1 }, imports: { type: "array", minItems: 14, items: object({ id: string, path: string, source: string, imports: strings }) }, contexts: { type: "array", minItems: 7, items: object({ id: string, source: string, phases: { type: "array", minItems: 2, items: phaseSchema } }) } }) };
const phase = (file: string, projects: Record<string, string>, targets: string[], packages = {}, locked = {}) => ({ file, projects: { caller: dirname(file), ...projects }, packages, locked, targets });
const fixture = {
  version: 1,
  imports: [
    { id: "static-relative", path: "case.ts", source: "import {value} from './leaf.ts'; export const result=value;", imports: ["./leaf.ts"] },
    { id: "scoped-side-effect", path: "case.ts", source: "import '@semio/example/feature';", imports: ["@semio/example/feature"] },
    { id: "bare-reexport", path: "case.ts", source: "export {value} from 'example';", imports: ["example"] },
    { id: "dynamic-literal", path: "case.ts", source: "export const load=()=>import('./leaf.ts');", imports: ["./leaf.ts"] },
    { id: "require-literal", path: "case.cjs", source: "module.exports=require('example');", imports: ["example"] },
    { id: "import-equals", path: "case.cts", source: "import value = require('example'); export const result=value;", imports: ["example"] },
    { id: "type-import", path: "case.ts", source: "import type {Value} from 'erased'; export const value=1;", imports: [] },
    { id: "named-type-import", path: "case.ts", source: "import {type Value} from 'erased'; export const value=1;", imports: [] },
    { id: "type-export", path: "case.ts", source: "export type {Value} from 'erased';", imports: [] },
    { id: "named-type-export", path: "case.ts", source: "export {type Value} from 'erased';", imports: [] },
    { id: "mixed-import", path: "case.ts", source: "import {type Value,value} from 'example'; export const result=value;", imports: ["example"] },
    { id: "mixed-export", path: "case.ts", source: "export {type Value,value} from 'example';", imports: ["example"] },
    { id: "import-type-expression", path: "case.ts", source: "export type Value=import('erased').Value;", imports: [] },
    { id: "declaration", path: "case.d.ts", source: "import {Value} from 'erased'; export declare const value:Value;", imports: [] },
    { id: "comments-and-strings", path: "case.ts", source: "// import 'commented';\n/* export {x} from 'commented'; */\nexport const fake=\"import 'quoted'; require('quoted')\";", imports: [] },
    { id: "nonliteral", path: "case.ts", source: "export const load=(name:string)=>import(name); export const resolve=(name:string)=>require(name);", imports: [] },
    { id: "builtin", path: "case.ts", source: "import {readFileSync} from 'node:fs'; export const read=readFileSync;", imports: ["node:fs"] },
    { id: "json", path: "case.json", source: '{"text":"import \'quoted\';"}', imports: [] },
  ],
  contexts: [
    { id: "same-content-moved", source: "import '../target/leaf.ts';", phases: [phase("alpha/src/index.ts", { alpha: "alpha/target", beta: "beta/target" }, ["alpha"]), phase("beta/src/index.ts", { alpha: "alpha/target", beta: "beta/target" }, ["beta"])] },
    { id: "provider-name-change", source: "import '../target/leaf.ts';", phases: [phase("caller/index.ts", { old: "target" }, ["old"]), phase("caller/index.ts", { next: "target" }, ["next"])] },
    { id: "nested-owner-change", source: "import '../target/nested/leaf.ts';", phases: [phase("caller/index.ts", { broad: "target" }, ["broad"]), phase("caller/index.ts", { broad: "target", nested: "target/nested" }, ["nested"])] },
    { id: "project-root-swap", source: "import '../target/leaf.ts';", phases: [phase("caller/index.ts", { first: "target", second: "spare" }, ["first"]), phase("caller/index.ts", { first: "spare", second: "target" }, ["second"])] },
    { id: "workspace-package-alias-change", source: "import '__semio_import_fixture__/feature';", phases: [phase("caller/index.ts", { first: "first", second: "second" }, ["first"], { __semio_import_fixture__: "first" }), phase("caller/index.ts", { first: "first", second: "second" }, ["second"], { __semio_import_fixture__: "second" })] },
    { id: "lock-version-change", source: "import 'semio-import-fixture-lock';", phases: [phase("caller/index.ts", {}, ["npm:semio-import-fixture-lock@1.0.0"], {}, { "semio-import-fixture-lock": "1.0.0" }), phase("caller/index.ts", {}, ["npm:semio-import-fixture-lock@2.0.0"], {}, { "semio-import-fixture-lock": "2.0.0" })] },
    { id: "removed-provider", source: "import '../target/leaf.ts';", phases: [phase("caller/index.ts", { provider: "target" }, ["provider"]), phase("caller/index.ts", {}, [])] },
    { id: "fake-import-targets", source: "// import 'fake';\n/* export {value} from 'fake'; */\nexport const quote=\"import 'fake'; require('fake')\"; import type {Value} from 'fake'; export {type Value} from 'fake'; import '../target/leaf.ts';", phases: [phase("caller/index.ts", { provider: "target", fake: "fake" }, ["provider"], { fake: "fake" }), phase("caller/index.ts", { next: "target", fake: "fake" }, ["next"], { fake: "fake" })] },
    { id: "old-resolved-cache-isolation", source: "import '../target/leaf.ts';", phases: [phase("caller/index.ts", { provider: "target" }, ["provider"]), phase("caller/index.ts", { provider: "target" }, ["provider"])] },
  ],
};
const rows: { path: string; before: string | null; beforeHash: string | null; authored: string; authoredHash: string }[] = [];
const propose = (path: string, authored: string) => { const absolute = join(workspace, path), before = existsSync(absolute) ? readFileSync(absolute, "utf8") : null; rows.push({ path, before, beforeHash: before === null ? null : hash(before), authored, authoredHash: hash(authored) }); };
if (process.argv[2] === "prepare") {
  propose(`${caching}/🧬️schema/🔗️import-edges/🔣️.json`, `${JSON.stringify(schema, null, 2)}\n`);
  propose(`${caching}/🧫️fixtures/import-edges/🔁️context/🔣️.json`, `${JSON.stringify(fixture, null, 2)}\n`);
  propose(`${caching}/🧪️tests/🔗️import-edges/🔁️context/🟦️.ts`, readFileSync(join(import.meta.dir, "🔁️context/🟦️.ts"), "utf8"));
  const parserLaws = `${inference}/🧪️tests/🔍️imports/🟦️.ts`;
  propose(parserLaws, readFileSync(join(workspace, parserLaws), "utf8") + '\nimport "../../../🧪️tests/🔗️import-edges/🔁️context/🟦️.ts";\nimport { testImportEdgeEquality } from "../../../🧪️tests/🔗️import-edges/🟦️.ts";\n\ntest("full registered repository import dependency equality remains intact", async () => {\n  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;\n  if (!output) throw Error("SEMIO_TEST_ARTIFACT_DIR is required");\n  await testImportEdgeEquality(process.cwd(), output);\n}, 120000);\n');
  const equality = `${caching}/🧪️tests/🔗️import-edges/🟦️.ts`;
  propose(equality, readFileSync(join(workspace, equality), "utf8").replace('const repoData = join(output, "import-edges-repo-cache");\n    rmSync(repoData, { recursive: true, force: true });\n    mkdirSync(repoData, { recursive: true });', 'const repoData = mkdtempSync(join(root, "repository-facts-"));').replace('"warm hash cache must preserve the full edge set"', '"warm immutable facts must preserve the full edge set"').replace('  } finally {', '    console.log(`[DEBUG] full repository import dependency equality: ${cold.length} edges, cold/warm/incremental fixture and independent import-only scan`);\n  } finally {'));
  const route = `${library}/📦️packages/🟦️typescript/📜️script.ts`;
  propose(route, readFileSync(join(workspace, route), "utf8").replace('env: repoTestArtifactEnvironment(this.repoRoot, "nx-project-inference"), budgetMs: 30_000', 'env: repoTestArtifactEnvironment(this.repoRoot, "nx-project-inference"), budgetMs: imports ? 180_000 : 30_000'));
  const projectPath = `${library}/📦️packages/🟦️typescript/📋️project.json`, project = JSON.parse(readFileSync(join(workspace, projectPath), "utf8"));
  project.targets["test-nx-project-inference-imports"].inputs.push(`{workspaceRoot}/${caching}/🧪️tests/🔗️import-edges/**/*`, `{workspaceRoot}/${caching}/🧫️fixtures/import-edges/**/*`, `{workspaceRoot}/${caching}/🧬️schema/🔗️import-edges/**/*`);
  propose(projectPath, JSON.stringify(project, null, 2) + "\n");
  writeFileSync(join(output, "test-only-schema-before-authored-1.json"), JSON.stringify({ workspace, rows }, null, 2));
  console.log(`[DEBUG] prepared closed import source/context schema: ${fixture.imports.length} source vectors, ${fixture.contexts.length} context replay laws; no production mutation`);
} else if (process.argv[2] === "mount-tests") {
  const capture = JSON.parse(readFileSync(join(output, "test-only-schema-before-authored-1.json"), "utf8"));
  for (const row of capture.rows) { const path = join(workspace, row.path), current = existsSync(path) ? readFileSync(path, "utf8") : null; if (current !== row.before) throw Error(`Source guard changed: ${row.path}`); }
  for (const row of capture.rows) { const path = join(workspace, row.path); if ((existsSync(path) ? readFileSync(path, "utf8") : null) !== row.before) throw Error(`Immediate source guard changed: ${row.path}`); mkdirSync(dirname(path), { recursive: true }); writeFileSync(path, row.authored); }
  writeFileSync(join(output, "test-only-mounted-post-1.json"), JSON.stringify({ rows: capture.rows.map(row => ({ path: row.path, authoredHash: row.authoredHash, currentHash: hash(readFileSync(join(workspace, row.path), "utf8")) })) }, null, 2));
  console.log(`[DEBUG] mounted ${capture.rows.length} guarded test-only schema/fixture/law/gate input rows; production parser/plugin unchanged`);
} else if (process.argv[2] === "amend-tests") {
  const capture = JSON.parse(readFileSync(join(output, "test-only-schema-before-authored-1.json"), "utf8")), path = `${caching}/🧪️tests/🔗️import-edges/🔁️context/🟦️.ts`;
  let ordinal = 2;
  while (existsSync(join(output, `test-only-oracle-amend-before-authored-${ordinal}.json`))) ordinal++;
  const previous = ordinal === 2 ? capture.rows.find(row => row.path === path) : JSON.parse(readFileSync(join(output, `test-only-oracle-amend-before-authored-${ordinal - 1}.json`), "utf8")).rows[0], before = readFileSync(join(workspace, path), "utf8"), authored = readFileSync(join(import.meta.dir, "🔁️context/🟦️.ts"), "utf8");
  if (before !== previous.authored) throw Error(`Source guard changed: ${path}`);
  writeFileSync(join(output, `test-only-oracle-amend-before-authored-${ordinal}.json`), JSON.stringify({ rows: [{ path, before, beforeHash: hash(before), authored, authoredHash: hash(authored) }] }, null, 2));
  if (readFileSync(join(workspace, path), "utf8") !== before) throw Error(`Immediate source guard changed: ${path}`);
  writeFileSync(join(workspace, path), authored);
  console.log("[DEBUG] test oracle amendment mounted with full inverse; production remains unchanged");
} else if (process.argv[2] === "prepare-production") {
  const plugin = `${library}/🟨️.mjs`, parser = `${inference}/🟨️.mjs`;
  const parserBefore = readFileSync(join(workspace, parser), "utf8");
  const parserAuthored = parserBefore.replace("content-derived command facts", "content-derived runtime module facts").replace("export function commandSourceImports(path, source) {", "export function moduleSourceImports(path, source) {").replace("compiler.isStringLiteralLike(node) && node.text.startsWith(\".\")", "compiler.isStringLiteralLike(node)").replace('if (compiler.isExportDeclaration(node)) { if (!node.isTypeOnly) add(node.moduleSpecifier); return; }', 'if (compiler.isExportDeclaration(node)) { const clause = node.exportClause; if (!node.isTypeOnly && !(clause && compiler.isNamedExports(clause) && clause.elements.length && clause.elements.every(element => element.isTypeOnly))) add(node.moduleSpecifier); return; }').replace("const result = [...imports], size", "const result = Object.freeze([...imports]), size").replace("\n\n/** 📊️", '\n\n/** 🧭️ Projects canonical runtime module facts onto relative command source imports. */\nexport function commandSourceImports(path, source) { return moduleSourceImports(path, source).filter(specifier => specifier.startsWith(".")); }\n\n/** 📊️');
  propose(parser, parserAuthored);
  const pluginBefore = readFileSync(join(workspace, plugin), "utf8"), start = pluginBefore.indexOf("/** 🗂️ Nx workspace-data directory"), end = pluginBefore.indexOf("/**\n * 🧭️ Files Nx asks", start);
  if (start < 0 || end < start) throw Error("Current import edge region changed");
  const productionRegion = readFileSync(join(import.meta.dir, "🔗️source-facts/🟨️.mjs"), "utf8");
  const pluginAuthored = (pluginBefore.slice(0, start) + productionRegion + pluginBefore.slice(end)).replace('import { createRequire } from "node:module";', 'import { createRequire, isBuiltin } from "node:module";').replace('const { commandSourceImports } = commandInputs;', 'const { commandSourceImports, moduleSourceImports } = commandInputs;').replace('  if (process.platform === "win32") {\n    const locked = _options?.analyzeLockfile && existsSync(join(context.workspaceRoot, "bun.lock")) ? readBunLockGraph(context.workspaceRoot) : undefined;\n    return locked?.dependencies ?? [];\n  }\n', '').replace('projectFilesToProcess, importEdgeCacheRoot, nxTrackedSourceFile', 'projectFilesToProcess, moduleSourceFactCacheRoot, nxTrackedSourceFile');
  propose(plugin, pluginAuthored);
  if (rows.some(row => row.before === row.authored)) throw Error("No-op production proposal");
  writeFileSync(join(output, "production-full-before-authored-1.json"), JSON.stringify({ workspace, rows }, null, 2));
  console.log(`[DEBUG] prepared ${rows.length} production rows with full before/inverses; mounted production remains unchanged`);
} else if (process.argv[2] === "mount-production") {
  const capture = JSON.parse(readFileSync(join(output, "production-full-before-authored-1.json"), "utf8"));
  for (const row of capture.rows) if (readFileSync(join(workspace, row.path), "utf8") !== row.before) throw Error(`Source guard changed: ${row.path}`);
  for (const row of capture.rows) { const path = join(workspace, row.path); if (readFileSync(path, "utf8") !== row.before) throw Error(`Immediate source guard changed: ${row.path}`); writeFileSync(path, row.authored); }
  writeFileSync(join(output, "production-mounted-post-1.json"), JSON.stringify({ rows: capture.rows.map(row => ({ path: row.path, authoredHash: row.authoredHash, currentHash: hash(readFileSync(join(workspace, row.path), "utf8")) })) }, null, 2));
  console.log(`[DEBUG] mounted ${capture.rows.length} canonical source-facts production rows under full current byte guards`);
} else if (process.argv[2] === "prepare-retained-tests") {
  const schemaPath = `${caching}/🧬️schema/🔗️import-edges/🔣️.json`, fixturePath = `${caching}/🧫️fixtures/import-edges/🔁️context/🔣️.json`;
  const currentSchema = JSON.parse(readFileSync(join(workspace, schemaPath), "utf8")), currentFixture = JSON.parse(readFileSync(join(workspace, fixturePath), "utf8"));
  currentSchema.required.push("repository");
  currentSchema.properties.repository = object({ owner: string, sources: { ...strings, minItems: 2 } });
  currentFixture.repository = { owner: "@semio-tech/framework-process", sources: ["🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🟦️.ts", "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🧪️tests/🟦️.ts"] };
  propose(schemaPath, JSON.stringify(currentSchema, null, 2) + "\n");
  propose(fixturePath, JSON.stringify(currentFixture, null, 2) + "\n");
  propose(`${caching}/🧪️tests/🔗️import-edges/🏛️graph/🟦️.ts`, readFileSync(join(import.meta.dir, "🏛️graph/🟦️.ts"), "utf8"));
  const parserLaws = `${inference}/🧪️tests/🔍️imports/🟦️.ts`;
  propose(parserLaws, readFileSync(join(workspace, parserLaws), "utf8") + '\nimport "../../../🧪️tests/🔗️import-edges/🏛️graph/🟦️.ts";\n');
  writeFileSync(join(output, "retained-graph-test-only-before-authored-1.json"), JSON.stringify({ workspace, rows }, null, 2));
  console.log(`[DEBUG] prepared ${rows.length} schema-first retained ordinary graph test-only rows`);
} else if (process.argv[2] === "mount-retained-tests") {
  const capture = JSON.parse(readFileSync(join(output, "retained-graph-test-only-before-authored-1.json"), "utf8"));
  for (const row of capture.rows) { const path = join(workspace, row.path); if ((existsSync(path) ? readFileSync(path, "utf8") : null) !== row.before) throw Error(`Source guard changed: ${row.path}`); }
  for (const row of capture.rows) { const path = join(workspace, row.path); if ((existsSync(path) ? readFileSync(path, "utf8") : null) !== row.before) throw Error(`Immediate source guard changed: ${row.path}`); mkdirSync(dirname(path), { recursive: true }); writeFileSync(path, row.authored); }
  writeFileSync(join(output, "retained-graph-test-only-mounted-post-1.json"), JSON.stringify({ rows: capture.rows.map(row => ({ path: row.path, authoredHash: row.authoredHash, currentHash: hash(readFileSync(join(workspace, row.path), "utf8")) })) }, null, 2));
  console.log(`[DEBUG] mounted ${capture.rows.length} retained ordinary graph test-only rows; patch/bootstrap/config/install unchanged`);
} else if (process.argv[2] === "amend-retained-tests") {
  const capture = JSON.parse(readFileSync(join(output, "retained-graph-test-only-before-authored-1.json"), "utf8")), path = `${caching}/🧪️tests/🔗️import-edges/🏛️graph/🟦️.ts`;
  let ordinal = 2;
  while (existsSync(join(output, `retained-graph-test-only-amend-before-authored-${ordinal}.json`))) ordinal++;
  const previous = ordinal === 2 ? capture.rows.find(row => row.path === path) : JSON.parse(readFileSync(join(output, `retained-graph-test-only-amend-before-authored-${ordinal - 1}.json`), "utf8")).rows[0];
  const before = readFileSync(join(workspace, path), "utf8"), authored = readFileSync(join(import.meta.dir, "🏛️graph/🟦️.ts"), "utf8");
  if (before !== previous.authored) throw Error(`Source guard changed: ${path}`);
  writeFileSync(join(output, `retained-graph-test-only-amend-before-authored-${ordinal}.json`), JSON.stringify({ rows: [{ path, before, beforeHash: hash(before), authored, authoredHash: hash(authored) }] }, null, 2));
  if (readFileSync(join(workspace, path), "utf8") !== before) throw Error(`Immediate source guard changed: ${path}`);
  writeFileSync(join(workspace, path), authored);
  console.log("[DEBUG] full configured Nx graph oracle amendment mounted with inverse; broader production held");
} else if (process.argv[2] === "prepare-retained-production") {
  const pluginPath = `${library}/🟨️.mjs`, pluginBefore = readFileSync(join(workspace, pluginPath), "utf8");
  const authority = readFileSync(join(import.meta.dir, "🏛️graph/🟨️.mjs"), "utf8");
  const pluginAuthored = pluginBefore.replace('return kind === "nodes" ? emojiProjectJsonNodes(...args) : createDependenciesImplementation(...args);', 'return kind === "nodes" ? emojiProjectJsonNodes(...args) : kind === "authority" ? dependencyResolutionAuthorityImplementation(...args) : createDependenciesImplementation(...args);').replace('return kind === "nodes" ? module.default.createNodesV2[1](...args) : module.createDependencies(...args);', 'return kind === "nodes" ? module.default.createNodesV2[1](...args) : kind === "authority" ? module.dependencyResolutionAuthority(...args) : module.createDependencies(...args);').replace('export function createDependencies(...args)', `${authority}\n\nexport function createDependencies(...args)`);
  propose(pluginPath, pluginAuthored);
  const nxPath = "nx.json", nx = JSON.parse(readFileSync(join(workspace, nxPath), "utf8"));
  nx.pluginsConfig["@repo/emoji-project-json"] = { ...nx.pluginsConfig["@repo/emoji-project-json"], dependencyResolutionAuthority: { module: `./${library}/🟨️.mjs`, export: "dependencyResolutionAuthority" } };
  propose(nxPath, JSON.stringify(nx, null, 2) + "\n");
  const bootstrapPath = `${caching}/🚀️bootstrap/📜️script.ts`, bootstrapBefore = readFileSync(join(workspace, bootstrapPath), "utf8");
  const bootstrapAuthored = bootstrapBefore.replace('/** 🛠️ Loads acquisition only when the selected Nx installation needs it. */', '/** 🛠️ Loads the owner of the current immutable Nx recipe and patch identity. */').replace('let tooling: { cli: string; modulePath: string } | undefined;', 'let tooling: { cli: string; modulePath: string };').replace('if (!existsSync(join(this.root, "node_modules/nx/package.json")) || segments.some(argument => /(?:^|[:,=])(?:deps-javascript|deps-js-all|setup)(?:$|[,:])/.test(argument))) {', '{').replace('const nxCli = tooling?.cli ?? createRequire(join(this.root, existsSync(join(this.root, ".nx/installation/package.json")) ? ".nx/installation/package.json" : "package.json")).resolve("nx/bin/nx.js");', 'const nxCli = tooling.cli;').replace('...(tooling ? { NODE_PATH: tooling.modulePath } : {})', 'NODE_PATH: tooling.modulePath');
  propose(bootstrapPath, bootstrapAuthored);
  const graphPath = "dist/src/project-graph/build-project-graph.js", cachePath = "dist/src/project-graph/nx-deps-cache.js";
  const graphBefore = readFileSync(join(workspace, "node_modules/nx", graphPath), "utf8"), cacheBefore = readFileSync(join(workspace, "node_modules/nx", cachePath), "utf8");
  const snippet = readFileSync(join(import.meta.dir, "🏛️graph/NX-DEPENDENCY-AUTHORITY-SNIPPET.md"), "utf8").split("```js\n")[1]!.split("\n```")[0]!;
  const graphAuthored = graphBefore.replace('async function buildProjectGraphUsingProjectFileMap(', `${snippet}\n\nasync function buildProjectGraphUsingProjectFileMap(`).replace('const externalNodesHash = hashExternalNodes(externalNodes);', 'const externalNodesHash = hashExternalNodes(externalNodes);\n    const dependencyResolutionAuthority = await readDependencyResolutionAuthority(nxJson, projects);').replace('projects, nxJson, rootTsConfig, externalNodesHash)', 'projects, nxJson, rootTsConfig, externalNodesHash, dependencyResolutionAuthority)').replace('projectGraphVersion, plugins, sourceMap);\n        projectFileMapCache', 'projectGraphVersion, plugins, sourceMap);\n        if (await readDependencyResolutionAuthority(nxJson, projects) !== dependencyResolutionAuthority) throw new Error("Dependency resolution authority changed during graph construction");\n        projectFileMapCache').replace('fileMap, rootTsConfig, externalNodesHash);', 'fileMap, rootTsConfig, externalNodesHash, dependencyResolutionAuthority);');
  const cacheAuthored = cacheBefore.replace('function createProjectFileMapCache(nxJson, packageJsonDeps, fileMap, tsConfig, externalNodesHash)', 'function createProjectFileMapCache(nxJson, packageJsonDeps, fileMap, tsConfig, externalNodesHash, dependencyResolutionAuthority)').replace('        externalNodesHash,\n', '        externalNodesHash,\n        dependencyResolutionAuthority,\n').replace('function shouldRecomputeWholeGraph(cache, packageJsonDeps, projects, nxJson, tsConfig, externalNodesHash) {', 'function shouldRecomputeWholeGraph(cache, packageJsonDeps, projects, nxJson, tsConfig, externalNodesHash, dependencyResolutionAuthority) {\n    if (cache.dependencyResolutionAuthority !== dependencyResolutionAuthority) return true;');
  const diff = (path: string, before: string, after: string): string => {
    const a = before.trimEnd().split("\n"), b = after.trimEnd().split("\n");
    let first = 0, tail = 0;
    while (first < a.length && first < b.length && a[first] === b[first]) first++;
    while (tail < a.length - first && tail < b.length - first && a[a.length - tail - 1] === b[b.length - tail - 1]) tail++;
    if (first === a.length && first === b.length) throw Error(`No-op patch ${path}`);
    const start = Math.max(0, first - 3), aEnd = Math.min(a.length, a.length - tail + 3), bEnd = Math.min(b.length, b.length - tail + 3);
    return `diff --git a/${path} b/${path}\n--- a/${path}\n+++ b/${path}\n@@ -${start + 1},${aEnd - start} +${start + 1},${bEnd - start} @@\n${a.slice(start, first).map(line => " " + line).join("\n")}\n${a.slice(first, a.length - tail).map(line => "-" + line).join("\n")}\n${b.slice(first, b.length - tail).map(line => "+" + line).join("\n")}\n${a.slice(a.length - tail, aEnd).map(line => " " + line).join("\n")}\n`;
  };
  const patchPath = `${caching}/🩹️patches/nx@23.2.0.patch`, patchBefore = readFileSync(join(workspace, patchPath), "utf8");
  if (patchBefore.includes(`diff --git a/${graphPath}`) || patchBefore.includes(`diff --git a/${cachePath}`)) throw Error("Graph patch already has another owner; rebase explicitly");
  propose(patchPath, patchBefore.trimEnd() + "\n" + diff(graphPath, graphBefore, graphAuthored) + diff(cachePath, cacheBefore, cacheAuthored));
  for (const row of rows) if (row.before === row.authored) throw Error(`No-op retained production row ${row.path}`);
  let ordinal = 1;
  while (existsSync(join(output, `retained-graph-production-full-before-authored-${ordinal}.json`))) ordinal++;
  writeFileSync(join(output, `retained-graph-production-full-before-authored-${ordinal}.json`), JSON.stringify({ workspace, rows, installedBase: [{ path: graphPath, before: graphBefore, beforeHash: hash(graphBefore), authored: graphAuthored, authoredHash: hash(graphAuthored) }, { path: cachePath, before: cacheBefore, beforeHash: hash(cacheBefore), authored: cacheAuthored, authoredHash: hash(cacheAuthored) }] }, null, 2));
  console.log(`[DEBUG] prepared ${rows.length} retained graph production rows with full current before/inverses; not mounted or installed`);
} else if (process.argv[2] === "mount-retained-production") {
  const ordinal = process.argv[3] ?? "1", capture = JSON.parse(readFileSync(join(output, `retained-graph-production-full-before-authored-${ordinal}.json`), "utf8"));
  for (const row of capture.rows) if (readFileSync(join(workspace, row.path), "utf8") !== row.before) throw Error(`Source guard changed: ${row.path}`);
  for (const row of capture.rows) { const path = join(workspace, row.path); if (readFileSync(path, "utf8") !== row.before) throw Error(`Immediate source guard changed: ${row.path}`); writeFileSync(path, row.authored); }
  writeFileSync(join(output, `retained-graph-production-mounted-post-${ordinal}.json`), JSON.stringify({ rows: capture.rows.map(row => ({ path: row.path, authoredHash: row.authoredHash, currentHash: hash(readFileSync(join(workspace, row.path), "utf8")) })) }, null, 2));
  console.log(`[DEBUG] mounted ${capture.rows.length} retained graph production rows under full byte guards; installation remains owner-driven through ordinary Nx bootstrap`);
} else if (process.argv[2] === "prepare-authority-tests") {
  const schemaPath = `${caching}/🧬️schema/🔗️import-edges/🔣️.json`, fixturePath = `${caching}/🧫️fixtures/import-edges/🔁️context/🔣️.json`;
  const currentSchema = JSON.parse(readFileSync(join(workspace, schemaPath), "utf8")), currentFixture = JSON.parse(readFileSync(join(workspace, fixturePath), "utf8"));
  currentSchema.required.push("graphAuthority", "lockScopes");
  currentSchema.properties.graphAuthority = object({ module: string, export: string });
  currentSchema.properties.lockScopes = { type: "array", minItems: 1, items: object({ id: string, file: string, source: string, before: string, after: string, beforeTarget: string, afterTarget: string }) };
  currentFixture.graphAuthority = { module: `./${library}/🟨️.mjs`, export: "dependencyResolutionAuthority" };
  const lock = { lockfileVersion: 3, workspaces: { "": { name: "scope-fixture" }, caller: { name: "caller" } }, packages: { caller: ["caller@workspace:caller"], "semio-import-fixture-lock": ["semio-import-fixture-lock@2.0.0", "", {}], "caller/semio-import-fixture-lock": ["semio-import-fixture-lock@1.0.0", "", {}] } };
  const before = JSON.stringify(lock, null, 2);
  lock.packages.caller[0] = "caller@workspace:spare";
  currentFixture.lockScopes = [{ id: "source-scoped-lock-with-stable-external-nodes", file: "caller/index.ts", source: "import 'semio-import-fixture-lock';", before, after: JSON.stringify(lock, null, 2), beforeTarget: "npm:caller/semio-import-fixture-lock", afterTarget: "npm:semio-import-fixture-lock" }];
  propose(schemaPath, JSON.stringify(currentSchema, null, 2) + "\n");
  propose(fixturePath, JSON.stringify(currentFixture, null, 2) + "\n");
  propose(`${caching}/🧪️tests/🔗️import-edges/🏛️graph/🔑️authority/🟦️.ts`, readFileSync(join(import.meta.dir, "🏛️graph/🔑️authority/🟦️.ts"), "utf8"));
  const parserLaws = `${inference}/🧪️tests/🔍️imports/🟦️.ts`;
  propose(parserLaws, readFileSync(join(workspace, parserLaws), "utf8") + '\nimport "../../../🧪️tests/🔗️import-edges/🏛️graph/🔑️authority/🟦️.ts";\n');
  writeFileSync(join(output, "retained-authority-test-only-before-authored-1.json"), JSON.stringify({ workspace, rows }, null, 2));
  console.log(`[DEBUG] prepared ${rows.length} schema-first retained authority test-only rows`);
} else if (process.argv[2] === "mount-authority-tests") {
  const capture = JSON.parse(readFileSync(join(output, "retained-authority-test-only-before-authored-1.json"), "utf8"));
  for (const row of capture.rows) { const path = join(workspace, row.path); if ((existsSync(path) ? readFileSync(path, "utf8") : null) !== row.before) throw Error(`Source guard changed: ${row.path}`); }
  for (const row of capture.rows) { const path = join(workspace, row.path); if ((existsSync(path) ? readFileSync(path, "utf8") : null) !== row.before) throw Error(`Immediate source guard changed: ${row.path}`); mkdirSync(dirname(path), { recursive: true }); writeFileSync(path, row.authored); }
  writeFileSync(join(output, "retained-authority-test-only-mounted-post-1.json"), JSON.stringify({ rows: capture.rows.map(row => ({ path: row.path, authoredHash: row.authoredHash, currentHash: hash(readFileSync(join(workspace, row.path), "utf8")) })) }, null, 2));
  console.log(`[DEBUG] mounted ${capture.rows.length} retained authority test-only rows; broader production remains held`);
} else if (process.argv[2] === "amend-retained-production") {
  const capture = JSON.parse(readFileSync(join(output, "retained-graph-production-full-before-authored-1.json"), "utf8"));
  const row = capture.rows.find(row => row.path.endsWith("nx@23.2.0.patch"));
  const path = join(workspace, row.path), before = readFileSync(path, "utf8");
  if (before !== row.authored) throw Error(`Source guard changed: ${row.path}`);
  const marker = "diff --git a/dist/src/project-graph/build-project-graph.js";
  const offset = before.indexOf(marker), prefix = before.slice(0, offset), patch = before.slice(offset);
  const clearing = "    for (const file of [...Object.values(fileMap.projectFileMap).flat(), ...fileMap.nonProjectFiles]) delete file.deps;";
  const authored = prefix + patch.replace(/(@@ -\d+,\d+ \+\d+,)(\d+)( @@)/, (_, a, count, c) => a + (Number(count) + 1) + c).replace("+    const dependencyResolutionAuthority = await readDependencyResolutionAuthority(nxJson, projects);", `+${clearing}\n+    const dependencyResolutionAuthority = await readDependencyResolutionAuthority(nxJson, projects);`);
  if (authored === before || offset < 0) throw Error("Incoming dependency admission patch was not amended");
  const graph = capture.installedBase.find(row => row.path.endsWith("build-project-graph.js"));
  const installedAuthored = graph.authored.replace("    const dependencyResolutionAuthority = await readDependencyResolutionAuthority(nxJson, projects);", `${clearing}\n    const dependencyResolutionAuthority = await readDependencyResolutionAuthority(nxJson, projects);`);
  writeFileSync(join(output, "retained-graph-production-amend-before-authored-2.json"), JSON.stringify({ rows: [{ path: row.path, before, beforeHash: hash(before), authored, authoredHash: hash(authored) }], installedBase: [{ ...graph, authored: installedAuthored, authoredHash: hash(installedAuthored) }, capture.installedBase.find(row => row.path.endsWith("nx-deps-cache.js"))] }, null, 2));
  if (readFileSync(path, "utf8") !== before) throw Error(`Immediate source guard changed: ${row.path}`);
  writeFileSync(path, authored);
  console.log("[DEBUG] incoming resolved dependency admission amendment mounted under full byte guard; provisioning remains owner-driven");
} else if (process.argv[2] === "amend-recipe-tests") {
  const schemaPath = `${caching}/🧬️schema/🔗️import-edges/🔣️.json`, fixturePath = `${caching}/🧫️fixtures/import-edges/🔁️context/🔣️.json`, lawPath = `${caching}/🧪️tests/🔗️import-edges/🏛️graph/🔑️authority/🟦️.ts`;
  const previous = JSON.parse(readFileSync(join(output, "retained-authority-test-only-before-authored-1.json"), "utf8"));
  for (const path of [schemaPath, fixturePath, lawPath]) if (readFileSync(join(workspace, path), "utf8") !== previous.rows.find(row => row.path === path).authored) throw Error(`Source guard changed: ${path}`);
  const schema = JSON.parse(readFileSync(join(workspace, schemaPath), "utf8")), fixture = JSON.parse(readFileSync(join(workspace, fixturePath), "utf8"));
  schema.required.push("authorityInputs");
  schema.properties.authorityInputs = { type: "array", minItems: 3, items: object({ id: string, files: { type: "object", additionalProperties: string }, path: string, after: string }) };
  const recipe = `${caching}/🚀️bootstrap/🛠️tools`, patch = `${caching}/🩹️patches/nx@23.2.0.patch`, manifest = { dependencies: { nx: "23.2.0" }, semio: { toolPatches: { "nx@23.2.0": patch } } };
  const files = { [`${recipe}/package.json`]: JSON.stringify(manifest), [`${recipe}/bun.lock`]: "first-lock", [patch]: "first-owned-patch" };
  fixture.authorityInputs = [{ id: "immutable-tool-recipe-manifest", files, path: `${recipe}/package.json`, after: JSON.stringify({ ...manifest, dependencies: { nx: "23.3.0" } }) }, { id: "immutable-tool-recipe-lock", files, path: `${recipe}/bun.lock`, after: "next-lock" }, { id: "immutable-tool-owned-patch", files, path: patch, after: "next-owned-patch" }];
  propose(schemaPath, JSON.stringify(schema, null, 2) + "\n");
  propose(fixturePath, JSON.stringify(fixture, null, 2) + "\n");
  propose(lawPath, readFileSync(join(import.meta.dir, "🏛️graph/🔑️authority/🟦️.ts"), "utf8"));
  writeFileSync(join(output, "retained-recipe-test-only-before-authored-1.json"), JSON.stringify({ workspace, rows }, null, 2));
  for (const row of rows) { const path = join(workspace, row.path); if (readFileSync(path, "utf8") !== row.before) throw Error(`Immediate source guard changed: ${row.path}`); writeFileSync(path, row.authored); }
  console.log("[DEBUG] three schema-first immutable recipe authority cases mounted within existing 17th law under byte guards");
} else if (process.argv[2] === "prepare-recipe-production") {
  const capture = JSON.parse(readFileSync(join(output, "retained-graph-production-full-before-authored-1.json"), "utf8")), amendment = JSON.parse(readFileSync(join(output, "retained-graph-production-amend-before-authored-2.json"), "utf8"));
  const pluginPath = `${library}/🟨️.mjs`, pluginBefore = readFileSync(join(workspace, pluginPath), "utf8");
  if (pluginBefore !== capture.rows.find(row => row.path === pluginPath).authored) throw Error(`Source guard changed: ${pluginPath}`);
  const recipe = `${caching}/🚀️bootstrap/🛠️tools`;
  const contribution = `  const recipe = join(workspaceRoot, ${JSON.stringify(recipe)}), manifest = join(recipe, "package.json");\n  for (const path of [join(workspaceRoot, ${JSON.stringify(`${caching}/🚀️bootstrap/📜️script.ts`)}), join(recipe, "📜️script.ts"), manifest, join(recipe, "bun.lock")]) source(path);\n  if (existsSync(manifest)) for (const path of [...new Set(Object.values(JSON.parse(readPhysicalSource(manifest)).semio?.toolPatches ?? {}))].sort()) {\n    if (typeof path !== "string") throw new Error("Invalid tooling authority patch path");\n    const file = resolve(workspaceRoot, path), local = nxPath(relative(workspaceRoot, file));\n    if (isAbsolute(local) || local === ".." || local.startsWith("../")) throw new Error("Tooling authority patch must belong to this workspace");\n    source(file);\n  }\n`;
  const marker = '  if (existsSync(join(workspaceRoot, "bun.lock"))) { readBunLockGraph(workspaceRoot); hash.update(BUN_LOCK_CACHE.hash); }';
  if (!pluginBefore.includes(marker)) throw Error("Authority owner marker missing");
  propose(pluginPath, pluginBefore.replace(marker, contribution + marker));
  const patch = amendment.rows[0], patchBefore = readFileSync(join(workspace, patch.path), "utf8");
  if (patchBefore !== patch.authored) throw Error(`Source guard changed: ${patch.path}`);
  const beforeClearing = "    for (const file of [...Object.values(fileMap.projectFileMap).flat(), ...fileMap.nonProjectFiles]) delete file.deps;";
  const clearing = "    fileMap = { ...fileMap, projectFileMap: Object.fromEntries(Object.entries(fileMap.projectFileMap).map(([project, files]) => [project, files.map(({ deps, ...file }) => file)])), nonProjectFiles: fileMap.nonProjectFiles.map(({ deps, ...file }) => file) }; storedFileMap = fileMap;";
  if (!patchBefore.includes("+" + beforeClearing)) throw Error("Incoming facts admission marker missing");
  propose(patch.path, patchBefore.replace("+" + beforeClearing, "+" + clearing));
  const installedBase = amendment.installedBase.map(row => row.path.endsWith("build-project-graph.js") ? { ...row, authored: row.authored.replace(beforeClearing, clearing), authoredHash: hash(row.authored.replace(beforeClearing, clearing)) } : row);
  writeFileSync(join(output, "retained-recipe-production-before-authored-3.json"), JSON.stringify({ workspace, rows, installedBase }, null, 2));
  console.log("[DEBUG] two recipe and immutable incoming-facts production amendments prepared with full inverse capture; unmounted");
} else if (process.argv[2] === "mount-recipe-production") {
  const capture = JSON.parse(readFileSync(join(output, "retained-recipe-production-before-authored-3.json"), "utf8"));
  for (const row of capture.rows) if (readFileSync(join(workspace, row.path), "utf8") !== row.before) throw Error(`Source guard changed: ${row.path}`);
  for (const row of capture.rows) { const path = join(workspace, row.path); if (readFileSync(path, "utf8") !== row.before) throw Error(`Immediate source guard changed: ${row.path}`); writeFileSync(path, row.authored); }
  console.log("[DEBUG] recipe authority and immutable incoming source facts mounted under full guards; no installed package bytes written");
} else if (process.argv[2] === "capture-retained-validation") {
  const run = process.argv[3], log = readFileSync(join(output, `registered-retained-${run}.log`), "utf8").replace(/\u001b\[[0-9;]*m/g, "");
  const tests = JSON.parse(readFileSync(join(workspace, `${caching}/🧫️fixtures/import-edges/🔁️context/🔣️.json`), "utf8")).contexts.length + 8;
  if (!new RegExp(`${tests} pass`).test(log) || !/0 fail/.test(log)) throw Error("Full registered gate is not GREEN");
  const names = ["test-only-schema-before-authored-1.json", "test-only-oracle-amend-before-authored-2.json", "test-only-oracle-amend-before-authored-3.json", "production-full-before-authored-1.json", "retained-graph-test-only-before-authored-1.json", "retained-graph-test-only-amend-before-authored-2.json", "retained-authority-test-only-before-authored-1.json", "retained-graph-production-full-before-authored-1.json", "retained-graph-test-only-amend-before-authored-3.json", "retained-graph-test-only-amend-before-authored-4.json", "retained-graph-production-amend-before-authored-2.json", "retained-recipe-test-only-before-authored-1.json", "retained-graph-test-only-amend-before-authored-5.json", "retained-recipe-production-before-authored-3.json"];
  names.push("retained-tooling-isolation-before-authored-1.json");
  names.push("runtime-core-test-only-before-authored-1.json", "runtime-core-test-only-amend-before-authored-2.json", "runtime-core-test-only-amend-before-authored-3.json", "runtime-core-production-before-authored-2.json");
  names.push("runtime-core-oracle-amend-before-authored-4.json");
  const admitted = new Map<string, any>();
  for (const name of names) for (const row of JSON.parse(readFileSync(join(output, name), "utf8")).rows) admitted.set(row.path, { ...row, originalBefore: admitted.get(row.path)?.originalBefore ?? row.before });
  const rows = [...admitted.values()].map(row => { const current = readFileSync(join(workspace, row.path), "utf8"); return { path: row.path, before: row.originalBefore, authored: row.authored, authoredHash: row.authoredHash, current, currentHash: hash(current), gap: current !== row.authored }; });
  const selectedDirectory = realpathSync(join(workspace, ".nx/installation")), toolingMarker = JSON.parse(readFileSync(join(selectedDirectory, ".semio-nx-tooling.json"), "utf8"));
  const installedBase = JSON.parse(readFileSync(join(output, "retained-recipe-production-before-authored-3.json"), "utf8")).installedBase;
  const installed = installedBase.map(row => { const path = join(selectedDirectory, "node_modules/nx", row.path), current = readFileSync(path, "utf8"); return { path, authoredHash: row.authoredHash, currentHash: hash(current), gap: current !== row.authored }; });
  const equality = JSON.parse(readFileSync(join(output, `retained-${run}/ordinary-retained-graph-equality.json`), "utf8"));
  const map = readFileSync(join(workspace, ".nx/workspace-data/file-map.json"), "utf8"), graph = readFileSync(join(workspace, ".nx/workspace-data/project-graph.json"), "utf8");
  const snapshotGaps = Number(hash(map) !== equality.publishedMapHash) + Number(hash(graph) !== equality.publishedGraphHash);
  const sourceGaps = rows.filter(row => row.gap).length, installedGaps = installed.filter(row => row.gap).length;
  const result = { run, exit: 0, tests, pass: tests, fail: 0, assertions: Number(log.match(/(\d+) expect\(\) calls/)?.[1]), rows, sourceGaps, installed, installedGaps, selectedDirectory, toolingMarker, equality, snapshotGaps, daemonAdmission: "unverified; existing daemon preserved; this ordinary graph was published by the admitted immutable CLI with NX_DAEMON=false" };
  writeFileSync(join(output, `registered-retained-${run}-receipt.json`), JSON.stringify(result, null, 2));
  if (sourceGaps || installedGaps || snapshotGaps || equality.excess.length || equality.missing.length) throw Error(`Current source/publication gaps: ${sourceGaps}/${installedGaps}/${snapshotGaps}`);
  console.log(`[DEBUG] actual retained graph GREEN: ${tests}/${tests}; ${rows.length} current source rows, two installed patch rows, ordinary snapshot publication all zero gaps; ${equality.actualEdges} canonical edges`);
} else if (process.argv[2] === "mount-tooling-isolation") {
  const path = `${caching}/🚀️bootstrap/🛠️tools/📜️script.ts`, before = readFileSync(join(workspace, path), "utf8");
  const authored = before.replace('const hash = createHash("sha256").update(`${process.platform}\\0${process.arch}\\0${libc}\\0`);', 'const hash = createHash("sha256").update(readFileSync(import.meta.filename)).update(`${process.platform}\\0${process.arch}\\0${libc}\\0`);').replace('await runBun(["install", "--frozen-lockfile", "--ignore-scripts"], staging, signal);', 'await runBun(["install", "--frozen-lockfile", "--ignore-scripts", "--cache-dir", join(staging, ".bun-cache"), "--backend", "copyfile"], staging, signal);');
  if (authored === before || !authored.includes('readFileSync(import.meta.filename)') || !authored.includes('"--backend", "copyfile"')) throw Error("Tooling isolation markers missing");
  writeFileSync(join(output, "retained-tooling-isolation-before-authored-1.json"), JSON.stringify({ workspace, rows: [{ path, before, beforeHash: hash(before), authored, authoredHash: hash(authored) }] }, null, 2));
  if (readFileSync(join(workspace, path), "utf8") !== before) throw Error(`Source guard changed: ${path}`);
  writeFileSync(join(workspace, path), authored);
  console.log("[DEBUG] recipe-private Bun acquisition and copyfile isolation mounted with implementation identity and full inverse; existing caches preserved");
} else if (process.argv[2] === "mount-core-tests") {
  const fixturePath = `${caching}/🧫️fixtures/import-edges/🔁️context/🔣️.json`, contextPath = `${caching}/🧪️tests/🔗️import-edges/🔁️context/🟦️.ts`;
  const fixtureBefore = JSON.parse(readFileSync(join(output, "retained-recipe-test-only-before-authored-1.json"), "utf8")).rows.find(row => row.path === fixturePath).authored;
  const contextBefore = JSON.parse(readFileSync(join(output, "test-only-oracle-amend-before-authored-3.json"), "utf8")).rows.find(row => row.path === contextPath).authored;
  if (readFileSync(join(workspace, fixturePath), "utf8") !== fixtureBefore || readFileSync(join(workspace, contextPath), "utf8") !== contextBefore) throw Error("Core test source guard changed");
  const fixture = JSON.parse(fixtureBefore);
  fixture.contexts.push({ id: "runtime-package-core-collision", source: 'import "node:fs"; import "fs"; import "ws"; import "undici";', phases: [phase("caller/index.ts", { coreAlias: "provider" }, ["npm:undici@7", "npm:ws@8"], { fs: "coreAlias" }, { fs: "1", ws: "8", undici: "7" }), phase("caller/index.ts", { coreAlias: "provider" }, ["npm:undici@8", "npm:ws@9"], { fs: "coreAlias" }, { fs: "2", ws: "9", undici: "8" })] });
  fixture.contexts.push({ id: "runtime-core-identity", source: 'import "node:fs"; import "fs";', phases: [phase("caller/index.ts", { coreAlias: "provider" }, [], { fs: "coreAlias" }, { fs: "1" }), phase("caller/index.ts", { coreAlias: "provider" }, [], { fs: "coreAlias" }, { fs: "2" })] });
  propose(fixturePath, JSON.stringify(fixture, null, 2) + "\n");
  propose(contextPath, readFileSync(join(import.meta.dir, "🔁️context/🟦️.ts"), "utf8"));
  writeFileSync(join(output, "runtime-core-test-only-before-authored-1.json"), JSON.stringify({ workspace, rows }, null, 2));
  for (const row of rows) { const path = join(workspace, row.path); if (readFileSync(path, "utf8") !== row.before) throw Error(`Immediate source guard changed: ${row.path}`); writeFileSync(path, row.authored); }
  console.log("[DEBUG] two runtime core/package collision cases mounted with Node installed-Nx oracle and callback-free core assertions; full registered corpus is now 19 laws");
} else if (process.argv[2] === "prepare-core-production") {
  const path = `${library}/🟨️.mjs`, before = readFileSync(join(workspace, path), "utf8"), previous = JSON.parse(readFileSync(join(output, "retained-recipe-production-before-authored-3.json"), "utf8")).rows.find(row => row.path === path).authored;
  if (before !== previous) throw Error(`Source guard changed: ${path}`);
  const helper = '/** 🧭️ Uses bare and explicit core loader identity so bundled packages retain their package identity. */\nfunction coreModuleImport(specifier) {\n  try {\n    const loader = createRequire(import.meta.url);\n    if (specifier.startsWith("node:")) return loader.resolve(specifier) === specifier;\n    if (!isBuiltin(specifier)) return false;\n    const core = "node:" + specifier;\n    return loader.resolve(specifier) === specifier && loader.resolve(core) === core;\n  } catch { return false; }\n}\n\n';
  const authored = before.replace('/** 🔗️ Resolves immutable runtime import strings against current ownership, package names, and lock scope. */', helper + '/** 🔗️ Resolves immutable runtime import strings against current ownership, package names, and lock scope. */').replace('    const workspaceTarget = byPackage.get(packageName);', '    if (coreModuleImport(packageName)) continue;\n    const workspaceTarget = byPackage.get(packageName);').replace('    if (isBuiltin(packageName)) continue;\n', '');
  if (before === authored) throw Error("Core admission marker missing");
  propose(path, authored);
  writeFileSync(join(output, "runtime-core-production-before-authored-2.json"), JSON.stringify({ workspace, rows }, null, 2));
  console.log("[DEBUG] canonical core namespace loader admission prepared with full inverse; unmounted");
} else if (process.argv[2] === "mount-core-production") {
  const capture = JSON.parse(readFileSync(join(output, "runtime-core-production-before-authored-2.json"), "utf8"));
  for (const row of capture.rows) { const path = join(workspace, row.path); if (readFileSync(path, "utf8") !== row.before) throw Error(`Source guard changed: ${row.path}`); writeFileSync(path, row.authored); }
  console.log("[DEBUG] canonical core namespace admission mounted after actual registered RED; one builtin admission owner, no copied classifier");
} else if (process.argv[2] === "amend-core-tests") {
  const previous = JSON.parse(readFileSync(join(output, "runtime-core-test-only-before-authored-1.json"), "utf8"));
  for (const row of previous.rows) if (readFileSync(join(workspace, row.path), "utf8") !== row.authored) throw Error(`Source guard changed: ${row.path}`);
  const fixturePath = `${caching}/🧫️fixtures/import-edges/🔁️context/🔣️.json`, contextPath = `${caching}/🧪️tests/🔗️import-edges/🔁️context/🟦️.ts`, fixture = JSON.parse(readFileSync(join(workspace, fixturePath), "utf8"));
  for (const row of fixture.contexts.filter(row => row.id.startsWith("runtime-"))) for (const phase of row.phases) { phase.locked = Object.fromEntries(Object.entries(phase.locked).map(([name, version]) => [name, version + ".0.0"])); phase.targets = phase.targets.map(target => target + ".0.0"); }
  propose(fixturePath, JSON.stringify(fixture, null, 2) + "\n");
  propose(contextPath, readFileSync(join(import.meta.dir, "🔁️context/🟦️.ts"), "utf8"));
  writeFileSync(join(output, "runtime-core-test-only-amend-before-authored-2.json"), JSON.stringify({ workspace, rows }, null, 2));
  for (const row of rows) { const path = join(workspace, row.path); if (readFileSync(path, "utf8") !== row.before) throw Error(`Immediate source guard changed: ${row.path}`); writeFileSync(path, row.authored); }
  console.log("[DEBUG] Node oracle fixture package entries and exact semver corrected under full source guards; production unchanged");
} else if (process.argv[2] === "amend-explicit-core-tests") {
  const path = `${caching}/🧫️fixtures/import-edges/🔁️context/🔣️.json`, before = readFileSync(join(workspace, path), "utf8"), previous = JSON.parse(readFileSync(join(output, "runtime-core-test-only-amend-before-authored-2.json"), "utf8")).rows.find(row => row.path === path).authored;
  if (before !== previous) throw Error(`Source guard changed: ${path}`);
  const fixture = JSON.parse(before), vector = fixture.contexts.find(row => row.id === "runtime-package-core-collision");
  vector.source += ' import "test"; import "sqlite";';
  for (const [index, phase] of vector.phases.entries()) { const version = `${index + 1}.0.0`; Object.assign(phase.locked, { test: version, sqlite: version }); phase.targets.push(`npm:test@${version}`, `npm:sqlite@${version}`); phase.targets.sort(); }
  propose(path, JSON.stringify(fixture, null, 2) + "\n");
  writeFileSync(join(output, "runtime-core-test-only-amend-before-authored-3.json"), JSON.stringify({ workspace, rows }, null, 2));
  if (readFileSync(join(workspace, path), "utf8") !== before) throw Error(`Immediate source guard changed: ${path}`);
  writeFileSync(join(workspace, path), rows[0].authored);
  console.log("[DEBUG] explicit-only core namespace collision vector added within existing 19-law corpus; production unchanged");
} else if (process.argv[2] === "amend-core-oracle") {
  const path = `${caching}/🧪️tests/🔗️import-edges/🔁️context/🟦️.ts`, before = readFileSync(join(workspace, path), "utf8"), previous = JSON.parse(readFileSync(join(output, "runtime-core-test-only-amend-before-authored-2.json"), "utf8")).rows.find(row => row.path === path).authored;
  if (before !== previous) throw Error(`Source guard changed: ${path}`);
  propose(path, readFileSync(join(import.meta.dir, "🔁️context/🟦️.ts"), "utf8"));
  writeFileSync(join(output, "runtime-core-oracle-amend-before-authored-4.json"), JSON.stringify({ workspace, rows }, null, 2));
  if (readFileSync(join(workspace, path), "utf8") !== before) throw Error(`Immediate source guard changed: ${path}`);
  writeFileSync(join(workspace, path), rows[0].authored);
  console.log("[DEBUG] independent Node builtin API now supplies callback expectation directly, preserving explicit-only bare package names; production unchanged");
} else throw Error("Unknown proposal command");
