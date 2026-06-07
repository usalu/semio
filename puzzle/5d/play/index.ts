// #region 🧲Header
// 💻 puzzle/5d/play/index.ts — Puzzle 5d play on `@framework/playground/core`: unified 5d fixture, LOD measures, relocate tools (no React).
// #endregion 🧲Header

import {
  CommandBus,
  Controller,
  Store,
  Platform,
  AppRuntime,
  ModeRuntime,
  WindowKindRuntime,
  buildPuzzle2dWindowBody,
  buildPuzzle3dWindowBody,
  createDefaultLayout,
  type ToolItem,
  type WindowBodyViewContext,
  type CommandDescriptor,
  type WindowEngagement,
  type WindowMeasure,
  type UiNode,
  Playground,
  PLAYGROUND_NO_FIXTURE_ID,
  type PlaygroundFixtureCatalog,
  type PlaygroundFixtureHost,
  type PlaygroundKeybinding,
  isPlaygroundNoFixtureId,
  playgroundTreePanelRootItems,
  platformFromViewContext,
  type UiTreeItemNode,
  type UiTreeNode,
  type UiTreeSectionNode,
  enforcePlaygroundWindowEngagementInput,
  collectUiTreeItemDragData,
} from "@framework/playground/core";

import { buildPuzzle2dPlayHierarchySections, buildPuzzle2dPlayKindsTree, buildPuzzle2dPlayToolbarTools, type Puzzle2dPlayToolbarState } from "../../2d/play/index.ts";
import {
  PUZZLE_2D_FIXTURE_DRAG_V1_MIME,
  beginPuzzle2dFixturePalettePointerDrag,
  cancelPuzzle2dFixturePalettePointerDrag,
  puzzle2dFixturePaletteTreeDragController,
} from "../../2d/react/index.tsx";
import nakagin2dJson from "../../2d/fixture/nakagin-capsule-tower.2d.json";
import {
  PUZZLE_2D_LOD_MODE_AUTOMATIC,
  puzzle2dLodAutomaticSelectLabel,
  puzzle2dLodCanvasProps,
  isPuzzle2dDrawLodKind,
  parsePuzzle2dFixtureV1,
  DEFAULT_PUZZLE_2D_BRUSH_FLUSH_DISTANCE_PX,
  type Puzzle2dDrawLodKind,
  type Puzzle2dFixtureV1,
  type Puzzle2dLodModeKind,
  type CameraState,
  type Puzzle2dSelectionMethod,
  type Puzzle2dSelectionMode,
  type Puzzle2dSelectionTargets,
} from "../../2d/react/index.tsx";
import nakagin3dJson from "../../3d/fixture/nakagin-capsule-tower.3d.json";
import { DEFAULT_GUMBALL_CONFIG, type GumballConfig } from "@ui/react";
import { buildPuzzle3dPlayHierarchyTree, buildPuzzle3dPlayKindsTree, PUZZLE_3D_GUMBALL_GROUPS, PUZZLE_3D_PLAY_EMPTY_SELECTION, type Puzzle3dGumballGroupKey } from "../../3d/play/index.ts";
import {
  FIXTURE_DRAG_V1_MIME,
  beginPuzzle3dFixturePalettePointerDrag,
  cancelPuzzle3dFixturePalettePointerDrag,
  puzzle3dFixturePaletteTreeDragController,
} from "../../3d/react/index.tsx";
import {
  DEFAULT_MANUAL_LOD,
  PUZZLE_3D_LOD_SLIDER_MAX,
  PUZZLE_3D_LOD_SLIDER_MIN,
  formatLod,
  lodFromSliderValue,
  parseFixtureV1,
  puzzle3dLodCanvasProps,
  sliderValueFromLod,
  type FixtureV1 as Puzzle3dFixtureV1,
  DEFAULT_BRUSH_PLACEMENT_OVERLAP_BUDGET,
  BRUSH_PLACEMENT_OVERLAP_BUDGET_MAX,
  BRUSH_PLACEMENT_OVERLAP_BUDGET_STEP,
} from "../../3d/react/index.tsx";
import {
  createStore,
  parseV1,
  project2d,
  project3d,
  project2dKindCatalogs,
  project3dKindCatalogs,
  compose5d,
  sharedKindsFromMetas,
  type KindCatalogBundle as Puzzle5dKindCatalogBundle,
  PUZZLE_5D_ENGAGEMENT_TOOL_BRUSH_ID,
  PUZZLE_5D_ENGAGEMENT_TOOL_FILL_ID,
  PUZZLE_5D_ENGAGEMENT_TOOL_SELECT_ID,
  PUZZLE_5D_FILL_COUNT_MAX,
  type Puzzle5dActiveTool,
  type Puzzle5dBrushPlacement,
  type Store as Puzzle5dStore,
  type StoreSnapshot as Puzzle5dStoreSnapshot,
  type V1 as Puzzle5dV1,
} from "../react/index.tsx";
import nakagin5dJson from "../fixture/nakagin-capsule-tower.5d.json";

//#region 🔖Ids
export const PUZZLE_5D_PLAY_APP_ID = "puzzle-5d-play";
export const PUZZLE_5D_PLAY_CONTROLLER_ID = "puzzle-5d-play";
export const PUZZLE_5D_PLAY_2D_WINDOW_ID = "puzzle-5d-2d";
export const PUZZLE_5D_PLAY_3D_WINDOW_ID = "puzzle-5d-3d";
export const PUZZLE_5D_PLAY_2D_WINDOW_LABEL = "Puzzle 2d";
export const PUZZLE_5D_PLAY_3D_WINDOW_LABEL = "Puzzle 3d";
export const PUZZLE_5D_PLAY_2D_BODY_KEY = "puzzle.5d.play.2d";
export const PUZZLE_5D_PLAY_3D_BODY_KEY = "puzzle.5d.play.3d";
export const PUZZLE_5D_PLAY_2D_SURFACE_ID = "puzzle.5d.play.2d/v1";
export const PUZZLE_5D_PLAY_3D_SURFACE_ID = "puzzle.5d.play.3d/v1";
export const PUZZLE_5D_PLAY_HIERARCHY_TAB_ID = "puzzle-5d-play-hierarchy";
export const PUZZLE_5D_PLAY_KINDS_TAB_ID = "puzzle-5d-play-kinds";
export const PUZZLE_5D_PLAY_ICON_KINDS = "puzzle.5d-play.icon.kinds";

export const PUZZLE_5D_PLAY_FIXTURE_NAKAGIN_ID = "nakagin";

export const PUZZLE_5D_PLAY_FIXTURE_OPTIONS = [{ id: PUZZLE_5D_PLAY_FIXTURE_NAKAGIN_ID, label: "Nakagin capsule tower" }] as const;

const PUZZLE_5D_PLAY_LOD_TIERS_2D: readonly Puzzle2dDrawLodKind[] = ["minimap", "overview", "compact", "normal", "detail", "micro"];

function puzzle5dPlayCmd(command: string, args?: Record<string, unknown>): CommandDescriptor {
  return { controllerId: PUZZLE_5D_PLAY_CONTROLLER_ID, command, args: args as never };
}

const PUZZLE_5D_BRUSH_FLUSH_DISTANCE_MIN = 0;
const PUZZLE_5D_BRUSH_FLUSH_DISTANCE_MAX = 160;
const PUZZLE_5D_BRUSH_FLUSH_DISTANCE_STEP = 4;

