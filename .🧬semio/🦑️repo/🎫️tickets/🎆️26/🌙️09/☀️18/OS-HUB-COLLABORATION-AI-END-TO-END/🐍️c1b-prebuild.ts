/** 🧱️ Slice C1b — runs ONLY the collaboration scenario's own plugin prebuild (`space` + `writer`), so the
 * cold wasm build is paid once, detached, and the real `verify collab` run finds the artifacts already on
 * disk instead of spending its hub-boot budget on cargo. */
import { collabPluginArtifactPath, collabPrebuildPlugins, COLLAB_E2E_REQUIRED_PLUGIN_IDS } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🤝️collaboration/🟦️.ts";
import { preparePluginBuildTargets } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🏃️execution/🟦️.ts";
import { DEFAULT_HOST_VARIANT } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds/🟦️.ts";
import { existsSync } from "node:fs";

console.log(`[c1b-prebuild] required: ${COLLAB_E2E_REQUIRED_PLUGIN_IDS.join(", ")}`);
await collabPrebuildPlugins();
const targets = await preparePluginBuildTargets(DEFAULT_HOST_VARIANT);
for (const pluginId of COLLAB_E2E_REQUIRED_PLUGIN_IDS) {
  const target = targets.find((entry) => entry.pluginId === pluginId);
  const path = target ? collabPluginArtifactPath(target) : "<no registry entry>";
  console.log(`[c1b-prebuild] ${pluginId}: ${target && existsSync(path) ? "PRESENT" : "MISSING"} ${path}`);
}
console.log("[c1b-prebuild] done");
