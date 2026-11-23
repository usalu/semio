import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { ACTIVATION_RECEIPT_FILE, builtPluginModulesRootIn, developmentRuntimeRoot, nextActivationReceipt, publishActivationReceipt, readActivationReceipt } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts";
import { activationFilesDigest } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🔖️identity/🟦️.ts";
import { artifactFiles } from "@semio-tech/framework-plugin/browser-bundle/distribution/inventory";

/** ♻️ Graph-free twin of `activate generation3d wgpu prototype`: publishes the activation receipt over the plugin modules already staged in the prototype modules root (the receipt the Vite host's reload observer needs). */
const workspace = process.cwd(), variant = "generation3d", profile = "prototype", renderer = process.env.S_G3D_RENDERER === "react" ? "react" : "wgpu", moduleRoot = builtPluginModulesRootIn(workspace, profile);
const runtime = developmentRuntimeRoot(workspace, variant, profile, renderer), receiptRoot = join(runtime, "activation");
mkdirSync(receiptRoot, { recursive: true }); mkdirSync(join(runtime, "extensions"), { recursive: true });
const staged = readdirSync(moduleRoot).filter(name => existsSync(join(moduleRoot, name, "🛂️.descriptor.semio")));
const completed = [];
for (const name of staged) {
  const digest = await activationFilesDigest(artifactFiles(join(moduleRoot, name)), new AbortController().signal);
  completed.push({ pluginId: name.replace(/^[^a-z0-9]+/u, "").replace(/️/gu, ""), artifactSha256: createHash("sha256").update(digest).digest("hex") });
}
const previous = existsSync(join(receiptRoot, ACTIVATION_RECEIPT_FILE)) ? readActivationReceipt(receiptRoot) : undefined;
const receipt = nextActivationReceipt(variant, profile, completed, previous);
console.log("[g3d-native] activation receipt", receipt.plugins.map(row => row.pluginId), publishActivationReceipt(receiptRoot, receipt) ? "changed" : "unchanged", receiptRoot);
