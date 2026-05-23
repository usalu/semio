// #region 🧲Header
// 💻 elements/lib/react/scene/play/index.ts — Scene play on `@framework/playground`: Nakagin fixture, LOD measures, selection/filter tools (no React).
// #endregion 🧲Header

import {
  AppRuntime,
  CommandBus,
  Controller,
  Expertise,
  ModeRuntime,
  Playground,
  ProductRuntime,
  WindowKindRuntime,
  buildPlaygroundBrowseFilterTools,
  buildPlaygroundBrowseSelectionTools,
  buildScene3dWindowBody,
  createStackLayout,
  playgroundTreePanelRootItems,
  registerSidePanelBody,
  registerWindowBody,
  type CommandDescriptor,
  type SideTabSpec,
  type ToolItem,
  type UiNode,
  type UiTreeItemNode,
  type UiTreeSectionNode,
  type WindowBodyViewContext,
  type WindowMeasure,
} from "@framework/playground";

import {
  DEFAULT_MANUAL_LOD,
  SCENE_LOD_SLIDER_MAX,
  SCENE_LOD_SLIDER_MIN,
  applyRelocateToSceneFixture,
  fixturePoseFingerprint,
  fixtureStateFingerprint,
  formatSceneLod,
  lodFromSliderValue,
  parseFixtureV1,
  parseVortexFullId,
  sceneLodCanvasProps,
  sceneVortexFullId,
  sliderValueFromLod,
  type AttractionProps,
  type CameraState,
  type EdgeKindCatalogEntry,
  type FixtureObjectV1,
  type FixtureV1,
  type HandleKindCatalogEntry,
  type KindCatalogBundle,
  type KindCompatEntry,
  type NodeKindCatalogEntry,
  type RelocateMode,
  type RelocatePayload,
  type SelectionMode,
  type SelectionSnapshot,
  type VortexProps,
  type WireKindCatalogEntry,
} from "../react/index.tsx";
import nakaginSceneFixtureJson from "./fixtures/nakagin-capsule-tower.scene.json";

//#region 🧾Meta
function parseKindCompatibility(meta: Record<string, unknown> | undefined): readonly KindCompatEntry[] {
  if (!meta || typeof meta !== "object") return [];
  const arr = (meta as { kindCompatibility?: unknown }).kindCompatibility;
  if (!Array.isArray(arr)) return [];
  const out: KindCompatEntry[] = [];
  for (const entry of arr) {
    if (!entry || typeof entry !== "object") continue;
    const e = entry as Record<string, unknown>;
    const source = typeof e.source === "string" ? e.source.trim() : "";
    const target = typeof e.target === "string" ? e.target.trim() : "";
    if (!source || !target) continue;
    const specificity =
      e.specificity === "general" || e.specificity === "node" || e.specificity === "edge" || e.specificity === "handle" || e.specificity === "wire" || e.specificity === "object" || e.specificity === "attraction" ? e.specificity : undefined;
    out.push({
      source,
      target,
      ...(e.bidirectional === true ? { bidirectional: true } : {}),
      ...(e.important === true ? { important: true } : {}),
      ...(specificity ? { specificity } : {}),
    });
  }
  return out;
}

function parseKindCatalogs(meta: Record<string, unknown> | undefined): KindCatalogBundle | undefined {
  const kc = meta?.kindCatalogs;
  if (!kc || typeof kc !== "object") return undefined;
  return kc as KindCatalogBundle;
}
//#endregion 🧾Meta

//#region 🖥️Surface
export const LS_THEME = "puzzle.2d-play.surface.theme";
export const LS_DEVICE = "puzzle.2d-play.surface.device";
export const LS_EXPERTISE = "puzzle.2d-play.surface.expertise";

export function parseStoredTheme(raw: string | null) {
  if (raw === "light" || raw === "dark" || raw === "system") return raw;
  return "system";
}

export function parseStoredDevice(raw: string | null) {
  if (raw === "desktop" || raw === "tablet" || raw === "mobile") return raw;
  return "desktop";
}

export function parseStoredExpertise(raw: string | null) {
  if (raw === Expertise.BEGINNER || raw === Expertise.NORMAL || raw === Expertise.EXPERT) return raw;
  return Expertise.NORMAL;
}
//#endregion 🖥️Surface

//#region 🎬Play
export const PLAY_APP_ID = "elements-puzzle-3d-play";
export const PUZZLE_3D_PLAY_WINDOW_ID = "puzzle-3d-main";
export const PUZZLE_3D_PLAY_WINDOW_LABEL = "Puzzle 3d";
export const PUZZLE_3D_PLAY_BODY_KEY = "puzzle.3d.play.window";
export const PUZZLE_3D_PLAY_CONTROLLER_ID = "puzzle-3d-play";
export const PUZZLE_3D_PLAY_SCENE_SURFACE_ID = "puzzle.3d.play.scene/v1";
export const PUZZLE_3D_PLAY_INSPECTOR_TAB_ID = "puzzle-3d-play-inspector";
export const PUZZLE_3D_PLAY_SETTINGS_TAB_ID = "puzzle-3d-play-settings";
export const PUZZLE_3D_PLAY_HIERARCHY_TAB_ID = "puzzle-3d-play-hierarchy";
export const PUZZLE_3D_PLAY_KINDS_TAB_ID = "puzzle-3d-play-kinds";
export const PUZZLE_3D_PLAY_HIERARCHY_BODY_KEY = "puzzle.3d.play.hierarchy";
export const PUZZLE_3D_PLAY_KINDS_BODY_KEY = "puzzle.3d.play.kinds";
export const PUZZLE_3D_PLAY_INSPECTOR_BODY_KEY = "puzzle.3d.play.inspector";
export const PUZZLE_3D_PLAY_SETTINGS_BODY_KEY = "puzzle.3d.play.settings";
export const PUZZLE_3D_PLAY_ICON_HIERARCHY = "puzzle.3d-play.icon.hierarchy";
export const PUZZLE_3D_PLAY_ICON_KINDS = "puzzle.3d-play.icon.kinds";
export const PUZZLE_3D_PLAY_ICON_INSPECTOR = "puzzle.3d-play.icon.inspector";
export const PUZZLE_3D_PLAY_ICON_SETTINGS = "puzzle.3d-play.icon.settings";
//#endregion 🎬Play

function scenePlayCmd(command: string, args?: Record<string, unknown>): CommandDescriptor {
	return { controllerId: PUZZLE_3D_PLAY_CONTROLLER_ID, command, args: args as never };
}

function scenePlaySelectObjectCommand(objectId: string): CommandDescriptor {
	return scenePlayCmd("setSelection", { selection: { objectIds: [objectId], vortexIds: [], attractionIds: [] } });
}

function scenePlaySelectVortexCommand(vortexFullId: string): CommandDescriptor {
	return scenePlayCmd("setSelection", { selection: { objectIds: [], vortexIds: [vortexFullId], attractionIds: [] } });
}

function scenePlaySelectAttractionCommand(attractionId: string): CommandDescriptor {
	return scenePlayCmd("setSelection", { selection: { objectIds: [], vortexIds: [], attractionIds: [attractionId] } });
}

export { parseKindCatalogs, parseKindCompatibility };

//#region 🔖Puzzle3dPlaySelection
/** @emoji 🎯 Play harness selection: objects, vortex full ids, and attractions. */
export interface Puzzle3dPlaySelection extends SelectionSnapshot {
  readonly attractionIds: readonly string[];
}

export const PUZZLE_3D_PLAY_EMPTY_SELECTION: Puzzle3dPlaySelection = {
  objectIds: [],
  vortexIds: [],
  attractionIds: [],
};

/** @emoji 📸 Stable idle snapshot for {@link useSyncExternalStore} when no controller is mounted. */
export const PUZZLE_3D_PLAY_IDLE_SNAPSHOT: Puzzle3dPlaySnapshot = {
  fixture: null,
  fixtureRevision: 0,
  lodProps: sceneLodCanvasProps({ automaticLod: true, depthVariableLod: false, manualLod: DEFAULT_MANUAL_LOD }),
  lodTag: DEFAULT_MANUAL_LOD,
  lodSlider: sliderValueFromLod(DEFAULT_MANUAL_LOD),
  automaticLod: true,
  depthVariableLod: false,
  relocateMode: "translate",
  selection: PUZZLE_3D_PLAY_EMPTY_SELECTION,
  selectedId: null,
  selectedLabel: null,
  selectionMode: "single",
  proximityRadius: 24,
  chunkSize: 256,
  gridFactor: 10,
  showLodGrid: false,
  gridSnapEnabled: true,
  proximityCount: 0,
  connectCount: 0,
  indirectCount: 0,
  compatibleObjectsCount: 0,
  targetRingCount: 0,
};

