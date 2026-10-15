/** 🧾️ REPL suggestion model, selection pruning and merging, history presentation, host value and callback types, default chrome state and the engagement and search builders.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | renderer}, which re-exports every public name below so import specifiers stay unchanged.
 */

// #region 🔌️Adapters
import {
  Checkbox,
  humanizeEngagementStepId,
  isUiTypingTarget,
  normalizeEngagementActionText,
  Pane,
  reactHostPort,
  uiDataLabel,
  usePaneSlot,
  useLabel,
  windowMeasuresBodyClass,
  WINDOW_SEARCH_USER,
  type Anchor,
  type EngagementControl,
  type EngagementSpec,
  type IconName,
  type SearchSpec,
  type CSSProperties,
  type ReactNode
} from "@semio-tech/ui-react";
// #endregion 🔌️Adapters
import {
  WorldProjectionKindSwitch,
  worldProjectionSpecIconId,
  type WorldProjectionSpec,
  type WorldReferenceProps,
  type WorldReferenceRelocatePayload
} from "@semio-tech/infinite-world-r3f";
import { type Vec3 } from "../../../../../../../../../../⚙️engine/📔️registry/🟦️.ts";
import {
  defaultModelDefinitionId,
  Model,
  type InteractionEvent,
  type InteractionSpec,
  type ResolvedInteractionEngagementControl,
  type TransformationSpec,
  type SelectionTarget,
  type SpatialInteraction,
  type ModelEntityKind
} from "../../../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️.ts";
import { type ModelDiff } from "../../../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/🗺️spatial/🟦️.ts";
import { listKeyedInteractionTransitions, parseNumericCommandLine, type InteractionKeybindRow, type CadGumballConfig } from "../../🎬️actions/🟦️.ts";
import {
  DocumentHistory,
  isInteractionSessionActive,
  resolveSpatialInteractionKeyForModelDefinition,
  type InteractionRuntime,
  type InteractionSnapshot,
  type ModelDocument
} from "../../🗿️artifact/🟦️.ts";
import {
  type ArchivedBoxLayout,
  buildGeometryTypologyIndex,
  defaultSpatialPrimitiveToggles,
  defaultSpatialTypologyTogglesForModelDefinition,
  modelDefinitionTypologyIds,
  type SpatialDragSelectionState,
  type SpatialPickGeometry,
  spatialPickKindsForActiveView,
  type SpatialPickTarget,
  type SpatialPickTargetKind,
  type SpatialPrimitiveToggles,
  type SpatialSelectionMethod,
  type SpatialSelectionRequest,
  spatialToggleCheckboxState,
  type SpatialToggleGroupState,
  type SpatialTypologyToggles
} from "../🧲️geometry-targets/🟦️.tsx";
import { type SpatialDisplayItemRenderer } from "../🖼️display/🟦️.tsx";
import { type InteractionSpatialViewSlots, type InteractionSpatialViewTheme } from "../🔖️interaction/🟦️.tsx";
import { mergeSelectionTargets, selectionTargetPickKind, spatialSelectionModeFromModifiers } from "../🎯️pick-layer/🟦️.tsx";
import { type InteractionCanvasProps, type InteractionSpatialViewProps, type SpatialAutoFitBehavior, useInteractionSnapshot } from "../🪩️canvas/🟦️.tsx";

// #region 🪩️Repl
/** ☑️ Master checkbox for a chrome toggle group (supports indeterminate partial state). */
export function SpatialChromeMasterToggle({ state, onEnabledChange, ariaLabel }: { readonly state: SpatialToggleGroupState; readonly onEnabledChange: (enabled: boolean) => void; readonly ariaLabel: string }): ReactNode {
  return <Checkbox aria-label={ariaLabel} checked={spatialToggleCheckboxState(state)} onChange={(event) => onEnabledChange(event.target.checked)} />;
}

type ReplSuggestKind = "interaction" | "transition" | "action" | "selection";

export interface ReplSuggestion {
  readonly kind: ReplSuggestKind;
  readonly key: string;
  readonly label: string;
  readonly detail?: string;
  readonly transition?: InteractionKeybindRow;
  readonly interactionId?: string;
  readonly onRun: () => void;
}

export function resolveScopedSpatialInteractionKey(token: string, modelDefinitionId: string): SpatialInteraction | null {
  return resolveSpatialInteractionKeyForModelDefinition(modelDefinitionId, token);
}

export function replActionTextWithoutSpaces(text: string): string {
  return text.replace(/\s+/g, "");
}

/** ⌨️ Normalizes REPL action text: engagement uses PascalCase; aside REPL strips whitespace only. */
export function replNormalizeActionText(text: string, engagementMode?: boolean): string {
  return engagementMode ? normalizeEngagementActionText(text) : replActionTextWithoutSpaces(text);
}

function replFirstWireId(model: Model): string | null {
  const ks = Object.keys(model.wires);
  return ks.length ? model.wires[ks[0]!]!.id : null;
}

function replFirstFaceId(model: Model): string | null {
  const ks = Object.keys(model.faces);
  return ks.length ? model.faces[ks[0]!]!.id : null;
}

