#!/usr/bin/env bun
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, readdirSync, statSync, writeFileSync } from "node:fs";
import { basename, dirname, join, relative, resolve, sep } from "node:path";

type OracleMove = { source: string; destination: string; sha256: string; bytes: number };
type TestContextMove = { source: string; owner: string; parent: string; unit: string };
type ScanFile = { absolute: string; relative: string; text?: string };

const ticketRoot = dirname(import.meta.dir);
const generatedRoot = join(ticketRoot, "🗑️generated", "testing-taxonomy", "plugins");

function repoRootFrom(start: string): string {
  let current = start;
  while (dirname(current) !== current) {
    if (existsSync(join(current, "AGENTS.md")) && existsSync(join(current, "✏️s"))) return current;
    current = dirname(current);
  }
  throw new Error(`repository root not found from ${start}`);
}

const repoRoot = repoRootFrom(ticketRoot);

function posix(path: string): string {
  return path.split(sep).join("/");
}

function sha256(path: string): string {
  return createHash("sha256").update(readFileSync(path)).digest("hex");
}

function scanTree(root: string): { directories: string[]; files: ScanFile[] } {
  const directories: string[] = [];
  const files: ScanFile[] = [];
  const frontier = [root];
  const ignored = new Set([".git", ".venv", "node_modules", "target"]);
  while (frontier.length > 0) {
    const directory = frontier.pop()!;
    for (const entry of readdirSync(directory, { withFileTypes: true })) {
      const absolute = join(directory, entry.name);
      const rel = posix(relative(repoRoot, absolute));
      if (entry.isDirectory()) {
        if (ignored.has(entry.name)) continue;
        directories.push(rel);
        frontier.push(absolute);
        continue;
      }
      if (!entry.isFile()) continue;
      const size = statSync(absolute).size;
      let text: string | undefined;
      if (size <= 8 * 1024 * 1024) {
        const bytes = readFileSync(absolute);
        if (!bytes.includes(0)) text = bytes.toString("utf8");
      }
      files.push({ absolute, relative: rel, text });
    }
  }
  return { directories, files };
}

function readJson<T>(path: string): T {
  return JSON.parse(readFileSync(path, "utf8")) as T;
}

function frameworkExports(): Set<string> {
  const path = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs");
  const text = readFileSync(path, "utf8");
  const start = text.indexOf("pub mod artifact_app_laws");
  const end = text.indexOf("//#endregion 🔖️ArtifactAppLaws", start);
  if (start < 0 || end < 0) throw new Error("artifact_app_laws module boundaries not found");
  const names = new Set<string>();
  for (const match of text.slice(start, end).matchAll(/pub\s+(?:async\s+)?(?:fn|struct|const|type)\s+([A-Za-z_][A-Za-z0-9_]*)/gu)) names.add(match[1]!);
  return names;
}

function protocolExports(): Set<string> {
  const path = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️tests/⚖️protocol-laws/🦀️.rs");
  const text = readFileSync(path, "utf8");
  return new Set([...text.matchAll(/pub\s+(?:async\s+)?(?:fn|struct|const|type)\s+([A-Za-z_][A-Za-z0-9_]*)/gu)].map((match) => match[1]!));
}

function lineNumber(text: string, index: number): number {
  return text.slice(0, index).split("\n").length;
}

function ownerManifest(path: string): string | null {
  let current = dirname(join(repoRoot, path));
  while (current.startsWith(repoRoot)) {
    const manifest = join(current, "📦️packages", "🦀️rust", "Cargo.toml");
    if (existsSync(manifest)) return manifest;
    const parent = dirname(current);
    if (parent === current) break;
    current = parent;
  }
  return null;
}

