/** 🧾️ Applied mutation witnesses target the same row and value as their snapshot and diff. */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";
import { semioSchemaAjvV1 } from "@semio-tech/framework/schema/oracles/validator";
import vectors from "../🧫️fixtures/🧾️operation-consistency/🔣️.json" with { type: "json" };

const read = (path: string): any => JSON.parse(readFileSync(path, "utf8"));

for (const row of vectors.cases) test(`${row.tag} applies its declared edit`, async () => {
  const root = join(import.meta.dir, "../../🗿️artifacts", row.artifact, "🏅️standards/🔖️1/🪆️subsets/✳️any");
  const fixture = join(root, "🧪️tests/🧫️fixtures/🧬️mutations", row.leaf, "✅apply");
  const mutation = read(join(fixture, "🔣️.mutation.json"));
  const payload = mutation[row.tag];
  const snapshot = read(join(root, "🧬️schema/📸️snapshot/🔣️.json"));
  const leaf = read(join(root, "🧬️schema/🧬️mutations", row.leaf, "🧬️schema/🔣️.json"));
  const validate = semioSchemaAjvV1({ strict: true, allErrors: true }).addSchema(snapshot).compile(leaf);
  expect(validate(payload), JSON.stringify(validate.errors)).toBe(true);
  const before = read(join(fixture, "🔣️.before.json"))[row.collection].find((item: any) => item.id === payload.id);
  const after = read(join(fixture, "🔣️.after.json"))[row.collection].find((item: any) => item.id === payload.id);
  expect(before).toBeDefined();
  expect(after).toBeDefined();
  expect(Object.hasOwn(before, row.field)).toBe(true);
  expect(before[row.field]).not.toEqual(payload[row.argument]);
  expect(after[row.field]).toEqual(payload[row.argument]);
  const diff = read(join(fixture, "🔣️.diff.json"));
  const edit = diff[row.collection].modified.find((item: any) => item.id === payload.id);
  expect(edit?.patch[row.field]).toEqual(payload[row.argument]);
  const aggregate = read(join(root, "🧬️schema/🧬️mutations/🔣️.json"));
  const twin = await import(pathToFileURL(join(root, "🧬️schema/🧬️mutations/🟦️.ts")).href);
  expect(twin[`parse${aggregate.title}`](mutation)).toEqual(mutation);
});
