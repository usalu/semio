#!/usr/bin/env bun
import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { join } from "node:path";

const REPO = "/Users/ueli/Documents/semio";
const SOURCE = join(REPO, "framework/product/playground/renderer/react/index.tsx");

const REGION_TARGETS: Record<string, string> = {
  Puzzle2dPlayHost: "puzzle/2d/react/index.tsx",
  Puzzle3dPlayHost: "puzzle/3d/react/index.tsx",
  Puzzle5dPlayHost: "puzzle/5d/react/index.tsx",
  MapPlayHost: "gis/2d/react/index.tsx",
  FlowPlayHost: "flow/react/index.tsx",
  DagPlayHost: "mathematical/graph/port/directed/dag/react/index.tsx",
  ImperativePlayHost: "imperative/react/index.tsx",
  SequencePlayHost: "sequence/react/index.tsx",
  LayoutPlayHost: "layout/react/index.tsx",
  LowpolyPlayHost: "lowpoly/react/index.tsx",
  TrinityPlayHost: "trinity/react/index.tsx",
  ProceduralPlayHost: "procedural/3d/react/index.tsx",
  Procedural2dPlayHost: "procedural/2d/react/index.tsx",
  ShootingPlayHost: "shooting/react/index.tsx",
  FormsPlayHost: "forms/react/index.tsx",
  RasterPlayHost: "raster/react/index.tsx",
  DrawPlayHost: "draw/react/index.tsx",
  NotePlayHost: "note/react/index.tsx",
  CadPlayHost: "cad/renderer/react/index.tsx",
  VcsPlayHost: "vcs/react/index.tsx",
  WriterPlayHost: "writer/react/index.tsx",
  PresentationPlayHost: "framework/product/presentation/renderer/react/index.tsx",
  SPlayHost: "s/react/index.tsx",
};

const CORE_BOOT_UPDATES: Array<{ corePath: string; reactPkg: string; bootFn: string }> = [
  { corePath: "forms/core/js/index.ts", reactPkg: "@semio-tech/forms-react", bootFn: "bootFormsPlay" },
  { corePath: "lowpoly/core/js/index.ts", reactPkg: "@semio-tech/lowpoly-react", bootFn: "bootLowpolyPlay" },
  { corePath: "flow/core/js/index.ts", reactPkg: "@semio-tech/flow-react", bootFn: "bootFlowPlay" },
  { corePath: "s/core/js/index.ts", reactPkg: "@semio-tech/s-react", bootFn: "bootSPlay" },
  { corePath: "draw/core/js/index.ts", reactPkg: "@semio-tech/draw-react", bootFn: "bootDrawPlay" },
  { corePath: "note/core/js/index.ts", reactPkg: "@semio-tech/note-react", bootFn: "bootNotePlay" },
  { corePath: "puzzle/5d/core/js/index.ts", reactPkg: "@semio-tech/puzzle-5d-react", bootFn: "boot5dPlay" },
  { corePath: "puzzle/2d/core/js/index.ts", reactPkg: "@semio-tech/puzzle-2d-react", bootFn: "boot2dPlay" },
  { corePath: "writer/core/js/index.ts", reactPkg: "@semio-tech/writer-react", bootFn: "bootWriterPlay" },
  { corePath: "procedural/3d/core/js/index.ts", reactPkg: "@semio-tech/procedural-3d-react", bootFn: "bootProceduralPlay" },
  { corePath: "procedural/2d/core/js/index.ts", reactPkg: "@semio-tech/procedural-2d-react", bootFn: "bootProcedural2dPlay" },
  { corePath: "mathematical/graph/port/directed/dag/core/js/index.ts", reactPkg: "@semio-tech/dag-react", bootFn: "bootDagPlay" },
  { corePath: "sequence/core/js/index.ts", reactPkg: "@semio-tech/sequence-react", bootFn: "bootSequencePlay" },
  { corePath: "layout/core/js/index.ts", reactPkg: "@semio-tech/layout-react", bootFn: "bootLayoutPlay" },
  { corePath: "cad/renderer/core/js/index.ts", reactPkg: "@semio-tech/cad-js-renderer-react", bootFn: "bootCadPlay" },
  { corePath: "reasoning/mindmap/wires/core/js/index.ts", reactPkg: "@semio-tech/puzzle-2d-react", bootFn: "bootWiresPlay" },
  { corePath: "puzzle/3d/core/js/index.ts", reactPkg: "@semio-tech/puzzle-3d-react", bootFn: "bootPuzzle3dPlay" },
  { corePath: "gis/2d/core/js/index.ts", reactPkg: "@semio-tech/gis-2d-react", bootFn: "bootMapPlay" },
  { corePath: "shooting/core/js/index.ts", reactPkg: "@semio-tech/shooting-react", bootFn: "bootShootingPlay" },
  { corePath: "raster/core/js/index.ts", reactPkg: "@semio-tech/raster-react", bootFn: "bootRasterPlay" },
  { corePath: "vcs/core/js/index.ts", reactPkg: "@semio-tech/vcs-react", bootFn: "bootVcsPlay" },
  { corePath: "framework/product/presentation/core/js/index.ts", reactPkg: "@semio-tech/framework-presentation-renderer-react", bootFn: "bootPresentationPlay" },
  { corePath: "trinity/rewrite/core/js/index.ts", reactPkg: "@semio-tech/trinity-react", bootFn: "bootTrinityRewritePlay" },
  { corePath: "trinity/jack/host-core/js/index.ts", reactPkg: "@semio-tech/trinity-react", bootFn: "bootTrinityJackPlay" },
  { corePath: "imperative/core/js/index.ts", reactPkg: "@semio-tech/imperative-react", bootFn: "bootImperativePlay" },
];

