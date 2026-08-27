import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { lstatSync, readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import stringify from "fast-json-stable-stringify";

const repoRoot = resolve(import.meta.dir, "../../../../../../../..");
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const vectorPath = library + "/📦️packages/🟦️typescript/🧫️fixtures/🧪️artifact-support-leaf-authority/🔣️.json";
const vector = JSON.parse(readFileSync(join(repoRoot, vectorPath), "utf8"));
const sourceRevision = "a8d1caf41f68204e73ff5e47ce40c5f543ed442d";
const changedPath = vector.owner + "/" + vector.subset + "/🧵️simulation-session/🦀️component.rs";
const sha = (bytes: string | Uint8Array) => createHash("sha256").update(bytes).digest("hex");
const order = (a: string, b: string) => Buffer.compare(Buffer.from(a), Buffer.from(b));

/** 🔬️ Reads an exact admitted implementation leaf without following any ancestor link. */
function physical(path: string): Buffer {
  assert.ok(path.startsWith(vector.owner + "/") || vector.ownerReadiness.contextSources.some((row: { path: string }) => row.path === path));
  let current = repoRoot;
  const parts = path.split("/");
  for (let index = 0; index < parts.length; index++) {
    current = join(current, parts[index]!);
    const state = lstatSync(current);
    assert.ok(!state.isSymbolicLink() && (index === parts.length - 1 ? state.isFile() : state.isDirectory()), path);
  }
  return readFileSync(current);
}

/** 🌳️ Recomputes the existing source identity with independent canonical JSON serialization. */
function sourceDigest(entries: any[]): string {
  return sha(stringify(entries.map(({ sourcePath, nodeKind, contentHash, mode, size, symlinkTarget }) => ({ sourcePath, nodeKind, contentHash, mode, size, symlinkTarget }))));
}

if (process.argv[2] !== "inspect-current-source" || process.cwd() !== repoRoot) throw new Error("Use workspace Nx exec with inspect-current-source");
const { inventoryTaxonomy } = await import(join(repoRoot, library, "🧹️normalization/🟦️.ts"));
const started = performance.now(), inventory = inventoryTaxonomy({ repoRoot, scope: vector.owner, workers: 1 });
const files = inventory.entries.filter((row: any) => row.nodeKind === "file");
assert.equal(sourceDigest(inventory.entries), inventory.sourceTreeDigest);
assert.equal(files.length, vector.ownerReadiness.files);
assert.equal(inventory.entries.length, vector.ownerReadiness.nodes);
assert.deepEqual(inventory.violations, []);
for (const row of files) { const bytes = physical(row.sourcePath); assert.equal(bytes.length, row.size); assert.equal(sha(bytes), row.contentHash); assert.equal(lstatSync(join(repoRoot, row.sourcePath)).mode & 0o7777, row.mode); }
const oldBytes = execFileSync("git", ["show", sourceRevision + ":" + changedPath], { cwd: repoRoot, timeout: 10000 }), currentBytes = physical(changedPath);
const hypothetical = inventory.entries.map((row: any) => ({ ...row }));
const changed = hypothetical.find((row: any) => row.sourcePath === changedPath)!;
changed.size = oldBytes.length;
changed.contentHash = sha(oldBytes);
for (const row of hypothetical.filter((row: any) => row.nodeKind === "directory").sort((a: any, b: any) => b.sourcePath.split("/").length - a.sourcePath.split("/").length)) {
  const prefix = row.sourcePath + "/";
  row.contentHash = sha(hypothetical.filter((child: any) => dirname(child.sourcePath).replaceAll("\\", "/") === row.sourcePath).sort((a: any, b: any) => order(a.sourcePath, b.sourcePath)).map((child: any) => `${child.nodeKind}\0${child.mode ?? ""}\0${child.sourcePath.slice(prefix.length)}\0${child.contentHash}`).join("\0"));
}
const contexts = vector.ownerReadiness.contextSources.map((expected: { path: string; size: number; sha256: string }) => { const bytes = physical(expected.path); return { path: expected.path, size: bytes.length, sha256: sha(bytes), matchesReviewed: bytes.length === expected.size && sha(bytes) === expected.sha256 }; });
console.log(JSON.stringify({ event: "energy-current-source-audit", observedAt: new Date().toISOString(), milliseconds: performance.now() - started, nodes: inventory.entries.length, files: files.length, violations: inventory.violations.length, sourceBytes: files.reduce((sum: number, row: any) => sum + row.size, 0), sourceTreeDigest: inventory.sourceTreeDigest, changedPath, sourceRevision, priorSourceBytes: oldBytes.length, priorSourceSha256: sha(oldBytes), currentSourceBytes: currentBytes.length, currentSourceSha256: sha(currentBytes), byteDelta: currentBytes.length - oldBytes.length, hypotheticalOldDigest: sourceDigest(hypothetical), reviewedOldDigest: vector.ownerReadiness.sourceTreeDigest, hypotheticalMatchesReviewed: sourceDigest(hypothetical) === vector.ownerReadiness.sourceTreeDigest, contexts, writes: false, recreatedHistoricalEvidence: false }, null, 2));
