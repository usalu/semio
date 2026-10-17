/** 🎯️ CAD renderer geometry targets: the spatial pick kinds, targets and requests, the visibility and typology toggles and the scene visibility resolution. */
import { sortCanvasPickTargetsGeneralFirst, type CanvasPickRequest, type CanvasPickTarget, type CheckboxState } from "@semio-tech/framework/ui/targets/react";
import { worldEntitySelectable } from "@semio-tech/infinite-world-r3f";
import { type Vec3 } from "@semio-tech/cad-engine-ts/registry";
import { type ModelEntityKind, type ModelJson } from "@semio-tech/s-spatial-kernel-ts/geometry";
import { isShapeModelDefinition, defaultModelDefinitionId } from "@semio-tech/s-spatial-kernel-ts/geometry/definition-catalog";
import { type SpatialEntityFlags } from "@semio-tech/s-spatial-kernel-ts/geometry/paths";
import { Model } from "@semio-tech/s-spatial-kernel-ts/geometry/model";
import { modelDefinitionUsesGeometryPicking, PRIMITIVE_MODEL_ENTITY_KINDS, listTypologiesForModelDefinition } from "@semio-tech/s-spatial-kernel-ts/geometry/definition-ownership";
import { modelDefinitionSelectionEntityKinds } from "@semio-tech/s-spatial-kernel-ts/geometry/selection-operations";
import { type InteractionEvent, type SelectionTarget } from "@semio-tech/s-spatial-kernel-ts/geometry/selection";
import { typologyObjectPascalFromLabel } from "../../../🧬️typology/🟦️.ts";

// #region 🧲️GeometryTargets
export type SpatialPickKind = "pointer.down" | "pointer.move";

/** 🎯️ Primitive and object pick kinds for renderer feedback (maps to kernel geometry via {@link SpatialPickTarget.geometryKind}). */
export type SpatialGeometryPickTargetKind = "object" | "face" | "edge" | "vertex";

export type SpatialPickTargetKind = SpatialGeometryPickTargetKind;

export const SPATIAL_PICK_TARGET_KINDS: readonly SpatialPickTargetKind[] = ["object", "face", "edge", "vertex"];

const CAD_PICK_GENERALITY: Readonly<Record<SpatialPickTargetKind, number>> = {
  object: 0,
  face: 1,
  edge: 2,
  vertex: 3,
};

function spatialPickTargetToCanvas(target: SpatialPickTarget): CanvasPickTarget {
  return { domain: target.kind, id: target.id, generality: CAD_PICK_GENERALITY[target.kind], label: target.id };
}

function canvasPickModifiersFromEvent(modifiers: InteractionEvent["modifiers"]): Readonly<Record<string, boolean>> | undefined {
  if (!modifiers || typeof modifiers !== "object") return undefined;
  const out: Record<string, boolean> = {};
  for (const [key, value] of Object.entries(modifiers)) if (typeof value === "boolean") out[key] = value;
  return out;
}

export function spatialPickCanvasRequest(request: SpatialSelectionRequest | null): CanvasPickRequest | null {
  if (!request) return null;
  return {
    targets: sortCanvasPickTargetsGeneralFirst(request.targets.map(spatialPickTargetToCanvas)),
    client: request.client,
    modifiers: canvasPickModifiersFromEvent(request.modifiers),
  };
}

export const GEOMETRY_KIND_TO_OBJECT_PICK: Partial<Record<ModelEntityKind, SpatialGeometryPickTargetKind>> = {
  vertex: "vertex",
  edge: "edge",
  wire: "edge",
  face: "face",
  shell: "face",
  solid: "object",
  anchor: "vertex",
};

export function spatialPickKindsForSelectionAccept(accept: readonly ModelEntityKind[]): ReadonlySet<SpatialPickTargetKind> | null {
  if (!accept.length) return null;
  const out = new Set<SpatialPickTargetKind>();
  for (const kind of accept) {
    if (kind === "object" || kind === "face" || kind === "edge" || kind === "vertex") {
      out.add(kind);
      continue;
    }
    const mapped = GEOMETRY_KIND_TO_OBJECT_PICK[kind];
    if (mapped) out.add(mapped);
  }
  return out;
}

export function kernelGeometryKindForObjectPick(kind: SpatialGeometryPickTargetKind, geometryKind?: ModelEntityKind): ModelEntityKind {
  if (geometryKind) return geometryKind;
  if (kind === "vertex") return "vertex";
  if (kind === "edge") return "edge";
  if (kind === "face") return "face";
  return "solid";
}