export { sceneVortexFullId };

/** @emoji 🏷️ Tree/inspector label: trimmed fixture label, else fallback id. */
export function scenePlayFixtureRowLabel(label: string | undefined, fallbackId: string): string {
  const trimmed = label?.trim();
  return trimmed && trimmed.length > 0 ? trimmed : fallbackId;
}

/** @emoji 🎯 Resolved selection label for play chrome (objects, vortices, attractions). */
export function scenePlaySelectionLabel(fixture: FixtureV1 | null, selection: Puzzle3dPlaySelection): string | null {
  if (!fixture) return null;
  if (selection.attractionIds[0]) {
    return selection.attractionIds[0];
  }
  if (selection.vortexIds[0]) {
    const { objectId, vortexId } = parseVortexFullId(selection.vortexIds[0]);
    const object = fixture.objects.find((row) => row.id === objectId);
    const vortex = object?.vortices.find((row) => row.id === vortexId || sceneVortexFullId(objectId, row.id) === selection.vortexIds[0]);
    return scenePlayFixtureRowLabel(vortex?.label, selection.vortexIds[0]);
  }
  if (selection.objectIds[0]) {
    const object = fixture.objects.find((row) => row.id === selection.objectIds[0]);
    return scenePlayFixtureRowLabel(object?.label, selection.objectIds[0]);
  }
  return null;
}

/** @emoji 🗑️ Removes an object and any attractions touching it or its vortices. */
export function deleteSceneObjectFromFixture(fixture: FixtureV1, objectId: string): FixtureV1 {
  const removedVortexFullIds = new Set<string>();
  for (const object of fixture.objects) {
    if (object.id !== objectId) {
      continue;
    }
    for (const vortex of object.vortices) {
      removedVortexFullIds.add(sceneVortexFullId(objectId, vortex.id));
    }
  }
  return {
    ...fixture,
    objects: fixture.objects.filter((object) => object.id !== objectId),
    attractions: fixture.attractions.filter((attraction) => {
      const sourceObjectId = parseVortexFullId(attraction.attracting).objectId;
      const targetObjectId = parseVortexFullId(attraction.attracted).objectId;
      if (sourceObjectId === objectId || targetObjectId === objectId) {
        return false;
      }
      return !removedVortexFullIds.has(attraction.attracting) && !removedVortexFullIds.has(attraction.attracted);
    }),
  };
}

/** @emoji 🗑️ Removes one vortex and stale attractions that referenced it. */
export function deleteSceneVortexFromFixture(fixture: FixtureV1, vortexFullId: string): FixtureV1 {
  const { objectId } = parseVortexFullId(vortexFullId);
  return {
    ...fixture,
    objects: fixture.objects.map((object) =>
      object.id !== objectId
        ? object
        : {
            ...object,
            vortices: object.vortices.filter((vortex) => sceneVortexFullId(objectId, vortex.id) !== vortexFullId),
          },
    ),
    attractions: fixture.attractions.filter((attraction) => attraction.attracting !== vortexFullId && attraction.attracted !== vortexFullId),
  };
}

/** @emoji 🗑️ Drops a single attraction row. */
export function deleteSceneAttractionFromFixture(fixture: FixtureV1, attractionId: string): FixtureV1 {
  return {
    ...fixture,
    attractions: fixture.attractions.filter((attraction) => attraction.id !== attractionId),
  };
}

function patchSceneObject(objects: readonly FixtureObjectV1[], objectId: string, patch: (object: FixtureObjectV1) => FixtureObjectV1): FixtureObjectV1[] {
  return objects.map((object) => (object.id === objectId ? patch(object) : object));
}

/** @emoji ✏️ Updates fields on one fixture object. */
export function updateSceneObjectInFixture(fixture: FixtureV1, objectId: string, patch: Partial<Omit<FixtureObjectV1, "id" | "vortices">>): FixtureV1 {
  return {
    ...fixture,
    objects: patchSceneObject(fixture.objects, objectId, (object) => ({ ...object, ...patch })),
  };
}

/** @emoji ✏️ Updates one vortex on an object. */
export function updateSceneVortexInFixture(fixture: FixtureV1, vortexFullId: string, patch: Partial<VortexProps>): FixtureV1 {
  const { objectId, vortexId } = parseVortexFullId(vortexFullId);
  return {
    ...fixture,
    objects: patchSceneObject(fixture.objects, objectId, (object) => ({
      ...object,
      vortices: object.vortices.map((vortex) => {
        const fullId = sceneVortexFullId(objectId, vortex.id);
        if (fullId !== vortexFullId && vortex.id !== vortexId) {
          return vortex;
        }
        return { ...vortex, ...patch, id: vortex.id };
      }),
    })),
  };
}

/** @emoji ✏️ Updates one attraction row. */
export function updateSceneAttractionInFixture(fixture: FixtureV1, attractionId: string, patch: Partial<AttractionProps>): FixtureV1 {
  return {
    ...fixture,
    attractions: fixture.attractions.map((attraction) => (attraction.id === attractionId ? { ...attraction, ...patch } : attraction)),
  };
}

/** @emoji 📷 True when two camera states match within epsilon (avoids redundant fixture writes). */
export function cameraStateNearEqual(a: CameraState, b: CameraState, epsilon = 1e-3): boolean {
  for (let i = 0; i < 3; i += 1) {
    if (Math.abs(a.position[i]! - b.position[i]!) > epsilon) return false;
    if (Math.abs(a.target[i]! - b.target[i]!) > epsilon) return false;
  }
  return Math.abs(a.zoom - b.zoom) <= epsilon;
}

/** @emoji 📷 Writes camera fields on the fixture; returns the same reference when unchanged. */
export function updateSceneCameraInFixture(fixture: FixtureV1, camera: Partial<CameraState>): FixtureV1 {
  const nextCamera: CameraState = { ...fixture.camera, ...camera };
  if (cameraStateNearEqual(fixture.camera, nextCamera)) {
    return fixture;
  }
  return { ...fixture, camera: nextCamera };
}

/** @emoji 🎯 Maps {@link SelectionSnapshot} to play selection (attractions filled separately). */
export function selectionSnapshotToPlaySelection(snap: SelectionSnapshot, attractionIds: readonly string[] = []): Puzzle3dPlaySelection {
  return {
    objectIds: snap.objectIds,
    vortexIds: snap.vortexIds,
    attractionIds,
  };
}

/** @emoji 🎯 True when two selection snapshots match (skips redundant shell updates). */
export function scenePlaySelectionEqual(a: Puzzle3dPlaySelection, b: Puzzle3dPlaySelection): boolean {
  if (a.objectIds.length !== b.objectIds.length || a.vortexIds.length !== b.vortexIds.length) {
    return false;
  }
  if (a.attractionIds.length !== b.attractionIds.length) {
    return false;
  }
  for (let i = 0; i < a.objectIds.length; i += 1) {
    if (a.objectIds[i] !== b.objectIds[i]) {
      return false;
    }
  }
  for (let i = 0; i < a.vortexIds.length; i += 1) {
    if (a.vortexIds[i] !== b.vortexIds[i]) {
      return false;
    }
  }
  for (let i = 0; i < a.attractionIds.length; i += 1) {
    if (a.attractionIds[i] !== b.attractionIds[i]) {
      return false;
    }
  }
  return true;
}

