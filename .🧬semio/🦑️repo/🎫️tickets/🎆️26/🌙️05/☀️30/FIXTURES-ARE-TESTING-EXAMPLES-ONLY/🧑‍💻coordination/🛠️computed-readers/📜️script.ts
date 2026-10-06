import assert from "node:assert/strict";
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
const root = process.env.SEMIO_FIXTURE_REPO_ROOT!, ticket = dirname(dirname(import.meta.dir));
const sub = "/🏅️standards/🔖️1/🪆️subsets/✳️any";
const energy = "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model" + sub;
const changes: { path: string; before: string; after: string }[] = [];
for (const name of ["600", "600FF", "610", "620", "630", "640", "650", "900", "900FF", "910", "920", "930", "940", "950"]) {
  const example = "🏛️bestest-" + name;
  const path = energy + "/📚️examples/" + example + "/🧪️tests/🧩️example/🟦️.ts";
  const target = join(root, energy, "🖼️assets", example, "🗣️.dsl.semio");
  assert(existsSync(target), target);
  const before = "../../🖼️assets/🗣️.dsl.semio", after = relative(dirname(join(root, path)), target).replaceAll("\\", "/");
  const source = readFileSync(join(root, path), "utf8");
  assert(source.includes(before) || source.includes(after), path);
  writeFileSync(join(root, path), source.replaceAll(before, after));
  changes.push({ path, before, after });
}
const cases = [
 "✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation" + sub + "/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️graph/🎚️config/🧪️tests/🔬️window-config-ownership/🟦️.ts",
 "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting" + sub + "/✏️editor/🪟️window/🎚️config/🧪️tests/🔬️window-config-ownership/🟦️.ts",
];
for (const path of cases) {
  const absolute = join(root, path); let source = readFileSync(absolute, "utf8");
  source = source.replace(/"([^"\n]*🧬️schema(?:\/🧬️mutations)?)\/\.\.\/\.\.\/🧫️fixtures\/[^"]+"/gu, (before, schema) => { const after = schema + "/🔣️.json"; assert.equal(existsSync(join(dirname(absolute), after)), !schema.startsWith("../../../../🎚️config/"), after); changes.push({ path, before, after }); return JSON.stringify(after); });
  writeFileSync(absolute, source);
}
const raster = "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster" + sub + "/🚪️io/🧪️tests/🧩️suite/🟦️.ts";
const source = readFileSync(join(root, raster), "utf8");
assert(source.includes("./🧫️fixtures/"), raster);
writeFileSync(join(root, raster), source.replaceAll("./🧫️fixtures/", "../../🧫️fixtures/"));
changes.push({ path: raster, before: "./🧫️fixtures/", after: "../../🧫️fixtures/" });
writeFileSync(join(ticket, "📓️computed-reader-repairs-2026-09-09.md"), "# Computed Test Reader Repairs\n\nFourteen Energy BESTEST cases now read their subset-owned assets; target paths were computed relative to the actual case file directory and checked for existence. Equation and Rewriting window tests now load their schema declarations directly rather than resolving a fixture as a schema. The Raster I/O case now reads its I/O owner's fixtures. Writer's analogous schema reader was concurrently corrected before the coordinator changed it.\n\n## Authored Paths\n\n```json\n" + JSON.stringify([...new Set(changes.map(row => row.path))].sort(), null, 2) + "\n```\n\n## Reader Coordinates\n\n```json\n" + JSON.stringify(changes, null, 2) + "\n```\n");
console.log("[DEBUG] " + JSON.stringify({ files: new Set(changes.map(row => row.path)).size, readers: changes.length }));
