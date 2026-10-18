/** 🔬 spd (copy of the pst tool, scandir errors tolerated): Ajv differential for every parser listed in a JSON file of {name, path} rows (path = repo-relative leaf). Usage: bun pst-verify.ts <root> <rows.json> <out.json> [limit-per-leaf] */
import Ajv from "ajv";
import { readFileSync, readdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { pathToFileURL } from "node:url";

type Json = unknown;
const [root, rowsFile, outFile, limitArg] = process.argv.slice(2);
const rows = JSON.parse(readFileSync(rowsFile!, "utf8")) as { name: string; path: string }[];
const schemaCache = new Map<string, { path: string; schema: any }[]>();
const globalRoots = ["🧰️framework/🔨️modules/🌱️value", "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store", "🧰️framework/🔨️modules/🧬️schema"];
let globals: { path: string; schema: any }[] | undefined;

function allSchemas(base: string, out: { path: string; schema: any }[] = []): { path: string; schema: any }[] {
  let entries: ReturnType<typeof readdirSync<{ withFileTypes: true }>>;
  try { entries = readdirSync(base, { withFileTypes: true }); } catch { return out; }
  for (const entry of entries) {
    if (entry.name === "node_modules" || entry.name === "target" || entry.name.startsWith(".")) continue;
    const path = join(base, entry.name);
    if (entry.isDirectory()) allSchemas(path, out);
    else if (entry.name === "🔣️.json" && (/\/🧬️schema(\/|$)/.test(path) || path.includes("/🧰️framework/")) && !path.includes("/🧪️tests/")) {
      try { out.push({ path, schema: JSON.parse(readFileSync(path, "utf8")) }); } catch { }
    }
  }
  return out;
}

const pluginRoot = (leaf: string): string => {
  const parts = leaf.split("/");
  const at = parts.findIndex((part) => part === "🔌️plugins" || part === "🧩️compositions");
  return parts.slice(0, at + 2).join("/");
};

function synthesize(node: any, ajv: Ajv, base: string, full: boolean, depth = 0): Json {
  if (node === true || node === undefined || node === null) return null;
  if (node.$ref) {
    const target = node.$ref.startsWith("#") ? base + node.$ref : node.$ref;
    const found = ajv.getSchema(target);
    return found ? synthesize(found.schema, ajv, target.split("#")[0]!, full, depth + 1) : null;
  }
  if (node.const !== undefined) return node.const;
  if (Array.isArray(node.enum)) return node.enum[0];
  for (const key of ["oneOf", "anyOf"]) if (Array.isArray(node[key])) return synthesize(node[key].find((item: any) => item.type !== "null") ?? node[key][0], ajv, base, full, depth + 1);
  if (Array.isArray(node.allOf)) return node.allOf.map((item: any) => synthesize(item, ajv, base, full, depth + 1)).reduce((all: any, item: any) => ({ ...all, ...(item && typeof item === "object" ? item : {}) }), {});
  const kind = Array.isArray(node.type) ? node.type.find((item: string) => item !== "null") : node.type;
  if (kind === "string") return "x".repeat(Math.max(1, node.minLength ?? 1));
  if (kind === "integer" || kind === "number") return node.minimum ?? 0;
  if (kind === "boolean") return true;
  if (kind === "null") return null;
  if (kind === "array") {
    if (Array.isArray(node.items)) return node.items.map((item: any) => synthesize(item, ajv, base, full, depth + 1));
    const count = Math.max(node.minItems ?? 0, full && depth < 5 ? 1 : 0);
    return Array.from({ length: count }, () => synthesize(node.items, ajv, base, full, depth + 1));
  }
  if (kind === "object" || node.properties || node.additionalProperties) {
    const out: Record<string, unknown> = {};
    const keys = full && depth < 5 ? Object.keys(node.properties ?? {}) : node.required ?? [];
    for (const key of keys) out[key] = synthesize(node.properties?.[key], ajv, base, full, depth + 1);
    if (!node.properties && node.additionalProperties && typeof node.additionalProperties === "object" && depth < 6) out["k"] = synthesize(node.additionalProperties, ajv, base, full, depth + 1);
    return out;
  }
  return null;
}

type Path = (string | number)[];
function paths(value: Json, prefix: Path = [], out: Path[] = []): Path[] {
  out.push(prefix);
  if (Array.isArray(value)) value.forEach((item, index) => paths(item, [...prefix, index], out));
  else if (value && typeof value === "object") for (const [key, item] of Object.entries(value)) paths(item, [...prefix, key], out);
  return out;
}
const clone = <T,>(value: T): T => JSON.parse(JSON.stringify(value));
function setAt(value: Json, path: Path, replacement: Json | undefined, mode: "set" | "delete" | "extra"): Json {
  if (path.length === 0) return mode === "set" ? replacement : value;
  const copy = clone(value) as any;
  let node = copy;
  for (const step of path.slice(0, -1)) node = node[step];
  const last = path.at(-1)!;
  if (mode === "delete") { if (Array.isArray(node)) node.splice(last as number, 1); else delete node[last]; }
  else if (mode === "set") node[last] = replacement;
  return copy;
}
function addExtra(value: Json, path: Path): Json {
  const copy = clone(value) as any;
  let node = copy;
  for (const step of path) node = node[step];
  if (node && typeof node === "object" && !Array.isArray(node)) node["__unknown"] = 1;
  return copy;
}


const bads: Json[] = [null, "bad", -1, 0.5, true, [], {}];
type Result = { name: string; leaf: string; cases: number; agree: number; stricter: string[]; lenientExtra: string[]; lenientOther: string[]; notes: string[] };
const results: Result[] = [];
const cap = (list: string[], n = 4) => list.slice(0, n);
for (const row of rows) {
  const name = row.name;
  const leaf = join(root!, row.path);
  const res: Result = { name, leaf: row.path, cases: 0, agree: 0, stricter: [], lenientExtra: [], lenientOther: [], notes: [] };
  results.push(res);
  let document: any;
  try { document = JSON.parse(readFileSync(join(dirname(leaf), "🔣️.json"), "utf8")); } catch (e) { res.notes.push("no document: " + (e as Error).message); continue; }
  const ajv = new Ajv({ strict: false, validateFormats: false, allErrors: false });
  try { ajv.addSchema(document); } catch (e) { res.notes.push("addSchema: " + (e as Error).message); continue; }
  const base = pluginRoot(leaf);
  if (!schemaCache.has(base)) schemaCache.set(base, allSchemas(base));
  for (const { schema } of schemaCache.get(base)!) { try { if (schema.$id !== document.$id) ajv.addSchema(schema); } catch { } }
  globals ??= globalRoots.flatMap((r) => allSchemas(join(root!, r)));
  for (const { schema } of globals) { try { if (schema.$id !== document.$id && !ajv.getSchema(schema.$id)) ajv.addSchema(schema); } catch { } }
  let validate: any;
  try { validate = ajv.getSchema(`${document.$id}#/$defs/${name}`) ?? (document.title === name ? ajv.getSchema(document.$id) : undefined); } catch (e) { res.notes.push("compile: " + (e as Error).message.slice(0, 160)); continue; }
  let parser: any;
  try { parser = ((await import(pathToFileURL(leaf).href)) as Record<string, unknown>)[`parse${name}`]; } catch (e) { res.notes.push("import: " + (e as Error).message.slice(0, 160)); continue; }
  if (!validate || typeof parser !== "function") { res.notes.push(`MISSING validate=${!!validate} parser=${typeof parser}`); continue; }
  for (const full of [false, true]) {
    const instance = synthesize(validate.schema, ajv, document.$id, full);
    if (!validate(instance)) { res.notes.push(`synthesized ${full ? "full" : "minimal"} instance not valid for Ajv`); continue; }
    try { const parsed = parser(instance); if (JSON.stringify(parsed) !== JSON.stringify(instance)) res.notes.push(`parser output differs from the ${full ? "full" : "minimal"} instance`); } catch (error) { res.notes.push(`parser REFUSES the ${full ? "full" : "minimal"} valid instance: ${(error as Error).message.slice(0, 120)}`); continue; }
    const mutations: { label: string; value: Json; extra: boolean }[] = [];
    for (const path of paths(instance)) {
      for (const bad of bads) mutations.push({ label: `${JSON.stringify(path)}=${JSON.stringify(bad)}`, value: setAt(instance, path, bad, "set"), extra: false });
      if (path.length > 0) mutations.push({ label: `${JSON.stringify(path)} deleted`, value: setAt(instance, path, undefined, "delete"), extra: false });
      mutations.push({ label: `${JSON.stringify(path)} + __unknown`, value: addExtra(instance, path), extra: true });
    }
    for (const mutation of mutations) {
      if (JSON.stringify(mutation.value) === JSON.stringify(instance)) continue;
      const expected = validate(mutation.value);
      let accepted = true;
      try { parser(mutation.value); } catch { accepted = false; }
      res.cases += 1;
      if (expected === accepted) res.agree += 1;
      else if (expected && !accepted) res.stricter.push(mutation.label);
      else if (mutation.extra) res.lenientExtra.push(mutation.label);
      else res.lenientOther.push(mutation.label);
    }
  }
}
writeFileSync(outFile!, JSON.stringify(results, null, 1));
const sum = (f: (r: Result) => number) => results.reduce((a, r) => a + f(r), 0);
console.log(JSON.stringify({ parsers: results.length, cases: sum((r) => r.cases), agree: sum((r) => r.agree), stricter: sum((r) => r.stricter.length), lenientExtra: sum((r) => r.lenientExtra.length), lenientOther: sum((r) => r.lenientOther.length), withNotes: results.filter((r) => r.notes.length).length }));
