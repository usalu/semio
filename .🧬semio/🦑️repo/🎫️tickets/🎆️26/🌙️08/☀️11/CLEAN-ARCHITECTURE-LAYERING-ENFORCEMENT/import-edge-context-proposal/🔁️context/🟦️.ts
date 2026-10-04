import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { createRequire } from "node:module";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { spawnSync } from "node:child_process";
import Ajv from "ajv";
import { build } from "esbuild";
import ts from "typescript";

type Phase = { file: string; projects: Record<string, string>; packages: Record<string, string>; locked: Record<string, string>; targets: string[] };
type Vector = { id: string; source: string; phases: Phase[] };
type ModuleProvider = { moduleSourceImports(path: string, source: string): readonly string[]; commandSourceImports(path: string, source: string): readonly string[] };
const caching = resolve(import.meta.dir, "../../.."), require = createRequire(import.meta.url);
const fixture = JSON.parse(readFileSync(join(caching, "🧫️fixtures/import-edges/🔁️context/🔣️.json"), "utf8")) as { version: number; imports: { id: string; path: string; source: string; imports: string[] }[]; contexts: Vector[] };
const schema = JSON.parse(readFileSync(join(caching, "🧬️schema/🔗️import-edges/🔣️.json"), "utf8"));

/** 🔍️ Obtains runtime strings from independent esbuild resolution and TypeScript emission/scanning. */
async function oracle(path: string, source: string): Promise<string[]> {
  if (/\.(?:json|d\.[cm]?ts)$/.test(path)) return [];
  const emitted = ts.transpileModule(source, { fileName: path, compilerOptions: { module: ts.ModuleKind.Preserve, target: ts.ScriptTarget.ES2022, verbatimModuleSyntax: true } }).outputText;
  const tree = ts.createSourceFile(path, emitted, ts.ScriptTarget.Latest, true);
  const statements = tree.statements.filter(node => !(ts.isImportDeclaration(node) && node.importClause && !node.importClause.name && node.importClause.namedBindings && ts.isNamedImports(node.importClause.namedBindings) && !node.importClause.namedBindings.elements.length) && !(ts.isExportDeclaration(node) && node.exportClause && ts.isNamedExports(node.exportClause) && !node.exportClause.elements.length));
  const runtime = ts.createPrinter().printFile(ts.factory.updateSourceFile(tree, statements));
  const paths = new Set<string>();
  await build({ stdin: { contents: runtime, sourcefile: path, loader: "js" }, bundle: true, write: false, format: "esm", logLevel: "silent", plugins: [{ name: "module-source-oracle", setup(builder) { builder.onResolve({ filter: /.*/ }, args => { paths.add(args.path); return { path: args.path, external: true }; }); } }] });
  const emittedPaths = new Set(ts.preProcessFile(runtime, true, true).importedFiles.map(row => row.fileName));
  expect([...emittedPaths].sort(), `${path}: TypeScript emitted AST/scanner versus esbuild`).toEqual([...paths].sort());
  return [...paths].sort();
}

test("closed module source and resolution context corpus has unique identities", () => {
  expect(new Ajv({ strict: true }).validate(schema, fixture)).toBe(true);
  for (const rows of [fixture.imports, fixture.contexts]) expect(new Set(rows.map(row => row.id)).size).toBe(rows.length);
  console.log(`[DEBUG] import edge corpus: ${fixture.imports.length} source vectors, ${fixture.contexts.length} context replay laws`);
});

test("canonical module facts agree with independent runtime import oracles", async () => {
  const expected = await Promise.all(fixture.imports.map(row => oracle(row.path, row.source)));
  for (const [index, row] of fixture.imports.entries()) expect(expected[index], row.id).toEqual(row.imports);
  const provider: ModuleProvider = await import(pathToFileURL(join(caching, "📥️inference/🟨️.mjs")).href);
  expect(typeof provider.moduleSourceImports).toBe("function");
  for (const [index, row] of fixture.imports.entries()) {
    const facts = provider.moduleSourceImports(row.path, row.source);
    expect([...facts].sort(), row.id).toEqual(expected[index]!);
    expect([...provider.commandSourceImports(row.path, row.source)].sort(), row.id).toEqual(expected[index]!.filter(value => value.startsWith(".")));
    const mutated = [...facts, "unowned"];
    expect(provider.moduleSourceImports(row.path, row.source)).toEqual(facts);
    expect(provider.moduleSourceImports(row.path, row.source)).not.toEqual(mutated);
  }
  console.log(`[DEBUG] canonical module runtime import facts agree with esbuild and TypeScript: ${fixture.imports.length}`);
});

