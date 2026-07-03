// #region 🧲Header
/** @emoji 🛝 Playground play host for Puzzle2d — loaded only via `./play` subpath. */
// #endregion 🧲Header

import type { ReactElement } from "react";
import type { AppRendererContribution, PlaygroundMountProps } from "@semio-tech/framework-platform-core";
import type { OsAppInstance } from "@semio-tech/framework-os-core";
import { OsUpstreamBadge, useOsInstanceMaterialization } from "@semio-tech/framework-os-renderer-react";
import { type PlaygroundKeybinding, PureSidePanelTabDefinition, CallbackTreePanelDefinition, buildPuzzle2dPlayInspectorTree, Platform, CommandBus, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, uiTreeNodeToTreePanelConfig, useShellWindowInstance, registerSidePanelBody, uiDeclarativeSectionsToTree, uiInspectorAllEqual, playgroundResolvedExampleId, PLAYGROUND_NO_EXAMPLE_ID, isPlaygroundNoExampleId, PlaygroundView } from "@semio-tech/framework-playground-renderer-react";
import { reactHostPort, Icon, Tree, createIconComponent } from "@semio-tech/ui-react";
import { type SidePanelTabConfig, type TreeDataSection, UiNode, UiTreeNode, UiSectionNode, UiPuzzle2dHostSurfaceNode } from "@semio-tech/framework-playground-core";
// #region 🔌Adapters
import {
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
    PUZZLE_2D_PLAY_SURFACE_ID_COMPILED_DAG,
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
    parsePuzzle2dFixture,
    subscribePuzzle2dFillSessionReady,
    type Puzzle2dPlayHostBridge,
    type Puzzle2dPlayPaneId,
    type Puzzle2dPlayStructuralDeleteItem,
    puzzle2dPlayWindowBodies,
} from "@semio-tech/puzzle-2d-core";

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
import { buildPuzzle2dSceneDescriptorFromFixture, Puzzle2dCanvas, puzzle2dSetBrushPlaceCommitHandler, puzzle2dFixturePaletteTreeDragController, PUZZLE_2D_FIXTURE_DRAG_MIME } from "./index.tsx";
// #endregion 🔌Adapters

const PUZZLE_2D_PLAY_IS_WIRES = import.meta.env.PLAYGROUND_APP_KIND === "wires";

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

function Puzzle2dPlayCompiledDagSurfaceHost({ node: _node }: { readonly node: import("@semio-tech/framework-platform-core").UiWriterHostSurfaceNode }): ReactElement {
  const ctrl = puzzle2dPlayShellControllerRef.current ?? undefined;
  const [revision, setRevision] = reactHostPort.useState(0);
  const [writerModule, setWriterModule] = reactHostPort.useState<{ readonly WriterCanvas: typeof import("@semio-tech/writer-react").WriterCanvas; readonly createWriterDocument: typeof import("@semio-tech/writer-core").createWriterDocument } | null>(null);
  reactHostPort.useEffect(() => {
    void Promise.all([import("@semio-tech/writer-core"), import("@semio-tech/writer-react")]).then(([core, react]) => {
      setWriterModule({ WriterCanvas: react.WriterCanvas, createWriterDocument: core.createWriterDocument });
    });
  }, []);
  reactHostPort.useEffect(() => ctrl?.subscribeSnapshot(() => setRevision((value) => value + 1)) ?? undefined, [ctrl]);
  void ctrl?.getHoverEpoch();
  void ctrl?.getSelectEpoch();
  const document = reactHostPort.useMemo(
    () => ctrl?.getWriterDocumentCompiledDag() ?? writerModule?.createWriterDocument({ id: "puzzle-2d-compiled-dag", languageId: "wire", text: "" }),
    [ctrl, revision, writerModule],
  );
  const onHoverChange = reactHostPort.useCallback((offset: number | null) => {
    puzzle2dPlayShellControllerRef.current?.run("setWireHover", { offset });
  }, []);
  const onSelectionChange = reactHostPort.useCallback((range: { start: number; end: number }) => {
    puzzle2dPlayShellControllerRef.current?.run("setWireSelect", range);
  }, []);
  if (!writerModule || !document) {
    return <div className="h-full min-h-0" />;
  }
  const { WriterCanvas } = writerModule;
  return (
    <WriterCanvas
      document={document}
      className="h-full min-h-0"
      onHoverChange={onHoverChange}
      onSelectionChange={onSelectionChange}
      externalHoverOccurrences={ctrl?.getWireHoverOccurrences()}
      externalHoverOccurrencesSignal={ctrl?.getHoverEpoch()}
      externalSelectionOccurrences={ctrl?.getWireSelectOccurrences()}
      externalSelectionOccurrencesSignal={ctrl?.getSelectEpoch()}
    />
  );
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

  const exampleContribution = reactHostPort.useMemo(() => ({
    options: PUZZLE_2D_PLAY_NAVBAR_EXAMPLE_OPTIONS,
    activeExampleId: () => activeExampleId,
    onSelect: applyNavbarFixtureId,
  }), [activeExampleId, applyNavbarFixtureId]);

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
              <PlaygroundView runtime={puzzle2dRuntime} defaultAppId={PUZZLE_2D_PLAY_APP_ID} augmentPanelTabs={augmentPanelTabs} playgroundKeybindings={playgroundKeybindings} onActiveWindowChange={onPuzzle2dPlayActiveWindowChange} exampleContribution={exampleContribution} />
            </Puzzle2dPlayLodRuntimeContext.Provider>
          </Puzzle2dPlayCamerasContext.Provider>
        </Puzzle2dPlayCanvasSelectionContext.Provider>
      </Puzzle2dPlaySelectionContext.Provider>
    </Puzzle2dPlayShellContext.Provider>
  );
}