function runScan(): void {
  mkdirSync(generatedRoot, { recursive: true });
  console.log("[DEBUG] scanning plugin and hub testing taxonomy");
  const trees = [scanTree(join(repoRoot, "✏️s")), scanTree(join(repoRoot, "🌎️hub"))];
  const directories = trees.flatMap((tree) => tree.directories);
  const files = trees.flatMap((tree) => tree.files);
  const allowedOpaqueOracle = "🌎️hub/📇️directory/🧫️fixtures/🎯️admin-intent-v1/🧪️oracle";
  const forbiddenDirectoryNames = new Set(["🔬️testkit", "🧪️testkit", "🔬️test-support", "🔬️testing-support", "🔬️compliance-helpers", "🔬️document-helpers", "🔬️testgen", "🔮️oracle", "🧪️oracle", "⚖️oracle"]);
  const forbiddenDirectories = directories.filter((path) => forbiddenDirectoryNames.has(basename(path)) && path !== allowedOpaqueOracle).sort();
  const pluralOracleRoots = directories.filter((path) => basename(path) === "🔮️oracles").sort();
  const activeTextFiles = files.filter((file) => file.text !== undefined && !file.relative.includes("/🧫️fixtures/") && !file.relative.includes("/🔮️oracles/"));
  const staleTextPattern = /[A-Za-z_]*testkit[A-Za-z_]*|test-support|testing-support|compliance-helpers|document-helpers|🔬️testgen|🔮️oracle\/|🧪️oracle\/|⚖️oracle\//gu;
  const staleText: { path: string; line: number; value: string }[] = [];
  for (const file of activeTextFiles) {
    for (const match of file.text!.matchAll(staleTextPattern)) staleText.push({ path: file.relative, line: lineNumber(file.text!, match.index!), value: match[0] });
  }
  const oracleLedgerPath = join(generatedRoot, "oracle-file-ledger.json");
  const oracleMoves = readJson<OracleMove[]>(oracleLedgerPath);
  const authoredBaseline = readJson<{ kind: string; source: string; destination?: string; sha256Before: string; root: string }[]>(join(generatedRoot, "authored-baseline-ledger.json"));
  const knownOracleSources = new Set(oracleMoves.map((move) => move.source));
  const protocolOracleCase = "✏️s/🔌️plugins/🎬️sequence/🧪️tests/🔮️protocol-oracle/🟨️.js";
  for (const move of authoredBaseline.filter((entry) => entry.kind === "oracle-move" && entry.source !== protocolOracleCase && entry.destination && !knownOracleSources.has(entry.source))) {
    const destination = join(repoRoot, move.destination!);
    oracleMoves.push({ source: move.source, destination: move.destination!, sha256: move.sha256Before, bytes: existsSync(destination) ? statSync(destination).size : 0 });
  }
  const legacyRootFailures = authoredBaseline.filter((entry) => entry.source !== protocolOracleCase && existsSync(join(repoRoot, entry.root))).map((entry) => entry.root).sort();
  const chainedMoves = new Map(readJson<{ source: string; destination: string }[]>(join(generatedRoot, "wfc-moves.json")).map((move) => [move.source, move.destination]));
  const oracleChecks = oracleMoves.map((move) => {
    const source = join(repoRoot, move.source);
    const terminalDestination = chainedMoves.get(move.destination) ?? move.destination;
    const destination = join(repoRoot, terminalDestination);
    const destinationExists = existsSync(destination);
    const postSha256 = destinationExists ? sha256(destination) : null;
    const postBytes = destinationExists ? statSync(destination).size : null;
    return { ...move, terminalDestination, sourceAbsent: !existsSync(source), destinationExists, postSha256, postBytes, bytePreserved: postSha256 === move.sha256 && postBytes === move.bytes };
  });
  const oracleFailures = oracleChecks.filter((check) => !check.sourceAbsent || !check.destinationExists);
  const rustIncludes: { reader: string; literal: string; resolved: string; bytes: number | null }[] = [];
  for (const file of files.filter((candidate) => candidate.relative.endsWith(".rs") && candidate.text?.includes("🔮️oracles"))) {
    for (const match of file.text!.matchAll(/include_(?:str|bytes)!\(\s*"([^"]*🔮️oracles[^"]*)"\s*\)/gu)) {
      const absolute = resolve(dirname(file.absolute), match[1]!);
      rustIncludes.push({ reader: file.relative, literal: match[1]!, resolved: posix(relative(repoRoot, absolute)), bytes: existsSync(absolute) ? statSync(absolute).size : null });
    }
  }
  const missingRustIncludes = rustIncludes.filter((entry) => entry.bytes === null);
  const exports = frameworkExports();
  const frameworkReferences: { path: string; line: number; name: string }[] = [];
  for (const file of activeTextFiles.filter((candidate) => candidate.text!.includes("artifact_app_laws::"))) {
    for (const match of file.text!.matchAll(/artifact_app_laws::([A-Za-z_][A-Za-z0-9_]*)/gu)) frameworkReferences.push({ path: file.relative, line: lineNumber(file.text!, match.index!), name: match[1]! });
  }
  const invalidFrameworkReferences = frameworkReferences.filter((reference) => !exports.has(reference.name));
  const frameworkConsumerFeatureFailures = [...new Set(activeTextFiles.filter((file) => file.relative.endsWith(".rs") && file.text!.includes("artifact_app_laws")).map((file) => file.relative))].flatMap((path) => {
    const manifest = ownerManifest(path);
    return manifest && readFileSync(manifest, "utf8").includes("artifact-app-testing") ? [] : [{ path, manifest: manifest ? posix(relative(repoRoot, manifest)) : null }];
  });
  const sprExports = protocolExports();
  const protocolReferences: { path: string; line: number; name: string }[] = [];
  for (const file of activeTextFiles.filter((candidate) => candidate.text!.includes("protocol_laws::"))) {
    for (const match of file.text!.matchAll(/protocol_laws::([A-Za-z_][A-Za-z0-9_]*)/gu)) protocolReferences.push({ path: file.relative, line: lineNumber(file.text!, match.index!), name: match[1]! });
  }
  const invalidProtocolReferences = protocolReferences.filter((reference) => !sprExports.has(reference.name));
  const protocolConsumerFeatureFailures = [...new Set(activeTextFiles.filter((file) => file.relative.endsWith(".rs") && file.text!.includes("protocol_laws")).map((file) => file.relative))].flatMap((path) => {
    const manifest = ownerManifest(path);
    return manifest && readFileSync(manifest, "utf8").includes("protocol-laws") ? [] : [{ path, manifest: manifest ? posix(relative(repoRoot, manifest)) : null }];
  });
  const testkitPlan = readJson<TestContextMove[]>(join(generatedRoot, "testkit-plan.json"));
  const testContextChecks = testkitPlan.map((move) => {
    const sourceAbsent = !existsSync(join(repoRoot, move.source));
    const unitText = existsSync(join(repoRoot, move.unit)) ? readFileSync(join(repoRoot, move.unit), "utf8") : "";
    const exception = move.source.includes("✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🧪️tests/🔬️testkit/") ? "sample-scene-fixture" : move.source.includes("✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/") && move.source.includes("/🧬️schema/🧪️tests/🔬️testkit/") ? "precompute-model-case" : null;
    const destinationPresent = exception === "sample-scene-fixture" ? existsSync(join(repoRoot, move.owner, "🧫️fixtures/🧩️sample-scene/🦀️.rs")) : exception === "precompute-model-case" ? existsSync(join(repoRoot, move.owner, "🧪️tests/🔬️precompute-model/🦀️.rs")) : unitText.includes("pub(crate) mod context");
    return { ...move, sourceAbsent, destinationPresent, exception };
  });
  const testContextFailures = testContextChecks.filter((check) => !check.sourceAbsent || !check.destinationPresent);
  const oracleReferenceFiles = files.filter((file) => file.text?.includes("🔮️oracles")).map((file) => file.relative).sort();
  const result = {
    status: forbiddenDirectories.length === 0 && legacyRootFailures.length === 0 && staleText.length === 0 && oracleFailures.length === 0 && missingRustIncludes.length === 0 && invalidFrameworkReferences.length === 0 && frameworkConsumerFeatureFailures.length === 0 && invalidProtocolReferences.length === 0 && protocolConsumerFeatureFailures.length === 0 && testContextFailures.length === 0 ? "passed" : "failed",
    counts: {
      scannedDirectories: directories.length,
      scannedFiles: files.length,
      pluralOracleRoots: pluralOracleRoots.length,
      oracleMoves: oracleChecks.length,
      bytePreservedOracleMoves: oracleChecks.filter((check) => check.bytePreserved).length,
      contentUpdatedAfterRelocation: oracleChecks.filter((check) => !check.bytePreserved).length,
      rustOracleIncludes: rustIncludes.length,
      oracleReferenceFiles: oracleReferenceFiles.length,
      frameworkReferences: frameworkReferences.length,
      frameworkConsumerCrates: new Set(frameworkReferences.map((reference) => ownerManifest(reference.path)).filter(Boolean)).size,
      protocolReferences: protocolReferences.length,
      protocolConsumerCrates: new Set(protocolReferences.map((reference) => ownerManifest(reference.path)).filter(Boolean)).size,
      testContextMoves: testContextChecks.length,
    },
    allowedOpaqueOracle,
    forbiddenDirectories,
    legacyRootFailures,
    staleText,
    oracleFailures,
    missingRustIncludes,
    invalidFrameworkReferences,
    frameworkConsumerFeatureFailures,
    invalidProtocolReferences,
    protocolConsumerFeatureFailures,
    testContextFailures,
    rustIncludes,
    oracleChecks,
    oracleReferenceFiles,
  };
  const output = join(generatedRoot, "verification-scan.json");
  writeFileSync(output, `${JSON.stringify(result, null, 2)}\n`);
  console.log(JSON.stringify({ status: result.status, counts: result.counts, failures: { forbiddenDirectories: forbiddenDirectories.length, legacyRoots: legacyRootFailures.length, staleText: staleText.length, oracleMoves: oracleFailures.length, rustIncludes: missingRustIncludes.length, frameworkReferences: invalidFrameworkReferences.length, frameworkConsumerFeatures: frameworkConsumerFeatureFailures.length, protocolReferences: invalidProtocolReferences.length, protocolConsumerFeatures: protocolConsumerFeatureFailures.length, testContexts: testContextFailures.length }, output: posix(relative(repoRoot, output)) }, null, 2));
  if (result.status !== "passed") process.exitCode = 1;
}

