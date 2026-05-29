// #region 🧲Header
/** @emoji 🛝 Playground shell renderer: {@link PlaygroundView}, tree panels, and surface hosts (no puzzle play imports). */
// #endregion 🧲Header

// #region 🔌Adapters
import {
  App,
  Footer,
  Layout,
  Mode,
  Navbar,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  Slider,
  Toggle,
  ToolbarDivider,
  ToolbarItem,
  ToolbarZone,
  Ui,
  cn,
  staticTreePanelDefinition,
  useMediaQuery,
  type EngagementSpec,
  type FooterItem,
  type ModeWindowDescriptor,
  type NavbarItem,
  type WindowLayoutNode as ShellWindowLayoutNode,
  type SidePanelTabConfig,
  type SidePanelTabDefinition,
  type TreeDataItem,
  type TreeDataSection,
  type TreePanelConfig,
  type TreePanelDefinition,
  type TreePanelSource,
  reactHostPort,
} from "@ui/react";
import { clsx, type ClassValue } from "clsx";
import type { LucideIcon } from "lucide-react";
import { ArrowRightLeft, Filter, Folder, FolderOpen, Hand, History, Info, Lasso, LayoutGrid, MoreHorizontal, MousePointer2, Plus, Save, Search, Settings2 } from "lucide-react";
import * as React from "react";
import { createRoot, type Root } from "react-dom/client";
import { twMerge } from "tailwind-merge";
import {
  APP_TOOL_CATEGORY_ORDER,
  CommandBus,
  ProductRuntime,
  WindowKindRuntime,
  getSidePanelBodyFactory,
  getWindowBodyFactory,
  type AppToolCategory,
  type AppTools,
  type Playground,
  type ResolvedAppState,
  type SidePanelBodyViewContext,
  type SideTabSpec,
  type UiBoardHostSurfaceNode,
  type UiNode,
  type UiScene3DHostSurfaceNode,
  type UiTableHostSurfaceNode,
  type WindowBodyViewContext,
  type WindowLayout,
  type WindowMeasure,
} from "@framework/playground";
// #endregion 🔌Adapters

export type {
  AppRuntime,
  AppTools,
  CommandBus,
  Controller,
  ModeRuntime,
  FooterItem as PlaygroundDeclarativeFooterItem,
  ProductRuntime,
  ResolvedAppState,
  SidePanelBodyViewContext,
  SideTabSpec,
  ToolItem,
  UiNode,
  WindowBodyViewContext,
  WindowKindRuntime,
  WindowLayout,
} from "@framework/playground";

export {
  APP_TOOL_CATEGORY_ORDER,
  AppRuntime,
  CommandBus,
  ModeRuntime,
  PlaygroundController,
  ProductRuntime,
  WindowKindRuntime,
  buildScene3dWindowBody,
  createDefaultLayout,
  createStackLayout,
  createWindowLayout,
  getSidePanelBodyFactory,
  getWindowBodyFactory,
  registerSidePanelBody,
  registerWindowBody,
  resolveAppState,
  playgroundTreePanelRootItems,
} from "@framework/playground";

function cnPlay(...inputs: ClassValue[]): string {
  return twMerge(clsx(inputs));
}

//#region 🔖TreePanels
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

//#region 🔖LayoutGolden
function convertFrameworkLayoutNodeToShellLayout(node: WindowLayout["root"]): ShellWindowLayoutNode {
  if (node.kind === "stack") {
    return {
      kind: "stack",
      size: node.size,
      children: node.children.map((child) => ({ kind: "window", id: child.windowKindId, title: child.title })),
    };
  }
  return {
    kind: node.kind,
    size: node.size,
    children: node.children.map((child) => convertFrameworkLayoutNodeToShellLayout(child)),
  };
}

function convertFrameworkLayoutToShellLayout(layout: WindowLayout): ShellWindowLayoutNode {
  return convertFrameworkLayoutNodeToShellLayout(layout.root);
}

