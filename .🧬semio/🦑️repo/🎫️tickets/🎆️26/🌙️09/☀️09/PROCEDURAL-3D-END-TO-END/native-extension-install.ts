import { cpSync, existsSync, mkdirSync, readdirSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { builtPluginModulesRootIn, developmentRuntimeRoot, nextActivationReceipt, publishActivationReceipt, readActivationReceipt } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts";
import { activationFilesDigest } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🔖️identity/🟦️.ts";
import { rewritePreview2ShimImportSource, PREVIEW2_VENDOR_RELATIVE } from "@semio-tech/framework-plugin/browser-bundle/imports";
import { artifactFiles } from "@semio-tech/framework-plugin/browser-bundle/distribution/inventory";
import { EXTENSION_INSTALL_META } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/📥️installation/🟦️.ts";
import { MODULE_BRIDGE_FILE, MODULE_PLUGIN_ROUTE, MODULE_EXTENSION_ROUTE } from "@semio-tech/plugin-registry/deployment";

/** 🧩️ Installs staged flow extensions into the development runtime, with optional freshly compiled Jco assets under `S_G3D_COMPONENT_ROOT`.
 * @see ./wgpu-native-activate.ts */
const workspace = process.cwd(), variant = "generation3d", profile = "prototype", renderer = process.env.S_G3D_RENDERER === "react" ? "react" : "wgpu";
const runtime = developmentRuntimeRoot(workspace, variant, profile, renderer), install = join(runtime, "extensions"), modules = builtPluginModulesRootIn(workspace, profile);
const receiptRoot = join(runtime, "activation");
const receipt = readActivationReceipt(receiptRoot), rows = JSON.parse(readFileSync(join(modules, "../sessions/generation3d/🔌️plugins.json"), "utf8")) as { pluginId: string; directoryName: string; extends?: string }[];
const freshRoot = process.env.S_G3D_COMPONENT_ROOT;
const publicationMarker = ".semio-artifact.json";
const plans = process.argv.slice(2).map((pluginId) => {
  const row = rows.find(r => r.pluginId === pluginId), staged = receipt.plugins.find(p => p.pluginId === pluginId);
  if (!row || !staged) throw new Error("not staged and activated: " + pluginId);
  const source = join(modules, row.directoryName), output = join(install, row.directoryName);
  const marker = join(output, publicationMarker);
  if (existsSync(marker) && JSON.parse(readFileSync(marker, "utf8")).owner === `development-extension:${pluginId}`) throw new Error("native installer requires a development fallback directory: " + output);
  const overrides = new Map<string, string>();
  if (freshRoot) {
    const component = /^flow-extension-(math|brep)$/.exec(pluginId)?.[1];
    if (!component) throw new Error("no fresh component identity for " + pluginId);
    const base = `semio_s_plugin_flow_${component}_component`;
    const directory = join(freshRoot, base);
    for (const name of readdirSync(directory)) {
      if (name === base + ".js" || new RegExp(`^${base}\\.core[0-9]*\\.wasm$`).test(name)) overrides.set(name, join(directory, name));
    }
    for (const name of [base + ".js", base + ".core.wasm"]) {
      const path = overrides.get(name);
      if (!path || !statSync(path).isFile() || statSync(path).size === 0) throw new Error("incomplete fresh component asset: " + name);
    }
  }
  const files = artifactFiles(source);
  files.delete(publicationMarker);
  for (const [name, path] of overrides) files.set(name, path);
  return { pluginId, row, staged, output, files };
});
const digests = new Map<string, string>();
for (const { pluginId, staged, output, files } of plans) {
  rmSync(output, { recursive: true, force: true }); mkdirSync(output, { recursive: true });
  for (const [name, from] of files) {
    const to = join(output, name);
    mkdirSync(dirname(to), { recursive: true });
    if (name.endsWith(".js")) writeFileSync(to, rewritePreview2ShimImportSource(readFileSync(from, "utf8"), `../..${MODULE_PLUGIN_ROUTE}/${PREVIEW2_VENDOR_RELATIVE}/`));
    else cpSync(from, to);
  }
  digests.set(pluginId, freshRoot ? await activationFilesDigest(artifactFiles(output), new AbortController().signal) : staged.artifactSha256);
}
const latest = nextActivationReceipt(variant, profile, receipt.plugins.map((plugin) => ({
  ...plugin,
  artifactSha256: digests.get(plugin.pluginId) ?? plugin.artifactSha256,
})), receipt);
for (const { pluginId, row, output } of plans) {
  const staged = latest.plugins.find((plugin) => plugin.pluginId === pluginId)!;
  writeFileSync(join(output, EXTENSION_INSTALL_META), JSON.stringify({ extensionId: pluginId, directoryName: row.directoryName, version: "0.0.0-dev", label: pluginId, extends: row.extends ?? "", moduleUrl: `${MODULE_EXTENSION_ROUTE}/${row.directoryName}/${MODULE_BRIDGE_FILE}`, packageHash: staged.artifactSha256, installedAt: staged.rebuiltAt }) + "\n");
  console.log("[DEBUG] installed development extension", pluginId, staged.artifactSha256);
}
if (freshRoot) publishActivationReceipt(receiptRoot, latest);
