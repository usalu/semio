import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { lstatSync, readFileSync } from "node:fs";
import { dirname, join, posix, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { build as esbuild } from "esbuild";
import { applyEdits, modify, parse } from "jsonc-parser";
import ts from "typescript";

if (process.argv[2] !== "check") throw new Error("Expected check");
delete process.env.FORCE_COLOR;
process.env.NO_COLOR = "1";
const testRoot = dirname(fileURLToPath(import.meta.url)), ticketRoot = dirname(testRoot), repoRoot = resolve(ticketRoot, "../../../../../../../");
const library = join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const discovery = await import(join(library, "🔍️discovery/🟦️component.ts")), producer = await import(join(repoRoot, "📜️script.ts"));
const inputText = readFileSync(join(testRoot, "🔣️.json"), "utf8"), input = JSON.parse(inputText), taxonomy = discovery.loadCatalogTaxonomy();
const catalogContract = taxonomy.semanticPackageProjectionContracts["nested-cargo-packages-v1"], catalogPath = join(repoRoot, catalogContract.authorityCatalogPath);
const catalogBytes = readFileSync(catalogPath, "utf8"), catalog = discovery.semanticPackageProjectionCatalog(repoRoot, taxonomy)!, owner = catalog.packages.find((row: any) => row.id === input.packageId)!;
const contract = taxonomy.generatorContracts[input.generatorId], base = discovery.registryCatalogInputView(repoRoot, taxonomy);
const hash = (value: string | Uint8Array) => createHash("sha256").update(value).digest("hex");
const order = (left: string, right: string) => Buffer.compare(Buffer.from(left), Buffer.from(right));
const bytes = new Map<string, string>();
const read = (path: string): string => {
  if (!bytes.has(path)) { assert.equal(base.kind(path), "file", path); bytes.set(path, base.readText(path)); }
  return bytes.get(path)!;
};
const view = { ...base, readText: read };
const nodes = owner.mappings.map((mapping: any) => ({ path: mapping.sourcePath, nodeKind: "file", content: read(mapping.sourcePath) }));
const facts = { packageId: owner.id, layout: "source", nodes, cargoWorkspaceContent: read("Cargo.toml"), nodeWorkspaceContent: read("package.json") };
const proposal = structuredClone(catalog), proposedOwner = proposal.packages.find((row: any) => row.id === input.packageId)!;
for (const row of input.mappings) Object.assign(proposedOwner.mappings[row.index], { sourceHash: row.afterHash, sourceSize: row.afterSize });
const profile = structuredClone(contract.packageGeneration.browserProfile);
profile.sourceModulePaths = [...new Set([...profile.sourceModulePaths, input.addedModule.path])].sort(order);
const outputStates = () => contract.outputRoots.map((row: any) => [row.path, base.kind(row.path)]);
const outputsBefore = outputStates(), results: { name: string; pass: boolean; error?: string }[] = [], started = performance.now();
const check = async (name: string, run: () => unknown | Promise<unknown>) => {
  try { await run(); results.push({ name, pass: true }); }
  catch (error) { results.push({ name, pass: false, error: error instanceof Error ? error.message : String(error) }); }
  process.stderr.write(`[wgpu-current-source] ${name}: ${results.at(-1)!.pass ? "pass" : "fail"}\n`);
};

await check("language-neutral input and exact current source census", () => {
  assert.deepEqual(parse(inputText), input);
  const paths = execFileSync("git", ["--literal-pathspecs", "ls-files", "--cached", "--others", "--exclude-standard", "-z", "--", owner.sourceRoot], { cwd: repoRoot, encoding: "utf8", timeout: 10_000 }).split("\0").filter(Boolean).sort(order);
  assert.equal(paths.length, input.sourceLeafCount);
  assert.deepEqual(paths, owner.mappings.map((row: any) => row.sourcePath).sort(order));
  for (const row of input.mappings) {
    const mapping = owner.mappings[row.index], content = read(mapping.sourcePath);
    assert.equal(mapping.sourcePath, owner.sourceRoot + "/" + row.sourceLeaf);
    assert.equal(mapping.destinationPath, owner.semanticOwnerRoot + "/" + row.destinationSuffix);
    assert.equal(hash(content), row.afterHash); assert.equal(Buffer.byteLength(content), row.afterSize);
    assert.equal(lstatSync(join(repoRoot, mapping.sourcePath)).mode & 0o7777, input.mode);
  }
});

await check("catalog uses only the six approved scalar replacements and paired pin", () => {
  let historical = catalogBytes, proposed = catalogBytes;
  for (const row of input.mappings) for (const [field, before, after] of [["sourceHash", row.beforeHash, row.afterHash], ["sourceSize", row.beforeSize, row.afterSize]]) {
    const path = ["packages", 0, "mappings", row.index, field];
    historical = applyEdits(historical, modify(historical, path, before, {}));
    proposed = applyEdits(proposed, modify(proposed, path, after, {}));
  }
  assert.equal(hash(historical), input.catalogBefore);
  assert.equal(hash(proposed), input.catalogAfter);
  assert.deepEqual(parse(proposed), proposal);
  assert.equal(hash(catalogBytes), input.catalogAfter);
  assert.equal(catalogContract.authorityCatalogSha256, input.catalogAfter);
});

await check("published source authority accepts the complete current package", () => {
  const result = discovery.semanticPackageProjectionAuthority(facts, catalog, taxonomy);
  assert.deepEqual(result.problems, []); assert.equal(result.mappings.length, input.sourceLeafCount);
});

for (const vector of input.sourceNegatives) await check("source rejects " + vector, () => {
  assert.deepEqual(discovery.semanticPackageProjectionAuthority(facts, proposal, taxonomy).problems, []);
  const changed = structuredClone(facts), selected = changed.nodes.find((row: any) => row.path === owner.sourceRoot + "/" + input.mappings[0].sourceLeaf)!, authority = structuredClone(proposal);
  if (vector === "content-drift") selected.content += "\n";
  if (vector === "missing-leaf") changed.nodes = changed.nodes.filter((row: any) => row.path !== selected.path);
  if (vector === "unknown-leaf") changed.nodes.push({ path: owner.sourceRoot + "/unknown.rs", nodeKind: "file", content: "" });
  if (vector === "symlink-leaf") selected.nodeKind = "symlink";
  if (vector === "directory-leaf") selected.nodeKind = "directory";
  if (vector === "cargo-identity") changed.nodes.find((row: any) => row.path === owner.sourceRoot + "/Cargo.toml")!.content = "[package]\nname=\"foreign\"\n";
  if (vector === "historical-preimages") for (const row of input.mappings) Object.assign(authority.packages[0].mappings[row.index], { sourceHash: row.beforeHash, sourceSize: row.beforeSize });
  assert.ok(discovery.semanticPackageProjectionAuthority(changed, authority, taxonomy).problems.length > 0);
});

await check("published profile and active authored vectors bind the exact runtime addition", () => {
  assert.deepEqual(discovery.validateTaxonomy(taxonomy), []);
  assert.deepEqual(contract.packageGeneration.browserProfile, profile);
  assert.equal(profile.sourceModulePaths.length, input.moduleCount);
  assert.equal(contract.inputPatterns.length, input.inputPatternCount);
  assert.equal(contract.inputPatterns.filter((path: string) => path === input.addedModule.path).length, 1);
  assert.deepEqual(JSON.parse(readFileSync(join(ticketRoot, "🔣️wgpu-browser-profile.json"), "utf8")), profile);
  assert.deepEqual(JSON.parse(readFileSync(join(ticketRoot, "🔣️wgpu-package-generator-contract.json"), "utf8")), contract);
  const content = read(input.addedModule.path), parsed = ts.createSourceFile(input.addedModule.path, content, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  assert.equal(hash(content), input.addedModule.hash); assert.equal(Buffer.byteLength(content), input.addedModule.size);
  const imports = parsed.statements.filter(ts.isImportDeclaration);
  assert.equal(imports.length, input.addedModule.typeOnlyImportCount); assert.ok(imports.every((node: ts.ImportDeclaration) => node.importClause?.isTypeOnly));
});

for (const vector of input.profileNegatives) await check("profile rejects " + vector, () => {
  const changed = structuredClone(profile);
  if (vector === "compose-path") changed.sourceModulePaths[0] = "compose/🟦️.ts";
  if (vector === "temp-compose-path") changed.sourceModulePaths[0] = "temp/compose/🟦️.ts";
  if (vector === "escaping-path") changed.sourceModulePaths[0] = "../🟦️.ts";
  if (vector === "duplicate-module") changed.sourceModulePaths.push(changed.sourceModulePaths[0]);
  if (vector === "inline-test-define") changed.inlineTestDefine = "false";
  if (vector === "entry-authority") changed.entries[0].sourceRelativePath = changed.entries[1].sourceRelativePath;
  if (vector === "workspace-authority") changed.workspaceImports.foreign = changed.workspaceImports["@semio-tech/framework"];
  assert.throws(() => discovery.parseSemanticPackageBrowserProfile(changed, owner));
});

await check("literal temp-compose is not a third opaque root", () => {
  const changed = structuredClone(profile); changed.sourceModulePaths = [...changed.sourceModulePaths, "temp-compose/🟦️.ts"].sort(order);
  assert.doesNotThrow(() => discovery.parseSemanticPackageBrowserProfile(changed, owner));
});

await check("published Bun browser closure is exact and repeatable", async () => {
  const result = await producer.renderWgpuBrowserBundles(repoRoot, contract.packageGeneration.browserProfile, owner, "source", { taxonomy, view });
  const repeated = await producer.renderWgpuBrowserBundles(repoRoot, contract.packageGeneration.browserProfile, owner, "source", { taxonomy, view });
  assert.deepEqual(result, repeated);
  assert.equal(result.inputs.length, input.inputCount);
  assert.deepEqual(result.inputs, [...profile.sourceModulePaths, ...Object.values(profile.workspaceImports).map((binding: any) => binding.manifestPath)].sort(order));
  assert.equal(result.nodes.length, 2);
});

await check("independent esbuild uses the identical published module set", async () => {
  const actual = contract.packageGeneration.browserProfile, admitted = new Set(actual.sourceModulePaths), observed = new Set<string>();
  for (const entry of actual.entries) {
    const result = await esbuild({ absWorkingDir: repoRoot, entryPoints: [join(repoRoot, owner.sourceRoot, entry.sourceRelativePath)], bundle: true, platform: "browser", format: "esm", write: false, metafile: true, logLevel: "silent", define: { "import.meta.vitest": "undefined" }, plugins: [{ name: "exact-read-only-oracle", setup(builder) {
      builder.onResolve({ filter: /.*/ }, (request) => {
        const path = request.kind === "entry-point" ? relative(repoRoot, request.path).replaceAll("\\", "/") : request.path.startsWith(".") ? posix.normalize(posix.join(posix.dirname(request.importer), request.path)) : actual.workspaceImports[request.path]?.entryPath;
        assert.ok(path && admitted.has(path), "Unadmitted oracle import: " + request.path);
        return { path, namespace: "wgpu-current-source" };
      });
      builder.onLoad({ filter: /.*/, namespace: "wgpu-current-source" }, (request) => ({ contents: read(request.path), loader: request.path.endsWith(".tsx") ? "tsx" : request.path.endsWith(".js") ? "js" : "ts" }));
    } }] });
    assert.deepEqual(result.errors, []); assert.equal(result.outputFiles.length, 1);
    for (const path of Object.keys(result.metafile!.inputs)) observed.add(path.replace(/^wgpu-current-source:/u, ""));
  }
  assert.deepEqual([...observed].sort(order), profile.sourceModulePaths);
});

for (const vector of input.browserNegatives) await check("browser rejects " + vector, async () => {
  const changed = structuredClone(profile), entry = owner.sourceRoot + "/" + profile.entries[0].sourceRelativePath;
  let reached = vector === "cancel-before", cancelled = reached;
  if (vector === "unread-module") changed.sourceModulePaths = [...changed.sourceModulePaths, "🧪️unread/🟦️.ts"].sort(order);
  const altered = { ...view, kind: (path: string) => {
    if (path === entry && ["missing-module", "symlink-module"].includes(vector)) { reached = true; return vector === "missing-module" ? null : "symlink"; }
    return view.kind(path);
  }, readText: (path: string) => {
    if (path === entry && vector === "undeclared-import") { reached = true; return read(path) + '\nexport * from "unowned-module";\n'; }
    return read(path);
  } };
  await assert.rejects(producer.renderWgpuBrowserBundles(repoRoot, changed, owner, "source", { taxonomy, view: altered, isCancelled: () => cancelled, progress: (event: any) => {
    if (vector === "cancel-module" && event.phase === "module-input" || vector === "cancel-bundle" && event.phase === "bundle") { reached = true; cancelled = true; }
  } }));
  assert.ok(reached || vector === "unread-module");
});

await check("read-only boundaries and completeness guard remain intact", () => {
  assert.deepEqual(outputStates(), outputsBefore);
  assert.equal(hash(readFileSync(catalogPath)), hash(catalogBytes));
  for (const [path, content] of bytes) assert.equal(hash(base.readText(path)), hash(content), path);
  assert.ok(readFileSync(join(library, "🧹️normalization/🟦️.ts"), "utf8").includes('violation("nested-cargo-generation-unresolved"'));
});

const failures = results.filter((result) => !result.pass);
console.log(JSON.stringify({ schemaVersion: 1, passed: results.length - failures.length, failed: failures.length, elapsedMs: performance.now() - started, sourceLeafCount: nodes.length, catalogHash: hash(catalogBytes), failures }, null, 2));
process.exitCode = failures.length ? 1 : 0;
