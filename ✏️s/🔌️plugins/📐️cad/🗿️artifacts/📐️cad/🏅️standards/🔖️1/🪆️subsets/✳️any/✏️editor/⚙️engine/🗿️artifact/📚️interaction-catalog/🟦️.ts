/** 📚️ CAD editor artifact: the selection operation catalog, the shipped spatial interaction catalog, the interaction registry and the compiled interaction builders. */
import { ephemeralMap } from "@semio-tech/framework";
import { ModelEntityKind } from "@semio-tech/s-spatial-kernel-ts/geometry";
import { InteractionSpec, compileInteraction, parseInteractionSpec } from "@semio-tech/s-spatial-kernel-ts/geometry/interaction-spec";
import { SpatialInteraction, listActionsForModelDefinition, listSpatialInteractionsForModelDefinition } from "@semio-tech/s-spatial-kernel-ts/geometry/definition-ownership";
import { SelectionTarget } from "@semio-tech/s-spatial-kernel-ts/geometry/selection";
import { onModelDefinitionAssetsChanged, modelDefinitionInteractionCatalog } from "@semio-tech/s-spatial-kernel-ts/geometry/definition-catalog";
import { SelectionApplyOperation, SelectionApplyParams, SelectionOperationInteractionDef } from "../../🎬️actions/🧲️selection-targets/🟦️.ts";

// #region 📦️📄️document
// #region 📦️Interactions
type ModelDefinitionInteractionRecord = InteractionSpec & { readonly key?: string };

const SELECTION_INTERACTION_KEYS: Readonly<Record<string, string>> = {
  "selection.selectAll": "sa",
  "selection.deselectAll": "ds",
  "selection.invert": "iv",
  "selection.selectAnchors": "xa",
  "selection.selectVertices": "xv",
  "selection.selectEdges": "xe",
  "selection.selectWires": "xw",
  "selection.selectFaces": "xf",
  "selection.selectSolids": "xc",
  "selection.selectGeometries": "xg",
  "selection.selectObjects": "xo",
};

const SELECTION_ACTION_META: Readonly<Record<string, { readonly operation: SelectionApplyOperation; readonly kinds?: readonly ModelEntityKind[] }>> = {
  "selection.selectAll": { operation: "selectAll" },
  "selection.deselectAll": { operation: "deselectAll" },
  "selection.invert": { operation: "invert" },
  "selection.selectAnchors": { operation: "selectKinds", kinds: ["anchor"] },
  "selection.selectVertices": { operation: "selectKinds", kinds: ["vertex"] },
  "selection.selectEdges": { operation: "selectKinds", kinds: ["edge"] },
  "selection.selectWires": { operation: "selectKinds", kinds: ["wire"] },
  "selection.selectFaces": { operation: "selectKinds", kinds: ["face"] },
  "selection.selectSolids": { operation: "selectKinds", kinds: ["solid"] },
  "selection.selectGeometries": { operation: "selectKinds", kinds: ["geometry"] },
  "selection.selectObjects": { operation: "selectKinds", kinds: ["object"] },
};

function selectionOperationDefForActionId(actionId: string, label?: string): SelectionOperationInteractionDef | null {
  if (!actionId.startsWith("selection.") || actionId === "selection.apply") return null;
  const meta = SELECTION_ACTION_META[actionId];
  const key = SELECTION_INTERACTION_KEYS[actionId];
  if (!meta || !key) return null;
  return {
    id: actionId,
    label: label ?? actionId.slice("selection.".length),
    key,
    operation: meta.operation,
    ...(meta.kinds ? { kinds: [...meta.kinds] } : {}),
  };
}

export function selectionOperationsForModelDefinitionFromActions(modelDefinitionId: string): readonly SelectionOperationInteractionDef[] {
  const out: SelectionOperationInteractionDef[] = [];
  for (const actionId of listActionsForModelDefinition(modelDefinitionId)) {
    const defn = selectionOperationDefForActionId(actionId);
    if (defn) out.push(defn);
  }
  return out.sort((a, b) => a.id.localeCompare(b.id));
}

/** 🪪️ Maps a `selection.*` interaction id to headless `SelectionApplyParams`. */
export function selectionApplyParamsForInteraction(defn: SelectionOperationInteractionDef, seedTargets: readonly SelectionTarget[] = []): SelectionApplyParams {
  return {
    operation: defn.operation,
    seedTargets,
    ...(defn.kinds ? { kinds: defn.kinds } : {}),
  };
}

/** 🪪️ True when a selection command targets authored `object` rows on the model. */
export function selectionOperationUsesModelObjects(defn: Pick<SelectionOperationInteractionDef, "kinds">): boolean {
  return defn.kinds?.includes("object") ?? false;
}

