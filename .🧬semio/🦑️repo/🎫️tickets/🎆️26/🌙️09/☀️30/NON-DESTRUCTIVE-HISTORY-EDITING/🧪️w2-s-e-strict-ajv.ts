/**
 * 🧷️ W2-S-E: compiles every mutation leaf payload schema and every aggregate `🧬️mutations/🔣️.json` of the layout + tail scope
 * with the strict x-semio vocabulary Ajv oracle (`semioSchemaAjvV1({ strict: true })`), resolving cross-document `$ref`s through
 * every `$id` document of the catalogued scopes. Exit 1 on any compile failure.
 *
 *   bun 🧪️w2-s-e-strict-ajv.ts
 */
import { semioSchemaAjvV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧪️tests/🧬️schema-oracle/🟦️.ts";
import { existsSync, readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";

const repo = "/Users/ueli/Documents/semio";
const roots = ["✏️s/🔌️plugins/📏️layout", "✏️s/🔌️plugins/➗️mathematical", "✏️s/🔌️plugins/📜️imperative", "✏️s/🔌️plugins/🏭️process", "✏️s/🔌️plugins/🎬️sequence", "✏️s/🔌️plugins/🌊️flow", "✏️s/🔌️plugins/🕸️dag", "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack", "✏️s/🔌️plugins/📋️forms", "✏️s/🔌️plugins/🎞️animate", "🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow", "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store", "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow", "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite", "🧰️framework/🛍️products/💻️os/🎚️config"];
const skipped = new Set(["node_modules", "target", "dist", "🗑️generated"]);
const catalog = JSON.parse(readFileSync(join(repo, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json"), "utf8")) as { scopes: Record<string, { path: string }> };
const documents = new Map<string, unknown>();
const seen = new Set<string>();
const index = (directory: string): void => {
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    const path = join(directory, entry.name);
    if (entry.isDirectory() && !skipped.has(entry.name)) index(path);
    else if (entry.isFile() && entry.name.endsWith(".json") && !seen.has(path)) {
      seen.add(path);
      try {
        const document = JSON.parse(readFileSync(path, "utf8")) as { $id?: unknown };
        if (typeof document.$id === "string" && !documents.has(document.$id)) documents.set(document.$id, document);
      } catch {}
    }
  }
};
for (const scope of Object.values(catalog.scopes)) if (existsSync(join(repo, scope.path)) && statSync(join(repo, scope.path)).isDirectory()) index(join(repo, scope.path));
for (const root of roots) index(join(repo, root));
const targets: string[] = [];
const walk = (directory: string): void => {
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    if (!entry.isDirectory() || skipped.has(entry.name)) continue;
    const path = join(directory, entry.name);
    const parts = path.split("/");
    if (entry.name === "🧬️schema" && parts.at(-3) === "🧬️mutations" && existsSync(join(path, "🔣️.json"))) targets.push(join(path, "🔣️.json"));
    if (entry.name === "🧬️mutations" && existsSync(join(path, "🔣️.json")) && existsSync(join(path, "🦀️.rs"))) targets.push(join(path, "🔣️.json"));
    walk(path);
  }
};
for (const root of roots) walk(join(repo, root));
let compiled = 0;
const failures: string[] = [];
for (const target of targets.sort()) {
  const schema = JSON.parse(readFileSync(target, "utf8")) as Record<string, unknown>;
  const ajv = semioSchemaAjvV1({ strict: true, loadSchema: async (uri: string) => (documents.get(uri.split("#")[0]!) as object | undefined) ?? Promise.reject(new Error(`unresolved ${uri}`)) });
  try {
    await ajv.compileAsync(schema);
    compiled += 1;
  } catch (error) {
    failures.push(`${target.replace(`${repo}/`, "")}: ${(error as Error).message.slice(0, 200)}`);
  }
}
for (const failure of failures) console.log(`[w2-s-e] AJV ${failure}`);
console.log(`[w2-s-e] strict-ajv compiled=${compiled} failed=${failures.length}`);
process.exit(failures.length === 0 ? 0 : 1);
