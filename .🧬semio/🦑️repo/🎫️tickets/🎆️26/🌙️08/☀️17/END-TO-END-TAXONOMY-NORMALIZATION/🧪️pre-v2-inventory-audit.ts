import { createHash } from "node:crypto";
import { mkdirSync, readFileSync, statSync, writeFileSync } from "node:fs";

//#region Model
type Json = null | boolean | number | string | Json[] | { [key: string]: Json };

type Violation = {
  code: string;
  message: string;
  path: string;
  severity: string;
};

type Entry = {
  areaId?: string;
  contentHash?: string;
  fileKind?: string | null;
  fixedContractId?: string;
  nodeKind: "directory" | "file" | "symlink";
  normalizedPath: string;
  ownerId?: string;
  packageRole?: string;
  sourcePath: string;
  violations: Violation[];
  [key: string]: unknown;
};

type Inventory = {
  activePathExclusions: string[];
  entries: Entry[];
  inventoryDigest: string;
  pathExclusions: string[];
  schemaVersion: number;
  sourceTreeDigest: string;
  taxonomyPath: string;
  taxonomySchemaVersion: number;
  violations: Violation[];
};
//#endregion Model

//#region Canonical
function canonicalValue(value: unknown): Json | undefined {
  if (value === null || typeof value === "boolean" || typeof value === "number" || typeof value === "string") return value;
  if (Array.isArray(value)) return value.map((item) => canonicalValue(item) ?? null);
  if (typeof value !== "object" || value === undefined) return undefined;
  const result: Record<string, Json> = {};
  for (const key of Object.keys(value).sort()) {
    const child = canonicalValue((value as Record<string, unknown>)[key]);
    if (child !== undefined) result[key] = child;
  }
  return result;
}

function canonicalJson(value: unknown): string {
  return JSON.stringify(canonicalValue(value));
}

function sha256(value: string | Uint8Array): string {
  return createHash("sha256").update(value).digest("hex");
}

function byteCompare(left: string, right: string): number {
  return Buffer.compare(Buffer.from(left), Buffer.from(right));
}
//#endregion Canonical

//#region Aggregation
function increment(target: Map<string, number>, key: string, count = 1): void {
  target.set(key, (target.get(key) ?? 0) + count);
}

function sortedCounts(target: Map<string, number>): { key: string; count: number }[] {
  return [...target].map(([key, count]) => ({ key, count })).sort((left, right) => right.count - left.count || byteCompare(left.key, right.key));
}

function violationKey(violation: Violation): string {
  return canonicalJson(violation);
}

function languageOf(path: string): string {
  return path.match(/(?:^|\/)\ud83d\udce6\ufe0fpackages\/([^/]+)(?:\/|$)/u)?.[1] ?? "not-package-path";
}

function collisionKey(path: string, comparison: string): string {
  if (comparison === "byte" || comparison === "same-kind") return path;
  if (comparison === "nfc") return path.normalize("NFC");
  if (comparison === "case-fold") return path.normalize("NFC").toLocaleLowerCase("und");
  return path.normalize("NFC").replaceAll("\uFE0F", "").toLocaleLowerCase("und");
}

function collisionGroups(entries: Entry[]): Record<string, unknown>[] {
  const groups: Record<string, unknown>[] = [];
  for (const comparison of ["byte", "nfc", "case-fold", "vs16-fold", "same-kind"]) {
    const keyed = new Map<string, Entry[]>();
    for (const entry of entries) {
      const normalized = collisionKey(entry.normalizedPath, comparison);
      const key = comparison === "same-kind" ? `${entry.nodeKind}\0${entry.fileKind ?? "fixed"}\0${normalized}` : normalized;
      const rows = keyed.get(key) ?? [];
      rows.push(entry);
      keyed.set(key, rows);
    }
    for (const [key, rows] of keyed) {
      if (rows.length < 2) continue;
      const sources = rows.map((row) => row.sourcePath).sort();
      groups.push({
        comparison,
        id: sha256(`${comparison}\0${key}\0${sources.join("\0")}`).slice(0, 24),
        paths: [...new Set(rows.map((row) => row.normalizedPath))].sort(),
        sources,
      });
    }
  }
  return groups.sort((left, right) => String(left.comparison).localeCompare(String(right.comparison)) || String(left.id).localeCompare(String(right.id)));
}
//#endregion Aggregation