//#region 🔖Puzzle3dPlayHierarchy
/** @emoji 🌳 Nested workbench tree: Scene → Objects → Vortices; Attractions sibling group. */
export function buildPuzzle3dPlayHierarchyTree(fixture: FixtureV1 | null, selection: Puzzle3dPlaySelection): UiNode {
  if (!fixture) {
    return playgroundTreePanelRootItems("puzzle-3d-play-hierarchy.root", [{ id: "puzzle-3d-play-hierarchy.invalid", label: "Invalid scene fixture" }]);
  }
  const selectedObjects = new Set(selection.objectIds);
  const selectedVortices = new Set(selection.vortexIds);
  const selectedAttractions = new Set(selection.attractionIds);
  const objectItems: UiTreeItemNode[] = fixture.objects.map((object) => {
    const vortexItems: UiTreeItemNode[] = object.vortices.map((vortex) => {
      const fullId = sceneVortexFullId(object.id, vortex.id);
      return {
        id: `puzzle-3d-play-hierarchy.vortex.${fullId}`,
        label: scenePlayFixtureRowLabel(vortex.label, fullId),
        selected: selectedVortices.has(fullId),
        command: scenePlaySelectVortexCommand(fullId),
      };
    });
    const vorticesGroup: UiTreeItemNode = {
      id: `puzzle-3d-play-hierarchy.object.${object.id}.vortices`,
      label: "Vortices",
      defaultOpen: true,
      items: vortexItems.length ? vortexItems : [{ id: `puzzle-3d-play-hierarchy.object.${object.id}.vortices.empty`, label: "(none)" }],
    };
    return {
      id: `puzzle-3d-play-hierarchy.object.${object.id}`,
      label: scenePlayFixtureRowLabel(object.label, object.id),
      selected: selectedObjects.has(object.id),
      defaultOpen: true,
      command: scenePlaySelectObjectCommand(object.id),
      items: [vorticesGroup],
    };
  });
  const objectsGroup: UiTreeItemNode = {
    id: "puzzle-3d-play-hierarchy.objects",
    label: "Objects",
    defaultOpen: true,
    items: objectItems.length ? objectItems : [{ id: "puzzle-3d-play-hierarchy.objects.empty", label: "(none)" }],
  };
  const attractionItems: UiTreeItemNode[] = fixture.attractions.map((attraction) => ({
    id: `puzzle-3d-play-hierarchy.attraction.${attraction.id}`,
    label: attraction.id,
    description: `${attraction.attracting} → ${attraction.attracted}`,
    selected: selectedAttractions.has(attraction.id),
    command: scenePlaySelectAttractionCommand(attraction.id),
  }));
  const attractionsGroup: UiTreeItemNode = {
    id: "puzzle-3d-play-hierarchy.attractions",
    label: "Attractions",
    defaultOpen: true,
    items: attractionItems.length ? attractionItems : [{ id: "puzzle-3d-play-hierarchy.attractions.empty", label: "(none)" }],
  };
  const sceneRoot: UiTreeItemNode = {
    id: "puzzle-3d-play-hierarchy.scene",
    label: "Scene",
    defaultOpen: true,
    items: [objectsGroup, attractionsGroup],
  };
  return playgroundTreePanelRootItems("puzzle-3d-play-hierarchy.root", [sceneRoot]);
}
//#endregion 🔖Puzzle3dPlayHierarchy

//#region 🔖Puzzle3dPlayKinds
type Puzzle3dPlayKindCatalogEntry = NodeKindCatalogEntry | HandleKindCatalogEntry | WireKindCatalogEntry | EdgeKindCatalogEntry;

function scenePlayKindCatalogEntryLabel(entry: Puzzle3dPlayKindCatalogEntry): string {
  const display = entry.label?.trim() || entry.name?.trim();
  return display && display.length > 0 ? display : entry.id;
}

function scenePlayKindCatalogSection(sectionId: string, label: string, entries: readonly Puzzle3dPlayKindCatalogEntry[] | undefined): UiTreeSectionNode | null {
  if (!entries?.length) {
    return null;
  }
  const items: UiTreeItemNode[] = [...entries]
    .sort((a, b) => scenePlayKindCatalogEntryLabel(a).localeCompare(scenePlayKindCatalogEntryLabel(b)))
    .map((entry) => ({
      id: `${sectionId}.${entry.id}`,
      label: scenePlayKindCatalogEntryLabel(entry),
      description: entry.id,
    }));
  return { id: sectionId, label, defaultOpen: true, items };
}

/** @emoji 🏷️ Workbench kinds tab: Objects, Vortices, Attractions (and Edges when catalogued). */
export function buildPuzzle3dPlayKindsTree(catalogs: KindCatalogBundle | undefined): UiNode {
  const sections = [
    scenePlayKindCatalogSection("puzzle-3d-play-kinds.objects", "Objects", catalogs?.nodes),
    scenePlayKindCatalogSection("puzzle-3d-play-kinds.vortices", "Vortices", catalogs?.handles),
    scenePlayKindCatalogSection("puzzle-3d-play-kinds.attractions", "Attractions", catalogs?.wires),
    scenePlayKindCatalogSection("puzzle-3d-play-kinds.edges", "Edges", catalogs?.edges),
  ].filter((section): section is UiTreeSectionNode => section !== null);
  if (!sections.length) {
    return {
      type: "tree",
      sections: [
        {
          id: "puzzle-3d-play-kinds.empty",
          label: "Kinds",
          defaultOpen: true,
          items: [{ id: "puzzle-3d-play-kinds.empty.msg", label: "No kind catalogs in this fixture" }],
        },
      ],
    };
  }
  return { type: "tree", sections };
}
//#endregion 🔖Puzzle3dPlayKinds

/** @emoji 🎯 Primary object id for relocate / legacy e2e hooks. */
export function primaryPuzzle3dPlayObjectId(selection: Puzzle3dPlaySelection): string | null {
  if (selection.objectIds[0]) {
    return selection.objectIds[0];
  }
  if (selection.vortexIds[0]) {
    return parseVortexFullId(selection.vortexIds[0]).objectId;
  }
  return null;
}
//#endregion 🔖Puzzle3dPlaySelection

//#region 🔖Puzzle3dPlayController
const PUZZLE_3D_PLAY_KINDS = ["object", "vortex", "attraction"] as const;
type Puzzle3dPlayPickKind = (typeof PUZZLE_3D_PLAY_KINDS)[number];

function scenePlayKindLabel(kind: Puzzle3dPlayPickKind): string {
  if (kind === "object") return "Objects";
  if (kind === "vortex") return "Vortices";
  return "Attractions";
}

/** @emoji 🎬 Playground scene play controller: fixture, LOD, selection/filter tools, and interaction counters. */
export class Puzzle3dPlayShellController extends Controller {
  readonly mainMode = new ModeRuntime("main", "Scene", undefined);
  readonly selectableKinds: Record<Puzzle3dPlayPickKind, boolean> = { object: true, vortex: true, attraction: true };
  readonly visibleKinds: Record<Puzzle3dPlayPickKind, boolean> = { object: true, vortex: true, attraction: true };
  private fixture: FixtureV1 | null;
  private fixtureRevision: number;
  private automaticLod: boolean;
  private depthVariableLod: boolean;
  private manualLod: number;
  private lodSlider: number;
  private lodTag: number;
  private relocateMode: RelocateMode;
  private selection: Puzzle3dPlaySelection;
  private selectionMode: SelectionMode;
  private proximityRadius: number;
  private chunkSize: number;
  private gridFactor: number;
  private showLodGrid: boolean;
  private gridSnapEnabled: boolean;
  private proximityCount: number;
  private connectCount: number;
  private indirectCount: number;
  private compatibleObjectsCount: number;
  private targetRingCount: number;
  private snapshotListeners = new Set<() => void>();
  private snapshotCache: Puzzle3dPlaySnapshot | null = null;

  constructor(commandBus: CommandBus, hostNotify: () => void) {
    super(PUZZLE_3D_PLAY_CONTROLLER_ID, commandBus, hostNotify);
    this.fixture = parseFixtureV1(nakaginSceneFixtureJson as unknown);
    this.fixtureRevision = 0;
    this.automaticLod = true;
    this.depthVariableLod = false;
    this.manualLod = DEFAULT_MANUAL_LOD;
    this.lodSlider = sliderValueFromLod(DEFAULT_MANUAL_LOD);
    this.lodTag = DEFAULT_MANUAL_LOD;
    this.relocateMode = "translate";
    this.selection = PUZZLE_3D_PLAY_EMPTY_SELECTION;
    this.selectionMode = "single";
    this.proximityRadius = 24;
    this.chunkSize = 256;
    this.gridFactor = 10;
    this.showLodGrid = false;
    this.gridSnapEnabled = true;
    this.proximityCount = 0;
    this.connectCount = 0;
    this.indirectCount = 0;
    this.compatibleObjectsCount = 0;
    this.targetRingCount = 0;
    this.rebuildShellMode();
    this.rebuildSnapshotCache();
  }

  /** @emoji 🔔 Subscribes to snapshot-only updates (selection, fixture, lod) without shell generation bumps. */
  subscribeSnapshot(listener: () => void): () => void {
    this.snapshotListeners.add(listener);
    return () => this.snapshotListeners.delete(listener);
  }

