/** 🧪️ CAD document facets compose exact model and drawing child identities. */
import artifactReferenceSchema from "../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/🔣️.json";
import assert from "node:assert/strict";
import { fileURLToPath } from "node:url";
import { assertDocumentContractOracle } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🔮️oracles/🪪️document/🟦️.ts";
import ioSchema from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json" with { type: "json" };
import childSchema from "../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🔣️.json" with { type: "json" };
import artifactSchema from "../../🔣️.json" with { type: "json" };
import snapshotSchema from "../../📸️snapshot/🔣️.json" with { type: "json" };
import diffSchema from "../../🔺️diff/🔣️.json" with { type: "json" };
import { parseCadArtifact } from "../../🟦️.ts";
import { parseCadSnapshot } from "../../📸️snapshot/🟦️.ts";
import { parseCadDiff } from "../../🔺️diff/🟦️.ts";
import vectors from "../../🧫️fixtures/🪪️document/🔣️.json" with { type: "json" };
import { testCadWorldWindowConfigContract } from "../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🎚️config/🧬️schema/🧪️tests/🪪️document/🟦️.ts";

type InvalidDocument = { kind: string; field?: string; value: unknown };

/** 🪪️ Compares production parsers with Ajv and the complete committed CAD mutation corpus. */
export function testCadDocumentContractOracle(): void {
  testCadWorldWindowConfigContract();
  const document = structuredClone(vectors.document) as Record<string, unknown>;
  const mismatchedChild = structuredClone(document) as Record<string, any>;
  mismatchedChild.shapeModel.childId = "foreign-id";
  const invalidRows = vectors.invalidDocuments as InvalidDocument[];
  const invalidDocuments = invalidRows.map((row) => {
    const candidate = structuredClone(document) as Record<string, any>;
    if (row.kind === "field") candidate[row.field!] = row.value;
    if (row.kind === "shapeSubset") candidate.shapeModel.target.dialect.subset = row.value;
    if (row.kind === "drawingSubset") candidate.drawings[0].target.dialect.subset = row.value;
    return candidate;
  });
  assertDocumentContractOracle({
    name: "CAD", dependencies: [ioSchema, childSchema,artifactReferenceSchema],
    artifact: { schema: artifactSchema, parse: parseCadArtifact },
    snapshot: { schema: snapshotSchema, parse: parseCadSnapshot },
    diff: { schema: diffSchema, parse: parseCadDiff },
    validDocuments: [{ input: vectors.document, output: vectors.document }, { input: mismatchedChild, output: mismatchedChild }],
    invalidDocuments, invalidDiffs: vectors.invalidDiffs,
    mutationRoots: [fileURLToPath(new URL("../../../🧫️fixtures/🧬️mutations", import.meta.url))],
    committed: { snapshots: 42, diffs: 21 },
  });
  assert.deepEqual(parseCadSnapshot(mismatchedChild), mismatchedChild);
  assert.deepEqual(parseCadDiff(vectors.diff), vectors.diff);
}
