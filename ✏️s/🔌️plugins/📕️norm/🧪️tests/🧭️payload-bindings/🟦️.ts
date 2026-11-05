/** 🎒️ Every native editor binding emits the complete key vocabulary of its canonical payload schema. */
import { expect, test } from "bun:test";
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { semioSchemaAjvV1 } from "@semio-tech/framework/schema/oracles/validator";
import fixture from "./🧫️cases/🔣️.json" with { type: "json" };
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

const root = join(import.meta.dir, "../../🗿️artifacts");
function files(directory: string): string[] {
  return readdirSync(directory, { withFileTypes: true }).flatMap(entry => {
    const path = join(directory, entry.name);
    return entry.isDirectory() ? ["🧪️tests", "📦️packages", "node_modules", "target"].includes(entry.name) ? [] : files(path) : path.endsWith(".rs") && path.includes("🧭️edit-rules") || entry.name === "🔣️.json" && path.includes("/🧬️mutations/") ? [path] : [];
  });
}
const sourceFiles = files(root);
const payloads = new Map(sourceFiles.filter(path => path.endsWith(".json")).flatMap(path => {
  const value = JSON.parse(readFileSync(path, "utf8"));
  const id = value.$id?.match(/\/s\/norm\/([^/]+)\/mutation\/([^/]+)\/schema\.json$/);
  return id ? [[id[1] + ":" + id[2], value] as const] : [];
}));
const source = sourceFiles.filter(path => path.endsWith(".rs")).map(path => ({
  path, family: path.slice(root.length + 1).split(/[\\/]/)[0]!.match(/(?:en\d+|din\d+|iso\d+|vdi\d+)/)![0], text: readFileSync(path, "utf8"),
}));
const field = (text: string, name: string) => text.match(new RegExp("\\b" + name + ':\\s*"([^"]*)"'))?.[1];
const argumentsActual = source.flatMap(row => [...row.text.matchAll(/\bSetFieldArgument\s*\{([^}]+)\}/g)].map(match => {
  const body = match[1]!, source = body.match(/source:\s*(?:\w+::)*(Text|Sibling)\(\s*"([^"]*)"\s*\)/);
  return { family: row.family, path: field(body, "path"), kind: field(body, "kind"), argument: field(body, "argument"), source: source?.[1] === "Text" ? { type: "text", value: source[2] } : source?.[1] === "Sibling" ? { type: "sibling", path: source[2] } : null };
}));
const bindings = source.flatMap(row => [...row.text.matchAll(/\b(SetFieldRule|InsertItemRule|RemoveItemRule|MoveItemRule|IdRemoval)\s*\{([^}]+)\}/g)].filter(match => field(match[2]!, "path") !== undefined).map(match => {
  const body = match[2]!, fields = Object.fromEntries([...body.matchAll(/\b(\w+):\s*("[^"]*"|&\[[^\]]*\])/g)].map(value => [value[1]!, value[1] === "selectors" ? [...value[2]!.matchAll(/SelectorKey::(?:Id|Index)\("([^"]+)"\)/g)].map(item => item[1]) : JSON.parse(value[2]!)]));
  return { family: row.family, type: match[1]!, fields, source: row.path };
}));
const fieldNames = {
  SetFieldRule: ["path", "kind", "selectors", "value"],
  InsertItemRule: ["path", "kind", "selectors", "index", "item"],
  RemoveItemRule: ["path", "kind", "selectors", "index"],
  MoveItemRule: ["path", "kind", "selectors", "from", "to"],
  IdRemoval: ["path", "kind", "selectors", "id"],
} as const;