/** @emoji 🔗 React host bridge: toolbar snapshot + commands that need canvas/fixture context. */
export interface Puzzle5dPlayHostBridge {
  getToolbarState(): Puzzle2dPlayToolbarState;
  runHostCommand(command: string, args?: unknown): void;
}
//#endregion 🔖Ids

//#region 🔖Puzzle5dPlayHierarchy
export interface Puzzle5dPlayHierarchySelectHandlers {
  readonly onSelect2d: (id: string) => void;
  readonly onSelect3dObject: (objectId: string) => void;
  readonly onSelect3dVortex: (vortexFullId: string) => void;
  readonly onSelect3dAttraction: (attractionId: string) => void;
}

/** @emoji 🌳 Puzzle 5d hierarchy: flat 2d and 3d composition sections. */
function puzzle5dPlayHierarchyTreeSections(domain: "2d" | "3d", labelPrefix: string, tree: UiTreeNode): UiTreeSectionNode[] {
  if (tree.type !== "tree") {
    return [];
  }
  return tree.sections.map((section) => ({
    ...section,
    id: section.id.replace(/^puzzle-(?:2d|3d)-play-hierarchy\./, `puzzle-5d-play-hierarchy.${domain}.`),
    label: `${labelPrefix} · ${section.label}`,
  }));
}

export function buildPuzzle5dPlayHierarchySections(snapshot: Puzzle5dPlaySnapshot, handlers: Puzzle5dPlayHierarchySelectHandlers): UiTreeNode {
  const sections: UiTreeSectionNode[] = [];
  if (snapshot.fixture2d) {
    const tree2d = buildPuzzle2dPlayHierarchySections(snapshot.fixture2d, [...snapshot.selected2d], handlers.onSelect2d);
    sections.push(...puzzle5dPlayHierarchyTreeSections("2d", "2d", tree2d));
  }
  if (snapshot.fixture3d) {
    const selection3d = snapshot.selected3d ? { ...PUZZLE_3D_PLAY_EMPTY_SELECTION, objectIds: [snapshot.selected3d] } : PUZZLE_3D_PLAY_EMPTY_SELECTION;
    const tree3d = buildPuzzle3dPlayHierarchyTree(snapshot.fixture3d, selection3d);
    sections.push(...puzzle5dPlayHierarchyTreeSections("3d", "3d", tree3d));
  }
  if (!sections.length) {
    return playgroundTreePanelRootItems("puzzle-5d-play-hierarchy.root", [{ id: "puzzle-5d-play-hierarchy.empty", label: "(no fixtures)" }]);
  }
  return { type: "tree", sections };
}
//#endregion 🔖Puzzle5dPlayHierarchy

//#region 🔖Puzzle5dPlayKinds
function puzzle5dPlayKindTreeItem(domain: "2d" | "3d", item: UiTreeItemNode): UiTreeItemNode {
  return {
    ...item,
    id: item.id.replace(/^puzzle-(?:2d|3d)-play-kinds\./, `puzzle-5d-play-kinds.${domain}.`),
    ...(item.items ? { items: item.items.map((child) => puzzle5dPlayKindTreeItem(domain, child)) } : {}),
  };
}

function puzzle5dPlayKindTreeSections(domain: "2d" | "3d", labelPrefix: string, tree: UiTreeNode): UiTreeSectionNode[] {
  if (tree.type !== "tree") {
    return [];
  }
  return tree.sections.map((section) => ({
    ...section,
    id: section.id.replace(/^puzzle-(?:2d|3d)-play-kinds\./, `puzzle-5d-play-kinds.${domain}.`),
    label: `${labelPrefix} · ${section.label}`,
    items: section.items?.map((item) => puzzle5dPlayKindTreeItem(domain, item)),
  }));
}

/** @emoji 🏷️ Workbench kinds tab: flat (2d) and volume (3d) palette rows with drag payloads. */
export function buildPuzzle5dPlayKindsTree(snapshot: Puzzle5dPlaySnapshot): UiTreeNode {
  const bundle: Puzzle5dKindCatalogBundle | undefined = snapshot.kindCatalogs ?? snapshot.sharedKinds.kindCatalogs;
  const catalogs2d = project2dKindCatalogs(bundle);
  const catalogs3d = project3dKindCatalogs(bundle);
  const flat = buildPuzzle2dPlayKindsTree(catalogs2d);
  const volume = buildPuzzle3dPlayKindsTree(catalogs3d, snapshot.fixture3d ?? undefined);
  const sections = [...puzzle5dPlayKindTreeSections("2d", "2d", flat), ...puzzle5dPlayKindTreeSections("3d", "3d", volume)];
  if (!sections.length) {
    return {
      type: "tree",
      sections: [
        {
          id: "puzzle-5d-play-kinds.empty",
          label: "Kinds",
          defaultOpen: true,
          items: [{ id: "puzzle-5d-play-kinds.empty.msg", label: "No kind catalogs in this fixture" }],
        },
      ],
    };
  }
  return { type: "tree", sections };
}

function puzzle5dPaletteDragDomainFromEncoded(encoded: string): "2d" | "3d" | null {
  try {
    const parsed = JSON.parse(encoded) as { readonly schema?: string };
    if (parsed.schema === "puzzle.2d.fixture/v1") {
      return "2d";
    }
    if (parsed.schema === "puzzle.3d.fixture/v1") {
      return "3d";
    }
  } catch {
    return null;
  }
  return null;
}

function puzzle5dPaletteDragDomainFromDragData(dragData: Record<string, string> | undefined): "2d" | "3d" | null {
  if (!dragData) {
    return null;
  }
  if (dragData[PUZZLE_2D_FIXTURE_DRAG_V1_MIME]?.trim()) {
    return "2d";
  }
  if (dragData[FIXTURE_DRAG_V1_MIME]?.trim()) {
    return "3d";
  }
  return null;
}

/** @emoji 🖱️ Tree drag controller for merged flat + volume palette rows in puzzle 5d play. */
export function puzzle5dFixturePaletteTreeDragController(dragDataByItemId: ReadonlyMap<string, Record<string, string>>) {
  const flatDragByItemId = new Map<string, Record<string, string>>();
  const volumeDragByItemId = new Map<string, Record<string, string>>();
  for (const [itemId, dragData] of dragDataByItemId) {
    const domain = puzzle5dPaletteDragDomainFromDragData(dragData);
    if (domain === "2d") {
      flatDragByItemId.set(itemId, dragData);
    } else if (domain === "3d") {
      volumeDragByItemId.set(itemId, dragData);
    }
  }
  const flatController = puzzle2dFixturePaletteTreeDragController(flatDragByItemId);
  const volumeController = puzzle3dFixturePaletteTreeDragController(volumeDragByItemId);
  const readEncoded = (dragData: Record<string, string>): string | undefined =>
    dragData[PUZZLE_2D_FIXTURE_DRAG_V1_MIME]?.trim() || dragData[FIXTURE_DRAG_V1_MIME]?.trim() || undefined;
  return {
    getDragData: ({ sourceItem }: { readonly sourceItem: { readonly id: string } }) => dragDataByItemId.get(sourceItem.id),
    pointerPaletteDrag: {
      readEncodedDragPayload: readEncoded,
      begin: (encoded: string) => {
        const domain = puzzle5dPaletteDragDomainFromEncoded(encoded);
        if (domain === "2d") {
          beginPuzzle2dFixturePalettePointerDrag(encoded);
          return;
        }
        if (domain === "3d") {
          beginPuzzle3dFixturePalettePointerDrag(encoded);
        }
      },
      cancel: () => {
        cancelPuzzle2dFixturePalettePointerDrag();
        cancelPuzzle3dFixturePalettePointerDrag();
      },
    },
    onDragStart: (ctx: { readonly sourceItem: { readonly id: string } }) => {
      const domain = puzzle5dPaletteDragDomainFromDragData(dragDataByItemId.get(ctx.sourceItem.id));
      if (domain === "2d") {
        flatController.onDragStart?.(ctx as never);
        return;
      }
      if (domain === "3d") {
        volumeController.onDragStart?.(ctx as never);
      }
    },
    onDragEnd: (ctx: { readonly sourceItem: { readonly id: string } }) => {
      const domain = puzzle5dPaletteDragDomainFromDragData(dragDataByItemId.get(ctx.sourceItem.id));
      if (domain === "2d") {
        flatController.onDragEnd?.(ctx as never);
        return;
      }
      if (domain === "3d") {
        volumeController.onDragEnd?.(ctx as never);
      }
    },
  };
}
//#endregion 🔖Puzzle5dPlayKinds

