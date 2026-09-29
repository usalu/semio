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
import definitionSchema from "../../📝️definition/🔣️.json";
import responseSchema from "../../📨️response/🔣️.json";
import vectors from "./../../🧫️fixtures/🪪️document-contract/🔣️.json" with { type: "json" };
import importVectors from "../../🧫️fixtures/📥️import/🔣️.json";
import contactTemplate from "../../../🖼️assets/📇️contact/🔣️.json";

/** 📥️ Canonical import shares schema and identity checks with an independent JSON Schema oracle. */
export async function testFormsDesignImport(): Promise<void> {
  const { default: Ajv } = await import("ajv");
  const ajv = new Ajv({ strict: false });
  for (const schema of [ioSchema, childSchema, semioChildSchema, definitionSchema, responseSchema]) ajv.addSchema(schema);
  const validate = ajv.compile(artifactSchema);
  const unique = ajv.compile({ type: "array", items: { type: "string", minLength: 1 }, uniqueItems: true });
  assert.equal(validate(contactTemplate), true, JSON.stringify(validate.errors));
  assert.deepEqual(artifact.parseFormsArtifact(contactTemplate), contactTemplate);
  for (const item of importVectors.cases) {
    let parsed: unknown;
    try { parsed = JSON.parse(item.source); } catch { assert.equal(item.valid, false); continue; }
    let actual: artifact.FormsArtifact | undefined;
    try { actual = artifact.parseFormsArtifact(parsed); } catch {}
    assert.equal(actual !== undefined, item.valid, item.name);
    const row = parsed as artifact.FormsArtifact;
    const valid = validate(parsed) && unique(row.definition.steps.map(step => step.id)) && unique(row.definition.steps.flatMap(step => step.blocks.map(question => question.id)));
    assert.equal(valid, item.valid, `${item.name}: independent oracle`);
    if (actual) assert.deepEqual(JSON.parse(JSON.stringify(actual)), parsed, item.name);
  }
}

/** 🪪️ Checks exact child identity and the editor/document boundary. */
export function testFormsDocumentContractOracle(): void {
  const { title: _, ...base } = vectors.document;
  assertDocumentContractOracle({
    name: "Forms",
    dependencies: [ioSchema, childSchema, semioChildSchema, definitionSchema, responseSchema],
    childIdentityFields: ["structure", "results"],
    artifact: { schema: artifactSchema, parse: artifact.parseFormsArtifact },
    snapshot: { schema: snapshotSchema, parse: snapshot.parseFormsSnapshot },
    diff: { schema: diffSchema, parse: diff.parseFormsDiff },
    validDocuments: [{ input: vectors.document, output: vectors.document }, { input: base, output: base }, { input: { ...base, title: null }, output: base }],
    invalidDocuments: [...vectors.invalidIdentityDocuments, ...Object.entries(vectors.invalidDocumentFields).map(([key, value]) => ({ ...vectors.document, [key]: value })), ...vectors.invalidChildren.map((child) => ({ ...vectors.document, structure: child }))],
    invalidDiffs: vectors.invalidDiffs,
    validDiffs: vectors.patchCases.map((item) => ({ input: item.diff, output: item.diff })),
    mutationRoots: [fileURLToPath(new URL("../../../🧫️fixtures/🧬️mutations", import.meta.url))],
    committed: { snapshots: 24, diffs: 8 },
  });
  for (const item of vectors.patchCases) {
    const base = artifact.parseFormsArtifact(item.before);
    assert.deepEqual(applyFormsDiff(base, item.diff), item.after, item.name);
    assert.deepEqual(applyPatch(structuredClone(item.before), compare(item.before, item.after)).newDocument, item.after, item.name);
    for (const field of ["structure", "results"] as const) if (!Object.hasOwn(item.diff, field)) assert.equal(applyFormsDiff(base, item.diff)[field], base[field]);
  }
}

/** 🧬️ The aggregate schema accepts every committed native mutation and rejects unknown tags. */
export async function testFormsMutationSchemas(): Promise<void> {
  const { readdirSync, readFileSync } = await import("node:fs");
  const { join } = await import("node:path");
  const { default: Ajv } = await import("ajv");
  const { parse } = await import("graphql");
  const root = fileURLToPath(new URL("../../", import.meta.url));
  const ajv = new Ajv({ strict: false });
  ajv.addSchema(definitionSchema).addSchema(responseSchema);
  const mutationRoot = join(root, "🧬️mutations");
  for (const entry of readdirSync(mutationRoot, { withFileTypes: true })) {
    if (!entry.isDirectory()) continue;
    const path = join(mutationRoot, entry.name, "🧬️schema", "🔣️.json");
    try { ajv.addSchema(JSON.parse(readFileSync(path, "utf8"))); }
    catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error; }
  }
  const validate = ajv.compile(JSON.parse(readFileSync(join(mutationRoot, "🔣️.json"), "utf8")));
  const walk = (directory: string): string[] => readdirSync(directory, { withFileTypes: true }).flatMap(entry => entry.isDirectory() ? walk(join(directory, entry.name)) : [join(directory, entry.name)]);
  const native = walk(join(root, "../🧫️fixtures/🧬️mutations")).filter(path => path.endsWith("/🦠️mutation/🔣️.json")).map(path => JSON.parse(readFileSync(path, "utf8")));
  const responses = JSON.parse(readFileSync(join(root, "📨️response/🧫️fixtures/🔣️events.json"), "utf8")).cases.map((entry: { event: unknown }) => entry.event);
  for (const value of [...native, ...responses]) assert.equal(validate(value), true, JSON.stringify(validate.errors));
  assert.equal(validate({ mutation: "unknown", id: "q" }), false);
  for (const path of ["🔗️.graphql", "📸️snapshot/🔗️.graphql", "🔺️diff/🔗️.graphql", "📝️definition/🔗️.graphql", "📨️response/🔗️.graphql", "🧬️mutations/🔗️.graphql"]) parse(readFileSync(join(root, path), "utf8"));
}
