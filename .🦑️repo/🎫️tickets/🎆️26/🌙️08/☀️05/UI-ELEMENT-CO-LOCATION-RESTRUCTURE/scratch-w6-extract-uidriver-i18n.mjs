import { readdirSync, readFileSync, writeFileSync, mkdirSync, statSync } from "fs";
import { join, relative, dirname } from "path";

const ticket = process.argv[2];
if (!ticket) throw new Error("pass ticket dir");
const paths = JSON.parse(readFileSync(join(ticket, "scratch-w6-paths.json"), "utf8"));
const { barrel, coreDir, elDir } = paths;
const log = [];
const say = (m) => {
  console.log(m);
  log.push(String(m));
};

function resolveUnder(parent, bare) {
  const hits = readdirSync(parent).filter((n) => n === bare || n.endsWith(bare));
  if (!hits.length) throw new Error(`cannot resolve ${bare} under ${parent}`);
  hits.sort((a, b) => a.length - b.length);
  return join(parent, hits[0]);
}
function compName(dir) {
  return readdirSync(dir).find((n) => n.endsWith("component.tsx")) ?? "🟦️component.tsx";
}
function compFile(dir) {
  return join(dir, compName(dir));
}
const rel = (fromDir, toFile) => {
  let r = relative(fromDir, toFile).replaceAll("\\", "/");
  if (!r.startsWith(".")) r = "./" + r;
  return r;
};
function headerFrom(dirName) {
  return [
    "// #region 🧲️Header",
    `// 💻️ framework/ui/elements/�) {
  return [
    "// #region 🧲️Header",
    `// 💻️ framework/ui/elements/🟡️core/${dirName}/component.tsx`,
    "// 2026 Ueli Saluz <ueli@semio-tech.com>",
    "// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.",
    "// #endregion 🧲️Header",
  ].join("\n");
}
function walk(dir, acc = []) {
  for (const e of readdirSync(dir)) {
    if (e === "node_modules") continue;
    const fp = join(dir, e);
    if (statSync(fp).isDirectory()) walk(fp, acc);
    else if (/\.tsx?$/.test(e)) acc.push(fp);
  }
  return acc;
}
function extractFnEnd(lines, startIdx) {
  let d = 0;
  let started = false;
  for (let i = startIdx; i < lines.length; i++) {
    for (const ch of lines[i]) {
      if (ch === "{") {
        d++;
        started = true;
      }
      if (ch === "}") d--;
    }
    if (started && d === 0) return i;
  }
  throw new Error("unclosed fn at " + (startIdx + 1));
}
function findLine(lines, re, from = 0) {
  for (let i = from; i < lines.length; i++) if (re.test(lines[i])) return i;
  return -1;
}
function findIncl(lines, s, from = 0) {
  for (let i = from; i < lines.length; i++) if (lines[i].includes(s)) return i;
  return -1;
}
function replaceRange(lines, start, end, replacement) {
  lines.splice(start, end - start + 1, ...replacement);
}
function ensureCoreDir(emojiName) {
  const dir = join(coreDir, emojiName);
  mkdirSync(dir, { recursive: true });
  return dir;
}

const classNamesDir = resolveUnder(coreDir, "ClassNames");
const portsDir = resolveUnder(coreDir, "Ports");
const labelDir = resolveUnder(coreDir, "Label");
const classNamesFile = compFile(classNamesDir);
const portsFile = compFile(portsDir);
const labelFile = compFile(labelDir);
const barrelDir = dirname(barrel);

let lines = readFileSync(barrel, "utf8").split("\n");