//#region 🔖Helpers
function puzzle5dPlayLodTierMenuLabel(tier: string): string {
  return tier.charAt(0).toUpperCase() + tier.slice(1);
}

function puzzle5dControllerFromContext(ctx: WindowBodyViewContext): Puzzle5dPlayShellController | undefined {
  return platformFromViewContext(ctx)?.getActiveApp()?.controller as Puzzle5dPlayShellController | undefined;
}

function sameCamera(a: CameraState | null, b: CameraState): boolean {
  return Boolean(a && a.x === b.x && a.y === b.y && a.zoom === b.zoom);
}
//#endregion 🔖Helpers

//#region 🔖Controller
export interface Puzzle5dPlaySnapshot {
  readonly manifestLabel: string | undefined;
  readonly fixture2d: Puzzle2dFixtureV1 | null;
  readonly fixture3d: Puzzle3dFixtureV1 | null;
  readonly selected2d: ReadonlySet<string>;
  readonly camera2d: CameraState | null;
  readonly camera3d: CameraState | null;
  readonly selected3d: string | null;
  readonly gumballConfig: GumballConfig;
  readonly lod3dTag: number;
  readonly lod2dTag: Puzzle2dDrawLodKind;
  readonly lod2dProps: ReturnType<typeof puzzle2dLodCanvasProps>;
  readonly lod3dProps: ReturnType<typeof puzzle3dLodCanvasProps>;
  readonly automaticLod3d: boolean;
  readonly depthVariableLod3d: boolean;
  readonly lod3dSlider: number;
  readonly sharedKinds: ReturnType<typeof sharedKindsFromMetas>;
  readonly kindCatalogs: Puzzle5dKindCatalogBundle | undefined;
  readonly connect2d: number;
  readonly connect3d: number;
  readonly proximity2d: number;
  readonly proximity3d: number;
  readonly activeTool: Puzzle5dActiveTool;
  readonly brushFlushDistance: number;
  readonly brushOverlapBudget: number;
  readonly fillCount: number;
  readonly fillBuildDone: boolean;
  readonly selectionMethod: Puzzle2dSelectionMethod;
  readonly selectionMode: Puzzle2dSelectionMode;
}

function loadNakagin5dModel(): Puzzle5dV1 {
  const model = parseV1(nakagin5dJson as unknown);
  if (!model) throw new Error("nakagin-capsule-tower.5d.json must use schema puzzle.5d/v1");
  return model;
}

function puzzle5dPlayEmptyModel(): Puzzle5dV1 {
  return {
    schema: "puzzle.5d/v1",
    domain: "architecture",
    camera2d: { x: 0, y: 0, zoom: 1 },
    camera3d: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1 },
    parts: [],
    ties: [],
    label: "",
  };
}

export const PUZZLE_5D_PLAY_STORE_ID = "puzzle-5d";

/** @emoji 🔗 Adapts {@link Puzzle5dStore} to {@link Store} for controller-owned registration. */
export class Puzzle5dStoreBridge extends Store<Puzzle5dStoreSnapshot> {
  private detach?: () => void;

  constructor(readonly inner: Puzzle5dStore) {
    super();
    this.detach = inner.subscribe(() => this.notify());
  }

  override getSnapshot(): Puzzle5dStoreSnapshot {
    return this.inner.getSnapshot();
  }

  override dispose(): void {
    this.detach?.();
    super.dispose();
  }
}

/** @emoji 🎛 Puzzle 5d play shell controller shared by declarative 2d and 3d windows. */
export class Puzzle5dPlayShellController extends Controller implements PlaygroundFixtureHost {
  readonly mainMode = new ModeRuntime("main", "Puzzle 5d", undefined);
  private activeFixtureId = PUZZLE_5D_PLAY_FIXTURE_NAKAGIN_ID;
  readonly puzzle5dStore: Puzzle5dStore = createStore(loadNakagin5dModel());
  readonly puzzle5dStoreBridge: Puzzle5dStoreBridge;
  private gumballConfig: GumballConfig = { ...DEFAULT_GUMBALL_CONFIG };
  private selected2d: ReadonlySet<string> = new Set();
  private selected3d: string | null = null;
  private camera2d: CameraState | null = { ...this.puzzle5dStore.read().camera2d };
  private camera3d: CameraState | null = { ...this.puzzle5dStore.read().camera3d };
  private lod3dTag = DEFAULT_MANUAL_LOD;
  private automaticLod3d = true;
  private depthVariableLod3d = false;
  private manualLod3d = DEFAULT_MANUAL_LOD;
  private lod3dSlider = sliderValueFromLod(DEFAULT_MANUAL_LOD);
  private lod2dTag: Puzzle2dDrawLodKind = "normal";
  private lod2dMode: Puzzle2dLodModeKind = PUZZLE_2D_LOD_MODE_AUTOMATIC;
  private connect2d = 0;
  private connect3d = 0;
  private proximity2d = 0;
  private proximity3d = 0;
  private engagementInputByWindow: Record<string, string> = {
    [PUZZLE_5D_PLAY_2D_WINDOW_ID]: "",
    [PUZZLE_5D_PLAY_3D_WINDOW_ID]: "",
  };
  private hostBridge: Puzzle5dPlayHostBridge | null = null;
  private activeTool: Puzzle5dActiveTool = "select";
  private brushFlushDistance = DEFAULT_PUZZLE_2D_BRUSH_FLUSH_DISTANCE_PX;
  private brushOverlapBudget = DEFAULT_BRUSH_PLACEMENT_OVERLAP_BUDGET;
  private brushEngagementPossibles: { readonly id: string; readonly label: string }[] = [];
  private puzzle2dSelectionMethod: Puzzle2dSelectionMethod = "rectangle";
  private puzzle2dSelectionMode: Puzzle2dSelectionMode = "default";
  private puzzle2dSelectionTargets: Puzzle2dSelectionTargets = { nodes: true, edges: true, handles: true };
  private puzzle2dGridSnapEnabled = true;
  private puzzle2dRedrawPlaying = false;

  private lastStoreShellModel: Puzzle5dV1 | null = null;
  private lastStoreShellSelectionKey = "";
  private lastStoreShellFillCount = 0;
  private lastStoreShellFillBuildDone = true;

