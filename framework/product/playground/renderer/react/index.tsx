// #region 🧲Header
/** @emoji 🛝 Playground shell renderer: {@link PlaygroundView}, tree panels, puzzle play hosts, and surface hosts. */
// #endregion 🧲Header

// #region 🔌Adapters
import {
    Button,
    ButtonGroup,
    ButtonGroupItem,
    ChromeAwareWindowScrollSurface,
    Icon,
    Input,
    LevelProvider,
    NavbarExampleSelect,
    PanelToggleGroup,
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
    SemioLogo,
    Toggle,
    Tree,
    bootstrapElementsSurfaceChromeDocument,
    cn,
    createIconComponent,
    engagementCommandTokenEquals,
    floatingFieldSurfaceClass,
    floatingMenuSurfaceClass,
    getLevelBgClass,
    interactiveActiveFillClass,
    isCrossOriginIsolatedRuntime,
    navbarFillItem,
    normalizeEngagementCommandText,
    reactHostPort,
    readStoredComputeWorkerCount,
    readStoredUiChromeCompact,
    readStoredUiChromeExpertise,
    readStoredUiChromeTheme,
    shellChromeSectionTitleClassName,
    shellChromeTitleClassName,
    useCommandHotkey,
    useElementsSurfaceChrome,
    useMediaQuery,
    writeStoredComputeWorkerCount,
    writeStoredUiChromeCompact,
    writeStoredUiChromeExpertise,
    writeStoredUiChromeTheme,
    type ContextMenuItem,
    type ElementsSurfaceTheme,
    type EngagementControl,
    type EngagementSpec,
    type FooterItem,
    type NavbarItem,
    type PanelToggleItem,
    type SidePanelTabConfig,
    type SidePanelTabDefinition,
    type TreeDataItem,
    type TreeDataSection,
    type TreeDragAndDropController,
    type TreePanelConfig,
    type TreePanelDefinition,
    type TreePanelSource,
    type UiTranslationKey
} from "@semio-tech/ui-react";
import { clsx, type ClassValue } from "clsx";

//#region 🪁I18n Compile Gate
const _playgroundCadToolbarI18nKeys = [
  "ui.toolbar.parent.save",
  "ui.toolbar.parent.view",
  "ui.toolbar.parent.transform",
  "ui.toolbar.parent.transfer",
] as const satisfies readonly UiTranslationKey[];
//#endregion 🪁I18n Compile Gate
import { CANVAS_HOVER_SOURCE_CANVAS, CANVAS_HOVER_SOURCE_CATALOG, CANVAS_HOVER_SOURCE_HIERARCHY, NamedLayoutStore, downloadMediaExportResult } from "@semio-tech/framework-core";
import {
    DisplayHostContext,
    ProductShell,
    SettingsHostContext,
    UIToolbar,
    createBrowserStoragePort,
    createFrameworkDisplayPanelTabs,
    createFrameworkSettingsPanelTabs,
    declareToolsToViewTools,
    findDefaultActiveWindowKindId,
    hasToolbarViewTools,
    mergePlatformFooterChromeRows,
    registerSurfaceBinding,
    registerUiPanelSurfaceHost,
    renderComponentHostSurface,
    renderUiControl,
    resolveDeclarativeControlIcon,
    shellTabIconComponent,
    shellWindowScopeId,
    sideTabsToPanelTabs,
    uiTreeNodeToTreePanelConfig,
    unregisterSurfaceBinding,
    useControllerStore,
    useShellWindowInstance,
    windowMeasuresToGolden,
    type DisplayHostApi,
    type SettingsHostApi,
    type UIWindowMeasure,
    type UiComponentHostSurfaceNode
} from "@semio-tech/framework-platform-renderer-react";
import {
    CommandBus,
    Expertise,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
    FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID,
    FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
    FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
    PLAYGROUND_NO_EXAMPLE_ID,
    PRODUCT_SHELL_DEFAULT_PANEL_VISIBILITY,
    Platform,
    WindowKindRuntime,
    buildCadWindowBody,
    collectUiTreeItemDragData,
    enforcePlaygroundWindowEngagementInput,
    enforceWindowKindsEngagementInput,
    getSidePanelBodyFactory,
    getWindowBodyFactory,
    isEdgelessWindowBody,
    isPlaygroundExampleLocked,
    isPlaygroundNoExampleId,
    playgroundResolvedExampleId,
    registerSidePanelBody,
    registerWindowBody,
    resolveInitialPanelVisibility,
    resolvePlaygroundExampleCatalog,
    uiDeclarativeSectionsToTree,
    type CommandDescriptor,
    type Playground,
    type PlaygroundExampleCatalog,
    type PlaygroundKeybinding,
    type ResolvedAppState,
    type SidePanelBodyViewContext,
    type SideTabSpec,
    type UiFieldNode,
    type UiInputNode,
    type UiKeyValueNode,
    type UiNode,
    type UiPuzzle2dHostSurfaceNode,
    type UiPuzzle3dHostSurfaceNode,
    type UiSectionNode,
    type UiSelectNode,
    type UiTableHostSurfaceNode,
    type UiToggleNode,
    type UiTreeItemNode,
    type UiTreeNode,
    type UiTreeSectionNode,
    type UiVec3Node,
    type WindowBodyViewContext,
    type WindowEngagement,
    type WindowEngagementControl
} from "@semio-tech/framework-playground-core";
import { renderControlIcon } from "@semio-tech/ui-react";
import type { ReactElement } from "react";
import * as React from "react";
import { createRoot, type Root } from "react-dom/client";
import { twMerge } from "tailwind-merge";
// #endregion 🔌Adapters

export type {
    AppRuntime,
    AppTools,
    CommandBus,
    Controller,
    ModeRuntime, Platform, FooterItem as PlaygroundDeclarativeFooterItem, ResolvedAppState,
    SidePanelBodyViewContext,
    SideTabSpec,
    ToolLeaf,
    ToolNode,
    UiNode,
    WindowBodyViewContext,
    WindowKindRuntime,
    WindowLayout
} from "@semio-tech/framework-playground-core";

export {
    FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
    FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID,
    FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
    FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, PLAYGROUND_NO_EXAMPLE_ID,
    PLAYGROUND_NO_EXAMPLE_OPTION, isPlaygroundNoExampleId, playgroundExampleCatalogWithNoOption,
    resolvePlaygroundExampleCatalog
} from "@semio-tech/framework-playground-core";
export type { PlaygroundExampleCatalog, PlaygroundExampleHost, PlaygroundExampleOption } from "@semio-tech/framework-playground-core";

import {
    AppRuntime,
    CommandBus,
    ModeRuntime, Platform, PlaygroundController, WindowKindRuntime, buildCadWindowBody, buildPuzzle3dWindowBody,
    toolCollection,
    createDefaultLayout,
    createStackLayout,
    createWindowLayout,
    getSidePanelBodyFactory,
    getWindowBodyFactory, playgroundTreePanelRootItems, registerSidePanelBody,
    registerWindowBody,
    resolveAppState,
    uiInspectorAllEqual,
} from "@semio-tech/framework-playground-core";

export {
    AppRuntime,
    CommandBus,
    ModeRuntime, Platform, PlaygroundController, WindowKindRuntime, buildCadWindowBody, buildPuzzle3dWindowBody,
    toolCollection,
    createDefaultLayout,
    createStackLayout,
    createWindowLayout,
    getSidePanelBodyFactory,
    getWindowBodyFactory, playgroundTreePanelRootItems, registerSidePanelBody,
    registerWindowBody,
    resolveAppState,
    uiInspectorAllEqual,
};

export { uiTreeNodeToTreePanelConfig, useControllerStore, useStore } from "@semio-tech/framework-platform-renderer-react";
export type { Store } from "@semio-tech/framework-playground-core";

function cnPlay(...inputs: ClassValue[]): string {
  return twMerge(clsx(inputs));
}

//#region 🔖TreePanels
function isPlaygroundReactDescription(value: unknown): boolean {
  return typeof value === "object" && value !== null && "$$typeof" in value;
}

function enforcePlaygroundTreeItemsNoReactDescription(items: readonly TreeDataItem[], path: string): void {
  for (const item of items) {
    if (item.description != null && isPlaygroundReactDescription(item.description)) {
      throw new Error(`Playground tree item "${path}/${item.id}" must not use a React description; use item.control instead.`);
    }
    if (item.items?.length) {
      enforcePlaygroundTreeItemsNoReactDescription(item.items, `${path}/${item.id}`);
    }
  }
}

/** @emoji 🌲 Enforces playground panels: each section must declare tree items. */
export function enforcePlaygroundTreePanel(config: TreePanelConfig): void {
  if (!config.sections?.length) {
    throw new Error("Playground tree panel must declare at least one section.");
  }
  for (const section of config.sections) {
    if (!section.items?.length) {
      throw new Error(`Playground tree section "${section.id}" must declare at least one item.`);
    }
    enforcePlaygroundTreeItemsNoReactDescription(section.items, section.id);
  }
}

/** @emoji 📑 Abstract side-panel tab resolved to a {@link SidePanelTabConfig} tree. */
export abstract class PureSidePanelTabDefinition implements SidePanelTabDefinition {
  private cachedTab: SidePanelTabConfig | null = null;

  abstract buildTab(): SidePanelTabConfig;

  resolveTab(): SidePanelTabConfig {
    if (!this.cachedTab) {
      this.cachedTab = this.buildTab();
    }
    return this.cachedTab;
  }
}

/** @emoji 🌲 Static tree panel: sections + items only. */
export class StaticTreePanelDefinition implements TreePanelDefinition {
  constructor(private readonly config: TreePanelConfig) {
    enforcePlaygroundTreePanel(config);
  }

  resolveTree(): TreePanelConfig {
    return this.config;
  }
}

function treeItemFingerprint(items: readonly TreeDataItem[]): string {
  return items
    .map((item) => {
      const nested = item.items?.length ? `[${treeItemFingerprint(item.items)}]` : "";
      return `${item.id}${nested}`;
    })
    .join(",");
}

function treePanelSectionsFingerprint(sections: readonly TreeDataSection[]): string {
  return sections.map((section) => `${section.id}:${treeItemFingerprint(section.items ?? [])}`).join("|");
}

const CALLBACK_TREE_PANEL_EMPTY_HIGHLIGHTS: readonly string[] = [];

/** @emoji 🌲 Tree panel that rebuilds when the builder returns sections or a full {@link TreePanelConfig}. */
export class CallbackTreePanelDefinition implements TreePanelDefinition {
  private resolved: TreePanelConfig | null = null;
  private resolvedSectionsFingerprint: string | null = null;
  private resolvedHighlightedFingerprint: string | null = null;
  private resolvedSelectedFingerprint: string | null = null;

  constructor(
    private readonly buildTree: () => TreeDataSection[] | TreePanelConfig,
    private readonly buildHighlightedIds: () => readonly string[] = () => CALLBACK_TREE_PANEL_EMPTY_HIGHLIGHTS,
  ) {}

  resolveTree(): TreePanelConfig {
    const built = this.buildTree();
    const sections = Array.isArray(built) ? built : built.sections;
    const sectionsFingerprint = treePanelSectionsFingerprint(sections);
    const extraHighlightedIds = this.buildHighlightedIds();
    const highlightedIds =
      extraHighlightedIds.length > 0
        ? extraHighlightedIds
        : Array.isArray(built)
          ? CALLBACK_TREE_PANEL_EMPTY_HIGHLIGHTS
          : (built.highlightedIds ?? CALLBACK_TREE_PANEL_EMPTY_HIGHLIGHTS);
    const highlightedFingerprint = highlightedIds.join("\0");
    const selectedIds = Array.isArray(built) ? undefined : built.selectedIds;
    const selectedFingerprint = selectedIds?.join("\0") ?? "";
    if (
      this.resolved &&
      this.resolvedSectionsFingerprint === sectionsFingerprint &&
      this.resolvedHighlightedFingerprint === highlightedFingerprint &&
      this.resolvedSelectedFingerprint === selectedFingerprint
    ) {
      return this.resolved;
    }
    const config: TreePanelConfig = Array.isArray(built)
      ? { sections, highlightedIds }
      : { ...built, sections, highlightedIds };
    enforcePlaygroundTreePanel(config);
    this.resolved = config;
    this.resolvedSectionsFingerprint = sectionsFingerprint;
    this.resolvedHighlightedFingerprint = highlightedFingerprint;
    this.resolvedSelectedFingerprint = selectedFingerprint;
    return config;
  }
}

/** @emoji 🌲 Factory for a static {@link StaticTreePanelDefinition}. */
export function playgroundStaticTreePanel(config: TreePanelConfig): StaticTreePanelDefinition {
  return new StaticTreePanelDefinition(config);
}

function resolveTreePanelSource(tree: TreePanelSource): TreePanelConfig {
  if (typeof (tree as TreePanelDefinition).resolveTree === "function") {
    const config = (tree as TreePanelDefinition).resolveTree();
    enforcePlaygroundTreePanel(config);
    return config;
  }
  enforcePlaygroundTreePanel(tree as TreePanelConfig);
  return tree as TreePanelConfig;
}

function resolveSidePanelTabSource(tab: SidePanelTabConfig | SidePanelTabDefinition): SidePanelTabConfig {
  if (typeof (tab as SidePanelTabDefinition).resolveTab === "function") {
    const resolved = (tab as SidePanelTabDefinition).resolveTab();
    if (!resolved.panel && resolved.tree) {
      resolveTreePanelSource(resolved.tree);
    }
    return resolved;
  }
  const config = tab as SidePanelTabConfig;
  if (!config.panel && config.tree) {
    resolveTreePanelSource(config.tree);
  }
  return config;
}

//#region 🔖UiRenderer
type Puzzle3dSurfaceHost = React.ComponentType<{ readonly node: UiPuzzle3dHostSurfaceNode }>;
type Puzzle2dSurfaceHost = React.ComponentType<{ readonly node: UiPuzzle2dHostSurfaceNode }>;
type TableSurfaceHost = React.ComponentType<{ readonly node: UiTableHostSurfaceNode }>;
type PlaygroundSurfaceBindingHost = React.ComponentType<{ readonly node: UiComponentHostSurfaceNode }>;

const puzzle3dSurfaceHosts = new Map<string, Puzzle3dSurfaceHost>();
const puzzle2dSurfaceHosts = new Map<string, Puzzle2dSurfaceHost>();
type GisMapSurfaceHost = React.ComponentType<{ readonly node: import("@semio-tech/framework-platform-core").UiGisMapHostSurfaceNode }>;
const gisMapSurfaceHosts = new Map<string, GisMapSurfaceHost>();
type FlowSurfaceHost = React.ComponentType<{ readonly node: import("@semio-tech/framework-platform-core").UiFlowHostSurfaceNode }>;
const flowSurfaceHosts = new Map<string, FlowSurfaceHost>();
type DagSurfaceHost = React.ComponentType<{ readonly node: import("@semio-tech/framework-platform-core").UiDagHostSurfaceNode }>;
const dagSurfaceHosts = new Map<string, DagSurfaceHost>();
type ImperativeSurfaceHost = React.ComponentType<{ readonly node: import("@semio-tech/framework-platform-core").UiImperativeHostSurfaceNode }>;
const imperativeSurfaceHosts = new Map<string, ImperativeSurfaceHost>();
type SequenceSurfaceHost = React.ComponentType<{ readonly node: import("@semio-tech/framework-platform-core").UiSequenceHostSurfaceNode }>;
const sequenceSurfaceHosts = new Map<string, SequenceSurfaceHost>();
type LayoutSurfaceHost = React.ComponentType<{ readonly node: import("@semio-tech/framework-platform-core").UiLayoutHostSurfaceNode }>;
const layoutSurfaceHosts = new Map<string, LayoutSurfaceHost>();
type TrinitySurfaceHost = React.ComponentType<{ readonly node: import("@semio-tech/framework-platform-core").UiTrinityHostSurfaceNode }>;
const trinitySurfaceHosts = new Map<string, TrinitySurfaceHost>();
const tableSurfaceHosts = new Map<string, TableSurfaceHost>();
type FormsSurfaceHost = React.ComponentType<{ readonly node: import("@semio-tech/framework-platform-core").UiFormsHostSurfaceNode }>;
const formsSurfaceHosts = new Map<string, FormsSurfaceHost>();
type RasterSurfaceHost = React.ComponentType<{ readonly node: import("@semio-tech/framework-platform-core").UiRasterHostSurfaceNode }>;
const rasterSurfaceHosts = new Map<string, RasterSurfaceHost>();
type DrawSurfaceHost = React.ComponentType<{ readonly node: import("@semio-tech/framework-platform-core").UiDrawHostSurfaceNode }>;
const drawSurfaceHosts = new Map<string, DrawSurfaceHost>();
type NoteSurfaceHost = React.ComponentType<{ readonly node: import("@semio-tech/framework-platform-core").UiNoteHostSurfaceNode }>;
const noteSurfaceHosts = new Map<string, NoteSurfaceHost>();
type VcsSurfaceHost = React.ComponentType<{ readonly node: import("@semio-tech/framework-platform-core").UiVcsHostSurfaceNode }>;
const vcsSurfaceHosts = new Map<string, VcsSurfaceHost>();
type EditorSurfaceHost = React.ComponentType<{ readonly node: import("@semio-tech/framework-platform-core").UiEditorHostSurfaceNode }>;
const editorSurfaceHosts = new Map<string, EditorSurfaceHost>();
type WriterSurfaceHost = React.ComponentType<{ readonly node: import("@semio-tech/framework-platform-core").UiWriterHostSurfaceNode }>;
const writerSurfaceHosts = new Map<string, WriterSurfaceHost>();
type SSurfaceHost = React.ComponentType<{ readonly node: import("@semio-tech/framework-platform-core").UiSHostSurfaceNode }>;
const sSurfaceHosts = new Map<string, SSurfaceHost>();

const PLAYGROUND_CANVAS_HOST_TYPES = new Set(["puzzle2d", "puzzle3d", "puzzle5d", "cad", "gismap", "flow", "dag", "imperative", "sequence", "layout", "trinity", "shooting", "forms", "raster", "draw", "note", "writer", "s", "vcs", "editor"]);

function isPlaygroundCanvasHostChild(child: UiNode): boolean {
  return PLAYGROUND_CANVAS_HOST_TYPES.has(child.type);
}

/** @emoji 🧭 Binds a `surfaceId` from {@link UiPuzzle3dHostSurfaceNode} to a host React canvas implementation. */
export function registerUiPuzzle3dSurfaceHost(surfaceId: string, Component: Puzzle3dSurfaceHost): void {
  puzzle3dSurfaceHosts.set(surfaceId, Component);
  registerSurfaceBinding(surfaceId, Component as PlaygroundSurfaceBindingHost);
}

export { registerSurfaceBinding, unregisterSurfaceBinding };

/** @emoji 📋 Binds `surfaceId` from {@link UiPuzzle2dHostSurfaceNode} to a puzzle 2d canvas. */
export function registerUiPuzzle2dSurfaceHost(surfaceId: string, Component: Puzzle2dSurfaceHost): void {
  puzzle2dSurfaceHosts.set(surfaceId, Component);
  registerSurfaceBinding(surfaceId, Component as PlaygroundSurfaceBindingHost);
}

/** @emoji 🗺️ Binds `surfaceId` from {@link UiGisMapHostSurfaceNode} to a GIS map canvas. */
export function registerUiGisMapSurfaceHost(surfaceId: string, Component: GisMapSurfaceHost): void {
  gisMapSurfaceHosts.set(surfaceId, Component);
  registerSurfaceBinding(surfaceId, Component as PlaygroundSurfaceBindingHost);
}

/** @emoji 🌊 Binds `surfaceId` from {@link UiFlowHostSurfaceNode} to a flow canvas. */
export function registerUiFlowSurfaceHost(surfaceId: string, Component: FlowSurfaceHost): void {
  flowSurfaceHosts.set(surfaceId, Component);
  registerSurfaceBinding(surfaceId, Component as PlaygroundSurfaceBindingHost);
}

/** @emoji 🌳 Binds `surfaceId` from {@link UiDagHostSurfaceNode} to a DAG canvas. */
export function registerUiDagSurfaceHost(surfaceId: string, Component: DagSurfaceHost): void {
  dagSurfaceHosts.set(surfaceId, Component);
  registerSurfaceBinding(surfaceId, Component as PlaygroundSurfaceBindingHost);
}

/** @emoji ⚙️ Binds `surfaceId` from {@link UiImperativeHostSurfaceNode} to an imperative editor. */
export function registerUiImperativeSurfaceHost(surfaceId: string, Component: ImperativeSurfaceHost): void {
  imperativeSurfaceHosts.set(surfaceId, Component);
  registerSurfaceBinding(surfaceId, Component as PlaygroundSurfaceBindingHost);
}

/** @emoji 📜 Binds `surfaceId` from {@link UiSequenceHostSurfaceNode} to a sequence canvas. */
export function registerUiSequenceSurfaceHost(surfaceId: string, Component: SequenceSurfaceHost): void {
  sequenceSurfaceHosts.set(surfaceId, Component);
  registerSurfaceBinding(surfaceId, Component as PlaygroundSurfaceBindingHost);
}

/** @emoji 📄 Binds `surfaceId` from {@link UiLayoutHostSurfaceNode} to a layout canvas. */
export function registerUiLayoutSurfaceHost(surfaceId: string, Component: LayoutSurfaceHost): void {
  layoutSurfaceHosts.set(surfaceId, Component);
  registerSurfaceBinding(surfaceId, Component as PlaygroundSurfaceBindingHost);
}

/** @emoji 🔺 Binds `surfaceId` from {@link UiTrinityHostSurfaceNode} to a trinity canvas. */
export function registerUiTrinitySurfaceHost(surfaceId: string, Component: TrinitySurfaceHost): void {
  trinitySurfaceHosts.set(surfaceId, Component);
  registerSurfaceBinding(surfaceId, Component as PlaygroundSurfaceBindingHost);
}

/** @emoji 📊 Binds `surfaceId` from {@link UiTableHostSurfaceNode} to a host table body. */
export function registerUiTableSurfaceHost(surfaceId: string, Component: TableSurfaceHost): void {
  tableSurfaceHosts.set(surfaceId, Component);
  registerSurfaceBinding(surfaceId, Component as PlaygroundSurfaceBindingHost);
}

/** @emoji 📋 Binds `surfaceId` from {@link UiFormsHostSurfaceNode} to a forms surface. */
export function registerUiFormsSurfaceHost(surfaceId: string, Component: FormsSurfaceHost): void {
  formsSurfaceHosts.set(surfaceId, Component);
  registerSurfaceBinding(surfaceId, Component as PlaygroundSurfaceBindingHost);
}

/** @emoji 🖼️ Binds `surfaceId` from {@link UiRasterHostSurfaceNode} to a raster canvas. */
export function registerUiRasterSurfaceHost(surfaceId: string, Component: RasterSurfaceHost): void {
  rasterSurfaceHosts.set(surfaceId, Component);
  registerSurfaceBinding(surfaceId, Component as PlaygroundSurfaceBindingHost);
}

/** @emoji ✏️ Binds `surfaceId` from {@link UiDrawHostSurfaceNode} to a draw canvas. */
export function registerUiDrawSurfaceHost(surfaceId: string, Component: DrawSurfaceHost): void {
  drawSurfaceHosts.set(surfaceId, Component);
  registerSurfaceBinding(surfaceId, Component as PlaygroundSurfaceBindingHost);
}

/** @emoji 📝 Binds `surfaceId` from {@link UiNoteHostSurfaceNode} to a note canvas. */
export function registerUiNoteSurfaceHost(surfaceId: string, Component: NoteSurfaceHost): void {
  noteSurfaceHosts.set(surfaceId, Component);
  registerSurfaceBinding(surfaceId, Component as PlaygroundSurfaceBindingHost);
}

/** @emoji 🗄️ Binds `surfaceId` from {@link UiVcsHostSurfaceNode} to a vcs surface. */
export function registerUiVcsSurfaceHost(surfaceId: string, Component: VcsSurfaceHost): void {
  vcsSurfaceHosts.set(surfaceId, Component);
  registerSurfaceBinding(surfaceId, Component as PlaygroundSurfaceBindingHost);
}

/** @emoji ✍️ Binds `surfaceId` from {@link UiEditorHostSurfaceNode} to a code editor body. */
export function registerUiEditorSurfaceHost(surfaceId: string, Component: EditorSurfaceHost): void {
  editorSurfaceHosts.set(surfaceId, Component);
  registerSurfaceBinding(surfaceId, Component as PlaygroundSurfaceBindingHost);
}

/** @emoji ✍️ Binds `surfaceId` from {@link UiWriterHostSurfaceNode} to a writer canvas body. */
export function registerUiWriterSurfaceHost(surfaceId: string, Component: WriterSurfaceHost): void {
  writerSurfaceHosts.set(surfaceId, Component);
  registerSurfaceBinding(surfaceId, Component as PlaygroundSurfaceBindingHost);
}

/** @emoji 🖥️ Binds `surfaceId` from {@link UiSHostSurfaceNode} to a s studio surface. */
export function registerUiSSurfaceHost(surfaceId: string, Component: SSurfaceHost): void {
  sSurfaceHosts.set(surfaceId, Component);
  registerSurfaceBinding(surfaceId, Component as PlaygroundSurfaceBindingHost);
}

function renderPlaygroundHostSurface(node: UiNode, layout: "canvas" | "panel"): React.ReactElement {
  if (node.type === "puzzle2d") {
    const Host = puzzle2dSurfaceHosts.get(node.surfaceId);
    if (Host) {
      return (
        <div className="absolute inset-0 min-h-0 min-w-0">
          <Host node={node} />
        </div>
      );
    }
  }
  if (node.type === "gismap") {
    const Host = gisMapSurfaceHosts.get(node.surfaceId);
    if (Host) {
      return (
        <div className="absolute inset-0 min-h-0 min-w-0">
          <Host node={node} />
        </div>
      );
    }
  }
  if (node.type === "flow") {
    const Host = flowSurfaceHosts.get(node.surfaceId);
    if (Host) {
      return (
        <div className="absolute inset-0 min-h-0 min-w-0">
          <Host node={node} />
        </div>
      );
    }
  }
  if (node.type === "dag") {
    const Host = dagSurfaceHosts.get(node.surfaceId);
    if (Host) {
      return (
        <div className="absolute inset-0 min-h-0 min-w-0">
          <Host node={node} />
        </div>
      );
    }
  }
  if (node.type === "imperative") {
    const Host = imperativeSurfaceHosts.get(node.surfaceId);
    if (Host) {
      return (
        <div className="absolute inset-0 min-h-0 min-w-0">
          <Host node={node} />
        </div>
      );
    }
  }
  if (node.type === "sequence") {
    const Host = sequenceSurfaceHosts.get(node.surfaceId);
    if (Host) {
      return (
        <div className="absolute inset-0 min-h-0 min-w-0">
          <Host node={node} />
        </div>
      );
    }
  }
  if (node.type === "layout") {
    const Host = layoutSurfaceHosts.get(node.surfaceId);
    if (Host) {
      return (
        <div className="absolute inset-0 min-h-0 min-w-0">
          <Host node={node} />
        </div>
      );
    }
  }
  if (node.type === "trinity") {
    const Host = trinitySurfaceHosts.get(node.surfaceId);
    if (Host) {
      return (
        <div className="absolute inset-0 min-h-0 min-w-0">
          <Host node={node} />
        </div>
      );
    }
  }
  if (node.type === "shooting") {
    const Host = shootingSurfaceHosts.get(node.surfaceId);
    if (Host) {
      return (
        <div className="absolute inset-0 min-h-0 min-w-0">
          <Host node={node} />
        </div>
      );
    }
  }
  if (node.type === "forms") {
    const Host = formsSurfaceHosts.get(node.surfaceId);
    if (Host) {
      return (
        <ChromeAwareWindowScrollSurface className="absolute inset-0 min-h-0 min-w-0">
          <Host node={node} />
        </ChromeAwareWindowScrollSurface>
      );
    }
  }
  if (node.type === "raster") {
    const Host = rasterSurfaceHosts.get(node.surfaceId);
    if (Host) {
      return (
        <div className="absolute inset-0 min-h-0 min-w-0">
          <Host node={node} />
        </div>
      );
    }
  }
  if (node.type === "draw") {
    const Host = drawSurfaceHosts.get(node.surfaceId);
    if (Host) {
      return (
        <div className="absolute inset-0 min-h-0 min-w-0">
          <Host node={node} />
        </div>
      );
    }
  }
  if (node.type === "note") {
    const Host = noteSurfaceHosts.get(node.surfaceId);
    if (Host) {
      return (
        <div className="absolute inset-0 min-h-0 min-w-0">
          <Host node={node} />
        </div>
      );
    }
  }
  if (node.type === "vcs") {
    const Host = vcsSurfaceHosts.get(node.surfaceId);
    if (Host) {
      return (
        <div className="absolute inset-0 min-h-0 min-w-0 overflow-auto">
          <Host node={node} />
        </div>
      );
    }
  }
  if (node.type === "writer") {
    const Host = writerSurfaceHosts.get(node.surfaceId);
    if (Host) {
      return (
        <div className="absolute inset-0 min-h-0 min-w-0">
          <Host node={node} />
        </div>
      );
    }
  }
  if (node.type === "s") {
    const Host = sSurfaceHosts.get(node.surfaceId);
    if (Host) {
      return (
        <div className="absolute inset-0 min-h-0 min-w-0">
          <Host node={node} />
        </div>
      );
    }
  }
  if (node.type === "table") {
    const Host = tableSurfaceHosts.get(node.surfaceId);
    if (Host) {
      return (
        <ChromeAwareWindowScrollSurface className="relative min-h-0 min-w-0 flex-1">
          <Host node={node} />
        </ChromeAwareWindowScrollSurface>
      );
    }
  }
  if (node.type === "editor") {
    const Host = editorSurfaceHosts.get(node.surfaceId);
    if (Host) {
      return (
        <div className="relative min-h-0 min-w-0 flex-1 overflow-hidden">
          <Host node={node} />
        </div>
      );
    }
  }
  if (
    node.type === "cad" ||
    node.type === "puzzle2d" ||
    node.type === "puzzle3d" ||
    node.type === "puzzle5d" ||
    node.type === "flow" ||
    node.type === "dag" ||
    node.type === "trinity" ||
    node.type === "shooting" ||
    node.type === "forms" ||
    node.type === "raster" ||
    node.type === "draw" ||
    node.type === "panel" ||
    node.type === "table" ||
    node.type === "editor"
  ) {
    return renderComponentHostSurface(node as UiComponentHostSurfaceNode, layout);
  }
  const surfaceId = "surfaceId" in node ? String((node as { surfaceId: string }).surfaceId) : "?";
  return (
    <div className="flex flex-1 items-center justify-center p-4 text-xs text-muted-foreground">
      Unsupported {node.type} surface &quot;{surfaceId}&quot;
    </div>
  );
}

function stackClass(spec: { direction: "horizontal" | "vertical"; gap?: string; padding?: string }): string {
  const dir = spec.direction === "horizontal" ? "flex-row" : "flex-col";
  const gap = spec.gap === "none" ? "gap-0" : spec.gap === "tight" ? "gap-1" : spec.gap === "relaxed" ? "gap-4" : "gap-2";
  const pad = spec.padding === "none" ? "p-0" : "p-2";
  return cnPlay("flex", dir, gap, pad, spec.direction === "vertical" ? "min-h-0 min-w-0" : "min-w-0");
}

function dispatchUiCommand(bus: CommandBus, descriptor: CommandDescriptor, patch: Record<string, unknown>): void {
  bus.dispatch(descriptor.controllerId, descriptor.command, { ...(descriptor.args as object | undefined), ...patch });
}

function uiTreeContextMenuToTreeData(items: UiTreeItemNode["contextMenu"]): TreeDataItem["contextMenu"] {
  if (!items?.length) {
    return undefined;
  }
  return items.map((item) => ({
    id: item.id,
    label: item.label,
    icon: item.icon,
    disabled: item.disabled,
    onSelect: item.onSelect ? () => item.onSelect!() : undefined,
    children: item.children ? uiTreeContextMenuToTreeData(item.children) : undefined,
  }));
}

function uiTreeItemsToTreeData(items: readonly UiTreeItemNode[], commandBus: CommandBus): TreeDataItem[] {
  return items.map((item) => {
    return {
      id: item.id,
      label: item.label,
      description: item.description,
      icon: item.icon ? renderControlIcon(item.icon, 12) : undefined,
      control: item.control ? renderUiControl(item.control, commandBus) : undefined,
      defaultOpen: item.defaultOpen,
      isSelected: item.selected,
      isHidden: item.isHidden,
      draggable: item.draggable,
      dragData: item.dragData,
      className: item.draggable || item.dragData ? "cursor-grab active:cursor-grabbing" : undefined,
      items: item.items?.length ? uiTreeItemsToTreeData(item.items, commandBus) : undefined,
      actions: item.actions?.map((action) => ({
        kind: "button" as const,
        id: action.id,
        icon: action.icon,
        title: action.title,
        onClick: action.onClick,
        revealOnHover: action.revealOnHover,
      })),
      contextMenu: uiTreeContextMenuToTreeData(item.contextMenu),
      onClick: item.command
        ? () => {
            dispatchUiCommand(commandBus, item.command!, {});
          }
        : undefined,
      onPointerEnter: item.onPointerEnter,
      onPointerLeave: item.onPointerLeave,
    };
  });
}


function buildUiTreeDragAndDropController(sections: readonly UiTreeSectionNode[], commandBus: CommandBus): TreeDragAndDropController | undefined {
  void commandBus;
  if (import.meta.env.PUZZLE_PLAY_ENTRY === "map") {
    return undefined;
  }
  const dragByItemId = collectUiTreeItemDragData(sections);
  if (dragByItemId.size === 0) {
    return undefined;
  }
  const sample = dragByItemId.values().next().value;
  if (sample && FLOW_WIDGET_DRAG_MIME in sample) {
    return flowWidgetPaletteTreeDragController(dragByItemId);
  }
  if (sample && PUZZLE_2D_FIXTURE_DRAG_MIME in sample) {
    return puzzle2dFixturePaletteTreeDragController(dragByItemId);
  }
  return puzzle3dFixturePaletteTreeDragController(dragByItemId);
}

/** @emoji 🌲 Renders a declarative {@link UiTreeNode}; memoizes TreeData by stable {@link UiTreeNode.sections} identity. */
function PlaygroundDeclarativeTree(props: { readonly treeNode: UiTreeNode; readonly commandBus: CommandBus }): React.ReactElement {
  const { treeNode, commandBus } = props;
  const treeSections = treeNode.sections;
  const treeDataSections = reactHostPort.useMemo(() => uiTreeSectionsToTreeData(treeSections, commandBus), [treeSections, commandBus]);
  const dragAndDropController = reactHostPort.useMemo(() => buildUiTreeDragAndDropController(treeSections, commandBus), [treeSections, commandBus]);
  const selectedIds = treeNode.selectedIds as string[] | undefined;
  const highlightedIds = treeNode.highlightedIds;
  const onSelectionChange = reactHostPort.useCallback(
    (ids: string[]) => {
      if (!treeNode.selectionChange) {
        return;
      }
      dispatchUiCommand(commandBus, treeNode.selectionChange, { ids });
    },
    [commandBus, treeNode.selectionChange],
  );
  return (
    <div className="flex min-h-0 min-w-0 flex-1 flex-col overflow-hidden">
      <Tree
        className="min-h-0 flex-1 overflow-auto"
        sections={treeDataSections}
        selectionMode="single"
        showLines
        dragAndDropController={dragAndDropController}
        selectedIds={selectedIds}
        highlightedIds={highlightedIds}
        onSelectionChange={onSelectionChange}
      />
    </div>
  );
}

/** @emoji 🌲 Imperative helper for non-React callers (e.g. {@link UiRenderer} tree nodes). */
function renderPlaygroundDeclarativeTree(treeNode: UiTreeNode, commandBus: CommandBus): React.ReactElement {
  return <PlaygroundDeclarativeTree treeNode={treeNode} commandBus={commandBus} />;
}

function uiTreeSectionsToTreeData(sections: readonly UiTreeSectionNode[], commandBus: CommandBus): TreeDataSection[] {
  return sections.map((section) => ({
    id: section.id,
    label: section.label ?? "",
    defaultOpen: section.defaultOpen,
    items: uiTreeItemsToTreeData(section.items, commandBus),
  }));
}

function renderPlaygroundInput(node: UiInputNode, commandBus: CommandBus): React.ReactElement {
  const commitOnBlur = node.commit === "blur";
  return (
    <Input
      id={node.id}
      type={node.inputKind === "number" ? "number" : "text"}
      className="h-medium w-full min-w-0"
      value={node.value}
      placeholder={node.placeholder}
      onChange={
        commitOnBlur
          ? undefined
          : (event) => {
              const value = node.inputKind === "number" ? Number(event.target.value) : event.target.value;
              dispatchUiCommand(commandBus, node.onChange, { value });
            }
      }
      onBlur={
        commitOnBlur
          ? (event) => {
              const value = node.inputKind === "number" ? Number(event.target.value) : event.target.value;
              dispatchUiCommand(commandBus, node.onChange, { value });
            }
          : undefined
      }
    />
  );
}

function renderPlaygroundSelect(node: UiSelectNode, commandBus: CommandBus): React.ReactElement {
  return (
    <Select
      value={node.value || undefined}
      onValueChange={(value) => dispatchUiCommand(commandBus, node.onChange, { value })}
    >
      <SelectTrigger id={node.id} className="h-medium w-full min-w-0" size="sm">
        <SelectValue placeholder={node.placeholder ?? "Select"} />
      </SelectTrigger>
      <SelectContent>
        {node.items.map((item, index) => (
          <SelectItem key={`${node.id}:${index}:${item.value}`} value={item.value}>
            {item.label}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
}

function renderPlaygroundVec3(node: UiVec3Node, commandBus: CommandBus): React.ReactElement {
  const mixed = node.value === null;
  const axes = ["x", "y", "z"] as const;
  return (
    <div className="grid grid-cols-3 gap-1">
      {axes.map((axis, index) => (
        <Input
          key={`${node.id}.${axis}`}
          id={`${node.id}.${axis}`}
          type="number"
          className="h-medium w-full min-w-0"
          value={mixed ? "" : String(node.value![index] ?? 0)}
          placeholder={mixed ? "—" : axis}
          disabled={mixed}
          onChange={(event) => {
            if (mixed) return;
            const parsed = Number(event.target.value);
            if (!Number.isFinite(parsed)) return;
            const next: [number, number, number] = [...node.value!];
            next[index] = parsed;
            dispatchUiCommand(commandBus, node.onChange, { value: next });
          }}
        />
      ))}
    </div>
  );
}

export function UiRenderer({ node, commandBus }: { readonly node: UiNode; readonly commandBus: CommandBus }): React.ReactElement {
  switch (node.type) {
    case "stack": {
      const stack = node;
      const isFormStack = stack.direction === "vertical" && stack.padding === "standard" && !stack.children.some(isPlaygroundCanvasHostChild);
      return (
        <div
          className={cnPlay(
            stackClass(stack),
            stack.direction === "vertical" && stack.children.some(isPlaygroundCanvasHostChild) && "relative min-h-0 flex-1",
            isFormStack && "gap-single overflow-auto",
          )}
        >
          {stack.children.map((child, index) => (
            <UiRenderer key={index} node={child} commandBus={commandBus} />
          ))}
        </div>
      );
    }
    case "text":
      return <span className="text-muted-foreground px-1 text-xs">{node.value}</span>;
    case "button":
      return (
        <Button type="button" variant="outline" size="sm" onClick={() => commandBus.dispatch(node.command.controllerId, node.command.command, node.command.args)}>
          {node.label}
        </Button>
      );
    case "separator":
      return <span role="separator" className="bg-border my-1 h-px w-full shrink-0" aria-hidden />;
    case "puzzle2d":
    case "puzzle3d":
    case "puzzle5d":
    case "cad":
    case "gismap":
    case "flow":
    case "dag":
    case "imperative":
    case "sequence":
    case "layout":
    case "trinity":
    case "shooting":
    case "forms":
    case "raster":
    case "draw":
    case "note":
    case "vcs":
    case "writer":
    case "s":
    case "panel":
    case "table":
    case "editor":
      return renderPlaygroundHostSurface(node, node.type === "table" || node.type === "panel" || node.type === "editor" ? "panel" : "canvas");
    case "section": {
      const section = node as UiSectionNode;
      return (
        <div className={cn("flex flex-col gap-single p-single", floatingFieldSurfaceClass)} data-ui-section={section.id}>
          {section.label ? <div className={shellChromeSectionTitleClassName}>{section.label}</div> : null}
          <div className="flex flex-col gap-single">
            {section.children.map((child, index) => (
              <UiRenderer key={index} node={child} commandBus={commandBus} />
            ))}
          </div>
        </div>
      );
    }
    case "field": {
      const field = node as UiFieldNode;
      return (
        <div className="flex flex-col gap-half" data-ui-field={field.id}>
          <label className="text-muted-foreground text-xs" htmlFor={field.child.type === "input" || field.child.type === "select" ? (field.child as UiInputNode | UiSelectNode).id : field.id}>
            {field.label}
          </label>
          <UiRenderer node={field.child} commandBus={commandBus} />
        </div>
      );
    }
    case "input":
      return renderPlaygroundInput(node as UiInputNode, commandBus);
    case "select":
      return renderPlaygroundSelect(node as UiSelectNode, commandBus);
    case "toggle": {
      const toggle = node as UiToggleNode;
      return (
        <Toggle
          id={toggle.id}
          pressed={toggle.pressed}
          text={toggle.text}
          icon={resolveDeclarativeControlIcon(toggle.iconId)}
          onPressedChange={(pressed) => dispatchUiCommand(commandBus, toggle.onChange, { pressed })}
        />
      );
    }
    case "vec3":
      return renderPlaygroundVec3(node as UiVec3Node, commandBus);
    case "keyValue": {
      const keyValue = node as UiKeyValueNode;
      return (
        <dl className="grid grid-cols-[auto_1fr] gap-x-2 gap-y-1 text-xs">
          {keyValue.entries.map((entry) => (
            <React.Fragment key={entry.label}>
              <dt className="text-muted-foreground">{entry.label}</dt>
              <dd className="tabular-nums">{entry.value}</dd>
            </React.Fragment>
          ))}
        </dl>
      );
    }
    case "tree":
      return renderPlaygroundDeclarativeTree(node as UiTreeNode, commandBus);
    default:
      return <div className="p-2 text-xs text-destructive">Unsupported UiNode</div>;
  }
}
//#endregion 🔖UiRenderer

//#region 🔖DeclarativeHosts
import { registerIcon, registerTabIcon } from "@semio-tech/framework-platform-renderer-react";
export { registerIcon, registerTabIcon };

const declarativeWindowBodyComponents = new Map<string, React.FC>();

function getDeclarativeWindowBodyComponent(windowKindId: string, bodyKey: string): React.FC {
  const cacheKey = `${bodyKey}\0${windowKindId}`;
  let component = declarativeWindowBodyComponents.get(cacheKey);
  if (!component) {
    component = function ShellDeclarativeWindowBody() {
      const { runtime, activeModeId } = useApp();
      const generation = reactHostPort.useSyncExternalStore(
        (listener) => runtime.subscribe(listener),
        () => runtime.generation,
        () => 0,
      );
      const ctx: WindowBodyViewContext = {
        runtime,
        windowKindId,
        bodyKey,
        activeModeId: activeModeId ?? null,
        generation,
      };
      const factory = getWindowBodyFactory(bodyKey);
      const node = factory?.(ctx) ?? { type: "text", value: `Missing declarative body "${bodyKey}"` };
      return (
        <div
          data-window-content-layout={isEdgelessWindowBody(node) ? "edgeless" : "chrome-aware"}
          className="relative flex h-full min-h-0 min-w-0 flex-1 flex-col overflow-hidden"
        >
          <UiRenderer node={node} commandBus={runtime.commandBus} />
        </div>
      );
    };
    declarativeWindowBodyComponents.set(cacheKey, component);
  }
  return component;
}

export interface UIWindowKindDefinition {
  id: string;
  label?: string;
  component: React.ComponentType;
  measures?: UIWindowMeasure[];
  engagement?: EngagementSpec;
}

/** @emoji 🎛 Maps a neutral {@link WindowEngagementControl} to a ui {@link EngagementControl} with bus callbacks. */
export function windowEngagementControlToGolden(control: WindowEngagementControl | undefined, bus: CommandBus): EngagementControl | undefined {
  if (!control) return undefined;
  if (control.kind === "ring") {
    return {
      kind: "ring",
      id: control.id,
      label: control.label,
      value: control.value,
      disabled: control.disabled,
      options: control.options.map((row) => ({ id: row.id, label: row.label, disabled: row.disabled })),
      onSelect: control.onSelect
        ? (id: string) => bus.dispatch(control.onSelect!.controllerId, control.onSelect!.command, { ...(control.onSelect!.args as object | undefined), id })
        : undefined,
    };
  }
  const dispatchNumeric = (cmd: CommandDescriptor | undefined, value: number) => {
    if (!cmd) return;
    bus.dispatch(cmd.controllerId, cmd.command, { ...(cmd.args as object | undefined), value });
  };
  return {
    kind: control.kind,
    id: control.id,
    label: control.label,
    value: control.value,
    min: control.min,
    max: control.max,
    step: control.step,
    unit: control.unit,
    disabled: control.disabled,
    onChange: control.onChange ? (value: number) => dispatchNumeric(control.onChange, value) : undefined,
    onCommit: control.onCommit ? (value: number) => dispatchNumeric(control.onCommit, value) : undefined,
  };
}

/** @emoji 🎛 Mirrors a live ui {@link EngagementControl} into a neutral {@link WindowEngagementControl} with command routing. */
export function engagementSpecControlMirror(
  control: EngagementControl | undefined,
  controllerId: string,
  commandArgs: Record<string, unknown>,
): WindowEngagementControl | undefined {
  if (!control) return undefined;
  if (control.kind === "ring") {
    return {
      kind: "ring",
      id: control.id,
      label: control.label,
      value: control.value,
      disabled: control.disabled,
      options: control.options.map((row) => ({ id: row.id, label: row.label, disabled: row.disabled })),
      onSelect: control.onSelect ? { controllerId, command: "engagementControlSelect", args: { ...commandArgs } } : undefined,
    };
  }
  return {
    kind: control.kind,
    id: control.id,
    label: control.label,
    value: control.value,
    min: control.min,
    max: control.max,
    step: control.step,
    unit: control.unit,
    disabled: control.disabled,
    onChange: control.onChange ? { controllerId, command: "engagementControlChange", args: { ...commandArgs, controlId: control.id } } : undefined,
    onCommit: control.onCommit ? { controllerId, command: "engagementControlCommit", args: { ...commandArgs, controlId: control.id } } : undefined,
  };
}

/** @emoji 💬 Converts a React-neutral {@link WindowEngagement} into a ui {@link EngagementSpec} with bus-dispatching callbacks. */
export function windowEngagementToGolden(engagement: WindowEngagement | undefined, bus: CommandBus): EngagementSpec | undefined {
  if (!engagement) return undefined;
  const options = engagement.options?.map((option) => ({
    id: option.id,
    label: option.label,
    icon: option.iconId ? shellTabIconComponent(option.iconId, "details")({}) : undefined,
    pressed: option.pressed,
    disabled: option.disabled,
    onPress: option.command ? () => bus.dispatch(option.command!.controllerId, option.command!.command, option.command!.args) : undefined,
  }));
  const input = engagement.input
    ? {
        id: engagement.input.id,
        value: engagement.input.value,
        placeholder: engagement.input.placeholder,
        disabled: engagement.input.disabled,
        onChange: engagement.input.onChange ? (value: string) => bus.dispatch(engagement.input!.onChange!.controllerId, engagement.input!.onChange!.command, { ...(engagement.input!.onChange!.args as object | undefined), value }) : undefined,
        onSubmit: engagement.input.onSubmit ? (value: string) => bus.dispatch(engagement.input!.onSubmit!.controllerId, engagement.input!.onSubmit!.command, { ...(engagement.input!.onSubmit!.args as object | undefined), value }) : undefined,
        onRepeatLast: engagement.input.onRepeatLast
          ? () => bus.dispatch(engagement.input!.onRepeatLast!.controllerId, engagement.input!.onRepeatLast!.command, engagement.input!.onRepeatLast!.args)
          : undefined,
        onAbort: engagement.input.onAbort ? () => bus.dispatch(engagement.input!.onAbort!.controllerId, engagement.input!.onAbort!.command, engagement.input!.onAbort!.args) : undefined,
      }
    : undefined;
  const status = engagement.status?.map((row) => ({ id: row.id, content: row.text }));
  const possibleEngagements = engagement.possibleEngagements?.map((row) => ({
    id: row.id,
    label: row.label,
    detail: row.detail,
    onSelect: row.command ? () => bus.dispatch(row.command!.controllerId, row.command!.command, row.command!.args) : undefined,
  }));
  const control = windowEngagementControlToGolden(engagement.control, bus);
  const controls = engagement.controls?.map((row) => windowEngagementControlToGolden(row, bus)).filter((row): row is EngagementControl => row !== undefined);
  const hasContent =
    (options?.length ?? 0) > 0 || Boolean(input) || Boolean(control) || (controls?.length ?? 0) > 0 || (status?.length ?? 0) > 0 || (possibleEngagements?.length ?? 0) > 0;
  if (!hasContent) return undefined;
  return { sessionActive: engagement.sessionActive, options, input, control, controls, status, possibleEngagements };
}

export function windowKindsToGolden(windowKinds: readonly WindowKindRuntime[], bus: CommandBus): UIWindowKindDefinition[] {
  enforceWindowKindsEngagementInput(windowKinds, "Playground app");
  return windowKinds.map((wk) => ({
    id: wk.id,
    label: wk.label,
    component: getDeclarativeWindowBodyComponent(wk.id, wk.bodyKey),
    measures: windowMeasuresToGolden(wk.measures, bus),
    engagement: windowEngagementToGolden(wk.engagement, bus),
  }));
}

/** @emoji 📑 Converts playground side tabs into enforced tree panel configs (sections with items). */
export function sideTabsToPlaygroundPanelTabs(tabs: readonly SideTabSpec[], runtime: Platform, bus: CommandBus): SidePanelTabConfig[] {
  return sideTabsToPanelTabs(tabs, runtime, bus);
}

/** @emoji 🌲 Declarative `type: "tree"` workbench tab mounted as the side-panel root (no nested shell tree). */
function DeclarativeTreeWorkbenchPanel(props: { readonly tabId: string; readonly bodyKey: string }): React.ReactElement {
  const { runtime, activeModeId } = useApp();
  reactHostPort.useSyncExternalStore(
    (listener) => runtime.subscribe(listener),
    () => runtime.generation,
    () => 0,
  );
  usePuzzle3dPlaySnapshotPanelRefresh(props.bodyKey);
  const ctx: SidePanelBodyViewContext = {
    runtime,
    windowKindId: props.tabId,
    bodyKey: props.bodyKey,
    activeModeId: activeModeId ?? null,
    generation: runtime.generation,
  };
  const bus = runtime.commandBus;
  const factory = getSidePanelBodyFactory(props.bodyKey);
  const node = factory?.(ctx);
  if (node?.type !== "tree") {
    return <div className="text-destructive p-single text-xs">Expected tree panel {props.bodyKey}</div>;
  }
  return <PlaygroundDeclarativeTree treeNode={node} commandBus={bus} />;
}

/** @emoji 📑 Declarative side-panel body: tree nodes mount as a root {@link Tree} (not nested via {@link UiRenderer}). */
function DeclarativeSidePanelBody(props: { readonly tabId: string; readonly bodyKey: string }): React.ReactElement {
  const { runtime, activeModeId } = useApp();
  const generation = reactHostPort.useSyncExternalStore(
    (listener) => runtime.subscribe(listener),
    () => runtime.generation,
    () => 0,
  );
  usePuzzle3dPlaySnapshotPanelRefresh(props.bodyKey);
  const ctx: SidePanelBodyViewContext = {
    runtime,
    windowKindId: props.tabId,
    bodyKey: props.bodyKey,
    activeModeId: activeModeId ?? null,
    generation,
  };
  const bus = runtime.commandBus;
  const factory = getSidePanelBodyFactory(props.bodyKey);
  const node = factory?.(ctx) ?? { type: "text", value: `Missing declarative panel "${props.bodyKey}"` };
  if (node.type === "tree") {
    return <PlaygroundDeclarativeTree treeNode={node} commandBus={bus} />;
  }
  return <div className="text-destructive p-single text-xs">Side panel {props.bodyKey} must be type tree.</div>;
}

const declarativeSidePanelBodyComponents = new Map<string, React.FC>();

function getDeclarativeSidePanelBodyComponent(tabId: string, bodyKey: string): React.FC {
  const cacheKey = `${bodyKey}\0${tabId}`;
  let component = declarativeSidePanelBodyComponents.get(cacheKey);
  if (!component) {
    component = function ShellDeclarativeSidePanelBody() {
      return <DeclarativeSidePanelBody tabId={tabId} bodyKey={bodyKey} />;
    };
    declarativeSidePanelBodyComponents.set(cacheKey, component);
  }
  return component;
}
//#endregion 🔖DeclarativeHosts

//#region 🔖PlaygroundKeybindings
/** @emoji ⌨️ Dispatches declarative {@link Playground.keybindings} through the active command bus. */
function PlaygroundKeybindingHotkey(props: {
  readonly binding: PlaygroundKeybinding;
  readonly bus: CommandBus;
}): null {
  const { binding, bus } = props;
  useCommandHotkey(
    binding.key,
    () => {
      bus.dispatch(binding.controllerId, binding.command, binding.args);
    },
    { preventDefault: true },
    [binding.command, binding.controllerId, binding.args, bus],
  );
  return null;
}

function PlaygroundKeybindings(props: {
  readonly keybindings: readonly PlaygroundKeybinding[] | undefined;
  readonly bus: CommandBus;
}): React.ReactElement | null {
  const { keybindings, bus } = props;
  if (!keybindings?.length) {
    return null;
  }
  return (
    <>
      {keybindings.map((binding) => (
        <PlaygroundKeybindingHotkey key={`${binding.controllerId}:${binding.command}:${binding.key}`} binding={binding} bus={bus} />
      ))}
    </>
  );
}
//#endregion 🔖PlaygroundKeybindings

//#region 🔖PlaygroundView
export interface PlaygroundPanelVisibility {
  leftSidePanel: boolean;
  rightSidePanel: boolean;
}

export interface PlaygroundContextValue {
  runtime: Platform;
  activeAppId: string;
  activeApp: ResolvedAppState;
  activeModeId: string | null;
}

export const PlaygroundContext = reactHostPort.createContext<PlaygroundContextValue | undefined>(undefined);

/** @emoji 🪝 Returns the active {@link Platform} from the nearest {@link PlaygroundView}. */
export function useApp(): PlaygroundContextValue {
  const ctx = reactHostPort.useContext(PlaygroundContext);
  if (!ctx) throw new Error("useApp must be used within PlaygroundView");
  return ctx;
}

export interface PlaygroundViewProps {
  readonly runtime: Platform;
  readonly playgroundKeybindings?: readonly PlaygroundKeybinding[];
  readonly defaultAppId?: string;
  readonly mobile?: boolean;
  readonly mobileQuery?: string;
  readonly initialPanelVisibility?: PlaygroundPanelVisibility;
  readonly slotToolbar?: React.ReactNode;
  /** @emoji 🧪 Overrides controller-backed navbar fixture dropdown (React-held fixture state). */
  readonly slotNavbarCenter?: React.ReactNode;
  readonly extraFooterItems?: readonly FooterItem[];
  readonly augmentPanelTabs?: Partial<Record<"workbench" | "details" | "settings", readonly (SidePanelTabConfig | SidePanelTabDefinition)[]>>;
  readonly onActiveWindowChange?: (windowKindId: string) => void;
}

const playgroundExampleCatalogSnapshotCache = new WeakMap<object, PlaygroundExampleCatalog | null>();

function playgroundExampleCatalogSemanticallyEqual(a: PlaygroundExampleCatalog, b: PlaygroundExampleCatalog): boolean {
  if (a.activeExampleId !== b.activeExampleId || a.options.length !== b.options.length) {
    return false;
  }
  for (let i = 0; i < a.options.length; i += 1) {
    const left = a.options[i];
    const right = b.options[i];
    if (!left || !right || left.id !== right.id || left.label !== right.label) {
      return false;
    }
  }
  return true;
}

/** @emoji 🔔 Subscribes to controller snapshot or platform generation for navbar fixture catalog. */
function usePlaygroundExampleCatalog(runtime: Platform, controllerId: string | undefined): PlaygroundExampleCatalog | null {
  return reactHostPort.useSyncExternalStore(
    (listener) => {
      const app = runtime.getActiveApp();
      const controller = app?.controller.id === controllerId ? app.controller : undefined;
      const unsubscribeSnapshot =
        controller && "subscribeSnapshot" in controller && typeof controller.subscribeSnapshot === "function"
          ? (controller as import("@semio-tech/framework-playground-core").Controller & { subscribeSnapshot: (l: () => void) => () => void }).subscribeSnapshot(listener)
          : undefined;
      const unsubscribeRuntime = runtime.subscribe(listener);
      return () => {
        unsubscribeSnapshot?.();
        unsubscribeRuntime();
      };
    },
    () => {
      const controller = runtime.getActiveApp()?.controller;
      if (!controller) {
        return null;
      }
      const next = resolvePlaygroundExampleCatalog(controller);
      const cached = playgroundExampleCatalogSnapshotCache.get(controller);
      if (cached === next) {
        return cached;
      }
      if (cached && next && playgroundExampleCatalogSemanticallyEqual(cached, next)) {
        return cached;
      }
      playgroundExampleCatalogSnapshotCache.set(controller, next);
      return next;
    },
    () => null,
  );
}

function mergePanelTabs(base: SidePanelTabConfig[] | undefined, extension: readonly (SidePanelTabConfig | SidePanelTabDefinition)[] | undefined): SidePanelTabConfig[] {
  if (!extension?.length) return base ?? [];
  const merged = new Map<string, SidePanelTabConfig>();
  const add = (tab: SidePanelTabConfig | SidePanelTabDefinition): void => {
    const resolved = resolveSidePanelTabSource(tab);
    merged.set(resolved.id, resolved);
  };
  base?.forEach(add);
  extension.forEach(add);
  return [...merged.values()];
}

/** @emoji 🛝 Playground shell data plane: subscribes to runtime generation, not local panel chrome. */
function usePlaygroundViewShellData(runtime: Platform, options: Pick<PlaygroundViewProps, "augmentPanelTabs" | "extraFooterItems" | "slotToolbar" | "onActiveWindowChange">) {
  const { augmentPanelTabs, extraFooterItems, slotToolbar, onActiveWindowChange } = options;
  reactHostPort.useSyncExternalStore((listener) => runtime.subscribe(listener), () => runtime.generation, () => 0);

  const activeAppBase = runtime.getActiveApp();
  const activeModeId = activeAppBase?.getActiveModeId() ?? null;
  const shellDataGeneration = runtime.generation;
  const activeApp = reactHostPort.useMemo(
    () => (activeAppBase ? activeAppBase.resolve(activeModeId) : null),
    [activeAppBase, activeModeId, shellDataGeneration],
  );
  const bus = runtime.commandBus;

  const workbenchTabs = reactHostPort.useMemo(
    () =>
      activeApp
        ? mergePanelTabs(
            sideTabsToPlaygroundPanelTabs(
              activeApp.panelTabs.filter((tab) => tab.panel === "workbench"),
              runtime,
              bus,
            ),
            augmentPanelTabs?.workbench,
          )
        : [],
    [activeApp, augmentPanelTabs?.workbench, bus, shellDataGeneration],
  );
  const detailsTabs = reactHostPort.useMemo(
    () =>
      activeApp
        ? mergePanelTabs(
            sideTabsToPlaygroundPanelTabs(
              activeApp.panelTabs.filter((tab) => tab.panel === "details"),
              runtime,
              bus,
            ),
            augmentPanelTabs?.details,
          )
        : [],
    [activeApp, augmentPanelTabs?.details, bus, shellDataGeneration],
  );
  const settingsTabs = reactHostPort.useMemo(
    () =>
      activeApp
        ? mergePanelTabs(
            sideTabsToPlaygroundPanelTabs(
              activeApp.panelTabs.filter((tab) => tab.panel === "settings"),
              runtime,
              bus,
            ),
            augmentPanelTabs?.settings,
          )
        : [],
    [activeApp, augmentPanelTabs?.settings, bus, shellDataGeneration],
  );

  const mergedTools = reactHostPort.useMemo(() => (activeApp ? declareToolsToViewTools(activeApp.tools, bus) : undefined), [activeApp, bus, shellDataGeneration]);
  const hasToolbarTools = hasToolbarViewTools(mergedTools);

  const [activeWindowKindId, setActiveWindowKindId] = reactHostPort.useState<string | null>(() =>
    activeApp ? findDefaultActiveWindowKindId(activeApp.defaultLayout, activeApp.windowKinds) : null,
  );

  reactHostPort.useEffect(() => {
    if (!activeApp) return;
    setActiveWindowKindId((previous) => {
      if (previous && activeApp.windowKinds.some((wk) => wk.id === previous)) return previous;
      return findDefaultActiveWindowKindId(activeApp.defaultLayout, activeApp.windowKinds);
    });
  }, [activeApp]);

  const goldenWindowKinds = reactHostPort.useMemo(
    () => (activeApp ? windowKindsToGolden(activeApp.windowKinds, bus) : []),
    [activeApp, bus, shellDataGeneration],
  );

  const footerItems = reactHostPort.useMemo(
    () =>
      activeApp
        ? [...mergePlatformFooterChromeRows(runtime, activeApp), ...(extraFooterItems ?? [])].sort(
            (a, b) => (a.order ?? 0) - (b.order ?? 0),
          )
        : [],
    [activeApp, extraFooterItems, runtime, shellDataGeneration],
  );

  const workbenchIcon = reactHostPort.useMemo(
    () => (workbenchTabs[0]?.icon ? reactHostPort.createElement(workbenchTabs[0].icon, { size: 16 }) : <Icon icon="folder" size={16} />),
    [workbenchTabs],
  );
  const detailsIcon = reactHostPort.useMemo(
    () => (detailsTabs[0]?.icon ? reactHostPort.createElement(detailsTabs[0].icon, { size: 16 }) : <Icon icon="info" size={16} />),
    [detailsTabs],
  );

  const toolbarElement = reactHostPort.useMemo(
    () => slotToolbar ?? (hasToolbarTools && mergedTools ? <UIToolbar tools={mergedTools} /> : undefined),
    [hasToolbarTools, mergedTools, slotToolbar],
  );

  const playgroundContextValue = reactHostPort.useMemo<PlaygroundContextValue | null>(
    () =>
      activeApp
        ? {
            runtime,
            activeAppId: runtime.activeAppId,
            activeApp,
            activeModeId,
          }
        : null,
    [activeApp, activeModeId, runtime, runtime.activeAppId],
  );

  const onActiveWindowKindChange = reactHostPort.useCallback(
    (windowKindId: string) => {
      setActiveWindowKindId(windowKindId);
      onActiveWindowChange?.(windowKindId);
    },
    [onActiveWindowChange],
  );

  return {
    activeApp,
    activeAppBase,
    activeModeId,
    activeWindowKindId,
    bus,
    detailsIcon,
    detailsTabs,
    settingsTabs,
    footerItems,
    goldenWindowKinds,
    playgroundContextValue,
    toolbarElement,
    workbenchIcon,
    workbenchTabs,
    onActiveWindowKindChange,
  };
}

/** @emoji 🛝 Playground application shell: tree-only side panels, no JSON fallback details tab. */
export const PlaygroundView: React.FC<PlaygroundViewProps> = ({ runtime, playgroundKeybindings, defaultAppId, mobile, mobileQuery = "(max-width: 767px)", initialPanelVisibility, slotToolbar, slotNavbarCenter, extraFooterItems, augmentPanelTabs, onActiveWindowChange }) => {
  reactHostPort.useSyncExternalStore((listener) => runtime.subscribeChrome(listener), () => runtime.chromeGeneration, () => 0);

  reactHostPort.useEffect(() => {
    if (defaultAppId) runtime.setActiveAppId(defaultAppId);
  }, [defaultAppId, runtime]);

  const shell = usePlaygroundViewShellData(runtime, { augmentPanelTabs, extraFooterItems, slotToolbar, onActiveWindowChange });

  const [leftPanelSize, setLeftPanelSize] = reactHostPort.useState(280);
  const [rightPanelSize, setRightPanelSize] = reactHostPort.useState(300);
  const [panelVisibility, setPanelVisibilityState] = reactHostPort.useState<PlaygroundPanelVisibility>(() =>
    resolveInitialPanelVisibility(initialPanelVisibility ?? PRODUCT_SHELL_DEFAULT_PANEL_VISIBILITY, runtime),
  );
  const setPanelVisibility = reactHostPort.useCallback(
    (next: PlaygroundPanelVisibility | ((prev: PlaygroundPanelVisibility) => PlaygroundPanelVisibility)) => {
      setPanelVisibilityState((prev) => {
        const resolved = typeof next === "function" ? next(prev) : next;
        runtime.assignPanelVisibility(resolved);
        return resolved;
      });
    },
    [runtime],
  );
  const detectedMobile = useMediaQuery(mobileQuery);
  const resolvedMobile = mobile ?? detectedMobile ?? runtime.mobile;

  const [activeLeftPanelKind, setActiveLeftPanelKind] = reactHostPort.useState<"workbench" | "display">("workbench");
  const [activeRightPanelKind, setActiveRightPanelKind] = reactHostPort.useState<"details" | "settings">("details");
  const [uiCompact, setUiCompact] = reactHostPort.useState(readStoredUiChromeCompact);
  const [uiExpertise, setUiExpertise] = reactHostPort.useState(readStoredUiChromeExpertise);
  const [uiTheme, setUiTheme] = reactHostPort.useState(readStoredUiChromeTheme);
  const [computeWorkerCount, setComputeWorkerCount] = reactHostPort.useState(readStoredComputeWorkerCount);
  useElementsSurfaceChrome({ ...PLAYGROUND_SYSTEM_SURFACE_CHROME, theme: uiTheme, compact: uiCompact, expertise: uiExpertise });

  const namedLayoutStore = reactHostPort.useMemo(
    () => (shell.activeApp ? new NamedLayoutStore(shell.activeApp.id, createBrowserStoragePort()) : null),
    [shell.activeApp?.id],
  );
  const [displayHost, setDisplayHost] = reactHostPort.useState<DisplayHostApi | null>(null);
  const onDisplayHostReady = reactHostPort.useCallback((host: DisplayHostApi) => {
    setDisplayHost((previous) => (previous?.windowKinds === host.windowKinds ? previous : host));
  }, []);
  const displayTabs = reactHostPort.useMemo(
    () => (shell.activeApp && shell.activeApp.windowKinds.length > 0 ? createFrameworkDisplayPanelTabs(() => displayHost, shell.bus) : []),
    [displayHost, shell.activeApp, shell.bus],
  );
  const displayIcon = reactHostPort.useMemo(
    () =>
      displayTabs[0]?.icon
        ? reactHostPort.createElement(displayTabs[0].icon, { size: 16 })
        : "layout-grid",
    [displayTabs],
  );

  const hasModeNav = (shell.activeAppBase?.modes.length ?? 0) > 1;
  const setActiveModeId = reactHostPort.useCallback(
    (id: string) => {
      shell.activeAppBase?.setActiveModeId(id);
      runtime.notifyChrome();
    },
    [runtime, shell.activeAppBase],
  );
  const settingsHostApi = reactHostPort.useMemo<SettingsHostApi>(
    () => ({
      compact: uiCompact,
      setCompact: (compact: boolean) => {
        setUiCompact(compact);
        writeStoredUiChromeCompact(compact);
      },
      expertise: uiExpertise,
      setExpertise: (expertise: Expertise) => {
        setUiExpertise(expertise);
        writeStoredUiChromeExpertise(expertise);
      },
      computeWorkerCount,
      setComputeWorkerCount: (count: number) => {
        const clamped = Math.max(1, Math.floor(count));
        setComputeWorkerCount(clamped);
        writeStoredComputeWorkerCount(clamped);
      },
      computeThreadsAvailable: isCrossOriginIsolatedRuntime(),
      theme: uiTheme,
      setTheme: (theme: ElementsSurfaceTheme) => {
        setUiTheme(theme);
        writeStoredUiChromeTheme(theme);
      },
      appId: shell.activeApp?.id ?? "",
      appLabel: shell.activeApp?.label ?? "",
      appIconId: shell.activeApp?.iconId,
      modes: (shell.activeAppBase?.modes ?? []).map((mode) => ({ id: mode.id, label: mode.label, iconId: mode.iconId })),
      activeModeId: shell.activeModeId,
      setActiveModeId,
      hasModeNav,
    }),
    [computeWorkerCount, hasModeNav, setActiveModeId, shell.activeApp?.iconId, shell.activeApp?.id, shell.activeApp?.label, shell.activeAppBase?.modes, shell.activeModeId, uiCompact, uiExpertise, uiTheme],
  );
  const frameworkSettingsTabs = reactHostPort.useMemo(
    () => createFrameworkSettingsPanelTabs(() => settingsHostApi, () => displayHost, () => runtime, shell.bus),
    [displayHost, runtime, settingsHostApi, shell.bus],
  );
  const settingsTabs = reactHostPort.useMemo(
    () => mergePanelTabs(frameworkSettingsTabs, shell.settingsTabs),
    [frameworkSettingsTabs, shell.settingsTabs],
  );
  const settingsIcon = reactHostPort.useMemo(() => <Icon icon="settings-2" size={16} />, []);

  const leftSidePanelTabs = activeLeftPanelKind === "display" ? displayTabs : shell.workbenchTabs;
  const rightSidePanelTabs = activeRightPanelKind === "settings" ? settingsTabs : shell.detailsTabs;

  const toggleLastActiveLeftSidePanel = reactHostPort.useCallback(() => {
    if (leftSidePanelTabs.length === 0) return;
    setPanelVisibility((prev) => ({ ...prev, leftSidePanel: !prev.leftSidePanel }));
  }, [leftSidePanelTabs.length, setPanelVisibility]);

  const toggleLastActiveRightSidePanel = reactHostPort.useCallback(() => {
    if (rightSidePanelTabs.length === 0) return;
    setPanelVisibility((prev) => ({ ...prev, rightSidePanel: !prev.rightSidePanel }));
  }, [rightSidePanelTabs.length, setPanelVisibility]);

  const controllerId = shell.activeAppBase?.controller.id;
  const exampleCatalog = usePlaygroundExampleCatalog(runtime, controllerId);
  const navbarExampleSelect = reactHostPort.useMemo(() => {
    if (slotNavbarCenter !== undefined) return slotNavbarCenter;
    if (!exampleCatalog || !controllerId) return null;
    return (
      <NavbarExampleSelect
        id="playground.navbar.fixture"
        value={exampleCatalog.activeExampleId}
        options={exampleCatalog.options}
        onValueChange={(exampleId) => {
          shell.bus.dispatch(controllerId, "setActiveExample", { exampleId });
        }}
      />
    );
  }, [controllerId, exampleCatalog, shell.bus, slotNavbarCenter]);

  const playgroundPanelToggleItems = reactHostPort.useMemo<PanelToggleItem[]>(() => {
    const items: PanelToggleItem[] = [];
    if (displayTabs.length > 0) {
      items.push({
        id: "ui.panelToggle.display",
        icon: displayIcon,
        pressed: panelVisibility.leftSidePanel && activeLeftPanelKind === "display",
        onPressedChange: (pressed) => {
          if (pressed) setActiveLeftPanelKind("display");
          setPanelVisibility((p) => ({ ...p, leftSidePanel: pressed || (activeLeftPanelKind === "workbench" && p.leftSidePanel) }));
        },
      });
    }
    items.push({
      id: "ui.panelToggle.workbench",
      icon: shell.workbenchIcon,
      pressed: panelVisibility.leftSidePanel && activeLeftPanelKind === "workbench",
      onPressedChange: (pressed) => {
        if (pressed) setActiveLeftPanelKind("workbench");
        setPanelVisibility((p) => ({ ...p, leftSidePanel: pressed || (activeLeftPanelKind === "display" && p.leftSidePanel) }));
      },
    });
    items.push({
      id: "ui.panelToggle.details",
      icon: shell.detailsIcon,
      pressed: panelVisibility.rightSidePanel && activeRightPanelKind === "details",
      onPressedChange: (pressed) => {
        if (pressed) setActiveRightPanelKind("details");
        setPanelVisibility((p) => ({ ...p, rightSidePanel: pressed || (activeRightPanelKind === "settings" && p.rightSidePanel) }));
      },
    });
    items.push({
      id: "ui.panelToggle.settings",
      icon: settingsIcon,
      pressed: panelVisibility.rightSidePanel && activeRightPanelKind === "settings",
      onPressedChange: (pressed) => {
        if (pressed) setActiveRightPanelKind("settings");
        setPanelVisibility((p) => ({ ...p, rightSidePanel: pressed || (activeRightPanelKind === "details" && p.rightSidePanel) }));
      },
    });
    return items;
  }, [
    activeLeftPanelKind,
    activeRightPanelKind,
    displayIcon,
    displayTabs.length,
    panelVisibility.leftSidePanel,
    panelVisibility.rightSidePanel,
    setPanelVisibility,
    settingsIcon,
    shell.detailsIcon,
    shell.workbenchIcon,
  ]);

  const navbarItems = reactHostPort.useMemo<NavbarItem[]>(() => {
    if (!shell.activeApp) {
      return [];
    }
    const items: NavbarItem[] = [
      {
        key: "logoAndTitle",
        className: "min-w-0 shrink-0 flex items-center gap-single",
        content: (
          <div className="flex items-center gap-single">
            <SemioLogo className="shrink-0 size-workbench" />
            <span data-slot="app-name" className={cn("px-single", shellChromeTitleClassName)}>{shell.activeAppBase?.label}</span>
          </div>
        ),
      },
    ];
    if (navbarExampleSelect) {
      items.push({
        key: "fixture",
        content: navbarExampleSelect,
      });
      items.push(navbarFillItem());
    } else {
      items.push(navbarFillItem());
    }
    items.push({
      key: "panelToggles",
      content: <PanelToggleGroup items={playgroundPanelToggleItems} />,
    });
    items.push({
      key: "modes",
      content: (
        <ButtonGroup id="playground.navbar.modes">
          {shell.activeAppBase?.modes.map((mode) => {
            const isActive = shell.activeModeId === mode.id;
            return (
              <ButtonGroupItem
                key={mode.id}
                id={`playground.navbar.modes.${mode.id}`}
                className={cn(isActive && interactiveActiveFillClass)}
                data-state={isActive ? "on" : undefined}
                onClick={() => {
                  shell.activeAppBase?.setActiveModeId(mode.id);
                  runtime.notifyChrome();
                }}
                icon={mode.iconId || <span className="hidden" />}
                text={mode.label}
              />
            );
          })}
        </ButtonGroup>
      ),
    });
    return items;
  }, [navbarExampleSelect, playgroundPanelToggleItems, shell.activeApp, shell.activeAppBase, shell.activeModeId, runtime]);

  if (!shell.activeAppBase || !shell.activeApp || !shell.playgroundContextValue) return null;

  if (!namedLayoutStore) return null;

  return (
    <PlaygroundContext.Provider value={shell.playgroundContextValue}>
      <DisplayHostContext.Provider value={displayHost}>
      <SettingsHostContext.Provider value={settingsHostApi}>
      <PlaygroundKeybindings keybindings={playgroundKeybindings} bus={shell.bus} />
      <ProductShell
        platform={runtime}
        defaultAppId={defaultAppId}
        className="min-h-0 flex-1"
        mobile={resolvedMobile}
        mobileQuery={mobileQuery}
        navbarItems={navbarItems}
        footerItems={shell.footerItems}
        slotToolbar={shell.toolbarElement}
        leftSidePanelTabs={leftSidePanelTabs}
        rightSidePanelTabs={rightSidePanelTabs}
        panelVisibility={panelVisibility}
        leftPanelSize={leftPanelSize}
        onLeftPanelSizeChange={setLeftPanelSize}
        rightPanelSize={rightPanelSize}
        onRightPanelSizeChange={setRightPanelSize}
        goldenWindowKinds={shell.goldenWindowKinds}
        windowKindCatalog={shell.activeApp.windowKinds}
        namedLayouts={shell.activeApp.namedLayouts}
        namedLayoutStore={namedLayoutStore}
        commandBus={shell.bus}
        onDisplayHostReady={onDisplayHostReady}
        defaultLayout={shell.activeApp.defaultLayout}
        activeWindowKindId={shell.activeWindowKindId}
        onActiveWindowKindChange={shell.onActiveWindowKindChange}
        multiApp={false}
        activeModeId={shell.activeModeId}
        onActiveModeChange={(modeId) => {
          shell.activeAppBase!.setActiveModeId(modeId);
          runtime.notifyChrome();
        }}
        onToggleLastActiveLeftSidePanel={toggleLastActiveLeftSidePanel}
        onToggleLastActiveRightSidePanel={toggleLastActiveRightSidePanel}
      />
      </SettingsHostContext.Provider>
      </DisplayHostContext.Provider>
    </PlaygroundContext.Provider>
  );
};
//#endregion 🔖PlaygroundView

//#region 🔖PlaygroundShell
/** @emoji 🌓 Fixed surface chrome for every playground static site (system theme, desktop device). */
export const PLAYGROUND_SYSTEM_SURFACE_CHROME = {
  theme: "system" as const,
  device: "desktop" as const,
  expertise: Expertise.NORMAL,
};

/** @emoji 🛝 Locks document theme to system preference; wraps the play viewport in window-level chrome. */
export function PlaygroundShell({ children }: { readonly children: React.ReactNode }): React.ReactElement {
  useElementsSurfaceChrome(PLAYGROUND_SYSTEM_SURFACE_CHROME);
  return (
    <LevelProvider level="window">
      <div className={`flex h-screen min-h-0 w-screen flex-col ${getLevelBgClass("window")}`}>{children}</div>
    </LevelProvider>
  );
}

//#endregion 🔖PlaygroundShell

//#region 🔖Mount
type PlaygroundDomRoot = HTMLElement & { __playgroundRoot?: Root };

/** @emoji 🚀 Mounts an arbitrary React tree into `#root` (or `rootId`) inside {@link PlaygroundShell}. */
export function mountPlaygroundApp(element: React.ReactElement, rootId = "root"): void {
  if (typeof document === "undefined") return;
  bootstrapElementsSurfaceChromeDocument(PLAYGROUND_SYSTEM_SURFACE_CHROME.theme);
  const rootElement = document.getElementById(rootId) as PlaygroundDomRoot | null;
  if (!rootElement) throw new Error(`React root #${rootId} missing.`);
  rootElement.__playgroundRoot ??= createRoot(rootElement);
  rootElement.__playgroundRoot.render(<PlaygroundShell>{element}</PlaygroundShell>);
}

/** @emoji 🚀 Alias for {@link mountPlaygroundApp}. */
export const mountReactApp = mountPlaygroundApp;
//#endregion 🔖Mount

//#region 🔖Puzzle3dPlayHost
// #region 🔌Adapters
import {
    PUZZLE_3D_FILL_COUNT_MAX,
    PUZZLE_3D_PLAY_APP_ID,
    PUZZLE_3D_PLAY_BODY_KEY_JACK,
    PUZZLE_3D_PLAY_CONTROLLER_ID,
    PUZZLE_3D_PLAY_EXAMPLE_CONCRETE_FOREST_ID,
    PUZZLE_3D_PLAY_ICON_HIERARCHY,
    PUZZLE_3D_PLAY_ICON_INSPECTOR,
    PUZZLE_3D_PLAY_ICON_KINDS,
    PUZZLE_3D_PLAY_ICON_SETTINGS,
    PUZZLE_3D_PLAY_IDLE_SNAPSHOT,
    PUZZLE_3D_PLAY_SNAPSHOT_PANEL_BODY_KEYS,
    PUZZLE_3D_PLAY_STORE_ID,
    PUZZLE_3D_PLAY_VIEWPORT_SURFACE_ID,
    PUZZLE_3D_PLAY_SURFACE_ID_JACK,
    PUZZLE_3D_PLAY_WINDOW_KIND_JACK,
    Puzzle3dPlayShellController,
    clearPuzzle3dFillSession,
    getPuzzle3dFillSessionReadyEpoch,
    installPuzzle3dPlayBrushHost,
    parseKindCatalogs,
    parseKindCompatibility,
    preparePuzzle3dFillSession,
    puzzle3dBrushMeshRootForFill,
    puzzle3dFillBuildProgressRef,
    puzzle3dFillPendingCountRef,
    puzzle3dFillSessionRef,
    puzzle3dPlayFixtureJson,
    rerollPuzzle3dFillTail,
    subscribePuzzle3dFillDistributionInvalidated,
    subscribePuzzle3dFillSessionReady,
    subscribePuzzle3dFillTargetVolumesInvalidated,
    type Puzzle3dPlayHostBridge,
    type Puzzle3dPlaySnapshot
} from "@semio-tech/puzzle-3d-core";
import {
    ORBIT_CAMERA_VIEW_COMMAND,
    ObjectStateProvider,
    PlayCanvas,
    applyBrushPlacementToFixture,
    applyConnectToFixture,
    applyPaletteObjectDropToFixture,
    applyReferenceRelocateToFixture,
    applyRelocateToFixture,
    applyTargetVolumeRelocateToFixture,
    blockedVortexFullIdsFromAttractions,
    brushMeshUrlsForFillSession,
    buildPuzzle3dPlayEngagement,
    computeOrbitCameraViewState,
    getPuzzle3dBrushEngagementEpoch,
    isLoadableMeshUrl,
    orbitCameraDistance,
    orbitCameraProjectionForView,
    parseFixture,
    puzzle3dBrushEngagementSourceRef,
    puzzle3dFixturePaletteTreeDragController,
    requestPuzzle3dZoomToSelection,
    resolveOrbitCameraViewFromTemplateId,
    resolvePuzzle3dFixtureDrop,
    subscribePuzzle3dBrushEngagementSource,
    type CameraState,
    type Fixture,
    type Puzzle3dFixtureDropDetail,
    type Puzzle3dHoverPayload,
    type RelocatePayload
} from "@semio-tech/puzzle-3d-react";
import { buildWriterWindowBody } from "@semio-tech/framework-platform-core";
import { createWriterDocument } from "@semio-tech/writer-core";
import { WriterCanvas } from "@semio-tech/writer-react";
import { sceneHostPort } from "@semio-tech/ui-react";
// #endregion 🔌Adapters

function usePuzzle3dPlayController(): Puzzle3dPlayShellController | undefined {
  const { runtime } = useApp();
  return runtime.getActiveApp()?.controller as Puzzle3dPlayShellController | undefined;
}

function usePuzzle3dPlaySnapshot(): Puzzle3dPlaySnapshot {
  const ctrl = usePuzzle3dPlayController();
  return useControllerStore(ctrl, PUZZLE_3D_PLAY_STORE_ID) ?? PUZZLE_3D_PLAY_IDLE_SNAPSHOT;
}

function puzzle3dPlaySelectionSnapshotKey(ctrl: Puzzle3dPlayShellController | undefined, bodyKey: string): string {
  if (!ctrl || !PUZZLE_3D_PLAY_SNAPSHOT_PANEL_BODY_KEYS.has(bodyKey)) {
    return "";
  }
  const snap = ctrl.getSnapshot();
  const selection = snap.selection;
  const hover = snap.hoverFocus;
  return `${selection.objectIds.join("\0")}\0${selection.vortexIds.join("\0")}\0${selection.attractionIds.join("\0")}\0${(selection.referenceIds ?? []).join("\0")}\0${(selection.targetVolumeIds ?? []).join("\0")}\0${hover.kindHover?.domain ?? ""}\0${hover.kindHover?.kindId ?? ""}`;
}

/** @emoji 🔔 Re-renders hierarchy/inspector panels on puzzle 3D selection without a shell generation bump. */
function usePuzzle3dPlaySnapshotPanelRefresh(bodyKey: string): void {
  const ctrl = usePuzzle3dPlayController();
  reactHostPort.useSyncExternalStore(
    (listener) => {
      if (!ctrl || !PUZZLE_3D_PLAY_SNAPSHOT_PANEL_BODY_KEYS.has(bodyKey)) {
        return () => {};
      }
      return ctrl.subscribeSnapshot(listener);
    },
    () => puzzle3dPlaySelectionSnapshotKey(ctrl, bodyKey),
    () => "",
  );
}

/** @emoji 💬 Enforces CAD-style puzzle 3D play engagement (command input row required). */
export function enforcePuzzle3dPlayWindowEngagement(engagement: WindowEngagement | undefined): void {
  if (!engagement) return;
  enforcePlaygroundWindowEngagementInput(engagement, "Puzzle 3D play viewport");
}

/** @emoji 💬 Mirrors live {@link EngagementSpec} into {@link WindowEngagement} with bus-routed engagement commands. */
export function puzzle3dPlayEngagementMirror(engagement: EngagementSpec | null): WindowEngagement | undefined {
  if (!engagement) return undefined;
  const options = engagement.options?.map((option) => ({
    id: option.id,
    label: option.label,
    pressed: option.pressed,
    disabled: option.disabled,
    command: { controllerId: PUZZLE_3D_PLAY_CONTROLLER_ID, command: "engagementOption", args: { optionId: option.id } },
  }));
  const input = engagement.input
    ? {
        id: engagement.input.id,
        value: engagement.input.value,
        placeholder: engagement.input.placeholder,
        disabled: engagement.input.disabled,
        onChange: { controllerId: PUZZLE_3D_PLAY_CONTROLLER_ID, command: "engagementInput", args: {} },
        onSubmit: { controllerId: PUZZLE_3D_PLAY_CONTROLLER_ID, command: "engagementSubmit", args: {} },
        onRepeatLast: engagement.input.onRepeatLast
          ? { controllerId: PUZZLE_3D_PLAY_CONTROLLER_ID, command: "engagementRepeatLast", args: {} }
          : undefined,
        onAbort: { controllerId: PUZZLE_3D_PLAY_CONTROLLER_ID, command: "engagementAbort", args: {} },
      }
    : undefined;
  const status = engagement.status?.map((row) => ({ id: row.id, text: typeof row.content === "string" ? row.content : String(row.content) }));
  const possibleEngagements = engagement.possibleEngagements?.map((row) => ({
    id: row.id,
    label: row.label,
    detail: row.detail,
    command: { controllerId: PUZZLE_3D_PLAY_CONTROLLER_ID, command: "engagementPossibleSelect", args: { possibleId: row.id } },
  }));
  const control = engagementSpecControlMirror(engagement.control, PUZZLE_3D_PLAY_CONTROLLER_ID, {});
  const controls = engagement.controls?.map((row) => engagementSpecControlMirror(row, PUZZLE_3D_PLAY_CONTROLLER_ID, {})).filter((row): row is WindowEngagementControl => row !== undefined);
  return { sessionActive: engagement.sessionActive, options, input, control, controls, status, possibleEngagements };
}

function Puzzle3dPlayEngagementPublisher(props: {
  readonly ctrl: Puzzle3dPlayShellController | undefined;
  readonly snap: Puzzle3dPlaySnapshot;
  readonly bus: CommandBus;
}): null {
  const { ctrl, snap, bus } = props;
  const kindCatalogs = reactHostPort.useMemo(() => parseKindCatalogs(snap.fixture.meta), [snap.fixture.meta]);
  const [cmdLine, setCmdLine] = reactHostPort.useState("");
  const [fillCount, setFillCount] = reactHostPort.useState(0);
  const fillSessionReadyEpoch = reactHostPort.useSyncExternalStore(
    subscribePuzzle3dFillSessionReady,
    getPuzzle3dFillSessionReadyEpoch,
    getPuzzle3dFillSessionReadyEpoch,
  );
  const engagementSpecRef = reactHostPort.useRef<EngagementSpec | null>(null);
  const brushEngagementEpoch = reactHostPort.useSyncExternalStore(subscribePuzzle3dBrushEngagementSource, getPuzzle3dBrushEngagementEpoch, getPuzzle3dBrushEngagementEpoch);
  const brushSource = puzzle3dBrushEngagementSourceRef.current;
  const selectionCount =
    snap.selection.objectIds.length +
    snap.selection.vortexIds.length +
    snap.selection.attractionIds.length +
    snap.selection.targetVolumeIds.length;
  const rememberEngagementRepeat = reactHostPort.useCallback(
    (key: string) => {
      bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "rememberEngagementRepeat", { key });
    },
    [bus],
  );
  const onSelectTool = reactHostPort.useCallback(() => {
    if (snap.activeTool === "fill") {
      const base = clearPuzzle3dFillSession();
      if (base && ctrl) {
        ctrl.patchFixture(() => structuredClone(base));
      }
      setFillCount(0);
    }
    bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "setActiveTool", { tool: "select" });
  }, [bus, ctrl, snap.activeTool]);
  const onBrushTool = reactHostPort.useCallback(() => {
    if (snap.activeTool === "fill") {
      const base = clearPuzzle3dFillSession();
      if (base && ctrl) {
        ctrl.patchFixture(() => structuredClone(base));
      }
      setFillCount(0);
    }
    bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "setActiveTool", { tool: "brush" });
  }, [bus, ctrl, snap.activeTool]);
  const onFillTool = reactHostPort.useCallback(() => {
    puzzle3dFillPendingCountRef.current = 0;
    setFillCount(0);
    bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "setActiveTool", { tool: "fill" });
  }, [bus]);
  const onFillCount = reactHostPort.useCallback(
    (count: number) => {
      const progress = puzzle3dFillBuildProgressRef.current;
      const maxAvailable = progress.done ? PUZZLE_3D_FILL_COUNT_MAX : progress.count;
      const prev = fillCount;
      const n = Math.max(0, Math.min(PUZZLE_3D_FILL_COUNT_MAX, Math.round(count), maxAvailable));
      puzzle3dFillPendingCountRef.current = n;
      setFillCount(n);
      bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "setFillCount", { count: n });
      if (n < prev) {
        const catalogs = parseKindCatalogs(snap.fixture.meta);
        const compatibility = parseKindCompatibility(snap.fixture.meta);
        rerollPuzzle3dFillTail(n, catalogs, compatibility, snap.brushPlacementOverlapBudget);
      }
    },
    [bus, fillCount, snap.brushPlacementOverlapBudget, snap.fixture.meta],
  );
  const fillAutoStartedRef = reactHostPort.useRef(false);
  reactHostPort.useEffect(() => {
    if (snap.activeTool !== "fill") {
      fillAutoStartedRef.current = false;
      return;
    }
    if (fillAutoStartedRef.current) {
      return;
    }
    const sequenceLength = puzzle3dFillSessionRef.current.sequence.length;
    if (sequenceLength === 0) {
      return;
    }
    fillAutoStartedRef.current = true;
    const pending = puzzle3dFillPendingCountRef.current;
    const nextCount = pending > 0 ? Math.min(pending, sequenceLength) : 1;
    puzzle3dFillPendingCountRef.current = nextCount;
    setFillCount(nextCount);
    bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "setFillCount", { count: nextCount });
  }, [bus, fillSessionReadyEpoch, snap.activeTool]);
  reactHostPort.useEffect(() => {
    if (snap.activeTool !== "fill") {
      return;
    }
    const progress = puzzle3dFillBuildProgressRef.current;
    if (progress.done) {
      return;
    }
    const maxAvailable = progress.count;
    if (fillCount <= maxAvailable) {
      return;
    }
    const capped = maxAvailable;
    puzzle3dFillPendingCountRef.current = capped;
    setFillCount(capped);
    bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "setFillCount", { count: capped });
  }, [bus, fillCount, fillSessionReadyEpoch, snap.activeTool]);
  const fillBuildProgress = puzzle3dFillBuildProgressRef.current;
  const onRepeatLastEngagement = reactHostPort.useCallback(() => {
    bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "engagementRepeatLast", {});
  }, [bus]);
  const onEngagementAbort = reactHostPort.useCallback(() => {
    setCmdLine("");
    if (snap.activeTool === "brush" || snap.activeTool === "fill") {
      onSelectTool();
    }
  }, [onSelectTool, snap.activeTool]);
  const onDeleteSelectedTargetVolume = reactHostPort.useCallback(() => {
    bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "deleteSelectedTargetVolume", {});
  }, [bus]);
  const onToggleFillEditTargetVolumes = reactHostPort.useCallback(() => {
    bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "setFillEditTargetVolumes", {});
  }, [bus]);
  const onVoxelBrushDimension = reactHostPort.useCallback(
    (axis: 0 | 1 | 2, value: number) => {
      bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "setVoxelBrushDimension", { axis, value });
    },
    [bus],
  );
  const onZoomToSelection = reactHostPort.useCallback(() => {
    requestPuzzle3dZoomToSelection(snap.selection);
  }, [snap.selection]);
  const onCmdLineSubmit = reactHostPort.useCallback(
    (value: string) => {
      const token = normalizeEngagementCommandText(value.trim());
      if (engagementCommandTokenEquals(token, "brush")) {
        onBrushTool();
        setCmdLine("");
        return;
      }
      if (engagementCommandTokenEquals(token, "fill")) {
        onFillTool();
        setCmdLine("");
        return;
      }
      if (engagementCommandTokenEquals(token, "select")) {
        onSelectTool();
        setCmdLine("");
        return;
      }
      if (engagementCommandTokenEquals(token, "zoom")) {
        rememberEngagementRepeat(PUZZLE_3D_ENGAGEMENT_ZOOM_ID);
        onZoomToSelection();
        setCmdLine("");
        return;
      }
      if (snap.activeTool === "brush") {
        const raw = value.trim();
        const idx = brushSource.candidates.findIndex(
          (candidate) => candidate.objectKindId === raw || engagementCommandTokenEquals(candidate.objectKindId, token),
        );
        if (idx >= 0) {
          const candidate = brushSource.candidates[idx]!;
          rememberEngagementRepeat(`puzzle3d.brush.${candidate.objectKindId}.${candidate.sourceVortexIndex}`);
          brushSource.pickCandidate(idx);
        }
      }
      setCmdLine("");
    },
    [brushSource, onBrushTool, onFillTool, onSelectTool, onZoomToSelection, rememberEngagementRepeat, snap.activeTool],
  );
  const spec = reactHostPort.useMemo(
    () =>
      buildPuzzle3dPlayEngagement({
        activeTool: snap.activeTool,
        cmdLine,
        fillCount,
        fillBuildProgress,
        fillEditTargetVolumes: snap.fillEditTargetVolumes,
        voxelBrushDimensions: snap.voxelBrushDimensions,
        selectedTargetVolumeCount: snap.selection.targetVolumeIds.length,
        selectionCount,
        onCmdLineChange: setCmdLine,
        onCmdLineSubmit,
        onRepeatLast: onRepeatLastEngagement,
        onAbort: onEngagementAbort,
        onSelectTool,
        onBrushTool,
        onFillTool,
        onFillCount,
        onToggleFillEditTargetVolumes,
        onDeleteSelectedTargetVolume,
        onVoxelBrushDimension,
        onCycleBrushCandidate: () => brushSource.cycleCandidate(),
        onPickBrushCandidate: (index) => {
          const candidate = brushSource.candidates[index];
          if (candidate) {
            rememberEngagementRepeat(`puzzle3d.brush.${candidate.objectKindId}.${candidate.sourceVortexIndex}`);
          }
          brushSource.pickCandidate(index);
        },
        onZoomToSelection,
        brushCandidates: brushSource.candidates,
        brushTargetActive: brushSource.targetActive,
        brushPlacementProbePending: brushSource.placementProbePending,
        kindCatalogs,
        sceneFixture: snap.fixture,
      }),
    [brushEngagementEpoch, brushSource, cmdLine, fillBuildProgress, fillCount, fillSessionReadyEpoch, kindCatalogs, onBrushTool, onCmdLineSubmit, onDeleteSelectedTargetVolume, onEngagementAbort, onFillCount, onFillTool, onRepeatLastEngagement, onSelectTool, onToggleFillEditTargetVolumes, onVoxelBrushDimension, onZoomToSelection, rememberEngagementRepeat, selectionCount, snap.activeTool, snap.fillEditTargetVolumes, snap.fixture, snap.selection.targetVolumeIds.length, snap.voxelBrushDimensions],
  );
  engagementSpecRef.current = spec;
  reactHostPort.useEffect(() => {
    const mirrored = puzzle3dPlayEngagementMirror(spec);
    enforcePuzzle3dPlayWindowEngagement(mirrored);
    ctrl?.setWindowEngagement(mirrored);
  }, [ctrl, spec]);
  reactHostPort.useLayoutEffect(() => {
    if (!ctrl) {
      return;
    }
    const bridge: Puzzle3dPlayHostBridge = {
      runHostCommand: (command, args) => {
        switch (command) {
          case "engagementOption": {
            const optionId = (args as { optionId?: string })?.optionId;
            engagementSpecRef.current?.options?.find((row) => row.id === optionId)?.onPress?.();
            break;
          }
          case "engagementInput": {
            const value = (args as { value?: string })?.value ?? "";
            engagementSpecRef.current?.input?.onChange?.(value);
            break;
          }
          case "engagementSubmit": {
            const value = (args as { value?: string })?.value ?? engagementSpecRef.current?.input?.value ?? "";
            engagementSpecRef.current?.input?.onSubmit?.(value);
            break;
          }
          case "engagementRepeatLast":
            engagementSpecRef.current?.input?.onRepeatLast?.();
            break;
          case "engagementAbort":
            engagementSpecRef.current?.input?.onAbort?.();
            break;
          case "engagementPossibleSelect": {
            const possibleId = (args as { possibleId?: string })?.possibleId;
            if (!possibleId) {
              break;
            }
            engagementSpecRef.current?.possibleEngagements?.find((row) => row.id === possibleId)?.onSelect?.();
            break;
          }
          case "engagementControlChange": {
            const value = (args as { value?: number; controlId?: string })?.value;
            const controlId = (args as { controlId?: string })?.controlId;
            const spec = engagementSpecRef.current;
            const control =
              controlId && spec?.controls?.length
                ? spec.controls.find((row) => row.id === controlId) ?? spec.control
                : spec?.control;
            if (value === undefined || !control || control.kind === "ring") break;
            control.onChange?.(value);
            break;
          }
          case "engagementControlCommit": {
            const value = (args as { value?: number; controlId?: string })?.value;
            const controlId = (args as { controlId?: string })?.controlId;
            const spec = engagementSpecRef.current;
            const control =
              controlId && spec?.controls?.length
                ? spec.controls.find((row) => row.id === controlId) ?? spec.control
                : spec?.control;
            if (value === undefined || !control || control.kind === "ring") break;
            control.onCommit?.(value);
            break;
          }
          case "engagementControlSelect": {
            const id = (args as { id?: string })?.id;
            const control = engagementSpecRef.current?.control;
            if (!id || !control || control.kind !== "ring") break;
            control.onSelect?.(id);
            break;
          }
          default:
            break;
        }
      },
    };
    ctrl.setHostBridge(bridge);
    return () => ctrl.setHostBridge(null);
  }, [ctrl]);
  return null;
}

/** @emoji 📷 Aligns shell viewport camera projection with a display-tree template on first paint. */
function puzzle3dPlayViewportCamera(base: CameraState, templateId?: string): CameraState {
  const view = templateId ? resolveOrbitCameraViewFromTemplateId(templateId) : null;
  if (!view) {
    return base;
  }
  const expectedProjection = orbitCameraProjectionForView(view);
  if ((base.projection ?? "perspective") === expectedProjection) {
    return base;
  }
  return computeOrbitCameraViewState(view, {
    target: base.target,
    distance: orbitCameraDistance({ ...base, projection: base.projection ?? "perspective" }),
    zoom: base.zoom,
  });
}

const Puzzle3dPlayViewportHost = reactHostPort.memo(function Puzzle3dPlayViewportHost({ node }: { readonly node: UiPuzzle3dHostSurfaceNode }): React.ReactElement {
  const { runtime } = useApp();
  const bus = runtime.commandBus;
  const ctrl = usePuzzle3dPlayController();
  const snap = usePuzzle3dPlaySnapshot();
  const shellInstance = useShellWindowInstance();
  const viewportCamera = reactHostPort.useMemo(() => {
    const base = ctrl?.cameraForInstance(shellInstance?.instanceId) ?? snap.fixture?.camera;
    if (!base) {
      return { position: [420, -420, 320] as const, target: [0, 0, 40] as const, zoom: 1 };
    }
    return puzzle3dPlayViewportCamera(base, shellInstance?.templateId);
  }, [ctrl, shellInstance?.instanceId, shellInstance?.templateId, snap.cameraSeedEpoch, snap.fixture?.camera]);
  reactHostPort.useLayoutEffect(() => {
    if (!ctrl || !shellInstance?.instanceId || !shellInstance.templateId) {
      return;
    }
    const view = resolveOrbitCameraViewFromTemplateId(shellInstance.templateId);
    if (!view) {
      return;
    }
    const current = ctrl.cameraForInstance(shellInstance.instanceId);
    if ((current.projection ?? "perspective") === orbitCameraProjectionForView(view)) {
      return;
    }
    ctrl.run(ORBIT_CAMERA_VIEW_COMMAND, { view, instanceId: shellInstance.instanceId });
  }, [ctrl, shellInstance?.instanceId, shellInstance?.templateId]);
  const cameraSeedKey = shellInstance ? `${shellInstance.instanceId}:${snap.cameraSeedEpoch}` : snap.cameraSeedEpoch;
  const engagementPublisher =
    ctrl && node.controllerId === PUZZLE_3D_PLAY_CONTROLLER_ID ? (
      <Puzzle3dPlayEngagementPublisher ctrl={ctrl} snap={snap} bus={bus} />
    ) : null;
  if (node.controllerId !== PUZZLE_3D_PLAY_CONTROLLER_ID) {
    return (
      <>
        {engagementPublisher}
        <div className="p-2 text-xs text-muted-foreground">Invalid puzzle 3D viewport binding</div>
      </>
    );
  }
  if (!snap.fixture) {
    return (
      <>
        {engagementPublisher}
        <div className="p-4 text-destructive">Invalid puzzle 3D fixture</div>
      </>
    );
  }
  const kindCompatibility = reactHostPort.useMemo(() => parseKindCompatibility(snap.fixture.meta), [snap.fixture]);
  const kindCatalogs = reactHostPort.useMemo(() => parseKindCatalogs(snap.fixture.meta), [snap.fixture]);
  reactHostPort.useEffect(() => {
    installPuzzle3dPlayBrushHost(snap.fixture.meta as Record<string, unknown> | undefined);
  }, [snap.fixture.meta]);
  const blockedVortexFullIds = reactHostPort.useMemo(() => blockedVortexFullIdsFromAttractions(snap.fixture.attractions), [snap.fixture]);
  const patchFixture = reactHostPort.useCallback(
    (updater: (prev: Fixture) => Fixture) => {
      ctrl?.patchFixture(updater);
    },
    [ctrl],
  );
  const onRelocatePersist = reactHostPort.useCallback(
    (payload: RelocatePayload, attractingByObjectId: ReadonlyMap<string, readonly string[]>) => {
      ctrl?.patchRelocate(payload, attractingByObjectId);
    },
    [ctrl],
  );
  const onReferenceRelocatePersist = reactHostPort.useCallback(
    (payload: import("@semio-tech/infinite-world-r3f").WorldReferenceRelocatePayload) => {
      ctrl?.patchReferenceRelocate(payload);
    },
    [ctrl],
  );
  const onTargetVolumeRelocatePersist = reactHostPort.useCallback(
    (payload: import("@semio-tech/infinite-world-r3f").WorldVolumeRelocatePayload) => {
      ctrl?.patchTargetVolumeRelocate(payload);
    },
    [ctrl],
  );
  const onVoxelBrushPaintPersist = reactHostPort.useCallback(
    (cad: import("../react/index.tsx").Vec3, scale: import("../react/index.tsx").Vec3) => {
      ctrl?.run("paintVoxel", { cad, scale });
    },
    [ctrl],
  );
  const proximityRelocateEnabled = snap.fixture.attractions.length > 0;
  const onCanvasHover = reactHostPort.useCallback(
    (payload: Puzzle3dHoverPayload) => {
      console.log("[DEBUG] puzzle3d hover", payload.kindHover?.domain, payload.kindHover?.kindId, payload.hoverTarget?.kind);
      ctrl?.setHoverFocus(payload);
    },
    [ctrl],
  );
  const handleFixtureDrop = reactHostPort.useCallback(
    (detail: Puzzle3dFixtureDropDetail) => {
      const result = resolvePuzzle3dFixtureDrop(detail, kindCatalogs, snap.fixture);
      if (result.kind === "palette-object") {
        patchFixture((fixture) => applyPaletteObjectDropToFixture(fixture, result.object));
        bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "setSelection", {
          selection: { objectIds: [result.object.id], vortexIds: [], attractionIds: [] },
        });
        return;
      }
      if (result.kind === "replace-fixture") {
        ctrl?.patchFixture(() => result.fixture);
      }
    },
    [bus, ctrl, kindCatalogs, patchFixture, snap.fixture],
  );
  const fillBaseCaptureRef = reactHostPort.useRef<Fixture | null>(null);
  const prevActiveToolRef = reactHostPort.useRef(snap.activeTool);
  reactHostPort.useLayoutEffect(() => {
    const prev = prevActiveToolRef.current;
    prevActiveToolRef.current = snap.activeTool;
    if (snap.activeTool === "fill" && prev !== "fill") {
      fillBaseCaptureRef.current = structuredClone(snap.fixture);
    }
    if (snap.activeTool !== "fill") {
      fillBaseCaptureRef.current = null;
    }
  }, [snap.activeTool, snap.fixture]);
  const fillPrepareTimerRef = reactHostPort.useRef<ReturnType<typeof setTimeout> | null>(null);
  const fillSessionPreparedRef = reactHostPort.useRef(false);
  const [fillDistributionEpoch, setFillDistributionEpoch] = reactHostPort.useState(0);
  const [fillTargetVolumesEpoch, setFillTargetVolumesEpoch] = reactHostPort.useState(0);
  reactHostPort.useEffect(() => {
    if (snap.activeTool !== "fill") {
      fillSessionPreparedRef.current = false;
    }
  }, [snap.activeTool]);
  reactHostPort.useEffect(
    () =>
      subscribePuzzle3dFillDistributionInvalidated(() => {
        fillSessionPreparedRef.current = false;
        setFillDistributionEpoch((epoch) => epoch + 1);
      }),
    [],
  );
  reactHostPort.useEffect(
    () =>
      subscribePuzzle3dFillTargetVolumesInvalidated(() => {
        fillSessionPreparedRef.current = false;
        setFillTargetVolumesEpoch((epoch) => epoch + 1);
      }),
    [],
  );
  const fillToleranceRef = reactHostPort.useRef(snap.brushPlacementOverlapBudget);
  reactHostPort.useEffect(() => {
    if (fillToleranceRef.current === snap.brushPlacementOverlapBudget) {
      return;
    }
    fillToleranceRef.current = snap.brushPlacementOverlapBudget;
    if (snap.activeTool !== "fill") {
      return;
    }
    fillSessionPreparedRef.current = false;
  }, [snap.activeTool, snap.brushPlacementOverlapBudget]);
  const onFillMeshesReady = reactHostPort.useCallback(() => {
    if (fillPrepareTimerRef.current !== null) {
      clearTimeout(fillPrepareTimerRef.current);
    }
    fillPrepareTimerRef.current = setTimeout(() => {
      fillPrepareTimerRef.current = null;
      const base = fillBaseCaptureRef.current;
      if (!base) {
        return;
      }
      if (!fillSessionPreparedRef.current) {
        preparePuzzle3dFillSession(base, kindCatalogs, kindCompatibility, snap.brushPlacementOverlapBudget, base.targetVolumes ?? []);
        fillSessionPreparedRef.current = true;
      }
    }, 0);
  }, [bus, kindCatalogs, kindCompatibility, snap.brushPlacementOverlapBudget]);
  reactHostPort.useEffect(() => {
    if (snap.activeTool !== "fill" || !fillBaseCaptureRef.current) {
      return;
    }
    if (fillSessionPreparedRef.current) {
      return;
    }
    onFillMeshesReady();
  }, [fillDistributionEpoch, fillTargetVolumesEpoch, onFillMeshesReady, snap.activeTool, snap.brushPlacementOverlapBudget]);
  reactHostPort.useEffect(
    () => () => {
      if (fillPrepareTimerRef.current !== null) {
        clearTimeout(fillPrepareTimerRef.current);
      }
    },
    [],
  );
  return (
    <>
      {engagementPublisher}
      <div className="absolute inset-0 min-h-0 min-w-0">
      <ObjectStateProvider
        fixture={snap.fixture}
        fixtureRevision={snap.fixtureRevision}
        onConnect={(payload) => {
          patchFixture((fixture) => applyConnectToFixture(fixture, payload));
          bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "noteConnect");
        }}
        onRelocate={onRelocatePersist}
      >
        <PlayCanvas
          fixture={snap.fixture}
          camera={viewportCamera}
          cameraSeedKey={cameraSeedKey}
          proximityRelocateEnabled={proximityRelocateEnabled}
          kindCatalogs={kindCatalogs}
          kindCompatibility={kindCompatibility}
          blockedVortexFullIds={blockedVortexFullIds}
          lodTag={snap.lodTag}
          lodProps={snap.lodProps}
          gumballConfig={snap.gumballConfig}
          selection={snap.selection}
          selectedId={snap.selectedId}
          selectedLabel={snap.selectedLabel}
          selectionMode={snap.selectionMode}
          selectionMethod={snap.selectionMethod}
          marqueeSelectableKinds={
            snap.fillEditTargetVolumes
              ? { object: false, vortex: false, attraction: false }
              : snap.selectableKinds
          }
          proximityRadius={snap.proximityRadius}
          chunkSize={snap.chunkSize}
          gridFactor={snap.gridFactor}
          showLodGrid={snap.showLodGrid}
          gridSnapEnabled={snap.gridSnapEnabled}
          hoverTarget={snap.hoverFocus.hoverTarget}
          kindHover={snap.hoverFocus.kindHover}
          onHover={onCanvasHover}
          setSelectedId={(id) => bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "setSelectedId", { id })}
          onSelect={(selection) => bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "noteSelection", selection)}
          onReferenceRelocate={onReferenceRelocatePersist}
          onTargetVolumeRelocate={onTargetVolumeRelocatePersist}
          onVoxelBrushPaint={onVoxelBrushPaintPersist}
          fillEditTargetVolumes={snap.fillEditTargetVolumes}
          voxelBrushDimensions={snap.voxelBrushDimensions}
          onToggleSelectionHidden={(value) => bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "setSelectionFlag", { flag: "hidden", value })}
          onToggleSelectionLocked={(value) => bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "setSelectionFlag", { flag: "locked", value })}
          onDeleteSelection={() => bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "deleteSelection")}
          onDuplicateSelection={() => bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "duplicateSelection")}
          onSelectSameKind={() => bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "selectSameKind")}
          onIndirectConnect={() => bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "noteIndirect")}
          onProximityConnect={() => bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "noteProximity")}
          onLodChange={(lod) => bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "setEffectiveLod", { lod })}
          onCamera={(camera) => ctrl?.setCamera(camera, shellInstance?.instanceId)}
          onAttractionCompatibleObjects={() => bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "noteCompatibleObjects")}
          onAttractionTargetRing={() => bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "noteTargetRing")}
          brushActive={snap.activeTool === "brush"}
          fillActive={snap.activeTool === "fill"}
          onFillMeshesReady={onFillMeshesReady}
          onBrushPlace={(payload) => bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "addBrushObject", payload)}
          brushPlacementOverlapBudget={snap.brushPlacementOverlapBudget}
          fixtureDragDrop
          onFixtureDrop={handleFixtureDrop}
        />
      </ObjectStateProvider>
      <div data-puzzle3d-play-probe className="pointer-events-none absolute left-0 top-0 select-none opacity-0" aria-hidden>
        <span data-e2e-selected>{snap.selectedLabel ?? "none"}</span>
        <span data-e2e-scene-lod>{snap.lodTag}</span>
        <span data-e2e-proximity-count>{snap.proximityCount}</span>
        <span data-e2e-connect-count>{snap.connectCount}</span>
        <span data-e2e-indirect-count>{snap.indirectCount}</span>
      </div>
    </div>
    </>
  );
}, (prev, next) => prev.node.surfaceId === next.node.surfaceId && prev.node.controllerId === next.node.controllerId);

function Puzzle3dPlayJackSurfaceHost({ node: _node }: { readonly node: import("@semio-tech/framework-platform-core").UiWriterHostSurfaceNode }): React.ReactElement {
  const ctrl = usePuzzle3dPlayController();
  void ctrl?.getHoverEpoch();
  void ctrl?.getSelectEpoch();
  const document = ctrl?.getWriterDocumentJack() ?? createWriterDocument({ id: "puzzle-3d-jack", languageId: "jack", text: "" });
  const onHoverChange = reactHostPort.useCallback((offset: number | null) => {
    ctrl?.run("setJackHover", { offset });
  }, [ctrl]);
  const onSelectionChange = reactHostPort.useCallback((range: { start: number; end: number }) => {
    ctrl?.run("setJackSelect", range);
  }, [ctrl]);
  return (
    <WriterCanvas
      document={document}
      className="h-full"
      onHoverChange={onHoverChange}
      onSelectionChange={onSelectionChange}
      externalHoverOccurrences={ctrl?.getJackHoverOccurrences()}
      externalHoverOccurrencesSignal={ctrl?.getHoverEpoch()}
      externalSelectionOccurrences={ctrl?.getJackSelectOccurrences()}
      externalSelectionOccurrencesSignal={ctrl?.getSelectEpoch()}
    />
  );
}

let puzzle3dPlayChromeRegistered = false;

/** @emoji 🧊 Registers puzzle 3D play surface host, tab icons, and mesh preload. */
export function registerPuzzle3dPlaySurfaceHosts(): void {
  if (puzzle3dPlayChromeRegistered) return;
  puzzle3dPlayChromeRegistered = true;
  registerUiPuzzle3dSurfaceHost(PUZZLE_3D_PLAY_VIEWPORT_SURFACE_ID, Puzzle3dPlayViewportHost);
  registerUiWriterSurfaceHost(PUZZLE_3D_PLAY_SURFACE_ID_JACK, Puzzle3dPlayJackSurfaceHost);
  registerWindowBody(PUZZLE_3D_PLAY_BODY_KEY_JACK, () =>
    buildWriterWindowBody(PUZZLE_3D_PLAY_SURFACE_ID_JACK, PUZZLE_3D_PLAY_CONTROLLER_ID, PUZZLE_3D_PLAY_WINDOW_KIND_JACK));
  registerTabIcon(PUZZLE_3D_PLAY_ICON_INSPECTOR, "clipboard-list");
  registerTabIcon(PUZZLE_3D_PLAY_ICON_KINDS, "tags");
  registerTabIcon(PUZZLE_3D_PLAY_ICON_HIERARCHY, "list-tree");
  registerTabIcon(PUZZLE_3D_PLAY_ICON_SETTINGS, "settings");
  const fixture = parseFixture(puzzle3dPlayFixtureJson(playgroundResolvedExampleId(PUZZLE_3D_PLAY_EXAMPLE_CONCRETE_FOREST_ID)) as unknown);
  if (fixture) {
    const catalogs = parseKindCatalogs(fixture.meta as Record<string, unknown> | undefined);
    const compatibility = parseKindCompatibility(fixture.meta as Record<string, unknown> | undefined);
    for (const url of brushMeshUrlsForFillSession(fixture, catalogs, compatibility)) {
      if (isLoadableMeshUrl(url)) {
        sceneHostPort.drei.useGLTF.preload(url);
      }
    }
  }
}

/** @emoji 🚀 Mounts puzzle 3d play via standard {@link PlaygroundView} (bodies registered in {@link Playground3d}). */
export function mountPuzzle3dPlayChrome(playground: Playground, rootId = "root"): void {
  mountPlaygroundApp(
    <PlaygroundView runtime={playground.runtime} defaultAppId={PUZZLE_3D_PLAY_APP_ID} playgroundKeybindings={playground.keybindings} />,
    rootId,
  );
}

const puzzle3dPlayChromeBoot: PlaygroundChromeBoot = {
  registerHosts: registerPuzzle3dPlaySurfaceHosts,
  mount: mountPuzzle3dPlayChrome,
};

/** @emoji 🛝 Puzzle 3D play entry: register hosts, bodies, mount chrome (from `puzzle/3d/play/index.ts`). */
export function bootPuzzle3dPlay(playground: Playground, rootId = "root"): void {
  bootPlayground(playground, puzzle3dPlayChromeBoot, rootId);
}
//#endregion 🔖Puzzle3dPlayHost

//#region 🔖Puzzle5dPlayHost
// #region 🔌Adapters
import {
    PUZZLE_5D_PLAY_2D_BODY_KEY,
    PUZZLE_5D_PLAY_2D_SURFACE_ID,
    PUZZLE_5D_PLAY_2D_WINDOW_ID,
    PUZZLE_5D_PLAY_3D_BODY_KEY,
    PUZZLE_5D_PLAY_3D_SURFACE_ID,
    PUZZLE_5D_PLAY_JACK_BODY_KEY,
    PUZZLE_5D_PLAY_JACK_SURFACE_ID,
    PUZZLE_5D_PLAY_JACK_WINDOW_ID,
    PUZZLE_5D_PLAY_APP_ID,
    PUZZLE_5D_PLAY_CONTROLLER_ID,
    PUZZLE_5D_PLAY_HIERARCHY_TAB_ID,
    PUZZLE_5D_PLAY_ICON_KINDS,
    PUZZLE_5D_PLAY_KINDS_TAB_ID,
    PUZZLE_5D_PLAY_STORE_ID,
    Puzzle5dPlayShellController,
    Puzzle5dStoreBridge,
    buildPuzzle5d2dDeclarativeBody,
    buildPuzzle5d3dDeclarativeBody,
    buildPuzzle5dPlayHierarchySections,
    buildPuzzle5dPlayInspectorTree,
    buildPuzzle5dPlayKindsTree,
    puzzle5dFixturePaletteTreeDragController,
    type Puzzle5dPlayHostBridge,
    type Puzzle5dPlaySnapshot
} from "@semio-tech/puzzle-5d-core";
import {
    FiveD,
    Puzzle5dBrushPairedSync,
    StoreProvider,
    createStore,
    parseModel,
    buildPuzzle5dFillSequence,
    project2dKindCatalogs,
    project3d,
    project3dKindCatalogs,
    puzzle5dBrushPlacementFromFlat,
    puzzle5dBrushPlacementFromVolume,
    puzzle5dCommitBrushPlacementToPlay,
    puzzle5dCommitVolumeBrushPlacementToPlay,
    puzzle5dFlatRendererRef,
    useStore as usePuzzle5dStore,
    type Model,
    type Store as Puzzle5dStore,
} from "@semio-tech/puzzle-5d-react";
// #endregion 🔌Adapters

//#region 🔖Snapshot
function usePuzzle5dPlaySnapshot(): { readonly controller: Puzzle5dPlayShellController | undefined; readonly snapshot: Puzzle5dPlaySnapshot | null } {
  const { runtime } = useApp();
  reactHostPort.useSyncExternalStore(
    (listener) => runtime.subscribe(listener),
    () => runtime.generation,
    () => 0,
  );
  const controller = runtime.getActiveApp()?.controller as Puzzle5dPlayShellController | undefined;
  return { controller, snapshot: controller?.getSnapshot() ?? null };
}
//#endregion 🔖Snapshot

//#region 🔖HostBridge
function Puzzle5dPlayHostBridgeInstaller(props: { readonly controller: Puzzle5dPlayShellController; readonly store: Puzzle5dStore }): null {
  const { controller, store } = props;
  const selectionMethodRef = reactHostPort.useRef<Puzzle2dSelectionMethod>("rectangle");
  const selectionModeRef = reactHostPort.useRef<Puzzle2dSelectionMode>("default");
  const selectionTargetsRef = reactHostPort.useRef<Puzzle2dSelectionTargets>({ nodes: true, edges: true, handles: true });
  const gridSnapRef = reactHostPort.useRef(true);
  const redrawPlayingRef = reactHostPort.useRef(false);
  const fillSeedRef = reactHostPort.useRef(1);

  reactHostPort.useEffect(() => {
    installPuzzle3dPlayBrushHost(store.read().meta);
  }, [store]);

  reactHostPort.useLayoutEffect(() => {
    const commitBrushPlacement = (payload: Parameters<typeof puzzle5dCommitBrushPlacementToPlay>[1]) => {
      if (puzzle5dCommitBrushPlacementToPlay(store, payload)) {
        controller.setBrushEngagementPossibles([]);
      }
    };
    puzzle2dSetBrushPlaceCommitHandler(commitBrushPlacement);
    return () => {
      puzzle2dSetBrushPlaceCommitHandler(null);
    };
  }, [controller, store]);

  reactHostPort.useEffect(() => {
    const bridge: Puzzle5dPlayHostBridge = {
      getToolbarState: () => ({
        puzzle2dActiveTool: controller.getActiveTool(),
        puzzle2dSuggestionOffset: controller.getSuggestionOffset(),
        puzzle2dSelectionMethod: selectionMethodRef.current,
        puzzle2dSelectionMode: selectionModeRef.current,
        puzzle2dSelectionTargets: selectionTargetsRef.current,
        puzzle2dGridSnapEnabled: gridSnapRef.current,
        puzzle2dRedrawPlaying: redrawPlayingRef.current,
      }),
      runHostCommand: (command, args) => {
        switch (command) {
          case "setActiveTool": {
            const tool = (args as { tool?: string }).tool;
            const prev = (args as { prevTool?: string }).prevTool;
            if (prev === "fill" && tool !== "fill") {
              store.clearFill();
            }
            if (tool === "fill" && prev !== "fill") {
              fillSeedRef.current = (Date.now() ^ Math.floor(Math.random() * 0x7fffffff)) >>> 0;
            }
            break;
          }
          case "setSuggestionOffset": {
            const distance = Number((args as { distance?: number }).distance);
            if (Number.isFinite(distance)) {
              puzzle2dActiveRenderer()?.setSuggestionOffset(distance);
            }
            break;
          }
          case "setBrushOverlapBudget": {
            break;
          }
          case "pickBrushCandidate": {
            const index = Number((args as { index?: number }).index);
            if (Number.isFinite(index)) {
              puzzle5dPickBrushCandidateAtIndex(index);
            }
            break;
          }
          case "engagementPossibleSelect": {
            const possibleId = (args as { possibleId?: string }).possibleId ?? "";
            const brushMatch = possibleId.match(/^puzzle(?:2d|3d|5d)\.brush\.(\d+)$/);
            if (brushMatch) {
              const index = Number(brushMatch[1]);
              if (Number.isFinite(index)) {
                puzzle5dPickBrushCandidateAtIndex(index);
              }
              break;
            }
            break;
          }
          case "setSelectionMethod": {
            const method = (args as { method?: Puzzle2dSelectionMethod }).method;
            if (method) {
              selectionMethodRef.current = method;
              puzzle2dActiveRenderer()?.setSelectionOptions({ method });
            }
            break;
          }
          case "setSelectionMode": {
            const mode = (args as { mode?: Puzzle2dSelectionMode }).mode;
            if (mode) {
              selectionModeRef.current = mode;
              puzzle2dActiveRenderer()?.setSelectionOptions({ mode });
            }
            break;
          }
          case "toggleSelectionTarget": {
            const kind = (args as { kind?: keyof Puzzle2dSelectionTargets }).kind;
            if (kind) {
              selectionTargetsRef.current = { ...selectionTargetsRef.current, [kind]: !selectionTargetsRef.current[kind] };
            }
            break;
          }
          case "toggleGridSnap": {
            gridSnapRef.current = !gridSnapRef.current;
            break;
          }
          case "toggleRedrawPlaying": {
            redrawPlayingRef.current = !redrawPlayingRef.current;
            break;
          }
          case "clearSelection": {
            controller.run("set2dSelection", { ids: [] });
            controller.run("set3dSelection", { objectIds: [] });
            store.setSelection({ partIds: [], gripIds: [] });
            break;
          }
          default:
            break;
        }
      },
    };
    controller.setHostBridge(bridge);
    return () => controller.setHostBridge(null);
  }, [controller, store]);

  return null;
}

function puzzle5dPickBrushCandidateAtIndex(index: number): void {
  const flatSession = puzzle2dGetBrushSessionSnapshot();
  const flatCandidate = flatSession?.candidates[index];
  puzzle5dFlatRendererRef.current?.setBrushCandidateIndex(index);
  if (!flatCandidate) {
    puzzle3dBrushEngagementSourceRef.current.pickCandidate(index);
    return;
  }
  const volumeIndex = puzzle3dBrushEngagementSourceRef.current.candidates.findIndex(
    (row) => row.objectKindId === flatCandidate.nodeKind && row.sourceVortexIndex === flatCandidate.targetHandleIndex,
  );
  if (volumeIndex >= 0) {
    puzzle3dBrushEngagementSourceRef.current.pickCandidate(volumeIndex);
  }
}

function puzzle5dBrushCandidateRows(payload: Puzzle2dBrushCandidatesPayload, kindCatalogs: ReturnType<typeof project2dKindCatalogs>): { readonly id: string; readonly label: string }[] {
  return payload.candidates.map((candidate, index) => {
    const labels = puzzle2dBrushCandidateDisplayLabels(candidate, kindCatalogs ?? undefined);
    return {
      id: `puzzle5d.brush.${index}`,
      label: `${labels.object} · ${labels.handle}`,
    };
  });
}

function usePuzzle5dPlayStore(): Puzzle5dStore {
  return usePuzzle5dStore();
}
//#endregion 🔖HostBridge

const puzzle5dPlayControllerRef: { current: Puzzle5dPlayShellController | null } = { current: null };

function buildPuzzle5dPlayInspectorTreePanel(snapshot: Puzzle5dPlaySnapshot | null): UiTreeNode {
  if (!snapshot) {
    return uiDeclarativeSectionsToTree([
      { type: "section", id: "puzzle-5d-play-inspector.empty", label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, children: [{ type: "text", value: "No puzzle 5d snapshot" }] },
    ]);
  }
  return buildPuzzle5dPlayInspectorTree(snapshot);
}

//#region 🔖DetailsPanel
class Puzzle5dPlayHierarchyPanelDefinition extends PureSidePanelTabDefinition {
  constructor(
    private readonly buildTree: () => import("@semio-tech/framework-playground-core").UiTreeNode,
    private readonly commandBus: CommandBus,
  ) {
    super();
  }

  buildTab(): SidePanelTabConfig {
    return {
      id: PUZZLE_5D_PLAY_HIERARCHY_TAB_ID,
      icon: createIconComponent(FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID),
      name: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const treeNode = this.buildTree();
        return uiTreeNodeToTreePanelConfig(treeNode, this.commandBus);
      }),
    };
  }
}

class Puzzle5dPlayKindsPanelDefinition extends PureSidePanelTabDefinition {
  constructor(
    private readonly buildTree: () => import("@semio-tech/framework-playground-core").UiTreeNode,
    private readonly commandBus: CommandBus,
  ) {
    super();
  }

  buildTab(): SidePanelTabConfig {
    return {
      id: PUZZLE_5D_PLAY_KINDS_TAB_ID,
      icon: createIconComponent(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID),
      name: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
      order: 1,
      tree: new CallbackTreePanelDefinition(() => {
        const treeNode = this.buildTree();
        const config = uiTreeNodeToTreePanelConfig(treeNode, this.commandBus);
        return {
          ...config,
          dragAndDropController: puzzle5dFixturePaletteTreeDragController(collectUiTreeItemDragData(treeNode.sections)),
        };
      }),
    };
  }
}

class Puzzle5dPlayInspectorPanelDefinition extends PureSidePanelTabDefinition {
  constructor(
    private readonly buildTree: () => import("@semio-tech/framework-playground-core").UiTreeNode,
    private readonly commandBus: CommandBus,
  ) {
    super();
  }

  buildTab(): SidePanelTabConfig {
    return {
      id: "puzzle-5d-play-inspector",
      icon: createIconComponent(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID),
      name: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => uiTreeNodeToTreePanelConfig(this.buildTree(), this.commandBus)),
    };
  }
}
//#endregion 🔖DetailsPanel

//#region 🔖Surfaces
function Puzzle5d2dSurfaceHost({ node }: { readonly node: UiPuzzle2dHostSurfaceNode }): React.ReactElement {
  const { controller, snapshot } = usePuzzle5dPlaySnapshot();
  const store = usePuzzle5dPlayStore();
  const bindingValid =
    node.controllerId === PUZZLE_5D_PLAY_CONTROLLER_ID &&
    node.surfaceId === PUZZLE_5D_PLAY_2D_SURFACE_ID &&
    node.paneId === PUZZLE_5D_PLAY_2D_WINDOW_ID &&
    Boolean(controller && snapshot?.fixture2d);
  const flatCatalogs = reactHostPort.useMemo(() => project2dKindCatalogs(store.read().kindCatalogs), [store, snapshot?.manifestLabel]);
  const controllerRef = reactHostPort.useRef(controller);
  const storeRef = reactHostPort.useRef(store);
  const activeToolRef = reactHostPort.useRef(snapshot?.activeTool ?? "select");
  controllerRef.current = controller;
  storeRef.current = store;
  activeToolRef.current = snapshot?.activeTool ?? "select";
  const onLodChange = reactHostPort.useCallback((lod: Puzzle2dDrawLodKind) => {
    controllerRef.current?.commandBus.dispatch(PUZZLE_5D_PLAY_CONTROLLER_ID, "set2dLodTag", { lod });
  }, []);
  const onSelect = reactHostPort.useCallback((snap: { readonly ids: readonly string[] }) => {
    controllerRef.current?.commandBus.dispatch(PUZZLE_5D_PLAY_CONTROLLER_ID, "set2dSelection", { ids: snap.ids });
  }, []);
  const onConnect = reactHostPort.useCallback(() => {
    controllerRef.current?.commandBus.dispatch(PUZZLE_5D_PLAY_CONTROLLER_ID, "note2dConnect");
  }, []);
  const onProximityConnect = reactHostPort.useCallback(() => {
    controllerRef.current?.commandBus.dispatch(PUZZLE_5D_PLAY_CONTROLLER_ID, "note2dProximity");
  }, []);
  const onBrushPlace = reactHostPort.useCallback((payload: Parameters<typeof puzzle5dBrushPlacementFromFlat>[0]) => {
    if (puzzle5dCommitBrushPlacementToPlay(storeRef.current, payload)) {
      controllerRef.current?.setBrushEngagementPossibles([]);
    }
  }, []);
  const onBrushCandidates = reactHostPort.useCallback((payload: Puzzle2dBrushCandidatesPayload) => {
    if (activeToolRef.current !== "brush") {
      controllerRef.current?.setBrushEngagementPossibles([]);
      return;
    }
    controllerRef.current?.setBrushEngagementPossibles(puzzle5dBrushCandidateRows(payload, flatCatalogs));
  }, [flatCatalogs]);
  const onDelete = reactHostPort.useCallback(() => {
    const selection = storeRef.current.getSnapshot().selection;
    controllerRef.current?.commandBus.dispatch(PUZZLE_5D_PLAY_CONTROLLER_ID, "set2dSelection", { ids: [...selection.partIds] });
    controllerRef.current?.commandBus.dispatch(PUZZLE_5D_PLAY_CONTROLLER_ID, "set3dSelection", {
      objectIds: selection.partIds.length > 0 ? [selection.partIds[0]!] : [],
    });
  }, []);
  const onFixtureDrop = reactHostPort.useCallback((detail: Puzzle2dFixtureDropDetail) => {
    const partId = storeRef.current.applyPaletteNodeDrop(detail);
    if (!partId) {
      return;
    }
    controllerRef.current?.commandBus.dispatch(PUZZLE_5D_PLAY_CONTROLLER_ID, "set2dSelection", { ids: [partId] });
    controllerRef.current?.commandBus.dispatch(PUZZLE_5D_PLAY_CONTROLLER_ID, "set3dSelection", { objectIds: [partId] });
  }, []);
  if (!bindingValid || !controller || !snapshot) {
    return <div className="p-2 text-xs text-muted-foreground">Invalid puzzle 5d 2d binding</div>;
  }
  return (
    <FiveD
      mode="2d"
      instanceId="play-2d"
      activeTool={snapshot.activeTool}
      suggestionOffset={snapshot.suggestionOffset}
      puzzle2d={{
        onLodChange,
        onSelect,
        onConnect,
        onProximityConnect,
        onBrushPlace,
        onBrushCandidates,
        onDelete,
        fixtureDragDrop: true,
        onFixtureDrop,
        selectionMethod: snapshot.selectionMethod,
        selectionMode: snapshot.selectionMode,
        ...snapshot.lod2dProps,
      }}
    />
  );
}

function Puzzle5d3dSurfaceHost({ node }: { readonly node: UiPuzzle3dHostSurfaceNode }): React.ReactElement {
  const { controller, snapshot } = usePuzzle5dPlaySnapshot();
  const store = usePuzzle5dPlayStore();
  const modelPartCount = reactHostPort.useSyncExternalStore(store.subscribe, () => store.read().parts.length, () => store.read().parts.length);
  const fillSeedRef = reactHostPort.useRef(1);
  const fillBaseCaptureRef = reactHostPort.useRef<ReturnType<Puzzle5dStore["read"]> | null>(null);
  const fillPrepareTimerRef = reactHostPort.useRef<ReturnType<typeof setTimeout> | null>(null);
  const fillSessionPreparedRef = reactHostPort.useRef(false);
  const prevActiveToolRef = reactHostPort.useRef(snapshot?.activeTool);
  const bindingValid =
    node.controllerId === PUZZLE_5D_PLAY_CONTROLLER_ID &&
    node.surfaceId === PUZZLE_5D_PLAY_3D_SURFACE_ID &&
    Boolean(controller && snapshot?.fixture3d && snapshot.fixture2d);
  const controllerRef = reactHostPort.useRef(controller);
  const storeRef = reactHostPort.useRef(store);
  const brushOverlapBudgetRef = reactHostPort.useRef(snapshot?.brushOverlapBudget ?? 0);
  controllerRef.current = controller;
  storeRef.current = store;
  brushOverlapBudgetRef.current = snapshot?.brushOverlapBudget ?? 0;
  reactHostPort.useEffect(() => {
    if (!bindingValid) return;
    const urls = [...new Set(store.read().parts.flatMap((part) => (part["3d"] ? [part["3d"].meshUrl] : [])))];
    for (const url of urls) sceneHostPort.drei.useGLTF.preload(url);
  }, [bindingValid, modelPartCount, store]);
  reactHostPort.useLayoutEffect(() => {
    const prev = prevActiveToolRef.current;
    prevActiveToolRef.current = snapshot?.activeTool;
    if (snapshot?.activeTool === "fill" && prev !== "fill") {
      fillBaseCaptureRef.current = structuredClone(store.read());
      fillSessionPreparedRef.current = false;
      fillSeedRef.current = (Date.now() ^ Math.floor(Math.random() * 0x7fffffff)) >>> 0;
    }
    if (snapshot?.activeTool !== "fill") {
      fillBaseCaptureRef.current = null;
      fillSessionPreparedRef.current = false;
    }
  }, [snapshot?.activeTool, store]);
  const volumeKindCatalogs = reactHostPort.useMemo(
    () => project3dKindCatalogs(snapshot?.kindCatalogs ?? snapshot?.sharedKinds.kindCatalogs),
    [snapshot?.kindCatalogs, snapshot?.sharedKinds.kindCatalogs],
  );
  reactHostPort.useEffect(() => {
    if (!bindingValid || snapshot?.activeTool !== "fill" || !snapshot.fixture3d) {
      return;
    }
    const urls = brushMeshUrlsForFillSession(snapshot.fixture3d, volumeKindCatalogs, snapshot.model.kindCompatibility);
    for (const url of urls) {
      if (isLoadableMeshUrl(url)) {
        sceneHostPort.drei.useGLTF.preload(url);
      }
    }
  }, [bindingValid, snapshot?.activeTool, snapshot?.fixture3d, snapshot?.model.kindCompatibility, volumeKindCatalogs]);
  const onSelect = reactHostPort.useCallback((selection: { readonly objectIds: readonly string[] }) => {
    controllerRef.current?.commandBus.dispatch(PUZZLE_5D_PLAY_CONTROLLER_ID, "set3dSelection", { objectIds: selection.objectIds });
  }, []);
  const onConnect = reactHostPort.useCallback(() => {
    controllerRef.current?.commandBus.dispatch(PUZZLE_5D_PLAY_CONTROLLER_ID, "note3dConnect");
  }, []);
  const onProximityConnect = reactHostPort.useCallback(() => {
    controllerRef.current?.commandBus.dispatch(PUZZLE_5D_PLAY_CONTROLLER_ID, "note3dProximity");
  }, []);
  const onBrushPlace = reactHostPort.useCallback((payload: Parameters<typeof puzzle5dBrushPlacementFromVolume>[0]) => {
    if (puzzle5dCommitVolumeBrushPlacementToPlay(storeRef.current, payload)) {
      controllerRef.current?.setBrushEngagementPossibles([]);
    }
  }, []);
  const onFillMeshesReady = reactHostPort.useCallback(() => {
    if (fillPrepareTimerRef.current !== null) {
      clearTimeout(fillPrepareTimerRef.current);
    }
    fillPrepareTimerRef.current = setTimeout(() => {
      fillPrepareTimerRef.current = null;
      const base = fillBaseCaptureRef.current;
      if (!base || fillSessionPreparedRef.current) {
        return;
      }
      const activeStore = storeRef.current;
      const activeController = controllerRef.current;
      activeStore.setFillBuildDone(false);
      const sequence = buildPuzzle5dFillSequence({
        model: base,
        seed: fillSeedRef.current,
        overlapBudget: brushOverlapBudgetRef.current,
        meshRootForUrl: puzzle3dBrushMeshRootForFill,
      });
      activeStore.prepareFillSession(sequence, base, fillSeedRef.current);
      activeStore.setFillBuildDone(true);
      fillSessionPreparedRef.current = true;
      if (sequence.length > 0) {
        activeController?.run("setFillCount", { count: 1 });
      }
    }, 0);
  }, []);
  reactHostPort.useEffect(
    () => () => {
      if (fillPrepareTimerRef.current !== null) {
        clearTimeout(fillPrepareTimerRef.current);
      }
    },
    [],
  );
  const onFixtureDrop = reactHostPort.useCallback(
    (detail: Puzzle3dFixtureDropDetail) => {
      const sceneFixture = project3d(storeRef.current.read());
      const result = resolvePuzzle3dFixtureDrop(detail, volumeKindCatalogs, sceneFixture);
      if (result.kind !== "palette-object") {
        return;
      }
      const partId = storeRef.current.applyPaletteObjectDrop(result.object);
      if (!partId) {
        return;
      }
      controllerRef.current?.commandBus.dispatch(PUZZLE_5D_PLAY_CONTROLLER_ID, "set2dSelection", { ids: [partId] });
      controllerRef.current?.commandBus.dispatch(PUZZLE_5D_PLAY_CONTROLLER_ID, "set3dSelection", { objectIds: [partId] });
    },
    [volumeKindCatalogs],
  );
  if (!bindingValid || !controller || !snapshot?.fixture3d) {
    return <div className="p-2 text-xs text-muted-foreground">Invalid puzzle 5d 3d binding</div>;
  }
  return (
    <FiveD
      mode="3d"
      instanceId="play-3d"
      activeTool={snapshot.activeTool}
      brushOverlapBudget={snapshot.brushOverlapBudget}
      gumballConfig={snapshot.gumballConfig}
      puzzle3d={{
        ...snapshot.lod3dProps,
        onSelect,
        onConnect,
        onProximityConnect,
        onBrushPlace,
        onFillMeshesReady,
        fixtureDragDrop: true,
        onFixtureDrop,
        selectionMethod: snapshot.selectionMethod,
        selectionMode: snapshot.selectionMode,
      }}
    />
  );
}
//#endregion 🔖Surfaces

function Puzzle5dPlayJackSurfaceHost({ node: _node }: { readonly node: import("@semio-tech/framework-platform-core").UiWriterHostSurfaceNode }): React.ReactElement {
  const { controller } = usePuzzle5dPlaySnapshot();
  void controller?.getHoverEpoch();
  void controller?.getSelectEpoch();
  const document = controller?.getWriterDocumentJack() ?? createWriterDocument({ id: "puzzle-5d-jack", languageId: "jack", text: "" });
  const onHoverChange = reactHostPort.useCallback((offset: number | null) => {
    puzzle5dPlayControllerRef.current?.run("setJackHover", { offset });
  }, []);
  const onSelectionChange = reactHostPort.useCallback((range: { start: number; end: number }) => {
    puzzle5dPlayControllerRef.current?.run("setJackSelect", range);
  }, []);
  return (
    <WriterCanvas
      document={document}
      className="h-full"
      onHoverChange={onHoverChange}
      onSelectionChange={onSelectionChange}
      externalHoverOccurrences={controller?.getJackHoverOccurrences()}
      externalHoverOccurrencesSignal={controller?.getHoverEpoch()}
      externalSelectionOccurrences={controller?.getJackSelectOccurrences()}
      externalSelectionOccurrencesSignal={controller?.getSelectEpoch()}
    />
  );
}

//#region 🔖Mount
let topologyPlayChromeRegistered = false;

/** @emoji 🧊 Registers topology play flat+volume surface hosts (called from `@semio-tech/framework-playground-renderer-react`). */
export function registerPuzzle5dPlaySurfaceHosts(): void {
  if (topologyPlayChromeRegistered) return;
  topologyPlayChromeRegistered = true;
  registerUiPuzzle2dSurfaceHost(PUZZLE_5D_PLAY_2D_SURFACE_ID, Puzzle5d2dSurfaceHost);
  registerUiPuzzle3dSurfaceHost(PUZZLE_5D_PLAY_3D_SURFACE_ID, Puzzle5d3dSurfaceHost);
  registerUiWriterSurfaceHost(PUZZLE_5D_PLAY_JACK_SURFACE_ID, Puzzle5dPlayJackSurfaceHost);
  registerTabIcon(PUZZLE_5D_PLAY_ICON_KINDS, "tags");
  registerWindowBody(PUZZLE_5D_PLAY_2D_BODY_KEY, buildPuzzle5d2dDeclarativeBody);
  registerWindowBody(PUZZLE_5D_PLAY_3D_BODY_KEY, buildPuzzle5d3dDeclarativeBody);
  registerWindowBody(PUZZLE_5D_PLAY_JACK_BODY_KEY, () =>
    buildWriterWindowBody(PUZZLE_5D_PLAY_JACK_SURFACE_ID, PUZZLE_5D_PLAY_CONTROLLER_ID, PUZZLE_5D_PLAY_JACK_WINDOW_ID));
}

function Puzzle5dPlayChrome({
  runtime,
  playgroundKeybindings,
}: {
  readonly runtime: Platform;
  readonly playgroundKeybindings?: readonly { readonly key: string; readonly controllerId: string; readonly command: string }[];
}): React.ReactElement {
  const generation = reactHostPort.useSyncExternalStore(
    (listener) => runtime.subscribe(listener),
    () => runtime.generation,
    () => 0,
  );
  void generation;
  const controller = runtime.getActiveApp()?.controller as Puzzle5dPlayShellController | undefined;
  puzzle5dPlayControllerRef.current = controller ?? null;
  const snapshot = controller?.getSnapshot() ?? null;
  const bus = runtime.commandBus;
  const puzzle5dStore = controller?.puzzle5dStore;
  const snapshotKey = snapshot
    ? `${snapshot.manifestLabel ?? ""}\u0001${snapshot.selection.partIds.join(",")}\u0001${snapshot.selection.gripIds.join(",")}`
    : "";
  const workbenchTabs = reactHostPort.useMemo(
    () =>
      snapshot && controller
        ? [
            new Puzzle5dPlayHierarchyPanelDefinition(() => buildPuzzle5dPlayHierarchySections(snapshot), bus).resolveTab(),
            new Puzzle5dPlayKindsPanelDefinition(() => buildPuzzle5dPlayKindsTree(snapshot), bus).resolveTab(),
          ]
        : [],
    [snapshot, snapshotKey, controller, bus, puzzle5dStore],
  );
  const detailTabs = reactHostPort.useMemo(
    () => (snapshot ? [new Puzzle5dPlayInspectorPanelDefinition(() => buildPuzzle5dPlayInspectorTreePanel(snapshot), bus).resolveTab()] : []),
    [snapshot, snapshotKey, bus],
  );
  const shell = (
    <PlaygroundView
      runtime={runtime}
      defaultAppId={PUZZLE_5D_PLAY_APP_ID}
      playgroundKeybindings={playgroundKeybindings}
      augmentPanelTabs={{ workbench: workbenchTabs, details: detailTabs }}
    />
  );
  if (!controller) {
    return shell;
  }
  const puzzle5dBridge = controller.getStore(PUZZLE_5D_PLAY_STORE_ID) as Puzzle5dStoreBridge | undefined;
  const storeForProvider = puzzle5dBridge?.inner ?? controller.puzzle5dStore;
  return (
    <StoreProvider store={storeForProvider}>
      <Puzzle5dBrushPairedSync />
      <Puzzle5dPlayHostBridgeInstaller controller={controller} store={storeForProvider} />
      {shell}
    </StoreProvider>
  );
}

/** @emoji 🚀 Mounts puzzle 5d play chrome for a {@link Playground}. */
export function mountPuzzle5dPlayChrome(playground: Playground, rootId = "root"): void {
  mountPlaygroundApp(<Puzzle5dPlayChrome runtime={playground.runtime} playgroundKeybindings={playground.keybindings} />, rootId);
}

const topologyPlayChromeBoot: PlaygroundChromeBoot = {
  registerHosts: registerPuzzle5dPlaySurfaceHosts,
  mount: mountPuzzle5dPlayChrome,
};

/** @emoji 🛝 Puzzle 5d play entry: register hosts, bodies, mount chrome (from `puzzle/5d/play/index.ts`). */
export function boot5dPlay(playground: Playground, rootId = "root"): void {
  bootPlayground(playground, topologyPlayChromeBoot, rootId);
}

//#endregion 🔖Mount

// #endregion 🛝PlayHost
//#endregion 🔖Puzzle5dPlayHost

//#region 🔖Puzzle2dPlayHost
// #region 🔌Adapters
import {
    PUZZLE_2D_FILL_COUNT_MAX,
    PUZZLE_2D_PLAY_APP_ID,
    PUZZLE_2D_PLAY_BODY_KEY_DETAIL,
    PUZZLE_2D_PLAY_BODY_KEY_JACK,
    PUZZLE_2D_PLAY_BODY_KEY_OVERVIEW,
    PUZZLE_2D_PLAY_BODY_KEY_SELECTION,
    PUZZLE_2D_PLAY_CONTROLLER_ID,
    PUZZLE_2D_PLAY_DEFAULT_FIXTURE,
    PUZZLE_2D_PLAY_EMPTY_FIXTURE,
    PUZZLE_2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID,
    PUZZLE_2D_PLAY_EXAMPLE_NAKAGIN_ID,
    PUZZLE_2D_PLAY_EXAMPLE_OPTIONS,
    PUZZLE_2D_PLAY_HIERARCHY_TAB_ID,
    PUZZLE_2D_PLAY_ICON_KINDS,
    PUZZLE_2D_PLAY_KINDS_TAB_ID,
    PUZZLE_2D_PLAY_SETTINGS_BODY_KEY,
    PUZZLE_2D_PLAY_SURFACE_ID,
    PUZZLE_2D_PLAY_SURFACE_ID_JACK,
    PUZZLE_2D_PLAY_SURFACE_ID_COMPILED_DAG,
    PUZZLE_2D_PLAY_WINDOW_KIND_JACK,
    Puzzle2dPlayShellController,
    applyPuzzle2dFillCount,
    buildPuzzle2dPlayDetailDeclarativeBody,
    buildPuzzle2dPlayHierarchySections,
    buildPuzzle2dPlayKindsTree,
    buildPuzzle2dPlayOverviewDeclarativeBody,
    buildPuzzle2dPlaySelectionDeclarativeBody,
    clearPuzzle2dFillSession,
    flushPuzzle2dPlayStructuralDeleteBatch,
    getPuzzle2dFillSessionReadyEpoch,
    preparePuzzle2dFillSession,
    puzzle2dFillBuildProgressRef,
    puzzle2dPlayAllSelectionFromFixture,
    puzzle2dPlayApplyNodeStructuralDeleteToFixture,
    puzzle2dPlayApplySelectionFlag,
    puzzle2dPlayCmd,
    puzzle2dPlayDeleteSelectionFromFixture,
    puzzle2dPlayDuplicateSelection,
    puzzle2dPlayFixtureForId,
    puzzle2dPlayFixtureJson,
    puzzle2dFixtureToJson,
    puzzle2dPlayForwardsCanvasStructuralDelete,
    puzzle2dPlayHierarchyGraphIdFromTreeItemId,
    puzzle2dPlayHierarchyTreeHighlightedIds,
    puzzle2dPlayHierarchyTreeSelectedIds,
    puzzle2dPlayInspectorKindSectionLabel,
    puzzle2dPlayKindCatalogSelectItems,
    puzzle2dPlayKindsTreeHighlightedIds,
    puzzle2dPlayPaneFromShellWindowId,
    puzzle2dPlayRehydrateFixtureEdgesIfMissing,
    puzzle2dPlaySelectSameKindIds,
    puzzle2dPlayToggleEntityFlag,
    puzzle2dPlayTriptychCamerasFromFixture,
    subscribePuzzle2dFillSessionReady,
    type Puzzle2dPlayHostBridge,
    type Puzzle2dPlayPaneId,
    type Puzzle2dPlayStructuralDeleteItem,
} from "@semio-tech/puzzle-2d-core";
import {
    BUILTIN_PORT_HANDLE_KIND,
    DEFAULT_PUZZLE_2D_BRUSH_NODE_SIZE_PX,
    DEFAULT_PUZZLE_2D_SUGGESTION_OFFSET_PX,
    PUZZLE_2D_CAMERA_ZOOM_MAX,
    PUZZLE_2D_CAMERA_ZOOM_MIN,
    PUZZLE_2D_FIXTURE_DRAG_MIME,
    PUZZLE_2D_LOD_MODE_AUTOMATIC,
    PUZZLE_2D_PRESELECT_EMPTY,
    PUZZLE_2D_SELECTION_TARGETS_DEFAULT,
    Puzzle2dCanvas,
    buildPuzzle2dSceneDescriptorFromFixture,
    clonePuzzle2dFixture,
    layoutPuzzle2dFixtureRedrawHandles,
    layoutPuzzle2dFixtureRedrawNodes,
    puzzle2dActiveRenderer,
    puzzle2dApplyLiveForceGraphLayoutTick,
    puzzle2dApplyNodeKindToFixtureNode,
    puzzle2dBrushSuggestionsMenuOpen,
    puzzle2dBrushCandidateDisplayLabels,
    puzzle2dCommitBrushPlacementToPlay,
    puzzle2dCommitPaletteNodeDropToPlay,
    puzzle2dEdgeKindOverlayLabel,
    puzzle2dFinalizeLiveForceGraphLayoutTick,
    puzzle2dFixtureHandleEndpointDisplayLabel,
    puzzle2dFixtureMergedKindCatalogs,
    puzzle2dFixtureMetaKindCompatibility,
    puzzle2dFixtureNodeCaption,
    puzzle2dFixtureObjectDisplayLabel,
    puzzle2dFixturePaletteTreeDragController,
    puzzle2dFixtureSceneMarkers,
    puzzle2dGetBrushSessionSnapshot,
    puzzle2dHandleAngleFromRingT,
    puzzle2dHandleAngleToRingT,
    puzzle2dHandleKindOverlayLabel,
    puzzle2dNodeKindOverlayLabel,
    puzzle2dIsBrushPlacementStructuralDeleteGuarded,
    puzzle2dSetBrushPlaceCommitHandler,
    puzzle2dSelectionActionsRef,
    puzzle2dSyncBrushSessionToAllAuthoringPeers,
    puzzle2dSyncFixtureDescriptorToAllAuthoringPeers,
    puzzle2dSyncSelectionToAllAuthoringPeers,
    type Puzzle2dActiveTool,
    type Puzzle2dBrushCandidatesPayload,
    type Puzzle2dBrushPlacePayload,
    type Puzzle2dDrawLodKind,
    type Puzzle2dFixtureCircleNode,
    type Puzzle2dFixtureEdge,
    type Puzzle2dFixtureDropDetail,
    type Puzzle2dFixtureHandle,
    type Puzzle2dFixtureNode,
    type Puzzle2dFixture,
    type Puzzle2dHierarchicalTreeDirectionKind,
    type Puzzle2dHoverPayload,
    type Puzzle2dKindHover,
    type Puzzle2dLiveForceGraphDragState,
    type Puzzle2dLodModeKind,
    type Puzzle2dPreselectSnapshot,
    type Puzzle2dSelectionMethod,
    type Puzzle2dSelectionMode,
    type Puzzle2dSelectionTargets,
    type Puzzle2dRedrawLayoutOptions,
    type Puzzle2dRedrawModeKind,
    type Puzzle2dSelectionSnapshot,
    type Puzzle2dStructureDeletePayload
} from "@semio-tech/puzzle-2d-react";
import {
    WIRES_PLAY_DEFAULT_FIXTURE,
    WIRES_PLAY_FIXTURE,
    WIRES_PLAY_EXAMPLE_METABOLISM_ID,
    WIRES_PLAY_EXAMPLE_OPTIONS,
    WIRES_PLAY_HIERARCHY_TAB_ID,
    WIRES_PLAY_KINDS_TAB_ID,
    WIRES_PLAY_LIVE_FORCE_GRAPH_DEFAULTS,
    buildWiresPlayHierarchySections,
    buildWiresPlayKindsTree,
    wiresPlayHierarchyGraphIdFromTreeItemId,
    wiresPlayHierarchyTreeHighlightedIds,
    wiresPlayHierarchyTreeSelectedIds,
    wiresPlayIdentityLabelForNodeId,
    wiresPlayRelationshipKindDisplayName,
} from "@semio-tech/reasoning-mindmap-wires-core";
import type { ReactNode } from "react";
// #endregion 🔌Adapters

const PUZZLE_2D_PLAY_IS_WIRES = import.meta.env.PUZZLE_PLAY_ENTRY === "wires";

function puzzle2dPlayHierarchyTreeSelectedIdsForFixture(fixture: Puzzle2dFixture, graphSelectionIds: readonly string[]): string[] {
  return PUZZLE_2D_PLAY_IS_WIRES
    ? wiresPlayHierarchyTreeSelectedIds(fixture, graphSelectionIds)
    : puzzle2dPlayHierarchyTreeSelectedIds(fixture, graphSelectionIds);
}

function puzzle2dPlayHierarchyTreeHighlightedIdsForFixture(
  fixture: Puzzle2dFixture,
  graphHoverId: string | null,
  kindHover: Puzzle2dKindHover | null = null,
): readonly string[] {
  return PUZZLE_2D_PLAY_IS_WIRES
    ? wiresPlayHierarchyTreeHighlightedIds(fixture, graphHoverId)
    : puzzle2dPlayHierarchyTreeHighlightedIds(fixture, graphHoverId, kindHover);
}

function puzzle2dPlayKindsTreeHighlightedIdsForFixture(
  fixture: Puzzle2dFixture,
  graphHoverId: string | null,
  kindHover: Puzzle2dKindHover | null = null,
): readonly string[] {
  if (PUZZLE_2D_PLAY_IS_WIRES) {
    return [];
  }
  return puzzle2dPlayKindsTreeHighlightedIds(puzzle2dFixtureMergedKindCatalogs(fixture), fixture, graphHoverId, kindHover);
}

function puzzle2dPlayHierarchyGraphIdFromTreeItemIdForPlay(treeItemId: string): string | null {
  return PUZZLE_2D_PLAY_IS_WIRES ? wiresPlayHierarchyGraphIdFromTreeItemId(treeItemId) : puzzle2dPlayHierarchyGraphIdFromTreeItemId(treeItemId);
}

function puzzle2dPlayResolvedDefaultFixture(): Puzzle2dFixture {
  return PUZZLE_2D_PLAY_IS_WIRES ? WIRES_PLAY_DEFAULT_FIXTURE : PUZZLE_2D_PLAY_DEFAULT_FIXTURE;
}

const PUZZLE_2D_PLAY_DEFAULT_KIND_CATALOGS = puzzle2dFixtureMergedKindCatalogs(puzzle2dPlayResolvedDefaultFixture());

// #region 🔖Kinds
export type { Puzzle2dPlayPaneId } from "@semio-tech/puzzle-2d-core";

const puzzle2dPlayOverviewWindowContextMenu: ContextMenuItem[] = [{ id: "win-demo", label: "Overview window menu demo" }];
const puzzle2dPlayDemoNodeContextMenu: ContextMenuItem[] = [
  { id: "demo-node", label: "Demo capsule action" },
  { children: [{ id: "demo-sub-1", label: "Nested item" }], id: "demo-sub", label: "Demo nested" },
];
const puzzle2dPlayDemoEdgeContextMenu: ContextMenuItem[] = [{ id: "demo-edge", label: "Demo edge action" }];
const puzzle2dPlayCanvasBackgroundMenu: ContextMenuItem[] = [{ id: "demo-bg", label: "Puzzle 2D background menu" }];

// #endregion 🔖Kinds

// #region 🔖Geometry
function clampZoom(value: number): number {
  return Math.min(PUZZLE_2D_CAMERA_ZOOM_MAX, Math.max(PUZZLE_2D_CAMERA_ZOOM_MIN, value));
}

function triptychCamerasFromFixture(fixture: Puzzle2dFixture, rawFixture?: unknown): Record<Puzzle2dPlayPaneId, CameraState> {
  return puzzle2dPlayTriptychCamerasFromFixture(fixture, rawFixture);
}

function puzzle2dPlayRawFixtureJsonForNavbarId(fixtureId: string): unknown | undefined {
  if (isPlaygroundNoExampleId(fixtureId) || fixtureId === WIRES_PLAY_EXAMPLE_METABOLISM_ID) {
    return undefined;
  }
  if (fixtureId === PUZZLE_2D_PLAY_EXAMPLE_NAKAGIN_ID || fixtureId === PUZZLE_2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID) {
    return puzzle2dPlayFixtureJson(fixtureId);
  }
  return undefined;
}

function puzzle2dPlayInitialCameras(): Record<Puzzle2dPlayPaneId, CameraState> {
  return triptychCamerasFromFixture(
    puzzle2dPlayResolvedDefaultFixture(),
    puzzle2dPlayFixtureJson(PUZZLE_2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID),
  );
}

/** @emoji ⏱️ After redraw play stops: camera stays fixed for the first third of this span, then eases in the remaining two thirds to bbox fit (3s total). */
const PUZZLE_2D_PLAY_CAMERA_POST_REDRAW_TOTAL_MS = 3000;

/** @emoji ⏱️ After one-shot “Redraw nodes”, shell cameras ease to bbox fit (first third hold, last two thirds smooth). */
const PUZZLE_2D_PLAY_NODES_REDRAW_CAMERA_EASE_TOTAL_MS = 1800;

/** @emoji 📷 Linear blend toward bbox-fit cameras each fixture commit while redraw play is on (damped follow). */
const PUZZLE_2D_PLAY_REDRAW_CAMERA_CHASE_BLEND = 0.22;

function easeInOutCubic01(t: number): number {
  const x = Math.min(1, Math.max(0, t));
  return x < 0.5 ? 4 * x * x * x : 1 - (-2 * x + 2) ** 3 / 2;
}

function lerpCameraState(a: CameraState, b: CameraState, tLinear: number): CameraState {
  const w = easeInOutCubic01(tLinear);
  const zoom = a.zoom > 1e-9 && b.zoom > 1e-9 ? a.zoom * (b.zoom / a.zoom) ** w : a.zoom + (b.zoom - a.zoom) * w;
  return {
    x: a.x + (b.x - a.x) * w,
    y: a.y + (b.y - a.y) * w,
    zoom: clampZoom(zoom),
  };
}

/** @emoji 🎯 Lerps only `activePane` between `from` and `to`; other panes keep shallow copies of `from`. */
function blendTriptychCamerasActivePaneOnly(from: Record<Puzzle2dPlayPaneId, CameraState>, to: Record<Puzzle2dPlayPaneId, CameraState>, tLinear: number, activePane: Puzzle2dPlayPaneId): Record<Puzzle2dPlayPaneId, CameraState> {
  const out: Record<Puzzle2dPlayPaneId, CameraState> = {
    "2d-detail": { ...from["2d-detail"] },
    "2d-overview": { ...from["2d-overview"] },
    "2d-selection": { ...from["2d-selection"] },
  };
  out[activePane] = lerpCameraState(from[activePane], to[activePane], tLinear);
  return out;
}

function dampCameraStateLinear(a: CameraState, b: CameraState, w: number): CameraState {
  const t = Math.min(1, Math.max(0, w));
  const zoom = a.zoom > 1e-9 && b.zoom > 1e-9 ? a.zoom * (b.zoom / a.zoom) ** t : a.zoom + (b.zoom - a.zoom) * t;
  return {
    x: a.x + (b.x - a.x) * t,
    y: a.y + (b.y - a.y) * t,
    zoom: clampZoom(zoom),
  };
}

/** @emoji ✅ Shared default selection for all play panes (overview node on the Nakagin graph). */
function selectionSeedForFixture(fixture: Puzzle2dFixture): Set<string> {
  const nodeA = fixture.nodes[0];
  return new Set(nodeA?.id ? [nodeA.id] : []);
}
// #endregion 🔖Geometry

// #region 🔖ShellContext
interface Puzzle2dPlayShellValue {
  fixture: Puzzle2dFixture;
  setFixture: (next: Puzzle2dFixture) => void;
  /** @emoji 🎯 Palette drags merge one node at the pointer; full fixtures replace the graph. */
  handleCanvasFixtureDrop: (pane: Puzzle2dPlayPaneId, detail: Puzzle2dFixtureDropDetail) => void;
  patchFixture: (updater: (prev: Puzzle2dFixture) => Puzzle2dFixture) => void;
  activePaneId: Puzzle2dPlayPaneId;
  setActivePaneId: (id: Puzzle2dPlayPaneId) => void;
  /** @emoji ✅ Commits selection to WASM peers + selection context; stable callback (not `selectionIds`). */
  setSelectionIds: (ids: readonly string[]) => void;
  hoveredId: string | null;
  hoveredKind: Puzzle2dKindHover | null;
  /** @emoji 🖱️ Pane that currently owns pointer hover updates for shared {@link Puzzle2dPlayShellValue.hoveredId}. */
  hoverSourcePane: Puzzle2dPlayPaneId | null;
  setHoverPane: (pane: Puzzle2dPlayPaneId) => void;
  setHoverForPane: (pane: Puzzle2dPlayPaneId, payload: Puzzle2dHoverPayload) => void;
  clearHoverForPane: (pane: Puzzle2dPlayPaneId) => void;
  /** @emoji 🌳 Sets shared graph hover from hierarchy rows without claiming a canvas pane. */
  setHierarchyHover: (payload: Puzzle2dHoverPayload) => void;
  /** @emoji 🔁 Rewrites selection ids when an object id changes (`replacedId` → `replacementId`); unrelated to edge endpoint fields. */
  remapIdInSelections: (replacedId: string, replacementId: string) => void;
  puzzle2dSelectionMethod: Puzzle2dSelectionMethod;
  setPuzzle2dSelectionMethod: (value: Puzzle2dSelectionMethod) => void;
  puzzle2dSelectionMode: Puzzle2dSelectionMode;
  setPuzzle2dSelectionMode: (value: Puzzle2dSelectionMode) => void;
  puzzle2dSelectionTargets: Puzzle2dSelectionTargets;
  setPuzzle2dSelectionTargets: (value: Puzzle2dSelectionTargets | ((prev: Puzzle2dSelectionTargets) => Puzzle2dSelectionTargets)) => void;
  puzzle2dGridSnapEnabled: boolean;
  setPuzzle2dGridSnapEnabled: (value: boolean) => void;
  puzzle2dActiveTool: Puzzle2dActiveTool;
  setPuzzle2dActiveTool: (tool: Puzzle2dActiveTool) => void;
  puzzle2dSuggestionOffset: number;
  setPuzzle2dSuggestionOffset: (distance: number) => void;
  /** @emoji 🖌️ Pushes brush candidate rows into play window engagement possibles. */
  notifyBrushCandidates: (payload: Puzzle2dBrushCandidatesPayload) => void;
  /** @emoji 🖌️ Commits brush placement with structural-delete guards and peer sync. */
  commitBrushPlacement: (payload: Puzzle2dBrushPlacePayload) => void;
  /** @emoji 📶 Per-pane LOD select value (`automatic` or a pinned tier). */
  puzzle2dLodModeByPane: Record<Puzzle2dPlayPaneId, Puzzle2dLodModeKind>;
  lodModeForScope: (scopeId: string, pane: Puzzle2dPlayPaneId) => Puzzle2dLodModeKind;
  setPuzzle2dLodModeForPane: (pane: Puzzle2dPlayPaneId, mode: Puzzle2dLodModeKind) => void;
  activeScopeId: string;
  /** @emoji 🗑️ Drops ids from the shared fixture after the canvas emits structural delete events. */
  applyStructuralDelete: (kind: "edge" | "node", id: string) => void;
  /** @emoji 🗑️ Batches canvas structural deletes; ignores ids already absent from the shared fixture. */
  queueStructuralDelete: (kind: "edge" | "node", id: string) => void;
  /** @emoji 🔁 Monotonic epoch bumped on shared fixture graph edits for multi-pane declarative resync. */
  sceneAuthoringEpoch: number;
  /** @emoji ⏯️ When true, play runs layout work on `requestAnimationFrame` (graph packs multiple WASM passes per ~14ms frame; tree one pass per frame). */
  puzzle2dRedrawPlaying: boolean;
  setPuzzle2dRedrawPlaying: (value: boolean) => void;
  puzzle2dRedrawMode: Puzzle2dRedrawModeKind;
  setPuzzle2dRedrawMode: (value: Puzzle2dRedrawModeKind) => void;
  forceLayoutFullIterations: number;
  setForceLayoutFullIterations: (value: number) => void;
  forceLayoutIdealEdgeLength: number;
  setForceLayoutIdealEdgeLength: (value: number) => void;
  forceLayoutGravity: number;
  setForceLayoutGravity: (value: number) => void;
  forceLayoutRepulsionStrength: number;
  setForceLayoutRepulsionStrength: (value: number) => void;
  puzzle2dRedrawPlayMaxItersPerFrame: number;
  setPuzzle2dRedrawPlayMaxItersPerFrame: (value: number) => void;
  puzzle2dRedrawProgressiveEnabled: boolean;
  setPuzzle2dRedrawProgressiveEnabled: (value: boolean) => void;
  puzzle2dRedrawProgressiveAutoStopMs: number;
  setPuzzle2dRedrawProgressiveAutoStopMs: (value: number) => void;
  /** @emoji 🔁 Restarts progressive iteration ramp and auto-stop clock (used when the user drags a node during play). */
  resetPuzzle2dRedrawProgressiveEpoch: () => void;
  /** @emoji 🖱️ Live force-graph play: pins dragged node centers in the fixture and passes them to WASM as locked. */
  notePuzzle2dPlayNodeDragMove: (payload: { readonly id: string; readonly x: number; readonly y: number }) => void;
  /** @emoji 🏁 Clears live force-graph drag pins after {@link Puzzle2dEventMap.nodeDragEnd}. */
  clearPuzzle2dPlayNodeDrag: () => void;
  treeLayoutLayerSpacing: number;
  setTreeLayoutLayerSpacing: (value: number) => void;
  treeLayoutSiblingGap: number;
  setTreeLayoutSiblingGap: (value: number) => void;
  treeLayoutDirection: Puzzle2dHierarchicalTreeDirectionKind;
  setTreeLayoutDirection: (value: Puzzle2dHierarchicalTreeDirectionKind) => void;
  applyPuzzle2dRedrawOnce: () => void;
  applyPuzzle2dRedrawHandlesOnce: () => void;
  puzzle2dRedrawHandlesAfterNodes: boolean;
  setPuzzle2dRedrawHandlesAfterNodes: (value: boolean) => void;
}

interface Puzzle2dPlaySelectionValue {
  selectionIds: Set<string>;
  /** @emoji ✅ Workbench/hierarchy/toolbar: mirror selection to every authoring pane. */
  setSelectionIds: (ids: readonly string[]) => void;
  /** @emoji ✅ Canvas click: React state only (WASM peers already synced on the originating pane). */
  applyCanvasSelection: (ids: readonly string[]) => void;
  preselection: Puzzle2dPreselectSnapshot;
  setPreselection: (snapshot: Puzzle2dPreselectSnapshot) => void;
}

interface Puzzle2dPlayCamerasValue {
  camerasByPane: Record<Puzzle2dPlayPaneId, CameraState>;
  cameraByScope: Record<string, CameraState>;
  /** @emoji 📷 Writes the active pane’s imperative camera into {@link puzzle2dPlayPaneCamerasBaseline}. */
  syncBaselineFromViewportCamera: (cam: CameraState) => void;
  cameraForScope: (scopeId: string, pane: Puzzle2dPlayPaneId) => CameraState;
}

/** @emoji 🌳 Workbench hierarchy bound to play fixture + selection (not static tree snapshots). */
function Puzzle2dPlayHierarchyPanel(): ReactElement {
  const { fixture, hoveredId, hoveredKind, setHierarchyHover } = usePuzzle2dPlayShell();
  const { selectionIds, setSelectionIds } = usePuzzle2dPlaySelection();
  const onHierarchyHover = reactHostPort.useCallback((payload: Puzzle2dHoverPayload) => setHierarchyHover(payload), [setHierarchyHover]);
  const onToggleHidden = reactHostPort.useCallback((graphId: string) => {
    puzzle2dPlayRuntimeRef.current?.commandBus.dispatch(PUZZLE_2D_PLAY_CONTROLLER_ID, "toggleEntityFlag", { graphId, flag: "hidden" });
  }, []);
  const onToggleLocked = reactHostPort.useCallback((graphId: string) => {
    puzzle2dPlayRuntimeRef.current?.commandBus.dispatch(PUZZLE_2D_PLAY_CONTROLLER_ID, "toggleEntityFlag", { graphId, flag: "locked" });
  }, []);
  const sections = reactHostPort.useMemo(() => {
    if (PUZZLE_2D_PLAY_IS_WIRES) {
      return buildWiresPlayHierarchySections(WIRES_PLAY_FIXTURE, fixture, [], {
        omitItemSelection: true,
        onHover: onHierarchyHover,
      }).sections as TreeDataSection[];
    }
    return buildPuzzle2dPlayHierarchySections(fixture, [], undefined, {
      omitItemSelection: true,
      onHover: onHierarchyHover,
      onToggleHidden,
      onToggleLocked,
    }).sections as TreeDataSection[];
  }, [fixture, onHierarchyHover, onToggleHidden, onToggleLocked]);
  const treeSelectedIds = reactHostPort.useMemo(
    () => puzzle2dPlayHierarchyTreeSelectedIdsForFixture(fixture, [...selectionIds]),
    [fixture, selectionIds],
  );
  const treeHighlightedIds = reactHostPort.useMemo(
    () => puzzle2dPlayHierarchyTreeHighlightedIdsForFixture(fixture, hoveredId, hoveredKind),
    [fixture, hoveredId, hoveredKind],
  );
  const onTreeSelectionChange = reactHostPort.useCallback(
    (treeIds: string[]) => {
      const graphIds = treeIds.map(puzzle2dPlayHierarchyGraphIdFromTreeItemIdForPlay).filter((id): id is string => id !== null);
      if (graphIds.length > 0) {
        setSelectionIds(graphIds);
      }
    },
    [setSelectionIds],
  );
  return (
    <Tree
      className="min-h-0 min-w-0 flex-1 overflow-y-auto overflow-x-hidden"
      highlightedIds={treeHighlightedIds}
      onSelectionChange={onTreeSelectionChange}
      sections={sections}
      selectedIds={treeSelectedIds}
      selectionMode="single"
    />
  );
}

class Puzzle2dPlayHierarchyPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: PUZZLE_2D_PLAY_IS_WIRES ? WIRES_PLAY_HIERARCHY_TAB_ID : PUZZLE_2D_PLAY_HIERARCHY_TAB_ID,
      icon: createIconComponent("list-tree"),
      name: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(
        () => {
          const shell = puzzle2dPlayShellRef.current;
          const selection = puzzle2dPlaySelectionRef.current;
          const bus = puzzle2dPlayRuntimeRef.current?.commandBus ?? new CommandBus();
          if (!shell || !selection) {
            const loadingId = PUZZLE_2D_PLAY_IS_WIRES ? "wires-play-hierarchy.loading" : "puzzle-2d-play-hierarchy.loading";
            return [{ id: loadingId, label: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, items: [{ id: "loading", label: "…" }] }];
          }
          const onHierarchyHover = (payload: Puzzle2dHoverPayload) => shell.setHierarchyHover(payload);
          const onToggleHidden = (graphId: string) => bus.dispatch(PUZZLE_2D_PLAY_CONTROLLER_ID, "toggleEntityFlag", { graphId, flag: "hidden" });
          const onToggleLocked = (graphId: string) => bus.dispatch(PUZZLE_2D_PLAY_CONTROLLER_ID, "toggleEntityFlag", { graphId, flag: "locked" });
          const treeNode = PUZZLE_2D_PLAY_IS_WIRES
            ? (buildWiresPlayHierarchySections(WIRES_PLAY_FIXTURE, shell.fixture, [...selection.selectionIds], {
                omitItemSelection: true,
                onHover: onHierarchyHover,
              }) as UiTreeNode)
            : buildPuzzle2dPlayHierarchySections(shell.fixture, [...selection.selectionIds], undefined, {
                omitItemSelection: true,
                onHover: onHierarchyHover,
                onToggleHidden,
                onToggleLocked,
              });
          return uiTreeNodeToTreePanelConfig(
            {
              ...treeNode,
              selectedIds: puzzle2dPlayHierarchyTreeSelectedIdsForFixture(shell.fixture, [...selection.selectionIds]),
              highlightedIds: puzzle2dPlayHierarchyTreeHighlightedIdsForFixture(shell.fixture, shell.hoveredId, shell.hoveredKind),
            },
            bus,
          );
        },
        () => {
          const shell = puzzle2dPlayShellRef.current;
          if (!shell) return [];
          return [...puzzle2dPlayHierarchyTreeHighlightedIdsForFixture(shell.fixture, shell.hoveredId, shell.hoveredKind)];
        },
      ),
    };
  }
}

function Puzzle2dPlayKindsPanel(): ReactElement {
  const { fixture, hoveredId, hoveredKind, setHierarchyHover } = usePuzzle2dPlayShell();
  const kindCatalogs = reactHostPort.useMemo(
    () => puzzle2dFixtureMergedKindCatalogs(fixture),
    [fixture],
  );
  const onKindsHover = reactHostPort.useCallback((payload: Puzzle2dHoverPayload) => setHierarchyHover(payload), [setHierarchyHover]);
  const treeNode = reactHostPort.useMemo(
    () =>
      PUZZLE_2D_PLAY_IS_WIRES
        ? buildWiresPlayKindsTree(WIRES_PLAY_FIXTURE.kindCatalogs)
        : buildPuzzle2dPlayKindsTree(kindCatalogs, {
            onHover: onKindsHover,
            highlightedIds: puzzle2dPlayKindsTreeHighlightedIdsForFixture(fixture, hoveredId, hoveredKind),
          }),
    [fixture, hoveredId, hoveredKind, kindCatalogs, onKindsHover],
  );
  const commandBus = reactHostPort.useMemo(() => new CommandBus(), []);
  return <PlaygroundDeclarativeTree treeNode={treeNode} commandBus={commandBus} />;
}

class Puzzle2dPlayKindsPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: PUZZLE_2D_PLAY_IS_WIRES ? WIRES_PLAY_KINDS_TAB_ID : PUZZLE_2D_PLAY_KINDS_TAB_ID,
      icon: createIconComponent("tags"),
      name: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
      order: 1,
      tree: new CallbackTreePanelDefinition(() => {
        const shell = puzzle2dPlayShellRef.current;
        const bus = puzzle2dPlayRuntimeRef.current?.commandBus ?? new CommandBus();
        if (!shell) {
          const loadingId = PUZZLE_2D_PLAY_IS_WIRES ? "wires-play-kinds.loading" : "puzzle-2d-play-kinds.loading";
          return [{ id: loadingId, label: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, items: [{ id: "loading", label: "…" }] }];
        }
        const treeNode = PUZZLE_2D_PLAY_IS_WIRES
          ? buildWiresPlayKindsTree(WIRES_PLAY_FIXTURE.kindCatalogs)
          : buildPuzzle2dPlayKindsTree(puzzle2dFixtureMergedKindCatalogs(shell.fixture), {
              onHover: (payload) => shell.setHierarchyHover(payload),
              highlightedIds: puzzle2dPlayKindsTreeHighlightedIdsForFixture(shell.fixture, shell.hoveredId, shell.hoveredKind),
            });
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

class Puzzle2dPlayInspectorPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: "puzzle-2d-play-inspector",
      icon: createIconComponent("clipboard-list"),
      name: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const shell = puzzle2dPlayShellRef.current;
        const selection = puzzle2dPlaySelectionRef.current;
        const bus = puzzle2dPlayRuntimeRef.current?.commandBus ?? new CommandBus();
        if (!shell || !selection) {
          return uiTreeNodeToTreePanelConfig(
            uiDeclarativeSectionsToTree([{ type: "section", id: "puzzle-2d-play-inspector.loading", label: "Detail", children: [{ type: "text", value: "…" }] }]),
            bus,
          );
        }
        return uiTreeNodeToTreePanelConfig(buildPuzzle2dPlayInspectorTree(shell.fixture, selection.selectionIds), bus);
      }),
    };
  }
}

const Puzzle2dPlayShellContext = reactHostPort.createContext<Puzzle2dPlayShellValue | null>(null);

const puzzle2dPlayShellRef: { current: Puzzle2dPlayShellValue | null } = { current: null };
const puzzle2dPlaySelectionRef: { current: Puzzle2dPlaySelectionValue | null } = { current: null };
const puzzle2dPlayRuntimeRef: { current: Platform | null } = { current: null };
const puzzle2dPlayShellControllerRef: { current: Puzzle2dPlayShellController | null } = { current: null };

const Puzzle2dPlaySelectionContext = reactHostPort.createContext<Puzzle2dPlaySelectionValue | null>(null);

/** @emoji ✅ Stable canvas selection actions so pane canvases skip re-render on selection-only updates. */
interface Puzzle2dPlayCanvasSelectionActions {
  applyCanvasSelection: (ids: readonly string[]) => void;
}

const Puzzle2dPlayCanvasSelectionContext = reactHostPort.createContext<Puzzle2dPlayCanvasSelectionActions | null>(null);

const Puzzle2dPlayCamerasContext = reactHostPort.createContext<Puzzle2dPlayCamerasValue | null>(null);

const Puzzle2dPlayLodRuntimeContext = reactHostPort.createContext<((pane: Puzzle2dPlayPaneId, lod: Puzzle2dDrawLodKind) => void) | null>(null);

function usePuzzle2dPlayShell(): Puzzle2dPlayShellValue {
  const value = reactHostPort.useContext(Puzzle2dPlayShellContext);
  if (!value) {
    throw new Error("usePuzzle2dPlayShell must be used inside Puzzle2dPlayShellContext.");
  }
  return value;
}

function usePuzzle2dPlaySelection(): Puzzle2dPlaySelectionValue {
  const value = reactHostPort.useContext(Puzzle2dPlaySelectionContext);
  if (!value) {
    throw new Error("usePuzzle2dPlaySelection must be used inside Puzzle2dPlaySelectionContext.");
  }
  return value;
}

function usePuzzle2dPlayCanvasSelection(): Puzzle2dPlayCanvasSelectionActions {
  const value = reactHostPort.useContext(Puzzle2dPlayCanvasSelectionContext);
  if (!value) {
    throw new Error("usePuzzle2dPlayCanvasSelection must be used inside Puzzle2dPlayCanvasSelectionContext.");
  }
  return value;
}

function usePuzzle2dPlayCameras(): Puzzle2dPlayCamerasValue {
  const value = reactHostPort.useContext(Puzzle2dPlayCamerasContext);
  if (!value) {
    throw new Error("usePuzzle2dPlayCameras must be used inside Puzzle2dPlayCamerasContext.");
  }
  return value;
}
// #endregion 🔖ShellContext

// #region 🔖PlayRedrawHelpers
function newPuzzle2dAuthoringId(prefix: string): string {
  if (typeof globalThis.crypto !== "undefined" && typeof globalThis.crypto.randomUUID === "function") {
    return `${prefix}-${globalThis.crypto.randomUUID()}`;
  }
  return `${prefix}-${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 10)}`;
}

/** @emoji 📐 Default node span in px: circle radius = span/2; rectangle width = height = span (40×40). */
const PUZZLE_2D_PLAY_DEFAULT_NODE_SIZE_PX = 40;

const PUZZLE_2D_PLAY_REDRAW_FRAME_BUDGET_MS = 14;

/** @emoji 📈 Force-graph play: iteration budget per inner WASM call ramps from 2 up to `playMax` over `autoStopMs` (or ~3.8s when stop is off). */
function puzzle2dPlayProgressiveForceIters(elapsedMs: number, autoStopMs: number, playMax: number): number {
  const cap = Math.max(4, Math.min(500, Math.round(playMax)));
  const rampWindow = autoStopMs > 0 ? autoStopMs * 0.88 : 3800;
  const t = Math.min(1, elapsedMs / Math.max(100, rampWindow));
  return Math.max(2, Math.round(2 + t * (cap - 2)));
}

/** @emoji 📐 Builds {@link Puzzle2dRedrawLayoutOptions}; force-graph uses relative springs/repulsion only (no viewport gravity anchor). */
function puzzle2dPlayRedrawLayoutOpts(
  pane: Puzzle2dPlayPaneId,
  camerasByPane: Record<Puzzle2dPlayPaneId, CameraState>,
  mode: Puzzle2dRedrawModeKind,
  forceIters: number,
  forceIdealEdge: number,
  forceGravity: number,
  forceRepulsion: number,
  treeLayerSpacing: number,
  treeSiblingGap: number,
  treeDirection: Puzzle2dHierarchicalTreeDirectionKind,
  redrawHandlesAfter: boolean,
  lockedNodeIds?: readonly string[],
): Puzzle2dRedrawLayoutOptions {
  const cam = camerasByPane[pane];
  const cx = cam.x;
  const cy = cam.y;
  const locked = lockedNodeIds?.length ? [...lockedNodeIds] : undefined;
  if (mode === "hierarchical-tree") {
    return {
      centerX: cx,
      centerY: cy,
      hierarchicalTree: {
        direction: treeDirection,
        layerSpacing: Math.max(24, treeLayerSpacing),
        siblingGap: Math.max(0, treeSiblingGap),
      },
      mode: "hierarchical-tree",
      redrawHandlesAfter,
      ...(locked !== undefined ? { lockedNodeIds: locked } : {}),
    };
  }
  const fg: Puzzle2dForceGraphLayoutOptions = {
    gravity: Math.max(0, forceGravity),
    idealEdgeLength: Math.max(8, forceIdealEdge),
    iterations: Math.max(1, Math.min(5000, Math.round(forceIters))),
    repulsionStrength: Math.max(40, Math.min(120, Math.round(forceRepulsion))),
  };
  return {
    forceGraph: fg,
    mode: "force-graph",
    redrawHandlesAfter,
    ...(locked !== undefined ? { lockedNodeIds: locked } : {}),
  };
}

function puzzle2dPlayLiveForceGraphDragState(
  dragAnchors: ReadonlyMap<string, { readonly x: number; readonly y: number }>,
  lockedNodeIds: readonly string[] | undefined,
): Puzzle2dLiveForceGraphDragState | undefined {
  const ids = lockedNodeIds ?? [];
  if (ids.length === 0 && dragAnchors.size === 0) {
    return undefined;
  }
  return { dragAnchors, lockedNodeIds: ids };
}
// #endregion 🔖PlayRedrawHelpers

// #region 🔖SettingsPanel
function buildPuzzle2dPlaySettingsTree(shell: Puzzle2dPlayShellValue): UiTreeNode {
  const redrawChildren: UiNode[] = [
    {
      type: "field",
      id: "puzzle2d.play.settings.redraw.mode",
      label: "Layout kind",
      child: {
        type: "select",
        id: "puzzle-2d-play-redraw-mode",
        value: shell.puzzle2dRedrawMode,
        items: [
          { value: "force-graph", label: "Graph" },
          { value: "hierarchical-tree", label: "Tree" },
        ],
        onChange: puzzle2dPlayCmd("setPuzzle2dRedrawMode"),
      },
    },
    {
      type: "field",
      id: "puzzle2d.play.settings.redraw.handlesAfter",
      label: "Also redraw handles after node redraw",
      child: {
        type: "toggle",
        id: "puzzle-2d-play-redraw-handles-after-nodes",
        iconId: "check",
        pressed: shell.puzzle2dRedrawHandlesAfterNodes,
        onChange: puzzle2dPlayCmd("setPuzzle2dRedrawHandlesAfterNodes"),
      },
    },
    {
      type: "field",
      id: "puzzle2d.play.settings.redraw.progressive",
      label: "Progressive iterations while play is on",
      child: {
        type: "toggle",
        id: "puzzle-2d-play-redraw-progressive",
        iconId: "check",
        pressed: shell.puzzle2dRedrawProgressiveEnabled,
        onChange: puzzle2dPlayCmd("setPuzzle2dRedrawProgressiveEnabled"),
      },
    },
    {
      type: "field",
      id: "puzzle2d.play.settings.redraw.autoStopMs",
      label: "Auto-stop play after (ms, 0 = off)",
      child: {
        type: "slider",
        id: "puzzle-2d-play-slider-redraw-autostop",
        value: shell.puzzle2dRedrawProgressiveAutoStopMs,
        min: 0,
        max: 12000,
        step: 250,
        onChange: puzzle2dPlayCmd("setPuzzle2dRedrawProgressiveAutoStopMs"),
      },
    },
  ];
  if (shell.puzzle2dRedrawMode === "force-graph") {
    redrawChildren.push({
      type: "field",
      id: "puzzle2d.play.settings.redraw.playMaxIters",
      label: "Max iterations per WASM call (play ramp ceiling)",
      child: {
        type: "slider",
        id: "puzzle-2d-play-slider-redraw-play-max-iters",
        value: shell.puzzle2dRedrawPlayMaxItersPerFrame,
        min: 12,
        max: 220,
        step: 2,
        onChange: puzzle2dPlayCmd("setPuzzle2dRedrawPlayMaxItersPerFrame"),
      },
    });
  } else {
    redrawChildren.push({ type: "text", value: "Tree redraw runs once per animation frame while play is on; use auto-stop to end play after a duration." });
  }
  redrawChildren.push({
    type: "button",
    id: "puzzle-2d-play-redraw-nodes",
    iconId: "refresh-cw",
    label: "Redraw nodes",
    command: puzzle2dPlayCmd("applyPuzzle2dRedrawOnce"),
  });
  const sections: UiSectionNode[] = [{ type: "section", id: "puzzle-2d-play-settings.redraw", label: "Redraw", children: redrawChildren }];
  if (shell.puzzle2dRedrawMode === "force-graph") {
    sections.push({
      type: "section",
      id: "puzzle-2d-play-settings.graph",
      label: "Graph",
      children: [
        {
          type: "field",
          id: "puzzle2d.play.settings.force.fullIterations",
          label: "Iterations (apply once)",
          child: {
            type: "slider",
            id: "puzzle-2d-play-slider-force-full-iters",
            value: shell.forceLayoutFullIterations,
            min: 24,
            max: 720,
            step: 4,
            onChange: puzzle2dPlayCmd("setForceLayoutFullIterations"),
          },
        },
        {
          type: "field",
          id: "puzzle2d.play.settings.force.idealEdge",
          label: "Ideal edge (px)",
          child: {
            type: "slider",
            id: "puzzle-2d-play-slider-force-ideal",
            value: shell.forceLayoutIdealEdgeLength,
            min: 20,
            max: 160,
            step: 2,
            onChange: puzzle2dPlayCmd("setForceLayoutIdealEdgeLength"),
          },
        },
        {
          type: "field",
          id: "puzzle2d.play.settings.force.repulsion",
          label: "Repulsion (medium 80, ±40)",
          child: {
            type: "slider",
            id: "puzzle-2d-play-slider-force-repulsion",
            value: shell.forceLayoutRepulsionStrength,
            min: 40,
            max: 120,
            step: 2,
            onChange: puzzle2dPlayCmd("setForceLayoutRepulsionStrength"),
          },
        },
        {
          type: "field",
          id: "puzzle2d.play.settings.force.gravity",
          label: "Gravity",
          child: {
            type: "slider",
            id: "puzzle-2d-play-slider-force-gravity",
            value: shell.forceLayoutGravity,
            min: 0,
            max: 0.05,
            step: 0.002,
            onChange: puzzle2dPlayCmd("setForceLayoutGravity"),
          },
        },
      ],
    });
  } else {
    sections.push({
      type: "section",
      id: "puzzle-2d-play-settings.tree",
      label: "Tree",
      children: [
        {
          type: "field",
          id: "puzzle2d.play.settings.tree.layerSpacing",
          label: "Layer spacing (px)",
          child: {
            type: "slider",
            id: "puzzle-2d-play-slider-tree-layer",
            value: shell.treeLayoutLayerSpacing,
            min: 40,
            max: 280,
            step: 4,
            onChange: puzzle2dPlayCmd("setTreeLayoutLayerSpacing"),
          },
        },
        {
          type: "field",
          id: "puzzle2d.play.settings.tree.siblingGap",
          label: "Sibling gap (px)",
          child: {
            type: "slider",
            id: "puzzle-2d-play-slider-tree-sibling",
            value: shell.treeLayoutSiblingGap,
            min: 0,
            max: 120,
            step: 2,
            onChange: puzzle2dPlayCmd("setTreeLayoutSiblingGap"),
          },
        },
        {
          type: "field",
          id: "puzzle2d.play.settings.tree.direction",
          label: "Direction",
          child: {
            type: "select",
            id: "puzzle-2d-play-tree-direction",
            value: shell.treeLayoutDirection,
            items: [
              { value: "downwards", label: "Downwards" },
              { value: "upwards", label: "Upwards" },
              { value: "right", label: "Right" },
              { value: "left", label: "Left" },
            ],
            onChange: puzzle2dPlayCmd("setTreeLayoutDirection"),
          },
        },
      ],
    });
  }
  sections.push({
    type: "section",
    id: "puzzle-2d-play-settings.handles",
    label: "Redraw handles",
    children: [
      {
        type: "text",
        value: "Each edge uses the straight segment between node centers; handle anchors move to where that segment meets each shape.",
      },
      {
        type: "button",
        id: "puzzle-2d-play-redraw-handles",
        iconId: "refresh-cw",
        label: "Redraw handles",
        command: puzzle2dPlayCmd("applyPuzzle2dRedrawHandlesOnce"),
      },
    ],
  });
  return uiDeclarativeSectionsToTree(sections);
}

registerSidePanelBody(PUZZLE_2D_PLAY_SETTINGS_BODY_KEY, (ctx) => {
  const shell = puzzle2dPlayShellRef.current;
  if (!shell) {
    return uiDeclarativeSectionsToTree([
      { type: "section", id: "puzzle-2d-play-settings.loading", label: "Settings", children: [{ type: "text", value: "…" }] },
    ]);
  }
  return buildPuzzle2dPlaySettingsTree(shell);
});
// #endregion 🔖SettingsPanel

// #region 🔖Scene
// #endregion 🔖Scene

// #region 🔖Panes
/** @emoji 🪟 Captures pointer focus for the active pane (tabs + canvas). */
function Puzzle2dPaneChrome({ children, paneId }: { children: ReactNode; paneId: Puzzle2dPlayPaneId }): ReactElement {
  const { clearHoverForPane, setActivePaneId, setHoverPane } = usePuzzle2dPlayShell();
  return (
    <div
      className="flex h-full min-h-0 w-full flex-col"
      onPointerDownCapture={() => {
        setActivePaneId(paneId);
      }}
      onPointerEnter={() => {
        setHoverPane(paneId);
      }}
      onPointerLeave={(event) => {
        const related = event.relatedTarget;
        if (related instanceof Node && event.currentTarget.contains(related)) {
          return;
        }
        clearHoverForPane(paneId);
      }}
    >
      {children}
    </div>
  );
}

function puzzle2dPlayLodCanvasProps(mode: Puzzle2dLodModeKind): { automaticLod: boolean; lod?: Puzzle2dDrawLodKind } {
  if (mode === PUZZLE_2D_LOD_MODE_AUTOMATIC) {
    return { automaticLod: true };
  }
  return { automaticLod: false, lod: mode };
}

const Puzzle2dPlayPaneCanvas = React.memo(function Puzzle2dPlayPaneCanvas({
  paneId,
  scopeId,
  lodMode,
  showBackgroundMenu,
}: {
  paneId: Puzzle2dPlayPaneId;
  scopeId: string;
  lodMode: Puzzle2dLodModeKind;
  showBackgroundMenu?: boolean;
}): ReactElement {
  const {
    activePaneId,
    activeScopeId,
    patchFixture,
    queueStructuralDelete,
    puzzle2dActiveTool,
    puzzle2dSuggestionOffset,
    puzzle2dGridSnapEnabled,
    sceneAuthoringEpoch,
    puzzle2dRedrawPlaying,
    puzzle2dSelectionMethod,
    puzzle2dSelectionMode,
    puzzle2dSelectionTargets,
    fixture,
    commitBrushPlacement,
    handleCanvasFixtureDrop,
    resetPuzzle2dRedrawProgressiveEpoch,
    notePuzzle2dPlayNodeDragMove,
    clearPuzzle2dPlayNodeDrag,
    hoveredId,
    hoveredKind,
    setHoverForPane,
  } = usePuzzle2dPlayShell();
  const { cameraForScope, syncBaselineFromViewportCamera } = usePuzzle2dPlayCameras();
  const camera = cameraForScope(scopeId, paneId);
  const lodProps = puzzle2dPlayLodCanvasProps(lodMode);
  const reportEffectiveLod = reactHostPort.useContext(Puzzle2dPlayLodRuntimeContext);
  const onLodChange = reactHostPort.useCallback((lod: Puzzle2dDrawLodKind) => reportEffectiveLod?.(paneId, lod), [paneId, reportEffectiveLod]);
  const { applyCanvasSelection } = usePuzzle2dPlayCanvasSelection();
  const { preselection: jackPreselection } = usePuzzle2dPlaySelection();
  const puzzle2dShellCtrl = puzzle2dPlayRuntimeRef.current?.getActiveApp()?.controller as Puzzle2dPlayShellController | undefined;
  const jackBridgeEpoch = reactHostPort.useSyncExternalStore(
    (listener) => puzzle2dShellCtrl?.subscribeSnapshot(listener) ?? (() => {}),
    () => (puzzle2dShellCtrl?.getHoverEpoch() ?? 0) + (puzzle2dShellCtrl?.getSelectEpoch() ?? 0),
    () => 0,
  );
  void jackBridgeEpoch;
  const jackPreselect = reactHostPort.useMemo((): Puzzle2dPreselectSnapshot => {
    const highlighted = puzzle2dShellCtrl?.getGraphHighlightedNodeIds() ?? [];
    return highlighted.length ? { ids: [], removedIds: [...highlighted] } : jackPreselection;
  }, [jackBridgeEpoch, jackPreselection, puzzle2dShellCtrl]);
  const onSelect = reactHostPort.useCallback(
    (snapshot: Puzzle2dSelectionSnapshot) => {
      applyCanvasSelection(snapshot.ids);
      puzzle2dPlayShellControllerRef.current?.run("setGraphSelect", { ids: [...snapshot.ids] });
    },
    [applyCanvasSelection],
  );
  reactHostPort.useEffect(() => {
    puzzle2dSelectionActionsRef.current = {
      toggleHidden: (value) => puzzle2dPlayRuntimeRef.current?.commandBus.dispatch(PUZZLE_2D_PLAY_CONTROLLER_ID, "setSelectionFlag", { flag: "hidden", value }),
      toggleLocked: (value) => puzzle2dPlayRuntimeRef.current?.commandBus.dispatch(PUZZLE_2D_PLAY_CONTROLLER_ID, "setSelectionFlag", { flag: "locked", value }),
      deleteSelection: () => puzzle2dPlayRuntimeRef.current?.commandBus.dispatch(PUZZLE_2D_PLAY_CONTROLLER_ID, "deleteSelection"),
      duplicateSelection: () => puzzle2dPlayRuntimeRef.current?.commandBus.dispatch(PUZZLE_2D_PLAY_CONTROLLER_ID, "duplicateSelection"),
      selectSameKind: () => puzzle2dPlayRuntimeRef.current?.commandBus.dispatch(PUZZLE_2D_PLAY_CONTROLLER_ID, "selectSameKind"),
    };
  }, []);
  const demoNodeId = fixture.nodes[0]?.id;
  const demoEdgeId = fixture.edges[0]?.id;
  const kindCompatibility = reactHostPort.useMemo(() => puzzle2dFixtureMetaKindCompatibility(fixture), [fixture]);
  const sceneMarkers = reactHostPort.useMemo(
    () =>
      puzzle2dFixtureSceneMarkers(fixture, {
        nodeContextMenuForId: (id) => (id === demoNodeId ? puzzle2dPlayDemoNodeContextMenu : undefined),
        edgeContextMenuForId: (id) => (id === demoEdgeId ? puzzle2dPlayDemoEdgeContextMenu : undefined),
      }),
    [demoEdgeId, demoNodeId, fixture],
  );
  const declarativeSceneDescriptor = reactHostPort.useMemo(() => buildPuzzle2dSceneDescriptorFromFixture(fixture), [fixture]);
  const acceptCanvasStructuralDeleteRef = reactHostPort.useRef(false);
  reactHostPort.useEffect(() => {
    const frame = requestAnimationFrame(() => {
      acceptCanvasStructuralDeleteRef.current = true;
    });
    return () => {
      cancelAnimationFrame(frame);
      acceptCanvasStructuralDeleteRef.current = false;
    };
  }, []);
  const onCanvasDelete = reactHostPort.useCallback(
    (payload: Puzzle2dStructureDeletePayload) => {
      if (!puzzle2dPlayForwardsCanvasStructuralDelete(payload.kind, acceptCanvasStructuralDeleteRef.current)) {
        return;
      }
      queueStructuralDelete(payload.kind, payload.id);
    },
    [queueStructuralDelete],
  );
  const onCanvasDrag = reactHostPort.useCallback(
    (payload: { id: string; x: number; y: number }) => {
      notePuzzle2dPlayNodeDragMove(payload);
    },
    [notePuzzle2dPlayNodeDragMove],
  );
  const onCanvasDragEnd = reactHostPort.useCallback(
    (payload: { moves: Array<{ id: string; x: number; y: number }> }) => {
      clearPuzzle2dPlayNodeDrag();
      if (payload.moves.length === 0) {
        return;
      }
      const byId = new Map(payload.moves.map((move) => [move.id, move]));
      patchFixture((prev) => ({
        ...prev,
        nodes: prev.nodes.map((node) => {
          const move = byId.get(node.id);
          return move ? { ...node, x: move.x, y: move.y } : node;
        }),
      }));
    },
    [clearPuzzle2dPlayNodeDrag, patchFixture],
  );
  const { notifyBrushCandidates } = usePuzzle2dPlayShell();
  const onCanvasHover = reactHostPort.useCallback(
    (payload: Puzzle2dHoverPayload) => {
      setHoverForPane(paneId, payload);
      puzzle2dPlayShellControllerRef.current?.run("setGraphHover", { id: payload.id });
    },
    [paneId, setHoverForPane],
  );
  const isWiresPlay = PUZZLE_2D_PLAY_IS_WIRES;
  const resolvedSelectionTargets = isWiresPlay ? { nodes: true, edges: true, handles: false } : puzzle2dSelectionTargets;
  return (
    <Puzzle2dPaneChrome paneId={paneId}>
      <Puzzle2dCanvas
        {...lodProps}
        graphPortMode={isWiresPlay ? "normal" : undefined}
        declarativeSceneDescriptor={declarativeSceneDescriptor}
        onLodChange={onLodChange}
        camera={camera}
        className="min-h-0 flex-1"
        contextMenu={showBackgroundMenu ? puzzle2dPlayCanvasBackgroundMenu : undefined}
        fixtureDragDrop={!isWiresPlay}
        activeTool={puzzle2dActiveTool}
        suggestionOffset={puzzle2dSuggestionOffset}
        brushNodeSize={DEFAULT_PUZZLE_2D_BRUSH_NODE_SIZE_PX}
        gridSnapEnabled={puzzle2dGridSnapEnabled}
        kindCatalogs={PUZZLE_2D_PLAY_DEFAULT_KIND_CATALOGS}
        kindCompatibility={isWiresPlay ? undefined : kindCompatibility}
        onCamera={activeScopeId === scopeId ? syncBaselineFromViewportCamera : undefined}
        onDelete={onCanvasDelete}
        onDrag={onCanvasDrag}
        onDragEnd={onCanvasDragEnd}
        onFixtureDrop={isWiresPlay ? undefined : (d) => handleCanvasFixtureDrop(paneId, d)}
        onSelect={onSelect}
        onBrushCandidates={notifyBrushCandidates}
        preselection={jackPreselect}
        hoveredId={hoveredId}
        kindHover={hoveredKind}
        onHover={onCanvasHover}
        sceneAuthoringEpoch={sceneAuthoringEpoch}
        selectionMethod={puzzle2dSelectionMethod}
        selectionMode={puzzle2dSelectionMode}
        selectionTargets={resolvedSelectionTargets}
      >
        {sceneMarkers}
      </Puzzle2dCanvas>
    </Puzzle2dPaneChrome>
  );
});

function Puzzle2dPlayPaneSurfaceHost({ node }: { readonly node: UiPuzzle2dHostSurfaceNode }): ReactElement {
  if (node.controllerId !== PUZZLE_2D_PLAY_CONTROLLER_ID || node.surfaceId !== PUZZLE_2D_PLAY_SURFACE_ID) {
    return <div className="p-2 text-xs text-muted-foreground">Invalid puzzle 2d surface binding</div>;
  }
  const shellInstance = useShellWindowInstance();
  const paneId = (shellInstance?.windowKindId ?? node.paneId) as Puzzle2dPlayPaneId;
  const scopeId = shellWindowScopeId(shellInstance, paneId);
  const { lodModeForScope } = usePuzzle2dPlayShell();
  const lodMode = lodModeForScope(scopeId, paneId);
  return <Puzzle2dPlayPaneCanvas paneId={paneId} scopeId={scopeId} lodMode={lodMode} showBackgroundMenu={paneId === "2d-overview"} />;
}

function Puzzle2dPlayJackSurfaceHost({ node: _node }: { readonly node: import("@semio-tech/framework-platform-core").UiWriterHostSurfaceNode }): ReactElement {
  const ctrl = puzzle2dPlayShellControllerRef.current ?? undefined;
  void ctrl?.getHoverEpoch();
  void ctrl?.getSelectEpoch();
  const document = ctrl?.getWriterDocumentJack() ?? createWriterDocument({ id: "puzzle-2d-jack", languageId: "jack", text: "" });
  const onHoverChange = reactHostPort.useCallback((offset: number | null) => {
    puzzle2dPlayShellControllerRef.current?.run("setJackHover", { offset });
  }, []);
  const onSelectionChange = reactHostPort.useCallback((range: { start: number; end: number }) => {
    puzzle2dPlayShellControllerRef.current?.run("setJackSelect", range);
  }, []);
  return (
    <WriterCanvas
      document={document}
      className="h-full"
      onHoverChange={onHoverChange}
      onSelectionChange={onSelectionChange}
      externalHoverOccurrences={ctrl?.getJackHoverOccurrences()}
      externalHoverOccurrencesSignal={ctrl?.getHoverEpoch()}
      externalSelectionOccurrences={ctrl?.getJackSelectOccurrences()}
      externalSelectionOccurrencesSignal={ctrl?.getSelectEpoch()}
    />
  );
}

function Puzzle2dPlayCompiledDagSurfaceHost({ node: _node }: { readonly node: import("@semio-tech/framework-platform-core").UiWriterHostSurfaceNode }): ReactElement {
  const ctrl = puzzle2dPlayShellControllerRef.current ?? undefined;
  const [revision, setRevision] = reactHostPort.useState(0);
  reactHostPort.useEffect(() => ctrl?.subscribeSnapshot(() => setRevision((value) => value + 1)) ?? undefined, [ctrl]);
  const document = reactHostPort.useMemo(
    () => ctrl?.getWriterDocumentCompiledDag() ?? createWriterDocument({ id: "puzzle-2d-compiled-dag", languageId: "wire", text: "" }),
    [ctrl, revision],
  );
  return <WriterCanvas document={document} className="h-full min-h-0" />;
}

let puzzle2dPlayChromeRegistered = false;

/** @emoji 🧊 Registers puzzle 2d play surface host, window bodies, and tab icons (called from `@semio-tech/framework-playground-renderer-react`). */
export function registerPuzzle2dPlaySurfaceHosts(): void {
  if (puzzle2dPlayChromeRegistered) return;
  puzzle2dPlayChromeRegistered = true;
  registerUiPuzzle2dSurfaceHost(PUZZLE_2D_PLAY_SURFACE_ID, Puzzle2dPlayPaneSurfaceHost);
  registerUiWriterSurfaceHost(PUZZLE_2D_PLAY_SURFACE_ID_JACK, Puzzle2dPlayJackSurfaceHost);
  registerUiWriterSurfaceHost(PUZZLE_2D_PLAY_SURFACE_ID_COMPILED_DAG, Puzzle2dPlayCompiledDagSurfaceHost);
  registerWindowBody(PUZZLE_2D_PLAY_BODY_KEY_JACK, () =>
    buildWriterWindowBody(PUZZLE_2D_PLAY_SURFACE_ID_JACK, PUZZLE_2D_PLAY_CONTROLLER_ID, PUZZLE_2D_PLAY_WINDOW_KIND_JACK));
  registerWindowBody(PUZZLE_2D_PLAY_BODY_KEY_OVERVIEW, buildPuzzle2dPlayOverviewDeclarativeBody);
  registerWindowBody(PUZZLE_2D_PLAY_BODY_KEY_DETAIL, buildPuzzle2dPlayDetailDeclarativeBody);
  registerWindowBody(PUZZLE_2D_PLAY_BODY_KEY_SELECTION, buildPuzzle2dPlaySelectionDeclarativeBody);
  registerTabIcon(PUZZLE_2D_PLAY_ICON_KINDS, "tags");
  registerTabIcon("puzzle.2d-play.icon.inspector", "clipboard-list");
  registerTabIcon("puzzle.2d-play.icon.settings", "settings");
}
// #endregion 🔖Panes

// #region 🔖SidePanels
function findNode(fixture: Puzzle2dFixture, id: string): Puzzle2dFixtureNode | undefined {
  return fixture.nodes.find((n) => n.id === id);
}

function findEdge(fixture: Puzzle2dFixture, id: string): Puzzle2dFixtureEdge | undefined {
  return fixture.edges.find((e) => e.id === id);
}

function findHandleOwner(fixture: Puzzle2dFixture, handleId: string): { node: Puzzle2dFixtureNode; handleId: string } | undefined {
  for (const node of fixture.nodes) {
    if (node.handles.some((h) => h.id === handleId)) {
      return { handleId, node };
    }
  }
  return undefined;
}

function findHandle(fixture: Puzzle2dFixture, handleId: string): Puzzle2dFixtureHandle | undefined {
  for (const node of fixture.nodes) {
    const h = node.handles.find((x) => x.id === handleId);
    if (h) {
      return h;
    }
  }
  return undefined;
}

function listHandleIds(fixture: Puzzle2dFixture): string[] {
  const out: string[] = [];
  for (const node of fixture.nodes) {
    for (const h of node.handles) {
      out.push(h.id);
    }
  }
  out.sort((a, b) => a.localeCompare(b));
  return out;
}

/** @emoji 🎯 Normalizes θ to `[0, 2π)`. */
function normalizeAngleRad(t: number): number {
  const twoPi = Math.PI * 2;
  let x = t % twoPi;
  if (x < 0) {
    x += twoPi;
  }
  return x;
}

function puzzle2dInspectorKindSelectItems(
  catalogRows: readonly { readonly id: string; readonly name: string }[] | undefined,
  currentKindIds: readonly string[],
  labelForOrphan: (kindId: string) => string,
): readonly { readonly value: string; readonly label: string }[] {
  const byId = new Map(puzzle2dPlayKindCatalogSelectItems(catalogRows).map((row) => [row.value, row] as const));
  for (const kindId of currentKindIds) {
    const trimmed = kindId.trim();
    if (trimmed !== "" && !byId.has(trimmed)) {
      byId.set(trimmed, { value: trimmed, label: labelForOrphan(trimmed) });
    }
  }
  return [...byId.values()].sort((a, b) => a.label.localeCompare(b.label));
}

export function buildPuzzle2dPlayInspectorTree(fixture: Puzzle2dFixture, selectionIds: ReadonlySet<string>): UiTreeNode {
  const kindCatalogs = puzzle2dFixtureMergedKindCatalogs(fixture);
  const { nodeIds, handleIds, edgeIds, unknownIds } = classifyPuzzle2dPlayInspectorSelection(fixture, selectionIds);
  const sections: UiSectionNode[] = [];
  if (nodeIds.length === 0 && handleIds.length === 0 && edgeIds.length === 0 && unknownIds.length === 0) {
    sections.push({
      type: 'section',
      id: 'puzzle-2d-play-inspector.empty',
      label: 'Detail',
      children: [{
        type: 'text',
        value: PUZZLE_2D_PLAY_IS_WIRES
          ? 'No selection. Click the graph or pick an identity or relationship in the hierarchy.'
          : 'No selection. Click the graph or pick a row in the hierarchy.',
      }],
    });
    return uiDeclarativeSectionsToTree(sections);
  }
  if (nodeIds.length > 0) {
    const targets = nodeIds.map((id) => findNode(fixture, id)).filter((n): n is Puzzle2dFixtureNode => Boolean(n));
    const textValues = targets.map((n) => puzzle2dFixtureNodeCaption(n) ?? '');
    const textUniform = uiInspectorAllEqual(textValues);
    const nodeKinds = targets.map((n) => n.nodeKind ?? '');
    const nodeKindUniform = uiInspectorAllEqual(nodeKinds);
    const iconKinds = targets.map((n) => n.iconKind ?? '');
    const iconKindUniform = uiInspectorAllEqual(iconKinds);
    const xs = targets.map((n) => n.x);
    const ys = targets.map((n) => n.y);
    const xUniform = uiInspectorAllEqual(xs);
    const yUniform = uiInspectorAllEqual(ys);
    sections.push({
      type: 'section',
      id: 'puzzle-2d-play-inspector-nodes',
      label: PUZZLE_2D_PLAY_IS_WIRES ? (nodeIds.length === 1 ? 'Identity' : 'Identities') : puzzle2dPlayInspectorKindSectionLabel('node', nodeIds.length),
      children: [
        {
          type: 'field',
          id: 'puzzle-2d-play.inspector.node.name',
          label: PUZZLE_2D_PLAY_IS_WIRES ? 'Label' : 'Name',
          child: {
            type: 'input',
            id: 'puzzle-2d-play.inspector.node.name.input',
            inputKind: 'text',
            value: textUniform ? (textValues[0] ?? '') : '',
            placeholder: textUniform ? undefined : 'Mixed',
            onChange: puzzle2dPlayCmd('patchInspectorNodes', { ids: nodeIds, field: 'text' }),
          },
        },
        {
          type: 'field',
          id: 'puzzle-2d-play.inspector.node.kind',
          label: PUZZLE_2D_PLAY_IS_WIRES ? 'Identity kind' : 'Node kind',
          child: {
            type: 'select',
            id: 'puzzle-2d-play.inspector.node.kind.select',
            value: nodeKindUniform ? (nodeKinds[0] ?? '') : '',
            placeholder: nodeKindUniform ? 'kind' : 'Mixed',
            items: puzzle2dInspectorKindSelectItems(kindCatalogs.nodes, nodeKinds, (kindId) => puzzle2dNodeKindOverlayLabel(kindId, kindCatalogs)),
            onChange: puzzle2dPlayCmd('patchInspectorNodes', { ids: nodeIds, field: 'nodeKind' }),
          },
        },
        {
          type: 'field',
          id: 'puzzle-2d-play.inspector.node.icon',
          label: 'Icon',
          child: {
            type: 'iconSelect',
            id: 'puzzle-2d-play.inspector.node.icon.selector',
            value: iconKindUniform ? (iconKinds[0] ?? '') : '',
            uniform: iconKindUniform,
            classifierKind: 'puzzle2d',
            onChange: puzzle2dPlayCmd('patchInspectorNodes', { ids: nodeIds, field: 'iconKind' }),
          },
        },
        {
          type: 'field',
          id: 'puzzle-2d-play.inspector.node.x',
          label: 'x',
          child: {
            type: 'numberStepper',
            id: 'puzzle-2d-play.inspector.node.x.stepper',
            value: xUniform ? xs[0]! : Number.NaN,
            step: 1,
            uniform: xUniform,
            onAbsolute: puzzle2dPlayCmd('patchInspectorNodes', { ids: nodeIds, field: 'x' }),
            onDelta: puzzle2dPlayCmd('patchInspectorNodes', { ids: nodeIds, field: 'xDelta' }),
          },
        },
        {
          type: 'field',
          id: 'puzzle-2d-play.inspector.node.y',
          label: 'y',
          child: {
            type: 'numberStepper',
            id: 'puzzle-2d-play.inspector.node.y.stepper',
            value: yUniform ? ys[0]! : Number.NaN,
            step: 1,
            uniform: yUniform,
            onAbsolute: puzzle2dPlayCmd('patchInspectorNodes', { ids: nodeIds, field: 'y' }),
            onDelta: puzzle2dPlayCmd('patchInspectorNodes', { ids: nodeIds, field: 'yDelta' }),
          },
        },
      ],
    });
  }
  if (handleIds.length > 0) {
    const handles = handleIds.map((id) => findHandle(fixture, id)).filter((h): h is Puzzle2dFixtureHandle => Boolean(h));
    const handleKinds = handles.map((h) => h.handleKind);
    const handleKindUniform = uiInspectorAllEqual(handleKinds);
    const angles = handles.map((h) => h.angle);
    const angleUniform = uiInspectorAllEqual(angles);
    const angleValue = angleUniform ? angles[0]! : 0;
    const radii = handles.map((h) => h.radius ?? 8);
    const radiusUniform = uiInspectorAllEqual(radii);
    const iconKinds = handles.map((h) => h.iconKind ?? '');
    const iconKindUniform = uiInspectorAllEqual(iconKinds);
    const ringParentNodes = handles.map((h) => findHandleOwner(fixture, h.id)?.node).filter((n): n is Puzzle2dFixtureNode => Boolean(n));
    const ringParentShapes = ringParentNodes.map((n) => n.shape ?? 'circle');
    const ringParentShapeUniform = uiInspectorAllEqual(ringParentShapes);
    const ringParentNode = ringParentShapeUniform ? ringParentNodes[0] : undefined;
    const ringEnabled = angleUniform && ringParentNode !== undefined;
    const ringOrbT = ringEnabled ? puzzle2dHandleAngleToRingT(ringParentNode, angleValue) : 0;
    const handleFields: UiNode[] = [
      {
        type: 'field',
        id: 'puzzle-2d-play.inspector.handle.kind',
        label: 'Handle kind',
        child: {
          type: 'select',
          id: 'puzzle-2d-play.inspector.handle.kind.select',
          value: handleKindUniform ? (handleKinds[0] ?? '') : '',
          placeholder: handleKindUniform ? 'kind' : 'Mixed',
          items: puzzle2dInspectorKindSelectItems(kindCatalogs.handles, handleKinds, (kindId) => puzzle2dHandleKindOverlayLabel(kindId, kindCatalogs)),
          onChange: puzzle2dPlayCmd('patchInspectorHandles', { ids: handleIds, field: 'handleKind' }),
        },
      },
      {
        type: 'field',
        id: 'puzzle-2d-play.inspector.handle.t.ring',
        label: 't',
        child: {
          type: 'ring',
          id: 'puzzle-2d-play.inspector.handle.t.ring.control',
          orbId: 'angle',
          t: ringOrbT,
          disabled: !ringEnabled,
          onChange: puzzle2dPlayCmd('patchInspectorHandles', { ids: handleIds, field: 'ringT', parentNodeId: ringParentNode?.id }),
        },
      },
      {
        type: 'field',
        id: 'puzzle-2d-play.inspector.handle.t',
        label: 't (rad)',
        child: {
          type: 'numberStepper',
          id: 'puzzle-2d-play.inspector.handle.t.stepper',
          value: angleUniform ? angleValue : Number.NaN,
          step: 0.05,
          uniform: angleUniform,
          onAbsolute: puzzle2dPlayCmd('patchInspectorHandles', { ids: handleIds, field: 'angle' }),
          onDelta: puzzle2dPlayCmd('patchInspectorHandles', { ids: handleIds, field: 'angleDelta' }),
        },
      },
      {
        type: 'field',
        id: 'puzzle-2d-play.inspector.handle.radius',
        label: 'Hit radius',
        child: {
          type: 'numberStepper',
          id: 'puzzle-2d-play.inspector.handle.radius.stepper',
          value: radiusUniform ? radii[0]! : Number.NaN,
          step: 1,
          uniform: radiusUniform,
          onAbsolute: puzzle2dPlayCmd('patchInspectorHandles', { ids: handleIds, field: 'radius' }),
          onDelta: puzzle2dPlayCmd('patchInspectorHandles', { ids: handleIds, field: 'radiusDelta' }),
        },
      },
      {
        type: 'field',
        id: 'puzzle-2d-play.inspector.handle.icon',
        label: 'Icon',
        child: {
          type: 'iconSelect',
          id: 'puzzle-2d-play.inspector.handle.icon.selector',
          value: iconKindUniform ? (iconKinds[0] ?? '') : '',
          uniform: iconKindUniform,
          classifierKind: 'puzzle2d',
          onChange: puzzle2dPlayCmd('patchInspectorHandles', { ids: handleIds, field: 'iconKind' }),
        },
      },
    ];
    sections.push({
      type: 'section',
      id: 'puzzle-2d-play-inspector-handles',
      label: puzzle2dPlayInspectorKindSectionLabel('handle', handleIds.length),
      children: handleFields,
    });
  }
  if (edgeIds.length > 0) {
    const edges = edgeIds.map((id) => findEdge(fixture, id)).filter((e): e is Puzzle2dFixtureEdge => Boolean(e));
    const sources = edges.map((e) => e.source);
    const targets = edges.map((e) => e.target);
    const sourceUniform = uiInspectorAllEqual(sources);
    const targetUniform = uiInspectorAllEqual(targets);
    const edgeKinds = edges.map((e) => e.edgeKind ?? '');
    const edgeKindUniform = uiInspectorAllEqual(edgeKinds);
    const handleOptions = PUZZLE_2D_PLAY_IS_WIRES ? fixture.nodes.map((node) => node.id) : listHandleIds(fixture);
    const endpointItems = handleOptions.map((hid) => ({
      value: hid,
      label: PUZZLE_2D_PLAY_IS_WIRES ? (wiresPlayIdentityLabelForNodeId(hid) ?? hid) : puzzle2dFixtureHandleEndpointDisplayLabel(hid, fixture, kindCatalogs),
    }));
    const edgeFields: UiNode[] = [];
    if (PUZZLE_2D_PLAY_IS_WIRES) {
      const wiresRelationshipKinds = edges.map((edge) => wiresPlayRelationshipKindDisplayName(edge.id) ?? '');
      const wiresRelationshipKindUniform = uiInspectorAllEqual(wiresRelationshipKinds);
      edgeFields.push({
        type: 'field',
        id: 'puzzle-2d-play.inspector.edge.relationship-kind',
        label: 'Relationship kind',
        child: { type: 'text', value: wiresRelationshipKindUniform ? (wiresRelationshipKinds[0] ?? '') : 'Mixed' },
      });
    } else {
      edgeFields.push({
        type: 'field',
        id: 'puzzle-2d-play.inspector.edge.kind',
        label: 'Edge kind',
        child: {
          type: 'select',
          id: 'puzzle-2d-play.inspector.edge.kind.select',
          value: edgeKindUniform ? (edgeKinds[0] ?? '') : '',
          placeholder: edgeKindUniform ? 'kind' : 'Mixed',
          items: puzzle2dInspectorKindSelectItems(kindCatalogs.edges, edgeKinds, (kindId) => puzzle2dEdgeKindOverlayLabel(kindId, kindCatalogs)),
          onChange: puzzle2dPlayCmd('patchInspectorEdges', { ids: edgeIds, field: 'edgeKind' }),
        },
      });
    }
    edgeFields.push(
      {
        type: 'field',
        id: 'puzzle-2d-play.inspector.edge.source',
        label: PUZZLE_2D_PLAY_IS_WIRES ? 'From identity' : 'Source',
        child: {
          type: 'select',
          id: 'puzzle-2d-play.inspector.edge.source.select',
          value: sourceUniform ? (sources[0] ?? '') : '',
          placeholder: sourceUniform ? undefined : 'Mixed',
          items: endpointItems,
          onChange: puzzle2dPlayCmd('patchInspectorEdges', { ids: edgeIds, field: 'source' }),
        },
      },
      {
        type: 'field',
        id: 'puzzle-2d-play.inspector.edge.target',
        label: PUZZLE_2D_PLAY_IS_WIRES ? 'To identity' : 'Target',
        child: {
          type: 'select',
          id: 'puzzle-2d-play.inspector.edge.target.select',
          value: targetUniform ? (targets[0] ?? '') : '',
          placeholder: targetUniform ? undefined : 'Mixed',
          items: endpointItems,
          onChange: puzzle2dPlayCmd('patchInspectorEdges', { ids: edgeIds, field: 'target' }),
        },
      },
    );
    sections.push({
      type: 'section',
      id: 'puzzle-2d-play-inspector-edges',
      label: PUZZLE_2D_PLAY_IS_WIRES ? (edgeIds.length === 1 ? 'Relationship' : 'Relationships') : puzzle2dPlayInspectorKindSectionLabel('edge', edgeIds.length),
      children: edgeFields,
    });
  }
  if (unknownIds.length > 0) {
    sections.push({
      type: 'section',
      id: 'puzzle-2d-play-inspector-unknown',
      label: 'Selection',
      children: [{ type: 'text', value: unknownIds.map((id) => puzzle2dFixtureObjectDisplayLabel(id, fixture, kindCatalogs)).join(', ') }],
    });
  }
  return uiDeclarativeSectionsToTree(sections);
}

function classifyPuzzle2dPlayInspectorSelection(fixture: Puzzle2dFixture, selectionIds: ReadonlySet<string>): {
  readonly nodeIds: readonly string[];
  readonly handleIds: readonly string[];
  readonly edgeIds: readonly string[];
  readonly unknownIds: readonly string[];
} {
  const ids = [...selectionIds].sort((a, b) => a.localeCompare(b));
  const nodeIds: string[] = [];
  const handleIds: string[] = [];
  const edgeIds: string[] = [];
  const unknownIds: string[] = [];
  for (const id of ids) {
    if (findNode(fixture, id)) {
      nodeIds.push(id);
    } else if (findEdge(fixture, id)) {
      edgeIds.push(id);
    } else if (findHandleOwner(fixture, id)) {
      handleIds.push(id);
    } else {
      unknownIds.push(id);
    }
  }
  return { nodeIds, handleIds, edgeIds, unknownIds };
}

// #endregion 🔖SidePanels

// #region 🔖Layout
// #endregion 🔖Layout

interface Puzzle2dPlayRedrawLoopSnapshot {
  activePaneId: Puzzle2dPlayPaneId;
  puzzle2dRedrawHandlesAfterNodes: boolean;
  puzzle2dRedrawProgressiveAutoStopMs: number;
  puzzle2dRedrawProgressiveEnabled: boolean;
  puzzle2dRedrawPlayMaxItersPerFrame: number;
  camerasByPane: Record<Puzzle2dPlayPaneId, CameraState>;
  forceLayoutGravity: number;
  forceLayoutIdealEdgeLength: number;
  forceLayoutRepulsionStrength: number;
  mode: Puzzle2dRedrawModeKind;
  treeLayoutDirection: Puzzle2dHierarchicalTreeDirectionKind;
  treeLayoutLayerSpacing: number;
  treeLayoutSiblingGap: number;
}

// #region 🔖Entrypoint
const initialFixture = clonePuzzle2dFixture(puzzle2dPlayResolvedDefaultFixture());

const PUZZLE_2D_PLAY_NAVBAR_EXAMPLE_OPTIONS = PUZZLE_2D_PLAY_IS_WIRES
  ? WIRES_PLAY_EXAMPLE_OPTIONS
  : [...PUZZLE_2D_PLAY_EXAMPLE_OPTIONS, { id: WIRES_PLAY_EXAMPLE_METABOLISM_ID, label: "Metabolism (WIRES)" }];

const PUZZLE_2D_PLAY_NAVBAR_EXAMPLE_DEFAULT_ID = playgroundResolvedExampleId(
  PUZZLE_2D_PLAY_IS_WIRES ? WIRES_PLAY_EXAMPLE_METABOLISM_ID : PUZZLE_2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID,
);

function puzzle2dPlayFixtureForNavbarId(fixtureId: string): Puzzle2dFixture {
  if (isPlaygroundNoExampleId(fixtureId)) {
    return clonePuzzle2dFixture(PUZZLE_2D_PLAY_EMPTY_FIXTURE);
  }
  if (fixtureId === WIRES_PLAY_EXAMPLE_METABOLISM_ID) {
    return clonePuzzle2dFixture(WIRES_PLAY_DEFAULT_FIXTURE);
  }
  if (fixtureId === PUZZLE_2D_PLAY_EXAMPLE_NAKAGIN_ID || fixtureId === PUZZLE_2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID) {
    return clonePuzzle2dFixture(puzzle2dPlayFixtureForId(fixtureId));
  }
  return clonePuzzle2dFixture(PUZZLE_2D_PLAY_DEFAULT_FIXTURE);
}

function Puzzle2dPlayInner({
  puzzle2dRuntime,
  playgroundKeybindings,
}: {
  readonly puzzle2dRuntime: Platform;
  readonly playgroundKeybindings?: readonly import("@semio-tech/framework-playground-core").PlaygroundKeybinding[];
}): ReactElement {
  const [activeExampleId, setActiveExampleId] = reactHostPort.useState(PUZZLE_2D_PLAY_NAVBAR_EXAMPLE_DEFAULT_ID);
  const [fixture, setFixtureState] = reactHostPort.useState<Puzzle2dFixture>(() => clonePuzzle2dFixture(initialFixture));
  const fixtureRef = reactHostPort.useRef<Puzzle2dFixture>(fixture);
  fixtureRef.current = fixture;
  const catalogRawFixtureRef = reactHostPort.useRef<unknown | undefined>(
    puzzle2dPlayRawFixtureJsonForNavbarId(PUZZLE_2D_PLAY_NAVBAR_EXAMPLE_DEFAULT_ID),
  );
  const triptychCamerasForFixture = reactHostPort.useCallback((next: Puzzle2dFixture) => {
    return triptychCamerasFromFixture(next, catalogRawFixtureRef.current);
  }, []);
  const [puzzle2dPlayPaneCamerasBaseline, setPuzzle2dPlayPaneCamerasBaseline] = reactHostPort.useState<Record<Puzzle2dPlayPaneId, CameraState>>(() => puzzle2dPlayInitialCameras());
  const puzzle2dPlayPaneCamerasBaselineRef = reactHostPort.useRef(puzzle2dPlayPaneCamerasBaseline);
  puzzle2dPlayPaneCamerasBaselineRef.current = puzzle2dPlayPaneCamerasBaseline;
  const [activeScopeId, setActiveScopeId] = reactHostPort.useState("2d-overview");
  const activeScopeIdRef = reactHostPort.useRef(activeScopeId);
  activeScopeIdRef.current = activeScopeId;
  const activePaneId = puzzle2dPlayPaneFromShellWindowId(activeScopeId) ?? "2d-overview";
  const activePaneIdRef = reactHostPort.useRef(activePaneId);
  activePaneIdRef.current = activePaneId;
  const [cameraByScope, setCameraByScope] = reactHostPort.useState<Record<string, CameraState>>({});
  const [selectionIds, setSelectionIdsState] = reactHostPort.useState<Set<string>>(() => selectionSeedForFixture(initialFixture));
  const [preselection, setPreselection] = reactHostPort.useState<Puzzle2dPreselectSnapshot>(PUZZLE_2D_PRESELECT_EMPTY);
  const [hoveredId, setHoveredId] = reactHostPort.useState<string | null>(null);
  const [hoveredKind, setHoveredKind] = reactHostPort.useState<Puzzle2dKindHover | null>(null);
  const [hoverSourcePane, setHoverSourcePane] = reactHostPort.useState<Puzzle2dPlayPaneId | null>(null);
  const hoverSourcePaneRef = reactHostPort.useRef<Puzzle2dPlayPaneId | null>(hoverSourcePane);
  hoverSourcePaneRef.current = hoverSourcePane;
  const [puzzle2dSelectionMethod, setPuzzle2dSelectionMethod] = reactHostPort.useState<Puzzle2dSelectionMethod>("rectangle");
  const [puzzle2dSelectionMode, setPuzzle2dSelectionMode] = reactHostPort.useState<Puzzle2dSelectionMode>("default");
  const [puzzle2dSelectionTargets, setPuzzle2dSelectionTargets] = reactHostPort.useState<Puzzle2dSelectionTargets>(() => ({ ...PUZZLE_2D_SELECTION_TARGETS_DEFAULT }));
  const [puzzle2dGridSnapEnabled, setPuzzle2dGridSnapEnabled] = reactHostPort.useState(false);
  const [puzzle2dActiveTool, setPuzzle2dActiveTool] = reactHostPort.useState<Puzzle2dActiveTool>("select");
  const [puzzle2dSuggestionOffset, setPuzzle2dSuggestionOffset] = reactHostPort.useState(DEFAULT_PUZZLE_2D_SUGGESTION_OFFSET_PX);
  const puzzle2dFillSessionReadyEpoch = reactHostPort.useSyncExternalStore(
    subscribePuzzle2dFillSessionReady,
    getPuzzle2dFillSessionReadyEpoch,
    () => 0,
  );
  void puzzle2dFillSessionReadyEpoch;
  const puzzle2dShellController = puzzle2dRuntime.getActiveApp()?.controller as Puzzle2dPlayShellController | undefined;
  const shellGeneration = reactHostPort.useSyncExternalStore(
    (onStoreChange) => puzzle2dRuntime.subscribe(onStoreChange),
    () => puzzle2dRuntime.generation,
    () => 0,
  );
  void shellGeneration;
  const puzzle2dLodModeByPane = puzzle2dShellController?.getLodModeByPane() ?? {
    "2d-detail": PUZZLE_2D_LOD_MODE_AUTOMATIC,
    "2d-overview": PUZZLE_2D_LOD_MODE_AUTOMATIC,
    "2d-selection": PUZZLE_2D_LOD_MODE_AUTOMATIC,
  };
  const lodModeForScope = reactHostPort.useCallback(
    (scopeId: string, pane: Puzzle2dPlayPaneId) => puzzle2dShellController?.lodModeForScope(scopeId, pane) ?? puzzle2dLodModeByPane[pane],
    [puzzle2dLodModeByPane, puzzle2dShellController],
  );
  const setPuzzle2dLodModeForPane = reactHostPort.useCallback(
    (pane: Puzzle2dPlayPaneId, mode: Puzzle2dLodModeKind) => {
      puzzle2dRuntime.commandBus.dispatch(PUZZLE_2D_PLAY_CONTROLLER_ID, "setLodModeForPane", { pane, value: mode });
    },
    [puzzle2dRuntime.commandBus],
  );
  const notifyBrushCandidates = reactHostPort.useCallback(
    (payload: Puzzle2dBrushCandidatesPayload) => {
      if (puzzle2dActiveTool !== "brush") {
        puzzle2dShellController?.setBrushEngagementPossibles([]);
        return;
      }
      const rows =
        payload.candidates.length > 0
          ? payload.candidates.map((kindId, index) => ({
              id: `puzzle2d.brush.${kindId}.${index}`,
              label: puzzle2dNodeKindOverlayLabel(kindId, PUZZLE_2D_PLAY_DEFAULT_KIND_CATALOGS),
            }))
          : [];
      puzzle2dShellController?.setBrushEngagementPossibles(rows);
    },
    [puzzle2dActiveTool, puzzle2dShellController],
  );

  const preparePuzzle2dFillSessionOnHost = reactHostPort.useCallback(
    (base: Puzzle2dFixture) => {
      preparePuzzle2dFillSession(base, puzzle2dActiveRenderer(), puzzle2dFixtureMergedKindCatalogs(base));
    },
    [],
  );

  const puzzle2dFillAutoStartedRef = reactHostPort.useRef(false);
  reactHostPort.useEffect(() => {
    if (puzzle2dActiveTool !== "fill") {
      puzzle2dFillAutoStartedRef.current = false;
      return;
    }
    const progress = puzzle2dFillBuildProgressRef.current;
    if (!progress.done || progress.count === 0 || puzzle2dFillAutoStartedRef.current) {
      return;
    }
    puzzle2dFillAutoStartedRef.current = true;
    puzzle2dShellController?.run("engagementControlChange", { pane: "2d-overview", value: 1 });
  }, [puzzle2dActiveTool, puzzle2dFillSessionReadyEpoch, puzzle2dShellController]);

  reactHostPort.useEffect(() => {
    puzzle2dShellController?.setKindCatalogs(PUZZLE_2D_PLAY_DEFAULT_KIND_CATALOGS);
  }, [puzzle2dShellController]);

  const setPuzzle2dEffectiveLodForPane = reactHostPort.useCallback(
    (pane: Puzzle2dPlayPaneId, lod: Puzzle2dDrawLodKind) => {
      puzzle2dRuntime.commandBus.dispatch(PUZZLE_2D_PLAY_CONTROLLER_ID, "setEffectiveLodForPane", { pane, lod });
    },
    [puzzle2dRuntime.commandBus],
  );
  const onPuzzle2dPlayActiveWindowChange = reactHostPort.useCallback((shellWindowId: string) => {
    if (puzzle2dPlayPaneFromShellWindowId(shellWindowId)) {
      setActiveScopeId(shellWindowId);
    }
  }, []);

  const setActivePaneId = reactHostPort.useCallback((pane: Puzzle2dPlayPaneId) => {
    setActiveScopeId((current) => (puzzle2dPlayPaneFromShellWindowId(current) === pane ? current : pane));
  }, []);
  const [puzzle2dRedrawPlaying, setPuzzle2dRedrawPlaying] = reactHostPort.useState(
    PUZZLE_2D_PLAY_IS_WIRES ? WIRES_PLAY_LIVE_FORCE_GRAPH_DEFAULTS.puzzle2dRedrawPlaying : false,
  );
  const [forceLayoutFullIterations, setForceLayoutFullIterations] = reactHostPort.useState(200);
  const [forceLayoutIdealEdgeLength, setForceLayoutIdealEdgeLength] = reactHostPort.useState(64);
  const [forceLayoutGravity, setForceLayoutGravity] = reactHostPort.useState(PUZZLE_2D_PLAY_IS_WIRES ? 0 : 0.012);
  const [forceLayoutRepulsionStrength, setForceLayoutRepulsionStrength] = reactHostPort.useState(80);
  const [puzzle2dRedrawPlayMaxItersPerFrame, setPuzzle2dRedrawPlayMaxItersPerFrame] = reactHostPort.useState(96);
  const [puzzle2dRedrawProgressiveEnabled, setPuzzle2dRedrawProgressiveEnabled] = reactHostPort.useState(true);
  const [puzzle2dRedrawProgressiveAutoStopMs, setPuzzle2dRedrawProgressiveAutoStopMs] = reactHostPort.useState(
    PUZZLE_2D_PLAY_IS_WIRES ? WIRES_PLAY_LIVE_FORCE_GRAPH_DEFAULTS.puzzle2dRedrawProgressiveAutoStopMs : 3000,
  );
  const [puzzle2dRedrawMode, setPuzzle2dRedrawMode] = reactHostPort.useState<Puzzle2dRedrawModeKind>("force-graph");
  const [puzzle2dRedrawHandlesAfterNodes, setPuzzle2dRedrawHandlesAfterNodes] = reactHostPort.useState(false);
  const [treeLayoutLayerSpacing, setTreeLayoutLayerSpacing] = reactHostPort.useState(120);
  const [treeLayoutSiblingGap, setTreeLayoutSiblingGap] = reactHostPort.useState(28);
  const [treeLayoutDirection, setTreeLayoutDirection] = reactHostPort.useState<Puzzle2dHierarchicalTreeDirectionKind>("downwards");

  const puzzle2dRedrawPlayingRef = reactHostPort.useRef(puzzle2dRedrawPlaying);
  puzzle2dRedrawPlayingRef.current = puzzle2dRedrawPlaying;

  const [sceneAuthoringEpoch, setSceneAuthoringEpoch] = reactHostPort.useState(0);
  const bumpSceneAuthoringEpoch = reactHostPort.useCallback(() => {
    setSceneAuthoringEpoch((epoch) => epoch + 1);
  }, []);

  const authoringStructuralMutationRef = reactHostPort.useRef(false);
  const applyStructuralDelete = reactHostPort.useCallback((kind: "edge" | "node", id: string) => {
    authoringStructuralMutationRef.current = true;
    const pruneSelections = (removeIds: readonly string[]): void => {
      const remove = new Set(removeIds);
      setSelectionIdsState((prev) => new Set([...prev].filter((x) => !remove.has(x))));
    };
    if (kind === "edge") {
      setFixtureState((prev) => {
        if (!prev.edges.some((e) => e.id === id)) {
          return prev;
        }
        const next = { ...prev, edges: prev.edges.filter((e) => e.id !== id) };
        puzzle2dSyncFixtureDescriptorToAllAuthoringPeers(next);
        return next;
      });
      pruneSelections([id]);
      bumpSceneAuthoringEpoch();
      return;
    }
    const node = fixtureRef.current.nodes.find((n) => n.id === id);
    const handleIds = node?.handles.map((h) => h.id) ?? [];
    setFixtureState((prev) => {
      const next = puzzle2dPlayApplyNodeStructuralDeleteToFixture(prev, id);
      if (next === prev) {
        return prev;
      }
      puzzle2dSyncFixtureDescriptorToAllAuthoringPeers(next);
      return next;
    });
    pruneSelections([id, ...handleIds]);
    bumpSceneAuthoringEpoch();
  }, [bumpSceneAuthoringEpoch]);

  const fixtureAuthoringQuietUntilRef = reactHostPort.useRef(0);
  const paletteDropNodeGuardRef = reactHostPort.useRef<Set<string>>(new Set());
  const guardFixtureAuthoringFromStructuralDeletes = reactHostPort.useCallback((quietMs = 100) => {
    const now = typeof performance !== "undefined" ? performance.now() : Date.now();
    fixtureAuthoringQuietUntilRef.current = Math.max(fixtureAuthoringQuietUntilRef.current, now + quietMs);
  }, []);

  reactHostPort.useLayoutEffect(() => {
    guardFixtureAuthoringFromStructuralDeletes(800);
    setFixtureState((prev) => puzzle2dPlayRehydrateFixtureEdgesIfMissing(prev, initialFixture));
  }, [guardFixtureAuthoringFromStructuralDeletes]);

  reactHostPort.useLayoutEffect(() => {
    if (authoringStructuralMutationRef.current) {
      authoringStructuralMutationRef.current = false;
      return;
    }
    setFixtureState((prev) => puzzle2dPlayRehydrateFixtureEdgesIfMissing(prev, initialFixture));
  }, [fixture.edges.length]);

  const structuralDeleteQueueRef = reactHostPort.useRef<Puzzle2dPlayStructuralDeleteItem[]>([]);
  const structuralDeleteFlushScheduledRef = reactHostPort.useRef(false);
  const flushStructuralDeleteQueue = reactHostPort.useCallback((): number => {
    structuralDeleteFlushScheduledRef.current = false;
    const batch = structuralDeleteQueueRef.current;
    if (batch.length === 0) {
      return 0;
    }
    structuralDeleteQueueRef.current = [];
    const applied = flushPuzzle2dPlayStructuralDeleteBatch(batch, fixtureRef.current, applyStructuralDelete);
    return applied.length;
  }, [applyStructuralDelete]);
  const queueStructuralDelete = reactHostPort.useCallback(
    (kind: "edge" | "node", id: string) => {
      if (puzzle2dIsBrushPlacementStructuralDeleteGuarded(id)) {
        return;
      }
      if (kind === "node" && paletteDropNodeGuardRef.current.has(id)) {
        return;
      }
      const now = typeof performance !== "undefined" ? performance.now() : Date.now();
      if (now < fixtureAuthoringQuietUntilRef.current) {
        return;
      }
      structuralDeleteQueueRef.current.push({ kind, id });
      if (structuralDeleteFlushScheduledRef.current) {
        return;
      }
      structuralDeleteFlushScheduledRef.current = true;
      queueMicrotask(() => {
        flushStructuralDeleteQueue();
      });
    },
    [flushStructuralDeleteQueue],
  );

  const setFixture = reactHostPort.useCallback((next: Puzzle2dFixture) => {
    guardFixtureAuthoringFromStructuralDeletes(120);
    setFixtureState(next);
    bumpSceneAuthoringEpoch();
    setSelectionIdsState(selectionSeedForFixture(next));
    setPreselection(PUZZLE_2D_PRESELECT_EMPTY);
    setHoveredId(null);
    hoverSourcePaneRef.current = null;
    setHoverSourcePane(null);
    catalogRawFixtureRef.current = undefined;
    setPuzzle2dPlayPaneCamerasBaseline(triptychCamerasForFixture(next));
    puzzle2dPlayShellControllerRef.current?.run("notifyFixtureRevision");
  }, [bumpSceneAuthoringEpoch, guardFixtureAuthoringFromStructuralDeletes, triptychCamerasForFixture]);

  const patchFixture = reactHostPort.useCallback(
    (updater: (prev: Puzzle2dFixture) => Puzzle2dFixture) => {
      guardFixtureAuthoringFromStructuralDeletes(80);
      catalogRawFixtureRef.current = undefined;
      setFixtureState((prev) => updater(prev));
      bumpSceneAuthoringEpoch();
      puzzle2dPlayShellControllerRef.current?.run("notifyFixtureRevision");
    },
    [bumpSceneAuthoringEpoch, guardFixtureAuthoringFromStructuralDeletes],
  );

  const applyCanvasSelection = reactHostPort.useCallback((ids: readonly string[]) => {
    setSelectionIdsState(new Set(ids));
    puzzle2dSyncSelectionToAllAuthoringPeers(ids);
  }, []);
  const setSelectionIds = reactHostPort.useCallback((ids: readonly string[]) => {
    setSelectionIdsState(new Set(ids));
    puzzle2dSyncSelectionToAllAuthoringPeers(ids);
    puzzle2dPlayShellControllerRef.current?.run("setGraphSelect", { ids: [...ids] });
  }, []);

  const setHoverPane = reactHostPort.useCallback((pane: Puzzle2dPlayPaneId) => {
    if (hoverSourcePaneRef.current === pane) {
      return;
    }
    hoverSourcePaneRef.current = pane;
    setHoverSourcePane(pane);
  }, []);

  const applyHoverFocus = reactHostPort.useCallback((payload: Puzzle2dHoverPayload) => {
    setHoveredId(payload.id);
    setHoveredKind(payload.kind);
  }, []);

  const setHoverForPane = reactHostPort.useCallback(
    (pane: Puzzle2dPlayPaneId, payload: Puzzle2dHoverPayload) => {
      hoverSourcePaneRef.current = pane;
      setHoverSourcePane(pane);
      applyHoverFocus(payload);
      puzzle2dPlayShellControllerRef.current?.run("setGraphHover", { id: payload.id });
    },
    [applyHoverFocus],
  );

  const clearHoverForPane = reactHostPort.useCallback((pane: Puzzle2dPlayPaneId) => {
    if (hoverSourcePaneRef.current !== pane) {
      return;
    }
    hoverSourcePaneRef.current = null;
    setHoverSourcePane(null);
    setHoveredId(null);
    setHoveredKind(null);
  }, []);

  const setHierarchyHover = reactHostPort.useCallback(
    (payload: Puzzle2dHoverPayload) => {
      hoverSourcePaneRef.current = null;
      setHoverSourcePane(null);
      applyHoverFocus(payload);
      puzzle2dPlayShellControllerRef.current?.run("setGraphHover", { id: payload.id });
    },
    [applyHoverFocus],
  );

  const handleCanvasFixtureDrop = reactHostPort.useCallback(
    (_pane: Puzzle2dPlayPaneId, detail: Puzzle2dFixtureDropDetail) => {
      skipNextCameraBasisResyncRef.current = true;
      guardFixtureAuthoringFromStructuralDeletes(200);
      const placedNodeId = puzzle2dCommitPaletteNodeDropToPlay(detail, { patchFixture, setSelectionIds });
      if (placedNodeId) {
        paletteDropNodeGuardRef.current.add(placedNodeId);
        if (typeof globalThis.setTimeout === "function") {
          globalThis.setTimeout(() => {
            paletteDropNodeGuardRef.current.delete(placedNodeId);
          }, 600);
        }
        return;
      }
      setFixture(detail.fixture);
    },
    [guardFixtureAuthoringFromStructuralDeletes, patchFixture, setFixture, setSelectionIds],
  );

  const commitBrushPlacement = reactHostPort.useCallback(
    (payload: Puzzle2dBrushPlacePayload) => {
      guardFixtureAuthoringFromStructuralDeletes(200);
      puzzle2dCommitBrushPlacementToPlay(payload, {
        catalogsForFixture: puzzle2dFixtureMergedKindCatalogs,
        patchFixture,
      });
    },
    [guardFixtureAuthoringFromStructuralDeletes, patchFixture],
  );

  reactHostPort.useLayoutEffect(() => {
    puzzle2dSetBrushPlaceCommitHandler(commitBrushPlacement);
    return () => {
      puzzle2dSetBrushPlaceCommitHandler(null);
    };
  }, [commitBrushPlacement]);

  reactHostPort.useEffect(() => {
    if (puzzle2dActiveTool !== "brush" && !puzzle2dBrushSuggestionsMenuOpen()) {
      puzzle2dSyncBrushSessionToAllAuthoringPeers(null);
      return;
    }
    const flushedCount = flushStructuralDeleteQueue();
    if (flushedCount > 0) {
      return;
    }
    puzzle2dSyncFixtureDescriptorToAllAuthoringPeers(fixture);
  }, [fixture, puzzle2dActiveTool, flushStructuralDeleteQueue]);

  const remapIdInSelections = reactHostPort.useCallback((replacedId: string, replacementId: string) => {
    if (replacedId === replacementId) {
      return;
    }
    setSelectionIdsState((prev) => new Set([...prev].map((id) => (id === replacedId ? replacementId : id))));
  }, []);

  const cameraBasisFixtureRef = reactHostPort.useRef<Puzzle2dFixture>(fixture);
  /** @emoji 📌 One-shot: sync {@link cameraBasisFixtureRef} without resetting {@link puzzle2dPlayPaneCamerasBaseline} after palette / shelf fixture drop. */
  const skipNextCameraBasisResyncRef = reactHostPort.useRef(false);
  const prevPuzzle2dRedrawPlayingRef = reactHostPort.useRef(false);
  const [cameraDisplayOverrideByPane, setCameraDisplayOverrideByPane] = reactHostPort.useState<Record<Puzzle2dPlayPaneId, CameraState> | null>(null);
  const cameraDisplayOverrideRef = reactHostPort.useRef<Record<Puzzle2dPlayPaneId, CameraState> | null>(null);
  cameraDisplayOverrideRef.current = cameraDisplayOverrideByPane;
  const suppressCameraBasisSyncRef = reactHostPort.useRef(false);
  const cameraPlayEndAnimRafRef = reactHostPort.useRef<number | null>(null);
  const puzzle2dPlayNodesRedrawCameraAnimRafRef = reactHostPort.useRef<number | null>(null);
  const puzzle2dPlayRedrawCameraChaseRef = reactHostPort.useRef<Record<Puzzle2dPlayPaneId, CameraState> | null>(null);
  const lastPlayingForCameraEaseRef = reactHostPort.useRef(false);
  const [nodesRedrawCameraEaseTick, setNodesRedrawCameraEaseTick] = reactHostPort.useState(0);
  /** @emoji 📷 Cameras shown on canvases at click time; set before {@link patchFixture} so `from` cannot lag one commit behind the graph. */
  const nodesRedrawEaseFromRef = reactHostPort.useRef<Record<Puzzle2dPlayPaneId, CameraState> | null>(null);
  /** @emoji 🔢 Bumped on each redraw click / competing camera path so stale RAF ticks never call {@link setPuzzle2dPlayPaneCamerasBaseline}. */
  const nodesRedrawEaseGenerationRef = reactHostPort.useRef(0);

  const syncBaselineFromViewportCamera = reactHostPort.useCallback((cam: CameraState) => {
    if (puzzle2dRedrawPlayingRef.current) {
      return;
    }
    if (suppressCameraBasisSyncRef.current) {
      return;
    }
    if (cameraDisplayOverrideRef.current !== null) {
      return;
    }
    const c = { x: cam.x, y: cam.y, zoom: cam.zoom };
    const scope = activeScopeIdRef.current;
    const pane = activePaneIdRef.current;
    setCameraByScope((prev) => {
      const p = prev[scope] ?? puzzle2dPlayPaneCamerasBaselineRef.current[pane];
      if (Math.abs(p.x - c.x) < 1e-6 && Math.abs(p.y - c.y) < 1e-6 && Math.abs(p.zoom - c.zoom) < 1e-9) {
        return prev;
      }
      return { ...prev, [scope]: { ...c } };
    });
    setPuzzle2dPlayPaneCamerasBaseline((prev) => {
      const p = prev[pane];
      if (Math.abs(p.x - c.x) < 1e-6 && Math.abs(p.y - c.y) < 1e-6 && Math.abs(p.zoom - c.zoom) < 1e-9) {
        return prev;
      }
      return { ...prev, [pane]: { ...c } };
    });
  }, []);

  const cameraForScope = reactHostPort.useCallback(
    (scopeId: string, pane: Puzzle2dPlayPaneId): CameraState => {
      const merged = cameraDisplayOverrideByPane ?? puzzle2dPlayPaneCamerasBaseline;
      return cameraByScope[scopeId] ?? merged[pane];
    },
    [cameraByScope, cameraDisplayOverrideByPane, puzzle2dPlayPaneCamerasBaseline],
  );

  reactHostPort.useEffect(() => {
    if (puzzle2dRedrawPlaying) {
      return;
    }
    if (suppressCameraBasisSyncRef.current) {
      return;
    }
    if (skipNextCameraBasisResyncRef.current) {
      skipNextCameraBasisResyncRef.current = false;
      cameraBasisFixtureRef.current = fixture;
      return;
    }
    cameraBasisFixtureRef.current = fixture;
  }, [fixture, puzzle2dRedrawPlaying]);

  reactHostPort.useEffect(() => {
    const prevPlaying = prevPuzzle2dRedrawPlayingRef.current;
    const playJustStarted = puzzle2dRedrawPlaying && !prevPlaying;

    if (playJustStarted) {
      nodesRedrawEaseGenerationRef.current += 1;
      nodesRedrawEaseFromRef.current = null;
      if (cameraPlayEndAnimRafRef.current != null) {
        cancelAnimationFrame(cameraPlayEndAnimRafRef.current);
        cameraPlayEndAnimRafRef.current = null;
      }
      if (puzzle2dPlayNodesRedrawCameraAnimRafRef.current != null) {
        cancelAnimationFrame(puzzle2dPlayNodesRedrawCameraAnimRafRef.current);
        puzzle2dPlayNodesRedrawCameraAnimRafRef.current = null;
      }
      setCameraDisplayOverrideByPane(null);
      suppressCameraBasisSyncRef.current = false;
      cameraBasisFixtureRef.current = fixture;
      const prevCam = puzzle2dPlayPaneCamerasBaselineRef.current;
      puzzle2dPlayRedrawCameraChaseRef.current = {
        "2d-detail": { ...prevCam["2d-detail"] },
        "2d-overview": { ...prevCam["2d-overview"] },
        "2d-selection": { ...prevCam["2d-selection"] },
      };
    } else if (!suppressCameraBasisSyncRef.current) {
      if (cameraPlayEndAnimRafRef.current != null) {
        cancelAnimationFrame(cameraPlayEndAnimRafRef.current);
        cameraPlayEndAnimRafRef.current = null;
      }
    }
    prevPuzzle2dRedrawPlayingRef.current = puzzle2dRedrawPlaying;
  }, [puzzle2dRedrawPlaying, fixture]);

  reactHostPort.useEffect(() => {
    if (!puzzle2dRedrawPlaying) {
      puzzle2dPlayRedrawCameraChaseRef.current = null;
      return;
    }
    if (suppressCameraBasisSyncRef.current) {
      return;
    }
    const pane = activePaneIdRef.current;
    const target = triptychCamerasForFixture(fixture);
    setPuzzle2dPlayPaneCamerasBaseline((baselinePrev) => {
      const prevChase = puzzle2dPlayRedrawCameraChaseRef.current ?? baselinePrev;
      const damped = dampCameraStateLinear(prevChase[pane], target[pane], PUZZLE_2D_PLAY_REDRAW_CAMERA_CHASE_BLEND);
      const nextChase: Record<Puzzle2dPlayPaneId, CameraState> = {
        "2d-detail": { ...prevChase["2d-detail"] },
        "2d-overview": { ...prevChase["2d-overview"] },
        "2d-selection": { ...prevChase["2d-selection"] },
      };
      nextChase[pane] = damped;
      puzzle2dPlayRedrawCameraChaseRef.current = nextChase;
      return nextChase;
    });
  }, [puzzle2dRedrawPlaying, fixture, triptychCamerasForFixture]);

  reactHostPort.useEffect(() => {
    if (puzzle2dRedrawPlaying) {
      lastPlayingForCameraEaseRef.current = true;
      return () => {
        if (cameraPlayEndAnimRafRef.current != null) {
          cancelAnimationFrame(cameraPlayEndAnimRafRef.current);
          cameraPlayEndAnimRafRef.current = null;
        }
        if (puzzle2dPlayNodesRedrawCameraAnimRafRef.current != null) {
          cancelAnimationFrame(puzzle2dPlayNodesRedrawCameraAnimRafRef.current);
          puzzle2dPlayNodesRedrawCameraAnimRafRef.current = null;
        }
      };
    }
    if (!lastPlayingForCameraEaseRef.current) {
      return;
    }
    lastPlayingForCameraEaseRef.current = false;

    const snapshotFixture = fixtureRef.current;
    const from: Record<Puzzle2dPlayPaneId, CameraState> = {
      "2d-detail": { ...puzzle2dPlayPaneCamerasBaseline["2d-detail"] },
      "2d-overview": { ...puzzle2dPlayPaneCamerasBaseline["2d-overview"] },
      "2d-selection": { ...puzzle2dPlayPaneCamerasBaseline["2d-selection"] },
    };
    cameraBasisFixtureRef.current = snapshotFixture;
    const to = triptychCamerasForFixture(snapshotFixture);
    const postPlayEasePaneId = activePaneIdRef.current;
    suppressCameraBasisSyncRef.current = true;
    if (puzzle2dPlayNodesRedrawCameraAnimRafRef.current != null) {
      cancelAnimationFrame(puzzle2dPlayNodesRedrawCameraAnimRafRef.current);
      puzzle2dPlayNodesRedrawCameraAnimRafRef.current = null;
    }
    nodesRedrawEaseGenerationRef.current += 1;
    setCameraDisplayOverrideByPane(from);

    const total = PUZZLE_2D_PLAY_CAMERA_POST_REDRAW_TOTAL_MS;
    const holdEnd = total / 3;
    const animSpan = total - holdEnd;
    const t0 = typeof performance !== "undefined" ? performance.now() : Date.now();
    const tickInner = () => {
      const now = typeof performance !== "undefined" ? performance.now() : Date.now();
      const elapsed = now - t0;
      if (elapsed >= total) {
        const endCameras = blendTriptychCamerasActivePaneOnly(from, to, 1, postPlayEasePaneId);
        setCameraDisplayOverrideByPane(endCameras);
        suppressCameraBasisSyncRef.current = false;
        cameraBasisFixtureRef.current = fixtureRef.current;
        cameraPlayEndAnimRafRef.current = requestAnimationFrame(() => {
          setCameraDisplayOverrideByPane(null);
          const fit = triptychCamerasForFixture(fixtureRef.current);
          const p = postPlayEasePaneId;
          setPuzzle2dPlayPaneCamerasBaseline((prev) => ({ ...prev, [p]: { ...fit[p] } }));
          cameraPlayEndAnimRafRef.current = null;
        });
        return;
      }
      if (elapsed >= holdEnd) {
        const u = Math.min(1, Math.max(0, (elapsed - holdEnd) / animSpan));
        setCameraDisplayOverrideByPane(blendTriptychCamerasActivePaneOnly(from, to, u, postPlayEasePaneId));
      }
      cameraPlayEndAnimRafRef.current = requestAnimationFrame(tickInner);
    };
    cameraPlayEndAnimRafRef.current = requestAnimationFrame(tickInner);

    return () => {
      if (cameraPlayEndAnimRafRef.current != null) {
        cancelAnimationFrame(cameraPlayEndAnimRafRef.current);
        cameraPlayEndAnimRafRef.current = null;
      }
    };
  }, [puzzle2dRedrawPlaying]);

  const camerasByPane = cameraDisplayOverrideByPane ?? puzzle2dPlayPaneCamerasBaseline;

  reactHostPort.useEffect(() => {
    if (nodesRedrawCameraEaseTick === 0) {
      return;
    }
    if (puzzle2dRedrawPlayingRef.current) {
      return;
    }
    if (suppressCameraBasisSyncRef.current) {
      return;
    }
    if (cameraDisplayOverrideRef.current !== null) {
      return;
    }
    const fromSnapshot = nodesRedrawEaseFromRef.current;
    if (fromSnapshot === null) {
      return;
    }
    const generationAtStart = nodesRedrawEaseGenerationRef.current;
    if (puzzle2dPlayNodesRedrawCameraAnimRafRef.current != null) {
      cancelAnimationFrame(puzzle2dPlayNodesRedrawCameraAnimRafRef.current);
      puzzle2dPlayNodesRedrawCameraAnimRafRef.current = null;
    }
    const snapshotFixture = fixtureRef.current;
    const from: Record<Puzzle2dPlayPaneId, CameraState> = {
      "2d-detail": { ...fromSnapshot["2d-detail"] },
      "2d-overview": { ...fromSnapshot["2d-overview"] },
      "2d-selection": { ...fromSnapshot["2d-selection"] },
    };
    const to = triptychCamerasForFixture(snapshotFixture);
    const nodesRedrawEasePaneId = activePaneIdRef.current;
    const total = PUZZLE_2D_PLAY_NODES_REDRAW_CAMERA_EASE_TOTAL_MS;
    const holdEnd = total / 3;
    const animSpan = total - holdEnd;
    const t0 = typeof performance !== "undefined" ? performance.now() : Date.now();
    const tickInner = () => {
      if (nodesRedrawEaseGenerationRef.current !== generationAtStart) {
        return;
      }
      const now = typeof performance !== "undefined" ? performance.now() : Date.now();
      const elapsed = now - t0;
      if (elapsed >= total) {
        const endCameras = blendTriptychCamerasActivePaneOnly(from, to, 1, nodesRedrawEasePaneId);
        setPuzzle2dPlayPaneCamerasBaseline(endCameras);
        puzzle2dPlayNodesRedrawCameraAnimRafRef.current = null;
        nodesRedrawEaseFromRef.current = null;
        return;
      }
      if (elapsed >= holdEnd) {
        const u = Math.min(1, Math.max(0, (elapsed - holdEnd) / animSpan));
        setPuzzle2dPlayPaneCamerasBaseline(blendTriptychCamerasActivePaneOnly(from, to, u, nodesRedrawEasePaneId));
      }
      puzzle2dPlayNodesRedrawCameraAnimRafRef.current = requestAnimationFrame(tickInner);
    };
    puzzle2dPlayNodesRedrawCameraAnimRafRef.current = requestAnimationFrame(tickInner);
    return () => {
      if (puzzle2dPlayNodesRedrawCameraAnimRafRef.current != null) {
        cancelAnimationFrame(puzzle2dPlayNodesRedrawCameraAnimRafRef.current);
        puzzle2dPlayNodesRedrawCameraAnimRafRef.current = null;
      }
    };
  }, [nodesRedrawCameraEaseTick]);

  reactHostPort.useEffect(() => {
    if (cameraDisplayOverrideByPane === null) {
      return;
    }
    nodesRedrawEaseGenerationRef.current += 1;
    if (puzzle2dPlayNodesRedrawCameraAnimRafRef.current != null) {
      cancelAnimationFrame(puzzle2dPlayNodesRedrawCameraAnimRafRef.current);
      puzzle2dPlayNodesRedrawCameraAnimRafRef.current = null;
    }
  }, [cameraDisplayOverrideByPane]);

  const redrawPlayingRef = reactHostPort.useRef(false);
  const redrawProgressiveEpochRef = reactHostPort.useRef(0);
  const puzzle2dPlayDraggingNodeIdsRef = reactHostPort.useRef<Set<string>>(new Set());
  const puzzle2dPlayDragAnchorsRef = reactHostPort.useRef<Map<string, { x: number; y: number }>>(new Map());

  const notePuzzle2dPlayNodeDragMove = reactHostPort.useCallback(
    (payload: { readonly id: string; readonly x: number; readonly y: number }) => {
      puzzle2dPlayDraggingNodeIdsRef.current.add(payload.id);
      puzzle2dPlayDragAnchorsRef.current.set(payload.id, { x: payload.x, y: payload.y });
      if (!puzzle2dRedrawPlayingRef.current) {
        return;
      }
      redrawProgressiveEpochRef.current = typeof performance !== "undefined" ? performance.now() : Date.now();
      patchFixture((prev) => ({
        ...prev,
        nodes: prev.nodes.map((node) => (node.id === payload.id ? { ...node, x: payload.x, y: payload.y } : node)),
      }));
    },
    [patchFixture],
  );

  const clearPuzzle2dPlayNodeDrag = reactHostPort.useCallback(() => {
    puzzle2dPlayDraggingNodeIdsRef.current.clear();
    puzzle2dPlayDragAnchorsRef.current.clear();
  }, []);

  const puzzle2dPlayLiveDragLockedNodeIds = reactHostPort.useCallback((): readonly string[] | undefined => {
    const ids = puzzle2dPlayDraggingNodeIdsRef.current;
    return ids.size > 0 ? [...ids] : undefined;
  }, []);
  const redrawLoopSnapshotRef = reactHostPort.useRef<Puzzle2dPlayRedrawLoopSnapshot>({
    activePaneId: "2d-overview",
    puzzle2dRedrawHandlesAfterNodes: false,
    puzzle2dRedrawProgressiveAutoStopMs: 3000,
    puzzle2dRedrawProgressiveEnabled: true,
    puzzle2dRedrawPlayMaxItersPerFrame: 96,
    camerasByPane: puzzle2dPlayInitialCameras(),
    forceLayoutGravity: PUZZLE_2D_PLAY_IS_WIRES ? 0 : 0.012,
    forceLayoutIdealEdgeLength: 64,
    forceLayoutRepulsionStrength: 80,
    mode: "force-graph",
    treeLayoutDirection: "downwards",
    treeLayoutLayerSpacing: 120,
    treeLayoutSiblingGap: 28,
  });

  const resetPuzzle2dRedrawProgressiveEpoch = reactHostPort.useCallback(() => {
    redrawProgressiveEpochRef.current = typeof performance !== "undefined" ? performance.now() : Date.now();
  }, []);

  redrawLoopSnapshotRef.current = {
    activePaneId,
    puzzle2dRedrawHandlesAfterNodes,
    puzzle2dRedrawProgressiveAutoStopMs,
    puzzle2dRedrawProgressiveEnabled,
    puzzle2dRedrawPlayMaxItersPerFrame,
    camerasByPane,
    forceLayoutGravity,
    forceLayoutIdealEdgeLength,
    forceLayoutRepulsionStrength,
    mode: puzzle2dRedrawMode,
    treeLayoutDirection,
    treeLayoutLayerSpacing,
    treeLayoutSiblingGap,
  };

  const applyPuzzle2dRedrawHandlesOnce = reactHostPort.useCallback(() => {
    patchFixture((prev) => layoutPuzzle2dFixtureRedrawHandles(prev));
  }, [patchFixture]);

  const applyPuzzle2dRedrawOnce = reactHostPort.useCallback(() => {
    if (puzzle2dPlayNodesRedrawCameraAnimRafRef.current != null) {
      cancelAnimationFrame(puzzle2dPlayNodesRedrawCameraAnimRafRef.current);
      puzzle2dPlayNodesRedrawCameraAnimRafRef.current = null;
    }
    nodesRedrawEaseGenerationRef.current += 1;
    nodesRedrawEaseFromRef.current = {
      "2d-detail": { ...camerasByPane["2d-detail"] },
      "2d-overview": { ...camerasByPane["2d-overview"] },
      "2d-selection": { ...camerasByPane["2d-selection"] },
    };
    const full = Math.max(1, Math.min(5000, Math.round(forceLayoutFullIterations)));
    const lockedNodeIds = puzzle2dPlayLiveDragLockedNodeIds();
    const dragAnchors = puzzle2dPlayDragAnchorsRef.current;
    const dragState = puzzle2dPlayLiveForceGraphDragState(dragAnchors, lockedNodeIds);
    patchFixture((prev) => {
      const layoutOpts = puzzle2dPlayRedrawLayoutOpts(
        activePaneId,
        camerasByPane,
        puzzle2dRedrawMode,
        full,
        forceLayoutIdealEdgeLength,
        forceLayoutGravity,
        forceLayoutRepulsionStrength,
        treeLayoutLayerSpacing,
        treeLayoutSiblingGap,
        treeLayoutDirection,
        puzzle2dRedrawHandlesAfterNodes,
        lockedNodeIds,
      );
      const laidOut =
        puzzle2dRedrawMode === "force-graph"
          ? puzzle2dApplyLiveForceGraphLayoutTick(prev, layoutOpts, dragState)
          : puzzle2dFinalizeLiveForceGraphLayoutTick(layoutPuzzle2dFixtureRedrawNodes(prev, layoutOpts), dragState);
      return { ...laidOut, camera: { ...prev.camera } };
    });
    setNodesRedrawCameraEaseTick((n) => n + 1);
  }, [
    activePaneId,
    puzzle2dRedrawHandlesAfterNodes,
    puzzle2dRedrawMode,
    camerasByPane,
    forceLayoutFullIterations,
    forceLayoutGravity,
    forceLayoutIdealEdgeLength,
    forceLayoutRepulsionStrength,
    patchFixture,
    treeLayoutLayerSpacing,
    treeLayoutDirection,
    treeLayoutSiblingGap,
    puzzle2dPlayLiveDragLockedNodeIds,
  ]);

  reactHostPort.useEffect(() => {
    if (!puzzle2dRedrawPlaying) {
      redrawPlayingRef.current = false;
      return;
    }
    redrawPlayingRef.current = true;
    redrawProgressiveEpochRef.current = typeof performance !== "undefined" ? performance.now() : Date.now();
    let raf = 0;
    const step = () => {
      if (!redrawPlayingRef.current) {
        return;
      }
      const snap = redrawLoopSnapshotRef.current;
      const lockedNodeIds = puzzle2dPlayLiveDragLockedNodeIds();
      const dragAnchors = puzzle2dPlayDragAnchorsRef.current;
      const now = typeof performance !== "undefined" ? performance.now() : Date.now();
      const elapsed = now - redrawProgressiveEpochRef.current;
      if (snap.puzzle2dRedrawProgressiveAutoStopMs > 0 && elapsed >= snap.puzzle2dRedrawProgressiveAutoStopMs) {
        redrawPlayingRef.current = false;
        setPuzzle2dRedrawPlaying(false);
        return;
      }
      let innerIters = 1;
      if (snap.mode === "force-graph") {
        if (snap.puzzle2dRedrawProgressiveEnabled) {
          innerIters = puzzle2dPlayProgressiveForceIters(elapsed, snap.puzzle2dRedrawProgressiveAutoStopMs, snap.puzzle2dRedrawPlayMaxItersPerFrame);
        } else {
          innerIters = Math.max(1, Math.min(500, Math.round(snap.puzzle2dRedrawPlayMaxItersPerFrame)));
        }
      }
      patchFixture((prev) => {
        if (prev.nodes.length === 0) {
          return prev;
        }
        if (snap.mode === "hierarchical-tree") {
          return puzzle2dPlayFixtureWithDragAnchors(
            layoutPuzzle2dFixtureRedrawNodes(
              prev,
              puzzle2dPlayRedrawLayoutOpts(
                snap.activePaneId,
                snap.camerasByPane,
                snap.mode,
                1,
                snap.forceLayoutIdealEdgeLength,
                snap.forceLayoutGravity,
                snap.forceLayoutRepulsionStrength,
                snap.treeLayoutLayerSpacing,
                snap.treeLayoutSiblingGap,
                snap.treeLayoutDirection,
                snap.puzzle2dRedrawHandlesAfterNodes,
                lockedNodeIds,
              ),
            ),
            dragAnchors,
          );
        }
        const t0 = typeof performance !== "undefined" ? performance.now() : Date.now();
        let cur = prev;
        while (redrawPlayingRef.current && (typeof performance !== "undefined" ? performance.now() : Date.now()) - t0 < PUZZLE_2D_PLAY_REDRAW_FRAME_BUDGET_MS) {
          cur = layoutPuzzle2dFixtureRedrawNodes(
            cur,
            puzzle2dPlayRedrawLayoutOpts(
              snap.activePaneId,
              snap.camerasByPane,
              snap.mode,
              innerIters,
              snap.forceLayoutIdealEdgeLength,
              snap.forceLayoutGravity,
              snap.forceLayoutRepulsionStrength,
              snap.treeLayoutLayerSpacing,
              snap.treeLayoutSiblingGap,
              snap.treeLayoutDirection,
              snap.puzzle2dRedrawHandlesAfterNodes,
              lockedNodeIds,
            ),
          );
        }
        return puzzle2dFinalizeLiveForceGraphLayoutTick(cur, puzzle2dPlayLiveForceGraphDragState(dragAnchors, lockedNodeIds));
      });
      raf = requestAnimationFrame(step);
    };
    raf = requestAnimationFrame(step);
    return () => {
      redrawPlayingRef.current = false;
      cancelAnimationFrame(raf);
    };
  }, [patchFixture, puzzle2dPlayLiveDragLockedNodeIds, puzzle2dRedrawPlaying, setPuzzle2dRedrawPlaying]);

  const shellValue = reactHostPort.useMemo<Puzzle2dPlayShellValue>(
    () => ({
      activePaneId,
      activeScopeId,
      applyPuzzle2dRedrawHandlesOnce,
      applyPuzzle2dRedrawOnce,
      applyStructuralDelete,
      queueStructuralDelete,
      puzzle2dRedrawHandlesAfterNodes,
      puzzle2dRedrawMode,
      puzzle2dRedrawPlayMaxItersPerFrame,
      puzzle2dRedrawPlaying,
      puzzle2dRedrawProgressiveAutoStopMs,
      puzzle2dRedrawProgressiveEnabled,
      puzzle2dSelectionMethod,
      puzzle2dSelectionMode,
      puzzle2dSelectionTargets,
      puzzle2dGridSnapEnabled,
      puzzle2dActiveTool,
      setPuzzle2dActiveTool,
      puzzle2dSuggestionOffset,
      setPuzzle2dSuggestionOffset,
      notifyBrushCandidates,
      fixture,
      forceLayoutFullIterations,
      forceLayoutGravity,
      forceLayoutIdealEdgeLength,
      forceLayoutRepulsionStrength,
      commitBrushPlacement,
      handleCanvasFixtureDrop,
      patchFixture,
      remapIdInSelections,
      resetPuzzle2dRedrawProgressiveEpoch,
      notePuzzle2dPlayNodeDragMove,
      clearPuzzle2dPlayNodeDrag,
      setActivePaneId,
      setPuzzle2dRedrawHandlesAfterNodes,
      setPuzzle2dRedrawMode,
      setPuzzle2dRedrawPlayMaxItersPerFrame,
      setPuzzle2dRedrawPlaying,
      setPuzzle2dRedrawProgressiveAutoStopMs,
      setPuzzle2dRedrawProgressiveEnabled,
      setPuzzle2dGridSnapEnabled,
      puzzle2dLodModeByPane,
      lodModeForScope,
      setPuzzle2dLodModeForPane,
      setPuzzle2dSelectionMethod,
      setPuzzle2dSelectionMode,
      setPuzzle2dSelectionTargets,
      setFixture,
      setForceLayoutFullIterations,
      setForceLayoutGravity,
      setForceLayoutIdealEdgeLength,
      setForceLayoutRepulsionStrength,
      setTreeLayoutLayerSpacing,
      setTreeLayoutDirection,
      setTreeLayoutSiblingGap,
      setSelectionIds,
      sceneAuthoringEpoch,
      hoveredId,
      hoveredKind,
      hoverSourcePane,
      setHoverPane,
      setHoverForPane,
      clearHoverForPane,
      setHierarchyHover,
      treeLayoutLayerSpacing,
      treeLayoutDirection,
      treeLayoutSiblingGap,
    }),
    [
      activePaneId,
      activeScopeId,
      applyPuzzle2dRedrawHandlesOnce,
      applyPuzzle2dRedrawOnce,
      applyStructuralDelete,
      queueStructuralDelete,
      puzzle2dRedrawHandlesAfterNodes,
      puzzle2dRedrawMode,
      puzzle2dRedrawPlayMaxItersPerFrame,
      puzzle2dRedrawPlaying,
      puzzle2dRedrawProgressiveAutoStopMs,
      puzzle2dRedrawProgressiveEnabled,
      puzzle2dSelectionMethod,
      puzzle2dSelectionMode,
      puzzle2dSelectionTargets,
      puzzle2dGridSnapEnabled,
      puzzle2dActiveTool,
      puzzle2dSuggestionOffset,
      notifyBrushCandidates,
      puzzle2dLodModeByPane,
      lodModeForScope,
      setPuzzle2dLodModeForPane,
      setActivePaneId,
      fixture,
      forceLayoutFullIterations,
      forceLayoutGravity,
      forceLayoutIdealEdgeLength,
      forceLayoutRepulsionStrength,
      commitBrushPlacement,
      handleCanvasFixtureDrop,
      patchFixture,
      remapIdInSelections,
      resetPuzzle2dRedrawProgressiveEpoch,
      notePuzzle2dPlayNodeDragMove,
      clearPuzzle2dPlayNodeDrag,
      setSelectionIds,
      sceneAuthoringEpoch,
      hoveredId,
      hoveredKind,
      hoverSourcePane,
      setHoverPane,
      setHoverForPane,
      clearHoverForPane,
      setHierarchyHover,
      treeLayoutLayerSpacing,
      treeLayoutDirection,
      treeLayoutSiblingGap,
    ],
  );

  const selectionValue = reactHostPort.useMemo(
    (): Puzzle2dPlaySelectionValue => ({
      selectionIds,
      setSelectionIds,
      applyCanvasSelection,
      preselection,
      setPreselection,
    }),
    [applyCanvasSelection, selectionIds, setSelectionIds, preselection, setPreselection],
  );

  const canvasSelectionValue = reactHostPort.useMemo(
    (): Puzzle2dPlayCanvasSelectionActions => ({
      applyCanvasSelection,
    }),
    [applyCanvasSelection],
  );

  const camerasValue = reactHostPort.useMemo(
    (): Puzzle2dPlayCamerasValue => ({
      camerasByPane,
      cameraByScope,
      syncBaselineFromViewportCamera,
      cameraForScope,
    }),
    [cameraByScope, cameraForScope, camerasByPane, syncBaselineFromViewportCamera],
  );

  // #region 🔖ToolbarHostBridge
  const puzzle2dPlayToolbarHostRef = reactHostPort.useRef({
    activePaneId: "2d-overview" as Puzzle2dPlayPaneId,
    applyPuzzle2dRedrawHandlesOnce: () => {},
    applyPuzzle2dRedrawOnce: () => {},
    camerasByPane: puzzle2dPlayInitialCameras(),
    patchFixture: (_updater: (prev: Puzzle2dFixture) => Puzzle2dFixture) => {},
    setForceLayoutFullIterations: (_value: number) => {},
    setForceLayoutGravity: (_value: number) => {},
    setForceLayoutIdealEdgeLength: (_value: number) => {},
    setForceLayoutRepulsionStrength: (_value: number) => {},
    setPuzzle2dGridSnapEnabled: (_value: boolean | ((prev: boolean) => boolean)) => {},
    setPuzzle2dRedrawHandlesAfterNodes: (_value: boolean) => {},
    setPuzzle2dRedrawMode: (_value: Puzzle2dRedrawModeKind) => {},
    setPuzzle2dRedrawPlayMaxItersPerFrame: (_value: number) => {},
    setPuzzle2dRedrawPlaying: (_value: boolean | ((prev: boolean) => boolean)) => {},
    setPuzzle2dRedrawProgressiveAutoStopMs: (_value: number) => {},
    setPuzzle2dRedrawProgressiveEnabled: (_value: boolean) => {},
    setPuzzle2dSelectionMethod: (_value: Puzzle2dSelectionMethod) => {},
    setPuzzle2dSelectionMode: (_value: Puzzle2dSelectionMode) => {},
    setPuzzle2dSelectionTargets: (_value: Puzzle2dSelectionTargets | ((prev: Puzzle2dSelectionTargets) => Puzzle2dSelectionTargets)) => {},
    setSelectionIds: (_ids: readonly string[]) => {},
    setTreeLayoutDirection: (_value: Puzzle2dHierarchicalTreeDirectionKind) => {},
    setTreeLayoutLayerSpacing: (_value: number) => {},
    setTreeLayoutSiblingGap: (_value: number) => {},
  });
  puzzle2dPlayToolbarHostRef.current = {
    activePaneId,
    applyPuzzle2dRedrawHandlesOnce,
    applyPuzzle2dRedrawOnce,
    camerasByPane,
    patchFixture,
    setForceLayoutFullIterations,
    setForceLayoutGravity,
    setForceLayoutIdealEdgeLength,
    setForceLayoutRepulsionStrength,
    setPuzzle2dGridSnapEnabled,
    setPuzzle2dRedrawHandlesAfterNodes,
    setPuzzle2dRedrawMode,
    setPuzzle2dRedrawPlayMaxItersPerFrame,
    setPuzzle2dRedrawPlaying,
    setPuzzle2dRedrawProgressiveAutoStopMs,
    setPuzzle2dRedrawProgressiveEnabled,
    setPuzzle2dSelectionMethod,
    setPuzzle2dSelectionMode,
    setPuzzle2dSelectionTargets,
    setSelectionIds,
    setTreeLayoutDirection,
    setTreeLayoutLayerSpacing,
    setTreeLayoutSiblingGap,
  };

  reactHostPort.useEffect(() => {
    if (!puzzle2dShellController) {
      return;
    }
    const bridge: Puzzle2dPlayHostBridge = {
      getToolbarState: () => ({
        puzzle2dActiveTool,
        puzzle2dSuggestionOffset,
        puzzle2dGridSnapEnabled,
        puzzle2dRedrawPlaying,
        puzzle2dSelectionMethod,
        puzzle2dSelectionMode,
        puzzle2dSelectionTargets,
      }),
      getFixtureJson: () => puzzle2dFixtureToJson(fixture),
      runHostCommand: (command, args) => {
        const h = puzzle2dPlayToolbarHostRef.current;
        switch (command) {
          case "setSelectionMethod":
            h.setPuzzle2dSelectionMethod((args as { method: Puzzle2dSelectionMethod }).method);
            break;
          case "setSelectionMode":
            h.setPuzzle2dSelectionMode((args as { mode: Puzzle2dSelectionMode }).mode);
            break;
          case "toggleSelectionTarget": {
            const { kind } = args as { kind: "edges" | "handles" | "nodes" };
            h.setPuzzle2dSelectionTargets((prev) => ({ ...prev, [kind]: !prev[kind] }));
            break;
          }
          case "clearSelection":
            h.setSelectionIds([]);
            break;
          case "hierarchySelect": {
            const id = (args as { id?: string }).id;
            if (typeof id === "string") {
              h.setSelectionIds([id]);
            }
            break;
          }
          case "selectAllSelection":
            h.setSelectionIds(puzzle2dPlayAllSelectionFromFixture(fixture, puzzle2dSelectionTargets));
            break;
          case "toggleGridSnap":
            h.setPuzzle2dGridSnapEnabled((prev) => !prev);
            break;
          case "appendCircle": {
            const camera = h.camerasByPane[h.activePaneId];
            const id = newPuzzle2dAuthoringId("node");
            const handleId = `${id}.h0`;
            const node: Puzzle2dFixtureCircleNode = {
              handles: [{ angle: 0, handleKind: BUILTIN_PORT_HANDLE_KIND, id: handleId }],
              id,
              radius: PUZZLE_2D_PLAY_DEFAULT_NODE_SIZE_PX / 2,
              shape: "circle",
              x: camera.x,
              y: camera.y,
            };
            h.patchFixture((prev) => ({ ...prev, nodes: [...prev.nodes, node] }));
            h.setSelectionIds([id]);
            break;
          }
          case "toggleRedrawPlaying":
            h.setPuzzle2dRedrawPlaying((prev) => !prev);
            break;
          case "redrawHandlesOnce":
            h.applyPuzzle2dRedrawHandlesOnce();
            break;
          case "setActiveTool": {
            const { tool, prevTool } = args as { tool: Puzzle2dActiveTool; prevTool?: Puzzle2dActiveTool };
            const prev = prevTool ?? puzzle2dActiveTool;
            setPuzzle2dActiveTool(tool);
            if (tool === "fill" && prev !== "fill") {
              preparePuzzle2dFillSessionOnHost(fixture);
              puzzle2dShellController?.setBrushEngagementPossibles([]);
            } else if (prev === "fill" && tool !== "fill") {
              const base = clearPuzzle2dFillSession(puzzle2dActiveRenderer());
              if (base) {
                patchFixture(() => clonePuzzle2dFixture(base));
              }
            }
            break;
          }
          case "setFillCount": {
            const { count } = args as { count?: number };
            const n = Math.max(0, Math.min(PUZZLE_2D_FILL_COUNT_MAX, Math.round(Number(count) ?? 0)));
            const catalogs = puzzle2dFixtureMergedKindCatalogs(fixture);
            const next = applyPuzzle2dFillCount(n, catalogs);
            if (!next) {
              break;
            }
            patchFixture(() => next);
            console.log("[DEBUG] puzzle2d fill count", n, "applied", n);
            break;
          }
          case "setSuggestionOffset":
            setPuzzle2dSuggestionOffset((args as { distance: number }).distance);
            break;
          case "setBrushKindWeights": {
            const payload = args as { nodeWeights?: Record<string, number>; handleWeights?: Record<string, number> };
            puzzle2dActiveRenderer()?.setBrushKindWeights(payload.nodeWeights ?? {}, payload.handleWeights ?? {});
            break;
          }
          case "pickBrushCandidate": {
            const { index } = args as { index?: number };
            if (typeof index === "number" && Number.isFinite(index)) {
              puzzle2dActiveRenderer()?.setBrushCandidateIndex(index);
            }
            break;
          }
          case "setSelectionFlag": {
            const { flag, value } = args as { flag?: "hidden" | "locked"; value?: boolean };
            if (flag !== "hidden" && flag !== "locked") {
              break;
            }
            const ids = [...selectionIds];
            patchFixture((prev) => puzzle2dPlayApplySelectionFlag(prev, ids, flag, value === true));
            break;
          }
          case "deleteSelection": {
            const ids = [...selectionIds];
            if (!ids.length) {
              break;
            }
            patchFixture((prev) => {
              const next = puzzle2dPlayDeleteSelectionFromFixture(prev, ids);
              puzzle2dSyncFixtureDescriptorToAllAuthoringPeers(next);
              return next;
            });
            setSelectionIds([]);
            break;
          }
          case "duplicateSelection": {
            const ids = [...selectionIds];
            const { fixture: nextFixture, newIds } = puzzle2dPlayDuplicateSelection(fixtureRef.current, ids);
            if (newIds.length === 0) {
              break;
            }
            patchFixture(() => {
              puzzle2dSyncFixtureDescriptorToAllAuthoringPeers(nextFixture);
              return nextFixture;
            });
            setSelectionIds([...newIds]);
            break;
          }
          case "selectSameKind": {
            const ids = puzzle2dPlaySelectSameKindIds(fixtureRef.current, [...selectionIds]);
            if (ids.length > 0) {
              h.setSelectionIds(ids);
            }
            break;
          }
          case "toggleEntityFlag": {
            const { graphId, flag } = args as { graphId?: string; flag?: "hidden" | "locked" };
            if (!graphId || (flag !== "hidden" && flag !== "locked")) {
              break;
            }
            patchFixture((prev) => puzzle2dPlayToggleEntityFlag(prev, graphId, flag));
            break;
          }
          case "setPuzzle2dRedrawMode":
            h.setPuzzle2dRedrawMode((args as { value?: Puzzle2dRedrawModeKind }).value ?? "force-graph");
            break;
          case "setPuzzle2dRedrawHandlesAfterNodes":
            h.setPuzzle2dRedrawHandlesAfterNodes((args as { pressed?: boolean }).pressed ?? false);
            break;
          case "setPuzzle2dRedrawProgressiveEnabled":
            h.setPuzzle2dRedrawProgressiveEnabled((args as { pressed?: boolean }).pressed ?? false);
            break;
          case "setPuzzle2dRedrawProgressiveAutoStopMs":
            h.setPuzzle2dRedrawProgressiveAutoStopMs(Number((args as { value?: number }).value) || 0);
            break;
          case "setPuzzle2dRedrawPlayMaxItersPerFrame":
            h.setPuzzle2dRedrawPlayMaxItersPerFrame(Number((args as { value?: number }).value) || 96);
            break;
          case "setForceLayoutFullIterations":
            h.setForceLayoutFullIterations(Number((args as { value?: number }).value) || 200);
            break;
          case "setForceLayoutIdealEdgeLength":
            h.setForceLayoutIdealEdgeLength(Number((args as { value?: number }).value) || 64);
            break;
          case "setForceLayoutRepulsionStrength":
            h.setForceLayoutRepulsionStrength(Number((args as { value?: number }).value) || 80);
            break;
          case "setForceLayoutGravity":
            h.setForceLayoutGravity(Number((args as { value?: number }).value) || 0);
            break;
          case "setTreeLayoutLayerSpacing":
            h.setTreeLayoutLayerSpacing(Number((args as { value?: number }).value) || 120);
            break;
          case "setTreeLayoutSiblingGap":
            h.setTreeLayoutSiblingGap(Number((args as { value?: number }).value) || 28);
            break;
          case "setTreeLayoutDirection":
            h.setTreeLayoutDirection((args as { value?: Puzzle2dHierarchicalTreeDirectionKind }).value ?? "downwards");
            break;
          case "applyPuzzle2dRedrawOnce":
            h.applyPuzzle2dRedrawOnce();
            break;
          case "applyPuzzle2dRedrawHandlesOnce":
            h.applyPuzzle2dRedrawHandlesOnce();
            break;
          case "patchInspectorNodes": {
            const payload = args as { ids?: readonly string[]; field?: string; value?: unknown; delta?: number };
            const ids = payload.ids ?? [];
            const idSet = new Set(ids);
            const catalogs = puzzle2dFixtureMergedKindCatalogs(fixtureRef.current);
            h.patchFixture((prev) => ({
              ...prev,
              nodes: prev.nodes.map((node) => {
                if (!idSet.has(node.id)) return node;
                switch (payload.field) {
                  case "text": {
                    const trimmed = String(payload.value ?? "").trim();
                    return trimmed === "" ? { ...node, text: undefined } : { ...node, text: trimmed };
                  }
                  case "nodeKind":
                    return puzzle2dApplyNodeKindToFixtureNode(node, String(payload.value ?? ""), catalogs);
                  case "iconKind": {
                    const t = String(payload.value ?? "").trim();
                    return t === "" ? { ...node, iconKind: undefined } : { ...node, iconKind: t };
                  }
                  case "x":
                    return { ...node, x: Number(payload.value) };
                  case "xDelta":
                    return { ...node, x: node.x + Number(payload.delta ?? 0) };
                  case "y":
                    return { ...node, y: Number(payload.value) };
                  case "yDelta":
                    return { ...node, y: node.y + Number(payload.delta ?? 0) };
                  default:
                    return node;
                }
              }),
            }));
            break;
          }
          case "patchInspectorHandles": {
            const payload = args as { ids?: readonly string[]; field?: string; value?: unknown; delta?: number; parentNodeId?: string; t?: number };
            const ids = payload.ids ?? [];
            const idSet = new Set(ids);
            h.patchFixture((prev) => ({
              ...prev,
              nodes: prev.nodes.map((node) => ({
                ...node,
                handles: node.handles.map((handle) => {
                  if (!idSet.has(handle.id)) return handle;
                  switch (payload.field) {
                    case "handleKind": {
                      const trimmed = String(payload.value ?? "").trim();
                      return trimmed === "" ? handle : { ...handle, handleKind: trimmed };
                    }
                    case "iconKind": {
                      const t = String(payload.value ?? "").trim();
                      return t === "" ? { ...handle, iconKind: undefined } : { ...handle, iconKind: t };
                    }
                    case "angle":
                      return { ...handle, angle: normalizeAngleRad(Number(payload.value)) };
                    case "angleDelta":
                      return { ...handle, angle: normalizeAngleRad(handle.angle + Number(payload.delta ?? 0)) };
                    case "radius":
                      return { ...handle, radius: Math.max(1e-6, Number(payload.value)) };
                    case "radiusDelta":
                      return { ...handle, radius: Math.max(1e-6, (handle.radius ?? 8) + Number(payload.delta ?? 0)) };
                    case "ringT": {
                      const parentNode = payload.parentNodeId ? findNode(prev, payload.parentNodeId) : undefined;
                      if (!parentNode) return handle;
                      const nextT = typeof payload.t === "number" ? payload.t : Number(payload.value);
                      return { ...handle, angle: normalizeAngleRad(puzzle2dHandleAngleFromRingT(parentNode, nextT)) };
                    }
                    default:
                      return handle;
                  }
                }),
              })),
            }));
            break;
          }
          case "patchInspectorEdges": {
            const payload = args as { ids?: readonly string[]; field?: string; value?: unknown };
            const ids = payload.ids ?? [];
            const idSet = new Set(ids);
            h.patchFixture((prev) => ({
              ...prev,
              edges: prev.edges.map((edge) => {
                if (!idSet.has(edge.id)) return edge;
                switch (payload.field) {
                  case "edgeKind": {
                    const trimmed = String(payload.value ?? "").trim();
                    if (trimmed === "") {
                      const { edgeKind: _drop, ...rest } = edge;
                      return rest;
                    }
                    return { ...edge, edgeKind: trimmed };
                  }
                  case "source":
                    return { ...edge, source: String(payload.value ?? "") };
                  case "target":
                    return { ...edge, target: String(payload.value ?? "") };
                  default:
                    return edge;
                }
              }),
            }));
            break;
          }
          default:
            break;
        }
      },
    };
    puzzle2dShellController.setHostBridge(bridge);
    return () => puzzle2dShellController.setHostBridge(null);
  }, [
    applyPuzzle2dRedrawHandlesOnce,
    puzzle2dActiveTool,
    puzzle2dSuggestionOffset,
    puzzle2dGridSnapEnabled,
    puzzle2dRedrawPlaying,
    puzzle2dSelectionMethod,
    puzzle2dSelectionMode,
    puzzle2dSelectionTargets,
    puzzle2dShellController,
    preparePuzzle2dFillSessionOnHost,
    fixture,
    patchFixture,
    selectionIds,
    setPuzzle2dActiveTool,
    setPuzzle2dSuggestionOffset,
    setSelectionIds,
  ]);
  // #endregion 🔖ToolbarHostBridge

  const puzzle2dPlayHierarchyPanel = reactHostPort.useMemo(() => new Puzzle2dPlayHierarchyPanelDefinition(), []);
  const puzzle2dPlayKindsPanel = reactHostPort.useMemo(() => new Puzzle2dPlayKindsPanelDefinition(), []);
  const puzzle2dPlayInspectorPanel = reactHostPort.useMemo(() => new Puzzle2dPlayInspectorPanelDefinition(), []);
  const augmentPanelTabs = reactHostPort.useMemo(
    () => ({
      workbench: [puzzle2dPlayHierarchyPanel, puzzle2dPlayKindsPanel],
      details: [puzzle2dPlayInspectorPanel],
    }),
    [puzzle2dPlayHierarchyPanel, puzzle2dPlayKindsPanel, puzzle2dPlayInspectorPanel],
  );

  const applyNavbarFixtureId = reactHostPort.useCallback(
    (fixtureId: string) => {
      const nextId = isPlaygroundNoExampleId(fixtureId) ? PLAYGROUND_NO_EXAMPLE_ID : fixtureId;
      if (nextId === activeExampleId) return;
      setActiveExampleId(nextId);
      const next = puzzle2dPlayFixtureForNavbarId(nextId);
      catalogRawFixtureRef.current = puzzle2dPlayRawFixtureJsonForNavbarId(nextId);
      setFixtureState(next);
      setSelectionIdsState(isPlaygroundNoExampleId(nextId) ? new Set() : selectionSeedForFixture(next));
      setPuzzle2dPlayPaneCamerasBaseline(triptychCamerasForFixture(next));
      puzzle2dSyncFixtureDescriptorToAllAuthoringPeers(next);
      bumpSceneAuthoringEpoch();
    },
    [activeExampleId, bumpSceneAuthoringEpoch, triptychCamerasForFixture],
  );

  const slotNavbarCenter = reactHostPort.useMemo(() => {
    if (isPlaygroundExampleLocked()) return null;
    return (
      <NavbarExampleSelect
        id="puzzle2d.play.fixture"
        value={activeExampleId}
        options={PUZZLE_2D_PLAY_NAVBAR_EXAMPLE_OPTIONS}
        onValueChange={applyNavbarFixtureId}
      />
    );
  }, [activeExampleId, applyNavbarFixtureId]);

  puzzle2dPlayRuntimeRef.current = puzzle2dRuntime;
  puzzle2dPlayShellControllerRef.current = puzzle2dShellController ?? null;
  puzzle2dPlayShellRef.current = shellValue;
  puzzle2dPlaySelectionRef.current = selectionValue;
  reactHostPort.useEffect(
    () => () => {
      puzzle2dPlayShellRef.current = null;
      puzzle2dPlaySelectionRef.current = null;
      puzzle2dPlayRuntimeRef.current = null;
      puzzle2dPlayShellControllerRef.current = null;
    },
    [],
  );

  return (
    <Puzzle2dPlayShellContext.Provider value={shellValue}>
      <Puzzle2dPlaySelectionContext.Provider value={selectionValue}>
        <Puzzle2dPlayCanvasSelectionContext.Provider value={canvasSelectionValue}>
          <Puzzle2dPlayCamerasContext.Provider value={camerasValue}>
            <Puzzle2dPlayLodRuntimeContext.Provider value={setPuzzle2dEffectiveLodForPane}>
              <PlaygroundView runtime={puzzle2dRuntime} defaultAppId={PUZZLE_2D_PLAY_APP_ID} augmentPanelTabs={augmentPanelTabs} playgroundKeybindings={playgroundKeybindings} onActiveWindowChange={onPuzzle2dPlayActiveWindowChange} slotNavbarCenter={slotNavbarCenter} />
            </Puzzle2dPlayLodRuntimeContext.Provider>
          </Puzzle2dPlayCamerasContext.Provider>
        </Puzzle2dPlayCanvasSelectionContext.Provider>
      </Puzzle2dPlaySelectionContext.Provider>
    </Puzzle2dPlayShellContext.Provider>
  );
}

function Puzzle2dPlayChrome({ playground }: { readonly playground: Playground }): ReactElement {
  return <Puzzle2dPlayInner puzzle2dRuntime={playground.runtime} playgroundKeybindings={playground.keybindings} />;
}

/** @emoji 🚀 Mounts puzzle 2d play chrome for a {@link Playground}. */
export function mountPuzzle2dPlayChrome(playground: Playground, rootId = "root"): void {
  mountPlaygroundApp(<Puzzle2dPlayChrome playground={playground} />, rootId);
}

const puzzle2dPlayChromeBoot: PlaygroundChromeBoot = {
  registerHosts: registerPuzzle2dPlaySurfaceHosts,
  mount: mountPuzzle2dPlayChrome,
};

/** @emoji 🛝 Puzzle 2D play entry: register hosts, bodies, mount chrome (from `puzzle/2d/play/index.ts`). */
export function boot2dPlay(playground: Playground, rootId = "root"): void {
  bootPlayground(playground, puzzle2dPlayChromeBoot, rootId);
}

/** @emoji 🔗 WIRES play entry: shared play chrome with WIRES fixture and domain labels (`reasoning/mindmap/wires/play`). */
export function bootWiresPlay(playground: Playground, rootId = "root"): void {
  boot2dPlay(playground, rootId);
}

// #endregion 🔖Entrypoint

// #endregion 🛝PlayHost
//#endregion 🔖Puzzle2dPlayHost

//#region 🔖MapPlayHost
import type { UiGisMapHostSurfaceNode } from "@semio-tech/framework-platform-core";
import {
    GIS_MAP_PLAY_APP_ID,
    GIS_MAP_PLAY_BODY_KEY_MAIN,
    GIS_MAP_PLAY_CATALOGUE_TAB_ID,
    GIS_MAP_PLAY_HIERARCHY_TAB_ID,
    GIS_MAP_PLAY_IDLE_SNAPSHOT,
    GIS_MAP_PLAY_INSPECTION_TAB_ID,
    GIS_MAP_PLAY_STORE_ID,
    GIS_MAP_PLAY_SURFACE_ID,
    GIS_MAP_PLAY_WINDOW_KIND_ID,
    buildMapPlayCatalogueTree,
    buildMapPlayHierarchyTree,
    buildMapPlayInspectorTree,
    buildMapPlayMainDeclarativeBody,
    parseGisMapFixtureV1,
    type MapPlayController
} from "@semio-tech/gis-2d-core";
import { MapCanvas, Position, Route, type GisMapLodId, type MapContextMenuContext, type MapHoveredFeature, type MapSelectPayload } from "@semio-tech/gis-2d-react";

let mapPlayChromeRegistered = false;
const mapPlayControllerRef: { current: MapPlayController | null } = { current: null };

function useMapPlayController(runtimeOverride?: Platform): MapPlayController | undefined {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const runtime = runtimeOverride ?? appCtx?.runtime;
  reactHostPort.useSyncExternalStore(
    (listener) => (runtime ? runtime.subscribeChrome(listener) : () => {}),
    () => runtime?.chromeGeneration ?? 0,
    () => 0,
  );
  const ctrl = runtime?.getActiveApp()?.controller as MapPlayController | undefined;
  mapPlayControllerRef.current = ctrl ?? null;
  return ctrl;
}

function useMapPlayInteractionRevision(runtime: Platform): number {
  return reactHostPort.useSyncExternalStore(
    (listener) => {
      const ctrl = runtime.getActiveApp()?.controller as MapPlayController | undefined;
      mapPlayControllerRef.current = ctrl ?? null;
      const unsubscribeRuntime = runtime.subscribe(listener);
      const unsubscribeSnapshot =
        ctrl && typeof ctrl.subscribeSnapshot === "function" ? ctrl.subscribeSnapshot(listener) : undefined;
      return () => {
        unsubscribeRuntime();
        unsubscribeSnapshot?.();
      };
    },
    () => (runtime.getActiveApp()?.controller as MapPlayController | undefined)?.getInteractionRevision() ?? 0,
    () => 0,
  );
}

function useMapPlaySnapshot() {
  const ctrl = useMapPlayController();
  return useControllerStore(ctrl, GIS_MAP_PLAY_STORE_ID) ?? GIS_MAP_PLAY_IDLE_SNAPSHOT;
}

function buildMapPlayContextMenuItems(ctrl: MapPlayController | null | undefined, context: MapContextMenuContext): ContextMenuItem[] {
  if (!ctrl) {
    return [];
  }
  const { feature } = context;
  if (feature) {
    const selected =
      feature.kind === "position"
        ? ctrl.getSelectedPositionIds().includes(feature.id)
        : ctrl.getSelectedRouteIds().includes(feature.id);
    const items: ContextMenuItem[] = [
      {
        id: "gis-map.ctx.select",
        label: "Select",
        onSelect: () => ctrl.run("setSelection", { positions: feature.kind === "position" ? [feature.id] : [], routes: feature.kind === "route" ? [feature.id] : [], mode: "default" }),
      },
    ];
    if (selected) {
      items.push({
        id: "gis-map.ctx.deselect",
        label: "Deselect",
        onSelect: () => ctrl.run("deselect", { featureId: feature.id, featureKind: feature.kind }),
      });
    }
    items.push({
      id: "gis-map.ctx.focus",
      label: "Focus / zoom to",
      onSelect: () => ctrl.run("focusFeature", { featureId: feature.id, featureKind: feature.kind }),
    });
    if (feature.kind === "position") {
      const position = ctrl.getActiveFixture()?.positions.find((row) => row.id === feature.id);
      if (position?.sourceUrl) {
        items.push({
          id: "gis-map.ctx.source",
          label: "Open source",
          onSelect: () => ctrl.run("openSource", { featureId: feature.id }),
        });
      }
    }
    return items;
  }
  return [
    {
      id: "gis-map.ctx.select-all",
      label: "Select all",
      onSelect: () => ctrl.run("selectAll"),
    },
    {
      id: "gis-map.ctx.clear",
      label: "Clear selection",
      disabled: ctrl.getSelectedPositionIds().length + ctrl.getSelectedRouteIds().length === 0,
      onSelect: () => ctrl.run("clearSelection"),
    },
    {
      id: "gis-map.ctx.fit-world",
      label: "Fit world",
      onSelect: () => ctrl.run("fitWorld"),
    },
  ];
}

function MapPlayPaneSurfaceHost({ node: _node }: { readonly node: UiGisMapHostSurfaceNode }): ReactElement {
  const shellInstance = useShellWindowInstance();
  const scopeId = shellWindowScopeId(shellInstance, GIS_MAP_PLAY_WINDOW_KIND_ID);
  const ctrl = useMapPlayController();
  const snapshot = useMapPlaySnapshot();
  const activeFixture = snapshot.activeFixture ?? ctrl?.getActiveFixture() ?? null;
  const selectedPositionIds = snapshot.selectedPositionIds ?? ctrl?.getSelectedPositionIds() ?? [];
  const selectedRouteIds = snapshot.selectedRouteIds ?? ctrl?.getSelectedRouteIds() ?? [];
  const hoveredFeature = snapshot.hoveredFeature ?? ctrl?.getHoveredFeature() ?? null;
  const selectionMethod = snapshot.selectionMethod ?? ctrl?.getSelectionMethod() ?? "rectangle";
  const fitWorldRevision = snapshot.fitWorldRevision ?? ctrl?.getFitWorldRevision() ?? 0;
  const renderMode = ctrl?.getRenderModeForScope(scopeId) ?? snapshot.renderModeByInstance[scopeId] ?? snapshot.renderMode;
  const vectorStyle = ctrl?.getVectorStyleForScope(scopeId) ?? snapshot.vectorStyleByInstance[scopeId] ?? snapshot.vectorStyle;
  const lodMode = ctrl?.getLodModeForScope(scopeId) ?? snapshot.lodModeByInstance[scopeId] ?? snapshot.lodMode;
  const layerVisibility = ctrl?.getLayerVisibilityForScope(scopeId) ?? snapshot.layerVisibilityByInstance[scopeId] ?? snapshot.layerVisibility;
  const layerStrokeScale = ctrl?.getLayerStrokeScaleForScope(scopeId) ?? snapshot.layerStrokeScaleByInstance[scopeId] ?? snapshot.layerStrokeScale;
  const reportEffectiveLod = reactHostPort.useCallback(
    (lodId: GisMapLodId) => {
      ctrl?.run("setEffectiveLod", { lod: lodId, instanceId: scopeId });
    },
    [ctrl, scopeId],
  );
  const handleSelect = reactHostPort.useCallback(
    (payload: MapSelectPayload) => {
      ctrl?.run("setSelection", {
        positions: [...payload.positions],
        routes: [...payload.routes],
        mode: payload.mode,
      });
    },
    [ctrl],
  );
  const handleHoverChange = reactHostPort.useCallback(
    (feature: MapHoveredFeature | null) => {
      ctrl?.run("setHover", {
        featureId: feature?.id ?? null,
        featureKind: feature?.kind ?? null,
      });
    },
    [ctrl],
  );
  const getContextMenuItems = reactHostPort.useCallback(
    (context: MapContextMenuContext) => buildMapPlayContextMenuItems(ctrl, context),
    [ctrl],
  );
  reactHostPort.useEffect(() => {
    if (!activeFixture) {
      return;
    }
    console.log(
      `[DEBUG] gis map fixture loaded: ${activeFixture.positions.length} positions, ${activeFixture.routes.length} routes`,
    );
  }, [activeFixture]);
  return (
    <MapCanvas
      renderMode={renderMode}
      vectorStyle={vectorStyle}
      lodMode={lodMode}
      layerVisibility={layerVisibility}
      layerStrokeScale={layerStrokeScale}
      onEffectiveLodChange={reportEffectiveLod}
      selectedPositionIds={selectedPositionIds}
      selectedRouteIds={selectedRouteIds}
      hoveredFeature={hoveredFeature}
      selectionMethod={selectionMethod}
      onSelect={handleSelect}
      onHoverChange={handleHoverChange}
      getContextMenuItems={getContextMenuItems}
      fitWorldRevision={fitWorldRevision}
    >
      {activeFixture?.positions.map((position) => (
        <Position
          key={position.id}
          id={position.id}
          lon={position.lon}
          lat={position.lat}
          label={position.label}
          name={position.name}
          icon={position.icon}
          sourceUrl={position.sourceUrl}
          kind={position.kind}
        />
      ))}
      {activeFixture?.routes.map((route) => (
        <Route key={route.id} id={route.id} points={route.points} />
      ))}
    </MapCanvas>
  );
}

export function registerMapPlaySurfaceHosts(): void {
  if (mapPlayChromeRegistered) return;
  mapPlayChromeRegistered = true;
  registerUiGisMapSurfaceHost(GIS_MAP_PLAY_SURFACE_ID, MapPlayPaneSurfaceHost);
  registerWindowBody(GIS_MAP_PLAY_BODY_KEY_MAIN, buildMapPlayMainDeclarativeBody);
}

class MapPlayHierarchyPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: GIS_MAP_PLAY_HIERARCHY_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = mapPlayControllerRef.current;
        const bus = new CommandBus();
        const treeNode = buildMapPlayHierarchyTree(
          ctrl?.getActiveFixture() ?? null,
          ctrl?.getSelectedPositionIds() ?? [],
          ctrl?.getSelectedRouteIds() ?? [],
          ctrl?.getHoveredFeature() ?? null,
          (payload) => ctrl?.run("setHover", payload),
        );
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

class MapPlayCataloguePanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: GIS_MAP_PLAY_CATALOGUE_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
      order: 1,
      tree: new CallbackTreePanelDefinition(() => {
        const bus = new CommandBus();
        return uiTreeNodeToTreePanelConfig(buildMapPlayCatalogueTree(), bus);
      }),
    };
  }
}

class MapPlayInspectionPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: GIS_MAP_PLAY_INSPECTION_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, "details"),
      name: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = mapPlayControllerRef.current;
        const bus = new CommandBus();
        const treeNode = buildMapPlayInspectorTree(
          ctrl?.getActiveFixture() ?? null,
          ctrl?.getSelectedPositionIds() ?? [],
          ctrl?.getSelectedRouteIds() ?? [],
        );
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

function MapPlayInner({ runtime }: { readonly runtime: Platform }): ReactElement {
  useMapPlayController(runtime);
  const interactionRevision = useMapPlayInteractionRevision(runtime);
  const mapPlayHierarchyPanel = reactHostPort.useMemo(() => new MapPlayHierarchyPanelDefinition(), []);
  const mapPlayCataloguePanel = reactHostPort.useMemo(() => new MapPlayCataloguePanelDefinition(), []);
  const mapPlayInspectionPanel = reactHostPort.useMemo(() => new MapPlayInspectionPanelDefinition(), []);
  const augmentPanelTabs = reactHostPort.useMemo(
    () => ({
      workbench: [mapPlayHierarchyPanel, mapPlayCataloguePanel],
      details: [mapPlayInspectionPanel],
    }),
    [interactionRevision, mapPlayCataloguePanel, mapPlayHierarchyPanel, mapPlayInspectionPanel],
  );
  return <PlaygroundView runtime={runtime} defaultAppId={GIS_MAP_PLAY_APP_ID} augmentPanelTabs={augmentPanelTabs} />;
}

function MapPlayChrome({ runtime }: { readonly runtime: Platform }): ReactElement {
  return <MapPlayInner runtime={runtime} />;
}

export function mountMapPlayChrome(playground: Playground, rootId = "root"): void {
  mountPlaygroundApp(<MapPlayChrome runtime={playground.runtime} />, rootId);
}

const mapPlayChromeBoot: PlaygroundChromeBoot = {
  registerHosts: registerMapPlaySurfaceHosts,
  mount: mountMapPlayChrome,
};

export function bootMapPlay(playground: Playground, rootId = "root"): void {
  bootPlayground(playground, mapPlayChromeBoot, rootId);
}
//#endregion 🔖MapPlayHost

//#region 🔖FlowPlayHost
import {
    FLOW_PLAY_APP_ID,
    FLOW_PLAY_CATALOGUE_TAB_ID,
    FLOW_PLAY_DEFAULT_FIXTURE_JSON,
    FLOW_PLAY_HIERARCHY_TAB_ID,
    FLOW_PLAY_INSPECTION_TAB_ID,
    FLOW_PLAY_SURFACE_ID,
    FLOW_PLAY_SURFACE_ID_GENERATE,
    FLOW_PLAY_SURFACE_ID_JACK,
    FLOW_PLAY_SURFACE_ID_COMPILED_DAG,
    FLOW_PLAY_WINDOW_KIND_ID,
    FlowPlayController,
    buildFlowPlayCanvasContextMenu,
    buildFlowPlayCatalogueTree,
    buildFlowPlayHierarchyTree,
    buildFlowPlayInspectorTree,
    registerFlowPlayDeclarativeBodies
} from "@semio-tech/flow-core";
import {
    DAG_LOD_MODE_AUTOMATIC,
    FLOW_DEFAULT_PROXIMITY_DISTANCE,
    FLOW_WIDGET_DRAG_MIME,
    FlowCanvas,
    dagLodCanvasProps,
    ensureFlowWasmLoaded,
    flowWidgetPaletteTreeDragController,
} from "@semio-tech/flow-react";
import { canvasDrawingPngExportPort } from "@semio-tech/procedural-2d-react";
import { FlowGenerateSurface } from "@semio-tech/forms-react";
import { parseFormSpec } from "@semio-tech/forms-core";
import type { UiFlowHostSurfaceNode, UiFormsHostSurfaceNode } from "@semio-tech/framework-platform-core";

let flowPlayChromeRegistered = false;
const flowPlayControllerRef: { current: FlowPlayController | null } = { current: null };

type FlowExportMesh = {
  readonly position?: readonly number[];
  readonly index?: readonly number[];
  readonly error?: string;
};

function flowExportCollectHandle(value: unknown): string | null {
  if (typeof value === "string" && value.length > 0) return value;
  if (!value || typeof value !== "object") return null;
  const record = value as Record<string, unknown>;
  if (typeof record.handle === "string" && record.handle.length > 0) return record.handle;
  for (const nested of Object.values(record)) {
    const handle = flowExportCollectHandle(nested);
    if (handle) return handle;
  }
  return null;
}

function flowExportMeshToObj(mesh: FlowExportMesh): string {
  const position = mesh.position ?? [];
  const index = mesh.index ?? [];
  const lines = ["# semio flow export"];
  for (let i = 0; i + 2 < position.length; i += 3) {
    lines.push(`v ${position[i]} ${position[i + 1]} ${position[i + 2]}`);
  }
  for (let i = 0; i + 2 < index.length; i += 3) {
    lines.push(`f ${index[i]! + 1} ${index[i + 1]! + 1} ${index[i + 2]! + 1}`);
  }
  return `${lines.join("\n")}\n`;
}

async function downloadFlowOutputExport(format: string, resolvedValueJson: string, widgetId: string): Promise<void> {
  await ensureFlowWasmLoaded();
  const { export_drawing_svg, render_drawing_scene, tessellate } = await import("@semio-tech/flow-core/pkg/flow_core.js");
  const parsed = JSON.parse(resolvedValueJson) as unknown;
  const handle = flowExportCollectHandle(parsed);
  const normalized = format.trim().toLowerCase();
  const baseName = `flow-export-${widgetId}`;
  if (!handle) {
    console.log(`[DEBUG] flow export skipped: no media handle in payload for ${widgetId}`);
    return;
  }
  if (normalized === "svg") {
    const payload = JSON.parse(export_drawing_svg(handle)) as { svg?: string; error?: string };
    if (!payload.svg) throw new Error(payload.error ?? "svg export failed");
    downloadMediaExportResult({ data: payload.svg, mimeType: "image/svg+xml", fileName: `${baseName}.svg` });
    return;
  }
  if (normalized === "png") {
    const sceneJson = render_drawing_scene(handle);
    const scene = JSON.parse(sceneJson) as { error?: string };
    if (scene.error) throw new Error(scene.error);
    const png = canvasDrawingPngExportPort.exportPng(scene);
    downloadMediaExportResult({ data: png, mimeType: "image/png", fileName: `${baseName}.png` });
    return;
  }
  const mesh = JSON.parse(tessellate(handle, 0.25)) as FlowExportMesh;
  if (mesh.error) throw new Error(mesh.error);
  if (normalized === "obj") {
    downloadMediaExportResult({ data: flowExportMeshToObj(mesh), mimeType: "text/plain", fileName: `${baseName}.obj` });
    return;
  }
  if (normalized === "glb") {
    downloadMediaExportResult({ data: JSON.stringify(mesh), mimeType: "application/json", fileName: `${baseName}.mesh.json` });
    return;
  }
  throw new Error(`unsupported export format: ${format}`);
}

/** @emoji 🔔 Re-renders flow play workbench kinds when WASM catalogue sections arrive. */
function useFlowPlaySnapshotRevision(runtime: Platform, selector: (ctrl: FlowPlayController) => number): number {
  return reactHostPort.useSyncExternalStore(
    (listener) => {
      const ctrl = runtime.getActiveApp()?.controller as FlowPlayController | undefined;
      flowPlayControllerRef.current = ctrl ?? null;
      const unsubscribeChrome = runtime.subscribeChrome(listener);
      const unsubscribeSnapshot = ctrl?.subscribeSnapshot(listener);
      return () => {
        unsubscribeChrome();
        unsubscribeSnapshot?.();
      };
    },
    () => {
      const ctrl = runtime.getActiveApp()?.controller as FlowPlayController | undefined;
      flowPlayControllerRef.current = ctrl ?? null;
      return ctrl ? selector(ctrl) : 0;
    },
    () => 0,
  );
}

function useFlowPlayCatalogueRevision(runtime: Platform): number {
  return useFlowPlaySnapshotRevision(runtime, (c) => c.getCatalogueRevision());
}

function useFlowPlayExtensionRevision(runtime: Platform): number {
  return useFlowPlaySnapshotRevision(runtime, (c) => c.getExtensionRevision());
}

function useFlowPlayInteractionRevision(runtime: Platform): number {
  return useFlowPlaySnapshotRevision(runtime, (c) => c.getInteractionRevision());
}

function useFlowPlayController(runtimeOverride?: Platform): FlowPlayController | undefined {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const runtime = runtimeOverride ?? appCtx?.runtime;
  reactHostPort.useSyncExternalStore(
    (listener) => (runtime ? runtime.subscribeChrome(listener) : () => {}),
    () => runtime?.chromeGeneration ?? 0,
    () => 0,
  );
  const ctrl = runtime?.getActiveApp()?.controller as FlowPlayController | undefined;
  flowPlayControllerRef.current = ctrl ?? null;
  return ctrl;
}

function FlowPlayPaneSurfaceHost({ node }: { readonly node: UiFlowHostSurfaceNode }): ReactElement {
  const { runtime } = useApp();
  const ctrl = useFlowPlayController();
  const extensionRevision = useFlowPlayExtensionRevision(runtime);
  const scopeId = node.paneId ?? FLOW_PLAY_WINDOW_KIND_ID;
  const lodProps = dagLodCanvasProps(ctrl?.lodModeForScope(scopeId) ?? DAG_LOD_MODE_AUTOMATIC);
  const proximityDistance = ctrl?.proximityDistanceValue() ?? FLOW_DEFAULT_PROXIMITY_DISTANCE;
  const onLodChange = reactHostPort.useCallback(
    (lod: import("@semio-tech/flow-react").DagDrawLodKind) => {
      ctrl?.run("setEffectiveLod", { lod, instanceId: scopeId });
    },
    [ctrl, scopeId],
  );
  const onPreviewText = reactHostPort.useCallback(
    (text: string) => {
      console.log(`[DEBUG] flow play preview: ${text}`);
      ctrl?.run("setPreviewText", { text });
    },
    [ctrl],
  );
  const onCatalogueReady = reactHostPort.useCallback(
    (sections: readonly import("@semio-tech/flow-react").CatalogueSection[]) => {
      ctrl?.run("setCatalogueSections", { sections: [...sections] });
    },
    [ctrl],
  );
  const onFixtureChange = reactHostPort.useCallback(
    (json: string) => {
      ctrl?.run("setFixtureJson", { json });
    },
    [ctrl],
  );
  const onCanvasCommand = reactHostPort.useCallback(
    (command: string, args?: Record<string, unknown>) => {
      ctrl?.run(command, args);
    },
    [ctrl],
  );
  const onSelectionChange = reactHostPort.useCallback(
    (ids: readonly string[]) => {
      ctrl?.run("setSelection", { ids: [...ids] });
    },
    [ctrl],
  );
  const onCompiledWireLiteralChange = reactHostPort.useCallback(
    (text: string) => {
      ctrl?.run("setCompiledWireLiteral", { text });
    },
    [ctrl],
  );
  const onOutputExport = reactHostPort.useCallback(
    (widgetId: string, format: string, resolvedValueJson: string) => {
      void downloadFlowOutputExport(format, resolvedValueJson, widgetId).catch((error) => {
        console.log(`[DEBUG] flow play export failed: ${String(error)}`);
      });
    },
    [],
  );
  return (
    <FlowCanvas
      fixtureJson={ctrl?.getFixtureJson() ?? FLOW_PLAY_DEFAULT_FIXTURE_JSON}
      fixtureDragDrop
      reorganize={ctrl?.getReorganize()}
      commandRequest={ctrl?.getCommandRequest()}
      extensionRevision={extensionRevision}
      onPreviewText={onPreviewText}
      onCatalogueReady={onCatalogueReady}
      onFixtureChange={onFixtureChange}
      onCompiledWireLiteralChange={onCompiledWireLiteralChange}
      onOutputExport={onOutputExport}
      contextMenu={(ctx) => buildFlowPlayCanvasContextMenu(ctx, onCanvasCommand)}
      selectedNodeIds={ctrl?.getSelectedNodeIds()}
      onSelectionChange={onSelectionChange}
      {...lodProps}
      onLodChange={onLodChange}
      proximityDistance={proximityDistance}
    />
  );
}

function FlowPlayJackSurfaceHost({ node: _node }: { readonly node: import("@semio-tech/framework-platform-core").UiWriterHostSurfaceNode }): ReactElement {
  const ctrl = useFlowPlayController();
  void ctrl?.getHoverEpoch();
  void ctrl?.getSelectEpoch();
  const document = ctrl?.getWriterDocumentJack() ?? createWriterDocument({ id: "flow-jack", languageId: "jack", text: "" });
  const onHoverChange = reactHostPort.useCallback((offset: number | null) => {
    flowPlayControllerRef.current?.run("setJackHover", { offset });
  }, []);
  const onSelectionChange = reactHostPort.useCallback((range: { start: number; end: number }) => {
    flowPlayControllerRef.current?.run("setJackSelect", range);
  }, []);
  return (
    <WriterCanvas
      document={document}
      className="h-full"
      onHoverChange={onHoverChange}
      onSelectionChange={onSelectionChange}
      externalHoverOccurrences={ctrl?.getJackHoverOccurrences()}
      externalHoverOccurrencesSignal={ctrl?.getHoverEpoch()}
      externalSelectionOccurrences={ctrl?.getJackSelectOccurrences()}
      externalSelectionOccurrencesSignal={ctrl?.getSelectEpoch()}
    />
  );
}

function FlowPlayCompiledDagSurfaceHost({ node: _node }: { readonly node: import("@semio-tech/framework-platform-core").UiWriterHostSurfaceNode }): ReactElement {
  const { runtime } = useApp();
  const ctrl = useFlowPlayController();
  const interactionRevision = useFlowPlayInteractionRevision(runtime);
  const document = reactHostPort.useMemo(
    () => ctrl?.getWriterDocumentCompiledDag() ?? createWriterDocument({ id: "flow-compiled-dag", languageId: "wire", text: "" }),
    [ctrl, interactionRevision],
  );
  return <WriterCanvas document={document} className="h-full min-h-0" />;
}

class FlowPlayHierarchyPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: FLOW_PLAY_HIERARCHY_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = flowPlayControllerRef.current;
        const bus = new CommandBus();
        const treeNode = buildFlowPlayHierarchyTree(ctrl?.getFixtureJson() ?? FLOW_PLAY_DEFAULT_FIXTURE_JSON, ctrl?.getSelectedNodeIds() ?? []);
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

class FlowPlayCataloguePanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: FLOW_PLAY_CATALOGUE_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
      order: 1,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = flowPlayControllerRef.current;
        const bus = new CommandBus();
        const treeNode = buildFlowPlayCatalogueTree(ctrl?.getCatalogueSections() ?? [], ctrl?.getExtensionEntries() ?? []);
        const config = uiTreeNodeToTreePanelConfig(treeNode, bus);
        return {
          ...config,
          dragAndDropController: flowWidgetPaletteTreeDragController(collectUiTreeItemDragData(treeNode.sections)),
        };
      }),
    };
  }
}

class FlowPlayInspectionPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: FLOW_PLAY_INSPECTION_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, "details"),
      name: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = flowPlayControllerRef.current;
        const bus = new CommandBus();
        const treeNode = buildFlowPlayInspectorTree(ctrl?.getFixtureJson() ?? FLOW_PLAY_DEFAULT_FIXTURE_JSON, ctrl?.getSelectedNodeIds() ?? []);
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

function FlowPlayGenerateSurfaceHost({ node }: { readonly node: UiFormsHostSurfaceNode }): ReactElement {
  const ctrl = useFlowPlayController();
  const spec = reactHostPort.useMemo(() => {
    try {
      return parseFormSpec(JSON.parse(ctrl?.getGenerateFormSpecJson() ?? "{}"));
    } catch {
      return parseFormSpec({ schema: "forms.form", id: "empty", version: "1", steps: [{ id: "s", title: "Inputs", questions: [] }] });
    }
  }, [ctrl]);
  return (
    <FlowGenerateSurface
      formSpec={spec}
      generations={[...(ctrl?.getGenerations() ?? [])]}
      selectedGenerationId={ctrl?.getSelectedGenerationId() ?? null}
      previewText={ctrl?.getGeneratePreviewText() ?? "—"}
      onSelectGeneration={(id) => ctrl?.run("selectGeneration", { id })}
      onAddGeneration={() => ctrl?.run("addGeneration")}
      onRemoveGeneration={(id) => ctrl?.run("removeGeneration", { id })}
      onGenerationValuesChange={(id, values) => ctrl?.run("updateGenerationValues", { id, values })}
      onRenameGeneration={(id, name) => ctrl?.run("renameGeneration", { id, name })}
      className="h-full"
    />
  );
}

function FlowPlayInner({ runtime }: { readonly runtime: Platform }): ReactElement {
  useFlowPlayController(runtime);
  const catalogueRevision = useFlowPlayCatalogueRevision(runtime);
  const extensionRevision = useFlowPlayExtensionRevision(runtime);
  const interactionRevision = useFlowPlayInteractionRevision(runtime);
  const flowPlayHierarchyPanel = reactHostPort.useMemo(() => new FlowPlayHierarchyPanelDefinition(), []);
  const flowPlayCataloguePanel = reactHostPort.useMemo(() => new FlowPlayCataloguePanelDefinition(), []);
  const flowPlayInspectionPanel = reactHostPort.useMemo(() => new FlowPlayInspectionPanelDefinition(), []);
  const augmentPanelTabs = reactHostPort.useMemo(
    () => ({
      workbench: [flowPlayHierarchyPanel, flowPlayCataloguePanel],
      details: [flowPlayInspectionPanel],
    }),
    [catalogueRevision, extensionRevision, interactionRevision, flowPlayCataloguePanel, flowPlayHierarchyPanel, flowPlayInspectionPanel],
  );
  return <PlaygroundView runtime={runtime} defaultAppId={FLOW_PLAY_APP_ID} augmentPanelTabs={augmentPanelTabs} />;
}

export function registerFlowPlaySurfaceHosts(): void {
  if (flowPlayChromeRegistered) return;
  flowPlayChromeRegistered = true;
  registerUiFlowSurfaceHost(FLOW_PLAY_SURFACE_ID, FlowPlayPaneSurfaceHost);
  registerUiFormsSurfaceHost(FLOW_PLAY_SURFACE_ID_GENERATE, FlowPlayGenerateSurfaceHost);
  registerUiWriterSurfaceHost(FLOW_PLAY_SURFACE_ID_JACK, FlowPlayJackSurfaceHost);
  registerUiWriterSurfaceHost(FLOW_PLAY_SURFACE_ID_COMPILED_DAG, FlowPlayCompiledDagSurfaceHost);
  registerFlowPlayDeclarativeBodies();
}

function FlowPlayChrome({ runtime }: { readonly runtime: Platform }): ReactElement {
  return <FlowPlayInner runtime={runtime} />;
}

export function mountFlowPlayChrome(playground: Playground, rootId = "root"): void {
  mountPlaygroundApp(<FlowPlayChrome runtime={playground.runtime} />, rootId);
}

const flowPlayChromeBoot: PlaygroundChromeBoot = {
  registerHosts: registerFlowPlaySurfaceHosts,
  mount: mountFlowPlayChrome,
};

export function bootFlowPlay(playground: Playground, rootId = "root"): void {
  bootPlayground(playground, flowPlayChromeBoot, rootId);
}
//#endregion 🔖FlowPlayHost

//#region 🔖DagPlayHost
import {
    DAG_PLAY_APP_ID,
    DAG_PLAY_CATALOGUE_TAB_ID,
    DAG_PLAY_DEFAULT_FIXTURE_JSON,
    DAG_PLAY_HIERARCHY_TAB_ID,
    DAG_PLAY_INSPECTION_TAB_ID,
    DAG_PLAY_SURFACE_ID,
    DAG_PLAY_SURFACE_ID_JACK,
    DAG_PLAY_WINDOW_KIND_ID,
    DagPlayController,
    buildDagPlayCatalogueTree,
    buildDagPlayHierarchyTree,
    buildDagPlayInspectorTree,
    registerDagPlayDeclarativeBodies
} from "@semio-tech/dag-host-core";
import { DAG_LOD_MODE_AUTOMATIC as DAG_HOST_LOD_AUTOMATIC, DagCanvas } from "@semio-tech/dag-react";
import type { UiDagHostSurfaceNode } from "@semio-tech/framework-platform-core";

let dagPlayChromeRegistered = false;
const dagPlayControllerRef: { current: DagPlayController | null } = { current: null };

function useDagPlayController(runtimeOverride?: Platform): DagPlayController | undefined {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const runtime = runtimeOverride ?? appCtx?.runtime;
  reactHostPort.useSyncExternalStore(
    (listener) => (runtime ? runtime.subscribeChrome(listener) : () => {}),
    () => runtime?.chromeGeneration ?? 0,
    () => 0,
  );
  const ctrl = runtime?.getActiveApp()?.controller as DagPlayController | undefined;
  dagPlayControllerRef.current = ctrl ?? null;
  return ctrl;
}

function useDagPlayInteractionRevision(runtime: Platform): number {
  return reactHostPort.useSyncExternalStore(
    (listener) => {
      const ctrl = runtime.getActiveApp()?.controller as DagPlayController | undefined;
      dagPlayControllerRef.current = ctrl ?? null;
      const unsubscribeRuntime = runtime.subscribe(listener);
      const unsubscribeSnapshot = ctrl?.subscribeSnapshot(listener);
      return () => {
        unsubscribeRuntime();
        unsubscribeSnapshot?.();
      };
    },
    () => (runtime.getActiveApp()?.controller as DagPlayController | undefined)?.getInteractionRevision() ?? 0,
    () => 0,
  );
}

function DagPlayPaneSurfaceHost({ node }: { readonly node: UiDagHostSurfaceNode }): ReactElement {
  const ctrl = useDagPlayController();
  const scopeId = node.paneId ?? DAG_PLAY_WINDOW_KIND_ID;
  const lodProps = dagLodCanvasProps(ctrl?.lodModeForScope(scopeId) ?? DAG_HOST_LOD_AUTOMATIC);
  const onLodChange = reactHostPort.useCallback(
    (lod: import("@semio-tech/dag-react").DagDrawLodKind) => {
      ctrl?.run("setEffectiveLod", { lod, instanceId: scopeId });
    },
    [ctrl, scopeId],
  );
  const onFixtureChange = reactHostPort.useCallback(
    (json: string) => {
      ctrl?.run("setFixtureJson", { json });
    },
    [ctrl],
  );
  console.log("[DEBUG] dag play surface mount");
  return (
    <DagCanvas
      fixtureJson={ctrl?.getFixtureJson() ?? DAG_PLAY_DEFAULT_FIXTURE_JSON}
      reorganize={ctrl?.getReorganize()}
      onFixtureChange={onFixtureChange}
      {...lodProps}
      onLodChange={onLodChange}
    />
  );
}

function DagPlayJackSurfaceHost({ node: _node }: { readonly node: import("@semio-tech/framework-platform-core").UiWriterHostSurfaceNode }): ReactElement {
  const ctrl = useDagPlayController();
  void ctrl?.getHoverEpoch();
  void ctrl?.getSelectEpoch();
  const document = ctrl?.getWriterDocumentJack() ?? createWriterDocument({ id: "dag-jack", languageId: "jack", text: "" });
  const onHoverChange = reactHostPort.useCallback((offset: number | null) => {
    dagPlayControllerRef.current?.run("setJackHover", { offset });
  }, []);
  const onSelectionChange = reactHostPort.useCallback((range: { start: number; end: number }) => {
    dagPlayControllerRef.current?.run("setJackSelect", range);
  }, []);
  return (
    <WriterCanvas
      document={document}
      className="h-full"
      onHoverChange={onHoverChange}
      onSelectionChange={onSelectionChange}
      externalHoverOccurrences={ctrl?.getJackHoverOccurrences()}
      externalHoverOccurrencesSignal={ctrl?.getHoverEpoch()}
      externalSelectionOccurrences={ctrl?.getJackSelectOccurrences()}
      externalSelectionOccurrencesSignal={ctrl?.getSelectEpoch()}
    />
  );
}

export function registerDagPlaySurfaceHosts(): void {
  if (dagPlayChromeRegistered) return;
  dagPlayChromeRegistered = true;
  registerUiDagSurfaceHost(DAG_PLAY_SURFACE_ID, DagPlayPaneSurfaceHost);
  registerUiWriterSurfaceHost(DAG_PLAY_SURFACE_ID_JACK, DagPlayJackSurfaceHost);
  registerDagPlayDeclarativeBodies();
}

class DagPlayHierarchyPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: DAG_PLAY_HIERARCHY_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = dagPlayControllerRef.current;
        const bus = new CommandBus();
        const treeNode = buildDagPlayHierarchyTree(ctrl?.getFixtureJson() ?? DAG_PLAY_DEFAULT_FIXTURE_JSON, ctrl?.getSelectedNodeIds() ?? []);
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

class DagPlayCataloguePanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: DAG_PLAY_CATALOGUE_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
      order: 1,
      tree: new CallbackTreePanelDefinition(() => {
        const bus = new CommandBus();
        return uiTreeNodeToTreePanelConfig(buildDagPlayCatalogueTree(), bus);
      }),
    };
  }
}

class DagPlayInspectionPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: DAG_PLAY_INSPECTION_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, "details"),
      name: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = dagPlayControllerRef.current;
        const bus = new CommandBus();
        const treeNode = buildDagPlayInspectorTree(ctrl?.getFixtureJson() ?? DAG_PLAY_DEFAULT_FIXTURE_JSON, ctrl?.getSelectedNodeIds() ?? []);
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

function DagPlayInner({ runtime }: { readonly runtime: Platform }): ReactElement {
  useDagPlayController(runtime);
  const interactionRevision = useDagPlayInteractionRevision(runtime);
  const dagPlayHierarchyPanel = reactHostPort.useMemo(() => new DagPlayHierarchyPanelDefinition(), []);
  const dagPlayCataloguePanel = reactHostPort.useMemo(() => new DagPlayCataloguePanelDefinition(), []);
  const dagPlayInspectionPanel = reactHostPort.useMemo(() => new DagPlayInspectionPanelDefinition(), []);
  const augmentPanelTabs = reactHostPort.useMemo(
    () => ({
      workbench: [dagPlayHierarchyPanel, dagPlayCataloguePanel],
      details: [dagPlayInspectionPanel],
    }),
    [interactionRevision, dagPlayCataloguePanel, dagPlayHierarchyPanel, dagPlayInspectionPanel],
  );
  return <PlaygroundView runtime={runtime} defaultAppId={DAG_PLAY_APP_ID} augmentPanelTabs={augmentPanelTabs} />;
}

function DagPlayChrome({ runtime }: { readonly runtime: Platform }): ReactElement {
  return <DagPlayInner runtime={runtime} />;
}

export function mountDagPlayChrome(playground: Playground, rootId = "root"): void {
  mountPlaygroundApp(<DagPlayChrome runtime={playground.runtime} />, rootId);
}

const dagPlayChromeBoot: PlaygroundChromeBoot = {
  registerHosts: registerDagPlaySurfaceHosts,
  mount: mountDagPlayChrome,
};

export function bootDagPlay(playground: Playground, rootId = "root"): void {
  bootPlayground(playground, dagPlayChromeBoot, rootId);
}
//#endregion 🔖DagPlayHost

//#region 🔖ImperativePlayHost
import {
  IMPERATIVE_PLAY_APP_ID,
  IMPERATIVE_PLAY_DEFAULT_DOCUMENT_JSON,
  IMPERATIVE_PLAY_SURFACE_ID,
  ImperativePlayController,
  registerImperativePlayDeclarativeBodies,
} from "@semio-tech/imperative-core";
import { ImperativeEditor } from "@semio-tech/imperative-react";
import type { UiImperativeHostSurfaceNode } from "@semio-tech/framework-platform-core";

let imperativePlayChromeRegistered = false;
const imperativePlayControllerRef: { current: ImperativePlayController | null } = { current: null };

function useImperativePlayController(runtimeOverride?: Platform): ImperativePlayController | undefined {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const runtime = runtimeOverride ?? appCtx?.runtime;
  reactHostPort.useSyncExternalStore(
    (listener) => (runtime ? runtime.subscribeChrome(listener) : () => {}),
    () => runtime?.chromeGeneration ?? 0,
    () => 0,
  );
  const ctrl = runtime?.getActiveApp()?.controller as ImperativePlayController | undefined;
  imperativePlayControllerRef.current = ctrl ?? null;
  return ctrl;
}

function ImperativePlayPaneSurfaceHost(_props: { readonly node: UiImperativeHostSurfaceNode }): ReactElement {
  const ctrl = useImperativePlayController();
  const onDocumentChange = reactHostPort.useCallback(
    (json: string) => {
      ctrl?.run("setDocumentJson", { json });
    },
    [ctrl],
  );
  return (
    <ImperativeEditor
      className="h-full min-h-0"
      documentJson={ctrl?.getDocumentJson() ?? IMPERATIVE_PLAY_DEFAULT_DOCUMENT_JSON}
      onDocumentChange={onDocumentChange}
    />
  );
}

export function registerImperativePlaySurfaceHosts(): void {
  if (imperativePlayChromeRegistered) return;
  imperativePlayChromeRegistered = true;
  registerUiImperativeSurfaceHost(IMPERATIVE_PLAY_SURFACE_ID, ImperativePlayPaneSurfaceHost);
  registerImperativePlayDeclarativeBodies();
}

function ImperativePlayInner({ runtime }: { readonly runtime: Platform }): ReactElement {
  useImperativePlayController(runtime);
  return <PlaygroundView runtime={runtime} defaultAppId={IMPERATIVE_PLAY_APP_ID} />;
}

function ImperativePlayChrome({ runtime }: { readonly runtime: Platform }): ReactElement {
  return <ImperativePlayInner runtime={runtime} />;
}

export function mountImperativePlayChrome(playground: Playground, rootId = "root"): void {
  mountPlaygroundApp(<ImperativePlayChrome runtime={playground.runtime} />, rootId);
}

const imperativePlayChromeBoot: PlaygroundChromeBoot = {
  registerHosts: registerImperativePlaySurfaceHosts,
  mount: mountImperativePlayChrome,
};

export function bootImperativePlay(playground: Playground, rootId = "root"): void {
  bootPlayground(playground, imperativePlayChromeBoot, rootId);
}
//#endregion 🔖ImperativePlayHost

//#region 🔖SequencePlayHost
import {
  SEQUENCE_PLAY_APP_ID,
  SEQUENCE_PLAY_CATALOGUE_TAB_ID,
  SEQUENCE_PLAY_DEFAULT_FIXTURE_JSON,
  SEQUENCE_PLAY_HIERARCHY_TAB_ID,
  SEQUENCE_PLAY_INSPECTION_TAB_ID,
  SEQUENCE_PLAY_SCRIPT_SURFACE_ID,
  SEQUENCE_PLAY_SCRIPT_WINDOW_KIND_ID,
  SEQUENCE_PLAY_SURFACE_ID,
  SEQUENCE_PLAY_SURFACE_ID_JACK,
  SEQUENCE_PLAY_SURFACE_ID_COMPILED_DAG,
  SEQUENCE_PLAY_WINDOW_KIND_ID,
  SequencePlayController,
  buildSequencePlayCatalogueTree,
  buildSequencePlayHierarchyTree,
  buildSequencePlayInspectorTree,
  registerSequencePlayDeclarativeBodies,
} from "@semio-tech/sequence-core";
import {
  DAG_LOD_MODE_AUTOMATIC as SEQUENCE_HOST_LOD_AUTOMATIC,
  dagLodCanvasProps as sequenceLodCanvasProps,
  SequenceCanvas,
  sequenceStepPaletteTreeDragController,
} from "@semio-tech/sequence-react";

let sequencePlayChromeRegistered = false;
const sequencePlayControllerRef: { current: SequencePlayController | null } = { current: null };

function useSequencePlayController(runtimeOverride?: Platform): SequencePlayController | undefined {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const runtime = runtimeOverride ?? appCtx?.runtime;
  reactHostPort.useSyncExternalStore(
    (listener) => (runtime ? runtime.subscribeChrome(listener) : () => {}),
    () => runtime?.chromeGeneration ?? 0,
    () => 0,
  );
  const ctrl = runtime?.getActiveApp()?.controller as SequencePlayController | undefined;
  sequencePlayControllerRef.current = ctrl ?? null;
  return ctrl;
}

function useSequencePlayInteractionRevision(runtime: Platform): number {
  return reactHostPort.useSyncExternalStore(
    (listener) => {
      const ctrl = runtime.getActiveApp()?.controller as SequencePlayController | undefined;
      sequencePlayControllerRef.current = ctrl ?? null;
      const unsubscribeRuntime = runtime.subscribe(listener);
      const unsubscribeSnapshot = ctrl?.subscribeSnapshot(listener);
      return () => {
        unsubscribeRuntime();
        unsubscribeSnapshot?.();
      };
    },
    () => (runtime.getActiveApp()?.controller as SequencePlayController | undefined)?.getInteractionRevision() ?? 0,
    () => 0,
  );
}

function SequencePlayPaneSurfaceHost({ node }: { readonly node: import("@semio-tech/framework-platform-core").UiSequenceHostSurfaceNode }): ReactElement {
  const ctrl = useSequencePlayController();
  const scopeId = node.paneId ?? SEQUENCE_PLAY_WINDOW_KIND_ID;
  const lodProps = sequenceLodCanvasProps(ctrl?.lodModeForScope(scopeId) ?? SEQUENCE_HOST_LOD_AUTOMATIC);
  const onLodChange = reactHostPort.useCallback(
    (lod: import("@semio-tech/dag-react").DagDrawLodKind) => {
      ctrl?.run("setEffectiveLod", { lod, instanceId: scopeId });
    },
    [ctrl, scopeId],
  );
  const onFixtureChange = reactHostPort.useCallback(
    (json: string) => {
      ctrl?.run("setFixtureJson", { json });
    },
    [ctrl],
  );
  const onSelectionChange = reactHostPort.useCallback(
    (ids: readonly string[]) => {
      ctrl?.run("setSelection", { ids: [...ids] });
    },
    [ctrl],
  );
  const onCompiledTextChange = reactHostPort.useCallback(
    (text: string) => {
      ctrl?.run("setCompiledText", { text });
    },
    [ctrl],
  );
  const onCompiledWireLiteralChange = reactHostPort.useCallback(
    (text: string) => {
      ctrl?.run("setCompiledWireLiteral", { text });
    },
    [ctrl],
  );
  const onRunResult = reactHostPort.useCallback(
    (result: import("@semio-tech/imperative-core").RunResult) => {
      ctrl?.run("setRunResult", { result });
    },
    [ctrl],
  );
  return (
    <SequenceCanvas
      fixtureJson={ctrl?.getFixtureJson() ?? SEQUENCE_PLAY_DEFAULT_FIXTURE_JSON}
      reorganize={ctrl?.getReorganize()}
      runRequest={ctrl?.getRunRequest()}
      runStopRequest={ctrl?.getRunStopRequest()}
      selectedStepIds={ctrl?.getSelectedStepIds() ?? []}
      fixtureDragDrop
      onFixtureChange={onFixtureChange}
      onSelectionChange={onSelectionChange}
      onCompiledTextChange={onCompiledTextChange}
      onCompiledWireLiteralChange={onCompiledWireLiteralChange}
      onRunResult={onRunResult}
      {...lodProps}
      onLodChange={onLodChange}
    />
  );
}

function SequencePlayScriptSurfaceHost({ node: _node }: { readonly node: import("@semio-tech/framework-platform-core").UiWriterHostSurfaceNode }): ReactElement {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const ctrl = useSequencePlayController();
  const interactionRevision = useSequencePlayInteractionRevision(appCtx?.runtime as Platform);
  const document = reactHostPort.useMemo(
    () =>
      createWriterDocument({
        id: "sequence-compiled-script",
        languageId: "plaintext",
        text: ctrl?.getCompiledText() ?? "",
      }),
    [ctrl?.getCompiledText(), interactionRevision],
  );
  return <WriterCanvas document={document} className="h-full min-h-0" />;
}

function SequencePlayJackSurfaceHost({ node: _node }: { readonly node: import("@semio-tech/framework-platform-core").UiWriterHostSurfaceNode }): ReactElement {
  const ctrl = useSequencePlayController();
  void ctrl?.getHoverEpoch();
  void ctrl?.getSelectEpoch();
  const document = ctrl?.getWriterDocumentJack() ?? createWriterDocument({ id: "sequence-jack", languageId: "jack", text: "" });
  const onHoverChange = reactHostPort.useCallback((offset: number | null) => {
    sequencePlayControllerRef.current?.run("setJackHover", { offset });
  }, []);
  const onSelectionChange = reactHostPort.useCallback((range: { start: number; end: number }) => {
    sequencePlayControllerRef.current?.run("setJackSelect", range);
  }, []);
  return (
    <WriterCanvas
      document={document}
      className="h-full"
      onHoverChange={onHoverChange}
      onSelectionChange={onSelectionChange}
      externalHoverOccurrences={ctrl?.getJackHoverOccurrences()}
      externalHoverOccurrencesSignal={ctrl?.getHoverEpoch()}
      externalSelectionOccurrences={ctrl?.getJackSelectOccurrences()}
      externalSelectionOccurrencesSignal={ctrl?.getSelectEpoch()}
    />
  );
}

function SequencePlayCompiledDagSurfaceHost({ node: _node }: { readonly node: import("@semio-tech/framework-platform-core").UiWriterHostSurfaceNode }): ReactElement {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const ctrl = useSequencePlayController();
  const interactionRevision = useSequencePlayInteractionRevision(appCtx?.runtime as Platform);
  const document = reactHostPort.useMemo(
    () => ctrl?.getWriterDocumentCompiledDag() ?? createWriterDocument({ id: "sequence-compiled-dag", languageId: "wire", text: "" }),
    [ctrl, interactionRevision],
  );
  return <WriterCanvas document={document} className="h-full min-h-0" />;
}

export function registerSequencePlaySurfaceHosts(): void {
  if (sequencePlayChromeRegistered) return;
  sequencePlayChromeRegistered = true;
  registerUiSequenceSurfaceHost(SEQUENCE_PLAY_SURFACE_ID, SequencePlayPaneSurfaceHost);
  registerUiWriterSurfaceHost(SEQUENCE_PLAY_SCRIPT_SURFACE_ID, SequencePlayScriptSurfaceHost);
  registerUiWriterSurfaceHost(SEQUENCE_PLAY_SURFACE_ID_JACK, SequencePlayJackSurfaceHost);
  registerUiWriterSurfaceHost(SEQUENCE_PLAY_SURFACE_ID_COMPILED_DAG, SequencePlayCompiledDagSurfaceHost);
  registerSequencePlayDeclarativeBodies();
}

class SequencePlayHierarchyPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: SEQUENCE_PLAY_HIERARCHY_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = sequencePlayControllerRef.current;
        const bus = new CommandBus();
        const treeNode = buildSequencePlayHierarchyTree(ctrl?.getFixtureJson() ?? SEQUENCE_PLAY_DEFAULT_FIXTURE_JSON, ctrl?.getSelectedStepIds() ?? []);
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

class SequencePlayCataloguePanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: SEQUENCE_PLAY_CATALOGUE_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
      order: 1,
      tree: new CallbackTreePanelDefinition(() => {
        const bus = new CommandBus();
        const treeNode = buildSequencePlayCatalogueTree();
        const config = uiTreeNodeToTreePanelConfig(treeNode, bus);
        return {
          ...config,
          dragAndDropController: sequenceStepPaletteTreeDragController(collectUiTreeItemDragData(treeNode.sections)),
        };
      }),
    };
  }
}

class SequencePlayInspectionPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: SEQUENCE_PLAY_INSPECTION_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, "details"),
      name: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = sequencePlayControllerRef.current;
        const bus = new CommandBus();
        const treeNode = buildSequencePlayInspectorTree(
          ctrl?.getFixtureJson() ?? SEQUENCE_PLAY_DEFAULT_FIXTURE_JSON,
          ctrl?.getSelectedStepIds() ?? [],
          ctrl?.getEffectLog() ?? [],
        );
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

function SequencePlayInner({ runtime }: { readonly runtime: Platform }): ReactElement {
  useSequencePlayController(runtime);
  const interactionRevision = useSequencePlayInteractionRevision(runtime);
  const sequencePlayHierarchyPanel = reactHostPort.useMemo(() => new SequencePlayHierarchyPanelDefinition(), []);
  const sequencePlayCataloguePanel = reactHostPort.useMemo(() => new SequencePlayCataloguePanelDefinition(), []);
  const sequencePlayInspectionPanel = reactHostPort.useMemo(() => new SequencePlayInspectionPanelDefinition(), []);
  const augmentPanelTabs = reactHostPort.useMemo(
    () => ({
      workbench: [sequencePlayHierarchyPanel, sequencePlayCataloguePanel],
      details: [sequencePlayInspectionPanel],
    }),
    [interactionRevision, sequencePlayCataloguePanel, sequencePlayHierarchyPanel, sequencePlayInspectionPanel],
  );
  return <PlaygroundView runtime={runtime} defaultAppId={SEQUENCE_PLAY_APP_ID} augmentPanelTabs={augmentPanelTabs} />;
}

function SequencePlayChrome({ runtime }: { readonly runtime: Platform }): ReactElement {
  return <SequencePlayInner runtime={runtime} />;
}

export function mountSequencePlayChrome(playground: Playground, rootId = "root"): void {
  mountPlaygroundApp(<SequencePlayChrome runtime={playground.runtime} />, rootId);
}

const sequencePlayChromeBoot: PlaygroundChromeBoot = {
  registerHosts: registerSequencePlaySurfaceHosts,
  mount: mountSequencePlayChrome,
};

export function bootSequencePlay(playground: Playground, rootId = "root"): void {
  bootPlayground(playground, sequencePlayChromeBoot, rootId);
}
//#endregion 🔖SequencePlayHost

//#region 🔖LayoutPlayHost
import {
  LAYOUT_PLAY_APP_ID,
  LAYOUT_PLAY_CONTROLLER_ID,
  LAYOUT_PLAY_HIERARCHY_TAB_ID,
  LAYOUT_PLAY_INSPECTION_TAB_ID,
  LAYOUT_PLAY_PREFLIGHT_TAB_ID,
  LAYOUT_PLAY_SURFACE_BLUEPRINT,
  LAYOUT_PLAY_SURFACE_PREVIEW,
  LAYOUT_PLAY_WINDOW_BLUEPRINT,
  LAYOUT_PLAY_WINDOW_PREVIEW,
  LayoutPlayController,
  buildLayoutPlayHierarchyTree,
  buildLayoutPlayInspectorTree,
  buildLayoutPlayPreflightTree,
  registerLayoutPlayDeclarativeBodies,
} from "@semio-tech/layout-core";
import { LayoutCanvas } from "@semio-tech/layout-react";
import { DEFAULT_LAYOUT_DOCUMENT_JSON } from "@semio-tech/layout-core";

let layoutPlayChromeRegistered = false;
const layoutPlayControllerRef: { current: LayoutPlayController | null } = { current: null };

function useLayoutPlayController(runtimeOverride?: Platform): LayoutPlayController | undefined {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const runtime = runtimeOverride ?? appCtx?.runtime;
  reactHostPort.useSyncExternalStore(
    (listener) => (runtime ? runtime.subscribeChrome(listener) : () => {}),
    () => runtime?.chromeGeneration ?? 0,
    () => 0,
  );
  const ctrl = runtime?.getActiveApp()?.controller as LayoutPlayController | undefined;
  layoutPlayControllerRef.current = ctrl ?? null;
  return ctrl;
}

function useLayoutPlayInteractionRevision(runtime: Platform): number {
  return reactHostPort.useSyncExternalStore(
    (listener) => {
      const ctrl = runtime.getActiveApp()?.controller as LayoutPlayController | undefined;
      layoutPlayControllerRef.current = ctrl ?? null;
      const unsubscribeRuntime = runtime.subscribe(listener);
      const unsubscribeSnapshot = ctrl?.subscribeSnapshot(listener);
      return () => {
        unsubscribeRuntime();
        unsubscribeSnapshot?.();
      };
    },
    () => (runtime.getActiveApp()?.controller as LayoutPlayController | undefined)?.getInteractionRevision() ?? 0,
    () => 0,
  );
}

function LayoutPlayPaneSurfaceHost({ node }: { readonly node: import("@semio-tech/framework-platform-core").UiLayoutHostSurfaceNode }): ReactElement {
  const ctrl = useLayoutPlayController();
  const chromeMode = node.chromeMode ?? (node.paneId === LAYOUT_PLAY_WINDOW_PREVIEW ? "preview" : "blueprint");
  const onSelectionChange = reactHostPort.useCallback(
    (objectId: string | null) => {
      if (objectId) ctrl?.run("setSelection", { ids: [objectId] });
    },
    [ctrl],
  );
  return (
    <LayoutCanvas
      chromeMode={chromeMode}
      documentJson={ctrl?.getDocumentJson() ?? DEFAULT_LAYOUT_DOCUMENT_JSON}
      pageId={ctrl?.getActivePageId() ?? "page-1"}
      selectedIds={ctrl?.getSelectedIds() ?? []}
      onHit={chromeMode === "blueprint" ? onSelectionChange : undefined}
      className="h-full min-h-0"
    />
  );
}

export function registerLayoutPlaySurfaceHosts(): void {
  if (layoutPlayChromeRegistered) return;
  layoutPlayChromeRegistered = true;
  registerUiLayoutSurfaceHost(LAYOUT_PLAY_SURFACE_BLUEPRINT, LayoutPlayPaneSurfaceHost);
  registerUiLayoutSurfaceHost(LAYOUT_PLAY_SURFACE_PREVIEW, LayoutPlayPaneSurfaceHost);
  registerLayoutPlayDeclarativeBodies();
}

class LayoutPlayHierarchyPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: LAYOUT_PLAY_HIERARCHY_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = layoutPlayControllerRef.current;
        const bus = new CommandBus();
        const treeNode = buildLayoutPlayHierarchyTree(ctrl?.getDocumentJson() ?? DEFAULT_LAYOUT_DOCUMENT_JSON, ctrl?.getSelectedIds() ?? []);
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

class LayoutPlayPreflightPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: LAYOUT_PLAY_PREFLIGHT_TAB_ID,
      icon: shellTabIconComponent("alert-triangle", "workbench"),
      name: "Preflight",
      order: 1,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = layoutPlayControllerRef.current;
        const bus = new CommandBus();
        const treeNode = buildLayoutPlayPreflightTree(ctrl?.getDocumentJson() ?? DEFAULT_LAYOUT_DOCUMENT_JSON);
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

class LayoutPlayInspectionPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: LAYOUT_PLAY_INSPECTION_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, "details"),
      name: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = layoutPlayControllerRef.current;
        const bus = new CommandBus();
        const treeNode = buildLayoutPlayInspectorTree(ctrl?.getDocumentJson() ?? DEFAULT_LAYOUT_DOCUMENT_JSON, ctrl?.getSelectedIds() ?? []);
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

const layoutPlayHierarchyPanel = new LayoutPlayHierarchyPanelDefinition();
const layoutPlayPreflightPanel = new LayoutPlayPreflightPanelDefinition();
const layoutPlayInspectionPanel = new LayoutPlayInspectionPanelDefinition();

function LayoutPlayInner({ runtime }: { readonly runtime: Platform }): ReactElement {
  const interactionRevision = useLayoutPlayInteractionRevision(runtime);
  const augmentPanelTabs = reactHostPort.useMemo(
    () => ({
      workbench: [layoutPlayHierarchyPanel, layoutPlayPreflightPanel],
      details: [layoutPlayInspectionPanel],
    }),
    [interactionRevision],
  );
  return <PlaygroundView runtime={runtime} defaultAppId={LAYOUT_PLAY_APP_ID} augmentPanelTabs={augmentPanelTabs} />;
}

function LayoutPlayChrome({ runtime }: { readonly runtime: Platform }): ReactElement {
  return <LayoutPlayInner runtime={runtime} />;
}

export function mountLayoutPlayChrome(playground: Playground, rootId = "root"): void {
  mountPlaygroundApp(<LayoutPlayChrome runtime={playground.runtime} />, rootId);
}

const layoutPlayChromeBoot: PlaygroundChromeBoot = {
  registerHosts: registerLayoutPlaySurfaceHosts,
  mount: mountLayoutPlayChrome,
};

export function bootLayoutPlay(playground: Playground, rootId = "root"): void {
  bootPlayground(playground, layoutPlayChromeBoot, rootId);
}
//#endregion 🔖LayoutPlayHost

//#region 🔖LowpolyPlayHost
import {
  LOWPOLY_PLAY_APP_ID,
  LOWPOLY_PLAY_CATALOGUE_TAB_ID,
  LOWPOLY_PLAY_DEFAULT_FIXTURE_JSON,
  LOWPOLY_PLAY_HIERARCHY_TAB_ID,
  LOWPOLY_PLAY_INSPECTION_TAB_ID,
  LOWPOLY_PLAY_LAYERS_TAB_ID,
  LOWPOLY_PLAY_SURFACE_ID,
  LOWPOLY_PLAY_UV_SURFACE_ID,
  LOWPOLY_PLAY_WINDOW_KIND_ID,
  LowpolyPlayController,
  buildLowpolyPlayCatalogueTree,
  buildLowpolyPlayHierarchyTree,
  buildLowpolyPlayInspectorTree,
  buildLowpolyPlayLayersTree,
  registerLowpolyPlayDeclarativeBodies,
} from "@semio-tech/lowpoly-core";
import {
  LowpolyCanvas,
  LowpolyUvCanvas,
  createLowpolySession,
  loadDefaultLowpolyFixtureJson,
  safeLoadLowpolyFixture,
  tessellateAllLowpolySession,
  type LowpolySessionWasm,
} from "@semio-tech/lowpoly-react";
import { isLowpolyFixtureReady, parseLowpolyFixtureJson, type LowpolySceneObject } from "@semio-tech/lowpoly-core";

let lowpolyPlayChromeRegistered = false;
const lowpolyPlayControllerRef: { current: LowpolyPlayController | null } = { current: null };

type LowpolySharedPlaySnapshot = {
	readonly session: LowpolySessionWasm | null;
	readonly sceneObjects: readonly LowpolySceneObject[];
	readonly paintTextureRevision: number;
	readonly generation: number;
};

const lowpolySharedPlaySnapshot: LowpolySharedPlaySnapshot = {
	session: null,
	sceneObjects: [],
	paintTextureRevision: 0,
	generation: 0,
};
const lowpolySharedPlayListeners = new Set<() => void>();
const lowpolyPaintStrokeHandlersRef: { current: { onBegin?: () => void; onEnd?: () => void } } = { current: {} };

function notifyLowpolySharedPlay(next?: Partial<Pick<LowpolySharedPlaySnapshot, "session" | "sceneObjects" | "paintTextureRevision">>): void {
	if (next?.session !== undefined) (lowpolySharedPlaySnapshot as { session: LowpolySessionWasm | null }).session = next.session;
	if (next?.sceneObjects !== undefined) (lowpolySharedPlaySnapshot as { sceneObjects: readonly LowpolySceneObject[] }).sceneObjects = next.sceneObjects;
	if (next?.paintTextureRevision !== undefined) (lowpolySharedPlaySnapshot as { paintTextureRevision: number }).paintTextureRevision = next.paintTextureRevision;
	(lowpolySharedPlaySnapshot as { generation: number }).generation += 1;
	for (const listener of lowpolySharedPlayListeners) listener();
}

function bumpLowpolyPaintTextureRevision(): void {
	notifyLowpolySharedPlay({ paintTextureRevision: lowpolySharedPlaySnapshot.paintTextureRevision + 1 });
}

function useLowpolySharedPlaySnapshot(): LowpolySharedPlaySnapshot {
	return reactHostPort.useSyncExternalStore(
		(listener) => {
			lowpolySharedPlayListeners.add(listener);
			return () => lowpolySharedPlayListeners.delete(listener);
		},
		() => lowpolySharedPlaySnapshot,
		() => lowpolySharedPlaySnapshot,
	);
}

function useLowpolyPlayController(runtimeOverride?: Platform): LowpolyPlayController | undefined {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const runtime = runtimeOverride ?? appCtx?.runtime;
  reactHostPort.useSyncExternalStore(
    (listener) => (runtime ? runtime.subscribeChrome(listener) : () => {}),
    () => runtime?.chromeGeneration ?? 0,
    () => 0,
  );
  const ctrl = runtime?.getActiveApp()?.controller as LowpolyPlayController | undefined;
  lowpolyPlayControllerRef.current = ctrl ?? null;
  return ctrl;
}

function useLowpolyPlayInteractionRevision(runtime: Platform): number {
  return reactHostPort.useSyncExternalStore(
    (listener) => {
      const ctrl = runtime.getActiveApp()?.controller as LowpolyPlayController | undefined;
      lowpolyPlayControllerRef.current = ctrl ?? null;
      const unsubscribeRuntime = runtime.subscribe(listener);
      const unsubscribeSnapshot = ctrl?.subscribeSnapshot(listener);
      return () => {
        unsubscribeRuntime();
        unsubscribeSnapshot?.();
      };
    },
    () => (runtime.getActiveApp()?.controller as LowpolyPlayController | undefined)?.getInteractionRevision() ?? 0,
    () => 0,
  );
}

function useLowpolyPlayHoverTarget(runtime: Platform): import("@semio-tech/lowpoly-core").LowpolyTarget | null {
	return reactHostPort.useSyncExternalStore(
		(listener) => {
			const ctrl = runtime.getActiveApp()?.controller as LowpolyPlayController | undefined;
			lowpolyPlayControllerRef.current = ctrl ?? null;
			return ctrl?.subscribeHover(listener) ?? (() => {});
		},
		() => (runtime.getActiveApp()?.controller as LowpolyPlayController | undefined)?.getHoveredTargetSnapshot() ?? null,
		() => null,
	);
}

function syncLowpolyControllerFromSession(ctrl: LowpolyPlayController, session: LowpolySessionWasm): void {
  const json = session.fixtureJson();
  ctrl.run("setFixtureJson", { json });
  const fixture = parseLowpolyFixtureJson(json);
  if (fixture) {
    ctrl.run("setSelection", { mode: fixture.selection.mode, ids: [...fixture.selection.ids] });
  }
}

function lowpolyMirrorAxis(toolParams: Record<string, number>): string {
  const axisIndex = toolParams.mirrorAxis ?? 0;
  return axisIndex === 1 ? "y" : axisIndex === 2 ? "z" : "x";
}

function LowpolyPlaySessionBridge(): null {
  const ctrl = useLowpolyPlayController();
  const meshEpoch = ctrl?.getMeshCommandEpoch() ?? 0;
  const toolParams = ctrl?.getToolParams() ?? {};
  const paintStrokeBeforeRef = reactHostPort.useRef<Uint8Array | null>(null);
  const activeObjectId = lowpolySharedPlaySnapshot.sceneObjects.find((object) => object.active)?.id;

  reactHostPort.useEffect(() => {
    let cancelled = false;
    void (async () => {
      if (lowpolySharedPlaySnapshot.session) {
        notifyLowpolySharedPlay();
        return;
      }
      const session = await createLowpolySession();
      if (cancelled) return;
      const json = ctrl?.getFixtureJson() ?? LOWPOLY_PLAY_DEFAULT_FIXTURE_JSON;
      if (isLowpolyFixtureReady(json)) {
        safeLoadLowpolyFixture(session, json);
      } else {
        const defaultJson = await loadDefaultLowpolyFixtureJson();
        safeLoadLowpolyFixture(session, defaultJson);
        if (ctrl) syncLowpolyControllerFromSession(ctrl, session);
      }
      notifyLowpolySharedPlay({
        session,
        sceneObjects: tessellateAllLowpolySession(session),
      });
    })();
    return () => {
      cancelled = true;
    };
  }, [ctrl]);

  reactHostPort.useEffect(() => {
    const session = lowpolySharedPlaySnapshot.session;
    if (!session || !ctrl) return;
    const json = ctrl.getFixtureJson();
    if (!isLowpolyFixtureReady(json)) return;
    safeLoadLowpolyFixture(session, json);
    session.setSelection(ctrl.getSelectionMode(), [...ctrl.getSelectedIds()]);
    notifyLowpolySharedPlay({ sceneObjects: tessellateAllLowpolySession(session) });
  }, [ctrl, ctrl?.getFixtureJson(), ctrl?.getSelectionMode(), ctrl?.getSelectedIds(), ctrl?.getInteractionRevision()]);

  reactHostPort.useEffect(() => {
    const session = lowpolySharedPlaySnapshot.session;
    if (!session || !ctrl || meshEpoch === 0) return;
    const pending = ctrl.getPendingMeshCommand();
    const paintPending = ctrl.getPendingPaintCommand();
    if (!pending && !paintPending) return;
    try {
      if (pending?.startsWith("addPrimitive:")) {
        const kind = pending.slice("addPrimitive:".length);
        session.addPrimitive(kind);
      } else if (pending?.startsWith("flipFace:")) {
        const [, objectId, faceId] = pending.split(":");
        if (objectId && faceId != null) {
          session.setActiveObject(objectId);
          session.flipFaces([Number(faceId)]);
        }
      } else if (pending === "extrude") session.extrudeFaces(toolParams.extrudeDistance ?? 0.25);
      else if (pending === "inset") session.insetFaces(toolParams.insetAmount ?? 0.1);
      else if (pending === "flipFaces") session.flipFaces([...ctrl.getSelectedIds()]);
      else if (pending === "bevel") session.bevelEdges(toolParams.bevelAmount ?? 0.05, toolParams.bevelSegments ?? 1);
      else if (pending === "loopCut") session.loopCut(toolParams.loopCuts ?? 1);
      else if (pending === "merge") session.mergeVertices();
      else if (pending === "dissolve") session.dissolveEdges();
      else if (pending === "subdivide") session.subdivideFaces();
      else if (pending === "triangulate") session.triangulate();
      else if (pending === "mirror") session.mirror(lowpolyMirrorAxis(toolParams), 0.001);
      else if (pending === "decimate") session.decimate(toolParams.decimateRatio ?? 0.5);
      else if (pending === "snap") session.snapToGrid(toolParams.snapGrid ?? 0.25);
      else if (pending === "toggleSmooth") session.setSmoothShading(!ctrl.getSmoothShading());
      else if (paintPending?.command === "unwrapActive") session.unwrapActive();
      else if (paintPending?.command === "markUvSeam") {
        session.markUvSeam(Boolean(paintPending.args?.seam), [...ctrl.getSelectedIds()]);
      }
      syncLowpolyControllerFromSession(ctrl, session);
      notifyLowpolySharedPlay({ sceneObjects: tessellateAllLowpolySession(session) });
    } catch {
      /* mesh command may fail on empty selection */
    } finally {
      ctrl.clearPendingMeshCommand();
      ctrl.clearPendingPaintCommand();
    }
  }, [meshEpoch, ctrl, toolParams]);

  const paintVcsGeneration = reactHostPort.useSyncExternalStore(
    (listener) => ctrl?.subscribePaintVcs(listener) ?? (() => {}),
    () => ctrl?.getPaintVcsGeneration() ?? 0,
    () => 0,
  );

  reactHostPort.useEffect(() => {
    const session = lowpolySharedPlaySnapshot.session;
    if (!session || !ctrl) return;
    const projection = ctrl.getPaintProjection();
    const expected = 1024 * 1024 * 4;
    if (projection.pixels.length !== expected) return;
    session.setPaintLayerPixels(projection.objectId, projection.layerIndex, new Uint8Array(projection.pixels));
    bumpLowpolyPaintTextureRevision();
  }, [paintVcsGeneration, ctrl]);

  reactHostPort.useEffect(() => {
    lowpolyPaintStrokeHandlersRef.current.onBegin = () => {
      const session = lowpolySharedPlaySnapshot.session;
      if (!session || !activeObjectId) return;
      const layerIndex = ctrl?.getActivePaintLayerIndex() ?? 0;
      paintStrokeBeforeRef.current = new Uint8Array(session.paintLayerPixels(activeObjectId, layerIndex));
    };
    lowpolyPaintStrokeHandlersRef.current.onEnd = () => {
      const session = lowpolySharedPlaySnapshot.session;
      if (!session || !ctrl || !activeObjectId) return;
      const layerIndex = ctrl.getActivePaintLayerIndex();
      const before = paintStrokeBeforeRef.current;
      const after = session.paintLayerPixels(activeObjectId, layerIndex);
      if (before) {
        ctrl.dispatchPaintVcs({
          kind: "apply",
          operations: [
            {
              kind: "layerPixels",
              objectId: activeObjectId,
              layerIndex,
              before: [...before],
              after: [...after],
            },
          ],
        });
      }
      paintStrokeBeforeRef.current = null;
      bumpLowpolyPaintTextureRevision();
    };
    return () => {
      lowpolyPaintStrokeHandlersRef.current = {};
    };
  }, [activeObjectId, ctrl]);

  return null;
}

function LowpolyPlaySurfaceHost({ node: _node }: { readonly node: UiPuzzle3dHostSurfaceNode }): ReactElement {
  const ctrl = useLowpolyPlayController();
  const { activeModeId, runtime } = useApp();
  const hoveredTarget = useLowpolyPlayHoverTarget(runtime);
  const shared = useLowpolySharedPlaySnapshot();
  const session = shared.session;
  const sceneObjects = shared.sceneObjects;
  const toolParams = ctrl?.getToolParams() ?? {};
  const controllerFixtureJson = ctrl?.getFixtureJson() ?? LOWPOLY_PLAY_DEFAULT_FIXTURE_JSON;
  const fixtureJson =
    session && isLowpolyFixtureReady(controllerFixtureJson)
      ? controllerFixtureJson
      : session?.fixtureJson() ?? controllerFixtureJson;
  const interactionMode = activeModeId === "paint" ? "paint" : "model";

  const onFixtureChange = reactHostPort.useCallback(
    (json: string) => {
      ctrl?.run("setFixtureJson", { json });
    },
    [ctrl],
  );
  const onSelectionChange = reactHostPort.useCallback(
    (mode: import("@semio-tech/lowpoly-core").LowpolySelectionMode, ids: readonly number[], activeObjectId?: string) => {
      if (activeObjectId && session) {
        session.setActiveObject(activeObjectId);
      }
      ctrl?.run("setSelection", { mode, ids: [...ids], activeObjectId });
    },
    [ctrl, session],
  );
  const onPaintStrokeBegin = reactHostPort.useCallback(() => {
    lowpolyPaintStrokeHandlersRef.current.onBegin?.();
  }, []);
  const onPaintStrokeEnd = reactHostPort.useCallback(() => {
    lowpolyPaintStrokeHandlersRef.current.onEnd?.();
  }, []);
  const onSceneChange = reactHostPort.useCallback((objects: readonly LowpolySceneObject[]) => {
    notifyLowpolySharedPlay({ sceneObjects: objects });
  }, []);

  return (
    <div className="absolute inset-0 min-h-0 min-w-0">
      <LowpolyCanvas
        fixtureJson={fixtureJson}
        sceneObjects={sceneObjects}
        selectionMode={ctrl?.getSelectionMode() ?? "object"}
        selectedIds={ctrl?.getSelectedIds() ?? []}
        hoveredTarget={hoveredTarget}
        transformTool={ctrl?.getTransformTool() ?? "move"}
        session={session}
        interactionMode={interactionMode}
        paintTool={ctrl?.getPaintTool() ?? "brush"}
        paintLayerIndex={ctrl?.getActivePaintLayerIndex() ?? 0}
        paintColor={ctrl?.getPaintColor() ?? [255, 64, 64, 255]}
        paintBrushSize={toolParams.brushSize ?? 16}
        paintBrushOpacity={toolParams.brushOpacity ?? 1}
        paintBrushHardness={toolParams.brushHardness ?? 0.5}
        paintTextureRevision={shared.paintTextureRevision}
        onFixtureChange={onFixtureChange}
        onSelectionChange={onSelectionChange}
        onHoverChange={(target) => ctrl?.run("setHover", { target })}
        onSceneChange={onSceneChange}
        onPaintStrokeBegin={onPaintStrokeBegin}
        onPaintStrokeEnd={onPaintStrokeEnd}
        onPaintTextureRefresh={bumpLowpolyPaintTextureRevision}
        className="h-full w-full"
      />
    </div>
  );
}

function LowpolyUvSurfaceHost({ node: _node }: { readonly node: UiPuzzle3dHostSurfaceNode }): ReactElement {
  const ctrl = useLowpolyPlayController();
  const shared = useLowpolySharedPlaySnapshot();
  const session = shared.session;
  const sceneObjects = shared.sceneObjects;
  const toolParams = ctrl?.getToolParams() ?? {};
  const fixtureJson = ctrl?.getFixtureJson() ?? LOWPOLY_PLAY_DEFAULT_FIXTURE_JSON;
  const activeObject = sceneObjects.find((object) => object.active) ?? sceneObjects[0] ?? null;
  const paintStrokeBeforeRef = reactHostPort.useRef<Uint8Array | null>(null);

  const onPaintStrokeBegin = reactHostPort.useCallback(() => {
    if (!session || !activeObject) return;
    const layerIndex = ctrl?.getActivePaintLayerIndex() ?? 0;
    paintStrokeBeforeRef.current = new Uint8Array(session.paintLayerPixels(activeObject.id, layerIndex));
  }, [activeObject, ctrl, session]);

  const onPaintStrokeEnd = reactHostPort.useCallback(() => {
    if (!session || !ctrl || !activeObject) return;
    const layerIndex = ctrl.getActivePaintLayerIndex();
    const before = paintStrokeBeforeRef.current;
    const after = session.paintLayerPixels(activeObject.id, layerIndex);
    if (before) {
      ctrl.dispatchPaintVcs({
        kind: "apply",
        operations: [
          {
            kind: "layerPixels",
            objectId: activeObject.id,
            layerIndex,
            before: [...before],
            after: [...after],
          },
        ],
      });
    }
    paintStrokeBeforeRef.current = null;
    bumpLowpolyPaintTextureRevision();
    ctrl.run("setFixtureJson", { json: session.fixtureJson() });
  }, [activeObject, ctrl, session]);

  return (
    <div className="absolute inset-0 min-h-0 min-w-0">
      <LowpolyUvCanvas
        sceneObject={activeObject}
        session={session}
        paintTool={ctrl?.getPaintTool() ?? "brush"}
        paintLayerIndex={ctrl?.getActivePaintLayerIndex() ?? 0}
        paintColor={ctrl?.getPaintColor() ?? [255, 64, 64, 255]}
        paintBrushSize={toolParams.brushSize ?? 16}
        paintBrushOpacity={toolParams.brushOpacity ?? 1}
        paintBrushHardness={toolParams.brushHardness ?? 0.5}
        paintTextureRevision={shared.paintTextureRevision}
        onFixtureChange={(json) => ctrl?.run("setFixtureJson", { json })}
        onPaintStrokeBegin={onPaintStrokeBegin}
        onPaintStrokeEnd={onPaintStrokeEnd}
        onPaintTextureRefresh={bumpLowpolyPaintTextureRevision}
        className="h-full w-full"
      />
    </div>
  );
}

export function registerLowpolyPlaySurfaceHosts(): void {
  if (lowpolyPlayChromeRegistered) return;
  lowpolyPlayChromeRegistered = true;
  registerUiPuzzle3dSurfaceHost(LOWPOLY_PLAY_SURFACE_ID, LowpolyPlaySurfaceHost);
  registerUiPuzzle3dSurfaceHost(LOWPOLY_PLAY_UV_SURFACE_ID, LowpolyUvSurfaceHost);
  registerLowpolyPlayDeclarativeBodies();
}

class LowpolyPlayHierarchyPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: LOWPOLY_PLAY_HIERARCHY_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = lowpolyPlayControllerRef.current;
        const bus = ctrl?.commandBus ?? new CommandBus();
        const fixture = ctrl?.getFixtureJson() ?? LOWPOLY_PLAY_DEFAULT_FIXTURE_JSON;
        const treeNode = buildLowpolyPlayHierarchyTree(
          fixture,
          ctrl?.getSelectionMode() ?? "object",
          ctrl?.getSelectedIds() ?? [],
          {
            hoveredTarget: ctrl?.getHoveredTarget() ?? null,
            onHover: (target) => ctrl?.run("setHover", { target }),
            onFlipFace: (objectId, faceId) => ctrl?.run("flipFace", { objectId, faceId }),
          },
        );
        return { ...uiTreeNodeToTreePanelConfig(treeNode, bus), selectionMode: "multiple" };
      }),
    };
  }
}

class LowpolyPlayCataloguePanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: LOWPOLY_PLAY_CATALOGUE_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
      order: 1,
      tree: new CallbackTreePanelDefinition(() => {
        const bus = lowpolyPlayControllerRef.current?.commandBus ?? new CommandBus();
        const treeNode = buildLowpolyPlayCatalogueTree();
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

class LowpolyPlayInspectionPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: LOWPOLY_PLAY_INSPECTION_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, "details"),
      name: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = lowpolyPlayControllerRef.current;
        const bus = ctrl?.commandBus ?? new CommandBus();
        const treeNode = buildLowpolyPlayInspectorTree(ctrl?.getFixtureJson() ?? LOWPOLY_PLAY_DEFAULT_FIXTURE_JSON, { ...(ctrl?.getToolParams() ?? {}) });
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

class LowpolyPlayLayersPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: LOWPOLY_PLAY_LAYERS_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, "details"),
      name: "Layers",
      order: 1,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = lowpolyPlayControllerRef.current;
        const bus = ctrl?.commandBus ?? new CommandBus();
        const treeNode = buildLowpolyPlayLayersTree(ctrl?.getFixtureJson() ?? LOWPOLY_PLAY_DEFAULT_FIXTURE_JSON, ctrl?.getActivePaintLayerIndex() ?? 0);
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

function LowpolyPlayInner({ runtime }: { readonly runtime: Platform }): ReactElement {
  useLowpolyPlayController(runtime);
  const interactionRevision = useLowpolyPlayInteractionRevision(runtime);
  const hierarchyPanel = reactHostPort.useMemo(() => new LowpolyPlayHierarchyPanelDefinition(), []);
  const cataloguePanel = reactHostPort.useMemo(() => new LowpolyPlayCataloguePanelDefinition(), []);
  const inspectionPanel = reactHostPort.useMemo(() => new LowpolyPlayInspectionPanelDefinition(), []);
  const layersPanel = reactHostPort.useMemo(() => new LowpolyPlayLayersPanelDefinition(), []);
  const augmentPanelTabs = reactHostPort.useMemo(
    () => ({
      workbench: [hierarchyPanel, cataloguePanel],
      details: [inspectionPanel, layersPanel],
    }),
    [interactionRevision, cataloguePanel, hierarchyPanel, inspectionPanel, layersPanel],
  );
  return (
    <>
      <LowpolyPlaySessionBridge />
      <PlaygroundView runtime={runtime} defaultAppId={LOWPOLY_PLAY_APP_ID} augmentPanelTabs={augmentPanelTabs} />
    </>
  );
}

function LowpolyPlayChrome({ runtime }: { readonly runtime: Platform }): ReactElement {
  return <LowpolyPlayInner runtime={runtime} />;
}

export function mountLowpolyPlayChrome(playground: Playground, rootId = "root"): void {
  mountPlaygroundApp(<LowpolyPlayChrome runtime={playground.runtime} />, rootId);
}

const lowpolyPlayChromeBoot: PlaygroundChromeBoot = {
  registerHosts: registerLowpolyPlaySurfaceHosts,
  mount: mountLowpolyPlayChrome,
};

export async function bootLowpolyPlay(playground: Playground, rootId = "root"): Promise<void> {
  const ctrl = playground.runtime.getActiveApp()?.controller as LowpolyPlayController | undefined;
  if (ctrl && !isLowpolyFixtureReady(ctrl.getFixtureJson())) {
    const json = await loadDefaultLowpolyFixtureJson();
    ctrl.run("setFixtureJson", { json });
    const fixture = parseLowpolyFixtureJson(json);
    if (fixture) {
      ctrl.run("setSelection", { mode: fixture.selection.mode, ids: [...fixture.selection.ids] });
    }
  }
  bootPlayground(playground, lowpolyPlayChromeBoot, rootId);
}
//#endregion 🔖LowpolyPlayHost

//#region 🔖TrinityPlayHost
import {
  TRINITY_JACK_PLAY_CONTROLLER_ID,
  TRINITY_JACK_PLAY_APP_ID,
  TRINITY_JACK_PLAY_CATALOGUE_TAB_ID,
  TRINITY_JACK_PLAY_DEFAULT_FIXTURE_JSON,
  TRINITY_JACK_PLAY_DEFAULT_QUERY,
  TRINITY_JACK_PLAY_EDITOR_SURFACE_ID,
  TRINITY_JACK_PLAY_HIERARCHY_TAB_ID,
  TRINITY_JACK_PLAY_INSPECTION_TAB_ID,
  TRINITY_JACK_PLAY_RESULTS_SURFACE_ID,
  TRINITY_JACK_PLAY_SURFACE_ID,
  TRINITY_JACK_PLAY_WINDOW_KIND_ID,
  TrinityJackPlayController,
  buildTrinityJackPlayCatalogueTree,
  registerTrinityJackPlayDeclarativeBodies,
} from "@semio-tech/trinity-jack-host-core";
import { buildTrinityPlayHierarchyTree, buildTrinityPlayInspectorTree } from "@semio-tech/trinity-react";
import {
  TRINITY_REWRITE_PLAY_CONTROLLER_ID,
  TRINITY_REWRITE_PLAY_APP_ID,
  TRINITY_REWRITE_PLAY_SURFACE_ID_AFTER,
  TRINITY_REWRITE_PLAY_SURFACE_ID_BEFORE,
  TRINITY_REWRITE_PLAY_SURFACE_ID_JACK,
  TRINITY_REWRITE_PLAY_SURFACE_ID_LHS,
  TRINITY_REWRITE_PLAY_SURFACE_ID_PARAMETERS,
  TRINITY_REWRITE_PLAY_SURFACE_ID_RHS,
  TRINITY_REWRITE_PLAY_WINDOW_KIND_AFTER,
  TRINITY_REWRITE_PLAY_WINDOW_KIND_BEFORE,
  TRINITY_REWRITE_PLAY_WINDOW_KIND_LHS,
  TRINITY_REWRITE_PLAY_WINDOW_KIND_RHS,
  TrinityRewritePlayController,
  REWRITE_DEFAULT_LHS_FIXTURE,
  REWRITE_DEFAULT_LHS_FIXTURE_JSON,
  REWRITE_DEFAULT_RHS_FIXTURE,
  REWRITE_DEFAULT_RHS_FIXTURE_JSON,
  rewriteLhsKindCatalogs,
  rewriteRhsKindCatalogs,
  parseRewriteGraphFixtureJson,
  registerTrinityRewritePlayDeclarativeBodies,
} from "@semio-tech/trinity-rewrite-core";
import {
  TRINITY_DEFAULT_FIXTURE_JSON,
  TRINITY_LOD_MODE_AUTOMATIC,
  TrinityCanvas,
  buildTrinityPlayCatalogueTree,
  createJackLspWorker,
  trinityLodCanvasProps,
  type TrinityDrawLodKind,
} from "@semio-tech/trinity-react";
import { createWorkerLspTransport as createTrinityWriterLspTransport, createWriterDocument as createTrinityWriterDocument } from "@semio-tech/writer-core";
import { WriterCanvas as TrinityWriterCanvas } from "@semio-tech/writer-react";

let trinityPlayChromeRegistered = false;
const trinityJackControllerRef: { current: TrinityJackPlayController | null } = { current: null };
const trinityRewriteControllerRef: { current: TrinityRewritePlayController | null } = { current: null };

function useTrinityJackController(runtimeOverride?: Platform): TrinityJackPlayController | undefined {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const runtime = runtimeOverride ?? appCtx?.runtime;
  reactHostPort.useSyncExternalStore(
    (listener) => (runtime ? runtime.subscribeChrome(listener) : () => {}),
    () => runtime?.chromeGeneration ?? 0,
    () => 0,
  );
  const ctrl = runtime?.getActiveApp()?.controller as TrinityJackPlayController | undefined;
  trinityJackControllerRef.current = ctrl ?? null;
  return ctrl;
}

function useTrinityJackInteractionRevision(runtime?: Platform): number {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const resolved = runtime ?? appCtx?.runtime;
  return reactHostPort.useSyncExternalStore(
    (listener) => {
      const ctrl = resolved?.getActiveApp()?.controller as TrinityJackPlayController | undefined;
      trinityJackControllerRef.current = ctrl ?? null;
      const unsubscribeRuntime = resolved ? resolved.subscribe(listener) : () => {};
      const unsubscribeSnapshot = ctrl?.subscribeSnapshot(listener);
      return () => {
        unsubscribeRuntime();
        unsubscribeSnapshot?.();
      };
    },
    () => (resolved?.getActiveApp()?.controller as TrinityJackPlayController | undefined)?.getInteractionRevision() ?? 0,
    () => 0,
  );
}

function useTrinityRewriteInteractionRevision(runtime?: Platform): number {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const resolved = runtime ?? appCtx?.runtime;
  return reactHostPort.useSyncExternalStore(
    (listener) => {
      const ctrl = resolved?.getActiveApp()?.controller as TrinityRewritePlayController | undefined;
      trinityRewriteControllerRef.current = ctrl ?? null;
      const unsubscribeRuntime = resolved ? resolved.subscribe(listener) : () => {};
      const unsubscribeSnapshot = ctrl?.subscribeSnapshot(listener);
      return () => {
        unsubscribeRuntime();
        unsubscribeSnapshot?.();
      };
    },
    () => (resolved?.getActiveApp()?.controller as TrinityRewritePlayController | undefined)?.getInteractionRevision() ?? 0,
    () => 0,
  );
}

function useTrinityRewriteController(runtimeOverride?: Platform): TrinityRewritePlayController | undefined {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const runtime = runtimeOverride ?? appCtx?.runtime;
  reactHostPort.useSyncExternalStore(
    (listener) => (runtime ? runtime.subscribeChrome(listener) : () => {}),
    () => runtime?.chromeGeneration ?? 0,
    () => 0,
  );
  const ctrl = runtime?.getActiveApp()?.controller as TrinityRewritePlayController | undefined;
  trinityRewriteControllerRef.current = ctrl ?? null;
  return ctrl;
}

function TrinityJackPlaySurfaceHost({ node }: { readonly node: import("@semio-tech/framework-platform-core").UiTrinityHostSurfaceNode }): ReactElement {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const revision = useTrinityJackInteractionRevision(appCtx?.runtime);
  const ctrl = useTrinityJackController();
  const scopeId = node.paneId ?? TRINITY_JACK_PLAY_WINDOW_KIND_ID;
  const lodProps = trinityLodCanvasProps(ctrl?.lodModeForScope(scopeId) ?? TRINITY_LOD_MODE_AUTOMATIC);
  const onFixtureChange = reactHostPort.useCallback((json: string) => ctrl?.run("setFixtureJson", { json }), [ctrl]);
  const onJackDispatchComplete = reactHostPort.useCallback((resultJson: string) => ctrl?.onJackDispatchComplete(resultJson), [ctrl]);
  const onVcsApplied = reactHostPort.useCallback((generation: number) => ctrl?.onVcsApplied(generation), [ctrl]);
  const onSelectionChange = reactHostPort.useCallback((ids: readonly string[]) => ctrl?.run("setSelection", { ids: [...ids] }), [ctrl]);
  const onLodChange = reactHostPort.useCallback(
    (lod: TrinityDrawLodKind) => {
      ctrl?.run("setEffectiveLod", { lod, instanceId: scopeId });
    },
    [ctrl, scopeId],
  );
  void revision;
  return (
    <TrinityCanvas
      fixtureJson={ctrl?.getFixtureJson() ?? TRINITY_JACK_PLAY_DEFAULT_FIXTURE_JSON}
      reorganize={ctrl?.getReorganize()}
      jackDispatch={ctrl?.getJackDispatch()}
      vcsRequest={ctrl?.getVcsRequest()}
      onFixtureChange={onFixtureChange}
      onJackDispatchComplete={onJackDispatchComplete}
      onVcsApplied={onVcsApplied}
      onSelectionChange={onSelectionChange}
      {...lodProps}
      onLodChange={onLodChange}
    />
  );
}

function TrinityJackEditorSurfaceHost({ node }: { readonly node: import("@semio-tech/framework-platform-core").UiWriterHostSurfaceNode }): ReactElement {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const revision = useTrinityJackInteractionRevision(appCtx?.runtime);
  const ctrl = useTrinityJackController();
  void revision;
  const fixtureJson = ctrl?.getFixtureJson() ?? TRINITY_JACK_PLAY_DEFAULT_FIXTURE_JSON;
  const document = ctrl?.getWriterDocument() ?? createTrinityWriterDocument({ id: "jack-query", languageId: "jack", text: TRINITY_JACK_PLAY_DEFAULT_QUERY });
  const createLspTransport = reactHostPort.useCallback(() => createTrinityWriterLspTransport(createJackLspWorker(fixtureJson)), [fixtureJson]);
  const onChange = reactHostPort.useCallback((next: import("@semio-tech/writer-core").WriterDocument) => {
    trinityJackControllerRef.current?.run("setJackQuery", { value: next.text });
  }, []);
  const onSubmit = reactHostPort.useCallback(() => {
    trinityJackControllerRef.current?.run("runJackQuery");
  }, []);
  return (
    <TrinityWriterCanvas
      document={document}
      onChange={onChange}
      onSubmit={onSubmit}
      createLspTransport={createLspTransport}
      fixtureJsonForLsp={fixtureJson}
      placeholder={TRINITY_JACK_PLAY_DEFAULT_QUERY}
      className="h-full"
    />
  );
}

function TrinityJackResultsSurfaceHost({ node }: { readonly node: UiTableHostSurfaceNode }): ReactElement {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const revision = useTrinityJackInteractionRevision(appCtx?.runtime);
  const ctrl = useTrinityJackController();
  const result = reactHostPort.useMemo(() => {
    try {
      return JSON.parse(ctrl?.getJackResultJson() || '{"kind":"table","columns":[],"rows":[]}') as {
        kind?: "table" | "graph";
        columns: string[];
        rows: unknown[][];
        graphFixture?: import("@semio-tech/trinity-react").TrinityFixture;
      };
    } catch {
      return { kind: "table" as const, columns: ["error"], rows: [["Invalid result json"]] };
    }
  }, [ctrl, revision]);
  if (result.kind === "graph" && result.graphFixture) {
    return <TrinityCanvas fixtureJson={JSON.stringify(result.graphFixture)} className="h-full min-h-0" />;
  }
  return (
    <div className="h-full min-h-0 overflow-auto p-2">
      {result.columns.length === 0 ? (
        <div className="text-xs text-muted-foreground">Run a Jack query to see results.</div>
      ) : (
        <table className="w-full border-collapse text-xs">
          <thead>
            <tr>
              {result.columns.map((column) => (
                <th key={column} className="border-b border-border px-2 py-1 text-left font-medium text-muted-foreground">
                  {column}
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            {result.rows.map((row, rowIndex) => (
              <tr key={rowIndex}>
                {row.map((cell, cellIndex) => (
                  <td key={cellIndex} className="border-b border-border px-2 py-1 font-mono text-foreground">
                    {cell == null ? "" : String(cell)}
                  </td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </div>
  );
}

function TrinityRewriteBeforeSurfaceHost({ node }: { readonly node: import("@semio-tech/framework-platform-core").UiTrinityHostSurfaceNode }): ReactElement {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const revision = useTrinityRewriteInteractionRevision(appCtx?.runtime);
  const ctrl = useTrinityRewriteController();
  const scopeId = node.paneId ?? TRINITY_REWRITE_PLAY_WINDOW_KIND_BEFORE;
  const lodProps = trinityLodCanvasProps(ctrl?.lodModeForScope(scopeId) ?? TRINITY_LOD_MODE_AUTOMATIC);
  const onFixtureChange = reactHostPort.useCallback((json: string) => ctrl?.run("setFixtureJson", { json }), [ctrl]);
  const onJackDispatchComplete = reactHostPort.useCallback((resultJson: string) => ctrl?.onBeforeJackDispatchComplete(resultJson), [ctrl]);
  const onVcsApplied = reactHostPort.useCallback((generation: number) => ctrl?.onVcsApplied(generation), [ctrl]);
  const onSelectionChange = reactHostPort.useCallback((ids: readonly string[]) => ctrl?.run("setSelection", { ids: [...ids] }), [ctrl]);
  const onLodChange = reactHostPort.useCallback(
    (lod: TrinityDrawLodKind) => {
      ctrl?.run("setEffectiveLod", { lod, instanceId: scopeId });
    },
    [ctrl, scopeId],
  );
  void revision;
  void ctrl?.getHoverEpoch();
  void ctrl?.getSelectEpoch();
  return (
    <TrinityCanvas
      fixtureJson={ctrl?.getBeforeFixtureJson() ?? TRINITY_DEFAULT_FIXTURE_JSON}
      reorganize={ctrl?.getReorganize()}
      jackDispatch={ctrl?.getBeforeJackDispatch()}
      vcsRequest={ctrl?.getVcsRequest()}
      highlightedNodeIds={ctrl?.getBeforeHighlightedNodeIds()}
      highlightedNodeIdsSignal={ctrl?.getHoverEpoch() + ctrl?.getSelectEpoch()}
      onFixtureChange={onFixtureChange}
      onJackDispatchComplete={onJackDispatchComplete}
      onVcsApplied={onVcsApplied}
      onSelectionChange={onSelectionChange}
      {...lodProps}
      onLodChange={onLodChange}
    />
  );
}

function TrinityRewriteAfterSurfaceHost({ node }: { readonly node: import("@semio-tech/framework-platform-core").UiTrinityHostSurfaceNode }): ReactElement {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const revision = useTrinityRewriteInteractionRevision(appCtx?.runtime);
  const ctrl = useTrinityRewriteController();
  const scopeId = node.paneId ?? TRINITY_REWRITE_PLAY_WINDOW_KIND_AFTER;
  const lodProps = trinityLodCanvasProps(ctrl?.lodModeForScope(scopeId) ?? TRINITY_LOD_MODE_AUTOMATIC);
  const onLodChange = reactHostPort.useCallback(
    (lod: TrinityDrawLodKind) => {
      ctrl?.run("setEffectiveLod", { lod, instanceId: scopeId });
    },
    [ctrl, scopeId],
  );
  void revision;
  void ctrl?.getHoverEpoch();
  void ctrl?.getSelectEpoch();
  return (
    <TrinityCanvas
      fixtureJson={ctrl?.getAfterFixtureJson() ?? TRINITY_DEFAULT_FIXTURE_JSON}
      highlightedNodeIds={ctrl?.getAfterHighlightedNodeIds()}
      highlightedNodeIdsSignal={ctrl?.getHoverEpoch() + ctrl?.getSelectEpoch()}
      {...lodProps}
      onLodChange={onLodChange}
      className="h-full min-h-0"
    />
  );
}

function TrinityRewriteLhsSurfaceHost({ node: _node }: { readonly node: import("@semio-tech/framework-platform-core").UiPuzzle2dHostSurfaceNode }): ReactElement {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const revision = useTrinityRewriteInteractionRevision(appCtx?.runtime);
  const ctrl = useTrinityRewriteController();
  const kindCatalogs = reactHostPort.useMemo(() => rewriteLhsKindCatalogs(), []);
  const fixture = reactHostPort.useMemo(() => {
    return parseRewriteGraphFixtureJson(ctrl?.getLhsFixtureJson() ?? REWRITE_DEFAULT_LHS_FIXTURE_JSON) ?? REWRITE_DEFAULT_LHS_FIXTURE;
  }, [ctrl, revision]);
  const declarativeSceneDescriptor = reactHostPort.useMemo(() => buildPuzzle2dSceneDescriptorFromFixture(fixture), [fixture]);
  const onDragEnd = reactHostPort.useCallback(
    (payload: { moves: Array<{ id: string; x: number; y: number }> }) => {
      if (!payload.moves.length) return;
      const current = parseRewriteGraphFixtureJson(trinityRewriteControllerRef.current?.getLhsFixtureJson() ?? REWRITE_DEFAULT_LHS_FIXTURE_JSON);
      if (!current) return;
      const byId = new Map(payload.moves.map((move) => [move.id, move]));
      trinityRewriteControllerRef.current?.run("setLhsFixtureJson", {
        json: JSON.stringify({
          ...current,
          nodes: current.nodes.map((entry) => {
            const move = byId.get(entry.id);
            return move ? { ...entry, x: move.x, y: move.y } : entry;
          }),
        }),
      });
    },
    [],
  );
  const onHover = reactHostPort.useCallback((payload: Puzzle2dHoverPayload) => {
    trinityRewriteControllerRef.current?.run("setLhsGraphHover", { id: payload.id });
  }, []);
  const onSelect = reactHostPort.useCallback((snapshot: { ids: readonly string[] }) => {
    trinityRewriteControllerRef.current?.run("setLhsGraphSelect", { ids: [...snapshot.ids] });
  }, []);
  void revision;
  void ctrl?.getHoverEpoch();
  void ctrl?.getSelectEpoch();
  return (
    <Puzzle2dCanvas
      declarativeSceneDescriptor={declarativeSceneDescriptor}
      camera={fixture.camera}
      kindCatalogs={kindCatalogs}
      fixtureDragDrop
      hoveredId={ctrl?.getLhsHoveredNodeId() ?? null}
      preselection={ctrl?.getLhsVarPreselection()}
      selection={ctrl?.getLhsVarSelection()}
      onDragEnd={onDragEnd}
      onHover={onHover}
      onSelect={onSelect}
      className="h-full min-h-0"
    />
  );
}

function TrinityRewriteRhsSurfaceHost({ node: _node }: { readonly node: import("@semio-tech/framework-platform-core").UiPuzzle2dHostSurfaceNode }): ReactElement {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const revision = useTrinityRewriteInteractionRevision(appCtx?.runtime);
  const ctrl = useTrinityRewriteController();
  const kindCatalogs = reactHostPort.useMemo(() => rewriteRhsKindCatalogs(), []);
  const fixture = reactHostPort.useMemo(() => {
    return parseRewriteGraphFixtureJson(ctrl?.getRhsFixtureJson() ?? REWRITE_DEFAULT_RHS_FIXTURE_JSON) ?? REWRITE_DEFAULT_RHS_FIXTURE;
  }, [ctrl, revision]);
  const declarativeSceneDescriptor = reactHostPort.useMemo(() => buildPuzzle2dSceneDescriptorFromFixture(fixture), [fixture]);
  const onDragEnd = reactHostPort.useCallback(
    (payload: { moves: Array<{ id: string; x: number; y: number }> }) => {
      if (!payload.moves.length) return;
      const current = parseRewriteGraphFixtureJson(trinityRewriteControllerRef.current?.getRhsFixtureJson() ?? REWRITE_DEFAULT_RHS_FIXTURE_JSON);
      if (!current) return;
      const byId = new Map(payload.moves.map((move) => [move.id, move]));
      trinityRewriteControllerRef.current?.run("setRhsFixtureJson", {
        json: JSON.stringify({
          ...current,
          nodes: current.nodes.map((entry) => {
            const move = byId.get(entry.id);
            return move ? { ...entry, x: move.x, y: move.y } : entry;
          }),
        }),
      });
    },
    [],
  );
  const onHover = reactHostPort.useCallback((payload: Puzzle2dHoverPayload) => {
    trinityRewriteControllerRef.current?.run("setRhsGraphHover", { id: payload.id });
  }, []);
  const onSelect = reactHostPort.useCallback((snapshot: { ids: readonly string[] }) => {
    trinityRewriteControllerRef.current?.run("setRhsGraphSelect", { ids: [...snapshot.ids] });
  }, []);
  void revision;
  void ctrl?.getHoverEpoch();
  void ctrl?.getSelectEpoch();
  return (
    <Puzzle2dCanvas
      declarativeSceneDescriptor={declarativeSceneDescriptor}
      camera={fixture.camera}
      kindCatalogs={kindCatalogs}
      fixtureDragDrop
      hoveredId={ctrl?.getRhsHoveredNodeId() ?? null}
      preselection={ctrl?.getRhsVarPreselection()}
      selection={ctrl?.getRhsVarSelection()}
      onDragEnd={onDragEnd}
      onHover={onHover}
      onSelect={onSelect}
      className="h-full min-h-0"
    />
  );
}

function TrinityRewriteJackSurfaceHost({ node: _node }: { readonly node: import("@semio-tech/framework-platform-core").UiWriterHostSurfaceNode }): ReactElement {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const revision = useTrinityRewriteInteractionRevision(appCtx?.runtime);
  const ctrl = useTrinityRewriteController();
  void revision;
  void ctrl?.getHoverEpoch();
  void ctrl?.getSelectEpoch();
  const document = ctrl?.getWriterDocumentJack() ?? createTrinityWriterDocument({ id: "rewrite-jack", languageId: "jack", text: "" });
  const onHoverChange = reactHostPort.useCallback((offset: number | null) => {
    trinityRewriteControllerRef.current?.run("setJackHover", { offset });
  }, []);
  const onSelectionChange = reactHostPort.useCallback((range: { start: number; end: number }) => {
    trinityRewriteControllerRef.current?.run("setJackSelect", range);
  }, []);
  return (
    <TrinityWriterCanvas
      document={document}
      className="h-full"
      placeholder="Generated Jack query"
      onHoverChange={onHoverChange}
      onSelectionChange={onSelectionChange}
      externalHoverOccurrences={ctrl?.getJackHoverOccurrences()}
      externalHoverOccurrencesSignal={ctrl?.getHoverEpoch()}
      externalSelectionOccurrences={ctrl?.getJackSelectOccurrences()}
      externalSelectionOccurrencesSignal={ctrl?.getSelectEpoch()}
    />
  );
}

function TrinityRewriteParametersSurfaceHost({ node: _node }: { readonly node: import("@semio-tech/framework-platform-core").UiFormsHostSurfaceNode }): ReactElement {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const revision = useTrinityRewriteInteractionRevision(appCtx?.runtime);
  const ctrl = useTrinityRewriteController();
  void revision;
  const spec = ctrl?.getParameterFormSpec();
  const values = ctrl?.getParameterValues() ?? {};
  if (!spec || spec.steps[0]?.questions.length === 0) {
    return <div className="p-double text-sm text-muted-foreground">No parameters declared on RHS.</div>;
  }
  return (
    <FormRenderer
      spec={spec}
      values={values}
      className="h-full"
      onChange={(next) => ctrl?.run("setParameterValues", { values: next })}
    />
  );
}

export function registerTrinityJackPlaySurfaceHosts(): void {
  if (trinityPlayChromeRegistered) return;
  trinityPlayChromeRegistered = true;
  registerUiTrinitySurfaceHost(TRINITY_JACK_PLAY_SURFACE_ID, TrinityJackPlaySurfaceHost);
  registerUiWriterSurfaceHost(TRINITY_JACK_PLAY_EDITOR_SURFACE_ID, TrinityJackEditorSurfaceHost);
  registerUiTableSurfaceHost(TRINITY_JACK_PLAY_RESULTS_SURFACE_ID, TrinityJackResultsSurfaceHost);
  registerTrinityJackPlayDeclarativeBodies();
}

export function registerTrinityRewritePlaySurfaceHosts(): void {
  registerUiTrinitySurfaceHost(TRINITY_REWRITE_PLAY_SURFACE_ID_BEFORE, TrinityRewriteBeforeSurfaceHost);
  registerUiTrinitySurfaceHost(TRINITY_REWRITE_PLAY_SURFACE_ID_AFTER, TrinityRewriteAfterSurfaceHost);
  registerUiPuzzle2dSurfaceHost(TRINITY_REWRITE_PLAY_SURFACE_ID_LHS, TrinityRewriteLhsSurfaceHost);
  registerUiPuzzle2dSurfaceHost(TRINITY_REWRITE_PLAY_SURFACE_ID_RHS, TrinityRewriteRhsSurfaceHost);
  registerUiWriterSurfaceHost(TRINITY_REWRITE_PLAY_SURFACE_ID_JACK, TrinityRewriteJackSurfaceHost);
  registerUiFormsSurfaceHost(TRINITY_REWRITE_PLAY_SURFACE_ID_PARAMETERS, TrinityRewriteParametersSurfaceHost);
  registerTrinityRewritePlayDeclarativeBodies();
}

class TrinityJackHierarchyPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: TRINITY_JACK_PLAY_HIERARCHY_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = trinityJackControllerRef.current;
        const bus = new CommandBus();
        return uiTreeNodeToTreePanelConfig(
          buildTrinityPlayHierarchyTree(ctrl?.getFixtureJson() ?? TRINITY_JACK_PLAY_DEFAULT_FIXTURE_JSON, ctrl?.getSelectedNodeIds() ?? []),
          bus,
        );
      }),
    };
  }
}

class TrinityJackCataloguePanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: TRINITY_JACK_PLAY_CATALOGUE_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
      order: 1,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = trinityJackControllerRef.current;
        const bus = new CommandBus();
        return uiTreeNodeToTreePanelConfig(buildTrinityJackPlayCatalogueTree(ctrl?.getActiveFixtureId()), bus);
      }),
    };
  }
}

class TrinityJackInspectionPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: TRINITY_JACK_PLAY_INSPECTION_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, "details"),
      name: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = trinityJackControllerRef.current;
        const bus = new CommandBus();
        return uiTreeNodeToTreePanelConfig(
          buildTrinityPlayInspectorTree(
            ctrl?.getFixtureJson() ?? TRINITY_JACK_PLAY_DEFAULT_FIXTURE_JSON,
            ctrl?.getSelectedNodeIds() ?? [],
            TRINITY_JACK_PLAY_CONTROLLER_ID,
          ),
          bus,
        );
      }),
    };
  }
}

class TrinityRewriteHierarchyPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: TRINITY_JACK_PLAY_HIERARCHY_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = trinityRewriteControllerRef.current;
        const bus = new CommandBus();
        return uiTreeNodeToTreePanelConfig(
          buildTrinityPlayHierarchyTree(ctrl?.getBeforeFixtureJson() ?? TRINITY_DEFAULT_FIXTURE_JSON, ctrl?.getSelectedNodeIds() ?? []),
          bus,
        );
      }),
    };
  }
}

class TrinityRewriteCataloguePanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: TRINITY_JACK_PLAY_CATALOGUE_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
      order: 1,
      tree: new CallbackTreePanelDefinition(() => {
        const bus = new CommandBus();
        return uiTreeNodeToTreePanelConfig(buildTrinityPlayCatalogueTree(), bus);
      }),
    };
  }
}

class TrinityRewriteInspectionPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: TRINITY_JACK_PLAY_INSPECTION_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, "details"),
      name: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = trinityRewriteControllerRef.current;
        const bus = new CommandBus();
        return uiTreeNodeToTreePanelConfig(
          buildTrinityPlayInspectorTree(
            ctrl?.getBeforeFixtureJson() ?? TRINITY_DEFAULT_FIXTURE_JSON,
            ctrl?.getSelectedNodeIds() ?? [],
            TRINITY_REWRITE_PLAY_CONTROLLER_ID,
          ),
          bus,
        );
      }),
    };
  }
}

function TrinityJackPlayInner({ runtime }: { readonly runtime: Platform }): ReactElement {
  useTrinityJackController(runtime);
  const hierarchy = reactHostPort.useMemo(() => new TrinityJackHierarchyPanelDefinition(), []);
  const catalogue = reactHostPort.useMemo(() => new TrinityJackCataloguePanelDefinition(), []);
  const inspection = reactHostPort.useMemo(() => new TrinityJackInspectionPanelDefinition(), []);
  return (
    <PlaygroundView
      runtime={runtime}
      defaultAppId={TRINITY_JACK_PLAY_APP_ID}
      augmentPanelTabs={{ workbench: [hierarchy, catalogue], details: [inspection] }}
    />
  );
}

function TrinityRewritePlayInner({ runtime }: { readonly runtime: Platform }): ReactElement {
  useTrinityRewriteController(runtime);
  const hierarchy = reactHostPort.useMemo(() => new TrinityRewriteHierarchyPanelDefinition(), []);
  const catalogue = reactHostPort.useMemo(() => new TrinityRewriteCataloguePanelDefinition(), []);
  const inspection = reactHostPort.useMemo(() => new TrinityRewriteInspectionPanelDefinition(), []);
  return (
    <PlaygroundView
      runtime={runtime}
      defaultAppId={TRINITY_REWRITE_PLAY_APP_ID}
      augmentPanelTabs={{ workbench: [hierarchy, catalogue], details: [inspection] }}
    />
  );
}

export function mountTrinityJackPlayChrome(playground: Playground, rootId = "root"): void {
  mountPlaygroundApp(<TrinityJackPlayInner runtime={playground.runtime} />, rootId);
}

export function mountTrinityRewritePlayChrome(playground: Playground, rootId = "root"): void {
  mountPlaygroundApp(<TrinityRewritePlayInner runtime={playground.runtime} />, rootId);
}

const trinityJackPlayChromeBoot: PlaygroundChromeBoot = {
  registerHosts: registerTrinityJackPlaySurfaceHosts,
  mount: mountTrinityJackPlayChrome,
};

const trinityRewritePlayChromeBoot: PlaygroundChromeBoot = {
  registerHosts: registerTrinityRewritePlaySurfaceHosts,
  mount: mountTrinityRewritePlayChrome,
};

export function bootTrinityJackPlay(playground: Playground, rootId = "root"): void {
  bootPlayground(playground, trinityJackPlayChromeBoot, rootId);
}

export function bootTrinityRewritePlay(playground: Playground, rootId = "root"): void {
  bootPlayground(playground, trinityRewritePlayChromeBoot, rootId);
}
//#endregion 🔖TrinityPlayHost

//#region 🔖ProceduralPlayHost
import type { UiPanelHostSurfaceNode } from "@semio-tech/framework-platform-core";
import { ProceduralFlowEditor, ProceduralPreview, useProceduralBrepBridge } from "@semio-tech/procedural-3d-react";
import {
    DAG_LOD_MODE_AUTOMATIC as PROCEDURAL_3D_DAG_LOD_MODE_AUTOMATIC,
    FLOW_DEFAULT_PROXIMITY_DISTANCE as PROCEDURAL_3D_DEFAULT_PROXIMITY_DISTANCE,
    dagLodCanvasProps as procedural3dDagLodCanvasProps,
    flowWidgetPaletteTreeDragController as procedural3dWidgetPaletteTreeDragController,
} from "@semio-tech/flow-react";
import {
    PROCEDURAL_3D_PLAY_APP_ID,
    PROCEDURAL_PLAY_CATALOGUE_TAB_ID,
    PROCEDURAL_PLAY_HIERARCHY_TAB_ID,
    PROCEDURAL_PLAY_INSPECTION_TAB_ID,
    PROCEDURAL_PLAY_SURFACE_ID,
    PROCEDURAL_PLAY_SURFACE_ID_GENERATE,
    PROCEDURAL_PLAY_SURFACE_ID_PREVIEW,
    ProceduralPlayController,
    buildProceduralPlayCanvasContextMenu,
    buildProceduralPlayCatalogueTree,
    buildProceduralPlayHierarchyTree,
    buildProceduralPlayInspectorTree,
    registerProceduralPlayDeclarativeBodies,
    type ProceduralPlayHostBridge,
} from "@semio-tech/procedural-3d-core";
import { PROCEDURAL_PLAY_EMPTY_FIXTURE_JSON } from "@semio-tech/procedural-3d-core";

let proceduralPlayChromeRegistered = false;
const proceduralPlayControllerRef: { current: ProceduralPlayController | null } = { current: null };

/** @emoji 🔔 Re-renders procedural play workbench kinds when WASM catalogue sections arrive. */
function useProceduralPlaySnapshotRevision(runtime: Platform, selector: (ctrl: ProceduralPlayController) => number): number {
  return reactHostPort.useSyncExternalStore(
    (listener) => {
      const ctrl = runtime.getActiveApp()?.controller as ProceduralPlayController | undefined;
      proceduralPlayControllerRef.current = ctrl ?? null;
      const unsubscribeChrome = runtime.subscribeChrome(listener);
      const unsubscribeSnapshot = ctrl?.subscribeSnapshot(listener);
      return () => {
        unsubscribeChrome();
        unsubscribeSnapshot?.();
      };
    },
    () => {
      const ctrl = runtime.getActiveApp()?.controller as ProceduralPlayController | undefined;
      proceduralPlayControllerRef.current = ctrl ?? null;
      return ctrl ? selector(ctrl) : 0;
    },
    () => 0,
  );
}

function useProceduralPlayCatalogueRevision(runtime: Platform): number {
  return useProceduralPlaySnapshotRevision(runtime, (c) => c.getCatalogueRevision());
}

function useProceduralPlayExtensionRevision(runtime: Platform): number {
  return useProceduralPlaySnapshotRevision(runtime, (c) => c.getExtensionRevision());
}

function useProceduralPlayInteractionRevision(runtime: Platform): number {
  return useProceduralPlaySnapshotRevision(runtime, (c) => c.getInteractionRevision());
}

function useProceduralPlayController(runtimeOverride?: Platform): ProceduralPlayController | undefined {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const runtime = runtimeOverride ?? appCtx?.runtime;
  reactHostPort.useSyncExternalStore(
    (listener) => (runtime ? runtime.subscribeChrome(listener) : () => {}),
    () => runtime?.chromeGeneration ?? 0,
    () => 0,
  );
  const ctrl = runtime?.getActiveApp()?.controller as ProceduralPlayController | undefined;
  proceduralPlayControllerRef.current = ctrl ?? null;
  return ctrl;
}

async function downloadProceduralFixtureJson(name: string, text: string): Promise<void> {
  const pickerWindow = window as Window & { showSaveFilePicker?: (options?: { suggestedName?: string; types?: { description: string; accept: Record<string, string[]> }[] }) => Promise<FileSystemFileHandle> };
  if (pickerWindow.showSaveFilePicker) {
    const handle = await pickerWindow.showSaveFilePicker({
      suggestedName: name,
      types: [{ description: "Flow fixture JSON", accept: { "application/json": [".json"] } }],
    });
    const writable = await handle.createWritable();
    await writable.write(`${text}\n`);
    await writable.close();
    return;
  }
  const href = URL.createObjectURL(new Blob([`${text}\n`], { type: "application/json" }));
  const link = document.createElement("a");
  link.href = href;
  link.download = name;
  link.click();
  URL.revokeObjectURL(href);
}

function ProceduralPlayToolbarHostBridge({ runtime, ctrl }: { readonly runtime: Platform; readonly ctrl: ProceduralPlayController | undefined }): ReactElement {
  const interactionRevision = useProceduralPlayInteractionRevision(runtime);
  const loadInputRef = reactHostPort.useRef<HTMLInputElement>(null);
  const downloadFixture = reactHostPort.useCallback(async () => {
    const json = ctrl?.getFixtureJson() ?? PROCEDURAL_PLAY_EMPTY_FIXTURE_JSON;
    try {
      await downloadProceduralFixtureJson("procedural.fixture.json", json);
      console.log("[DEBUG] procedural play downloaded fixture");
    } catch (error) {
      console.log(`[DEBUG] procedural play download failed: ${String(error)}`);
    }
  }, [ctrl]);
  const handleLoadFile = reactHostPort.useCallback(
    (event: reactHostPort.ChangeEvent<HTMLInputElement>) => {
      const file = event.target.files?.[0];
      event.target.value = "";
      if (!file || !ctrl) return;
      void file.text().then((text) => {
        if (!text.includes("flow.fixture")) {
          console.log("[DEBUG] procedural play load rejected: not a flow fixture");
          return;
        }
        ctrl.run("setFixtureJson", { json: text, resetInteraction: true });
        console.log("[DEBUG] procedural play loaded fixture from file");
      });
    },
    [ctrl],
  );
  reactHostPort.useEffect(() => {
    if (!ctrl) return;
    const bridge: ProceduralPlayHostBridge = {
      getToolbarState: () => ({
        selectionMethod: ctrl.getSelectionMethod(),
        selectionMode: ctrl.getSelectionMode(),
        showMode: ctrl.getShowMode(),
        selectionCount: ctrl.getSelectedNodeIds().length,
        hasStoredFixture: ctrl.hasStoredFixture(),
      }),
      runHostCommand: (command) => {
        if (command === "saveDownload") {
          void downloadFixture();
          return;
        }
        if (command === "loadRequest") {
          loadInputRef.current?.click();
        }
      },
    };
    ctrl.setHostBridge(bridge);
    return () => ctrl.setHostBridge(null);
  }, [ctrl, downloadFixture, interactionRevision]);
  return <input ref={loadInputRef} type="file" accept=".json,application/json" className="hidden" onChange={handleLoadFile} />;
}

function ProceduralPlayPaneSurfaceHost({ node }: { readonly node: UiFlowHostSurfaceNode }): ReactElement {
  const { runtime } = useApp();
  const ctrl = useProceduralPlayController();
  const extensionRevision = useProceduralPlayExtensionRevision(runtime);
  const interactionRevision = useProceduralPlayInteractionRevision(runtime);
  void interactionRevision;
  const scopeId = node.paneId ?? PROCEDURAL_PLAY_WINDOW_KIND_ID;
  const lodProps = procedural3dDagLodCanvasProps(ctrl?.lodModeForScope(scopeId) ?? PROCEDURAL_3D_DAG_LOD_MODE_AUTOMATIC);
  const proximityDistance = ctrl?.proximityDistanceValue() ?? PROCEDURAL_3D_DEFAULT_PROXIMITY_DISTANCE;
  const onLodChange = reactHostPort.useCallback(
    (lod: import("@semio-tech/flow-react").DagDrawLodKind) => {
      ctrl?.run("setEffectiveLod", { lod, instanceId: scopeId });
    },
    [ctrl, scopeId],
  );
  const onPreviewText = reactHostPort.useCallback(
    (text: string) => {
      console.log(`[DEBUG] procedural play preview: ${text}`);
      ctrl?.run("setPreviewText", { text });
    },
    [ctrl],
  );
  const onEvalOutputs = reactHostPort.useCallback(
    (outputsJson: string, previewMeshes?: Readonly<Record<string, unknown>>) => {
      console.log(`[DEBUG] procedural play eval outputs: ${outputsJson.slice(0, 120)}`);
      ctrl?.run("setEvalOutputs", { outputsJson, previewMeshes });
    },
    [ctrl],
  );
  const onCatalogueReady = reactHostPort.useCallback(
    (sections: readonly import("@semio-tech/flow-react").CatalogueSection[]) => {
      ctrl?.run("setCatalogueSections", { sections: [...sections] });
    },
    [ctrl],
  );
  const onFixtureChange = reactHostPort.useCallback(
    (json: string) => {
      ctrl?.run("setFixtureJson", { json });
    },
    [ctrl],
  );
  const onSelectionChange = reactHostPort.useCallback(
    (ids: readonly string[]) => {
      ctrl?.run("setSelection", { ids: [...ids], mode: "default", fromFlow: true });
    },
    [ctrl],
  );
  const onPreselectChange = reactHostPort.useCallback(
    (snapshot: { readonly ids: readonly string[]; readonly removedIds: readonly string[] }) => {
      ctrl?.run("setPreselect", { ids: [...snapshot.ids], removedIds: [...snapshot.removedIds] });
    },
    [ctrl],
  );
  const onHoverChange = reactHostPort.useCallback(
    (id: string | null) => {
      ctrl?.run("setHover", { id, channel: null });
    },
    [ctrl],
  );
  const onChannelHoverChange = reactHostPort.useCallback(
    (channel: import("@semio-tech/procedural-3d-react").ProceduralChannelRef | null) => {
      ctrl?.run("setHover", { id: channel?.widgetId ?? null, channel });
    },
    [ctrl],
  );
  const onSelectedChannelsChange = reactHostPort.useCallback(
    (channels: readonly import("@semio-tech/procedural-3d-react").ProceduralChannelRef[]) => {
      ctrl?.run("setSelectedChannels", { channels: [...channels] });
    },
    [ctrl],
  );
  const onPreviewOffChange = reactHostPort.useCallback(
    (ids: readonly string[]) => {
      ctrl?.run("setPreviewOff", { ids: [...ids], fromFlow: true });
    },
    [ctrl],
  );
  const onCanvasCommand = reactHostPort.useCallback(
    (command: string, args?: Record<string, unknown>) => {
      ctrl?.run(command, args);
    },
    [ctrl],
  );
  const onOutputExport = reactHostPort.useCallback(
    (widgetId: string, format: string, resolvedValueJson: string) => {
      void downloadFlowOutputExport(format, resolvedValueJson, widgetId).catch((error) => {
        console.log(`[DEBUG] procedural play export failed: ${String(error)}`);
      });
    },
    [],
  );
  return (
    <ProceduralFlowEditor
      fixtureJson={ctrl?.getFixtureJson() ?? PROCEDURAL_PLAY_EMPTY_FIXTURE_JSON}
      reorganize={ctrl?.getReorganize()}
      commandRequest={ctrl?.getCommandRequest()}
      extensionRevision={extensionRevision}
      onPreviewText={onPreviewText}
      onEvalOutputs={onEvalOutputs}
      onOutputExport={onOutputExport}
      onCatalogueReady={onCatalogueReady}
      onFixtureChange={onFixtureChange}
      onSelectionChange={onSelectionChange}
      onPreselectChange={onPreselectChange}
      onHoverChange={onHoverChange}
      onChannelHoverChange={onChannelHoverChange}
      onSelectedChannelsChange={onSelectedChannelsChange}
      onPreviewOffChange={onPreviewOffChange}
      selectedNodeIds={ctrl?.getSelectedNodeIds()}
      selectedChannels={ctrl?.getSelectedChannels()}
      preselectNodeIds={ctrl?.getPreselectNodeIds()}
      preselectRemovedNodeIds={ctrl?.getPreselectRemovedNodeIds()}
      hoveredNodeId={ctrl?.getHoveredNodeId()}
      hoveredChannel={ctrl?.getHoveredChannel()}
      previewOffNodeIds={ctrl?.getPreviewOffNodeIds()}
      selectionMode={ctrl?.getSelectionMode()}
      selectionMethod={ctrl?.getSelectionMethod()}
      contextMenu={(ctx) => buildProceduralPlayCanvasContextMenu(ctx, onCanvasCommand)}
      {...lodProps}
      onLodChange={onLodChange}
      proximityDistance={proximityDistance}
      className="h-full w-full"
    />
  );
}

function ProceduralPreviewSurfaceHost({ node: _node }: { readonly node: UiPuzzle3dHostSurfaceNode }): ReactElement {
  const { runtime } = useApp();
  const ctrl = useProceduralPlayController();
  const brepBridge = useProceduralBrepBridge();
  const interactionRevision = useProceduralPlayInteractionRevision(runtime);
  void interactionRevision;
  const onHover = reactHostPort.useCallback(
    (channel: import("@semio-tech/procedural-3d-react").ProceduralChannelRef | null) => {
      ctrl?.run("setHover", { id: channel?.widgetId ?? null, channel });
    },
    [ctrl],
  );
  const onSelect = reactHostPort.useCallback(
    (channel: import("@semio-tech/procedural-3d-react").ProceduralChannelRef) => {
      ctrl?.run("setSelectChannels", { channels: [channel] });
    },
    [ctrl],
  );
  const onSelectionChange = reactHostPort.useCallback(
    (ids: readonly string[], mode: import("@semio-tech/procedural-3d-react").ProceduralSelectionMode) => {
      ctrl?.run("setSelection", { ids: [...ids], mode });
    },
    [ctrl],
  );
  const onGumballTransform = reactHostPort.useCallback(
    (request: import("@semio-tech/procedural-3d-react").ProceduralGumballTransformRequest) => {
      ctrl?.run("applyGumballTransform", request);
    },
    [ctrl],
  );
  return (
    <div className="absolute inset-0 min-h-0 min-w-0">
      <ProceduralPreview
        items={ctrl?.getPreviewItems() ?? []}
        selectedNodeIds={ctrl?.getSelectedNodeIds()}
        preselectNodeIds={ctrl?.getPreselectNodeIds()}
        preselectRemovedNodeIds={ctrl?.getPreselectRemovedNodeIds()}
        hoveredNodeId={ctrl?.getHoveredNodeId()}
        hoveredChannel={ctrl?.getHoveredChannel()}
        hoveredGeometryTargets={ctrl?.getHoveredGeometryTargets()}
        selectedChannels={ctrl?.getSelectedChannels()}
        selectedGeometryTargets={ctrl?.getSelectedGeometryTargets()}
        previewOffNodeIds={ctrl?.getPreviewOffNodeIds()}
        showMode={ctrl?.getShowMode() ?? "everything"}
        selectionMode={ctrl?.getSelectionMode()}
        selectionMethod={ctrl?.getSelectionMethod()}
        transformGranularity={ctrl?.getTransformGranularity() ?? "full"}
        onGumballTransform={onGumballTransform}
        gumballActiveWidgetIds={ctrl?.getGumballActiveWidgetIds()}
        onHover={onHover}
        onSelect={onSelect}
        onSelectionChange={onSelectionChange}
        kernel={brepBridge ?? undefined}
        className="h-full w-full"
      />
    </div>
  );
}

class ProceduralPlayHierarchyPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: PROCEDURAL_PLAY_HIERARCHY_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = proceduralPlayControllerRef.current;
        const bus = new CommandBus();
        const treeNode = buildProceduralPlayHierarchyTree(ctrl?.getFixtureJson() ?? PROCEDURAL_PLAY_EMPTY_FIXTURE_JSON, ctrl?.getSelectedNodeIds() ?? []);
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

class ProceduralPlayCataloguePanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: PROCEDURAL_PLAY_CATALOGUE_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
      order: 1,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = proceduralPlayControllerRef.current;
        const bus = new CommandBus();
        const treeNode = buildProceduralPlayCatalogueTree(ctrl?.getCatalogueSections() ?? [], ctrl?.getExtensionEntries() ?? []);
        const config = uiTreeNodeToTreePanelConfig(treeNode, bus);
        return {
          ...config,
          dragAndDropController: procedural3dWidgetPaletteTreeDragController(collectUiTreeItemDragData(treeNode.sections)),
        };
      }),
    };
  }
}

class ProceduralPlayInspectionPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: PROCEDURAL_PLAY_INSPECTION_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, "details"),
      name: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = proceduralPlayControllerRef.current;
        const bus = new CommandBus();
        const treeNode = buildProceduralPlayInspectorTree(ctrl?.getFixtureJson() ?? PROCEDURAL_PLAY_EMPTY_FIXTURE_JSON, ctrl?.getSelectedNodeIds() ?? []);
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

function ProceduralPlayInner({ runtime }: { readonly runtime: Platform }): ReactElement {
  const ctrl = useProceduralPlayController(runtime);
  const catalogueRevision = useProceduralPlayCatalogueRevision(runtime);
  const extensionRevision = useProceduralPlayExtensionRevision(runtime);
  const interactionRevision = useProceduralPlayInteractionRevision(runtime);
  const proceduralPlayHierarchyPanel = reactHostPort.useMemo(() => new ProceduralPlayHierarchyPanelDefinition(), []);
  const proceduralPlayCataloguePanel = reactHostPort.useMemo(() => new ProceduralPlayCataloguePanelDefinition(), []);
  const proceduralPlayInspectionPanel = reactHostPort.useMemo(() => new ProceduralPlayInspectionPanelDefinition(), []);
  const augmentPanelTabs = reactHostPort.useMemo(
    () => ({
      workbench: [proceduralPlayHierarchyPanel, proceduralPlayCataloguePanel],
      details: [proceduralPlayInspectionPanel],
    }),
    [catalogueRevision, extensionRevision, interactionRevision, proceduralPlayCataloguePanel, proceduralPlayHierarchyPanel, proceduralPlayInspectionPanel],
  );
  return (
    <>
      <ProceduralPlayToolbarHostBridge runtime={runtime} ctrl={ctrl} />
      <PlaygroundView runtime={runtime} defaultAppId={PROCEDURAL_3D_PLAY_APP_ID} augmentPanelTabs={augmentPanelTabs} />
    </>
  );
}

function Procedural3dGenerateSurfaceHost({ node }: { readonly node: UiFormsHostSurfaceNode }): ReactElement {
  const ctrl = useProceduralPlayController();
  const spec = reactHostPort.useMemo(() => {
    try {
      return parseFormSpec(JSON.parse(ctrl?.getGenerateFormSpecJson() ?? "{}"));
    } catch {
      return parseFormSpec({ schema: "forms.form", id: "empty", version: "1", steps: [{ id: "s", title: "Inputs", questions: [] }] });
    }
  }, [ctrl]);
  return (
    <FlowGenerateSurface
      formSpec={spec}
      generations={[...(ctrl?.getGenerations() ?? [])]}
      selectedGenerationId={ctrl?.getSelectedGenerationId() ?? null}
      previewText={ctrl?.getGeneratePreviewText() ?? "—"}
      onSelectGeneration={(id) => ctrl?.run("selectGeneration", { id })}
      onAddGeneration={() => ctrl?.run("addGeneration")}
      onRemoveGeneration={(id) => ctrl?.run("removeGeneration", { id })}
      onGenerationValuesChange={(id, values) => ctrl?.run("updateGenerationValues", { id, values })}
      onRenameGeneration={(id, name) => ctrl?.run("renameGeneration", { id, name })}
      className="h-full"
    />
  );
}

export function registerProceduralPlaySurfaceHosts(): void {
  if (proceduralPlayChromeRegistered) return;
  proceduralPlayChromeRegistered = true;
  registerUiFlowSurfaceHost(PROCEDURAL_PLAY_SURFACE_ID, ProceduralPlayPaneSurfaceHost);
  registerUiPuzzle3dSurfaceHost(PROCEDURAL_PLAY_SURFACE_ID_PREVIEW, ProceduralPreviewSurfaceHost);
  registerUiFormsSurfaceHost(PROCEDURAL_PLAY_SURFACE_ID_GENERATE, Procedural3dGenerateSurfaceHost);
  registerProceduralPlayDeclarativeBodies();
}

function ProceduralPlayChrome({ runtime }: { readonly runtime: Platform }): ReactElement {
  return <ProceduralPlayInner runtime={runtime} />;
}

export function mountProceduralPlayChrome(playground: Playground, rootId = "root"): void {
  mountPlaygroundApp(<ProceduralPlayChrome runtime={playground.runtime} />, rootId);
}

const proceduralPlayChromeBoot: PlaygroundChromeBoot = {
  registerHosts: registerProceduralPlaySurfaceHosts,
  mount: mountProceduralPlayChrome,
};

export function bootProceduralPlay(playground: Playground, rootId = "root"): void {
  bootPlayground(playground, proceduralPlayChromeBoot, rootId);
}
//#endregion 🔖ProceduralPlayHost

//#region 🔖Procedural2dPlayHost
import { Procedural2dFlowEditor, Procedural2dPreview, useProcedural2dDrawingBridge, canvasDrawingPngExportPort } from "@semio-tech/procedural-2d-react";
import { flowWidgetPaletteTreeDragController as procedural2dWidgetPaletteTreeDragController } from "@semio-tech/flow-react";
import {
    PROCEDURAL_2D_PLAY_APP_ID,
    PROCEDURAL_2D_PLAY_CATALOGUE_TAB_ID,
    PROCEDURAL_2D_PLAY_HIERARCHY_TAB_ID,
    PROCEDURAL_2D_PLAY_INSPECTION_TAB_ID,
    PROCEDURAL_2D_PLAY_SURFACE_ID,
    PROCEDURAL_2D_PLAY_SURFACE_ID_GENERATE,
    PROCEDURAL_2D_PLAY_SURFACE_ID_PREVIEW,
    PROCEDURAL_2D_PLAY_WINDOW_KIND_ID,
    Procedural2dPlayController,
    buildProcedural2dPlayCanvasContextMenu,
    buildProcedural2dPlayCatalogueTree,
    buildProcedural2dPlayHierarchyTree,
    buildProcedural2dPlayInspectorTree,
    registerProcedural2dPlayDeclarativeBodies,
    type Procedural2dPlayHostBridge,
} from "@semio-tech/procedural-2d-core";
import { PROCEDURAL_2D_PLAY_EMPTY_FIXTURE_JSON } from "@semio-tech/procedural-2d-core";

let procedural2dPlayChromeRegistered = false;
const procedural2dPlayControllerRef: { current: Procedural2dPlayController | null } = { current: null };

function useProcedural2dPlaySnapshotRevision(runtime: Platform, selector: (ctrl: Procedural2dPlayController) => number): number {
  return reactHostPort.useSyncExternalStore(
    (listener) => {
      const ctrl = runtime.getActiveApp()?.controller as Procedural2dPlayController | undefined;
      procedural2dPlayControllerRef.current = ctrl ?? null;
      const unsubscribeChrome = runtime.subscribeChrome(listener);
      const unsubscribeSnapshot = ctrl?.subscribeSnapshot(listener);
      return () => {
        unsubscribeChrome();
        unsubscribeSnapshot?.();
      };
    },
    () => {
      const ctrl = runtime.getActiveApp()?.controller as Procedural2dPlayController | undefined;
      procedural2dPlayControllerRef.current = ctrl ?? null;
      return ctrl ? selector(ctrl) : 0;
    },
    () => 0,
  );
}

function useProcedural2dPlayCatalogueRevision(runtime: Platform): number {
  return useProcedural2dPlaySnapshotRevision(runtime, (c) => c.getCatalogueRevision());
}

function useProcedural2dPlayExtensionRevision(runtime: Platform): number {
  return useProcedural2dPlaySnapshotRevision(runtime, (c) => c.getExtensionRevision());
}

function useProcedural2dPlayInteractionRevision(runtime: Platform): number {
  return useProcedural2dPlaySnapshotRevision(runtime, (c) => c.getInteractionRevision());
}

function useProcedural2dPlayController(runtimeOverride?: Platform): Procedural2dPlayController | undefined {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const runtime = runtimeOverride ?? appCtx?.runtime;
  reactHostPort.useSyncExternalStore(
    (listener) => (runtime ? runtime.subscribeChrome(listener) : () => {}),
    () => runtime?.chromeGeneration ?? 0,
    () => 0,
  );
  const ctrl = runtime?.getActiveApp()?.controller as Procedural2dPlayController | undefined;
  procedural2dPlayControllerRef.current = ctrl ?? null;
  return ctrl;
}

async function downloadProcedural2dExport(name: string, data: BlobPart, mime: string): Promise<void> {
  const pickerWindow = window as Window & { showSaveFilePicker?: (options?: { suggestedName?: string; types?: { description: string; accept: Record<string, string[]> }[] }) => Promise<FileSystemFileHandle> };
  const ext = name.includes(".") ? name.slice(name.lastIndexOf(".")) : "";
  if (pickerWindow.showSaveFilePicker) {
    const handle = await pickerWindow.showSaveFilePicker({
      suggestedName: name,
      types: [{ description: "Export", accept: { [mime]: [ext] } }],
    });
    const writable = await handle.createWritable();
    await writable.write(data);
    await writable.close();
    return;
  }
  const href = URL.createObjectURL(new Blob([data], { type: mime }));
  const link = document.createElement("a");
  link.href = href;
  link.download = name;
  link.click();
  URL.revokeObjectURL(href);
}

function Procedural2dPlayToolbarHostBridge({ runtime, ctrl }: { readonly runtime: Platform; readonly ctrl: Procedural2dPlayController | undefined }): ReactElement {
  const interactionRevision = useProcedural2dPlayInteractionRevision(runtime);
  const loadInputRef = reactHostPort.useRef<HTMLInputElement>(null);
  const drawingBridge = useProcedural2dDrawingBridge();
  const downloadFixture = reactHostPort.useCallback(async () => {
    const json = ctrl?.getFixtureJson() ?? PROCEDURAL_2D_PLAY_EMPTY_FIXTURE_JSON;
    try {
      await downloadProcedural2dExport("procedural2d.fixture.json", `${json}\n`, "application/json");
      console.log("[DEBUG] procedural 2d play downloaded fixture");
    } catch (error) {
      console.log(`[DEBUG] procedural 2d play download failed: ${String(error)}`);
    }
  }, [ctrl]);
  const handleLoadFile = reactHostPort.useCallback(
    (event: reactHostPort.ChangeEvent<HTMLInputElement>) => {
      const file = event.target.files?.[0];
      event.target.value = "";
      if (!file || !ctrl) return;
      void file.text().then((text) => {
        if (!text.includes("flow.fixture")) {
          console.log("[DEBUG] procedural 2d play load rejected: not a flow fixture");
          return;
        }
        ctrl.run("setFixtureJson", { json: text, resetInteraction: true });
        console.log("[DEBUG] procedural 2d play loaded fixture from file");
      });
    },
    [ctrl],
  );
  reactHostPort.useEffect(() => {
    if (!ctrl) return;
    const bridge: Procedural2dPlayHostBridge = {
      getToolbarState: () => ({
        selectionMethod: ctrl.getSelectionMethod(),
        selectionMode: ctrl.getSelectionMode(),
        showMode: ctrl.getShowMode(),
        selectionCount: ctrl.getSelectedNodeIds().length,
        hasStoredFixture: ctrl.hasStoredFixture(),
      }),
      runHostCommand: (command, args) => {
        if (command === "saveDownload") {
          void downloadFixture();
          return;
        }
        if (command === "loadRequest") {
          loadInputRef.current?.click();
          return;
        }
        if (command === "exportSvg" || command === "exportPdf" || command === "exportPng") {
          const handle = (args as { handle?: string } | undefined)?.handle ?? ctrl.getPrimaryDrawingHandle();
          const primaryItem = ctrl.getPreviewItems().find((item) => item.kind === "drawing" && (!handle || item.handle === handle));
          if (!handle && !primaryItem?.scene) {
            console.log(`[DEBUG] procedural 2d play ${command} skipped: no drawing handle or scene`);
            return;
          }
          void (async () => {
            try {
              if (command === "exportPng" && primaryItem?.scene) {
                const png = canvasDrawingPngExportPort.exportPng(primaryItem.scene);
                await downloadProcedural2dExport("procedural2d.export.png", png, "image/png");
              } else if (!drawingBridge || !handle) {
                console.log(`[DEBUG] procedural 2d play ${command} skipped: no drawing bridge`);
                return;
              } else if (command === "exportSvg") {
                const svg = drawingBridge.exportSvg(handle);
                await downloadProcedural2dExport("procedural2d.export.svg", svg, "image/svg+xml");
              } else if (command === "exportPdf") {
                const pdfBase64 = drawingBridge.exportPdf(handle);
                const binary = atob(pdfBase64);
                const bytes = Uint8Array.from(binary, (char) => char.charCodeAt(0));
                await downloadProcedural2dExport("procedural2d.export.pdf", bytes, "application/pdf");
              } else {
                const png = drawingBridge.exportPng(handle);
                await downloadProcedural2dExport("procedural2d.export.png", png, "image/png");
              }
              console.log(`[DEBUG] procedural 2d play ${command} completed`);
            } catch (error) {
              console.log(`[DEBUG] procedural 2d play ${command} failed: ${String(error)}`);
            }
          })();
        }
      },
    };
    ctrl.setHostBridge(bridge);
    return () => ctrl.setHostBridge(null);
  }, [ctrl, downloadFixture, drawingBridge, interactionRevision]);
  return <input ref={loadInputRef} type="file" accept=".json,application/json" className="hidden" onChange={handleLoadFile} />;
}

function Procedural2dPlayPaneSurfaceHost({ node }: { readonly node: UiFlowHostSurfaceNode }): ReactElement {
  const { runtime } = useApp();
  const ctrl = useProcedural2dPlayController();
  const extensionRevision = useProcedural2dPlayExtensionRevision(runtime);
  const interactionRevision = useProcedural2dPlayInteractionRevision(runtime);
  void interactionRevision;
  const onPreviewText = reactHostPort.useCallback(
    (text: string) => {
      console.log(`[DEBUG] procedural 2d play preview: ${text}`);
      ctrl?.run("setPreviewText", { text });
    },
    [ctrl],
  );
  const onEvalOutputs = reactHostPort.useCallback(
    (outputsJson: string, previewMeshes?: Readonly<Record<string, unknown>>) => {
      console.log(`[DEBUG] procedural 2d play eval outputs: ${outputsJson.slice(0, 120)}`);
      ctrl?.run("setEvalOutputs", { outputsJson, previewMeshes });
    },
    [ctrl],
  );
  const onCatalogueReady = reactHostPort.useCallback(
    (sections: readonly import("@semio-tech/flow-react").CatalogueSection[]) => {
      ctrl?.run("setCatalogueSections", { sections: [...sections] });
    },
    [ctrl],
  );
  const onFixtureChange = reactHostPort.useCallback(
    (json: string) => {
      ctrl?.run("setFixtureJson", { json });
    },
    [ctrl],
  );
  const onSelectionChange = reactHostPort.useCallback(
    (ids: readonly string[]) => {
      ctrl?.run("setSelection", { ids: [...ids], mode: "default", fromFlow: true });
    },
    [ctrl],
  );
  const onPreselectChange = reactHostPort.useCallback(
    (snapshot: { readonly ids: readonly string[]; readonly removedIds: readonly string[] }) => {
      ctrl?.run("setPreselect", { ids: [...snapshot.ids], removedIds: [...snapshot.removedIds] });
    },
    [ctrl],
  );
  const onHoverChange = reactHostPort.useCallback(
    (id: string | null) => {
      ctrl?.run("setHover", { id, channel: null });
    },
    [ctrl],
  );
  const onChannelHoverChange = reactHostPort.useCallback(
    (channel: import("@semio-tech/procedural-2d-react").ProceduralChannelRef | null) => {
      ctrl?.run("setHover", { id: channel?.widgetId ?? null, channel });
    },
    [ctrl],
  );
  const onSelectedChannelsChange = reactHostPort.useCallback(
    (channels: readonly import("@semio-tech/procedural-2d-react").ProceduralChannelRef[]) => {
      ctrl?.run("setSelectedChannels", { channels: [...channels] });
    },
    [ctrl],
  );
  const onPreviewOffChange = reactHostPort.useCallback(
    (ids: readonly string[]) => {
      ctrl?.run("setPreviewOff", { ids: [...ids], fromFlow: true });
    },
    [ctrl],
  );
  const onCanvasCommand = reactHostPort.useCallback(
    (command: string, args?: Record<string, unknown>) => {
      ctrl?.run(command, args);
    },
    [ctrl],
  );
  const onOutputExport = reactHostPort.useCallback(
    (widgetId: string, format: string, resolvedValueJson: string) => {
      void downloadFlowOutputExport(format, resolvedValueJson, widgetId).catch((error) => {
        console.log(`[DEBUG] procedural 2d play export failed: ${String(error)}`);
      });
    },
    [],
  );
  return (
    <Procedural2dFlowEditor
      fixtureJson={ctrl?.getFixtureJson() ?? PROCEDURAL_2D_PLAY_EMPTY_FIXTURE_JSON}
      reorganize={ctrl?.getReorganize()}
      commandRequest={ctrl?.getCommandRequest()}
      extensionRevision={extensionRevision}
      onPreviewText={onPreviewText}
      onEvalOutputs={onEvalOutputs}
      onOutputExport={onOutputExport}
      onCatalogueReady={onCatalogueReady}
      onFixtureChange={onFixtureChange}
      onSelectionChange={onSelectionChange}
      onPreselectChange={onPreselectChange}
      onHoverChange={onHoverChange}
      onChannelHoverChange={onChannelHoverChange}
      onSelectedChannelsChange={onSelectedChannelsChange}
      onPreviewOffChange={onPreviewOffChange}
      selectedNodeIds={ctrl?.getSelectedNodeIds()}
      selectedChannels={ctrl?.getSelectedChannels()}
      preselectNodeIds={ctrl?.getPreselectNodeIds()}
      preselectRemovedNodeIds={ctrl?.getPreselectRemovedNodeIds()}
      hoveredNodeId={ctrl?.getHoveredNodeId()}
      hoveredChannel={ctrl?.getHoveredChannel()}
      previewOffNodeIds={ctrl?.getPreviewOffNodeIds()}
      selectionMode={ctrl?.getSelectionMode()}
      selectionMethod={ctrl?.getSelectionMethod()}
      contextMenu={(ctx) => buildProcedural2dPlayCanvasContextMenu(ctx, onCanvasCommand)}
      className="h-full w-full"
    />
  );
}

function Procedural2dPreviewSurfaceHost({ node: _node }: { readonly node: UiPuzzle2dHostSurfaceNode }): ReactElement {
  const { runtime } = useApp();
  const ctrl = useProcedural2dPlayController();
  const drawingBridge = useProcedural2dDrawingBridge();
  const interactionRevision = useProcedural2dPlayInteractionRevision(runtime);
  void interactionRevision;
  const onHover = reactHostPort.useCallback(
    (channel: import("@semio-tech/procedural-2d-react").ProceduralChannelRef | null) => {
      ctrl?.run("setHover", { id: channel?.widgetId ?? null, channel });
    },
    [ctrl],
  );
  const onSelect = reactHostPort.useCallback(
    (channel: import("@semio-tech/procedural-2d-react").ProceduralChannelRef) => {
      ctrl?.run("setSelectChannels", { channels: [channel] });
    },
    [ctrl],
  );
  const onSelectionChange = reactHostPort.useCallback(
    (ids: readonly string[], mode: import("@semio-tech/procedural-2d-react").ProceduralSelectionMode) => {
      ctrl?.run("setSelection", { ids: [...ids], mode });
    },
    [ctrl],
  );
  return (
    <div className="absolute inset-0 min-h-0 min-w-0">
      <Procedural2dPreview
        items={ctrl?.getPreviewItems() ?? []}
        selectedNodeIds={ctrl?.getSelectedNodeIds()}
        preselectNodeIds={ctrl?.getPreselectNodeIds()}
        preselectRemovedNodeIds={ctrl?.getPreselectRemovedNodeIds()}
        hoveredNodeId={ctrl?.getHoveredNodeId()}
        hoveredChannel={ctrl?.getHoveredChannel()}
        hoveredGeometryTargets={ctrl?.getHoveredGeometryTargets()}
        selectedChannels={ctrl?.getSelectedChannels()}
        selectedGeometryTargets={ctrl?.getSelectedGeometryTargets()}
        previewOffNodeIds={ctrl?.getPreviewOffNodeIds()}
        showMode={ctrl?.getShowMode() ?? "everything"}
        selectionMode={ctrl?.getSelectionMode()}
        selectionMethod={ctrl?.getSelectionMethod()}
        onHover={onHover}
        onSelect={onSelect}
        onSelectionChange={onSelectionChange}
        kernel={drawingBridge ?? undefined}
        className="h-full w-full"
      />
    </div>
  );
}

class Procedural2dPlayHierarchyPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: PROCEDURAL_2D_PLAY_HIERARCHY_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = procedural2dPlayControllerRef.current;
        const bus = new CommandBus();
        const treeNode = buildProcedural2dPlayHierarchyTree(ctrl?.getFixtureJson() ?? PROCEDURAL_2D_PLAY_EMPTY_FIXTURE_JSON, ctrl?.getSelectedNodeIds() ?? []);
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

class Procedural2dPlayCataloguePanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: PROCEDURAL_2D_PLAY_CATALOGUE_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
      order: 1,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = procedural2dPlayControllerRef.current;
        const bus = new CommandBus();
        const treeNode = buildProcedural2dPlayCatalogueTree(ctrl?.getCatalogueSections() ?? [], ctrl?.getExtensionEntries() ?? []);
        const config = uiTreeNodeToTreePanelConfig(treeNode, bus);
        return {
          ...config,
          dragAndDropController: procedural2dWidgetPaletteTreeDragController(collectUiTreeItemDragData(treeNode.sections)),
        };
      }),
    };
  }
}

class Procedural2dPlayInspectionPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: PROCEDURAL_2D_PLAY_INSPECTION_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, "details"),
      name: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = procedural2dPlayControllerRef.current;
        const bus = new CommandBus();
        const treeNode = buildProcedural2dPlayInspectorTree(ctrl?.getFixtureJson() ?? PROCEDURAL_2D_PLAY_EMPTY_FIXTURE_JSON, ctrl?.getSelectedNodeIds() ?? []);
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

function Procedural2dPlayInner({ runtime }: { readonly runtime: Platform }): ReactElement {
  const ctrl = useProcedural2dPlayController(runtime);
  const catalogueRevision = useProcedural2dPlayCatalogueRevision(runtime);
  const extensionRevision = useProcedural2dPlayExtensionRevision(runtime);
  const interactionRevision = useProcedural2dPlayInteractionRevision(runtime);
  const procedural2dPlayHierarchyPanel = reactHostPort.useMemo(() => new Procedural2dPlayHierarchyPanelDefinition(), []);
  const procedural2dPlayCataloguePanel = reactHostPort.useMemo(() => new Procedural2dPlayCataloguePanelDefinition(), []);
  const procedural2dPlayInspectionPanel = reactHostPort.useMemo(() => new Procedural2dPlayInspectionPanelDefinition(), []);
  const augmentPanelTabs = reactHostPort.useMemo(
    () => ({
      workbench: [procedural2dPlayHierarchyPanel, procedural2dPlayCataloguePanel],
      details: [procedural2dPlayInspectionPanel],
    }),
    [catalogueRevision, extensionRevision, interactionRevision, procedural2dPlayCataloguePanel, procedural2dPlayHierarchyPanel, procedural2dPlayInspectionPanel],
  );
  return (
    <>
      <Procedural2dPlayToolbarHostBridge runtime={runtime} ctrl={ctrl} />
      <PlaygroundView runtime={runtime} defaultAppId={PROCEDURAL_2D_PLAY_APP_ID} augmentPanelTabs={augmentPanelTabs} />
    </>
  );
}

function Procedural2dGenerateSurfaceHost({ node }: { readonly node: UiFormsHostSurfaceNode }): ReactElement {
  const ctrl = useProcedural2dPlayController();
  const spec = reactHostPort.useMemo(() => {
    try {
      return parseFormSpec(JSON.parse(ctrl?.getGenerateFormSpecJson() ?? "{}"));
    } catch {
      return parseFormSpec({ schema: "forms.form", id: "empty", version: "1", steps: [{ id: "s", title: "Inputs", questions: [] }] });
    }
  }, [ctrl]);
  return (
    <FlowGenerateSurface
      formSpec={spec}
      generations={[...(ctrl?.getGenerations() ?? [])]}
      selectedGenerationId={ctrl?.getSelectedGenerationId() ?? null}
      previewText={ctrl?.getGeneratePreviewText() ?? "—"}
      onSelectGeneration={(id) => ctrl?.run("selectGeneration", { id })}
      onAddGeneration={() => ctrl?.run("addGeneration")}
      onRemoveGeneration={(id) => ctrl?.run("removeGeneration", { id })}
      onGenerationValuesChange={(id, values) => ctrl?.run("updateGenerationValues", { id, values })}
      onRenameGeneration={(id, name) => ctrl?.run("renameGeneration", { id, name })}
      className="h-full"
    />
  );
}

export function registerProcedural2dPlaySurfaceHosts(): void {
  if (procedural2dPlayChromeRegistered) return;
  procedural2dPlayChromeRegistered = true;
  registerUiFlowSurfaceHost(PROCEDURAL_2D_PLAY_SURFACE_ID, Procedural2dPlayPaneSurfaceHost);
  registerUiPuzzle2dSurfaceHost(PROCEDURAL_2D_PLAY_SURFACE_ID_PREVIEW, Procedural2dPreviewSurfaceHost);
  registerUiFormsSurfaceHost(PROCEDURAL_2D_PLAY_SURFACE_ID_GENERATE, Procedural2dGenerateSurfaceHost);
  registerProcedural2dPlayDeclarativeBodies();
}

function Procedural2dPlayChrome({ runtime }: { readonly runtime: Platform }): ReactElement {
  return <Procedural2dPlayInner runtime={runtime} />;
}

export function mountProcedural2dPlayChrome(playground: Playground, rootId = "root"): void {
  mountPlaygroundApp(<Procedural2dPlayChrome runtime={playground.runtime} />, rootId);
}

const procedural2dPlayChromeBoot: PlaygroundChromeBoot = {
  registerHosts: registerProcedural2dPlaySurfaceHosts,
  mount: mountProcedural2dPlayChrome,
};

export function bootProcedural2dPlay(playground: Playground, rootId = "root"): void {
  bootPlayground(playground, procedural2dPlayChromeBoot, rootId);
}
//#endregion 🔖Procedural2dPlayHost

//#region 🔖ShootingPlayHost
import type { UiShootingHostSurfaceNode } from "@semio-tech/framework-platform-core";
import {
    SHOOTING_PLAY_APP_ID,
    SHOOTING_PLAY_CATALOGUE_TAB_ID,
    SHOOTING_PLAY_HIERARCHY_TAB_ID,
    SHOOTING_PLAY_INSPECTION_TAB_ID,
    SHOOTING_PLAY_SURFACE_ID_ICON,
    SHOOTING_PLAY_SURFACE_ID_MODEL,
    ShootingPlayController,
    buildShootingPlayCatalogueTree,
    buildShootingPlayHierarchyTree,
    buildShootingPlayInspectorTree,
    registerShootingPlayDeclarativeBodies,
    type ShootingPlayHostBridge
} from "@semio-tech/shooting-core";
import { ShootingIconCanvas, ShootingModelCanvas, renderShootingShot, resolveActiveShot } from "@semio-tech/shooting-react";

type ShootingSurfaceHost = React.ComponentType<{ readonly node: UiShootingHostSurfaceNode }>;
const shootingSurfaceHosts = new Map<string, ShootingSurfaceHost>();
const shootingPlayControllerRef: { current: ShootingPlayController | null } = { current: null };
let shootingPlayChromeRegistered = false;

export function registerUiShootingSurfaceHost(surfaceId: string, Component: ShootingSurfaceHost): void {
	shootingSurfaceHosts.set(surfaceId, Component);
	registerSurfaceBinding(surfaceId, Component as PlaygroundSurfaceBindingHost);
}

function useShootingPlayController(runtimeOverride?: Platform): ShootingPlayController | undefined {
	const appCtx = reactHostPort.useContext(PlaygroundContext);
	const runtime = runtimeOverride ?? appCtx?.runtime;
	reactHostPort.useSyncExternalStore(
		(listener) => (runtime ? runtime.subscribe(listener) : () => {}),
		() => runtime?.generation ?? 0,
		() => 0,
	);
	const ctrl = runtime?.getActiveApp()?.controller as ShootingPlayController | undefined;
	shootingPlayControllerRef.current = ctrl ?? null;
	return ctrl;
}

function ShootingPlayFileBridge(): ReactElement | null {
	const ctrl = useShootingPlayController();
	const loadInputRef = reactHostPort.useRef<HTMLInputElement | null>(null);
	const assetInputRef = reactHostPort.useRef<HTMLInputElement | null>(null);
	const downloadFixture = reactHostPort.useCallback(async () => {
		if (!ctrl) return;
		const text = ctrl.getFixtureJson();
		const blob = new Blob([`${text}\n`], { type: "application/json" });
		const url = URL.createObjectURL(blob);
		const anchor = document.createElement("a");
		anchor.href = url;
		anchor.download = "shooting.fixture.json";
		anchor.click();
		URL.revokeObjectURL(url);
	}, [ctrl]);
	const downloadShot = reactHostPort.useCallback(
		async (shotId?: string) => {
			if (!ctrl) return;
			const fixture = ctrl.getFixture();
			const shots = shotId ? fixture.shots.filter((shot) => shot.id === shotId) : fixture.shots;
			for (const shot of shots) {
				const result = await renderShootingShot(fixture, shot);
				const extension = shot.format === "svg" ? "svg" : "png";
				const anchor = document.createElement("a");
				anchor.href = result.dataUrl;
				anchor.download = `${shot.id}.${extension}`;
				anchor.click();
				console.log(`[DEBUG] shooting exported shot ${shot.id}.${extension}`);
			}
		},
		[ctrl],
	);
	const handleLoadFile = reactHostPort.useCallback(
		(event: React.ChangeEvent<HTMLInputElement>) => {
			const file = event.target.files?.[0];
			event.target.value = "";
			if (!file || !ctrl) return;
			void file.text().then((text) => {
				ctrl.run("setFixtureJson", { json: text });
				console.log("[DEBUG] shooting play loaded fixture from file");
			});
		},
		[ctrl],
	);
	const handleImportAsset = reactHostPort.useCallback(
		(event: React.ChangeEvent<HTMLInputElement>) => {
			const file = event.target.files?.[0];
			event.target.value = "";
			if (!file || !ctrl) return;
			const objectUrl = URL.createObjectURL(file);
			const id = file.name.replace(/\.[^.]+$/, "").replace(/[^\w-]+/g, "-") || `asset_${Date.now()}`;
			ctrl.run("importAsset", {
				asset: { id, name: file.name, url: objectUrl, format: "glb" },
			});
		},
		[ctrl],
	);
	reactHostPort.useEffect(() => {
		if (!ctrl) return;
		const bridge: ShootingPlayHostBridge = {
			getToolbarState: () => ({
				hasStoredFixture: ctrl.hasStoredFixture(),
				activeShotId: ctrl.getFixture().activeShotId ?? resolveActiveShot(ctrl.getFixture())?.id ?? null,
			}),
			runHostCommand: (command) => {
				if (command === "saveDownload") {
					void downloadFixture();
					return;
				}
				if (command === "loadRequest") {
					loadInputRef.current?.click();
					return;
				}
				if (command === "importAssetRequest") {
					assetInputRef.current?.click();
					return;
				}
				if (command === "exportActiveShot") {
					const active = resolveActiveShot(ctrl.getFixture());
					if (active) void downloadShot(active.id);
					return;
				}
				if (command === "exportAllShots") {
					void downloadShot();
				}
			},
		};
		ctrl.setHostBridge(bridge);
		return () => ctrl.setHostBridge(null);
	}, [ctrl, downloadFixture, downloadShot]);
	return (
		<>
			<input ref={loadInputRef} type="file" accept=".json,application/json" className="hidden" onChange={handleLoadFile} />
			<input ref={assetInputRef} type="file" accept=".glb,model/gltf-binary" className="hidden" onChange={handleImportAsset} />
		</>
	);
}

function ShootingModelSurfaceHost({ node }: { readonly node: UiShootingHostSurfaceNode }): ReactElement {
	const ctrl = useShootingPlayController();
	const fixture = ctrl?.getFixture();
	if (!fixture || node.view !== "model") {
		return <div className="absolute inset-0 min-h-0 min-w-0" />;
	}
	return (
		<div className="absolute inset-0 min-h-0 min-w-0">
			<ShootingModelCanvas
				fixture={fixture}
				className="h-full w-full"
				centerModel={ctrl?.getCenterModel() ?? true}
				fitRevision={ctrl?.getFitRevision() ?? 0}
				onCamera={(camera) => ctrl?.run("setCamera", { camera })}
			/>
		</div>
	);
}

function ShootingIconSurfaceHost({ node }: { readonly node: UiShootingHostSurfaceNode }): ReactElement {
	const { runtime } = useApp();
	const ctrl = useShootingPlayController();
	const revision = ctrl?.getRenderRevision() ?? 0;
	void runtime.generation;
	const fixture = ctrl?.getFixture();
	if (!fixture || node.view !== "icon") {
		return <div className="absolute inset-0 min-h-0 min-w-0" />;
	}
	return (
		<div className="absolute inset-0 min-h-0 min-w-0">
			<ShootingIconCanvas fixture={fixture} className="h-full w-full" renderRevision={revision} />
		</div>
	);
}

function useShootingPlayInteractionRevision(runtime: Platform): number {
	return reactHostPort.useSyncExternalStore(
		(listener) => {
			const ctrl = runtime.getActiveApp()?.controller as ShootingPlayController | undefined;
			shootingPlayControllerRef.current = ctrl ?? null;
			const unsubscribeRuntime = runtime.subscribe(listener);
			const unsubscribeSnapshot = ctrl?.subscribeSnapshot(listener);
			return () => {
				unsubscribeRuntime();
				unsubscribeSnapshot?.();
			};
		},
		() => (runtime.getActiveApp()?.controller as ShootingPlayController | undefined)?.getInteractionRevision() ?? 0,
		() => 0,
	);
}

class ShootingPlayHierarchyPanelDefinition extends PureSidePanelTabDefinition {
	buildTab(): SidePanelTabConfig {
		return {
			id: SHOOTING_PLAY_HIERARCHY_TAB_ID,
			icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, "workbench"),
			name: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
			order: 0,
			tree: new CallbackTreePanelDefinition(() => {
				const ctrl = shootingPlayControllerRef.current;
				const bus = new CommandBus();
				const fixture = ctrl?.getFixture();
				if (!fixture) {
					return [{ id: "shooting-play-hierarchy.loading", label: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, items: [{ id: "loading", label: "…" }] }];
				}
				const treeNode = buildShootingPlayHierarchyTree(fixture, ctrl?.getSelectedShotIds() ?? [], ctrl?.getSelectedAssetIds() ?? []);
				return uiTreeNodeToTreePanelConfig(treeNode, bus);
			}),
		};
	}
}

class ShootingPlayCataloguePanelDefinition extends PureSidePanelTabDefinition {
	buildTab(): SidePanelTabConfig {
		return {
			id: SHOOTING_PLAY_CATALOGUE_TAB_ID,
			icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, "workbench"),
			name: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
			order: 1,
			tree: new CallbackTreePanelDefinition(() => {
				const bus = new CommandBus();
				return uiTreeNodeToTreePanelConfig(buildShootingPlayCatalogueTree(), bus);
			}),
		};
	}
}

class ShootingPlayInspectionPanelDefinition extends PureSidePanelTabDefinition {
	buildTab(): SidePanelTabConfig {
		return {
			id: SHOOTING_PLAY_INSPECTION_TAB_ID,
			icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, "details"),
			name: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
			order: 0,
			tree: new CallbackTreePanelDefinition(() => {
				const ctrl = shootingPlayControllerRef.current;
				const bus = new CommandBus();
				const fixture = ctrl?.getFixture();
				if (!fixture) {
					return [{ id: "shooting-play-inspector.loading", label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, items: [{ id: "loading", label: "…" }] }];
				}
				const treeNode = buildShootingPlayInspectorTree(fixture, ctrl?.getSelectedShotIds() ?? [], ctrl?.getSelectedAssetIds() ?? []);
				return uiTreeNodeToTreePanelConfig(treeNode, bus);
			}),
		};
	}
}

function ShootingPlayInner({ playground }: { readonly playground: Playground }): ReactElement {
	const interactionRevision = useShootingPlayInteractionRevision(playground.runtime);
	const ctrl = useShootingPlayController(playground.runtime);
	shootingPlayControllerRef.current = ctrl ?? null;
	const shootingPlayHierarchyPanel = reactHostPort.useMemo(() => new ShootingPlayHierarchyPanelDefinition(), []);
	const shootingPlayCataloguePanel = reactHostPort.useMemo(() => new ShootingPlayCataloguePanelDefinition(), []);
	const shootingPlayInspectionPanel = reactHostPort.useMemo(() => new ShootingPlayInspectionPanelDefinition(), []);
	const augmentPanelTabs = reactHostPort.useMemo(
		() => ({
			workbench: [shootingPlayHierarchyPanel, shootingPlayCataloguePanel],
			details: [shootingPlayInspectionPanel],
		}),
		[interactionRevision, shootingPlayCataloguePanel, shootingPlayHierarchyPanel, shootingPlayInspectionPanel],
	);
	return (
		<>
			<ShootingPlayFileBridge />
			<PlaygroundView runtime={playground.runtime} defaultAppId={SHOOTING_PLAY_APP_ID} augmentPanelTabs={augmentPanelTabs} playgroundKeybindings={playground.keybindings} />
		</>
	);
}

export function registerShootingPlaySurfaceHosts(): void {
	if (shootingPlayChromeRegistered) return;
	shootingPlayChromeRegistered = true;
	registerUiShootingSurfaceHost(SHOOTING_PLAY_SURFACE_ID_MODEL, ShootingModelSurfaceHost);
	registerUiShootingSurfaceHost(SHOOTING_PLAY_SURFACE_ID_ICON, ShootingIconSurfaceHost);
	registerShootingPlayDeclarativeBodies();
}

function ShootingPlayChrome({ playground }: { readonly playground: Playground }): ReactElement {
	return <ShootingPlayInner playground={playground} />;
}

export function mountShootingPlayChrome(playground: Playground, rootId = "root"): void {
	mountPlaygroundApp(<ShootingPlayChrome playground={playground} />, rootId);
}

const shootingPlayChromeBoot: PlaygroundChromeBoot = {
	registerHosts: registerShootingPlaySurfaceHosts,
	mount: mountShootingPlayChrome,
};

export function bootShootingPlay(playground: Playground, rootId = "root"): void {
  bootPlayground(playground, shootingPlayChromeBoot, rootId);
}
//#endregion 🔖ShootingPlayHost

//#region 🔖FormsPlayHost
import {
  FORMS_PLAY_APP_ID,
  FORMS_PLAY_CATALOGUE_TAB_ID,
  FORMS_PLAY_CONTROLLER_ID,
  FORMS_PLAY_HIERARCHY_TAB_ID,
  FORMS_PLAY_INSPECTION_TAB_ID,
  FORMS_PLAY_SURFACE_ID_EDIT,
  FORMS_PLAY_SURFACE_ID_TRY,
  FormsPlayController,
  buildFormsPlayCatalogueTree,
  buildFormsPlayHierarchyTree,
  buildFormsPlayInspectorTree,
  commitFormsPlayQuestionDropAtClient,
  createFormsPlayHierarchyTreeDragController,
  registerFormsPlayDeclarativeBodies,
} from "@semio-tech/forms-core";
import {
  FormEditSurface,
  FormRenderer,
  FormsQuestionPaletteDragBridge,
  FormsQuestionPaletteDragGhost,
  FORMS_QUESTION_DRAG_MIME,
  defaultFormSpec,
  formsQuestionDragAcceptsTransfer,
  formsQuestionPaletteTreeDragController,
} from "@semio-tech/forms-react";

let formsPlayChromeRegistered = false;
const formsPlayControllerRef: { current: FormsPlayController | null } = { current: null };

function useFormsPlayController(runtimeOverride?: Platform): FormsPlayController | undefined {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const runtime = runtimeOverride ?? appCtx?.runtime;
  reactHostPort.useSyncExternalStore(
    (listener) => (runtime ? runtime.subscribe(listener) : () => {}),
    () => runtime?.generation ?? 0,
    () => 0,
  );
  const ctrl = runtime?.getActiveApp()?.controller as FormsPlayController | undefined;
  formsPlayControllerRef.current = ctrl ?? null;
  return ctrl;
}

function useFormsPlayInteractionRevision(runtime: Platform): number {
  return reactHostPort.useSyncExternalStore(
    (listener) => {
      const ctrl = runtime.getActiveApp()?.controller as FormsPlayController | undefined;
      formsPlayControllerRef.current = ctrl ?? null;
      const unsubscribeRuntime = runtime.subscribe(listener);
      const unsubscribeSnapshot = ctrl?.subscribeSnapshot(listener);
      return () => {
        unsubscribeRuntime();
        unsubscribeSnapshot?.();
      };
    },
    () => (runtime.getActiveApp()?.controller as FormsPlayController | undefined)?.getInteractionRevision() ?? 0,
    () => 0,
  );
}

function useFormsPlayExtensionRevision(runtime: Platform): number {
  return reactHostPort.useSyncExternalStore(
    (listener) => {
      const ctrl = runtime.getActiveApp()?.controller as FormsPlayController | undefined;
      formsPlayControllerRef.current = ctrl ?? null;
      const unsubscribeRuntime = runtime.subscribe(listener);
      const unsubscribeSnapshot = ctrl?.subscribeSnapshot(listener);
      return () => {
        unsubscribeRuntime();
        unsubscribeSnapshot?.();
      };
    },
    () => (runtime.getActiveApp()?.controller as FormsPlayController | undefined)?.getExtensionRevision() ?? 0,
    () => 0,
  );
}

function FormsEditSurfaceHost({ node: _node }: { readonly node: UiFormsHostSurfaceNode }): ReactElement {
  const ctrl = useFormsPlayController();
  const spec = ctrl?.getSpec() ?? defaultFormSpec("empty");
  const selectedIds = ctrl?.getSelectedIds() ?? [];
  return (
    <FormEditSurface
      spec={spec}
      className="h-full"
      selectedIds={selectedIds}
      onSelectionChange={(ids) => ctrl?.run("setSelection", { ids: [...ids] })}
      onChange={(next) => ctrl?.run("setSpecJson", { json: JSON.stringify(next) })}
    />
  );
}

function FormsTrySurfaceHost({ node }: { readonly node: UiFormsHostSurfaceNode }): ReactElement {
  const ctrl = useFormsPlayController();
  const spec = ctrl?.getSpec();
  const tryValues = ctrl?.getTryValues() ?? {};
  if (!spec) return <div className="p-double text-sm text-muted-foreground">No form loaded</div>;
  return (
    <FormRenderer
      spec={spec}
      values={tryValues}
      className="h-full"
      onChange={(values) => ctrl?.run("setTryValues", { values })}
      onSubmit={(values) => console.log("[DEBUG] forms try submit", values)}
    />
  );
}

class FormsPlayHierarchyPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: FORMS_PLAY_HIERARCHY_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = formsPlayControllerRef.current;
        const bus = new CommandBus();
        const treeNode = buildFormsPlayHierarchyTree(ctrl?.getSpec() ?? defaultFormSpec("empty"), ctrl?.getSelectedIds() ?? []);
        const config = uiTreeNodeToTreePanelConfig(treeNode, bus);
        return {
          ...config,
          dragAndDropController: createFormsPlayHierarchyTreeDragController(() => formsPlayControllerRef.current ?? undefined),
        };
      }),
    };
  }
}

class FormsPlayCataloguePanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: FORMS_PLAY_CATALOGUE_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
      order: 1,
      tree: new CallbackTreePanelDefinition(() => {
        const bus = new CommandBus();
        const treeNode = buildFormsPlayCatalogueTree();
        const config = uiTreeNodeToTreePanelConfig(treeNode, bus);
        return {
          ...config,
          dragAndDropController: formsQuestionPaletteTreeDragController(collectUiTreeItemDragData(treeNode.sections)),
        };
      }),
    };
  }
}

class FormsPlayInspectionPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: FORMS_PLAY_INSPECTION_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, "details"),
      name: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = formsPlayControllerRef.current;
        const bus = new CommandBus();
        const treeNode = buildFormsPlayInspectorTree(ctrl?.getSpec() ?? defaultFormSpec("empty"), ctrl?.getSelectedIds() ?? []);
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

function FormsPlayInner({ playground }: { readonly playground: Playground }): ReactElement {
  useFormsPlayController(playground.runtime);
  const interactionRevision = useFormsPlayInteractionRevision(playground.runtime);
  const extensionRevision = useFormsPlayExtensionRevision(playground.runtime);
  const formsPlayHierarchyPanel = reactHostPort.useMemo(() => new FormsPlayHierarchyPanelDefinition(), []);
  const formsPlayCataloguePanel = reactHostPort.useMemo(() => new FormsPlayCataloguePanelDefinition(), []);
  const formsPlayInspectionPanel = reactHostPort.useMemo(() => new FormsPlayInspectionPanelDefinition(), []);
  const augmentPanelTabs = reactHostPort.useMemo(
    () => ({
      workbench: [formsPlayHierarchyPanel, formsPlayCataloguePanel],
      details: [formsPlayInspectionPanel],
    }),
    [interactionRevision, extensionRevision, formsPlayCataloguePanel, formsPlayHierarchyPanel, formsPlayInspectionPanel],
  );
  const onPaletteDrop = reactHostPort.useCallback(
    (detail: { kind: string; clientX: number; clientY: number }) =>
      commitFormsPlayQuestionDropAtClient(formsPlayControllerRef.current ?? undefined, detail.clientX, detail.clientY, detail.kind),
    [],
  );
  return (
    <>
      <FormsQuestionPaletteDragBridge enabled onCommitDrop={onPaletteDrop} />
      <FormsQuestionPaletteDragGhost />
      <PlaygroundView runtime={playground.runtime} defaultAppId={FORMS_PLAY_APP_ID} augmentPanelTabs={augmentPanelTabs} playgroundKeybindings={playground.keybindings} />
    </>
  );
}

export function registerFormsPlaySurfaceHosts(): void {
  if (formsPlayChromeRegistered) return;
  formsPlayChromeRegistered = true;
  registerUiFormsSurfaceHost(FORMS_PLAY_SURFACE_ID_EDIT, FormsEditSurfaceHost);
  registerUiFormsSurfaceHost(FORMS_PLAY_SURFACE_ID_TRY, FormsTrySurfaceHost);
  registerFormsPlayDeclarativeBodies();
}

function FormsPlayChrome({ playground }: { readonly playground: Playground }): ReactElement {
  return <FormsPlayInner playground={playground} />;
}

export function mountFormsPlayChrome(playground: Playground, rootId = "root"): void {
  mountPlaygroundApp(<FormsPlayChrome playground={playground} />, rootId);
}

const formsPlayChromeBoot: PlaygroundChromeBoot = {
  registerHosts: registerFormsPlaySurfaceHosts,
  mount: mountFormsPlayChrome,
};

export function bootFormsPlay(playground: Playground, rootId = "root"): void {
  bootPlayground(playground, formsPlayChromeBoot, rootId);
}
//#endregion 🔖FormsPlayHost

//#region 🔖RasterPlayHost
import type { UiRasterHostSurfaceNode } from "@semio-tech/framework-platform-core";
import {
  RASTER_PLAY_APP_ID,
  RASTER_PLAY_CATALOGUE_TAB_ID,
  RASTER_PLAY_CONTROLLER_ID,
  RASTER_PLAY_LAYERS_TAB_ID,
  RASTER_PLAY_MASKS_TAB_ID,
  RASTER_PLAY_PROPERTIES_TAB_ID,
  RASTER_PLAY_SURFACE_ID_COMPOSITE,
  RASTER_PLAY_SURFACE_ID_NAVIGATOR,
  RasterPlayController,
  buildRasterPlayCatalogueTree,
  buildRasterPlayInspectorTree,
  buildRasterPlayLayersTree,
  buildRasterPlayMasksTree,
  createRasterPlayHierarchyTreeDragController,
  registerRasterPlayDeclarativeBodies,
  type RasterPlayHierarchyBuildOptions,
  type RasterPlayHostBridge,
} from "@semio-tech/raster-core";
import { RasterCanvas, RasterLayerView, RasterMaskView } from "@semio-tech/raster-react";

let rasterPlayChromeRegistered = false;
const rasterPlayControllerRef: { current: RasterPlayController | null } = { current: null };

function useRasterPlayController(runtimeOverride?: Platform): RasterPlayController | undefined {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const runtime = runtimeOverride ?? appCtx?.runtime;
  reactHostPort.useSyncExternalStore(
    (listener) => {
      const unsubscribeRuntime = runtime ? runtime.subscribe(listener) : () => {};
      const ctrl = runtime?.getActiveApp()?.controller as RasterPlayController | undefined;
      const unsubscribeCtrl = ctrl?.subscribe(listener);
      return () => {
        unsubscribeRuntime();
        unsubscribeCtrl?.();
      };
    },
    () => {
      const ctrl = runtime?.getActiveApp()?.controller as RasterPlayController | undefined;
      return ctrl?.getInteractionRevision() ?? runtime?.generation ?? 0;
    },
    () => 0,
  );
  const ctrl = runtime?.getActiveApp()?.controller as RasterPlayController | undefined;
  rasterPlayControllerRef.current = ctrl ?? null;
  return ctrl;
}

function rasterPlayHierarchyOptions(ctrl: RasterPlayController | undefined): RasterPlayHierarchyBuildOptions {
  return {
    onToggleVisible: (layerId) => ctrl?.run("toggleLayerVisible", { layerId }),
    onDeleteLayer: (layerId) => ctrl?.run("deleteLayer", { layerId }),
    onDuplicateLayer: (layerId) => ctrl?.run("duplicateLayer", { layerId }),
    onAddMask: (layerId) => ctrl?.run("addLayerMask", { layerId }),
  };
}

function useRasterPlayInteractionRevision(runtime: Platform): number {
  return reactHostPort.useSyncExternalStore(
    (listener) => {
      const ctrl = runtime.getActiveApp()?.controller as RasterPlayController | undefined;
      rasterPlayControllerRef.current = ctrl ?? null;
      const unsubscribeRuntime = runtime.subscribe(listener);
      const unsubscribeCtrl = ctrl?.subscribe(listener);
      return () => {
        unsubscribeRuntime();
        unsubscribeCtrl?.();
      };
    },
    () => (runtime.getActiveApp()?.controller as RasterPlayController | undefined)?.getInteractionRevision() ?? 0,
    () => 0,
  );
}

function RasterPlayPaneSurfaceHost({ node }: { readonly node: UiRasterHostSurfaceNode }): ReactElement {
  const ctrl = useRasterPlayController();
  const doc = ctrl?.getDocument();
  if (!doc) return <div className="p-double text-sm text-muted-foreground">No raster document</div>;
  const selectedIds = ctrl?.getSelectedIds() ?? [];
  const hoveredId = ctrl?.getHoveredId() ?? null;
  const kindHover = ctrl?.getHoveredKind() ?? null;
  const onHover = reactHostPort.useCallback((payload: import("@semio-tech/raster-core").RasterHoverPayload) => {
    ctrl?.run("setHover", { id: payload.id, kind: payload.kind, sourceId: CANVAS_HOVER_SOURCE_CANVAS });
  }, [ctrl]);
  const onViewportChange = reactHostPort.useCallback((viewport: import("@semio-tech/raster-core").RasterViewport) => {
    ctrl?.run("setCompositeViewport", viewport);
  }, [ctrl]);
  const common = {
    document: doc,
    selectedIds,
    hoveredId,
    kindHover,
    activeTool: doc.activeTool,
    camera: doc.camera,
    contentViewport: ctrl.getCompositeViewport(),
    onViewportChange: node.view === "composite" ? onViewportChange : undefined,
    onHover,
    onSelect: (ids: readonly string[]) => ctrl?.run("setSelection", { ids: [...ids] }),
    onCommit: (document: typeof doc, selectLayerId?: string) => ctrl?.run("commitDocument", { document, selectLayerId }),
    onCameraChange: (camera: typeof doc.camera) => ctrl?.run("setCamera", { camera }),
    className: "h-full",
  };
  if (node.view === "layer") {
    return <RasterLayerView {...common} isolatedLayerId={node.layerId ?? selectedIds[0] ?? null} />;
  }
  if (node.view === "mask") {
    return <RasterMaskView {...common} isolatedLayerId={node.layerId ?? selectedIds[0] ?? null} />;
  }
  if (node.view === "navigator") {
    return <RasterCanvas {...common} viewMode="navigator" />;
  }
  return <RasterCanvas {...common} viewMode="composite" />;
}

class RasterPlayLayersPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: RASTER_PLAY_LAYERS_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(
        () => {
          const ctrl = rasterPlayControllerRef.current;
          const doc = ctrl?.getDocument();
          const bus = new CommandBus();
          if (!doc) return { sections: [{ id: "raster-empty", items: [{ id: "empty", label: "No document" }] }] };
          const treeNode = buildRasterPlayLayersTree(
            doc,
            ctrl?.getSelectedIds() ?? [],
            ctrl?.getHoveredId() ?? null,
            ctrl?.getHoveredKind() ?? null,
            (payload) => ctrl?.run("setHover", { id: payload.id, kind: payload.kind, sourceId: CANVAS_HOVER_SOURCE_HIERARCHY }),
            rasterPlayHierarchyOptions(ctrl),
          );
          const config = uiTreeNodeToTreePanelConfig(treeNode, bus);
          return {
            ...config,
            dragAndDropController: createRasterPlayHierarchyTreeDragController(() => rasterPlayControllerRef.current ?? undefined),
          };
        },
        () => {
          const ctrl = rasterPlayControllerRef.current;
          const doc = ctrl?.getDocument();
          if (!doc) return [];
          return [...(buildRasterPlayLayersTree(doc, [], ctrl?.getHoveredId() ?? null, ctrl?.getHoveredKind() ?? null).highlightedIds ?? [])];
        },
      ),
    };
  }
}

class RasterPlayCataloguePanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: RASTER_PLAY_CATALOGUE_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
      order: 1,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = rasterPlayControllerRef.current;
        const bus = new CommandBus();
        const treeNode = buildRasterPlayCatalogueTree(
          ctrl?.getSelectedIds() ?? [],
          (payload) => ctrl?.run("setHover", { id: payload.id, kind: payload.kind, sourceId: CANVAS_HOVER_SOURCE_CATALOG }),
        );
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

class RasterPlayMasksPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: RASTER_PLAY_MASKS_TAB_ID,
      icon: shellTabIconComponent("square-dashed", "workbench"),
      name: "Masks",
      order: 1,
      tree: new CallbackTreePanelDefinition(
        () => {
          const ctrl = rasterPlayControllerRef.current;
          const doc = ctrl?.getDocument();
          const bus = new CommandBus();
          if (!doc) return { sections: [{ id: "raster-masks-empty", items: [{ id: "empty", label: "No masks" }] }] };
          const treeNode = buildRasterPlayMasksTree(
            doc,
            ctrl?.getSelectedIds() ?? [],
            ctrl?.getHoveredId() ?? null,
            ctrl?.getHoveredKind() ?? null,
            (payload) => ctrl?.run("setHover", { id: payload.id, kind: payload.kind, sourceId: CANVAS_HOVER_SOURCE_HIERARCHY }),
          );
          return uiTreeNodeToTreePanelConfig(treeNode, bus);
        },
        () => {
          const ctrl = rasterPlayControllerRef.current;
          const doc = ctrl?.getDocument();
          if (!doc) return [];
          return [...(buildRasterPlayMasksTree(doc, [], ctrl?.getHoveredId() ?? null, ctrl?.getHoveredKind() ?? null).highlightedIds ?? [])];
        },
      ),
    };
  }
}

class RasterPlayPropertiesPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: RASTER_PLAY_PROPERTIES_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, "details"),
      name: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = rasterPlayControllerRef.current;
        const doc = ctrl?.getDocument();
        const bus = new CommandBus();
        if (!doc) return { sections: [{ id: "raster-props-empty", items: [{ id: "empty", label: "No document" }] }] };
        const treeNode = buildRasterPlayInspectorTree(doc, ctrl?.getSelectedIds() ?? []);
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

function RasterPlayFileBridge(): ReactElement | null {
  const ctrl = useRasterPlayController();
  const loadInputRef = reactHostPort.useRef<HTMLInputElement | null>(null);
  const downloadFixture = reactHostPort.useCallback(async () => {
    if (!ctrl) return;
    const text = ctrl.getDocumentJson();
    const blob = new Blob([text], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const anchor = document.createElement("a");
    anchor.href = url;
    anchor.download = "semio.raster.json";
    anchor.click();
    URL.revokeObjectURL(url);
    console.log("[DEBUG] raster play exported document");
  }, [ctrl]);
  const handleLoadFile = reactHostPort.useCallback(
    (event: React.ChangeEvent<HTMLInputElement>) => {
      const file = event.target.files?.[0];
      event.target.value = "";
      if (!file || !ctrl) return;
      void file.text().then((text) => {
        ctrl.run("setFixtureJson", { json: text, resetInteraction: true });
        console.log("[DEBUG] raster play imported document from file");
      });
    },
    [ctrl],
  );
  reactHostPort.useEffect(() => {
    if (!ctrl) return;
    const bridge: RasterPlayHostBridge = {
      runHostCommand: (command) => {
        if (command === "saveDownload") {
          void downloadFixture();
          return;
        }
        if (command === "loadRequest") {
          loadInputRef.current?.click();
        }
      },
    };
    ctrl.setHostBridge(bridge);
    return () => ctrl.setHostBridge(null);
  }, [ctrl, downloadFixture]);
  return <input ref={loadInputRef} type="file" accept=".json,.raster.json,application/json" className="hidden" onChange={handleLoadFile} />;
}

function RasterPlayInner({ playground }: { readonly playground: Playground }): ReactElement {
  useRasterPlayController(playground.runtime);
  useRasterPlayInteractionRevision(playground.runtime);
  const rasterLayersPanel = reactHostPort.useMemo(() => new RasterPlayLayersPanelDefinition(), []);
  const rasterCataloguePanel = reactHostPort.useMemo(() => new RasterPlayCataloguePanelDefinition(), []);
  const rasterMasksPanel = reactHostPort.useMemo(() => new RasterPlayMasksPanelDefinition(), []);
  const rasterPropertiesPanel = reactHostPort.useMemo(() => new RasterPlayPropertiesPanelDefinition(), []);
  return (
    <>
      <RasterPlayFileBridge />
      <PlaygroundView
        runtime={playground.runtime}
        defaultAppId={RASTER_PLAY_APP_ID}
        augmentPanelTabs={{
          workbench: [rasterLayersPanel, rasterCataloguePanel, rasterMasksPanel],
          details: [rasterPropertiesPanel],
        }}
      />
    </>
  );
}

export function registerRasterPlaySurfaceHosts(): void {
  if (rasterPlayChromeRegistered) return;
  rasterPlayChromeRegistered = true;
  registerUiRasterSurfaceHost(RASTER_PLAY_SURFACE_ID_COMPOSITE, RasterPlayPaneSurfaceHost);
  registerUiRasterSurfaceHost(RASTER_PLAY_SURFACE_ID_NAVIGATOR, RasterPlayPaneSurfaceHost);
  registerRasterPlayDeclarativeBodies();
}

function RasterPlayChrome({ playground }: { readonly playground: Playground }): ReactElement {
  return <RasterPlayInner playground={playground} />;
}

export function mountRasterPlayChrome(playground: Playground, rootId = "root"): void {
  mountPlaygroundApp(<RasterPlayChrome playground={playground} />, rootId);
}

const rasterPlayChromeBoot: PlaygroundChromeBoot = {
  registerHosts: registerRasterPlaySurfaceHosts,
  mount: mountRasterPlayChrome,
};

export function bootRasterPlay(playground: Playground, rootId = "root"): void {
  bootPlayground(playground, rasterPlayChromeBoot, rootId);
}
//#endregion 🔖RasterPlayHost

//#region 🔖DrawPlayHost
import type { UiDrawHostSurfaceNode } from "@semio-tech/framework-platform-core";
import {
  DRAW_PLAY_APP_ID,
  DRAW_PLAY_CATALOGUE_TAB_ID,
  DRAW_PLAY_CONTROLLER_ID,
  DRAW_PLAY_LAYERS_TAB_ID,
  DRAW_PLAY_PROPERTIES_TAB_ID,
  DRAW_PLAY_SURFACE_ID_COMPOSITE,
  DRAW_PLAY_SURFACE_ID_NAVIGATOR,
  DrawPlayController,
  buildDrawPlayCatalogueTree,
  buildDrawPlayInspectorTree,
  buildDrawPlayLayersTree,
  createDrawPlayHierarchyTreeDragController,
  registerDrawPlayDeclarativeBodies,
  type DrawPlayHierarchyBuildOptions,
  type DrawPlayHostBridge,
} from "@semio-tech/draw-core";
import { DrawCanvas } from "@semio-tech/draw-react";

let drawPlayChromeRegistered = false;
const drawPlayControllerRef: { current: DrawPlayController | null } = { current: null };

function useDrawPlayController(runtimeOverride?: Platform): DrawPlayController | undefined {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const runtime = runtimeOverride ?? appCtx?.runtime;
  reactHostPort.useSyncExternalStore(
    (listener) => {
      const unsubscribeRuntime = runtime ? runtime.subscribe(listener) : () => {};
      const ctrl = runtime?.getActiveApp()?.controller as DrawPlayController | undefined;
      const unsubscribeCtrl = ctrl?.subscribe(listener);
      return () => {
        unsubscribeRuntime();
        unsubscribeCtrl?.();
      };
    },
    () => {
      const generation = runtime?.generation ?? 0;
      const revision = (runtime?.getActiveApp()?.controller as DrawPlayController | undefined)?.getInteractionRevision() ?? 0;
      return generation * 1_000_000 + revision;
    },
    () => 0,
  );
  const ctrl = runtime?.getActiveApp()?.controller as DrawPlayController | undefined;
  drawPlayControllerRef.current = ctrl ?? null;
  return ctrl;
}

function drawPlayHierarchyOptions(ctrl: DrawPlayController | undefined): DrawPlayHierarchyBuildOptions {
  return {
    onToggleVisible: (layerId) => ctrl?.run("toggleLayerVisible", { layerId }),
    onDeleteLayer: (layerId) => ctrl?.run("deleteLayer", { layerId }),
    onDuplicateLayer: (layerId) => ctrl?.run("duplicateLayer", { layerId }),
  };
}

function useDrawPlayInteractionRevision(runtime: Platform): number {
  return reactHostPort.useSyncExternalStore(
    (listener) => {
      const ctrl = runtime.getActiveApp()?.controller as DrawPlayController | undefined;
      drawPlayControllerRef.current = ctrl ?? null;
      const unsubscribeRuntime = runtime.subscribe(listener);
      const unsubscribeCtrl = ctrl?.subscribe(listener);
      return () => {
        unsubscribeRuntime();
        unsubscribeCtrl?.();
      };
    },
    () => (runtime.getActiveApp()?.controller as DrawPlayController | undefined)?.getInteractionRevision() ?? 0,
    () => 0,
  );
}

function DrawPlayPaneSurfaceHost({ node }: { readonly node: UiDrawHostSurfaceNode }): ReactElement {
  const ctrl = useDrawPlayController();
  const doc = ctrl?.getDocument();
  if (!doc) return <div className="p-double text-sm text-muted-foreground">No draw document</div>;
  const selectedIds = ctrl?.getSelectedIds() ?? [];
  const hoveredId = ctrl?.getHoveredId() ?? null;
  const kindHover = ctrl?.getHoveredKind() ?? null;
  const onHover = reactHostPort.useCallback((payload: import("@semio-tech/draw-core").DrawHoverPayload) => {
    ctrl?.run("setHover", { id: payload.id, kind: payload.kind, sourceId: CANVAS_HOVER_SOURCE_CANVAS });
  }, [ctrl]);
  const common = {
    document: doc,
    selectedIds,
    hoveredId,
    kindHover,
    activeTool: doc.activeTool,
    camera: doc.camera,
    onHover,
    onSelect: (ids: readonly string[]) => ctrl?.run("setSelection", { ids: [...ids] }),
    onCommit: (document: typeof doc, selectLayerId?: string) => ctrl?.run("commitDocument", { document, selectLayerId }),
    onCameraChange: (camera: typeof doc.camera) => ctrl?.run("setCamera", { camera }),
    className: "h-full",
  };
  if (node.view === "navigator") return <DrawCanvas {...common} viewMode="navigator" />;
  return <DrawCanvas {...common} viewMode="composite" />;
}

class DrawPlayLayersPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: DRAW_PLAY_LAYERS_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(
        () => {
          const ctrl = drawPlayControllerRef.current;
          const doc = ctrl?.getDocument();
          const bus = new CommandBus();
          if (!doc) return { sections: [{ id: "draw-empty", items: [{ id: "empty", label: "No document" }] }] };
          const treeNode = buildDrawPlayLayersTree(
            doc,
            ctrl?.getSelectedIds() ?? [],
            ctrl?.getHoveredId() ?? null,
            ctrl?.getHoveredKind() ?? null,
            (payload) => ctrl?.run("setHover", { id: payload.id, kind: payload.kind, sourceId: CANVAS_HOVER_SOURCE_HIERARCHY }),
            drawPlayHierarchyOptions(ctrl),
          );
          const config = uiTreeNodeToTreePanelConfig(treeNode, bus);
          return {
            ...config,
            dragAndDropController: createDrawPlayHierarchyTreeDragController(() => drawPlayControllerRef.current ?? undefined),
          };
        },
        () => {
          const ctrl = drawPlayControllerRef.current;
          const doc = ctrl?.getDocument();
          if (!doc) return [];
          return [...(buildDrawPlayLayersTree(doc, [], ctrl?.getHoveredId() ?? null, ctrl?.getHoveredKind() ?? null).highlightedIds ?? [])];
        },
      ),
    };
  }
}

class DrawPlayCataloguePanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: DRAW_PLAY_CATALOGUE_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
      order: 1,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = drawPlayControllerRef.current;
        const bus = new CommandBus();
        const treeNode = buildDrawPlayCatalogueTree(
          ctrl?.getSelectedIds() ?? [],
          (payload) => ctrl?.run("setHover", { id: payload.id, kind: payload.kind, sourceId: CANVAS_HOVER_SOURCE_HIERARCHY }),
        );
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

class DrawPlayPropertiesPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: DRAW_PLAY_PROPERTIES_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, "details"),
      name: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = drawPlayControllerRef.current;
        const doc = ctrl?.getDocument();
        const bus = new CommandBus();
        if (!doc) return { sections: [{ id: "draw-props-empty", items: [{ id: "empty", label: "No document" }] }] };
        const treeNode = buildDrawPlayInspectorTree(doc, ctrl?.getSelectedIds() ?? []);
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

function DrawPlayFileBridge(): ReactElement | null {
  const ctrl = useDrawPlayController();
  const loadInputRef = reactHostPort.useRef<HTMLInputElement | null>(null);
  const downloadFixture = reactHostPort.useCallback(async () => {
    if (!ctrl) return;
    const text = ctrl.getDocumentJson();
    const blob = new Blob([text], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const anchor = document.createElement("a");
    anchor.href = url;
    anchor.download = "semio.draw.json";
    anchor.click();
    URL.revokeObjectURL(url);
    console.log("[DEBUG] draw play exported document");
  }, [ctrl]);
  const handleLoadFile = reactHostPort.useCallback(
    (event: React.ChangeEvent<HTMLInputElement>) => {
      const file = event.target.files?.[0];
      event.target.value = "";
      if (!file || !ctrl) return;
      void file.text().then((text) => {
        ctrl.run("setFixtureJson", { json: text, resetInteraction: true });
        console.log("[DEBUG] draw play imported document from file");
      });
    },
    [ctrl],
  );
  reactHostPort.useEffect(() => {
    if (!ctrl) return;
    const bridge: DrawPlayHostBridge = {
      runHostCommand: (command) => {
        if (command === "saveDownload") {
          void downloadFixture();
          return;
        }
        if (command === "loadRequest") {
          loadInputRef.current?.click();
        }
      },
    };
    ctrl.setHostBridge(bridge);
    return () => ctrl.setHostBridge(null);
  }, [ctrl, downloadFixture]);
  return <input ref={loadInputRef} type="file" accept=".json,.draw.json,application/json" className="hidden" onChange={handleLoadFile} />;
}

function DrawPlayInner({ playground }: { readonly playground: Playground }): ReactElement {
  useDrawPlayController(playground.runtime);
  useDrawPlayInteractionRevision(playground.runtime);
  const drawLayersPanel = reactHostPort.useMemo(() => new DrawPlayLayersPanelDefinition(), []);
  const drawCataloguePanel = reactHostPort.useMemo(() => new DrawPlayCataloguePanelDefinition(), []);
  const drawPropertiesPanel = reactHostPort.useMemo(() => new DrawPlayPropertiesPanelDefinition(), []);
  return (
    <>
      <DrawPlayFileBridge />
      <PlaygroundView
        runtime={playground.runtime}
        defaultAppId={DRAW_PLAY_APP_ID}
        augmentPanelTabs={{
          workbench: [drawLayersPanel, drawCataloguePanel],
          details: [drawPropertiesPanel],
        }}
      />
    </>
  );
}

export function registerDrawPlaySurfaceHosts(): void {
  if (drawPlayChromeRegistered) return;
  drawPlayChromeRegistered = true;
  registerUiDrawSurfaceHost(DRAW_PLAY_SURFACE_ID_COMPOSITE, DrawPlayPaneSurfaceHost);
  registerUiDrawSurfaceHost(DRAW_PLAY_SURFACE_ID_NAVIGATOR, DrawPlayPaneSurfaceHost);
  registerDrawPlayDeclarativeBodies();
}

function DrawPlayChrome({ playground }: { readonly playground: Playground }): ReactElement {
  return <DrawPlayInner playground={playground} />;
}

export function mountDrawPlayChrome(playground: Playground, rootId = "root"): void {
  mountPlaygroundApp(<DrawPlayChrome playground={playground} />, rootId);
}

const drawPlayChromeBoot: PlaygroundChromeBoot = {
  registerHosts: registerDrawPlaySurfaceHosts,
  mount: mountDrawPlayChrome,
};

export function bootDrawPlay(playground: Playground, rootId = "root"): void {
  bootPlayground(playground, drawPlayChromeBoot, rootId);
}
//#endregion 🔖DrawPlayHost

//#region 🔖NotePlayHost
import type { UiNoteHostSurfaceNode } from "@semio-tech/framework-platform-core";
import {
  NOTE_PLAY_APP_ID,
  NOTE_PLAY_CATALOGUE_TAB_ID,
  NOTE_PLAY_CONTROLLER_ID,
  NOTE_PLAY_HIERARCHY_TAB_ID,
  NOTE_PLAY_PROPERTIES_TAB_ID,
  NOTE_PLAY_SURFACE_ID_COMPOSITE,
  NOTE_PLAY_SURFACE_ID_NAVIGATOR,
  NotePlayController,
  buildNotePlayCatalogueTree,
  buildNotePlayHierarchyTree,
  buildNotePlayInspectorTree,
  createNotePlayHierarchyTreeDragController,
  registerNotePlayDeclarativeBodies,
  type NotePlayHostBridge,
} from "@semio-tech/note-core";
import { NoteCanvas } from "@semio-tech/note-react";

let notePlayChromeRegistered = false;
const notePlayControllerRef: { current: NotePlayController | null } = { current: null };

function useNotePlayController(runtimeOverride?: Platform): NotePlayController | undefined {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const runtime = runtimeOverride ?? appCtx?.runtime;
  reactHostPort.useSyncExternalStore(
    (listener) => {
      const unsubscribeRuntime = runtime ? runtime.subscribe(listener) : () => {};
      const ctrl = runtime?.getActiveApp()?.controller as NotePlayController | undefined;
      const unsubscribeCtrl = ctrl?.subscribe(listener);
      return () => {
        unsubscribeRuntime();
        unsubscribeCtrl?.();
      };
    },
    () => {
      const generation = runtime?.generation ?? 0;
      const revision = (runtime?.getActiveApp()?.controller as NotePlayController | undefined)?.getInteractionRevision() ?? 0;
      return generation * 1_000_000 + revision;
    },
    () => 0,
  );
  const ctrl = runtime?.getActiveApp()?.controller as NotePlayController | undefined;
  notePlayControllerRef.current = ctrl ?? null;
  return ctrl;
}

function useNotePlayInteractionRevision(runtime: Platform): void {
  reactHostPort.useSyncExternalStore(
    (listener) => {
      const unsubscribeRuntime = runtime.subscribe(listener);
      const ctrl = runtime.getActiveApp()?.controller as NotePlayController | undefined;
      const unsubscribeCtrl = ctrl?.subscribe(listener);
      return () => {
        unsubscribeRuntime();
        unsubscribeCtrl?.();
      };
    },
    () => (runtime.getActiveApp()?.controller as NotePlayController | undefined)?.getInteractionRevision() ?? 0,
    () => 0,
  );
}

function NotePlayPaneSurfaceHost({ node }: { readonly node: UiNoteHostSurfaceNode }): ReactElement {
  const ctrl = useNotePlayController();
  const doc = ctrl?.getDocument();
  if (!doc) return <div className="p-double text-sm text-muted-foreground">No note document</div>;
  const common = {
    document: doc,
    selectedIds: ctrl?.getSelectedIds() ?? [],
    hoveredId: ctrl?.getHoveredId() ?? null,
    kindHover: ctrl?.getHoveredKind() ?? null,
    activeTool: doc.activeTool,
    camera: doc.camera,
    onHover: (payload: import("@semio-tech/note-core").NoteHoverPayload) => ctrl?.run("setHover", { id: payload.id, kind: payload.kind, sourceId: CANVAS_HOVER_SOURCE_CANVAS }),
    onSelect: (ids: readonly string[]) => ctrl?.run("setSelection", { ids: [...ids] }),
    onCommit: (document: typeof doc, selectBlockId?: string) => ctrl?.run("commitDocument", { document, selectBlockId }),
    onCameraChange: (camera: typeof doc.camera) => ctrl?.run("setCamera", { camera }),
    className: "h-full",
  };
  if (node.view === "navigator") return <NoteCanvas {...common} viewMode="navigator" />;
  return <NoteCanvas {...common} viewMode="composite" />;
}

class NotePlayHierarchyPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: NOTE_PLAY_HIERARCHY_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(
        () => {
          const ctrl = notePlayControllerRef.current;
          const doc = ctrl?.getDocument();
          const bus = new CommandBus();
          if (!doc) return { sections: [{ id: "note-empty", items: [{ id: "empty", label: "No document" }] }] };
          const treeNode = buildNotePlayHierarchyTree(
            doc,
            ctrl?.getSelectedIds() ?? [],
            ctrl?.getHoveredId() ?? null,
            ctrl?.getHoveredKind() ?? null,
            (payload) => ctrl?.run("setHover", { id: payload.id, kind: payload.kind, sourceId: CANVAS_HOVER_SOURCE_HIERARCHY }),
          );
          const config = uiTreeNodeToTreePanelConfig(treeNode, bus);
          return {
            ...config,
            dragAndDropController: createNotePlayHierarchyTreeDragController(() => notePlayControllerRef.current ?? undefined),
          };
        },
        () => {
          const ctrl = notePlayControllerRef.current;
          const doc = ctrl?.getDocument();
          if (!doc) return [];
          return [...(buildNotePlayHierarchyTree(doc, [], ctrl?.getHoveredId() ?? null, ctrl?.getHoveredKind() ?? null).highlightedIds ?? [])];
        },
      ),
    };
  }
}

class NotePlayCataloguePanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: NOTE_PLAY_CATALOGUE_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
      order: 1,
      tree: new CallbackTreePanelDefinition(() => {
        const bus = new CommandBus();
        const treeNode = buildNotePlayCatalogueTree((payload) =>
          notePlayControllerRef.current?.run("setHover", { id: payload.id, kind: payload.kind, sourceId: CANVAS_HOVER_SOURCE_CATALOG }),
        );
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

class NotePlayPropertiesPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: NOTE_PLAY_PROPERTIES_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, "details"),
      name: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = notePlayControllerRef.current;
        const doc = ctrl?.getDocument();
        const bus = new CommandBus();
        if (!doc) return { sections: [{ id: "note-props-empty", items: [{ id: "empty", label: "No document" }] }] };
        const treeNode = buildNotePlayInspectorTree(doc, ctrl?.getSelectedIds() ?? []);
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

function NotePlayFileBridge(): ReactElement | null {
  const ctrl = useNotePlayController();
  const loadInputRef = reactHostPort.useRef<HTMLInputElement | null>(null);
  const downloadFixture = reactHostPort.useCallback(async () => {
    if (!ctrl) return;
    const text = ctrl.getDocumentJson();
    const blob = new Blob([text], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const anchor = document.createElement("a");
    anchor.href = url;
    anchor.download = "semio.note.json";
    anchor.click();
    URL.revokeObjectURL(url);
    console.log("[DEBUG] note play exported document");
  }, [ctrl]);
  const handleLoadFile = reactHostPort.useCallback(
    (event: React.ChangeEvent<HTMLInputElement>) => {
      const file = event.target.files?.[0];
      event.target.value = "";
      if (!file || !ctrl) return;
      void file.text().then((text) => {
        ctrl.run("setFixtureJson", { json: text, resetInteraction: true });
        console.log("[DEBUG] note play imported document from file");
      });
    },
    [ctrl],
  );
  reactHostPort.useEffect(() => {
    if (!ctrl) return;
    const bridge: NotePlayHostBridge = {
      runHostCommand: (command) => {
        if (command === "saveDownload") {
          void downloadFixture();
          return;
        }
        if (command === "loadRequest") loadInputRef.current?.click();
      },
    };
    ctrl.setHostBridge(bridge);
    return () => ctrl.setHostBridge(null);
  }, [ctrl, downloadFixture]);
  return <input ref={loadInputRef} type="file" accept=".json,.note.json,application/json" className="hidden" onChange={handleLoadFile} />;
}

function NotePlayInner({ playground }: { readonly playground: Playground }): ReactElement {
  useNotePlayController(playground.runtime);
  useNotePlayInteractionRevision(playground.runtime);
  const hierarchyPanel = reactHostPort.useMemo(() => new NotePlayHierarchyPanelDefinition(), []);
  const cataloguePanel = reactHostPort.useMemo(() => new NotePlayCataloguePanelDefinition(), []);
  const propertiesPanel = reactHostPort.useMemo(() => new NotePlayPropertiesPanelDefinition(), []);
  return (
    <>
      <NotePlayFileBridge />
      <PlaygroundView
        runtime={playground.runtime}
        defaultAppId={NOTE_PLAY_APP_ID}
        playgroundKeybindings={playground.keybindings}
        augmentPanelTabs={{
          workbench: [hierarchyPanel, cataloguePanel],
          details: [propertiesPanel],
        }}
      />
    </>
  );
}

export function registerNotePlaySurfaceHosts(): void {
  if (notePlayChromeRegistered) return;
  notePlayChromeRegistered = true;
  registerUiNoteSurfaceHost(NOTE_PLAY_SURFACE_ID_COMPOSITE, NotePlayPaneSurfaceHost);
  registerUiNoteSurfaceHost(NOTE_PLAY_SURFACE_ID_NAVIGATOR, NotePlayPaneSurfaceHost);
  registerNotePlayDeclarativeBodies();
}

function NotePlayChrome({ playground }: { readonly playground: Playground }): ReactElement {
  return <NotePlayInner playground={playground} />;
}

export function mountNotePlayChrome(playground: Playground, rootId = "root"): void {
  mountPlaygroundApp(<NotePlayChrome playground={playground} />, rootId);
}

const notePlayChromeBoot: PlaygroundChromeBoot = {
  registerHosts: registerNotePlaySurfaceHosts,
  mount: mountNotePlayChrome,
};

export function bootNotePlay(playground: Playground, rootId = "root"): void {
  bootPlayground(playground, notePlayChromeBoot, rootId);
}
//#endregion 🔖NotePlayHost

//#region 🔖CadPlayHost
import { CadPlayRoot, registerCadPlaySurfaceHosts } from "@semio-tech/cad-js-renderer-react";

let cadPlayChromeRegistered = false;

function registerCadPlayPlaygroundHosts(): void {
  if (cadPlayChromeRegistered) return;
  cadPlayChromeRegistered = true;
  registerCadPlaySurfaceHosts();
}

function CadPlayPlaygroundChrome({ playground }: { readonly playground: Playground }): ReactElement {
  return <CadPlayRoot runtime={playground.runtime} />;
}

export function mountCadPlayChrome(playground: Playground, rootId = "root"): void {
  mountPlaygroundApp(<CadPlayPlaygroundChrome playground={playground} />, rootId);
}

const cadPlayChromeBoot: PlaygroundChromeBoot = {
  registerHosts: registerCadPlayPlaygroundHosts,
  mount: mountCadPlayChrome,
};

export function bootCadPlay(playground: Playground, rootId = "root"): void {
  bootPlayground(playground, cadPlayChromeBoot, rootId);
}
//#endregion 🔖CadPlayHost

//#region 🔖VcsPlayHost
import type { UiVcsHostSurfaceNode } from "@semio-tech/framework-platform-core";
import { HistoryTable } from "@semio-tech/vcs-react";
import {
  VCS_PLAY_APP_ID,
  VCS_PLAY_CONTROLLER_ID,
  VCS_PLAY_SURFACE_ID_EDITOR,
  VCS_PLAY_SURFACE_ID_HISTORY,
  VcsPlayController,
  registerVcsPlayDeclarativeBodies,
} from "@semio-tech/vcs-core";

let vcsPlayChromeRegistered = false;
const vcsPlayControllerRef: { current: VcsPlayController | null } = { current: null };

function useVcsPlayController(runtimeOverride?: Platform): VcsPlayController | undefined {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const runtime = runtimeOverride ?? appCtx?.runtime;
  reactHostPort.useSyncExternalStore(
    (listener) => (runtime ? runtime.subscribe(listener) : () => {}),
    () => runtime?.generation ?? 0,
    () => 0,
  );
  const ctrl = runtime?.getActiveApp()?.controller as VcsPlayController | undefined;
  vcsPlayControllerRef.current = ctrl ?? null;
  return ctrl;
}

function useVcsPlayInteractionRevision(runtime: Platform): number {
  return reactHostPort.useSyncExternalStore(
    (listener) => {
      const ctrl = runtime.getActiveApp()?.controller as VcsPlayController | undefined;
      vcsPlayControllerRef.current = ctrl ?? null;
      const unsubscribeRuntime = runtime.subscribe(listener);
      const unsubscribeSnapshot = ctrl?.subscribeSnapshot(listener);
      return () => {
        unsubscribeRuntime();
        unsubscribeSnapshot?.();
      };
    },
    () => (runtime.getActiveApp()?.controller as VcsPlayController | undefined)?.getInteractionRevision() ?? 0,
    () => 0,
  );
}

function VcsPlayEditorSurfaceHost({ node: _node }: { readonly node: UiVcsHostSurfaceNode }): ReactElement {
  const ctrl = useVcsPlayController();
  const projection = ctrl?.projection();
  if (!ctrl || !projection) {
    return <div className="p-double text-sm text-muted-foreground">No VCS document</div>;
  }
  return (
    <div className="flex h-full min-h-0 flex-col gap-double p-double">
      <div className="flex flex-wrap items-center gap-single">
        <button type="button" className="rounded border px-2 py-1 text-xs" onClick={() => ctrl.run("incrementCounter")}>
          + Counter ({projection.counter})
        </button>
        <button type="button" className="rounded border px-2 py-1 text-xs" onClick={() => ctrl.run("commitCheckpoint")}>
          Commit checkpoint
        </button>
        <button type="button" className="rounded border px-2 py-1 text-xs" onClick={() => ctrl.run("undo")}>
          Undo
        </button>
        <button type="button" className="rounded border px-2 py-1 text-xs" onClick={() => ctrl.run("redo")}>
          Redo
        </button>
        <button type="button" className="rounded border px-2 py-1 text-xs" onClick={() => ctrl.run("createAlternative")}>
          New alternative
        </button>
      </div>
      <section className="rounded border p-double text-sm">
        <div>
          <strong>{projection.title}</strong> · counter {projection.counter}
        </div>
        <div className="text-muted-foreground">{projection.notes || "—"}</div>
      </section>
    </div>
  );
}

function VcsPlayHistorySurfaceHost({ node: _node }: { readonly node: UiVcsHostSurfaceNode }): ReactElement {
  const ctrl = useVcsPlayController();
  const columns = ctrl?.historyColumns() ?? [];
  return (
    <div className="h-full min-h-0 overflow-auto p-single">
      <HistoryTable columns={columns} />
    </div>
  );
}

function VcsPlayInner({ playground }: { readonly playground: Playground }): ReactElement {
  useVcsPlayController(playground.runtime);
  useVcsPlayInteractionRevision(playground.runtime);
  return <PlaygroundView runtime={playground.runtime} defaultAppId={VCS_PLAY_APP_ID} />;
}

export function registerVcsPlaySurfaceHosts(): void {
  if (vcsPlayChromeRegistered) return;
  vcsPlayChromeRegistered = true;
  registerUiVcsSurfaceHost(VCS_PLAY_SURFACE_ID_EDITOR, VcsPlayEditorSurfaceHost);
  registerUiVcsSurfaceHost(VCS_PLAY_SURFACE_ID_HISTORY, VcsPlayHistorySurfaceHost);
  registerVcsPlayDeclarativeBodies();
}

function VcsPlayChrome({ playground }: { readonly playground: Playground }): ReactElement {
  return <VcsPlayInner playground={playground} />;
}

export function mountVcsPlayChrome(playground: Playground, rootId = "root"): void {
  mountPlaygroundApp(<VcsPlayChrome playground={playground} />, rootId);
}

const vcsPlayChromeBoot: PlaygroundChromeBoot = {
  registerHosts: registerVcsPlaySurfaceHosts,
  mount: mountVcsPlayChrome,
};

export function bootVcsPlay(playground: Playground, rootId = "root"): void {
  bootPlayground(playground, vcsPlayChromeBoot, rootId);
}
//#endregion 🔖VcsPlayHost

//#region 🔖WriterPlayHost
import type { UiWriterHostSurfaceNode } from "@semio-tech/framework-platform-core";
import { createJackLspWorker as createWriterJackLspWorker } from "@semio-tech/trinity-react";
import { createWorkerLspTransport as createWriterPlayWorkerLspTransport, createWriterDocument as createWriterPlayDocument } from "@semio-tech/writer-core";
import { WriterCanvas as WriterPlayCanvas } from "@semio-tech/writer-react";
import {
  WRITER_PLAY_APP_ID,
  WRITER_PLAY_CONTROLLER_ID,
  WRITER_PLAY_SURFACE_ID,
  WriterPlayController,
  buildWriterPlayCatalogueTree,
  buildWriterPlayHierarchyTree,
  buildWriterPlayInspectorTree,
  registerWriterPlayDeclarativeBodies,
} from "@semio-tech/writer-core";

let writerPlayChromeRegistered = false;
const writerPlayControllerRef: { current: WriterPlayController | null } = { current: null };

function useWriterPlayController(runtimeOverride?: Platform): WriterPlayController | undefined {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const runtime = runtimeOverride ?? appCtx?.runtime;
  reactHostPort.useSyncExternalStore(
    (listener) => (runtime ? runtime.subscribe(listener) : () => {}),
    () => runtime?.generation ?? 0,
    () => 0,
  );
  const ctrl = runtime?.getActiveApp()?.controller as WriterPlayController | undefined;
  writerPlayControllerRef.current = ctrl ?? null;
  return ctrl;
}

function WriterPlaySurfaceHost({ node: _node }: { readonly node: UiWriterHostSurfaceNode }): ReactElement {
  const ctrl = useWriterPlayController();
  const document = ctrl?.getDocument() ?? createWriterPlayDocument({ id: "jack", languageId: "jack", text: "" });
  const formatSignal = ctrl?.getFormatSignal() ?? 0;
  const lintSignal = ctrl?.getLintSignal() ?? 0;
  const editorSelection = ctrl?.getEditorSelection();
  const editorSelectionSignal = ctrl?.getEditorSelectionSignal() ?? 0;
  const externalHoverRange = ctrl?.getHoveredAstSpan() ?? null;
  const externalHoverSignal = ctrl?.getExternalHoverSignal() ?? 0;
  const editorSettings = ctrl?.getEditorSettings();
  const createLspTransport = reactHostPort.useCallback(() => createWriterPlayWorkerLspTransport(createWriterJackLspWorker()), []);
  const onChange = reactHostPort.useCallback((next: import("@semio-tech/writer-core").WriterDocument) => {
    const ctrl = writerPlayControllerRef.current;
    if (!ctrl) return;
    const prev = ctrl.getDocument();
    if (prev.id === next.id && prev.languageId === next.languageId && prev.uri === next.uri && prev.schema === next.schema) {
      ctrl.run("setText", { text: next.text });
      return;
    }
    ctrl.run("setDocument", { document: next });
  }, []);
  const onLintMessages = reactHostPort.useCallback((messages: readonly string[]) => {
    writerPlayControllerRef.current?.setLintMessages(messages);
  }, []);
  const onSelectionChange = reactHostPort.useCallback((range: { readonly start: number; readonly end: number }) => {
    writerPlayControllerRef.current?.run("setEditorSelection", range);
  }, []);
  const onHoverChange = reactHostPort.useCallback((offset: number | null) => {
    writerPlayControllerRef.current?.run("setEditorHover", { offset });
  }, []);
  return (
    <WriterPlayCanvas
      document={document}
      onChange={onChange}
      createLspTransport={createLspTransport}
      formatSignal={formatSignal}
      lintSignal={lintSignal}
      onLintMessages={onLintMessages}
      externalSelection={editorSelection}
      externalSelectionSignal={editorSelectionSignal}
      externalHoverRange={externalHoverRange}
      externalHoverSignal={externalHoverSignal}
      onSelectionChange={onSelectionChange}
      onHoverChange={onHoverChange}
      editorSettings={editorSettings}
      className="h-full"
    />
  );
}

export function registerWriterPlaySurfaceHosts(): void {
  if (writerPlayChromeRegistered) return;
  writerPlayChromeRegistered = true;
  registerUiWriterSurfaceHost(WRITER_PLAY_SURFACE_ID, WriterPlaySurfaceHost);
  registerWriterPlayDeclarativeBodies();
}

class WriterPlayHierarchyPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: "framework.panel.hierarchy",
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(
        () => {
          const ctrl = writerPlayControllerRef.current;
          const bus = new CommandBus();
          return uiTreeNodeToTreePanelConfig(
            buildWriterPlayHierarchyTree(
              ctrl?.getDocument() ?? createWriterPlayDocument({ id: "jack", languageId: "jack" }),
              ctrl?.getSelectedAstIds() ?? [],
              ctrl?.getHoveredAstId() ?? null,
              (id) => ctrl?.run("setAstHover", { id }),
            ),
            bus,
          );
        },
        () => {
          const ctrl = writerPlayControllerRef.current;
          const hovered = ctrl?.getHoveredAstId();
          return hovered ? [hovered] : [];
        },
      ),
    };
  }
}

class WriterPlayCataloguePanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: "framework.panel.catalogue",
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
      order: 1,
      tree: new CallbackTreePanelDefinition(() => uiTreeNodeToTreePanelConfig(buildWriterPlayCatalogueTree(), new CommandBus())),
    };
  }
}

class WriterPlayInspectionPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: "framework.panel.inspection",
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, "details"),
      name: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = writerPlayControllerRef.current;
        const bus = new CommandBus();
        return uiTreeNodeToTreePanelConfig(
          buildWriterPlayInspectorTree(
            ctrl?.getDocument() ?? createWriterPlayDocument({ id: "jack", languageId: "jack" }),
            ctrl?.getLintMessages() ?? [],
          ),
          bus,
        );
      }),
    };
  }
}

function WriterPlayInner({ runtime }: { readonly runtime: Platform }): ReactElement {
  useWriterPlayController(runtime);
  const hierarchy = reactHostPort.useMemo(() => new WriterPlayHierarchyPanelDefinition(), []);
  const catalogue = reactHostPort.useMemo(() => new WriterPlayCataloguePanelDefinition(), []);
  const inspection = reactHostPort.useMemo(() => new WriterPlayInspectionPanelDefinition(), []);
  return (
    <PlaygroundView runtime={runtime} defaultAppId={WRITER_PLAY_APP_ID} augmentPanelTabs={{ workbench: [hierarchy, catalogue], details: [inspection] }} />
  );
}

export function mountWriterPlayChrome(playground: Playground, rootId = "root"): void {
  mountPlaygroundApp(<WriterPlayInner runtime={playground.runtime} />, rootId);
}

const writerPlayChromeBoot: PlaygroundChromeBoot = {
  registerHosts: registerWriterPlaySurfaceHosts,
  mount: mountWriterPlayChrome,
};

export function bootWriterPlay(playground: Playground, rootId = "root"): void {
  bootPlayground(playground, writerPlayChromeBoot, rootId);
}
//#endregion 🔖WriterPlayHost

//#region 🔖PresentationPlayHost
import {
    FIGURE_TILE_PDF_PAGE_ASPECT,
    NORMALIZED_RECT_MIN_FRACTION,
    figureTileMediaKindFromFile,
    moveNormalizedRect,
    resizeNormalizedRect,
    type DispositionPosition,
    type FigureTileMediaKind,
    type FigureTileSource,
    type NormalizedRectHandle,
} from "@semio-tech/framework-presentation-core";
import {
    PRESENTATION_PLAY_CONTROLLER_ID,
    PRESENTATION_PLAY_ICON_DETAILS,
    PRESENTATION_PLAY_ICON_HIERARCHY,
    PRESENTATION_PLAY_IDLE_SNAPSHOT,
    PRESENTATION_PLAY_STORE_ID,
    PRESENTATION_PLAY_SURFACE_ID,
    PresentationPlayController,
    registerPresentationPlayDeclarativeBodies,
    type PresentationPlaySnapshot
} from "@semio-tech/framework-presentation-core";

const PRESENTATION_TILE_HANDLES: readonly NormalizedRectHandle[] = ["nw", "n", "ne", "e", "se", "s", "sw", "w"];
const PRESENTATION_TILE_VIEWPORT_MIN_ZOOM = 0.2;
const PRESENTATION_TILE_VIEWPORT_MAX_ZOOM = 12;
const PRESENTATION_FIGURE_FILE_ACCEPT =
	"image/*,video/*,application/pdf,.pdf,.svg,.png,.jpg,.jpeg,.webp,.gif,.bmp,.avif,.mp4,.webm,.ogg,.ogv,.mov,.m4v,.mkv";

function clampFigureTileZoom(zoom: number): number {
	return Math.min(PRESENTATION_TILE_VIEWPORT_MAX_ZOOM, Math.max(PRESENTATION_TILE_VIEWPORT_MIN_ZOOM, zoom));
}

interface FigureTileViewportState {
	readonly zoom: number;
	readonly panX: number;
	readonly panY: number;
}

interface FigureTileContentLayout {
	readonly width: number;
	readonly height: number;
	readonly offsetX: number;
	readonly offsetY: number;
}

function figureTileContentLayout(viewportWidth: number, viewportHeight: number, aspect: number): FigureTileContentLayout {
	if (viewportWidth <= 0 || viewportHeight <= 0) {
		return { width: 1, height: 1, offsetX: 0, offsetY: 0 };
	}
	const viewportAspect = viewportWidth / viewportHeight;
	if (viewportAspect >= aspect) {
		const height = viewportHeight;
		const width = height * aspect;
		return { width, height, offsetX: (viewportWidth - width) / 2, offsetY: 0 };
	}
	const width = viewportWidth;
	const height = width / aspect;
	return { width, height, offsetX: 0, offsetY: (viewportHeight - height) / 2 };
}

function figureTileZoomAtClient(
	viewport: FigureTileViewportState,
	clientX: number,
	clientY: number,
	viewportRect: DOMRect,
	layout: FigureTileContentLayout,
	deltaScale: number,
): FigureTileViewportState {
	const nextZoom = clampFigureTileZoom(viewport.zoom * deltaScale);
	if (nextZoom === viewport.zoom) {
		return viewport;
	}
	const anchorX = clientX - viewportRect.left;
	const anchorY = clientY - viewportRect.top;
	const contentX = (anchorX - layout.offsetX - viewport.panX) / viewport.zoom;
	const contentY = (anchorY - layout.offsetY - viewport.panY) / viewport.zoom;
	return {
		zoom: nextZoom,
		panX: anchorX - layout.offsetX - contentX * nextZoom,
		panY: anchorY - layout.offsetY - contentY * nextZoom,
	};
}

function revokeFigureObjectUrl(url: string | null): void {
	if (url?.startsWith("blob:")) {
		URL.revokeObjectURL(url);
	}
}

function probeFigureTileMediaAspect(
	src: string,
	kind: FigureTileMediaKind,
): Promise<number> {
	if (kind === "video") {
		return new Promise((resolve, reject) => {
			const video = document.createElement("video");
			video.preload = "metadata";
			video.onloadedmetadata = () => {
				const aspect = video.videoWidth > 0 && video.videoHeight > 0 ? video.videoWidth / video.videoHeight : 16 / 9;
				resolve(aspect);
			};
			video.onerror = () => reject(new Error("video metadata"));
			video.src = src;
		});
	}
	if (kind === "pdf") {
		return Promise.resolve(FIGURE_TILE_PDF_PAGE_ASPECT);
	}
	return new Promise((resolve, reject) => {
		const img = new Image();
		img.onload = () => {
			const aspect = img.naturalWidth > 0 && img.naturalHeight > 0 ? img.naturalWidth / img.naturalHeight : 1;
			resolve(aspect);
		};
		img.onerror = () => reject(new Error("image metadata"));
		img.src = src;
	});
}

function FigureTileMediaPreview(props: { readonly source: FigureTileSource }): ReactElement {
	const { source } = props;
	const kind = source.kind ?? "figure";
	if (kind === "video") {
		return (
			<video
				className="pointer-events-none absolute inset-0 h-full w-full object-contain"
				src={source.src}
				muted
				playsInline
				preload="metadata"
				controls={false}
			/>
		);
	}
	if (kind === "pdf") {
		const page = source.pdfPage ?? 1;
		const pdfSrc = `${source.src}#page=${page}&view=FitH`;
		return <iframe className="pointer-events-none absolute inset-0 h-full w-full border-0 bg-background" src={pdfSrc} title="PDF preview" />;
	}
	return <img alt="" className="pointer-events-none absolute inset-0 h-full w-full object-contain" draggable={false} src={source.src} />;
}

function FigureSourcePicker(props: {
	readonly onPickFile: (file: File) => void;
}): ReactElement {
	const { onPickFile } = props;
	const fileInputRef = reactHostPort.useRef<HTMLInputElement | null>(null);
	const [dragActive, setDragActive] = reactHostPort.useState(false);

	const onInputChange = reactHostPort.useCallback(
		(event: React.ChangeEvent<HTMLInputElement>) => {
			const file = event.target.files?.[0];
			if (file) {
				onPickFile(file);
			}
			event.target.value = "";
		},
		[onPickFile],
	);

	const onDragOver = reactHostPort.useCallback((event: React.DragEvent<HTMLDivElement>) => {
		event.preventDefault();
		setDragActive(true);
	}, []);

	const onDragLeave = reactHostPort.useCallback((event: React.DragEvent<HTMLDivElement>) => {
		event.preventDefault();
		setDragActive(false);
	}, []);

	const onDrop = reactHostPort.useCallback(
		(event: React.DragEvent<HTMLDivElement>) => {
			event.preventDefault();
			setDragActive(false);
			const file = event.dataTransfer.files?.[0];
			if (file) {
				onPickFile(file);
			}
		},
		[onPickFile],
	);

	return (
		<div
			className={cn(
				"flex min-h-0 flex-1 flex-col items-center justify-center gap-3 border-dashed p-6 text-center",
				floatingFieldSurfaceClass,
				dragActive && "border-primary",
			)}
			onDragLeave={onDragLeave}
			onDragOver={onDragOver}
			onDrop={onDrop}
		>
			<Icon icon="image-up" size="large" className="text-muted-foreground" />
			<div className="flex flex-col gap-1">
				<p className={shellChromeTitleClassName}>Pick figure media</p>
				<p className="text-muted-foreground text-xs">Image, SVG, video, or PDF — drag and drop or choose a file</p>
			</div>
			<Button id="presentation.play.pick-figure" type="button" variant="secondary" onClick={() => fileInputRef.current?.click()}>
				Choose file…
			</Button>
			<input accept={PRESENTATION_FIGURE_FILE_ACCEPT} className="hidden" onChange={onInputChange} ref={fileInputRef} type="file" />
		</div>
	);
}

function usePresentationPlayController(): PresentationPlayController | undefined {
	const { runtime } = useApp();
	return runtime.getActiveApp()?.controller as PresentationPlayController | undefined;
}

function usePresentationPlaySnapshot(): PresentationPlaySnapshot {
	const ctrl = usePresentationPlayController();
	return useControllerStore(ctrl, PRESENTATION_PLAY_STORE_ID) ?? PRESENTATION_PLAY_IDLE_SNAPSHOT;
}

function clampUnit(value: number): number {
	return Math.min(1, Math.max(0, value));
}

function normalizedPointFromClient(
	clientX: number,
	clientY: number,
	viewportRect: DOMRect,
	viewport: FigureTileViewportState,
	layout: FigureTileContentLayout,
): { readonly x: number; readonly y: number } {
	const localX = (clientX - viewportRect.left - layout.offsetX - viewport.panX) / viewport.zoom;
	const localY = (clientY - viewportRect.top - layout.offsetY - viewport.panY) / viewport.zoom;
	return {
		x: clampUnit(localX / layout.width),
		y: clampUnit(localY / layout.height),
	};
}

function normalizedRectFromDrag(
	start: { readonly x: number; readonly y: number },
	end: { readonly x: number; readonly y: number },
): DispositionPosition {
	const x = Math.min(start.x, end.x);
	const y = Math.min(start.y, end.y);
	const width = Math.max(NORMALIZED_RECT_MIN_FRACTION, Math.abs(end.x - start.x));
	const height = Math.max(NORMALIZED_RECT_MIN_FRACTION, Math.abs(end.y - start.y));
	return {
		x: clampUnit(x),
		y: clampUnit(y),
		width: Math.min(width, 1 - x),
		height: Math.min(height, 1 - y),
	};
}

function FigureTilesSurfaceHost({ node }: { readonly node: UiPanelHostSurfaceNode }): ReactElement {
	const { runtime } = useApp();
	const controller = usePresentationPlayController();
	const snapshot = usePresentationPlaySnapshot();
	const viewportRef = reactHostPort.useRef<HTMLDivElement | null>(null);
	const contentRef = reactHostPort.useRef<HTMLDivElement | null>(null);
	const figureObjectUrlRef = reactHostPort.useRef<string | null>(null);
	const spacePressedRef = reactHostPort.useRef(false);
	const [viewportSize, setViewportSize] = reactHostPort.useState({ width: 0, height: 0 });
	const [viewport, setViewport] = reactHostPort.useState<FigureTileViewportState>({ zoom: 1, panX: 0, panY: 0 });
	const [spacePressed, setSpacePressed] = reactHostPort.useState(false);
	const [isPanning, setIsPanning] = reactHostPort.useState(false);
	const [marquee, setMarquee] = reactHostPort.useState<{ readonly start: { readonly x: number; readonly y: number }; readonly end: { readonly x: number; readonly y: number } } | null>(null);
	const dragRef = reactHostPort.useRef<
		| {
				readonly kind: "move" | NormalizedRectHandle | "marquee" | "pan";
				readonly tileId?: string;
				readonly startClient: { readonly x: number; readonly y: number };
				readonly startCrop?: DispositionPosition;
				readonly marqueeStart?: { readonly x: number; readonly y: number };
				readonly startPan?: { readonly x: number; readonly y: number };
		  }
		| null
	>(null);

	reactHostPort.useEffect(() => {
		if (!snapshot.clipboardPrompt || snapshot.clipboardEpoch <= 0) {
			return;
		}
		void navigator.clipboard?.writeText(snapshot.clipboardPrompt).catch(() => undefined);
	}, [snapshot.clipboardEpoch, snapshot.clipboardPrompt]);

	const dispatch = reactHostPort.useCallback(
		(command: string, args?: unknown) => {
			if (!controller) {
				return;
			}
			runtime.commandBus.dispatch(controller.id, command, args);
		},
		[controller, runtime.commandBus],
	);

	reactHostPort.useEffect(() => () => revokeFigureObjectUrl(figureObjectUrlRef.current), []);

	reactHostPort.useEffect(() => {
		setViewport({ zoom: 1, panX: 0, panY: 0 });
	}, [snapshot.source.src]);

	reactHostPort.useEffect(() => {
		const onKeyDown = (event: KeyboardEvent) => {
			if (event.code !== "Space" || event.target instanceof HTMLInputElement || event.target instanceof HTMLTextAreaElement) {
				return;
			}
			event.preventDefault();
			spacePressedRef.current = true;
			setSpacePressed(true);
		};
		const onKeyUp = (event: KeyboardEvent) => {
			if (event.code !== "Space") {
				return;
			}
			spacePressedRef.current = false;
			setSpacePressed(false);
		};
		window.addEventListener("keydown", onKeyDown);
		window.addEventListener("keyup", onKeyUp);
		return () => {
			window.removeEventListener("keydown", onKeyDown);
			window.removeEventListener("keyup", onKeyUp);
		};
	}, []);

	const applyFigureFile = reactHostPort.useCallback(
		(file: File) => {
			const kind = figureTileMediaKindFromFile(file.type, file.name);
			if (!kind) {
				return;
			}
			revokeFigureObjectUrl(figureObjectUrlRef.current);
			const url = URL.createObjectURL(file);
			figureObjectUrlRef.current = url;
			void probeFigureTileMediaAspect(url, kind)
				.then((sourceAspect) => {
					dispatch("setSource", {
						src: url,
						kind,
						sourceAspect,
						...(kind === "pdf" ? { pdfPage: 1 } : {}),
					});
				})
				.catch(() => {
					revokeFigureObjectUrl(url);
					if (figureObjectUrlRef.current === url) {
						figureObjectUrlRef.current = null;
					}
				});
		},
		[dispatch],
	);

	const hasFigure = snapshot.source.src.trim().length > 0;
	const aspect = snapshot.source.sourceAspect ?? 1;
	const contentLayout = reactHostPort.useMemo(
		() => figureTileContentLayout(viewportSize.width, viewportSize.height, aspect),
		[aspect, viewportSize.height, viewportSize.width],
	);

	reactHostPort.useEffect(() => {
		const element = viewportRef.current;
		if (!element || !hasFigure) {
			return;
		}
		const observer = new ResizeObserver(([entry]) => {
			const { width, height } = entry.contentRect;
			setViewportSize({ width, height });
		});
		observer.observe(element);
		return () => observer.disconnect();
	}, [hasFigure]);

	reactHostPort.useEffect(() => {
		const element = viewportRef.current;
		if (!element || !hasFigure) {
			return;
		}
		const onWheel = (event: WheelEvent) => {
			event.preventDefault();
			const rect = element.getBoundingClientRect();
			const layout = figureTileContentLayout(viewportSize.width, viewportSize.height, aspect);
			const deltaScale = event.deltaY < 0 ? 1.1 : 1 / 1.1;
			setViewport((current) => figureTileZoomAtClient(current, event.clientX, event.clientY, rect, layout, deltaScale));
		};
		element.addEventListener("wheel", onWheel, { passive: false });
		return () => element.removeEventListener("wheel", onWheel);
	}, [aspect, hasFigure, viewportSize.height, viewportSize.width]);

	const viewportPoint = reactHostPort.useCallback(
		(clientX: number, clientY: number) => {
			const rect = viewportRef.current?.getBoundingClientRect();
			if (!rect) {
				return { x: 0, y: 0 };
			}
			return normalizedPointFromClient(clientX, clientY, rect, viewport, contentLayout);
		},
		[contentLayout, viewport],
	);

	const onContentPointerDown = reactHostPort.useCallback(
		(event: React.PointerEvent<HTMLDivElement>) => {
			if (!viewportRef.current) {
				return;
			}
			const target = event.target as HTMLElement;
			if (target.dataset.tileHandle || target.dataset.tileId) {
				return;
			}
			if (event.button === 1 || (event.button === 0 && (spacePressedRef.current || event.altKey))) {
				dragRef.current = {
					kind: "pan",
					startClient: { x: event.clientX, y: event.clientY },
					startPan: { x: viewport.panX, y: viewport.panY },
				};
				setIsPanning(true);
				event.currentTarget.setPointerCapture(event.pointerId);
				return;
			}
			if (event.button !== 0) {
				return;
			}
			const point = viewportPoint(event.clientX, event.clientY);
			dragRef.current = {
				kind: "marquee",
				startClient: { x: event.clientX, y: event.clientY },
				marqueeStart: point,
			};
			setMarquee({ start: point, end: point });
			event.currentTarget.setPointerCapture(event.pointerId);
		},
		[viewport.panX, viewport.panY, viewportPoint],
	);

	const onTilePointerDown = reactHostPort.useCallback(
		(tileId: string, crop: DispositionPosition) => (event: React.PointerEvent) => {
			event.stopPropagation();
			if (spacePressedRef.current || event.altKey) {
				return;
			}
			dispatch("setSelectedIds", { ids: [tileId] });
			dragRef.current = {
				kind: "move",
				tileId,
				startClient: { x: event.clientX, y: event.clientY },
				startCrop: crop,
			};
			event.currentTarget.setPointerCapture(event.pointerId);
		},
		[dispatch],
	);

	const onHandlePointerDown = reactHostPort.useCallback(
		(tileId: string, crop: DispositionPosition, handle: NormalizedRectHandle) => (event: React.PointerEvent) => {
			event.stopPropagation();
			dispatch("setSelectedIds", { ids: [tileId] });
			dragRef.current = {
				kind: handle,
				tileId,
				startClient: { x: event.clientX, y: event.clientY },
				startCrop: crop,
			};
			event.currentTarget.setPointerCapture(event.pointerId);
		},
		[dispatch],
	);

	const onPointerMove = reactHostPort.useCallback(
		(event: React.PointerEvent<HTMLDivElement>) => {
			const drag = dragRef.current;
			if (!drag) {
				return;
			}
			if (drag.kind === "pan" && drag.startPan) {
				setViewport((current) => ({
					...current,
					panX: drag.startPan!.x + (event.clientX - drag.startClient.x),
					panY: drag.startPan!.y + (event.clientY - drag.startClient.y),
				}));
				return;
			}
			const scaleX = contentLayout.width * viewport.zoom;
			const scaleY = contentLayout.height * viewport.zoom;
			const dx = scaleX > 0 ? (event.clientX - drag.startClient.x) / scaleX : 0;
			const dy = scaleY > 0 ? (event.clientY - drag.startClient.y) / scaleY : 0;
			if (drag.kind === "marquee" && drag.marqueeStart) {
				const end = viewportPoint(event.clientX, event.clientY);
				setMarquee({ start: drag.marqueeStart, end });
				return;
			}
			if (!drag.tileId || !drag.startCrop) {
				return;
			}
			const nextCrop =
				drag.kind === "move"
					? moveNormalizedRect(drag.startCrop, dx, dy)
					: resizeNormalizedRect(drag.startCrop, drag.kind, dx, dy);
			dispatch("setTileCrop", { id: drag.tileId, crop: nextCrop });
		},
		[contentLayout.height, contentLayout.width, dispatch, viewport.zoom, viewportPoint],
	);

	const onPointerUp = reactHostPort.useCallback(
		(event: React.PointerEvent<HTMLDivElement>) => {
			const drag = dragRef.current;
			if (!drag) {
				return;
			}
			if (drag.kind === "marquee" && drag.marqueeStart) {
				const end = viewportPoint(event.clientX, event.clientY);
				const crop = normalizedRectFromDrag(drag.marqueeStart, end);
				dispatch("addTile", { crop });
				setMarquee(null);
			}
			if (drag.kind === "pan") {
				setIsPanning(false);
			}
			dragRef.current = null;
			try {
				event.currentTarget.releasePointerCapture(event.pointerId);
			} catch {
				// pointer already released
			}
		},
		[dispatch, viewportPoint],
	);

	const onViewportDoubleClick = reactHostPort.useCallback((event: React.MouseEvent<HTMLDivElement>) => {
		const target = event.target as HTMLElement;
		if (target.dataset.tileHandle || target.dataset.tileId) {
			return;
		}
		setViewport({ zoom: 1, panX: 0, panY: 0 });
	}, []);

	if (node.controllerId !== PRESENTATION_PLAY_CONTROLLER_ID || node.surfaceId !== PRESENTATION_PLAY_SURFACE_ID) {
		return <div className="p-2 text-xs text-muted-foreground">Invalid presentation tile surface binding</div>;
	}

	if (!hasFigure) {
		return (
			<div className="flex h-full min-h-0 w-full p-2">
				<FigureSourcePicker onPickFile={applyFigureFile} />
			</div>
		);
	}

	const viewportCursor = isPanning ? "grabbing" : spacePressed ? "grab" : undefined;

	return (
		<div className="flex h-full min-h-0 w-full flex-col">
			<div ref={viewportRef} className="relative min-h-0 flex-1 overflow-hidden bg-muted/30" style={{ cursor: viewportCursor }}>
				<div
					ref={contentRef}
					className="absolute touch-none select-none"
					style={{
						left: contentLayout.offsetX,
						top: contentLayout.offsetY,
						width: contentLayout.width,
						height: contentLayout.height,
						transform: `translate(${viewport.panX}px, ${viewport.panY}px) scale(${viewport.zoom})`,
						transformOrigin: "0 0",
					}}
					onPointerDown={onContentPointerDown}
					onPointerMove={onPointerMove}
					onPointerUp={onPointerUp}
					onPointerCancel={onPointerUp}
					onDoubleClick={onViewportDoubleClick}
				>
					<FigureTileMediaPreview source={snapshot.source} />
					{snapshot.tiles.map((tile) => {
						const selected = snapshot.selectedIds.includes(tile.id);
						return (
							<div
								key={tile.id}
								data-tile-id={tile.id}
								className={cn(
									"absolute box-border cursor-move border-2",
									selected ? "border-primary bg-primary/20" : "border-accent bg-accent/10",
								)}
								style={{
									left: `${tile.crop.x * 100}%`,
									top: `${tile.crop.y * 100}%`,
									width: `${tile.crop.width * 100}%`,
									height: `${tile.crop.height * 100}%`,
								}}
								onPointerDown={onTilePointerDown(tile.id, tile.crop)}
							>
								<span className={cn("pointer-events-none absolute left-0 top-0 max-w-full truncate px-1 text-2xs", floatingMenuSurfaceClass)}>{tile.name}</span>
								{selected
									? PRESENTATION_TILE_HANDLES.map((handle) => (
											<button
												key={handle}
												type="button"
												data-tile-handle={handle}
												className="bg-primary absolute z-10 size-2 -translate-x-1/2 -translate-y-1/2 rounded-full border border-background"
												style={{
													left: handle.includes("w") ? "0%" : handle.includes("e") ? "100%" : "50%",
													top: handle.includes("n") ? "0%" : handle.includes("s") ? "100%" : "50%",
													cursor: `${handle}-resize`,
												}}
												onPointerDown={onHandlePointerDown(tile.id, tile.crop, handle)}
											/>
										))
									: null}
							</div>
						);
					})}
					{marquee ? (
						<div
							className="border-primary/80 bg-primary/10 pointer-events-none absolute border border-dashed"
							style={{
								left: `${Math.min(marquee.start.x, marquee.end.x) * 100}%`,
								top: `${Math.min(marquee.start.y, marquee.end.y) * 100}%`,
								width: `${Math.abs(marquee.end.x - marquee.start.x) * 100}%`,
								height: `${Math.abs(marquee.end.y - marquee.start.y) * 100}%`,
							}}
						/>
					) : null}
				</div>
			</div>
		</div>
	);
}

let presentationPlayChromeRegistered = false;

export function registerPresentationPlaySurfaceHosts(): void {
	if (presentationPlayChromeRegistered) {
		return;
	}
	presentationPlayChromeRegistered = true;
	registerUiPanelSurfaceHost(PRESENTATION_PLAY_SURFACE_ID, FigureTilesSurfaceHost);
	registerPresentationPlayDeclarativeBodies();
	registerTabIcon(PRESENTATION_PLAY_ICON_HIERARCHY, "list-tree");
	registerTabIcon(PRESENTATION_PLAY_ICON_DETAILS, "clipboard-list");
}

function PresentationPlayChrome({ playground }: { readonly playground: Playground }): ReactElement {
	return (
		<PlaygroundView
			runtime={playground.runtime}
			defaultAppId={PRESENTATION_PLAY_CONTROLLER_ID}
			playgroundKeybindings={playground.keybindings}
		/>
	);
}

export function mountPresentationPlayChrome(playground: Playground, rootId = "root"): void {
	mountPlaygroundApp(<PresentationPlayChrome playground={playground} />, rootId);
}

const presentationPlayChromeBoot: PlaygroundChromeBoot = {
	registerHosts: registerPresentationPlaySurfaceHosts,
	mount: mountPresentationPlayChrome,
};

export function bootPresentationPlay(playground: Playground, rootId = "root"): void {
	bootPlayground(playground, presentationPlayChromeBoot, rootId);
}
//#endregion 🔖PresentationPlayHost

//#region 🔖SPlayHost
import type { UiSHostSurfaceNode } from "@semio-tech/framework-platform-core";
import {
	S_PLAY_APP_ID,
	S_PLAY_CONTROLLER_ID,
	S_PLAY_SURFACE_APP_HOST,
	S_PLAY_SURFACE_HISTORY,
	S_PLAY_SURFACE_LAUNCHER,
	S_PLAY_SURFACE_MEDIA_GRAPH,
	S_PLAY_SURFACE_JACK,
	S_PLAY_SURFACE_COMPILED_DAG,
	S_PLAY_BODY_JACK,
	S_PLAY_WINDOW_JACK,
	SPlayController,
	buildSPlayInspectorTree,
	registerSPlayDeclarativeBodies,
} from "@semio-tech/s-core";
import {
	SAppHostSurface,
	SMediaGraphCanvas,
	SProgramLauncherPanel,
	SStudioHistoryPanel,
	SStudioProvider,
	buildSPlayCatalogueTree,
} from "@semio-tech/s-react";
import {
	appInstanceResourceProjection,
	materializeAppInstanceProjection,
	sResourceDescriptor,
} from "@semio-tech/s-core";
import { defaultDrawDocument, drawDocumentFromJson, drawDocumentToJson, type DrawDocument } from "@semio-tech/draw-core";
import { defaultNoteDocument, noteDocumentFromJson, noteDocumentToJson, type NoteDocument } from "@semio-tech/note-core";
import { defaultRasterDocument, parseRasterDocument, rasterDocumentToJson, type RasterDocument } from "@semio-tech/raster-core";
import type { FormSpec } from "@semio-tech/forms-core";
import { PlayCanvas as Puzzle3dPlayCanvas, parseFixture as parsePuzzle3dFixture } from "@semio-tech/puzzle-3d-react";
import { PresentationDeck } from "@semio-tech/framework-presentation-renderer-react";
import type { PresentationDeck as PresentationDeckDocument } from "@semio-tech/framework-presentation-core";

const EMPTY_PUZZLE3D_FIXTURE = {
	schema: "puzzle.3d.fixture",
	camera: { position: [4, 4, 4], target: [0, 0, 0], zoom: 1 },
	objects: [],
	attractions: [],
	references: [],
	targetVolumes: [],
} as const;

function SPuzzle3dHost({
	fixtureJson,
	onFixtureChange,
}: {
	readonly fixtureJson: string;
	readonly onFixtureChange: (json: string) => void;
}): ReactElement {
	const fixture = reactHostPort.useMemo(() => parsePuzzle3dFixture(JSON.parse(fixtureJson)) ?? EMPTY_PUZZLE3D_FIXTURE, [fixtureJson]);
	const [selectedId, setSelectedId] = reactHostPort.useState<string | null>(null);
	const kindCatalogs = reactHostPort.useMemo(() => parseKindCatalogs(fixture.meta as Record<string, unknown> | undefined), [fixture]);
	const patchFixture = reactHostPort.useCallback(
		(updater: (prev: Fixture) => Fixture) => {
			onFixtureChange(JSON.stringify(updater(fixture)));
		},
		[fixture, onFixtureChange],
	);
	return (
		<ObjectStateProvider fixture={fixture}>
			<Puzzle3dPlayCanvas
				fixture={fixture}
				kindCatalogs={kindCatalogs}
				setSelectedId={setSelectedId}
				selectedId={selectedId}
				fixtureDragDrop
				onBrushPlace={(payload) => patchFixture((prev) => applyBrushPlacementToFixture(prev, payload, kindCatalogs))}
				onConnect={(payload) => patchFixture((prev) => applyConnectToFixture(prev, payload))}
				onReferenceRelocate={(payload) => patchFixture((prev) => applyReferenceRelocateToFixture(prev, payload))}
				onTargetVolumeRelocate={(payload) => patchFixture((prev) => applyTargetVolumeRelocateToFixture(prev, payload))}
				onCamera={(camera) => patchFixture((prev) => ({ ...prev, camera }))}
				className="h-full"
			/>
		</ObjectStateProvider>
	);
}

function SPuzzle2dHost({
	fixtureJson,
	onFixtureChange,
}: {
	readonly fixtureJson: string;
	readonly onFixtureChange: (json: string) => void;
}): ReactElement {
	const fixture = reactHostPort.useMemo(() => {
		try {
			return JSON.parse(fixtureJson);
		} catch {
			return null;
		}
	}, [fixtureJson]);
	const declarativeSceneDescriptor = reactHostPort.useMemo(
		() => (fixture ? buildPuzzle2dSceneDescriptorFromFixture(fixture) : undefined),
		[fixture],
	);
	const patchFixture = reactHostPort.useCallback(
		(updater: (prev: Record<string, unknown>) => Record<string, unknown>) => {
			if (!fixture) return;
			onFixtureChange(JSON.stringify(updater(fixture)));
		},
		[fixture, onFixtureChange],
	);
	const onDragEnd = reactHostPort.useCallback(
		(payload: { moves: Array<{ id: string; x: number; y: number }> }) => {
			if (!payload.moves.length || !fixture) return;
			const byId = new Map(payload.moves.map((move) => [move.id, move]));
			patchFixture((prev) => ({
				...prev,
				nodes: (prev.nodes as Array<{ id: string; x: number; y: number }>).map((node) => {
					const move = byId.get(node.id);
					return move ? { ...node, x: move.x, y: move.y } : node;
				}),
			}));
		},
		[fixture, patchFixture],
	);
	const onConnect = reactHostPort.useCallback(
		(payload: { id: string; source: string; target: string }) => {
			patchFixture((prev) => ({
				...prev,
				edges: [...((prev.edges as unknown[]) ?? []), { id: payload.id, source: payload.source, target: payload.target }],
			}));
		},
		[patchFixture],
	);
	const onDelete = reactHostPort.useCallback(
		(payload: { kind: "node" | "edge"; id: string }) => {
			if (payload.kind === "node") {
				patchFixture((prev) => ({
					...prev,
					nodes: (prev.nodes as Array<{ id: string }>).filter((node) => node.id !== payload.id),
					edges: (prev.edges as Array<{ source: string; target: string }>).filter(
						(edge) => edge.source !== payload.id && edge.target !== payload.id,
					),
				}));
				return;
			}
			patchFixture((prev) => ({
				...prev,
				edges: (prev.edges as Array<{ id: string }>).filter((edge) => edge.id !== payload.id),
			}));
		},
		[patchFixture],
	);
	if (!declarativeSceneDescriptor) {
		return <div className="p-4 text-sm text-muted-foreground">Invalid puzzle 2D fixture</div>;
	}
	return (
		<Puzzle2dCanvas
			declarativeSceneDescriptor={declarativeSceneDescriptor}
			camera={fixture?.camera}
			fixtureDragDrop
			onDragEnd={onDragEnd}
			onConnect={onConnect}
			onDelete={onDelete}
			className="h-full"
		/>
	);
}

function SLowpolyHost({
	fixtureJson,
	onFixtureChange,
}: {
	readonly fixtureJson: string;
	readonly onFixtureChange: (json: string) => void;
}): ReactElement {
	const [session, setSession] = reactHostPort.useState<LowpolySessionWasm | null>(null);
	const [sceneObjects, setSceneObjects] = reactHostPort.useState<readonly LowpolySceneObject[]>([]);
	const [selectedIds, setSelectedIds] = reactHostPort.useState<readonly number[]>([]);
	reactHostPort.useEffect(() => {
		let cancelled = false;
		void (async () => {
			const nextSession = await createLowpolySession();
			if (cancelled) return;
			const json = isLowpolyFixtureReady(fixtureJson) ? fixtureJson : loadDefaultLowpolyFixtureJson();
			safeLoadLowpolyFixture(nextSession, json);
			setSession(nextSession);
			setSceneObjects(tessellateAllLowpolySession(nextSession));
		})();
		return () => {
			cancelled = true;
		};
	}, []);
	reactHostPort.useEffect(() => {
		if (!session || !isLowpolyFixtureReady(fixtureJson)) return;
		safeLoadLowpolyFixture(session, fixtureJson);
		setSceneObjects(tessellateAllLowpolySession(session));
	}, [fixtureJson, session]);
	return (
		<LowpolyCanvas
			fixtureJson={fixtureJson}
			sceneObjects={sceneObjects}
			selectionMode="object"
			selectedIds={selectedIds}
			transformTool="move"
			session={session}
			onFixtureChange={onFixtureChange}
			onSelectionChange={(_mode, ids) => setSelectedIds(ids)}
			onSceneChange={(objects) => setSceneObjects(objects)}
			className="h-full w-full"
		/>
	);
}

function SVcsHost({
	projection,
	onIncrement,
	onCommitCheckpoint,
}: {
	readonly projection: { readonly title: string; readonly counter: number; readonly notes?: string };
	readonly onIncrement: () => void;
	readonly onCommitCheckpoint: () => void;
}): ReactElement {
	return (
		<div className="flex h-full min-h-0 flex-col gap-3 p-4">
			<div className="flex flex-wrap items-center gap-2">
				<button type="button" className="rounded border px-2 py-1 text-xs" onClick={onIncrement}>
					+ Counter ({projection.counter})
				</button>
				<button type="button" className="rounded border px-2 py-1 text-xs" onClick={onCommitCheckpoint}>
					Commit checkpoint
				</button>
			</div>
			<section className="rounded border p-3 text-sm">
				<div>
					<strong>{projection.title}</strong> · counter {projection.counter}
				</div>
				<div className="text-muted-foreground">{projection.notes || "—"}</div>
			</section>
			<div className="min-h-0 flex-1 overflow-auto rounded border">
				<HistoryTable columns={[]} />
			</div>
		</div>
	);
}

function STrinityRewriteHost({
	fixtureJson,
	onFixtureChange,
}: {
	readonly fixtureJson: string;
	readonly onFixtureChange: (json: string) => void;
}): ReactElement {
	const [lhsJson, setLhsJson] = reactHostPort.useState(REWRITE_DEFAULT_LHS_FIXTURE_JSON);
	const [rhsJson, setRhsJson] = reactHostPort.useState(REWRITE_DEFAULT_RHS_FIXTURE_JSON);
	const lhsFixture = reactHostPort.useMemo(
		() => parseRewriteGraphFixtureJson(lhsJson) ?? REWRITE_DEFAULT_LHS_FIXTURE,
		[lhsJson],
	);
	const rhsFixture = reactHostPort.useMemo(
		() => parseRewriteGraphFixtureJson(rhsJson) ?? REWRITE_DEFAULT_RHS_FIXTURE,
		[rhsJson],
	);
	const lhsScene = reactHostPort.useMemo(() => buildPuzzle2dSceneDescriptorFromFixture(lhsFixture), [lhsFixture]);
	const rhsScene = reactHostPort.useMemo(() => buildPuzzle2dSceneDescriptorFromFixture(rhsFixture), [rhsFixture]);
	const patchLhs = reactHostPort.useCallback(
		(payload: { moves: Array<{ id: string; x: number; y: number }> }) => {
			if (!payload.moves.length) return;
			const current = parseRewriteGraphFixtureJson(lhsJson) ?? REWRITE_DEFAULT_LHS_FIXTURE;
			const byId = new Map(payload.moves.map((move) => [move.id, move]));
			setLhsJson(
				JSON.stringify({
					...current,
					nodes: current.nodes.map((entry) => {
						const move = byId.get(entry.id);
						return move ? { ...entry, x: move.x, y: move.y } : entry;
					}),
				}),
			);
		},
		[lhsJson],
	);
	const patchRhs = reactHostPort.useCallback(
		(payload: { moves: Array<{ id: string; x: number; y: number }> }) => {
			if (!payload.moves.length) return;
			const current = parseRewriteGraphFixtureJson(rhsJson) ?? REWRITE_DEFAULT_RHS_FIXTURE;
			const byId = new Map(payload.moves.map((move) => [move.id, move]));
			setRhsJson(
				JSON.stringify({
					...current,
					nodes: current.nodes.map((entry) => {
						const move = byId.get(entry.id);
						return move ? { ...entry, x: move.x, y: move.y } : entry;
					}),
				}),
			);
		},
		[rhsJson],
	);
	return (
		<div className="grid h-full min-h-0 grid-cols-2 grid-rows-2 gap-1">
			<TrinityCanvas fixtureJson={fixtureJson} onFixtureChange={onFixtureChange} reorganize className="min-h-0" />
			<TrinityCanvas fixtureJson={fixtureJson} className="min-h-0" />
			<Puzzle2dCanvas
				declarativeSceneDescriptor={lhsScene}
				camera={lhsFixture.camera}
				kindCatalogs={rewriteLhsKindCatalogs()}
				fixtureDragDrop
				onDragEnd={patchLhs}
				className="min-h-0"
			/>
			<Puzzle2dCanvas
				declarativeSceneDescriptor={rhsScene}
				camera={rhsFixture.camera}
				kindCatalogs={rewriteRhsKindCatalogs()}
				fixtureDragDrop
				onDragEnd={patchRhs}
				className="min-h-0"
			/>
		</div>
	);
}

function SGisMapHost({ fixtureJson }: { readonly fixtureJson: string }): ReactElement {
	const mapFixture = reactHostPort.useMemo(() => parseGisMapFixtureV1(JSON.parse(fixtureJson)), [fixtureJson]);
	const [selectedPositionIds, setSelectedPositionIds] = reactHostPort.useState<readonly string[]>([]);
	const [selectedRouteIds, setSelectedRouteIds] = reactHostPort.useState<readonly string[]>([]);
	const [hoveredFeature, setHoveredFeature] = reactHostPort.useState<MapHoveredFeature | null>(null);
	if (!mapFixture) {
		return <div className="p-4 text-sm text-destructive">Invalid GIS map fixture</div>;
	}
	return (
		<MapCanvas
			className="h-full"
			selectedPositionIds={selectedPositionIds}
			selectedRouteIds={selectedRouteIds}
			hoveredFeature={hoveredFeature}
			onSelect={(payload) => {
				setSelectedPositionIds(payload.positions);
				setSelectedRouteIds(payload.routes);
			}}
			onHoverChange={setHoveredFeature}
		>
			{mapFixture.positions.map((position) => (
				<Position
					key={position.id}
					id={position.id}
					lon={position.lon}
					lat={position.lat}
					label={position.label}
					name={position.name}
					icon={position.icon}
					sourceUrl={position.sourceUrl}
					kind={position.kind}
				/>
			))}
			{mapFixture.routes.map((route) => (
				<Route key={route.id} id={route.id} points={route.points} />
			))}
		</MapCanvas>
	);
}

function SCatalogueHost({ bundle }: { readonly bundle: unknown }): ReactElement {
	const sections = (bundle as { sections?: readonly { id: string; label?: string; kinds?: readonly { id: string; name?: string }[] }[] })?.sections ?? [];
	const kinds = (bundle as { kinds?: readonly { id: string; name?: string }[] })?.kinds ?? [];
	return (
		<div className="h-full overflow-auto p-4 text-sm">
			{kinds.length > 0 ? (
				<ul className="space-y-1">
					{kinds.map((kind) => (
						<li key={kind.id}>
							{kind.name ?? kind.id} <span className="text-muted-foreground">({kind.id})</span>
						</li>
					))}
				</ul>
			) : null}
			{sections.map((section) => (
				<section key={section.id} className="mb-4">
					<div className="mb-2 font-medium">{section.label ?? section.id}</div>
					<ul className="space-y-1 pl-3">
						{(section.kinds ?? []).map((kind) => (
							<li key={kind.id}>
								{kind.name ?? kind.id} <span className="text-muted-foreground">({kind.id})</span>
							</li>
						))}
					</ul>
				</section>
			))}
			{kinds.length === 0 && sections.length === 0 ? (
				<pre className="whitespace-pre-wrap text-xs text-muted-foreground">{JSON.stringify(bundle, null, 2)}</pre>
			) : null}
		</div>
	);
}

function SPuzzle5dHost({
	model,
	instanceId,
	onModelChange,
}: {
	readonly model: Model;
	readonly instanceId: string;
	readonly onModelChange: (model: Model) => void;
}): ReactElement {
	const store = reactHostPort.useMemo(() => createStore(model), [model]);
	reactHostPort.useEffect(() => {
		const unsub = store.subscribe(() => {
			onModelChange(store.read());
		});
		return unsub;
	}, [store, onModelChange]);
	return (
		<StoreProvider store={store}>
			<FiveD mode="3d" instanceId={instanceId} className="h-full" />
		</StoreProvider>
	);
}

function SPresentationDeckHost({ deck }: { readonly deck: PresentationDeckDocument }): ReactElement {
	return <PresentationDeck presentation={deck as never} />;
}

const SCadPlayRoot = reactHostPort.lazy(() =>
	import("@semio-tech/cad-js-renderer-react").then((module) => ({ default: module.CadPlayRoot })),
);

function SUpstreamBadge({
	upstreamInstanceId,
	instances,
}: {
	readonly upstreamInstanceId: string | null;
	readonly instances: readonly import("@semio-tech/s-core").SAppInstance[];
}): ReactElement | null {
	if (!upstreamInstanceId) return null;
	const upstream = instances.find((entry) => entry.id === upstreamInstanceId);
	if (!upstream) return null;
	return (
		<div className="border-b border-border/60 bg-muted/40 px-3 py-1 text-xs text-muted-foreground">
			Upstream · {upstream.label} ({upstream.yields})
		</div>
	);
}

function SSketchpadHost({ appId }: { readonly appId: string }): ReactElement {
	const [platform, setPlatform] = reactHostPort.useState<Platform | null>(null);
	reactHostPort.useEffect(() => {
		let active = true;
		void import("@semio-tech/compose-sketchpad").then(({ ensureSketchpadPlatform }) =>
			ensureSketchpadPlatform().then((runtime) => {
				if (!active) return;
				runtime.activeAppId = appId;
				runtime.notify();
				setPlatform(runtime);
			}),
		);
		return () => {
			active = false;
		};
	}, [appId]);
	if (!platform) {
		return <div className="flex h-full items-center justify-center p-6 text-sm text-muted-foreground">Loading sketchpad…</div>;
	}
	return <PlaygroundView runtime={platform} defaultAppId={appId} />;
}

let sPlayChromeRegistered = false;
const sPlayControllerRef: { current: SPlayController | null } = { current: null };

function useSPlayController(runtimeOverride?: Platform): SPlayController | undefined {
	const appCtx = reactHostPort.useContext(PlaygroundContext);
	const runtime = runtimeOverride ?? appCtx?.runtime;
	reactHostPort.useSyncExternalStore(
		(listener) => (runtime ? runtime.subscribe(listener) : () => {}),
		() => runtime?.generation ?? 0,
		() => 0,
	);
	const ctrl = runtime?.getActiveApp()?.controller as SPlayController | undefined;
	sPlayControllerRef.current = ctrl ?? null;
	return ctrl;
}

function SMediaGraphSurfaceHost({ node: _node }: { readonly node: UiSHostSurfaceNode }): ReactElement {
	const ctrl = useSPlayController();
	const generation = ctrl?.getStudioStore().getGeneration() ?? 0;
	void generation;
	const projection = ctrl?.getStudioStore().projection() ?? {
		activeProgramId: null,
		activeAlternativeId: null,
		appInstances: [],
		mediaGraph: { schema: "s.media-graph", nodes: [], edges: [] },
	};
	const activeInstanceId = ctrl?.getActiveInstanceId() ?? null;
	const store = ctrl?.getStudioStore();
	const onSelect = reactHostPort.useCallback((instanceId: string) => {
		const current = sPlayControllerRef.current;
		if (!current) return;
		const node = current.getStudioStore().projection().mediaGraph.nodes.find((row) => row.instanceId === instanceId);
		current.run("setMediaNodeSelection", { nodeIds: node ? [node.id] : [] });
		current.run("selectInstance", { instanceId });
	}, []);
	return (
		<SMediaGraphCanvas
			graph={projection.mediaGraph}
			instances={projection.appInstances}
			activeInstanceId={activeInstanceId}
			onSelectInstance={onSelect}
			onOpenInstance={(instanceId) => sPlayControllerRef.current?.run("openInstance", { instanceId })}
			editable
			onMoveNode={(nodeId, x, y) => store?.dispatch({ kind: "moveMediaNode", nodeId, x, y })}
			onConnectPorts={(sourceNodeId, sourcePortId, targetNodeId, targetPortId) =>
				store?.dispatch({ kind: "connectMediaPorts", sourceNodeId, sourcePortId, targetNodeId, targetPortId })
			}
			onDisconnectEdge={(edgeId) => store?.dispatch({ kind: "disconnectMediaEdge", edgeId })}
			onRemoveInstance={(instanceId) => store?.dispatch({ kind: "removeAppInstance", instanceId })}
			onSpawnApp={(programId, appId, position) => sPlayControllerRef.current?.run("spawnApp", { programId, appId, position })}
			peers={store?.getPresencePeers() ?? []}
		/>
	);
}

function SLauncherSurfaceHost({ node: _node }: { readonly node: UiSHostSurfaceNode }): ReactElement {
	return <SProgramLauncherPanel />;
}

function SHistorySurfaceHost({ node: _node }: { readonly node: UiSHostSurfaceNode }): ReactElement {
	return <SStudioHistoryPanel />;
}

function SAppHostContent({ instance }: { readonly instance: import("@semio-tech/s-core").SAppInstance | null }): ReactElement {
	return <SAppHostRouter instance={instance} />;
}

export function SAppHostRouter({ instance }: { readonly instance: import("@semio-tech/s-core").SAppInstance | null }): ReactElement {
	const ctrl = useSPlayController();
	const store = ctrl?.getStudioStore();
	const generation = store?.getGeneration() ?? 0;
	const projection = store?.projection();
	const resourceBundle = reactHostPort.useMemo(() => {
		if (!instance || !store) return null;
		const current = store.projection();
		return appInstanceResourceProjection(current.mediaGraph, current.appInstances, instance.id);
	}, [instance, store, generation]);
	const materialized = resourceBundle?.projection;
	const upstreamInstanceId = resourceBundle?.upstreamInstanceId ?? null;
	const resource = instance ? sResourceDescriptor(instance.yields) : null;
	const dispatchDraw = reactHostPort.useCallback(
		(document: DrawDocument) => {
			if (!instance || !store) return;
			store.dispatch({
				kind: "patchAppSource",
				instanceId: instance.id,
				inline: drawDocumentToJson(document),
			});
		},
		[instance, store],
	);
	const drawDoc = reactHostPort.useMemo(() => {
		if (instance?.sourceDocument.payloadRef === "fixture:semio.draw.json") return defaultDrawDocument("semio", "Semio Emblem");
		if (materialized && typeof materialized === "object" && (materialized as DrawDocument).schema === "draw.document") return materialized as DrawDocument;
		return defaultDrawDocument(instance?.id ?? "draw");
	}, [instance, materialized]);
	const rasterDoc = reactHostPort.useMemo(() => {
		if (materialized && typeof materialized === "object" && (materialized as RasterDocument).schema === "raster.document") {
			return materialized as RasterDocument;
		}
		return defaultRasterDocument(instance?.id ?? "raster");
	}, [instance, materialized]);
	const formsSpec = reactHostPort.useMemo(() => {
		if (materialized && typeof materialized === "object" && (materialized as FormSpec).schema === "forms.form") {
			return materialized as FormSpec;
		}
		if (instance?.sourceDocument.inline) {
			try {
				return parseFormSpec(JSON.parse(instance.sourceDocument.inline));
			} catch {
				return defaultFormSpec(instance?.id ?? "forms");
			}
		}
		return defaultFormSpec(instance?.id ?? "forms");
	}, [instance, materialized]);
	const dispatchRaster = reactHostPort.useCallback(
		(document: RasterDocument) => {
			if (!instance || !store) return;
			store.dispatch({
				kind: "applyAppOperation",
				instanceId: instance.id,
				forwards: [{ op: "replaceProjection", projection: document }],
				backwards: [{ op: "replaceProjection", projection: rasterDoc }],
			});
		},
		[instance, store, rasterDoc],
	);
	const dispatchForms = reactHostPort.useCallback(
		(spec: FormSpec) => {
			if (!instance || !store) return;
			store.dispatch({
				kind: "applyAppOperation",
				instanceId: instance.id,
				forwards: [{ op: "replaceProjection", projection: spec }],
				backwards: [{ op: "replaceProjection", projection: formsSpec }],
			});
		},
		[instance, store, formsSpec],
	);
	const dispatchFixtureJson = reactHostPort.useCallback(
		(json: string) => {
			if (!instance || !store) return;
			let document: unknown;
			try {
				document = JSON.parse(json);
			} catch {
				return;
			}
			store.dispatch({
				kind: "applyAppOperation",
				instanceId: instance.id,
				forwards: [{ op: "setDocument", document }],
				backwards: [{ op: "setDocument", document: materialized }],
			});
		},
		[instance, store, materialized],
	);
	const dispatchInlineJson = reactHostPort.useCallback(
		(json: string) => {
			if (!instance || !store) return;
			store.dispatch({ kind: "patchAppSource", instanceId: instance.id, inline: json });
		},
		[instance, store],
	);
	const [rasterSelectedIds, setRasterSelectedIds] = reactHostPort.useState<readonly string[]>([]);
	const [rasterHoveredId, setRasterHoveredId] = reactHostPort.useState<string | null>(null);
	const puzzle5dModel = reactHostPort.useMemo(() => {
		if (materialized && typeof materialized === "object" && (materialized as Model).schema === "puzzle.5d") {
			return materialized as Model;
		}
		return (
			parseModel(materialized) ??
			parseModel({
				schema: "puzzle.5d",
				domain: "architecture",
				camera2d: { x: 0, y: 0, zoom: 1 },
				camera3d: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1 },
				parts: [],
				fasteners: [],
			})!
		);
	}, [materialized]);
	const dispatchPuzzle5dModel = reactHostPort.useCallback(
		(model: Model) => {
			if (!instance || !store) return;
			store.dispatch({
				kind: "patchAppSource",
				instanceId: instance.id,
				inline: JSON.stringify(model),
			});
		},
		[instance, store],
	);
	const writerDoc = reactHostPort.useMemo(() => {
		const doc = materialized as { text?: string } | null;
		return createWriterPlayDocument({ id: instance?.id ?? "writer", languageId: "jack", text: doc?.text ?? instance?.sourceDocument.inline ?? "" });
	}, [instance, materialized]);
	const noteDoc = reactHostPort.useMemo(() => {
		if (materialized && typeof materialized === "object" && (materialized as NoteDocument).schema === "note.document") {
			return materialized as NoteDocument;
		}
		if (instance?.sourceDocument.inline) {
			try {
				return noteDocumentFromJson(instance.sourceDocument.inline);
			} catch {
				return defaultNoteDocument(instance?.id ?? "note");
			}
		}
		return defaultNoteDocument(instance?.id ?? "note");
	}, [instance, materialized]);
	const dispatchNote = reactHostPort.useCallback(
		(document: NoteDocument) => {
			if (!instance || !store) return;
			store.dispatch({
				kind: "patchAppSource",
				instanceId: instance.id,
				inline: noteDocumentToJson(document),
			});
		},
		[instance, store],
	);
	const fixtureJson = reactHostPort.useMemo(() => JSON.stringify(materialized ?? {}), [materialized]);
	const layoutPageId = reactHostPort.useMemo(() => {
		try {
			const doc = JSON.parse(fixtureJson) as { pages?: readonly { id: string }[] };
			return doc.pages?.[0]?.id ?? "page-1";
		} catch {
			return "page-1";
		}
	}, [fixtureJson]);
	const vcsProjection = reactHostPort.useMemo(() => {
		if (materialized && typeof materialized === "object" && (materialized as { schema?: string }).schema === "vcs.demo/v1") {
			return materialized as { title: string; counter: number; notes?: string };
		}
		return { title: "VCS Demo", counter: 0, notes: "" };
	}, [materialized]);
	const dispatchVcsCounter = reactHostPort.useCallback(() => {
		if (!instance || !store) return;
		store.dispatch({
			kind: "applyAppOperation",
			instanceId: instance.id,
			forwards: [{ op: "setCounter", counter: vcsProjection.counter + 1 }],
			backwards: [{ op: "setCounter", counter: vcsProjection.counter }],
		});
	}, [instance, store, vcsProjection.counter]);
	const dispatchVcsCheckpoint = reactHostPort.useCallback(() => {
		store?.dispatch({ kind: "commitCheckpoint", message: "vcs" });
	}, [store]);
	const [jackEpoch, setJackEpoch] = reactHostPort.useState(0);
	const [jackQuery, setJackQuery] = reactHostPort.useState("MATCH (n) RETURN n");
	const hostChrome = (
		<SUpstreamBadge upstreamInstanceId={upstreamInstanceId} instances={projection?.appInstances ?? []} />
	);
	if (!instance || !resource) {
		return <div className="flex h-full items-center justify-center p-6 text-sm text-muted-foreground">No active app</div>;
	}
	if (instance.programId === "compose.sketchpad") {
		return (
			<div className="flex h-full min-h-0 flex-col overflow-hidden">
				{hostChrome}
				<div className="min-h-0 flex-1">
					<SSketchpadHost appId={instance.appId} />
				</div>
			</div>
		);
	}
	switch (resource.componentKind) {
		case "note":
			return (
				<div className="flex h-full min-h-0 flex-col overflow-hidden">
					{hostChrome}
					<NoteCanvas
						document={noteDoc}
						selectedIds={[]}
						hoveredId={null}
						kindHover={null}
						activeTool={noteDoc.activeTool}
						camera={noteDoc.camera}
						onCommit={(document) => dispatchNote(document)}
						onCameraChange={(camera) => dispatchNote({ ...noteDoc, camera })}
						className="min-h-0 flex-1"
					/>
				</div>
			);
		case "draw":
			return (
				<div className="flex h-full min-h-0 flex-col overflow-hidden">
					{hostChrome}
					<DrawCanvas document={drawDoc} onCommit={(document) => dispatchDraw(document)} className="min-h-0 flex-1" />
				</div>
			);
		case "writer":
			return (
				<div className="flex h-full min-h-0 flex-col overflow-hidden">
					{hostChrome}
					<WriterPlayCanvas
						document={writerDoc}
						onChange={(document) => {
							if (!store) return;
							store.dispatch({
								kind: "patchAppSource",
								instanceId: instance.id,
								inline: JSON.stringify(document),
							});
						}}
						createLspTransport={() => ({ dispose() {} } as never)}
						className="min-h-0 flex-1"
					/>
				</div>
			);
		case "raster":
			return (
				<div className="flex h-full min-h-0 flex-col overflow-hidden">
					{hostChrome}
					<RasterCanvas
						document={rasterDoc}
						selectedIds={rasterSelectedIds}
						hoveredId={rasterHoveredId}
						kindHover={null}
						activeTool={rasterDoc.activeTool}
						camera={rasterDoc.camera}
						onSelect={(ids) => setRasterSelectedIds(ids)}
						onHover={(id) => setRasterHoveredId(id)}
						onDocumentChange={(document) => dispatchRaster(document)}
						onCameraChange={(camera) => dispatchRaster({ ...rasterDoc, camera })}
						className="min-h-0 flex-1"
						viewMode="composite"
					/>
				</div>
			);
		case "forms":
			return (
				<div className="flex h-full min-h-0 flex-col overflow-hidden">
					{hostChrome}
					<FormEditSurface spec={formsSpec} onChange={(spec) => dispatchForms(spec)} className="min-h-0 flex-1 overflow-auto p-4" />
				</div>
			);
		case "cad":
			return (
				<div className="relative h-full min-h-0 overflow-hidden">
					{hostChrome}
					<reactHostPort.Suspense fallback={<div className="p-6 text-sm text-muted-foreground">Loading CAD…</div>}>
						<SCadPlayRoot />
					</reactHostPort.Suspense>
				</div>
			);
		case "flow":
			return (
				<div className="flex h-full min-h-0 flex-col overflow-hidden">
					{hostChrome}
					<FlowCanvas fixtureJson={fixtureJson} onFixtureChange={dispatchFixtureJson} className="min-h-0 flex-1" />
				</div>
			);
		case "dag":
			return (
				<div className="flex h-full min-h-0 flex-col overflow-hidden">
					{hostChrome}
					<DagCanvas fixtureJson={fixtureJson} onFixtureChange={dispatchFixtureJson} className="min-h-0 flex-1" reorganize />
				</div>
			);
		case "imperative":
			return (
				<div className="flex h-full min-h-0 flex-col overflow-hidden">
					{hostChrome}
					<ImperativeEditor documentJson={fixtureJson} onDocumentChange={dispatchInlineJson} className="min-h-0 flex-1" />
				</div>
			);
		case "sequence":
			return (
				<div className="flex h-full min-h-0 flex-col overflow-hidden">
					{hostChrome}
					<SequenceCanvas fixtureJson={fixtureJson} onFixtureChange={dispatchFixtureJson} className="min-h-0 flex-1" />
				</div>
			);
		case "layout":
			return (
				<div className="flex h-full min-h-0 flex-col overflow-hidden">
					{hostChrome}
					<div className="grid min-h-0 flex-1 grid-rows-[1fr_auto]">
						<LayoutCanvas documentJson={fixtureJson} pageId={layoutPageId} className="min-h-0" chromeMode="blueprint" />
						<textarea
							className="min-h-28 border-t bg-muted/20 p-3 font-mono text-xs"
							value={fixtureJson}
							spellCheck={false}
							onChange={(event) => dispatchFixtureJson(event.target.value)}
						/>
					</div>
				</div>
			);
		case "lowpoly":
			return (
				<div className="relative h-full min-h-0 overflow-hidden">
					{hostChrome}
					<SLowpolyHost fixtureJson={fixtureJson} onFixtureChange={dispatchInlineJson} />
				</div>
			);
		case "vcs":
			return (
				<div className="flex h-full min-h-0 flex-col overflow-hidden">
					{hostChrome}
					<SVcsHost projection={vcsProjection} onIncrement={dispatchVcsCounter} onCommitCheckpoint={dispatchVcsCheckpoint} />
				</div>
			);
		case "trinity":
			if (instance.appId === "jack") {
				return (
					<div className="flex h-full min-h-0 flex-col overflow-hidden">
						{hostChrome}
						<div className="flex items-center gap-2 border-b border-border/60 p-2">
							<WriterPlayCanvas
								document={createWriterPlayDocument({ id: instance.id, languageId: "jack", text: jackQuery })}
								onChange={(document) => setJackQuery(document.text)}
								createLspTransport={() => ({ dispose() {} } as never)}
								className="min-h-24 flex-1"
							/>
							<button type="button" className="rounded border px-2 py-1 text-xs" onClick={() => setJackEpoch((epoch) => epoch + 1)}>
								Run Jack
							</button>
						</div>
						<TrinityCanvas
							fixtureJson={fixtureJson}
							onFixtureChange={dispatchFixtureJson}
							jackDispatch={{ query: jackQuery, epoch: jackEpoch }}
							onJackDispatchComplete={dispatchFixtureJson}
							reorganize
							className="min-h-0 flex-1"
						/>
					</div>
				);
			}
			return (
				<div className="flex h-full min-h-0 flex-col overflow-hidden">
					{hostChrome}
					<TrinityCanvas fixtureJson={fixtureJson} onFixtureChange={dispatchFixtureJson} className="min-h-0 flex-1" reorganize />
				</div>
			);
		case "trinityRewrite":
			return (
				<div className="flex h-full min-h-0 flex-col overflow-hidden">
					{hostChrome}
					<STrinityRewriteHost fixtureJson={fixtureJson} onFixtureChange={dispatchFixtureJson} />
				</div>
			);
		case "catalogue":
			return (
				<div className="relative h-full min-h-0 overflow-hidden">
					{hostChrome}
					<SCatalogueHost bundle={materialized} />
				</div>
			);
		case "gismap":
			return (
				<div className="relative h-full min-h-0">
					{hostChrome}
					<SGisMapHost fixtureJson={fixtureJson} />
				</div>
			);
		case "puzzle2d":
			return (
				<div className="relative h-full min-h-0">
					{hostChrome}
					<SPuzzle2dHost fixtureJson={fixtureJson} onFixtureChange={dispatchFixtureJson} />
				</div>
			);
		case "puzzle3d":
			return (
				<div className="relative h-full min-h-0">
					{hostChrome}
					<SPuzzle3dHost fixtureJson={fixtureJson} onFixtureChange={dispatchFixtureJson} />
				</div>
			);
		case "presentation":
			return (
				<div className="flex h-full min-h-0 flex-col overflow-hidden">
					{hostChrome}
					<div className="grid min-h-0 flex-1 grid-rows-[1fr_auto]">
						<SPresentationDeckHost deck={(materialized as PresentationDeckDocument) ?? { schema: "presentation.deck", tiles: [] }} />
						<textarea
							className="min-h-28 border-t bg-muted/20 p-3 font-mono text-xs"
							value={fixtureJson}
							spellCheck={false}
							onChange={(event) => dispatchInlineJson(event.target.value)}
						/>
					</div>
				</div>
			);
		case "puzzle5d":
			return (
				<div className="relative h-full min-h-0">
					{hostChrome}
					<SPuzzle5dHost model={puzzle5dModel} instanceId={instance.id} onModelChange={dispatchPuzzle5dModel} />
				</div>
			);
		case "shooting":
			return (
				<div className="relative h-full min-h-0">
					{hostChrome}
					<ShootingModelCanvas
						fixture={JSON.parse(fixtureJson) as never}
						className="h-full"
						onCamera={(camera) =>
							store?.dispatch({
								kind: "applyAppOperation",
								instanceId: instance.id,
								forwards: [{ op: "setCamera", camera }],
								backwards: [{ op: "setCamera", camera: (materialized as { camera?: unknown })?.camera }],
							})
						}
					/>
				</div>
			);
		case "panel":
			if (materialized && typeof materialized === "object" && (materialized as PresentationDeckDocument).schema === "presentation.deck") {
				return (
					<div className="flex h-full min-h-0 flex-col overflow-hidden">
						{hostChrome}
						<SPresentationDeckHost deck={materialized as PresentationDeckDocument} />
					</div>
				);
			}
			return (
				<div className="h-full overflow-auto p-4 text-xs text-muted-foreground">
					<div className="mb-2 font-medium text-foreground">
						{resource.name} ({resource.componentKind})
					</div>
					<pre className="whitespace-pre-wrap">{fixtureJson}</pre>
				</div>
			);
		case "virtualFileSystem":
		case "s":
			return (
				<div className="h-full overflow-auto p-4 text-xs text-muted-foreground">
					<div className="mb-2 font-medium text-foreground">
						{resource.name} ({resource.componentKind})
					</div>
					<pre className="whitespace-pre-wrap">{fixtureJson}</pre>
				</div>
			);
		default:
			return (
				<div className="h-full overflow-auto p-4 text-xs text-muted-foreground">
					<div className="mb-2 font-medium text-foreground">
						{resource.name} ({resource.componentKind})
					</div>
					<pre className="whitespace-pre-wrap">{fixtureJson}</pre>
				</div>
			);
	}
}

function SAppHostSurfaceHost({ node: _node }: { readonly node: UiSHostSurfaceNode }): ReactElement {
	const ctrl = useSPlayController();
	const generation = ctrl?.getStudioStore().getGeneration() ?? 0;
	void generation;
	const instance = ctrl?.getActiveInstance() ?? null;
	return (
		<SAppHostSurface instance={instance}>
			<SAppHostRouter instance={instance} />
		</SAppHostSurface>
	);
}

function SSSurfaceHost({ node }: { readonly node: UiSHostSurfaceNode }): ReactElement {
	if (node.view === "mediaGraph") return <SMediaGraphSurfaceHost node={node} />;
	if (node.view === "appHost") return <SAppHostSurfaceHost node={node} />;
	if (node.view === "launcher") return <SLauncherSurfaceHost node={node} />;
	if (node.view === "history") return <SHistorySurfaceHost node={node} />;
	return <SMediaGraphSurfaceHost node={node} />;
}

function SPlayInner({ playground }: { readonly playground: Playground }): ReactElement {
	const ctrl = useSPlayController(playground.runtime);
	const bus = playground.runtime.commandBus;
	const focusedInstanceId = ctrl?.getFocusedInstanceId() ?? null;
	const detailTabs = reactHostPort.useMemo(
		() =>
			ctrl
				? [
						new SPlayInspectionPanelDefinition(() => buildSPlayInspectorTree(ctrl), bus).resolveTab(),
					]
				: [],
		[ctrl, bus],
	);
	const catalogueTabs = reactHostPort.useMemo(
		() => (ctrl ? [new SPlayCataloguePanelDefinition(() => buildSPlayCatalogueTree(), bus).resolveTab()] : []),
		[ctrl, bus],
	);
	const augmentPanelTabs = reactHostPort.useMemo(() => ({ details: detailTabs, workbench: catalogueTabs }), [detailTabs, catalogueTabs]);
	if (!ctrl) return <PlaygroundView runtime={playground.runtime} defaultAppId={S_PLAY_APP_ID} />;
	if (focusedInstanceId) {
		const instance = ctrl.getStudioStore().projection().appInstances.find((entry) => entry.id === focusedInstanceId) ?? null;
		return (
			<SStudioProvider store={ctrl.getStudioStore()}>
				<div className="flex h-full min-h-0 flex-col bg-background">
					<button
						type="button"
						className="border-b border-border px-3 py-2 text-left text-sm font-medium hover:bg-muted/50"
						onClick={() => ctrl.run("closeFocusedInstance")}
					>
						← Back to Media Graph · {instance?.label ?? focusedInstanceId}
					</button>
					<div className="min-h-0 flex-1">
						<SAppHostContent instance={instance} />
					</div>
				</div>
			</SStudioProvider>
		);
	}
	return (
		<SStudioProvider store={ctrl.getStudioStore()}>
			<PlaygroundView runtime={playground.runtime} defaultAppId={S_PLAY_APP_ID} augmentPanelTabs={augmentPanelTabs} />
		</SStudioProvider>
	);
}

class SPlayCataloguePanelDefinition extends PureSidePanelTabDefinition {
	constructor(
		private readonly buildTree: () => UiTreeNode,
		private readonly commandBus: CommandBus,
	) {
		super();
	}

	buildTab(): SidePanelTabConfig {
		return {
			id: "s-play-catalogue",
			icon: createIconComponent(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID),
			name: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
			order: 1,
			tree: new CallbackTreePanelDefinition(() => uiTreeNodeToTreePanelConfig(this.buildTree(), this.commandBus)),
		};
	}
}

class SPlayInspectionPanelDefinition extends PureSidePanelTabDefinition {
	constructor(
		private readonly buildTree: () => UiTreeNode,
		private readonly commandBus: CommandBus,
	) {
		super();
	}

	buildTab(): SidePanelTabConfig {
		return {
			id: "s-play-inspector",
			icon: createIconComponent(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID),
			name: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
			order: 0,
			tree: new CallbackTreePanelDefinition(() => uiTreeNodeToTreePanelConfig(this.buildTree(), this.commandBus)),
		};
	}
}

function SPlayJackSurfaceHost({ node: _node }: { readonly node: import("@semio-tech/framework-platform-core").UiWriterHostSurfaceNode }): ReactElement {
	const ctrl = useSPlayController();
	void ctrl?.getHoverEpoch();
	void ctrl?.getSelectEpoch();
	const document = ctrl?.getWriterDocumentJack() ?? createWriterDocument({ id: "s-jack", languageId: "jack", text: "" });
	const onHoverChange = reactHostPort.useCallback((offset: number | null) => {
		sPlayControllerRef.current?.run("setJackHover", { offset });
	}, []);
	const onSelectionChange = reactHostPort.useCallback((range: { start: number; end: number }) => {
		sPlayControllerRef.current?.run("setJackSelect", range);
	}, []);
	return (
		<WriterCanvas
			document={document}
			className="h-full"
			onHoverChange={onHoverChange}
			onSelectionChange={onSelectionChange}
			externalHoverOccurrences={ctrl?.getJackHoverOccurrences()}
			externalHoverOccurrencesSignal={ctrl?.getHoverEpoch()}
			externalSelectionOccurrences={ctrl?.getJackSelectOccurrences()}
			externalSelectionOccurrencesSignal={ctrl?.getSelectEpoch()}
		/>
	);
}

function SPlayCompiledDagSurfaceHost({ node: _node }: { readonly node: import("@semio-tech/framework-platform-core").UiWriterHostSurfaceNode }): ReactElement {
	const ctrl = useSPlayController();
	const [revision, setRevision] = reactHostPort.useState(0);
	reactHostPort.useEffect(() => ctrl?.subscribeSnapshot(() => setRevision((value) => value + 1)) ?? undefined, [ctrl]);
	const document = reactHostPort.useMemo(
		() => ctrl?.getWriterDocumentCompiledDag() ?? createWriterDocument({ id: "s-compiled-dag", languageId: "wire", text: "" }),
		[ctrl, revision],
	);
	return <WriterCanvas document={document} className="h-full min-h-0" />;
}

export function registerSPlaySurfaceHosts(): void {
	if (sPlayChromeRegistered) return;
	sPlayChromeRegistered = true;
	registerUiSSurfaceHost(S_PLAY_SURFACE_MEDIA_GRAPH, SSSurfaceHost);
	registerUiSSurfaceHost(S_PLAY_SURFACE_APP_HOST, SSSurfaceHost);
	registerUiSSurfaceHost(S_PLAY_SURFACE_LAUNCHER, SSSurfaceHost);
	registerUiSSurfaceHost(S_PLAY_SURFACE_HISTORY, SSSurfaceHost);
	registerUiWriterSurfaceHost(S_PLAY_SURFACE_JACK, SPlayJackSurfaceHost);
	registerUiWriterSurfaceHost(S_PLAY_SURFACE_COMPILED_DAG, SPlayCompiledDagSurfaceHost);
	registerSPlayDeclarativeBodies();
	registerDrawPlaySurfaceHosts();
	registerWriterPlaySurfaceHosts();
	registerRasterPlaySurfaceHosts();
	registerFlowPlaySurfaceHosts();
	registerDagPlaySurfaceHosts();
	registerMapPlaySurfaceHosts();
	registerPuzzle2dPlaySurfaceHosts();
	registerPuzzle3dPlaySurfaceHosts();
	registerPuzzle5dPlaySurfaceHosts();
	registerTrinityJackPlaySurfaceHosts();
	registerTrinityRewritePlaySurfaceHosts();
	registerProceduralPlaySurfaceHosts();
	registerProcedural2dPlaySurfaceHosts();
	registerShootingPlaySurfaceHosts();
	registerFormsPlaySurfaceHosts();
	registerPresentationPlaySurfaceHosts();
	void import("@semio-tech/cad-js-renderer-react").then((module) => module.registerCadPlaySurfaceHosts());
}

function SPlayChrome({ playground }: { readonly playground: Playground }): ReactElement {
	return <SPlayInner playground={playground} />;
}

export function mountSPlayChrome(playground: Playground, rootId = "root"): void {
	mountPlaygroundApp(<SPlayChrome playground={playground} />, rootId);
}

const sPlayChromeBoot: PlaygroundChromeBoot = {
	registerHosts: registerSPlaySurfaceHosts,
	mount: mountSPlayChrome,
};

export function bootSPlay(playground: Playground, rootId = "root"): void {
	bootPlayground(playground, sPlayChromeBoot, rootId);
}
//#endregion 🔖SPlayHost

//#region 🔖Boot

/** @emoji 🧩 Play package supplies host registration + React mount (one puzzle surface per boot). */
export interface PlaygroundChromeBoot {
  registerHosts(): void;
  mount(playground: Playground, rootId?: string): void;
}

/** @emoji 🛝 Registers hosts, declarative bodies, and mounts play chrome synchronously. */
export function bootPlayground(playground: Playground, boot: PlaygroundChromeBoot, rootId = "root"): void {
  boot.registerHosts();
  playground.registerBodies();
  playground.registerSurfaceHosts();
  boot.mount(playground, rootId);
}
//#endregion 🔖Boot

//#region 🧪Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("playground renderer slices", () => {
    it("keeps cross-dimensional brush host imports with their consumers", async () => {
      const { readFileSync } = await import("node:fs");
      const source = readFileSync("index.tsx", "utf8");
      const hostRegion = (kind: "2d" | "5d") => {
        const start = `//#region 🔖Puzzle${kind}PlayHost`;
        return source.slice(source.indexOf(start), source.indexOf(`//#endregion 🔖Puzzle${kind}PlayHost`));
      };
      const puzzle2d = hostRegion("2d");
      const puzzle5d = hostRegion("5d");
      expect(puzzle2d).toMatch(
        /import\s*\{[^}]*puzzle2dSetBrushPlaceCommitHandler[^}]*\}\s*from\s*["']@semio-tech\/puzzle-2d-react["']/,
      );
      expect(puzzle5d).toMatch(
        /import\s*\{[^}]*installPuzzle3dPlayBrushHost[^}]*\}\s*from\s*["']@semio-tech\/puzzle-3d-play["']/,
      );
      expect(puzzle5d).toMatch(
        /import\s*\{[^}]*puzzle2dSetBrushPlaceCommitHandler[^}]*\}\s*from\s*["']@semio-tech\/puzzle-2d-react["']/,
      );
    });
  });

  describe("PlaygroundView shell notify", () => {
    it("panel visibility uses chrome generation without bumping data generation", () => {
      const runtime = new Platform({ id: "p", name: "P" });
      const dataGen = runtime.generation;
      runtime.setPanelVisibility({ leftSidePanel: true, rightSidePanel: false });
      expect(runtime.generation).toBe(dataGen);
      expect(runtime.chromeGeneration).toBeGreaterThan(0);
    });

    it("keeps panel toggles right-aligned when no navbar center slot is present", async () => {
      const { renderToStaticMarkup } = await import("react-dom/server");
      const { AppRuntime, Controller, createTabStackLayout, registerWindowBody, buildPanelWindowBody } = await import("@semio-tech/framework-playground-core");
      const runtime = new Platform({ initialPanelVisibility: { leftSidePanel: true, rightSidePanel: true } });
      class TestController extends Controller {
        constructor() {
          super("playground-navbar-align-test", runtime.commandBus, () => runtime.notify());
        }
        run(): void {}
      }
      const app = new AppRuntime("playground-navbar-align-test", "Navbar Align Test", undefined, new TestController(), createTabStackLayout(["main"], ["Main"]), [
        new WindowKindRuntime("main", "Main", "playground.navbar.align.main"),
      ]);
      registerWindowBody("playground.navbar.align.main", () => buildPanelWindowBody("playground.navbar.align", "playground-navbar-align-test"));
      runtime.addApp(app);
      const markup = renderToStaticMarkup(<PlaygroundView runtime={runtime} defaultAppId="playground-navbar-align-test" />);
      expect(markup).toContain('data-slot="app-name"');
      expect(markup).toContain(">Navbar Align Test</span>");
      expect(markup).toContain('id="playground.navbar.modes.edit"');
      expect(markup).toContain(">Edit</span>");
      expect(markup).toContain('data-slot="app-panel-toggle-group"');
      expect(markup).toContain("flex-1 min-w-0");
      expect(markup.indexOf("flex-1 min-w-0")).toBeLessThan(markup.indexOf('data-slot="app-panel-toggle-group"'));
    });

    it("renders display panel toggle with layout-grid icon when app has window kinds", async () => {
      const { renderToStaticMarkup } = await import("react-dom/server");
      const { AppRuntime, Controller, createTabStackLayout, registerWindowBody, registerSidePanelBody, buildPanelWindowBody } = await import("@semio-tech/framework-playground-core");
      const runtime = new Platform({ initialPanelVisibility: { leftSidePanel: true, rightSidePanel: true } });
      class TestController extends Controller {
        constructor() {
          super("playground-view-test", runtime.commandBus, () => runtime.notify());
        }
        run(): void {}
      }
      const app = new AppRuntime("playground-view-test", "Playground View Test", undefined, new TestController(), createTabStackLayout(["main"], ["Main"]), [
        new WindowKindRuntime("main", "Main", "playground.view.test.main"),
      ]);
      app.panelTabs = [
        { id: "workbench", iconId: "folder", panel: "workbench", order: 0, bodyKey: "playground.view.test.workbench", label: "Workbench" },
        { id: "details", iconId: "info", panel: "details", order: 0, bodyKey: "playground.view.test.details", label: "Details" },
      ];
      registerWindowBody("playground.view.test.main", () => buildPanelWindowBody("playground.view.test", "playground-view-test"));
      registerSidePanelBody("playground.view.test.workbench", () => ({ type: "tree", sections: [{ id: "workbench", items: [{ id: "item", label: "Workbench" }] }] }));
      registerSidePanelBody("playground.view.test.details", () => ({ type: "tree", sections: [{ id: "details", items: [{ id: "item", label: "Details" }] }] }));
      runtime.addApp(app);
      const markup = renderToStaticMarkup(
        <PlaygroundView runtime={runtime} defaultAppId="playground-view-test" initialPanelVisibility={{ leftSidePanel: true, rightSidePanel: true }} />,
      );
      expect(markup).toContain('id="ui.panelToggle.display"');
      expect(markup).not.toContain("data-missing-icon");
      expect(markup).toContain('data-icon="layout-grid"');
    });
  });

  describe("Toolbar tree", () => {
    it("detects interactive toolbar nodes and omits separator-only groups", () => {
      expect(
        hasToolbarViewTools([
          { id: "save", kind: "collection", icon: null, children: [{ id: "save.selected", label: "Selected" }] },
        ]),
      ).toBe(true);
      expect(
        hasToolbarViewTools([{ id: "filter", kind: "collection", icon: null, children: [{ id: "sep", kind: "separator" }] }]),
      ).toBe(false);
    });
  });

  describe("windowEngagementToGolden", () => {
    it("returns undefined when there is no engagement", () => {
      expect(windowEngagementToGolden(undefined, new CommandBus())).toBeUndefined();
    });

    it("converts neutral engagement and dispatches option/input commands on the bus", () => {
      const bus = new CommandBus();
      const dispatched: { controllerId: string; command: string; args?: unknown }[] = [];
      bus.register({
        id: "ctrl",
        commandBus: bus,
        dispose() {},
        run(command: string, args?: unknown) {
          dispatched.push({ controllerId: "ctrl", command, args });
        },
      } as never);
      const spec = windowEngagementToGolden(
        {
          options: [{ id: "opt", label: "Confirm", command: { controllerId: "ctrl", command: "confirm" } }],
          input: { id: "in", value: "x", placeholder: "type", onSubmit: { controllerId: "ctrl", command: "submit" } },
          status: [{ id: "st", text: "Ready" }],
        },
        bus,
      );
      expect(spec?.options?.[0]?.label).toBe("Confirm");
      expect(spec?.input?.value).toBe("x");
      expect(spec?.status?.[0]?.content).toBe("Ready");
      spec?.options?.[0]?.onPress?.();
      spec?.input?.onSubmit?.("hello");
      expect(dispatched).toEqual([
        { controllerId: "ctrl", command: "confirm", args: undefined },
        { controllerId: "ctrl", command: "submit", args: { value: "hello" } },
      ]);
    });

    it("converts possible engagements and dispatches their select command", () => {
      const bus = new CommandBus();
      const dispatched: { command: string; args?: unknown }[] = [];
      bus.register({
        id: "ctrl",
        commandBus: bus,
        dispose() {},
        run(command: string, args?: unknown) {
          dispatched.push({ command, args });
        },
      } as never);
      const spec = windowEngagementToGolden(
        {
          possibleEngagements: [{ id: "primitive.box", label: "Box", detail: "b", command: { controllerId: "ctrl", command: "start" } }],
        },
        bus,
      );
      spec?.possibleEngagements?.[0]?.onSelect?.();
      expect(dispatched).toEqual([{ command: "start", args: undefined }]);
    });

    it("maps engagement control commands through the bus", () => {
      const bus = new CommandBus();
      const dispatched: { command: string; args?: unknown }[] = [];
      bus.register({
        id: "ctrl",
        commandBus: bus,
        dispose() {},
        run(command: string, args?: unknown) {
          dispatched.push({ command, args });
        },
      } as never);
      const spec = windowEngagementToGolden(
        {
          input: { id: "engagement-input", onChange: { controllerId: "ctrl", command: "engagementInput" } },
          control: {
            kind: "stepper",
            value: 2,
            min: 0,
            step: 0.1,
            onChange: { controllerId: "ctrl", command: "engagementControlChange" },
            onCommit: { controllerId: "ctrl", command: "engagementControlCommit" },
          },
        },
        bus,
      );
      spec?.control?.kind === "stepper" && spec.control.onChange?.(3);
      spec?.control?.kind === "stepper" && spec.control.onCommit?.(3);
      expect(dispatched).toEqual([
        { command: "engagementControlChange", args: { value: 3 } },
        { command: "engagementControlCommit", args: { value: 3 } },
      ]);
    });

    it("threads engagement through windowKindsToGolden", () => {
      const wk = new WindowKindRuntime("w", "W", "body", undefined, [], {
        input: { id: "engagement-input", onChange: { controllerId: "ctrl", command: "engagementInput" } },
        status: [{ id: "s", text: "ready" }],
      });
      const golden = windowKindsToGolden([wk], new CommandBus());
      expect(golden[0]?.engagement?.status?.[0]?.content).toBe("ready");
    });
  });

  describe("CallbackTreePanelDefinition", () => {
    it("accepts a full TreePanelConfig from the builder", () => {
      const panel = new CallbackTreePanelDefinition(() => ({
        sections: [{ id: "a", items: [{ id: "i", label: "Item" }] }],
        selectedIds: ["i"],
      }));
      expect(panel.resolveTree().selectedIds).toEqual(["i"]);
    });

    it("reuses resolved config when section content is unchanged", () => {
      const panel = new CallbackTreePanelDefinition(() => [{ id: "a", items: [{ id: "i", label: "Item" }] }]);
      const first = panel.resolveTree();
      const second = panel.resolveTree();
      expect(second).toBe(first);
    });

    it("refreshes resolved config when nested item order changes", () => {
      let nestedIds = ["q-a", "q-b"];
      const panel = new CallbackTreePanelDefinition(() => [
        {
          id: "forms",
          items: [{ id: "step:one", label: "Step", items: nestedIds.map((id) => ({ id, label: id })) }],
        },
      ]);
      const first = panel.resolveTree();
      nestedIds = ["q-b", "q-a"];
      const second = panel.resolveTree();
      expect(second).not.toBe(first);
      expect(second.sections[0]?.items[0]?.items?.map((item) => item.id)).toEqual(["q-b", "q-a"]);
    });
  });

  describe("enforcePlaygroundTreePanel", () => {
    it("rejects sections without items", () => {
      expect(() =>
        enforcePlaygroundTreePanel({
          sections: [{ id: "a" }],
        }),
      ).toThrow(/at least one item/);
    });

    it("accepts sections with items", () => {
      expect(() =>
        enforcePlaygroundTreePanel({
          sections: [{ id: "a", items: [{ id: "i", label: "Item" }] }],
        }),
      ).not.toThrow();
    });

    it("rejects React element descriptions on tree items", () => {
      expect(() =>
        enforcePlaygroundTreePanel({
          sections: [{ id: "a", items: [{ id: "i", label: "Item", description: <span>panel</span> }] }],
        }),
      ).toThrow(/React description/);
    });
  });

  describe("puzzle 2d play cameras", () => {
    it("imports puzzle 2d camera zoom limits used by host clamping", async () => {
      const { PUZZLE_2D_CAMERA_ZOOM_MIN, PUZZLE_2D_CAMERA_ZOOM_MAX } = await import("@semio-tech/puzzle-2d-react");
      expect(PUZZLE_2D_CAMERA_ZOOM_MIN).toBeGreaterThan(0);
      expect(PUZZLE_2D_CAMERA_ZOOM_MAX).toBeGreaterThan(PUZZLE_2D_CAMERA_ZOOM_MIN);
    });
  });

  describe("UiRenderer host surfaces", () => {
    it("renders cad nodes through platform surface bindings", async () => {
      const { renderToStaticMarkup } = await import("react-dom/server");
      const surfaceId = "playground.test/cad";
      function TestCadHost(): React.ReactElement {
        return <div data-host="cad">cad canvas</div>;
      }
      registerSurfaceBinding(surfaceId, TestCadHost);
      try {
        const html = renderToStaticMarkup(<UiRenderer node={buildCadWindowBody(surfaceId, "ctrl")} commandBus={new CommandBus()} />);
        expect(html).toContain('data-host="cad"');
        expect(html).not.toContain("Unsupported UiNode");
      } finally {
        unregisterSurfaceBinding(surfaceId);
      }
    });

    it("renders declarative tree nodes", async () => {
      const { renderToStaticMarkup } = await import("react-dom/server");
      const bus = new CommandBus();
      const html = renderToStaticMarkup(
        <UiRenderer
          commandBus={bus}
          node={{
            type: "tree",
            sections: [
              {
                id: "root",
                defaultOpen: true,
                items: [{ id: "scene", label: "Scene", defaultOpen: true, items: [{ id: "obj.a", label: "Alpha" }] }],
              },
            ],
          }}
        />,
      );
      expect(html).toContain("Scene");
      expect(html).toContain("Alpha");
      expect(html).not.toContain("Unsupported UiNode");
    });

    it("renders declarative tree nodes with selectedIds overlay", async () => {
      const { renderToStaticMarkup } = await import("react-dom/server");
      const bus = new CommandBus();
      const html = renderToStaticMarkup(
        <UiRenderer
          commandBus={bus}
          node={{
            type: "tree",
            selectedIds: ["obj.a"],
            sections: [
              {
                id: "root",
                defaultOpen: true,
                items: [{ id: "scene", label: "Scene", defaultOpen: true, items: [{ id: "obj.a", label: "Alpha" }] }],
              },
            ],
          }}
        />,
      );
      expect(html).toContain("Alpha");
      expect(html).not.toContain("Unsupported UiNode");
    });

    it("renders puzzle2d nodes through platform surface bindings", async () => {
      const { renderToStaticMarkup } = await import("react-dom/server");
      const { buildPuzzle2dWindowBody } = await import("@semio-tech/framework-playground-core");
      const surfaceId = "playground.test/puzzle2d";
      function TestPuzzle2dHost(): React.ReactElement {
        return <div data-host="puzzle2d">puzzle 2d canvas</div>;
      }
      registerSurfaceBinding(surfaceId, TestPuzzle2dHost);
      try {
        const html = renderToStaticMarkup(
          <UiRenderer node={buildPuzzle2dWindowBody(surfaceId, "ctrl", "pane")} commandBus={new CommandBus()} />,
        );
        expect(html).toContain('data-host="puzzle2d"');
        expect(html).not.toContain("Unsupported UiNode");
      } finally {
        unregisterSurfaceBinding(surfaceId);
      }
    });

    it("renders gismap nodes through GIS map surface hosts", async () => {
      const { renderToStaticMarkup } = await import("react-dom/server");
      const { buildMapWindowBody } = await import("@semio-tech/framework-playground-core");
      const surfaceId = "playground.test/gismap";
      function TestGisMapHost(): React.ReactElement {
        return <div data-host="gismap">gis map canvas</div>;
      }
      registerUiGisMapSurfaceHost(surfaceId, TestGisMapHost);
      try {
        const html = renderToStaticMarkup(<UiRenderer node={buildMapWindowBody(surfaceId, "ctrl", "main")} commandBus={new CommandBus()} />);
        expect(html).toContain('data-host="gismap"');
        expect(html).not.toContain("Unsupported UiNode");
      } finally {
        gisMapSurfaceHosts.delete(surfaceId);
        unregisterSurfaceBinding(surfaceId);
      }
    });

    it("renders flow nodes through flow surface hosts", async () => {
      const { renderToStaticMarkup } = await import("react-dom/server");
      const { buildFlowWindowBody } = await import("@semio-tech/framework-playground-core");
      const surfaceId = "playground.test/flow";
      function TestFlowHost(): React.ReactElement {
        return <div data-host="flow">flow canvas</div>;
      }
      registerUiFlowSurfaceHost(surfaceId, TestFlowHost);
      try {
        const html = renderToStaticMarkup(<UiRenderer node={buildFlowWindowBody(surfaceId, "ctrl", "main")} commandBus={new CommandBus()} />);
        expect(html).toContain('data-host="flow"');
        expect(html).not.toContain("Unsupported UiNode");
      } finally {
        flowSurfaceHosts.delete(surfaceId);
        unregisterSurfaceBinding(surfaceId);
      }
    });

    it("renders shooting nodes through shooting surface hosts", async () => {
      const { renderToStaticMarkup } = await import("react-dom/server");
      const { buildShootingWindowBody } = await import("@semio-tech/framework-playground-core");
      const surfaceId = "playground.test/shooting-model";
      function TestShootingHost(): React.ReactElement {
        return <div data-host="shooting">shooting canvas</div>;
      }
      registerSurfaceBinding(surfaceId, TestShootingHost);
      try {
        const html = renderToStaticMarkup(
          <UiRenderer node={buildShootingWindowBody(surfaceId, "ctrl", "model")} commandBus={new CommandBus()} />,
        );
        expect(html).toContain('data-host="shooting"');
        expect(html).not.toContain("Unsupported UiNode");
      } finally {
        unregisterSurfaceBinding(surfaceId);
      }
    });

    it("renders raster nodes through raster surface hosts", async () => {
      const { renderToStaticMarkup } = await import("react-dom/server");
      const { buildRasterWindowBody } = await import("@semio-tech/framework-playground-core");
      const surfaceId = "playground.test/raster-composite";
      function TestRasterHost(): React.ReactElement {
        return <div data-host="raster">raster canvas</div>;
      }
      registerUiRasterSurfaceHost(surfaceId, TestRasterHost);
      try {
        const html = renderToStaticMarkup(
          <UiRenderer node={buildRasterWindowBody(surfaceId, "ctrl", "composite", "composite")} commandBus={new CommandBus()} />,
        );
        expect(html).toContain('data-host="raster"');
        expect(html).not.toContain("Unsupported UiNode");
      } finally {
        rasterSurfaceHosts.delete(surfaceId);
        unregisterSurfaceBinding(surfaceId);
      }
    });
  });

}
