import assert from "node:assert/strict";
import { testMeshEdgeAuthority } from "../../../../../../../../../../../🔨️modules/🏗️fem/⚙️engine/🕸️mesh/🧪️tests/🕸️edge-authority/🟦️.ts";
import { testFemAssemblyMergeRefusalOracle } from "../../../../../../../../../../../🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧪️tests/🔀️merge-refusal/🟦️.ts";
import { testFemAssemblyStepGrantOracle } from "../../../../../../../../../../../🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧪️tests/⛽️step-grant/🟦️.ts";
import { testFemPagedCsrOracle } from "../../../../../../../../../../../🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🧪️tests/📚️paged-csr/🟦️.ts";
import { testFemPcgPublicationGrantOracle } from "../../../../../../../../../../../🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🧪️tests/⛽️publication-grant/🟦️.ts";
import { testFemScalarOwnerOracle } from "../../../../../../../../../../../🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🧪️tests/🔢️scalar-owners/🟦️.ts";
import Ajv from "ajv";
import { applyPatch, type Operation } from "fast-json-patch";
import commandLimits from "../../🧫️fixtures/🚧️retained-command-limits/🔣️.json" with { type: "json" };
import commandLimitsSchema from "../../🎮️commands/🧬️schema/🚧️retained-limits/🔣️.json" with { type: "json" };
import documentAdmission from "../../../🧫️fixtures/🧬️document-admission/🔣️.json" with { type: "json" };
import fem3dDocumentSchema from "../../../🧬️schema/🔣️.json" with { type: "json" };
import { parseFem3dArtifact } from "../../../🧬️schema/🟦️.ts";
import mountedClose from "../../../../../../../../../🧫️fixtures/🧹️mounted-close/🔣️.json" with { type: "json" };
import mountedCloseSchema from "../../../../../../../../../🧫️fixtures/🧹️mounted-close/📐️schema/🔣️.json" with { type: "json" };
import closeGrants from "../../../../../../../../../🧫️fixtures/💳️visual-close-grants/🔣️.json" with { type: "json" };
import childOutcomes from "../../../../../../../../../🧫️fixtures/🧒️child-outcomes/🔣️.json" with { type: "json" };
import childOutcomesSchema from "../../../../../../../../../🧫️fixtures/🧒️child-outcomes/📐️schema/🔣️.json" with { type: "json" };
import closeGrantsSchema from "../../../../../../../../../🧫️fixtures/💳️visual-close-grants/📐️schema/🔣️.json" with { type: "json" };
import { testFem3dModelWindowConfigContract } from "../../🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts";
import { testFem3dResultsWindowConfigContract } from "../../🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts";


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
  testMeshEdgeAuthority();
  const validateOutcomes = new Ajv({ strict: true, allErrors: true }).compile(childOutcomesSchema);
  assert(validateOutcomes(childOutcomes), JSON.stringify(validateOutcomes.errors));
  for (const row of childOutcomes.cases) assert.deepEqual(applyPatch(structuredClone(row.before), row.patch as Operation[], true).newDocument, row.expected, row.kind);
  testFemDocumentAdmission();
  const validateGrants = new Ajv({ strict: true, allErrors: true }).compile(closeGrantsSchema);
  assert(validateGrants(closeGrants), JSON.stringify(validateGrants.errors));
  for (const row of closeGrants.cases) assert.deepEqual(applyPatch(structuredClone(row.before), row.patch as Operation[], true).newDocument, row.expected, row.id);
  const validateClose = new Ajv({ strict: true, allErrors: true }).compile(mountedCloseSchema);
  assert(validateClose(mountedClose), JSON.stringify(validateClose.errors));
  for (const row of mountedClose.cases) assert.deepEqual(applyPatch(structuredClone(row.before), row.patch as Operation[], true).newDocument, row.expected, row.id);
  testFem3dModelWindowConfigContract();
  testFem3dResultsWindowConfigContract();
}

function testFemDocumentAdmission(): void {
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addKeyword("x-semio-state").addKeyword("x-semio-invariant");
  ajv.addFormat("double", { type: "number", validate: Number.isFinite });
  ajv.addFormat("uint32", { type: "number", validate: (value: number) => Number.isInteger(value) && value >= 0 && value <= 0xffffffff });
  const schema = fem3dDocumentSchema;
  const parse = parseFem3dArtifact;
  const validate = ajv.compile(schema);
  const row = documentAdmission;
  assert(validate(row.document), JSON.stringify(validate.errors));
  assert.deepEqual(parse(row.document), row.document);
  for (const field of row.foreignFields) {
    const candidate = { ...structuredClone(row.document), [field.key]: field.value };
    assert(!validate(candidate), field.key);
    assert.throws(() => parse(candidate), field.key);
  }
}

if (import.meta.main) testFem3dWindowConfigContract();
