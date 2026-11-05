/** 🛡️ The shared scalar-admission law and every family's owned mutation vocabulary. */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { semioSchemaAjvV1 } from "@semio-tech/framework/schema/oracles/validator";
import fixture from "../🧫️fixtures/🛡️inspector-admission/🔣️.json" with { type: "json" };
import schema from "../🧫️fixtures/🛡️inspector-admission/🧬️.json" with { type: "json" };

const plugin = join(import.meta.dir, "../..");
const surface = join(plugin, "📇️registry/🧬️contract/🖥️app-surface");
const read = (path: string) => readFileSync(path, "utf8");

test("the language-neutral inspector law is strict and covers every family and scalar state", () => {
  const validate = semioSchemaAjvV1({ strict: true, allErrors: true }).compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  expect(new Set(fixture.families.map(row => row.id)).size).toBe(15);
  expect(validate({ ...fixture, unexpected: true })).toBe(false);
  expect(validate({ ...fixture, fields: [{ ...fixture.fields[0], editable: "true" }] })).toBe(false);
});

test("scalar metadata carries explicit read-only admission through the actual control pipeline", () => {
  expect(read(join(surface, "🔖️field-meta/🦀️.rs"))).toContain("pub read_only: bool");
  expect(read(join(surface, "🪟️visual-windows/🧾️properties/🎛️controls/🦀️.rs"))).toContain("if field.read_only");
});

for (const row of fixture.families) test(row.id + " binds its field metadata to its own native setter vocabulary", () => {
  const source = read(join(plugin, row.provider));
  const signature = "pub fn " + row.function + "(path: &str)";
  const at = source.indexOf(signature);
  expect(at).toBeGreaterThanOrEqual(0);
  const body = source.slice(at, source.indexOf("\n}", at) + 2);
  expect(body).toContain("admit_field_meta(");
  expect(body).toContain(row.mutations + "::EDIT_RULES.can_set_field(path)");
});