function packageNamesWithArtifactLaws(): string[] {
  const manifests = scanTree(join(repoRoot, "✏️s", "🔌️plugins")).files.filter((file) => basename(file.absolute) === "Cargo.toml" && file.text?.includes("artifact-app-testing"));
  const names = manifests.map((file) => file.text!.match(/\[package\][\s\S]*?\nname\s*=\s*"([^"]+)"/u)?.[1]).filter((name): name is string => Boolean(name));
  return [...new Set(names)].sort();
}

async function runProcess(command: string[], cwd = repoRoot): Promise<void> {
  mkdirSync(generatedRoot, { recursive: true });
  console.log(`[DEBUG] ${command.join(" ")}`);
  const child = Bun.spawn(command, { cwd, env: { ...process.env, CARGO_TARGET_DIR: join(generatedRoot, "cargo-target"), CARGO_TERM_COLOR: "never" }, stdin: "inherit", stdout: "inherit", stderr: "inherit" });
  const interrupt = () => child.kill("SIGINT");
  process.once("SIGINT", interrupt);
  const code = await child.exited;
  process.off("SIGINT", interrupt);
  if (code !== 0) throw new Error(`${command[0]} exited with ${code}`);
}

async function cargoCheck(packages: string[]): Promise<void> {
  const selected = packages.length > 0 ? packages : packageNamesWithArtifactLaws();
  console.log(`[DEBUG] checking ${selected.length} Cargo packages`);
  await runProcess(["cargo", "check", "--tests", ...selected.flatMap((name) => ["-p", name])]);
}

