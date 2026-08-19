// #region 🧲️Header
/** @emoji 🧭️ `@semio-tech/cad-js` — CAD domain module facet. See `cad/AGENTS.md`. */
import { ephemeralBox, ephemeralMap, ephemeralWeakMap } from "@semio-tech/framework";
import type { ArcPlaneFrame, EdgeCurve, EdgeGroup, EdgeInfo, FaceGroup, FaceInfo, MeshTransfer, Vec3 } from "@semio-tech/s-3d-js";
import { emptyMeshTransfer, kernelGeometry, solidRef } from "@semio-tech/s-3d-js";
// #endregion 🧲️Header


import { Model, ObjectRef, TypologyPrimitiveKind, TypologyRef, TypologySpec, listModelDefinitionTypologies, listTypologiesForModelDefinition, loadTypology, typologyForInteraction } from "../../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️component.ts";
import type { ModelDiff } from "../../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/🗺️spatial/🟦️component.ts";
import type { ActionSpec } from "../🎬️actions/🟦️component.ts";
import { loadSpatialInteraction } from "../📄️artifact/🟦️component.ts";



// #region 📦️🧬️typology
// #region 🏗️TypologyConstruct
/** @emoji 🏷️ PascalCase object name from a typology label (`External Wall` → `ExternalWall`). */
export function typologyObjectPascalFromLabel(label: string): string {
  return label
    .replace(/[^a-zA-Z0-9]+/g, " ")
    .trim()
    .split(/\s+/)
    .filter(Boolean)
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join("");
}

export type TypologyConstructMode = "2PointsAndHeight" | "curveAndHeight" | "surface";

/** @emoji 🧭️ Per-typology construct kit: three mode actions + one interaction id. */
export type TypologyConstructKit = {
  readonly typology: string;
  readonly interaction: string;
  readonly constructFrom2PointsAndHeight: string;
  readonly constructFromCurveAndHeight: string;
  readonly constructFromSurface: string;
};

/** @emoji 🧭️ Stable ids: three `construct*From*` actions and one `construct*` interaction. */
export function typologyConstructAssetIds(typology: string, label: string): TypologyConstructKit & { readonly construct: string } {
  const parts = typology.split(".");
  const prefix = parts.length > 1 ? `${parts.slice(0, -1).join(".")}.` : "";
  const pascal = typologyObjectPascalFromLabel(label);
  const interaction = `${prefix}construct${pascal}`;
  return {
    typology,
    interaction,
    construct: interaction,
    constructFrom2PointsAndHeight: `${prefix}construct${pascal}From2PointsAndHeight`,
    constructFromCurveAndHeight: `${prefix}construct${pascal}FromCurveAndHeight`,
    constructFromSurface: `${prefix}construct${pascal}FromSurface`,
  };
}

/** @emoji 🏷️ True when typology construct exposes surface-only workflow (e.g. base plate). */
function typologyConstructIsSurfacePrimary(typologyId: string): boolean {
  return typologyId.endsWith(".baseplate");
}

/** @emoji 🧭️ `construct*` action ids declared on a typology (`surface`-primary typologies ship surface only). */
export function typologyConstructModeActionIds(typologyId: string, label: string): readonly string[] {
  const ids = typologyConstructAssetIds(typologyId, label);
  if (typologyConstructIsSurfacePrimary(typologyId)) return [ids.constructFromSurface];
  return [ids.constructFrom2PointsAndHeight, ids.constructFromCurveAndHeight, ids.constructFromSurface];
}

/** @emoji 🎯️ Resolves the single mode action an interaction commit must run for `constructMode`. */
export function typologyConstructCommitActionForMode(kit: TypologyConstructKit, mode: string): string {
  switch (mode as TypologyConstructMode) {
    case "2PointsAndHeight":
      return kit.constructFrom2PointsAndHeight;
    case "curveAndHeight":
      return kit.constructFromCurveAndHeight;
    case "surface":
      return kit.constructFromSurface;
    default:
      throw new Error(`Unknown constructMode ${mode} for ${kit.interaction}`);
  }
}

/** @emoji 📄️ Declarative capability action JSON for typology construction steps. */
export function capabilityActionSpecJson(id: string, label: string): ActionSpec {
  return {
    schema: "spatial.action",
    id,
    version: "1.0.0",
    label,
    steps: [
      { operation: "kernel.call", function: "spatial.action.capability", assignTo: "result" },
      { operation: "return", result: { kind: "var", name: "result" } },
    ],
  } as ActionSpec;
}