  private rebuildSnapshotCache(): void {
    this.snapshotCache = {
      fixture: this.fixture,
      fixtureRevision: this.fixtureRevision,
      lodProps: sceneLodCanvasProps({
        automaticLod: this.automaticLod,
        depthVariableLod: this.depthVariableLod,
        manualLod: this.manualLod,
      }),
      lodTag: this.lodTag,
      lodSlider: this.lodSlider,
      automaticLod: this.automaticLod,
      depthVariableLod: this.depthVariableLod,
      relocateMode: this.relocateMode,
      selection: this.selection,
      selectedId: primaryPuzzle3dPlayObjectId(this.selection),
      selectedLabel: scenePlaySelectionLabel(this.fixture, this.selection),
      selectionMode: this.selectionMode,
      proximityRadius: this.proximityRadius,
      chunkSize: this.chunkSize,
      gridFactor: this.gridFactor,
      showLodGrid: this.showLodGrid,
      gridSnapEnabled: this.gridSnapEnabled,
      proximityCount: this.proximityCount,
      connectCount: this.connectCount,
      indirectCount: this.indirectCount,
      compatibleObjectsCount: this.compatibleObjectsCount,
      targetRingCount: this.targetRingCount,
    };
  }

  private notifySnapshot(): void {
    this.rebuildSnapshotCache();
    for (const listener of this.snapshotListeners) {
      listener();
    }
  }

  /** @emoji 🐚 Rebuilds mode chrome and bumps shell generation (toolbar, window measures). */
  private syncShell(): void {
    this.rebuildSnapshotCache();
    this.rebuildShellMode();
    this.emit();
  }

  getFixture(): FixtureV1 | null {
    return this.fixture;
  }

  getFixtureRevision(): number {
    return this.fixtureRevision;
  }

  patchFixture(updater: (prev: FixtureV1) => FixtureV1): void {
    if (!this.fixture) {
      return;
    }
    const prev = this.fixture;
    const next = updater(prev);
    if (next === prev) {
      return;
    }
    this.fixture = next;
    const structureChanged = fixtureStateFingerprint(next) !== fixtureStateFingerprint(prev);
    if (structureChanged) {
      this.fixtureRevision += 1;
    }
    const poseChanged = fixturePoseFingerprint(next) !== fixturePoseFingerprint(prev);
    if (structureChanged || poseChanged) {
      this.notifySnapshot();
    }
  }

  /** @emoji ✋ Persists a gumball relocate on the fixture (pose-only; no React emit). */
  patchRelocate(payload: RelocatePayload, attractingByObjectId?: ReadonlyMap<string, readonly string[]>): void {
    if (!this.fixture) {
      return;
    }
    const next = applyRelocateToSceneFixture(this.fixture, payload, attractingByObjectId);
    if (next === this.fixture) {
      return;
    }
    this.fixture = next;
  }

  /** @emoji 📷 Persists orbit camera on the fixture without bumping structure revision or re-emitting React state. */
  setCamera(camera: Partial<CameraState>): void {
    if (!this.fixture) {
      return;
    }
    const next = updateSceneCameraInFixture(this.fixture, camera);
    if (next === this.fixture) {
      return;
    }
    this.fixture = next;
  }

  private lodMeasures(): readonly WindowMeasure[] {
    return [
      {
        kind: "toggle",
        id: `${PUZZLE_3D_PLAY_WINDOW_ID}-auto`,
        label: "LOD",
        text: "Auto zoom",
        pressed: this.automaticLod,
        onChange: { controllerId: PUZZLE_3D_PLAY_CONTROLLER_ID, command: "setAutoLod" },
      },
      {
        kind: "toggle",
        id: `${PUZZLE_3D_PLAY_WINDOW_ID}-depth`,
        text: "Depth-variable",
        pressed: this.depthVariableLod,
        onChange: { controllerId: PUZZLE_3D_PLAY_CONTROLLER_ID, command: "setDepthLod" },
      },
      {
        kind: "slider",
        id: `${PUZZLE_3D_PLAY_WINDOW_ID}-lod`,
        label: formatSceneLod(this.lodTag),
        value: this.lodSlider,
        min: SCENE_LOD_SLIDER_MIN,
        max: SCENE_LOD_SLIDER_MAX,
        step: 1,
        onChange: { controllerId: PUZZLE_3D_PLAY_CONTROLLER_ID, command: "setManualLod" },
      },
    ];
  }

  private rebuildShellMode(): void {
    this.mainMode.windowKinds = [new WindowKindRuntime(PUZZLE_3D_PLAY_WINDOW_ID, PUZZLE_3D_PLAY_WINDOW_LABEL, PUZZLE_3D_PLAY_BODY_KEY, undefined, this.lodMeasures())];
    const relocateTools: ToolItem[] = (["translate", "rotate", "scale"] as const).map((mode, order) => ({
      id: `scene.relocate.${mode}`,
      kind: "toggle" as const,
      text: mode.charAt(0).toUpperCase() + mode.slice(1),
      order,
      pressed: this.relocateMode === mode,
      controllerId: PUZZLE_3D_PLAY_CONTROLLER_ID,
      command: "setRelocateMode",
      args: { mode },
    }));
    this.mainMode.tools = {
      selection: buildPlaygroundBrowseSelectionTools(PUZZLE_3D_PLAY_KINDS, scenePlayKindLabel, this.selectableKinds, PUZZLE_3D_PLAY_CONTROLLER_ID),
      filter: buildPlaygroundBrowseFilterTools(PUZZLE_3D_PLAY_KINDS, scenePlayKindLabel, this.visibleKinds, PUZZLE_3D_PLAY_CONTROLLER_ID),
      actions: relocateTools,
    };
  }

  private filterSelectionByPlaygroundKinds(selection: Puzzle3dPlaySelection): Puzzle3dPlaySelection {
    return {
      objectIds: this.selectableKinds.object && this.visibleKinds.object ? [...selection.objectIds] : [],
      vortexIds: this.selectableKinds.vortex && this.visibleKinds.vortex ? [...selection.vortexIds] : [],
      attractionIds: this.selectableKinds.attraction && this.visibleKinds.attraction ? [...selection.attractionIds] : [],
    };
  }

