/** 🔍️ W2-R design group: reads every leaf payload schema of puzzle 3d/5d, shooting, cad, procedural, note, forms, raster and draw
 * (catalogued or not) through W1-D's `mutationInputDefs`, resolving `$ref`s over every schema document under those plugins and the
 * framework, and compiles every leaf with the strict Ajv oracle (`x-semio-*` vocabulary). Prints the widget census; `--verbose`
 * lists every input.
 *
 *   bun 🧪️w2-r-design-check-inputs.ts [--verbose]
 *
 * @see ./🧪️w2-r-design-annotate-inputs.py
 * @see ../../../../../../../🧰️framework/🔨️modules/🛂️manifest/🟦️.ts */
import { mutationInputDefs } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🟦️.ts";
import { semioSchemaAjvV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧪️tests/🧬️schema-oracle/🟦️.ts";
import { readdirSync, readFileSync } from "node:fs";

const repo = "/Users/ueli/Documents/semio";
const plugins = `${repo}/✏️s/🔌️plugins`;
const roots = [`${plugins}/🧩️puzzle/🗿️artifacts/🧊️3d`, `${plugins}/🧩️puzzle/🗿️artifacts/🖐️5d`, `${plugins}/🎥️shooting`, `${plugins}/📐️cad`, `${plugins}/🌀️procedural`, `${plugins}/🗒️note`, `${plugins}/📋️forms`, `${plugins}/🖨️raster`, `${plugins}/🖍️draw`];
const documents = new Map<string, Record<string, unknown>>();
const leaves: string[] = [];
const walk = (directory: string): void => {
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    const path = `${directory}/${entry.name}`;
    if (entry.isDirectory() && entry.name !== "🧫️fixtures" && entry.name !== "node_modules") walk(path);
    else if (entry.isFile() && entry.name === "🔣️.json") {
      let document: Record<string, unknown>;
      try {
        document = JSON.parse(readFileSync(path, "utf8"));
      } catch {
        continue;
      }
      if (typeof document?.$schema !== "string") continue;
      if (typeof document.$id === "string") documents.set(document.$id, document);
      if (/\/🧬️mutations\/[^/]+\/🧬️schema\/🔣️\.json$/u.test(path)) leaves.push(path);
    }
  }
};
for (const root of roots) walk(root);
const leafCount = leaves.length;
walk(`${repo}/🧰️framework`);
leaves.length = leafCount;

const ajv = semioSchemaAjvV1({ strict: true, validateFormats: false, strictTypes: false, strictTuples: false });
for (const document of documents.values()) {
  try {
    ajv.addSchema(document);
  } catch {}
}
const widgets = new Map<string, number>();
let inputs = 0;
const failures: string[] = [];
const count = (key: string): void => void widgets.set(key, (widgets.get(key) ?? 0) + 1);
const tally = (defs: readonly any[], leaf: string, prefix = ""): void => {
  for (const def of defs) {
    inputs += 1;
    const kind = def.presentation?.kind ?? def.schema?.kind;
    count(kind);
    if (process.argv.includes("--verbose")) console.log(`${leaf}\t${prefix}${def.id}\t${kind}\t${def.label?.native?.en ?? ""}`);
    if (def.schema?.kind === "object") tally(def.schema.fields ?? [], leaf, `${prefix}${def.id}`);
    if (def.schema?.kind === "array" && def.schema.items?.kind === "object") tally(def.schema.items.fields ?? [], leaf, `${prefix}${def.id}/-`);
  }
};
for (const path of leaves.sort()) {
  const leaf = path.replace(`${plugins}/`, "");
  const document = JSON.parse(readFileSync(path, "utf8"));
  try {
    tally(mutationInputDefs(document, (id) => documents.get(id)), leaf);
  } catch (error) {
    failures.push(`reader ${leaf}: ${(error as Error).message}`);
  }
  try {
    ajv.compile(typeof document.$id === "string" ? { $ref: document.$id } : document);
  } catch (error) {
    failures.push(`ajv ${leaf}: ${(error as Error).message}`);
  }
}
for (const failure of failures) console.log(`FAIL ${failure}`);
console.log(`leaves=${leaves.length} inputs=${inputs} (nested included) failures=${failures.length}`);
console.log([...widgets.entries()].sort((left, right) => right[1] - left[1]).map(([kind, total]) => `${kind}=${total}`).join(" "));
process.exit(failures.length === 0 ? 0 : 1);
