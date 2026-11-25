/** 🌱️ An independent oracle of the default row of an empty collection: derived from the item schema of every insert mutation alone (required fields, schema defaults, first enum option, minimum numbers, minimum item counts), the row must satisfy the strict item schema, so no collection of any norm family is left without a valid first row. */
import { expect, test } from "bun:test";
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { semioSchemaAjvV1 } from "@semio-tech/framework/schema/oracles/validator";
import fixture from "./🧫️cases/🔣️.json" with { type: "json" };

const root = join(import.meta.dir, "../..");
const files = (directory: string): string[] => readdirSync(directory, { withFileTypes: true }).flatMap(entry => {
  const path = join(directory, entry.name);
  return entry.isDirectory() ? (entry.name === "node_modules" ? [] : files(path)) : entry.name.endsWith(".json") && path.includes("🧬️schema") ? [path] : [];
});
const documents = new Map<string, any>();
for (const path of files(join(root, "🗿️artifacts"))) {
  let document: any;
  try { document = JSON.parse(readFileSync(path, "utf8")); } catch { continue; }
  if (typeof document?.$id === "string" && !documents.has(document.$id)) documents.set(document.$id, document);
}

function resolve(schema: any, base: any): [any, any] {
  if (typeof schema?.$ref !== "string") return [schema, base];
  const [url, pointer = ""] = schema.$ref.split("#");
  const document = url === "" ? base : documents.get(url);
  expect(document, "unresolved reference " + schema.$ref).toBeDefined();
  let target: any = document;
  for (const key of pointer.split("/").filter(Boolean)) target = target?.[key];
  expect(target, "unresolved pointer " + schema.$ref).toBeDefined();
  return resolve(target, document);
}

function seed(raw: any, base: any, depth = 0): unknown {
  expect(depth, "schema recursion").toBeLessThan(40);
  const [schema, home] = resolve(raw, base);
  if (schema.default !== undefined) return schema.default;
  if (schema.const !== undefined) return schema.const;
  if (schema.enum) return schema.enum[0];
  const branches = schema.oneOf ?? schema.anyOf;
  if (branches) return seed(branches[0], home, depth + 1);
  const type = Array.isArray(schema.type) ? schema.type.find((name: string) => name !== "null") ?? "null" : schema.type;
  if (type === "object" || schema.properties) {
    const row: Record<string, unknown> = {};
    for (const key of schema.required ?? []) row[key] = seed(schema.properties?.[key] ?? {}, home, depth + 1);
    return row;
  }
  if (type === "array") return Array.from({ length: schema.minItems ?? 0 }, () => seed(schema.items ?? {}, home, depth + 1));
  if (type === "string") return "";
  if (type === "boolean") return false;
  if (type === "integer" || type === "number") {
    const low = schema.minimum !== undefined ? schema.minimum : schema.exclusiveMinimum !== undefined ? schema.exclusiveMinimum + 1 : Math.min(schema.maximum ?? 0, 0);
    return type === "integer" ? Math.ceil(low) : low;
  }
  return null;
}

const ajv = semioSchemaAjvV1({ strict: false, allErrors: true });
for (const document of documents.values()) { try { ajv.addSchema(document); } catch { } }

test("every insert mutation of every norm cohort yields a default row the strict item schema accepts", () => {
  const failures: string[] = [];
  for (const rule of fixture.rules) {
    const payload = JSON.parse(readFileSync(join(root, rule.schema), "utf8"));
    const item = payload.properties[rule.item];
    const row = seed(item, payload);
    const validate = ajv.compile(item);
    if (!validate(row)) failures.push(rule.family + ":" + rule.path + " " + JSON.stringify(row).slice(0, 120) + " " + JSON.stringify(validate.errors?.slice(0, 2)));
  }
  expect(failures).toEqual([]);
});

test("a row without required members is refused by the same schemas", () => {
  const rule = fixture.rules[0];
  const payload = JSON.parse(readFileSync(join(root, rule.schema), "utf8"));
  const item = payload.properties[rule.item];
  if (resolve(item, payload)[0].required?.length) expect(ajv.compile(item)({})).toBe(false);
});