/** 👁️ Per-kind on/off map for visibility filters or selection/hover gates (`false` disables). */
export type SpatialPickKindToggles = Partial<Record<SpatialPickTargetKind, boolean>>;

/** 👁️ Per-typology on/off map for play chrome (`false` disables show or selection). */
export type SpatialTypologyToggles = Partial<Record<string, boolean>>;

export interface SpatialPickTarget {
  readonly kind: SpatialPickTargetKind;
  readonly id: string;
  readonly point: Vec3;
  readonly points?: readonly Vec3[];
  /** 🧭️ Kernel-private geometry entity kind for primitive picks (e.g. `wire` vs `edge`). */
  readonly geometryKind?: ModelEntityKind;
  /** 🏷️ Typology id when the target belongs to a model-definition object row. */
  readonly typologyId?: string;
}

export interface SpatialSelectionRequest {
  readonly targets: readonly SpatialPickTarget[];
  readonly point: Vec3;
  readonly client: { readonly x: number; readonly y: number };
  readonly modifiers: InteractionEvent["modifiers"];
}

export type SpatialSelectionMethod = "rectangle" | "lasso";
export type SpatialSelectionCoverage = "partial" | "full";
export type SpatialSelectionMode = "default" | "additive" | "subtractive" | "invertive";

export interface SpatialDragSelectionState {
  readonly method: SpatialSelectionMethod;
  readonly coverage: SpatialSelectionCoverage;
  readonly startClient: { readonly x: number; readonly y: number };
  readonly currentClient: { readonly x: number; readonly y: number };
  readonly path: readonly { readonly x: number; readonly y: number }[];
  readonly modifiers: InteractionEvent["modifiers"];
}

export type SpatialPickGeometry = Model | ModelJson;

export function spatialPickTargetKey(target: SpatialPickTarget): string {
  return `${target.kind}:${target.id}`;
}

/** 🪪️ Stable hover/selection key for a {@link SelectionTarget} (primitive kinds use entity kind + id). */
export function selectionTargetHoverKey(target: SelectionTarget): string {
  return `${target.kind}:${target.id}`;
}

export function spatialSelectionTargetKey(target: SelectionTarget): string {
  return selectionTargetHoverKey(target);
}

/** 👁️ Default all geometry pick kinds enabled (visibility + selection). */
export function defaultSpatialPickKindToggles(): Record<SpatialPickTargetKind, boolean> {
  return Object.fromEntries(SPATIAL_PICK_TARGET_KINDS.map((kind) => [kind, true])) as Record<SpatialPickTargetKind, boolean>;
}

/** 👁️ Filters pick targets by visibility (show/hide highlights); does not affect ray pick or selection. */
export function filterSpatialPickTargetsForVisibility(targets: readonly SpatialPickTarget[], filterKindToggles: SpatialPickKindToggles = {}): SpatialPickTarget[] {
  return targets.filter((target) => filterKindToggles[target.kind] !== false);
}

/** 👁️ Reads persisted hide/lock flags from a model document for a pick-target entity id. */
export function spatialEntityFlagsForModelEntity(model: Model | null | undefined, entityId: string): SpatialEntityFlags {
  return model?.metadata.getEntityFlags(entityId) ?? {};
}

/** 👁️ Excludes hidden/locked entities from canvas pick and selection. */
export function filterSpatialPickTargetsForEntityFlags(targets: readonly SpatialPickTarget[], flagsForId: (entityId: string) => SpatialEntityFlags): SpatialPickTarget[] {
  return targets.filter((target) => worldEntitySelectable(flagsForId(target.id)));
}

/** 👁️ Drops locked/hidden entities from committed selection targets. */
export function pruneSelectionTargetsForEntityFlags(targets: readonly SelectionTarget[], flagsForId: (entityId: string) => SpatialEntityFlags): SelectionTarget[] {
  return targets.filter((target) => worldEntitySelectable(flagsForId(target.id)));
}

/** 👁️ Effective pick kinds must be both visible and enabled for selection/hover. */
export function intersectSpatialPickKindToggles(visibleKindToggles: SpatialPickKindToggles = {}, selectionKindToggles: SpatialPickKindToggles = {}): SpatialPickKindToggles {
  const merged: SpatialPickKindToggles = {};
  for (const kind of SPATIAL_PICK_TARGET_KINDS) {
    if (visibleKindToggles[kind] === false || selectionKindToggles[kind] === false) merged[kind] = false;
  }
  return merged;
}