const typologyConstructKitByInteractionCache = ephemeralBox<ReadonlyMap<string, TypologyConstructKit> | null>("s.plugins.cad.modules.core.component.ts.typologyConstructKitByInteractionCache", null);

/** @emoji 🧭️ Maps each typology construct interaction id to its mode actions (not the interaction id). */
export function typologyConstructKitByInteraction(): ReadonlyMap<string, TypologyConstructKit> {
  if (typologyConstructKitByInteractionCache.current) return typologyConstructKitByInteractionCache.current;
  const map = new Map<string, TypologyConstructKit>();
  for (const typology of listModelDefinitionTypologies()) {
    const ids = typologyConstructAssetIds(typology.id, typology.label);
    map.set(ids.interaction, {
      typology: ids.typology,
      interaction: ids.interaction,
      constructFrom2PointsAndHeight: ids.constructFrom2PointsAndHeight,
      constructFromCurveAndHeight: ids.constructFromCurveAndHeight,
      constructFromSurface: ids.constructFromSurface,
    });
  }
  typologyConstructKitByInteractionCache.current = map;
  return map;
}

/** @emoji 🏗️ True when a typology ships exactly one construct interaction and its mode `construct*` actions. */
export function typologyHasNativeConstructKit(typology: TypologySpec): boolean {
  const ids = typologyConstructAssetIds(typology.id, typology.label);
  const expectedActions = [...typologyConstructModeActionIds(typology.id, typology.label)].sort();
  const actualActions = [...typology.actions].sort();
  return typology.interactions.length === 1 && typology.interactions[0] === ids.interaction && actualActions.join() === expectedActions.join();
}

/** @emoji 🏗️ Typologies in a model definition that expose the native construct interaction. */
export function listConstructableTypologiesForModelDefinition(modelDefinitionId: string): readonly TypologySpec[] {
  return listTypologiesForModelDefinition(modelDefinitionId).filter(typologyHasNativeConstructKit);
}

/** @emoji 🧭️ Typology id for an interaction commit (`construct` kit or typology `interactions` list). */
export function typologyIdForInteractionCommit(interactionId: string): string | null {
  const fromKit = typologyConstructKitByInteraction().get(interactionId)?.typology;
  if (fromKit) return fromKit;
  const fromTypology = typologyForInteraction(interactionId)?.id;
  if (fromTypology) return fromTypology;
  const produces = loadSpatialInteraction(interactionId)?.produces?.typology;
  return typeof produces === "string" && produces.length > 0 ? produces : null;
}

/** @emoji 📦️ Binds a typology object row to the primary primitive added by a create/construct diff. */
export function ensureTypologyObjectFromCreateDiff(model: Model, typology: string, diff: ModelDiff): ObjectRef | null {
  const typologySpec = loadTypology(typology);
  if (!typologySpec || typologySpec.primitiveKinds.length === 0) return null;
  const candidates: { readonly kind: TypologyPrimitiveKind; readonly id: string }[] = [];
  const solidId = diff.solids?.added?.[0]?.id;
  const faceId = diff.faces?.added?.[0]?.id;
  const wireId = diff.wires?.added?.[0]?.id;
  const edgeId = diff.edges?.added?.[0]?.id;
  if (solidId && typologySpec.primitiveKinds.includes("solid")) candidates.push({ kind: "solid", id: solidId });
  if (faceId && typologySpec.primitiveKinds.includes("surface")) candidates.push({ kind: "surface", id: faceId });
  if (wireId && typologySpec.primitiveKinds.includes("curve")) candidates.push({ kind: "curve", id: wireId });
  if (edgeId && typologySpec.primitiveKinds.includes("curve")) candidates.push({ kind: "curve", id: edgeId });
  const picked = candidates[0];
  if (!picked) return null;
  const typologyObjectId = typology as ObjectRef;
  const objectId = model.objects[typologyObjectId] ? (String(picked.id) as ObjectRef) : typologyObjectId;
  model.objects[objectId] = {
    id: objectId,
    typology: typology as TypologyRef,
    primitives: { [picked.kind]: String(picked.id) },
  };
  model.bump();
  return objectId;
}

// #endregion 🏗️TypologyConstruct

// #endregion 📦️🧬️typology
