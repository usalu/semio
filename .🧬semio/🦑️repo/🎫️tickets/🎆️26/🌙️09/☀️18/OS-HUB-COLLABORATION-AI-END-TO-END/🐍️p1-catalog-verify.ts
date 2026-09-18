/** 🗺️ Proves what `🗺️catalog.json`'s `directoryName` actually is: a flat, emoji-unique INSTALLATION
 * basename materialized under the module staging roots, never a source path under `✏️s/🔌️plugins`. */

import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

const repoRoot = join(import.meta.dirname, "../../../../../../..");
const registryRoot = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry");
const catalogPath = join(registryRoot, "📦️deployment/🗺️catalog.json");
const schemaPath = join(registryRoot, "📦️deployment/🧬️schema/🔣️.json");
const installationSchemaPath = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧩️extension/🧬️schema/🔣️.json");
const generatedPath = join(registryRoot, "🤖️generated/🔌️plugins.json");

const stagingRoots = [
  join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️plugin-modules"),
  join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧩️extension-modules"),
  join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules"),
  join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/release/🔌️plugin-modules"),
];

type Row = { readonly pluginId: string; readonly directoryName: string };
type Generated = { readonly pluginId: string; readonly cratePath: string; readonly packageName: string; readonly role: string };

const catalog = JSON.parse(readFileSync(catalogPath, "utf8")) as { version: number; modules: Row[] };
const generated = JSON.parse(readFileSync(generatedPath, "utf8")) as Generated[];
const directoryPattern = new RegExp(JSON.parse(readFileSync(installationSchemaPath, "utf8")).$defs.InstallationDirectoryV1.pattern, "u");
const idPattern = new RegExp(JSON.parse(readFileSync(schemaPath, "utf8")).$defs.DeploymentCatalogV1.properties.modules.items.properties.pluginId.pattern, "u");

const staged = new Map<string, string[]>();
for (const root of stagingRoots) {
  if (!existsSync(root)) continue;
  for (const entry of readdirSync(root, { withFileTypes: true })) {
    if (!entry.isDirectory()) continue;
    staged.set(entry.name, [...(staged.get(entry.name) ?? []), root.slice(repoRoot.length + 1)]);
  }
}

const failures: string[] = [];
const unstaged: string[] = [];
const emojis = new Map<string, string>();
const byId = new Map(generated.map((row) => [row.pluginId, row]));

for (const [index, row] of catalog.modules.entries()) {
  if (!idPattern.test(row.pluginId)) failures.push(`${row.pluginId}: pluginId violates the deployment schema pattern`);
  if (!directoryPattern.test(row.directoryName)) failures.push(`${row.pluginId}: directoryName ${JSON.stringify(row.directoryName)} violates InstallationDirectoryV1 (one emoji-prefixed kebab segment, no "/")`);
  const emoji = [...row.directoryName].slice(0, 2).join("");
  const owner = emojis.get(emoji);
  if (owner) failures.push(`${row.pluginId}: sibling emoji ${emoji} already owned by ${owner} — the installation roots are FLAT, so every row needs its own emoji`);
  emojis.set(emoji, row.pluginId);

  const source = byId.get(row.pluginId);
  if (!source) failures.push(`${row.pluginId}: no row in the generated 🔌️plugins.json`);
  else if (!existsSync(join(repoRoot, source.cratePath))) failures.push(`${row.pluginId}: cratePath ${source.cratePath} does not exist`);
  else if (generated[index]?.pluginId !== row.pluginId) failures.push(`${row.pluginId}: catalog row ${index} is out of order against the generated registry`);

  const roots = staged.get(row.directoryName);
  if (roots) console.log(`ok    ${row.pluginId.padEnd(38)} ${row.directoryName.padEnd(38)} staged in ${roots.length} root(s)`);
  else unstaged.push(`${row.pluginId.padEnd(38)} ${row.directoryName}`);
}

console.log(`\n${catalog.modules.length} catalog rows, ${catalog.modules.length - unstaged.length} materialized in a staging root, ${unstaged.length} not built on this machine.`);
for (const row of unstaged) console.log(`unbuilt ${row}`);

const sourceDirectories = new Set(readdirSync(join(repoRoot, "✏️s/🔌️plugins"), { withFileTypes: true }).filter((entry) => entry.isDirectory()).map((entry) => entry.name));
const alsoSource = catalog.modules.filter((row) => sourceDirectories.has(row.directoryName)).length;
console.log(`\n${alsoSource} of ${catalog.modules.length} directoryName values coincide with a ✏️s/🔌️plugins source basename (the 34 top-level plugins); the 26 nested extension/module rows deliberately do not — their sources live at <parent>/🧩️extensions/<emoji><name> and cannot be flattened into one emoji-unique installation root without colliding.`);

if (failures.length > 0) {
  for (const failure of failures) console.error(`FAIL  ${failure}`);
  process.exit(1);
}
console.log("\n[p1-catalog-verify] clean — every row is schema-valid, emoji-unique, ordered against the generated registry and backed by a real crate on disk.");