  constructor(commandBus: CommandBus, hostNotify: () => void) {
    super(PUZZLE_5D_PLAY_CONTROLLER_ID, commandBus, hostNotify);
    this.puzzle5dStoreBridge = new Puzzle5dStoreBridge(this.puzzle5dStore);
    this.provideStore(PUZZLE_5D_PLAY_STORE_ID, this.puzzle5dStoreBridge);
    this.puzzle5dStore.subscribe(() => this.notifyStoreShellIfNeeded());
    this.rebuildShellMode();
  }

  private notifyStoreShellIfNeeded(): void {
    const snap = this.puzzle5dStore.getSnapshot();
    const selectionKey = `${snap.selection.partIds.join("\u0001")}\u0002${snap.selection.anchorIds.join("\u0001")}`;
    if (
      this.lastStoreShellModel === snap.model &&
      this.lastStoreShellSelectionKey === selectionKey &&
      this.lastStoreShellFillCount === snap.fillCount &&
      this.lastStoreShellFillBuildDone === snap.fillBuildDone
    ) {
      return;
    }
    this.lastStoreShellModel = snap.model;
    this.lastStoreShellSelectionKey = selectionKey;
    this.lastStoreShellFillCount = snap.fillCount;
    this.lastStoreShellFillBuildDone = snap.fillBuildDone;
    this.emit();
  }

  setHostBridge(bridge: Puzzle5dPlayHostBridge | null): void {
    this.hostBridge = bridge;
    this.rebuildShellMode();
  }

  getActiveTool(): Puzzle5dActiveTool {
    return this.activeTool;
  }

  getBrushFlushDistance(): number {
    return this.brushFlushDistance;
  }

  setBrushEngagementPossibles(rows: readonly { readonly id: string; readonly label: string }[]): void {
    const next = [...rows];
    if (next.length === this.brushEngagementPossibles.length && next.every((row, index) => row.id === this.brushEngagementPossibles[index]?.id)) {
      return;
    }
    this.brushEngagementPossibles = next;
    this.rebuildShellMode();
    this.emit();
  }

  private toolbarState(): Puzzle2dPlayToolbarState {
    return (
      this.hostBridge?.getToolbarState() ?? {
        puzzle2dActiveTool: this.activeTool,
        puzzle2dBrushFlushDistance: this.brushFlushDistance,
        puzzle2dSelectionMethod: this.puzzle2dSelectionMethod,
        puzzle2dSelectionMode: this.puzzle2dSelectionMode,
        puzzle2dSelectionTargets: this.puzzle2dSelectionTargets,
        puzzle2dGridSnapEnabled: this.puzzle2dGridSnapEnabled,
        puzzle2dRedrawPlaying: this.puzzle2dRedrawPlaying,
      }
    );
  }

  private setPlayActiveTool(tool: Puzzle5dActiveTool): void {
    const prev = this.activeTool;
    if (prev === tool) return;
    this.activeTool = tool;
    if (tool !== "brush") {
      this.brushEngagementPossibles = [];
    }
    this.hostBridge?.runHostCommand("setActiveTool", { tool, prevTool: prev });
    this.rebuildShellMode();
    this.emit();
  }

  private rebuildShellMode(): void {
    const relocateTools: ToolItem[] = PUZZLE_3D_GUMBALL_GROUPS.map(({ key, label, iconId }, order) => ({
      id: `puzzle5d.gumball.${key}`,
      kind: "toggle" as const,
      iconId,
      text: label,
      order: order + 100,
      pressed: this.gumballConfig[key] !== false,
      controllerId: PUZZLE_5D_PLAY_CONTROLLER_ID,
      command: "setGumballConfigToggle",
      args: { key },
    }));
    const flatTools = buildPuzzle2dPlayToolbarTools(this.toolbarState(), PUZZLE_5D_PLAY_CONTROLLER_ID);
    this.mainMode.tools = {
      selection: flatTools.selection,
      view: flatTools.view,
      create: flatTools.create,
      actions: [...(flatTools.actions ?? []), ...relocateTools],
    };
    this.mainMode.windowKinds = this.getWindowKinds();
  }

  private brushMeasuresGroup(windowId: string): WindowMeasure {
    return {
      kind: "group",
      id: `${windowId}-brush`,
      label: "Brush",
      children: [
        {
          kind: "slider",
          id: `${windowId}-brush-flush-distance`,
          label: `Flush ${this.brushFlushDistance.toFixed(0)}`,
          value: this.brushFlushDistance,
          min: PUZZLE_5D_BRUSH_FLUSH_DISTANCE_MIN,
          max: PUZZLE_5D_BRUSH_FLUSH_DISTANCE_MAX,
          step: PUZZLE_5D_BRUSH_FLUSH_DISTANCE_STEP,
          onChange: puzzle5dPlayCmd("setBrushFlushDistance"),
        },
        {
          kind: "slider",
          id: `${windowId}-brush-overlap-budget`,
          label: `Overlap ${this.brushOverlapBudget.toFixed(2)} m³`,
          value: this.brushOverlapBudget,
          min: 0,
          max: BRUSH_PLACEMENT_OVERLAP_BUDGET_MAX,
          step: BRUSH_PLACEMENT_OVERLAP_BUDGET_STEP,
          onChange: puzzle5dPlayCmd("setBrushOverlapBudget"),
        },
      ],
    };
  }

  private lod2dMeasure(): WindowMeasure {
    return {
      kind: "select",
      id: `${PUZZLE_5D_PLAY_2D_WINDOW_ID}-lod`,
      value: this.lod2dMode,
      items: [{ id: "automatic", label: puzzle2dLodAutomaticSelectLabel(this.lod2dTag), value: PUZZLE_2D_LOD_MODE_AUTOMATIC }, ...PUZZLE_5D_PLAY_LOD_TIERS_2D.map((tier) => ({ id: tier, label: puzzle5dPlayLodTierMenuLabel(tier), value: tier }))],
      onChange: { controllerId: PUZZLE_5D_PLAY_CONTROLLER_ID, command: "set2dLodMode" },
    };
  }

  private lod3dMeasures(): readonly WindowMeasure[] {
    return [
      {
        kind: "toggle",
        id: `${PUZZLE_5D_PLAY_3D_WINDOW_ID}-auto`,
        iconId: "zoom-in",
        text: "Auto zoom",
        pressed: this.automaticLod3d,
        onChange: { controllerId: PUZZLE_5D_PLAY_CONTROLLER_ID, command: "set3dAutoLod" },
      },
      {
        kind: "toggle",
        id: `${PUZZLE_5D_PLAY_3D_WINDOW_ID}-depth`,
        iconId: "layers",
        text: "Depth-variable",
        pressed: this.depthVariableLod3d,
        onChange: { controllerId: PUZZLE_5D_PLAY_CONTROLLER_ID, command: "set3dDepthLod" },
      },
      {
        kind: "slider",
        id: `${PUZZLE_5D_PLAY_3D_WINDOW_ID}-lod`,
        label: formatLod(this.lod3dTag),
        value: this.lod3dSlider,
        min: PUZZLE_3D_LOD_SLIDER_MIN,
        max: PUZZLE_3D_LOD_SLIDER_MAX,
        step: 1,
        onChange: { controllerId: PUZZLE_5D_PLAY_CONTROLLER_ID, command: "set3dManualLod" },
      },
    ];
  }

