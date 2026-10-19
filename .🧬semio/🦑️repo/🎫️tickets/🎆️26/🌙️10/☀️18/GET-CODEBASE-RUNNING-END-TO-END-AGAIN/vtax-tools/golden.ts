// derives the golden corruption vectors from the PRE-split validateTaxonomy and checks the decomposed one against them. usage: bun golden.ts [WRITE <fixture.json>]
import { readFileSync, writeFileSync } from "node:fs";
import { validateTaxonomy as validateNew } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { validateTaxonomy as validateOld } from "./orig/taxonomy-authority.abs.ts";
const baseText = JSON.stringify(JSON.parse(readFileSync("/Users/ueli/.cache/vtax/taxonomy.base.json", "utf8")));
type Row = { group: string; path: (string | number)[]; value: unknown };
const rows: Row[] = [
  { group: "auditSchemaVersionAndEvidence", path: ["schemaVersion"], value: 6 },
  { group: "auditLeafAndReferencePolicies", path: ["physicalLeafRendering"], value: {} },
  { group: "auditFileKinds", path: ["fileKinds", "json", "emoji"], value: "x" },
  { group: "auditScopedAndDirectoryKinds", path: ["scopedFileKinds"], value: [] },
  { group: "auditTransactionArtifacts", path: ["scopedFileKinds", "transaction-edit-candidate", "pathPattern"], value: "**/x" },
  { group: "auditMemberAndDescendantContracts", path: ["semanticPathProjectionProfileRenderers"], value: 1 },
  { group: "auditProjectionAndPackageContracts", path: ["semanticPolicyStateCoordinateContracts"], value: 1 },
  { group: "auditOwnedFileProjection", path: ["semanticOwnedFileProjectionContracts"], value: {} },
  { group: "auditReferenceConsumersAndMutationCatalog", path: ["semanticPathProjectionReferenceConsumerContracts"], value: 1 },
  { group: "auditFixedContracts", path: ["fixedFilenameRejectionContracts"], value: 1 },
  { group: "auditPackageBoundaryContracts", path: ["packageSourceDispositions"], value: 1 },
  { group: "auditGeneratorContracts", path: ["generatorContracts"], value: {} },
  { group: "auditPathPoliciesAndAreas", path: ["unicodeNormalization"], value: {} },
  { group: "auditDependencyDirections", path: ["areaLayers"], value: {} },
  { group: "auditEcosystemsAndTargets", path: ["targets", "__probe__"], value: {} },
  { group: "auditTestVocabulary", path: ["testExamplesDirName"], value: "x" },
  { group: "auditManifestAndRootContracts", path: ["rootDataContractIds"], value: ["missing"] },
  { group: "auditSchemaFormatsAndFacets", path: ["schemaJsonDialect"], value: "x" },
  { group: "auditMutationVocabulary", path: ["mutationComponentFileKindId"], value: "x" },
  { group: "auditRepresentationAndChildDirectories", path: ["nativeRepresentationDirs"], value: [] },
  { group: "auditPackageBoundaryContracts", path: ["configurableEntryContracts"], value: 1 },
];
const apply = (path: (string | number)[], value: unknown): Record<string, any> => {
  const clone = JSON.parse(baseText);
  let cursor = clone;
  for (const key of path.slice(0, -1)) cursor = cursor[key];
  cursor[path[path.length - 1]!] = value;
  return clone;
};
const run = (validate: (t: never) => string[], row: Row): string[] | string => { try { return validate(apply(row.path, row.value) as never); } catch (error) { return `THROWS ${(error as Error).message}`; } };
const baseline = [JSON.stringify(validateOld(JSON.parse(baseText))), JSON.stringify(validateNew(JSON.parse(baseText)))];
console.log("baseline old/new", baseline);
const out = rows.map((row) => {
  const old = run(validateOld as never, row), now = run(validateNew as never, row);
  const same = JSON.stringify(old) === JSON.stringify(now);
  console.log(row.group.padEnd(44), same ? "same" : "DIFFERENT", typeof old === "string" ? old : `${old.length} problem(s): ${old[0]?.slice(0, 110)}`);
  return { ...row, expected: old };
});
if (process.argv[2] === "WRITE") writeFileSync(process.argv[3]!, JSON.stringify(out, null, 2) + "\n");
