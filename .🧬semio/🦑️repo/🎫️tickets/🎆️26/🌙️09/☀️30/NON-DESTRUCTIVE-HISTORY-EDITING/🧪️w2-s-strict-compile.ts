#!/usr/bin/env bun
/**
 * 🧪️ W2-S strict-oracle compile check: compiles every mutation leaf payload schema under the given roots in the one strict Ajv
 * oracle (`semioSchemaAjvV1`, vendor vocabulary + pinned format policy), with every `$id` schema document under `✏️s` and
 * `🧰️framework` available for cross-document `$ref`s, and prints each leaf that does not compile.
 *
 *   bun 🧪️w2-s-strict-compile.ts [--aggregates] <root>…   (--aggregates: the `🧬️mutations/🔣️.json` aggregate documents instead)
 */
import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { semioSchemaAjvV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🧪️tests/🧬️schema-oracle/🟦️.ts";

const repo = "/Users/ueli/Documents/semio";
const documents = new Map<string, unknown>();
const leaves: string[] = [];
const walk = (directory: string, collect: boolean): void => {
  for (const entry of readdirSync(join(repo, directory), { withFileTypes: true })) {
    const path = `${directory}/${entry.name}`;
    if (entry.isDirectory() && !["node_modules", "target", "dist", "🗑️generated"].includes(entry.name)) walk(path, collect);
    else if (entry.isFile() && entry.name.endsWith(".json") && path.includes("🧬️schema")) {
      try {
        const document = JSON.parse(readFileSync(join(repo, path), "utf8"));
        if (typeof document?.$id === "string") documents.set(document.$id, document);
      } catch {}
      if (collect && (aggregates ? /🧬️mutations\/🔣️\.json$/u : /🧬️mutations\/.+\/🧬️schema\/🔣️\.json$/u).test(path) && (aggregates || !path.split("/").some((segment) => segment.endsWith("fixtures")))) leaves.push(path);
    }
  }
};
const aggregates = process.argv.includes("--aggregates");
const roots = process.argv.slice(2).filter((argument) => argument !== "--aggregates");
walk("✏️s", false);
walk("🧰️framework", false);
for (const root of roots) walk(root, true);
let failed = 0;
for (const leaf of leaves.sort()) {
  const ajv = semioSchemaAjvV1({ strict: true, allErrors: true });
  const schema = JSON.parse(readFileSync(join(repo, leaf), "utf8"));
  const loaded = new Set<string>();
  const load = (node: unknown): void => {
    if (node === null || typeof node !== "object") return;
    for (const [key, value] of Object.entries(node)) {
      if (key === "$ref" && typeof value === "string" && !value.startsWith("#")) {
        const id = value.split("#")[0]!;
        const document = documents.get(id);
        if (document !== undefined && !loaded.has(id) && id !== schema.$id) {
          loaded.add(id);
          if (ajv.getSchema(id) === undefined) ajv.addSchema(document as object);
          load(document);
        }
      } else load(value);
    }
  };
  try {
    load(schema);
    ajv.compile(schema);
  } catch (error) {
    failed += 1;
    console.log(`[strict] ${leaf.split("/🗿️artifacts/").at(-1)} — ${error instanceof Error ? error.message.slice(0, 200) : String(error)}`);
  }
}
console.log(`[w2-s] strict oracle compiled ${leaves.length - failed}/${leaves.length} leaf schemas under ${roots.join(", ")}`);