for (const vector of fixture.contexts) test(`immutable source facts resolve current context: ${vector.id}`, async () => {
  const { cacheInternals } = await import("../../../../🟨️.mjs");
  const artifact = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifact) throw Error("SEMIO_TEST_ARTIFACT_DIR is required");
  mkdirSync(artifact, { recursive: true });
  const root = mkdtempSync(join(artifact, `import-context-${vector.id}-`)), previous = process.env.NX_WORKSPACE_DATA_DIRECTORY;
  process.env.NX_WORKSPACE_DATA_DIRECTORY = join(root, ".nx/workspace-data");
  const hash = createHash("sha256").update(vector.source).digest("hex");
  let current: Phase, calls: { file: string; package: string }[] = [];
  const resolver = (file: string, name: string) => { calls.push({ file, package: name }); const version = current.locked[name]; return version ? `${name}@${version}` : undefined; };
  const oldCache = join(root, ".nx/workspace-data/emoji-import-edges");
  mkdirSync(oldCache, { recursive: true });
  if (vector.id === "old-resolved-cache-isolation") writeFileSync(join(oldCache, `${hash}.json`), JSON.stringify(["obsolete-resolved-target"]));
  try {
    for (const phase of vector.phases) {
      current = phase;
      const absolute = resolve(root, phase.file);
      mkdirSync(dirname(absolute), { recursive: true });
      writeFileSync(absolute, vector.source);
      const projects = Object.fromEntries(Object.entries(phase.projects).map(([name, root]) => [name, { name, root, targets: {} }]));
      const nodes = Object.fromEntries(Object.entries(projects).map(([name, project]) => [name, { name, type: "lib", data: { ...project, metadata: { js: { packageName: Object.entries(phase.packages).find(([, target]) => target === name)?.[0] ?? `__unused_${name}`, packageExports: { ".": "./index.ts", "./*": "./*.ts" }, isInPackageManagerWorkspaces: true } } } }]));
      const externalNodes: Record<string, unknown> = {};
      for (const [name, version] of Object.entries(phase.locked)) {
        const node = { name: `npm:${name}@${version}`, type: "npm", data: { packageName: name, version } };
        externalNodes[`npm:${name}`] = node;
        externalNodes[node.name] = node;
        const manifest = join(dirname(absolute), "node_modules", name, "package.json");
        mkdirSync(dirname(manifest), { recursive: true });
        writeFileSync(manifest, JSON.stringify({ name, version, main: "index.js" }));
        writeFileSync(join(dirname(manifest), "index.js"), "module.exports = {};\n");
      }
      const imported = await oracle(phase.file, vector.source);
      const resolved = spawnSync("node", ["-e", 'const fs=require("node:fs"),{TargetProjectLocator}=require(process.argv[1]),x=JSON.parse(fs.readFileSync(0,"utf8")),l=new TargetProjectLocator(x.nodes,x.externalNodes,new Map(),new Map()); const core=x.imported.filter(require("node:module").isBuiltin);console.log(JSON.stringify({core,targets:[...new Set(x.imported.map(s=>l.findProjectFromImport(s,s.startsWith(".")?x.file:x.absolute)).filter(Boolean))].sort()}));', require.resolve("nx/src/plugins/js/project-graph/build-dependencies/target-project-locator")], { input: JSON.stringify({ nodes, externalNodes, imported, file: phase.file, absolute }), encoding: "utf8", timeout: 10000 });
      expect(resolved.status, resolved.stderr).toBe(0);
      const { targets: expected, core } = JSON.parse(resolved.stdout) as { targets: string[]; core: string[] };
      expect(expected, `${vector.id}: installed Nx resolution`).toEqual(phase.targets);
      const projectFiles = { caller: [{ file: phase.file, hash }] }, byPackage = new Map(Object.entries(phase.packages)), locked = { resolveImport: resolver, externalNodes };
      for (const temperature of ["cold", "warm"]) {
        calls = [];
        const targets = new Set<string>();
        await cacheInternals.collectImportEdges(root, projectFiles, projects, byPackage, locked, (_source: string, target: string) => { targets.add(target); });
        expect([...targets].sort(), `${vector.id}: ${temperature}, ${phase.file}`).toEqual(expected);
        expect(calls.length, `${vector.id}: resolution callbacks run only for actual package imports`).toBe(imported.filter(value => !core.includes(value) && !value.startsWith(".") && !byPackage.has(value.startsWith("@") ? value.split("/").slice(0, 2).join("/") : value.split("/")[0])).length);
        expect(calls.every(call => call.file === phase.file)).toBe(true);
      }
      if (!Object.keys(phase.locked).length) {
        for (const [name, project] of Object.entries(projects)) {
          const manifest = join(root, project.root, "package.json");
          mkdirSync(dirname(manifest), { recursive: true });
          writeFileSync(manifest, JSON.stringify({ name: Object.entries(phase.packages).find(([, target]) => target === name)?.[0] ?? `__unused_${name}` }));
        }
        const fileMap = { projectFileMap: projectFiles, nonProjectFiles: [] };
        const edges = await cacheInternals.createDependenciesImplementation({ analyzeLockfile: false }, { workspaceRoot: root, projects, fileMap, filesToProcess: fileMap, externalNodes: {}, nxJsonConfiguration: {} });
        expect([...new Set(edges.map((edge: { target: string }) => edge.target))].sort(), `${vector.id}: actual current project manifests`).toEqual(expected);
      }
    }
    if (vector.id === "old-resolved-cache-isolation") expect(readFileSync(join(oldCache, `${hash}.json`), "utf8")).toBe(JSON.stringify(["obsolete-resolved-target"]));
    console.log(`[DEBUG] current import context ${vector.id}: ${vector.phases.length} phases, cold/warm, installed Nx oracle; old resolved cache ignored`);
  } finally {
    if (previous === undefined) delete process.env.NX_WORKSPACE_DATA_DIRECTORY;
    else process.env.NX_WORKSPACE_DATA_DIRECTORY = previous;
    rmSync(root, { recursive: true, force: true });
  }
}, 120000);
