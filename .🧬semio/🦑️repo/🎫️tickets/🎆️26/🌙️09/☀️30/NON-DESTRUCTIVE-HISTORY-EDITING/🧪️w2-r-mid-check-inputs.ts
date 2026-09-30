/**
 * 🔍️ W2-R-mid: reads every mutation leaf payload schema of the wfc, block, fem and layout plugins and of the os-owned leaves
 * (catalogued or not, `🧫️fixtures` excluded) through W1-D's `mutationInputDefs`, one top-level input at a time like the
 * `schema-mutation-input-ui` lint, resolving cross-document `$ref`s through every `$id` document under the catalogued scopes.
 * Every accepted input must carry an explicit `x-semio-ui` label (no glossary fallback) and every `x-semio-ui` in the file must
 * validate against the manifest `$defs/InputUi` meta-schema (npm `jsonschema`).
 *
 *   bun 🧪️w2-r-mid-check-inputs.ts [--leaves <dir>] [--verbose]   (--leaves reads the leaf files from a preview copy)
 */
import { InputSchemaError, mutationInputDefs } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🟦️.ts";
import { Validator } from "jsonschema";
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";

const repo = "/Users/ueli/Documents/semio";
const roots = [
  "✏️s/🔌️plugins/🀄️wfc",
  "✏️s/🔌️plugins/🧱️block",
  "✏️s/🔌️plugins/🏗️fem",
  "✏️s/🔌️plugins/📏️layout",
  "🧰️framework/🛍️products/💻️os",
  "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config",
];
const leavesRoot = process.argv.includes("--leaves") ? process.argv[process.argv.indexOf("--leaves") + 1]! : repo;
const verbose = process.argv.includes("--verbose");
const catalog = JSON.parse(readFileSync(join(repo, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json"), "utf8")) as { scopes: Record<string, { path: string }> };
const documents = new Map<string, unknown>();
const scopePaths = new Set(Object.values(catalog.scopes).map((scope) => scope.path));
const index = (directory: string): void => {
  for (const entry of readdirSync(join(repo, directory), { withFileTypes: true })) {
    const path = `${directory}/${entry.name}`;
    if (entry.isDirectory() && !scopePaths.has(path)) index(path);
    else if (entry.isFile() && entry.name.endsWith(".json")) {
      try {
        const document = JSON.parse(readFileSync(join(repo, path), "utf8")) as { $id?: unknown };
        if (typeof document.$id === "string") documents.set(document.$id, document);
      } catch {}
    }
  }
};
for (const path of scopePaths) if (existsSync(join(repo, path))) index(path);

const manifest = JSON.parse(readFileSync(join(repo, "🧰️framework/🔨️modules/🛂️manifest/🧬️schema/🔣️.json"), "utf8")) as { $id: string };
const validator = new Validator();
validator.addSchema(manifest, manifest.$id);
const inputUi = { $ref: `${manifest.$id}#/$defs/InputUi` };

const leaves: string[] = [];
const walk = (directory: string): void => {
  if (!existsSync(join(repo, directory))) return;
  for (const entry of readdirSync(join(repo, directory), { withFileTypes: true })) {
    if (!entry.isDirectory() || entry.name === "🧫️fixtures" || entry.name === "node_modules") continue;
    const path = `${directory}/${entry.name}`;
    if (entry.name === "🧬️schema" && existsSync(join(repo, path, "🔣️.json")) && directory.split("/").at(-2) === "🧬️mutations") leaves.push(`${path}/🔣️.json`);
    walk(path);
  }
};
for (const root of roots) walk(root);

const tally = new Map<string, number>();
let inputs = 0;
let annotations = 0;
const bump = (code: string): void => void tally.set(code, (tally.get(code) ?? 0) + 1);
const annotationsOf = (node: unknown, at: string, found: [string, unknown][]): [string, unknown][] => {
  if (Array.isArray(node)) node.forEach((item, position) => annotationsOf(item, `${at}/${position}`, found));
  else if (node !== null && typeof node === "object") {
    for (const [key, value] of Object.entries(node)) {
      if (key === "x-semio-ui") found.push([at, value]);
      else annotationsOf(value, `${at}/${key}`, found);
    }
  }
  return found;
};
for (const path of leaves.sort()) {
  const leaf = JSON.parse(readFileSync(existsSync(join(leavesRoot, path)) ? join(leavesRoot, path) : join(repo, path), "utf8")) as Record<string, unknown>;
  for (const [at, value] of annotationsOf(leaf, "", [])) {
    annotations += 1;
    const result = validator.validate(value, inputUi);
    if (!result.valid) {
      bump("metaSchema");
      console.log(`META ${path} ${at}: ${result.errors.map((error) => error.stack).join("; ")}`);
    }
  }
  const properties = leaf.properties !== null && typeof leaf.properties === "object" ? Object.keys(leaf.properties as object) : [];
  for (const key of properties.length === 0 ? [null] : properties) {
    const single = key === null ? leaf : { ...leaf, properties: { [key]: (leaf.properties as Record<string, unknown>)[key] }, required: Array.isArray(leaf.required) ? (leaf.required as string[]).filter((name) => name === key) : [] };
    try {
      const defs = mutationInputDefs(single, (id) => documents.get(id));
      inputs += defs.length;
      if (verbose) for (const def of defs) console.log(`ok ${path.split("/🧬️mutations/").at(-1)} ${def.id} ${JSON.stringify(def.label).slice(0, 80)}`);
      const node = key === null ? undefined : ((leaf.properties as Record<string, Record<string, unknown>>)[key] ?? {});
      if (node !== undefined && !("const" in node) && (node["x-semio-ui"] as { label?: unknown } | undefined)?.label === undefined) {
        bump("unannotated");
        console.log(`UNANNOTATED ${path} /${key}`);
      }
    } catch (error) {
      if (!(error instanceof InputSchemaError)) throw error;
      inputs += 1;
      bump(error.code);
      console.log(`${error.code} ${path} ${error.pointer}: ${error.message}`);
    }
  }
}
console.log(`leaves=${leaves.length} inputs=${inputs} annotations=${annotations} findings=${JSON.stringify(Object.fromEntries(tally))}`);
