import { join } from "node:path";
import { developmentRuntimeRoot, readActivationReceipt } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts";
import { demonstratorRuntimeComponentIds } from "../📦️assets/🟦️.ts";

/** 🧾️ Requires the exact completed Demonstrator component union before starting its development host. */
export function readDemonstratorActivation(workspace: string) {
  const root = developmentRuntimeRoot(join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"), "generator", "dev", "react");
  const receiptDirectory = join(root, "activation"), receipt = readActivationReceipt(receiptDirectory);
  if (receipt.variant !== "generator" || receipt.profile !== "dev" || receipt.plugins.map(row => row.pluginId).sort().join() !== demonstratorRuntimeComponentIds().join()) throw new Error("Demonstrator activation does not contain its exact runtime union");
  return { receiptDirectory, extensionsDirectory: join(root, "extensions"), receipt };
}
