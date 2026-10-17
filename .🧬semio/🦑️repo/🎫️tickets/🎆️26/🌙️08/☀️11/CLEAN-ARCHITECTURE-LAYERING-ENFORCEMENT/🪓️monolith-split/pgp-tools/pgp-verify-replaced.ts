/** 🔬 Ajv differential for the parsers the permanent projection regenerated: synthesizes minimal and full instances from the sibling JSON Schema and compares Ajv with the parser on every single-point mutation. Usage: bun pgp-verify-replaced.ts <root> <name@leaf>... */
import Ajv from "ajv";
import { readFileSync, readdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { pathToFileURL } from "node:url";

type Json = unknown;
const [root, ...targets] = process.argv.slice(2);

function allSchemas(base: string, out: { path: string; schema: any }[] = []): { path: string; schema: any }[] {
  for (const entry of readdirSync(base, { withFileTypes: true })) {
    if (entry.name === "node_modules" || entry.name === "target" || entry.name.startsWith(".")) continue;
    const path = join(base, entry.name);
    if (entry.isDirectory()) allSchemas(path, out);
    else if (entry.name === "🔣️.json" && /\/🧬️schema(\/|$)/.test(path)) {
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
const summary: string[] = [];
let failures = 0;
for (const target of targets) {
  const [name, leafRel] = target.split("@") as [string, string];
  const leaf = join(root!, leafRel);
  const document = JSON.parse(readFileSync(join(dirname(leaf), "🔣️.json"), "utf8"));
  const ajv = new Ajv({ strict: false, validateFormats: false, allErrors: false });
  ajv.addSchema(document);
  for (const { schema } of allSchemas(pluginRoot(leaf))) { try { if (schema.$id !== document.$id) ajv.addSchema(schema); } catch { } }
  const id = `${document.$id}#/$defs/${name}`;
  const validate = ajv.getSchema(id) ?? (document.title === name ? ajv.getSchema(document.$id) : undefined);
  const module = await import(pathToFileURL(leaf).href) as Record<string, (value: unknown) => unknown>;
  const parser = module[`parse${name}`]!;
  if (!validate || typeof parser !== "function") { summary.push(`${name}: MISSING validate=${!!validate} parser=${typeof parser}`); failures += 1; continue; }
  const stat = { cases: 0, agree: 0, stricter: [] as string[], lenientExtra: [] as string[], lenientOther: [] as string[] };
  for (const full of [false, true]) {
    const instance = synthesize(validate.schema, ajv, document.$id, full);
    if (!validate(instance)) { summary.push(`${name}: synthesized ${full ? "full" : "minimal"} instance is not valid for Ajv (skipped)`); continue; }
    let parsed: unknown;
    try { parsed = parser(instance); } catch (error) { summary.push(`${name}: parser REFUSES the ${full ? "full" : "minimal"} instance: ${(error as Error).message}`); failures += 1; continue; }
    if (JSON.stringify(parsed) !== JSON.stringify(instance)) summary.push(`${name}: parser output differs from the ${full ? "full" : "minimal"} instance`);
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
      stat.cases += 1;
      if (expected === accepted) stat.agree += 1;
      else if (expected && !accepted) stat.stricter.push(mutation.label);
      else if (mutation.extra) stat.lenientExtra.push(mutation.label);
      else stat.lenientOther.push(mutation.label);
    }
  }
  summary.push(`${name}: cases=${stat.cases} agree=${stat.agree} parser-stricter=${stat.stricter.length} lenient-unknown-key=${stat.lenientExtra.length} lenient-other=${stat.lenientOther.length}${stat.stricter.length ? ` stricter e.g. ${stat.stricter.slice(0, 3).join("; ")}` : ""}${stat.lenientOther.length ? ` LENIENT-OTHER e.g. ${stat.lenientOther.slice(0, 4).join("; ")}` : ""}`);
  if (stat.lenientOther.length) failures += 1;
}
console.log(summary.join("\n"));
console.log(`targets=${targets.length} failures=${failures}`);
