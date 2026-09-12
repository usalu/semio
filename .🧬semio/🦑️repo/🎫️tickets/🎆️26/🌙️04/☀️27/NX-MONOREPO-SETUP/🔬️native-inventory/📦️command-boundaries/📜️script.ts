import { readFileSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
const root = process.env.SEMIO_REPO_ROOT!, require = createRequire(import.meta.url);
const fixture = JSON.parse(readFileSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧫️fixtures/command-boundaries/🧫️cases.json"), "utf8"));
const rows = [];
for (const row of fixture.cases) {
  const result = await require("esbuild").build({ entryPoints: [join(root, row.entry)], absWorkingDir: root, bundle: true, write: false, platform: "node", format: "esm", packages: "external", metafile: true, logLevel: "silent" });
  rows.push({ name: row.name, maximum: row.maximumInputs, inputs: Object.keys(result.metafile.inputs).sort() });
}
writeFileSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!, "command-boundaries-observed.json"), JSON.stringify(rows, null, 2));
console.log("[DEBUG] Command import counts " + JSON.stringify(rows.map(row => ({ name: row.name, maximum: row.maximum, observed: row.inputs.length }))));