export function replBuildDispatchEvent(row: InteractionKeybindRow, opts: { readonly interactionId: string; readonly model: Model }): InteractionEvent | null {
  const { interactionId, model } = opts;
  if (row.eventKind === "set.height" || row.eventKind === "set.distance" || row.eventKind === "set.footprint") return null;
  if (row.eventKind === "selection.changed") {
    if (interactionId === "feature.extrudeWire") {
      const wid = replFirstWireId(model);
      if (!wid) return null;
      return { kind: "selection.changed", targets: [{ kind: "wire", id: wid, editable: true }], modifiers: {} };
    }
    if (interactionId === "feature.offsetSurface") {
      const fid = replFirstFaceId(model);
      if (!fid) return null;
      return { kind: "selection.changed", targets: [{ kind: "face", id: fid, editable: true }], modifiers: {} };
    }
    return null;
  }
  return { kind: row.eventKind, modifiers: {} };
}

/** 📏️ Parses REPL `cmdLine` as a live direct-distance value (`null` = empty, `undefined` = not numeric). */
export function replLengthEntryLiveValue(cmdLine: string): number | null | undefined {
  return parseNumericCommandLine(cmdLine);
}

export function replTryParseValueInteraction(line: string, spec: InteractionSpec, state: string): InteractionEvent | null {
  const t = line.trim();
  const m = t.match(/^(\S+)\s+(.+)$/);
  if (!m) return null;
  const head = m[1]!.toLowerCase();
  const tail = m[2]!.trim();
  const rows = listKeyedInteractionTransitions(spec, state);
  for (const row of rows) {
    if (row.eventKind === "set.height") {
      if (head !== row.key.toLowerCase() && head !== "height") continue;
      const v = Number(tail);
      if (!Number.isFinite(v) || v <= 0) return null;
      return { kind: "set.height", value: v, modifiers: {} };
    }
    if (row.eventKind === "set.distance") {
      if (head !== row.key.toLowerCase() && head !== "dist" && head !== "distance") continue;
      const v = Number(tail);
      if (!Number.isFinite(v)) return null;
      return { kind: "set.distance", value: v, modifiers: {} };
    }
    if (row.eventKind === "set.footprint") {
      if (head !== row.key.toLowerCase() && head !== "footprint" && head !== "lw") continue;
      const parts = tail.split(/\s+/);
      const L = Number(parts[0]);
      const W = Number(parts[1]);
      if (!Number.isFinite(L) || !Number.isFinite(W)) return null;
      return { kind: "set.footprint", value: { length: L, width: W }, modifiers: {} };
    }
    if (row.eventKind.startsWith("set.")) {
      const alias = row.eventKind.slice("set.".length).toLowerCase();
      if (head !== row.key.toLowerCase() && head !== alias && head !== "number" && head !== "n") continue;
      const v = Number(tail);
      if (!Number.isFinite(v)) return null;
      return { kind: row.eventKind, value: v, modifiers: {} };
    }
  }
  return null;
}

function replSuggestionHaystack(s: ReplSuggestion): string {
  return `${s.key} ${s.label} ${s.detail ?? ""}`.toLowerCase();
}

function replRankScore(query: string, s: ReplSuggestion): number {
  const ql = query.trim().toLowerCase();
  if (!ql) return -1;
  const key = s.key.toLowerCase();
  const label = s.label.toLowerCase();
  const detail = (s.detail ?? "").toLowerCase();
  if (key.startsWith(ql)) return 4000 - key.length;
  if (label.startsWith(ql)) return 3000 - label.length;
  if (detail && detail.startsWith(ql)) return 2000 - detail.length;
  if (replSuggestionHaystack(s).includes(ql)) return 1000;
  return -1;
}

export function replFilterSuggestions(query: string, all: readonly ReplSuggestion[]): ReplSuggestion[] {
  const q = query.trim();
  if (!q) return [];
  return all
    .map((s) => ({ s, score: replRankScore(q, s) }))
    .filter((row) => row.score >= 0)
    .sort((a, b) => b.score - a.score)
    .map((row) => row.s);
}

/** ⌨️ Inline completion suffix for the active suggestion (longest prefix match on key, label, or detail). */
export function replCompletionSuffix(query: string, suggestion: ReplSuggestion | undefined): string {
  if (!query.trim() || !suggestion) return "";
  const q = query;
  const ql = q.toLowerCase();
  let best = "";
  for (const text of [suggestion.label, suggestion.detail, suggestion.key].filter((value): value is string => Boolean(value))) {
    if (!text.toLowerCase().startsWith(ql)) continue;
    const suffix = text.slice(q.length);
    if (suffix.length > best.length) best = suffix;
  }
  return best;
}

/** ⌨️ First non-empty inline completion suffix across ranked matches. */
export function replActiveCompletionSuffix(query: string, matches: readonly ReplSuggestion[], index: number): string {
  if (!query.trim() || !matches.length) return "";
  const order = [matches[Math.min(index, matches.length - 1)]!, ...matches];
  const seen = new Set<ReplSuggestion>();
  for (const s of order) {
    if (seen.has(s)) continue;
    seen.add(s);
    const suffix = replCompletionSuffix(query, s);
    if (suffix) return suffix;
  }
  return "";
}

