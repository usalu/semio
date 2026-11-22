import { existsSync } from "node:fs";
import { join } from "node:path";

/** 🛠️ [DEBUG] Native activation of the standalone XLSX pane, mirroring `ActivationScript` without the issued invocation: support digest, per-component digests and the receipt. */
const root = "/Users/ueli/Documents/semio";
const variant = "stdio-xlsx", profile = "prototype" as const;
const dev = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev");
const activation = await import(join(dev, "♻️activation/🟦️.ts"));
const { collectActivationArtifactsV1, activationFilesDigest } = await import(join(dev, "♻️activation/🔖️identity/🟦️.ts"));
const { artifactFiles } = await import("@semio-tech/framework-plugin/browser-bundle/distribution/inventory");
const { PREVIEW2_VENDOR_RELATIVE } = await import("@semio-tech/framework-plugin/browser-bundle/imports");
const { MODULE_SHARD_DIRECTORY, moduleDirectoryName, componentOwnerRootV1 } = await import("@semio-tech/plugin-registry/deployment");
const { FONT_ASSET } = await import("@semio-tech/framework-canvas-ts/fonts");
const { fontAssetPublicationRootV1 } = await import("@semio-tech/framework-canvas-ts/fonts/publication");
const { readGeneratedCatalogProjection } = await import("@semio-tech/plugin-registry/catalog-view");
const { runtimeCatalogBuildInventoryV1 } = await import("@semio-tech/plugin-registry/catalog-view");
const moduleRoot = activation.builtPluginModulesRootIn(root, profile);
const runtime = activation.developmentRuntimeRoot(root, variant, profile, "react"), receiptRoot = join(runtime, "activation");
const projection = readGeneratedCatalogProjection();
const catalog = new Map(projection.entries.map((entry: { pluginId: string }) => [entry.pluginId, entry]));
const session = (await import(join(root, ".cache/b/p/m/browser/prototype/sessions/stdio-xlsx/🎮️playground-session/🟦️.ts"))).PLAYGROUND_SESSION as { plugins: { pluginId: string }[] };
const support = new Map<string, string>();
for (const directory of [PREVIEW2_VENDOR_RELATIVE, MODULE_SHARD_DIRECTORY]) for (const [name, path] of artifactFiles(join(moduleRoot, directory))) support.set(join(directory, name), path);
const fonts = fontAssetPublicationRootV1(join(root, ".cache/b/p/m"), profile);
support.set(FONT_ASSET, join(fonts, FONT_ASSET));
const controller = new AbortController();
const supportDigest = await activationFilesDigest(support, controller.signal);
const ids = session.plugins.map(plugin => plugin.pluginId);
const rows = ids.map(pluginId => {
  const directory = join(moduleRoot, moduleDirectoryName(pluginId, runtimeCatalogBuildInventoryV1().moduleDirectories));
  const entry = catalog.get(pluginId) as { cratePath: string } | undefined;
  return { pluginId, files: artifactFiles(directory), sourceRoot: entry ? join(root, componentOwnerRootV1(entry.cratePath)) : directory, freshnessDirectory: activation.activationSourceFreshnessDirectory(runtime, pluginId) };
});
const completed = await collectActivationArtifactsV1(rows, supportDigest, controller.signal);
const previous = existsSync(join(receiptRoot, activation.ACTIVATION_RECEIPT_FILE)) ? activation.readActivationReceipt(receiptRoot) : undefined;
const receipt = activation.nextActivationReceipt(variant, profile, completed, previous);
console.log(`[DEBUG] Activated ${variant} react ${profile}: ${receipt.plugins.length} components, receipt ${activation.publishActivationReceipt(receiptRoot, receipt) ? "changed" : "unchanged"}`);
