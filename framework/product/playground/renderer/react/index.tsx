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
    NAVBAR_NO_EXAMPLE_ID,
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
    renderControlIcon,
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
    registerIcon,
    registerTabIcon,
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
import type { AppExampleContribution, AppExampleOption, AppRendererContribution, UiSurfaceHostNode } from "@semio-tech/framework-platform-core";
import {
    AppRuntime,
    CommandBus,
    Expertise,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
    FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID,
    FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
    FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
    FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID,
    FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL,
    ModeRuntime,
    PLAYGROUND_NO_EXAMPLE_ID,
    PRODUCT_SHELL_DEFAULT_PANEL_VISIBILITY,
    Platform,
    PlaygroundController,
    WindowKindRuntime,
    buildCadWindowBody,
    buildPuzzle3dWindowBody,
    collectUiTreeItemDragData,
    createDefaultLayout,
    createStackLayout,
    createWindowLayout,
    enforcePlaygroundWindowEngagementInput,
    enforceWindowKindsEngagementInput,
    getSidePanelBodyFactory,
    getWindowBodyFactory,
    isEdgelessWindowBody,
    isPlaygroundExampleLocked,
    isPlaygroundNoExampleId,
    playgroundExampleCatalogWithNoOption,
    playgroundResolvedExampleId,
    playgroundTreePanelRootItems,
    registerSidePanelBody,
    registerWindowBody,
    resolveAppState,
    resolveInitialPanelVisibility,
    resolvePlaygroundExampleCatalog,
    toolCollection,
    uiDeclarativeSectionsToTree,
    uiInspectorAllEqual,
    type CommandDescriptor,
    type Playground,
    type PlaygroundAppDefinition,
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
import { loadPlaygroundRendererContribution } from "@semio-tech/framework-playground-core/app-registry";
import type { OsAppInstance } from "@semio-tech/framework-os-core";
import { OsUpstreamBadge, useOsInstanceHostBridge, useOsInstanceMaterialization } from "@semio-tech/framework-os-renderer-react";
import type { ReactElement } from "react";
import * as React from "react";
import type { Root } from "react-dom/client";
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
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
    PLAYGROUND_NO_EXAMPLE_ID,
    PLAYGROUND_NO_EXAMPLE_OPTION, isPlaygroundNoExampleId, playgroundExampleCatalogWithNoOption,
    playgroundResolvedExampleId,
    resolvePlaygroundExampleCatalog
} from "@semio-tech/framework-playground-core";
export {
    FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID,
    FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL,
} from "@semio-tech/framework-core";
export type { AppExampleContribution, AppExampleOption } from "@semio-tech/framework-platform-core";

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
function isUiSurfaceHostNode(node: UiNode): node is UiComponentHostSurfaceNode {
  return typeof node === "object" && node !== null && "surfaceId" in node && "controllerId" in node;
}

function surfaceHostLayout(node: UiNode): "canvas" | "panel" {
  if (isUiSurfaceHostNode(node)) {
    const layout = (node as UiSurfaceHostNode).layout;
    if (layout) return layout;
  }
  if (node.type === "table" || node.type === "panel" || node.type === "editor" || node.type === "virtualFileSystem") {
    return "panel";
  }
  return "canvas";
}

function isPlaygroundCanvasHostChild(child: UiNode): boolean {
  return isUiSurfaceHostNode(child) && surfaceHostLayout(child) === "canvas";
}

export { registerSurfaceBinding, unregisterSurfaceBinding };

