/** 🧪️ Norm document facets share child identities and reject editor state and whole-document diffs. */
import assert from "node:assert/strict";
import { readdirSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { assertDocumentContractOracle } from "../../../../../🧰️framework/🔨️modules/🧬️schema/🧪️testkit/🪪️document-contract/🟦️.ts";
import ioSchema from "../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json" with { type: "json" };
import childSchema from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🔣️.json" with { type: "json" };
import vectors from "./../../🧫️fixtures/🪪️document-contract/🔣️.json" with { type: "json" };

function snapshotFiles(root: string): string[] {
  return readdirSync(root, { withFileTypes: true }).flatMap((entry) => {
    const path = join(root, entry.name);
    return entry.isDirectory() ? snapshotFiles(path) : entry.name === "🔣️.json" && path.includes("/📸️snapshot/") ? [path] : [];
  });
}

function childKind(schema: Record<string, any>, field: string): unknown {
  const property = schema.properties[field];
  return property["x-semio-child-kind"] ?? property.anyOf?.[0]?.["x-semio-child-kind"];
}

/** ⚖️ Compares real native mutation fixtures with independent Ajv validation and production parsers. */
export async function testNormDocumentContractOracle(): Promise<void> {
  for (const owner of vectors.artifacts) {
    const root = join(dirname(fileURLToPath(import.meta.url)), "../../🗿️artifacts", owner.artifact, "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema");
    const facets = await Promise.all(["", "📸️snapshot", "🔺️diff"].map(async (path) => ({ schema: (await import(pathToFileURL(join(root, path, "🔣️.json")).href)).default, module: await import(pathToFileURL(join(root, path, "🟦️.ts")).href) })));
    assertDocumentContractOracle({
      name: owner.name,
      dependencies: [ioSchema, childSchema],
      artifact: { schema: facets[0].schema, parse: facets[0].module[`parse${owner.name}Artifact`] },
      snapshot: { schema: facets[1].schema, parse: facets[1].module[`parse${owner.name}Snapshot`] },
      diff: { schema: facets[2].schema, parse: facets[2].module[`parse${owner.name}Diff`] },
      validDocuments: [{ input: owner.document, output: owner.document }],
      invalidDocuments: [...Object.entries(owner.invalidFields).map(([field, value]) => ({ ...owner.document, [field]: value })), ...vectors.invalidChildren.map((child) => ({ ...owner.document, [owner.childField]: child }))],
      invalidDiffs: [{ artifact: owner.document }, ...vectors.invalidChildren.filter((child) => child !== null).map((child) => ({ [owner.childField]: child }))],
      mutationRoots: [join(root, "../🧫️fixtures/🧬️mutations")],
      committed: owner.committed,
    });
    for (const facet of facets) assert.equal(childKind(facet.schema, owner.childField), "s.stdio.semio");
    const files = snapshotFiles(join(root, "../🧫️fixtures/🧬️mutations"));
    assert.equal(files.length, owner.committed.snapshots);
    for (const path of files) {
      const value = facets[1].module[`parse${owner.name}Snapshot`](JSON.parse(readFileSync(path, "utf8")));
      const child = value[owner.childField];
      assert.equal(child.target.dialect.artifactKind, "s.stdio.semio", `${path}:${owner.childField} kind`);
      assert.equal(child.target.artifactId, child.childId, `${path}:${owner.childField} identity`);
    }
  }
}
