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
  useElementsSurfaceChrome,
  useMediaQuery,
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
  IconSelector,
  useNativeDragAndDrop,
  usePointerDrag,
  type ContextMenuItem,
  type UiTranslationKey,
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
import type { LucideIcon } from "lucide-react";
import { ClipboardList, Folder, Info, Library, ListTree, Settings, Tags } from "lucide-react";
import * as React from "react";
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
  getSidePanelBodyMount,
  type UiPuzzle3dHostSurfaceNode,
  type UiTableHostSurfaceNode,
  enforcePlaygroundWindowEngagementInput,
  enforceWindowKindsEngagementInput,
  type WindowBodyViewContext,
  type WindowEngagement,
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
  declareToolsToViewTools,
  findDefaultActiveWindowKindId,
  listPopulatedToolbarViewCategories,
  mergePlatformFooterChromeRows,
  registerSurfaceBinding,
  renderComponentHostSurface,
  unregisterSurfaceBinding,
  UIToolbar,
  useControllerStore,
  useStore,
  windowMeasuresToGolden,
  type UiComponentHostSurfaceNode,
  type UIWindowMeasure,
} from "@framework/platform/renderer/react";

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
      throw new Error(`Playground tree item "${path}/${item.id}" must not use a React description; use section.content with playgroundPanelSection().`);
    }
    if (item.items?.length) {
      enforcePlaygroundTreeItemsNoReactDescription(item.items, `${path}/${item.id}`);
    }
  }
}