//#region Sharding
function shardProjection(entries: Entry[]): Record<string, unknown> {
  const byteLimitExclusive = 5 * 1024 * 1024;
  const owners = new Map<string, Entry[]>();
  for (const entry of entries) {
    const ownerId = entry.ownerId ?? "(absent)";
    const rows = owners.get(ownerId) ?? [];
    rows.push(entry);
    owners.set(ownerId, rows);
  }
  const shards: Record<string, unknown>[] = [];
  let maximumEntryBytes = 0;
  let maximumShardBytes = 0;
  let ordinal = 0;
  for (const [ownerId, rows] of [...owners].sort(([left], [right]) => byteCompare(left, right))) {
    rows.sort((left, right) => byteCompare(left.sourcePath, right.sourcePath));
    const prefixBytes = Buffer.byteLength(`{"entries":[`);
    const suffixBytes = Buffer.byteLength(`],"ownerId":${JSON.stringify(ownerId)},"schemaVersion":1}`);
    let part = 0;
    let shardRows: { entry: Entry; json: string; bytes: number }[] = [];
    let shardBytes = prefixBytes + suffixBytes;
    const flush = (): void => {
      if (shardRows.length === 0) return;
      const path = `\ud83d\udcca\ufe0finventory-shards/${String(ordinal).padStart(4, "0")}-${String(part).padStart(4, "0")}/\ud83d\udd23\ufe0f.json`;
      const payload = `{"entries":[${shardRows.map((row) => row.json).join(",")}],"ownerId":${JSON.stringify(ownerId)},"schemaVersion":1}`;
      const payloadBytes = Buffer.byteLength(payload);
      maximumShardBytes = Math.max(maximumShardBytes, payloadBytes);
      shards.push({
        path,
        ownerId,
        part,
        entries: shardRows.length,
        firstSourcePath: shardRows[0]!.entry.sourcePath,
        lastSourcePath: shardRows.at(-1)!.entry.sourcePath,
        bytes: payloadBytes,
        digest: sha256(payload),
      });
      part += 1;
      shardRows = [];
      shardBytes = prefixBytes + suffixBytes;
    };
    for (const entry of rows) {
      const json = canonicalJson(entry);
      const bytes = Buffer.byteLength(json);
      maximumEntryBytes = Math.max(maximumEntryBytes, bytes);
      const separatorBytes = shardRows.length === 0 ? 0 : 1;
      if (shardRows.length > 0 && shardBytes + separatorBytes + bytes >= byteLimitExclusive) flush();
      if (shardBytes + bytes >= byteLimitExclusive) throw new Error(`Entry exceeds shard limit: ${entry.sourcePath}`);
      shardRows.push({ entry, json, bytes });
      shardBytes += (shardRows.length === 1 ? 0 : 1) + bytes;
    }
    flush();
    ordinal += 1;
  }
  const entryCount = shards.reduce((sum, shard) => sum + Number(shard.entries), 0);
  return {
    status: "proposal-only-no-snapshot-rewrite",
    schemaVersion: 1,
    ordering: "ownerId UTF-8 byte order, then sourcePath UTF-8 byte order",
    pathRule: "ordinal-owner-shard paths declared by the manifest; ordinal derives from byte-sorted ownerId",
    byteLimitExclusive,
    shardCount: shards.length,
    ownerCount: owners.size,
    entryCount,
    duplicateEntryCount: entries.length - new Set(entries.map((entry) => entry.sourcePath)).size,
    maximumEntryBytes,
    maximumShardBytes,
    shardsDigest: sha256(canonicalJson(shards)),
    shards,
  };
}
//#endregion Sharding

//#region Audit
const ticketRoot = "/Users/ueli/Documents/semio/.\ud83e\uddecsemio/\ud83e\udd91\ufe0frepo/\ud83c\udfab\ufe0ftickets/\ud83c\udf86\ufe0f26/\ud83c\udf19\ufe0f08/\u2600\ufe0f17/END-TO-END-TAXONOMY-NORMALIZATION";
const inventoryPath = `${ticketRoot}/\ud83d\udcca\ufe0ftaxonomy-inventory/\ud83d\udd23\ufe0f.json`;
const outputDirectory = `${ticketRoot}/\ud83d\udcca\ufe0fpre-transaction-v2-current-inventory`;
const outputPath = `${outputDirectory}/\ud83d\udd23\ufe0f.json`;
const inventoryBytes = readFileSync(inventoryPath);
const inventory = JSON.parse(inventoryBytes.toString("utf8")) as Inventory;
const taxonomyBytes = readFileSync(inventory.taxonomyPath);
const currentTaxonomy = JSON.parse(taxonomyBytes.toString("utf8")) as Record<string, unknown>;
const snapshotTaxonomy = structuredClone(currentTaxonomy);
delete (snapshotTaxonomy.pathExclusions as Record<string, unknown>)["temp-compose"];
(snapshotTaxonomy.areaEnforcement as Record<string, unknown>).opaquePathExclusionIds = ["compose"];
snapshotTaxonomy._areaStateComment = "\ud83d\uddfa\ufe0f Every declared and undeclared non-opaque area is clean-enforced. compose is represented only by pathExclusions.compose.";

