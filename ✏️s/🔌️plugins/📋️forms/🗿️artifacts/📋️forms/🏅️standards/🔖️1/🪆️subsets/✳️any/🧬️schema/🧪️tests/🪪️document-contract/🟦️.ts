import { assertDocumentContractOracle } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧪️testkit/🪪️document-contract/🟦️.ts";
/** 🧪️ Forms persisted fields and child identities agree with independent JSON Schema validation. */
import { join } from "node:path";
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
    dependencies: [ioSchema, childSchema],
    artifact: { schema: artifactSchema, parse: artifact.parseFormsArtifact },
    snapshot: { schema: snapshotSchema, parse: snapshot.parseFormsSnapshot },
    diff: { schema: diffSchema, parse: diff.parseFormsDiff },
    validDocuments: [{ input: vectors.document, output: vectors.document }, { input: base, output: base }, { input: { ...base, title: null }, output: base }],
    invalidDocuments: [...Object.entries(vectors.invalidDocumentFields).map(([key, value]) => ({ ...vectors.document, [key]: value })), ...vectors.invalidChildren.map((child) => ({ ...vectors.document, structure: child }))],
    mutationRoots: [join(import.meta.dir, "../../../🧫️fixtures/🧬️mutations")],
    committed: { snapshots: 20, diffs: 6 },
  });
}
