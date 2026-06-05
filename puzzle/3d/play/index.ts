// #region 🧲Header
// 💻 puzzle/3d/play/index.ts — Puzzle 3D play on `@framework/playground/core`: Nakagin fixture, LOD measures, selection/filter tools (no React).
// #endregion 🧲Header

import { DEFAULT_GUMBALL_CONFIG, bootstrapElementsSurfaceChromeDocument, formatNumber, type GumballConfig } from "@ui/react";
import {
  AppRuntime,
  CommandBus,
  Controller,
  Store,
  Expertise,
  ModeRuntime,
  Playground,
  PLAYGROUND_NO_FIXTURE_ID,
  type PlaygroundFixtureCatalog,
  type PlaygroundFixtureHost,
  isPlaygroundNoFixtureId,
  Platform,
  WindowKindRuntime,
  buildPlaygroundBrowseFilterTools,
  buildPlaygroundBrowseSelectionTools,
  buildPuzzle3dWindowBody,
  createStackLayout,
  createNamedLayout,
  createWindowLayout,
  playgroundTreePanelRootItems,
  platformFromViewContext,
  type WindowTemplate,
  registerSidePanelBody,
  registerWindowBody,
  type CommandDescriptor,
  type SideTabSpec,
  type ToolItem,
  type UiNode,
  type UiSectionNode,
  type UiTreeItemNode,
  type UiTreeNode,
  type UiTreeSectionNode,
  uiDeclarativeSectionsToTree,
  type WindowBodyViewContext,
  enforcePlaygroundWindowEngagementInput,
  windowEngagementsEqual,
  type WindowEngagement,
  type WindowMeasure,
  normalizeKindWeightGroup,
  syncKindWeightMap,
  type KindWeightMap,
} from "@framework/playground/core";

import {
  DEFAULT_MANUAL_LOD,
  PUZZLE_3D_LOD_SLIDER_MAX,
  PUZZLE_3D_LOD_SLIDER_MIN,
  BRUSH_PLACEMENT_OVERLAP_BUDGET_MAX,
  BRUSH_PLACEMENT_OVERLAP_BUDGET_STEP,
  DEFAULT_BRUSH_PLACEMENT_OVERLAP_BUDGET,
  applyBrushPlacementToFixture,
  applyBrushFillPlacementsToFixture,
  brushMeshUrlsForFillSession,
  buildBrushFillSequence,
  createBrushFillSequenceStepper,
  PUZZLE_3D_FILL_COUNT_MAX,
  puzzle3dBrushKindWeightsRef,
  puzzle3dBrushMeshRootForFill,
  applyRelocateToFixture,
  applyObjectKindToFixtureObject,
  fixtureAppearanceFingerprint,
  fixturePoseFingerprint,
  fixtureStateFingerprint,
  formatLod,
  lodFromSliderValue,
  cameraStateNearEqual,
  computeOrbitCameraViewState,
  createOrbitCameraViewTemplates,
  ORBIT_CAMERA_VIEW_COMMAND,
  orbitCameraDistance,
  parseFixtureV1,
  type OrbitCameraViewId,
  parseVortexFullId,
  puzzle3dLodCanvasProps,
  puzzle3dVortexFullId,
  puzzle3dPlayObjectKindDragData,
  sliderValueFromLod,
  updatePuzzle3dCameraInFixture,
  type AttractionProps,
  type CameraState,
  type AttractionKind,
  type CableKind,
  type FixtureObjectV1,
  type FixtureV1,
  type KindCatalogBundle,
  type KindCompatEntry,
  type KitConnectorCadRow,
  puzzle3dVortexKindLabelFromHandleKind,
  type AttractionVortexContext,
  type ObjectKind,
  type ObjectKindVortexTemplate,
  type BrushPlacePayload,
  type RelocatePayload,
  type SelectionMethod,
  type SelectionMode,
  PUZZLE_3D_ENGAGEMENT_TOOL_FILL_ID,
  type SelectionSnapshot,
  type MarqueeSelectableKinds,
  type VortexKind,
  type VortexProps,
  puzzle3dBrushEngagementSourceRef,
  publishPuzzle3dBrushKindWeights,
  PUZZLE_3D_ENGAGEMENT_BRUSH_NEXT_ID,
  PUZZLE_3D_ENGAGEMENT_ZOOM_ID,
  getPuzzle3dZoomToSelectionEpoch,
  getPuzzle3dZoomToSelectionTarget,
  requestPuzzle3dZoomToSelection,
  resolveObjectKindMeshUrl,
  isLoadableMeshUrl,
  brushCompatibleCandidates,
  kindsCompatible,
  publishPuzzle3dBrushCandidateAccept,
  type BrushCompatibleCandidate,
  type HoverTarget,
  type Puzzle3dHoverPayload,
  type Puzzle3dKindHover,
  type Puzzle3dKindHoverDomain,
  puzzle3dHoverTargetsEqual,
  puzzle3dKindHoversEqual,
} from "../react/index.tsx";
import nakaginPuzzle3dFixtureJson from "../fixture/nakagin-capsule-tower.3d.json";
import concreteForestPuzzle3dFixtureJson from "../fixture/concrete-forest.3d.json";

export const PUZZLE_3D_PLAY_FIXTURE_NAKAGIN_ID = "nakagin";
export const PUZZLE_3D_PLAY_FIXTURE_CONCRETE_FOREST_ID = "concrete-forest";

export const PUZZLE_3D_PLAY_FIXTURE_OPTIONS = [
  { id: PUZZLE_3D_PLAY_FIXTURE_CONCRETE_FOREST_ID, label: "Concrete Forest" },
  { id: PUZZLE_3D_PLAY_FIXTURE_NAKAGIN_ID, label: "Nakagin capsule tower" },
] as const;

const PUZZLE_3D_PLAY_FIXTURE_JSON_BY_ID: Record<string, unknown> = {
  [PUZZLE_3D_PLAY_FIXTURE_NAKAGIN_ID]: nakaginPuzzle3dFixtureJson,
  [PUZZLE_3D_PLAY_FIXTURE_CONCRETE_FOREST_ID]: concreteForestPuzzle3dFixtureJson,
};

/** @emoji 🧪 Resolves imported puzzle 3d fixture JSON by catalog id. */
export function puzzle3dPlayFixtureJson(fixtureId: string = PUZZLE_3D_PLAY_FIXTURE_CONCRETE_FOREST_ID): unknown {
  return PUZZLE_3D_PLAY_FIXTURE_JSON_BY_ID[fixtureId] ?? concreteForestPuzzle3dFixtureJson;
}

//#region 🏗️NakaginCatalog
function cadVec3FromKitPoint(row: { readonly x: number; readonly y: number; readonly z: number }): [number, number, number] {
  return [row.x, row.y, row.z];
}

/** @emoji 🧲 Builds object-kind vortex templates from kit connectors (nakagin play fixture tooling). */
export function puzzle3dPlayObjectKindVorticesFromKitConnectors(
  connectors: readonly KitConnectorCadRow[],
  labelHandleKind: (handleKind: string) => string,
  defaultRadius = 0.36,
  objectKindId?: string,
): ObjectKindVortexTemplate[] {
  const seenPositions = new Set<string>();
  const out: ObjectKindVortexTemplate[] = [];
  for (const connector of connectors) {
    const handleKind = connector.port?.handleKind?.trim() ?? "";
    const point = connector.point;
    if (handleKind === "" || !point) {
      continue;
    }
    const position = cadVec3FromKitPoint(point);
    const posKey = position.map((n) => n.toFixed(6)).join(",");
    if (seenPositions.has(posKey)) {
      continue;
    }
    seenPositions.add(posKey);
    const portName = labelHandleKind(handleKind);
    const vortexKind = puzzle3dPlayDoorCapsuleVortexKindFromKindPortAndPoint(objectKindId, portName, point);
    out.push({
      vortexKind,
      position,
      ...(connector.direction ? { direction: cadVec3FromKitPoint(connector.direction) } : {}),
      radius: defaultRadius,
    });
  }
  return out;
}

const NAKAGIN_CAPSULE_KIND_SPECIFICITY_PREFIXES = ["Capsule With Balcony ", "Trapezoid Capsule "] as const;

/** @emoji 🏷️ Picks the most specific nakagin capsule kind when a plainer alias exists in `availableKindNames`. */
export function puzzle3dPlayPreferSpecificCapsuleKindName(kindName: string, availableKindNames: ReadonlySet<string>): string {
  const name = kindName.trim();
  if (name === "") {
    return name;
  }
  if (NAKAGIN_CAPSULE_KIND_SPECIFICITY_PREFIXES.some((prefix) => name.startsWith(prefix))) {
    return name;
  }
  const plain = /^Capsule (.+)$/.exec(name);
  if (!plain) {
    return name;
  }
  const tail = plain[1]!;
  for (const prefix of NAKAGIN_CAPSULE_KIND_SPECIFICITY_PREFIXES) {
    const candidate = `${prefix}${tail}`;
    if (availableKindNames.has(candidate)) {
      return candidate;
    }
  }
  return name;
}

const DOOR_CAPSULE_RIGHT_VORTEX_KIND = "door capsule right";
const DOOR_CAPSULE_LEFT_VORTEX_KIND = "door capsule left";

function puzzle3dPlayDoorCapsuleSideFromKindName(kindName: string | undefined): "left" | "right" | null {
  const name = kindName?.trim() ?? "";
  if (name === "") {
    return null;
  }
  const tail = /(?:Capsule(?: With Balcony)?|Trapezoid Capsule)\s+([A-Za-z])\s*$/.exec(name)?.[1] ?? /Capsule\s+([A-Za-z])\s*$/.exec(name)?.[1];
  if (!tail) {
    return null;
  }
  if (tail === "J" || tail === "S") {
    return "right";
  }
  if (tail === "L" || tail === "Z") {
    return "left";
  }
  return null;
}

/** @emoji 🚪 Resolves left vs right door vortex kind from port name and connector CAD. */
export function puzzle3dPlayDoorCapsuleVortexKindFromPortNameAndPoint(
  portName: string,
  point: { readonly x: number; readonly y: number; readonly z: number },
): string {
  if (!portName.includes("door capsule")) {
    return portName;
  }
  return point.x < 0 ? DOOR_CAPSULE_LEFT_VORTEX_KIND : DOOR_CAPSULE_RIGHT_VORTEX_KIND;
}

function puzzle3dPlayDoorCapsuleVortexKindFromKindPortAndPoint(
  objectKindId: string | undefined,
  portName: string,
  point: { readonly x: number; readonly y: number; readonly z: number },
): string {
  if (!portName.includes("door capsule")) {
    return portName;
  }
  const side = puzzle3dPlayDoorCapsuleSideFromKindName(objectKindId);
  if (side === "left") {
    return DOOR_CAPSULE_LEFT_VORTEX_KIND;
  }
  if (side === "right") {
    return DOOR_CAPSULE_RIGHT_VORTEX_KIND;
  }
  return puzzle3dPlayDoorCapsuleVortexKindFromPortNameAndPoint(portName, point);
}

/** @emoji 🖌️ Nakagin brush filter: tambour doors accept facade door-capsule ports only (not platform or floor-door P/Slash). */
function puzzle3dPlayBrushCandidateAccept(
  target: AttractionVortexContext,
  candidate: BrushCompatibleCandidate,
  template: ObjectKindVortexTemplate,
): boolean {
  const targetVk = target.vortexKind ?? "";
  if (targetVk.includes("tambour circular") || targetVk.includes("tambour rectangular")) {
    if (candidate.objectKindId.includes("Capital")) {
      return false;
    }
    const hostKind = target.objectKind ?? "";
    if ((hostKind === "Tambour" || hostKind === "Cylindric Tambour") && (candidate.objectKindId.includes("Last Storey") || candidate.objectKindId.includes("Single Storey"))) {
      return false;
    }
  }
  if (!targetVk.includes("door tambour")) {
    return true;
  }
  const sourceVk = template.vortexKind ?? "";
  if (!sourceVk.includes("door capsule")) {
    return false;
  }
  const [x, y] = template.position;
  return Math.abs(x) >= 0.9 && Math.abs(y) < 1.6;
}

function relabelDoorCapsuleVortexTemplate(objectKindId: string, vortex: ObjectKindVortexTemplate): ObjectKindVortexTemplate {
  if (!vortex.vortexKind?.includes("door capsule")) {
    return vortex;
  }
  const [x, y, z] = vortex.position;
  const nextKind = puzzle3dPlayDoorCapsuleVortexKindFromKindPortAndPoint(objectKindId, vortex.vortexKind, { x, y, z });
  return nextKind === vortex.vortexKind ? vortex : { ...vortex, vortexKind: nextKind };
}

/** @emoji 🚪 Relabels palette door vortex kinds to match nakagin capsule kind ids in the kind catalog. */
function enrichKindCatalogBundleDoorCapsules(bundle: KindCatalogBundle | undefined): KindCatalogBundle | undefined {
  if (!bundle?.objects?.length) {
    return bundle;
  }
  let touched = false;
  const objects = bundle.objects.map((kind) => {
    if (!kind.vortices?.length) {
      return kind;
    }
    const vortices = kind.vortices.map((vortex) => relabelDoorCapsuleVortexTemplate(kind.id, vortex));
    if (vortices.every((v, i) => v === kind.vortices![i])) {
      return kind;
    }
    touched = true;
    return { ...kind, vortices };
  });
  return touched ? { ...bundle, objects } : bundle;
}
//#endregion 🏗️NakaginCatalog

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
      e.specificity === "general" || e.specificity === "object" || e.specificity === "vortex" || e.specificity === "cable" || e.specificity === "attraction" ? e.specificity : undefined;
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
  return enrichKindCatalogBundleDoorCapsules(kc as KindCatalogBundle);
}

/** @emoji 🖌️ Publishes nakagin brush candidate filter for {@link PlayCanvas} / playground hosts (weights come from {@link Puzzle3dPlayShellController}). */
export function installPuzzle3dPlayBrushHost(_meta: Record<string, unknown> | undefined): void {
  publishPuzzle3dBrushCandidateAccept(puzzle3dPlayBrushCandidateAccept);
}

/** @emoji 🪣 Cached fill session for interactive slider prefix application. */
export interface Puzzle3dFillSessionState {
  baseFixture: FixtureV1 | null;
  sequence: readonly BrushPlacePayload[];
  appendedObjects: readonly FixtureObjectV1[];
  appendedAttractions: readonly AttractionProps[];
  seed: number;
}

/** @emoji 🪣 Live fill build progress while {@link preparePuzzle3dFillSession} runs chunked. */
export interface Puzzle3dFillBuildProgress {
  readonly count: number;
  readonly maxCount: number;
  readonly done: boolean;
}

export const puzzle3dFillSessionRef: { current: Puzzle3dFillSessionState } = {
  current: { baseFixture: null, sequence: [], appendedObjects: [], appendedAttractions: [], seed: 0 },
};

/** @emoji 🪣 Latest fill build progress (updated each chunked step). */
export const puzzle3dFillBuildProgressRef: { current: Puzzle3dFillBuildProgress } = {
  current: { count: 0, maxCount: PUZZLE_3D_FILL_COUNT_MAX, done: false },
};

const PUZZLE_3D_FILL_BUILD_CHUNK_BUDGET = 8;
let puzzle3dFillBuildTimer: ReturnType<typeof setTimeout> | null = null;
let puzzle3dFillBuildStepper: ReturnType<typeof createBrushFillSequenceStepper> | null = null;

function cancelPuzzle3dFillBuild(): void {
  if (puzzle3dFillBuildTimer !== null) {
    clearTimeout(puzzle3dFillBuildTimer);
    puzzle3dFillBuildTimer = null;
  }
  puzzle3dFillBuildStepper = null;
}

/** @emoji 🪣 Latest fill slider count from the playground host (re-applied after mesh preload). */
export const puzzle3dFillPendingCountRef = { current: 0 };

let puzzle3dFillSessionReadyEpoch = 0;
const puzzle3dFillSessionReadyListeners = new Set<() => void>();

/** @emoji 🪣 Subscribes to fill session rebuilds (after mesh preload + {@link preparePuzzle3dFillSession}). */
export function subscribePuzzle3dFillSessionReady(listener: () => void): () => void {
  puzzle3dFillSessionReadyListeners.add(listener);
  return () => {
    puzzle3dFillSessionReadyListeners.delete(listener);
  };
}

/** @emoji 🪣 Epoch bumped when a fill session is prepared. */
export function getPuzzle3dFillSessionReadyEpoch(): number {
  return puzzle3dFillSessionReadyEpoch;
}

function notifyPuzzle3dFillSessionReady(): void {
  puzzle3dFillSessionReadyEpoch += 1;
  for (const listener of puzzle3dFillSessionReadyListeners) {
    listener();
  }
}

/** @emoji 🪣 Builds a deterministic fill sequence from the current fixture snapshot. */
export function preparePuzzle3dFillSession(
  baseFixture: FixtureV1,
  kindCatalogs: KindCatalogBundle | undefined,
  kindCompatibility: readonly KindCompatEntry[] | undefined,
  overlapBudget: number,
): void {
  cancelPuzzle3dFillBuild();
  const seed = (Date.now() ^ Math.floor(Math.random() * 0x7fffffff)) >>> 0;
  const clonedBase = structuredClone(baseFixture);
  puzzle3dFillSessionRef.current = {
    baseFixture: clonedBase,
    sequence: [],
    appendedObjects: [],
    appendedAttractions: [],
    seed,
  };
  puzzle3dFillBuildProgressRef.current = { count: 0, maxCount: PUZZLE_3D_FILL_COUNT_MAX, done: false };
  puzzle3dFillBuildStepper = createBrushFillSequenceStepper({
    baseFixture: clonedBase,
    maxCount: PUZZLE_3D_FILL_COUNT_MAX,
    seed,
    kindCatalogs,
    kindCompatibility,
    overlapBudget,
    meshRootForUrl: puzzle3dBrushMeshRootForFill,
    weights: puzzle3dBrushKindWeightsRef.current,
  });
  notifyPuzzle3dFillSessionReady();
  const tick = (): void => {
    const stepper = puzzle3dFillBuildStepper;
    if (!stepper) {
      return;
    }
    const started = performance.now();
    const result = stepper.step(PUZZLE_3D_FILL_BUILD_CHUNK_BUDGET);
    puzzle3dFillSessionRef.current = {
      ...puzzle3dFillSessionRef.current,
      sequence: result.sequence,
      appendedObjects: result.appendedObjects,
      appendedAttractions: result.appendedAttractions,
    };
    puzzle3dFillBuildProgressRef.current = {
      count: result.sequence.length,
      maxCount: PUZZLE_3D_FILL_COUNT_MAX,
      done: result.done,
    };
    console.log(
      `[DEBUG] puzzle3d fill build chunk count=${result.sequence.length}/${PUZZLE_3D_FILL_COUNT_MAX} done=${result.done} ms=${(performance.now() - started).toFixed(1)}`,
    );
    notifyPuzzle3dFillSessionReady();
    if (!result.done) {
      puzzle3dFillBuildTimer = setTimeout(tick, 0);
      return;
    }
    puzzle3dFillBuildStepper = null;
    puzzle3dFillBuildTimer = null;
  };
  puzzle3dFillBuildTimer = setTimeout(tick, 0);
}

/** @emoji 🪣 Applies a fill prefix count onto the cached base fixture. */
export function applyPuzzle3dFillCount(count: number, _kindCatalogs?: KindCatalogBundle | undefined): FixtureV1 | null {
  const session = puzzle3dFillSessionRef.current;
  if (!session.baseFixture) {
    return null;
  }
  const available = session.appendedObjects.length;
  const n = Math.max(0, Math.min(PUZZLE_3D_FILL_COUNT_MAX, Math.round(count), available));
  if (n === 0) {
    return session.baseFixture;
  }
  return {
    ...session.baseFixture,
    objects: [...session.baseFixture.objects, ...session.appendedObjects.slice(0, n)],
    attractions: [...session.baseFixture.attractions, ...session.appendedAttractions.slice(0, n)],
  };
}

