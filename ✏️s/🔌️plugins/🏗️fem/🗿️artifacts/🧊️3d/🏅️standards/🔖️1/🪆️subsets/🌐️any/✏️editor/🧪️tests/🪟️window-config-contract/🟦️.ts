import assert from "node:assert/strict";
import { testMeshEdgeAuthority } from "@semio-tech/s-fem-engine-ts/mesh/testing/edge-authority";
import { testFemAssemblyMergeRefusalOracle } from "@semio-tech/s-fem-engine-ts/analyses/testing/merge-refusal";
import { testFemAssemblyStepGrantOracle } from "@semio-tech/s-fem-engine-ts/analyses/testing/step-grant";
import { testFemPagedCsrOracle } from "@semio-tech/s-fem-engine-ts/sparse/testing/paged-csr";
import { testFemPcgPublicationGrantOracle } from "@semio-tech/s-fem-engine-ts/sparse/testing/publication-grant";
import { testFemScalarOwnerOracle } from "@semio-tech/s-fem-engine-ts/sparse/testing/scalar-owners";
import Ajv from "ajv";
import { applyPatch, type Operation } from "fast-json-patch";
import commandLimits from "../🧫️fixtures/🚧️retained-command-limits/🔣️.json" with { type: "json" };

import documentAdmission from "../../../🧪️tests/🧫️fixtures/🧬️document-admission/🔣️.json" with { type: "json" };
import fem3dDocumentSchema from "../../../🧬️schema/🔣️.json" with { type: "json" };
import { parseFem3dArtifact } from "../../../🧬️schema/🟦️.ts";
import { mountedCloseFixture as mountedClose } from "@semio-tech/fem-js/testing/fixtures/mounted-close";

import { visualCloseGrantsFixture as closeGrants } from "@semio-tech/fem-js/testing/fixtures/visual-close-grants";
import { childOutcomesFixture as childOutcomes } from "@semio-tech/fem-js/testing/fixtures/child-outcomes";


import { testFem3dModelWindowConfigContract } from "../../🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts";
import { testFem3dResultsWindowConfigContract } from "../../🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts";


export function testFem3dWindowConfigContract(): void {
  testFemAssemblyMergeRefusalOracle();
  testFemAssemblyStepGrantOracle();
  testFemPagedCsrOracle();
  testFemPcgPublicationGrantOracle();
  testFemScalarOwnerOracle();

  for (const id of ["setCamera", "setResultDisplay"]) {
    const index = commandLimits.routes.findIndex((row) => row.id === id);
    assert(index >= 0);
    assert.deepEqual(commandLimits.routes[index]!.lanes, ["WindowConfig"]);
    const wrongOwner = applyPatch(structuredClone(commandLimits), [{ op: "replace", path: `/routes/${index}/lanes`, value: ["Config"] }], true).newDocument;
    assert.notDeepEqual(wrongOwner.routes[index]!.lanes, ["WindowConfig"], id);
  }
  assert(!Object.hasOwn(fem3dDocumentSchema.$defs, "Fem3dRetainedCommandLimits"));
  testMeshEdgeAuthority();
  
  
  for (const row of childOutcomes.cases) assert.deepEqual(applyPatch(structuredClone(row.before), row.patch as Operation[], true).newDocument, row.expected, row.kind);
  testFemDocumentAdmission();
  
  
  for (const row of closeGrants.cases) assert.deepEqual(applyPatch(structuredClone(row.before), row.patch as Operation[], true).newDocument, row.expected, row.id);
  
  
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
