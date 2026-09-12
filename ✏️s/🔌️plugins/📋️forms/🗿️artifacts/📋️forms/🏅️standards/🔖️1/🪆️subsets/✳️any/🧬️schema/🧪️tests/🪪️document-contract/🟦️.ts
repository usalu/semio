import { applyPatch, compare } from "fast-json-patch";
import { applyFormsDiff } from "../../🔺️diff/🟦️.ts";
import assert from "node:assert/strict";
import semioChildSchema from "../../../../../../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🪆️child/🔣️.json" with { type: "json" };
import { assertDocumentContractOracle } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🔮️oracles/🪪️document-contract/🟦️.ts";
/** 🧪️ Forms persisted fields and child identities agree with independent JSON Schema validation. */
import { fileURLToPath } from "node:url";
import ioSchema from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json" with { type: "json" };
import childSchema from "../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🔣️.json" with { type: "json" };
import artifactSchema from "../../🔣️.json" with { type: "json" };
import snapshotSchema from "../../📸️snapshot/🔣️.json" with { type: "json" };
import diffSchema from "../../🔺️diff/🔣️.json" with { type: "json" };
import * as artifact from "../../🟦️.ts";
import * as snapshot from "../../📸️snapshot/🟦️.ts";
import * as diff from "../../🔺️diff/🟦️.ts";
import vectors from "./../../🧫️fixtures/🪪️document-contract/🔣️.json" with { type: "json" };

/** 🪪️ Checks exact child identity and the editor/document boundary. */
export function testFormsDocumentContractOracle(): void {
  const { title: _, ...base } = vectors.document;
  assertDocumentContractOracle({
    name: "Forms",
    dependencies: [ioSchema, childSchema, semioChildSchema],
    childIdentityFields: ["structure", "results"],
    artifact: { schema: artifactSchema, parse: artifact.parseFormsArtifact },
    snapshot: { schema: snapshotSchema, parse: snapshot.parseFormsSnapshot },
    diff: { schema: diffSchema, parse: diff.parseFormsDiff },
    validDocuments: [{ input: vectors.document, output: vectors.document }, { input: base, output: base }, { input: { ...base, title: null }, output: base }],
    invalidDocuments: [...vectors.invalidIdentityDocuments, ...Object.entries(vectors.invalidDocumentFields).map(([key, value]) => ({ ...vectors.document, [key]: value })), ...vectors.invalidChildren.map((child) => ({ ...vectors.document, structure: child }))],
    invalidDiffs: vectors.invalidDiffs,
    validDiffs: vectors.patchCases.map((item) => ({ input: item.diff, output: item.diff })),
    mutationRoots: [fileURLToPath(new URL("../../../🧫️fixtures/🧬️mutations", import.meta.url))],
    committed: { snapshots: 20, diffs: 6 },
  });
  for (const item of vectors.patchCases) {
    const base = artifact.parseFormsArtifact(item.before);
    assert.deepEqual(applyFormsDiff(base, item.diff), item.after, item.name);
    assert.deepEqual(applyPatch(structuredClone(item.before), compare(item.before, item.after)).newDocument, item.after, item.name);
    for (const field of ["structure", "results"] as const) if (!Object.hasOwn(item.diff, field)) assert.equal(applyFormsDiff(base, item.diff)[field], base[field]);
  }
  console.log("[DEBUG] Forms sparse edit laws matched independent JSON Patch and retained untouched child identities");
}
