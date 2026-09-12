import assert from "node:assert/strict";
import { testMeshEdgeAuthority } from "../../../../🔨️modules/🏗️fem/⚙️engine/🕸️mesh/🧪️tests/🕸️edge-authority/🟦️.ts";
import { testFemAssemblyMergeRefusalOracle } from "../../../../🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧪️tests/🔀️merge-refusal/🟦️.ts";
import { testFemAssemblyStepGrantOracle } from "../../../../🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧪️tests/⛽️step-grant/🟦️.ts";
import { testFemPagedCsrOracle } from "../../../../🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🧪️tests/📚️paged-csr/🟦️.ts";
import { testFemPcgPublicationGrantOracle } from "../../../../🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🧪️tests/⛽️publication-grant/🟦️.ts";
import { testFemScalarOwnerOracle } from "../../../../🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🧪️tests/🔢️scalar-owners/🟦️.ts";
import Ajv from "ajv";
import { applyPatch, type Operation } from "fast-json-patch";
import commandLimits from "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧫️fixtures/🚧️retained-command-limits/🔣️.json" with { type: "json" };
import commandLimitsSchema from "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🧬️schema/🚧️retained-limits/🔣️.json" with { type: "json" };
import documentAdmission from "./🧫️fixtures/🧬️document-admission/🔣️.json" with { type: "json" };
import fem2dDocumentSchema from "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🔣️.json" with { type: "json" };
import fem3dDocumentSchema from "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🔣️.json" with { type: "json" };
import { parseFem2dArtifact } from "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🟦️.ts";
import { parseFem3dArtifact } from "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🟦️.ts";
import mountedClose from "./🧫️fixtures/🧹️mounted-close/🔣️.json" with { type: "json" };
import mountedCloseSchema from "./🧬️schema/🧹️mounted-close/🔣️.json" with { type: "json" };
import closeGrants from "./🧫️fixtures/💳️visual-close-grants/🔣️.json" with { type: "json" };
import childOutcomes from "./🧫️fixtures/🧒️child-outcomes/🔣️.json" with { type: "json" };
import childOutcomesSchema from "./🧬️schema/🧒️child-outcomes/🔣️.json" with { type: "json" };
import closeGrantsSchema from "./🧬️schema/💳️visual-close-grants/🔣️.json" with { type: "json" };
import { testFem2dModelWindowConfigContract } from "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts";
import { testFem2dResultsWindowConfigContract } from "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts";
import { testFem3dModelWindowConfigContract } from "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts";
import { testFem3dResultsWindowConfigContract } from "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts";

export function testFem2dWindowConfigContract(): void {
  testFemDocumentAdmission("2d");
  testFem2dModelWindowConfigContract();
  testFem2dResultsWindowConfigContract();
  console.log("[DEBUG] FEM 2D exact-window ownership facets passed independent neutral validation");
}

export function testFem3dWindowConfigContract(): void {
  testFemAssemblyMergeRefusalOracle();
  testFemAssemblyStepGrantOracle();
  testFemPagedCsrOracle();
  testFemPcgPublicationGrantOracle();
  testFemScalarOwnerOracle();
  const validateCommands = new Ajv({ strict: true, allErrors: true }).addKeyword("x-semio-formats").compile(commandLimitsSchema);
  assert(validateCommands(commandLimits), JSON.stringify(validateCommands.errors));
  for (const id of ["setCamera", "setResultDisplay"]) {
    const index = commandLimits.routes.findIndex((row) => row.id === id);
    assert(index >= 0);
    assert.deepEqual(commandLimits.routes[index]!.lanes, ["WindowConfig"]);
    const wrongOwner = applyPatch(structuredClone(commandLimits), [{ op: "replace", path: `/routes/${index}/lanes`, value: ["Config"] }], true).newDocument;
    assert(!validateCommands(wrongOwner), id);
  }
  assert(!Object.hasOwn(fem3dDocumentSchema.$defs, "Fem3dRetainedCommandLimits"));
  console.log("[DEBUG] FEM 3D command-owned route schema rejects application config lanes for exact window commands");
  testMeshEdgeAuthority();
  const validateOutcomes = new Ajv({ strict: true, allErrors: true }).compile(childOutcomesSchema);
  assert(validateOutcomes(childOutcomes), JSON.stringify(validateOutcomes.errors));
  for (const row of childOutcomes.cases) assert.deepEqual(applyPatch(structuredClone(row.before), row.patch as Operation[], true).newDocument, row.expected, row.kind);
  console.log("[DEBUG] All child outcome ownership cases agree with Ajv and fast-json-patch");
  testFemDocumentAdmission("3d");
  const validateGrants = new Ajv({ strict: true, allErrors: true }).compile(closeGrantsSchema);
  assert(validateGrants(closeGrants), JSON.stringify(validateGrants.errors));
  for (const row of closeGrants.cases) assert.deepEqual(applyPatch(structuredClone(row.before), row.patch as Operation[], true).newDocument, row.expected, row.id);
  console.log("[DEBUG] FEM visual close grant cases agree with Ajv and fast-json-patch");
  const validateClose = new Ajv({ strict: true, allErrors: true }).compile(mountedCloseSchema);
  assert(validateClose(mountedClose), JSON.stringify(validateClose.errors));
  for (const row of mountedClose.cases) assert.deepEqual(applyPatch(structuredClone(row.before), row.patch as Operation[], true).newDocument, row.expected, row.id);
  console.log("[DEBUG] FEM 3D mounted-close neutral cases agree with Ajv and fast-json-patch");
  testFem3dModelWindowConfigContract();
  testFem3dResultsWindowConfigContract();
  console.log("[DEBUG] FEM 3D exact-window ownership facets passed independent neutral validation");
}

function testFemDocumentAdmission(dimension: "2d" | "3d"): void {
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addKeyword("x-semio-state").addKeyword("x-semio-invariant");
  ajv.addFormat("double", { type: "number", validate: Number.isFinite });
  ajv.addFormat("uint32", { type: "number", validate: (value: number) => Number.isInteger(value) && value >= 0 && value <= 0xffffffff });
  const schema = dimension === "2d" ? fem2dDocumentSchema : fem3dDocumentSchema;
  const parse = dimension === "2d" ? parseFem2dArtifact : parseFem3dArtifact;
  const validate = ajv.compile(schema);
  const row = documentAdmission.cases.find((candidate) => candidate.dimension === dimension)!;
  assert(validate(row.document), JSON.stringify(validate.errors));
  assert.deepEqual(parse(row.document), row.document);
  for (const field of row.foreignFields) {
    const candidate = { ...structuredClone(row.document), [field.key]: field.value };
    assert(!validate(candidate), field.key);
    assert.throws(() => parse(candidate), field.key);
  }
  console.log(`[DEBUG] FEM ${dimension} document admission rejects window/OS fields in agreement with Ajv`);
}

if (import.meta.main) {
  testFem2dWindowConfigContract();
  testFem3dWindowConfigContract();
}