const PLAYGROUND_RENDERER_EXPORTS = [
  "bootPlayground",
  "mountPlaygroundApp",
  "PlaygroundView",
  "PlaygroundContext",
  "useApp",
  "PureSidePanelTabDefinition",
  "CallbackTreePanelDefinition",
  "StaticTreePanelDefinition",
  "playgroundStaticTreePanel",
  "registerUiPuzzle3dSurfaceHost",
  "registerUiPuzzle2dSurfaceHost",
  "registerUiGisMapSurfaceHost",
  "registerUiFlowSurfaceHost",
  "registerUiDagSurfaceHost",
  "registerUiImperativeSurfaceHost",
  "registerUiSequenceSurfaceHost",
  "registerUiLayoutSurfaceHost",
  "registerUiTrinitySurfaceHost",
  "registerUiTableSurfaceHost",
  "registerUiFormsSurfaceHost",
  "registerUiRasterSurfaceHost",
  "registerUiDrawSurfaceHost",
  "registerUiNoteSurfaceHost",
  "registerUiVcsSurfaceHost",
  "registerUiEditorSurfaceHost",
  "registerUiWriterSurfaceHost",
  "registerUiSSurfaceHost",
  "registerUiShootingSurfaceHost",
  "registerSurfaceBinding",
  "unregisterSurfaceBinding",
  "UiRenderer",
  "registerIcon",
  "registerTabIcon",
  "windowEngagementControlToGolden",
  "engagementSpecControlMirror",
  "windowEngagementToGolden",
  "windowKindsToGolden",
  "sideTabsToPlaygroundPanelTabs",
  "enforcePuzzle3dPlayWindowEngagement",
  "puzzle3dPlayEngagementMirror",
  "buildPuzzle2dPlayInspectorTree",
  "Platform",
  "CommandBus",
  "collectUiTreeItemDragData",
  "FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID",
  "FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL",
  "FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID",
  "FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL",
  "FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID",
  "FRAMEWORK_PANEL_TAB_INSPECTION_LABEL",
  "FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID",
  "FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL",
  "uiTreeNodeToTreePanelConfig",
  "registerUiPanelSurfaceHost",
  "renderComponentHostSurface",
  "renderUiControl",
  "declareToolsToViewTools",
  "hasToolbarViewTools",
  "mergePlatformFooterChromeRows",
  "createFrameworkDisplayPanelTabs",
  "createFrameworkSettingsPanelTabs",
  "sideTabsToPanelTabs",
  "useControllerStore",
  "useShellWindowInstance",
  "windowMeasuresToGolden",
  "registerWindowBody",
  "registerSidePanelBody",
  "getSidePanelBodyFactory",
  "getWindowBodyFactory",
  "buildCadWindowBody",
  "buildPuzzle3dWindowBody",
  "createDefaultLayout",
  "createStackLayout",
  "createWindowLayout",
  "toolCollection",
  "uiDeclarativeSectionsToTree",
  "uiInspectorAllEqual",
  "resolveAppState",
  "playgroundTreePanelRootItems",
  "enforcePlaygroundWindowEngagementInput",
  "enforceWindowKindsEngagementInput",
  "isPlaygroundExampleLocked",
  "playgroundResolvedExampleId",
  "resolvePlaygroundExampleCatalog",
  "PLAYGROUND_NO_EXAMPLE_ID",
  "isPlaygroundNoExampleId",
  "AppRuntime",
  "ModeRuntime",
  "WindowKindRuntime",
  "PlaygroundController",
  "Expertise",
  "PLAYGROUND_SYSTEM_SURFACE_CHROME",
  "PlaygroundShell",
];