  private windowEngagementFor(windowId: string): WindowEngagement {
    const toolPossibles =
      this.activeTool === "brush" && this.brushEngagementPossibles.length > 0
        ? this.brushEngagementPossibles.map((row) => ({
            id: row.id,
            label: row.label,
            command: puzzle5dPlayCmd("engagementPossibleSelect", { windowId, possibleId: row.id }),
          }))
        : [
            { id: PUZZLE_5D_ENGAGEMENT_TOOL_BRUSH_ID, label: "Brush", command: puzzle5dPlayCmd("engagementPossibleSelect", { windowId, possibleId: PUZZLE_5D_ENGAGEMENT_TOOL_BRUSH_ID }) },
            { id: PUZZLE_5D_ENGAGEMENT_TOOL_FILL_ID, label: "Fill", command: puzzle5dPlayCmd("engagementPossibleSelect", { windowId, possibleId: PUZZLE_5D_ENGAGEMENT_TOOL_FILL_ID }) },
            { id: PUZZLE_5D_ENGAGEMENT_TOOL_SELECT_ID, label: "Select", command: puzzle5dPlayCmd("engagementPossibleSelect", { windowId, possibleId: PUZZLE_5D_ENGAGEMENT_TOOL_SELECT_ID }) },
          ];
    const storeSnap = this.puzzle5dStore.getSnapshot();
    const fillSliderMax = storeSnap.fillBuildDone ? PUZZLE_5D_FILL_COUNT_MAX : Math.max(storeSnap.fillCount, 1);
    const control =
      this.activeTool === "fill"
        ? {
            kind: "slider" as const,
            id: "puzzle5d-fill-count",
            label: `Fill ${storeSnap.fillCount}`,
            value: Math.min(storeSnap.fillCount, fillSliderMax),
            min: 0,
            max: fillSliderMax,
            step: 1,
            onChange: puzzle5dPlayCmd("engagementControlChange", { windowId }),
          }
        : {
            kind: "ring" as const,
            id: "puzzle5d-command-ring",
            label: "Command",
            value:
              this.activeTool === "brush"
                ? PUZZLE_5D_ENGAGEMENT_TOOL_BRUSH_ID
                : this.activeTool === "fill"
                  ? PUZZLE_5D_ENGAGEMENT_TOOL_FILL_ID
                  : PUZZLE_5D_ENGAGEMENT_TOOL_SELECT_ID,
            options: toolPossibles.map((row) => ({ id: row.id, label: row.label })),
            onSelect: puzzle5dPlayCmd("engagementControlSelect", { windowId }),
          };
    return {
      sessionActive: this.activeTool === "brush" || this.activeTool === "fill",
      input: {
        id: "engagement-input",
        value: this.engagementInputByWindow[windowId] ?? "",
        placeholder: this.activeTool === "fill" ? "Fill" : this.activeTool === "brush" ? "Brush" : "Command",
        onChange: puzzle5dPlayCmd("engagementInput", { windowId }),
        onSubmit: puzzle5dPlayCmd("engagementSubmit", { windowId }),
        onAbort: puzzle5dPlayCmd("engagementAbort", { windowId }),
      },
      control,
      possibleEngagements: toolPossibles,
    };
  }

  getWindowKinds(): readonly WindowKindRuntime[] {
    const windowKinds = [
      new WindowKindRuntime(
        PUZZLE_5D_PLAY_2D_WINDOW_ID,
        PUZZLE_5D_PLAY_2D_WINDOW_LABEL,
        PUZZLE_5D_PLAY_2D_BODY_KEY,
        undefined,
        [
          { kind: "group", id: `${PUZZLE_5D_PLAY_2D_WINDOW_ID}-lod`, label: "LOD", children: [this.lod2dMeasure()] },
          this.brushMeasuresGroup(PUZZLE_5D_PLAY_2D_WINDOW_ID),
        ],
        this.windowEngagementFor(PUZZLE_5D_PLAY_2D_WINDOW_ID),
      ),
      new WindowKindRuntime(
        PUZZLE_5D_PLAY_3D_WINDOW_ID,
        PUZZLE_5D_PLAY_3D_WINDOW_LABEL,
        PUZZLE_5D_PLAY_3D_BODY_KEY,
        undefined,
        [
          { kind: "group", id: `${PUZZLE_5D_PLAY_3D_WINDOW_ID}-lod`, label: "LOD", children: this.lod3dMeasures() },
          this.brushMeasuresGroup(PUZZLE_5D_PLAY_3D_WINDOW_ID),
        ],
        this.windowEngagementFor(PUZZLE_5D_PLAY_3D_WINDOW_ID),
      ),
    ];
    for (const windowKind of windowKinds) {
      enforcePlaygroundWindowEngagementInput(windowKind.engagement, `Puzzle 5D play window "${windowKind.id}"`);
    }
    return windowKinds;
  }

  getFixtureCatalog(): PlaygroundFixtureCatalog {
    return { activeFixtureId: this.activeFixtureId, options: PUZZLE_5D_PLAY_FIXTURE_OPTIONS };
  }

  private loadFixtureById(fixtureId: string): void {
    const model = isPlaygroundNoFixtureId(fixtureId) ? puzzle5dPlayEmptyModel() : fixtureId === PUZZLE_5D_PLAY_FIXTURE_NAKAGIN_ID ? loadNakagin5dModel() : null;
    if (!model) return;
    this.puzzle5dStore.replaceModel(model);
    this.selected2d = new Set();
    this.selected3d = null;
    this.activeTool = "select";
    this.brushEngagementPossibles = [];
    this.hostBridge?.runHostCommand("setActiveTool", { tool: "select", prevTool: "select" });
    const snap = this.puzzle5dStore.read();
    this.camera2d = { ...snap.camera2d };
    this.camera3d = { ...snap.camera3d };
    this.rebuildShellMode();
    this.emit();
  }