export function replPaletteRows(cmdLine: string, all: readonly ReplSuggestion[]): ReplSuggestion[] {
  return replFilterSuggestions(cmdLine, all);
}

export function replInteractionSuggestions(query: string, all: readonly ReplSuggestion[]): ReplSuggestion[] {
  const xs = query.trim() ? replFilterSuggestions(query, all) : all;
  return xs.filter((suggestion) => suggestion.kind === "interaction");
}

function replExactInteractionSuggestion(query: string, all: readonly ReplSuggestion[]): ReplSuggestion | null {
  const raw = query.trim().toLowerCase();
  if (!raw) return null;
  for (const suggestion of all) {
    if (suggestion.kind !== "interaction") continue;
    for (const text of [suggestion.key, suggestion.label, suggestion.detail].filter((value): value is string => Boolean(value))) {
      if (text.toLowerCase() === raw) return suggestion;
    }
  }
  return null;
}

function replInteractionSuggestionOnSpace(query: string, matches: readonly ReplSuggestion[], all: readonly ReplSuggestion[]): ReplSuggestion | null {
  const exact = replExactInteractionSuggestion(query, all);
  if (exact) return exact;
  return matches.find((suggestion) => suggestion.kind === "interaction") ?? null;
}

export function replInteractionIdOnSpace(query: string, matches: readonly ReplSuggestion[], all: readonly ReplSuggestion[], lastFinalizedInteractionId: string, repeatLastWhenIdle: boolean): string | null {
  if (!query.trim()) return repeatLastWhenIdle ? lastFinalizedInteractionId || null : null;
  return replInteractionSuggestionOnSpace(query, matches, all)?.interactionId ?? null;
}

/** ⌨️ True when the event target is already a text field (skip REPL global key capture). */
export function replIsQueryTypingTarget(t: EventTarget | null): boolean {
  return isUiTypingTarget(t);
}

export function replShouldRepeatInteractionOnSpace(
  event: {
    readonly key: string;
    readonly ctrlKey: boolean;
    readonly metaKey: boolean;
    readonly altKey: boolean;
    readonly defaultPrevented: boolean;
    readonly isComposing: boolean;
    readonly target: EventTarget | null;
  },
  state: {
    readonly interactionId: string;
    readonly interactionActive: boolean;
    readonly cmdTarget: EventTarget | null;
  },
): boolean {
  if (event.defaultPrevented || event.isComposing || state.interactionId || state.interactionActive) return false;
  if (event.key !== " " || event.ctrlKey || event.metaKey || event.altKey) return false;
  if (replIsQueryTypingTarget(event.target)) return false;
  return event.target !== state.cmdTarget;
}

export function replEscapeAction(state: { readonly hasInteraction: boolean; readonly interactionActive: boolean; readonly cmdLine: string; readonly hasSelectionMenu: boolean }): "abort" | "dismiss" | "none" {
  if (state.hasInteraction || state.interactionActive) return "abort";
  if (state.cmdLine.trim() || state.hasSelectionMenu) return "dismiss";
  return "none";
}

export function replSelectionEvent(selection: readonly SelectionTarget[], point?: Vec3): InteractionEvent {
  return point ? { kind: "selection.changed", targets: selection, point, modifiers: {} } : { kind: "selection.changed", targets: selection, modifiers: {} };
}

export function replStartEvent(selection: readonly SelectionTarget[]): InteractionEvent {
  return { kind: "start", targets: selection, modifiers: {} };
}

export function replSelectionAccepted(accept: readonly ModelEntityKind[], selection: readonly SelectionTarget[]): SelectionTarget[] {
  return selection.filter((target) => accept.includes(target.kind));
}

/** 🪪️ Reads validated `context.targets` for interaction highlight sync. */
export function replInteractionSelectionFromContext(ctx: Record<string, unknown>): readonly SelectionTarget[] {
  const raw = ctx.targets;
  if (!Array.isArray(raw)) return [];
  return raw.filter((target): target is SelectionTarget => {
    return Boolean(target && typeof target === "object" && "kind" in target && "id" in target && typeof (target as { kind?: unknown }).kind === "string" && typeof (target as { id?: unknown }).id === "string");
  });
}

/** 🪪️ Shallow equality for ordered selection target lists. */
export function replSelectionTargetsEqual(a: readonly SelectionTarget[], b: readonly SelectionTarget[]): boolean {
  if (a.length !== b.length) return false;
  for (let i = 0; i < a.length; i++) {
    const x = a[i]!;
    const y = b[i]!;
    if (x.kind !== y.kind || x.id !== y.id || x.editable !== y.editable) return false;
  }
  return true;
}

function replApplySelectionPick(current: readonly SelectionTarget[], picked: readonly SelectionTarget[], modifiers: InteractionEvent["modifiers"]): SelectionTarget[] {
  const modeModifiers = (modifiers ?? {}) as { readonly alt?: boolean; readonly ctrl?: boolean; readonly meta?: boolean; readonly shift?: boolean };
  return mergeSelectionTargets(current, picked, spatialSelectionModeFromModifiers(modeModifiers));
}

/** 🗂️ Renderer highlight targets keyed by model definition id. */
export type SpatialRendererSelectionByModel = Readonly<Record<string, readonly SelectionTarget[]>>;