  override run(command: string, args?: unknown): void {
    switch (command) {
      case "setAutoLod": {
        const pressed = (args as { pressed?: boolean }).pressed;
        if (typeof pressed === "boolean") this.automaticLod = pressed;
        this.syncShell();
        return;
      }
      case "setDepthLod": {
        const pressed = (args as { pressed?: boolean }).pressed;
        if (typeof pressed === "boolean") this.depthVariableLod = pressed;
        this.syncShell();
        return;
      }
      case "setManualLod": {
        const value = (args as { value?: number }).value;
        if (typeof value === "number" && Number.isFinite(value)) {
          this.lodSlider = value;
          this.manualLod = lodFromSliderValue(value);
        }
        this.syncShell();
        return;
      }
      case "setEffectiveLod": {
        const lod = (args as { lod: number }).lod;
        if (typeof lod === "number" && Number.isFinite(lod) && lod > 0) {
          this.lodTag = lod;
          this.notifySnapshot();
        }
        return;
      }
      case "setRelocateMode": {
        const mode = (args as { mode: RelocateMode }).mode;
        if (mode === "translate" || mode === "rotate" || mode === "scale") this.relocateMode = mode;
        this.syncShell();
        return;
      }
      case "toggleSelectableKind": {
        const { kind } = args as { kind: Puzzle3dPlayPickKind };
        if (kind === "object" || kind === "vortex" || kind === "attraction") {
          this.selectableKinds[kind] = !this.selectableKinds[kind];
          this.selection = this.filterSelectionByPlaygroundKinds(this.selection);
        }
        this.syncShell();
        this.notifySnapshot();
        return;
      }
      case "toggleVisibleKind": {
        const { kind } = args as { kind: Puzzle3dPlayPickKind };
        if (kind === "object" || kind === "vortex" || kind === "attraction") {
          this.visibleKinds[kind] = !this.visibleKinds[kind];
          this.selection = this.filterSelectionByPlaygroundKinds(this.selection);
        }
        this.syncShell();
        this.notifySnapshot();
        return;
      }
      case "setSelection": {
        const next = (args as { selection: Puzzle3dPlaySelection }).selection;
        if (next && typeof next === "object") {
          const resolved = this.filterSelectionByPlaygroundKinds({
            objectIds: [...(next.objectIds ?? [])],
            vortexIds: [...(next.vortexIds ?? [])],
            attractionIds: [...(next.attractionIds ?? [])],
          });
          if (scenePlaySelectionEqual(this.selection, resolved)) {
            return;
          }
          this.selection = resolved;
          this.notifySnapshot();
        }
        return;
      }
      case "setSelectedId": {
        const id = (args as { id: string | null }).id;
        const resolved: Puzzle3dPlaySelection = id ? { objectIds: [id], vortexIds: [], attractionIds: [] } : PUZZLE_3D_PLAY_EMPTY_SELECTION;
        if (scenePlaySelectionEqual(this.selection, resolved)) {
          return;
        }
        this.selection = resolved;
        this.notifySnapshot();
        return;
      }
      case "noteSelection": {
        const snap = args as SelectionSnapshot & { attractionIds?: readonly string[] };
        const resolved = this.filterSelectionByPlaygroundKinds({
          objectIds: [...(snap.objectIds ?? [])],
          vortexIds: [...(snap.vortexIds ?? [])],
          attractionIds: snap.attractionIds !== undefined ? [...snap.attractionIds] : snap.objectIds.length === 0 && snap.vortexIds.length === 0 ? [] : [...this.selection.attractionIds],
        });
        if (scenePlaySelectionEqual(this.selection, resolved)) {
          return;
        }
        this.selection = resolved;
        this.notifySnapshot();
        return;
      }
      case "deleteSelection": {
        this.applyDeleteSelection();
        return;
      }
      case "patchSceneObjects": {
        const { objectIds, field, value } = args as {
          objectIds: readonly string[];
          field: "label" | "objectKind" | "origin" | "wormhole";
          value?: unknown;
        };
        if (!objectIds.length || !field) return;
        const patch: Partial<Omit<FixtureObjectV1, "id" | "vortices">> = {};
        if (field === "label" && typeof value === "string") patch.label = value;
        if (field === "objectKind" && typeof value === "string") patch.objectKind = value;
        if (field === "wormhole" && typeof value === "string") patch.wormhole = value === "true";
        if (field === "origin" && Array.isArray(value) && value.length === 3) {
          patch.origin = [Number(value[0]), Number(value[1]), Number(value[2])] as [number, number, number];
        }
        this.patchFixture((fixture) => {
          let next = fixture;
          for (const objectId of objectIds) {
            next = updateSceneObjectInFixture(next, objectId, patch);
          }
          return next;
        });
        return;
      }
      case "renameSceneObject": {
        const { oldId, value } = args as { oldId: string; value?: string };
        const trimmed = (typeof value === "string" ? value : "").trim();
        if (!trimmed || trimmed === oldId) return;
        this.patchFixture((fixture) => ({
          ...fixture,
          objects: fixture.objects.map((object) => (object.id === oldId ? { ...object, id: trimmed } : object)),
        }));
        this.selection = { objectIds: [trimmed], vortexIds: [], attractionIds: [] };
        this.notifySnapshot();
        return;
      }
      case "patchSceneVortex": {
        const { vortexFullId, field, value } = args as {
          vortexFullId: string;
          field: "vortexKind" | "position" | "radius";
          value?: unknown;
        };
        const patch: Partial<VortexProps> = {};
        if (field === "vortexKind" && typeof value === "string") patch.vortexKind = value;
        if (field === "position" && Array.isArray(value) && value.length === 3) {
          patch.position = [Number(value[0]), Number(value[1]), Number(value[2])] as [number, number, number];
        }
        if (field === "radius" && typeof value === "number") patch.radius = value;
        if (field === "radius" && typeof value === "string") {
          const parsed = Number(value);
          if (Number.isFinite(parsed)) patch.radius = parsed;
        }
        this.patchFixture((fixture) => updateSceneVortexInFixture(fixture, vortexFullId, patch));
        return;
      }
      case "patchSceneAttraction": {
        const { attractionId, field, value } = args as {
          attractionId: string;
          field: "attracting" | "attracted" | "attractionKind";
          value?: unknown;
        };
        const patch: Partial<AttractionProps> = {};
        if (field === "attracting" && typeof value === "string") patch.attracting = value.trim() as AttractionProps["attracting"];
        if (field === "attracted" && typeof value === "string") patch.attracted = value.trim() as AttractionProps["attracted"];
        if (field === "attractionKind" && typeof value === "string") patch.attractionKind = value;
        this.patchFixture((fixture) => updateSceneAttractionInFixture(fixture, attractionId, patch));
        return;
      }
      case "setSelectionMode": {
        const mode = ((args as { mode?: SelectionMode; value?: string }).mode ?? (args as { value?: string }).value) as SelectionMode;
        if (mode === "single" || mode === "additive" || mode === "subtractive" || mode === "toggle") {
          this.selectionMode = mode;
          this.notifySnapshot();
        }
        return;
      }
      case "setProximityRadius": {
        const value = Number((args as { value: number }).value);
        if (typeof value === "number" && Number.isFinite(value) && value > 0) {
          this.proximityRadius = value;
          this.notifySnapshot();
        }
        return;
      }
      case "setChunkSize": {
        const value = (args as { value: number }).value;
        if (typeof value === "number" && Number.isFinite(value) && value > 0) {
          this.chunkSize = value;
          this.notifySnapshot();
        }
        return;
      }
      case "setGridFactor": {
        const value = (args as { value: number }).value;
        if (typeof value === "number" && Number.isFinite(value) && value > 0) {
          this.gridFactor = value;
          this.notifySnapshot();
        }
        return;
      }
      case "setShowLodGrid": {
        const pressed = (args as { pressed?: boolean }).pressed;
        if (typeof pressed === "boolean") {
          this.showLodGrid = pressed;
          this.notifySnapshot();
        }
        return;
      }
      case "setGridSnapEnabled": {
        const pressed = (args as { pressed?: boolean }).pressed;
        if (typeof pressed === "boolean") {
          this.gridSnapEnabled = pressed;
          this.notifySnapshot();
        }
        return;
      }
      case "noteProximity":
        this.proximityCount += 1;
        this.notifySnapshot();
        return;
      case "noteConnect":
        this.connectCount += 1;
        this.notifySnapshot();
        return;
      case "noteIndirect":
        this.indirectCount += 1;
        this.notifySnapshot();
        return;
      case "noteCompatibleObjects":
        this.compatibleObjectsCount += 1;
        this.notifySnapshot();
        return;
      case "noteTargetRing":
        this.targetRingCount += 1;
        this.notifySnapshot();
        return;
      default:
        return;
    }
  }

  private applyDeleteSelection(): void {
    if (!this.fixture) {
      return;
    }
    const objectIds = [...this.selection.objectIds];
    const vortexIds = [...this.selection.vortexIds];
    const attractionIds = [...this.selection.attractionIds];
    if (objectIds.length === 0 && vortexIds.length === 0 && attractionIds.length === 0) {
      return;
    }
    this.patchFixture((fixture) => {
      let next = fixture;
      for (const objectId of objectIds) {
        next = deleteSceneObjectFromFixture(next, objectId);
      }
      for (const vortexFullId of vortexIds) {
        next = deleteSceneVortexFromFixture(next, vortexFullId);
      }
      for (const attractionId of attractionIds) {
        next = deleteSceneAttractionFromFixture(next, attractionId);
      }
      return next;
    });
    this.selection = PUZZLE_3D_PLAY_EMPTY_SELECTION;
    this.notifySnapshot();
  }

  getSnapshot(): Puzzle3dPlaySnapshot {
    if (!this.snapshotCache) {
      this.rebuildSnapshotCache();
    }
    return this.snapshotCache!;
  }
}

/** @emoji 📸 Host-consumed scene play state (no React/DOM). */
export interface Puzzle3dPlaySnapshot {
  readonly fixture: FixtureV1 | null;
  readonly fixtureRevision: number;
  readonly lodProps: ReturnType<typeof sceneLodCanvasProps>;
  readonly lodTag: number;
  readonly lodSlider: number;
  readonly automaticLod: boolean;
  readonly depthVariableLod: boolean;
  readonly relocateMode: RelocateMode;
  readonly selection: Puzzle3dPlaySelection;
  readonly selectedId: string | null;
  readonly selectedLabel: string | null;
  readonly selectionMode: SelectionMode;
  readonly proximityRadius: number;
  readonly chunkSize: number;
  readonly gridFactor: number;
  readonly showLodGrid: boolean;
  readonly gridSnapEnabled: boolean;
  readonly proximityCount: number;
  readonly connectCount: number;
  readonly indirectCount: number;
  readonly compatibleObjectsCount: number;
  readonly targetRingCount: number;
}

