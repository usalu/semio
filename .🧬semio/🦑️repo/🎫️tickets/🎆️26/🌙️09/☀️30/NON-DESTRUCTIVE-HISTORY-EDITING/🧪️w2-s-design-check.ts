/** 🔬️ W2-S-C third-party cross-check for the design group (shooting, puzzle 3d/5d, procedural, cad, raster, note, forms,
 * block, draw): compiles every leaf payload schema and every `#[derive(Mutations)]` aggregate document with the strict Ajv
 * oracle, validates every committed `🦠️mutation/🔣️.json` fixture's whole aggregate wire value against its owner's aggregate
 * document with Ajv (the lint itself uses npm `jsonschema`, so the two validators must agree), and reads every leaf through
 * W1-D's `mutationInputDefs`.
 *
 *   bun 🧪️w2-s-design-check.ts
 *
 * @see ./🧪️w2-s-design-parity-fix.py
 * @see ../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts */
import { mutationInputDefs } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🟦️.ts";
import { semioSchemaAjvV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧪️tests/🧬️schema-oracle/🟦️.ts";
import { existsSync, readdirSync, readFileSync } from "node:fs";

const repo = "/Users/ueli/Documents/semio";
const plugins = `${repo}/✏️s/🔌️plugins`;
const roots = ["🎥️shooting", "🧩️puzzle/🗿️artifacts/🧊️3d", "🧩️puzzle/🗿️artifacts/🖐️5d", "🌀️procedural", "📐️cad", "🖨️raster", "🗒️note", "📋️forms", "🧱️block", "🖍️draw"].map((root) => `${plugins}/${root}`);
const documents = new Map<string, Record<string, unknown>>();
const leaves: string[] = [];
const aggregates: string[] = [];
const fixtures: string[] = [];
const walk = (directory: string, collect: boolean): void => {
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    const path = `${directory}/${entry.name}`;
    if (entry.isDirectory() && !["node_modules", "target", "dist", "🗑️generated"].includes(entry.name)) walk(path, collect);
    else if (entry.isFile() && entry.name === "🔣️.json") {
      if (collect && path.endsWith("/🦠️mutation/🔣️.json") && /fixtures\//u.test(path)) fixtures.push(path);
      let document: Record<string, unknown>;
      try {
        document = JSON.parse(readFileSync(path, "utf8"));
      } catch {
        continue;
      }
      if (typeof document?.$schema !== "string") continue;
      if (typeof document.$id === "string") documents.set(document.$id, document);
      if (!collect || /fixtures\//u.test(path)) continue;
      if (/\/🧬️mutations\/[^/]+\/🧬️schema\/🔣️\.json$/u.test(path)) leaves.push(path);
      if (/\/🧬️schema\/🧬️mutations\/🔣️\.json$/u.test(path) && Array.isArray(document.oneOf)) aggregates.push(path);
    }
  }
};
for (const root of roots) walk(root, true);
walk(`${repo}/🧰️framework`, false);

const ajv = semioSchemaAjvV1({ strict: true, validateFormats: false, strictTypes: false, strictTuples: false });
for (const document of documents.values()) {
  try {
    ajv.addSchema(document);
  } catch {}
}
const failures: string[] = [];
const short = (path: string): string => path.replace(`${plugins}/`, "");
const compile = (path: string): ReturnType<typeof ajv.compile> | undefined => {
  const document = JSON.parse(readFileSync(path, "utf8"));
  if (document.$schema !== "http://json-schema.org/draft-07/schema#") failures.push(`dialect ${short(path)}: ${document.$schema}`);
  try {
    return ajv.compile(typeof document.$id === "string" ? { $ref: document.$id } : document);
  } catch (error) {
    failures.push(`ajv ${short(path)}: ${(error as Error).message}`);
    return undefined;
  }
};
let inputs = 0;
for (const path of leaves.sort()) {
  compile(path);
  try {
    inputs += mutationInputDefs(JSON.parse(readFileSync(path, "utf8")), (id) => documents.get(id)).length;
  } catch (error) {
    failures.push(`reader ${short(path)}: ${(error as Error).message}`);
  }
}
const validators = new Map(aggregates.map((path) => [path, compile(path)]));
let validated = 0;
for (const fixture of fixtures.sort()) {
  const owner = fixture.slice(0, fixture.lastIndexOf("/🧫️fixtures/"));
  const aggregate = `${owner}/🧬️schema/🧬️mutations/🔣️.json`;
  const validate = validators.get(aggregate);
  if (validate === undefined) {
    if (existsSync(aggregate) && aggregates.includes(aggregate)) failures.push(`uncompiled ${short(aggregate)}`);
    continue;
  }
  const raw = JSON.parse(readFileSync(fixture, "utf8"));
  const wire = raw !== null && typeof raw === "object" && "before" in raw && "after" in raw && typeof raw.mutation === "object" ? raw.mutation : raw;
  validated += 1;
  if (!validate(wire)) failures.push(`fixture ${short(fixture)}: ${ajv.errorsText(validate.errors?.slice(0, 3))}`);
}
for (const failure of failures) console.log(`FAIL ${failure}`);
console.log(`leaves=${leaves.length} inputs=${inputs} aggregates=${aggregates.length} fixtures=${validated}/${fixtures.length} failures=${failures.length}`);
process.exit(failures.length === 0 ? 0 : 1);
