# Current Repo Physical and Finite RED Source Epochs

Independent source capture while Root owns corrections. These complete test/input snapshots preserve assertions and identify exact current inputs; the native logs retain their distinct earlier execution epoch.

## 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧲️rust-physical-reference-context/🟦️.ts

Bytes 39685; SHA-256 `83f095c67effa068b6951449228259709959f0af26081b1d1888e3f8a6fcf09f`.

````text
//#region Imports
import { expect, test } from "bun:test";
import { existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, realpathSync, readlinkSync, symlinkSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { dirname, join, relative, resolve } from "node:path";
import Ajv from "ajv";
import { parse as parseJsonc } from "jsonc-parser";
import { join as oraclePathJoin } from "pathe";
import ts from "typescript";
import * as rustDiscovery from "../../🔍️discovery/🟦️.ts";
import { inspectRustAssertionMessageSpans, inspectRustJoinArgumentSpans, inspectRustManifestPathReferences, inspectRustModuleGraph } from "../../🔍️discovery/🟦️.ts";
import { applyTaxonomyPlan, inventoryTaxonomy, planTaxonomy } from "../../🧹️normalization/🟦️.ts";
import { canonicalJson } from "../../🧾️serialization/🔣️json/🟦️.ts";
//#endregion Imports

//#region Authority
const root = resolve(import.meta.dir, "../../../../../../..");
const rustSyntaxPath = join(root, "🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts");
const rustSyntax = ts.createSourceFile(rustSyntaxPath, readFileSync(rustSyntaxPath, "utf8"), ts.ScriptTarget.Latest, true);
const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
if (!output) throw Error("SEMIO_TEST_ARTIFACT_DIR is required");
mkdirSync(output, { recursive: true });
const runRoot = resolve(output);
const vectorPath = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧲️rust-physical-reference-context/🔣️.json");
const golden = JSON.parse(readFileSync(vectorPath, "utf8"));
const schemaPath = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json";
const schemaBytes = readFileSync(join(root, schemaPath));
const retainedRunParent = join(runRoot, "semio-rust-physical-reference", ...golden.joinArguments.retention.parentSegments);
const retainedRuns = new Map<string, { dev: number; ino: number; reportHash: string }>();

/** 🛡️ Validates every ancestor of the exact retained run parent without following links. */
function verifyRetainedParent(create: boolean): void {
  let current = runRoot;
  const anchor = lstatSync(current);
  if (!anchor.isDirectory() || anchor.isSymbolicLink()) throw new Error("Rust run root is not a no-follow directory");
  for (const segment of relative(current, retainedRunParent).split(/[\\/]/u)) {
    current = join(current, segment);
    let stat;
    try { stat = lstatSync(current); }
    catch (error) {
      const withinRun = relative(runRoot, current);
      if (!create || (error as NodeJS.ErrnoException).code !== "ENOENT" || withinRun === "" || withinRun.startsWith("..") || resolve(runRoot, withinRun) !== current) throw error;
      mkdirSync(current);
      stat = lstatSync(current);
    }
    if (!stat.isDirectory() || stat.isSymbolicLink()) throw new Error(`Rust retained run ancestor is not a no-follow directory: ${current}`);
  }
}

/** 🧪️ Allocates one exact no-follow run owner without discarding authored or recovery evidence. */
function retainedRun(name: string): string {
  verifyRetainedParent(true);
  const directory = mkdtempSync(join(retainedRunParent, `🧪️${name}-`)), stat = lstatSync(directory);
  const report = `# Rust Physical Reference Run\n\nCase: ${name}.\n\nDisposition: retain all authored inputs, generated outputs and active or failed recovery evidence until exact review.\n\nThe run is allocated; the enclosing gate outcome is recorded in the parent report.\n`;
  writeFileSync(join(directory, "📝️.md"), report, { flag: "wx" });
  retainedRuns.set(directory, { dev: stat.dev, ino: stat.ino, reportHash: createHash("sha256").update(report).digest("hex") });
  return directory;
}

/** 📓️ Records a terminal assertion outcome only in the unchanged uniquely owned run. */
function retainRun(directory: string, passed: boolean): void {
  verifyRetainedParent(false);
  const ownership = retainedRuns.get(directory), stat = lstatSync(directory), reportPath = join(directory, "📝️.md"), reportStat = lstatSync(reportPath);
  if (!ownership || dirname(directory) !== retainedRunParent || !stat.isDirectory() || stat.isSymbolicLink() || stat.dev !== ownership.dev || stat.ino !== ownership.ino || !reportStat.isFile() || reportStat.isSymbolicLink()) throw new Error("Rust retained run ownership changed");
  const report = readFileSync(reportPath, "utf8");
  if (createHash("sha256").update(report).digest("hex") !== ownership.reportHash) throw new Error("Rust retained run report changed");
  writeFileSync(reportPath, `${report}\nAssertion outcome: ${passed ? "passed" : "failed or interrupted"}. No files were deleted.\n`);
}

/** 🧫️ Isolates Cargo ownership, a misleading sibling, and the exact normalized target. */
function fixture(kind = "mounted") {
  const directory = retainedRun(`path-${kind}`), vector = golden.transaction;
  const put = (path: string, content: string | Buffer) => { mkdirSync(dirname(join(directory, path)), { recursive: true }); writeFileSync(join(directory, path), content); };
  const schema = JSON.parse(schemaBytes.toString());
  delete schema.generatorContracts["plugin-registry"].inputDiscovery;
  put(schemaPath, `${JSON.stringify(schema, null, 2)}\n`);
  put(`${vector.scope}/${vector.source}`, vector.bytes);
  put(vector.lookalike, "wrong sibling\n");
  const manifest = '[package]\nname = "fixture"\nversion = "0.0.0"\nedition = "2021"\n[workspace]\n[lib]\npath = "entry.rs"\n[[bin]]\nname = "reader"\npath = "main.rs"\n';
  if (kind !== "symlink") put(vector.manifest, kind === "missing" ? '[package]\nname = "fixture"\n' : manifest);
  const reader = `pub fn read() { let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../${vector.scope}"); for path in ["${vector.source}"] { let source = std::fs::read_to_string(root.join(path)).unwrap(); println!("READ:{}", source.trim()); } }\n`;
  put(vector.consumer, kind === "unproven-base" ? reader.replace('std::path::Path::new(env!("CARGO_MANIFEST_DIR"))', "std::env::current_dir().unwrap()") : kind === "mutable-base" ? reader.replace("let root =", "let mut root =") : reader);
  put("pkg/main.rs", "fn main() { fixture::read(); }\n");
  if (kind === "ambiguous") put("second/Cargo.toml", '[package]\nname = "second"\n[lib]\npath = "../pkg/entry.rs"\n');
  if (kind === "symlink") { put("actual/Cargo.toml", manifest); symlinkSync("../actual/Cargo.toml", join(directory, vector.manifest)); }
  const git = (args: string[]) => { const result = Bun.spawnSync(["git", ...args], { cwd: directory, stdout: "pipe", stderr: "pipe" }); if (result.exitCode !== 0) throw new Error(result.stderr.toString()); return result.stdout.toString().trim(); };
  git(["init", "--quiet", "--object-format=sha1"]);
  put(".git/info/exclude", `${schemaPath}\n🧪️build/\npkg/Cargo.lock\n`);
  git(["-c", "user.name=Fixture", "-c", "user.email=fixture@invalid.example", "-c", "commit.gpgsign=false", "commit", "--quiet", "--allow-empty", "-m", "fixture"]);
  const baselineCommit = git(["rev-parse", "HEAD"]), ticketDir = join(directory, "🧪️transaction");
  return { directory, baselineCommit, ticketDir, put, options: { repoRoot: directory, scope: vector.scope, ticketDir, workers: 1 }, plan() { return planTaxonomy(inventoryTaxonomy(this.options), { baselineCommit, excludedTreeDigests: [] }); } };
}
//#endregion Authority

//#region Cases
test("string collection joins require exact standard receiver provenance", () => {
  const oracle = new Ajv({ strict: true }).compile(JSON.parse(readFileSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧲️rust-physical-reference-context/🧬️join-provenance/🔣️.json"), "utf8")));
  expect(oracle(golden.joinArguments)).toBe(true);
  for (const changed of [{ ...golden.joinArguments, ownership: "guessed" }, { ...golden.joinArguments, extra: true }, { ...golden.joinArguments, cases: [] }]) expect(oracle(changed)).toBe(false);
  expect(new Set(golden.joinArguments.cases.map((row: { id: string }) => row.id)).size).toBe(golden.joinArguments.cases.length);
  for (const row of golden.joinArguments.cases) {
    const arguments_ = inspectRustJoinArgumentSpans(row.source);
    expect(arguments_.map(({ value }) => value), row.id).toEqual(row.expected);
    for (const argument of arguments_) expect(row.source.slice(argument.start, argument.end)).toBe(argument.value);
  }
});

test("literal predicates keep identifier tokens reachable under strict TypeScript narrowing", () => {
  const path = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts");
  const source = ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true);
  const types = rustSyntax.statements.filter((node) => (ts.isTypeAliasDeclaration(node) || ts.isInterfaceDeclaration(node)) && ["RustTokenKind", "RustToken"].includes(node.name.text)).map((node) => node.getText(rustSyntax).replace(/^export /u, "")).join("\n");
  for (const name of golden.tokenNarrowing.functions) {
    const owner = source.statements.find((node): node is ts.FunctionDeclaration => ts.isFunctionDeclaration(node) && node.name?.text === name)!;
    const declarations: ts.VariableDeclaration[] = [];
    const visit = (node: ts.Node): void => { if (ts.isVariableDeclaration(node) && node.name.getText(source) === "literal") declarations.push(node); ts.forEachChild(node, visit); };
    visit(owner);
    expect(declarations).toHaveLength(1);
    const code = `${types}\nfunction probe(token: RustToken | undefined) { const ${declarations[0]!.getText(source)}; if (literal(token)) return "literal"; if (token?.kind === "identifier") return token.text; return null; }`;
    const virtualPath = join(runRoot, `🟦️${name}.ts`), options: ts.CompilerOptions = { strict: true, noEmit: true, types: [], lib: ["lib.es5.d.ts", "lib.es2015.core.d.ts"], target: ts.ScriptTarget.ES2022, skipLibCheck: true };
    const host = ts.createCompilerHost(options), getSourceFile = host.getSourceFile.bind(host);
    host.getSourceFile = (path, languageVersion, onError, shouldCreateNewSourceFile) => path === virtualPath ? ts.createSourceFile(path, code, languageVersion, true) : getSourceFile(path, languageVersion, onError, shouldCreateNewSourceFile);
    const program = ts.createProgram([virtualPath], options, host);
    expect(ts.getPreEmitDiagnostics(program).map((diagnostic) => ({ code: diagnostic.code, message: ts.flattenDiagnosticMessageText(diagnostic.messageText, "\n") }))).toEqual([]);
    for (const compiled of [new Bun.Transpiler({ loader: "ts" }).transformSync(code), ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText]) {
      const probe = new Function(`${compiled}\nreturn probe;`)();
      for (const row of golden.tokenNarrowing.cases) expect(probe(row.token === null ? undefined : { ...row.token, start: 0, end: row.token.text.length })).toEqual(row.expected);
    }
  }
});

test("registers the physical Rust reference gate through Nx and both launch catalogs", () => {
  const expected = golden.execution;
  const project = JSON.parse(readFileSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json"), "utf8"));
  expect(project.targets[expected.target]?.options.command).toBe(expected.command);
  for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
    const launches = parseJsonc(readFileSync(join(root, path), "utf8")).configurations.filter((entry: { name: string }) => entry.name === expected.launchName);
    expect(launches).toHaveLength(1);
    expect(launches[0].command).toBe(expected.launchCommand);
    expect(launches[0].presentation).toEqual({ group: expected.launchGroup, order: expected.launchOrder });
  }
});

test("manifest-relative joins require immutable lexical bindings and exact loop ownership", () => {
  expect(golden.manifestPaths.roots).toEqual(['std::path::Path::new(env!("CARGO_MANIFEST_DIR"))', 'std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))']);
  expect(golden.manifestPaths.binding).toBe("immutable-lexical-local-only");
  expect(golden.manifestPaths.literal).toBe("unescaped-normal-string-only");
  expect(golden.manifestPaths.loop).toBe("literal-array-variable-used-only-as-one-proven-join-argument");
  expect(new Set(golden.manifestPaths.cases.map((row: { id: string }) => row.id)).size).toBe(golden.manifestPaths.cases.length);
  for (const row of golden.manifestPaths.cases) {
    const references = inspectRustManifestPathReferences(row.source);
    expect({ id: row.id, references: references.map(({ value, base }) => ({ value, base })) }).toEqual({ id: row.id, references: row.expected });
    for (const reference of references) expect(row.source.slice(reference.start, reference.end)).toBe(reference.value);
  }
});

test("implicit format captures cannot widen the single-use manifest path law", () => {
  const row = golden.manifestPaths.cases.find((row: { id: string }) => row.id === "path-loop-implicit-label");
  expect(row.expected).toEqual([]);
  expect(inspectRustManifestPathReferences(row.source)).toEqual([]);
});

test("finite manifest candidates prove complete correlated targets without editable loop authority", () => {
  const contract = golden.manifestCandidates;
  expect(contract.contract).toBe("rust-finite-manifest-path-candidates-v1");
  expect(contract.authority).toBe("candidate-only-never-editable");
  expect(contract.missingFact).toBe("unproven-never-complete-empty");
  expect(contract.maxExpandedIterations).toBe(256);
  expect(contract.maxTargetsPerSpan).toBe(256);
  expect(contract.tupleCorrelation).toBe("per-row-environment");
  expect(contract.coordinates).toBe("relative-component-chains-only");
  expect(contract.environment).toBe("standard-env-macro-with-no-shadow-or-foreign-glob");
  expect(contract.literal).toBe("unescaped-normal-string-only");
  expect(contract.grammar).toEqual(["exact-standard-manifest-root", "immutable-literal-string-or-tuple-array", "literal-number-boolean-and-full-slice-metadata", "lexical-destructured-for", "exact-array-iter-enumerate", "immutable-join-chain", "known-standard-read-only-macro-use"]);
  expect(new Set(contract.cases.map((row: { id: string }) => row.id)).size).toBe(contract.cases.length);
  const inspect = rustDiscovery.inspectRustManifestPathCandidates;
  expect(typeof inspect).toBe("function");
  for (const row of contract.cases) {
    const candidates = inspect(row.source).filter((candidate) => candidate.value === contract.selectedValue);
    expect(candidates.map(({ value, targets }) => ({ value, targets })), row.id).toEqual(row.expected);
    for (const candidate of candidates) expect<string>(row.source.slice(candidate.start, candidate.end), row.id).toBe(candidate.value);
    const files: Record<string, string> = { [contract.manifestPath]: '[package]\nname="candidate"\n[lib]\npath="lib.rs"\n', [contract.consumerPath]: row.source };
    const graph = inspectRustModuleGraph(Object.keys(files), (path) => files[path], { strictManifests: true });
    const manifests = [...new Set((graph.contexts.get(contract.consumerPath) ?? []).map((context) => context.manifestPath).filter(Boolean))];
    expect(manifests).toEqual([contract.manifestPath]);
    const targets = [...new Set(candidates.flatMap((candidate) => candidate.targets.map((parts) => oraclePathJoin("pkg", ...parts))))].sort();
    expect(targets, row.id).toEqual(row.physicalTargets);
    const relevance = candidates.length === 0 ? "unproven" : targets.some((target) => target === contract.affectedRoot || target.startsWith(contract.affectedRoot + "/")) ? "intersects" : "disjoint";
    expect(relevance, row.id).toBe(row.relevance);
    expect(inspectRustManifestPathReferences(row.source).filter((reference) => reference.value === contract.selectedValue), row.id).toEqual([]);
  }
  const row = contract.cases[0];
  const files: Record<string, string> = {
    [contract.manifestPath]: '[package]\nname="one"\n[lib]\npath="lib.rs"\n',
    "second/Cargo.toml": '[package]\nname="two"\n[lib]\npath="../pkg/lib.rs"\n',
    [contract.consumerPath]: row.source,
  };
  const graph = inspectRustModuleGraph(Object.keys(files), (path) => files[path], { strictManifests: true });
  expect(new Set((graph.contexts.get(contract.consumerPath) ?? []).map((context) => context.manifestPath)).size).toBe(2);
});

test("Cargo manifest ownership requires an actual unique module-mount proof", () => {
  for (const row of golden.manifestPaths.moduleGraphCases) {
    const graph = inspectRustModuleGraph(Object.keys(row.files), (path) => row.files[path], { strictManifests: true });
    const manifests = [...new Set((graph.contexts.get(row.target) ?? []).map((context) => context.manifestPath).filter(Boolean))].sort();
    expect({ id: row.id, manifests }).toEqual({ id: row.id, manifests: row.expectedManifests });
  }
});

test("scoped joins bind the real target instead of a sibling and survive rollback, retry, runtime and an empty replan", () => {
  const row = fixture(), vector = golden.transaction, plan = row.plan();
  let passed = false;
  try {
  expect(plan.unresolved).toEqual([]);
  expect(plan.moves.map((move) => [move.sourcePath, move.destinationPath])).toEqual([[`${vector.scope}/${vector.source}`, `${vector.scope}/${vector.destination}`]]);
  expect(plan.edits.map((edit) => [edit.path, edit.oldValue, edit.newValue])).toEqual([[vector.consumer, vector.source, vector.destination]]);
  expect(plan.edits[0]!.structuredLocation.startsWith("rust-path-join:")).toBe(true);
  const runtime = () => {
    const result = Bun.spawnSync(["cargo", "run", "--quiet", "--offline", "--manifest-path", join(row.directory, vector.manifest), "--target-dir", join(row.directory, "🧪️build"), "--bin", "reader"], { cwd: row.directory, env: { ...process.env }, stdout: "pipe", stderr: "pipe" });
    expect(result.exitCode, result.stderr.toString()).toBe(0);
    expect(result.stdout.toString().trim()).toBe(vector.runtimeOutput);
  };
  runtime();
  const planPath = join(row.ticketDir, "🧾️plan/🔣️.json");
  row.put("🧪️transaction/🧾️plan/🔣️.json", `${canonicalJson(plan)}\n`);
  const options = { ...row.options, expectedBaselineCommit: row.baselineCommit, expectedPlanDigest: plan.planDigest, planArtifactPath: planPath };
  const rollback = applyTaxonomyPlan(plan, { ...options, injectFailureAt: "after-edits" });
  expect(rollback.state).toBe("rolled-back");
  expect(readFileSync(join(row.directory, vector.scope, vector.source), "utf8")).toBe(vector.bytes);
  expect(existsSync(join(row.directory, vector.scope, vector.destination))).toBe(false);
  runtime();
  expect(applyTaxonomyPlan(plan, options).state).toBe("committed");
  runtime();
  expect(readFileSync(join(row.directory, vector.lookalike), "utf8")).toBe("wrong sibling\n");
  expect(lstatSync(join(row.directory, vector.scope, vector.destination)).mode & 0o777).toBe(0o644);
  const empty = row.plan();
  expect([empty.moves.length, empty.edits.length, empty.unresolved.length]).toEqual([0, 0, 0]);
  passed = true;
  } finally {
    retainRun(row.directory, passed);
  }
}, 120_000);

for (const kind of ["missing", "ambiguous"]) test(`unproven ${kind} Cargo ownership fails closed`, () => {
  const row = fixture(kind), plan = row.plan();
  expect(plan.edits.filter((edit) => edit.path === golden.transaction.consumer)).toEqual([]);
  expect(plan.unresolved.some((problem) => problem.code === "reference-syntax-unsupported" && problem.message.includes("proven Cargo owner"))).toBe(true);
});

for (const kind of ["unproven-base", "mutable-base"]) test(`an external ${kind} path join is not silently dropped`, () => {
  const row = fixture(kind), plan = row.plan();
  expect(plan.edits.filter((edit) => edit.path === golden.transaction.consumer)).toEqual([]);
  expect(plan.unresolved.some((problem) => problem.code === "reference-syntax-unsupported" && problem.message.includes("immutable"))).toBe(true);
});

test("a symlinked Cargo ownership input is rejected without following it", () => {
  const row = fixture("symlink");
  expect(() => row.plan()).toThrow("symlink ancestor");
});

test("an unrelated unsafe Cargo candidate cannot become ownership authority or block the actual owner", () => {
  const row = fixture(), vector = golden.transaction, manifest = golden.referenceUniverse.unsafeUnrelatedManifest;
  row.put("unrelated-declaration/⚙️.toml", '[package]\nname = "unrelated"\n[lib]\npath = "entry.rs"\n');
  row.put("unrelated/entry.rs", "pub fn unrelated() {}\n");
  symlinkSync("../unrelated-declaration/⚙️.toml", join(row.directory, manifest));
  const plan = row.plan();
  expect(plan.unresolved).toEqual([]);
  expect(plan.edits.map((edit) => [edit.path, edit.oldValue, edit.newValue])).toEqual([[vector.consumer, vector.source, vector.destination]]);
  expect(readlinkSync(join(row.directory, manifest))).toBe("../unrelated-declaration/⚙️.toml");
});

for (const scenario of golden.referenceUniverse.cases) test(`Cargo reference ownership respects ${scenario.id} coordinates`, () => {
  const row = fixture(), vector = golden.transaction, independent = golden.referenceUniverse.independentRoot;
  const manifest = `${independent}/pkg/Cargo.toml`, consumer = `${independent}/pkg/entry.rs`;
  row.put(`${independent}/actual/⚙️.toml`, '[package]\nname = "independent"\n[lib]\npath = "entry.rs"\n');
  row.put(consumer, `pub fn read() { let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("${scenario.base}"); root.join("${vector.source}"); }\n`);
  row.put(`${independent}/${vector.scope}/${vector.source}`, "pub const LOCAL: bool = true;\n");
  symlinkSync("../actual/⚙️.toml", join(row.directory, manifest));
  const git = Bun.spawnSync(["git", "init", "--quiet", "--object-format=sha1"], { cwd: join(row.directory, independent), stdout: "pipe", stderr: "pipe" });
  expect(git.exitCode, git.stderr.toString()).toBe(0);
  const target = oraclePathJoin(independent, "pkg", scenario.base, vector.source);
  expect(target).toBe(scenario.physicalTarget);
  expect(target === `${vector.scope}/${vector.source}`).toBe(scenario.affectsParent);
  if (scenario.affectsParent) expect(() => row.plan()).toThrow("symlink ancestor");
  else {
    const plan = row.plan();
    expect(plan.unresolved).toEqual([]);
    expect(plan.edits.map((edit) => [edit.path, edit.oldValue, edit.newValue])).toEqual([[vector.consumer, vector.source, vector.destination]]);
    const localBytes = readFileSync(join(row.directory, consumer));
    const options = { ...row.options, expectedBaselineCommit: row.baselineCommit, expectedPlanDigest: plan.planDigest };
    expect(applyTaxonomyPlan(plan, { ...options, injectFailureAt: "after-edits" }).state).toBe("rolled-back");
    expect(readFileSync(join(row.directory, consumer))).toEqual(localBytes);
    expect(applyTaxonomyPlan(plan, options).state).toBe("committed");
    expect(readFileSync(join(row.directory, consumer))).toEqual(localBytes);
    const empty = row.plan();
    expect([empty.moves.length, empty.edits.length, empty.unresolved.length]).toEqual([0, 0, 0]);
  }
  expect(readlinkSync(join(row.directory, manifest))).toBe("../actual/⚙️.toml");
});

test("a newly relevant unsafe Cargo consumer rejects the frozen plan without overwriting its source", () => {
  const row = fixture(), vector = golden.transaction, plan = row.plan();
  row.put("new/entry.rs", readFileSync(join(row.directory, vector.consumer)));
  row.put("declaration/⚙️.toml", readFileSync(join(row.directory, vector.manifest)));
  symlinkSync("../declaration/⚙️.toml", join(row.directory, "new/Cargo.toml"));
  const bytes = readFileSync(join(row.directory, "new/entry.rs"));
  expect(() => applyTaxonomyPlan(plan, { ...row.options, expectedBaselineCommit: row.baselineCommit, expectedPlanDigest: plan.planDigest })).toThrow("symlink ancestor");
  expect(readFileSync(join(row.directory, "new/entry.rs"))).toEqual(bytes);
  expect(readFileSync(join(row.directory, vector.scope, vector.source), "utf8")).toBe(vector.bytes);
  expect(existsSync(join(row.directory, vector.scope, vector.destination))).toBe(false);
  expect(readlinkSync(join(row.directory, "new/Cargo.toml"))).toBe("../declaration/⚙️.toml");
});

for (const kind of ["consumer-drift", "manifest-drift", "new-incoming"]) test(`frozen scoped join authority rejects ${kind} without overwriting it`, () => {
  const row = fixture(), plan = row.plan(), vector = golden.transaction;
  expect(row.plan().planDigest).toBe(plan.planDigest);
  const consumer = readFileSync(join(row.directory, vector.consumer), "utf8"), manifest = readFileSync(join(row.directory, vector.manifest), "utf8");
  if (kind === "consumer-drift") row.put(vector.consumer, `${consumer}// concurrent source edit\n`);
  if (kind === "manifest-drift") row.put(vector.manifest, `${manifest}# concurrent manifest edit\n`);
  if (kind === "new-incoming") { row.put("new/Cargo.toml", manifest.replace('name = "fixture"', 'name = "new"')); row.put("new/entry.rs", consumer); }
  const bytes = readFileSync(join(row.directory, kind === "manifest-drift" ? vector.manifest : kind === "new-incoming" ? "new/entry.rs" : vector.consumer));
  expect(() => applyTaxonomyPlan(plan, { ...row.options, expectedBaselineCommit: row.baselineCommit, expectedPlanDigest: plan.planDigest })).toThrow();
  expect(readFileSync(join(row.directory, kind === "manifest-drift" ? vector.manifest : kind === "new-incoming" ? "new/entry.rs" : vector.consumer))).toEqual(bytes);
  expect(readFileSync(join(row.directory, vector.scope, vector.source), "utf8")).toBe(vector.bytes);
  expect(existsSync(join(row.directory, vector.scope, vector.destination))).toBe(false);
});

test("cancellation during Cargo-context planning remains read-only", () => {
  const row = fixture(), cancelFile = "🔣️cancel.json";
  expect(() => planTaxonomy(inventoryTaxonomy(row.options), { baselineCommit: row.baselineCommit, excludedTreeDigests: [], cancelFile, progress(event) { if (event.phase === "incoming-parse" && event.path === golden.transaction.consumer) row.put(cancelFile, "{}"); } })).toThrow(/cancel/iu);
  expect(readFileSync(join(row.directory, golden.transaction.scope, golden.transaction.source), "utf8")).toBe(golden.transaction.bytes);
  expect(existsSync(join(row.directory, golden.transaction.scope, golden.transaction.destination))).toBe(false);
});

test("Rust diagnostic references require exact unescaped assertion-message arguments", () => {
  const oracle = new Ajv().compile({ type: "object", required: ["schemaVersion", "contract", "assertionMessages"], properties: { schemaVersion: { const: 1 }, contract: { const: "rust-physical-reference-context-v1" }, assertionMessages: { type: "object", required: ["macros", "literal", "context", "cases"] } } });
  expect(oracle(golden)).toBe(true);
  for (const row of golden.assertionMessages.cases) {
    const messages = inspectRustAssertionMessageSpans(row.source);
    expect(messages.map(({ macroName, value }) => ({ macroName, value }))).toEqual(row.expected);
    for (const message of messages) expect(row.source.slice(message.start, message.end)).toBe(message.value);
  }
});

test("independent syn parsing reproduces assertion-message, manifest-path and join-provenance facts", async () => {
  const directory = retainedRun("syn-oracle"), target = join(directory, "🧪️target");
  writeFileSync(join(directory, "Cargo.toml"), readFileSync(join(root, golden.oracle.manifestInput)));
  writeFileSync(join(directory, "🦀️.rs"), readFileSync(join(root, golden.oracle.sourceInput)));
  let passed = false;
  try {
    const result = Bun.spawn(["cargo", "run", "--offline", "--quiet", "--manifest-path", join(directory, "Cargo.toml"), "--target-dir", target, "--", vectorPath], { cwd: directory, env: { ...process.env }, stdout: "pipe", stderr: "pipe" });
    const [stdout, stderr, exitCode] = await Promise.all([new Response(result.stdout).text(), new Response(result.stderr).text(), result.exited]);
    expect({ exitCode, stderr }).toEqual({ exitCode: 0, stderr: "" });
    expect(JSON.parse(stdout)).toEqual({
      assertionMessages: golden.assertionMessages.cases.map((row: any) => ({ id: row.id, messages: inspectRustAssertionMessageSpans(row.source).map(({ macroName, value }) => ({ macroName, value })) })),
      manifestPaths: golden.manifestPaths.cases.map((row: any) => ({ id: row.id, references: inspectRustManifestPathReferences(row.source).map(({ value, base }) => ({ value, base })) })),
      manifestCandidates: [...golden.manifestCandidates.cases, ...golden.manifestCandidates.adversarial.cases].map((row: any) => ({ id: row.id, candidates: rustDiscovery.inspectRustManifestPathCandidates(row.source).filter((candidate) => candidate.value === golden.manifestCandidates.selectedValue) })),
      joinArguments: golden.joinArguments.cases.map((row: any) => ({ id: row.id, candidates: inspectRustJoinArgumentSpans(row.source).map(({ value }) => value), allArguments: row.allArguments })),
    });
    passed = true;
  } finally {
    retainRun(directory, passed);
  }
}, 120_000);

test("rustc independently confirms delimiter strings and actual custom or standard path joins", () => {
  const directory = retainedRun("compiler-oracle");
  let passed = false;
  try {
    for (const row of golden.joinArguments.cases.filter((row: { compiler?: string }) => row.compiler)) {
      const owner = join(directory, `🧪️${row.id}`), input = join(owner, "🦀️.rs"), executable = join(owner, process.platform === "win32" ? "🧪️.exe" : "🧪️.bin");
      mkdirSync(owner);
      writeFileSync(input, `${row.source}\n${row.compiler}\n`, { flag: "wx" });
      const compiled = Bun.spawnSync(["rustc", "--edition=2021", "--crate-name", "join_oracle", input, "-o", executable], { cwd: owner, env: { ...process.env }, stdout: "pipe", stderr: "pipe", timeout: 30_000 });
      expect(compiled.exitCode, `${row.id}: ${compiled.stderr.toString()}`).toBe(0);
      const runtime = Bun.spawnSync([executable], { cwd: owner, env: { ...process.env }, stdout: "pipe", stderr: "pipe", timeout: 5_000 });
      expect(runtime.exitCode, `${row.id}: ${runtime.stderr.toString()}`).toBe(0);
      expect(runtime.stdout.toString().trim(), row.id).toBe(row.compilerOutput);
    }
    passed = true;
    console.log("Rust join compiler oracle confirmed six string/path runtime cases.");
  } finally { retainRun(directory, passed); }
}, 120_000);

test("finite candidate helper compiles independently under strict TypeScript", () => {
  const input = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts"), source = ts.createSourceFile(input, readFileSync(input, "utf8"), ts.ScriptTarget.Latest, true);
  const names = new Set(["RustTokenKind", "RustToken", "RustManifestPathCandidate", "inspectRustManifestPathCandidates"]);
  const declarations = [...rustSyntax.statements, ...source.statements].filter((node) => (ts.isTypeAliasDeclaration(node) || ts.isInterfaceDeclaration(node) || ts.isFunctionDeclaration(node)) && node.name && names.has(node.name.text));
  expect(declarations).toHaveLength(4);
  const text = declarations.map((node) => node.getText(node.getSourceFile())).join("\n") + '\ndeclare function rustTokens(source: string): RustToken[]; declare function rustTokenPairs(tokens: readonly RustToken[]): Map<number, number>; declare function rustTokenSegments(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, start: number, end: number, delimiter: string): [number, number][]; declare function rustFindTopLevel(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, start: number, end: number, values: ReadonlySet<string>): number; declare function rustRepoRootAncestorWalkHelperNames(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>): ReadonlySet<string>;\n';
  const virtualPath = join(runRoot, "📓️energy-rust-reference-diagnostics/🧭️manifest-pathbuf/🟦️typescript.ts"), options: ts.CompilerOptions = { strict: true, noEmit: true, types: [], target: ts.ScriptTarget.ES2022, skipLibCheck: true }, host = ts.createCompilerHost(options), original = host.getSourceFile.bind(host);
  host.getSourceFile = (path, language, onError, create) => path === virtualPath ? ts.createSourceFile(path, text, language, true) : original(path, language, onError, create);
  expect(ts.getPreEmitDiagnostics(ts.createProgram([virtualPath], options, host)).map((diagnostic) => ({ code: diagnostic.code, message: ts.flattenDiagnosticMessageText(diagnostic.messageText, "\n") }))).toEqual([]);
});

test("rustc confirms correlated finite receiver targets and missing-target rejection", () => {
  const directory = retainedRun("finite-candidate-compiler");
  let passed = false;
  try {
    const cases = golden.manifestCandidates.cases.filter((row: { runtime?: boolean }) => row.runtime);
    expect(cases).toHaveLength(5);
    for (const row of cases) for (const missing of row.id === "tuple-row-correlation" ? [false, true] : [false]) {
      const owner = join(directory, row.id + (missing ? "-missing" : "-present")), manifestDirectory = join(owner, "pkg"), input = join(owner, "🦀️.rs"), executable = join(owner, process.platform === "win32" ? "🧪️.exe" : "🧪️.bin");
      mkdirSync(manifestDirectory, { recursive: true });
      for (const path of row.physicalTargets.slice(missing ? 1 : 0)) { mkdirSync(dirname(join(owner, path)), { recursive: true }); writeFileSync(join(owner, path), "exact finite target\n", { flag: "wx" }); }
      writeFileSync(input, row.source + '\nfn main() { inspect(); println!("finite-targets-confirmed"); }\n', { flag: "wx" });
      const compiled = Bun.spawnSync(["rustc", "--edition=2021", "--crate-name", "finite_candidate_oracle", input, "-o", executable], { cwd: owner, env: { ...process.env, CARGO_MANIFEST_DIR: manifestDirectory }, stdout: "pipe", stderr: "pipe", timeout: 30_000 });
      expect(compiled.exitCode, row.id + ": " + compiled.stderr.toString()).toBe(0);
      const runtime = Bun.spawnSync([executable], { cwd: owner, env: { ...process.env }, stdout: "pipe", stderr: "pipe", timeout: 5_000 });
      if (missing) { expect(runtime.exitCode).not.toBe(0); expect(runtime.stderr.toString()).toContain("leaf.rs"); }
      else { expect(runtime.exitCode, row.id + ": " + runtime.stderr.toString()).toBe(0); expect(runtime.stdout.toString().trim()).toBe("finite-targets-confirmed"); }
    }
    passed = true;
    console.log("Rust finite receiver compiler oracle confirmed five exact-target cases and one missing-target rejection.");
  } finally { retainRun(directory, passed); }
}, 120_000);

for (const row of golden.manifestCandidates.adversarial.cases) test(`finite candidate adversarial runtime: ${row.id}`, () => {
  const contract = golden.manifestCandidates.adversarial;
  expect(contract.contract).toBe("rust-finite-candidate-adversarial-v1");
  expect(contract.unknownControlFlow).toBe("captured-bindings-remain-unproven");
  expect(contract.namespace).toBe("standard-type-and-macro-identity-required");
  const directory = retainedRun("finite-adversarial-" + row.id), manifestDirectory = join(directory, "pkg"), input = join(directory, "🦀️.rs"), executable = join(directory, process.platform === "win32" ? "🧪️.exe" : "🧪️.bin");
  let passed = false;
  try {
    mkdirSync(manifestDirectory);
    for (const [path, bytes] of Object.entries(row.runtimeProof.files ?? {})) writeFileSync(join(directory, path), bytes as string, { flag: "wx" });
    writeFileSync(input, row.source + `\nfn main() { ${row.runtimeProof.call}; }\n`, { flag: "wx" });
    const compiled = Bun.spawnSync(["rustc", "--edition=2021", "--crate-name", "candidate_adversarial", input, "-o", executable], { cwd: directory, env: { ...process.env, CARGO_MANIFEST_DIR: manifestDirectory }, stdout: "pipe", stderr: "pipe", timeout: 30_000 });
    expect(compiled.exitCode, compiled.stderr.toString()).toBe(0);
    const runtime = Bun.spawnSync([executable], { cwd: directory, env: { ...process.env }, stdout: "pipe", stderr: "pipe", timeout: 5_000 });
    expect(runtime.exitCode, runtime.stderr.toString()).toBe(0);
    const observed = runtime.stdout.toString().trim().split(/\r?\n/u).map((path) => relative(directory, path).replaceAll("\\", "/"));
    expect(observed).toEqual(row.runtimeProof.targets);
    console.log(`Rust adversarial ${row.id}: ${JSON.stringify(observed)}`);
    const candidates = rustDiscovery.inspectRustManifestPathCandidates(row.source).filter((candidate) => candidate.value === golden.manifestCandidates.selectedValue);
    expect(candidates.map(({ value, targets }) => ({ value, targets }))).toEqual(row.expected);
    expect(inspectRustManifestPathReferences(row.source).filter((reference) => reference.value === golden.manifestCandidates.selectedValue)).toEqual([]);
    passed = true;
  } finally { retainRun(directory, passed); }
}, 120_000);

test("finite candidate generic std namespace and expanded env ambiguities are rejected by Rust", () => {
  const directory = retainedRun("finite-generic-namespace");
  let passed = false;
  try {
    for (const row of golden.manifestCandidates.adversarial.namespaceReviews) {
      const owner = join(directory, row.id), input = join(owner, "🦀️.rs");
      mkdirSync(owner);
      for (const [path, bytes] of Object.entries(row.files ?? {})) writeFileSync(join(owner, path), bytes as string, { flag: "wx" });
      writeFileSync(input, row.source, { flag: "wx" });
      const compiled = Bun.spawnSync(["rustc", "--edition=2021", "--crate-name", "namespace_review", input, "-o", join(owner, "🧪️.bin")], { cwd: owner, env: { ...process.env, CARGO_MANIFEST_DIR: owner }, stdout: "pipe", stderr: "pipe", timeout: 30_000 });
      expect(compiled.exitCode).not.toBe(0);
      expect(compiled.stderr.toString()).toContain(row.errorCode);
    }
    passed = true;
  } finally { retainRun(directory, passed); }
}, 120_000);

test("rustc independently confirms manifest-root PathBuf construction and format capture behavior", () => {
  const directory = retainedRun("manifest-pathbuf-compiler");
  let passed = false;
  try {
    const cases = golden.manifestPaths.cases.filter((row: { compiler?: string }) => row.compiler);
    expect(cases).toHaveLength(5);
    for (const row of cases) {
      const owner = join(directory, `🧪️${row.id}`), input = join(owner, "🦀️.rs"), executable = join(owner, process.platform === "win32" ? "🧪️.exe" : "🧪️.bin");
      mkdirSync(owner);
      writeFileSync(input, `${row.source}\n${row.compiler}\n`, { flag: "wx" });
      const compiled = Bun.spawnSync(["rustc", "--edition=2021", "--crate-name", "manifest_pathbuf_oracle", input, "-o", executable], { cwd: owner, env: { ...process.env, CARGO_MANIFEST_DIR: owner }, stdout: "pipe", stderr: "pipe", timeout: 30_000 });
      expect(compiled.exitCode, `${row.id}: ${compiled.stderr.toString()}`).toBe(0);
      const runtime = Bun.spawnSync([executable], { cwd: owner, env: { ...process.env }, stdout: "pipe", stderr: "pipe", timeout: 5_000 });
      expect(runtime.exitCode, `${row.id}: ${runtime.stderr.toString()}`).toBe(0);
      expect(runtime.stdout.toString().trim().replaceAll("\r\n", "\n"), row.id).toBe(row.compilerOutput);
    }
    passed = true;
    console.log("Rust manifest PathBuf compiler oracle confirmed five constructor/capture runtime cases.");
  } finally { retainRun(directory, passed); }
}, 120_000);
//#endregion Cases

````

## 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥤️rust-finite-target-consumption/🟦️.ts

Bytes 37210; SHA-256 `a1531c73e76540bbbc9584a152f4dee4e0f14b90b95e620bd82e4b1da90f22ea`.

````text
import { normalizationSourceDeclarations, normalizationSourceFiles } from "../../🧹️normalization/🧪️support/🏗️source-services/🟦️.ts";
import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, symlinkSync, writeFileSync } from "node:fs";
import { basename, dirname, isAbsolute, join, parse, posix, relative, resolve, sep } from "node:path";
import Ajv from "ajv";
import { parse as parseJsonc } from "jsonc-parser";
import { parse as parseToml } from "@iarna/toml";
import { join as oracleJoin, normalize as oracleNormalize } from "pathe";
import ts from "typescript";
import { canonicalJson } from "../../🧾️serialization/🔣️json/🟦️.ts";
import { inspectRustAssertionMessageSpans, inspectRustCargoManifest, inspectRustJoinArgumentSpans, inspectRustManifestPathCandidates, inspectRustManifestPathReferences, inspectRustModuleGraph, inspectRustModuleGraphFacts, rustModuleScopeProof, inspectRustNonRepoJoinBaseSpans } from "../../🔍️discovery/🟦️.ts";
import { rustTokens as rustSyntaxTokens, rustTokenPairs } from "../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";

const root = resolve(import.meta.dir, "../../../../../../..");
const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
if (!output) throw Error("SEMIO_TEST_ARTIFACT_DIR is required");
mkdirSync(output, { recursive: true });
const ticket = resolve(output);
const vector = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🥤️rust-finite-target-consumption/🔣️.json"), "utf8"));
const sourcePath = resolve(import.meta.dir, "../../🧹️normalization/🟦️.ts");
const sourceInputs = normalizationSourceFiles(sourcePath);
const source = normalizationSourceDeclarations(sourcePath), syntax = ts.createSourceFile(sourcePath, source, ts.ScriptTarget.Latest, true);
const discoveryPath = resolve(import.meta.dir, "../../🔍️discovery/🟦️.ts"), discovery = ts.createSourceFile(discoveryPath, readFileSync(discoveryPath, "utf8"), ts.ScriptTarget.Latest, true);
const rustSyntaxPath = join(root, "🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts"), rustSyntax = ts.createSourceFile(rustSyntaxPath, readFileSync(rustSyntaxPath, "utf8"), ts.ScriptTarget.Latest, true);
const marker = "rust-finite-manifest-targets";
type Token = { start: number; end: number; value: string; structuredLocation: string; adapter: string; physicalTargets?: string[]; physicalInterpretation?: string; rewriteKind?: string; unsupportedReason?: string };
type Row = { id: string; source: string; targets: string[]; expected: string; affected: string[]; condition: string };
const functions = new Set(["sha256", "generatorPathCompare", "sourceRelative", "normalizeRelative", "assertNoFollowAncestors", "assertLexicalInputOutsideOpaque", "lstatOrNull", "checkCancellation", "ancestorReferenceCoordinateRoot", "lineLocation", "regexTokens", "rustTokens", "rustCodeOnlyTextForMacroTrust", "referenceTokens", "referenceAdapter", "unsupportedReferenceTokens", "addUniqueIndex", "referencePathIndex", "rustContextFiles", "unprovenRustReferenceTargets", "rustReferenceNeedsOwnership", "rustReferenceGraph", "rustFiniteManifestTargets", "rustManifestReferenceTokens", "rustReferenceInterpretationCovers", "referenceTokensIncludingUnsupported", "splitTokenSuffix", "resolveReferencePath", "resolveReferenceTokenPath"]);
const constants = new Set(["LEXICAL_OPAQUE_ROOTS", "RUST_MODULE_STRUCTURE_TRANSPARENT_MACRO_INVOCATIONS", "RUST_MODULE_STRUCTURE_TRANSPARENT_MACRO_DEFINITIONS", "RUST_MODULE_STRUCTURE_TRANSPARENT_STD_EXPRESSION_MACROS", "RUST_MODULE_STRUCTURE_TRANSPARENT_ATTRIBUTE_NAMES", "RUST_MODULE_STRUCTURE_TRANSPARENT_ATTRIBUTE_PATHS", "RUST_RESERVED_KEYWORDS", "indexedLineContent", "indexedLineStarts", "rustReferenceGraphs", "rustUnprovenReferenceTargets", "rustReferenceContextFiles"]);
const extracted = syntax.statements.filter((node) => ts.isFunctionDeclaration(node) ? functions.has(node.name?.text ?? "") : ts.isClassDeclaration(node) ? node.name?.text === "TaxonomyCancellationError" : ts.isVariableStatement(node) && node.declarationList.declarations.some((declaration) => constants.has(declaration.name.getText(syntax)))).map((node) => node.getText(syntax).replace(/^export /u, "")).join("\n");
const compilers = [
  { name: "Bun", compile: (text: string) => new Bun.Transpiler({ loader: "ts" }).transformSync(text) },
  { name: "TypeScript", compile: (text: string) => ts.transpileModule(text, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText },
];
const runParent = join(ticket, ...vector.retention.parentSegments);

/** 🛡️ Allocates a fresh ticket-owned run after validating every ancestor without following links. */
function newRun(name: string): string {
  let current = parse(runParent).root;
  for (const segment of relative(current, runParent).split(sep)) {
    current = join(current, segment);
    let stat;
    try { stat = lstatSync(current); }
    catch (error) {
      const local = relative(ticket, current);
      if ((error as NodeJS.ErrnoException).code !== "ENOENT" || !local || local.startsWith("..") || isAbsolute(local)) throw error;
      mkdirSync(current);
      stat = lstatSync(current);
    }
    if (!stat.isDirectory() || stat.isSymbolicLink()) throw new Error("Unsafe finite-consumer run parent: " + current);
  }
  const directory = mkdtempSync(join(runParent, vector.retention.runPrefix));
  writeFileSync(join(directory, "📝️.md"), "# Finite Target Consumer Run\n\nCase: " + name + ".\n\nNew isolated inputs; no production inventory, generators, Git mutation, source moves, or cleanup.\n\nThe enclosing gate records the assertion result. These inputs are not reconstructed historical evidence.\n", { flag: "wx" });
  return directory;
}

/** 🧫️ Materializes only an explicit language-neutral source/Cargo/target graph. */
function fixture(row: Row) {
  const directory = newRun(row.id), known = new Set<string>(), prefix = row.condition === "nested-coordinate" ? "nested/" : "", pkg = prefix + "pkg";
  const consumer = pkg + "/reader.rs", manifest = pkg + "/Cargo.toml", entry = pkg + "/entry.rs";
  const put = (path: string, content: string): void => {
    mkdirSync(dirname(join(directory, path)), { recursive: true });
    writeFileSync(join(directory, path), content);
    for (let current = path; current && current !== "."; current = posix.dirname(current)) known.add(current);
  };
  const manifestBytes = '[package]\nname = "finite_consumer"\nversion = "0.0.0"\nedition = "2021"\n[workspace]\n[lib]\npath = "entry.rs"\n';
  if (row.condition !== "no-owner") put(manifest, manifestBytes);
  if (row.condition !== "missing-chain") put(entry, '#[path = "reader.rs"] mod reader;\n');
  else known.add(entry);
  put(consumer, row.source);
  for (const path of row.affected) put(path, "affected sibling\n");
  for (const path of row.targets) {
    if (row.condition === "symlink-leaf" || row.condition === "symlink-ancestor") continue;
    put(path, "physical target\n");
  }
  if (row.condition === "two-owners") put("second/Cargo.toml", '[package]\nname = "second"\nversion = "0.0.0"\n[workspace]\n[lib]\npath = "../pkg/entry.rs"\n');
  if (row.condition === "symlink-leaf") {
    put("actual/item.json", "target behind link\n");
    mkdirSync(join(directory, "foreign"), { recursive: true });
    symlinkSync("../actual/item.json", join(directory, "foreign/item.json"));
    known.add("foreign"); known.add("foreign/item.json");
  }
  if (row.condition === "symlink-ancestor") {
    put("actual/item.json", "target behind ancestor\n");
    symlinkSync("actual", join(directory, "foreign"), process.platform === "win32" ? "junction" : "dir");
    known.add("foreign"); known.add("foreign/item.json");
  }
  if (row.condition === "unadmitted") known.delete("foreign/item.json");
  if (row.id === "missing-target") { known.add("absent"); known.add("absent/item.json"); }
  if (["cancelled-symlink", "cancelled-module-edge", "cancelled-manifest-edge"].includes(row.condition)) {
    put("elsewhere/deep/placeholder", "directory owner\n");
    put("elsewhere/foreign/item.json", "actual joined target\n");
    symlinkSync("elsewhere/deep", join(directory, "alias"), process.platform === "win32" ? "junction" : "dir");
    known.add("alias");
  }
  if (row.condition === "cancelled-module-edge") {
    put(entry, '#[path = "../alias/../pkg/reader.rs"] mod reader;\npub fn origin() -> &\'static str { reader::origin() }\n');
    put("elsewhere/pkg/reader.rs", 'pub fn origin() -> &\'static str { "actual physical module" }\n');
  }
  if (row.condition === "cancelled-manifest-edge") {
    put(manifest, manifestBytes.replace('path = "entry.rs"', 'path = "../alias/../pkg/entry.rs"'));
    put("elsewhere/pkg/entry.rs", "pub const ACTUAL_CRATE: bool = true;\n");
  }
  if (row.condition === "cancelled-file") put("not-directory", "not a directory\n");
  if (row.condition === "parent-env-macro") {
    put("shadow/deep/placeholder", "macro root\n");
    put("shadow/foreign/item.json", "actual macro target\n");
    put(entry, 'macro_rules! env { ("CARGO_MANIFEST_DIR") => { ' + JSON.stringify(join(directory, "shadow/deep")) + ' }; }\n#[path = "reader.rs"] mod reader;\npub fn run() { reader::read(); }\n');
  }
  if (row.condition === "parent-doc-comment") put(entry, '//! 🧭️ Crate root docs mention `!important` and a `#heading` in prose — neither is code.\n//!\n#[path = "reader.rs"] mod reader;\n');
  if (row.condition === "parent-known-macro") put(entry, '#[path = "reader.rs"] mod reader;\nsemio_framework_plugin::plugin_exports!(plugin::plugin, plugin::TestApps);\n');
  if (row.condition === "parent-cfg-test-mod") {
    put(entry, '#[path = "reader.rs"] mod reader;\n#[cfg(test)]\n#[path = "tests_x.rs"]\nmod tests_x;\n');
    put("pkg/tests_x.rs", 'mod tests { #[test] fn it_renames() {} }\n');
    known.add("pkg/tests_x.rs");
  }
  if (row.condition === "parent-glob-reexport") put(entry, '#[path = "reader.rs"] mod reader;\npub use reader::*;\n');
  if (row.condition === "parent-known-attribute-path") put(entry, '#[path = "reader.rs"] mod reader;\n#[semio_framework_async_macros::async_test]\nasync fn placeholder_test() {}\n');
  if (row.condition === "parent-crate-local-macro-and-std-expression-macros") put(entry, '#[path = "reader.rs"] mod reader;\nmacro_rules! impl_serde_op_codec { ($t:ty) => { impl $t {} }; }\nimpl_serde_op_codec!(Placeholder);\npub fn run() -> String { if true { format!("ok") } else { unreachable!() } }\n');
  if (row.condition === "opaque") known.add(row.source.includes("../temp/compose") ? "temp/compose/item.json" : "compose/item.json");
  const coordinateRoots = row.condition === "nested-coordinate" ? ["nested"] : row.condition === "foreign-coordinate" ? ["foreign"] : [];
  return { directory, known, consumer, manifest, entry, coordinateRoots, put };
}

/** 🔬️ Runs the actual private token pipeline with independent compilers and observable physical reads. */
function implementation(compiler: typeof compilers[number], directory: string) {
  const accesses: string[] = [];
  const observe = (path: string): void => {
    const local = relative(directory, String(path)).split(sep).join("/");
    accesses.push(local);
    if (["compose", "temp/compose"].some((opaque) => local === opaque || local.startsWith(opaque + "/"))) throw new Error("Opaque filesystem access: " + local);
    if (local === ".." || local.startsWith("..") || isAbsolute(local)) throw new Error("Foreign fixture filesystem access: " + local);
  };
  const dependencies = { createHash, canonicalJson, posix, basename, dirname, join, resolve, relative, isAbsolute, sep,
    lstatSync: (path: string) => { observe(path); return lstatSync(path); },
    readFileSync: (...args: Parameters<typeof readFileSync>) => { observe(String(args[0])); return (readFileSync as any)(...args); },
    existsSync: (path: string) => { observe(path); return existsSync(path); },
    inspectRustAssertionMessageSpans, inspectRustCargoManifest, inspectRustJoinArgumentSpans, inspectRustManifestPathCandidates, inspectRustManifestPathReferences, inspectRustModuleGraph, inspectRustModuleGraphFacts, rustModuleScopeProof, inspectRustNonRepoJoinBaseSpans, rustSyntaxTokens, rustTokenPairs };
  const actual = new Function(...Object.keys(dependencies), compiler.compile(extracted) + "\nreturn { index: referencePathIndex, graph: rustReferenceGraph, tokens: rustManifestReferenceTokens, all: referenceTokensIncludingUnsupported, unsupported: unsupportedReferenceTokens, resolve: resolveReferenceTokenPath, finite: typeof rustFiniteManifestTargets === 'undefined' ? undefined : rustFiniteManifestTargets, covers: typeof rustReferenceInterpretationCovers === 'undefined' ? undefined : rustReferenceInterpretationCovers };")(...Object.values(dependencies));
  return { ...actual, accesses };
}

/** 🧭️ Resolves explicit Cargo roots through an independent TOML parser and cross-platform path implementation. */
function cargoOracle(f: ReturnType<typeof fixture>) {
  return [...f.known].filter((path) => path.endsWith("/Cargo.toml")).map((path) => {
    const document = parseToml(readFileSync(join(f.directory, path), "utf8")) as any;
    const entry = oracleNormalize(oracleJoin(dirname(path), document.lib?.path ?? "src/lib.rs"));
    const entrySource = existsSync(join(f.directory, entry)) ? readFileSync(join(f.directory, entry), "utf8") : "";
    const child = entrySource.match(/#\[path = "([^"]+)"\] mod reader;/u)?.[1];
    return { manifest: path, entry, consumer: child ? oracleNormalize(oracleJoin(dirname(entry), child)) : null };
  }).filter((row) => row.consumer === f.consumer);
}

/** 🧾️ Locates only the exact authored leaf literal, independently of production token offsets. */
function leafSpan(content: string, value = "item.json") {
  const start = content.indexOf('"' + value + '"') + 1;
  if (start <= 0 || content.indexOf('"' + value + '"', start + value.length + 1) >= 0) throw new Error("Expected one authored leaf");
  return { start, end: start + value.length, value };
}

test("new finite interpretation declarations satisfy strict TypeScript without ambient any callbacks", () => {
  const declarations = new Set(["ReferenceToken", "ReferencePathIndex", "RustReferenceGraphView", "rustReferenceNeedsOwnership", "rustFiniteManifestTargets", "rustManifestReferenceTokens", "rustReferenceInterpretationCovers"]);
  const body = syntax.statements.filter((node) => (ts.isFunctionDeclaration(node) || ts.isInterfaceDeclaration(node)) && declarations.has(node.name?.text ?? "")).map((node) => node.getText(syntax)).join("\n");
  expect(body.includes("function rustFiniteManifestTargets")).toBe(true);
  const graphNames = new Set(["RustStructuralVisibility", "RustModuleMount", "RustModuleContext", "RustModuleGraph", "RustModuleParticipationReason", "RustModuleParticipation"]);
  const graphContracts = [...rustSyntax.statements, ...discovery.statements].filter((node) => (ts.isTypeAliasDeclaration(node) || ts.isInterfaceDeclaration(node)) && graphNames.has(node.name.text));
  expect(graphContracts.length).toBe(graphNames.size);
  const contracts = [
    ...graphContracts.map((node) => node.getText(node.getSourceFile())),
    'type TaxonomyReferenceAdapter = string;',
    'interface Reference { readonly start: number; readonly end: number; readonly value: string; readonly base: readonly string[] }',
    'interface Candidate { readonly start: number; readonly end: number; readonly value: string; readonly targets: readonly (readonly string[])[] }',
    'type RustModuleMetadataProblem = Readonly<{ code: "unsupported-attribute"; attributes: readonly string[] } | { code: "ambiguous-path"; paths: readonly (string | null)[] }>;',
    'interface ModuleFact { readonly name: string; readonly modulePath: readonly string[]; readonly inline: boolean; readonly pathTarget: string | null; readonly unresolved?: RustModuleMetadataProblem }',
    'interface RustModuleScopeFact { readonly kind: "root" | "inline"; readonly modulePath: readonly string[]; readonly bodyStartOffset: number; readonly bodyEndOffset: number; readonly unresolved?: RustModuleMetadataProblem }',
    'interface RustModuleGraphFacts { readonly modules: readonly ModuleFact[]; readonly uses: readonly { readonly specifier: string }[]; readonly includes: readonly { readonly modulePath: readonly string[]; readonly path: string; readonly conditional?: true }[]; readonly scopes: readonly RustModuleScopeFact[] }',
    'type RustModuleScopeProof = Readonly<{ state: "resolved"; scopes: readonly RustModuleScopeFact[] } | { state: "unresolved"; modulePath: readonly string[]; problem: RustModuleMetadataProblem | Readonly<{ code: "scope-not-unique"; count: number }> }>;',
    'interface Stat { readonly mode: number; readonly size: number; readonly mtimeMs: number; isFile(): boolean; isDirectory(): boolean; }',
    'interface Bytes { readonly byteLength: number; toString(encoding: "utf8"): string }',
    'declare const posix: { dirname(path: string): string; join(...parts: string[]): string; isAbsolute(path: string): boolean };',
    'declare function inspectRustManifestPathReferences(source: string): readonly Reference[];',
    'declare function inspectRustManifestPathCandidates(source: string): readonly Candidate[];',
    'declare function inspectRustJoinArgumentSpans(source: string): readonly Pick<Reference, "start" | "end" | "value">[];',
    'declare function inspectRustNonRepoJoinBaseSpans(source: string): ReadonlySet<number>;',
    'declare function inspectRustCargoManifest(source: string, strict: boolean): { readonly valid: boolean; readonly libPath: string | null; readonly dependencies: readonly string[] };',
    'declare function inspectRustModuleGraphFacts(source: string): RustModuleGraphFacts;',
    'declare function rustModuleScopeProof(facts: RustModuleGraphFacts, sourceScope: readonly string[]): RustModuleScopeProof;',
    'declare function rustCodeOnlyTextForMacroTrust(source: string): string;',
    'declare function sha256(source: string | Bytes): string;',
    'declare function canonicalJson(value: unknown): string;',
    'declare function checkCancellation(root: string, path?: string): void;',
    'declare function normalizeRelative(path: string): string;',
    'declare function ancestorReferenceCoordinateRoot(path: string, roots: ReadonlySet<string>): string | undefined;',
    'declare function assertLexicalInputOutsideOpaque(root: string, path: string, label: string, leaf: boolean): string;',
    'declare function lstatOrNull(path: string): Stat | null;',
    'declare function lstatSync(path: string): Stat;',
    'declare function readFileSync(path: string): Bytes;',
    'declare function generatorPathCompare(left: string, right: string): number;',
    'declare function dirname(path: string): string;',
    'declare function basename(path: string): string;',
    'declare function lineLocation(source: string, start: number, label: string): string;',
    'declare function rustContextFiles(path: string, index: ReferencePathIndex): readonly string[];',
    'declare function unprovenRustReferenceTargets(path: string, value: string, index: ReferencePathIndex): readonly string[];',
    'declare function rustReferenceGraph(path: string, index: ReferencePathIndex): RustReferenceGraphView | null;',
    'declare class TaxonomyCancellationError extends Error {};',
  ].join("\n");
  const file = join(import.meta.dir, "🧾️strict/🟦️.ts"), input = contracts + "\n" + body, options = { strict: true, noEmit: true, skipLibCheck: true, target: ts.ScriptTarget.ES2022, lib: ["lib.es2022.d.ts"], types: [] as string[] };
  const host = ts.createCompilerHost(options), read = host.readFile.bind(host), exists = host.fileExists.bind(host), get = host.getSourceFile.bind(host);
  host.readFile = (path) => path === file ? input : read(path);
  host.fileExists = (path) => path === file || exists(path);
  host.getSourceFile = (path, language, onError, fresh) => path === file ? ts.createSourceFile(path, input, language, true) : get(path, language, onError, fresh);
  const diagnostics = ts.getPreEmitDiagnostics(ts.createProgram([file], options, host));
  expect(diagnostics.map((item) => ts.flattenDiagnosticMessageText(item.messageText, "\n"))).toEqual([]);
});


test("exact finite consumer route and launch registration preserve the canonical semantic leaf", () => {
  const registration = vector.registration, project = JSON.parse(readFileSync(join(root, registration.projectPath), "utf8"));
  expect(project.targets[registration.target]).toEqual({ executor: "nx:run-commands", options: { cwd: dirname(registration.projectPath), command: registration.command } });
  const routerText = readFileSync(join(root, registration.routerPath), "utf8"), router = ts.createSourceFile(registration.routerPath, routerText, ts.ScriptTarget.Latest, true);
  const branches: ts.IfStatement[] = [];
  const visit = (node: ts.Node) => { if (ts.isIfStatement(node) && node.expression.getText(router) === 'segments[0] === "' + registration.route + '"') branches.push(node); ts.forEachChild(node, visit); };
  visit(router);
  expect(branches).toHaveLength(1);
  expect(branches[0]!.thenStatement.getText(router)).toContain(JSON.stringify(registration.testPath));
  expect(branches[0]!.thenStatement.getText(router)).toContain('runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot })');
  const launch = parseJsonc(readFileSync(join(root, ".vscode/launch.json"), "utf8")).configurations.filter((item: any) => item.name === registration.launchName);
  expect(launch).toHaveLength(1);
  expect(launch[0]).toEqual({ name: registration.launchName, type: "node-terminal", request: "launch", command: registration.launchCommand, cwd: "$" + "{workspaceFolder}", env: { SEMIO_TEST_ARTIFACT_DIR: "$" + "{workspaceFolder}/$" + "{input:processContractArtifacts}/rust-finite-target-consumption" }, presentation: { group: "4_gate", order: registration.launchOrder } });
});


test("language-neutral finite consumer contract is closed and retains all physical proof obligations", () => {
  const validate = new Ajv({ strict: true }).compile(JSON.parse(readFileSync(join(import.meta.dir, "../../🧬️schema/🥤️rust-finite-target-consumption/🔣️.json"), "utf8")));
  expect(validate(vector), JSON.stringify(validate.errors)).toBe(true);
  for (const changed of [{ ...vector, unknown: true }, { ...vector, semantics: { ...vector.semantics, failure: "empty-is-disjoint" } }, { ...vector, cases: [] }]) expect(validate(changed)).toBe(false);
  expect(new Set(vector.cases.map((row: Row) => row.id)).size).toBe(vector.cases.length);
});

for (const compiler of compilers) for (const row of vector.cases as Row[]) test(compiler.name + " physical finite targets: " + row.id, () => {
  const f = fixture(row), actual = implementation(compiler, f.directory);
  const index = actual.index(f.known, f.directory, f.coordinateRoots, f.known, undefined, new Set(row.affected));
  if (row.condition.startsWith("changed-")) {
    actual.graph(f.consumer, index);
    const changed = row.condition === "changed-consumer" ? f.consumer : row.condition === "changed-chain" ? f.entry : f.manifest;
    f.put(changed, readFileSync(join(f.directory, changed), "utf8") + "\n" + (changed.endsWith(".rs") ? "pub const SNAPSHOT_CHANGE: u8 = 1;\n" : 'description = "changed snapshot"\n'));
  }
  const span = leafSpan(row.source), tokens = actual.tokens(f.consumer, row.source, index) as Token[], same = tokens.filter((token) => token.start === span.start && token.end === span.end);
  if (row.expected === "finite") {
    expect(cargoOracle(f)).toHaveLength(1);
    const candidate = inspectRustManifestPathCandidates(row.source).find((item) => item.start === span.start && item.end === span.end)!;
    const expected = [...new Set(candidate.targets.map((parts) => oracleNormalize(oracleJoin(dirname(f.manifest), ...parts))))].sort();
    expect(expected).toEqual([...row.targets].sort());
    expect(same).toHaveLength(1);
    expect(same[0]!.physicalInterpretation).toBe(marker);
    expect(same[0]!.physicalTargets).toEqual(expected);
    expect(same[0]!.rewriteKind).toBeUndefined();
    expect(same[0]!.unsupportedReason).toBeTruthy();
    const all = actual.all(f.consumer, row.source, index) as Token[];
    expect(all.filter((token) => token.start === span.start && token.end === span.end)).toHaveLength(1);
    expect(same[0]!.physicalTargets!.filter((path) => row.affected.includes(path))).toEqual(expected.filter((path) => row.affected.includes(path)));
    if (row.id === "foreign-disjoint") expect(all.some((token) => actual.resolve(f.consumer, token, index) === "pkg/item.json")).toBe(false);
  } else {
    expect(same.some((token) => token.physicalInterpretation === marker)).toBe(false);
    const all = actual.all(f.consumer, row.source, index) as Token[];
    expect(all.some((token) => token.start === span.start && token.end === span.end && (token.physicalTargets?.some((path) => row.affected.includes(path)) || row.affected.includes(actual.resolve(f.consumer, token, index))))).toBe(true);
  }
  expect(actual.accesses.some((path: string) => ["compose", "temp/compose"].some((opaque) => path === opaque || path.startsWith(opaque + "/")))).toBe(false);
});

for (const compiler of compilers) test(compiler.name + " terminal exact-target index keeps the complete foreign physical interpretation", () => {
  const row = vector.cases[0] as Row, f = fixture(row), actual = implementation(compiler, f.directory), span = leafSpan(row.source);
  const index = actual.index(row.affected, f.directory, [], f.known);
  const tokens = actual.all(f.consumer, row.source, index) as Token[];
  const same = tokens.filter((token) => token.start === span.start && token.end === span.end);
  expect(same).toHaveLength(1);
  expect(same[0]!.physicalTargets).toEqual(["foreign/item.json"]);
  expect(same[0]!.physicalInterpretation).toBe(marker);
  expect(tokens.some((token) => row.affected.includes(actual.resolve(f.consumer, token, index)))).toBe(false);
});

for (const compiler of compilers) test(compiler.name + " tuple row correlation does not manufacture cross-paired physical targets", () => {
  const row = { ...vector.cases[0], id: "tuple-correlation", source: vector.correlated.source, targets: vector.correlated.targets.map((item: any) => item.target), affected: ["affected/beta.json", "pkg/alpha.json", "pkg/beta.json"] };
  const f = fixture(row), actual = implementation(compiler, f.directory), index = actual.index(f.known, f.directory, [], f.known, undefined, new Set(row.affected));
  const tokens = actual.all(f.consumer, row.source, index) as Token[];
  for (const item of vector.correlated.targets) {
    const span = leafSpan(row.source, item.value), same = tokens.filter((token) => token.start === span.start && token.end === span.end);
    expect(same).toHaveLength(1);
    expect(same[0]!.physicalInterpretation).toBe(marker);
    expect(same[0]!.physicalTargets).toEqual([item.target]);
    expect(same[0]!.rewriteKind).toBeUndefined();
  }
});

for (const compiler of compilers) test(compiler.name + " writable proof keeps precedence while finite suppression requires exact span identity", () => {
  const row = { ...vector.cases[0], id: "writable-precedence", source: vector.writable.source }, f = fixture(row), actual = implementation(compiler, f.directory), index = actual.index(f.known, f.directory);
  const span = leafSpan(row.source), rows = (actual.all(f.consumer, row.source, index) as Token[]).filter((token) => token.start === span.start && token.end === span.end);
  expect(rows).toHaveLength(1);
  expect(rows[0]!.rewriteKind).toBe("rust-path-join");
  expect(rows[0]!.physicalInterpretation).toBeUndefined();
  expect(rows[0]!.physicalTargets).toEqual([vector.writable.target]);
  expect(actual.covers).toBeFunction();
  const interpreted = { ...span, adapter: "rust", physicalTargets: ["foreign/item.json"], physicalInterpretation: marker, unsupportedReason: "candidate-only" };
  expect(actual.covers(interpreted, { ...span, adapter: "rust" })).toBe(true);
  for (const changed of [{ ...span, start: span.start + 1 }, { ...span, end: span.end - 1 }, { ...span, value: "other.json" }, { ...span, start: span.start + 100, end: span.end + 100 }]) expect(actual.covers(interpreted, { ...changed, adapter: "rust" })).toBe(false);
  expect(actual.covers({ ...interpreted, physicalInterpretation: undefined }, { ...span, adapter: "rust" })).toBe(false);
});

for (const compiler of compilers) test(compiler.name + " changed source bytes and cancellation cannot obtain finite authority", () => {
  const row = vector.cases[0] as Row, f = fixture(row), actual = implementation(compiler, f.directory), index = actual.index(f.known, f.directory);
  expect(() => actual.tokens(f.consumer, row.source + "\n", index)).toThrow("source changed");
  f.put("cancel", "cancel\n");
  expect(() => actual.tokens(f.consumer, row.source, actual.index(f.known, f.directory, [], f.known, "cancel"))).toThrow("cancelled");
});

for (const compiler of compilers) test(compiler.name + " neighboring equal-value literals remain conservative and incomplete candidate facts never mean disjoint", () => {
  const initial = vector.cases[0] as Row, row = { ...initial, id: "neighbor-span", source: initial.source + '\nconst UNRELATED: &str = "item.json";\n' };
  const f = fixture(row), actual = implementation(compiler, f.directory), index = actual.index(f.known, f.directory, [], f.known, undefined, new Set(row.affected)), graph = actual.graph(f.consumer, index);
  const first = leafSpan(initial.source), neighbor = row.source.lastIndexOf('"item.json"') + 1;
  const tokens = actual.all(f.consumer, row.source, index) as Token[];
  expect(tokens.filter((token) => token.start === first.start && token.physicalInterpretation === marker)).toHaveLength(1);
  expect(tokens.some((token) => token.start === neighbor && actual.resolve(f.consumer, token, index) === "pkg/item.json")).toBe(true);
  const candidate = inspectRustManifestPathCandidates(row.source).find((item) => item.start === first.start)!;
  for (const changed of [{ ...candidate, start: candidate.start + 1 }, { ...candidate, end: candidate.end - 1 }, { ...candidate, value: "other.json" }, { ...candidate, targets: [] }, { ...candidate, targets: Array.from({ length: 257 }, () => candidate.targets[0]) }]) expect(actual.finite(f.consumer, row.source, [changed], index, graph).size).toBe(0);
});

test("actual rustc proves cancelled symlink steps target different bytes from normalized lexical paths", () => {
  const row = vector.cases.find((item: Row) => item.id === "cancelled-symlink-ancestor") as Row, f = fixture(row);
  const nativeSource = 'fn main() { let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")); let actual = root.join("../alias/../foreign").join("item.json"); println!("{}", std::fs::read_to_string(actual).unwrap().trim()); }\n';
  f.put("🧾️native/🦀️.rs", nativeSource);
  const binary = join(f.directory, "🧾️native", process.platform === "win32" ? "🔣️.exe" : "../../🧫️fixtures/🥤️rust-finite-target-consumption/🔣️.json");
  const compile = Bun.spawnSync(["rustc", "--edition=2021", "--crate-name", "finite_path_identity", join(f.directory, "🧾️native/🦀️.rs"), "-o", binary], { cwd: f.directory, env: { ...process.env, CARGO_MANIFEST_DIR: join(f.directory, "pkg") }, stdout: "pipe", stderr: "pipe" });
  expect(compile.exitCode, compile.stderr.toString()).toBe(0);
  const execution = Bun.spawnSync([binary], { cwd: f.directory, stdout: "pipe", stderr: "pipe" });
  expect(execution.exitCode, execution.stderr.toString()).toBe(0);
  expect(execution.stdout.toString()).toBe("actual joined target\n");
  expect(readFileSync(join(f.directory, "foreign/item.json"), "utf8")).toBe("physical target\n");
  writeFileSync(join(f.directory, "🧾️native/📝️.md"), "# Native Physical Identity Oracle\n\nrustc compiled the exact new isolated input. Runtime stdout was actual joined target; the normalized lexical target contained physical target. The cancelled symlink segment therefore changes physical identity and must not receive finite authority.\n");
});

test("actual rustc resolves raw module ownership paths before lexical cancellation", () => {
  const row = vector.cases.find((item: Row) => item.id === "cancelled-module-ownership-edge") as Row, f = fixture(row);
  f.put("🧾️native/🦀️.rs", '#[path = "../pkg/entry.rs"] mod owner;\nfn main() { println!("{}", owner::origin()); }\n');
  const binary = join(f.directory, "🧾️native", process.platform === "win32" ? "🔣️.exe" : "../../🧫️fixtures/🥤️rust-finite-target-consumption/🔣️.json");
  const compile = Bun.spawnSync(["rustc", "--edition=2021", "--crate-name", "finite_source_identity", join(f.directory, "🧾️native/🦀️.rs"), "-o", binary], { cwd: f.directory, env: { ...process.env, CARGO_MANIFEST_DIR: join(f.directory, "pkg") }, stdout: "pipe", stderr: "pipe" });
  expect(compile.exitCode, compile.stderr.toString()).toBe(0);
  const execution = Bun.spawnSync([binary], { cwd: f.directory, stdout: "pipe", stderr: "pipe" });
  expect(execution.exitCode, execution.stderr.toString()).toBe(0);
  expect(execution.stdout.toString()).toBe("actual physical module\n");
  expect(readFileSync(join(f.directory, f.consumer), "utf8")).toBe(row.source);
});


test("actual rustc proves inherited env macro provenance is part of physical source authority", () => {
  const row = vector.cases.find((item: Row) => item.id === "inherited-env-macro") as Row, f = fixture(row);
  f.put("🧾️native/🦀️.rs", '#[path = "../pkg/entry.rs"] mod owner;\nfn main() { owner::run(); }\n');
  const binary = join(f.directory, "🧾️native", process.platform === "win32" ? "🔣️.exe" : "../../🧫️fixtures/🥤️rust-finite-target-consumption/🔣️.json");
  const compile = Bun.spawnSync(["rustc", "--edition=2021", "--crate-name", "finite_macro_identity", join(f.directory, "🧾️native/🦀️.rs"), "-o", binary], { cwd: f.directory, env: { ...process.env, CARGO_MANIFEST_DIR: join(f.directory, "pkg") }, stdout: "pipe", stderr: "pipe" });
  expect(compile.exitCode, compile.stderr.toString()).toBe(0);
  const execution = Bun.spawnSync([binary], { cwd: f.directory, stdout: "pipe", stderr: "pipe" });
  expect(execution.exitCode, execution.stderr.toString()).toBe(0);
  expect(execution.stdout.toString()).toBe("TARGET:actual macro target\nitem.json\n");
  expect(readFileSync(join(f.directory, "foreign/item.json"), "utf8")).toBe("physical target\n");
});


test("actual Cargo metadata independently agrees with explicit lib-root and module ownership inputs", () => {
  const f = fixture({ ...vector.cases[0], id: "cargo-metadata-oracle" });
  const result = Bun.spawnSync(["cargo", "metadata", "--offline", "--no-deps", "--format-version", "1", "--manifest-path", join(f.directory, f.manifest)], { cwd: f.directory, stdout: "pipe", stderr: "pipe" });
  expect(result.exitCode, result.stderr.toString()).toBe(0);
  const metadata = JSON.parse(result.stdout.toString()), library = metadata.packages[0].targets.find((target: any) => target.kind.includes("lib"));
  expect(metadata.packages).toHaveLength(1);
  expect(relative(f.directory, library.src_path).split(sep).join("/")).toBe(f.entry);
  expect(cargoOracle(f)).toEqual([{ manifest: f.manifest, entry: f.entry, consumer: f.consumer }]);
});


test("incoming, planning unsupported pass, and terminal verification share one exact-span suppression predicate", () => {
  const declaration = (name: string) => syntax.statements.find((node) => ts.isFunctionDeclaration(node) && node.name?.text === name) as ts.FunctionDeclaration | undefined;
  const calls = (name: string, target: string) => {
    const owner = declaration(name); if (!owner) throw new Error("Missing actual caller: " + name);
    const found: ts.CallExpression[] = [];
    const visit = (node: ts.Node) => { if (ts.isCallExpression(node) && ts.isIdentifier(node.expression) && node.expression.text === target) found.push(node); ts.forEachChild(node, visit); };
    visit(owner); return found;
  };
  expect(calls("referenceTokensIncludingUnsupported", "rustReferenceInterpretationCovers")).toHaveLength(1);
  expect(calls("buildReferenceEdits", "rustReferenceInterpretationCovers")).toHaveLength(1);
  expect(calls("incomingReferenceSnapshot", "referenceTokensIncludingUnsupported")).toHaveLength(1);
  expect(calls("lexicalTargetIncomingReferences", "referenceTokensIncludingUnsupported")).toHaveLength(1);
  const planner = declaration("buildReferenceEdits")!.getText(syntax);
  expect(planner).toContain('if (token.unsupportedReason && token.physicalTargets !== undefined');
  expect(planner).toContain('token.physicalTargets.some((target) => destinationBySource.has(target))');
  expect(planner).toContain('unresolved.push(violation("reference-syntax-unsupported"');
  expect(source).toBe(normalizationSourceDeclarations(sourcePath));
  expect(normalizationSourceFiles(sourcePath)).toEqual(sourceInputs);
});

````

## 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧲️rust-physical-reference-context/🔣️.json

Bytes 68396; SHA-256 `6e1a83fb2b0bd1d7f912337b61d1702e756b74f4201153117f02a93c592486d0`.

````text
{
  "schemaVersion": 1,
  "contract": "rust-physical-reference-context-v1",
  "joinArguments": {
    "contract": "rust-standard-string-collection-join-v1",
    "ownership": "source-local-standard-vec-with-string-elements-and-no-shadow-or-escape",
    "retention": {
      "parentSegments": [
        "📓️rust-join-provenance",
        "🧪️runs"
      ],
      "disposition": "retain-all-until-reviewed"
    },
    "cases": [
      {"id":"same-file-parent-glob","source":"mod tests { use super::*; fn check() { let mut failures = Vec::new(); failures.push(format!(\"a\")); failures.join(\"\\n\"); } }","expected":[],"allArguments":["\\n"]},
      {"id":"external-parent-glob","source":"use super::*; fn check() { let mut failures = Vec::new(); failures.push(format!(\"a\")); failures.join(\"payload.rs\"); }","expected":["payload.rs"],"allArguments":["payload.rs"]},
      {"id":"same-file-parent-custom-vec","source":"struct Vec; mod tests { use super::*; fn check() { let mut failures = Vec::new(); failures.push(format!(\"a\")); failures.join(\"payload.rs\"); } }","expected":["payload.rs"],"allArguments":["payload.rs"]},
      {
        "id": "mutable-format-branch",
        "source": "fn check() -> String { let mut failures = Vec::new(); for value in [\"a\", \"b\"] { if !value.is_empty() { failures.push(format!(\"{}\", value)); } } assert!(!failures.is_empty()); failures.join(\"\\n\") }",
        "expected": [],
        "allArguments": [
          "\\n"
        ],
        "compiler": "fn main() { let result: String = check(); assert_eq!(result, \"a\\nb\"); println!(\"delimiter\"); }",
        "compilerOutput": "delimiter"
      },
      {
        "id": "path-looking-delimiter",
        "source": "fn check() -> String { let mut values = Vec::new(); values.push(\"a\"); values.push(\"b\"); values.join(\"payload.rs\") }",
        "expected": [],
        "allArguments": [
          "payload.rs"
        ],
        "compiler": "fn main() { let result: String = check(); assert_eq!(result, \"apayload.rsb\"); println!(\"delimiter\"); }",
        "compilerOutput": "delimiter"
      },
      {
        "id": "qualified-standard",
        "source": "fn check() -> String { let mut values = ::std::vec::Vec::new(); values.push(::std::format!(\"a\")); values.join(\"\\n\") }",
        "expected": [],
        "allArguments": [
          "\\n"
        ],
        "compiler": "fn main() { let result: String = check(); assert_eq!(result, \"a\"); println!(\"delimiter\"); }",
        "compilerOutput": "delimiter"
      },
      {
        "id": "typed-empty",
        "source": "fn check() -> String { let values: Vec<String> = Vec::new(); values.join(\"\\n\") }",
        "expected": [],
        "allArguments": [
          "\\n"
        ],
        "compiler": "fn main() { let result: String = check(); assert_eq!(result, \"\"); println!(\"delimiter\"); }",
        "compilerOutput": "delimiter"
      },
      {
        "id": "nested-shadow",
        "source": "fn check() { let mut values = Vec::new(); values.push(\"a\"); { let values = other(); values.join(\"payload.rs\"); } values.join(\"\\n\"); }",
        "expected": [
          "payload.rs"
        ],
        "allArguments": [
          "payload.rs",
          "\\n"
        ]
      },
      {
        "id": "closure-shadow",
        "source": "fn check() { let mut values = Vec::new(); values.push(\"a\"); let callback = |values| values.join(\"payload.rs\"); values.join(\"\\n\"); }",
        "expected": [
          "payload.rs"
        ],
        "allArguments": [
          "payload.rs",
          "\\n"
        ]
      },
      {
        "id": "if-let-shadow",
        "source": "fn check() { let mut values = Vec::new(); values.push(\"a\"); if let Some(values) = other() { values.join(\"payload.rs\"); } values.join(\"\\n\"); }",
        "expected": [
          "payload.rs"
        ],
        "allArguments": [
          "payload.rs",
          "\\n"
        ]
      },
      {
        "id": "match-shadow",
        "source": "fn check() { let mut values = Vec::new(); values.push(\"a\"); match other() { Some(values) => { values.join(\"payload.rs\"); }, None => {} } values.join(\"\\n\"); }",
        "expected": [
          "payload.rs"
        ],
        "allArguments": [
          "payload.rs",
          "\\n"
        ]
      },
      {
        "id": "for-shadow",
        "source": "fn check() { let mut values = Vec::new(); values.push(\"a\"); for values in paths() { values.join(\"payload.rs\"); } values.join(\"\\n\"); }",
        "expected": [
          "payload.rs"
        ],
        "allArguments": [
          "payload.rs",
          "\\n"
        ]
      },
      {
        "id": "destructured-shadow",
        "source": "fn check() { let mut values = Vec::new(); values.push(\"a\"); let (values, _) = other(); values.join(\"payload.rs\"); }",
        "expected": [
          "payload.rs"
        ],
        "allArguments": [
          "payload.rs"
        ]
      },
      {
        "id": "reassigned-receiver",
        "source": "fn check() { let mut values = Vec::new(); values.push(\"a\"); values = other(); values.join(\"payload.rs\"); }",
        "expected": [
          "payload.rs"
        ],
        "allArguments": [
          "payload.rs"
        ]
      },
      {
        "id": "escaped-receiver",
        "source": "fn check() { let mut values = Vec::new(); values.push(\"a\"); replace(&mut values); values.join(\"payload.rs\"); }",
        "expected": [
          "payload.rs"
        ],
        "allArguments": [
          "payload.rs"
        ]
      },
      {
        "id": "unknown-mutation",
        "source": "fn check() { let mut values = Vec::new(); values.push(\"a\"); values.replace_from(other()); values.join(\"payload.rs\"); }",
        "expected": [
          "payload.rs"
        ],
        "allArguments": [
          "payload.rs"
        ]
      },
      {
        "id": "unknown-pushed-type",
        "source": "fn check() { let mut values = Vec::new(); values.push(other()); values.join(\"payload.rs\"); }",
        "expected": [
          "payload.rs"
        ],
        "allArguments": [
          "payload.rs"
        ]
      },
      {
        "id": "untyped-empty",
        "source": "fn check() { let values = Vec::new(); values.join(\"payload.rs\"); }",
        "expected": [
          "payload.rs"
        ],
        "allArguments": [
          "payload.rs"
        ]
      },
      {
        "id": "custom-vec",
        "source": "struct Vec; impl Vec { fn new() -> Self { Self } fn push(&mut self, _: String) {} fn join(&self, value: &str) -> std::path::PathBuf { std::path::PathBuf::from(value) } } fn check() -> std::path::PathBuf { let mut values = Vec::new(); values.push(format!(\"a\")); values.join(\"payload.rs\") }",
        "expected": [
          "payload.rs"
        ],
        "allArguments": [
          "payload.rs"
        ],
        "compiler": "fn main() { let result: std::path::PathBuf = check(); assert_eq!(result, std::path::PathBuf::from(\"payload.rs\")); println!(\"path\"); }",
        "compilerOutput": "path"
      },
      {
        "id": "imported-vec",
        "source": "use custom::Vec; fn check() { let mut values = Vec::new(); values.push(format!(\"a\")); values.join(\"payload.rs\"); }",
        "expected": [
          "payload.rs"
        ],
        "allArguments": [
          "payload.rs"
        ]
      },
      {
        "id": "aliased-vec",
        "source": "use custom::PathList as Vec; fn check() { let mut values = Vec::new(); values.push(format!(\"a\")); values.join(\"payload.rs\"); }",
        "expected": [
          "payload.rs"
        ],
        "allArguments": [
          "payload.rs"
        ]
      },
      {
        "id": "generic-vec",
        "source": "fn check<Vec>() { let mut values = Vec::new(); values.push(format!(\"a\")); values.join(\"payload.rs\"); }",
        "expected": [
          "payload.rs"
        ],
        "allArguments": [
          "payload.rs"
        ]
      },
      {
        "id": "shadowed-std",
        "source": "mod std {} fn check() { let mut values = std::vec::Vec::new(); values.push(format!(\"a\")); values.join(\"payload.rs\"); }",
        "expected": [
          "payload.rs"
        ],
        "allArguments": [
          "payload.rs"
        ]
      },
      {
        "id": "custom-format",
        "source": "macro_rules! format { ($($x:tt)*) => { other() }; } fn check() { let mut values = Vec::new(); values.push(format!(\"a\")); values.join(\"payload.rs\"); }",
        "expected": [
          "payload.rs"
        ],
        "allArguments": [
          "payload.rs"
        ]
      },
      {
        "id": "wildcard-import",
        "source": "use custom::*; fn check() { let mut values = Vec::new(); values.push(format!(\"a\")); values.join(\"payload.rs\"); }",
        "expected": [
          "payload.rs"
        ],
        "allArguments": [
          "payload.rs"
        ]
      },
      {
        "id": "custom-join-trait",
        "source": "trait CustomJoin { fn join(&self, value: &str) -> std::path::PathBuf; } fn check() { let mut values = Vec::new(); values.push(\"a\"); values.join(\"payload.rs\"); }",
        "expected": [
          "payload.rs"
        ],
        "allArguments": [
          "payload.rs"
        ]
      },
      {
        "id": "unknown-newline-receiver",
        "source": "fn check() { let values = other(); values.join(\"\\n\"); }",
        "expected": [
          "\\n"
        ],
        "allArguments": [
          "\\n"
        ]
      },
      {
        "id": "path-and-delimiter",
        "source": "fn check() -> std::path::PathBuf { let mut values = Vec::new(); values.push(\"a\"); let _ = values.join(\"\\n\"); std::path::Path::new(\"base\").join(\"payload.rs\") }",
        "expected": [
          "payload.rs"
        ],
        "allArguments": [
          "\\n",
          "payload.rs"
        ],
        "compiler": "fn main() { let result: std::path::PathBuf = check(); assert_eq!(result, std::path::PathBuf::from(\"base\").join(\"payload.rs\")); println!(\"path\"); }",
        "compilerOutput": "path"
      },
      {
        "id": "macro-escape",
        "source": "fn check() { let mut values = Vec::new(); values.push(\"a\"); replace!(values); values.join(\"payload.rs\"); }",
        "expected": [
          "payload.rs"
        ],
        "allArguments": [
          "payload.rs"
        ]
      },
      {
        "id": "cross-function",
        "source": "fn first() { let mut values = Vec::new(); values.push(\"a\"); } fn second() { values.join(\"payload.rs\"); }",
        "expected": [
          "payload.rs"
        ],
        "allArguments": [
          "payload.rs"
        ]
      },
      {
        "id": "comment-and-string-decoys",
        "source": "fn check() { let mut values = Vec::new(); values.push(\"struct Vec; fn join() {}\"); /* use custom::*; */ values.join(\"\\n\"); }",
        "expected": [],
        "allArguments": [
          "\\n"
        ]
      },
      {
        "id": "typed-collect-initializer",
        "source": "fn check() -> String { let tokens: Vec<String> = [\"a\", \"b\"].iter().map(|value| value.to_string()).collect(); tokens.join(\",\") }",
        "expected": [],
        "allArguments": [
          ","
        ],
        "compiler": "fn main() { let result: String = check(); assert_eq!(result, \"a,b\"); println!(\"delimiter\"); }",
        "compilerOutput": "delimiter"
      },
      {
        "id": "trait-impl-for-precedes-collection",
        "source": "struct Foo; impl core::fmt::Display for Foo { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { write!(f, \"foo\") } } fn check() -> String { let mut values = Vec::new(); values.push(format!(\"a\")); values.join(\",\") }",
        "expected": [],
        "allArguments": [
          ","
        ],
        "compiler": "fn main() { let result: String = check(); assert_eq!(result, \"a\"); println!(\"delimiter\"); }",
        "compilerOutput": "delimiter"
      }
    ]
  },
  "tokenNarrowing": {
    "functions": ["inspectRustManifestPathReferences", "inspectRustJoinArgumentSpans"],
    "cases": [
      { "token": null, "expected": null },
      { "token": { "kind": "identifier", "text": "surface" }, "expected": "surface" },
      { "token": { "kind": "string", "text": "\"payload.json\"" }, "expected": "literal" },
      { "token": { "kind": "string", "text": "r#\"payload.json\"#" }, "expected": null },
      { "token": { "kind": "number", "text": "42" }, "expected": null },
      { "token": { "kind": "punctuation", "text": "." }, "expected": null }
    ]
  },
  "execution": {
    "target": "test-rust-physical-reference-context",
    "command": "bun ./📜️script.ts test rust-physical-reference-context long",
    "launchName": "🧹clean🧩️taxonomy🧲️rust-physical-reference-context",
    "launchCommand": "bun nx run @semio-tech/repo-lib:test-rust-physical-reference-context --skip-nx-cache",
    "launchGroup": "4_gate",
    "launchOrder": 410.08
  },
  "oracle": {
    "manifestInput": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔮️oracles/🧲️rust-physical-reference-context/⚙️.toml",
    "sourceInput": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔮️oracles/🧲️rust-physical-reference-context/🦀️.rs"
  },
  "transaction": {
    "scope": "🧪️tests/🧪️target",
    "source": "component.rs",
    "destination": "🦀️.rs",
    "bytes": "pub const VALUE: &str = \"right\";\n",
    "manifest": "pkg/Cargo.toml",
    "consumer": "pkg/entry.rs",
    "lookalike": "pkg/🦀️.rs",
    "runtimeOutput": "READ:pub const VALUE: &str = \"right\";"
  },
  "referenceUniverse": {
    "ownership": "no-follow-regular-manifests-and-exact-mounted-source-chains",
    "admission": "affected-consumer-or-possible-affected-physical-target",
    "unsafeUnrelatedManifest": "unrelated/Cargo.toml",
    "independentRoot": "🧪️transaction/🧪️independent",
    "cases": [
      { "id": "independent-local", "base": "../🧪️tests/🧪️target", "physicalTarget": "🧪️transaction/🧪️independent/🧪️tests/🦀️target.rs", "affectsParent": false },
      { "id": "independent-parent-escape", "base": "../../../🧪️tests/🧪️target", "physicalTarget": "🧪️tests/🦀️target.rs", "affectsParent": true }
    ]
  },
  "manifestCandidates": {
    "adversarial": {
  "contract": "rust-finite-candidate-adversarial-v1",
  "unknownControlFlow": "captured-bindings-remain-unproven",
  "namespace": "standard-type-and-macro-identity-required",
  "cases": [
    {
      "id": "captured-closure",
      "source": "fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); for leaf in [\"leaf.rs\"] { std::assert!(!leaf.is_empty(), \"{leaf}\"); std::println!(\"{}\", root.join(leaf).display()); let other = |_: ()| std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../energy\").join(leaf); std::println!(\"{}\", other(()).display()); } }",
      "expected": [],
      "runtimeProof": {
        "call": "inspect()",
        "targets": [
          "foreign/leaf.rs",
          "energy/leaf.rs"
        ]
      }
    },
    {
      "id": "unknown-range-loop",
      "source": "fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); for leaf in [\"leaf.rs\"] { std::assert!(!leaf.is_empty(), \"{leaf}\"); std::println!(\"{}\", root.join(leaf).display()); for _ in 0..1 { std::println!(\"{}\", std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../energy\").join(leaf).display()); } } }",
      "expected": [],
      "runtimeProof": {
        "call": "inspect()",
        "targets": [
          "foreign/leaf.rs",
          "energy/leaf.rs"
        ]
      }
    },
    {
      "id": "function-std-value",
      "source": "fn inspect(std: ()) { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); for leaf in [\"leaf.rs\"] { std::assert!(!leaf.is_empty(), \"{leaf}\"); std::println!(\"{}\", root.join(leaf).display()); } }",
      "expected": [
        {
          "value": "leaf.rs",
          "targets": [
            [
              "../foreign",
              "leaf.rs"
            ]
          ]
        }
      ],
      "runtimeProof": {
        "call": "inspect(())",
        "targets": [
          "foreign/leaf.rs"
        ]
      }
    },
    {
      "id": "macro-use-module",
      "source": "#[macro_use] mod provider; fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); for leaf in [\"leaf.rs\"] { std::assert!(!leaf.is_empty(), \"{leaf}\"); std::println!(\"{}\", root.join(leaf).display()); } }",
      "expected": [],
      "runtimeProof": {
        "call": "inspect()",
        "targets": [
          "pkg/foreign/leaf.rs"
        ],
        "files": {
          "provider.rs": "macro_rules! env { ($name:literal) => { concat!(::std::env!(\"CARGO_MANIFEST_DIR\"), \"/redirect\") }; }\n"
        }
      }
    }
  ],
  "namespaceReviews": [
    {"id":"included-environment-macro","source":"include!(\"provider.rs\"); fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); for leaf in [\"leaf.rs\"] { std::assert!(!leaf.is_empty(), \"{leaf}\"); std::println!(\"{}\", root.join(leaf).display()); } }\nfn main() { inspect(); }\n","errorCode":"E0659","files":{"provider.rs":"macro_rules! env { ($name:literal) => { concat!(::std::env!(\"CARGO_MANIFEST_DIR\"), \"/redirect\") }; }\n"}},
    {
      "id": "generic-std-type",
      "source": "fn inspect<std>() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); for leaf in [\"leaf.rs\"] { ::std::println!(\"{}\", root.join(leaf).display()); } }\nfn main() { inspect::<()>(); }\n",
      "errorCode": "E0220"
    }
  ]
},
    "contract": "rust-finite-manifest-path-candidates-v1",
    "authority": "candidate-only-never-editable",
    "missingFact": "unproven-never-complete-empty",
    "grammar": [
      "exact-standard-manifest-root",
      "immutable-literal-string-or-tuple-array",
      "literal-number-boolean-and-full-slice-metadata",
      "lexical-destructured-for",
      "exact-array-iter-enumerate",
      "immutable-join-chain",
      "known-standard-read-only-macro-use"
    ],
    "maxExpandedIterations": 256,
    "maxTargetsPerSpan": 256,
    "tupleCorrelation": "per-row-environment",
    "coordinates": "relative-component-chains-only",
    "environment": "standard-env-macro-with-no-shadow-or-foreign-glob",
    "literal": "unescaped-normal-string-only",
    "manifestPath": "pkg/Cargo.toml",
    "consumerPath": "pkg/lib.rs",
    "affectedRoot": "energy",
    "selectedValue": "leaf.rs",
    "cases": [
      {
        "id": "shadowed-environment-macro",
        "source": "macro_rules! env { ($x:literal) => { \"/other\" }; } fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); let owners = [(\"first\", \"one\"), (\"second\", \"two\")]; for (label, directory) in owners { let owner = root.join(directory); for leaf in [\"leaf.rs\"] { assert!(owner.join(leaf).is_file(), \"{label}: {directory}/{leaf}\"); } } }",
        "expected": [],
        "physicalTargets": [],
        "relevance": "unproven"
      },
      {
        "id": "imported-environment-macro",
        "source": "use foreign::env; fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); let owners = [(\"first\", \"one\"), (\"second\", \"two\")]; for (label, directory) in owners { let owner = root.join(directory); for leaf in [\"leaf.rs\"] { assert!(owner.join(leaf).is_file(), \"{label}: {directory}/{leaf}\"); } } }",
        "expected": [],
        "physicalTargets": [],
        "relevance": "unproven"
      },
      {
        "id": "no-implicit-prelude",
        "source": "#![no_implicit_prelude] fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); let owners = [(\"first\", \"one\"), (\"second\", \"two\")]; for (label, directory) in owners { let owner = root.join(directory); for leaf in [\"leaf.rs\"] { assert!(owner.join(leaf).is_file(), \"{label}: {directory}/{leaf}\"); } } }",
        "expected": [],
        "physicalTargets": [],
        "relevance": "unproven"
      },
      {
        "id": "no-standard-prelude",
        "source": "#![no_std] fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); let owners = [(\"first\", \"one\"), (\"second\", \"two\")]; for (label, directory) in owners { let owner = root.join(directory); for leaf in [\"leaf.rs\"] { assert!(owner.join(leaf).is_file(), \"{label}: {directory}/{leaf}\"); } } }",
        "expected": [],
        "physicalTargets": [],
        "relevance": "unproven"
      },
      {
        "id": "standard-qualified-despite-named-import",
        "source": "use foreign::assert; fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); let owners = [(\"first\", \"one\"), (\"second\", \"two\")]; for (label, directory) in owners { let owner = root.join(directory); for leaf in [\"leaf.rs\"] { ::std::assert!(owner.join(leaf).is_file(), \"{label}: {directory}/{leaf}\"); } } }",
        "expected": [
          {
            "value": "leaf.rs",
            "targets": [
              [
                "../foreign",
                "one",
                "leaf.rs"
              ],
              [
                "../foreign",
                "two",
                "leaf.rs"
              ]
            ]
          }
        ],
        "physicalTargets": [
          "foreign/one/leaf.rs",
          "foreign/two/leaf.rs"
        ],
        "relevance": "disjoint"
      },
      {
        "id": "duplicate-tuple-binding",
        "source": "fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); let owners = [(\"first\", \"one\"), (\"second\", \"two\")]; for (directory, directory) in owners { let owner = root.join(directory); for leaf in [\"leaf.rs\"] { assert!(owner.join(leaf).is_file(), \"{label}: {directory}/{leaf}\"); } } }",
        "expected": [],
        "physicalTargets": [],
        "relevance": "unproven"
      },
      {
        "id": "absolute-manifest-component",
        "source": "fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"/foreign\"); let owners = [(\"first\", \"one\"), (\"second\", \"two\")]; for (label, directory) in owners { let owner = root.join(directory); for leaf in [\"leaf.rs\"] { assert!(owner.join(leaf).is_file(), \"{label}: {directory}/{leaf}\"); } } }",
        "expected": [],
        "physicalTargets": [],
        "relevance": "unproven"
      },
      {
        "id": "drive-manifest-component",
        "source": "fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"C:/foreign\"); let owners = [(\"first\", \"one\"), (\"second\", \"two\")]; for (label, directory) in owners { let owner = root.join(directory); for leaf in [\"leaf.rs\"] { assert!(owner.join(leaf).is_file(), \"{label}: {directory}/{leaf}\"); } } }",
        "expected": [],
        "physicalTargets": [],
        "relevance": "unproven"
      },
      {
        "id": "standard-qualified-despite-foreign-glob",
        "source": "use foreign::*; fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); let owners = [(\"first\", \"one\"), (\"second\", \"two\")]; for (label, directory) in owners { let owner = root.join(directory); for leaf in [\"leaf.rs\"] { ::std::assert!(owner.join(leaf).is_file(), \"{label}: {directory}/{leaf}\"); } } }",
        "expected": [],
        "physicalTargets": [],
        "relevance": "unproven"
      },
      {
        "id": "same-file-parent-glob",
        "source": "mod tests { use super::*; fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); let owners = [(\"first\", \"one\"), (\"second\", \"two\")]; for (label, directory) in owners { let owner = root.join(directory); for leaf in [\"leaf.rs\"] { assert!(owner.join(leaf).is_file(), \"{label}: {directory}/{leaf}\"); } } } }",
        "expected": [
          {
            "value": "leaf.rs",
            "targets": [
              [
                "../foreign",
                "one",
                "leaf.rs"
              ],
              [
                "../foreign",
                "two",
                "leaf.rs"
              ]
            ]
          }
        ],
        "physicalTargets": [
          "foreign/one/leaf.rs",
          "foreign/two/leaf.rs"
        ],
        "relevance": "disjoint"
      },
      {
        "id": "unicode-source-span",
        "source": "/*🧭*/ fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); let owners = [(\"first\", \"one🧭\"), (\"second\", \"two\")]; for (label, directory) in owners { let owner = root.join(directory); for leaf in [\"leaf.rs\"] { assert!(owner.join(leaf).is_file(), \"{label}: {directory}/{leaf}\"); } } }",
        "expected": [
          {
            "value": "leaf.rs",
            "targets": [
              [
                "../foreign",
                "one🧭",
                "leaf.rs"
              ],
              [
                "../foreign",
                "two",
                "leaf.rs"
              ]
            ]
          }
        ],
        "physicalTargets": [
          "foreign/one🧭/leaf.rs",
          "foreign/two/leaf.rs"
        ],
        "relevance": "disjoint"
      },
      {
        "id": "foreign-nested-std-macro",
        "source": "fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); let owners = [(\"first\", \"one\"), (\"second\", \"two\")]; for (label, directory) in owners { let owner = root.join(directory); for leaf in [\"leaf.rs\"] { foreign::std::assert!(owner.join(leaf).is_file(), \"{label}: {directory}/{leaf}\"); } } }",
        "expected": [],
        "physicalTargets": [],
        "relevance": "unproven"
      },
      {
        "id": "imported-assert-macro",
        "source": "use foreign::assert; fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); let owners = [(\"first\", \"one\"), (\"second\", \"two\")]; for (label, directory) in owners { let owner = root.join(directory); for leaf in [\"leaf.rs\"] { assert!(owner.join(leaf).is_file(), \"{label}: {directory}/{leaf}\"); } } }",
        "expected": [],
        "physicalTargets": [],
        "relevance": "unproven"
      },
      {
        "id": "foreign-glob-macro",
        "source": "use foreign::*; fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); let owners = [(\"first\", \"one\"), (\"second\", \"two\")]; for (label, directory) in owners { let owner = root.join(directory); for leaf in [\"leaf.rs\"] { assert!(owner.join(leaf).is_file(), \"{label}: {directory}/{leaf}\"); } } }",
        "expected": [],
        "physicalTargets": [],
        "relevance": "unproven"
      },
      {
        "id": "shadowed-standard-macro",
        "source": "macro_rules! assert { ($($x:tt)*) => {}; } fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); let owners = [(\"first\", \"one\"), (\"second\", \"two\")]; for (label, directory) in owners { let owner = root.join(directory); for leaf in [\"leaf.rs\"] { assert!(owner.join(leaf).is_file(), \"{label}: {directory}/{leaf}\"); } } }",
        "expected": [],
        "physicalTargets": [],
        "relevance": "unproven"
      },
      {
        "id": "unproven-qualified-macro",
        "source": "fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); let owners = [(\"first\", \"one\"), (\"second\", \"two\")]; for (label, directory) in owners { let owner = root.join(directory); for leaf in [\"leaf.rs\"] { foreign::assert!(owner.join(leaf).is_file(), \"{label}: {directory}/{leaf}\"); } } }",
        "expected": [],
        "physicalTargets": [],
        "relevance": "unproven"
      },
      {
        "id": "partial-tuple-binding",
        "source": "fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); let owners = [(\"first\", \"one\"), (\"second\", \"two\")]; for (label, directory, extra) in owners { let owner = root.join(directory); for leaf in [\"leaf.rs\"] { assert!(owner.join(leaf).is_file(), \"{label}: {directory}/{leaf}\"); } } }",
        "expected": [],
        "physicalTargets": [],
        "relevance": "unproven"
      },
      {
        "id": "unknown-environment",
        "source": "fn inspect() { let root = std::path::Path::new(env!(\"OTHER_DIR\")).join(\"../foreign\"); let owners = [(\"first\", \"one\"), (\"second\", \"two\")]; for (label, directory) in owners { let owner = root.join(directory); for leaf in [\"leaf.rs\"] { assert!(owner.join(leaf).is_file(), \"{label}: {directory}/{leaf}\"); } } }",
        "expected": [],
        "physicalTargets": [],
        "relevance": "unproven"
      },
      {
        "id": "unsupported-constructor",
        "source": "fn inspect() { let root = std::path::PathBuf::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); let owners = [(\"first\", \"one\"), (\"second\", \"two\")]; for (label, directory) in owners { let owner = root.join(directory); for leaf in [\"leaf.rs\"] { assert!(owner.join(leaf).is_file(), \"{label}: {directory}/{leaf}\"); } } }",
        "expected": [],
        "physicalTargets": [],
        "relevance": "unproven"
      },
      {
        "id": "conditional-mutation",
        "source": "fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); let owners = [(\"first\", \"one\"), (\"second\", \"two\")]; for (label, directory) in owners { let owner = root.join(directory); if let Some(owner) = other() { escape(owner); } for leaf in [\"leaf.rs\"] { assert!(owner.join(leaf).is_file(), \"{label}: {directory}/{leaf}\"); } } }",
        "expected": [],
        "physicalTargets": [],
        "relevance": "unproven"
      },
      {
        "id": "escaping-after-candidate",
        "source": "fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); let owners = [(\"first\", \"one\"), (\"second\", \"two\")]; for (label, directory) in owners { let owner = root.join(directory); for leaf in [\"leaf.rs\"] { assert!(owner.join(leaf).is_file(), \"{label}: {directory}/{leaf}\"); escape(&owner); } } }",
        "expected": [],
        "physicalTargets": [],
        "relevance": "unproven"
      },
      {
        "id": "tuple-shared-label-disjoint",
        "source": "fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); let owners = [(\"first\", \"one\"), (\"second\", \"two\")]; for (label, directory) in owners { let owner = root.join(directory); for leaf in [\"leaf.rs\"] { assert!(owner.join(leaf).is_file(), \"{label}: {directory}/{leaf}\"); } } }",
        "expected": [
          {
            "value": "leaf.rs",
            "targets": [
              [
                "../foreign",
                "one",
                "leaf.rs"
              ],
              [
                "../foreign",
                "two",
                "leaf.rs"
              ]
            ]
          }
        ],
        "physicalTargets": [
          "foreign/one/leaf.rs",
          "foreign/two/leaf.rs"
        ],
        "relevance": "disjoint",
        "runtime": true
      },
      {
        "id": "tuple-shared-label-intersection",
        "source": "fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); let owners = [(\"first\", \"one\"), (\"second\", \"../energy\")]; for (label, directory) in owners { let owner = root.join(directory); for leaf in [\"leaf.rs\"] { assert!(owner.join(leaf).is_file(), \"{label}: {directory}/{leaf}\"); } } }",
        "expected": [
          {
            "value": "leaf.rs",
            "targets": [
              [
                "../foreign",
                "../energy",
                "leaf.rs"
              ],
              [
                "../foreign",
                "one",
                "leaf.rs"
              ]
            ]
          }
        ],
        "physicalTargets": [
          "energy/leaf.rs",
          "foreign/one/leaf.rs"
        ],
        "relevance": "intersects",
        "runtime": true
      },
      {
        "id": "tuple-row-correlation",
        "source": "fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); let owners = [(\"one\", \"a\"), (\"two\", \"b\")]; for (parent, child) in owners { let owner = root.join(parent).join(child); for leaf in [\"leaf.rs\"] { assert!(owner.join(leaf).is_file(), \"{parent}/{child}/{leaf}\"); } } }",
        "expected": [
          {
            "value": "leaf.rs",
            "targets": [
              [
                "../foreign",
                "one",
                "a",
                "leaf.rs"
              ],
              [
                "../foreign",
                "two",
                "b",
                "leaf.rs"
              ]
            ]
          }
        ],
        "physicalTargets": [
          "foreign/one/a/leaf.rs",
          "foreign/two/b/leaf.rs"
        ],
        "relevance": "disjoint",
        "runtime": true
      },
      {
        "id": "five-column-literal-metadata",
        "source": "fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); let owners = [(\"first\", \"First\", \"one\", 0, &[\"ok\", \"warning\"][..]), (\"second\", \"Second\", \"two\", 1, &[\"ok\"][..])]; for (kind, variant, directory, tag, outcomes) in owners { let owner = root.join(directory); for leaf in [\"leaf.rs\"] { assert!(owner.join(leaf).is_file(), \"{kind}/{variant}/{tag}/{leaf}\"); let _ = outcomes; } } }",
        "expected": [
          {
            "value": "leaf.rs",
            "targets": [
              [
                "../foreign",
                "one",
                "leaf.rs"
              ],
              [
                "../foreign",
                "two",
                "leaf.rs"
              ]
            ]
          }
        ],
        "physicalTargets": [
          "foreign/one/leaf.rs",
          "foreign/two/leaf.rs"
        ],
        "relevance": "disjoint"
      },
      {
        "id": "enumerated-array",
        "source": "fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); let owners = [\"one\", \"two\"]; for (index, owner) in owners.iter().enumerate() { assert!(root.join(owner).join(\"leaf.rs\").is_file(), \"{index}/{owner}\"); } }",
        "expected": [
          {
            "value": "leaf.rs",
            "targets": [
              [
                "../foreign",
                "one",
                "leaf.rs"
              ],
              [
                "../foreign",
                "two",
                "leaf.rs"
              ]
            ]
          }
        ],
        "physicalTargets": [
          "foreign/one/leaf.rs",
          "foreign/two/leaf.rs"
        ],
        "relevance": "disjoint",
        "runtime": true
      },
      {
        "id": "independent-inner-array",
        "source": "fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); let owners = [(\"first\", \"one\"), (\"second\", \"two\")]; for (label, directory) in owners { let owner = root.join(directory); for leaf in [\"leaf.rs\", \"second.rs\"] { assert!(owner.join(leaf).is_file(), \"{label}/{leaf}\"); } } }",
        "expected": [
          {
            "value": "leaf.rs",
            "targets": [
              [
                "../foreign",
                "one",
                "leaf.rs"
              ],
              [
                "../foreign",
                "two",
                "leaf.rs"
              ]
            ]
          }
        ],
        "physicalTargets": [
          "foreign/one/leaf.rs",
          "foreign/two/leaf.rs"
        ],
        "relevance": "disjoint"
      },
      {
        "id": "pathbuf-shared-array",
        "source": "fn inspect() { let root = ::std::path::PathBuf::from(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); for leaf in [\"leaf.rs\"] { assert!(root.join(leaf).is_file(), \"{leaf}\"); } }",
        "expected": [
          {
            "value": "leaf.rs",
            "targets": [
              [
                "../foreign",
                "leaf.rs"
              ]
            ]
          }
        ],
        "physicalTargets": [
          "foreign/leaf.rs"
        ],
        "relevance": "disjoint",
        "runtime": true
      },
      {
        "id": "duplicate-value-distinct-spans",
        "source": "fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); let owners = [(\"one\", \"leaf.rs\"), (\"two\", \"leaf.rs\")]; for (directory, leaf) in owners { assert!(root.join(directory).join(leaf).is_file(), \"{directory}/{leaf}\"); } }",
        "expected": [
          {
            "value": "leaf.rs",
            "targets": [
              [
                "../foreign",
                "one",
                "leaf.rs"
              ]
            ]
          },
          {
            "value": "leaf.rs",
            "targets": [
              [
                "../foreign",
                "two",
                "leaf.rs"
              ]
            ]
          }
        ],
        "physicalTargets": [
          "foreign/one/leaf.rs",
          "foreign/two/leaf.rs"
        ],
        "relevance": "disjoint"
      },
      {
        "id": "dynamic-array",
        "source": "fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); let owners = owners(); for (label, directory) in owners { let owner = root.join(directory); for leaf in [\"leaf.rs\"] { assert!(owner.join(leaf).is_file(), \"{label}: {directory}/{leaf}\"); } } }",
        "expected": [],
        "physicalTargets": [],
        "relevance": "unproven"
      },
      {
        "id": "dynamic-tuple-column",
        "source": "fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); let owners = [(\"first\", \"one\"), (\"second\", directory())]; for (label, directory) in owners { let owner = root.join(directory); for leaf in [\"leaf.rs\"] { assert!(owner.join(leaf).is_file(), \"{label}: {directory}/{leaf}\"); } } }",
        "expected": [],
        "physicalTargets": [],
        "relevance": "unproven"
      },
      {
        "id": "mutable-root",
        "source": "fn inspect() { let mut root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); let owners = [(\"first\", \"one\"), (\"second\", \"two\")]; for (label, directory) in owners { let owner = root.join(directory); for leaf in [\"leaf.rs\"] { assert!(owner.join(leaf).is_file(), \"{label}: {directory}/{leaf}\"); } } }",
        "expected": [],
        "physicalTargets": [],
        "relevance": "unproven"
      },
      {
        "id": "mutable-array",
        "source": "fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); let mut owners = [(\"first\", \"one\"), (\"second\", \"two\")]; for (label, directory) in owners { let owner = root.join(directory); for leaf in [\"leaf.rs\"] { assert!(owner.join(leaf).is_file(), \"{label}: {directory}/{leaf}\"); } } }",
        "expected": [],
        "physicalTargets": [],
        "relevance": "unproven"
      },
      {
        "id": "mutable-loop-binding",
        "source": "fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); let owners = [(\"first\", \"one\"), (\"second\", \"two\")]; for (label, mut directory) in owners { let owner = root.join(directory); for leaf in [\"leaf.rs\"] { assert!(owner.join(leaf).is_file(), \"{label}: {directory}/{leaf}\"); } } }",
        "expected": [],
        "physicalTargets": [],
        "relevance": "unproven"
      },
      {
        "id": "mutated-array",
        "source": "fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); let owners = [(\"first\", \"one\"), (\"second\", \"two\")]; owners[0] = (\"third\", \"three\"); for (label, directory) in owners { let owner = root.join(directory); for leaf in [\"leaf.rs\"] { assert!(owner.join(leaf).is_file(), \"{label}: {directory}/{leaf}\"); } } }",
        "expected": [],
        "physicalTargets": [],
        "relevance": "unproven"
      },
      {
        "id": "escaped-receiver",
        "source": "fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); let owners = [(\"first\", \"one\"), (\"second\", \"two\")]; for (label, directory) in owners { let owner = root.join(directory); escape(&owner); for leaf in [\"leaf.rs\"] { assert!(owner.join(leaf).is_file(), \"{label}: {directory}/{leaf}\"); } } }",
        "expected": [],
        "physicalTargets": [],
        "relevance": "unproven"
      },
      {
        "id": "unknown-macro",
        "source": "fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); let owners = [(\"first\", \"one\"), (\"second\", \"two\")]; for (label, directory) in owners { let owner = root.join(directory); for leaf in [\"leaf.rs\"] { opaque!(leaf); assert!(owner.join(leaf).is_file(), \"{label}: {directory}/{leaf}\"); } } }",
        "expected": [],
        "physicalTargets": [],
        "relevance": "unproven"
      },
      {
        "id": "loop-shadow",
        "source": "fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); let owners = [(\"first\", \"one\"), (\"second\", \"two\")]; for (label, directory) in owners { let owner = root.join(directory); for leaf in [\"leaf.rs\"] { let owner = other(); assert!(owner.join(leaf).is_file(), \"{label}: {directory}/{leaf}\"); } } }",
        "expected": [],
        "physicalTargets": [],
        "relevance": "unproven"
      },
      {
        "id": "wrong-enumeration",
        "source": "fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); let owners = [\"one\", \"two\"]; for (index, owner) in owners.into_iter().enumerate() { root.join(owner).join(\"leaf.rs\"); } }",
        "expected": [],
        "physicalTargets": [],
        "relevance": "unproven"
      },
      {
        "id": "raw-target",
        "source": "fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); let owners = [(\"first\", \"one\"), (\"second\", \"two\")]; for (label, directory) in owners { let owner = root.join(directory); for leaf in [r\"leaf.rs\"] { assert!(owner.join(leaf).is_file(), \"{label}: {directory}/{leaf}\"); } } }",
        "expected": [],
        "physicalTargets": [],
        "relevance": "unproven"
      },
      {
        "id": "escaped-target",
        "source": "fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); let owners = [(\"first\", \"one\"), (\"second\", \"two\")]; for (label, directory) in owners { let owner = root.join(directory); for leaf in [\"leaf\\\\.rs\"] { assert!(owner.join(leaf).is_file(), \"{label}: {directory}/{leaf}\"); } } }",
        "expected": [],
        "physicalTargets": [],
        "relevance": "unproven"
      },
      {
        "id": "mixed-receivers",
        "source": "fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); let owners = [(\"first\", \"one\"), (\"second\", \"two\")]; for (label, directory) in owners { let owner = root.join(directory); for leaf in [\"leaf.rs\"] { assert!(owner.join(leaf).is_file(), \"{label}/{leaf}\"); other().join(leaf); } } }",
        "expected": [],
        "physicalTargets": [],
        "relevance": "unproven"
      },
      {
        "id": "foreign-std",
        "source": "mod std {} fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); let owners = [(\"first\", \"one\"), (\"second\", \"two\")]; for (label, directory) in owners { let owner = root.join(directory); for leaf in [\"leaf.rs\"] { assert!(owner.join(leaf).is_file(), \"{label}: {directory}/{leaf}\"); } } }",
        "expected": [],
        "physicalTargets": [],
        "relevance": "unproven"
      },
      {
        "id": "empty-array",
        "source": "fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); let owners = []; for (label, directory) in owners { let owner = root.join(directory); for leaf in [\"leaf.rs\"] { assert!(owner.join(leaf).is_file(), \"{label}: {directory}/{leaf}\"); } } }",
        "expected": [],
        "physicalTargets": [],
        "relevance": "unproven"
      },
      {
        "id": "over-limit",
        "source": "fn inspect() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); let owners = [(\"label0\", \"dir0\"), (\"label1\", \"dir1\"), (\"label2\", \"dir2\"), (\"label3\", \"dir3\"), (\"label4\", \"dir4\"), (\"label5\", \"dir5\"), (\"label6\", \"dir6\"), (\"label7\", \"dir7\"), (\"label8\", \"dir8\"), (\"label9\", \"dir9\"), (\"label10\", \"dir10\"), (\"label11\", \"dir11\"), (\"label12\", \"dir12\"), (\"label13\", \"dir13\"), (\"label14\", \"dir14\"), (\"label15\", \"dir15\"), (\"label16\", \"dir16\"), (\"label17\", \"dir17\"), (\"label18\", \"dir18\"), (\"label19\", \"dir19\"), (\"label20\", \"dir20\"), (\"label21\", \"dir21\"), (\"label22\", \"dir22\"), (\"label23\", \"dir23\"), (\"label24\", \"dir24\"), (\"label25\", \"dir25\"), (\"label26\", \"dir26\"), (\"label27\", \"dir27\"), (\"label28\", \"dir28\"), (\"label29\", \"dir29\"), (\"label30\", \"dir30\"), (\"label31\", \"dir31\"), (\"label32\", \"dir32\"), (\"label33\", \"dir33\"), (\"label34\", \"dir34\"), (\"label35\", \"dir35\"), (\"label36\", \"dir36\"), (\"label37\", \"dir37\"), (\"label38\", \"dir38\"), (\"label39\", \"dir39\"), (\"label40\", \"dir40\"), (\"label41\", \"dir41\"), (\"label42\", \"dir42\"), (\"label43\", \"dir43\"), (\"label44\", \"dir44\"), (\"label45\", \"dir45\"), (\"label46\", \"dir46\"), (\"label47\", \"dir47\"), (\"label48\", \"dir48\"), (\"label49\", \"dir49\"), (\"label50\", \"dir50\"), (\"label51\", \"dir51\"), (\"label52\", \"dir52\"), (\"label53\", \"dir53\"), (\"label54\", \"dir54\"), (\"label55\", \"dir55\"), (\"label56\", \"dir56\"), (\"label57\", \"dir57\"), (\"label58\", \"dir58\"), (\"label59\", \"dir59\"), (\"label60\", \"dir60\"), (\"label61\", \"dir61\"), (\"label62\", \"dir62\"), (\"label63\", \"dir63\"), (\"label64\", \"dir64\"), (\"label65\", \"dir65\"), (\"label66\", \"dir66\"), (\"label67\", \"dir67\"), (\"label68\", \"dir68\"), (\"label69\", \"dir69\"), (\"label70\", \"dir70\"), (\"label71\", \"dir71\"), (\"label72\", \"dir72\"), (\"label73\", \"dir73\"), (\"label74\", \"dir74\"), (\"label75\", \"dir75\"), (\"label76\", \"dir76\"), (\"label77\", \"dir77\"), (\"label78\", \"dir78\"), (\"label79\", \"dir79\"), (\"label80\", \"dir80\"), (\"label81\", \"dir81\"), (\"label82\", \"dir82\"), (\"label83\", \"dir83\"), (\"label84\", \"dir84\"), (\"label85\", \"dir85\"), (\"label86\", \"dir86\"), (\"label87\", \"dir87\"), (\"label88\", \"dir88\"), (\"label89\", \"dir89\"), (\"label90\", \"dir90\"), (\"label91\", \"dir91\"), (\"label92\", \"dir92\"), (\"label93\", \"dir93\"), (\"label94\", \"dir94\"), (\"label95\", \"dir95\"), (\"label96\", \"dir96\"), (\"label97\", \"dir97\"), (\"label98\", \"dir98\"), (\"label99\", \"dir99\"), (\"label100\", \"dir100\"), (\"label101\", \"dir101\"), (\"label102\", \"dir102\"), (\"label103\", \"dir103\"), (\"label104\", \"dir104\"), (\"label105\", \"dir105\"), (\"label106\", \"dir106\"), (\"label107\", \"dir107\"), (\"label108\", \"dir108\"), (\"label109\", \"dir109\"), (\"label110\", \"dir110\"), (\"label111\", \"dir111\"), (\"label112\", \"dir112\"), (\"label113\", \"dir113\"), (\"label114\", \"dir114\"), (\"label115\", \"dir115\"), (\"label116\", \"dir116\"), (\"label117\", \"dir117\"), (\"label118\", \"dir118\"), (\"label119\", \"dir119\"), (\"label120\", \"dir120\"), (\"label121\", \"dir121\"), (\"label122\", \"dir122\"), (\"label123\", \"dir123\"), (\"label124\", \"dir124\"), (\"label125\", \"dir125\"), (\"label126\", \"dir126\"), (\"label127\", \"dir127\"), (\"label128\", \"dir128\"), (\"label129\", \"dir129\"), (\"label130\", \"dir130\"), (\"label131\", \"dir131\"), (\"label132\", \"dir132\"), (\"label133\", \"dir133\"), (\"label134\", \"dir134\"), (\"label135\", \"dir135\"), (\"label136\", \"dir136\"), (\"label137\", \"dir137\"), (\"label138\", \"dir138\"), (\"label139\", \"dir139\"), (\"label140\", \"dir140\"), (\"label141\", \"dir141\"), (\"label142\", \"dir142\"), (\"label143\", \"dir143\"), (\"label144\", \"dir144\"), (\"label145\", \"dir145\"), (\"label146\", \"dir146\"), (\"label147\", \"dir147\"), (\"label148\", \"dir148\"), (\"label149\", \"dir149\"), (\"label150\", \"dir150\"), (\"label151\", \"dir151\"), (\"label152\", \"dir152\"), (\"label153\", \"dir153\"), (\"label154\", \"dir154\"), (\"label155\", \"dir155\"), (\"label156\", \"dir156\"), (\"label157\", \"dir157\"), (\"label158\", \"dir158\"), (\"label159\", \"dir159\"), (\"label160\", \"dir160\"), (\"label161\", \"dir161\"), (\"label162\", \"dir162\"), (\"label163\", \"dir163\"), (\"label164\", \"dir164\"), (\"label165\", \"dir165\"), (\"label166\", \"dir166\"), (\"label167\", \"dir167\"), (\"label168\", \"dir168\"), (\"label169\", \"dir169\"), (\"label170\", \"dir170\"), (\"label171\", \"dir171\"), (\"label172\", \"dir172\"), (\"label173\", \"dir173\"), (\"label174\", \"dir174\"), (\"label175\", \"dir175\"), (\"label176\", \"dir176\"), (\"label177\", \"dir177\"), (\"label178\", \"dir178\"), (\"label179\", \"dir179\"), (\"label180\", \"dir180\"), (\"label181\", \"dir181\"), (\"label182\", \"dir182\"), (\"label183\", \"dir183\"), (\"label184\", \"dir184\"), (\"label185\", \"dir185\"), (\"label186\", \"dir186\"), (\"label187\", \"dir187\"), (\"label188\", \"dir188\"), (\"label189\", \"dir189\"), (\"label190\", \"dir190\"), (\"label191\", \"dir191\"), (\"label192\", \"dir192\"), (\"label193\", \"dir193\"), (\"label194\", \"dir194\"), (\"label195\", \"dir195\"), (\"label196\", \"dir196\"), (\"label197\", \"dir197\"), (\"label198\", \"dir198\"), (\"label199\", \"dir199\"), (\"label200\", \"dir200\"), (\"label201\", \"dir201\"), (\"label202\", \"dir202\"), (\"label203\", \"dir203\"), (\"label204\", \"dir204\"), (\"label205\", \"dir205\"), (\"label206\", \"dir206\"), (\"label207\", \"dir207\"), (\"label208\", \"dir208\"), (\"label209\", \"dir209\"), (\"label210\", \"dir210\"), (\"label211\", \"dir211\"), (\"label212\", \"dir212\"), (\"label213\", \"dir213\"), (\"label214\", \"dir214\"), (\"label215\", \"dir215\"), (\"label216\", \"dir216\"), (\"label217\", \"dir217\"), (\"label218\", \"dir218\"), (\"label219\", \"dir219\"), (\"label220\", \"dir220\"), (\"label221\", \"dir221\"), (\"label222\", \"dir222\"), (\"label223\", \"dir223\"), (\"label224\", \"dir224\"), (\"label225\", \"dir225\"), (\"label226\", \"dir226\"), (\"label227\", \"dir227\"), (\"label228\", \"dir228\"), (\"label229\", \"dir229\"), (\"label230\", \"dir230\"), (\"label231\", \"dir231\"), (\"label232\", \"dir232\"), (\"label233\", \"dir233\"), (\"label234\", \"dir234\"), (\"label235\", \"dir235\"), (\"label236\", \"dir236\"), (\"label237\", \"dir237\"), (\"label238\", \"dir238\"), (\"label239\", \"dir239\"), (\"label240\", \"dir240\"), (\"label241\", \"dir241\"), (\"label242\", \"dir242\"), (\"label243\", \"dir243\"), (\"label244\", \"dir244\"), (\"label245\", \"dir245\"), (\"label246\", \"dir246\"), (\"label247\", \"dir247\"), (\"label248\", \"dir248\"), (\"label249\", \"dir249\"), (\"label250\", \"dir250\"), (\"label251\", \"dir251\"), (\"label252\", \"dir252\"), (\"label253\", \"dir253\"), (\"label254\", \"dir254\"), (\"label255\", \"dir255\"), (\"label256\", \"dir256\")]; for (label, directory) in owners { let owner = root.join(directory); for leaf in [\"leaf.rs\"] { assert!(owner.join(leaf).is_file(), \"{label}: {directory}/{leaf}\"); } } }",
        "expected": [],
        "physicalTargets": [],
        "relevance": "unproven"
      }
    ]
  },
  "manifestPaths": {
    "roots": ["std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\"))", "std::path::PathBuf::from(env!(\"CARGO_MANIFEST_DIR\"))"],
    "binding": "immutable-lexical-local-only",
    "literal": "unescaped-normal-string-only",
    "loop": "literal-array-variable-used-only-as-one-proven-join-argument",
    "cases": [
      { "id": "direct", "source": "fn check() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../../artifacts\"); std::fs::read_to_string(root.join(\"payload.json\")); }", "expected": [{ "value": "../../artifacts", "base": [] }, { "value": "payload.json", "base": ["../../artifacts"] }] },
      { "id": "nested-owner-loop", "source": "fn check() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../../artifacts\"); let owner = root.join(\"replace-model\"); for surface in [\"code.rs\", \"text/code.rs\"] { let source = std::fs::read_to_string(owner.join(surface)); } }", "expected": [{ "value": "../../artifacts", "base": [] }, { "value": "replace-model", "base": ["../../artifacts"] }, { "value": "code.rs", "base": ["../../artifacts", "replace-model"] }, { "value": "text/code.rs", "base": ["../../artifacts", "replace-model"] }] },
      { "id": "shadowed", "source": "fn check() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")); let root = other(); root.join(\"payload.json\"); }", "expected": [] },
      { "id": "destructured-shadow", "source": "fn check() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")); let (root, _) = other(); root.join(\"payload.json\"); }", "expected": [] },
      { "id": "closure-shadow", "source": "fn check() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")); let reader = |root| root.join(\"payload.json\"); }", "expected": [] },
      { "id": "if-let-shadow", "source": "fn check() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")); if let Some(root) = other() { root.join(\"wrong.json\"); } root.join(\"right.json\"); }", "expected": [{ "value": "right.json", "base": [] }] },
      { "id": "match-shadow", "source": "fn check() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")); match other() { Some(root) => { root.join(\"wrong.json\"); }, None => { root.join(\"right.json\"); } } }", "expected": [{ "value": "right.json", "base": [] }] },
      { "id": "mutable", "source": "fn check() { let mut root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")); root.push(\"other\"); root.join(\"payload.json\"); }", "expected": [] },
      { "id": "scope", "source": "fn check() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")); { let root = other(); root.join(\"wrong.json\"); } root.join(\"right.json\"); }", "expected": [{ "value": "right.json", "base": [] }] },
      { "id": "unknown-constructor", "source": "fn check() { let root = Path::new(env!(\"CARGO_MANIFEST_DIR\")); root.join(\"payload.json\"); }", "expected": [] },
      { "id": "wrong-environment", "source": "fn check() { let root = std::path::Path::new(env!(\"OTHER_ROOT\")); root.join(\"payload.json\"); }", "expected": [] },
      { "id": "loop-label-use", "source": "fn check() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")); for path in [\"payload.json\"] { print!(\"{}\", path); root.join(path); } }", "expected": [] },
      { "id": "loop-two-bases", "source": "fn check() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")); let other = root.join(\"nested\"); for path in [\"payload.json\"] { root.join(path); other.join(path); } }", "expected": [{ "value": "nested", "base": [] }] },
      { "id": "loop-computed-array", "source": "fn check() { let root = std::path::Path::Path::new(env!(\"CARGO_MANIFEST_DIR\")); for path in paths() { root.join(path); } }", "expected": [] },
      { "id": "comment", "source": "fn check() {} /* let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")); root.join(\"payload.json\"); */", "expected": [] },
      { "id": "macro-payload", "source": "fn check() { quote! { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")); root.join(\"payload.json\"); } }", "expected": [] },
      { "id": "escaped", "source": "fn check() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")); root.join(\"payload\\u{2e}json\"); }", "expected": [] },
      { "id": "cross-function", "source": "fn first() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")); } fn second() { root.join(\"payload.json\"); }", "expected": [] },
      {"id":"pathbuf-direct","source":"fn check() -> std::path::PathBuf { let root = std::path::PathBuf::from(env!(\"CARGO_MANIFEST_DIR\")).join(\"fixtures\"); root.join(\"payload.json\") }","expected":[{"value":"fixtures","base":[]},{"value":"payload.json","base":["fixtures"]}],"compiler":"fn main() { let result: std::path::PathBuf = check(); assert_eq!(result, std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"fixtures/payload.json\")); println!(\"pathbuf-direct\"); }","compilerOutput":"pathbuf-direct"},
      {"id":"pathbuf-absolute-qualified","source":"fn check() -> std::path::PathBuf { ::std::path::PathBuf::from(env!(\"CARGO_MANIFEST_DIR\")).join(\"payload.json\") }","expected":[{"value":"payload.json","base":[]}],"compiler":"fn main() { let result: std::path::PathBuf = check(); assert_eq!(result, std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"payload.json\")); println!(\"pathbuf-absolute-qualified\"); }","compilerOutput":"pathbuf-absolute-qualified"},
      {"id":"pathbuf-parenthesized","source":"fn check() -> std::path::PathBuf { (std::path::PathBuf::from(env!(\"CARGO_MANIFEST_DIR\"))).join(\"nested\").join(\"payload.json\") }","expected":[{"value":"nested","base":[]},{"value":"payload.json","base":["nested"]}],"compiler":"fn main() { let result: std::path::PathBuf = check(); assert_eq!(result, std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"nested/payload.json\")); println!(\"pathbuf-parenthesized\"); }","compilerOutput":"pathbuf-parenthesized"},
      {"id":"pathbuf-single-use-loop","source":"fn check() { let root = std::path::PathBuf::from(env!(\"CARGO_MANIFEST_DIR\")); for path in [\"one.json\", \"two.json\"] { root.join(path); } }","expected":[{"value":"one.json","base":[]},{"value":"two.json","base":[]}]},
      {"id":"pathbuf-shadowed-binding","source":"fn check() { let root = std::path::PathBuf::from(env!(\"CARGO_MANIFEST_DIR\")); let root = other(); root.join(\"payload.json\"); }","expected":[]},
      {"id":"pathbuf-destructured-shadow","source":"fn check() { let root = std::path::PathBuf::from(env!(\"CARGO_MANIFEST_DIR\")); let (root, _) = other(); root.join(\"payload.json\"); }","expected":[]},
      {"id":"pathbuf-closure-shadow","source":"fn check() { let root = std::path::PathBuf::from(env!(\"CARGO_MANIFEST_DIR\")); let reader = |root| root.join(\"payload.json\"); }","expected":[]},
      {"id":"pathbuf-mutable-binding","source":"fn check() { let mut root = std::path::PathBuf::from(env!(\"CARGO_MANIFEST_DIR\")); root.push(\"other\"); root.join(\"payload.json\"); }","expected":[]},
      {"id":"pathbuf-unknown-constructor","source":"fn check() { let root = PathBuf::from(env!(\"CARGO_MANIFEST_DIR\")); root.join(\"payload.json\"); }","expected":[]},
      {"id":"pathbuf-wrong-constructor","source":"fn check() { let root = std::path::PathBuf::new(env!(\"CARGO_MANIFEST_DIR\")); root.join(\"payload.json\"); }","expected":[]},
      {"id":"pathbuf-wrong-environment","source":"fn check() { let root = std::path::PathBuf::from(env!(\"OTHER_ROOT\")); root.join(\"payload.json\"); }","expected":[]},
      {"id":"pathbuf-optional-environment","source":"fn check() { let root = std::path::PathBuf::from(option_env!(\"CARGO_MANIFEST_DIR\")); root.join(\"payload.json\"); }","expected":[]},
      {"id":"pathbuf-shadowed-std","source":"mod std {} fn check() { let root = std::path::PathBuf::from(env!(\"CARGO_MANIFEST_DIR\")); root.join(\"payload.json\"); }","expected":[]},
      {"id":"pathbuf-loop-explicit-label","source":"fn check() { let root = std::path::PathBuf::from(env!(\"CARGO_MANIFEST_DIR\")); for path in [\"payload.json\"] { println!(\"{}\", path); root.join(path); } }","expected":[]},
      {"id":"pathbuf-loop-two-bases","source":"fn check() { let root = std::path::PathBuf::from(env!(\"CARGO_MANIFEST_DIR\")); let other = root.join(\"nested\"); for path in [\"payload.json\"] { root.join(path); other.join(path); } }","expected":[{"value":"nested","base":[]}]},
      {"id":"path-loop-implicit-label","source":"fn check() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")); for path in [\"payload.json\"] { root.join(path); println!(\"{path}\"); } }","expected":[]},
      {"id":"pathbuf-loop-implicit-label","source":"fn check() -> std::path::PathBuf { let root = std::path::PathBuf::from(env!(\"CARGO_MANIFEST_DIR\")); for path in [\"payload.json\"] { let joined = root.join(path); println!(\"{path}\"); return joined; } unreachable!() }","expected":[],"compiler":"fn main() { let result: std::path::PathBuf = check(); assert_eq!(result, std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"payload.json\")); println!(\"pathbuf-loop-implicit-label\"); }","compilerOutput":"payload.json\npathbuf-loop-implicit-label"},
      {"id":"pathbuf-loop-escaped-label","source":"fn check() -> std::path::PathBuf { let root = std::path::PathBuf::from(env!(\"CARGO_MANIFEST_DIR\")); for path in [\"payload.json\"] { let joined = root.join(path); println!(\"{{path}}\"); return joined; } unreachable!() }","expected":[{"value":"payload.json","base":[]}],"compiler":"fn main() { let result: std::path::PathBuf = check(); assert_eq!(result, std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"payload.json\")); println!(\"pathbuf-loop-escaped-label\"); }","compilerOutput":"{path}\npathbuf-loop-escaped-label"},
      {"id":"path-loop-escaped-label","source":"fn check() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")); for path in [\"payload.json\"] { root.join(path); println!(\"{{path}}\"); } }","expected":[{"value":"payload.json","base":[]}]},
      {"id":"pathbuf-loop-triple-brace-label","source":"fn check() { let root = std::path::PathBuf::from(env!(\"CARGO_MANIFEST_DIR\")); for path in [\"payload.json\"] { root.join(path); println!(\"{{{path}}}\"); } }","expected":[]},
      {"id":"pathbuf-loop-assert-label","source":"fn check() { let root = std::path::PathBuf::from(env!(\"CARGO_MANIFEST_DIR\")); for path in [\"payload.json\"] { root.join(path); assert!(true, \"{path:?}\"); } }","expected":[]},
      {"id":"pathbuf-loop-panic-label","source":"fn check() { let root = std::path::PathBuf::from(env!(\"CARGO_MANIFEST_DIR\")); for path in [\"payload.json\"] { root.join(path); panic!(\"{path}\"); } }","expected":[]},
      {"id":"pathbuf-loop-unknown-macro-label","source":"fn check() { let root = std::path::PathBuf::from(env!(\"CARGO_MANIFEST_DIR\")); for path in [\"payload.json\"] { root.join(path); opaque!(\"path\"); } }","expected":[]},
      {"id":"pathbuf-loop-imported-macro-is-uncertain","source":"use foreign::println; fn check() { let root = std::path::PathBuf::from(env!(\"CARGO_MANIFEST_DIR\")); for path in [\"payload.json\"] { root.join(path); println!(\"{{path}}\"); } }","expected":[]},
      {"id":"pathbuf-loop-qualified-escaped-format-with-import","source":"use std::path::Path; fn check() { let root = std::path::PathBuf::from(env!(\"CARGO_MANIFEST_DIR\")); for path in [\"payload.json\"] { root.join(path); std::println!(\"{{path}}\"); } }","expected":[{"value":"payload.json","base":[]}]},
      {"id":"pathbuf-loop-known-noncapturing-label","source":"fn check() { let root = std::path::PathBuf::from(env!(\"CARGO_MANIFEST_DIR\")); for path in [\"payload.json\"] { root.join(path); println!(\"path\"); } }","expected":[{"value":"payload.json","base":[]}]}
    ],
    "moduleGraphCases": [
      { "id": "mounted", "target": "artifacts/code.rs", "files": { "pkg/Cargo.toml": "[package]\nname = \"example\"\n[lib]\npath = \"glue.rs\"\n", "pkg/glue.rs": "#[path = \".\"] mod artifact { #[path = \"../artifacts/code.rs\"] mod component; }", "artifacts/code.rs": "pub fn inspect() {}" }, "expectedManifests": ["pkg/Cargo.toml"] },
      { "id": "unmounted-lookalike", "target": "artifacts/code.rs", "files": { "pkg/Cargo.toml": "[package]\nname = \"example\"\n[lib]\npath = \"glue.rs\"\n", "pkg/glue.rs": "const TEXT: &str = r#\"#[path = \"../artifacts/code.rs\"] mod component;\"#;", "artifacts/code.rs": "pub fn inspect() {}" }, "expectedManifests": [] },
      { "id": "ambiguous", "target": "artifacts/code.rs", "files": { "one/Cargo.toml": "[package]\nname = \"one\"\n[lib]\npath = \"glue.rs\"\n", "one/glue.rs": "#[path = \"../artifacts/code.rs\"] mod component;", "two/Cargo.toml": "[package]\nname = \"two\"\n[lib]\npath = \"glue.rs\"\n", "two/glue.rs": "#[path = \"../artifacts/code.rs\"] mod component;", "artifacts/code.rs": "pub fn inspect() {}" }, "expectedManifests": ["one/Cargo.toml", "two/Cargo.toml"] },
      { "id": "conventional-not-manifest", "target": "src/lib.rs", "files": { "src/lib.rs": "pub fn inspect() {}" }, "expectedManifests": [] },
      { "id": "absolute-mount-is-not-relative", "target": "pkg/absolute/code.rs", "files": { "pkg/Cargo.toml": "[package]\nname = \"example\"\n[lib]\npath = \"glue.rs\"\n", "pkg/glue.rs": "#[path = \"/absolute/code.rs\"] mod component;", "pkg/absolute/code.rs": "pub fn inspect() {}" }, "expectedManifests": [] },
      { "id": "malformed-manifest", "target": "pkg/glue.rs", "files": { "pkg/Cargo.toml": "[package]\nname = \"one\"\nname = \"two\"\n[lib]\npath = \"glue.rs\"\n", "pkg/glue.rs": "pub fn inspect() {}" }, "expectedManifests": [] }
    ]
  },
  "assertionMessages": {
    "macros": { "assert": 1, "assert_eq": 2, "assert_ne": 2, "debug_assert": 1, "debug_assert_eq": 2, "debug_assert_ne": 2 },
    "literal": "unescaped-normal-string-only",
    "context": "exact-top-level-macro-argument",
    "cases": [
      { "id": "equal", "source": "fn check() { assert_eq!(left, right, \"committed fixtures/payload.json\"); }", "expected": [{ "macroName": "assert_eq", "value": "committed fixtures/payload.json" }] },
      { "id": "nested", "source": "fn check() { assert!(f(1, 2), \"missing fixtures/payload.json\"); }", "expected": [{ "macroName": "assert", "value": "missing fixtures/payload.json" }] },
      { "id": "qualified", "source": "fn check() { std::debug_assert_ne!(left, right, \"wrong fixtures/payload.json\",); }", "expected": [{ "macroName": "debug_assert_ne", "value": "wrong fixtures/payload.json" }] },
      { "id": "format-argument", "source": "fn check() { assert_eq!(left, right, \"{} fixtures/payload.json\", actual); }", "expected": [{ "macroName": "assert_eq", "value": "{} fixtures/payload.json" }] },
      { "id": "comment", "source": "fn check() {} // assert_eq!(left, right, \"fixtures/payload.json\");", "expected": [] },
      { "id": "nested-comment", "source": "/* outer /* assert!(ready, \"fixtures/payload.json\"); */ end */ fn check() {}", "expected": [] },
      { "id": "string-lookalike", "source": "const TEXT: &str = r#\"assert_eq!(left, right, \"fixtures/payload.json\");\"#;", "expected": [] },
      { "id": "other-macro", "source": "fn check() { identity!(left, right, \"fixtures/payload.json\"); }", "expected": [] },
      { "id": "nonliteral", "source": "fn check() { assert_eq!(left, right, concat!(\"fixtures/\", \"payload.json\")); }", "expected": [] },
      { "id": "escaped", "source": "fn check() { assert_eq!(left, right, \"fixtures\\u{2f}payload.json\"); }", "expected": [] },
      { "id": "raw", "source": "fn check() { assert_eq!(left, right, r#\"fixtures/payload.json\"#); }", "expected": [] }
    ]
  }
}

