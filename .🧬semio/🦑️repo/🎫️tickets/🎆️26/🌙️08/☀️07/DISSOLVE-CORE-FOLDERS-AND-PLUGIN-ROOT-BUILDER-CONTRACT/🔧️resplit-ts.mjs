import { readFileSync, writeFileSync, readdirSync, mkdirSync, existsSync, rmSync } from "fs";
import { join } from "path";

const ROOT = "/Users/ueli/Documents/semio";
const TICKET = join(
  ROOT,
  ".🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️07/DISSOLVE-CORE-FOLDERS-AND-PLUGIN-ROOT-BUILDER-CONTRACT",
);
const MODULES = join(ROOT, "🧰️framework", "🔨️modules");
const PKG = join(ROOT, "🧰️framework", "📦️packages", "🟦️typescript");
const original = readFileSync(join(TICKET, "original-component.ts"), "utf8");
const lines = original.split("\n");
const slice = (a, b) => lines.slice(a - 1, b).join("\n");

function findDir(needle) {
  const hit = readdirSync(MODULES).find((n) => n.includes(needle));
  if (!hit) throw new Error("missing " + needle);
  return hit;
}

const manifestDir = findDir("manifest");
const kernelDir = findDir("kernel");
const platformDir = findDir("platform");
const meshDir = findDir("mesh");
const actionDir = findDir("action-bus");
const comp = "🟦️component.ts";

const playImport = lines[5];
const pluginImport = lines[6];

const manifestImports = `${playImport}
${pluginImport}
import type { IconName } from "@semio-tech/assets";
export type { IconName };
import { SHELL_LOCALES, isShellLocale, SHELL_TERMINOLOGIES, isShellTerminology, type ShellLocale, type ShellTerminology, type LocalizedLabel } from "./🤖️generated/🟦️ui-axes.ts";
export { SHELL_LOCALES, isShellLocale, SHELL_TERMINOLOGIES, isShellTerminology };
export type { ShellLocale, ShellTerminology, LocalizedLabel };
`;

const kernelImports = `${playImport}
${pluginImport}
import type { IconName } from "@semio-tech/assets";
import type { ShellLocale, ShellTerminology, LocalizedLabel } from "../${manifestDir}/🤖️generated/🟦️ui-axes.ts";
`;

function writeModule(dir, body) {
  const p = join(MODULES, dir, comp);
  writeFileSync(p, body.endsWith("\n") ? body : body + "\n");
  console.log("wrote", dir, body.split("\n").length);
}

// Manifest: contract + declarative UI (no tests, no sibling imports)
writeModule(
  manifestDir,
  `// #region 🛂️Manifest
/// <reference types="vitest/importMeta" />
/** @emoji 🛂️ \`@semio-tech/framework\` — AppDefinition, PluginManifest, contributions, and declarative UI contract. */
${manifestImports}
${slice(14, 66)}

${slice(68, 90)}

${slice(136, 161)}

${slice(197, 582)}

//#region 🔌️PluginAndAppContract
${slice(1860, 2420)}
//#endregion 🔌️PluginAndAppContract
// #endregion 🛂️Manifest
`,
);

writeModule(
  meshDir,
  `// #region 🔺️Mesh
/// <reference types="vitest/importMeta" />
/** @emoji 🔺️ \`@semio-tech/framework\` — component scene protocol payloads shared by render hosts. */
import type { IconName } from "@semio-tech/assets";
import type { LocalizedLabel } from "../${manifestDir}/🤖️generated/🟦️ui-axes.ts";
import type { ActionDescriptor } from "../${manifestDir}/${comp}";

${slice(583, 1210)}
// #endregion 🔺️Mesh
`,
);

