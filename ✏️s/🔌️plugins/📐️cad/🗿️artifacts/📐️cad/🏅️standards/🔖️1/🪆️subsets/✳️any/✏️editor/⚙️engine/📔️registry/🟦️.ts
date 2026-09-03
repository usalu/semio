// #region 🧲️Header
/** @emoji 🧭️ `@semio-tech/cad-js` — CAD domain module facet. See `cad/AGENTS.md`. */
import { ephemeralBox, ephemeralMap, ephemeralWeakMap } from "@semio-tech/framework";
import type { ArcPlaneFrame, EdgeCurve, EdgeGroup, EdgeInfo, FaceGroup, FaceInfo, MeshTransfer, Vec3 } from "@semio-tech/s-3d-js";
import { emptyMeshTransfer, kernelGeometry, solidRef } from "@semio-tech/s-3d-js";
export type { ArcPlaneFrame, EdgeCurve, EdgeGroup, EdgeInfo, FaceGroup, FaceInfo, MeshTransfer, Vec3 };
export { emptyMeshTransfer, kernelGeometry, solidRef };
// #endregion 🧲️Header

import type { TypologyRef } from "../../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️.ts";
import { SemioBrepKernel, semioBrepKernel } from "../../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🟦️.ts";
import type { SpatialKernel } from "../../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/🗺️spatial/🟦️.ts";
export { SemioBrepKernel, semioBrepKernel };

// #region 🧠️DefaultSpatialKernel
/** @emoji 🧠️ THE production CAD `SpatialKernel` (ticket 26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME
 * W4-A) — the first-party Rust `BrepKernel` over `invokeBrep`/`flow_core` wasm. `🧱️brepjs`
 * (OpenCascade) is retained ONLY as the vitest differential oracle, never constructed here. */
export function defaultSpatialKernel(): SpatialKernel {
  return semioBrepKernel;
}
// #endregion 🧠️DefaultSpatialKernel

// #region 📦️📔️registry
// #region 📥️ModelDefinitionRegistry
/** @emoji 📥️ Registered model-definition asset modules (populated by `ModelDefinitionAssets` region). */
export interface ModelDefinitionAssetModules {
  readonly typologies: Readonly<Record<string, unknown>>;
  readonly actions: Readonly<Record<string, unknown>>;
  readonly interactions: Readonly<Record<string, unknown>>;
  readonly manifests: Readonly<Record<string, unknown>>;
  readonly extensions: Readonly<Record<string, unknown>>;
  readonly attributes: Readonly<Record<string, unknown>>;
  readonly propertyDefinitions: Readonly<Record<string, unknown>>;
  readonly properties: Readonly<Record<string, unknown>>;
  readonly statDefinitions: Readonly<Record<string, unknown>>;
  readonly transformations: Readonly<Record<string, unknown>>;
}

const emptyModelDefinitionAssetModules = (): ModelDefinitionAssetModules => ({
  typologies: {},
  actions: {},
  interactions: {},
  manifests: {},
  extensions: {},
  attributes: {},
  propertyDefinitions: {},
  properties: {},
  statDefinitions: {},
  transformations: {},
});

const modelDefinitionAssetModules = ephemeralBox<ModelDefinitionAssetModules>("s.plugins.cad.modules.core.component.ts.modelDefinitionAssetModules", emptyModelDefinitionAssetModules());

/** Derived indexes from {@link modelDefinitionAssetModules.current} — not authoritative document state; cleared on {@link registerModelDefinitionAssets}. */
const modelDefinitionFolderIdMapCache = ephemeralBox<ReadonlyMap<string, string> | null>("s.plugins.cad.modules.core.component.ts.modelDefinitionFolderIdMapCache", null);
const typologyOwnerByIdCache = ephemeralBox<ReadonlyMap<string, string> | null>("s.plugins.cad.modules.core.component.ts.typologyOwnerByIdCache", null);
const actionOwnerByIdCache = ephemeralBox<ReadonlyMap<string, string> | null>("s.plugins.cad.modules.core.component.ts.actionOwnerByIdCache", null);
const interactionOwnerByIdCache = ephemeralBox<ReadonlyMap<string, string> | null>("s.plugins.cad.modules.core.component.ts.interactionOwnerByIdCache", null);
const attributeOwnerByIdCache = ephemeralBox<ReadonlyMap<string, string> | null>("s.plugins.cad.modules.core.component.ts.attributeOwnerByIdCache", null);
const propertyOwnerByIdCache = ephemeralBox<ReadonlyMap<string, string> | null>("s.plugins.cad.modules.core.component.ts.propertyOwnerByIdCache", null);
const statOwnerByIdCache = ephemeralBox<ReadonlyMap<string, string> | null>("s.plugins.cad.modules.core.component.ts.statOwnerByIdCache", null);
const defaultModelDefinitionIdCache = ephemeralBox<string | null>("s.plugins.cad.modules.core.component.ts.defaultModelDefinitionIdCache", null);

function resetModelDefinitionCaches(): void {
  modelDefinitionFolderIdMapCache.current = null;
  typologyOwnerByIdCache.current = null;
  actionOwnerByIdCache.current = null;
  interactionOwnerByIdCache.current = null;
  attributeOwnerByIdCache.current = null;
  propertyOwnerByIdCache.current = null;
  statOwnerByIdCache.current = null;
  defaultModelDefinitionIdCache.current = null;
}