/** 👁️ Maps active model-definition entity kinds to renderer pick-kind toggles. */
export function modelDefinitionPickTargetKinds(modelDefinitionId: string | null): readonly SpatialPickTargetKind[] {
  const entityKinds = modelDefinitionSelectionEntityKinds(modelDefinitionId ?? defaultModelDefinitionId());
  const out = new Set<SpatialPickTargetKind>();
  for (const kind of entityKinds) {
    if (kind === "vertex" || kind === "anchor") out.add("vertex");
    else if (kind === "edge" || kind === "wire") out.add("edge");
    else if (kind === "face" || kind === "shell") out.add("face");
    else if (kind === "solid" || kind === "geometry" || kind === "object") out.add("object");
  }
  if (out.size > 0) return [...out];
  return isShapeModelDefinition(modelDefinitionId) ? SPATIAL_PICK_TARGET_KINDS : ["object"];
}

/** 👁️ Default visibility/selection toggles for kinds allowed by the active model definition. */
export function defaultSpatialPickKindTogglesForModelDefinition(modelDefinitionId: string | null): Record<SpatialPickTargetKind, boolean> {
  const allowed = new Set(modelDefinitionPickTargetKinds(modelDefinitionId));
  return Object.fromEntries(SPATIAL_PICK_TARGET_KINDS.map((kind) => [kind, allowed.has(kind)])) as Record<SpatialPickTargetKind, boolean>;
}

/** 👁️ Typology ids declared on the active model definition (sorted). */
export function modelDefinitionTypologyIds(modelDefinitionId: string | null): readonly string[] {
  return listTypologiesForModelDefinition(modelDefinitionId ?? defaultModelDefinitionId())
    .map((row) => row.id)
    .sort((a, b) => a.localeCompare(b));
}

/** 👁️ Default all typologies on the active model definition enabled for show/selection. */
export function defaultSpatialTypologyTogglesForModelDefinition(modelDefinitionId: string | null): Record<string, boolean> {
  return Object.fromEntries(modelDefinitionTypologyIds(modelDefinitionId).map((id) => [id, true]));
}

/** 🏷️ Short typology label for play chrome (`Base Plate` → `BasePlate`). */
export function spatialTypologyToggleLabel(typologyId: string, label?: string): string {
  if (label?.trim()) return typologyObjectPascalFromLabel(label);
  const tail = typologyId.split(".").pop() ?? typologyId;
  return typologyObjectPascalFromLabel(tail.replace(/[._-]+/g, " "));
}

function typologyToggleAllowsTarget(target: SpatialPickTarget, toggles: SpatialTypologyToggles, typologyIds: readonly string[]): boolean {
  if (target.typologyId) return toggles[target.typologyId] !== false;
  return typologyIds.some((id) => toggles[id] !== false);
}

/** 👁️ Filters pick targets by typology show/selection toggles. */
export function filterSpatialPickTargetsForTypologyToggles(targets: readonly SpatialPickTarget[], toggles: SpatialTypologyToggles, typologyIds: readonly string[]): SpatialPickTarget[] {
  return targets.filter((target) => typologyToggleAllowsTarget(target, toggles, typologyIds));
}

/** 👁️ Derives per-kind toggles from typology-filtered targets (scene layers + legacy gates). */
export function spatialPickKindTogglesFromTypologyFilteredTargets(modelDefinitionId: string | null, visibleTargets: readonly SpatialPickTarget[]): SpatialPickKindToggles {
  const allowed = new Set(modelDefinitionPickTargetKinds(modelDefinitionId));
  const merged: SpatialPickKindToggles = {};
  for (const kind of SPATIAL_PICK_TARGET_KINDS) {
    merged[kind] = allowed.has(kind) && visibleTargets.some((target) => target.kind === kind);
  }
  return merged;
}

/** 👁️ Scene-layer pick-kind toggles from model definition + primitive show toggles (not typology pick targets). */
export function spatialSceneKindTogglesForModelDefinition(modelDefinitionId: string | null, primitiveToggles: SpatialPrimitiveToggles = defaultSpatialPrimitiveToggles()): SpatialPickKindToggles {
  const toggles = defaultSpatialPickKindTogglesForModelDefinition(modelDefinitionId);
  if (primitiveToggles.vertex === false && primitiveToggles.anchor === false) toggles.vertex = false;
  if (primitiveToggles.edge === false && primitiveToggles.wire === false) toggles.edge = false;
  if (primitiveToggles.face === false && primitiveToggles.shell === false) toggles.face = false;
  if (primitiveToggles.solid === false) toggles.object = false;
  return toggles;
}

/** 👁️ Per-primitive on/off map for play chrome (`false` disables show or filter). */
export type SpatialPrimitiveToggles = Partial<Record<ModelEntityKind, boolean>>;