writeModule(
  platformDir,
  `// #region 🖥️Platform
/// <reference types="vitest/importMeta" />
/** @emoji 🖥️ \`@semio-tech/framework\` — element ids, presence, dock/pane persistence, and inspector helpers. */
import type { IconName } from "@semio-tech/assets";
import {
  type ActionDescriptor,
  type UiStackNode,
  type UiTreeNode,
  type UiControlNode,
  type UiFieldNode,
  type UiGroupNode,
  type UiInspectorFieldGroup,
  type UiNode,
  type UiSectionNode,
  type UiTreeItemNode,
  type UiTreeSectionNode,
  type CanvasPickTarget,
  type CanvasHoverFocus,
  type WindowLayout,
  UI_INSPECTOR_MIXED_PLACEHOLDER,
} from "../${manifestDir}/${comp}";

${slice(92, 135)}

${slice(163, 196)}

${slice(1211, 1651)}

${slice(1704, 1859)}
// #endregion 🖥️Platform
`,
);

writeModule(
  actionDir,
  `// #region 🎯️ActionBus
/// <reference types="vitest/importMeta" />
/** @emoji 🎯️ \`@semio-tech/framework\` — action arg resolution and utility/tool derivation helpers. */
import type { IconName } from "@semio-tech/assets";
import {
  type ActionArgDef,
  type ActionDefinition,
  type AppActionRef,
  type ToolDefinition,
  type ToolRef,
  type UtilityCategory,
  type UtilityNode,
  type WindowMeasure,
  SET_ACTIVE_TOOL_ACTION_ID,
  SET_ACTIVE_UTILITY_ACTION_ID,
} from "../${manifestDir}/${comp}";

${slice(2451, 2608)}
// #endregion 🎯️ActionBus
`,
);

writeModule(
  kernelDir,
  `// #region 🎠️Kernel
/// <reference types="vitest/importMeta" />
/** @emoji 🎠️ \`@semio-tech/framework\` — plugin runtime, leases, invocation responses, and playground boot. */
${kernelImports}
import type {
  PluginManifest,
  PluginUiNode,
  PluginViewState,
  ProgramContributionEntry,
  WindowLayout,
  NamedLayout,
} from "../${manifestDir}/${comp}";
import type { StoragePort } from "../${platformDir}/${comp}";

${slice(1652, 1703)}

${slice(2421, 2450)}

${slice(2609, 3658)}
// #endregion 🎠️Kernel
`,
);

const testsBlock = slice(3660, 4090);
const glue = `/** @emoji 📦️ \`@semio-tech/framework\` — package glue (reexports + inline vitest). */
export * from "../../🔨️modules/${actionDir}/${comp}";
export * from "../../🔨️modules/${platformDir}/${comp}";
export * from "../../🔨️modules/${meshDir}/${comp}";
export * from "../../🔨️modules/${manifestDir}/${comp}";
export * from "../../🔨️modules/${kernelDir}/${comp}";

import {
  organizeContextMenu,
} from "../../🔨️modules/${meshDir}/${comp}";
import {
  createMemoryStoragePort,
  emptyPaneState,
  emptySkeleton,
  emptyUiState,
  DockLayoutStore,
  DockUiStateStore,
  WindowPaneStateStore,
} from "../../🔨️modules/${platformDir}/${comp}";
import {
  createDevPluginSource,
  createExtensionSource,
  multiplexPluginSources,
  pluginWorkerUrl,
  resolvePlaygroundBoot,
  resolvePluginHostConfig,
  resolvePluginRegistryId,
  acquirePluginModule,
  evictPluginModule,
  createLeasePool,
} from "../../🔨️modules/${kernelDir}/${comp}";

${testsBlock}
`;
writeFileSync(join(PKG, "🟦️glue.ts"), glue);
console.log("glue lines", glue.split("\n").length);

// Ensure core is gone
const coreDir = readdirSync(MODULES).find((n) => n.endsWith("core") && n.includes("core"));
if (coreDir && /core$/.test(coreDir) && !coreDir.includes("manifest")) {
  // only delete puzzle-core
  const hex = Buffer.from(coreDir).toString("hex");
  if (hex.includes("636f7265") && coreDir.includes("core")) {
    rmSync(join(MODULES, coreDir), { recursive: true, force: true });
    console.log("removed leftover", coreDir);
  }
}
console.log(
  "module dirs",
  readdirSync(MODULES).filter((n) => /manifest|kernel|action|platform|mesh|core/.test(n)),
);