// ============================================================================
// 1) ClassNames append
// ============================================================================
{
  let cn = readFileSync(classNamesFile, "utf8");
  if (!cn.includes("loadingBorderStateClass")) {
    if (!cn.includes('from "@semio-tech/ui-styling"')) {
      cn = cn.replace(
        /(\/\/ #endregion 🔌️Adapters\n)/,
        'import type { UiStatus } from "@semio-tech/ui-styling";\n$1',
      );
    }
    const append = `
//#region 🎨️ChromeControlClasses
/** @emoji 🌀️ Waiting ring matching the element's current state color; empty when not waiting. */
export function waitingBorderStateClass(waiting: boolean, active = false): string {
  return waiting ? (active ? waitingBorderActiveClass : waitingBorderClass) : "";
}

/** @emoji 🌀️ Loading ring matching the element's current state color; empty when not loading. */
export function loadingBorderStateClass(loading: boolean, active = false): string {
  return loading ? (active ? loadingBorderActiveClass : loadingBorderClass) : "";
}

/** @emoji � loadingBorderStateClass(loading: boolean, active = false): string {
  return loading ? (active ? loadingBorderActiveClass : loadingBorderClass) : "";
}

/** @emoji 🌀️ Maps shell chrome {@link UiStatus} to the shared border ring utilities. */
export function chromeStatusBorderClass(status: UiStatus | undefined, active = false): string {
  if (status === "loading") return loadingBorderStateClass(true, active);
  if (status === "waiting") return waitingBorderStateClass(true, active);
  return "";
}

/** @emoji 🎛️ Shared control cell base — transparent on the group glass. */
export const chromeControlItemBaseClass = cn(
  "text-element inline-flex items-center justify-center gap-single text-xs font-medium bg-transparent",
  "cursor-selectable disabled:pointer-events-none disabled:opacity-50 disabled:cursor-not-allowed",
  "[&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-small [&_svg]:shrink-0",
  formControlFocusBorderClass,
  "whitespace-nowrap h-medium p-single overflow-hidden leading-none",
);

/** @emoji 🎛️ Navbar/button/toggle cell hover — matches ShellParentHover group items. */
export const chromeControlItemClass = cn(chromeControlItemBaseClass, interactiveHoverClass);

/** @emoji 🎛️ Tab/chip cell hover — preserves drag-handle exclusion beside labels. */
export const chromeControlTabItemClass = cn(chromeControlItemBaseClass, hoverExcludingHandleBgFillClass, hoverExcludingHandleTextEmphasizedClass);

/** @emoji 📑️ Default mode-dock tab label — element gray; emphasize on hover/active only. */
export const modeDockTabClassName = cn(chromeControlTabItemClass, "group max-w-[12rem] shrink-0 cursor-pointer items-center px-single select-none transition-colors");

/** @emoji 📑️ Pane chrome toggle — leading semantic icon, label, trailing DragHandle. */
export const windowPaneChromeToggleClass = cn(
  modeDockTabClassName,
  "relative z-30 box-border min-h-medium shrink-0 border-0 bg-transparent",
  "outline-none focus-visible:ring-1 focus-visible:ring-inset focus-visible:ring-active-base",
  "disabled:pointer-events-none disabled:opacity-50",
);
//#endregion 🎨️ChromeControlClasses
`;
    if (cn.includes("//#endregion 🎨️StyleClasses")) {
      cn = cn.replace("//#endregion �🎨️ChromeControlClasses
`;
    if (cn.includes("//#endregion 🎨️StyleClasses")) {
      cn = cn.replace("//#endregion 🎨️StyleClasses", append + "\n//#endregion 🎨️StyleClasses");
    } else {
      cn = cn.trimEnd() + "\n" + append + "\n";
    }
    writeFileSync(classNamesFile, cn);
    say("ClassNames: appended border-state + chrome control classes");
  } else {
    say("ClassNames: already has loadingBorderStateClass");
  }
}

const classNamesExtras = [
  "waitingBorderStateClass",
  "loadingBorderStateClass",
  "chromeStatusBorderClass",
  "chromeControlItemBaseClass",
  "chromeControlItemClass",
  "chromeControlTabItemClass",
  "modeDockTabClassName",
  "windowPaneChromeToggleClass",
];

// ============================================================================
// 2) UiDriver core file
// ============================================================================
const uiDriverDir = ensureCoreDir("🚗️UiDriver");
const uiDriverFile = join(uiDriverDir, "🟦️component.tsx");
{
  const typesStart = findLine(lines, /^\/\/ #region 🚗️UiDriver$/);
  const typesEnd = findLine(lines, /^\/\/ #endregion 🚗️UiDriver$/, typesStart);
  if (typesStart < 0 || typesEnd < 0) throw new Error("UiDriver region missing");
  const typesBody = lines.slice(typesStart + 1, typesEnd).join("\n");

  const storageKey = findIncl(lines, "export const UI_CHROME_DRIVER_STORAGE_KEY");
  const storageFn = findIncl(lines, "export function readStoredUiDriver(storage: StoragePort)");
  const storageEnd = extractFnEnd(lines, storageFn);
  const storageBody = lines.slice(storageKey, storageEnd + 1).join("\n");

  const ctxStart = findIncl(lines, "const UiDriverContext = reactHostPort.createContext");
  const humanizeStart = findIncl(lines, "export function humanizeControlId(id: string)");
  const humanizeEnd = extractFnEnd(lines, humanizeStart);
  const hooksBody = lines.slice(ctxStart, humanizeEnd + 1).join("\n");

  const engStart = findIncl(lines, "export function humanizeEngagementStepId(stepId: string)");
  const engEnd = extractFnEnd(lines, engStart);
  const engDoc = lines[engStart - 1]?.includes("@emoji") ? lines[engStart - 1] + "\n" : "";
  const engBody = engDoc + lines.slice(engStart, engEnd + 1).join("\n");

  const body = [
    headerFrom("🚗️UiDriver"),
    "",
    "// #region 🔌️Adapters",
    'import * as React from "react";',
    'import { type StoragePort, createBrowserStoragePort } from "@semio-tech/framework-core";',
    `import { reactHostPort } from "${rel(uiDriverDir, portsFile)}";`,
    "// #endregion 🔌️Adapters",
    "",
    "// #region 🚗️UiDriver",
    typesBody,
    "",
    storageBody,
    "",
    hooksBody,
    "",
    engBody,
    "// #endregion 🚗️UiDriver",
    "",
  ].join("\n");
  writeFileSync(uiDriverFile, body);
  say("wrote " + uiDriverFile);
}

const uiDriverTypeExports = [
  "UiDriverLabels",
  "UiDriverLabelTier",
  "UiDriverDrag",
  "UiDriverReveal",
  "UiDriverTooltips",
  "UiDriverHotkeys",
  "UiDriver",
];
const uiDriverValueExports = [
  "DEFAULT_UI_DRIVER",
  "COMPACT_UI_DRIVER",
  "builtinUiDrivers",
  "parseUiDriver",
  "serializeUiDriver",
  "resolveUiDriver",
  "UI_CHROME_DRIVER_STORAGE_KEY",
  "readStoredUiDriverId",
  "writeStoredUiDriverId",
  "UI_CUSTOM_DRIVERS_STORAGE_KEY",
  "readStoredUiCustomDrivers",
  "writeStoredUiCustomDrivers",
  "readStoredUiDriver",
  "setUiDriverProvider",
  "activeUiDriver",
  "useUiDriver",
  "UiDriverProvider",
  "useUiDriverDragSurface",
  "useNativeDragArm",
  "useUiDriverTooltips",
  "setControlLabelIdResolver",
  "resolveControlLabelId",
  "panelKindFromPanelToggleControlId",
  "isInternalChromeControlId",
  "humanizeControlSegment",
  "humanizeControlId",
  "humanizeEngagementStepId",
];
const uiDriverAll = [...uiDriverTypeExports, ...uiDriverValueExports];

// ============================================================================
// 3) I18n core — schema/key types + branded registered key + UiTranslateFn
// ============================================================================
const i18nDir = ensureCoreDir("📚️I18n");
const i18nFile = join(i18nDir, "🟦️component.tsx");
{
  const labelPair = findIncl(lines, "export type UiLabelPair");
  const schemaEnd = findIncl(lines, "export interface UiI18nPort");
  const i18nPortEnd = extractFnEnd(lines, schemaEnd); // interface uses braces
  // include assert types before UiI18nPort
  const schemaBody = lines.slice(labelPair, i18nPortEnd + 1).join("\n");

  const brandStart = findIncl(lines, "declare const uiRegisteredTranslationKeyBrand");
  const brandEnd = findIncl(lines, "export type UiRegisteredTranslationKey");
  const brandBody = lines.slice(brandStart, brandEnd + 1).join("\n");

  const body = [
    headerFrom("📚️I18n"),
    "",
    "// #region 🔌️Adapters",
    'import { type ShellLocale, type ShellTerminology } from "@semio-tech/framework-core";',
    `import { type UiLabel } from "${rel(i18nDir, compFile(resolveUnder(coreDir, "UiLabel")))}";`,
    "// #endregion 🔌️Adapters",
    "",
    "// #region 📚️I18n",
    '/** @emoji 📚️ Supported UI locale codes — single source is framework-core ShellLocale. */',
    "export type UiLocale = ShellLocale;",
    "",
    schemaBody,
    "",
    brandBody,
    "// #endregion 📚️I18n",
    "",
  ].join("\n");
  // UiLabel may be unused — remove if schema doesn't need it
  let fixed = body;
  if (!schemaBody.includes("UiLabel") && !brandBody.includes("UiLabel")) {
    fixed = fixed.replace(/import \{ type UiLabel \} from "[^"]+";\n/, "");
  }
  writeFileSync(i18nFile, fixed);
  say("wrote " + i18nFile);
}

const i18nTypeExports = [
  "UiLocale",
  "UiLabelPair",
  "UiLabelValue",
  "UiRibbonParentCategory",
  "UiRibbonParentKey",
  "UiTranslationSchema",
  "UiTranslationKey",
  "AssertUiRibbonParentKeysCovered",
  "AssertUiSettingsLanguageKeysCovered",
  "UiChromeTerminologyId",
  "AssertUiSettingsTerminologyKeysCovered",
  "UiTranslateFn",
  "UiI18nPort",
  "UiRegisteredTranslationKey",
];
const i18nValueExports = ["UI_RIBBON_PARENT_CATEGORIES"];
const i18nAll = [...i18nTypeExports, ...i18nValueExports];

// ============================================================================
// 4) Chrome core — ChromeControlHint, LoadingRow, dead-line scroll
// ============================================================================
const chromeDir = ensureCoreDir("🎛️Chrome");
const chromeFile = join(chromeDir, "🟦️component.tsx");
{
  const hintStart = findIncl(lines, "export function ChromeControlHint");
  const hintDoc = lines[hintStart - 1]?.includes("@emoji") ? lines[hintStart - 1] + "\n" : "";
  const hintEnd = extractFnEnd(lines, hintStart);
  const hintBody = hintDoc + lines.slice(hintStart, hintEnd + 1).join("\n");

  const lrRegion = findLine(lines, /^\/\/ #region 🎺️LoadingRow$/);
  const lrEnd = findLine(lines, /^\/\/ #endregion 🎺️LoadingRow$/, lrRegion);
  const lrBody = lines.slice(lrRegion + 1, lrEnd).join("\n");

  const deadVar = findIncl(lines, "export const windowChromeScrollClearanceVar");
  const deadHook = findIncl(lines, "export function useWindowContentDeadLineScroll");
  const deadHookEnd = extractFnEnd(lines, deadHook);
  const deadBody = lines.slice(deadVar, deadHookEnd + 1).join("\n");

  const body = [
    headerFrom("🎛️Chrome"),
    "",
    "// #region 🔌️Adapters",
    'import * as React from "react";',
    'import { readSizeVarPx, STYLING_COMPACT_ROOT_PX, uiSpacingPx } from "@semio-tech/ui-styling";',
    `import { reactHostPort } from "${rel(chromeDir, portsFile)}";`,
    `import { cn, loadingBorderClass } from "${rel(chromeDir, classNamesFile)}";`,
    `import { useControlAccessibleLabel } from "${rel(chromeDir, labelFile)}";`,
    "// #endregion 🔌️Adapters",
    "",
    "// #region 🎛️Chrome",
    hintBody,
    "",
    "// #region 🎺️LoadingRow",
    lrBody,
    "// #endregion 🎺️LoadingRow",
    "",
    "// #region 🏝️WindowContentDeadLine",
    deadBody,
    "// #endregion 🏝️WindowContentDeadLine",
    "// #endregion 🎛️Chrome",
    "",
  ].join("\n");
  writeFileSync(chromeFile, body);
  say("wrote " + chromeFile);
}

const chromeExports = [
  "ChromeControlHint",
  "LoadingRowProps",
  "LoadingRow",
  "windowChromeScrollClearanceVar",
  "windowContentDeadLineVar",
  "windowContentDeadLineScrollClass",
  "readWindowChromeScrollClearancePx",
  "measureWindowChromeScrollClearancePx",
  "isWindowContentDeadLineHost",
  "readWindowContentDeadLinePx",
  "readScrollerContentOverflows",
  "useWindowContentDeadLineScroll",
];

// ============================================================================
// 5) FlowHost into Ports — HostReactFlow aliases + flowHostPort move
// ============================================================================
{
  let ports = readFileSync(portsFile, "utf8");
  if (!ports.includes("HostReactFlow")) {
    if (!ports.includes("@xyflow/react")) {
      ports = ports.replace(
        /(\/\/ #endregion 🔌️Adapters\n)/,
        'import { ReactFlow, ReactFlowProvider } from "@xyflow/react";\n$1',
      );
    }
    // update comment about flowHostPort staying in barrel
    ports = ports.replace(
      /`flowHostPort`\/`threeHostPort`\/`iconRenderPort`\/`configureHostPorts` stay in the barrel for\n \* now \(no top-level consumer needs them yet, same deferred-core posture as everything else under\n \* barrel-interim imports\)\./,
      "`threeHostPort`/`iconRenderPort`/`configureHostPorts` stay in the barrel for now; `flowHostPort` moved here so Diagram can import HostReactFlow without W3-interim.",
    );
    const flowBlock = `
/** @emoji 🕸️ Host surface for diagram runtime (implemented by Adapters). */
export interface FlowHostPort {
  readonly flow: typeof ReactFlow;
  readonly provider: typeof ReactFlowProvider;
}

/** @emoji 🔌️ Default diagram host port wired to @xyflow/react adapters. */
export let flowHostPort: FlowHostPort = {
  flow: ReactFlow,
  provider: ReactFlowProvider,
};

/** @emoji 🔌️ ESM-safe setter for {@link flowHostPort}. */
export function setFlowHostPort(port: FlowHostPort): FlowHostPort {
  const previous = flowHostPort;
  flowHostPort = port;
  return previous;
}

/** @emoji 🔌️ JSX alias for diagram flow host. */
export const HostReactFlow = flowHostPort.flow;
/** @emoji 🔌️ JSX alias for diagram flow provider. */
export const HostReactFlowProvider = flowHostPort.provider;
`;
    ports = ports.replace("//#endregion 🔌️Ports", flowBlock + "\n//#endregion 🔌️Ports");
    writeFileSync(portsFile, ports);
    say("Ports: added FlowHostPort + HostReactFlow");
  } else {
    say("Ports: already has HostReactFlow");
  }
}

// ============================================================================
// 6) RibbonZone into Ribbon element
// ============================================================================
const ribbonDir = resolveUnder(elDir, "Ribbon");
const ribbonFile = compFile(ribbonDir);
{
  const zoneRegion = findLine(lines, /^\/\/ #region 🎉️Ribbon Components$/) >= 0
    ? findLine(lines, /Ribbon Components/)
    : findIncl(lines, "interface RibbonZoneProps");
  // find RibbonZoneProps through export { RibbonDivider...}
  const propsStart = findIncl(lines, "interface RibbonZoneProps");
  const exportLine = findIncl(lines, "export { RibbonDivider, RibbonGroup, RibbonItem, RibbonZone }");
  if (propsStart < 0 || exportLine < 0) throw new Error("RibbonZone block missing");
  // include region comment if present just above
  let blockStart = propsStart;
  while (blockStart > 0 && (lines[blockStart - 1].startsWith("//") || lines[blockStart - 1].trim() === "")) blockStart--;
  const zoneBlock = lines.slice(propsStart, exportLine + 1).join("\n");

  let ribbon = readFileSync(ribbonFile, "utf8");
  if (!ribbon.includes("function RibbonZone")) {
    ribbon = ribbon.replace(
      /\/\/ 🚧️W3-interim:[\s\S]*?from "[^"]+";\n/,
      "",
    );
    ribbon = ribbon.replace(
      /(\/\/ #region 🎀️Ribbon\n)/,
      `$1// #region 🎉️RibbonZone\n${zoneBlock}\n// #endregion �Ribbon\n)/,
      `$1// #region 🎉️RibbonZone\n${zoneBlock}\n// #endregion 🎉️RibbonZone\n\n`,
    );
    writeFileSync(ribbonFile, ribbon);
    say("Ribbon: inlined RibbonZone helpers");
  } else {
    say("Ribbon: already has RibbonZone");
  }
}

// ============================================================================
// 7) Barrel surgery — remove moved defs, add import-then-export
// ============================================================================
function barrelImportExport(regionName, fromRel, names, typeNames = new Set()) {
  const importParts = names.map((n) => (typeNames.has(n) ? `type ${n}` : n));
  return [
    `// #region ${regionName}`,
    `import { ${importParts.join(", ")} } from "${fromRel}";`,
    `export { ${names.join(", ")} };`,
    `// #endregion ${regionName}`,
  ];
}

// Refresh lines after any prior edits? We still have original lines — good, we edit now.

// 7a) Expand ClassNames import/export list
{
  const impStart = findIncl(lines, "waitingBorderClass,");
  // find the import { block that contains waitingBorderClass from ClassNames
  let blockStart = impStart;
  while (blockStart > 0 && !lines[blockStart].trim().startsWith("import {")) blockStart--;
  const fromLine = findIncl(lines, "ClassNames/", blockStart);
  const expStart = findIncl(lines, "export {", fromLine);
  const expEnd = findIncl(lines, "};", expStart);
  // insert extras before closing of import and export
  const impClose = fromLine; // `} from ...`
  for (const name of classNamesExtras) {
    if (!lines.slice(blockStart, impClose + 1).join("\n").includes(name)) {
      lines.splice(impClose, 0, `  ${name},`);
    }
  }
  // re-find export after splice
  const expStart2 = findIncl(lines, "export {", findIncl(lines, "ClassNames/"));
  let expClose = expStart2;
  while (expClose < lines.length && lines[expClose].trim() !== "};") expClose++;
  for (const name of classNamesExtras) {
    if (!lines.slice(expStart2, expClose + 1).join("\n").includes(name)) {
      lines.splice(expClose, 0, `  ${name},`);
    }
  }
  say("barrel: ClassNames import/export expanded");
}

// 7b) Remove inline defs that moved to ClassNames (modeDock, border state, chrome control classes, windowPane)
function removeExportBlock(matchSubstr) {
  const idx = findIncl(lines, matchSubstr);
  if (idx < 0) {
    say("skip remove (missing): " + matchSubstr);
    return;
  }
  // walk back over doc comments
  let start = idx;
  while (start > 0 && (lines[start - 1].trim().startsWith("/**") || lines[start - 1].trim().startsWith("*") || lines[start - 1].trim() === "*/" || lines[start - 1].trim().startsWith("/** @emoji") || lines[start - 1].includes("@emoji") || lines[start - 1].trim() === "")) {
    if (lines[start - 1].includes("@emoji") || lines[start - 1].trim().startsWith("/**")) {
      start--;
      break;
    }
    start--;
  }
  // also grab consecutive /** @emoji */ lines immediately above
  while (start > 0 && lines[start - 1].includes("@emoji")) start--;
  let end;
  if (lines[idx].includes("= cn(") || lines[idx].includes("= cn (")) {
    end = idx;
    while (end < lines.length && !lines[end].includes(");")) end++;
  } else if (lines[idx].includes("function ") || lines[idx].includes("=> {") || lines[idx].endsWith("{")) {
    end = extractFnEnd(lines, idx);
  } else if (lines[idx].includes("= `") || lines[idx].includes('= "')) {
    end = idx;
  } else {
    end = extractFnEnd(lines, idx);
  }
  // remove trailing blank lines
  while (end + 1 < lines.length && lines[end + 1].trim() === "") end++;
  replaceRange(lines, start, end, []);
  say(`removed barrel block: ${matchSubstr} @${idx + 1}`);
}

removeExportBlock("export const windowPaneChromeToggleClass = cn(");
removeExportBlock("export const modeDockTabClassName = cn(");
removeExportBlock("export function chromeStatusBorderClass");
removeExportBlock("export function waitingBorderStateClass");
removeExportBlock("export function loadingBorderStateClass");
removeExportBlock("export const chromeControlItemBaseClass = cn(");
removeExportBlock("export const chromeControlItemClass = cn(");
removeExportBlock("export const chromeControlTabItemClass = cn(");

// 7c) Replace UiDriver region with import-then-export
{
  const typesStart = findLine(lines, /^\/\/ #region 🚗️UiDriver$/);
  const typesEnd = findLine(lines, /^\/\/ #endregion 🚗️UiDriver$/, typesStart);
  const fromRel = rel(barrelDir, uiDriverFile);
  const typeSet = new Set(uiDriverTypeExports);
  replaceRange(
    lines,
    typesStart,
    typesEnd,
    barrelImportExport("� = new Set(uiDriverTypeExports);
  replaceRange(
    lines,
    typesStart,
    typesEnd,
    barrelImportExport("🚗️UiDriver", fromRel, uiDriverAll, typeSet),
  );
  say("barrel: UiDriver region → import-then-export");
}

// 7d) Remove driver storage + hooks + ChromeControlHint from UiChromePrefs; wire import
{
  const storageKey = findIncl(lines, "export const UI_CHROME_DRIVER_STORAGE_KEY");
  if (storageKey >= 0) {
    const storageFn = findIncl(lines, "export function readStoredUiDriver(storage: StoragePort)");
    const storageEnd = extractFnEnd(lines, storageFn);
    let start = storageKey;
    while (start > 0 && lines[start - 1].includes("@emoji")) start--;
    replaceRange(lines, start, storageEnd, []);
    say("barrel: removed driver storage helpers");
  }

  const ctxStart = findIncl(lines, "const UiDriverContext = reactHostPort.createContext");
  if (ctxStart >= 0) {
    const humanizeStart = findIncl(lines, "export function humanizeControlId(id: string)");
    const humanizeEnd = extractFnEnd(lines, humanizeStart);
    // also remove ChromeControlHint after
    let end = humanizeEnd;
    const hint = findIncl(lines, "export function ChromeControlHint", end);
    if (hint >= 0 && hint < end + 20) {
      end = extractFnEnd(lines, hint);
      let hs = hint;
      while (hs > 0 && lines[hs - 1].includes("@emoji")) hs--;
      // remove from ctxStart through hint, but ctxStart may be before humanize
    }
    // Remove from UiDriverContext through ChromeControlHint
    let remStart = ctxStart;
    let remEnd = humanizeEnd;
    const hint2 = findIncl(lines, "export function ChromeControlHint", remStart);
    if (hint2 >= 0) remEnd = extractFnEnd(lines, hint2);
    while (remStart > 0 && lines[remStart - 1].trim() === "") remStart--;
    replaceRange(lines, remStart, remEnd, [
      `import { ${[...uiDriverValueExports, ...uiDriverTypeExports.map((t) => "type " + t)].join(", ")} } from "${rel(barrelDir, uiDriverFile)}";`,
      `export { ${uiDriverAll.join(", ")} };`,
      `import { ChromeControlHint } from "${rel(barrelDir, chromeFile)}";`,
      "export { ChromeControlHint };",
    ]);
    say("barrel: replaced driver hooks + ChromeControlHint with imports");
  }
}

// 7e) I18n schema keys → import (keep UiLabel stub + runtime asserts that need bundles)
{
  // Remove UiLocale through UiI18nPort (types we moved), keep asserts that reference uiChromeTranslationBundles
  const localeLine = findIncl(lines, "export type UiLocale = ShellLocale");
  const labelPair = findIncl(lines, "export type UiLabelPair");
  const start = localeLine >= 0 ? localeLine : labelPair;
  const i18nPort = findIncl(lines, "export interface UiI18nPort");
  const i18nPortEnd = extractFnEnd(lines, i18nPort);
  // Don't remove UiLabel stub region — it's before UiLabelPair
  // Remove from UiLocale (or UiLabelPair) through UiI18nPort
  let remStart = start;
  while (remStart > 0 && lines[remStart - 1].includes("@emoji")) remStart--;
  const fromRel = rel(barrelDir, i18nFile);
  const typeSet = new Set(i18nTypeExports);
  replaceRange(
    lines,
    remStart,
    i18nPortEnd,
    [
      `import { ${i18nAll.map((n) => (typeSet.has(n) ? "type " + n : n)).join(", ")} } from "${fromRel}";`,
      `export { ${i18nAll.join(", ")} };`,
    ],
  );
  say("barrel: I18n schema/types → import-then-export");

  // Replace UiRegisteredTranslationKey brand/type (keep registerUiTranslationBundles)
  const brand = findIncl(lines, "declare const uiRegisteredTranslationKeyBrand");
  if (brand >= 0) {
    const keyType = findIncl(lines, "export type UiRegisteredTranslationKey", brand);
    let startB = brand;
    while (startB > 0 && lines[startB - 1].includes("@emoji")) startB--;
    // may have multi-line doc before export type
    replaceRange(lines, startB, keyType, [
      `import { type UiRegisteredTranslationKey } from "${fromRel}";`,
      "export type { UiRegisteredTranslationKey };",
    ]);
    say("barrel: UiRegisteredTranslationKey → import");
  }
}

// 7f) HostReactFlow — use Ports
{
  const hr = findIncl(lines, "export const HostReactFlow = flowHostPort.flow");
  if (hr >= 0) {
    let start = hr;
    while (start > 0 && (lines[start - 1].includes("@emoji") || lines[start - 1].includes("JSX aliases"))) start--;
    const hr2 = findIncl(lines, "export const HostReactFlowProvider", start);
    replaceRange(lines, start, hr2, [
      `import { HostReactFlow, HostReactFlowProvider, flowHostPort, setFlowHostPort, type FlowHostPort } from "${rel(barrelDir, portsFile)}";`,
      "export { HostReactFlow, HostReactFlowProvider, flowHostPort, type FlowHostPort };",
    ]);
    say("barrel: HostReactFlow → Ports import");
  }
  // Update original FlowHostPort / flowHostPort definitions in Ports region to import-then-export
  const flowIface = findIncl(lines, "export interface FlowHostPort");
  if (flowIface >= 0) {
    const ifaceEnd = extractFnEnd(lines, flowIface);
    replaceRange(lines, flowIface, ifaceEnd, []);
    say("barrel: removed inline FlowHostPort interface");
  }
  const flowLet = findIncl(lines, "export let flowHostPort: FlowHostPort");
  if (flowLet >= 0) {
    // multi-line object
    let end = flowLet;
    while (end < lines.length && !lines[end].includes("};")) end++;
    replaceRange(lines, flowLet, end, [
      // already imported above in PortWiringAliases — but PortWiring needs the let binding locally.
      // Use import of flowHostPort + setFlowHostPort; re-export; configureHostPorts must call setFlowHostPort.
    ]);
    say("barrel: removed inline flowHostPort let (will fix configureHostPorts)");
  }
}

// Fix configureHostPorts to use setFlowHostPort
{
  for (let i = 0; i < lines.length; i++) {
    if (lines[i].includes("flowHostPort = overrides.flow")) {
      lines[i] = lines[i].replace("flowHostPort = overrides.flow", "setFlowHostPort(overrides.flow)");
    }
    if (lines[i].includes("flowHostPort = previous.flow")) {
      lines[i] = lines[i].replace("flowHostPort = previous.flow ?? defaultFlowHostPort", "setFlowHostPort(previous.flow ?? defaultFlowHostPort)");
    }
  }
  // Ensure setFlowHostPort is imported in the Ports import that has flowHostPort
  const portsImp = findIncl(lines, "HostReactFlow, HostReactFlowProvider, flowHostPort");
  if (portsImp >= 0 && !lines[portsImp].includes("setFlowHostPort")) {
    lines[portsImp] = lines[portsImp].replace("flowHostPort,", "flowHostPort, setFlowHostPort,");
  }
  // If flowHostPort import missing from early Ports region, add re-export near sceneHostPort
  if (findIncl(lines, "setFlowHostPort") < 0) {
    const sceneExp = findIncl(lines, "export { sceneHostPort");
    if (sceneExp >= 0) {
      lines.splice(sceneExp + 1, 0,
        `import { flowHostPort, setFlowHostPort, type FlowHostPort, HostReactFlow, HostReactFlowProvider } from "${rel(barrelDir, portsFile)}";`,
        "export { flowHostPort, type FlowHostPort, HostReactFlow, HostReactFlowProvider };",
      );
    }
  }
  say("barrel: configureHostPorts uses setFlowHostPort");
}

// 7g) Dead-line scroll + LoadingRow → Chrome imports
{
  const deadVar = findIncl(lines, "export const windowChromeScrollClearanceVar");
  if (deadVar >= 0) {
    const deadHook = findIncl(lines, "export function useWindowContentDeadLineScroll");
    const deadHookEnd = extractFnEnd(lines, deadHook);
    let start = deadVar;
    while (start > 0 && lines[start - 1].includes("@emoji")) start--;
    replaceRange(lines, start, deadHookEnd, [
      `import { ${chromeExports.filter((n) => n !== "ChromeControlHint" && n !== "LoadingRow" && n !== "LoadingRowProps").join(", ")} } from "${rel(barrelDir, chromeFile)}";`,
      `export { ${chromeExports.filter((n) => n !== "ChromeControlHint" && n !== "LoadingRow" && n !== "LoadingRowProps").join(", ")} };`,
    ]);
    say("barrel: dead-line scroll → Chrome import");
  }

  const lrRegion = findLine(lines, /^\/\/ #region 🎺️LoadingRow$/);
  if (lrRegion >= 0) {
    const lrEnd = findLine(lines, /^\/\/ #endregion 🎺️LoadingRow$/, lrRegion);
    replaceRange(lines, lrRegion, lrEnd, [
      "// #region 🎺️LoadingRow",
      `import { LoadingRow, type LoadingRowProps } from "${rel(barrelDir, chromeFile)}";`,
      "export { LoadingRow };",
      "export type { LoadingRowProps };",
      "// #endregion � } from "${rel(barrelDir, chromeFile)}";`,
      "export { LoadingRow };",
      "export type { LoadingRowProps };",
      "// #endregion 🎺️LoadingRow",
    ]);
    say("barrel: LoadingRow → Chrome import");
  }
}

// 7h) RibbonZone block → import from Ribbon
{
  const propsStart = findIncl(lines, "interface RibbonZoneProps");
  const exportLine = findIncl(lines, "export { RibbonDivider, RibbonGroup, RibbonItem, RibbonZone }");
  if (propsStart >= 0 && exportLine >= 0) {
    let start = propsStart;
    while (start > 0 && (lines[start - 1].startsWith("//") || lines[start - 1].trim() === "")) {
      if (lines[start - 1].includes("#region")) break;
      start--;
    }
    const regionStart = findIncl(lines, "Ribbon Components");
    const rs = regionStart >= 0 && regionStart < propsStart ? regionStart : start;
    // Keep region, replace body
    const regionEnd = findLine(lines, /#endregion.*Ribbon Components/, exportLine);
    replaceRange(lines, propsStart, exportLine, [
      `import { RibbonZone, RibbonDivider, RibbonGroup, RibbonItem } from "${rel(barrelDir, ribbonFile)}";`,
      "export { RibbonZone, RibbonDivider, RibbonGroup, RibbonItem };",
    ]);
    say("barrel: RibbonZone → Ribbon import");
  }
}

// 7i) humanizeEngagementStepId remove (now in UiDriver)
{
  const eng = findIncl(lines, "export function humanizeEngagementStepId");
  if (eng >= 0) {
    const end = extractFnEnd(lines, eng);
    let start = eng;
    while (start > 0 && lines[start - 1].includes("@emoji")) start--;
    replaceRange(lines, start, end, []);
    say("barrel: removed humanizeEngagementStepId def");
  }
}

// Deduplicate UiDriver imports if we inserted twice
{
  const seen = new Set();
  for (let i = lines.length - 1; i >= 0; i--) {
    if (lines[i].includes("UiDriver/") && lines[i].includes("import {")) {
      const key = lines[i];
      if (seen.has(key)) {
        // also remove following export line if present
        if (lines[i + 1]?.includes("export {") && lines[i + 1].includes("activeUiDriver")) {
          lines.splice(i, 2);
        } else {
          lines.splice(i, 1);
        }
        say("deduped UiDriver import");
      } else {
        seen.add(key);
      }
    }
  }
}

writeFileSync(barrel, lines.join("\n"));
say("wrote barrel");

// Region balance
{
  const text = lines.join("\n");
  const opens = (text.match(/^\/\/\s*#region/gm) || []).length;
  const closes = (text.match(/^\/\/\s*#endregion/gm) || []).length;
  say(`region balance: ${opens} open / ${closes} close ${opens === closes ? "OK" : "BROKEN"}`);
}

// ============================================================================
// 8) Rewire W3-interim leaves
// ============================================================================
const rewireMap = {
  activeUiDriver: uiDriverFile,
  useUiDriver: uiDriverFile,
  isInternalChromeControlId: uiDriverFile,
  resolveControlLabelId: uiDriverFile,
  panelKindFromPanelToggleControlId: uiDriverFile,
  humanizeEngagementStepId: uiDriverFile,
  humanizeControlId: uiDriverFile,
  UiTranslationKey: i18nFile,
  UiRegisteredTranslationKey: i18nFile,
  UiTranslateFn: i18nFile,
  ChromeControlHint: chromeFile,
  LoadingRow: chromeFile,
  useWindowContentDeadLineScroll: chromeFile,
  windowContentDeadLineScrollClass: chromeFile,
  HostReactFlow: portsFile,
  HostReactFlowProvider: portsFile,
  RibbonZone: ribbonFile,
  loadingBorderStateClass: classNamesFile,
  waitingBorderStateClass: classNamesFile,
  chromeControlItemBaseClass: classNamesFile,
  modeDockTabClassName: classNamesFile,
};

const typeOnly = new Set(["UiTranslationKey", "UiRegisteredTranslationKey", "UiTranslateFn", "LoadingRowProps"]);

let rewiredFiles = 0;
for (const file of walk(elDir)) {
  let text = readFileSync(file, "utf8");
  if (!text.includes("W3-interim")) continue;
  const m = text.match(/\/\/ � let text = readFileSync(file, "utf8");
  if (!text.includes("W3-interim")) continue;
  const m = text.match(/\/\/ 🚧️W3-interim:[\s\S]*?import\s*\{([^}]+)\}\s*from\s*"[^"]+";/);
  if (!m) {
    say("W3 marker without import: " + file);
    continue;
  }
  const names = m[1].split(",").map((p) => {
    const cleaned = p.replace(/\btype\b/g, "").trim();
    return cleaned.split(/\s+as\s+/)[0].trim();
  }).filter(Boolean);

  const byFile = new Map();
  const unresolved = [];
  for (const name of names) {
    const target = rewireMap[name];
    if (!target) {
      unresolved.push(name);
      continue;
    }
    if (!byFile.has(target)) byFile.set(target, []);
    byFile.get(target).push(name);
  }

  const fromDir = dirname(file);
  const importLines = [];
  for (const [target, syms] of byFile) {
    const parts = syms.map((n) => (typeOnly.has(n) ? `type ${n}` : n));
    importLines.push(`import { ${parts.join(", ")} } from "${rel(fromDir, target)}";`);
  }

  if (unresolved.length) {
    const still = `// � } from "${rel(fromDir, target)}";`);
  }

  if (unresolved.length) {
    const still = `// 🚧️W3-interim: remaining symbols still live in the ui-react barrel — clear before W6.\nimport { ${unresolved.map((n) => (typeOnly.has(n) ? `type ${n}` : n)).join(", ")} } from "${rel(fromDir, barrel)}";\n`;
    text = text.replace(m[0], importLines.join("\n") + "\n" + still);
    say(`rewired partial ${file}: ${names.length - unresolved.length} moved, ${unresolved.length} remain`);
  } else {
    text = text.replace(m[0], importLines.join("\n"));
    say(`rewired clear ${file}: ${names.join(", ")}`);
  }
  writeFileSync(file, text);
  rewiredFiles++;
}

const w3After = walk(elDir).filter((f) => readFileSync(f, "utf8").includes("W3-interim"));
say(`W3-interim after: ${w3After.length}`);
for (const f of w3After) say("  still: " + f);

writeFileSync(join(ticket, "scratch-w6-extract-uidriver-log.txt"), log.join("\n") + "\n");
writeFileSync(join(ticket, "scratch-w6-uidriver-w3-after.txt"), w3After.join("\n") + "\n");
say("DONE");
