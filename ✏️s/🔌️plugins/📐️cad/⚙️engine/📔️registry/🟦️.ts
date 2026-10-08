// #region 🧲️Header
/** 🧭️ `@semio-tech/cad-js` — CAD domain module facet. See `cad/AGENTS.md`. */
import { ephemeralMap } from "@semio-tech/framework";
import type { ArcPlaneFrame, EdgeCurve, EdgeGroup, EdgeInfo, FaceGroup, FaceInfo, MeshTransfer, Vec3 } from "@semio-tech/framework-3d-js";
import { emptyMeshTransfer, kernelGeometry, solidRef } from "@semio-tech/framework-3d-js";
export type { ArcPlaneFrame, EdgeCurve, EdgeGroup, EdgeInfo, FaceGroup, FaceInfo, MeshTransfer, Vec3 };
export { emptyMeshTransfer, kernelGeometry, solidRef };
// #endregion 🧲️Header

import type { TypologyRef } from "../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️.ts";



// #region 📦️📔️registry
// #region 📥️ImportProfiles
/** 🪪️ STEP/BIM import profile for one model definition. */
export interface ModelImportProfile {
  readonly layerTypology: Readonly<Record<string, TypologyRef>>;
  readonly fallbackTypology: TypologyRef;
  readonly preferPresentationLayers?: boolean;
  readonly presentationGeometry?: "wireframe" | "solid";
  readonly namespacedDomain?: string;
}

const importProfiles = ephemeralMap<string, ModelImportProfile>("s.plugins.cad.modules.core.component.ts.importProfiles");

/** 📥️ Registers STEP layer → typology mapping for one model definition. */
export function registerImportProfile(modelDefinitionId: string, profile: ModelImportProfile): void {
  importProfiles.set(modelDefinitionId, profile);
}

/** 🧭️ Resolves the STEP import profile for one model definition. */
export function importProfileFor(modelDefinitionId: string): ModelImportProfile | null {
  return importProfiles.get(modelDefinitionId) ?? null;
}

/** 🏷️ Maps a STEP presentation-layer token to a typology via registered import profiles. */
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
