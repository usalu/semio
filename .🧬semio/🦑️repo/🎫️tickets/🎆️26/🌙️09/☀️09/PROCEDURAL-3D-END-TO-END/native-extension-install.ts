import { cpSync, existsSync, mkdirSync, readdirSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { builtPluginModulesRootIn, developmentRuntimeRoot, readActivationReceipt, ACTIVATION_RECEIPT_FILE } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts";
import { rewritePreview2ShimImportSource, PREVIEW2_VENDOR_RELATIVE } from "@semio-tech/framework-plugin/browser-bundle/imports";
import { EXTENSION_INSTALL_META } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/📥️installation/🟦️.ts";
import { MODULE_BRIDGE_FILE, MODULE_PLUGIN_ROUTE, MODULE_EXTENSION_ROUTE } from "@semio-tech/plugin-registry/deployment";

/** 🧩️ Graph-free twin of `publishActivatedExtension`: installs each staged flow extension into the development runtime namespace (rewritten preview2 imports plus the install record) so the host lists it as installed. Usage: `S_G3D_RENDERER=react bun native-extension-install.ts <pluginId>...` after `wgpu-native-activate.ts`. */
const workspace = process.cwd(), variant = "generation3d", profile = "prototype", renderer = process.env.S_G3D_RENDERER === "react" ? "react" : "wgpu";
const runtime = developmentRuntimeRoot(workspace, variant, profile, renderer), install = join(runtime, "extensions"), modules = builtPluginModulesRootIn(workspace, profile);
const receipt = readActivationReceipt(join(runtime, "activation")), rows = JSON.parse(readFileSync(join(modules, "../sessions/generation3d/🔌️plugins.json"), "utf8")) as { pluginId: string; directoryName: string; extends?: string }[];
for (const pluginId of process.argv.slice(2)) {
  const row = rows.find(r => r.pluginId === pluginId), staged = receipt.plugins.find(p => p.pluginId === pluginId);
  if (!row || !staged) throw new Error("not staged and activated: " + pluginId);
  const source = join(modules, row.directoryName), output = join(install, row.directoryName);
  rmSync(output, { recursive: true, force: true }); mkdirSync(output, { recursive: true });
  const walk = (dir: string, rel = ""): void => { for (const name of readdirSync(dir)) { const from = join(dir, name), to = join(output, rel, name); if (statSync(from).isDirectory()) walk(from, join(rel, name)); else { mkdirSync(dirname(to), { recursive: true }); if (name.endsWith(".js")) writeFileSync(to, rewritePreview2ShimImportSource(readFileSync(from, "utf8"), `../..${MODULE_PLUGIN_ROUTE}/${PREVIEW2_VENDOR_RELATIVE}/`)); else cpSync(from, to); } } };
  walk(source);
  writeFileSync(join(output, EXTENSION_INSTALL_META), JSON.stringify({ extensionId: pluginId, directoryName: row.directoryName, version: "0.0.0-dev", label: pluginId, extends: row.extends ?? "", moduleUrl: `${MODULE_EXTENSION_ROUTE}/${row.directoryName}/${MODULE_BRIDGE_FILE}`, packageHash: staged.artifactSha256, installedAt: Date.now() }) + "\n");
  console.log("[g3d-native] installed extension", pluginId, output);
}