const nodeKinds = new Map<string, number>();
const fileKinds = new Map<string, number>();
const entryAreas = new Map<string, number>();
const entryOwners = new Map<string, number>();
const packageRoles = new Map<string, number>();
const packageRolesByLanguage = new Map<string, number>();
const fixedContracts = new Map<string, number>();
const violationCodes = new Map<string, number>();
const violationSeverities = new Map<string, number>();
const violationAreas = new Map<string, number>();
const violationOwners = new Map<string, number>();
const flattenedViolationMultiset = new Map<string, number>();
const topViolationMultiset = new Map<string, number>();
const entriesBySource = new Map<string, Entry>();
let missingMode = 0;
let missingSize = 0;
let missingRawSymlinkTarget = 0;

for (const entry of inventory.entries) {
  entriesBySource.set(entry.sourcePath, entry);
  increment(nodeKinds, entry.nodeKind);
  increment(fileKinds, entry.fileKind ?? "(none/fixed)");
  increment(entryAreas, entry.areaId ?? "(absent)");
  increment(entryOwners, entry.ownerId ?? "(absent)");
  increment(packageRoles, entry.packageRole ?? "(absent)");
  increment(packageRolesByLanguage, `${languageOf(entry.sourcePath)}\0${entry.packageRole ?? "(absent)"}`);
  if (entry.fixedContractId) increment(fixedContracts, entry.fixedContractId);
  if (!("mode" in entry)) missingMode += 1;
  if (!("size" in entry)) missingSize += 1;
  if (entry.nodeKind === "symlink" && !("symlinkTarget" in entry)) missingRawSymlinkTarget += 1;
  for (const violation of entry.violations) {
    increment(violationCodes, violation.code);
    increment(violationSeverities, violation.severity);
    increment(violationAreas, entry.areaId ?? "(absent)");
    increment(violationOwners, entry.ownerId ?? "(absent)");
    increment(flattenedViolationMultiset, violationKey(violation));
  }
}
for (const violation of inventory.violations) increment(topViolationMultiset, violationKey(violation));

const multisetKeys = new Set([...flattenedViolationMultiset.keys(), ...topViolationMultiset.keys()]);
const violationMultisetDelta = [...multisetKeys].reduce((sum, key) => sum + Math.abs((flattenedViolationMultiset.get(key) ?? 0) - (topViolationMultiset.get(key) ?? 0)), 0);
const taxonomyRejections = currentTaxonomy.fixedFilenameRejectionContracts as Record<string, { sourcePathIdentities: string[]; disposition: string; reason: string }>;
const rejectionOperations = Object.entries(taxonomyRejections).flatMap(([contractId, contract]) => contract.sourcePathIdentities.map((sourcePath) => {
  const entry = entriesBySource.get(sourcePath);
  return {
    contractId,
    disposition: contract.disposition,
    reason: contract.reason,
    sourcePath,
    admittedCount: entry ? 1 : 0,
    normalizedPath: entry?.normalizedPath ?? null,
    violationCodes: entry?.violations.map((violation) => violation.code).sort() ?? [],
  };
})).sort((left, right) => byteCompare(left.sourcePath, right.sourcePath));

