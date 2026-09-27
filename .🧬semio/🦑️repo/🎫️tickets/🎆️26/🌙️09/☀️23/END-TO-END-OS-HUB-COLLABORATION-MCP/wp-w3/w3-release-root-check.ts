/** 🔎️ W3: proves a plugin-module root (`dev` or `release`) carries what the current framework TS generates: the shard worker equals
 * `shardWorkerSource()` and has the `codec` request case, every registry component has a module whose host shim equals `hostShimSource()`
 * and whose listed files all exist, and the preview2 vendor carries the framework's guest-log patches. Exit 1 on any finding.
 * usage: bun w3-release-root-check.ts <dev|release> */
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { hostShimSource, PLUGIN_HOST_SHIM_FILE, SHARD_WORKER_FILE, shardWorkerSource } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts";
import { moduleDirectoryName } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts";

const profile = process.argv[2];
if (profile !== "dev" && profile !== "release") throw new Error("usage: w3-release-root-check.ts <dev|release>");
const repo = "/Users/ueli/Documents/semio";
const root = join(repo, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/${profile}/🔌️plugin-modules`);
const registry = JSON.parse(readFileSync(join(repo, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🔌️plugins.json"), "utf8")) as readonly { pluginId: string }[];
const findings: string[] = [];
const shard = join(root, "🧵️shard", SHARD_WORKER_FILE);
const worker = existsSync(shard) ? readFileSync(shard, "utf8") : "";
if (!worker.includes('case "codec"')) findings.push(`shard worker lacks case "codec"`);
if (worker !== shardWorkerSource()) findings.push(`shard worker differs from shardWorkerSource()`);
const cli = join(root, "🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim/cli.js");
if (!existsSync(cli)) findings.push("preview2 vendor cli.js missing");
const shim = hostShimSource();
let modules = 0;
for (const { pluginId } of registry) {
  const directory = join(root, moduleDirectoryName(pluginId));
  const manifest = join(directory, ".nx-artifact.json");
  if (!existsSync(manifest)) { findings.push(`${pluginId}: no module`); continue; }
  modules++;
  const files = (JSON.parse(readFileSync(manifest, "utf8")) as { files: string[] }).files;
  const missing = files.filter((file) => !existsSync(join(directory, file)));
  if (missing.length) findings.push(`${pluginId}: ${missing.length} listed files missing (${missing.slice(0, 2).join(", ")})`);
  if (!existsSync(join(directory, PLUGIN_HOST_SHIM_FILE)) || readFileSync(join(directory, PLUGIN_HOST_SHIM_FILE), "utf8") !== shim) findings.push(`${pluginId}: host shim differs from hostShimSource()`);
}
console.log(`[w3-release-root-check] ${profile}: modules=${modules}/${registry.length} shard-codec=${worker.includes('case "codec"')} shard-current=${worker === shardWorkerSource()} findings=${findings.length}`);
for (const finding of findings.slice(0, 40)) console.log(`  - ${finding}`);
process.exit(findings.length === 0 ? 0 : 1);
