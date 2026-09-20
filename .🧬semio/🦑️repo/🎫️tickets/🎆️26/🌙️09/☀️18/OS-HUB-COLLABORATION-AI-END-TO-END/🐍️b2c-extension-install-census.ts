/** 🧩️ Slice B2c — census of every extension crate's built output and its install-root copy.
 * C1b §12.3 recorded the five `imperative-extension-*` rows as having "no built output in this tree at
 * all". This re-measures that claim per extension instead of asserting it: for each registry row with
 * `role === "extension"` it reports whether the build output (`🔌️plugin-modules/<dir>/🌉️bridge.js`)
 * exists, whether the install root carries the same directory with its `📥️install.json`, and the mtime
 * of the component `.core.wasm` on both sides — a published copy older than its crate's own sources is
 * the only remaining sense in which one of these can be "stale".
 */
import { existsSync, readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";

import { generatePluginRegistry } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts";
import { moduleDirectoryName } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts";
import { defaultExtensionInstallRoot } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/📥️installation/🟦️.ts";
import { getWorkspaceRoot } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const repoRoot = getWorkspaceRoot();
const buildRoot = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules");
const installRoot = defaultExtensionInstallRoot(repoRoot);

const newestSource = (dir: string): number => {
  let newest = 0;
  const walk = (current: string): void => {
    for (const entry of readdirSync(current, { withFileTypes: true })) {
      if (entry.name.startsWith(".") || entry.name === "target" || entry.name === "node_modules") continue;
      const path = join(current, entry.name);
      if (entry.isDirectory()) walk(path);
      else if (entry.name.endsWith(".rs") || entry.name === "Cargo.toml") newest = Math.max(newest, statSync(path).mtimeMs);
    }
  };
  walk(dir);
  return newest;
};

const wasmMtime = (dir: string): number => {
  if (!existsSync(dir)) return 0;
  const wasm = readdirSync(dir).find((name) => name.endsWith(".core.wasm"));
  return wasm ? statSync(join(dir, wasm)).mtimeMs : 0;
};

const registry = generatePluginRegistry(repoRoot);
const rows = registry.filter((entry) => entry.role === "extension").map((entry) => {
  const directory = moduleDirectoryName(entry.pluginId);
  const built = join(buildRoot, directory);
  const installed = join(installRoot, directory);
  const source = newestSource(join(repoRoot, entry.cratePath));
  const builtWasm = wasmMtime(built);
  return {
    pluginId: entry.pluginId,
    directory,
    builtBridge: existsSync(join(built, "🌉️bridge.js")),
    installedBridge: existsSync(join(installed, "🌉️bridge.js")),
    installMeta: existsSync(join(installed, "📥️install.json")) ? JSON.parse(readFileSync(join(installed, "📥️install.json"), "utf8")).installedAt : null,
    builtWasmAt: builtWasm ? new Date(builtWasm).toISOString() : null,
    installedWasmAt: wasmMtime(installed) ? new Date(wasmMtime(installed)).toISOString() : null,
    newestSourceAt: source ? new Date(source).toISOString() : null,
    wasmOlderThanSource: Boolean(builtWasm && source && builtWasm < source),
  };
});

for (const row of rows) console.log(JSON.stringify(row));
console.log(
  "SUMMARY",
  JSON.stringify({
    extensions: rows.length,
    missingBuiltOutput: rows.filter((row) => !row.builtBridge).map((row) => row.pluginId),
    missingInstall: rows.filter((row) => !row.installedBridge).map((row) => row.pluginId),
    wasmOlderThanSource: rows.filter((row) => row.wasmOlderThanSource).map((row) => row.pluginId),
  }),
);
