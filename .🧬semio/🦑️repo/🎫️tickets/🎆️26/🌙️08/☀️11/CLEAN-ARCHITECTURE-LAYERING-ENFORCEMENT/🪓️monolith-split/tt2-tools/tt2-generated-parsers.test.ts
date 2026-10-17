/** 🧪 Temporary oracle for the parsers the schema projection now emits: each must accept the schema's synthesized minimal instance and agree with Ajv on hostile probes. */
import { test, expect } from "bun:test";
import Ajv from "ajv";
import { readFileSync, readdirSync, statSync } from "node:fs";
import { dirname, join } from "node:path";

const root = "/Users/ueli/Documents/semio/";
const generated = readFileSync(join(root, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/tt2-gen-touched.txt"), "utf8").split("\n").filter(Boolean);
const snap = (index: number) => join(root, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/tt2-snap/gen", `${index + 1}.ts.pre`);
const names = (source: string) => new Set([...source.matchAll(/^export function parse([A-Za-z0-9]+)\(/gm)].map((match) => match[1]!));

function allSchemas(directory: string, out: object[] = []): object[] {
  for (const entry of readdirSync(directory)) {
    const path = join(directory, entry);
    if (entry === "🔣️.json") { try { out.push(JSON.parse(readFileSync(path, "utf8"))); } catch { } }
    else if (statSync(path).isDirectory() && !["node_modules", "🧪️tests", "📦️packages", "🧬️mutations"].includes(entry)) allSchemas(path, out);
  }
  return out;
}

function synthesize(node: any, ajv: Ajv, base: string, depth = 0): unknown {
  if (node === true || node === undefined) return null;
  if (node.$ref) {
    const target = node.$ref.startsWith("#") ? base + node.$ref : node.$ref;
    const found = ajv.getSchema(target);
    return found ? synthesize(found.schema, ajv, target.split("#")[0]!, depth + 1) : null;
  }
  if (depth > 4) return minimal(node);
  if (node.const !== undefined) return node.const;
  if (Array.isArray(node.enum)) return node.enum[0];
  for (const key of ["oneOf", "anyOf"]) if (Array.isArray(node[key])) return synthesize({ ...node[key].find((item: any) => item.type !== "null") ?? node[key][0], ...(node.type ? {} : {}) }, ajv, base, depth + 1);
  if (Array.isArray(node.allOf)) return node.allOf.map((item: any) => synthesize(item, ajv, base, depth + 1)).reduce((all: any, item: any) => ({ ...all, ...(item && typeof item === "object" ? item : {}) }), {});
  const kind = Array.isArray(node.type) ? node.type.find((item: string) => item !== "null") : node.type;
  if (kind === "string") return "x".repeat(Math.max(1, node.minLength ?? 1));
  if (kind === "integer" || kind === "number") return node.minimum ?? 0;
  if (kind === "boolean") return true;
  if (kind === "null") return null;
  if (kind === "array") {
    if (Array.isArray(node.items)) return node.items.map((item: any) => synthesize(item, ajv, base, depth + 1));
    return Array.from({ length: node.minItems ?? 0 }, () => synthesize(node.items, ajv, base, depth + 1));
  }
  if (kind === "object" || node.properties) {
    const out: Record<string, unknown> = {};
    for (const key of node.required ?? []) out[key] = synthesize(node.properties?.[key], ajv, base, depth + 1);
    return out;
  }
  return null;
}
const minimal = (node: any): unknown => (node.type === "array" ? [] : node.type === "object" ? {} : node.type === "string" ? "x" : 0);

const only = process.env.TT2_ONLY ? Number(process.env.TT2_ONLY) : undefined;
for (const [index, file] of generated.entries()) {
  if (only !== undefined && index !== only) continue;
  const absolute = join(root, file), before = names(readFileSync(snap(index), "utf8")), module = await import(absolute), added = [...names(readFileSync(absolute, "utf8"))].filter((name) => !before.has(name));
  const document = JSON.parse(readFileSync(join(dirname(absolute), "🔣️.json"), "utf8"));
  const moduleRoot = absolute.slice(0, absolute.indexOf("/🧬️schema/") + "/🧬️schema".length);
  const ajv = new Ajv({ strict: false, validateFormats: false, allErrors: false });
  for (const schema of allSchemas(moduleRoot)) { try { ajv.addSchema(schema as object); } catch { } }
  for (const name of added) {
    test(`${file.slice(file.indexOf("artifacts/") + 10, file.indexOf("/🏅️"))} ${file.includes("snapshot") ? "snapshot" : "diff"}: parse${name} agrees with Ajv`, () => {
      const target = `${document.$id}#/$defs/${name}`;
      const found = ajv.getSchema(target) ?? (document.title === name ? ajv.getSchema(document.$id) : undefined);
      expect(found, `no schema for ${name}`).toBeDefined();
      const parser = module[`parse${name}`] as (value: unknown) => unknown;
      expect(typeof parser).toBe("function");
      const valid = synthesize(found!.schema, ajv, document.$id);
      if (found!(valid)) expect(() => parser(valid)).not.toThrow();
      for (const probe of [42, null, "x", true]) {
        const expected = found!(probe);
        let accepted = true;
        try { parser(probe); } catch { accepted = false; }
        expect(accepted, `${name} on ${JSON.stringify(probe)}`).toBe(expected);
      }
    });
  }
}