/** 🪪️ Default seed targets for invert/deselectAll (otherwise empty). */
export function selectionSeedTargetsForOperation(operation: SelectionApplyOperation, seedCell: SelectionTarget = { kind: "solid", id: "e2e-box", editable: true }): readonly SelectionTarget[] {
  return operation === "invert" || operation === "deselectAll" ? [seedCell] : [];
}

function shippedInteractionJsons(): readonly ModelDefinitionInteractionRecord[] {
  return modelDefinitionInteractionCatalog() as readonly ModelDefinitionInteractionRecord[];
}

function interactionCatalogRow(spec: ModelDefinitionInteractionRecord): SpatialInteraction {
  return { id: spec.id, label: spec.label ?? spec.id, key: typeof spec.key === "string" ? spec.key : (spec.id[0] ?? "?") };
}

function shippedSpatialInteractionCatalog(): readonly SpatialInteraction[] {
  return shippedInteractionJsons().map(interactionCatalogRow);
}

/** 🧭️ Resolves a typed token to an interaction in one model definition (`key`, `id`, or compact `label`). */
export function resolveSpatialInteractionKeyForModelDefinition(modelDefinitionId: string, token: string): SpatialInteraction | null {
  const t = token.trim().toLowerCase();
  if (!t) return null;
  for (const p of listSpatialInteractionsForModelDefinition(modelDefinitionId)) {
    if (p.key.toLowerCase() === t) return p;
    if (p.id.toLowerCase() === t) return p;
    const slug = p.label.toLowerCase().replace(/\s+/g, "");
    if (slug === t) return p;
  }
  return null;
}

/** 🧭️ model-definition `InteractionSpec` map (fixtures + host `registerInteraction`). */
export type InteractionRegistry = ReadonlyMap<string, InteractionSpec>;

/** 🧭️ Registers one compiled interaction; returns a new registry (immutable update). */
export function registerInteractionSpec(registry: InteractionRegistry, spec: InteractionSpec): InteractionRegistry {
  const next = new Map(registry);
  next.set(spec.id, spec);
  return next;
}

/** 🧭️ Lists registered interaction specs in stable id order. */
export function listInteractionSpecs(registry: InteractionRegistry): readonly InteractionSpec[] {
  return [...registry.values()].sort((a, b) => a.id.localeCompare(b.id));
}

/** 🧭️ Shipped model-definition interactions compiled from fixtures. */
export function modelDefinitionInteractionRegistry(): InteractionRegistry {
  const map = new Map<string, InteractionSpec>();
  for (const raw of shippedInteractionJsons()) {
    const spec = parseInteractionSpec(raw);
    if (spec) map.set(spec.id, compileInteraction(spec));
  }
  return map;
}

const COMPILED_INTERACTION_BY_ID = ephemeralMap<string, InteractionSpec>("s.plugins.cad.modules.core.component.ts.COMPILED_INTERACTION_BY_ID");
/** 🧬️ Clears the derived interaction compile cache when model-definition assets change. */
onModelDefinitionAssetsChanged(() => COMPILED_INTERACTION_BY_ID.clear());

/** 📚️ Loads a model-definition interaction by stable `id` (compiled once per id for stable React runtime identity). */
export function loadSpatialInteraction(interactionId: string): InteractionSpec | null {
  const cached = COMPILED_INTERACTION_BY_ID.get(interactionId);
  if (cached) return cached;
  const raw = shippedInteractionJsons().find((spec) => spec.id === interactionId);
  const spec = raw ? parseInteractionSpec(raw) : null;
  if (!spec) return null;
  const compiled = compileInteraction(spec);
  COMPILED_INTERACTION_BY_ID.set(interactionId, compiled);
  return compiled;
}

export function requireSpatialInteraction(interactionId: string): InteractionSpec {
  const spec = loadSpatialInteraction(interactionId);
  if (!spec) throw new Error(`${interactionId} interaction missing from modelDefinition assets`);
  return spec;
}

/** 📦️ Compiled `primitive.box` interaction from model-definition assets. */
export function buildBoxInteractionSpec(): InteractionSpec {
  return requireSpatialInteraction("primitive.box");
}

/** 📦️ Compiled `feature.extrudeWire` interaction from model-definition assets. */
export function buildExtrudeInteractionSpec(): InteractionSpec {
  return requireSpatialInteraction("feature.extrudeWire");
}

/** 📦️ Compiled `feature.offsetSurface` interaction from model-definition assets. */
export function buildOffsetSurfaceInteractionSpec(): InteractionSpec {
  return requireSpatialInteraction("feature.offsetSurface");
}

/** 📦️ Compiled `measure.distance` interaction from model-definition assets. */
export function buildDistanceInteractionSpec(): InteractionSpec {
  return requireSpatialInteraction("measure.distance");
}

/** 📦️ Compiled `measure.area` interaction from model-definition assets. */
export function buildAreaInteractionSpec(): InteractionSpec {
  return requireSpatialInteraction("measure.area");
}
// #endregion 📦️Interactions
// #endregion 📦️📄️document