function mergeModelDefinitionAssetModules(base: ModelDefinitionAssetModules, patch: ModelDefinitionAssetModules): ModelDefinitionAssetModules {
  return {
    typologies: { ...base.typologies, ...patch.typologies },
    actions: { ...base.actions, ...patch.actions },
    interactions: { ...base.interactions, ...patch.interactions },
    manifests: { ...base.manifests, ...patch.manifests },
    extensions: { ...base.extensions, ...patch.extensions },
    attributes: { ...base.attributes, ...patch.attributes },
    propertyDefinitions: { ...base.propertyDefinitions, ...patch.propertyDefinitions },
    properties: { ...base.properties, ...patch.properties },
    statDefinitions: { ...base.statDefinitions, ...patch.statDefinitions },
    transformations: { ...base.transformations, ...patch.transformations },
  };
}

export const interactionCompileCacheClear = ephemeralBox<() => void>("s.plugins.cad.modules.core.component.ts.interactionCompileCacheClear", () => {});

/** @emoji 📥️ Merges model-definition asset catalogs (host or module injection). */
export function registerModelDefinitionAssets(modules: ModelDefinitionAssetModules): void {
  modelDefinitionAssetModules.current = mergeModelDefinitionAssetModules(modelDefinitionAssetModules.current, modules);
  resetModelDefinitionCaches();
  interactionCompileCacheClear.current();
}

function modelDefinitionTypologyCatalog(): readonly unknown[] {
  return Object.values(modelDefinitionAssetModules.current.typologies);
}

export function modelDefinitionActionCatalog(): readonly unknown[] {
  return Object.values(modelDefinitionAssetModules.current.actions);
}

export function modelDefinitionInteractionCatalog(): readonly unknown[] {
  return Object.values(modelDefinitionAssetModules.current.interactions);
}

function modelDefinitionManifestCatalog(): readonly unknown[] {
  return [...Object.values(modelDefinitionAssetModules.current.manifests), ...Object.values(modelDefinitionAssetModules.current.extensions)];
}

function modelDefinitionAttributeCatalog(): readonly unknown[] {
  return Object.values(modelDefinitionAssetModules.current.attributes);
}

function modelDefinitionPropertyCatalog(): readonly unknown[] {
  return [...Object.values(modelDefinitionAssetModules.current.propertyDefinitions), ...Object.values(modelDefinitionAssetModules.current.properties)];
}

function modelDefinitionStatCatalog(): readonly unknown[] {
  return Object.values(modelDefinitionAssetModules.current.statDefinitions);
}

function modelDefinitionTransformationModules(): Readonly<Record<string, unknown>> {
  return modelDefinitionAssetModules.current.transformations;
}
// #endregion 📥️ModelDefinitionRegistry

// #region 📥️ImportProfiles
/** @emoji 🪪️ STEP/BIM import profile for one model definition. */
export interface ModelImportProfile {
  readonly layerTypology: Readonly<Record<string, TypologyRef>>;
  readonly fallbackTypology: TypologyRef;
  readonly preferPresentationLayers?: boolean;
  readonly presentationGeometry?: "wireframe" | "solid";
  readonly namespacedDomain?: string;
}

const importProfiles = ephemeralMap<string, ModelImportProfile>("s.plugins.cad.modules.core.component.ts.importProfiles");

/** @emoji 📥️ Registers STEP layer → typology mapping for one model definition. */
export function registerImportProfile(modelDefinitionId: string, profile: ModelImportProfile): void {
  importProfiles.set(modelDefinitionId, profile);
}

/** @emoji 🧭️ Resolves the STEP import profile for one model definition. */
export function importProfileFor(modelDefinitionId: string): ModelImportProfile | null {
  return importProfiles.get(modelDefinitionId) ?? null;
}

/** @emoji 🏷️ Maps a STEP presentation-layer token to a typology via registered import profiles. */
export function typologyFromStepLayer(layerName: string, modelDefinitionId: string): TypologyRef {
  const trimmed = layerName.trim();
  const namespaced = trimmed.match(/^([^:]+)::(.+)$/i);
  if (namespaced) {
    const domain = namespaced[1]!.trim().toLowerCase();
    const part = namespaced[2]!.trim().toLowerCase();
    for (const profile of importProfiles.values()) {
      if (profile.namespacedDomain !== domain) continue;
      const mapped = resolveImportLayerTypology(profile.layerTypology, part);
      if (mapped) return mapped;
    }
  }
  const profile = importProfileFor(modelDefinitionId);
  if (!profile) return "" as TypologyRef;
  const mapped = resolveImportLayerTypology(profile.layerTypology, trimmed.toLowerCase());
  return mapped ?? profile.fallbackTypology;
}

function resolveImportLayerTypology(table: Readonly<Record<string, TypologyRef>>, key: string): TypologyRef | null {
  const direct = table[key];
  if (direct) return direct;
  if (key.endsWith("s")) {
    const singular = table[key.slice(0, -1)];
    if (singular) return singular;
  }
  const plural = table[`${key}s`];
  return plural ?? null;
}
// #endregion 📥️ImportProfiles

// #endregion 📦️📔️registry
