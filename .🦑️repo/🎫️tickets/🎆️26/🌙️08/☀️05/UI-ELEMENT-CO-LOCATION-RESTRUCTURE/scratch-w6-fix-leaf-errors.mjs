import { readdirSync, readFileSync, writeFileSync } from "fs";
import { join, relative } from "path";
const [, el, barrel] = readFileSync("/tmp/semio-w6-paths.txt", "utf8").trim().split("\n");
function resolveUnder(parent, bare) {
  const hits = readdirSync(parent).filter((n) => n === bare || n.endsWith(bare));
  hits.sort((a, b) => a.length - b.length);
  return join(parent, hits[0]);
}
function compFile(dir) {
  return join(dir, readdirSync(dir).find((n) => n.endsWith("component.tsx")));
}
const rel = (from, to) => {
  let r = relative(from, to).replaceAll("\\", "/");
  return r.startsWith(".") ? r : "./" + r;
};

const core = resolveUnder(el, "core");
const uiLabel = compFile(resolveUnder(core, "UiLabel"));
const cnFile = compFile(resolveUnder(core, "ClassNames"));
const iconsDir = resolveUnder(el, "Icons");
const icons = compFile(iconsDir);

// --- Icons ---
let it = readFileSync(icons, "utf8");
const iconsAdapters = `// #region 🔌️Adapters
import * as React from "react";
import { domSizePx, uiSpacingLen, activeUiTheme, subscribeActiveUiTheme, type UiTheme } from "@semio-tech/ui-styling";
import {
  ICONS,
  isIconName,
  resolveCatalogIconSvgFromTheme,
  shortcodeCatalogKey,
  shortcodeEmoji,
  type IconName,
} from "@semio-tech/assets";
import {
  isMetabolismIconName,
  METABOLISM_ICONS,
  resolveMetabolismIconSvgFromTheme,
  type MetabolismIconName,
} from "@semio-tech/assets";
import { cn } from "${rel(iconsDir, cnFile)}";
import { type UiLabel } from "${rel(iconsDir, uiLabel)}";
export type { IconName };
// #endregion 🔌️Adapters`;
it = it.replace(/\/\/ #region [^\n]*Adapters[\s\S]*?\/\/ #endregion [^\n]*Adapters/, iconsAdapters);
writeFileSync(icons, it);
console.log("Icons fixed");

// --- PanelTabBar ---
const ptbDir = resolveUnder(el, "PanelTabBar");
const ptb = compFile(ptbDir);
const ports = compFile(resolveUnder(core, "Ports"));
const iconsComp = icons;
const drag = compFile(resolveUnder(el, "DragHandle"));
const ribbon = compFile(resolveUnder(el, "Ribbon"));

// Discover DockSkeleton
const barrelText = readFileSync(barrel, "utf8");
const findExport = (name) => {
  const idx = barrelText.split("\n").findIndex((l) => new RegExp(`^export (type|interface|const|function) ${name}\\b`).test(l));
  return idx >= 0 ? idx + 1 : null;
};
console.log("DockSkeleton", findExport("DockSkeleton"), "DockTabSkeleton", findExport("DockTabSkeleton"), "PanelDockContextValue", findExport("PanelDockContextValue"));

const needed = [
  "type Anchor",
  "type FlowBlock",
  "FlowProvider",
  "useFlow",
  "flowFromAnchor",
  "ChromeControlHint",
  "useLabel",
  "useControlInlineText",
  "useLevel",
  "useNativeDragArm",
  "usePanelDockContext",
  "useUiDriverDragSurface",
  "useSurfaceActive",
  "panelTabIconSlotClass",
  "panelTabLabelClass",
  "interactiveActiveFillClass",
  "chromeControlTabItemClass",
  "panelTabButtonClass",
  "panelTabBarClass",
  "panelTabButtonDividerClass",
  "panelWindowInactiveTabClass",
  "modeDockInactiveTabBeforeGapClass",
  "modeDockInactiveTabClass",
  "modeDockTabClassName",
  "modeDockActiveTabClass",
  "modeDockActiveTabFillClass",
  "modeDockTabLabelClassName",
  "dropZoneReadyClass",
  "panelAnchorTabBarClass",
  "mobilePanelTabBarClass",
  "mobilePanelTabButtonClass",
  "panelAnchorTabButtonClass",
  "PANEL_TREE_UNIT_MIME",
  "beginPanelTreeUnitDrag",
  "endPanelTreeUnitDrag",
  "readActivePanelTreeUnitDrag",
  "usePanelTreeUnitDragActive",
  "type PanelDockContextValue",
  "type UiStatus",
  "type TreePanelSource",
  "ANCHORS",
];

// Check which exist
for (const item of [...needed]) {
  const name = item.replace(/^type /, "");
  if (!findExport(name) && !barrelText.includes(`export type ${name}`) && !barrelText.includes(`export interface ${name}`) && !barrelText.includes(`export const ${name}`) && !barrelText.includes(`export function ${name}`)) {
    // search softer
    if (!new RegExp(`\\b${name}\\b`).test(barrelText.split("\n").filter(l=>l.startsWith("export")).join("\n"))) {
      console.log("MISSING export", name);
    }
  }
}

const ag = resolveUnder(el, "ActionGroup");
const agLines = readFileSync(compFile(ag), "utf8").split("\n");
const adaptersOpen = agLines.find((l) => l.startsWith("// #region ") && l.includes("Adapters"));
const adaptersClose = agLines.find((l) => l.startsWith("// #endregion ") && l.includes("Adapters"));
const interimLine = agLines.find((l) => l.includes("W3-interim") && l.includes("remaining symbols"));
const headerSrc = readFileSync(ptb, "utf8").split("\n").slice(0, 6).join("\n");
const bodyMatch = readFileSync(ptb, "utf8").match(/\/\/ #region [^\n]*PanelTabBar[\s\S]*\/\/ #endregion [^\n]*PanelTabBar/);
if (!bodyMatch) throw new Error("no body");
const body = bodyMatch[0];

const file = `${headerSrc}

${adaptersOpen}
import * as React from "react";
import { type IconName } from "@semio-tech/assets";
import { reactHostPort } from "${rel(ptbDir, ports)}";
import { cn } from "${rel(ptbDir, cnFile)}";
import { type UiLabel } from "${rel(ptbDir, uiLabel)}";
import { Icon, type ControlIcon, renderControlIcon } from "${rel(ptbDir, iconsComp)}";
import { DragHandle } from "${rel(ptbDir, drag)}";
import { Ribbon, type RibbonRow } from "${rel(ptbDir, ribbon)}";
${interimLine}
import {
  ${needed.join(",\n  ")}
} from "${rel(ptbDir, barrel)}";
${adaptersClose}

${body}
`;
writeFileSync(ptb, file);
console.log("PanelTabBar rewritten adapters");
console.log(file.split("\n").slice(8, 60).join("\n"));