````

## 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🥤️rust-finite-target-consumption/🔣️.json

Bytes 20478; SHA-256 `75a3d2469ebcbac450c2923caf479392b4d83812553d98ac170a804573533885`.

````text
{
  "schemaVersion": 1,
  "contract": "rust-finite-target-consumption-v1",
  "semantics": {
    "authority": "candidate-only-never-editable",
    "required": [
      "complete-finite-expansion",
      "exact-utf16-source-span",
      "unique-cargo-owner",
      "exact-hashed-source-chain",
      "physically-present-admitted-targets",
      "no-follow",
      "non-opaque",
      "coordinate-root-local",
      "unshadowed-inherited-environment"
    ],
    "failure": "retain-conservative-unsupported-interpretation",
    "suppression": "same-start-end-value-only",
    "writablePrecedence": "existing-immutable-join-authority",
    "bound": 256
  },
  "cases": [
    {
      "id": "foreign-disjoint",
      "source": "pub fn read() {\n    let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\"));\n    for directory in [root.join(\"../foreign\")] {\n        for leaf in [\"item.json\"] {\n            let _ = directory.join(leaf);\n            println!(\"{leaf}\");\n        }\n    }\n}\n",
      "targets": [
        "foreign/item.json"
      ],
      "expected": "finite",
      "affected": [
        "pkg/item.json"
      ],
      "condition": "regular"
    },
    {
      "id": "mixed-intersection",
      "source": "pub fn read() {\n    let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\"));\n    for directory in [root.join(\"../foreign\"), root.join(\"../affected\")] {\n        for leaf in [\"item.json\"] {\n            let _ = directory.join(leaf);\n            println!(\"{leaf}\");\n        }\n    }\n}\n",
      "targets": [
        "affected/item.json",
        "foreign/item.json"
      ],
      "expected": "finite",
      "affected": [
        "affected/item.json",
        "pkg/item.json"
      ],
      "condition": "regular"
    },
    {
      "id": "missing-target",
      "source": "pub fn read() {\n    let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\"));\n    for directory in [root.join(\"../foreign\"), root.join(\"../absent\")] {\n        for leaf in [\"item.json\"] {\n            let _ = directory.join(leaf);\n            println!(\"{leaf}\");\n        }\n    }\n}\n",
      "targets": [
        "foreign/item.json"
      ],
      "expected": "fallback",
      "affected": [
        "pkg/item.json"
      ],
      "condition": "regular"
    },
    {
      "id": "unadmitted-target",
      "source": "pub fn read() {\n    let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\"));\n    for directory in [root.join(\"../foreign\")] {\n        for leaf in [\"item.json\"] {\n            let _ = directory.join(leaf);\n            println!(\"{leaf}\");\n        }\n    }\n}\n",
      "targets": [
        "foreign/item.json"
      ],
      "expected": "fallback",
      "affected": [
        "pkg/item.json"
      ],
      "condition": "unadmitted"
    },
    {
      "id": "two-cargo-owners",
      "source": "pub fn read() {\n    let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\"));\n    for directory in [root.join(\"../foreign\")] {\n        for leaf in [\"item.json\"] {\n            let _ = directory.join(leaf);\n            println!(\"{leaf}\");\n        }\n    }\n}\n",
      "targets": [
        "foreign/item.json"
      ],
      "expected": "fallback",
      "affected": [
        "pkg/item.json"
      ],
      "condition": "two-owners"
    },
    {
      "id": "no-cargo-owner",
      "source": "pub fn read() {\n    let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\"));\n    for directory in [root.join(\"../foreign\")] {\n        for leaf in [\"item.json\"] {\n            let _ = directory.join(leaf);\n            println!(\"{leaf}\");\n        }\n    }\n}\n",
      "targets": [
        "foreign/item.json"
      ],
      "expected": "fallback",
      "affected": [
        "pkg/item.json"
      ],
      "condition": "no-owner"
    },
    {
      "id": "symlink-target-leaf",
      "source": "pub fn read() {\n    let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\"));\n    for directory in [root.join(\"../foreign\")] {\n        for leaf in [\"item.json\"] {\n            let _ = directory.join(leaf);\n            println!(\"{leaf}\");\n        }\n    }\n}\n",
      "targets": [
        "foreign/item.json"
      ],
      "expected": "fallback",
      "affected": [
        "pkg/item.json"
      ],
      "condition": "symlink-leaf"
    },
    {
      "id": "symlink-target-ancestor",
      "source": "pub fn read() {\n    let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\"));\n    for directory in [root.join(\"../foreign\")] {\n        for leaf in [\"item.json\"] {\n            let _ = directory.join(leaf);\n            println!(\"{leaf}\");\n        }\n    }\n}\n",
      "targets": [
        "foreign/item.json"
      ],
      "expected": "fallback",
      "affected": [
        "pkg/item.json"
      ],
      "condition": "symlink-ancestor"
    },
    {
      "id": "opaque-root-no-access",
      "source": "pub fn read() {\n    let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\"));\n    for directory in [root.join(\"../compose\")] {\n        for leaf in [\"item.json\"] {\n            let _ = directory.join(leaf);\n            println!(\"{leaf}\");\n        }\n    }\n}\n",
      "targets": [],
      "expected": "fallback",
      "affected": [
        "pkg/item.json"
      ],
      "condition": "opaque"
    },
    {
      "id": "opaque-nested-no-access",
      "source": "pub fn read() {\n    let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\"));\n    for directory in [root.join(\"../temp/compose\")] {\n        for leaf in [\"item.json\"] {\n            let _ = directory.join(leaf);\n            println!(\"{leaf}\");\n        }\n    }\n}\n",
      "targets": [],
      "expected": "fallback",
      "affected": [
        "pkg/item.json"
      ],
      "condition": "opaque"
    },
    {
      "id": "literal-temp-compose-not-opaque",
      "source": "pub fn read() {\n    let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\"));\n    for directory in [root.join(\"../temp-compose\")] {\n        for leaf in [\"item.json\"] {\n            let _ = directory.join(leaf);\n            println!(\"{leaf}\");\n        }\n    }\n}\n",
      "targets": [
        "temp-compose/item.json"
      ],
      "expected": "finite",
      "affected": [
        "pkg/item.json"
      ],
      "condition": "regular"
    },
    {
      "id": "coordinate-owner-escape",
      "source": "pub fn read() {\n    let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\"));\n    for directory in [root.join(\"../../foreign\")] {\n        for leaf in [\"item.json\"] {\n            let _ = directory.join(leaf);\n            println!(\"{leaf}\");\n        }\n    }\n}\n",
      "targets": [
        "foreign/item.json"
      ],
      "expected": "fallback",
      "affected": [
        "nested/pkg/item.json"
      ],
      "condition": "nested-coordinate"
    },
    {
      "id": "nested-foreign-owner-entry",
      "source": "pub fn read() {\n    let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\"));\n    for directory in [root.join(\"../foreign\")] {\n        for leaf in [\"item.json\"] {\n            let _ = directory.join(leaf);\n            println!(\"{leaf}\");\n        }\n    }\n}\n",
      "targets": [
        "foreign/item.json"
      ],
      "expected": "fallback",
      "affected": [
        "pkg/item.json"
      ],
      "condition": "foreign-coordinate"
    },
    {
      "id": "repository-escape",
      "source": "pub fn read() {\n    let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\"));\n    for directory in [root.join(\"../../outside\")] {\n        for leaf in [\"item.json\"] {\n            let _ = directory.join(leaf);\n            println!(\"{leaf}\");\n        }\n    }\n}\n",
      "targets": [],
      "expected": "fallback",
      "affected": [
        "pkg/item.json"
      ],
      "condition": "regular"
    },
    {
      "id": "changed-consumer",
      "source": "pub fn read() {\n    let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\"));\n    for directory in [root.join(\"../foreign\")] {\n        for leaf in [\"item.json\"] {\n            let _ = directory.join(leaf);\n            println!(\"{leaf}\");\n        }\n    }\n}\n",
      "targets": [
        "foreign/item.json"
      ],
      "expected": "fallback",
      "affected": [
        "pkg/item.json"
      ],
      "condition": "changed-consumer"
    },
    {
      "id": "changed-source-chain",
      "source": "pub fn read() {\n    let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\"));\n    for directory in [root.join(\"../foreign\")] {\n        for leaf in [\"item.json\"] {\n            let _ = directory.join(leaf);\n            println!(\"{leaf}\");\n        }\n    }\n}\n",
      "targets": [
        "foreign/item.json"
      ],
      "expected": "fallback",
      "affected": [
        "pkg/item.json"
      ],
      "condition": "changed-chain"
    },
    {
      "id": "changed-manifest",
      "source": "pub fn read() {\n    let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\"));\n    for directory in [root.join(\"../foreign\")] {\n        for leaf in [\"item.json\"] {\n            let _ = directory.join(leaf);\n            println!(\"{leaf}\");\n        }\n    }\n}\n",
      "targets": [
        "foreign/item.json"
      ],
      "expected": "fallback",
      "affected": [
        "pkg/item.json"
      ],
      "condition": "changed-manifest"
    },
    {
      "id": "missing-source-chain",
      "source": "pub fn read() {\n    let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\"));\n    for directory in [root.join(\"../foreign\")] {\n        for leaf in [\"item.json\"] {\n            let _ = directory.join(leaf);\n            println!(\"{leaf}\");\n        }\n    }\n}\n",
      "targets": [
        "foreign/item.json"
      ],
      "expected": "fallback",
      "affected": [
        "pkg/item.json"
      ],
      "condition": "missing-chain"
    },
    {
      "id": "dynamic-receiver",
      "source": "pub fn read() {\n    let root = std::env::current_dir().unwrap();\n    for directory in [root.join(\"../foreign\")] {\n        for leaf in [\"item.json\"] {\n            let _ = directory.join(leaf);\n            println!(\"{leaf}\");\n        }\n    }\n}\n",
      "targets": [
        "foreign/item.json"
      ],
      "expected": "fallback",
      "affected": [
        "pkg/item.json"
      ],
      "condition": "regular"
    },
    {
      "id": "mutated-receiver",
      "source": "pub fn read() {\n    let mut root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\"));\n    root = std::path::Path::new(\".\");\n    for directory in [root.join(\"../foreign\")] {\n        for leaf in [\"item.json\"] {\n            let _ = directory.join(leaf);\n            println!(\"{leaf}\");\n        }\n    }\n}\n",
      "targets": [
        "foreign/item.json"
      ],
      "expected": "fallback",
      "affected": [
        "pkg/item.json"
      ],
      "condition": "regular"
    },
    {
      "id": "cancelled-symlink-ancestor",
      "source": "pub fn read() {\n    let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\"));\n    for directory in [root.join(\"../foreign\")] {\n        for leaf in [\"item.json\"] {\n            let _ = directory.join(leaf);\n            println!(\"{leaf}\");\n        }\n    }\n}\n",
      "targets": [
        "foreign/item.json"
      ],
      "expected": "fallback",
      "affected": [
        "pkg/item.json"
      ],
      "condition": "cancelled-symlink"
    },
    {
      "id": "cancelled-missing-directory",
      "source": "pub fn read() {\n    let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\"));\n    for directory in [root.join(\"../foreign\")] {\n        for leaf in [\"item.json\"] {\n            let _ = directory.join(leaf);\n            println!(\"{leaf}\");\n        }\n    }\n}\n",
      "targets": [
        "foreign/item.json"
      ],
      "expected": "fallback",
      "affected": [
        "pkg/item.json"
      ],
      "condition": "regular"
    },
    {
      "id": "cancelled-opaque-directory",
      "source": "pub fn read() {\n    let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\"));\n    for directory in [root.join(\"../foreign\")] {\n        for leaf in [\"item.json\"] {\n            let _ = directory.join(leaf);\n            println!(\"{leaf}\");\n        }\n    }\n}\n",
      "targets": [
        "foreign/item.json"
      ],
      "expected": "fallback",
      "affected": [
        "pkg/item.json"
      ],
      "condition": "regular"
    },
    {
      "id": "cancelled-nondirectory-parent",
      "source": "pub fn read() {\n    let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\"));\n    for directory in [root.join(\"../foreign\")] {\n        for leaf in [\"item.json\"] {\n            let _ = directory.join(leaf);\n            println!(\"{leaf}\");\n        }\n    }\n}\n",
      "targets": [
        "foreign/item.json"
      ],
      "expected": "fallback",
      "affected": [
        "pkg/item.json"
      ],
      "condition": "cancelled-file"
    },
    {
      "id": "coordinate-root-excursion",
      "source": "pub fn read() {\n    let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\"));\n    for directory in [root.join(\"../../nested/foreign\")] {\n        for leaf in [\"item.json\"] {\n            let _ = directory.join(leaf);\n            println!(\"{leaf}\");\n        }\n    }\n}\n",
      "targets": [
        "nested/foreign/item.json"
      ],
      "expected": "fallback",
      "affected": [
        "nested/pkg/item.json"
      ],
      "condition": "nested-coordinate"
    },
    {
      "id": "cancelled-module-ownership-edge",
      "source": "pub fn read() {\n    let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\"));\n    for directory in [root.join(\"../foreign\")] {\n        for leaf in [\"item.json\"] {\n            let _ = directory.join(leaf);\n            println!(\"{leaf}\");\n        }\n    }\n}\n",
      "targets": [
        "foreign/item.json"
      ],
      "expected": "fallback",
      "affected": [
        "pkg/item.json"
      ],
      "condition": "cancelled-module-edge"
    },
    {
      "id": "cancelled-manifest-ownership-edge",
      "source": "pub fn read() {\n    let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\"));\n    for directory in [root.join(\"../foreign\")] {\n        for leaf in [\"item.json\"] {\n            let _ = directory.join(leaf);\n            println!(\"{leaf}\");\n        }\n    }\n}\n",
      "targets": [
        "foreign/item.json"
      ],
      "expected": "fallback",
      "affected": [
        "pkg/item.json"
      ],
      "condition": "cancelled-manifest-edge"
    },
    {
      "id": "inherited-env-macro",
      "source": "pub fn read() {\n    let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\"));\n    for directory in [root.join(\"../foreign\")] {\n        for leaf in [\"item.json\"] {\n            println!(\"TARGET:{}\", std::fs::read_to_string(directory.join(leaf)).unwrap().trim());\n            println!(\"{leaf}\");\n        }\n    }\n}\n",
      "targets": [
        "foreign/item.json"
      ],
      "expected": "fallback",
      "affected": [
        "pkg/item.json"
      ],
      "condition": "parent-env-macro"
    },
    {
      "id": "ancestor-doc-comment-noise",
      "source": "pub fn read() {\n    let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\"));\n    for directory in [root.join(\"../foreign\")] {\n        for leaf in [\"item.json\"] {\n            let _ = directory.join(leaf);\n            println!(\"{leaf}\");\n        }\n    }\n}\n",
      "targets": [
        "foreign/item.json"
      ],
      "expected": "finite",
      "affected": [
        "pkg/item.json"
      ],
      "condition": "parent-doc-comment"
    },
    {
      "id": "ancestor-known-transparent-macro",
      "source": "pub fn read() {\n    let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\"));\n    for directory in [root.join(\"../foreign\")] {\n        for leaf in [\"item.json\"] {\n            let _ = directory.join(leaf);\n            println!(\"{leaf}\");\n        }\n    }\n}\n",
      "targets": [
        "foreign/item.json"
      ],
      "expected": "finite",
      "affected": [
        "pkg/item.json"
      ],
      "condition": "parent-known-macro"
    },
    {
      "id": "ancestor-cfg-gated-test-mod",
      "source": "pub fn read() {\n    let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\"));\n    for directory in [root.join(\"../foreign\")] {\n        for leaf in [\"item.json\"] {\n            let _ = directory.join(leaf);\n            println!(\"{leaf}\");\n        }\n    }\n}\n",
      "targets": [
        "foreign/item.json"
      ],
      "expected": "finite",
      "affected": [
        "pkg/item.json"
      ],
      "condition": "parent-cfg-test-mod"
    },
    {
      "id": "ancestor-glob-reexport-with-super-star-target",
      "source": "mod structural_correspondence_tests {\n    use super::*;\n    pub fn read() {\n        let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\"));\n        for directory in [root.join(\"../foreign\")] {\n            for leaf in [\"item.json\"] {\n                let _ = directory.join(leaf);\n                println!(\"{leaf}\");\n            }\n        }\n    }\n}\n",
      "targets": [
        "foreign/item.json"
      ],
      "expected": "finite",
      "affected": [
        "pkg/item.json"
      ],
      "condition": "parent-glob-reexport"
    },
    {
      "id": "ancestor-known-attribute-path",
      "source": "pub fn read() {\n    let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\"));\n    for directory in [root.join(\"../foreign\")] {\n        for leaf in [\"item.json\"] {\n            let _ = directory.join(leaf);\n            println!(\"{leaf}\");\n        }\n    }\n}\n",
      "targets": [
        "foreign/item.json"
      ],
      "expected": "finite",
      "affected": [
        "pkg/item.json"
      ],
      "condition": "parent-known-attribute-path"
    },
    {
      "id": "ancestor-crate-local-macro-and-std-expression-macros",
      "source": "pub fn read() {\n    let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\"));\n    for directory in [root.join(\"../foreign\")] {\n        for leaf in [\"item.json\"] {\n            let _ = directory.join(leaf);\n            println!(\"{leaf}\");\n        }\n    }\n}\n",
      "targets": [
        "foreign/item.json"
      ],
      "expected": "finite",
      "affected": [
        "pkg/item.json"
      ],
      "condition": "parent-crate-local-macro-and-std-expression-macros"
    }
  ],
  "correlated": {
    "source": "pub fn read() {\n    let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\"));\n    for (directory, leaf) in [(root.join(\"../foreign\"), \"alpha.json\"), (root.join(\"../affected\"), \"beta.json\")] {\n        let _ = directory.join(leaf);\n        println!(\"{leaf}\");\n    }\n}\n",
    "targets": [
      {
        "value": "alpha.json",
        "target": "foreign/alpha.json"
      },
      {
        "value": "beta.json",
        "target": "affected/beta.json"
      }
    ]
  },
  "writable": {
    "source": "pub fn read() { let root = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../foreign\"); let _ = root.join(\"item.json\"); }\n",
    "target": "foreign/item.json"
  },
  "registration": {
    "projectPath": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json",
    "routerPath": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
    "target": "test-rust-finite-target-consumption",
    "command": "bun ./📜️script.ts test rust-finite-target-consumption",
    "route": "rust-finite-target-consumption",
    "testPath": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥤️rust-finite-target-consumption/🟦️.ts",
    "launchName": "🧹clean🧩️taxonomy🥤️rust-finite-target-consumption",
    "launchOrder": 410.193,
    "launchCommand": "bun nx run @semio-tech/repo-lib:test-rust-finite-target-consumption --skip-nx-cache"
  },
  "retention": {
    "parentSegments": [
      "📓️energy-rust-reference-diagnostics",
      "🧭️finite-target-consumption",
      "🧾️runs"
    ],
    "runPrefix": "🔖️"
  }
}