const PLATFORM_RENDERER_EXPORTS = [
  "shellTabIconComponent",
  "ProductShell",
  "UIToolbar",
  "registerUiPanelSurfaceHost",
  "renderComponentHostSurface",
  "renderUiControl",
  "resolveDeclarativeControlIcon",
  "createFrameworkDisplayPanelTabs",
  "createFrameworkSettingsPanelTabs",
  "declareToolsToViewTools",
  "hasToolbarViewTools",
  "mergePlatformFooterChromeRows",
  "sideTabsToPanelTabs",
  "useControllerStore",
  "useShellWindowInstance",
  "windowMeasuresToGolden",
  "DisplayHostContext",
  "SettingsHostContext",
  "createBrowserStoragePort",
];

const UI_REACT_EXPORTS = ["reactHostPort", "cn", "Icon", "Button", "Toggle", "Select", "Tree", "LevelProvider", "getLevelBgClass", "useElementsSurfaceChrome", "bootstrapElementsSurfaceChromeDocument", "useMediaQuery", "useCommandHotkey", "createIconComponent", "ChromeAwareWindowScrollSurface", "PanelToggleGroup", "NavbarExampleSelect", "NAVBAR_NO_EXAMPLE_ID", "engagementCommandTokenEquals", "normalizeEngagementCommandText", "floatingFieldSurfaceClass", "floatingMenuSurfaceClass", "shellChromeSectionTitleClassName", "shellChromeTitleClassName", "interactiveActiveFillClass", "isCrossOriginIsolatedRuntime", "navbarFillItem", "readStoredComputeWorkerCount", "readStoredUiChromeCompact", "readStoredUiChromeExpertise", "readStoredUiChromeTheme", "writeStoredComputeWorkerCount", "writeStoredUiChromeCompact", "writeStoredUiChromeExpertise", "writeStoredUiChromeTheme", "renderControlIcon", "ButtonGroup", "ButtonGroupItem", "Input", "SelectContent", "SelectItem", "SelectTrigger", "SelectValue", "SemioLogo"];

const FRAMEWORK_CORE_EXPORTS = ["downloadMediaExportResult", "NamedLayoutStore", "CANVAS_HOVER_SOURCE_CANVAS", "CANVAS_HOVER_SOURCE_CATALOG", "CANVAS_HOVER_SOURCE_HIERARCHY"];

const PLAYGROUND_CORE_EXPORTS = ["SidePanelTabConfig", "TreePanelDefinition", "SidePanelTabDefinition", "TreePanelConfig", "TreeDataItem", "TreeDataSection", "SideTabSpec", "WindowEngagement", "WindowEngagementControl", "UiNode", "UiTreeNode", "UiSectionNode", "UiFieldNode", "UiInputNode", "UiSelectNode", "UiToggleNode", "UiTreeItemNode", "UiTreeSectionNode", "UiVec3Node", "UiKeyValueNode", "UiTableHostSurfaceNode", "UiPuzzle2dHostSurfaceNode", "UiPuzzle3dHostSurfaceNode"];