async function cargoTest(segments: string[]): Promise<void> {
  const [name, ...filter] = segments;
  if (!name) throw new Error("cargo-test requires a package name");
  await runProcess(["cargo", "test", "-p", name, "--lib", ...filter]);
}

async function energyStatus(): Promise<void> {
  const directory = join(repoRoot, "✏️s/🔌️plugins/🔋️energy/🔮️oracles/📦️packages/🐍️python");
  await runProcess(["bun", "./📜️script.ts", "status"], directory);
}

async function energyTest(): Promise<void> {
  const directory = join(repoRoot, "✏️s/🔌️plugins/🔋️energy/🔮️oracles/📦️packages/🐍️python");
  await runProcess(["bun", "./📜️script.ts", "test"], directory);
}

async function sequenceTest(): Promise<void> {
  const directory = join(repoRoot, "✏️s/🔌️plugins/🎬️sequence/📦️packages/🟦️typescript");
  await runProcess(["bun", "./📜️script.ts", "test"], directory);
}

async function sequenceOracle(): Promise<void> {
  const path = join(repoRoot, "✏️s/🔌️plugins/🎬️sequence/🧪️tests/🔮️protocol-oracle/🟨️.js");
  await runProcess(["bun", path]);
}

function authoredRustFiles(): string[] {
  const paths = new Set<string>();
  const add = (path: string | undefined) => {
    if (path?.endsWith(".rs") && existsSync(join(repoRoot, path))) paths.add(path);
  };
  for (const path of readJson<string[]>(join(generatedRoot, "testkit-final-edits.json"))) add(path);
  for (const move of readJson<TestContextMove[]>(join(generatedRoot, "testkit-plan.json"))) {
    add(move.parent);
    add(move.unit);
  }
  for (const name of ["framework-context-corrections.json", "local-context-restorations.json"])
    for (const entry of readJson<{ path: string }[]>(join(generatedRoot, name))) add(entry.path);
  for (const name of ["compliance-moves.json", "document-moves.json", "wfc-moves.json", "hub-moves.json"])
    for (const move of readJson<{ destination: string }[]>(join(generatedRoot, name))) add(move.destination);
  const chainedMoves = new Map(readJson<{ source: string; destination: string }[]>(join(generatedRoot, "wfc-moves.json")).map((move) => [move.source, move.destination]));
  for (const move of readJson<OracleMove[]>(join(generatedRoot, "oracle-file-ledger.json"))) add(chainedMoves.get(move.destination) ?? move.destination);
  add("✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🧪️tests/🔬️unit/🦀️.rs");
  add("✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧪️tests/🔬️unit/🦀️.rs");
  return [...paths].sort();
}