  override run(command: string, args?: unknown): void {
    let changed = true;
    switch (command) {
      case "setActiveFixture": {
        const fixtureId = (args as { fixtureId?: string }).fixtureId ?? "";
        const nextId = isPlaygroundNoFixtureId(fixtureId) ? PLAYGROUND_NO_FIXTURE_ID : fixtureId;
        if (nextId === this.activeFixtureId) {
          changed = false;
          break;
        }
        this.activeFixtureId = nextId;
        this.loadFixtureById(nextId);
        changed = false;
        break;
      }
      case "set2dLodMode": {
        const value = (args as { value?: string }).value;
        if ((value === PUZZLE_2D_LOD_MODE_AUTOMATIC || (typeof value === "string" && isPuzzle2dDrawLodKind(value))) && this.lod2dMode !== value) this.lod2dMode = value as Puzzle2dLodModeKind;
        else changed = false;
        break;
      }
      case "set3dAutoLod": {
        const pressed = (args as { pressed?: boolean }).pressed;
        if (typeof pressed === "boolean" && this.automaticLod3d !== pressed) this.automaticLod3d = pressed;
        else changed = false;
        break;
      }
      case "set3dDepthLod": {
        const pressed = (args as { pressed?: boolean }).pressed;
        if (typeof pressed === "boolean" && this.depthVariableLod3d !== pressed) this.depthVariableLod3d = pressed;
        else changed = false;
        break;
      }
      case "set3dManualLod": {
        const value = (args as { value?: number }).value;
        if (typeof value === "number" && Number.isFinite(value)) {
          this.lod3dSlider = value;
          this.manualLod3d = lodFromSliderValue(value);
        } else changed = false;
        break;
      }
      case "set2dLodTag": {
        const lod = (args as { lod: Puzzle2dDrawLodKind }).lod;
        if (this.lod2dTag !== lod) this.lod2dTag = lod;
        else changed = false;
        break;
      }
      case "set3dLodTag": {
        const lod = (args as { lod: number }).lod;
        if (typeof lod === "number" && Number.isFinite(lod) && lod > 0) {
          this.lod3dTag = lod;
        }
        changed = false;
        break;
      }
      case "set2dSelection": {
        const ids = (args as { ids: readonly string[] }).ids;
        if (ids.length !== this.selected2d.size || ids.some((id) => !this.selected2d.has(id))) this.selected2d = new Set(ids);
        else changed = false;
        break;
      }
      case "set3dSelection": {
        const selected = (args as { objectIds: readonly string[] }).objectIds[0] ?? null;
        if (this.selected3d !== selected) this.selected3d = selected;
        else changed = false;
        break;
      }
      case "set2dCamera": {
        const camera = (args as { camera: CameraState }).camera;
        if (!sameCamera(this.camera2d, camera)) this.camera2d = { ...camera };
        else changed = false;
        break;
      }
      case "set3dCamera": {
        const camera = (args as { camera: CameraState }).camera;
        if (!sameCamera(this.camera3d, camera)) this.camera3d = { ...camera };
        else changed = false;
        break;
      }
      case "setGumballConfigToggle": {
        const key = (args as { key?: Puzzle3dGumballGroupKey }).key;
        if (!key || !PUZZLE_3D_GUMBALL_GROUPS.some((row) => row.key === key)) {
          changed = false;
          break;
        }
        this.gumballConfig = { ...this.gumballConfig, [key]: this.gumballConfig[key] === false };
        this.rebuildShellMode();
        break;
      }
      case "note2dConnect":
        this.connect2d += 1;
        break;
      case "note3dConnect":
        this.connect3d += 1;
        break;
      case "note2dProximity":
        this.proximity2d += 1;
        break;
      case "note3dProximity":
        this.proximity3d += 1;
        break;
      case "setActiveTool": {
        const tool = (args as { tool?: Puzzle5dActiveTool }).tool;
        if (tool === "select" || tool === "brush" || tool === "fill") {
          this.setPlayActiveTool(tool);
        } else {
          changed = false;
        }
        break;
      }
      case "addBrushPart": {
        const placement = args as Puzzle5dBrushPlacement;
        if (!placement?.partKind) {
          changed = false;
          break;
        }
        if (this.puzzle5dStore.applyBrushPlacement(placement)) {
          this.emit();
        }
        changed = false;
        break;
      }
      case "deleteSelection": {
        if (this.puzzle5dStore.applySelectionDelete()) {
          this.selected2d = new Set();
          this.selected3d = null;
          this.emit();
        }
        changed = false;
        break;
      }
      case "setFillCount": {
        const count = Number((args as { count?: number }).count);
        if (!Number.isFinite(count)) {
          changed = false;
          break;
        }
        this.puzzle5dStore.applyFillCount(count);
        this.rebuildShellMode();
        this.emit();
        changed = false;
        break;
      }
      case "setBrushFlushDistance": {
        const distance = Number((args as { value?: number }).value);
        if (Number.isFinite(distance)) {
          this.brushFlushDistance = Math.max(PUZZLE_5D_BRUSH_FLUSH_DISTANCE_MIN, Math.min(PUZZLE_5D_BRUSH_FLUSH_DISTANCE_MAX, distance));
          this.hostBridge?.runHostCommand("setBrushFlushDistance", { distance: this.brushFlushDistance });
        } else {
          changed = false;
        }
        break;
      }
      case "setBrushOverlapBudget": {
        const value = Number((args as { value?: number }).value);
        if (Number.isFinite(value)) {
          this.brushOverlapBudget = Math.max(0, Math.min(BRUSH_PLACEMENT_OVERLAP_BUDGET_MAX, value));
          this.hostBridge?.runHostCommand("setBrushOverlapBudget", { value: this.brushOverlapBudget });
        } else {
          changed = false;
        }
        break;
      }
      case "pickBrushCandidate": {
        this.hostBridge?.runHostCommand(command, args);
        changed = false;
        break;
      }
      case "engagementPossibleSelect": {
        const { possibleId, windowId } = args as { possibleId?: string; windowId?: string };
        if (!possibleId) {
          changed = false;
          break;
        }
        if (possibleId === PUZZLE_5D_ENGAGEMENT_TOOL_BRUSH_ID) {
          this.setPlayActiveTool("brush");
        } else if (possibleId === PUZZLE_5D_ENGAGEMENT_TOOL_FILL_ID) {
          this.setPlayActiveTool("fill");
        } else if (possibleId === PUZZLE_5D_ENGAGEMENT_TOOL_SELECT_ID) {
          this.setPlayActiveTool("select");
        } else if (possibleId.startsWith("puzzle5d.brush.") || possibleId.startsWith("puzzle2d.brush.") || possibleId.startsWith("puzzle3d.brush.")) {
          this.hostBridge?.runHostCommand(command, args);
        } else {
          this.hostBridge?.runHostCommand(command, args);
        }
        if (windowId && windowId in this.engagementInputByWindow) {
          this.engagementInputByWindow = { ...this.engagementInputByWindow, [windowId]: "" };
        }
        changed = false;
        break;
      }
      case "engagementControlChange": {
        if (this.activeTool === "fill") {
          const value = Number((args as { value?: number }).value);
          if (Number.isFinite(value)) {
            this.puzzle5dStore.applyFillCount(value);
            this.rebuildShellMode();
            this.emit();
          }
        } else {
          this.hostBridge?.runHostCommand(command, args);
        }
        changed = false;
        break;
      }
      case "engagementControlSelect":
      case "engagementControlCommit":
        this.hostBridge?.runHostCommand(command, args);
        changed = false;
        break;
      case "engagementInput": {
        const { windowId, value } = args as { windowId?: string; value?: string };
        if (!windowId || !(windowId in this.engagementInputByWindow)) {
          changed = false;
          break;
        }
        this.engagementInputByWindow = { ...this.engagementInputByWindow, [windowId]: String(value ?? "") };
        break;
      }
      case "engagementSubmit": {
        const { windowId, value } = args as { windowId?: string; value?: string };
        if (!windowId || !(windowId in this.engagementInputByWindow)) {
          changed = false;
          break;
        }
        const token = String(value ?? this.engagementInputByWindow[windowId] ?? "")
          .trim()
          .toLowerCase();
        if (token === "brush") this.setPlayActiveTool("brush");
        else if (token === "fill") this.setPlayActiveTool("fill");
        else if (token === "select") this.setPlayActiveTool("select");
        else this.hostBridge?.runHostCommand(command, args);
        this.engagementInputByWindow = { ...this.engagementInputByWindow, [windowId]: "" };
        changed = false;
        break;
      }
      case "engagementAbort": {
        const { windowId } = args as { windowId?: string };
        if (!windowId || !(windowId in this.engagementInputByWindow)) {
          changed = false;
          break;
        }
        if (this.activeTool === "brush" || this.activeTool === "fill") {
          this.setPlayActiveTool("select");
        }
        this.hostBridge?.runHostCommand(command, args);
        this.engagementInputByWindow = { ...this.engagementInputByWindow, [windowId]: "" };
        changed = false;
        break;
      }
      default:
        if (command === "setSelectionMethod" || command === "setSelectionMode" || command === "toggleSelectionTarget") {
          this.hostBridge?.runHostCommand(command, args);
          this.rebuildShellMode();
          this.emit();
        } else if (
          command === "clearSelection" ||
          command === "selectAllSelection" ||
          command === "toggleGridSnap" ||
          command === "appendCircle" ||
          command === "appendRectangle" ||
          command === "toggleRedrawPlaying" ||
          command === "redrawHandlesOnce" ||
          command === "setBrushKindWeights" ||
          command === "setNodeKindWeight" ||
          command === "setHandleKindWeight" ||
          command === "setObjectKindWeight" ||
          command === "setVortexKindWeight"
        ) {
          this.hostBridge?.runHostCommand(command, args);
        }
        changed = false;
        break;
    }
    if (changed) {
      this.rebuildShellMode();
      this.emit();
    }
  }

