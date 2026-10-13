/** ⏱️ Rebuilds the frozen-markdown scoped-transaction fixture (`❄️frozen-markdown-coordinates` test) and times its inventory and plan
 * separately against the normalization module under `<libraryRoot>`, so two module revisions can be compared under the same load; the last
 * round's plan is then applied with an injected failure (rollback) and for real (commit), each timed. argv: <libraryRoot> <repoRoot> [rounds] */
import { createHash } from "node:crypto";
import { chmodSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, posix, resolve } from "node:path";
import ts from "typescript";

const [libraryRoot, repoRoot, rounds = "2"] = process.argv.slice(2) as [string, string, string?];
const discovery = await import(resolve(libraryRoot, "🔍️discovery/🟦️.ts"));
const normalization = await import(resolve(libraryRoot, "🧹️normalization/🟦️.ts"));
const sha = (bytes: Uint8Array | string): string => createHash("sha256").update(bytes).digest("hex");
const repoLibrary = join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const path = join(repoLibrary, "🧪️tests/🔬️workspace-contract/🟦️.ts"), tree = ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true);
const names = new Set(["artifactProjectionProducerInput", "artifactProjectionProducerInputs", "projectionByteSort"]);
const compiled = new Bun.Transpiler({ loader: "ts" }).transformSync(tree.statements.filter((node) => ts.isFunctionDeclaration(node) && names.has(node.name?.text ?? "")).map((node) => node.getText(tree)).join("\n"));
const scenario = JSON.parse(readFileSync(join(repoLibrary, "🧫️fixtures/🖍️draw-source-scenario/🔣️.json"), "utf8"));
const build = new Function("getWorkspaceRoot", "lstatSync", "readFileSync", "createHash", "join", "posix", "registryCompilerInputDependencies", "DRAW_SOURCE_SCENARIO", `${compiled}\nreturn { collect: artifactProjectionProducerInputs };`)(() => repoRoot, lstatSync, readFileSync, createHash, join, posix, discovery.registryCompilerInputDependencies, scenario);
const owner = mkdtempSync(join(realpathSync(tmpdir()), "semio-frozen-timing-")), fixture = join(owner, "🧪️fixture");
try {
  mkdirSync(fixture);
  const put = (file: string, bytes: string): void => { mkdirSync(dirname(join(fixture, file)), { recursive: true }); writeFileSync(join(fixture, file), bytes); };
  const git = (args: string[]): string => Bun.spawnSync(["git", ...args], { cwd: fixture, stdout: "pipe" }).stdout.toString().trim();
  const scope = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/MARKDOWN-SOURCE/🧪️tests/🧪️fixture", source = `${scope}/🦀️component.rs`, historyPath = "🧪️tests/🧪️history/📝️.md", livePath = "🧪️tests/🔣️consumer.json";
  const history = `Recorded \`${source}\` before normalization.\n`, encodedHistoryPath = "🧪️tests/🧪️history/🔣️.json", encodedHistory = `[{"path":${JSON.stringify(source).replaceAll("/", "\\/")}}]\n`;
  const declaration = { start: history.indexOf(source), end: history.indexOf(source) + source.length, kind: "source", form: "inline-code", valueSha256: sha(source) };
  const schema = { ...structuredClone(discovery.loadCatalogTaxonomy()), frozenCoordinateEvidenceContracts: { history: { path: encodedHistoryPath, sha256: sha(encodedHistory), schemaVersion: null, rootKind: "array", coordinates: [{ pointer: "/0/path", kind: "source", representation: "json-escaped-source-path" }] } }, frozenMarkdownCoordinateEvidenceContracts: { history: { path: historyPath, grammar: "frozen-markdown-source-coordinates-v1", sha256: sha(history), coordinates: [declaration] } } };
  const producer = build.collect(schema) as { files: Record<string, { content: string; mode: number }> };
  for (const [file, input] of Object.entries(producer.files)) { put(file, input.content); chmodSync(join(fixture, file), input.mode); }
  const schemaPath = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json";
  put(schemaPath, `${JSON.stringify(schema, null, 2)}\n`); put(source, "pub fn value() -> u32 { 7 }\n"); put(historyPath, history); put(encodedHistoryPath, encodedHistory); put(livePath, `${JSON.stringify({ sourcePath: source })}\n`);
  git(["init", "-q"]); put(".git/info/exclude", `${schemaPath}\n`); git(["add", "--all"]);
  git(["-c", "user.name=Timing", "-c", "user.email=timing@invalid.example", "-c", "commit.gpgsign=false", "commit", "-qm", "timing"]);
  const baselineCommit = git(["rev-parse", "HEAD"]);
  console.log(`[DEBUG] files=${Object.keys(producer.files).length}`);
  let plan: { planDigest: string; moves: unknown[]; edits: unknown[]; unresolved: unknown[] } | undefined;
  for (let round = 0; round < Number(rounds); round++) {
    const started = performance.now(), inventory = normalization.inventoryTaxonomy({ repoRoot: fixture, scope, workers: 1 }), inventoried = performance.now();
    plan = normalization.planTaxonomy(inventory, { baselineCommit, excludedTreeDigests: [] });
    const planned = performance.now();
    console.log(`[DEBUG] round=${round} inventoryMs=${(inventoried - started).toFixed(0)} planMs=${(planned - inventoried).toFixed(0)} moves=${plan!.moves.length} edits=${plan!.edits.length} unresolved=${plan!.unresolved.length}`);
  }
  const ticketDir = join(fixture, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/MARKDOWN-TRANSACTION");
  mkdirSync(ticketDir, { recursive: true });
  const options = { repoRoot: fixture, ticketDir, expectedBaselineCommit: baselineCommit, expectedPlanDigest: plan!.planDigest };
  const applying = performance.now(), rolledBack = normalization.applyTaxonomyPlan(plan, { ...options, injectFailureAt: "after-edits" }).state, rolled = performance.now();
  const committed = normalization.applyTaxonomyPlan(plan, options).state, done = performance.now();
  console.log(`[DEBUG] rollback=${rolledBack} rollbackMs=${(rolled - applying).toFixed(0)} commit=${committed} commitMs=${(done - rolled).toFixed(0)}`);
} finally { rmSync(owner, { recursive: true, force: true }); }
