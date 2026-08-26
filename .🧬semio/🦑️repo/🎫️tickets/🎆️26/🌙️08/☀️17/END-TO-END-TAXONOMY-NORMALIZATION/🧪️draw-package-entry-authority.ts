//#region 🧪️Authority
import { strict as assert } from "node:assert";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";

const repoRoot = process.cwd();
const discoveryPath = join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts");
const goldenPath = join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️cad-draw-path-projection/🔣️.json");
const { loadTaxonomy, semanticPathProjectionAuthority, semanticPathProjectionReferenceConsumers } = await import(pathToFileURL(discoveryPath).href);
const golden = JSON.parse(readFileSync(goldenPath, "utf8"));
const taxonomy = loadTaxonomy();
let resolvedConsumers = 0;
for (const consumer of golden.referenceConsumers) for (const sourcePath of consumer.sourcePathIdentities) {
  const matches = semanticPathProjectionReferenceConsumers(consumer.projectionContractId, sourcePath, consumer.adapter, consumer.form, taxonomy);
  assert.deepEqual(matches.map(({ id }: { id: string }) => id), [consumer.id]);
  assert(matches[0].contract.staleMarkers.includes(consumer.staleMarker));
  const absolute = join(repoRoot, sourcePath);
  if (existsSync(absolute)) assert(readFileSync(absolute, "utf8").includes(consumer.staleMarker));
  resolvedConsumers += 1;
}
assert.deepEqual(semanticPathProjectionReferenceConsumers("artifact-example-model-catalog-v1", "prefix/✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️component.ts", "typescript", "artifact-catalog-prose:root-marker", taxonomy), []);
const broadened = structuredClone(taxonomy);
broadened.semanticPathProjectionReferenceConsumerContracts["cad-spatial-kernel-geometry"].sourcePathPattern = "^.*🟦️(?:component)?\\.ts$";
assert.deepEqual(semanticPathProjectionReferenceConsumers("artifact-example-model-catalog-v1", "counterfeit/🟦️component.ts", "typescript", "artifact-catalog-prose:root-marker", broadened), []);
const projection = golden.projections.find(({ contractId }: { contractId: string }) => contractId === "artifact-editor-command-bundle-v1");
assert(projection);
const directories = new Set<string>([projection.sourceRoot]);
const files = projection.mappings.map(({ sourcePath }: { sourcePath: string }) => {
  let parent = sourcePath.slice(0, sourcePath.lastIndexOf("/"));
  while (parent.length >= projection.sourceRoot.length) {
    directories.add(parent);
    if (parent === projection.sourceRoot) break;
    parent = parent.slice(0, parent.lastIndexOf("/"));
  }
  return { path: sourcePath, nodeKind: "file" as const, content: readFileSync(join(repoRoot, sourcePath), "utf8") };
});
const nodes = [...[...directories].map((path) => ({ path, nodeKind: "directory" as const })), ...files];
const result = semanticPathProjectionAuthority({
  artifactRoot: projection.sourceRoot.slice(0, projection.sourceRoot.indexOf("/🏅️standards/")),
  contractId: projection.contractId,
  nodes,
  sourceRoot: projection.sourceRoot,
});
assert.deepEqual(result.problems, []);
assert.deepEqual(result.mappings, projection.mappings);
assert.deepEqual(result.referenceEdits, projection.referenceEdits);
assert.equal(result.destinationDirectoryCount, projection.destinationDirectoryCount);
assert.equal(result.destinationNodeCount, projection.destinationNodeCount);
assert.equal(result.mappingDigest, projection.mappingDigest);
assert.equal(result.maxPathBytes, projection.maxPathBytes);
console.log(JSON.stringify({
  destinationDirectoryCount: result.destinationDirectoryCount,
  destinationNodeCount: result.destinationNodeCount,
  mappingDigest: result.mappingDigest,
  maxPathBytes: result.maxPathBytes,
  referenceEdits: result.referenceEdits.length,
  resolvedConsumerIdentities: resolvedConsumers,
  sourceFileCount: result.mappings.length,
}));
//#endregion 🧪️Authority