  getSnapshot(): Puzzle5dPlaySnapshot {
    const model = this.puzzle5dStore.read();
    const fixture2d = project2d(model);
    const fixture3d = project3d(model);
    const toolbar = this.toolbarState();
    return {
      manifestLabel: model.label,
      fixture2d,
      fixture3d,
      selected2d: this.selected2d,
      camera2d: this.camera2d,
      camera3d: this.camera3d,
      selected3d: this.selected3d,
      gumballConfig: this.gumballConfig,
      lod3dTag: this.lod3dTag,
      lod2dTag: this.lod2dTag,
      lod2dProps: puzzle2dLodCanvasProps(this.lod2dMode),
      lod3dProps: puzzle3dLodCanvasProps({
        automaticLod: this.automaticLod3d,
        depthVariableLod: this.depthVariableLod3d,
        manualLod: this.manualLod3d,
      }),
      automaticLod3d: this.automaticLod3d,
      depthVariableLod3d: this.depthVariableLod3d,
      lod3dSlider: this.lod3dSlider,
      sharedKinds: sharedKindsFromMetas({ meta2d: fixture2d.meta, meta3d: fixture3d.meta }),
      kindCatalogs: model.kindCatalogs,
      connect2d: this.connect2d,
      connect3d: this.connect3d,
      proximity2d: this.proximity2d,
      proximity3d: this.proximity3d,
      activeTool: this.activeTool,
      brushFlushDistance: this.brushFlushDistance,
      brushOverlapBudget: this.brushOverlapBudget,
      fillCount: this.puzzle5dStore.getSnapshot().fillCount,
      fillBuildDone: this.puzzle5dStore.getSnapshot().fillBuildDone,
      selectionMethod: toolbar.puzzle2dSelectionMethod,
      selectionMode: toolbar.puzzle2dSelectionMode,
    };
  }
}
//#endregion 🔖Controller

//#region 🔖Puzzle5dPlayRuntime
export function buildPuzzle5dPlayAppRuntime(controller: Puzzle5dPlayShellController): AppRuntime {
  const app = new AppRuntime(
    PUZZLE_5D_PLAY_APP_ID,
    "Puzzle 5d play",
    undefined,
    controller,
    createDefaultLayout([PUZZLE_5D_PLAY_2D_WINDOW_ID, PUZZLE_5D_PLAY_3D_WINDOW_ID], "row", [50, 50], [PUZZLE_5D_PLAY_2D_WINDOW_LABEL, PUZZLE_5D_PLAY_3D_WINDOW_LABEL]) as never,
    controller.getWindowKinds(),
  );
  app.defaultModeId = controller.mainMode.id;
  app.addMode(controller.mainMode);
  app.panelTabs = [];
  return app;
}

export function buildPuzzle5dPlayRuntime(initialPanelVisibility?: { leftSidePanel: boolean; rightSidePanel: boolean }): Platform {
  const runtime = new Platform({ initialPanelVisibility });
  const controller = new Puzzle5dPlayShellController(runtime.commandBus, () => runtime.notify());
  runtime.addApp(buildPuzzle5dPlayAppRuntime(controller));
  return runtime;
}

/** @emoji 🛝 Puzzle 5d play harness as a single {@link Playground} instance. */
export class Playground5d extends Playground {
  readonly id = PUZZLE_5D_PLAY_APP_ID;
  readonly keybindings: readonly PlaygroundKeybinding[] = [
    { key: "Delete", controllerId: PUZZLE_5D_PLAY_CONTROLLER_ID, command: "deleteSelection" },
    { key: "Backspace", controllerId: PUZZLE_5D_PLAY_CONTROLLER_ID, command: "deleteSelection" },
  ];

  createRuntime(): Platform {
    return buildPuzzle5dPlayRuntime();
  }

  registerBodies(): void {
    /* window bodies registered with surface hosts in {@link registerPuzzle5dPlaySurfaceHosts} */
  }
}
//#endregion 🔖Puzzle5dPlayRuntime

//#region 🔖DeclarativeBodies
export function buildPuzzle5d2dDeclarativeBody(ctx: WindowBodyViewContext): UiNode {
  const ctrl = puzzle5dControllerFromContext(ctx);
  const snap = ctrl?.getSnapshot();
  if (!snap?.fixture2d) return { type: "text", value: "Invalid 2d fixture" };
  return buildPuzzle2dWindowBody(PUZZLE_5D_PLAY_2D_SURFACE_ID, PUZZLE_5D_PLAY_CONTROLLER_ID, PUZZLE_5D_PLAY_2D_WINDOW_ID);
}

export function buildPuzzle5d3dDeclarativeBody(ctx: WindowBodyViewContext): UiNode {
  const ctrl = puzzle5dControllerFromContext(ctx);
  const snap = ctrl?.getSnapshot();
  if (!snap?.fixture3d) return { type: "text", value: "Invalid 3d fixture" };
  return buildPuzzle3dWindowBody(PUZZLE_5D_PLAY_3D_SURFACE_ID, PUZZLE_5D_PLAY_CONTROLLER_ID);
}
//#endregion 🔖DeclarativeBodies

