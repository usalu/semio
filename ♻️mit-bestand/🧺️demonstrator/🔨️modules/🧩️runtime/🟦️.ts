import catalog from "./🔣️.json";
import { runtimeComponentClosure } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧩️runtime/🟨️.mjs";
import { PLAYGROUND_BUILD_TARGETS } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds/🟦️.ts";
import { EXTENSION_TARGETS, PLUGIN_BUILD_TARGETS } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🧩️plugins/🟦️.ts";
import { MODULE_SHARD_DIRECTORY, MODULE_VENDOR_DIRECTORY, moduleDirectoryName } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts";

export const DEMONSTRATOR_HOST = catalog.host;
export const DEMONSTRATOR_ASSETS_DIR = catalog.assetsDirectory;
export const DEMONSTRATOR_RUNTIME_PANES = catalog.panes;

/** 🎛️ Selects a pane's authored runtime variant. */
export function demonstratorPaneRuntimeVariant(variant: string): string {
  const pane = catalog.panes.find(row => row.variant === variant);
  if (!pane) throw new Error(`Unknown demonstrator pane variant: ${variant}`);
  return pane.runtimeVariant;
}

/** 🪪️ Resolves a runtime variant to its generated component identity. */
function runtimePluginId(variant: string): string {
  const target = PLAYGROUND_BUILD_TARGETS.find(row => row.variant === variant);
  if (!target) throw new Error(`Unknown demonstrator runtime variant: ${variant}`);
  return target.pluginId;
}

/** 🧮️ Selects one representative variant per additional runtime component. */
export function demonstratorRuntimeBuildVariants(primaryVariant: string): readonly string[] {
  const selected = new Set([runtimePluginId(primaryVariant)]), result: string[] = [];
  for (const pane of catalog.panes) {
    const id = runtimePluginId(pane.runtimeVariant);
    if (!selected.has(id)) { selected.add(id); result.push(pane.runtimeVariant); }
  }
  return result;
}

export type DemonstratorRuntimeModuleLayout = {
  readonly pluginModuleDirNames: readonly string[];
  readonly extensionModuleDirNames: readonly string[];
};

/** 🛣️ Maps the complete runtime closure to its authored public deployment directories. */
export function demonstratorRuntimeModuleLayout(rootPluginIds: readonly string[]): DemonstratorRuntimeModuleLayout {
  const components = [...PLUGIN_BUILD_TARGETS, ...EXTENSION_TARGETS], byId = new Map(components.map(row => [row.pluginId, row]));
  const ids = runtimeComponentClosure(components, rootPluginIds);
  return {
    pluginModuleDirNames: [MODULE_VENDOR_DIRECTORY, MODULE_SHARD_DIRECTORY, ...ids.filter(id => byId.get(id)!.role === "plugin").map(moduleDirectoryName)],
    extensionModuleDirNames: ids.filter(id => byId.get(id)!.role === "extension").map(moduleDirectoryName),
  };
}

const variants = new Set(catalog.panes.flatMap(row => [row.variant, row.runtimeVariant]));
export const DEMONSTRATOR_RUNTIME_TARGETS = PLAYGROUND_BUILD_TARGETS.filter(row => variants.has(row.variant));
for (const variant of variants) runtimePluginId(variant);