/** @emoji 🌲 Enforces playground panels: each section needs `items` and/or `content` (no JSON-only fallbacks). */
export function enforcePlaygroundTreePanel(config: TreePanelConfig): void {
  if (!config.sections?.length) {
    throw new Error("Playground tree panel must declare at least one section.");
  }
  for (const section of config.sections) {
    const hasItems = Boolean(section.items?.length);
    const hasContent = section.content != null;
    if (!hasItems && !hasContent) {
      throw new Error(`Playground tree section "${section.id}" must declare items or content.`);
    }
    if (section.items?.length) {
      enforcePlaygroundTreeItemsNoReactDescription(section.items, section.id);
    }
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

/** @emoji 🌲 Tree panel that rebuilds sections when the builder returns a new section list. */
export class CallbackTreePanelDefinition implements TreePanelDefinition {
  private resolved: TreePanelConfig | null = null;
  private resolvedSections: TreeDataSection[] | null = null;
  private resolvedHighlightedIds: readonly string[] | null = null;

  constructor(
    private readonly buildSections: () => TreeDataSection[],
    private readonly buildHighlightedIds: () => readonly string[] = () => [],
  ) {}

  resolveTree(): TreePanelConfig {
    const sections = this.buildSections();
    const highlightedIds = this.buildHighlightedIds();
    if (this.resolved && this.resolvedSections === sections && this.resolvedHighlightedIds === highlightedIds) {
      return this.resolved;
    }
    const config: TreePanelConfig = { sections, highlightedIds };
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
const tableSurfaceHosts = new Map<string, TableSurfaceHost>();

const PLAYGROUND_CANVAS_HOST_TYPES = new Set(["puzzle2d", "puzzle3d", "puzzle5d", "cad"]);

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

function uiTreeItemsToTreeData(items: readonly UiTreeItemNode[], commandBus: CommandBus): TreeDataItem[] {
  return items.map((item) => ({
    id: item.id,
    label: item.label,
    description: item.description,
    defaultOpen: item.defaultOpen,
    isSelected: item.selected,
    draggable: item.draggable,
    dragData: item.dragData,
    className: item.draggable || item.dragData ? "cursor-grab active:cursor-grabbing" : undefined,
    items: item.items?.length ? uiTreeItemsToTreeData(item.items, commandBus) : undefined,
    onClick: item.command
      ? () => {
          dispatchUiCommand(commandBus, item.command!, {});
        }
      : undefined,
    onPointerEnter: item.onPointerEnter,
    onPointerLeave: item.onPointerLeave,
  }));
}

function buildUiTreeDragAndDropController(sections: readonly UiTreeSectionNode[], commandBus: CommandBus): TreeDragAndDropController | undefined {
  void commandBus;
  const dragByItemId = collectUiTreeItemDragData(sections);
  if (dragByItemId.size === 0) {
    return undefined;
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
    case "panel":
    case "table":
      return renderPlaygroundHostSurface(node, node.type === "table" || node.type === "panel" ? "panel" : "canvas");
    case "section": {
      const section = node as UiSectionNode;
      return (
        <div className="border-element/60 flex flex-col gap-single rounded-md border p-single" data-ui-section={section.id}>
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
const shellTabIcons = new Map<string, LucideIcon>();

/** @emoji 🖼 Registers a Lucide icon constructor for side-panel tab headers keyed by `iconId`. */
export function registerTabIcon(iconId: string, Icon: LucideIcon): void {
  shellTabIcons.set(iconId, Icon);
}

function shellTabIconComponent(iconId: string): React.ComponentType<{ size?: number }> {
  return function ShellResolvedTabIcon({ size = 16 }: { size?: number }) {
    const Lucide = shellTabIcons.get(iconId);
    return Lucide ? <Lucide size={size} /> : <span style={{ display: "inline-block", width: size }} data-missing-icon={iconId} />;
  };
}

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
      return <UiRenderer node={node} commandBus={runtime.commandBus} />;
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

/** @emoji 💬 Converts a React-neutral {@link WindowEngagement} into a ui {@link EngagementSpec} with bus-dispatching callbacks. */
export function windowEngagementToGolden(engagement: WindowEngagement | undefined, bus: CommandBus): EngagementSpec | undefined {
  if (!engagement) return undefined;
  const options = engagement.options?.map((option) => ({
    id: option.id,
    label: option.label,
    icon: option.iconId ? shellTabIconComponent(option.iconId)({}) : undefined,
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
  const hasContent = (options?.length ?? 0) > 0 || Boolean(input) || (status?.length ?? 0) > 0 || (possibleEngagements?.length ?? 0) > 0;
  if (!hasContent) return undefined;
  return { options, input, status, possibleEngagements };
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
export function sideTabsToPlaygroundPanelTabs(tabs: readonly SideTabSpec[], bus: CommandBus): SidePanelTabConfig[] {
  void bus;
  return tabs.map((tab, orderIndex) => {
    const declarativeFactory = getSidePanelBodyFactory(tab.bodyKey);
    if (declarativeFactory && getSidePanelBodyMount(tab.bodyKey) === "treeRoot") {
      return resolveSidePanelTabSource({
        id: tab.id,
        icon: shellTabIconComponent(tab.iconId),
        order: tab.order ?? orderIndex,
        panel: <DeclarativeTreeWorkbenchPanel tabId={tab.id} bodyKey={tab.bodyKey} />,
      });
    }
    const Body = declarativeFactory ? getDeclarativeSidePanelBodyComponent(tab.id, tab.bodyKey) : () => <PlaygroundPanelBody><div className="p-2 text-xs">Missing panel {tab.bodyKey}</div></PlaygroundPanelBody>;
    const panelBody = <Body />;
    const sectionLabel = tab.label?.trim();
    const sections: TreeDataSection[] = sectionLabel
      ? [{ id: `${tab.id}.section`, label: sectionLabel, defaultOpen: true, content: panelBody }]
      : [{ id: `${tab.id}.body`, label: "", defaultOpen: true, content: panelBody }];
    return resolveSidePanelTabSource({
      id: tab.id,
      icon: shellTabIconComponent(tab.iconId),
      order: tab.order ?? orderIndex,
      tree: staticTreePanelDefinition({ sections }),
    });
  });
}

/** @emoji 🌲 Declarative `type: "tree"` workbench tab mounted as the side-panel root (no nested shell tree). */
function DeclarativeTreeWorkbenchPanel(props: { readonly tabId: string; readonly bodyKey: string }): React.ReactElement {
  const { runtime, activeModeId } = useApp();
  reactHostPort.useSyncExternalStore(
    (listener) => runtime.subscribe(listener),
    () => runtime.generation,
    () => 0,
  );
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
  readonly extraFooterItems?: readonly FooterItem[];
  readonly augmentPanelTabs?: Partial<Record<"workbench" | "details", readonly (SidePanelTabConfig | SidePanelTabDefinition)[]>>;
  readonly onActiveWindowChange?: (windowKindId: string) => void;
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
    () => (workbenchTabs[0]?.icon ? reactHostPort.createElement(workbenchTabs[0].icon, { size: 16 }) : <Folder size={16} />),
    [workbenchTabs],
  );
  const detailsIcon = reactHostPort.useMemo(
    () => (detailsTabs[0]?.icon ? reactHostPort.createElement(detailsTabs[0].icon, { size: 16 }) : <Info size={16} />),
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
export const PlaygroundView: React.FC<PlaygroundViewProps> = ({ runtime, playgroundKeybindings, defaultAppId, mobile, mobileQuery = "(max-width: 767px)", initialPanelVisibility, slotToolbar, extraFooterItems, augmentPanelTabs, onActiveWindowChange }) => {
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

  if (!shell.activeAppBase || !shell.activeApp || !shell.playgroundContextValue) return null;

  const navbarItems = reactHostPort.useMemo<NavbarItem[]>(
    () => [
      {
        key: "title",
        className: "flex-1 min-w-0",
        content: <span className="truncate px-single text-sm font-medium">{shell.activeApp!.label}</span>,
      },
      {
        key: "panelToggles",
        content: (
          <div className="flex min-w-0 items-stretch border border-element h-medium">
            <Toggle
              id="ui.panelToggle.workbench"
              pressed={panelVisibility.leftSidePanel}
              onPressedChange={(pressed) => setPanelVisibility((p) => ({ ...p, leftSidePanel: pressed }))}
              icon={shell.workbenchIcon}
              className="rounded-none border-0 shrink-0"
            />
            <Toggle
              id="ui.panelToggle.details"
              pressed={panelVisibility.rightSidePanel}
              onPressedChange={(pressed) => setPanelVisibility((p) => ({ ...p, rightSidePanel: pressed }))}
              icon={shell.detailsIcon}
              className="rounded-none border-0 border-l shrink-0"
            />
          </div>
        ),
      },
    ],
    [panelVisibility.leftSidePanel, panelVisibility.rightSidePanel, setPanelVisibility, shell.activeApp, shell.detailsIcon, shell.workbenchIcon],
  );

  return (
    <PlaygroundContext.Provider value={shell.playgroundContextValue}>
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
        leftSidePanelTabs={shell.workbenchTabs}
        rightSidePanelTabs={shell.detailsTabs}
        panelVisibility={panelVisibility}
        leftPanelSize={leftPanelSize}
        onLeftPanelSizeChange={setLeftPanelSize}
        rightPanelSize={rightPanelSize}
        onRightPanelSizeChange={setRightPanelSize}
        goldenWindowKinds={shell.goldenWindowKinds}
        defaultLayout={shell.activeApp.defaultLayout}
        activeWindowKindId={shell.activeWindowKindId}
        onActiveWindowKindChange={shell.onActiveWindowKindChange}
        multiApp={false}
        activeModeId={shell.activeModeId}
        onActiveModeChange={(modeId) => {
          shell.activeAppBase!.setActiveModeId(modeId);
          runtime.notifyChrome();
        }}
      />
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

/** @emoji 🌲 Content-only tree section for playground workbench/details panels. */
export function playgroundPanelSection(id: string, label: string, body: React.ReactNode, options?: { readonly defaultOpen?: boolean }): TreeDataSection {
  return {
    id,
    label,
    defaultOpen: options?.defaultOpen ?? true,
    content: <PlaygroundPanelBody>{body}</PlaygroundPanelBody>,
  };
}
//#endregion 🔖PlaygroundShell

//#region 🔖Mount
type PlaygroundDomRoot = HTMLElement & { __playgroundRoot?: Root };

/** @emoji 🚀 Mounts an arbitrary React tree into `#root` (or `rootId`) inside {@link PlaygroundShell}. */
export function mountPlaygroundApp(element: React.ReactElement, rootId = "root"): void {
  if (typeof document === "undefined") return;
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
  mergePaletteObjectFromDrop,
  puzzle3dFixturePaletteTreeDragController,
  buildPuzzle3dPlayEngagement,
  getPuzzle3dBrushEngagementEpoch,
  puzzle3dBrushEngagementSourceRef,
  requestPuzzle3dZoomToSelection,
  subscribePuzzle3dBrushEngagementSource,
  type FixtureV1,
  type Puzzle3dFixtureDropDetail,
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
  Puzzle3dPlayShellController,
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
  return { options, input, status, possibleEngagements };
}

function Puzzle3dPlayEngagementPublisher(props: {
  readonly ctrl: Puzzle3dPlayShellController | undefined;
  readonly snap: Puzzle3dPlaySnapshot;
  readonly bus: CommandBus;
}): null {
  const { ctrl, snap, bus } = props;
  const [cmdLine, setCmdLine] = reactHostPort.useState("");
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
    bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "setActiveTool", { tool: "select" });
  }, [bus]);
  const onBrushTool = reactHostPort.useCallback(() => {
    bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "setActiveTool", { tool: "brush" });
  }, [bus]);
  const onRepeatLastEngagement = reactHostPort.useCallback(() => {
    bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "engagementRepeatLast", {});
  }, [bus]);
  const onEngagementAbort = reactHostPort.useCallback(() => {
    setCmdLine("");
    if (snap.activeTool === "brush") {
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
    [brushSource, onBrushTool, onSelectTool, onZoomToSelection, rememberEngagementRepeat, snap.activeTool],
  );
  const spec = reactHostPort.useMemo(
    () =>
      buildPuzzle3dPlayEngagement({
        activeTool: snap.activeTool,
        cmdLine,
        selectionCount,
        onCmdLineChange: setCmdLine,
        onCmdLineSubmit,
        onRepeatLast: onRepeatLastEngagement,
        onAbort: onEngagementAbort,
        onSelectTool,
        onBrushTool,
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
    [brushEngagementEpoch, brushSource, cmdLine, onBrushTool, onCmdLineSubmit, onEngagementAbort, onRepeatLastEngagement, onSelectTool, onZoomToSelection, rememberEngagementRepeat, selectionCount, snap.activeTool],
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

function Puzzle3dPlayViewportHost({ node }: { readonly node: UiPuzzle3dHostSurfaceNode }): React.ReactElement {
  const { runtime } = useApp();
  const bus = runtime.commandBus;
  const ctrl = usePuzzle3dPlayController();
  const snap = usePuzzle3dPlaySnapshot();
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
  const handleFixtureDrop = reactHostPort.useCallback(
    (detail: Puzzle3dFixtureDropDetail) => {
      const placed = mergePaletteObjectFromDrop(detail, kindCatalogs, snap.fixture);
      if (placed) {
        patchFixture((fixture) => applyPaletteObjectDropToFixture(fixture, placed));
        bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "setSelection", {
          selection: { objectIds: [placed.id], vortexIds: [], attractionIds: [] },
        });
        return;
      }
      const parsed = parseFixtureV1(detail.fixture);
      if (parsed) {
        ctrl?.patchFixture(() => parsed);
      }
    },
    [bus, ctrl, kindCatalogs, patchFixture],
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
          proximityRelocateEnabled={proximityRelocateEnabled}
          kindCatalogs={kindCatalogs}
          kindCompatibility={kindCompatibility}
          blockedVortexFullIds={blockedVortexFullIds}
          lodTag={snap.lodTag}
          lodProps={snap.lodProps}
          relocateMode={snap.relocateMode}
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
          setSelectedId={(id) => bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "setSelectedId", { id })}
          onSelect={(selection) => bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "noteSelection", selection)}
          onIndirectConnect={() => bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "noteIndirect")}
          onProximityConnect={() => bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "noteProximity")}
          onLodChange={(lod) => bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "setEffectiveLod", { lod })}
          onCamera={(camera) => ctrl?.setCamera(camera)}
          onAttractionCompatibleObjects={() => bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "noteCompatibleObjects")}
          onAttractionTargetRing={() => bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "noteTargetRing")}
          brushActive={snap.activeTool === "brush"}
          onBrushPlace={(payload) => bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "addBrushObject", payload)}
          brushPlacementCollisionTolerance={snap.brushPlacementCollisionTolerance}
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
}

let puzzle3dPlayChromeRegistered = false;

/** @emoji 🧊 Registers puzzle 3D play surface host, tab icons, and mesh preload. */
export function registerPuzzle3dPlaySurfaceHosts(): void {
  if (puzzle3dPlayChromeRegistered) return;
  puzzle3dPlayChromeRegistered = true;
  registerUiPuzzle3dSurfaceHost(PUZZLE_3D_PLAY_VIEWPORT_SURFACE_ID, Puzzle3dPlayViewportHost);
  registerTabIcon(PUZZLE_3D_PLAY_ICON_INSPECTOR, ClipboardList);
  registerTabIcon(PUZZLE_3D_PLAY_ICON_KINDS, Tags);
  registerTabIcon(PUZZLE_3D_PLAY_ICON_HIERARCHY, ListTree);
  registerTabIcon(PUZZLE_3D_PLAY_ICON_SETTINGS, Settings);
  const fixture = parseFixtureV1(nakaginPuzzle3dFixtureJson as unknown);
  if (fixture) {
    const urls = [...new Set(fixture.objects.map((object) => object.meshUrl))];
    for (const url of urls) sceneHostPort.drei.useGLTF.preload(url);
  }
}

/** @emoji 🚀 Mounts puzzle 3d play via standard {@link PlaygroundView} (bodies registered in {@link Playground3d}). */
export function mountPuzzle3dPlayChrome(playground: Playground, rootId = "root"): void {
  mountPlaygroundApp(
    <PlaygroundView runtime={playground.runtime} defaultAppId={PUZZLE_3D_PLAY_APP_ID} initialPanelVisibility={playground.initialPanelVisibility} playgroundKeybindings={playground.keybindings} />,
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
import { FiveD, StoreProvider } from "@puzzle/5d/react";
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
  buildPuzzle5d2dDeclarativeBody,
  buildPuzzle5dPlayHierarchySections,
  buildPuzzle5dPlayRuntime,
  buildPuzzle5d3dDeclarativeBody,
  type Puzzle5dPlaySnapshot,
} from "@puzzle/5d/play";
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
        <dd>{snapshot.relocateMode}</dd>
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
      icon: ListTree,
      order: 0,
      tree: new StaticTreePanelDefinition({ sections: this.buildTree().sections as TreeDataSection[] }),
    };
  }
}

class Puzzle5dPlayStatusPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: "puzzle-5d-play-status",
      icon: ClipboardList,
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
  if (node.controllerId !== PUZZLE_5D_PLAY_CONTROLLER_ID || node.surfaceId !== PUZZLE_5D_PLAY_2D_SURFACE_ID || node.paneId !== PUZZLE_5D_PLAY_2D_WINDOW_ID || !controller || !snapshot?.fixture2d || !snapshot.camera2d) {
    return <div className="p-2 text-xs text-muted-foreground">Invalid puzzle 5d 2d binding</div>;
  }
  return (
    <FiveD
      mode="2d"
      instanceId="play-2d"
      puzzle2d={{
        camera: snapshot.camera2d,
        onLodChange: (lod) => controller.commandBus.dispatch(PUZZLE_5D_PLAY_CONTROLLER_ID, "set2dLodTag", { lod }),
        onSelect: (snap) => controller.commandBus.dispatch(PUZZLE_5D_PLAY_CONTROLLER_ID, "set2dSelection", { ids: snap.ids }),
        onConnect: () => controller.commandBus.dispatch(PUZZLE_5D_PLAY_CONTROLLER_ID, "note2dConnect"),
        onProximityConnect: () => controller.commandBus.dispatch(PUZZLE_5D_PLAY_CONTROLLER_ID, "note2dProximity"),
        ...snapshot.lod2dProps,
      }}
    />
  );
}

function Puzzle5d3dSurfaceHost({ node }: { readonly node: UiPuzzle3dHostSurfaceNode }): React.ReactElement {
  const { controller, snapshot } = usePuzzle5dPlaySnapshot();
  if (node.controllerId !== PUZZLE_5D_PLAY_CONTROLLER_ID || node.surfaceId !== PUZZLE_5D_PLAY_3D_SURFACE_ID || !controller || !snapshot?.fixture3d || !snapshot.camera3d || !snapshot.fixture2d) {
    return <div className="p-2 text-xs text-muted-foreground">Invalid puzzle 5d 3d binding</div>;
  }
  const meshUrls = reactHostPort.useMemo(() => [...new Set(snapshot.fixture3d.objects.map((object) => object.meshUrl))], [snapshot.fixture3d]);
  reactHostPort.useEffect(() => {
    for (const url of meshUrls) sceneHostPort.drei.useGLTF.preload(url);
  }, [meshUrls]);
  return (
    <FiveD
      mode="3d"
      instanceId="play-3d"
      relocateMode={snapshot.relocateMode}
      puzzle3d={{
        ...snapshot.lod3dProps,
        camera: snapshot.camera3d ?? snapshot.fixture3d.camera,
        onConnect: () => controller.commandBus.dispatch(PUZZLE_5D_PLAY_CONTROLLER_ID, "note3dConnect"),
        onProximityConnect: () => controller.commandBus.dispatch(PUZZLE_5D_PLAY_CONTROLLER_ID, "note3dProximity"),
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

function Puzzle5dPlayChrome({ runtime }: { readonly runtime: Platform }): React.ReactElement {
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
  const shell = <PlaygroundView runtime={runtime} defaultAppId={PUZZLE_5D_PLAY_APP_ID} augmentPanelTabs={{ workbench: workbenchTabs, details: detailTabs }} initialPanelVisibility={{ leftSidePanel: true, rightSidePanel: true }} />;
  if (!controller) {
    return shell;
  }
  const puzzle5dBridge = controller.getStore(PUZZLE_5D_PLAY_STORE_ID) as Puzzle5dStoreBridge | undefined;
  const puzzle5dStore = puzzle5dBridge?.inner ?? controller.puzzle5dStore;
  return <StoreProvider store={puzzle5dStore}>{shell}</StoreProvider>;
}

/** @emoji 🚀 Mounts puzzle 5d play chrome for a {@link Playground}. */
export function mountPuzzle5dPlayChrome(playground: Playground, rootId = "root"): void {
  mountPlaygroundApp(<Puzzle5dPlayChrome runtime={playground.runtime} />, rootId);
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
  PUZZLE_2D_PLAY_HIERARCHY_TAB_ID,
  Puzzle2dPlayShellController,
  PUZZLE_2D_ENGAGEMENT_TOOL_BRUSH_ID,
  buildPuzzle2dPlayHierarchySections,
  puzzle2dPlayHierarchyGraphIdFromTreeItemId,
  puzzle2dPlayHierarchyTreeHighlightedIds,
  puzzle2dPlayHierarchyTreeSelectedIds,
  buildPuzzle2dPlayOverviewDeclarativeBody,
  buildPuzzle2dPlayDetailDeclarativeBody,
  buildPuzzle2dPlaySelectionDeclarativeBody,
  buildPuzzle2dPlayRuntime,
  filterPuzzle2dPlayStructuralDeleteBatch,
  puzzle2dPlayForwardsCanvasStructuralDelete,
  puzzle2dPlayRehydrateFixtureEdgesIfMissing,
  type Puzzle2dPlayHostBridge,
  type Puzzle2dPlayPaneId,
  type Puzzle2dPlayStructuralDeleteItem,
} from "@puzzle/2d/play";
import {
  mergeKindCatalogBundleByRowId,
  DEFAULT_KIND_CATALOG_BUNDLE,
  BUILTIN_PORT_HANDLE_KIND,
  PUZZLE_2D_CAMERA_ZOOM_MIN,
  PUZZLE_2D_CAMERA_ZOOM_MAX,
  PUZZLE_2D_PRESELECT_EMPTY,
  PUZZLE_2D_SELECTION_TARGETS_DEFAULT,
  fixtureMetaKindCatalogBundle,
  puzzle2dFixtureMetaKindCompatibility,
  puzzle2dFixtureNodeCaption,
  puzzle2dFixtureHandleEndpointDisplayLabel,
  puzzle2dFixtureMergedKindCatalogs,
  puzzle2dFixtureObjectDisplayLabel,
  classifyPuzzle2dIconSelectorMode,
  parsePuzzle2dFixtureV1,
  Puzzle2dCanvas,
  applyBrushPlacementToFixture,
  puzzle2dGuardBrushPlacementStructuralDeletes,
  puzzle2dIsBrushPlacementStructuralDeleteGuarded,
  puzzle2dSyncFixtureDescriptorToAllAuthoringPeers,
  puzzle2dActiveRenderer,
  DEFAULT_PUZZLE_2D_BRUSH_FLUSH_DISTANCE_PX,
  DEFAULT_PUZZLE_2D_BRUSH_NODE_SIZE_PX,
  puzzle2dSyncSelectionToAllAuthoringPeers,
  buildPuzzle2dSceneDescriptorFromFixture,
  clonePuzzle2dFixtureV1,
  puzzle2dFixtureSceneMarkers,
  type Puzzle2dStructureDeletePayload,
  encodePuzzle2dFixtureForDragV1,
  mergePaletteNodeFromDrop,
  setPuzzle2dFixtureDragDataTransfer,
  PUZZLE_2D_FIXTURE_DRAG_V1_MIME,
  PUZZLE_2D_FIXTURE_DRAG_KIND_PALETTE_NODE,
  PUZZLE_2D_LOD_MODE_AUTOMATIC,
  DEFAULT_PUZZLE_2D_LOD_ZOOM_THRESHOLDS,
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
  type KindCatalogBundle,
  type CameraState,
} from "@puzzle/2d/react";
import type { Playground } from "@framework/playground/core";
// #endregion 🔌Adapters

const PUZZLE_2D_PLAY_DEFAULT_KIND_CATALOGS = mergeKindCatalogBundleByRowId({ ...DEFAULT_KIND_CATALOG_BUNDLE }, fixtureMetaKindCatalogBundle(PUZZLE_2D_PLAY_DEFAULT_FIXTURE) ?? {});

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
  /** @emoji 🖱️ Pane that currently owns pointer hover updates for shared {@link Puzzle2dPlayShellValue.hoveredId}. */
  hoverSourcePane: Puzzle2dPlayPaneId | null;
  setHoverPane: (pane: Puzzle2dPlayPaneId) => void;
  setHoverForPane: (pane: Puzzle2dPlayPaneId, id: string | null) => void;
  clearHoverForPane: (pane: Puzzle2dPlayPaneId) => void;
  /** @emoji 🌳 Sets shared graph hover from hierarchy rows without claiming a canvas pane. */
  setHierarchyHover: (id: string | null) => void;
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
  setPuzzle2dLodModeForPane: (pane: Puzzle2dPlayPaneId, mode: Puzzle2dLodModeKind) => void;
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
  /** @emoji 📷 Writes the active pane’s imperative camera into {@link puzzle2dPlayPaneCamerasBaseline}. */
  syncBaselineFromViewportCamera: (cam: CameraState) => void;
}

/** @emoji 🌳 Workbench hierarchy bound to play fixture + selection (not static tree snapshots). */
function Puzzle2dPlayHierarchyPanel(): ReactElement {
  const { fixture, hoveredId, setHierarchyHover } = usePuzzle2dPlayShell();
  const { selectionIds, setSelectionIds } = usePuzzle2dPlaySelection();
  const onHierarchySelect = reactHostPort.useCallback((id: string) => setSelectionIds([id]), [setSelectionIds]);
  const onHierarchyHover = reactHostPort.useCallback((id: string | null) => setHierarchyHover(id), [setHierarchyHover]);
  const sections = reactHostPort.useMemo(
    () => buildPuzzle2dPlayHierarchySections(fixture, [], onHierarchySelect, undefined, { omitItemSelection: true, onHover: onHierarchyHover }).sections as TreeDataSection[],
    [fixture, onHierarchyHover, onHierarchySelect],
  );
  const treeSelectedIds = reactHostPort.useMemo(
    () => puzzle2dPlayHierarchyTreeSelectedIds(fixture, [...selectionIds]),
    [fixture, selectionIds],
  );
  const treeHighlightedIds = reactHostPort.useMemo(() => puzzle2dPlayHierarchyTreeHighlightedIds(fixture, hoveredId), [fixture, hoveredId]);
  const onTreeSelectionChange = reactHostPort.useCallback(
    (treeIds: string[]) => {
      const graphIds = treeIds.map(puzzle2dPlayHierarchyGraphIdFromTreeItemId).filter((id): id is string => id !== null);
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
      id: PUZZLE_2D_PLAY_HIERARCHY_TAB_ID,
      icon: ListTree,
      order: 0,
      tree: new StaticTreePanelDefinition({
        sections: [
          {
            id: "puzzle-2d-play-hierarchy.shell",
            defaultOpen: true,
            content: <Puzzle2dPlayHierarchyPanel />,
            items: [],
          },
        ],
      }),
    };
  }
}

class Puzzle2dPlayLibraryPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: "puzzle-2d-play-library",
      icon: Library,
      order: 1,
      tree: new StaticTreePanelDefinition({
        sections: [
          {
            id: "puzzle-2d-play-library.section",
            label: "Library",
            defaultOpen: true,
            content: <Puzzle2dFixtureLibraryPanel />,
            items: [],
          },
        ],
      }),
    };
  }
}

class Puzzle2dPlayInspectorPanelDefinition extends PureSidePanelTabDefinition {
  private cachedSections: TreeDataSection[] | null = null;
  private cacheKey = "";

  constructor(private readonly buildSections: () => TreeDataSection[]) {
    super();
  }

  private resolveSections(): TreeDataSection[] {
    const sections = this.buildSections();
    const key = sections.map((section) => `${section.id}:${section.items?.length ?? 0}`).join("|");
    if (key === this.cacheKey && this.cachedSections) {
      return this.cachedSections;
    }
    this.cacheKey = key;
    this.cachedSections = sections;
    return sections;
  }

  buildTab(): SidePanelTabConfig {
    return {
      id: "puzzle-2d-play-inspector",
      icon: ClipboardList,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => this.resolveSections()),
    };
  }
}

class Puzzle2dPlaySettingsPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: "puzzle-2d-play-settings",
      icon: Settings,
      order: 1,
      tree: new StaticTreePanelDefinition({
        sections: [
          {
            id: "puzzle-2d-play-settings.section",
            label: "Settings",
            defaultOpen: true,
            content: <Puzzle2dPlaySettingsPanel />,
            items: [],
          },
        ],
      }),
    };
  }
}

const Puzzle2dPlayShellContext = reactHostPort.createContext<Puzzle2dPlayShellValue | null>(null);

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

/** @emoji 📐 Builds {@link Puzzle2dRedrawLayoutOptions} for the active pane camera center and redraw mode. */
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
): Puzzle2dRedrawLayoutOptions {
  const cam = camerasByPane[pane];
  const cx = cam.x;
  const cy = cam.y;
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
    };
  }
  const fg: Puzzle2dForceGraphLayoutOptions = {
    centerX: cx,
    centerY: cy,
    gravity: Math.max(0, forceGravity),
    idealEdgeLength: Math.max(8, forceIdealEdge),
    iterations: Math.max(1, Math.min(5000, Math.round(forceIters))),
    repulsionStrength: Math.max(40, Math.min(120, Math.round(forceRepulsion))),
  };
  return { centerX: cx, centerY: cy, forceGraph: fg, mode: "force-graph", redrawHandlesAfter };
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
      <div className="text-muted-foreground flex shrink-0 items-center gap-2 border-b border-element pb-2">
        <Settings className="size-4 shrink-0" />
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
        <div className="text-muted-foreground border-t border-element pt-2 text-[11px] font-medium uppercase tracking-wide">Redraw handles</div>
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
        if (event.currentTarget.contains(event.relatedTarget as globalThis.Node)) {
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
  showBackgroundMenu,
}: {
  paneId: Puzzle2dPlayPaneId;
  showBackgroundMenu?: boolean;
}): ReactElement {
  const {
    activePaneId,
    patchFixture,
    queueStructuralDelete,
    puzzle2dActiveTool,
    puzzle2dBrushFlushDistance,
    puzzle2dGridSnapEnabled,
    sceneAuthoringEpoch,
    puzzle2dLodModeByPane,
    puzzle2dRedrawPlaying,
    puzzle2dSelectionMethod,
    puzzle2dSelectionMode,
    puzzle2dSelectionTargets,
    fixture,
    commitBrushPlacement,
    handleCanvasFixtureDrop,
    resetPuzzle2dRedrawProgressiveEpoch,
  } = usePuzzle2dPlayShell();
  const { camerasByPane, syncBaselineFromViewportCamera } = usePuzzle2dPlayCameras();
  const camera = camerasByPane[paneId];
  const lodProps = puzzle2dPlayLodCanvasProps(puzzle2dLodModeByPane[paneId]);
  const reportEffectiveLod = reactHostPort.useContext(Puzzle2dPlayLodRuntimeContext);
  const onLodChange = reactHostPort.useCallback((lod: Puzzle2dDrawLodKind) => reportEffectiveLod?.(paneId, lod), [paneId, reportEffectiveLod]);
  const { applyCanvasSelection } = usePuzzle2dPlayCanvasSelection();
  const onSelect = reactHostPort.useCallback((snapshot: Puzzle2dSelectionSnapshot) => applyCanvasSelection(snapshot.ids), [applyCanvasSelection]);
  const demoNodeId = fixture.nodes[0]?.id;
  const demoEdgeId = fixture.edges[0]?.id;
  const kindCompatibility = reactHostPort.useMemo(() => puzzle2dFixtureMetaKindCompatibility(fixture.meta), [fixture.meta]);
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
    (_payload: { id: string; x: number; y: number }) => {
      if (puzzle2dRedrawPlaying) {
        resetPuzzle2dRedrawProgressiveEpoch();
      }
    },
    [puzzle2dRedrawPlaying, resetPuzzle2dRedrawProgressiveEpoch],
  );
  const onCanvasDragEnd = reactHostPort.useCallback(
    (payload: { moves: Array<{ id: string; x: number; y: number }> }) => {
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
    [patchFixture],
  );
  const { notifyBrushCandidates } = usePuzzle2dPlayShell();
  return (
    <Puzzle2dPaneChrome paneId={paneId}>
      <Puzzle2dCanvas
        {...lodProps}
        declarativeSceneDescriptor={declarativeSceneDescriptor}
        onLodChange={onLodChange}
        camera={camera}
        className="min-h-0 flex-1"
        contextMenu={showBackgroundMenu ? puzzle2dPlayCanvasBackgroundMenu : undefined}
        fixtureDragDrop
        activeTool={puzzle2dActiveTool}
        brushFlushDistance={puzzle2dBrushFlushDistance}
        brushNodeSize={DEFAULT_PUZZLE_2D_BRUSH_NODE_SIZE_PX}
        gridSnapEnabled={puzzle2dGridSnapEnabled}
        kindCatalogs={PUZZLE_2D_PLAY_DEFAULT_KIND_CATALOGS}
        kindCompatibility={kindCompatibility}
        lodZoomThresholds={DEFAULT_PUZZLE_2D_LOD_ZOOM_THRESHOLDS}
        onCamera={activePaneId === paneId ? syncBaselineFromViewportCamera : undefined}
        onDelete={onCanvasDelete}
        onDrag={onCanvasDrag}
        onDragEnd={onCanvasDragEnd}
        onFixtureDrop={(d) => handleCanvasFixtureDrop(paneId, d)}
        onSelect={onSelect}
        onBrushPlace={commitBrushPlacement}
        onBrushCandidates={notifyBrushCandidates}
        sceneAuthoringEpoch={sceneAuthoringEpoch}
        selectionMethod={puzzle2dSelectionMethod}
        selectionMode={puzzle2dSelectionMode}
        selectionTargets={puzzle2dSelectionTargets}
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
  const paneId = node.paneId as Puzzle2dPlayPaneId;
  return <Puzzle2dPlayPaneCanvas paneId={paneId} showBackgroundMenu={paneId === "2d-overview"} />;
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
  registerTabIcon("puzzle.2d-play.icon.library", Library);
  registerTabIcon("puzzle.2d-play.icon.inspector", ClipboardList);
  registerTabIcon("puzzle.2d-play.icon.settings", Settings);
}
// #endregion 🔖Panes

// #region 🔖SidePanels
// #region 🔖PaletteFixtureShelf
/** @emoji 📐 Palette seeds match {@link PUZZLE_2D_PLAY_DEFAULT_NODE_SIZE_PX} (circle radius = span/2). */

const PUZZLE_2D_PLAY_PALETTE_CIRCLE_DRAG_FIXTURE: Puzzle2dFixtureV1 =
  parsePuzzle2dFixtureV1({
    camera: { x: 0, y: 0, zoom: 1 },
    edges: [],
    meta: { puzzle2dFixtureDragKind: PUZZLE_2D_FIXTURE_DRAG_KIND_PALETTE_NODE },
    nodes: [{ handles: [{ angle: 0, id: "palette-seed-circle.h0" }], id: "palette-seed-circle", radius: PUZZLE_2D_PLAY_DEFAULT_NODE_SIZE_PX / 2, x: 0, y: 0 }],
    schema: "puzzle.2d.fixture/v1",
  }) ??
  (() => {
    throw new Error("Puzzle 2d play: palette circle drag fixture failed validation.");
  })();

const PUZZLE_2D_PLAY_PALETTE_RECTANGLE_DRAG_FIXTURE: Puzzle2dFixtureV1 =
  parsePuzzle2dFixtureV1({
    camera: { x: 0, y: 0, zoom: 1 },
    edges: [],
    meta: { puzzle2dFixtureDragKind: PUZZLE_2D_FIXTURE_DRAG_KIND_PALETTE_NODE },
    nodes: [
      {
        handles: [{ angle: 0, id: "palette-seed-rectangle.h0" }],
        height: PUZZLE_2D_PLAY_DEFAULT_NODE_SIZE_PX,
        id: "palette-seed-rectangle",
        shape: "rectangle",
        width: PUZZLE_2D_PLAY_DEFAULT_NODE_SIZE_PX,
        x: 0,
        y: 0,
      },
    ],
    schema: "puzzle.2d.fixture/v1",
  }) ??
  (() => {
    throw new Error("Puzzle 2d play: palette rectangle drag fixture failed validation.");
  })();

/** @emoji 👻 Draggable chip with drag image rendered under `document.body` so host panel overflow does not clip the preview. */
function Puzzle2dFixturePaletteDraggable(props: { fixture: Puzzle2dFixtureV1; label: string; preview: ReactNode }): ReactElement {
  const { fixture: dragFixture, label, preview } = props;
  const dragProps = useNativeDragAndDrop(
    reactHostPort.useMemo(
      () => ({
        onDragStart: (event: React.DragEvent<HTMLDivElement>) => {
          setPuzzle2dFixtureDragDataTransfer(event.dataTransfer, dragFixture);
          event.dataTransfer.effectAllowed = "copy";
          const { clientHeight, clientWidth } = event.currentTarget;
          event.dataTransfer.setDragImage(event.currentTarget, clientWidth / 2, clientHeight / 2);
        },
      }),
      [dragFixture],
    ),
  );
  return (
    <div className="border-element bg-background flex h-10 w-10 shrink-0 cursor-grab items-center justify-center rounded-lg border active:cursor-grabbing" title={label} {...dragProps}>
      {preview}
    </div>
  );
}
// #endregion 🔖PaletteFixtureShelf

/** @emoji 📥 Left rail: drag the active graph onto a puzzle 2d pane (in-app MIME payload, not filesystem JSON files). */
function Puzzle2dFixtureLibraryPanel(): ReactElement {
  const { fixture } = usePuzzle2dPlayShell();

  const shelfDragProps = useNativeDragAndDrop(
    reactHostPort.useMemo(
      () => ({
        onDragStart: (event: React.DragEvent<HTMLDivElement>) => {
          setPuzzle2dFixtureDragDataTransfer(event.dataTransfer, fixture);
          event.dataTransfer.effectAllowed = "copy";
        },
      }),
      [fixture],
    ),
  );

  return (
    <div className="flex h-full min-h-0 flex-col gap-3 p-3 text-sm">
      <div className="text-muted-foreground text-xs uppercase tracking-wide" data-testid="puzzle-2d-play-fixture-shelf">
        Fixture shelf
      </div>
      <div className="flex flex-col gap-2">
        <div className="text-muted-foreground text-[11px] uppercase tracking-wide">Shapes</div>
        <div className="flex flex-wrap gap-2">
          <Puzzle2dFixturePaletteDraggable fixture={PUZZLE_2D_PLAY_PALETTE_CIRCLE_DRAG_FIXTURE} label="Drag circle onto the puzzle 2d canvas" preview={<div className="border-primary size-10 shrink-0 rounded-full border-2 bg-accent/30" />} />
          <Puzzle2dFixturePaletteDraggable fixture={PUZZLE_2D_PLAY_PALETTE_RECTANGLE_DRAG_FIXTURE} label="Drag rectangle onto the puzzle 2d canvas" preview={<div className="border-primary size-10 shrink-0 rounded-sm border-2 bg-accent/30" />} />
        </div>
      </div>
      <div className="border-element bg-muted/30 flex min-h-30 cursor-grab flex-col justify-center gap-2 rounded-md border p-4 active:cursor-grabbing" {...shelfDragProps}>
        <p className="font-medium">Active graph</p>
        <p className="text-muted-foreground text-xs">Drag onto any puzzle 2d tab to load this graph (same payload for all panes).</p>
      </div>
      <div className="border-element space-y-1 rounded border p-2 text-xs">
        <div className="text-muted-foreground">Loaded</div>
        <div>schema: {fixture.schema}</div>
        <div>
          nodes: {fixture.nodes.length} · edges: {fixture.edges.length}
        </div>
      </div>
    </div>
  );
}

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

/** @emoji ⭕ Draggable ring control for handle polar angle `t` (radians, east-zero CCW in puzzle 2d space). */
function AngleTRing({ angleUniform, onChange, value }: { angleUniform: boolean; onChange: (next: number) => void; value: number }): ReactElement {
  const ref = reactHostPort.useRef<HTMLDivElement | null>(null);

  const setFromClient = reactHostPort.useCallback(
    (clientX: number, clientY: number) => {
      const el = ref.current;
      if (!el) {
        return;
      }
      const r = el.getBoundingClientRect();
      const cx = r.left + r.width / 2;
      const cy = r.top + r.height / 2;
      const dx = clientX - cx;
      const dy = clientY - cy;
      onChange(normalizeAngleRad(Math.atan2(dy, dx)));
    },
    [onChange],
  );

  const pointerDragProps = usePointerDrag<HTMLDivElement>({
    onStart: (event) => {
      event.preventDefault();
      setFromClient(event.clientX, event.clientY);
    },
    onMove: (event) => {
      setFromClient(event.clientX, event.clientY);
    },
  });

  const size = 88;
  const stroke = 3;
  const r = size / 2 - stroke * 2;
  const cx = size / 2;
  const cy = size / 2;
  const knobX = cx + r * Math.cos(value);
  const knobY = cy + r * Math.sin(value);

  return (
    <div className="flex flex-col items-center gap-1">
      <div
        className={`border-element bg-muted/20 touch-none select-none rounded-full border ${angleUniform ? "" : "pointer-events-none opacity-40"}`}
        ref={ref}
        style={{ height: size, width: size }}
        {...(angleUniform ? pointerDragProps : {})}
      >
        <svg aria-label="Angle t" height={size} viewBox={`0 0 ${size} ${size}`} width={size}>
          <circle cx={cx} cy={cy} fill="none" r={r} stroke="currentColor" strokeOpacity={0.35} strokeWidth={stroke} />
          <line stroke="currentColor" strokeOpacity={0.45} strokeWidth={1} x1={cx} x2={cx + r} y1={cy} y2={cy} />
          <line stroke="currentColor" strokeOpacity={0.25} strokeWidth={1} x1={cx} x2={cx} y1={cy} y2={cy - r} />
          <circle cx={knobX} cy={knobY} fill="var(--foreground)" r={5} stroke="var(--background)" strokeWidth={2} />
        </svg>
      </div>
      <div className="text-muted-foreground font-mono text-[10px]">{angleUniform ? `t = ${value.toFixed(4)} rad` : "Mixed t"}</div>
    </div>
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
  nodeIds,
  patchFixture,
}: {
  fixture: Puzzle2dFixtureV1;
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

  return (
    <div className="border-element/60 space-y-3 border-l pl-2">
      <Label id="puzzle-2d-play.inspector.node.name" label="Name">
        <Input className="h-7 font-mono text-xs" onChange={(e: ChangeEvent<HTMLInputElement>) => onText(e.target.value)} placeholder={textUniform ? undefined : "Mixed"} value={textValue} />
      </Label>
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
  handleIds,
  patchFixture,
}: {
  fixture: Puzzle2dFixtureV1;
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

  return (
    <div className="border-element/60 space-y-3 border-l pl-2">
      <div className="flex flex-wrap items-start gap-4">
        <AngleTRing
          angleUniform={angleUniform}
          onChange={(t) => {
            patchHandles((h) => ({ ...h, angle: t }));
          }}
          value={angleValue}
        />
        <div className="min-w-0 flex-1 space-y-3">
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
      </div>
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
  const handleOptions = reactHostPort.useMemo(() => listHandleIds(fixture), [fixture]);

  const patchEdges = reactHostPort.useCallback(
    (updater: (e: Puzzle2dFixtureEdgeV1) => Puzzle2dFixtureEdgeV1) => {
      patchFixture((prev) => ({
        ...prev,
        edges: prev.edges.map((e) => (idSet.has(e.id) ? updater(e) : e)),
      }));
    },
    [idSet, patchFixture],
  );

  return (
    <div className="border-element/60 space-y-3 border-l pl-2">
      <Label id="puzzle-2d-play.inspector.edge.source" label="Source">
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
                {puzzle2dFixtureHandleEndpointDisplayLabel(hid, fixture, kindCatalogs)}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </Label>
      <Label id="puzzle-2d-play.inspector.edge.target" label="Target">
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
                {puzzle2dFixtureHandleEndpointDisplayLabel(hid, fixture, kindCatalogs)}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </Label>
    </div>
  );
}

/** @emoji 🔎 Playground tree inspector sections for the active pane selection (every section has items). */
function buildPuzzle2dPlayInspectorSections(shell: Puzzle2dPlayShellValue, selection: Puzzle2dPlaySelectionValue): TreeDataSection[] {
  const { activePaneId, fixture, patchFixture } = shell;
  const { selectionIds } = selection;
  const kindCatalogs = puzzle2dFixtureMergedKindCatalogs(fixture);
  const ids = [...selectionIds].sort((a, b) => a.localeCompare(b));
  const nodeIds: string[] = [];
  const handleIds: string[] = [];
  const edgeIds: string[] = [];
  for (const id of ids) {
    if (findNode(fixture, id)) {
      nodeIds.push(id);
    } else if (findEdge(fixture, id)) {
      edgeIds.push(id);
    } else if (findHandleOwner(fixture, id)) {
      handleIds.push(id);
    }
  }
  if (ids.length === 0) {
    return [
      playgroundPanelSection(
        "puzzle-2d-play-inspector.empty",
        "Detail",
        <p className="text-muted-foreground leading-snug">
          pane: {activePaneId}. No selection. Click the graph or pick another tab.
        </p>,
      ),
    ];
  }
  const sections: TreeDataSection[] = [
    playgroundPanelSection(
      "puzzle-2d-play-inspector.header",
      "Detail",
      <p className="text-muted-foreground text-[11px] leading-snug">
        {activePaneId} · {ids.length} selected
      </p>,
    ),
  ];
  if (nodeIds.length > 0) {
    sections.push(
      playgroundPanelSection(
        "puzzle-2d-play-inspector-nodes",
        `Nodes (${nodeIds.length})`,
        <InspectorNodeBatch fixture={fixture} nodeIds={nodeIds} patchFixture={patchFixture} />,
      ),
    );
  }
  if (handleIds.length > 0) {
    sections.push(
      playgroundPanelSection(
        "puzzle-2d-play-inspector-handles",
        `Handles (${handleIds.length})`,
        <InspectorHandleBatch fixture={fixture} handleIds={handleIds} patchFixture={patchFixture} />,
      ),
    );
  }
  if (edgeIds.length > 0) {
    sections.push(
      playgroundPanelSection(
        "puzzle-2d-play-inspector-edges",
        `Edges (${edgeIds.length})`,
        <InspectorEdgeBatch edgeIds={edgeIds} fixture={fixture} kindCatalogs={kindCatalogs} patchFixture={patchFixture} />,
      ),
    );
  }
  if (nodeIds.length === 0 && handleIds.length === 0 && edgeIds.length === 0) {
    sections.push(
      playgroundPanelSection(
        "puzzle-2d-play-inspector-unknown",
        "Selection",
        <p className="text-[11px] text-warning-foreground leading-snug">{ids.map((id) => puzzle2dFixtureObjectDisplayLabel(id, fixture, kindCatalogs)).join(", ")}</p>,
      ),
    );
  }
  return sections;
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
const initialFixture = clonePuzzle2dFixtureV1(PUZZLE_2D_PLAY_DEFAULT_FIXTURE);

function Puzzle2dPlayInner({ puzzle2dRuntime }: { readonly puzzle2dRuntime: Platform }): ReactElement {
  const [fixture, setFixtureState] = reactHostPort.useState<Puzzle2dFixtureV1>(() => clonePuzzle2dFixtureV1(initialFixture));
  const fixtureRef = reactHostPort.useRef<Puzzle2dFixtureV1>(fixture);
  fixtureRef.current = fixture;
  const [puzzle2dPlayPaneCamerasBaseline, setPuzzle2dPlayPaneCamerasBaseline] = reactHostPort.useState<Record<Puzzle2dPlayPaneId, CameraState>>(() => triptychCamerasFromFixture(initialFixture));
  const puzzle2dPlayPaneCamerasBaselineRef = reactHostPort.useRef(puzzle2dPlayPaneCamerasBaseline);
  puzzle2dPlayPaneCamerasBaselineRef.current = puzzle2dPlayPaneCamerasBaseline;
  const [activePaneId, setActivePaneId] = reactHostPort.useState<Puzzle2dPlayPaneId>("2d-overview");
  const activePaneIdRef = reactHostPort.useRef(activePaneId);
  activePaneIdRef.current = activePaneId;
  const [selectionIds, setSelectionIdsState] = reactHostPort.useState<Set<string>>(() => selectionSeedForFixture(initialFixture));
  const [preselection, setPreselection] = reactHostPort.useState<Puzzle2dPreselectSnapshot>(PUZZLE_2D_PRESELECT_EMPTY);
  const [hoveredId, setHoveredId] = reactHostPort.useState<string | null>(null);
  const [hoverSourcePane, setHoverSourcePane] = reactHostPort.useState<Puzzle2dPlayPaneId | null>(null);
  const hoverSourcePaneRef = reactHostPort.useRef<Puzzle2dPlayPaneId | null>(hoverSourcePane);
  hoverSourcePaneRef.current = hoverSourcePane;
  const [puzzle2dSelectionMethod, setPuzzle2dSelectionMethod] = reactHostPort.useState<Puzzle2dSelectionMethod>("rectangle");
  const [puzzle2dSelectionMode, setPuzzle2dSelectionMode] = reactHostPort.useState<Puzzle2dSelectionMode>("default");
  const [puzzle2dSelectionTargets, setPuzzle2dSelectionTargets] = reactHostPort.useState<Puzzle2dSelectionTargets>(() => ({ ...PUZZLE_2D_SELECTION_TARGETS_DEFAULT }));
  const [puzzle2dGridSnapEnabled, setPuzzle2dGridSnapEnabled] = reactHostPort.useState(false);
  const [puzzle2dActiveTool, setPuzzle2dActiveTool] = reactHostPort.useState<Puzzle2dActiveTool>("select");
  const [puzzle2dBrushFlushDistance, setPuzzle2dBrushFlushDistance] = reactHostPort.useState(DEFAULT_PUZZLE_2D_BRUSH_FLUSH_DISTANCE_PX);
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
              label: kindId.split(".").pop() ?? kindId,
            }))
          : [];
      puzzle2dShellController?.setBrushEngagementPossibles(rows);
    },
    [puzzle2dActiveTool, puzzle2dShellController],
  );

  const setPuzzle2dEffectiveLodForPane = reactHostPort.useCallback(
    (pane: Puzzle2dPlayPaneId, lod: Puzzle2dDrawLodKind) => {
      puzzle2dRuntime.commandBus.dispatch(PUZZLE_2D_PLAY_CONTROLLER_ID, "setEffectiveLodForPane", { pane, lod });
    },
    [puzzle2dRuntime.commandBus],
  );
  const onPuzzle2dPlayActiveWindowChange = reactHostPort.useCallback((windowKindId: string) => {
    if (windowKindId === "2d-overview" || windowKindId === "2d-detail" || windowKindId === "2d-selection") {
      setActivePaneId(windowKindId);
    }
  }, []);
  const [puzzle2dRedrawPlaying, setPuzzle2dRedrawPlaying] = reactHostPort.useState(false);
  const [forceLayoutFullIterations, setForceLayoutFullIterations] = reactHostPort.useState(200);
  const [forceLayoutIdealEdgeLength, setForceLayoutIdealEdgeLength] = reactHostPort.useState(64);
  const [forceLayoutGravity, setForceLayoutGravity] = reactHostPort.useState(0.012);
  const [forceLayoutRepulsionStrength, setForceLayoutRepulsionStrength] = reactHostPort.useState(80);
  const [puzzle2dRedrawPlayMaxItersPerFrame, setPuzzle2dRedrawPlayMaxItersPerFrame] = reactHostPort.useState(96);
  const [puzzle2dRedrawProgressiveEnabled, setPuzzle2dRedrawProgressiveEnabled] = reactHostPort.useState(true);
  const [puzzle2dRedrawProgressiveAutoStopMs, setPuzzle2dRedrawProgressiveAutoStopMs] = reactHostPort.useState(3000);
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

  const applyStructuralDelete = reactHostPort.useCallback((kind: "edge" | "node", id: string) => {
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
      const n = prev.nodes.find((x) => x.id === id);
      if (!n) {
        return prev;
      }
      const hset = new Set(n.handles.map((h) => h.id));
      const next = {
        ...prev,
        edges: prev.edges.filter((e) => !hset.has(e.source) && !hset.has(e.target)),
        nodes: prev.nodes.filter((x) => x.id !== id),
      };
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
    setFixtureState((prev) => puzzle2dPlayRehydrateFixtureEdgesIfMissing(prev, initialFixture));
  }, [fixture.edges.length]);

  const structuralDeleteQueueRef = reactHostPort.useRef<Puzzle2dPlayStructuralDeleteItem[]>([]);
  const structuralDeleteFlushScheduledRef = reactHostPort.useRef(false);
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
        structuralDeleteFlushScheduledRef.current = false;
        const batch = structuralDeleteQueueRef.current;
        structuralDeleteQueueRef.current = [];
        const pending = filterPuzzle2dPlayStructuralDeleteBatch(batch, fixtureRef.current);
        for (const item of pending) {
          if (item.kind === "edge") {
            applyStructuralDelete("edge", item.id);
            continue;
          }
          applyStructuralDelete("node", item.id);
        }
      });
    },
    [applyStructuralDelete],
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

  const setHoverForPane = reactHostPort.useCallback((pane: Puzzle2dPlayPaneId, id: string | null) => {
    hoverSourcePaneRef.current = pane;
    setHoverSourcePane(pane);
    setHoveredId(id);
  }, []);

  const clearHoverForPane = reactHostPort.useCallback((pane: Puzzle2dPlayPaneId) => {
    if (hoverSourcePaneRef.current !== pane) {
      return;
    }
    hoverSourcePaneRef.current = null;
    setHoverSourcePane(null);
    setHoveredId(null);
  }, []);

  const setHierarchyHover = reactHostPort.useCallback((id: string | null) => {
    hoverSourcePaneRef.current = null;
    setHoverSourcePane(null);
    setHoveredId(id);
  }, []);

  const handleCanvasFixtureDrop = reactHostPort.useCallback(
    (_pane: Puzzle2dPlayPaneId, detail: Puzzle2dFixtureDropDetail) => {
      skipNextCameraBasisResyncRef.current = true;
      guardFixtureAuthoringFromStructuralDeletes(200);
      const merged = mergePaletteNodeFromDrop(detail);
      if (merged) {
        paletteDropNodeGuardRef.current.add(merged.id);
        if (typeof globalThis.setTimeout === "function") {
          globalThis.setTimeout(() => {
            paletteDropNodeGuardRef.current.delete(merged.id);
          }, 600);
        }
        patchFixture((prev) => ({ ...prev, nodes: [...prev.nodes, merged] }));
        setSelectionIds([merged.id]);
        return;
      }
      setFixture(detail.fixture);
    },
    [guardFixtureAuthoringFromStructuralDeletes, patchFixture, setFixture, setSelectionIds],
  );

  const commitBrushPlacement = reactHostPort.useCallback(
    (payload: Puzzle2dBrushPlacePayload) => {
      guardFixtureAuthoringFromStructuralDeletes(200);
      patchFixture((prev) => {
        const result = applyBrushPlacementToFixture(prev, payload, puzzle2dFixtureMergedKindCatalogs(prev));
        if (result.kind !== "placed") {
          return prev;
        }
        puzzle2dGuardBrushPlacementStructuralDeletes(result.nodeId, result.edgeId);
        puzzle2dSyncFixtureDescriptorToAllAuthoringPeers(result.fixture);
        return result.fixture;
      });
    },
    [guardFixtureAuthoringFromStructuralDeletes, patchFixture],
  );

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
    setPuzzle2dPlayPaneCamerasBaseline((prev) => {
      const pane = activePaneIdRef.current;
      const p = prev[pane];
      if (Math.abs(p.x - c.x) < 1e-6 && Math.abs(p.y - c.y) < 1e-6 && Math.abs(p.zoom - c.zoom) < 1e-9) {
        return prev;
      }
      return { ...prev, [pane]: { ...c } };
    });
  }, []);

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
  const redrawLoopSnapshotRef = reactHostPort.useRef<Puzzle2dPlayRedrawLoopSnapshot>({
    activePaneId: "2d-overview",
    puzzle2dRedrawHandlesAfterNodes: false,
    puzzle2dRedrawProgressiveAutoStopMs: 3000,
    puzzle2dRedrawProgressiveEnabled: true,
    puzzle2dRedrawPlayMaxItersPerFrame: 96,
    camerasByPane: triptychCamerasFromFixture(initialFixture),
    forceLayoutGravity: 0.012,
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
    patchFixture((prev) => {
      const laidOut = layoutPuzzle2dFixtureRedrawNodes(
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
        ),
      );
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
          return layoutPuzzle2dFixtureRedrawNodes(
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
            ),
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
            ),
          );
        }
        return cur;
      });
      raf = requestAnimationFrame(step);
    };
    raf = requestAnimationFrame(step);
    return () => {
      redrawPlayingRef.current = false;
      cancelAnimationFrame(raf);
    };
  }, [puzzle2dRedrawPlaying, patchFixture, setPuzzle2dRedrawPlaying]);

  const shellValue = reactHostPort.useMemo<Puzzle2dPlayShellValue>(
    () => ({
      activePaneId,
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
      setActivePaneId,
      setPuzzle2dRedrawHandlesAfterNodes,
      setPuzzle2dRedrawMode,
      setPuzzle2dRedrawPlayMaxItersPerFrame,
      setPuzzle2dRedrawPlaying,
      setPuzzle2dRedrawProgressiveAutoStopMs,
      setPuzzle2dRedrawProgressiveEnabled,
      setPuzzle2dGridSnapEnabled,
      puzzle2dLodModeByPane,
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
      setPuzzle2dLodModeForPane,
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
      setSelectionIds,
      sceneAuthoringEpoch,
      hoveredId,
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
      syncBaselineFromViewportCamera,
    }),
    [camerasByPane, syncBaselineFromViewportCamera],
  );

  // #region 🔖ToolbarHostBridge
  const puzzle2dPlayToolbarHostRef = reactHostPort.useRef({
    activePaneId: "2d-overview" as Puzzle2dPlayPaneId,
    applyPuzzle2dRedrawHandlesOnce: () => {},
    camerasByPane: triptychCamerasFromFixture(initialFixture),
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
          case "setActiveTool":
            setPuzzle2dActiveTool((args as { tool: Puzzle2dActiveTool }).tool);
            break;
          case "setBrushFlushDistance":
            setPuzzle2dBrushFlushDistance((args as { distance: number }).distance);
            break;
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
    setPuzzle2dActiveTool,
    setPuzzle2dBrushFlushDistance,
  ]);
  // #endregion 🔖ToolbarHostBridge

  const shellValueRef = reactHostPort.useRef(shellValue);
  shellValueRef.current = shellValue;
  const selectionValueRef = reactHostPort.useRef(selectionValue);
  selectionValueRef.current = selectionValue;
  const puzzle2dPlayHierarchyPanel = reactHostPort.useMemo(() => new Puzzle2dPlayHierarchyPanelDefinition(), []);
  const puzzle2dPlayLibraryPanel = reactHostPort.useMemo(() => new Puzzle2dPlayLibraryPanelDefinition(), []);
  const puzzle2dPlaySettingsPanel = reactHostPort.useMemo(() => new Puzzle2dPlaySettingsPanelDefinition(), []);
  const puzzle2dPlayInspectorPanel = reactHostPort.useMemo(
    () =>
      new Puzzle2dPlayInspectorPanelDefinition(() =>
        buildPuzzle2dPlayInspectorSections(shellValueRef.current, selectionValueRef.current),
      ),
    [],
  );
  const augmentPanelTabs = reactHostPort.useMemo(
    () => ({
      workbench: [puzzle2dPlayHierarchyPanel, puzzle2dPlayLibraryPanel],
      details: [puzzle2dPlayInspectorPanel, puzzle2dPlaySettingsPanel],
    }),
    [puzzle2dPlayHierarchyPanel, puzzle2dPlayInspectorPanel, puzzle2dPlaySettingsPanel, puzzle2dPlayLibraryPanel],
  );

  return (
    <Puzzle2dPlayShellContext.Provider value={shellValue}>
      <Puzzle2dPlaySelectionContext.Provider value={selectionValue}>
        <Puzzle2dPlayCanvasSelectionContext.Provider value={canvasSelectionValue}>
          <Puzzle2dPlayCamerasContext.Provider value={camerasValue}>
            <Puzzle2dPlayLodRuntimeContext.Provider value={setPuzzle2dEffectiveLodForPane}>
              <PlaygroundView runtime={puzzle2dRuntime} defaultAppId={PUZZLE_2D_PLAY_APP_ID} augmentPanelTabs={augmentPanelTabs} initialPanelVisibility={{ leftSidePanel: true, rightSidePanel: true }} onActiveWindowChange={onPuzzle2dPlayActiveWindowChange} />
            </Puzzle2dPlayLodRuntimeContext.Provider>
          </Puzzle2dPlayCamerasContext.Provider>
        </Puzzle2dPlayCanvasSelectionContext.Provider>
      </Puzzle2dPlaySelectionContext.Provider>
    </Puzzle2dPlayShellContext.Provider>
  );
}

