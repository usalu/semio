// #region 🧲Header
// 💻 elements/client/lib/board/play/react.tsx — Board play: triptych Nakagin views, in-app fixture drag shelf, selection inspector, `UI` shell (same `@elements/ui` + globals pattern as semio rendering / algorithms).
// #endregion 🧲Header

// #region 📥Imports
import {
  Button,
  Expertise,
  IconSelector,
  Input,
  Label,
  LevelProvider,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  Slider,
  ToolbarDivider,
  ToolbarGroup,
  ToolbarItem,
  ToolbarZone,
  Tree,
  TreeStateProvider,
  Controller,
  NativeDragAndDropController,
  PointerDragController,
  PureSidePanelTabDefinition,
  Workbench,
  WorkbenchApp,
  WorkbenchWindowKind,
  WorkbenchView,
  createWindowLayout,
  getLevelBgClass,
  StaticTreePanelDefinition,
  useElementsSurfaceChrome,
  type CommandBus,
  type ContextMenuItem,
  type ElementsSurfaceDevice,
  type ElementsSurfaceTheme,
  type FooterItem,
  type TreeDataSection,
  type UIWindowKindDefinition,
  type UIWindowLayout,
} from "@elements/ui";
import { BoxSelect, Circle, ClipboardList, Lasso, Library, Link2, Magnet, Minus, MousePointer2, Pause, Play, Plus, Repeat2, Settings, Square } from "lucide-react";
import { createContext, useCallback, useContext, useEffect, useMemo, useRef, useState, type ChangeEvent, type DragEvent, type PointerEvent, type ReactElement, type ReactNode } from "react";

import nakaginFixtureJson from "./fixtures/nakagin-capsule-tower.board.json";
import {
  BOARD_BUILTIN_PORT_HANDLE_KIND,
  BOARD_CAMERA_ZOOM_MAX,
  BOARD_CAMERA_ZOOM_MIN,
  BOARD_DEFAULT_KIND_CATALOG_BUNDLE,
  BOARD_FIXTURE_DRAG_KIND_PALETTE_NODE,
  BOARD_FIXTURE_DRAG_V1_MIME,
  BOARD_LOD_MODE_AUTOMATIC,
  boardLodAutomaticSelectLabel,
  BOARD_PRESELECT_EMPTY,
  BOARD_SELECTION_TARGETS_DEFAULT,
  DEFAULT_BOARD_LOD_ZOOM_THRESHOLDS,
  normalizeBoardSelectionProp,
  isBoardDrawLodKind,
  boardFixtureMetaKindCatalogBundle,
  boardFixtureNodeCaption,
  classifyElementsBoardIconSelectorMode,
  encodeBoardFixtureForDragV1,
  layoutBoardFixtureRedrawHandles,
  layoutBoardFixtureRedrawNodes,
  mergeBoardKindCatalogBundleByRowId,
  parseBoardFixtureV1,
  type BoardFixtureCircleNodeV1,
  type BoardFixtureDropDetail,
  type BoardFixtureEdgeV1,
  type BoardFixtureHandleV1,
  type BoardFixtureNodeV1,
  type BoardFixtureRectangleNodeV1,
  type BoardFixtureV1,
  type BoardForceGraphLayoutOptions,
  type BoardHierarchicalTreeDirectionKind,
  type BoardRedrawLayoutOptions,
  type BoardRedrawModeKind,
  type BoardDrawLodKind,
  type BoardLodModeKind,
  type BoardPreselectSnapshot,
  type BoardSelectionMethod,
  type BoardSelectionMode,
  type BoardSelectionSnapshot,
  type BoardSelectionTargets,
  type CameraState,
} from "../index";
import { BoardCanvas, Edge, Handle, Node, useBoardEvent } from "../index.tsx";
import "./globals.css";
// #endregion 📥Imports

const NAKAGIN_BOARD_PLAY_KIND_CATALOGS = mergeBoardKindCatalogBundleByRowId({ ...BOARD_DEFAULT_KIND_CATALOG_BUNDLE }, boardFixtureMetaKindCatalogBundle(nakaginFixtureJson) ?? {});

// #region 🔖Kinds
export type BoardPlayPaneId = "board-overview" | "board-detail" | "board-selection";

const BOARD_PLAY_APP_ID = "elements-board-play";

const boardPlayOverviewWindowContextMenu: ContextMenuItem[] = [{ id: "win-demo", label: "Overview window menu demo" }];
const boardPlayDemoNodeContextMenu: ContextMenuItem[] = [
  { id: "demo-node", label: "Demo capsule action" },
  { children: [{ id: "demo-sub-1", label: "Nested item" }], id: "demo-sub", label: "Demo nested" },
];
const boardPlayDemoEdgeContextMenu: ContextMenuItem[] = [{ id: "demo-edge", label: "Demo edge action" }];
const boardPlayCanvasBackgroundMenu: ContextMenuItem[] = [{ id: "demo-bg", label: "Board background menu" }];

const LS_THEME = "elements.board-play.surface.theme";
const LS_DEVICE = "elements.board-play.surface.device";
const LS_EXPERTISE = "elements.board-play.surface.expertise";

function parseStoredTheme(raw: string | null): ElementsSurfaceTheme {
  if (raw === "light" || raw === "dark" || raw === "system") return raw;
  return "system";
}

function parseStoredDevice(raw: string | null): ElementsSurfaceDevice {
  if (raw === "desktop" || raw === "tablet" || raw === "mobile") return raw;
  return "desktop";
}

function parseStoredExpertise(raw: string | null): Expertise {
  if (raw === Expertise.BEGINNER || raw === Expertise.NORMAL || raw === Expertise.EXPERT) return raw;
  return Expertise.NORMAL;
}

function readTheme(): ElementsSurfaceTheme {
  if (typeof localStorage === "undefined") return "system";
  try {
    return parseStoredTheme(localStorage.getItem(LS_THEME));
  } catch {
    return "system";
  }
}

function readDevice(): ElementsSurfaceDevice {
  if (typeof localStorage === "undefined") return "desktop";
  try {
    return parseStoredDevice(localStorage.getItem(LS_DEVICE));
  } catch {
    return "desktop";
  }
}

function readExpertise(): Expertise {
  if (typeof localStorage === "undefined") return Expertise.NORMAL;
  try {
    return parseStoredExpertise(localStorage.getItem(LS_EXPERTISE));
  } catch {
    return Expertise.NORMAL;
  }
}
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
function triptychCamerasFromFixture(fixture: BoardFixtureV1): Record<BoardPlayPaneId, CameraState> {
  const { cx, cy, halfSpan } = fixtureWorldBounds(fixture);
  const base = fixture.camera;
  const margin = 0.06;
  const usable = REF_VIEWPORT_SHORT_PX * (1 - 2 * margin);
  const worldSpan = Math.max(2 * halfSpan, 1);
  const zoom = clampZoom(usable / worldSpan);
  const cam: CameraState = { x: cx + base.x, y: cy + base.y, zoom };
  return {
    "board-detail": { ...cam },
    "board-overview": { ...cam },
    "board-selection": { ...cam },
  };
}

/** @emoji ⏱️ After redraw play stops: camera stays fixed for the first third of this span, then eases in the remaining two thirds to bbox fit (3s total). */
const BOARD_PLAY_CAMERA_POST_REDRAW_TOTAL_MS = 3000;

/** @emoji ⏱️ After one-shot “Redraw nodes”, shell cameras ease to bbox fit (first third hold, last two thirds smooth). */
const BOARD_PLAY_NODES_REDRAW_CAMERA_EASE_TOTAL_MS = 1800;