const PLAYGROUND_RENDERER_TYPES = ["Playground", "PlaygroundChromeBoot", "PlaygroundContextValue", "PlaygroundViewProps", "PlaygroundPanelVisibility", "PlaygroundExampleCatalog", "ResolvedAppState", "SidePanelBodyViewContext", "WindowBodyViewContext", "CommandDescriptor", "PlaygroundKeybinding", "Store", "UIWindowKindDefinition", "UIWindowMeasure", "UiComponentHostSurfaceNode", "DisplayHostApi", "SettingsHostApi"];

const PLATFORM_CORE_TYPES = ["UiFlowHostSurfaceNode", "UiFormsHostSurfaceNode", "UiWriterHostSurfaceNode", "UiDagHostSurfaceNode", "UiGisMapHostSurfaceNode", "UiImperativeHostSurfaceNode", "UiSequenceHostSurfaceNode", "UiLayoutHostSurfaceNode", "UiTrinityHostSurfaceNode", "UiShootingHostSurfaceNode", "UiRasterHostSurfaceNode", "UiDrawHostSurfaceNode", "UiNoteHostSurfaceNode", "UiVcsHostSurfaceNode", "UiEditorHostSurfaceNode", "UiSHostSurfaceNode"];

function extractRegion(source: string, regionName: string): string {
  const start = `//#region 🔖${regionName}`;
  const end = `//#endregion 🔖${regionName}`;
  const startIdx = source.indexOf(start);
  const endIdx = source.indexOf(end);
  if (startIdx < 0 || endIdx < 0) throw new Error(`Region ${regionName} not found`);
  return source.slice(startIdx, endIdx + end.length);
}

function removeAllPlayHostRegions(source: string): string {
  const firstPlayHost = source.indexOf("//#region 🔖Puzzle3dPlayHost");
  const bootRegion = source.indexOf("//#region 🔖Boot");
  if (firstPlayHost < 0 || bootRegion < 0) throw new Error("Could not find PlayHost/Boot boundaries");
  return source.slice(0, firstPlayHost) + "\n" + source.slice(bootRegion);
}

function collectUsedSymbols(region: string, symbols: string[]): string[] {
  const used: string[] = [];
  for (const sym of symbols) {
    const re = new RegExp(`\\b${sym.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}\\b`);
    if (re.test(region)) used.push(sym);
  }
  return used;
}

function formatImport(pkg: string, names: string[], typeNames: string[]): string {
  const valueNames = names.filter((n) => !typeNames.includes(n));
  const types = typeNames.filter((n) => names.includes(n));
  const parts: string[] = [];
  if (types.length) parts.push(`type ${types.join(", type ")}`);
  if (valueNames.length) parts.push(valueNames.join(", "));
  if (!parts.length) return "";
  return `import { ${parts.join(", ")} } from "${pkg}";`;
}

