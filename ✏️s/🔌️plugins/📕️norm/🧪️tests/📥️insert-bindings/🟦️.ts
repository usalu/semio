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
  expect(new Set(fixture.rules.map(row => row.family)).size).toBe(15);
  expect(fixture.families).toBe(15);
  expect(new Set(fixture.rules.map(row => row.source + ":" + row.path)).size).toBe(234);
});

const payloads = new Map<string, any>();
const payloadOf = (path: string) => payloads.get(path) ?? (payloads.set(path, JSON.parse(readFileSync(join(root, path), "utf8"))), payloads.get(path));
const ajv = semioSchemaAjvV1({ strict: true, allErrors: true });
const validators = new Map<string, ReturnType<typeof ajv.compile>>();
const membership = (properties: string[], required: string[]) => {
  const key = JSON.stringify([properties, required]);
  if (!validators.has(key)) validators.set(key, ajv.compile({ type: "object", additionalProperties: false, properties: Object.fromEntries(properties.map(name => [name, true])), required }));
  return validators.get(key)!;
};
const bindings = new Map(actual.map(candidate => [candidate.source + "\0" + candidate.path, candidate]));

test("every native insert key matches its payload schema and independent strict Ajv membership oracle", () => {
  const failures: string[] = [];
  for (const row of fixture.rules) {
    const binding = bindings.get(row.source + "\0" + row.path);
    expect(binding, row.family + ":" + row.path).toBeDefined();
    if (!binding) continue;
    const payload = payloadOf(row.schema);
    const properties = payload.properties as Record<string, { const?: unknown; enum?: unknown[] }>;
    const required = (payload.required as string[]).filter(key => properties[key].const === undefined && properties[key].enum?.length !== 1);
    const keys = [...binding.selectors, binding.index, binding.item];
    const own = keys.every(key => Object.hasOwn(properties, key)) && required.every(key => keys.includes(key)) && new Set(keys).size === keys.length;
    const validate = membership(Object.keys(properties), required);
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
