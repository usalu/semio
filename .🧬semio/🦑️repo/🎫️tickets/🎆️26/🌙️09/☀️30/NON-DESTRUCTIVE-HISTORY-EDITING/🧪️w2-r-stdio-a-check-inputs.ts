/** 🔍️ W2-R stdio-a: compiles every annotated leaf payload schema of the group with the strict Ajv oracle (x-semio vocabulary), once
 * as written and once with every `x-semio-ui` stripped (the pre-rollout shape), parses every annotation with `parseInputUi`, and reads
 * every leaf with the framework reader `mutationInputDefs`. Run: `bun 🧪️w2-r-stdio-a-check-inputs.ts`. */
import { mutationInputDefs } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🟦️.ts";
import { parseInputUi } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🧬️schema/🟦️.ts";
import { semioSchemaAjvV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧪️tests/🧬️schema-oracle/🟦️.ts";
import { existsSync, readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";

const repo = "/Users/ueli/Documents/semio";
const base = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/";
const artifacts = ["📼️avi", "💬️bcf", "💾️binary", "🪟️bmp", "🏃️commands", "🛂️contract", "📊️csv", "🗜️deflate", "📜️docx", "🖊️dwg", "🖋️dxf", "🌦️epw", "🎞️gif", "🧊️gltf", "🕸️graph", "🌐️html", "🏗️ifc"];
type Json = null | boolean | number | string | Json[] | { [key: string]: Json };
const catalog = JSON.parse(readFileSync(join(repo, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json"), "utf8")) as { scopes: Record<string, { path: string; level?: string }> };
const leaves = Object.values(catalog.scopes).filter((scope) => scope.level === "mutation-leaf" && artifacts.some((artifact) => scope.path.startsWith(base + artifact + "/"))).map((scope) => `${scope.path}/🔣️.json`).sort();
const documents = new Map<string, Json>();
const index = (directory: string): void => {
  for (const entry of readdirSync(join(repo, directory))) {
    const path = `${directory}/${entry}`;
    if (statSync(join(repo, path)).isDirectory()) index(path);
    else if (entry.endsWith(".json")) {
      try {
        const document = JSON.parse(readFileSync(join(repo, path), "utf8")) as { $id?: unknown };
        if (typeof document.$id === "string" && !documents.has(document.$id)) documents.set(document.$id, document as Json);
      } catch {}
    }
  }
};
for (const artifact of ["📼️avi", "💬️bcf", "💾️binary", "🪟️bmp", "📊️csv", "🗜️deflate", "📜️docx", "🖊️dwg", "🖋️dxf", "🌦️epw", "🎞️gif", "🧊️gltf", "🌐️html", "🏗️ifc", "📰️xml"]) index(`${base}${artifact}`);
const leafIds = new Set(leaves.map((leaf) => (JSON.parse(readFileSync(join(repo, leaf), "utf8")) as { $id?: string }).$id));
const strip = (node: Json): Json => (Array.isArray(node) ? node.map(strip) : node !== null && typeof node === "object" ? Object.fromEntries(Object.entries(node).filter(([key]) => key !== "x-semio-ui").map(([key, value]) => [key, strip(value)])) : node);
const oracle = (transform: (node: Json) => Json) => {
  const ajv = semioSchemaAjvV1({ strict: true, strictTypes: false, strictTuples: false, strictRequired: false, validateFormats: false });
  for (const keyword of ["x-semio", "x-semio-mutation"]) if (ajv.getKeyword(keyword) === false) ajv.addKeyword({ keyword });
  for (const [id, document] of documents) {
    if (leafIds.has(id) || String((document as { $schema?: unknown }).$schema ?? "").includes("2020-12")) continue;
    try {
      ajv.addSchema(transform(document) as object, id);
    } catch (error) {
      console.log("SKIP-DOCUMENT", id, (error as Error).message.slice(0, 120));
    }
  }
  return ajv;
};
const annotated = oracle((node) => node);
const stripped = oracle(strip);
let annotations = 0;
let uiFaults = 0;
let newFaults = 0;
let staleFaults = 0;
let inputs = 0;
let readerFaults = 0;
const visit = (node: Json, where: string): void => {
  if (Array.isArray(node)) node.forEach((child) => visit(child, where));
  else if (node !== null && typeof node === "object")
    for (const [key, child] of Object.entries(node)) {
      if (key === "x-semio-ui") {
        annotations += 1;
        try {
          parseInputUi(child);
        } catch (error) {
          uiFaults += 1;
          console.log("PARSE", where, (error as Error).message);
        }
      } else visit(child, where);
    }
};
for (const leaf of leaves) {
  if (!existsSync(join(repo, leaf))) continue;
  const document = JSON.parse(readFileSync(join(repo, leaf), "utf8")) as Json;
  visit(document, leaf);
  const compile = (ajv: ReturnType<typeof semioSchemaAjvV1>, schema: Json): string | null => {
    try {
      ajv.compile(schema as object);
      return null;
    } catch (error) {
      return (error as Error).message.slice(0, 160);
    }
  };
  const after = compile(annotated, document);
  const before = compile(stripped, strip(document));
  if (after !== null && before === null) {
    newFaults += 1;
    console.log("NEW-AJV", leaf.slice(base.length), after);
  } else if (after !== null) {
    staleFaults += 1;
    console.log("PRE-EXISTING-AJV", leaf.slice(base.length), after);
  }
  try {
    inputs += mutationInputDefs(document, (id) => documents.get(id)).length;
  } catch (error) {
    readerFaults += 1;
    console.log("READER", leaf.slice(base.length), (error as Error).message.slice(0, 160));
  }
}
for (const [id, document] of documents) if (id.includes("/stdio/") && !id.includes("/xml/") && !leafIds.has(id)) visit(document, id);
console.log(`leaves=${leaves.length} topLevelInputs=${inputs} readerFaults=${readerFaults} annotations=${annotations} parseInputUiFaults=${uiFaults} ajvNew=${newFaults} ajvPreExisting=${staleFaults}`);
