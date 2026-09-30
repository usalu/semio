/**
 * 🧪️ W2-S stdio: for every non-glTF stdio mutation leaf (catalog `mutation-leaf` scopes) —
 * (1) the strict third-party Ajv oracle (`semioSchemaAjvV1`, registered `x-semio-ui` vocabulary) compiles the annotated leaf
 * whenever it compiled with every `x-semio-ui` stripped, (2) every annotation parses as `InputUi`, and (3) the framework reader
 * `mutationInputDefs` reads the WHOLE payload (all inputs at once) without a refusal other than `refUnresolved`.
 *
 * (4) every non-glTF stdio aggregate document compiles under the same strict oracle, and (5) every `x-semio-ui` of every stdio schema document (leaves and the shared snapshot contracts they reach) parses as
 * `InputUi`.
 *
 *   bun 🧪️w2-s-stdio-check.ts
 */
import { InputSchemaError, mutationInputDefs } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🟦️.ts";
import { parseInputUi } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🧬️schema/🟦️.ts";
import { semioSchemaAjvV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧪️tests/🧬️schema-oracle/🟦️.ts";
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";

const repo = "/Users/ueli/Documents/semio";
const mine = { has: (artifact: string): boolean => artifact !== "🧊️gltf" };
const catalog = JSON.parse(readFileSync(join(repo, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json"), "utf8")) as { scopes: Record<string, { path: string; level?: string; formats: Record<string, string> }> };
const scopePaths = new Set(Object.values(catalog.scopes).map((scope) => scope.path));
const documents = new Map<string, Record<string, unknown>>();
const index = (directory: string): void => {
  for (const entry of readdirSync(join(repo, directory), { withFileTypes: true })) {
    const path = `${directory}/${entry.name}`;
    if (entry.isDirectory() && !scopePaths.has(path)) index(path);
    else if (entry.isFile() && entry.name.endsWith(".json")) {
      try {
        const document = JSON.parse(readFileSync(join(repo, path), "utf8"));
        if (typeof document?.$id === "string") documents.set(document.$id, document);
      } catch {}
    }
  }
};
for (const path of scopePaths) if (existsSync(join(repo, path))) index(path);

const strip = (node: unknown): unknown => (Array.isArray(node) ? node.map(strip) : node !== null && typeof node === "object" ? Object.fromEntries(Object.entries(node).filter(([key]) => key !== "x-semio-ui").map(([key, value]) => [key, strip(value)])) : node);
const annotations = (node: unknown, found: unknown[]): unknown[] => {
  if (Array.isArray(node)) node.forEach((child) => annotations(child, found));
  else if (node !== null && typeof node === "object") for (const [key, child] of Object.entries(node)) key === "x-semio-ui" ? found.push(child) : annotations(child, found);
  return found;
};
const oracle = (stripped: boolean) => {
  const ajv = semioSchemaAjvV1({ strict: true });
  for (const [id, document] of documents) {
    if (typeof document.$schema === "string" && !document.$schema.includes("draft-07")) continue;
    try {
      if (ajv.getSchema(id) === undefined) ajv.addSchema((stripped ? strip(document) : document) as object, id);
    } catch {}
  }
  return ajv;
};
const annotatedAjv = oracle(false);
const strippedAjv = oracle(true);
const compiles = (ajv: ReturnType<typeof oracle>, id: string): string | null => {
  try {
    ajv.getSchema(id);
    return null;
  } catch (error) {
    return (error as Error).message.slice(0, 160);
  }
};

let leaves = 0;
let parsed = 0;
let inputs = 0;
const failures: string[] = [];
const unresolved: string[] = [];
const preexisting: string[] = [];
for (const [scopeId, scope] of Object.entries(catalog.scopes)) {
  const parts = scope.path.split("/");
  if (scope.level !== "mutation-leaf" || parts[2] !== "🗄️stdio" || parts[3] !== "🗿️artifacts" || !mine.has(parts[4]!)) continue;
  leaves += 1;
  const leaf = JSON.parse(readFileSync(join(repo, scope.path, scope.formats["🔣️jsonschema"] ?? "🔣️.json"), "utf8"));
  const id = leaf.$id as string;
  const before = compiles(strippedAjv, id);
  const after = compiles(annotatedAjv, id);
  if (before !== null) preexisting.push(`${scopeId}: ${before}`);
  else if (after !== null) failures.push(`ajv ${scopeId}: ${after}`);
  for (const annotation of annotations(leaf, [])) {
    try {
      parseInputUi(annotation);
      parsed += 1;
    } catch (error) {
      failures.push(`InputUi ${scopeId}: ${(error as Error).message}`);
    }
  }
  try {
    inputs += mutationInputDefs(leaf, (reference) => documents.get(reference)).length;
  } catch (error) {
    if (error instanceof InputSchemaError && error.code === "refUnresolved") unresolved.push(`${scopeId}: ${error.message}`);
    else failures.push(`reader ${scopeId}: ${(error as Error).message}`);
  }
}
let aggregates = 0;
for (const [id, document] of documents) {
  if (!/\/s\/stdio\/(?!gltf\/).*\/mutations\.json$/u.test(id) || !(Array.isArray(document.oneOf) || Array.isArray(document.allOf))) continue;
  aggregates += 1;
  const failure = compiles(annotatedAjv, id);
  if (failure !== null) failures.push(`ajv aggregate ${id}: ${failure}`);
}
console.log(`aggregatesCompiled=${aggregates}`);
let shared = 0;
for (const [id, document] of documents) {
  if (!id.includes("/s/stdio/") || id.includes("/s/stdio/gltf/")) continue;
  for (const annotation of annotations(document, [])) {
    try {
      parseInputUi(annotation);
      shared += 1;
    } catch (error) {
      failures.push(`InputUi ${id}: ${(error as Error).message}`);
    }
  }
}
console.log(`documentAnnotationsParsed=${shared}`);
console.log(`leaves=${leaves} inputs=${inputs} leafAnnotationsParsed=${parsed} failures=${failures.length} refUnresolved=${unresolved.length} ajvPreexisting=${preexisting.length}`);
for (const line of [...failures, ...unresolved.map((line) => `refUnresolved ${line}`), ...preexisting.map((line) => `ajv-preexisting ${line}`)]) console.log(`  ${line}`);
process.exit(failures.length === 0 ? 0 : 1);
