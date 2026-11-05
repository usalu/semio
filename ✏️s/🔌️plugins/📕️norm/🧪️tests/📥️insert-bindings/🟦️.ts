/** 📥️ Native insert bindings agree with the closed language-neutral corpus and strict schema key oracle. */
import { expect, test } from "bun:test";
import { readFileSync, readdirSync } from "node:fs";
import { join, relative } from "node:path";
import { semioSchemaAjvV1 } from "@semio-tech/framework/schema/oracles/validator";
import fixture from "./🧫️cases/🔣️.json" with { type: "json" };
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

const root = join(import.meta.dir, "../..");
const declaration = /InsertItemRule\s*\{\s*path:\s*"([^"]+)",\s*kind:\s*"([^"]+)",\s*selectors:\s*&\[([^\]]*)\],\s*index:\s*"([^"]+)",\s*item:\s*"([^"]+)"/g;
function ruleSources(directory: string): string[] {
  return readdirSync(directory, { withFileTypes: true }).flatMap(entry => {
    const path = join(directory, entry.name);
    return entry.isDirectory() ? ruleSources(path) : entry.name.endsWith(".rs") && path.includes("🧭️edit-rules") ? [path] : [];
  });
}
const sources = new Map(ruleSources(join(root, "🗿️artifacts")).map(path => [relative(root, path).replaceAll("\\", "/"), readFileSync(path, "utf8")]));
const actual = [...sources].flatMap(([source, text]) => [...text.matchAll(declaration)].map(match => ({
  source, path: match[1], kind: match[2],
  selectors: [...match[3].matchAll(/SelectorKey::(?:Id|Index)\("([^"]+)"\)/g)].map(item => item[1]),
  index: match[4], item: match[5],
})));

test("the closed insert corpus covers all fifteen cohorts and every declared binding", () => {
  const validate = semioSchemaAjvV1({ strict: true, allErrors: true }).compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  expect(validate({ ...fixture, extra: true })).toBe(false);
  expect(actual.length).toBe(fixture.rules.length);
  expect(new Set(fixture.rules.map(row => row.family)).size).toBe(14);
  expect(fixture.families).toBe(15);
  expect(new Set(fixture.rules.map(row => row.source + ":" + row.path)).size).toBe(227);
});

test("every native insert key matches its payload schema and independent strict Ajv membership oracle", () => {
  const failures: string[] = [];
  for (const row of fixture.rules) {
    const binding = actual.find(candidate => candidate.source === row.source && candidate.path === row.path);
    expect(binding, row.family + ":" + row.path).toBeDefined();
    if (!binding) continue;
    const payload = JSON.parse(readFileSync(join(root, row.schema), "utf8"));
    const properties = payload.properties as Record<string, { const?: unknown; enum?: unknown[] }>;
    const required = (payload.required as string[]).filter(key => properties[key].const === undefined && properties[key].enum?.length !== 1);
    const keys = [...binding.selectors, binding.index, binding.item];
    const own = keys.every(key => Object.hasOwn(properties, key)) && required.every(key => keys.includes(key)) && new Set(keys).size === keys.length;
    const validate = semioSchemaAjvV1({ strict: true, allErrors: true }).compile({
      type: "object", additionalProperties: false,
      properties: Object.fromEntries(Object.keys(properties).map(key => [key, true])),
      required,
    });
    const oracle = validate(Object.fromEntries(keys.map(key => [key, null])));
    const errors = JSON.stringify(validate.errors);
    expect(own, row.family + ":" + row.path + " oracle disagreement").toBe(oracle);
    expect(validate({ ...Object.fromEntries(required.map(key => [key, null])), unknownInsertBinding: null })).toBe(false);
    if (!own || binding.item !== row.item || binding.index !== row.index || JSON.stringify(binding.selectors) !== JSON.stringify(row.selectors)) {
      failures.push(row.family + ":" + row.path + " emits " + keys.join(",") + "; expected item " + row.item + "; " + errors);
    }
  }
  expect(failures).toEqual([]);
});
