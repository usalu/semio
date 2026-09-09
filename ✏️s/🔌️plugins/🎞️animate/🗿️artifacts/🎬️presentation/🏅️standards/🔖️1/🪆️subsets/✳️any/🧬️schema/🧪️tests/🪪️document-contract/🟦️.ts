/** 🧪️ Presentation document facets compose the shared presentation and animation child identities. */
import assert from "node:assert/strict";
import { join } from "node:path";
import { assertDocumentContractOracle } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧪️testkit/🪪️document-contract/🟦️.ts";
import ioSchema from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json" with { type: "json" };
import childSchema from "../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🔣️.json" with { type: "json" };
import artifactSchema from "../../🔣️.json" with { type: "json" };
import snapshotSchema from "../../📸️snapshot/🔣️.json" with { type: "json" };
import diffSchema from "../../🔺️diff/🔣️.json" with { type: "json" };
import { parsePresentationArtifact } from "../../🟦️.ts";
import { parsePresentationSnapshot } from "../../📸️snapshot/🟦️.ts";
import { parsePresentationDiff } from "../../🔺️diff/🟦️.ts";
import vectors from "../../🧫️fixtures/🪪️document-contract/🔣️.json" with { type: "json" };

/** 🪆️ Compares first-party parsers with Ajv and every committed Presentation mutation document. */
export function testPresentationDocumentContractOracle(): void {
  assertDocumentContractOracle({
    name: "Presentation", dependencies: [ioSchema, childSchema],
    artifact: { schema: artifactSchema, parse: parsePresentationArtifact },
    snapshot: { schema: snapshotSchema, parse: parsePresentationSnapshot },
    diff: { schema: diffSchema, parse: parsePresentationDiff },
    validDocuments: [{ input: vectors.document, output: vectors.document }],
    invalidDocuments: vectors.invalidDocuments, invalidDiffs: vectors.invalidDiffs,
    mutationRoots: [join(import.meta.dir, "../../../🧫️fixtures/🧬️mutations")],
    committed: { snapshots: 18, diffs: 5 },
  });
  assert.deepEqual(parsePresentationDiff(vectors.diff), vectors.diff);
  assert.deepEqual(parsePresentationDiff({}), { artifact: null, schema: null, presentation: null });
}
