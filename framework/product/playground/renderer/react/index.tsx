// #region 🧲Header
/** @emoji 🛝 Playground shell renderer: {@link PlaygroundView}, tree panels, puzzle play hosts, and surface hosts. */
// #endregion 🧲Header

// #region 🔌Adapters
import {
  Input,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  Slider,
  Toggle,
  Tree,
  cn,
  getLevelBgClass,
  Label,
  engagementCommandTokenEquals,
  normalizeEngagementCommandText,
  LevelProvider,
  staticTreePanelDefinition,
  useCommandHotkey,
  bootstrapElementsSurfaceChromeDocument,
  useElementsSurfaceChrome,
  useMediaQuery,
  type EngagementControl,
  type EngagementSpec,
  type FooterItem,
  type NavbarItem,
  type SidePanelTabConfig,
  type SidePanelTabDefinition,
  type TreeDataItem,
  type TreeDataSection,
  type TreeDragAndDropController,
  type TreePanelConfig,
  type TreePanelDefinition,
  type TreePanelSource,
  reactHostPort,
  Button,
  Icon,
  IconSelector,
  createIconComponent,
  Ring,
  type ContextMenuItem,
  type UiTranslationKey,
  NavbarFixtureSelect,
  readStoredUiChromeCompact,
  readStoredUiChromeExpertise,
  writeStoredUiChromeCompact,
  writeStoredUiChromeExpertise,
} from "@ui/react";
import { clsx, type ClassValue } from "clsx";

//#region 🪁I18n Compile Gate
const _playgroundCadToolbarI18nKeys = [
  "ui.toolbar.parent.save",
  "ui.toolbar.parent.view",
  "ui.toolbar.parent.transform",
  "ui.toolbar.parent.transfer",
] as const satisfies readonly UiTranslationKey[];
//#endregion 🪁I18n Compile Gate
import * as React from "react";
import type { ReactElement } from "react";
import { createRoot, type Root } from "react-dom/client";
import { twMerge } from "tailwind-merge";
import {
  APP_TOOL_CATEGORY_ORDER,
  CommandBus,
  Expertise,
  Platform,
  resolveInitialPanelVisibility,
  WindowKindRuntime,
  getSidePanelBodyFactory,
  getWindowBodyFactory,
  registerWindowBody,
  buildCadWindowBody,
  type AppToolCategory,
  type AppTools,
  type CommandDescriptor,
  type Playground,
  type PlaygroundKeybinding,
  type ResolvedAppState,
  type SidePanelBodyViewContext,
  type SideTabSpec,
  type UiPuzzle2dHostSurfaceNode,
  type UiFieldNode,
  type UiInputNode,
  type UiKeyValueNode,
  type UiNode,
  type UiSectionNode,
  type UiSelectNode,
  type UiToggleNode,
  type UiTreeItemNode,
  type UiTreeNode,
  type UiTreeSectionNode,
  type UiVec3Node,
  collectUiTreeItemDragData,
  type UiPuzzle3dHostSurfaceNode,
  type UiTableHostSurfaceNode,
  enforcePlaygroundWindowEngagementInput,
  enforceWindowKindsEngagementInput,
  isPlaygroundNoFixtureId,
  PLAYGROUND_NO_FIXTURE_ID,
  resolvePlaygroundFixtureCatalog,
  type PlaygroundFixtureCatalog,
  type WindowBodyViewContext,
  type WindowEngagement,
  type WindowEngagementControl,
  type WindowLayout,
  type WindowMeasure,
} from "@framework/playground/core";
// #endregion 🔌Adapters

export type {
  AppRuntime,
  AppTools,
  CommandBus,
  Controller,
  ModeRuntime,
  FooterItem as PlaygroundDeclarativeFooterItem,
  Platform,
  ResolvedAppState,
  SidePanelBodyViewContext,
  SideTabSpec,
  ToolItem,
  UiNode,
  WindowBodyViewContext,
  WindowKindRuntime,
  WindowLayout,
} from "@framework/playground/core";

export type { PlaygroundFixtureCatalog, PlaygroundFixtureHost, PlaygroundFixtureOption } from "@framework/playground/core";
export {
  isPlaygroundNoFixtureId,
  PLAYGROUND_NO_FIXTURE_ID,
  PLAYGROUND_NO_FIXTURE_OPTION,
  playgroundFixtureCatalogWithNoOption,
  resolvePlaygroundFixtureCatalog,
} from "@framework/playground/core";

export {
  APP_TOOL_CATEGORY_ORDER,
  AppRuntime,
  CommandBus,
  ModeRuntime,
  PlaygroundController,
  Platform,
  WindowKindRuntime,
  buildPuzzle3dWindowBody,
  createDefaultLayout,
  createStackLayout,
  createWindowLayout,
  getSidePanelBodyFactory,
  getWindowBodyFactory,
  registerSidePanelBody,
  registerWindowBody,
  resolveAppState,
  playgroundTreePanelRootItems,
  buildCadWindowBody,
} from "@framework/playground/core";
import {
  ProductShell,
  createBrowserStoragePort,
  createFrameworkDisplayPanelTabs,
  createFrameworkSettingsPanelTabs,
  sideTabsToPanelTabs,
  uiTreeNodeToTreePanelConfig,
  renderUiControl,
  resolveDeclarativeControlIcon,
  declareToolsToViewTools,
  DisplayHostContext,
  SettingsHostContext,
  findDefaultActiveWindowKindId,
  listPopulatedToolbarViewCategories,
  mergePlatformFooterChromeRows,
  registerSurfaceBinding,
  renderComponentHostSurface,
  unregisterSurfaceBinding,
  registerUiPanelSurfaceHost,
  UIToolbar,
  useControllerStore,
  shellWindowScopeId,
  useShellWindowInstance,
  useStore,
  windowMeasuresToGolden,
  shellTabIconComponent,
  type DisplayHostApi,
  type SettingsHostApi,
  type UiComponentHostSurfaceNode,
  type UIWindowMeasure,
} from "@framework/platform/renderer/react";
import { NamedLayoutStore } from "@framework/core";

export { useControllerStore, useStore } from "@framework/platform/renderer/react";
export type { Store } from "@framework/playground/core";

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

/** @emoji 🌲 Tree panel that rebuilds when the builder returns sections or a full {@link TreePanelConfig}. */
export class CallbackTreePanelDefinition implements TreePanelDefinition {
  private resolved: TreePanelConfig | null = null;
  private resolvedSections: TreeDataSection[] | null = null;
  private resolvedHighlightedIds: readonly string[] | null = null;

  constructor(
    private readonly buildTree: () => TreeDataSection[] | TreePanelConfig,
    private readonly buildHighlightedIds: () => readonly string[] = () => [],
  ) {}

