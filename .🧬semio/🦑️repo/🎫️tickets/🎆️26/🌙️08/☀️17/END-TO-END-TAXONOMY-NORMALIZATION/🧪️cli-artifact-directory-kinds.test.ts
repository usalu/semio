import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import picomatch from "picomatch";
import {
  loadTaxonomy,
  scopedFileKindIdForSourcePath,
  semanticDirectoryKindId,
  taxonomyCliAttemptPreparationsProblems,
  taxonomyCliBackupPreparationProblems,
  taxonomyCliBackupWritePreparationProblems,
  taxonomyCliEditPreparationProblems,
  taxonomyCliEditWritePreparationProblems,
  taxonomyCliJsonWritePreparationProblems,
  taxonomyCliLeaseDirectoryProblems,
  taxonomyCliRestorePreparationProblems,
  validateTaxonomy,
} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts";

//#region 🧪️TaxonomyCliArtifacts
const roots = [
  ["📊️taxonomy-inventory", "taxonomy-inventory-data"],
  ["📊️taxonomy-plan", "taxonomy-plan-data"],
  ["📊️taxonomy-apply", "taxonomy-apply-data"],
  ["📊️taxonomy-verification", "taxonomy-verification-data"],
  ["📓️taxonomy-inventory", "taxonomy-inventory-summary"],
  ["📓️taxonomy-plan", "taxonomy-plan-summary"],
  ["📓️taxonomy-apply", "taxonomy-apply-summary"],
  ["📓️taxonomy-verification", "taxonomy-verification-summary"],
] as const;
const ticketRoot = fileURLToPath(new URL(".", import.meta.url));
const attemptVectors = JSON.parse(readFileSync(join(ticketRoot, "🧪️transaction-attempt-authority", "🔣️.json"), "utf8")) as {
  schemaVersion: number;
  vectors: { name: string; parentKindId: string; expectedKindId: string | null; thirdPartyCandidate: boolean }[];
  attemptPreparations: { parentKindId: string; directoryName: string; children: { name: string; nodeKind: "directory" | "file" }[]; valid: boolean }[];
  attemptPreparationCollections: { preparationIndexes: number[]; valid: boolean }[];
  editPreparations: { parentKindId: string; directoryName: string; leafNames: string[]; writePreparations: { directoryName: string; leafNames: string[] }[]; valid: boolean }[];
  backupPreparations: { parentKindId: string; directoryName: string; leafNames: string[]; writePreparations: { directoryName: string; leafNames: string[] }[]; valid: boolean }[];
  editWritePreparations: { parentKindId: string; leafNames: string[]; valid: boolean }[];
  backupWritePreparations: { parentKindId: string; leafNames: string[]; valid: boolean }[];
  leaseDirectories: { parentKindId: string; directoryName: string; leafNames: string[]; writePreparations: { directoryName: string; leafNames: string[] }[]; valid: boolean }[];
  restorePreparations: { leafNames: string[]; valid: boolean }[];
  jsonWritePreparations: { parentKindId: string; leafNames: string[]; valid: boolean }[];
};

