import assert from "node:assert/strict";
import { testMeshEdgeAuthority } from "@semio-tech/s-fem-engine-ts/mesh/testing/edge-authority";
import { testFemAssemblyMergeRefusalOracle } from "@semio-tech/s-fem-engine-ts/analyses/testing/merge-refusal";
import { testFemAssemblyStepGrantOracle } from "@semio-tech/s-fem-engine-ts/analyses/testing/step-grant";
import { testFemPagedCsrOracle } from "@semio-tech/s-fem-engine-ts/sparse/testing/paged-csr";
import { testFemPcgPublicationGrantOracle } from "@semio-tech/s-fem-engine-ts/sparse/testing/publication-grant";
import { testFemScalarOwnerOracle } from "@semio-tech/s-fem-engine-ts/sparse/testing/scalar-owners";
import Ajv from "ajv";
import commandLimits from "../🧫️fixtures/🚧️retained-command-limits/🔣️.json" with { type: "json" };

import documentAdmission from "../../../🧪️tests/🧫️fixtures/🧬️document-admission/🔣️.json" with { type: "json" };
import { applyPatch, getValueByPointer, type Operation } from "fast-json-patch";
import identityAdmission from "../../🕹️interaction/🧫️fixtures/🔣️.json" with { type: "json" };
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
  ajv.addKeyword("x-semio-state").addKeyword("x-semio-invariant").addKeyword("x-semio-ui");
  ajv.addFormat("double", { type: "number", validate: Number.isFinite });
  ajv.addFormat("uint32", { type: "number", validate: (value: number) => Number.isInteger(value) && value >= 0 && value <= 0xffffffff });
  const schema = fem3dDocumentSchema;
  const parse = parseFem3dArtifact;
  const validate = ajv.compile(schema);
  const row = documentAdmission;
  assert(validate(row.document), JSON.stringify(validate.errors));
  const nativePatch: Operation[] = row.nativeWords.map(({ path, word }) => {
    assert.equal(Buffer.from(word, "hex").readDoubleBE(), getValueByPointer(row.document, path), path);
    return { op: "replace", path, value: { bits: BigInt(`0x${word}`) } };
  });
  for (const { path, word } of row.nativeCounts) {
    assert.equal(BigInt(word), BigInt(getValueByPointer(row.document, path)), path);
    nativePatch.push({ op: "replace", path, value: BigInt(word) });
  }
  const native = applyPatch(structuredClone(row.document), nativePatch, true).newDocument;
  for (let index = 0; index < native.combinations.length; index++) {
    applyPatch(native, [{ op: "replace", path: `/combinations/${index}/terms`, value: new Map(Object.entries(native.combinations[index]!.terms)) }], true);
  }
  assert.deepEqual(parse(native), native);
  for (const row of identityAdmission.invalidOwnerCases) {
    const invalid = structuredClone(native);
    if (row.kind === "emptyNode") invalid.nodes[0]!.id = "";
    else if (row.kind === "duplicateNode") invalid.nodes.push(structuredClone(invalid.nodes[0]!));
    else if (row.kind === "duplicateLoad") invalid.loadCases[0]!.loads.push(structuredClone(invalid.loadCases[0]!.loads[0]!));
    else if (row.kind === "oversizedNode") invalid.nodes[0]!.id = "x".repeat(row.bytes!);
    else throw Error("unknown owner admission vector");
    assert.throws(() => parse(invalid), row.kind);
  }
  const scopedCollision = structuredClone(native);
  scopedCollision.nodes[0]!.id = scopedCollision.materials[0]!.id;
  scopedCollision.loadCases.push({ ...structuredClone(scopedCollision.loadCases[0]!), id: "another-case" });
  assert.deepEqual(parse(scopedCollision), scopedCollision);
  console.log("[DEBUG] FEM3d native TypeScript root refused four invalid owners and admitted cross-kind/cross-case collisions");
  assert.throws(() => parse(row.document));
  for (const field of row.foreignFields) {
    const candidate = { ...structuredClone(row.document), [field.key]: field.value };
    assert(!validate(candidate), field.key);
    assert.throws(() => parse({ ...structuredClone(native), [field.key]: field.value }), field.key);
  }
  console.log(`[DEBUG] FEM3d document admission verified ${row.nativeWords.length} native words and ${row.nativeCounts.length} native counts`);
}

if (import.meta.main) testFem3dWindowConfigContract();