test("the complete fifteen-family rule declarations and companion arguments have closed shapes", () => {
  const ajv = semioSchemaAjvV1({ strict: true, allErrors: true });
  const validate = ajv.compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  expect(validate({ ...fixture, extra: true })).toBe(false);
  const counts = Object.fromEntries(Object.keys(fixture.familyCounts).map(family => [family, bindings.filter(row => row.family === family).length]));
  expect(counts).toEqual(fixture.familyCounts);
  const shapeErrors: string[] = [];
  for (const [type, names] of Object.entries(fieldNames)) {
    const valid = ajv.compile({ type: "object", additionalProperties: false, required: names, properties: Object.fromEntries(names.map(name => [name, name === "selectors" ? { type: "array", items: { type: "string" } } : { type: "string" }])) });
    for (const row of bindings.filter(row => row.type === type)) if (!valid(row.fields)) shapeErrors.push(row.family + ":" + row.fields.path + ":" + JSON.stringify(valid.errors));
  }
  expect(shapeErrors).toEqual([]);
  const identity = (row: { family: string; path?: string; argument?: string }) => row.family + ":" + row.path + ":" + row.argument;
  expect(argumentsActual.toSorted((a, b) => identity(a).localeCompare(identity(b)))).toEqual([...fixture.arguments].sort((a, b) => identity(a).localeCompare(identity(b))));
});

test("all native binding keys satisfy their exact payload vocabulary and independent strict Ajv key-set oracle", () => {
  const ajv = semioSchemaAjvV1({ strict: true, allErrors: true }), validators = new Map<string, ReturnType<typeof ajv.compile>>();
  const failures: string[] = [];
  for (const row of bindings) {
    const payload = payloads.get(row.family + ":" + row.fields.kind);
    expect(payload, row.family + ":" + row.fields.path).toBeDefined();
    if (!payload) continue;
    const fields = row.fields;
    const own = Object.keys(payload.properties), required = (payload.required ?? []).filter((key: string) => payload.properties[key].const === undefined && payload.properties[key].enum?.length !== 1);
    const values = ({ SetFieldRule: [fields.value], InsertItemRule: [fields.index, fields.item], RemoveItemRule: [fields.index], MoveItemRule: [fields.from, fields.to], IdRemoval: [fields.id] } as Record<string, unknown[]>)[row.type]!;
    const keys = [...fields.selectors, ...values.filter(key => typeof key === "string" && key.length), ...argumentsActual.filter(extra => extra.family === row.family && extra.path === fields.path && extra.kind === fields.kind).map(extra => extra.argument)];
    const model = keys.every(key => own.includes(key)) && required.every((key: string) => keys.includes(key)) && new Set(keys).size === keys.length;
    const signature = JSON.stringify({ own: [...own].sort(), required: [...required].sort() });
    let validate = validators.get(signature);
    if (!validate) {
      validate = ajv.compile({ type: "array", uniqueItems: true, items: { enum: own }, ...(required.length ? { allOf: required.map((key: string) => ({ contains: { const: key } })) } : {}) });
      validators.set(signature, validate);
    }
    expect(validate(keys), row.family + ":" + fields.path + ": oracle disagreement").toBe(model);
    expect(validate([...keys, "unknownPayloadArgument"])).toBe(false);
    if (!model) failures.push(row.family + ":" + fields.path + ":" + JSON.stringify(keys) + ":" + JSON.stringify(validate.errors));
  }
  expect(failures).toEqual([]);
}, 120_000);

test("the independent native payload schemas admit the declared composite-field witnesses", () => {
  const ajv = semioSchemaAjvV1({ strict: true, allErrors: true });
  for (const row of fixture.cases) {
    if (!row.expected) continue;
    const payload = payloads.get("en1999:" + row.expected.kind);
    const implicit = Object.fromEntries(Object.entries(payload.properties).filter(([, value]) => (value as { const?: unknown }).const !== undefined).map(([key, value]) => [key, (value as { const: unknown }).const]));
    const validate = ajv.compile(payload);
    expect(validate({ ...implicit, ...row.expected.payload }), JSON.stringify(validate.errors)).toBe(true);
    const missing = { ...implicit, ...row.expected.payload } as Record<string, unknown>;
    delete missing[row.rules === "axis" ? "axis" : "newBoltsPerRow"];
    expect(validate(missing)).toBe(false);
  }
});