/** 🗂️ Interaction pick targets keyed by interaction state id (session-local). */
export type SpatialInteractionSelectionByState = Readonly<Record<string, readonly SelectionTarget[]>>;

/** 🪪️ Reads renderer selection for one model definition (empty when unset). */
export function replRendererSelectionTargets(byModel: SpatialRendererSelectionByModel, modelDefinitionId: string): readonly SelectionTarget[] {
  return byModel[modelDefinitionId] ?? [];
}

/** 🪪️ Updates renderer selection for one model definition without touching other models. */
export function replWithRendererSelectionTargets(byModel: SpatialRendererSelectionByModel, modelDefinitionId: string, targets: readonly SelectionTarget[]): SpatialRendererSelectionByModel {
  const prev = byModel[modelDefinitionId] ?? [];
  if (replSelectionTargetsEqual(prev, targets)) return byModel;
  return { ...byModel, [modelDefinitionId]: [...targets] };
}

/** 🪪️ Reads interaction selection for one state (empty when unset). */
export function replInteractionSelectionTargets(byState: SpatialInteractionSelectionByState, stateId: string): readonly SelectionTarget[] {
  return byState[stateId] ?? [];
}

/** 🪪️ Updates interaction selection for one state without touching other states. */
export function replWithInteractionSelectionTargets(byState: SpatialInteractionSelectionByState, stateId: string, targets: readonly SelectionTarget[]): SpatialInteractionSelectionByState {
  const prev = byState[stateId] ?? [];
  if (replSelectionTargetsEqual(prev, targets)) return byState;
  return { ...byState, [stateId]: [...targets] };
}

/** 🪪️ Removes in-view targets of a pick kind when its selection toggle is turned off. */
export function replPruneSelectionByKind(selection: readonly SelectionTarget[], activeModelDefinitionId: string | null, kind: SpatialPickTargetKind): SelectionTarget[] {
  if (!spatialPickKindsForActiveView(activeModelDefinitionId).has(kind)) return [...selection];
  if (kind === "object") {
    return selection.filter((target) => target.kind !== "object" || target.editable !== false);
  }
  const geometryKinds: readonly ModelEntityKind[] = kind === "vertex" ? ["vertex", "anchor"] : kind === "edge" ? ["edge", "wire"] : kind === "face" ? ["face", "shell"] : ["solid", "geometry"];
  return selection.filter((target) => !geometryKinds.includes(target.kind) && selectionTargetPickKind(target) !== kind);
}

/** 🪪️ Removes in-view selection rows for a factory primitive kind when its filter toggle is turned off. */
export function replPruneSelectionByPrimitive(selection: readonly SelectionTarget[], primitiveKind: ModelEntityKind): SelectionTarget[] {
  return selection.filter((target) => {
    if (target.kind === "object" && target.editable === false) return true;
    return target.kind !== primitiveKind;
  });
}

/** 🪪️ Removes in-view selection rows for a typology when its selection toggle is turned off. */
export function replPruneSelectionByTypology(selection: readonly SelectionTarget[], model: Model, activeModelDefinitionId: string | null, typologyId: string): SelectionTarget[] {
  const typologyIds = modelDefinitionTypologyIds(activeModelDefinitionId);
  if (!typologyIds.includes(typologyId)) return [...selection];
  const index = buildGeometryTypologyIndex(model, activeModelDefinitionId ?? defaultModelDefinitionId());
  return selection.filter((target) => {
    if (target.kind === "object" && target.editable === false) {
      const row = model.objects[target.id];
      return row?.typology !== typologyId;
    }
    const geometryKind = target.kind === "object" ? "solid" : target.kind;
    return index.get(`${geometryKind}:${target.id}`) !== typologyId;
  });
}

/** 🪪️ Picks the highlight layer: interaction state selection while active, else renderer selection for the active model. */
export function replDisplayedSelectionTargets(
  interactionActive: boolean,
  activeModelDefinitionId: string | null,
  interactionState: string,
  rendererByModel: SpatialRendererSelectionByModel,
  interactionByState: SpatialInteractionSelectionByState,
): readonly SelectionTarget[] {
  const mdId = activeModelDefinitionId ?? defaultModelDefinitionId();
  if (interactionActive) return replInteractionSelectionTargets(interactionByState, interactionState);
  return replRendererSelectionTargets(rendererByModel, mdId);
}

/** 🪪️ Merges a pick into the active renderer model or interaction state selection slice. */
export function replMergeSelectionPickInView(
  interactionActive: boolean,
  activeModelDefinitionId: string | null,
  interactionState: string,
  rendererByModel: SpatialRendererSelectionByModel,
  interactionByState: SpatialInteractionSelectionByState,
  picked: readonly SelectionTarget[],
  modifiers: InteractionEvent["modifiers"] = {},
): SelectionTarget[] {
  const mdId = activeModelDefinitionId ?? defaultModelDefinitionId();
  const current = interactionActive ? replInteractionSelectionTargets(interactionByState, interactionState) : replRendererSelectionTargets(rendererByModel, mdId);
  return replApplySelectionPick(current, picked, modifiers);
}

