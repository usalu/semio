import { describe, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import fastGlob from "fast-glob";
import {
  TAXONOMY_INVENTORY_SHARD_MAX_BYTES,
  buildTaxonomyInventoryArtifactShards,
  publishTaxonomyInventoryArtifactShards,
  taxonomyCliPlanOperationConsoleFields,
  taxonomyCliPlanOperationCounts,
  taxonomyCliPlanOperationSummaryRows,
  taxonomyCliGuardedPath,
  taxonomyCliRequireCommittedApply,
  taxonomyCliValidateOperationOptions,
  taxonomyInventoryCanonicalChunks,
  taxonomyInventoryIncrementalCanonicalDigest,
  validateTaxonomyInventoryArtifactShards,
} from "../../../../../../../📜️script.ts";
import { canonicalJson } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts";

//#region 🧪️InventoryArtifactShards
const ticketRoot = import.meta.dir;
const repoRoot = resolve(import.meta.dir, "../../../../../../../");

function inventory(entries: readonly Record<string, unknown>[]): Record<string, unknown> {
  return {
    schemaVersion: 1,
    taxonomySchemaVersion: 7,
    repoRoot: "/fixture",
    taxonomyPath: "/fixture/🔣️taxonomy.json",
    sourceTreeDigest: "a".repeat(64),
    inventoryDigest: "b".repeat(64),
    pathExclusions: ["compose", "temp/compose"],
    activePathExclusions: [],
    entries,
    violations: [],
  };
}

function entry(sourcePath: string, ownerId: string, payload = ""): Record<string, unknown> {
  return { sourcePath, normalizedPath: sourcePath, nodeKind: "file", ownerId, contentHash: "c".repeat(64), mode: 0o100644, size: payload.length, payload, violations: [] };
}

async function webSha256(content: string): Promise<string> {
  const digest = await crypto.subtle.digest("SHA-256", new TextEncoder().encode(content));
  return [...new Uint8Array(digest)].map((byte) => byte.toString(16).padStart(2, "0")).join("");
}

describe("inventory artifact shards", () => {
  test("builds deterministic lossless byte-sorted owner shards", () => {
    const source = inventory([entry("a", "z-owner"), entry("b", "a-owner"), entry("c", "a-owner")]);
    const first = buildTaxonomyInventoryArtifactShards(source);
    const second = buildTaxonomyInventoryArtifactShards(structuredClone(source));
    expect(first.manifestContent).toBe(second.manifestContent);
    expect(first.shards.map((shard) => shard.content)).toEqual(second.shards.map((shard) => shard.content));
    expect(first.manifest.shards.map((shard) => shard.ownerId)).toEqual(["a-owner", "z-owner"]);
    expect(validateTaxonomyInventoryArtifactShards(first).inventory).toEqual(source);
    expect(taxonomyInventoryIncrementalCanonicalDigest(source)).toBe(createHash("sha256").update(canonicalJson(source)).digest("hex"));
    expect([...taxonomyInventoryCanonicalChunks(source)].join("")).toBe(canonicalJson(source));
    const witness = inventory([entry("a", "owner", "x".repeat(900_000)), entry("b", "owner", "y".repeat(900_000))]);
    const witnessChunks = [...taxonomyInventoryCanonicalChunks(witness)];
    expect(Math.max(...witnessChunks.map((chunk) => Buffer.byteLength(chunk)))).toBeLessThan(Buffer.byteLength(canonicalJson(witness.entries)));
  });

  test("byte-sorts the full violation identity independently of insertion order", () => {
    const error = { path: "same", code: "same", severity: "error", message: "same" };
    const warning = { path: "same", code: "same", severity: "warning", message: "same" };
    const leftEntry = { ...entry("a", "owner"), violations: [warning, error] };
    const rightEntry = { ...entry("a", "owner"), violations: [error, warning] };
    const left = { ...inventory([leftEntry]), violations: [error, warning] };
    const right = { ...inventory([rightEntry]), violations: [error, warning] };
    expect(buildTaxonomyInventoryArtifactShards(left).manifest.violationsDigest).toBe(buildTaxonomyInventoryArtifactShards(right).manifest.violationsDigest);
  });

  test("splits oversized owners and rejects one-entry overflow", () => {
    const payload = "x".repeat(2_700_000);
    const split = buildTaxonomyInventoryArtifactShards(inventory([entry("a", "owner", payload), entry("b", "owner", payload), entry("c", "owner", payload)]));
    expect(split.shards).toHaveLength(3);
    expect(split.shards.every((shard) => shard.descriptor.bytes < TAXONOMY_INVENTORY_SHARD_MAX_BYTES)).toBeTrue();
    expect(() => buildTaxonomyInventoryArtifactShards(inventory([entry("a", "owner", "x".repeat(TAXONOMY_INVENTORY_SHARD_MAX_BYTES))]))).toThrow("exceeds the shard byte limit");
  });

  test("rejects duplicate, order, digest, count and unreferenced-shard drift", () => {
    expect(() => buildTaxonomyInventoryArtifactShards(inventory([entry("a", "owner"), entry("a", "owner")]))).toThrow("Duplicate inventory sourcePath");
    const built = buildTaxonomyInventoryArtifactShards(inventory([entry("a", "z-owner"), entry("b", "a-owner")]));
    const reordered = structuredClone(built) as any;
    reordered.manifest.shards.reverse();
    expect(() => validateTaxonomyInventoryArtifactShards(reordered)).toThrow();
    const corrupted = structuredClone(built) as any;
    corrupted.shards[0]!.content = `${corrupted.shards[0]!.content} `;
    expect(() => validateTaxonomyInventoryArtifactShards(corrupted)).toThrow("byte/digest mismatch");
    const wrongCount = structuredClone(built) as any;
    wrongCount.manifest.entryCount += 1;
    wrongCount.manifestContent = `${JSON.stringify(wrongCount.manifest)}\n`;
    expect(() => validateTaxonomyInventoryArtifactShards(wrongCount)).toThrow();
    expect(() => validateTaxonomyInventoryArtifactShards(built, [...built.shards.map((shard) => shard.descriptor.path), "📊️shards/extra"])).toThrow("missing or unreferenced shards");
  });

  test("publishes atomically with third-party path parity and independent SHA parity", async () => {
    const fixture = mkdtempSync(join(ticketRoot, "🧪️inventory-shards-"));
    try {
      const dataRoot = join(fixture, "📊️taxonomy-inventory");
      mkdirSync(dataRoot, { recursive: true });
      const monolith = join(dataRoot, "🔣️.json");
      writeFileSync(monolith, "legacy-pre-v2\n");
      const progress: { phase: string; current: number; total: number }[] = [];
      const manifest = publishTaxonomyInventoryArtifactShards(dataRoot, inventory([entry("a", "z-owner"), entry("b", "a-owner")]), (event) => progress.push(event));
      const discovered = (await fastGlob("📊️shards/🔖️*/🔣️.json", { cwd: dataRoot, onlyFiles: true })).sort();
      expect(discovered).toEqual(manifest.shards.map((shard) => shard.path).sort());
      for (const shard of manifest.shards) {
        const content = readFileSync(join(dataRoot, shard.path), "utf8");
        expect(await webSha256(content)).toBe(shard.digest);
      }
      expect(readFileSync(monolith, "utf8")).toBe("legacy-pre-v2\n");
      expect(progress[0]).toEqual({ phase: "write-shards", current: 0, total: manifest.shards.length + 1 });
      expect(progress.at(-1)).toEqual({ phase: "write-shards", current: manifest.shards.length + 1, total: manifest.shards.length + 1, path: "📊️shards/🔣️.json" });
      const replacement = publishTaxonomyInventoryArtifactShards(dataRoot, inventory([entry("a", "new-owner")]));
      const replacementPaths = (await fastGlob("📊️shards/🔖️*/🔣️.json", { cwd: dataRoot, onlyFiles: true })).sort();
      expect(replacementPaths).toEqual(replacement.shards.map((shard) => shard.path).sort());
      expect(readFileSync(monolith, "utf8")).toBe("legacy-pre-v2\n");
      writeFileSync(join(dataRoot, replacement.shards[0]!.path), "corrupt\n");
      expect(() => publishTaxonomyInventoryArtifactShards(dataRoot, inventory([entry("a", "new-owner")]))).toThrow("digest collision or corruption");
    } finally {
      rmSync(fixture, { recursive: true, force: true });
    }
  });

  test("guards opaque, outside, symlink and non-directory CLI ancestors lexically", () => {
    const fixture = mkdtempSync(join(ticketRoot, "🧪️cli-path-guard-"));
    try {
      mkdirSync(join(fixture, "safe"));
      mkdirSync(join(fixture, "compose"));
      mkdirSync(join(fixture, "temp", "compose"), { recursive: true });
      writeFileSync(join(fixture, "not-directory"), "x");
      symlinkSync(join(fixture, "compose"), join(fixture, "alias"), process.platform === "win32" ? "junction" : "dir");
      expect(() => taxonomyCliGuardedPath(fixture, "compose/plan.json", "--plan")).toThrow("opaque path");
      expect(() => taxonomyCliGuardedPath(fixture, "temp/compose/journal.json", "--resume")).toThrow("opaque path");
      expect(() => taxonomyCliGuardedPath(fixture, "../outside", "--cancel-file")).toThrow("inside the repository");
      expect(() => taxonomyCliGuardedPath(fixture, "alias/plan.json", "--plan")).toThrow("cannot traverse symlink");
      expect(() => taxonomyCliGuardedPath(fixture, "not-directory/plan.json", "--plan")).toThrow("non-directory ancestor");
      expect(taxonomyCliGuardedPath(fixture, "safe/missing.json", "--cancel-file")).toBe(join(fixture, "safe", "missing.json"));
    } finally {
      rmSync(fixture, { recursive: true, force: true });
    }
  });

  test("closes operation options, reports all plan groups and fails non-committed apply states", () => {
    const common = { failOnWarning: false, format: "human" as const };
    expect(() => taxonomyCliValidateOperationOptions("inventory", { ...common, resume: "journal.json" })).toThrow("--resume is not valid");
    expect(() => taxonomyCliValidateOperationOptions("verify", { ...common, plan: "plan.json" })).toThrow("--plan is not valid");
    expect(() => taxonomyCliValidateOperationOptions("apply", { ...common, baseline: "HEAD" })).toThrow("--baseline is not valid");
    expect(() => taxonomyCliValidateOperationOptions("plan", { ...common, digest: "a".repeat(64) })).toThrow("--digest is not valid");
    expect(() => taxonomyCliValidateOperationOptions("inventory", { ...common, failOnWarning: true })).toThrow("--fail-on-warning is not valid");
    expect(() => taxonomyCliValidateOperationOptions("apply", { ...common, scope: "x" })).toThrow("--scope is not valid");
    const counts = taxonomyCliPlanOperationCounts({
      moves: [0],
      embeddedTicketRoots: [0, 1],
      embeddedTicketRootRelocations: [0, 1, 2],
      symlinkTargetEdits: [0, 1, 2, 3],
      evidenceRemovals: [0, 1, 2, 3, 4],
      edits: [0, 1, 2, 3, 4, 5],
      regenerations: [0, 1, 2, 3, 4, 5, 6],
    });
    expect(taxonomyCliPlanOperationSummaryRows(counts)).toEqual([
      "- Moves: 1",
      "- Embedded ticket roots: 2",
      "- Embedded ticket-root relocations: 3",
      "- Symlink target edits: 4",
      "- Evidence removals: 5",
      "- Edits: 6",
      "- Regenerations: 7",
    ]);
    expect(taxonomyCliPlanOperationConsoleFields(counts)).toBe("moves=1 roots=2 relocations=3 symlinks=4 removals=5 edits=6 regenerations=7");
    expect(() => taxonomyCliRequireCommittedApply("rolled-back")).toThrow("is not committed");
    expect(() => taxonomyCliRequireCommittedApply("cancelled")).toThrow("is not committed");
    expect(taxonomyCliRequireCommittedApply("committed")).toBeUndefined();
    const rootScript = join(repoRoot, "📜️script.ts");
    const closedCli = spawnSync(process.execPath, [rootScript, "clean", "taxonomy", "inventory", "--resume", "journal.json"], { cwd: repoRoot, encoding: "utf8" });
    expect(closedCli.status).not.toBe(0);
    expect(closedCli.stderr).toContain("--resume is not valid for this operation");
  }, 30_000);
});
//#endregion 🧪️InventoryArtifactShards
