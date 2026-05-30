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
  LevelProvider,
  staticTreePanelDefinition,
  useElementsSurfaceChrome,
  useMediaQuery,
  type EngagementSpec,
  type FooterItem,
  type NavbarItem,
  type SidePanelTabConfig,
  type SidePanelTabDefinition,
  type TreeDataItem,
  type TreeDataSection,
  type TreePanelConfig,
  type TreePanelDefinition,
  type TreePanelSource,
  reactHostPort,
  Button,
  IconSelector,
  useNativeDragAndDrop,
  type ContextMenuItem,
} from "@ui/react";
import { clsx, type ClassValue } from "clsx";
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
  type ResolvedAppState,
  type SidePanelBodyViewContext,
  type SideTabSpec,
  type UiBoardHostSurfaceNode,
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
  type UiPuzzle3dHostSurfaceNode,
  type UiTableHostSurfaceNode,
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
  abstract resolveTab(): SidePanelTabConfig;
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

/** @emoji 🌲 Tree panel that rebuilds sections on every {@link TreePanelDefinition.resolveTree} call. */
export class CallbackTreePanelDefinition implements TreePanelDefinition {
  constructor(private readonly buildSections: () => TreeDataSection[]) {}

  resolveTree(): TreePanelConfig {
    const config: TreePanelConfig = { sections: this.buildSections() };
    enforcePlaygroundTreePanel(config);
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
    resolveTreePanelSource(resolved.tree);
    return resolved;
  }
  const config = tab as SidePanelTabConfig;
  resolveTreePanelSource(config.tree);
  return config;
}

//#region 🔖UiRenderer
type Puzzle3dSurfaceHost = React.ComponentType<{ readonly node: UiPuzzle3dHostSurfaceNode }>;
type BoardSurfaceHost = React.ComponentType<{ readonly node: UiBoardHostSurfaceNode }>;
type TableSurfaceHost = React.ComponentType<{ readonly node: UiTableHostSurfaceNode }>;
type PlaygroundSurfaceBindingHost = React.ComponentType<{ readonly node: UiComponentHostSurfaceNode }>;

const puzzle3dSurfaceHosts = new Map<string, Puzzle3dSurfaceHost>();
const boardSurfaceHosts = new Map<string, BoardSurfaceHost>();
const tableSurfaceHosts = new Map<string, TableSurfaceHost>();

const PLAYGROUND_CANVAS_HOST_TYPES = new Set(["board", "puzzle2d", "puzzle3d", "puzzle5d", "cad"]);

function isPlaygroundCanvasHostChild(child: UiNode): boolean {
  return PLAYGROUND_CANVAS_HOST_TYPES.has(child.type);
}

/** @emoji 🧭 Binds a `surfaceId` from {@link UiPuzzle3dHostSurfaceNode} to a host React canvas implementation. */
export function registerUiPuzzle3dSurfaceHost(surfaceId: string, Component: Puzzle3dSurfaceHost): void {
  puzzle3dSurfaceHosts.set(surfaceId, Component);
  registerSurfaceBinding(surfaceId, Component as PlaygroundSurfaceBindingHost);
}

export { registerSurfaceBinding, unregisterSurfaceBinding };

/** @emoji 📋 Binds `surfaceId` from {@link UiBoardHostSurfaceNode} to a host board canvas. */
export function registerUiBoardSurfaceHost(surfaceId: string, Component: BoardSurfaceHost): void {
  boardSurfaceHosts.set(surfaceId, Component);
  registerSurfaceBinding(surfaceId, Component as PlaygroundSurfaceBindingHost);
}

/** @emoji 📊 Binds `surfaceId` from {@link UiTableHostSurfaceNode} to a host table body. */
export function registerUiTableSurfaceHost(surfaceId: string, Component: TableSurfaceHost): void {
  tableSurfaceHosts.set(surfaceId, Component);
  registerSurfaceBinding(surfaceId, Component as PlaygroundSurfaceBindingHost);
}

