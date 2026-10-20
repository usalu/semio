/** 🧪️ Presentation document facets compose the shared presentation and animation child identities. */
import { artifactReferenceSchema as artifactReferenceSchema, artifactChildSchema as childSchema } from "@semio-tech/framework/schema/artifact-reference";
import assert from "node:assert/strict";
import { join } from "node:path";
import { assertDocumentContractOracle } from "@semio-tech/framework/schema/oracles/document";
import { ioSchema as ioSchema } from "@semio-tech/framework/io/schema";
import artifactSchema from "../../🔣️.json" with { type: "json" };
import snapshotSchema from "../../📸️snapshot/🔣️.json" with { type: "json" };
import diffSchema from "../../🔺️diff/🔣️.json" with { type: "json" };
import { parsePresentationArtifact } from "../../🟦️.ts";
import { parsePresentationSnapshot } from "../../📸️snapshot/🟦️.ts";
import { parsePresentationDiff } from "../../🔺️diff/🟦️.ts";
import vectors from "../../../🧪️tests/🧫️fixtures/🔬️contract/🪪️document-contract/🔣️.json" with { type: "json" };

/** 🪆️ Compares first-party parsers with Ajv and every committed Presentation mutation document. */
export function testPresentationDocumentContractOracle(): void {
  assertDocumentContractOracle({
    name: "Presentation", dependencies: [ioSchema, childSchema,artifactReferenceSchema],
    artifact: { schema: artifactSchema, parse: parsePresentationArtifact },
    snapshot: { schema: snapshotSchema, parse: parsePresentationSnapshot },
    diff: { schema: diffSchema, parse: parsePresentationDiff },
    validDocuments: [{ input: vectors.document, output: vectors.document }],
    invalidDocuments: vectors.invalidDocuments, invalidDiffs: vectors.invalidDiffs,
    mutationRoots: [join(import.meta.dir, "../../../🧪️tests/🧫️fixtures/🧬️mutations")],
    committed: { snapshots: 16, diffs: 4 },
  });
  assert.deepEqual(parsePresentationDiff(vectors.diff), vectors.diff);
  assert.deepEqual(parsePresentationDiff({}), { schema: null, source: null, tiles: null });
}