export function buildPuzzle3dPlayAppRuntime(controller: Puzzle3dPlayShellController): AppRuntime {
  const app = new AppRuntime(PLAY_APP_ID, "Scene play", undefined, controller, createStackLayout([PUZZLE_3D_PLAY_WINDOW_ID], [PUZZLE_3D_PLAY_WINDOW_LABEL]) as never, [
    new WindowKindRuntime(PUZZLE_3D_PLAY_WINDOW_ID, PUZZLE_3D_PLAY_WINDOW_LABEL, PUZZLE_3D_PLAY_BODY_KEY),
  ]);
  app.defaultModeId = controller.mainMode.id;
  app.addMode(controller.mainMode);
  app.leftTabs = [
    { id: PUZZLE_3D_PLAY_HIERARCHY_TAB_ID, iconId: PUZZLE_3D_PLAY_ICON_HIERARCHY, order: 0, bodyKey: PUZZLE_3D_PLAY_HIERARCHY_BODY_KEY },
    { id: PUZZLE_3D_PLAY_KINDS_TAB_ID, iconId: PUZZLE_3D_PLAY_ICON_KINDS, order: 1, bodyKey: PUZZLE_3D_PLAY_KINDS_BODY_KEY },
  ];
  app.rightTabs = [
    { id: PUZZLE_3D_PLAY_INSPECTOR_TAB_ID, iconId: PUZZLE_3D_PLAY_ICON_INSPECTOR, order: 0, bodyKey: PUZZLE_3D_PLAY_INSPECTOR_BODY_KEY },
    { id: PUZZLE_3D_PLAY_SETTINGS_TAB_ID, iconId: PUZZLE_3D_PLAY_ICON_SETTINGS, order: 1, bodyKey: PUZZLE_3D_PLAY_SETTINGS_BODY_KEY },
  ];
  return app;
}

/** @emoji 🚀 Creates a {@link ProductRuntime} with scene play app registered. */
export function buildPuzzle3dPlayRuntime(): ProductRuntime {
  const runtime = new ProductRuntime();
  const controller = new Puzzle3dPlayShellController(runtime.commandBus, () => runtime.notify());
  runtime.addApp(buildPuzzle3dPlayAppRuntime(controller));
  return runtime;
}

function sceneControllerFromContext(ctx: WindowBodyViewContext): Puzzle3dPlayShellController | undefined {
  return ctx.runtime.getActiveApp()?.controller as Puzzle3dPlayShellController | undefined;
}

/** @emoji 🧩 Declarative scene window: fullscreen scene3d only (relocate tools live on {@link ModeRuntime.tools}). */
export function buildPuzzle3dPlayDeclarativeBody(ctx: WindowBodyViewContext): UiNode {
  const ctrl = sceneControllerFromContext(ctx);
  if (!ctrl) {
    return { type: "text", value: "Missing scene controller" };
  }
  const snap = ctrl.getSnapshot();
  if (!snap.fixture) {
    return { type: "text", value: "Invalid scene fixture" };
  }
  return buildScene3dWindowBody(PUZZLE_3D_PLAY_SCENE_SURFACE_ID, PUZZLE_3D_PLAY_CONTROLLER_ID);
}

function scenePlayAllEqual<T>(values: readonly T[]): boolean {
  if (values.length <= 1) return true;
  const first = values[0];
  for (let i = 1; i < values.length; i += 1) {
    if (values[i] !== first) return false;
  }
  return true;
}

/** @emoji 🔎 Declarative inspector panel for scene play selection. */
export function buildPuzzle3dPlayInspectorBody(ctx: WindowBodyViewContext): UiNode {
  const ctrl = sceneControllerFromContext(ctx);
  const snap = ctrl?.getSnapshot();
  const fixture = snap?.fixture;
  if (!ctrl || !snap || !fixture) {
    return { type: "text", value: "Invalid scene fixture" };
  }
  const selection = snap.selection;
  const hasSelection = selection.objectIds.length > 0 || selection.vortexIds.length > 0 || selection.attractionIds.length > 0;
  const catalogs = parseKindCatalogs(fixture.meta);
  const nodeKinds = catalogs?.nodes ?? [];
  const handleKinds = catalogs?.handles ?? [];
  const wireKinds = catalogs?.wires ?? [];
  const children: UiNode[] = [
    {
      type: "section",
      id: "puzzle-3d-play-inspector.header",
      label: "Inspector",
      children: [
        {
          type: "text",
          value: `${selection.objectIds.length} objects · ${selection.vortexIds.length} vortices · ${selection.attractionIds.length} attractions`,
        },
        ...(hasSelection
          ? []
          : [{ type: "text", value: "Select objects, vortices, or attractions in the canvas or workbench hierarchy." }]),
        {
          type: "button",
          id: "puzzle-3d-play-inspector.delete",
          label: "Delete selection",
          command: scenePlayCmd("deleteSelection"),
        },
      ],
    },
  ];
  if (selection.objectIds.length > 0) {
    const objects = fixture.objects.filter((object) => selection.objectIds.includes(object.id));
    const labels = objects.map((object) => object.label ?? "");
    const labelUniform = scenePlayAllEqual(labels);
    const kinds = objects.map((object) => object.objectKind ?? "");
    const kindUniform = scenePlayAllEqual(kinds);
    const origins = objects.map((object) => object.origin);
    const originUniform = scenePlayAllEqual(origins);
    const objectFields: UiNode[] = [];
    if (selection.objectIds.length === 1) {
      objectFields.push({
        type: "field",
        id: "puzzle-3d-play-inspector.object.id",
        label: "Id",
        child: {
          type: "input",
          id: "puzzle-3d-play-inspector.object.id.input",
          inputKind: "text",
          value: selection.objectIds[0]!,
          commit: "blur",
          onChange: scenePlayCmd("renameSceneObject", { oldId: selection.objectIds[0] }),
        },
      });
    }
    objectFields.push(
      {
        type: "field",
        id: "puzzle-3d-play-inspector.object.label",
        label: "Label",
        child: {
          type: "input",
          id: "puzzle-3d-play-inspector.object.label.input",
          inputKind: "text",
          value: labelUniform ? (labels[0] ?? "") : "",
          placeholder: labelUniform ? undefined : "Mixed",
          onChange: scenePlayCmd("patchSceneObjects", { objectIds: selection.objectIds, field: "label" }),
        },
      },
      {
        type: "field",
        id: "puzzle-3d-play-inspector.object.kind",
        label: "Object kind",
        child: {
          type: "select",
          id: "puzzle-3d-play-inspector.object.kind.select",
          value: kindUniform ? (kinds[0] ?? "") : "",
          placeholder: kindUniform ? "kind" : "Mixed",
          items: nodeKinds.map((entry) => ({ value: entry.id, label: entry.label ?? entry.name ?? entry.id })),
          onChange: scenePlayCmd("patchSceneObjects", { objectIds: selection.objectIds, field: "objectKind" }),
        },
      },
      {
        type: "field",
        id: "puzzle-3d-play-inspector.object.origin",
        label: "Origin",
        child: {
          type: "vec3",
          id: "puzzle-3d-play-inspector.object.origin.vec3",
          value: originUniform ? (origins[0] as [number, number, number]) : null,
          onChange: scenePlayCmd("patchSceneObjects", { objectIds: selection.objectIds, field: "origin" }),
        },
      },
    );
    children.push({
      type: "section",
      id: "puzzle-3d-play-inspector.objects",
      label: `Objects (${selection.objectIds.length})`,
      children: objectFields,
    });
  }
  for (const vortexFullId of selection.vortexIds) {
    const { objectId, vortexId } = parseVortexFullId(vortexFullId);
    const object = fixture.objects.find((entry) => entry.id === objectId);
    const vortex = object?.vortices.find((entry) => sceneVortexFullId(objectId, entry.id) === vortexFullId || entry.id === vortexId);
    if (!object || !vortex) continue;
    children.push({
      type: "section",
      id: `puzzle-3d-play-inspector.vortex.${vortexFullId}`,
      label: scenePlayFixtureRowLabel(vortex.label, vortexFullId),
      children: [
        {
          type: "field",
          id: `puzzle-3d-play-inspector.vortex.kind.${vortexFullId}`,
          label: "Vortex kind",
          child: {
            type: "select",
            id: `puzzle-3d-play-inspector.vortex.kind.select.${vortexFullId}`,
            value: vortex.vortexKind ?? "",
            items: handleKinds.map((entry) => ({ value: entry.id, label: entry.label ?? entry.name ?? entry.id })),
            onChange: scenePlayCmd("patchSceneVortex", { vortexFullId, field: "vortexKind" }),
          },
        },
        {
          type: "field",
          id: `puzzle-3d-play-inspector.vortex.position.${vortexFullId}`,
          label: "Position",
          child: {
            type: "vec3",
            id: `puzzle-3d-play-inspector.vortex.position.vec3.${vortexFullId}`,
            value: vortex.position as [number, number, number],
            onChange: scenePlayCmd("patchSceneVortex", { vortexFullId, field: "position" }),
          },
        },
        {
          type: "field",
          id: `puzzle-3d-play-inspector.vortex.radius.${vortexFullId}`,
          label: "Radius",
          child: {
            type: "input",
            id: `puzzle-3d-play-inspector.vortex.radius.input.${vortexFullId}`,
            inputKind: "number",
            value: String(vortex.radius ?? 0.35),
            onChange: scenePlayCmd("patchSceneVortex", { vortexFullId, field: "radius" }),
          },
        },
      ],
    });
  }
  for (const attractionId of selection.attractionIds) {
    const attraction = fixture.attractions.find((entry) => entry.id === attractionId);
    if (!attraction) continue;
    children.push({
      type: "section",
      id: `puzzle-3d-play-inspector.attraction.${attractionId}`,
      label: attraction.id,
      children: [
        {
          type: "field",
          id: `puzzle-3d-play-inspector.attraction.attracting.${attractionId}`,
          label: "Attracting",
          child: {
            type: "input",
            id: `puzzle-3d-play-inspector.attraction.attracting.input.${attractionId}`,
            inputKind: "text",
            value: attraction.attracting,
            onChange: scenePlayCmd("patchSceneAttraction", { attractionId, field: "attracting" }),
          },
        },
        {
          type: "field",
          id: `puzzle-3d-play-inspector.attraction.attracted.${attractionId}`,
          label: "Attracted",
          child: {
            type: "input",
            id: `puzzle-3d-play-inspector.attraction.attracted.input.${attractionId}`,
            inputKind: "text",
            value: attraction.attracted,
            onChange: scenePlayCmd("patchSceneAttraction", { attractionId, field: "attracted" }),
          },
        },
      ],
    });
  }
  return { type: "stack", direction: "vertical", gap: "tight", padding: "standard", children };
}