const pathBudgetEntries = inventory.entries.filter((entry) => entry.violations.some((violation) => violation.code === "path-too-long"));
const pathBudgetByArea = new Map<string, number>();
const pathBudgetByOwner = new Map<string, number>();
for (const entry of pathBudgetEntries) {
  increment(pathBudgetByArea, entry.areaId ?? "(absent)");
  increment(pathBudgetByOwner, entry.ownerId ?? "(absent)");
}
const collisions = collisionGroups(inventory.entries);
const collisionsByComparison = new Map<string, number>();
for (const group of collisions) increment(collisionsByComparison, String(group.comparison));
const collisionSourceMemberships = collisions.reduce((sum, group) => sum + (group.sources as string[]).length, 0);
const topCollisions = [...collisions].sort((left, right) => (right.sources as string[]).length - (left.sources as string[]).length || byteCompare(String(left.id), String(right.id))).slice(0, 25);
const shardPlan = shardProjection(inventory.entries);

const sourceTreeDigestRecomputed = sha256(canonicalJson(inventory.entries.map((entry) => ({ sourcePath: entry.sourcePath, nodeKind: entry.nodeKind, contentHash: entry.contentHash }))));
const inventoryDigestRecomputed = sha256(canonicalJson({
  schemaVersion: inventory.schemaVersion,
  taxonomySchemaVersion: inventory.taxonomySchemaVersion,
  pathExclusions: inventory.pathExclusions,
  activePathExclusions: inventory.activePathExclusions,
  entries: inventory.entries,
  violations: inventory.violations,
  sourceTreeDigest: inventory.sourceTreeDigest,
}));
const violationCount = inventory.violations.length;
const result = {
  artifactStatus: "pre-transaction-v2-current-tree-residual-snapshot",
  acceptanceStatus: "not-final-v2-acceptance",
  command: "bun ./\ud83d\udcdc\ufe0fscript.ts clean taxonomy inventory --ticket 26/08/17/END-TO-END-TAXONOMY-NORMALIZATION",
  observedWallDuration: "00:16:38",
  artifacts: {
    inventoryPath,
    inventoryBytes: statSync(inventoryPath).size,
    inventoryArtifactSha256: sha256(inventoryBytes),
    taxonomyPath: inventory.taxonomyPath,
    taxonomyCurrentBytes: statSync(inventory.taxonomyPath).size,
    taxonomyCurrentFileSha256: sha256(taxonomyBytes),
    taxonomyCanonicalDigestDerivedAtSnapshot: sha256(canonicalJson(snapshotTaxonomy)),
    taxonomyCanonicalDigestCurrentPostGuard: sha256(canonicalJson(currentTaxonomy)),
    taxonomySnapshotDerivation: "Current taxonomy minus the exact post-census temp-compose exclusion delta; no other post-census taxonomy mutation is assumed.",
  },
  digests: {
    sourceTreeDigest: inventory.sourceTreeDigest,
    sourceTreeDigestRecomputed,
    sourceTreeDigestMatches: sourceTreeDigestRecomputed === inventory.sourceTreeDigest,
    inventoryDigest: inventory.inventoryDigest,
    inventoryDigestRecomputed,
    inventoryDigestMatches: inventoryDigestRecomputed === inventory.inventoryDigest,
  },
  schema: {
    inventorySchemaVersion: inventory.schemaVersion,
    taxonomySchemaVersion: inventory.taxonomySchemaVersion,
    missingMode,
    missingSize,
    symlinksMissingRawTarget: missingRawSymlinkTarget,
  },
  exclusions: {
    snapshotPathExclusions: inventory.pathExclusions,
    snapshotActivePathExclusions: inventory.activePathExclusions,
    forbiddenSourcePrefixes: ["compose", "temp/compose"],
    forbiddenSourceEntryCounts: {
      compose: inventory.entries.filter((entry) => entry.sourcePath === "compose" || entry.sourcePath.startsWith("compose/")).length,
      tempCompose: inventory.entries.filter((entry) => entry.sourcePath === "temp/compose" || entry.sourcePath.startsWith("temp/compose/")).length,
    },
    forbiddenNormalizedEntryCounts: {
      compose: inventory.entries.filter((entry) => entry.normalizedPath === "compose" || entry.normalizedPath.startsWith("compose/")).length,
      tempCompose: inventory.entries.filter((entry) => entry.normalizedPath === "temp/compose" || entry.normalizedPath.startsWith("temp/compose/")).length,
    },
    postSnapshotSchemaGuard: ["compose/", "temp/compose/"],
  },
  counts: {
    entries: inventory.entries.length,
    nodeKinds: sortedCounts(nodeKinds),
    fileKinds: sortedCounts(fileKinds),
    areas: sortedCounts(entryAreas),
    owners: sortedCounts(entryOwners),
    violations: violationCount,
    violationCodes: sortedCounts(violationCodes),
    violationSeverities: sortedCounts(violationSeverities),
    violationsByArea: sortedCounts(violationAreas),
    violationsByOwner: sortedCounts(violationOwners),
    fixedContracts: sortedCounts(fixedContracts),
  },
  packages: {
    roles: sortedCounts(packageRoles),
    rolesByLanguage: sortedCounts(packageRolesByLanguage).map(({ key, count }) => {
      const [language, role] = key.split("\0");
      return { language, role, count };
    }),
    sourceDispositions: currentTaxonomy.packageSourceDispositions,
  },
  fixedRejections: {
    contracts: Object.keys(taxonomyRejections).length,
    identities: rejectionOperations.length,
    admittedOnce: rejectionOperations.filter((operation) => operation.admittedCount === 1).length,
    missing: rejectionOperations.filter((operation) => operation.admittedCount === 0).length,
    byDisposition: sortedCounts(rejectionOperations.reduce((counts, operation) => {
      increment(counts, operation.disposition);
      return counts;
    }, new Map<string, number>())),
    operations: rejectionOperations,
  },
  pathBudget: {
    configuredMaximumBytes: Number((currentTaxonomy.collisionPolicy as Record<string, unknown>).maxPathBytes),
    offendingEntries: pathBudgetEntries.length,
    maximumSourcePathBytes: Math.max(...pathBudgetEntries.map((entry) => Buffer.byteLength(entry.sourcePath))),
    maximumNormalizedPathBytes: Math.max(...pathBudgetEntries.map((entry) => Buffer.byteLength(entry.normalizedPath))),
    byArea: sortedCounts(pathBudgetByArea),
    byOwner: sortedCounts(pathBudgetByOwner),
    windowsReservedViolations: violationCodes.get("windows-reserved-name") ?? 0,
    trailingDotOrSpaceViolations: violationCodes.get("trailing-dot-or-space") ?? 0,
  },
  collisions: {
    algorithm: "planner-equivalent byte/nfc/case-fold/vs16-fold/same-kind grouping",
    groups: collisions.length,
    byComparison: sortedCounts(collisionsByComparison),
    sourceMemberships: collisionSourceMemberships,
    uniqueSourceMemberships: new Set(collisions.flatMap((group) => group.sources as string[])).size,
    ledgerDigest: sha256(canonicalJson(collisions)),
    topGroups: topCollisions,
    allGroups: collisions,
  },
  zeroDoubleCount: {
    uniqueSourcePaths: new Set(inventory.entries.map((entry) => entry.sourcePath)).size,
    duplicateSourcePaths: inventory.entries.length - new Set(inventory.entries.map((entry) => entry.sourcePath)).size,
    nodeKindPartition: [...nodeKinds.values()].reduce((sum, count) => sum + count, 0),
    packageRolePartition: [...packageRoles.values()].reduce((sum, count) => sum + count, 0),
    topLevelViolationCount: inventory.violations.length,
    flattenedEntryViolationCount: [...violationCodes.values()].reduce((sum, count) => sum + count, 0),
    violationMultisetDelta,
    violationCodePartition: [...violationCodes.values()].reduce((sum, count) => sum + count, 0),
    violationAreaPartition: [...violationAreas.values()].reduce((sum, count) => sum + count, 0),
    violationOwnerPartition: [...violationOwners.values()].reduce((sum, count) => sum + count, 0),
  },
  deterministicShardingProposal: shardPlan,
  priorDigestComparison: {
    priorCanonicalSourceTreeDigest: "e8504fdfe1cb218b37d6abafadde51469c0d128db427db4ac05e22453ac89bc8",
    currentSourceTreeDigest: inventory.sourceTreeDigest,
    equal: inventory.sourceTreeDigest === "e8504fdfe1cb218b37d6abafadde51469c0d128db427db4ac05e22453ac89bc8",
    interpretation: "Digests differ; counts are not merged and no continuity claim is made.",
  },
};

mkdirSync(outputDirectory, { recursive: true });
writeFileSync(outputPath, `${canonicalJson(result)}\n`);
console.log(canonicalJson({ outputPath, bytes: statSync(outputPath).size, inventoryDigestMatches: result.digests.inventoryDigestMatches, sourceTreeDigestMatches: result.digests.sourceTreeDigestMatches, collisionGroups: result.collisions.groups, shardCount: (shardPlan.shardCount as number) }));
//#endregion Audit