function findDefaultActiveWindowKindId(layout: WindowLayout | undefined, windowKinds: readonly { readonly id: string }[]): string | null {
  const allowed = new Set(windowKinds.map((windowKind) => windowKind.id));
  const visit = (node: WindowLayout["root"]): string | null => {
    if (node.kind === "stack") {
      for (const child of node.children) {
        if (allowed.has(child.windowKindId)) return child.windowKindId;
      }
      return null;
    }
    for (const child of node.children) {
      const match = visit(child);
      if (match) return match;
    }
    return null;
  };
  if (layout) {
    const match = visit(layout.root);
    if (match) return match;
  }
  return windowKinds[0]?.id ?? null;
}
//#endregion 🔖LayoutGolden

//#region 🔖UiRenderer
type Scene3DSurfaceHost = React.ComponentType<{ readonly node: UiScene3DHostSurfaceNode }>;
type BoardSurfaceHost = React.ComponentType<{ readonly node: UiBoardHostSurfaceNode }>;
type TableSurfaceHost = React.ComponentType<{ readonly node: UiTableHostSurfaceNode }>;

const scene3dSurfaceHosts = new Map<string, Scene3DSurfaceHost>();
const boardSurfaceHosts = new Map<string, BoardSurfaceHost>();
const tableSurfaceHosts = new Map<string, TableSurfaceHost>();

/** @emoji 🧭 Binds a `surfaceId` from {@link UiScene3DHostSurfaceNode} to a host React canvas implementation. */
export function registerUiScene3DSurfaceHost(surfaceId: string, Component: Scene3DSurfaceHost): void {
  scene3dSurfaceHosts.set(surfaceId, Component);
}

/** @emoji 📋 Binds `surfaceId` from {@link UiBoardHostSurfaceNode} to a host board canvas. */
export function registerUiBoardSurfaceHost(surfaceId: string, Component: BoardSurfaceHost): void {
  boardSurfaceHosts.set(surfaceId, Component);
}

/** @emoji 📊 Binds `surfaceId` from {@link UiTableHostSurfaceNode} to a host table body. */
export function registerUiTableSurfaceHost(surfaceId: string, Component: TableSurfaceHost): void {
  tableSurfaceHosts.set(surfaceId, Component);
}

function stackClass(spec: { direction: "horizontal" | "vertical"; gap?: string; padding?: string }): string {
  const dir = spec.direction === "horizontal" ? "flex-row" : "flex-col";
  const gap = spec.gap === "none" ? "gap-0" : spec.gap === "tight" ? "gap-1" : spec.gap === "relaxed" ? "gap-4" : "gap-2";
  const pad = spec.padding === "none" ? "p-0" : "p-2";
  return cnPlay("flex", dir, gap, pad, spec.direction === "vertical" ? "min-h-0 min-w-0" : "min-w-0");
}