/** 🧱️ Factory primitive kinds toggled in play (anchor → solid). */
export const SPATIAL_PRIMITIVE_KINDS: readonly ModelEntityKind[] = PRIMITIVE_MODEL_ENTITY_KINDS;

/** 👁️ Default all factory primitive kinds enabled for show/filter. */
export function defaultSpatialPrimitiveToggles(): Record<ModelEntityKind, boolean> {
  return Object.fromEntries(SPATIAL_PRIMITIVE_KINDS.map((kind) => [kind, true])) as Record<ModelEntityKind, boolean>;
}

/** ☑️ Aggregate enabled state for a fixed-key boolean toggle map (`false` = off). */
export type SpatialToggleGroupState = "all" | "none" | "partial";

/** ☑️ Returns whether every key is on, every key is off, or the group is mixed. */
export function spatialToggleGroupState(keys: readonly string[], toggles: Readonly<Record<string, boolean | undefined>>): SpatialToggleGroupState {
  if (keys.length === 0) return "none";
  let on = 0;
  for (const key of keys) {
    if (toggles[key] !== false) on += 1;
  }
  if (on === 0) return "none";
  if (on === keys.length) return "all";
  return "partial";
}

/** ☑️ Sets every key in a chrome toggle group on or off. */
export function spatialToggleGroupFill<T extends string>(keys: readonly T[], enabled: boolean): Record<T, boolean> {
  return Object.fromEntries(keys.map((key) => [key, enabled])) as Record<T, boolean>;
}

/** ☑️ Maps a chrome group aggregate onto the owned native checkbox state. */
export function spatialToggleCheckboxState(state: SpatialToggleGroupState): CheckboxState {
  if (state === "partial") return "indeterminate";
  return state === "all";
}

/** 🧭️ Resolves the primitive entity kind for a pick target (typology object rows → `null`). */
export function pickTargetPrimitiveKind(target: SpatialPickTarget): ModelEntityKind | null {
  if (target.kind === "object" && !target.geometryKind) return null;
  return target.geometryKind ?? kernelGeometryKindForObjectPick(target.kind as SpatialGeometryPickTargetKind, target.geometryKind);
}

/** 👁️ Filters pick targets by primitive show/filter toggles (typology object rows pass through). */
export function filterSpatialPickTargetsForPrimitiveToggles(targets: readonly SpatialPickTarget[], toggles: SpatialPrimitiveToggles): SpatialPickTarget[] {
  return targets.filter((target) => {
    const primitive = pickTargetPrimitiveKind(target);
    if (!primitive) return true;
    return toggles[primitive] !== false;
  });
}

/** 👁️ Resolves which scene layers stay visible for geometry edit vs typology object picking. */
export function resolveSpatialSceneVisibility(
  activeModelDefinitionId: string | null,
  filterKindToggles: SpatialPickKindToggles = {},
): {
  readonly showFactoryWireframe: boolean;
  readonly showCommittedFaces: boolean;
  readonly showCommittedEdges: boolean;
} {
  const visible = (kind: SpatialPickTargetKind) => filterKindToggles[kind] !== false;
  const mdId = activeModelDefinitionId ?? defaultModelDefinitionId();
  if (modelDefinitionUsesGeometryPicking(mdId)) {
    return {
      showFactoryWireframe: visible("edge"),
      showCommittedFaces: visible("face") || visible("object"),
      showCommittedEdges: visible("edge"),
    };
  }
  return {
    showFactoryWireframe: false,
    showCommittedFaces: false,
    showCommittedEdges: false,
  };
}

export function spatialPickKindsForActiveView(activeModelDefinitionId: string | null): ReadonlySet<SpatialPickTargetKind> {
  return new Set(modelDefinitionPickTargetKinds(activeModelDefinitionId));
}

/** 👁️ Keeps pick targets allowed by the active model definition (primitives + typology objects). */
export function filterSpatialPickTargetsForActiveView(targets: readonly SpatialPickTarget[], activeModelDefinitionId: string | null): SpatialPickTarget[] {
  const mdId = activeModelDefinitionId ?? defaultModelDefinitionId();
  const allowedPickKinds = spatialPickKindsForActiveView(mdId);
  const entityKinds = new Set(modelDefinitionSelectionEntityKinds(mdId));
  return targets.filter((target) => {
    if (!allowedPickKinds.has(target.kind)) return false;
    if (target.kind === "object" && !target.geometryKind) {
      if (!entityKinds.has("object")) return false;
      return !isShapeModelDefinition(mdId);
    }
    const geometryKind = target.geometryKind ?? kernelGeometryKindForObjectPick(target.kind, undefined);
    return entityKinds.has(geometryKind);
  });
}
// #endregion 🧲️GeometryTargets
