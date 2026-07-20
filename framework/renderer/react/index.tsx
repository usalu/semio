// #region 🧱Header
/** @emoji 🎨 `@semio-tech/framework-renderer-react` — trusted React renderer for declarative Rust plugin UI trees. */
// #endregion 🧱Header

export type { ActionDescriptor, UiComponentSceneNode, UiNode } from "@semio-tech/framework-core";

import React, {
  lazy,
  memo,
  Suspense,
  useState,
  type ComponentType,
  type LazyExoticComponent,
  type ReactElement,
  type ReactNode,
  Component,
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useReducer,
  useRef,
  useSyncExternalStore,
  type CSSProperties,
  type DragEvent,
  type ComponentProps,
  type KeyboardEvent,
  type PointerEvent as ReactPointerEvent,
  type WheelEvent as ReactWheelEvent,
  useLayoutEffect,
  type MouseEvent,
} from "react";
import {
  Button,
  ChromeAwareWindowScrollSurface,
  Field,
  Icon,
  IconSelector,
  Input,
  Ring,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  Section,
  Slider,
  Stepper,
  Textarea,
  Toggle,
  Tree,
  VirtualFileSystem,
  borderElementClass,
  borderNormalTopClass,
  catalogueTreeDragController,
  classifyIconSelectorMode,
  cn,
  loadingBorderClass,
  loadingBorderElementClass,
  renderControlIcon,
  resolveTranslationLabel,
  uiI18n,
  useLabel,
  type TreeDataItem,
  type TreeDataSection,
  type TreeDragAndDropController,
  type TreePanelConfig,
  App,
  applyDockSkeleton,
  ButtonGroup,
  ButtonGroupItem,
  COMPOSE_WINDOW_TEMPLATE_MIME,
  CommandDialog,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
  dockSkeletonOf,
  dockSkeletonsEqual,
  findPanelTabInDock,
  Footer,
  Fuse,
  Layout,
  LevelProvider,
  Mode,
  moveTabInDock,
  moveTreeUnitInDock,
  Navbar,
  NavbarExampleSelect,
  PANEL_ANCHORS,
  PanelChromeTabBar,
  PanelDockProvider,
  Popover,
  PopoverAnchor,
  PopoverContent,
  Ribbon,
  SemioLogo,
  ShellBrandLogo,
  singleTreeLeaf,
  ToggleGroup,
  RibbonDivider,
  RibbonGroup,
  RibbonItem,
  RibbonZone,
  findPanelTabNode,
  findPanelTabPath,
  panelTabChildren,
  reconcileActivePath,
  WindowMeasureTreeGroup,
  WindowMeasureTreeLeaf,
  WindowMeasuresTree,
  bootstrapElementsSurfaceChromeDocument,
  borderNormalBottomClass,
  createEvenWindowLayout,
  iconRenderPort,
  getLevelBgClass,
  insertWindowAtDropZone,
  interactiveActiveFillClass,
  shellChromeTitleClassName,
  staticTreePanelDefinition,
  UiChromeLabelPolicyProvider,
  UI_MOBILE_MEDIA_QUERY,
  usePanelChromeHotkeys,
  useElementsSurfaceChrome,
  useMediaQuery,
  useActionHotkey,
  readStoredUiChromeCompact,
  readStoredUiChromeExpertise,
  readStoredUiChromeLayout,
  readStoredUiChromeLocale,
  readStoredUiChromeAppearance,
  readStoredUiChromeTerminology,
  readStoredUiChromeThemeId,
  readStoredUiChromeThemeSnapshot,
  readStoredUiCustomThemes,
  writeStoredUiChromeCompact,
  writeStoredUiChromeExpertise,
  writeStoredUiChromeLayout,
  writeStoredUiChromeLocale,
  writeStoredUiChromeAppearance,
  writeStoredUiChromeTerminology,
  writeStoredUiChromeThemeId,
  writeStoredUiChromeThemeSnapshot,
  writeStoredUiCustomThemes,
  activeUiTheme,
  builtinUiThemes,
  parseUiTheme,
  resolveThemeAppearancePalettes,
  semioTheme,
  serializeUiTheme,
  setActiveUiTheme,
  type ThemeAppearanceName,
  type ThemePaletteGroup,
  type UiTheme,
  windowTemplatePaletteTreeDragController,
  Expertise,
  setUiLocale,
  UI_TERMINOLOGY_NATIVE,
  UIIntroduction,
  UIDialog,
  introductionWindowActionPaneUnfoldSelector,
  introductionUtilityBarUnfoldSelector,
  readStoredIntroductionSeen,
  writeStoredIntroductionSeen,
  fundedByZukunftBauFooterItem,
  navbarFillItem,
  type ElementsSurfaceAppearance,
  type ElementsSurfaceDevice,
  type EngagementControl,
  type EngagementSpec,
  type FuseResult,
  type ModeWindowDescriptor,
  type NavbarItem,
  type PanelAnchor,
  type PanelDock,
  type PanelTabDockMove,
  type PanelTabNode,
  type PanelTabSelectionOptions,
  type PanelTreeUnitDockMove,
  type RibbonDirection,
  type RibbonRow,
  type UiChromeLayout,
  type UiChromeTerminologyId,
  type UiLocale,
  type UiTranslationKey,
  type WindowLayoutNode,
  type ModeCanvasDropTarget,
  type WindowTemplateDropPayload,
  CATALOGUE_DRAG_MIME,
  IconShotFrame,
  UnifiedGumball,
  ContextMenuController,
  getActiveCatalogueDragPayload,
  marqueeCoverageFromGesture,
  marqueeModeFromModifiers,
  menuListItemClassName,
  sceneHostPort,
  SelectionMarquee,
  sunPositionFromAzimuthElevation,
  type GumballConfig,
  type GumballHandleKind,
  type GumballPose,
  type SelectionMarqueeCoverage,
  type SelectionMarqueeMethod,
  type SelectionMarqueePoint,
  useCanvasAppearanceSync,
  CanvasPickMenu,
  Diagram,
  Handle,
  Position,
  useCanvasPickInteraction,
  type CanvasPickTarget,
  type Edge,
  type Node,
  type NodeProps,
  type NodeTypes,
  type ContextMenuItem,
  Table,
  type TableColumn,
  screenRectFromPoints,
  selectionMergeIds,
  type SelectionMergeMode,
  floatingMenuSurfaceClass,
  type IconName,
  pickMostSpecificCanvasTarget,
  type IconRenderRequest,
  HistoryTable,
  type HistoryColumn,
  closestCenter,
  DndContext,
  DndCSS,
  SortableContext,
  useSortable,
  verticalListSortingStrategy,
  type DragEndEvent,
  type UiRibbonParentCategory,
} from "@semio-tech/ui-react";
import { ICONS } from "@semio-tech/ui-asset";
import {
  type ActionDescriptor,
  type ComponentKind,
  type ComponentSceneHostProps,
  type UiControlNode,
  type UiNode,
  type UiStackNode,
  type UiTreeItemNode,
  type UiTreeNode,
  type UiTreeSectionNode,
  type InvocationResponse,
  type HostEffect,
  FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID,
  FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
  FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
  FRAMEWORK_PANEL_TAB_DOCUMENT_ICON_ID,
  FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
  FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
  FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID,
  FRAMEWORK_PANEL_TAB_INSPECTION_ID,
  FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
  FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID,
  FRAMEWORK_PANEL_TAB_PARAMETERS_ID,
  FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL,
  DockLayoutStore,
  DockUiStateStore,
  NamedLayoutStore,
  createBrowserStoragePort,
  createNamedLayout,
  type DockSkeleton,
  type DockUiPanelState,
  type DockUiState,
  loadPluginModule as loadCorePluginModule,
  loadPluginWasm as loadCorePluginWasm,
  buildContributionsJson,
  expandPluginRegistry,
  nodeGraphActions,
  panelTabKindId,
  resolveExternalSlots,
  resolveLayoutForMode,
  resolvePlaygroundDefaultAppId,
  resolvePluginHostConfig,
  resolvePluginRegistryId,
  resolveUiDirtyScope,
  textEditorActions,
  normalizeAppLabelsOverlay,
  deriveUtilityNodes,
  resolveWindowActions,
  partitionWindowMeasures,
  effectiveActionArgs,
  missingRequiredArgs,
  SET_ACTIVE_UTILITY_ACTION_ID,
  START_INTRODUCTION_ACTION_ID,
  type ActionArgControl,
  type ActionArgDef,
  type ActionDefinition,
  type AppDefinition,
  type AppModeDefinition,
  type AppWindowKindDefinition,
  type CommandDefinition,
  type DerivedUtilitySpec,
  type UtilityDefinition,
  type AppPanelTabDefinition,
  type Canvas2dScene,
  type DialogDefinition,
  type IntroductionAnchor,
  type IntroductionDefinition,
  type IntroductionStepDefinition,
  type TiledMapScene,
  type IconRenderScene,
  type NamedLayout,
  type NodeGraphScene,
  type InkCanvasScene,
  type PluginAppLabelsOverlay,
  type PluginHotSwapEvent,
  type PanelTabKind,
  type PluginRegistryEntry,
  type PluginUiRefreshRequest,
  type PluginUiRefreshResponse,
  type PluginUiRefreshSectionResponse,
  type PluginViewState,
  type PluginWasmHandle as CorePluginWasmHandle,
  type PresencePeer,
  type UiDirtyScope,
  type Paint2dScene,
  type Board2dScene,
  type StyleSpec,
  type TableScene,
  type TextEditorScene,
  type UtilityCategory,
  type UtilityLeaf,
  type UtilityNode,
  type UiButtonNode,
  type UiComponentSceneNode,
  type UiExternalSlotNode,
  type UiFieldNode,
  type UiIconSelectNode,
  type UiImageNode,
  type UiInputNode,
  type UiInspectorFieldGroup,
  type UiKeyValueEntry,
  type UiKeyValueNode,
  type UiNumberStepperNode,
  type UiRingNode,
  type UiSectionNode,
  type UiSelectItem,
  type UiSelectNode,
  type UiSeparatorNode,
  type UiSliderNode,
  type UiTextNode,
  type UiToggleNode,
  type UiTreeItemAction,
  type UiVec3Node,
  type GraphTimelineScene,
  type VirtualFileSystemScene,
  type WindowEngagement,
  type WindowEngagementControl,
  type WindowEngagementInput,
  type WindowEngagementOption,
  type WindowEngagementPossible,
  type WindowEngagementRingOption,
  type WindowEngagementSelectItem,
  type WindowEngagementStatus,
  type WindowEngagementToggleGroupOption,
  type WindowLayout,
  type WindowLayoutAxisNode,
  type WindowLayoutStackNode,
  type WindowLayoutWindowNode,
  type WindowMeasure,
  type World3dScene,
  postPluginBackboneInbound,
  setPluginBackboneOutboundRelay,
  inkCanvasActions,
  type EventFeedEntry,
  type ShellBrand,
} from "@semio-tech/framework-core";
import { createRoot } from "react-dom/client";
import { type GraphWasmSession, GraphWasmCanvas, type CanvasInputModifiers } from "@semio-tech/infinite-cavas-react-renderer";
import {
  FRAMEWORK_SYNC_CONTROLLER_ID,
  buildFileBackboneUri,
  buildFolderBackboneUri,
  buildFrameworkSyncUtilities,
  buildRemoteBackboneUri,
  type BackboneWorkerRequest,
  type BackboneWorkerResponse,
  type DocumentActorMsg,
  type DocumentSyncStatus,
  type FrameworkSyncUtilityLeaf,
  type PersistenceBinding,
} from "@semio-tech/framework-os-core";
import {
  BufferAttribute,
  BufferGeometry,
  CanvasTexture,
  ClampToEdgeWrapping,
  DoubleSide,
  MeshStandardMaterial,
  Box3,
  Color,
  EdgesGeometry,
  Group,
  LineBasicMaterial,
  LineSegments,
  Mesh,
  Object3D,
  PointsMaterial,
  Quaternion,
  TextureLoader,
  Vector3,
  type ThreeEvent,
} from "three";
import { useFrame, useLoader, useThree } from "@react-three/fiber";
import { GLTFLoader } from "three/examples/jsm/loaders/GLTFLoader.js";
import {
  DEFAULT_LOD_GRID_FACTOR,
  DEFAULT_MANUAL_LOD,
  GLB_MESH_FRAME_ROTATION_X,
  WORLD_MESH_OUTLINE_USER_DATA_KEY,
  WorldCanvas,
  WorldLayerStack,
  WorldLodBridge,
  WorldOrbitCameraViewRig,
  WorldOrbitGated,
  WorldOrbitProjectionSwitch,
  WorldOrbitViewSnapGateProvider,
  WorldReferenceLayer,
  WorldVolumeLayer,
  type OrbitCameraProjection,
  type WorldCameraState,
} from "@semio-tech/infinite-world-r3f";
import { clearColorResolveCache, resolveColorHex, semanticVar, themeColorVar, tokenVar, syncSessionCanvasTheme, resolveSemanticColorHex } from "@semio-tech/ui-styling";

//#region 🔖UiInterpreter
//#region ComponentSceneHostRegistry
/** 🧩 Wraps a dynamic host-module import into a lazily-loaded component bound to a named export. */
function lazyHost<P>(loader: () => Promise<Record<string, unknown>>, exportName: string): LazyExoticComponent<ComponentType<P>> {
  return lazy(async () => {
    const module = await loader();
    return { default: module[exportName] as ComponentType<P> };
  });
}

const COMPONENT_SCENE_HOSTS: Record<ComponentKind, LazyExoticComponent<ComponentType<ComponentSceneHostProps>>> = {
  "canvas-2d": lazyHost(() => Promise.resolve({ Canvas2dHost }), "Canvas2dHost"),
  "world-3d": lazyHost(() => Promise.resolve({ World3dHost }), "World3dHost"),
  "node-graph": lazyHost(() => Promise.resolve({ NodeGraphHost }), "NodeGraphHost"),
  "text-editor": lazyHost(() => Promise.resolve({ TextEditorHost }), "TextEditorHost"),
  table: lazyHost(() => Promise.resolve({ TableHost }), "TableHost"),
  "paint-2d": lazyHost(() => Promise.resolve({ Paint2dHost }), "Paint2dHost"),
  "tiled-map": lazyHost(() => Promise.resolve({ TiledMapHost }), "TiledMapHost"),
  "board-2d": lazyHost(() => Promise.resolve({ Board2dHost }), "Board2dHost"),
  "icon-render": lazyHost(() => Promise.resolve({ IconRenderHost }), "IconRenderHost"),
  "ink-canvas": lazyHost(() => Promise.resolve({ InkCanvasHost }), "InkCanvasHost"),
  "graph-timeline": lazyHost(() => Promise.resolve({ GraphTimelineHost }), "GraphTimelineHost"),
  "block-list": lazyHost(() => Promise.resolve({ BlockListHost }), "BlockListHost"),
  "diff-view": lazyHost(() => Promise.resolve({ DiffViewHost }), "DiffViewHost"),
  "event-feed": lazyHost(() => Promise.resolve({ EventFeedHost }), "EventFeedHost"),
};
//#endregion ComponentSceneHostRegistry

/** @emoji 🗣️ Resolves a chrome translation key outside hook context (plain node-builder functions run there). */
function interpLabel(key: string): string {
  return resolveTranslationLabel(uiI18n.t(key as never)) ?? key;
}

function ComponentSceneFallback() {
  const loadingSurfaceLabel = useLabel("ui.common.loadingSurface");
  return (
    <p className={cn("text-muted-foreground p-2 text-xs", loadingBorderClass)} role="status">
      {loadingSurfaceLabel}
    </p>
  );
}

function renderComponentSceneHost(node: Extract<UiNode, { type: "componentScene" }>, onAction: (action: ActionDescriptor) => void): ReactNode {
  if (node.componentKind === "virtualFileSystem") {
    return <VirtualFileSystemHost node={node} onAction={onAction} />;
  }
  const Host = COMPONENT_SCENE_HOSTS[node.componentKind as ComponentKind];
  if (!Host) {
    return (
      <p className="text-muted-foreground text-xs">
        {interpLabel("ui.common.unknownComponent")}: {node.componentKind}
      </p>
    );
  }
  return (
    <Suspense fallback={<ComponentSceneFallback />}>
      <Host node={node} onAction={onAction} />
    </Suspense>
  );
}

//#region UiInterpreterContext
export type UiInterpreterContext = {
  readonly onAction: (action: ActionDescriptor) => void;
};
//#endregion UiInterpreterContext

//#region ActionDispatch
function dispatchUiAction(onAction: UiInterpreterContext["onAction"], descriptor: ActionDescriptor, patch: Record<string, unknown>): void {
  onAction({
    ...descriptor,
    args: { ...(typeof descriptor.args === "object" && descriptor.args != null ? descriptor.args : {}), ...patch },
  });
}

function resolveDeclarativeControlIcon(iconId: string, size: number | "tiny" | "small" | "base" | "large" = "small"): ReactNode {
  const iconName = iconId in ICONS ? (iconId as IconName) : "circle-dot";
  return <Icon icon={iconName} size={size} />;
}
//#endregion ActionDispatch

//#region RenderUiControl
/** @emoji 🎛 Renders a declarative control node with ui-react primitives. */
export function renderUiControl(control: UiControlNode, onAction: UiInterpreterContext["onAction"]): ReactElement {
  switch (control.type) {
    case "input": {
      const commitOnBlur = control.commit === "blur";
      const commitValue = (raw: string) => {
        const value = control.inputKind === "number" ? Number(raw) : raw;
        dispatchUiAction(onAction, control.onChange, { value });
      };
      if (control.inputKind === "longText") {
        return (
          <Textarea
            id={control.id}
            className="min-h-[4.5rem] w-full min-w-0"
            value={control.value}
            placeholder={control.placeholder}
            onChange={commitOnBlur ? undefined : (event) => commitValue(event.target.value)}
            onBlur={commitOnBlur ? (event) => commitValue(event.target.value) : undefined}
          />
        );
      }
      const inputType = control.inputKind === "number" ? "number" : control.inputKind === "date" ? "date" : control.inputKind === "color" ? "color" : control.inputKind === "file" ? "file" : "text";
      return (
        <Input
          id={control.id}
          type={inputType}
          className="h-medium w-full min-w-0"
          value={control.inputKind === "file" ? undefined : control.value}
          placeholder={control.placeholder}
          min={control.min}
          max={control.max}
          step={control.step}
          accept={control.inputKind === "file" ? control.accept : undefined}
          onChange={
            commitOnBlur
              ? undefined
              : (event) => {
                  if (control.inputKind === "file") {
                    commitValue(event.target.files?.[0]?.name ?? "");
                    return;
                  }
                  commitValue(event.target.value);
                }
          }
          onBlur={
            commitOnBlur
              ? (event) => {
                  if (control.inputKind === "file") {
                    commitValue(event.target.files?.[0]?.name ?? "");
                    return;
                  }
                  commitValue(event.target.value);
                }
              : undefined
          }
        />
      );
    }
    case "select":
      return (
        <Select value={control.value || undefined} onValueChange={(value) => dispatchUiAction(onAction, control.onChange, { value })}>
          <SelectTrigger id={control.id} className="h-medium w-full min-w-0" size="sm">
            <SelectValue placeholder={control.placeholder ?? interpLabel("ui.common.select")} />
          </SelectTrigger>
          <SelectContent>
            {control.items.map((item, index) => (
              <SelectItem key={`${control.id}:${index}:${item.value}`} value={item.value}>
                {item.label}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      );
    case "toggle":
      return <Toggle id={control.id} pressed={control.pressed} text={control.text} icon={resolveDeclarativeControlIcon(control.iconId)} onPressedChange={(pressed) => dispatchUiAction(onAction, control.onChange, { pressed })} />;
    case "vec3": {
      const tuple = control.value;
      const mixed = tuple == null || !Array.isArray(tuple) || tuple.length < 3;
      const axes = ["x", "y", "z"] as const;
      return (
        <div className="grid grid-cols-3 gap-single">
          {axes.map((axis, index) => (
            <Input
              key={`${control.id}.${axis}`}
              id={`${control.id}.${axis}`}
              type="number"
              className="h-medium w-full min-w-0"
              value={mixed ? "" : String(tuple[index] ?? 0)}
              placeholder={mixed ? "—" : axis}
              disabled={mixed}
              onChange={(event) => {
                if (mixed) return;
                const parsed = Number(event.target.value);
                if (!Number.isFinite(parsed)) return;
                const next: [number, number, number] = [tuple[0] ?? 0, tuple[1] ?? 0, tuple[2] ?? 0];
                next[index] = parsed;
                dispatchUiAction(onAction, control.onChange, { value: next });
              }}
            />
          ))}
        </div>
      );
    }
    case "keyValue":
      return (
        <dl className="grid grid-cols-[auto_1fr] gap-x-single gap-y-single text-xs">
          {control.entries.map((entry) => (
            <div key={entry.label} className="contents">
              <dt className="text-muted-foreground">{entry.label}</dt>
              <dd className="tabular-nums">{entry.value}</dd>
            </div>
          ))}
        </dl>
      );
    case "slider": {
      const slider = (
        <Slider
          id={control.id}
          className="w-full min-w-0"
          max={control.max}
          min={control.min}
          step={control.step}
          value={[control.value]}
          onValueChange={(values) => dispatchUiAction(onAction, control.onChange, { value: values[0] ?? control.value })}
        />
      );
      if (!control.unit) return slider;
      return (
        <div className="flex min-w-0 w-full items-center gap-single">
          {slider}
          <span className="text-muted-foreground shrink-0 text-xs tabular-nums">
            {control.value} {control.unit}
          </span>
        </div>
      );
    }
    case "numberStepper":
      return (
        <Stepper
          id={control.id}
          step={control.step}
          value={control.uniform ? control.value : undefined}
          mixed={!control.uniform}
          onChange={(value) => dispatchUiAction(onAction, control.onAbsolute, { value })}
          onDelta={(delta) => dispatchUiAction(onAction, control.onDelta, { delta })}
        />
      );
    case "ring":
      return <Ring id={control.id} onOrbChange={(_orbId, _oldT, newT) => dispatchUiAction(onAction, control.onChange, { t: newT })} orbs={[{ disabled: control.disabled, id: control.orbId, selected: true, t: control.t }]} />;
    case "iconSelect":
      return (
        <IconSelector
          classifyIconSelectorMode={control.classifierKind === "puzzle2d" ? classifyIconSelectorMode : undefined}
          id={control.id}
          onChange={(next) => dispatchUiAction(onAction, control.onChange, { value: next })}
          uniform={control.uniform}
          value={control.value}
        />
      );
    case "button":
      return (
        <Button
          id={control.id}
          text={control.label}
          icon={resolveDeclarativeControlIcon(control.iconId)}
          disabled={control.disabled}
          onClick={() => onAction(control.action)}
          className={control.loading ? loadingBorderElementClass : undefined}
          aria-busy={control.loading || undefined}
        />
      );
  }
}
//#endregion RenderUiControl

//#region UiTreePanel
function uiTreeItemsToTreeData(items: readonly UiTreeItemNode[], onAction: UiInterpreterContext["onAction"]): TreeDataItem[] {
  return items.map((item) => ({
    id: item.id,
    label: item.label,
    description: item.description,
    icon: item.iconId ? renderControlIcon(item.iconId, 12) : undefined,
    control: item.control ? renderUiControl(item.control, onAction) : undefined,
    defaultOpen: item.defaultOpen,
    isSelected: item.selected,
    loading: item.loading,
    isHidden: item.isHidden,
    draggable: item.draggable,
    dragData: item.dragData,
    className: item.draggable || item.dragData ? "cursor-grab active:cursor-grabbing" : undefined,
    items: item.items?.length ? uiTreeItemsToTreeData(item.items, onAction) : undefined,
    onClick: item.action ? () => dispatchUiAction(onAction, item.action!, {}) : undefined,
    onPointerEnter: item.hoverAction ? () => dispatchUiAction(onAction, item.hoverAction!, {}) : undefined,
    onPointerLeave: item.unhoverAction ? () => dispatchUiAction(onAction, item.unhoverAction!, {}) : undefined,
    actions: item.actions?.map((action) => ({
      kind: "button" as const,
      icon: renderControlIcon(action.iconId, 12),
      title: action.label,
      revealOnHover: action.revealOnHover,
      onClick: () => dispatchUiAction(onAction, action.action, {}),
    })),
  }));
}

/** @emoji 🌲 Maps a declarative {@link UiTreeNode} to a {@link TreePanelConfig}. */
export function uiTreeNodeToTreePanelConfig(treeNode: UiTreeNode, onAction: UiInterpreterContext["onAction"]): TreePanelConfig {
  const sections: TreeDataSection[] = treeNode.sections.map((section: UiTreeSectionNode) => ({
    id: section.id,
    label: section.label ?? "",
    defaultOpen: section.defaultOpen,
    loading: section.loading,
    items: uiTreeItemsToTreeData(section.items, onAction),
  }));
  return {
    sections,
    selectedIds: treeNode.selectedIds as string[] | undefined,
    highlightedIds: treeNode.highlightedIds,
    onSelectionChange: treeNode.selectionChange ? (selectedIds) => dispatchUiAction(onAction, treeNode.selectionChange!, { ids: selectedIds }) : undefined,
    sortableSections: sections.length > 1,
  };
}

function treeDragPayloadMime(treeNode: UiTreeNode): string | undefined {
  for (const section of treeNode.sections) {
    const visit = (items: readonly UiTreeItemNode[]): string | undefined => {
      for (const item of items) {
        const mime = item.dragData ? Object.keys(item.dragData)[0] : undefined;
        if (mime) return mime;
        const nested = item.items?.length ? visit(item.items) : undefined;
        if (nested) return nested;
      }
      return undefined;
    };
    const mime = visit(section.items);
    if (mime) return mime;
  }
  return undefined;
}

/** @emoji 🖱️ Builds the drag/drop controller for a declarative tree — palette source when items carry drag payloads, drop dispatch when the tree declares a drop action. */
export function declarativeTreeDragController(treeNode: UiTreeNode, onAction: UiInterpreterContext["onAction"]): TreeDragAndDropController | undefined {
  const mime = treeDragPayloadMime(treeNode);
  const source = mime ? catalogueTreeDragController(mime) : undefined;
  const dropAction = treeNode.dropAction;
  if (!dropAction) return source;
  return {
    ...(source ?? {}),
    handleDrop: ({ data, target, dropPosition }) => {
      const encoded = Object.entries(data).find(([kind, value]) => kind.startsWith("application/x-semio-") && value.trim())?.[1];
      if (!encoded) return;
      let payload: Record<string, unknown>;
      try {
        payload = JSON.parse(encoded) as Record<string, unknown>;
      } catch {
        return;
      }
      dispatchUiAction(onAction, dropAction, { ...payload, targetId: target.id, dropPosition: dropPosition ?? "inside" });
    },
  };
}

function DeclarativeTreePanel({ treeNode, onAction }: { readonly treeNode: UiTreeNode; readonly onAction: UiInterpreterContext["onAction"] }) {
  const config = uiTreeNodeToTreePanelConfig(treeNode, onAction);
  const dragController = declarativeTreeDragController(treeNode, onAction);
  return (
    <Tree
      className="min-h-0 min-w-0 flex-1 overflow-auto"
      sections={config.sections}
      selectionMode={config.selectedIds?.length ? "multiple" : "single"}
      showLines
      selectedIds={config.selectedIds}
      highlightedIds={config.highlightedIds}
      onSelectionChange={config.onSelectionChange}
      dragAndDropController={dragController}
      sortableSections={config.sortableSections ?? config.sections.length > 1}
      onSectionsReorder={config.onSectionsReorder}
    />
  );
}
//#endregion UiTreePanel

//#region VirtualFileSystemHost
function VirtualFileSystemHost({ node, onAction }: { readonly node: Extract<UiNode, { type: "componentScene" }>; readonly onAction: (action: ActionDescriptor) => void }) {
  const scene = node.virtualFileSystem;
  if (!scene) return <div className="semio-vfs-empty">No virtual file system scene</div>;
  const schema = JSON.parse(scene.schemaJson) as Parameters<typeof VirtualFileSystem>[0]["schema"];
  const rows = JSON.parse(scene.rowsJson) as Parameters<typeof VirtualFileSystem>[0]["rows"];
  const selectedRowIds = scene.selectedRowIdsJson ? (JSON.parse(scene.selectedRowIdsJson) as string[]) : undefined;
  return (
    <VirtualFileSystem
      className="min-h-0 flex-1"
      schema={schema}
      rows={rows}
      selectedRowIds={selectedRowIds}
      emptyMessage={scene.emptyMessage}
      dragDrop={scene.dragDropEnabled ? { enabled: true } : undefined}
      onSelectionChange={(ids) =>
        onAction({
          controllerId: node.controllerId,
          action: "selectRows",
          args: { surfaceId: node.surfaceId, ids },
        })
      }
      onRowDoubleClick={(row) => {
        const uri = row.navigateUri;
        if (!uri) return;
        if (uri.startsWith("os://instance/")) {
          onAction({
            controllerId: node.controllerId,
            action: "openInstance",
            args: { surfaceId: node.surfaceId, instanceId: uri.slice("os://instance/".length) },
          });
          return;
        }
        if (uri.startsWith("os://export/")) {
          const [, , , instanceId, , format] = uri.split("/");
          if (instanceId && format) {
            onAction({
              controllerId: node.controllerId,
              action: "exportMedia",
              args: { surfaceId: node.surfaceId, instanceId, format },
            });
          }
          return;
        }
        if (uri.startsWith("os://import/")) {
          const [, , , instanceId, resourceKind, format] = uri.split("/");
          if (instanceId && format) {
            onAction({
              controllerId: node.controllerId,
              action: "importMedia",
              args: { surfaceId: node.surfaceId, instanceId, resourceKind, format },
            });
          }
          return;
        }
        if (uri.startsWith("/studios/")) {
          const studioId = uri.split("/")[2];
          if (studioId) {
            onAction({
              controllerId: node.controllerId,
              action: "navigateVirtualFileSystemNode",
              args: { surfaceId: node.surfaceId, studioId },
            });
          }
          return;
        }
        if (uri.startsWith("studio:")) {
          onAction({
            controllerId: node.controllerId,
            action: "navigateVirtualFileSystemNode",
            args: { surfaceId: node.surfaceId, studioId: uri.slice("studio:".length) },
          });
        }
      }}
    />
  );
}
//#endregion VirtualFileSystemHost

//#region InterpretUiNode
function uiNodeKey(node: UiNode, index: number): string {
  if ("id" in node && typeof node.id === "string" && node.id) return node.id;
  return `${node.type}:${index}`;
}

/** @emoji 🫳 Stateful host for a {@link UiStackNode} — the plain stack layout/click/drop wiring plus local drag-over tracking so `dropOverlay` can show a full-bleed hint while a drag hovers, ahead of `dropAction` firing on release. */
function UiStackHost({ node, context }: { readonly node: UiStackNode; readonly context: UiInterpreterContext }) {
  const [dragOver, setDragOver] = useState(false);
  const activate = node.activate;
  const dropAction = node.dropAction;
  const dropOverlay = node.dropOverlay;
  return (
    <div
      className={cn(
        "relative flex min-h-0 min-w-0 flex-1",
        node.direction === "horizontal" ? "flex-row" : "flex-col",
        node.gap === "none" ? "gap-0" : node.gap === "tight" ? "gap-single" : node.gap === "relaxed" ? "gap-small" : "gap-double",
        node.padding === "none" ? "p-0" : "p-double",
        `semio-ui-stack semio-ui-stack--${node.direction}`,
        activate && cn(borderElementClass, "border bg-panel cursor-pointer rounded-md"),
        node.selected && "ring-primary border-primary ring-1",
      )}
      data-ui-stack={node.id}
      role={activate ? "button" : undefined}
      onClick={
        activate
          ? (event) => {
              event.stopPropagation();
              dispatchUiAction(context.onAction, activate, {});
            }
          : undefined
      }
      onDragOver={
        dropAction
          ? (event) => {
              event.preventDefault();
              event.dataTransfer.dropEffect = "copy";
              if (dropOverlay && !dragOver) setDragOver(true);
            }
          : undefined
      }
      onDragLeave={
        dropAction && dropOverlay
          ? (event) => {
              event.preventDefault();
              setDragOver(false);
            }
          : undefined
      }
      onDrop={
        dropAction
          ? (event) => {
              event.preventDefault();
              event.stopPropagation();
              setDragOver(false);
              const encoded = [...event.dataTransfer.types].filter((kind) => kind.startsWith("application/x-semio-")).map((kind) => event.dataTransfer.getData(kind))[0];
              if (!encoded?.trim()) return;
              try {
                dispatchUiAction(context.onAction, dropAction, JSON.parse(encoded) as Record<string, unknown>);
              } catch {
                return;
              }
            }
          : undefined
      }
    >
      {node.children.map((child, index) => (
        <div key={uiNodeKey(child, index)} className="flex-auto">
          {interpretUiNode(child, context)}
        </div>
      ))}
      {dropOverlay && dragOver ? (
        <div
          className="border-primary pointer-events-none absolute inset-0 z-10 flex flex-col items-center justify-center gap-single rounded-md border-2 border-dashed p-double text-center"
          style={{ background: "color-mix(in oklab, var(--panel) 92%, transparent)" }}
        >
          <p className="text-sm font-semibold">{dropOverlay.title}</p>
          <p className="text-muted-foreground text-xs">{dropOverlay.hint}</p>
        </div>
      ) : null}
    </div>
  );
}

/** @emoji 🌳 Interprets a declarative {@link UiNode} tree into ui-react components. */
export function interpretUiNode(node: UiNode, context: UiInterpreterContext): ReactNode {
  switch (node.type) {
    case "stack":
      return <UiStackHost node={node} context={context} />;
    case "text":
      return <p className={node.emphasize ? "font-semibold" : "text-sm"}>{node.value}</p>;
    case "button":
      return <Button id={node.id} text={node.label} icon={resolveDeclarativeControlIcon(node.iconId)} disabled={node.disabled} onClick={() => context.onAction(node.action)} />;
    case "separator":
      return <hr className={cn("border-0", borderNormalTopClass)} />;
    case "image":
      return <img id={node.id} src={node.src} alt={node.alt ?? ""} className="max-h-64 max-w-full rounded-md object-contain" data-ui-image={node.id} />;
    case "input":
      return renderUiControl(node, context.onAction);
    case "select":
      return renderUiControl(node, context.onAction);
    case "toggle":
      return renderUiControl(node, context.onAction);
    case "vec3":
      return renderUiControl(node, context.onAction);
    case "keyValue":
      return renderUiControl(node, context.onAction);
    case "slider":
      return renderUiControl(node, context.onAction);
    case "numberStepper":
      return renderUiControl(node, context.onAction);
    case "ring":
      return renderUiControl(node, context.onAction);
    case "iconSelect":
      return renderUiControl(node, context.onAction);
    case "field":
      return (
        <Field id={node.id} label={node.label} description={node.description} required={node.required} error={node.error}>
          {interpretUiNode(node.child, context)}
        </Field>
      );
    case "section":
      return (
        <Section id={node.id} title={node.label}>
          {node.children.map((child, index) => (
            <div key={uiNodeKey(child, index)}>{interpretUiNode(child, context)}</div>
          ))}
        </Section>
      );
    case "tree":
      return <DeclarativeTreePanel treeNode={node} onAction={context.onAction} />;
    case "componentScene":
      return renderComponentSceneHost(node, context.onAction);
    case "externalSlot":
      return <p className="text-muted-foreground text-xs">Extension unavailable: {node.pluginId}</p>;
  }
}

/**
 * @emoji 🐢 `React.memo`'d entry point into `interpretUiNode` — bails on re-interpreting (and
 * reconciling) an entire window/panel subtree when both `node` and `onAction` keep the same object
 * identity as last render. Only pays off when callers pass a stable `onAction` (see `os-shell.tsx`'s
 * `onActionStable`) and a `node` whose identity is preserved across no-op refreshes (see
 * `os-shell.tsx`'s `preserveJsonIdentity`/`mergeRecordPreservingIdentity`) — without both, `node`/
 * `onAction` are fresh every render and this degenerates to the unmemoized call.
 */
export const InterpretedUiNode = memo(function InterpretedUiNode({ node, onAction }: { readonly node: UiNode; readonly onAction: UiInterpreterContext["onAction"] }): ReactNode {
  return interpretUiNode(node, { onAction });
});
//#endregion InterpretUiNode
//#endregion 🔖UiInterpreter

//#region 🔖OsShell
/** 🔁 Re-exported so `node-graph-host.tsx`/`text-editor-host.tsx` can import action-name maps from this shell module rather than reaching into `@semio-tech/framework-core` directly. */
export { nodeGraphActions, textEditorActions };

//#region 🔖types
/** 🌐 Locale-resolved mixed-value placeholder for this renderer layer; framework/core/js/index.ts keeps its own non-reactive low-level default. */
export const UI_INSPECTOR_MIXED_PLACEHOLDER = shellLabel("ui.common.mixedValues");

/** 🎭 Renderer-side view state passed to plugin wasm calls — structurally mirrors `@semio-tech/framework-core`'s {@link PluginViewState}, kept as a distinct local alias since `ViewState` is the established name used throughout this file. */
export type ViewState = PluginViewState;

/** ⚠️ Not folded into `@semio-tech/framework-core`'s `PluginManifest`: this shell-local shape types `apps`/`programs` richly (`AppDefinition[]`, `document` on programs) where core intentionally keeps the wasm-boundary shape loose (`Record<string, unknown>[]`) for other consumers (e.g. compose, coda). Left for a human to decide whether to widen core's `PluginManifest` itself. */
export type PluginManifest = {
  readonly pluginId: string;
  readonly label: string;
  readonly version: string;
  readonly apps: readonly AppDefinition[];
  readonly programs: readonly { readonly programId: string; readonly appId: string; readonly label: string; readonly document: readonly string[]; readonly yields: string }[];
  readonly examples: readonly { readonly id: string; readonly label: string; readonly documentJson: string; readonly appId: string }[];
  readonly contributions?: readonly {
    readonly kind: "protocolBlockKind";
    readonly appId: string;
    readonly blockKind: string;
    readonly label: string;
    readonly iconId: string;
    readonly defaultValueJson?: string;
    readonly paramsBodyKey: string;
    readonly previewBodyKey: string;
  }[];
  /** 🎛️ Plugin-scope commands this plugin exposes — apply whenever any of its apps is focused. */
  readonly commands?: readonly CommandDefinition[];
};

type LoadedPluginState = {
  readonly handle: PluginWasmHandle;
  readonly manifest: PluginManifest;
};

type ActiveSession = {
  readonly pluginId: string;
  readonly instanceId: number;
  readonly app: AppDefinition;
  readonly viewState: ViewState;
};

type StudioProgramEntry = {
  readonly pluginId: string;
  readonly programId: string;
  readonly appId: string;
  readonly label: string;
  readonly document: readonly string[];
  readonly yields: string;
};

type SpawnedAppEntry = {
  readonly id: string;
  readonly pluginId: string;
  readonly instanceId: number;
  readonly appId: string;
  readonly label: string;
  readonly document: readonly string[];
};

type StudioPanelState = {
  readonly activePanelTab: string;
  readonly programs: readonly StudioProgramEntry[];
  readonly spawnedApps: readonly SpawnedAppEntry[];
  readonly activeSpawnedId?: string;
};

export type FrameworkOsBootOptions = {
  readonly rootId?: string;
  readonly plugin?: string;
  readonly plugins?: readonly { readonly pluginId: string; readonly moduleUrl: string }[];
  readonly appId?: string;
  readonly locks?: FrameworkOsLocks;
  readonly defaults?: FrameworkOsDefaults;
  readonly brand?: ShellBrand;
};

//#region 🔒FrameworkOsLocks
/** 🔒 Raw boot-time lock values from env, before validation (any of the five may be unset). */
export type FrameworkOsLocks = {
  readonly exampleId?: string;
  readonly locale?: string;
  readonly terminology?: string;
  readonly themeId?: string;
  readonly appearance?: string;
};

/** 🔒 Validated locks: unknown values warn and fall back to a safe default while staying locked. */
export type ResolvedShellLocks = {
  readonly exampleId?: string;
  readonly locale?: UiLocale;
  readonly terminology?: string;
  readonly themeId?: string;
  readonly appearance?: ElementsSurfaceAppearance;
};

/**
 * 🔒 Validates raw `FrameworkOsLocks` against what the shell can actually apply at boot. A locked
 * session stays locked even on an invalid value (falls back to a default) rather than silently
 * degrading to switchable — the CLI asked for no in-app switching, so a typo must not remove that.
 */
export function resolveShellLocks(locks: FrameworkOsLocks | undefined): ResolvedShellLocks {
  if (!locks) return {};
  const resolved: { -readonly [K in keyof ResolvedShellLocks]?: ResolvedShellLocks[K] } = {};
  if (locks.exampleId) resolved.exampleId = locks.exampleId;
  if (locks.locale !== undefined) {
    if (locks.locale === "en" || locks.locale === "de") {
      resolved.locale = locks.locale;
    } else {
      console.warn(`[os] invalid SEMIO_LOCKED_LOCALE ${JSON.stringify(locks.locale)}, falling back to "en"`);
      resolved.locale = "en";
    }
  }
  if (locks.terminology) resolved.terminology = locks.terminology;
  if (locks.themeId !== undefined) {
    const known = new Set([...builtinUiThemes().map((t) => t.id), ...Object.keys(readStoredUiCustomThemes())]);
    if (known.has(locks.themeId)) {
      resolved.themeId = locks.themeId;
    } else {
      console.warn(`[os] invalid SEMIO_LOCKED_THEME ${JSON.stringify(locks.themeId)}, falling back to "semio"`);
      resolved.themeId = "semio";
    }
  }
  if (locks.appearance !== undefined) {
    if (locks.appearance === "light" || locks.appearance === "dark") {
      resolved.appearance = locks.appearance;
    } else {
      console.warn(`[os] invalid SEMIO_LOCKED_APPEARANCE ${JSON.stringify(locks.appearance)}, falling back to "system"`);
      resolved.appearance = "system";
    }
  }
  return resolved;
}

/** 🔒 Stable empty-locks reference so an omitted `locks` prop never busts memo dependency arrays. */
const EMPTY_SHELL_LOCKS: ResolvedShellLocks = {};

/** 🔒 Overlays env locks onto a brand's locks per key — a lock set by either source stays locked, an env value wins over the brand value. */
export function mergeShellLockSources(brandLocks: FrameworkOsLocks | undefined, envLocks: FrameworkOsLocks | undefined): FrameworkOsLocks | undefined {
  if (!brandLocks || !envLocks) return envLocks ?? brandLocks;
  return { ...brandLocks, ...Object.fromEntries(Object.entries(envLocks).filter(([, value]) => value !== undefined)) };
}

/** 🎛️ Boot-time default values that seed shell state without locking it — the matching in-app switcher stays visible, unlike locks. */
export type FrameworkOsDefaults = {
  readonly exampleId?: string;
};

/** 🎛️ Resolves boot defaults: an env-provided default wins over the brand's. */
export function resolveShellDefaults(brand: ShellBrand | undefined, defaults: FrameworkOsDefaults | undefined): FrameworkOsDefaults {
  return { exampleId: defaults?.exampleId ?? brand?.defaults?.exampleId };
}

/** 🎛️ Stable empty-defaults reference so an omitted `defaults` prop never busts memo dependency arrays. */
const EMPTY_SHELL_DEFAULTS: FrameworkOsDefaults = {};
//#endregion 🔒FrameworkOsLocks

type SyncCardKind = "file" | "folder" | "remote";

type UIHistoryEntry = { readonly uri: string };
type UIHistory = { readonly entries: readonly UIHistoryEntry[]; readonly index: number };
//#endregion 🔖types

//#region 🧮ShellStore
/** 🧮 Single consolidated `useReducer` state tree for `FrameworkOsShell`, replacing what used to be ~38 independent `useState` calls with one dispatch-driven store, grouped by concern. */

//#region slice shapes
type PluginRuntimeState = {
  readonly loadedPlugins: readonly LoadedPluginState[];
  readonly session: ActiveSession | null;
  readonly error: string | null;
};

type WindowUiState = {
  readonly windowUiByKind: Readonly<Record<string, UiNode>>;
  readonly windowEngagementsByKind: Readonly<Record<string, WindowEngagement>>;
  readonly windowMeasuresByKind: Readonly<Record<string, readonly WindowMeasure[]>>;
  readonly panelUiByKey: Readonly<Record<string, UiNode>>;
  readonly appLabelsOverlay: PluginAppLabelsOverlay;
};

type SpawnedWindowState = {
  readonly spawnedWindowUi: UiNode | null;
  readonly spawnedWindowEngagements: Readonly<Record<string, WindowEngagement>>;
  readonly spawnedWindowMeasures: Readonly<Record<string, readonly WindowMeasure[]>>;
};

/**
 * 🧰 Per-window Action rail (P1–P5) state: fold/expand chrome, locally-buffered staged arg values
 * (keyed `${windowId}:${actionId}`, never dispatched until Execute), and the host-owned active utility per
 * window (never a document field, never a VCS op). See {@link WindowActionPane}.
 */
type ActionPaneState = {
  readonly foldedByWindowId: Readonly<Record<string, boolean>>;
  readonly expandedByWindowId: Readonly<Record<string, string | null>>;
  readonly stagedArgsByKey: Readonly<Record<string, Readonly<Record<string, unknown>>>>;
  readonly activeUtilityByWindowId: Readonly<Record<string, string | null>>;
};

/** 🧰 Composite key into {@link ActionPaneState.stagedArgsByKey}. */
export function actionStageKey(windowId: string, actionId: string): string {
  return `${windowId}:${actionId}`;
}

type ExtraWindowInstance = { readonly id: string; readonly windowKindId: string; readonly title: string };

/** 🧭 Per-anchor fold/size/active-tab-path state for one of the six {@link Panel}s. */
type PanelState = {
  readonly visible: boolean;
  readonly size: number;
  readonly path: readonly string[];
};

type ShellLayoutState = {
  readonly panels: Record<PanelAnchor, PanelState>;
  /** 🗄️ User-rearranged dock diff against `defaultDock`, persisted via `DockLayoutStore`; `null` means "use the computed default arrangement". */
  readonly dockOverride: DockSkeleton | null;
  /** 🌱 Per-branch drill-down memory across every anchor + mobile (see `progressPanelTabSelection`), persisted via `DockUiStateStore`. */
  readonly panelPathMemory: Readonly<Record<string, string>>;
  /** 🌱 Persisted tree section/group expansion, namespaced per {@link PanelTreeUnit}, persisted via `DockUiStateStore`. */
  readonly treeOpenStates: Readonly<Record<string, boolean>>;
  readonly activeWindowId: string | null;
  readonly shellLayout: WindowLayoutNode | null;
  readonly activeExampleId: string;
  readonly mobilePanelPath: readonly string[];
  readonly extraWindowInstances: readonly ExtraWindowInstance[];
};

type OverlayState = {
  readonly searchOpen: boolean;
  readonly findOpen: boolean;
  /** 🎓 Current step of the active app's introduction walkthrough, or `null` when none is playing. */
  readonly introductionStepIndex: number | null;
  /** 🗨️ The open declared dialog (id + `HostEffect`-seeded args), or `null` when none is open. */
  readonly dialog: { readonly dialogId: string; readonly seedArgs?: Readonly<Record<string, unknown>> } | null;
};

type UiPrefsState = {
  readonly uiAppearance: ElementsSurfaceAppearance;
  readonly uiLayout: UiChromeLayout;
  readonly uiCompact: boolean;
  readonly uiExpertise: Expertise;
  readonly uiLocale: UiLocale;
  readonly uiTerminology: string;
  readonly uiThemeId: string;
  readonly uiCustomThemes: Record<string, UiTheme>;
  readonly uiThemeDraft: UiTheme | null;
};

type SyncState = {
  readonly syncBackboneUri: string | null;
  readonly syncCardKind: SyncCardKind | null;
  readonly syncDraftPath: string;
  /** 🚦 Per-document sync health fed by `backbone-worker.ts`'s `DocumentEvent::Status` events, keyed by `documentId`. */
  readonly syncStatusByDocumentId: Readonly<Record<string, DocumentSyncStatus>>;
};

/**
 * 🎛️ Command palette state not already covered by the generic per-anchor `Panel` state: the command
 * whose arg form is expanded one level above the command list (exclusive — only one at a time), and
 * locally-buffered staged arg values (never dispatched until Execute). Which category is active/folded
 * lives in `layout.panels["bottom-middle"]` like any other anchor — see `buildCommandCategoryTabs`.
 */
export type CommandPanelState = {
  readonly expandedCommandId: string | null;
  readonly stagedArgsByCommandId: Readonly<Record<string, Readonly<Record<string, unknown>>>>;
};

export type ShellState = {
  readonly pluginRuntime: PluginRuntimeState;
  readonly windowUi: WindowUiState;
  readonly spawnedWindow: SpawnedWindowState;
  readonly actionPane: ActionPaneState;
  readonly commandPanel: CommandPanelState;
  readonly layout: ShellLayoutState;
  readonly overlays: OverlayState;
  readonly uiPrefs: UiPrefsState;
  readonly sync: SyncState;
};
//#endregion slice shapes

//#region actions
/** 🌀 A `useState`-style value-or-updater payload, kept so every migrated `setXxx` call-site can dispatch its existing `value` or `(prev) => next` argument unchanged. */
type Updatable<T> = T | ((prev: T) => T);

const resolveUpdatable = <T,>(next: Updatable<T>, prev: T): T => (typeof next === "function" ? (next as (prev: T) => T)(prev) : next);

export type ShellAction =
  | { readonly type: "SET_LOADED_PLUGINS"; readonly value: Updatable<readonly LoadedPluginState[]> }
  | { readonly type: "SET_SESSION"; readonly value: Updatable<ActiveSession | null> }
  | { readonly type: "SET_ERROR"; readonly value: Updatable<string | null> }
  | { readonly type: "SET_WINDOW_UI_BY_KIND"; readonly value: Updatable<Readonly<Record<string, UiNode>>> }
  | { readonly type: "SET_WINDOW_ENGAGEMENTS_BY_KIND"; readonly value: Updatable<Readonly<Record<string, WindowEngagement>>> }
  | { readonly type: "SET_WINDOW_MEASURES_BY_KIND"; readonly value: Updatable<Readonly<Record<string, readonly WindowMeasure[]>>> }
  | { readonly type: "SET_PANEL_UI_BY_KEY"; readonly value: Updatable<Readonly<Record<string, UiNode>>> }
  | { readonly type: "SET_APP_LABELS_OVERLAY"; readonly value: Updatable<PluginAppLabelsOverlay> }
  | { readonly type: "SET_SPAWNED_WINDOW_UI"; readonly value: Updatable<UiNode | null> }
  | { readonly type: "SET_SPAWNED_WINDOW_ENGAGEMENTS"; readonly value: Updatable<Readonly<Record<string, WindowEngagement>>> }
  | { readonly type: "SET_SPAWNED_WINDOW_MEASURES"; readonly value: Updatable<Readonly<Record<string, readonly WindowMeasure[]>>> }
  | { readonly type: "SET_ACTION_PANE_FOLDED"; readonly windowId: string; readonly value: boolean }
  | { readonly type: "SET_ACTION_PANE_EXPANDED"; readonly windowId: string; readonly value: string | null }
  | { readonly type: "STAGE_ACTION_ARG"; readonly windowId: string; readonly actionId: string; readonly argId: string; readonly value: unknown }
  | { readonly type: "RESET_ACTION_ARGS"; readonly windowId: string; readonly actionId: string }
  | { readonly type: "SET_ACTIVE_UTILITY"; readonly windowId: string; readonly utilityId: string | null }
  | { readonly type: "SET_COMMAND_EXPANDED"; readonly value: string | null }
  | { readonly type: "STAGE_COMMAND_ARG"; readonly commandId: string; readonly argId: string; readonly value: unknown }
  | { readonly type: "RESET_COMMAND_ARGS"; readonly commandId: string }
  | { readonly type: "SET_PANEL_VISIBLE"; readonly anchor: PanelAnchor; readonly value: Updatable<boolean> }
  | { readonly type: "SET_PANEL_SIZE"; readonly anchor: PanelAnchor; readonly value: Updatable<number> }
  | { readonly type: "SET_PANEL_PATH"; readonly anchor: PanelAnchor; readonly value: Updatable<readonly string[]> }
  | { readonly type: "SET_DOCK_OVERRIDE"; readonly value: DockSkeleton | null }
  | { readonly type: "SET_PANEL_PATH_MEMORY"; readonly value: Updatable<Readonly<Record<string, string>>> }
  | { readonly type: "SET_TREE_OPEN_STATE"; readonly id: string; readonly open: boolean }
  | { readonly type: "HYDRATE_DOCK_UI"; readonly value: DockUiState | null }
  | { readonly type: "RESET_DOCK" }
  | { readonly type: "SET_ACTIVE_WINDOW_ID"; readonly value: Updatable<string | null> }
  | { readonly type: "SET_SHELL_LAYOUT"; readonly value: Updatable<WindowLayoutNode | null> }
  | { readonly type: "SET_ACTIVE_EXAMPLE_ID"; readonly value: Updatable<string> }
  | { readonly type: "SET_MOBILE_PANEL_PATH"; readonly value: Updatable<readonly string[]> }
  | { readonly type: "SET_EXTRA_WINDOW_INSTANCES"; readonly value: Updatable<readonly ExtraWindowInstance[]> }
  | { readonly type: "SET_SEARCH_OPEN"; readonly value: Updatable<boolean> }
  | { readonly type: "SET_FIND_OPEN"; readonly value: Updatable<boolean> }
  | { readonly type: "SET_INTRODUCTION_STEP"; readonly value: Updatable<number | null> }
  | { readonly type: "SET_DIALOG"; readonly value: OverlayState["dialog"] }
  | { readonly type: "SET_UI_APPEARANCE"; readonly value: Updatable<ElementsSurfaceAppearance> }
  | { readonly type: "SET_UI_LAYOUT"; readonly value: Updatable<UiChromeLayout> }
  | { readonly type: "SET_UI_COMPACT"; readonly value: Updatable<boolean> }
  | { readonly type: "SET_UI_EXPERTISE"; readonly value: Updatable<Expertise> }
  | { readonly type: "SET_UI_LOCALE"; readonly value: Updatable<UiLocale> }
  | { readonly type: "SET_UI_TERMINOLOGY"; readonly value: Updatable<string> }
  | { readonly type: "SET_UI_THEME_ID"; readonly value: Updatable<string> }
  | { readonly type: "SET_UI_CUSTOM_THEMES"; readonly value: Updatable<Record<string, UiTheme>> }
  | { readonly type: "SET_UI_THEME_DRAFT"; readonly value: Updatable<UiTheme | null> }
  | { readonly type: "SET_SYNC_BACKBONE_URI"; readonly value: Updatable<string | null> }
  | { readonly type: "SET_SYNC_CARD_KIND"; readonly value: Updatable<SyncCardKind | null> }
  | { readonly type: "SET_SYNC_DRAFT_PATH"; readonly value: Updatable<string> }
  | { readonly type: "SET_SYNC_STATUS_FOR_DOCUMENT"; readonly documentId: string; readonly status: DocumentSyncStatus };
//#endregion actions

//#region slice reducers
function pluginRuntimeReducer(state: PluginRuntimeState, action: ShellAction): PluginRuntimeState {
  switch (action.type) {
    case "SET_LOADED_PLUGINS":
      return { ...state, loadedPlugins: resolveUpdatable(action.value, state.loadedPlugins) };
    case "SET_SESSION":
      return { ...state, session: resolveUpdatable(action.value, state.session) };
    case "SET_ERROR":
      return { ...state, error: resolveUpdatable(action.value, state.error) };
    default:
      return state;
  }
}

function windowUiReducer(state: WindowUiState, action: ShellAction): WindowUiState {
  switch (action.type) {
    case "SET_WINDOW_UI_BY_KIND":
      return { ...state, windowUiByKind: resolveUpdatable(action.value, state.windowUiByKind) };
    case "SET_WINDOW_ENGAGEMENTS_BY_KIND":
      return { ...state, windowEngagementsByKind: resolveUpdatable(action.value, state.windowEngagementsByKind) };
    case "SET_WINDOW_MEASURES_BY_KIND":
      return { ...state, windowMeasuresByKind: resolveUpdatable(action.value, state.windowMeasuresByKind) };
    case "SET_PANEL_UI_BY_KEY":
      return { ...state, panelUiByKey: resolveUpdatable(action.value, state.panelUiByKey) };
    case "SET_APP_LABELS_OVERLAY":
      return { ...state, appLabelsOverlay: resolveUpdatable(action.value, state.appLabelsOverlay) };
    default:
      return state;
  }
}

function spawnedWindowReducer(state: SpawnedWindowState, action: ShellAction): SpawnedWindowState {
  switch (action.type) {
    case "SET_SPAWNED_WINDOW_UI":
      return { ...state, spawnedWindowUi: resolveUpdatable(action.value, state.spawnedWindowUi) };
    case "SET_SPAWNED_WINDOW_ENGAGEMENTS":
      return { ...state, spawnedWindowEngagements: resolveUpdatable(action.value, state.spawnedWindowEngagements) };
    case "SET_SPAWNED_WINDOW_MEASURES":
      return { ...state, spawnedWindowMeasures: resolveUpdatable(action.value, state.spawnedWindowMeasures) };
    default:
      return state;
  }
}

/** 🧰 Reducer for the per-window Action rail slice (P1–P5). Every case preserves referential identity when nothing actually changes so downstream memos can bail. */
function actionPaneReducer(state: ActionPaneState, action: ShellAction): ActionPaneState {
  switch (action.type) {
    case "SET_ACTION_PANE_FOLDED": {
      if (state.foldedByWindowId[action.windowId] === action.value) return state;
      return { ...state, foldedByWindowId: { ...state.foldedByWindowId, [action.windowId]: action.value } };
    }
    case "SET_ACTION_PANE_EXPANDED": {
      if ((state.expandedByWindowId[action.windowId] ?? null) === action.value) return state;
      return { ...state, expandedByWindowId: { ...state.expandedByWindowId, [action.windowId]: action.value } };
    }
    case "STAGE_ACTION_ARG": {
      const key = actionStageKey(action.windowId, action.actionId);
      const current = state.stagedArgsByKey[key] ?? {};
      if (Object.prototype.hasOwnProperty.call(current, action.argId) && current[action.argId] === action.value) return state;
      return { ...state, stagedArgsByKey: { ...state.stagedArgsByKey, [key]: { ...current, [action.argId]: action.value } } };
    }
    case "RESET_ACTION_ARGS": {
      const key = actionStageKey(action.windowId, action.actionId);
      if (!Object.prototype.hasOwnProperty.call(state.stagedArgsByKey, key)) return state;
      const next = { ...state.stagedArgsByKey };
      delete next[key];
      return { ...state, stagedArgsByKey: next };
    }
    case "SET_ACTIVE_UTILITY": {
      if ((state.activeUtilityByWindowId[action.windowId] ?? null) === action.utilityId) return state;
      return { ...state, activeUtilityByWindowId: { ...state.activeUtilityByWindowId, [action.windowId]: action.utilityId } };
    }
    default:
      return state;
  }
}

/** 🎛️ Reducer for the command palette's arg-expansion/staging slice — category active/fold state is the `bottom-middle` anchor's own generic `SET_PANEL_VISIBLE`/`SET_PANEL_PATH` (see `shellLayoutReducer`), not handled here. */
function commandPanelReducer(state: CommandPanelState, action: ShellAction): CommandPanelState {
  switch (action.type) {
    case "SET_COMMAND_EXPANDED": {
      if (state.expandedCommandId === action.value) return state;
      return { ...state, expandedCommandId: action.value };
    }
    case "STAGE_COMMAND_ARG": {
      const current = state.stagedArgsByCommandId[action.commandId] ?? {};
      if (Object.prototype.hasOwnProperty.call(current, action.argId) && current[action.argId] === action.value) return state;
      return { ...state, stagedArgsByCommandId: { ...state.stagedArgsByCommandId, [action.commandId]: { ...current, [action.argId]: action.value } } };
    }
    case "RESET_COMMAND_ARGS": {
      if (!Object.prototype.hasOwnProperty.call(state.stagedArgsByCommandId, action.commandId)) return state;
      const next = { ...state.stagedArgsByCommandId };
      delete next[action.commandId];
      return { ...state, stagedArgsByCommandId: next };
    }
    default:
      return state;
  }
}

function shellLayoutReducer(state: ShellLayoutState, action: ShellAction): ShellLayoutState {
  switch (action.type) {
    case "SET_PANEL_VISIBLE":
      return { ...state, panels: { ...state.panels, [action.anchor]: { ...state.panels[action.anchor], visible: resolveUpdatable(action.value, state.panels[action.anchor].visible) } } };
    case "SET_PANEL_SIZE":
      return { ...state, panels: { ...state.panels, [action.anchor]: { ...state.panels[action.anchor], size: resolveUpdatable(action.value, state.panels[action.anchor].size) } } };
    case "SET_PANEL_PATH":
      return { ...state, panels: { ...state.panels, [action.anchor]: { ...state.panels[action.anchor], path: resolveUpdatable(action.value, state.panels[action.anchor].path) } } };
    case "SET_DOCK_OVERRIDE":
      return { ...state, dockOverride: action.value };
    case "SET_PANEL_PATH_MEMORY":
      return { ...state, panelPathMemory: resolveUpdatable(action.value, state.panelPathMemory) };
    case "SET_TREE_OPEN_STATE":
      return { ...state, treeOpenStates: { ...state.treeOpenStates, [action.id]: action.open } };
    case "HYDRATE_DOCK_UI": {
      if (!action.value) return state;
      const panels = { ...state.panels };
      for (const anchor of PANEL_ANCHORS) {
        const saved = action.value.anchors[anchor];
        if (!saved) continue;
        panels[anchor] = {
          visible: saved.visible ?? panels[anchor].visible,
          size: saved.size ?? panels[anchor].size,
          path: saved.path ?? panels[anchor].path,
        };
      }
      return { ...state, panels, panelPathMemory: action.value.pathMemory ?? state.panelPathMemory, treeOpenStates: action.value.treeOpen ?? state.treeOpenStates };
    }
    case "RESET_DOCK": {
      const panels = {} as Record<PanelAnchor, PanelState>;
      for (const anchor of PANEL_ANCHORS) panels[anchor] = { visible: false, size: DEFAULT_PANEL_SIZES[anchor], path: [] };
      return { ...state, dockOverride: null, panels, panelPathMemory: {}, treeOpenStates: {} };
    }
    case "SET_ACTIVE_WINDOW_ID":
      return { ...state, activeWindowId: resolveUpdatable(action.value, state.activeWindowId) };
    case "SET_SHELL_LAYOUT":
      return { ...state, shellLayout: resolveUpdatable(action.value, state.shellLayout) };
    case "SET_ACTIVE_EXAMPLE_ID":
      return { ...state, activeExampleId: resolveUpdatable(action.value, state.activeExampleId) };
    case "SET_MOBILE_PANEL_PATH":
      return { ...state, mobilePanelPath: resolveUpdatable(action.value, state.mobilePanelPath) };
    case "SET_EXTRA_WINDOW_INSTANCES":
      return { ...state, extraWindowInstances: resolveUpdatable(action.value, state.extraWindowInstances) };
    default:
      return state;
  }
}

function overlayReducer(state: OverlayState, action: ShellAction): OverlayState {
  switch (action.type) {
    case "SET_SEARCH_OPEN":
      return { ...state, searchOpen: resolveUpdatable(action.value, state.searchOpen) };
    case "SET_FIND_OPEN":
      return { ...state, findOpen: resolveUpdatable(action.value, state.findOpen) };
    case "SET_INTRODUCTION_STEP":
      return { ...state, introductionStepIndex: resolveUpdatable(action.value, state.introductionStepIndex) };
    case "SET_DIALOG":
      return { ...state, dialog: action.value };
    default:
      return state;
  }
}

function uiPrefsReducer(state: UiPrefsState, action: ShellAction): UiPrefsState {
  switch (action.type) {
    case "SET_UI_APPEARANCE":
      return { ...state, uiAppearance: resolveUpdatable(action.value, state.uiAppearance) };
    case "SET_UI_LAYOUT":
      return { ...state, uiLayout: resolveUpdatable(action.value, state.uiLayout) };
    case "SET_UI_COMPACT":
      return { ...state, uiCompact: resolveUpdatable(action.value, state.uiCompact) };
    case "SET_UI_EXPERTISE":
      return { ...state, uiExpertise: resolveUpdatable(action.value, state.uiExpertise) };
    case "SET_UI_LOCALE":
      return { ...state, uiLocale: resolveUpdatable(action.value, state.uiLocale) };
    case "SET_UI_TERMINOLOGY":
      return { ...state, uiTerminology: resolveUpdatable(action.value, state.uiTerminology) };
    case "SET_UI_THEME_ID":
      return { ...state, uiThemeId: resolveUpdatable(action.value, state.uiThemeId) };
    case "SET_UI_CUSTOM_THEMES":
      return { ...state, uiCustomThemes: resolveUpdatable(action.value, state.uiCustomThemes) };
    case "SET_UI_THEME_DRAFT":
      return { ...state, uiThemeDraft: resolveUpdatable(action.value, state.uiThemeDraft) };
    default:
      return state;
  }
}

function syncReducer(state: SyncState, action: ShellAction): SyncState {
  switch (action.type) {
    case "SET_SYNC_BACKBONE_URI":
      return { ...state, syncBackboneUri: resolveUpdatable(action.value, state.syncBackboneUri) };
    case "SET_SYNC_CARD_KIND":
      return { ...state, syncCardKind: resolveUpdatable(action.value, state.syncCardKind) };
    case "SET_SYNC_DRAFT_PATH":
      return { ...state, syncDraftPath: resolveUpdatable(action.value, state.syncDraftPath) };
    case "SET_SYNC_STATUS_FOR_DOCUMENT":
      return { ...state, syncStatusByDocumentId: { ...state.syncStatusByDocumentId, [action.documentId]: action.status } };
    default:
      return state;
  }
}
//#endregion slice reducers

/** 🧵 Root reducer for `FrameworkOsShell` — fans every action out to its owning slice reducer; slices that ignore an action's type return their input unchanged, so unrelated slices keep referential identity. */
export function shellReducer(state: ShellState, action: ShellAction): ShellState {
  return {
    pluginRuntime: pluginRuntimeReducer(state.pluginRuntime, action),
    windowUi: windowUiReducer(state.windowUi, action),
    spawnedWindow: spawnedWindowReducer(state.spawnedWindow, action),
    actionPane: actionPaneReducer(state.actionPane, action),
    commandPanel: commandPanelReducer(state.commandPanel, action),
    layout: shellLayoutReducer(state.layout, action),
    overlays: overlayReducer(state.overlays, action),
    uiPrefs: uiPrefsReducer(state.uiPrefs, action),
    sync: syncReducer(state.sync, action),
  };
}

//#region selectors
export const selectUiDevice = (state: ShellState, mobile: boolean): ElementsSurfaceDevice => (mobile ? "mobile" : state.uiPrefs.uiLayout);
//#endregion selectors

/** 🌱 Builds the starting `ShellState` for `FrameworkOsShell`, mirroring exactly what each migrated `useState` used to initialize to (including reads from local storage for UI prefs). */
export function initialShellState(_props: { readonly pluginFilter?: string; readonly plugins: readonly { readonly pluginId: string; readonly moduleUrl: string }[]; readonly locks?: ResolvedShellLocks; readonly defaults?: FrameworkOsDefaults }): ShellState {
  const locks = _props.locks ?? {};
  const defaults = _props.defaults ?? {};
  return {
    pluginRuntime: { loadedPlugins: [], session: null, error: null },
    windowUi: { windowUiByKind: {}, windowEngagementsByKind: {}, windowMeasuresByKind: {}, panelUiByKey: {}, appLabelsOverlay: EMPTY_APP_LABELS_OVERLAY },
    spawnedWindow: { spawnedWindowUi: null, spawnedWindowEngagements: {}, spawnedWindowMeasures: {} },
    actionPane: { foldedByWindowId: {}, expandedByWindowId: {}, stagedArgsByKey: {}, activeUtilityByWindowId: {} },
    commandPanel: { expandedCommandId: null, stagedArgsByCommandId: {} },
    layout: {
      panels: Object.fromEntries(PANEL_ANCHORS.map((anchor) => [anchor, { visible: false, size: DEFAULT_PANEL_SIZES[anchor], path: [] }])) as Record<PanelAnchor, PanelState>,
      dockOverride: null,
      panelPathMemory: {},
      treeOpenStates: {},
      activeWindowId: null,
      shellLayout: null,
      activeExampleId: locks.exampleId ?? defaults.exampleId ?? "",
      mobilePanelPath: [],
      extraWindowInstances: [],
    },
    overlays: { searchOpen: false, findOpen: false, introductionStepIndex: null, dialog: null },
    uiPrefs: {
      uiAppearance: locks.appearance ?? readStoredUiChromeAppearance(),
      uiLayout: readStoredUiChromeLayout(),
      uiCompact: readStoredUiChromeCompact(),
      uiExpertise: readStoredUiChromeExpertise(),
      uiLocale: locks.locale ?? readStoredUiChromeLocale() ?? (uiI18n.resolvedLanguage?.toLowerCase().startsWith("de") ? "de" : "en"),
      uiTerminology: locks.terminology ?? readStoredUiChromeTerminology(),
      uiThemeId: locks.themeId ?? readStoredUiChromeThemeId() ?? "semio",
      uiCustomThemes: readStoredUiCustomThemes(),
      uiThemeDraft: null,
    },
    sync: { syncBackboneUri: null, syncCardKind: null, syncDraftPath: "", syncStatusByDocumentId: {} },
  };
}
//#endregion 🧮ShellStore

//#region ShellHelpers
function syncDocumentId(session: ActiveSession, panel: StudioPanelState | null, studioMode: boolean): string {
  if (studioMode && panel?.activeSpawnedId) {
    const spawned = panel.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId);
    if (spawned) return `${spawned.pluginId}-${spawned.instanceId}`;
  }
  return `${session.pluginId}-${session.instanceId}`;
}

/** @emoji 🧭 Starting width for each panel anchor — `top-left`/`top-right` mirror the old left/right side-panel defaults; `bottom-left`/`bottom-right` host the sync card and a compact utility tree, so a narrower default suits them; the middle anchors start empty but default wider since they grow both ways and tend to host transient centered content (e.g. search). */
const DEFAULT_PANEL_SIZES: Record<PanelAnchor, number> = {
  "top-left": 280,
  "top-middle": 360,
  "top-right": 320,
  "bottom-left": 240,
  "bottom-middle": 360,
  "bottom-right": 240,
};

/** @emoji 🌳 Root category id for the nested dock tab tree — the top row of {@link defaultDock}'s bottom-left (Display) anchor tabs; top-left (Workbench), top-right (Details) and bottom-right (Settings) render their tabs flat instead of under a category branch. */
const FRAMEWORK_CATEGORY_DISPLAY_ID = "framework.category.display";
/** @emoji 🎛 Root category id bundling every command-category leaf under one expandable Command toggle on bottom-middle (mirrors Display on bottom-left). */
const FRAMEWORK_CATEGORY_COMMAND_ID = "framework.category.command";

/** @emoji 🎛️ Every anchor's root tab row renders inline in navbar/footer chrome (via {@link PanelChromeTabBar}) instead of on the floating panel — the single source of truth `buildPanelSelectionProps`/`buildPanelProps` key off of. */
const PANEL_TAB_BAR_HOSTS: Record<PanelAnchor, "navbar" | "footer"> = {
  "top-left": "navbar",
  "top-middle": "navbar",
  "top-right": "navbar",
  "bottom-left": "footer",
  "bottom-middle": "footer",
  "bottom-right": "footer",
};
const APP_DOCUMENT_SEPARATOR = " · ";

const PRESENCE_CLIENT_STORAGE_KEY = "semio.presence.client";
const PRESENCE_HEARTBEAT_INTERVAL_MS = 5000;

function presenceClientIdentity(): { readonly clientId: string; readonly name: string } {
  if (typeof window === "undefined") return { clientId: "server", name: "Server" };
  const stored = window.sessionStorage.getItem(PRESENCE_CLIENT_STORAGE_KEY);
  if (stored) {
    try {
      const parsed = JSON.parse(stored) as { readonly clientId?: string; readonly name?: string };
      if (parsed.clientId && parsed.name) return { clientId: parsed.clientId, name: parsed.name };
    } catch {
      /* reseed identity */
    }
  }
  const clientId = `client-${Math.random().toString(36).slice(2, 10)}`;
  const identity = { clientId, name: `Guest ${clientId.slice(-4).toUpperCase()}` };
  window.sessionStorage.setItem(PRESENCE_CLIENT_STORAGE_KEY, JSON.stringify(identity));
  return identity;
}

function readBrowserUri(): string {
  if (typeof window === "undefined") return "/";
  return `${window.location.pathname}${window.location.search}` || "/";
}

function useUIHistory(initialUri = "/", syncBrowser = false) {
  const [history, setHistory] = useState<UIHistory>(() => ({
    entries: [{ uri: syncBrowser ? readBrowserUri() : initialUri }],
    index: 0,
  }));
  const uri = history.entries[history.index]?.uri ?? initialUri;
  const canGoBack = history.index > 0;
  const canGoForward = history.index < history.entries.length - 1;
  const segments = uri.split("/").filter(Boolean);
  const canGoUp = segments.length > 0;
  const parentUri = canGoUp ? `/${segments.slice(0, -1).join("/")}` : null;

  const goBack = useCallback(() => {
    setHistory((prev) => (prev.index > 0 ? { ...prev, index: prev.index - 1 } : prev));
  }, []);
  const goForward = useCallback(() => {
    setHistory((prev) => (prev.index < prev.entries.length - 1 ? { ...prev, index: prev.index + 1 } : prev));
  }, []);
  const goUp = useCallback(() => {
    if (!canGoUp || parentUri === null) return;
    setHistory((prev) => {
      const newEntries = prev.entries.slice(0, prev.index + 1);
      return { entries: [...newEntries, { uri: parentUri }], index: newEntries.length };
    });
  }, [canGoUp, parentUri]);
  const navigate = useCallback((targetUri: string) => {
    setHistory((prev) => {
      const existingIndex = prev.entries.findIndex((entry) => entry.uri === targetUri);
      if (existingIndex >= 0) return { ...prev, index: existingIndex };
      const newEntries = prev.entries.slice(0, prev.index + 1);
      return { entries: [...newEntries, { uri: targetUri }], index: newEntries.length };
    });
  }, []);
  const syncUri = useCallback((targetUri: string) => {
    setHistory((prev) => {
      const existingIndex = prev.entries.findIndex((entry) => entry.uri === targetUri);
      if (existingIndex >= 0) return { ...prev, index: existingIndex };
      const newEntries = prev.entries.slice(0, prev.index + 1);
      return { entries: [...newEntries, { uri: targetUri }], index: newEntries.length };
    });
  }, []);

  useEffect(() => {
    if (!syncBrowser || typeof window === "undefined") return;
    const current = `${window.location.pathname}${window.location.search}`;
    if (current !== uri) window.history.pushState(null, "", uri);
  }, [syncBrowser, uri]);

  useEffect(() => {
    if (!syncBrowser || typeof window === "undefined") return;
    const onPopState = () => syncUri(readBrowserUri());
    window.addEventListener("popstate", onPopState);
    return () => window.removeEventListener("popstate", onPopState);
  }, [syncBrowser, syncUri]);

  return { uri, canGoBack, canGoForward, canGoUp, parentUri, goBack, goForward, goUp, navigate, syncUri };
}

function downloadMediaExport(filename: string, mimeType: string, data: string, encoding?: string): void {
  if (typeof document === "undefined") return;
  const payload = encoding === "base64" ? Uint8Array.from(atob(data), (char) => char.charCodeAt(0)) : data;
  const blob = new Blob([payload], { type: mimeType });
  const url = URL.createObjectURL(blob);
  const anchor = document.createElement("a");
  anchor.href = url;
  anchor.download = filename;
  anchor.click();
  URL.revokeObjectURL(url);
}

function downloadDataUrl(filename: string, dataUrl: string): void {
  if (typeof document === "undefined") return;
  const anchor = document.createElement("a");
  anchor.href = dataUrl;
  anchor.download = filename;
  anchor.click();
}

/** 📤 Opens the native file picker. Resolves with one entry per selected file, in selection order —
 * always an array (empty on cancel) so single-file callers just read `[0]` and `multiple` callers can
 * fan out over the whole list; single-file behavior (one `<input>`, one resolved entry) is unchanged
 * when `multiple` is false/absent. */
function requestFileOpen(accept: string, readAs?: string, multiple?: boolean): Promise<readonly { contents: string; name: string }[]> {
  if (typeof document === "undefined") return Promise.resolve([]);
  return new Promise((resolve) => {
    const input = document.createElement("input");
    input.type = "file";
    input.accept = accept;
    if (multiple) input.multiple = true;
    input.onchange = async () => {
      const files = input.files ? Array.from(input.files) : [];
      if (files.length === 0) {
        resolve([]);
        return;
      }
      const opened: { contents: string; name: string }[] = [];
      for (const file of files) {
        if (readAs === "dataUrl") {
          const contents = await new Promise<string | null>((resolveFile) => {
            const reader = new FileReader();
            reader.onload = () => resolveFile(typeof reader.result === "string" ? reader.result : null);
            reader.onerror = () => resolveFile(null);
            reader.readAsDataURL(file);
          });
          if (contents !== null) opened.push({ contents, name: file.name });
          continue;
        }
        opened.push({ contents: await file.text(), name: file.name });
      }
      resolve(opened);
    };
    input.click();
  });
}

/** 🔁 The one-action-at-a-time callback shared by the `requestFileOpen`/`dispatchAction`/
 * `requestMediaFrames` `applyHostEffects` branches: dispatches `action` against the emitting plugin
 * instance and feeds its own `requestedEffects` back through `applyHostEffects` recursively. */
type EffectDispatchOne = (action: string, args?: Record<string, unknown>) => Promise<void>;

/** 🔁 Builds an {@link EffectDispatchOne} bound to one plugin instance + `applyHostEffects` closure —
 * extracted so the D3/D2/D5 fan-out loops below are plain functions testable without React/plugin
 * wiring, while production callers get the exact same `handleAction` + recursive-effects behavior. */
function makeEffectDispatchOne(
  pluginEntry: LoadedPluginState,
  baseSession: ActiveSession,
  applyEffects: (effects: readonly HostEffect[], baseSession: ActiveSession, uiScope?: UiDirtyScope) => Promise<void>,
): EffectDispatchOne {
  return async (action, args) => {
    const response = await pluginEntry.handle.handleAction(
      baseSession.instanceId,
      JSON.stringify({ controllerId: baseSession.app.controllerId, action, args }),
      baseSession.viewState,
    );
    await applyEffects(response.requestedEffects ?? [], baseSession, resolveUiDirtyScope(response.uiScope));
  };
}

/** 📤 D3 fan-out: one {@link EffectDispatchOne} call per opened file — single-file behavior (`multiple`
 * absent/false, exactly one call, plain `{payload, name}`) is byte-for-byte what this loop always did
 * before `multiple` existed, since it's just a one-entry `opened` array through the same path. */
export async function dispatchOpenedFiles(
  opened: readonly { readonly contents: string; readonly name: string }[],
  importAction: string,
  multiple: boolean,
  dispatchOne: EffectDispatchOne,
): Promise<void> {
  const total = opened.length;
  for (let index = 0; index < opened.length; index += 1) {
    const file = opened[index]!;
    await dispatchOne(importAction, multiple ? { payload: file.contents, name: file.name, index, total } : { payload: file.contents, name: file.name });
  }
}

/** 🔁 D2: schedules `action` onto `dispatchOne` after `delayMs` (0 = next tick) via `schedule` (real
 * callers pass `setTimeout`; tests pass `vi.useFakeTimers()`-driven `setTimeout` or a synchronous stub). */
export function scheduleDispatchAction(
  action: string,
  args: Record<string, unknown> | undefined,
  delayMs: number,
  dispatchOne: EffectDispatchOne,
  schedule: (fn: () => void, delayMs: number) => void = (fn, ms) => setTimeout(fn, ms),
): void {
  schedule(() => {
    void dispatchOne(action, args);
  }, delayMs);
}

//#region RequestMediaFrames
//#region Bmff
/** 🧱 One parsed ISO-BMFF box: `[type, payloadStart, payloadEnd)` — enough to recurse into containers
 * and slice leaf payloads without copying. */
type BmffBox = { readonly type: string; readonly start: number; readonly end: number };

/** 🧱 Walks sibling boxes in `[start, end)` — handles 64-bit extended sizes (`size===1`) and to-end
 * boxes (`size===0`); malformed/truncated input just stops early rather than throwing, since MP4
 * probing here is best-effort — the Tier-2 `<video>` fallback covers anything this can't parse. */
function walkBmffBoxes(view: DataView, start: number, end: number): BmffBox[] {
  const boxes: BmffBox[] = [];
  let offset = start;
  while (offset + 8 <= end) {
    const size32 = view.getUint32(offset);
    const type = String.fromCharCode(view.getUint8(offset + 4), view.getUint8(offset + 5), view.getUint8(offset + 6), view.getUint8(offset + 7));
    let headerSize = 8;
    let boxSize = size32;
    if (size32 === 1) {
      if (offset + 16 > end) break;
      boxSize = Number(view.getBigUint64(offset + 8));
      headerSize = 16;
    } else if (size32 === 0) {
      boxSize = end - offset;
    }
    if (boxSize < headerSize || offset + boxSize > end) break;
    boxes.push({ type, start: offset + headerSize, end: offset + boxSize });
    offset += boxSize;
  }
  return boxes;
}

function findBmffBox(boxes: readonly BmffBox[], type: string): BmffBox | undefined {
  return boxes.find((box) => box.type === type);
}
//#endregion Bmff

//#region Tier1
type Mp4Sample = { readonly offset: number; readonly size: number; readonly timestampMs: number; readonly isSync: boolean };
type Mp4Track = {
  readonly width: number;
  readonly height: number;
  readonly codec: "avc1" | "hvc1";
  readonly description: Uint8Array;
  readonly samples: readonly Mp4Sample[];
};

/** 🎞️ Minimal MP4 sample-table extraction — `moov > trak[] > mdia > {mdhd, hdlr, minf > stbl}` for the
 * first video track (`hdlr`'s handler-type `"vide"`), enough to feed `VideoDecoder`: sample byte ranges
 * from `stsc` + `stco`/`co64` + `stsz`, decode timestamps from `stts`, sync flags from `stss` (absent
 * `stss` ⇒ every sample is sync per spec), and the AVC/HEVC decoder config from `stsd`'s `avcC`/`hvcC`.
 * Returns `null` for anything unrecognized (non-AVC/HEVC, missing boxes, malformed tables) so the
 * caller falls back to Tier 2 rather than guessing. */
function probeMp4VideoTrack(bytes: Uint8Array): Mp4Track | null {
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  const moov = findBmffBox(walkBmffBoxes(view, 0, bytes.byteLength), "moov");
  if (!moov) return null;
  for (const trak of walkBmffBoxes(view, moov.start, moov.end).filter((box) => box.type === "trak")) {
    const mdia = findBmffBox(walkBmffBoxes(view, trak.start, trak.end), "mdia");
    if (!mdia) continue;
    const mdiaBoxes = walkBmffBoxes(view, mdia.start, mdia.end);
    const hdlr = findBmffBox(mdiaBoxes, "hdlr");
    if (!hdlr || hdlr.end - hdlr.start < 12) continue;
    const handlerType = String.fromCharCode(view.getUint8(hdlr.start + 8), view.getUint8(hdlr.start + 9), view.getUint8(hdlr.start + 10), view.getUint8(hdlr.start + 11));
    if (handlerType !== "vide") continue;
    const mdhd = findBmffBox(mdiaBoxes, "mdhd");
    const minf = findBmffBox(mdiaBoxes, "minf");
    if (!mdhd || !minf) continue;
    const timescale = view.getUint8(mdhd.start) === 1 ? view.getUint32(mdhd.start + 20) : view.getUint32(mdhd.start + 12);
    if (timescale <= 0) continue;
    const stbl = findBmffBox(walkBmffBoxes(view, minf.start, minf.end), "stbl");
    if (!stbl) continue;
    const track = probeSampleTable(view, walkBmffBoxes(view, stbl.start, stbl.end), timescale);
    if (track) return track;
  }
  return null;
}

function parseStsd(view: DataView, stsd: BmffBox): { width: number; height: number; codec: "avc1" | "hvc1"; description: Uint8Array } | null {
  if (view.getUint32(stsd.start + 4) < 1) return null;
  const entryOffset = stsd.start + 8;
  const entrySize = view.getUint32(entryOffset);
  const format = String.fromCharCode(
    view.getUint8(entryOffset + 4),
    view.getUint8(entryOffset + 5),
    view.getUint8(entryOffset + 6),
    view.getUint8(entryOffset + 7),
  );
  if (format !== "avc1" && format !== "hvc1" && format !== "hev1") return null;
  const codec = format === "avc1" ? "avc1" : "hvc1";
  const visualEntryStart = entryOffset + 8;
  const width = view.getUint16(visualEntryStart + 24);
  const height = view.getUint16(visualEntryStart + 26);
  const inner = walkBmffBoxes(view, visualEntryStart + 78, entryOffset + entrySize);
  const config = findBmffBox(inner, codec === "avc1" ? "avcC" : "hvcC");
  if (!config) return null;
  return { width, height, codec, description: new Uint8Array(view.buffer.slice(config.start, config.end)) };
}

function parseStsz(view: DataView, box: BmffBox): number[] {
  const uniformSize = view.getUint32(box.start + 4);
  const sampleCount = view.getUint32(box.start + 8);
  if (uniformSize !== 0) return new Array(sampleCount).fill(uniformSize) as number[];
  const sizes: number[] = [];
  for (let i = 0; i < sampleCount; i += 1) sizes.push(view.getUint32(box.start + 12 + i * 4));
  return sizes;
}

function parseChunkOffsets(view: DataView, box: BmffBox, is64: boolean): number[] {
  const count = view.getUint32(box.start + 4);
  const offsets: number[] = [];
  for (let i = 0; i < count; i += 1) {
    offsets.push(is64 ? Number(view.getBigUint64(box.start + 8 + i * 8)) : view.getUint32(box.start + 8 + i * 4));
  }
  return offsets;
}

function parseChunkOfSample(view: DataView, box: BmffBox, sampleCount: number, chunkCount: number): number[] | null {
  const entryCount = view.getUint32(box.start + 4);
  const entries: { firstChunk: number; samplesPerChunk: number }[] = [];
  for (let i = 0; i < entryCount; i += 1) {
    entries.push({ firstChunk: view.getUint32(box.start + 8 + i * 12), samplesPerChunk: view.getUint32(box.start + 12 + i * 12) });
  }
  const chunkOfSample: number[] = [];
  for (let entryIndex = 0; entryIndex < entries.length; entryIndex += 1) {
    const entry = entries[entryIndex]!;
    const nextFirstChunk = entries[entryIndex + 1]?.firstChunk ?? chunkCount + 1;
    for (let chunk = entry.firstChunk; chunk < nextFirstChunk; chunk += 1) {
      for (let inChunk = 0; inChunk < entry.samplesPerChunk; inChunk += 1) chunkOfSample.push(chunk);
    }
  }
  return chunkOfSample.length >= sampleCount ? chunkOfSample : null;
}

function computeSampleOffsets(chunkOfSample: readonly number[], chunkOffsets: readonly number[], sizes: readonly number[]): number[] {
  const offsets: number[] = [];
  const cursorByChunk = new Map<number, number>();
  for (let i = 0; i < sizes.length; i += 1) {
    const chunk = chunkOfSample[i]!;
    const base = cursorByChunk.get(chunk) ?? chunkOffsets[chunk - 1] ?? 0;
    offsets.push(base);
    cursorByChunk.set(chunk, base + sizes[i]!);
  }
  return offsets;
}

function accumulateTimestampsMs(view: DataView, stts: BmffBox, sampleCount: number, timescale: number): number[] {
  const entryCount = view.getUint32(stts.start + 4);
  const timestamps: number[] = [];
  let ticks = 0;
  for (let entryIndex = 0; entryIndex < entryCount && timestamps.length < sampleCount; entryIndex += 1) {
    const count = view.getUint32(stts.start + 8 + entryIndex * 8);
    const delta = view.getUint32(stts.start + 12 + entryIndex * 8);
    for (let i = 0; i < count && timestamps.length < sampleCount; i += 1) {
      timestamps.push((ticks / timescale) * 1000);
      ticks += delta;
    }
  }
  return timestamps;
}

function parseSyncSamples(view: DataView, box: BmffBox): Set<number> {
  const count = view.getUint32(box.start + 4);
  const sync = new Set<number>();
  for (let i = 0; i < count; i += 1) sync.add(view.getUint32(box.start + 8 + i * 4));
  return sync;
}

function probeSampleTable(view: DataView, stblBoxes: readonly BmffBox[], timescale: number): Mp4Track | null {
  const stsd = findBmffBox(stblBoxes, "stsd");
  const stts = findBmffBox(stblBoxes, "stts");
  const stsc = findBmffBox(stblBoxes, "stsc");
  const stsz = findBmffBox(stblBoxes, "stsz");
  const stco = findBmffBox(stblBoxes, "stco") ?? findBmffBox(stblBoxes, "co64");
  if (!stsd || !stts || !stsc || !stsz || !stco) return null;
  const entry = parseStsd(view, stsd);
  if (!entry) return null;
  const sizes = parseStsz(view, stsz);
  const offsets = parseChunkOffsets(view, stco, stco.type === "co64");
  const chunkOfSample = parseChunkOfSample(view, stsc, sizes.length, offsets.length);
  if (!chunkOfSample) return null;
  const sampleOffsets = computeSampleOffsets(chunkOfSample, offsets, sizes);
  const timestampsMs = accumulateTimestampsMs(view, stts, sizes.length, timescale);
  const stss = findBmffBox(stblBoxes, "stss");
  const syncSamples = stss ? parseSyncSamples(view, stss) : null;
  const samples: Mp4Sample[] = sizes.map((size, index) => ({
    offset: sampleOffsets[index]!,
    size,
    timestampMs: timestampsMs[index] ?? 0,
    isSync: syncSamples ? syncSamples.has(index + 1) : true,
  }));
  return { width: entry.width, height: entry.height, codec: entry.codec, description: entry.description, samples };
}

/** 🌐 Feature-detects the WebCodecs `VideoDecoder`/`EncodedVideoChunk` globals (Tier 1's prerequisite;
 * absent in most JS test environments and in browsers that only support WebM/VP9 without an AVC path). */
function webCodecsAvailable(): boolean {
  const scope = window as unknown as { VideoDecoder?: unknown; EncodedVideoChunk?: unknown };
  return typeof scope.VideoDecoder === "function" && typeof scope.EncodedVideoChunk === "function";
}

/** 🔢 Derives a WebCodecs `avc1.PPCCLL` codec string from an `avcC` box's profile/compat/level bytes
 * (offsets 1/2/3 — version is byte 0). */
function avcCodecString(description: Uint8Array): string {
  const hex = (byte: number | undefined) => (byte ?? 0).toString(16).padStart(2, "0");
  return `avc1.${hex(description[1])}${hex(description[2])}${hex(description[3])}`;
}

type WebCodecsVideoFrame = { readonly codedWidth: number; readonly codedHeight: number; close: () => void };
type WebCodecsVideoDecoderCtor = new (init: { output: (frame: WebCodecsVideoFrame) => void; error: (error: unknown) => void }) => {
  configure: (config: { codec: string; codedWidth: number; codedHeight: number; description: Uint8Array }) => void;
  decode: (chunk: unknown) => void;
  flush: () => Promise<void>;
  close: () => void;
};
type WebCodecsEncodedVideoChunkCtor = new (init: { type: "key" | "delta"; timestamp: number; data: Uint8Array }) => unknown;

function jpegDataUrlFromFrame(frame: WebCodecsVideoFrame): { readonly dataUrl: string; readonly width: number; readonly height: number } {
  const canvas = document.createElement("canvas");
  canvas.width = frame.codedWidth;
  canvas.height = frame.codedHeight;
  canvas.getContext("2d")?.drawImage(frame as unknown as CanvasImageSource, 0, 0);
  return { dataUrl: canvas.toDataURL("image/jpeg", 0.9), width: frame.codedWidth, height: frame.codedHeight };
}

/** 🎞️ Decodes exactly the samples needed for one target frame — from its nearest preceding sync sample
 * through the target — via a fresh `VideoDecoder`, capturing only the last output frame. Simplification:
 * each target frame re-decodes its GOP prefix from scratch instead of streaming continuously across
 * targets and demuxing outputs by timestamp; acceptable because sampled ingestion (`sampleStride`/
 * `maxFrames`) keeps GOP prefixes short between targets, and Tier 2's `<video>` element is always the
 * correctness fallback if Tier 1 fails or the codec isn't baseline-friendly. */
async function decodeOneMp4Frame(track: Mp4Track, bytes: Uint8Array, targetIndex: number): Promise<{ dataUrl: string; width: number; height: number } | null> {
  const scope = window as unknown as { VideoDecoder: WebCodecsVideoDecoderCtor; EncodedVideoChunk: WebCodecsEncodedVideoChunkCtor };
  let syncIndex = targetIndex;
  while (syncIndex > 0 && !track.samples[syncIndex]!.isSync) syncIndex -= 1;
  let captured: { dataUrl: string; width: number; height: number } | null = null;
  await new Promise<void>((resolve, reject) => {
    const decoder = new scope.VideoDecoder({
      output: (frame) => {
        captured = jpegDataUrlFromFrame(frame);
        frame.close();
      },
      error: reject,
    });
    decoder.configure({ codec: avcCodecString(track.description), codedWidth: track.width, codedHeight: track.height, description: track.description });
    for (let i = syncIndex; i <= targetIndex; i += 1) {
      const sample = track.samples[i]!;
      decoder.decode(
        new scope.EncodedVideoChunk({ type: sample.isSync ? "key" : "delta", timestamp: sample.timestampMs * 1000, data: bytes.subarray(sample.offset, sample.offset + sample.size) }),
      );
    }
    decoder.flush().then(() => {
      decoder.close();
      resolve();
    }, reject);
  });
  return captured;
}

/** 🎞️ Tier 1 orchestration: demuxes `bytes` as MP4/AVC, decodes one frame per sampled timestamp, and
 * dispatches `frameAction` per frame + `doneAction` once. Returns `false` (no dispatch performed at
 * all) when the demux can't find a usable AVC video track, so the caller falls through to Tier 2. */
async function runTier1VideoFrames(bytes: Uint8Array, effect: RequestMediaFramesArgs, name: string, dispatchOne: EffectDispatchOne): Promise<boolean> {
  const track = probeMp4VideoTrack(bytes);
  if (!track || track.samples.length === 0) return false;
  const durationMs = track.samples[track.samples.length - 1]!.timestampMs;
  const timestamps = sampleMediaFrameTimestampsMs(durationMs, effect.sampleStride, effect.maxFrames, effect.fpsHint);
  let sampledCount = 0;
  for (let index = 0; index < timestamps.length; index += 1) {
    const targetMs = timestamps[index]!;
    let targetSampleIndex = 0;
    for (let i = 0; i < track.samples.length; i += 1) if (track.samples[i]!.timestampMs <= targetMs) targetSampleIndex = i;
    const frame = await decodeOneMp4Frame(track, bytes, targetSampleIndex);
    if (!frame) continue;
    sampledCount += 1;
    await dispatchOne(effect.frameAction, {
      payload: frame.dataUrl,
      name,
      frameIndex: index,
      timestampMs: targetMs,
      index,
      total: timestamps.length,
      width: frame.width,
      height: frame.height,
      ...effect.args,
    });
  }
  await dispatchOne(effect.doneAction, {
    name,
    durationMs,
    frameCount: track.samples.length,
    sampledCount,
    width: track.width,
    height: track.height,
    codec: track.codec,
    ...effect.args,
  });
  return true;
}
//#endregion Tier1

//#region Tier2
/** ⏱️ Tier-2 (`<video>` seek-and-capture) target timestamps, ms — one every `sampleStride /
 * (fpsHint || 30)` seconds starting at 0, capped at `maxFrames` (0 ⇒ unlimited, bounded only by
 * `durationMs`). Pure/deterministic so it's unit-testable without any DOM or media APIs. Computes each
 * timestamp as `k * stepMs` rather than an accumulating `ts += stepMs` loop — repeated float addition
 * drifts enough over dozens of steps to occasionally land just under an exact multiple of `durationMs`,
 * sneaking in one extra timestamp; multiplying from the loop index is exact per-step and deterministic. */
export function sampleMediaFrameTimestampsMs(durationMs: number, sampleStride: number, maxFrames: number, fpsHint: number): number[] {
  const stride = sampleStride > 0 ? sampleStride : 1;
  const fps = fpsHint > 0 ? fpsHint : 30;
  const stepMs = (stride / fps) * 1000;
  const timestamps: number[] = [];
  if (durationMs <= 0 || stepMs <= 0) return timestamps;
  for (let k = 0; ; k += 1) {
    if (maxFrames > 0 && timestamps.length >= maxFrames) break;
    const ts = k * stepMs;
    if (ts >= durationMs) break;
    timestamps.push(ts);
  }
  return timestamps;
}

function captureCanvasFrame(video: HTMLVideoElement, maxLongEdgePx: number): { readonly dataUrl: string; readonly width: number; readonly height: number } {
  const sourceWidth = video.videoWidth || 0;
  const sourceHeight = video.videoHeight || 0;
  const scale = maxLongEdgePx > 0 ? Math.min(1, maxLongEdgePx / Math.max(sourceWidth, sourceHeight, 1)) : 1;
  const width = Math.max(1, Math.round(sourceWidth * scale));
  const height = Math.max(1, Math.round(sourceHeight * scale));
  const canvas = document.createElement("canvas");
  canvas.width = width;
  canvas.height = height;
  canvas.getContext("2d")?.drawImage(video, 0, 0, width, height);
  return { dataUrl: canvas.toDataURL("image/jpeg", 0.9), width, height };
}

function waitForVideoEvent(video: HTMLVideoElement, type: string): Promise<void> {
  return new Promise((resolve) => {
    const handler = () => {
      video.removeEventListener(type, handler);
      resolve();
    };
    video.addEventListener(type, handler);
  });
}

/** 🎞️ Tier 2 orchestration: waits for `loadedmetadata` (if not already available), seeks `video`
 * through {@link sampleMediaFrameTimestampsMs}'s schedule, captures each landed frame to a scaled JPEG
 * data URL, dispatches `frameAction` per frame, then `doneAction` once. Used both as the WebM/no-
 * WebCodecs fallback and directly by tests (which inject a real `<video>` element with overridden
 * `duration`/`videoWidth`/`videoHeight`/`readyState` and manually dispatch `loadedmetadata`/`seeked`,
 * since headless test environments have no real media decoder). */
export async function runTier2VideoFrames(video: HTMLVideoElement, effect: RequestMediaFramesArgs, name: string, dispatchOne: EffectDispatchOne): Promise<void> {
  if (video.readyState < 1) await waitForVideoEvent(video, "loadedmetadata");
  const durationMs = Number.isFinite(video.duration) ? video.duration * 1000 : 0;
  const width = video.videoWidth || 0;
  const height = video.videoHeight || 0;
  const timestamps = sampleMediaFrameTimestampsMs(durationMs, effect.sampleStride, effect.maxFrames, effect.fpsHint);
  const total = timestamps.length;
  for (let index = 0; index < total; index += 1) {
    const timestampMs = timestamps[index]!;
    video.currentTime = timestampMs / 1000;
    await waitForVideoEvent(video, "seeked");
    const frame = captureCanvasFrame(video, effect.maxLongEdgePx);
    await dispatchOne(effect.frameAction, {
      payload: frame.dataUrl,
      name,
      frameIndex: index,
      timestampMs,
      index,
      total,
      width: frame.width,
      height: frame.height,
      ...effect.args,
    });
  }
  await dispatchOne(effect.doneAction, { name, durationMs, frameCount: total, sampledCount: total, width, height, codec: "unknown", ...effect.args });
}
//#endregion Tier2

/** 🎞️ D5 `RequestMediaFrames` fields the two decode tiers need, decoupled from the raw `HostEffect`
 * union member shape so orchestration functions above take a plain, easily-constructed-in-tests object. */
export type RequestMediaFramesArgs = {
  readonly frameAction: string;
  readonly doneAction: string;
  readonly fallbackAction: string;
  readonly sampleStride: number;
  readonly maxFrames: number;
  readonly maxLongEdgePx: number;
  readonly fpsHint: number;
  readonly args?: Record<string, unknown>;
};

function bytesFromDataUrl(dataUrl: string): Uint8Array {
  const binary = atob(dataUrl.slice(dataUrl.indexOf(",") + 1));
  const bytes = new Uint8Array(binary.length);
  for (let i = 0; i < binary.length; i += 1) bytes[i] = binary.charCodeAt(i);
  return bytes;
}

function bytesToDataUrl(bytes: Uint8Array, mime: string): string {
  let binary = "";
  for (let i = 0; i < bytes.length; i += 1) binary += String.fromCharCode(bytes[i]!);
  return `data:${mime};base64,${btoa(binary)}`;
}

/** 🎞️ D5 top-level: sources video bytes (`payload` data URL, or the native file picker when unset),
 * tries Tier 1 when WebCodecs is available and the demux finds a usable AVC track, otherwise Tier 2's
 * `<video>` seek-and-capture; on total failure (can't demux AND Tier 2 also throws, e.g. a corrupt
 * file) dispatches `fallbackAction` once with the raw original bytes as a data URL. */
export async function runRequestMediaFrames(
  effect: RequestMediaFramesArgs,
  accept: string,
  payload: string | undefined,
  dispatchOne: EffectDispatchOne,
  createVideoElement: () => HTMLVideoElement = () => document.createElement("video"),
): Promise<void> {
  let bytes: Uint8Array;
  let name = "video";
  if (payload) {
    bytes = bytesFromDataUrl(payload);
  } else {
    const opened = await requestFileOpen(accept || "video/*", "dataUrl", false);
    if (opened.length === 0) return;
    bytes = bytesFromDataUrl(opened[0]!.contents);
    name = opened[0]!.name;
  }
  try {
    if (webCodecsAvailable() && (await runTier1VideoFrames(bytes, effect, name, dispatchOne))) return;
    const url = URL.createObjectURL(new Blob([bytes], { type: "video/mp4" }));
    const video = createVideoElement();
    video.muted = true;
    video.playsInline = true;
    video.src = url;
    try {
      await runTier2VideoFrames(video, effect, name, dispatchOne);
    } finally {
      URL.revokeObjectURL(url);
    }
  } catch (error) {
    console.error("[os-shell] requestMediaFrames: decode failed, falling back to raw bytes", error);
    await dispatchOne(effect.fallbackAction, { payload: bytesToDataUrl(bytes, "video/mp4"), name, ...effect.args });
  }
}
//#endregion RequestMediaFrames

function isStudioMode(pluginFilter?: string): boolean {
  return pluginFilter !== undefined && resolvePluginHostConfig(pluginFilter) !== undefined;
}

export interface StudioShellPath {
  readonly studioId: string;
  readonly instanceId?: string;
}

/** @emoji 🧭 Parses `/studios/:id` and `/studios/:id/instances/:iid` shell paths; null for any other route (e.g. home). */
export function parseStudioShellPath(path: string): StudioShellPath | null {
  const match = /^\/studios\/([^/]+)(?:\/instances\/([^/]+))?$/.exec(path);
  if (!match) return null;
  return { studioId: match[1]!, instanceId: match[2] };
}

function buildStudioPrograms(loaded: readonly LoadedPluginState[]): readonly StudioProgramEntry[] {
  return loaded.flatMap((entry) =>
    entry.manifest.programs.map((program) => ({
      pluginId: entry.handle.pluginId,
      programId: program.programId,
      appId: program.appId,
      label: program.label,
      document: program.document,
      yields: program.yields,
    })),
  );
}

export function appDocumentLabel(document: readonly string[]): string {
  return document.join(APP_DOCUMENT_SEPARATOR);
}

/** 🗺️ Resolves the document path effective under the active terminology; unknown/native ids fall back to `app.document`. */
export function resolveAppDocument(app: Pick<AppDefinition, "document" | "terminologyDocuments">, terminology: string): readonly string[] {
  return app.terminologyDocuments?.[terminology] ?? app.document;
}

/** 🗺️ Resolves the document path for a non-active app (studio spawn palette/spawned entries) by looking up its `AppDefinition` across loaded plugins; falls back to the raw `document` when the app can't be found. */
export function resolveDocumentByAppId(loadedPlugins: readonly LoadedPluginState[], appId: string, document: readonly string[], terminology: string): readonly string[] {
  for (const plugin of loadedPlugins) {
    const app = plugin.manifest.apps.find((candidate) => candidate.id === appId);
    if (app) return resolveAppDocument(app, terminology);
  }
  return document;
}

export function appWindowDocumentLabel(app: AppDefinition, terminology: string, windowLabel: string): string {
  const trimmed = windowLabel.trim();
  if (trimmed) return trimmed;
  const override = app.terminologyDocuments?.[terminology];
  return override?.[override.length - 1]?.trim() || app.label.trim();
}

function buildStudioPanelState(programs: readonly StudioProgramEntry[], spawnedApps: readonly SpawnedAppEntry[], activePanelTab = "s-play-catalogue", activeSpawnedId?: string): StudioPanelState {
  return { activePanelTab, programs, spawnedApps, activeSpawnedId };
}

function panelJsonFromState(state: StudioPanelState): string {
  return JSON.stringify(state);
}

function parsePanelState(viewState: ViewState): StudioPanelState | null {
  if (!viewState.panelJson) return null;
  try {
    return JSON.parse(viewState.panelJson) as StudioPanelState;
  } catch {
    return null;
  }
}

/** @emoji 🧭 Default anchor a plugin-declared panel-tab `group` docks into — groups only ever map to the four corners; the two middle anchors start empty and are user-populated via drag-and-drop or a dock skeleton override. */
function panelAnchorForGroup(group: string): PanelAnchor {
  if (group === "workbench" || group === "document") return "top-left";
  if (group === "details") return "top-right";
  if (group === "display") return "bottom-left";
  if (group === "settings") return "bottom-right";
  return "top-right";
}

function convertFrameworkLayoutNodeToModeLayout(node: WindowLayoutAxisNode | WindowLayoutStackNode | WindowLayoutWindowNode, appLabelsOverlay: PluginAppLabelsOverlay): WindowLayoutNode {
  if (node.kind === "window") {
    return { kind: "window", id: node.windowKindId, title: resolveAppLabel(appLabelsOverlay, "windowKind", node.windowKindId, node.title ?? node.windowKindId) };
  }
  if (node.kind === "stack") {
    return {
      kind: "stack",
      size: node.size,
      children: node.children.map((child) => ({
        kind: "window" as const,
        id: child.windowKindId,
        title: resolveAppLabel(appLabelsOverlay, "windowKind", child.windowKindId, child.title ?? child.windowKindId),
      })),
    };
  }
  return {
    kind: node.kind,
    size: node.size,
    children: node.children.map((child) => convertFrameworkLayoutNodeToModeLayout(child, appLabelsOverlay)),
  };
}

/** @emoji 🗣️ Re-resolves every window's title from the current app-labels overlay in place, preserving the tree's structure/sizes/arrangement — used to react to a locale/terminology switch without discarding the user's live layout. */
function retitleWindowLayoutNode(node: WindowLayoutNode, appLabelsOverlay: PluginAppLabelsOverlay): WindowLayoutNode {
  if (node.kind === "window") {
    return { ...node, title: resolveAppLabel(appLabelsOverlay, "windowKind", node.id, node.title ?? node.id) };
  }
  return { ...node, children: node.children.map((child) => retitleWindowLayoutNode(child, appLabelsOverlay)) } as WindowLayoutNode;
}

function convertFrameworkLayoutToModeLayout(layout: WindowLayout | undefined, windowIds: readonly string[], appLabelsOverlay: PluginAppLabelsOverlay): WindowLayoutNode {
  if (!layout?.root) return createEvenWindowLayout(windowIds.length ? windowIds : ["main"]);
  return convertFrameworkLayoutNodeToModeLayout(layout.root, appLabelsOverlay);
}

function modeLayoutNodeToFramework(node: WindowLayoutNode): WindowLayoutAxisNode | WindowLayoutStackNode | WindowLayoutWindowNode {
  if (node.kind === "window") {
    return { kind: "window", windowKindId: node.id, ...(node.title ? { title: node.title } : {}) };
  }
  if (node.kind === "stack") {
    return {
      kind: "stack",
      ...(node.size !== undefined ? { size: node.size } : {}),
      children: node.children.map((child) => ({
        kind: "window" as const,
        windowKindId: child.id,
        ...(child.title ? { title: child.title } : {}),
      })),
    };
  }
  return {
    kind: node.kind,
    ...(node.size !== undefined ? { size: node.size } : {}),
    children: node.children.map((child) => modeLayoutNodeToFramework(child) as WindowLayoutStackNode | WindowLayoutAxisNode),
  };
}

function captureCurrentFrameworkLayout(shellLayout: WindowLayoutNode | null, fallback?: WindowLayout): WindowLayout | undefined {
  if (!shellLayout) return fallback;
  const root = modeLayoutNodeToFramework(shellLayout);
  if (root.kind === "window") return { root: { kind: "stack", children: [root] } };
  return { root };
}

function findDefaultActiveWindowKindId(layout: WindowLayout | undefined, windowKinds: readonly { readonly id: string }[]): string | null {
  const collectWindowIds = (node: WindowLayoutAxisNode | WindowLayoutStackNode | WindowLayoutWindowNode): string[] => {
    if (node.kind === "window") return [node.windowKindId];
    if (node.kind === "stack") return node.children.map((child) => child.windowKindId);
    return node.children.flatMap((child) => collectWindowIds(child));
  };
  const ordered = layout?.root ? collectWindowIds(layout.root) : windowKinds.map((kind) => kind.id);
  for (const id of ordered) {
    if (windowKinds.some((kind) => kind.id === id)) return id;
  }
  return windowKinds[0]?.id ?? null;
}

function windowEngagementControlToSpec(control: WindowEngagementControl | undefined, onAction: (action: ActionDescriptor) => void): EngagementControl | undefined {
  if (!control) return undefined;
  if (control.kind === "ring" || control.kind === "toggleGroup") {
    return {
      kind: control.kind,
      id: control.id,
      label: control.label,
      value: control.value,
      disabled: control.disabled,
      options: control.options.map((row) => ({ id: row.id, label: row.label, disabled: row.disabled })),
      onSelect: control.onSelect ? (id: string) => onAction({ ...control.onSelect!, args: { ...(control.onSelect!.args as object | undefined), id } }) : undefined,
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
      onChange: control.onChange ? (value: string) => onAction({ ...control.onChange!, args: { ...(control.onChange!.args as object | undefined), value } }) : undefined,
    };
  }
  const dispatchNumeric = (action: ActionDescriptor | undefined, value: number) => {
    if (!action) return;
    onAction({ ...action, args: { ...(action.args as object | undefined), value } });
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

const PLUGIN_LOAD_TIMEOUT_MS = 30_000;

async function loadPluginModuleResilient(pluginId: string, moduleUrl: string): Promise<PluginWasmHandle | null> {
  try {
    return await Promise.race([
      loadPluginModule(pluginId, moduleUrl),
      new Promise<never>((_, reject) => {
        window.setTimeout(() => reject(new Error(`timeout loading ${pluginId}`)), PLUGIN_LOAD_TIMEOUT_MS);
      }),
    ]);
  } catch (error) {
    console.error("[DEBUG] plugin load failed", pluginId, error);
    return null;
  }
}

function isViewportSurface(surfaceKind: string | undefined): boolean {
  return surfaceKind === "world-3d" || surfaceKind === "node-graph" || surfaceKind === "canvas-2d";
}

function defaultViewportEngagement(): WindowEngagement {
  return {
    sessionActive: true,
    status: [{ id: "framework.viewport.status", text: shellLabel("ui.engagement.viewport") }],
  };
}

function resolveWindowEngagement(kind: AppDefinition["windowKinds"][number], byKind: Readonly<Record<string, WindowEngagement>>): WindowEngagement | undefined {
  const surfaceKind = (kind as { surfaceKind?: string }).surfaceKind;
  const declaredEngagement = kind.options.engagement.kind === "some" ? kind.options.engagement.value : undefined;
  return byKind[kind.id] ?? declaredEngagement ?? (isViewportSurface(surfaceKind) ? defaultViewportEngagement() : undefined);
}

function windowEngagementToSpec(engagement: WindowEngagement | undefined, onAction: (action: ActionDescriptor) => void): EngagementSpec | undefined {
  if (!engagement) return undefined;
  const options = engagement.options?.map((option) => ({
    id: option.id,
    label: option.label,
    icon: option.iconId ? <Icon icon={option.iconId in ICONS ? (option.iconId as IconName) : "circle-dot"} size="small" /> : undefined,
    pressed: option.pressed,
    disabled: option.disabled,
    onPress: option.action ? () => onAction(option.action!) : undefined,
  }));
  const input = engagement.input
    ? {
        id: engagement.input.id,
        value: engagement.input.value,
        placeholder: engagement.input.placeholder,
        disabled: engagement.input.disabled,
        onChange: engagement.input.onChange ? (value: string) => onAction({ ...engagement.input!.onChange!, args: { ...(engagement.input!.onChange!.args as object | undefined), value } }) : undefined,
        onSubmit: engagement.input.onSubmit ? (value: string) => onAction({ ...engagement.input!.onSubmit!, args: { ...(engagement.input!.onSubmit!.args as object | undefined), value } }) : undefined,
        onRepeatLast: engagement.input.onRepeatLast ? () => onAction(engagement.input!.onRepeatLast!) : undefined,
        onAbort: engagement.input.onAbort ? () => onAction(engagement.input!.onAbort!) : undefined,
      }
    : undefined;
  const status = engagement.status?.map((row) => ({ id: row.id, content: row.text }));
  const possibleEngagements = engagement.possibleEngagements?.map((row) => ({
    id: row.id,
    label: row.label,
    detail: row.detail,
    onSelect: row.action ? () => onAction(row.action!) : undefined,
  }));
  const control = windowEngagementControlToSpec(engagement.control, onAction);
  const controls = engagement.controls?.map((row) => windowEngagementControlToSpec(row, onAction)).filter((row): row is EngagementControl => row !== undefined);
  const hasContent = (options?.length ?? 0) > 0 || Boolean(input) || Boolean(control) || (controls?.length ?? 0) > 0 || (status?.length ?? 0) > 0 || (possibleEngagements?.length ?? 0) > 0;
  if (!hasContent) return undefined;
  return { sessionActive: engagement.sessionActive, options, input, control, controls, status, possibleEngagements };
}

function panelTabIcon(tabId: string, group: string): React.FC<{ size?: number }> {
  // 🌱 `group === "workbench"` already covers every host-app catalogue tab (each such app declares its
  // catalogue tab under `PanelGroup::Workbench` — see `s/plugin/rs`'s `App::builder(...).panel_tab(...)`)
  // so no separate app-specific tab-id literal is needed here.
  if (group === "workbench") return shellTabIcon(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID);
  if (tabId.includes("parameters")) return shellTabIcon(FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID);
  if (tabId.includes("inspector")) return shellTabIcon(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID);
  return shellTabIcon(tabId);
}

/** @emoji 🌳 Category-row icon: the first child's icon, or `fallback` when the category has no tabs yet. */
function categoryTabIcon(tabs: readonly PanelTabNode[], fallback: IconName): React.FC<{ size?: number }> {
  const FirstIcon = tabs[0]?.icon;
  return function CategoryTabIcon({ size = 16 }: { size?: number }) {
    return FirstIcon ? <FirstIcon size={size} /> : <Icon icon={fallback} size="small" />;
  };
}

/** @emoji 🌳 Depth-first leaves of a recursive panel-tab tree — the nodes that actually carry a `bodyKey` to render. */
export function flattenPanelTabLeaves<T extends { readonly children?: readonly T[] }>(tabs: readonly T[]): T[] {
  return tabs.flatMap((tab) => (tab.children && tab.children.length > 0 ? flattenPanelTabLeaves(tab.children) : [tab]));
}

/** @emoji 🌳 Converts one plugin-declared {@link AppPanelTabDefinition} (recursively) into a {@link PanelTabNode}. */
function panelTabDefinitionToNode(tab: AppPanelTabDefinition, group: string, panelUiByKey: Readonly<Record<string, UiNode>>, onAction: (action: ActionDescriptor) => void, order: number, appLabelsOverlay: PluginAppLabelsOverlay): PanelTabNode {
  const tabId = panelTabKindId(tab.kind);
  const label = resolveAppLabel(appLabelsOverlay, "panelTab", tabId, tab.label);
  if (tab.children && tab.children.length > 0) {
    return {
      kind: "branch",
      id: tabId,
      icon: panelTabIcon(tabId, group),
      name: label,
      order,
      children: tab.children.map((child, childOrder) => panelTabDefinitionToNode(child, group, panelUiByKey, onAction, childOrder, appLabelsOverlay)),
    };
  }
  return singleTreeLeaf({
    id: tabId,
    icon: panelTabIcon(tabId, group),
    name: label,
    order,
    tree: staticTreePanelDefinition(uiNodeToTreePanelConfig(panelUiByKey[tabId] ?? { type: "text", value: shellLabel("ui.common.loading") }, onAction)),
  });
}

function resolveCanvasBodyKey(app: AppDefinition): string {
  const windowKind = app.windowKinds[0];
  if (!windowKind) return "main";
  if (windowKind.bodyKey.includes("composite")) {
    const mediaGraph = app.windowKinds.find((kind) => kind.bodyKey.includes("media-graph"));
    return mediaGraph?.bodyKey ?? windowKind.bodyKey;
  }
  return windowKind.bodyKey;
}

//#region 🧰UtilityRegistry
/**
 * 🧰 Resolves the `UtilityDefinition`s in scope for one window kind against the app's utility registry:
 * the window kind's own `utilities` refs when non-empty, otherwise every utility the app declares (the
 * scoping fallback, mirroring `resolveWindowActions`' intent for utilities). Unresolvable refs are dropped.
 */
export function resolveUtilities(app: Pick<AppDefinition, "utilities">, windowKind: Pick<AppWindowKindDefinition, "utilities">): UtilityDefinition[] {
  const registry = app.utilities ?? [];
  const refs = windowKind.utilities ?? [];
  if (refs.length === 0) return [...registry];
  const resolved: UtilityDefinition[] = [];
  for (const ref of refs) {
    const utility = registry.find((entry) => entry.id === ref);
    if (utility) resolved.push(utility);
  }
  return resolved;
}

/** 🧰 Chrome-known ribbon-group ids that already have a `ui.ribbon.parent.*` translation key — the fallback tier for plugin-declared utility groups not covered by that plugin's own `groupLabels` overlay. */
const CHROME_KNOWN_RIBBON_PARENT_CATEGORIES = new Set(["history", "hand", "selection", "lasso", "filter", "open", "save", "transfer", "transform", "create", "view", "actions", "settings", "methods", "mode", "targets", "export", "utilities", "sync"]);

/** 🧰 Resolves a `UtilityDefinition.group` id's display label: the app's own `groupLabels` overlay first, then the shared `ui.ribbon.parent.*` chrome vocabulary for known category ids, else the raw id. */
function resolveUtilityGroupLabel(group: string, appLabelsOverlay: PluginAppLabelsOverlay): string {
  const fallback = CHROME_KNOWN_RIBBON_PARENT_CATEGORIES.has(group) ? shellLabel(`ui.ribbon.parent.${group as UiRibbonParentCategory}`) : group;
  return resolveAppLabel(appLabelsOverlay, "group", group, fallback);
}

/** 🧰 One `UtilityDefinition` → the lean `DerivedUtilitySpec` consumed by {@link deriveUtilityNodes}, resolving the label (and, for grouped utilities, the group label) through the app's locale/terminology overlay. */
function utilityDefinitionToSpec(utility: UtilityDefinition, appLabelsOverlay: PluginAppLabelsOverlay): DerivedUtilitySpec {
  return {
    id: utility.id,
    label: resolveAppLabel(appLabelsOverlay, "utility", utility.id, utility.label),
    iconId: utility.iconId,
    group: utility.group ?? undefined,
    groupLabel: utility.group ? resolveUtilityGroupLabel(utility.group, appLabelsOverlay) : undefined,
    category: utility.category ?? "utilities",
  };
}

/** 🧰 Stamps the owning `windowId` onto every `setActiveUtility` descriptor in a derived utility tree so the shell's `onAction` interceptor targets the right window regardless of which window is globally active. */
function tagSetActiveUtilityWindow(nodes: readonly UtilityNode[], windowId: string): UtilityNode[] {
  return nodes.map((node) => {
    if (node.kind === "collection") return { ...node, children: tagSetActiveUtilityWindow(node.children, windowId) };
    if (node.kind === "toggle" && "onChange" in node && node.onChange.action === SET_ACTIVE_UTILITY_ACTION_ID) {
      return { ...node, onChange: { ...node.onChange, args: { ...(node.onChange.args as object | undefined), windowId } } };
    }
    return node;
  });
}

/**
 * 🧰 Builds the window utility bar `UtilityNode[]` for one window purely from the static utility registry plus
 * the host-owned active utility id — the replacement for the deleted plugin `list-tools` sourcing. Each
 * `setActiveUtility` descriptor is tagged with `windowId` so activation is scoped to this exact window.
 */
export function resolveUtilityNodes(
  app: Pick<AppDefinition, "utilities" | "controllerId">,
  windowKind: Pick<AppWindowKindDefinition, "utilities">,
  activeUtilityId: string | null | undefined,
  windowId: string,
  appLabelsOverlay: PluginAppLabelsOverlay = EMPTY_APP_LABELS_OVERLAY,
): UtilityNode[] {
  const utilities = resolveUtilities(app, windowKind);
  if (utilities.length === 0) return [];
  return tagSetActiveUtilityWindow(
    deriveUtilityNodes(
      app.controllerId,
      utilities.map((utility) => utilityDefinitionToSpec(utility, appLabelsOverlay)),
      activeUtilityId ?? undefined,
    ),
    windowId,
  );
}
//#endregion 🧰UtilityRegistry

/** @emoji 💬 Builds spawned-window engagement, measures, and utility-options chrome for one window kind. */
export function spawnedWindowChromeForKind(
  kind: AppDefinition["windowKinds"][number],
  engagementsByKind: Readonly<Record<string, WindowEngagement>>,
  measuresByKind: Readonly<Record<string, readonly WindowMeasure[]>>,
  activeUtilityId: string | undefined,
  onAction: (action: ActionDescriptor) => void,
): { readonly engagement?: EngagementSpec; readonly measures: ReactNode; readonly utilityOptions: ReactNode } {
  const { measures, utilityOptions } = windowMeasuresChrome(measuresByKind[kind.id] ?? kind.options.measures, activeUtilityId, kind.id, onAction);
  return {
    engagement: windowEngagementToSpec(resolveWindowEngagement(kind, engagementsByKind), onAction),
    measures,
    utilityOptions,
  };
}

function isTreeNode(node: UiNode): node is UiTreeNode {
  return node.type === "tree";
}

export function uiNodeToTreePanelConfig(node: UiNode, onAction: (action: ActionDescriptor) => void): TreePanelConfig {
  if (isTreeNode(node)) return { ...uiTreeNodeToTreePanelConfig(node, onAction), dragAndDropController: declarativeTreeDragController(node, onAction) };
  return {
    sections: [
      {
        id: "panel.body",
        label: "",
        items: [
          {
            id: "panel.body.content",
            label: "",
            control: <ChromeAwareWindowScrollSurface className="min-h-0 flex-1">{interpretUiNode(node, { onAction })}</ChromeAwareWindowScrollSurface>,
          },
        ],
      },
    ],
  };
}

function shellTabIcon(iconId: string): React.FC<{ size?: number }> {
  return function ShellTabIcon({ size = 16 }: { size?: number }) {
    let iconName: IconName = "circle-dot";
    if (iconId === FRAMEWORK_PANEL_TAB_DOCUMENT_ICON_ID) {
      iconName = "file-text";
    } else if (iconId in ICONS) {
      iconName = iconId as IconName;
    }
    return <Icon icon={iconName} size={size} />;
  };
}

/** @emoji 🌐 Resolves a chrome translation key outside hook context (tree builders run there). */
function shellLabel(key: UiTranslationKey): string {
  return resolveTranslationLabel(uiI18n.t(key)) ?? key;
}

/** @emoji 🗣️ Stable empty overlay reference so components depending on it don't re-render before the first `appLabels` fetch resolves. */
const EMPTY_APP_LABELS_OVERLAY: PluginAppLabelsOverlay = {
  windowKindLabels: {},
  panelTabLabels: {},
  modeLabels: {},
  actionLabels: {},
  utilityLabels: {},
  exampleLabels: {},
  actionArgLabels: {},
  dialogLabels: {},
  introductionLabels: {},
  groupLabels: {},
};

/** @emoji 🗣️ Resolves a window-kind/panel-tab/mode/action/utility/example/actionArg/dialog/introduction/group id's locale-aware label from the active app's overlay, falling back to the static manifest label. */
function resolveAppLabel(overlay: PluginAppLabelsOverlay, kind: "windowKind" | "panelTab" | "mode" | "action" | "utility" | "example" | "actionArg" | "dialog" | "introduction" | "group", id: string, fallback: string): string {
  const map =
    kind === "windowKind"
      ? overlay.windowKindLabels
      : kind === "panelTab"
        ? overlay.panelTabLabels
        : kind === "mode"
          ? overlay.modeLabels
          : kind === "action"
            ? overlay.actionLabels
            : kind === "utility"
              ? overlay.utilityLabels
              : kind === "example"
                ? overlay.exampleLabels
                : kind === "actionArg"
                  ? overlay.actionArgLabels
                  : kind === "dialog"
                    ? overlay.dialogLabels
                    : kind === "introduction"
                      ? overlay.introductionLabels
                      : overlay.groupLabels;
  return map[id] ?? fallback;
}

/** @emoji 🗣️ Resolves one action-arg's label + (for `select` controls) its options' labels from the overlay's `actionArgLabels` map, keyed `"{scopeId}.{argId}"` / `"{scopeId}.{argId}.option.{value}"`. `scopeId` is an action id for staged/palette forms or a dialog id for dialog args. */
function resolveActionArgDef(def: ActionArgDef, scopeId: string, overlay: PluginAppLabelsOverlay): ActionArgDef {
  const label = resolveAppLabel(overlay, "actionArg", `${scopeId}.${def.id}`, def.label);
  if (def.control.kind !== "select") return label === def.label ? def : { ...def, label };
  const options = def.control.options.map((option) => ({ ...option, label: resolveAppLabel(overlay, "actionArg", `${scopeId}.${def.id}.option.${option.value}`, option.label) }));
  return { ...def, label, control: { ...def.control, options } };
}

/** @emoji 🗣️ Resolves a `DialogDefinition`'s title/body/submitLabel/args from the overlay's `dialogLabels`/`actionArgLabels` maps, keyed by the dialog's own id. */
function resolveDialogDefinition(dialog: DialogDefinition, overlay: PluginAppLabelsOverlay): DialogDefinition {
  return {
    ...dialog,
    title: resolveAppLabel(overlay, "dialog", `${dialog.id}.title`, dialog.title),
    body: dialog.body ? resolveAppLabel(overlay, "dialog", `${dialog.id}.body`, dialog.body) : dialog.body,
    submitLabel: resolveAppLabel(overlay, "dialog", `${dialog.id}.submit`, dialog.submitLabel),
    args: dialog.args.map((def) => resolveActionArgDef(def, dialog.id, overlay)),
  };
}

/** @emoji 🗣️ Resolves an `IntroductionDefinition`'s title and every step's title/body from the overlay's `introductionLabels` map. */
function resolveIntroductionDefinition(introduction: IntroductionDefinition, overlay: PluginAppLabelsOverlay): IntroductionDefinition {
  return {
    title: resolveAppLabel(overlay, "introduction", "intro.title", introduction.title),
    steps: introduction.steps.map(
      (step): IntroductionStepDefinition => ({
        ...step,
        title: resolveAppLabel(overlay, "introduction", `intro.step.${step.id}.title`, step.title),
        body: resolveAppLabel(overlay, "introduction", `intro.step.${step.id}.body`, step.body),
      }),
    ),
  };
}

/** @emoji 🗣️ Resolves a terminology id's display name; chrome-known ids get a translated label, app-declared ids fall back to their raw id. */
function shellTerminologyLabel(id: string): string {
  const isChromeKnown = id === "native" || id === "reuse";
  return isChromeKnown ? shellLabel(`ui.settings.terminology.${id as UiChromeTerminologyId}`) : id;
}

/** @emoji 🎚️ Serializes async updates while retaining only the newest value requested during an in-flight update. */
export function createLatestAsyncDispatcher<T>(dispatchValue: (value: T) => unknown): (value: T) => void {
  let running = false;
  let queued: T | undefined;
  let hasQueued = false;
  const dispatchLatest = (value: T) => {
    if (running) {
      queued = value;
      hasQueued = true;
      return;
    }
    running = true;
    void Promise.resolve(dispatchValue(value)).finally(() => {
      running = false;
      if (!hasQueued) return;
      const next = queued as T;
      queued = undefined;
      hasQueued = false;
      dispatchLatest(next);
    });
  };
  return dispatchLatest;
}

/** @emoji 🎚️ Keeps a measure slider live without accumulating stale document actions behind the pointer. */
function WindowMeasureSlider({ measure, onAction }: { readonly measure: Extract<WindowMeasure, { kind: "slider" }>; readonly onAction: (action: ActionDescriptor) => unknown }) {
  const dispatchLatest = useMemo(() => createLatestAsyncDispatcher(onAction), [onAction]);

  return (
    <Slider
      id={measure.id}
      value={[measure.value]}
      min={measure.min}
      max={measure.max}
      step={measure.step}
      onValueChange={(values) => dispatchLatest({ ...measure.onChange, args: { ...(measure.onChange.args as object | undefined), value: values[0] ?? measure.value } })}
    />
  );
}

function renderWindowMeasure(measure: WindowMeasure, onAction: (action: ActionDescriptor) => unknown): ReactNode {
  if (measure.kind === "group") {
    return (
      <WindowMeasureTreeGroup key={measure.id} id={measure.id} label={measure.label} defaultOpen={measure.defaultOpen}>
        {measure.children.map((child) => renderWindowMeasure(child, onAction))}
      </WindowMeasureTreeGroup>
    );
  }
  if (measure.kind === "select") {
    return (
      <WindowMeasureTreeLeaf key={measure.id} label={measure.label}>
        <Select value={measure.value} onValueChange={(value) => onAction({ ...measure.onChange, args: { ...(measure.onChange.args as object | undefined), value } })}>
          <SelectTrigger id={measure.id} className="h-small w-full min-w-0" size="sm">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            {measure.items.map((item) => (
              <SelectItem key={item.id} value={item.value}>
                {item.label}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </WindowMeasureTreeLeaf>
    );
  }
  if (measure.kind === "slider") {
    return (
      <WindowMeasureTreeLeaf key={measure.id} label={measure.label}>
        <WindowMeasureSlider measure={measure} onAction={onAction} />
      </WindowMeasureTreeLeaf>
    );
  }
  if (measure.kind === "toggle") {
    return (
      <WindowMeasureTreeLeaf key={measure.id} label={measure.label}>
        <Toggle
          id={measure.id}
          pressed={measure.pressed}
          text={measure.text}
          icon={<Icon icon={measure.iconId in ICONS ? (measure.iconId as IconName) : "circle-dot"} size="small" />}
          onPressedChange={(pressed) => onAction({ ...measure.onChange, args: { ...(measure.onChange.args as object | undefined), pressed } })}
        />
      </WindowMeasureTreeLeaf>
    );
  }
  return null;
}

function windowMeasuresOverlay(measures: readonly WindowMeasure[] | undefined, onAction: (action: ActionDescriptor) => unknown): ReactNode | undefined {
  if (!measures || measures.length === 0) return undefined;
  return <WindowMeasuresTree>{measures.map((measure) => renderWindowMeasure(measure, onAction))}</WindowMeasuresTree>;
}

function SelectionUtilityOptions({ activeUtilityId, windowId, onAction }: { readonly activeUtilityId: string | undefined; readonly windowId: string; readonly onAction: (action: ActionDescriptor) => void }) {
  const selectionMethod = activeUtilityId === "selectLasso" ? "lasso" : "rectangle";

  const [selectionMode, setSelectionMode] = useState<"default" | "additive" | "subtractive" | "invertive">(() => {
    return (globalThis as any).__selectionMode || "default";
  });

  const handleModeChange = (mode: "default" | "additive" | "subtractive" | "invertive") => {
    (globalThis as any).__selectionMode = mode;
    setSelectionMode(mode);
    window.dispatchEvent(new CustomEvent("semio:selectionOptionsChanged"));
  };

  const handleMethodChange = (method: "rectangle" | "lasso") => {
    onAction({
      controllerId: "window",
      action: SET_ACTIVE_UTILITY_ACTION_ID,
      args: { windowId, utilityId: method === "lasso" ? "selectLasso" : "selectMarquee" },
    });
  };

  return (
    <div className="flex items-center gap-double">
      <div className="flex items-center gap-single">
        <span className="text-tiny text-muted-foreground uppercase tracking-wider font-semibold">Method</span>
        <ToggleGroup
          kind="single"
          value={selectionMethod}
          onValueChange={(val) => {
            if (val === "rectangle" || val === "lasso") {
              handleMethodChange(val);
            }
          }}
          items={[
            { value: "rectangle", icon: <Icon icon="square-dashed" size="small" />, text: "Rectangle" },
            { value: "lasso", icon: <Icon icon="lasso" size="small" />, text: "Lasso" },
          ]}
        />
      </div>
      <RibbonDivider />
      <div className="flex items-center gap-single">
        <span className="text-tiny text-muted-foreground uppercase tracking-wider font-semibold">Mode</span>
        <ToggleGroup
          kind="single"
          value={selectionMode}
          onValueChange={(val) => {
            if (val === "default" || val === "additive" || val === "subtractive" || val === "invertive") {
              handleModeChange(val);
            }
          }}
          items={[
            { value: "default", text: "Selective" },
            { value: "additive", text: "Additive" },
            { value: "subtractive", text: "Subtractive" },
            { value: "invertive", text: "Invertive" },
          ]}
        />
      </div>
    </div>
  );
}

function windowMeasuresChrome(
  measures: readonly WindowMeasure[] | undefined,
  activeUtilityId: string | undefined,
  windowId: string,
  onAction: (action: ActionDescriptor) => unknown,
): { readonly measures: ReactNode | undefined; readonly utilityOptions: ReactNode | undefined } {
  const { general, utilityOptions } = partitionWindowMeasures(measures ?? [], activeUtilityId);
  return {
    measures: windowMeasuresOverlay(general, onAction),
    utilityOptions: windowMeasuresOverlay(utilityOptions, onAction),
  };
}

/** @emoji 🎓 Whether a utility node tree has a node (leaf or group) with the given id anywhere in it — used
 * to decide if this window's utility bar is the one an introduction step's `Utility` anchor targets. */
function utilityNodeTreeContainsId(nodes: readonly UtilityNode[], targetId: string): boolean {
  return nodes.some((node) => node.id === targetId || (node.kind === "collection" && utilityNodeTreeContainsId(node.children, targetId)));
}

function utilityBarNode(utilities: readonly UtilityNode[] | undefined, windowId: string, onAction: (action: ActionDescriptor) => void, revealUtilityId?: string | null, utilityOptions?: ReactNode): ReactNode {
  if (!utilities?.length && !utilityOptions) return undefined;
  const categories = groupUtilityNodesByCategory(utilities ?? [], UTILITY_CATEGORIES);
  if (!categories.length && !utilityOptions) return undefined;
  const grouped: UtilityNode[] = [];
  for (const node of categories) {
    if (node.kind === "collection" && (node.category === "utilities" || node.category === "selection")) {
      if (node.id === "group:Select" || node.id === "group:selection" || node.label === "Select" || node.text === "Select") {
        grouped.push(...node.children);
      } else {
        for (const child of node.children) {
          if (child.kind === "collection" && (child.id === "group:Select" || child.id === "group:selection" || child.label === "Select" || child.text === "Select")) {
            grouped.push(...child.children);
          } else {
            grouped.push(child);
          }
        }
      }
    } else {
      grouped.push(node);
    }
  }
  return <UtilityTree id={`ui.utilities.${windowId}`} utilities={grouped} onAction={onAction} direction="up" revealUtilityId={revealUtilityId} utilityOptions={utilityOptions} />;
}

//#region 🧰WindowActionPane
/**
 * 🎛 Renders one {@link ActionArgControl} into a STAGED form field — the crucial difference from
 * `renderUiControl` in `ui-interpreter.tsx` is that this dispatches NOTHING globally; `onChange` only
 * writes to the caller's local staged buffer. `value` is the already-resolved effective value
 * (staged ?? default ?? unset).
 */
export function renderStagedArgControl(def: ActionArgDef, value: unknown, onChange: (value: unknown) => void, disabled?: boolean): ReactElement {
  const control: ActionArgControl = def.control;
  switch (control.kind) {
    case "text":
      return <Input id={def.id} type="text" className="h-medium w-full min-w-0" value={typeof value === "string" ? value : ""} placeholder={control.placeholder} disabled={disabled} onChange={(event) => onChange(event.target.value)} />;
    case "number":
      return (
        <Input
          id={def.id}
          type="number"
          className="h-medium w-full min-w-0"
          value={value === undefined || value === null || value === "" ? "" : String(value)}
          min={control.min}
          max={control.max}
          step={control.step}
          disabled={disabled}
          onChange={(event) => onChange(event.target.value === "" ? undefined : Number(event.target.value))}
        />
      );
    case "slider": {
      const numeric = typeof value === "number" && Number.isFinite(value) ? value : control.min;
      const slider = <Slider id={def.id} className="w-full min-w-0" min={control.min} max={control.max} step={control.step ?? 1} value={[numeric]} disabled={disabled} onValueChange={(values) => onChange(values[0] ?? numeric)} />;
      if (!control.unit) return slider;
      return (
        <div className="flex w-full min-w-0 items-center gap-single">
          {slider}
          <span className="shrink-0 text-xs tabular-nums text-muted-foreground">
            {numeric} {control.unit}
          </span>
        </div>
      );
    }
    case "toggle":
      return <Toggle id={def.id} pressed={value === true} text={def.label} disabled={disabled} onPressedChange={(pressed) => onChange(pressed)} />;
    case "select":
      return (
        <Select value={typeof value === "string" && value ? value : undefined} disabled={disabled} onValueChange={(next) => onChange(next)}>
          <SelectTrigger id={def.id} className="h-medium w-full min-w-0" size="sm">
            <SelectValue placeholder={def.label} />
          </SelectTrigger>
          <SelectContent>
            {control.options.map((option, index) => (
              <SelectItem key={`${def.id}:${index}:${option.value}`} value={option.value}>
                {option.label}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      );
    case "vec3": {
      const tuple = Array.isArray(value) && value.length >= 3 ? (value as readonly number[]) : null;
      const axes = ["x", "y", "z"] as const;
      return (
        <div className="grid grid-cols-3 gap-single">
          {axes.map((axis, index) => (
            <Input
              key={`${def.id}.${axis}`}
              id={`${def.id}.${axis}`}
              type="number"
              className="h-medium w-full min-w-0"
              value={tuple ? String(tuple[index] ?? 0) : ""}
              placeholder={axis}
              disabled={disabled}
              onChange={(event) => {
                const parsed = Number(event.target.value);
                if (!Number.isFinite(parsed)) return;
                const next: [number, number, number] = tuple ? [tuple[0] ?? 0, tuple[1] ?? 0, tuple[2] ?? 0] : [0, 0, 0];
                next[index] = parsed;
                onChange(next);
              }}
            />
          ))}
        </div>
      );
    }
    case "iconSelect":
      return <IconSelector id={def.id} classifyIconSelectorMode={undefined} value={typeof value === "string" ? value : ""} uniform onChange={(next) => onChange(next)} />;
  }
}

/** 🧰 True when an action carries arguments and therefore stages a form instead of firing immediately (P1–P4). */
export function actionRequiresStagedForm(action: Pick<ActionDefinition, "args">): boolean {
  return (action.args?.length ?? 0) > 0;
}

/** 🧰 The decision a bound hotkey makes for one action (P4). */
/** ⌨️ Splits a declared `keys` binding (comma-separated chord alternatives, e.g. `"mod+z,ctrl+z"`) into individual chords. Shared by the action and command keydown listeners. */
export function parseKeybindingChords(keys: string): string[] {
  return keys
    .split(",")
    .map((key) => key.trim().toLowerCase())
    .filter(Boolean);
}

/** ⌨️ True when a keydown's target is a text-editing surface (input/textarea/select/contenteditable) — hotkeys never fire while the user is typing. */
export function isEditableEventTarget(target: EventTarget | null): boolean {
  if (!(target instanceof HTMLElement)) return false;
  const tag = target.tagName;
  if (tag === "INPUT" || tag === "TEXTAREA" || tag === "SELECT") return true;
  if (target.isContentEditable) return true;
  return target.closest("[contenteditable='true'], [role='textbox']") != null;
}

/** ⌨️ True when a keydown event matches one `+`-joined chord (e.g. `"mod+shift+z"`), where `mod` accepts either ctrl or meta. */
export function keyboardEventMatchesChord(event: KeyboardEvent, chord: string): boolean {
  const parts = chord.split("+").map((part) => part.trim());
  const key = parts[parts.length - 1] ?? "";
  const needsCtrl = parts.includes("ctrl") || parts.includes("meta") || parts.includes("mod");
  const needsShift = parts.includes("shift");
  const needsAlt = parts.includes("alt");
  const hasCtrl = event.ctrlKey || event.metaKey;
  if (needsCtrl !== hasCtrl) return false;
  if (needsShift !== event.shiftKey) return false;
  if (needsAlt !== event.altKey) return false;
  return event.key.toLowerCase() === key;
}

export type KeybindingIntent = { readonly kind: "fire" } | { readonly kind: "open"; readonly actionId: string } | { readonly kind: "execute"; readonly actionId: string; readonly args: Record<string, unknown> };

/**
 * ✍️ Pure P4 decision: an arg-less action fires directly; an arg-carrying action opens its staged form,
 * unless that form is already the expanded one in the active window AND validation passes, in which case
 * the hotkey executes with the merged effective args. An already-open-but-invalid form stays open.
 */
export function resolveKeybindingIntent(definition: ActionDefinition | undefined, expandedActionId: string | null, stagedArgs: Readonly<Record<string, unknown>>): KeybindingIntent {
  if (!definition || !actionRequiresStagedForm(definition)) return { kind: "fire" };
  if (expandedActionId === definition.id) {
    const effective = effectiveActionArgs(definition.args, stagedArgs);
    if (missingRequiredArgs(definition.args, effective).length === 0) return { kind: "execute", actionId: definition.id, args: effective };
  }
  return { kind: "open", actionId: definition.id };
}

/** 🧰 Pure P5 activation decision: an empty request, or re-requesting the already-active utility, deactivates (null); otherwise the requested utility becomes active. */
export function resolveUtilityActivation(current: string | null | undefined, requested: string): string | null {
  return requested === "" || (current ?? null) === requested ? null : requested;
}

/** 🎛 Props for the per-window Action rail body (P1/P2). */
export type WindowActionPaneProps = {
  readonly windowId: string;
  readonly controllerId: string;
  readonly actions: readonly ActionDefinition[];
  readonly expandedActionId: string | null;
  readonly stagedArgsByKey: Readonly<Record<string, Readonly<Record<string, unknown>>>>;
  readonly disabled: boolean;
  readonly onExpandedChange: (actionId: string | null) => void;
  readonly onStageArg: (actionId: string, argId: string, value: unknown) => void;
  readonly onResetArgs: (actionId: string) => void;
  readonly onExecute: (descriptor: ActionDescriptor) => void;
};

/**
 * 🎛 The per-window Actions rail body (P1/P2). Zero-arg actions ARE the execute button; arg-carrying
 * actions expand a locally-buffered staged form — nothing dispatches on edit, effective value is
 * `staged ?? default ?? unset`, Execute is enabled only when every required arg has an effective value,
 * fires exactly ONE `ActionDescriptor` with the merged args, and keeps the staged values afterward.
 * When `disabled` (an active utility with `allowsActionsWhileActive === false`), every row renders disabled.
 */
export function WindowActionPane(props: WindowActionPaneProps): ReactElement {
  const { windowId, controllerId, actions, expandedActionId, stagedArgsByKey, disabled, onExpandedChange, onStageArg, onResetArgs, onExecute } = props;
  return (
    <div data-slot="window-action-pane" className="flex min-w-0 flex-col gap-single p-single">
      {actions.map((action) => {
        if (!actionRequiresStagedForm(action)) {
          return (
            <Button
              key={action.id}
              id={`${windowId}-action-${action.id}`}
              text={action.label}
              icon={action.iconId && action.iconId in ICONS ? (action.iconId as IconName) : "play"}
              disabled={disabled}
              onClick={() => onExecute({ controllerId, action: action.id })}
            />
          );
        }
        const expanded = expandedActionId === action.id;
        const staged = stagedArgsByKey[actionStageKey(windowId, action.id)] ?? {};
        const effective = effectiveActionArgs(action.args, staged);
        const missing = missingRequiredArgs(action.args, effective);
        return (
          <div key={action.id} data-slot="window-action-row" className={cn("flex min-w-0 flex-col rounded-md border", borderElementClass)}>
            <Button id={`${windowId}-action-${action.id}-disclosure`} text={`${action.label}…`} icon={expanded ? "chevron-down" : "chevron-right"} onClick={() => onExpandedChange(expanded ? null : action.id)} />
            {expanded ? (
              <div data-slot="window-action-form" className="flex min-w-0 flex-col gap-single p-single">
                {action.args.map((def) => (
                  <Field key={def.id} id={`${windowId}-action-${action.id}-arg-${def.id}`} label={def.label} description={def.description} required={def.required}>
                    {renderStagedArgControl(def, effective[def.id], (value) => onStageArg(action.id, def.id, value), disabled)}
                  </Field>
                ))}
                <div className="flex items-center gap-single">
                  <Button
                    id={`${windowId}-action-${action.id}-execute`}
                    text={shellLabel("ui.common.execute")}
                    icon="check"
                    disabled={disabled || missing.length > 0}
                    onClick={() => onExecute({ controllerId, action: action.id, args: effectiveActionArgs(action.args, staged) })}
                  />
                  <Button id={`${windowId}-action-${action.id}-reset`} text={shellLabel("ui.common.reset")} icon="undo" onClick={() => onResetArgs(action.id)} />
                </div>
              </div>
            ) : null}
          </div>
        );
      })}
    </div>
  );
}

/** 🧰 Slice of the {@link ActionPaneState} the {@link windowActionPaneNode} builder reads. */
type ActionPaneSlice = Pick<ActionPaneState, "expandedByWindowId" | "stagedArgsByKey" | "activeUtilityByWindowId">;

/**
 * 🧰 Sibling of {@link utilityBarNode}: resolves a window kind's panel-eligible actions and returns a
 * bound {@link WindowActionPane}, or `undefined` when the window has no resolved actions (so the rail
 * chip never renders). Rows render disabled while an active utility gates actions
 * (`allowsActionsWhileActive === false`).
 */
function windowActionPaneNode(
  app: AppDefinition,
  windowKind: AppWindowKindDefinition,
  windowId: string,
  actionPane: ActionPaneSlice,
  onAction: (action: ActionDescriptor) => void,
  dispatch: (action: ShellAction) => void,
  appLabelsOverlay: PluginAppLabelsOverlay = EMPTY_APP_LABELS_OVERLAY,
): ReactNode {
  const resolvedActions = resolveWindowActions(app, windowKind);
  if (resolvedActions.length === 0) return undefined;
  const actions = resolvedActions.map((action) => ({
    ...action,
    label: resolveAppLabel(appLabelsOverlay, "action", action.id, action.label),
    args: action.args.map((def) => resolveActionArgDef(def, action.id, appLabelsOverlay)),
  }));
  const activeUtilityId = actionPane.activeUtilityByWindowId[windowId] ?? null;
  const activeUtility = activeUtilityId ? (app.utilities ?? []).find((utility) => utility.id === activeUtilityId) : undefined;
  const disabled = Boolean(activeUtility && activeUtility.allowsActionsWhileActive === false);
  return (
    <WindowActionPane
      windowId={windowId}
      controllerId={app.controllerId}
      actions={actions}
      expandedActionId={actionPane.expandedByWindowId[windowId] ?? null}
      stagedArgsByKey={actionPane.stagedArgsByKey}
      disabled={disabled}
      onExpandedChange={(actionId) => dispatch({ type: "SET_ACTION_PANE_EXPANDED", windowId, value: actionId })}
      onStageArg={(actionId, argId, value) => dispatch({ type: "STAGE_ACTION_ARG", windowId, actionId, argId, value })}
      onResetArgs={(actionId) => dispatch({ type: "RESET_ACTION_ARGS", windowId, actionId })}
      onExecute={onAction}
    />
  );
}
//#endregion 🧰WindowActionPane

//#region 🎛CommandRegistry
/** 🎛 Where a resolved command came from — drives palette/footer category grouping and dispatch routing. */
export type ResolvedCommand = {
  readonly definition: CommandDefinition;
  readonly source: { readonly kind: "os" } | { readonly kind: "plugin" } | { readonly kind: "app" } | { readonly kind: "mode"; readonly modeId: string };
};

/**
 * 🎛 Aggregates every command visible for the current session: os built-ins, the active session's
 * plugin-scope commands, the app's App-scope commands, and Mode-scope commands referenced by the
 * active mode's `commands` refs. There are no window-level commands (see `CommandScope`) — unlike
 * `resolveWindowActions`/`resolveUtilities`, this never takes a window kind.
 */
export function resolveCommands(
  osCommands: readonly CommandDefinition[],
  activePluginManifest: Pick<PluginManifest, "commands"> | null | undefined,
  app: Pick<AppDefinition, "commands" | "modes"> | null | undefined,
  activeModeId: string,
): ResolvedCommand[] {
  const resolved: ResolvedCommand[] = osCommands.map((definition) => ({ definition, source: { kind: "os" as const } }));
  for (const definition of activePluginManifest?.commands ?? []) {
    resolved.push({ definition, source: { kind: "plugin" as const } });
  }
  if (!app) return resolved;
  const activeMode = (app.modes as readonly AppModeDefinition[] | undefined)?.find((mode) => mode.id === activeModeId);
  const modeCommandIds = new Set(activeMode?.commands ?? []);
  for (const definition of app.commands ?? []) {
    if (definition.scope === "app") resolved.push({ definition, source: { kind: "app" as const } });
    else if (definition.scope === "mode" && modeCommandIds.has(definition.id)) resolved.push({ definition, source: { kind: "mode" as const, modeId: activeModeId } });
  }
  return resolved;
}

/** 🎛 Chrome-known command category ids that already have a `ui.settings.tab.*` translation key. */
const CHROME_KNOWN_COMMAND_CATEGORIES = new Set(["general", "mode", "expertise", "app", "appearance", "layout", "language", "terminology", "theme"]);

/** 🎛 Loose title-case for an open-set command category id (e.g. "appearance" -> "Appearance"). Falls back to this for app/plugin-invented categories that have no fixed framework vocabulary entry. */
function titleizeCommandCategory(category: string): string {
  return category.replace(/[-_]+/g, " ").replace(/\b\w/g, (char) => char.toUpperCase());
}

/** 🎛 Resolves a command category's display label, reusing the existing `ui.settings.tab.*` keys for chrome-known ids and falling back to a loose title-case for open-set app/plugin categories. */
function commandCategoryLabel(category: string): string {
  return CHROME_KNOWN_COMMAND_CATEGORIES.has(category) ? shellLabel(`ui.settings.tab.${category as "general" | "mode" | "expertise" | "app" | "appearance" | "layout" | "language" | "terminology" | "theme"}`) : titleizeCommandCategory(category);
}

/** 🎛 Ordered, deduped category tabs for the footer command panel, derived from whatever commands actually resolved. */
export function commandCategories(commands: readonly ResolvedCommand[]): { readonly id: string; readonly label: string }[] {
  const seen = new Set<string>();
  const categories: { readonly id: string; readonly label: string }[] = [];
  for (const { definition } of commands) {
    if (seen.has(definition.category)) continue;
    seen.add(definition.category);
    categories.push({ id: definition.category, label: commandCategoryLabel(definition.category) });
  }
  return categories;
}

function selectCommandArg(id: string, label: string, options: readonly { readonly value: string; readonly label: string }[]): ActionArgDef {
  return { id, label, control: { kind: "select", options: options.map((option) => ({ ...option })) }, required: true };
}

/**
 * 🎛 Os-level built-in commands — app introduction/theme/layout/locale/appearance/expertise,
 * `scope: "os"`, handled
 * locally by the shell (never routed to a plugin). Rebuilt via `useMemo` since the theme and
 * terminology option lists are live state.
 */
export function buildOsCommands(themeList: readonly UiTheme[], terminologies: readonly string[], hasIntroduction: boolean, locks: ResolvedShellLocks = EMPTY_SHELL_LOCKS): CommandDefinition[] {
  const lockedCommandIds = new Set<string>([...(locks.appearance ? ["os.setAppearance"] : []), ...(locks.themeId ? ["os.setThemeId"] : []), ...(locks.locale ? ["os.setLocale"] : []), ...(locks.terminology ? ["os.setTerminology"] : [])]);
  const commands: CommandDefinition[] = [
    ...(hasIntroduction ? [{ id: "os.introduceApp", label: shellLabel("ui.command.introduceApp"), scope: "os" as const, category: "app", inPalette: true, args: [] }] : []),
    {
      id: "os.setAppearance",
      label: shellLabel("ui.command.setAppearance"),
      scope: "os",
      category: "appearance",
      inPalette: true,
      args: [
        selectCommandArg("appearance", shellLabel("ui.settings.tab.appearance"), [
          { value: "system", label: shellLabel("ui.settings.appearance.system") },
          { value: "light", label: shellLabel("ui.settings.appearance.light") },
          { value: "dark", label: shellLabel("ui.settings.appearance.dark") },
        ]),
      ],
    },
    {
      id: "os.setThemeId",
      label: shellLabel("ui.command.setTheme"),
      scope: "os",
      category: "appearance",
      inPalette: true,
      args: [
        selectCommandArg(
          "themeId",
          shellLabel("ui.settings.tab.theme"),
          themeList.map((theme) => ({ value: theme.id, label: theme.label || theme.id })),
        ),
      ],
    },
    {
      id: "os.setLayout",
      label: shellLabel("ui.command.setLayout"),
      scope: "os",
      category: "layout",
      inPalette: true,
      args: [
        selectCommandArg("layout", shellLabel("ui.settings.tab.layout"), [
          { value: "desktop", label: shellLabel("settings.layout.desktop") },
          { value: "tablet", label: shellLabel("settings.layout.tablet") },
        ]),
      ],
    },
    { id: "os.toggleCompact", label: shellLabel("ui.command.toggleCompact"), scope: "os", category: "layout", inPalette: true, args: [] },
    { id: "os.resetDock", label: shellLabel("ui.settings.resetDock"), scope: "os", category: "layout", inPalette: true, args: [] },
    {
      id: "os.setLocale",
      label: shellLabel("ui.command.setLocale"),
      scope: "os",
      category: "language",
      inPalette: true,
      args: [
        selectCommandArg("locale", shellLabel("ui.settings.tab.language"), [
          { value: "en", label: shellLabel("ui.settings.language.en") },
          { value: "de", label: shellLabel("ui.settings.language.de") },
        ]),
      ],
    },
    {
      id: "os.setTerminology",
      label: shellLabel("ui.command.setTerminology"),
      scope: "os",
      category: "language",
      inPalette: true,
      args: [
        selectCommandArg(
          "terminology",
          shellLabel("ui.settings.tab.terminology"),
          terminologies.map((id) => ({ value: id, label: shellTerminologyLabel(id) })),
        ),
      ],
    },
    {
      id: "os.setExpertise",
      label: shellLabel("ui.command.setExpertise"),
      scope: "os",
      category: "general",
      inPalette: true,
      args: [
        selectCommandArg("expertise", shellLabel("ui.settings.tab.expertise"), [
          { value: "beginner", label: shellLabel("settings.expertise.beginner") },
          { value: "normal", label: shellLabel("settings.expertise.normal") },
          { value: "expert", label: shellLabel("settings.expertise.expert") },
        ]),
      ],
    },
  ];
  return commands.filter((command) => !lockedCommandIds.has(command.id));
}

/** 🎛 Os-scope command ids that are handled locally by the shell — mirrors {@link buildOsCommands}. */
export function dispatchOsCommand(
  commandId: string,
  args: Record<string, unknown> | undefined,
  dispatch: (action: ShellAction) => void,
  dockLayoutStore: DockLayoutStore,
  dockUiStateStore: DockUiStateStore,
  locks: ResolvedShellLocks = EMPTY_SHELL_LOCKS,
): void {
  switch (commandId) {
    case "os.introduceApp":
      dispatch({ type: "SET_INTRODUCTION_STEP", value: 0 });
      return;
    case "os.setAppearance":
      if (locks.appearance) return;
      dispatch({ type: "SET_UI_APPEARANCE", value: (args?.appearance as ElementsSurfaceAppearance) ?? "system" });
      return;
    case "os.setThemeId":
      if (locks.themeId) return;
      if (typeof args?.themeId === "string") dispatch({ type: "SET_UI_THEME_ID", value: args.themeId });
      return;
    case "os.setLayout":
      dispatch({ type: "SET_UI_LAYOUT", value: (args?.layout as UiChromeLayout) ?? "desktop" });
      return;
    case "os.toggleCompact":
      dispatch({ type: "SET_UI_COMPACT", value: (current) => !current });
      return;
    case "os.resetDock":
      dispatch({ type: "RESET_DOCK" });
      dockLayoutStore.reset();
      dockUiStateStore.reset();
      return;
    case "os.setLocale":
      if (locks.locale) return;
      if (typeof args?.locale === "string") {
        setUiLocale(args.locale as UiLocale);
        dispatch({ type: "SET_UI_LOCALE", value: args.locale as UiLocale });
      }
      return;
    case "os.setTerminology":
      if (locks.terminology) return;
      if (typeof args?.terminology === "string") dispatch({ type: "SET_UI_TERMINOLOGY", value: args.terminology });
      return;
    case "os.setExpertise":
      if (typeof args?.expertise === "string") dispatch({ type: "SET_UI_EXPERTISE", value: args.expertise as Expertise });
      return;
    default:
      return;
  }
}

/** @emoji 🎛 Fallback icon for every command-category leaf — categories are open-set strings any plugin/app/mode author can invent, so there's no per-category icon metadata to key off (unlike the framework's own Workbench/Details/Display/Settings categories). */
const COMMAND_CATEGORY_ICON = shellTabIcon("wrench");

/**
 * 🎛 One category's command list (and, if a command is expanded, its staged arg form) as a `TreePanelConfig`
 * — the content a category `PanelTabLeaf` resolves to. A zero-arg command's row fires immediately on click
 * (a plain fire-and-forget tree row, same pattern as {@link groupNamedLayoutsToTreeItems}'s layout rows —
 * no `selectedIds`/`onSelectionChange` override, so it takes `Tree`'s default single-select highlight after
 * firing, same as clicking a Display→Layout row does). An arg-carrying command's row toggles `expandedCommandId`
 * itself (kept as its own exclusive, bespoke state — not `Tree`'s per-row `openStates`, which isn't naturally
 * exclusive across sibling rows) and, when expanded, a synthetic form section (one row per arg, `control`
 * holding the staged input, replacing the old `Field` wrapper since `TreeDataItem` already renders label +
 * description + control in the same two-column layout) is prepended so it renders above the command list —
 * `Tree` reverses top-level `sections` for `direction="up"` (bottom anchors), threaded here via `flowFromAnchor`/
 * `FlowProvider`/`useFlow` down from the hosting `Panel`, not any manual reversal in this function.
 */
export function buildCommandCategoryTree(
  commands: readonly ResolvedCommand[],
  expandedCommandId: string | null,
  stagedArgsByCommandId: Readonly<Record<string, Readonly<Record<string, unknown>>>>,
  onExecute: (entry: ResolvedCommand, executeArgs?: Record<string, unknown>) => void,
  onToggleExpanded: (commandId: string | null) => void,
  onStageArg: (commandId: string, argId: string, value: unknown) => void,
  onResetArgs: (commandId: string) => void,
): TreePanelConfig {
  const argCarryingCommands = commands.filter((entry) => entry.definition.args.length > 0);
  const autoExpandedSingleton = argCarryingCommands.length === 1 ? argCarryingCommands[0] : undefined;
  const expanded = (expandedCommandId ? commands.find((entry) => entry.definition.id === expandedCommandId) : undefined) ?? autoExpandedSingleton;
  const effectiveExpandedId = expanded?.definition.id ?? null;
  const sections: TreeDataSection[] = [];
  if (expanded && expanded.definition.args.length > 0) {
    const staged = stagedArgsByCommandId[expanded.definition.id] ?? {};
    const effective = effectiveActionArgs(expanded.definition.args, staged);
    const missing = missingRequiredArgs(expanded.definition.args, effective);
    sections.push({
      id: `command.category.${expanded.definition.category}.form`,
      items: expanded.definition.args.map(
        (def): TreeDataItem => ({
          id: `command.${expanded.definition.id}.arg.${def.id}`,
          label: def.label,
          description: def.description,
          control: renderStagedArgControl(def, effective[def.id], (value) => onStageArg(expanded.definition.id, def.id, value)),
        }),
      ),
      actions: [
        {
          id: `command-${expanded.definition.id}-execute`,
          icon: <Icon icon="check" size="small" />,
          text: shellLabel("ui.common.execute"),
          disabled: missing.length > 0,
          onClick: () => onExecute(expanded, effective),
        },
        {
          id: `command-${expanded.definition.id}-reset`,
          icon: <Icon icon="undo" size="small" />,
          text: shellLabel("ui.common.reset"),
          onClick: () => onResetArgs(expanded.definition.id),
        },
      ],
    });
  }
  const listCommands = commands.filter((entry) => entry.definition.id !== effectiveExpandedId);
  if (listCommands.length > 0) {
    sections.push({
      id: "command.category.list",
      items: listCommands.map((entry): TreeDataItem => {
        const argCarrying = entry.definition.args.length > 0;
        const icon = entry.definition.iconId && entry.definition.iconId in ICONS ? <Icon icon={entry.definition.iconId as IconName} size="small" /> : undefined;
        if (!argCarrying) return { id: `command.${entry.definition.id}`, label: entry.definition.label, icon, onClick: () => onExecute(entry) };
        return {
          id: `command.${entry.definition.id}`,
          label: `${entry.definition.label}…`,
          icon: <Icon icon={expandedCommandId === entry.definition.id ? "chevron-down" : "chevron-up"} size="small" />,
          onClick: () => onToggleExpanded(expandedCommandId === entry.definition.id ? null : entry.definition.id),
        };
      }),
    });
  }
  return { sections };
}

/**
 * 🎛 One `PanelTabLeaf` per resolved command category — consumers wrap these under the Command branch
 * (`FRAMEWORK_CATEGORY_COMMAND_ID`) on `defaultDock.anchors["bottom-middle"]` so the folded chrome shows
 * a single expandable Command toggle. The command palette's fold/active-category/size/persistence is the
 * generic per-anchor `Panel` state (see `buildPanelProps`); this only builds the category tab leaves.
 * Content is a *lazy* `resolveTree` (mirrors {@link createFrameworkDisplayPanelTabs}'s windows tab) so
 * this array — and therefore `defaultDock`'s own memo — never depends on `expandedCommandId`/
 * `stagedArgsByCommandId`, which change on every keystroke while staging a command argument; `resolveTree`
 * reads those fresh off refs at render time instead.
 */
export function buildCommandCategoryTabs(
  resolvedCommands: readonly ResolvedCommand[],
  categories: readonly { readonly id: string; readonly label: string }[],
  expandedCommandIdRef: React.RefObject<string | null>,
  stagedArgsByCommandIdRef: React.RefObject<Readonly<Record<string, Readonly<Record<string, unknown>>>>>,
  onCommand: (source: ResolvedCommand["source"], commandId: string, args?: Record<string, unknown>) => void,
  dispatch: (action: ShellAction) => void,
): PanelTabNode[] {
  return categories.map((category) => {
    const categoryCommands = resolvedCommands.filter((entry) => entry.definition.category === category.id);
    return singleTreeLeaf({
      id: `command.category.${category.id}`,
      icon: COMMAND_CATEGORY_ICON,
      name: category.label,
      tree: {
        resolveTree: () =>
          buildCommandCategoryTree(
            categoryCommands,
            expandedCommandIdRef.current,
            stagedArgsByCommandIdRef.current,
            (entry, executeArgs) => onCommand(entry.source, entry.definition.id, executeArgs),
            (commandId) => dispatch({ type: "SET_COMMAND_EXPANDED", value: commandId }),
            (commandId, argId, value) => dispatch({ type: "STAGE_COMMAND_ARG", commandId, argId, value }),
            (commandId) => dispatch({ type: "RESET_COMMAND_ARGS", commandId }),
          ),
      },
    });
  });
}
//#endregion 🎛CommandRegistry

/** @emoji 🐢 Structural equality over plain JSON-shaped values (the shape every `UiNode`/`WindowEngagement`/`WindowMeasure` plugin payload takes) — no cycles, no non-JSON types. */
function uiJsonDeepEqual(a: unknown, b: unknown): boolean {
  if (a === b) return true;
  if (typeof a !== "object" || typeof b !== "object" || a === null || b === null) return false;
  if (Array.isArray(a) !== Array.isArray(b)) return false;
  if (Array.isArray(a) && Array.isArray(b)) {
    if (a.length !== b.length) return false;
    for (let index = 0; index < a.length; index += 1) {
      if (!uiJsonDeepEqual(a[index], b[index])) return false;
    }
    return true;
  }
  const aRecord = a as Record<string, unknown>;
  const bRecord = b as Record<string, unknown>;
  const aKeys = Object.keys(aRecord);
  const bKeys = Object.keys(bRecord);
  if (aKeys.length !== bKeys.length) return false;
  for (const key of aKeys) {
    if (!Object.prototype.hasOwnProperty.call(bRecord, key)) return false;
    if (!uiJsonDeepEqual(aRecord[key], bRecord[key])) return false;
  }
  return true;
}

/**
 * @emoji 🐢 Reuses `previous`'s object identity when it's structurally equal to `next` — every plugin
 * `render()`/`utilities()`/`windowEngagements()`/`windowMeasures()` call re-parses a fresh JSON payload
 * every time, even when nothing about that body actually changed (e.g. a camera-only or selection-only
 * action still returns byte-identical panel/utility JSON). Without this, every downstream `React.memo`
 * (see `InterpretedUiNode`) sees a new prop reference every render and can never bail.
 */
export function preserveJsonIdentity<T>(previous: T | undefined, next: T): T {
  return previous !== undefined && uiJsonDeepEqual(previous, next) ? previous : next;
}

/**
 * @emoji 🐢 Builds a `Record<string, V>` from `entries`, reusing `prev`'s per-key value reference where
 * `preserveJsonIdentity` finds no structural change, and reusing `prev` itself (the whole record) when
 * no key actually changed — so a no-op action's `dispatch` doesn't hand `windowUiByKind`/etc. a new
 * object reference and cascade an unmemoizable re-render through every downstream consumer.
 */
export function mergeRecordPreservingIdentity<V>(prev: Readonly<Record<string, V>>, entries: readonly (readonly [string, V])[]): Readonly<Record<string, V>> {
  const next: Record<string, V> = {};
  let changed = Object.keys(prev).length !== entries.length;
  for (const [key, value] of entries) {
    const preserved = preserveJsonIdentity(prev[key], value);
    next[key] = preserved;
    if (preserved !== prev[key]) changed = true;
  }
  return changed ? next : prev;
}

//#region UiRefresh
/** @emoji 🐢 One cached section value keyed by `${section}:${key}` (e.g. `window:2d-overview`, `engagements`) — the hash is what gets sent back to the plugin next time so it can skip re-serializing unchanged content. */
export type UiRefreshCache = Map<string, { readonly hash: string; readonly value: unknown }>;

function uiRefreshWantsWindow(scope: UiDirtyScope, bodyKey: string): boolean {
  return scope.kind === "full" || (scope.kind === "partial" && (scope.windowBodies ?? []).includes(bodyKey));
}
function uiRefreshWantsPanel(scope: UiDirtyScope, bodyKey: string): boolean {
  return scope.kind === "full" || (scope.kind === "partial" && (scope.panelBodies ?? []).includes(bodyKey));
}
function uiRefreshWantsFlag(scope: UiDirtyScope, flag: "engagements" | "measures" | "labels"): boolean {
  return scope.kind === "full" || (scope.kind === "partial" && scope[flag] === true);
}

/**
 * @emoji 🐢 Builds one batched `refresh-ui` request restricted to `scope` — `null` when the scope
 * resolves to nothing worth fetching (`none`, or a `partial` whose fields all miss this app's actual
 * bodies/kinds). Every requested entry carries the host's cached hash so the plugin can omit payloads
 * for sections that didn't change.
 */
export function buildUiRefreshRequest(
  scope: UiDirtyScope,
  windowKinds: readonly { readonly id: string; readonly bodyKey: string }[],
  panelTabLeaves: readonly { readonly kind: PanelTabKind; readonly bodyKey?: string }[],
  viewState: PluginViewState,
  cache: UiRefreshCache,
): PluginUiRefreshRequest | null {
  if (scope.kind === "none") return null;
  const windows = windowKinds.filter((kind) => uiRefreshWantsWindow(scope, kind.bodyKey)).map((kind) => ({ key: kind.id, bodyKey: kind.bodyKey, hash: cache.get(`window:${kind.id}`)?.hash }));
  const panels = panelTabLeaves
    .filter((tab): tab is { readonly kind: string; readonly bodyKey: string } => Boolean(tab.bodyKey) && uiRefreshWantsPanel(scope, tab.bodyKey!))
    .map((tab) => ({ key: panelTabKindId(tab.kind), bodyKey: tab.bodyKey, hash: cache.get(`panel:${panelTabKindId(tab.kind)}`)?.hash }));
  const engagements = uiRefreshWantsFlag(scope, "engagements") ? { hash: cache.get("engagements")?.hash } : undefined;
  const measures = uiRefreshWantsFlag(scope, "measures") ? { hash: cache.get("measures")?.hash } : undefined;
  const labels = uiRefreshWantsFlag(scope, "labels") ? { hash: cache.get("labels")?.hash } : undefined;
  if (windows.length === 0 && panels.length === 0 && !engagements && !measures && !labels) return null;
  return { viewState, windows, panels, engagements, measures, labels };
}

/** @emoji 🐢 Writes every changed section (`value !== undefined`) from a `refresh-ui` response into `cache`; unchanged sections are left as-is since the cached value is still current. */
function applyUiRefreshSectionsToCache(cache: UiRefreshCache, prefix: string, entries: readonly PluginUiRefreshSectionResponse[] | undefined): void {
  for (const entry of entries ?? []) {
    if (entry.value !== undefined) cache.set(`${prefix}:${entry.key}`, { hash: entry.hash, value: entry.value });
  }
}

export function applyUiRefreshResponseToCache(cache: UiRefreshCache, response: PluginUiRefreshResponse): void {
  applyUiRefreshSectionsToCache(cache, "window", response.windows);
  applyUiRefreshSectionsToCache(cache, "panel", response.panels);
  if (response.engagements?.value !== undefined) cache.set("engagements", { hash: response.engagements.hash, value: response.engagements.value });
  if (response.measures?.value !== undefined) cache.set("measures", { hash: response.measures.hash, value: response.measures.value });
  if (response.labels?.value !== undefined) cache.set("labels", { hash: response.labels.hash, value: response.labels.value });
}
//#endregion UiRefresh
//#endregion ShellHelpers

//#region Boot
export async function bootFrameworkOs(options: FrameworkOsBootOptions = {}): Promise<void> {
  const root = document.getElementById(options.rootId ?? "root");
  if (!root) throw new Error("missing #root");
  const locks = resolveShellLocks(mergeShellLockSources(options.brand?.locks, options.locks));
  const defaults = resolveShellDefaults(options.brand, options.defaults);
  if (options.brand) document.title = options.brand.windowTitle;
  bootstrapElementsSurfaceChromeDocument(locks.appearance ?? readStoredUiChromeAppearance());
  // 🐢 No hardcoded fallback app — an omitted `plugins` list boots the shell with an explicit
  // "no plugins available" state rather than silently picking one app.
  createRoot(root).render(<FrameworkOsShell pluginFilter={options.plugin} plugins={options.plugins ?? []} appId={options.appId} locks={locks} defaults={defaults} brand={options.brand} />);
}
//#endregion Boot

//#region ErrorBoundary
class ShellRenderErrorBoundary extends Component<{ readonly children: ReactNode }, { readonly hasError: boolean; readonly message: string }> {
  constructor(props: { readonly children: ReactNode }) {
    super(props);
    this.state = { hasError: false, message: "" };
  }

  static getDerivedStateFromError(error: Error) {
    return { hasError: true, message: error.message };
  }

  render() {
    if (this.state.hasError) {
      return (
        <p className="p-double text-sm text-destructive" role="alert">
          {shellLabel("ui.common.renderError")}: {this.state.message}
        </p>
      );
    }
    return this.props.children;
  }
}
//#endregion ErrorBoundary

//#region FrameworkOsShell
export function FrameworkOsShell({
  pluginFilter,
  plugins,
  appId,
  locks: locksProp,
  defaults: defaultsProp,
  brand,
}: {
  readonly pluginFilter?: string;
  readonly plugins: readonly { readonly pluginId: string; readonly moduleUrl: string }[];
  readonly appId?: string;
  readonly locks?: ResolvedShellLocks;
  readonly defaults?: FrameworkOsDefaults;
  readonly brand?: ShellBrand;
}) {
  // 🏠🧳 `hostConfig` is the sole piece of per-plugin identity knowledge the shell needs (which app id is
  // "landing", which is "host") — every controller id / default panel tab derives from the *loaded*
  // manifest's own `controllerId`/`panelTabs` on those apps below, never from a separate literal.
  const hostConfig = pluginFilter ? resolvePluginHostConfig(pluginFilter) : undefined;
  const studioMode = hostConfig !== undefined;
  const mobile = useMediaQuery(UI_MOBILE_MEDIA_QUERY);
  const locks = locksProp ?? EMPTY_SHELL_LOCKS;
  const defaults = defaultsProp ?? EMPTY_SHELL_DEFAULTS;
  const [shellState, dispatch] = useReducer(shellReducer, undefined, () => initialShellState({ pluginFilter, plugins, locks, defaults }));
  const { loadedPlugins, session, error } = shellState.pluginRuntime;
  const hostPlugin = useMemo(() => (hostConfig ? loadedPlugins.find((entry) => entry.handle.pluginId === hostConfig.pluginId) : undefined), [loadedPlugins, hostConfig]);
  const hostApp = useMemo(() => hostPlugin?.manifest.apps.find((app) => app.id === hostConfig?.hostAppId), [hostPlugin, hostConfig]);
  const landingApp = useMemo(() => hostPlugin?.manifest.apps.find((app) => app.id === hostConfig?.landingAppId) ?? hostPlugin?.manifest.apps[0], [hostPlugin, hostConfig]);
  const landingAppId = hostConfig?.landingAppId;
  const hostAppId = hostConfig?.hostAppId;
  const hostControllerId = hostApp?.controllerId;
  const landingControllerId = landingApp?.controllerId;
  const hostCatalogueTabId = hostApp?.panelTabs[0] ? panelTabKindId(hostApp.panelTabs[0].kind) : undefined;
  const { windowUiByKind, windowEngagementsByKind, windowMeasuresByKind, panelUiByKey, appLabelsOverlay } = shellState.windowUi;
  const { spawnedWindowUi, spawnedWindowEngagements, spawnedWindowMeasures } = shellState.spawnedWindow;
  const { foldedByWindowId: actionPaneFoldedByWindowId, expandedByWindowId: actionPaneExpandedByWindowId, stagedArgsByKey: actionPaneStagedArgsByKey, activeUtilityByWindowId } = shellState.actionPane;
  const { expandedCommandId, stagedArgsByCommandId: commandStagedArgsByCommandId } = shellState.commandPanel;
  const { panels, dockOverride, panelPathMemory, treeOpenStates, activeWindowId, shellLayout, activeExampleId, mobilePanelPath, extraWindowInstances } = shellState.layout;
  const { searchOpen, findOpen, introductionStepIndex, dialog: overlayDialog } = shellState.overlays;
  const { uiAppearance, uiLayout, uiCompact, uiExpertise, uiLocale, uiTerminology, uiThemeId, uiCustomThemes, uiThemeDraft } = shellState.uiPrefs;
  const { syncBackboneUri, syncCardKind, syncDraftPath, syncStatusByDocumentId } = shellState.sync;
  const importStudioInputRef = useRef<HTMLInputElement>(null);
  const refreshGenerationRef = useRef(0);
  const spawnedRefreshGenerationRef = useRef(0);
  const contributorInstancesRef = useRef<Map<string, number>>(new Map());
  const layoutSeedKeyRef = useRef<string | null>(null);
  const noExampleResetInstanceIdRef = useRef<number | null>(null);
  const extraWindowCounterRef = useRef(0);
  // 🐢 Per-instance content-hash cache for the batched `refresh-ui` call, keyed by the same
  // `pluginId:appId:instanceId` triple as `layoutSeedKeyRef` — cleared on session switch below.
  const uiRefreshCacheRef = useRef<UiRefreshCache>(new Map());
  // 🐢 Same idea for the studio-mode spawned-instance view, keyed by spawned instanceId — cleared when
  // the spawned instance itself changes (tracked via `spawnedLayoutSeedRef`).
  const spawnedUiRefreshCacheRef = useRef<UiRefreshCache>(new Map());
  const spawnedLayoutSeedRef = useRef<string | null>(null);
  const openStudioIdRef = useRef<string | null>(null);
  const openInstanceIdRef = useRef<string | null>(null);
  const sessionRef = useRef<ActiveSession | null>(null);
  const uiDevice: ElementsSurfaceDevice = mobile ? "mobile" : uiLayout;
  const uiTheme: UiTheme = useMemo(() => {
    if (uiThemeDraft) return uiThemeDraft;
    const found = builtinUiThemes().find((t) => t.id === uiThemeId) ?? uiCustomThemes[uiThemeId];
    return found ?? readStoredUiChromeThemeSnapshot() ?? semioTheme();
  }, [uiThemeId, uiCustomThemes, uiThemeDraft]);
  /** 🧵 Lazily-created worker running `backbone-worker.ts` — one per shell instance, reused across `openDocument` calls. */
  const backboneWorkerRef = useRef<Worker | null>(null);
  /** 🖋️ Stable per-tab actor id for hub `Hello`/presence frames and op-origin filtering. */
  const shellActorIdRef = useRef<string>(`client-${Math.random().toString(36).slice(2)}`);
  /** 🗂️ Which session/plugin owns each open document id, so incoming worker events route correctly. */
  const openDocumentSessionsRef = useRef<Map<string, { session: ActiveSession; plugin: PluginWasmHandle }>>(new Map());

  const ensureBackboneWorker = useCallback((): Worker => {
    if (backboneWorkerRef.current) return backboneWorkerRef.current;
    const worker = new Worker(new URL("../../product/os/core/js/backbone-worker.ts", import.meta.url), { type: "module" });
    worker.onmessage = (messageEvent: MessageEvent<BackboneWorkerResponse>) => {
      const message = messageEvent.data;
      if (message.kind !== "event") return;
      const entry = openDocumentSessionsRef.current.get(message.documentId);
      if (!entry) return;
      const { event } = message;
      if (event.kind === "status") {
        dispatch({ type: "SET_SYNC_STATUS_FOR_DOCUMENT", documentId: message.documentId, status: { persisted: event.persisted, pendingOps: event.pendingOps, remote: event.remote } });
      } else if (event.kind === "presence") {
        const peersJson = JSON.stringify(event.peers.map((peer) => ({ clientId: peer.actor, name: peer.label ?? peer.actor, selectionCount: 0 })));
        dispatch({
          type: "SET_SESSION",
          value: (current) => (current && current.instanceId === entry.session.instanceId ? { ...current, viewState: { ...current.viewState, presencePeersJson: peersJson } } : current),
        });
      } else if (event.kind === "remoteOps" && entry.plugin.applyOperations) {
        void entry.plugin.applyOperations(entry.session.instanceId, JSON.stringify(event.envelopes));
        const actorUri = `actor://${message.documentId}`;
        postPluginBackboneInbound(entry.session.pluginId, actorUri, [JSON.stringify({ kind: "ops", envelopes: event.envelopes })]);
      } else if (event.kind === "snapshotReplaced" && entry.plugin.loadAppDocument) {
        void entry.plugin.loadAppDocument(entry.session.instanceId, event.envelopeJson);
        const actorUri = `actor://${message.documentId}`;
        postPluginBackboneInbound(entry.session.pluginId, actorUri, [JSON.stringify({ kind: "snapshot", envelopeJson: event.envelopeJson })]);
      } else if (event.kind === "conflict") {
        console.warn("[os-shell] sync conflict", message.documentId, event.message);
      }
    };
    backboneWorkerRef.current = worker;
    return worker;
  }, []);

  const { uri: shellUri, canGoBack, canGoForward, canGoUp, goBack, goForward, goUp, navigate: navigateHistory } = useUIHistory("/", studioMode);

  const namedLayoutStore = useMemo(() => new NamedLayoutStore(session?.app.id ?? "framework-os", createBrowserStoragePort()), [session?.app.id]);
  const dockLayoutStore = useMemo(() => new DockLayoutStore(createBrowserStoragePort(), session?.app.id), [session?.app.id]);
  const dockUiStateStore = useMemo(() => new DockUiStateStore(createBrowserStoragePort(), session?.app.id), [session?.app.id]);

  const registry = useMemo(() => {
    const expanded = expandPluginRegistry(plugins, pluginFilter ? resolvePluginRegistryId(pluginFilter) : undefined, studioMode);
    if (studioMode) return expanded;
    return pluginFilter ? expanded : plugins;
  }, [pluginFilter, plugins, studioMode]);

  // 🐢 Memoized on the raw `panelJson` string (not `session` object identity, which churns every
  // action) so a `session` refresh that leaves `panelJson` untouched reuses the same parsed `panel`
  // object — a prerequisite for any downstream `useMemo`/`React.memo` keyed on `panel` to bail.
  const panel = useMemo(() => (session ? parsePanelState(session.viewState) : null), [session?.viewState.panelJson]);
  const activeSpawnedEntry = panel?.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId);
  const activeAppTitle = appDocumentLabel(activeSpawnedEntry ? resolveDocumentByAppId(loadedPlugins, activeSpawnedEntry.appId, activeSpawnedEntry.document, uiTerminology) : session ? resolveAppDocument(session.app, uiTerminology) : []);

  useEffect(() => {
    sessionRef.current = session;
  }, [session]);

  // 🎓 A brand-owned introduction fully replaces the app's own (already localized, rendered verbatim);
  // its first-run-seen flag is brand-scoped so the branded tour plays even on a device that saw the
  // unbranded one.
  const activeIntroduction = brand?.introduction ?? session?.app.introduction;
  const introductionSeenKey = session ? (brand ? `${brand.id}:${session.app.id}` : session.app.id) : "";
  const activeIntroductionRef = useRef(activeIntroduction);
  activeIntroductionRef.current = activeIntroduction;

  // 🎓 Auto-starts an app's introduction the first time it launches on this device; replaying stays
  // available afterward via the shell-owned Introduce App command.
  useEffect(() => {
    if (!session || !activeIntroduction) return;
    if (readStoredIntroductionSeen(introductionSeenKey)) return;
    dispatch({ type: "SET_INTRODUCTION_STEP", value: 0 });
  }, [session?.app.id, activeIntroduction, introductionSeenKey]);

  // 🧰 Refs so `refreshUi`/`onAction`/`applyHostEffects` can read the current host-owned active utility and
  // active window without re-creating those callbacks on every utility switch.
  const activeUtilityByWindowIdRef = useRef(activeUtilityByWindowId);
  activeUtilityByWindowIdRef.current = activeUtilityByWindowId;
  const activeWindowIdRef = useRef(activeWindowId);
  activeWindowIdRef.current = activeWindowId;
  const actionPaneExpandedByWindowIdRef = useRef(actionPaneExpandedByWindowId);
  actionPaneExpandedByWindowIdRef.current = actionPaneExpandedByWindowId;
  const actionPaneStagedArgsByKeyRef = useRef(actionPaneStagedArgsByKey);
  actionPaneStagedArgsByKeyRef.current = actionPaneStagedArgsByKey;
  const introductionStepIndexRef = useRef(introductionStepIndex);
  introductionStepIndexRef.current = introductionStepIndex;
  // 🎛️ So the command-category leaves' lazily-resolved tree content (built once per resolved-commands
  // change, not per keystroke — see `buildCommandCategoryTabs`) can read the latest expand/staged-arg
  // state without becoming a `defaultDock` memo dependency, which would otherwise persist-write the dock
  // skeleton on every keystroke while staging a command argument.
  const expandedCommandIdRef = useRef(expandedCommandId);
  expandedCommandIdRef.current = expandedCommandId;
  const commandStagedArgsByCommandIdRef = useRef(commandStagedArgsByCommandId);
  commandStagedArgsByCommandIdRef.current = commandStagedArgsByCommandId;

  /** 🧰 Overlays the active window's host-owned `activeUtilityId` onto a view state at plugin-call time. */
  const injectActiveUtility = useCallback((viewState: ViewState, windowId?: string | null): ViewState => {
    const key = windowId ?? activeWindowIdRef.current;
    const utilityId = key ? (activeUtilityByWindowIdRef.current[key] ?? undefined) : undefined;
    return viewState.activeUtilityId === utilityId ? viewState : { ...viewState, activeUtilityId: utilityId };
  }, []);

  useEffect(() => {
    dispatch({ type: "SET_SYNC_BACKBONE_URI", value: null });
    dispatch({ type: "SET_SYNC_CARD_KIND", value: null });
  }, [panel?.activeSpawnedId, session, studioMode]);

  useEffect(() => {
    setPluginBackboneOutboundRelay((uri, messageJson) => {
      const documentId = uri.startsWith("actor://") ? uri.slice("actor://".length) : null;
      if (!documentId) return;
      const worker = backboneWorkerRef.current;
      if (!worker) return;
      let actorMessage: DocumentActorMsg;
      try {
        const parsed = JSON.parse(messageJson) as { kind?: string; envelopes?: unknown; envelopeJson?: string };
        if (parsed.kind === "ops") {
          actorMessage = { kind: "localOps", envelopes: (parsed.envelopes ?? []) as DocumentActorMsg extends { kind: "localOps"; envelopes: infer E } ? E : never };
        } else if (parsed.kind === "snapshot") {
          actorMessage = { kind: "localSnapshot", envelopeJson: parsed.envelopeJson ?? "{}" };
        } else {
          return;
        }
      } catch {
        return;
      }
      const request: BackboneWorkerRequest = { kind: "send", documentId, message: actorMessage };
      worker.postMessage(request);
    });
    return () => setPluginBackboneOutboundRelay(null);
  }, []);

  useEffect(() => {
    const worker = backboneWorkerRef.current;
    return () => worker?.terminate();
  }, []);

  useEffect(() => {
    if (brand) {
      document.title = brand.windowTitle;
    } else if (activeAppTitle) {
      document.title = activeAppTitle;
    }
  }, [activeAppTitle, brand]);

  useEffect(() => {
    let cancelled = false;
    void (async () => {
      try {
        const settled = await Promise.allSettled(registry.map((entry) => loadPluginModuleResilient(entry.pluginId, entry.moduleUrl)));
        const loaded = settled.flatMap((result, index) => {
          if (result.status === "fulfilled" && result.value) return [result.value];
          if (result.status === "rejected") {
            console.error(`[DEBUG] plugin rejected: ${registry[index]?.pluginId}`, result.reason);
          }
          return [];
        });
        if (loaded.length === 0) throw new Error(shellLabel("ui.common.noPluginsLoaded"));
        if (cancelled) return;
        const loadedState = loaded.map((handle) => ({ handle, manifest: handle.manifest }));
        dispatch({ type: "SET_LOADED_PLUGINS", value: loadedState });

        if (hostConfig) {
          const sPlugin = loadedState.find((entry) => entry.handle.pluginId === hostConfig.pluginId);
          const sApp = sPlugin?.manifest.apps.find((app) => app.id === hostConfig.landingAppId) ?? sPlugin?.manifest.apps[0];
          if (!sPlugin || !sApp) throw new Error("host plugin missing landing app");
          const programs = buildStudioPrograms(loadedState);
          const panelState = buildStudioPanelState(programs, []);
          const instanceId = await sPlugin.handle.createApp(sApp.id);
          const viewState: ViewState = {
            activeModeId: sApp.defaultModeId ?? sApp.modes[0]?.id,
            activeWindowKindId: sApp.windowKinds[0]?.id,
            panelJson: panelJsonFromState(panelState),
          };
          dispatch({ type: "SET_SESSION", value: { pluginId: sPlugin.handle.pluginId, instanceId, app: sApp, viewState } });
          dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: sApp.windowKinds[0]?.id ?? null });
          return;
        }

        const registryPluginId = pluginFilter ? resolvePluginRegistryId(pluginFilter) : undefined;
        const primary = (registryPluginId ? loaded.find((entry) => entry.pluginId === registryPluginId) : undefined) ?? loaded[0];
        const primaryApp = appId
          ? (() => {
              const found = primary?.manifest.apps.find((app) => app.id === appId);
              if (!found) throw new Error(`appId "${appId}" does not resolve to any app in the loaded plugin manifest`);
              return found;
            })()
          : (() => {
              const defaultAppId = pluginFilter ? resolvePlaygroundDefaultAppId(pluginFilter) : undefined;
              return (defaultAppId ? primary?.manifest.apps.find((app) => app.id === defaultAppId) : undefined) ?? primary?.manifest.apps[0];
            })();
        if (primary && primaryApp) {
          const instanceId = await primary.createApp(primaryApp.id);
          dispatch({
            type: "SET_SESSION",
            value: {
              pluginId: primary.pluginId,
              instanceId,
              app: primaryApp,
              viewState: {
                activeModeId: primaryApp.defaultModeId ?? primaryApp.modes[0]?.id,
                activeWindowKindId: primaryApp.windowKinds[0]?.id,
              },
            },
          });
          dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: primaryApp.windowKinds[0]?.id ?? null });
        }
      } catch (bootError) {
        if (!cancelled) {
          console.error("[DEBUG] framework os boot failed", bootError);
          dispatch({ type: "SET_ERROR", value: bootError instanceof Error ? bootError.message : String(bootError) });
        }
      }
    })();
    return () => {
      cancelled = true;
    };
  }, [registry, studioMode, hostConfig, appId]);

  const findPluginForAction = useCallback(
    (action: ActionDescriptor) => {
      const byController = loadedPlugins.find((entry) => entry.manifest.apps.some((app) => app.controllerId === action.controllerId));
      if (byController) return byController;
      return loadedPlugins.find((entry) => entry.handle.pluginId === session?.pluginId);
    },
    [loadedPlugins, session?.pluginId],
  );

  const refreshUi = useCallback(
    async (nextSession: ActiveSession, scopeArg: UiDirtyScope = { kind: "full" }) => {
      if (scopeArg.kind === "none") return;
      const generation = ++refreshGenerationRef.current;
      const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === nextSession.pluginId)?.handle;
      if (!plugin) return;
      const layoutSeedKey = `${nextSession.pluginId}:${nextSession.app.id}:${nextSession.instanceId}`;
      // 🐢 A session switch invalidates every cached hash from the previous instance — force a full
      // fetch regardless of what scope this particular call was given.
      let scope = scopeArg;
      if (layoutSeedKeyRef.current !== layoutSeedKey) {
        uiRefreshCacheRef.current = new Map();
        scope = { kind: "full" };
      }
      const cache = uiRefreshCacheRef.current;
      const contributionsJson = buildContributionsJson(loadedPlugins.map((entry) => ({ pluginId: entry.handle.pluginId, manifest: entry.manifest })));
      const viewState: ViewState = injectActiveUtility({ ...nextSession.viewState, contributionsJson, locale: uiLocale, terminology: uiTerminology });
      const panelTabLeaves = flattenPanelTabLeaves(nextSession.app.panelTabs);
      // 🐢 One batched, hash-conditional round trip replaces the old ~12 sequential
      // render/utilities/windowEngagements/windowMeasures/appLabels calls — the plugin omits payloads for
      // any section whose hash still matches what `cache` already holds.
      const request = buildUiRefreshRequest(scope, nextSession.app.windowKinds, panelTabLeaves, viewState, cache);
      if (request) {
        const response = await plugin.refreshUi(nextSession.instanceId, request);
        if (generation !== refreshGenerationRef.current) return;
        const slotContext = {
          plugins: new Map(loadedPlugins.map((entry) => [entry.handle.pluginId, entry.handle])),
          contributorInstances: contributorInstancesRef.current,
          viewState,
        };
        // Resolve external slots on freshly-changed window/panel bodies only, before caching them, so a
        // later no-op refresh reuses the already-resolved cached value instead of re-resolving.
        const resolveIfChanged = async (entry: PluginUiRefreshSectionResponse): Promise<PluginUiRefreshSectionResponse> => (entry.value !== undefined ? { ...entry, value: await resolveExternalSlots(entry.value as UiNode, slotContext) } : entry);
        const [resolvedWindows, resolvedPanels] = await Promise.all([Promise.all((response.windows ?? []).map(resolveIfChanged)), Promise.all((response.panels ?? []).map(resolveIfChanged))]);
        if (generation !== refreshGenerationRef.current) return;
        applyUiRefreshResponseToCache(cache, { ...response, windows: resolvedWindows, panels: resolvedPanels });
      }
      // 🐢 Merge-with-identity-preservation: unrequested/unchanged sections keep exactly the object
      // reference already in `cache` (dispatched from a prior refresh), so `mergeRecordPreservingIdentity`
      // bails on them via reference equality — this is what lets `InterpretedUiNode`'s `React.memo` (and
      // `modeWindows`'s `useMemo`) skip reconciling the whole shell on every interaction.
      dispatch({
        type: "SET_WINDOW_UI_BY_KIND",
        value: (current) =>
          mergeRecordPreservingIdentity(
            current,
            nextSession.app.windowKinds.map((kind) => [kind.id, (cache.get(`window:${kind.id}`)?.value as UiNode | undefined) ?? current[kind.id] ?? { type: "text", value: `${shellLabel("ui.common.loading")}: ${kind.id}` }] as const),
          ),
      });
      const dynamicEngagements = (cache.get("engagements")?.value as Readonly<Record<string, WindowEngagement>> | undefined) ?? {};
      dispatch({
        type: "SET_WINDOW_ENGAGEMENTS_BY_KIND",
        value: (current) => mergeRecordPreservingIdentity(current, Object.entries(dynamicEngagements)),
      });
      const dynamicMeasures = (cache.get("measures")?.value as Readonly<Record<string, readonly WindowMeasure[]>> | undefined) ?? {};
      dispatch({
        type: "SET_WINDOW_MEASURES_BY_KIND",
        value: (current) => mergeRecordPreservingIdentity(current, Object.entries(dynamicMeasures)),
      });
      const appLabelsOverlay = normalizeAppLabelsOverlay(cache.get("labels")?.value as Partial<PluginAppLabelsOverlay> | undefined);
      dispatch({ type: "SET_APP_LABELS_OVERLAY", value: (current) => preserveJsonIdentity(current, appLabelsOverlay) });
      dispatch({
        type: "SET_PANEL_UI_BY_KEY",
        value: (current) =>
          mergeRecordPreservingIdentity(
            current,
            panelTabLeaves
              .filter((tab) => tab.bodyKey)
              .map((tab) => [panelTabKindId(tab.kind), (cache.get(`panel:${panelTabKindId(tab.kind)}`)?.value as UiNode | undefined) ?? current[panelTabKindId(tab.kind)] ?? { type: "text", value: shellLabel("ui.common.loading") }] as const),
          ),
      });
      const windowIds = nextSession.app.windowKinds.map((kind) => kind.id);
      if (layoutSeedKeyRef.current !== layoutSeedKey) {
        layoutSeedKeyRef.current = layoutSeedKey;
        dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: [] });
        extraWindowCounterRef.current = 0;
        dispatch({ type: "SET_SHELL_LAYOUT", value: convertFrameworkLayoutToModeLayout(nextSession.app.defaultLayout, windowIds, appLabelsOverlay) });
        const defaultWindowId = findDefaultActiveWindowKindId(nextSession.app.defaultLayout, nextSession.app.windowKinds);
        if (defaultWindowId) dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: defaultWindowId });
        else if (windowIds[0]) dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: windowIds[0] });
      }
    },
    [injectActiveUtility, loadedPlugins, uiLocale, uiTerminology],
  );

  /** @emoji 🗣️ Keeps already-built window titles (workbench layout, extra spawned windows) in sync with the app-labels overlay on every locale/terminology switch — `refreshUi` only rebuilds `shellLayout` from scratch on a session change, so an existing session's baked-in titles would otherwise go stale. */
  useEffect(() => {
    dispatch({ type: "SET_SHELL_LAYOUT", value: (current) => (current ? retitleWindowLayoutNode(current, appLabelsOverlay) : current) });
    dispatch({
      type: "SET_EXTRA_WINDOW_INSTANCES",
      value: (current) => current.map((entry) => ({ ...entry, title: resolveAppLabel(appLabelsOverlay, "windowKind", entry.windowKindId, entry.title) })),
    });
  }, [appLabelsOverlay]);

  const refreshSpawnedUi = useCallback(
    async (spawned: SpawnedAppEntry, viewState: ViewState, scopeArg: UiDirtyScope = { kind: "full" }) => {
      if (scopeArg.kind === "none") return;
      const generation = ++spawnedRefreshGenerationRef.current;
      const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === spawned.pluginId);
      const plugin = pluginEntry?.handle;
      const app = pluginEntry?.manifest.apps.find((candidate) => candidate.id === spawned.appId);
      if (!plugin || !app) {
        console.warn("[os-shell] refreshSpawnedUi: plugin/app unavailable", { pluginId: spawned.pluginId, appId: spawned.appId });
        dispatch({ type: "SET_SPAWNED_WINDOW_UI", value: { type: "text", value: `Plugin unavailable: ${spawned.pluginId}/${spawned.appId}` } as UiNode });
        dispatch({ type: "SET_SPAWNED_WINDOW_ENGAGEMENTS", value: {} });
        dispatch({ type: "SET_SPAWNED_WINDOW_MEASURES", value: {} });
        return;
      }
      const spawnedSeed = `${spawned.pluginId}:${spawned.appId}:${spawned.instanceId}`;
      if (spawnedLayoutSeedRef.current !== spawnedSeed) {
        spawnedLayoutSeedRef.current = spawnedSeed;
        spawnedUiRefreshCacheRef.current = new Map();
      }
      const cache = spawnedUiRefreshCacheRef.current;
      const contributionsJson = buildContributionsJson(loadedPlugins.map((entry) => ({ pluginId: entry.handle.pluginId, manifest: entry.manifest })));
      const fullViewState: ViewState = injectActiveUtility({ ...viewState, contributionsJson, locale: uiLocale, terminology: uiTerminology }, spawned.id);
      const bodyKey = resolveCanvasBodyKey(app);
      // 🐢 A spawned instance's view is a single body + utilities + engagements + measures (no panels, no
      // labels) — that's already the minimal grouping, so there is no narrower-than-full "partial" scope
      // worth expressing here; only `none` (handled above) short-circuits the request.
      const singleWindowKind = [{ id: bodyKey, bodyKey }];
      const request = buildUiRefreshRequest({ kind: "full" }, singleWindowKind, [], fullViewState, cache);
      if (request) {
        const response = await plugin.refreshUi(spawned.instanceId, request);
        if (generation !== spawnedRefreshGenerationRef.current) return;
        applyUiRefreshResponseToCache(cache, response);
      }
      const ui = (cache.get(`window:${bodyKey}`)?.value as UiNode | undefined) ?? { type: "text", value: shellLabel("ui.common.loading") };
      const dynamicEngagements = (cache.get("engagements")?.value as Readonly<Record<string, WindowEngagement>> | undefined) ?? {};
      const dynamicMeasures = (cache.get("measures")?.value as Readonly<Record<string, readonly WindowMeasure[]>> | undefined) ?? {};
      dispatch({ type: "SET_SPAWNED_WINDOW_UI", value: (current: UiNode | null) => preserveJsonIdentity(current ?? undefined, ui) });
      dispatch({ type: "SET_SPAWNED_WINDOW_ENGAGEMENTS", value: dynamicEngagements });
      dispatch({ type: "SET_SPAWNED_WINDOW_MEASURES", value: dynamicMeasures });
    },
    [injectActiveUtility, loadedPlugins, uiLocale, uiTerminology],
  );

  // 🐢 Keyed on the pluginId/app/instance triple (not `session` object identity) so this only fires on
  // a genuine session switch (app open/spawn/instance change) — every other action already calls
  // `refreshUi` explicitly via `applyHostEffects`, and re-running it here too on every `session` object
  // churn was a second, redundant full-shell refresh cascade per interaction.
  const sessionIdentityKey = session ? `${session.pluginId}:${session.app.id}:${session.instanceId}` : null;
  useEffect(() => {
    const current = sessionRef.current;
    if (!current) return;
    void refreshUi(current).catch((renderError) => {
      console.error("[DEBUG] render failed", renderError);
      dispatch({ type: "SET_ERROR", value: renderError instanceof Error ? renderError.message : String(renderError) });
    });
  }, [loadedPlugins, refreshUi, sessionIdentityKey]);

  useEffect(() => {
    if (!studioMode || !session) {
      dispatch({ type: "SET_SPAWNED_WINDOW_UI", value: null });
      dispatch({ type: "SET_SPAWNED_WINDOW_ENGAGEMENTS", value: {} });
      dispatch({ type: "SET_SPAWNED_WINDOW_MEASURES", value: {} });
      return;
    }
    const activeSpawned = panel?.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId);
    if (!activeSpawned) {
      dispatch({ type: "SET_SPAWNED_WINDOW_UI", value: null });
      dispatch({ type: "SET_SPAWNED_WINDOW_ENGAGEMENTS", value: {} });
      dispatch({ type: "SET_SPAWNED_WINDOW_MEASURES", value: {} });
      return;
    }
    void refreshSpawnedUi(activeSpawned, session.viewState).catch((renderError) => {
      console.error("[DEBUG] spawned render failed", renderError);
      dispatch({ type: "SET_SPAWNED_WINDOW_UI", value: null });
    });
  }, [loadedPlugins, panel, refreshSpawnedUi, session, studioMode]);

  const updateStudioPanel = useCallback((panelState: StudioPanelState) => {
    dispatch({
      type: "SET_SESSION",
      value: (current) => {
        if (!current) return current;
        return { ...current, viewState: { ...current.viewState, panelJson: panelJsonFromState(panelState) } };
      },
    });
  }, []);

  // 🏠🧳 Generic replacement for the old `switchToSApp` — switches to either the host plugin's landing
  // or host app by id (both resolved via `hostConfig`, never a specific app's identity).
  const switchToManagedApp = useCallback(
    async (appId: string, viewState?: ViewState): Promise<ActiveSession | null> => {
      const sPlugin = hostConfig ? loadedPlugins.find((entry) => entry.handle.pluginId === hostConfig.pluginId) : undefined;
      const app = sPlugin?.manifest.apps.find((candidate) => candidate.id === appId);
      if (!sPlugin || !app) return null;
      if (session?.pluginId === sPlugin.handle.pluginId && session.app.id === appId) {
        if (!viewState) return session;
        const nextSession: ActiveSession = { ...session, viewState };
        dispatch({ type: "SET_SESSION", value: nextSession });
        await refreshUi(nextSession);
        return nextSession;
      }
      const instanceId = await sPlugin.handle.createApp(app.id);
      const programs = buildStudioPrograms(loadedPlugins);
      const nextViewState: ViewState = viewState ?? {
        activeModeId: app.defaultModeId ?? app.modes[0]?.id,
        activeWindowKindId: app.windowKinds[0]?.id,
        panelJson: panelJsonFromState(buildStudioPanelState(programs, [])),
      };
      const nextSession: ActiveSession = { pluginId: sPlugin.handle.pluginId, instanceId, app, viewState: nextViewState };
      dispatch({ type: "SET_SESSION", value: nextSession });
      dispatch({
        type: "SET_SHELL_LAYOUT",
        value: convertFrameworkLayoutToModeLayout(
          app.defaultLayout,
          app.windowKinds.map((kind) => kind.id),
          appLabelsOverlay,
        ),
      });
      dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: findDefaultActiveWindowKindId(app.defaultLayout, app.windowKinds) ?? app.windowKinds[0]?.id ?? null });
      if (appId === landingAppId) {
        openStudioIdRef.current = null;
        openInstanceIdRef.current = null;
      }
      await refreshUi(nextSession);
      return nextSession;
    },
    [loadedPlugins, refreshUi, session, appLabelsOverlay, hostConfig, landingAppId],
  );

  const syncSpawnedPluginDocument = useCallback(async (plugin: PluginWasmHandle, app: AppDefinition, pluginInstanceId: number, documentJson: string, viewState: ViewState) => {
    try {
      const document = JSON.parse(documentJson) as Record<string, unknown>;
      await plugin.handleAction(pluginInstanceId, JSON.stringify({ controllerId: app.controllerId, action: "setDocument", args: { document } }), viewState);
    } catch (syncError) {
      console.error("[DEBUG] spawned plugin document sync failed", syncError);
    }
  }, []);

  const ensureSpawnedPlugin = useCallback(
    async (program: StudioProgramEntry, label?: string, osInstanceId?: string, documentJson?: string) => {
      const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === program.pluginId);
      if (!pluginEntry || !session) return;
      const app = pluginEntry.manifest.apps.find((candidate) => candidate.id === program.appId);
      const currentPanel = parsePanelState(session.viewState) ?? buildStudioPanelState(buildStudioPrograms(loadedPlugins), []);
      const existing = osInstanceId ? currentPanel.spawnedApps.find((entry) => entry.id === osInstanceId) : currentPanel.spawnedApps.find((entry) => entry.appId === program.appId && entry.pluginId === program.pluginId);
      if (existing) {
        if (documentJson && app) {
          await syncSpawnedPluginDocument(pluginEntry.handle, app, existing.instanceId, documentJson, session.viewState);
        }
        updateStudioPanel(buildStudioPanelState(currentPanel.programs, currentPanel.spawnedApps, currentPanel.activePanelTab, existing.id));
        return;
      }
      const instanceId = await pluginEntry.handle.createApp(program.appId);
      if (documentJson && app) {
        await syncSpawnedPluginDocument(pluginEntry.handle, app, instanceId, documentJson, session.viewState);
      }
      const spawnedId = osInstanceId ?? `${program.pluginId}-${instanceId}`;
      updateStudioPanel(
        buildStudioPanelState(
          currentPanel.programs,
          [
            ...currentPanel.spawnedApps,
            {
              id: spawnedId,
              pluginId: program.pluginId,
              instanceId,
              appId: program.appId,
              label: label ?? program.label,
              document: program.document,
            },
          ],
          currentPanel.activePanelTab,
          spawnedId,
        ),
      );
    },
    [loadedPlugins, session, syncSpawnedPluginDocument, updateStudioPanel],
  );

  /**
   * 🐚 Consumes a plugin action's typed `requestedEffects: HostEffect[]` (WS-D's `InvocationResponse`) —
   * replaces the deleted `processPluginOps` string-matching. The legacy `setDocument`-mirror
   * backbone-write block is gone entirely: document content sync now flows through
   * `openDocument`/`closeDocument`'s worker-backed `DocumentHost` lifecycle, not a per-op JS mirror.
   */
  const applyHostEffects = useCallback(
    async (effects: readonly HostEffect[], baseSession: ActiveSession, uiScope: UiDirtyScope = { kind: "full" }) => {
      let nextViewState = baseSession.viewState;
      for (const effect of effects) {
        if (effect === "requestSync") continue;
        if ("setPanel" in effect) {
          nextViewState = { ...nextViewState, panelJson: effect.setPanel.panelJson };
          continue;
        }
        if ("setActiveUtility" in effect) {
          // 🧰 A plugin programmatically switched utility: mirror it into the host-owned store slice and,
          // when it targets the active window, into the view state fed to the follow-up refresh.
          const { windowKindId, utilityId } = effect.setActiveUtility;
          dispatch({ type: "SET_ACTIVE_UTILITY", windowId: windowKindId, utilityId: utilityId || null });
          if (windowKindId === activeWindowIdRef.current) nextViewState = { ...nextViewState, activeUtilityId: utilityId || undefined };
          continue;
        }
        if ("openDialog" in effect) {
          // 🗨️ Renders from the active `baseSession.app` — dialogs opened by spawned plugin
          // instances are v1-out-of-scope, mirroring the introduction's active-session-only scope.
          const { dialogId, args } = effect.openDialog;
          if (baseSession.app.dialogs?.some((entry) => entry.id === dialogId)) {
            dispatch({ type: "SET_DIALOG", value: { dialogId, seedArgs: args as Record<string, unknown> | undefined } });
          } else {
            console.error(`[os-shell] openDialog: app ${baseSession.app.id} declares no dialog "${dialogId}"`);
          }
          continue;
        }
        if ("navigate" in effect) {
          navigateHistory(effect.navigate.uri);
          continue;
        }
        if ("openExternalUrl" in effect) {
          window.open(effect.openExternalUrl.url, "_blank", "noopener,noreferrer");
          continue;
        }
        if ("downloadMediaExport" in effect) {
          const { filename, mimeType, data, encoding } = effect.downloadMediaExport;
          downloadMediaExport(filename, mimeType, data, encoding);
          continue;
        }
        if ("iconRenderExport" in effect) {
          for (const item of effect.iconRenderExport.items) {
            try {
              const result = await iconRenderPort.render(item.request as Parameters<typeof iconRenderPort.render>[0]);
              downloadDataUrl(item.filename, result.dataUrl);
            } catch (error) {
              console.error(`icon render export failed for ${item.filename}`, error);
            }
          }
          continue;
        }
        if ("requestFileOpen" in effect) {
          const { accept, readAs, importAction, multiple } = effect.requestFileOpen;
          const opened = await requestFileOpen(accept || ".json,.spatial.json", readAs, multiple);
          if (opened.length > 0) {
            const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === baseSession.pluginId);
            if (pluginEntry) {
              // 📤 Single-file (multiple absent/false): identical to the pre-multi-select shape, one
              // `handleAction` call with `{payload, name}`. Multi-file: one sequential call per selected
              // file, each extending args with `{index, total}` so the plugin can stage/merge imports.
              await dispatchOpenedFiles(opened, importAction, Boolean(multiple), makeEffectDispatchOne(pluginEntry, baseSession, applyHostEffects));
            }
          }
          continue;
        }
        if ("dispatchAction" in effect) {
          // 🔁 Self re-dispatch (D2): re-invokes the same plugin instance with `action` after `delayMs`,
          // without blocking the current `applyHostEffects` pass — `setTimeout` (0 is "next tick") fires
          // the follow-up call and feeds its own `requestedEffects` back through `applyHostEffects`
          // recursively, so a plugin can chain several ticks of staged/progressive work (e.g. a
          // multi-pass reconstruction) purely by re-emitting `dispatchAction` from its own handler.
          const { action: dispatchActionId, args: dispatchArgs, delayMs } = effect.dispatchAction;
          const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === baseSession.pluginId);
          if (pluginEntry) {
            scheduleDispatchAction(dispatchActionId, dispatchArgs as Record<string, unknown> | undefined, delayMs, makeEffectDispatchOne(pluginEntry, baseSession, applyHostEffects));
          }
          continue;
        }
        if ("requestMediaFrames" in effect) {
          // 🎞️ D5: decodes a video (file picker, or `payload` bytes already in hand from a drop zone)
          // and fans sampled frames + a completion marker out through the same `dispatchOne` path as
          // every other effect branch — see `runRequestMediaFrames` for the Tier 1 (WebCodecs)/Tier 2
          // (`<video>` seek-and-capture)/fallback decision tree.
          const { accept, payload, frameAction, doneAction, fallbackAction, sampleStride, maxFrames, maxLongEdgePx, fpsHint, args } = effect.requestMediaFrames;
          const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === baseSession.pluginId);
          if (pluginEntry) {
            await runRequestMediaFrames(
              {
                frameAction,
                doneAction,
                fallbackAction,
                sampleStride: sampleStride ?? 0,
                maxFrames: maxFrames ?? 0,
                maxLongEdgePx: maxLongEdgePx ?? 0,
                fpsHint: fpsHint ?? 0,
                args: args as Record<string, unknown> | undefined,
              },
              accept,
              payload,
              makeEffectDispatchOne(pluginEntry, baseSession, applyHostEffects),
            );
          }
          continue;
        }
        if ("spawnPluginInstance" in effect) {
          const { programId, appId, osInstanceId, label, documentJson } = effect.spawnPluginInstance;
          const currentPanel = parsePanelState(nextViewState) ?? buildStudioPanelState(buildStudioPrograms(loadedPlugins), []);
          const program = currentPanel.programs.find((entry) => entry.programId === programId && entry.appId === appId) ?? currentPanel.programs.find((entry) => entry.programId === programId);
          if (program) await ensureSpawnedPlugin(program, label, osInstanceId, documentJson);
          continue;
        }
        if ("openPluginInstance" in effect) {
          const { programId, appId, osInstanceId } = effect.openPluginInstance;
          const currentPanel = parsePanelState(nextViewState) ?? buildStudioPanelState(buildStudioPrograms(loadedPlugins), []);
          const program = currentPanel.programs.find((entry) => entry.programId === programId && entry.appId === appId) ?? currentPanel.programs.find((entry) => entry.programId === programId);
          if (program) {
            await ensureSpawnedPlugin(program, undefined, osInstanceId, undefined);
            if (osInstanceId && openStudioIdRef.current) {
              openInstanceIdRef.current = osInstanceId;
              navigateHistory(`/studios/${openStudioIdRef.current}/instances/${osInstanceId}`);
            }
          } else {
            console.warn(
              "[os-shell] openPluginInstance: no program matches",
              { programId, appId },
              "available:",
              currentPanel.programs.map((entry) => `${entry.programId}/${entry.appId}`),
            );
          }
          continue;
        }
      }
      const nextSession = { ...baseSession, viewState: nextViewState };
      const isSpawnedPluginSession = studioMode && session && baseSession.pluginId !== session.pluginId;
      dispatch({
        type: "SET_SESSION",
        value: (current) => {
          if (!current) return nextSession;
          if (isSpawnedPluginSession) return current.viewState === nextViewState ? current : { ...current, viewState: nextViewState };
          if (current.instanceId !== nextSession.instanceId) return current;
          // 🐢 Preserve `current`'s identity when the viewState didn't actually change — otherwise every
          // action mints a new `session` object, which cascades into a new `onAction` identity, which
          // busts every memo keyed on it (windows, panels, the boot-refresh effect below) even when
          // nothing about the session changed.
          return current.viewState === nextViewState ? current : { ...current, viewState: nextViewState };
        },
      });
      if (isSpawnedPluginSession) {
        const spawned = parsePanelState(nextViewState)?.spawnedApps.find((entry) => entry.pluginId === baseSession.pluginId && entry.instanceId === baseSession.instanceId);
        if (spawned) await refreshSpawnedUi(spawned, nextViewState, uiScope);
      } else if (session?.instanceId === nextSession.instanceId || baseSession.instanceId === nextSession.instanceId) {
        await refreshUi(nextSession, uiScope);
      }
    },
    [ensureSpawnedPlugin, loadedPlugins, navigateHistory, refreshSpawnedUi, refreshUi, session, studioMode],
  );

  const applyShellUri = useCallback(
    async (uri: string, preservedViewState?: ViewState) => {
      const currentSession = sessionRef.current;
      if (!hostConfig || !currentSession || loadedPlugins.length === 0) return;
      const path = uri.split("?")[0] ?? "/";
      const studioPath = parseStudioShellPath(path);
      const sPlugin = loadedPlugins.find((entry) => entry.handle.pluginId === hostConfig.pluginId)?.handle;
      if (!sPlugin) return;
      if (!studioPath) {
        openStudioIdRef.current = null;
        openInstanceIdRef.current = null;
        if (currentSession.app.id !== hostConfig.landingAppId) await switchToManagedApp(hostConfig.landingAppId, preservedViewState);
        return;
      }
      const { studioId, instanceId } = studioPath;
      const studioSession = currentSession.app.id === hostConfig.hostAppId ? currentSession : await switchToManagedApp(hostConfig.hostAppId, preservedViewState);
      if (!studioSession) return;
      const studioControllerId = studioSession.app.controllerId;
      if (openStudioIdRef.current !== studioId) {
        openStudioIdRef.current = studioId;
        openInstanceIdRef.current = null;
        await sPlugin.handleAction(studioSession.instanceId, JSON.stringify({ controllerId: studioControllerId, action: "openStudio", args: { studioId } }), studioSession.viewState);
        await refreshUi(studioSession);
      }
      if (openInstanceIdRef.current === (instanceId ?? null)) return;
      openInstanceIdRef.current = instanceId ?? null;
      if (instanceId) {
        const response = await sPlugin.handleAction(studioSession.instanceId, JSON.stringify({ controllerId: studioControllerId, action: "openInstance", args: { instanceId } }), studioSession.viewState);
        await applyHostEffects(response.requestedEffects ?? [], studioSession, resolveUiDirtyScope(response.uiScope));
      } else {
        const response = await sPlugin.handleAction(studioSession.instanceId, JSON.stringify({ controllerId: studioControllerId, action: "closeFocusedInstance" }), studioSession.viewState);
        const currentPanel = parsePanelState(studioSession.viewState) ?? buildStudioPanelState(buildStudioPrograms(loadedPlugins), []);
        updateStudioPanel(buildStudioPanelState(currentPanel.programs, currentPanel.spawnedApps, currentPanel.activePanelTab, undefined));
        await applyHostEffects(response.requestedEffects ?? [], studioSession, resolveUiDirtyScope(response.uiScope));
      }
    },
    [applyHostEffects, loadedPlugins, refreshUi, hostConfig, switchToManagedApp, updateStudioPanel],
  );

  useEffect(() => {
    if (!studioMode || loadedPlugins.length === 0) return;
    void applyShellUri(shellUri).catch((uriError) => {
      console.error("[DEBUG] shell uri apply failed", uriError);
    });
  }, [applyShellUri, loadedPlugins.length, shellUri, studioMode]);

  const resolveSyncTargetSession = useCallback((): ActiveSession | null => {
    if (!session) return null;
    if (studioMode && panel?.activeSpawnedId) {
      const spawned = panel.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId);
      if (spawned) {
        const app = loadedPlugins.find((entry) => entry.handle.pluginId === spawned.pluginId)?.manifest.apps.find((candidate) => candidate.id === spawned.appId);
        if (app) return { pluginId: spawned.pluginId, instanceId: spawned.instanceId, app, viewState: session.viewState };
      }
    }
    return session;
  }, [loadedPlugins, panel, session, studioMode]);

  /**
   * 🧵 `openDocument(ref, bindings)` — replaces `attachSyncBackbone`'s URI-string mirror. Spins up (or
   * reuses) `backbone-worker.ts`, tells it to open the document, subscribes to its postMessage events,
   * and calls the plugin instance's `attachBackbone`/`loadAppDocument` WIT-exported methods (WS-D) so
   * the plugin-side store starts pumping through the same logical channel. The `actor://<documentId>`
   * uri mirrors `framework/sync`'s `ChannelBackbone::pair` convention on the Rust side.
   *
   * Full loop note: this wires the main-thread half of the contract. The remaining hop — the
   * sandboxed plugin's own `backbone-send`/`backbone-poll` WIT host-import calls relaying through its
   * dedicated plugin worker, through this main thread, into `backbone-worker.ts` — is
   * `framework/product/os/dev/script.ts`'s `pluginWorkerSource` responsibility (dev workflow, deferred
   * per this session's priority order if not otherwise completed); see that file's own notes.
   */
  const openDocument = useCallback(
    async (ref: { readonly documentId: string; readonly schema: string }, bindings: readonly PersistenceBinding[]) => {
      const targetSession = resolveSyncTargetSession();
      if (!targetSession) return;
      const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === targetSession.pluginId)?.handle;
      if (!plugin) return;
      const worker = ensureBackboneWorker();
      openDocumentSessionsRef.current.set(ref.documentId, { session: targetSession, plugin });
      const request: BackboneWorkerRequest = {
        kind: "open",
        documentId: ref.documentId,
        schema: ref.schema,
        bindings,
        watchExternal: true,
        actor: shellActorIdRef.current,
      };
      worker.postMessage(request);
      const uri = `actor://${ref.documentId}`;
      if (plugin.attachBackbone) await plugin.attachBackbone(targetSession.instanceId, uri);
      dispatch({ type: "SET_SYNC_BACKBONE_URI", value: uri });
      dispatch({ type: "SET_SYNC_CARD_KIND", value: null });
    },
    [loadedPlugins, resolveSyncTargetSession],
  );

  const closeDocument = useCallback((documentId: string) => {
    const entry = openDocumentSessionsRef.current.get(documentId);
    if (entry?.plugin.detachBackbone) void entry.plugin.detachBackbone(entry.session.instanceId);
    openDocumentSessionsRef.current.delete(documentId);
    const request: BackboneWorkerRequest = { kind: "close", documentId };
    backboneWorkerRef.current?.postMessage(request);
  }, []);

  /** @deprecated superseded by {@link openDocument}; kept as a thin URI-parsing adapter only for the
   * existing sync-card UI (`onAction`'s `attach` handler below), which still collects a single uri
   * from file/folder/remote pickers — translates that uri into an `OsDocumentRef` + `PersistenceBinding`. */
  const attachSyncBackbone = useCallback(
    async (uri: string) => {
      const targetSession = resolveSyncTargetSession();
      if (!targetSession) return;
      const documentId = syncDocumentId(targetSession, panel, studioMode);
      const bindings: PersistenceBinding[] = uri.startsWith("remote://")
        ? (() => {
            const rest = uri.slice("remote://".length);
            const slash = rest.indexOf("/");
            const baseUrl = slash > 0 ? `http://${rest.slice(0, slash)}` : `http://${rest}`;
            const studioId = slash > 0 ? rest.slice(slash + 1) || "default" : "default";
            return [{ kind: "hub", baseUrl, studioId }];
          })()
        : uri.startsWith("folder://")
          ? [{ kind: "folder", path: uri.slice("folder://".length) }]
          : uri.startsWith("file://")
            ? [{ kind: "folder", path: uri.slice("file://".length).replace(/\/[^/]*$/, "") }]
            : [];
      await openDocument({ documentId, schema: targetSession.app.document.join(".") }, bindings);
    },
    [openDocument, panel, resolveSyncTargetSession, studioMode],
  );

  const detachSyncBackbone = useCallback(() => {
    if (syncBackboneUri) closeDocument(syncBackboneUri.replace(/^actor:\/\//, ""));
    dispatch({ type: "SET_SYNC_BACKBONE_URI", value: null });
    dispatch({ type: "SET_SYNC_CARD_KIND", value: null });
  }, [closeDocument, syncBackboneUri]);

  const spawnProgram = useCallback(
    async (program: StudioProgramEntry) => {
      const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === program.pluginId);
      if (!pluginEntry || !session) return;
      const instanceId = await pluginEntry.handle.createApp(program.appId);
      const currentPanel = parsePanelState(session.viewState) ?? buildStudioPanelState(buildStudioPrograms(loadedPlugins), []);
      const spawnedId = `${program.pluginId}-${instanceId}`;
      updateStudioPanel(
        buildStudioPanelState(
          currentPanel.programs,
          [
            ...currentPanel.spawnedApps,
            {
              id: spawnedId,
              pluginId: program.pluginId,
              instanceId,
              appId: program.appId,
              label: program.label,
              document: program.document,
            },
          ],
          currentPanel.activePanelTab,
          spawnedId,
        ),
      );
    },
    [loadedPlugins, session, updateStudioPanel],
  );

  const onAction = useCallback(
    (action: ActionDescriptor) => {
      if (!session) return;

      // 🎓 First-run walkthrough (mirrors setActiveUtility below): fully shell-intercepted, resets
      // playback to the first step, never forwarded to the plugin.
      if (action.action === START_INTRODUCTION_ACTION_ID) {
        dispatch({ type: "SET_INTRODUCTION_STEP", value: 0 });
        return;
      }
      const introductionStep = introductionStepIndexRef.current != null ? (activeIntroductionRef.current?.steps[introductionStepIndexRef.current] ?? null) : null;
      const advanceIntroductionStep = () => {
        const stepIndex = introductionStepIndexRef.current;
        const introduction = activeIntroductionRef.current;
        if (stepIndex == null || !introduction) return;
        if (stepIndex >= introduction.steps.length - 1) {
          dispatch({ type: "SET_INTRODUCTION_STEP", value: null });
          writeStoredIntroductionSeen(introductionSeenKey);
        } else {
          dispatch({ type: "SET_INTRODUCTION_STEP", value: stepIndex + 1 });
        }
      };

      // 🧰 Utility activation (P5): host-owned session state, never a document op. Re-clicking the active
      // utility (or an empty utilityId) deactivates. We resolve the target window from the descriptor's tagged
      // `windowId` (see `tagSetActiveUtilityWindow`), falling back to the active window, update the store,
      // then forward the resolved utility to the plugin so it can clear/prepare scratch.
      if (action.action === SET_ACTIVE_UTILITY_ACTION_ID) {
        const args = typeof action.args === "object" && action.args != null ? (action.args as { utilityId?: unknown; windowId?: unknown }) : {};
        const windowId = typeof args.windowId === "string" && args.windowId ? args.windowId : (activeWindowIdRef.current ?? "");
        if (!windowId) return;
        const requested = typeof args.utilityId === "string" ? args.utilityId : "";
        const next = resolveUtilityActivation(activeUtilityByWindowIdRef.current[windowId], requested);
        dispatch({ type: "SET_ACTIVE_UTILITY", windowId, utilityId: next });
        if (introductionStep?.advance.kind === "utility" && next && introductionStep.advance.id === next) advanceIntroductionStep();
        const pluginEntry = findPluginForAction(action);
        const plugin = pluginEntry?.handle;
        if (plugin) {
          const viewState: ViewState = { ...session.viewState, activeUtilityId: next ?? undefined };
          const forwarded: ActionDescriptor = { controllerId: action.controllerId, action: action.action, args: { utilityId: next } };
          void plugin
            .handleAction(session.instanceId, JSON.stringify(forwarded), viewState)
            .then((response) => applyHostEffects(response.requestedEffects ?? [], { ...session, viewState }, resolveUiDirtyScope(response.uiScope)))
            .catch((utilityError) => console.error("[DEBUG] setActiveUtility failed", utilityError));
        }
        return;
      }

      if (introductionStep?.advance.kind === "action" && introductionStep.advance.id === action.action) advanceIntroductionStep();

      if (action.controllerId === FRAMEWORK_SYNC_CONTROLLER_ID) {
        if (action.action === "selectFile") {
          dispatch({ type: "SET_SYNC_CARD_KIND", value: "file" });
          dispatch({ type: "SET_SYNC_DRAFT_PATH", value: syncBackboneUri?.startsWith("file://") ? syncBackboneUri.slice("file://".length) : "" });
          return;
        }
        if (action.action === "selectFolder") {
          dispatch({ type: "SET_SYNC_CARD_KIND", value: "folder" });
          dispatch({ type: "SET_SYNC_DRAFT_PATH", value: syncBackboneUri?.startsWith("folder://") ? syncBackboneUri.slice("folder://".length) : "" });
          return;
        }
        if (action.action === "selectRemote") {
          dispatch({ type: "SET_SYNC_CARD_KIND", value: "remote" });
          const remote = syncBackboneUri?.startsWith("remote://") ? syncBackboneUri.slice("remote://".length) : "";
          dispatch({ type: "SET_SYNC_DRAFT_PATH", value: remote });
          return;
        }
        if (action.action === "attach") {
          const path = typeof action.args === "object" && action.args != null && "path" in action.args ? String((action.args as { path?: string }).path ?? "") : syncDraftPath;
          if (!path.trim()) return;
          const uri =
            action.args && typeof action.args === "object" && "kind" in action.args
              ? String((action.args as { kind?: string }).kind) === "remote"
                ? (() => {
                    const [hostPort, ...rest] = path.split("/");
                    const [studioId, documentId] = rest.length >= 2 ? [rest[0], rest.slice(1).join("/")] : ["default", rest[0] || syncDocumentId(session, panel, studioMode)];
                    return buildRemoteBackboneUri(hostPort ?? "127.0.0.1:8787", studioId, documentId);
                  })()
                : String((action.args as { kind?: string }).kind) === "folder"
                  ? buildFolderBackboneUri(path)
                  : buildFileBackboneUri(path)
              : buildFileBackboneUri(path);
          void attachSyncBackbone(uri);
          return;
        }
        if (action.action === "detach") {
          void detachSyncBackbone();
          return;
        }
        return;
      }

      if (studioMode && action.controllerId === landingControllerId && action.action === "importStudio") {
        importStudioInputRef.current?.click();
        return;
      }

      if (studioMode && action.action === "spawnApp" && action.controllerId !== hostControllerId) {
        const programId = typeof action.args === "object" && action.args != null && "programId" in action.args ? String((action.args as { programId?: string }).programId ?? "") : "";
        const pluginId = typeof action.args === "object" && action.args != null && "pluginId" in action.args ? String((action.args as { pluginId?: string }).pluginId ?? "") : "";
        const currentPanel = parsePanelState(session.viewState);
        const program = currentPanel?.programs.find((entry) => entry.programId === programId && entry.pluginId === pluginId);
        if (program) void spawnProgram(program);
        return;
      }

      if (studioMode && action.controllerId === hostControllerId && action.action === "setActivePanelTab") {
        const tabId = typeof action.args === "object" && action.args != null && "tabId" in action.args ? String((action.args as { tabId?: string }).tabId ?? hostCatalogueTabId ?? "") : (hostCatalogueTabId ?? "");
        const currentPanel = parsePanelState(session.viewState) ?? buildStudioPanelState(buildStudioPrograms(loadedPlugins), []);
        updateStudioPanel(buildStudioPanelState(currentPanel.programs, currentPanel.spawnedApps, tabId, currentPanel.activeSpawnedId));
        return;
      }

      const pluginEntry = findPluginForAction(action);
      const plugin = pluginEntry?.handle;
      if (!plugin) return;

      const targetSession =
        studioMode && action.controllerId !== session.app.controllerId
          ? (() => {
              const spawned = panel?.spawnedApps.find((entry) => {
                const app = loadedPlugins.find((p) => p.handle.pluginId === entry.pluginId)?.manifest.apps.find((a) => a.id === entry.appId);
                return app?.controllerId === action.controllerId;
              });
              if (!spawned) return session;
              const app = loadedPlugins.find((p) => p.handle.pluginId === spawned.pluginId)?.manifest.apps.find((a) => a.id === spawned.appId);
              if (!app) return session;
              return { pluginId: spawned.pluginId, instanceId: spawned.instanceId, app, viewState: session.viewState };
            })()
          : session;

      // 🚫 The old `setDocument` → `patchAppSource` mirror (spawned-instance content write-back on the
      // os document) is deleted — app content no longer embeds on the os document at all
      // (`OsAppInstance.document` is now just an `OsDocumentRef` handle). A spawned instance's content
      // sync now goes through its own `openDocument`-opened `DocumentHost` channel, same as any other
      // document; there is no host-side JS mirroring step anymore.
      const dispatchViewState = injectActiveUtility(targetSession.viewState);
      return plugin
        .handleAction(targetSession.instanceId, JSON.stringify(action), dispatchViewState)
        .then((response) => applyHostEffects(response.requestedEffects ?? [], { ...targetSession, viewState: dispatchViewState }, resolveUiDirtyScope(response.uiScope)))
        .catch((actionError) => {
          console.error("[DEBUG] action failed", actionError);
        });
    },
    [
      applyHostEffects,
      attachSyncBackbone,
      detachSyncBackbone,
      findPluginForAction,
      injectActiveUtility,
      loadedPlugins,
      panel,
      session,
      spawnProgram,
      studioMode,
      syncBackboneUri,
      syncDraftPath,
      updateStudioPanel,
      hostControllerId,
      landingControllerId,
      hostCatalogueTabId,
      introductionSeenKey,
    ],
  );

  const onActionRef = useRef(onAction);
  useEffect(() => {
    onActionRef.current = onAction;
  }, [onAction]);

  // 🐢 `onAction`'s own identity churns every action (its deps include `session`, `panel`, …). Render
  // trees built from `UiNode`s only need a *callable* action dispatcher, not a fresh one each time —
  // route them through this permanently-stable ref indirection so `interpretUiNode`'s `React.memo`
  // (and any `useMemo` keyed on the dispatcher passed to it) can actually bail.
  const onActionStable = useCallback((action: Parameters<typeof onAction>[0]) => onActionRef.current(action), []);

  const studioSessionActive = studioMode && session?.app.id === hostAppId;
  // 🏠🧳 Once `studioSessionActive` is true, `session.app` *is* the host app, so its own self-declared
  // `controllerId` is the right value — no separate app-identity lookup needed.
  const studioSessionControllerId = studioSessionActive ? session?.app.controllerId : undefined;
  useEffect(() => {
    if (!studioSessionActive || !studioSessionControllerId || typeof window === "undefined") return;
    const identity = presenceClientIdentity();
    const beat = () => onActionRef.current({ controllerId: studioSessionControllerId, action: "presenceHeartbeat", args: identity });
    const initial = window.setTimeout(beat, 1000);
    const timer = window.setInterval(beat, PRESENCE_HEARTBEAT_INTERVAL_MS);
    return () => {
      window.clearTimeout(initial);
      window.clearInterval(timer);
    };
  }, [studioSessionActive, studioSessionControllerId]);

  usePanelChromeHotkeys({
    onToggle: (anchor) => dispatch({ type: "SET_PANEL_VISIBLE", anchor, value: (visible) => !visible }),
  });

  useElementsSurfaceChrome({ appearance: uiAppearance, device: uiDevice, expertise: uiExpertise, compact: uiCompact });

  //#region 💾 uiPrefs persistence (skips localStorage writes for any locked preference)
  useEffect(() => {
    if (!locks.appearance) writeStoredUiChromeAppearance(uiAppearance);
    writeStoredUiChromeLayout(uiLayout);
    writeStoredUiChromeCompact(uiCompact);
    writeStoredUiChromeExpertise(uiExpertise);
    if (!locks.locale) writeStoredUiChromeLocale(uiLocale);
    void setUiLocale(uiLocale);
    if (!locks.terminology) writeStoredUiChromeTerminology(uiTerminology);
    setActiveUiTheme(uiTheme);
    if (!locks.themeId) {
      writeStoredUiChromeThemeSnapshot(uiTheme);
      writeStoredUiChromeThemeId(uiThemeId);
    }
    writeStoredUiCustomThemes(uiCustomThemes);
  }, [uiAppearance, uiLayout, uiCompact, uiExpertise, uiLocale, uiTerminology, uiTheme, uiThemeId, uiCustomThemes, locks]);
  //#endregion

  useActionHotkey(
    "mod+[",
    useCallback(() => {
      if (canGoBack) goBack();
    }, [canGoBack, goBack]),
  );
  useActionHotkey(
    "mod+]",
    useCallback(() => {
      if (canGoForward) goForward();
    }, [canGoForward, goForward]),
  );
  useActionHotkey(
    "mod+up",
    useCallback(() => {
      if (canGoUp) goUp();
    }, [canGoUp, goUp]),
  );
  useActionHotkey(
    "mod+p",
    useCallback(() => dispatch({ type: "SET_SEARCH_OPEN", value: (open) => !open }), []),
  );
  useActionHotkey(
    "mod+f",
    useCallback(() => dispatch({ type: "SET_FIND_OPEN", value: (open) => !open }), []),
  );

  const applyNamedLayout = useCallback(
    (layout: WindowLayout) => {
      if (!session) return;
      const windowIds = session.app.windowKinds.map((kind) => kind.id);
      dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: [] });
      extraWindowCounterRef.current = 0;
      dispatch({ type: "SET_SHELL_LAYOUT", value: convertFrameworkLayoutToModeLayout(layout, windowIds, appLabelsOverlay) });
      const defaultWindowId = findDefaultActiveWindowKindId(layout, session.app.windowKinds);
      if (defaultWindowId) dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: defaultWindowId });
    },
    [session, appLabelsOverlay],
  );

  const applyModeChange = useCallback(
    (modeId: string) => {
      dispatch({
        type: "SET_SESSION",
        value: (current) => {
          if (!current) return current;
          const layout = resolveLayoutForMode(current.app, modeId);
          if (layout) {
            dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: [] });
            extraWindowCounterRef.current = 0;
            dispatch({
              type: "SET_SHELL_LAYOUT",
              value: convertFrameworkLayoutToModeLayout(
                layout,
                current.app.windowKinds.map((kind) => kind.id),
                appLabelsOverlay,
              ),
            });
            const defaultWindowId = findDefaultActiveWindowKindId(layout, current.app.windowKinds);
            if (defaultWindowId) dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: defaultWindowId });
          }
          return { ...current, viewState: { ...current.viewState, activeModeId: modeId } };
        },
      });
    },
    [appLabelsOverlay],
  );

  const handleTemplateDrop = useCallback(
    (payload: WindowTemplateDropPayload, target: ModeCanvasDropTarget) => {
      if (!session) return;
      const kind = session.app.windowKinds.find((entry) => entry.id === payload.windowKindId);
      if (!kind) return;
      extraWindowCounterRef.current += 1;
      const instanceId = `${payload.windowKindId}-${extraWindowCounterRef.current}`;
      dispatch({
        type: "SET_EXTRA_WINDOW_INSTANCES",
        value: (current) => [...current, { id: instanceId, windowKindId: payload.windowKindId, title: resolveAppLabel(appLabelsOverlay, "windowKind", kind.id, kind.label) }],
      });
      dispatch({
        type: "SET_SHELL_LAYOUT",
        value: (current) => {
          const base =
            current ??
            convertFrameworkLayoutToModeLayout(
              session.app.defaultLayout,
              session.app.windowKinds.map((entry) => entry.id),
              appLabelsOverlay,
            );
          return insertWindowAtDropZone(base, instanceId, target);
        },
      });
      dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: instanceId });
    },
    [appLabelsOverlay, session],
  );

  const displayHostRef = useRef<DisplayHostApi | null>(null);
  const displayHost = useNamedLayoutHost({
    appId: session?.app.id ?? "framework-os",
    windowKinds: session?.app.windowKinds.map((kind) => ({ ...kind, label: resolveAppLabel(appLabelsOverlay, "windowKind", kind.id, kind.label) })) ?? [],
    builtinLayouts: session?.app.namedLayouts ?? [],
    currentLayout: captureCurrentFrameworkLayout(shellLayout, session?.app.defaultLayout),
    onApplyLayout: applyNamedLayout,
    namedLayoutStore,
  });
  displayHostRef.current = displayHost;

  //#region 🔖ThemeMutators
  const uiThemeBase = uiThemeDraft ?? uiTheme;
  const uiThemeDirty = uiThemeDraft !== null;
  const uiThemeList = useMemo((): readonly UiTheme[] => [...builtinUiThemes(), ...Object.values(uiCustomThemes)], [uiCustomThemes]);
  const osCommands = useMemo(
    () => buildOsCommands(uiThemeList, [UI_TERMINOLOGY_NATIVE, ...(session?.app.terminologies ?? [])], activeIntroduction != null, locks),
    [uiThemeList, session?.app.terminologies, activeIntroduction, uiLocale, uiTerminology, locks],
  );

  const draftThemePatch = useCallback(
    (patch: (next: UiTheme) => void) => {
      const next = structuredClone(uiThemeBase);
      patch(next);
      dispatch({ type: "SET_UI_THEME_DRAFT", value: next });
    },
    [uiThemeBase],
  );

  const setThemeId = useCallback((id: string) => {
    dispatch({ type: "SET_UI_THEME_DRAFT", value: null });
    dispatch({ type: "SET_UI_THEME_ID", value: id });
  }, []);

  const setThemeColor = useCallback(
    (key: string, hex: string) =>
      draftThemePatch((next) => {
        next.colors[key] = hex;
      }),
    [draftThemePatch],
  );
  const setThemeSpacing = useCallback(
    (key: string, value: string) =>
      draftThemePatch((next) => {
        next.spacing[key] = value;
      }),
    [draftThemePatch],
  );
  const setThemeFontStack = useCallback(
    (key: string, value: string) =>
      draftThemePatch((next) => {
        next.fontStacks[key] = value;
      }),
    [draftThemePatch],
  );
  const setThemeStroke = useCallback(
    (key: string, value: number | number[]) =>
      draftThemePatch((next) => {
        next.strokes[key] = value;
      }),
    [draftThemePatch],
  );
  const setThemeRadius = useCallback(
    (key: string, value: number) =>
      draftThemePatch((next) => {
        next.radii[key] = value;
      }),
    [draftThemePatch],
  );
  const setThemeOpacity = useCallback(
    (key: string, value: number) =>
      draftThemePatch((next) => {
        next.opacities[key] = value;
      }),
    [draftThemePatch],
  );
  const setThemeMetric = useCallback(
    (section: string, key: string, value: number | number[]) =>
      draftThemePatch((next) => {
        next.metrics[section] = { ...(next.metrics[section] ?? {}), [key]: value };
      }),
    [draftThemePatch],
  );
  const setThemeAppearancePaint = useCallback(
    (appearance: ThemeAppearanceName, group: ThemePaletteGroup, key: string, hex: string, alpha?: number) =>
      draftThemePatch((next) => {
        next.appearances[appearance][group][key] = alpha === undefined ? { hex } : { hex, alpha };
      }),
    [draftThemePatch],
  );

  const resetTheme = useCallback(() => {
    dispatch({ type: "SET_UI_THEME_DRAFT", value: null });
    dispatch({ type: "SET_UI_THEME_ID", value: "semio" });
  }, []);

  const saveTheme = useCallback(
    (label: string) => {
      const trimmed = label.trim();
      if (!trimmed) return;
      const slug = trimmed
        .toLowerCase()
        .replace(/[^a-z0-9]+/g, "-")
        .replace(/(^-+|-+$)/g, "");
      if (!slug) return;
      const id = `custom.${slug}`;
      const saved: UiTheme = { ...uiThemeBase, id, label: trimmed };
      dispatch({ type: "SET_UI_CUSTOM_THEMES", value: (current) => ({ ...current, [id]: saved }) });
      dispatch({ type: "SET_UI_THEME_DRAFT", value: null });
      dispatch({ type: "SET_UI_THEME_ID", value: id });
    },
    [uiThemeBase],
  );

  const deleteTheme = useCallback((id: string) => {
    if (!id.startsWith("custom.")) return;
    dispatch({
      type: "SET_UI_CUSTOM_THEMES",
      value: (current) => {
        const { [id]: _removed, ...rest } = current;
        return rest;
      },
    });
    dispatch({ type: "SET_UI_THEME_ID", value: (current) => (current === id ? "semio" : current) });
    dispatch({ type: "SET_UI_THEME_DRAFT", value: null });
  }, []);

  const exportTheme = useCallback(() => {
    downloadMediaExport(`${uiThemeBase.id}.theme.json`, "application/json", serializeUiTheme(uiThemeBase));
  }, [uiThemeBase]);

  const importTheme = useCallback(async () => {
    const opened = (await requestFileOpen(".theme.json,application/json"))[0];
    if (!opened) return;
    try {
      const parsed = parseUiTheme(JSON.parse(opened.contents));
      saveTheme(parsed.label || parsed.id);
    } catch {
      /* invalid theme file, ignore */
    }
  }, [saveTheme]);
  //#endregion 🔖ThemeMutators

  const [themeSaveLabel, setThemeSaveLabel] = useState("");
  const settingsHostRef = useRef<SettingsHostApi | null>(null);
  const settingsHost: SettingsHostApi = useMemo(
    () => ({
      appId: session?.app.id,
      appLabel: session ? appDocumentLabel(resolveAppDocument(session.app, uiTerminology)) : undefined,
      controllerId: session?.app.controllerId,
      pluginId: session?.pluginId,
      compact: uiCompact,
      setCompact: (value: boolean) => dispatch({ type: "SET_UI_COMPACT", value }),
      expertise: uiExpertise,
      setExpertise: (value: string) => dispatch({ type: "SET_UI_EXPERTISE", value: value as Expertise }),
      appearance: uiAppearance,
      setAppearance: (value: string) => dispatch({ type: "SET_UI_APPEARANCE", value: value as ElementsSurfaceAppearance }),
      layout: uiLayout,
      setLayout: (value: UiChromeLayout) => dispatch({ type: "SET_UI_LAYOUT", value }),
      mobileActive: mobile,
      onResetDock: () => {
        dispatch({ type: "RESET_DOCK" });
        dockLayoutStore.reset();
        dockUiStateStore.reset();
      },
      locale: uiLocale,
      setLocale: (value: UiLocale) => dispatch({ type: "SET_UI_LOCALE", value }),
      terminology: uiTerminology,
      setTerminology: (value: string) => dispatch({ type: "SET_UI_TERMINOLOGY", value }),
      terminologies: [UI_TERMINOLOGY_NATIVE, ...(session?.app.terminologies ?? [])],
      theme: uiThemeBase,
      themeId: uiThemeId,
      themeDirty: uiThemeDirty,
      themes: uiThemeList,
      setThemeId,
      setThemeColor,
      setThemeSpacing,
      setThemeFontStack,
      setThemeStroke,
      setThemeRadius,
      setThemeOpacity,
      setThemeMetric,
      setThemeAppearancePaint,
      saveTheme,
      deleteTheme,
      resetTheme,
      exportTheme,
      importTheme,
      themeSaveLabel,
      setThemeSaveLabel,
      locks,
    }),
    [
      session,
      dockLayoutStore,
      uiCompact,
      uiExpertise,
      uiAppearance,
      uiLayout,
      mobile,
      uiLocale,
      uiTerminology,
      uiThemeBase,
      uiThemeId,
      uiThemeDirty,
      uiThemeList,
      locks,
      setThemeId,
      setThemeColor,
      setThemeSpacing,
      setThemeFontStack,
      setThemeStroke,
      setThemeRadius,
      setThemeOpacity,
      setThemeMetric,
      setThemeAppearancePaint,
      saveTheme,
      deleteTheme,
      resetTheme,
      exportTheme,
      importTheme,
      themeSaveLabel,
      setThemeSaveLabel,
    ],
  );
  settingsHostRef.current = settingsHost;

  const frameworkDisplayTabs = useMemo(() => createFrameworkDisplayPanelTabs(() => displayHostRef.current), [displayHost, uiLocale]);
  const frameworkSettingsTabs = useMemo(() => createFrameworkSettingsPanelTabs(() => settingsHostRef.current), [settingsHost]);

  useEffect(() => {
    if (!session) return;
    const parseKeys = (keys: string) =>
      keys
        .split(",")
        .map((key) => key.trim().toLowerCase())
        .filter(Boolean);
    const isEditableTarget = (target: EventTarget | null) => {
      if (!(target instanceof HTMLElement)) return false;
      const tag = target.tagName;
      if (tag === "INPUT" || tag === "TEXTAREA" || tag === "SELECT") return true;
      if (target.isContentEditable) return true;
      return target.closest("[contenteditable='true'], [role='textbox']") != null;
    };
    const matches = (event: KeyboardEvent, binding: string) => {
      const parts = binding.split("+").map((part) => part.trim());
      const key = parts[parts.length - 1] ?? "";
      const needsCtrl = parts.includes("ctrl") || parts.includes("meta") || parts.includes("mod");
      const needsShift = parts.includes("shift");
      const needsAlt = parts.includes("alt");
      const hasCtrl = event.ctrlKey || event.metaKey;
      if (needsCtrl !== hasCtrl) return false;
      if (needsShift !== event.shiftKey) return false;
      if (needsAlt !== event.altKey) return false;
      return event.key.toLowerCase() === key;
    };
    const actionById = new Map(session.app.actions.map((action) => [action.id, action]));
    const onKeyDown = (event: KeyboardEvent) => {
      if (isEditableTarget(event.target)) return;
      // 🧰 Escape deactivates the active window's active utility (P5) when nothing is being typed.
      if (event.key === "Escape") {
        const windowId = activeWindowIdRef.current;
        if (windowId && activeUtilityByWindowIdRef.current[windowId]) {
          event.preventDefault();
          onAction({ controllerId: session.app.controllerId, action: SET_ACTIVE_UTILITY_ACTION_ID, args: { windowId, utilityId: "" } });
          return;
        }
      }
      for (const binding of session.app.keybindings) {
        for (const chord of parseKeys(binding.keys)) {
          if (!matches(event, chord)) continue;
          event.preventDefault();
          // ✍️ Arg-carrying hotkeys never silent-fire defaults (P4): open the staged form, or — if that
          // form is already expanded in the active window — treat the hotkey as Execute (with validation).
          const definition = actionById.get(binding.action.action);
          if (definition && actionRequiresStagedForm(definition)) {
            const windowId = activeWindowIdRef.current;
            if (!windowId) return;
            const expanded = actionPaneExpandedByWindowIdRef.current[windowId] ?? null;
            const staged = actionPaneStagedArgsByKeyRef.current[actionStageKey(windowId, definition.id)] ?? {};
            const intent = resolveKeybindingIntent(definition, expanded, staged);
            if (intent.kind === "execute") {
              onAction({ controllerId: session.app.controllerId, action: intent.actionId, args: intent.args });
            } else if (intent.kind === "open") {
              dispatch({ type: "SET_ACTION_PANE_FOLDED", windowId, value: false });
              dispatch({ type: "SET_ACTION_PANE_EXPANDED", windowId, value: intent.actionId });
            }
            return;
          }
          onAction(binding.action);
          return;
        }
      }
    };
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [onAction, session]);

  const activeRightPanelTab = session?.app.panelTabs.find((tab) => panelAnchorForGroup(tab.group) === "top-right");
  const activePanelTabId = panel?.activePanelTab ?? (activeRightPanelTab ? panelTabKindId(activeRightPanelTab.kind) : undefined) ?? (session?.app.panelTabs[0] ? panelTabKindId(session.app.panelTabs[0].kind) : undefined);

  const workbenchLeftTabs = useMemo((): PanelTabNode[] => {
    if (!session) return [];
    const pluginLeftTabs = session.app.panelTabs.filter((tab) => panelAnchorForGroup(tab.group) === "top-left").map((tab, order) => panelTabDefinitionToNode(tab, tab.group, panelUiByKey, onAction, order, appLabelsOverlay));
    if (studioMode && session.app.id === hostAppId && pluginLeftTabs.length > 0) return pluginLeftTabs;
    const hasPluginDocumentTab = pluginLeftTabs.some((tab) => tab.id === FRAMEWORK_PANEL_TAB_DOCUMENT_ID);
    if (hasPluginDocumentTab) return pluginLeftTabs;
    const documentTab = singleTreeLeaf({
      id: FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
      icon: shellTabIcon(FRAMEWORK_PANEL_TAB_DOCUMENT_ICON_ID),
      name: shellLabel("ui.panel.document"),
      order: 0,
      tree: staticTreePanelDefinition({
        sections: [
          {
            id: "document.root",
            label: shellLabel("ui.panel.document"),
            items: [{ id: "document.empty", label: studioMode ? `${panel?.spawnedApps.length ?? 0} ${shellLabel("ui.panel.spawnedAppsSuffix")}` : shellLabel("ui.panel.documentEmpty") }],
          },
        ],
      }),
    });
    return [documentTab, ...pluginLeftTabs];
  }, [appLabelsOverlay, onAction, panel?.spawnedApps.length, panelUiByKey, session, studioMode, uiLocale, hostAppId]);

  const detailsRightTabs = useMemo((): PanelTabNode[] => {
    if (!session) return [];
    return session.app.panelTabs.filter((tab) => panelAnchorForGroup(tab.group) === "top-right").map((tab, order) => panelTabDefinitionToNode(tab, tab.group, panelUiByKey, onAction, order, appLabelsOverlay));
  }, [appLabelsOverlay, onAction, panelUiByKey, session]);

  const settingsRightTabs = useMemo((): PanelTabNode[] => frameworkSettingsTabs, [frameworkSettingsTabs]);

  //#region 🧰FooterUtilityLeaves — bottom-right's History tab, now sourced from the framework-owned History
  // actions in the app registry (the plugin `list-tools` surface is gone; the per-window Actions rail
  // replaces the old footer Actions tab entirely per P6).
  const frameworkUtilitiesHistoryTab = useMemo((): PanelTabNode | null => {
    if (!session) return null;
    const grouped = groupUtilityNodesByCategory(frameworkHistoryUtilityNodes(session.app), ["history"]);
    if (!grouped.length) return null;
    return singleTreeLeaf({
      id: "framework.utilities.history",
      icon: shellTabIcon(UTILITY_CATEGORY_ICON_ID.history),
      name: shellLabel("ui.panel.history"),
      order: 1,
      tree: {
        sections: [{ id: "framework.utilities.history.root", label: "", items: [{ id: "framework.utilities.history.tree", label: "", control: <UtilityTree id="ui.utilities.footer.history" utilities={grouped} onAction={onAction} direction="up" /> }] }],
      },
    });
  }, [onAction, session, uiLocale]);
  //#endregion 🧰FooterUtilityLeaves

  //#region 🔄SyncLeaf — bottom-left's sync tab, replacing the old floating footer SyncAttachCard.
  const frameworkSyncTab = useMemo((): PanelTabNode | null => {
    const syncUtilities = buildFrameworkSyncUtilities(syncBackboneUri) as readonly UtilityNode[];
    if (!syncUtilities.length) return null;
    const syncStatus = syncBackboneUri ? (syncStatusByDocumentId[syncBackboneUri.replace(/^actor:\/\//, "")] ?? null) : null;
    return singleTreeLeaf({
      id: "framework.sync",
      icon: shellTabIcon(UTILITY_CATEGORY_ICON_ID.sync),
      name: shellLabel("ui.panel.sync"),
      order: 0,
      tree: {
        sections: [
          {
            id: "framework.sync.root",
            label: "",
            items: [
              {
                id: "framework.sync.card",
                label: "",
                control: (
                  <SyncAttachCard
                    activeUri={syncBackboneUri}
                    cardKind={syncCardKind}
                    draftPath={syncDraftPath}
                    syncUtilities={syncUtilities}
                    status={syncStatus}
                    onAction={onAction}
                    onDraftPathChange={(value) => dispatch({ type: "SET_SYNC_DRAFT_PATH", value })}
                    onClose={() => dispatch({ type: "SET_SYNC_CARD_KIND", value: null })}
                    onAttach={attachSyncBackbone}
                    onDetach={detachSyncBackbone}
                  />
                ),
              },
            ],
          },
        ],
      },
    });
  }, [attachSyncBackbone, detachSyncBackbone, onAction, syncBackboneUri, syncCardKind, syncDraftPath, syncStatusByDocumentId, uiLocale]);
  //#endregion 🔄SyncLeaf

  const activePluginManifest = useMemo(() => loadedPlugins.find((entry) => entry.handle.pluginId === session?.pluginId)?.manifest, [loadedPlugins, session?.pluginId]);
  const activeModeId = session?.viewState.activeModeId ?? session?.app.modes[0]?.id ?? session?.app.id ?? "";

  const resolvedCommands = useMemo(() => resolveCommands(osCommands, activePluginManifest, session?.app, activeModeId), [osCommands, activePluginManifest, session?.app, activeModeId]);

  const commandCategoryList = useMemo(() => commandCategories(resolvedCommands), [resolvedCommands, uiLocale]);

  /**
   * 🎛 Dispatches a resolved command: os-scope commands are handled locally (no plugin round trip);
   * plugin/app/mode-scope commands route through the active session's plugin `handleCommand`, mirroring
   * `onAction`'s tail. Plugin commands are only resolvable/dispatchable for the active session's plugin
   * instance (no headless-instance routing for non-focused plugins yet).
   */
  const onCommand = useCallback(
    (source: ResolvedCommand["source"], commandId: string, args?: Record<string, unknown>) => {
      if (source.kind === "os") {
        dispatchOsCommand(commandId, args, dispatch, dockLayoutStore, dockUiStateStore, locks);
        return;
      }
      if (!session) return;
      const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === session.pluginId)?.handle;
      if (!plugin?.handleCommand) return;
      const dispatchViewState = injectActiveUtility(session.viewState);
      void plugin
        .handleCommand(session.instanceId, JSON.stringify({ command: commandId, args }), dispatchViewState)
        .then((response) => applyHostEffects(response.requestedEffects ?? [], { ...session, viewState: dispatchViewState }, resolveUiDirtyScope(response.uiScope)))
        .catch((commandError) => {
          console.error("[DEBUG] command failed", commandError);
        });
    },
    [applyHostEffects, dockLayoutStore, dockUiStateStore, injectActiveUtility, loadedPlugins, session, locks],
  );

  const commandCategoryTabs = useMemo(() => buildCommandCategoryTabs(resolvedCommands, commandCategoryList, expandedCommandIdRef, commandStagedArgsByCommandIdRef, onCommand, dispatch), [resolvedCommands, commandCategoryList, onCommand]);

  //#region 🧭DockAssembly — default four-corner arrangement (the two middle anchors start empty save the command palette in bottom-middle) + persisted-override reconciliation + drag-and-drop wiring.
  const defaultDock = useMemo((): PanelDock => {
    // 🧭 Top-left (Workbench: Document/Catalogue), top-right (Details: Inspection/Parameters) and bottom-right
    // (Settings: Theme/Settings) render their tabs flat, one level up from where they used to sit — the
    // category-branch wrapper tab is gone, so each leaf is a top-level toggle instead of two clicks deep.
    const topLeft: PanelTabNode[] = [...workbenchLeftTabs];
    const bottomLeft: PanelTabNode[] = [];
    if (frameworkDisplayTabs.length > 0) {
      bottomLeft.push({ kind: "branch", id: FRAMEWORK_CATEGORY_DISPLAY_ID, icon: categoryTabIcon(frameworkDisplayTabs, "layout-grid"), name: shellLabel("ui.panelToggle.display"), order: 0, children: frameworkDisplayTabs });
    }
    if (frameworkSyncTab) bottomLeft.push(frameworkSyncTab);
    const topRight: PanelTabNode[] = [...detailsRightTabs];
    const bottomRight: PanelTabNode[] = [...settingsRightTabs];
    if (frameworkUtilitiesHistoryTab) bottomRight.push(frameworkUtilitiesHistoryTab);
    // 🎛 Command categories stay nested under one expandable Command branch (unlike flat Theme/Settings
    // footer toggles) so the folded bottom-middle chrome shows a single Command toggle, not every
    // category leaf inlined along the footer.
    const bottomMiddle: PanelTabNode[] =
      commandCategoryTabs.length > 0 ? [{ kind: "branch", id: FRAMEWORK_CATEGORY_COMMAND_ID, icon: categoryTabIcon(commandCategoryTabs, "wrench"), name: shellLabel("ui.panelToggle.command"), order: 0, children: commandCategoryTabs }] : [];
    return { anchors: { "top-left": topLeft, "top-middle": [], "top-right": topRight, "bottom-left": bottomLeft, "bottom-middle": bottomMiddle, "bottom-right": bottomRight } };
  }, [commandCategoryTabs, detailsRightTabs, frameworkDisplayTabs, frameworkSyncTab, frameworkUtilitiesHistoryTab, settingsRightTabs, uiLocale, workbenchLeftTabs]);

  useEffect(() => {
    dispatch({ type: "SET_DOCK_OVERRIDE", value: dockLayoutStore.getSnapshot() });
  }, [dockLayoutStore]);

  const dock = useMemo((): PanelDock => applyDockSkeleton(defaultDock, dockOverride), [defaultDock, dockOverride]);

  /** 🗄️ Skips the very first (pre-hydration) commit so a persisted skeleton isn't clobbered with `null` before the seeding effect above has a chance to read and apply it. */
  const dockPersistedOnceRef = useRef(false);
  useEffect(() => {
    if (!dockPersistedOnceRef.current) {
      dockPersistedOnceRef.current = true;
      return;
    }
    const nextSkeleton = dockSkeletonOf(dock);
    const defaultSkeleton = dockSkeletonOf(defaultDock);
    dockLayoutStore.save(dockSkeletonsEqual(nextSkeleton, defaultSkeleton) ? null : nextSkeleton);
  }, [dock, defaultDock, dockLayoutStore]);

  useEffect(() => {
    dispatch({ type: "HYDRATE_DOCK_UI", value: dockUiStateStore.getSnapshot() });
  }, [dockUiStateStore]);

  /** 🗄️ Same first-commit-skip as the dock skeleton effect above, but also re-arms when the store identity itself changes (app switch) — otherwise the new app's pre-hydration state would be written into its own key on the first post-switch commit. */
  const dockUiPersistedOnceRef = useRef(false);
  const dockUiPersistedStoreRef = useRef(dockUiStateStore);
  useEffect(() => {
    if (dockUiPersistedStoreRef.current !== dockUiStateStore) {
      dockUiPersistedStoreRef.current = dockUiStateStore;
      dockUiPersistedOnceRef.current = false;
    }
    if (!dockUiPersistedOnceRef.current) {
      dockUiPersistedOnceRef.current = true;
      return;
    }
    const anchors: Partial<Record<PanelAnchor, DockUiPanelState>> = {};
    for (const anchor of PANEL_ANCHORS) {
      const panelState = panels[anchor];
      const entry: DockUiPanelState = {};
      if (panelState.visible) entry.visible = true;
      if (panelState.size !== DEFAULT_PANEL_SIZES[anchor]) entry.size = panelState.size;
      if (panelState.path.length > 0) entry.path = panelState.path;
      if (Object.keys(entry).length > 0) anchors[anchor] = entry;
    }
    const hasPathMemory = Object.keys(panelPathMemory).length > 0;
    const hasTreeOpen = Object.keys(treeOpenStates).length > 0;
    const isDefault = Object.keys(anchors).length === 0 && !hasPathMemory && !hasTreeOpen;
    dockUiStateStore.save(isDefault ? null : { version: 2, anchors, pathMemory: hasPathMemory ? panelPathMemory : undefined, treeOpen: hasTreeOpen ? treeOpenStates : undefined });
  }, [panels, panelPathMemory, treeOpenStates, dockUiStateStore]);

  const handleTabDockDrop = useCallback(
    (move: PanelTabDockMove) => {
      const nextDock = moveTabInDock(dock, move);
      if (nextDock === dock) return;
      const nextSkeleton = dockSkeletonOf(nextDock);
      const defaultSkeleton = dockSkeletonOf(defaultDock);
      dispatch({ type: "SET_DOCK_OVERRIDE", value: dockSkeletonsEqual(nextSkeleton, defaultSkeleton) ? null : nextSkeleton });
      const targetPath = findPanelTabPath(nextDock.anchors[move.target.anchor], move.tabId);
      if (targetPath) dispatch({ type: "SET_PANEL_PATH", anchor: move.target.anchor, value: targetPath });
      if (move.fromAnchor !== move.target.anchor) {
        const sourceTabs = nextDock.anchors[move.fromAnchor];
        dispatch({ type: "SET_PANEL_PATH", anchor: move.fromAnchor, value: (prev) => reconcileActivePath(sourceTabs, prev, panelTabChildren) });
      }
      dispatch({ type: "SET_PANEL_VISIBLE", anchor: move.target.anchor, value: true });
    },
    [dock, defaultDock],
  );

  const handleTreeUnitDockDrop = useCallback(
    (move: PanelTreeUnitDockMove) => {
      const nextDock = moveTreeUnitInDock(dock, move);
      if (nextDock === dock) return;
      const nextSkeleton = dockSkeletonOf(nextDock);
      const defaultSkeleton = dockSkeletonOf(defaultDock);
      dispatch({ type: "SET_DOCK_OVERRIDE", value: dockSkeletonsEqual(nextSkeleton, defaultSkeleton) ? null : nextSkeleton });
      dispatch({ type: "SET_PANEL_VISIBLE", anchor: move.target.anchor, value: true });
    },
    [dock, defaultDock],
  );

  const studioOverrideTabId = studioMode && session?.app.id === hostAppId ? (panel?.activePanelTab ?? hostCatalogueTabId) : undefined;
  const studioOverrideAnchor = studioOverrideTabId ? findPanelTabInDock(dock, studioOverrideTabId)?.anchor : undefined;
  const detailsOverrideTabId = panel?.activePanelTab;
  const detailsOverrideAnchor = detailsOverrideTabId ? findPanelTabInDock(dock, detailsOverrideTabId)?.anchor : undefined;

  /** @emoji 🎓 The current introduction step's anchor, decomposed by kind — `null` unless that kind is
   * active, so every reveal override below (here and in `modeWindows`) is a plain truthiness check. A
   * folded utility bar/Actions rail/dock panel would otherwise hide the step's anchor from ever mounting (see
   * `useIntroductionAnchorRect`), leaving the step centered with no highlight and no way for the user to
   * find what to do. */
  const activeIntroductionStepAnchor: IntroductionAnchor | null = activeIntroduction && introductionStepIndex != null ? (activeIntroduction.steps[introductionStepIndex]?.anchor ?? null) : null;
  const introductionUtilityId = activeIntroductionStepAnchor?.kind === "utility" ? activeIntroductionStepAnchor.id : null;
  const introductionActionId = activeIntroductionStepAnchor?.kind === "action" ? activeIntroductionStepAnchor.id : null;
  const introductionPanelTabId = activeIntroductionStepAnchor?.kind === "panelTab" ? activeIntroductionStepAnchor.id : null;
  const introductionPanelTabAnchor = introductionPanelTabId ? findPanelTabInDock(dock, introductionPanelTabId)?.anchor : undefined;
  const introductionUtilityWindowId = useMemo(() => {
    if (!introductionUtilityId || !session) return null;
    for (const kind of session.app.windowKinds) {
      const utilities = resolveUtilityNodes(session.app, kind, null, kind.id, appLabelsOverlay);
      if (utilityNodeTreeContainsId(utilities, introductionUtilityId)) return kind.id;
    }
    return null;
  }, [appLabelsOverlay, introductionUtilityId, session]);
  const introductionActionWindowId = useMemo(() => {
    if (!introductionActionId || !session) return null;
    for (const kind of session.app.windowKinds) {
      const actions = resolveWindowActions(session.app, kind);
      if (actions.some((action) => action.id === introductionActionId)) return kind.id;
    }
    return null;
  }, [introductionActionId, session]);
  const introductionAnchorFallbackSelectors = useMemo((): readonly string[] => {
    if (introductionUtilityWindowId) return [introductionUtilityBarUnfoldSelector(introductionUtilityWindowId)];
    if (introductionActionWindowId) return [introductionWindowActionPaneUnfoldSelector(introductionActionWindowId)];
    return [];
  }, [introductionActionWindowId, introductionUtilityWindowId]);

  const lastIntroductionPanelTabIdRef = useRef<string | undefined>(undefined);
  useEffect(() => {
    if (!introductionPanelTabId || !introductionPanelTabAnchor) {
      lastIntroductionPanelTabIdRef.current = undefined;
      return;
    }
    if (lastIntroductionPanelTabIdRef.current === introductionPanelTabId) return;
    lastIntroductionPanelTabIdRef.current = introductionPanelTabId;
    const resolved = findPanelTabPath(dock.anchors[introductionPanelTabAnchor], introductionPanelTabId);
    if (resolved) dispatch({ type: "SET_PANEL_PATH", anchor: introductionPanelTabAnchor, value: resolved });
    dispatch({ type: "SET_PANEL_VISIBLE", anchor: introductionPanelTabAnchor, value: true });
  }, [introductionPanelTabId, introductionPanelTabAnchor, dock]);

  /** 🧭 Progressive reveal means a stored path can legitimately end at a branch (or be empty) — this is now a plain per-anchor truncation-validate, no override reassertion (see the write-through effects below). */
  const panelActivePaths = useMemo((): Record<PanelAnchor, readonly string[]> => {
    const result = {} as Record<PanelAnchor, readonly string[]>;
    for (const anchor of PANEL_ANCHORS) result[anchor] = reconcileActivePath(dock.anchors[anchor], panels[anchor].path, panelTabChildren);
    return result;
  }, [panels, dock]);

  /**
   * 🧭 Generalizes the old `leftPanelActivePath`/`rightPanelActivePath` studio/plugin "snap to the active panel
   * tab" overrides across all six anchors. Write-through rather than read-time: each override dispatches
   * `SET_PANEL_PATH` only when its target tab id actually changes, so a user's own collapse/navigation
   * afterward sticks instead of being reasserted on every render (progressive reveal made read-time reassertion
   * fight the user's own collapses). Studio wins over details when both would touch the same anchor.
   **/
  const lastStudioOverrideTabIdRef = useRef<string | undefined>(undefined);
  useEffect(() => {
    if (!studioOverrideTabId || !studioOverrideAnchor) {
      lastStudioOverrideTabIdRef.current = undefined;
      return;
    }
    if (lastStudioOverrideTabIdRef.current === studioOverrideTabId) return;
    lastStudioOverrideTabIdRef.current = studioOverrideTabId;
    if (panels[studioOverrideAnchor].path[0] === FRAMEWORK_CATEGORY_DISPLAY_ID) return;
    const resolved = findPanelTabPath(dock.anchors[studioOverrideAnchor], studioOverrideTabId);
    if (resolved) dispatch({ type: "SET_PANEL_PATH", anchor: studioOverrideAnchor, value: resolved });
  }, [studioOverrideTabId, studioOverrideAnchor, dock, panels]);

  const lastDetailsOverrideTabIdRef = useRef<string | undefined>(undefined);
  useEffect(() => {
    if (!detailsOverrideTabId || !detailsOverrideAnchor) {
      lastDetailsOverrideTabIdRef.current = undefined;
      return;
    }
    if (lastDetailsOverrideTabIdRef.current === detailsOverrideTabId) return;
    lastDetailsOverrideTabIdRef.current = detailsOverrideTabId;
    if (detailsOverrideAnchor === studioOverrideAnchor) return;
    // 🧭 Settings tabs render flat now (no category branch to check against) — skip the override if the
    // anchor's active leaf already belongs to Settings, so browsing Theme/Settings there doesn't get stomped.
    if (settingsRightTabs.some((tab) => tab.id === panels[detailsOverrideAnchor].path[0])) return;
    const resolved = findPanelTabPath(dock.anchors[detailsOverrideAnchor], detailsOverrideTabId);
    if (resolved) dispatch({ type: "SET_PANEL_PATH", anchor: detailsOverrideAnchor, value: resolved });
  }, [detailsOverrideTabId, detailsOverrideAnchor, studioOverrideAnchor, dock, panels, settingsRightTabs]);
  //#endregion 🧭DockAssembly

  const mobilePanelTabs = useMemo(
    () => [...defaultDock.anchors["top-left"], ...defaultDock.anchors["top-middle"], ...defaultDock.anchors["top-right"], ...defaultDock.anchors["bottom-left"], ...defaultDock.anchors["bottom-middle"], ...defaultDock.anchors["bottom-right"]],
    [defaultDock],
  );

  const mobilePanel = useMemo(() => {
    if (mobilePanelTabs.length === 0) return undefined;
    return {
      visible: PANEL_ANCHORS.some((anchor) => panels[anchor].visible),
      tabs: mobilePanelTabs,
      activeTabPath: mobilePanelPath,
      onActiveTabPathChange: (path: readonly string[]) => {
        dispatch({ type: "SET_MOBILE_PANEL_PATH", value: path });
        const tabId = path[path.length - 1];
        // 🌱 Progressive paths often end at a branch (or are empty) — only leaves are meaningful "active panel tab" selections.
        if (tabId && studioMode && session?.app.id === hostAppId && findPanelTabNode(mobilePanelTabs, path)?.kind === "leaf") {
          onAction({ controllerId: session.app.controllerId, action: "setActivePanelTab", args: { tabId } });
        }
      },
      pathMemory: panelPathMemory,
      onPathMemoryChange: (value: Readonly<Record<string, string>>) => dispatch({ type: "SET_PANEL_PATH_MEMORY", value }),
      treeOpenStates,
      onTreeOpenStateChange: (id: string, open: boolean) => dispatch({ type: "SET_TREE_OPEN_STATE", id, open }),
    };
  }, [panels, mobilePanelPath, mobilePanelTabs, onAction, panelPathMemory, session, studioMode, treeOpenStates, hostAppId]);

  const exampleOptions = useMemo(() => {
    const appId = session?.app.id ?? "";
    if (!appId) return [];
    const seen = new Set<string>();
    return (activePluginManifest?.examples ?? [])
      .filter((example) => example.appId === appId)
      .filter((example) => {
        if (seen.has(example.id)) return false;
        seen.add(example.id);
        return true;
      })
      .map((example) => ({ id: example.id, label: resolveAppLabel(appLabelsOverlay, "example", example.id, example.label) }));
  }, [activePluginManifest, session?.app.id, appLabelsOverlay]);

  useEffect(() => {
    if (exampleOptions.length === 0) return;
    dispatch({ type: "SET_ACTIVE_EXAMPLE_ID", value: (current) => (!current || exampleOptions.some((option) => option.id === current) ? current : "") });
  }, [exampleOptions, session?.app.id, session?.pluginId]);

  // 🎛️ Announces the boot example to the fresh session exactly once per instance — the same path
  // whether the example is locked, defaulted, or absent (an empty id resets the plugin's default fixture).
  useEffect(() => {
    if (exampleOptions.length === 0 || !session) return;
    if (noExampleResetInstanceIdRef.current === session.instanceId) return;
    noExampleResetInstanceIdRef.current = session.instanceId;
    onAction({ controllerId: session.app.controllerId, action: "setActiveExample", args: { exampleId: activeExampleId || "" } });
  }, [activeExampleId, exampleOptions, onAction, session]);

  //#region 🎛️PanelTabBarHosting — `buildPanelSelectionProps` is the single source of an anchor's tab
  // selection state, shared by the chrome-hosted `PanelChromeTabBar` (below, for anchors in
  // {@link PANEL_TAB_BAR_HOSTS}) and the floating `Panel` itself (`buildPanelProps`) — the two hosts of the
  // SAME anchor always read/write the exact same controlled state.
  const buildPanelSelectionProps = useCallback(
    (anchor: PanelAnchor): PanelTabSelectionOptions => ({
      tabs: dock.anchors[anchor],
      visible: panels[anchor].visible,
      onVisibleChange: (value: boolean) => dispatch({ type: "SET_PANEL_VISIBLE", anchor, value }),
      activeTabPath: panelActivePaths[anchor],
      onActiveTabPathChange: (path: readonly string[]) => {
        dispatch({ type: "SET_PANEL_PATH", anchor, value: path });
        // 🎛️ Command palette only: switching category leaves always collapses any expanded arg form — the
        // next hierarchy level up only makes sense under its own category's command list (mirrors the old
        // dedicated `SET_COMMAND_CATEGORY` reducer case, now expressed at the generic path-change call site
        // since category-active state itself is just this anchor's `activeTabPath`). Categories sit under
        // the Command branch, so compare the category segment (path[1]), not the shared branch root.
        if (anchor === "bottom-middle" && panels[anchor].path[1] !== path[1]) {
          dispatch({ type: "SET_COMMAND_EXPANDED", value: null });
        }
        const tabId = path[path.length - 1];
        // 🌱 Progressive paths often end at a branch (or are empty) — only leaves are meaningful "active panel tab" selections.
        if (tabId && studioMode && session?.app.id === hostAppId && findPanelTabNode(dock.anchors[anchor], path)?.kind === "leaf") {
          onAction({ controllerId: session.app.controllerId, action: "setActivePanelTab", args: { tabId } });
        }
      },
      pathMemory: panelPathMemory,
      onPathMemoryChange: (value: Readonly<Record<string, string>>) => dispatch({ type: "SET_PANEL_PATH_MEMORY", value }),
    }),
    [dock, onAction, panelActivePaths, panelPathMemory, panels, session, studioMode, hostAppId],
  );
  //#endregion 🎛️PanelTabBarHosting

  const navbarItems = useMemo((): NavbarItem[] => {
    if (!session) return [];
    // Logo/title, example selector, and mode switcher render as one cluster, centered as a group in the navbar
    // (via `centered`) rather than left-anchored with fill spacers pushing the rest toward the trailing edge.
    const centerContent: ReactNode[] = [
      <div key="logoAndTitle" className="flex min-w-0 shrink-0 items-center gap-single">
        {brand?.logoSvg ? <ShellBrandLogo svg={brand.logoSvg} className="size-workbench shrink-0" /> : <SemioLogo className="size-workbench shrink-0" />}
        <span data-slot="app-name" className={cn("px-single", shellChromeTitleClassName)}>
          {appDocumentLabel(resolveAppDocument(session.app, uiTerminology))}
        </span>
      </div>,
    ];
    if (exampleOptions.length > 0 && !locks.exampleId && (!studioMode || session.app.id !== landingAppId)) {
      centerContent.push(
        <NavbarExampleSelect
          key="fixture"
          id="playground.navbar.fixture"
          value={activeExampleId}
          options={exampleOptions}
          onValueChange={(exampleId) => {
            dispatch({ type: "SET_ACTIVE_EXAMPLE_ID", value: exampleId });
            onAction({ controllerId: session.app.controllerId, action: "setActiveExample", args: { exampleId: exampleId || "" } });
          }}
        />,
      );
    }
    if (session.app.modes.length > 1) {
      centerContent.push(
        <ButtonGroup key="modes" id="playground.navbar.modes">
          {session.app.modes.map((mode) => {
            const isActive = activeModeId === mode.id;
            return (
              <ButtonGroupItem
                key={mode.id}
                id={`playground.navbar.modes.${mode.id}`}
                className={cn(isActive && interactiveActiveFillClass)}
                data-state={isActive ? "on" : undefined}
                onClick={() => applyModeChange(mode.id)}
                icon={<span className="hidden" />}
                text={resolveAppLabel(appLabelsOverlay, "mode", mode.id, mode.label)}
              />
            );
          })}
        </ButtonGroup>,
      );
    }
    return [
      { key: "topLeftPanelTabs", content: <PanelChromeTabBar anchor="top-left" {...buildPanelSelectionProps("top-left")} /> },
      navbarFillItem("navbarTrailingFill"),
      { key: "topRightPanelTabs", content: <PanelChromeTabBar anchor="top-right" {...buildPanelSelectionProps("top-right")} /> },
      {
        key: "center",
        centered: true,
        content: (
          <div className="flex min-w-0 items-center gap-double">
            {centerContent}
            <PanelChromeTabBar anchor="top-middle" {...buildPanelSelectionProps("top-middle")} />
          </div>
        ),
      },
    ];
  }, [activeExampleId, activeModeId, appLabelsOverlay, applyModeChange, brand, buildPanelSelectionProps, exampleOptions, locks.exampleId, onAction, session, uiTerminology, studioMode, landingAppId]);

  const searchItems = useMemo(() => {
    if (!session) return [];
    const items: UISearchItem[] = [];
    for (const tab of flattenPanelTabLeaves(session.app.panelTabs)) {
      const tabId = panelTabKindId(tab.kind);
      items.push({
        id: `panel.${tabId}`,
        label: resolveAppLabel(appLabelsOverlay, "panelTab", tabId, tab.label),
        category: shellLabel("ui.search.category.panels"),
        icon: <Icon icon="panel-left" size="small" />,
        onSelect: () => onAction({ controllerId: session.app.controllerId, action: "setActivePanelTab", args: { tabId } }),
      });
    }
    for (const kind of session.app.windowKinds) {
      items.push({
        id: `window.${kind.id}`,
        label: resolveAppLabel(appLabelsOverlay, "windowKind", kind.id, kind.label),
        category: shellLabel("ui.search.category.windows"),
        icon: <Icon icon="app-window" size="small" />,
        onSelect: () => dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: kind.id }),
      });
    }
    const keysByActionId = new Map(session.app.keybindings.map((binding) => [binding.action.action, binding.keys]));
    const declaredActionIds = new Set<string>();
    // 📇 First window kind whose resolved actions include this id (orphan/global actions fall through to
    // the active window, then the first window) — the redirect target for arg-carrying palette entries.
    const hostWindowForAction = (actionId: string): string | undefined => {
      for (const kind of session.app.windowKinds) {
        if (resolveWindowActions(session.app, kind).some((entry) => entry.id === actionId)) return kind.id;
      }
      return activeWindowId ?? session.app.windowKinds[0]?.id;
    };
    for (const action of session.app.actions ?? []) {
      if (!action.inPalette) continue;
      declaredActionIds.add(action.id);
      const argCarrying = actionRequiresStagedForm(action);
      const resolvedActionLabel = resolveAppLabel(appLabelsOverlay, "action", action.id, action.label);
      items.push({
        id: `action.${action.id}`,
        // ✍️ Arg-carrying actions never fire from the palette (P3): the "…" entry activates the hosting
        // window, unfolds its Actions rail, and expands this action's staged form instead of dispatching.
        label: argCarrying ? `${resolvedActionLabel}…` : resolvedActionLabel,
        description: action.keys ?? keysByActionId.get(action.id),
        category: action.category ?? (action.kind === "history" ? shellLabel("ui.ribbon.parent.history") : shellLabel("ui.ribbon.parent.actions")),
        onSelect: () => {
          if (argCarrying) {
            const windowId = hostWindowForAction(action.id);
            if (windowId) {
              dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: windowId });
              dispatch({ type: "SET_ACTION_PANE_FOLDED", windowId, value: false });
              dispatch({ type: "SET_ACTION_PANE_EXPANDED", windowId, value: action.id });
            }
            dispatch({ type: "SET_SEARCH_OPEN", value: false });
            return;
          }
          onAction({ controllerId: session.app.controllerId, action: action.id });
        },
      });
    }
    for (const binding of session.app.keybindings) {
      if (declaredActionIds.has(binding.action.action)) continue;
      items.push({
        id: `keybinding.${binding.keys}`,
        label: binding.action.action,
        description: binding.keys,
        category: shellLabel("ui.ribbon.parent.actions"),
        onSelect: () => onAction(binding.action),
      });
    }
    // 🎛️ Commands (os/plugin/app/mode) — the footer twin of the window-rail P3 redirect above: an
    // arg-carrying command never fires from the palette, it opens the bottom-middle command panel at its
    // category and expands its form instead.
    for (const { definition, source } of resolvedCommands) {
      if (!definition.inPalette) continue;
      const argCarrying = (definition.args?.length ?? 0) > 0;
      items.push({
        id: `command.${definition.id}`,
        label: argCarrying ? `${definition.label}…` : definition.label,
        description: definition.keys,
        category: commandCategoryLabel(definition.category),
        onSelect: () => {
          if (argCarrying) {
            dispatch({ type: "SET_PANEL_VISIBLE", anchor: "bottom-middle", value: true });
            dispatch({ type: "SET_PANEL_PATH", anchor: "bottom-middle", value: [FRAMEWORK_CATEGORY_COMMAND_ID, `command.category.${definition.category}`] });
            dispatch({ type: "SET_COMMAND_EXPANDED", value: definition.id });
            dispatch({ type: "SET_SEARCH_OPEN", value: false });
            return;
          }
          onCommand(source, definition.id);
        },
      });
    }
    if (studioMode && panel) {
      for (const program of panel.programs) {
        items.push({
          id: `spawn.${program.programId}`,
          label: `${shellLabel("ui.palette.spawnPrefix")} ${appDocumentLabel(resolveDocumentByAppId(loadedPlugins, program.appId, program.document, uiTerminology))}`,
          category: shellLabel("ui.search.category.catalogue"),
          onSelect: () => onAction({ controllerId: hostControllerId ?? "", action: "spawnApp", args: { programId: program.programId } }),
        });
      }
      items.push(
        {
          id: "studio.undo",
          label: shellLabel("ui.palette.undo"),
          category: shellLabel("ui.search.category.studio"),
          icon: <Icon icon="undo-2" size="small" />,
          onSelect: () => onAction({ controllerId: hostControllerId ?? "", action: "undo" }),
        },
        {
          id: "studio.redo",
          label: shellLabel("ui.palette.redo"),
          category: shellLabel("ui.search.category.studio"),
          icon: <Icon icon="redo-2" size="small" />,
          onSelect: () => onAction({ controllerId: hostControllerId ?? "", action: "redo" }),
        },
        {
          id: "studio.home",
          label: shellLabel("ui.palette.goHome"),
          category: shellLabel("ui.search.category.navigation"),
          onSelect: () => onAction({ controllerId: hostControllerId ?? "", action: "goHome" }),
        },
      );
    }
    return items;
  }, [activeWindowId, appLabelsOverlay, loadedPlugins, onAction, onCommand, panel, resolvedCommands, session, studioMode, uiLocale, uiTerminology, hostControllerId]);

  const modeWindows = useMemo((): ModeWindowDescriptor[] => {
    if (!session) return [];
    const actionPaneSlice: ActionPaneSlice = { expandedByWindowId: actionPaneExpandedByWindowId, stagedArgsByKey: actionPaneStagedArgsByKey, activeUtilityByWindowId };
    const actionsFoldedFor = (windowId: string, actions: readonly ActionDefinition[]) => (introductionActionId && actions.some((action) => action.id === introductionActionId) ? false : (actionPaneFoldedByWindowId[windowId] ?? true));
    const onActionsFoldedFor = (windowId: string) => (folded: boolean) => dispatch({ type: "SET_ACTION_PANE_FOLDED", windowId, value: folded });
    // 🖱️ Window-body cursor follows the active utility's declared `cursor` (P5).
    const cursorFor = (app: AppDefinition, windowId: string): CSSProperties | undefined => {
      const utilityId = activeUtilityByWindowId[windowId];
      const cursor = utilityId ? (app.utilities ?? []).find((utility) => utility.id === utilityId)?.cursor : undefined;
      return cursor ? { cursor } : undefined;
    };
    if (studioMode && spawnedWindowUi && panel?.activeSpawnedId) {
      const spawned = panel.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId);
      if (spawned) {
        const spawnedApp = loadedPlugins.find((entry) => entry.handle.pluginId === spawned.pluginId)?.manifest.apps.find((candidate) => candidate.id === spawned.appId);
        const windowKind = spawnedApp?.windowKinds[0];
        const chrome = windowKind ? spawnedWindowChromeForKind(windowKind, spawnedWindowEngagements, spawnedWindowMeasures, activeUtilityByWindowId[spawned.id], onActionStable) : undefined;
        const spawnedUtilities = spawnedApp && windowKind ? resolveUtilityNodes(spawnedApp, windowKind, activeUtilityByWindowId[spawned.id], spawned.id, appLabelsOverlay) : [];
        const spawnedActions = spawnedApp && windowKind ? resolveWindowActions(spawnedApp, windowKind) : [];
        return [
          {
            id: spawned.id,
            title: appDocumentLabel(spawnedApp ? resolveAppDocument(spawnedApp, uiTerminology) : spawned.document),
            fill: true,
            showControls: true,
            measures: chrome?.measures,
            engagement: chrome?.engagement,
            utilityBar: spawnedApp && windowKind ? utilityBarNode(spawnedUtilities, spawned.id, onActionStable, introductionUtilityId, chrome?.utilityOptions) : undefined,
            actionPane: spawnedApp && windowKind ? windowActionPaneNode(spawnedApp, windowKind, spawned.id, actionPaneSlice, onActionStable, dispatch, appLabelsOverlay) : undefined,
            actionsFolded: actionsFoldedFor(spawned.id, spawnedActions),
            onActionsFoldedChange: onActionsFoldedFor(spawned.id),
            children: (
              <ChromeAwareWindowScrollSurface className="relative flex h-full min-h-0 min-w-0 flex-1 flex-col overflow-hidden" style={spawnedApp ? cursorFor(spawnedApp, spawned.id) : undefined}>
                <InterpretedUiNode node={spawnedWindowUi} onAction={onActionStable} />
              </ChromeAwareWindowScrollSurface>
            ),
          },
        ];
      }
    }
    if (Object.keys(windowUiByKind).length === 0) return [];
    const baseWindows = session.app.windowKinds.map((kind) => {
      const utilities = resolveUtilityNodes(session.app, kind, activeUtilityByWindowId[kind.id], kind.id, appLabelsOverlay);
      const actions = resolveWindowActions(session.app, kind);
      const chrome = windowMeasuresChrome(windowMeasuresByKind[kind.id] ?? kind.options.measures, activeUtilityByWindowId[kind.id], kind.id, onActionStable);
      return {
        id: kind.id,
        title: appWindowDocumentLabel(session.app, uiTerminology, resolveAppLabel(appLabelsOverlay, "windowKind", kind.id, kind.label)),
        fill: true,
        showControls: true,
        measures: chrome.measures,
        engagement: windowEngagementToSpec(resolveWindowEngagement(kind, windowEngagementsByKind), onActionStable),
        utilityBar: utilityBarNode(utilities, kind.id, onActionStable, introductionUtilityId, chrome.utilityOptions),
        actionPane: windowActionPaneNode(session.app, kind, kind.id, actionPaneSlice, onActionStable, dispatch, appLabelsOverlay),
        actionsFolded: actionsFoldedFor(kind.id, actions),
        onActionsFoldedChange: onActionsFoldedFor(kind.id),
        children: (
          <ChromeAwareWindowScrollSurface className="relative flex h-full min-h-0 min-w-0 flex-1 flex-col overflow-hidden" data-window-kind-id={kind.id} style={cursorFor(session.app, kind.id)}>
            <InterpretedUiNode node={windowUiByKind[kind.id] ?? { type: "text", value: `${shellLabel("ui.common.missingWindow")}: ${kind.id}` }} onAction={onActionStable} />
          </ChromeAwareWindowScrollSurface>
        ),
      };
    });
    const extraWindows = extraWindowInstances.flatMap((instance) => {
      const kind = session.app.windowKinds.find((entry) => entry.id === instance.windowKindId);
      if (!kind) return [];
      const utilities = resolveUtilityNodes(session.app, kind, activeUtilityByWindowId[instance.id], instance.id, appLabelsOverlay);
      const actions = resolveWindowActions(session.app, kind);
      const chrome = windowMeasuresChrome(windowMeasuresByKind[kind.id] ?? kind.options.measures, activeUtilityByWindowId[instance.id], instance.id, onActionStable);
      return [
        {
          id: instance.id,
          title: instance.title,
          fill: true,
          showControls: true,
          measures: chrome.measures,
          engagement: windowEngagementToSpec(resolveWindowEngagement(kind, windowEngagementsByKind), onActionStable),
          utilityBar: utilityBarNode(utilities, instance.id, onActionStable, introductionUtilityId, chrome.utilityOptions),
          actionPane: windowActionPaneNode(session.app, kind, instance.id, actionPaneSlice, onActionStable, dispatch, appLabelsOverlay),
          actionsFolded: actionsFoldedFor(instance.id, actions),
          onActionsFoldedChange: onActionsFoldedFor(instance.id),
          children: (
            <ChromeAwareWindowScrollSurface className="relative flex h-full min-h-0 min-w-0 flex-1 flex-col overflow-hidden" data-window-kind-id={kind.id} style={cursorFor(session.app, instance.id)}>
              <InterpretedUiNode node={windowUiByKind[kind.id] ?? { type: "text", value: `${shellLabel("ui.common.missingWindow")}: ${kind.id}` }} onAction={onActionStable} />
            </ChromeAwareWindowScrollSurface>
          ),
        },
      ];
    });
    return [...baseWindows, ...extraWindows];
  }, [
    actionPaneExpandedByWindowId,
    actionPaneFoldedByWindowId,
    actionPaneStagedArgsByKey,
    activeUtilityByWindowId,
    appLabelsOverlay,
    extraWindowInstances,
    introductionActionId,
    introductionUtilityId,
    loadedPlugins,
    onActionStable,
    panel,
    session,
    spawnedWindowEngagements,
    spawnedWindowMeasures,
    spawnedWindowUi,
    studioMode,
    uiLocale,
    uiTerminology,
    windowEngagementsByKind,
    windowMeasuresByKind,
    windowUiByKind,
  ]);

  const effectiveModeLayout = useMemo(
    () =>
      shellLayout ??
      (session
        ? convertFrameworkLayoutToModeLayout(
            session.app.defaultLayout,
            modeWindows.map((window) => window.id),
            appLabelsOverlay,
          )
        : { kind: "stack" as const, children: [] }),
    [appLabelsOverlay, modeWindows, session, shellLayout],
  );

  const canvas = useMemo(() => {
    if (!session) return <p className="p-double text-sm text-muted-foreground">{shellLabel("ui.common.loadingPlugins")}</p>;
    if (error)
      return (
        <p className="p-double text-sm text-destructive" role="alert">
          {error}
        </p>
      );
    const modes = session.app.modes.length > 0 ? session.app.modes : [{ id: session.app.id, label: appDocumentLabel(resolveAppDocument(session.app, uiTerminology)) }];
    const studioHomeBar =
      studioMode && session.app.id === hostAppId && !panel?.activeSpawnedId ? (
        <button
          type="button"
          className={cn(borderNormalBottomClass, "px-single py-single text-left text-sm text-muted-foreground hover:bg-muted/40 hover:text-foreground")}
          onClick={() => onAction({ controllerId: session.app.controllerId, action: "goHome" })}
        >
          ← {shellLabel("ui.common.home")}
        </button>
      ) : null;
    const focusedSpawned = panel?.activeSpawnedId ? panel.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId) : undefined;
    const focusedBar = focusedSpawned ? (
      <div className={cn(borderNormalBottomClass, "flex items-center gap-single px-single py-single text-sm text-muted-foreground")}>
        <button type="button" className="hover:text-foreground" onClick={() => (openStudioIdRef.current ? navigateHistory(`/studios/${openStudioIdRef.current}`) : onAction({ controllerId: session.app.controllerId, action: "closeFocusedInstance" }))}>
          ← {shellLabel("ui.common.backToMediaGraph")}
        </button>
        <span>·</span>
        <span>{appDocumentLabel(resolveDocumentByAppId(loadedPlugins, focusedSpawned.appId, focusedSpawned.document, uiTerminology))}</span>
      </div>
    ) : null;
    return (
      <div className="flex h-full min-h-0 flex-col overflow-hidden">
        {studioHomeBar}
        {focusedBar}
        <input
          ref={importStudioInputRef}
          type="file"
          accept="application/json,.json"
          className="hidden"
          onChange={(event) => {
            const file = event.target.files?.[0];
            if (!file) return;
            void file.text().then((json) => {
              onAction({ controllerId: landingControllerId ?? "", action: "importStudio", args: { json } });
              event.target.value = "";
            });
          }}
        />
        <div className="min-h-0 flex-1">
          <App
            modes={modes.map((mode) => ({ id: mode.id, label: resolveAppLabel(appLabelsOverlay, "mode", mode.id, mode.label), children: null }))}
            activeModeId={session.viewState.activeModeId ?? modes[0]?.id ?? session.app.id}
            onActiveModeChange={applyModeChange}
            chrome={false}
          >
            <Mode
              className="h-full w-full"
              mobile={mobile}
              windows={modeWindows}
              layout={effectiveModeLayout}
              activeWindowId={activeWindowId}
              onActiveWindowChange={(value) => dispatch({ type: "SET_ACTIVE_WINDOW_ID", value })}
              onLayoutChange={(value) => dispatch({ type: "SET_SHELL_LAYOUT", value })}
              onTemplateDrop={mobile ? undefined : handleTemplateDrop}
              onWindowClose={(windowId) => {
                if (studioMode && panel?.spawnedApps.some((entry) => entry.id === windowId)) {
                  const nextSpawned = panel.spawnedApps.filter((entry) => entry.id !== windowId);
                  updateStudioPanel(buildStudioPanelState(panel.programs, nextSpawned, panel.activePanelTab, nextSpawned[0]?.id));
                }
                dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: (current) => current.filter((entry) => entry.id !== windowId) });
                dispatch({
                  type: "SET_SHELL_LAYOUT",
                  value: (current) =>
                    current ??
                    convertFrameworkLayoutToModeLayout(
                      session.app.defaultLayout,
                      modeWindows.map((window) => window.id),
                      appLabelsOverlay,
                    ),
                });
              }}
            />
          </App>
        </div>
      </div>
    );
  }, [activeWindowId, effectiveModeLayout, error, handleTemplateDrop, loadedPlugins, mobile, modeWindows, navigateHistory, onAction, panel, session, studioMode, uiLocale, uiTerminology, updateStudioPanel]);

  const footerItems = useMemo(
    (): NavbarItem[] => [
      { key: "bottomLeftPanelTabs", content: <PanelChromeTabBar anchor="bottom-left" {...buildPanelSelectionProps("bottom-left")} /> },
      { key: "bottomMiddlePanelTabs", centered: true, content: <PanelChromeTabBar anchor="bottom-middle" {...buildPanelSelectionProps("bottom-middle")} /> },
      // 🏛️ A single leading fill spacer pushes the funding credit toward the trailing edge. Bracketing with a SECOND
      // flex-1 spacer on the other side would instead center it at the exact midpoint of the footer — directly under
      // the `centered` Command overlay above — so a fixed `w-huge` gap (not another flex-1, and not the far smaller
      // `w-double`, which read as flush against the toggle group) separates it from the corner toggle instead.
      navbarFillItem("footerLeadingFill"),
      fundedByZukunftBauFooterItem(),
      { key: "footerFundedByGap", className: "w-huge", content: null },
      { key: "bottomRightPanelTabs", content: <PanelChromeTabBar anchor="bottom-right" {...buildPanelSelectionProps("bottom-right")} /> },
    ],
    [buildPanelSelectionProps],
  );

  const buildPanelProps = useCallback(
    (anchor: PanelAnchor) => ({
      ...buildPanelSelectionProps(anchor),
      size: panels[anchor].size,
      onSizeChange: (value: number) => dispatch({ type: "SET_PANEL_SIZE", anchor, value }),
      tabBarHost: (PANEL_TAB_BAR_HOSTS[anchor] ? "chrome" : "panel") as "panel" | "chrome",
      treeOpenStates,
      onTreeOpenStateChange: (id: string, open: boolean) => dispatch({ type: "SET_TREE_OPEN_STATE", id, open }),
    }),
    [buildPanelSelectionProps, panels, treeOpenStates],
  );

  // #region 🔖ReadinessBeacon
  /** 🚦 Deterministic DOM beacon for headless smoke tests (e.g. Storybook's OS-shell plugin-boot matrix)
   * to wait on instead of screenshots/timeouts — set once a session resolves or errors, cleared on unmount. */
  useEffect(() => {
    const root = document.documentElement;
    const beaconId = pluginFilter ?? "unknown";
    if (error) {
      root.dataset.semioOsError = beaconId;
      delete root.dataset.semioOsReady;
    } else if (session) {
      root.dataset.semioOsReady = beaconId;
      delete root.dataset.semioOsError;
    }
    return () => {
      delete root.dataset.semioOsReady;
      delete root.dataset.semioOsError;
    };
  }, [session, error, pluginFilter]);
  // #endregion 🔖ReadinessBeacon

  return (
    <UIFindProvider>
      <LevelProvider level="window">
        <div className={`flex h-screen min-h-0 w-screen flex-col ${getLevelBgClass("window")}`}>
          <PanelDockProvider dock={dock} onTabDockDrop={handleTabDockDrop} onTreeUnitDockDrop={handleTreeUnitDockDrop}>
            <Layout
              mobile={mobile}
              mobilePanel={mobilePanel}
              navbar={<Navbar items={navbarItems} showFullscreenToggle />}
              footer={<Footer items={footerItems} />}
              panels={{
                "top-left": buildPanelProps("top-left"),
                "top-middle": buildPanelProps("top-middle"),
                "top-right": buildPanelProps("top-right"),
                "bottom-left": buildPanelProps("bottom-left"),
                "bottom-middle": buildPanelProps("bottom-middle"),
                "bottom-right": buildPanelProps("bottom-right"),
              }}
              canvas={<ShellRenderErrorBoundary>{canvas}</ShellRenderErrorBoundary>}
            />
          </PanelDockProvider>
        </div>
        <UISearch items={searchItems} open={searchOpen} onOpenChange={(value) => dispatch({ type: "SET_SEARCH_OPEN", value })} />
        <UIFind open={findOpen} onOpenChange={(value) => dispatch({ type: "SET_FIND_OPEN", value })} />
        {session && activeIntroduction && introductionStepIndex != null && (
          <UIIntroduction
            introduction={brand?.introduction ?? resolveIntroductionDefinition(activeIntroduction, appLabelsOverlay)}
            stepIndex={introductionStepIndex}
            anchorFallbackSelectors={introductionAnchorFallbackSelectors}
            onStepIndexChange={(value) => dispatch({ type: "SET_INTRODUCTION_STEP", value })}
            onDismiss={() => {
              dispatch({ type: "SET_INTRODUCTION_STEP", value: null });
              writeStoredIntroductionSeen(introductionSeenKey);
            }}
          />
        )}
        {session &&
          overlayDialog &&
          (() => {
            const dialog = session.app.dialogs?.find((entry) => entry.id === overlayDialog.dialogId);
            if (!dialog) return null;
            return (
              <UIDialog
                dialog={resolveDialogDefinition(dialog, appLabelsOverlay)}
                seedArgs={overlayDialog.seedArgs}
                renderField={(def, value, onChange) => renderStagedArgControl(def, value, onChange)}
                onSubmit={(args) => {
                  dispatch({ type: "SET_DIALOG", value: null });
                  onAction({ controllerId: session.app.controllerId, action: dialog.submitAction, args });
                }}
                onCancel={() => {
                  dispatch({ type: "SET_DIALOG", value: null });
                  if (dialog.cancelAction) onAction({ controllerId: session.app.controllerId, action: dialog.cancelAction });
                }}
              />
            );
          })()}
      </LevelProvider>
    </UIFindProvider>
  );
}
//#endregion FrameworkOsShell

//#region 🔖plugin-runtime

export type PluginWasmHandle = {
  readonly pluginId: string;
  readonly manifest: PluginManifest;
  readonly createApp: (appId: string) => Promise<number>;
  readonly destroyApp: (instanceId: number) => Promise<void>;
  readonly handleAction: (instanceId: number, actionJson: string, viewState: ViewState) => Promise<InvocationResponse>;
  /** 🎛️ Dispatches a scoped command (os/plugin/app/mode) — optional since not every plugin declares commands. */
  readonly handleCommand?: (instanceId: number, commandJson: string, viewState: ViewState) => Promise<InvocationResponse>;
  readonly render: (instanceId: number, bodyKey: string, viewState: ViewState) => Promise<UiNode>;
  readonly renderWithDocument?: (instanceId: number, bodyKey: string, viewState: ViewState, documentJson: string) => Promise<UiNode>;
  readonly refreshUi: (instanceId: number, request: PluginUiRefreshRequest) => Promise<PluginUiRefreshResponse>;
  /** 🔗 The `DocumentApp` document-sync surface (WS-D) — optional since not every plugin has migrated onto it yet (WS-F). */
  readonly applyOperations?: (instanceId: number, operationsJson: string) => Promise<void>;
  readonly readAppDocument?: (instanceId: number) => Promise<string>;
  readonly loadAppDocument?: (instanceId: number, documentJson: string) => Promise<void>;
  readonly attachBackbone?: (instanceId: number, uri: string) => Promise<void>;
  readonly detachBackbone?: (instanceId: number) => Promise<void>;
  readonly dispose: () => void;
};

export type { PluginRegistryEntry };

export async function loadPluginModule(pluginId: string, moduleUrl: string): Promise<PluginWasmHandle> {
  return adaptPluginHandle(await loadCorePluginModule(pluginId, moduleUrl));
}

export async function loadPluginWasm(pluginId: string, moduleUrl: string): Promise<PluginWasmHandle> {
  return adaptPluginHandle(await loadCorePluginWasm(pluginId, moduleUrl));
}

function adaptPluginHandle(handle: CorePluginWasmHandle): PluginWasmHandle {
  return {
    pluginId: handle.pluginId,
    manifest: handle.manifest as unknown as PluginManifest,
    createApp: (appId) => handle.createApp(appId),
    destroyApp: (instanceId) => handle.destroyApp(instanceId),
    handleAction: (instanceId, actionJson, viewState) => handle.handleAction(instanceId, actionJson, viewState),
    handleCommand: handle.handleCommand ? (instanceId, commandJson, viewState) => handle.handleCommand!(instanceId, commandJson, viewState) : undefined,
    render: async (instanceId, bodyKey, viewState) => (await handle.render(instanceId, bodyKey, viewState)) as unknown as UiNode,
    renderWithDocument: handle.renderWithDocument ? async (instanceId, bodyKey, viewState, documentJson) => (await handle.renderWithDocument!(instanceId, bodyKey, viewState, documentJson)) as unknown as UiNode : undefined,
    refreshUi: (instanceId, request) => handle.refreshUi(instanceId, request),
    applyOperations: handle.applyOperations ? (instanceId, operationsJson) => handle.applyOperations!(instanceId, operationsJson) : undefined,
    readAppDocument: handle.readAppDocument ? (instanceId) => handle.readAppDocument!(instanceId) : undefined,
    loadAppDocument: handle.loadAppDocument ? (instanceId, documentJson) => handle.loadAppDocument!(instanceId, documentJson) : undefined,
    attachBackbone: handle.attachBackbone ? (instanceId, uri) => handle.attachBackbone!(instanceId, uri) : undefined,
    detachBackbone: handle.detachBackbone ? (instanceId) => handle.detachBackbone!(instanceId) : undefined,
    dispose: () => handle.dispose(),
  };
}
//#endregion 🔖plugin-runtime

//#region 🔖wasm-session-loader

//#region 🔖EngineSessionLoader
/** 🔌 One generic lazy-loader table for every `framework/surface/*` engine crate (plus `board-2d`,
 * still app-hosted) — each surface kind maps to its wasm-pack package; a single cached module promise
 * per kind replaces the five near-identical hand-rolled `let xPromise…create X()` blocks this used to be. */
type EngineSessionWasmModule = { readonly default: (input?: unknown) => Promise<unknown> } & Record<string, new () => unknown>;

const ENGINE_SESSION_IMPORTERS: Record<string, () => Promise<EngineSessionWasmModule>> = {
  "node-graph": () => import("@semio-tech/framework-surface-node-graph-rs/pkg/framework_surface_node_graph.js"),
  "paint-2d": () => import("@semio-tech/framework-surface-paint-rs/pkg/framework_surface_paint.js"),
  "tiled-map": () => import("@semio-tech/framework-surface-tiled-map-rs/pkg/framework_surface_tiled_map.js"),
  terrain: () => import("@semio-tech/framework-surface-terrain-rs/pkg/framework_surface_terrain.js"),
  "board-2d": () => import("@semio-tech/puzzle-2d-rs/pkg/puzzle_2d.js"),
};

const engineSessionModulePromises = new Map<string, Promise<EngineSessionWasmModule>>();

async function createEngineSession<TSession>(engineKind: keyof typeof ENGINE_SESSION_IMPORTERS, sessionClassName: string): Promise<TSession> {
  let modulePromise = engineSessionModulePromises.get(engineKind);
  if (!modulePromise) {
    modulePromise = ENGINE_SESSION_IMPORTERS[engineKind]().then(async (mod) => {
      await mod.default();
      return mod;
    });
    engineSessionModulePromises.set(engineKind, modulePromise);
  }
  const mod = await modulePromise;
  return new mod[sessionClassName]() as TSession;
}
//#endregion 🔖EngineSessionLoader

//#region GraphSession
export async function createGraphSession(): Promise<GraphWasmSession> {
  return createEngineSession<GraphWasmSession>("node-graph", "GraphSession");
}
//#endregion GraphSession

//#region FlowSession
export type FlowWasmSession = GraphWasmSession & {
  loadFixtureJson(json: string): void;
  fixtureJson(): string;
  syncFromSceneJson?(json: string): void;
  setSelection(json: string): void;
  setPreviewOff(json: string): void;
  setCatalogueJson(json: string): void;
  setNeuronKindInfosJson(json: string): void;
  setComputingProgress(json: string): void;
  setAutomaticLod(enabled: boolean): void;
  setForcedDrawLodLabel(label: string): void;
  setCanvasThemeJson(json: string): void;
  setCamera(x: number, y: number, zoom: number): void;
  pointerDownScreen(sx: number, sy: number, button: number, shift: boolean, ctrlOrMeta: boolean, alt: boolean, pan: boolean): void;
  pointerMoveScreen(sx: number, sy: number, shift: boolean, ctrlOrMeta: boolean, alt: boolean): void;
  pointerUpScreen(sx: number, sy: number, shift: boolean, ctrlOrMeta: boolean, alt: boolean): void;
  wheelScreen(sx: number, sy: number, deltaX: number, deltaY: number, zoomGesture: boolean): void;
  labelOverlayPaintStateJson(): string;
  paramOverlayPaintStateJson(): string;
  stepperOverlayStateJson(): string;
  sliderOverlayStateJson(): string;
  selectionUnionBoundsScreenJson(): string;
  selectionPreviewPointsJson(): string;
  selectionPreviewCrossing(): boolean;
  selectedWidgetIds(): string;
  hoveredWidgetId(): string | undefined;
  hoveredChannelJson(): string;
  pickTargetsAtScreenJson(sx: number, sy: number): string;
  previewText(): string;
  preselectWidgetIdsJson(): string;
  previewOffWidgetIds(): string;
  alignSelection(mode: string): void;
  undo(): boolean;
  redo(): boolean;
  selectAll(): void;
  deleteSelection(): void;
  addWidget(descriptorJson: string, worldX: number, worldY: number): string;
  setGhostWidget(descriptorJson: string, worldX: number, worldY: number): void;
  clearGhostWidget(): void;
  worldFromScreen(sx: number, sy: number): string;
  evaluateSync(): string;
  noteInsertText(chunk: string): void;
  noteBackspace(): void;
  noteDeleteForward(): void;
  noteCommitEdit(): void;
  noteMoveCaret(direction: string, extend: boolean): void;
  setSliderValue(widgetId: string, value: number): void;
  setStepperFieldValue(widgetId: string, fieldKey: string, value: number): void;
  setNeuronParams(widgetId: string, paramsJson: string): void;
  setHover?(widgetId: string | null): void;
  setHoverChannel?(widgetId: string | null, port?: string | null): void;
  cameraJson?(): string;
};

type FlowSessionModule = {
  readonly default: (input?: unknown) => Promise<unknown>;
  readonly FlowSession: new () => FlowWasmSession;
};

let flowSessionPromise: Promise<FlowSessionModule> | null = null;

export async function createFlowSession(): Promise<FlowWasmSession> {
  if (!flowSessionPromise) {
    flowSessionPromise = import("@semio-tech/flow-core/pkg/flow_core.js").then(async (mod) => {
      await mod.default();
      return mod as FlowSessionModule;
    });
  }
  const mod = await flowSessionPromise;
  return new mod.FlowSession();
}
//#endregion FlowSession

//#region EditorSession
export type EditorWasmSession = GraphWasmSession & {
  syncFromSceneJson(json: string): void;
  setText(text: string): void;
  text(): string;
  caret(): number;
  anchor(): number;
  pointerDownScreen(sx: number, sy: number, button: number): void;
  pointerMoveScreen(sx: number, sy: number, buttons: number): void;
  pointerUpScreen(sx: number, sy: number, buttons: number): void;
  wheelScrollScreen(deltaY: number): void;
  insertText(text: string): void;
  backspace(): void;
  deleteForward(): void;
  selectAll(): void;
  replaceSelection(text: string): void;
  selectionText(): string;
  setCanvasThemeJson(json: string): void;
  hoverTokenRangeJson(): string;
  setHoverRange(start: number, end: number): void;
  cameraJson(): string;
  moveLeft(extend: boolean): void;
  moveRight(extend: boolean): void;
  moveUp(extend: boolean): void;
  moveDown(extend: boolean): void;
  moveLineStart(extend: boolean): void;
  moveLineEnd(extend: boolean): void;
  tabInsertText(): string;
  setSelectionRange(anchor: number, caret: number): void;
  selectSpanAt(offset: number): void;
  selectSpanAtScreen(sx: number, sy: number): void;
  pickTargetsAtScreenJson(sx: number, sy: number): string;
  caretWorldJson(): string;
  worldToScreenJson(wx: number, wy: number): string;
  setSelectionOccurrencesJson(json: string): void;
  setExtraCaretsJson(json: string): void;
  setCaretVisible(visible: boolean): void;
};

type EditorSessionModule = {
  readonly default: (input?: unknown) => Promise<unknown>;
  readonly EditorSession: new () => EditorWasmSession;
};

let editorSessionPromise: Promise<EditorSessionModule> | null = null;

export async function createEditorSession(): Promise<EditorWasmSession> {
  if (!editorSessionPromise) {
    editorSessionPromise = import("@semio-tech/framework-editor-rs/pkg/framework_editor.js").then(async (mod) => {
      await mod.default();
      return mod as EditorSessionModule;
    });
  }
  const mod = await editorSessionPromise;
  return new mod.EditorSession();
}
//#endregion EditorSession

//#region RasterSession
export type RasterWasmSession = {
  gpuReady(): boolean;
  attachCanvas(canvas: HTMLCanvasElement, logicalW: number, logicalH: number, dpr: number): Promise<unknown>;
  setSize(width: number, height: number, dpr: number): void;
  renderFrame(): void;
  setCamera(x: number, y: number, zoom: number): void;
  wheelScreen(sx: number, sy: number, deltaY: number): void;
  pointerDownScreen(sx: number, sy: number, button: number): void;
  pointerMoveScreen(sx: number, sy: number): void;
  pointerUpScreen(sx: number, sy: number): void;
  syncDocumentJson(json: string): void;
  uploadLayerImage(layerId: string, bytes: Uint8Array): void;
  uploadRasterImageKey(key: string, bytes: Uint8Array): void;
  setActiveUtility(utility: string): void;
  setBrushSize(size: number): void;
  setBrushOpacity(opacity: number): void;
  setHoveredIdSilent(id?: string | null): void;
  setSelectionIdsJson(json: string): void;
  setCanvasThemeJson(json: string): void;
  cameraJson(): string;
  setViewMode(mode: string, layerId?: string | null): void;
  pickTargetsAtScreenJson(sx: number, sy: number): string;
  marqueeHitsJson(queryJson: string): string;
  navigatorFitCameraJson(viewportW: number, viewportH: number): string;
  navigatorViewportOverlayJson(contentCameraJson: string, contentViewportJson: string): string;
  free(): void;
};

export async function createRasterSession(): Promise<RasterWasmSession> {
  return createEngineSession<RasterWasmSession>("paint-2d", "RasterSession");
}
//#endregion RasterSession

//#region MapSession
export type MapWasmSession = {
  attachCanvas(canvas: HTMLCanvasElement, logicalW: number, logicalH: number, dpr: number): Promise<unknown>;
  setSize(width: number, height: number, dpr: number): void;
  renderFrame(): void;
  setCamera(x: number, y: number, zoom: number): void;
  cameraJson(): string;
  cameraLimitsJson(): string;
  fitWorldCamera(): void;
  reclampCamera(): void;
  pointerDownScreen(sx: number, sy: number, button: number): void;
  pointerMoveScreen(sx: number, sy: number): void;
  pointerUpScreen(sx: number, sy: number): void;
  wheelScreen(sx: number, sy: number, deltaY: number): void;
  syncMapJson(json: string): void;
  uploadTile(z: number, x: number, y: number, bytes: Uint8Array): void;
  uploadVectorTile(z: number, x: number, y: number, bytes: Uint8Array): void;
  visibleTilesJson(): string;
  visibleVectorTilesJson(): string;
  setRenderMode(mode: string): void;
  setVectorStyle(style: string): void;
  setLodMode(mode: string): void;
  setLayerVisibilityJson(json: string): void;
  setLayerStrokeScaleJson(json: string): void;
  setSelectionJson(json: string): void;
  setHoverJson(json: string): void;
  featuresInRectJson(x0: number, y0: number, x1: number, y1: number, crossing: boolean): string;
  featuresInPolygonJson(pointsJson: string, crossing: boolean): string;
  hitTestFeatureJson(sx: number, sy: number): string;
  featureScreenJson(kind: string, id: string): string;
  positionScreenJson(id: string): string;
  currentLodJson(): string;
  setMapThemeJson(json: string): void;
  gpuReady(): boolean;
  free(): void;
};

export async function createMapSession(): Promise<MapWasmSession> {
  return createEngineSession<MapWasmSession>("tiled-map", "MapSession");
}
//#endregion MapSession

//#region TerrainSession
export type TerrainWasmSession = {
  set_project_origin(lon: number, lat: number): void;
  set_exaggeration(exaggeration: number): void;
  visible_terrain_tiles_json(cameraJson: string): string;
  upload_elevation_tile(z: number, x: number, y: number, bytes: Uint8Array): boolean;
  evict_terrain_tile(z: number, x: number, y: number): void;
  terrain_tile_mesh_json(z: number, x: number, y: number): string;
};

export async function createTerrainSession(): Promise<TerrainWasmSession> {
  return createEngineSession<TerrainWasmSession>("terrain", "TerrainSession");
}
//#endregion TerrainSession

//#region Board2dSession
export type Board2dWasmSession = {
  attach_canvas(canvas: HTMLCanvasElement, logicalW: number, logicalH: number, dpr: number): Promise<unknown>;
  setSize(width: number, height: number, dpr: number): void;
  renderFrame(): void;
  parseFixtureJson(json: string): boolean;
  syncDescriptorJson(json: string): void;
  setKindCatalogsJson(json: string): void;
  setCamera(x: number, y: number, zoom: number): void;
  setSelectionIdsJson(json: string): void;
  setCanvasThemeJson(json: string): void;
  pointerDownScreen(sx: number, sy: number, button: number, shift: boolean, ctrlOrMeta: boolean): void;
  pointerMoveScreen(sx: number, sy: number, shift: boolean, ctrlOrMeta: boolean, alt: boolean): void;
  pointerUpScreen(sx: number, sy: number, shift: boolean, ctrlOrMeta: boolean, alt: boolean): void;
  wheelScreen(sx: number, sy: number, deltaY: number): void;
  drainEventsJson(): string;
  cameraJson(): string;
  gpuReady(): boolean;
  setHoveredIdSilent?(id?: string | null): void;
  setActiveUtility?(label: string): void;
  setSelectionOptions?(method: string, mode: string, selectNodes: boolean, selectEdges: boolean, selectHandles: boolean): void;
  setGridSnapEnabled?(enabled: boolean): void;
  setGridFactor?(v: number): void;
  setSuggestionOffset?(distance: number): void;
  setBrushKindWeights?(json: string): void;
  setHandleLinkCompatJson?(json: string): void;
  setAutomaticLod?(enabled: boolean): void;
  setForcedDrawLodLabel?(label: string): void;
  setSelectionIdsJsonSilent?(json: string): void;
  setCameraSilent?(x: number, y: number, zoom: number): void;
  pointerLeaveScreen?(alt: boolean): void;
  pickTargetsAtScreenJson?(sx: number, sy: number): string;
  deleteSelection?(): void;
  cancelAreaSelect?(): boolean;
  brushCycleCandidate?(forward: boolean): void;
  setFixtureDropPreviewJson?(json: string): void;
  clearFixtureDropPreview?(): void;
  defersDescriptorSyncFromJs?(): boolean;
  isDraggingAreaSelect?(): boolean;
  /** @emoji 🐢 Silent cross-pane mirror setters (WS-live-sync round 4) — move nodes/set preselect/set the marquee outline without emitting board events or a fixture reset, so a peer pane can mirror another pane's live gesture without round-tripping through the plugin. */
  setNodePositionsJson?(json: string): void;
  setPreselectStateJsonSilent?(json: string): void;
  setSelectionScreenPreview?(flatXy: readonly number[]): void;
  clearSelectionScreenPreview?(): void;
  free(): void;
};

export async function createBoard2dSession(): Promise<Board2dWasmSession> {
  return createEngineSession<Board2dWasmSession>("board-2d", "BoardSession");
}
//#endregion Board2dSession

//#region SceneHelpers
export function isFlowGraphScene(capabilitiesJson?: string): boolean {
  if (!capabilitiesJson) return false;
  try {
    const caps = JSON.parse(capabilitiesJson) as { readonly engine?: string; readonly spotlight?: boolean; readonly noteEdit?: boolean };
    return caps.engine === "flow" || caps.spotlight === true || caps.noteEdit === true;
  } catch {
    return false;
  }
}
//#endregion SceneHelpers
//#endregion 🔖wasm-session-loader

//#region 🔖ui-search-find

//#region UISearch
export type UISearchItem = {
  readonly id: string;
  readonly label: string;
  readonly description?: string;
  readonly icon?: ReactNode;
  readonly category?: string;
  readonly onSelect: () => void;
};

export function UISearch({
  items,
  open,
  onOpenChange,
  placeholder = shellLabel("ui.search.placeholder"),
  emptyMessage = shellLabel("ui.search.empty"),
}: {
  readonly items: readonly UISearchItem[];
  readonly open: boolean;
  readonly onOpenChange: (open: boolean) => void;
  readonly placeholder?: string;
  readonly emptyMessage?: string;
}) {
  const [query, setQuery] = useState("");
  const fuse = useMemo(
    () =>
      new Fuse(items, {
        keys: [
          { name: "label", weight: 2 },
          { name: "description", weight: 1 },
          { name: "category", weight: 0.5 },
        ],
        threshold: 0.4,
        includeScore: true,
      }),
    [items],
  );
  const results = useMemo(() => {
    if (query.trim()) return fuse.search(query).slice(0, 20);
    return items.slice(0, 20).map((item, idx) => ({ item, refIndex: idx, score: 0 }) as FuseResult<UISearchItem>);
  }, [fuse, items, query]);
  const grouped = useMemo(() => {
    const groups: Record<string, FuseResult<UISearchItem>[]> = {};
    for (const result of results) {
      const category = result.item.category || "";
      if (!groups[category]) groups[category] = [];
      groups[category].push(result);
    }
    return groups;
  }, [results]);
  const handleSelect = useCallback(
    (item: UISearchItem) => {
      onOpenChange(false);
      setQuery("");
      item.onSelect();
    },
    [onOpenChange],
  );

  return (
    <CommandDialog title={shellLabel("ui.search.title")} description={shellLabel("ui.search.description")} open={open} onOpenChange={onOpenChange} shouldFilter={false}>
      <CommandInput id="ui.search.input" placeholder={placeholder} value={query} onValueChange={setQuery} />
      <CommandList>
        <CommandEmpty>{emptyMessage}</CommandEmpty>
        {Object.entries(grouped).map(([category, categoryResults]) => (
          <CommandGroup key={category || "__default"} heading={category || undefined}>
            {categoryResults.map((result, idx) => (
              <CommandItem key={`${result.item.id}-${idx}`} value={`${result.item.label} ${result.item.description ?? ""} ${result.item.category ?? ""}`.trim()} onSelect={() => handleSelect(result.item)}>
                <div className="flex items-center gap-single">
                  {result.item.icon}
                  <div className="flex flex-col">
                    <span>{result.item.label}</span>
                    {result.item.description ? <span className="text-xs text-muted-foreground">{result.item.description}</span> : null}
                  </div>
                </div>
              </CommandItem>
            ))}
          </CommandGroup>
        ))}
      </CommandList>
    </CommandDialog>
  );
}
//#endregion UISearch

//#region UIFind
export type UIFindItem = {
  readonly id: string;
  readonly label: string;
  readonly description?: string;
  readonly category?: string;
};

export type UIFindContextValue = {
  readonly findItems: readonly UIFindItem[];
  readonly setFindItems: (items: readonly UIFindItem[]) => void;
  readonly setOnFindItem: (callback: ((itemId: string) => void) | undefined) => void;
  readonly triggerFindItem: (itemId: string) => void;
};

const UIFindContext = createContext<UIFindContextValue | null>(null);

function areFindItemsShallowEqual(previousItems: readonly UIFindItem[], nextItems: readonly UIFindItem[]): boolean {
  if (previousItems === nextItems) return true;
  if (previousItems.length !== nextItems.length) return false;
  for (let index = 0; index < nextItems.length; index += 1) {
    const previous = previousItems[index];
    const next = nextItems[index];
    if (!previous || !next || previous.id !== next.id || previous.label !== next.label || previous.description !== next.description || previous.category !== next.category) {
      return false;
    }
  }
  return true;
}

export function UIFindProvider({ children }: { readonly children: ReactNode }) {
  const [findItems, setFindItemsState] = useState<readonly UIFindItem[]>([]);
  const onFindItemCallbackRef = useRef<((itemId: string) => void) | undefined>(undefined);
  const setFindItems = useCallback((items: readonly UIFindItem[]) => {
    setFindItemsState((previousItems) => (areFindItemsShallowEqual(previousItems, items) ? previousItems : items));
  }, []);
  const setOnFindItem = useCallback((callback: ((itemId: string) => void) | undefined) => {
    onFindItemCallbackRef.current = callback;
  }, []);
  const triggerFindItem = useCallback((itemId: string) => {
    onFindItemCallbackRef.current?.(itemId);
  }, []);
  const contextValue = useMemo(() => ({ findItems, setFindItems, setOnFindItem, triggerFindItem }), [findItems, setFindItems, setOnFindItem, triggerFindItem]);
  return <UIFindContext.Provider value={contextValue}>{children}</UIFindContext.Provider>;
}

export function useUIFind(): UIFindContextValue {
  const context = useContext(UIFindContext);
  if (!context) throw new Error("useUIFind must be used within UIFindProvider");
  return context;
}

export function useUIFindSafe(): UIFindContextValue | null {
  return useContext(UIFindContext);
}

export function UIFind({
  open,
  onOpenChange,
  placeholder = shellLabel("ui.find.placeholder"),
  emptyMessage = shellLabel("ui.find.empty"),
}: {
  readonly open: boolean;
  readonly onOpenChange: (open: boolean) => void;
  readonly placeholder?: string;
  readonly emptyMessage?: string;
}) {
  const [query, setQuery] = useState("");
  const findContext = useContext(UIFindContext);
  const findItems = findContext?.findItems ?? [];
  const triggerFindItem = findContext?.triggerFindItem;
  const fuse = useMemo(
    () =>
      new Fuse(findItems, {
        keys: [
          { name: "label", weight: 2 },
          { name: "description", weight: 1 },
          { name: "category", weight: 0.5 },
        ],
        threshold: 0.4,
        includeScore: true,
      }),
    [findItems],
  );
  const results = useMemo(() => {
    if (query.trim()) return fuse.search(query).slice(0, 20);
    return findItems.slice(0, 20).map((item, idx) => ({ item, refIndex: idx, score: 0 }) as FuseResult<UIFindItem>);
  }, [findItems, fuse, query]);
  const grouped = useMemo(() => {
    const groups: Record<string, FuseResult<UIFindItem>[]> = {};
    for (const result of results) {
      const category = result.item.category || "";
      if (!groups[category]) groups[category] = [];
      groups[category].push(result);
    }
    return groups;
  }, [results]);
  const handleSelect = useCallback(
    (item: UIFindItem) => {
      onOpenChange(false);
      setQuery("");
      triggerFindItem?.(item.id);
    },
    [onOpenChange, triggerFindItem],
  );

  if (!findContext) return null;

  return (
    <CommandDialog title={shellLabel("ui.find.title")} description={shellLabel("ui.find.description")} open={open} onOpenChange={onOpenChange} shouldFilter={false}>
      <CommandInput id="ui.find.input" placeholder={placeholder} value={query} onValueChange={setQuery} />
      <CommandList>
        <CommandEmpty>{emptyMessage}</CommandEmpty>
        {Object.entries(grouped).map(([category, categoryResults]) => (
          <CommandGroup key={category || "__default"} heading={category || undefined}>
            {categoryResults.map((result, idx) => (
              <CommandItem key={`${result.item.id}-${idx}`} value={`${result.item.label} ${result.item.description ?? ""} ${result.item.category ?? ""}`.trim()} onSelect={() => handleSelect(result.item)}>
                <div className="flex flex-col">
                  <span>{result.item.label}</span>
                  {result.item.description ? <span className="text-xs text-muted-foreground">{result.item.description}</span> : null}
                </div>
              </CommandItem>
            ))}
          </CommandGroup>
        ))}
      </CommandList>
    </CommandDialog>
  );
}
//#endregion UIFind
//#endregion 🔖ui-search-find

//#region 🔖sync-attach-card

type SyncAttachCardProps = {
  readonly activeUri: string | null;
  readonly cardKind: SyncCardKind | null;
  readonly draftPath: string;
  readonly syncUtilities: readonly FrameworkSyncUtilityLeaf[];
  readonly status: DocumentSyncStatus | null;
  readonly onAction: (action: ActionDescriptor) => void;
  readonly onDraftPathChange: (value: string) => void;
  readonly onClose: () => void;
  readonly onAttach: (uri: string) => void;
  readonly onDetach: () => void;
};

/** 🚦 Minimal status label for a `DocumentSyncStatus` — matches this file's small-badge-text style
 * (see the `activeUri` line right below it), not a new component system. */
function syncStatusLabel(status: DocumentSyncStatus | null): string | null {
  if (!status) return null;
  const remote =
    status.remote.kind === "live" ? `live · ${status.remote.peerCount} peer${status.remote.peerCount === 1 ? "" : "s"}` : status.remote.kind === "connecting" ? "connecting…" : status.remote.kind === "backoff" ? "reconnecting…" : "offline";
  const persisted = status.persisted ? "saved" : "unsaved";
  const pending = status.pendingOps > 0 ? ` · ${status.pendingOps} pending` : "";
  return `${remote} · ${persisted}${pending}`;
}

function SyncAttachCard({ activeUri, cardKind, draftPath, syncUtilities, status, onAction, onDraftPathChange, onClose, onAttach, onDetach }: SyncAttachCardProps): ReactElement {
  const open = cardKind != null;
  const placeholder = cardKind === "remote" ? "127.0.0.1:8787/studio-1/demo" : cardKind === "folder" ? "/absolute/project/folder" : "/absolute/document.json";

  const attachFromDraft = () => {
    if (!cardKind || !draftPath.trim()) return;
    if (cardKind === "remote") {
      const [hostPort, ...rest] = draftPath.split("/");
      const [studioId, documentId] = rest.length >= 2 ? [rest[0], rest.slice(1).join("/")] : ["default", rest[0] || "document"];
      onAttach(buildRemoteBackboneUri(hostPort || draftPath, studioId, documentId));
      return;
    }
    onAttach(cardKind === "folder" ? buildFolderBackboneUri(draftPath) : buildFileBackboneUri(draftPath));
  };

  return (
    <Popover
      open={open}
      onOpenChange={(nextOpen) => {
        if (!nextOpen) onClose();
      }}
    >
      <PopoverAnchor asChild>
        <div>
          <UtilityTree utilities={groupUtilityNodesByCategory(syncUtilities as readonly UtilityNode[], ["sync"])} onAction={onAction} />
        </div>
      </PopoverAnchor>
      {open ? (
        <PopoverContent side="top" align="center" className="w-80 space-y-3 p-3">
          <div className="space-y-1">
            <p className="text-sm font-medium capitalize">{cardKind} backbone</p>
            {activeUri ? <p className="break-all text-xs text-muted-foreground">{activeUri}</p> : null}
            {activeUri && status ? <p className="text-xs text-muted-foreground">{syncStatusLabel(status)}</p> : null}
          </div>
          <Input value={draftPath} placeholder={placeholder} onChange={(event) => onDraftPathChange(event.target.value)} />
          <div className="flex items-center gap-2">
            <Button type="button" onClick={attachFromDraft}>
              Attach
            </Button>
            {activeUri ? (
              <Button type="button" onClick={onDetach}>
                Detach
              </Button>
            ) : null}
          </div>
        </PopoverContent>
      ) : null}
    </Popover>
  );
}
//#endregion 🔖sync-attach-card

//#region 🔖utility-tree

type UtilityTreeProps = {
  readonly utilities: readonly UtilityNode[];
  readonly onAction: (action: ActionDescriptor) => void;
  readonly id?: string;
  /** @emoji 🎀 `up` stacks a new ribbon line above the base row per pressed collection (window utility bar); `inline` keeps the horizontal drill-down (footer). */
  readonly direction?: RibbonDirection;
  /** @emoji 🎓 A utility id the introduction walkthrough is anchored on — when it names a leaf nested inside
   * a collapsed group picker, the picker auto-drills into that group so the leaf actually mounts (see
   * {@link findUtilityGroupPath}). `null`/not-found leaves `activePath` alone. */
  readonly revealUtilityId?: string | null;
  /** @emoji 🎯 Utility-scoped measure chrome for the active utility — rendered as an extra ribbon row under the utilities. */
  readonly utilityOptions?: ReactNode;
};

function resolveLeafAction(node: UtilityLeaf | Extract<UtilityNode, { readonly kind: "button" | "toggle" }>): ActionDescriptor | null {
  if ("onPress" in node && node.onPress) return node.onPress;
  if ("onChange" in node && node.onChange) return node.onChange;
  if (node.kind === "button" || node.kind === "toggle") {
    if (!node.action || !node.controllerId) return null;
    return { controllerId: node.controllerId, action: node.action, args: node.args as Record<string, unknown> | undefined };
  }
  return null;
}

function utilityIcon(iconId: string): IconName {
  return iconId in ICONS ? (iconId as IconName) : "circle";
}

/** @emoji 🔢 Sorts utility nodes by `order`. */
export function sortUtilityNodes(nodes: readonly UtilityNode[]): UtilityNode[] {
  return [...nodes].sort((left, right) => (left.order ?? 0) - (right.order ?? 0));
}

//#region 🗂️UtilityCategoryGrouping

const UTILITY_CATEGORY_ORDER: readonly UtilityCategory[] = ["selection", "utilities", "history", "sync"];

/** @emoji 🪟 Categories that are scoped to whatever window/pane the user is interacting with — selecting or editing content varies per window, so these live in each window's own bottom-left panel. */
const UTILITY_CATEGORIES: readonly UtilityCategory[] = ["selection", "utilities"];

const UTILITY_CATEGORY_ICON_ID: Readonly<Record<UtilityCategory, string>> = {
  selection: "mouse-pointer",
  utilities: "wrench",
  history: "undo",
  sync: "cloud",
};

function utilityNodeCategory(node: UtilityNode): UtilityCategory {
  if (node.kind === "separator") return "utilities";
  if (node.category) return node.category;
  return "utilities";
}

/**
 * 🕰️ Framework-owned History utility nodes derived from an app's registry (the six injected History
 * actions). Sources the bottom-right History footer tab now that the plugin `list-tools` surface is gone.
 */
export function frameworkHistoryUtilityNodes(app: Pick<AppDefinition, "actions" | "controllerId">): UtilityNode[] {
  return (app.actions ?? [])
    .filter((action) => action.kind === "history")
    .map((action, order) => ({
      id: action.id,
      kind: "button" as const,
      iconId: action.iconId ?? "undo",
      label: action.label,
      text: action.label,
      order,
      category: "history" as const,
      onPress: { controllerId: app.controllerId, action: action.id },
    }));
}

/** @emoji 🗂️ Buckets top-level utility nodes into the given categories (default: all) so activating a category expands the panel with another line, matching {@link buildUtilityRibbonSegments}'s one-active-group-per-level picker. A category with a single already-meaningful collection is used as-is instead of being re-wrapped in a synthetic one, avoiding a redundant picker level with a duplicate-looking label (e.g. a lone "Selection" collection nested under a "Selection" category chip). Separators default to `utilities` (mirrors Rust `UtilityNode::category()`), so dividers between same-category runs survive; dividers that only separated different categories become redundant once those categories are separate picker lines. */
export function groupUtilityNodesByCategory(nodes: readonly UtilityNode[], categories: readonly UtilityCategory[] = UTILITY_CATEGORY_ORDER): UtilityNode[] {
  const buckets = new Map<UtilityCategory, UtilityNode[]>();
  for (const node of nodes) {
    const category = utilityNodeCategory(node);
    if (!categories.includes(category)) continue;
    const bucket = buckets.get(category) ?? [];
    bucket.push(node);
    buckets.set(category, bucket);
  }
  return categories
    .filter((category) => hasInteractiveUtilityNodes(buckets.get(category)))
    .map((category, order) => {
      const bucket = buckets.get(category)!;
      if (bucket.length === 1 && bucket[0].kind === "collection") return { ...bucket[0], order };
      return { id: category, kind: "collection" as const, iconId: UTILITY_CATEGORY_ICON_ID[category], text: category, order, category, children: bucket };
    });
}

/** @emoji 🦶 Deduplicates utility nodes by id across every window's utility set (mode-wide utilities are attached identically to each window kind when a plugin doesn't differentiate per window), for a single shared footer entry per utility. */
export function dedupeUtilityNodesById(nodeLists: readonly (readonly UtilityNode[])[]): UtilityNode[] {
  const seen = new Map<string, UtilityNode>();
  for (const nodes of nodeLists) {
    for (const node of nodes) {
      if (!seen.has(node.id)) seen.set(node.id, node);
    }
  }
  return [...seen.values()];
}

//#endregion 🗂️UtilityCategoryGrouping

function isInteractiveUtilityNode(node: UtilityNode): boolean {
  if (node.kind === "separator") return false;
  if (node.kind === "collection") return hasInteractiveUtilityNodes(node.children);
  return true;
}

function hasInteractiveUtilityNodes(nodes?: readonly UtilityNode[]): boolean {
  return Boolean(nodes?.some((node) => isInteractiveUtilityNode(node)));
}

function hasInteractiveUtilityLeaves(items: readonly UtilityLeaf[]): boolean {
  return items.some((node) => node.kind !== "separator");
}

type UtilityCollectionNode = Extract<UtilityNode, { readonly kind: "collection" }>;

export type UtilityRibbonSegment = { readonly kind: "picker"; readonly collections: readonly UtilityCollectionNode[]; readonly depth: number } | { readonly kind: "utilities"; readonly items: readonly UtilityLeaf[]; readonly depth: number };

/** @emoji 🎀 Builds drill-down ribbon segments from a utility tree and active collection path; `depth` marks how many collections were drilled into to reach a segment. Collections never auto-activate: a level only recurses when `path[depth]` names one of its enabled collections, so at most one group per level is active and an unresolved level simply shows its picker. */
export function buildUtilityRibbonSegments(nodes: readonly UtilityNode[], path: readonly string[], depth = 0): UtilityRibbonSegment[] {
  const sorted = sortUtilityNodes(nodes);
  const collections = sorted.filter((node): node is UtilityCollectionNode => node.kind === "collection" && !node.disabled);
  const looseLeaves = sorted.filter((node): node is UtilityLeaf => node.kind !== "collection");
  const segments: UtilityRibbonSegment[] = [];

  if (collections.length > 0) segments.push({ kind: "picker", collections, depth });
  if (hasInteractiveUtilityLeaves(looseLeaves)) segments.push({ kind: "utilities", items: looseLeaves, depth });
  if (collections.length === 0) return segments;

  const activeId = path[depth];
  const active = activeId ? collections.find((node) => node.id === activeId) : undefined;
  if (!active) return segments;
  return [...segments, ...buildUtilityRibbonSegments(active.children, path, depth + 1)];
}

/** @emoji 🎀 Validates an active-group path against the current utility tree: keeps each entry only while it still names an enabled collection at that level, truncating at the first miss rather than substituting a default. */
export function reconcileUtilityPath(nodes: readonly UtilityNode[], path: readonly string[]): readonly string[] {
  let current = nodes;
  const reconciled: string[] = [];

  for (const collectionId of path) {
    const collections = sortUtilityNodes(current).filter((node): node is UtilityCollectionNode => node.kind === "collection" && !node.disabled);
    const active = collections.find((node) => node.id === collectionId);
    if (!active) break;
    reconciled.push(collectionId);
    current = active.children;
  }

  return reconciled;
}

/** @emoji 🎓 Finds the group-id path (in {@link reconcileUtilityPath} shape) leading down to a utility leaf,
 * so a folded picker can drill straight to it. Returns `[]` when the id is a top-level (ungrouped) node,
 * `null` when the tree has no node with that id at all. */
export function findUtilityGroupPath(nodes: readonly UtilityNode[], targetId: string, prefix: readonly string[] = []): readonly string[] | null {
  for (const node of nodes) {
    if (node.id === targetId) return prefix;
    if (node.kind === "collection") {
      const nested = findUtilityGroupPath(node.children, targetId, [...prefix, node.id]);
      if (nested) return nested;
    }
  }
  return null;
}

function UtilityRibbonItems({ items, onAction }: { readonly items: readonly UtilityLeaf[]; readonly onAction: (action: ActionDescriptor) => void }): ReactElement {
  const sorted = useMemo(() => sortUtilityNodes(items) as UtilityLeaf[], [items]);
  const nodes = useMemo(() => {
    const rendered: ReactElement[] = [];
    let buttonRun: UtilityLeaf[] = [];
    let toggleRun: UtilityLeaf[] = [];

    const flushButtons = () => {
      if (buttonRun.length === 0) return;
      const run = buttonRun;
      buttonRun = [];
      rendered.push(
        <RibbonItem key={`buttons-${run.map((entry) => entry.id).join("-")}`}>
          <ButtonGroup>
            {run.map((entry) => {
              const action = resolveLeafAction(entry);
              if (!action) return null;
              return (
                <ButtonGroupItem
                  key={entry.id}
                  id={entry.id}
                  aria-label={entry.title ?? entry.label ?? entry.id}
                  title={entry.title ?? entry.label}
                  disabled={entry.disabled}
                  onClick={() => onAction(action)}
                  icon={<Icon icon={utilityIcon(entry.iconId)} size="small" />}
                  text={entry.text ?? entry.label}
                />
              );
            })}
          </ButtonGroup>
        </RibbonItem>,
      );
    };

    const flushToggles = () => {
      if (toggleRun.length === 0) return;
      const run = toggleRun;
      toggleRun = [];
      rendered.push(
        <RibbonItem key={`toggles-${run.map((entry) => entry.id).join("-")}`}>
          <ToggleGroup
            kind="multiple"
            value={run.filter((entry) => entry.pressed).map((entry) => entry.id)}
            onValueChange={(values) => {
              for (const entry of run) {
                const action = resolveLeafAction(entry);
                if (!action) continue;
                const pressed = values.includes(entry.id);
                if ((entry.pressed ?? false) !== pressed) onAction(action);
              }
            }}
            items={run.map((entry) => ({
              value: entry.id,
              id: entry.id,
              icon: <Icon icon={utilityIcon(entry.iconId)} size="small" />,
              text: entry.text ?? entry.label,
            }))}
          />
        </RibbonItem>,
      );
    };

    const flushRuns = () => {
      flushButtons();
      flushToggles();
    };

    for (const item of sorted) {
      if (item.kind === "separator") {
        flushRuns();
        rendered.push(<RibbonDivider key={item.id} />);
        continue;
      }
      if (item.kind === "toggle") {
        flushButtons();
        toggleRun.push(item);
        continue;
      }
      flushToggles();
      buttonRun.push(item);
    }
    flushRuns();
    return rendered;
  }, [onAction, sorted]);

  return <RibbonGroup>{nodes}</RibbonGroup>;
}

function utilityRibbonSegmentKey(segment: UtilityRibbonSegment, index: number): string {
  return segment.kind === "picker" ? `picker-${segment.depth}-${segment.collections.map((entry) => entry.id).join("-")}` : `utilities-${index}-${segment.items.map((entry) => entry.id).join("-")}`;
}

export function UtilityTree({ utilities, onAction, id = "ui.utilities", direction = "inline", revealUtilityId = null, utilityOptions }: UtilityTreeProps): ReactElement | null {
  const [activePath, setActivePath] = useState<readonly string[]>([]);

  useEffect(() => {
    setActivePath((previousPath) => reconcileUtilityPath(utilities, previousPath));
  }, [utilities]);

  useEffect(() => {
    if (!revealUtilityId) return;
    const path = findUtilityGroupPath(utilities, revealUtilityId);
    if (path) setActivePath((previousPath) => (previousPath.length === path.length && previousPath.every((entry, index) => entry === path[index]) ? previousPath : path));
  }, [revealUtilityId, utilities]);

  const segments = useMemo(() => buildUtilityRibbonSegments(utilities, activePath), [utilities, activePath]);

  if (!hasInteractiveUtilityNodes(utilities) && !utilityOptions) return null;

  const renderSegment = (segment: UtilityRibbonSegment): ReactNode =>
    segment.kind === "picker" ? (
      <RibbonItem>
        <ToggleGroup
          kind="single"
          value={activePath[segment.depth] ?? ""}
          onValueChange={(value) => {
            setActivePath(value ? reconcileUtilityPath(utilities, [...activePath.slice(0, segment.depth), value]) : activePath.slice(0, segment.depth));
          }}
          items={segment.collections.map((entry) => ({
            value: entry.id,
            id: `${id}.group.${entry.id}`,
            icon: <Icon icon={utilityIcon(entry.iconId)} size="small" />,
            text: entry.text ?? entry.label,
          }))}
        />
      </RibbonItem>
    ) : (
      <UtilityRibbonItems items={segment.items} onAction={onAction} />
    );

  const windowId = id.startsWith("ui.utilities.") ? id.slice("ui.utilities.".length) : "";

  const findPressedSelectionUtility = (nodes: readonly UtilityNode[]): UtilityNode | undefined => {
    for (const node of nodes) {
      if (node.kind === "collection") {
        const found = findPressedSelectionUtility(node.children);
        if (found) return found;
      } else if (node.kind === "toggle" && node.pressed && node.id.startsWith("select")) {
        return node;
      }
    }
    return undefined;
  };

  const activeSelectionUtility = findPressedSelectionUtility(utilities);
  const hasActiveSelection = activeSelectionUtility != null;

  const rows: RibbonRow[] =
    direction === "inline"
      ? segments.map((segment, index) => ({ key: utilityRibbonSegmentKey(segment, index), content: renderSegment(segment) }))
      : Array.from(
          segments.reduce((byDepth, segment, index) => {
            const zones = byDepth.get(segment.depth) ?? [];
            zones.push(<RibbonZone key={utilityRibbonSegmentKey(segment, index)}>{renderSegment(segment)}</RibbonZone>);
            byDepth.set(segment.depth, zones);
            return byDepth;
          }, new Map<number, ReactElement[]>()),
        )
          .sort(([left], [right]) => left - right)
          .map(([depth, content]) => ({ key: `row-${depth}`, content }));

  if (utilityOptions && direction !== "inline") {
    rows.push({
      key: "row-utility-options",
      content: (
        <RibbonZone>
          <RibbonItem>{utilityOptions}</RibbonItem>
        </RibbonZone>
      ),
    });
  } else if (hasActiveSelection && direction !== "inline") {
    rows.push({
      key: "row-selection-options",
      content: (
        <RibbonZone>
          <RibbonItem>
            <SelectionUtilityOptions activeUtilityId={activeSelectionUtility.id} windowId={windowId} onAction={onAction} />
          </RibbonItem>
        </RibbonZone>
      ),
    });
  }

  return (
    <UiChromeLabelPolicyProvider policy="always">
      <Ribbon id={id} direction={direction} rows={rows} />
    </UiChromeLabelPolicyProvider>
  );
}
//#endregion 🔖utility-tree

//#region 🔖os-chrome-panels

//#region DisplayPanel
export type DisplayHostApi = {
  readonly windowKinds: readonly { readonly id: string; readonly label: string }[];
  readonly namedLayouts: readonly NamedLayout[];
  readonly userLayouts: readonly NamedLayout[];
  readonly saveCurrentLayout: (label: string) => void;
  readonly applyNamedLayout: (layoutId: string) => void;
  readonly deleteUserLayout: (layoutId: string) => void;
  readonly layoutSaveLabel: string;
  readonly setLayoutSaveLabel: (value: string) => void;
};

const FRAMEWORK_DISPLAY_WINDOWS_TAB_ID = "framework.display.windows";
const FRAMEWORK_DISPLAY_LAYOUT_TAB_ID = "framework.display.layout";
const FRAMEWORK_SETTINGS_GENERAL_TAB_ID = "framework.settings.general";
const FRAMEWORK_SETTINGS_THEME_TAB_ID = "framework.settings.theme";

function groupNamedLayoutsToTreeItems(layouts: readonly NamedLayout[], onApply: (layoutId: string) => void, onDeleteUser?: (layoutId: string) => void): TreeDataItem[] {
  const root: TreeDataItem[] = [];
  const folderByKey = new Map<string, TreeDataItem>();
  const layoutLeaf = (entry: NamedLayout): TreeDataItem => ({
    id: `framework.display.layout.${entry.id}`,
    label: entry.label,
    onClick: () => onApply(entry.id),
    ...(entry.origin === "user" && onDeleteUser
      ? {
          actions: [
            {
              id: `framework.display.delete.${entry.id}`,
              icon: <Icon icon="trash-2" size="small" />,
              onClick: () => onDeleteUser(entry.id),
            },
          ],
        }
      : {}),
  });
  for (const entry of layouts) {
    if (!entry.groupPath?.length) {
      root.push(layoutLeaf(entry));
      continue;
    }
    let siblings = root;
    let pathKey = "";
    for (let index = 0; index < entry.groupPath.length; index += 1) {
      const segment = entry.groupPath[index]!;
      pathKey = pathKey ? `${pathKey}/${segment}` : segment;
      let folder = folderByKey.get(pathKey);
      if (!folder) {
        folder = { id: `framework.display.layout.group.${pathKey}`, label: segment, defaultOpen: false, items: [] };
        folderByKey.set(pathKey, folder);
        siblings.push(folder);
      }
      const folderItems = folder.items ?? (folder.items = []);
      if (index === entry.groupPath.length - 1) folder.items = [...folderItems, layoutLeaf(entry)];
      else siblings = folderItems;
    }
  }
  return root;
}

function buildDisplayWindowsTree(host: DisplayHostApi): TreePanelConfig {
  return {
    dragAndDropController: windowTemplatePaletteTreeDragController(),
    sections: host.windowKinds.length
      ? host.windowKinds.map((kind) => ({
          id: `framework.display.windows.${kind.id}`,
          label: kind.label,
          defaultOpen: false,
          items: [
            {
              id: `framework.display.windows.${kind.id}.kind`,
              label: kind.label,
              dragData: {
                [COMPOSE_WINDOW_TEMPLATE_MIME]: JSON.stringify({ windowKindId: kind.id }),
              },
            },
          ],
        }))
      : [{ id: "framework.display.windows.empty", items: [{ id: "empty", label: "—" }] }],
  };
}

function buildDisplayLayoutTree(host: DisplayHostApi): TreePanelConfig {
  const builtinLayouts = host.namedLayouts.filter((entry) => entry.origin === "builtin");
  const userLayouts = host.userLayouts;
  const builtinItems = groupNamedLayoutsToTreeItems(builtinLayouts, (layoutId) => host.applyNamedLayout(layoutId));
  const userItems = userLayouts.length
    ? [
        {
          id: "framework.display.layout.group.saved",
          label: shellLabel("ui.display.saved"),
          defaultOpen: false,
          items: groupNamedLayoutsToTreeItems(
            userLayouts,
            (layoutId) => host.applyNamedLayout(layoutId),
            (layoutId) => host.deleteUserLayout(layoutId),
          ),
        },
      ]
    : [];
  return {
    sections: [
      {
        id: "framework.display.layout.save",
        label: shellLabel("ui.display.saveLayout"),
        defaultOpen: false,
        items: [
          {
            id: "framework.display.layout.save.label",
            label: shellLabel("ui.common.name"),
            control: <Input id="framework.display.save-label" value={host.layoutSaveLabel} onChange={(event) => host.setLayoutSaveLabel(event.target.value)} placeholder={shellLabel("ui.display.saveLayoutPlaceholder")} />,
          },
          {
            id: "framework.display.layout.save.action",
            label: shellLabel("ui.common.save"),
            control: (
              <Button
                id="framework.display.save"
                size="sm"
                text={shellLabel("ui.display.saveCurrentLayout")}
                disabled={!host.layoutSaveLabel.trim()}
                onClick={() => {
                  const label = host.layoutSaveLabel.trim();
                  if (!label) return;
                  host.saveCurrentLayout(label);
                  host.setLayoutSaveLabel("");
                }}
              />
            ),
          },
        ],
      },
      {
        id: "framework.display.layout.list",
        label: shellLabel("ui.display.layouts"),
        defaultOpen: true,
        items: [...builtinItems, ...userItems],
      },
    ],
  };
}

export function createFrameworkDisplayPanelTabs(getHost: () => DisplayHostApi | null): PanelTabNode[] {
  return [
    singleTreeLeaf({
      id: FRAMEWORK_DISPLAY_WINDOWS_TAB_ID,
      icon: shellTabIcon("framework.display.windows"),
      name: shellLabel("ui.display.tab.windows"),
      order: -100,
      tree: {
        resolveTree: () => {
          const host = getHost();
          return host ? buildDisplayWindowsTree(host) : { sections: [{ id: "unavailable", items: [{ id: "unavailable", label: shellLabel("ui.display.unavailable") }] }] };
        },
      },
    }),
    singleTreeLeaf({
      id: FRAMEWORK_DISPLAY_LAYOUT_TAB_ID,
      icon: shellTabIcon("framework.display.layout"),
      name: shellLabel("ui.display.tab.layout"),
      order: -99,
      tree: {
        resolveTree: () => {
          const host = getHost();
          return host ? buildDisplayLayoutTree(host) : { sections: [{ id: "unavailable", items: [{ id: "unavailable", label: shellLabel("ui.display.unavailable") }] }] };
        },
      },
    }),
  ];
}
//#endregion DisplayPanel

//#region SettingsPanel
export type SettingsHostApi = {
  readonly appId?: string;
  readonly appLabel?: string;
  readonly controllerId?: string;
  readonly pluginId?: string;
  readonly compact: boolean;
  readonly setCompact: (compact: boolean) => void;
  readonly expertise: string;
  readonly setExpertise: (expertise: string) => void;
  readonly appearance: string;
  readonly setAppearance: (appearance: string) => void;
  readonly layout: UiChromeLayout;
  readonly setLayout: (layout: UiChromeLayout) => void;
  readonly mobileActive: boolean;
  /** 🧭 Clears the persisted corner-panel arrangement and folds every corner's active path back to its default — undefined when a shell doesn't wire up dock persistence. */
  readonly onResetDock?: () => void;
  readonly locale: UiLocale;
  readonly setLocale: (locale: UiLocale) => void;
  readonly terminology: string;
  readonly setTerminology: (id: string) => void;
  readonly terminologies: readonly string[];
  readonly theme: UiTheme;
  readonly themeId: string;
  readonly themeDirty: boolean;
  readonly themes: readonly UiTheme[];
  readonly setThemeId: (id: string) => void;
  readonly setThemeColor: (key: string, hex: string) => void;
  readonly setThemeSpacing: (key: string, value: string) => void;
  readonly setThemeFontStack: (key: string, value: string) => void;
  readonly setThemeStroke: (key: string, value: number | number[]) => void;
  readonly setThemeRadius: (key: string, value: number) => void;
  readonly setThemeOpacity: (key: string, value: number) => void;
  readonly setThemeMetric: (section: string, key: string, value: number | number[]) => void;
  readonly setThemeAppearancePaint: (appearance: ThemeAppearanceName, group: ThemePaletteGroup, key: string, hex: string, alpha?: number) => void;
  readonly saveTheme: (label: string) => void;
  readonly deleteTheme: (id: string) => void;
  readonly resetTheme: () => void;
  readonly exportTheme: () => void;
  readonly importTheme: () => void;
  readonly themeSaveLabel: string;
  readonly setThemeSaveLabel: (value: string) => void;
  readonly locks: ResolvedShellLocks;
};

function buildSettingsGeneralTree(host: SettingsHostApi): TreePanelConfig {
  return {
    sections: [
      ...(host.appId || host.appLabel || host.controllerId || host.pluginId
        ? [
            {
              id: "framework.settings.app",
              label: shellLabel("ui.settings.tab.app"),
              defaultOpen: true,
              items: [
                ...(host.appLabel ? [{ id: "framework.settings.app.label", label: `${shellLabel("ui.settings.app.name")}: ${host.appLabel}` }] : []),
                ...(host.appId ? [{ id: "framework.settings.app.id", label: `${shellLabel("ui.settings.app.id")}: ${host.appId}` }] : []),
                ...(host.controllerId ? [{ id: "framework.settings.app.controller", label: `${shellLabel("ui.settings.app.controller")}: ${host.controllerId}` }] : []),
                ...(host.pluginId ? [{ id: "framework.settings.app.plugin", label: `${shellLabel("ui.settings.app.plugin")}: ${host.pluginId}` }] : []),
              ],
            },
          ]
        : []),
      {
        id: "framework.settings.general",
        label: shellLabel("ui.settings.tab.general"),
        defaultOpen: true,
        items: [
          ...(host.locks.appearance
            ? []
            : [
                {
                  id: "framework.settings.appearance",
                  label: shellLabel("ui.settings.tab.appearance"),
                  control: (
                    <Select value={host.appearance} onValueChange={(value) => host.setAppearance(value)}>
                      <SelectTrigger id="framework.settings.appearance" className="h-small w-32" size="sm">
                        <SelectValue />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="system">{shellLabel("ui.settings.appearance.system")}</SelectItem>
                        <SelectItem value="light">{shellLabel("ui.settings.appearance.light")}</SelectItem>
                        <SelectItem value="dark">{shellLabel("ui.settings.appearance.dark")}</SelectItem>
                      </SelectContent>
                    </Select>
                  ),
                },
              ]),
          {
            id: "framework.settings.layout",
            label: shellLabel("ui.settings.tab.layout"),
            control: host.mobileActive ? (
              <span className="text-sm text-muted-foreground">{shellLabel("settings.layout.mobile")}</span>
            ) : (
              <Select value={host.layout} onValueChange={(value) => host.setLayout(value === "tablet" ? "tablet" : "desktop")}>
                <SelectTrigger id="framework.settings.layout" className="h-small w-32" size="sm">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="desktop">{shellLabel("settings.layout.desktop")}</SelectItem>
                  <SelectItem value="tablet">{shellLabel("settings.layout.tablet")}</SelectItem>
                </SelectContent>
              </Select>
            ),
          },
          {
            id: "framework.settings.compact",
            label: shellLabel("settings.compact"),
            control: <input id="framework.settings.compact" type="checkbox" checked={host.compact} onChange={(event) => host.setCompact(event.target.checked)} />,
          },
          {
            id: "framework.settings.expertise",
            label: shellLabel("ui.settings.tab.expertise"),
            control: (
              <Select value={host.expertise} onValueChange={(value) => host.setExpertise(value)}>
                <SelectTrigger id="framework.settings.expertise" className="h-small w-32" size="sm">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="beginner">{shellLabel("settings.expertise.beginner")}</SelectItem>
                  <SelectItem value="normal">{shellLabel("settings.expertise.normal")}</SelectItem>
                  <SelectItem value="expert">{shellLabel("settings.expertise.expert")}</SelectItem>
                </SelectContent>
              </Select>
            ),
          },
          ...(host.locks.locale
            ? []
            : [
                {
                  id: "framework.settings.language",
                  label: shellLabel("ui.settings.tab.language"),
                  control: (
                    <Select value={host.locale} onValueChange={(value) => host.setLocale(value === "de" ? "de" : "en")}>
                      <SelectTrigger id="framework.settings.language" className="h-small w-32" size="sm">
                        <SelectValue />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="en">{shellLabel("ui.settings.language.en")}</SelectItem>
                        <SelectItem value="de">{shellLabel("ui.settings.language.de")}</SelectItem>
                      </SelectContent>
                    </Select>
                  ),
                },
              ]),
          ...(host.locks.terminology
            ? []
            : [
                {
                  id: "framework.settings.terminology",
                  label: shellLabel("ui.settings.tab.terminology"),
                  control: (
                    <Select value={host.terminology} onValueChange={(value) => host.setTerminology(value)}>
                      <SelectTrigger id="framework.settings.terminology" className="h-small w-32" size="sm">
                        <SelectValue />
                      </SelectTrigger>
                      <SelectContent>
                        {host.terminologies.map((id) => (
                          <SelectItem key={id} value={id}>
                            {shellTerminologyLabel(id)}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                  ),
                },
              ]),
          ...(host.onResetDock
            ? [
                {
                  id: "framework.settings.resetDock.action",
                  label: shellLabel("ui.settings.resetDock"),
                  control: <Button id="framework.settings.resetDock" size="sm" icon="rotate-ccw" text={shellLabel("ui.settings.resetDock")} onClick={() => host.onResetDock?.()} />,
                },
              ]
            : []),
        ],
      },
    ],
  };
}

function rgba8ToHex(rgba: readonly [number, number, number, number]): string {
  const [r, g, b] = rgba;
  return `#${r.toString(16).padStart(2, "0")}${g.toString(16).padStart(2, "0")}${b.toString(16).padStart(2, "0")}`;
}

function themeColorInputRow(id: string, label: string, hex: string, onChange: (hex: string) => void): TreeDataItem {
  return {
    id,
    label,
    control: <input id={id} type="color" className={cn(borderElementClass, "h-small w-16 shrink-0 rounded border bg-background")} value={hex} onChange={(event) => onChange(event.target.value)} />,
  };
}

function themeTextInputRow(id: string, label: string, value: string, onCommit: (value: string) => void): TreeDataItem {
  return {
    id,
    label,
    control: <Input id={id} defaultValue={value} onBlur={(event) => onCommit(event.target.value)} className="h-small w-32" />,
  };
}

function themeNumberInputRow(id: string, label: string, value: number | number[], onCommit: (value: number | number[]) => void): TreeDataItem {
  const text = Array.isArray(value) ? value.join(", ") : String(value);
  return {
    id,
    label,
    control: (
      <Input
        id={id}
        defaultValue={text}
        onBlur={(event) => {
          const raw = event.target.value.trim();
          if (raw.includes(",")) {
            const parts = raw
              .split(",")
              .map((part) => Number.parseFloat(part.trim()))
              .filter((n) => !Number.isNaN(n));
            if (parts.length) onCommit(parts);
            return;
          }
          const n = Number.parseFloat(raw);
          if (!Number.isNaN(n)) onCommit(n);
        }}
        className="h-small w-32"
      />
    ),
  };
}

const THEME_PALETTE_GROUP_LABEL_KEYS = {
  board: "ui.settings.theme.group.board",
  map: "ui.settings.theme.group.map",
  canvas: "ui.settings.theme.group.canvas",
  chrome: "ui.settings.theme.group.chrome",
} as const satisfies Record<ThemePaletteGroup, UiTranslationKey>;

function buildThemeAppearanceGroupItems(host: SettingsHostApi, appearance: ThemeAppearanceName, group: ThemePaletteGroup): TreeDataItem[] {
  const refs = host.theme.appearances[appearance][group];
  const resolved = resolveThemeAppearancePalettes(host.theme, appearance)[group];
  return Object.keys(refs)
    .sort()
    .map((paintKey) => {
      const rgba = resolved[paintKey] ?? [0, 0, 0, 255];
      const hex = rgba8ToHex(rgba);
      const alpha = rgba[3] / 255;
      return {
        id: `framework.settings.theme.appearances.${appearance}.${group}.${paintKey}`,
        label: paintKey,
        control: (
          <div className="flex w-full items-center gap-single">
            <input type="color" className={cn(borderElementClass, "h-small w-10 shrink-0 rounded border bg-background")} value={hex} onChange={(event) => host.setThemeAppearancePaint(appearance, group, paintKey, event.target.value, alpha)} />
            <Input
              id={`framework.settings.theme.appearances.${appearance}.${group}.${paintKey}.alpha`}
              defaultValue={alpha.toFixed(2)}
              onBlur={(event) => {
                const nextAlpha = Number.parseFloat(event.target.value);
                if (!Number.isNaN(nextAlpha)) host.setThemeAppearancePaint(appearance, group, paintKey, hex, Math.min(1, Math.max(0, nextAlpha)));
              }}
              className="h-small w-14 shrink-0"
            />
          </div>
        ),
      } satisfies TreeDataItem;
    });
}

function buildSettingsThemeTree(host: SettingsHostApi): TreePanelConfig {
  const colorItems = Object.keys(host.theme.colors)
    .sort()
    .map((key) => themeColorInputRow(`framework.settings.theme.colors.${key}`, key, host.theme.colors[key]!, (hex) => host.setThemeColor(key, hex)));

  const spacingItems = Object.keys(host.theme.spacing)
    .sort()
    .map((key) => themeTextInputRow(`framework.settings.theme.spacing.${key}`, key, host.theme.spacing[key]!, (value) => host.setThemeSpacing(key, value)));

  const fontItems = Object.keys(host.theme.fontStacks)
    .sort()
    .map((key) => themeTextInputRow(`framework.settings.theme.fonts.${key}`, key, host.theme.fontStacks[key]!, (value) => host.setThemeFontStack(key, value)));

  const strokeItems = Object.keys(host.theme.strokes)
    .sort()
    .map((key) => themeNumberInputRow(`framework.settings.theme.strokes.${key}`, key, host.theme.strokes[key]!, (value) => host.setThemeStroke(key, value)));

  const radiusItems = Object.keys(host.theme.radii)
    .sort()
    .map((key) => themeNumberInputRow(`framework.settings.theme.radii.${key}`, key, host.theme.radii[key]!, (value) => host.setThemeRadius(key, typeof value === "number" ? value : value[0]!)));

  const opacityItems = Object.keys(host.theme.opacities)
    .sort()
    .map((key) => themeNumberInputRow(`framework.settings.theme.opacities.${key}`, key, host.theme.opacities[key]!, (value) => host.setThemeOpacity(key, typeof value === "number" ? value : value[0]!)));

  const metricSections = Object.keys(host.theme.metrics)
    .sort()
    .map(
      (section): TreeDataItem => ({
        id: `framework.settings.theme.metrics.${section}`,
        label: section,
        defaultOpen: false,
        items: Object.keys(host.theme.metrics[section]!)
          .sort()
          .map((key) => themeNumberInputRow(`framework.settings.theme.metrics.${section}.${key}`, key, host.theme.metrics[section]![key]!, (value) => host.setThemeMetric(section, key, value))),
      }),
    );

  const appearanceGroups: readonly ThemePaletteGroup[] = ["board", "map", "canvas", "chrome"];
  const appearanceItems: TreeDataItem[] = (["light", "dark"] as const).map((appearance) => ({
    id: `framework.settings.theme.appearances.${appearance}`,
    label: shellLabel(appearance === "light" ? "ui.settings.theme.appearance.light" : "ui.settings.theme.appearance.dark"),
    defaultOpen: false,
    items: appearanceGroups.map((group) => ({
      id: `framework.settings.theme.appearances.${appearance}.${group}`,
      label: shellLabel(THEME_PALETTE_GROUP_LABEL_KEYS[group]),
      defaultOpen: false,
      items: buildThemeAppearanceGroupItems(host, appearance, group),
    })),
  }));

  return {
    sections: [
      {
        id: "framework.settings.theme.select",
        label: `${shellLabel("ui.settings.theme.select")}${host.themeDirty ? ` (${shellLabel("ui.settings.theme.dirty")})` : ""}`,
        defaultOpen: true,
        items: [
          {
            id: "framework.settings.theme.select.picker",
            label: shellLabel("ui.settings.theme.select"),
            control: (
              <Select value={host.themeId} onValueChange={(value) => host.setThemeId(value)}>
                <SelectTrigger id="framework.settings.theme.select" className="h-small w-32" size="sm">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  {host.themes.map((theme) => (
                    <SelectItem key={theme.id} value={theme.id}>
                      {theme.label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            ),
          },
          {
            id: "framework.settings.theme.save.label",
            label: shellLabel("ui.common.name"),
            control: (
              <Input id="framework.settings.theme.save-label" value={host.themeSaveLabel} onChange={(event) => host.setThemeSaveLabel(event.target.value)} placeholder={shellLabel("ui.settings.theme.savePlaceholder")} className="h-small w-32" />
            ),
          },
          {
            id: "framework.settings.theme.save.action",
            label: shellLabel("ui.settings.theme.save"),
            control: (
              <Button
                id="framework.settings.theme.save"
                size="sm"
                text={shellLabel("ui.settings.theme.save")}
                disabled={!host.themeSaveLabel.trim()}
                onClick={() => {
                  const label = host.themeSaveLabel.trim();
                  if (!label) return;
                  host.saveTheme(label);
                  host.setThemeSaveLabel("");
                }}
              />
            ),
          },
          {
            id: "framework.settings.theme.reset.action",
            label: shellLabel("ui.settings.theme.reset"),
            control: <Button id="framework.settings.theme.reset" size="sm" text={shellLabel("ui.settings.theme.reset")} disabled={!host.themeDirty && host.themeId === "semio"} onClick={() => host.resetTheme()} />,
          },
          {
            id: "framework.settings.theme.export.action",
            label: shellLabel("ui.settings.theme.export"),
            control: <Button id="framework.settings.theme.export" size="sm" text={shellLabel("ui.settings.theme.export")} onClick={() => host.exportTheme()} />,
          },
          {
            id: "framework.settings.theme.import.action",
            label: shellLabel("ui.settings.theme.import"),
            control: <Button id="framework.settings.theme.import" size="sm" text={shellLabel("ui.settings.theme.import")} onClick={() => host.importTheme()} />,
          },
          ...(host.themeId.startsWith("custom.")
            ? [
                {
                  id: "framework.settings.theme.delete.action",
                  label: shellLabel("ui.settings.theme.delete"),
                  control: <Button id="framework.settings.theme.delete" size="sm" text={shellLabel("ui.settings.theme.delete")} onClick={() => host.deleteTheme(host.themeId)} />,
                },
              ]
            : []),
        ],
      },
      { id: "framework.settings.theme.colors", label: shellLabel("ui.settings.theme.colors"), defaultOpen: false, items: colorItems },
      { id: "framework.settings.theme.spacing", label: shellLabel("ui.settings.theme.spacing"), defaultOpen: false, items: spacingItems },
      { id: "framework.settings.theme.fonts", label: shellLabel("ui.settings.theme.fonts"), defaultOpen: false, items: fontItems },
      { id: "framework.settings.theme.strokes", label: shellLabel("ui.settings.theme.strokes"), defaultOpen: false, items: strokeItems },
      { id: "framework.settings.theme.radii", label: shellLabel("ui.settings.theme.radii"), defaultOpen: false, items: radiusItems },
      { id: "framework.settings.theme.opacities", label: shellLabel("ui.settings.theme.opacities"), defaultOpen: false, items: opacityItems },
      { id: "framework.settings.theme.metrics", label: shellLabel("ui.settings.theme.metrics"), defaultOpen: false, items: metricSections },
      { id: "framework.settings.theme.appearances", label: shellLabel("ui.settings.theme.appearances"), defaultOpen: false, items: appearanceItems },
    ],
  };
}

export function createFrameworkSettingsPanelTabs(getHost: () => SettingsHostApi | null): PanelTabNode[] {
  return [
    singleTreeLeaf({
      id: FRAMEWORK_SETTINGS_GENERAL_TAB_ID,
      icon: shellTabIcon("framework.settings.general"),
      name: shellLabel("ui.panelToggle.settings"),
      order: -98,
      tree: {
        resolveTree: () => {
          const host = getHost();
          return host ? buildSettingsGeneralTree(host) : { sections: [{ id: "unavailable", items: [{ id: "unavailable", label: shellLabel("ui.settings.unavailable") }] }] };
        },
      },
    }),
    // 🔒 A locked theme means no theme editing/saving either — drop the whole tab (the footer's chrome tab
    // bar renders `settingsRightTabs` directly, so its toggle disappears for free).
    ...(getHost()?.locks.themeId
      ? []
      : [
          singleTreeLeaf({
            id: FRAMEWORK_SETTINGS_THEME_TAB_ID,
            icon: shellTabIcon("paintbrush"),
            name: shellLabel("ui.settings.tab.theme"),
            order: -97,
            tree: {
              resolveTree: () => {
                const host = getHost();
                return host ? buildSettingsThemeTree(host) : { sections: [{ id: "unavailable", items: [{ id: "unavailable", label: shellLabel("ui.settings.unavailable") }] }] };
              },
            },
          }),
        ]),
  ];
}

export function useNamedLayoutHost(options: {
  readonly appId: string;
  readonly windowKinds: readonly { readonly id: string; readonly label: string }[];
  readonly builtinLayouts: readonly NamedLayout[];
  readonly currentLayout: WindowLayout | undefined;
  readonly onApplyLayout: (layout: WindowLayout) => void;
  readonly namedLayoutStore: { getSnapshot: () => readonly NamedLayout[]; save: (layout: NamedLayout) => void; remove: (layoutId: string) => void; subscribe: (listener: () => void) => () => void };
}): DisplayHostApi {
  const userLayouts = useSyncExternalStore(
    (listener) => options.namedLayoutStore.subscribe(listener),
    () => options.namedLayoutStore.getSnapshot(),
    () => options.namedLayoutStore.getSnapshot(),
  );
  const [layoutSaveLabel, setLayoutSaveLabel] = useState("");
  return useMemo(
    (): DisplayHostApi => ({
      windowKinds: options.windowKinds,
      namedLayouts: options.builtinLayouts,
      userLayouts,
      saveCurrentLayout: (label) => {
        if (!options.currentLayout) return;
        const id = `user-${Date.now()}`;
        options.namedLayoutStore.save(createNamedLayout(id, label, options.currentLayout, "user"));
      },
      applyNamedLayout: (layoutId) => {
        const layout = [...options.builtinLayouts, ...userLayouts].find((entry) => entry.id === layoutId);
        if (layout) options.onApplyLayout(layout.layout);
      },
      deleteUserLayout: (layoutId) => options.namedLayoutStore.remove(layoutId),
      layoutSaveLabel,
      setLayoutSaveLabel,
    }),
    [options, userLayouts, layoutSaveLabel],
  );
}
//#endregion SettingsPanel
//#endregion 🔖os-chrome-panels
//#endregion 🔖OsShell

//#region 🔖WorldTerrainLayerHost
//#region TerrainStyle
export type WorldTerrainStyle = {
  readonly tileUrlTemplate: string;
  readonly projectOriginLon: number;
  readonly projectOriginLat: number;
  readonly exaggeration: number;
  readonly colorRamp: string;
  readonly minZoom: number;
  readonly maxZoom: number;
};

export function parseWorldTerrainStyle(json: string | undefined): WorldTerrainStyle | null {
  if (!json) return null;
  try {
    const parsed = JSON.parse(json) as Partial<WorldTerrainStyle>;
    if (typeof parsed.tileUrlTemplate !== "string") return null;
    return {
      tileUrlTemplate: parsed.tileUrlTemplate,
      projectOriginLon: parsed.projectOriginLon ?? 0,
      projectOriginLat: parsed.projectOriginLat ?? 0,
      exaggeration: parsed.exaggeration ?? 1,
      colorRamp: parsed.colorRamp ?? "hypsometric",
      minZoom: parsed.minZoom ?? 6,
      maxZoom: parsed.maxZoom ?? 14,
    };
  } catch {
    return null;
  }
}
//#endregion TerrainStyle

//#region TerrainMesh
type TerrainTileMeshPayload = {
  readonly positions: number[];
  readonly normals: number[];
  readonly indices: number[];
  readonly uvs: number[];
};

function geometryFromTerrainMesh(mesh: TerrainTileMeshPayload): BufferGeometry {
  const geometry = new BufferGeometry();
  geometry.setAttribute("position", new BufferAttribute(new Float32Array(mesh.positions), 3));
  geometry.setAttribute("normal", new BufferAttribute(new Float32Array(mesh.normals), 3));
  geometry.setAttribute("uv", new BufferAttribute(new Float32Array(mesh.uvs), 2));
  geometry.setIndex(mesh.indices);
  return geometry;
}

/** 🎨 A vertical hypsometric ramp (low -> green/tan, high -> grey rock, peak -> white) sampled by
 * each terrain vertex's normalized-elevation `uv.y` — generated once client-side rather than
 * round-tripped through Rust, since it's a pure display convenience. */
let hypsometricTexture: CanvasTexture | null = null;
function getHypsometricTexture(): CanvasTexture {
  if (hypsometricTexture) return hypsometricTexture;
  const canvas = document.createElement("canvas");
  canvas.width = 2;
  canvas.height = 256;
  const ctx = canvas.getContext("2d");
  if (ctx) {
    const gradient = ctx.createLinearGradient(0, canvas.height, 0, 0);
    gradient.addColorStop(0, "#4b6b3a");
    gradient.addColorStop(0.5, "#a68a5b");
    gradient.addColorStop(0.85, "#8f8f8f");
    gradient.addColorStop(1, "#ffffff");
    ctx.fillStyle = gradient;
    ctx.fillRect(0, 0, canvas.width, canvas.height);
  }
  hypsometricTexture = new CanvasTexture(canvas);
  hypsometricTexture.wrapS = ClampToEdgeWrapping;
  hypsometricTexture.wrapT = ClampToEdgeWrapping;
  hypsometricTexture.needsUpdate = true;
  return hypsometricTexture;
}
//#endregion TerrainMesh

//#region TerrainTileRenderer
const TERRAIN_TILE_REFRESH_DEBOUNCE_MS = 150;
const MAX_CONCURRENT_TERRAIN_TILE_FETCHES = 8;

type TerrainTileRow = { readonly z: number; readonly x: number; readonly y: number; readonly key: string };

function parseVisibleTerrainTilesJson(raw: string): TerrainTileRow[] {
  try {
    const rows = JSON.parse(raw) as TerrainTileRow[];
    return Array.isArray(rows) ? rows : [];
  } catch {
    return [];
  }
}

/** 🧵 Owns a `TerrainSession`, fetches/uploads/evicts DEM tiles as the camera moves, and reports
 * the current set of tile geometries back to React — the 3D analog of `tiled-map-host.tsx`'s
 * `MapRenderer`, except it hands back mesh buffers for three.js instead of driving a canvas. */
class TerrainTileRenderer {
  private disposed = false;
  private session: TerrainWasmSession | null = null;
  private readonly tileMiss = new Set<string>();
  private refreshTimer: ReturnType<typeof setTimeout> | null = null;
  private refreshInFlight: Promise<void> | null = null;
  private readonly geometries = new Map<string, BufferGeometry>();

  constructor(
    private readonly style: WorldTerrainStyle,
    private readonly onGeometriesChanged: (geometries: Map<string, BufferGeometry>) => void,
  ) {}

  async init(): Promise<void> {
    const session = await createTerrainSession();
    if (this.disposed) return;
    session.set_project_origin(this.style.projectOriginLon, this.style.projectOriginLat);
    session.set_exaggeration(this.style.exaggeration);
    this.session = session;
  }

  scheduleRefresh(cameraJson: string): void {
    if (this.disposed) return;
    if (this.refreshTimer !== null) clearTimeout(this.refreshTimer);
    this.refreshTimer = setTimeout(() => {
      this.refreshTimer = null;
      void this.refresh(cameraJson);
    }, TERRAIN_TILE_REFRESH_DEBOUNCE_MS);
  }

  private async refresh(cameraJson: string): Promise<void> {
    if (this.disposed || !this.session) return;
    if (this.refreshInFlight) await this.refreshInFlight;
    if (this.disposed) return;
    this.refreshInFlight = this.doRefresh(cameraJson).finally(() => {
      this.refreshInFlight = null;
    });
    return this.refreshInFlight;
  }

  private async doRefresh(cameraJson: string): Promise<void> {
    const session = this.session;
    if (!session) return;
    const rows = parseVisibleTerrainTilesJson(session.visible_terrain_tiles_json(cameraJson));
    const visibleKeys = new Set(rows.map((row) => row.key));
    for (const key of [...this.geometries.keys()]) {
      if (visibleKeys.has(key)) continue;
      this.geometries.get(key)?.dispose();
      this.geometries.delete(key);
      const [z, x, y] = key.split("/").map(Number);
      if (z !== undefined && x !== undefined && y !== undefined) session.evict_terrain_tile(z, x, y);
    }
    const missing = rows.filter((row) => !this.geometries.has(row.key) && !this.tileMiss.has(row.key));
    const uploadOne = async (row: TerrainTileRow): Promise<void> => {
      const url = this.style.tileUrlTemplate.replace("{z}", String(row.z)).replace("{x}", String(row.x)).replace("{y}", String(row.y));
      let response: Response;
      try {
        response = await fetch(url);
      } catch {
        this.tileMiss.add(row.key);
        return;
      }
      if (!response.ok) {
        this.tileMiss.add(row.key);
        return;
      }
      const bytes = new Uint8Array(await response.arrayBuffer());
      if (this.disposed) return;
      if (!session.upload_elevation_tile(row.z, row.x, row.y, bytes)) {
        this.tileMiss.add(row.key);
        return;
      }
      const meshJson = session.terrain_tile_mesh_json(row.z, row.x, row.y);
      if (meshJson === "null" || this.disposed) return;
      const mesh = JSON.parse(meshJson) as TerrainTileMeshPayload;
      this.geometries.set(row.key, geometryFromTerrainMesh(mesh));
    };
    for (let i = 0; i < missing.length; i += MAX_CONCURRENT_TERRAIN_TILE_FETCHES) {
      await Promise.all(missing.slice(i, i + MAX_CONCURRENT_TERRAIN_TILE_FETCHES).map((row) => uploadOne(row)));
    }
    if (!this.disposed) this.onGeometriesChanged(new Map(this.geometries));
  }

  dispose(): void {
    this.disposed = true;
    if (this.refreshTimer !== null) clearTimeout(this.refreshTimer);
    for (const geometry of this.geometries.values()) geometry.dispose();
    this.geometries.clear();
  }
}
//#endregion TerrainTileRenderer

//#region WorldTerrainLayer
/** ⛰️ Renders GIS 3D terrain as chunked DEM-tile meshes inside the shared `World3d` viewport —
 * mounted alongside `WorldInstancesLayer` when `scene.terrainJson` is present. */
export function WorldTerrainLayer({ terrainJson, cameraPosition, cameraTarget }: { readonly terrainJson: string | undefined; readonly cameraPosition: readonly [number, number, number]; readonly cameraTarget: readonly [number, number, number] }) {
  const style = useMemo(() => parseWorldTerrainStyle(terrainJson), [terrainJson]);
  const rendererRef = useRef<TerrainTileRenderer | null>(null);
  const [geometries, setGeometries] = useState<Map<string, BufferGeometry>>(new Map());
  const material = useMemo(() => new MeshStandardMaterial({ map: getHypsometricTexture(), side: DoubleSide, roughness: 1, metalness: 0 }), []);

  useEffect(() => {
    if (!style) {
      rendererRef.current?.dispose();
      rendererRef.current = null;
      setGeometries(new Map());
      return undefined;
    }
    const renderer = new TerrainTileRenderer(style, setGeometries);
    rendererRef.current = renderer;
    void renderer.init().then(() => {
      if (rendererRef.current === renderer) renderer.scheduleRefresh(JSON.stringify({ position: cameraPosition, target: cameraTarget }));
    });
    return () => {
      renderer.dispose();
      if (rendererRef.current === renderer) rendererRef.current = null;
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps -- camera changes are handled by the effect below; this one only (re)creates the session when the terrain source itself changes.
  }, [style?.tileUrlTemplate, style?.projectOriginLon, style?.projectOriginLat, style?.exaggeration]);

  useEffect(() => {
    if (!style) return;
    rendererRef.current?.scheduleRefresh(JSON.stringify({ position: cameraPosition, target: cameraTarget }));
  }, [style, cameraPosition, cameraTarget]);

  if (!style) return null;

  return (
    <group>
      {[...geometries.entries()].map(([key, geometry]) => (
        <mesh key={key} geometry={geometry} material={material} receiveShadow />
      ))}
    </group>
  );
}
//#endregion WorldTerrainLayer
//#endregion 🔖WorldTerrainLayerHost

//#region 🔖Canvas2dHost
//#region CanvasCameraMath
export const CANVAS_CAMERA_ZOOM_MIN = 0.05;
export const CANVAS_CAMERA_ZOOM_MAX = 32;
const WHEEL_ZOOM_IN_FACTOR = 1.1;
const WHEEL_ZOOM_OUT_FACTOR = 0.9;

export type CanvasCamera = {
  x: number;
  y: number;
  zoom: number;
};

export function clampCanvasZoom(zoom: number): number {
  return Math.min(CANVAS_CAMERA_ZOOM_MAX, Math.max(CANVAS_CAMERA_ZOOM_MIN, zoom));
}

/** 🧭 Maps world coordinates to logical (CSS-pixel) screen space — matches `infinite_cavas::camera::world_to_screen`. */
export function worldToScreenLogical(worldX: number, worldY: number, camera: CanvasCamera, viewportWidth: number, viewportHeight: number): { readonly x: number; readonly y: number } {
  const zoom = camera.zoom || 1;
  return {
    x: (worldX - camera.x) * zoom + viewportWidth * 0.5,
    y: (worldY - camera.y) * zoom + viewportHeight * 0.5,
  };
}

/** 🧭 Maps logical (CSS-pixel) screen space to world coordinates — matches `infinite_cavas::camera::screen_to_world`. */
export function screenToWorldLogical(screenX: number, screenY: number, camera: CanvasCamera, viewportWidth: number, viewportHeight: number): { readonly x: number; readonly y: number } {
  const zoom = camera.zoom || 1;
  return {
    x: (screenX - viewportWidth * 0.5) / zoom + camera.x,
    y: (screenY - viewportHeight * 0.5) / zoom + camera.y,
  };
}

/** 🔍 Cursor-anchored wheel zoom — matches `infinite_cavas::camera::wheel_screen`. */
export function wheelCameraAtScreen(camera: CanvasCamera, screenX: number, screenY: number, deltaY: number, viewportWidth: number, viewportHeight: number): CanvasCamera {
  const zoomFactor = deltaY < 0 ? WHEEL_ZOOM_IN_FACTOR : WHEEL_ZOOM_OUT_FACTOR;
  const nextZoom = clampCanvasZoom((camera.zoom || 1) * zoomFactor);
  const worldBefore = screenToWorldLogical(screenX, screenY, camera, viewportWidth, viewportHeight);
  return {
    x: worldBefore.x - (screenX - viewportWidth * 0.5) / nextZoom,
    y: worldBefore.y - (screenY - viewportHeight * 0.5) / nextZoom,
    zoom: nextZoom,
  };
}
//#endregion CanvasCameraMath

//#region JsonLayersCanvasSession
type CanvasGradientStop = { readonly offset?: number; readonly color?: readonly number[] };

type CanvasLayerRecord = {
  readonly id?: string;
  readonly kind?: string;
  readonly role?: string;
  readonly utility?: string;
  readonly name?: string;
  readonly color?: string;
  readonly selected?: boolean;
  readonly x?: number;
  readonly y?: number;
  readonly width?: number;
  readonly height?: number;
  readonly x0?: number;
  readonly y0?: number;
  readonly y1?: number;
  readonly x1?: number;
  readonly dataUrl?: string;
  readonly points?: readonly (readonly [number, number])[];
  readonly seams?: readonly number[];
  readonly base?: { readonly name?: string; readonly x?: number; readonly y?: number; readonly width?: number; readonly height?: number };
  readonly transform?: readonly number[];
  readonly segments?: readonly {
    readonly kind?: string;
    readonly to?: readonly [number, number];
    readonly ctrl?: readonly [number, number];
    readonly ctrl1?: readonly [number, number];
    readonly ctrl2?: readonly [number, number];
    readonly rx?: number;
    readonly ry?: number;
    readonly rotation?: number;
    readonly largeArc?: boolean;
    readonly sweep?: boolean;
  }[];
  readonly fill?: {
    readonly kind?: string;
    readonly color?: readonly number[];
    readonly x1?: number;
    readonly y1?: number;
    readonly x2?: number;
    readonly y2?: number;
    readonly cx?: number;
    readonly cy?: number;
    readonly r?: number;
    readonly stops?: readonly CanvasGradientStop[];
  };
  readonly stroke?: { readonly color?: readonly number[]; readonly width?: number; readonly dash?: readonly number[]; readonly cap?: string; readonly join?: string };
  readonly opacity?: number;
  readonly blendMode?: string;
  readonly fillRule?: string;
  readonly visible?: boolean;
  readonly text?: { readonly content?: string; readonly size?: number };
  readonly image?: { readonly src?: string; readonly width?: number; readonly height?: number };
};

function rgbaToCss(color: readonly number[] | undefined, opacity = 1): string {
  if (!color || color.length < 3) return `rgba(148, 163, 184, ${opacity})`;
  const alpha = (color[3] ?? 1) * opacity;
  return `rgba(${color[0]! * 255}, ${color[1]! * 255}, ${color[2]! * 255}, ${alpha})`;
}

/** 🎨 Maps a `draw.document` blend mode to its `GlobalCompositeOperation` equivalent (16 modes, matches `DRAW_BLEND_MODES`). */
const BLEND_MODE_TO_COMPOSITE: Readonly<Record<string, GlobalCompositeOperation>> = {
  normal: "source-over",
  multiply: "multiply",
  screen: "screen",
  overlay: "overlay",
  darken: "darken",
  lighten: "lighten",
  colorDodge: "color-dodge",
  colorBurn: "color-burn",
  hardLight: "hard-light",
  softLight: "soft-light",
  difference: "difference",
  exclusion: "exclusion",
  hue: "hue",
  saturation: "saturation",
  color: "color",
  luminosity: "luminosity",
};

function blendModeToComposite(mode: string | undefined): GlobalCompositeOperation {
  return BLEND_MODE_TO_COMPOSITE[mode ?? "normal"] ?? "source-over";
}

/** 🪣 Resolves a fill record into a canvas paint — solid color or gradient (linear/radial, in local layer coordinates). */
function fillStyleToPaint(ctx: CanvasRenderingContext2D, fill: CanvasLayerRecord["fill"], opacity: number): string | CanvasGradient | null {
  if (!fill) return null;
  if (fill.kind === "linearGradient" && fill.stops?.length) {
    const gradient = ctx.createLinearGradient(fill.x1 ?? 0, fill.y1 ?? 0, fill.x2 ?? 0, fill.y2 ?? 0);
    for (const stop of fill.stops) gradient.addColorStop(Math.min(1, Math.max(0, stop.offset ?? 0)), rgbaToCss(stop.color, opacity));
    return gradient;
  }
  if (fill.kind === "radialGradient" && fill.stops?.length) {
    const gradient = ctx.createRadialGradient(fill.cx ?? 0, fill.cy ?? 0, 0, fill.cx ?? 0, fill.cy ?? 0, Math.max(fill.r ?? 0, 0));
    for (const stop of fill.stops) gradient.addColorStop(Math.min(1, Math.max(0, stop.offset ?? 0)), rgbaToCss(stop.color, opacity));
    return gradient;
  }
  if (fill.color) return rgbaToCss(fill.color, opacity);
  return null;
}

/** 🖊️ Builds a `Path2D` from the full (possibly multi-contour) segment list — evenodd fill handles holes correctly across contours. */
function buildScenePath(segments: CanvasLayerRecord["segments"]): Path2D | null {
  if (!segments?.length) return null;
  const path = new Path2D();
  for (const segment of segments) {
    const kind = segment.kind ?? "line";
    if (kind === "move" && segment.to) {
      path.moveTo(segment.to[0]!, segment.to[1]!);
    } else if (kind === "line" && segment.to) {
      path.lineTo(segment.to[0]!, segment.to[1]!);
    } else if (kind === "quad" && segment.ctrl && segment.to) {
      path.quadraticCurveTo(segment.ctrl[0]!, segment.ctrl[1]!, segment.to[0]!, segment.to[1]!);
    } else if (kind === "cubic" && segment.ctrl1 && segment.ctrl2 && segment.to) {
      path.bezierCurveTo(segment.ctrl1[0]!, segment.ctrl1[1]!, segment.ctrl2[0]!, segment.ctrl2[1]!, segment.to[0]!, segment.to[1]!);
    } else if (kind === "arc" && segment.to) {
      path.lineTo(segment.to[0]!, segment.to[1]!);
    } else if (kind === "close") {
      path.closePath();
    }
  }
  return path;
}

function layerColorCss(layer: CanvasLayerRecord, fallbackHue: number, opacity = 1): string {
  if (layer.color) {
    if (layer.color.startsWith("#") || layer.color.startsWith("hsl")) {
      if (layer.color.startsWith("hsl") && opacity < 1) {
        return layer.color.replace(")", ` / ${opacity})`).replace("hsl(", "hsla(");
      }
      return layer.color;
    }
  }
  return `hsla(${fallbackHue}, 70%, 55%, ${opacity})`;
}

function applySceneTransform(ctx: CanvasRenderingContext2D, transform: readonly number[] | undefined): void {
  if (!transform || transform.length < 6) return;
  const [a, b, c, d, e, f] = transform;
  ctx.transform(a ?? 1, b ?? 0, c ?? 0, d ?? 1, e ?? 0, f ?? 0);
}

function drawSceneNode(ctx: CanvasRenderingContext2D, layer: CanvasLayerRecord, zoom: number, imageCache: ReadonlyMap<string, HTMLImageElement>): void {
  if (layer.visible === false) return;
  const opacity = layer.opacity ?? 1;
  ctx.save();
  ctx.globalCompositeOperation = blendModeToComposite(layer.blendMode);
  applySceneTransform(ctx, layer.transform);
  const path = buildScenePath(layer.segments);
  if (path) {
    const fillRule = layer.fillRule === "nonzero" ? "nonzero" : "evenodd";
    const fillPaint = fillStyleToPaint(ctx, layer.fill, opacity);
    if (fillPaint) {
      ctx.fillStyle = fillPaint;
      ctx.fill(path, fillRule);
    }
    if (layer.stroke) {
      ctx.strokeStyle = rgbaToCss(layer.stroke.color, opacity);
      ctx.lineWidth = Math.max((layer.stroke.width ?? 1) / zoom, 1 / zoom);
      ctx.lineCap = (layer.stroke.cap as CanvasLineCap) ?? "butt";
      ctx.lineJoin = (layer.stroke.join as CanvasLineJoin) ?? "miter";
      ctx.setLineDash(layer.stroke.dash?.map((value) => value / zoom) ?? []);
      ctx.stroke(path);
      ctx.setLineDash([]);
    } else if (!fillPaint) {
      ctx.strokeStyle = rgbaToCss([0.58, 0.64, 0.72, 0.95], opacity);
      ctx.lineWidth = Math.max(1 / zoom, 1);
      ctx.stroke(path);
    }
  }
  if (layer.text?.content) {
    ctx.fillStyle = layer.fill?.color ? rgbaToCss(layer.fill.color, opacity) : rgbaToCss([0.89, 0.91, 0.94, 1], opacity);
    ctx.font = `${layer.text.size ?? 14}px ui-monospace, monospace`;
    ctx.fillText(layer.text.content, 0, layer.text.size ?? 14);
  }
  if (layer.image?.src) {
    const width = layer.image.width ?? layer.width ?? 64;
    const height = layer.image.height ?? layer.height ?? 64;
    const image = imageCache.get(layer.image.src);
    if (image?.complete) {
      ctx.globalAlpha = opacity;
      ctx.drawImage(image, 0, 0, width, height);
      ctx.globalAlpha = 1;
    }
  }
  ctx.restore();
}

function layerBounds(layer: CanvasLayerRecord): { readonly x: number; readonly y: number; readonly width: number; readonly height: number } | null {
  const x = layer.x ?? layer.base?.x;
  const y = layer.y ?? layer.base?.y;
  const width = layer.width ?? layer.base?.width;
  const height = layer.height ?? layer.base?.height;
  if (x == null || y == null || width == null || height == null) return null;
  return { x, y, width, height };
}

function layerLabel(layer: CanvasLayerRecord): string {
  return layer.name ?? layer.base?.name ?? layer.kind ?? layer.id ?? "layer";
}

function drawRoundedRect(ctx: CanvasRenderingContext2D, x: number, y: number, width: number, height: number, radius: number): void {
  const r = Math.min(radius, width * 0.5, height * 0.5);
  ctx.beginPath();
  ctx.moveTo(x + r, y);
  ctx.lineTo(x + width - r, y);
  ctx.quadraticCurveTo(x + width, y, x + width, y + r);
  ctx.lineTo(x + width, y + height - r);
  ctx.quadraticCurveTo(x + width, y + height, x + width - r, y + height);
  ctx.lineTo(x + r, y + height);
  ctx.quadraticCurveTo(x, y + height, x, y + height - r);
  ctx.lineTo(x, y + r);
  ctx.quadraticCurveTo(x, y, x + r, y);
  ctx.closePath();
}

function drawBoundsLayer(ctx: CanvasRenderingContext2D, layer: CanvasLayerRecord, bounds: { readonly x: number; readonly y: number; readonly width: number; readonly height: number }, label: string, hue: number, zoom: number): void {
  const isHandle = layer.role === "handle";
  const isSelected = layer.selected === true;
  const fillOpacity = isHandle ? 0.35 : isSelected ? 0.42 : 0.22;
  const strokeOpacity = isSelected ? 1 : isHandle ? 0.7 : 0.85;
  const fillColor = layerColorCss(layer, hue, fillOpacity);
  const strokeColor = isSelected ? "rgba(251, 191, 36, 0.95)" : layerColorCss(layer, hue, strokeOpacity);
  const lineWidth = Math.max((isSelected ? 2.5 : 1) / zoom, 1 / zoom);
  if (isSelected) {
    ctx.strokeStyle = "rgba(251, 191, 36, 0.28)";
    ctx.lineWidth = Math.max(5 / zoom, 2 / zoom);
    if (layer.kind === "circle") {
      const cx = bounds.x + bounds.width * 0.5;
      const cy = bounds.y + bounds.height * 0.5;
      const radius = Math.min(bounds.width, bounds.height) * 0.5 + 4 / zoom;
      ctx.beginPath();
      ctx.arc(cx, cy, radius, 0, Math.PI * 2);
      ctx.stroke();
    } else {
      drawRoundedRect(ctx, bounds.x - 4 / zoom, bounds.y - 4 / zoom, bounds.width + 8 / zoom, bounds.height + 8 / zoom, 6 / zoom);
      ctx.stroke();
    }
  }
  ctx.fillStyle = fillColor;
  ctx.strokeStyle = strokeColor;
  ctx.lineWidth = lineWidth;
  if (layer.kind === "circle") {
    const cx = bounds.x + bounds.width * 0.5;
    const cy = bounds.y + bounds.height * 0.5;
    const radius = Math.min(bounds.width, bounds.height) * 0.5;
    ctx.beginPath();
    ctx.arc(cx, cy, radius, 0, Math.PI * 2);
    ctx.fill();
    ctx.stroke();
  } else {
    drawRoundedRect(ctx, bounds.x, bounds.y, bounds.width, bounds.height, 4 / zoom);
    ctx.fill();
    ctx.stroke();
  }
  if (!isHandle && label) {
    ctx.fillStyle = "rgba(226, 232, 240, 0.92)";
    ctx.font = `${12 / zoom}px ui-monospace, monospace`;
    ctx.fillText(label, bounds.x + 4, bounds.y + 14 / zoom);
  }
}

function drawCheckerboard(ctx: CanvasRenderingContext2D, width: number, height: number, zoom: number): void {
  const cell = 16 / Math.max(zoom, 0.25);
  const cols = Math.ceil(width / cell) + 1;
  const rows = Math.ceil(height / cell) + 1;
  for (let row = 0; row < rows; row += 1) {
    for (let col = 0; col < cols; col += 1) {
      ctx.fillStyle = (row + col) % 2 === 0 ? "#2a2d34" : "#1f2228";
      ctx.fillRect(col * cell - width / 2, row * cell - height / 2, cell, cell);
    }
  }
}

class JsonLayersCanvasSession implements GraphWasmSession {
  private canvas: HTMLCanvasElement | null = null;
  private ctx: CanvasRenderingContext2D | null = null;
  private logicalWidth = 1;
  private logicalHeight = 1;
  private dpr = 1;
  private readonly imageCache = new Map<string, HTMLImageElement>();
  private panning = false;
  private panStart = { x: 0, y: 0 };
  private panCameraStart = { x: 0, y: 0 };
  private activeUtility = "selectDirect";

  constructor(
    private readonly layersJson: string,
    private camera: CanvasCamera,
    private readonly onCameraChange: (camera: CanvasCamera) => void,
    private readonly onPointer?: (action: string, args?: Record<string, unknown>) => void,
  ) {}

  async attachCanvas(canvas: HTMLCanvasElement, logicalW: number, logicalH: number, dpr: number): Promise<unknown> {
    this.canvas = canvas;
    this.ctx = canvas.getContext("2d");
    this.logicalWidth = logicalW;
    this.logicalHeight = logicalH;
    this.dpr = dpr;
    await this.preloadImages();
    this.renderFrame();
    return undefined;
  }

  setSize(width: number, height: number, dpr: number): void {
    this.logicalWidth = width;
    this.logicalHeight = height;
    this.dpr = dpr;
  }

  updateCamera(camera: CanvasCamera): void {
    this.camera = camera;
  }

  private parseLayers(): CanvasLayerRecord[] {
    try {
      return JSON.parse(this.layersJson) as CanvasLayerRecord[];
    } catch {
      return [];
    }
  }

  private async preloadImages(): Promise<void> {
    const layers = this.parseLayers();
    const urls = new Set<string>();
    for (const layer of layers) {
      if (layer.kind === "image" && layer.dataUrl) urls.add(layer.dataUrl);
      if (layer.image?.src) urls.add(layer.image.src);
    }
    await Promise.all(
      [...urls].map(async (key) => {
        if (this.imageCache.has(key)) return;
        const image = new Image();
        image.decoding = "async";
        image.src = key;
        await image.decode().catch(() => undefined);
        this.imageCache.set(key, image);
      }),
    );
  }

  renderFrame(): void {
    const ctx = this.ctx;
    const canvas = this.canvas;
    if (!ctx || !canvas) return;
    const deviceWidth = canvas.width;
    const deviceHeight = canvas.height;
    const logicalWidth = this.logicalWidth;
    const logicalHeight = this.logicalHeight;
    const dpr = this.dpr;
    const zoom = this.camera.zoom || 1;
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, deviceWidth, deviceHeight);
    ctx.fillStyle = "#111318";
    ctx.fillRect(0, 0, deviceWidth, deviceHeight);
    const records = this.parseLayers();
    const meta = records.find((record) => record.role === "meta");
    if (meta?.utility) this.activeUtility = meta.utility;
    const layers = records.filter((record) => record.role !== "meta");
    ctx.save();
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    ctx.translate(logicalWidth * 0.5 - this.camera.x * zoom, logicalHeight * 0.5 - this.camera.y * zoom);
    ctx.scale(zoom, zoom);
    drawCheckerboard(ctx, logicalWidth, logicalHeight, zoom);
    for (const [index, layer] of layers.entries()) {
      if (layer.segments?.length || layer.text || layer.image?.src) {
        drawSceneNode(ctx, layer, zoom, this.imageCache);
        continue;
      }
      if (layer.kind === "image" && layer.dataUrl) {
        const bounds = layerBounds(layer);
        const image = this.imageCache.get(layer.dataUrl);
        if (bounds && image && image.complete) {
          ctx.drawImage(image, bounds.x, bounds.y, bounds.width, bounds.height);
        }
        continue;
      }
      if (layer.kind === "polyline" && layer.points?.length) {
        const seams = layer.seams ?? [];
        for (let segment = 0; segment + 1 < layer.points.length; segment += 2) {
          const [x0, y0] = layer.points[segment]!;
          const [x1, y1] = layer.points[segment + 1]!;
          const seamIndex = segment / 2;
          ctx.strokeStyle = layerColorCss(layer, (index * 47) % 360, 0.95);
          ctx.lineWidth = Math.max(1 / zoom, 1);
          ctx.setLineDash(seams[seamIndex] ? [6 / zoom, 4 / zoom] : []);
          ctx.beginPath();
          ctx.moveTo(x0, y0);
          ctx.lineTo(x1, y1);
          ctx.stroke();
        }
        ctx.setLineDash([]);
        continue;
      }
      const bounds = layerBounds(layer);
      const label = layerLabel(layer);
      const hue = (index * 47) % 360;
      if (layer.kind === "line" || layer.x0 != null) {
        const x0 = layer.x0 ?? layer.x ?? 0;
        const y0 = layer.y0 ?? layer.y ?? 0;
        const x1 = layer.x1 ?? (layer.x ?? 0) + (layer.width ?? 0);
        const y1 = layer.y1 ?? (layer.y ?? 0) + (layer.height ?? 0);
        const isWire = layer.role === "wire";
        ctx.strokeStyle = layerColorCss(layer, hue, 0.9);
        ctx.lineWidth = Math.max((isWire ? 1.25 : 2) / zoom, 1 / zoom);
        ctx.setLineDash(isWire ? [6 / zoom, 4 / zoom] : []);
        ctx.beginPath();
        ctx.moveTo(x0, y0);
        ctx.lineTo(x1, y1);
        ctx.stroke();
        ctx.setLineDash([]);
        continue;
      }
      if (bounds) {
        drawBoundsLayer(ctx, layer, bounds, label, hue, zoom);
      } else {
        ctx.fillStyle = "rgba(226, 232, 240, 0.75)";
        ctx.font = `${12 / zoom}px ui-monospace, monospace`;
        ctx.fillText(label, -logicalWidth / 2 + 16, -logicalHeight / 2 + 20 + index * 18);
      }
    }
    if (layers.length === 0) {
      ctx.fillStyle = "rgba(148, 163, 184, 0.7)";
      ctx.font = `${12 / zoom}px ui-monospace, monospace`;
      ctx.fillText("Empty canvas", -36, 0);
    }
    ctx.restore();
  }

  pointerDown(x: number, y: number, button: number, _extend: boolean, modifiers?: CanvasInputModifiers): void {
    if (button === 1 || this.activeUtility === "transformMove") {
      this.panning = true;
      this.panStart = { x, y };
      this.panCameraStart = { x: this.camera.x, y: this.camera.y };
      return;
    }
    this.onPointer?.("canvasPointerDown", {
      x,
      y,
      button,
      shift: modifiers?.shift ?? false,
      ctrl: modifiers?.ctrl ?? false,
      meta: modifiers?.meta ?? false,
      alt: modifiers?.alt ?? false,
      width: this.logicalWidth,
      height: this.logicalHeight,
    });
  }

  pointerMove(x: number, y: number): void {
    if (this.panning) {
      const zoom = this.camera.zoom || 1;
      const next = {
        ...this.camera,
        x: this.panCameraStart.x - (x - this.panStart.x) / zoom,
        y: this.panCameraStart.y - (y - this.panStart.y) / zoom,
      };
      this.camera = next;
      this.onCameraChange(next);
      this.renderFrame();
      return;
    }
    this.onPointer?.("canvasPointerMove", {
      x,
      y,
      width: this.logicalWidth,
      height: this.logicalHeight,
    });
  }

  pointerUp(x: number, y: number, modifiers?: CanvasInputModifiers): void {
    if (this.panning) {
      this.panning = false;
      return;
    }
    this.onPointer?.("canvasPointerUp", {
      x,
      y,
      shift: modifiers?.shift ?? false,
      ctrl: modifiers?.ctrl ?? false,
      meta: modifiers?.meta ?? false,
      alt: modifiers?.alt ?? false,
      width: this.logicalWidth,
      height: this.logicalHeight,
    });
  }

  doubleClick(x: number, y: number): void {
    this.onPointer?.("canvasDoubleClick", { x, y, width: this.logicalWidth, height: this.logicalHeight });
  }

  wheel(x: number, y: number, deltaY: number): void {
    const next = wheelCameraAtScreen(this.camera, x, y, deltaY, this.logicalWidth, this.logicalHeight);
    this.camera = next;
    this.onCameraChange(next);
    this.renderFrame();
  }
}
//#endregion JsonLayersCanvasSession

//#region Canvas2dHost
const CAMERA_SYNC_DEBOUNCE_MS = 120;
const DRAG_OVER_THROTTLE_MS = 50;
const DRAG_OVER_THROTTLE_DISTANCE = 4;

export function Canvas2dHost({ node, onAction }: ComponentSceneHostProps) {
  const scene = node.canvas2d;
  const emptySceneLabel = useLabel("ui.host.emptyScene");
  const initialCamera = useMemo(() => ({ x: scene?.cameraX ?? 0, y: scene?.cameraY ?? 0, zoom: scene?.zoom ?? 1 }), [scene?.cameraX, scene?.cameraY, scene?.zoom]);
  const cameraRef = useRef<CanvasCamera>(initialCamera);
  cameraRef.current = initialCamera;
  const sessionRef = useRef<JsonLayersCanvasSession | null>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const cameraSyncTimeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const dragOverStateRef = useRef<{ x: number; y: number; time: number } | null>(null);
  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      onAction({
        controllerId: node.controllerId,
        action,
        args: { surfaceId: node.surfaceId, ...args },
      });
    },
    [node.controllerId, node.surfaceId, onAction],
  );
  const sessionFactory = useMemo(() => {
    return () => {
      const session = new JsonLayersCanvasSession(
        scene?.layersJson ?? "[]",
        cameraRef.current,
        (next) => {
          cameraRef.current = next;
          sessionRef.current?.updateCamera(next);
          if (cameraSyncTimeoutRef.current) clearTimeout(cameraSyncTimeoutRef.current);
          cameraSyncTimeoutRef.current = setTimeout(() => dispatch("setCamera", { camera: next }), CAMERA_SYNC_DEBOUNCE_MS);
        },
        (action, args) => {
          if (action === "canvasPointerDown" && args?.button === 0) {
            dispatch("paintStrokeBegin");
          }
          if (action === "canvasPointerUp") {
            dispatch("paintStrokeEnd");
          }
          dispatch(action, args);
        },
      );
      sessionRef.current = session;
      return session;
    };
  }, [dispatch, scene?.layersJson]);

  const handleDragOver = useCallback(
    (event: DragEvent<HTMLDivElement>) => {
      if (!event.dataTransfer.types.includes(CATALOGUE_DRAG_MIME)) return;
      event.preventDefault();
      const rect = containerRef.current?.getBoundingClientRect();
      if (!rect) return;
      const x = event.clientX - rect.left;
      const y = event.clientY - rect.top;
      const last = dragOverStateRef.current;
      const now = Date.now();
      if (last && now - last.time < DRAG_OVER_THROTTLE_MS && Math.abs(x - last.x) < DRAG_OVER_THROTTLE_DISTANCE && Math.abs(y - last.y) < DRAG_OVER_THROTTLE_DISTANCE) return;
      dragOverStateRef.current = { x, y, time: now };
      dispatch("canvasDragOver", { x, y, width: rect.width, height: rect.height, types: [...event.dataTransfer.types] });
    },
    [dispatch],
  );

  const handleDragLeave = useCallback(() => {
    dragOverStateRef.current = null;
    dispatch("canvasDragLeave");
  }, [dispatch]);

  const handleDrop = useCallback(
    (event: DragEvent<HTMLDivElement>) => {
      const raw = event.dataTransfer.getData(CATALOGUE_DRAG_MIME);
      if (!raw) return;
      event.preventDefault();
      dragOverStateRef.current = null;
      const rect = containerRef.current?.getBoundingClientRect();
      if (!rect) return;
      dispatch("canvasDrop", { x: event.clientX - rect.left, y: event.clientY - rect.top, width: rect.width, height: rect.height, dragData: raw });
    },
    [dispatch],
  );

  if (!scene) return <div className="semio-canvas-2d-empty">{emptySceneLabel}</div>;

  return (
    <div ref={containerRef} className="semio-canvas-2d-host h-full min-h-[24rem] w-full bg-canvas" data-controller-id={node.controllerId} data-surface-id={node.surfaceId} onDragOver={handleDragOver} onDragLeave={handleDragLeave} onDrop={handleDrop}>
      <GraphWasmCanvas className="h-full w-full" sessionFactory={sessionFactory} />
    </div>
  );
}
//#endregion Canvas2dHost
//#endregion 🔖Canvas2dHost

//#region 🔖World3dHost
//#region WorldSceneParsing
type WorldMeshData = {
  readonly positions: readonly number[];
  readonly normals: readonly number[];
  readonly indices: readonly number[];
  /** Per-vertex RGB (0..1, 3 floats per vertex) — e.g. FEM stress contours. Native wgpu renderer has no
   * per-vertex color pipeline yet, so this is a react-renderer-only capability for now. */
  readonly colors?: readonly number[];
  readonly uvs?: readonly number[];
  readonly faceIds?: readonly number[];
  readonly vertexIds?: readonly number[];
  readonly edgePositions?: readonly number[];
  readonly edgeIds?: readonly number[];
  readonly paintTextureBase64?: string;
};

type WorldCameraRecord = {
  readonly position?: readonly [number, number, number];
  readonly target?: readonly [number, number, number];
  readonly fov?: number;
  readonly x?: number;
  readonly y?: number;
  readonly z?: number;
};

type WorldMeshRecord = {
  readonly id: string;
  readonly data?: WorldMeshData;
  readonly url?: string;
};

type WorldInstanceRecord = {
  readonly id: string;
  readonly meshId?: string;
  readonly position?: readonly [number, number, number];
  readonly rotation?: readonly [number, number, number, number];
  readonly scale?: readonly [number, number, number];
  readonly x?: number;
  readonly y?: number;
  readonly z?: number;
  readonly selected?: boolean;
  readonly hovered?: boolean;
  /** 🎨 Compatible/suggested state (e.g. catalog-kind hover in puzzle) — resolves to the secondary "highlighted" mesh style. */
  readonly highlighted?: boolean;
  /** 🎨 Non-interactive/locked state — resolves to the muted "disabled" mesh style at reduced opacity. */
  readonly disabled?: boolean;
  readonly smoothShading?: boolean;
};

type WorldSelectionTargets = {
  readonly mesh?: boolean;
  readonly vertex?: boolean;
  readonly edge?: boolean;
  readonly face?: boolean;
};

type WorldHoverComponent = {
  readonly objectId?: string;
  readonly mode?: string;
  readonly id?: number;
};

type WorldContextMenuItem = {
  readonly id: string;
  readonly label: string;
  readonly action: string;
  readonly args?: Record<string, unknown>;
};

type WorldSelectionRecord = {
  readonly method?: SelectionMarqueeMethod;
  readonly ids?: readonly string[];
  readonly hoveredId?: string | null;
  readonly referenceSelectedId?: string;
  readonly granularity?: string;
  readonly selectionMode?: string;
  readonly activeObjectId?: string;
  readonly componentIds?: readonly number[];
  readonly targets?: WorldSelectionTargets;
  readonly transformMode?: string;
  readonly interactionMode?: "model" | "paint";
  readonly gumballTarget?: readonly [number, number, number];
  readonly gumballActive?: boolean;
  readonly hoveredComponent?: WorldHoverComponent;
  readonly showEdges?: boolean;
  readonly engagementSessionActive?: boolean;
  /** 🖱️➡️ When true and `targets.face` is set, dragging an already-selected face starts a push/pull gesture (`worldFaceDragEnd` on release) instead of the default marquee/orbit. */
  readonly faceDragActive?: boolean;
};

type WorldSuggestionCandidateRecord = {
  readonly index: number;
  readonly objectLabel: string;
  readonly vortexLabel: string;
};

type WorldSuggestionMenuRecord = {
  readonly open: boolean;
  readonly x: number;
  readonly y: number;
  readonly pending: boolean;
  readonly candidates: readonly WorldSuggestionCandidateRecord[];
};

type WorldFillBuildRecord = {
  readonly count: number;
  readonly maxCount: number;
  readonly done: boolean;
};

type WorldInteractionRecord = {
  readonly activeUtility?: string;
  readonly brushCandidateIndex?: number;
  readonly hoveredVortexFullId?: string;
  readonly fillEditTargetVolumes?: boolean;
  readonly voxelDims?: readonly [number, number, number];
  readonly gridFactor?: number;
  readonly suggestionMenu?: WorldSuggestionMenuRecord | null;
  readonly fillBuild?: WorldFillBuildRecord;
};

type WorldLodRecord = {
  readonly gridFactor?: number;
  readonly gridSnapEnabled?: boolean;
  readonly showLodGrid?: boolean;
  readonly automaticLod?: boolean;
  readonly depthVariableLod?: boolean;
  readonly manualLod?: number;
};

type WorldVortexRecord = {
  readonly fullId: string;
  readonly objectId?: string;
  readonly vortexKind?: string;
  readonly position: readonly [number, number, number];
  readonly direction?: readonly [number, number, number];
  readonly radius?: number;
  readonly color?: string;
  readonly selected?: boolean;
  readonly hovered?: boolean;
};

type WorldAttractionRecord = {
  readonly id: string;
  readonly from: readonly [number, number, number];
  readonly to: readonly [number, number, number];
  readonly color?: string;
};

type WorldTargetVolumeRecord = {
  readonly id: string;
  readonly origin: readonly [number, number, number];
  readonly orientation?: readonly [number, number, number, number];
  readonly scale?: readonly [number, number, number] | number;
  readonly color?: string;
};

type WorldReferenceRecord = {
  readonly id: string;
  readonly url: string;
  readonly origin: readonly [number, number, number];
  readonly widthWorld?: number;
  readonly locked?: boolean;
  readonly hidden?: boolean;
  readonly opacity?: number;
};

type WorldBrushPreviewRecord = {
  readonly targetVortexFullId?: string;
  readonly objectKindId?: string;
  readonly sourceVortexIndex?: number;
  readonly meshUrl?: string;
  readonly origin?: readonly [number, number, number];
  readonly orientation?: readonly [number, number, number, number];
  readonly scale?: readonly [number, number, number] | number;
};

/** ☁️ One point-cloud rendering layer (`World3dScene.pointsJson` entries) — the cheap path for
 * 10^5-10^6 points, distinct from per-point meshes. `positionsB64` is base64 of little-endian f32 xyz
 * interleaved; `colorsB64` (optional) is base64 of u8 rgb interleaved, one triplet per point. */
type WorldPointCloudLayerRecord = {
  readonly id: string;
  readonly positionsB64: string;
  readonly colorsB64?: string;
  readonly size: number;
  readonly sizeAttenuation: boolean;
};

type WorldEngagementPreviewPoint = {
  readonly kind: "point";
  readonly role?: string;
  readonly position: readonly [number, number, number];
};

type WorldEngagementPreviewSegment = {
  readonly kind: "segment";
  readonly role?: string;
  readonly from: readonly [number, number, number];
  readonly to: readonly [number, number, number];
};

type WorldEngagementPreviewBox = {
  readonly kind: "box-preview";
  readonly role?: string;
  readonly cornerA?: readonly [number, number, number];
  readonly cornerB?: readonly [number, number, number];
  readonly height?: number;
};

type WorldEngagementPreviewLinearHandle = {
  readonly kind: "linear-handle";
  readonly role?: string;
  readonly axis: readonly [number, number, number];
  readonly origin: readonly [number, number, number];
};

type WorldEngagementPreviewItem = WorldEngagementPreviewPoint | WorldEngagementPreviewSegment | WorldEngagementPreviewBox | WorldEngagementPreviewLinearHandle;

//#region WorldMeshPaint
/** 🎨 Mesh style kinds, in {@link resolveMeshStyle} priority order (highest first). */
type MeshStyleKind = "disabled" | "selected" | "highlighted" | "hovered" | "neutral";

type MeshStyleColors = {
  readonly meshColor: string;
  readonly lineColor: string;
  readonly emissiveIntensity: number;
  readonly opacity: number;
};

type MeshStylePalette = Readonly<Record<MeshStyleKind, MeshStyleColors>>;

/** 🎨 CSS-expression paint spec per style kind, ported from the premigration puzzle 3d paint table. */
const MESH_STYLE_PAINT: Readonly<Record<MeshStyleKind, { readonly fill: string; readonly line: string; readonly emissiveIntensity: number; readonly opacity: number }>> = {
  neutral: { fill: "var(--panel)", line: semanticVar("border-normal-color"), emissiveIntensity: 0, opacity: 1 },
  hovered: { fill: semanticVar("hover-interactive-fill"), line: semanticVar("border-emphasized-color"), emissiveIntensity: 0.08, opacity: 1 },
  selected: { fill: tokenVar("primary"), line: tokenVar("primary"), emissiveIntensity: 0.35, opacity: 1 },
  highlighted: { fill: tokenVar("secondary"), line: tokenVar("secondary"), emissiveIntensity: 0.2, opacity: 1 },
  disabled: { fill: "color-mix(in oklab, var(--color-muted-foreground) 55%, var(--panel))", line: themeColorVar("muted-foreground"), emissiveIntensity: 0, opacity: 0.45 },
};

/** 🎨 Resolves the full {@link MeshStylePalette} from live CSS custom properties (theme/dark-mode aware). */
function resolveMeshStylePalette(): MeshStylePalette {
  const resolved = {} as Record<MeshStyleKind, MeshStyleColors>;
  for (const kind of Object.keys(MESH_STYLE_PAINT) as MeshStyleKind[]) {
    const spec = MESH_STYLE_PAINT[kind];
    resolved[kind] = {
      meshColor: resolveColorHex(spec.fill),
      lineColor: resolveColorHex(spec.line),
      emissiveIntensity: spec.emissiveIntensity,
      opacity: spec.opacity,
    };
  }
  return resolved;
}

function useMeshStylePalette(): MeshStylePalette {
  const [palette, setPalette] = useState(resolveMeshStylePalette);
  useCanvasAppearanceSync(
    useCallback(() => {
      // 🎨 resolveColorHex caches by CSS-expression string only (no theme key), so a theme flip must bust it before re-resolving or every kind keeps its stale color.
      clearColorResolveCache();
      setPalette(resolveMeshStylePalette());
    }, []),
  );
  return palette;
}

/** 🎨 Resolves the effective style kind for an instance/component, premigration priority: disabled → selected → highlighted → hovered → neutral. */
export function resolveMeshStyle(state: { readonly disabled?: boolean; readonly selected?: boolean; readonly highlighted?: boolean; readonly hovered?: boolean }): MeshStyleKind {
  if (state.disabled) return "disabled";
  if (state.selected) return "selected";
  if (state.highlighted) return "highlighted";
  if (state.hovered) return "hovered";
  return "neutral";
}

/** 🎨 Resolves live group-selection preview paint: the new selection is active, while only objects exiting the old selection are highlighted. */
export function resolveMeshSelectionPreviewStyle(instance: Pick<WorldInstanceRecord, "disabled" | "selected" | "highlighted" | "hovered">, previewSelected?: boolean): MeshStyleKind {
  const selectionExited = previewSelected === false && instance.selected === true;
  return resolveMeshStyle({
    disabled: instance.disabled,
    selected: previewSelected ?? instance.selected,
    highlighted: selectionExited || instance.highlighted,
    hovered: instance.hovered,
  });
}

/** 🎨 Slim alias over {@link MeshStylePalette} for call sites that only need the four legacy semantic colors (face/edge/vertex component overlays, markers). */
type SemanticColors = {
  readonly mesh: string;
  readonly edge: string;
  readonly select: string;
  readonly hover: string;
};

function semanticColorsFromPalette(palette: MeshStylePalette): SemanticColors {
  return {
    mesh: palette.neutral.meshColor,
    edge: palette.neutral.lineColor,
    select: palette.selected.lineColor,
    hover: palette.hovered.meshColor,
  };
}
//#endregion WorldMeshPaint

type WorldParsedCameraState = WorldCameraState & { readonly fov: number; readonly explicitProjection: boolean };

function parseCameraState(cameraJson: string): WorldParsedCameraState {
  try {
    const parsed = JSON.parse(cameraJson) as WorldCameraRecord & { target?: readonly [number, number, number]; zoom?: number; up?: readonly [number, number, number]; projection?: string };
    const position: [number, number, number] = parsed.position ? [parsed.position[0], parsed.position[1], parsed.position[2]] : [parsed.x ?? 4, parsed.y ?? -4, parsed.z ?? 3];
    const target: [number, number, number] = parsed.target ? [parsed.target[0], parsed.target[1], parsed.target[2]] : [0, 0, 0];
    const explicitProjection = parsed.projection === "perspective" || parsed.projection === "orthographic";
    return {
      position,
      target,
      up: parsed.up ? [parsed.up[0], parsed.up[1], parsed.up[2]] : undefined,
      zoom: typeof parsed.zoom === "number" ? parsed.zoom : 1,
      projection: parsed.projection === "orthographic" ? "orthographic" : "perspective",
      fov: parsed.fov ?? 45,
      explicitProjection,
    };
  } catch {
    return { position: [4, -4, 3], target: [0, 0, 0], zoom: 1, projection: "perspective", fov: 45, explicitProjection: false };
  }
}

type WorldEnvironmentMaterialRecord = {
  readonly color?: string;
  readonly metalness?: number;
  readonly roughness?: number;
  readonly emissive?: string;
  readonly emissiveIntensity?: number;
};

type WorldEnvironmentRecord = {
  readonly background?: string;
  readonly ambient?: { readonly intensity?: number; readonly color?: string };
  readonly sun?: { readonly enabled?: boolean; readonly azimuth?: number; readonly elevation?: number; readonly intensity?: number; readonly color?: string };
  readonly shadow?: { readonly enabled?: boolean; readonly opacity?: number; readonly softness?: number };
  readonly material?: WorldEnvironmentMaterialRecord;
};

type WorldFrameRecord = {
  readonly width: number;
  readonly height: number;
  readonly shape?: string;
  readonly badge?: boolean;
  readonly background?: string;
};

type WorldFitRecord = {
  readonly enabled?: boolean;
  readonly revision?: number;
  readonly padding?: number;
};

function parseJsonRecord<T>(json?: string): T | null {
  if (!json) return null;
  try {
    const parsed = JSON.parse(json) as T | null;
    return typeof parsed === "object" ? parsed : null;
  } catch {
    return null;
  }
}

const parseEnvironment = (json?: string) => parseJsonRecord<WorldEnvironmentRecord>(json);
const parseFrame = (json?: string) => parseJsonRecord<WorldFrameRecord>(json);
const parseFit = (json?: string) => parseJsonRecord<WorldFitRecord>(json);

function isTransparentWorldBackground(background?: string): boolean {
  return !background || background === "transparent";
}

function fitCameraFromBounds(center: readonly [number, number, number], radius: number, camera: WorldParsedCameraState, padding: number): { position: [number, number, number]; target: [number, number, number]; zoom: number } {
  const distance = Math.max(radius * padding, 2);
  const dx = camera.position[0] - camera.target[0];
  const dy = camera.position[1] - camera.target[1];
  const dz = camera.position[2] - camera.target[2];
  const length = Math.hypot(dx, dy, dz);
  const nx = length > 1e-6 ? dx / length : 1;
  const ny = length > 1e-6 ? dy / length : -1;
  const nz = length > 1e-6 ? dz / length : 0.85;
  const norm = Math.hypot(nx, ny, nz) || 1;
  return {
    position: [center[0] + (nx / norm) * distance, center[1] + (ny / norm) * distance, center[2] + (nz / norm) * distance],
    target: [center[0], center[1], center[2]],
    zoom: camera.zoom,
  };
}

/** @emoji 🎯 Fits the orbit camera to the bounds of a scene group once per fit key, preserving the view direction. */
function WorldAutoFit({
  groupRef,
  fitKey,
  padding,
  camera,
  onFitted,
}: {
  readonly groupRef: React.RefObject<Group | null>;
  readonly fitKey: string;
  readonly padding: number;
  readonly camera: WorldParsedCameraState;
  readonly onFitted: (state: WorldCameraState) => void;
}): null {
  const { camera: sceneCamera, controls, invalidate } = useThree();
  const appliedKeyRef = useRef("");
  const targetScratch = useMemo(() => new Vector3(), []);
  useFrame(() => {
    if (!sceneCamera) return;
    const group = groupRef.current;
    if (!group) return;
    if (appliedKeyRef.current === fitKey) return;
    const box = new Box3().setFromObject(group);
    if (box.isEmpty()) return;
    const center = box.getCenter(new Vector3());
    const size = box.getSize(new Vector3());
    const radius = Math.max(size.x, size.y, size.z) * 0.5;
    if (radius <= 0) return;
    appliedKeyRef.current = fitKey;
    const fitted = fitCameraFromBounds([center.x, center.y, center.z], radius, camera, padding);
    const orbit = controls as { target: Vector3; update?: () => void } | null;
    const target = orbit?.target ?? targetScratch;
    target.set(fitted.target[0], fitted.target[1], fitted.target[2]);
    sceneCamera.position.set(fitted.position[0], fitted.position[1], fitted.position[2]);
    if ("zoom" in sceneCamera) sceneCamera.zoom = fitted.zoom;
    sceneCamera.updateProjectionMatrix();
    if (orbit) orbit.update?.();
    else sceneCamera.lookAt(target);
    invalidate();
    onFitted({ ...camera, position: fitted.position, target: fitted.target, zoom: fitted.zoom });
  });
  return null;
}

function autofitCameraFromInstances(instances: readonly WorldInstanceRecord[]): WorldParsedCameraState {
  if (instances.length === 0) {
    return { position: [4, -4, 3], target: [0, 0, 0], zoom: 1, projection: "perspective", fov: 45, explicitProjection: false };
  }
  let minX = Number.POSITIVE_INFINITY;
  let minY = Number.POSITIVE_INFINITY;
  let minZ = Number.POSITIVE_INFINITY;
  let maxX = Number.NEGATIVE_INFINITY;
  let maxY = Number.NEGATIVE_INFINITY;
  let maxZ = Number.NEGATIVE_INFINITY;
  for (const instance of instances) {
    const position = instance.position ?? [instance.x ?? 0, instance.y ?? 0, instance.z ?? 0];
    minX = Math.min(minX, position[0]);
    minY = Math.min(minY, position[1]);
    minZ = Math.min(minZ, position[2]);
    maxX = Math.max(maxX, position[0]);
    maxY = Math.max(maxY, position[1]);
    maxZ = Math.max(maxZ, position[2]);
  }
  const center: [number, number, number] = [(minX + maxX) / 2, (minY + maxY) / 2, (minZ + maxZ) / 2];
  const span = Math.max(maxX - minX, maxY - minY, maxZ - minZ, 1);
  const distance = span * 2.5;
  return {
    position: [center[0] + distance * 0.7, center[1] - distance * 0.7, center[2] + distance * 0.45],
    target: center,
    zoom: 1,
    projection: "perspective",
    fov: 45,
    explicitProjection: false,
  };
}

function parseMeshes(meshesJson: string): WorldMeshRecord[] {
  try {
    const parsed = JSON.parse(meshesJson);
    return Array.isArray(parsed) ? (parsed as WorldMeshRecord[]) : [];
  } catch {
    return [];
  }
}

function parseInstances(instancesJson: string): WorldInstanceRecord[] {
  try {
    const parsed = JSON.parse(instancesJson);
    return Array.isArray(parsed) ? (parsed as WorldInstanceRecord[]) : [];
  } catch {
    return [];
  }
}

function parseSelection(selectionJson: string): WorldSelectionRecord {
  try {
    return JSON.parse(selectionJson) as WorldSelectionRecord;
  } catch {
    return { method: "rectangle", ids: [] };
  }
}

function parseJsonArray<T>(json: string | undefined): readonly T[] {
  if (!json) return [];
  try {
    const parsed = JSON.parse(json);
    return Array.isArray(parsed) ? (parsed as T[]) : [];
  } catch {
    return [];
  }
}

function parseInteraction(interactionJson: string | undefined): WorldInteractionRecord {
  if (!interactionJson) return {};
  try {
    return JSON.parse(interactionJson) as WorldInteractionRecord;
  } catch {
    return {};
  }
}

function parseLod(lodJson: string | undefined): WorldLodRecord {
  if (!lodJson) return {};
  try {
    return JSON.parse(lodJson) as WorldLodRecord;
  } catch {
    return {};
  }
}

function parseBrushPreview(brushPreviewJson: string | undefined): WorldBrushPreviewRecord | null {
  if (!brushPreviewJson) return null;
  try {
    return JSON.parse(brushPreviewJson) as WorldBrushPreviewRecord;
  } catch {
    return null;
  }
}

type WorldContextMenuTarget = { readonly kind: "vortex" | "object" | "reference"; readonly id: string };

/** @emoji 🖱️ Resolves which entity a plain right-click should select-then-open a menu for, by priority: hovered vortex, then hovered object component, then hovered reference. */
export function resolveWorldContextMenuTarget(interaction: WorldInteractionRecord, selection: WorldSelectionRecord): WorldContextMenuTarget | null {
  if (interaction.hoveredVortexFullId) return { kind: "vortex", id: interaction.hoveredVortexFullId };
  if (selection.hoveredComponent?.objectId) return { kind: "object", id: selection.hoveredComponent.objectId };
  const hoveredId = selection.hoveredId;
  if (hoveredId?.startsWith("reference:")) return { kind: "reference", id: hoveredId.slice("reference:".length) };
  return null;
}

/** @emoji 🚫 Instance-mesh picking must be disabled for fill/brush engagements — otherwise a click meant for a vortex marker or a fill/voxel gesture falls through and selects/gumballs the underlying object instead. */
export function worldInstancePickBlocked(activeUtility: string | undefined): boolean {
  return activeUtility === "fill" || activeUtility === "brush";
}

/** @emoji 🖱️ In brush mode or vertex selection mode, pointer-down on a vortex selects immediately; otherwise a click selects and a drag starts connect. */
export function resolveVortexPointerDownIntent(brushMode: boolean, selectionMode?: string): "select" | "click-or-drag" {
  return brushMode || selectionMode === "vertex" ? "select" : "click-or-drag";
}

/** @emoji 🧱 Builds the `addBrushObject` action args from a parsed brush preview, or `null` if there is nothing to place yet. */
export function brushObjectPlacementArgs(preview: WorldBrushPreviewRecord | null): Record<string, unknown> | null {
  if (!preview) return null;
  return {
    targetVortexFullId: preview.targetVortexFullId,
    objectKindId: preview.objectKindId,
    sourceVortexIndex: preview.sourceVortexIndex ?? 0,
    origin: preview.origin,
    orientation: preview.orientation,
    scale: preview.scale,
  };
}

function parseEngagementPreview(engagementPreviewJson: string | undefined): readonly WorldEngagementPreviewItem[] {
  return parseJsonArray<WorldEngagementPreviewItem>(engagementPreviewJson);
}

function scaleTuple(scale: WorldBrushPreviewRecord["scale"]): [number, number, number] {
  if (typeof scale === "number") return [scale, scale, scale];
  if (Array.isArray(scale) && scale.length >= 3) return [scale[0]!, scale[1]!, scale[2]!];
  return [1, 1, 1];
}

function geometryFromMesh(mesh: WorldMeshData) {
  const geometry = new BufferGeometry();
  geometry.setAttribute("position", new BufferAttribute(new Float32Array(mesh.positions), 3));
  geometry.setAttribute("normal", new BufferAttribute(new Float32Array(mesh.normals), 3));
  if (mesh.uvs?.length) geometry.setAttribute("uv", new BufferAttribute(new Float32Array(mesh.uvs), 2));
  if (mesh.colors?.length) geometry.setAttribute("color", new BufferAttribute(new Float32Array(mesh.colors), 3));
  if (mesh.indices.length > 0) geometry.setIndex([...mesh.indices]);
  return geometry;
}

type VertexPickData = {
  readonly geometry: BufferGeometry;
  readonly vertexIds: readonly number[];
};

function buildVertexPickData(mesh: WorldMeshData): VertexPickData | null {
  if (!mesh.vertexIds?.length) return null;
  const positions: number[] = [];
  const vertexIds: number[] = [];
  const emitted = new Set<number>();
  for (let index = 0; index < mesh.vertexIds.length; index += 1) {
    const id = mesh.vertexIds[index]!;
    if (emitted.has(id)) continue;
    emitted.add(id);
    vertexIds.push(id);
    positions.push(mesh.positions[index * 3]!, mesh.positions[index * 3 + 1]!, mesh.positions[index * 3 + 2]!);
  }
  if (!positions.length) return null;
  const geometry = new BufferGeometry();
  geometry.setAttribute("position", new BufferAttribute(new Float32Array(positions), 3));
  return { geometry, vertexIds };
}

function buildEdgeGeometry(mesh: WorldMeshData): BufferGeometry | null {
  if (!mesh.edgePositions?.length) return null;
  const geometry = new BufferGeometry();
  geometry.setAttribute("position", new BufferAttribute(new Float32Array(mesh.edgePositions), 3));
  return geometry;
}

function buildFaceOverlayGeometry(mesh: WorldMeshData, faceIds: ReadonlySet<number>): BufferGeometry | null {
  if (!mesh.faceIds?.length || !mesh.indices.length || faceIds.size === 0) return null;
  const positions: number[] = [];
  const normals: number[] = [];
  for (let faceIndex = 0; faceIndex < mesh.faceIds.length; faceIndex += 1) {
    const faceId = mesh.faceIds[faceIndex]!;
    if (!faceIds.has(faceId)) continue;
    const i0 = mesh.indices[faceIndex * 3] ?? 0;
    const i1 = mesh.indices[faceIndex * 3 + 1] ?? 0;
    const i2 = mesh.indices[faceIndex * 3 + 2] ?? 0;
    for (const index of [i0, i1, i2]) {
      positions.push(mesh.positions[index * 3]!, mesh.positions[index * 3 + 1]!, mesh.positions[index * 3 + 2]!);
      normals.push(mesh.normals[index * 3]!, mesh.normals[index * 3 + 1]!, mesh.normals[index * 3 + 2]!);
    }
  }
  if (!positions.length) return null;
  const geometry = new BufferGeometry();
  geometry.setAttribute("position", new BufferAttribute(new Float32Array(positions), 3));
  geometry.setAttribute("normal", new BufferAttribute(new Float32Array(normals), 3));
  return geometry;
}

/** @emoji 🖱️➡️ Approximates a picked face's in-plane size from its triangles' local bounding box, dropping the
 * smallest axis (roughly the one aligned with the face normal for axis-aligned primitive faces) — good
 * enough to size a push/pull tool's footprint without needing a true tangent-plane projection. */
function faceExtentFromMesh(mesh: WorldMeshData, faceId: number): readonly [number, number] | undefined {
  if (!mesh.faceIds?.length || !mesh.indices.length) return undefined;
  let minX = Infinity;
  let maxX = -Infinity;
  let minY = Infinity;
  let maxY = -Infinity;
  let minZ = Infinity;
  let maxZ = -Infinity;
  let found = false;
  for (let faceIndex = 0; faceIndex < mesh.faceIds.length; faceIndex += 1) {
    if (mesh.faceIds[faceIndex] !== faceId) continue;
    found = true;
    for (const corner of [0, 1, 2]) {
      const vertexIndex = mesh.indices[faceIndex * 3 + corner];
      if (vertexIndex == null) continue;
      const x = mesh.positions[vertexIndex * 3] ?? 0;
      const y = mesh.positions[vertexIndex * 3 + 1] ?? 0;
      const z = mesh.positions[vertexIndex * 3 + 2] ?? 0;
      minX = Math.min(minX, x);
      maxX = Math.max(maxX, x);
      minY = Math.min(minY, y);
      maxY = Math.max(maxY, y);
      minZ = Math.min(minZ, z);
      maxZ = Math.max(maxZ, z);
    }
  }
  if (!found) return undefined;
  const extents = [maxX - minX, maxY - minY, maxZ - minZ].sort((a, b) => b - a);
  return [extents[0] ?? 0.2, extents[1] ?? 0.2];
}

function buildEdgeOverlayGeometry(mesh: WorldMeshData, edgeIds: ReadonlySet<number>): BufferGeometry | null {
  if (!mesh.edgeIds?.length || !mesh.edgePositions?.length || edgeIds.size === 0) return null;
  const positions: number[] = [];
  for (let edgeIndex = 0; edgeIndex < mesh.edgeIds.length; edgeIndex += 1) {
    if (!edgeIds.has(mesh.edgeIds[edgeIndex]!)) continue;
    const base = edgeIndex * 6;
    positions.push(mesh.edgePositions[base]!, mesh.edgePositions[base + 1]!, mesh.edgePositions[base + 2]!, mesh.edgePositions[base + 3]!, mesh.edgePositions[base + 4]!, mesh.edgePositions[base + 5]!);
  }
  if (!positions.length) return null;
  const geometry = new BufferGeometry();
  geometry.setAttribute("position", new BufferAttribute(new Float32Array(positions), 3));
  return geometry;
}

function buildVertexOverlayGeometry(mesh: WorldMeshData, vertexIds: ReadonlySet<number>): BufferGeometry | null {
  const pick = buildVertexPickData(mesh);
  if (!pick) return null;
  const positions: number[] = [];
  for (let index = 0; index < pick.vertexIds.length; index += 1) {
    if (!vertexIds.has(pick.vertexIds[index]!)) continue;
    positions.push(pick.geometry.attributes.position!.getX(index), pick.geometry.attributes.position!.getY(index), pick.geometry.attributes.position!.getZ(index));
  }
  if (!positions.length) return null;
  const geometry = new BufferGeometry();
  geometry.setAttribute("position", new BufferAttribute(new Float32Array(positions), 3));
  return geometry;
}

function paintTextureUrl(base64: string): string {
  return `data:image/png;base64,${base64}`;
}

function PaintTexturedMesh({
  geometry,
  style,
  textureBase64,
  flatShading,
  children,
  ...meshProps
}: {
  readonly geometry: BufferGeometry;
  readonly style: MeshStyleColors;
  readonly textureBase64?: string;
  readonly flatShading?: boolean;
  readonly children?: React.ReactNode;
} & ComponentProps<"mesh">) {
  const paintMap = textureBase64 ? useLoader(TextureLoader, paintTextureUrl(textureBase64)) : null;
  // Per-vertex colors (e.g. FEM stress contours) multiply against the material's own `color` in
  // three.js, so white lets them show through unmodified — `style.meshColor` would otherwise tint them.
  const hasVertexColors = geometry.hasAttribute("color");
  return (
    <mesh geometry={geometry} {...meshProps}>
      <meshStandardMaterial
        color={hasVertexColors ? "#ffffff" : style.meshColor}
        vertexColors={hasVertexColors}
        map={paintMap ?? undefined}
        side={DoubleSide}
        flatShading={flatShading}
        metalness={0}
        roughness={1}
        emissive={hasVertexColors ? "#000000" : style.meshColor}
        emissiveIntensity={hasVertexColors ? 0 : style.emissiveIntensity}
        transparent={style.opacity < 1}
        opacity={style.opacity}
      />
      {children}
    </mesh>
  );
}

//#region GlbMeshStyling
/** 🎨 EdgesGeometry cache keyed by source BufferGeometry — `gltf.scene.clone(true)` shares geometries across every per-instance clone of the same GLB, so this dedupes edge computation across instances. */
const GLB_EDGE_GEOMETRY_CACHE = new WeakMap<BufferGeometry, EdgesGeometry>();

/** 🎨 Adds a border-color {@link EdgesGeometry} outline to every mesh under `root` (idempotent), using the shared {@link GLB_EDGE_GEOMETRY_CACHE}. */
function applyGlbMeshEdgeBorders(root: Object3D, borderColor: string): void {
  // 🧵 Collect targets before mutating: `object.add(...)` during `traverse()` would splice the new
  // (itself a Mesh) child into the live `children` array traverse is still walking, so it gets visited
  // and outlined again — and again — recursing until the stack overflows.
  const targets: Mesh[] = [];
  root.traverse((object) => {
    if (!(object instanceof Mesh)) return;
    const geometry = object.geometry;
    if (!geometry || object.children.some((child) => child.userData[WORLD_MESH_OUTLINE_USER_DATA_KEY])) return;
    targets.push(object);
  });
  for (const object of targets) {
    let edges = GLB_EDGE_GEOMETRY_CACHE.get(object.geometry);
    if (!edges) {
      edges = new EdgesGeometry(object.geometry);
      GLB_EDGE_GEOMETRY_CACHE.set(object.geometry, edges);
    }
    const outline = new LineSegments(edges, new LineBasicMaterial({ color: new Color(borderColor) }));
    outline.userData[WORLD_MESH_OUTLINE_USER_DATA_KEY] = true;
    outline.scale.setScalar(1.001);
    object.add(outline);
  }
}

//#endregion GlbMeshStyling

function GlbInstanceMesh({
  url,
  color,
  emissive,
  emissiveIntensity,
  opacity,
  borderColor,
  material,
  shadowEnabled,
}: {
  readonly url: string;
  readonly color: string;
  readonly emissive: string;
  readonly emissiveIntensity: number;
  readonly opacity: number;
  readonly borderColor: string;
  readonly material?: WorldEnvironmentMaterialRecord;
  readonly shadowEnabled?: boolean;
}) {
  const gltf = useLoader(GLTFLoader, url);
  const scene = useMemo(() => {
    const cloned = gltf.scene.clone(true);
    cloned.traverse((child) => {
      if (!(child instanceof Mesh)) return;
      child.material = new MeshStandardMaterial({ metalness: material?.metalness ?? 0, roughness: material?.roughness ?? 1 });
      child.castShadow = shadowEnabled === true;
      child.receiveShadow = shadowEnabled === true;
    });
    applyGlbMeshEdgeBorders(cloned, borderColor);
    return cloned;
    // eslint-disable-next-line react-hooks/exhaustive-deps -- borderColor intentionally excluded: applied once at clone time, then kept in sync imperatively by the effect below without rebuilding the clone.
  }, [gltf.scene, material?.metalness, material?.roughness, shadowEnabled]);

  useEffect(() => {
    scene.traverse((child) => {
      if (child instanceof Mesh) {
        const standard = child.material;
        if (standard instanceof MeshStandardMaterial) {
          standard.color.set(color);
          standard.emissive.set(emissive);
          standard.emissiveIntensity = emissiveIntensity;
          standard.transparent = opacity < 1;
          standard.opacity = opacity;
        }
        return;
      }
      if (child instanceof LineSegments && child.userData[WORLD_MESH_OUTLINE_USER_DATA_KEY]) {
        (child.material as LineBasicMaterial).color.set(borderColor);
      }
    });
  }, [scene, color, emissive, emissiveIntensity, opacity, borderColor]);

  return (
    <group rotation={[GLB_MESH_FRAME_ROTATION_X, 0, 0]}>
      <primitive object={scene} />
    </group>
  );
}

function extractGlbCollisionMesh(gltf: Awaited<ReturnType<GLTFLoader["loadAsync"]>>): {
  readonly positions: number[];
  readonly indices: number[];
} {
  const frame = new Object3D();
  frame.rotation.x = GLB_MESH_FRAME_ROTATION_X;
  frame.updateMatrixWorld(true);
  const positions: number[] = [];
  const indices: number[] = [];
  let vertexOffset = 0;
  const scratch = new Vector3();
  gltf.scene.updateMatrixWorld(true);
  gltf.scene.traverse((child) => {
    if (!(child instanceof Mesh)) return;
    const geometry = child.geometry;
    const positionAttr = geometry.getAttribute("position");
    if (!positionAttr) return;
    const worldMatrix = frame.matrixWorld.clone().multiply(child.matrixWorld);
    for (let index = 0; index < positionAttr.count; index += 1) {
      scratch.fromBufferAttribute(positionAttr, index).applyMatrix4(worldMatrix);
      positions.push(scratch.x, scratch.y, scratch.z);
    }
    const indexAttr = geometry.index;
    if (indexAttr) {
      for (let index = 0; index < indexAttr.count; index += 1) {
        indices.push(indexAttr.getX(index) + vertexOffset);
      }
    } else {
      for (let index = 0; index < positionAttr.count; index += 3) {
        indices.push(vertexOffset + index, vertexOffset + index + 1, vertexOffset + index + 2);
      }
    }
    vertexOffset += positionAttr.count;
  });
  return { positions, indices };
}

function BrushMeshRegistrar({ url, onRegister }: { readonly url: string; readonly onRegister: (url: string, positions: number[], indices: number[]) => void }) {
  const gltf = useLoader(GLTFLoader, url);
  useEffect(() => {
    const mesh = extractGlbCollisionMesh(gltf);
    if (mesh.positions.length === 0 || mesh.indices.length === 0) return;
    onRegister(url, mesh.positions, mesh.indices);
  }, [gltf, onRegister, url]);
  return null;
}

function gumballConfigForTransformMode(mode: string): GumballConfig {
  if (mode === "rotate") {
    return { moveAxes: false, movePlanes: false, rotate: true, scaleAxes: false, scalePlanes: false, scaleUniform: false };
  }
  if (mode === "scale") {
    return { moveAxes: false, movePlanes: false, rotate: false, scaleAxes: true, scalePlanes: true, scaleUniform: true };
  }
  return { moveAxes: true, movePlanes: true, rotate: false, scaleAxes: false, scalePlanes: false, scaleUniform: false };
}

function SceneGumball({
  target,
  config,
  active,
  onDraggingChanged,
  onDragEnd,
}: {
  readonly target?: readonly [number, number, number];
  readonly config: GumballConfig;
  readonly active: boolean;
  readonly onDraggingChanged: (dragging: boolean) => void;
  readonly onDragEnd: (kind: GumballHandleKind, before: GumballPose, after: GumballPose) => void;
}) {
  const pivotRef = useRef<Object3D>(new Object3D());
  const [ready, setReady] = useState(false);
  useEffect(() => {
    if (!target) return;
    pivotRef.current.position.set(target[0], target[1], target[2]);
    pivotRef.current.quaternion.set(0, 0, 0, 1);
    pivotRef.current.scale.set(1, 1, 1);
    pivotRef.current.updateMatrixWorld(true);
    setReady(true);
  }, [target]);
  if (!active || !target || !ready) return null;
  return (
    <>
      <primitive object={pivotRef.current} />
      <UnifiedGumball
        target={pivotRef.current}
        config={config}
        onDraggingChanged={onDraggingChanged}
        onDragEnd={(kind, before, after) => {
          onDragEnd(kind, before, after);
          pivotRef.current.position.set(target[0], target[1], target[2]);
          pivotRef.current.quaternion.set(0, 0, 0, 1);
          pivotRef.current.scale.set(1, 1, 1);
          pivotRef.current.updateMatrixWorld(true);
        }}
      />
    </>
  );
}

function WorldInstanceNode({
  instance,
  index,
  meshRecord,
  meshData,
  geometry,
  borderGeometry,
  palette,
  vertexPick,
  edgeGeometry,
  paintTextureBase64,
  position,
  scale,
  quaternion,
  targets,
  activeObjectId,
  selectionMode,
  selectedComponentIds,
  previewComponentIds,
  hoveredComponent,
  showEdges,
  pickEnabled,
  onPaintAt,
  paintFromHit,
  flatShading,
  onInstancePointerDown,
  onInstancePointerMove,
  onWorldPick,
  onComponentHover,
  mergeMode,
  previewInstanceSelected,
  environmentMaterial,
  environmentShadowEnabled,
  faceDragActive,
  onFaceDragStart,
}: {
  readonly instance: WorldInstanceRecord;
  readonly index: number;
  readonly meshRecord?: WorldMeshRecord;
  readonly meshData?: WorldMeshData;
  readonly geometry?: BufferGeometry;
  /** 🎨 Shared per-meshId edge outline geometry (see {@link WorldInstancesLayer}'s `geometries` memo); never rebuilt per instance. */
  readonly borderGeometry?: EdgesGeometry;
  readonly palette: MeshStylePalette;
  readonly vertexPick: VertexPickData | null;
  readonly edgeGeometry: BufferGeometry | null;
  readonly paintTextureBase64?: string;
  readonly position: readonly [number, number, number];
  readonly scale: readonly [number, number, number];
  readonly quaternion?: Quaternion;
  readonly targets: WorldSelectionTargets;
  readonly activeObjectId?: string;
  readonly selectionMode: string;
  readonly selectedComponentIds: ReadonlySet<number>;
  readonly previewComponentIds: ReadonlySet<number>;
  readonly hoveredComponent?: WorldHoverComponent;
  readonly showEdges?: boolean;
  readonly pickEnabled: boolean;
  readonly onPaintAt?: (objectId: string, u: number, v: number) => void;
  readonly paintFromHit: (objectId: string, mesh: WorldMeshData, event: ThreeEvent<PointerEvent> & { faceIndex?: number | null; uv?: { x: number; y: number } }) => void;
  readonly flatShading?: boolean;
  readonly onInstancePointerDown: (id: string, index: number, event: { shiftKey: boolean; ctrlKey: boolean; metaKey: boolean }) => void;
  readonly onInstancePointerMove: (id: string | null) => void;
  readonly onWorldPick: (args: { granularity: string; id: number; merge: string }) => void;
  readonly onComponentHover: (args: { objectId: string; mode: string; id: number } | null) => void;
  readonly mergeMode: (event: { shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => string;
  /** 🖱️➡️ When true, pointer-down on an already-selected face starts a push/pull drag instead of falling through to selection/orbit. */
  readonly faceDragActive?: boolean;
  readonly onFaceDragStart?: (args: { objectId: string; faceId: number; normal: readonly [number, number, number]; point: readonly [number, number, number]; faceExtent?: readonly [number, number] }) => void;
  /** Live marquee-drag merged selection state for this instance; undefined when no drag is in progress. */
  readonly previewInstanceSelected?: boolean;
  readonly environmentMaterial?: WorldEnvironmentMaterialRecord;
  readonly environmentShadowEnabled?: boolean;
}) {
  const isActiveObject = instance.id === activeObjectId;
  const colors = semanticColorsFromPalette(palette);
  const styleKind = resolveMeshSelectionPreviewStyle(instance, previewInstanceSelected);
  const style = palette[styleKind];
  const glbUsesEnvironmentColor = styleKind === "neutral" && environmentMaterial?.color != null;
  const glbColor = glbUsesEnvironmentColor ? environmentMaterial!.color! : style.meshColor;
  const glbEmissive = glbUsesEnvironmentColor && environmentMaterial?.emissive ? environmentMaterial.emissive : style.meshColor;
  const glbEmissiveIntensity = glbUsesEnvironmentColor && environmentMaterial?.emissive ? (environmentMaterial.emissiveIntensity ?? 1) : style.emissiveIntensity;
  const hoveredFaceId = hoveredComponent?.mode === "face" && hoveredComponent.objectId === instance.id ? hoveredComponent.id : undefined;
  const hoveredVertexId = hoveredComponent?.mode === "vertex" && hoveredComponent.objectId === instance.id ? hoveredComponent.id : undefined;
  const hoveredEdgeId = hoveredComponent?.mode === "edge" && hoveredComponent.objectId === instance.id ? hoveredComponent.id : undefined;
  const selectedFaceIds = isActiveObject && selectionMode === "face" ? selectedComponentIds : new Set<number>();
  const selectedVertexIds = isActiveObject && selectionMode === "vertex" ? selectedComponentIds : new Set<number>();
  const selectedEdgeIds = isActiveObject && selectionMode === "edge" ? selectedComponentIds : new Set<number>();
  const previewFaceIds = isActiveObject && selectionMode === "face" ? previewComponentIds : new Set<number>();
  const previewVertexIds = isActiveObject && selectionMode === "vertex" ? previewComponentIds : new Set<number>();
  const previewEdgeIds = isActiveObject && selectionMode === "edge" ? previewComponentIds : new Set<number>();
  const facePreviewOverlay = meshData && previewFaceIds.size > 0 ? buildFaceOverlayGeometry(meshData, previewFaceIds) : null;
  const edgePreviewOverlay = meshData && previewEdgeIds.size > 0 ? buildEdgeOverlayGeometry(meshData, previewEdgeIds) : null;
  const vertexPreviewOverlay = meshData && previewVertexIds.size > 0 ? buildVertexOverlayGeometry(meshData, previewVertexIds) : null;
  const faceSelectedOverlay = meshData ? buildFaceOverlayGeometry(meshData, selectedFaceIds) : null;
  const faceHoveredOverlay = meshData && hoveredFaceId != null ? buildFaceOverlayGeometry(meshData, new Set([hoveredFaceId])) : null;
  const edgeSelectedOverlay = meshData ? buildEdgeOverlayGeometry(meshData, selectedEdgeIds) : null;
  const edgeHoveredOverlay = meshData && hoveredEdgeId != null ? buildEdgeOverlayGeometry(meshData, new Set([hoveredEdgeId])) : null;
  const vertexSelectedOverlay = meshData ? buildVertexOverlayGeometry(meshData, selectedVertexIds) : null;
  const vertexHoveredOverlay = meshData && hoveredVertexId != null ? buildVertexOverlayGeometry(meshData, new Set([hoveredVertexId])) : null;

  return (
    <group position={position as [number, number, number]} scale={scale as [number, number, number]} quaternion={quaternion}>
      {geometry && meshData ? (
        <>
          <PaintTexturedMesh
            geometry={geometry}
            style={style}
            textureBase64={paintTextureBase64}
            flatShading={flatShading}
            onPointerDown={(event) => {
              if (onPaintAt || !faceDragActive || !onFaceDragStart || !event.face) return;
              if (!(targets.face && event.faceIndex != null && meshData.faceIds?.[event.faceIndex] != null)) return;
              const faceId = meshData.faceIds[event.faceIndex]!;
              if (!(isActiveObject && selectionMode === "face" && selectedComponentIds.has(faceId))) return;
              event.stopPropagation();
              const normal = event.face.normal.clone().transformDirection(event.object.matrixWorld).normalize();
              onFaceDragStart({
                objectId: instance.id,
                faceId,
                normal: [normal.x, normal.y, normal.z],
                point: [event.point.x, event.point.y, event.point.z],
                faceExtent: faceExtentFromMesh(meshData, faceId),
              });
            }}
            onClick={(event) => {
              if (onPaintAt) {
                paintFromHit(instance.id, meshData, event);
                return;
              }
              if (!pickEnabled) return;
              event.stopPropagation();
              if (targets.face && event.faceIndex != null && meshData.faceIds?.[event.faceIndex] != null) {
                onWorldPick({
                  granularity: "face",
                  id: meshData.faceIds[event.faceIndex]!,
                  merge: mergeMode(event),
                });
              } else if (targets.mesh) {
                onInstancePointerDown(instance.id, index, event);
              }
            }}
            onPointerMove={(event) => {
              if (onPaintAt) {
                if ((event.buttons & 1) !== 0) paintFromHit(instance.id, meshData, event);
                return;
              }
              if (!pickEnabled) return;
              event.stopPropagation();
              if (targets.face && event.faceIndex != null && meshData.faceIds?.[event.faceIndex] != null) {
                onComponentHover({
                  objectId: instance.id,
                  mode: "face",
                  id: meshData.faceIds[event.faceIndex]!,
                });
              } else {
                onInstancePointerMove(instance.id);
              }
            }}
            onPointerOut={() => {
              onInstancePointerMove(null);
              onComponentHover(null);
            }}
          ></PaintTexturedMesh>
          {borderGeometry && (showEdges ?? true) ? (
            <lineSegments geometry={borderGeometry} scale={1.001} raycast={() => null}>
              <lineBasicMaterial color={palette.neutral.lineColor} />
            </lineSegments>
          ) : null}
          {(targets.edge || (showEdges ?? true) || (selectionMode === "mesh" && selectedComponentIds.size > 0)) && edgeGeometry ? (
            <lineSegments
              geometry={edgeGeometry}
              onClick={(event) => {
                if (!pickEnabled || !meshData?.edgeIds?.length) return;
                event.stopPropagation();
                const edgeIndex = Math.floor((event.index ?? 0) / 2);
                const edgeId = meshData.edgeIds[edgeIndex];
                if (edgeId == null) return;
                onWorldPick({ granularity: "edge", id: edgeId, merge: mergeMode(event) });
              }}
              onPointerMove={(event) => {
                if (!pickEnabled || !meshData?.edgeIds?.length) return;
                event.stopPropagation();
                const edgeIndex = Math.floor((event.index ?? 0) / 2);
                const edgeId = meshData.edgeIds[edgeIndex];
                if (edgeId == null) return;
                onComponentHover({ objectId: instance.id, mode: "edge", id: edgeId });
              }}
              onPointerOut={() => onComponentHover(null)}
            >
              <lineBasicMaterial color={colors.edge} linewidth={1} />
            </lineSegments>
          ) : null}
          {targets.vertex && vertexPick ? (
            <points
              geometry={vertexPick.geometry}
              onClick={(event) => {
                if (!pickEnabled) return;
                event.stopPropagation();
                const idx = event.index ?? 0;
                const vertexId = vertexPick.vertexIds[idx];
                if (vertexId == null) return;
                onWorldPick({ granularity: "vertex", id: vertexId, merge: mergeMode(event) });
              }}
              onPointerMove={(event) => {
                if (!pickEnabled) return;
                event.stopPropagation();
                const idx = event.index ?? 0;
                const vertexId = vertexPick.vertexIds[idx];
                if (vertexId == null) return;
                onComponentHover({ objectId: instance.id, mode: "vertex", id: vertexId });
              }}
              onPointerOut={() => onComponentHover(null)}
            >
              <pointsMaterial color={colors.edge} size={0.05} sizeAttenuation />
            </points>
          ) : null}
          {faceSelectedOverlay ? (
            <mesh geometry={faceSelectedOverlay} raycast={() => null}>
              <meshBasicMaterial color={colors.select} transparent opacity={0.62} side={DoubleSide} depthWrite={false} polygonOffset polygonOffsetFactor={-2} />
            </mesh>
          ) : null}
          {faceHoveredOverlay ? (
            <mesh geometry={faceHoveredOverlay} raycast={() => null}>
              <meshBasicMaterial color={colors.hover} transparent opacity={0.48} side={DoubleSide} depthWrite={false} polygonOffset polygonOffsetFactor={-3} />
            </mesh>
          ) : null}
          {facePreviewOverlay ? (
            <mesh geometry={facePreviewOverlay} raycast={() => null}>
              <meshBasicMaterial color={colors.hover} transparent opacity={0.36} side={DoubleSide} depthWrite={false} polygonOffset polygonOffsetFactor={-4} />
            </mesh>
          ) : null}
          {edgeSelectedOverlay ? (
            <lineSegments geometry={edgeSelectedOverlay} raycast={() => null}>
              <lineBasicMaterial color={colors.select} linewidth={3} />
            </lineSegments>
          ) : null}
          {edgeHoveredOverlay ? (
            <lineSegments geometry={edgeHoveredOverlay} raycast={() => null}>
              <lineBasicMaterial color={colors.hover} linewidth={3} />
            </lineSegments>
          ) : null}
          {edgePreviewOverlay ? (
            <lineSegments geometry={edgePreviewOverlay} raycast={() => null}>
              <lineBasicMaterial color={colors.hover} linewidth={2} />
            </lineSegments>
          ) : null}
          {vertexSelectedOverlay ? (
            <points geometry={vertexSelectedOverlay} raycast={() => null}>
              <pointsMaterial color={colors.select} size={0.09} sizeAttenuation depthTest={false} />
            </points>
          ) : null}
          {vertexHoveredOverlay ? (
            <points geometry={vertexHoveredOverlay} raycast={() => null}>
              <pointsMaterial color={colors.hover} size={0.09} sizeAttenuation depthTest={false} />
            </points>
          ) : null}
          {vertexPreviewOverlay ? (
            <points geometry={vertexPreviewOverlay} raycast={() => null}>
              <pointsMaterial color={colors.hover} size={0.09} sizeAttenuation depthTest={false} />
            </points>
          ) : null}
        </>
      ) : meshRecord?.url ? (
        <group
          onPointerDown={(event) => {
            event.stopPropagation();
            onInstancePointerDown(instance.id, index, event);
          }}
          onPointerMove={(event) => {
            event.stopPropagation();
            onInstancePointerMove(instance.id);
          }}
          onPointerOut={() => onInstancePointerMove(null)}
        >
          <Suspense fallback={null}>
            <GlbInstanceMesh
              url={meshRecord.url}
              color={glbColor}
              emissive={glbEmissive}
              emissiveIntensity={glbEmissiveIntensity}
              opacity={style.opacity}
              borderColor={palette.neutral.lineColor}
              material={environmentMaterial}
              shadowEnabled={environmentShadowEnabled}
            />
          </Suspense>
        </group>
      ) : (
        <mesh
          onPointerDown={(event) => {
            event.stopPropagation();
            onInstancePointerDown(instance.id, index, event);
          }}
        >
          <boxGeometry args={[1, 1, 1]} />
          <meshStandardMaterial color={style.meshColor} metalness={0} roughness={1} emissive={style.meshColor} emissiveIntensity={style.emissiveIntensity} transparent={style.opacity < 1} opacity={style.opacity} />
        </mesh>
      )}
    </group>
  );
}
//#endregion WorldSceneParsing

//#region WorldInstancesLayer
function WorldInstancesLayer({
  instances,
  meshes,
  selection,
  palette,
  onInstancePointerDown,
  onInstancePointerMove,
  onWorldPick,
  onComponentHover,
  onPaintAt,
  gumballDragActive,
  onGumballDraggingChanged,
  onGumballDragEnd,
  onFaceDragStart,
  mergedComponentIds,
  mergedInstanceIds,
  blockPick,
  environment,
}: {
  readonly instances: readonly WorldInstanceRecord[];
  readonly meshes: readonly WorldMeshRecord[];
  readonly selection: WorldSelectionRecord;
  readonly palette: MeshStylePalette;
  readonly onInstancePointerDown: (id: string, index: number, event: { shiftKey: boolean; ctrlKey: boolean; metaKey: boolean }) => void;
  readonly onInstancePointerMove: (id: string | null) => void;
  readonly onWorldPick: (args: { granularity: string; id: number; merge: string }) => void;
  readonly onComponentHover: (args: { objectId: string; mode: string; id: number } | null) => void;
  readonly onPaintAt?: (objectId: string, u: number, v: number) => void;
  readonly gumballDragActive: boolean;
  readonly onGumballDraggingChanged: (dragging: boolean) => void;
  readonly onGumballDragEnd: (kind: GumballHandleKind, before: GumballPose, after: GumballPose) => void;
  readonly onFaceDragStart?: (args: { objectId: string; faceId: number; normal: readonly [number, number, number]; point: readonly [number, number, number]; faceExtent?: readonly [number, number] }) => void;
  /** Live drag-preview merged component id set (null when no marquee drag is in progress). */
  readonly mergedComponentIds?: readonly number[] | null;
  /** Live drag-preview merged whole-instance id set (null when no marquee drag is in progress). */
  readonly mergedInstanceIds?: readonly string[] | null;
  /** Disables instance picking; passed for fill and brush engagements so a click meant for a vortex marker can't fall through and select/gumball the underlying object instead. */
  readonly blockPick?: boolean;
  readonly environment?: WorldEnvironmentRecord | null;
}) {
  const meshById = useMemo(() => new Map(meshes.map((mesh) => [mesh.id, mesh])), [meshes]);
  const geometries = useMemo(() => {
    const map = new Map<string, BufferGeometry>();
    for (const mesh of meshes) {
      if (mesh.data) map.set(mesh.id, geometryFromMesh(mesh.data));
    }
    return map;
  }, [meshes]);
  /** 🎨 Per-meshId border outline geometry, shared by every instance of that mesh — never rebuilt per instance. */
  const borderGeometries = useMemo(() => {
    const map = new Map<string, EdgesGeometry>();
    for (const [meshId, geometry] of geometries) map.set(meshId, new EdgesGeometry(geometry));
    return map;
  }, [geometries]);
  const targets = selection.targets ?? { mesh: true, vertex: false, edge: false, face: false };
  const selectionMode = selection.selectionMode ?? selection.granularity ?? "mesh";
  const currentComponentIds = new Set(selection.componentIds ?? []);
  const mergedComponentIdsSet = mergedComponentIds ? new Set(mergedComponentIds) : null;
  // Still-selected (solid) = current ∩ merged when dragging; newly-added (preview tint) = merged − current.
  const selectedComponentIds = mergedComponentIdsSet ? new Set([...currentComponentIds].filter((id) => mergedComponentIdsSet.has(id))) : currentComponentIds;
  const previewComponentIds = mergedComponentIdsSet ? new Set([...mergedComponentIdsSet].filter((id) => !currentComponentIds.has(id))) : new Set<number>();
  const mergedInstanceIdsSet = mergedInstanceIds ? new Set(mergedInstanceIds) : null;
  const pickEnabled = !gumballDragActive && !onPaintAt && !blockPick && !mergedComponentIdsSet && !mergedInstanceIdsSet;
  const transformMode = selection.transformMode ?? "move";
  const gumballConfig = useMemo(() => gumballConfigForTransformMode(transformMode), [transformMode]);
  const paintMode = selection.interactionMode === "paint";

  const mergeMode = (event: { shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => componentMergeArg(marqueeModeFromModifiers(event));

  const paintFromHit = (objectId: string, mesh: WorldMeshData, event: ThreeEvent<PointerEvent> & { faceIndex?: number | null; uv?: { x: number; y: number } }) => {
    if (!onPaintAt) return;
    let u = event.uv?.x;
    let v = event.uv?.y;
    if (u == null || v == null) {
      if (event.faceIndex == null || !mesh.indices.length) return;
      const i0 = mesh.indices[event.faceIndex * 3] ?? 0;
      const i1 = mesh.indices[event.faceIndex * 3 + 1] ?? 0;
      const i2 = mesh.indices[event.faceIndex * 3 + 2] ?? 0;
      if (!mesh.uvs || mesh.uvs.length < 6) return;
      u = (mesh.uvs[i0 * 2]! + mesh.uvs[i1 * 2]! + mesh.uvs[i2 * 2]!) / 3;
      v = (mesh.uvs[i0 * 2 + 1]! + mesh.uvs[i1 * 2 + 1]! + mesh.uvs[i2 * 2 + 1]!) / 3;
    }
    onPaintAt(objectId, u, v);
  };

  return (
    <WorldLayerStack>
      <group>
        {instances.map((instance, index) => {
          const meshId = instance.meshId ?? instance.id;
          const meshRecord = meshById.get(meshId);
          const meshData = meshRecord?.data;
          const geometry = geometries.get(meshId);
          const position = instance.position ?? [instance.x ?? index, instance.y ?? 0, instance.z ?? 0];
          const scale = instance.scale ?? [1, 1, 1];
          const rotation = instance.rotation;
          const quaternion = rotation ? new Quaternion(rotation[0], rotation[1], rotation[2], rotation[3]) : undefined;
          const previewInstanceSelected = mergedInstanceIdsSet ? mergedInstanceIdsSet.has(instance.id) : undefined;
          return (
            <WorldInstanceNode
              key={instance.id}
              instance={instance}
              previewInstanceSelected={previewInstanceSelected}
              index={index}
              meshRecord={meshRecord}
              meshData={meshData}
              geometry={geometry}
              borderGeometry={borderGeometries.get(meshId)}
              palette={palette}
              vertexPick={meshData ? buildVertexPickData(meshData) : null}
              edgeGeometry={meshData ? buildEdgeGeometry(meshData) : null}
              paintTextureBase64={meshData?.paintTextureBase64}
              position={position as [number, number, number]}
              scale={scale as [number, number, number]}
              quaternion={quaternion}
              targets={targets}
              activeObjectId={selection.activeObjectId}
              selectionMode={selectionMode}
              selectedComponentIds={selectedComponentIds}
              previewComponentIds={previewComponentIds}
              hoveredComponent={selection.hoveredComponent}
              showEdges={selection.showEdges}
              pickEnabled={pickEnabled}
              onPaintAt={onPaintAt}
              paintFromHit={paintFromHit}
              flatShading={instance.smoothShading === false}
              onInstancePointerDown={onInstancePointerDown}
              onInstancePointerMove={onInstancePointerMove}
              onWorldPick={onWorldPick}
              onComponentHover={onComponentHover}
              mergeMode={mergeMode}
              faceDragActive={selection.faceDragActive === true}
              onFaceDragStart={onFaceDragStart}
              environmentMaterial={environment?.material}
              environmentShadowEnabled={environment?.shadow?.enabled === true}
            />
          );
        })}
      </group>
      <SceneGumball target={selection.gumballTarget} config={gumballConfig} active={Boolean(selection.gumballActive) && !paintMode} onDraggingChanged={onGumballDraggingChanged} onDragEnd={onGumballDragEnd} />
    </WorldLayerStack>
  );
}
//#endregion WorldInstancesLayer

//#region WorldPointCloudLayer
/** ☁️ Renders `World3dScene.pointsJson` layers as GPU point sprites — decodes each layer's base64
 * position/color buffers into a `BufferGeometry` and draws it with a `PointsMaterial`, mounted
 * alongside `WorldTerrainLayer` in the `World3dHost` scene tree. */
type WorldPointCloudLayerVisual = { readonly geometry: BufferGeometry; readonly material: PointsMaterial };

function pointCloudLayerVisual(layer: WorldPointCloudLayerRecord): WorldPointCloudLayerVisual {
  const geometry = new BufferGeometry();
  const positionBytes = base64ToBytes(layer.positionsB64);
  const positions = new Float32Array(positionBytes.buffer, positionBytes.byteOffset, positionBytes.byteLength / Float32Array.BYTES_PER_ELEMENT);
  geometry.setAttribute("position", new BufferAttribute(positions, 3));
  const hasColors = Boolean(layer.colorsB64);
  if (layer.colorsB64) geometry.setAttribute("color", new BufferAttribute(base64ToBytes(layer.colorsB64), 3, true));
  const material = new PointsMaterial({ size: layer.size, sizeAttenuation: layer.sizeAttenuation, vertexColors: hasColors });
  return { geometry, material };
}

function WorldPointCloudLayer({ pointsJson }: { readonly pointsJson: string | undefined }) {
  const layers = useMemo(() => parseJsonArray<WorldPointCloudLayerRecord>(pointsJson), [pointsJson]);
  const visuals = useMemo(() => {
    const map = new Map<string, WorldPointCloudLayerVisual>();
    for (const layer of layers) map.set(layer.id, pointCloudLayerVisual(layer));
    return map;
  }, [layers]);

  useEffect(() => {
    return () => {
      for (const visual of visuals.values()) {
        visual.geometry.dispose();
        visual.material.dispose();
      }
    };
  }, [visuals]);

  if (layers.length === 0) return null;

  return (
    <group>
      {layers.map((layer) => {
        const visual = visuals.get(layer.id);
        if (!visual) return null;
        return <points key={layer.id} geometry={visual.geometry} material={visual.material} />;
      })}
    </group>
  );
}
//#endregion WorldPointCloudLayer

function WorldVortexMarkers({
  vortices,
  palette,
  brushMode,
  selectionMode,
  connectSourceFullId,
  onHover,
  onVortexSelect,
  onBrushPlace,
  onVortexPointerArm,
  onVortexPointerMove,
  onVortexPointerUp,
  onConnectDragHover,
  onConnectDragDrop,
}: {
  readonly vortices: readonly WorldVortexRecord[];
  readonly palette: MeshStylePalette;
  readonly brushMode: boolean;
  readonly selectionMode?: string;
  readonly connectSourceFullId?: string;
  readonly onHover: (fullId: string | null) => void;
  readonly onVortexSelect: (fullId: string, event?: { shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => void;
  readonly onBrushPlace: () => void;
  readonly onVortexPointerArm: (args: {
    readonly fullId: string;
    readonly position: readonly [number, number, number];
    readonly clientX: number;
    readonly clientY: number;
    readonly event: { readonly shiftKey?: boolean; readonly ctrlKey?: boolean; readonly metaKey?: boolean };
  }) => void;
  readonly onVortexPointerMove: (fullId: string, clientX: number, clientY: number) => void;
  readonly onVortexPointerUp: (fullId: string, event?: { shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => void;
  readonly onConnectDragHover: (position: readonly [number, number, number]) => void;
  readonly onConnectDragDrop: (fullId: string, event?: { shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => void;
}) {
  if (!vortices.length) return null;
  return (
    <group>
      {vortices.map((vortex) => {
        const radius = vortex.radius ?? 0.36;
        const isConnectSource = connectSourceFullId === vortex.fullId;
        const style = vortex.selected ? palette.selected : vortex.hovered ? palette.hovered : null;
        const color = isConnectSource ? "#f59e0b" : (style?.meshColor ?? vortex.color ?? "#38bdf8");
        return (
          <mesh
            key={vortex.fullId}
            position={vortex.position as [number, number, number]}
            onPointerOver={(event) => {
              event.stopPropagation();
              onHover(vortex.fullId);
              if (connectSourceFullId) onConnectDragHover(vortex.position);
            }}
            onPointerOut={(event) => {
              event.stopPropagation();
              onHover(null);
            }}
            onPointerDown={(event) => {
              event.stopPropagation();
              if (resolveVortexPointerDownIntent(brushMode, selectionMode) === "select") {
                onVortexSelect(vortex.fullId, event);
                return;
              }
              onVortexPointerArm({
                fullId: vortex.fullId,
                position: vortex.position,
                clientX: event.clientX,
                clientY: event.clientY,
                event,
              });
            }}
            onPointerMove={(event) => {
              if (brushMode) return;
              event.stopPropagation();
              onVortexPointerMove(vortex.fullId, event.clientX, event.clientY);
              if (connectSourceFullId) onConnectDragHover(vortex.position);
            }}
            onPointerUp={(event) => {
              if (brushMode) return;
              if (connectSourceFullId) {
                event.stopPropagation();
                onConnectDragDrop(vortex.fullId, event);
                return;
              }
              event.stopPropagation();
              onVortexPointerUp(vortex.fullId, event);
            }}
            onClick={(event) => {
              event.stopPropagation();
              if (brushMode) onBrushPlace();
            }}
          >
            <sphereGeometry args={[radius, 16, 16]} />
            <meshStandardMaterial color={color} emissive={style?.meshColor ?? "#000000"} emissiveIntensity={style?.emissiveIntensity ?? 0} transparent opacity={0.88} />
          </mesh>
        );
      })}
    </group>
  );
}

/** @emoji 🧲 Rubber-band line drawn from the drag-connect source vortex to the currently hovered vortex (or itself, if hovering nothing). */
function WorldConnectRubberBand({ from, to }: { readonly from: readonly [number, number, number]; readonly to: readonly [number, number, number] }) {
  const geometry = useMemo(() => {
    const positions = new Float32Array([from[0], from[1], from[2], to[0], to[1], to[2]]);
    const geom = new BufferGeometry();
    geom.setAttribute("position", new BufferAttribute(positions, 3));
    return geom;
  }, [from, to]);
  return (
    <lineSegments geometry={geometry} raycast={() => null}>
      <lineBasicMaterial color="#f59e0b" linewidth={2} />
    </lineSegments>
  );
}

/** @emoji 🧊 Invisible ground plane (Z-up XY plane, matching this world's up axis) that tracks the grid-snapped cursor while voxel-editing target volumes; Alt+click commits a volume there. */
function WorldVoxelGroundPlane({ gridFactor, onHover, onPlace }: { readonly gridFactor: number; readonly onHover: (origin: readonly [number, number, number] | null) => void; readonly onPlace: (origin: readonly [number, number, number]) => void }) {
  const snap = (value: number) => Math.round(value / gridFactor) * gridFactor;
  return (
    <mesh
      onPointerMove={(event) => {
        event.stopPropagation();
        onHover([snap(event.point.x), snap(event.point.y), snap(event.point.z)]);
      }}
      onPointerOut={(event) => {
        event.stopPropagation();
        onHover(null);
      }}
      onClick={(event) => {
        event.stopPropagation();
        if (!event.nativeEvent.altKey) return;
        onPlace([snap(event.point.x), snap(event.point.y), snap(event.point.z)]);
      }}
    >
      <planeGeometry args={[10000, 10000]} />
      <meshBasicMaterial visible={false} />
    </mesh>
  );
}

/** @emoji 🧊 Cursor-follow ghost box previewing the target volume that Alt+click would place, sized by the engagement's W/D/H steppers. */
function WorldVoxelPreviewBox({ origin, dims, gridFactor }: { readonly origin: readonly [number, number, number]; readonly dims: readonly [number, number, number]; readonly gridFactor: number }) {
  return (
    <mesh position={origin as [number, number, number]} raycast={() => null}>
      <boxGeometry args={[dims[0] * gridFactor, dims[1] * gridFactor, dims[2] * gridFactor]} />
      <meshStandardMaterial color="#38bdf8" transparent opacity={0.48} />
    </mesh>
  );
}

function WorldAttractionLines({ attractions }: { readonly attractions: readonly WorldAttractionRecord[] }) {
  if (!attractions.length) return null;
  return (
    <group>
      {attractions.map((attraction) => {
        const positions = new Float32Array([attraction.from[0], attraction.from[1], attraction.from[2], attraction.to[0], attraction.to[1], attraction.to[2]]);
        const geometry = new BufferGeometry();
        geometry.setAttribute("position", new BufferAttribute(positions, 3));
        return (
          <lineSegments key={attraction.id} geometry={geometry} raycast={() => null}>
            <lineBasicMaterial color={attraction.color ?? "#60a5fa"} linewidth={2} />
          </lineSegments>
        );
      })}
    </group>
  );
}

function BrushPreviewGhost({ preview, meshes, palette }: { readonly preview: WorldBrushPreviewRecord; readonly meshes: readonly WorldMeshRecord[]; readonly palette: MeshStylePalette }) {
  if (!preview.origin) return null;
  const style = palette.highlighted;
  const meshUrl = preview.meshUrl;
  const meshRecord = meshUrl ? meshes.find((mesh) => mesh.url === meshUrl) : undefined;
  const position = preview.origin as [number, number, number];
  const rotation = preview.orientation as [number, number, number, number] | undefined;
  const scale = scaleTuple(preview.scale);
  const quaternion = rotation ? new Quaternion(rotation[0], rotation[1], rotation[2], rotation[3]) : undefined;
  return (
    <group position={position} scale={scale} quaternion={quaternion}>
      {meshRecord?.url ? (
        <Suspense fallback={null}>
          <GlbInstanceMesh url={meshRecord.url} color={style.meshColor} emissive={style.meshColor} emissiveIntensity={0.6} opacity={1} borderColor={palette.neutral.lineColor} />
        </Suspense>
      ) : (
        <mesh raycast={() => null}>
          <boxGeometry args={[1, 1, 1]} />
          <meshBasicMaterial color={style.meshColor} transparent opacity={0.42} depthWrite={false} />
        </mesh>
      )}
    </group>
  );
}

function EngagementPreviewLayer({ items, color }: { readonly items: readonly WorldEngagementPreviewItem[]; readonly color: string }) {
  if (!items.length) return null;
  return (
    <group>
      {items.map((item, index) => {
        if (item.kind === "point") {
          return (
            <mesh key={`preview-point-${index}`} position={item.position as [number, number, number]} raycast={() => null}>
              <sphereGeometry args={[0.08, 12, 12]} />
              <meshStandardMaterial color={color} />
            </mesh>
          );
        }
        if (item.kind === "segment") {
          const positions = new Float32Array([item.from[0], item.from[1], item.from[2], item.to[0], item.to[1], item.to[2]]);
          const geometry = new BufferGeometry();
          geometry.setAttribute("position", new BufferAttribute(positions, 3));
          return (
            <lineSegments key={`preview-segment-${index}`} geometry={geometry} raycast={() => null}>
              <lineBasicMaterial color={color} linewidth={2} />
            </lineSegments>
          );
        }
        if (item.kind === "box-preview" && item.cornerA && item.cornerB) {
          const [ax, ay, az] = item.cornerA;
          const [bx, by] = item.cornerB;
          const width = Math.max(Math.abs(bx - ax), 0.05);
          const depth = Math.max(Math.abs(by - ay), 0.05);
          // `height` is a separate vertical extrusion from the footprint plane (az), not derived
          // from cornerB's z — the interaction specs author cornerA/cornerB as ground-plane points.
          const height = Math.max(Math.abs(item.height ?? 0.05), 0.05);
          return (
            <mesh key={`preview-box-${index}`} position={[(ax + bx) * 0.5, (ay + by) * 0.5, az + height * 0.5]} raycast={() => null}>
              <boxGeometry args={[width, depth, height]} />
              <meshBasicMaterial color={color} transparent opacity={0.35} depthWrite={false} wireframe />
            </mesh>
          );
        }
        if (item.kind === "linear-handle") {
          const [ox, oy, oz] = item.origin;
          const [dx, dy, dz] = item.axis;
          const length = Math.max(Math.hypot(dx, dy, dz), 0.05);
          const direction = new Vector3(dx, dy, dz).normalize();
          const quaternion = new Quaternion().setFromUnitVectors(new Vector3(0, 1, 0), direction);
          return (
            <mesh key={`preview-handle-${index}`} position={[ox + dx * 0.5, oy + dy * 0.5, oz + dz * 0.5]} quaternion={quaternion} raycast={() => null}>
              <cylinderGeometry args={[0.02, 0.02, length, 8]} />
              <meshBasicMaterial color={color} transparent opacity={0.6} depthWrite={false} />
            </mesh>
          );
        }
        return null;
      })}
    </group>
  );
}

/** @emoji 🧭 Floating per-vortex brush-candidate popup opened by Alt+right-click or the context menu's "Suggest objects" — hovering a row previews it as the brush ghost, clicking places it. */
function WorldSuggestionMenu({
  menu,
  activeIndex,
  onHoverCandidate,
  onAcceptCandidate,
  onClose,
}: {
  readonly menu: WorldSuggestionMenuRecord;
  readonly activeIndex: number;
  readonly onHoverCandidate: (index: number) => void;
  readonly onAcceptCandidate: (index: number) => void;
  readonly onClose: () => void;
}) {
  const rootRef = useRef<HTMLDivElement | null>(null);
  const checkingPlacementLabel = useLabel("ui.host.checkingPlacement");
  const noPlacementLabel = useLabel("ui.host.noPlacement");
  useEffect(() => {
    const handlePointerDown = (event: PointerEvent) => {
      if (rootRef.current && !rootRef.current.contains(event.target as Node)) onClose();
    };
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape") onClose();
    };
    window.addEventListener("pointerdown", handlePointerDown, true);
    window.addEventListener("keydown", handleKeyDown);
    return () => {
      window.removeEventListener("pointerdown", handlePointerDown, true);
      window.removeEventListener("keydown", handleKeyDown);
    };
  }, [onClose]);
  return (
    <div
      ref={rootRef}
      className="semio-world-suggestion-menu"
      style={{
        position: "absolute",
        left: menu.x,
        top: menu.y,
        zIndex: 50,
        minWidth: "12rem",
        borderRadius: "0.375rem",
        border: "1px solid var(--border-normal-color)",
        background: "var(--panel)",
        padding: "0.25rem 0",
        boxShadow: "0 4px 16px rgba(0, 0, 0, 0.24)",
      }}
    >
      {menu.pending ? (
        <div style={{ padding: "0.375rem 0.75rem", fontSize: "0.8125rem", opacity: 0.7 }}>{checkingPlacementLabel}</div>
      ) : menu.candidates.length === 0 ? (
        <div style={{ padding: "0.375rem 0.75rem", fontSize: "0.8125rem", opacity: 0.7 }}>{noPlacementLabel}</div>
      ) : (
        menu.candidates.map((candidate) => (
          <div
            key={candidate.index}
            className={menuListItemClassName}
            data-selected={candidate.index === activeIndex}
            style={{ padding: "0.375rem 0.75rem", fontSize: "0.8125rem", cursor: "pointer" }}
            onMouseEnter={() => onHoverCandidate(candidate.index)}
            onClick={() => onAcceptCandidate(candidate.index)}
          >
            {candidate.objectLabel} · {candidate.vortexLabel}
          </div>
        ))
      )}
    </div>
  );
}

const MARQUEE_DRAG_THRESHOLD_PX = 4;

/** @emoji 🎯 Generic add/remove/toggle/replace merge, mirrors `selectionMergeIds` from `@semio-tech/ui-react` for non-string id sets. */
function mergeIdSet<T>(mode: ReturnType<typeof marqueeModeFromModifiers>, current: readonly T[], incoming: readonly T[]): T[] {
  const currentSet = new Set(current);
  const incomingSet = new Set(incoming);
  if (mode === "default") return [...incomingSet];
  if (mode === "additive") {
    for (const id of incomingSet) currentSet.add(id);
    return [...currentSet];
  }
  if (mode === "subtractive") {
    for (const id of incomingSet) currentSet.delete(id);
    return [...currentSet];
  }
  for (const id of incomingSet) {
    if (currentSet.has(id)) currentSet.delete(id);
    else currentSet.add(id);
  }
  return [...currentSet];
}

/** @emoji 🖱️ additive→add, subtractive→remove, invertive→toggle, default→replace (whole-instance picks/marquee). */
function instanceMergeArg(mode: ReturnType<typeof marqueeModeFromModifiers>): string {
  if (mode === "additive") return "add";
  if (mode === "subtractive") return "remove";
  if (mode === "invertive") return "toggle";
  return "replace";
}

/** @emoji 🖱️ Same as {@link instanceMergeArg} but a bare click (no modifiers) defaults to invertive. */
function componentMergeArg(mode: ReturnType<typeof marqueeModeFromModifiers>): string {
  if (mode === "additive") return "add";
  if (mode === "subtractive") return "remove";
  return "toggle";
}

function pointInMarqueeRect(sx: number, sy: number, marquee: readonly SelectionMarqueePoint[]): boolean {
  if (marquee.length < 2) return false;
  const start = marquee[0]!;
  const end = marquee[marquee.length - 1]!;
  const minX = Math.min(start.x, end.x);
  const maxX = Math.max(start.x, end.x);
  const minY = Math.min(start.y, end.y);
  const maxY = Math.max(start.y, end.y);
  return sx >= minX && sx <= maxX && sy >= minY && sy <= maxY;
}

/** @emoji 🎯 Even-odd point-in-polygon test for lasso selection. */
function pointInPolygon(sx: number, sy: number, polygon: readonly SelectionMarqueePoint[]): boolean {
  let inside = false;
  for (let i = 0, j = polygon.length - 1; i < polygon.length; j = i++) {
    const a = polygon[i]!;
    const b = polygon[j]!;
    const intersects = a.y > sy !== b.y > sy && sx < ((b.x - a.x) * (sy - a.y)) / (b.y - a.y) + a.x;
    if (intersects) inside = !inside;
  }
  return inside;
}

function pointInMarqueeRegion(sx: number, sy: number, method: SelectionMarqueeMethod, marquee: readonly SelectionMarqueePoint[]): boolean {
  if (method === "lasso" && marquee.length >= 3) return pointInPolygon(sx, sy, marquee);
  return pointInMarqueeRect(sx, sy, marquee);
}

/** @emoji 🎯 Window (full containment, all points) vs crossing (partial, any point) semantics for multi-point elements. */
function pointsSatisfyMarquee(points: readonly (readonly [number, number])[], method: SelectionMarqueeMethod, marquee: readonly SelectionMarqueePoint[], coverage: SelectionMarqueeCoverage): boolean {
  if (points.length === 0) return false;
  const test = (point: readonly [number, number]) => pointInMarqueeRegion(point[0], point[1], method, marquee);
  return coverage === "full" ? points.every(test) : points.some(test);
}

function projectWorldPoint(point: readonly [number, number, number], offset: readonly [number, number, number], camera: import("three").Camera, rect: DOMRect): { readonly x: number; readonly y: number } {
  const projected = new Vector3(point[0] + offset[0], point[1] + offset[1], point[2] + offset[2]).project(camera);
  return {
    x: ((projected.x + 1) / 2) * rect.width,
    y: ((-projected.y + 1) / 2) * rect.height,
  };
}

function resolveMarqueeComponentIds(
  instances: readonly WorldInstanceRecord[],
  meshes: readonly WorldMeshRecord[],
  selectionMode: string,
  activeObjectId: string | undefined,
  marquee: readonly SelectionMarqueePoint[],
  rect: DOMRect,
  camera: import("three").Camera,
  method: SelectionMarqueeMethod,
  coverage: SelectionMarqueeCoverage,
): readonly number[] {
  const active = instances.find((instance) => instance.id === activeObjectId);
  if (!active) return [];
  const meshId = active.meshId ?? active.id;
  const meshData = meshes.find((mesh) => mesh.id === meshId)?.data;
  if (!meshData) return [];
  const offset = (active.position ?? [0, 0, 0]) as [number, number, number];
  const project = (point: readonly [number, number, number]): readonly [number, number] => {
    const screen = projectWorldPoint(point, offset, camera, rect);
    return [screen.x, screen.y];
  };
  const hits = new Set<number>();
  if (selectionMode === "vertex") {
    const pick = buildVertexPickData(meshData);
    if (!pick) return [];
    const positions = pick.geometry.attributes.position!;
    for (let index = 0; index < pick.vertexIds.length; index += 1) {
      const point = project([positions.getX(index), positions.getY(index), positions.getZ(index)]);
      if (pointsSatisfyMarquee([point], method, marquee, coverage)) hits.add(pick.vertexIds[index]!);
    }
  } else if (selectionMode === "edge" && meshData.edgeIds && meshData.edgePositions) {
    for (let edgeIndex = 0; edgeIndex < meshData.edgeIds.length; edgeIndex += 1) {
      const base = edgeIndex * 6;
      const a = project([meshData.edgePositions[base]!, meshData.edgePositions[base + 1]!, meshData.edgePositions[base + 2]!]);
      const b = project([meshData.edgePositions[base + 3]!, meshData.edgePositions[base + 4]!, meshData.edgePositions[base + 5]!]);
      if (pointsSatisfyMarquee([a, b], method, marquee, coverage)) hits.add(meshData.edgeIds[edgeIndex]!);
    }
  } else if (selectionMode === "face" && meshData.faceIds && meshData.indices.length) {
    for (let faceIndex = 0; faceIndex < meshData.faceIds.length; faceIndex += 1) {
      const i0 = meshData.indices[faceIndex * 3] ?? 0;
      const i1 = meshData.indices[faceIndex * 3 + 1] ?? 0;
      const i2 = meshData.indices[faceIndex * 3 + 2] ?? 0;
      const p0 = project([meshData.positions[i0 * 3]!, meshData.positions[i0 * 3 + 1]!, meshData.positions[i0 * 3 + 2]!]);
      const p1 = project([meshData.positions[i1 * 3]!, meshData.positions[i1 * 3 + 1]!, meshData.positions[i1 * 3 + 2]!]);
      const p2 = project([meshData.positions[i2 * 3]!, meshData.positions[i2 * 3 + 1]!, meshData.positions[i2 * 3 + 2]!]);
      if (pointsSatisfyMarquee([p0, p1, p2], method, marquee, coverage)) hits.add(meshData.faceIds[faceIndex]!);
    }
  }
  return [...hits];
}

/** @emoji 📦 Local-space AABB corners of a mesh's vertex positions (fallback: origin only). */
function meshBoundsCorners(meshData: WorldMeshData): readonly (readonly [number, number, number])[] {
  let minX = Infinity;
  let minY = Infinity;
  let minZ = Infinity;
  let maxX = -Infinity;
  let maxY = -Infinity;
  let maxZ = -Infinity;
  for (let index = 0; index < meshData.positions.length; index += 3) {
    const x = meshData.positions[index]!;
    const y = meshData.positions[index + 1]!;
    const z = meshData.positions[index + 2]!;
    if (x < minX) minX = x;
    if (x > maxX) maxX = x;
    if (y < minY) minY = y;
    if (y > maxY) maxY = y;
    if (z < minZ) minZ = z;
    if (z > maxZ) maxZ = z;
  }
  if (!Number.isFinite(minX)) return [[0, 0, 0]];
  return [
    [minX, minY, minZ],
    [maxX, minY, minZ],
    [minX, maxY, minZ],
    [maxX, maxY, minZ],
    [minX, minY, maxZ],
    [maxX, minY, maxZ],
    [minX, maxY, maxZ],
    [maxX, maxY, maxZ],
  ];
}

function resolveMarqueeInstanceIds(
  instances: readonly WorldInstanceRecord[],
  meshes: readonly WorldMeshRecord[],
  marquee: readonly SelectionMarqueePoint[],
  rect: DOMRect,
  camera: import("three").Camera,
  method: SelectionMarqueeMethod,
  coverage: SelectionMarqueeCoverage,
): readonly string[] {
  const meshById = new Map(meshes.map((mesh) => [mesh.id, mesh]));
  const hits: string[] = [];
  instances.forEach((instance, index) => {
    const meshId = instance.meshId ?? instance.id;
    const meshData = meshById.get(meshId)?.data;
    const position = (instance.position ?? [instance.x ?? index, instance.y ?? 0, instance.z ?? 0]) as [number, number, number];
    const scale = (instance.scale ?? [1, 1, 1]) as [number, number, number];
    const rotation = instance.rotation;
    const quaternion = rotation ? new Quaternion(rotation[0], rotation[1], rotation[2], rotation[3]) : undefined;
    const localCorners = meshData ? meshBoundsCorners(meshData) : [[0, 0, 0] as const];
    const worldCorners = localCorners.map((corner) => {
      const v = new Vector3(corner[0] * scale[0], corner[1] * scale[1], corner[2] * scale[2]);
      if (quaternion) v.applyQuaternion(quaternion);
      v.add(new Vector3(position[0], position[1], position[2]));
      return [v.x, v.y, v.z] as const;
    });
    const points = worldCorners.map((corner) => {
      const screen = projectWorldPoint(corner, [0, 0, 0], camera, rect);
      return [screen.x, screen.y] as const;
    });
    if (pointsSatisfyMarquee(points, method, marquee, coverage)) hits.push(instance.id);
  });
  return hits;
}

function CameraRefBridge({ cameraRef }: { readonly cameraRef: React.MutableRefObject<import("three").Camera | null> }) {
  const camera = useThree((state) => state.camera);
  useEffect(() => {
    cameraRef.current = camera;
  }, [camera, cameraRef]);
  return null;
}

/** @emoji 🎯 Widens Line/Points raycast hit area so thin edge/vertex geometry is reliably pickable without stealing face clicks. */
function RaycasterPickTuning() {
  const raycaster = useThree((state) => state.raycaster);
  useEffect(() => {
    raycaster.params.Line = { threshold: 0.05 };
    raycaster.params.Points = { threshold: 0.05 };
  }, [raycaster]);
  return null;
}

function paneSuffixFromSurfaceId(surfaceId?: string): string | undefined {
  if (!surfaceId) return undefined;
  const slash = surfaceId.lastIndexOf("/");
  return slash >= 0 ? surfaceId.slice(slash + 1) : surfaceId;
}

function raycastGroundPoint(clientX: number, clientY: number, hostRect: DOMRect, camera: import("three").Camera): [number, number, number] | null {
  const ndcX = ((clientX - hostRect.left) / hostRect.width) * 2 - 1;
  const ndcY = -(((clientY - hostRect.top) / hostRect.height) * 2 - 1);
  const ray = new Vector3(ndcX, ndcY, 0.5).unproject(camera);
  const origin = camera.position.clone();
  const direction = ray.sub(origin).normalize();
  if (Math.abs(direction.z) < 1e-6) return null;
  const t = -origin.z / direction.z;
  if (t < 0) return null;
  const hit = origin.add(direction.multiplyScalar(t));
  return [hit.x, hit.y, hit.z];
}

//#region CatalogueDrop
type Puzzle3dCatalogueDropPayload = {
  readonly objectKind: string;
  readonly meshUrl?: string;
};

type Puzzle3dCatalogueDropPreview = Puzzle3dCatalogueDropPayload & {
  readonly origin: readonly [number, number, number];
};

export function parsePuzzle3dCatalogueDragPayload(encoded: string | null | undefined): Puzzle3dCatalogueDropPayload | null {
  if (!encoded) return null;
  try {
    const parsed = JSON.parse(encoded) as Partial<Puzzle3dCatalogueDropPayload>;
    if (typeof parsed.objectKind !== "string" || !parsed.objectKind) return null;
    return {
      objectKind: parsed.objectKind,
      meshUrl: typeof parsed.meshUrl === "string" && parsed.meshUrl ? parsed.meshUrl : undefined,
    };
  } catch {
    return null;
  }
}

export function snapWorldPointToGrid(point: readonly [number, number, number], gridSnapEnabled: boolean, gridFactor: number): [number, number, number] {
  if (!gridSnapEnabled || gridFactor <= 0) return [point[0], point[1], point[2]];
  const snap = (value: number) => Math.round(value / gridFactor) * gridFactor;
  return [snap(point[0]), snap(point[1]), snap(point[2])];
}

function resolveCatalogueDropOrigin(clientX: number, clientY: number, hostRect: DOMRect, camera: import("three").Camera | null, gridSnapEnabled: boolean, gridFactor: number): [number, number, number] | null {
  if (!camera) return null;
  const hit = raycastGroundPoint(clientX, clientY, hostRect, camera);
  if (!hit) return null;
  return snapWorldPointToGrid(hit, gridSnapEnabled, gridFactor);
}

function clientPointOverHost(clientX: number, clientY: number, hostRect: DOMRect): boolean {
  return clientX >= hostRect.left && clientX <= hostRect.right && clientY >= hostRect.top && clientY <= hostRect.bottom;
}

function CatalogueDropGhost({ preview, meshes, palette }: { readonly preview: Puzzle3dCatalogueDropPreview; readonly meshes: readonly WorldMeshRecord[]; readonly palette: MeshStylePalette }) {
  const style = palette.highlighted;
  const meshRecord = preview.meshUrl ? meshes.find((mesh) => mesh.url === preview.meshUrl) : undefined;
  const url = meshRecord?.url ?? preview.meshUrl;
  return (
    <group position={preview.origin as [number, number, number]} raycast={() => null}>
      {url ? (
        <Suspense fallback={null}>
          <GlbInstanceMesh url={url} color={style.meshColor} emissive={style.meshColor} emissiveIntensity={0.6} opacity={0.88} borderColor={palette.neutral.lineColor} />
        </Suspense>
      ) : (
        <mesh raycast={() => null}>
          <boxGeometry args={[1, 1, 1]} />
          <meshBasicMaterial color={style.meshColor} transparent opacity={0.42} depthWrite={false} />
        </mesh>
      )}
    </group>
  );
}
//#endregion CatalogueDrop

/** @emoji 🖱️➡️ Signed distance along `axis` (unit vector) from `origin` to the point on that line closest to the
 * camera ray through the current pointer position — the standard closest-point-between-two-lines
 * construction, used so a face-normal drag tracks naturally instead of needing a ground/tangent-plane
 * intersection (which is undefined for motion parallel to the plane, i.e. exactly along the normal). */
function axisDragParam(clientX: number, clientY: number, hostRect: DOMRect, camera: import("three").Camera, origin: readonly [number, number, number], axis: readonly [number, number, number]): number | null {
  const ndcX = ((clientX - hostRect.left) / hostRect.width) * 2 - 1;
  const ndcY = -(((clientY - hostRect.top) / hostRect.height) * 2 - 1);
  const rayOrigin = camera.position.clone();
  const rayDirection = new Vector3(ndcX, ndcY, 0.5).unproject(camera).sub(rayOrigin).normalize();
  const axisOrigin = new Vector3(origin[0], origin[1], origin[2]);
  const axisDirection = new Vector3(axis[0], axis[1], axis[2]).normalize();
  const originDelta = rayOrigin.clone().sub(axisOrigin);
  const a = rayDirection.dot(rayDirection);
  const b = rayDirection.dot(axisDirection);
  const c = axisDirection.dot(axisDirection);
  const d = rayDirection.dot(originDelta);
  const e = axisDirection.dot(originDelta);
  const denominator = a * c - b * b;
  if (Math.abs(denominator) < 1e-9) return null;
  return (a * e - b * d) / denominator;
}

//#region World3dHost
export function World3dHost({ node, onAction }: ComponentSceneHostProps) {
  const scene = node.world3d;
  const emptySceneLabel = useLabel("ui.host.emptyScene");
  const meshStylePalette = useMeshStylePalette();
  const colors = useMemo(() => semanticColorsFromPalette(meshStylePalette), [meshStylePalette]);
  const parsedCamera = useMemo(() => parseCameraState(scene?.cameraJson ?? "{}"), [scene?.cameraJson]);
  const instances = useMemo(() => parseInstances(scene?.instancesJson ?? "[]"), [scene?.instancesJson]);
  const cameraState = useMemo(() => {
    if ((scene?.cameraJson ?? "").includes('"position"')) return parsedCamera;
    return instances.length > 0 ? autofitCameraFromInstances(instances) : parsedCamera;
  }, [instances, parsedCamera, scene?.cameraJson]);
  const meshes = useMemo(() => parseMeshes(scene?.meshesJson ?? "[]"), [scene?.meshesJson]);
  const selection = useMemo(() => parseSelection(scene?.selectionJson ?? "{}"), [scene?.selectionJson]);
  const vortices = useMemo(() => parseJsonArray<WorldVortexRecord>(scene?.vorticesJson), [scene?.vorticesJson]);
  const attractions = useMemo(() => parseJsonArray<WorldAttractionRecord>(scene?.attractionsJson), [scene?.attractionsJson]);
  const targetVolumes = useMemo(() => parseJsonArray<WorldTargetVolumeRecord>(scene?.targetVolumesJson), [scene?.targetVolumesJson]);
  const references = useMemo(() => parseJsonArray<WorldReferenceRecord>(scene?.referencesJson), [scene?.referencesJson]);
  const interaction = useMemo(() => parseInteraction(scene?.interactionJson), [scene?.interactionJson]);
  const lod = useMemo(() => parseLod(scene?.lodJson), [scene?.lodJson]);
  const engagementPreview = useMemo(() => parseEngagementPreview(scene?.engagementPreviewJson), [scene?.engagementPreviewJson]);
  const brushPreview = useMemo(() => parseBrushPreview(scene?.brushPreviewJson), [scene?.brushPreviewJson]);
  const contextMenuItems = useMemo(() => parseJsonArray<WorldContextMenuItem>(scene?.contextMenuJson), [scene?.contextMenuJson]);
  const environment = useMemo(() => parseEnvironment(scene?.environmentJson), [scene?.environmentJson]);
  const frame = useMemo(() => parseFrame(scene?.frameJson), [scene?.frameJson]);
  const fit = useMemo(() => parseFit(scene?.fitJson), [scene?.fitJson]);
  const activeUtility = interaction.activeUtility ?? "select";
  const fillMode = activeUtility === "fill";
  const brushMode = activeUtility === "brush";
  const hostRef = useRef<HTMLDivElement | null>(null);
  const instancesGroupRef = useRef<Group | null>(null);
  const lodRef = useRef(DEFAULT_MANUAL_LOD);
  const [marqueePath, setMarqueePath] = useState<readonly SelectionMarqueePoint[]>([]);
  const [marqueeModifiers, setMarqueeModifiers] = useState<{ readonly shiftKey: boolean; readonly ctrlKey: boolean; readonly metaKey: boolean }>({ shiftKey: false, ctrlKey: false, metaKey: false });
  const [gumballDragActive, setGumballDragActive] = useState(false);
  const [faceDragSession, setFaceDragSession] = useState<{
    readonly objectId: string;
    readonly faceId: number;
    readonly normal: readonly [number, number, number];
    readonly startPoint: readonly [number, number, number];
    readonly faceExtent?: readonly [number, number];
  } | null>(null);
  const [connectDragSource, setConnectDragSource] = useState<{ readonly fullId: string; readonly position: readonly [number, number, number] } | null>(null);
  const [connectDragHoverPosition, setConnectDragHoverPosition] = useState<readonly [number, number, number] | null>(null);
  const [vortexPointerArm, setVortexPointerArm] = useState<{
    readonly fullId: string;
    readonly position: readonly [number, number, number];
    readonly clientX: number;
    readonly clientY: number;
    readonly event: { readonly shiftKey?: boolean; readonly ctrlKey?: boolean; readonly metaKey?: boolean };
  } | null>(null);
  const [voxelHoverOrigin, setVoxelHoverOrigin] = useState<readonly [number, number, number] | null>(null);
  const [paintStrokeActive, setPaintStrokeActive] = useState(false);
  const [catalogueDropPreview, setCatalogueDropPreview] = useState<Puzzle3dCatalogueDropPreview | null>(null);
  const [contextMenu, setContextMenu] = useState<{ readonly x: number; readonly y: number } | null>(null);
  const cameraRef = useRef<import("three").Camera | null>(null);
  const catalogueDragDepthRef = useRef(0);
  const catalogueDragEncodedRef = useRef<string | null>(null);
  const wasMarqueeDragRef = useRef(false);
  const connectDropConsumedRef = useRef(false);
  const engagementPointerMoveInFlightRef = useRef(false);
  const engagementPointerMoveLastPointRef = useRef<readonly [number, number, number] | null>(null);
  const selectionMode = selection.selectionMode ?? selection.granularity ?? "mesh";
  const gridSnapEnabled = lod.gridSnapEnabled ?? false;
  const gridFactor = lod.gridFactor ?? interaction.gridFactor ?? DEFAULT_LOD_GRID_FACTOR;
  const marqueeDown = marqueePath.length > 0;
  const method = selection.method ?? "rectangle";
  const marqueeStart = marqueePath[0];
  const marqueeEnd = marqueePath[marqueePath.length - 1];
  const marqueeDragActive = marqueeDown && marqueePath.length > 1 && marqueeStart != null && marqueeEnd != null && Math.hypot(marqueeEnd.x - marqueeStart.x, marqueeEnd.y - marqueeStart.y) > MARQUEE_DRAG_THRESHOLD_PX;
  const marqueeMergeMode = useMemo(() => marqueeModeFromModifiers(marqueeModifiers), [marqueeModifiers]);
  const marqueeCoverage: SelectionMarqueeCoverage = useMemo(() => {
    if (!marqueeDragActive || !marqueeStart || !marqueeEnd) return "full";
    return marqueeCoverageFromGesture({ method, startX: marqueeStart.x, endX: marqueeEnd.x, path: marqueePath });
  }, [marqueeDragActive, marqueeEnd, marqueePath, marqueeStart, method]);

  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      onAction({
        controllerId: node.controllerId,
        action,
        args: { surfaceId: node.surfaceId, ...args },
      });
    },
    [node.controllerId, node.surfaceId, onAction],
  );

  const referenceSelectedIds = useMemo(() => {
    if (!selection.referenceSelectedId) return new Set<string>();
    return new Set([selection.referenceSelectedId]);
  }, [selection.referenceSelectedId]);

  const referenceHoveredId = useMemo(() => {
    const hovered = selection.hoveredId;
    if (!hovered?.startsWith("reference:")) return null;
    return hovered.slice("reference:".length);
  }, [selection.hoveredId]);

  const handleReferenceSelect = useCallback(
    (id: string) => {
      dispatch("setReferenceSelection", {
        pane: paneSuffixFromSurfaceId(node.surfaceId),
        referenceId: id,
      });
    },
    [dispatch, node.surfaceId],
  );

  const handleReferenceHover = useCallback(
    (id: string | null) => {
      if (!id) {
        dispatch("referenceHover", {});
        return;
      }
      dispatch("referenceHover", { referenceId: id });
    },
    [dispatch],
  );

  const registeredBrushMeshesRef = useRef(new Set<string>());
  const handleRegisterBrushMesh = useCallback(
    (url: string, positions: number[], indices: number[]) => {
      if (registeredBrushMeshesRef.current.has(url)) return;
      registeredBrushMeshesRef.current.add(url);
      dispatch("registerBrushMesh", { url, positions, indices });
    },
    [dispatch],
  );

  const brushMeshUrls = useMemo(() => [...new Set(meshes.map((mesh) => mesh.url).filter((url): url is string => Boolean(url)))], [meshes]);

  const handleZoomToSelection = useCallback(() => {
    const selectedIds = new Set(selection.ids ?? []);
    if (selectedIds.size === 0) return;
    const selected = instances.filter((instance) => selectedIds.has(instance.id));
    if (selected.length === 0) return;
    let centerX = 0;
    let centerY = 0;
    let centerZ = 0;
    for (const instance of selected) {
      const position = instance.position ?? [instance.x ?? 0, instance.y ?? 0, instance.z ?? 0];
      centerX += position[0];
      centerY += position[1];
      centerZ += position[2];
    }
    const count = selected.length;
    centerX /= count;
    centerY /= count;
    centerZ /= count;
    let maxDistance = 1;
    for (const instance of selected) {
      const position = instance.position ?? [instance.x ?? 0, instance.y ?? 0, instance.z ?? 0];
      const dx = position[0] - centerX;
      const dy = position[1] - centerY;
      const dz = position[2] - centerZ;
      maxDistance = Math.max(maxDistance, Math.hypot(dx, dy, dz));
    }
    const distance = maxDistance * 3 + 2;
    dispatch("setCamera", {
      camera: {
        position: [centerX + distance * 0.6, centerY - distance * 0.6, centerZ + distance * 0.5],
        target: [centerX, centerY, centerZ],
        fov: cameraState.fov,
      },
    });
  }, [cameraState.fov, dispatch, instances, selection.ids]);

  const handleContextMenuSelect = useCallback(
    (item: WorldContextMenuItem) => {
      if (item.action === "zoomToSelection") {
        handleZoomToSelection();
        return;
      }
      if (item.action === "openVortexSuggestions") {
        dispatch(item.action, { ...item.args, x: contextMenu?.x ?? 0, y: contextMenu?.y ?? 0 });
        return;
      }
      dispatch(item.action, item.args);
    },
    [contextMenu, dispatch, handleZoomToSelection],
  );

  const hoveredVortexFullIdRef = useRef<string | null>(null);
  useEffect(() => {
    hoveredVortexFullIdRef.current = interaction.hoveredVortexFullId ?? null;
  }, [interaction.hoveredVortexFullId]);

  const handleWorldOrbitRightPointerDown = useCallback(
    (event: PointerEvent) => {
      if (event.altKey && hoveredVortexFullIdRef.current) {
        dispatch("openVortexSuggestions", { fullId: hoveredVortexFullIdRef.current, x: event.clientX, y: event.clientY });
        return false;
      }
      return true;
    },
    [dispatch],
  );

  const handleSuggestionHover = useCallback((index: number) => dispatch("hoverSuggestion", { index }), [dispatch]);
  const handleSuggestionAccept = useCallback((index: number) => dispatch("acceptSuggestion", { index }), [dispatch]);
  const handleSuggestionClose = useCallback(() => dispatch("closeVortexSuggestions"), [dispatch]);

  useEffect(() => {
    if (interaction.suggestionMenu?.open && interaction.suggestionMenu.pending) {
      const timer = window.setInterval(() => dispatch("suggestionsTick"), 120);
      return () => window.clearInterval(timer);
    }
  }, [dispatch, interaction.suggestionMenu?.open, interaction.suggestionMenu?.pending]);

  useEffect(() => {
    if (activeUtility === "fill" && interaction.fillBuild && !interaction.fillBuild.done) {
      const timer = window.setInterval(() => dispatch("fillBuildTick"), 120);
      return () => window.clearInterval(timer);
    }
  }, [activeUtility, dispatch, interaction.fillBuild]);

  const selectionArgs = useCallback(
    () => ({
      mode: selection.selectionMode ?? selection.granularity ?? "mesh",
      ids: selection.componentIds ?? [],
    }),
    [selection.componentIds, selection.granularity, selection.selectionMode],
  );

  const handleInstancePointerDown = useCallback(
    (id: string, index: number, event: { shiftKey: boolean; ctrlKey: boolean; metaKey: boolean }) => {
      const merge = instanceMergeArg(marqueeModeFromModifiers(event));
      if (selectionMode === "mesh" || selectionMode === "object") {
        dispatch("worldPick", { granularity: "mesh", id: index, merge });
        return;
      }
      dispatch("worldSelect", {
        ids: [id],
        merge,
      });
    },
    [dispatch, selectionMode],
  );

  const handleInstancePointerMove = useCallback(
    (id: string | null) => {
      if (id == null) {
        dispatch("setHover", {});
        return;
      }
      dispatch("setHover", { objectId: id, mode: "mesh", id: 0 });
    },
    [dispatch],
  );

  const handleComponentHover = useCallback(
    (args: { objectId: string; mode: string; id: number } | null) => {
      if (!args) {
        dispatch("setHover", {});
        return;
      }
      dispatch("setHover", args);
    },
    [dispatch],
  );

  const handleVortexHover = useCallback(
    (fullId: string | null) => {
      if (!fullId) {
        dispatch("worldVortexHover", {});
        return;
      }
      dispatch("worldVortexHover", { fullId });
    },
    [dispatch],
  );

  const handleVortexSelect = useCallback(
    (fullId: string, event?: { shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => {
      const merge = event ? instanceMergeArg(marqueeModeFromModifiers(event)) : (globalThis as any).__selectionMode || "default";
      dispatch("worldVortexSelect", { fullId, merge });
    },
    [dispatch],
  );

  const handleConnectDragStart = useCallback((fullId: string, position: readonly [number, number, number]) => {
    setVortexPointerArm(null);
    setConnectDragSource({ fullId, position });
    setConnectDragHoverPosition(position);
  }, []);

  const handleVortexPointerArm = useCallback(
    (arm: {
      readonly fullId: string;
      readonly position: readonly [number, number, number];
      readonly clientX: number;
      readonly clientY: number;
      readonly event: { readonly shiftKey?: boolean; readonly ctrlKey?: boolean; readonly metaKey?: boolean };
    }) => {
      setVortexPointerArm(arm);
    },
    [],
  );

  const handleVortexPointerMove = useCallback(
    (fullId: string, clientX: number, clientY: number) => {
      setVortexPointerArm((arm) => {
        if (!arm || arm.fullId !== fullId) return arm;
        const distance = Math.hypot(clientX - arm.clientX, clientY - arm.clientY);
        if (distance > MARQUEE_DRAG_THRESHOLD_PX) {
          handleConnectDragStart(arm.fullId, arm.position);
          return null;
        }
        return arm;
      });
    },
    [handleConnectDragStart],
  );

  const handleVortexPointerUp = useCallback(
    (fullId: string, event?: { shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => {
      setVortexPointerArm((arm) => {
        if (arm && arm.fullId === fullId) {
          handleVortexSelect(fullId, arm.event ?? event);
        }
        return null;
      });
    },
    [handleVortexSelect],
  );

  const handleConnectDragHover = useCallback((position: readonly [number, number, number]) => {
    setConnectDragHoverPosition(position);
  }, []);

  const handleConnectDragDrop = useCallback(
    (targetFullId: string, event?: { shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => {
      connectDropConsumedRef.current = true;
      setConnectDragSource((source) => {
        if (source) {
          if (source.fullId === targetFullId) {
            handleVortexSelect(targetFullId, event);
          } else {
            dispatch("createAttraction", { attracting: source.fullId, attracted: targetFullId });
          }
        }
        return null;
      });
      setConnectDragHoverPosition(null);
    },
    [dispatch, handleVortexSelect],
  );

  const handleConnectDragCancel = useCallback(() => {
    setVortexPointerArm(null);
    setConnectDragSource(null);
    setConnectDragHoverPosition(null);
  }, []);

  const handleVoxelPlace = useCallback(
    (origin: readonly [number, number, number]) => {
      dispatch("addTargetVolume", { origin });
    },
    [dispatch],
  );

  const handleBrushPlace = useCallback(() => {
    const args = brushObjectPlacementArgs(brushPreview);
    if (!args) return;
    dispatch("addBrushObject", args);
  }, [brushPreview, dispatch]);

  const handleWorldPick = useCallback(
    (args: { granularity: string; id: number; merge: string }) => {
      dispatch("worldPick", args);
    },
    [dispatch],
  );

  const paintMode = selection.interactionMode === "paint";
  const handlePaintAt = useCallback(
    (objectId: string, u: number, v: number) => {
      dispatch("paintAt", { objectId, u, v });
    },
    [dispatch],
  );

  const handleCameraChange = useCallback(
    (state: WorldCameraState) => {
      dispatch("setCamera", {
        camera: {
          position: state.position,
          target: state.target,
          zoom: state.zoom,
          fov: cameraState.fov,
          ...(cameraState.explicitProjection ? { projection: state.projection ?? cameraState.projection } : {}),
          ...(cameraState.up ? { up: cameraState.up } : {}),
        },
      });
    },
    [cameraState.explicitProjection, cameraState.fov, cameraState.projection, cameraState.up, dispatch],
  );

  const handleProjectionChange = useCallback(
    (projection: OrbitCameraProjection) => {
      dispatch("setCamera", {
        camera: {
          position: cameraState.position,
          target: cameraState.target,
          zoom: cameraState.zoom,
          fov: cameraState.fov,
          projection,
          ...(cameraState.up ? { up: cameraState.up } : {}),
        },
      });
    },
    [cameraState, dispatch],
  );

  const marqueePreview = useMemo<{ readonly mergedComponentIds: readonly number[] | null; readonly mergedInstanceIds: readonly string[] | null }>(() => {
    if (!marqueeDragActive || !hostRef.current || !cameraRef.current) return { mergedComponentIds: null, mergedInstanceIds: null };
    const rect = hostRef.current.getBoundingClientRect();
    const camera = cameraRef.current;
    if (selectionMode === "mesh" || selectionMode === "object") {
      const hits = resolveMarqueeInstanceIds(instances, meshes, marqueePath, rect, camera, method, marqueeCoverage);
      return { mergedComponentIds: null, mergedInstanceIds: mergeIdSet(marqueeMergeMode, selection.ids ?? [], hits) };
    }
    const hits = resolveMarqueeComponentIds(instances, meshes, selectionMode, selection.activeObjectId, marqueePath, rect, camera, method, marqueeCoverage);
    return { mergedComponentIds: mergeIdSet(marqueeMergeMode, selection.componentIds ?? [], hits), mergedInstanceIds: null };
  }, [instances, marqueeCoverage, marqueeDragActive, marqueeMergeMode, marqueePath, meshes, method, selection.activeObjectId, selection.componentIds, selection.ids, selectionMode]);

  const handleGumballDragEnd = useCallback(
    (_kind: GumballHandleKind, before: GumballPose, after: GumballPose) => {
      const tool = selection.transformMode === "rotate" ? "rotate" : selection.transformMode === "scale" ? "scale" : "translate";
      const base = selectionArgs();
      if (tool === "translate") {
        dispatch("translateSelection", {
          ...base,
          dx: after.position[0] - before.position[0],
          dy: after.position[1] - before.position[1],
          dz: after.position[2] - before.position[2],
        });
        return;
      }
      if (tool === "rotate") {
        const beforeQuat = new Quaternion(...before.quaternion);
        const afterQuat = new Quaternion(...after.quaternion);
        const delta = afterQuat.multiply(beforeQuat.invert());
        // Quaternion.w = cos(angle/2); clamp for asin/acos precision at the identity boundary.
        const angle = 2 * Math.acos(Math.min(1, Math.max(-1, delta.w)));
        const sinHalfAngle = Math.sqrt(Math.max(0, 1 - delta.w * delta.w));
        const axis = sinHalfAngle < 1e-6 ? { x: 0, y: 0, z: 1 } : { x: delta.x / sinHalfAngle, y: delta.y / sinHalfAngle, z: delta.z / sinHalfAngle };
        dispatch("rotateSelection", {
          ...base,
          ax: axis.x,
          ay: axis.y,
          az: axis.z,
          angle,
        });
        return;
      }
      const sx = after.scale[0] / Math.max(before.scale[0], 1e-6);
      const sy = after.scale[1] / Math.max(before.scale[1], 1e-6);
      const sz = after.scale[2] / Math.max(before.scale[2], 1e-6);
      dispatch("scaleSelection", { ...base, sx, sy, sz });
    },
    [dispatch, selection.transformMode, selectionArgs],
  );

  const handleFaceDragStart = useCallback((args: { objectId: string; faceId: number; normal: readonly [number, number, number]; point: readonly [number, number, number]; faceExtent?: readonly [number, number] }) => {
    setFaceDragSession({ objectId: args.objectId, faceId: args.faceId, normal: args.normal, startPoint: args.point, faceExtent: args.faceExtent });
  }, []);

  const toLocalPoint = useCallback((event: React.PointerEvent<HTMLDivElement>): SelectionMarqueePoint => {
    const rect = hostRef.current?.getBoundingClientRect();
    if (!rect) return { x: event.clientX, y: event.clientY };
    return { x: event.clientX - rect.left, y: event.clientY - rect.top };
  }, []);

  const handlePointerDown = useCallback(
    (event: React.PointerEvent<HTMLDivElement>) => {
      if (event.button !== 0) return;
      if (selection.engagementSessionActive && hostRef.current && cameraRef.current) {
        const rect = hostRef.current.getBoundingClientRect();
        const point = raycastGroundPoint(event.clientX, event.clientY, rect, cameraRef.current);
        if (point) {
          dispatch("worldPointerDown", {
            pane: paneSuffixFromSurfaceId(node.surfaceId),
            position: point,
            shiftKey: event.shiftKey,
            ctrlKey: event.ctrlKey,
            metaKey: event.metaKey,
          });
          return;
        }
      }
      if (paintMode) {
        setPaintStrokeActive(true);
        dispatch("paintStrokeBegin");
      }
      setMarqueeModifiers({ shiftKey: event.shiftKey, ctrlKey: event.ctrlKey, metaKey: event.metaKey });
      setMarqueePath([toLocalPoint(event)]);
    },
    [dispatch, node.surfaceId, paintMode, selection.engagementSessionActive, toLocalPoint],
  );

  const handlePointerMove = useCallback(
    (event: React.PointerEvent<HTMLDivElement>) => {
      if (selection.engagementSessionActive && hostRef.current && cameraRef.current) {
        const rect = hostRef.current.getBoundingClientRect();
        const point = raycastGroundPoint(event.clientX, event.clientY, rect, cameraRef.current);
        const last = engagementPointerMoveLastPointRef.current;
        const unchanged = point && last && point[0] === last[0] && point[1] === last[1] && point[2] === last[2];
        if (point && !unchanged && !engagementPointerMoveInFlightRef.current) {
          engagementPointerMoveInFlightRef.current = true;
          engagementPointerMoveLastPointRef.current = point;
          requestAnimationFrame(() => {
            engagementPointerMoveInFlightRef.current = false;
            dispatch("worldPointerMove", { pane: paneSuffixFromSurfaceId(node.surfaceId), position: point });
          });
        }
        return;
      }
      if (!marqueeDown) return;
      setMarqueeModifiers({ shiftKey: event.shiftKey, ctrlKey: event.ctrlKey, metaKey: event.metaKey });
      setMarqueePath((path) => [...path, toLocalPoint(event)]);
    },
    [dispatch, marqueeDown, node.surfaceId, selection.engagementSessionActive, toLocalPoint],
  );

  const handlePointerUp = useCallback(
    (event: React.PointerEvent<HTMLDivElement>) => {
      if (faceDragSession) {
        const session = faceDragSession;
        setFaceDragSession(null);
        if (hostRef.current && cameraRef.current) {
          const rect = hostRef.current.getBoundingClientRect();
          const distance = axisDragParam(event.clientX, event.clientY, rect, cameraRef.current, session.startPoint, session.normal);
          if (distance != null && Math.abs(distance) > 1e-4) {
            dispatch("worldFaceDragEnd", {
              pane: paneSuffixFromSurfaceId(node.surfaceId),
              objectId: session.objectId,
              faceId: session.faceId,
              normal: session.normal,
              startPoint: session.startPoint,
              distance,
              faceExtent: session.faceExtent,
            });
          }
        }
        return;
      }
      if (marqueeDragActive) {
        if (marqueePreview.mergedInstanceIds) {
          dispatch("worldSelect", { ids: marqueePreview.mergedInstanceIds, merge: "replace" });
        } else if (marqueePreview.mergedComponentIds) {
          dispatch("setSelection", { mode: selectionMode, ids: marqueePreview.mergedComponentIds });
        }
      }
      wasMarqueeDragRef.current = marqueeDragActive;
      if (paintStrokeActive) {
        dispatch("paintStrokeEnd");
        setPaintStrokeActive(false);
      }
      setMarqueePath([]);
      setVortexPointerArm(null);
      if (connectDropConsumedRef.current) {
        connectDropConsumedRef.current = false;
      } else {
        handleConnectDragCancel();
      }
    },
    [dispatch, faceDragSession, handleConnectDragCancel, marqueeDragActive, marqueePreview, node.surfaceId, paintStrokeActive, selectionMode],
  );

  const handleEmptyClick = useCallback(
    (event: MouseEvent) => {
      if (wasMarqueeDragRef.current) return;
      if (selection.engagementSessionActive || paintMode) return;
      dispatch("worldPick", { granularity: selectionMode, id: null, merge: instanceMergeArg(marqueeModeFromModifiers(event)) });
    },
    [dispatch, paintMode, selection.engagementSessionActive, selectionMode],
  );

  const clearCatalogueDrop = useCallback(() => {
    setCatalogueDropPreview(null);
    catalogueDragEncodedRef.current = null;
    catalogueDragDepthRef.current = 0;
  }, []);

  const readCatalogueDragEncoded = useCallback((): string | null => {
    return getActiveCatalogueDragPayload() ?? catalogueDragEncodedRef.current;
  }, []);

  const updateCatalogueDropPreviewAt = useCallback(
    (clientX: number, clientY: number) => {
      const encoded = readCatalogueDragEncoded();
      const payload = parsePuzzle3dCatalogueDragPayload(encoded);
      if (!payload || !hostRef.current || !cameraRef.current) {
        setCatalogueDropPreview(null);
        return;
      }
      const rect = hostRef.current.getBoundingClientRect();
      if (!clientPointOverHost(clientX, clientY, rect)) {
        setCatalogueDropPreview(null);
        return;
      }
      const origin = resolveCatalogueDropOrigin(clientX, clientY, rect, cameraRef.current, gridSnapEnabled, gridFactor);
      if (!origin) {
        setCatalogueDropPreview(null);
        return;
      }
      if (encoded) catalogueDragEncodedRef.current = encoded;
      setCatalogueDropPreview({ ...payload, origin });
    },
    [gridFactor, gridSnapEnabled, readCatalogueDragEncoded],
  );

  const commitCatalogueDropAt = useCallback(
    (clientX: number, clientY: number, encoded?: string | null) => {
      const payload = parsePuzzle3dCatalogueDragPayload(encoded ?? readCatalogueDragEncoded());
      if (!payload || !hostRef.current || !cameraRef.current) return;
      const rect = hostRef.current.getBoundingClientRect();
      const origin = resolveCatalogueDropOrigin(clientX, clientY, rect, cameraRef.current, gridSnapEnabled, gridFactor);
      if (!origin) return;
      dispatch("addObjectKind", { objectKind: payload.objectKind, origin });
    },
    [dispatch, gridFactor, gridSnapEnabled, readCatalogueDragEncoded],
  );

  const onCatalogueDragEnter = useCallback(
    (event: DragEvent<HTMLDivElement>) => {
      if (!scene) return;
      if (!event.dataTransfer.types.includes(CATALOGUE_DRAG_MIME) && !getActiveCatalogueDragPayload()) return;
      event.preventDefault();
      catalogueDragDepthRef.current += 1;
    },
    [scene],
  );

  const onCatalogueDragLeave = useCallback(
    (_event: DragEvent<HTMLDivElement>) => {
      if (!scene) return;
      catalogueDragDepthRef.current = Math.max(0, catalogueDragDepthRef.current - 1);
      if (catalogueDragDepthRef.current === 0) clearCatalogueDrop();
    },
    [clearCatalogueDrop, scene],
  );

  const onCatalogueDragOver = useCallback(
    (event: DragEvent<HTMLDivElement>) => {
      if (!scene) return;
      if (!event.dataTransfer.types.includes(CATALOGUE_DRAG_MIME) && !getActiveCatalogueDragPayload()) return;
      const encoded = getActiveCatalogueDragPayload();
      if (!parsePuzzle3dCatalogueDragPayload(encoded) && !event.dataTransfer.types.includes(CATALOGUE_DRAG_MIME)) return;
      event.preventDefault();
      event.dataTransfer.dropEffect = "copy";
      if (encoded) catalogueDragEncodedRef.current = encoded;
      updateCatalogueDropPreviewAt(event.clientX, event.clientY);
    },
    [scene, updateCatalogueDropPreviewAt],
  );

  const onCatalogueDrop = useCallback(
    (event: DragEvent<HTMLDivElement>) => {
      if (!scene) return;
      event.preventDefault();
      const encoded = event.dataTransfer.getData(CATALOGUE_DRAG_MIME) || getActiveCatalogueDragPayload() || catalogueDragEncodedRef.current;
      commitCatalogueDropAt(event.clientX, event.clientY, encoded);
      clearCatalogueDrop();
    },
    [clearCatalogueDrop, commitCatalogueDropAt, scene],
  );

  useEffect(() => {
    const onPointerMove = (event: PointerEvent) => {
      const encoded = getActiveCatalogueDragPayload();
      if (!encoded) return;
      if (!parsePuzzle3dCatalogueDragPayload(encoded)) return;
      catalogueDragEncodedRef.current = encoded;
      updateCatalogueDropPreviewAt(event.clientX, event.clientY);
    };

    const onPointerUp = (event: PointerEvent) => {
      const encoded = getActiveCatalogueDragPayload() ?? catalogueDragEncodedRef.current;
      const payload = parsePuzzle3dCatalogueDragPayload(encoded);
      if (!payload) return;
      const host = hostRef.current;
      if (!host) return;
      const rect = host.getBoundingClientRect();
      if (!clientPointOverHost(event.clientX, event.clientY, rect)) {
        clearCatalogueDrop();
        return;
      }
      commitCatalogueDropAt(event.clientX, event.clientY, encoded);
      clearCatalogueDrop();
    };

    window.addEventListener("pointermove", onPointerMove);
    window.addEventListener("pointerup", onPointerUp, true);
    return () => {
      window.removeEventListener("pointermove", onPointerMove);
      window.removeEventListener("pointerup", onPointerUp, true);
    };
  }, [clearCatalogueDrop, commitCatalogueDropAt, updateCatalogueDropPreviewAt]);

  if (!scene) return <div className="semio-world-3d-empty">{emptySceneLabel}</div>;

  return (
    <div
      ref={hostRef}
      className="semio-world-3d-host relative h-full min-h-[24rem] w-full"
      data-surface-id={node.surfaceId}
      data-puzzle3d-fixture-drag-active={catalogueDropPreview ? "" : undefined}
      onContextMenu={(event) => {
        if (event.altKey) return;
        const target = resolveWorldContextMenuTarget(interaction, selection);
        if (!target) return;
        event.preventDefault();
        dispatch("contextMenuAt", { kind: target.kind, id: target.id });
        setContextMenu({ x: event.clientX, y: event.clientY });
      }}
      onPointerDown={handlePointerDown}
      onPointerMove={handlePointerMove}
      onPointerUp={handlePointerUp}
      onDragEnter={onCatalogueDragEnter}
      onDragLeave={onCatalogueDragLeave}
      onDragOver={onCatalogueDragOver}
      onDrop={onCatalogueDrop}
    >
      <WorldCanvas
        className="h-full w-full"
        cameraUp={(cameraState.up as [number, number, number] | undefined) ?? [0, 0, 1]}
        cameraFov={cameraState.fov}
        background={environment && !isTransparentWorldBackground(environment.background) ? environment.background : undefined}
        gl={environment && isTransparentWorldBackground(environment.background) ? { antialias: true, alpha: true } : undefined}
        shadows={environment?.shadow?.enabled === true ? true : undefined}
        onPointerMissed={handleEmptyClick}
        overlay={
          frame || cameraState.explicitProjection ? (
            <>
              {frame ? <IconShotFrame width={frame.width} height={frame.height} shape={frame.shape === "ellipse" ? "ellipse" : "rectangle"} badge={frame.badge !== false} background={frame.background} /> : null}
              {cameraState.explicitProjection ? <WorldOrbitProjectionSwitch projection={cameraState.projection ?? "perspective"} onProjectionChange={handleProjectionChange} /> : null}
            </>
          ) : undefined
        }
      >
        <WorldOrbitViewSnapGateProvider>
          <WorldOrbitCameraViewRig state={cameraState} seedKey={scene?.cameraJson ?? "default"} perspectiveFov={cameraState.fov} />
          <WorldOrbitGated
            controlsGate={marqueeDown || gumballDragActive || connectDragSource !== null || faceDragSession !== null}
            onCamera={handleCameraChange}
            zoom={cameraState.zoom}
            projection={cameraState.explicitProjection ? cameraState.projection : undefined}
            onRightPointerDown={handleWorldOrbitRightPointerDown}
          />
          <WorldLodBridge
            lodRef={lodRef}
            distanceReference={100}
            gridFactor={lod.gridFactor ?? DEFAULT_LOD_GRID_FACTOR}
            gridSnapEnabled={lod.gridSnapEnabled ?? false}
            showLodGrid={lod.showLodGrid ?? true}
            automaticLod={lod.automaticLod ?? true}
            depthVariableLod={lod.depthVariableLod ?? false}
            manualLod={lod.manualLod ?? DEFAULT_MANUAL_LOD}
            gridDatum={[0, 0, 0]}
          >
            <ambientLight color={environment?.ambient?.color ?? "#ffffff"} intensity={environment?.ambient?.intensity ?? 1.15} />
            {environment?.sun?.enabled === true ? (
              <directionalLight
                color={environment.sun?.color ?? "#ffffff"}
                intensity={environment.sun?.intensity ?? 0.85}
                position={sunPositionFromAzimuthElevation(environment.sun?.azimuth ?? 45, environment.sun?.elevation ?? 35)}
                castShadow={environment.shadow?.enabled === true}
              />
            ) : (
              <>
                <hemisphereLight color="#ffffff" groundColor="#9aa0ab" intensity={1.35} position={[0, 0, 1]} />
                <directionalLight position={[12, 18, 10]} intensity={2.4} />
                <directionalLight position={[-14, -10, 6]} intensity={1.2} />
                <directionalLight position={[0, 0, -16]} intensity={0.75} />
              </>
            )}
            {fit?.enabled ? <WorldAutoFit groupRef={instancesGroupRef} fitKey={`${fit.revision ?? 0}:${meshes.map((mesh) => mesh.url ?? mesh.id).join(",")}`} padding={fit.padding ?? 1.25} camera={cameraState} onFitted={handleCameraChange} /> : null}
            <CameraRefBridge cameraRef={cameraRef} />
            <RaycasterPickTuning />
            {brushMeshUrls.map((url) => (
              <Suspense key={url} fallback={null}>
                <BrushMeshRegistrar url={url} onRegister={handleRegisterBrushMesh} />
              </Suspense>
            ))}
            <WorldTerrainLayer terrainJson={scene?.terrainJson} cameraPosition={cameraState.position} cameraTarget={cameraState.target} />
            <WorldPointCloudLayer pointsJson={scene?.pointsJson} />
            <group ref={instancesGroupRef}>
              <WorldInstancesLayer
                instances={instances}
                meshes={meshes}
                selection={selection}
                palette={meshStylePalette}
                onInstancePointerDown={handleInstancePointerDown}
                onInstancePointerMove={handleInstancePointerMove}
                onWorldPick={handleWorldPick}
                onComponentHover={handleComponentHover}
                onPaintAt={paintMode ? handlePaintAt : undefined}
                gumballDragActive={gumballDragActive}
                onGumballDraggingChanged={setGumballDragActive}
                onGumballDragEnd={handleGumballDragEnd}
                onFaceDragStart={handleFaceDragStart}
                mergedComponentIds={marqueePreview.mergedComponentIds}
                mergedInstanceIds={marqueePreview.mergedInstanceIds}
                blockPick={worldInstancePickBlocked(activeUtility)}
                environment={environment}
              />
            </group>
            <WorldVortexMarkers
              vortices={vortices}
              palette={meshStylePalette}
              brushMode={brushMode}
              selectionMode={selectionMode}
              connectSourceFullId={connectDragSource?.fullId}
              onHover={handleVortexHover}
              onVortexSelect={handleVortexSelect}
              onBrushPlace={handleBrushPlace}
              onVortexPointerArm={handleVortexPointerArm}
              onVortexPointerMove={handleVortexPointerMove}
              onVortexPointerUp={handleVortexPointerUp}
              onConnectDragHover={handleConnectDragHover}
              onConnectDragDrop={handleConnectDragDrop}
            />
            {connectDragSource && connectDragHoverPosition ? <WorldConnectRubberBand from={connectDragSource.position} to={connectDragHoverPosition} /> : null}
            <WorldAttractionLines attractions={attractions} />
            {brushPreview ? <BrushPreviewGhost preview={brushPreview} meshes={meshes} palette={meshStylePalette} /> : null}
            {!brushPreview && catalogueDropPreview ? <CatalogueDropGhost preview={catalogueDropPreview} meshes={meshes} palette={meshStylePalette} /> : null}
            {engagementPreview.length > 0 ? <EngagementPreviewLayer items={engagementPreview} color={colors.hover} /> : null}
            <WorldVolumeLayer
              volumes={targetVolumes.map((volume) => ({
                id: volume.id,
                origin: volume.origin as [number, number, number],
                orientation: volume.orientation as [number, number, number, number] | undefined,
                scale: volume.scale,
                color: volume.color,
              }))}
              interactive={false}
            />
            {interaction.fillEditTargetVolumes ? <WorldVoxelGroundPlane gridFactor={interaction.gridFactor ?? DEFAULT_LOD_GRID_FACTOR} onHover={setVoxelHoverOrigin} onPlace={handleVoxelPlace} /> : null}
            {interaction.fillEditTargetVolumes && voxelHoverOrigin ? <WorldVoxelPreviewBox origin={voxelHoverOrigin} dims={interaction.voxelDims ?? [1, 1, 1]} gridFactor={interaction.gridFactor ?? DEFAULT_LOD_GRID_FACTOR} /> : null}
            <WorldReferenceLayer
              references={references
                .filter((reference) => !reference.hidden)
                .map((reference) => ({
                  id: reference.id,
                  source: { url: reference.url, mediaKind: "image" as const },
                  origin: reference.origin as [number, number, number],
                  widthWorld: reference.widthWorld,
                  locked: reference.locked,
                  opacity: reference.opacity,
                }))}
              selectedIds={referenceSelectedIds}
              hoveredId={referenceHoveredId}
              onSelect={(id) => handleReferenceSelect(id)}
              onHover={handleReferenceHover}
            />
          </WorldLodBridge>
        </WorldOrbitViewSnapGateProvider>
      </WorldCanvas>
      {marqueeDragActive && marqueeStart && marqueeEnd ? (
        method === "lasso" ? (
          <SelectionMarquee coverage={marqueeCoverage} shape="polygon" points={marqueePath} />
        ) : (
          <SelectionMarquee
            coverage={marqueeCoverage}
            shape="rect"
            rect={{
              x: Math.min(marqueeStart.x, marqueeEnd.x),
              y: Math.min(marqueeStart.y, marqueeEnd.y),
              width: Math.abs(marqueeEnd.x - marqueeStart.x),
              height: Math.abs(marqueeEnd.y - marqueeStart.y),
            }}
          />
        )
      ) : null}
      <ContextMenuController
        open={contextMenu != null && contextMenuItems.length > 0}
        position={contextMenu ?? { x: 0, y: 0 }}
        items={contextMenuItems.map((item) => ({
          id: item.id,
          label: item.label,
          onSelect: () => handleContextMenuSelect(item),
        }))}
        onOpenChange={(open) => {
          if (!open) setContextMenu(null);
        }}
      />
      {interaction.suggestionMenu?.open ? (
        <WorldSuggestionMenu menu={interaction.suggestionMenu} activeIndex={interaction.brushCandidateIndex ?? 0} onHoverCandidate={handleSuggestionHover} onAcceptCandidate={handleSuggestionAccept} onClose={handleSuggestionClose} />
      ) : null}
    </div>
  );
}
//#endregion World3dHost
//#endregion 🔖World3dHost

//#region 🔖NodeGraphHost
//#region Types
type MediaGraphPort = {
  readonly id: string;
  readonly resourceKind?: string;
  readonly direction?: string;
  readonly label?: string;
};

type MediaGraphNodeRecord = {
  readonly id: string;
  readonly instanceId?: string;
  readonly label?: string;
  readonly x?: number;
  readonly y?: number;
  readonly width?: number;
  readonly height?: number;
  readonly inputs?: readonly MediaGraphPort[];
  readonly outputs?: readonly MediaGraphPort[];
};

type MediaGraphEdgeRecord = {
  readonly id: string;
  readonly sourceNodeId: string;
  readonly sourcePortId: string;
  readonly targetNodeId: string;
  readonly targetPortId: string;
};

type MediaGraphNodeData = {
  readonly label: string;
  readonly inputs: readonly MediaGraphPort[];
  readonly outputs: readonly MediaGraphPort[];
  readonly width: number;
  readonly height: number;
};

type DiagramViewport = { readonly x: number; readonly y: number; readonly zoom: number };

type GraphFindItem = { readonly id: string; readonly label: string; readonly category?: string };

type GraphContextMenuItem = {
  readonly id: string;
  readonly label: string;
  readonly action: string;
  readonly args?: Record<string, unknown>;
};

type FrameworkGraphSession = GraphWasmSession & {
  syncFromSceneJson(json: string): void;
  pointerDownScreen(sx: number, sy: number, button: number, shift: boolean, ctrlOrMeta: boolean, alt: boolean): void;
  pointerMoveScreen(sx: number, sy: number, shift: boolean, ctrlOrMeta: boolean, alt: boolean): void;
  pointerUpScreen(sx: number, sy: number, shift: boolean, ctrlOrMeta: boolean, alt: boolean): void;
  wheelScreen(sx: number, sy: number, deltaX: number, deltaY: number, zoomGesture: boolean): void;
  labelOverlayPaintStateJson(): string;
  paramOverlayPaintStateJson(): string;
  stepperOverlayStateJson(): string;
  sliderOverlayStateJson(): string;
  selectionUnionBoundsScreenJson(): string;
  selectionPreviewPointsJson(): string;
  selectionPreviewCrossing(): boolean;
  selectedNodeIdsJson(): string;
  hoveredNodeId(): string | null | undefined;
  hoveredChannelJson(): string;
  cameraJson(): string;
  takePendingOpenInstanceId(): string | null | undefined;
  pickTargetsAtScreenJson(sx: number, sy: number): string;
  setHover?(widgetId: string | null): void;
  setHoverChannel?(widgetId: string | null, port?: string | null): void;
  alignSelection?(mode: string): void;
  fixtureJson?(): string;
  setCanvasThemeJson?(json: string): void;
};
//#endregion Types

//#region Viewport
export function nodeGraphViewportActionArgs(cameraJson: string): { readonly viewportJson: string } {
  return { viewportJson: cameraJson };
}
//#endregion Viewport

//#region Parsing
function parseViewport(viewportJson: string): DiagramViewport {
  try {
    const parsed = JSON.parse(viewportJson) as Partial<DiagramViewport>;
    return { x: Number(parsed.x ?? 0), y: Number(parsed.y ?? 0), zoom: Number(parsed.zoom ?? 1) };
  } catch {
    return { x: 0, y: 0, zoom: 1 };
  }
}

/** @emoji 🔎 Resolves a flow fixture widget id to the media-graph instance id it previews, used to open an app instance without depending on plugin-side selection state. */
export function resolveFixtureWidgetInstanceId(fixtureJson: string | undefined, widgetId: string | undefined | null): string | undefined {
  if (!fixtureJson || !widgetId) return undefined;
  try {
    const fixture = JSON.parse(fixtureJson) as {
      readonly widgets?: readonly { readonly id?: string; readonly params?: { readonly instanceId?: string } }[];
    };
    return fixture.widgets?.find((widget) => widget.id === widgetId)?.params?.instanceId;
  } catch {
    return undefined;
  }
}

export interface CatalogueAppDragPayload {
  readonly programId: string;
  readonly appId: string;
  readonly label?: string;
}

/** @emoji 🎯 Parses a catalogue drag payload; returns null for non-catalogue-app payloads (garbage/legacy descriptors). */
export function parseCatalogueAppDragPayload(raw: string): CatalogueAppDragPayload | null {
  try {
    const parsed = JSON.parse(raw) as { readonly programId?: string; readonly appId?: string; readonly label?: string };
    if (!parsed.programId || !parsed.appId) return null;
    return { programId: parsed.programId, appId: parsed.appId, label: parsed.label };
  } catch {
    return null;
  }
}

/** @emoji 👻 Builds the ghost widget descriptor shown while a catalogue app is dragged over the media graph. */
export function catalogueGhostDescriptorJson(payload: CatalogueAppDragPayload): string {
  return JSON.stringify({ kind: "neuron", neuronKind: payload.label ?? payload.appId });
}

function portLabel(port: MediaGraphPort): string {
  if (port.label) return port.label;
  const segments = port.id.split(":");
  return segments[segments.length - 1] ?? port.id;
}

function mediaGraphNodesToDiagramNodes(records: readonly MediaGraphNodeRecord[]): Node<MediaGraphNodeData>[] {
  return records.map((record) => ({
    id: record.id,
    type: "mediaGraph",
    position: { x: record.x ?? 0, y: record.y ?? 0 },
    data: {
      label: record.label?.trim() || record.instanceId || record.id,
      inputs: record.inputs ?? [],
      outputs: record.outputs ?? [],
      width: record.width ?? 180,
      height: record.height ?? 72,
    },
  }));
}

function mediaGraphEdgesToDiagramEdges(records: readonly MediaGraphEdgeRecord[]): Edge[] {
  return records.map((record) => ({
    id: record.id,
    source: record.sourceNodeId,
    target: record.targetNodeId,
    sourceHandle: record.sourcePortId,
    targetHandle: record.targetPortId,
  }));
}
//#endregion Parsing

//#region Keyboard
function isEditableGraphKeyTarget(target: EventTarget | null): boolean {
  if (!(target instanceof HTMLElement)) return false;
  const tag = target.tagName;
  if (tag === "INPUT" || tag === "TEXTAREA" || tag === "SELECT") return true;
  if (target.isContentEditable) return true;
  return target.closest("[contenteditable='true'], [role='textbox']") != null;
}

function handleGraphKeyboard(event: KeyboardEvent<HTMLDivElement>, editable: boolean, parsedNodes: readonly MediaGraphNodeRecord[], dispatch: (action: string, args?: Record<string, unknown>) => void) {
  if (!editable || isEditableGraphKeyTarget(event.target)) return;
  const mod = event.metaKey || event.ctrlKey;
  if (mod && event.key.toLowerCase() === "a") {
    event.preventDefault();
    dispatch("setMediaNodeSelection", { nodeIds: parsedNodes.map((node) => node.id) });
    return;
  }
  if (event.key === "Escape") {
    event.preventDefault();
    dispatch("setMediaNodeSelection", { nodeIds: [] });
    return;
  }
  if (event.key === "Delete" || event.key === "Backspace") {
    event.preventDefault();
    dispatch("deleteSelection", {});
  }
}
//#endregion Keyboard

//#region DiagramNode
function MediaGraphDiagramNode({ data }: NodeProps<MediaGraphNodeData>) {
  const inputCount = Math.max(data.inputs.length, 1);
  const outputCount = Math.max(data.outputs.length, 1);
  const rowCount = Math.max(inputCount, outputCount);
  const rowHeight = 18;
  const bodyHeight = Math.max(data.height, 56 + rowCount * rowHeight);
  return (
    <div className="rounded border border-border bg-panel text-panel-foreground shadow-sm" style={{ width: data.width, minHeight: bodyHeight }}>
      <div className="border-b border-border px-2 py-1 text-xs font-medium">{data.label}</div>
      <div className="relative px-2 py-1 text-[10px] leading-[18px]">
        {Array.from({ length: rowCount }, (_, rowIndex) => {
          const input = data.inputs[rowIndex];
          const output = data.outputs[rowIndex];
          const top = 8 + rowIndex * rowHeight;
          return (
            <div key={`${input?.id ?? "in"}:${output?.id ?? "out"}:${rowIndex}`} className="relative h-[18px]">
              {input ? (
                <>
                  <Handle id={input.id} type="target" position={Position.Left} className="!size-2 !border-panel !bg-foreground" style={{ top }} />
                  <span className="pl-3 text-muted-foreground">{portLabel(input)}</span>
                </>
              ) : null}
              {output ? (
                <>
                  <Handle id={output.id} type="source" position={Position.Right} className="!size-2 !border-panel !bg-foreground" style={{ top }} />
                  <span className="absolute right-3 top-0 text-right text-muted-foreground">{portLabel(output)}</span>
                </>
              ) : null}
            </div>
          );
        })}
      </div>
    </div>
  );
}

const mediaGraphNodeTypes: NodeTypes = { mediaGraph: MediaGraphDiagramNode };
//#endregion DiagramNode

//#region WasmGraphSurface
function WasmGraphSurface({
  scene,
  surfaceId,
  controllerId,
  editable,
  contextMenuItems,
  onAction,
}: {
  readonly scene: NodeGraphScene;
  readonly surfaceId: string;
  readonly controllerId: string;
  readonly editable: boolean;
  readonly contextMenuItems: readonly GraphContextMenuItem[];
  readonly onAction: (action: ActionDescriptor) => void;
}) {
  const sessionRef = useRef<FrameworkGraphSession | null>(null);
  const labelCanvasRef = useRef<HTMLCanvasElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const [contextMenu, setContextMenu] = useState<{ readonly x: number; readonly y: number } | null>(null);
  const [selectionBounds, setSelectionBounds] = useState<ReturnType<typeof parseDagSelectionUnionBoundsScreen>>(null);
  const [marquee, setMarquee] = useState<ReturnType<typeof computeDagMarqueeOverlay>>(null);
  const [overlaySize, setOverlaySize] = useState({ w: 0, h: 0 });
  const [paramStateJson, setParamStateJson] = useState("{}");
  const [stepperStateJson, setStepperStateJson] = useState("{}");
  const [sliderStateJson, setSliderStateJson] = useState("{}");
  const sceneJson = useMemo(() => sceneToSyncJson(scene), [scene]);

  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      onAction({ controllerId, action, args: { surfaceId, ...args } });
    },
    [controllerId, onAction, surfaceId],
  );

  const paintOverlays = useCallback(() => {
    const session = sessionRef.current;
    const labelCanvas = labelCanvasRef.current;
    const container = containerRef.current;
    if (!session || !labelCanvas || !container) return;
    const rect = container.getBoundingClientRect();
    const dpr = globalThis.devicePixelRatio || 1;
    try {
      paintDagLabelOverlays(session.labelOverlayPaintStateJson(), labelCanvas, rect.width, rect.height, dpr, {
        hoveredId: session.hoveredNodeId() ?? null,
        selectedIds: parseDagNodeIdArray(session.selectedNodeIdsJson()),
        preselect: { ids: [], removedIds: [] },
        dimmedIds: [],
      });
    } catch {
      /* gpu not ready */
    }
    setSelectionBounds(parseDagSelectionUnionBoundsScreen(session.selectionUnionBoundsScreenJson()));
    setMarquee(computeDagMarqueeOverlay(session.selectionPreviewPointsJson(), session.selectionPreviewCrossing(), "rectangle"));
    try {
      setParamStateJson(session.paramOverlayPaintStateJson());
      setStepperStateJson(session.stepperOverlayStateJson());
      setSliderStateJson(session.sliderOverlayStateJson());
    } catch {
      /* session not ready */
    }
    setOverlaySize({ w: rect.width, h: rect.height });
  }, []);

  useEffect(() => {
    sessionRef.current?.syncFromSceneJson(sceneJson);
    paintOverlays();
  }, [sceneJson, paintOverlays]);

  const onSessionReady = useCallback(
    (session: GraphWasmSession) => {
      sessionRef.current = session as FrameworkGraphSession;
      sessionRef.current.syncFromSceneJson(sceneJson);
      paintOverlays();
    },
    [sceneJson, paintOverlays],
  );

  const [wasmSession, setWasmSession] = useState<FrameworkGraphSession | null>(null);

  useEffect(() => {
    let cancelled = false;
    void createGraphSession().then((session) => {
      if (!cancelled) setWasmSession(session as FrameworkGraphSession);
    });
    return () => {
      cancelled = true;
    };
  }, []);

  const sessionFactory = useCallback(() => {
    if (wasmSession) return wasmSession;
    return {
      attachCanvas: async () => undefined,
      setSize: () => {},
      renderFrame: () => {},
      syncFromSceneJson: () => {},
      pointerDownScreen: () => {},
      pointerMoveScreen: () => {},
      pointerUpScreen: () => {},
      wheelScreen: () => {},
      labelOverlayPaintStateJson: () => '{"labels":[]}',
      paramOverlayPaintStateJson: () => "{}",
      stepperOverlayStateJson: () => "{}",
      sliderOverlayStateJson: () => "{}",
      selectionUnionBoundsScreenJson: () => "{}",
      selectionPreviewPointsJson: () => "[]",
      selectionPreviewCrossing: () => false,
      selectedNodeIdsJson: () => "[]",
      hoveredNodeId: () => null,
      hoveredChannelJson: () => "{}",
      cameraJson: () => scene.viewportJson,
      pickTargetsAtScreenJson: () => "[]",
      setHover: () => {},
      setHoverChannel: () => {},
      alignSelection: () => {},
      fixtureJson: () => "{}",
      takePendingOpenInstanceId: () => null,
    } satisfies FrameworkGraphSession;
  }, [scene.viewportJson, wasmSession]);

  const emitInteractionState = useCallback(() => {
    const session = sessionRef.current;
    if (!session) return;
    try {
      const nodeIds = JSON.parse(session.selectedNodeIdsJson()) as string[];
      dispatch(nodeGraphActions.select, { nodeIds });
      const hovered = session.hoveredNodeId();
      dispatch(nodeGraphActions.hover, { hoverJson: hovered ? JSON.stringify({ nodeId: hovered }) : null });
      dispatch(nodeGraphActions.viewport, nodeGraphViewportActionArgs(session.cameraJson()));
      const openId = session.takePendingOpenInstanceId?.();
      if (openId) dispatch("openInstance", { instanceId: openId });
    } catch {
      /* session not ready */
    }
    paintOverlays();
  }, [dispatch, paintOverlays]);

  const commitGraphFixture = useCallback(() => {
    const session = sessionRef.current;
    if (!session?.fixtureJson) return;
    try {
      const fixtureJson = session.fixtureJson();
      dispatch(nodeGraphActions.edit, { ops: [{ op: "setFixture", fixtureJson }] });
    } catch {
      /* session not ready */
    }
  }, [dispatch]);

  const pickInteraction = useCanvasPickInteraction({
    resolveTargetsAtClient: (client) => {
      const session = sessionRef.current;
      const container = containerRef.current;
      if (!session?.pickTargetsAtScreenJson || !container) return [];
      const rect = container.getBoundingClientRect();
      const sx = client.x - rect.left;
      const sy = client.y - rect.top;
      try {
        return JSON.parse(session.pickTargetsAtScreenJson(sx, sy)) as CanvasPickTarget[];
      } catch {
        return [];
      }
    },
    onHoverFocus: (focus) => {
      const session = sessionRef.current;
      if (!session) return;
      const target = focus.target;
      if (!target) {
        session.setHover?.(null);
      } else if (target.portId) {
        session.setHoverChannel?.(target.id, target.portId);
      } else {
        session.setHover?.(target.id);
      }
      session.renderFrame();
      paintOverlays();
    },
    onSelectTarget: () => {
      emitInteractionState();
    },
  });

  return (
    <div
      ref={containerRef}
      className="relative h-full w-full"
      onContextMenu={(event) => {
        if (!editable || contextMenuItems.length === 0) return;
        event.preventDefault();
        setContextMenu({ x: event.clientX, y: event.clientY });
      }}
      onPointerUp={emitInteractionState}
    >
      <GraphWasmCanvas className="absolute inset-0" sessionFactory={sessionFactory} onSessionReady={onSessionReady} enablePointer={false} />
      <canvas ref={labelCanvasRef} className="pointer-events-none absolute inset-0 z-40" />
      {selectionBounds ? <div className="pointer-events-none absolute z-20 border-2 border-accent" style={{ left: selectionBounds.x, top: selectionBounds.y, width: selectionBounds.width, height: selectionBounds.height }} /> : null}
      {marquee ? (
        marquee.kind === "lasso" ? (
          <SelectionMarquee coverage={marquee.coverage ?? "full"} shape="polygon" points={marquee.points ?? []} />
        ) : (
          <SelectionMarquee coverage={marquee.coverage ?? "full"} shape="rect" rect={{ x: marquee.x ?? 0, y: marquee.y ?? 0, width: marquee.width ?? 0, height: marquee.height ?? 0 }} />
        )
      ) : null}
      <div
        className="absolute inset-0 z-30"
        onPointerDown={(event) => {
          if (!editable) return;
          const session = sessionRef.current;
          if (!session?.pointerDownScreen) return;
          const rect = event.currentTarget.getBoundingClientRect();
          const client = { x: event.clientX, y: event.clientY };
          pickInteraction.onCanvasPointerDown(client);
          session.pointerDownScreen(event.clientX - rect.left, event.clientY - rect.top, event.button, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey);
          session.renderFrame();
          paintOverlays();
        }}
        onPointerMove={(event) => {
          const session = sessionRef.current;
          if (!session?.pointerMoveScreen) return;
          const rect = event.currentTarget.getBoundingClientRect();
          const client = { x: event.clientX, y: event.clientY };
          pickInteraction.onCanvasPointerMove(client);
          session.pointerMoveScreen(event.clientX - rect.left, event.clientY - rect.top, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey);
          session.renderFrame();
          paintOverlays();
        }}
        onPointerUp={(event) => {
          const session = sessionRef.current;
          if (!session?.pointerUpScreen) return;
          const rect = event.currentTarget.getBoundingClientRect();
          const client = { x: event.clientX, y: event.clientY };
          pickInteraction.onCanvasPointerUp(client, { shift: event.shiftKey, ctrlOrMeta: event.metaKey || event.ctrlKey, alt: event.altKey });
          session.pointerUpScreen(event.clientX - rect.left, event.clientY - rect.top, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey);
          session.renderFrame();
          emitInteractionState();
        }}
        onPointerLeave={() => pickInteraction.onCanvasPointerLeave()}
        onWheel={(event) => {
          event.preventDefault();
          const session = sessionRef.current;
          if (!session?.wheelScreen) return;
          const rect = event.currentTarget.getBoundingClientRect();
          const delta = event.deltaMode === 1 ? event.deltaY * 16 : event.deltaMode === 2 ? event.deltaY * 400 : event.deltaY;
          session.wheelScreen(event.clientX - rect.left, event.clientY - rect.top, 0, delta, true);
          session.renderFrame();
          emitInteractionState();
        }}
      />
      {selectionBounds && editable ? (
        <SelectionAlignChrome
          bounds={selectionBounds}
          onAlign={(mode) => {
            const session = sessionRef.current;
            if (!session?.alignSelection) return;
            session.alignSelection(alignModeToDag(mode));
            commitGraphFixture();
            session.renderFrame();
            emitInteractionState();
          }}
        />
      ) : null}
      <GraphParamOverlays stateJson={paramStateJson} logicalW={overlaySize.w} logicalH={overlaySize.h} editable={editable} onParamChange={(nodeId, portId, value) => dispatch(nodeGraphActions.edit, { op: "setParam", nodeId, portId, value })} />
      <GraphStepperOverlays
        stateJson={stepperStateJson}
        logicalW={overlaySize.w}
        logicalH={overlaySize.h}
        editable={editable}
        onStepperChange={(widgetId, fieldKey, value) => dispatch(nodeGraphActions.edit, { op: "setStepper", widgetId, fieldKey, value })}
      />
      <GraphSliderOverlays stateJson={sliderStateJson} logicalW={overlaySize.w} logicalH={overlaySize.h} editable={editable} onSliderChange={(widgetId, value) => dispatch(nodeGraphActions.edit, { op: "setSlider", widgetId, value })} />
      <CanvasPickMenu request={pickInteraction.pickMenu} hoveredKey={pickInteraction.menuHoveredKey} onHoverKey={pickInteraction.onMenuHoverKey} onPick={pickInteraction.onMenuPick} onDismiss={pickInteraction.dismissPickMenu} />
      <ContextMenuController
        open={contextMenu != null}
        position={contextMenu ?? { x: 0, y: 0 }}
        items={contextMenuItems.map((item) => ({
          id: item.id,
          label: item.label,
          onSelect: () => dispatch(item.action, item.args),
        }))}
        onOpenChange={(open) => {
          if (!open) setContextMenu(null);
        }}
      />
    </div>
  );
}
//#endregion WasmGraphSurface

//#region DiagramFallback
function DiagramGraphFallback({
  scene,
  node,
  editable,
  parsedNodes,
  parsedEdges,
  findItems,
  contextMenuItems,
  onAction,
}: {
  readonly scene: NodeGraphScene;
  readonly node: UiComponentSceneNode;
  readonly editable: boolean;
  readonly parsedNodes: readonly MediaGraphNodeRecord[];
  readonly parsedEdges: readonly MediaGraphEdgeRecord[];
  readonly findItems: readonly GraphFindItem[];
  readonly contextMenuItems: readonly GraphContextMenuItem[];
  readonly onAction: (action: ActionDescriptor) => void;
}) {
  const viewport = useMemo(() => parseViewport(scene.viewportJson ?? "{}"), [scene.viewportJson]);
  const initialNodes = useMemo(() => mediaGraphNodesToDiagramNodes(parsedNodes), [parsedNodes]);
  const initialEdges = useMemo(() => mediaGraphEdgesToDiagramEdges(parsedEdges), [parsedEdges]);
  const [nodes, setNodes] = useState(initialNodes);
  const [edges, setEdges] = useState(initialEdges);
  useEffect(() => {
    setNodes(initialNodes);
    setEdges(initialEdges);
  }, [initialNodes, initialEdges]);

  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      onAction({ controllerId: node.controllerId, action, args: { surfaceId: node.surfaceId, ...args } });
    },
    [node.controllerId, node.surfaceId, onAction],
  );

  const containerRef = useRef<HTMLDivElement>(null);
  const [contextMenu, setContextMenu] = useState<{ readonly x: number; readonly y: number } | null>(null);

  return (
    <div
      ref={containerRef}
      className="relative h-full w-full"
      onDragOver={(event) => {
        if (editable && event.dataTransfer.types.includes(CATALOGUE_DRAG_MIME)) event.preventDefault();
      }}
      onDrop={(event: DragEvent<HTMLDivElement>) => {
        if (!editable) return;
        event.preventDefault();
        const raw = event.dataTransfer.getData(CATALOGUE_DRAG_MIME);
        if (!raw) return;
        let payload: { readonly programId?: string; readonly appId?: string };
        try {
          payload = JSON.parse(raw) as { readonly programId?: string; readonly appId?: string };
        } catch {
          return;
        }
        if (!payload.programId || !payload.appId) return;
        const rect = containerRef.current?.getBoundingClientRect();
        if (!rect) return;
        const x = (event.clientX - rect.left - viewport.x) / viewport.zoom;
        const y = (event.clientY - rect.top - viewport.y) / viewport.zoom;
        dispatch("spawnApp", { programId: payload.programId, appId: payload.appId, position: { x, y } });
      }}
      onContextMenu={(event) => {
        if (!editable || contextMenuItems.length === 0) return;
        event.preventDefault();
        setContextMenu({ x: event.clientX, y: event.clientY });
      }}
    >
      <Diagram
        className="h-full w-full"
        nodeTypes={mediaGraphNodeTypes}
        nodes={nodes}
        edges={edges}
        fitView={false}
        defaultViewport={viewport}
        minZoom={0.05}
        maxZoom={32}
        panOnDrag={[0, 1]}
        selectionOnDrag
        elementsSelectable
        nodesDraggable={editable}
        nodesConnectable={editable}
        edgesReconnectable={editable}
        onNodesChange={(nextNodes) => setNodes(nextNodes as Node<MediaGraphNodeData>[])}
        onEdgesChange={(nextEdges) => setEdges(nextEdges)}
        onNodeDragStop={
          editable
            ? (_event, draggedNode) => {
                dispatch(nodeGraphActions.edit, {
                  ops: [{ op: "move", nodeId: draggedNode.id, x: draggedNode.position.x, y: draggedNode.position.y }],
                });
              }
            : undefined
        }
        onConnect={
          editable
            ? (connection) => {
                if (!connection.source || !connection.target || !connection.sourceHandle || !connection.targetHandle) return;
                dispatch(nodeGraphActions.edit, {
                  ops: [
                    {
                      op: "connect",
                      sourceNodeId: connection.source,
                      sourcePortId: connection.sourceHandle,
                      targetNodeId: connection.target,
                      targetPortId: connection.targetHandle,
                    },
                  ],
                });
              }
            : undefined
        }
        onNodeClick={(_event, clickedNode) => {
          const record = parsedNodes.find((entry) => entry.id === clickedNode.id);
          if (record?.instanceId) dispatch("selectInstance", { instanceId: record.instanceId });
          dispatch(nodeGraphActions.select, { nodeIds: [clickedNode.id] });
        }}
        onNodeDoubleClick={(_event, clickedNode) => {
          const record = parsedNodes.find((entry) => entry.id === clickedNode.id);
          if (record?.instanceId) dispatch("openInstance", { instanceId: record.instanceId });
        }}
        onSelectionChange={(selection) => {
          const nodeIds = selection.nodes.map((entry) => entry.id);
          dispatch(nodeGraphActions.select, { nodeIds });
        }}
      />
      <ContextMenuController
        open={contextMenu != null}
        position={contextMenu ?? { x: 0, y: 0 }}
        items={contextMenuItems.map((item) => ({
          id: item.id,
          label: item.label,
          onSelect: () => dispatch(item.action, item.args),
        }))}
        onOpenChange={(open) => {
          if (!open) setContextMenu(null);
        }}
      />
    </div>
  );
}
//#endregion DiagramFallback

//#region NodeGraphHost
//#region Helpers
const useClient = () => {
  const [client, setClient] = useState(false);
  useEffect(() => setClient(true), []);
  return client;
};

function PresencePeersOverlay({ peers }: { readonly peers: readonly PresencePeer[] }) {
  if (peers.length === 0) return null;
  return (
    <div className="pointer-events-none absolute right-2 top-2 z-panel flex max-w-[14rem] flex-col gap-1 rounded border border-border/60 bg-window/90 px-2 py-1 text-xs shadow-sm">
      {peers.map((peer) => (
        <div key={peer.clientId} className="flex items-center justify-between gap-2 text-muted-foreground">
          <span className="truncate font-medium text-foreground">{peer.name}</span>
          <span>{peer.selectionCount} selected</span>
        </div>
      ))}
    </div>
  );
}
//#endregion Helpers

//#region Component
export function NodeGraphHost({ node, onAction }: ComponentSceneHostProps) {
  const scene = node.nodeGraph;
  const editable = scene?.editable ?? true;
  const parsedNodes = useMemo(() => parseJsonArray<MediaGraphNodeRecord>(scene?.nodesJson), [scene?.nodesJson]);
  const parsedEdges = useMemo(() => parseJsonArray<MediaGraphEdgeRecord>(scene?.edgesJson), [scene?.edgesJson]);
  const findItems = useMemo(() => parseJsonArray<GraphFindItem>(scene?.findItemsJson), [scene?.findItemsJson]);
  const contextMenuItems = useMemo(() => parseJsonArray<GraphContextMenuItem>(scene?.contextMenuJson), [scene?.contextMenuJson]);
  const presencePeers = useMemo(() => parseJsonArray<PresencePeer>(scene?.presencePeersJson), [scene?.presencePeersJson]);
  const isClient = useClient();
  const emptySceneLabel = useLabel("ui.host.emptyScene");

  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      onAction({ controllerId: node.controllerId, action, args: { surfaceId: node.surfaceId, ...args } });
    },
    [node.controllerId, node.surfaceId, onAction],
  );

  const findContext = useUIFindSafe();
  const onFindItemRef = useRef<(itemId: string) => void>(() => {});
  onFindItemRef.current = (itemId: string) => {
    const mediaNode = parsedNodes.find((entry) => entry.instanceId === itemId);
    if (!mediaNode) return;
    dispatch(nodeGraphActions.select, { nodeIds: [mediaNode.id] });
    dispatch("selectInstance", { instanceId: mediaNode.instanceId! });
  };

  useEffect(() => {
    if (!findContext?.setFindItems || findItems.length === 0) return;
    findContext.setFindItems(findItems);
  }, [findContext?.setFindItems, findItems]);

  useEffect(() => {
    if (!findContext?.setOnFindItem || findItems.length === 0) return;
    findContext.setOnFindItem((itemId) => onFindItemRef.current(itemId));
    return () => findContext.setOnFindItem?.(undefined);
  }, [findContext?.setOnFindItem, findItems.length]);

  if (!scene) return <div className="semio-node-graph-empty">{emptySceneLabel}</div>;

  const useFlowEngine = isFlowGraphScene(scene.capabilitiesJson) || Boolean(scene.fixtureJson);

  return (
    <div className="semio-node-graph-host relative h-full min-h-[24rem] w-full" data-surface-id={node.surfaceId} tabIndex={editable ? 0 : undefined} onKeyDown={(event) => handleGraphKeyboard(event, editable, parsedNodes, dispatch)}>
      {isClient ? (
        useFlowEngine ? (
          <FlowGraphCanvasHost scene={scene} surfaceId={node.surfaceId} controllerId={node.controllerId} editable={editable} contextMenuItems={contextMenuItems} onAction={onAction} />
        ) : (
          <WasmGraphSurface scene={scene} surfaceId={node.surfaceId} controllerId={node.controllerId} editable={editable} contextMenuItems={contextMenuItems} onAction={onAction} />
        )
      ) : (
        <DiagramGraphFallback scene={scene} node={node} editable={editable} parsedNodes={parsedNodes} parsedEdges={parsedEdges} findItems={findItems} contextMenuItems={contextMenuItems} onAction={onAction} />
      )}
      <PresencePeersOverlay peers={presencePeers} />
    </div>
  );
}
//#endregion Component
//#endregion NodeGraphHost

//#region 🔖graph-canvas-overlays

//#region DagOverlayTypes
export type DagLabelOverlayRow = {
  readonly id: string;
  readonly kind?: "port" | "node" | string;
  readonly text: string;
  readonly layout: "horizontal" | "vertical";
  readonly align?: "left" | "center" | "right";
  readonly x: number;
  readonly y: number;
  readonly nodeW: number;
  readonly nodeH: number;
  readonly fontScreenPx?: number;
  readonly maxScreenH?: number;
  readonly ghost?: boolean;
};

export type DagPreselectSnapshot = {
  readonly ids: readonly string[];
  readonly removedIds: readonly string[];
};

export type DagLabelOverlayInteraction = {
  readonly hoveredId: string | null;
  readonly selectedIds: readonly string[];
  readonly preselect: DagPreselectSnapshot;
  readonly dimmedIds?: readonly string[];
};

export type DagMarqueeOverlay = {
  readonly kind: "rect" | "lasso";
  readonly x?: number;
  readonly y?: number;
  readonly width?: number;
  readonly height?: number;
  readonly points?: readonly { readonly x: number; readonly y: number }[];
  readonly coverage?: "full" | "partial";
};

export type DagCameraState = { readonly x: number; readonly y: number; readonly zoom: number };

export type DagParamEditorRow = {
  readonly nodeId: string;
  readonly portId: string;
  readonly label: string;
  readonly type?: string;
  readonly value?: unknown;
  readonly default?: unknown;
  readonly x: number;
  readonly y: number;
  readonly w: number;
  readonly h: number;
};

export type DagStepperFieldRow = {
  readonly key: string;
  readonly label: string;
  readonly value: number;
  readonly step?: number;
  readonly x: number;
  readonly y: number;
  readonly w: number;
  readonly h: number;
};

export type DagStepperOverlayRow = {
  readonly widgetId: string;
  readonly fields: readonly DagStepperFieldRow[];
};

export type DagSliderOverlayRow = {
  readonly widgetId: string;
  readonly value: number;
  readonly min: number;
  readonly max: number;
  readonly step: number;
  readonly x: number;
  readonly y: number;
  readonly w: number;
  readonly h: number;
};

export type DagSelectionBounds = {
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
};
//#endregion DagOverlayTypes

//#region DagOverlayGeometry
export function parseDagCameraState(json: string): DagCameraState {
  try {
    const parsed = JSON.parse(json) as Partial<DagCameraState>;
    return { x: Number(parsed.x ?? 0), y: Number(parsed.y ?? 0), zoom: Number(parsed.zoom ?? 1) };
  } catch {
    return { x: 0, y: 0, zoom: 1 };
  }
}

export function dagWorldToScreen(camera: DagCameraState, width: number, height: number, wx: number, wy: number): { readonly x: number; readonly y: number } {
  const zoom = camera.zoom > 0 ? camera.zoom : 1;
  const cx = width * 0.5;
  const cy = height * 0.5;
  return { x: (wx - camera.x) * zoom + cx, y: (wy - camera.y) * zoom + cy };
}

export function dagScreenToWorld(camera: DagCameraState, width: number, height: number, sx: number, sy: number): { readonly x: number; readonly y: number } {
  const zoom = camera.zoom > 0 ? camera.zoom : 1;
  const cx = width * 0.5;
  const cy = height * 0.5;
  return { x: (sx - cx) / zoom + camera.x, y: (sy - cy) / zoom + camera.y };
}
//#endregion DagOverlayGeometry

//#region DagOverlayPaint
const DAG_LABEL_SCREEN_PX = 11;
const DAG_LABEL_FONT_FAMILY = "ui-sans-serif, system-ui, sans-serif";

export function parseDagNodeIdArray(json: string): string[] {
  try {
    const parsed = JSON.parse(json) as unknown;
    return Array.isArray(parsed) ? parsed.filter((value): value is string => typeof value === "string") : [];
  } catch {
    return [];
  }
}

export function parseDagPreselectJson(json: string): DagPreselectSnapshot {
  try {
    const parsed = JSON.parse(json) as { ids?: unknown; removedIds?: unknown };
    const ids = Array.isArray(parsed.ids) ? parsed.ids.filter((value): value is string => typeof value === "string") : [];
    const removedIds = Array.isArray(parsed.removedIds) ? parsed.removedIds.filter((value): value is string => typeof value === "string") : [];
    return { ids, removedIds };
  } catch {
    return { ids: [], removedIds: [] };
  }
}

export function dagElementInteractionChrome(selectionIds: Iterable<string>, preselection: DagPreselectSnapshot): { readonly selectedIds: Set<string>; readonly highlightedIds: Set<string> } {
  if (!preselection.ids.length && !preselection.removedIds.length) {
    return { selectedIds: new Set(selectionIds), highlightedIds: new Set() };
  }
  return { selectedIds: new Set(preselection.ids), highlightedIds: new Set(preselection.removedIds) };
}

export function parseDagLabelRows(stateJson: string): DagLabelOverlayRow[] {
  try {
    const parsed = JSON.parse(stateJson) as {
      readonly labels?: readonly Record<string, unknown>[];
      readonly rows?: readonly Record<string, unknown>[];
    };
    const raw = parsed.labels ?? parsed.rows ?? [];
    return raw
      .map((row) => {
        const text = typeof row.text === "string" ? row.text.trim() : "";
        if (!text) return null;
        const align = row.align === "left" || row.align === "right" || row.align === "center" ? row.align : undefined;
        return {
          id: String(row.id ?? ""),
          kind: typeof row.kind === "string" ? row.kind : undefined,
          text,
          layout: row.layout === "vertical" ? "vertical" : "horizontal",
          align,
          x: Number(row.x ?? 0),
          y: Number(row.y ?? 0),
          nodeW: Number(row.nodeW ?? row.width ?? 0),
          nodeH: Number(row.nodeH ?? row.height ?? 0),
          fontScreenPx: typeof row.fontScreenPx === "number" ? row.fontScreenPx : undefined,
          maxScreenH: typeof row.maxScreenH === "number" ? row.maxScreenH : undefined,
          ghost: row.ghost === true,
        } satisfies DagLabelOverlayRow;
      })
      .filter((row): row is DagLabelOverlayRow => row !== null);
  } catch {
    return [];
  }
}

function dagClampLabelFontPx(ctx: CanvasRenderingContext2D, text: string, targetPx: number, maxW: number, maxH: number): number {
  let px = Math.max(4, Math.round(targetPx));
  ctx.font = `${px}px ${DAG_LABEL_FONT_FAMILY}`;
  if (ctx.measureText(text).width <= maxW && px * 1.2 <= maxH) {
    return px;
  }
  let low = 4;
  let high = px;
  let best = 4;
  while (low <= high) {
    const mid = Math.floor((low + high) / 2);
    ctx.font = `${mid}px ${DAG_LABEL_FONT_FAMILY}`;
    const w = ctx.measureText(text).width;
    const h = mid * 1.2;
    if (w <= maxW && h <= maxH) {
      best = mid;
      low = mid + 1;
    } else {
      high = mid - 1;
    }
  }
  return best;
}

function dagClampPortLabelFontPx(ctx: CanvasRenderingContext2D, text: string, targetPx: number, maxW: number, maxH: number): number {
  let px = Math.max(8, Math.round(targetPx));
  ctx.font = `${px}px ${DAG_LABEL_FONT_FAMILY}`;
  if (ctx.measureText(text).width <= maxW && px * 1.25 <= maxH) {
    return px;
  }
  let low = 8;
  let high = px;
  let best = 8;
  while (low <= high) {
    const mid = Math.floor((low + high) / 2);
    ctx.font = `${mid}px ${DAG_LABEL_FONT_FAMILY}`;
    if (ctx.measureText(text).width <= maxW) {
      best = mid;
      low = mid + 1;
    } else {
      high = mid - 1;
    }
  }
  return best;
}

export function parseDagParamEditors(stateJson: string): readonly DagParamEditorRow[] {
  try {
    const parsed = JSON.parse(stateJson) as { readonly editors?: DagParamEditorRow[] };
    return parsed.editors ?? [];
  } catch {
    return [];
  }
}

export function parseDagStepperOverlays(stateJson: string): readonly DagStepperOverlayRow[] {
  try {
    const parsed = JSON.parse(stateJson) as { readonly steppers?: DagStepperOverlayRow[] };
    return parsed.steppers ?? [];
  } catch {
    return [];
  }
}

export function parseDagSliderOverlays(stateJson: string): readonly DagSliderOverlayRow[] {
  try {
    const parsed = JSON.parse(stateJson) as { readonly sliders?: DagSliderOverlayRow[] };
    return parsed.sliders ?? [];
  } catch {
    return [];
  }
}

export function parseDagOverlayCamera(stateJson: string): DagCameraState {
  try {
    const parsed = JSON.parse(stateJson) as { readonly camera?: DagCameraState; readonly width?: number; readonly height?: number };
    return parseDagCameraState(JSON.stringify(parsed.camera ?? {}));
  } catch {
    return { x: 0, y: 0, zoom: 1 };
  }
}
export function dagOverlayLabelFill(nodeId: string, ghost: boolean, hoveredId: string | null, chrome: { readonly selectedIds: Set<string>; readonly highlightedIds: Set<string> }, dimmedIds: readonly string[] = []): string {
  if (ghost) return "var(--color-secondary)";
  if (dimmedIds.includes(nodeId)) return "var(--color-border)";
  if (chrome.selectedIds.has(nodeId)) return "var(--color-foreground)";
  if (chrome.highlightedIds.has(nodeId)) return "var(--color-secondary)";
  if (hoveredId === nodeId) return "var(--color-foreground)";
  return "var(--color-muted-foreground)";
}

export function paintDagLabelOverlays(stateJson: string, canvas: HTMLCanvasElement, logicalW: number, logicalH: number, dpr: number, interaction: DagLabelOverlayInteraction): void {
  let state: { readonly camera?: DagCameraState; readonly width?: number; readonly height?: number; readonly labels?: readonly DagLabelOverlayRow[] };
  try {
    state = JSON.parse(stateJson) as typeof state;
  } catch {
    return;
  }
  const ctx = canvas.getContext("2d");
  if (!ctx) return;
  const pixelW = Math.max(1, Math.round(logicalW * dpr));
  const pixelH = Math.max(1, Math.round(logicalH * dpr));
  if (canvas.width !== pixelW || canvas.height !== pixelH) {
    canvas.width = pixelW;
    canvas.height = pixelH;
  }
  canvas.style.width = `${logicalW}px`;
  canvas.style.height = `${logicalH}px`;
  ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
  ctx.clearRect(0, 0, logicalW, logicalH);
  const zoom = Math.max(0.05, Number(state.camera?.zoom) || 1);
  const camera = {
    x: Number(state.camera?.x) || 0,
    y: Number(state.camera?.y) || 0,
    zoom,
  };
  const viewportW = Number(state.width) || logicalW;
  const viewportH = Number(state.height) || logicalH;
  const chrome = dagElementInteractionChrome(interaction.selectedIds, interaction.preselect);
  const dimmedIds = interaction.dimmedIds ?? [];
  const rows = state.labels ?? parseDagLabelRows(stateJson);
  const inset = 0.88;
  for (const row of rows) {
    const anchor = dagWorldToScreen(camera, viewportW, viewportH, row.x, row.y);
    const isPort = row.kind === "port" || row.align === "left" || row.align === "right";
    const maxW = Math.max(4, Number(row.nodeW) * zoom * inset);
    const maxH = Math.max(4, isPort && Number.isFinite(Number(row.maxScreenH)) && Number(row.maxScreenH) > 0 ? Number(row.maxScreenH) : Number(row.nodeH) * zoom * inset);
    const fontScreenPx = Number(row.fontScreenPx);
    const targetPx = Number.isFinite(fontScreenPx) && fontScreenPx > 0 ? fontScreenPx : DAG_LABEL_SCREEN_PX;
    const fontPx = isPort ? dagClampPortLabelFontPx(ctx, row.text, targetPx, maxW, maxH) : dagClampLabelFontPx(ctx, row.text, targetPx, maxW, maxH);
    ctx.font = `${fontPx}px ${DAG_LABEL_FONT_FAMILY}`;
    ctx.fillStyle = dagOverlayLabelFill(row.id, row.ghost === true, interaction.hoveredId, chrome, dimmedIds);
    ctx.globalAlpha = row.ghost ? 0.85 : dimmedIds.includes(row.id) ? 0.5 : 1;
    if (row.layout === "vertical") {
      ctx.save();
      ctx.translate(anchor.x, anchor.y);
      ctx.rotate(-Math.PI / 2);
      ctx.textAlign = "center";
      ctx.textBaseline = "middle";
      ctx.fillText(row.text, 0, 0);
      ctx.restore();
    } else {
      const align = row.align === "left" || row.align === "right" ? row.align : "center";
      ctx.textAlign = align;
      ctx.textBaseline = "middle";
      ctx.fillText(row.text, anchor.x, anchor.y);
    }
    ctx.globalAlpha = 1;
  }
}

export function parseDagSelectionUnionBoundsScreen(json: string): DagSelectionBounds | null {
  try {
    const parsed = JSON.parse(json) as Partial<DagSelectionBounds>;
    if (parsed.x == null || parsed.y == null || parsed.width == null || parsed.height == null) return null;
    return { x: parsed.x, y: parsed.y, width: parsed.width, height: parsed.height };
  } catch {
    return null;
  }
}

export function computeDagMarqueeOverlay(pointsJson: string, crossing: boolean, method: string): DagMarqueeOverlay | null {
  let points: { readonly x: number; readonly y: number }[] = [];
  try {
    points = JSON.parse(pointsJson) as { readonly x: number; readonly y: number }[];
  } catch {
    return null;
  }
  if (points.length < 2) return null;
  const coverage = crossing ? "partial" : "full";
  if (method === "lasso") return { kind: "lasso", points, coverage };
  const xs = points.map((point) => point.x);
  const ys = points.map((point) => point.y);
  const x = Math.min(...xs);
  const y = Math.min(...ys);
  return { kind: "rect", x, y, width: Math.max(...xs) - x, height: Math.max(...ys) - y, coverage };
}

export function sceneToSyncJson(scene: NodeGraphScene): string {
  return JSON.stringify(scene);
}

//#region DagDomOverlays
export function GraphParamOverlays({
  stateJson,
  logicalW,
  logicalH,
  editable,
  onParamChange,
}: {
  readonly stateJson: string;
  readonly logicalW: number;
  readonly logicalH: number;
  readonly editable: boolean;
  readonly onParamChange: (nodeId: string, portId: string, value: unknown) => void;
}) {
  const camera = parseDagOverlayCamera(stateJson);
  const editors = parseDagParamEditors(stateJson);
  if (editors.length === 0) return null;
  return (
    <div className="pointer-events-none absolute inset-0 z-45">
      {editors.map((editor) => {
        const screen = dagWorldToScreen(camera, logicalW, logicalH, editor.x, editor.y);
        const w = editor.w * camera.zoom;
        const h = editor.h * camera.zoom;
        return (
          <input
            key={`${editor.nodeId}:${editor.portId}`}
            className="pointer-events-auto absolute rounded border border-border bg-panel px-1 font-mono text-[10px] text-foreground"
            style={{ left: screen.x - w / 2, top: screen.y - h / 2, width: w, height: h }}
            defaultValue={String(editor.value ?? editor.default ?? "")}
            readOnly={!editable}
            onPointerDown={(event) => event.stopPropagation()}
            onChange={(event) => onParamChange(editor.nodeId, editor.portId, event.target.value)}
          />
        );
      })}
    </div>
  );
}

export function GraphStepperOverlays({
  stateJson,
  logicalW,
  logicalH,
  editable,
  onStepperChange,
}: {
  readonly stateJson: string;
  readonly logicalW: number;
  readonly logicalH: number;
  readonly editable: boolean;
  readonly onStepperChange: (widgetId: string, fieldKey: string, value: number) => void;
}) {
  const camera = parseDagOverlayCamera(stateJson);
  const steppers = parseDagStepperOverlays(stateJson);
  if (steppers.length === 0) return null;
  return (
    <div className="pointer-events-none absolute inset-0 z-45">
      {steppers.flatMap((stepper) =>
        stepper.fields.map((field) => {
          const screen = dagWorldToScreen(camera, logicalW, logicalH, field.x, field.y);
          const w = field.w * camera.zoom;
          const h = field.h * camera.zoom;
          return (
            <input
              key={`${stepper.widgetId}:${field.key}`}
              type="number"
              className="pointer-events-auto absolute rounded border border-border bg-panel px-1 font-mono text-[10px] text-foreground"
              style={{ left: screen.x, top: screen.y - h / 2, width: w, height: h }}
              defaultValue={field.value}
              step={field.step ?? 1}
              readOnly={!editable}
              onPointerDown={(event) => event.stopPropagation()}
              onChange={(event) => onStepperChange(stepper.widgetId, field.key, Number(event.target.value))}
            />
          );
        }),
      )}
    </div>
  );
}

export function GraphSliderOverlays({
  stateJson,
  logicalW,
  logicalH,
  editable,
  onSliderChange,
  onSliderPointerDown,
  onSliderPointerUp,
}: {
  readonly stateJson: string;
  readonly logicalW: number;
  readonly logicalH: number;
  readonly editable: boolean;
  readonly onSliderChange: (widgetId: string, value: number) => void;
  readonly onSliderPointerDown?: () => void;
  readonly onSliderPointerUp?: () => void;
}) {
  const camera = parseDagOverlayCamera(stateJson);
  const sliders = parseDagSliderOverlays(stateJson);
  if (sliders.length === 0) return null;
  return (
    <div className="pointer-events-none absolute inset-0 z-45">
      {sliders.map((slider) => {
        const screen = dagWorldToScreen(camera, logicalW, logicalH, slider.x, slider.y);
        const w = slider.w * camera.zoom;
        const h = Math.max(slider.h * camera.zoom, 16);
        return (
          <div key={slider.widgetId} className="pointer-events-auto absolute flex items-center px-1" style={{ left: screen.x - w / 2, top: screen.y - h / 2, width: w, height: h }} onPointerDown={(event) => event.stopPropagation()}>
            <Slider
              className="w-full min-w-0"
              max={slider.max}
              min={slider.min}
              step={slider.step}
              value={[slider.value]}
              disabled={!editable}
              onValueChange={(values) => onSliderChange(slider.widgetId, values[0] ?? slider.value)}
              onPointerDown={onSliderPointerDown}
              onPointerUp={onSliderPointerUp}
              onPointerCancel={onSliderPointerUp}
            />
          </div>
        );
      })}
    </div>
  );
}

const ALIGN_MODES = [
  { id: "left", label: "⬅" },
  { id: "center-h", label: "↔" },
  { id: "right", label: "➡" },
  { id: "top", label: "⬆" },
  { id: "center-v", label: "↕" },
  { id: "bottom", label: "⬇" },
] as const;

export function alignModeToDag(mode: string): string {
  const map: Record<string, string> = {
    left: "alignLeft",
    right: "alignRight",
    top: "alignTop",
    bottom: "alignBottom",
    "center-h": "alignHorizontal",
    "center-v": "alignVertical",
  };
  return map[mode] ?? mode;
}

export function SelectionAlignChrome({ bounds, onAlign }: { readonly bounds: DagSelectionBounds; readonly onAlign: (mode: string) => void }) {
  return (
    <div className="pointer-events-auto absolute z-50 flex gap-0.5 rounded border border-border bg-panel p-0.5 shadow-sm" style={{ left: bounds.x, top: Math.max(0, bounds.y - 28) }}>
      {ALIGN_MODES.map((mode) => (
        <button key={mode.id} type="button" className="size-5 rounded text-xs hover:bg-active-base" aria-label={mode.id} onPointerDown={(event) => event.stopPropagation()} onClick={() => onAlign(mode.id)}>
          {mode.label}
        </button>
      ))}
    </div>
  );
}
//#endregion DagDomOverlays
//#endregion DagOverlayPaint
//#endregion 🔖graph-canvas-overlays

//#region 🔖flow-graph-canvas-host

//#region Sync
// @emoji 🎥 `applyCamera` must stay false for every resync after the first: FlowWasmSession never
// reports its live camera back into the document (`cameraJson` is unimplemented, see the wheel
// handler below), so `scene.viewportJson` is frozen at its initial value for the whole session —
// applying it on every edit-triggered resync would snap the user's camera back on every commit.
function syncFlowSessionFromScene(session: FlowWasmSession, scene: NodeGraphScene, applyCamera: boolean): void {
  if (scene.operatorsJson) session.setNeuronKindInfosJson(scene.operatorsJson);
  if (scene.fixtureJson) session.loadFixtureJson(scene.fixtureJson);
  if (scene.selectionJson) session.setSelection(scene.selectionJson);
  if (scene.previewOffJson) session.setPreviewOff(scene.previewOffJson);
  if (scene.catalogueJson) session.setCatalogueJson(scene.catalogueJson);
  if (scene.computingJson) session.setComputingProgress(scene.computingJson);
  if (scene.lodJson) {
    try {
      const lod = JSON.parse(scene.lodJson) as { readonly automatic?: boolean; readonly forcedLabel?: string };
      session.setAutomaticLod(lod.automatic !== false);
      if (lod.forcedLabel) session.setForcedDrawLodLabel(lod.forcedLabel);
    } catch {
      /* ignore */
    }
  }
  if (!applyCamera) return;
  try {
    const viewport = JSON.parse(scene.viewportJson) as { readonly x?: number; readonly y?: number; readonly zoom?: number };
    session.setCamera(viewport.x ?? 0, viewport.y ?? 0, viewport.zoom ?? 1);
  } catch {
    /* ignore */
  }
}
//#endregion Sync

//#region Spotlight
function SpotlightOverlay({ previewText, onCommit, onDismiss }: { readonly previewText: string; readonly onCommit: () => void; readonly onDismiss: () => void }) {
  const previewLabel = useLabel("ui.host.preview");
  if (!previewText.trim()) return null;
  return (
    <div className="pointer-events-auto absolute inset-x-4 bottom-4 z-60 rounded border border-border bg-panel p-3 shadow-lg">
      <div className="mb-2 text-xs font-medium text-muted-foreground">{previewLabel}</div>
      <pre className="max-h-40 overflow-auto whitespace-pre-wrap font-mono text-xs text-foreground">{previewText}</pre>
      <div className="mt-2 flex justify-end gap-2">
        <button type="button" className="rounded px-2 py-1 text-xs hover:bg-active-base" onClick={onDismiss}>
          Dismiss
        </button>
        <button type="button" className="rounded bg-accent px-2 py-1 text-xs text-accent-foreground" onClick={onCommit}>
          Commit
        </button>
      </div>
    </div>
  );
}
//#endregion Spotlight

//#region FlowGraphCanvasHost
export function FlowGraphCanvasHost({
  scene,
  surfaceId,
  controllerId,
  editable,
  contextMenuItems,
  onAction,
}: {
  readonly scene: NodeGraphScene;
  readonly surfaceId: string;
  readonly controllerId: string;
  readonly editable: boolean;
  readonly contextMenuItems: readonly GraphContextMenuItem[];
  readonly onAction: (action: ActionDescriptor) => void;
}) {
  const sessionRef = useRef<FlowWasmSession | null>(null);
  const gpuCanvasRef = useRef<HTMLCanvasElement | null>(null);
  const labelCanvasRef = useRef<HTMLCanvasElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const [contextMenu, setContextMenu] = useState<{ readonly x: number; readonly y: number; readonly widgetId?: string } | null>(null);
  const [selectionBounds, setSelectionBounds] = useState<ReturnType<typeof parseDagSelectionUnionBoundsScreen>>(null);
  const [marquee, setMarquee] = useState<ReturnType<typeof computeDagMarqueeOverlay>>(null);
  const [labelStateJson, setLabelStateJson] = useState("{}");
  const [paramStateJson, setParamStateJson] = useState("{}");
  const [stepperStateJson, setStepperStateJson] = useState("{}");
  const [sliderStateJson, setSliderStateJson] = useState("{}");
  const [previewText, setPreviewText] = useState("");
  const [containerSize, setContainerSize] = useState({ w: 800, h: 600 });
  const [sessionReady, setSessionReady] = useState(false);
  const sceneSignature = useMemo(() => JSON.stringify(scene), [scene]);
  // Always holds the latest `scene` without forcing effects to depend on (and re-run per) it.
  const sceneRef = useRef(scene);
  sceneRef.current = scene;

  useEffect(() => {
    console.log("[DEBUG] FlowGraphCanvasHost mounted", { surfaceId, controllerId });
    return () => console.log("[DEBUG] FlowGraphCanvasHost UNMOUNTED", { surfaceId, controllerId });
  }, [surfaceId, controllerId]);

  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      onAction({ controllerId, action, args: { surfaceId, ...args } });
    },
    [controllerId, onAction, surfaceId],
  );

  const commitFixture = useCallback(() => {
    const session = sessionRef.current;
    if (!session) return;
    try {
      const fixtureJson = session.fixtureJson();
      console.log("[DEBUG] commitFixture: dispatching setFixture, isGestureActive=", isGestureActiveRef.current, "len=", fixtureJson.length);
      dispatch(nodeGraphActions.edit, { ops: [{ op: "setFixture", fixtureJson }] });
      session.evaluateSync();
    } catch {
      /* session not ready */
    }
  }, [dispatch]);

  // A continuous gesture (e.g. dragging a slider) fires many onValueChange ticks per second, each
  // committing the whole document through an async plugin round-trip; concurrent in-flight commits
  // can resolve out of order, and the scene-resync effect below would apply whichever one lands
  // last — visibly reverting the drag mid-gesture. isGestureActiveRef suppresses that resync while
  // a gesture is active, and commitFixtureThrottled caps how many concurrent commits are in flight.
  const isGestureActiveRef = useRef(false);
  const lastCommitAtRef = useRef(0);
  const pendingCommitTimeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const GESTURE_COMMIT_THROTTLE_MS = 80;

  const commitFixtureThrottled = useCallback(() => {
    if (pendingCommitTimeoutRef.current != null) {
      clearTimeout(pendingCommitTimeoutRef.current);
      pendingCommitTimeoutRef.current = null;
    }
    const elapsed = Date.now() - lastCommitAtRef.current;
    if (elapsed >= GESTURE_COMMIT_THROTTLE_MS) {
      lastCommitAtRef.current = Date.now();
      commitFixture();
    } else {
      pendingCommitTimeoutRef.current = setTimeout(() => {
        pendingCommitTimeoutRef.current = null;
        lastCommitAtRef.current = Date.now();
        commitFixture();
      }, GESTURE_COMMIT_THROTTLE_MS - elapsed);
    }
  }, [commitFixture]);

  const handleGesturePointerDown = useCallback(() => {
    console.log("[DEBUG] gesture pointerDown: isGestureActiveRef -> true");
    isGestureActiveRef.current = true;
  }, []);

  const handleGesturePointerUp = useCallback(() => {
    console.log("[DEBUG] gesture pointerUp: isGestureActiveRef -> false, firing final commitFixture");
    isGestureActiveRef.current = false;
    if (pendingCommitTimeoutRef.current != null) {
      clearTimeout(pendingCommitTimeoutRef.current);
      pendingCommitTimeoutRef.current = null;
    }
    lastCommitAtRef.current = Date.now();
    commitFixture();
  }, [commitFixture]);

  useEffect(() => {
    return () => {
      if (pendingCommitTimeoutRef.current != null) clearTimeout(pendingCommitTimeoutRef.current);
    };
  }, []);

  const paintOverlays = useCallback(() => {
    const session = sessionRef.current;
    const labelCanvas = labelCanvasRef.current;
    const container = containerRef.current;
    if (!session || !labelCanvas || !container) return;
    const rect = container.getBoundingClientRect();
    const dpr = globalThis.devicePixelRatio || 1;
    setContainerSize({ w: rect.width, h: rect.height });
    try {
      const labelJson = session.labelOverlayPaintStateJson();
      setLabelStateJson(labelJson);
      const selectedIds = parseDagNodeIdArray(session.selectedWidgetIds());
      const preselect = parseDagPreselectJson(session.preselectWidgetIdsJson());
      const dimmedIds = parseDagNodeIdArray(session.previewOffWidgetIds());
      paintDagLabelOverlays(labelJson, labelCanvas, rect.width, rect.height, dpr, {
        hoveredId: session.hoveredWidgetId() ?? null,
        selectedIds,
        preselect,
        dimmedIds,
      });
      setParamStateJson(session.paramOverlayPaintStateJson());
      setStepperStateJson(session.stepperOverlayStateJson());
      setSliderStateJson(session.sliderOverlayStateJson());
    } catch {
      /* gpu not ready */
    }
    setSelectionBounds(parseDagSelectionUnionBoundsScreen(session.selectionUnionBoundsScreenJson()));
    setMarquee(computeDagMarqueeOverlay(session.selectionPreviewPointsJson(), session.selectionPreviewCrossing(), "rectangle"));
    try {
      setPreviewText(session.previewText());
    } catch {
      setPreviewText("");
    }
  }, []);

  const emitInteractionState = useCallback(() => {
    const session = sessionRef.current;
    if (!session) return;
    try {
      const nodeIds = JSON.parse(session.selectedWidgetIds()) as string[];
      dispatch(nodeGraphActions.select, { nodeIds });
      const hovered = session.hoveredWidgetId();
      const channelJson = session.hoveredChannelJson();
      dispatch(nodeGraphActions.hover, { hoverJson: hovered ? channelJson : null });
    } catch {
      /* session not ready */
    }
    paintOverlays();
  }, [dispatch, paintOverlays]);

  useEffect(() => {
    let cancelled = false;
    void createFlowSession().then((session) => {
      if (cancelled) return;
      sessionRef.current = session;
      setSessionReady(true);
    });
    return () => {
      cancelled = true;
    };
  }, []);

  // Attaches the GPU canvas exactly once per session (NOT per document edit — `scene` must stay out
  // of this effect's deps). It used to depend on `scene`, so it re-ran `attachCanvas` on every single
  // commit (including every slider tick): the wasm session rejects a second attach ("canvas surface
  // already attached"), and because the cleanup below was returned from inside the `.then()` instead
  // of from the effect itself, React never saw it — every re-run leaked its ResizeObserver/rAF loop
  // and could disrupt the live GPU surface, which is what read as the whole view "resetting".
  useEffect(() => {
    const session = sessionRef.current;
    const canvas = gpuCanvasRef.current;
    const container = containerRef.current;
    if (!session || !canvas || !container || !sessionReady) return;
    const rect = container.getBoundingClientRect();
    const dpr = globalThis.devicePixelRatio || 1;
    let raf = 0;
    let cancelled = false;
    let cleanupAttached: (() => void) | undefined;
    console.log("[DEBUG] attachCanvas effect: attaching (should log ONCE per session)");
    session
      .attachCanvas(canvas, Math.round(rect.width), Math.round(rect.height), dpr)
      .then(() => {
        if (cancelled) return;
        console.log("[DEBUG] attachCanvas effect: attached OK, applying initial camera");
        syncFlowSessionFromScene(session, sceneRef.current, true);
        const resize = () => {
          const next = container.getBoundingClientRect();
          const nextDpr = globalThis.devicePixelRatio || 1;
          session.setSize(Math.round(next.width), Math.round(next.height), nextDpr);
          session.renderFrame();
          paintOverlays();
        };
        resize();
        const ro = new ResizeObserver(resize);
        ro.observe(container);
        const tick = () => {
          session.renderFrame();
          raf = requestAnimationFrame(tick);
        };
        raf = requestAnimationFrame(tick);
        cleanupAttached = () => {
          ro.disconnect();
          if (raf) cancelAnimationFrame(raf);
        };
      })
      .catch((err) => {
        /* already attached (e.g. a stale re-run) or transient failure; nothing to clean up */
        console.log("[DEBUG] attachCanvas effect: attach FAILED/REJECTED", err);
      });
    return () => {
      console.log("[DEBUG] attachCanvas effect: cleanup running (effect re-run or unmount)");
      cancelled = true;
      cleanupAttached?.();
    };
  }, [sessionReady, paintOverlays]);

  useEffect(() => {
    const session = sessionRef.current;
    if (!session || !sessionReady) return;
    // Skip while a gesture (e.g. slider drag) is active: an in-flight commit's response landing
    // mid-gesture would otherwise reload the fixture and visibly revert the live local edit.
    if (isGestureActiveRef.current) {
      console.log("[DEBUG] resync effect: SKIPPED (gesture active), sceneSignature len=", sceneSignature.length);
      return;
    }
    console.log("[DEBUG] resync effect: APPLYING syncFlowSessionFromScene, sceneSignature len=", sceneSignature.length, "fixtureJson len=", scene.fixtureJson?.length);
    syncFlowSessionFromScene(session, scene, false);
    session.renderFrame();
    paintOverlays();
  }, [sceneSignature, paintOverlays, scene, sessionReady]);

  useCanvasAppearanceSync(() => {
    syncSessionCanvasTheme(sessionRef.current);
  });

  const pickInteraction = useCanvasPickInteraction({
    resolveTargetsAtClient: (client) => {
      const session = sessionRef.current;
      const container = containerRef.current;
      if (!session || !container) return [];
      const rect = container.getBoundingClientRect();
      const sx = client.x - rect.left;
      const sy = client.y - rect.top;
      try {
        return JSON.parse(session.pickTargetsAtScreenJson(sx, sy)) as CanvasPickTarget[];
      } catch {
        return [];
      }
    },
    onHoverFocus: (focus) => {
      const session = sessionRef.current;
      if (!session) return;
      const target = focus.target;
      if (!target) {
        session.setHover?.(null);
      } else if (target.portId) {
        session.setHoverChannel?.(target.id, target.portId);
      } else {
        session.setHover?.(target.id);
      }
      session.renderFrame();
      paintOverlays();
    },
    onSelectTarget: () => {
      emitInteractionState();
    },
  });

  useEffect(() => {
    const onKeyDown = (event: KeyboardEvent) => {
      const session = sessionRef.current;
      if (!session || !editable) return;
      const mod = event.metaKey || event.ctrlKey;
      if (mod && event.key === "z" && !event.shiftKey) {
        event.preventDefault();
        if (session.undo()) {
          commitFixture();
          emitInteractionState();
        }
        return;
      }
      if (mod && (event.key === "Z" || (event.key === "z" && event.shiftKey))) {
        event.preventDefault();
        if (session.redo()) {
          commitFixture();
          emitInteractionState();
        }
        return;
      }
      if (mod && event.key === "a") {
        event.preventDefault();
        session.selectAll();
        emitInteractionState();
        return;
      }
      if (event.key === "Delete" || event.key === "Backspace") {
        if ((event.target as HTMLElement).tagName === "INPUT" || (event.target as HTMLElement).tagName === "TEXTAREA") return;
        event.preventDefault();
        session.deleteSelection();
        commitFixture();
        emitInteractionState();
      }
    };
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [commitFixture, editable, emitInteractionState]);

  const clearGhostPreview = useCallback(() => {
    const session = sessionRef.current;
    if (!session) return;
    session.clearGhostWidget();
    session.renderFrame();
    paintOverlays();
  }, [paintOverlays]);

  const onDragOverCanvas = useCallback(
    (event: DragEvent<HTMLDivElement>) => {
      if (!editable) return;
      event.preventDefault();
      const session = sessionRef.current;
      const container = containerRef.current;
      if (!session || !container) return;
      const encoded = getActiveCatalogueDragPayload();
      if (!encoded) return;
      const catalogueApp = parseCatalogueAppDragPayload(encoded);
      if (!catalogueApp) return;
      const rect = container.getBoundingClientRect();
      const sx = event.clientX - rect.left;
      const sy = event.clientY - rect.top;
      let world = { x: sx, y: sy };
      try {
        const parsed = JSON.parse(session.worldFromScreen(sx, sy)) as { readonly x?: number; readonly y?: number };
        world = { x: parsed.x ?? sx, y: parsed.y ?? sy };
      } catch {
        const camera = parseDagOverlayCamera(labelStateJson);
        world = dagScreenToWorld(camera, rect.width, rect.height, sx, sy);
      }
      session.setGhostWidget(catalogueGhostDescriptorJson(catalogueApp), world.x, world.y);
      session.renderFrame();
      paintOverlays();
    },
    [editable, labelStateJson, paintOverlays],
  );

  const onDrop = useCallback(
    (event: DragEvent<HTMLDivElement>) => {
      if (!editable) return;
      clearGhostPreview();
      const raw = event.dataTransfer.getData(CATALOGUE_DRAG_MIME) || event.dataTransfer.getData("text/plain") || getActiveCatalogueDragPayload() || "";
      if (!raw) return;
      event.preventDefault();
      const session = sessionRef.current;
      const container = containerRef.current;
      if (!session || !container) return;
      const rect = container.getBoundingClientRect();
      const sx = event.clientX - rect.left;
      const sy = event.clientY - rect.top;
      let world = { x: sx, y: sy };
      try {
        const parsed = JSON.parse(session.worldFromScreen(sx, sy)) as { readonly x?: number; readonly y?: number };
        world = { x: parsed.x ?? sx, y: parsed.y ?? sy };
      } catch {
        const camera = parseDagOverlayCamera(labelStateJson);
        world = dagScreenToWorld(camera, rect.width, rect.height, sx, sy);
      }
      const catalogueApp = parseCatalogueAppDragPayload(raw);
      if (catalogueApp) {
        dispatch("spawnApp", { programId: catalogueApp.programId, appId: catalogueApp.appId, position: { x: world.x, y: world.y } });
        return;
      }
      try {
        const descriptor = raw.startsWith("{") ? raw : JSON.stringify({ kind: raw });
        session.addWidget(descriptor, world.x, world.y);
        commitFixture();
        emitInteractionState();
      } catch {
        /* invalid descriptor */
      }
    },
    [clearGhostPreview, commitFixture, dispatch, editable, emitInteractionState, labelStateJson],
  );

  const openHoveredInstance = useCallback(() => {
    const session = sessionRef.current;
    if (!session) return;
    const instanceId = resolveFixtureWidgetInstanceId(scene.fixtureJson, session.hoveredWidgetId());
    if (instanceId) dispatch("openInstance", { instanceId });
  }, [dispatch, scene.fixtureJson]);

  useEffect(() => clearGhostPreview, [clearGhostPreview]);

  return (
    <div
      ref={containerRef}
      className="relative h-full w-full"
      onDragOver={onDragOverCanvas}
      onDragLeave={() => {
        if (!editable) return;
        clearGhostPreview();
      }}
      onDrop={onDrop}
      onContextMenu={(event) => {
        if (!editable || contextMenuItems.length === 0) return;
        event.preventDefault();
        setContextMenu({ x: event.clientX, y: event.clientY, widgetId: sessionRef.current?.hoveredWidgetId() });
      }}
    >
      <canvas ref={gpuCanvasRef} className="absolute inset-0 block h-full w-full" />
      <canvas ref={labelCanvasRef} className="pointer-events-none absolute inset-0 z-40" />
      <GraphParamOverlays
        stateJson={paramStateJson}
        logicalW={containerSize.w}
        logicalH={containerSize.h}
        editable={editable}
        onParamChange={(nodeId, portId, value) => {
          const session = sessionRef.current;
          if (!session) return;
          session.setNeuronParams(nodeId, JSON.stringify({ [portId]: value }));
          commitFixture();
          paintOverlays();
        }}
      />
      <GraphStepperOverlays
        stateJson={stepperStateJson}
        logicalW={containerSize.w}
        logicalH={containerSize.h}
        editable={editable}
        onStepperChange={(widgetId, fieldKey, value) => {
          sessionRef.current?.setStepperFieldValue(widgetId, fieldKey, value);
          commitFixture();
          paintOverlays();
        }}
      />
      <GraphSliderOverlays
        stateJson={sliderStateJson}
        logicalW={containerSize.w}
        logicalH={containerSize.h}
        editable={editable}
        onSliderChange={(widgetId, value) => {
          console.log("[DEBUG] onSliderChange (TS handler fired)", widgetId, value, "isGestureActive=", isGestureActiveRef.current);
          sessionRef.current?.setSliderValue(widgetId, value);
          commitFixtureThrottled();
          paintOverlays();
        }}
        onSliderPointerDown={handleGesturePointerDown}
        onSliderPointerUp={handleGesturePointerUp}
      />
      {selectionBounds ? (
        <>
          <div className="pointer-events-none absolute z-20 border-2 border-accent" style={{ left: selectionBounds.x, top: selectionBounds.y, width: selectionBounds.width, height: selectionBounds.height }} />
          {editable ? (
            <SelectionAlignChrome
              bounds={selectionBounds}
              onAlign={(mode) => {
                sessionRef.current?.alignSelection(mode);
                commitFixture();
                paintOverlays();
              }}
            />
          ) : null}
        </>
      ) : null}
      {marquee ? (
        marquee.kind === "lasso" ? (
          <SelectionMarquee coverage={marquee.coverage ?? "full"} shape="polygon" points={marquee.points ?? []} />
        ) : (
          <SelectionMarquee coverage={marquee.coverage ?? "full"} shape="rect" rect={{ x: marquee.x ?? 0, y: marquee.y ?? 0, width: marquee.width ?? 0, height: marquee.height ?? 0 }} />
        )
      ) : null}
      <div
        className="absolute inset-0 z-30"
        onPointerDown={(event) => {
          if (!editable) return;
          const session = sessionRef.current;
          if (!session) return;
          const rect = event.currentTarget.getBoundingClientRect();
          const client = { x: event.clientX, y: event.clientY };
          pickInteraction.onCanvasPointerDown(client);
          session.pointerDownScreen(event.clientX - rect.left, event.clientY - rect.top, event.button, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey, event.button === 1 || event.buttons === 4);
          session.renderFrame();
          paintOverlays();
        }}
        onPointerMove={(event) => {
          const session = sessionRef.current;
          if (!session) return;
          const rect = event.currentTarget.getBoundingClientRect();
          const client = { x: event.clientX, y: event.clientY };
          pickInteraction.onCanvasPointerMove(client);
          session.pointerMoveScreen(event.clientX - rect.left, event.clientY - rect.top, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey);
          session.renderFrame();
          paintOverlays();
        }}
        onPointerUp={(event) => {
          const session = sessionRef.current;
          if (!session) return;
          const rect = event.currentTarget.getBoundingClientRect();
          const client = { x: event.clientX, y: event.clientY };
          pickInteraction.onCanvasPointerUp(client, { shift: event.shiftKey, ctrlOrMeta: event.metaKey || event.ctrlKey, alt: event.altKey });
          session.pointerUpScreen(event.clientX - rect.left, event.clientY - rect.top, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey);
          session.renderFrame();
          commitFixture();
          emitInteractionState();
        }}
        onPointerLeave={() => pickInteraction.onCanvasPointerLeave()}
        onDoubleClick={openHoveredInstance}
        onWheel={(event) => {
          event.preventDefault();
          const session = sessionRef.current;
          if (!session) return;
          const rect = event.currentTarget.getBoundingClientRect();
          const delta = event.deltaMode === 1 ? event.deltaY * 16 : event.deltaMode === 2 ? event.deltaY * 400 : event.deltaY;
          session.wheelScreen(event.clientX - rect.left, event.clientY - rect.top, 0, delta, true);
          session.renderFrame();
          const cameraJson = session.cameraJson?.();
          if (cameraJson) dispatch(nodeGraphActions.viewport, nodeGraphViewportActionArgs(cameraJson));
          paintOverlays();
        }}
      />
      <CanvasPickMenu request={pickInteraction.pickMenu} hoveredKey={pickInteraction.menuHoveredKey} onHoverKey={pickInteraction.onMenuHoverKey} onPick={pickInteraction.onMenuPick} onDismiss={pickInteraction.dismissPickMenu} />
      <SpotlightOverlay previewText={previewText} onCommit={() => dispatch(nodeGraphActions.spotlightCommit, {})} onDismiss={() => setPreviewText("")} />
      <ContextMenuController
        open={contextMenu != null}
        position={contextMenu ?? { x: 0, y: 0 }}
        items={contextMenuItems.map((item) => ({
          id: item.id,
          label: item.label,
          onSelect: () => dispatch(item.action, item.action === "openInstance" ? { ...item.args, instanceId: resolveFixtureWidgetInstanceId(scene.fixtureJson, contextMenu?.widgetId) } : item.args),
        }))}
        onOpenChange={(open) => {
          if (!open) setContextMenu(null);
        }}
      />
    </div>
  );
}
//#endregion FlowGraphCanvasHost
//#endregion 🔖flow-graph-canvas-host
//#endregion 🔖NodeGraphHost

//#region 🔖TextEditorHost
//#region Types
type GrammarToken = { readonly class: string; readonly start: number; readonly end: number };

type EditorDiagnostic = { readonly start: number; readonly end: number; readonly severity?: string; readonly message: string };

type FrameworkEditorSession = EditorWasmSession;

type SpanRange = { readonly start: number; readonly end: number };

type CompletionItem = { readonly label: string; readonly detail?: string; readonly insertText?: string };

type RenameInfo = { readonly name: string; readonly occurrences: readonly SpanRange[] };

type RenameDraft = { readonly occurrences: readonly SpanRange[]; readonly text: string };

type PickTarget = { readonly domain: string; readonly id: string; readonly generality?: number; readonly label: string };
//#endregion Types

const TOKEN_CLASS_COLORS: Record<string, string> = {
  keyword: "text-sky-400",
  string: "text-emerald-400",
  number: "text-amber-400",
  operator: "text-violet-400",
  ident: "text-foreground",
};

//#region HighlightedBuffer
function HighlightedBuffer({ buffer, tokens }: { readonly buffer: string; readonly tokens: readonly GrammarToken[] }) {
  if (tokens.length === 0) {
    return <span className="whitespace-pre-wrap font-mono text-xs text-foreground">{buffer}</span>;
  }
  const parts: ReactNode[] = [];
  let cursor = 0;
  for (const token of tokens) {
    if (token.start > cursor) parts.push(<span key={`plain-${cursor}`}>{buffer.slice(cursor, token.start)}</span>);
    const color = TOKEN_CLASS_COLORS[token.class] ?? "text-foreground";
    parts.push(
      <span key={`token-${token.start}-${token.end}`} className={`font-mono text-xs ${color}`}>
        {buffer.slice(token.start, token.end)}
      </span>,
    );
    cursor = Math.max(cursor, token.end);
  }
  if (cursor < buffer.length) parts.push(<span key={`tail-${cursor}`}>{buffer.slice(cursor)}</span>);
  return <div className="pointer-events-none absolute inset-0 overflow-hidden whitespace-pre-wrap p-3">{parts}</div>;
}
//#endregion HighlightedBuffer

//#region EditingHelpers
/** 🌐 Resolves a translation key outside of component render (e.g. `buildTextEditorContextMenuItems`), mirroring `shellLabel`/`interpLabel`. */
function hostLabel(key: string): string {
  return resolveTranslationLabel(uiI18n.t(key as never)) ?? key;
}

/** ✂️ Language-agnostic multi-span rename preview: replaces every span with `nextName`, remapping spans left-to-right. */
export function multiSpanReplace(text: string, occurrences: readonly SpanRange[], nextName: string): { readonly text: string; readonly occurrences: readonly SpanRange[] } {
  const sorted = [...occurrences].sort((a, b) => b.start - a.start);
  let out = text;
  const nextOccurrences: SpanRange[] = [];
  for (const occ of sorted) {
    out = `${out.slice(0, occ.start)}${nextName}${out.slice(occ.end)}`;
    nextOccurrences.unshift({ start: occ.start, end: occ.start + nextName.length });
  }
  return { text: out, occurrences: nextOccurrences };
}

export function lineRangeAt(text: string, offset: number): SpanRange {
  const start = text.lastIndexOf("\n", Math.max(0, offset - 1)) + 1;
  const nextNewline = text.indexOf("\n", offset);
  const end = nextNewline === -1 ? text.length : nextNewline;
  return { start, end };
}

function identifierPrefixStart(text: string, caret: number): number {
  let start = caret;
  while (start > 0 && /[A-Za-z0-9_]/.test(text[start - 1] ?? "")) start -= 1;
  return start;
}

function parseJsonOr<T>(json: string | undefined, fallback: T): T {
  if (!json) return fallback;
  try {
    return JSON.parse(json) as T;
  } catch {
    return fallback;
  }
}

/** 🧭 Builds the right-click menu rows for a text-editor surface, independent of the active language. */
export function buildTextEditorContextMenuItems(
  input: { readonly canSuggest: boolean; readonly hasSelection: boolean; readonly canRename: boolean; readonly pickTargets: readonly PickTarget[] },
  actions: {
    readonly suggest: () => void;
    readonly selectToken: () => void;
    readonly selectLine: () => void;
    readonly selectAll: () => void;
    readonly rename: () => void;
    readonly cut: () => void;
    readonly copy: () => void;
    readonly paste: () => void;
    readonly format: () => void;
    readonly lint: () => void;
    readonly pickTarget: (target: PickTarget) => void;
  },
): ContextMenuItem[] {
  const items: ContextMenuItem[] = [];
  if (input.canSuggest) {
    items.push({ id: "writer-suggest", label: hostLabel("ui.contextMenu.suggestCompletions"), icon: "sparkles", shortcut: "Alt+Right click", onSelect: actions.suggest });
    items.push({ id: "writer-suggest-sep", separator: true });
  }
  if (input.pickTargets.length > 1) {
    for (const target of input.pickTargets) {
      items.push({
        id: `writer-pick-${target.domain}-${target.id}`,
        label: `${hostLabel("ui.contextMenu.select")} ${target.domain === "token" ? target.label : target.domain}`,
        icon: target.domain === "token" ? "text-cursor" : "list-ordered",
        onSelect: () => actions.pickTarget(target),
      });
    }
    items.push({ id: "writer-pick-sep", separator: true });
  }
  items.push({ id: "writer-select-token", label: hostLabel("ui.contextMenu.selectToken"), icon: "text-cursor", onSelect: actions.selectToken });
  items.push({ id: "writer-select-line", label: hostLabel("ui.contextMenu.selectLine"), icon: "list-ordered", onSelect: actions.selectLine });
  items.push({ id: "writer-select-all", label: hostLabel("ui.contextMenu.selectAll"), icon: "maximize-2", shortcut: "⌘A", onSelect: actions.selectAll });
  if (input.canRename) {
    items.push({ id: "writer-rename", label: hostLabel("ui.contextMenu.rename"), icon: "edit-3", shortcut: "F2", onSelect: actions.rename });
  }
  items.push({ id: "writer-clip-sep", separator: true });
  items.push({ id: "writer-cut", label: hostLabel("ui.contextMenu.cut"), icon: "scissors", shortcut: "⌘X", disabled: !input.hasSelection, onSelect: actions.cut });
  items.push({ id: "writer-copy", label: hostLabel("ui.contextMenu.copy"), icon: "copy", shortcut: "⌘C", disabled: !input.hasSelection, onSelect: actions.copy });
  items.push({ id: "writer-paste", label: hostLabel("ui.contextMenu.paste"), icon: "clipboard", shortcut: "⌘V", onSelect: actions.paste });
  items.push({ id: "writer-format-sep", separator: true });
  items.push({ id: "writer-format", label: hostLabel("ui.contextMenu.formatDocument"), icon: "align-left", shortcut: "⇧⌘F", onSelect: actions.format });
  items.push({ id: "writer-lint", label: hostLabel("ui.contextMenu.lintDocument"), icon: "alert-circle", onSelect: actions.lint });
  return items;
}
//#endregion EditingHelpers

//#region WasmEditorSurface
function WasmEditorSurface({ scene, controllerId, surfaceId, onAction }: { readonly scene: TextEditorScene; readonly controllerId: string; readonly surfaceId: string; readonly onAction: (action: ActionDescriptor) => void }) {
  const sessionRef = useRef<FrameworkEditorSession | null>(null);
  const renameActiveRef = useRef(false);
  const lastHoverRangeRef = useRef<SpanRange | null>(null);
  const sceneJson = useMemo(() => JSON.stringify(scene), [scene]);

  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      onAction({ controllerId, action, args: { surfaceId, ...args } });
    },
    [controllerId, onAction, surfaceId],
  );

  const syncSession = useCallback(() => {
    if (renameActiveRef.current) return;
    sessionRef.current?.syncFromSceneJson(sceneJson);
    sessionRef.current?.renderFrame();
  }, [sceneJson]);

  const [sessionEpoch, setSessionEpoch] = useState(0);

  useEffect(() => {
    syncSession();
    // sessionEpoch: re-sync immediately after GraphWasmCanvas (re)attaches a session (e.g. the stub -> real wasm swap),
    // since the attach lifecycle is independent of scene changes and a ref update alone would not otherwise re-trigger this effect.
  }, [syncSession, sessionEpoch]);

  const emitSelection = useCallback(() => {
    const session = sessionRef.current;
    if (!session) return;
    dispatch(textEditorActions.select, { start: session.anchor(), end: session.caret() });
  }, [dispatch]);

  const [wasmSession, setWasmSession] = useState<FrameworkEditorSession | null>(null);
  const [renameDraft, setRenameDraft] = useState<RenameDraft | null>(null);
  const [renamePosition, setRenamePosition] = useState<{ readonly x: number; readonly y: number } | null>(null);
  const [completionsOpen, setCompletionsOpen] = useState(false);
  const [completionIndex, setCompletionIndex] = useState(0);
  const [contextMenu, setContextMenu] = useState<{ readonly position: { readonly x: number; readonly y: number }; readonly items: ContextMenuItem[] } | null>(null);

  const completions = useMemo(() => parseJsonOr<readonly CompletionItem[]>(scene.completionsJson, []), [scene.completionsJson]);
  const renameInfo = useMemo(() => parseJsonOr<RenameInfo | null>(scene.renameJson, null), [scene.renameJson]);
  const newlineGates = useMemo(() => (scene.newlineGatesJson ? new Set(parseJsonOr<readonly number[]>(scene.newlineGatesJson, [])) : null), [scene.newlineGatesJson]);

  useEffect(() => {
    if (completions.length === 0 && completionsOpen) setCompletionsOpen(false);
    if (completionIndex >= completions.length) setCompletionIndex(0);
  }, [completions, completionsOpen, completionIndex]);

  useEffect(() => {
    let cancelled = false;
    void createEditorSession().then((session) => {
      if (!cancelled) setWasmSession(session);
    });
    return () => {
      cancelled = true;
    };
  }, []);

  const sessionFactory = useCallback(() => {
    if (wasmSession) return wasmSession;
    return {
      attachCanvas: async () => undefined,
      setSize: () => {},
      renderFrame: () => {},
      syncFromSceneJson: () => {},
      setText: () => {},
      text: () => scene.buffer,
      caret: () => scene.buffer.length,
      anchor: () => 0,
      pointerDownScreen: () => {},
      pointerMoveScreen: () => {},
      pointerUpScreen: () => {},
      wheelScrollScreen: () => {},
      insertText: () => {},
      backspace: () => {},
      deleteForward: () => {},
      selectAll: () => {},
      replaceSelection: () => {},
      selectionText: () => "",
      hoverTokenRangeJson: () => "null",
      setHoverRange: () => {},
      cameraJson: () => "{}",
      setCanvasThemeJson: () => {},
      moveLeft: () => {},
      moveRight: () => {},
      moveUp: () => {},
      moveDown: () => {},
      moveLineStart: () => {},
      moveLineEnd: () => {},
      tabInsertText: () => "  ",
      setSelectionRange: () => {},
      selectSpanAt: () => {},
      selectSpanAtScreen: () => {},
      pickTargetsAtScreenJson: () => "[]",
      caretWorldJson: () => "null",
      worldToScreenJson: () => "null",
      setSelectionOccurrencesJson: () => {},
      setExtraCaretsJson: () => {},
      setCaretVisible: () => {},
    } satisfies FrameworkEditorSession;
    // Deliberately omits scene.buffer: GraphWasmCanvas re-attaches the GPU canvas whenever sessionFactory's
    // identity changes, so this must stay stable across content edits — only the wasmSession load transition matters.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [wasmSession]);

  const caretScreenPosition = useCallback((session: FrameworkEditorSession): { readonly x: number; readonly y: number } | null => {
    try {
      const world = JSON.parse(session.caretWorldJson()) as { readonly x?: number; readonly y?: number } | null;
      if (world?.x == null || world?.y == null) return null;
      const screen = JSON.parse(session.worldToScreenJson(world.x, world.y)) as { readonly x?: number; readonly y?: number } | null;
      if (screen?.x == null || screen?.y == null) return null;
      return { x: screen.x, y: screen.y };
    } catch {
      return null;
    }
  }, []);

  const openCompletions = useCallback(() => {
    if (completions.length === 0) return;
    setCompletionsOpen(true);
    setCompletionIndex(0);
  }, [completions.length]);

  const applyCompletion = useCallback(
    (item: CompletionItem) => {
      const session = sessionRef.current;
      if (!session) return;
      const text = session.text();
      const caret = session.caret();
      const prefixStart = identifierPrefixStart(text, caret);
      session.setSelectionRange(prefixStart, caret);
      session.replaceSelection(item.insertText ?? item.label);
      dispatch(textEditorActions.edit, { text: session.text() });
      session.renderFrame();
      emitSelection();
      setCompletionsOpen(false);
    },
    [dispatch, emitSelection],
  );

  const startRename = useCallback(() => {
    const session = sessionRef.current;
    if (!session || !renameInfo) return;
    renameActiveRef.current = true;
    setRenameDraft({ occurrences: renameInfo.occurrences, text: renameInfo.name });
    setRenamePosition(caretScreenPosition(session));
  }, [renameInfo, caretScreenPosition]);

  const updateRenamePreview = useCallback(
    (nextText: string) => {
      const session = sessionRef.current;
      if (!session || !renameDraft) return;
      const preview = multiSpanReplace(scene.buffer, renameDraft.occurrences, nextText);
      session.setText(preview.text);
      session.setSelectionOccurrencesJson(JSON.stringify(preview.occurrences));
      session.setExtraCaretsJson(JSON.stringify(preview.occurrences.map((occ) => occ.start)));
      session.renderFrame();
      setRenameDraft({ ...renameDraft, text: nextText });
    },
    [renameDraft, scene.buffer],
  );

  const commitRename = useCallback(() => {
    if (!renameDraft) return;
    dispatch(textEditorActions.commitRename, { occurrences: renameDraft.occurrences, text: renameDraft.text });
    renameActiveRef.current = false;
    setRenameDraft(null);
    setRenamePosition(null);
  }, [dispatch, renameDraft]);

  const cancelRename = useCallback(() => {
    const session = sessionRef.current;
    if (session) {
      session.setText(scene.buffer);
      session.renderFrame();
    }
    renameActiveRef.current = false;
    setRenameDraft(null);
    setRenamePosition(null);
  }, [scene.buffer]);

  const dismissContextMenu = useCallback(() => setContextMenu(null), []);

  // Stable identity: GraphWasmCanvas re-attaches the GPU canvas whenever this prop's identity changes,
  // so it must not close over anything that changes per scene update (see sessionEpoch above for re-sync).
  const onSessionReady = useCallback((session: GraphWasmSession) => {
    sessionRef.current = session as FrameworkEditorSession;
    setSessionEpoch((epoch) => epoch + 1);
  }, []);

  return (
    <div className="relative min-h-0 flex-1">
      <GraphWasmCanvas className="absolute inset-0" sessionFactory={sessionFactory} onSessionReady={onSessionReady} enablePointer={false} />
      <div
        className="absolute inset-0"
        onPointerDown={(event) => {
          const session = sessionRef.current;
          if (!session) return;
          const rect = event.currentTarget.getBoundingClientRect();
          const sx = event.clientX - rect.left;
          const sy = event.clientY - rect.top;
          if (event.detail >= 2) {
            session.selectSpanAtScreen(sx, sy);
            session.renderFrame();
            emitSelection();
            return;
          }
          session.pointerDownScreen(sx, sy, event.button);
          session.renderFrame();
          emitSelection();
        }}
        onPointerMove={(event) => {
          const session = sessionRef.current;
          if (!session) return;
          const rect = event.currentTarget.getBoundingClientRect();
          session.pointerMoveScreen(event.clientX - rect.left, event.clientY - rect.top, event.buttons);
          try {
            const hover = JSON.parse(session.hoverTokenRangeJson()) as SpanRange | null;
            const changed = (hover?.start ?? null) !== (lastHoverRangeRef.current?.start ?? null) || (hover?.end ?? null) !== (lastHoverRangeRef.current?.end ?? null);
            if (changed) {
              lastHoverRangeRef.current = hover;
              if (hover) {
                session.setHoverRange(hover.start, hover.end);
                dispatch(textEditorActions.hover, { start: hover.start, end: hover.end });
              }
            }
          } catch {
            /* hover range unavailable */
          }
          session.renderFrame();
        }}
        onPointerUp={(event) => {
          const session = sessionRef.current;
          if (!session) return;
          const rect = event.currentTarget.getBoundingClientRect();
          session.pointerUpScreen(event.clientX - rect.left, event.clientY - rect.top, event.buttons);
          session.renderFrame();
          emitSelection();
        }}
        onWheel={(event) => {
          const session = sessionRef.current;
          if (!session) return;
          event.preventDefault();
          session.wheelScrollScreen(event.deltaY);
          session.renderFrame();
          dispatch("setCamera", { camera: JSON.parse(session.cameraJson()) });
        }}
        onContextMenu={(event) => {
          event.preventDefault();
          const session = sessionRef.current;
          if (!session) return;
          dismissContextMenu();
          const rect = event.currentTarget.getBoundingClientRect();
          const sx = event.clientX - rect.left;
          const sy = event.clientY - rect.top;
          session.pointerDownScreen(sx, sy, 0);
          session.pointerUpScreen(sx, sy, 0);
          session.renderFrame();
          emitSelection();
          if (event.altKey && completions.length > 0) {
            openCompletions();
            return;
          }
          const pickTargets = parseJsonOr<readonly PickTarget[]>(session.pickTargetsAtScreenJson(sx, sy), []);
          const hasSelection = session.anchor() !== session.caret();
          const items = buildTextEditorContextMenuItems(
            { canSuggest: completions.length > 0, hasSelection, canRename: renameInfo != null, pickTargets },
            {
              suggest: openCompletions,
              selectToken: () => {
                session.selectSpanAt(session.caret());
                session.renderFrame();
                emitSelection();
              },
              selectLine: () => {
                const range = lineRangeAt(scene.buffer, session.caret());
                session.setSelectionRange(range.start, range.end);
                session.renderFrame();
                emitSelection();
              },
              selectAll: () => {
                session.selectAll();
                session.renderFrame();
                emitSelection();
              },
              rename: startRename,
              cut: () => {
                void navigator.clipboard.writeText(session.selectionText());
                session.replaceSelection("");
                dispatch(textEditorActions.edit, { text: session.text() });
                session.renderFrame();
                emitSelection();
              },
              copy: () => {
                void navigator.clipboard.writeText(session.selectionText());
              },
              paste: () => {
                void navigator.clipboard.readText().then((text) => {
                  session.replaceSelection(text);
                  dispatch(textEditorActions.edit, { text: session.text() });
                  session.renderFrame();
                  emitSelection();
                });
              },
              format: () => dispatch(textEditorActions.formatDocument, {}),
              lint: () => dispatch("lintDocument", {}),
              pickTarget: (target) => {
                if (target.domain === "token") {
                  const [start, end] = target.id.split(":").map(Number);
                  if (start != null && end != null) {
                    session.setSelectionRange(start, end);
                    session.renderFrame();
                    emitSelection();
                  }
                } else if (target.domain === "line") {
                  const lines = scene.buffer.split("\n");
                  const lineIndex = Number(target.id);
                  let offset = 0;
                  for (let i = 0; i < lineIndex; i++) offset += (lines[i]?.length ?? 0) + 1;
                  const lineLength = lines[lineIndex]?.length ?? 0;
                  session.setSelectionRange(offset, offset + lineLength);
                  session.renderFrame();
                  emitSelection();
                }
              },
            },
          );
          setContextMenu({ position: { x: event.clientX, y: event.clientY }, items });
        }}
      >
        {renameDraft ? (
          <input
            className="pointer-events-auto absolute z-50 min-w-[12rem] rounded border border-border bg-panel px-2 py-1 font-mono text-xs text-foreground shadow-md"
            style={renamePosition ? { left: renamePosition.x, top: renamePosition.y - 4 } : { left: 12, top: 12 }}
            value={renameDraft.text}
            autoFocus
            onChange={(event) => updateRenamePreview(event.target.value)}
            onKeyDown={(event) => {
              event.stopPropagation();
              if (event.key === "Escape") {
                event.preventDefault();
                cancelRename();
                return;
              }
              if (event.key === "Enter") {
                event.preventDefault();
                commitRename();
              }
            }}
            onBlur={commitRename}
          />
        ) : null}
        {completionsOpen && completions.length > 0
          ? (() => {
              const session = sessionRef.current;
              const position = session ? caretScreenPosition(session) : null;
              return (
                <div className="pointer-events-auto absolute z-50 max-h-48 min-w-40 overflow-auto rounded border border-border bg-popover p-1 shadow-md" style={position ? { left: position.x, top: position.y + 18 } : { left: 12, top: 12 }}>
                  {completions.map((item, index) => (
                    <button
                      key={`${item.label}-${index}`}
                      type="button"
                      className={`block w-full rounded px-2 py-1 text-left font-mono text-[11px] ${index === completionIndex ? "bg-accent text-accent-foreground" : "hover:bg-active-base"}`}
                      onPointerDown={(event) => event.stopPropagation()}
                      onClick={() => applyCompletion(item)}
                    >
                      <span>{item.label}</span>
                      {item.detail ? <span className="ml-2 text-muted-foreground">{item.detail}</span> : null}
                    </button>
                  ))}
                </div>
              );
            })()
          : null}
      </div>
      <ContextMenuController open={contextMenu != null} position={contextMenu?.position ?? null} items={contextMenu?.items ?? []} onOpenChange={(open) => !open && dismissContextMenu()} />
      <textarea
        className="absolute inset-0 resize-none bg-transparent font-mono text-xs text-transparent caret-foreground opacity-0"
        value={scene.buffer}
        onChange={(event) => dispatch(textEditorActions.edit, { text: event.target.value })}
        onKeyDown={(event) => {
          const session = sessionRef.current;
          if (event.key === "Enter" && (event.metaKey || event.ctrlKey)) {
            event.preventDefault();
            dispatch("submit", {});
            return;
          }
          if ((event.key === " " || event.code === "Space") && (event.metaKey || event.ctrlKey)) {
            event.preventDefault();
            openCompletions();
            return;
          }
          if (event.key === "F2" && renameInfo) {
            event.preventDefault();
            startRename();
            return;
          }
          if (event.key === "a" && (event.metaKey || event.ctrlKey)) {
            event.preventDefault();
            session?.selectAll();
            session?.renderFrame();
            emitSelection();
            return;
          }
          if (event.key.toLowerCase() === "f" && event.shiftKey && (event.metaKey || event.ctrlKey)) {
            event.preventDefault();
            dispatch(textEditorActions.formatDocument, {});
            return;
          }
          if (!session) return;

          if (completionsOpen && completions.length > 0) {
            if (event.key === "ArrowDown") {
              event.preventDefault();
              setCompletionIndex((index) => (index + 1) % completions.length);
              return;
            }
            if (event.key === "ArrowUp") {
              event.preventDefault();
              setCompletionIndex((index) => (index - 1 + completions.length) % completions.length);
              return;
            }
            if (event.key === "Tab" || event.key === "Enter") {
              event.preventDefault();
              applyCompletion(completions[completionIndex] ?? completions[0]!);
              return;
            }
            if (event.key === "Escape") {
              event.preventDefault();
              setCompletionsOpen(false);
              return;
            }
          }

          const extend = event.shiftKey;
          if (event.key === "ArrowLeft") {
            event.preventDefault();
            session.moveLeft(extend);
            session.renderFrame();
            emitSelection();
            return;
          }
          if (event.key === "ArrowRight") {
            event.preventDefault();
            session.moveRight(extend);
            session.renderFrame();
            emitSelection();
            return;
          }
          if (event.key === "ArrowUp") {
            event.preventDefault();
            session.moveUp(extend);
            session.renderFrame();
            emitSelection();
            return;
          }
          if (event.key === "ArrowDown") {
            event.preventDefault();
            session.moveDown(extend);
            session.renderFrame();
            emitSelection();
            return;
          }
          if (event.key === "Home") {
            event.preventDefault();
            session.moveLineStart(extend);
            session.renderFrame();
            emitSelection();
            return;
          }
          if (event.key === "End") {
            event.preventDefault();
            session.moveLineEnd(extend);
            session.renderFrame();
            emitSelection();
            return;
          }
          if (event.key === "Tab") {
            event.preventDefault();
            session.insertText(session.tabInsertText());
            dispatch(textEditorActions.edit, { text: session.text() });
            session.renderFrame();
            emitSelection();
            return;
          }
          if (event.key === "Enter") {
            event.preventDefault();
            const allowed = newlineGates == null || newlineGates.has(session.caret());
            if (allowed) {
              session.insertText("\n");
              dispatch(textEditorActions.edit, { text: session.text() });
              session.renderFrame();
              emitSelection();
            }
            return;
          }
          if ((event.target as HTMLElement).tagName === "TEXTAREA" && event.key.length !== 1) return;
          if (event.key.length === 1 && !event.metaKey && !event.ctrlKey && !event.altKey) {
            event.preventDefault();
            session.insertText(event.key);
            dispatch(textEditorActions.edit, { text: session.text() });
            session.renderFrame();
            emitSelection();
            return;
          }
          if (event.key === "Backspace") {
            event.preventDefault();
            session.backspace();
            dispatch(textEditorActions.edit, { text: session.text() });
            session.renderFrame();
            emitSelection();
            return;
          }
          if (event.key === "Delete") {
            event.preventDefault();
            session.deleteForward();
            dispatch(textEditorActions.edit, { text: session.text() });
            session.renderFrame();
            emitSelection();
          }
        }}
        spellCheck={false}
        aria-label={scene.language ? `${scene.language} editor` : "Editor"}
      />
    </div>
  );
}
//#endregion WasmEditorSurface

//#region TextEditorHost
export function TextEditorHost({ node, onAction }: ComponentSceneHostProps) {
  const scene = node.textEditor;
  const isClient = useClient();
  const tokens = useMemo((): readonly GrammarToken[] => {
    if (!scene?.tokensJson) return [];
    try {
      return JSON.parse(scene.tokensJson) as GrammarToken[];
    } catch {
      return [];
    }
  }, [scene?.tokensJson]);
  const diagnostics = useMemo((): readonly EditorDiagnostic[] => {
    if (!scene?.diagnosticsJson) return [];
    try {
      return JSON.parse(scene.diagnosticsJson) as EditorDiagnostic[];
    } catch {
      return [];
    }
  }, [scene?.diagnosticsJson]);
  const emptySceneLabel = useLabel("ui.host.emptyScene");

  if (!scene) return <div className="semio-text-editor-empty">{emptySceneLabel}</div>;

  return (
    <div className="semio-text-editor-host flex h-full min-h-[16rem] w-full flex-col bg-canvas" data-surface-id={node.surfaceId}>
      {isClient ? (
        <WasmEditorSurface scene={scene} controllerId={node.controllerId} surfaceId={node.surfaceId} onAction={onAction} />
      ) : (
        <div className="relative min-h-0 flex-1">
          <HighlightedBuffer buffer={scene.buffer} tokens={tokens} />
          <Textarea
            className="relative min-h-0 flex-1 resize-none bg-transparent font-mono text-xs text-transparent caret-foreground"
            id={`${node.surfaceId}.editor`}
            lazy
            rows={24}
            value={scene.buffer}
            placeholder={scene.language ? `${scene.language} document` : "Document"}
            onLazyChange={(value) =>
              onAction({
                controllerId: node.controllerId,
                action: textEditorActions.edit,
                args: { surfaceId: node.surfaceId, text: value },
              })
            }
          />
        </div>
      )}
      {diagnostics.length > 0 ? (
        <div className="border-t border-border px-3 py-2 text-[11px] text-muted-foreground">
          {diagnostics.slice(0, 4).map((diag, index) => (
            <div key={`${diag.start}-${diag.end}-${index}`} className="truncate">
              {diag.severity ? `[${diag.severity}] ` : ""}
              {diag.message}
            </div>
          ))}
        </div>
      ) : null}
    </div>
  );
}
//#endregion TextEditorHost
//#endregion 🔖TextEditorHost

//#region 🔖TableHost
//#region TableHost
//#region Types
type TableColumnRecord = { readonly id: string; readonly label: string; readonly sortable?: boolean };
type TableCellButton = { readonly iconId: string; readonly label?: string; readonly action: ActionDescriptor; readonly revealOnHover?: boolean };
type TableCellRecord =
  | { readonly kind: "text"; readonly value: string }
  | { readonly kind: "number"; readonly value: number }
  | { readonly kind: "stepper"; readonly value: number; readonly min: number; readonly max: number; readonly step: number; readonly action: ActionDescriptor }
  | { readonly kind: "buttons"; readonly buttons: readonly TableCellButton[] };
type TableRowRecord = Record<string, unknown> & { readonly id?: string; readonly _drag?: Record<string, unknown> };
//#endregion Types

//#region Helpers
function isTableCellRecord(value: unknown): value is TableCellRecord {
  return typeof value === "object" && value !== null && "kind" in value;
}

function dispatchCellAction(onAction: (action: ActionDescriptor) => void, descriptor: ActionDescriptor, patch: Record<string, unknown>): void {
  onAction({
    ...descriptor,
    args: { ...(typeof descriptor.args === "object" && descriptor.args != null ? descriptor.args : {}), ...patch },
  });
}

function resolveTableCellIcon(iconId: string): IconName {
  return iconId in ICONS ? (iconId as IconName) : "circle-dot";
}

function renderTableCell(cell: TableCellRecord, onAction: (action: ActionDescriptor) => void): React.ReactNode {
  switch (cell.kind) {
    case "text":
      return cell.value;
    case "number":
      return String(cell.value);
    case "stepper":
      return (
        <div className="flex min-w-0 items-center gap-1" onClick={(event) => event.stopPropagation()}>
          <Button className="h-medium shrink-0 px-2" onClick={() => dispatchCellAction(onAction, cell.action, { delta: -cell.step })} disabled={cell.value <= cell.min} type="button" variant="outline">
            −
          </Button>
          <Input className="h-medium w-14 min-w-0 text-center font-mono text-xs" readOnly value={String(cell.value)} />
          <Button className="h-medium shrink-0 px-2" onClick={() => dispatchCellAction(onAction, cell.action, { delta: cell.step })} disabled={cell.value >= cell.max} type="button" variant="outline">
            +
          </Button>
        </div>
      );
    case "buttons":
      return (
        <div className="flex min-w-0 items-center gap-1" onClick={(event) => event.stopPropagation()}>
          {cell.buttons.map((button, index) => (
            <Button key={index} className="h-medium shrink-0 px-2" onClick={() => dispatchCellAction(onAction, button.action, {})} title={button.label} type="button" variant="outline">
              <Icon icon={resolveTableCellIcon(button.iconId)} size="small" />
            </Button>
          ))}
        </div>
      );
  }
}
//#endregion Helpers

//#region Component
export function TableHost({ node, onAction }: ComponentSceneHostProps) {
  const scene = node.table;
  const emptySceneLabel = useLabel("ui.host.emptyScene");
  const columns = useMemo(() => {
    if (!scene) return [] as TableColumnRecord[];
    try {
      return JSON.parse(scene.columnsJson) as TableColumnRecord[];
    } catch {
      return [];
    }
  }, [scene]);
  const rows = useMemo(() => {
    if (!scene) return [] as TableRowRecord[];
    try {
      return JSON.parse(scene.rowsJson) as TableRowRecord[];
    } catch {
      return [];
    }
  }, [scene]);
  const selectedRows = useMemo(() => {
    if (!scene?.selectionJson) return undefined;
    try {
      const parsed = JSON.parse(scene.selectionJson) as { readonly selectedIds?: readonly string[] };
      return new Set(parsed.selectedIds ?? []);
    } catch {
      return undefined;
    }
  }, [scene]);
  const sort = useMemo(() => {
    if (!scene?.sortJson) return undefined;
    try {
      return JSON.parse(scene.sortJson) as { readonly columnId?: string; readonly direction?: "asc" | "desc" };
    } catch {
      return undefined;
    }
  }, [scene]);
  const tableColumns = useMemo<TableColumn<TableRowRecord>[]>(
    () =>
      columns.map((column) => ({
        id: column.id,
        header: column.label,
        sortable: column.sortable,
        accessor: (row) => {
          const value = row[column.id];
          if (isTableCellRecord(value)) return renderTableCell(value, onAction);
          return String(value ?? "");
        },
      })),
    [columns, onAction],
  );

  if (!scene) return <div className="semio-table-empty">{emptySceneLabel}</div>;

  const rowDragMime = scene.rowDragMime;
  const dropAction = scene.dropAction;

  return (
    <div
      className="semio-table-host h-full min-h-0 w-full"
      data-surface-id={node.surfaceId}
      onDragOver={
        dropAction
          ? (event) => {
              event.preventDefault();
              event.dataTransfer.dropEffect = "copy";
            }
          : undefined
      }
      onDrop={
        dropAction
          ? (event) => {
              event.preventDefault();
              const encoded = [...event.dataTransfer.types].filter((kind) => kind.startsWith("application/x-semio-")).map((kind) => event.dataTransfer.getData(kind))[0];
              if (!encoded?.trim()) return;
              try {
                dispatchCellAction(onAction, dropAction, JSON.parse(encoded) as Record<string, unknown>);
              } catch {
                return;
              }
            }
          : undefined
      }
    >
      <Table
        className="h-full w-full"
        columns={tableColumns}
        data={rows}
        emptyMessage="No rows"
        getRowId={(row, index) => String(row.id ?? row.programId ?? index)}
        selectedRows={selectedRows}
        sortColumn={sort?.columnId}
        sortDirection={sort?.direction}
        onSort={(columnId, direction) =>
          onAction({
            controllerId: node.controllerId,
            action: "sortTable",
            args: { surfaceId: node.surfaceId, columnId, direction },
          })
        }
        rowDragProps={
          rowDragMime
            ? (row) =>
                row._drag
                  ? {
                      draggable: true,
                      onDragStart: (event) => {
                        event.dataTransfer.setData(rowDragMime, JSON.stringify(row._drag));
                        event.dataTransfer.effectAllowed = "copy";
                      },
                    }
                  : {}
            : undefined
        }
        onRowClick={(row) =>
          onAction({
            controllerId: node.controllerId,
            action: "selectRow",
            args: { surfaceId: node.surfaceId, row },
          })
        }
      />
    </div>
  );
}
//#endregion Component
//#endregion TableHost
//#endregion 🔖TableHost

//#region 🔖Paint2dHost
//#region Paint2dParsing
const PAINT_2D_MARQUEE_THRESHOLD_PX = 4;

type Paint2dViewportSize = { readonly width: number; readonly height: number };
type Paint2dScreenRect = { readonly x: number; readonly y: number; readonly width: number; readonly height: number };
type Paint2dPickTarget = { readonly domain: string; readonly id: string; readonly generality: number };

function parsePaint2dCameraJson(json: string | undefined): CanvasCamera {
  try {
    const parsed = JSON.parse(json ?? "{}") as Partial<CanvasCamera>;
    return { x: Number(parsed.x ?? 0), y: Number(parsed.y ?? 0), zoom: Number(parsed.zoom ?? 1) };
  } catch {
    return { x: 0, y: 0, zoom: 1 };
  }
}

function paint2dCameraEqual(a: CanvasCamera, b: CanvasCamera): boolean {
  return a.x === b.x && a.y === b.y && a.zoom === b.zoom;
}

function parsePaint2dViewport(json: string | undefined): Paint2dViewportSize | null {
  if (!json) return null;
  try {
    const parsed = JSON.parse(json) as Partial<Paint2dViewportSize>;
    if (parsed.width == null || parsed.height == null) return null;
    return { width: Number(parsed.width), height: Number(parsed.height) };
  } catch {
    return null;
  }
}

function parsePaint2dSelection(json: string | undefined): string[] {
  try {
    const parsed = JSON.parse(json ?? "[]") as unknown;
    return Array.isArray(parsed) ? parsed.filter((value): value is string => typeof value === "string") : [];
  } catch {
    return [];
  }
}

type Paint2dAssetRecord = { readonly mime?: string; readonly data: string };

function parsePaint2dAssets(json: string | undefined): Record<string, Paint2dAssetRecord> {
  try {
    return JSON.parse(json ?? "{}") as Record<string, Paint2dAssetRecord>;
  } catch {
    return {};
  }
}

function base64ToBytes(base64: string): Uint8Array {
  const binary = atob(base64);
  const bytes = new Uint8Array(binary.length);
  for (let index = 0; index < binary.length; index += 1) bytes[index] = binary.charCodeAt(index);
  return bytes;
}

function paint2dSelectionMethod(activeUtility: string): SelectionMarqueeMethod | null {
  if (activeUtility === "selectMarquee") return "rectangle";
  if (activeUtility === "selectLasso") return "lasso";
  return null;
}

function isPaint2dSelectionUtility(activeUtility: string): boolean {
  return activeUtility === "selectMarquee" || activeUtility === "selectLasso" || activeUtility === "selectWand";
}
//#endregion Paint2dParsing

//#region Paint2dNoopSession
function noopPaint2dSession(): RasterWasmSession {
  return {
    gpuReady: () => false,
    attachCanvas: async () => undefined,
    setSize: () => {},
    renderFrame: () => {},
    setCamera: () => {},
    wheelScreen: () => {},
    pointerDownScreen: () => {},
    pointerMoveScreen: () => {},
    pointerUpScreen: () => {},
    syncDocumentJson: () => {},
    uploadLayerImage: () => {},
    uploadRasterImageKey: () => {},
    setActiveUtility: () => {},
    setBrushSize: () => {},
    setBrushOpacity: () => {},
    setHoveredIdSilent: () => {},
    setSelectionIdsJson: () => {},
    setCanvasThemeJson: () => {},
    cameraJson: () => '{"x":0,"y":0,"zoom":1}',
    setViewMode: () => {},
    pickTargetsAtScreenJson: () => "[]",
    marqueeHitsJson: () => "[]",
    navigatorFitCameraJson: () => '{"x":0,"y":0,"zoom":1}',
    navigatorViewportOverlayJson: () => '{"x":0,"y":0,"width":0,"height":0}',
    free: () => {},
  };
}
//#endregion Paint2dNoopSession

//#region Paint2dMarqueeOverlay
type Paint2dMarqueeOverlay =
  | { readonly coverage: SelectionMarqueeCoverage; readonly shape: "rect"; readonly rect: Paint2dScreenRect }
  | { readonly coverage: SelectionMarqueeCoverage; readonly shape: "polygon"; readonly points: readonly { readonly x: number; readonly y: number }[] };
//#endregion Paint2dMarqueeOverlay

//#region Paint2dCanvasSurface
function Paint2dCanvasSurface({ node, scene, onAction }: { readonly node: UiComponentSceneNode; readonly scene: Paint2dScene; readonly onAction: (action: ActionDescriptor) => void }) {
  const isNavigator = scene.viewMode === "navigator";
  const containerRef = useRef<HTMLDivElement>(null);
  const sessionRef = useRef<RasterWasmSession | null>(null);
  const cameraRef = useRef<CanvasCamera>({ x: 0, y: 0, zoom: 1 });
  const documentSyncRef = useRef<string | null>(null);
  const assetsRef = useRef<string | null>(null);
  const marqueeRef = useRef<{ tracking: boolean; active: boolean; start: { x: number; y: number }; points: { x: number; y: number }[] }>({
    tracking: false,
    active: false,
    start: { x: 0, y: 0 },
    points: [],
  });
  const panRef = useRef<{ last: { x: number; y: number } } | null>(null);

  const [wasmSession, setWasmSession] = useState<RasterWasmSession | null>(null);
  const [attachError, setAttachError] = useState<string | null>(null);
  const [marqueeOverlay, setMarqueeOverlay] = useState<Paint2dMarqueeOverlay | null>(null);
  const [overlayRect, setOverlayRect] = useState<Paint2dScreenRect | null>(null);
  const canvasUnavailableLabel = useLabel("ui.host.canvasUnavailable");

  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      onAction({ controllerId: node.controllerId, action, args: { surfaceId: node.surfaceId, ...args } });
    },
    [node.controllerId, node.surfaceId, onAction],
  );

  //#region Session lifecycle
  useEffect(() => {
    let cancelled = false;
    void createRasterSession().then((session) => {
      if (!cancelled) setWasmSession(session);
    });
    return () => {
      cancelled = true;
    };
  }, []);

  useEffect(() => {
    if (!wasmSession) return;
    let cancelled = false;
    const timer = setTimeout(() => {
      if (!cancelled && !wasmSession.gpuReady()) setAttachError("WebGPU did not initialize");
    }, 4000);
    return () => {
      cancelled = true;
      clearTimeout(timer);
    };
  }, [wasmSession]);

  const syncAll = useCallback(() => {
    const session = sessionRef.current;
    if (!session) return;
    if (documentSyncRef.current !== scene.documentSyncJson) {
      session.syncDocumentJson(scene.documentSyncJson);
      documentSyncRef.current = scene.documentSyncJson;
    }
    if (assetsRef.current !== scene.assetsJson) {
      const assets = parsePaint2dAssets(scene.assetsJson);
      for (const [key, asset] of Object.entries(assets)) {
        try {
          session.uploadRasterImageKey(key, base64ToBytes(asset.data));
        } catch {
          /* asset decode failed */
        }
      }
      assetsRef.current = scene.assetsJson;
    }
    session.setActiveUtility(scene.activeUtility);
    session.setBrushSize(scene.brushSize);
    session.setBrushOpacity(scene.brushOpacity);
    session.setSelectionIdsJson(scene.selectionJson);
    session.setHoveredIdSilent(scene.hoveredId ?? null);
    session.setViewMode(scene.viewMode);
    if (isNavigator) {
      const rect = containerRef.current?.getBoundingClientRect();
      const width = rect?.width || 1;
      const height = rect?.height || 1;
      const fit = parsePaint2dCameraJson(session.navigatorFitCameraJson(width, height));
      session.setCamera(fit.x, fit.y, fit.zoom);
      cameraRef.current = fit;
      if (scene.compositeViewportJson) {
        try {
          setOverlayRect(JSON.parse(session.navigatorViewportOverlayJson(scene.cameraJson, scene.compositeViewportJson)) as Paint2dScreenRect);
        } catch {
          setOverlayRect(null);
        }
      } else {
        setOverlayRect(null);
      }
    } else {
      const sceneCamera = parsePaint2dCameraJson(scene.cameraJson);
      if (!paint2dCameraEqual(sceneCamera, cameraRef.current)) {
        session.setCamera(sceneCamera.x, sceneCamera.y, sceneCamera.zoom);
        cameraRef.current = sceneCamera;
      }
    }
    session.renderFrame();
  }, [isNavigator, scene.documentSyncJson, scene.assetsJson, scene.cameraJson, scene.selectionJson, scene.hoveredId, scene.activeUtility, scene.brushSize, scene.brushOpacity, scene.viewMode, scene.compositeViewportJson]);

  useEffect(() => {
    syncAll();
  }, [syncAll]);

  useCanvasAppearanceSync(() => {
    if (!sessionRef.current) return;
    syncSessionCanvasTheme(sessionRef.current);
    sessionRef.current.renderFrame();
  });

  const onSessionReady = useCallback(
    (session: RasterWasmSession) => {
      sessionRef.current = session;
      syncAll();
    },
    [syncAll],
  );

  const sessionFactory = useCallback((): RasterWasmSession => wasmSession ?? noopPaint2dSession(), [wasmSession]);
  //#endregion Session lifecycle

  //#region CompositeViewportReporting
  useEffect(() => {
    if (isNavigator) return;
    const container = containerRef.current;
    if (!container) return;
    let last: Paint2dViewportSize = { width: 0, height: 0 };
    const report = () => {
      const rect = container.getBoundingClientRect();
      const width = Math.round(rect.width);
      const height = Math.round(rect.height);
      if (width === last.width && height === last.height) return;
      last = { width, height };
      dispatch("setCompositeViewport", { width, height });
    };
    report();
    const observer = new ResizeObserver(report);
    observer.observe(container);
    return () => observer.disconnect();
  }, [dispatch, isNavigator]);
  //#endregion CompositeViewportReporting

  //#region PickInteraction
  const clientPoint = useCallback((event: { readonly clientX: number; readonly clientY: number }): { readonly x: number; readonly y: number } => {
    const rect = containerRef.current?.getBoundingClientRect();
    return { x: event.clientX - (rect?.left ?? 0), y: event.clientY - (rect?.top ?? 0) };
  }, []);

  const pickInteraction = useCanvasPickInteraction({
    resolveTargetsAtClient: (client) => {
      const session = sessionRef.current;
      const container = containerRef.current;
      if (!session || !container) return [];
      const rect = container.getBoundingClientRect();
      const point = { x: client.x - rect.left, y: client.y - rect.top };
      try {
        const targets = JSON.parse(session.pickTargetsAtScreenJson(point.x, point.y)) as Paint2dPickTarget[];
        return targets.map((target) => ({ ...target, label: target.id }));
      } catch {
        return [];
      }
    },
    onHoverFocus: (focus) => {
      const session = sessionRef.current;
      if (!session) return;
      const id = focus.target?.id ?? null;
      session.setHoveredIdSilent(id);
      session.renderFrame();
      dispatch("setHover", { id });
    },
    onSelectTarget: (target, request) => {
      const mergeMode = marqueeModeFromModifiers({
        shiftKey: request.modifiers?.shift === true,
        ctrlKey: request.modifiers?.ctrl === true,
        metaKey: request.modifiers?.meta === true,
      });
      dispatch("setSelection", { ids: selectionMergeIds(mergeMode, parsePaint2dSelection(scene.selectionJson), [target.id]) });
    },
  });
  //#endregion PickInteraction

  //#region Marquee
  const selectionMethod = paint2dSelectionMethod(scene.activeUtility);

  const updateMarqueeOverlay = useCallback(
    (point: { readonly x: number; readonly y: number }) => {
      if (!selectionMethod) return;
      const marquee = marqueeRef.current;
      const points = selectionMethod === "lasso" ? marquee.points : [marquee.start, point];
      const coverage = marqueeCoverageFromGesture({ method: selectionMethod, startX: marquee.start.x, endX: point.x, path: points });
      if (selectionMethod === "lasso") {
        setMarqueeOverlay({ coverage, shape: "polygon", points });
        return;
      }
      const rect = screenRectFromPoints(points);
      if (!rect) return;
      setMarqueeOverlay({ coverage, shape: "rect", rect });
    },
    [selectionMethod],
  );

  const commitMarqueeSelection = useCallback(
    (point: { readonly x: number; readonly y: number }, mergeMode: SelectionMergeMode) => {
      const session = sessionRef.current;
      if (!session) return;
      const marquee = marqueeRef.current;
      const points = selectionMethod === "lasso" ? [...marquee.points, point] : [marquee.start, point];
      const coverage = marqueeCoverageFromGesture({ method: selectionMethod ?? "rectangle", startX: marquee.start.x, endX: point.x, path: points });
      try {
        const hits = JSON.parse(session.marqueeHitsJson(JSON.stringify({ points, crossing: coverage === "partial" }))) as string[];
        dispatch("setSelection", { ids: selectionMergeIds(mergeMode, parsePaint2dSelection(scene.selectionJson), hits) });
      } catch {
        /* marquee hit test failed */
      }
    },
    [dispatch, scene.selectionJson, selectionMethod],
  );
  //#endregion Marquee

  //#region Pointer
  const onPointerDown = useCallback(
    (event: ReactPointerEvent<HTMLDivElement>) => {
      const point = clientPoint(event);
      const session = sessionRef.current;
      if (event.button === 1) {
        if (isNavigator) panRef.current = { last: point };
        else session?.pointerDownScreen(point.x, point.y, event.button);
        event.currentTarget.setPointerCapture(event.pointerId);
        return;
      }
      if (isNavigator || !session) return;
      if (isPaint2dSelectionUtility(scene.activeUtility)) {
        pickInteraction.onCanvasPointerDown({ x: event.clientX, y: event.clientY });
        if (selectionMethod) marqueeRef.current = { tracking: true, active: false, start: point, points: [point] };
        return;
      }
      session.pointerDownScreen(point.x, point.y, event.button);
      session.renderFrame();
    },
    [clientPoint, isNavigator, pickInteraction, scene.activeUtility, selectionMethod],
  );

  const onPointerMove = useCallback(
    (event: ReactPointerEvent<HTMLDivElement>) => {
      const point = clientPoint(event);
      const session = sessionRef.current;
      const pan = panRef.current;
      if (pan) {
        if (isNavigator) {
          const contentCamera = parsePaint2dCameraJson(scene.cameraJson);
          const next = {
            x: contentCamera.x - (point.x - pan.last.x) / contentCamera.zoom,
            y: contentCamera.y - (point.y - pan.last.y) / contentCamera.zoom,
            zoom: contentCamera.zoom,
          };
          panRef.current = { last: point };
          dispatch("setCamera", { camera: next });
        }
        return;
      }
      if (isNavigator || !session) return;
      const marquee = marqueeRef.current;
      if (marquee.tracking) {
        const distance = Math.hypot(point.x - marquee.start.x, point.y - marquee.start.y);
        if (!marquee.active && distance >= PAINT_2D_MARQUEE_THRESHOLD_PX) marquee.active = true;
        if (marquee.active) {
          if (selectionMethod === "lasso") marquee.points = [...marquee.points, point];
          updateMarqueeOverlay(point);
        }
      }
      if (isPaint2dSelectionUtility(scene.activeUtility) && !pickInteraction.pickMenuOpen) {
        pickInteraction.onCanvasPointerMove({ x: event.clientX, y: event.clientY });
        return;
      }
      session.pointerMoveScreen(point.x, point.y);
      const nextCamera = parsePaint2dCameraJson(session.cameraJson());
      if (!paint2dCameraEqual(nextCamera, cameraRef.current)) {
        cameraRef.current = nextCamera;
        dispatch("setCamera", { camera: nextCamera });
      }
      session.renderFrame();
    },
    [clientPoint, dispatch, isNavigator, pickInteraction, scene.activeUtility, scene.cameraJson, selectionMethod, updateMarqueeOverlay],
  );

  const onPointerUp = useCallback(
    (event: ReactPointerEvent<HTMLDivElement>) => {
      const point = clientPoint(event);
      const session = sessionRef.current;
      if (event.currentTarget.hasPointerCapture(event.pointerId)) event.currentTarget.releasePointerCapture(event.pointerId);
      if (panRef.current) {
        panRef.current = null;
        return;
      }
      if (isNavigator || !session) return;
      const marquee = marqueeRef.current;
      if (marquee.tracking) {
        if (marquee.active) {
          const mergeMode = marqueeModeFromModifiers({ shiftKey: event.shiftKey, ctrlKey: event.ctrlKey, metaKey: event.metaKey });
          commitMarqueeSelection(point, mergeMode);
        }
        marqueeRef.current = { tracking: false, active: false, start: point, points: [] };
        setMarqueeOverlay(null);
      }
      if (isPaint2dSelectionUtility(scene.activeUtility)) {
        pickInteraction.onCanvasPointerUp({ x: event.clientX, y: event.clientY }, { shift: event.shiftKey, ctrl: event.ctrlKey, meta: event.metaKey, alt: event.altKey });
        return;
      }
      session.pointerUpScreen(point.x, point.y);
      session.renderFrame();
    },
    [clientPoint, commitMarqueeSelection, isNavigator, pickInteraction, scene.activeUtility],
  );

  const onWheel = useCallback(
    (event: ReactWheelEvent<HTMLDivElement>) => {
      event.preventDefault();
      const point = clientPoint(event);
      const session = sessionRef.current;
      if (isNavigator) {
        const contentCamera = parsePaint2dCameraJson(scene.cameraJson);
        const contentViewport = parsePaint2dViewport(scene.compositeViewportJson) ?? { width: 800, height: 600 };
        const next = wheelCameraAtScreen(contentCamera, point.x, point.y, event.deltaY, contentViewport.width, contentViewport.height);
        dispatch("setCamera", { camera: next });
        return;
      }
      if (!session) return;
      session.wheelScreen(point.x, point.y, event.deltaY);
      const nextCamera = parsePaint2dCameraJson(session.cameraJson());
      cameraRef.current = nextCamera;
      session.renderFrame();
      dispatch("setCamera", { camera: nextCamera });
    },
    [clientPoint, dispatch, isNavigator, scene.cameraJson, scene.compositeViewportJson],
  );
  //#endregion Pointer

  return (
    <div ref={containerRef} className="semio-paint-2d-canvas-surface relative h-full min-h-[24rem] w-full bg-canvas" data-controller-id={node.controllerId} data-surface-id={node.surfaceId} data-view-mode={scene.viewMode}>
      <Paint2dWasmCanvas sessionFactory={sessionFactory} onSessionReady={onSessionReady} />
      {attachError ? (
        <div className="absolute inset-0 flex items-center justify-center bg-canvas text-xs text-muted-foreground">
          {canvasUnavailableLabel}: {attachError}
        </div>
      ) : null}
      {marqueeOverlay?.shape === "rect" ? <SelectionMarquee coverage={marqueeOverlay.coverage} shape="rect" rect={marqueeOverlay.rect} /> : null}
      {marqueeOverlay?.shape === "polygon" ? <SelectionMarquee coverage={marqueeOverlay.coverage} shape="polygon" points={marqueeOverlay.points} /> : null}
      {isNavigator && overlayRect ? <div className="pointer-events-none absolute z-20 border-2 border-accent" style={{ left: overlayRect.x, top: overlayRect.y, width: overlayRect.width, height: overlayRect.height }} /> : null}
      <div
        className="absolute inset-0 z-30"
        onPointerDown={onPointerDown}
        onPointerMove={onPointerMove}
        onPointerUp={onPointerUp}
        onPointerLeave={() => pickInteraction.onCanvasPointerLeave()}
        onWheel={onWheel}
        onContextMenu={(event) => event.preventDefault()}
      />
      {!isNavigator ? (
        <CanvasPickMenu request={pickInteraction.pickMenu} hoveredKey={pickInteraction.menuHoveredKey} onHoverKey={pickInteraction.onMenuHoverKey} onPick={pickInteraction.onMenuPick} onDismiss={pickInteraction.dismissPickMenu} />
      ) : null}
    </div>
  );
}
//#endregion Paint2dCanvasSurface

//#region Paint2dWasmCanvas
/** 🖼️ Minimal canvas-attach wrapper (no pointer forwarding — {@link Paint2dCanvasSurface} owns pointer/wheel routing). */
function Paint2dWasmCanvas({ sessionFactory, onSessionReady }: { readonly sessionFactory: () => RasterWasmSession; readonly onSessionReady: (session: RasterWasmSession) => void }) {
  const containerRef = useRef<HTMLDivElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const rafRef = useRef<number | null>(null);
  const observerRef = useRef<ResizeObserver | null>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    const container = containerRef.current;
    if (!canvas || !container) return;
    const session = sessionFactory();
    onSessionReady(session);
    const rect = container.getBoundingClientRect();
    const dpr = globalThis.devicePixelRatio || 1;
    const initW = Math.max(1, Math.round(rect.width));
    const initH = Math.max(1, Math.round(rect.height));
    canvas.width = Math.round(initW * dpr);
    canvas.height = Math.round(initH * dpr);
    canvas.style.width = `${initW}px`;
    canvas.style.height = `${initH}px`;
    let disposed = false;
    void session
      .attachCanvas(canvas, initW, initH, dpr)
      .then(() => {
        if (disposed) return;
        const resize = () => {
          const nextRect = container.getBoundingClientRect();
          const nextDpr = globalThis.devicePixelRatio || 1;
          const w = Math.max(1, Math.round(nextRect.width));
          const h = Math.max(1, Math.round(nextRect.height));
          canvas.width = Math.round(w * nextDpr);
          canvas.height = Math.round(h * nextDpr);
          canvas.style.width = `${w}px`;
          canvas.style.height = `${h}px`;
          session.setSize(w, h, nextDpr);
          session.renderFrame();
        };
        resize();
        const observer = new ResizeObserver(resize);
        observer.observe(container);
        observerRef.current = observer;
        const tick = () => {
          session.renderFrame();
          rafRef.current = requestAnimationFrame(tick);
        };
        rafRef.current = requestAnimationFrame(tick);
      })
      .catch(() => {
        /* attach failed — surfaced via session.gpuReady() polling in Paint2dCanvasSurface */
      });
    return () => {
      disposed = true;
      if (rafRef.current != null) cancelAnimationFrame(rafRef.current);
      observerRef.current?.disconnect();
      observerRef.current = null;
    };
  }, [onSessionReady, sessionFactory]);

  return (
    <div ref={containerRef} className="absolute inset-0">
      <canvas ref={canvasRef} className="block h-full w-full touch-none" />
    </div>
  );
}
//#endregion Paint2dWasmCanvas

//#region Paint2dHost
export function Paint2dHost({ node, onAction }: ComponentSceneHostProps) {
  const scene = node.paint2d;
  const emptySceneLabel = useLabel("ui.host.emptyScene");
  if (!scene) return <div className="semio-paint-2d-empty">{emptySceneLabel}</div>;
  return <Paint2dCanvasSurface node={node} scene={scene} onAction={onAction} />;
}
//#endregion Paint2dHost
//#endregion 🔖Paint2dHost

//#region 🔖TiledMapHost
//#region Types
type MapCamera = { x: number; y: number; zoom: number };

type MapRenderMode = "image" | "vector" | "combined";

type MapVectorStyle = "colored" | "figureGround" | "invertedFigure";

type MapFeatureKind = "position" | "route";

type MapHoveredFeature = { readonly kind: MapFeatureKind; readonly id: string };

type MapFeatureHit = { readonly positions: readonly string[]; readonly routes: readonly string[] };

type MapPositionMeta = {
  readonly id: string;
  readonly label?: string;
  readonly name?: string;
  readonly icon?: IconName;
  readonly sourceUrl?: string;
};

type VisibleTileRow = { z: number; x: number; y: number; key: string };

const DEFAULT_CAMERA_JSON = '{"x":0,"y":0,"zoom":1}';
const MAP_MARQUEE_THRESHOLD_PX = 6;
const MAX_CONCURRENT_TILE_FETCHES = 12;
const TILE_REFRESH_DEBOUNCE_MS = 120;

const MAP_VELLO_THEME_FALLBACK_RGBA = {
  surfaceClear: [12, 28, 33, 255] as [number, number, number, number],
  landFill: [46, 60, 61, 255] as [number, number, number, number],
  landStroke: [51, 64, 65, 107] as [number, number, number, number],
  labelFill: [247, 243, 227, 255] as [number, number, number, number],
  labelHalo: [12, 28, 33, 235] as [number, number, number, number],
  regionFill: [52, 209, 191, 56] as [number, number, number, number],
  regionStroke: [52, 209, 191, 230] as [number, number, number, number],
  routeStroke: [250, 149, 0, 235] as [number, number, number, number],
  positionFill: [255, 52, 79, 255] as [number, number, number, number],
  positionStroke: [247, 243, 227, 255] as [number, number, number, number],
  selectionStroke: [255, 52, 79, 255] as [number, number, number, number],
  hoverStroke: [52, 209, 191, 235] as [number, number, number, number],
};
//#endregion Types

//#region Parsing
function parseVisibleTilesJson(raw: string): VisibleTileRow[] {
  try {
    const rows = JSON.parse(raw) as VisibleTileRow[];
    return Array.isArray(rows) ? rows : [];
  } catch {
    return [];
  }
}

function parseCameraJson(raw: string): MapCamera | null {
  try {
    const v = JSON.parse(raw) as { x?: number; y?: number; zoom?: number };
    if (typeof v.x !== "number" || typeof v.y !== "number" || typeof v.zoom !== "number") return null;
    return { x: v.x, y: v.y, zoom: v.zoom };
  } catch {
    return null;
  }
}

function parseMapFeatureHit(raw: string): MapFeatureHit {
  try {
    const v = JSON.parse(raw) as { positions?: string[]; routes?: string[] };
    const positions = Array.isArray(v.positions) ? v.positions.filter((id): id is string => typeof id === "string") : [];
    const routes = Array.isArray(v.routes) ? v.routes.filter((id): id is string => typeof id === "string") : [];
    return { positions, routes };
  } catch {
    return { positions: [], routes: [] };
  }
}

function parseMapHoveredFeature(raw: string): MapHoveredFeature | null {
  if (raw === "null") return null;
  try {
    const v = JSON.parse(raw) as { kind?: string; id?: string };
    if ((v.kind === "position" || v.kind === "route") && typeof v.id === "string") {
      return { kind: v.kind, id: v.id };
    }
  } catch {
    return null;
  }
  return null;
}

function parseMapPositionScreen(raw: string): { x: number; y: number } | null {
  if (raw === "null") return null;
  try {
    const v = JSON.parse(raw) as { x?: number; y?: number };
    if (typeof v.x !== "number" || typeof v.y !== "number") return null;
    return { x: v.x, y: v.y };
  } catch {
    return null;
  }
}

function parsePositionMeta(mapFixtureJson: string): Map<string, MapPositionMeta> {
  try {
    const descriptor = JSON.parse(mapFixtureJson) as {
      positions?: Array<{ id?: string; label?: string; name?: string; icon?: string; source_url?: string; sourceUrl?: string }>;
    };
    const out = new Map<string, MapPositionMeta>();
    for (const row of descriptor.positions ?? []) {
      if (typeof row.id !== "string") continue;
      out.set(row.id, {
        id: row.id,
        label: row.label,
        name: row.name,
        icon: row.icon as IconName | undefined,
        sourceUrl: row.source_url ?? row.sourceUrl,
      });
    }
    return out;
  } catch {
    return new Map();
  }
}

function parseFeatureSelection(raw: string): { positions: string[]; routes: string[] } {
  try {
    const v = JSON.parse(raw) as { positions?: string[]; routes?: string[] };
    return {
      positions: Array.isArray(v.positions) ? v.positions.filter((id): id is string => typeof id === "string") : [],
      routes: Array.isArray(v.routes) ? v.routes.filter((id): id is string => typeof id === "string") : [],
    };
  } catch {
    return { positions: [], routes: [] };
  }
}

function getTiledMapCameraLimits(session?: MapWasmSession): { min: number; max: number } {
  if (session) {
    return JSON.parse(session.cameraLimitsJson()) as { min: number; max: number };
  }
  return { min: 0.05, max: 64 };
}
//#endregion Parsing

//#region MapTheme
function mapParseCssColorToRgba8888(css: string, fallback: [number, number, number, number]): [number, number, number, number] {
  const m = css.match(/rgba?\(\s*([\d.]+)\s*,\s*([\d.]+)\s*,\s*([\d.]+)(?:\s*,\s*([\d.]+%?)\s*)?\)/u);
  if (!m) return fallback;
  const r = Math.min(255, Math.max(0, Math.round(Number(m[1]))));
  const g = Math.min(255, Math.max(0, Math.round(Number(m[2]))));
  const b = Math.min(255, Math.max(0, Math.round(Number(m[3]))));
  let a = 255;
  if (m[4] !== undefined && m[4] !== "") {
    const raw = m[4];
    if (raw.endsWith("%")) {
      a = Math.min(255, Math.max(0, Math.round((Number(raw.slice(0, -1)) / 100) * 255)));
    } else {
      const n = Number(raw);
      a = Math.min(255, Math.max(0, Math.round(n <= 1 ? n * 255 : n)));
    }
  }
  return [r, g, b, a];
}

function mapProbeCssComputed(property: "color" | "backgroundColor", value: string): string {
  if (typeof document === "undefined") return "";
  const el = document.createElement("span");
  const key = property === "color" ? "color" : "background-color";
  el.setAttribute("style", `${key}:${value};position:absolute;left:0;top:0;visibility:hidden;pointer-events:none`);
  if (document.documentElement.classList.contains("dark")) el.classList.add("dark");
  document.documentElement.appendChild(el);
  const out = getComputedStyle(el)[property];
  el.remove();
  return out;
}

function serializeMapCanvasThemeJson(): string {
  const fb = MAP_VELLO_THEME_FALLBACK_RGBA;
  const pc = (prop: "color" | "backgroundColor", expr: string, fall: [number, number, number, number]): number[] => {
    const raw = mapProbeCssComputed(prop, expr);
    return [...mapParseCssColorToRgba8888(raw, fall)];
  };
  return JSON.stringify({
    surfaceClear: pc("backgroundColor", "var(--canvas)", fb.surfaceClear),
    landFill: pc("backgroundColor", "color-mix(in oklab, var(--color-muted-foreground) 32%, var(--color-canvas))", fb.landFill),
    landStroke: pc("color", "color-mix(in oklab, var(--color-muted-foreground) 42%, transparent)", fb.landStroke),
    labelFill: pc("color", "var(--foreground)", fb.labelFill),
    labelHalo: pc("backgroundColor", "var(--canvas)", fb.labelHalo),
    regionFill: pc("backgroundColor", "color-mix(in oklab, var(--color-secondary) 22%, transparent)", fb.regionFill),
    regionStroke: pc("color", "var(--color-secondary)", fb.regionStroke),
    routeStroke: pc("color", "var(--color-tertiary)", fb.routeStroke),
    positionFill: pc("backgroundColor", "var(--color-active-base)", fb.positionFill),
    positionStroke: pc("color", "var(--color-active-foreground)", fb.positionStroke),
    selectionStroke: pc("color", "var(--color-active-base)", fb.selectionStroke),
    hoverStroke: pc("color", "var(--color-secondary)", fb.hoverStroke),
  });
}
//#endregion MapTheme

//#region MapRenderer
class MapRenderer {
  readonly session: MapWasmSession;
  camera: MapCamera = { x: 0, y: 0, zoom: 1 };
  private raf = 0;
  private disposed = false;
  private canvasEl: HTMLCanvasElement | null = null;
  private tileCache = new Map<string, ArrayBuffer>();
  private vectorTileCache = new Map<string, ArrayBuffer>();
  private tileMiss = new Set<string>();
  private vectorTileMiss = new Set<string>();
  private tileUrlTemplate: string;
  private vectorTileUrlTemplate: string;
  private renderMode: MapRenderMode = "vector";
  private refreshTimer: ReturnType<typeof setTimeout> | null = null;
  private refreshInFlight: Promise<void> | null = null;
  private lastRasterVisibleKey = "";
  private lastVectorVisibleKey = "";
  private lastPolledRasterVisibleKey = "";
  private lastPolledVectorVisibleKey = "";
  private tilesRefreshQueued = false;
  private lastMapThemeJson = "";
  private logicalWidth = 1;
  private logicalHeight = 1;
  private dpr = 1;

  constructor(tileUrlTemplate: string, vectorTileUrlTemplate: string, session: MapWasmSession) {
    this.session = session;
    this.tileUrlTemplate = tileUrlTemplate;
    this.vectorTileUrlTemplate = vectorTileUrlTemplate;
  }

  private applyCanvasPixelSize(lw: number, lh: number, nextDpr: number): void {
    const canvas = this.canvasEl;
    if (!canvas) return;
    const pw = Math.max(1, Math.round(lw * nextDpr));
    const ph = Math.max(1, Math.round(lh * nextDpr));
    if (canvas.width !== pw || canvas.height !== ph) {
      canvas.width = pw;
      canvas.height = ph;
    }
  }

  setRenderMode(mode: MapRenderMode): void {
    this.renderMode = mode;
    this.session.setRenderMode(mode);
  }

  setVectorStyle(style: MapVectorStyle): void {
    this.session.setVectorStyle(style);
  }

  setLayerVisibilityJson(json: string): void {
    this.session.setLayerVisibilityJson(json);
  }

  setLayerStrokeScaleJson(json: string): void {
    this.session.setLayerStrokeScaleJson(json);
  }

  setLodMode(mode: string): void {
    this.session.setLodMode(mode);
    this.tileCache.clear();
    this.vectorTileCache.clear();
    this.tileMiss.clear();
    this.vectorTileMiss.clear();
    this.lastRasterVisibleKey = "";
    this.lastVectorVisibleKey = "";
    this.lastPolledRasterVisibleKey = "";
    this.lastPolledVectorVisibleKey = "";
    this.scheduleRefreshTiles();
  }

  async attach(canvas: HTMLCanvasElement, width: number, height: number, dpr: number): Promise<void> {
    this.canvasEl = canvas;
    const lw = Math.max(1, Math.round(width));
    const lh = Math.max(1, Math.round(height));
    const nextDpr = dpr > 0 ? dpr : 1;
    this.logicalWidth = lw;
    this.logicalHeight = lh;
    this.dpr = nextDpr;
    this.applyCanvasPixelSize(lw, lh, nextDpr);
    await this.session.attachCanvas(canvas, lw, lh, nextDpr);
  }

  setSize(width: number, height: number, dpr: number): boolean {
    const lw = Math.max(1, Math.round(width));
    const lh = Math.max(1, Math.round(height));
    const nextDpr = dpr > 0 ? dpr : 1;
    if (!this.canvasEl) return false;
    if (lw === this.logicalWidth && lh === this.logicalHeight && nextDpr === this.dpr) return false;
    this.logicalWidth = lw;
    this.logicalHeight = lh;
    this.dpr = nextDpr;
    this.applyCanvasPixelSize(lw, lh, nextDpr);
    this.session.setSize(lw, lh, nextDpr);
    this.session.reclampCamera();
    const parsed = this.readCameraFromSession();
    if (parsed) this.camera = parsed;
    return true;
  }

  syncDescriptor(json: string): void {
    this.session.syncMapJson(json);
  }

  readCameraFromSession(): MapCamera | null {
    return parseCameraJson(this.session.cameraJson());
  }

  applyCameraToSession(camera: MapCamera): void {
    this.camera = camera;
    this.session.setCamera(camera.x, camera.y, camera.zoom);
  }

  private needsRasterTiles(): boolean {
    return this.renderMode === "image" || this.renderMode === "combined";
  }

  private needsVectorTiles(): boolean {
    return this.renderMode === "vector" || this.renderMode === "combined";
  }

  scheduleRefreshTiles(): void {
    if (this.disposed || !this.canvasEl) return;
    if (this.refreshTimer !== null) clearTimeout(this.refreshTimer);
    this.refreshTimer = setTimeout(() => {
      this.refreshTimer = null;
      void this.refreshTiles();
    }, TILE_REFRESH_DEBOUNCE_MS);
  }

  async refreshTiles(): Promise<void> {
    if (this.disposed || !this.canvasEl) return;
    if (this.refreshInFlight) {
      this.tilesRefreshQueued = true;
      return this.refreshInFlight;
    }
    this.refreshInFlight = (async () => {
      const tasks: Promise<void>[] = [];
      if (this.needsRasterTiles()) tasks.push(this.refreshRasterTiles());
      if (this.needsVectorTiles()) tasks.push(this.refreshVectorTiles());
      await Promise.all(tasks);
    })().finally(() => {
      this.refreshInFlight = null;
      if (this.tilesRefreshQueued) {
        this.tilesRefreshQueued = false;
        void this.refreshTiles();
      }
    });
    return this.refreshInFlight;
  }

  private pollVisibleTilesForRefresh(): void {
    if (!this.canvasEl || !this.session.gpuReady()) return;
    if (this.needsRasterTiles()) {
      const rasterKey = this.session.visibleTilesJson();
      if (rasterKey !== this.lastPolledRasterVisibleKey) {
        this.lastPolledRasterVisibleKey = rasterKey;
        this.scheduleRefreshTiles();
      }
    }
    if (this.needsVectorTiles()) {
      const vectorKey = this.session.visibleVectorTilesJson();
      if (vectorKey !== this.lastPolledVectorVisibleKey) {
        this.lastPolledVectorVisibleKey = vectorKey;
        this.scheduleRefreshTiles();
      }
    }
  }

  private async refreshRasterTiles(): Promise<void> {
    if (this.disposed) return;
    const visibleKey = this.session.visibleTilesJson();
    const rows = parseVisibleTilesJson(visibleKey);
    if (rows.length === 0) return;
    if (visibleKey !== this.lastRasterVisibleKey) {
      this.lastRasterVisibleKey = visibleKey;
      this.tileMiss.clear();
    }
    const uploadOne = async (row: VisibleTileRow): Promise<void> => {
      const key = row.key;
      let buf = this.tileCache.get(key);
      if (!buf) {
        if (this.tileMiss.has(key)) return;
        const url = this.tileUrlTemplate.replace("{z}", String(row.z)).replace("{x}", String(row.x)).replace("{y}", String(row.y));
        const res = await fetch(url);
        if (!res.ok) {
          this.tileMiss.add(key);
          return;
        }
        buf = await res.arrayBuffer();
        this.tileCache.set(key, buf);
      }
      if (this.disposed) return;
      this.session.uploadTile(row.z, row.x, row.y, new Uint8Array(buf));
    };
    for (let i = 0; i < rows.length; i += MAX_CONCURRENT_TILE_FETCHES) {
      await Promise.all(rows.slice(i, i + MAX_CONCURRENT_TILE_FETCHES).map((row) => uploadOne(row)));
    }
  }

  async refreshVectorTiles(): Promise<void> {
    if (this.disposed) return;
    const visibleKey = this.session.visibleVectorTilesJson();
    const rows = parseVisibleTilesJson(visibleKey);
    if (rows.length === 0) return;
    if (visibleKey !== this.lastVectorVisibleKey) {
      this.lastVectorVisibleKey = visibleKey;
      this.vectorTileMiss.clear();
    }
    const uploadOne = async (row: VisibleTileRow): Promise<void> => {
      const key = row.key;
      let buf = this.vectorTileCache.get(key);
      if (!buf) {
        if (this.vectorTileMiss.has(key)) return;
        const url = this.vectorTileUrlTemplate.replace("{z}", String(row.z)).replace("{x}", String(row.x)).replace("{y}", String(row.y));
        const res = await fetch(url);
        if (!res.ok) {
          this.vectorTileMiss.add(key);
          return;
        }
        buf = await res.arrayBuffer();
        this.vectorTileCache.set(key, buf);
      }
      if (this.disposed) return;
      this.session.uploadVectorTile(row.z, row.x, row.y, new Uint8Array(buf));
    };
    for (let i = 0; i < rows.length; i += MAX_CONCURRENT_TILE_FETCHES) {
      await Promise.all(rows.slice(i, i + MAX_CONCURRENT_TILE_FETCHES).map((row) => uploadOne(row)));
    }
  }

  private syncMapThemeFromDocument(): void {
    if (typeof document === "undefined") return;
    try {
      const json = serializeMapCanvasThemeJson();
      if (json !== this.lastMapThemeJson) {
        this.lastMapThemeJson = json;
        this.session.setMapThemeJson(json);
      }
    } catch {
      this.lastMapThemeJson = "";
    }
  }

  startLoop(): void {
    const tick = () => {
      if (this.disposed) return;
      this.syncMapThemeFromDocument();
      this.pollVisibleTilesForRefresh();
      void this.session.renderFrame();
      this.raf = requestAnimationFrame(tick);
    };
    this.raf = requestAnimationFrame(tick);
  }

  stopLoop(): void {
    if (this.raf) {
      cancelAnimationFrame(this.raf);
      this.raf = 0;
    }
  }

  dispose(): void {
    if (this.disposed) return;
    this.disposed = true;
    if (this.refreshTimer !== null) {
      clearTimeout(this.refreshTimer);
      this.refreshTimer = null;
    }
    this.stopLoop();
    this.session.free();
    this.canvasEl = null;
  }
}
//#endregion MapRenderer

//#region ContextMenu
function buildTiledMapContextMenuItems(scene: TiledMapScene, feature: MapHoveredFeature | null, dispatch: (action: string, args?: Record<string, unknown>) => void): ContextMenuItem[] {
  const selection = parseFeatureSelection(scene.selectionJson);
  if (feature) {
    const selected = feature.kind === "position" ? selection.positions.includes(feature.id) : selection.routes.includes(feature.id);
    const items: ContextMenuItem[] = [
      {
        id: "tiled-map.ctx.select",
        label: hostLabel("ui.contextMenu.select"),
        onSelect: () =>
          dispatch("setFeatureSelection", {
            positions: feature.kind === "position" ? [feature.id] : [],
            routes: feature.kind === "route" ? [feature.id] : [],
            mode: "default",
          }),
      },
    ];
    if (selected) {
      items.push({
        id: "tiled-map.ctx.deselect",
        label: hostLabel("ui.contextMenu.deselect"),
        onSelect: () => dispatch("deselect", { featureId: feature.id, featureKind: feature.kind }),
      });
    }
    items.push({
      id: "tiled-map.ctx.focus",
      label: hostLabel("ui.contextMenu.focusZoom"),
      onSelect: () => dispatch("focusFeature", { featureId: feature.id, featureKind: feature.kind }),
    });
    if (feature.kind === "position") {
      const meta = parsePositionMeta(scene.mapFixtureJson).get(feature.id);
      if (meta?.sourceUrl) {
        items.push({
          id: "tiled-map.ctx.source",
          label: hostLabel("ui.contextMenu.openSource"),
          onSelect: () => dispatch("openSource", { featureId: feature.id }),
        });
      }
    }
    return items;
  }
  return [
    { id: "tiled-map.ctx.select-all", label: hostLabel("ui.contextMenu.selectAll"), onSelect: () => dispatch("selectAll") },
    {
      id: "tiled-map.ctx.clear",
      label: hostLabel("ui.contextMenu.clearSelection"),
      disabled: selection.positions.length + selection.routes.length === 0,
      onSelect: () => dispatch("clearSelection"),
    },
    { id: "tiled-map.ctx.fit-world", label: hostLabel("ui.contextMenu.fitWorld"), onSelect: () => dispatch("fitWorld") },
  ];
}
//#endregion ContextMenu

//#region TiledMapHost
export function TiledMapHost({ node, onAction }: ComponentSceneHostProps) {
  const scene = node.tiledMap;
  const containerRef = useRef<HTMLDivElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const rendererRef = useRef<MapRenderer | null>(null);
  const panningRef = useRef(false);
  const userAdjustedCameraRef = useRef(false);
  const popupRef = useRef<HTMLDivElement>(null);
  const [marqueeOverlay, setMarqueeOverlay] = useState<
    { coverage: SelectionMarqueeCoverage; shape: "rect"; rect: { x: number; y: number; width: number; height: number } } | { coverage: SelectionMarqueeCoverage; shape: "polygon"; points: readonly SelectionMarqueePoint[] } | null
  >(null);
  const [contextMenu, setContextMenu] = useState<{ open: boolean; position: { x: number; y: number } | null; items: ContextMenuItem[] }>({
    open: false,
    position: null,
    items: [],
  });
  const emptySceneLabel = useLabel("ui.host.emptyScene");
  const sourceAvailableLabel = useLabel("ui.host.sourceAvailable");

  const positionMetaById = useMemo(() => (scene ? parsePositionMeta(scene.mapFixtureJson) : new Map()), [scene?.mapFixtureJson]);
  const hoveredFeature = useMemo(() => (scene ? parseMapHoveredFeature(scene.hoverJson) : null), [scene?.hoverJson]);
  const selectionMethod = (scene?.selectionMethod ?? "rectangle") as SelectionMarqueeMethod;

  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      onAction({ controllerId: node.controllerId, action, args: { surfaceId: node.surfaceId, ...args } });
    },
    [node.controllerId, node.surfaceId, onAction],
  );

  const dispatchCamera = useCallback(
    (camera: MapCamera) => {
      dispatch("setCamera", { camera });
    },
    [dispatch],
  );

  const clampMapZoom = useCallback((zoom: number): number => {
    const { min, max } = getTiledMapCameraLimits(rendererRef.current?.session);
    return Math.min(max, Math.max(min, zoom));
  }, []);

  const clampCamera = useCallback((next: MapCamera): MapCamera => ({ x: next.x, y: next.y, zoom: clampMapZoom(next.zoom) }), [clampMapZoom]);

  const mirrorSessionCameraToReact = useCallback(() => {
    const parsed = rendererRef.current?.readCameraFromSession();
    if (!parsed) return;
    rendererRef.current!.camera = parsed;
    dispatchCamera(parsed);
  }, [dispatchCamera]);

  const clientToLocal = useCallback((clientX: number, clientY: number): SelectionMarqueePoint => {
    const rect = canvasRef.current?.getBoundingClientRect();
    if (!rect) return { x: 0, y: 0 };
    return { x: clientX - rect.left, y: clientY - rect.top };
  }, []);

  const queryFeatureHits = useCallback(
    (points: readonly SelectionMarqueePoint[], crossing: boolean): MapFeatureHit => {
      const session = rendererRef.current?.session;
      if (!session) return { positions: [], routes: [] };
      if (selectionMethod === "lasso" && points.length >= 3) {
        return parseMapFeatureHit(session.featuresInPolygonJson(JSON.stringify(points.map((point) => [point.x, point.y])), crossing));
      }
      const rect = screenRectFromPoints(points);
      if (!rect) return { positions: [], routes: [] };
      return parseMapFeatureHit(session.featuresInRectJson(rect.x, rect.y, rect.x + rect.width, rect.y + rect.height, crossing));
    },
    [selectionMethod],
  );

  const queryHitFeature = useCallback((point: SelectionMarqueePoint): MapHoveredFeature | null => {
    const session = rendererRef.current?.session;
    if (!session) return null;
    return parseMapHoveredFeature(session.hitTestFeatureJson(point.x, point.y));
  }, []);

  const resolveMapPaneElement = useCallback((container: HTMLElement): HTMLElement => {
    let el: HTMLElement | null = container;
    while (el) {
      const slot = el.dataset.slot;
      if (slot === "window" || slot === "mode-dock-stack-body") return el;
      el = el.parentElement;
    }
    return container;
  }, []);

  const readContainerSize = useCallback((): { w: number; h: number } => {
    const container = containerRef.current;
    if (!container) return { w: 1, h: 1 };
    const pane = resolveMapPaneElement(container);
    const rect = pane.getBoundingClientRect();
    const style = globalThis.getComputedStyle(pane);
    const padX = Number.parseFloat(style.paddingLeft) + Number.parseFloat(style.paddingRight);
    const padY = Number.parseFloat(style.paddingTop) + Number.parseFloat(style.paddingBottom);
    const innerW = rect.width - (Number.isFinite(padX) ? padX : 0);
    const innerH = rect.height - (Number.isFinite(padY) ? padY : 0);
    return {
      w: Math.max(1, Math.round(innerW || pane.clientWidth || container.clientWidth)),
      h: Math.max(1, Math.round(innerH || pane.clientHeight || container.clientHeight)),
    };
  }, [resolveMapPaneElement]);

  const mirrorSessionCameraToReactRef = useRef(mirrorSessionCameraToReact);
  mirrorSessionCameraToReactRef.current = mirrorSessionCameraToReact;
  const dispatchCameraRef = useRef(dispatchCamera);
  dispatchCameraRef.current = dispatchCamera;

  useLayoutEffect(() => {
    const canvas = canvasRef.current;
    const container = containerRef.current;
    if (!canvas || !container || !scene) return;
    let disposed = false;
    let resizeRafId: number | null = null;
    let resizeObserver: ResizeObserver | null = null;
    const dpr = globalThis.devicePixelRatio || 1;

    void createMapSession().then((session) => {
      if (disposed) {
        session.free();
        return;
      }
      const renderer = new MapRenderer(scene.tileUrlTemplate, scene.vectorTileUrlTemplate, session);
      renderer.setRenderMode(scene.renderMode as MapRenderMode);
      renderer.setVectorStyle(scene.vectorStyle as MapVectorStyle);
      renderer.setLodMode(scene.lodMode);
      renderer.setLayerVisibilityJson(scene.layerVisibilityJson);
      renderer.setLayerStrokeScaleJson(scene.layerStrokeScaleJson);
      rendererRef.current = renderer;

      const applySize = (): void => {
        const nextDpr = globalThis.devicePixelRatio || 1;
        const { w, h } = readContainerSize();
        if (!renderer.setSize(w, h, nextDpr)) return;
        mirrorSessionCameraToReactRef.current();
        renderer.scheduleRefreshTiles();
      };

      resizeObserver =
        typeof ResizeObserver === "undefined"
          ? null
          : new ResizeObserver(() => {
              if (resizeRafId !== null) return;
              const schedule =
                typeof globalThis.requestAnimationFrame === "function"
                  ? (fn: () => void) => {
                      resizeRafId = globalThis.requestAnimationFrame(() => {
                        resizeRafId = null;
                        fn();
                      });
                    }
                  : (fn: () => void) => {
                      queueMicrotask(fn);
                    };
              schedule(() => {
                if (disposed) return;
                applySize();
              });
            });
      const pane = resolveMapPaneElement(container);
      resizeObserver?.observe(pane);
      if (pane !== container) resizeObserver?.observe(container);

      const boot = async (): Promise<void> => {
        let { w, h } = readContainerSize();
        for (let attempt = 0; attempt < 240 && (w < 64 || h < 64); attempt += 1) {
          await new Promise<void>((resolve) => {
            if (typeof globalThis.requestAnimationFrame === "function") globalThis.requestAnimationFrame(() => resolve());
            else queueMicrotask(resolve);
          });
          if (disposed) return;
          ({ w, h } = readContainerSize());
        }
        await renderer.attach(canvas, w, h, dpr);
        if (disposed) {
          renderer.dispose();
          return;
        }
        applySize();
        if (!userAdjustedCameraRef.current) {
          if (scene.cameraJson === DEFAULT_CAMERA_JSON) {
            renderer.session.fitWorldCamera();
          } else {
            const bootCamera = parseCameraJson(scene.cameraJson);
            if (bootCamera) renderer.applyCameraToSession(clampCamera(bootCamera));
          }
          const bootCamera = renderer.readCameraFromSession();
          if (bootCamera) {
            renderer.applyCameraToSession(bootCamera);
            dispatchCameraRef.current(bootCamera);
          }
        }
        renderer.syncDescriptor(scene.mapFixtureJson);
        renderer.session.setSelectionJson(scene.selectionJson);
        renderer.session.setHoverJson(scene.hoverJson);
        await renderer.refreshTiles();
        renderer.startLoop();
      };

      void boot();
    });

    return () => {
      disposed = true;
      resizeObserver?.disconnect();
      if (resizeRafId !== null && typeof globalThis.cancelAnimationFrame === "function") {
        globalThis.cancelAnimationFrame(resizeRafId);
      }
      rendererRef.current?.dispose();
      rendererRef.current = null;
    };
  }, [clampCamera, readContainerSize, resolveMapPaneElement, scene?.tileUrlTemplate, scene?.vectorTileUrlTemplate]);

  useEffect(() => {
    if (!scene) return;
    const renderer = rendererRef.current;
    if (!renderer) return;
    renderer.setRenderMode(scene.renderMode as MapRenderMode);
    renderer.scheduleRefreshTiles();
  }, [scene?.renderMode]);

  useEffect(() => {
    if (!scene) return;
    rendererRef.current?.setVectorStyle(scene.vectorStyle as MapVectorStyle);
  }, [scene?.vectorStyle]);

  useEffect(() => {
    if (!scene) return;
    rendererRef.current?.setLodMode(scene.lodMode);
  }, [scene?.lodMode]);

  useEffect(() => {
    if (!scene) return;
    rendererRef.current?.setLayerVisibilityJson(scene.layerVisibilityJson);
  }, [scene?.layerVisibilityJson]);

  useEffect(() => {
    if (!scene) return;
    rendererRef.current?.setLayerStrokeScaleJson(scene.layerStrokeScaleJson);
  }, [scene?.layerStrokeScaleJson]);

  useEffect(() => {
    if (!scene) return;
    rendererRef.current?.syncDescriptor(scene.mapFixtureJson);
    rendererRef.current?.scheduleRefreshTiles();
  }, [scene?.mapFixtureJson]);

  useEffect(() => {
    if (!scene) return;
    rendererRef.current?.session.setSelectionJson(scene.selectionJson);
  }, [scene?.selectionJson]);

  useEffect(() => {
    if (!scene) return;
    rendererRef.current?.session.setHoverJson(scene.hoverJson);
  }, [scene?.hoverJson]);

  useEffect(() => {
    if (!scene || panningRef.current) return;
    const camera = parseCameraJson(scene.cameraJson);
    if (!camera) return;
    rendererRef.current?.applyCameraToSession(clampCamera(camera));
    rendererRef.current?.scheduleRefreshTiles();
  }, [clampCamera, scene?.cameraJson]);

  useEffect(() => {
    if (!hoveredFeature || hoveredFeature.kind !== "position") return undefined;
    let raf = 0;
    const tick = () => {
      const screen = rendererRef.current?.session.featureScreenJson("position", hoveredFeature.id);
      const parsed = parseMapPositionScreen(screen ?? "null");
      const popup = popupRef.current;
      if (parsed && popup) {
        popup.style.left = `${parsed.x}px`;
        popup.style.top = `${parsed.y}px`;
      }
      raf = requestAnimationFrame(tick);
    };
    raf = requestAnimationFrame(tick);
    return () => cancelAnimationFrame(raf);
  }, [hoveredFeature]);

  const applyWheelZoom = useCallback(
    (event: WheelEvent): void => {
      event.preventDefault();
      event.stopPropagation();
      const r = rendererRef.current;
      const canvas = canvasRef.current;
      if (!r || !canvas) return;
      const rect = canvas.getBoundingClientRect();
      let deltaY = event.deltaY * (event.deltaMode === WheelEvent.DOM_DELTA_LINE ? 16 : event.deltaMode === WheelEvent.DOM_DELTA_PAGE ? 400 : 1);
      if (event.ctrlKey) deltaY *= 2.5;
      userAdjustedCameraRef.current = true;
      r.session.wheelScreen(event.clientX - rect.left, event.clientY - rect.top, deltaY);
      mirrorSessionCameraToReact();
      r.scheduleRefreshTiles();
    },
    [mirrorSessionCameraToReact],
  );

  useEffect(() => {
    const element = containerRef.current;
    if (!element) return undefined;
    element.addEventListener("wheel", applyWheelZoom, { passive: false });
    return () => element.removeEventListener("wheel", applyWheelZoom);
  }, [applyWheelZoom]);

  const pointer = useRef({
    leftDown: false,
    middleDown: false,
    marqueeTracking: false,
    marqueeActive: false,
    start: { x: 0, y: 0 } as SelectionMarqueePoint,
    points: [] as SelectionMarqueePoint[],
  });

  const resetMarquee = useCallback(() => {
    pointer.current.marqueeTracking = false;
    pointer.current.marqueeActive = false;
    pointer.current.points = [];
    setMarqueeOverlay(null);
  }, []);

  const emitFeatureSelection = useCallback(
    (hits: MapFeatureHit, mode: SelectionMergeMode, crossing: boolean) => {
      dispatch("setFeatureSelection", {
        positions: [...hits.positions],
        routes: [...hits.routes],
        mode,
        crossing,
      });
    },
    [dispatch],
  );

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas || !scene) return undefined;
    const onPointerDown = (event: PointerEvent): void => {
      event.stopPropagation();
      const point = clientToLocal(event.clientX, event.clientY);
      if (event.button === 0) {
        pointer.current.leftDown = true;
        pointer.current.marqueeTracking = true;
        pointer.current.marqueeActive = false;
        pointer.current.start = point;
        pointer.current.points = [point];
        canvas.setPointerCapture?.(event.pointerId);
        return;
      }
      if (event.button === 1) {
        event.preventDefault();
        pointer.current.middleDown = true;
        panningRef.current = true;
        userAdjustedCameraRef.current = true;
        canvas.setPointerCapture?.(event.pointerId);
        rendererRef.current?.session.pointerDownScreen(point.x, point.y, 1);
      }
    };
    const onPointerMove = (event: PointerEvent): void => {
      const point = clientToLocal(event.clientX, event.clientY);
      if (pointer.current.middleDown) {
        event.stopPropagation();
        rendererRef.current?.session.pointerMoveScreen(point.x, point.y);
        mirrorSessionCameraToReact();
        rendererRef.current?.scheduleRefreshTiles();
        return;
      }
      if (!pointer.current.marqueeTracking) {
        const hit = queryHitFeature(point);
        const nextHover = hit ? { kind: hit.kind, id: hit.id } : null;
        const currentHover = parseMapHoveredFeature(scene.hoverJson);
        if ((currentHover?.id ?? null) !== (nextHover?.id ?? null) || (currentHover?.kind ?? null) !== (nextHover?.kind ?? null)) {
          dispatch("setHover", { hover: nextHover });
        }
        return;
      }
      event.stopPropagation();
      const distance = Math.hypot(point.x - pointer.current.start.x, point.y - pointer.current.start.y);
      if (!pointer.current.marqueeActive && distance >= MAP_MARQUEE_THRESHOLD_PX) {
        pointer.current.marqueeActive = true;
      }
      if (!pointer.current.marqueeActive) return;
      const method = selectionMethod;
      const points = method === "lasso" ? [...pointer.current.points, point] : [pointer.current.start, point];
      pointer.current.points = points;
      const coverage = marqueeCoverageFromGesture({
        method,
        startX: pointer.current.start.x,
        endX: point.x,
        path: points,
      });
      const rect = screenRectFromPoints(points);
      setMarqueeOverlay(method === "lasso" ? { coverage, shape: "polygon", points } : { coverage, shape: "rect", rect: rect ?? { x: 0, y: 0, width: 0, height: 0 } });
    };
    const onPointerUp = (event: PointerEvent): void => {
      event.stopPropagation();
      const point = clientToLocal(event.clientX, event.clientY);
      if (event.button === 1 && pointer.current.middleDown) {
        pointer.current.middleDown = false;
        panningRef.current = false;
        rendererRef.current?.session.pointerUpScreen(point.x, point.y);
        if (canvas.hasPointerCapture?.(event.pointerId)) canvas.releasePointerCapture(event.pointerId);
        mirrorSessionCameraToReact();
        rendererRef.current?.scheduleRefreshTiles();
        return;
      }
      if (event.button !== 0 || !pointer.current.leftDown) return;
      pointer.current.leftDown = false;
      if (canvas.hasPointerCapture?.(event.pointerId)) canvas.releasePointerCapture(event.pointerId);
      const distance = Math.hypot(point.x - pointer.current.start.x, point.y - pointer.current.start.y);
      const mode = marqueeModeFromModifiers(event);
      const method = selectionMethod;
      if (pointer.current.marqueeActive && distance >= MAP_MARQUEE_THRESHOLD_PX) {
        const points = method === "lasso" ? [...pointer.current.points, point] : [pointer.current.start, point];
        const coverage = marqueeCoverageFromGesture({
          method,
          startX: pointer.current.start.x,
          endX: point.x,
          path: points,
        });
        emitFeatureSelection(queryFeatureHits(points, coverage === "partial"), mode, coverage === "partial");
      } else if (distance < MAP_MARQUEE_THRESHOLD_PX) {
        const hit = queryHitFeature(point);
        emitFeatureSelection(
          {
            positions: hit?.kind === "position" ? [hit.id] : [],
            routes: hit?.kind === "route" ? [hit.id] : [],
          },
          mode,
          false,
        );
      }
      resetMarquee();
    };
    const onPointerCancel = (event: PointerEvent): void => {
      pointer.current.leftDown = false;
      pointer.current.middleDown = false;
      panningRef.current = false;
      resetMarquee();
      if (canvas.hasPointerCapture?.(event.pointerId)) canvas.releasePointerCapture(event.pointerId);
      mirrorSessionCameraToReact();
    };
    const onContextMenu = (event: MouseEvent): void => {
      event.preventDefault();
      event.stopPropagation();
      const point = clientToLocal(event.clientX, event.clientY);
      const feature = queryHitFeature(point);
      const items = buildTiledMapContextMenuItems(scene, feature, dispatch);
      setContextMenu({ open: items.length > 0, position: { x: event.clientX, y: event.clientY }, items });
    };
    canvas.addEventListener("pointerdown", onPointerDown);
    window.addEventListener("pointermove", onPointerMove);
    window.addEventListener("pointerup", onPointerUp);
    window.addEventListener("pointercancel", onPointerCancel);
    canvas.addEventListener("contextmenu", onContextMenu);
    return () => {
      canvas.removeEventListener("pointerdown", onPointerDown);
      window.removeEventListener("pointermove", onPointerMove);
      window.removeEventListener("pointerup", onPointerUp);
      window.removeEventListener("pointercancel", onPointerCancel);
      canvas.removeEventListener("contextmenu", onContextMenu);
    };
  }, [clientToLocal, dispatch, emitFeatureSelection, mirrorSessionCameraToReact, queryFeatureHits, queryHitFeature, resetMarquee, scene, selectionMethod]);

  if (!scene) return <div className="semio-tiled-map-empty text-muted-foreground p-2 text-xs">{emptySceneLabel}</div>;

  return (
    <div ref={containerRef} className="semio-tiled-map-host absolute inset-0 box-border min-h-0 min-w-0 overflow-hidden select-none" data-surface-id={node.surfaceId} style={{ touchAction: "none" }}>
      <canvas ref={canvasRef} className="absolute inset-0 block size-full touch-none outline-none focus:outline-none" />
      {marqueeOverlay?.shape === "rect" ? <SelectionMarquee coverage={marqueeOverlay.coverage} shape="rect" rect={marqueeOverlay.rect} /> : null}
      {marqueeOverlay?.shape === "polygon" ? <SelectionMarquee coverage={marqueeOverlay.coverage} shape="polygon" points={marqueeOverlay.points} /> : null}
      <ContextMenuController open={contextMenu.open} position={contextMenu.position} items={contextMenu.items} onOpenChange={(open) => setContextMenu((prev) => ({ ...prev, open }))} />
      {hoveredFeature?.kind === "position" ? (
        <div ref={popupRef} className={cn("pointer-events-none absolute z-10 max-w-56 -translate-x-1/2 -translate-y-[calc(100%+12px)] px-2 py-1.5", floatingMenuSurfaceClass)} style={{ left: 0, top: 0 }}>
          {(() => {
            const meta = positionMetaById.get(hoveredFeature.id);
            const title = meta?.name ?? meta?.label ?? hoveredFeature.id;
            return (
              <div className="flex items-start gap-1.5">
                {meta?.icon ? <Icon icon={meta.icon} size="small" className="mt-0.5 shrink-0" /> : null}
                <div className="min-w-0">
                  <div className="truncate text-sm font-medium">{title}</div>
                  {meta?.sourceUrl ? <span className="text-xs text-secondary underline-offset-2">{sourceAvailableLabel}</span> : null}
                </div>
              </div>
            );
          })()}
        </div>
      ) : null}
    </div>
  );
}
//#endregion TiledMapHost
//#endregion 🔖TiledMapHost

//#region 🔖Board2dHost
//#region Types
type BoardCamera = { readonly x: number; readonly y: number; readonly zoom: number };
type BoardEventRow = { readonly name: string; readonly payload?: unknown };
type Puzzle2dSelectionMenuItem = {
  readonly id: string;
  readonly label: string;
  readonly action: string;
  readonly args?: Record<string, unknown>;
  readonly destructive?: boolean;
  readonly disabled?: boolean;
};
type Puzzle2dFixtureDropPayload = {
  readonly kindId: string;
  readonly catalogSlice: string;
  readonly shape?: string;
  readonly radius?: number;
  readonly width?: number;
  readonly height?: number;
  readonly iconKind?: string;
};
//#endregion Types

//#region Parsing
function parseBoardCamera(json: string): BoardCamera | null {
  try {
    const parsed = JSON.parse(json) as Partial<BoardCamera>;
    if (typeof parsed.x !== "number" || typeof parsed.y !== "number" || typeof parsed.zoom !== "number") return null;
    return { x: parsed.x, y: parsed.y, zoom: parsed.zoom };
  } catch {
    return null;
  }
}

export function board2dCameraActionArgs(cameraJson: string): { readonly camera: BoardCamera } | null {
  const camera = parseBoardCamera(cameraJson);
  return camera ? { camera } : null;
}

export function parsePuzzle2dCatalogueDragPayload(encoded: string | null | undefined): Puzzle2dFixtureDropPayload | null {
  if (!encoded) return null;
  try {
    const parsed = JSON.parse(encoded) as Partial<Puzzle2dFixtureDropPayload>;
    if (typeof parsed.kindId !== "string") return null;
    return {
      kindId: parsed.kindId,
      catalogSlice: typeof parsed.catalogSlice === "string" ? parsed.catalogSlice : "nodes",
      shape: typeof parsed.shape === "string" ? parsed.shape : undefined,
      radius: typeof parsed.radius === "number" ? parsed.radius : undefined,
      width: typeof parsed.width === "number" ? parsed.width : undefined,
      height: typeof parsed.height === "number" ? parsed.height : undefined,
      iconKind: typeof parsed.iconKind === "string" ? parsed.iconKind : undefined,
    };
  } catch {
    return null;
  }
}
//#endregion Parsing

//#region BoardEvents
const PUZZLE2D_TRANSIENT_EVENT_NAMES = new Set(["preselect", "brushPreview", "linkCompatibleNodes", "linkTargetRing"]);
const PUZZLE2D_FLUSH_NOW_EVENT_NAMES = new Set(["select", "preselectCancel", "brushCandidates", "brushPlace", "edgeCreate", "edgeDelete", "nodeDelete"]);

/** @emoji 📬 Drops transient rows, coalesces `camera` to its latest value and `nodeMove` to one row per id (unless a `nodeDragEnd` follows), and flags whether the buffer should flush immediately. */
export function coalesceBoard2dEvents(rows: readonly BoardEventRow[]): { readonly flushNow: boolean; readonly eventsJson: string } {
  const hasDragEnd = rows.some((row) => row.name === "nodeDragEnd");
  let flushNow = false;
  let lastCamera: BoardEventRow | null = null;
  const nodeMoveById = new Map<string, BoardEventRow>();
  const rest: BoardEventRow[] = [];

  for (const row of rows) {
    if (PUZZLE2D_TRANSIENT_EVENT_NAMES.has(row.name)) continue;
    if (row.name === "camera") {
      lastCamera = row;
      continue;
    }
    if (row.name === "nodeMove") {
      if (hasDragEnd) continue;
      const id = (row.payload as { readonly id?: unknown } | undefined)?.id;
      if (typeof id === "string") {
        nodeMoveById.set(id, row);
        continue;
      }
    }
    if (PUZZLE2D_FLUSH_NOW_EVENT_NAMES.has(row.name)) flushNow = true;
    rest.push(row);
  }

  const coalesced: BoardEventRow[] = [];
  if (lastCamera) coalesced.push(lastCamera);
  coalesced.push(...nodeMoveById.values());
  coalesced.push(...rest);
  return { flushNow, eventsJson: JSON.stringify(coalesced) };
}

/** @emoji 🐢 Live cross-pane mirror payload extracted from a batch of freshly-drained rows — positions/selection/preselect only, everything else (camera, brush/link chrome, hover) stays pane-local. */
export type Puzzle2dLiveMirrorOps = {
  readonly positions: readonly { readonly id: string; readonly x: number; readonly y: number }[];
  readonly selectionIds: readonly string[] | null;
  readonly preselect: { readonly ids: readonly string[]; readonly removedIds: readonly string[] } | null;
  readonly clearPreselect: boolean;
};

function stringArray(value: unknown): readonly string[] {
  return Array.isArray(value) ? value.filter((entry): entry is string => typeof entry === "string") : [];
}

/**
 * @emoji 🐢 Classifies a batch of raw board-event rows (as seen straight off `drainEventsJson`, before
 * the transient-event filter/coalescer runs) into the subset worth mirroring imperatively into sibling
 * panes: latest node position per id (from `nodeMove` frames and/or a terminal `nodeDragEnd`), and the
 * live selection/preselect state (`select`/`preselectCancel` commit or restore selection and clear
 * preselect; `preselect` sets the live marquee highlight). Multiple rows of the same kind in one batch
 * collapse to the latest.
 */
export function collectPuzzle2dLiveMirrorOps(rows: readonly BoardEventRow[]): Puzzle2dLiveMirrorOps {
  const positionsById = new Map<string, { readonly id: string; readonly x: number; readonly y: number }>();
  let selectionIds: readonly string[] | null = null;
  let preselect: { readonly ids: readonly string[]; readonly removedIds: readonly string[] } | null = null;
  let clearPreselect = false;

  for (const row of rows) {
    const payload = row.payload as Record<string, unknown> | undefined;
    switch (row.name) {
      case "nodeMove": {
        const id = payload?.id;
        const x = payload?.x;
        const y = payload?.y;
        if (typeof id === "string" && typeof x === "number" && typeof y === "number") positionsById.set(id, { id, x, y });
        break;
      }
      case "nodeDragEnd": {
        const moves = payload?.moves;
        if (!Array.isArray(moves)) break;
        for (const move of moves as readonly Record<string, unknown>[]) {
          const id = move.id;
          const x = move.x;
          const y = move.y;
          if (typeof id === "string" && typeof x === "number" && typeof y === "number") positionsById.set(id, { id, x, y });
        }
        break;
      }
      case "preselect": {
        preselect = { ids: stringArray(payload?.ids), removedIds: stringArray(payload?.removedIds) };
        clearPreselect = false;
        break;
      }
      case "preselectCancel": {
        selectionIds = stringArray(payload?.ids);
        preselect = null;
        clearPreselect = true;
        break;
      }
      case "select": {
        selectionIds = stringArray(payload?.ids);
        preselect = null;
        clearPreselect = true;
        break;
      }
      default:
        break;
    }
  }

  return { positions: [...positionsById.values()], selectionIds, preselect, clearPreselect };
}
//#endregion BoardEvents

//#region SelectionMenu
function puzzle2dEntityFlag(entity: Record<string, unknown> | undefined, key: "hidden" | "locked"): boolean {
  return Boolean(entity && entity[key] === true);
}

/** @emoji 🖱️ Right-click menu for the current selection: Hide/Show, Lock/Unlock, Duplicate, Select same kind, Zoom to selection, Delete — mirrors the premigration canvas context menu. */
export function buildPuzzle2dSelectionMenuItems(fixtureJson: string, selectionJson: string): readonly Puzzle2dSelectionMenuItem[] {
  let fixture: { readonly nodes?: readonly Record<string, unknown>[]; readonly edges?: readonly Record<string, unknown>[] } = {};
  try {
    fixture = JSON.parse(fixtureJson) as typeof fixture;
  } catch {
    /* empty fixture */
  }
  const selected = parseSelectionIds(selectionJson);
  if (selected.length === 0) {
    return [{ id: "selectAll", label: hostLabel("ui.contextMenu.selectAll"), action: "selectAll" }];
  }

  const selectedSet = new Set(selected);
  const nodes = fixture.nodes ?? [];
  const edges = fixture.edges ?? [];
  const selectedEntities: Record<string, unknown>[] = [];
  let hasSelectedNode = false;
  for (const node of nodes) {
    const id = node.id;
    if (typeof id === "string" && selectedSet.has(id)) {
      selectedEntities.push(node);
      hasSelectedNode = true;
    }
    const handles = node.handles;
    if (Array.isArray(handles)) {
      for (const handle of handles as Record<string, unknown>[]) {
        const handleId = handle.id;
        if (typeof handleId === "string" && selectedSet.has(handleId)) selectedEntities.push(handle);
      }
    }
  }
  for (const edge of edges) {
    const id = edge.id;
    if (typeof id === "string" && selectedSet.has(id)) selectedEntities.push(edge);
  }

  const anyVisible = selectedEntities.some((entity) => !puzzle2dEntityFlag(entity, "hidden"));
  const anyUnlocked = selectedEntities.some((entity) => !puzzle2dEntityFlag(entity, "locked"));

  return [
    { id: "toggleHidden", label: anyVisible ? "Hide" : "Show", action: "setSelectionFlag", args: { flag: "hidden", value: anyVisible } },
    { id: "toggleLocked", label: anyUnlocked ? "Lock" : "Unlock", action: "setSelectionFlag", args: { flag: "locked", value: anyUnlocked } },
    { id: "duplicate", label: hostLabel("ui.contextMenu.duplicate"), action: "duplicateSelection", disabled: !hasSelectedNode },
    { id: "selectSameKind", label: "Select all of same kind", action: "selectSameKind" },
    { id: "focusSelection", label: hostLabel("ui.contextMenu.zoomToSelection"), action: "focusSelection" },
    { id: "deleteSelection", label: hostLabel("ui.contextMenu.delete"), action: "deleteSelection", destructive: true },
  ];
}
//#endregion SelectionMenu

//#region FixtureDrop
export function puzzle2dFixtureDropPreviewJson(payload: Puzzle2dFixtureDropPayload, screenX: number, screenY: number): string {
  return JSON.stringify({ nodeKind: payload.kindId, screenX, screenY, shape: payload.shape, radius: payload.radius, width: payload.width, height: payload.height, iconKind: payload.iconKind });
}

/** @emoji 📐 Inverse of the canonical `screenX = (worldX - camera.x) * zoom + width / 2` transform shared across board renderers. */
export function puzzle2dScreenToWorld(cameraJson: string, containerSize: { readonly w: number; readonly h: number }, screen: { readonly x: number; readonly y: number }): { readonly x: number; readonly y: number } | null {
  const camera = parseBoardCamera(cameraJson);
  if (!camera) return null;
  const zoom = camera.zoom || 1;
  return {
    x: camera.x + (screen.x - containerSize.w / 2) / zoom,
    y: camera.y + (screen.y - containerSize.h / 2) / zoom,
  };
}
//#endregion FixtureDrop

//#region Sync
function applyToSession(session: Board2dWasmSession | null, action: (session: Board2dWasmSession) => void): void {
  if (!session) return;
  try {
    action(session);
    session.renderFrame();
  } catch {
    /* session not ready */
  }
}

/** @emoji 🔁 Re-parses the fixture and silently re-applies selection/camera, since `parseFixtureJson` resets both to the fixture's own defaults. */
function applyFixtureToSession(session: Board2dWasmSession, scene: Board2dScene): void {
  session.parseFixtureJson(scene.fixtureJson);
  session.setSelectionOptions?.(scene.selectionMethod, "replace", true, true, true);
  if (session.setSelectionIdsJsonSilent) session.setSelectionIdsJsonSilent(scene.selectionJson);
  else session.setSelectionIdsJson(scene.selectionJson);
  const camera = parseBoardCamera(scene.cameraJson);
  if (camera) {
    if (session.setCameraSilent) session.setCameraSilent(camera.x, camera.y, camera.zoom);
    else session.setCamera(camera.x, camera.y, camera.zoom);
  }
}
//#endregion Sync

//#region PeerSync
/** @emoji 🐢 One triptych pane, registered so siblings can mirror its live gesture state without a plugin round trip. */
type Board2dPeer = {
  readonly session: Board2dWasmSession;
  /** @emoji 🫧 `flushed` is true when the gesture that just ended pushed a commit to the plugin — in that case a fresh scene is already in flight and any stashed pending echo should be dropped rather than applied, or it would flash the stale in-between state for one frame before the fresh one supersedes it. */
  readonly onPeerGestureEnded: (flushed: boolean) => void;
};

/** @emoji 🗺️ controllerId -> surfaceId -> peer. Assumes one puzzle2d-play triptych on screen at a time (matches the existing controllerId/surfaceId scoping used for action routing). */
const board2dPeerRegistry = new Map<string, Map<string, Board2dPeer>>();
/** @emoji 🔒 controllerId -> surfaceId of the pane currently owning a live pointer gesture (drag/marquee), so siblings can defer conflicting echoes. */
const board2dGestureOwner = new Map<string, string>();

export function registerBoard2dPeer(controllerId: string, surfaceId: string, peer: Board2dPeer): void {
  let peers = board2dPeerRegistry.get(controllerId);
  if (!peers) {
    peers = new Map();
    board2dPeerRegistry.set(controllerId, peers);
  }
  peers.set(surfaceId, peer);
}

export function unregisterBoard2dPeer(controllerId: string, surfaceId: string): void {
  const peers = board2dPeerRegistry.get(controllerId);
  if (!peers) return;
  peers.delete(surfaceId);
  if (peers.size === 0) board2dPeerRegistry.delete(controllerId);
  if (board2dGestureOwner.get(controllerId) === surfaceId) board2dGestureOwner.delete(controllerId);
}

export function board2dPeers(controllerId: string, excludeSurfaceId: string): readonly Board2dPeer[] {
  const peers = board2dPeerRegistry.get(controllerId);
  if (!peers) return [];
  const result: Board2dPeer[] = [];
  for (const [surfaceId, peer] of peers) if (surfaceId !== excludeSurfaceId) result.push(peer);
  return result;
}

export function beginPuzzle2dPeerGesture(controllerId: string, surfaceId: string): void {
  board2dGestureOwner.set(controllerId, surfaceId);
}

export function endPuzzle2dPeerGesture(controllerId: string, surfaceId: string): void {
  if (board2dGestureOwner.get(controllerId) === surfaceId) board2dGestureOwner.delete(controllerId);
}

/** @emoji 🙅 True when a *different* pane owns the live gesture for this controller — the caller should defer applying an echoed scene. */
export function puzzle2dPeerOwnsGesture(controllerId: string, surfaceId: string): boolean {
  const owner = board2dGestureOwner.get(controllerId);
  return owner !== undefined && owner !== surfaceId;
}

export function pushPuzzle2dLiveMirrorOps(controllerId: string, surfaceId: string, ops: Puzzle2dLiveMirrorOps): void {
  if (ops.positions.length === 0 && !ops.selectionIds && !ops.preselect && !ops.clearPreselect) return;
  const peers = board2dPeers(controllerId, surfaceId);
  if (peers.length === 0) return;
  const positionsJson = ops.positions.length > 0 ? JSON.stringify(ops.positions) : null;
  const selectionJson = ops.selectionIds ? JSON.stringify(ops.selectionIds) : null;
  const preselectJson = ops.preselect ? JSON.stringify(ops.preselect) : ops.clearPreselect ? JSON.stringify({ ids: [], removedIds: [] }) : null;
  for (const peer of peers) {
    try {
      if (positionsJson) peer.session.setNodePositionsJson?.(positionsJson);
      if (selectionJson) peer.session.setSelectionIdsJsonSilent?.(selectionJson);
      if (preselectJson) peer.session.setPreselectStateJsonSilent?.(preselectJson);
    } catch {
      /* peer session not ready */
    }
  }
}

export function notifyPuzzle2dPeersGestureEnded(controllerId: string, surfaceId: string, flushed: boolean): void {
  for (const peer of board2dPeers(controllerId, surfaceId)) {
    try {
      peer.onPeerGestureEnded(flushed);
    } catch {
      /* peer session not ready */
    }
  }
}
//#endregion PeerSync

//#region Board2dHost
export function Board2dHost({ node, onAction }: ComponentSceneHostProps) {
  const scene = node.board2d;
  const emptySceneLabel = useLabel("ui.host.emptyScene");
  const containerRef = useRef<HTMLDivElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const sessionRef = useRef<Board2dWasmSession | null>(null);
  const bootSyncedRef = useRef(false);
  const pendingFixtureSceneRef = useRef<Board2dScene | null>(null);
  const pendingEventRowsRef = useRef<BoardEventRow[]>([]);
  const hoverActiveRef = useRef(false);
  const cameraInteractionActiveRef = useRef(false);
  const cameraSettleTimeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const renderScheduledRef = useRef(false);
  const pendingCameraDispatchRef = useRef<{ readonly camera: BoardCamera } | null>(null);
  const pendingSelectionJsonRef = useRef<string | null>(null);
  const onPeerGestureEndedRef = useRef<() => void>(() => {});
  const [sessionEpoch, setSessionEpoch] = useState(0);
  const [contextMenu, setContextMenu] = useState<{ readonly x: number; readonly y: number; readonly items: readonly Puzzle2dSelectionMenuItem[] } | null>(null);

  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      onAction({ controllerId: node.controllerId, action, args: { surfaceId: node.surfaceId, ...args } });
    },
    [node.controllerId, node.surfaceId, onAction],
  );

  /** @emoji 🎞️ Coalesces renderFrame() to at most one per animation frame, no matter how many raw pointer/wheel events fire in between — mirrors the premigration `scheduleInputInvalidate()` pattern. */
  const scheduleRender = useCallback((): void => {
    if (renderScheduledRef.current) return;
    renderScheduledRef.current = true;
    requestAnimationFrame(() => {
      renderScheduledRef.current = false;
      try {
        sessionRef.current?.renderFrame();
      } catch {
        /* gpu not ready */
      }
    });
  }, []);

  const readContainerSize = useCallback((): { w: number; h: number } => {
    const container = containerRef.current;
    if (!container) return { w: 1, h: 1 };
    const rect = container.getBoundingClientRect();
    return {
      w: Math.max(1, Math.round(rect.width || container.clientWidth)),
      h: Math.max(1, Math.round(rect.height || container.clientHeight)),
    };
  }, []);

  //#region BoardEventFlush
  const drainIntoBuffer = useCallback((): void => {
    const session = sessionRef.current;
    if (!session) return;
    try {
      const json = session.drainEventsJson();
      if (!json || json === "[]") return;
      const rows = JSON.parse(json) as BoardEventRow[];
      pendingEventRowsRef.current.push(...rows);
      pushPuzzle2dLiveMirrorOps(node.controllerId, node.surfaceId, collectPuzzle2dLiveMirrorOps(rows));
    } catch {
      /* session not ready */
    }
  }, [node.controllerId, node.surfaceId]);

  const dispatchBufferedEvents = useCallback((): void => {
    if (pendingEventRowsRef.current.length === 0) return;
    const { eventsJson } = coalesceBoard2dEvents(pendingEventRowsRef.current);
    pendingEventRowsRef.current = [];
    if (eventsJson && eventsJson !== "[]") dispatch("applyBoardEvents", { eventsJson });
  }, [dispatch]);

  const drainAndMaybeFlush = useCallback((): void => {
    drainIntoBuffer();
    if (pendingEventRowsRef.current.length === 0) return;
    const { flushNow } = coalesceBoard2dEvents(pendingEventRowsRef.current);
    if (flushNow) dispatchBufferedEvents();
  }, [drainIntoBuffer, dispatchBufferedEvents]);

  const flushBoardEvents = useCallback((): void => {
    drainIntoBuffer();
    dispatchBufferedEvents();
  }, [drainIntoBuffer, dispatchBufferedEvents]);

  const applyPendingFixtureIfReady = useCallback(
    (session: Board2dWasmSession): void => {
      const pendingScene = pendingFixtureSceneRef.current;
      if (!pendingScene) return;
      if (session.defersDescriptorSyncFromJs?.() || cameraInteractionActiveRef.current || puzzle2dPeerOwnsGesture(node.controllerId, node.surfaceId)) return;
      pendingFixtureSceneRef.current = null;
      applyToSession(session, (s) => applyFixtureToSession(s, pendingScene));
    },
    [node.controllerId, node.surfaceId],
  );

  /** @emoji 🐢 Mirror of `applyPendingFixtureIfReady` for the selection-only echo — a peer-owned gesture defers the plugin's `selectionJson` so it doesn't clobber a mirrored preselect highlight mid-marquee. */
  const applyPendingSelectionIfReady = useCallback(
    (session: Board2dWasmSession): void => {
      const pendingSelectionJson = pendingSelectionJsonRef.current;
      if (pendingSelectionJson === null) return;
      if (puzzle2dPeerOwnsGesture(node.controllerId, node.surfaceId)) return;
      pendingSelectionJsonRef.current = null;
      applyToSession(session, (s) => {
        if (s.setSelectionIdsJsonSilent) s.setSelectionIdsJsonSilent(pendingSelectionJson);
        else s.setSelectionIdsJson(pendingSelectionJson);
      });
    },
    [node.controllerId, node.surfaceId],
  );

  onPeerGestureEndedRef.current = (flushed: boolean): void => {
    const session = sessionRef.current;
    if (!session) return;
    if (flushed) {
      pendingFixtureSceneRef.current = null;
      pendingSelectionJsonRef.current = null;
      return;
    }
    applyPendingFixtureIfReady(session);
    applyPendingSelectionIfReady(session);
  };

  /**
   * @emoji 🫧 Call when a gesture on this pane ends, right before flushing. Drains first so we know
   * whether a commit is about to go out; if so, drops any pending fixture/selection stashed mid-gesture
   * instead of applying it — that stashed snapshot is stale (typically from an early mid-gesture flush,
   * e.g. the `select` event a node-drag's pointerdown pushes) and the flush response due back in a moment
   * will supersede it anyway, so applying it here would flicker: correct live state -> stale snapshot ->
   * correct committed state. Returns whether a flush is pending, so the caller can pass it on to peers.
   */
  const settleGestureEnd = useCallback(
    (session: Board2dWasmSession): boolean => {
      drainIntoBuffer();
      const flushed = pendingEventRowsRef.current.length > 0;
      if (flushed) {
        pendingFixtureSceneRef.current = null;
        pendingSelectionJsonRef.current = null;
      } else {
        applyPendingFixtureIfReady(session);
        applyPendingSelectionIfReady(session);
      }
      return flushed;
    },
    [applyPendingFixtureIfReady, applyPendingSelectionIfReady, drainIntoBuffer],
  );

  /** @emoji 🐁 Marks a wheel-zoom gesture in flight so scene-driven camera echoes (which lag several ticks behind during a fast scroll) don't fight the live local zoom — mirrors `defersDescriptorSyncFromJs` for pan/drag, which the engine doesn't track for wheel. */
  const beginCameraInteraction = useCallback((): void => {
    cameraInteractionActiveRef.current = true;
    if (cameraSettleTimeoutRef.current) clearTimeout(cameraSettleTimeoutRef.current);
    cameraSettleTimeoutRef.current = setTimeout(() => {
      cameraInteractionActiveRef.current = false;
      cameraSettleTimeoutRef.current = null;
      const session = sessionRef.current;
      if (session) applyPendingFixtureIfReady(session);
      const pendingCamera = pendingCameraDispatchRef.current;
      if (pendingCamera) {
        pendingCameraDispatchRef.current = null;
        dispatch("setCamera", pendingCamera);
      }
    }, 350);
  }, [applyPendingFixtureIfReady, dispatch]);

  useEffect(
    () => () => {
      if (cameraSettleTimeoutRef.current) clearTimeout(cameraSettleTimeoutRef.current);
    },
    [],
  );
  //#endregion BoardEventFlush

  //#region SessionLifecycle
  useLayoutEffect(() => {
    const canvas = canvasRef.current;
    const container = containerRef.current;
    if (!canvas || !container) return undefined;
    let disposed = false;
    let resizeObserver: ResizeObserver | null = null;
    let raf = 0;

    void createBoard2dSession().then((session) => {
      if (disposed) {
        session.free();
        return;
      }
      sessionRef.current = session;
      registerBoard2dPeer(node.controllerId, node.surfaceId, { session, onPeerGestureEnded: () => onPeerGestureEndedRef.current() });

      const applySize = (): void => {
        const nextDpr = globalThis.devicePixelRatio || 1;
        const { w, h } = readContainerSize();
        session.setSize(w, h, nextDpr);
      };

      const boot = async (): Promise<void> => {
        let { w, h } = readContainerSize();
        for (let attempt = 0; attempt < 240 && (w < 64 || h < 64); attempt += 1) {
          await new Promise<void>((resolve) => {
            if (typeof globalThis.requestAnimationFrame === "function") globalThis.requestAnimationFrame(() => resolve());
            else queueMicrotask(resolve);
          });
          if (disposed) return;
          ({ w, h } = readContainerSize());
        }
        const dpr = globalThis.devicePixelRatio || 1;
        await session.attach_canvas(canvas, w, h, dpr);
        if (disposed) {
          session.free();
          return;
        }
        applySize();
        syncSessionCanvasTheme(session);
        const tick = () => {
          if (disposed) return;
          try {
            session.renderFrame();
          } catch {
            /* gpu not ready */
          }
          raf = requestAnimationFrame(tick);
        };
        raf = requestAnimationFrame(tick);
        setSessionEpoch((epoch) => epoch + 1);
      };

      resizeObserver =
        typeof ResizeObserver === "undefined"
          ? null
          : new ResizeObserver(() => {
              applySize();
            });
      resizeObserver?.observe(container);
      void boot();
    });

    return () => {
      disposed = true;
      resizeObserver?.disconnect();
      if (raf) cancelAnimationFrame(raf);
      unregisterBoard2dPeer(node.controllerId, node.surfaceId);
      sessionRef.current?.free();
      sessionRef.current = null;
    };
  }, [node.controllerId, node.surfaceId, readContainerSize]);
  //#endregion SessionLifecycle

  //#region SceneSync
  useEffect(() => {
    if (!scene) return;
    const session = sessionRef.current;
    if (!session) return;
    if (session.defersDescriptorSyncFromJs?.() || cameraInteractionActiveRef.current || puzzle2dPeerOwnsGesture(node.controllerId, node.surfaceId)) {
      pendingFixtureSceneRef.current = scene;
      return;
    }
    applyToSession(session, (s) => applyFixtureToSession(s, scene));
    if (!bootSyncedRef.current) {
      bootSyncedRef.current = true;
      try {
        session.drainEventsJson();
      } catch {
        /* session not ready */
      }
    }
  }, [sessionEpoch, scene?.fixtureJson, node.controllerId, node.surfaceId]);

  useEffect(() => {
    if (!scene) return;
    applyToSession(sessionRef.current, (session) => session.setKindCatalogsJson(scene.glyphCatalogsJson));
  }, [sessionEpoch, scene?.glyphCatalogsJson]);

  useEffect(() => {
    if (!scene) return;
    applyToSession(sessionRef.current, (session) => session.setHandleLinkCompatJson?.(scene.placementCompatibilityJson));
  }, [sessionEpoch, scene?.placementCompatibilityJson]);

  useEffect(() => {
    if (!scene) return;
    const session = sessionRef.current;
    if (!session) return;
    if (puzzle2dPeerOwnsGesture(node.controllerId, node.surfaceId)) {
      pendingSelectionJsonRef.current = scene.selectionJson;
      return;
    }
    applyToSession(session, (s) => {
      if (s.setSelectionIdsJsonSilent) s.setSelectionIdsJsonSilent(scene.selectionJson);
      else s.setSelectionIdsJson(scene.selectionJson);
    });
  }, [sessionEpoch, scene?.selectionJson, node.controllerId, node.surfaceId]);

  useEffect(() => {
    if (!scene) return;
    const session = sessionRef.current;
    if (!session || session.defersDescriptorSyncFromJs?.() || cameraInteractionActiveRef.current) return;
    applyToSession(session, (s) => {
      const camera = parseBoardCamera(scene.cameraJson);
      if (!camera) return;
      if (s.setCameraSilent) s.setCameraSilent(camera.x, camera.y, camera.zoom);
      else s.setCamera(camera.x, camera.y, camera.zoom);
    });
  }, [sessionEpoch, scene?.cameraJson]);

  useEffect(() => {
    if (!scene) return;
    applyToSession(sessionRef.current, (session) => session.setHoveredIdSilent?.(scene.hoveredId ?? null));
  }, [sessionEpoch, scene?.hoveredId]);

  useEffect(() => {
    if (!scene) return;
    applyToSession(sessionRef.current, (session) => session.setActiveUtility?.(scene.activeUtility ?? "select"));
  }, [sessionEpoch, scene?.activeUtility]);

  useEffect(() => {
    if (!scene) return;
    const updateOptions = () => {
      const mode = (globalThis as any).__selectionMode || "default";
      const wasmMode = mode === "default" ? "replace" : mode;
      applyToSession(sessionRef.current, (session) => session.setSelectionOptions?.(scene.selectionMethod, wasmMode, true, true, true));
    };
    updateOptions();
    window.addEventListener("semio:selectionOptionsChanged", updateOptions);
    return () => {
      window.removeEventListener("semio:selectionOptionsChanged", updateOptions);
    };
  }, [sessionEpoch, scene?.selectionMethod]);

  useEffect(() => {
    if (!scene) return;
    applyToSession(sessionRef.current, (session) => session.setGridSnapEnabled?.(scene.gridSnapEnabled));
  }, [sessionEpoch, scene?.gridSnapEnabled]);

  useEffect(() => {
    if (!scene) return;
    applyToSession(sessionRef.current, (session) => session.setGridFactor?.(scene.gridFactor));
  }, [sessionEpoch, scene?.gridFactor]);

  useEffect(() => {
    if (!scene || scene.suggestionOffset <= 0) return;
    applyToSession(sessionRef.current, (session) => session.setSuggestionOffset?.(scene.suggestionOffset));
  }, [sessionEpoch, scene?.suggestionOffset]);

  useEffect(() => {
    if (!scene) return;
    applyToSession(sessionRef.current, (session) => session.setBrushKindWeights?.(scene.brushWeightsJson));
  }, [sessionEpoch, scene?.brushWeightsJson]);

  useEffect(() => {
    if (!scene) return;
    applyToSession(sessionRef.current, (session) => {
      if (scene.lodMode === "automatic") {
        session.setAutomaticLod?.(true);
      } else {
        session.setAutomaticLod?.(false);
        session.setForcedDrawLodLabel?.(scene.lodMode);
      }
    });
  }, [sessionEpoch, scene?.lodMode]);
  //#endregion SceneSync

  useCanvasAppearanceSync(() => {
    syncSessionCanvasTheme(sessionRef.current);
    try {
      sessionRef.current?.renderFrame();
    } catch {
      /* gpu not ready */
    }
  });

  //#region Pointer
  useEffect(() => {
    const canvas = canvasRef.current;
    const container = containerRef.current;
    if (!canvas || !container || !scene?.interactive) return undefined;

    const clientToLocal = (clientX: number, clientY: number): { x: number; y: number } => {
      const rect = canvas.getBoundingClientRect();
      return { x: clientX - rect.left, y: clientY - rect.top };
    };

    const onPointerDown = (event: PointerEvent): void => {
      event.stopPropagation();
      const session = sessionRef.current;
      if (!session) return;
      const point = clientToLocal(event.clientX, event.clientY);
      if (event.button === 0 || event.button === 1) {
        canvas.setPointerCapture?.(event.pointerId);
      }
      beginPuzzle2dPeerGesture(node.controllerId, node.surfaceId);
      session.pointerDownScreen(point.x, point.y, event.button, event.shiftKey, event.metaKey || event.ctrlKey);
      scheduleRender();
    };

    const onPointerMove = (event: PointerEvent): void => {
      const session = sessionRef.current;
      if (!session) return;
      const point = clientToLocal(event.clientX, event.clientY);
      session.pointerMoveScreen(point.x, point.y, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey);
      scheduleRender();
      drainAndMaybeFlush();
    };

    const onPointerUp = (event: PointerEvent): void => {
      const session = sessionRef.current;
      if (!session) return;
      const point = clientToLocal(event.clientX, event.clientY);
      session.pointerUpScreen(point.x, point.y, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey);
      if (canvas.hasPointerCapture?.(event.pointerId)) canvas.releasePointerCapture(event.pointerId);
      endPuzzle2dPeerGesture(node.controllerId, node.surfaceId);
      const flushed = settleGestureEnd(session);
      scheduleRender();
      dispatchBufferedEvents();
      notifyPuzzle2dPeersGestureEnded(node.controllerId, node.surfaceId, flushed);
    };

    const onPointerEnter = (): void => {
      hoverActiveRef.current = true;
    };

    const onPointerLeave = (event: PointerEvent): void => {
      hoverActiveRef.current = false;
      const session = sessionRef.current;
      if (!session) return;
      session.pointerLeaveScreen?.(event.altKey);
      endPuzzle2dPeerGesture(node.controllerId, node.surfaceId);
      const flushed = settleGestureEnd(session);
      scheduleRender();
      dispatchBufferedEvents();
      notifyPuzzle2dPeersGestureEnded(node.controllerId, node.surfaceId, flushed);
    };

    /** @emoji 🐁 Wheel-zoom stays instant locally (WASM renders every tick via `scheduleRender`); only the React-visible camera echo and event flush are deferred until the gesture settles via `beginCameraInteraction`'s timeout. */
    const onWheel = (event: WheelEvent): void => {
      event.preventDefault();
      event.stopPropagation();
      const session = sessionRef.current;
      if (!session) return;
      beginCameraInteraction();
      const point = clientToLocal(event.clientX, event.clientY);
      const delta = event.deltaY * (event.deltaMode === WheelEvent.DOM_DELTA_LINE ? 16 : event.deltaMode === WheelEvent.DOM_DELTA_PAGE ? 400 : 1);
      session.wheelScreen(point.x, point.y, delta);
      scheduleRender();
      const cameraArgs = board2dCameraActionArgs(session.cameraJson());
      if (cameraArgs) pendingCameraDispatchRef.current = cameraArgs;
      drainIntoBuffer();
    };

    canvas.addEventListener("pointerdown", onPointerDown);
    canvas.addEventListener("pointerenter", onPointerEnter);
    canvas.addEventListener("pointerleave", onPointerLeave);
    window.addEventListener("pointermove", onPointerMove);
    window.addEventListener("pointerup", onPointerUp);
    container.addEventListener("wheel", onWheel, { passive: false });
    return () => {
      canvas.removeEventListener("pointerdown", onPointerDown);
      canvas.removeEventListener("pointerenter", onPointerEnter);
      canvas.removeEventListener("pointerleave", onPointerLeave);
      window.removeEventListener("pointermove", onPointerMove);
      window.removeEventListener("pointerup", onPointerUp);
      container.removeEventListener("wheel", onWheel);
    };
  }, [beginCameraInteraction, dispatchBufferedEvents, drainAndMaybeFlush, drainIntoBuffer, node.controllerId, node.surfaceId, scheduleRender, scene?.interactive, settleGestureEnd]);
  //#endregion Pointer

  //#region Keyboard
  useEffect(() => {
    if (!scene?.interactive) return undefined;
    const isEditableTarget = (target: EventTarget | null): boolean => {
      if (!(target instanceof HTMLElement)) return false;
      return target.tagName === "INPUT" || target.tagName === "TEXTAREA" || target.isContentEditable;
    };
    const onKeyDown = (event: KeyboardEvent): void => {
      if (!hoverActiveRef.current || isEditableTarget(event.target)) return;
      const session = sessionRef.current;
      if (!session) return;
      if (event.key === "Escape") {
        if (session.cancelAreaSelect?.()) {
          event.preventDefault();
          endPuzzle2dPeerGesture(node.controllerId, node.surfaceId);
          const flushed = settleGestureEnd(session);
          try {
            session.renderFrame();
          } catch {
            /* gpu not ready */
          }
          dispatchBufferedEvents();
          notifyPuzzle2dPeersGestureEnded(node.controllerId, node.surfaceId, flushed);
        }
        return;
      }
      if (event.key === "Tab" && scene.activeUtility === "brush") {
        event.preventDefault();
        session.brushCycleCandidate?.(!event.shiftKey);
        try {
          session.renderFrame();
        } catch {
          /* gpu not ready */
        }
        flushBoardEvents();
        return;
      }
      if ((event.metaKey || event.ctrlKey) && event.key.toLowerCase() === "a") {
        event.preventDefault();
        dispatch("selectAll");
        return;
      }
      if (event.key === "Delete" || event.key === "Backspace") {
        if (parseSelectionIds(scene.selectionJson).length === 0) return;
        event.preventDefault();
        session.deleteSelection?.();
        try {
          session.renderFrame();
        } catch {
          /* gpu not ready */
        }
        flushBoardEvents();
      }
    };
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [dispatch, dispatchBufferedEvents, flushBoardEvents, node.controllerId, node.surfaceId, scene?.activeUtility, scene?.interactive, scene?.selectionJson, settleGestureEnd]);
  //#endregion Keyboard

  //#region ContextMenu
  const onContextMenu = useCallback(
    (event: MouseEvent<HTMLDivElement>): void => {
      if (!scene?.interactive) return;
      const session = sessionRef.current;
      if (!session?.pickTargetsAtScreenJson) return;
      event.preventDefault();
      const rect = event.currentTarget.getBoundingClientRect();
      const sx = event.clientX - rect.left;
      const sy = event.clientY - rect.top;
      let targets: CanvasPickTarget[] = [];
      try {
        targets = JSON.parse(session.pickTargetsAtScreenJson(sx, sy)) as CanvasPickTarget[];
      } catch {
        targets = [];
      }
      const best = pickMostSpecificCanvasTarget(targets);
      let selectionIds = parseSelectionIds(scene.selectionJson);
      if (best && !selectionIds.includes(best.id)) {
        selectionIds = [best.id];
        if (session.setSelectionIdsJsonSilent) session.setSelectionIdsJsonSilent(JSON.stringify(selectionIds));
        try {
          session.renderFrame();
        } catch {
          /* gpu not ready */
        }
        dispatch("setSelection", { ids: selectionIds });
      }
      const items = buildPuzzle2dSelectionMenuItems(scene.fixtureJson, JSON.stringify(selectionIds));
      setContextMenu({ x: event.clientX, y: event.clientY, items });
    },
    [dispatch, scene?.fixtureJson, scene?.interactive, scene?.selectionJson],
  );
  //#endregion ContextMenu

  //#region FixtureDropHandlers
  const onDragOver = useCallback(
    (event: DragEvent<HTMLDivElement>): void => {
      if (!scene?.interactive || !event.dataTransfer.types.includes(CATALOGUE_DRAG_MIME)) return;
      const session = sessionRef.current;
      if (!session?.setFixtureDropPreviewJson) return;
      const payload = parsePuzzle2dCatalogueDragPayload(getActiveCatalogueDragPayload());
      if (!payload) return;
      event.preventDefault();
      const rect = event.currentTarget.getBoundingClientRect();
      applyToSession(session, (s) => s.setFixtureDropPreviewJson?.(puzzle2dFixtureDropPreviewJson(payload, event.clientX - rect.left, event.clientY - rect.top)));
    },
    [scene?.interactive],
  );

  const onDragLeave = useCallback((): void => {
    applyToSession(sessionRef.current, (session) => session.clearFixtureDropPreview?.());
  }, []);

  const onDrop = useCallback(
    (event: DragEvent<HTMLDivElement>): void => {
      if (!scene?.interactive) return;
      const encoded = event.dataTransfer.getData(CATALOGUE_DRAG_MIME) || getActiveCatalogueDragPayload();
      const payload = parsePuzzle2dCatalogueDragPayload(encoded);
      const session = sessionRef.current;
      applyToSession(session, (s) => s.clearFixtureDropPreview?.());
      if (!payload || !session) return;
      event.preventDefault();
      const rect = event.currentTarget.getBoundingClientRect();
      const world = puzzle2dScreenToWorld(session.cameraJson(), readContainerSize(), { x: event.clientX - rect.left, y: event.clientY - rect.top });
      dispatch("addNode", {
        kind: payload.kindId,
        x: world?.x,
        y: world?.y,
        shape: payload.shape,
        radius: payload.radius,
        width: payload.width,
        height: payload.height,
        iconKind: payload.iconKind,
      });
    },
    [dispatch, readContainerSize, scene?.interactive],
  );
  //#endregion FixtureDropHandlers

  if (!scene) return <div className="semio-board-2d-empty text-muted-foreground p-2 text-xs">{emptySceneLabel}</div>;

  return (
    <div
      ref={containerRef}
      className="semio-board-2d-host absolute inset-0 box-border min-h-0 min-w-0 overflow-hidden select-none"
      data-surface-id={node.surfaceId}
      style={{ touchAction: "none" }}
      onContextMenu={onContextMenu}
      onDragOver={onDragOver}
      onDragLeave={onDragLeave}
      onDrop={onDrop}
    >
      <canvas ref={canvasRef} className="absolute inset-0 block size-full touch-none outline-none focus:outline-none" />
      <ContextMenuController
        open={contextMenu != null}
        position={contextMenu ?? { x: 0, y: 0 }}
        items={(contextMenu?.items ?? []).map((item) => ({
          id: item.id,
          label: item.label,
          disabled: item.disabled,
          destructive: item.destructive,
          onSelect: () => dispatch(item.action, item.args),
        }))}
        onOpenChange={(open) => {
          if (!open) setContextMenu(null);
        }}
      />
    </div>
  );
}
//#endregion Board2dHost
//#endregion 🔖Board2dHost

//#region 🔖IconRenderHost
//#region IconRenderHost
/** @emoji 🖼️ Renders an icon-render scene: offscreen GLB shot preview inside a shot frame, see https://threejs.org/docs/#examples/en/renderers/SVGRenderer. */
export function IconRenderHost({ node }: ComponentSceneHostProps) {
  const scene = node.iconRender;
  const requestJson = scene?.requestJson;
  const request = useMemo<IconRenderRequest | null>(() => {
    if (!requestJson) return null;
    try {
      return JSON.parse(requestJson) as IconRenderRequest;
    } catch {
      return null;
    }
  }, [requestJson]);
  const [preview, setPreview] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const emptySceneLabel = useLabel("ui.host.emptyScene");
  useEffect(() => {
    setPreview(null);
    setError(null);
    if (!request) return;
    let cancelled = false;
    void iconRenderPort
      .render(request)
      .then((result) => {
        if (!cancelled) setPreview(result.dataUrl);
      })
      .catch((renderError: unknown) => {
        if (!cancelled) setError(renderError instanceof Error ? renderError.message : String(renderError));
      });
    return () => {
      cancelled = true;
    };
  }, [request]);
  if (!scene || !request) {
    return <div className="flex h-full items-center justify-center text-sm opacity-60">{emptySceneLabel}</div>;
  }
  const content = error ? (
    <div className="flex h-full items-center justify-center p-4 text-sm text-destructive">{error}</div>
  ) : preview ? (
    <img alt={scene.footer ?? "Icon shot"} className="block h-full w-full" src={preview} />
  ) : (
    <div className="flex h-full items-center justify-center text-sm opacity-60">Rendering…</div>
  );
  return (
    <div className="semio-icon-render-host absolute inset-0 flex flex-col" data-surface-id={node.surfaceId}>
      <div className="relative min-h-0 flex-1">
        <IconShotFrame background={request.background} height={request.height} shape={request.shape ?? "rectangle"} width={request.width}>
          {content}
        </IconShotFrame>
      </div>
      {scene.footer ? <div className="shrink-0 px-3 pb-2 text-center text-xs opacity-60">{scene.footer}</div> : null}
    </div>
  );
}
//#endregion IconRenderHost
//#endregion 🔖IconRenderHost

//#region 🔖InkCanvasHost
//#region Types
export type Vec2 = readonly [number, number];

export type InkCamera = { readonly x: number; readonly y: number; readonly zoom: number };

export interface InkTextRun {
  readonly text: string;
  readonly bold?: boolean;
  readonly italic?: boolean;
  readonly underline?: boolean;
  readonly link?: string;
}

export interface InkTextParagraph {
  readonly runs: readonly InkTextRun[];
}

export interface InkItemBase {
  readonly id: string;
  readonly name: string;
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
  readonly rotation?: number;
  readonly visible: boolean;
  readonly locked: boolean;
}

export interface InkTextItem extends InkItemBase {
  readonly kind: "text";
  readonly paragraphs: readonly InkTextParagraph[];
  readonly fontSize: number;
  readonly fontWeight: "normal" | "bold";
  readonly align: "left" | "center" | "right";
}

export interface InkImageAsset {
  readonly mime: string;
  readonly data: string;
  readonly width?: number;
  readonly height?: number;
}

export interface InkImageItem extends InkItemBase {
  readonly kind: "image";
  readonly imageKey: string;
}

export interface InkTableCell {
  readonly content: string;
}

export interface InkTableItem extends InkItemBase {
  readonly kind: "table";
  readonly columns: readonly string[];
  readonly rows: readonly (readonly InkTableCell[])[];
}

export interface InkMathItem extends InkItemBase {
  readonly kind: "math";
  readonly tex: string;
  readonly displayMode: boolean;
}

export interface InkStrokeItem extends InkItemBase {
  readonly kind: "stroke";
  readonly points: readonly Vec2[];
  readonly strokeWidth: number;
  readonly color: readonly [number, number, number, number];
}

export interface InkGroupItem extends InkItemBase {
  readonly kind: "group";
  readonly children: readonly InkItem[];
}

export type InkItem = InkTextItem | InkImageItem | InkTableItem | InkMathItem | InkStrokeItem | InkGroupItem;
export type InkItemKind = InkItem["kind"];

export interface InkDocument {
  readonly schema: "ink.document";
  readonly id: string;
  readonly title?: string;
  readonly camera: InkCamera;
  readonly blocks: readonly InkItem[];
  readonly assets?: Readonly<Record<string, InkImageAsset>>;
  readonly activeUtility?: string;
  readonly gridVisible?: boolean;
  readonly gridSpacing?: number;
  readonly gridSubdivisions?: number;
  readonly gridOpacity?: number;
  readonly snapEnabled?: boolean;
  readonly snapGridSpacing?: number;
  readonly pencilWidth?: number;
  readonly eraserRadius?: number;
}

export type InkResizeHandle = "nw" | "n" | "ne" | "e" | "se" | "s" | "sw" | "w";

export interface InkBounds {
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
}

export type InkCanvasEvent =
  | { readonly op: "addBlock"; readonly block: InkItem; readonly parentId?: string | null; readonly index?: number | null }
  | { readonly op: "updateBlock"; readonly blockId: string; readonly block: InkItem }
  | { readonly op: "removeBlock"; readonly blockId: string }
  | { readonly op: "putAsset"; readonly key: string; readonly asset: InkImageAsset }
  | { readonly op: "setCamera"; readonly camera: InkCamera };

type InkGesturePhase = "begin" | "live" | "commit" | "atomic";

function parseInkScene(documentJson: string | undefined): InkDocument | null {
  if (!documentJson) return null;
  try {
    const parsed = JSON.parse(documentJson) as Partial<InkDocument>;
    if (parsed.schema !== "ink.document" || !Array.isArray(parsed.blocks)) return null;
    return parsed as InkDocument;
  } catch {
    return null;
  }
}

function parseSelectionIds(json: string | undefined): readonly string[] {
  if (!json) return [];
  try {
    const parsed = JSON.parse(json) as unknown;
    return Array.isArray(parsed) ? parsed.filter((id): id is string => typeof id === "string") : [];
  } catch {
    return [];
  }
}
//#endregion Types

//#region GeometryHelpers
let inkHostIdCounter = 0;

/** @emoji 🆔 Host-generated ids only need to be unique client-side (Rust re-derives its own on the next round-trip). */
export function createInkHostId(prefix: string): string {
  inkHostIdCounter += 1;
  return `${prefix}-host-${inkHostIdCounter}`;
}

export function inkPositiveMod(value: number, modulus: number): number {
  if (modulus <= 0) return 0;
  return ((value % modulus) + modulus) % modulus;
}

export function inkSnapWorldCoordinate(value: number, spacing: number): number {
  if (spacing <= 0) return value;
  return Math.round(value / spacing) * spacing;
}

export function inkSnapWorldPoint(x: number, y: number, spacing: number): Vec2 {
  return [inkSnapWorldCoordinate(x, spacing), inkSnapWorldCoordinate(y, spacing)];
}

function inkMaybeSnapWorldPoint(doc: InkDocument, x: number, y: number): Vec2 {
  if (!doc.snapEnabled) return [x, y];
  return inkSnapWorldPoint(x, y, doc.snapGridSpacing ?? 8);
}

export function screenToWorld(camera: InkCamera, screenX: number, screenY: number): Vec2 {
  return [(screenX - camera.x) / camera.zoom, (screenY - camera.y) / camera.zoom];
}

export function worldToScreen(camera: InkCamera, worldX: number, worldY: number): { readonly x: number; readonly y: number } {
  return { x: worldX * camera.zoom + camera.x, y: worldY * camera.zoom + camera.y };
}

export function inkItemBounds(block: InkItem): InkBounds {
  if (block.kind === "stroke" && block.points.length > 0) {
    let minX = block.points[0]![0];
    let minY = block.points[0]![1];
    let maxX = minX;
    let maxY = minY;
    for (const point of block.points) {
      minX = Math.min(minX, point[0]);
      minY = Math.min(minY, point[1]);
      maxX = Math.max(maxX, point[0]);
      maxY = Math.max(maxY, point[1]);
    }
    return { x: block.x + minX, y: block.y + minY, width: Math.max(1, maxX - minX), height: Math.max(1, maxY - minY) };
  }
  return { x: block.x, y: block.y, width: block.width, height: block.height };
}

export function flattenInkItems(blocks: readonly InkItem[]): InkItem[] {
  const out: InkItem[] = [];
  for (const block of blocks) {
    out.push(block);
    if (block.kind === "group") out.push(...flattenInkItems(block.children));
  }
  return out;
}

export function findInkItem(doc: InkDocument, blockId: string): InkItem | null {
  function visit(node: InkItem): InkItem | null {
    if (node.id === blockId) return node;
    if (node.kind === "group") {
      for (const child of node.children) {
        const found = visit(child);
        if (found) return found;
      }
    }
    return null;
  }
  for (const block of doc.blocks) {
    const found = visit(block);
    if (found) return found;
  }
  return null;
}

export function inkSelectionBounds(blocks: readonly InkItem[], ids: readonly string[]): InkBounds | null {
  const idSet = new Set(ids);
  const selected = flattenInkItems(blocks).filter((block) => idSet.has(block.id));
  if (!selected.length) return null;
  let minX = Infinity;
  let minY = Infinity;
  let maxX = -Infinity;
  let maxY = -Infinity;
  for (const block of selected) {
    const bounds = inkItemBounds(block);
    minX = Math.min(minX, bounds.x);
    minY = Math.min(minY, bounds.y);
    maxX = Math.max(maxX, bounds.x + bounds.width);
    maxY = Math.max(maxY, bounds.y + bounds.height);
  }
  return { x: minX, y: minY, width: Math.max(1, maxX - minX), height: Math.max(1, maxY - minY) };
}

function scaleValue(value: number, fromMin: number, fromSize: number, toMin: number, toSize: number): number {
  if (fromSize <= 0) return toMin;
  return toMin + ((value - fromMin) / fromSize) * toSize;
}

export function inkScaleItemWithinGroup(block: InkItem, fromBounds: InkBounds, toBounds: InkBounds): InkItem {
  const bounds = inkItemBounds(block);
  const nextX = scaleValue(bounds.x, fromBounds.x, fromBounds.width, toBounds.x, toBounds.width);
  const nextY = scaleValue(bounds.y, fromBounds.y, fromBounds.height, toBounds.y, toBounds.height);
  const nextWidth = Math.max(8, scaleValue(bounds.x + bounds.width, fromBounds.x, fromBounds.width, toBounds.x, toBounds.width) - nextX);
  const nextHeight = Math.max(8, scaleValue(bounds.y + bounds.height, fromBounds.y, fromBounds.height, toBounds.y, toBounds.height) - nextY);
  if (block.kind === "stroke") {
    const scaleX = fromBounds.width > 0 ? toBounds.width / fromBounds.width : 1;
    const scaleY = fromBounds.height > 0 ? toBounds.height / fromBounds.height : 1;
    const points = block.points.map(([px, py]) => [px * scaleX, py * scaleY] as Vec2);
    return { ...block, x: nextX, y: nextY, width: nextWidth, height: nextHeight, points };
  }
  if (block.kind === "group") {
    return { ...block, x: nextX, y: nextY, width: nextWidth, height: nextHeight, children: block.children.map((child) => inkScaleItemWithinGroup(child, fromBounds, toBounds)) };
  }
  return { ...block, x: nextX, y: nextY, width: nextWidth, height: nextHeight };
}

export function inkResizeBounds(fromBounds: InkBounds, handle: InkResizeHandle, dx: number, dy: number, minSize = 8): InkBounds {
  let { x, y, width, height } = fromBounds;
  if (handle.includes("e")) width = Math.max(minSize, width + dx);
  if (handle.includes("w")) {
    const nextWidth = Math.max(minSize, width - dx);
    x += width - nextWidth;
    width = nextWidth;
  }
  if (handle.includes("s")) height = Math.max(minSize, height + dy);
  if (handle.includes("n")) {
    const nextHeight = Math.max(minSize, height - dy);
    y += height - nextHeight;
    height = nextHeight;
  }
  return { x, y, width, height };
}

export function inkItemsAtPoint(blocks: readonly InkItem[], x: number, y: number): InkItem[] {
  const hits: InkItem[] = [];
  for (const block of [...flattenInkItems(blocks)].reverse()) {
    const bounds = inkItemBounds(block);
    if (x >= bounds.x && x <= bounds.x + bounds.width && y >= bounds.y && y <= bounds.y + bounds.height) hits.push(block);
  }
  return hits;
}

export function inkItemsIntersectingRect(blocks: readonly InkItem[], rect: InkBounds): string[] {
  const hits: string[] = [];
  for (const block of flattenInkItems(blocks)) {
    const bounds = inkItemBounds(block);
    const intersects = bounds.x < rect.x + rect.width && bounds.x + bounds.width > rect.x && bounds.y < rect.y + rect.height && bounds.y + bounds.height > rect.y;
    if (intersects) hits.push(block.id);
  }
  return hits;
}

export function inkTableCellAtPoint(block: InkTableItem, localX: number, localY: number): { readonly row: number; readonly col: number } | null {
  const rowCount = block.rows.length + 1;
  const colCount = block.columns.length;
  if (rowCount <= 0 || colCount <= 0) return null;
  const rowHeight = block.height / rowCount;
  const colWidth = block.width / colCount;
  const row = Math.floor(localY / rowHeight) - 1;
  const col = Math.floor(localX / colWidth);
  if (row < 0 || row >= block.rows.length || col < 0 || col >= colCount) return null;
  return { row, col };
}

function pointToSegmentDistance(px: number, py: number, x1: number, y1: number, x2: number, y2: number): number {
  const dx = x2 - x1;
  const dy = y2 - y1;
  if (dx === 0 && dy === 0) return Math.hypot(px - x1, py - y1);
  const t = Math.max(0, Math.min(1, ((px - x1) * dx + (py - y1) * dy) / (dx * dx + dy * dy)));
  return Math.hypot(px - (x1 + t * dx), py - (y1 + t * dy));
}

function inkWorldPoints(block: InkStrokeItem): Vec2[] {
  return block.points.map(([px, py]) => [block.x + px, block.y + py] as Vec2);
}

function inkHitsPoint(block: InkStrokeItem, x: number, y: number, threshold: number): boolean {
  const points = inkWorldPoints(block);
  if (points.length < 2) {
    if (!points[0]) return false;
    return Math.hypot(x - points[0][0], y - points[0][1]) <= threshold;
  }
  for (let index = 1; index < points.length; index += 1) {
    const prev = points[index - 1]!;
    const next = points[index]!;
    if (pointToSegmentDistance(x, y, prev[0], prev[1], next[0], next[1]) <= threshold + block.strokeWidth / 2) return true;
  }
  return false;
}

/** @emoji 🧹 Whole-stroke eraser: returns removeBlock events for every ink stroke under the point. */
export function eraseInkStrokeEventsAtPoint(doc: InkDocument, x: number, y: number, threshold = 8): readonly InkCanvasEvent[] {
  const hits = flattenInkItems(doc.blocks).filter((block): block is InkStrokeItem => block.kind === "stroke" && inkHitsPoint(block, x, y, threshold));
  return hits.map((block) => ({ op: "removeBlock", blockId: block.id }));
}

/** @emoji ✂️ Splits an ink stroke into surviving point-runs after removing points within `radius` of (x, y). */
export function eraseInkStrokePointsInItem(block: InkStrokeItem, x: number, y: number, radius: number): InkStrokeItem[] {
  const keptIndices: number[] = [];
  for (let index = 0; index < block.points.length; index += 1) {
    const point = block.points[index]!;
    if (Math.hypot(block.x + point[0] - x, block.y + point[1] - y) > radius) keptIndices.push(index);
  }
  if (keptIndices.length === block.points.length) return [block];
  if (!keptIndices.length) return [];
  const runs: Vec2[][] = [];
  let current: Vec2[] = [block.points[keptIndices[0]!]!];
  for (let index = 1; index < keptIndices.length; index += 1) {
    if (keptIndices[index]! - keptIndices[index - 1]! > 1) {
      if (current.length >= 2) runs.push(current);
      current = [block.points[keptIndices[index]!]!];
    } else {
      current.push(block.points[keptIndices[index]!]!);
    }
  }
  if (current.length >= 2) runs.push(current);
  return runs.map((points, index) => ({ ...block, id: index === 0 ? block.id : createInkHostId("stroke"), name: index === 0 ? block.name : `${block.name} fragment`, points }));
}

/** @emoji ✂️ Point-eraser events: removeBlock for the original stroke, addBlock for each surviving fragment (skipped if untouched). */
export function eraseInkStrokePointEventsNearPoint(doc: InkDocument, x: number, y: number, radius: number): readonly InkCanvasEvent[] {
  const events: InkCanvasEvent[] = [];
  const inkBlocks = flattenInkItems(doc.blocks).filter((block): block is InkStrokeItem => block.kind === "stroke");
  for (const block of inkBlocks) {
    const fragments = eraseInkStrokePointsInItem(block, x, y, radius);
    if (fragments.length === 1 && fragments[0] === block) continue;
    events.push({ op: "removeBlock", blockId: block.id });
    for (const fragment of fragments) events.push({ op: "addBlock", block: fragment });
  }
  return events;
}

export function inkTextParagraphsFromPlainText(text: string): readonly InkTextParagraph[] {
  return text.split(/\n/).map((line) => ({ runs: [{ text: line }] }));
}

export function inkTextPlainText(paragraphs: readonly InkTextParagraph[]): string {
  return paragraphs.map((paragraph) => paragraph.runs.map((run) => run.text).join("")).join("\n");
}

export function inkParagraphsToHtml(paragraphs: readonly InkTextParagraph[]): string {
  return paragraphs
    .map((paragraph) => {
      const inner = paragraph.runs
        .map((run) => {
          let text = run.text.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
          if (run.link) text = `<a href="${run.link}">${text}</a>`;
          if (run.underline) text = `<u>${text}</u>`;
          if (run.italic) text = `<em>${text}</em>`;
          if (run.bold) text = `<strong>${text}</strong>`;
          return text;
        })
        .join("");
      return `<div>${inner || "<br>"}</div>`;
    })
    .join("");
}

export function inkHtmlToParagraphs(root: HTMLElement): readonly InkTextParagraph[] {
  const paragraphs: InkTextParagraph[] = [];
  const children = root.childNodes.length ? [...root.childNodes] : [root];
  for (const child of children) {
    if (child.nodeType === Node.TEXT_NODE) {
      const text = child.textContent ?? "";
      if (text) paragraphs.push({ runs: [{ text }] });
      continue;
    }
    if (!(child instanceof HTMLElement)) continue;
    const tag = child.tagName.toLowerCase();
    if (tag === "br") {
      paragraphs.push({ runs: [{ text: "" }] });
      continue;
    }
    const runs: InkTextRun[] = [];
    const walk = (node: Node, marks: Partial<InkTextRun>) => {
      if (node.nodeType === Node.TEXT_NODE) {
        const text = node.textContent ?? "";
        if (text) runs.push({ text, ...marks });
        return;
      }
      if (!(node instanceof HTMLElement)) return;
      const nextMarks = { ...marks };
      const nodeTag = node.tagName.toLowerCase();
      if (nodeTag === "strong" || nodeTag === "b") nextMarks.bold = true;
      if (nodeTag === "em" || nodeTag === "i") nextMarks.italic = true;
      if (nodeTag === "u") nextMarks.underline = true;
      if (nodeTag === "a") nextMarks.link = node.getAttribute("href") ?? undefined;
      for (const childNode of node.childNodes) walk(childNode, nextMarks);
    };
    for (const childNode of child.childNodes) walk(childNode, {});
    if (!runs.length) runs.push({ text: "" });
    paragraphs.push({ runs });
  }
  return paragraphs.length ? paragraphs : [{ runs: [{ text: "" }] }];
}

export function inkImageAssetDataUrl(asset: InkImageAsset): string {
  if (asset.data.startsWith("data:")) return asset.data;
  if (asset.mime === "image/svg+xml") return `data:image/svg+xml;utf8,${encodeURIComponent(asset.data)}`;
  return `data:${asset.mime};base64,${asset.data}`;
}

export interface InkClipboardPayload {
  readonly schema: "ink.clipboard";
  readonly blocks: readonly InkItem[];
}

export function inkClipboardPayload(blocks: readonly InkItem[]): string {
  const payload: InkClipboardPayload = { schema: "ink.clipboard", blocks: [...blocks] };
  return JSON.stringify(payload);
}

export function inkItemsFromClipboardPayload(json: string): readonly InkItem[] | null {
  try {
    const parsed = JSON.parse(json) as InkClipboardPayload;
    if (parsed.schema !== "ink.clipboard" || !Array.isArray(parsed.blocks)) return null;
    return parsed.blocks;
  } catch {
    return null;
  }
}

function reidItemTree(block: InkItem, renameTop: boolean): InkItem {
  const id = createInkHostId(block.kind);
  const name = renameTop ? `${block.name} copy` : block.name;
  if (block.kind === "group") return { ...block, id, name, children: block.children.map((child) => reidItemTree(child, false)) };
  return { ...block, id, name };
}

export function cloneInkItemsWithOffset(blocks: readonly InkItem[], dx: number, dy: number): InkItem[] {
  return blocks.map((block) => {
    const clone = reidItemTree(block, false);
    return { ...clone, x: clone.x + dx, y: clone.y + dy };
  });
}

const INK_ITEM_DEFAULT_SIZE: Record<InkItemKind, { readonly width: number; readonly height: number }> = {
  text: { width: 280, height: 120 },
  image: { width: 240, height: 160 },
  table: { width: 320, height: 160 },
  math: { width: 200, height: 80 },
  stroke: { width: 1, height: 1 },
  group: { width: 280, height: 120 },
};

export function createInkItemByKind(kind: InkItemKind, x: number, y: number): InkItem {
  const size = INK_ITEM_DEFAULT_SIZE[kind];
  const base = { id: createInkHostId(kind), x, y, width: size.width, height: size.height, rotation: 0, visible: true, locked: false };
  if (kind === "image") return { ...base, kind, name: hostLabel("ui.host.blockImage"), imageKey: "placeholder" };
  if (kind === "table")
    return {
      ...base,
      kind,
      name: hostLabel("ui.host.blockTable"),
      columns: ["A", "B", "C"],
      rows: [
        [{ content: "" }, { content: "" }, { content: "" }],
        [{ content: "" }, { content: "" }, { content: "" }],
      ],
    };
  if (kind === "math") return { ...base, kind, name: hostLabel("ui.host.blockMath"), tex: "E = mc^2", displayMode: true };
  if (kind === "stroke") return { ...base, kind, name: hostLabel("ui.host.blockInk"), points: [], strokeWidth: 3, color: [0, 0, 0, 1] };
  if (kind === "group") return { ...base, kind, name: hostLabel("ui.host.blockGroup"), children: [] };
  return { ...base, kind: "text", name: hostLabel("ui.host.blockText"), paragraphs: [{ runs: [{ text: "" }] }], fontSize: 18, fontWeight: "normal", align: "left" };
}

/** @emoji 🖊️ Local pure application of the generic ink-apply-events op vocabulary — mirrors the note plugin's event-apply function for optimistic in-gesture rendering. */
export function applyInkCanvasEventLocal(doc: InkDocument, event: InkCanvasEvent): InkDocument {
  switch (event.op) {
    case "addBlock": {
      const blocks = [...doc.blocks];
      if (!event.parentId) {
        blocks.splice(event.index ?? blocks.length, 0, event.block);
        return { ...doc, blocks };
      }
      return { ...doc, blocks: insertIntoParent(doc.blocks, event.parentId, event.index ?? Number.MAX_SAFE_INTEGER, event.block) };
    }
    case "updateBlock":
      return { ...doc, blocks: updateInTree(doc.blocks, event.blockId, event.block) };
    case "removeBlock":
      return { ...doc, blocks: removeFromTree(doc.blocks, event.blockId) };
    case "putAsset":
      return { ...doc, assets: { ...(doc.assets ?? {}), [event.key]: event.asset } };
    case "setCamera":
      return { ...doc, camera: event.camera };
    default:
      return doc;
  }
}

function insertIntoParent(blocks: readonly InkItem[], parentId: string, index: number, block: InkItem): InkItem[] {
  return blocks.map((node) => {
    if (node.kind !== "group") return node;
    if (node.id === parentId) {
      const children = [...node.children];
      children.splice(Math.min(index, children.length), 0, block);
      return { ...node, children };
    }
    return { ...node, children: insertIntoParent(node.children, parentId, index, block) };
  });
}

function updateInTree(blocks: readonly InkItem[], blockId: string, nextBlock: InkItem): InkItem[] {
  return blocks.map((block) => {
    if (block.id === blockId) return nextBlock;
    if (block.kind === "group") return { ...block, children: updateInTree(block.children, blockId, nextBlock) };
    return block;
  });
}

function removeFromTree(blocks: readonly InkItem[], blockId: string): InkItem[] {
  return blocks.filter((block) => block.id !== blockId).map((block) => (block.kind === "group" ? { ...block, children: removeFromTree(block.children, blockId) } : block));
}

function applyEventsLocal(doc: InkDocument, events: readonly InkCanvasEvent[]): InkDocument {
  return events.reduce((acc, event) => applyInkCanvasEventLocal(acc, event), doc);
}
//#endregion GeometryHelpers

//#region MathRenderer
export interface InkMathRenderer {
  render(tex: string, displayMode: boolean): string;
}

let inkMathRenderer: InkMathRenderer = {
  render(tex: string, displayMode: boolean) {
    return `<span class="ink-math-fallback">${displayMode ? `$$${tex}$$` : `$${tex}$`}</span>`;
  },
};

/** @emoji ∑ Sets the active ink math renderer adapter (defaults to a plain-text fallback until KaTeX loads). */
export function setInkMathRenderer(renderer: InkMathRenderer): void {
  inkMathRenderer = renderer;
}

async function ensureKatexMathRenderer(): Promise<void> {
  try {
    const katex = await import("katex");
    await import("katex/dist/katex.min.css");
    setInkMathRenderer({
      render(tex: string, displayMode: boolean) {
        return katex.default.renderToString(tex, { displayMode, throwOnError: false });
      },
    });
  } catch {
    /* fallback renderer stays active */
  }
}

if (typeof window !== "undefined") void ensureKatexMathRenderer();
//#endregion MathRenderer

//#region BlockViews
function inkTextRunStyle(run: InkTextRun): React.CSSProperties {
  return { fontWeight: run.bold ? "bold" : undefined, fontStyle: run.italic ? "italic" : undefined, textDecoration: run.underline ? "underline" : undefined };
}

function InkTextRunView({ run }: { readonly run: InkTextRun }) {
  if (run.link) {
    return (
      <a href={run.link} className="text-primary underline" style={inkTextRunStyle(run)} onPointerDown={(event) => event.stopPropagation()}>
        {run.text}
      </a>
    );
  }
  return <span style={inkTextRunStyle(run)}>{run.text}</span>;
}

function InkTextContentView({ block }: { readonly block: InkTextItem }) {
  return (
    <div className="text-foreground h-full w-full overflow-auto p-2 whitespace-pre-wrap" style={{ fontSize: block.fontSize, fontWeight: block.fontWeight, textAlign: block.align }}>
      {block.paragraphs.map((paragraph, paragraphIndex) => (
        <div key={paragraphIndex}>
          {paragraph.runs.map((run, runIndex) => (
            <InkTextRunView key={runIndex} run={run} />
          ))}
        </div>
      ))}
    </div>
  );
}

function InkItemView({
  block,
  assets,
  selected,
  hovered,
  hidden,
  onPointerDown,
}: {
  readonly block: InkItem;
  readonly assets?: Readonly<Record<string, InkImageAsset>>;
  readonly selected: boolean;
  readonly hovered: boolean;
  readonly hidden: boolean;
  readonly onPointerDown: (event: React.PointerEvent, blockId: string) => void;
}) {
  const groupLabel = useLabel("ui.host.blockGroup");
  if (!block.visible) return null;
  const bounds = inkItemBounds(block);
  const common = {
    className: cn("bg-background/90 absolute overflow-hidden rounded border shadow-sm", selected && "ring-primary ring-2", hovered && !selected && "ring-primary/60 ring-1", block.locked && "opacity-70", hidden && "pointer-events-none opacity-0"),
    style: {
      left: bounds.x,
      top: bounds.y,
      width: Math.max(8, bounds.width),
      height: Math.max(8, bounds.height),
      transform: block.rotation ? `rotate(${block.rotation}deg)` : undefined,
    },
    onPointerDown: (event: React.PointerEvent) => onPointerDown(event, block.id),
  };
  if (block.kind === "text")
    return (
      <div {...common}>
        <InkTextContentView block={block} />
      </div>
    );
  if (block.kind === "math") {
    const html = inkMathRenderer.render(block.tex, block.displayMode);
    return (
      <div {...common}>
        <div className="flex h-full w-full items-center justify-center p-2">
          <div className="ink-math" dangerouslySetInnerHTML={{ __html: html }} />
        </div>
      </div>
    );
  }
  if (block.kind === "table") {
    return (
      <div {...common}>
        <table className="h-full w-full border-collapse text-sm">
          <thead>
            <tr>
              {block.columns.map((column) => (
                <th key={column} className="border-border border px-2 py-1 text-left font-medium">
                  {column}
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            {block.rows.map((row, rowIndex) => (
              <tr key={rowIndex}>
                {row.map((cell, cellIndex) => (
                  <td key={cellIndex} className="border-border border px-2 py-1 align-top">
                    {cell.content}
                  </td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    );
  }
  if (block.kind === "image") {
    const asset = assets?.[block.imageKey];
    const src = asset ? inkImageAssetDataUrl(asset) : null;
    return (
      <div {...common}>
        {src ? <img src={src} alt={block.name} className="h-full w-full object-contain" draggable={false} /> : <div className="bg-muted text-muted-foreground flex h-full w-full items-center justify-center text-xs">{block.imageKey}</div>}
      </div>
    );
  }
  if (block.kind === "stroke") {
    if (block.points.length < 2) return null;
    const path = block.points.map((point, index) => `${index === 0 ? "M" : "L"} ${block.x + point[0]} ${block.y + point[1]}`).join(" ");
    const [r, g, b, a] = block.color;
    return (
      <svg className="pointer-events-none absolute inset-0 overflow-visible" style={{ width: "100%", height: "100%" }}>
        <path d={path} fill="none" stroke={`rgba(${Math.round(r * 255)},${Math.round(g * 255)},${Math.round(b * 255)},${a})`} strokeWidth={block.strokeWidth} strokeLinecap="round" strokeLinejoin="round" />
      </svg>
    );
  }
  if (block.kind === "group") {
    return (
      <div {...common}>
        <div className="text-muted-foreground p-1 text-xs">
          {groupLabel} · {block.children.length} children
        </div>
      </div>
    );
  }
  return null;
}

const INK_RESIZE_HANDLES: readonly InkResizeHandle[] = ["nw", "n", "ne", "e", "se", "s", "sw", "w"];
const INK_RESIZE_CURSOR: Record<InkResizeHandle, string> = { nw: "nwse-resize", n: "ns-resize", ne: "nesw-resize", e: "ew-resize", se: "nwse-resize", s: "ns-resize", sw: "nesw-resize", w: "ew-resize" };

function InkSelectionChrome({ camera, bounds, onResizePointerDown }: { readonly camera: InkCamera; readonly bounds: InkBounds; readonly onResizePointerDown: (handle: InkResizeHandle, event: React.PointerEvent) => void }) {
  const topLeft = worldToScreen(camera, bounds.x, bounds.y);
  const width = bounds.width * camera.zoom;
  const height = bounds.height * camera.zoom;
  return (
    <>
      <div className="border-primary pointer-events-none absolute z-20 border" style={{ left: topLeft.x, top: topLeft.y, width, height }} />
      {INK_RESIZE_HANDLES.map((handle) => {
        const left = handle.includes("w") ? topLeft.x - 4 : handle.includes("e") ? topLeft.x + width - 4 : topLeft.x + width / 2 - 4;
        const top = handle.includes("n") ? topLeft.y - 4 : handle.includes("s") ? topLeft.y + height - 4 : topLeft.y + height / 2 - 4;
        return <div key={handle} className="border-primary bg-background absolute z-30 h-2 w-2 rounded-sm border" style={{ left, top, cursor: INK_RESIZE_CURSOR[handle] }} onPointerDown={(event) => onResizePointerDown(handle, event)} />;
      })}
    </>
  );
}

function InkViewportGrid({ camera, spacing, subdivisions, opacity, color }: { readonly camera: InkCamera; readonly spacing: number; readonly subdivisions: number; readonly opacity: number; readonly color: string }) {
  const majorPx = spacing * camera.zoom;
  const minorPx = majorPx / Math.max(1, subdivisions);
  const offsetX = inkPositiveMod(camera.x, majorPx);
  const offsetY = inkPositiveMod(camera.y, majorPx);
  const patternId = `ink-viewport-grid-${spacing}-${subdivisions}`;
  const minorLines: React.ReactNode[] = [];
  for (let index = 1; index < subdivisions; index += 1) {
    const position = index * minorPx;
    minorLines.push(
      <line key={`v-${index}`} x1={position} y1={0} x2={position} y2={majorPx} stroke={color} strokeWidth={0.5} opacity={opacity * 0.55} />,
      <line key={`h-${index}`} x1={0} y1={position} x2={majorPx} y2={position} stroke={color} strokeWidth={0.5} opacity={opacity * 0.55} />,
    );
  }
  return (
    <svg className="pointer-events-none absolute inset-0 h-full w-full" aria-hidden>
      <defs>
        <pattern id={patternId} width={majorPx} height={majorPx} patternUnits="userSpaceOnUse" x={offsetX} y={offsetY}>
          {minorLines}
          <path d={`M ${majorPx} 0 L 0 0 0 ${majorPx}`} fill="none" stroke={color} strokeWidth={1} opacity={opacity} />
        </pattern>
      </defs>
      <rect width="100%" height="100%" fill={`url(#${patternId})`} />
    </svg>
  );
}
//#endregion BlockViews

//#region Overlays
function InkTextEditorOverlay({ block, screenBounds, onCommit, onCancel }: { readonly block: InkTextItem; readonly screenBounds: InkBounds; readonly onCommit: (paragraphs: readonly InkTextParagraph[]) => void; readonly onCancel: () => void }) {
  const editorRef = useRef<HTMLDivElement | null>(null);
  const applyCommand = (command: string, value?: string) => {
    editorRef.current?.focus();
    document.execCommand(command, false, value);
  };
  useEffect(() => {
    const editor = editorRef.current;
    if (!editor) return;
    editor.focus();
    const selection = window.getSelection();
    const range = document.createRange();
    range.selectNodeContents(editor);
    selection?.removeAllRanges();
    selection?.addRange(range);
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        event.preventDefault();
        onCancel();
      }
    };
    editor.addEventListener("keydown", onKeyDown);
    return () => editor.removeEventListener("keydown", onKeyDown);
  }, [onCancel]);
  return (
    <div className="absolute z-30" style={{ left: screenBounds.x, top: screenBounds.y, width: screenBounds.width, height: screenBounds.height }}>
      <div className="bg-background/95 mb-1 flex gap-1 rounded border p-1 shadow-sm">
        <button
          type="button"
          className="hover:bg-muted rounded px-2 py-0.5 text-xs"
          onMouseDown={(event) => {
            event.preventDefault();
            applyCommand("bold");
          }}
        >
          B
        </button>
        <button
          type="button"
          className="hover:bg-muted rounded px-2 py-0.5 text-xs italic"
          onMouseDown={(event) => {
            event.preventDefault();
            applyCommand("italic");
          }}
        >
          I
        </button>
        <button
          type="button"
          className="hover:bg-muted rounded px-2 py-0.5 text-xs underline"
          onMouseDown={(event) => {
            event.preventDefault();
            applyCommand("underline");
          }}
        >
          U
        </button>
        <button
          type="button"
          className="hover:bg-muted rounded px-2 py-0.5 text-xs"
          onMouseDown={(event) => {
            event.preventDefault();
            const url = window.prompt("Link URL");
            if (url) applyCommand("createLink", url);
          }}
        >
          Link
        </button>
      </div>
      <div
        ref={editorRef}
        contentEditable
        suppressContentEditableWarning
        className="text-foreground bg-background h-[calc(100%-2rem)] w-full overflow-auto rounded border p-2 outline-none"
        style={{ fontSize: block.fontSize, fontWeight: block.fontWeight, textAlign: block.align }}
        dangerouslySetInnerHTML={{ __html: inkParagraphsToHtml(block.paragraphs) }}
        onBlur={() => {
          if (!editorRef.current) return;
          onCommit(inkHtmlToParagraphs(editorRef.current));
        }}
      />
    </div>
  );
}

function InkTableCellEditorOverlay({
  block,
  row,
  col,
  screenBounds,
  onCommit,
  onCancel,
}: {
  readonly block: InkTableItem;
  readonly row: number;
  readonly col: number;
  readonly screenBounds: InkBounds;
  readonly onCommit: (content: string, advance?: boolean) => void;
  readonly onCancel: () => void;
}) {
  const inputRef = useRef<HTMLInputElement | null>(null);
  useEffect(() => {
    inputRef.current?.focus();
    inputRef.current?.select();
  }, []);
  return (
    <input
      ref={inputRef}
      className="bg-background ring-primary absolute z-30 rounded border px-2 py-1 text-sm ring-2 outline-none"
      style={{ left: screenBounds.x, top: screenBounds.y, width: screenBounds.width, height: screenBounds.height }}
      defaultValue={block.rows[row]?.[col]?.content ?? ""}
      onKeyDown={(event) => {
        if (event.key === "Escape") {
          event.preventDefault();
          onCancel();
        }
        if (event.key === "Enter" || event.key === "Tab") {
          event.preventDefault();
          onCommit(event.currentTarget.value, true);
        }
      }}
      onBlur={(event) => onCommit(event.currentTarget.value)}
    />
  );
}
//#endregion Overlays

//#region DragState
type InkDragState =
  | { readonly kind: "pan"; readonly startX: number; readonly startY: number; readonly camera: InkCamera }
  | { readonly kind: "move"; readonly origins: Readonly<Record<string, { readonly x: number; readonly y: number }>>; readonly startX: number; readonly startY: number }
  | { readonly kind: "marquee"; readonly start: SelectionMarqueePoint }
  | { readonly kind: "stroke"; readonly blockId: string }
  | { readonly kind: "eraser"; readonly mode: "eraserStroke" | "eraserPoint" }
  | { readonly kind: "resize"; readonly handle: InkResizeHandle; readonly fromBounds: InkBounds; readonly startX: number; readonly startY: number; readonly selectedIds: readonly string[] };

type InkTextEditState = { readonly blockId: string; readonly created?: boolean };
type InkTableEditState = { readonly blockId: string; readonly row: number; readonly col: number };

const INK_MARQUEE_THRESHOLD_PX = 4;
//#endregion DragState

//#region InkCanvasHost
export function InkCanvasHost({ node, onAction }: ComponentSceneHostProps) {
  const scene = node.inkCanvas;
  const rootRef = useRef<HTMLDivElement | null>(null);
  const gestureActiveRef = useRef(false);
  const rafRef = useRef<number | null>(null);
  const pendingLiveEventsRef = useRef<readonly InkCanvasEvent[] | null>(null);
  const [draftDoc, setDraftDoc] = useState<InkDocument | null>(null);
  const [dragState, setDragState] = useState<InkDragState | null>(null);
  const [marqueePoints, setMarqueePoints] = useState<readonly SelectionMarqueePoint[]>([]);
  const [textEdit, setTextEdit] = useState<InkTextEditState | null>(null);
  const [tableEdit, setTableEdit] = useState<InkTableEditState | null>(null);
  const emptySceneLabel = useLabel("ui.host.emptyScene");

  const sceneDoc = useMemo(() => parseInkScene(scene?.documentJson), [scene?.documentJson]);
  const doc = draftDoc ?? sceneDoc;
  const selectedIds = useMemo(() => parseSelectionIds(scene?.selectionJson), [scene?.selectionJson]);
  const selectedSet = useMemo(() => new Set(selectedIds), [selectedIds]);
  const hoveredId = scene?.hoveredId ?? null;
  const isNavigator = scene?.viewMode === "navigator";
  const interactive = scene?.interactive ?? false;

  useEffect(() => {
    if (!gestureActiveRef.current) setDraftDoc(null);
  }, [scene?.documentJson]);

  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      if (!node.controllerId) return;
      onAction({ controllerId: node.controllerId, action, args: { surfaceId: node.surfaceId, ...args } });
    },
    [node.controllerId, node.surfaceId, onAction],
  );

  const flushPendingLive = useCallback(() => {
    if (rafRef.current != null) {
      cancelAnimationFrame(rafRef.current);
      rafRef.current = null;
    }
    pendingLiveEventsRef.current = null;
  }, []);

  const beginGesture = useCallback(
    (events: readonly InkCanvasEvent[], selectIds?: readonly string[]) => {
      gestureActiveRef.current = true;
      setDraftDoc((current) => applyEventsLocal(current ?? sceneDoc ?? { schema: "ink.document", id: "empty", camera: { x: 0, y: 0, zoom: 1 }, blocks: [] }, events));
      dispatch(inkCanvasActions.applyEvents, { eventsJson: JSON.stringify(events), phase: "begin", ...(selectIds ? { selectIds: [...selectIds] } : {}) });
    },
    [dispatch, sceneDoc],
  );

  const liveGesture = useCallback(
    (events: readonly InkCanvasEvent[]) => {
      setDraftDoc((current) => (current ? applyEventsLocal(current, events) : current));
      pendingLiveEventsRef.current = events;
      if (rafRef.current == null) {
        rafRef.current = requestAnimationFrame(() => {
          rafRef.current = null;
          const pending = pendingLiveEventsRef.current;
          pendingLiveEventsRef.current = null;
          if (pending) dispatch(inkCanvasActions.applyEvents, { eventsJson: JSON.stringify(pending), phase: "live" });
        });
      }
    },
    [dispatch],
  );

  const commitGesture = useCallback(
    (events: readonly InkCanvasEvent[], selectIds?: readonly string[]) => {
      flushPendingLive();
      gestureActiveRef.current = false;
      dispatch(inkCanvasActions.applyEvents, { eventsJson: JSON.stringify(events), phase: "commit", ...(selectIds ? { selectIds: [...selectIds] } : {}) });
    },
    [dispatch, flushPendingLive],
  );

  const atomicGesture = useCallback(
    (events: readonly InkCanvasEvent[], selectIds?: readonly string[]) => {
      dispatch(inkCanvasActions.applyEvents, { eventsJson: JSON.stringify(events), phase: "atomic", ...(selectIds ? { selectIds: [...selectIds] } : {}) });
    },
    [dispatch],
  );

  const selectionBounds = useMemo(() => (doc ? inkSelectionBounds(doc.blocks, selectedIds) : null), [doc, selectedIds]);
  const utility = doc?.activeUtility ?? "selectDirect";
  const showResizeHandles = !isNavigator && (utility === "selectDirect" || utility === "selectMarquee") && Boolean(selectionBounds) && selectedIds.length > 0;

  const beginMove = useCallback(
    (event: React.PointerEvent, blockId: string) => {
      if (!rootRef.current || !doc) return;
      const block = findInkItem(doc, blockId);
      if (!block || block.locked) return;
      const rect = rootRef.current.getBoundingClientRect();
      const screenX = event.clientX - rect.left;
      const screenY = event.clientY - rect.top;
      const moveIds = selectedSet.has(blockId) ? selectedIds : [blockId];
      const origins: Record<string, { x: number; y: number }> = {};
      for (const id of moveIds) {
        const entry = findInkItem(doc, id);
        if (entry) origins[id] = { x: entry.x, y: entry.y };
      }
      setDragState({ kind: "move", origins, startX: screenX, startY: screenY });
    },
    [doc, selectedIds, selectedSet],
  );

  const handlePointerDown = useCallback(
    (event: React.PointerEvent<HTMLDivElement>) => {
      if (!rootRef.current || !doc || isNavigator || !interactive) return;
      rootRef.current.focus();
      const camera = doc.camera;
      const rect = rootRef.current.getBoundingClientRect();
      const screenX = event.clientX - rect.left;
      const screenY = event.clientY - rect.top;
      const [worldX, worldY] = screenToWorld(camera, screenX, screenY);
      if (utility === "pan" || event.button === 1 || (utility === "selectDirect" && event.altKey)) {
        setDragState({ kind: "pan", startX: screenX, startY: screenY, camera });
        return;
      }
      if (utility === "eraserStroke" || utility === "eraserPoint") {
        setDragState({ kind: "eraser", mode: utility });
        const events = utility === "eraserStroke" ? eraseInkStrokeEventsAtPoint(doc, worldX, worldY) : eraseInkStrokePointEventsNearPoint(doc, worldX, worldY, doc.eraserRadius ?? 12);
        if (events.length) beginGesture(events);
        return;
      }
      if (utility === "selectMarquee") {
        setDragState({ kind: "marquee", start: { x: screenX, y: screenY } });
        setMarqueePoints([{ x: screenX, y: screenY }]);
        return;
      }
      if (utility === "pencil") {
        const block = createInkItemByKind("stroke", worldX, worldY);
        beginGesture([{ op: "addBlock", block }], [block.id]);
        setDragState({ kind: "stroke", blockId: block.id });
        return;
      }
      if (utility === "text" || utility === "image" || utility === "table" || utility === "math") {
        const [placeX, placeY] = inkMaybeSnapWorldPoint(doc, worldX, worldY);
        const block = createInkItemByKind(utility, placeX, placeY);
        atomicGesture([{ op: "addBlock", block }], [block.id]);
        if (utility === "text") setTextEdit({ blockId: block.id, created: true });
        return;
      }
      const hits = inkItemsAtPoint(doc.blocks, worldX, worldY);
      const top = hits[0];
      if (!top || top.locked) {
        if (utility === "selectDirect") dispatch(inkCanvasActions.setSelection, { ids: [] });
        return;
      }
      if (utility === "selectDirect") {
        const nextSelection = event.shiftKey ? [...new Set([...selectedIds, top.id])] : [top.id];
        dispatch(inkCanvasActions.setSelection, { ids: nextSelection });
        beginMove(event, top.id);
      }
    },
    [atomicGesture, beginGesture, beginMove, dispatch, doc, interactive, isNavigator, selectedIds, utility],
  );

  const handleBlockPointerDown = useCallback(
    (event: React.PointerEvent, blockId: string) => {
      event.stopPropagation();
      if (!rootRef.current || !doc || !interactive) return;
      const block = findInkItem(doc, blockId);
      if (!block || block.locked) return;
      const nextSelection = event.shiftKey ? [...new Set([...selectedIds, blockId])] : [blockId];
      dispatch(inkCanvasActions.setSelection, { ids: nextSelection });
      if (utility === "selectDirect" || utility === "selectMarquee") beginMove(event, blockId);
    },
    [beginMove, dispatch, doc, interactive, selectedIds, utility],
  );

  const handleResizePointerDown = useCallback(
    (handle: InkResizeHandle, event: React.PointerEvent) => {
      event.stopPropagation();
      if (!rootRef.current || !selectionBounds) return;
      const rect = rootRef.current.getBoundingClientRect();
      setDragState({ kind: "resize", handle, fromBounds: selectionBounds, startX: event.clientX - rect.left, startY: event.clientY - rect.top, selectedIds: [...selectedIds] });
    },
    [selectedIds, selectionBounds],
  );

  const handlePointerMove = useCallback(
    (event: React.PointerEvent<HTMLDivElement>) => {
      if (!rootRef.current || !doc) return;
      const camera = doc.camera;
      const rect = rootRef.current.getBoundingClientRect();
      const screenX = event.clientX - rect.left;
      const screenY = event.clientY - rect.top;
      const [worldX, worldY] = screenToWorld(camera, screenX, screenY);
      if (!dragState) {
        if (!interactive) return;
        const hits = inkItemsAtPoint(doc.blocks, worldX, worldY);
        const top = hits[0] ?? null;
        dispatch(inkCanvasActions.setHover, { id: top?.id ?? null });
        return;
      }
      if (dragState.kind === "pan") {
        const nextCamera = { ...dragState.camera, x: dragState.camera.x + (screenX - dragState.startX), y: dragState.camera.y + (screenY - dragState.startY) };
        setDraftDoc((current) => ({ ...(current ?? doc), camera: nextCamera }));
        dispatch(inkCanvasActions.setCamera, { camera: nextCamera });
        return;
      }
      if (dragState.kind === "move") {
        const dx = (screenX - dragState.startX) / camera.zoom;
        const dy = (screenY - dragState.startY) / camera.zoom;
        const events: InkCanvasEvent[] = [];
        for (const [blockId, origin] of Object.entries(dragState.origins)) {
          const block = findInkItem(doc, blockId);
          if (!block) continue;
          events.push({ op: "updateBlock", blockId, block: { ...block, x: origin.x + dx, y: origin.y + dy } });
        }
        if (events.length) liveGesture(events);
        return;
      }
      if (dragState.kind === "marquee") {
        setMarqueePoints([dragState.start, { x: screenX, y: screenY }]);
        return;
      }
      if (dragState.kind === "stroke") {
        const block = findInkItem(doc, dragState.blockId);
        if (!block || block.kind !== "stroke") return;
        const localX = worldX - block.x;
        const localY = worldY - block.y;
        liveGesture([{ op: "updateBlock", blockId: block.id, block: { ...block, points: [...block.points, [localX, localY]] } }]);
        return;
      }
      if (dragState.kind === "eraser") {
        const events = dragState.mode === "eraserStroke" ? eraseInkStrokeEventsAtPoint(doc, worldX, worldY) : eraseInkStrokePointEventsNearPoint(doc, worldX, worldY, doc.eraserRadius ?? 12);
        if (events.length) liveGesture(events);
        return;
      }
      if (dragState.kind === "resize") {
        const dx = (screenX - dragState.startX) / camera.zoom;
        const dy = (screenY - dragState.startY) / camera.zoom;
        const toBounds = inkResizeBounds(dragState.fromBounds, dragState.handle, dx, dy);
        const events: InkCanvasEvent[] = [];
        for (const blockId of dragState.selectedIds) {
          const block = findInkItem(doc, blockId);
          if (!block) continue;
          events.push({ op: "updateBlock", blockId, block: inkScaleItemWithinGroup(block, dragState.fromBounds, toBounds) });
        }
        if (events.length) liveGesture(events);
      }
    },
    [dispatch, doc, dragState, interactive, liveGesture],
  );

  const handlePointerUp = useCallback(() => {
    if (!doc) {
      setDragState(null);
      setMarqueePoints([]);
      return;
    }
    if (dragState?.kind === "move") {
      const events: InkCanvasEvent[] = [];
      for (const blockId of Object.keys(dragState.origins)) {
        const block = findInkItem(doc, blockId);
        if (!block) continue;
        if (doc.snapEnabled) {
          const spacing = doc.snapGridSpacing ?? 8;
          const [x, y] = inkSnapWorldPoint(block.x, block.y, spacing);
          events.push({ op: "updateBlock", blockId, block: { ...block, x, y } });
        } else {
          events.push({ op: "updateBlock", blockId, block });
        }
      }
      commitGesture(events);
    } else if (dragState?.kind === "stroke") {
      const block = findInkItem(doc, dragState.blockId);
      if (block) commitGesture([{ op: "updateBlock", blockId: block.id, block }]);
      else commitGesture([]);
    } else if (dragState?.kind === "resize") {
      const events: InkCanvasEvent[] = [];
      for (const blockId of dragState.selectedIds) {
        const block = findInkItem(doc, blockId);
        if (block) events.push({ op: "updateBlock", blockId, block });
      }
      commitGesture(events);
    } else if (dragState?.kind === "eraser") {
      commitGesture([]);
    } else if (dragState?.kind === "pan") {
      flushPendingLive();
      gestureActiveRef.current = false;
    }
    if (dragState?.kind === "marquee" && marqueePoints.length >= 2 && rootRef.current) {
      const screenRect = screenRectFromPoints(marqueePoints);
      if (screenRect) {
        const camera = doc.camera;
        const worldRect = { x: (screenRect.x - camera.x) / camera.zoom, y: (screenRect.y - camera.y) / camera.zoom, width: screenRect.width / camera.zoom, height: screenRect.height / camera.zoom };
        dispatch(inkCanvasActions.setSelection, { ids: inkItemsIntersectingRect(doc.blocks, worldRect) });
      }
    }
    setDragState(null);
    setMarqueePoints([]);
  }, [commitGesture, dispatch, doc, dragState, flushPendingLive, marqueePoints]);

  const handleWheel = useCallback(
    (event: React.WheelEvent<HTMLDivElement>) => {
      if (!rootRef.current || !doc || isNavigator) return;
      event.preventDefault();
      const camera = doc.camera;
      const rect = rootRef.current.getBoundingClientRect();
      const screenX = event.clientX - rect.left;
      const screenY = event.clientY - rect.top;
      const zoomFactor = event.deltaY < 0 ? 1.08 : 0.92;
      const nextZoom = Math.min(8, Math.max(0.1, camera.zoom * zoomFactor));
      const worldX = (screenX - camera.x) / camera.zoom;
      const worldY = (screenY - camera.y) / camera.zoom;
      const nextCamera = { x: screenX - worldX * nextZoom, y: screenY - worldY * nextZoom, zoom: nextZoom };
      setDraftDoc((current) => ({ ...(current ?? doc), camera: nextCamera }));
      dispatch(inkCanvasActions.setCamera, { camera: nextCamera });
    },
    [dispatch, doc, isNavigator],
  );

  const handleDoubleClick = useCallback(
    (event: React.MouseEvent<HTMLDivElement>) => {
      if (!rootRef.current || !doc || isNavigator || !interactive) return;
      const camera = doc.camera;
      const rect = rootRef.current.getBoundingClientRect();
      const screenX = event.clientX - rect.left;
      const screenY = event.clientY - rect.top;
      const [worldX, worldY] = screenToWorld(camera, screenX, screenY);
      const hits = inkItemsAtPoint(doc.blocks, worldX, worldY);
      const top = hits[0];
      if (top?.kind === "text" && !top.locked) {
        setTableEdit(null);
        setTextEdit({ blockId: top.id });
        dispatch(inkCanvasActions.setSelection, { ids: [top.id] });
        return;
      }
      if (top?.kind === "table" && !top.locked) {
        const cell = inkTableCellAtPoint(top, worldX - top.x, worldY - top.y);
        if (!cell) return;
        setTextEdit(null);
        setTableEdit({ blockId: top.id, row: cell.row, col: cell.col });
        dispatch(inkCanvasActions.setSelection, { ids: [top.id] });
        return;
      }
      if (top) return;
      const [placeX, placeY] = inkMaybeSnapWorldPoint(doc, worldX, worldY);
      const block = createInkItemByKind("text", placeX, placeY);
      atomicGesture([{ op: "addBlock", block }], [block.id]);
      setTextEdit({ blockId: block.id, created: true });
    },
    [atomicGesture, dispatch, doc, interactive, isNavigator],
  );

  const commitTextEdit = useCallback(
    (blockId: string, paragraphs: readonly InkTextParagraph[], created?: boolean) => {
      if (!doc) {
        setTextEdit(null);
        return;
      }
      const block = findInkItem(doc, blockId);
      if (!block || block.kind !== "text") {
        setTextEdit(null);
        return;
      }
      const plain = inkTextPlainText(paragraphs).trim();
      if (!plain && created) {
        atomicGesture([{ op: "removeBlock", blockId }]);
        dispatch(inkCanvasActions.setSelection, { ids: [] });
      } else {
        atomicGesture([{ op: "updateBlock", blockId, block: { ...block, paragraphs } }]);
      }
      setTextEdit(null);
    },
    [atomicGesture, dispatch, doc],
  );

  const commitTableEdit = useCallback(
    (blockId: string, row: number, col: number, content: string, advance?: boolean) => {
      if (!doc) {
        setTableEdit(null);
        return;
      }
      const block = findInkItem(doc, blockId);
      if (!block || block.kind !== "table") {
        setTableEdit(null);
        return;
      }
      const rows = block.rows.map((entry, rowIndex) => (rowIndex === row ? entry.map((cell, colIndex) => (colIndex === col ? { content } : cell)) : entry));
      atomicGesture([{ op: "updateBlock", blockId, block: { ...block, rows } }]);
      if (advance) {
        const nextCol = col + 1 < block.columns.length ? col + 1 : 0;
        const nextRow = col + 1 < block.columns.length ? row : row + 1;
        if (nextRow < block.rows.length) setTableEdit({ blockId, row: nextRow, col: nextCol });
        else setTableEdit(null);
        return;
      }
      setTableEdit(null);
    },
    [atomicGesture, doc],
  );

  const pasteImageAsset = useCallback(
    (dataUrl: string, mime: string, worldX: number, worldY: number) => {
      const assetKey = `asset-${createInkHostId("image")}`;
      const imageBlock = createInkItemByKind("image", worldX - 120, worldY - 80);
      if (imageBlock.kind !== "image") return;
      atomicGesture(
        [
          { op: "putAsset", key: assetKey, asset: { mime, data: dataUrl } },
          { op: "addBlock", block: { ...imageBlock, imageKey: assetKey } },
        ],
        [imageBlock.id],
      );
    },
    [atomicGesture],
  );

  const handleCopy = useCallback(
    (event: React.ClipboardEvent<HTMLDivElement>) => {
      if (!doc) return;
      if (textEdit && (event.target as HTMLElement).closest("[contenteditable]")) return;
      if (!selectedIds.length) return;
      const blocks = selectedIds.map((id) => findInkItem(doc, id)).filter((block): block is InkItem => Boolean(block));
      if (!blocks.length) return;
      event.preventDefault();
      event.clipboardData.setData("text/plain", inkClipboardPayload(blocks));
    },
    [doc, selectedIds, textEdit],
  );

  const handlePaste = useCallback(
    (event: React.ClipboardEvent<HTMLDivElement>) => {
      if (!doc || !rootRef.current) return;
      if (textEdit && (event.target as HTMLElement).closest("[contenteditable]")) return;
      event.preventDefault();
      const rect = rootRef.current.getBoundingClientRect();
      const [worldX, worldY] = inkMaybeSnapWorldPoint(doc, ...screenToWorld(doc.camera, rect.width / 2, rect.height / 2));
      for (const item of event.clipboardData.items) {
        if (item.type.startsWith("image/")) {
          const file = item.getAsFile();
          if (!file) continue;
          const reader = new FileReader();
          reader.onload = () => {
            if (typeof reader.result === "string") pasteImageAsset(reader.result, file.type, worldX, worldY);
          };
          reader.readAsDataURL(file);
          return;
        }
      }
      const text = event.clipboardData.getData("text/plain");
      const clipboardBlocks = inkItemsFromClipboardPayload(text);
      if (clipboardBlocks) {
        const clones = cloneInkItemsWithOffset(clipboardBlocks, worldX, worldY);
        atomicGesture(
          clones.map((block) => ({ op: "addBlock", block }) as const),
          clones.map((block) => block.id),
        );
        return;
      }
      if (text.trim().startsWith("<svg")) {
        const assetKey = `asset-${createInkHostId("image")}`;
        const imageBlock = createInkItemByKind("image", worldX - 120, worldY - 80);
        if (imageBlock.kind !== "image") return;
        atomicGesture(
          [
            { op: "putAsset", key: assetKey, asset: { mime: "image/svg+xml", data: text.trim() } },
            { op: "addBlock", block: { ...imageBlock, imageKey: assetKey } },
          ],
          [imageBlock.id],
        );
        return;
      }
      if (text.trim()) {
        const block = createInkItemByKind("text", worldX, worldY);
        const seeded: InkTextItem = { ...(block as InkTextItem), paragraphs: inkTextParagraphsFromPlainText(text.trim()) };
        atomicGesture([{ op: "addBlock", block: seeded }], [seeded.id]);
      }
    },
    [atomicGesture, doc, pasteImageAsset, textEdit],
  );

  if (!scene || !doc) return <div className="text-muted-foreground p-2 text-xs">{emptySceneLabel}</div>;

  const camera = doc.camera;
  const visibleBlocks = flattenInkItems(doc.blocks);
  const gridColor = resolveSemanticColorHex("border");
  const gridSpacing = doc.gridSpacing ?? 32;
  const gridSubdivisions = doc.gridSubdivisions ?? 4;
  const gridOpacity = doc.gridOpacity ?? 0.35;
  const scale = isNavigator ? Math.min(0.2, 1 / Math.max(camera.zoom, 1)) : camera.zoom;
  const editingTextBlock = textEdit ? (findInkItem(doc, textEdit.blockId) as InkTextItem | null) : null;
  const editingTableBlock = tableEdit ? (findInkItem(doc, tableEdit.blockId) as InkTableItem | null) : null;

  return (
    <div
      ref={rootRef}
      tabIndex={0}
      data-surface-id={node.surfaceId}
      className={cn("bg-muted/20 relative h-full w-full touch-none overflow-hidden outline-none")}
      onPointerDown={handlePointerDown}
      onPointerMove={handlePointerMove}
      onPointerUp={handlePointerUp}
      onPointerLeave={handlePointerUp}
      onWheel={handleWheel}
      onDoubleClick={handleDoubleClick}
      onCopy={handleCopy}
      onPaste={handlePaste}
    >
      {doc.gridVisible !== false && !isNavigator ? <InkViewportGrid camera={camera} spacing={gridSpacing} subdivisions={gridSubdivisions} opacity={gridOpacity} color={gridColor} /> : null}
      <div className="absolute origin-top-left" style={{ transform: `translate(${camera.x}px, ${camera.y}px) scale(${scale})`, width: isNavigator ? 4000 : undefined, height: isNavigator ? 3000 : undefined }}>
        {visibleBlocks.map((block) => (
          <InkItemView key={block.id} block={block} assets={doc.assets} selected={selectedIds.includes(block.id)} hovered={hoveredId === block.id} hidden={textEdit?.blockId === block.id} onPointerDown={handleBlockPointerDown} />
        ))}
      </div>
      {showResizeHandles && selectionBounds ? <InkSelectionChrome camera={camera} bounds={selectionBounds} onResizePointerDown={handleResizePointerDown} /> : null}
      {editingTextBlock && textEdit?.blockId === editingTextBlock.id ? (
        <InkTextEditorOverlay
          block={editingTextBlock}
          screenBounds={{
            x: worldToScreen(camera, editingTextBlock.x, editingTextBlock.y).x,
            y: worldToScreen(camera, editingTextBlock.x, editingTextBlock.y).y,
            width: editingTextBlock.width * camera.zoom,
            height: editingTextBlock.height * camera.zoom,
          }}
          onCommit={(paragraphs) => commitTextEdit(editingTextBlock.id, paragraphs, textEdit.created)}
          onCancel={() => {
            if (textEdit.created) atomicGesture([{ op: "removeBlock", blockId: editingTextBlock.id }]);
            setTextEdit(null);
          }}
        />
      ) : null}
      {editingTableBlock && tableEdit
        ? (() => {
            const rowHeight = editingTableBlock.height / (editingTableBlock.rows.length + 1);
            const colWidth = editingTableBlock.width / editingTableBlock.columns.length;
            const cellX = editingTableBlock.x + tableEdit.col * colWidth;
            const cellY = editingTableBlock.y + (tableEdit.row + 1) * rowHeight;
            const screen = worldToScreen(camera, cellX, cellY);
            return (
              <InkTableCellEditorOverlay
                block={editingTableBlock}
                row={tableEdit.row}
                col={tableEdit.col}
                screenBounds={{ x: screen.x, y: screen.y, width: colWidth * camera.zoom, height: rowHeight * camera.zoom }}
                onCommit={(content, advance) => commitTableEdit(editingTableBlock.id, tableEdit.row, tableEdit.col, content, advance)}
                onCancel={() => setTableEdit(null)}
              />
            );
          })()
        : null}
      {marqueePoints.length >= 2 ? (
        <SelectionMarquee
          shape="rect"
          coverage={marqueeCoverageFromGesture({ method: "rectangle", startX: marqueePoints[0]!.x, endX: marqueePoints[marqueePoints.length - 1]!.x, path: marqueePoints })}
          rect={screenRectFromPoints(marqueePoints) ?? { x: 0, y: 0, width: 0, height: 0 }}
        />
      ) : null}
    </div>
  );
}
//#endregion InkCanvasHost
//#endregion 🔖InkCanvasHost

//#region 🔖GraphTimelineHost
//#region GraphTimelineHost
export function GraphTimelineHost({ node, onAction }: ComponentSceneHostProps) {
  const scene = node.graphTimeline;
  const emptySceneLabel = useLabel("ui.host.emptyScene");
  const columns = useMemo(() => {
    if (!scene) return [] as HistoryColumn[];
    try {
      return JSON.parse(scene.columnsJson) as HistoryColumn[];
    } catch {
      return [];
    }
  }, [scene]);

  if (!scene) return <div className="semio-graph-timeline-empty">{emptySceneLabel}</div>;

  return (
    <div className="semio-graph-timeline-host h-full min-h-0 w-full overflow-auto p-single" data-surface-id={node.surfaceId}>
      <HistoryTable
        columns={columns}
        onSelectCheckpoint={(checkpointId) =>
          onAction({
            controllerId: node.controllerId,
            action: "checkoutCheckpoint",
            args: { checkpointId },
          })
        }
      />
    </div>
  );
}
//#endregion GraphTimelineHost
//#endregion 🔖GraphTimelineHost

//#region 🔖BlockListHost
//#region BlockListHost
//#region Types
type BlockRecord = { readonly id: string; readonly label: string; readonly kind: string; readonly description?: string };
type StepRecord = { readonly id: string; readonly title: string; readonly description?: string; readonly blocks: readonly BlockRecord[] };
type PaletteEntryRecord = { readonly blockKind: string; readonly label: string; readonly iconId: string };
const PALETTE_DRAG_MIME = "application/x-semio-block-list-block-kind";
//#endregion Types

//#region Helpers
function resolveBlockIcon(iconId: string): IconName {
  return iconId in ICONS ? (iconId as IconName) : "circle-dot";
}

function dispatchBlockListAction(onAction: (action: ActionDescriptor) => void, controllerId: string, action: string, args: Record<string, unknown>): void {
  onAction({ controllerId, action, args });
}
//#endregion Helpers

//#region SortableRow
/** 🧩 Wraps a row in dnd-kit's sortable machinery so it can be reordered within its enclosing `SortableContext`. */
function SortableRow({ id, children }: { readonly id: string; readonly children: (dragHandleProps: { readonly ref: (node: HTMLElement | null) => void; readonly style: React.CSSProperties }) => React.ReactNode }) {
  const { attributes, listeners, setNodeRef, transform, transition, isDragging } = useSortable({ id });
  const style: React.CSSProperties = { transform: DndCSS.Transform.toString(transform), transition, opacity: isDragging ? 0.5 : 1 };
  return (
    <div ref={setNodeRef} style={style} {...attributes} {...listeners}>
      {children({ ref: setNodeRef, style })}
    </div>
  );
}
//#endregion SortableRow

//#region Block
function BlockCard({ block, stepId, controllerId, onAction }: { readonly block: BlockRecord; readonly stepId: string; readonly controllerId: string; readonly onAction: (action: ActionDescriptor) => void }) {
  return (
    <SortableRow id={block.id}>
      {() => (
        <div className="semio-block-card flex items-center gap-2 rounded border border-border bg-panel p-single" data-block-id={block.id}>
          <Icon icon="grip-vertical" size="small" />
          <div className="min-w-0 flex-1">
            <div className="truncate text-xs font-medium">{block.label}</div>
            <div className="truncate text-xs text-muted-foreground">{block.kind}</div>
          </div>
          <Button className="h-medium shrink-0 px-2" icon="trash-2" type="button" variant="outline" onClick={() => dispatchBlockListAction(onAction, controllerId, "removeBlock", { stepId, blockId: block.id })} />
        </div>
      )}
    </SortableRow>
  );
}
//#endregion Block

//#region Step
function StepCard({ step, palette, controllerId, onAction }: { readonly step: StepRecord; readonly palette: readonly PaletteEntryRecord[]; readonly controllerId: string; readonly onAction: (action: ActionDescriptor) => void }) {
  const blockIds = useMemo(() => step.blocks.map((block) => block.id), [step.blocks]);

  function handleBlockDragEnd(event: DragEndEvent) {
    const { active, over } = event;
    if (!over || active.id === over.id) return;
    const index = step.blocks.findIndex((block) => block.id === over.id);
    if (index === -1) return;
    dispatchBlockListAction(onAction, controllerId, "moveBlock", { blockId: active.id, fromStepId: step.id, toStepId: step.id, index });
  }

  return (
    <SortableRow id={step.id}>
      {() => (
        <div
          className="semio-step-card flex flex-col gap-2 rounded border border-border bg-panel p-single"
          data-step-id={step.id}
          onDragOver={(event) => {
            event.preventDefault();
            event.dataTransfer.dropEffect = "copy";
          }}
          onDrop={(event) => {
            event.preventDefault();
            const kind = event.dataTransfer.getData(PALETTE_DRAG_MIME);
            if (!kind) return;
            dispatchBlockListAction(onAction, controllerId, "addBlock", { stepId: step.id, kind });
          }}
        >
          <div className="flex items-center gap-2">
            <Icon icon="grip-vertical" size="small" />
            <div className="min-w-0 flex-1 truncate text-sm font-medium">{step.title}</div>
            <Button className="h-medium shrink-0 px-2" icon="trash-2" type="button" variant="outline" onClick={() => dispatchBlockListAction(onAction, controllerId, "removeStep", { stepId: step.id })} />
          </div>
          {step.description && <div className="text-xs text-muted-foreground">{step.description}</div>}
          <DndContext collisionDetection={closestCenter} onDragEnd={handleBlockDragEnd}>
            <SortableContext items={blockIds} strategy={verticalListSortingStrategy}>
              <div className="flex flex-col gap-1">
                {step.blocks.map((block) => (
                  <BlockCard key={block.id} block={block} stepId={step.id} controllerId={controllerId} onAction={onAction} />
                ))}
              </div>
            </SortableContext>
          </DndContext>
        </div>
      )}
    </SortableRow>
  );
}
//#endregion Step

//#region Palette
function PalettePanel({ palette, controllerId, onAction }: { readonly palette: readonly PaletteEntryRecord[]; readonly controllerId: string; readonly onAction: (action: ActionDescriptor) => void }) {
  return (
    <div className="semio-palette flex shrink-0 flex-col gap-1 border-l border-border p-single">
      {palette.map((entry) => (
        <div
          key={entry.blockKind}
          draggable
          onDragStart={(event) => {
            event.dataTransfer.setData(PALETTE_DRAG_MIME, entry.blockKind);
            event.dataTransfer.effectAllowed = "copy";
          }}
          className="flex cursor-grab items-center gap-1 rounded border border-border p-single text-xs"
          onClick={() => dispatchBlockListAction(onAction, controllerId, "addBlock", { kind: entry.blockKind })}
        >
          <Icon icon={resolveBlockIcon(entry.iconId)} size="small" />
          {entry.label}
        </div>
      ))}
    </div>
  );
}
//#endregion Palette

//#region Component
export function BlockListHost({ node, onAction }: ComponentSceneHostProps) {
  const scene = node.blockList;
  const steps = useMemo(() => {
    if (!scene) return [] as StepRecord[];
    try {
      return JSON.parse(scene.stepsJson) as StepRecord[];
    } catch {
      return [];
    }
  }, [scene]);
  const palette = useMemo(() => {
    if (!scene) return [] as PaletteEntryRecord[];
    try {
      return JSON.parse(scene.paletteJson) as PaletteEntryRecord[];
    } catch {
      return [];
    }
  }, [scene]);
  const stepIds = useMemo(() => steps.map((step) => step.id), [steps]);
  const stepsLabel = useLabel("ui.blockList.steps");
  const addStepLabel = useLabel("ui.blockList.addStep");
  const emptyLabel = useLabel("ui.host.emptyScene");

  if (!scene) return <div className="semio-block-list-empty">{emptyLabel}</div>;

  function handleStepDragEnd(event: DragEndEvent) {
    const { active, over } = event;
    if (!over || active.id === over.id) return;
    const index = steps.findIndex((step) => step.id === over.id);
    if (index === -1) return;
    dispatchBlockListAction(onAction, node.controllerId, "moveStep", { stepId: active.id, index });
  }

  return (
    <div className="semio-block-list-host flex h-full min-h-0 w-full" data-surface-id={node.surfaceId}>
      <div className="flex min-w-0 flex-1 flex-col gap-2 overflow-auto p-single">
        <div className="flex items-center justify-between">
          <span className="text-sm font-medium">{stepsLabel}</span>
          <Button className="h-medium shrink-0 px-2" icon="plus" text={addStepLabel} type="button" variant="outline" onClick={() => dispatchBlockListAction(onAction, node.controllerId, "addStep", {})} />
        </div>
        <DndContext collisionDetection={closestCenter} onDragEnd={handleStepDragEnd}>
          <SortableContext items={stepIds} strategy={verticalListSortingStrategy}>
            <div className="flex flex-col gap-2">
              {steps.map((step) => (
                <StepCard key={step.id} step={step} palette={palette} controllerId={node.controllerId} onAction={onAction} />
              ))}
            </div>
          </SortableContext>
        </DndContext>
      </div>
      <PalettePanel palette={palette} controllerId={node.controllerId} onAction={onAction} />
    </div>
  );
}
//#endregion Component
//#endregion BlockListHost
//#endregion 🔖BlockListHost

//#region 🔖DiffViewHost
//#region DiffViewHost
//#region Types
type DiffLineKind = "equal" | "add" | "remove";
type DiffLine = { readonly kind: DiffLineKind; readonly beforeNo?: number; readonly afterNo?: number; readonly text: string };
type SplitRow = { readonly left?: DiffLine; readonly right?: DiffLine };
//#endregion Types

//#region LineDiff
/** 🔍 Minimal O(before·after) LCS-based line diff — no external dependency, adequate for the moderate-sized before/after buffers a `DiffViewScene` carries. */
function diffLines(before: readonly string[], after: readonly string[]): DiffLine[] {
  const beforeLen = before.length;
  const afterLen = after.length;
  const lcs: number[][] = Array.from({ length: beforeLen + 1 }, () => new Array<number>(afterLen + 1).fill(0));
  for (let i = beforeLen - 1; i >= 0; i--) {
    for (let j = afterLen - 1; j >= 0; j--) {
      lcs[i][j] = before[i] === after[j] ? lcs[i + 1][j + 1] + 1 : Math.max(lcs[i + 1][j], lcs[i][j + 1]);
    }
  }
  const lines: DiffLine[] = [];
  let i = 0;
  let j = 0;
  while (i < beforeLen && j < afterLen) {
    if (before[i] === after[j]) {
      lines.push({ kind: "equal", beforeNo: i + 1, afterNo: j + 1, text: before[i] });
      i += 1;
      j += 1;
    } else if (lcs[i + 1][j] >= lcs[i][j + 1]) {
      lines.push({ kind: "remove", beforeNo: i + 1, text: before[i] });
      i += 1;
    } else {
      lines.push({ kind: "add", afterNo: j + 1, text: after[j] });
      j += 1;
    }
  }
  while (i < beforeLen) {
    lines.push({ kind: "remove", beforeNo: i + 1, text: before[i] });
    i += 1;
  }
  while (j < afterLen) {
    lines.push({ kind: "add", afterNo: j + 1, text: after[j] });
    j += 1;
  }
  return lines;
}

/** 🪞 Pairs consecutive remove/add runs into aligned rows for the split-pane layout; equal lines mirror onto both sides. */
function buildSplitRows(diff: readonly DiffLine[]): SplitRow[] {
  const rows: SplitRow[] = [];
  let i = 0;
  while (i < diff.length) {
    const line = diff[i];
    if (line.kind === "equal") {
      rows.push({ left: line, right: line });
      i += 1;
      continue;
    }
    const removes: DiffLine[] = [];
    while (i < diff.length && diff[i].kind === "remove") {
      removes.push(diff[i]);
      i += 1;
    }
    const adds: DiffLine[] = [];
    while (i < diff.length && diff[i].kind === "add") {
      adds.push(diff[i]);
      i += 1;
    }
    const pairCount = Math.max(removes.length, adds.length);
    for (let pair = 0; pair < pairCount; pair += 1) {
      rows.push({ left: removes[pair], right: adds[pair] });
    }
  }
  return rows;
}
//#endregion LineDiff

//#region Rendering
const DIFF_LINE_CLASS: Record<DiffLineKind, string> = {
  equal: "text-foreground",
  add: "text-emerald-400",
  remove: "text-destructive",
};

const DIFF_LINE_PREFIX: Record<DiffLineKind, string> = { equal: " ", add: "+", remove: "-" };

function UnifiedDiff({ lines }: { readonly lines: readonly DiffLine[] }) {
  return (
    <div className="semio-diff-view-unified font-mono text-xs">
      {lines.map((line, index) => (
        <div key={index} className={cn("flex gap-single whitespace-pre-wrap px-single", DIFF_LINE_CLASS[line.kind])}>
          <span className="text-muted-foreground w-10 shrink-0 select-none text-right tabular-nums">{line.beforeNo ?? ""}</span>
          <span className="text-muted-foreground w-10 shrink-0 select-none text-right tabular-nums">{line.afterNo ?? ""}</span>
          <span className="w-3 shrink-0 select-none">{DIFF_LINE_PREFIX[line.kind]}</span>
          <span>{line.text}</span>
        </div>
      ))}
    </div>
  );
}

function SplitDiffPane({ rows, side }: { readonly rows: readonly SplitRow[]; readonly side: "left" | "right" }) {
  return (
    <div className="semio-diff-view-split-pane min-w-0 flex-1 font-mono text-xs">
      {rows.map((row, index) => {
        const line = row[side];
        return (
          <div key={index} className={cn("flex gap-single whitespace-pre-wrap px-single", line ? DIFF_LINE_CLASS[line.kind] : "text-muted-foreground")}>
            <span className="text-muted-foreground w-10 shrink-0 select-none text-right tabular-nums">{line ? (side === "left" ? line.beforeNo : line.afterNo) : ""}</span>
            <span>{line?.text ?? ""}</span>
          </div>
        );
      })}
    </div>
  );
}
//#endregion Rendering

//#region Component
/** @emoji 🆚 Renders a `DiffViewScene`: a minimal, dependency-free line-based diff between `before`/`after`, unified (default) or split per `mode`. */
export function DiffViewHost({ node }: ComponentSceneHostProps) {
  const scene = node.diffView;
  const emptySceneLabel = useLabel("ui.host.emptyScene");
  const before = scene?.before ?? "";
  const after = scene?.after ?? "";
  const lines = useMemo(() => diffLines(before.split("\n"), after.split("\n")), [before, after]);
  const splitRows = useMemo(() => (scene?.mode === "split" ? buildSplitRows(lines) : []), [lines, scene?.mode]);

  if (!scene) return <div className="semio-diff-view-empty">{emptySceneLabel}</div>;

  return (
    <div className="semio-diff-view-host h-full min-h-0 w-full overflow-auto p-single" data-surface-id={node.surfaceId} data-diff-language={scene.language}>
      {scene.mode === "split" ? (
        <div className="flex min-h-0 w-full gap-single">
          <SplitDiffPane rows={splitRows} side="left" />
          <div className="border-border w-px shrink-0 border-l" />
          <SplitDiffPane rows={splitRows} side="right" />
        </div>
      ) : (
        <UnifiedDiff lines={lines} />
      )}
    </div>
  );
}
//#endregion Component
//#endregion DiffViewHost
//#endregion 🔖DiffViewHost

//#region 🔖EventFeedHost
//#region EventFeedHost
//#region Helpers
function resolveFeedIcon(iconId: string): IconName {
  return iconId in ICONS ? (iconId as IconName) : "circle-dot";
}

const FEED_TONE_CLASS: Record<string, string> = {
  info: "text-foreground",
  success: "text-emerald-400",
  warning: "text-amber-400",
  error: "text-destructive",
};

function formatFeedTimestamp(timestampMs: number): string {
  try {
    return new Date(timestampMs).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit", second: "2-digit" });
  } catch {
    return "";
  }
}
//#endregion Helpers

//#region Component
/** @emoji 📰 Renders an `EventFeedScene`: a scrollable log of `entriesJson` entries (icon + timestamp + title/detail), auto-scrolling to the newest entry while `follow` is set, dispatching `activateAction` on entry click. */
export function EventFeedHost({ node, onAction }: ComponentSceneHostProps) {
  const scene = node.eventFeed;
  const emptySceneLabel = useLabel("ui.host.emptyScene");
  const entries = useMemo(() => {
    if (!scene?.entriesJson) return [] as EventFeedEntry[];
    try {
      return JSON.parse(scene.entriesJson) as EventFeedEntry[];
    } catch {
      return [];
    }
  }, [scene?.entriesJson]);
  const listRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (!scene?.follow) return;
    const list = listRef.current;
    if (!list) return;
    list.scrollTop = list.scrollHeight;
  }, [entries, scene?.follow]);

  if (!scene) return <div className="semio-event-feed-empty">{emptySceneLabel}</div>;

  const activateAction = scene.activateAction;
  return (
    <div ref={listRef} className="semio-event-feed-host flex h-full min-h-0 w-full flex-col gap-single overflow-auto p-single" data-surface-id={node.surfaceId}>
      {entries.map((entry) => (
        <div
          key={entry.id}
          className={cn("flex items-start gap-single rounded-md p-single", activateAction && "hover:bg-panel cursor-pointer")}
          role={activateAction ? "button" : undefined}
          onClick={
            activateAction
              ? () =>
                  onAction({
                    controllerId: node.controllerId,
                    action: activateAction,
                    args: { surfaceId: node.surfaceId, id: entry.id },
                  })
              : undefined
          }
        >
          <Icon icon={resolveFeedIcon(entry.iconId)} size="small" />
          <div className="min-w-0 flex-1">
            <div className="flex items-center gap-single">
              <span className={cn("truncate text-xs font-medium", entry.tone ? FEED_TONE_CLASS[entry.tone] : undefined)}>{entry.title}</span>
              <span className="text-muted-foreground ml-auto shrink-0 text-[10px] tabular-nums">{formatFeedTimestamp(entry.timestampMs)}</span>
            </div>
            {entry.detail ? <p className="text-muted-foreground truncate text-xs">{entry.detail}</p> : null}
          </div>
        </div>
      ))}
    </div>
  );
}
//#endregion Component
//#endregion EventFeedHost
//#endregion 🔖EventFeedHost
