import artifactReferenceSchema from "../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/🔣️.json";
import valueSchema from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🔣️.json" with { type: "json" };
/** 🧪️ Canonical Wires document contracts agree with independent schema validation. */
import assert from "node:assert/strict";
import Ajv from "ajv";
import ioSchema from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json" with { type: "json" };
import childSchema from "../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🔣️.json" with { type: "json" };
import artifactSchema from "../../🔣️.json" with { type: "json" };
import snapshotSchema from "../../📸️snapshot/🔣️.json" with { type: "json" };
import diffSchema from "../../🔺️diff/🔣️.json" with { type: "json" };
import mutationSchema from "../../🧬️mutations/🔣️.json" with { type: "json" };
import fixture from "../../🧫️fixtures/🪪️document-contract/🔣️.json" with { type: "json" };

import { parseWiresArtifact } from "../../🟦️.ts";
import { parseWiresSnapshot } from "../../📸️snapshot/🟦️.ts";
import { decodeWiresJsonSnapshot } from "../../../🚪️io/📝️text/📸️snapshot/🔣️json/🟦️.ts";

/** 🪪️ Parent document admission agrees with its closed schemas and child ownership. */
export function testWiresDocumentContractOracle(): void {
  const ajv = new Ajv({ strict: false, allErrors: true }).addSchema(artifactReferenceSchema);
  ajv.addSchema(valueSchema).addSchema(ioSchema).addSchema(childSchema).addSchema(artifactSchema);
  
  
  
  assert.equal(new Set(fixture.cases.map(row => row.id)).size, fixture.cases.length);
  for (const schema of [artifactSchema, snapshotSchema]) assert.equal(schema.properties.content["x-semio-child-kind"], fixture.declaredChildKind);
  assert.deepEqual(Object.keys(diffSchema.properties), []);
  const documents = [[ajv.compile(artifactSchema), parseWiresArtifact], [ajv.compile(snapshotSchema), parseWiresSnapshot]] as const;
  const validateDiff = ajv.compile(diffSchema), validateMutation = ajv.compile(mutationSchema);
  for (const row of fixture.cases) {
    if (row.kind === "snapshot") {
      for (const [validate, parse] of documents) {
        assert.equal(validate(row.value), row.accepted, row.id + ": " + JSON.stringify(validate.errors));
        if (row.accepted) {
          const canonical = decodeWiresJsonSnapshot(row.value);
          assert.deepEqual(parse(canonical), canonical, row.id);
        } else assert.throws(() => parse(decodeWiresJsonSnapshot(row.value)), row.id);
      }
    } else if (row.kind === "diff") assert.equal(validateDiff(row.value), row.accepted, row.id);
    else {
      assert.equal(row.accepted, false, row.id);
      assert.equal(validateMutation(row.value), false, row.id);
    }
  }
}