describe("taxonomy CLI artifact directory authority", () => {
  const taxonomy = loadTaxonomy();

  test("keeps strict taxonomy valid", () => {
    expect(validateTaxonomy(taxonomy)).toEqual([]);
  });

  test("resolves all eight permanent operation roots exactly", () => {
    for (const [name, id] of roots) expect(semanticDirectoryKindId(name, taxonomy)).toBe(id);
    expect(semanticDirectoryKindId("📊️taxonomy-inventory-extra", taxonomy)).toBeNull();
    expect(semanticDirectoryKindId("📓️taxonomy-verify", taxonomy)).toBeNull();
  });

  test("scopes shard, digest and transaction-attempt children to their exact parents", () => {
    const digest = `🔖️${"a".repeat(64)}`;
    expect(semanticDirectoryKindId("📊️shards", taxonomy)).toBeNull();
    expect(semanticDirectoryKindId("📊️shards", taxonomy, { parentKindId: "taxonomy-inventory-data" })).toBe("taxonomy-inventory-shards");
    expect(semanticDirectoryKindId(digest, taxonomy, { parentKindId: "taxonomy-inventory-shards" })).toBe("taxonomy-inventory-shard-digest");
    expect(semanticDirectoryKindId(digest, taxonomy, { parentKindId: "taxonomy-transaction" })).toBe("transaction-digest");
    expect(semanticDirectoryKindId(digest, taxonomy, { parentKindId: "taxonomy-plan-data" })).not.toBe("taxonomy-inventory-shard-digest");
    expect(semanticDirectoryKindId(`🔖️${"g".repeat(64)}`, taxonomy, { parentKindId: "taxonomy-inventory-shards" })).not.toBe("taxonomy-inventory-shard-digest");
    expect(semanticDirectoryKindId("🔂️attempts", taxonomy, { parentKindId: "transaction-digest" })).toBe("transaction-attempts");
    expect(semanticDirectoryKindId("🔢️000001", taxonomy, { parentKindId: "transaction-attempts" })).toBe("transaction-attempt");
    expect(semanticDirectoryKindId("🔂️attempts", taxonomy, { parentKindId: "taxonomy-inventory-data" })).toBeNull();
    expect(semanticDirectoryKindId("🔢️000001", taxonomy, { parentKindId: "transaction-digest" })).toBeNull();
    expect(semanticDirectoryKindId("🔢️00001", taxonomy, { parentKindId: "transaction-attempts" })).toBeNull();
    expect(semanticDirectoryKindId("🔢️1000000", taxonomy, { parentKindId: "transaction-attempts" })).toBeNull();
  });

  test("matches the language-neutral attempt golden and third-party candidate discovery", () => {
    const uuid = "[0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f]-[0-9a-f][0-9a-f][0-9a-f][0-9a-f]-4[0-9a-f][0-9a-f][0-9a-f]-[89ab][0-9a-f][0-9a-f][0-9a-f]-[0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f]";
    const attemptsCandidate = picomatch("🔂️attempts");
    const ordinalCandidate = picomatch("🔢️[0-9][0-9][0-9][0-9][0-9][0-9]");
    const attemptChildCandidate = picomatch("{🚧️stage,💾️backup,🔒️lease}");
    const attemptPreparationCandidate = picomatch(`🚧️prepare-[0-9][0-9][0-9][0-9][0-9][0-9]-[1-9]*([0-9])-${uuid}`);
    const journalWriteCandidate = picomatch("🚧️journal");
    const leasePreparationCandidate = picomatch(`🚧️lease-[1-9]*([0-9])-${uuid}-@(preparing|stale)`);
    const backupPreparationCandidate = picomatch(`🚧️backup-${"[0-9a-f]".repeat(24)}-[1-9]*([0-9])-${uuid}`);
    const restorePreparationCandidate = picomatch(`🚧️restore-${"[0-9a-f]".repeat(24)}-[1-9]*([0-9])-${uuid}`);
    const editPreparationCandidate = picomatch(`🚧️edit-${"[0-9a-f]".repeat(24)}-[1-9]*([0-9])-${uuid}`);
    const jsonWritePreparationCandidate = picomatch(`🚧️write-[1-9]*([0-9])-${uuid}`);
    expect(attemptVectors.schemaVersion).toBe(1);
    for (const vector of attemptVectors.vectors) {
      const candidate = attemptsCandidate(vector.name) || ordinalCandidate(vector.name) || attemptChildCandidate(vector.name) || attemptPreparationCandidate(vector.name) || journalWriteCandidate(vector.name) || leasePreparationCandidate(vector.name) || backupPreparationCandidate(vector.name) || restorePreparationCandidate(vector.name) || editPreparationCandidate(vector.name) || jsonWritePreparationCandidate(vector.name);
      expect(candidate).toBe(vector.thirdPartyCandidate);
      expect(semanticDirectoryKindId(vector.name, taxonomy, { parentKindId: vector.parentKindId })).toBe(vector.expectedKindId);
    }
    for (const vector of attemptVectors.attemptPreparations) expect(taxonomyCliAttemptPreparationsProblems([vector], taxonomy).length === 0).toBe(vector.valid);
    for (const vector of attemptVectors.attemptPreparationCollections) expect(taxonomyCliAttemptPreparationsProblems(vector.preparationIndexes.map((index) => attemptVectors.attemptPreparations[index]!), taxonomy).length === 0).toBe(vector.valid);
    const duplicateOrdinal = [attemptVectors.attemptPreparations[0]!, attemptVectors.attemptPreparations[5]!];
    expect(taxonomyCliAttemptPreparationsProblems(duplicateOrdinal, taxonomy)).toEqual(taxonomyCliAttemptPreparationsProblems([...duplicateOrdinal].reverse(), taxonomy));
    for (const vector of attemptVectors.editPreparations) expect(taxonomyCliEditPreparationProblems(vector, taxonomy).length === 0).toBe(vector.valid);
    for (const vector of attemptVectors.backupPreparations) expect(taxonomyCliBackupPreparationProblems(vector, taxonomy).length === 0).toBe(vector.valid);
    const restoreDirectory = "🚧️restore-0123456789abcdef01234567-42-123e4567-e89b-42d3-a456-426614174000";
    for (const vector of attemptVectors.restorePreparations) expect(taxonomyCliRestorePreparationProblems({ parentKindId: "transaction-backup", directoryName: restoreDirectory, leafNames: vector.leafNames }, taxonomy).length === 0).toBe(vector.valid);
    const writeDirectory = "🚧️write-42-123e4567-e89b-42d3-a456-426614174000";
    for (const vector of attemptVectors.editWritePreparations) expect(taxonomyCliEditWritePreparationProblems({ parentKindId: vector.parentKindId, directoryName: writeDirectory, leafNames: vector.leafNames }, taxonomy).length === 0).toBe(vector.valid);
    for (const vector of attemptVectors.backupWritePreparations) expect(taxonomyCliBackupWritePreparationProblems({ parentKindId: vector.parentKindId, directoryName: writeDirectory, leafNames: vector.leafNames }, taxonomy).length === 0).toBe(vector.valid);
    for (const vector of attemptVectors.leaseDirectories) expect(taxonomyCliLeaseDirectoryProblems(vector, taxonomy).length === 0).toBe(vector.valid);
    const scopedWriters = [
      ["transaction-edit-write-candidate", `root/🚧️stage/🚧️edit-owner/${writeDirectory}/🚧️.edit`, "transaction-edit-write-preparation"],
      ["transaction-backup-write-candidate", `root/💾️backup/🚧️backup-owner/${writeDirectory}/🚧️.backup`, "transaction-backup-write-preparation"],
    ] as const;
    for (const [kindId, path, parentDirectoryKindId] of scopedWriters) {
      expect(picomatch(taxonomy.scopedFileKinds[kindId]!.pathPattern)(path)).toBe(true);
      expect(scopedFileKindIdForSourcePath(path, taxonomy, { parentDirectoryKindId })).toBe(kindId);
      expect(scopedFileKindIdForSourcePath(path.replace("🚧️.", "partial."), taxonomy, { parentDirectoryKindId })).toBeNull();
    }
    for (const vector of attemptVectors.jsonWritePreparations) expect(taxonomyCliJsonWritePreparationProblems({ parentKindId: vector.parentKindId, directoryName: writeDirectory, leafNames: vector.leafNames }, taxonomy).length === 0).toBe(vector.valid);
  });
});
//#endregion 🧪️TaxonomyCliArtifacts