/** 🪪️ Applies archived interaction result to renderer selection for the active model when `archiveContext.targets` is set (including `[]`). */
export function replFinalizeSelection(rendererByModel: SpatialRendererSelectionByModel, activeModelDefinitionId: string | null, result: InteractionSnapshot["lastResponse"]): SpatialRendererSelectionByModel {
  const ctx = result?.archiveContext;
  const mdId = activeModelDefinitionId ?? defaultModelDefinitionId();
  if (!ctx || typeof ctx !== "object" || !Object.hasOwn(ctx, "targets")) return rendererByModel;
  const targets = replInteractionSelectionFromContext(ctx as Record<string, unknown>);
  return replWithRendererSelectionTargets(rendererByModel, mdId, targets);
}

/** 🪩️ Memoized `DocumentHistory` for REPL hosts. */
export function useDocumentHistory(): DocumentHistory {
  return reactHostPort.useMemo(() => new DocumentHistory(), []);
}

/** 🪩️ Labels + capability mirror for undo/redo chrome (uses `InteractionSnapshot.capabilities`). */
export function getReplHistoryPresentation(spec: InteractionSpec, snap: InteractionSnapshot, history: DocumentHistory): { readonly canUndo: boolean; readonly canRedo: boolean; readonly undoLabel: string; readonly redoLabel: string } {
  const active = isInteractionSessionActive(spec, snap.state);
  const u = history.peekUndo()?.label ?? "";
  const r = history.peekRedo()?.label ?? "";
  return {
    canUndo: snap.capabilities.canUndo,
    canRedo: snap.capabilities.canRedo,
    undoLabel: active ? "Interaction input" : u,
    redoLabel: active ? "Interaction input" : r,
  };
}

/** 🪩️ Subscribes to runtime revisions and derives REPL undo/redo labels. */
export function useReplHistoryState(rt: InteractionRuntime, spec: InteractionSpec, history: DocumentHistory) {
  const snap = useInteractionSnapshot(rt);
  return reactHostPort.useMemo(() => getReplHistoryPresentation(spec, snap, history), [spec, snap, history]);
}

/** 🎛️ Optional controlled chrome state for {@link InteractionRepl}. */
export interface InteractionReplHostValues {
  readonly cmdLine?: string;
  readonly activeSuggestionIndex?: number;
  readonly filterTypologyToggles?: SpatialTypologyToggles;
  readonly selectionTypologyToggles?: SpatialTypologyToggles;
  readonly filterPrimitiveToggles?: SpatialPrimitiveToggles;
  readonly selectionPrimitiveToggles?: SpatialPrimitiveToggles;
  readonly activeModelDefinitionId?: string | null;
  readonly selectionMethod?: SpatialSelectionMethod;
  readonly modelDefinitionRevision?: number;
  readonly dragSelection?: SpatialDragSelectionState | null;
  readonly selectionMenu?: SpatialSelectionRequest | null;
  readonly hoveredPickKey?: string | null;
  readonly rendererSelectionByModel?: SpatialRendererSelectionByModel;
  readonly interactionSelectionByState?: SpatialInteractionSelectionByState;
  readonly interactionMenuOpen?: boolean;
  readonly lastFinalizedInteractionId?: string;
}

/** 📡️ Optional `on*` host callbacks for {@link InteractionRepl}. */
export interface InteractionReplHostCallbacks {
  readonly onCmdLineChange?: (value: string) => void;
  readonly onActiveSuggestionIndexChange?: (index: number) => void;
  readonly onFilterTypologyTogglesChange?: (value: SpatialTypologyToggles) => void;
  readonly onSelectionTypologyTogglesChange?: (value: SpatialTypologyToggles) => void;
  readonly onFilterPrimitiveTogglesChange?: (value: SpatialPrimitiveToggles) => void;
  readonly onSelectionPrimitiveTogglesChange?: (value: SpatialPrimitiveToggles) => void;
  readonly onActiveModelDefinitionIdChange?: (value: string) => void;
  readonly onSelectionMethodChange?: (value: SpatialSelectionMethod) => void;
  readonly onModelDefinitionRevisionChange?: (revision: number) => void;
  readonly onDragSelectionChange?: (value: SpatialDragSelectionState | null) => void;
  readonly onSelectionMenuChange?: (value: SpatialSelectionRequest | null) => void;
  readonly onHoveredPickKeyChange?: (key: string | null) => void;
  readonly onRendererSelectionByModelChange?: (value: SpatialRendererSelectionByModel) => void;
  readonly onInteractionSelectionByStateChange?: (value: SpatialInteractionSelectionByState) => void;
  readonly onInteractionMenuOpenChange?: (open: boolean) => void;
  readonly onLastFinalizedInteractionIdChange?: (id: string) => void;
  readonly onCanvasReady?: InteractionCanvasProps["onCanvasReady"];
  readonly onInteractionEvent?: (event: InteractionEvent) => void;
  readonly onGroundPick?: (point: Vec3, event: InteractionEvent) => void;
  readonly onScenePointerMove?: (point: Vec3, event: InteractionEvent) => void;
  readonly onSelectionRequest?: (request: SpatialSelectionRequest) => void;
  readonly onHoverTarget?: (target: SpatialPickTarget | null) => void;
  readonly onCameraNavigate?: (active: boolean) => void;
  readonly onActionSubmit?: (line: string) => boolean | void;
  readonly onTransitionRun?: (row: InteractionKeybindRow) => void;
  readonly onCancel?: () => void;
  readonly onUndo?: () => void;
  readonly onRedo?: () => void;
  readonly onSnapshotChange?: (snapshot: InteractionSnapshot) => void;
  /** 💬️ Publishes the window engagement spec (or `null`) whenever the interaction state changes; the host renders it in the {@link Window} engagement slot. */
  readonly onEngagementChange?: (engagement: EngagementSpec | null) => void;
  /** 🔎️ Publishes the window search spec (or `null`) whenever the interaction state changes; the host renders it in the {@link Window} search slot. */
  readonly onSearchChange?: (search: SearchSpec | null) => void;
  readonly onEscape?: () => void;
  /** 🗑️ Delete/Backspace when deletable selection exists; return true when handled. */
  readonly onDeleteSelection?: () => boolean;
  readonly onApplyTransformation?: (spec: TransformationSpec) => void;
  /** 🧲️ Geometry used for pick targets (defaults to `geometry`; use spatial.shape geometry when the active model is typology-only). */
  readonly pickGeometry?: SpatialPickGeometry | null;
  readonly onDocumentModelChange?: (model: Model) => void;
}