function puzzle2dMountChrome({ runtime }: PlaygroundMountProps): ReactElement {
  return <Puzzle2dPlayInner puzzle2dRuntime={runtime as Platform} />;
}

function Puzzle2dOsInstanceHost({ instance }: { readonly instance: OsAppInstance }): ReactElement {
  const bundle = useOsInstanceMaterialization(instance);
  const fixture = reactHostPort.useMemo(() => {
    if (bundle.projection && typeof bundle.projection === "object") {
      return parsePuzzle2dFixture(bundle.projection) ?? PUZZLE_2D_PLAY_EMPTY_FIXTURE;
    }
    if (instance.sourceDocument.inline) {
      try {
        return parsePuzzle2dFixture(JSON.parse(instance.sourceDocument.inline)) ?? PUZZLE_2D_PLAY_EMPTY_FIXTURE;
      } catch {
        return PUZZLE_2D_PLAY_EMPTY_FIXTURE;
      }
    }
    return PUZZLE_2D_PLAY_EMPTY_FIXTURE;
  }, [bundle.projection, instance.sourceDocument.inline]);
  const declarativeSceneDescriptor = reactHostPort.useMemo(() => buildPuzzle2dSceneDescriptorFromFixture(fixture), [fixture]);
  return (
    <div className="relative h-full min-h-0">
      <OsUpstreamBadge upstreamInstanceId={bundle.upstreamInstanceId} />
      <Puzzle2dCanvas className="h-full" declarativeSceneDescriptor={declarativeSceneDescriptor} />
    </div>
  );
}

/** @emoji 🛝 Puzzle 2D app renderer contribution for playground and OS shells. */
export const puzzle2dAppRenderer: AppRendererContribution = {
  windowBodies: puzzle2dPlayWindowBodies,
  surfaceHosts: {
    [PUZZLE_2D_PLAY_SURFACE_ID]: Puzzle2dPlayPaneSurfaceHost,
    [PUZZLE_2D_PLAY_SURFACE_ID_COMPILED_DAG]: Puzzle2dPlayCompiledDagSurfaceHost,
  },
  tabIcons: {
    [PUZZLE_2D_PLAY_ICON_KINDS]: "tags",
    "puzzle.2d-play.icon.inspector": "clipboard-list",
    "puzzle.2d-play.icon.settings": "settings",
  },
  mountChrome: puzzle2dMountChrome,
  instanceHost: Puzzle2dOsInstanceHost,
  treeDragController: (dragByItemId) => {
    const sample = dragByItemId.values().next().value;
    if (sample && PUZZLE_2D_FIXTURE_DRAG_MIME in sample) return puzzle2dFixturePaletteTreeDragController(dragByItemId);
    return undefined;
  },
};

/** @emoji 🔗 WIRES play renderer — same chrome as puzzle 2D play. */
export const wiresAppRenderer: AppRendererContribution = puzzle2dAppRenderer;

// #endregion 🔖Entrypoint

// #endregion 🛝PlayHost
//#endregion 🔖Puzzle2dPlayHost