function Puzzle2dPlayChrome({ runtime }: { readonly runtime: Platform }): ReactElement {
  return <Puzzle2dPlayInner puzzle2dRuntime={runtime} />;
}

/** @emoji 🚀 Mounts puzzle 2d play chrome for a {@link Playground}. */
export function mountPuzzle2dPlayChrome(playground: Playground, rootId = "root"): void {
  mountPlaygroundApp(<Puzzle2dPlayChrome runtime={playground.runtime} />, rootId);
}

const puzzle2dPlayChromeBoot: PlaygroundChromeBoot = {
  registerHosts: registerPuzzle2dPlaySurfaceHosts,
  mount: mountPuzzle2dPlayChrome,
};

/** @emoji 🛝 Puzzle 2D play entry: register hosts, bodies, mount chrome (from `puzzle/2d/play/index.ts`). */
export function boot2dPlay(playground: Playground, rootId = "root"): void {
  bootPlayground(playground, puzzle2dPlayChromeBoot, rootId);
}

// #endregion 🔖Entrypoint

// #endregion 🛝PlayHost
//#endregion 🔖Puzzle2dPlayHost

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

    it("threads engagement through windowKindsToGolden", () => {
      const wk = new WindowKindRuntime("w", "W", "body", undefined, [], {
        input: { id: "engagement-input", onChange: { controllerId: "ctrl", command: "engagementInput" } },
        status: [{ id: "s", text: "ready" }],
      });
      const golden = windowKindsToGolden([wk], new CommandBus());
      expect(golden[0]?.engagement?.status?.[0]?.content).toBe("ready");
    });
  });

  describe("enforcePlaygroundTreePanel", () => {
    it("rejects sections without items or content", () => {
      expect(() =>
        enforcePlaygroundTreePanel({
          sections: [{ id: "a" }],
        }),
      ).toThrow(/items or content/);
    });

    it("accepts content-only sections", () => {
      expect(() =>
        enforcePlaygroundTreePanel({
          sections: [{ id: "a", content: "panel body" }],
        }),
      ).not.toThrow();
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
    it("wraps panel bodies in PlaygroundPanelBody content", () => {
      const section = playgroundPanelSection("panel.test", "Test", <span data-testid="body">x</span>);
      expect(section.content).toBeTruthy();
      expect(section.items).toBeUndefined();
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
  });

}
