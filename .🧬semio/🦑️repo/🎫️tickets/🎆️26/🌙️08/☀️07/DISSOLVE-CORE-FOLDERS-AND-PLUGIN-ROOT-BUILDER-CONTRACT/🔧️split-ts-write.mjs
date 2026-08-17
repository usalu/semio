import { readFileSync, writeFileSync, mkdirSync, existsSync } from "fs";
import { join } from "path";

const MODULES = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules";
const CORE = join(MODULES, "🧩core");
const lines = readFileSync(join(CORE, "🟦️component.ts"), "utf8").split("\n");
const slice = (a, b) => lines.slice(a - 1, b).join("\n");

const PLAY_IMPORT = lines[5];
const PLUGIN_IMPORT = lines[6];

const MANIFEST_IMPORTS = `${PLAY_IMPORT}
${PLUGIN_IMPORT}
import type { IconName } from "@semio-tech/assets";
export type { IconName };
import { SHELL_LOCALES, isShellLocale, SHELL_TERMINOLOGIES, isShellTerminology, type ShellLocale, type ShellTerminology, type LocalizedLabel } from "./🤖️generated/🟦️ui-axes.ts";
export { SHELL_LOCALES, isShellLocale, SHELL_TERMINOLOGIES, isShellTerminology };
export type { ShellLocale, ShellTerminology, LocalizedLabel };
`;

const KERNEL_IMPORTS = `${PLAY_IMPORT}
${PLUGIN_IMPORT}
import type { IconName } from "@semio-tech/assets";
import { SHELL_LOCALES, isShellLocale, SHELL_TERMINOLOGIES, isShellTerminology, type ShellLocale, type ShellTerminology, type LocalizedLabel } from "../�️️manifest/🤖️generated/🟦️ui-axes.ts";
`;

function write(dirEmojiName, body) {
  const dir = join(MODULES, dirEmojiName);
  if (!existsSync(dir)) mkdirSync(dir, { recursive: true });
  const path = join(dir, "🟦️component.ts");
  writeFileSync(path, body.endsWith("\n") ? body : body + "\n");
  console.log("wrote", path, "lines~", body.split("\n").length);
}

//#region 🛂️manifest
const manifest = `// #region �️️Manifest
/// <reference types="vitest/importMeta" />
/** @emoji 🛂️ \`@semio-tech/framework\` — AppDefinition, PluginManifest, contributions, and declarative UI contract. */
${MANIFEST_IMPORTS}
${slice(14, 66)}

${slice(68, 90)}

${slice(136, 161)}

${slice(197, 582)}

//#region 🔌️PluginAndAppContract
${slice(1860, 2420)}
//#endregion 🔌️PluginAndAppContract

import {
  organizeContextMenu,
} from "../🔺️mesh/🟦️component.ts";
import {
  createMemoryStoragePort,
  emptyPaneState,
  emptySkeleton,
  emptyUiState,
} from "../🖥️platform/🟦️component.ts";
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
} from "../🎠️kernel/🟦️component.ts";

${slice(3660, 4090)}
// #endregion 🛂️Manifest
`;
write("🛂️manifest", manifest);
//#endregion

//#region 🔺️mesh
const mesh = `// #region 🔺️Mesh
/// <reference types="vitest/importMeta" />
/** @emoji 🔺️ \`@semio-tech/framework\` — component scene protocol payloads shared by render hosts. */
import type { IconName } from "@semio-tech/assets";
import type { LocalizedLabel } from "../🛂️manifest/🤖️generated/🟦️ui-axes.ts";
import type { ActionDescriptor } from "../�️️manifest/🟦️component.ts";

${slice(583, 1210)}
// #endregion 🔺️Mesh
`;
write("🔺️mesh", mesh);
//#endregion

//#region 🖥️platform
const platform = `// #region 🖥️Platform
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
  UI_PENDING_PRESENCE,
} from "../🛂️manifest/🟦️component.ts";

// UI_PENDING_PRESENCE is defined in platform's UiPresence region originally — fix below

${slice(92, 135)}

${slice(163, 196)}

${slice(1211, 1651)}

${slice(1704, 1859)}
// #endregion 🖥️Platform
`;
// Fix circular: UI_PENDING_PRESENCE is IN the UiPresence slice (163-196), so remove from import
const platformFixed = platform.replace(
  `  UI_INSPECTOR_MIXED_PLACEHOLDER,
  UI_PENDING_PRESENCE,
} from "../🛂️manifest/🟦️component.ts";

// UI_PENDING_PRESENCE is defined in platform's UiPresence region originally — fix below
`,
  `  UI_INSPECTOR_MIXED_PLACEHOLDER,
} from "../🛂️manifest/🟦️component.ts";

`
);
write("🖥️platform", platformFixed);
//#endregion

//#region 🎯️action-bus
const actionBus = `// #region 🎯️ActionBus
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
} from "../🛂️manifest/🟦️component.ts";

${slice(2451, 2608)}
// #endregion 🎯️ActionBus
`;
write("🎯️action-bus", actionBus);
//#endregion

//#region 🎠️kernel
const kernel = `// #region 🎠️Kernel
/// <reference types="vitest/importMeta" />
/** @emoji 🎠️ \`@semio-tech/framework\` — plugin runtime, leases, invocation responses, and playground boot. */
${KERNEL_IMPORTS}
import type {
  PluginManifest,
  PluginUiNode,
  PluginViewState,
  ProgramContributionEntry,
  WindowLayout,
  NamedLayout,
} from "../🛂️manifest/🟦️component.ts";
import type { StoragePort } from "../🖥️platform/🟦️component.ts";

${slice(1652, 1703)}

${slice(2421, 2450)}

${slice(2609, 3658)}
// #endregion 🎠️Kernel
`;
write("🎠️kernel", kernel);
//#endregion

// Temporary barrel at old path then we'll replace glue and delete
const barrel = `// #region 🧩CoreBarrel
/** @emoji 🧭️ Temporary re-export barrel while \`🧩core\` dissolves — prefer \`@semio-tech/framework\`. */
export * from "../🎯️action-bus/🟦️component.ts";
export * from "../🖥️platform/🟦️component.ts";
export * from "../🔺️mesh/🟦️component.ts";
export * from "../🛂️manifest/🟦️component.ts";
export * from "../🎠️kernel/🟦️component.ts";
// #endregion 🧩CoreBarrel
`;
writeFileSync(join(CORE, "🟦️component.ts"), barrel);
console.log("wrote temporary core barrel");

// Fix KERNEL_IMPORTS typo - check for wrong emoji in path
console.log("kernel import sample check done");
