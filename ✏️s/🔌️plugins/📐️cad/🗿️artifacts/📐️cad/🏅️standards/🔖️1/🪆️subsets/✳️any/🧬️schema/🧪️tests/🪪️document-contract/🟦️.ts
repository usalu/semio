/** 🧪️ CAD document facets compose exact model and drawing child identities. */
import assert from "node:assert/strict";
import { fileURLToPath } from "node:url";
import { assertDocumentContractOracle } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧪️testkit/🪪️document-contract/🟦️.ts";
import ioSchema from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json" with { type: "json" };
import childSchema from "../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🔣️.json" with { type: "json" };
import artifactSchema from "../../🔣️.json" with { type: "json" };
import snapshotSchema from "../../📸️snapshot/🔣️.json" with { type: "json" };
import diffSchema from "../../🔺️diff/🔣️.json" with { type: "json" };
import { parseCadArtifact } from "../../🟦️.ts";
import { parseCadSnapshot } from "../../📸️snapshot/🟦️.ts";
import { parseCadDiff } from "../../🔺️diff/🟦️.ts";
import vectors from "../../🧫️fixtures/🪪️document-contract/🔣️.json" with { type: "json" };

type InvalidDocument = { kind: string; field?: string; value: unknown };

/** 🪪️ Compares production parsers with Ajv and the complete committed CAD mutation corpus. */
export function testCadDocumentContractOracle(): void {
  const document = structuredClone(vectors.document) as Record<string, unknown>;
  const invalidRows = vectors.invalidDocuments as InvalidDocument[];
  const invalidDocuments = invalidRows.filter((row) => row.kind !== "shapeChildId").map((row) => {
    const candidate = structuredClone(document) as Record<string, any>;
    if (row.kind === "field") candidate[row.field!] = row.value;
    if (row.kind === "shapeChildId") candidate.shapeModel.childId = row.value;
    if (row.kind === "shapeSubset") candidate.shapeModel.target.dialect.subset = row.value;
    if (row.kind === "drawingSubset") candidate.drawings[0].target.dialect.subset = row.value;
    return candidate;
  });
  assertDocumentContractOracle({
    name: "CAD", dependencies: [ioSchema, childSchema],
    artifact: { schema: artifactSchema, parse: parseCadArtifact },
    snapshot: { schema: snapshotSchema, parse: parseCadSnapshot },
    diff: { schema: diffSchema, parse: parseCadDiff },
    validDocuments: [{ input: vectors.document, output: vectors.document }],
    invalidDocuments, invalidDiffs: vectors.invalidDiffs,
    mutationRoots: [fileURLToPath(new URL("../../../🧫️fixtures/🧬️mutations", import.meta.url))],
    committed: { snapshots: 38, diffs: 19 },
  });
  const mismatchedChild = structuredClone(document) as Record<string, any>;
  mismatchedChild.shapeModel.childId = "foreign-id";
  assert.throws(() => parseCadSnapshot(mismatchedChild), /childId must equal target.artifactId/);
  assert.deepEqual(parseCadDiff(vectors.diff), vectors.diff);
  console.log("[DEBUG] CAD exact document contract matched model/drawing child identity vectors and the committed mutation corpus");
}