/** 📐️ Layout and partial canvas/spatial-view overrides for {@link InteractionRepl}. */
export interface InteractionReplLayoutProps {
  readonly rootStyle?: CSSProperties;
  readonly asideStyle?: CSSProperties;
  readonly showAside?: boolean;
  /** 💬️ Builds the window {@link EngagementSpec} (interaction options/status/control) and {@link SearchSpec} (action input/possibles), published via {@link InteractionReplHostCallbacks.onEngagementChange} and {@link InteractionReplHostCallbacks.onSearchChange}. */
  readonly showEngagement?: boolean;
  /** 📐️ Size the REPL to its host instead of the viewport (`100vh`); stacks aside under the canvas. */
  readonly fillHost?: boolean;
  /** ⌨️ Registers document-level REPL key capture; disable on inactive hosts (e.g. CAD play quad panes). */
  readonly captureGlobalKeys?: boolean;
  /** 🙈️ Hides model-definition and transformation dropdowns (e.g. play hosts them in `asideExtra`). */
  readonly hideModelDefinitionControls?: boolean;
  readonly frameloop?: InteractionCanvasProps["frameloop"];
  readonly canvas?: Omit<InteractionCanvasProps, "children">;
  /** 🖼️ Spread after REPL wiring; overrides win (use for theme/slots/face handlers, not session pick state). */
  readonly spatialView?: Omit<
    InteractionSpatialViewProps,
    | "snapshot"
    | "geometry"
    | "committedMeshes"
    | "displayModel"
    | "modelDefinitionRevision"
    | "activeModelDefinitionId"
    | "filterTypologyToggles"
    | "selectionTypologyToggles"
    | "hoveredTargetKey"
    | "selectedTargetKey"
    | "selectedTargetKeys"
    | "selectionAccept"
    | "showPickLayer"
    | "pickEnabled"
    | "onInteractionEvent"
    | "onScenePointerMove"
    | "onSelectionRequest"
    | "onCameraNavigate"
    | "onGroundPick"
  >;
}

/** 🎛️ Default uncontrolled chrome for {@link InteractionRepl}. */
export function defaultInteractionReplChromeState(): Required<
  Pick<
    InteractionReplHostValues,
    | "cmdLine"
    | "activeSuggestionIndex"
    | "filterTypologyToggles"
    | "selectionTypologyToggles"
    | "filterPrimitiveToggles"
    | "selectionPrimitiveToggles"
    | "activeModelDefinitionId"
    | "selectionMethod"
    | "modelDefinitionRevision"
    | "dragSelection"
    | "selectionMenu"
    | "hoveredPickKey"
    | "rendererSelectionByModel"
    | "interactionSelectionByState"
    | "interactionMenuOpen"
    | "lastFinalizedInteractionId"
  >
> {
  return {
    cmdLine: "",
    activeSuggestionIndex: 0,
    filterTypologyToggles: defaultSpatialTypologyTogglesForModelDefinition(defaultModelDefinitionId()),
    selectionTypologyToggles: defaultSpatialTypologyTogglesForModelDefinition(defaultModelDefinitionId()),
    filterPrimitiveToggles: defaultSpatialPrimitiveToggles(),
    selectionPrimitiveToggles: defaultSpatialPrimitiveToggles(),
    activeModelDefinitionId: defaultModelDefinitionId(),
    selectionMethod: "rectangle",
    modelDefinitionRevision: 0,
    dragSelection: null,
    selectionMenu: null,
    hoveredPickKey: null,
    rendererSelectionByModel: {},
    interactionSelectionByState: {},
    interactionMenuOpen: false,
    lastFinalizedInteractionId: "",
  };
}