````

## Root Pre-Repair Captures

### current-repo-repair-before.json

Captured at `2026-10-02T20:37:55.234Z`.

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts`: verified captured SHA-256 `4aa09dbf067d976284df77f9e073f11eb4d405698132597823fdad1a73f60292`; declared `4aa09dbf067d976284df77f9e073f11eb4d405698132597823fdad1a73f60292`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts`: verified captured SHA-256 `363271c9dfd4db2338b0b6254b6590ace4ab74c21c17ed55dd24f91dbf6ec69f`; declared `363271c9dfd4db2338b0b6254b6590ace4ab74c21c17ed55dd24f91dbf6ec69f`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥤️rust-finite-target-consumption/🟦️.ts`: verified captured SHA-256 `4ede4cfac606af18a857a893eaa4f6fd4112a8178314430e27a78c38348aa909`; declared `4ede4cfac606af18a857a893eaa4f6fd4112a8178314430e27a78c38348aa909`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🥤️rust-finite-target-consumption/🔣️.json`: verified captured SHA-256 `75a3d2469ebcbac450c2923caf479392b4d83812553d98ac170a804573533885`; declared `75a3d2469ebcbac450c2923caf479392b4d83812553d98ac170a804573533885`.
- `.vscode/🧩️launch.seed.jsonc`: verified captured SHA-256 `4e1405e1bb95df5e87956bc742f6dc646fc66c3c7bfe058157b1e471fff2962b`; declared `4e1405e1bb95df5e87956bc742f6dc646fc66c3c7bfe058157b1e471fff2962b`.
- `.vscode/launch.json`: verified captured SHA-256 `0b20c130a2180d5f5caeebc8920fbbbed273a8a654d1b58010f4493e96d91459`; declared `0b20c130a2180d5f5caeebc8920fbbbed273a8a654d1b58010f4493e96d91459`.
### physical-finite-fixture-repair-before.json

Captured at `2026-10-02T20:40:08.861Z`.

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧲️rust-physical-reference-context/🔣️.json`: verified captured SHA-256 `f635a592f46a255efc8d9184e0acc0099cd61ec94ba2500f429e551e7a0a296d`; declared `f635a592f46a255efc8d9184e0acc0099cd61ec94ba2500f429e551e7a0a296d`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥤️rust-finite-target-consumption/🟦️.ts`: verified captured SHA-256 `4ede4cfac606af18a857a893eaa4f6fd4112a8178314430e27a78c38348aa909`; declared `4ede4cfac606af18a857a893eaa4f6fd4112a8178314430e27a78c38348aa909`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🥤️rust-finite-target-consumption/🔣️.json`: verified captured SHA-256 `75a3d2469ebcbac450c2923caf479392b4d83812553d98ac170a804573533885`; declared `75a3d2469ebcbac450c2923caf479392b4d83812553d98ac170a804573533885`.
- `.vscode/🧩️launch.seed.jsonc`: verified captured SHA-256 `4e1405e1bb95df5e87956bc742f6dc646fc66c3c7bfe058157b1e471fff2962b`; declared `4e1405e1bb95df5e87956bc742f6dc646fc66c3c7bfe058157b1e471fff2962b`.