/** @emoji 🪣 Clears the cached fill session and returns the base fixture when present. */
export function clearPuzzle3dFillSession(): FixtureV1 | null {
  cancelPuzzle3dFillBuild();
  const base = puzzle3dFillSessionRef.current.baseFixture;
  puzzle3dFillSessionRef.current = { baseFixture: null, sequence: [], appendedObjects: [], appendedAttractions: [], seed: 0 };
  puzzle3dFillBuildProgressRef.current = { count: 0, maxCount: PUZZLE_3D_FILL_COUNT_MAX, done: false };
  return base;
}

/** @emoji 📋 Kind catalog rows as unique select options (last row wins per `id`; sorted by label). */
export function puzzle3dPlayKindCatalogSelectItems<T extends { readonly id: string; readonly label?: string; readonly name?: string }>(
  entries: readonly T[] | undefined,
): readonly { readonly value: string; readonly label: string }[] {
  if (!entries?.length) {
    return [];
  }
  const byId = new Map<string, { value: string; label: string }>();
  for (const entry of entries) {
    byId.set(entry.id, { value: entry.id, label: entry.label?.trim() || entry.name?.trim() || entry.id });
  }
  return [...byId.values()].sort((a, b) => a.label.localeCompare(b.label));
}
//#endregion 🧾Meta

//#region 🖥️Surface
export const PUZZLE_3D_PLAY_LS_THEME = "puzzle.3d-play.surface.theme";
export const PUZZLE_3D_PLAY_LS_DEVICE = "puzzle.3d-play.surface.device";
export const PUZZLE_3D_PLAY_LS_EXPERTISE = "puzzle.3d-play.surface.expertise";

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
export const PUZZLE_3D_PLAY_APP_ID = "puzzle-3d-play";
export const PUZZLE_3D_PLAY_WINDOW_ID = "puzzle-3d-main";
export const PUZZLE_3D_PLAY_WINDOW_LABEL = "Puzzle 3D";
export const PUZZLE_3D_PLAY_BODY_KEY = "puzzle.3d.play.window";
export const PUZZLE_3D_PLAY_CONTROLLER_ID = "puzzle-3d-play";

const PUZZLE_3D_VIEW_TEMPLATES: readonly WindowTemplate[] = createOrbitCameraViewTemplates({
  controllerId: PUZZLE_3D_PLAY_CONTROLLER_ID,
}) as readonly WindowTemplate[];
export const PUZZLE_3D_PLAY_VIEWPORT_SURFACE_ID = "puzzle.3d.play.viewport/v1";
export const PUZZLE_3D_PLAY_INSPECTOR_TAB_ID = "puzzle-3d-play-inspector";
export const PUZZLE_3D_PLAY_SETTINGS_TAB_ID = "puzzle-3d-play-settings";
export const PUZZLE_3D_PLAY_HIERARCHY_TAB_ID = "puzzle-3d-play-hierarchy";
export const PUZZLE_3D_PLAY_KINDS_TAB_ID = "puzzle-3d-play-kinds";
export const PUZZLE_3D_PLAY_HIERARCHY_BODY_KEY = "puzzle.3d.play.hierarchy";
export const PUZZLE_3D_PLAY_KINDS_BODY_KEY = "puzzle.3d.play.kinds";
export const PUZZLE_3D_PLAY_INSPECTOR_BODY_KEY = "puzzle.3d.play.inspector";
export const PUZZLE_3D_PLAY_SETTINGS_BODY_KEY = "puzzle.3d.play.settings";

/** @emoji 🎯 Side-panel body keys that refresh from controller snapshot on selection (not shell generation). */
export const PUZZLE_3D_PLAY_SNAPSHOT_PANEL_BODY_KEYS: ReadonlySet<string> = new Set([
  PUZZLE_3D_PLAY_HIERARCHY_BODY_KEY,
  PUZZLE_3D_PLAY_KINDS_BODY_KEY,
  PUZZLE_3D_PLAY_INSPECTOR_BODY_KEY,
]);
export const PUZZLE_3D_PLAY_ICON_HIERARCHY = "puzzle.3d-play.icon.hierarchy";
export const PUZZLE_3D_PLAY_ICON_KINDS = "puzzle.3d-play.icon.kinds";
export const PUZZLE_3D_PLAY_ICON_INSPECTOR = "puzzle.3d-play.icon.inspector";
export const PUZZLE_3D_PLAY_ICON_SETTINGS = "puzzle.3d-play.icon.settings";

/** @emoji 🖌️ Window engagement possible id for the brush tool. */
export const PUZZLE_3D_ENGAGEMENT_TOOL_BRUSH_ID = "puzzle3d.tool.brush";

/** @emoji 🎯 Window engagement possible id for the select tool. */
export const PUZZLE_3D_ENGAGEMENT_TOOL_SELECT_ID = "puzzle3d.tool.select";

export {
  PUZZLE_3D_ENGAGEMENT_ZOOM_ID,
  PUZZLE_3D_FILL_COUNT_MAX,
  applyBrushFillPlacementsToFixture,
  brushMeshUrlsForFillSession,
  buildBrushFillSequence,
  puzzle3dBrushMeshRootForFill,
} from "@puzzle/3d/react";
export { PUZZLE_3D_ENGAGEMENT_TOOL_FILL_ID } from "@puzzle/3d/react";
//#endregion 🎬Play

function puzzle3dPlayCmd(command: string, args?: Record<string, unknown>): CommandDescriptor {
	return { controllerId: PUZZLE_3D_PLAY_CONTROLLER_ID, command, args: args as never };
}

/** @emoji ⌨️ Lowercase PascalCase engagement token for tool command matching (mirrors ui {@link normalizeEngagementCommandText}). */
function puzzle3dPlayEngagementCommandToken(text: string): string {
	const words = text
		.replace(/[^a-zA-Z0-9]+/g, " ")
		.trim()
		.split(/\s+/)
		.filter(Boolean)
		.flatMap((word) => word.split(/(?=[A-Z])/))
		.filter(Boolean);
	return words.map((word) => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase()).join("").toLowerCase();
}

function puzzle3dPlaySelectObjectCommand(objectId: string): CommandDescriptor {
	return puzzle3dPlayCmd("setSelection", { selection: { objectIds: [objectId], vortexIds: [], attractionIds: [] } });
}

function puzzle3dPlaySelectVortexCommand(vortexFullId: string): CommandDescriptor {
	return puzzle3dPlayCmd("setSelection", { selection: { objectIds: [], vortexIds: [vortexFullId], attractionIds: [] } });
}

function puzzle3dPlaySelectAttractionCommand(attractionId: string): CommandDescriptor {
	return puzzle3dPlayCmd("setSelection", { selection: { objectIds: [], vortexIds: [], attractionIds: [attractionId] } });
}

export { parseKindCatalogs, parseKindCompatibility };

/** @emoji 🔗 React host bridge: routes engagement bus commands to live {@link EngagementSpec} callbacks. */
export interface Puzzle3dPlayHostBridge {
  runHostCommand(command: string, args?: unknown): void;
}

//#region 🔖Puzzle3dPlaySelection
/** @emoji 🎯 Play harness selection: objects, vortex full ids, and attractions. */
export type Puzzle3dPlaySelection = SelectionSnapshot;

export type Puzzle3dGumballGroupKey = keyof Pick<GumballConfig, "moveAxes" | "movePlanes" | "rotate" | "scaleAxes" | "scaleUniform">;

export const PUZZLE_3D_GUMBALL_GROUPS: readonly { readonly key: Puzzle3dGumballGroupKey; readonly label: string; readonly iconId: string }[] = [
  { key: "moveAxes", label: "Move Axes", iconId: "move" },
  { key: "movePlanes", label: "Move Planes", iconId: "move-3d" },
  { key: "rotate", label: "Rotate", iconId: "rotate-cw" },
  { key: "scaleAxes", label: "Scale Axes", iconId: "maximize-2" },
  { key: "scaleUniform", label: "Scale Uniform", iconId: "box" },
];

export const PUZZLE_3D_PLAY_EMPTY_SELECTION: Puzzle3dPlaySelection = {
  objectIds: [],
  vortexIds: [],
  attractionIds: [],
};

/** @emoji 📸 Stable idle snapshot for {@link useSyncExternalStore} when no controller is mounted. */
export const PUZZLE_3D_PLAY_IDLE_SNAPSHOT: Puzzle3dPlaySnapshot = {
  fixture: null,
  fixtureRevision: 0,
  lodProps: puzzle3dLodCanvasProps({ automaticLod: true, depthVariableLod: false, manualLod: DEFAULT_MANUAL_LOD }),
  lodTag: DEFAULT_MANUAL_LOD,
  lodSlider: sliderValueFromLod(DEFAULT_MANUAL_LOD),
  automaticLod: true,
  depthVariableLod: false,
  gumballConfig: DEFAULT_GUMBALL_CONFIG,
  selection: PUZZLE_3D_PLAY_EMPTY_SELECTION,
  selectedId: null,
  selectedLabel: null,
  selectionMode: "default",
  selectionMethod: "rectangle",
  selectableKinds: { object: true, vortex: true, attraction: true },
  proximityRadius: 24,
  chunkSize: 256,
  gridFactor: 10,
  showLodGrid: true,
  gridSnapEnabled: true,
  proximityCount: 0,
  connectCount: 0,
  indirectCount: 0,
  compatibleObjectsCount: 0,
  targetRingCount: 0,
  activeTool: "select",
  brushPlacementOverlapBudget: DEFAULT_BRUSH_PLACEMENT_OVERLAP_BUDGET,
  cameraSeedEpoch: 0,
  hoverFocus: { hoverTarget: null, kindHover: null },
};

/** @emoji 🖌️ Puzzle 3D play active viewport tool. */
export type Puzzle3dActiveTool = "select" | "brush" | "fill";

export { puzzle3dVortexFullId };

const puzzle3dPlayFixtureCaptionSeparator = " · ";

/** @emoji 🏷️ Tree/inspector primary caption (fixture ` · ` captions split to the name only). */
export function puzzle3dPlayFixtureRowLabel(label: string | undefined, fallbackId: string): string {
  return puzzle3dPlayFixtureTreeRowFields(label, fallbackId).label;
}

/** @emoji 🏷️ Hierarchy row fields: primary name with optional muted id on the same line (no separator glyph). */
export function puzzle3dPlayFixtureTreeRowFields(
  label: string | undefined,
  id: string,
): Pick<UiTreeItemNode, "label" | "description"> {
  const trimmed = label?.trim();
  if (trimmed?.includes(puzzle3dPlayFixtureCaptionSeparator)) {
    const separatorIndex = trimmed.indexOf(puzzle3dPlayFixtureCaptionSeparator);
    const name = trimmed.slice(0, separatorIndex).trim();
    const captionId = trimmed.slice(separatorIndex + puzzle3dPlayFixtureCaptionSeparator.length).trim();
    if (name.length > 0 && captionId.length > 0) {
      return { label: name, description: captionId };
    }
  }
  if (!trimmed || trimmed.length === 0) {
    return { label: id };
  }
  if (trimmed === id) {
    return { label: id };
  }
  return { label: trimmed, description: id };
}

/** @emoji 🎯 Resolved selection label for play chrome (objects, vortices, attractions). */
export function puzzle3dPlaySelectionLabel(fixture: FixtureV1 | null, selection: Puzzle3dPlaySelection): string | null {
  if (!fixture) return null;
  if (selection.attractionIds[0]) {
    return selection.attractionIds[0];
  }
  if (selection.vortexIds[0]) {
    const { objectId, vortexId } = parseVortexFullId(selection.vortexIds[0]);
    const object = fixture.objects.find((row) => row.id === objectId);
    const vortex = object?.vortices.find((row) => row.id === vortexId || puzzle3dVortexFullId(objectId, row.id) === selection.vortexIds[0]);
    return puzzle3dPlayFixtureRowLabel(vortex?.label, selection.vortexIds[0]);
  }
  if (selection.objectIds[0]) {
    const object = fixture.objects.find((row) => row.id === selection.objectIds[0]);
    return puzzle3dPlayFixtureRowLabel(object?.label, selection.objectIds[0]);
  }
  return null;
}