export interface InteractionReplProps extends InteractionReplHostValues, InteractionReplHostCallbacks, InteractionReplLayoutProps {
  readonly interactionId: string;
  readonly spec: InteractionSpec;
  readonly onInteractionId: (id: string) => void;
  readonly runtime: InteractionRuntime;
  readonly history: DocumentHistory;
  readonly document: ModelDocument;
  readonly geometry: SpatialPickGeometry | null;
  readonly asideExtra?: ReactNode;
  readonly archivedBoxLayouts?: readonly ArchivedBoxLayout[];
  /** 🔁️ When host bumps this positive counter for the same interaction, `cancel()` then `start` without remounting GL. */
  readonly sessionRestartNonce?: number;
  readonly viewTheme?: InteractionSpatialViewTheme;
  readonly viewSlots?: InteractionSpatialViewSlots;
  readonly renderDisplayItem?: SpatialDisplayItemRenderer;
  readonly autoFitMeshes?: boolean;
  readonly autoFitBehavior?: SpatialAutoFitBehavior;
  readonly committedMeshesKeepPrevious?: boolean;
  readonly tessellationTolerance?: number;
  /** 🎛️ Utility bar gumball mode; hidden while an interaction session is active. */
  readonly transformGumballConfig?: CadGumballConfig | null;
  readonly onTransformGumballCommit?: (diff: ModelDiff) => void;
  readonly worldReferences?: readonly WorldReferenceProps[];
  readonly selectedReferenceIds?: ReadonlySet<string>;
  readonly hoveredReferenceId?: string | null;
  readonly revealedReferenceIds?: ReadonlySet<string>;
  readonly referenceRelocateActive?: boolean;
  readonly onReferenceSelect?: InteractionSpatialViewProps["onReferenceSelect"];
  readonly onReferenceHover?: InteractionSpatialViewProps["onReferenceHover"];
  readonly onReferenceRelocate?: (payload: WorldReferenceRelocatePayload) => void;
}

/** 💬️ One interaction a window can start from the floating pane while idle. */
export interface InteractionReplEngagementInteraction {
  readonly id: string;
  readonly key: string;
  readonly label: string;
}

function engagementRingOptionNumericValue(optionId: string): number | null {
  const match = optionId.match(/^(?:angle|length)-(.+)$/);
  if (!match) return null;
  const value = Number(match[1]);
  return Number.isFinite(value) ? value : null;
}

/** 🎛️ Maps {@link ResolvedInteractionEngagementControl} to a live {@link EngagementControl}. */
export function buildEngagementControlFromResolved(
  resolved: ResolvedInteractionEngagementControl,
  handlers: {
    readonly onNumericChange: (value: number) => void;
    readonly onNumericCommit?: (value: number) => void;
  },
): EngagementControl {
  if (resolved.kind === "ring") {
    return {
      kind: "ring",
      id: `engagement-control-${resolved.label.toLowerCase().replace(/\s+/g, "-")}`,
      label: uiDataLabel(resolved.label),
      value: resolved.value,
      options: resolved.options.map((row) => ({ id: row.id, label: row.label })),
      onSelect: (id) => {
        const numeric = engagementRingOptionNumericValue(id);
        if (numeric !== null) handlers.onNumericChange(numeric);
      },
    };
  }
  if (resolved.kind === "slider") {
    return {
      kind: "slider",
      id: `engagement-control-${resolved.label.toLowerCase().replace(/\s+/g, "-")}`,
      label: uiDataLabel(resolved.label),
      value: resolved.value,
      min: resolved.min,
      max: resolved.max!,
      step: resolved.step,
      unit: resolved.unit,
      onChange: handlers.onNumericChange,
      onCommit: handlers.onNumericCommit,
    };
  }
  return {
    kind: "stepper",
    id: `engagement-control-${resolved.label.toLowerCase().replace(/\s+/g, "-")}`,
    label: uiDataLabel(resolved.label),
    value: resolved.value,
    min: resolved.min,
    max: resolved.max,
    step: resolved.step,
    unit: resolved.unit,
    onChange: handlers.onNumericChange,
    onCommit: handlers.onNumericCommit,
  };
}

/** 💬️ Inputs for {@link buildInteractionReplEngagement} (interaction state + callbacks for the floating pane). */
export interface InteractionReplEngagementInputs {
  readonly showEngagement: boolean;
  readonly boundInteractionSession: boolean;
  readonly interactionId: string;
  readonly state: string;
  readonly lastResponseOk: boolean | null;
  readonly lastResponseErrorCount: number;
  readonly selectionCount: number;
  readonly cmdLine: string;
  readonly control?: EngagementControl;
  readonly transitions: readonly InteractionKeybindRow[];
  readonly interactions: readonly InteractionReplEngagementInteraction[];
  readonly onTransition: (row: InteractionKeybindRow) => void;
  readonly onStartInteraction: (interactionId: string) => void;
  readonly onInputChange: (value: string) => void;
  readonly onInputSubmit: (value: string) => void;
  readonly onRepeatLast?: () => void;
  readonly onAbort?: () => void;
}

/** 🏷️ Omits machine ids from action suggestion sublines (keeps short shortcut keys). */
export function replUserFacingSuggestionDetail(detail: string): string | undefined {
  const trimmed = detail.trim();
  if (!trimmed) return undefined;
  if (trimmed.includes(".") || trimmed.includes("_")) return undefined;
  if (/^(action|interaction|transition)$/i.test(trimmed)) return undefined;
  if (trimmed.length <= 2) return trimmed;
  return undefined;
}