async function rustfmtParse(): Promise<void> {
  mkdirSync(generatedRoot, { recursive: true });
  const files = authoredRustFiles();
  const failures: { path: string; stderr: string }[] = [];
  for (const [index, path] of files.entries()) {
    if (index % 25 === 0) console.log(`[DEBUG] rustfmt parser ${index}/${files.length}`);
    const child = Bun.spawn(["rustfmt", "--edition", "2021", "--emit", "stdout", join(repoRoot, path)], { cwd: repoRoot, stdout: "ignore", stderr: "pipe" });
    const stderr = await new Response(child.stderr).text();
    const code = await child.exited;
    if (code !== 0) failures.push({ path, stderr });
  }
  const output = join(generatedRoot, "rustfmt-parser.json");
  writeFileSync(output, `${JSON.stringify({ files: files.length, failures }, null, 2)}\n`);
  console.log(JSON.stringify({ files: files.length, failures: failures.length, output: posix(relative(repoRoot, output)) }, null, 2));
  if (failures.length > 0) process.exitCode = 1;
}

function executionLedger(): void {
  mkdirSync(generatedRoot, { recursive: true });
  const protocolOracleCase = "✏️s/🔌️plugins/🎬️sequence/🧪️tests/🔮️protocol-oracle/🟨️.js";
  const baseline = readJson<{ kind: string; source: string; destination?: string; sha256Before: string }[]>(join(generatedRoot, "authored-baseline-ledger.json")).filter((entry) => entry.source !== protocolOracleCase);
  const helperLedger = readJson<{ category: string; source: string; sha256: string; bytes: number }[]>(join(generatedRoot, "helper-file-ledger.json"));
  const helperBySource = new Map(helperLedger.map((entry) => [entry.source, entry]));
  const verification = readJson<{ oracleChecks: { source: string; destination: string; terminalDestination: string; sha256: string; bytes: number; postSha256: string | null; postBytes: number | null; bytePreserved: boolean }[] }>(join(generatedRoot, "verification-scan.json"));
  const removedSources = baseline.map((entry) => ({ category: entry.kind, path: entry.source, sha256Before: entry.sha256Before, absentAfter: !existsSync(join(repoRoot, entry.source)) })).sort((left, right) => left.path.localeCompare(right.path));
  const createdDestinations: Record<string, unknown>[] = verification.oracleChecks.map((move) => ({ category: "oracle", source: move.source, path: move.destination, terminalPath: move.terminalDestination, presentAfter: existsSync(join(repoRoot, move.destination)), sha256Before: move.sha256, sha256After: move.postSha256, bytesBefore: move.bytes, bytesAfter: move.postBytes, bytePreserved: move.bytePreserved }));
  const terminalOverrides = new Map<string, string>([
    ["✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️document-helpers/🦀️.rs", "✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🧩️document-behavior/🦀️.rs"],
  ]);
  for (const name of ["compliance-moves.json", "document-moves.json", "wfc-moves.json", "hub-moves.json"]) {
    for (const move of readJson<{ source: string; destination: string }[]>(join(generatedRoot, name))) {
      const before = helperBySource.get(move.source);
      const terminalPath = terminalOverrides.get(move.source) ?? move.destination;
      const destination = join(repoRoot, terminalPath);
      createdDestinations.push({ category: before?.category ?? name.replace("-moves.json", ""), source: move.source, path: move.destination, terminalPath, presentAfter: existsSync(join(repoRoot, move.destination)), sha256Before: before?.sha256 ?? null, sha256After: existsSync(destination) ? sha256(destination) : null, bytesBefore: before?.bytes ?? null, bytesAfter: existsSync(destination) ? statSync(destination).size : null, bytePreserved: Boolean(before && existsSync(destination) && before.sha256 === sha256(destination) && before.bytes === statSync(destination).size) });
    }
  }
  const testkitPlan = readJson<TestContextMove[]>(join(generatedRoot, "testkit-plan.json"));
  const mergedSources: Record<string, unknown>[] = [];
  for (const move of testkitPlan) {
    const before = helperBySource.get(move.source);
    const destination = move.source.includes("✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🧪️tests/🔬️testkit/") ? join(move.owner, "🧫️fixtures/🧩️sample-scene/🦀️.rs") : move.source.includes("✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/") && move.source.includes("/🧬️schema/🧪️tests/🔬️testkit/") ? join(move.owner, "🧪️tests/🔬️precompute-model/🦀️.rs") : move.unit;
    const absolute = join(repoRoot, destination);
    const record = { category: "testkit", source: move.source, path: posix(destination), sha256Before: before?.sha256 ?? null, sha256After: existsSync(absolute) ? sha256(absolute) : null, bytesBefore: before?.bytes ?? null, bytesAfter: existsSync(absolute) ? statSync(absolute).size : null, bytePreserved: Boolean(before && existsSync(absolute) && before.sha256 === sha256(absolute) && before.bytes === statSync(absolute).size) };
    if (destination !== move.unit) createdDestinations.push(record);
    else mergedSources.push(record);
  }
  const supportDestinations = new Map<string, string>([
    ["✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️test-support/🦀️.rs", "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs"],
    ["✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🧪️tests/🔬️test-support/🦀️.rs", "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🧪️tests/🔬️unit/🦀️.rs"],
    ["✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🔄️fsm/🧪️tests/🔬️testing-support/🦀️.rs", "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🔄️fsm/🧪️tests/🔀️toggle-machine/🦀️.rs"],
  ]);
  for (const [source, destination] of supportDestinations) {
    const before = helperBySource.get(source);
    const absolute = join(repoRoot, destination);
    mergedSources.push({ category: before?.category ?? "support", source, path: destination, sha256Before: before?.sha256 ?? null, sha256After: existsSync(absolute) ? sha256(absolute) : null, bytesBefore: before?.bytes ?? null, bytesAfter: existsSync(absolute) ? statSync(absolute).size : null, bytePreserved: false });
  }
  const modified = new Set<string>();
  const add = (path: string | undefined) => {
    if (path && existsSync(join(repoRoot, path)) && statSync(join(repoRoot, path)).isFile()) modified.add(posix(path));
  };
  for (const line of readFileSync(join(generatedRoot, "oracle-consumers-before.txt"), "utf8").split("\n")) add(line.trim());
  for (const path of readJson<string[]>(join(generatedRoot, "testkit-final-edits.json"))) add(path);
  for (const name of ["framework-context-corrections.json", "local-context-restorations.json"])
    for (const entry of readJson<{ path: string }[]>(join(generatedRoot, name))) add(entry.path);
  for (const path of readJson<string[]>(join(generatedRoot, "norm-feature-edits.json"))) add(path);
  for (const move of testkitPlan) {
    add(move.parent);
    add(move.unit);
  }
  for (const record of [...createdDestinations, ...mergedSources]) {
    add(record.path as string);
    add(record.terminalPath as string | undefined);
  }
  add("✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🧪️tests/🔬️unit/🦀️.rs");
  add("✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧪️tests/🔬️unit/🦀️.rs");
  add("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🦀️.rs");
  add("✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs");
  add("✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📜️script.ts");
  const currentTrees = [scanTree(join(repoRoot, "✏️s")), scanTree(join(repoRoot, "🌎️hub"))];
  const semanticConsumerPattern = /artifact_app_laws|protocol_laws|integration-fixtures|compliance-testing|sample_scene_fixture|model_vectors|engine_test_vectors/u;
  for (const file of currentTrees.flatMap((tree) => tree.files)) if (file.text && semanticConsumerPattern.test(file.text)) add(file.relative);
  for (const file of scanTree(join(repoRoot, "✏️s", "🔌️plugins")).files.filter((candidate) => basename(candidate.absolute) === "Cargo.toml" && candidate.text?.includes("artifact-app-testing"))) add(file.relative);
  const modifiedConsumers = [...modified].sort().map((path) => ({ path, sha256After: sha256(join(repoRoot, path)) }));
  const scriptPath = posix(relative(repoRoot, import.meta.path));
  const handoffPath = posix(relative(repoRoot, join(ticketRoot, "📓️plugins-handoff-2026-09-12.md")));
  const reportPath = posix(relative(repoRoot, join(ticketRoot, "📓️plugins-testing-taxonomy-execution-2026-09-12.md")));
  const ledger = {
    summary: { removedSources: removedSources.length, createdDestinations: createdDestinations.length, mergedSources: mergedSources.length, modifiedConsumers: modifiedConsumers.length, createdTicketFiles: 3 },
    removedSources,
    createdDestinations: createdDestinations.sort((left, right) => String(left.path).localeCompare(String(right.path))),
    mergedSources: mergedSources.sort((left, right) => String(left.source).localeCompare(String(right.source))),
    modifiedConsumers,
    createdTicketFiles: [
      { path: scriptPath, sha256After: sha256(import.meta.path) },
      { path: handoffPath, sha256After: sha256(join(repoRoot, handoffPath)) },
      { path: reportPath, sha256After: null, note: "self-contained report embedding this ledger" },
    ],
    preservedClassifications: [
      { path: protocolOracleCase, classification: "test-case implementation, not an oracle collection alias" },
      { pattern: "🔬️oracle-{standalone,unit} / 🔬️oracles-unit / 🦀️oracle-probe", classification: "semantic test cases or developer probes, not collection aliases" },
      { path: "🌎️hub/📇️directory/🧫️fixtures/🎯️admin-intent-v1/🧪️oracle", classification: "opaque hostile fixture path" },
    ],
  };
  const output = join(generatedRoot, "execution-ledger.json");
  writeFileSync(output, `${JSON.stringify(ledger, null, 2)}\n`);
  console.log(JSON.stringify({ summary: ledger.summary, output: posix(relative(repoRoot, output)) }, null, 2));
}

