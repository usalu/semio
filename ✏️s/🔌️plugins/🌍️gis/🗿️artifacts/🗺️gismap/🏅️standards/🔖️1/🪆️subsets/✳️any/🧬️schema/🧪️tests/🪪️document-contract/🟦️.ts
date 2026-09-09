/** 🧪️ Map document facets preserve every durable handle and dynamic feature value. */
import assert from "node:assert/strict";
import { testSchemaRecordOracle } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🧪️tests/🔬️unit/🟦️.ts";
import { join } from "node:path";
import { assertDocumentContractOracle } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧪️testkit/🪪️document-contract/🟦️.ts";
import ioSchema from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json" with { type: "json" };
import childSchema from "../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🔣️.json" with { type: "json" };
import valueSchema from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🔣️.json" with { type: "json" };
import featureSchema from "../../📍️feature/🔣️.json" with { type: "json" };
import artifactSchema from "../../🔣️.json" with { type: "json" };
import snapshotSchema from "../../📸️snapshot/🔣️.json" with { type: "json" };
import diffSchema from "../../🔺️diff/🔣️.json" with { type: "json" };
import { parseGisMapArtifact } from "../../🟦️.ts";
import { parseGisMapSnapshot } from "../../📸️snapshot/🟦️.ts";
import { parseGisMapDiff } from "../../🔺️diff/🟦️.ts";
import vectors from "../../🧫️fixtures/🪪️document-contract/🔣️.json" with { type: "json" };

/** 🗺️ Matches first-party parsing to Ajv and all committed Map mutation fixtures. */
export function testMapDocumentContractOracle(): void {
  testSchemaRecordOracle();
  assertDocumentContractOracle({
    name: "GIS Map", dependencies: [ioSchema, childSchema, valueSchema, featureSchema],
    artifact: { schema: artifactSchema, parse: parseGisMapArtifact },
    snapshot: { schema: snapshotSchema, parse: parseGisMapSnapshot },
    diff: { schema: diffSchema, parse: parseGisMapDiff },
    validDocuments: [{ input: vectors.document, output: vectors.document }],
    invalidDocuments: vectors.invalidDocuments, invalidDiffs: vectors.invalidDiffs,
    mutationRoots: [join(import.meta.dir, "../../../🧫️fixtures/🧬️mutations")],
    committed: { snapshots: 24, diffs: 12 },
  });
  assert.deepEqual(parseGisMapDiff(vectors.diff), vectors.diff);
  assert.deepEqual(parseGisMapDiff({}), { artifact: null, positions: null, routes: null, regions: null });
}