function renderPlaygroundHostSurface(node: UiNode, layout: "canvas" | "panel"): React.ReactElement {
  if (node.type === "board") {
    const Host = boardSurfaceHosts.get(node.surfaceId);
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
    items: item.items?.length ? uiTreeItemsToTreeData(item.items, commandBus) : undefined,
    onClick: item.command
      ? () => {
          dispatchUiCommand(commandBus, item.command!, {});
        }
      : undefined,
  }));
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
        {node.items.map((item) => (
          <SelectItem key={item.value} value={item.value}>
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
    case "board":
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
      return (
        <div className="flex min-h-0 min-w-0 flex-1 flex-col overflow-hidden">
          <Tree className="min-h-0 flex-1 overflow-auto" sections={uiTreeSectionsToTreeData((node as UiTreeNode).sections, commandBus)} selectionMode="single" showLines />
        </div>
      );
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
    const Body = declarativeFactory ? getDeclarativeSidePanelBodyComponent(tab.id, tab.bodyKey) : () => <div className="p-2 text-xs">Missing panel {tab.bodyKey}</div>;
    const panelBody = (
      <PlaygroundPanelBody>
        <Body />
      </PlaygroundPanelBody>
    );
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

const declarativeSidePanelBodyComponents = new Map<string, React.FC>();

function getDeclarativeSidePanelBodyComponent(tabId: string, bodyKey: string): React.FC {
  const cacheKey = `${bodyKey}\0${tabId}`;
  let component = declarativeSidePanelBodyComponents.get(cacheKey);
  if (!component) {
    component = function ShellDeclarativeSidePanelBody() {
      const { runtime, activeModeId } = useApp();
      const generation = reactHostPort.useSyncExternalStore(
        (listener) => runtime.subscribe(listener),
        () => runtime.generation,
        () => 0,
      );
      const ctx: SidePanelBodyViewContext = {
        runtime,
        windowKindId: tabId,
        bodyKey,
        activeModeId: activeModeId ?? null,
        generation,
      };
      const factory = getSidePanelBodyFactory(bodyKey);
      const node = factory?.(ctx) ?? { type: "text", value: `Missing declarative panel "${bodyKey}"` };
      return <UiRenderer node={node} commandBus={runtime.commandBus} />;
    };
    declarativeSidePanelBodyComponents.set(cacheKey, component);
  }
  return component;
}
//#endregion 🔖DeclarativeHosts

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
  base?.forEach((tab) => merged.set(tab.id, resolveSidePanelTabSource(tab)));
  extension.forEach((tab) => merged.set(resolveSidePanelTabSource(tab).id, resolveSidePanelTabSource(tab)));
  return [...merged.values()];
}

/** @emoji 🛝 Playground application shell: tree-only side panels, no JSON fallback details tab. */
export const PlaygroundView: React.FC<PlaygroundViewProps> = ({ runtime, defaultAppId, mobile, mobileQuery = "(max-width: 767px)", initialPanelVisibility, slotToolbar, extraFooterItems, augmentPanelTabs, onActiveWindowChange }) => {
  reactHostPort.useSyncExternalStore(
    (onStoreChange) => runtime.subscribe(onStoreChange),
    () => runtime.generation,
    () => 0,
  );

  reactHostPort.useEffect(() => {
    if (defaultAppId) runtime.setActiveAppId(defaultAppId);
  }, [defaultAppId, runtime]);

  const [leftPanelSize, setLeftPanelSize] = reactHostPort.useState(280);
  const [rightPanelSize, setRightPanelSize] = reactHostPort.useState(300);
  const [panelVisibility, setPanelVisibilityState] = reactHostPort.useState<PlaygroundPanelVisibility>(() =>
    resolveInitialPanelVisibility(initialPanelVisibility, runtime),
  );
  const setPanelVisibility = reactHostPort.useCallback(
    (next: PlaygroundPanelVisibility | ((prev: PlaygroundPanelVisibility) => PlaygroundPanelVisibility)) => {
      setPanelVisibilityState((prev) => {
        const resolved = typeof next === "function" ? next(prev) : next;
        runtime.setPanelVisibility(resolved);
        return resolved;
      });
    },
    [runtime],
  );
  const detectedMobile = useMediaQuery(mobileQuery);
  const resolvedMobile = mobile ?? detectedMobile ?? runtime.mobile;

  const activeAppBase = runtime.getActiveApp();
  if (!activeAppBase) return null;

  const activeModeId = activeAppBase.getActiveModeId();
  const activeApp = activeAppBase.resolve(activeModeId);
  const bus = runtime.commandBus;

  const workbenchTabs = mergePanelTabs(
    sideTabsToPlaygroundPanelTabs(
      activeApp.panelTabs.filter((tab) => tab.panel === "workbench"),
      bus,
    ),
    augmentPanelTabs?.workbench,
  );
  const detailsTabs = mergePanelTabs(
    sideTabsToPlaygroundPanelTabs(
      activeApp.panelTabs.filter((tab) => tab.panel === "details"),
      bus,
    ),
    augmentPanelTabs?.details,
  );

  const mergedTools = declareToolsToViewTools(activeApp.tools, bus);
  const hasToolbarTools = listPopulatedToolbarViewCategories(mergedTools).length > 0;

  const [activeWindowKindId, setActiveWindowKindId] = reactHostPort.useState<string | null>(() => findDefaultActiveWindowKindId(activeApp.defaultLayout, activeApp.windowKinds));

  reactHostPort.useEffect(() => {
    setActiveWindowKindId((previous) => {
      if (previous && activeApp.windowKinds.some((wk) => wk.id === previous)) return previous;
      return findDefaultActiveWindowKindId(activeApp.defaultLayout, activeApp.windowKinds);
    });
  }, [activeApp.defaultLayout, activeApp.windowKinds]);

  const goldenWindowKinds = reactHostPort.useMemo(() => windowKindsToGolden(activeApp.windowKinds, bus), [activeApp.windowKinds, bus]);

  const footerItems: FooterItem[] = [
    ...mergePlatformFooterChromeRows(runtime, activeApp),
    ...(extraFooterItems ?? []),
  ].sort((a, b) => (a.order ?? 0) - (b.order ?? 0));

  const workbenchIcon = workbenchTabs[0]?.icon ? reactHostPort.createElement(workbenchTabs[0].icon, { size: 16 }) : <Folder size={16} />;
  const detailsIcon = detailsTabs[0]?.icon ? reactHostPort.createElement(detailsTabs[0].icon, { size: 16 }) : <Info size={16} />;

  const navbarItems: NavbarItem[] = [
    {
      key: "title",
      className: "flex-1 min-w-0",
      content: <span className="truncate px-single text-sm font-medium">{activeApp.label}</span>,
    },
    {
      key: "panelToggles",
      content: (
        <div className="flex items-stretch overflow-hidden border border-element h-medium">
          <Toggle
            kind="icon"
            id="playground.panel.workbench"
            pressed={panelVisibility.leftSidePanel}
            onPressedChange={(pressed) => setPanelVisibility((p) => ({ ...p, leftSidePanel: pressed }))}
            icon={workbenchIcon}
            className="rounded-none border-0"
          />
          <Toggle
            kind="icon"
            id="playground.panel.details"
            pressed={panelVisibility.rightSidePanel}
            onPressedChange={(pressed) => setPanelVisibility((p) => ({ ...p, rightSidePanel: pressed }))}
            icon={detailsIcon}
            className="rounded-none border-0 border-l"
          />
        </div>
      ),
    },
  ];

  const toolbarElement = slotToolbar ?? (hasToolbarTools && mergedTools ? <UIToolbar tools={mergedTools} /> : undefined);

  return (
    <PlaygroundContext.Provider
      value={{
        runtime,
        activeAppId: runtime.activeAppId,
        activeApp,
        activeModeId,
      }}
    >
      <ProductShell
        platform={runtime}
        defaultAppId={defaultAppId}
        className="min-h-0 flex-1"
        mobile={resolvedMobile}
        mobileQuery={mobileQuery}
        navbarItems={navbarItems}
        footerItems={footerItems}
        slotToolbar={toolbarElement}
        leftSidePanelTabs={workbenchTabs}
        rightSidePanelTabs={detailsTabs}
        panelVisibility={panelVisibility}
        leftPanelSize={leftPanelSize}
        onLeftPanelSizeChange={setLeftPanelSize}
        rightPanelSize={rightPanelSize}
        onRightPanelSizeChange={setRightPanelSize}
        goldenWindowKinds={goldenWindowKinds}
        defaultLayout={activeApp.defaultLayout}
        activeWindowKindId={activeWindowKindId}
        onActiveWindowKindChange={(windowKindId) => {
          setActiveWindowKindId(windowKindId);
          onActiveWindowChange?.(windowKindId);
        }}
        multiApp={false}
        activeModeId={activeModeId}
        onActiveModeChange={(modeId) => {
          activeAppBase.setActiveModeId(modeId);
          runtime.notify();
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
import { PlayCanvas, ObjectStateProvider, parseFixtureV1, applyConnectToFixture, blockedVortexFullIdsFromAttractions, type FixtureV1, type RelocatePayload } from "@puzzle/3d/react";
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

function Puzzle3dPlayViewportHost({ node }: { readonly node: UiPuzzle3dHostSurfaceNode }): React.ReactElement {
  const { runtime } = useApp();
  const bus = runtime.commandBus;
  const ctrl = usePuzzle3dPlayController();
  if (node.controllerId !== PUZZLE_3D_PLAY_CONTROLLER_ID) {
    return <div className="p-2 text-xs text-muted-foreground">Invalid puzzle 3D viewport binding</div>;
  }
  const snap = usePuzzle3dPlaySnapshot();
  if (!snap.fixture) {
    return <div className="p-4 text-destructive">Invalid puzzle 3D fixture</div>;
  }
  const kindCompatibility = parseKindCompatibility(snap.fixture.meta);
  const kindCatalogs = parseKindCatalogs(snap.fixture.meta);
  const blockedVortexFullIds = blockedVortexFullIdsFromAttractions(snap.fixture.attractions);
  const selectedVortexFullIds = reactHostPort.useMemo(() => new Set(snap.selection.vortexIds), [snap.selection.vortexIds]);
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
  return (
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
          selectedVortexFullIds={selectedVortexFullIds}
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
        />
      </ObjectStateProvider>
    </div>
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
  mountPlaygroundApp(<PlaygroundView runtime={playground.runtime} defaultAppId={PUZZLE_3D_PLAY_APP_ID} initialPanelVisibility={playground.initialPanelVisibility} />, rootId);
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

  resolveTab(): SidePanelTabConfig {
    return {
      id: PUZZLE_5D_PLAY_HIERARCHY_TAB_ID,
      icon: ListTree,
      order: 0,
      tree: new StaticTreePanelDefinition({ sections: this.buildTree().sections as TreeDataSection[] }),
    };
  }
}

class Puzzle5dPlayStatusPanelDefinition extends PureSidePanelTabDefinition {
  resolveTab(): SidePanelTabConfig {
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
function Puzzle5d2dSurfaceHost({ node }: { readonly node: UiBoardHostSurfaceNode }): React.ReactElement {
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
  registerUiBoardSurfaceHost(PUZZLE_5D_PLAY_2D_SURFACE_ID, Puzzle5d2dSurfaceHost);
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
  buildPuzzle2dPlayHierarchySections,
  buildPuzzle2dPlayOverviewDeclarativeBody,
  buildPuzzle2dPlayDetailDeclarativeBody,
  buildPuzzle2dPlaySelectionDeclarativeBody,
  buildPuzzle2dPlayRuntime,
  type Puzzle2dPlayHostBridge,
  type Puzzle2dPlayPaneId,
} from "@puzzle/2d/play";
import {
  mergeKindCatalogBundleByRowId,
  DEFAULT_KIND_CATALOG_BUNDLE,
  BUILTIN_PORT_HANDLE_KIND,
  BOARD_CAMERA_ZOOM_MIN,
  BOARD_CAMERA_ZOOM_MAX,
  BOARD_PRESELECT_EMPTY,
  BOARD_SELECTION_TARGETS_DEFAULT,
  fixtureMetaKindCatalogBundle,
  boardFixtureNodeCaption,
  classifyElementsBoardIconSelectorMode,
  parseBoardFixtureV1,
  BoardCanvas,
  boardFixtureSceneMarkers,
  type BoardStructureDeletePayload,
  encodeBoardFixtureForDragV1,
  BOARD_FIXTURE_DRAG_V1_MIME,
  BOARD_FIXTURE_DRAG_KIND_PALETTE_NODE,
  BOARD_LOD_MODE_AUTOMATIC,
  DEFAULT_BOARD_LOD_ZOOM_THRESHOLDS,
  layoutBoardFixtureRedrawNodes,
  normalizeBoardSelectionProp,
  type BoardFixtureV1,
  type BoardFixtureNodeV1,
  type BoardFixtureRectangleNodeV1,
  type BoardFixtureCircleNodeV1,
  type BoardFixtureHandleV1,
  type BoardFixtureEdgeV1,
  type BoardFixtureDropDetail,
  type BoardDrawLodKind,
  type BoardLodModeKind,
  type BoardSelectionMethod,
  type BoardSelectionMode,
  type BoardSelectionTargets,
  type BoardSelectionSnapshot,
  type BoardPreselectSnapshot,
  type BoardRedrawModeKind,
  type BoardHierarchicalTreeDirectionKind,
  type BoardRedrawLayoutOptions,
  type CameraState,
} from "@puzzle/2d/react";
import type { Playground } from "@framework/playground/core";
// #endregion 🔌Adapters

const PUZZLE_2D_PLAY_DEFAULT_KIND_CATALOGS = mergeKindCatalogBundleByRowId({ ...DEFAULT_KIND_CATALOG_BUNDLE }, fixtureMetaKindCatalogBundle(PUZZLE_2D_PLAY_DEFAULT_FIXTURE) ?? {});

// #region 🔖Kinds
export type { Puzzle2dPlayPaneId } from "@puzzle/2d/play";

const boardPlayOverviewWindowContextMenu: ContextMenuItem[] = [{ id: "win-demo", label: "Overview window menu demo" }];
const boardPlayDemoNodeContextMenu: ContextMenuItem[] = [
  { id: "demo-node", label: "Demo capsule action" },
  { children: [{ id: "demo-sub-1", label: "Nested item" }], id: "demo-sub", label: "Demo nested" },
];
const boardPlayDemoEdgeContextMenu: ContextMenuItem[] = [{ id: "demo-edge", label: "Demo edge action" }];
const boardPlayCanvasBackgroundMenu: ContextMenuItem[] = [{ id: "demo-bg", label: "Board background menu" }];

// #endregion 🔖Kinds

// #region 🔖Geometry
const REF_VIEWPORT_SHORT_PX = 640;

function clampZoom(value: number): number {
  return Math.min(BOARD_CAMERA_ZOOM_MAX, Math.max(BOARD_CAMERA_ZOOM_MIN, value));
}

/** @emoji 📐 Axis-aligned bounds of all fixture nodes (world units). */
function fixtureWorldBounds(fixture: BoardFixtureV1): { cx: number; cy: number; halfSpan: number } {
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
function triptychCamerasFromFixture(fixture: BoardFixtureV1): Record<Puzzle2dPlayPaneId, CameraState> {
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
function selectionSeedForFixture(fixture: BoardFixtureV1): Set<string> {
  const nodeA = fixture.nodes[0];
  return new Set(nodeA?.id ? [nodeA.id] : []);
}
// #endregion 🔖Geometry

// #region 🔖ShellContext
interface Puzzle2dPlayShellValue {
  fixture: BoardFixtureV1;
  setFixture: (next: BoardFixtureV1) => void;
  /** @emoji 🎯 Palette drags merge one node at the pointer; full fixtures replace the graph. */
  handleCanvasFixtureDrop: (pane: Puzzle2dPlayPaneId, detail: BoardFixtureDropDetail) => void;
  patchFixture: (updater: (prev: BoardFixtureV1) => BoardFixtureV1) => void;
  activePaneId: Puzzle2dPlayPaneId;
  setActivePaneId: (id: Puzzle2dPlayPaneId) => void;
  selectionIds: Set<string>;
  setSelectionIds: (ids: readonly string[]) => void;
  preselection: BoardPreselectSnapshot;
  setPreselection: (snapshot: BoardPreselectSnapshot) => void;
  hoveredId: string | null;
  /** @emoji 🖱️ Pane that currently owns pointer hover updates for shared {@link Puzzle2dPlayShellValue.hoveredId}. */
  hoverSourcePane: Puzzle2dPlayPaneId | null;
  setHoverPane: (pane: Puzzle2dPlayPaneId) => void;
  setHoverForPane: (pane: Puzzle2dPlayPaneId, id: string | null) => void;
  clearHoverForPane: (pane: Puzzle2dPlayPaneId) => void;
  /** @emoji 🔁 Rewrites selection ids when an object id changes (`replacedId` → `replacementId`); unrelated to edge endpoint fields. */
  remapIdInSelections: (replacedId: string, replacementId: string) => void;
  camerasByPane: Record<Puzzle2dPlayPaneId, CameraState>;
  /** @emoji 📷 Writes the **active** pane’s imperative camera (wheel/pan) into that pane’s entry in {@link puzzle2dPlayPaneCamerasBaseline}; other panes unchanged. */
  syncBaselineFromViewportCamera: (cam: CameraState) => void;
  puzzle2dSelectionMethod: BoardSelectionMethod;
  setBoardSelectionMethod: (value: BoardSelectionMethod) => void;
  puzzle2dSelectionMode: BoardSelectionMode;
  setBoardSelectionMode: (value: BoardSelectionMode) => void;
  puzzle2dSelectionTargets: BoardSelectionTargets;
  setBoardSelectionTargets: (value: BoardSelectionTargets | ((prev: BoardSelectionTargets) => BoardSelectionTargets)) => void;
  puzzle2dGridSnapEnabled: boolean;
  setBoardGridSnapEnabled: (value: boolean) => void;
  /** @emoji 📶 Per-pane LOD select value (`automatic` or a pinned tier). */
  boardLodModeByPane: Record<Puzzle2dPlayPaneId, BoardLodModeKind>;
  setBoardLodModeForPane: (pane: Puzzle2dPlayPaneId, mode: BoardLodModeKind) => void;
  /** @emoji 🗑️ Drops ids from the shared fixture after the canvas emits structural delete events. */
  applyStructuralDelete: (kind: "edge" | "node", id: string) => void;
  /** @emoji ⏯️ When true, play runs layout work on `requestAnimationFrame` (graph packs multiple WASM passes per ~14ms frame; tree one pass per frame). */
  puzzle2dRedrawPlaying: boolean;
  setBoardRedrawPlaying: (value: boolean) => void;
  boardRedrawMode: BoardRedrawModeKind;
  setBoardRedrawMode: (value: BoardRedrawModeKind) => void;
  forceLayoutFullIterations: number;
  setForceLayoutFullIterations: (value: number) => void;
  forceLayoutIdealEdgeLength: number;
  setForceLayoutIdealEdgeLength: (value: number) => void;
  forceLayoutGravity: number;
  setForceLayoutGravity: (value: number) => void;
  forceLayoutRepulsionStrength: number;
  setForceLayoutRepulsionStrength: (value: number) => void;
  boardRedrawPlayMaxItersPerFrame: number;
  setBoardRedrawPlayMaxItersPerFrame: (value: number) => void;
  boardRedrawProgressiveEnabled: boolean;
  setBoardRedrawProgressiveEnabled: (value: boolean) => void;
  boardRedrawProgressiveAutoStopMs: number;
  setBoardRedrawProgressiveAutoStopMs: (value: number) => void;
  /** @emoji 🔁 Restarts progressive iteration ramp and auto-stop clock (used when the user drags a node during play). */
  resetBoardRedrawProgressiveEpoch: () => void;
  treeLayoutLayerSpacing: number;
  setTreeLayoutLayerSpacing: (value: number) => void;
  treeLayoutSiblingGap: number;
  setTreeLayoutSiblingGap: (value: number) => void;
  treeLayoutDirection: BoardHierarchicalTreeDirectionKind;
  setTreeLayoutDirection: (value: BoardHierarchicalTreeDirectionKind) => void;
  applyBoardRedrawOnce: () => void;
  applyBoardRedrawHandlesOnce: () => void;
  boardRedrawHandlesAfterNodes: boolean;
  setBoardRedrawHandlesAfterNodes: (value: boolean) => void;
}

class Puzzle2dPlayHierarchyPanelDefinition extends PureSidePanelTabDefinition {
  constructor(private readonly buildTree: () => UiTreeNode) {
    super();
  }

  resolveTab(): SidePanelTabConfig {
    return {
      id: PUZZLE_2D_PLAY_HIERARCHY_TAB_ID,
      icon: ListTree,
      order: 0,
      tree: new StaticTreePanelDefinition({ sections: this.buildTree().sections as TreeDataSection[] }),
    };
  }
}

class Puzzle2dPlayLibraryPanelDefinition extends PureSidePanelTabDefinition {
  resolveTab(): SidePanelTabConfig {
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
            content: <BoardFixtureLibraryPanel />,
            items: [],
          },
        ],
      }),
    };
  }
}

class Puzzle2dPlayInspectorPanelDefinition extends PureSidePanelTabDefinition {
  constructor(private readonly buildSections: () => TreeDataSection[]) {
    super();
  }

  resolveTab(): SidePanelTabConfig {
    return {
      id: "puzzle-2d-play-inspector",
      icon: ClipboardList,
      order: 0,
      tree: new StaticTreePanelDefinition({ sections: this.buildSections() }),
    };
  }
}

class Puzzle2dPlaySettingsPanelDefinition extends PureSidePanelTabDefinition {
  resolveTab(): SidePanelTabConfig {
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

const Puzzle2dPlayLodRuntimeContext = reactHostPort.createContext<((pane: Puzzle2dPlayPaneId, lod: BoardDrawLodKind) => void) | null>(null);

function usePuzzle2dPlayShell(): Puzzle2dPlayShellValue {
  const value = reactHostPort.useContext(Puzzle2dPlayShellContext);
  if (!value) {
    throw new Error("usePuzzle2dPlayShell must be used inside Puzzle2dPlayShellContext.");
  }
  return value;
}
// #endregion 🔖ShellContext

// #region 🔖PlayRedrawHelpers
function newBoardAuthoringId(prefix: string): string {
  if (typeof globalThis.crypto !== "undefined" && typeof globalThis.crypto.randomUUID === "function") {
    return `${prefix}-${globalThis.crypto.randomUUID()}`;
  }
  return `${prefix}-${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 10)}`;
}

/** @emoji 📐 Default node span in px: circle radius = span/2; rectangle width = height = span (40×40). */
const PUZZLE_2D_PLAY_DEFAULT_NODE_SIZE_PX = 40;

const BOARD_PLAYRedraw_FRAME_BUDGET_MS = 14;

/** @emoji 📈 Force-graph play: iteration budget per inner WASM call ramps from 2 up to `playMax` over `autoStopMs` (or ~3.8s when stop is off). */
function boardPlayProgressiveForceIters(elapsedMs: number, autoStopMs: number, playMax: number): number {
  const cap = Math.max(4, Math.min(500, Math.round(playMax)));
  const rampWindow = autoStopMs > 0 ? autoStopMs * 0.88 : 3800;
  const t = Math.min(1, elapsedMs / Math.max(100, rampWindow));
  return Math.max(2, Math.round(2 + t * (cap - 2)));
}

/** @emoji 📐 Builds {@link BoardRedrawLayoutOptions} for the active pane camera center and redraw mode. */
function boardPlayRedrawLayoutOpts(
  pane: Puzzle2dPlayPaneId,
  camerasByPane: Record<Puzzle2dPlayPaneId, CameraState>,
  mode: BoardRedrawModeKind,
  forceIters: number,
  forceIdealEdge: number,
  forceGravity: number,
  forceRepulsion: number,
  treeLayerSpacing: number,
  treeSiblingGap: number,
  treeDirection: BoardHierarchicalTreeDirectionKind,
  redrawHandlesAfter: boolean,
): BoardRedrawLayoutOptions {
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
  const fg: BoardForceGraphLayoutOptions = {
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
/** @emoji ⚙️ Board play redraw settings: play uses requestAnimationFrame (packed WASM per frame), progressive ramp, and per-mode layout parameters. */
function Puzzle2dPlaySettingsPanel(): ReactElement {
  const {
    activePaneId,
    applyBoardRedrawHandlesOnce,
    applyBoardRedrawOnce,
    boardRedrawHandlesAfterNodes,
    boardRedrawMode,
    boardRedrawPlayMaxItersPerFrame,
    boardRedrawProgressiveAutoStopMs,
    boardRedrawProgressiveEnabled,
    forceLayoutFullIterations,
    forceLayoutGravity,
    forceLayoutIdealEdgeLength,
    forceLayoutRepulsionStrength,
    setBoardRedrawMode,
    setBoardRedrawHandlesAfterNodes,
    setBoardRedrawPlayMaxItersPerFrame,
    setBoardRedrawProgressiveAutoStopMs,
    setBoardRedrawProgressiveEnabled,
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
        <Label id="board.play.settings.redraw.mode" label="Layout kind">
          <Select onValueChange={(v) => setBoardRedrawMode(v as BoardRedrawModeKind)} value={boardRedrawMode}>
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
          <input checked={boardRedrawHandlesAfterNodes} className="accent-accent size-3.5 shrink-0" id="puzzle-2d-play-redraw-handles-after-nodes" onChange={(e) => setBoardRedrawHandlesAfterNodes(e.target.checked)} type="checkbox" />
          <label className="text-muted-foreground cursor-pointer select-none text-[11px] leading-snug" htmlFor="puzzle-2d-play-redraw-handles-after-nodes">
            Also redraw handles after node redraw
          </label>
        </div>
        <div className="flex items-center gap-2">
          <input checked={boardRedrawProgressiveEnabled} className="accent-accent size-3.5 shrink-0" id="puzzle-2d-play-redraw-progressive" onChange={(e) => setBoardRedrawProgressiveEnabled(e.target.checked)} type="checkbox" />
          <label className="text-muted-foreground cursor-pointer select-none text-[11px] leading-snug" htmlFor="puzzle-2d-play-redraw-progressive">
            Progressive iterations while play is on (graph ramps up; tree still one pass per frame)
          </label>
        </div>
        <Label id="board.play.settings.redraw.autoStopMs" label="Auto-stop play after (ms, 0 = off)">
          <Slider id="puzzle-2d-play-slider-redraw-autostop" max={12000} min={0} step={250} value={[boardRedrawProgressiveAutoStopMs]} onValueChange={(vals) => setBoardRedrawProgressiveAutoStopMs(vals[0] ?? 3000)} />
        </Label>
        {boardRedrawMode === "force-graph" ? (
          <Label id="board.play.settings.redraw.playMaxIters" label="Max iterations per WASM call (play ramp ceiling)">
            <Slider id="puzzle-2d-play-slider-redraw-play-max-iters" max={220} min={12} step={2} value={[boardRedrawPlayMaxItersPerFrame]} onValueChange={(vals) => setBoardRedrawPlayMaxItersPerFrame(vals[0] ?? 96)} />
          </Label>
        ) : (
          <p className="text-muted-foreground text-[11px] leading-snug">Tree redraw runs once per animation frame while play is on; use auto-stop to end play after a duration.</p>
        )}
        {boardRedrawMode === "force-graph" ? (
          <>
            <div className="text-muted-foreground pt-1 text-[11px] font-medium uppercase tracking-wide">Graph</div>
            <Label id="board.play.settings.force.fullIterations" label="Iterations (apply once)">
              <Slider id="puzzle-2d-play-slider-force-full-iters" max={720} min={24} step={4} value={[forceLayoutFullIterations]} onValueChange={(vals) => setForceLayoutFullIterations(vals[0] ?? 200)} />
            </Label>
            <Label id="board.play.settings.force.idealEdge" label="Ideal edge (px)">
              <Slider id="puzzle-2d-play-slider-force-ideal" max={160} min={20} step={2} value={[forceLayoutIdealEdgeLength]} onValueChange={(vals) => setForceLayoutIdealEdgeLength(vals[0] ?? 64)} />
            </Label>
            <Label id="board.play.settings.force.repulsion" label="Repulsion (medium 80, ±40)">
              <Slider id="puzzle-2d-play-slider-force-repulsion" max={120} min={40} step={2} value={[forceLayoutRepulsionStrength]} onValueChange={(vals) => setForceLayoutRepulsionStrength(vals[0] ?? 80)} />
            </Label>
            <Label id="board.play.settings.force.gravity" label="Gravity">
              <Slider id="puzzle-2d-play-slider-force-gravity" max={0.05} min={0} step={0.002} value={[forceLayoutGravity]} onValueChange={(vals) => setForceLayoutGravity(vals[0] ?? 0)} />
            </Label>
          </>
        ) : (
          <>
            <div className="text-muted-foreground pt-1 text-[11px] font-medium uppercase tracking-wide">Tree</div>
            <Label id="board.play.settings.tree.layerSpacing" label="Layer spacing (px)">
              <Slider id="puzzle-2d-play-slider-tree-layer" max={280} min={40} step={4} value={[treeLayoutLayerSpacing]} onValueChange={(vals) => setTreeLayoutLayerSpacing(vals[0] ?? 120)} />
            </Label>
            <Label id="board.play.settings.tree.siblingGap" label="Sibling gap (px)">
              <Slider id="puzzle-2d-play-slider-tree-sibling" max={120} min={0} step={2} value={[treeLayoutSiblingGap]} onValueChange={(vals) => setTreeLayoutSiblingGap(vals[0] ?? 28)} />
            </Label>
            <Label id="board.play.settings.tree.direction" label="Direction">
              <Select onValueChange={(v) => setTreeLayoutDirection(v as BoardHierarchicalTreeDirectionKind)} value={treeLayoutDirection}>
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
        <Button className="h-8 w-full text-xs" id="puzzle-2d-play-redraw-nodes" type="button" variant="secondary" onClick={applyBoardRedrawOnce}>
          Redraw nodes
        </Button>
        <div className="text-muted-foreground border-t border-element pt-2 text-[11px] font-medium uppercase tracking-wide">Redraw handles</div>
        <p className="text-muted-foreground text-[11px] leading-snug">Each edge uses the straight segment between node centers; handle anchors move to where that segment meets each shape (shortest chord through the bodies).</p>
        <Button className="h-8 w-full text-xs" id="puzzle-2d-play-redraw-handles" type="button" variant="secondary" onClick={applyBoardRedrawHandlesOnce}>
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
function BoardPaneChrome({ children, paneId }: { children: ReactNode; paneId: Puzzle2dPlayPaneId }): ReactElement {
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

function boardPlayLodCanvasProps(mode: BoardLodModeKind): { automaticLod: boolean; lod?: BoardDrawLodKind } {
  if (mode === BOARD_LOD_MODE_AUTOMATIC) {
    return { automaticLod: true };
  }
  return { automaticLod: false, lod: mode };
}

function Puzzle2dPlayPaneCanvas({ paneId, showBackgroundMenu }: { paneId: Puzzle2dPlayPaneId; showBackgroundMenu?: boolean }): ReactElement {
  const {
    activePaneId,
    applyStructuralDelete,
    puzzle2dGridSnapEnabled,
    boardLodModeByPane,
    puzzle2dRedrawPlaying,
    puzzle2dSelectionMethod,
    puzzle2dSelectionMode,
    puzzle2dSelectionTargets,
    fixture,
    handleCanvasFixtureDrop,
    camerasByPane,
    hoveredId,
    preselection,
    resetBoardRedrawProgressiveEpoch,
    selectionIds,
    setHoverForPane,
    setPreselection,
    setSelectionIds,
    syncBaselineFromViewportCamera,
  } = usePuzzle2dPlayShell();
  const camera = camerasByPane[paneId];
  const lodProps = boardPlayLodCanvasProps(boardLodModeByPane[paneId]);
  const reportEffectiveLod = reactHostPort.useContext(Puzzle2dPlayLodRuntimeContext);
  const onLodChange = reactHostPort.useCallback((lod: BoardDrawLodKind) => reportEffectiveLod?.(paneId, lod), [paneId, reportEffectiveLod]);
  const selection = reactHostPort.useMemo(() => normalizeBoardSelectionProp([...selectionIds]), [selectionIds]);
  const onSelect = reactHostPort.useCallback((snapshot: BoardSelectionSnapshot) => setSelectionIds(snapshot.ids), [setSelectionIds]);
  const onPreselect = reactHostPort.useCallback((snapshot: BoardPreselectSnapshot) => setPreselection(snapshot), [setPreselection]);
  const onHover = reactHostPort.useCallback(
    (payload: { id: string | null }) => {
      setHoverForPane(paneId, payload.id);
    },
    [paneId, setHoverForPane],
  );
  const demoNodeId = fixture.nodes[0]?.id;
  const demoEdgeId = fixture.edges[0]?.id;
  const sceneMarkers = reactHostPort.useMemo(
    () =>
      boardFixtureSceneMarkers(fixture, {
        nodeContextMenuForId: (id) => (id === demoNodeId ? boardPlayDemoNodeContextMenu : undefined),
        edgeContextMenuForId: (id) => (id === demoEdgeId ? boardPlayDemoEdgeContextMenu : undefined),
      }),
    [demoEdgeId, demoNodeId, fixture],
  );
  const onCanvasDelete = reactHostPort.useCallback(
    (payload: BoardStructureDeletePayload) => {
      if (payload.kind === "wire") {
        return;
      }
      applyStructuralDelete(payload.kind, payload.id);
    },
    [applyStructuralDelete],
  );
  const onCanvasDrag = reactHostPort.useCallback(
    (_payload: { id: string }) => {
      if (!puzzle2dRedrawPlaying) {
        return;
      }
      resetBoardRedrawProgressiveEpoch();
    },
    [puzzle2dRedrawPlaying, resetBoardRedrawProgressiveEpoch],
  );
  return (
    <BoardPaneChrome paneId={paneId}>
      <BoardCanvas
        {...lodProps}
        onLodChange={onLodChange}
        camera={camera}
        className="min-h-0 flex-1"
        contextMenu={showBackgroundMenu ? boardPlayCanvasBackgroundMenu : undefined}
        fixtureDragDrop
        gridSnapEnabled={puzzle2dGridSnapEnabled}
        hoveredId={hoveredId}
        kindCatalogs={PUZZLE_2D_PLAY_DEFAULT_KIND_CATALOGS}
        lodZoomThresholds={DEFAULT_BOARD_LOD_ZOOM_THRESHOLDS}
        onCamera={activePaneId === paneId ? syncBaselineFromViewportCamera : undefined}
        onDelete={onCanvasDelete}
        onDrag={onCanvasDrag}
        onFixtureDrop={(d) => handleCanvasFixtureDrop(paneId, d)}
        onHover={onHover}
        onPreselect={onPreselect}
        onSelect={onSelect}
        preselection={preselection}
        selection={selection}
        selectionMethod={puzzle2dSelectionMethod}
        selectionMode={puzzle2dSelectionMode}
        selectionTargets={puzzle2dSelectionTargets}
      >
        {sceneMarkers}
      </BoardCanvas>
    </BoardPaneChrome>
  );
}

function Puzzle2dPlayPaneSurfaceHost({ node }: { readonly node: UiBoardHostSurfaceNode }): ReactElement {
  if (node.controllerId !== PUZZLE_2D_PLAY_CONTROLLER_ID || node.surfaceId !== PUZZLE_2D_PLAY_SURFACE_ID) {
    return <div className="p-2 text-xs text-muted-foreground">Invalid board surface binding</div>;
  }
  const paneId = node.paneId as Puzzle2dPlayPaneId;
  return <Puzzle2dPlayPaneCanvas paneId={paneId} showBackgroundMenu={paneId === "2d-overview"} />;
}

let puzzle2dPlayChromeRegistered = false;

/** @emoji 🧊 Registers board play surface host, window bodies, and tab icons (called from `@framework/playground/renderer/react`). */
export function registerPuzzle2dPlaySurfaceHosts(): void {
  if (puzzle2dPlayChromeRegistered) return;
  puzzle2dPlayChromeRegistered = true;
  registerUiBoardSurfaceHost(PUZZLE_2D_PLAY_SURFACE_ID, Puzzle2dPlayPaneSurfaceHost);
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

const PUZZLE_2D_PLAY_PALETTE_CIRCLE_DRAG_FIXTURE: BoardFixtureV1 =
  parseBoardFixtureV1({
    camera: { x: 0, y: 0, zoom: 1 },
    edges: [],
    meta: { boardFixtureDragKind: BOARD_FIXTURE_DRAG_KIND_PALETTE_NODE },
    nodes: [{ handles: [{ angle: 0, id: "palette-seed-circle.h0" }], id: "palette-seed-circle", radius: PUZZLE_2D_PLAY_DEFAULT_NODE_SIZE_PX / 2, x: 0, y: 0 }],
    schema: "puzzle.2d.fixture/v1",
  }) ??
  (() => {
    throw new Error("Board play: palette circle drag fixture failed validation.");
  })();

const PUZZLE_2D_PLAY_PALETTE_RECTANGLE_DRAG_FIXTURE: BoardFixtureV1 =
  parseBoardFixtureV1({
    camera: { x: 0, y: 0, zoom: 1 },
    edges: [],
    meta: { boardFixtureDragKind: BOARD_FIXTURE_DRAG_KIND_PALETTE_NODE },
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
    throw new Error("Board play: palette rectangle drag fixture failed validation.");
  })();

/** @emoji 🧩 When {@link BOARD_FIXTURE_DRAG_KIND_PALETTE_NODE} is on meta, returns one node placed at the drop world point; else null so the scene should be replaced. */
function mergePaletteNodeFromDrop(detail: BoardFixtureDropDetail): BoardFixtureNodeV1 | null {
  if (detail.fixture.meta?.boardFixtureDragKind !== BOARD_FIXTURE_DRAG_KIND_PALETTE_NODE) {
    return null;
  }
  const template = detail.fixture.nodes[0];
  if (!template) {
    return null;
  }
  const newId = newBoardAuthoringId("node");
  return {
    ...template,
    handles: template.handles.map((h, i) => ({ ...h, id: `${newId}.h${i}` })),
    id: newId,
    x: detail.world.x,
    y: detail.world.y,
  };
}

/** @emoji 👻 Draggable chip with drag image rendered under `document.body` so host panel overflow does not clip the preview. */
function BoardFixturePaletteDraggable(props: { fixture: BoardFixtureV1; label: string; preview: ReactNode }): ReactElement {
  const { fixture: dragFixture, label, preview } = props;
  const dragProps = useNativeDragAndDrop(
    reactHostPort.useMemo(
      () => ({
        onDragStart: (event: React.DragEvent<HTMLDivElement>) => {
          event.dataTransfer.setData(BOARD_FIXTURE_DRAG_V1_MIME, encodeBoardFixtureForDragV1(dragFixture));
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

/** @emoji 📥 Left rail: drag the active graph onto a board pane (in-app MIME payload, not filesystem JSON files). */
function BoardFixtureLibraryPanel(): ReactElement {
  const { fixture } = usePuzzle2dPlayShell();

  const shelfDragProps = useNativeDragAndDrop(
    reactHostPort.useMemo(
      () => ({
        onDragStart: (event: React.DragEvent<HTMLDivElement>) => {
          event.dataTransfer.setData(BOARD_FIXTURE_DRAG_V1_MIME, encodeBoardFixtureForDragV1(fixture));
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
          <BoardFixturePaletteDraggable fixture={PUZZLE_2D_PLAY_PALETTE_CIRCLE_DRAG_FIXTURE} label="Drag circle onto the board" preview={<div className="border-primary size-10 shrink-0 rounded-full border-2 bg-accent/30" />} />
          <BoardFixturePaletteDraggable fixture={PUZZLE_2D_PLAY_PALETTE_RECTANGLE_DRAG_FIXTURE} label="Drag rectangle onto the board" preview={<div className="border-primary size-10 shrink-0 rounded-sm border-2 bg-accent/30" />} />
        </div>
      </div>
      <div className="border-element bg-muted/30 flex min-h-30 cursor-grab flex-col justify-center gap-2 rounded-md border p-4 active:cursor-grabbing" {...shelfDragProps}>
        <p className="font-medium">Active graph</p>
        <p className="text-muted-foreground text-xs">Drag onto any board tab to load this graph (same payload for all panes).</p>
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

function findNode(fixture: BoardFixtureV1, id: string): BoardFixtureNodeV1 | undefined {
  return fixture.nodes.find((n) => n.id === id);
}

function findEdge(fixture: BoardFixtureV1, id: string): BoardFixtureEdgeV1 | undefined {
  return fixture.edges.find((e) => e.id === id);
}

function findHandleOwner(fixture: BoardFixtureV1, handleId: string): { node: BoardFixtureNodeV1; handleId: string } | undefined {
  for (const node of fixture.nodes) {
    if (node.handles.some((h) => h.id === handleId)) {
      return { handleId, node };
    }
  }
  return undefined;
}

function findHandle(fixture: BoardFixtureV1, handleId: string): BoardFixtureHandleV1 | undefined {
  for (const node of fixture.nodes) {
    const h = node.handles.find((x) => x.id === handleId);
    if (h) {
      return h;
    }
  }
  return undefined;
}

function nodeIsRectangle(n: BoardFixtureNodeV1): n is BoardFixtureRectangleNodeV1 {
  return n.shape === "rectangle";
}

function allEqual<T>(values: T[]): boolean {
  if (values.length === 0) {
    return true;
  }
  const first = values[0];
  return values.every((v) => v === first);
}

function listHandleIds(fixture: BoardFixtureV1): string[] {
  const out: string[] = [];
  for (const node of fixture.nodes) {
    for (const h of node.handles) {
      out.push(h.id);
    }
  }
  out.sort((a, b) => a.localeCompare(b));
  return out;
}

function toCircleNode(n: BoardFixtureRectangleNodeV1): BoardFixtureCircleNodeV1 {
  const { width, height, shape: _s, ...rest } = n;
  const radius = Math.min(width, height) / 2;
  return { ...rest, radius, shape: "circle" };
}

function toRectangleNode(n: BoardFixtureCircleNodeV1): BoardFixtureRectangleNodeV1 {
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

/** @emoji ⭕ Draggable ring control for handle polar angle `t` (radians, east-zero CCW in board space). */
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

  const pointerController = reactHostPort.useMemo(
    () =>
      new PointerDragController<HTMLDivElement>({
        onStart: (event) => {
          event.preventDefault();
          setFromClient(event.clientX, event.clientY);
        },
        onMove: (event) => {
          setFromClient(event.clientX, event.clientY);
        },
      }),
    [setFromClient],
  );

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
        {...(angleUniform ? pointerController.getProps() : {})}
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
  remapIdInSelections,
}: {
  fixture: BoardFixtureV1;
  nodeIds: readonly string[];
  patchFixture: (updater: (prev: BoardFixtureV1) => BoardFixtureV1) => void;
  remapIdInSelections: (replacedId: string, replacementId: string) => void;
}): ReactElement {
  const idSet = reactHostPort.useMemo(() => new Set(nodeIds), [nodeIds]);
  const targets = reactHostPort.useMemo(() => nodeIds.map((id) => findNode(fixture, id)).filter((n): n is BoardFixtureNodeV1 => Boolean(n)), [fixture, nodeIds]);

  const textValues = targets.map((n) => boardFixtureNodeCaption(n) ?? "");
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
    (updater: (n: BoardFixtureNodeV1) => BoardFixtureNodeV1) => {
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
      {nodeIds.length === 1 ? (
        <Label id="puzzle-2d-play.inspector.node.id" label="Id">
          <Input
            className="h-7 font-mono text-xs"
            defaultValue={nodeIds[0]}
            key={nodeIds[0]}
            onBlur={(e) => {
              const nextId = e.currentTarget.value.trim();
              const oldId = nodeIds[0];
              if (!oldId || !nextId || nextId === oldId) {
                return;
              }
              patchFixture((prev) => ({
                ...prev,
                nodes: prev.nodes.map((n) => (n.id === oldId ? { ...n, id: nextId } : n)),
              }));
              remapIdInSelections(oldId, nextId);
            }}
          />
        </Label>
      ) : null}
      <Label id="puzzle-2d-play.inspector.node.name" label="Name">
        <Input className="h-7 font-mono text-xs" onChange={(e: ChangeEvent<HTMLInputElement>) => onText(e.target.value)} placeholder={textUniform ? undefined : "Mixed"} value={textValue} />
      </Label>
      <Label id="puzzle-2d-play.inspector.node.icon" label="Icon">
        <IconSelector classifyElementsBoardIconSelectorMode={classifyElementsBoardIconSelectorMode} id="puzzle-2d-play.inspector.node.icon.selector" onChange={onIconKind} uniform={iconKindUniform} value={iconKindValue} />
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
  remapIdInSelections,
}: {
  fixture: BoardFixtureV1;
  handleIds: readonly string[];
  patchFixture: (updater: (prev: BoardFixtureV1) => BoardFixtureV1) => void;
  remapIdInSelections: (replacedId: string, replacementId: string) => void;
}): ReactElement {
  const idSet = reactHostPort.useMemo(() => new Set(handleIds), [handleIds]);
  const handles = reactHostPort.useMemo(() => handleIds.map((id) => findHandle(fixture, id)).filter((h): h is BoardFixtureHandleV1 => Boolean(h)), [fixture, handleIds]);
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
    (updater: (h: BoardFixtureHandleV1) => BoardFixtureHandleV1) => {
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
            <IconSelector classifyElementsBoardIconSelectorMode={classifyElementsBoardIconSelectorMode} id="puzzle-2d-play.inspector.handle.icon.selector" onChange={onIconKind} uniform={iconKindUniform} value={iconKindValue} />
          </Label>
          {handleIds.length === 1 ? (
            <Label id="puzzle-2d-play.inspector.handle.id" label="Id">
              <Input
                className="h-7 font-mono text-xs"
                defaultValue={handleIds[0]}
                key={handleIds[0]}
                onBlur={(e) => {
                  const nextId = e.currentTarget.value.trim();
                  const oldId = handleIds[0];
                  if (!oldId || !nextId || nextId === oldId) {
                    return;
                  }
                  patchFixture((prev) => ({
                    ...prev,
                    edges: prev.edges.map((edge) => ({
                      ...edge,
                      source: edge.source === oldId ? nextId : edge.source,
                      target: edge.target === oldId ? nextId : edge.target,
                    })),
                    nodes: prev.nodes.map((node) => ({
                      ...node,
                      handles: node.handles.map((h) => (h.id === oldId ? { ...h, id: nextId } : h)),
                    })),
                  }));
                  remapIdInSelections(oldId, nextId);
                }}
              />
            </Label>
          ) : null}
        </div>
      </div>
    </div>
  );
}

/** @emoji 🪢 Batch edge inspector: endpoints and id (single). */
function InspectorEdgeBatch({
  fixture,
  edgeIds,
  patchFixture,
  remapIdInSelections,
}: {
  fixture: BoardFixtureV1;
  edgeIds: readonly string[];
  patchFixture: (updater: (prev: BoardFixtureV1) => BoardFixtureV1) => void;
  remapIdInSelections: (replacedId: string, replacementId: string) => void;
}): ReactElement {
  const idSet = reactHostPort.useMemo(() => new Set(edgeIds), [edgeIds]);
  const edges = reactHostPort.useMemo(() => edgeIds.map((id) => findEdge(fixture, id)).filter((e): e is BoardFixtureEdgeV1 => Boolean(e)), [edgeIds, fixture]);
  const sources = edges.map((e) => e.source);
  const targets = edges.map((e) => e.target);
  const sourceUniform = allEqual(sources);
  const targetUniform = allEqual(targets);
  const handleOptions = reactHostPort.useMemo(() => listHandleIds(fixture), [fixture]);

  const patchEdges = reactHostPort.useCallback(
    (updater: (e: BoardFixtureEdgeV1) => BoardFixtureEdgeV1) => {
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
                {hid}
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
                {hid}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </Label>
      {edgeIds.length === 1 ? (
        <Label id="puzzle-2d-play.inspector.edge.id" label="Id">
          <Input
            className="h-7 font-mono text-xs"
            defaultValue={edgeIds[0]}
            key={edgeIds[0]}
            onBlur={(e) => {
              const nextId = e.currentTarget.value.trim();
              const oldId = edgeIds[0];
              if (!oldId || !nextId || nextId === oldId) {
                return;
              }
              patchFixture((prev) => ({
                ...prev,
                edges: prev.edges.map((edge) => (edge.id === oldId ? { ...edge, id: nextId } : edge)),
              }));
              remapIdInSelections(oldId, nextId);
            }}
          />
        </Label>
      ) : null}
    </div>
  );
}

/** @emoji 🔎 Playground tree inspector sections for the active pane selection (every section has items). */
function buildPuzzle2dPlayInspectorSections(shell: Puzzle2dPlayShellValue): TreeDataSection[] {
  const { activePaneId, fixture, patchFixture, remapIdInSelections, selectionIds } = shell;
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
        {activePaneId} · {ids.length} selected id{ids.length === 1 ? "" : "s"}
      </p>,
    ),
  ];
  if (nodeIds.length > 0) {
    sections.push(
      playgroundPanelSection(
        "puzzle-2d-play-inspector-nodes",
        `Nodes (${nodeIds.length})`,
        <InspectorNodeBatch fixture={fixture} nodeIds={nodeIds} patchFixture={patchFixture} remapIdInSelections={remapIdInSelections} />,
      ),
    );
  }
  if (handleIds.length > 0) {
    sections.push(
      playgroundPanelSection(
        "puzzle-2d-play-inspector-handles",
        `Handles (${handleIds.length})`,
        <InspectorHandleBatch fixture={fixture} handleIds={handleIds} patchFixture={patchFixture} remapIdInSelections={remapIdInSelections} />,
      ),
    );
  }
  if (edgeIds.length > 0) {
    sections.push(
      playgroundPanelSection(
        "puzzle-2d-play-inspector-edges",
        `Edges (${edgeIds.length})`,
        <InspectorEdgeBatch edgeIds={edgeIds} fixture={fixture} patchFixture={patchFixture} remapIdInSelections={remapIdInSelections} />,
      ),
    );
  }
  if (nodeIds.length === 0 && handleIds.length === 0 && edgeIds.length === 0) {
    sections.push(
      playgroundPanelSection(
        "puzzle-2d-play-inspector-unknown",
        "Selection",
        <p className="font-mono text-[11px] text-warning-foreground leading-snug">{ids.join(", ")}</p>,
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
  boardRedrawHandlesAfterNodes: boolean;
  boardRedrawProgressiveAutoStopMs: number;
  boardRedrawProgressiveEnabled: boolean;
  boardRedrawPlayMaxItersPerFrame: number;
  camerasByPane: Record<Puzzle2dPlayPaneId, CameraState>;
  forceLayoutGravity: number;
  forceLayoutIdealEdgeLength: number;
  forceLayoutRepulsionStrength: number;
  mode: BoardRedrawModeKind;
  treeLayoutDirection: BoardHierarchicalTreeDirectionKind;
  treeLayoutLayerSpacing: number;
  treeLayoutSiblingGap: number;
}

// #region 🔖Entrypoint
const initialFixture = PUZZLE_2D_PLAY_DEFAULT_FIXTURE;

function Puzzle2dPlayInner({ boardRuntime }: { readonly boardRuntime: Platform }): ReactElement {
  const [fixture, setFixtureState] = reactHostPort.useState<BoardFixtureV1>(initialFixture);
  const fixtureRef = reactHostPort.useRef<BoardFixtureV1>(fixture);
  fixtureRef.current = fixture;
  const [puzzle2dPlayPaneCamerasBaseline, setPuzzle2dPlayPaneCamerasBaseline] = reactHostPort.useState<Record<Puzzle2dPlayPaneId, CameraState>>(() => triptychCamerasFromFixture(initialFixture));
  const puzzle2dPlayPaneCamerasBaselineRef = reactHostPort.useRef(puzzle2dPlayPaneCamerasBaseline);
  puzzle2dPlayPaneCamerasBaselineRef.current = puzzle2dPlayPaneCamerasBaseline;
  const [activePaneId, setActivePaneId] = reactHostPort.useState<Puzzle2dPlayPaneId>("2d-overview");
  const activePaneIdRef = reactHostPort.useRef(activePaneId);
  activePaneIdRef.current = activePaneId;
  const [selectionIds, setSelectionIdsState] = reactHostPort.useState<Set<string>>(() => selectionSeedForFixture(initialFixture));
  const [preselection, setPreselection] = reactHostPort.useState<BoardPreselectSnapshot>(BOARD_PRESELECT_EMPTY);
  const [hoveredId, setHoveredId] = reactHostPort.useState<string | null>(null);
  const [hoverSourcePane, setHoverSourcePane] = reactHostPort.useState<Puzzle2dPlayPaneId | null>(null);
  const hoverSourcePaneRef = reactHostPort.useRef<Puzzle2dPlayPaneId | null>(hoverSourcePane);
  hoverSourcePaneRef.current = hoverSourcePane;
  const [puzzle2dSelectionMethod, setBoardSelectionMethod] = reactHostPort.useState<BoardSelectionMethod>("rectangle");
  const [puzzle2dSelectionMode, setBoardSelectionMode] = reactHostPort.useState<BoardSelectionMode>("default");
  const [puzzle2dSelectionTargets, setBoardSelectionTargets] = reactHostPort.useState<BoardSelectionTargets>(() => ({ ...BOARD_SELECTION_TARGETS_DEFAULT }));
  const [puzzle2dGridSnapEnabled, setBoardGridSnapEnabled] = reactHostPort.useState(false);
  const puzzle2dShellController = boardRuntime.getActiveApp()?.controller as Puzzle2dPlayShellController | undefined;
  const shellGeneration = reactHostPort.useSyncExternalStore(
    (onStoreChange) => boardRuntime.subscribe(onStoreChange),
    () => boardRuntime.generation,
    () => 0,
  );
  void shellGeneration;
  const boardLodModeByPane = puzzle2dShellController?.getLodModeByPane() ?? {
    "2d-detail": BOARD_LOD_MODE_AUTOMATIC,
    "2d-overview": BOARD_LOD_MODE_AUTOMATIC,
    "2d-selection": BOARD_LOD_MODE_AUTOMATIC,
  };
  const setBoardLodModeForPane = reactHostPort.useCallback(
    (pane: Puzzle2dPlayPaneId, mode: BoardLodModeKind) => {
      boardRuntime.commandBus.dispatch(PUZZLE_2D_PLAY_CONTROLLER_ID, "setLodModeForPane", { pane, value: mode });
    },
    [boardRuntime.commandBus],
  );
  const setBoardEffectiveLodForPane = reactHostPort.useCallback(
    (pane: Puzzle2dPlayPaneId, lod: BoardDrawLodKind) => {
      boardRuntime.commandBus.dispatch(PUZZLE_2D_PLAY_CONTROLLER_ID, "setEffectiveLodForPane", { pane, lod });
    },
    [boardRuntime.commandBus],
  );
  const onPuzzle2dPlayActiveWindowChange = reactHostPort.useCallback((windowKindId: string) => {
    if (windowKindId === "2d-overview" || windowKindId === "2d-detail" || windowKindId === "2d-selection") {
      setActivePaneId(windowKindId);
    }
  }, []);
  const [puzzle2dRedrawPlaying, setBoardRedrawPlaying] = reactHostPort.useState(false);
  const [forceLayoutFullIterations, setForceLayoutFullIterations] = reactHostPort.useState(200);
  const [forceLayoutIdealEdgeLength, setForceLayoutIdealEdgeLength] = reactHostPort.useState(64);
  const [forceLayoutGravity, setForceLayoutGravity] = reactHostPort.useState(0.012);
  const [forceLayoutRepulsionStrength, setForceLayoutRepulsionStrength] = reactHostPort.useState(80);
  const [boardRedrawPlayMaxItersPerFrame, setBoardRedrawPlayMaxItersPerFrame] = reactHostPort.useState(96);
  const [boardRedrawProgressiveEnabled, setBoardRedrawProgressiveEnabled] = reactHostPort.useState(true);
  const [boardRedrawProgressiveAutoStopMs, setBoardRedrawProgressiveAutoStopMs] = reactHostPort.useState(3000);
  const [boardRedrawMode, setBoardRedrawMode] = reactHostPort.useState<BoardRedrawModeKind>("force-graph");
  const [boardRedrawHandlesAfterNodes, setBoardRedrawHandlesAfterNodes] = reactHostPort.useState(false);
  const [treeLayoutLayerSpacing, setTreeLayoutLayerSpacing] = reactHostPort.useState(120);
  const [treeLayoutSiblingGap, setTreeLayoutSiblingGap] = reactHostPort.useState(28);
  const [treeLayoutDirection, setTreeLayoutDirection] = reactHostPort.useState<BoardHierarchicalTreeDirectionKind>("downwards");

  const puzzle2dRedrawPlayingRef = reactHostPort.useRef(puzzle2dRedrawPlaying);
  puzzle2dRedrawPlayingRef.current = puzzle2dRedrawPlaying;

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
        return { ...prev, edges: prev.edges.filter((e) => e.id !== id) };
      });
      pruneSelections([id]);
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
      return {
        ...prev,
        edges: prev.edges.filter((e) => !hset.has(e.source) && !hset.has(e.target)),
        nodes: prev.nodes.filter((x) => x.id !== id),
      };
    });
    pruneSelections([id, ...handleIds]);
  }, []);

  const setFixture = reactHostPort.useCallback((next: BoardFixtureV1) => {
    setFixtureState(next);
    setSelectionIdsState(selectionSeedForFixture(next));
    setPreselection(BOARD_PRESELECT_EMPTY);
    setHoveredId(null);
    hoverSourcePaneRef.current = null;
    setHoverSourcePane(null);
    setPuzzle2dPlayPaneCamerasBaseline(triptychCamerasFromFixture(next));
  }, []);

  const patchFixture = reactHostPort.useCallback((updater: (prev: BoardFixtureV1) => BoardFixtureV1) => {
    setFixtureState((prev) => updater(prev));
  }, []);

  const setSelectionIds = reactHostPort.useCallback((ids: readonly string[]) => {
    setSelectionIdsState(new Set(ids));
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

  const handleCanvasFixtureDrop = reactHostPort.useCallback(
    (pane: Puzzle2dPlayPaneId, detail: BoardFixtureDropDetail) => {
      skipNextCameraBasisResyncRef.current = true;
      const merged = mergePaletteNodeFromDrop(detail);
      if (merged) {
        patchFixture((prev) => ({ ...prev, nodes: [...prev.nodes, merged] }));
        setSelectionIds([merged.id]);
        return;
      }
      setFixture(detail.fixture);
    },
    [patchFixture, setFixture, setSelectionIds],
  );

  const remapIdInSelections = reactHostPort.useCallback((replacedId: string, replacementId: string) => {
    if (replacedId === replacementId) {
      return;
    }
    setSelectionIdsState((prev) => new Set([...prev].map((id) => (id === replacedId ? replacementId : id))));
  }, []);

  const cameraBasisFixtureRef = reactHostPort.useRef<BoardFixtureV1>(fixture);
  /** @emoji 📌 One-shot: sync {@link cameraBasisFixtureRef} without resetting {@link puzzle2dPlayPaneCamerasBaseline} after palette / shelf fixture drop. */
  const skipNextCameraBasisResyncRef = reactHostPort.useRef(false);
  const prevBoardRedrawPlayingRef = reactHostPort.useRef(false);
  const [cameraDisplayOverrideByPane, setCameraDisplayOverrideByPane] = reactHostPort.useState<Record<Puzzle2dPlayPaneId, CameraState> | null>(null);
  const cameraDisplayOverrideRef = reactHostPort.useRef<Record<Puzzle2dPlayPaneId, CameraState> | null>(null);
  cameraDisplayOverrideRef.current = cameraDisplayOverrideByPane;
  const suppressCameraBasisSyncRef = reactHostPort.useRef(false);
  const cameraPlayEndAnimRafRef = reactHostPort.useRef<number | null>(null);
  const boardPlayNodesRedrawCameraAnimRafRef = reactHostPort.useRef<number | null>(null);
  const boardPlayRedrawCameraChaseRef = reactHostPort.useRef<Record<Puzzle2dPlayPaneId, CameraState> | null>(null);
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
    const prevPlaying = prevBoardRedrawPlayingRef.current;
    const playJustStarted = puzzle2dRedrawPlaying && !prevPlaying;

    if (playJustStarted) {
      nodesRedrawEaseGenerationRef.current += 1;
      nodesRedrawEaseFromRef.current = null;
      if (cameraPlayEndAnimRafRef.current != null) {
        cancelAnimationFrame(cameraPlayEndAnimRafRef.current);
        cameraPlayEndAnimRafRef.current = null;
      }
      if (boardPlayNodesRedrawCameraAnimRafRef.current != null) {
        cancelAnimationFrame(boardPlayNodesRedrawCameraAnimRafRef.current);
        boardPlayNodesRedrawCameraAnimRafRef.current = null;
      }
      setCameraDisplayOverrideByPane(null);
      suppressCameraBasisSyncRef.current = false;
      cameraBasisFixtureRef.current = fixture;
      const prevCam = puzzle2dPlayPaneCamerasBaselineRef.current;
      boardPlayRedrawCameraChaseRef.current = {
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
    prevBoardRedrawPlayingRef.current = puzzle2dRedrawPlaying;
  }, [puzzle2dRedrawPlaying, fixture]);

  reactHostPort.useEffect(() => {
    if (!puzzle2dRedrawPlaying) {
      boardPlayRedrawCameraChaseRef.current = null;
      return;
    }
    if (suppressCameraBasisSyncRef.current) {
      return;
    }
    const pane = activePaneIdRef.current;
    const target = triptychCamerasFromFixture(fixture);
    setPuzzle2dPlayPaneCamerasBaseline((baselinePrev) => {
      const prevChase = boardPlayRedrawCameraChaseRef.current ?? baselinePrev;
      const damped = dampCameraStateLinear(prevChase[pane], target[pane], PUZZLE_2D_PLAY_REDRAW_CAMERA_CHASE_BLEND);
      const nextChase: Record<Puzzle2dPlayPaneId, CameraState> = {
        "2d-detail": { ...prevChase["2d-detail"] },
        "2d-overview": { ...prevChase["2d-overview"] },
        "2d-selection": { ...prevChase["2d-selection"] },
      };
      nextChase[pane] = damped;
      boardPlayRedrawCameraChaseRef.current = nextChase;
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
        if (boardPlayNodesRedrawCameraAnimRafRef.current != null) {
          cancelAnimationFrame(boardPlayNodesRedrawCameraAnimRafRef.current);
          boardPlayNodesRedrawCameraAnimRafRef.current = null;
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
    if (boardPlayNodesRedrawCameraAnimRafRef.current != null) {
      cancelAnimationFrame(boardPlayNodesRedrawCameraAnimRafRef.current);
      boardPlayNodesRedrawCameraAnimRafRef.current = null;
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
    if (boardPlayNodesRedrawCameraAnimRafRef.current != null) {
      cancelAnimationFrame(boardPlayNodesRedrawCameraAnimRafRef.current);
      boardPlayNodesRedrawCameraAnimRafRef.current = null;
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
        boardPlayNodesRedrawCameraAnimRafRef.current = null;
        nodesRedrawEaseFromRef.current = null;
        return;
      }
      if (elapsed >= holdEnd) {
        const u = Math.min(1, Math.max(0, (elapsed - holdEnd) / animSpan));
        setPuzzle2dPlayPaneCamerasBaseline(blendTriptychCamerasActivePaneOnly(from, to, u, nodesRedrawEasePaneId));
      }
      boardPlayNodesRedrawCameraAnimRafRef.current = requestAnimationFrame(tickInner);
    };
    boardPlayNodesRedrawCameraAnimRafRef.current = requestAnimationFrame(tickInner);
    return () => {
      if (boardPlayNodesRedrawCameraAnimRafRef.current != null) {
        cancelAnimationFrame(boardPlayNodesRedrawCameraAnimRafRef.current);
        boardPlayNodesRedrawCameraAnimRafRef.current = null;
      }
    };
  }, [nodesRedrawCameraEaseTick]);

  reactHostPort.useEffect(() => {
    if (cameraDisplayOverrideByPane === null) {
      return;
    }
    nodesRedrawEaseGenerationRef.current += 1;
    if (boardPlayNodesRedrawCameraAnimRafRef.current != null) {
      cancelAnimationFrame(boardPlayNodesRedrawCameraAnimRafRef.current);
      boardPlayNodesRedrawCameraAnimRafRef.current = null;
    }
  }, [cameraDisplayOverrideByPane]);

  const redrawPlayingRef = reactHostPort.useRef(false);
  const redrawProgressiveEpochRef = reactHostPort.useRef(0);
  const redrawLoopSnapshotRef = reactHostPort.useRef<Puzzle2dPlayRedrawLoopSnapshot>({
    activePaneId: "2d-overview",
    boardRedrawHandlesAfterNodes: false,
    boardRedrawProgressiveAutoStopMs: 3000,
    boardRedrawProgressiveEnabled: true,
    boardRedrawPlayMaxItersPerFrame: 96,
    camerasByPane: triptychCamerasFromFixture(initialFixture),
    forceLayoutGravity: 0.012,
    forceLayoutIdealEdgeLength: 64,
    forceLayoutRepulsionStrength: 80,
    mode: "force-graph",
    treeLayoutDirection: "downwards",
    treeLayoutLayerSpacing: 120,
    treeLayoutSiblingGap: 28,
  });

  const resetBoardRedrawProgressiveEpoch = reactHostPort.useCallback(() => {
    redrawProgressiveEpochRef.current = typeof performance !== "undefined" ? performance.now() : Date.now();
  }, []);

  redrawLoopSnapshotRef.current = {
    activePaneId,
    boardRedrawHandlesAfterNodes,
    boardRedrawProgressiveAutoStopMs,
    boardRedrawProgressiveEnabled,
    boardRedrawPlayMaxItersPerFrame,
    camerasByPane,
    forceLayoutGravity,
    forceLayoutIdealEdgeLength,
    forceLayoutRepulsionStrength,
    mode: boardRedrawMode,
    treeLayoutDirection,
    treeLayoutLayerSpacing,
    treeLayoutSiblingGap,
  };

  const applyBoardRedrawHandlesOnce = reactHostPort.useCallback(() => {
    patchFixture((prev) => layoutBoardFixtureRedrawHandles(prev));
  }, [patchFixture]);

  const applyBoardRedrawOnce = reactHostPort.useCallback(() => {
    if (boardPlayNodesRedrawCameraAnimRafRef.current != null) {
      cancelAnimationFrame(boardPlayNodesRedrawCameraAnimRafRef.current);
      boardPlayNodesRedrawCameraAnimRafRef.current = null;
    }
    nodesRedrawEaseGenerationRef.current += 1;
    nodesRedrawEaseFromRef.current = {
      "2d-detail": { ...camerasByPane["2d-detail"] },
      "2d-overview": { ...camerasByPane["2d-overview"] },
      "2d-selection": { ...camerasByPane["2d-selection"] },
    };
    const full = Math.max(1, Math.min(5000, Math.round(forceLayoutFullIterations)));
    patchFixture((prev) => {
      const laidOut = layoutBoardFixtureRedrawNodes(
        prev,
        boardPlayRedrawLayoutOpts(
          activePaneId,
          camerasByPane,
          boardRedrawMode,
          full,
          forceLayoutIdealEdgeLength,
          forceLayoutGravity,
          forceLayoutRepulsionStrength,
          treeLayoutLayerSpacing,
          treeLayoutSiblingGap,
          treeLayoutDirection,
          boardRedrawHandlesAfterNodes,
        ),
      );
      return { ...laidOut, camera: { ...prev.camera } };
    });
    setNodesRedrawCameraEaseTick((n) => n + 1);
  }, [
    activePaneId,
    boardRedrawHandlesAfterNodes,
    boardRedrawMode,
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
      if (snap.boardRedrawProgressiveAutoStopMs > 0 && elapsed >= snap.boardRedrawProgressiveAutoStopMs) {
        redrawPlayingRef.current = false;
        setBoardRedrawPlaying(false);
        return;
      }
      let innerIters = 1;
      if (snap.mode === "force-graph") {
        if (snap.boardRedrawProgressiveEnabled) {
          innerIters = boardPlayProgressiveForceIters(elapsed, snap.boardRedrawProgressiveAutoStopMs, snap.boardRedrawPlayMaxItersPerFrame);
        } else {
          innerIters = Math.max(1, Math.min(500, Math.round(snap.boardRedrawPlayMaxItersPerFrame)));
        }
      }
      patchFixture((prev) => {
        if (prev.nodes.length === 0) {
          return prev;
        }
        if (snap.mode === "hierarchical-tree") {
          return layoutBoardFixtureRedrawNodes(
            prev,
            boardPlayRedrawLayoutOpts(
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
              snap.boardRedrawHandlesAfterNodes,
            ),
          );
        }
        const t0 = typeof performance !== "undefined" ? performance.now() : Date.now();
        let cur = prev;
        while (redrawPlayingRef.current && (typeof performance !== "undefined" ? performance.now() : Date.now()) - t0 < BOARD_PLAYRedraw_FRAME_BUDGET_MS) {
          cur = layoutBoardFixtureRedrawNodes(
            cur,
            boardPlayRedrawLayoutOpts(
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
              snap.boardRedrawHandlesAfterNodes,
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
  }, [puzzle2dRedrawPlaying, patchFixture, setBoardRedrawPlaying]);

  const shellValue = reactHostPort.useMemo<Puzzle2dPlayShellValue>(
    () => ({
      activePaneId,
      applyBoardRedrawHandlesOnce,
      applyBoardRedrawOnce,
      applyStructuralDelete,
      boardRedrawHandlesAfterNodes,
      boardRedrawMode,
      boardRedrawPlayMaxItersPerFrame,
      puzzle2dRedrawPlaying,
      boardRedrawProgressiveAutoStopMs,
      boardRedrawProgressiveEnabled,
      puzzle2dSelectionMethod,
      puzzle2dSelectionMode,
      puzzle2dSelectionTargets,
      puzzle2dGridSnapEnabled,
      camerasByPane,
      syncBaselineFromViewportCamera,
      fixture,
      forceLayoutFullIterations,
      forceLayoutGravity,
      forceLayoutIdealEdgeLength,
      forceLayoutRepulsionStrength,
      handleCanvasFixtureDrop,
      patchFixture,
      remapIdInSelections,
      resetBoardRedrawProgressiveEpoch,
      setActivePaneId,
      setBoardRedrawHandlesAfterNodes,
      setBoardRedrawMode,
      setBoardRedrawPlayMaxItersPerFrame,
      setBoardRedrawPlaying,
      setBoardRedrawProgressiveAutoStopMs,
      setBoardRedrawProgressiveEnabled,
      setBoardGridSnapEnabled,
      boardLodModeByPane,
      setBoardLodModeForPane,
      setBoardSelectionMethod,
      setBoardSelectionMode,
      setBoardSelectionTargets,
      setFixture,
      setForceLayoutFullIterations,
      setForceLayoutGravity,
      setForceLayoutIdealEdgeLength,
      setForceLayoutRepulsionStrength,
      setTreeLayoutLayerSpacing,
      setTreeLayoutDirection,
      setTreeLayoutSiblingGap,
      selectionIds,
      setSelectionIds,
      preselection,
      setPreselection,
      hoveredId,
      hoverSourcePane,
      setHoverPane,
      setHoverForPane,
      clearHoverForPane,
      treeLayoutLayerSpacing,
      treeLayoutDirection,
      treeLayoutSiblingGap,
    }),
    [
      activePaneId,
      applyBoardRedrawHandlesOnce,
      applyBoardRedrawOnce,
      applyStructuralDelete,
      boardRedrawHandlesAfterNodes,
      boardRedrawMode,
      boardRedrawPlayMaxItersPerFrame,
      puzzle2dRedrawPlaying,
      boardRedrawProgressiveAutoStopMs,
      boardRedrawProgressiveEnabled,
      puzzle2dSelectionMethod,
      puzzle2dSelectionMode,
      puzzle2dSelectionTargets,
      puzzle2dGridSnapEnabled,
      boardLodModeByPane,
      setBoardLodModeForPane,
      camerasByPane,
      syncBaselineFromViewportCamera,
      fixture,
      forceLayoutFullIterations,
      forceLayoutGravity,
      forceLayoutIdealEdgeLength,
      forceLayoutRepulsionStrength,
      handleCanvasFixtureDrop,
      patchFixture,
      remapIdInSelections,
      resetBoardRedrawProgressiveEpoch,
      selectionIds,
      preselection,
      hoveredId,
      hoverSourcePane,
      setHoverPane,
      setHoverForPane,
      clearHoverForPane,
      treeLayoutLayerSpacing,
      treeLayoutDirection,
      treeLayoutSiblingGap,
    ],
  );

  // #region 🔖ToolbarHostBridge
  const puzzle2dPlayToolbarHostRef = reactHostPort.useRef({
    activePaneId: "2d-overview" as Puzzle2dPlayPaneId,
    applyBoardRedrawHandlesOnce: () => {},
    camerasByPane: triptychCamerasFromFixture(initialFixture),
    patchFixture: (_updater: (prev: BoardFixtureV1) => BoardFixtureV1) => {},
    setBoardGridSnapEnabled: (_value: boolean | ((prev: boolean) => boolean)) => {},
    setBoardRedrawPlaying: (_value: boolean | ((prev: boolean) => boolean)) => {},
    setBoardSelectionMethod: (_value: BoardSelectionMethod) => {},
    setBoardSelectionMode: (_value: BoardSelectionMode) => {},
    setBoardSelectionTargets: (_value: BoardSelectionTargets | ((prev: BoardSelectionTargets) => BoardSelectionTargets)) => {},
    setSelectionIds: (_ids: readonly string[]) => {},
  });
  puzzle2dPlayToolbarHostRef.current = {
    activePaneId,
    applyBoardRedrawHandlesOnce,
    camerasByPane,
    patchFixture,
    setBoardGridSnapEnabled,
    setBoardRedrawPlaying,
    setBoardSelectionMethod,
    setBoardSelectionMode,
    setBoardSelectionTargets,
    setSelectionIds,
  };

  reactHostPort.useEffect(() => {
    if (!puzzle2dShellController) {
      return;
    }
    const bridge: Puzzle2dPlayHostBridge = {
      getToolbarState: () => ({
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
            h.setBoardSelectionMethod((args as { method: BoardSelectionMethod }).method);
            break;
          case "setSelectionMode":
            h.setBoardSelectionMode((args as { mode: BoardSelectionMode }).mode);
            break;
          case "toggleSelectionTarget": {
            const { kind } = args as { kind: "edges" | "handles" | "nodes" };
            h.setBoardSelectionTargets((prev) => ({ ...prev, [kind]: !prev[kind] }));
            break;
          }
          case "clearSelection":
            h.setSelectionIds([]);
            break;
          case "toggleGridSnap":
            h.setBoardGridSnapEnabled((prev) => !prev);
            break;
          case "appendCircle": {
            const camera = h.camerasByPane[h.activePaneId];
            const id = newBoardAuthoringId("node");
            const handleId = `${id}.h0`;
            const node: BoardFixtureCircleNodeV1 = {
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
            const id = newBoardAuthoringId("node");
            const handleId = `${id}.h0`;
            const d = PUZZLE_2D_PLAY_DEFAULT_NODE_SIZE_PX;
            const node: BoardFixtureRectangleNodeV1 = {
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
            h.setBoardRedrawPlaying((prev) => !prev);
            break;
          case "redrawHandlesOnce":
            h.applyBoardRedrawHandlesOnce();
            break;
          default:
            break;
        }
      },
    };
    puzzle2dShellController.setHostBridge(bridge);
    return () => puzzle2dShellController.setHostBridge(null);
  }, [applyBoardRedrawHandlesOnce, puzzle2dGridSnapEnabled, puzzle2dRedrawPlaying, puzzle2dSelectionMethod, puzzle2dSelectionMode, puzzle2dSelectionTargets, puzzle2dShellController]);
  // #endregion 🔖ToolbarHostBridge

  const shellValueRef = reactHostPort.useRef(shellValue);
  shellValueRef.current = shellValue;
  const puzzle2dPlaySelectionKey = reactHostPort.useMemo(() => [...selectionIds].sort().join("\0"), [selectionIds]);
  const puzzle2dPlayFixtureKey = reactHostPort.useMemo(() => `${shellValue.fixture.nodes.map((node) => node.id).join(",")}\u0001${shellValue.fixture.edges.map((edge) => edge.id).join(",")}`, [shellValue.fixture]);
  const puzzle2dPlayLibraryTab = reactHostPort.useMemo(() => new Puzzle2dPlayLibraryPanelDefinition().resolveTab(), []);
  const puzzle2dPlayHierarchyTab = reactHostPort.useMemo(
    () => new Puzzle2dPlayHierarchyPanelDefinition(() => buildPuzzle2dPlayHierarchySections(shellValueRef.current.fixture, [...shellValueRef.current.selectionIds], (id) => shellValueRef.current.setSelectionIds([id]))).resolveTab(),
    [puzzle2dPlaySelectionKey, puzzle2dPlayFixtureKey],
  );
  const puzzle2dPlaySettingsTab = reactHostPort.useMemo(() => new Puzzle2dPlaySettingsPanelDefinition().resolveTab(), []);
  const puzzle2dPlayInspectorTab = reactHostPort.useMemo(() => new Puzzle2dPlayInspectorPanelDefinition(() => buildPuzzle2dPlayInspectorSections(shellValueRef.current)).resolveTab(), [puzzle2dPlaySelectionKey]);
  const augmentPanelTabs = reactHostPort.useMemo(
    () => ({
      workbench: [puzzle2dPlayHierarchyTab, puzzle2dPlayLibraryTab],
      details: [puzzle2dPlayInspectorTab, puzzle2dPlaySettingsTab],
    }),
    [puzzle2dPlayHierarchyTab, puzzle2dPlayInspectorTab, puzzle2dPlaySettingsTab, puzzle2dPlayLibraryTab],
  );

  return (
    <Puzzle2dPlayShellContext.Provider value={shellValue}>
      <Puzzle2dPlayLodRuntimeContext.Provider value={setBoardEffectiveLodForPane}>
        <PlaygroundView runtime={boardRuntime} defaultAppId={PUZZLE_2D_PLAY_APP_ID} augmentPanelTabs={augmentPanelTabs} initialPanelVisibility={{ leftSidePanel: true, rightSidePanel: true }} onActiveWindowChange={onPuzzle2dPlayActiveWindowChange} />
      </Puzzle2dPlayLodRuntimeContext.Provider>
    </Puzzle2dPlayShellContext.Provider>
  );
}

function Puzzle2dPlayChrome({ runtime }: { readonly runtime: Platform }): ReactElement {
  return <Puzzle2dPlayInner boardRuntime={runtime} />;
}

/** @emoji 🚀 Mounts board play chrome for a {@link Playground}. */
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
          status: [{ id: "st", text: "State: idle" }],
        },
        bus,
      );
      expect(spec?.options?.[0]?.label).toBe("Confirm");
      expect(spec?.input?.value).toBe("x");
      expect(spec?.status?.[0]?.content).toBe("State: idle");
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

  describe("board play cameras", () => {
    it("imports board camera zoom limits used by host clamping", async () => {
      const { BOARD_CAMERA_ZOOM_MIN, BOARD_CAMERA_ZOOM_MAX } = await import("@puzzle/2d/react");
      expect(BOARD_CAMERA_ZOOM_MIN).toBeGreaterThan(0);
      expect(BOARD_CAMERA_ZOOM_MAX).toBeGreaterThan(BOARD_CAMERA_ZOOM_MIN);
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

    it("renders puzzle2d nodes through platform surface bindings", async () => {
      const { renderToStaticMarkup } = await import("react-dom/server");
      const { buildPuzzle2dWindowBody } = await import("@framework/playground/core");
      const surfaceId = "playground.test/puzzle2d";
      function TestBoardHost(): React.ReactElement {
        return <div data-host="puzzle2d">board canvas</div>;
      }
      registerSurfaceBinding(surfaceId, TestBoardHost);
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