function executionReport(): void {
  const ledgerPath = join(generatedRoot, "execution-ledger.json");
  const scan = readJson<{ counts: Record<string, number>; failures: Record<string, number> }>(join(generatedRoot, "verification-scan.json"));
  const ledgerText = readFileSync(ledgerPath, "utf8").trimEnd();
  const reportPath = join(ticketRoot, "📓️plugins-testing-taxonomy-execution-2026-09-12.md");
  const lines = [
    "# Plugins Testing Taxonomy Execution — 2026-09-12",
    "",
    "## Outcome",
    "",
    "The bounded non-framework plugin and hub lane is implemented. The final private-Nx structural scan passed with zero forbidden testing buckets, legacy roots, stale active references, unresolved oracle moves/includes, invalid framework/protocol readers, missing Cargo feature wiring, or missing co-located test contexts.",
    "",
    `The scan covered ${scan.counts.scannedDirectories} directories and ${scan.counts.scannedFiles} files. It found ${scan.counts.pluralOracleRoots} canonical \`🔮️oracles\` roots, verified ${scan.counts.oracleMoves} file moves, resolved ${scan.counts.rustOracleIncludes} Rust oracle includes, checked ${scan.counts.oracleReferenceFiles} oracle-reference files, and verified ${scan.counts.testContextMoves} former testkit contexts.`,
    "",
    "## Implemented Taxonomy",
    "",
    "- Relocated every authored singular or alternate non-framework oracle collection to canonical `🔮️oracles`, including manifests, Rust modules, packages, and active readers. Of 362 moved oracle files, 226 remain byte-identical. The remaining 136 intentionally changed after relocation: 116 JSON files received embedded canonical-path rewrites, 19 Rust files received reader/module path changes, and one TypeScript Energy reader received its canonical path change.",
    "",
    "- Eliminated all 56 `🔬️testkit` roots after inspecting their contents. Fifty-four reusable case-local contexts now live privately inside their owning unit implementations. CAD sample-scene data is an owner fixture at `🧫️fixtures/🧩️sample-scene`; Puzzle3d precomputation is a semantic `🧪️tests/🔬️precompute-model` implementation.",
    "",
    "- Rehomed 15 compliance helper roots to semantic `🧪️tests/⚖️compliance` ownership and replaced the generic feature with test-only `compliance-testing` wiring. Rehomed the two document helper roots to semantic document-behavior/block-default test implementations. WFC test generation is now `🧪️tests/🧮️model-vectors` with module `model_vectors`.",
    "",
    "- Merged Draw `testing-support`, Procedural `test-support`, and Raster `test-support` directly into their owning test implementations. Hub trusted-catalog data now belongs to `🌎️hub/🧪️tests/🔏️trusted-catalog-profile` behind the non-default `integration-fixtures` feature.",
    "",
    "- Updated framework consumers to `artifact_app_laws` and OS SPR consumers to `protocol_laws` following the framework handoff. FEM numerical test algorithms now use the semantically owned test-only `engine_test_vectors` module.",
    "",
    "- Preserved `✏️s/🔌️plugins/🎬️sequence/🧪️tests/🔮️protocol-oracle/🟨️.js` because the audit evidence identifies it as a semantic test case, not an oracle collection. Preserved the malformed `🧪️oracle` directory only inside the explicitly opaque hostile hub fixture payload. No arbitrary helper code was relabeled as an oracle.",
    "",
    "## Production Isolation",
    "",
    "CAD sample-scene fixtures and WFC model vectors are compiled only for tests. Norm compliance support is excluded from defaults and consumed through development feature wiring. Hub `integration-fixtures` is excluded from defaults, and no other Cargo dependency enables it. The structural scan found no production dependency on the removed helper buckets.",
    "",
    "## Verification",
    "",
    "| Check | Actual outcome |",
    "| --- | --- |",
    "| Private Nx `scan` | Passed: every recorded structural failure count is zero. |",
    "| Energy environment status | Passed: OpenStudio 3.11.0, EnergyPlus 25.2.0, Python 3.12.13, honeybee-energy 1.123.32, honeybee-openstudio 0.7.2, ladybug-core 0.44.59. |",
    "| Energy neutral-vector run | Passed all 11 checks over ASHRAE cases 600, 600FF, 610, 620, 640, 900, 900FF, 910, 920, and 940, including translation. |",
    "| Sequence independent oracle | Passed against third-party Ajv: `{\"oracle\":\"ajv-test-only\",\"interface\":\"owned\",\"feature\":\"editing\",\"protocol\":\"equal\",\"semantic\":\"equal\"}`. |",
    "| Full Sequence script | Reached the first two JavaScript checks, then failed in the root-owned retained-actions document comparator because strict Ajv rejects unknown `x-semio-child-kind`. This lane did not alter the root-owned comparator. |",
    "| Five-package Cargo check | Blocked before selected plugin crates by concurrent framework errors: missing OS SPR `LoadDocumentArchive`/`ReadDocumentArchive`/`DocumentArchive` match arms and missing mounted UI runtime output `../../📤️output/🦀️.rs`. No full Cargo-pass claim is made. |",
    "| Rust parser sweep | Parsed 365 of 366 selected files. The remaining failure is a concurrent CAD production parse error at `🧬️schema/🔺️diff/🦀️.rs:36` (`expected identifier`), reached while recursively loading the CAD artifact root. |",
    "",
    "All commands above were run through the ticket's bounded `layout-probe` Nx workspace with the daemon and plugin isolation disabled and Nx workspace/cache data confined to this lane's generated directory. Generated verifier outputs were removed after their evidence was embedded below.",
    "",
    "## Coordination Handoff",
    "",
    "Exact root-owned and concurrent-scope findings are recorded in `📓️plugins-handoff-2026-09-12.md`. No root `Cargo.toml` or root `📜️script.ts` change is required by this lane.",
    "",
    "## Exact Authored File Ledger",
    "",
    "The ledger records every removed source, created destination (including terminal paths where another coordinated normalization followed this lane's move), merged source, modified consumer, ticket artifact, pre-move hash, post-move hash, byte size, and byte-preservation result.",
    "",
    "```json",
    ledgerText,
    "```",
    "",
  ];
  writeFileSync(reportPath, lines.join("\n"));
  console.log(posix(relative(repoRoot, reportPath)));
}

const [command = "scan", ...segments] = Bun.argv.slice(2);
if (command === "scan") runScan();
else if (command === "packages") console.log(packageNamesWithArtifactLaws().join("\n"));
else if (command === "cargo-check") await cargoCheck(segments);
else if (command === "cargo-test") await cargoTest(segments);
else if (command === "energy-status") await energyStatus();
else if (command === "energy-test") await energyTest();
else if (command === "sequence-test") await sequenceTest();
else if (command === "sequence-oracle") await sequenceOracle();
else if (command === "rustfmt-parse") await rustfmtParse();
else if (command === "ledger") executionLedger();
else if (command === "report") executionReport();
else throw new Error(`unknown command: ${command}`);
