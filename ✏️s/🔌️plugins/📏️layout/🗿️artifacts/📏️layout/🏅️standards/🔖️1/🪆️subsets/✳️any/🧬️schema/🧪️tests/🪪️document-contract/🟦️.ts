/** 🧪️ Layout parent facets retain the live drawing payload while enforcing its child identity. */
import assert from "node:assert/strict";
import { fileURLToPath } from "node:url";
import { assertDocumentContractOracle } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🔮️oracles/🪪️document-contract/🟦️.ts";
import ioSchema from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json" with { type: "json" };
import blobSchema from "../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️blob/🧬️schema/🔣️.json" with { type: "json" };
import childSchema from "../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🔣️.json" with { type: "json" };
import linkSchema from "../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️link/🧬️schema/🔣️.json" with { type: "json" };
import drawingSchema from "../../../../../../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/📸️snapshot/🔣️.json" with { type: "json" };
import artifactSchema from "../../🔣️.json" with { type: "json" };
import snapshotSchema from "../../📸️snapshot/🔣️.json" with { type: "json" };
import diffSchema from "../../🔺️diff/🔣️.json" with { type: "json" };
import { parseLayoutArtifact } from "../../🟦️.ts";
import { parseLayoutSnapshot } from "../../📸️snapshot/🟦️.ts";
import { parseLayoutDiff } from "../../🔺️diff/🟦️.ts";
import vectors from "../../🧫️fixtures/🪪️document-contract/🔣️.json" with { type: "json" };

const fields = ["schema", "name", "grid", "paragraphStyles", "characterStyles", "stories", "links", "parentPages", "spreads", "pages", "printTarget", "dataFieldsJson", "backgroundDrawing", "referencedModel"] as const;

/** 🪪️ Validates the neutral wrapper, independent Ajv schemas and every committed mutation result. */
export function testLayoutDocumentContractOracle(): void {
  const document = structuredClone(vectors.document) as Record<string, any>;
  const missingContent = structuredClone(document);
  delete missingContent.backgroundDrawing.content;
  const modelAsChild = structuredClone(document);
  modelAsChild.referencedModel = { childId: "model", target: document.referencedModel.target };
  assertDocumentContractOracle({
    name: "Layout",
    dependencies: [ioSchema, blobSchema, childSchema, linkSchema, drawingSchema],
    artifact: { schema: artifactSchema, parse: parseLayoutArtifact },
    snapshot: { schema: snapshotSchema, parse: parseLayoutSnapshot },
    diff: { schema: diffSchema, parse: parseLayoutDiff },
    validDocuments: [{ input: vectors.document, output: vectors.document }],
    invalidDocuments: [{ ...document, camera: { x: 0, y: 0, zoom: 1 } }, missingContent, modelAsChild],
    mutationRoots: [fileURLToPath(new URL("../../../🧫️fixtures/🧬️mutations", import.meta.url))],
    committed: { snapshots: 50, diffs: 25 },
  });
  assert.deepEqual(Object.keys(artifactSchema.properties), fields);
  assert.deepEqual(Object.keys(snapshotSchema.properties), fields);
  assert.deepEqual(Object.keys(diffSchema.properties), ["artifact", ...fields]);
  assert.deepEqual(parseLayoutDiff(vectors.diff), vectors.diff);
  const child = parseLayoutArtifact(document).backgroundDrawing!;
  assert.equal(child.handle.childId, child.handle.target.artifactId);
  assert.deepEqual(child.handle.target.dialect, { artifactKind: "s.stdio.semio", standard: "v1", subset: "drawing" });
  assert.deepEqual(child.content, document.backgroundDrawing.content, "inline drawing remains available to live exporters");
  const wrongIdentity = structuredClone(document);
  wrongIdentity.backgroundDrawing.handle.target.artifactId = "wrong";
  assert.throws(() => parseLayoutArtifact(wrongIdentity), /childId must equal target.artifactId/);
  console.log("[DEBUG] Layout document contract preserved drawing content and exact child/link authority");
}
