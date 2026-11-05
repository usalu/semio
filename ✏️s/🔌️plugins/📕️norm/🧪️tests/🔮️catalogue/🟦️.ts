/** 🔮️ Independent structured oracle registrations cover every current native mutation kind. */
import { expect, test } from "bun:test";
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { semioSchemaAjvV1 } from "@semio-tech/framework/schema/oracles/validator";
import fixture from "../../../../🧪️tests/🧪️mutation-vectors/🔮️catalogue/cases.json" with { type: "json" };
import schema from "../../../../🧪️tests/🧪️mutation-vectors/🔮️catalogue/schema.json" with { type: "json" };
const ajv = semioSchemaAjvV1({ strict: true, allErrors: true });
const structure = ajv.compile({ type: "object", required: ["mutationCatalogs"], properties: { mutationCatalogs: { type: "array", items: { type: "object", required: ["vectors"], properties: { vectors: { type: "array", items: { type: "object", required: ["mutationId"], properties: { mutationId: { type: "string", minLength: 1 } } } } } } } } });
function missing(kinds: readonly string[], manifest: { mutationCatalogs: { vectors: { mutationId: string }[] }[] }): string[] {
  const known = [...new Set(manifest.mutationCatalogs.flatMap(catalogue => catalogue.vectors.map(row => row.mutationId)))];
  const admits = ajv.compile({ type: "array", items: known.length ? { enum: known } : false });
  return kinds.filter(kind => !admits([kind]));
}
test("neutral catalogue cases reject narrative-only coverage and malformed registration members", () => {
  const validate = ajv.compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  expect(validate({ ...fixture, extra: true })).toBe(false);
  for (const row of fixture.cases) {
    const valid = structure(row.manifest);
    expect(valid, row.name).toBe(!row.error);
    if (valid) {
      if (!row.missing) throw new Error("Valid neutral catalogue case lacks expected missing kinds: " + row.name);
      expect(missing(row.kinds, row.manifest as Parameters<typeof missing>[1]), row.name).toEqual(row.missing);
    }
  }
});
test("all fifteen current native mutation rosters occur as actual oracle vectors", () => {
  const root = join(import.meta.dir, "../../🗿️artifacts");
  const entries = readdirSync(root).filter(name => /(?:en\d+|din\d+|iso\d+|vdi\d+)/.test(name));
  expect(entries).toHaveLength(15);
  const failures: string[] = [];
  for (const name of entries) {
    const subset = join(root, name, "🏅️standards/🔖️1/🪆️subsets/✳️any");
    const text = readFileSync(join(subset, "🧬️schema/🧬️mutations/🦀️.rs"), "utf8");
    const roster = text.match(/pub const KINDS:\s*&\[&str\]\s*=\s*&\[([\s\S]*?)\];/);
    expect(roster, name).not.toBeNull();
    const kinds = [...roster![1]!.matchAll(/"([^"]+)"/g)].map(row => row[1]!);
    const manifest = JSON.parse(readFileSync(join(subset, "🔮️oracles/🔣️.json"), "utf8"));
    expect(structure(manifest), JSON.stringify(structure.errors)).toBe(true);
    const absent = missing(kinds, manifest);
    if (absent.length) failures.push(name + ": " + absent.join(", "));
    expect(missing(["unregisteredMutationKind"], manifest)).toEqual(["unregisteredMutationKind"]);
  }
  expect(failures).toEqual([]);
});

test("each subset has one unambiguous catalogue in the exact native declaration order", () => {
  const root = join(import.meta.dir, "../../🗿️artifacts");
  const unique = ajv.compile({ type: "array", uniqueItems: true, items: { type: "string" } });
  const failures: string[] = [];
  for (const name of readdirSync(root).filter(name => /(?:en\d+|din\d+|iso\d+|vdi\d+)/.test(name))) {
    const subset = join(root, name, "🏅️standards/🔖️1/🪆️subsets/✳️any");
    const text = readFileSync(join(subset, "🧬️schema/🧬️mutations/🦀️.rs"), "utf8");
    const roster = text.match(/pub const KINDS:\s*&\[&str\]\s*=\s*&\[([\s\S]*?)\];/)!;
    const kinds = [...roster[1]!.matchAll(/"([^"]+)"/g)].map(row => row[1]!);
    const manifest = JSON.parse(readFileSync(join(subset, "🔮️oracles/🔣️.json"), "utf8"));
    const ids = manifest.mutationCatalogs.map((catalogue: { id: string }) => catalogue.id);
    if (!unique(ids)) failures.push(name + ": duplicate catalogue identity");
    for (const catalogue of manifest.mutationCatalogs) {
      if (JSON.stringify(catalogue.kinds) !== JSON.stringify(kinds)) failures.push(name + ": incomplete or unordered kinds");
      if (JSON.stringify(catalogue.vectors.map((row: { mutationId: string }) => row.mutationId)) !== JSON.stringify(kinds)) failures.push(name + ": incomplete or unordered vectors");
    }
  }
  expect(failures).toEqual([]);
});
