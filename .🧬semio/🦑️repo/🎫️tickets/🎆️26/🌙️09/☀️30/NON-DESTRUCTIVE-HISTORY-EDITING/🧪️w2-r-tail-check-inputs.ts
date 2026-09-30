/** 🔍️ W2-R tail: reads EVERY mutation leaf payload schema of the tail plugins (catalogued or not) through W1-D's `mutationInputDefs`, resolving
 * cross-document `$ref`s through every `$id` document under the catalogued scopes, and prints refusals plus (with `--verbose`) each input's control;
 * then compiles every leaf with the strict x-semio vocabulary Ajv oracle (`semioSchemaAjvV1`, strict mode). */
import { mutationInputDefs, InputSchemaError } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🟦️.ts";
import { semioSchemaAjvV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧪️tests/🧬️schema-oracle/🟦️.ts";
import { existsSync, readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";

const repo = "/Users/ueli/Documents/semio";
const roots = ["✒️writer", "➗️mathematical", "🌊️flow", "🌍️gis", "🌿️vcs", "🎞️animate", "🎪️demonstrator", "🎬️sequence", "🏭️process", "💠️lowpoly", "💡️reasoning", "📖️playbook", "📜️imperative", "🔱️trinity", "🕸️dag", "🪐️space", "🪵️sourcing"];
const catalog = JSON.parse(readFileSync(join(repo, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json"), "utf8")) as { scopes: Record<string, { path: string }> };
const documents = new Map<string, unknown>();
const seen = new Set<string>();
const index = (directory: string): void => {
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    const path = join(directory, entry.name);
    if (entry.isDirectory()) index(path);
    else if (entry.name.endsWith(".json") && !seen.has(path)) {
      seen.add(path);
      try {
        const document = JSON.parse(readFileSync(path, "utf8")) as { $id?: unknown };
        if (typeof document.$id === "string" && !documents.has(document.$id)) documents.set(document.$id, document);
      } catch {}
    }
  }
};
for (const scope of Object.values(catalog.scopes)) if (existsSync(join(repo, scope.path)) && statSync(join(repo, scope.path)).isDirectory()) index(join(repo, scope.path));
const leaves: string[] = [];
const walk = (directory: string): void => {
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    if (!entry.isDirectory() || entry.name === "node_modules" || entry.name === "target") continue;
    const path = join(directory, entry.name);
    const parts = path.split("/");
    if (entry.name === "🧬️schema" && parts.at(-3) === "🧬️mutations" && existsSync(join(path, "🔣️.json"))) leaves.push(join(path, "🔣️.json"));
    walk(path);
  }
};
for (const root of roots) walk(join(repo, "✏️s/🔌️plugins", root));
const verbose = process.argv.includes("--verbose");
let inputs = 0;
const refusals: Record<string, number> = {};
const describe = (def: any, depth = 0): string[] => {
  const schema = def.schema ?? {};
  const head = `${"  ".repeat(depth)}${def.id} ${def.label?.native?.en ?? "?"} / ${def.label?.native?.de ?? "?"} :: ${schema.kind}${def.presentation ? `(${def.presentation.kind})` : ""}${schema.kind === "reference" ? ` ${JSON.stringify({ kinds: schema.kinds, domain: schema.domain, granularity: schema.granularity, many: schema.many })}` : ""}`;
  const fields = schema.kind === "object" ? schema.fields : schema.kind === "array" && schema.items?.kind === "object" ? schema.items.fields : [];
  return [head, ...(fields ?? []).flatMap((field: any) => describe(field, depth + 1))];
};
for (const leaf of leaves.sort()) {
  const name = leaf.replace(`${repo}/✏️s/🔌️plugins/`, "").replace(/\/🏅️standards\/🔖️1\/🪆️subsets\/[^/]+/u, "").replace("/🧬️schema/🔣️.json", "");
  try {
    const defs = mutationInputDefs(readFileSync(leaf, "utf8"), (id) => documents.get(id));
    inputs += defs.length;
    if (verbose) console.log(`${name}\n${defs.flatMap((def) => describe(def, 1)).join("\n")}`);
  } catch (error) {
    if (!(error instanceof InputSchemaError)) throw error;
    refusals[error.code] = (refusals[error.code] ?? 0) + 1;
    console.log(`REFUSED ${name} ${error.message.slice(0, 200)}`);
  }
}
console.log(`leaves=${leaves.length} inputs=${inputs} refusals=${JSON.stringify(refusals)}`);
let compiled = 0;
const compileFailures: string[] = [];
for (const leaf of leaves) {
  const schema = JSON.parse(readFileSync(leaf, "utf8")) as { $id?: string };
  const ajv = semioSchemaAjvV1({ strict: true, loadSchema: async (uri: string) => (documents.get(uri.split("#")[0]!) as object | undefined) ?? Promise.reject(new Error(`unresolved ${uri}`)) });
  try {
    await ajv.compileAsync(schema.$id === undefined ? schema : { ...schema });
    compiled += 1;
  } catch (error) {
    compileFailures.push(`${leaf.replace(`${repo}/✏️s/🔌️plugins/`, "")}: ${(error as Error).message.slice(0, 160)}`);
  }
}
for (const failure of compileFailures) console.log(`AJV ${failure}`);
console.log(`strict-ajv compiled=${compiled} failed=${compileFailures.length}`);
