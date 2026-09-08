import assert from "node:assert/strict";
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { createRequire } from "node:module";
const here = dirname(fileURLToPath(import.meta.url)), require = createRequire(import.meta.url);
const cases = JSON.parse(readFileSync(join(here, "🧫️cases.json"), "utf8"));
assert.equal(require("jsonschema").validate(cases, JSON.parse(readFileSync(join(here, "🧬️schema.json"), "utf8"))).valid, true);
const reports = [];
for (const row of cases.cases) {
  const artifact = await require("esbuild").build({ entryPoints: [row.entry], bundle: true, write: false, platform: "node", format: "esm", packages: "external", metafile: true, logLevel: "silent" });
  const paths = Object.keys(artifact.metafile.inputs).sort();
  assert.ok(paths.length <= row.maximumInputs, `${row.name}: command imported ${paths.length} files`);
  assert.ok(paths.every((path) => !path.includes("🔬️") && !path.includes("🧪️") && !path.endsWith("⚡️caching/📜️script.ts") && path !== "📜️script.ts"), row.name);
  const actual = await import(pathToFileURL(resolve(row.entry)).href);
  for (const name of row.exports) assert.equal(typeof actual[name], "function", `${row.name}: ${name}`);
  reports.push({ name: row.name, inputs: paths, exports: row.exports });
}
const out = join(dirname(here), "🗑️generated"); mkdirSync(out, { recursive: true });
writeFileSync(join(out, "command-boundaries.json"), JSON.stringify(reports, null, 2));
console.log("[DEBUG] Artifact, native producer and command router have bounded implementation closures; independent esbuild oracle PASS");