/** @emoji 📷 Linear blend toward bbox-fit cameras each fixture commit while redraw play is on (damped follow). */
const BOARD_PLAY_REDRAW_CAMERA_CHASE_BLEND = 0.22;

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
function blendTriptychCamerasActivePaneOnly(from: Record<BoardPlayPaneId, CameraState>, to: Record<BoardPlayPaneId, CameraState>, tLinear: number, activePane: BoardPlayPaneId): Record<BoardPlayPaneId, CameraState> {
  const out: Record<BoardPlayPaneId, CameraState> = {
    "board-detail": { ...from["board-detail"] },
    "board-overview": { ...from["board-overview"] },
    "board-selection": { ...from["board-selection"] },
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
interface BoardPlayShellValue {
  fixture: BoardFixtureV1;
  setFixture: (next: BoardFixtureV1) => void;
  /** @emoji 🎯 Palette drags merge one node at the pointer; full fixtures replace the graph. */
  handleCanvasFixtureDrop: (pane: BoardPlayPaneId, detail: BoardFixtureDropDetail) => void;
  patchFixture: (updater: (prev: BoardFixtureV1) => BoardFixtureV1) => void;
  activePaneId: BoardPlayPaneId;
  setActivePaneId: (id: BoardPlayPaneId) => void;
  selectionIds: Set<string>;
  setSelectionIds: (ids: readonly string[]) => void;
  preselection: BoardPreselectSnapshot;
  setPreselection: (snapshot: BoardPreselectSnapshot) => void;
  hoveredId: string | null;
  /** @emoji 🖱️ Pane that currently owns pointer hover updates for shared {@link BoardPlayShellValue.hoveredId}. */
  hoverSourcePane: BoardPlayPaneId | null;
  setHoverPane: (pane: BoardPlayPaneId) => void;
  setHoverForPane: (pane: BoardPlayPaneId, id: string | null) => void;
  clearHoverForPane: (pane: BoardPlayPaneId) => void;
  /** @emoji 🔁 Rewrites selection ids when an object id changes (`replacedId` → `replacementId`); unrelated to edge endpoint fields. */
  remapIdInSelections: (replacedId: string, replacementId: string) => void;
  camerasByPane: Record<BoardPlayPaneId, CameraState>;
  /** @emoji 📷 Writes the **active** pane’s imperative camera (wheel/pan) into that pane’s entry in {@link boardPlayPaneCamerasBaseline}; other panes unchanged. */
  syncBaselineFromViewportCamera: (cam: CameraState) => void;
  boardSelectionMethod: BoardSelectionMethod;
  setBoardSelectionMethod: (value: BoardSelectionMethod) => void;
  boardSelectionMode: BoardSelectionMode;
  setBoardSelectionMode: (value: BoardSelectionMode) => void;
  boardSelectionTargets: BoardSelectionTargets;
  setBoardSelectionTargets: (value: BoardSelectionTargets | ((prev: BoardSelectionTargets) => BoardSelectionTargets)) => void;
  boardGridSnapEnabled: boolean;
  setBoardGridSnapEnabled: (value: boolean) => void;
  /** @emoji 📶 Per-pane LOD select value (`automatic` or a pinned tier). */
  boardLodModeByPane: Record<BoardPlayPaneId, BoardLodModeKind>;
  setBoardLodModeForPane: (pane: BoardPlayPaneId, mode: BoardLodModeKind) => void;
  /** @emoji 🗑️ Drops ids from the shared fixture after the canvas emits structural delete events. */
  applyStructuralDelete: (kind: "edge" | "node", id: string) => void;
  /** @emoji ⏯️ When true, play runs layout work on `requestAnimationFrame` (graph packs multiple WASM passes per ~14ms frame; tree one pass per frame). */
  boardRedrawPlaying: boolean;
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

class BoardFixtureLibraryPanelDefinition extends PureSidePanelTabDefinition {
  resolveTab() {
    return {
      id: "board-play-library",
      icon: Library,
      order: 0,
      tree: new StaticTreePanelDefinition({
        sections: [{ id: "board-play-library.section", content: <BoardFixtureLibraryPanel /> }],
      }),
    };
  }
}

class BoardSelectionInspectorPanelDefinition extends PureSidePanelTabDefinition {
  resolveTab() {
    return {
      id: "board-play-inspector",
      icon: ClipboardList,
      order: 0,
      tree: new StaticTreePanelDefinition({
        sections: createBoardSelectionInspectorSections(),
      }),
    };
  }
}

class BoardPlaySettingsPanelDefinition extends PureSidePanelTabDefinition {
  resolveTab() {
    return {
      id: "board-play-settings",
      icon: Settings,
      order: 1,
      tree: new StaticTreePanelDefinition({
        sections: [{ id: "board-play-settings.section", content: <BoardPlaySettingsPanel /> }],
      }),
    };
  }
}

const BoardPlayShellContext = createContext<BoardPlayShellValue | null>(null);

const BoardPlayLodRuntimeContext = createContext<((pane: BoardPlayPaneId, lod: BoardDrawLodKind) => void) | null>(null);

function useBoardPlayShell(): BoardPlayShellValue {
  const value = useContext(BoardPlayShellContext);
  if (!value) {
    throw new Error("useBoardPlayShell must be used inside BoardPlayShellContext.");
  }
  return value;
}
// #endregion 🔖ShellContext

// #region 🔖Toolbar
function newBoardAuthoringId(prefix: string): string {
  if (typeof globalThis.crypto !== "undefined" && typeof globalThis.crypto.randomUUID === "function") {
    return `${prefix}-${globalThis.crypto.randomUUID()}`;
  }
  return `${prefix}-${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 10)}`;
}

function boardToolbarToggleClass(active: boolean): string {
  return [
    "inline-flex shrink-0 items-center justify-center rounded px-2 py-1 text-xs font-medium transition-colors",
    active ? "bg-accent text-accent-foreground border border-element" : "text-muted-foreground hover:bg-hover-panel border border-transparent",
  ].join(" ");
}

/** @emoji 📐 Default node span in px: circle radius = span/2; rectangle width = height = span (40×40). */
const BOARD_PLAY_DEFAULT_NODE_SIZE_PX = 40;

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
  pane: BoardPlayPaneId,
  camerasByPane: Record<BoardPlayPaneId, CameraState>,
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

/** @emoji 🧰 Sketchpad-style tools: marquee kind, merge mode, hit target, and circle or rectangle authoring at the active pane camera. */
function BoardPlayToolbar(): ReactElement {
  const {
    activePaneId,
    applyBoardRedrawHandlesOnce,
    boardGridSnapEnabled,
    boardSelectionMethod,
    boardSelectionMode,
    boardSelectionTargets,
    camerasByPane,
    boardRedrawPlaying,
    patchFixture,
    setBoardGridSnapEnabled,
    setBoardSelectionMethod,
    setBoardSelectionMode,
    setBoardSelectionTargets,
    setBoardRedrawPlaying,
    setSelectionIds,
  } = useBoardPlayShell();

  const camera = camerasByPane[activePaneId];

  const appendCircle = useCallback(() => {
    const id = newBoardAuthoringId("node");
    const handleId = `${id}.h0`;
    const node: BoardFixtureCircleNodeV1 = {
      handles: [{ angle: 0, handleKind: BOARD_BUILTIN_PORT_HANDLE_KIND, id: handleId }],
      id,
      radius: BOARD_PLAY_DEFAULT_NODE_SIZE_PX / 2,
      x: camera.x,
      y: camera.y,
    };
    patchFixture((prev) => ({ ...prev, nodes: [...prev.nodes, node] }));
    setSelectionIds([id]);
  }, [camera.x, camera.y, patchFixture, setSelectionIds]);

  const appendRectangle = useCallback(() => {
    const id = newBoardAuthoringId("node");
    const handleId = `${id}.h0`;
    const d = BOARD_PLAY_DEFAULT_NODE_SIZE_PX;
    const node: BoardFixtureRectangleNodeV1 = {
      handles: [{ angle: 0, handleKind: BOARD_BUILTIN_PORT_HANDLE_KIND, id: handleId }],
      height: d,
      id,
      shape: "rectangle",
      width: d,
      x: camera.x,
      y: camera.y,
    };
    patchFixture((prev) => ({ ...prev, nodes: [...prev.nodes, node] }));
    setSelectionIds([id]);
  }, [camera.x, camera.y, patchFixture, setSelectionIds]);

  return (
    <div className="pointer-events-none flex w-full justify-center px-2 py-1">
      <ToolbarZone className="pointer-events-auto max-w-full flex-wrap justify-center gap-(--toolbar-gap) px-2">
        <ToolbarGroup className="min-w-0 items-center gap-1">
          <ToolbarItem>
            <span className="text-muted-foreground pr-1 text-[10px] font-semibold uppercase tracking-wide">Select</span>
          </ToolbarItem>
          <ToolbarItem>
            <button type="button" className={boardToolbarToggleClass(boardSelectionMethod === "rectangle")} title="Rectangle selection" onClick={() => setBoardSelectionMethod("rectangle")}>
              <BoxSelect className="size-4" aria-hidden />
            </button>
          </ToolbarItem>
          <ToolbarItem>
            <button type="button" className={boardToolbarToggleClass(boardSelectionMethod === "lasso")} title="Lasso selection" onClick={() => setBoardSelectionMethod("lasso")}>
              <Lasso className="size-4" aria-hidden />
            </button>
          </ToolbarItem>
          <ToolbarItem>
            <button type="button" className={boardToolbarToggleClass(boardSelectionMode === "default")} title="Default" onClick={() => setBoardSelectionMode("default")}>
              <MousePointer2 className="size-4" aria-hidden />
            </button>
          </ToolbarItem>
          <ToolbarItem>
            <button type="button" className={boardToolbarToggleClass(boardSelectionMode === "additive")} title="Additive" onClick={() => setBoardSelectionMode("additive")}>
              <Plus className="size-4" aria-hidden />
            </button>
          </ToolbarItem>
          <ToolbarItem>
            <button type="button" className={boardToolbarToggleClass(boardSelectionMode === "subtractive")} title="Subtractive" onClick={() => setBoardSelectionMode("subtractive")}>
              <Minus className="size-4" aria-hidden />
            </button>
          </ToolbarItem>
          <ToolbarItem>
            <button type="button" className={boardToolbarToggleClass(boardSelectionMode === "invertive")} title="Invertive" onClick={() => setBoardSelectionMode("invertive")}>
              <Repeat2 className="size-4" aria-hidden />
            </button>
          </ToolbarItem>
          <ToolbarItem>
            <span className="text-muted-foreground pr-1 text-[10px] font-semibold uppercase tracking-wide">Targets</span>
          </ToolbarItem>
          <ToolbarItem>
            <button type="button" className={boardToolbarToggleClass(boardSelectionTargets.nodes)} title="Select nodes" onClick={() => setBoardSelectionTargets((p) => ({ ...p, nodes: !p.nodes }))}>
              <span className="px-0.5">Nodes</span>
            </button>
          </ToolbarItem>
          <ToolbarItem>
            <button type="button" className={boardToolbarToggleClass(boardSelectionTargets.edges)} title="Select edges" onClick={() => setBoardSelectionTargets((p) => ({ ...p, edges: !p.edges }))}>
              <span className="px-0.5">Edges</span>
            </button>
          </ToolbarItem>
          <ToolbarItem>
            <button type="button" className={boardToolbarToggleClass(boardSelectionTargets.handles)} title="Select handles" onClick={() => setBoardSelectionTargets((p) => ({ ...p, handles: !p.handles }))}>
              <span className="px-0.5">Handles</span>
            </button>
          </ToolbarItem>
        </ToolbarGroup>
        <ToolbarDivider />
        <ToolbarGroup className="min-w-0 items-center gap-1">
          <ToolbarItem>
            <span className="text-muted-foreground pr-1 text-[10px] font-semibold uppercase tracking-wide">Grid</span>
          </ToolbarItem>
          <ToolbarItem>
            <button type="button" className={boardToolbarToggleClass(boardGridSnapEnabled)} title="Snap node drags to the finest visible LOD grid" onClick={() => setBoardGridSnapEnabled(!boardGridSnapEnabled)}>
              <Magnet className="size-4" aria-hidden />
            </button>
          </ToolbarItem>
        </ToolbarGroup>
        <ToolbarDivider />
        <ToolbarGroup className="min-w-0 items-center gap-1">
          <ToolbarItem>
            <span className="text-muted-foreground pr-1 text-[10px] font-semibold uppercase tracking-wide">Create</span>
          </ToolbarItem>
          <ToolbarItem>
            <button type="button" className={boardToolbarToggleClass(false)} title="Circle" onClick={appendCircle}>
              <Circle className="size-4" aria-hidden />
            </button>
          </ToolbarItem>
          <ToolbarItem>
            <button type="button" className={boardToolbarToggleClass(false)} title="Rectangle" onClick={appendRectangle}>
              <Square className="size-4" aria-hidden />
            </button>
          </ToolbarItem>
        </ToolbarGroup>
        <ToolbarDivider />
        <ToolbarGroup className="min-w-0 items-center gap-1">
          <ToolbarItem>
            <span className="text-muted-foreground pr-1 text-[10px] font-semibold uppercase tracking-wide">Layout</span>
          </ToolbarItem>
          <ToolbarItem>
            <button
              type="button"
              className={boardToolbarToggleClass(boardRedrawPlaying)}
              title={boardRedrawPlaying ? "Pause redraw (requestAnimationFrame; packs WASM work per frame)" : "Play redraw: as much layout work per frame as fits ~14ms budget"}
              onClick={() => setBoardRedrawPlaying(!boardRedrawPlaying)}
            >
              {boardRedrawPlaying ? <Pause className="size-4" aria-hidden /> : <Play className="size-4" aria-hidden />}
            </button>
          </ToolbarItem>
          <ToolbarItem>
            <button type="button" className={boardToolbarToggleClass(false)} title="Redraw handles: anchors on the straight segment between node centers" onClick={() => applyBoardRedrawHandlesOnce()}>
              <Link2 className="size-4" aria-hidden />
            </button>
          </ToolbarItem>
        </ToolbarGroup>
      </ToolbarZone>
    </div>
  );
}
// #endregion 🔖Toolbar

// #region 🔖SettingsPanel
/** @emoji ⚙️ Board play redraw settings: play uses requestAnimationFrame (packed WASM per frame), progressive ramp, and per-mode layout parameters. */
function BoardPlaySettingsPanel(): ReactElement {
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
  } = useBoardPlayShell();

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
            <SelectTrigger className="h-8 w-full" id="board-play-redraw-mode" size="sm">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="force-graph">Graph</SelectItem>
              <SelectItem value="hierarchical-tree">Tree</SelectItem>
            </SelectContent>
          </Select>
        </Label>
        <div className="flex items-center gap-2">
          <input checked={boardRedrawHandlesAfterNodes} className="accent-accent size-3.5 shrink-0" id="board-play-redraw-handles-after-nodes" onChange={(e) => setBoardRedrawHandlesAfterNodes(e.target.checked)} type="checkbox" />
          <label className="text-muted-foreground cursor-pointer select-none text-[11px] leading-snug" htmlFor="board-play-redraw-handles-after-nodes">
            Also redraw handles after node redraw
          </label>
        </div>
        <div className="flex items-center gap-2">
          <input checked={boardRedrawProgressiveEnabled} className="accent-accent size-3.5 shrink-0" id="board-play-redraw-progressive" onChange={(e) => setBoardRedrawProgressiveEnabled(e.target.checked)} type="checkbox" />
          <label className="text-muted-foreground cursor-pointer select-none text-[11px] leading-snug" htmlFor="board-play-redraw-progressive">
            Progressive iterations while play is on (graph ramps up; tree still one pass per frame)
          </label>
        </div>
        <Label id="board.play.settings.redraw.autoStopMs" label="Auto-stop play after (ms, 0 = off)">
          <Slider id="board-play-slider-redraw-autostop" max={12000} min={0} step={250} value={[boardRedrawProgressiveAutoStopMs]} onValueChange={(vals) => setBoardRedrawProgressiveAutoStopMs(vals[0] ?? 3000)} />
        </Label>
        {boardRedrawMode === "force-graph" ? (
          <Label id="board.play.settings.redraw.playMaxIters" label="Max iterations per WASM call (play ramp ceiling)">
            <Slider id="board-play-slider-redraw-play-max-iters" max={220} min={12} step={2} value={[boardRedrawPlayMaxItersPerFrame]} onValueChange={(vals) => setBoardRedrawPlayMaxItersPerFrame(vals[0] ?? 96)} />
          </Label>
        ) : (
          <p className="text-muted-foreground text-[11px] leading-snug">Tree redraw runs once per animation frame while play is on; use auto-stop to end play after a duration.</p>
        )}
        {boardRedrawMode === "force-graph" ? (
          <>
            <div className="text-muted-foreground pt-1 text-[11px] font-medium uppercase tracking-wide">Graph</div>
            <Label id="board.play.settings.force.fullIterations" label="Iterations (apply once)">
              <Slider id="board-play-slider-force-full-iters" max={720} min={24} step={4} value={[forceLayoutFullIterations]} onValueChange={(vals) => setForceLayoutFullIterations(vals[0] ?? 200)} />
            </Label>
            <Label id="board.play.settings.force.idealEdge" label="Ideal edge (px)">
              <Slider id="board-play-slider-force-ideal" max={160} min={20} step={2} value={[forceLayoutIdealEdgeLength]} onValueChange={(vals) => setForceLayoutIdealEdgeLength(vals[0] ?? 64)} />
            </Label>
            <Label id="board.play.settings.force.repulsion" label="Repulsion (medium 80, ±40)">
              <Slider id="board-play-slider-force-repulsion" max={120} min={40} step={2} value={[forceLayoutRepulsionStrength]} onValueChange={(vals) => setForceLayoutRepulsionStrength(vals[0] ?? 80)} />
            </Label>
            <Label id="board.play.settings.force.gravity" label="Gravity">
              <Slider id="board-play-slider-force-gravity" max={0.05} min={0} step={0.002} value={[forceLayoutGravity]} onValueChange={(vals) => setForceLayoutGravity(vals[0] ?? 0)} />
            </Label>
          </>
        ) : (
          <>
            <div className="text-muted-foreground pt-1 text-[11px] font-medium uppercase tracking-wide">Tree</div>
            <Label id="board.play.settings.tree.layerSpacing" label="Layer spacing (px)">
              <Slider id="board-play-slider-tree-layer" max={280} min={40} step={4} value={[treeLayoutLayerSpacing]} onValueChange={(vals) => setTreeLayoutLayerSpacing(vals[0] ?? 120)} />
            </Label>
            <Label id="board.play.settings.tree.siblingGap" label="Sibling gap (px)">
              <Slider id="board-play-slider-tree-sibling" max={120} min={0} step={2} value={[treeLayoutSiblingGap]} onValueChange={(vals) => setTreeLayoutSiblingGap(vals[0] ?? 28)} />
            </Label>
            <Label id="board.play.settings.tree.direction" label="Direction">
              <Select onValueChange={(v) => setTreeLayoutDirection(v as BoardHierarchicalTreeDirectionKind)} value={treeLayoutDirection}>
                <SelectTrigger className="h-8 w-full" id="board-play-tree-direction" size="sm">
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
        <Button className="h-8 w-full text-xs" type="button" variant="secondary" onClick={applyBoardRedrawOnce}>
          Redraw nodes
        </Button>
        <div className="text-muted-foreground border-t border-element pt-2 text-[11px] font-medium uppercase tracking-wide">Redraw handles</div>
        <p className="text-muted-foreground text-[11px] leading-snug">Each edge uses the straight segment between node centers; handle anchors move to where that segment meets each shape (shortest chord through the bodies).</p>
        <Button className="h-8 w-full text-xs" type="button" variant="secondary" onClick={applyBoardRedrawHandlesOnce}>
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
/** @emoji 🗼 Marker tree for {@link BoardCanvas} — must stay a Fragment of {@link Node}/{@link Edge} so {@link buildBoardSceneDescriptor} sees markers (custom wrappers are opaque to the static walk). */
function nakaginBoardMarkers(fixture: BoardFixtureV1): ReactElement {
  const demoNodeId = fixture.nodes[0]?.id;
  const demoEdgeId = fixture.edges[0]?.id;
  return (
    <>
      {fixture.nodes.map((node) =>
        node.shape === "rectangle" ? (
          <Node
            contextMenu={node.id === demoNodeId ? boardPlayDemoNodeContextMenu : undefined}
            draggable
            height={node.height}
            id={node.id}
            key={node.id}
            {...(node.nodeKind !== undefined ? { nodeKind: node.nodeKind } : {})}
            shape="rectangle"
            text={boardFixtureNodeCaption(node)}
            textAlignment={node.textAlignment}
            textAutofit={node.textAutofit === true}
            textFontFamily={node.textFontFamily}
            textFontSize={node.textFontSize}
            width={node.width}
            x={node.x}
            y={node.y}
            {...(node.iconKind ? { iconKind: node.iconKind } : {})}
          >
            {node.handles.map((handle) => (
              <Handle angle={handle.angle} color={handle.color} handleKind={handle.handleKind} id={handle.id} key={handle.id} radius={handle.radius} {...(handle.iconKind ? { iconKind: handle.iconKind } : {})} />
            ))}
          </Node>
        ) : (
          <Node
            contextMenu={node.id === demoNodeId ? boardPlayDemoNodeContextMenu : undefined}
            draggable
            id={node.id}
            key={node.id}
            {...(node.nodeKind !== undefined ? { nodeKind: node.nodeKind } : {})}
            radius={node.radius}
            text={boardFixtureNodeCaption(node)}
            textAlignment={node.textAlignment}
            textAutofit={node.textAutofit === true}
            textFontFamily={node.textFontFamily}
            textFontSize={node.textFontSize}
            x={node.x}
            y={node.y}
            {...(node.iconKind ? { iconKind: node.iconKind } : {})}
          >
            {node.handles.map((handle) => (
              <Handle angle={handle.angle} color={handle.color} handleKind={handle.handleKind} id={handle.id} key={handle.id} radius={handle.radius} {...(handle.iconKind ? { iconKind: handle.iconKind } : {})} />
            ))}
          </Node>
        ),
      )}
      {fixture.edges.map((edge) => (
        <Edge contextMenu={edge.id === demoEdgeId ? boardPlayDemoEdgeContextMenu : undefined} id={edge.id} key={edge.id} source={edge.source} target={edge.target} />
      ))}
    </>
  );
}

/** @emoji 🗑️ Keeps the shared shell fixture aligned with canvas `edgeDelete` / `nodeDelete` events. */
function BoardStructuralDeleteReporter(): null {
  const { applyStructuralDelete } = useBoardPlayShell();
  const onEdgeDelete = useCallback(
    (event: { id: string }) => {
      applyStructuralDelete("edge", event.id);
    },
    [applyStructuralDelete],
  );
  const onNodeDelete = useCallback(
    (event: { id: string }) => {
      applyStructuralDelete("node", event.id);
    },
    [applyStructuralDelete],
  );
  useBoardEvent("edgeDelete", onEdgeDelete);
  useBoardEvent("nodeDelete", onNodeDelete);
  return null;
}

/** @emoji 🔁 While play is on, each user `nodeMove` restarts the progressive graph ramp and auto-stop clock. */
function BoardPlayRedrawProgressReset(): null {
  const { boardRedrawPlaying, resetBoardRedrawProgressiveEpoch } = useBoardPlayShell();
  const handler = useCallback(() => {
    if (!boardRedrawPlaying) {
      return;
    }
    resetBoardRedrawProgressiveEpoch();
  }, [boardRedrawPlaying, resetBoardRedrawProgressiveEpoch]);
  useBoardEvent("nodeMove", handler);
  return null;
}
// #endregion 🔖Scene

// #region 🔖Panes
/** @emoji 🪟 Captures pointer focus for the active pane (tabs + canvas). */
function BoardPaneChrome({ children, paneId }: { children: ReactNode; paneId: BoardPlayPaneId }): ReactElement {
  const { clearHoverForPane, setActivePaneId, setHoverPane } = useBoardPlayShell();
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

function BoardPlayPaneCanvas({ paneId, showBackgroundMenu }: { paneId: BoardPlayPaneId; showBackgroundMenu?: boolean }): ReactElement {
  const {
    activePaneId,
    boardGridSnapEnabled,
    boardLodModeByPane,
    boardSelectionMethod,
    boardSelectionMode,
    boardSelectionTargets,
    fixture,
    handleCanvasFixtureDrop,
    camerasByPane,
    hoveredId,
    preselection,
    selectionIds,
    setHoverForPane,
    setPreselection,
    setSelectionIds,
    syncBaselineFromViewportCamera,
  } = useBoardPlayShell();
  const camera = camerasByPane[paneId];
  const lodProps = boardPlayLodCanvasProps(boardLodModeByPane[paneId]);
  const reportEffectiveLod = useContext(BoardPlayLodRuntimeContext);
  const onLodChange = useCallback((lod: BoardDrawLodKind) => reportEffectiveLod?.(paneId, lod), [paneId, reportEffectiveLod]);
  const selection = useMemo(() => normalizeBoardSelectionProp([...selectionIds]), [selectionIds]);
  const onSelect = useCallback((snapshot: BoardSelectionSnapshot) => setSelectionIds(snapshot.ids), [setSelectionIds]);
  const onPreselect = useCallback((snapshot: BoardPreselectSnapshot) => setPreselection(snapshot), [setPreselection]);
  const onHover = useCallback(
    (payload: { id: string | null }) => {
      setHoverForPane(paneId, payload.id);
    },
    [paneId, setHoverForPane],
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
        gridSnapEnabled={boardGridSnapEnabled}
        hoveredId={hoveredId}
        kindCatalogs={NAKAGIN_BOARD_PLAY_KIND_CATALOGS}
        lodZoomThresholds={DEFAULT_BOARD_LOD_ZOOM_THRESHOLDS}
        onCamera={activePaneId === paneId ? syncBaselineFromViewportCamera : undefined}
        onFixtureDrop={(d) => handleCanvasFixtureDrop(paneId, d)}
        onHover={onHover}
        onPreselect={onPreselect}
        onSelect={onSelect}
        preselection={preselection}
        selection={selection}
        selectionMethod={boardSelectionMethod}
        selectionMode={boardSelectionMode}
        selectionTargets={boardSelectionTargets}
      >
        <BoardStructuralDeleteReporter />
        <BoardPlayRedrawProgressReset />
        {nakaginBoardMarkers(fixture)}
      </BoardCanvas>
    </BoardPaneChrome>
  );
}

function BoardOverviewPane(): ReactElement {
  return <BoardPlayPaneCanvas paneId="board-overview" showBackgroundMenu />;
}

function BoardDetailPane(): ReactElement {
  return <BoardPlayPaneCanvas paneId="board-detail" />;
}

function BoardSelectionPane(): ReactElement {
  return <BoardPlayPaneCanvas paneId="board-selection" />;
}
// #endregion 🔖Panes

// #region 🔖SidePanels
// #region 🔖PaletteFixtureShelf
/** @emoji 📐 Palette seeds match {@link BOARD_PLAY_DEFAULT_NODE_SIZE_PX} (circle radius = span/2). */

const BOARD_PLAY_PALETTE_CIRCLE_DRAG_FIXTURE: BoardFixtureV1 =
  parseBoardFixtureV1({
    camera: { x: 0, y: 0, zoom: 1 },
    edges: [],
    meta: { boardFixtureDragKind: BOARD_FIXTURE_DRAG_KIND_PALETTE_NODE },
    nodes: [{ handles: [{ angle: 0, id: "palette-seed-circle.h0" }], id: "palette-seed-circle", radius: BOARD_PLAY_DEFAULT_NODE_SIZE_PX / 2, x: 0, y: 0 }],
    schema: "elements.board.fixture/v1",
  }) ??
  (() => {
    throw new Error("Board play: palette circle drag fixture failed validation.");
  })();

const BOARD_PLAY_PALETTE_RECTANGLE_DRAG_FIXTURE: BoardFixtureV1 =
  parseBoardFixtureV1({
    camera: { x: 0, y: 0, zoom: 1 },
    edges: [],
    meta: { boardFixtureDragKind: BOARD_FIXTURE_DRAG_KIND_PALETTE_NODE },
    nodes: [
      {
        handles: [{ angle: 0, id: "palette-seed-rectangle.h0" }],
        height: BOARD_PLAY_DEFAULT_NODE_SIZE_PX,
        id: "palette-seed-rectangle",
        shape: "rectangle",
        width: BOARD_PLAY_DEFAULT_NODE_SIZE_PX,
        x: 0,
        y: 0,
      },
    ],
    schema: "elements.board.fixture/v1",
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
  const dragController = useMemo(
    () =>
      new NativeDragAndDropController<HTMLDivElement>({
        onDragStart: (event) => {
          event.dataTransfer.setData(BOARD_FIXTURE_DRAG_V1_MIME, encodeBoardFixtureForDragV1(dragFixture));
          event.dataTransfer.effectAllowed = "copy";
          const { clientHeight, clientWidth } = event.currentTarget;
          event.dataTransfer.setDragImage(event.currentTarget, clientWidth / 2, clientHeight / 2);
        },
      }),
    [dragFixture],
  );
  return (
    <div className="border-element bg-background flex h-10 w-10 shrink-0 cursor-grab items-center justify-center rounded-lg border active:cursor-grabbing" title={label} {...dragController.getProps()}>
      {preview}
    </div>
  );
}
// #endregion 🔖PaletteFixtureShelf

/** @emoji 📥 Left rail: drag the active graph onto a board pane (in-app MIME payload, not filesystem JSON files). */
function BoardFixtureLibraryPanel(): ReactElement {
  const { fixture } = useBoardPlayShell();

  const shelfDragController = useMemo(
    () =>
      new NativeDragAndDropController<HTMLDivElement>({
        onDragStart: (event) => {
          event.dataTransfer.setData(BOARD_FIXTURE_DRAG_V1_MIME, encodeBoardFixtureForDragV1(fixture));
          event.dataTransfer.effectAllowed = "copy";
        },
      }),
    [fixture],
  );

  return (
    <div className="flex h-full min-h-0 flex-col gap-3 p-3 text-sm">
      <div className="text-muted-foreground text-xs uppercase tracking-wide" data-testid="board-play-fixture-shelf">
        Fixture shelf
      </div>
      <div className="flex flex-col gap-2">
        <div className="text-muted-foreground text-[11px] uppercase tracking-wide">Shapes</div>
        <div className="flex flex-wrap gap-2">
          <BoardFixturePaletteDraggable fixture={BOARD_PLAY_PALETTE_CIRCLE_DRAG_FIXTURE} label="Drag circle onto the board" preview={<div className="border-primary size-10 shrink-0 rounded-full border-2 bg-accent/30" />} />
          <BoardFixturePaletteDraggable fixture={BOARD_PLAY_PALETTE_RECTANGLE_DRAG_FIXTURE} label="Drag rectangle onto the board" preview={<div className="border-primary size-10 shrink-0 rounded-sm border-2 bg-accent/30" />} />
        </div>
      </div>
      <div className="border-element bg-muted/30 flex min-h-30 cursor-grab flex-col justify-center gap-2 rounded-md border p-4 active:cursor-grabbing" {...shelfDragController.getProps()}>
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
  const ref = useRef<HTMLDivElement | null>(null);

  const setFromClient = useCallback(
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

  const pointerController = useMemo(
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
  const idSet = useMemo(() => new Set(nodeIds), [nodeIds]);
  const targets = useMemo(() => nodeIds.map((id) => findNode(fixture, id)).filter((n): n is BoardFixtureNodeV1 => Boolean(n)), [fixture, nodeIds]);

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

  const patchNodes = useCallback(
    (updater: (n: BoardFixtureNodeV1) => BoardFixtureNodeV1) => {
      patchFixture((prev) => ({
        ...prev,
        nodes: prev.nodes.map((n) => (idSet.has(n.id) ? updater(n) : n)),
      }));
    },
    [idSet, patchFixture],
  );

  const onText = useCallback(
    (next: string) => {
      const trimmed = next.trim();
      patchNodes((n) =>
        trimmed === "" ? { ...n, text: undefined } : { ...n, text: trimmed },
      );
    },
    [patchNodes],
  );

  const onIconKind = useCallback(
    (next: string) => {
      const t = next.trim();
      patchNodes((n) => ({ ...n, ...(t === "" ? { iconKind: undefined } : { iconKind: t }) }));
    },
    [patchNodes],
  );

  const onShape = useCallback(
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
        <Label id="board-play.inspector.node.id" label="Id">
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
      <Label id="board-play.inspector.node.name" label="Name">
        <Input className="h-7 font-mono text-xs" onChange={(e: ChangeEvent<HTMLInputElement>) => onText(e.target.value)} placeholder={textUniform ? undefined : "Mixed"} value={textValue} />
      </Label>
      <Label id="board-play.inspector.node.icon" label="Icon">
        <IconSelector classifyElementsBoardIconSelectorMode={classifyElementsBoardIconSelectorMode} id="board-play.inspector.node.icon.selector" onChange={onIconKind} uniform={iconKindUniform} value={iconKindValue} />
      </Label>
      <Label id="board-play.inspector.node.shape" label="Shape">
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
      <NumericStepperRow id="board-play.inspector.node.x" label="x" onAbsolute={(v) => patchNodes((n) => ({ ...n, x: v }))} onDelta={(d) => patchNodes((n) => ({ ...n, x: n.x + d }))} step={1} uniform={xUniform} value={xValue} />
      <NumericStepperRow id="board-play.inspector.node.y" label="y" onAbsolute={(v) => patchNodes((n) => ({ ...n, y: v }))} onDelta={(d) => patchNodes((n) => ({ ...n, y: n.y + d }))} step={1} uniform={yUniform} value={yValue} />
      {targets.some((n) => !nodeIsRectangle(n)) ? (
        <NumericStepperRow
          id="board-play.inspector.node.r"
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
            id="board-play.inspector.node.w"
            label="width"
            onAbsolute={(v) => patchNodes((n) => (nodeIsRectangle(n) ? { ...n, width: Math.max(1e-6, v) } : n))}
            onDelta={(d) => patchNodes((n) => (nodeIsRectangle(n) ? { ...n, width: Math.max(1e-6, n.width + d) } : n))}
            step={1}
            uniform={wUniform}
            value={wValue}
          />
          <NumericStepperRow
            id="board-play.inspector.node.h"
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
  const idSet = useMemo(() => new Set(handleIds), [handleIds]);
  const handles = useMemo(() => handleIds.map((id) => findHandle(fixture, id)).filter((h): h is BoardFixtureHandleV1 => Boolean(h)), [fixture, handleIds]);
  const angles = handles.map((h) => h.angle);
  const angleUniform = allEqual(angles);
  const angleValue = angleUniform ? angles[0]! : 0;
  const radii = handles.map((h) => h.radius ?? 8);
  const radiusUniform = allEqual(radii);
  const radiusValue = radiusUniform ? radii[0]! : Number.NaN;

  const iconKinds = handles.map((h) => h.iconKind ?? "");
  const iconKindUniform = allEqual(iconKinds);
  const iconKindValue = iconKindUniform ? (iconKinds[0] ?? "") : "";

  const patchHandles = useCallback(
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

  const onIconKind = useCallback(
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
            id="board-play.inspector.handle.t"
            label="t (rad)"
            onAbsolute={(v) => patchHandles((h) => ({ ...h, angle: normalizeAngleRad(v) }))}
            onDelta={(d) => patchHandles((h) => ({ ...h, angle: normalizeAngleRad(h.angle + d) }))}
            step={0.05}
            uniform={angleUniform}
            value={angleUniform ? angleValue : Number.NaN}
          />
          <NumericStepperRow
            id="board-play.inspector.handle.radius"
            label="Hit radius"
            onAbsolute={(v) => patchHandles((h) => ({ ...h, radius: Math.max(1e-6, v) }))}
            onDelta={(d) => patchHandles((h) => ({ ...h, radius: Math.max(1e-6, (h.radius ?? 8) + d) }))}
            step={1}
            uniform={radiusUniform}
            value={radiusValue}
          />
          <Label id="board-play.inspector.handle.icon" label="Icon">
            <IconSelector classifyElementsBoardIconSelectorMode={classifyElementsBoardIconSelectorMode} id="board-play.inspector.handle.icon.selector" onChange={onIconKind} uniform={iconKindUniform} value={iconKindValue} />
          </Label>
          {handleIds.length === 1 ? (
            <Label id="board-play.inspector.handle.id" label="Id">
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
  const idSet = useMemo(() => new Set(edgeIds), [edgeIds]);
  const edges = useMemo(() => edgeIds.map((id) => findEdge(fixture, id)).filter((e): e is BoardFixtureEdgeV1 => Boolean(e)), [edgeIds, fixture]);
  const sources = edges.map((e) => e.source);
  const targets = edges.map((e) => e.target);
  const sourceUniform = allEqual(sources);
  const targetUniform = allEqual(targets);
  const handleOptions = useMemo(() => listHandleIds(fixture), [fixture]);

  const patchEdges = useCallback(
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
      <Label id="board-play.inspector.edge.source" label="Source">
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
      <Label id="board-play.inspector.edge.target" label="Target">
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
        <Label id="board-play.inspector.edge.id" label="Id">
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

/** @emoji 🔎 Sketchpad-style tree inspector sections for the active pane selection. */
function createBoardSelectionInspectorSections(): TreeDataSection[] {
  const { activePaneId, fixture, patchFixture, remapIdInSelections, selectionIds } = useBoardPlayShell();
  const ids = useMemo(() => [...selectionIds].sort((a, b) => a.localeCompare(b)), [selectionIds]);

  const { edgeIds, handleIds, nodeIds } = useMemo(() => {
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
    return { edgeIds, handleIds, nodeIds };
  }, [fixture, ids]);

  return useMemo<TreeDataSection[]>(() => {
    if (ids.length === 0) {
      return [
        {
          content: (
            <div className="text-muted-foreground flex shrink-0 items-center gap-2 border-b border-element px-1 py-2">
              <ClipboardList className="size-4 shrink-0" />
              <div>
                <div className="font-semibold uppercase tracking-wide">Detail</div>
                <div className="text-[11px] opacity-80">pane: {activePaneId}</div>
              </div>
            </div>
          ),
          id: "board-play-inspector-header.empty",
        },
        {
          content: <p className="text-muted-foreground px-1 py-2 text-xs">No selection. Click the graph or pick another tab.</p>,
          id: "board-play-inspector-empty",
          label: null,
        },
      ];
    }
    const sections: TreeDataSection[] = [];
    sections.push({
      content: (
        <div className="text-muted-foreground flex shrink-0 items-center gap-2 border-b border-element px-1 py-2">
          <ClipboardList className="size-4 shrink-0" />
          <div>
            <div className="font-semibold uppercase tracking-wide">Detail</div>
            <div className="text-[11px] opacity-80">pane: {activePaneId}</div>
          </div>
        </div>
      ),
      id: "board-play-inspector-header",
    });
    if (nodeIds.length > 0) {
      sections.push({
        content: <InspectorNodeBatch fixture={fixture} nodeIds={nodeIds} patchFixture={patchFixture} remapIdInSelections={remapIdInSelections} />,
        defaultOpen: true,
        id: "board-play-inspector-nodes",
        label: `Nodes (${nodeIds.length})`,
      });
    }
    if (handleIds.length > 0) {
      sections.push({
        content: <InspectorHandleBatch fixture={fixture} handleIds={handleIds} patchFixture={patchFixture} remapIdInSelections={remapIdInSelections} />,
        defaultOpen: true,
        id: "board-play-inspector-handles",
        label: `Handles (${handleIds.length})`,
      });
    }
    if (edgeIds.length > 0) {
      sections.push({
        content: <InspectorEdgeBatch edgeIds={edgeIds} fixture={fixture} patchFixture={patchFixture} remapIdInSelections={remapIdInSelections} />,
        defaultOpen: true,
        id: "board-play-inspector-edges",
        label: `Edges (${edgeIds.length})`,
      });
    }
    if (sections.length === 0) {
      sections.push({
        content: (
          <div className="px-1 py-2 font-mono text-xs" style={{ color: "var(--warning-foreground)" }}>
            Unknown ids: {ids.join(", ")}
          </div>
        ),
        id: "board-play-inspector-unknown",
        label: "Selection",
      });
    }
    return sections;
  }, [edgeIds, fixture, handleIds, ids, nodeIds, patchFixture, remapIdInSelections]);
}
// #endregion 🔖SidePanels

// #region 🔖Layout
const boardPlayLayout: UIWindowLayout = {
  root: {
    kind: "row",
    children: [
      {
        kind: "stack",
        size: 50,
        children: [createWindowLayout("board-overview", "Overview")],
      },
      {
        kind: "column",
        size: 50,
        children: [
          { kind: "stack", size: 50, children: [createWindowLayout("board-detail", "Zoom")] },
          { kind: "stack", size: 50, children: [createWindowLayout("board-selection", "Selection")] },
        ],
      },
	],
  },
};

const BOARD_PLAY_SHELL_CONTROLLER_ID = "board-play";

class BoardPlayShellController extends Controller {
	constructor(commandBus: CommandBus, hostNotify: () => void) {
		super(BOARD_PLAY_SHELL_CONTROLLER_ID, commandBus, hostNotify);
	}

	override run(_command: string, _args?: unknown): void {}
}

function buildBoardPlayWorkbenchApp(controller: BoardPlayShellController): WorkbenchApp {
	return new WorkbenchApp(
		BOARD_PLAY_APP_ID,
		"Board",
		undefined,
		controller,
		boardPlayLayout as never,
		[
			new WorkbenchWindowKind("board-overview", "Overview", "elements.board.placeholder"),
			new WorkbenchWindowKind("board-detail", "Zoom", "elements.board.placeholder"),
			new WorkbenchWindowKind("board-selection", "Selection", "elements.board.placeholder"),
		],
	);
}

const BOARD_PLAY_LOD_TIERS: BoardDrawLodKind[] = ["minimap", "overview", "compact", "normal", "detail", "micro"];

function boardPlayLodTierMenuLabel(tier: BoardDrawLodKind): string {
  return tier.charAt(0).toUpperCase() + tier.slice(1);
}

function boardWindowKindsWithLodMeasures(
  lodModeByPane: Record<BoardPlayPaneId, BoardLodModeKind>,
  effectiveLodByPane: Record<BoardPlayPaneId, BoardDrawLodKind>,
  setLodModeForPane: (pane: BoardPlayPaneId, mode: BoardLodModeKind) => void,
): UIWindowKindDefinition[] {
  const lodMeasure = (paneId: BoardPlayPaneId): UIWindowKindDefinition["measures"] => [
    {
      id: `${paneId}-lod`,
      items: [
        {
          id: "automatic",
          label: boardLodAutomaticSelectLabel(effectiveLodByPane[paneId]),
          value: BOARD_LOD_MODE_AUTOMATIC,
        },
        ...BOARD_PLAY_LOD_TIERS.map((tier) => ({
          id: tier,
          label: boardPlayLodTierMenuLabel(tier),
          value: tier,
        })),
      ],
      kind: "select",
      label: "LOD",
      onValueChange: (value) => {
        if (value === BOARD_LOD_MODE_AUTOMATIC || isBoardDrawLodKind(value)) {
          setLodModeForPane(paneId, value as BoardLodModeKind);
        }
      },
      value: lodModeByPane[paneId],
    },
  ];
  return [
    {
      component: BoardOverviewPane,
      contextMenu: boardPlayOverviewWindowContextMenu,
      id: "board-overview",
      label: "Overview",
      measures: lodMeasure("board-overview"),
    },
    { component: BoardDetailPane, id: "board-detail", label: "Zoom", measures: lodMeasure("board-detail") },
    { component: BoardSelectionPane, id: "board-selection", label: "Selection", measures: lodMeasure("board-selection") },
  ];
}

// #endregion 🔖Layout

// #region 🔖Surface
function BoardPlaySurfaceFooter(props: {
  theme: ElementsSurfaceTheme;
  device: ElementsSurfaceDevice;
  expertise: Expertise;
  onTheme: (v: ElementsSurfaceTheme) => void;
  onDevice: (v: ElementsSurfaceDevice) => void;
  onExpertise: (v: Expertise) => void;
}): ReactElement {
  const { theme, device, expertise, onDevice, onExpertise, onTheme } = props;
  return (
    <div className="flex min-w-0 flex-wrap items-center gap-double px-single py-tiny">
      <span className="shrink-0 text-xs text-muted-foreground">Theme</span>
      <Select onValueChange={(v) => onTheme(v as ElementsSurfaceTheme)} value={theme}>
        <SelectTrigger className="h-medium w-30" id="board-play-surface-theme" size="sm">
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="system">System</SelectItem>
          <SelectItem value="light">Light</SelectItem>
          <SelectItem value="dark">Dark</SelectItem>
        </SelectContent>
      </Select>
      <span className="shrink-0 text-xs text-muted-foreground">Device</span>
      <Select onValueChange={(v) => onDevice(v as ElementsSurfaceDevice)} value={device}>
        <SelectTrigger className="h-medium w-30" id="board-play-surface-device" size="sm">
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="desktop">Desktop</SelectItem>
          <SelectItem value="tablet">Tablet</SelectItem>
          <SelectItem value="mobile">Mobile</SelectItem>
        </SelectContent>
      </Select>
      <span className="shrink-0 text-xs text-muted-foreground">Expertise</span>
      <Select onValueChange={(v) => onExpertise(v as Expertise)} value={expertise}>
        <SelectTrigger className="h-medium w-30" id="board-play-surface-expertise" size="sm">
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value={Expertise.BEGINNER}>Beginner</SelectItem>
          <SelectItem value={Expertise.NORMAL}>Normal</SelectItem>
          <SelectItem value={Expertise.EXPERT}>Expert</SelectItem>
        </SelectContent>
      </Select>
    </div>
  );
}
// #endregion 🔖Surface

interface BoardPlayRedrawLoopSnapshot {
  activePaneId: BoardPlayPaneId;
  boardRedrawHandlesAfterNodes: boolean;
  boardRedrawProgressiveAutoStopMs: number;
  boardRedrawProgressiveEnabled: boolean;
  boardRedrawPlayMaxItersPerFrame: number;
  camerasByPane: Record<BoardPlayPaneId, CameraState>;
  forceLayoutGravity: number;
  forceLayoutIdealEdgeLength: number;
  forceLayoutRepulsionStrength: number;
  mode: BoardRedrawModeKind;
  treeLayoutDirection: BoardHierarchicalTreeDirectionKind;
  treeLayoutLayerSpacing: number;
  treeLayoutSiblingGap: number;
}

// #region 🔖Entrypoint
const initialFixture = parseBoardFixtureV1(nakaginFixtureJson as unknown) ?? (nakaginFixtureJson as BoardFixtureV1);

function BoardPlayInner(): ReactElement {
  const [fixture, setFixtureState] = useState<BoardFixtureV1>(initialFixture);
  const fixtureRef = useRef<BoardFixtureV1>(fixture);
  fixtureRef.current = fixture;
  const [boardPlayPaneCamerasBaseline, setBoardPlayPaneCamerasBaseline] = useState<Record<BoardPlayPaneId, CameraState>>(() => triptychCamerasFromFixture(initialFixture));
  const boardPlayPaneCamerasBaselineRef = useRef(boardPlayPaneCamerasBaseline);
  boardPlayPaneCamerasBaselineRef.current = boardPlayPaneCamerasBaseline;
  const [activePaneId, setActivePaneId] = useState<BoardPlayPaneId>("board-overview");
  const activePaneIdRef = useRef(activePaneId);
  activePaneIdRef.current = activePaneId;
  const [selectionIds, setSelectionIdsState] = useState<Set<string>>(() => selectionSeedForFixture(initialFixture));
  const [preselection, setPreselection] = useState<BoardPreselectSnapshot>(BOARD_PRESELECT_EMPTY);
  const [hoveredId, setHoveredId] = useState<string | null>(null);
  const [hoverSourcePane, setHoverSourcePane] = useState<BoardPlayPaneId | null>(null);
  const hoverSourcePaneRef = useRef<BoardPlayPaneId | null>(hoverSourcePane);
  hoverSourcePaneRef.current = hoverSourcePane;
  const [theme, setTheme] = useState<ElementsSurfaceTheme>(readTheme);
  const [device, setDevice] = useState<ElementsSurfaceDevice>(readDevice);
  const [expertise, setExpertise] = useState<Expertise>(readExpertise);
  const { mobile } = useElementsSurfaceChrome({ theme, device, expertise });
  const [boardSelectionMethod, setBoardSelectionMethod] = useState<BoardSelectionMethod>("rectangle");
  const [boardSelectionMode, setBoardSelectionMode] = useState<BoardSelectionMode>("default");
  const [boardSelectionTargets, setBoardSelectionTargets] = useState<BoardSelectionTargets>(() => ({ ...BOARD_SELECTION_TARGETS_DEFAULT }));
  const [boardGridSnapEnabled, setBoardGridSnapEnabled] = useState(false);
  const [boardLodModeByPane, setBoardLodModeByPane] = useState<Record<BoardPlayPaneId, BoardLodModeKind>>({
    "board-detail": BOARD_LOD_MODE_AUTOMATIC,
    "board-overview": BOARD_LOD_MODE_AUTOMATIC,
    "board-selection": BOARD_LOD_MODE_AUTOMATIC,
  });
  const setBoardLodModeForPane = useCallback((pane: BoardPlayPaneId, mode: BoardLodModeKind) => {
    setBoardLodModeByPane((prev) => ({ ...prev, [pane]: mode }));
  }, []);
  const [boardEffectiveLodByPane, setBoardEffectiveLodByPane] = useState<Record<BoardPlayPaneId, BoardDrawLodKind>>({
    "board-detail": "normal",
    "board-overview": "normal",
    "board-selection": "normal",
  });
  const setBoardEffectiveLodForPane = useCallback((pane: BoardPlayPaneId, lod: BoardDrawLodKind) => {
    setBoardEffectiveLodByPane((prev) => (prev[pane] === lod ? prev : { ...prev, [pane]: lod }));
  }, []);
  const onBoardPlayActiveWindowChange = useCallback((windowKindId: string) => {
    if (windowKindId === "board-overview" || windowKindId === "board-detail" || windowKindId === "board-selection") {
      setActivePaneId(windowKindId);
    }
  }, []);
  const [boardRedrawPlaying, setBoardRedrawPlaying] = useState(false);
  const [forceLayoutFullIterations, setForceLayoutFullIterations] = useState(200);
  const [forceLayoutIdealEdgeLength, setForceLayoutIdealEdgeLength] = useState(64);
  const [forceLayoutGravity, setForceLayoutGravity] = useState(0.012);
  const [forceLayoutRepulsionStrength, setForceLayoutRepulsionStrength] = useState(80);
  const [boardRedrawPlayMaxItersPerFrame, setBoardRedrawPlayMaxItersPerFrame] = useState(96);
  const [boardRedrawProgressiveEnabled, setBoardRedrawProgressiveEnabled] = useState(true);
  const [boardRedrawProgressiveAutoStopMs, setBoardRedrawProgressiveAutoStopMs] = useState(3000);
  const [boardRedrawMode, setBoardRedrawMode] = useState<BoardRedrawModeKind>("force-graph");
  const [boardRedrawHandlesAfterNodes, setBoardRedrawHandlesAfterNodes] = useState(false);
  const [treeLayoutLayerSpacing, setTreeLayoutLayerSpacing] = useState(120);
  const [treeLayoutSiblingGap, setTreeLayoutSiblingGap] = useState(28);
  const [treeLayoutDirection, setTreeLayoutDirection] = useState<BoardHierarchicalTreeDirectionKind>("downwards");

  const boardRedrawPlayingRef = useRef(boardRedrawPlaying);
  boardRedrawPlayingRef.current = boardRedrawPlaying;

  useEffect(() => {
    try {
      localStorage.setItem(LS_THEME, theme);
    } catch {
      /* ignore */
    }
  }, [theme]);

  useEffect(() => {
    try {
      localStorage.setItem(LS_DEVICE, device);
    } catch {
      /* ignore */
    }
  }, [device]);

  useEffect(() => {
    try {
      localStorage.setItem(LS_EXPERTISE, expertise);
    } catch {
      /* ignore */
    }
  }, [expertise]);

  const surfaceFooterItems = useMemo<FooterItem[]>(
    () => [
      {
        content: <BoardPlaySurfaceFooter device={device} expertise={expertise} onDevice={setDevice} onExpertise={setExpertise} onTheme={setTheme} theme={theme} />,
        id: "board-play-surface",
        order: 0,
      },
    ],
    [device, expertise, theme],
  );

  const applyStructuralDelete = useCallback((kind: "edge" | "node", id: string) => {
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

  const setFixture = useCallback((next: BoardFixtureV1) => {
    setFixtureState(next);
    setSelectionIdsState(selectionSeedForFixture(next));
    setPreselection(BOARD_PRESELECT_EMPTY);
    setHoveredId(null);
    hoverSourcePaneRef.current = null;
    setHoverSourcePane(null);
    setBoardPlayPaneCamerasBaseline(triptychCamerasFromFixture(next));
  }, []);

  const patchFixture = useCallback((updater: (prev: BoardFixtureV1) => BoardFixtureV1) => {
    setFixtureState((prev) => updater(prev));
  }, []);

  const setSelectionIds = useCallback((ids: readonly string[]) => {
    setSelectionIdsState(new Set(ids));
  }, []);

  const setHoverPane = useCallback((pane: BoardPlayPaneId) => {
    if (hoverSourcePaneRef.current === pane) {
      return;
    }
    hoverSourcePaneRef.current = pane;
    setHoverSourcePane(pane);
  }, []);

  const setHoverForPane = useCallback((pane: BoardPlayPaneId, id: string | null) => {
    hoverSourcePaneRef.current = pane;
    setHoverSourcePane(pane);
    setHoveredId(id);
  }, []);

  const clearHoverForPane = useCallback((pane: BoardPlayPaneId) => {
    if (hoverSourcePaneRef.current !== pane) {
      return;
    }
    hoverSourcePaneRef.current = null;
    setHoverSourcePane(null);
    setHoveredId(null);
  }, []);

  const handleCanvasFixtureDrop = useCallback(
    (pane: BoardPlayPaneId, detail: BoardFixtureDropDetail) => {
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

  const remapIdInSelections = useCallback((replacedId: string, replacementId: string) => {
    if (replacedId === replacementId) {
      return;
    }
    setSelectionIdsState((prev) => new Set([...prev].map((id) => (id === replacedId ? replacementId : id))));
  }, []);

  const cameraBasisFixtureRef = useRef<BoardFixtureV1>(fixture);
  /** @emoji 📌 One-shot: sync {@link cameraBasisFixtureRef} without resetting {@link boardPlayPaneCamerasBaseline} after palette / shelf fixture drop. */
  const skipNextCameraBasisResyncRef = useRef(false);
  const prevBoardRedrawPlayingRef = useRef(false);
  const [cameraDisplayOverrideByPane, setCameraDisplayOverrideByPane] = useState<Record<BoardPlayPaneId, CameraState> | null>(null);
  const cameraDisplayOverrideRef = useRef<Record<BoardPlayPaneId, CameraState> | null>(null);
  cameraDisplayOverrideRef.current = cameraDisplayOverrideByPane;
  const suppressCameraBasisSyncRef = useRef(false);
  const cameraPlayEndAnimRafRef = useRef<number | null>(null);
  const boardPlayNodesRedrawCameraAnimRafRef = useRef<number | null>(null);
  const boardPlayRedrawCameraChaseRef = useRef<Record<BoardPlayPaneId, CameraState> | null>(null);
  const lastPlayingForCameraEaseRef = useRef(false);
  const [nodesRedrawCameraEaseTick, setNodesRedrawCameraEaseTick] = useState(0);
  /** @emoji 📷 Cameras shown on canvases at click time; set before {@link patchFixture} so `from` cannot lag one commit behind the graph. */
  const nodesRedrawEaseFromRef = useRef<Record<BoardPlayPaneId, CameraState> | null>(null);
  /** @emoji 🔢 Bumped on each redraw click / competing camera path so stale RAF ticks never call {@link setBoardPlayPaneCamerasBaseline}. */
  const nodesRedrawEaseGenerationRef = useRef(0);

  const syncBaselineFromViewportCamera = useCallback((cam: CameraState) => {
    if (boardRedrawPlayingRef.current) {
      return;
    }
    if (suppressCameraBasisSyncRef.current) {
      return;
    }
    if (cameraDisplayOverrideRef.current !== null) {
      return;
    }
    const c = { x: cam.x, y: cam.y, zoom: cam.zoom };
    setBoardPlayPaneCamerasBaseline((prev) => {
      const pane = activePaneIdRef.current;
      const p = prev[pane];
      if (Math.abs(p.x - c.x) < 1e-6 && Math.abs(p.y - c.y) < 1e-6 && Math.abs(p.zoom - c.zoom) < 1e-9) {
        return prev;
      }
      return { ...prev, [pane]: { ...c } };
    });
  }, []);

  useEffect(() => {
    if (boardRedrawPlaying) {
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
  }, [fixture, boardRedrawPlaying]);

  useEffect(() => {
    const prevPlaying = prevBoardRedrawPlayingRef.current;
    const playJustStarted = boardRedrawPlaying && !prevPlaying;

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
      const prevCam = boardPlayPaneCamerasBaselineRef.current;
      boardPlayRedrawCameraChaseRef.current = {
        "board-detail": { ...prevCam["board-detail"] },
        "board-overview": { ...prevCam["board-overview"] },
        "board-selection": { ...prevCam["board-selection"] },
      };
    } else if (!suppressCameraBasisSyncRef.current) {
      if (cameraPlayEndAnimRafRef.current != null) {
        cancelAnimationFrame(cameraPlayEndAnimRafRef.current);
        cameraPlayEndAnimRafRef.current = null;
      }
    }
    prevBoardRedrawPlayingRef.current = boardRedrawPlaying;
  }, [boardRedrawPlaying, fixture]);

  useEffect(() => {
    if (!boardRedrawPlaying) {
      boardPlayRedrawCameraChaseRef.current = null;
      return;
    }
    if (suppressCameraBasisSyncRef.current) {
      return;
    }
    const pane = activePaneIdRef.current;
    const target = triptychCamerasFromFixture(fixture);
    setBoardPlayPaneCamerasBaseline((baselinePrev) => {
      const prevChase = boardPlayRedrawCameraChaseRef.current ?? baselinePrev;
      const damped = dampCameraStateLinear(prevChase[pane], target[pane], BOARD_PLAY_REDRAW_CAMERA_CHASE_BLEND);
      const nextChase: Record<BoardPlayPaneId, CameraState> = {
        "board-detail": { ...prevChase["board-detail"] },
        "board-overview": { ...prevChase["board-overview"] },
        "board-selection": { ...prevChase["board-selection"] },
      };
      nextChase[pane] = damped;
      boardPlayRedrawCameraChaseRef.current = nextChase;
      return nextChase;
    });
  }, [boardRedrawPlaying, fixture]);

  useEffect(() => {
    if (boardRedrawPlaying) {
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
    const from: Record<BoardPlayPaneId, CameraState> = {
      "board-detail": { ...boardPlayPaneCamerasBaseline["board-detail"] },
      "board-overview": { ...boardPlayPaneCamerasBaseline["board-overview"] },
      "board-selection": { ...boardPlayPaneCamerasBaseline["board-selection"] },
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

    const total = BOARD_PLAY_CAMERA_POST_REDRAW_TOTAL_MS;
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
          setBoardPlayPaneCamerasBaseline((prev) => ({ ...prev, [p]: { ...fit[p] } }));
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
  }, [boardRedrawPlaying]);

  const camerasByPane = cameraDisplayOverrideByPane ?? boardPlayPaneCamerasBaseline;

  useEffect(() => {
    if (nodesRedrawCameraEaseTick === 0) {
      return;
    }
    if (boardRedrawPlayingRef.current) {
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
    const from: Record<BoardPlayPaneId, CameraState> = {
      "board-detail": { ...fromSnapshot["board-detail"] },
      "board-overview": { ...fromSnapshot["board-overview"] },
      "board-selection": { ...fromSnapshot["board-selection"] },
    };
    const to = triptychCamerasFromFixture(snapshotFixture);
    const nodesRedrawEasePaneId = activePaneIdRef.current;
    const total = BOARD_PLAY_NODES_REDRAW_CAMERA_EASE_TOTAL_MS;
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
        setBoardPlayPaneCamerasBaseline(endCameras);
        boardPlayNodesRedrawCameraAnimRafRef.current = null;
        nodesRedrawEaseFromRef.current = null;
        return;
      }
      if (elapsed >= holdEnd) {
        const u = Math.min(1, Math.max(0, (elapsed - holdEnd) / animSpan));
        setBoardPlayPaneCamerasBaseline(blendTriptychCamerasActivePaneOnly(from, to, u, nodesRedrawEasePaneId));
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

  useEffect(() => {
    if (cameraDisplayOverrideByPane === null) {
      return;
    }
    nodesRedrawEaseGenerationRef.current += 1;
    if (boardPlayNodesRedrawCameraAnimRafRef.current != null) {
      cancelAnimationFrame(boardPlayNodesRedrawCameraAnimRafRef.current);
      boardPlayNodesRedrawCameraAnimRafRef.current = null;
    }
  }, [cameraDisplayOverrideByPane]);

  const redrawPlayingRef = useRef(false);
  const redrawProgressiveEpochRef = useRef(0);
  const redrawLoopSnapshotRef = useRef<BoardPlayRedrawLoopSnapshot>({
    activePaneId: "board-overview",
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

  const resetBoardRedrawProgressiveEpoch = useCallback(() => {
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

  const applyBoardRedrawHandlesOnce = useCallback(() => {
    patchFixture((prev) => layoutBoardFixtureRedrawHandles(prev));
  }, [patchFixture]);

  const applyBoardRedrawOnce = useCallback(() => {
    if (boardPlayNodesRedrawCameraAnimRafRef.current != null) {
      cancelAnimationFrame(boardPlayNodesRedrawCameraAnimRafRef.current);
      boardPlayNodesRedrawCameraAnimRafRef.current = null;
    }
    nodesRedrawEaseGenerationRef.current += 1;
    nodesRedrawEaseFromRef.current = {
      "board-detail": { ...camerasByPane["board-detail"] },
      "board-overview": { ...camerasByPane["board-overview"] },
      "board-selection": { ...camerasByPane["board-selection"] },
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

  useEffect(() => {
    if (!boardRedrawPlaying) {
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
  }, [boardRedrawPlaying, patchFixture, setBoardRedrawPlaying]);

  const shellValue = useMemo<BoardPlayShellValue>(
    () => ({
      activePaneId,
      applyBoardRedrawHandlesOnce,
      applyBoardRedrawOnce,
      applyStructuralDelete,
      boardRedrawHandlesAfterNodes,
      boardRedrawMode,
      boardRedrawPlayMaxItersPerFrame,
      boardRedrawPlaying,
      boardRedrawProgressiveAutoStopMs,
      boardRedrawProgressiveEnabled,
      boardSelectionMethod,
      boardSelectionMode,
      boardSelectionTargets,
      boardGridSnapEnabled,
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
      boardRedrawPlaying,
      boardRedrawProgressiveAutoStopMs,
      boardRedrawProgressiveEnabled,
      boardSelectionMethod,
      boardSelectionMode,
      boardSelectionTargets,
      boardGridSnapEnabled,
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

  const boardWindowKinds = useMemo(
    () => boardWindowKindsWithLodMeasures(boardLodModeByPane, boardEffectiveLodByPane, setBoardLodModeForPane),
    [boardLodModeByPane, boardEffectiveLodByPane, setBoardLodModeForPane],
  );

  const augmentPanelTabs = useMemo(
    () => ({
      workbench: [new BoardFixtureLibraryPanelDefinition().resolveTab()],
      details: [new BoardSelectionInspectorPanelDefinition().resolveTab(), new BoardPlaySettingsPanelDefinition().resolveTab()],
    }),
    [],
  );

  const boardWorkbenchRef = useRef<Workbench | null>(null);
  if (!boardWorkbenchRef.current) {
    const wb = new Workbench();
    const ctrl = new BoardPlayShellController(wb.commandBus, () => wb.notify());
    wb.addApp(buildBoardPlayWorkbenchApp(ctrl));
    boardWorkbenchRef.current = wb;
  }
  const boardWorkbench = boardWorkbenchRef.current;

  useEffect(() => {
    const app = boardWorkbench.apps[0];
    if (app) app.onActiveWindowChange = onBoardPlayActiveWindowChange;
  }, [boardWorkbench, onBoardPlayActiveWindowChange]);

  return (
    <BoardPlayShellContext.Provider value={shellValue}>
      <BoardPlayLodRuntimeContext.Provider value={setBoardEffectiveLodForPane}>
        <WorkbenchView
          workbench={boardWorkbench}
          defaultAppId={BOARD_PLAY_APP_ID}
          augmentPanelTabs={augmentPanelTabs}
          extraFooterItems={surfaceFooterItems}
          initialPanelVisibility={{ leftSidePanel: true, rightSidePanel: true }}
          mobile={mobile}
          resolvedWindowKindsOverride={boardWindowKinds}
          slotToolbar={<BoardPlayToolbar />}
        />
      </BoardPlayLodRuntimeContext.Provider>
    </BoardPlayShellContext.Provider>
  );
}

function BoardPlayApp(): ReactElement {
  return (
    <LevelProvider level="window">
      <div className={`flex h-screen min-h-0 w-screen flex-col ${getLevelBgClass("window")}`}>
        <BoardPlayInner />
      </div>
    </LevelProvider>
  );
}

export function createBoardPlayElement(): ReactElement {
  return <BoardPlayApp />;
}
// #endregion 🔖Entrypoint