/** @emoji 🗑️ Removes an object and any attractions touching it or its vortices. */
export function deletePuzzle3dObjectFromFixture(fixture: FixtureV1, objectId: string): FixtureV1 {
  const removedVortexFullIds = new Set<string>();
  for (const object of fixture.objects) {
    if (object.id !== objectId) {
      continue;
    }
    for (const vortex of object.vortices) {
      removedVortexFullIds.add(puzzle3dVortexFullId(objectId, vortex.id));
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
export function deletePuzzle3dVortexFromFixture(fixture: FixtureV1, vortexFullId: string): FixtureV1 {
  const { objectId } = parseVortexFullId(vortexFullId);
  return {
    ...fixture,
    objects: fixture.objects.map((object) =>
      object.id !== objectId
        ? object
        : {
            ...object,
            vortices: object.vortices.filter((vortex) => puzzle3dVortexFullId(objectId, vortex.id) !== vortexFullId),
          },
    ),
    attractions: fixture.attractions.filter((attraction) => attraction.attracting !== vortexFullId && attraction.attracted !== vortexFullId),
  };
}

/** @emoji 🗑️ Drops a single attraction row. */
export function deletePuzzle3dAttractionFromFixture(fixture: FixtureV1, attractionId: string): FixtureV1 {
  return {
    ...fixture,
    attractions: fixture.attractions.filter((attraction) => attraction.id !== attractionId),
  };
}

function patchPuzzle3dObject(objects: readonly FixtureObjectV1[], objectId: string, patch: (object: FixtureObjectV1) => FixtureObjectV1): FixtureObjectV1[] {
  return objects.map((object) => (object.id === objectId ? patch(object) : object));
}

/** @emoji ✏️ Updates fields on one fixture object. */
export function updatePuzzle3dObjectInFixture(fixture: FixtureV1, objectId: string, patch: Partial<Omit<FixtureObjectV1, "id" | "vortices">>): FixtureV1 {
  return {
    ...fixture,
    objects: patchPuzzle3dObject(fixture.objects, objectId, (object) => ({ ...object, ...patch })),
  };
}

/** @emoji ✏️ Updates one vortex on an object. */
export function updatePuzzle3dVortexInFixture(fixture: FixtureV1, vortexFullId: string, patch: Partial<VortexProps>): FixtureV1 {
  const { objectId, vortexId } = parseVortexFullId(vortexFullId);
  return {
    ...fixture,
    objects: patchPuzzle3dObject(fixture.objects, objectId, (object) => ({
      ...object,
      vortices: object.vortices.map((vortex) => {
        const fullId = puzzle3dVortexFullId(objectId, vortex.id);
        if (fullId !== vortexFullId && vortex.id !== vortexId) {
          return vortex;
        }
        return { ...vortex, ...patch, id: vortex.id };
      }),
    })),
  };
}

/** @emoji ✏️ Updates one attraction row. */
export function updatePuzzle3dAttractionInFixture(fixture: FixtureV1, attractionId: string, patch: Partial<AttractionProps>): FixtureV1 {
  return {
    ...fixture,
    attractions: fixture.attractions.map((attraction) => (attraction.id === attractionId ? { ...attraction, ...patch } : attraction)),
  };
}

export { cameraStateNearEqual, updatePuzzle3dCameraInFixture } from "../react/index.tsx";

/** @emoji 🎯 Maps {@link SelectionSnapshot} to play selection. */
export function selectionSnapshotToPlaySelection(snap: SelectionSnapshot): Puzzle3dPlaySelection {
  return {
    objectIds: snap.objectIds,
    vortexIds: snap.vortexIds,
    attractionIds: snap.attractionIds,
  };
}

/** @emoji 🎯 True when two selection snapshots match (skips redundant shell updates). */
/** @emoji ⌨️ Select-all snapshot for the fixture honoring playground object/vortex/attraction kind toggles. */
export function puzzle3dPlayAllSelectionFromFixture(
  fixture: FixtureV1,
  kinds: Readonly<Record<Puzzle3dPlayPickKind, boolean>>,
): Puzzle3dPlaySelection {
  return {
    objectIds: kinds.object ? fixture.objects.map((object) => object.id) : [],
    vortexIds: kinds.vortex
      ? fixture.objects.flatMap((object) => object.vortices.map((vortex) => puzzle3dVortexFullId(object.id, vortex.id)))
      : [],
    attractionIds: kinds.attraction ? fixture.attractions.map((attraction) => attraction.id) : [],
  };
}

export function puzzle3dPlaySelectionEqual(a: Puzzle3dPlaySelection, b: Puzzle3dPlaySelection): boolean {
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

//#region 🔖Puzzle3dPlayHover
/** @emoji 🖱️ Imperative hover sink wired by the play renderer (stable ref for cached hierarchy trees). */
export const puzzle3dPlayHoverBridgeRef: { current: ((payload: Puzzle3dHoverPayload) => void) | null } = { current: null };

export type Puzzle3dPlayHierarchyHoverBuildOptions = {
  readonly onHover?: (payload: Puzzle3dHoverPayload) => void;
};

function puzzle3dPlayHoverSink(onHover?: (payload: Puzzle3dHoverPayload) => void): ((payload: Puzzle3dHoverPayload) => void) | undefined {
  if (onHover) {
    return onHover;
  }
  const bridge = puzzle3dPlayHoverBridgeRef.current;
  return bridge ?? undefined;
}

function puzzle3dPlayHierarchyInstanceHoverHandlers(
  onHover: ((payload: Puzzle3dHoverPayload) => void) | undefined,
  target: HoverTarget,
): Pick<UiTreeItemNode, "onPointerEnter" | "onPointerLeave"> {
  const sink = puzzle3dPlayHoverSink(onHover);
  if (!sink) {
    return {};
  }
  return {
    onPointerEnter: () => sink({ hoverTarget: target, kindHover: null }),
    onPointerLeave: () => sink({ hoverTarget: null, kindHover: null }),
  };
}

function puzzle3dPlayKindRowHoverHandlers(
  onHover: ((payload: Puzzle3dHoverPayload) => void) | undefined,
  kind: Puzzle3dKindHover,
): Pick<UiTreeItemNode, "onPointerEnter" | "onPointerLeave"> {
  const sink = puzzle3dPlayHoverSink(onHover);
  if (!sink) {
    return {};
  }
  return {
    onPointerEnter: () => sink({ hoverTarget: null, kindHover: kind }),
    onPointerLeave: () => sink({ hoverTarget: null, kindHover: null }),
  };
}

/** @emoji 🌳 Maps transitive kind hover to hierarchy tree row ids. */
export function puzzle3dPlayHierarchyTreeHighlightedIds(fixture: FixtureV1, kindHover: Puzzle3dKindHover | null): readonly string[] {
  if (!kindHover?.kindId) {
    return [];
  }
  const ids: string[] = [];
  if (kindHover.domain === "object") {
    for (const object of fixture.objects) {
      if (object.objectKind === kindHover.kindId) {
        ids.push(`puzzle-3d-play-hierarchy.object.${object.id}`);
      }
    }
    return ids;
  }
  if (kindHover.domain === "vortex") {
    for (const object of fixture.objects) {
      for (const vortex of object.vortices) {
        if (vortex.vortexKind === kindHover.kindId) {
          ids.push(`puzzle-3d-play-hierarchy.vortex.${puzzle3dVortexFullId(object.id, vortex.id)}`);
        }
      }
    }
    return ids;
  }
  for (const attraction of fixture.attractions) {
    if (attraction.attractionKind === kindHover.kindId) {
      ids.push(`puzzle-3d-play-hierarchy.attraction.${attraction.id}`);
    }
  }
  return ids;
}

/** @emoji 🖱️ Resolves catalog kind hover from a play fixture instance target. */
export function puzzle3dKindHoverFromPlayTarget(fixture: FixtureV1 | null, target: HoverTarget): Puzzle3dKindHover | null {
  if (!fixture) {
    return null;
  }
  switch (target.kind) {
    case "object": {
      const object = fixture.objects.find((row) => row.id === target.id);
      const kindId = object?.objectKind?.trim();
      return kindId ? { domain: "object", kindId } : null;
    }
    case "vortex": {
      const { objectId, vortexId } = parseVortexFullId(target.fullId);
      const object = fixture.objects.find((row) => row.id === objectId);
      const vortex = object?.vortices.find((row) => row.id === vortexId);
      const kindId = vortex?.vortexKind?.trim();
      return kindId ? { domain: "vortex", kindId } : null;
    }
    case "attraction": {
      const attraction = fixture.attractions.find((row) => row.id === target.id);
      const kindId = attraction?.attractionKind?.trim();
      return kindId ? { domain: "attraction", kindId } : null;
    }
    default:
      return null;
  }
}

function puzzle3dPlayKindsSectionDomain(sectionId: string): Puzzle3dKindHoverDomain | null {
  if (sectionId === "puzzle-3d-play-kinds.objects") {
    return "object";
  }
  if (sectionId === "puzzle-3d-play-kinds.vortices") {
    return "vortex";
  }
  if (sectionId === "puzzle-3d-play-kinds.attractions") {
    return "attraction";
  }
  return null;
}
//#endregion 🔖Puzzle3dPlayHover

//#region 🔖Puzzle3dPlayHierarchy
/** @emoji 🎯 Maps play selection to declarative hierarchy row ids for {@link UiTreeNode.selectedIds}. */
export function puzzle3dPlayHierarchySelectedIds(selection: Puzzle3dPlaySelection): readonly string[] {
  const ids: string[] = [];
  for (const objectId of selection.objectIds) {
    ids.push(`puzzle-3d-play-hierarchy.object.${objectId}`);
  }
  for (const fullId of selection.vortexIds) {
    ids.push(`puzzle-3d-play-hierarchy.vortex.${fullId}`);
  }
  for (const attractionId of selection.attractionIds) {
    ids.push(`puzzle-3d-play-hierarchy.attraction.${attractionId}`);
  }
  return ids;
}

/** @emoji 🌳 Structural hierarchy sections (no per-row selected flags; selection via {@link puzzle3dPlayHierarchySelectedIds}). */
export function buildPuzzle3dPlayHierarchySections(
  fixture: FixtureV1,
  options?: Puzzle3dPlayHierarchyHoverBuildOptions,
): readonly UiTreeSectionNode[] {
  const onHover = options?.onHover;
  const objectItems: UiTreeItemNode[] = fixture.objects.map((object) => {
    const vortexItems: UiTreeItemNode[] = object.vortices.map((vortex) => {
      const fullId = puzzle3dVortexFullId(object.id, vortex.id);
      return {
        id: `puzzle-3d-play-hierarchy.vortex.${fullId}`,
        ...puzzle3dPlayFixtureTreeRowFields(vortex.label, fullId),
        command: puzzle3dPlaySelectVortexCommand(fullId),
        ...puzzle3dPlayHierarchyInstanceHoverHandlers(onHover, { kind: "vortex", fullId }),
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
      ...puzzle3dPlayFixtureTreeRowFields(object.label, object.id),
      defaultOpen: true,
      command: puzzle3dPlaySelectObjectCommand(object.id),
      ...puzzle3dPlayHierarchyInstanceHoverHandlers(onHover, { kind: "object", id: object.id }),
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
    command: puzzle3dPlaySelectAttractionCommand(attraction.id),
    ...puzzle3dPlayHierarchyInstanceHoverHandlers(onHover, { kind: "attraction", id: attraction.id }),
  }));
  const attractionsGroup: UiTreeItemNode = {
    id: "puzzle-3d-play-hierarchy.attractions",
    label: "Attractions",
    defaultOpen: true,
    items: attractionItems.length ? attractionItems : [{ id: "puzzle-3d-play-hierarchy.attractions.empty", label: "(none)" }],
  };
  const viewportRoot: UiTreeItemNode = {
    id: "puzzle-3d-play-hierarchy.viewport",
    label: "Puzzle 3D",
    defaultOpen: true,
    items: [objectsGroup, attractionsGroup],
  };
  return playgroundTreePanelRootItems("puzzle-3d-play-hierarchy.root", [viewportRoot]).sections;
}

/** @emoji 🌳 Nested workbench tree: Puzzle 3D → Objects → Vortices; Attractions sibling group. */
export function buildPuzzle3dPlayHierarchyTree(
  fixture: FixtureV1 | null,
  selection: Puzzle3dPlaySelection,
  options?: Puzzle3dPlayHierarchyHoverBuildOptions & { readonly highlightedIds?: readonly string[] },
): UiNode {
  if (!fixture) {
    return playgroundTreePanelRootItems("puzzle-3d-play-hierarchy.root", [{ id: "puzzle-3d-play-hierarchy.invalid", label: "Invalid puzzle 3D fixture" }]);
  }
  return {
    type: "tree",
    sections: buildPuzzle3dPlayHierarchySections(fixture, options),
    selectedIds: puzzle3dPlayHierarchySelectedIds(selection),
    ...(options?.highlightedIds?.length ? { highlightedIds: options.highlightedIds } : {}),
  };
}
//#endregion 🔖Puzzle3dPlayHierarchy

//#region 🔖Puzzle3dPlayKinds
type Puzzle3dCatalogKind = ObjectKind | VortexKind | CableKind | AttractionKind;

function puzzle3dCatalogKindLabel(entry: Puzzle3dCatalogKind): string {
  const display = entry.label?.trim() || entry.name?.trim();
  return display && display.length > 0 ? display : entry.id;
}

function puzzle3dCatalogVortexKindLabel(vortexKindId: string, vortexKinds: readonly VortexKind[] | undefined): string {
  const entry = vortexKinds?.find((row) => row.id === vortexKindId);
  return entry ? puzzle3dCatalogKindLabel(entry) : vortexKindId;
}

function puzzle3dObjectKindVortexTemplateCatalogDescription(template: ObjectKindVortexTemplate): string {
  return template.position.map((n) => n.toFixed(1)).join(", ");
}

function puzzle3dPlayObjectKindVortexCatalogItems(
  sectionId: string,
  objectIndex: number,
  objectKindId: string,
  templates: readonly ObjectKindVortexTemplate[],
  vortexKinds: readonly VortexKind[] | undefined,
): readonly UiTreeItemNode[] {
  return templates.map((template, vortexIndex) => ({
    id: `${sectionId}.${objectIndex}.${objectKindId}.vortex.${vortexIndex}`,
    label: puzzle3dCatalogVortexKindLabel(template.vortexKind, vortexKinds),
    description: puzzle3dObjectKindVortexTemplateCatalogDescription(template),
  }));
}

function puzzle3dPlayKindCatalogSection(
  sectionId: string,
  label: string,
  entries: readonly Puzzle3dCatalogKind[] | undefined,
  vortexKinds?: readonly VortexKind[],
  sectionDefaultOpen = true,
  kindCatalogs?: KindCatalogBundle,
  sceneFixture?: FixtureV1,
  onHover?: (payload: Puzzle3dHoverPayload) => void,
): UiTreeSectionNode | null {
  if (!entries?.length) {
    return null;
  }
  const isObjectPalette = sectionId === "puzzle-3d-play-kinds.objects";
  const sectionDomain = puzzle3dPlayKindsSectionDomain(sectionId);
  const items: UiTreeItemNode[] = [...entries]
    .sort((a, b) => puzzle3dCatalogKindLabel(a).localeCompare(puzzle3dCatalogKindLabel(b)))
    .map((entry, index) => {
      const objectKind = isObjectPalette ? (entry as ObjectKind) : null;
      const vortexItems = objectKind?.vortices?.length
        ? puzzle3dPlayObjectKindVortexCatalogItems(sectionId, index, entry.id, objectKind.vortices, vortexKinds)
        : [];
      const kindHover = sectionDomain ? { domain: sectionDomain, kindId: entry.id } satisfies Puzzle3dKindHover : null;
      return {
        id: `${sectionId}.${index}.${entry.id}`,
        label: puzzle3dCatalogKindLabel(entry),
        description: entry.id,
        defaultOpen: vortexItems.length === 0,
        ...(vortexItems.length ? { items: vortexItems } : {}),
        ...(kindHover ? puzzle3dPlayKindRowHoverHandlers(onHover, kindHover) : {}),
        ...(isObjectPalette && isLoadableMeshUrl(resolveObjectKindMeshUrl(entry.id, kindCatalogs, sceneFixture))
          ? {
              draggable: true,
              dragData: puzzle3dPlayObjectKindDragData(entry.id),
            }
          : {}),
      };
    });
  return { id: sectionId, label, defaultOpen: sectionDefaultOpen, items };
}

/** @emoji 🏷️ Workbench kinds tab: Objects, Vortices, Cables, Attractions. */
export function buildPuzzle3dPlayKindsTree(
  catalogs: KindCatalogBundle | undefined,
  sceneFixture?: FixtureV1,
  options?: Puzzle3dPlayHierarchyHoverBuildOptions & { readonly highlightedIds?: readonly string[] },
): UiNode {
  const onHover = options?.onHover;
  const sections = [
    puzzle3dPlayKindCatalogSection("puzzle-3d-play-kinds.objects", "Objects", catalogs?.objects, catalogs?.vortices, true, catalogs, sceneFixture, onHover),
    puzzle3dPlayKindCatalogSection("puzzle-3d-play-kinds.vortices", "Vortices", catalogs?.vortices, undefined, false, undefined, undefined, onHover),
    puzzle3dPlayKindCatalogSection("puzzle-3d-play-kinds.cables", "Cables", catalogs?.cables, undefined, true, undefined, undefined, onHover),
    puzzle3dPlayKindCatalogSection("puzzle-3d-play-kinds.attractions", "Attractions", catalogs?.attractions, undefined, true, undefined, undefined, onHover),
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
  return {
    type: "tree",
    sections,
    ...(options?.highlightedIds?.length ? { highlightedIds: options.highlightedIds } : {}),
  };
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

function puzzle3dPlayPickKindLabel(kind: Puzzle3dPlayPickKind): string {
  if (kind === "object") return "Objects";
  if (kind === "vortex") return "Vortices";
  return "Attractions";
}

/** @emoji 🎬 Playground puzzle 3D play controller: fixture, LOD, selection/filter tools, and interaction counters. */
export class Puzzle3dPlayShellController extends Controller implements PlaygroundFixtureHost {
  private activeFixtureId = PUZZLE_3D_PLAY_FIXTURE_CONCRETE_FOREST_ID;
  readonly mainMode = new ModeRuntime("main", "Puzzle 3D", undefined);
  readonly selectableKinds: Record<Puzzle3dPlayPickKind, boolean> = { object: true, vortex: true, attraction: true };
  readonly visibleKinds: Record<Puzzle3dPlayPickKind, boolean> = { object: true, vortex: true, attraction: true };
  private fixture: FixtureV1 | null;
  private fixtureRevision: number;
  private automaticLod: boolean;
  private depthVariableLod: boolean;
  private manualLod: number;
  private lodSlider: number;
  private lodTag: number;
  private gumballConfig: GumballConfig;
  private activeTool: Puzzle3dActiveTool;
  private selection: Puzzle3dPlaySelection;
  private selectionMode: SelectionMode;
  private selectionMethod: SelectionMethod;
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
  private brushPlacementOverlapBudget: number;
  private objectKindIds: string[] = [];
  private vortexKindIds: string[] = [];
  private objectKindWeights: KindWeightMap = {};
  private vortexKindWeights: KindWeightMap = {};
  private snapshotListeners = new Set<() => void>();
  private snapshotCache: Puzzle3dPlaySnapshot | null = null;
  private windowEngagement: WindowEngagement | undefined;
  private lastEngagementRepeat: string | null = null;
  private hostBridge: Puzzle3dPlayHostBridge | null = null;
  private hierarchySectionsCache: readonly UiTreeSectionNode[] | null = null;
  private hierarchySectionsRevision = -1;
  private hoverFocus: Puzzle3dHoverPayload = { hoverTarget: null, kindHover: null };
  private instanceCameras = new Map<string, CameraState>();
  private cameraSeedEpoch = 0;
  private fixtureCatalogCache: PlaygroundFixtureCatalog | null = null;

  constructor(commandBus: CommandBus, hostNotify: () => void) {
    super(PUZZLE_3D_PLAY_CONTROLLER_ID, commandBus, hostNotify);
    this.fixture = parseFixtureV1(puzzle3dPlayFixtureJson(this.activeFixtureId));
    this.fixtureRevision = 0;
    this.automaticLod = true;
    this.depthVariableLod = false;
    this.manualLod = DEFAULT_MANUAL_LOD;
    this.lodSlider = sliderValueFromLod(DEFAULT_MANUAL_LOD);
    this.lodTag = DEFAULT_MANUAL_LOD;
    this.gumballConfig = { ...DEFAULT_GUMBALL_CONFIG };
    this.activeTool = "select";
    this.selection = PUZZLE_3D_PLAY_EMPTY_SELECTION;
    this.selectionMode = "default";
    this.selectionMethod = "rectangle";
    this.proximityRadius = 24;
    this.chunkSize = 256;
    this.gridFactor = 10;
    this.showLodGrid = true;
    this.gridSnapEnabled = true;
    this.proximityCount = 0;
    this.connectCount = 0;
    this.indirectCount = 0;
    this.compatibleObjectsCount = 0;
    this.targetRingCount = 0;
    this.brushPlacementOverlapBudget = DEFAULT_BRUSH_PLACEMENT_OVERLAP_BUDGET;
    this.windowEngagement = this.placeholderWindowEngagement();
    this.syncBrushKindWeightsFromFixture();
    this.rebuildShellMode();
    this.rebuildSnapshotCache();
    this.provideStore(PUZZLE_3D_PLAY_STORE_ID, new Puzzle3dPlaySnapshotStore(this));
  }

  private kindWeightLabel(kindId: string): string {
    const tail = kindId.split(".").pop() ?? kindId;
    return tail.length > 24 ? `${tail.slice(0, 21)}…` : tail;
  }

  private kindWeightMeasures(prefix: string, ids: readonly string[], weights: KindWeightMap, command: string): readonly WindowMeasure[] {
    return ids.map((kindId) => {
      const w = weights[kindId] ?? 0;
      return {
        kind: "slider" as const,
        id: `${PUZZLE_3D_PLAY_WINDOW_ID}-${prefix}-${kindId}`,
        label: `${this.kindWeightLabel(kindId)} ${(w * 100).toFixed(0)}%`,
        value: w,
        min: 0,
        max: 1,
        step: 0.01,
        onChange: puzzle3dPlayCmd(command, { kindId }),
      };
    });
  }

  private syncBrushKindWeightsFromFixture(): void {
    const catalogs = parseKindCatalogs(this.fixture?.meta as Record<string, unknown> | undefined);
    const objects = catalogs?.objects?.map((row) => row.id).filter((id): id is string => Boolean(id)) ?? [];
    const vortices = catalogs?.vortices?.map((row) => row.id).filter((id): id is string => Boolean(id)) ?? [];
    this.objectKindIds = [...objects];
    this.vortexKindIds = [...vortices];
    this.objectKindWeights = syncKindWeightMap(this.objectKindIds, this.objectKindWeights);
    this.vortexKindWeights = syncKindWeightMap(this.vortexKindIds, this.vortexKindWeights);
    installPuzzle3dPlayBrushHost(this.fixture?.meta as Record<string, unknown> | undefined);
    publishPuzzle3dBrushKindWeights(this.objectKindWeights, this.vortexKindWeights);
  }

  /** @emoji 🔔 Subscribes to snapshot-only updates (selection, fixture, lod) without shell generation bumps. */
  subscribeSnapshot(listener: () => void): () => void {
    this.snapshotListeners.add(listener);
    return () => this.snapshotListeners.delete(listener);
  }

  /** @emoji 🖱️ Updates shared hover focus for canvas + hierarchy transitive highlight. */
  setHoverFocus(payload: Puzzle3dHoverPayload): void {
    const nextKind =
      payload.kindHover ??
      (payload.hoverTarget
        ? puzzle3dKindHoverFromPlayTarget(this.fixture, payload.hoverTarget)
        : null);
    const next: Puzzle3dHoverPayload = { hoverTarget: payload.hoverTarget, kindHover: nextKind };
    const prev = this.hoverFocus;
    if (
      puzzle3dHoverTargetsEqual(prev.hoverTarget, next.hoverTarget) &&
      puzzle3dKindHoversEqual(prev.kindHover, next.kindHover)
    ) {
      return;
    }
    this.hoverFocus = next;
    this.rebuildSnapshotCache();
    for (const listener of this.snapshotListeners) {
      listener();
    }
  }

  /** @emoji 🌳 Hierarchy panel tree with stable {@link UiTreeNode.sections} across selection-only updates. */
  getHierarchyPanelTree(selection: Puzzle3dPlaySelection): UiNode {
    if (!this.fixture) {
      return playgroundTreePanelRootItems("puzzle-3d-play-hierarchy.root", [{ id: "puzzle-3d-play-hierarchy.invalid", label: "Invalid puzzle 3D fixture" }]);
    }
    if (this.hierarchySectionsRevision !== this.fixtureRevision || !this.hierarchySectionsCache) {
      this.hierarchySectionsCache = buildPuzzle3dPlayHierarchySections(this.fixture, {
        onHover: (payload) => this.setHoverFocus(payload),
      });
      this.hierarchySectionsRevision = this.fixtureRevision;
    }
    const highlightedIds = puzzle3dPlayHierarchyTreeHighlightedIds(this.fixture, this.hoverFocus.kindHover);
    return {
      type: "tree",
      sections: this.hierarchySectionsCache,
      selectedIds: puzzle3dPlayHierarchySelectedIds(selection),
      ...(highlightedIds.length ? { highlightedIds } : {}),
    };
  }

  getKindsPanelTree(): UiNode {
    const catalogs = this.fixture ? parseKindCatalogs(this.fixture.meta) : undefined;
    const highlightedIds = this.fixture ? puzzle3dPlayHierarchyTreeHighlightedIds(this.fixture, this.hoverFocus.kindHover) : [];
    return buildPuzzle3dPlayKindsTree(catalogs, this.fixture ?? undefined, {
      onHover: (payload) => this.setHoverFocus(payload),
      ...(highlightedIds.length ? { highlightedIds } : {}),
    });
  }

  private rebuildSnapshotCache(): void {
    this.snapshotCache = {
      fixture: this.fixture,
      fixtureRevision: this.fixtureRevision,
      lodProps: puzzle3dLodCanvasProps({
        automaticLod: this.automaticLod,
        depthVariableLod: this.depthVariableLod,
        manualLod: this.manualLod,
      }),
      lodTag: this.lodTag,
      lodSlider: this.lodSlider,
      automaticLod: this.automaticLod,
      depthVariableLod: this.depthVariableLod,
      gumballConfig: this.gumballConfig,
      activeTool: this.activeTool,
      selection: this.selection,
      selectedId: primaryPuzzle3dPlayObjectId(this.selection),
      selectedLabel: puzzle3dPlaySelectionLabel(this.fixture, this.selection),
      selectionMode: this.selectionMode,
      selectionMethod: this.selectionMethod,
      selectableKinds: { ...this.selectableKinds },
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
      brushPlacementOverlapBudget: this.brushPlacementOverlapBudget,
      cameraSeedEpoch: this.cameraSeedEpoch,
      hoverFocus: this.hoverFocus,
    };
  }

  /** @emoji 📷 Camera for a shell window instance (falls back to shared fixture camera). */
  cameraForInstance(instanceId?: string): CameraState {
    if (!this.fixture) {
      return { position: [420, -420, 320], target: [0, 0, 40], zoom: 1 };
    }
    if (instanceId) {
      const scoped = this.instanceCameras.get(instanceId);
      if (scoped) {
        return scoped;
      }
    }
    return this.fixture.camera;
  }

  private notifySnapshot(): void {
    this.rebuildSnapshotCache();
    for (const listener of this.snapshotListeners) {
      listener();
    }
  }

  /** @emoji 🎯 Refreshes the viewport snapshot and snapshot-subscribed panels without a shell generation bump. */
  private notifySelection(): void {
    this.notifySnapshot();
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

  getFixtureCatalog(): PlaygroundFixtureCatalog {
    if (!this.fixtureCatalogCache || this.fixtureCatalogCache.activeFixtureId !== this.activeFixtureId) {
      this.fixtureCatalogCache = { activeFixtureId: this.activeFixtureId, options: PUZZLE_3D_PLAY_FIXTURE_OPTIONS };
    }
    return this.fixtureCatalogCache;
  }

  private clearFixture(): void {
    this.fixture = null;
    this.fixtureRevision += 1;
    this.selection = PUZZLE_3D_PLAY_EMPTY_SELECTION;
    this.hierarchySectionsCache = null;
    this.hierarchySectionsRevision = -1;
    this.syncShell();
  }

  private loadFixtureById(fixtureId: string): void {
    if (isPlaygroundNoFixtureId(fixtureId)) {
      this.clearFixture();
      return;
    }
    const raw = PUZZLE_3D_PLAY_FIXTURE_JSON_BY_ID[fixtureId];
    if (!raw) return;
    const parsed = parseFixtureV1(raw);
    if (!parsed) return;
    this.fixture = parsed;
    this.fixtureRevision += 1;
    this.selection = PUZZLE_3D_PLAY_EMPTY_SELECTION;
    this.syncBrushKindWeightsFromFixture();
    this.syncShell();
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
      this.syncBrushKindWeightsFromFixture();
    }
    const poseChanged = fixturePoseFingerprint(next) !== fixturePoseFingerprint(prev);
    const appearanceChanged = fixtureAppearanceFingerprint(next) !== fixtureAppearanceFingerprint(prev);
    if (structureChanged) {
      this.syncShell();
    } else if (poseChanged || appearanceChanged) {
      this.notifySnapshot();
    }
  }

  /** @emoji ✋ Persists a gumball relocate on the fixture (pose-only; no React emit). */
  patchRelocate(payload: RelocatePayload, attractingByObjectId?: ReadonlyMap<string, readonly string[]>): void {
    if (!this.fixture) {
      return;
    }
    const next = applyRelocateToFixture(this.fixture, payload, attractingByObjectId);
    if (next === this.fixture) {
      return;
    }
    this.fixture = next;
  }

  /** @emoji 📷 Persists orbit camera on the fixture or a shell instance without structure revision bumps. */
  setCamera(camera: Partial<CameraState>, instanceId?: string): void {
    if (!this.fixture) {
      return;
    }
    if (instanceId) {
      const prev = this.instanceCameras.get(instanceId) ?? this.fixture.camera;
      const next: CameraState = { ...prev, ...camera };
      if (cameraStateNearEqual(prev, next)) {
        return;
      }
      this.instanceCameras.set(instanceId, next);
      this.cameraSeedEpoch += 1;
      this.notifySnapshot();
      return;
    }
    const next = updatePuzzle3dCameraInFixture(this.fixture, camera);
    if (next === this.fixture) {
      return;
    }
    this.fixture = next;
    this.cameraSeedEpoch += 1;
    this.notifySnapshot();
  }

  private applyOrbitCameraView(view: OrbitCameraViewId, instanceId?: string): void {
    if (!this.fixture) {
      return;
    }
    const current = this.cameraForInstance(instanceId);
    const next = computeOrbitCameraViewState(view, {
      target: current.target,
      distance: orbitCameraDistance(current),
      zoom: current.zoom,
    });
    if (instanceId) {
      this.instanceCameras.set(instanceId, next);
    } else {
      const updated = updatePuzzle3dCameraInFixture(this.fixture, next);
      if (updated === this.fixture) {
        return;
      }
      this.fixture = updated;
    }
    this.cameraSeedEpoch += 1;
    this.notifySnapshot();
  }

  private orbitCameraViewFromLegacyPreset(preset: string): OrbitCameraViewId | null {
    if (preset === "top" || preset === "bottom" || preset === "front" || preset === "back" || preset === "north" || preset === "south" || preset === "east" || preset === "west" || preset === "perspective") {
      return preset;
    }
    return null;
  }

  private lodMeasures(): readonly WindowMeasure[] {
    return [
      {
        kind: "toggle",
        id: `${PUZZLE_3D_PLAY_WINDOW_ID}-auto`,
        iconId: "zoom-in",
        text: "Auto zoom",
        pressed: this.automaticLod,
        onChange: { controllerId: PUZZLE_3D_PLAY_CONTROLLER_ID, command: "setAutoLod" },
      },
      {
        kind: "toggle",
        id: `${PUZZLE_3D_PLAY_WINDOW_ID}-depth`,
        iconId: "layers",
        text: "Depth-variable",
        pressed: this.depthVariableLod,
        onChange: { controllerId: PUZZLE_3D_PLAY_CONTROLLER_ID, command: "setDepthLod" },
      },
      {
        kind: "slider",
        id: `${PUZZLE_3D_PLAY_WINDOW_ID}-lod`,
        label: formatLod(this.lodTag),
        value: this.lodSlider,
        min: PUZZLE_3D_LOD_SLIDER_MIN,
        max: PUZZLE_3D_LOD_SLIDER_MAX,
        step: 1,
        onChange: { controllerId: PUZZLE_3D_PLAY_CONTROLLER_ID, command: "setManualLod" },
      },
      {
        kind: "toggle",
        id: `${PUZZLE_3D_PLAY_WINDOW_ID}-lod-grid`,
        iconId: "layout-grid",
        text: "Grid",
        pressed: this.showLodGrid,
        onChange: { controllerId: PUZZLE_3D_PLAY_CONTROLLER_ID, command: "setShowLodGrid" },
      },
    ];
  }

  private selectionMeasures(): readonly WindowMeasure[] {
    return [
      {
        kind: "toggle",
        id: `${PUZZLE_3D_PLAY_WINDOW_ID}-marquee-rectangle`,
        iconId: "square",
        text: "Rectangle",
        pressed: this.selectionMethod === "rectangle",
        onChange: { controllerId: PUZZLE_3D_PLAY_CONTROLLER_ID, command: "setSelectionMethod", args: { method: "rectangle" } },
      },
      {
        kind: "toggle",
        id: `${PUZZLE_3D_PLAY_WINDOW_ID}-marquee-lasso`,
        iconId: "lasso",
        text: "Lasso",
        pressed: this.selectionMethod === "lasso",
        onChange: { controllerId: PUZZLE_3D_PLAY_CONTROLLER_ID, command: "setSelectionMethod", args: { method: "lasso" } },
      },
      {
        kind: "toggle",
        id: `${PUZZLE_3D_PLAY_WINDOW_ID}-select-objects`,
        iconId: "box",
        text: "Objects",
        pressed: this.selectableKinds.object,
        onChange: { controllerId: PUZZLE_3D_PLAY_CONTROLLER_ID, command: "toggleSelectableKind", args: { kind: "object" } },
      },
      {
        kind: "toggle",
        id: `${PUZZLE_3D_PLAY_WINDOW_ID}-select-vortices`,
        iconId: "circle-dot",
        text: "Vortices",
        pressed: this.selectableKinds.vortex,
        onChange: { controllerId: PUZZLE_3D_PLAY_CONTROLLER_ID, command: "toggleSelectableKind", args: { kind: "vortex" } },
      },
      {
        kind: "toggle",
        id: `${PUZZLE_3D_PLAY_WINDOW_ID}-select-attractions`,
        iconId: "link",
        text: "Attractions",
        pressed: this.selectableKinds.attraction,
        onChange: { controllerId: PUZZLE_3D_PLAY_CONTROLLER_ID, command: "toggleSelectableKind", args: { kind: "attraction" } },
      },
    ];
  }

  private brushMeasuresGroup(): WindowMeasure {
    return {
      kind: "group",
      id: `${PUZZLE_3D_PLAY_WINDOW_ID}-brush`,
      label: "Brush",
      children: [
        {
          kind: "slider",
          id: `${PUZZLE_3D_PLAY_WINDOW_ID}-brush-overlap-budget`,
          label: "Overlap budget (m³)",
          value: this.brushPlacementOverlapBudget,
          min: 0,
          max: BRUSH_PLACEMENT_OVERLAP_BUDGET_MAX,
          step: BRUSH_PLACEMENT_OVERLAP_BUDGET_STEP,
          onChange: { controllerId: PUZZLE_3D_PLAY_CONTROLLER_ID, command: "setBrushPlacementOverlapBudget", args: { cad: true } },
        },
        {
          kind: "group",
          id: `${PUZZLE_3D_PLAY_WINDOW_ID}-brush-distribution`,
          label: "Distribution",
          defaultOpen: false,
          children: [
            {
              kind: "group",
              id: `${PUZZLE_3D_PLAY_WINDOW_ID}-brush-distribution-objects`,
              label: "Objects",
              defaultOpen: false,
              children: this.kindWeightMeasures("object-kind", this.objectKindIds, this.objectKindWeights, "setObjectKindWeight"),
            },
            {
              kind: "group",
              id: `${PUZZLE_3D_PLAY_WINDOW_ID}-brush-distribution-vortices`,
              label: "Vortices",
              defaultOpen: false,
              children: this.kindWeightMeasures("vortex-kind", this.vortexKindIds, this.vortexKindWeights, "setVortexKindWeight"),
            },
          ],
        },
      ],
    };
  }

  private windowMeasures(): readonly WindowMeasure[] {
    return [
      { kind: "group", id: `${PUZZLE_3D_PLAY_WINDOW_ID}-lod`, label: "LOD", children: this.lodMeasures() },
      { kind: "group", id: `${PUZZLE_3D_PLAY_WINDOW_ID}-select`, label: "Select", children: this.selectionMeasures() },
      this.brushMeasuresGroup(),
    ];
  }

  /** @emoji 💬 Placeholder engagement until the viewport host publishes a live snapshot (requires `input`). */
  placeholderWindowEngagement(): WindowEngagement {
    return {
      input: {
        id: "engagement-input",
        value: "",
        placeholder: "Brush",
        onChange: puzzle3dPlayCmd("engagementInput"),
        onSubmit: puzzle3dPlayCmd("engagementSubmit"),
        onRepeatLast: puzzle3dPlayCmd("engagementRepeatLast"),
      },
      possibleEngagements: [
        { id: PUZZLE_3D_ENGAGEMENT_TOOL_BRUSH_ID, label: "Brush", command: puzzle3dPlayCmd("engagementPossibleSelect", { possibleId: PUZZLE_3D_ENGAGEMENT_TOOL_BRUSH_ID }) },
        { id: PUZZLE_3D_ENGAGEMENT_TOOL_SELECT_ID, label: "Select", command: puzzle3dPlayCmd("engagementPossibleSelect", { possibleId: PUZZLE_3D_ENGAGEMENT_TOOL_SELECT_ID }) },
      ],
    };
  }

  /** @emoji 💬 Sets viewport window engagement from the live host publisher (CAD play layout). */
  setWindowEngagement(engagement: WindowEngagement | undefined): void {
    if (engagement) {
      enforcePlaygroundWindowEngagementInput(engagement, "Puzzle 3D play");
    }
    if (windowEngagementsEqual(this.windowEngagement, engagement)) {
      return;
    }
    this.windowEngagement = engagement;
    const existing = this.mainMode.windowKinds.find((wk) => wk.id === PUZZLE_3D_PLAY_WINDOW_ID);
    if (existing) {
      existing.engagement = engagement;
      this.mainMode.windowKinds = [...this.mainMode.windowKinds];
    } else {
      this.rebuildShellMode();
    }
    this.emit();
  }

  /** @emoji 🔗 Attaches the React host bridge for engagement commands. */
  setHostBridge(bridge: Puzzle3dPlayHostBridge | null): void {
    this.hostBridge = bridge;
  }

  /** @emoji 🔁 Records the last engagement command eligible for Space repeat. */
  private rememberEngagementRepeat(key: string): void {
    this.lastEngagementRepeat = key;
  }

  /** @emoji 🔁 Replays {@link lastEngagementRepeat} (tools in-shell, brush/zoom via host bridge). */
  private repeatLastEngagement(): void {
    const last = this.lastEngagementRepeat;
    if (!last) {
      return;
    }
    if (last === PUZZLE_3D_ENGAGEMENT_ZOOM_ID) {
      requestPuzzle3dZoomToSelection(this.selection);
      return;
    }
    if (this.applyEngagementToolCommand(last)) {
      return;
    }
    if (last.startsWith("puzzle3d.brush.")) {
      this.hostBridge?.runHostCommand("engagementPossibleSelect", { possibleId: last });
    }
  }

  /** @emoji 🖌️ Activates select or brush from engagement possibles or command-line tokens (no React host bridge required). */
  private applyEngagementToolCommand(possibleIdOrToken: string | undefined): boolean {
    if (!possibleIdOrToken) {
      return false;
    }
    if (possibleIdOrToken === PUZZLE_3D_ENGAGEMENT_TOOL_BRUSH_ID || puzzle3dPlayEngagementCommandToken(possibleIdOrToken) === "brush") {
      this.rememberEngagementRepeat(PUZZLE_3D_ENGAGEMENT_TOOL_BRUSH_ID);
      if (this.activeTool === "brush") {
        return true;
      }
      this.activeTool = "brush";
      this.notifySnapshot();
      return true;
    }
    if (possibleIdOrToken === PUZZLE_3D_ENGAGEMENT_TOOL_FILL_ID || puzzle3dPlayEngagementCommandToken(possibleIdOrToken) === "fill") {
      this.rememberEngagementRepeat(PUZZLE_3D_ENGAGEMENT_TOOL_FILL_ID);
      if (this.activeTool === "fill") {
        return true;
      }
      this.activeTool = "fill";
      this.notifySnapshot();
      return true;
    }
    if (possibleIdOrToken === PUZZLE_3D_ENGAGEMENT_TOOL_SELECT_ID || puzzle3dPlayEngagementCommandToken(possibleIdOrToken) === "select") {
      this.rememberEngagementRepeat(PUZZLE_3D_ENGAGEMENT_TOOL_SELECT_ID);
      if (this.activeTool === "select") {
        return true;
      }
      this.activeTool = "select";
      this.notifySnapshot();
      return true;
    }
    return false;
  }

  private rebuildShellMode(): void {
    this.mainMode.windowKinds = [
      new WindowKindRuntime(PUZZLE_3D_PLAY_WINDOW_ID, PUZZLE_3D_PLAY_WINDOW_LABEL, PUZZLE_3D_PLAY_BODY_KEY, undefined, this.windowMeasures(), this.windowEngagement, PUZZLE_3D_VIEW_TEMPLATES),
    ];
    const relocateTools: ToolItem[] = PUZZLE_3D_GUMBALL_GROUPS.map(({ key, label, iconId }, order) => ({
      id: `puzzle3d.gumball.${key}`,
      kind: "toggle" as const,
      iconId,
      text: label,
      order,
      pressed: this.gumballConfig[key] !== false,
      controllerId: PUZZLE_3D_PLAY_CONTROLLER_ID,
      command: "setGumballConfigToggle",
      args: { key },
    }));
    this.mainMode.tools = {
      selection: buildPlaygroundBrowseSelectionTools(PUZZLE_3D_PLAY_KINDS, puzzle3dPlayPickKindLabel, this.selectableKinds, PUZZLE_3D_PLAY_CONTROLLER_ID),
      filter: buildPlaygroundBrowseFilterTools(PUZZLE_3D_PLAY_KINDS, puzzle3dPlayPickKindLabel, this.visibleKinds, PUZZLE_3D_PLAY_CONTROLLER_ID),
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
      case "setActiveFixture": {
        const fixtureId = (args as { fixtureId?: string }).fixtureId ?? "";
        const nextId = isPlaygroundNoFixtureId(fixtureId) ? PLAYGROUND_NO_FIXTURE_ID : fixtureId;
        if (nextId === this.activeFixtureId) return;
        this.activeFixtureId = nextId;
        this.fixtureCatalogCache = null;
        this.loadFixtureById(nextId);
        return;
      }
      case ORBIT_CAMERA_VIEW_COMMAND: {
        const view = (args as { view?: OrbitCameraViewId }).view;
        const instanceId = (args as { instanceId?: string }).instanceId;
        if (!view) return;
        this.applyOrbitCameraView(view, instanceId);
        return;
      }
      case "setCameraPreset": {
        const preset = (args as { preset?: string; instanceId?: string }).preset;
        const instanceId = (args as { instanceId?: string }).instanceId;
        if (!preset) return;
        const view = this.orbitCameraViewFromLegacyPreset(preset);
        if (!view) return;
        this.applyOrbitCameraView(view, instanceId);
        return;
      }
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
      case "setGumballConfigToggle": {
        const key = (args as { key?: Puzzle3dGumballGroupKey }).key;
        if (!key || !PUZZLE_3D_GUMBALL_GROUPS.some((row) => row.key === key)) break;
        this.gumballConfig = { ...this.gumballConfig, [key]: this.gumballConfig[key] === false };
        this.syncShell();
        return;
      }
      case "setActiveTool": {
        const tool = (args as { tool?: Puzzle3dActiveTool }).tool;
        if (tool === "select" || tool === "brush" || tool === "fill") {
          this.activeTool = tool;
        }
        this.notifySnapshot();
        return;
      }
      case "setFillCount": {
        const count = Number((args as { count?: number }).count);
        if (!Number.isFinite(count) || !this.fixture) {
          return;
        }
        const catalogs = parseKindCatalogs(this.fixture.meta);
        const next = applyPuzzle3dFillCount(count, catalogs);
        if (!next) {
          return;
        }
        this.patchFixture(() => next);
        this.notifySnapshot();
        return;
      }
      case "setBrushPlacementOverlapBudget":
      case "setBrushPlacementCollisionTolerance": {
        const payload = args as { value?: number; cad?: boolean };
        const value = payload.value;
        if (typeof value !== "number" || !Number.isFinite(value)) {
          return;
        }
        this.brushPlacementOverlapBudget = Math.max(0, Math.min(BRUSH_PLACEMENT_OVERLAP_BUDGET_MAX, value));
        this.notifySnapshot();
        this.syncShell();
        return;
      }
      case "setObjectKindWeight": {
        const { kindId, value } = args as { kindId?: string; value?: number };
        if (typeof kindId !== "string" || !this.objectKindIds.includes(kindId)) {
          return;
        }
        const next = Number(value);
        if (!Number.isFinite(next)) {
          return;
        }
        this.objectKindWeights = normalizeKindWeightGroup(this.objectKindWeights, kindId, next);
        publishPuzzle3dBrushKindWeights(this.objectKindWeights, this.vortexKindWeights);
        this.syncShell();
        return;
      }
      case "setVortexKindWeight": {
        const { kindId, value } = args as { kindId?: string; value?: number };
        if (typeof kindId !== "string" || !this.vortexKindIds.includes(kindId)) {
          return;
        }
        const next = Number(value);
        if (!Number.isFinite(next)) {
          return;
        }
        this.vortexKindWeights = normalizeKindWeightGroup(this.vortexKindWeights, kindId, next);
        publishPuzzle3dBrushKindWeights(this.objectKindWeights, this.vortexKindWeights);
        this.syncShell();
        return;
      }
      case "cycleBrushCandidate": {
        if (this.activeTool !== "brush") {
          return;
        }
        puzzle3dBrushEngagementSourceRef.current.cycleCandidate();
        return;
      }
      case "engagementPossibleSelect": {
        const possibleId = (args as { possibleId?: string })?.possibleId;
        if (possibleId?.startsWith("puzzle3d.brush.")) {
          this.rememberEngagementRepeat(possibleId);
          this.hostBridge?.runHostCommand(command, args);
          return;
        }
        if (this.applyEngagementToolCommand(possibleId)) {
          return;
        }
        this.hostBridge?.runHostCommand(command, args);
        return;
      }
      case "engagementSubmit": {
        const value = (args as { value?: string })?.value ?? "";
        const token = puzzle3dPlayEngagementCommandToken(String(value).trim());
        if (this.applyEngagementToolCommand(token)) {
          return;
        }
        if (token === "zoom") {
          this.rememberEngagementRepeat(PUZZLE_3D_ENGAGEMENT_ZOOM_ID);
          requestPuzzle3dZoomToSelection(this.selection);
          return;
        }
        this.hostBridge?.runHostCommand(command, args);
        return;
      }
      case "engagementOption": {
        const optionId = (args as { optionId?: string })?.optionId;
        if (optionId === PUZZLE_3D_ENGAGEMENT_BRUSH_NEXT_ID && this.activeTool === "brush") {
          puzzle3dBrushEngagementSourceRef.current.cycleCandidate();
          return;
        }
        if (optionId === PUZZLE_3D_ENGAGEMENT_ZOOM_ID) {
          this.rememberEngagementRepeat(PUZZLE_3D_ENGAGEMENT_ZOOM_ID);
          requestPuzzle3dZoomToSelection(this.selection);
          return;
        }
        this.hostBridge?.runHostCommand(command, args);
        return;
      }
      case "rememberEngagementRepeat": {
        const key = (args as { key?: string })?.key;
        if (key) {
          this.rememberEngagementRepeat(key);
        }
        return;
      }
      case "engagementRepeatLast":
        this.repeatLastEngagement();
        return;
      case "engagementInput":
      case "engagementAbort":
      case "engagementControlChange":
      case "engagementControlCommit":
      case "engagementControlSelect":
        this.hostBridge?.runHostCommand(command, args);
        return;
      case "addBrushObject": {
        const payload = args as BrushPlacePayload;
        if (!payload?.targetVortexFullId || !payload.objectKindId) {
          return;
        }
        const catalogs = parseKindCatalogs(this.fixture?.meta);
        this.patchFixture((fixture) => applyBrushPlacementToFixture(fixture, payload, catalogs));
        this.notifySnapshot();
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
          if (puzzle3dPlaySelectionEqual(this.selection, resolved)) {
            return;
          }
          this.selection = resolved;
          this.notifySelection();
        }
        return;
      }
      case "setSelectedId": {
        const id = (args as { id: string | null }).id;
        const resolved: Puzzle3dPlaySelection = id ? { objectIds: [id], vortexIds: [], attractionIds: [] } : PUZZLE_3D_PLAY_EMPTY_SELECTION;
        if (puzzle3dPlaySelectionEqual(this.selection, resolved)) {
          return;
        }
        this.selection = resolved;
        this.notifySelection();
        return;
      }
      case "noteSelection": {
        const snap = args as SelectionSnapshot;
        const resolved = this.filterSelectionByPlaygroundKinds({
          objectIds: [...(snap.objectIds ?? [])],
          vortexIds: [...(snap.vortexIds ?? [])],
          attractionIds: [...(snap.attractionIds ?? [])],
        });
        if (puzzle3dPlaySelectionEqual(this.selection, resolved)) {
          return;
        }
        this.selection = resolved;
        this.notifySelection();
        return;
      }
      case "deleteSelection": {
        this.applyDeleteSelection();
        return;
      }
      case "selectAllSelection": {
        if (!this.fixture) {
          return;
        }
        const resolved = this.filterSelectionByPlaygroundKinds(
          puzzle3dPlayAllSelectionFromFixture(this.fixture, {
            object: this.selectableKinds.object && this.visibleKinds.object,
            vortex: this.selectableKinds.vortex && this.visibleKinds.vortex,
            attraction: this.selectableKinds.attraction && this.visibleKinds.attraction,
          }),
        );
        if (puzzle3dPlaySelectionEqual(this.selection, resolved)) {
          return;
        }
        this.selection = resolved;
        this.notifySelection();
        return;
      }
      case "patchPuzzle3dObjects": {
        const { objectIds, field, value } = args as {
          objectIds: readonly string[];
          field: "label" | "objectKind" | "origin" | "wormhole";
          value?: unknown;
        };
        if (!objectIds.length || !field) return;
        const catalogs = parseKindCatalogs(this.fixture?.meta);
        this.patchFixture((fixture) => {
          let next = fixture;
          for (const objectId of objectIds) {
            if (field === "objectKind" && typeof value === "string") {
              const object = next.objects.find((row) => row.id === objectId);
              if (!object) {
                continue;
              }
              next = {
                ...next,
                objects: patchPuzzle3dObject(next.objects, objectId, (row) => applyObjectKindToFixtureObject(row, value, catalogs, next)),
              };
              continue;
            }
            const patch: Partial<Omit<FixtureObjectV1, "id" | "vortices">> = {};
            if (field === "label" && typeof value === "string") patch.label = value;
            if (field === "wormhole" && typeof value === "string") patch.wormhole = value === "true";
            if (field === "origin" && Array.isArray(value) && value.length === 3) {
              patch.origin = [Number(value[0]), Number(value[1]), Number(value[2])] as [number, number, number];
            }
            next = updatePuzzle3dObjectInFixture(next, objectId, patch);
          }
          return next;
        });
        return;
      }
      case "renamePuzzle3dObject": {
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
      case "patchPuzzle3dVortex": {
        const { vortexFullId, field, value } = args as {
          vortexFullId: string;
          field: "label" | "vortexKind" | "position" | "direction" | "radius";
          value?: unknown;
        };
        const patch: Partial<VortexProps> = {};
        if (field === "label" && typeof value === "string") patch.label = value;
        if (field === "vortexKind" && typeof value === "string") patch.vortexKind = value;
        if (field === "position" && Array.isArray(value) && value.length === 3) {
          patch.position = [Number(value[0]), Number(value[1]), Number(value[2])] as [number, number, number];
        }
        if (field === "direction" && Array.isArray(value) && value.length === 3) {
          patch.direction = [Number(value[0]), Number(value[1]), Number(value[2])] as [number, number, number];
        }
        if (field === "radius" && typeof value === "number") patch.radius = value;
        if (field === "radius" && typeof value === "string") {
          const parsed = Number(value);
          if (Number.isFinite(parsed)) patch.radius = parsed;
        }
        this.patchFixture((fixture) => updatePuzzle3dVortexInFixture(fixture, vortexFullId, patch));
        return;
      }
      case "patchPuzzle3dAttraction": {
        const { attractionId, field, value } = args as {
          attractionId: string;
          field: "attracting" | "attracted" | "attractionKind";
          value?: unknown;
        };
        const patch: Partial<AttractionProps> = {};
        if (field === "attracting" && typeof value === "string") patch.attracting = value.trim() as AttractionProps["attracting"];
        if (field === "attracted" && typeof value === "string") patch.attracted = value.trim() as AttractionProps["attracted"];
        if (field === "attractionKind" && typeof value === "string") patch.attractionKind = value;
        this.patchFixture((fixture) => updatePuzzle3dAttractionInFixture(fixture, attractionId, patch));
        return;
      }
      case "setSelectionMode": {
        const mode = ((args as { mode?: SelectionMode; value?: string }).mode ?? (args as { value?: string }).value) as SelectionMode;
        if (mode === "default" || mode === "additive" || mode === "subtractive" || mode === "invertive") {
          this.selectionMode = mode;
          this.notifySnapshot();
        }
        return;
      }
      case "setSelectionMethod": {
        const method = (args as { method?: SelectionMethod }).method;
        if (method === "rectangle" || method === "lasso") {
          this.selectionMethod = method;
          this.syncShell();
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
          this.syncShell();
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
        next = deletePuzzle3dObjectFromFixture(next, objectId);
      }
      for (const vortexFullId of vortexIds) {
        next = deletePuzzle3dVortexFromFixture(next, vortexFullId);
      }
      for (const attractionId of attractionIds) {
        next = deletePuzzle3dAttractionFromFixture(next, attractionId);
      }
      return next;
    });
    this.selection = PUZZLE_3D_PLAY_EMPTY_SELECTION;
    this.notifySelection();
  }

  getSnapshot(): Puzzle3dPlaySnapshot {
    if (!this.snapshotCache) {
      this.rebuildSnapshotCache();
    }
    return this.snapshotCache!;
  }
}

/** @emoji 📸 Host-consumed puzzle 3D play state (no React/DOM). */
export interface Puzzle3dPlaySnapshot {
  readonly fixture: FixtureV1 | null;
  readonly fixtureRevision: number;
  readonly lodProps: ReturnType<typeof puzzle3dLodCanvasProps>;
  readonly lodTag: number;
  readonly lodSlider: number;
  readonly automaticLod: boolean;
  readonly depthVariableLod: boolean;
  readonly gumballConfig: GumballConfig;
  readonly activeTool: Puzzle3dActiveTool;
  readonly selection: Puzzle3dPlaySelection;
  readonly selectedId: string | null;
  readonly selectedLabel: string | null;
  readonly selectionMode: SelectionMode;
  readonly selectionMethod: SelectionMethod;
  readonly selectableKinds: MarqueeSelectableKinds;
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
  readonly brushPlacementOverlapBudget: number;
  readonly cameraSeedEpoch: number;
  readonly hoverFocus: Puzzle3dHoverPayload;
}

export const PUZZLE_3D_PLAY_STORE_ID = "play";

/** @emoji 🔗 Adapts {@link Puzzle3dPlayShellController} snapshot API to {@link Store}. */
class Puzzle3dPlaySnapshotStore extends Store<Puzzle3dPlaySnapshot> {
  private detach?: () => void;

  constructor(private readonly ctrl: Puzzle3dPlayShellController) {
    super();
    this.detach = ctrl.subscribeSnapshot(() => this.notify());
  }

  override getSnapshot(): Puzzle3dPlaySnapshot {
    return this.ctrl.getSnapshot();
  }

  override dispose(): void {
    this.detach?.();
    super.dispose();
  }
}

export function buildPuzzle3dPlayAppRuntime(controller: Puzzle3dPlayShellController): AppRuntime {
  const app = new AppRuntime(PUZZLE_3D_PLAY_APP_ID, "Puzzle 3D play", undefined, controller, createStackLayout([PUZZLE_3D_PLAY_WINDOW_ID], [PUZZLE_3D_PLAY_WINDOW_LABEL]) as never, [
    new WindowKindRuntime(PUZZLE_3D_PLAY_WINDOW_ID, PUZZLE_3D_PLAY_WINDOW_LABEL, PUZZLE_3D_PLAY_BODY_KEY, undefined, [], controller.placeholderWindowEngagement(), PUZZLE_3D_VIEW_TEMPLATES),
  ]);
  app.defaultModeId = controller.mainMode.id;
  app.addMode(controller.mainMode);
  controller.mainMode.namedLayouts = [
    createNamedLayout("puzzle-3d-quad", "Quad", {
      root: {
        kind: "row",
        children: [
          {
            kind: "column",
            size: 50,
            children: [
              { kind: "stack", size: 50, children: [createWindowLayout(PUZZLE_3D_PLAY_WINDOW_ID, "Top", { templateId: "top" })] },
              { kind: "stack", size: 50, children: [createWindowLayout(PUZZLE_3D_PLAY_WINDOW_ID, "North", { templateId: "north" })] },
            ],
          },
          { kind: "stack", size: 50, children: [createWindowLayout(PUZZLE_3D_PLAY_WINDOW_ID, "Perspective", { templateId: "perspective" })] },
        ],
      },
    }),
  ];
  app.panelTabs = [
    { id: PUZZLE_3D_PLAY_HIERARCHY_TAB_ID, iconId: PUZZLE_3D_PLAY_ICON_HIERARCHY, panel: "workbench", order: 0, bodyKey: PUZZLE_3D_PLAY_HIERARCHY_BODY_KEY },
    { id: PUZZLE_3D_PLAY_KINDS_TAB_ID, iconId: PUZZLE_3D_PLAY_ICON_KINDS, panel: "workbench", order: 1, bodyKey: PUZZLE_3D_PLAY_KINDS_BODY_KEY },
    { id: PUZZLE_3D_PLAY_INSPECTOR_TAB_ID, iconId: PUZZLE_3D_PLAY_ICON_INSPECTOR, panel: "details", order: 0, bodyKey: PUZZLE_3D_PLAY_INSPECTOR_BODY_KEY, label: "Inspector" },
    { id: PUZZLE_3D_PLAY_SETTINGS_TAB_ID, iconId: PUZZLE_3D_PLAY_ICON_SETTINGS, panel: "settings", order: 0, bodyKey: PUZZLE_3D_PLAY_SETTINGS_BODY_KEY, label: "Settings" },
  ];
  return app;
}

/** @emoji 🚀 Creates a {@link Platform} with puzzle 3D play app registered. */
export function buildPuzzle3dPlayRuntime(initialPanelVisibility?: { leftSidePanel: boolean; rightSidePanel: boolean }): Platform {
  const runtime = new Platform({ initialPanelVisibility });
  const controller = new Puzzle3dPlayShellController(runtime.commandBus, () => runtime.notify());
  runtime.addApp(buildPuzzle3dPlayAppRuntime(controller));
  return runtime;
}

function puzzle3dPlayControllerFromContext(ctx: WindowBodyViewContext): Puzzle3dPlayShellController | undefined {
  return platformFromViewContext(ctx)?.getActiveApp()?.controller as Puzzle3dPlayShellController | undefined;
}

/** @emoji 🧩 Declarative puzzle 3D window: fullscreen puzzle3d viewport only (relocate tools live on {@link ModeRuntime.tools}). */
export function buildPuzzle3dPlayDeclarativeBody(ctx: WindowBodyViewContext): UiNode {
  const ctrl = puzzle3dPlayControllerFromContext(ctx);
  if (!ctrl) {
    return { type: "text", value: "Missing puzzle 3D play controller" };
  }
  const snap = ctrl.getSnapshot();
  if (!snap.fixture) {
    return { type: "text", value: "Invalid puzzle 3D fixture" };
  }
  return buildPuzzle3dWindowBody(PUZZLE_3D_PLAY_VIEWPORT_SURFACE_ID, PUZZLE_3D_PLAY_CONTROLLER_ID);
}

function puzzle3dPlayAllEqual<T>(values: readonly T[]): boolean {
  if (values.length <= 1) return true;
  const first = values[0];
  for (let i = 1; i < values.length; i += 1) {
    if (values[i] !== first) return false;
  }
  return true;
}

function puzzle3dPlayVec3AllEqual(values: readonly (readonly [number, number, number])[]): boolean {
  if (values.length <= 1) return true;
  const first = values[0];
  for (let i = 1; i < values.length; i += 1) {
    const next = values[i]!;
    if (next[0] !== first![0] || next[1] !== first![1] || next[2] !== first![2]) {
      return false;
    }
  }
  return true;
}

type Puzzle3dPlayInspectorVortexRow = { readonly fullId: string; readonly vortex: VortexProps };

/** @emoji 🗺️ Resolves selected vortex rows via object index (O(V) not O(V×objects)). */
export function puzzle3dPlayInspectorVortexRows(fixture: FixtureV1, vortexFullIds: readonly string[]): Puzzle3dPlayInspectorVortexRow[] {
  const objectById = new Map(fixture.objects.map((object) => [object.id, object]));
  const rows: Puzzle3dPlayInspectorVortexRow[] = [];
  for (const fullId of vortexFullIds) {
    const { objectId, vortexId } = parseVortexFullId(fullId);
    const object = objectById.get(objectId);
    const vortex = object?.vortices.find((entry) => puzzle3dVortexFullId(objectId, entry.id) === fullId || entry.id === vortexId);
    if (vortex) {
      rows.push({ fullId, vortex });
    }
  }
  return rows;
}

/** @emoji 🔎 Declarative inspector panel for puzzle 3D play selection. */
export function buildPuzzle3dPlayInspectorBody(ctx: WindowBodyViewContext): UiTreeNode {
  const ctrl = puzzle3dPlayControllerFromContext(ctx);
  const snap = ctrl?.getSnapshot();
  const fixture = snap?.fixture;
  if (!ctrl || !snap || !fixture) {
    return uiDeclarativeSectionsToTree([
      { type: "section", id: "puzzle-3d-play-inspector.invalid", label: "Inspector", children: [{ type: "text", value: "Invalid puzzle 3D fixture" }] },
    ]);
  }
  const selection = snap.selection;
  const hasSelection = selection.objectIds.length > 0 || selection.vortexIds.length > 0 || selection.attractionIds.length > 0;
  const catalogs = parseKindCatalogs(fixture.meta);
  const objectKinds = catalogs?.objects ?? [];
  const vortexKinds = catalogs?.vortices ?? [];
  const attractionKinds = catalogs?.attractions ?? [];
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
          command: puzzle3dPlayCmd("deleteSelection"),
        },
      ],
    },
  ];
  const selectedObjectIdSet = new Set(selection.objectIds);
  if (selection.objectIds.length > 0) {
    const objects = fixture.objects.filter((object) => selectedObjectIdSet.has(object.id));
    const labels = objects.map((object) => object.label ?? "");
    const labelUniform = puzzle3dPlayAllEqual(labels);
    const kinds = objects.map((object) => object.objectKind ?? "");
    const kindUniform = puzzle3dPlayAllEqual(kinds);
    const origins = objects.map((object) => object.origin);
    const originUniform = puzzle3dPlayAllEqual(origins);
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
          onChange: puzzle3dPlayCmd("renamePuzzle3dObject", { oldId: selection.objectIds[0] }),
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
          onChange: puzzle3dPlayCmd("patchPuzzle3dObjects", { objectIds: selection.objectIds, field: "label" }),
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
          items: puzzle3dPlayKindCatalogSelectItems(objectKinds),
          onChange: puzzle3dPlayCmd("patchPuzzle3dObjects", { objectIds: selection.objectIds, field: "objectKind" }),
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
          onChange: puzzle3dPlayCmd("patchPuzzle3dObjects", { objectIds: selection.objectIds, field: "origin" }),
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
  const selectedVortexRows = puzzle3dPlayInspectorVortexRows(fixture, selection.vortexIds);
  if (selectedVortexRows.length === 1) {
    const { fullId: vortexFullId, vortex } = selectedVortexRows[0]!;
    const vortexFields: UiNode[] = [
      {
        type: "field",
        id: "puzzle-3d-play-inspector.vortex.id",
        label: "Full id",
        child: { type: "text", value: vortexFullId },
      },
      {
        type: "field",
        id: "puzzle-3d-play-inspector.vortex.label",
        label: "Label",
        child: {
          type: "input",
          id: "puzzle-3d-play-inspector.vortex.label.input",
          inputKind: "text",
          value: vortex.label ?? "",
          onChange: puzzle3dPlayCmd("patchPuzzle3dVortex", { vortexFullId, field: "label" }),
        },
      },
      {
        type: "field",
        id: "puzzle-3d-play-inspector.vortex.kind",
        label: "Vortex kind",
        child: {
          type: "select",
          id: "puzzle-3d-play-inspector.vortex.kind.select",
          value: vortex.vortexKind ?? "",
          items: puzzle3dPlayKindCatalogSelectItems(vortexKinds),
          onChange: puzzle3dPlayCmd("patchPuzzle3dVortex", { vortexFullId, field: "vortexKind" }),
        },
      },
      {
        type: "field",
        id: "puzzle-3d-play-inspector.vortex.position",
        label: "Position",
        child: {
          type: "vec3",
          id: "puzzle-3d-play-inspector.vortex.position.vec3",
          value: vortex.position as [number, number, number],
          onChange: puzzle3dPlayCmd("patchPuzzle3dVortex", { vortexFullId, field: "position" }),
        },
      },
    ];
    if (vortex.direction) {
      vortexFields.push({
        type: "field",
        id: "puzzle-3d-play-inspector.vortex.direction",
        label: "Direction",
        child: {
          type: "vec3",
          id: "puzzle-3d-play-inspector.vortex.direction.vec3",
          value: vortex.direction as [number, number, number],
          onChange: puzzle3dPlayCmd("patchPuzzle3dVortex", { vortexFullId, field: "direction" }),
        },
      });
    }
    vortexFields.push({
      type: "field",
      id: "puzzle-3d-play-inspector.vortex.radius",
      label: "Radius",
      child: {
        type: "input",
        id: "puzzle-3d-play-inspector.vortex.radius.input",
        inputKind: "number",
        value: String(vortex.radius ?? 0.35),
        onChange: puzzle3dPlayCmd("patchPuzzle3dVortex", { vortexFullId, field: "radius" }),
      },
    });
    children.push({
      type: "section",
      id: "puzzle-3d-play-inspector.vortex",
      label: puzzle3dPlayFixtureRowLabel(vortex.label, vortexFullId),
      children: vortexFields,
    });
  } else if (selectedVortexRows.length > 1) {
    const labels = selectedVortexRows.map((row) => row.vortex.label ?? "");
    const kinds = selectedVortexRows.map((row) => row.vortex.vortexKind ?? "");
    const positions = selectedVortexRows.map((row) => row.vortex.position);
    const radii = selectedVortexRows.map((row) => row.vortex.radius ?? 0.35);
    const labelUniform = puzzle3dPlayAllEqual(labels);
    const kindUniform = puzzle3dPlayAllEqual(kinds);
    const positionUniform = puzzle3dPlayVec3AllEqual(positions);
    const radiusUniform = puzzle3dPlayAllEqual(radii);
    children.push({
      type: "section",
      id: "puzzle-3d-play-inspector.vortices",
      label: `Vortices (${selectedVortexRows.length})`,
      children: [
        {
          type: "field",
          id: "puzzle-3d-play-inspector.vortices.label",
          label: "Label",
          child: {
            type: "text",
            value: labelUniform ? (labels[0] ?? "") || "—" : "Mixed",
          },
        },
        {
          type: "field",
          id: "puzzle-3d-play-inspector.vortices.kind",
          label: "Vortex kind",
          child: { type: "text", value: kindUniform ? kinds[0] || "—" : "Mixed" },
        },
        {
          type: "field",
          id: "puzzle-3d-play-inspector.vortices.position",
          label: "Position",
          child: {
            type: "text",
            value: positionUniform ? `[${positions[0]!.map(formatNumber).join(", ")}]` : "Mixed",
          },
        },
        {
          type: "field",
          id: "puzzle-3d-play-inspector.vortices.radius",
          label: "Radius",
          child: { type: "text", value: radiusUniform ? String(radii[0]) : "Mixed" },
        },
      ],
    });
  }
  const attractionById = new Map(fixture.attractions.map((attraction) => [attraction.id, attraction]));
  const selectedAttractions = selection.attractionIds.map((id) => attractionById.get(id)).filter((row): row is NonNullable<typeof row> => row !== undefined);
  if (selectedAttractions.length === 1) {
    const attraction = selectedAttractions[0]!;
    const attractionId = attraction.id;
    children.push({
      type: "section",
      id: "puzzle-3d-play-inspector.attraction",
      label: attraction.id,
      children: [
        {
          type: "field",
          id: "puzzle-3d-play-inspector.attraction.kind",
          label: "Attraction kind",
          child: {
            type: "select",
            id: "puzzle-3d-play-inspector.attraction.kind.select",
            value: attraction.attractionKind ?? "",
            items: puzzle3dPlayKindCatalogSelectItems(attractionKinds),
            onChange: puzzle3dPlayCmd("patchPuzzle3dAttraction", { attractionId, field: "attractionKind" }),
          },
        },
        {
          type: "field",
          id: "puzzle-3d-play-inspector.attraction.attracting",
          label: "Attracting",
          child: {
            type: "input",
            id: "puzzle-3d-play-inspector.attraction.attracting.input",
            inputKind: "text",
            value: attraction.attracting,
            onChange: puzzle3dPlayCmd("patchPuzzle3dAttraction", { attractionId, field: "attracting" }),
          },
        },
        {
          type: "field",
          id: "puzzle-3d-play-inspector.attraction.attracted",
          label: "Attracted",
          child: {
            type: "input",
            id: "puzzle-3d-play-inspector.attraction.attracted.input",
            inputKind: "text",
            value: attraction.attracted,
            onChange: puzzle3dPlayCmd("patchPuzzle3dAttraction", { attractionId, field: "attracted" }),
          },
        },
      ],
    });
  } else if (selectedAttractions.length > 1) {
    const kinds = selectedAttractions.map((row) => row.attractionKind ?? "");
    const attracting = selectedAttractions.map((row) => row.attracting);
    const attracted = selectedAttractions.map((row) => row.attracted);
    children.push({
      type: "section",
      id: "puzzle-3d-play-inspector.attractions",
      label: `Attractions (${selectedAttractions.length})`,
      children: [
        {
          type: "field",
          id: "puzzle-3d-play-inspector.attractions.kind",
          label: "Attraction kind",
          child: { type: "text", value: puzzle3dPlayAllEqual(kinds) ? kinds[0] || "—" : "Mixed" },
        },
        {
          type: "field",
          id: "puzzle-3d-play-inspector.attractions.attracting",
          label: "Attracting",
          child: { type: "text", value: puzzle3dPlayAllEqual(attracting) ? attracting[0]! : "Mixed" },
        },
        {
          type: "field",
          id: "puzzle-3d-play-inspector.attractions.attracted",
          label: "Attracted",
          child: { type: "text", value: puzzle3dPlayAllEqual(attracted) ? attracted[0]! : "Mixed" },
        },
      ],
    });
  }
  return uiDeclarativeSectionsToTree(children as UiSectionNode[]);
}

/** @emoji ⚙️ Declarative settings panel for puzzle 3D play. */
export function buildPuzzle3dPlaySettingsBody(ctx: WindowBodyViewContext): UiTreeNode {
  const ctrl = puzzle3dPlayControllerFromContext(ctx);
  const snap = ctrl?.getSnapshot();
  if (!snap) {
    return uiDeclarativeSectionsToTree([
      { type: "section", id: "puzzle-3d-play-settings.missing", label: "Settings", children: [{ type: "text", value: "Missing puzzle 3D play controller" }] },
    ]);
  }
  return uiDeclarativeSectionsToTree([
      {
        type: "section",
        id: "puzzle-3d-play-settings.root",
        label: "Puzzle 3D options",
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
                { value: "default", label: "default" },
                { value: "additive", label: "additive" },
                { value: "subtractive", label: "subtractive" },
                { value: "invertive", label: "invertive" },
              ],
              onChange: puzzle3dPlayCmd("setSelectionMode"),
            },
          },
          {
            type: "field",
            id: "puzzle-3d-play-settings.brushOverlapBudget",
            label: "Brush overlap budget (m³)",
            child: {
              type: "input",
              id: "puzzle-3d-play-settings.brushOverlapBudget.input",
              inputKind: "number",
              value: formatNumber(snap.brushPlacementOverlapBudget),
              onChange: puzzle3dPlayCmd("setBrushPlacementOverlapBudget", { cad: true }),
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
              onChange: puzzle3dPlayCmd("setProximityRadius", { value: 0 }),
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
              onChange: puzzle3dPlayCmd("setChunkSize", { value: 0 }),
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
              onChange: puzzle3dPlayCmd("setGridFactor", { value: 0 }),
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
  ]);
}

export function buildPuzzle3dPlayHierarchyPanelBody(ctx: WindowBodyViewContext): UiTreeNode {
  const ctrl = puzzle3dPlayControllerFromContext(ctx);
  const snap = ctrl?.getSnapshot();
  if (ctrl) {
    return ctrl.getHierarchyPanelTree(snap?.selection ?? PUZZLE_3D_PLAY_EMPTY_SELECTION);
  }
  return buildPuzzle3dPlayHierarchyTree(snap?.fixture ?? null, snap?.selection ?? PUZZLE_3D_PLAY_EMPTY_SELECTION);
}

export function buildPuzzle3dPlayKindsPanelBody(ctx: WindowBodyViewContext): UiTreeNode {
  const ctrl = puzzle3dPlayControllerFromContext(ctx);
  if (ctrl) {
    return ctrl.getKindsPanelTree();
  }
  const snap = ctrl?.getSnapshot();
  const catalogs = snap?.fixture ? parseKindCatalogs(snap.fixture.meta) : undefined;
  return buildPuzzle3dPlayKindsTree(catalogs, snap?.fixture ?? undefined);
}

/** @emoji 🛝 Puzzle 3D play harness as a single {@link Playground} instance. */
export class Playground3d extends Playground {
  readonly id = PUZZLE_3D_PLAY_APP_ID;
  readonly initialPanelVisibility = { leftSidePanel: true, rightSidePanel: true };
  readonly keybindings = [
    { key: "ctrl+a,meta+a", controllerId: PUZZLE_3D_PLAY_CONTROLLER_ID, command: "selectAllSelection" },
    { key: "Delete", controllerId: PUZZLE_3D_PLAY_CONTROLLER_ID, command: "deleteSelection" },
    { key: "Backspace", controllerId: PUZZLE_3D_PLAY_CONTROLLER_ID, command: "deleteSelection" },
  ];

  createRuntime(): Platform {
    return buildPuzzle3dPlayRuntime(this.initialPanelVisibility);
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

  function flattenWindowMeasures(measures: readonly WindowMeasure[]): WindowMeasure[] {
    const out: WindowMeasure[] = [];
    for (const measure of measures) {
      if (measure.kind === "group") {
        out.push(...flattenWindowMeasures(measure.children));
      } else {
        out.push(measure);
      }
    }
    return out;
  }

  describe("puzzle 3D play fixture", () => {
    it("parses nakagin fixture", () => {
      const f = parseFixtureV1(nakaginPuzzle3dFixtureJson as unknown);
      expect(f?.domain).toBe("architecture");
      expect(f?.attractions).toEqual([]);
      expect(f?.objects.length).toBeGreaterThan(0);
    });

    it("getFixtureCatalog returns a stable snapshot reference for useSyncExternalStore", () => {
      const bus = new CommandBus();
      const ctrl = new Puzzle3dPlayShellController(bus, () => {});
      const first = ctrl.getFixtureCatalog();
      const second = ctrl.getFixtureCatalog();
      expect(second).toBe(first);
      ctrl.run("setActiveFixture", { fixtureId: PUZZLE_3D_PLAY_FIXTURE_NAKAGIN_ID });
      const third = ctrl.getFixtureCatalog();
      expect(third).not.toBe(first);
      expect(third.activeFixtureId).toBe(PUZZLE_3D_PLAY_FIXTURE_NAKAGIN_ID);
    });

    it("parses concrete forest fixture with b and c vortex compatibility rules", () => {
      const f = parseFixtureV1(concreteForestPuzzle3dFixtureJson as unknown);
      const catalogs = parseKindCatalogs(f?.meta as Record<string, unknown> | undefined);
      const compat = parseKindCompatibility(f?.meta as Record<string, unknown> | undefined);
      expect(f?.objects).toHaveLength(1);
      expect(catalogs?.objects?.map((row) => row.id)).toEqual([
        "Hexagonal Cut Concrete Forest Left",
        "Hexagonal Cut Concrete Forest Right",
      ]);
      expect(kindsCompatible("b-l", "b-s-m", compat)).toBe(true);
      expect(kindsCompatible("b-l", "c-b", compat)).toBe(false);
      expect(kindsCompatible("c-b", "c-t", compat)).toBe(true);
      expect(kindsCompatible("c-b", "c-b", compat)).toBe(false);
      expect(kindsCompatible("c-t", "c-t", compat)).toBe(false);
      const target: AttractionVortexContext = {
        objectId: "seed-left-001",
        objectKind: "Hexagonal Cut Concrete Forest Left",
        vortexKind: "b-l",
      };
      const candidates = brushCompatibleCandidates(target, catalogs, compat);
      expect(candidates.some((entry) => entry.objectKindId === "Hexagonal Cut Concrete Forest Right")).toBe(true);
    });

    it("concrete forest object kinds resolve abbau-aufbau mesh urls for fill preload", () => {
      const f = parseFixtureV1(concreteForestPuzzle3dFixtureJson as unknown);
      const catalogs = parseKindCatalogs(f?.meta as Record<string, unknown> | undefined);
      expect(resolveObjectKindMeshUrl("Hexagonal Cut Concrete Forest Left", catalogs, f ?? undefined)).toBe(
        "/meshes/hexagonal-cut-concrete-forest-left.glb",
      );
      expect(resolveObjectKindMeshUrl("Hexagonal Cut Concrete Forest Right", catalogs, f ?? undefined)).toBe(
        "/meshes/hexagonal-cut-concrete-forest-right.glb",
      );
      const urls = brushMeshUrlsForFillSession(f!, catalogs, parseKindCompatibility(f?.meta as Record<string, unknown> | undefined));
      expect(urls).toEqual(
        expect.arrayContaining([
          "/meshes/hexagonal-cut-concrete-forest-left.glb",
          "/meshes/hexagonal-cut-concrete-forest-right.glb",
        ]),
      );
    });

    it("nakagin scene object meshUrl matches kind catalog for every placed object", () => {
      const fixture = parseFixtureV1(nakaginPuzzle3dFixtureJson as unknown);
      const catalogs = parseKindCatalogs(fixture?.meta);
      expect(fixture).toBeTruthy();
      for (const object of fixture!.objects) {
        const kindId = object.objectKind?.trim() ?? "";
        if (!kindId) {
          continue;
        }
        const expected = resolveObjectKindMeshUrl(kindId, catalogs, fixture!);
        if (!expected) {
          continue;
        }
        expect(object.meshUrl).toBe(expected);
      }
    });

    it("nakagin fixture kind catalog uses specific human-readable object kind names", () => {
      const catalogs = parseKindCatalogs((nakaginPuzzle3dFixtureJson as { meta?: Record<string, unknown> }).meta);
      const objectKindIds = (catalogs?.objects ?? []).map((row) => row.id);
      expect(objectKindIds).toEqual(
        expect.arrayContaining(["Capsule With Balcony J", "Trapezoid Capsule J", "Last Storey Tambour", "First Storey Tambour", "Cylindric Tambour"]),
      );
      expect(objectKindIds.some((id) => id === "J" || id === "Last Storey" || id === "Tambour Last Storey")).toBe(false);
      const f = parseFixtureV1(nakaginPuzzle3dFixtureJson as unknown);
      const placedKinds = new Set((f?.objects ?? []).map((object) => object.objectKind));
      expect(placedKinds.has("Capsule With Balcony J")).toBe(true);
      expect(placedKinds.has("Capsule J")).toBe(false);
      expect(placedKinds.has("J")).toBe(false);
    });

    it("nakagin Bridge kind resolves meshUrl from catalog", () => {
      const f = parseFixtureV1(nakaginPuzzle3dFixtureJson as unknown);
      const catalogs = parseKindCatalogs(f?.meta as Record<string, unknown> | undefined);
      expect(resolveObjectKindMeshUrl("Bridge", catalogs, f ?? undefined)).toBe("/meshes/bridge.glb");
      for (const object of f?.objects ?? []) {
        if (object.objectKind === "Bridge") {
          expect(object.meshUrl).toBe("/meshes/bridge.glb");
        }
      }
    });

    it("builds canonical vortex full ids", () => {
      expect(puzzle3dVortexFullId("obj", "vx")).toBe("obj:vx");
      expect(puzzle3dVortexFullId("obj", "obj:vx")).toBe("obj:vx");
    });

    it("splits fixture captions into primary label and muted id without a separator glyph", () => {
      expect(puzzle3dPlayFixtureTreeRowFields("Ellipsoid Capsule · cs_1_d0_t_f4_b_c1", "object-1")).toEqual({
        label: "Ellipsoid Capsule",
        description: "cs_1_d0_t_f4_b_c1",
      });
      expect(puzzle3dPlayFixtureRowLabel("Ellipsoid Capsule · cs_1_d0_t_f4_b_c1", "object-1")).toBe("Ellipsoid Capsule");
    });

    describe("nakagin kind catalog helpers", () => {
    it("upgrades plain Capsule J when a more specific catalog name exists", () => {
      const available = new Set(["Capsule J", "Capsule With Balcony J", "Trapezoid Capsule J", "Tambour"]);
      expect(puzzle3dPlayPreferSpecificCapsuleKindName("Capsule J", available)).toBe("Capsule With Balcony J");
      expect(puzzle3dPlayPreferSpecificCapsuleKindName("Tambour", available)).toBe("Tambour");
    });

    it("relabels door capsule vortex kinds from kind id", () => {
      const catalogs: KindCatalogBundle = {
        objects: [
          {
            id: "Capsule J",
            meshUrl: "/meshes/capsule_J.glb",
            vortices: [{ vortexKind: "door capsule right", position: [-1.3, -1.25, 0], direction: [-1, 0, 0], radius: 0.36 }],
          },
          {
            id: "Capsule L",
            meshUrl: "/meshes/capsule_L.glb",
            vortices: [{ vortexKind: "door capsule right", position: [1.3, -1.25, 0], direction: [1, 0, 0], radius: 0.36 }],
          },
        ],
        vortices: [{ id: "door capsule right" }, { id: "door capsule left" }, { id: "door tambour right" }],
      };
      const enriched = parseKindCatalogs({ kindCatalogs: catalogs })!;
      const target: AttractionVortexContext = { objectId: "host", objectKind: "Tambour", vortexKind: "door tambour right" };
      const compat: readonly KindCompatEntry[] = [
        { bidirectional: true, specificity: "vortex", source: "door capsule right", target: "door tambour right" },
      ];
      const list = brushCompatibleCandidates(target, enriched, compat);
      expect(list.some((entry) => entry.objectKindId === "Capsule J")).toBe(true);
      expect(list.some((entry) => entry.objectKindId === "Capsule L")).toBe(false);
      expect(enriched.objects.find((k) => k.id === "Capsule L")?.vortices?.map((v) => v.vortexKind)).toEqual(["door capsule left"]);
    });

    it("nakagin door tambour left has horizontal door capsule compatible candidates", () => {
      const fixture = parseFixtureV1(nakaginPuzzle3dFixtureJson as unknown);
      const catalogs = parseKindCatalogs(fixture?.meta as Record<string, unknown> | undefined);
      const compat = parseKindCompatibility(fixture?.meta as Record<string, unknown> | undefined);
      expect(catalogs).toBeTruthy();
      installPuzzle3dPlayBrushHost(fixture?.meta as Record<string, unknown> | undefined);
      const target: AttractionVortexContext = { objectId: "host", objectKind: "Tambour", vortexKind: "door tambour left" };
      const list = brushCompatibleCandidates(target, catalogs, compat);
      const ids = list.map((entry) => entry.objectKindId);
      expect(ids).toContain("Capsule L");
      expect(ids).toContain("Capsule Z");
      expect(ids.length).toBeGreaterThan(0);
    });

    it("brush on rectangular base rejects cylindric first storey tambour", () => {
      const fixture = parseFixtureV1(nakaginPuzzle3dFixtureJson as unknown);
      const catalogs = parseKindCatalogs(fixture?.meta as Record<string, unknown> | undefined);
      const compat = parseKindCompatibility(fixture?.meta as Record<string, unknown> | undefined);
      const target: AttractionVortexContext = { objectId: "host", objectKind: "Base", vortexKind: "core rectangular bottom" };
      const list = brushCompatibleCandidates(target, catalogs, compat);
      const ids = list.map((entry) => entry.objectKindId);
      expect(ids).toContain("First Storey Tambour");
      expect(ids).not.toContain("Cylindric First Storey Tambour");
      expect(ids).not.toContain("Cylindric Tambour");
    });

    it("brush on tambour circular stack prefers cylindric tambour over capital", () => {
      const fixture = parseFixtureV1(nakaginPuzzle3dFixtureJson as unknown);
      const catalogs = parseKindCatalogs(fixture?.meta as Record<string, unknown> | undefined);
      const compat = parseKindCompatibility(fixture?.meta as Record<string, unknown> | undefined);
      installPuzzle3dPlayBrushHost(fixture?.meta as Record<string, unknown> | undefined);
      const target: AttractionVortexContext = { objectId: "host", objectKind: "Tambour", vortexKind: "tambour circular top" };
      const list = brushCompatibleCandidates(target, catalogs, compat);
      const ids = list.map((entry) => entry.objectKindId);
      expect(ids).toContain("Cylindric Tambour");
      expect(ids).not.toContain("Cylindric Capital");
      expect(ids).not.toContain("Cylindric Last Storey Tambour");
      expect(ids[0]).toBe("Cylindric Tambour");
    });

    it("brush on last storey roof port still suggests cylindric capital", () => {
      const fixture = parseFixtureV1(nakaginPuzzle3dFixtureJson as unknown);
      const catalogs = parseKindCatalogs(fixture?.meta as Record<string, unknown> | undefined);
      const compat = parseKindCompatibility(fixture?.meta as Record<string, unknown> | undefined);
      installPuzzle3dPlayBrushHost(fixture?.meta as Record<string, unknown> | undefined);
      const target: AttractionVortexContext = {
        objectId: "host",
        objectKind: "Last Storey Tambour",
        vortexKind: "roof circular bottom",
      };
      const list = brushCompatibleCandidates(target, catalogs, compat);
      expect(list.map((entry) => entry.objectKindId)).toEqual(["Cylindric Capital"]);
    });

    it("cylindric first storey tambour participates when stacking below first storey", () => {
      const fixture = parseFixtureV1(nakaginPuzzle3dFixtureJson as unknown);
      const catalogs = parseKindCatalogs(fixture?.meta as Record<string, unknown> | undefined);
      const compat = parseKindCompatibility(fixture?.meta as Record<string, unknown> | undefined);
      installPuzzle3dPlayBrushHost(fixture?.meta as Record<string, unknown> | undefined);
      const target: AttractionVortexContext = {
        objectId: "host",
        objectKind: "First Storey Tambour",
        vortexKind: "tambour circular bottom",
      };
      const list = brushCompatibleCandidates(target, catalogs, compat);
      const ids = list.map((entry) => entry.objectKindId);
      expect(ids).toContain("Cylindric First Storey Tambour");
      expect(ids).not.toContain("Cylindric Capital");
    });

    it("brush candidate accept on door tambour left lists horizontal door capsules only", () => {
      const doorCatalogs: KindCatalogBundle = {
        objects: [
          {
            id: "Capsule L",
            meshUrl: "/meshes/capsule_L.glb",
            vortices: [{ vortexKind: "door capsule left", position: [1.3, -1.25, 0], direction: [1, 0, 0], radius: 0.36 }],
          },
          {
            id: "Capsule Z",
            meshUrl: "/meshes/capsule_z.glb",
            vortices: [{ vortexKind: "door capsule left", position: [1.3, -1.25, 0], direction: [1, 0, 0], radius: 0.36 }],
          },
          {
            id: "Capsule P",
            meshUrl: "/meshes/capsule_p.glb",
            vortices: [{ vortexKind: "door capsule left", position: [-0.45, -2.1, 0], direction: [0, -1, 0], radius: 0.36 }],
          },
          {
            id: "Capsule Slash",
            meshUrl: "/meshes/capsule_slash.glb",
            vortices: [{ vortexKind: "door capsule left", position: [-0.45, -2.1, 0], direction: [0, -1, 0], radius: 0.36 }],
          },
          {
            id: "Bridge",
            meshUrl: "/meshes/bridge.glb",
            vortices: [{ vortexKind: "platform left", position: [0, -1.3, 0], direction: [-1, 0, 0], radius: 0.36 }],
          },
        ],
        vortices: [
          { id: "door capsule left", defaultCableKind: "cable.link" },
          { id: "door tambour left", defaultCableKind: "cable.link" },
          { id: "platform left", defaultCableKind: "cable.link" },
        ],
        cables: [{ id: "cable.link", defaultAttractionKind: "puzzle3d.attraction.link" }],
      };
      const doorCompat: readonly KindCompatEntry[] = [
        { bidirectional: true, specificity: "vortex", source: "door capsule left", target: "door tambour left" },
        { bidirectional: true, specificity: "vortex", source: "platform left", target: "door tambour left" },
      ];
      const target: AttractionVortexContext = { objectId: "host", objectKind: "Tambour", vortexKind: "door tambour left" };
      publishPuzzle3dBrushCandidateAccept(puzzle3dPlayBrushCandidateAccept);
      const list = brushCompatibleCandidates(target, doorCatalogs, doorCompat);
      const ids = list.map((entry) => entry.objectKindId);
      expect(ids).toContain("Capsule L");
      expect(ids).toContain("Capsule Z");
      expect(ids).not.toContain("Capsule P");
      expect(ids).not.toContain("Capsule Slash");
      expect(ids).not.toContain("Bridge");
    });

    it("relabels Capsule Z door port to door capsule left", () => {
      const catalogs: KindCatalogBundle = {
        objects: [
          {
            id: "Capsule Z",
            meshUrl: "/meshes/capsule_z.glb",
            vortices: [{ vortexKind: "door capsule right", position: [1.3, -1.25, 0], direction: [1, 0, 0], radius: 0.36 }],
          },
        ],
        vortices: [{ id: "door capsule left" }, { id: "door capsule right" }, { id: "door tambour left" }],
      };
      const enriched = parseKindCatalogs({ kindCatalogs: catalogs })!;
      expect(enriched.objects.find((k) => k.id === "Capsule Z")?.vortices?.map((v) => v.vortexKind)).toEqual(["door capsule left"]);
    });

    it("builds vortex templates from kit connectors", () => {
      const vortexKinds: VortexKind[] = [{ id: "core rectangular bottom", name: "core rectangular bottom", color: "#000" }];
      const handleRows = [{ id: "kit.handle.core-rect-bottom", name: "core rectangular bottom" }];
      const label = (hk: string) => puzzle3dVortexKindLabelFromHandleKind(hk, vortexKinds, handleRows);
      const vortices = puzzle3dPlayObjectKindVorticesFromKitConnectors(
        [
          { point: { x: -7.5, y: -7.7, z: 7.5 }, direction: { x: 0, y: 0, z: 1 }, port: { handleKind: handleRows[0]!.id } },
          { point: { x: -18.6, y: -7.7, z: 7.5 }, direction: { x: 0, y: 0, z: 1 }, port: { handleKind: handleRows[0]!.id } },
        ],
        label,
      );
      expect(vortices).toHaveLength(2);
      expect(vortices[0]?.vortexKind).toBe("core rectangular bottom");
    });
    });

    it("stores nakagin vortex positions in type-local CAD space", () => {
      const f = parseFixtureV1(nakaginPuzzle3dFixtureJson as unknown);
      const o = f?.objects.find((obj) => obj.id === "01890804-66f2-4544-98f0-b6f0c0615492");
      const v = o?.vortices.find((vx) => vx.id.endsWith(":link"));
      expect(v?.position[0]).toBeCloseTo(-1.3, 5);
      expect(v?.position[1]).toBeCloseTo(-1.25, 5);
      expect(v?.position[2]).toBeCloseTo(0, 5);
    });

    it("applies orbit camera views per shell instance", () => {
      const bus = new CommandBus();
      const ctrl = new Puzzle3dPlayShellController(bus, () => {});
      const sharedBefore = ctrl.cameraForInstance();
      ctrl.run(ORBIT_CAMERA_VIEW_COMMAND, { view: "top", instanceId: "win-a" });
      ctrl.run(ORBIT_CAMERA_VIEW_COMMAND, { view: "front", instanceId: "win-b" });
      const top = ctrl.cameraForInstance("win-a");
      const front = ctrl.cameraForInstance("win-b");
      expect(top.position[2]).toBeGreaterThan(top.target[2]);
      expect(front.position[1]).toBeLessThan(front.target[1]);
      expect(ctrl.cameraForInstance()).toEqual(sharedBefore);
      expect(ctrl.getSnapshot().cameraSeedEpoch).toBeGreaterThan(0);
    });

    it("windowMeasures groups brush overlap budget and kind distribution", () => {
      const bus = new CommandBus();
      const ctrl = new Puzzle3dPlayShellController(bus, () => {});
      const measures = ctrl.mainMode.windowKinds[0]?.measures ?? [];
      const brush = measures.find((row) => row.kind === "group" && row.id === `${PUZZLE_3D_PLAY_WINDOW_ID}-brush`);
      expect(brush?.kind).toBe("group");
      if (brush?.kind !== "group") {
        return;
      }
      expect(brush.children.some((row) => row.kind === "slider" && row.id === `${PUZZLE_3D_PLAY_WINDOW_ID}-brush-overlap-budget`)).toBe(true);
      const distribution = brush.children.find((row) => row.kind === "group" && row.label === "Distribution");
      expect(distribution?.kind).toBe("group");
      if (distribution?.kind !== "group") {
        return;
      }
      expect(distribution.defaultOpen).toBe(false);
      expect(distribution.children.some((row) => row.kind === "group" && row.label === "Objects")).toBe(true);
      expect(distribution.children.some((row) => row.kind === "group" && row.label === "Vortices")).toBe(true);
    });

    it("patchFixture bumps revision only when structure changes", () => {
      const bus = new CommandBus();
      const wb = new Platform();
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

    it("patchPuzzle3dObjects objectKind notifies snapshot listeners and updates meshUrl", () => {
      const bus = new CommandBus();
      const wb = new Platform();
      const ctrl = new Puzzle3dPlayShellController(bus, () => wb.notify());
      const fixture = parseFixtureV1({
        schema: "puzzle.3d.fixture/v1",
        camera: { position: [0, 0, 0], target: [0, 0, 1], zoom: 1 },
        meta: {
          kindCatalogs: {
            objects: [
              { id: "kind-a", meshUrl: "/meshes/a.glb" },
              { id: "kind-b", meshUrl: "/meshes/b.glb" },
            ],
          },
        },
        attractions: [],
        objects: [{ id: "obj", objectKind: "kind-a", meshUrl: "/meshes/a.glb", origin: [0, 0, 0], vortices: [] }],
      });
      expect(fixture).not.toBeNull();
      ctrl.patchFixture(() => fixture!);
      let snapshotCount = 0;
      const unsubscribe = ctrl.subscribeSnapshot(() => {
        snapshotCount += 1;
      });
      ctrl.run("setSelection", { selection: { objectIds: ["obj"], vortexIds: [], attractionIds: [] } });
      const snapshotsBeforeKind = snapshotCount;
      ctrl.run("patchPuzzle3dObjects", { objectIds: ["obj"], field: "objectKind", value: "kind-b" });
      expect(snapshotCount).toBeGreaterThan(snapshotsBeforeKind);
      const updated = ctrl.getFixture()?.objects.find((object) => object.id === "obj");
      expect(updated?.objectKind).toBe("kind-b");
      expect(updated?.meshUrl).toBe("/meshes/b.glb");
      unsubscribe();
    });

    it("selection commands refresh the viewport snapshot without shell generation bump", async () => {
      const trackingBus = new CommandBus();
      let shellNotifyCount = 0;
      const trackingCtrl = new Puzzle3dPlayShellController(trackingBus, () => {
        shellNotifyCount += 1;
      });
      let snapshotCount = 0;
      const unsubscribe = trackingCtrl.subscribeSnapshot(() => {
        snapshotCount += 1;
      });
      const flushDeferredShell = () => new Promise<void>((resolve) => queueMicrotask(resolve));
      const fixture = trackingCtrl.getFixture();
      expect(fixture).not.toBeNull();
      const objectId = fixture!.objects[0]!.id;
      const vortexFullId = puzzle3dVortexFullId(objectId, fixture!.objects[0]!.vortices[0]!.id);
      trackingCtrl.run("noteSelection", { objectIds: [objectId], vortexIds: [], attractionIds: [] });
      expect(snapshotCount).toBe(1);
      expect(shellNotifyCount).toBe(0);
      await flushDeferredShell();
      expect(shellNotifyCount).toBe(0);
      trackingCtrl.run("noteSelection", { objectIds: [objectId], vortexIds: [], attractionIds: [] });
      expect(snapshotCount).toBe(1);
      expect(shellNotifyCount).toBe(0);
      trackingCtrl.run("setSelection", { selection: { objectIds: [], vortexIds: [vortexFullId], attractionIds: [] } });
      expect(snapshotCount).toBe(2);
      await flushDeferredShell();
      expect(shellNotifyCount).toBe(0);
      expect(trackingCtrl.getSnapshot().selection.vortexIds).toEqual([vortexFullId]);
      trackingCtrl.run("setSelectedId", { id: objectId });
      expect(snapshotCount).toBe(3);
      await flushDeferredShell();
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

    it("window engagement requires command input and tool possibles (CAD play layout)", () => {
      const bus = new CommandBus();
      const wb = new Platform();
      const ctrl = new Puzzle3dPlayShellController(bus, () => wb.notify());
      const engagement = ctrl.mainMode.windowKinds[0]?.engagement;
      expect(engagement?.input?.id).toBe("engagement-input");
      expect(engagement?.possibleEngagements?.map((row) => row.id)).toEqual(["puzzle3d.tool.brush", "puzzle3d.tool.select"]);
      expect(engagement?.options).toBeUndefined();
      expect(ctrl.mainMode.tools.actions?.some((tool) => tool.id === "puzzle3d.tool.brush")).toBe(false);
    });

    it("setWindowEngagement enforces input and skips equal digest", () => {
      const bus = new CommandBus();
      const wb = new Platform();
      const ctrl = new Puzzle3dPlayShellController(bus, () => wb.notify());
      expect(() => ctrl.setWindowEngagement({ options: [{ id: "x", label: "X" }] })).toThrow(/engagement\.input/);
      let shellNotifyCount = 0;
      const trackingCtrl = new Puzzle3dPlayShellController(bus, () => {
        shellNotifyCount += 1;
      });
      const live: WindowEngagement = {
        input: {
          id: "engagement-input",
          value: "brush",
          onChange: puzzle3dPlayCmd("engagementInput"),
          onSubmit: puzzle3dPlayCmd("engagementSubmit"),
        },
        possibleEngagements: [{ id: "puzzle3d.tool.brush", label: "Brush", command: puzzle3dPlayCmd("engagementPossibleSelect", { possibleId: "puzzle3d.tool.brush" }) }],
      };
      trackingCtrl.setWindowEngagement(live);
      const afterFirst = shellNotifyCount;
      trackingCtrl.setWindowEngagement(live);
      expect(shellNotifyCount).toBe(afterFirst);
      expect(trackingCtrl.mainMode.windowKinds[0]?.engagement?.input?.value).toBe("brush");
    });

    it("engagementPossibleSelect activates brush without host bridge", () => {
      const bus = new CommandBus();
      const wb = new Platform();
      const ctrl = new Puzzle3dPlayShellController(bus, () => wb.notify());
      ctrl.run("engagementPossibleSelect", { possibleId: PUZZLE_3D_ENGAGEMENT_TOOL_BRUSH_ID });
      expect(ctrl.getSnapshot().activeTool).toBe("brush");
    });

    it("engagementRepeatLast replays the last remembered engagement command", () => {
      const bus = new CommandBus();
      const wb = new Platform();
      const ctrl = new Puzzle3dPlayShellController(bus, () => wb.notify());
      ctrl.run("engagementPossibleSelect", { possibleId: PUZZLE_3D_ENGAGEMENT_TOOL_BRUSH_ID });
      expect(ctrl.getSnapshot().activeTool).toBe("brush");
      ctrl.run("setActiveTool", { tool: "select" });
      expect(ctrl.getSnapshot().activeTool).toBe("select");
      ctrl.run("engagementRepeatLast", {});
      expect(ctrl.getSnapshot().activeTool).toBe("brush");
    });

    it("engagementRepeatLast forwards brush possibles to the host bridge", () => {
      const bus = new CommandBus();
      const wb = new Platform();
      const ctrl = new Puzzle3dPlayShellController(bus, () => wb.notify());
      const hostCalls: string[] = [];
      ctrl.setHostBridge({
        runHostCommand: (command) => {
          hostCalls.push(command);
        },
      });
      ctrl.run("engagementPossibleSelect", { possibleId: "puzzle3d.brush.J.0" });
      expect(hostCalls).toEqual(["engagementPossibleSelect"]);
      ctrl.run("engagementRepeatLast", {});
      expect(hostCalls).toEqual(["engagementPossibleSelect", "engagementPossibleSelect"]);
    });

    it("engagementSubmit activates brush from normalized command text without host bridge", () => {
      const bus = new CommandBus();
      const wb = new Platform();
      const ctrl = new Puzzle3dPlayShellController(bus, () => wb.notify());
      ctrl.run("engagementSubmit", { value: "brush" });
      expect(ctrl.getSnapshot().activeTool).toBe("brush");
    });

    it("engagementOption Zoom requests camera frame for current selection", () => {
      const bus = new CommandBus();
      const wb = new Platform();
      const ctrl = new Puzzle3dPlayShellController(bus, () => wb.notify());
      const before = getPuzzle3dZoomToSelectionEpoch();
      ctrl.run("setSelection", { selection: { objectIds: ["tower-a"], vortexIds: [], attractionIds: [] } });
      ctrl.run("engagementOption", { optionId: PUZZLE_3D_ENGAGEMENT_ZOOM_ID });
      expect(getPuzzle3dZoomToSelectionEpoch()).toBe(before + 1);
      expect(getPuzzle3dZoomToSelectionTarget().objectIds).toEqual(["tower-a"]);
    });

    it("engagement bus commands route through host bridge for non-tool commands", () => {
      const bus = new CommandBus();
      const wb = new Platform();
      const ctrl = new Puzzle3dPlayShellController(bus, () => wb.notify());
      let lastCommand = "";
      ctrl.setHostBridge({
        runHostCommand: (command) => {
          lastCommand = command;
        },
      });
      ctrl.run("engagementPossibleSelect", { possibleId: "puzzle3d.brush.J.0" });
      expect(lastCommand).toBe("engagementPossibleSelect");
      ctrl.run("engagementInput", { value: "J" });
      expect(lastCommand).toBe("engagementInput");
    });

    it("window measures include selection kind toggles and marquee method", () => {
      const bus = new CommandBus();
      const wb = new Platform();
      const ctrl = new Puzzle3dPlayShellController(bus, () => wb.notify());
      const measures = flattenWindowMeasures(ctrl.mainMode.windowKinds[0]?.measures ?? []);
      const texts = measures.map((measure) => measure.text);
      expect(texts).toContain("Objects");
      expect(texts).toContain("Vortices");
      expect(texts).toContain("Attractions");
      expect(texts).toContain("Rectangle");
      expect(texts).toContain("Lasso");
    });

    it("window measures include brush overlap budget slider", () => {
      const bus = new CommandBus();
      const wb = new Platform();
      const ctrl = new Puzzle3dPlayShellController(bus, () => wb.notify());
      const measures = flattenWindowMeasures(ctrl.mainMode.windowKinds[0]?.measures ?? []);
      const brushBudget = measures.find((measure) => measure.id.endsWith("-brush-overlap-budget"));
      expect(brushBudget?.kind).toBe("slider");
      expect(brushBudget?.max).toBe(BRUSH_PLACEMENT_OVERLAP_BUDGET_MAX);
      expect(brushBudget?.value).toBeCloseTo(DEFAULT_BRUSH_PLACEMENT_OVERLAP_BUDGET, 5);
    });

    it("setBrushPlacementOverlapBudget updates snapshot", () => {
      const bus = new CommandBus();
      const wb = new Platform();
      const ctrl = new Puzzle3dPlayShellController(bus, () => wb.notify());
      ctrl.run("setBrushPlacementOverlapBudget", { value: 0.15, cad: true });
      expect(ctrl.getSnapshot().brushPlacementOverlapBudget).toBeCloseTo(0.15, 5);
      const measures = flattenWindowMeasures(ctrl.mainMode.windowKinds[0]?.measures ?? []);
      const brushBudget = measures.find((measure) => measure.id.endsWith("-brush-overlap-budget"));
      expect(brushBudget?.kind).toBe("slider");
      if (brushBudget?.kind === "slider") {
        expect(brushBudget.value).toBeCloseTo(0.15, 5);
      }
    });

    it("setSelectionMethod and toggleSelectableKind update snapshot", () => {
      const bus = new CommandBus();
      const wb = new Platform();
      const ctrl = new Puzzle3dPlayShellController(bus, () => wb.notify());
      ctrl.run("setSelectionMethod", { method: "lasso" });
      expect(ctrl.getSnapshot().selectionMethod).toBe("lasso");
      ctrl.run("toggleSelectableKind", { kind: "object" });
      expect(ctrl.getSnapshot().selectableKinds.object).toBe(false);
    });

    it("setActiveTool and addBrushObject update fixture and snapshot", () => {
      const bus = new CommandBus();
      const wb = new Platform();
      const ctrl = new Puzzle3dPlayShellController(bus, () => wb.notify());
      const revisionBefore = ctrl.getSnapshot().fixtureRevision;
      ctrl.run("setActiveTool", { tool: "brush" });
      expect(ctrl.getSnapshot().activeTool).toBe("brush");
      const fixture = ctrl.getFixture();
      expect(fixture).not.toBeNull();
      const host = fixture!.objects[0]!;
      const targetFullId = puzzle3dVortexFullId(host.id, host.vortices[0]!.id);
      ctrl.run("addBrushObject", {
        targetVortexFullId: targetFullId,
        objectKindId: "Hexagonal Cut Concrete Forest Right",
        sourceVortexIndex: 0,
        origin: [0, 0, 0],
        orientation: [0, 0, 0, 1],
        objectId: "play-brush-test",
      });
      const snap = ctrl.getSnapshot();
      expect(snap.fixture?.objects.some((object) => object.id === "play-brush-test")).toBe(true);
      expect(snap.fixture?.attractions.length).toBeGreaterThan(0);
      expect(snap.fixtureRevision).toBeGreaterThan(revisionBefore);
    });

    it("puzzle3dPlayAllSelectionFromFixture lists every row for enabled kinds", () => {
      const fixture = parseFixtureV1(nakaginPuzzle3dFixtureJson as unknown);
      expect(fixture).not.toBeNull();
      const all = puzzle3dPlayAllSelectionFromFixture(fixture!, { object: true, vortex: true, attraction: true });
      expect(all.objectIds.length).toBe(fixture!.objects.length);
      expect(all.vortexIds.length).toBe(fixture!.objects.reduce((count, object) => count + object.vortices.length, 0));
      expect(all.attractionIds).toEqual(fixture!.attractions.map((attraction) => attraction.id));
      const objectsOnly = puzzle3dPlayAllSelectionFromFixture(fixture!, { object: true, vortex: false, attraction: false });
      expect(objectsOnly.vortexIds).toEqual([]);
      expect(objectsOnly.attractionIds).toEqual([]);
    });

    it("buildPuzzle3dPlayInspectorBody exposes editable object fields for single selection", () => {
      const bus = new CommandBus();
      const wb = new Platform();
      const ctrl = new Puzzle3dPlayShellController(bus, () => wb.notify());
      wb.addApp(buildPuzzle3dPlayAppRuntime(ctrl));
      const objectId = ctrl.getFixture()!.objects[0]!.id;
      ctrl.run("setSelection", { selection: { objectIds: [objectId], vortexIds: [], attractionIds: [] } });
      const tree = buildPuzzle3dPlayInspectorBody({
        runtime: wb,
        windowKindId: PUZZLE_3D_PLAY_WINDOW_ID,
        bodyKey: PUZZLE_3D_PLAY_INSPECTOR_BODY_KEY,
        activeModeId: "main",
        generation: wb.generation,
      });
      const objectSection = tree.sections.find((section) => section.label?.startsWith("Objects"));
      expect(objectSection).toBeDefined();
      const idField = objectSection!.items.find((item) => item.id === "puzzle-3d-play-inspector.object.id");
      const labelField = objectSection!.items.find((item) => item.id === "puzzle-3d-play-inspector.object.label");
      expect(idField?.control?.type).toBe("input");
      expect((idField?.control as { value?: string } | undefined)?.value).toBe(objectId);
      expect(labelField?.control?.type).toBe("input");
      expect((labelField?.control as { value?: string } | undefined)?.value).toBeTruthy();
    });

    it("buildPuzzle3dPlayInspectorBody aggregates multi vortex and attraction selection", () => {
      const bus = new CommandBus();
      const wb = new Platform();
      const ctrl = new Puzzle3dPlayShellController(bus, () => wb.notify());
      wb.addApp(buildPuzzle3dPlayAppRuntime(ctrl));
      const fixture = parseFixtureV1({
        schema: "puzzle.3d.fixture/v1",
        camera: { position: [0, 0, 0], target: [0, 0, 1], zoom: 1 },
        attractions: [
          { id: "t1", attracting: "a:v1", attracted: "b:v2" },
          { id: "t2", attracting: "b:v2", attracted: "c:v3" },
        ],
        objects: [
          { id: "a", meshUrl: "/m.glb", origin: [0, 0, 0], vortices: [{ id: "v1", position: [0, 0, 0] }] },
          { id: "b", meshUrl: "/m.glb", origin: [1, 0, 0], vortices: [{ id: "v2", position: [0, 0, 0] }] },
        ],
      });
      expect(fixture).not.toBeNull();
      ctrl.patchFixture(() => fixture!);
      ctrl.run("setSelection", {
        selection: {
          objectIds: [],
          vortexIds: ["a:v1", "b:v2"],
          attractionIds: ["t1", "t2"],
        },
      });
      const tree = buildPuzzle3dPlayInspectorBody({
        runtime: wb,
        windowKindId: PUZZLE_3D_PLAY_WINDOW_ID,
        bodyKey: PUZZLE_3D_PLAY_INSPECTOR_BODY_KEY,
        activeModeId: "main",
        generation: wb.generation,
      });
      const labels = (tree.sections ?? []).map((section) => section.label);
      expect(labels).toContain("Vortices (2)");
      expect(labels).toContain("Attractions (2)");
      expect(labels.filter((label) => label?.startsWith("a:v1")).length).toBe(0);
    });

    it("puzzle3dPlayInspectorVortexRows resolves rows via object map", () => {
      const fixture = parseFixtureV1({
        schema: "puzzle.3d.fixture/v1",
        camera: { position: [0, 0, 0], target: [0, 0, 1], zoom: 1 },
        attractions: [],
        objects: [
          { id: "a", meshUrl: "/m.glb", origin: [0, 0, 0], vortices: [{ id: "v1", label: "A", position: [0, 0, 0] }] },
          { id: "b", meshUrl: "/m.glb", origin: [1, 0, 0], vortices: [{ id: "v2", label: "B", position: [0, 0, 0] }] },
        ],
      });
      expect(fixture).not.toBeNull();
      const rows = puzzle3dPlayInspectorVortexRows(fixture!, ["a:v1", "b:v2"]);
      expect(rows).toHaveLength(2);
      expect(rows[0]?.vortex.label).toBe("A");
    });

    it("selectAllSelection selects every selectable fixture row", () => {
      const bus = new CommandBus();
      const wb = new Platform();
      const ctrl = new Puzzle3dPlayShellController(bus, () => wb.notify());
      const fixture = ctrl.getFixture();
      expect(fixture).not.toBeNull();
      ctrl.run("setSelection", { selection: PUZZLE_3D_PLAY_EMPTY_SELECTION });
      const started = performance.now();
      ctrl.run("selectAllSelection");
      expect(performance.now() - started).toBeLessThan(100);
      const snap = ctrl.getSnapshot();
      expect(snap.selection.objectIds.length).toBe(fixture!.objects.length);
      expect(snap.selection.vortexIds.length).toBe(fixture!.objects.reduce((count, object) => count + object.vortices.length, 0));
      ctrl.run("toggleSelectableKind", { kind: "vortex" });
      ctrl.run("selectAllSelection");
      expect(ctrl.getSnapshot().selection.vortexIds).toEqual([]);
    });

    it("deleteSelection removes selected fixture rows and clears selection", () => {
      const bus = new CommandBus();
      const wb = new Platform();
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

    it("deletePuzzle3dObjectFromFixture removes child vortices and stale attractions", () => {
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
      const next = deletePuzzle3dObjectFromFixture(base!, "b");
      expect(next.objects.map((object) => object.id)).toEqual(["a", "c"]);
      expect(next.attractions).toEqual([]);
    });

    it("puzzle3dPlaySelectionLabel resolves object and vortex fixture labels", () => {
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
      expect(puzzle3dPlaySelectionLabel(fixture, { objectIds: ["a"], vortexIds: [], attractionIds: [] })).toBe("Alpha");
      expect(puzzle3dPlaySelectionLabel(fixture, { objectIds: [], vortexIds: ["a:v1"], attractionIds: [] })).toBe("Handle A");
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
      const tree = buildPuzzle3dPlayHierarchyTree(fixture!, PUZZLE_3D_PLAY_EMPTY_SELECTION);
      const viewportRoot = tree.sections[0]?.items?.[0];
      expect(viewportRoot?.label).toBe("Puzzle 3D");
      const objectsGroup = viewportRoot?.items?.find((row) => row.label === "Objects");
      expect(objectsGroup?.items?.length).toBe(2);
      const firstObject = objectsGroup?.items?.[0];
      expect(firstObject?.label).toBe("Alpha");
      expect(firstObject?.items?.[0]?.label).toBe("Vortices");
      expect(firstObject?.items?.[0]?.items?.[0]?.label).toBe("Handle A");
      expect(firstObject?.items?.[0]?.items?.[0]?.id).toBe("puzzle-3d-play-hierarchy.vortex.a:v1");
      const attractionsGroup = viewportRoot?.items?.find((row) => row.label === "Attractions");
      expect(attractionsGroup?.items?.[0]?.id).toBe("puzzle-3d-play-hierarchy.attraction.t1");
    });

    it("buildPuzzle3dPlayHierarchySections omits per-row selected flags", () => {
      const fixture = parseFixtureV1({
        schema: "puzzle.3d.fixture/v1",
        camera: { position: [0, 0, 0], target: [0, 0, 1], zoom: 1 },
        attractions: [],
        objects: [{ id: "a", meshUrl: "/m.glb", origin: [0, 0, 0], vortices: [{ id: "v1", position: [0, 0, 0] }] }],
      });
      expect(fixture).not.toBeNull();
      const visit = (items: readonly UiTreeItemNode[]): void => {
        for (const item of items) {
          expect(item.selected).toBeUndefined();
          if (item.items?.length) {
            visit(item.items);
          }
        }
      };
      for (const section of buildPuzzle3dPlayHierarchySections(fixture!)) {
        visit(section.items);
      }
    });

    it("puzzle3dPlayHierarchySelectedIds maps selection to hierarchy row ids", () => {
      expect(
        puzzle3dPlayHierarchySelectedIds({
          objectIds: ["a"],
          vortexIds: ["a:v1"],
          attractionIds: ["t1"],
        }),
      ).toEqual(["puzzle-3d-play-hierarchy.object.a", "puzzle-3d-play-hierarchy.vortex.a:v1", "puzzle-3d-play-hierarchy.attraction.t1"]);
    });

    it("puzzle3dPlayHierarchyTreeHighlightedIds expands transitive vortex kind hover", () => {
      const fixture = parseFixtureV1({
        schema: "puzzle.3d.fixture/v1",
        camera: { position: [0, 0, 0], target: [0, 0, 1], zoom: 1 },
        attractions: [],
        objects: [
          {
            id: "a",
            meshUrl: "/m.glb",
            origin: [0, 0, 0],
            vortices: [
              { id: "v1", vortexKind: "b-l", position: [0, 0, 0] },
              { id: "v2", vortexKind: "c-t", position: [1, 0, 0] },
            ],
          },
          {
            id: "b",
            meshUrl: "/m.glb",
            origin: [2, 0, 0],
            vortices: [{ id: "v3", vortexKind: "b-l", position: [0, 0, 0] }],
          },
        ],
      });
      expect(fixture).not.toBeNull();
      expect(puzzle3dPlayHierarchyTreeHighlightedIds(fixture!, { domain: "vortex", kindId: "b-l" })).toEqual([
        "puzzle-3d-play-hierarchy.vortex.a:v1",
        "puzzle-3d-play-hierarchy.vortex.b:v3",
      ]);
    });

    it("setHoverFocus stores direct kind row hover", () => {
      const bus = new CommandBus();
      const wb = new Platform();
      const ctrl = new Puzzle3dPlayShellController(bus, () => wb.notify());
      ctrl.setHoverFocus({ hoverTarget: null, kindHover: { domain: "vortex", kindId: "b-l" } });
      expect(ctrl.getSnapshot().hoverFocus).toEqual({ hoverTarget: null, kindHover: { domain: "vortex", kindId: "b-l" } });
    });

    it("noteSelection does not bump platform generation", () => {
      const bus = new CommandBus();
      const wb = new Platform();
      const ctrl = new Puzzle3dPlayShellController(bus, () => wb.notify());
      wb.addApp(buildPuzzle3dPlayAppRuntime(ctrl));
      const fixture = ctrl.getFixture();
      expect(fixture).not.toBeNull();
      const objectId = fixture!.objects[0]!.id;
      const generationBefore = wb.generation;
      ctrl.run("noteSelection", { objectIds: [objectId], vortexIds: [], attractionIds: [] });
      expect(wb.generation).toBe(generationBefore);
      expect(ctrl.getSnapshot().selection.objectIds).toEqual([objectId]);
    });

    it("getHierarchyPanelTree keeps stable sections across selection-only changes", () => {
      const bus = new CommandBus();
      const wb = new Platform();
      const ctrl = new Puzzle3dPlayShellController(bus, () => wb.notify());
      const fixture = ctrl.getFixture();
      expect(fixture).not.toBeNull();
      const objectId = fixture!.objects[0]!.id;
      ctrl.run("setSelection", { selection: { objectIds: [objectId], vortexIds: [], attractionIds: [] } });
      const treeA = ctrl.getHierarchyPanelTree(ctrl.getSnapshot().selection);
      ctrl.run("setSelection", { selection: PUZZLE_3D_PLAY_EMPTY_SELECTION });
      const treeB = ctrl.getHierarchyPanelTree(ctrl.getSnapshot().selection);
      expect(treeA.sections).toBe(treeB.sections);
      expect(treeA.selectedIds).toEqual([`puzzle-3d-play-hierarchy.object.${objectId}`]);
      expect(treeB.selectedIds).toEqual([]);
    });

    it("buildPuzzle3dPlayKindsSections lists object, vortex, cable, and attraction kind categories", () => {
      const catalogs = parseKindCatalogs({
        kindCatalogs: {
          objects: [{ id: "capsule", label: "Capsule", name: "Capsule" }],
          vortices: [{ id: "core circular top", label: "Core circular top", name: "Core circular top" }],
          cables: [{ id: "cable.link", label: "Link", name: "Link" }],
          attractions: [{ id: "puzzle3d.attraction.link", label: "Link", name: "Link" }],
        },
      });
      const tree = buildPuzzle3dPlayKindsTree(catalogs);
      expect(tree.sections.map((section) => section.label)).toEqual(["Objects", "Vortices", "Cables", "Attractions"]);
      expect(tree.sections[0]?.items?.[0]?.label).toBe("Capsule");
    });

    it("buildPuzzle3dPlayKindsTree marks object catalog rows draggable with fixture drag payload", async () => {
      const { FIXTURE_DRAG_V1_MIME, decodePuzzle3dFixtureFromDragV1 } = await import("../react/index.tsx");
      const catalogs = parseKindCatalogs({
        kindCatalogs: {
          objects: [{ id: "J", label: "J", name: "J", meshUrl: "/meshes/capsule_J.glb", vortices: [] }],
        },
      });
      const tree = buildPuzzle3dPlayKindsTree(catalogs);
      const row = tree.sections[0]?.items?.[0];
      expect(row?.draggable).toBe(true);
      const encoded = row?.dragData?.[FIXTURE_DRAG_V1_MIME];
      expect(encoded).toBeTruthy();
      const dragFixture = decodePuzzle3dFixtureFromDragV1(encoded!);
      expect(dragFixture?.objects[0]?.objectKind).toBe("J");
    });

    it("buildPuzzle3dPlayKindsTree assigns unique item ids when catalog ids repeat", () => {
      const catalogs = parseKindCatalogs({
        kindCatalogs: {
          objects: [
            { id: "dup", label: "Alpha", name: "Alpha" },
            { id: "dup", label: "Beta", name: "Beta" },
          ],
        },
      });
      const tree = buildPuzzle3dPlayKindsTree(catalogs);
      const ids = tree.sections[0]?.items?.map((item) => item.id) ?? [];
      expect(ids).toHaveLength(2);
      expect(new Set(ids).size).toBe(2);
    });

    it("nakagin fixture seeds default object and vortex suggestion ratios", () => {
      const bus = new CommandBus();
      const ctrl = new Puzzle3dPlayShellController(bus, () => {});
      ctrl.run("setActiveFixture", { fixtureId: PUZZLE_3D_PLAY_FIXTURE_NAKAGIN_ID });
      const { objectWeights, vortexWeights } = puzzle3dBrushKindWeightsRef.current;
      expect(objectWeights.Tambour / objectWeights.Base).toBeCloseTo(15, 4);
      expect(objectWeights.Tambour / objectWeights.Capital).toBeCloseTo(10, 4);
      expect(objectWeights["Capsule J"] / objectWeights.Tambour).toBeCloseTo(8, 4);
      expect(vortexWeights["tambour circular top"] / vortexWeights["core rectangular bottom"]).toBeCloseTo(15, 4);
      expect(vortexWeights["door capsule right"] / vortexWeights["tambour circular top"]).toBeCloseTo(8, 4);
    });

    it("buildPuzzle3dPlayKindsTree omits draggable on object kinds without a loadable mesh", () => {
      const catalogs = parseKindCatalogs({
        kindCatalogs: {
          objects: [
            { id: "Balcony", label: "Balcony", name: "Balcony" },
            { id: "Base", label: "Base", name: "Base", meshUrl: "/meshes/base.glb" },
          ],
        },
      });
      const tree = buildPuzzle3dPlayKindsTree(catalogs);
      const balcony = tree.sections[0]?.items?.find((item) => item.label === "Balcony");
      const base = tree.sections[0]?.items?.find((item) => item.label === "Base");
      expect(balcony?.draggable).toBeUndefined();
      expect(base?.draggable).toBe(true);
    });

    it("buildPuzzle3dPlayKindsTree nests object-kind vortex templates as child rows", () => {
      const catalogs = parseKindCatalogs({
        kindCatalogs: {
          objects: [
            {
              id: "base",
              label: "Base",
              name: "Base",
              meshUrl: "/meshes/base.glb",
              vortices: [
                { vortexKind: "core rectangular bottom", position: [-7.5, -7.7, 7.5] },
                { vortexKind: "core rectangular bottom", position: [-18.6, -7.7, 7.5] },
              ],
            },
          ],
          vortices: [{ id: "core rectangular bottom", label: "Core rectangular bottom", name: "Core rectangular bottom" }],
        },
      });
      const tree = buildPuzzle3dPlayKindsTree(catalogs);
      expect(tree.sections.find((section) => section.id === "puzzle-3d-play-kinds.vortices")?.defaultOpen).toBe(false);
      const base = tree.sections[0]?.items?.find((item) => item.label === "Base");
      expect(base?.defaultOpen).toBe(false);
      expect(base?.items).toHaveLength(2);
      expect(base?.items?.[0]?.label).toBe("Core rectangular bottom");
      expect(base?.items?.[0]?.description).toBe("-7.5, -7.7, 7.5");
      expect(base?.items?.[1]?.description).toBe("-18.6, -7.7, 7.5");
      expect(base?.items?.[0]?.draggable).toBeUndefined();
    });

    it("puzzle3dPlayKindCatalogSelectItems dedupes duplicate catalog ids for inspector selects", () => {
      const items = puzzle3dPlayKindCatalogSelectItems([
        { id: "Single Storey", label: "Single Storey" },
        { id: "Single Storey", label: "Single Storey (duplicate)" },
        { id: "/", label: "/" },
        { id: "/", label: "/" },
      ]);
      expect(items.map((item) => item.value)).toEqual(["/", "Single Storey"]);
      expect(items.find((item) => item.value === "Single Storey")?.label).toBe("Single Storey (duplicate)");
    });

    it("declarative window body is a lone puzzle3d viewport surface", () => {
      const bus = new CommandBus();
      const wb = new Platform();
      const ctrl = new Puzzle3dPlayShellController(bus, () => wb.notify());
      wb.addApp(buildPuzzle3dPlayAppRuntime(ctrl));
      const tree = buildPuzzle3dPlayDeclarativeBody({
        runtime: wb,
        windowKindId: PUZZLE_3D_PLAY_WINDOW_ID,
        bodyKey: PUZZLE_3D_PLAY_BODY_KEY,
        activeModeId: "main",
        generation: wb.generation,
      });
      expect(tree).toEqual(buildPuzzle3dWindowBody(PUZZLE_3D_PLAY_VIEWPORT_SURFACE_ID, PUZZLE_3D_PLAY_CONTROLLER_ID));
    });
  });
}
//#endregion 🧪Tests

//#region 🔖Boot
if (
  typeof document !== "undefined" &&
  document.getElementById("root") != null &&
  !import.meta.vitest &&
  import.meta.env.PUZZLE_PLAY_ENTRY === "3d"
) {
  bootstrapElementsSurfaceChromeDocument("system");
  void (async () => {
    await import("./globals.css");
    const { bootPuzzle3dPlay } = await import("@framework/playground/renderer/react/puzzle/3d");
    bootPuzzle3dPlay(new Playground3d());
  })();
}
//#endregion 🔖Boot