function buildFrameworkImports(region: string): string {
  const lines: string[] = [];
  const pgRendererValues = collectUsedSymbols(region, PLAYGROUND_RENDERER_EXPORTS);
  const pgRendererTypes = collectUsedSymbols(region, PLAYGROUND_RENDERER_TYPES);
  const allPgRenderer = [...new Set([...pgRendererValues, ...pgRendererTypes])];
  const mandatory = ["bootPlayground", "mountPlaygroundApp"];
  for (const m of mandatory) {
    if (!allPgRenderer.includes(m) && region.includes(m)) allPgRenderer.push(m);
  }
  if (region.includes("Playground") && !allPgRenderer.includes("Playground")) pgRendererTypes.push("Playground");
  if (region.includes("PlaygroundChromeBoot") && !allPgRenderer.includes("PlaygroundChromeBoot")) pgRendererTypes.push("PlaygroundChromeBoot");
  if (region.includes("ReactElement") || region.includes(": ReactElement")) pgRendererTypes.push("ReactElement");

  const pgTypes = [...new Set(pgRendererTypes.filter((t) => ["Playground", "PlaygroundChromeBoot", "PlaygroundContextValue", "PlaygroundViewProps", "PlaygroundPanelVisibility", "PlaygroundExampleCatalog", "ResolvedAppState", "SidePanelBodyViewContext", "WindowBodyViewContext", "CommandDescriptor", "PlaygroundKeybinding", "Store", "UIWindowKindDefinition", "UIWindowMeasure", "UiComponentHostSurfaceNode", "DisplayHostApi", "SettingsHostApi", "ReactElement"].includes(t) || region.match(new RegExp(`\\b${t}\\b`))))];
  const pgValues = [...new Set(pgRendererValues.filter((v) => !pgTypes.includes(v)))];

  const typeImports = pgTypes.filter((t) => t !== "ReactElement");
  const valueImports = [...pgValues];
  if (region.includes("ReactElement")) {
    lines.push(`import type { ReactElement } from "react";`);
  }
  const pgParts: string[] = [];
  if (typeImports.length) pgParts.push(`type ${typeImports.join(", type ")}`);
  if (valueImports.length) pgParts.push(valueImports.join(", "));
  if (pgParts.length) {
    lines.push(`import { ${pgParts.join(", ")} } from "@semio-tech/framework-playground-renderer-react";`);
  } else if (region.includes("bootPlayground") || region.includes("mountPlaygroundApp")) {
    lines.push(`import { bootPlayground, mountPlaygroundApp, type Playground, type PlaygroundChromeBoot } from "@semio-tech/framework-playground-renderer-react";`);
  }

  const platformValues = collectUsedSymbols(region, PLATFORM_RENDERER_EXPORTS).filter((s) => !allPgRenderer.includes(s));
  if (platformValues.length) {
    lines.push(formatImport("@semio-tech/framework-platform-renderer-react", platformValues, []));
  }

  const uiValues = collectUsedSymbols(region, UI_REACT_EXPORTS);
  if (uiValues.length) {
    lines.push(formatImport("@semio-tech/ui-react", uiValues, []));
  }

  const coreValues = collectUsedSymbols(region, FRAMEWORK_CORE_EXPORTS);
  if (coreValues.length) {
    lines.push(formatImport("@semio-tech/framework-core", coreValues, []));
  }

  const pgCoreValues = collectUsedSymbols(region, PLAYGROUND_CORE_EXPORTS);
  const pgCoreTypes = collectUsedSymbols(region, ["SidePanelTabConfig", "TreePanelDefinition", "SidePanelTabDefinition", "TreePanelConfig", "TreeDataItem", "TreeDataSection", "SideTabSpec", "WindowEngagement", "WindowEngagementControl"]);
  const allPgCore = [...new Set([...pgCoreValues, ...pgCoreTypes])].filter((s) => !allPgRenderer.includes(s));
  if (allPgCore.length) {
    const types = allPgCore.filter((s) => pgCoreTypes.includes(s));
    const values = allPgCore.filter((s) => !types.includes(s));
    const parts: string[] = [];
    if (types.length) parts.push(`type ${types.join(", type ")}`);
    if (values.length) parts.push(values.join(", "));
    lines.push(`import { ${parts.join(", ")} } from "@semio-tech/framework-playground-core";`);
  }

  if (region.includes("React.") && !region.includes('from "react"') && !region.includes("from 'react'")) {
    lines.push(`import * as React from "react";`);
  }

  return lines.filter(Boolean).join("\n");
}

