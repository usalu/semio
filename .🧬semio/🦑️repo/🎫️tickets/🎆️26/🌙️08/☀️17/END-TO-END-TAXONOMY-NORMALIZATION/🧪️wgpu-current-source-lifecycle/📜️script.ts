import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { lstatSync, readFileSync } from "node:fs";
import { dirname, isAbsolute, join, posix, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { parse as parseJsonc } from "jsonc-parser";
import ts from "typescript";

const acceptanceRoot = dirname(fileURLToPath(import.meta.url)), ticketRoot = dirname(acceptanceRoot), repoRoot = resolve(ticketRoot, "../../../../../../../");
const runDirectory = "🧾️runs", repositoryDirectory = "🧪️workspace";
const hash = (bytes: string | Uint8Array) => createHash("sha256").update(bytes).digest("hex");
const order = (left: string, right: string) => Buffer.compare(Buffer.from(left), Buffer.from(right));
const scenarioText = readFileSync(join(acceptanceRoot, "🔣️.json"), "utf8"), scenario = JSON.parse(scenarioText);
assert.equal(scenario.authority.taxonomyPath, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json");
const taxonomyText = readFileSync(join(repoRoot, scenario.authority.taxonomyPath), "utf8"), taxonomy = JSON.parse(taxonomyText);
const exclusions = Object.values(taxonomy.pathExclusions).map((value: any) => value.path.replace(/\/$/u, ""));

function readInput(path: string): Buffer {
  if (!path || isAbsolute(path) || path.includes("\\") || posix.normalize(path) !== path || path.split("/").includes("..") || exclusions.some((root) => path === root || path.startsWith(root + "/"))) throw new Error("Input is outside admitted lexical authority: " + path);
  let absolute = repoRoot;
  const parts = path.split("/");
  for (let index = 0; index < parts.length; index++) {
    absolute = join(absolute, parts[index]!);
    const stat = lstatSync(absolute);
    if (stat.isSymbolicLink() || (index === parts.length - 1 ? !stat.isFile() : !stat.isDirectory())) throw new Error("Input is not a no-follow regular leaf: " + path);
  }
  return readFileSync(absolute);
}

function existingKind(path: string): "directory" | "file" | "symlink" | null {
  try { const stat = lstatSync(path); return stat.isSymbolicLink() ? "symlink" : stat.isDirectory() ? "directory" : "file"; }
  catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") return null; throw error; }
}

function assertFixtureRoot(path: string, runId: string, kind = existingKind): string {
  assert.match(runId, /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/u);
  const expected = join(acceptanceRoot, runDirectory, "🔖️" + runId, repositoryDirectory);
  assert.ok(isAbsolute(path) && resolve(path) === path && path === expected);
  let current = repoRoot;
  assert.equal(kind(current), "directory");
  for (const part of relative(repoRoot, acceptanceRoot).replaceAll("\\", "/").split("/")) { current = join(current, part); assert.equal(kind(current), "directory", current); }
  for (const part of [runDirectory, "🔖️" + runId, repositoryDirectory]) {
    current = join(current, part);
    const value = kind(current);
    if (value === null) break;
    assert.equal(value, "directory", current);
  }
  return expected;
}

function completenessGuard(source: string): { start: number; end: number; hash: string } {
  const parsed = ts.createSourceFile("normalizer.ts", source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS), guards: { start: number; end: number; hash: string }[] = [];
  const visit = (node: ts.Node): void => {
    if (ts.isIfStatement(node) && source.slice(node.getStart(parsed), node.end) === scenario.executionBoundary.source) {
      let parent: ts.Node | undefined = node.parent;
      while (parent && !ts.isFunctionDeclaration(parent)) parent = parent.parent;
      assert.ok(parent && ts.isFunctionDeclaration(parent) && parent.name?.text === scenario.executionBoundary.functionName);
      guards.push({ start: node.getStart(parsed), end: node.end, hash: hash(scenario.executionBoundary.source) });
    }
    ts.forEachChild(node, visit);
  };
  visit(parsed);
  assert.equal(guards.length, 1);
  return guards[0]!;
}

function inspect(): void {
  assert.deepEqual(parseJsonc(scenarioText), scenario);
  assert.deepEqual(parseJsonc(taxonomyText), taxonomy);
  assert.equal(hash(taxonomyText), scenario.authority.taxonomyHash);
  assert.equal(scenario.provenance, "new-current-source-run");
  assert.equal(scenario.fixture.sourceFallback, false);
  assert.equal(scenario.fixture.restoreHistoricalRun, false);
  assert.equal(scenario.fixture.runDirectory, runDirectory);
  assert.equal(scenario.fixture.repositoryDirectory, repositoryDirectory);
  assert.equal(scenario.executionBoundary.productionGuardRequired, true);
  assert.equal(scenario.executionBoundary.approval, "preparation-rehearsal-only");
  assert.equal(scenario.executionBoundary.acceptanceClass, "fixture-rehearsal-only");
  assert.equal(scenario.executionBoundary.planFiltering, false);
  assert.equal(scenario.budget.lifecycleElapsedLimitMs, 120000);
  assert.equal(scenario.budget.scope, "single-monotonic-end-to-end-deadline");
  assert.equal(scenario.budget.phaseResetAllowed, false);
  const catalogContract = taxonomy.semanticPackageProjectionContracts[scenario.authority.catalogId], catalogBytes = readInput(catalogContract.authorityCatalogPath), catalog = JSON.parse(catalogBytes.toString());
  assert.equal(hash(catalogBytes), scenario.authority.catalogHash);
  assert.equal(catalogContract.authorityCatalogSha256, scenario.authority.catalogHash);
  assert.deepEqual(parseJsonc(catalogBytes.toString()), catalog);
  const owner = catalog.packages.find((row: any) => row.id === scenario.authority.packageId), generator = taxonomy.generatorContracts[scenario.authority.generatorId];
  assert.equal(owner.mappings.length, scenario.authority.sourceLeafCount);
  const sourcePaths = owner.mappings.map((row: any) => row.sourcePath).sort(order);
  const admitted = execFileSync("git", ["--literal-pathspecs", "ls-files", "--cached", "--others", "--exclude-standard", "-z", "--", owner.sourceRoot], { cwd: repoRoot, encoding: "utf8", timeout: 10000 }).split("\0").filter(Boolean).sort(order);
  assert.deepEqual(admitted, sourcePaths);
  const sources = owner.mappings.map((row: any) => {
    const bytes = readInput(row.sourcePath), mode = lstatSync(join(repoRoot, row.sourcePath)).mode & 0o7777;
    assert.equal(hash(bytes), row.sourceHash, row.sourcePath);
    assert.equal(bytes.length, row.sourceSize, row.sourcePath);
    assert.equal(mode, scenario.authority.sourceMode, row.sourcePath);
    return { path: row.sourcePath, hash: hash(bytes), size: bytes.length, mode };
  });
  assert.equal(generator.packageGeneration.browserProfile.sourceModulePaths.length, scenario.authority.browserModuleCount);
  assert.equal(generator.packageGeneration.browserProfile.sourceModulePaths.length + Object.keys(generator.packageGeneration.browserProfile.workspaceImports).length, scenario.authority.browserInputCount);
  assert.equal(owner.generatedSourceRetirements.length, scenario.expected.generatedSourceRetirements);
  assert.equal(owner.mappings.length - owner.generatedSourceRetirements.length, scenario.expected.authoredMoves);
  assert.equal(generator.outputRoots.length, scenario.expected.generatedArtifacts);
  assert.equal(generator.outputRoots.filter((row: any) => row.inclusion === "tracked").length, scenario.expected.trackedArtifacts);
  assert.equal(generator.outputRoots.filter((row: any) => row.inclusion === "ignored").length, scenario.expected.ignoredArtifacts);
  const normalizationPath = posix.dirname(scenario.authority.taxonomyPath) + "/🧹️normalization/🟦️.ts", normalization = readInput(normalizationPath).toString();
  assert.equal(hash(normalization), scenario.authority.normalizationHash);
  const guard = completenessGuard(normalization);
  for (const vector of scenario.executionBoundary.invalidClauseCases) {
    const replacements: Record<string, string> = { missing: "", duplicate: scenario.executionBoundary.source + "\n" + scenario.executionBoundary.source, "different-package": scenario.executionBoundary.source.replace("wgpu-renderer", "foreign-package"), "foreign-function": "" };
    assert.ok(vector in replacements);
    const candidate = normalization.replace(scenario.executionBoundary.source, replacements[vector]!) + (vector === "foreign-function" ? "\nfunction foreign() { " + scenario.executionBoundary.source + " }\n" : "");
    assert.throws(() => completenessGuard(candidate));
  }
  const matrix = scenario.retainedVectors.flatMap((reference: any) => {
    assert.ok(!isAbsolute(reference.path) && !reference.path.split("/").includes(".."));
    const absolute = join(ticketRoot, reference.path), bytes = readInput(relative(repoRoot, absolute).replaceAll("\\", "/")), vector = JSON.parse(bytes.toString());
    assert.equal(hash(bytes), reference.hash, reference.path);
    assert.deepEqual(parseJsonc(bytes.toString()), vector);
    return reference.groups.map((group: string) => {
      assert.ok(Array.isArray(vector[group]) && vector[group].length > 0);
      assert.equal(new Set(vector[group]).size, vector[group].length);
      return { path: reference.path, hash: hash(bytes), group, cases: vector[group] };
    });
  });
  const phaseIds = scenario.phases.map((phase: any) => phase.id);
  assert.equal(new Set(phaseIds).size, phaseIds.length);
  for (const [index, phase] of scenario.phases.entries()) for (const previous of phase.requires) assert.ok(phaseIds.indexOf(previous) >= 0 && phaseIds.indexOf(previous) < index);
  assert.deepEqual(scenario.expected.emptyCanonicalPlan, { moves: 0, evidenceRemovals: 0, edits: 0, regenerations: 0, unresolved: 0 });
  const runId = scenario.confinement.inspectionRunId, expected = join(acceptanceRoot, scenario.fixture.runDirectory, "🔖️" + runId, scenario.fixture.repositoryDirectory);
  for (const vector of scenario.confinement.cases) {
    const paths: Record<string, string> = { "exact-new-run": expected, "production-root": repoRoot, "acceptance-root": acceptanceRoot, "old-ticket-fixture": join(ticketRoot, "🧪️unrelated"), "different-run": expected.replace(runId, "00000000-0000-4000-8000-000000000002"), "parent-segment": expected + "/../" + scenario.fixture.repositoryDirectory, "relative-root": relative(repoRoot, expected), "symlink-ancestor": expected, "file-ancestor": expected };
    assert.ok(vector.id in paths);
    const kind = (path: string) => path === join(acceptanceRoot, scenario.fixture.runDirectory) && vector.id === "symlink-ancestor" ? "symlink" as const : path === join(acceptanceRoot, scenario.fixture.runDirectory) && vector.id === "file-ancestor" ? "file" as const : existingKind(path);
    if (vector.accepted) assert.equal(assertFixtureRoot(paths[vector.id]!, runId, kind), expected);
    else assert.throws(() => assertFixtureRoot(paths[vector.id]!, runId, kind));
  }
  assert.equal(hash(readInput(normalizationPath)), hash(normalization));
  assert.equal(hash(readInput(scenario.authority.taxonomyPath)), scenario.authority.taxonomyHash);
  assert.equal(hash(readInput(catalogContract.authorityCatalogPath)), scenario.authority.catalogHash);
  for (const row of sources) assert.equal(hash(readInput(row.path)), row.hash, row.path);
  console.log(JSON.stringify({ schemaVersion: 1, phase: "inspection-only", observedAt: new Date().toISOString(), sourceLeafCount: sources.length, sourceCensusHash: hash(JSON.stringify(sources.sort((left: any, right: any) => order(left.path, right.path)))), taxonomyHash: hash(taxonomyText), catalogHash: hash(catalogBytes), normalizationHash: hash(normalization), guard, invalidGuardCases: scenario.executionBoundary.invalidClauseCases.length, confinementCases: scenario.confinement.cases.length, preservedNegativeCount: matrix.reduce((sum: number, row: any) => sum + row.cases.length, 0), additionalNegativeCount: Object.values(scenario.additionalNegatives).reduce((sum: number, values: any) => sum + values.length, 0), matrix, phases: phaseIds, execution: { fixtureCreated: false, compilerRun: false, planRun: false, executorTransformed: false, applyRun: false } }, null, 2));
}

async function testBoundary(): Promise<void> {
  inspect();
  const boundary = await import("./🔒️executor/📜️script.ts");
  assert.equal(typeof boundary.inventoryTaxonomy, "function");
}

if (process.argv[2] === "inspect") inspect();
else if (process.argv[2] === "test-boundary") await testBoundary();
else throw new Error("Expected inspect or test-boundary");