/** 💬️ Builds the compact window {@link EngagementSpec}: an active session lists its transitions as options, plus state/selection/response status. */
export function buildInteractionReplEngagement(inputs: InteractionReplEngagementInputs): EngagementSpec | null {
  if (!inputs.showEngagement) return null;
  const options = inputs.boundInteractionSession
    ? inputs.transitions.map((row) => ({
        id: `engagement-transition-${row.eventKind}-${row.key}`,
        label: uiDataLabel(normalizeEngagementActionText(`${row.key} ${row.label}`)),
        icon: "arrow-right" as IconName,
        onPress: () => inputs.onTransition(row),
      }))
    : [];
  const status: { id: string; content: ReactNode }[] = [];
  if (inputs.interactionId) status.push({ id: "engagement-step", content: `Step: ${humanizeEngagementStepId(inputs.state)}` });
  if (inputs.selectionCount > 0) {
    status.push({
      id: "engagement-selection",
      content: inputs.selectionCount === 1 ? "1 selected" : `${inputs.selectionCount} selected`,
    });
  }
  if (inputs.lastResponseOk !== null) {
    status.push({
      id: "engagement-response",
      content: inputs.lastResponseOk ? "OK" : `Error${inputs.lastResponseErrorCount > 0 ? ` (${inputs.lastResponseErrorCount})` : ""}`,
    });
  }
  if (options.length === 0 && !inputs.control && status.length === 0) return null;
  return {
    sessionActive: inputs.boundInteractionSession,
    options: options.length ? options : undefined,
    control: inputs.control,
    status: status.length ? status : undefined,
  };
}

/** 🔎️ Builds the top-middle window {@link SearchSpec}: idle exposes an action input to start an interaction, an active session accepts step values; both offer autocomplete possibles. */
export function buildInteractionReplSearch(inputs: InteractionReplEngagementInputs): SearchSpec | null {
  if (!inputs.showEngagement) return null;
  const input =
    inputs.boundInteractionSession || inputs.interactions.length > 0 || inputs.onRepeatLast
      ? {
          id: "search-input",
          value: inputs.cmdLine,
          placeholder: uiDataLabel(inputs.boundInteractionSession ? WINDOW_SEARCH_USER.actionPlaceholderActive : WINDOW_SEARCH_USER.actionPlaceholder),
          onChange: inputs.onInputChange,
          onSubmit: inputs.onInputSubmit,
          onRepeatLast: inputs.onRepeatLast,
          onAbort: inputs.onAbort,
        }
      : undefined;
  const possibles = inputs.boundInteractionSession
    ? inputs.transitions.map((row) => ({
        id: `engagement-possible-transition-${row.eventKind}-${row.key}`,
        label: normalizeEngagementActionText(`${row.key} ${row.label}`),
        onSelect: () => inputs.onTransition(row),
      }))
    : inputs.interactions.map((interaction) => ({
        id: interaction.id,
        label: normalizeEngagementActionText(interaction.label),
        detail: replUserFacingSuggestionDetail(interaction.key),
        onSelect: () => inputs.onStartInteraction(interaction.id),
      }));
  if (!input && possibles.length === 0) return null;
  return {
    sessionActive: inputs.boundInteractionSession,
    input,
    possibles: possibles.length ? possibles : undefined,
  };
}

/** 🪪️ Element id of the CAD editor's projection pane — dot-separated camelCase, the grammar `Pane`'s own
 * `assertElementId` demands and `childElementId` extends for the two fold controls it derives. */
const CAD_ORBIT_PROJECTION_PANE_ID = "cad.orbitProjection";

/** 🔀️ Portals the world's projection-kind switch into the enclosing window's pane host (see `usePaneSlot`). Defaults to bottom-right under the navigation cube; the unfolded pane grows over it. Falls back to a local overlay when no pane host is available. */
export function WorldOrbitProjectionSwitchPane({ spec, onSpecChange }: { readonly spec: WorldProjectionSpec; readonly onSpecChange: (spec: WorldProjectionSpec) => void }) {
  const [anchor, setAnchor] = reactHostPort.useState<Anchor>("bottom-right");
  const [folded, setFolded] = reactHostPort.useState(true);
  const projectionLabel = useLabel("ui.host.projection");
  const pane = (
    <Pane
      id={CAD_ORBIT_PROJECTION_PANE_ID}
      anchor={anchor}
      onAnchorChange={setAnchor}
      folded={folded}
      onFoldToggle={() => setFolded((value) => !value)}
      icon={worldProjectionSpecIconId(spec) as IconName}
      label={projectionLabel}
      bodyClassName={windowMeasuresBodyClass}
      stackDataAttrs={{ "data-level": "pane" }}
      dimWhenOpen
    >
      <WorldProjectionKindSwitch id={CAD_ORBIT_PROJECTION_PANE_ID} spec={spec} onSpecChange={onSpecChange} />
    </Pane>
  );
  const portaled = usePaneSlot(pane);
  if (portaled) return portaled;
  return (
    <div className="pointer-events-none absolute inset-0" data-slot="world-orbit-projection-switch-fallback" data-world-projection-kind-switch-host="">
      {pane}
    </div>
  );
}
// #endregion 🪩️Repl