function prependImportsToRegion(region: string, frameworkImports: string): string {
  const regionHeader = region.match(/^\/\/#region[^\n]*/)?.[0] ?? "";
  const body = region.slice(regionHeader.length).trimStart();
  const existingImports = body.match(/^(import[\s\S]*?)(?=\n(?:let |const |function |export |class |type |interface |\/\/#region|\/\/ #region))/);
  if (existingImports) {
    const afterImports = body.slice(existingImports[0].length);
    return `${regionHeader}\n${frameworkImports}\n${existingImports[0].trimEnd()}\n${afterImports}`;
  }
  return `${regionHeader}\n${frameworkImports}\n${body}`;
}

function appendRegionToTarget(targetPath: string, region: string): void {
  const fullPath = join(REPO, targetPath);
  if (!existsSync(fullPath)) {
    console.error(`MISSING TARGET: ${targetPath}`);
    return;
  }
  const existing = readFileSync(fullPath, "utf8");
  const frameworkImports = buildFrameworkImports(region);
  const enriched = prependImportsToRegion(region, frameworkImports);
  const separator = existing.endsWith("\n") ? "\n" : "\n\n";
  writeFileSync(fullPath, existing + separator + enriched + "\n");
}

function updateBootRenderer(corePath: string, reactPkg: string, bootFn: string): void {
  const fullPath = join(REPO, corePath);
  let content = readFileSync(fullPath, "utf8");
  const oldPattern = /await import\("@semio-tech\/framework-playground-renderer-react[^"]*"\)/g;
  const newImport = `await import("${reactPkg}")`;
  if (!content.includes("framework-playground-renderer-react")) return;
  content = content.replace(oldPattern, newImport);
  writeFileSync(fullPath, content);
}

function addPlaygroundRendererDep(reactPkgPath: string): void {
  const pkgPath = join(REPO, reactPkgPath, "package.json");
  if (!existsSync(pkgPath)) return;
  const pkg = JSON.parse(readFileSync(pkgPath, "utf8"));
  pkg.dependencies ??= {};
  if (!pkg.dependencies["@semio-tech/framework-playground-renderer-react"]) {
    pkg.dependencies["@semio-tech/framework-playground-renderer-react"] = "workspace:*";
  }
  if (!pkg.dependencies["@semio-tech/framework-playground-core"] && reactPkgPath.includes("presentation")) {
    pkg.dependencies["@semio-tech/framework-playground-core"] = "workspace:*";
  }
  writeFileSync(pkgPath, JSON.stringify(pkg, null, 2) + "\n");
}

// Main
const source = readFileSync(SOURCE, "utf8");
const extracted: string[] = [];

for (const [regionName, target] of Object.entries(REGION_TARGETS)) {
  const region = extractRegion(source, regionName);
  appendRegionToTarget(target, region);
  extracted.push(regionName);
  console.log(`Extracted ${regionName} -> ${target}`);
  const targetDir = target.replace(/\/index\.tsx$/, "");
  addPlaygroundRendererDep(targetDir);
}

const slimmed = removeAllPlayHostRegions(source);
writeFileSync(SOURCE, slimmed);
console.log("Removed PlayHost regions from source");

for (const { corePath, reactPkg, bootFn } of CORE_BOOT_UPDATES) {
  updateBootRenderer(corePath, reactPkg, bootFn);
  console.log(`Updated bootRenderer in ${corePath}`);
}

// Slim renderer package.json
const rendererPkgPath = join(REPO, "framework/product/playground/renderer/react/package.json");
const rendererPkg = JSON.parse(readFileSync(rendererPkgPath, "utf8"));
rendererPkg.exports = { ".": "./index.tsx" };
const keepDeps = new Set([
  "@semio-tech/framework-platform-core",
  "@semio-tech/framework-platform-renderer-react",
  "@semio-tech/framework-playground-core",
  "@semio-tech/ui-react",
  "clsx",
  "react",
  "react-dom",
  "tailwind-merge",
]);
rendererPkg.dependencies = Object.fromEntries(
  Object.entries(rendererPkg.dependencies).filter(([k]) => keepDeps.has(k)),
);
writeFileSync(rendererPkgPath, JSON.stringify(rendererPkg, null, 2) + "\n");
console.log("Slimmed renderer package.json");

// Slim playground core package.json
const corePkgPath = join(REPO, "framework/product/playground/core/package.json");
const corePkg = JSON.parse(readFileSync(corePkgPath, "utf8"));
const coreKeepDeps = new Set(["@semio-tech/framework-core", "@semio-tech/framework-platform-core"]);
corePkg.dependencies = Object.fromEntries(
  Object.entries(corePkg.dependencies).filter(([k]) => coreKeepDeps.has(k)),
);
writeFileSync(corePkgPath, JSON.stringify(corePkg, null, 2) + "\n");
console.log("Slimmed playground core package.json");

console.log("DONE");
