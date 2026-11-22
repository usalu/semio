import {existsSync, readFileSync} from "node:fs";
import {join, resolve} from "node:path";
import {artifactFiles} from "@semio-tech/framework-plugin/browser-bundle/distribution/inventory";
import {MODULE_SHARD_DIRECTORY, componentOwnerRootV1} from "@semio-tech/plugin-registry/deployment";
import {PREVIEW2_VENDOR_RELATIVE} from "@semio-tech/framework-plugin/browser-bundle/imports";
import {ACTIVATION_RECEIPT_FILE, activationSourceFreshnessDirectory, builtPluginModulesRootIn, developmentRuntimeRoot, nextActivationReceipt, publishActivationReceipt, readActivationReceipt} from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts";
import {activationFilesDigest, collectActivationArtifactsV1} from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🔖️identity/🟦️.ts";

const repoRoot = resolve(import.meta.dir, "../../../../../../..");
const [variant, ...ids] = process.argv.slice(2) as [string, ...string[]];
const catalog = JSON.parse(readFileSync(join(repoRoot, "✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🔌️plugins.json"), "utf8")) as {pluginId: string; cratePath: string; directoryName: string}[];
const moduleRoot = builtPluginModulesRootIn(repoRoot, "prototype");
const runtime = developmentRuntimeRoot(repoRoot, variant, "prototype", "react"), receiptRoot = join(runtime, "activation");
const controller = new AbortController();
const support = new Map<string, string>();
for (const directory of [PREVIEW2_VENDOR_RELATIVE, MODULE_SHARD_DIRECTORY]) for (const [name, path] of artifactFiles(join(moduleRoot, directory))) support.set(join(directory, name), path);
const supportDigest = await activationFilesDigest(support, controller.signal);
const completed = await collectActivationArtifactsV1(ids.map(pluginId => {
  const entry = catalog.find(e => e.pluginId === pluginId)!;
  const dir = join(moduleRoot, entry.directoryName);
  return {pluginId, files: artifactFiles(dir), sourceRoot: join(repoRoot, componentOwnerRootV1(entry.cratePath)), freshnessDirectory: activationSourceFreshnessDirectory(runtime, pluginId)};
}), supportDigest, controller.signal);
const previous = existsSync(join(receiptRoot, ACTIVATION_RECEIPT_FILE)) ? readActivationReceipt(receiptRoot) : undefined;
const receipt = nextActivationReceipt(variant, "prototype", completed, previous);
console.log("[DEBUG] receipt changed:", publishActivationReceipt(receiptRoot, receipt), receiptRoot, JSON.stringify(receipt.plugins.map(p => p.pluginId)));