//#region 🧪Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  describe("puzzle 5d play hierarchy", () => {
    it("buildPuzzle5dPlayKindsTree exposes draggable flat and volume palette rows", () => {
      const runtime = buildPuzzle5dPlayRuntime();
      const controller = runtime.getActiveApp()?.controller as Puzzle5dPlayShellController;
      expect(controller).toBeTruthy();
      const tree = buildPuzzle5dPlayKindsTree(controller!.getSnapshot());
      expect(tree.type).toBe("tree");
      const flatNodes = tree.sections.find((section) => section.id === "puzzle-5d-play-kinds.2d.nodes");
      const volumeObjects = tree.sections.find((section) => section.id === "puzzle-5d-play-kinds.3d.objects");
      expect(flatNodes?.items?.some((row) => row.draggable === true && row.dragData)).toBe(true);
      expect(volumeObjects?.items?.some((row) => row.draggable === true && row.dragData)).toBe(true);
    });

    it("puzzle5dFixturePaletteTreeDragController routes flat and volume palette rows", () => {
      const runtime = buildPuzzle5dPlayRuntime();
      const controller = runtime.getActiveApp()?.controller as Puzzle5dPlayShellController;
      const tree = buildPuzzle5dPlayKindsTree(controller!.getSnapshot());
      const dragByItemId = collectUiTreeItemDragData(tree.sections);
      const dragController = puzzle5dFixturePaletteTreeDragController(dragByItemId);
      const flatRow = tree.sections.find((section) => section.id === "puzzle-5d-play-kinds.2d.nodes")?.items?.find((row) => row.dragData);
      const volumeRow = tree.sections.find((section) => section.id === "puzzle-5d-play-kinds.3d.objects")?.items?.find((row) => row.dragData);
      expect(flatRow?.dragData?.[PUZZLE_2D_FIXTURE_DRAG_V1_MIME]).toBeTruthy();
      expect(volumeRow?.dragData?.[FIXTURE_DRAG_V1_MIME]).toBeTruthy();
      expect(dragController.pointerPaletteDrag?.readEncodedDragPayload(flatRow!.dragData!)).toBeTruthy();
      expect(dragController.pointerPaletteDrag?.readEncodedDragPayload(volumeRow!.dragData!)).toBeTruthy();
    });

    it("buildPuzzle5dPlayHierarchySections includes 2d and 3d branches", () => {
      const runtime = buildPuzzle5dPlayRuntime();
      const controller = runtime.getActiveApp()?.controller as Puzzle5dPlayShellController;
      expect(controller).toBeTruthy();
      const tree = buildPuzzle5dPlayHierarchySections(controller!.getSnapshot(), {
        onSelect2d: () => {},
        onSelect3dObject: () => {},
        onSelect3dVortex: () => {},
        onSelect3dAttraction: () => {},
      });
      const sectionLabels = tree.sections.map((section) => section.label);
      expect(sectionLabels.some((label) => label?.startsWith("2d ·"))).toBe(true);
      expect(sectionLabels.some((label) => label?.startsWith("3d ·"))).toBe(true);
    });
  });

  describe("puzzle 5d play fixtures", () => {
    it("parses nakagin 2d and 3d fixtures", () => {
      const fixture2d = parsePuzzle2dFixtureV1(nakagin2dJson as unknown);
      const fixture3d = parseFixtureV1(nakagin3dJson as unknown);
      expect(fixture2d?.nodes.length).toBeGreaterThan(0);
      expect(fixture3d?.objects.length).toBeGreaterThan(0);
    });
    it("parses nakagin unified puzzle 5d v1", () => {
      const model = parseV1(nakagin5dJson as unknown);
      expect(model?.schema).toBe("puzzle.5d/v1");
      expect(model?.parts.length).toBeGreaterThan(0);
    });
    it("regenerates nakagin 5d fixture when REGENERATE_NAKAGIN_5D=1", async () => {
      if (process.env.REGENERATE_NAKAGIN_5D !== "1") return;
      const fixture2d = parsePuzzle2dFixtureV1(nakagin2dJson as unknown);
      const fixture3d = parseFixtureV1(nakagin3dJson as unknown);
      expect(fixture2d).toBeTruthy();
      expect(fixture3d).toBeTruthy();
      const model = {
        ...compose5d(fixture2d!, fixture3d!),
        label: "Nakagin capsule tower",
        meta: {
          description: "Unified puzzle 5d source for Nakagin play; 2d and 3d views project from this model.",
        },
      };
      const { writeFile } = await import("node:fs/promises");
      const { join } = await import("node:path");
      const outPath = join(process.cwd(), "../fixture/nakagin-capsule-tower.5d.json");
      await writeFile(outPath, `${JSON.stringify(model, null, 2)}\n`, "utf8");
      expect(model.parts.length).toBeGreaterThan(0);
    });
    it("shared kinds merge metas like the play harness", () => {
      const sk = sharedKindsFromMetas({
        meta2d: undefined,
        meta3d: { kindCompatibility: [{ source: "u", target: "v" }] },
      });
      expect(sk.kindCompatibility?.length).toBeGreaterThan(0);
    });
    it("activates brush via engagement submit", () => {
      const runtime = buildPuzzle5dPlayRuntime();
      const controller = runtime.getActiveApp()?.controller as Puzzle5dPlayShellController;
      expect(controller).toBeTruthy();
      controller.run("engagementSubmit", { windowId: PUZZLE_5D_PLAY_2D_WINDOW_ID, value: "Brush" });
      expect(controller.getActiveTool()).toBe("brush");
    });

    it("addBrushPart grows unified store parts when placement is valid", () => {
      const runtime = buildPuzzle5dPlayRuntime();
      const controller = runtime.getActiveApp()?.controller as Puzzle5dPlayShellController;
      const host = controller.puzzle5dStore.read().parts[0];
      const hostAnchor = host?.anchors[0]?.id;
      if (!host?.id || !hostAnchor) return;
      const peerKind = controller.puzzle5dStore.read().parts.find((part) => part.partKind && part.id !== host.id)?.partKind;
      if (!peerKind) return;
      const before = controller.puzzle5dStore.read().parts.length;
      controller.run("addBrushPart", {
        partKind: peerKind,
        sourceAnchorFullId: `${host.id}:${hostAnchor}`,
        aspect3d: {
          targetVortexFullId: `${host.id}:${hostAnchor}`,
          objectKindId: peerKind,
          sourceVortexIndex: 0,
          origin: [2, 0, 0],
          orientation: [0, 0, 0, 1],
          objectId: "brush-test-part",
        },
      });
      expect(controller.puzzle5dStore.read().parts.length).toBeGreaterThan(before);
      const placed = controller.puzzle5dStore.read().parts.find((part) => part.id === "brush-test-part");
      expect(placed?.puzzle2d).toBeTruthy();
      expect(placed?.puzzle3d).toBeTruthy();
    });

    it("builds declarative 2d and 3d canvas-only bodies", () => {
      const wb = buildPuzzle5dPlayRuntime();
      const body2d = buildPuzzle5d2dDeclarativeBody({
        runtime: wb,
        windowKindId: PUZZLE_5D_PLAY_2D_WINDOW_ID,
        bodyKey: PUZZLE_5D_PLAY_2D_BODY_KEY,
        activeModeId: "main",
        generation: 0,
      });
      const body3d = buildPuzzle5d3dDeclarativeBody({
        runtime: wb,
        windowKindId: PUZZLE_5D_PLAY_3D_WINDOW_ID,
        bodyKey: PUZZLE_5D_PLAY_3D_BODY_KEY,
        activeModeId: "main",
        generation: 0,
      });
      expect(body2d).toEqual(buildPuzzle2dWindowBody(PUZZLE_5D_PLAY_2D_SURFACE_ID, PUZZLE_5D_PLAY_CONTROLLER_ID, PUZZLE_5D_PLAY_2D_WINDOW_ID));
      expect(body3d).toEqual(buildPuzzle3dWindowBody(PUZZLE_5D_PLAY_3D_SURFACE_ID, PUZZLE_5D_PLAY_CONTROLLER_ID));
    });
  });
}
//#endregion 🧪Tests

//#region 🔖Boot
if (
  typeof document !== "undefined" &&
  document.getElementById("root") != null &&
  !import.meta.vitest &&
  import.meta.env.PUZZLE_PLAY_ENTRY === "5d"
) {
  void (async () => {
    await import("./globals.css");
    const { boot5dPlay } = await import("@framework/playground/renderer/react/puzzle/5d");
    boot5dPlay(new Playground5d());
  })();
}
//#endregion 🔖Boot
