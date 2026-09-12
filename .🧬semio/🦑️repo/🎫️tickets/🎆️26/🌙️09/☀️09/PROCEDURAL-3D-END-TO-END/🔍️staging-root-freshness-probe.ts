/** 🔎️ Ticket probe: runs the live staged-module freshness pass against the real generation3d dev
 * activation receipt and the one staging root, without starting or touching any server. */
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { pluginModulesRoot, readActivationReceipt } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts";
import { reportActivationFreshness } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/🔌️vite-plugins.ts";
import { moduleDirectoryName } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts";
import { PLUGIN_BUILD_TARGETS, EXTENSION_TARGETS } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🧩️plugins.ts";

const repoRoot = resolve(import.meta.dir, "../../../../../../..");
const variant = process.argv[2] ?? "generation3d";
const devPackage = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript");
const runtime = join(devPackage, "dist/runtime/dev", variant);
const receipt = readActivationReceipt(join(runtime, "activation"));
const components = [...PLUGIN_BUILD_TARGETS, ...EXTENSION_TARGETS]
  .filter((target) => receipt.plugins.some((row) => row.pluginId === target.pluginId))
  .map((target) => ({ pluginId: target.pluginId, directoryName: moduleDirectoryName(target.pluginId), role: target.role === "extension" ? ("extension" as const) : ("plugin" as const), sourceRoot: resolve(repoRoot, target.cratePath, "..", "..") }));
console.log(`[DEBUG] root ${pluginModulesRoot("dev")}`);
console.log(`[DEBUG] receipt ${variant} ${receipt.profile}: ${receipt.plugins.length} activated components, ${components.length} checked`);
const started = Date.now();
const lines = reportActivationFreshness(receipt, { moduleRoot: pluginModulesRoot("dev"), installRoot: join(runtime, "extensions"), components });
console.log(`[DEBUG] freshness pass took ${Date.now() - started} ms`);
for (const line of lines) console.log(line);
if (lines.length === 0) console.log(`[fresh] ${components.length} staged components match their sources and the activation receipt`);
void readFileSync;