/** @emoji ⚙️ Declarative settings panel for scene play. */
export function buildPuzzle3dPlaySettingsBody(ctx: WindowBodyViewContext): UiNode {
  const ctrl = sceneControllerFromContext(ctx);
  const snap = ctrl?.getSnapshot();
  if (!snap) {
    return { type: "text", value: "Missing scene controller" };
  }
  return {
    type: "stack",
    direction: "vertical",
    gap: "tight",
    padding: "standard",
    children: [
      {
        type: "section",
        id: "puzzle-3d-play-settings.root",
        label: "Scene options",
        children: [
          {
            type: "field",
            id: "puzzle-3d-play-settings.selectionMode",
            label: "Selection mode",
            child: {
              type: "select",
              id: "puzzle-3d-play-settings.selectionMode.select",
              value: snap.selectionMode,
              items: [
                { value: "single", label: "single" },
                { value: "additive", label: "additive" },
                { value: "subtractive", label: "subtractive" },
                { value: "toggle", label: "toggle" },
              ],
              onChange: scenePlayCmd("setSelectionMode"),
            },
          },
          {
            type: "field",
            id: "puzzle-3d-play-settings.proximityRadius",
            label: "Proximity radius",
            child: {
              type: "input",
              id: "puzzle-3d-play-settings.proximityRadius.input",
              inputKind: "number",
              value: String(snap.proximityRadius),
              onChange: scenePlayCmd("setProximityRadius", { value: 0 }),
            },
          },
          {
            type: "field",
            id: "puzzle-3d-play-settings.chunkSize",
            label: "Chunk size",
            child: {
              type: "input",
              id: "puzzle-3d-play-settings.chunkSize.input",
              inputKind: "number",
              value: String(snap.chunkSize),
              onChange: scenePlayCmd("setChunkSize", { value: 0 }),
            },
          },
          {
            type: "field",
            id: "puzzle-3d-play-settings.gridFactor",
            label: "Grid factor",
            child: {
              type: "input",
              id: "puzzle-3d-play-settings.gridFactor.input",
              inputKind: "number",
              value: String(snap.gridFactor),
              onChange: scenePlayCmd("setGridFactor", { value: 0 }),
            },
          },
          {
            type: "keyValue",
            entries: [
              { label: "connect", value: String(snap.connectCount) },
              { label: "proximity", value: String(snap.proximityCount) },
              { label: "indirect", value: String(snap.indirectCount) },
              { label: "compatible", value: String(snap.compatibleObjectsCount) },
              { label: "target ring", value: String(snap.targetRingCount) },
            ],
          },
        ],
      },
    ],
  };
}

export function buildPuzzle3dPlayHierarchyPanelBody(ctx: WindowBodyViewContext): UiNode {
  const ctrl = sceneControllerFromContext(ctx);
  const snap = ctrl?.getSnapshot();
  return buildPuzzle3dPlayHierarchyTree(snap?.fixture ?? null, snap?.selection ?? PUZZLE_3D_PLAY_EMPTY_SELECTION);
}

export function buildPuzzle3dPlayKindsPanelBody(ctx: WindowBodyViewContext): UiNode {
  const ctrl = sceneControllerFromContext(ctx);
  const snap = ctrl?.getSnapshot();
  const catalogs = snap?.fixture ? parseKindCatalogs(snap.fixture.meta) : undefined;
  return buildPuzzle3dPlayKindsTree(catalogs);
}

/** @emoji 🛝 Scene play harness as a single {@link Playground} instance. */
export class Playground3d extends Playground {
  readonly id = PLAY_APP_ID;
  readonly initialPanelVisibility = { leftSidePanel: true, rightSidePanel: true };
  readonly keybindings = [
    { key: "Delete", controllerId: PUZZLE_3D_PLAY_CONTROLLER_ID, command: "deleteSelection" },
    { key: "Backspace", controllerId: PUZZLE_3D_PLAY_CONTROLLER_ID, command: "deleteSelection" },
  ];

  createRuntime(): ProductRuntime {
    return buildPuzzle3dPlayRuntime();
  }

  registerBodies(): void {
    registerWindowBody(PUZZLE_3D_PLAY_BODY_KEY, buildPuzzle3dPlayDeclarativeBody);
    registerSidePanelBody(PUZZLE_3D_PLAY_HIERARCHY_BODY_KEY, buildPuzzle3dPlayHierarchyPanelBody);
    registerSidePanelBody(PUZZLE_3D_PLAY_KINDS_BODY_KEY, buildPuzzle3dPlayKindsPanelBody);
    registerSidePanelBody(PUZZLE_3D_PLAY_INSPECTOR_BODY_KEY, buildPuzzle3dPlayInspectorBody);
    registerSidePanelBody(PUZZLE_3D_PLAY_SETTINGS_BODY_KEY, buildPuzzle3dPlaySettingsBody);
  }

}
//#endregion 🔖Puzzle3dPlayController

