import { readFileSync, writeFileSync, mkdirSync, existsSync } from "fs";
import { join } from "path";

const MODULES = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules";
const CORE = join(MODULES, "🧩core");
const src = readFileSync(join(CORE, "🟦️component.ts"), "utf8");
const lines = src.split("\n");
// helper: slice 1-indexed inclusive, preserve trailing behavior
const slice = (a, b) => lines.slice(a - 1, b).join("\n");

const SHARED_HEADER = `/// <reference types="vitest/importMeta" />
`;

const SHARED_IMPORTS = `import { PLAYGROUND_BUILD_TARGETS, type PlaygroundBuildTarget } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🎯️targets/📋️registry/🟦️component.ts";
import { PLUGIN_BUILD_TARGETS, PLUGIN_HOST_CONFIGS, EXTENSION_TARGETS, pluginModuleUrl, extensionModuleUrl } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🎯️targets/📋️registry/🟦️component.ts";
import type { IconName } from "@semio-tech/assets";
export type { IconName };
import { SHELL_LOCALES, isShellLocale, SHELL_TERMINOLOGIES, isShellTerminology, type ShellLocale, type ShellTerminology, type LocalizedLabel } from "@semio-tech/framework-ui-styling";
export { SHELL_LOCALES, isShellLocale, SHELL_TERMINOLOGIES, isShellTerminology };
export type { ShellLocale, ShellTerminology, LocalizedLabel };
`;

// Verify imports match original lines 6-12
console.log("ORIG IMPORTS:");
console.log(lines.slice(5, 12).join("\n"));

// === MANIFEST ===
// Generated mirror + constants + pick/aliases + window/ui nodes + plugin/app contract (1860-2450) + tests
const genMirror = slice(14, 66).replace(
  'from "./🤖️generated/🟦️manifest.ts"',
  'from "./🤖️generated/🟦️manifest.ts"'
);

const manifestBody = [
  SHARED_HEADER,
  `/** @emoji 🛂️ \`@semio-tech/framework\` manifest — AppDefinition, PluginManifest, contributions, and declarative UI contract. */`,
  SHARED_IMPORTS,
  "",
  genMirror,
  "",
  slice(68, 90),
  "",
  slice(136, 161),
  "",
  slice(197, 582),
  "",
  // Plugin/app contract block (was under PluginRuntime but is manifest altitude)
  slice(1860, 2450),
  "",
  // Tests stay with the package surface; keep in manifest as the primary barrel owner
  slice(3660, 4090),
  "",
].join("\n");

// === MESH ===
const meshBody = [
  SHARED_HEADER,
  `/** @emoji 🔺️ \`@semio-tech/framework\` mesh — component scene protocol payloads shared by render hosts. */`,
  `import type { IconName } from "@semio-tech/assets";`,
  `import type { LocalizedLabel } from "@semio-tech/framework-ui-styling";`,
  `import type { ActionDescriptor } from "../🛂️manifest/🟦️component.ts";`,
  "",
  slice(583, 1210),
  "",
].join("\n");

// === PLATFORM ===
const platformBody = [
  SHARED_HEADER,
  `/** @emoji 🖥️ \`@semio-tech/framework\` platform — element ids, presence, dock/pane persistence, inspector helpers. */`,
  `import type { IconName } from "@semio-tech/assets";`,
  `import {`,
  `  type ActionDescriptor,`,
  `  type UiStackNode,`,
  `  type UiTreeNode,`,
  `  type UiControlNode,`,
  `  type UiFieldNode,`,
  `  type UiGroupNode,`,
  `  type UiInspectorFieldGroup,`,
  `  type UiNode,`,
  `  type UiSectionNode,`,
  `  type UiTreeItemNode,`,
  `  type UiTreeSectionNode,`,
  `  type CanvasPickTarget,`,
  `  type CanvasHoverFocus,`,
  `  type WindowLayout,`,
  `  UI_INSPECTOR_MIXED_PLACEHOLDER,`,
  `} from "../🛂️manifest/🟦️component.ts";`,
  "",
  slice(92, 135),
  "",
  slice(163, 196),
  "",
  slice(1211, 1651),
  "",
  slice(1704, 1859),
  "",
].join("\n");

// === ACTION-BUS ===
const actionBusBody = [
  SHARED_HEADER,
  `/** @emoji 🎯️ \`@semio-tech/framework\` action-bus — action arg resolution and utility/tool derivation helpers. */`,
  `import type {`,
  `  ActionArgDef,`,
  `  WindowMeasure,`,
  `  UtilityNode,`,
  `} from "../🛂️manifest/🟦️component.ts";`,
  // UtilityNode may be in manifest - verify
  "",
  slice(2451, 2608),
  "",
].join("\n");

// === KERNEL ===
const kernelBody = [
  SHARED_HEADER,
  `/** @emoji 🎠️ \`@semio-tech/framework\` kernel — plugin runtime, leases, invocation responses, and playground boot. */`,
  SHARED_IMPORTS,
  `import type {`,
  `  PluginManifest,`,
  `  PluginUiNode,`,
  `  AppDefinition,`,
  `  PluginContribution,`,
  `} from "../�️️manifest/🟦️component.ts";`,
  "",
  slice(1652, 1703),
  "",
  slice(2421, 2450), // PluginWasmHandle + buildContributionsJson start area - wait 2421-2450 overlaps manifest
  "",
].join("\n");

// Fix kernel more carefully - PluginWasmHandle starts 2421, expandPluginRegistry 2616, through 3658
// Manifest took 1860-2450 which INCLUDES PluginWasmHandle and buildContributionsJson and resolveLayoutForMode
// Adjust: manifest only to 2420, action-bus 2451-2608, kernel gets 2421-2450 + 2609-3658 + ephemeral

console.log("Adjusting ranges...");
console.log("line 2420:", lines[2419]?.slice(0,100));
console.log("line 2421:", lines[2420]?.slice(0,100));
console.log("line 2450:", lines[2449]?.slice(0,100));
console.log("line 2609:", lines[2608]?.slice(0,100));
console.log("line 2616:", lines[2615]?.slice(0,100));

// Check UtilityNode export and ActionArgDef
for (const name of ["UtilityNode", "ActionArgDef", "UiControlNode", "UiGroupNode", "UiNode", "UiTreeItemNode", "UiTreeSectionNode"]) {
  const i = lines.findIndex(l => l.includes(`export type ${name}`) || l.includes(`export interface ${name}`));
  console.log(name, i >= 0 ? i+1 : "MISSING", i>=0 ? lines[i].slice(0,100) : "");
}
