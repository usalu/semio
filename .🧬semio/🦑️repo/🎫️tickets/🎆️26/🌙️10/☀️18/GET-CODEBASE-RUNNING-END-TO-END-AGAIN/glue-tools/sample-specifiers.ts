/**
 * 🎯️ Prints a deterministic sample of public specifiers (`package/subpath`) from `repoapi-tools/exports-final.json` that have importers, are not JSON and whose package is linked in the root node_modules.
 *
 *   bun glue-tools/sample-specifiers.ts [--count=10]
 */
import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import { ROOT } from "./check-exports.ts";

const count = Number(process.argv.find(a => a.startsWith("--count="))?.slice(8) ?? 10);
const spec = JSON.parse(readFileSync(resolve(import.meta.dir, "../repoapi-tools/exports-final.json"), "utf8"));
const rows: string[] = [];
for (const [name, p] of Object.entries<any>(spec.packages)) {
  if (!existsSync(resolve(ROOT, "node_modules", name))) continue;
  for (const e of Object.values<any>(p.entries)) if (e.kind === "module" && !e.existing && e.importers?.length && /\.tsx?$/u.test(e.targetFile)) rows.push(name + e.subpath.slice(1));
}
rows.sort();
const step = Math.max(1, Math.floor(rows.length / count));
console.error(`candidates ${rows.length}`);
for (let i = 0; i < rows.length && i / step < count; i += step) console.log(rows[i]);