export function UiRenderer({ node, commandBus }: { readonly node: UiNode; readonly commandBus: CommandBus }): React.ReactElement {
  switch (node.type) {
    case "stack":
      return (
        <div className={cnPlay(stackClass(node), node.direction === "vertical" && node.children.some((c) => c.type === "scene3d" || c.type === "board") && "relative min-h-0 flex-1")}>
          {node.children.map((child, index) => (
            <UiRenderer key={index} node={child} commandBus={commandBus} />
          ))}
        </div>
      );
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
    case "scene3d": {
      const Host = scene3dSurfaceHosts.get(node.surfaceId);
      if (!Host) {
        return <div className="flex flex-1 items-center justify-center p-4 text-xs text-muted-foreground">Unsupported scene3d surface &quot;{node.surfaceId}&quot;</div>;
      }
      return (
        <div className="absolute inset-0 min-h-0 min-w-0">
          <Host node={node} />
        </div>
      );
    }
    case "board": {
      const Host = boardSurfaceHosts.get(node.surfaceId);
      if (!Host) {
        return <div className="flex flex-1 items-center justify-center p-4 text-xs text-muted-foreground">Unsupported board surface &quot;{node.surfaceId}&quot;</div>;
      }
      return (
        <div className="absolute inset-0 min-h-0 min-w-0">
          <Host node={node} />
        </div>
      );
    }
    case "table": {
      const Host = tableSurfaceHosts.get(node.surfaceId);
      if (!Host) {
        return <div className="flex flex-1 items-center justify-center p-4 text-xs text-muted-foreground">Unsupported table surface &quot;{node.surfaceId}&quot;</div>;
      }
      return (
        <div className="relative min-h-0 min-w-0 flex-1 overflow-auto">
          <Host node={node} />
        </div>
      );
    }
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
  measures?: React.ReactNode;
  engagement?: EngagementSpec;
}

function windowMeasureShell(measureId: string, label: string | undefined, children: React.ReactNode): React.ReactNode {
  return (
    <div data-slot="window-measure-float" data-measure-id={measureId} className="border-element/80 bg-window/90 max-w-[11rem] min-w-0 rounded-md border px-single py-half shadow-md backdrop-blur-sm">
      {label ? <span className="text-muted-foreground mb-half block max-w-full truncate text-[10px] font-semibold uppercase tracking-wide">{label}</span> : null}
      <div className="min-w-0 w-full">{children}</div>
    </div>
  );
}

function windowMeasuresToGolden(measures: readonly WindowMeasure[], bus: CommandBus): React.ReactNode {
  if (!measures.length) return undefined;
  return (
    <div data-slot="window-measures-stack-inner" className="pointer-events-auto flex flex-col items-end gap-half p-single">
      {measures.map((measure) => {
        if (measure.kind === "select") {
          return (
            <React.Fragment key={measure.id}>
              {windowMeasureShell(
                measure.id,
                measure.label,
                <Select
                  id={measure.id}
                  value={measure.value}
                  onValueChange={(value) =>
                    bus.dispatch(measure.onChange.controllerId, measure.onChange.command, {
                      ...(measure.onChange.args as object | undefined),
                      value,
                    })
                  }
                >
                  <SelectTrigger id={measure.id} className="h-medium w-full min-w-0 max-w-[9.5rem]" size="sm">
                    <SelectValue placeholder={measure.label} />
                  </SelectTrigger>
                  <SelectContent>
                    {measure.items.map((item) => (
                      <SelectItem key={item.id} value={item.value}>
                        {item.label}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>,
              )}
            </React.Fragment>
          );
        }
        if (measure.kind === "slider") {
          return (
            <React.Fragment key={measure.id}>
              {windowMeasureShell(
                measure.id,
                measure.label,
                <Slider
                  id={measure.id}
                  value={[measure.value]}
                  min={measure.min}
                  max={measure.max}
                  step={measure.step}
                  onValueChange={(vals) =>
                    bus.dispatch(measure.onChange.controllerId, measure.onChange.command, {
                      ...(measure.onChange.args as object | undefined),
                      value: vals[0] ?? measure.min,
                    })
                  }
                />,
              )}
            </React.Fragment>
          );
        }
        if (measure.kind === "toggle") {
          return (
            <React.Fragment key={measure.id}>
              {windowMeasureShell(
                measure.id,
                measure.label,
                <Toggle
                  id={measure.id}
                  pressed={measure.pressed}
                  text={measure.text}
                  onPressedChange={(pressed) =>
                    bus.dispatch(measure.onChange.controllerId, measure.onChange.command, {
                      ...(measure.onChange.args as object | undefined),
                      pressed,
                    })
                  }
                />,
              )}
            </React.Fragment>
          );
        }
        return null;
      })}
    </div>
  );
}

export function windowKindsToGolden(windowKinds: readonly WindowKindRuntime[], bus: CommandBus): UIWindowKindDefinition[] {
  return windowKinds.map((wk) => ({
    id: wk.id,
    label: wk.label,
    component: getDeclarativeWindowBodyComponent(wk.id, wk.bodyKey),
    measures: windowMeasuresToGolden(wk.measures, bus),
  }));
}

/** @emoji 📑 Converts playground side tabs into enforced tree panel configs (sections with items). */
export function sideTabsToPlaygroundPanelTabs(tabs: readonly SideTabSpec[], bus: CommandBus): SidePanelTabConfig[] {
  void bus;
  return tabs.map((tab, orderIndex) => {
    const declarativeFactory = getSidePanelBodyFactory(tab.bodyKey);
    const Body = declarativeFactory ? getDeclarativeSidePanelBodyComponent(tab.id, tab.bodyKey) : () => <div className="p-2 text-xs">Missing panel {tab.bodyKey}</div>;
    return resolveSidePanelTabSource({
      id: tab.id,
      icon: shellTabIconComponent(tab.iconId),
      order: tab.order ?? orderIndex,
      tree: staticTreePanelDefinition({
        sections: [
          {
            id: `${tab.id}.host`,
            label: tab.id,
            defaultOpen: true,
            items: [{ id: `${tab.id}.body`, label: tab.id, description: <Body /> }],
          },
        ],
      }).resolveTab().tree,
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

//#region 🔖ShellModeCanvas
const ShellModeCanvas: React.FC<{
  windowKinds: UIWindowKindDefinition[];
  defaultLayout: WindowLayout;
  activeWindowId: string | null;
  onActiveWindowChange?: (windowId: string) => void;
}> = ({ windowKinds, defaultLayout, activeWindowId, onActiveWindowChange }) => {
  const windows = reactHostPort.useMemo<ModeWindowDescriptor[]>(
    () =>
      windowKinds.map((windowKind) => {
        const WindowComponent = windowKind.component;
        return {
          id: windowKind.id,
          title: windowKind.label,
          showControls: true,
          measures: windowKind.measures,
          engagement: windowKind.engagement,
          children: (
            <div className="flex min-h-0 min-w-0 flex-1 flex-col">
              <WindowComponent />
            </div>
          ),
        };
      }),
    [windowKinds],
  );
  const shellLayout = reactHostPort.useMemo(() => convertFrameworkLayoutToShellLayout(defaultLayout), [defaultLayout]);

  return <Mode windows={windows} layout={shellLayout} activeWindowId={activeWindowId} onActiveWindowChange={onActiveWindowChange} className="h-full w-full" />;
};
//#endregion 🔖ShellModeCanvas

//#region 🔖Toolbar
type UIToolbarItem = {
  id: string;
  kind?: "separator" | "toggle";
  icon?: React.ReactNode;
  text?: string;
  label?: string;
  title?: string;
  order?: number;
  pressed?: boolean;
  disabled?: boolean;
  onPressedChange?: (pressed: boolean) => void;
  onClick?: () => void;
};

function sortToolbarItems(items: readonly UIToolbarItem[]): UIToolbarItem[] {
  return [...items].sort((left, right) => (left.order ?? 0) - (right.order ?? 0));
}

function hasToolbarCategoryItems(items: readonly UIToolbarItem[] | undefined): boolean {
  return Boolean(items?.some((item) => item.kind !== "separator"));
}

function listPopulatedToolbarCategories(tools: Partial<Record<AppToolCategory, UIToolbarItem[]>>): AppToolCategory[] {
  return APP_TOOL_CATEGORY_ORDER.filter((category) => hasToolbarCategoryItems(tools[category]));
}

function resolvePlaygroundToolCategoryIcon(category: AppToolCategory): React.ReactNode {
  switch (category) {
    case "hand":
      return <Hand className="size-tiny" aria-hidden />;
    case "selection":
      return <MousePointer2 className="size-tiny" aria-hidden />;
    case "lasso":
      return <Lasso className="size-tiny" aria-hidden />;
    case "filter":
      return <Filter className="size-tiny" aria-hidden />;
    case "open":
      return <FolderOpen className="size-tiny" aria-hidden />;
    case "save":
      return <Save className="size-tiny" aria-hidden />;
    case "transform":
      return <ArrowRightLeft className="size-tiny" aria-hidden />;
    case "create":
      return <Plus className="size-tiny" aria-hidden />;
    case "view":
      return <LayoutGrid className="size-tiny" aria-hidden />;
    case "actions":
      return <MoreHorizontal className="size-tiny" aria-hidden />;
    case "settings":
      return <Settings2 className="size-tiny" aria-hidden />;
    case "history":
      return <History className="size-tiny" aria-hidden />;
    default:
      return <Search className="size-tiny" aria-hidden />;
  }
}

function resolvePlaygroundToolCategoryLabel(category: AppToolCategory): string {
  if (category === "history") return "History";
  return category.charAt(0).toUpperCase() + category.slice(1);
}

function declareToolsToViewTools(tools: AppTools | undefined, bus: CommandBus): Partial<Record<AppToolCategory, UIToolbarItem[]>> | undefined {
  if (!tools) return undefined;
  const merged: Partial<Record<AppToolCategory, UIToolbarItem[]>> = {};
  for (const category of APP_TOOL_CATEGORY_ORDER) {
    const list = tools[category];
    if (!list?.length) continue;
    merged[category] = list.map((item) => {
      if (item.kind === "separator") return { id: item.id, kind: "separator", order: item.order };
      if (item.kind === "toggle") {
        return {
          id: item.id,
          kind: "toggle",
          text: item.text,
          label: item.label,
          title: item.title,
          order: item.order,
          pressed: item.pressed,
          disabled: item.disabled,
          onPressedChange: (pressed: boolean) => {
            if (item.disabled) return;
            if (item.controllerId && item.command) bus.dispatch(item.controllerId, item.command, { ...(item.args as object | undefined), pressed });
          },
        };
      }
      return {
        id: item.id,
        text: item.text,
        label: item.label,
        title: item.title,
        order: item.order,
        disabled: item.disabled,
        onClick: item.disabled || !item.controllerId || !item.command ? undefined : () => bus.dispatch(item.controllerId!, item.command!, item.args),
      };
    });
  }
  return Object.keys(merged).length > 0 ? merged : undefined;
}

const PlaygroundToolbarItems: React.FC<{ items: readonly UIToolbarItem[] }> = ({ items }) => {
  const sorted = reactHostPort.useMemo(() => sortToolbarItems(items), [items]);
  return (
    <>
      {sorted.map((item) => {
        const tooltip = item.title ?? item.label ?? item.text;
        if (item.kind === "separator") {
          return <ToolbarDivider key={item.id} id={item.id} />;
        }
        if (item.kind === "toggle") {
          return (
            <ToolbarItem key={item.id}>
              <Toggle id={item.id} title={tooltip} text={item.text ?? item.label} pressed={item.pressed ?? false} disabled={item.disabled} onPressedChange={(pressed) => item.onPressedChange?.(pressed)} />
            </ToolbarItem>
          );
        }
        return (
          <ToolbarItem key={item.id}>
            <button
              type="button"
              id={item.id}
              title={tooltip}
              disabled={item.disabled}
              onClick={item.onClick}
              className="flex cursor-pointer items-center gap-single rounded px-single py-tiny text-sm hover:bg-hover-panel disabled:cursor-not-allowed disabled:opacity-50"
            >
              {item.icon}
              {(item.text ?? item.label) ? <span>{item.text ?? item.label}</span> : null}
            </button>
          </ToolbarItem>
        );
      })}
    </>
  );
};

/** @emoji 🧰 Playground toolbar: category toggles with one active category exposing its tools. */
const PlaygroundToolbar: React.FC<{ tools: Partial<Record<AppToolCategory, UIToolbarItem[]>> }> = ({ tools }) => {
  const populatedCategories = reactHostPort.useMemo(() => listPopulatedToolbarCategories(tools), [tools]);
  const [activeCategory, setActiveCategory] = reactHostPort.useState<AppToolCategory | null>(null);

  reactHostPort.useEffect(() => {
    if (populatedCategories.length === 0) {
      setActiveCategory(null);
      return;
    }
    setActiveCategory((previousValue) => {
      if (previousValue && populatedCategories.includes(previousValue)) return previousValue;
      return populatedCategories.find((category) => category !== "history" && category !== "hand") ?? populatedCategories[0] ?? null;
    });
  }, [populatedCategories]);

  if (populatedCategories.length === 0) return null;

  const activeItems = activeCategory ? (tools[activeCategory] ?? []) : [];
  const showCategoryNav = populatedCategories.length > 1;

  return (
    <div className="flex min-w-0 flex-1 items-center justify-center px-single">
      <div role="toolbar" id="playground.toolbar" className={cn("flex max-w-full items-center gap-single", showCategoryNav && "relative h-[var(--toolbar-item-height)] w-full max-w-[min(100%,48rem)]")}>
        {showCategoryNav ? (
          <>
            <ToolbarZone id="playground.toolbar.zone.categories" className="shrink-0">
              {populatedCategories.map((category) => (
                <Toggle
                  key={category}
                  kind="single"
                  id={`playground.toolbar.group.${category}`}
                  pressed={activeCategory === category}
                  onPressedChange={() => setActiveCategory((previousValue) => (previousValue === category ? null : category))}
                  icon={resolvePlaygroundToolCategoryIcon(category)}
                  text={resolvePlaygroundToolCategoryLabel(category)}
                />
              ))}
            </ToolbarZone>
            {activeCategory && hasToolbarCategoryItems(activeItems) ? (
              <ToolbarZone id="playground.toolbar.zone.tools" className="min-h-[var(--toolbar-item-height)] min-w-0 h-auto flex-1 flex-wrap overflow-visible p-half">
                <PlaygroundToolbarItems items={activeItems} />
              </ToolbarZone>
            ) : null}
          </>
        ) : (
          <ToolbarZone className="max-w-full flex-wrap h-auto min-h-[var(--toolbar-item-height)] overflow-visible p-half">
            <PlaygroundToolbarItems items={tools[populatedCategories[0]!] ?? []} />
          </ToolbarZone>
        )}
      </div>
    </div>
  );
};
//#endregion 🔖Toolbar

//#region 🔖PlaygroundView
export interface PlaygroundPanelVisibility {
  leftSidePanel: boolean;
  rightSidePanel: boolean;
}

export interface PlaygroundContextValue {
  runtime: ProductRuntime;
  activeAppId: string;
  activeApp: ResolvedAppState;
  activeModeId: string | null;
}

export const PlaygroundContext = reactHostPort.createContext<PlaygroundContextValue | undefined>(undefined);

/** @emoji 🪝 Returns the active {@link ProductRuntime} from the nearest {@link PlaygroundView}. */
export function useApp(): PlaygroundContextValue {
  const ctx = reactHostPort.useContext(PlaygroundContext);
  if (!ctx) throw new Error("useApp must be used within PlaygroundView");
  return ctx;
}

export interface PlaygroundViewProps {
  readonly runtime: ProductRuntime;
  readonly defaultAppId?: string;
  readonly className?: string;
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
export const PlaygroundView: React.FC<PlaygroundViewProps> = ({ runtime, defaultAppId, className, mobile, mobileQuery = "(max-width: 767px)", initialPanelVisibility, slotToolbar, extraFooterItems, augmentPanelTabs, onActiveWindowChange }) => {
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
  const [panelVisibility, setPanelVisibility] = reactHostPort.useState<PlaygroundPanelVisibility>(() => ({
    leftSidePanel: initialPanelVisibility?.leftSidePanel ?? false,
    rightSidePanel: initialPanelVisibility?.rightSidePanel ?? false,
  }));
  const detectedMobile = useMediaQuery(mobileQuery);
  const resolvedMobile = mobile ?? detectedMobile ?? runtime.mobile;

  const activeAppBase = runtime.getActiveApp();
  if (!activeAppBase) return null;

  const activeModeId = activeAppBase.getActiveModeId();
  const activeApp = activeAppBase.resolve(activeModeId);
  const bus = runtime.commandBus;

  const workbenchTabs = mergePanelTabs(sideTabsToPlaygroundPanelTabs(activeApp.leftTabs, bus), augmentPanelTabs?.workbench);
  const detailsTabs = mergePanelTabs(undefined, augmentPanelTabs?.details);

  const mergedTools = declareToolsToViewTools(activeApp.tools, bus);
  const hasToolbarTools = mergedTools && APP_TOOL_CATEGORY_ORDER.some((c) => mergedTools[c]?.some((i) => i.kind !== "separator"));

  const [activeWindowKindId, setActiveWindowKindId] = reactHostPort.useState<string | null>(() => findDefaultActiveWindowKindId(activeApp.defaultLayout, activeApp.windowKinds));

  reactHostPort.useEffect(() => {
    setActiveWindowKindId((previous) => {
      if (previous && activeApp.windowKinds.some((wk) => wk.id === previous)) return previous;
      return findDefaultActiveWindowKindId(activeApp.defaultLayout, activeApp.windowKinds);
    });
  }, [activeApp.defaultLayout, activeApp.windowKinds]);

  const goldenWindowKinds = reactHostPort.useMemo(() => windowKindsToGolden(activeApp.windowKinds, bus), [activeApp.windowKinds, bus]);

  const footerItems: FooterItem[] = [
    ...(activeApp.footerItems.map((item) => ({
      id: item.id,
      text: item.text,
      order: item.order,
      className: item.className,
      disabled: item.disabled,
      onClick: item.controllerId && item.command ? () => bus.dispatch(item.controllerId!, item.command!, item.args) : undefined,
    })) as FooterItem[]),
    ...(extraFooterItems ?? []),
  ].sort((a, b) => (a.order ?? 0) - (b.order ?? 0));

  const workbenchIcon = workbenchTabs[0]?.icon ? React.createElement(workbenchTabs[0].icon, { size: 16 }) : <Folder size={16} />;
  const detailsIcon = detailsTabs[0]?.icon ? React.createElement(detailsTabs[0].icon, { size: 16 }) : <Info size={16} />;

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

  const toolbarElement = slotToolbar ?? (hasToolbarTools && mergedTools ? <PlaygroundToolbar tools={mergedTools} /> : undefined);

  return (
    <PlaygroundContext.Provider
      value={{
        runtime,
        activeAppId: runtime.activeAppId,
        activeApp,
        activeModeId,
      }}
    >
      <Layout
        className={className}
        mobile={resolvedMobile}
        navbar={<Navbar items={navbarItems} />}
        footer={footerItems.length > 0 ? <Footer items={footerItems} /> : undefined}
        toolbar={toolbarElement}
        leftSidePanel={
          !resolvedMobile && workbenchTabs.length > 0
            ? {
                position: "left",
                visible: panelVisibility.leftSidePanel,
                size: leftPanelSize,
                onSizeChange: setLeftPanelSize,
                tabs: workbenchTabs,
              }
            : undefined
        }
        rightSidePanel={
          !resolvedMobile && detailsTabs.length > 0
            ? {
                position: "right",
                visible: panelVisibility.rightSidePanel,
                size: rightPanelSize,
                onSizeChange: setRightPanelSize,
                tabs: detailsTabs,
              }
            : undefined
        }
        canvas={
          <Ui
            apps={[
              {
                id: activeApp.id,
                label: activeApp.label,
                children: (
                  <App
                    modes={activeAppBase.modes.length > 0 ? activeAppBase.modes.map((mode) => ({ id: mode.id, label: mode.label, children: null })) : [{ id: activeApp.id, label: activeApp.label, children: null }]}
                    activeModeId={activeModeId ?? activeAppBase.modes[0]?.id ?? activeApp.id}
                    onActiveModeChange={(modeId) => {
                      activeAppBase.setActiveModeId(modeId);
                      runtime.notify();
                    }}
                    chrome={false}
                  >
                    <ShellModeCanvas
                      windowKinds={goldenWindowKinds}
                      defaultLayout={activeApp.defaultLayout}
                      activeWindowId={activeWindowKindId}
                      onActiveWindowChange={(windowKindId) => {
                        setActiveWindowKindId(windowKindId);
                        onActiveWindowChange?.(windowKindId);
                      }}
                    />
                  </App>
                ),
              },
            ]}
            activeAppId={runtime.activeAppId}
            chrome={false}
          />
        }
      />
    </PlaygroundContext.Provider>
  );
};
//#endregion 🔖PlaygroundView

//#region 🔖Mount
type ElementsDomRoot = HTMLElement & { __elementsPlaygroundRoot?: Root };

/** @emoji 🚀 Mounts an arbitrary React tree into `#root` (or `rootId`). */
export function mountPlaygroundApp(element: React.ReactElement, rootId = "root"): void {
  if (typeof document === "undefined") return;
  const rootElement = document.getElementById(rootId) as ElementsDomRoot | null;
  if (!rootElement) throw new Error(`React root #${rootId} missing.`);
  rootElement.__elementsPlaygroundRoot ??= createRoot(rootElement);
  rootElement.__elementsPlaygroundRoot.render(element);
}

/** @emoji 🚀 Alias for {@link mountPlaygroundApp}. */
export const mountReactApp = mountPlaygroundApp;
//#endregion 🔖Mount

//#region 🧪Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("PlaygroundToolbar categories", () => {
    it("lists populated categories and omits separator-only groups", () => {
      expect(
        listPopulatedToolbarCategories({
          save: [{ id: "save.selected", label: "Selected" }],
          filter: [{ id: "sep", kind: "separator" }],
        }),
      ).toEqual(["save"]);
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
  });
}
//#endregion 🧪Tests


//#region 🔖Boot
import type { Playground } from "@framework/playground";

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
