import { join, resolve } from "node:path";
import { developmentRuntimeRoot, readActivationReceipt } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts";
import { demonstratorRuntimeComponentIds } from "../📦️assets/🟦️.ts";
import type { ActivationComponentSpec } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts";
import { EXTENSION_TARGETS, PLUGIN_BUILD_TARGETS } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🧩️plugins/🟦️.ts";
import { moduleDirectoryName } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts";

/** 🧾️ Requires the exact completed Demonstrator component union before starting its development host. */
export function readDemonstratorActivation(workspace: string) {
  const root = developmentRuntimeRoot(join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"), "generator", "dev", "react");
  const receiptDirectory = join(root, "activation"), receipt = readActivationReceipt(receiptDirectory);
  if (receipt.variant !== "generator" || receipt.profile !== "dev" || receipt.plugins.map(row => row.pluginId).sort().join() !== demonstratorRuntimeComponentIds().join()) throw new Error("Demonstrator activation does not contain its exact runtime union");
  return { receiptDirectory, extensionsDirectory: join(root, "extensions"), receipt };
}

/** 🔎️ Describes every closure component for the activation-receipt freshness watcher, keyed by the owner root descriptors are staged from. */
export function demonstratorActivationComponents(workspace: string): readonly ActivationComponentSpec[] {
  const byId = new Map([...PLUGIN_BUILD_TARGETS, ...EXTENSION_TARGETS].map(row => [row.pluginId, row]));
  return demonstratorRuntimeComponentIds().map(id => {
    const row = byId.get(id)!;
    return { pluginId: id, directoryName: moduleDirectoryName(id), role: row.role === "extension" ? "extension" : "plugin", sourceRoot: resolve(workspace, row.cratePath, "..", "..") };
  });
}