  resolveTree(): TreePanelConfig {
    const built = this.buildTree();
    const sections = Array.isArray(built) ? built : built.sections;
    const extraHighlightedIds = this.buildHighlightedIds();
    const highlightedIds =
      extraHighlightedIds.length > 0
        ? extraHighlightedIds
        : Array.isArray(built)
          ? extraHighlightedIds
          : built.highlightedIds;
    if (this.resolved && this.resolvedSections === sections && this.resolvedHighlightedIds === highlightedIds) {
      return this.resolved;
    }
    const config: TreePanelConfig = Array.isArray(built)
      ? { sections, highlightedIds }
      : { ...built, sections, highlightedIds };
    enforcePlaygroundTreePanel(config);
    this.resolved = config;
    this.resolvedSections = sections;
    this.resolvedHighlightedIds = highlightedIds;
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
type GisMapSurfaceHost = React.ComponentType<{ readonly node: import("@framework/platform/core").UiGisMapHostSurfaceNode }>;
const gisMapSurfaceHosts = new Map<string, GisMapSurfaceHost>();
const tableSurfaceHosts = new Map<string, TableSurfaceHost>();

const PLAYGROUND_CANVAS_HOST_TYPES = new Set(["puzzle2d", "puzzle3d", "puzzle5d", "cad", "gismap"]);

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

/** @emoji 📊 Binds `surfaceId` from {@link UiTableHostSurfaceNode} to a host table body. */
export function registerUiTableSurfaceHost(surfaceId: string, Component: TableSurfaceHost): void {
  tableSurfaceHosts.set(surfaceId, Component);
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
  if (node.type === "table") {
    const Host = tableSurfaceHosts.get(node.surfaceId);
    if (Host) {
      return (
        <div className="relative min-h-0 min-w-0 flex-1 overflow-auto">
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
    node.type === "panel" ||
    node.type === "table"
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
    const legacyActivate = (item as UiTreeItemNode & { readonly onClick?: () => void }).onClick;
    return {
      id: item.id,
      label: item.label,
      description: item.description,
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
      onClick:
        legacyActivate ??
        (item.command
          ? () => {
              dispatchUiCommand(commandBus, item.command!, {});
            }
          : undefined),
      onPointerEnter: item.onPointerEnter,
      onPointerLeave: item.onPointerLeave,
    };
  });
}


function buildUiTreeDragAndDropController(sections: readonly UiTreeSectionNode[], commandBus: CommandBus): TreeDragAndDropController | undefined {
  void commandBus;
  const dragByItemId = collectUiTreeItemDragData(sections);
  if (dragByItemId.size === 0) {
    return undefined;
  }
  const sample = dragByItemId.values().next().value;
  if (sample && PUZZLE_2D_FIXTURE_DRAG_V1_MIME in sample) {
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
        <button type="button" className="rounded-md border border-border bg-background px-2 py-1 text-sm" onClick={() => commandBus.dispatch(node.command.controllerId, node.command.command, node.command.args)}>
          {node.label}
        </button>
      );
    case "separator":
      return <span role="separator" className="bg-border my-1 h-px w-full shrink-0" aria-hidden />;
    case "puzzle2d":
    case "puzzle3d":
    case "puzzle5d":
    case "cad":
    case "gismap":
    case "panel":
    case "table":
      return renderPlaygroundHostSurface(node, node.type === "table" || node.type === "panel" ? "panel" : "canvas");
    case "section": {
      const section = node as UiSectionNode;
      return (
        <div className="border-normal/60 flex flex-col gap-single rounded-md border p-single" data-ui-section={section.id}>
          {section.label ? <div className="text-muted-foreground text-[10px] font-semibold uppercase tracking-wide">{section.label}</div> : null}
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
          <label className="text-muted-foreground text-[11px]" htmlFor={field.child.type === "input" || field.child.type === "select" ? (field.child as UiInputNode | UiSelectNode).id : field.id}>
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
import { registerIcon, registerTabIcon } from "@framework/platform/renderer/react";
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
        <div className="relative flex h-full min-h-0 min-w-0 flex-1 flex-col overflow-hidden">
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
    onChange: control.onChange ? { controllerId, command: "engagementControlChange", args: { ...commandArgs } } : undefined,
    onCommit: control.onCommit ? { controllerId, command: "engagementControlCommit", args: { ...commandArgs } } : undefined,
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
  const hasContent =
    (options?.length ?? 0) > 0 || Boolean(input) || Boolean(control) || (status?.length ?? 0) > 0 || (possibleEngagements?.length ?? 0) > 0;
  if (!hasContent) return undefined;
  return { sessionActive: engagement.sessionActive, options, input, control, status, possibleEngagements };
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
    return <PlaygroundPanelBody><div className="p-2 text-xs text-destructive">Expected tree panel {props.bodyKey}</div></PlaygroundPanelBody>;
  }
  return (
    <PlaygroundPanelBody>
      <PlaygroundDeclarativeTree treeNode={node} commandBus={bus} />
    </PlaygroundPanelBody>
  );
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
    return (
      <PlaygroundPanelBody>
        <PlaygroundDeclarativeTree treeNode={node} commandBus={bus} />
      </PlaygroundPanelBody>
    );
  }
  return (
    <PlaygroundPanelBody>
      <UiRenderer node={node} commandBus={bus} />
    </PlaygroundPanelBody>
  );
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
  readonly augmentPanelTabs?: Partial<Record<"workbench" | "details", readonly (SidePanelTabConfig | SidePanelTabDefinition)[]>>;
  readonly onActiveWindowChange?: (windowKindId: string) => void;
}

const playgroundFixtureCatalogSnapshotCache = new WeakMap<object, PlaygroundFixtureCatalog | null>();

function playgroundFixtureCatalogSemanticallyEqual(a: PlaygroundFixtureCatalog, b: PlaygroundFixtureCatalog): boolean {
  if (a.activeFixtureId !== b.activeFixtureId || a.options.length !== b.options.length) {
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
function usePlaygroundFixtureCatalog(runtime: Platform, controllerId: string | undefined): PlaygroundFixtureCatalog | null {
  return reactHostPort.useSyncExternalStore(
    (listener) => {
      const app = runtime.getActiveApp();
      const controller = app?.controller.id === controllerId ? app.controller : undefined;
      const unsubscribeSnapshot =
        controller && "subscribeSnapshot" in controller && typeof controller.subscribeSnapshot === "function"
          ? (controller as import("@framework/playground/core").Controller & { subscribeSnapshot: (l: () => void) => () => void }).subscribeSnapshot(listener)
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
      const next = resolvePlaygroundFixtureCatalog(controller);
      const cached = playgroundFixtureCatalogSnapshotCache.get(controller);
      if (cached === next) {
        return cached;
      }
      if (cached && next && playgroundFixtureCatalogSemanticallyEqual(cached, next)) {
        return cached;
      }
      playgroundFixtureCatalogSnapshotCache.set(controller, next);
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

  const mergedTools = reactHostPort.useMemo(() => (activeApp ? declareToolsToViewTools(activeApp.tools, bus) : undefined), [activeApp, bus, shellDataGeneration]);
  const hasToolbarTools = listPopulatedToolbarViewCategories(mergedTools ?? {}).length > 0;

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
    resolveInitialPanelVisibility(initialPanelVisibility, runtime),
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
  useElementsSurfaceChrome({ ...PLAYGROUND_SYSTEM_SURFACE_CHROME, compact: uiCompact, expertise: uiExpertise });

  const namedLayoutStore = reactHostPort.useMemo(
    () => (shell.activeApp ? new NamedLayoutStore(shell.activeApp.id, createBrowserStoragePort()) : null),
    [shell.activeApp?.id],
  );
  const [displayHost, setDisplayHost] = reactHostPort.useState<DisplayHostApi | null>(null);
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
      modes: (shell.activeAppBase?.modes ?? []).map((mode) => ({ id: mode.id, label: mode.label, iconId: mode.iconId })),
      activeModeId: shell.activeModeId,
      setActiveModeId,
      hasModeNav,
    }),
    [hasModeNav, setActiveModeId, shell.activeAppBase?.modes, shell.activeModeId, uiCompact, uiExpertise],
  );
  const settingsTabs = reactHostPort.useMemo(() => createFrameworkSettingsPanelTabs(() => settingsHostApi, shell.bus), [settingsHostApi, shell.bus]);
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
  const fixtureCatalog = usePlaygroundFixtureCatalog(runtime, controllerId);
  const navbarFixtureSelect = reactHostPort.useMemo(() => {
    if (slotNavbarCenter !== undefined) return slotNavbarCenter;
    if (!fixtureCatalog || !controllerId) return null;
    return (
      <NavbarFixtureSelect
        id="playground.navbar.fixture"
        value={fixtureCatalog.activeFixtureId}
        options={fixtureCatalog.options}
        onValueChange={(fixtureId) => {
          shell.bus.dispatch(controllerId, "setActiveFixture", { fixtureId });
        }}
      />
    );
  }, [controllerId, fixtureCatalog, shell.bus, slotNavbarCenter]);

  const navbarItems = reactHostPort.useMemo<NavbarItem[]>(() => {
    if (!shell.activeApp) {
      return [];
    }
    const items: NavbarItem[] = [
      {
        key: "title",
        className: "min-w-0 shrink-0 max-w-[40%]",
        content: <span className="truncate px-single text-sm font-medium">{shell.activeApp.label}</span>,
      },
    ];
    if (navbarFixtureSelect) {
      items.push({
        key: "fixture",
        className: "flex-1 min-w-0 flex justify-center",
        content: navbarFixtureSelect,
      });
    }
    items.push({
        key: "panelToggles",
        content: (
          <div className="flex min-w-0 items-stretch border border-normal h-medium">
            {displayTabs.length > 0 ? (
              <Toggle
                id="ui.panelToggle.display"
                pressed={panelVisibility.leftSidePanel && activeLeftPanelKind === "display"}
                onPressedChange={(pressed) => {
                  if (pressed) setActiveLeftPanelKind("display");
                  setPanelVisibility((p) => ({ ...p, leftSidePanel: pressed || (activeLeftPanelKind === "workbench" && p.leftSidePanel) }));
                }}
                icon={displayIcon}
                className="rounded-none border-0 shrink-0"
              />
            ) : null}
            <Toggle
              id="ui.panelToggle.workbench"
              pressed={panelVisibility.leftSidePanel && activeLeftPanelKind === "workbench"}
              onPressedChange={(pressed) => {
                if (pressed) setActiveLeftPanelKind("workbench");
                setPanelVisibility((p) => ({ ...p, leftSidePanel: pressed || (activeLeftPanelKind === "display" && p.leftSidePanel) }));
              }}
              icon={shell.workbenchIcon}
              className={cn("rounded-none border-0 shrink-0", displayTabs.length > 0 && "border-l")}
            />
            <Toggle
              id="ui.panelToggle.details"
              pressed={panelVisibility.rightSidePanel && activeRightPanelKind === "details"}
              onPressedChange={(pressed) => {
                if (pressed) setActiveRightPanelKind("details");
                setPanelVisibility((p) => ({ ...p, rightSidePanel: pressed || (activeRightPanelKind === "settings" && p.rightSidePanel) }));
              }}
              icon={shell.detailsIcon}
              className="rounded-none border-0 border-l shrink-0"
            />
            <Toggle
              id="ui.panelToggle.settings"
              pressed={panelVisibility.rightSidePanel && activeRightPanelKind === "settings"}
              onPressedChange={(pressed) => {
                if (pressed) setActiveRightPanelKind("settings");
                setPanelVisibility((p) => ({ ...p, rightSidePanel: pressed || (activeRightPanelKind === "details" && p.rightSidePanel) }));
              }}
              icon={settingsIcon}
              className="rounded-none border-0 border-l shrink-0"
            />
          </div>
        ),
      });
    return items;
  }, [
    activeLeftPanelKind,
    activeRightPanelKind,
    displayIcon,
    displayTabs.length,
    navbarFixtureSelect,
    panelVisibility.leftSidePanel,
    panelVisibility.rightSidePanel,
    setPanelVisibility,
    settingsIcon,
    shell.activeApp,
    shell.detailsIcon,
    shell.workbenchIcon,
  ]);

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
        onDisplayHostReady={setDisplayHost}
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

/** @emoji 📦 Standard side-panel body; playgrounds must not use inline styles on panel chrome. */
export function PlaygroundPanelBody({ children }: { readonly children: React.ReactNode }): React.ReactElement {
  return <div className="text-foreground flex min-h-0 min-w-0 flex-1 flex-col gap-single overflow-hidden p-single text-xs">{children}</div>;
}

/** @emoji 🌲 Tree section whose sole row hosts arbitrary React as an inline control (inspector batches). */
export function playgroundPanelSection(id: string, label: string, body: React.ReactNode, options?: { readonly defaultOpen?: boolean }): TreeDataSection {
  return {
    id,
    label,
    defaultOpen: options?.defaultOpen ?? true,
    items: [{ id: `${id}.host`, label: "", control: <PlaygroundPanelBody>{body}</PlaygroundPanelBody> }],
  };
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
import { sceneHostPort } from "@ui/react";
import nakaginPuzzle3dFixtureJson from "../../../../../puzzle/3d/fixture/nakagin-capsule-tower.3d.json";
import {
  PlayCanvas,
  ObjectStateProvider,
  parseFixtureV1,
  applyConnectToFixture,
  applyPaletteObjectDropToFixture,
  blockedVortexFullIdsFromAttractions,
  resolvePuzzle3dFixtureDrop,
  puzzle3dFixturePaletteTreeDragController,
  buildPuzzle3dPlayEngagement,
  getPuzzle3dBrushEngagementEpoch,
  puzzle3dBrushEngagementSourceRef,
  requestPuzzle3dZoomToSelection,
  subscribePuzzle3dBrushEngagementSource,
  isLoadableMeshUrl,
  resolveObjectKindMeshUrl,
  type FixtureV1,
  type Puzzle3dFixtureDropDetail,
  type Puzzle3dHoverPayload,
  type RelocatePayload,
} from "@puzzle/3d/react";
import {
  PUZZLE_3D_PLAY_BODY_KEY,
  PUZZLE_3D_PLAY_CONTROLLER_ID,
  PUZZLE_3D_PLAY_IDLE_SNAPSHOT,
  PUZZLE_3D_PLAY_ICON_HIERARCHY,
  PUZZLE_3D_PLAY_ICON_INSPECTOR,
  PUZZLE_3D_PLAY_ICON_KINDS,
  PUZZLE_3D_PLAY_ICON_SETTINGS,
  PUZZLE_3D_PLAY_VIEWPORT_SURFACE_ID,
  PUZZLE_3D_PLAY_APP_ID,
  PUZZLE_3D_PLAY_STORE_ID,
  PUZZLE_3D_PLAY_SNAPSHOT_PANEL_BODY_KEYS,
  Puzzle3dPlayShellController,
  installPuzzle3dPlayBrushHost,
  clearPuzzle3dFillSession,
  preparePuzzle3dFillSession,
  rerollPuzzle3dFillTail,
  puzzle3dFillBuildProgressRef,
  puzzle3dFillPendingCountRef,
  puzzle3dFillSessionRef,
  PUZZLE_3D_FILL_COUNT_MAX,
  subscribePuzzle3dFillSessionReady,
  subscribePuzzle3dFillDistributionInvalidated,
  getPuzzle3dFillSessionReadyEpoch,
  parseKindCatalogs,
  parseKindCompatibility,
  type Puzzle3dPlayHostBridge,
  type Puzzle3dPlaySnapshot,
} from "@puzzle/3d/play";
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
  return `${selection.objectIds.join("\0")}\0${selection.vortexIds.join("\0")}\0${selection.attractionIds.join("\0")}\0${hover.kindHover?.domain ?? ""}\0${hover.kindHover?.kindId ?? ""}`;
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
  return { sessionActive: engagement.sessionActive, options, input, control, status, possibleEngagements };
}

function Puzzle3dPlayEngagementPublisher(props: {
  readonly ctrl: Puzzle3dPlayShellController | undefined;
  readonly snap: Puzzle3dPlaySnapshot;
  readonly bus: CommandBus;
}): null {
  const { ctrl, snap, bus } = props;
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
  const selectionCount = snap.selection.objectIds.length + snap.selection.vortexIds.length + snap.selection.attractionIds.length;
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
        selectionCount,
        onCmdLineChange: setCmdLine,
        onCmdLineSubmit,
        onRepeatLast: onRepeatLastEngagement,
        onAbort: onEngagementAbort,
        onSelectTool,
        onBrushTool,
        onFillTool,
        onFillCount,
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
      }),
    [brushEngagementEpoch, brushSource, cmdLine, fillBuildProgress, fillCount, fillSessionReadyEpoch, onBrushTool, onCmdLineSubmit, onEngagementAbort, onFillCount, onFillTool, onRepeatLastEngagement, onSelectTool, onZoomToSelection, rememberEngagementRepeat, selectionCount, snap.activeTool],
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
            const value = (args as { value?: number })?.value;
            const control = engagementSpecRef.current?.control;
            if (value === undefined || !control || control.kind === "ring") break;
            control.onChange?.(value);
            break;
          }
          case "engagementControlCommit": {
            const value = (args as { value?: number })?.value;
            const control = engagementSpecRef.current?.control;
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

const Puzzle3dPlayViewportHost = reactHostPort.memo(function Puzzle3dPlayViewportHost({ node }: { readonly node: UiPuzzle3dHostSurfaceNode }): React.ReactElement {
  const { runtime } = useApp();
  const bus = runtime.commandBus;
  const ctrl = usePuzzle3dPlayController();
  const snap = usePuzzle3dPlaySnapshot();
  const shellInstance = useShellWindowInstance();
  const viewportCamera = reactHostPort.useMemo(
    () => ctrl?.cameraForInstance(shellInstance?.instanceId) ?? snap.fixture?.camera,
    [ctrl, shellInstance?.instanceId, snap.cameraSeedEpoch, snap.fixture?.camera],
  );
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
    (updater: (prev: FixtureV1) => FixtureV1) => {
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
  const fillBaseCaptureRef = reactHostPort.useRef<FixtureV1 | null>(null);
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
        preparePuzzle3dFillSession(base, kindCatalogs, kindCompatibility, snap.brushPlacementOverlapBudget);
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
  }, [fillDistributionEpoch, onFillMeshesReady, snap.activeTool, snap.brushPlacementOverlapBudget]);
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
          marqueeSelectableKinds={snap.selectableKinds}
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

let puzzle3dPlayChromeRegistered = false;

/** @emoji 🧊 Registers puzzle 3D play surface host, tab icons, and mesh preload. */
export function registerPuzzle3dPlaySurfaceHosts(): void {
  if (puzzle3dPlayChromeRegistered) return;
  puzzle3dPlayChromeRegistered = true;
  registerUiPuzzle3dSurfaceHost(PUZZLE_3D_PLAY_VIEWPORT_SURFACE_ID, Puzzle3dPlayViewportHost);
  registerTabIcon(PUZZLE_3D_PLAY_ICON_INSPECTOR, "clipboard-list");
  registerTabIcon(PUZZLE_3D_PLAY_ICON_KINDS, "tags");
  registerTabIcon(PUZZLE_3D_PLAY_ICON_HIERARCHY, "list-tree");
  registerTabIcon(PUZZLE_3D_PLAY_ICON_SETTINGS, "settings");
  const fixture = parseFixtureV1(nakaginPuzzle3dFixtureJson as unknown);
  if (fixture) {
    const catalogs = parseKindCatalogs(fixture.meta);
    const urls = [
      ...new Set(
        fixture.objects
          .map((object) => resolveObjectKindMeshUrl(object.objectKind ?? "", catalogs, fixture) ?? object.meshUrl)
          .filter((url): url is string => Boolean(url)),
      ),
    ];
    for (const url of urls) {
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
  FiveD,
  StoreProvider,
  useStore as usePuzzle5dStore,
  buildPuzzle5dFillSequence,
  project2dKindCatalogs,
  puzzle5dBrushPlacementFromFlat,
  puzzle5dBrushPlacementFromVolume,
  type Store as Puzzle5dStore,
} from "@puzzle/5d/react";
import type { Playground } from "@framework/playground/core";
import {
  PUZZLE_5D_PLAY_APP_ID,
  PUZZLE_5D_PLAY_2D_BODY_KEY,
  PUZZLE_5D_PLAY_2D_SURFACE_ID,
  PUZZLE_5D_PLAY_2D_WINDOW_ID,
  PUZZLE_5D_PLAY_CONTROLLER_ID,
  PUZZLE_5D_PLAY_STORE_ID,
  PUZZLE_5D_PLAY_3D_BODY_KEY,
  PUZZLE_5D_PLAY_3D_SURFACE_ID,
  PUZZLE_5D_PLAY_HIERARCHY_TAB_ID,
  Puzzle5dPlayShellController,
  Puzzle5dStoreBridge,
  type Puzzle5dPlayHostBridge,
  buildPuzzle5d2dDeclarativeBody,
  buildPuzzle5dPlayHierarchySections,
  buildPuzzle5dPlayRuntime,
  buildPuzzle5d3dDeclarativeBody,
  type Puzzle5dPlaySnapshot,
} from "@puzzle/5d/play";
import {
  puzzle2dActiveRenderer,
  puzzle2dNodeKindOverlayLabel,
  type Puzzle2dBrushCandidatesPayload,
  type Puzzle2dDrawLodKind,
  type Puzzle2dSelectionMethod,
  type Puzzle2dSelectionMode,
  type Puzzle2dSelectionTargets,
} from "@puzzle/2d/react";
import { installPuzzle3dPlayBrushHost, puzzle3dBrushMeshRootForFill } from "@puzzle/3d/play";
import { puzzle3dBrushEngagementSourceRef } from "@puzzle/3d/react";
import { sceneHostPort } from "@ui/react";
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

  reactHostPort.useEffect(() => {
    const bridge: Puzzle5dPlayHostBridge = {
      getToolbarState: () => ({
        puzzle2dActiveTool: controller.getActiveTool(),
        puzzle2dBrushFlushDistance: controller.getSnapshot().brushFlushDistance,
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
          case "setBrushFlushDistance": {
            const distance = Number((args as { distance?: number }).distance);
            if (Number.isFinite(distance)) {
              puzzle2dActiveRenderer()?.setBrushFlushDistance(distance);
            }
            break;
          }
          case "setBrushOverlapBudget": {
            break;
          }
          case "pickBrushCandidate": {
            const index = Number((args as { index?: number }).index);
            if (Number.isFinite(index)) {
              puzzle2dActiveRenderer()?.setBrushCandidateIndex(index);
            }
            break;
          }
          case "engagementPossibleSelect": {
            const possibleId = (args as { possibleId?: string }).possibleId ?? "";
            const brushMatch = possibleId.match(/^puzzle(?:2d|3d|5d)\.brush\.(.+)\.(\d+)$/);
            if (brushMatch) {
              const index = Number(brushMatch[2]);
              if (Number.isFinite(index)) {
                puzzle2dActiveRenderer()?.setBrushCandidateIndex(index);
                puzzle3dBrushEngagementSourceRef.current.pickCandidate(index);
              }
            }
            break;
          }
          case "setSelectionMethod": {
            const method = (args as { method?: Puzzle2dSelectionMethod }).method;
            if (method) selectionMethodRef.current = method;
            break;
          }
          case "setSelectionMode": {
            const mode = (args as { mode?: Puzzle2dSelectionMode }).mode;
            if (mode) selectionModeRef.current = mode;
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
            store.setSelection({ partIds: [], anchorIds: [] });
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

function puzzle5dBrushCandidateRows(payload: Puzzle2dBrushCandidatesPayload, kindCatalogs: ReturnType<typeof project2dKindCatalogs>): { readonly id: string; readonly label: string }[] {
  return payload.candidates.map((kindId, index) => ({
    id: `puzzle5d.brush.${kindId}.${index}`,
    label: puzzle2dNodeKindOverlayLabel(kindId, kindCatalogs ?? undefined),
  }));
}

function usePuzzle5dPlayStore(): Puzzle5dStore {
  return usePuzzle5dStore();
}
//#endregion 🔖HostBridge

//#region 🔖DetailsPanel
function Puzzle5dPlayStatusPanel(): React.ReactElement {
  const { snapshot } = usePuzzle5dPlaySnapshot();
  if (!snapshot) {
    return <p className="text-muted-foreground p-2 text-xs">No puzzle 5d snapshot</p>;
  }
  return (
    <dl className="grid gap-2 p-2 text-xs">
      <div>
        <dt className="text-muted-foreground font-medium">Manifest</dt>
        <dd>{snapshot.manifestLabel ?? "—"}</dd>
      </div>
      <div>
        <dt className="text-muted-foreground font-medium">2d selection</dt>
        <dd>{snapshot.selected2d.size} id(s)</dd>
      </div>
      <div>
        <dt className="text-muted-foreground font-medium">3d selection</dt>
        <dd>{snapshot.selected3d ?? "—"}</dd>
      </div>
      <div>
        <dt className="text-muted-foreground font-medium">Relocate</dt>
        <dd>{JSON.stringify(snapshot.gumballConfig)}</dd>
      </div>
      <div>
        <dt className="text-muted-foreground font-medium">Connect events</dt>
        <dd>
          2d {snapshot.connect2d} · 3d {snapshot.connect3d}
        </dd>
      </div>
      <div>
        <dt className="text-muted-foreground font-medium">Proximity events</dt>
        <dd>
          2d {snapshot.proximity2d} · 3d {snapshot.proximity3d}
        </dd>
      </div>
      <div>
        <dt className="text-muted-foreground font-medium">Tool</dt>
        <dd>
          {snapshot.activeTool} · fill {snapshot.fillCount}
        </dd>
      </div>
    </dl>
  );
}

class Puzzle5dPlayHierarchyPanelDefinition extends PureSidePanelTabDefinition {
  constructor(private readonly buildTree: () => import("@framework/playground/core").UiTreeNode) {
    super();
  }

  buildTab(): SidePanelTabConfig {
    return {
      id: PUZZLE_5D_PLAY_HIERARCHY_TAB_ID,
      icon: createIconComponent("list-tree"),
      order: 0,
      tree: new StaticTreePanelDefinition({ sections: this.buildTree().sections as TreeDataSection[] }),
    };
  }
}

class Puzzle5dPlayStatusPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: "puzzle-5d-play-status",
      icon: createIconComponent("clipboard-list"),
      order: 0,
      tree: new StaticTreePanelDefinition({
        sections: [playgroundPanelSection("puzzle-5d-play-status.section", "Paired play", <Puzzle5dPlayStatusPanel />)],
      }),
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
    if (storeRef.current.applyBrushPlacement(puzzle5dBrushPlacementFromFlat(payload))) {
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
  if (!bindingValid || !controller || !snapshot) {
    return <div className="p-2 text-xs text-muted-foreground">Invalid puzzle 5d 2d binding</div>;
  }
  return (
    <FiveD
      mode="2d"
      instanceId="play-2d"
      activeTool={snapshot.activeTool}
      brushFlushDistance={snapshot.brushFlushDistance}
      puzzle2d={{
        onLodChange,
        onSelect,
        onConnect,
        onProximityConnect,
        onBrushPlace,
        onBrushCandidates,
        onDelete,
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
    const urls = [...new Set(store.read().parts.flatMap((part) => (part.puzzle3d ? [part.puzzle3d.meshUrl] : [])))];
    for (const url of urls) sceneHostPort.drei.useGLTF.preload(url);
  }, [bindingValid, modelPartCount, store]);
  reactHostPort.useEffect(() => {
    if (!bindingValid || snapshot?.activeTool !== "fill") return;
    fillSeedRef.current = (Date.now() ^ Math.floor(Math.random() * 0x7fffffff)) >>> 0;
  }, [bindingValid, snapshot?.activeTool]);
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
    if (storeRef.current.applyBrushPlacement(puzzle5dBrushPlacementFromVolume(payload))) {
      controllerRef.current?.setBrushEngagementPossibles([]);
    }
  }, []);
  const onFillMeshesReady = reactHostPort.useCallback(() => {
    const activeStore = storeRef.current;
    const activeController = controllerRef.current;
    const model = activeStore.read();
    activeStore.setFillBuildDone(false);
    const sequence = buildPuzzle5dFillSequence({
      model,
      seed: fillSeedRef.current,
      overlapBudget: brushOverlapBudgetRef.current,
      meshRootForUrl: puzzle3dBrushMeshRootForFill,
    });
    activeStore.prepareFillSession(sequence, model, fillSeedRef.current);
    activeStore.setFillBuildDone(true);
    if (sequence.length > 0) {
      activeController?.run("setFillCount", { count: 1 });
    }
  }, []);
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
      }}
    />
  );
}
//#endregion 🔖Surfaces

//#region 🔖Mount
let topologyPlayChromeRegistered = false;

/** @emoji 🧊 Registers topology play flat+volume surface hosts (called from `@framework/playground/renderer/react`). */
export function registerPuzzle5dPlaySurfaceHosts(): void {
  if (topologyPlayChromeRegistered) return;
  topologyPlayChromeRegistered = true;
  registerUiPuzzle2dSurfaceHost(PUZZLE_5D_PLAY_2D_SURFACE_ID, Puzzle5d2dSurfaceHost);
  registerUiPuzzle3dSurfaceHost(PUZZLE_5D_PLAY_3D_SURFACE_ID, Puzzle5d3dSurfaceHost);
  registerWindowBody(PUZZLE_5D_PLAY_2D_BODY_KEY, buildPuzzle5d2dDeclarativeBody);
  registerWindowBody(PUZZLE_5D_PLAY_3D_BODY_KEY, buildPuzzle5d3dDeclarativeBody);
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
  const snapshot = controller?.getSnapshot() ?? null;
  const bus = runtime.commandBus;
  const snapshotKey = snapshot ? `${snapshot.manifestLabel ?? ""}\u0001${snapshot.selected3d ?? ""}\u0001${[...snapshot.selected2d].sort().join(",")}` : "";
  const workbenchTabs = reactHostPort.useMemo(
    () =>
      snapshot && controller
        ? [
            new Puzzle5dPlayHierarchyPanelDefinition(() =>
              buildPuzzle5dPlayHierarchySections(snapshot, {
                onSelect2d: (id) => bus.dispatch(PUZZLE_5D_PLAY_CONTROLLER_ID, "set2dSelection", { ids: [id] }),
                onSelect3dObject: (objectId) => bus.dispatch(PUZZLE_5D_PLAY_CONTROLLER_ID, "set3dSelection", { objectIds: [objectId] }),
                onSelect3dVortex: () => {},
                onSelect3dAttraction: () => {},
              }),
            ).resolveTab(),
          ]
        : [],
    [snapshot, snapshotKey, controller, bus],
  );
  const detailTabs = reactHostPort.useMemo(() => [new Puzzle5dPlayStatusPanelDefinition().resolveTab()], []);
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
  const puzzle5dStore = puzzle5dBridge?.inner ?? controller.puzzle5dStore;
  return (
    <StoreProvider store={puzzle5dStore}>
      <Puzzle5dPlayHostBridgeInstaller controller={controller} store={puzzle5dStore} />
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
import type { ReactElement, ReactNode } from "react";
import {
  PUZZLE_2D_PLAY_APP_ID,
  PUZZLE_2D_PLAY_SURFACE_ID,
  PUZZLE_2D_PLAY_BODY_KEY_DETAIL,
  PUZZLE_2D_PLAY_BODY_KEY_OVERVIEW,
  PUZZLE_2D_PLAY_BODY_KEY_SELECTION,
  PUZZLE_2D_PLAY_CONTROLLER_ID,
  PUZZLE_2D_PLAY_DEFAULT_FIXTURE,
  PUZZLE_2D_PLAY_EMPTY_FIXTURE,
  PUZZLE_2D_PLAY_FIXTURE_NAKAGIN_ID,
  PUZZLE_2D_PLAY_FIXTURE_OPTIONS,
} from "@puzzle/2d/play";
import {
  buildWiresPlayHierarchySections,
  buildWiresPlayKindsTree,
  WIRES_PLAY_DEFAULT_FIXTURE,
  WIRES_PLAY_FIXTURE,
  WIRES_PLAY_FIXTURE_METABOLISM_ID,
  WIRES_PLAY_FIXTURE_OPTIONS,
  WIRES_PLAY_HIERARCHY_TAB_ID,
  WIRES_PLAY_KINDS_TAB_ID,
  WIRES_PLAY_LIVE_FORCE_GRAPH_DEFAULTS,
  wiresPlayHierarchyGraphIdFromTreeItemId,
  wiresPlayHierarchyTreeHighlightedIds,
  wiresPlayHierarchyTreeSelectedIds,
  wiresPlayIdentityLabelForNodeId,
  wiresPlayRelationshipKindDisplayName,
} from "@reasoning/mindmap/wires/play";
import {
  PUZZLE_2D_PLAY_HIERARCHY_TAB_ID,
  Puzzle2dPlayShellController,
  puzzle2dPlayPaneFromShellWindowId,
  PUZZLE_2D_ENGAGEMENT_TOOL_BRUSH_ID,
  buildPuzzle2dPlayHierarchySections,
  buildPuzzle2dPlayKindsTree,
  PUZZLE_2D_PLAY_KINDS_TAB_ID,
  PUZZLE_2D_PLAY_ICON_KINDS,
  puzzle2dPlayAllSelectionFromFixture,
  puzzle2dPlayHierarchyGraphIdFromTreeItemId,
  puzzle2dPlayHierarchyTreeHighlightedIds,
  puzzle2dPlayKindsTreeHighlightedIds,
  puzzle2dPlayHierarchyTreeSelectedIds,
  buildPuzzle2dPlayOverviewDeclarativeBody,
  buildPuzzle2dPlayDetailDeclarativeBody,
  buildPuzzle2dPlaySelectionDeclarativeBody,
  buildPuzzle2dPlayRuntime,
  flushPuzzle2dPlayStructuralDeleteBatch,
  puzzle2dPlayForwardsCanvasStructuralDelete,
  puzzle2dPlayApplyNodeStructuralDeleteToFixture,
  puzzle2dPlayRehydrateFixtureEdgesIfMissing,
  puzzle2dPlayInspectorKindSectionLabel,
  puzzle2dPlayKindCatalogSelectItems,
  type Puzzle2dPlayHostBridge,
  type Puzzle2dPlayPaneId,
  type Puzzle2dPlayStructuralDeleteItem,
} from "@puzzle/2d/play";
import {
  DEFAULT_KIND_CATALOG_BUNDLE,
  BUILTIN_PORT_HANDLE_KIND,
  PUZZLE_2D_CAMERA_ZOOM_MIN,
  PUZZLE_2D_CAMERA_ZOOM_MAX,
  PUZZLE_2D_PRESELECT_EMPTY,
  PUZZLE_2D_SELECTION_TARGETS_DEFAULT,
  puzzle2dFixtureMetaKindCompatibility,
  puzzle2dFixtureNodeCaption,
  puzzle2dFixtureHandleEndpointDisplayLabel,
  puzzle2dFixtureMergedKindCatalogs,
  puzzle2dFixtureObjectDisplayLabel,
  puzzle2dNodeKindOverlayLabel,
  puzzle2dHandleKindOverlayLabel,
  puzzle2dEdgeKindOverlayLabel,
  puzzle2dApplyNodeKindToFixtureNode,
  puzzle2dHandleAngleFromRingT,
  puzzle2dHandleAngleToRingT,
  classifyPuzzle2dIconSelectorMode,
  parsePuzzle2dFixtureV1,
  Puzzle2dCanvas,
  puzzle2dIsBrushPlacementStructuralDeleteGuarded,
  puzzle2dSyncFixtureDescriptorToAllAuthoringPeers,
  puzzle2dSyncLayoutNodePositionsToAllAuthoringPeers,
  puzzle2dSyncBrushSessionToAllAuthoringPeers,
  puzzle2dCommitBrushPlacementToPlay,
  puzzle2dSetBrushPlaceCommitHandler,
  puzzle2dActiveRenderer,
  applyBrushFillPlacementsToFixture,
  DEFAULT_PUZZLE_2D_BRUSH_FLUSH_DISTANCE_PX,
  DEFAULT_PUZZLE_2D_BRUSH_NODE_SIZE_PX,
  puzzle2dSyncSelectionToAllAuthoringPeers,
  buildPuzzle2dSceneDescriptorFromFixture,
  clonePuzzle2dFixtureV1,
  puzzle2dFixtureSceneMarkers,
  type Puzzle2dStructureDeletePayload,
  mergePaletteNodeFromDrop,
  puzzle2dCommitPaletteNodeDropToPlay,
  puzzle2dFixturePaletteTreeDragController,
  PUZZLE_2D_FIXTURE_DRAG_V1_MIME,
  PUZZLE_2D_LOD_MODE_AUTOMATIC,
  layoutPuzzle2dFixtureRedrawHandles,
  layoutPuzzle2dFixtureRedrawNodes,
  normalizePuzzle2dSelectionProp,
  type Puzzle2dFixtureV1,
  type Puzzle2dFixtureNodeV1,
  type Puzzle2dFixtureRectangleNodeV1,
  type Puzzle2dFixtureCircleNodeV1,
  type Puzzle2dFixtureHandleV1,
  type Puzzle2dFixtureEdgeV1,
  type Puzzle2dFixtureDropDetail,
  type Puzzle2dDrawLodKind,
  type Puzzle2dLodModeKind,
  type Puzzle2dSelectionMethod,
  type Puzzle2dSelectionMode,
  type Puzzle2dSelectionTargets,
  type Puzzle2dActiveTool,
  type Puzzle2dBrushPlacePayload,
  type Puzzle2dBrushCandidatesPayload,
  type Puzzle2dSelectionSnapshot,
  type Puzzle2dPreselectSnapshot,
  type Puzzle2dRedrawModeKind,
  type Puzzle2dHierarchicalTreeDirectionKind,
  type Puzzle2dRedrawLayoutOptions,
  type Puzzle2dKindHover,
  type Puzzle2dHoverPayload,
  type KindCatalogBundle,
  type CameraState,
} from "@puzzle/2d/react";
import type { Playground } from "@framework/playground/core";
// #endregion 🔌Adapters

const PUZZLE_2D_PLAY_IS_WIRES = import.meta.env.PUZZLE_PLAY_ENTRY === "wires";

function puzzle2dPlayHierarchyTreeSelectedIdsForFixture(fixture: Puzzle2dFixtureV1, graphSelectionIds: readonly string[]): string[] {
  return PUZZLE_2D_PLAY_IS_WIRES
    ? wiresPlayHierarchyTreeSelectedIds(fixture, graphSelectionIds)
    : puzzle2dPlayHierarchyTreeSelectedIds(fixture, graphSelectionIds);
}

function puzzle2dPlayHierarchyTreeHighlightedIdsForFixture(
  fixture: Puzzle2dFixtureV1,
  graphHoverId: string | null,
  kindHover: Puzzle2dKindHover | null = null,
): readonly string[] {
  return PUZZLE_2D_PLAY_IS_WIRES
    ? wiresPlayHierarchyTreeHighlightedIds(fixture, graphHoverId)
    : puzzle2dPlayHierarchyTreeHighlightedIds(fixture, graphHoverId, kindHover);
}

function puzzle2dPlayKindsTreeHighlightedIdsForFixture(
  fixture: Puzzle2dFixtureV1,
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

function puzzle2dPlayResolvedDefaultFixture(): Puzzle2dFixtureV1 {
  return PUZZLE_2D_PLAY_IS_WIRES ? WIRES_PLAY_DEFAULT_FIXTURE : PUZZLE_2D_PLAY_DEFAULT_FIXTURE;
}

const PUZZLE_2D_PLAY_DEFAULT_KIND_CATALOGS = puzzle2dFixtureMergedKindCatalogs(puzzle2dPlayResolvedDefaultFixture());

// #region 🔖Kinds
export type { Puzzle2dPlayPaneId } from "@puzzle/2d/play";

const puzzle2dPlayOverviewWindowContextMenu: ContextMenuItem[] = [{ id: "win-demo", label: "Overview window menu demo" }];
const puzzle2dPlayDemoNodeContextMenu: ContextMenuItem[] = [
  { id: "demo-node", label: "Demo capsule action" },
  { children: [{ id: "demo-sub-1", label: "Nested item" }], id: "demo-sub", label: "Demo nested" },
];
const puzzle2dPlayDemoEdgeContextMenu: ContextMenuItem[] = [{ id: "demo-edge", label: "Demo edge action" }];
const puzzle2dPlayCanvasBackgroundMenu: ContextMenuItem[] = [{ id: "demo-bg", label: "Puzzle 2D background menu" }];

// #endregion 🔖Kinds

// #region 🔖Geometry
const REF_VIEWPORT_SHORT_PX = 640;

function clampZoom(value: number): number {
  return Math.min(PUZZLE_2D_CAMERA_ZOOM_MAX, Math.max(PUZZLE_2D_CAMERA_ZOOM_MIN, value));
}

/** @emoji 📐 Axis-aligned bounds of all fixture nodes (world units). */
function fixtureWorldBounds(fixture: Puzzle2dFixtureV1): { cx: number; cy: number; halfSpan: number } {
  let minX = Number.POSITIVE_INFINITY;
  let minY = Number.POSITIVE_INFINITY;
  let maxX = Number.NEGATIVE_INFINITY;
  let maxY = Number.NEGATIVE_INFINITY;
  for (const node of fixture.nodes) {
    if (node.shape === "rectangle") {
      const hw = node.width / 2;
      const hh = node.height / 2;
      minX = Math.min(minX, node.x - hw);
      maxX = Math.max(maxX, node.x + hw);
      minY = Math.min(minY, node.y - hh);
      maxY = Math.max(maxY, node.y + hh);
    } else {
      minX = Math.min(minX, node.x - node.radius);
      maxX = Math.max(maxX, node.x + node.radius);
      minY = Math.min(minY, node.y - node.radius);
      maxY = Math.max(maxY, node.y + node.radius);
    }
  }
  if (!Number.isFinite(minX)) {
    return { cx: 0, cy: 0, halfSpan: 400 };
  }
  const cx = (minX + maxX) / 2;
  const cy = (minY + maxY) / 2;
  const halfSpan = Math.max(maxX - minX, maxY - minY, 1) / 2;
  return { cx, cy, halfSpan };
}

/** @emoji 📷 Default cameras for all play panes: center on fixture bounds; zoom fits the graph’s longest axis into the reference short viewport (margin padding). */
function triptychCamerasFromFixture(fixture: Puzzle2dFixtureV1): Record<Puzzle2dPlayPaneId, CameraState> {
  const { cx, cy, halfSpan } = fixtureWorldBounds(fixture);
  const base = fixture.camera;
  const margin = 0.06;
  const usable = REF_VIEWPORT_SHORT_PX * (1 - 2 * margin);
  const worldSpan = Math.max(2 * halfSpan, 1);
  const zoom = clampZoom(usable / worldSpan);
  const cam: CameraState = { x: cx + base.x, y: cy + base.y, zoom };
  return {
    "2d-detail": { ...cam },
    "2d-overview": { ...cam },
    "2d-selection": { ...cam },
  };
}

function puzzle2dPlayInitialCameras(): Record<Puzzle2dPlayPaneId, CameraState> {
  return triptychCamerasFromFixture(puzzle2dPlayResolvedDefaultFixture());
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
function selectionSeedForFixture(fixture: Puzzle2dFixtureV1): Set<string> {
  const nodeA = fixture.nodes[0];
  return new Set(nodeA?.id ? [nodeA.id] : []);
}
// #endregion 🔖Geometry

// #region 🔖ShellContext
interface Puzzle2dPlayShellValue {
  fixture: Puzzle2dFixtureV1;
  setFixture: (next: Puzzle2dFixtureV1) => void;
  /** @emoji 🎯 Palette drags merge one node at the pointer; full fixtures replace the graph. */
  handleCanvasFixtureDrop: (pane: Puzzle2dPlayPaneId, detail: Puzzle2dFixtureDropDetail) => void;
  patchFixture: (updater: (prev: Puzzle2dFixtureV1) => Puzzle2dFixtureV1) => void;
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
  puzzle2dBrushFlushDistance: number;
  setPuzzle2dBrushFlushDistance: (distance: number) => void;
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
  const onHierarchySelect = reactHostPort.useCallback((id: string) => setSelectionIds([id]), [setSelectionIds]);
  const onHierarchyHover = reactHostPort.useCallback((payload: Puzzle2dHoverPayload) => setHierarchyHover(payload), [setHierarchyHover]);
  const sections = reactHostPort.useMemo(() => {
    if (PUZZLE_2D_PLAY_IS_WIRES) {
      return buildWiresPlayHierarchySections(WIRES_PLAY_FIXTURE, fixture, [], onHierarchySelect, {
        omitItemSelection: true,
        onHover: onHierarchyHover,
      }).sections as TreeDataSection[];
    }
    return buildPuzzle2dPlayHierarchySections(fixture, [], onHierarchySelect, undefined, {
      omitItemSelection: true,
      onHover: onHierarchyHover,
    }).sections as TreeDataSection[];
  }, [fixture, onHierarchyHover, onHierarchySelect]);
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
      order: 0,
      tree: new CallbackTreePanelDefinition(
        () => {
          const shell = puzzle2dPlayShellRef.current;
          const selection = puzzle2dPlaySelectionRef.current;
          const bus = puzzle2dPlayRuntimeRef.current?.commandBus ?? new CommandBus();
          if (!shell || !selection) {
            const loadingId = PUZZLE_2D_PLAY_IS_WIRES ? "wires-play-hierarchy.loading" : "puzzle-2d-play-hierarchy.loading";
            return [{ id: loadingId, label: "Hierarchy", items: [{ id: "loading", label: "…" }] }];
          }
          const onHierarchySelect = (id: string) => selection.setSelectionIds([id]);
          const onHierarchyHover = (payload: Puzzle2dHoverPayload) => shell.setHierarchyHover(payload);
          const treeNode = PUZZLE_2D_PLAY_IS_WIRES
            ? (buildWiresPlayHierarchySections(WIRES_PLAY_FIXTURE, shell.fixture, [...selection.selectionIds], onHierarchySelect, {
                omitItemSelection: true,
                onHover: onHierarchyHover,
              }) as UiTreeNode)
            : buildPuzzle2dPlayHierarchySections(shell.fixture, [...selection.selectionIds], onHierarchySelect, undefined, {
                omitItemSelection: true,
                onHover: onHierarchyHover,
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
      order: 1,
      tree: new CallbackTreePanelDefinition(() => {
        const shell = puzzle2dPlayShellRef.current;
        const bus = puzzle2dPlayRuntimeRef.current?.commandBus ?? new CommandBus();
        if (!shell) {
          const loadingId = PUZZLE_2D_PLAY_IS_WIRES ? "wires-play-kinds.loading" : "puzzle-2d-play-kinds.loading";
          return [{ id: loadingId, label: "Kinds", items: [{ id: "loading", label: "…" }] }];
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
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const shell = puzzle2dPlayShellRef.current;
        const selection = puzzle2dPlaySelectionRef.current;
        if (!shell || !selection) {
          return [{ id: "puzzle-2d-play-inspector.loading", label: "Detail", items: [{ id: "loading", label: "…" }] }];
        }
        return buildPuzzle2dPlayInspectorSections(shell.fixture, selection.selectionIds, shell.patchFixture);
      }),
    };
  }
}

class Puzzle2dPlaySettingsPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: "puzzle-2d-play-settings",
      icon: createIconComponent("settings"),
      order: 1,
      tree: new CallbackTreePanelDefinition(() => [playgroundPanelSection("puzzle-2d-play-settings.section", "Settings", <Puzzle2dPlaySettingsPanel />)]),
    };
  }
}

const Puzzle2dPlayShellContext = reactHostPort.createContext<Puzzle2dPlayShellValue | null>(null);

const puzzle2dPlayShellRef: { current: Puzzle2dPlayShellValue | null } = { current: null };
const puzzle2dPlaySelectionRef: { current: Puzzle2dPlaySelectionValue | null } = { current: null };
const puzzle2dPlayRuntimeRef: { current: Platform | null } = { current: null };

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

/** @emoji 🖱️ Re-applies authoritative drag centers after a layout pass so RAF cannot stomp an in-flight pointer drag. */
function puzzle2dPlayFixtureWithDragAnchors(
  fixture: Puzzle2dFixtureV1,
  dragAnchors: ReadonlyMap<string, { readonly x: number; readonly y: number }>,
): Puzzle2dFixtureV1 {
  if (dragAnchors.size === 0) {
    return fixture;
  }
  return {
    ...fixture,
    nodes: fixture.nodes.map((node) => {
      const anchor = dragAnchors.get(node.id);
      return anchor ? { ...node, x: anchor.x, y: anchor.y } : node;
    }),
  };
}
// #endregion 🔖PlayRedrawHelpers

// #region 🔖SettingsPanel
/** @emoji ⚙️ Puzzle 2d play redraw settings: play uses requestAnimationFrame (packed WASM per frame), progressive ramp, and per-mode layout parameters. */
function Puzzle2dPlaySettingsPanel(): ReactElement {
  const {
    activePaneId,
    applyPuzzle2dRedrawHandlesOnce,
    applyPuzzle2dRedrawOnce,
    puzzle2dRedrawHandlesAfterNodes,
    puzzle2dRedrawMode,
    puzzle2dRedrawPlayMaxItersPerFrame,
    puzzle2dRedrawProgressiveAutoStopMs,
    puzzle2dRedrawProgressiveEnabled,
    forceLayoutFullIterations,
    forceLayoutGravity,
    forceLayoutIdealEdgeLength,
    forceLayoutRepulsionStrength,
    setPuzzle2dRedrawMode,
    setPuzzle2dRedrawHandlesAfterNodes,
    setPuzzle2dRedrawPlayMaxItersPerFrame,
    setPuzzle2dRedrawProgressiveAutoStopMs,
    setPuzzle2dRedrawProgressiveEnabled,
    setForceLayoutFullIterations,
    setForceLayoutGravity,
    setForceLayoutIdealEdgeLength,
    setForceLayoutRepulsionStrength,
    setTreeLayoutLayerSpacing,
    setTreeLayoutDirection,
    setTreeLayoutSiblingGap,
    treeLayoutLayerSpacing,
    treeLayoutDirection,
    treeLayoutSiblingGap,
  } = usePuzzle2dPlayShell();

  return (
    <div className="flex h-full min-h-0 flex-col gap-2 p-3 text-xs">
      <div className="text-muted-foreground flex shrink-0 items-center gap-2 border-b border-normal pb-2">
        <Icon icon="settings" size={16} className="shrink-0" />
        <div>
          <div className="font-semibold uppercase tracking-wide">Settings</div>
          <div className="text-[11px] opacity-80">pane: {activePaneId}</div>
        </div>
      </div>
      <div className="text-muted-foreground shrink-0 text-[11px] font-medium uppercase tracking-wide">Redraw</div>
      <div className="min-h-0 flex-1 space-y-4 overflow-y-auto">
        <div className="text-muted-foreground text-[11px] font-medium uppercase tracking-wide">Redraw nodes</div>
        <Label id="puzzle2d.play.settings.redraw.mode" label="Layout kind">
          <Select onValueChange={(v) => setPuzzle2dRedrawMode(v as Puzzle2dRedrawModeKind)} value={puzzle2dRedrawMode}>
            <SelectTrigger className="h-8 w-full" id="puzzle-2d-play-redraw-mode" size="sm">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="force-graph">Graph</SelectItem>
              <SelectItem value="hierarchical-tree">Tree</SelectItem>
            </SelectContent>
          </Select>
        </Label>
        <div className="flex items-center gap-2">
          <input checked={puzzle2dRedrawHandlesAfterNodes} className="accent-accent size-3.5 shrink-0" id="puzzle-2d-play-redraw-handles-after-nodes" onChange={(e) => setPuzzle2dRedrawHandlesAfterNodes(e.target.checked)} type="checkbox" />
          <label className="text-muted-foreground cursor-pointer select-none text-[11px] leading-snug" htmlFor="puzzle-2d-play-redraw-handles-after-nodes">
            Also redraw handles after node redraw
          </label>
        </div>
        <div className="flex items-center gap-2">
          <input checked={puzzle2dRedrawProgressiveEnabled} className="accent-accent size-3.5 shrink-0" id="puzzle-2d-play-redraw-progressive" onChange={(e) => setPuzzle2dRedrawProgressiveEnabled(e.target.checked)} type="checkbox" />
          <label className="text-muted-foreground cursor-pointer select-none text-[11px] leading-snug" htmlFor="puzzle-2d-play-redraw-progressive">
            Progressive iterations while play is on (graph ramps up; tree still one pass per frame)
          </label>
        </div>
        <Label id="puzzle2d.play.settings.redraw.autoStopMs" label="Auto-stop play after (ms, 0 = off)">
          <Slider id="puzzle-2d-play-slider-redraw-autostop" max={12000} min={0} step={250} value={[puzzle2dRedrawProgressiveAutoStopMs]} onValueChange={(vals) => setPuzzle2dRedrawProgressiveAutoStopMs(vals[0] ?? 3000)} />
        </Label>
        {puzzle2dRedrawMode === "force-graph" ? (
          <Label id="puzzle2d.play.settings.redraw.playMaxIters" label="Max iterations per WASM call (play ramp ceiling)">
            <Slider id="puzzle-2d-play-slider-redraw-play-max-iters" max={220} min={12} step={2} value={[puzzle2dRedrawPlayMaxItersPerFrame]} onValueChange={(vals) => setPuzzle2dRedrawPlayMaxItersPerFrame(vals[0] ?? 96)} />
          </Label>
        ) : (
          <p className="text-muted-foreground text-[11px] leading-snug">Tree redraw runs once per animation frame while play is on; use auto-stop to end play after a duration.</p>
        )}
        {puzzle2dRedrawMode === "force-graph" ? (
          <>
            <div className="text-muted-foreground pt-1 text-[11px] font-medium uppercase tracking-wide">Graph</div>
            <Label id="puzzle2d.play.settings.force.fullIterations" label="Iterations (apply once)">
              <Slider id="puzzle-2d-play-slider-force-full-iters" max={720} min={24} step={4} value={[forceLayoutFullIterations]} onValueChange={(vals) => setForceLayoutFullIterations(vals[0] ?? 200)} />
            </Label>
            <Label id="puzzle2d.play.settings.force.idealEdge" label="Ideal edge (px)">
              <Slider id="puzzle-2d-play-slider-force-ideal" max={160} min={20} step={2} value={[forceLayoutIdealEdgeLength]} onValueChange={(vals) => setForceLayoutIdealEdgeLength(vals[0] ?? 64)} />
            </Label>
            <Label id="puzzle2d.play.settings.force.repulsion" label="Repulsion (medium 80, ±40)">
              <Slider id="puzzle-2d-play-slider-force-repulsion" max={120} min={40} step={2} value={[forceLayoutRepulsionStrength]} onValueChange={(vals) => setForceLayoutRepulsionStrength(vals[0] ?? 80)} />
            </Label>
            <Label id="puzzle2d.play.settings.force.gravity" label="Gravity">
              <Slider id="puzzle-2d-play-slider-force-gravity" max={0.05} min={0} step={0.002} value={[forceLayoutGravity]} onValueChange={(vals) => setForceLayoutGravity(vals[0] ?? 0)} />
            </Label>
          </>
        ) : (
          <>
            <div className="text-muted-foreground pt-1 text-[11px] font-medium uppercase tracking-wide">Tree</div>
            <Label id="puzzle2d.play.settings.tree.layerSpacing" label="Layer spacing (px)">
              <Slider id="puzzle-2d-play-slider-tree-layer" max={280} min={40} step={4} value={[treeLayoutLayerSpacing]} onValueChange={(vals) => setTreeLayoutLayerSpacing(vals[0] ?? 120)} />
            </Label>
            <Label id="puzzle2d.play.settings.tree.siblingGap" label="Sibling gap (px)">
              <Slider id="puzzle-2d-play-slider-tree-sibling" max={120} min={0} step={2} value={[treeLayoutSiblingGap]} onValueChange={(vals) => setTreeLayoutSiblingGap(vals[0] ?? 28)} />
            </Label>
            <Label id="puzzle2d.play.settings.tree.direction" label="Direction">
              <Select onValueChange={(v) => setTreeLayoutDirection(v as Puzzle2dHierarchicalTreeDirectionKind)} value={treeLayoutDirection}>
                <SelectTrigger className="h-8 w-full" id="puzzle-2d-play-tree-direction" size="sm">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="downwards">Downwards</SelectItem>
                  <SelectItem value="upwards">Upwards</SelectItem>
                  <SelectItem value="right">Right</SelectItem>
                  <SelectItem value="left">Left</SelectItem>
                </SelectContent>
              </Select>
            </Label>
          </>
        )}
        <Button className="h-8 w-full text-xs" id="puzzle-2d-play-redraw-nodes" type="button" variant="secondary" onClick={applyPuzzle2dRedrawOnce}>
          Redraw nodes
        </Button>
        <div className="text-muted-foreground border-t border-normal pt-2 text-[11px] font-medium uppercase tracking-wide">Redraw handles</div>
        <p className="text-muted-foreground text-[11px] leading-snug">Each edge uses the straight segment between node centers; handle anchors move to where that segment meets each shape (shortest chord through the bodies).</p>
        <Button className="h-8 w-full text-xs" id="puzzle-2d-play-redraw-handles" type="button" variant="secondary" onClick={applyPuzzle2dRedrawHandlesOnce}>
          Redraw handles
        </Button>
        <p className="text-muted-foreground text-[11px] leading-snug">
          While play is on, cameras ease each tick toward a bbox fit of the current layout (damped). After pause, over three seconds the camera stays fixed for the first third, then eases through the last two thirds (slow–fast–slow) to the final bbox
          fit without a jump. Dragging a node resets progressive ramp and the auto-stop timer.
        </p>
      </div>
    </div>
  );
}
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
  showBackgroundMenu,
}: {
  paneId: Puzzle2dPlayPaneId;
  scopeId: string;
  showBackgroundMenu?: boolean;
}): ReactElement {
  const {
    activePaneId,
    activeScopeId,
    patchFixture,
    queueStructuralDelete,
    puzzle2dActiveTool,
    puzzle2dBrushFlushDistance,
    puzzle2dGridSnapEnabled,
    sceneAuthoringEpoch,
    lodModeForScope,
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
  const lodProps = puzzle2dPlayLodCanvasProps(lodModeForScope(scopeId, paneId));
  const reportEffectiveLod = reactHostPort.useContext(Puzzle2dPlayLodRuntimeContext);
  const onLodChange = reactHostPort.useCallback((lod: Puzzle2dDrawLodKind) => reportEffectiveLod?.(paneId, lod), [paneId, reportEffectiveLod]);
  const { applyCanvasSelection } = usePuzzle2dPlayCanvasSelection();
  const onSelect = reactHostPort.useCallback((snapshot: Puzzle2dSelectionSnapshot) => applyCanvasSelection(snapshot.ids), [applyCanvasSelection]);
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
        brushFlushDistance={puzzle2dBrushFlushDistance}
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
  return <Puzzle2dPlayPaneCanvas paneId={paneId} scopeId={scopeId} showBackgroundMenu={paneId === "2d-overview"} />;
}

let puzzle2dPlayChromeRegistered = false;

/** @emoji 🧊 Registers puzzle 2d play surface host, window bodies, and tab icons (called from `@framework/playground/renderer/react`). */
export function registerPuzzle2dPlaySurfaceHosts(): void {
  if (puzzle2dPlayChromeRegistered) return;
  puzzle2dPlayChromeRegistered = true;
  registerUiPuzzle2dSurfaceHost(PUZZLE_2D_PLAY_SURFACE_ID, Puzzle2dPlayPaneSurfaceHost);
  registerWindowBody(PUZZLE_2D_PLAY_BODY_KEY_OVERVIEW, buildPuzzle2dPlayOverviewDeclarativeBody);
  registerWindowBody(PUZZLE_2D_PLAY_BODY_KEY_DETAIL, buildPuzzle2dPlayDetailDeclarativeBody);
  registerWindowBody(PUZZLE_2D_PLAY_BODY_KEY_SELECTION, buildPuzzle2dPlaySelectionDeclarativeBody);
  registerTabIcon(PUZZLE_2D_PLAY_ICON_KINDS, "tags");
  registerTabIcon("puzzle.2d-play.icon.inspector", "clipboard-list");
  registerTabIcon("puzzle.2d-play.icon.settings", "settings");
}
// #endregion 🔖Panes

// #region 🔖SidePanels
function findNode(fixture: Puzzle2dFixtureV1, id: string): Puzzle2dFixtureNodeV1 | undefined {
  return fixture.nodes.find((n) => n.id === id);
}

function findEdge(fixture: Puzzle2dFixtureV1, id: string): Puzzle2dFixtureEdgeV1 | undefined {
  return fixture.edges.find((e) => e.id === id);
}

function findHandleOwner(fixture: Puzzle2dFixtureV1, handleId: string): { node: Puzzle2dFixtureNodeV1; handleId: string } | undefined {
  for (const node of fixture.nodes) {
    if (node.handles.some((h) => h.id === handleId)) {
      return { handleId, node };
    }
  }
  return undefined;
}

function findHandle(fixture: Puzzle2dFixtureV1, handleId: string): Puzzle2dFixtureHandleV1 | undefined {
  for (const node of fixture.nodes) {
    const h = node.handles.find((x) => x.id === handleId);
    if (h) {
      return h;
    }
  }
  return undefined;
}

function nodeIsRectangle(n: Puzzle2dFixtureNodeV1): n is Puzzle2dFixtureRectangleNodeV1 {
  return n.shape === "rectangle";
}

function allEqual<T>(values: T[]): boolean {
  if (values.length === 0) {
    return true;
  }
  const first = values[0];
  return values.every((v) => v === first);
}

function listHandleIds(fixture: Puzzle2dFixtureV1): string[] {
  const out: string[] = [];
  for (const node of fixture.nodes) {
    for (const h of node.handles) {
      out.push(h.id);
    }
  }
  out.sort((a, b) => a.localeCompare(b));
  return out;
}

function toCircleNode(n: Puzzle2dFixtureRectangleNodeV1): Puzzle2dFixtureCircleNodeV1 {
  const { width, height, shape: _s, ...rest } = n;
  const radius = Math.min(width, height) / 2;
  return { ...rest, radius, shape: "circle" };
}

function toRectangleNode(n: Puzzle2dFixtureCircleNodeV1): Puzzle2dFixtureRectangleNodeV1 {
  const { radius, shape: _s, ...rest } = n;
  return { ...rest, shape: "rectangle", width: radius * 2, height: radius * 2 };
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

function InspectorKindSelect({
  id,
  items,
  label,
  onValueChange,
  uniform,
  value,
}: {
  id: string;
  items: readonly { readonly value: string; readonly label: string }[];
  label: string;
  onValueChange: (next: string) => void;
  uniform: boolean;
  value: string;
}): ReactElement {
  const selectValue = uniform && value !== "" ? value : undefined;
  return (
    <Label id={id} label={label}>
      <Select
        key={uniform && value ? `${id}-${value}` : `${id}-mixed`}
        onValueChange={onValueChange}
        value={selectValue}
      >
        <SelectTrigger className="h-7 font-mono text-xs">
          <SelectValue placeholder={uniform ? "kind" : "Mixed"} />
        </SelectTrigger>
        <SelectContent>
          {items.map((item) => (
            <SelectItem key={item.value} value={item.value}>
              {item.label}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
    </Label>
  );
}

function NumericStepperRow({ id, label, onAbsolute, onDelta, step, uniform, value }: { id: string; label: string; onAbsolute: (next: number) => void; onDelta: (delta: number) => void; step: number; uniform: boolean; value: number }): ReactElement {
  return (
    <Label id={id} label={label}>
      <div className="flex min-w-0 items-center gap-1">
        <Button className="h-7 shrink-0 px-2" onClick={() => onDelta(-step)} type="button" variant="outline">
          −
        </Button>
        <Input
          className="h-7 min-w-0 flex-1 font-mono text-xs"
          onChange={(e: ChangeEvent<HTMLInputElement>) => {
            const parsed = Number(e.target.value);
            if (Number.isFinite(parsed)) {
              onAbsolute(parsed);
            }
          }}
          placeholder={uniform ? undefined : "Mixed"}
          value={uniform && Number.isFinite(value) ? String(value) : ""}
        />
        <Button className="h-7 shrink-0 px-2" onClick={() => onDelta(step)} type="button" variant="outline">
          +
        </Button>
      </div>
    </Label>
  );
}

/** @emoji 🟠 Batch node inspector: name (`text`), shape, center, size fields apply to every selected node. */
function InspectorNodeBatch({
  fixture,
  kindCatalogs,
  nodeIds,
  patchFixture,
}: {
  fixture: Puzzle2dFixtureV1;
  kindCatalogs: KindCatalogBundle;
  nodeIds: readonly string[];
  patchFixture: (updater: (prev: Puzzle2dFixtureV1) => Puzzle2dFixtureV1) => void;
}): ReactElement {
  const idSet = reactHostPort.useMemo(() => new Set(nodeIds), [nodeIds]);
  const targets = reactHostPort.useMemo(() => nodeIds.map((id) => findNode(fixture, id)).filter((n): n is Puzzle2dFixtureNodeV1 => Boolean(n)), [fixture, nodeIds]);

  const textValues = targets.map((n) => puzzle2dFixtureNodeCaption(n) ?? "");
  const textUniform = allEqual(textValues);
  const textValue = textUniform ? (textValues[0] ?? "") : "";

  const iconKinds = targets.map((n) => n.iconKind ?? "");
  const iconKindUniform = allEqual(iconKinds);
  const iconKindValue = iconKindUniform ? (iconKinds[0] ?? "") : "";

  const nodeKinds = targets.map((n) => n.nodeKind ?? "");
  const nodeKindUniform = allEqual(nodeKinds);
  const nodeKindValue = nodeKindUniform ? (nodeKinds[0] ?? "") : "";
  const nodeKindItems = reactHostPort.useMemo(
    () => puzzle2dInspectorKindSelectItems(kindCatalogs.nodes, nodeKinds, (kindId) => puzzle2dNodeKindOverlayLabel(kindId, kindCatalogs)),
    [kindCatalogs, nodeKinds],
  );

  const shapes = targets.map((n) => (nodeIsRectangle(n) ? "rectangle" : "circle"));
  const shapeUniform = allEqual(shapes);
  const shapeValue = shapeUniform ? shapes[0] : undefined;

  const xs = targets.map((n) => n.x);
  const ys = targets.map((n) => n.y);
  const xUniform = allEqual(xs);
  const yUniform = allEqual(ys);
  const xValue = xUniform ? xs[0] : Number.NaN;
  const yValue = yUniform ? ys[0] : Number.NaN;

  const radii = targets.filter((n) => !nodeIsRectangle(n)).map((n) => n.radius);
  const widths = targets.filter(nodeIsRectangle).map((n) => n.width);
  const heights = targets.filter(nodeIsRectangle).map((n) => n.height);
  const rUniform = radii.length > 0 && allEqual(radii);
  const wUniform = widths.length > 0 && allEqual(widths);
  const hUniform = heights.length > 0 && allEqual(heights);
  const rValue = rUniform ? radii[0] : Number.NaN;
  const wValue = wUniform ? widths[0] : Number.NaN;
  const hValue = hUniform ? heights[0] : Number.NaN;

  const patchNodes = reactHostPort.useCallback(
    (updater: (n: Puzzle2dFixtureNodeV1) => Puzzle2dFixtureNodeV1) => {
      patchFixture((prev) => ({
        ...prev,
        nodes: prev.nodes.map((n) => (idSet.has(n.id) ? updater(n) : n)),
      }));
    },
    [idSet, patchFixture],
  );

  const onText = reactHostPort.useCallback(
    (next: string) => {
      const trimmed = next.trim();
      patchNodes((n) => (trimmed === "" ? { ...n, text: undefined } : { ...n, text: trimmed }));
    },
    [patchNodes],
  );

  const onIconKind = reactHostPort.useCallback(
    (next: string) => {
      const t = next.trim();
      patchNodes((n) => ({ ...n, ...(t === "" ? { iconKind: undefined } : { iconKind: t }) }));
    },
    [patchNodes],
  );

  const onShape = reactHostPort.useCallback(
    (next: "circle" | "rectangle") => {
      patchNodes((n) => {
        if (next === "rectangle" && !nodeIsRectangle(n)) {
          return toRectangleNode(n);
        }
        if (next === "circle" && nodeIsRectangle(n)) {
          return toCircleNode(n);
        }
        return n;
      });
    },
    [patchNodes],
  );

  const onNodeKind = reactHostPort.useCallback(
    (next: string) => {
      patchNodes((n) => puzzle2dApplyNodeKindToFixtureNode(n, next, kindCatalogs));
    },
    [kindCatalogs, patchNodes],
  );

  return (
    <div className="border-normal/60 space-y-3 border-l pl-2">
      <Label id="puzzle-2d-play.inspector.node.name" label={PUZZLE_2D_PLAY_IS_WIRES ? "Label" : "Name"}>
        <Input className="h-7 font-mono text-xs" onChange={(e: ChangeEvent<HTMLInputElement>) => onText(e.target.value)} placeholder={textUniform ? undefined : "Mixed"} value={textValue} />
      </Label>
      <InspectorKindSelect
        id="puzzle-2d-play.inspector.node.kind"
        items={nodeKindItems}
        label={PUZZLE_2D_PLAY_IS_WIRES ? "Identity kind" : "Node kind"}
        onValueChange={onNodeKind}
        uniform={nodeKindUniform}
        value={nodeKindValue}
      />
      <Label id="puzzle-2d-play.inspector.node.icon" label="Icon">
        <IconSelector classifyPuzzle2dIconSelectorMode={classifyPuzzle2dIconSelectorMode} id="puzzle-2d-play.inspector.node.icon.selector" onChange={onIconKind} uniform={iconKindUniform} value={iconKindValue} />
      </Label>
      <Label id="puzzle-2d-play.inspector.node.shape" label="Shape">
        <Select
          key={shapeUniform && shapeValue ? `shape-${shapeValue}` : "shape-mixed"}
          onValueChange={(v) => {
            if (v === "circle" || v === "rectangle") {
              onShape(v);
            }
          }}
          value={shapeUniform && shapeValue ? shapeValue : undefined}
        >
          <SelectTrigger className="h-7 font-mono text-xs">
            <SelectValue placeholder={shapeUniform ? "shape" : "Mixed"} />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="circle">circle</SelectItem>
            <SelectItem value="rectangle">rectangle</SelectItem>
          </SelectContent>
        </Select>
      </Label>
      <NumericStepperRow id="puzzle-2d-play.inspector.node.x" label="x" onAbsolute={(v) => patchNodes((n) => ({ ...n, x: v }))} onDelta={(d) => patchNodes((n) => ({ ...n, x: n.x + d }))} step={1} uniform={xUniform} value={xValue} />
      <NumericStepperRow id="puzzle-2d-play.inspector.node.y" label="y" onAbsolute={(v) => patchNodes((n) => ({ ...n, y: v }))} onDelta={(d) => patchNodes((n) => ({ ...n, y: n.y + d }))} step={1} uniform={yUniform} value={yValue} />
      {targets.some((n) => !nodeIsRectangle(n)) ? (
        <NumericStepperRow
          id="puzzle-2d-play.inspector.node.r"
          label="radius"
          onAbsolute={(v) => patchNodes((n) => (nodeIsRectangle(n) ? n : { ...n, radius: Math.max(1e-6, v) }))}
          onDelta={(d) => patchNodes((n) => (nodeIsRectangle(n) ? n : { ...n, radius: Math.max(1e-6, n.radius + d) }))}
          step={1}
          uniform={rUniform}
          value={rValue}
        />
      ) : null}
      {targets.some(nodeIsRectangle) ? (
        <>
          <NumericStepperRow
            id="puzzle-2d-play.inspector.node.w"
            label="width"
            onAbsolute={(v) => patchNodes((n) => (nodeIsRectangle(n) ? { ...n, width: Math.max(1e-6, v) } : n))}
            onDelta={(d) => patchNodes((n) => (nodeIsRectangle(n) ? { ...n, width: Math.max(1e-6, n.width + d) } : n))}
            step={1}
            uniform={wUniform}
            value={wValue}
          />
          <NumericStepperRow
            id="puzzle-2d-play.inspector.node.h"
            label="height"
            onAbsolute={(v) => patchNodes((n) => (nodeIsRectangle(n) ? { ...n, height: Math.max(1e-6, v) } : n))}
            onDelta={(d) => patchNodes((n) => (nodeIsRectangle(n) ? { ...n, height: Math.max(1e-6, n.height + d) } : n))}
            step={1}
            uniform={hUniform}
            value={hValue}
          />
        </>
      ) : null}
    </div>
  );
}

/** @emoji 🟣 Batch handle inspector: polar `t`, hit radius, optional id when single selection. */
function InspectorHandleBatch({
  fixture,
  kindCatalogs,
  handleIds,
  patchFixture,
}: {
  fixture: Puzzle2dFixtureV1;
  kindCatalogs: KindCatalogBundle;
  handleIds: readonly string[];
  patchFixture: (updater: (prev: Puzzle2dFixtureV1) => Puzzle2dFixtureV1) => void;
}): ReactElement {
  const idSet = reactHostPort.useMemo(() => new Set(handleIds), [handleIds]);
  const handles = reactHostPort.useMemo(() => handleIds.map((id) => findHandle(fixture, id)).filter((h): h is Puzzle2dFixtureHandleV1 => Boolean(h)), [fixture, handleIds]);
  const angles = handles.map((h) => h.angle);
  const angleUniform = allEqual(angles);
  const angleValue = angleUniform ? angles[0]! : 0;
  const radii = handles.map((h) => h.radius ?? 8);
  const radiusUniform = allEqual(radii);
  const radiusValue = radiusUniform ? radii[0]! : Number.NaN;

  const iconKinds = handles.map((h) => h.iconKind ?? "");
  const iconKindUniform = allEqual(iconKinds);
  const iconKindValue = iconKindUniform ? (iconKinds[0] ?? "") : "";

  const handleKinds = handles.map((h) => h.handleKind);
  const handleKindUniform = allEqual(handleKinds);
  const handleKindValue = handleKindUniform ? (handleKinds[0] ?? "") : "";
  const handleKindItems = reactHostPort.useMemo(
    () => puzzle2dInspectorKindSelectItems(kindCatalogs.handles, handleKinds, (kindId) => puzzle2dHandleKindOverlayLabel(kindId, kindCatalogs)),
    [handleKinds, kindCatalogs],
  );

  const patchHandles = reactHostPort.useCallback(
    (updater: (h: Puzzle2dFixtureHandleV1) => Puzzle2dFixtureHandleV1) => {
      patchFixture((prev) => ({
        ...prev,
        nodes: prev.nodes.map((node) => ({
          ...node,
          handles: node.handles.map((h) => (idSet.has(h.id) ? updater(h) : h)),
        })),
      }));
    },
    [idSet, patchFixture],
  );

  const onIconKind = reactHostPort.useCallback(
    (next: string) => {
      const t = next.trim();
      patchHandles((h) => ({ ...h, ...(t === "" ? { iconKind: undefined } : { iconKind: t }) }));
    },
    [patchHandles],
  );

  const onHandleKind = reactHostPort.useCallback(
    (next: string) => {
      const trimmed = next.trim();
      if (trimmed === "") {
        return;
      }
      patchHandles((h) => ({ ...h, handleKind: trimmed }));
    },
    [patchHandles],
  );

  const ringParentNodes = reactHostPort.useMemo(
    () =>
      handles
        .map((h) => findHandleOwner(fixture, h.id)?.node)
        .filter((n): n is Puzzle2dFixtureNodeV1 => Boolean(n)),
    [fixture, handles],
  );
  const ringParentShapes = ringParentNodes.map((n) => n.shape ?? "circle");
  const ringParentShapeUniform = allEqual(ringParentShapes);
  const ringParentNode = ringParentShapeUniform ? ringParentNodes[0] : undefined;
  const ringEnabled = angleUniform && ringParentNode !== undefined;
  const ringOrbT = ringEnabled ? puzzle2dHandleAngleToRingT(ringParentNode, angleValue) : 0;

  const onRingOrbChange = reactHostPort.useCallback(
    (_orbId: string, _oldT: number, newT: number) => {
      if (!ringParentNode) {
        return;
      }
      const next = normalizeAngleRad(puzzle2dHandleAngleFromRingT(ringParentNode, newT));
      patchHandles((h) => ({ ...h, angle: next }));
    },
    [patchHandles, ringParentNode],
  );

  return (
    <div className="border-normal/60 space-y-3 border-l pl-2">
      <InspectorKindSelect
        id="puzzle-2d-play.inspector.handle.kind"
        items={handleKindItems}
        label="Handle kind"
        onValueChange={onHandleKind}
        uniform={handleKindUniform}
        value={handleKindValue}
      />
      <Label id="puzzle-2d-play.inspector.handle.t.ring" label="t">
        <Ring
          id="puzzle-2d-play.inspector.handle.t.ring.control"
          onOrbChange={onRingOrbChange}
          orbs={[{ disabled: !ringEnabled, id: "angle", selected: true, t: ringOrbT }]}
        />
      </Label>
      <NumericStepperRow
        id="puzzle-2d-play.inspector.handle.t"
        label="t (rad)"
        onAbsolute={(v) => patchHandles((h) => ({ ...h, angle: normalizeAngleRad(v) }))}
        onDelta={(d) => patchHandles((h) => ({ ...h, angle: normalizeAngleRad(h.angle + d) }))}
        step={0.05}
        uniform={angleUniform}
        value={angleUniform ? angleValue : Number.NaN}
      />
      <NumericStepperRow
        id="puzzle-2d-play.inspector.handle.radius"
        label="Hit radius"
        onAbsolute={(v) => patchHandles((h) => ({ ...h, radius: Math.max(1e-6, v) }))}
        onDelta={(d) => patchHandles((h) => ({ ...h, radius: Math.max(1e-6, (h.radius ?? 8) + d) }))}
        step={1}
        uniform={radiusUniform}
        value={radiusValue}
      />
      <Label id="puzzle-2d-play.inspector.handle.icon" label="Icon">
        <IconSelector classifyPuzzle2dIconSelectorMode={classifyPuzzle2dIconSelectorMode} id="puzzle-2d-play.inspector.handle.icon.selector" onChange={onIconKind} uniform={iconKindUniform} value={iconKindValue} />
      </Label>
    </div>
  );
}

/** @emoji 🪢 Batch edge inspector: endpoints and id (single). */
function InspectorEdgeBatch({
  fixture,
  edgeIds,
  kindCatalogs,
  patchFixture,
}: {
  fixture: Puzzle2dFixtureV1;
  edgeIds: readonly string[];
  kindCatalogs: KindCatalogBundle;
  patchFixture: (updater: (prev: Puzzle2dFixtureV1) => Puzzle2dFixtureV1) => void;
}): ReactElement {
  const idSet = reactHostPort.useMemo(() => new Set(edgeIds), [edgeIds]);
  const edges = reactHostPort.useMemo(() => edgeIds.map((id) => findEdge(fixture, id)).filter((e): e is Puzzle2dFixtureEdgeV1 => Boolean(e)), [edgeIds, fixture]);
  const sources = edges.map((e) => e.source);
  const targets = edges.map((e) => e.target);
  const sourceUniform = allEqual(sources);
  const targetUniform = allEqual(targets);
  const handleOptions = reactHostPort.useMemo(
    () => (PUZZLE_2D_PLAY_IS_WIRES ? fixture.nodes.map((node) => node.id) : listHandleIds(fixture)),
    [fixture],
  );
  const endpointLabel = reactHostPort.useCallback(
    (endpointId: string) =>
      PUZZLE_2D_PLAY_IS_WIRES ? (wiresPlayIdentityLabelForNodeId(endpointId) ?? endpointId) : puzzle2dFixtureHandleEndpointDisplayLabel(endpointId, fixture, kindCatalogs),
    [fixture, kindCatalogs],
  );
  const edgeKinds = edges.map((e) => e.edgeKind ?? "");
  const edgeKindUniform = allEqual(edgeKinds);
  const edgeKindValue = edgeKindUniform ? (edgeKinds[0] ?? "") : "";
  const edgeKindItems = reactHostPort.useMemo(
    () => puzzle2dInspectorKindSelectItems(kindCatalogs.edges, edgeKinds, (kindId) => puzzle2dEdgeKindOverlayLabel(kindId, kindCatalogs)),
    [edgeKinds, kindCatalogs],
  );
  const wiresRelationshipKinds = reactHostPort.useMemo(
    () => edges.map((edge) => wiresPlayRelationshipKindDisplayName(edge.id) ?? ""),
    [edges],
  );
  const wiresRelationshipKindUniform = allEqual(wiresRelationshipKinds);
  const wiresRelationshipKindValue = wiresRelationshipKindUniform ? (wiresRelationshipKinds[0] ?? "") : "";

  const patchEdges = reactHostPort.useCallback(
    (updater: (e: Puzzle2dFixtureEdgeV1) => Puzzle2dFixtureEdgeV1) => {
      patchFixture((prev) => ({
        ...prev,
        edges: prev.edges.map((e) => (idSet.has(e.id) ? updater(e) : e)),
      }));
    },
    [idSet, patchFixture],
  );

  const onEdgeKind = reactHostPort.useCallback(
    (next: string) => {
      const trimmed = next.trim();
      patchEdges((edge) => {
        if (trimmed === "") {
          const { edgeKind: _drop, ...rest } = edge;
          return rest;
        }
        return { ...edge, edgeKind: trimmed };
      });
    },
    [patchEdges],
  );

  return (
    <div className="border-normal/60 space-y-3 border-l pl-2">
      {PUZZLE_2D_PLAY_IS_WIRES ? (
        <Label id="puzzle-2d-play.inspector.edge.relationship-kind" label="Relationship kind">
          <Input className="h-7 font-mono text-xs" readOnly value={wiresRelationshipKindUniform ? wiresRelationshipKindValue : "Mixed"} />
        </Label>
      ) : (
        <InspectorKindSelect
          id="puzzle-2d-play.inspector.edge.kind"
          items={edgeKindItems}
          label="Edge kind"
          onValueChange={onEdgeKind}
          uniform={edgeKindUniform}
          value={edgeKindValue}
        />
      )}
      <Label id="puzzle-2d-play.inspector.edge.source" label={PUZZLE_2D_PLAY_IS_WIRES ? "From identity" : "Source"}>
        <Select
          onValueChange={(v) => {
            patchEdges((e) => ({ ...e, source: v }));
          }}
          value={sourceUniform ? sources[0] : undefined}
        >
          <SelectTrigger className="h-7 font-mono text-xs">
            <SelectValue placeholder={sourceUniform ? undefined : "Mixed"} />
          </SelectTrigger>
          <SelectContent>
            {handleOptions.map((hid) => (
              <SelectItem key={hid} value={hid}>
                {endpointLabel(hid)}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </Label>
      <Label id="puzzle-2d-play.inspector.edge.target" label={PUZZLE_2D_PLAY_IS_WIRES ? "To identity" : "Target"}>
        <Select
          onValueChange={(v) => {
            patchEdges((e) => ({ ...e, target: v }));
          }}
          value={targetUniform ? targets[0] : undefined}
        >
          <SelectTrigger className="h-7 font-mono text-xs">
            <SelectValue placeholder={targetUniform ? undefined : "Mixed"} />
          </SelectTrigger>
          <SelectContent>
            {handleOptions.map((hid) => (
              <SelectItem key={`target-${hid}`} value={hid}>
                {endpointLabel(hid)}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </Label>
    </div>
  );
}

function classifyPuzzle2dPlayInspectorSelection(fixture: Puzzle2dFixtureV1, selectionIds: ReadonlySet<string>): {
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

/** @emoji 🔎 Playground tree inspector sections for the active selection (up to three kind sections). */
export function buildPuzzle2dPlayInspectorSections(
  fixture: Puzzle2dFixtureV1,
  selectionIds: ReadonlySet<string>,
  patchFixture: (updater: (prev: Puzzle2dFixtureV1) => Puzzle2dFixtureV1) => void,
): TreeDataSection[] {
  const kindCatalogs = puzzle2dFixtureMergedKindCatalogs(fixture);
  const { nodeIds, handleIds, edgeIds, unknownIds } = classifyPuzzle2dPlayInspectorSelection(fixture, selectionIds);
  if (nodeIds.length === 0 && handleIds.length === 0 && edgeIds.length === 0 && unknownIds.length === 0) {
    return [
      playgroundPanelSection(
        "puzzle-2d-play-inspector.empty",
        "Detail",
        <p className="text-muted-foreground leading-snug">
          {PUZZLE_2D_PLAY_IS_WIRES
            ? "No selection. Click the graph or pick an identity or relationship in the hierarchy."
            : "No selection. Click the graph or pick a row in the hierarchy."}
        </p>,
      ),
    ];
  }
  const sections: TreeDataSection[] = [];
  if (nodeIds.length > 0) {
    sections.push(
      playgroundPanelSection(
        "puzzle-2d-play-inspector-nodes",
        PUZZLE_2D_PLAY_IS_WIRES
          ? nodeIds.length === 1
            ? "Identity"
            : "Identities"
          : puzzle2dPlayInspectorKindSectionLabel("node", nodeIds.length),
        <InspectorNodeBatch fixture={fixture} kindCatalogs={kindCatalogs} nodeIds={nodeIds} patchFixture={patchFixture} />,
      ),
    );
  }
  if (handleIds.length > 0) {
    sections.push(
      playgroundPanelSection(
        "puzzle-2d-play-inspector-handles",
        puzzle2dPlayInspectorKindSectionLabel("handle", handleIds.length),
        <InspectorHandleBatch fixture={fixture} kindCatalogs={kindCatalogs} handleIds={handleIds} patchFixture={patchFixture} />,
      ),
    );
  }
  if (edgeIds.length > 0) {
    sections.push(
      playgroundPanelSection(
        "puzzle-2d-play-inspector-edges",
        PUZZLE_2D_PLAY_IS_WIRES
          ? edgeIds.length === 1
            ? "Relationship"
            : "Relationships"
          : puzzle2dPlayInspectorKindSectionLabel("edge", edgeIds.length),
        <InspectorEdgeBatch edgeIds={edgeIds} fixture={fixture} kindCatalogs={kindCatalogs} patchFixture={patchFixture} />,
      ),
    );
  }
  if (unknownIds.length > 0) {
    sections.push(
      playgroundPanelSection(
        "puzzle-2d-play-inspector-unknown",
        "Selection",
        <p className="text-[11px] text-warning-foreground leading-snug">{unknownIds.map((id) => puzzle2dFixtureObjectDisplayLabel(id, fixture, kindCatalogs)).join(", ")}</p>,
      ),
    );
  }
  return sections;
}

/** @emoji 🔎 Details side panel bound to play fixture + selection (reacts to context, not shell generation). */
function Puzzle2dPlayInspectorPanel(): ReactElement {
  const { fixture, patchFixture } = usePuzzle2dPlayShell();
  const { selectionIds } = usePuzzle2dPlaySelection();
  const sections = reactHostPort.useMemo(
    () => buildPuzzle2dPlayInspectorSections(fixture, selectionIds, patchFixture),
    [fixture, patchFixture, selectionIds],
  );
  return <Tree className="min-h-0 min-w-0 flex-1 overflow-y-auto overflow-x-hidden" sections={sections} />;
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
const initialFixture = clonePuzzle2dFixtureV1(puzzle2dPlayResolvedDefaultFixture());

const PUZZLE_2D_PLAY_NAVBAR_FIXTURE_OPTIONS = PUZZLE_2D_PLAY_IS_WIRES
  ? WIRES_PLAY_FIXTURE_OPTIONS
  : [...PUZZLE_2D_PLAY_FIXTURE_OPTIONS, { id: WIRES_PLAY_FIXTURE_METABOLISM_ID, label: "Metabolism (WIRES)" }];

const PUZZLE_2D_PLAY_NAVBAR_FIXTURE_DEFAULT_ID = PUZZLE_2D_PLAY_IS_WIRES ? WIRES_PLAY_FIXTURE_METABOLISM_ID : PUZZLE_2D_PLAY_FIXTURE_NAKAGIN_ID;

function puzzle2dPlayFixtureForNavbarId(fixtureId: string): Puzzle2dFixtureV1 {
  if (isPlaygroundNoFixtureId(fixtureId)) {
    return clonePuzzle2dFixtureV1(PUZZLE_2D_PLAY_EMPTY_FIXTURE);
  }
  if (fixtureId === WIRES_PLAY_FIXTURE_METABOLISM_ID) {
    return clonePuzzle2dFixtureV1(WIRES_PLAY_DEFAULT_FIXTURE);
  }
  return clonePuzzle2dFixtureV1(PUZZLE_2D_PLAY_DEFAULT_FIXTURE);
}

function Puzzle2dPlayInner({
  puzzle2dRuntime,
  playgroundKeybindings,
}: {
  readonly puzzle2dRuntime: Platform;
  readonly playgroundKeybindings?: readonly import("@framework/playground/core").PlaygroundKeybinding[];
}): ReactElement {
  const [activeFixtureId, setActiveFixtureId] = reactHostPort.useState(PUZZLE_2D_PLAY_NAVBAR_FIXTURE_DEFAULT_ID);
  const [fixture, setFixtureState] = reactHostPort.useState<Puzzle2dFixtureV1>(() => clonePuzzle2dFixtureV1(initialFixture));
  const fixtureRef = reactHostPort.useRef<Puzzle2dFixtureV1>(fixture);
  fixtureRef.current = fixture;
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
  const [puzzle2dBrushFlushDistance, setPuzzle2dBrushFlushDistance] = reactHostPort.useState(DEFAULT_PUZZLE_2D_BRUSH_FLUSH_DISTANCE_PX);
  const puzzle2dFillBaseFixtureRef = reactHostPort.useRef<Puzzle2dFixtureV1 | null>(null);
  const puzzle2dFillSequenceRef = reactHostPort.useRef<Puzzle2dBrushPlacePayload[]>([]);
  const puzzle2dFillSeedRef = reactHostPort.useRef(0);
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

  const preparePuzzle2dFillSession = reactHostPort.useCallback((base: Puzzle2dFixtureV1) => {
    puzzle2dFillBaseFixtureRef.current = clonePuzzle2dFixtureV1(base);
    puzzle2dFillSeedRef.current = (Date.now() ^ Math.floor(Math.random() * 0x7fffffff)) >>> 0;
    const renderer = puzzle2dActiveRenderer();
    puzzle2dFillSequenceRef.current =
      renderer?.computeBrushFillSequence(puzzle2dFillBaseFixtureRef.current, 1000, puzzle2dFillSeedRef.current) ?? [];
    console.log("[DEBUG] puzzle2d fill sequence length", puzzle2dFillSequenceRef.current.length, "seed", puzzle2dFillSeedRef.current);
  }, []);

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

  const setFixture = reactHostPort.useCallback((next: Puzzle2dFixtureV1) => {
    guardFixtureAuthoringFromStructuralDeletes(120);
    setFixtureState(next);
    bumpSceneAuthoringEpoch();
    setSelectionIdsState(selectionSeedForFixture(next));
    setPreselection(PUZZLE_2D_PRESELECT_EMPTY);
    setHoveredId(null);
    hoverSourcePaneRef.current = null;
    setHoverSourcePane(null);
    setPuzzle2dPlayPaneCamerasBaseline(triptychCamerasFromFixture(next));
  }, [bumpSceneAuthoringEpoch, guardFixtureAuthoringFromStructuralDeletes]);

  const patchFixture = reactHostPort.useCallback(
    (updater: (prev: Puzzle2dFixtureV1) => Puzzle2dFixtureV1) => {
      guardFixtureAuthoringFromStructuralDeletes(80);
      setFixtureState((prev) => updater(prev));
      bumpSceneAuthoringEpoch();
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
    if (puzzle2dActiveTool !== "brush") {
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

  const cameraBasisFixtureRef = reactHostPort.useRef<Puzzle2dFixtureV1>(fixture);
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
    const target = triptychCamerasFromFixture(fixture);
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
  }, [puzzle2dRedrawPlaying, fixture]);

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
    const to = triptychCamerasFromFixture(snapshotFixture);
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
          const fit = triptychCamerasFromFixture(fixtureRef.current);
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
    const to = triptychCamerasFromFixture(snapshotFixture);
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
    patchFixture((prev) => {
      const laidOut = puzzle2dPlayFixtureWithDragAnchors(
        layoutPuzzle2dFixtureRedrawNodes(
          prev,
          puzzle2dPlayRedrawLayoutOpts(
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
          ),
        ),
        dragAnchors,
      );
      puzzle2dSyncLayoutNodePositionsToAllAuthoringPeers(laidOut);
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
        const laidOut = puzzle2dPlayFixtureWithDragAnchors(cur, dragAnchors);
        puzzle2dSyncLayoutNodePositionsToAllAuthoringPeers(laidOut);
        return laidOut;
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
      puzzle2dBrushFlushDistance,
      setPuzzle2dBrushFlushDistance,
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
      puzzle2dBrushFlushDistance,
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
    camerasByPane: puzzle2dPlayInitialCameras(),
    patchFixture: (_updater: (prev: Puzzle2dFixtureV1) => Puzzle2dFixtureV1) => {},
    setPuzzle2dGridSnapEnabled: (_value: boolean | ((prev: boolean) => boolean)) => {},
    setPuzzle2dRedrawPlaying: (_value: boolean | ((prev: boolean) => boolean)) => {},
    setPuzzle2dSelectionMethod: (_value: Puzzle2dSelectionMethod) => {},
    setPuzzle2dSelectionMode: (_value: Puzzle2dSelectionMode) => {},
    setPuzzle2dSelectionTargets: (_value: Puzzle2dSelectionTargets | ((prev: Puzzle2dSelectionTargets) => Puzzle2dSelectionTargets)) => {},
    setSelectionIds: (_ids: readonly string[]) => {},
  });
  puzzle2dPlayToolbarHostRef.current = {
    activePaneId,
    applyPuzzle2dRedrawHandlesOnce,
    camerasByPane,
    patchFixture,
    setPuzzle2dGridSnapEnabled,
    setPuzzle2dRedrawPlaying,
    setPuzzle2dSelectionMethod,
    setPuzzle2dSelectionMode,
    setPuzzle2dSelectionTargets,
    setSelectionIds,
  };

  reactHostPort.useEffect(() => {
    if (!puzzle2dShellController) {
      return;
    }
    const bridge: Puzzle2dPlayHostBridge = {
      getToolbarState: () => ({
        puzzle2dActiveTool,
        puzzle2dBrushFlushDistance,
        puzzle2dGridSnapEnabled,
        puzzle2dRedrawPlaying,
        puzzle2dSelectionMethod,
        puzzle2dSelectionMode,
        puzzle2dSelectionTargets,
      }),
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
            const node: Puzzle2dFixtureCircleNodeV1 = {
              handles: [{ angle: 0, handleKind: BUILTIN_PORT_HANDLE_KIND, id: handleId }],
              id,
              radius: PUZZLE_2D_PLAY_DEFAULT_NODE_SIZE_PX / 2,
              x: camera.x,
              y: camera.y,
            };
            h.patchFixture((prev) => ({ ...prev, nodes: [...prev.nodes, node] }));
            h.setSelectionIds([id]);
            break;
          }
          case "appendRectangle": {
            const camera = h.camerasByPane[h.activePaneId];
            const id = newPuzzle2dAuthoringId("node");
            const handleId = `${id}.h0`;
            const d = PUZZLE_2D_PLAY_DEFAULT_NODE_SIZE_PX;
            const node: Puzzle2dFixtureRectangleNodeV1 = {
              handles: [{ angle: 0, handleKind: BUILTIN_PORT_HANDLE_KIND, id: handleId }],
              height: d,
              id,
              shape: "rectangle",
              width: d,
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
              preparePuzzle2dFillSession(fixture);
              puzzle2dShellController?.setBrushEngagementPossibles([]);
            } else if (prev === "fill" && tool !== "fill") {
              const base = puzzle2dFillBaseFixtureRef.current;
              if (base) {
                patchFixture(() => clonePuzzle2dFixtureV1(base));
              }
              puzzle2dFillBaseFixtureRef.current = null;
              puzzle2dFillSequenceRef.current = [];
            }
            break;
          }
          case "setFillCount": {
            const { count } = args as { count?: number };
            const n = Math.max(0, Math.min(1000, Math.round(Number(count) ?? 0)));
            const base = puzzle2dFillBaseFixtureRef.current;
            if (!base) {
              break;
            }
            const prefix = puzzle2dFillSequenceRef.current.slice(0, n);
            const catalogs = puzzle2dFixtureMergedKindCatalogs(fixture);
            patchFixture(() => applyBrushFillPlacementsToFixture(base, prefix, catalogs));
            console.log("[DEBUG] puzzle2d fill count", n, "applied", prefix.length);
            break;
          }
          case "setBrushFlushDistance":
            setPuzzle2dBrushFlushDistance((args as { distance: number }).distance);
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
    puzzle2dBrushFlushDistance,
    puzzle2dGridSnapEnabled,
    puzzle2dRedrawPlaying,
    puzzle2dSelectionMethod,
    puzzle2dSelectionMode,
    puzzle2dSelectionTargets,
    puzzle2dShellController,
    preparePuzzle2dFillSession,
    fixture,
    patchFixture,
    setPuzzle2dActiveTool,
    setPuzzle2dBrushFlushDistance,
  ]);
  // #endregion 🔖ToolbarHostBridge

  const puzzle2dPlayHierarchyPanel = reactHostPort.useMemo(() => new Puzzle2dPlayHierarchyPanelDefinition(), []);
  const puzzle2dPlayKindsPanel = reactHostPort.useMemo(() => new Puzzle2dPlayKindsPanelDefinition(), []);
  const puzzle2dPlaySettingsPanel = reactHostPort.useMemo(() => new Puzzle2dPlaySettingsPanelDefinition(), []);
  const puzzle2dPlayInspectorPanel = reactHostPort.useMemo(() => new Puzzle2dPlayInspectorPanelDefinition(), []);
  const augmentPanelTabs = reactHostPort.useMemo(
    () => ({
      workbench: [puzzle2dPlayHierarchyPanel, puzzle2dPlayKindsPanel],
      details: [puzzle2dPlayInspectorPanel, puzzle2dPlaySettingsPanel],
    }),
    [puzzle2dPlayHierarchyPanel, puzzle2dPlayKindsPanel, puzzle2dPlayInspectorPanel, puzzle2dPlaySettingsPanel],
  );

  const applyNavbarFixtureId = reactHostPort.useCallback(
    (fixtureId: string) => {
      const nextId = isPlaygroundNoFixtureId(fixtureId) ? PLAYGROUND_NO_FIXTURE_ID : fixtureId;
      if (nextId === activeFixtureId) return;
      setActiveFixtureId(nextId);
      const next = puzzle2dPlayFixtureForNavbarId(nextId);
      setFixtureState(next);
      setSelectionIdsState(isPlaygroundNoFixtureId(nextId) ? new Set() : selectionSeedForFixture(next));
      setPuzzle2dPlayPaneCamerasBaseline(triptychCamerasFromFixture(next));
      puzzle2dSyncFixtureDescriptorToAllAuthoringPeers(next);
      bumpSceneAuthoringEpoch();
    },
    [activeFixtureId, bumpSceneAuthoringEpoch],
  );

  const slotNavbarCenter = reactHostPort.useMemo(
    () => (
      <NavbarFixtureSelect
        id="puzzle2d.play.fixture"
        value={activeFixtureId}
        options={PUZZLE_2D_PLAY_NAVBAR_FIXTURE_OPTIONS}
        onValueChange={applyNavbarFixtureId}
      />
    ),
    [activeFixtureId, applyNavbarFixtureId],
  );

  puzzle2dPlayRuntimeRef.current = puzzle2dRuntime;
  puzzle2dPlayShellRef.current = shellValue;
  puzzle2dPlaySelectionRef.current = selectionValue;
  reactHostPort.useEffect(
    () => () => {
      puzzle2dPlayShellRef.current = null;
      puzzle2dPlaySelectionRef.current = null;
      puzzle2dPlayRuntimeRef.current = null;
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
import {
  GIS_MAP_PLAY_APP_ID,
  GIS_MAP_PLAY_BODY_KEY_MAIN,
  GIS_MAP_PLAY_CONTROLLER_ID,
  GIS_MAP_PLAY_IDLE_SNAPSHOT,
  GIS_MAP_PLAY_STORE_ID,
  GIS_MAP_PLAY_SURFACE_ID,
  GIS_MAP_PLAY_WINDOW_KIND_ID,
  buildMapPlayMainDeclarativeBody,
  type MapPlayController,
} from "@gis/map/play";
import { MapCanvas, Position, Route, type GisMapLodId } from "@gis/map/react";
import type { UiGisMapHostSurfaceNode } from "@framework/platform/core";

let mapPlayChromeRegistered = false;

function useMapPlayController(): MapPlayController | undefined {
  const { runtime } = useApp();
  return runtime.getActiveApp()?.controller as MapPlayController | undefined;
}

function useMapPlaySnapshot() {
  const ctrl = useMapPlayController();
  return useControllerStore(ctrl, GIS_MAP_PLAY_STORE_ID) ?? GIS_MAP_PLAY_IDLE_SNAPSHOT;
}

function MapPlayPaneSurfaceHost({ node: _node }: { readonly node: UiGisMapHostSurfaceNode }): ReactElement {
  const shellInstance = useShellWindowInstance();
  const scopeId = shellWindowScopeId(shellInstance, GIS_MAP_PLAY_WINDOW_KIND_ID);
  const ctrl = useMapPlayController();
  const snapshot = useMapPlaySnapshot();
  const activeFixture = snapshot.activeFixture ?? ctrl?.getActiveFixture() ?? null;
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

function MapPlayChrome({ runtime }: { readonly runtime: Platform }): ReactElement {
  return <PlaygroundView runtime={runtime} defaultAppId={GIS_MAP_PLAY_APP_ID} />;
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

//#region 🔖PresentationPlayHost
import {
	PRESENTATION_PLAY_BODY_KEY_MAIN,
	PRESENTATION_PLAY_CONTROLLER_ID,
	PRESENTATION_PLAY_ICON_DETAILS,
	PRESENTATION_PLAY_ICON_HIERARCHY,
	PRESENTATION_PLAY_IDLE_SNAPSHOT,
	PRESENTATION_PLAY_STORE_ID,
	PRESENTATION_PLAY_SURFACE_ID,
	PresentationPlayController,
	registerPresentationPlayDeclarativeBodies,
	type PresentationPlaySnapshot,
} from "@framework/presentation/play";
import {
	moveNormalizedRect,
	resizeNormalizedRect,
	NORMALIZED_RECT_MIN_FRACTION,
	FIGURE_TILE_PDF_PAGE_ASPECT,
	figureTileMediaKindFromFile,
	type FigureTileMediaKind,
	type NormalizedRectHandle,
	type DispositionPosition,
	type FigureTileSource,
} from "@framework/presentation/core";
import type { UiPanelHostSurfaceNode } from "@framework/platform/core";

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
				"border-border bg-muted/20 flex min-h-0 flex-1 flex-col items-center justify-center gap-3 rounded-md border border-dashed p-6 text-center",
				dragActive && "border-primary bg-primary/5",
			)}
			onDragLeave={onDragLeave}
			onDragOver={onDragOver}
			onDrop={onDrop}
		>
			<Icon icon="image-up" size="large" className="text-muted-foreground" />
			<div className="flex flex-col gap-1">
				<p className="text-sm font-medium">Pick figure media</p>
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
								<span className="bg-background/80 pointer-events-none absolute left-0 top-0 max-w-full truncate px-1 text-[10px]">{tile.name}</span>
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

//#region 🔖Boot
import type { Playground } from "@framework/playground/core";

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

  describe("PlaygroundView shell notify", () => {
    it("panel visibility uses chrome generation without bumping data generation", () => {
      const runtime = new Platform({ id: "p", name: "P" });
      const dataGen = runtime.generation;
      runtime.setPanelVisibility({ leftSidePanel: true, rightSidePanel: false });
      expect(runtime.generation).toBe(dataGen);
      expect(runtime.chromeGeneration).toBeGreaterThan(0);
    });

    it("renders display panel toggle with layout-grid icon when app has window kinds", async () => {
      const { renderToStaticMarkup } = await import("react-dom/server");
      const { AppRuntime, Controller, createTabStackLayout } = await import("@framework/playground/core");
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
        { id: "workbench", iconId: "folder", panel: "workbench", order: 0, bodyKey: "playground.view.test.workbench" },
        { id: "details", iconId: "info", panel: "details", order: 0, bodyKey: "playground.view.test.details" },
      ];
      registerWindowBody("playground.view.test.main", () => <div>Main</div>);
      registerSidePanelBody("playground.view.test.workbench", () => <div data-testid="playground-view-test.workbench" />);
      registerSidePanelBody("playground.view.test.details", () => <div data-testid="playground-view-test.details" />);
      runtime.addApp(app);
      const markup = renderToStaticMarkup(
        <PlaygroundView runtime={runtime} defaultAppId="playground-view-test" initialPanelVisibility={{ leftSidePanel: true, rightSidePanel: true }} />,
      );
      expect(markup).toContain('id="ui.panelToggle.display"');
      expect(markup).not.toContain("data-missing-icon");
      expect(markup).toContain('data-icon="layout-grid"');
    });
  });

  describe("Toolbar categories", () => {
    it("lists populated categories and omits separator-only groups", () => {
      expect(
        listPopulatedToolbarViewCategories({
          save: [{ id: "save.selected", label: "Selected" }],
          filter: [{ id: "sep", kind: "separator" }],
        }),
      ).toEqual(["save"]);
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

  describe("playgroundPanelSection", () => {
    it("wraps panel bodies in a tree item control", () => {
      const section = playgroundPanelSection("panel.test", "Test", <span data-testid="body">x</span>);
      expect(section.items?.length).toBe(1);
      expect(section.items?.[0]?.control).toBeTruthy();
    });
  });

  describe("puzzle 2d play cameras", () => {
    it("imports puzzle 2d camera zoom limits used by host clamping", async () => {
      const { PUZZLE_2D_CAMERA_ZOOM_MIN, PUZZLE_2D_CAMERA_ZOOM_MAX } = await import("@puzzle/2d/react");
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
      const { buildPuzzle2dWindowBody } = await import("@framework/playground/core");
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
      const { buildMapWindowBody } = await import("@framework/playground/core");
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
  });

}