function renderPlaygroundHostSurface(node: UiNode, layout: "canvas" | "panel", platform?: Platform): React.ReactElement {
  if (isUiSurfaceHostNode(node)) {
    return renderComponentHostSurface(node, layout, platform);
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


let activeTreeDragController: AppRendererContribution["treeDragController"];

function buildUiTreeDragAndDropController(sections: readonly UiTreeSectionNode[], commandBus: CommandBus): TreeDragAndDropController | undefined {
  void commandBus;
  const dragByItemId = collectUiTreeItemDragData(sections);
  if (dragByItemId.size === 0 || !activeTreeDragController) {
    return undefined;
  }
  return activeTreeDragController(dragByItemId) as TreeDragAndDropController | undefined;
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
  const playground = reactHostPort.useContext(PlaygroundContext);
  const platform = playground?.runtime;
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
      if (isUiSurfaceHostNode(node)) {
        return renderPlaygroundHostSurface(node, surfaceHostLayout(node), platform);
      }
      return <div className="p-2 text-xs text-destructive">Unsupported UiNode</div>;
  }
}
//#endregion 🔖UiRenderer

//#region 🔖DeclarativeHosts
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
  if (control.kind === "ring" || control.kind === "toggleGroup") {
    return {
      kind: control.kind,
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
  if (control.kind === "select") {
    return {
      kind: "select",
      id: control.id,
      label: control.label,
      value: control.value,
      placeholder: control.placeholder,
      disabled: control.disabled,
      items: control.items.map((row) => ({ id: row.id, value: row.value, label: row.label })),
      onChange: control.onChange
        ? (value: string) => bus.dispatch(control.onChange!.controllerId, control.onChange!.command, { ...(control.onChange!.args as object | undefined), value })
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
  if (control.kind === "ring" || control.kind === "toggleGroup") {
    return {
      kind: control.kind,
      id: control.id,
      label: control.label,
      value: control.value,
      disabled: control.disabled,
      options: control.options.map((row) => ({ id: row.id, label: row.label, disabled: row.disabled })),
      onSelect: control.onSelect ? { controllerId, command: "engagementControlSelect", args: { ...commandArgs } } : undefined,
    };
  }
  if (control.kind === "select") {
    return {
      kind: "select",
      id: control.id,
      label: control.label,
      value: control.value,
      placeholder: control.placeholder,
      disabled: control.disabled,
      items: control.items.map((row) => ({ id: row.id, value: row.value, label: row.label })),
      onChange: control.onChange ? { controllerId, command: "engagementControlChange", args: { ...commandArgs, controlId: control.id } } : undefined,
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
  /** @emoji 🧪 App-declared navbar example dropdown (from {@link AppRendererContribution.examples}). */
  readonly exampleContribution?: AppExampleContribution;
  readonly extraFooterItems?: readonly FooterItem[];
  readonly augmentPanelTabs?: Partial<Record<"workbench" | "details" | "settings", readonly (SidePanelTabConfig | SidePanelTabDefinition)[]>>;
  readonly onActiveWindowChange?: (windowKindId: string) => void;
}

/** @emoji 🎛 Wraps controller {@link PlaygroundExampleHost} catalog + `setActiveExample` for {@link AppRendererContribution.examples}. */
export function controllerBackedExampleContribution(controllerId: string, options: readonly AppExampleOption[]): AppExampleContribution {
  return {
    options,
    activeExampleId: (runtime) => {
      const catalog = resolvePlaygroundExampleCatalog(runtime.getActiveApp()?.controller);
      return catalog?.activeExampleId ?? playgroundResolvedExampleId(options[0]?.id ?? PLAYGROUND_NO_EXAMPLE_ID);
    },
    onSelect: (exampleId, runtime) => {
      runtime.commandBus.dispatch(controllerId, "setActiveExample", { exampleId });
    },
  };
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

/** @emoji 🔔 Subscribes to runtime generation for app-declared navbar example catalog. */
function usePlaygroundExampleCatalog(runtime: Platform, contribution: AppExampleContribution | undefined): PlaygroundExampleCatalog | null {
  return reactHostPort.useSyncExternalStore(
    (listener) => {
      const controller = runtime.getActiveApp()?.controller;
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
      if (isPlaygroundExampleLocked() || !contribution) {
        return null;
      }
      const next = playgroundExampleCatalogWithNoOption(contribution.activeExampleId(runtime), contribution.options);
      const cacheKey = contribution;
      const cached = playgroundExampleCatalogSnapshotCache.get(cacheKey);
      if (cached === next) {
        return cached;
      }
      if (cached && playgroundExampleCatalogSemanticallyEqual(cached, next)) {
        return cached;
      }
      playgroundExampleCatalogSnapshotCache.set(cacheKey, next);
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
export const PlaygroundView: React.FC<PlaygroundViewProps> = ({ runtime, playgroundKeybindings, defaultAppId, mobile, mobileQuery = "(max-width: 767px)", initialPanelVisibility, slotToolbar, exampleContribution, extraFooterItems, augmentPanelTabs, onActiveWindowChange }) => {
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

  const exampleCatalog = usePlaygroundExampleCatalog(runtime, exampleContribution);
  const navbarExampleSelect = reactHostPort.useMemo(() => {
    if (!exampleContribution || !exampleCatalog) {
      return (
        <NavbarExampleSelect
          id="playground.navbar.fixture"
          value={NAVBAR_NO_EXAMPLE_ID}
          options={[{ id: NAVBAR_NO_EXAMPLE_ID, label: "No examples" }]}
          onValueChange={() => {}}
        />
      );
    }
    return (
      <NavbarExampleSelect
        id="playground.navbar.fixture"
        value={exampleCatalog.activeExampleId}
        options={exampleCatalog.options}
        onValueChange={(exampleId) => {
          exampleContribution.onSelect(exampleId, runtime);
        }}
      />
    );
  }, [exampleCatalog, exampleContribution, runtime]);

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
  rootElement.__playgroundRoot ??= reactHostPort.createRoot(rootElement);
  rootElement.__playgroundRoot.render(<PlaygroundShell>{element}</PlaygroundShell>);
}

/** @emoji 🚀 Alias for {@link mountPlaygroundApp}. */
export const mountReactApp = mountPlaygroundApp;
//#endregion 🔖Mount


//#region 🔖Boot

/** @emoji 🎮 Generic playground controller subscription (runtime generation + optional interaction revision). */
export function usePlayController<T extends PlaygroundController<string>>(
  runtimeOverride?: Platform,
  revision?: (ctrl: T | undefined) => number,
): T | undefined {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const runtime = runtimeOverride ?? appCtx?.runtime;
  reactHostPort.useSyncExternalStore(
    (listener) => {
      const unsubscribeRuntime = runtime ? runtime.subscribe(listener) : () => {};
      const ctrl = runtime?.getActiveApp()?.controller as T | undefined;
      const unsubscribeCtrl =
        ctrl && "subscribe" in ctrl && typeof ctrl.subscribe === "function"
          ? (ctrl as T & { subscribe: (l: () => void) => () => void }).subscribe(listener)
          : undefined;
      return () => {
        unsubscribeRuntime();
        unsubscribeCtrl?.();
      };
    },
    () => {
      const generation = runtime?.generation ?? 0;
      const ctrl = runtime?.getActiveApp()?.controller as T | undefined;
      const rev =
        revision?.(ctrl) ??
        (ctrl && "getInteractionRevision" in ctrl && typeof ctrl.getInteractionRevision === "function"
          ? (ctrl as T & { getInteractionRevision: () => number }).getInteractionRevision()
          : 0);
      return generation * 1_000_000 + rev;
    },
    () => 0,
  );
  return runtime?.getActiveApp()?.controller as T | undefined;
}

/** @emoji 📁 Generic fixture JSON file bridge wired through a controller host bridge. */
export function createFixtureFileBridge<T extends PlaygroundController<string>>(options: {
  readonly filename: string;
  readonly accept: string;
  readonly useController: (runtime?: Platform) => T | undefined;
  readonly getJson: (ctrl: T) => string;
  readonly applyJson: (ctrl: T, json: string) => void;
  readonly hostCommands?: Readonly<Record<string, "saveDownload" | "loadRequest">>;
}): React.FC {
  const FixtureFileBridge: React.FC = () => {
    const ctrl = options.useController();
    const loadInputRef = reactHostPort.useRef<HTMLInputElement | null>(null);
    const downloadFixture = reactHostPort.useCallback(async () => {
      if (!ctrl) return;
      const text = options.getJson(ctrl);
      const blob = new Blob([text], { type: "application/json" });
      const url = URL.createObjectURL(blob);
      const anchor = document.createElement("a");
      anchor.href = url;
      anchor.download = options.filename;
      anchor.click();
      URL.revokeObjectURL(url);
      console.log(`[DEBUG] ${options.filename} exported`);
    }, [ctrl]);
    const handleLoadFile = reactHostPort.useCallback(
      (event: React.ChangeEvent<HTMLInputElement>) => {
        const file = event.target.files?.[0];
        event.target.value = "";
        if (!file || !ctrl) return;
        void file.text().then((text) => {
          options.applyJson(ctrl, text);
          console.log(`[DEBUG] ${options.filename} imported from file`);
        });
      },
      [ctrl],
    );
    reactHostPort.useEffect(() => {
      if (!ctrl || !options.hostCommands) return;
      const bridge = {
        runHostCommand: (command: string) => {
          if (command === options.hostCommands?.saveDownload) void downloadFixture();
          if (command === options.hostCommands?.loadRequest) loadInputRef.current?.click();
        },
      };
      if ("setHostBridge" in ctrl && typeof ctrl.setHostBridge === "function") {
        (ctrl as T & { setHostBridge: (b: typeof bridge | null) => void }).setHostBridge(bridge);
        return () => (ctrl as T & { setHostBridge: (b: null) => void }).setHostBridge(null);
      }
      return undefined;
    }, [ctrl, downloadFixture]);
    return <input ref={loadInputRef} type="file" accept={options.accept} className="hidden" onChange={handleLoadFile} />;
  };
  return FixtureFileBridge;
}

/** @emoji 🖥️ Generic OS instance host wrapping materialization + upstream badge around a canvas component. */
export function createOsInstanceHost<TDocument>(options: {
  readonly Canvas: React.ComponentType<{ readonly document: TDocument; readonly onCommit: (document: TDocument) => void; readonly className?: string }>;
  readonly materialize: (instance: OsAppInstance, projection: unknown) => TDocument;
  readonly dispatch: (bridge: ReturnType<typeof useOsInstanceHostBridge>, instance: OsAppInstance, document: TDocument) => void;
}): React.FC<{ readonly instance: OsAppInstance }> {
  const InstanceHost: React.FC<{ readonly instance: OsAppInstance }> = ({ instance }) => {
    const bridge = useOsInstanceHostBridge();
    const bundle = useOsInstanceMaterialization(instance);
    const document = reactHostPort.useMemo(
      () => options.materialize(instance, bundle.projection),
      [instance, bundle.projection],
    );
    const onCommit = reactHostPort.useCallback(
      (next: TDocument) => options.dispatch(bridge, instance, next),
      [bridge, instance],
    );
    return (
      <div className="flex h-full min-h-0 flex-col overflow-hidden">
        <OsUpstreamBadge upstreamInstanceId={bundle.upstreamInstanceId} />
        <options.Canvas document={document} onCommit={onCommit} className="min-h-0 flex-1" />
      </div>
    );
  };
  return InstanceHost;
}

/** @emoji 🎛 Fills missing {@link AppRendererContribution.examples} from {@link PlaygroundExampleHost} on the active controller. */
export function finalizeRendererContribution(
  app: PlaygroundAppDefinition,
  contribution: AppRendererContribution,
  runtime: Platform,
): AppRendererContribution {
  if (contribution.examples || contribution.mountChrome) return contribution;
  const catalog = resolvePlaygroundExampleCatalog(runtime.getActiveApp()?.controller);
  if (!catalog?.options.length) return contribution;
  return {
    ...contribution,
    examples: controllerBackedExampleContribution(app.controllerId, catalog.options),
  };
}

/** @emoji 🧩 Registers surface hosts and tab icons from an app renderer contribution. */
export function applyAppRendererContribution(contribution: AppRendererContribution): void {
  for (const [surfaceId, component] of Object.entries(contribution.surfaceHosts)) {
    registerSurfaceBinding(surfaceId, component as React.ComponentType<{ readonly node: UiComponentHostSurfaceNode; readonly platform?: Platform }>);
  }
  if (contribution.windowBodies) {
    for (const [bodyKey, build] of Object.entries(contribution.windowBodies)) {
      registerWindowBody(bodyKey, build);
    }
  }
  if (contribution.sidePanelBodies) {
    for (const [bodyKey, build] of Object.entries(contribution.sidePanelBodies)) {
      registerSidePanelBody(bodyKey, build);
    }
  }
  if (contribution.tabIcons) {
    for (const [iconId, icon] of Object.entries(contribution.tabIcons)) {
      registerTabIcon(iconId, icon as Parameters<typeof registerTabIcon>[1]);
    }
  }
  activeTreeDragController = contribution.treeDragController;
}

/** @emoji 🛝 Boots a playground app from its definition — fully derived from {@link AppRendererContribution}. */
export async function bootPlaygroundApp(app: PlaygroundAppDefinition, playground: Playground, rootId = "root"): Promise<void> {
  const playEntryKind = app.devHost?.playEntryKind;
  if (!playEntryKind) throw new Error(`Playground app "${app.id}" is missing devHost.playEntryKind`);
  const contribution = finalizeRendererContribution(app, await loadPlaygroundRendererContribution(playEntryKind), playground.runtime);
  if (contribution.preload) {
    await contribution.preload();
  }
  applyAppRendererContribution(contribution);
  const mountProps = { runtime: playground.runtime, appId: app.id, panelTabs: contribution.panelTabs, examples: contribution.examples };
  const chrome = contribution.mountChrome
    ? (contribution.mountChrome(mountProps) as React.ReactElement)
    : (
        <PlaygroundView
          runtime={playground.runtime}
          defaultAppId={app.id}
          augmentPanelTabs={contribution.panelTabs as PlaygroundViewProps["augmentPanelTabs"]}
          exampleContribution={contribution.examples}
        />
      );
  mountPlaygroundApp(chrome, rootId);
}
//#endregion 🔖Boot

//#region 🧪Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("playground renderer slices", () => {
    it("keeps cross-dimensional brush host imports with their consumers", async () => {
      const { readFileSync } = await import("node:fs");
      const { dirname, join } = await import("node:path");
      const { fileURLToPath } = await import("node:url");
      const rendererDir = dirname(fileURLToPath(import.meta.url));
      const puzzle2dSource = readFileSync(join(rendererDir, "../../../../../puzzle/2d/react/index.tsx"), "utf8");
      expect(puzzle2dSource).toMatch(
        /puzzle2dSetBrushPlaceCommitHandler/,
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
      registerSurfaceBinding(surfaceId, TestGisMapHost);
      try {
        const html = renderToStaticMarkup(<UiRenderer node={buildMapWindowBody(surfaceId, "ctrl", "main")} commandBus={new CommandBus()} />);
        expect(html).toContain('data-host="gismap"');
        expect(html).not.toContain("Unsupported UiNode");
      } finally {
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
      registerSurfaceBinding(surfaceId, TestFlowHost);
      try {
        const html = renderToStaticMarkup(<UiRenderer node={buildFlowWindowBody(surfaceId, "ctrl", "main")} commandBus={new CommandBus()} />);
        expect(html).toContain('data-host="flow"');
        expect(html).not.toContain("Unsupported UiNode");
      } finally {
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
      registerSurfaceBinding(surfaceId, TestRasterHost);
      try {
        const html = renderToStaticMarkup(
          <UiRenderer node={buildRasterWindowBody(surfaceId, "ctrl", "composite", "composite")} commandBus={new CommandBus()} />,
        );
        expect(html).toContain('data-host="raster"');
        expect(html).not.toContain("Unsupported UiNode");
      } finally {
        unregisterSurfaceBinding(surfaceId);
      }
    });
  });

}
