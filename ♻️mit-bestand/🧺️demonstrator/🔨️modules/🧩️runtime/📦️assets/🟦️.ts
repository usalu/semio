import {repositoryCachePartitionRootDataV1} from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📂️paths/🟦️.ts";
import {frameworkOsViteArtifactPathsV1} from "@semio-tech/framework-os-dev/builder/vite/artifacts";
import { builtPluginModulesRootIn } from "@semio-tech/framework-os-dev/activation";
import { COMPONENT_MODULE_DIRECTORIES } from "@semio-tech/s-dev-catalog-ts/generated/plugins";
import { join } from "node:path";
import { DEMONSTRATOR_RUNTIME_TARGETS } from "../🟦️.ts";
import { runtimeComponentClosure } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧩️runtime/🟨️.mjs";
import { PLUGIN_BUILD_TARGETS, EXTENSION_TARGETS } from "@semio-tech/s-dev-catalog-ts/generated/plugins";
import { MODULE_PLUGIN_ROUTE, MODULE_EXTENSION_ROUTE, MODULE_VENDOR_DIRECTORY, MODULE_SHARD_DIRECTORY, moduleDirectoryName } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts";
import { PREVIEW2_VENDOR_RELATIVE } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🕸️imports/🟦️.ts";
import type { BrowserArtifactSource } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📦️distribution/🟦️.ts";

/** 🧩️ Resolves the complete component union shared by distribution and development activation. */
export function demonstratorRuntimeComponentIds(): string[] {
  return runtimeComponentClosure([...PLUGIN_BUILD_TARGETS, ...EXTENSION_TARGETS], DEMONSTRATOR_RUNTIME_TARGETS.map(row => row.pluginId));
}

/** 🎪️ Selects only immutable component, browser-support and font outputs for the Demonstrator. */
export function demonstratorRuntimeAssetSources(workspace: string, profile: "prototype" | "release"): readonly BrowserArtifactSource[] {
  if (!["prototype", "release"].includes(profile)) throw new Error(`Unknown Demonstrator profile: ${profile}`);
  const staged = frameworkOsViteArtifactPathsV1(repositoryCachePartitionRootDataV1(workspace, "products"), profile);
  const moduleRoot = builtPluginModulesRootIn(workspace, profile);
  const components = [...PLUGIN_BUILD_TARGETS, ...EXTENSION_TARGETS], catalog = new Map(components.map(row => [row.pluginId, row]));
  const ids = demonstratorRuntimeComponentIds(), pluginRoute = MODULE_PLUGIN_ROUTE.slice(1);
  return [
    ...ids.map(id => {
      const row = catalog.get(id)!, name = moduleDirectoryName(id, COMPONENT_MODULE_DIRECTORIES), extension = row.role === "extension";
      return { root: join(moduleRoot, name), destination: `${(extension ? MODULE_EXTENSION_ROUTE : MODULE_PLUGIN_ROUTE).slice(1)}/${name}`, owner: `${row.cratePath}/Cargo.toml:browser:${profile}`, ...(extension ? { shimDirectory: `${pluginRoute}/${PREVIEW2_VENDOR_RELATIVE}` } : {}) };
    }),
    { root: join(moduleRoot, PREVIEW2_VENDOR_RELATIVE), destination: `${pluginRoute}/${PREVIEW2_VENDOR_RELATIVE}`, owner: `browser-support:${profile}:preview2` },
    { root: join(moduleRoot, MODULE_SHARD_DIRECTORY), destination: `${pluginRoute}/${MODULE_SHARD_DIRECTORY}`, owner: `browser-support:${profile}:shard` },
    { root: staged.fonts, destination: `${pluginRoute}/${MODULE_VENDOR_DIRECTORY}`, owner: "infinite:fonts" },
    ...staged.canvas.map(({root,destination,owner})=>({root,destination,owner})),
  ];
}