//#region 🧪Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("scene play fixture", () => {
    it("parses nakagin fixture", () => {
      const f = parseFixtureV1(nakaginSceneFixtureJson as unknown);
      expect(f?.domain).toBe("architecture");
      expect(f?.attractions).toEqual([]);
      expect(f?.objects.length).toBeGreaterThan(0);
    });

    it("builds canonical vortex full ids", () => {
      expect(sceneVortexFullId("obj", "vx")).toBe("obj:vx");
      expect(sceneVortexFullId("obj", "obj:vx")).toBe("obj:vx");
    });

    it("stores nakagin vortex positions in type-local CAD space", () => {
      const f = parseFixtureV1(nakaginSceneFixtureJson as unknown);
      const o = f?.objects.find((obj) => obj.id === "01890804-66f2-4544-98f0-b6f0c0615492");
      const v = o?.vortices.find((vx) => vx.id.endsWith(":link"));
      expect(v?.position[0]).toBeCloseTo(-1.3, 5);
      expect(v?.position[1]).toBeCloseTo(-1.25, 5);
      expect(v?.position[2]).toBeCloseTo(0, 5);
    });

    it("patchFixture bumps revision only when structure changes", () => {
      const bus = new CommandBus();
      const wb = new ProductRuntime();
      const ctrl = new Puzzle3dPlayShellController(bus, () => wb.notify());
      const base = ctrl.getFixture();
      expect(base).not.toBeNull();
      const revisionBefore = ctrl.getFixtureRevision();
      ctrl.patchFixture((fixture) => ({
        ...fixture,
        objects: fixture.objects.map((object, index) => (index === 0 ? { ...object, origin: [object.origin[0]! + 1, object.origin[1]!, object.origin[2]!] as const } : object)),
      }));
      expect(ctrl.getFixtureRevision()).toBe(revisionBefore);
      ctrl.patchFixture((fixture) => ({
        ...fixture,
        objects: fixture.objects.slice(0, -1),
      }));
      expect(ctrl.getFixtureRevision()).toBe(revisionBefore + 1);
    });

    it("noteSelection notifies snapshot listeners without shell generation", () => {
      const trackingBus = new CommandBus();
      const trackingWb = new ProductRuntime();
      let shellNotifyCount = 0;
      const trackingCtrl = new Puzzle3dPlayShellController(trackingBus, () => {
        shellNotifyCount += 1;
      });
      let snapshotCount = 0;
      const unsubscribe = trackingCtrl.subscribeSnapshot(() => {
        snapshotCount += 1;
      });
      trackingCtrl.run("noteSelection", { objectIds: ["a"], vortexIds: [] });
      expect(snapshotCount).toBe(1);
      expect(shellNotifyCount).toBe(0);
      trackingCtrl.run("noteSelection", { objectIds: ["a"], vortexIds: [] });
      expect(snapshotCount).toBe(1);
      trackingCtrl.run("noteSelection", { objectIds: ["b"], vortexIds: [] });
      expect(snapshotCount).toBe(2);
      expect(shellNotifyCount).toBe(0);
      unsubscribe();
    });

    it("setAutoLod still bumps shell generation", () => {
      const trackingBus = new CommandBus();
      let shellNotifyCount = 0;
      const trackingCtrl = new Puzzle3dPlayShellController(trackingBus, () => {
        shellNotifyCount += 1;
      });
      trackingCtrl.run("setAutoLod", { pressed: true });
      expect(shellNotifyCount).toBe(1);
    });

    it("deleteSelection removes selected fixture rows and clears selection", () => {
      const bus = new CommandBus();
      const wb = new ProductRuntime();
      const ctrl = new Puzzle3dPlayShellController(bus, () => wb.notify());
      const before = ctrl.getSnapshot().fixture;
      expect(before).not.toBeNull();
      const target = before!.objects[0]!;
      const countBefore = before!.objects.length;
      ctrl.run("setSelection", {
        selection: { objectIds: [target.id], vortexIds: [], attractionIds: [] },
      });
      ctrl.run("deleteSelection");
      const snap = ctrl.getSnapshot();
      expect(snap.fixture?.objects.some((object) => object.id === target.id)).toBe(false);
      expect(snap.fixture?.objects.length).toBe(countBefore - 1);
      expect(snap.selection).toEqual(PUZZLE_3D_PLAY_EMPTY_SELECTION);
    });

    it("deleteSceneObjectFromFixture removes child vortices and stale attractions", () => {
      const base = parseFixtureV1({
        schema: "puzzle.3d.fixture/v1",
        camera: { position: [0, 0, 0], target: [0, 0, 1], zoom: 1 },
        attractions: [
          { id: "t1", attracting: "a:v1", attracted: "b:v2" },
          { id: "t2", attracting: "b:v2", attracted: "c:v3" },
        ],
        objects: [
          { id: "a", meshUrl: "/m.glb", origin: [0, 0, 0], vortices: [{ id: "v1", position: [0, 0, 0] }] },
          { id: "b", meshUrl: "/m.glb", origin: [1, 0, 0], vortices: [{ id: "v2", position: [0, 0, 0] }] },
          { id: "c", meshUrl: "/m.glb", origin: [2, 0, 0], vortices: [{ id: "v3", position: [0, 0, 0] }] },
        ],
      });
      expect(base).not.toBeNull();
      const next = deleteSceneObjectFromFixture(base!, "b");
      expect(next.objects.map((object) => object.id)).toEqual(["a", "c"]);
      expect(next.attractions).toEqual([]);
    });

    it("scenePlaySelectionLabel resolves object and vortex fixture labels", () => {
      const fixture = parseFixtureV1({
        schema: "puzzle.3d.fixture/v1",
        camera: { position: [0, 0, 0], target: [0, 0, 1], zoom: 1 },
        attractions: [],
        objects: [
          {
            id: "a",
            label: "Alpha",
            meshUrl: "/m.glb",
            origin: [0, 0, 0],
            vortices: [{ id: "v1", label: "Handle A", position: [0, 0, 0] }],
          },
        ],
      });
      expect(scenePlaySelectionLabel(fixture, { objectIds: ["a"], vortexIds: [], attractionIds: [] })).toBe("Alpha");
      expect(scenePlaySelectionLabel(fixture, { objectIds: [], vortexIds: ["a:v1"], attractionIds: [] })).toBe("Handle A");
    });

    it("buildPuzzle3dPlayHierarchySections nests objects, vortices, and attractions", () => {
      const fixture = parseFixtureV1({
        schema: "puzzle.3d.fixture/v1",
        camera: { position: [0, 0, 0], target: [0, 0, 1], zoom: 1 },
        attractions: [{ id: "t1", attracting: "a:v1", attracted: "b:v2" }],
        objects: [
          {
            id: "a",
            label: "Alpha",
            meshUrl: "/m.glb",
            origin: [0, 0, 0],
            vortices: [{ id: "v1", label: "Handle A", position: [0, 0, 0] }],
          },
          { id: "b", label: "Beta", meshUrl: "/m.glb", origin: [1, 0, 0], vortices: [{ id: "v2", label: "Handle B", position: [0, 0, 0] }] },
        ],
      });
      expect(fixture).not.toBeNull();
      const tree = buildPuzzle3dPlayHierarchyTree(fixture, PUZZLE_3D_PLAY_EMPTY_SELECTION);
      const sceneRoot = tree.sections[0]?.items?.[0];
      expect(sceneRoot?.label).toBe("Scene");
      const objectsGroup = sceneRoot?.items?.find((row) => row.label === "Objects");
      expect(objectsGroup?.items?.length).toBe(2);
      const firstObject = objectsGroup?.items?.[0];
      expect(firstObject?.label).toBe("Alpha");
      expect(firstObject?.items?.[0]?.label).toBe("Vortices");
      expect(firstObject?.items?.[0]?.items?.[0]?.label).toBe("Handle A");
      expect(firstObject?.items?.[0]?.items?.[0]?.id).toBe("puzzle-3d-play-hierarchy.vortex.a:v1");
      const attractionsGroup = sceneRoot?.items?.find((row) => row.label === "Attractions");
      expect(attractionsGroup?.items?.[0]?.id).toBe("puzzle-3d-play-hierarchy.attraction.t1");
    });

    it("buildPuzzle3dPlayKindsSections lists object, vortex, and attraction kind categories", () => {
      const catalogs = parseKindCatalogs({
        kindCatalogs: {
          nodes: [{ id: "capsule", label: "Capsule", name: "Capsule" }],
          handles: [{ id: "core circular top", label: "Core circular top", name: "Core circular top" }],
          wires: [{ id: "board.wire.link", label: "Link", name: "Link" }],
        },
      });
      const tree = buildPuzzle3dPlayKindsTree(catalogs);
      expect(tree.sections.map((section) => section.label)).toEqual(["Objects", "Vortices", "Attractions"]);
      expect(tree.sections[0]?.items?.[0]?.label).toBe("Capsule");
    });

    it("declarative window body is a lone scene3d surface", () => {
      const bus = new CommandBus();
      const wb = new ProductRuntime();
      const ctrl = new Puzzle3dPlayShellController(bus, () => wb.notify());
      wb.addApp(buildPuzzle3dPlayAppRuntime(ctrl));
      const tree = buildPuzzle3dPlayDeclarativeBody({
        runtime: wb,
        windowKindId: PUZZLE_3D_PLAY_WINDOW_ID,
        bodyKey: PUZZLE_3D_PLAY_BODY_KEY,
        activeModeId: "main",
        generation: wb.generation,
      });
      expect(tree).toEqual(buildScene3dWindowBody(PUZZLE_3D_PLAY_SCENE_SURFACE_ID, PUZZLE_3D_PLAY_CONTROLLER_ID));
    });
  });
}
//#endregion 🧪Tests
