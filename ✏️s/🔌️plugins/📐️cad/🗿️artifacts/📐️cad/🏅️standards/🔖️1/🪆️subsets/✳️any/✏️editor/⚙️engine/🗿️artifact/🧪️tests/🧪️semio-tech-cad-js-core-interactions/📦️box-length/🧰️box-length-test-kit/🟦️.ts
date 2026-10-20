/** 🧰️ The shared box and length entry test kit: the destructured vitest, the cad-js core API, the Brepjs kernel and the precise spatial math. */
import { preciseSpatialKernelMath } from "@semio-tech/s-spatial-kernel-ts/preview";
import type { ArtifactTestDependencies } from "../../../../🎒️test-dependencies/🟦️.ts";

/** 🏭 Creates the shared box and length entry test kit of one registration from the injected dependencies. */
export function createBoxLengthTestKit(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: ArtifactTestDependencies) {
  const { CAD_E2E_ROUTES_MODEL_SPACE_JSON, EMPTY_MODEL_DIFF, Model, __artifactTestKernel, applyModelDiff, buildBoxInteractionSpec, clampPointAlongDirection, createInteractionRuntime, defaultModelDefinitionId, emptyMeshTransfer, ensureTypologyObjectFromCreateDiff, interactionLengthEntryForState, loadSpatialInteraction, pureTsStateEngineProvider, readInteractionContextVec3, requireSpatialInteraction, resolveDisplay, solidRef } = dependencies;
  const { describe, expect, it } = vitest;
  const { BrepjsKernel } = __artifactTestKernel!;
  const M = preciseSpatialKernelMath;
  return { BrepjsKernel, CAD_E2E_ROUTES_MODEL_SPACE_JSON, EMPTY_MODEL_DIFF, M, Model, applyModelDiff, buildBoxInteractionSpec, clampPointAlongDirection, createInteractionRuntime, defaultModelDefinitionId, describe, emptyMeshTransfer, ensureTypologyObjectFromCreateDiff, expect, interactionLengthEntryForState, it, loadSpatialInteraction, pureTsStateEngineProvider, readInteractionContextVec3, requireSpatialInteraction, resolveDisplay, solidRef };
}
export type BoxLengthTestKit = ReturnType<typeof createBoxLengthTestKit>;
