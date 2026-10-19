/** 🧰️ The shared cad-js registries test kit: the destructured vitest, the cad-js core API, the Brepjs kernel and the precise spatial math. */
import { preciseSpatialKernelMath } from "@semio-tech/s-spatial-kernel-ts/preview";
import type { ArtifactTestDependencies } from "../../../../🎒️test-dependencies/🟦️.ts";

/** 🏭 Creates the shared registries test kit of one registration from the injected dependencies. */
export function createRegistriesTestKit(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: ArtifactTestDependencies) {
  const { DocumentHistory, EMPTY_MODEL_DIFF, Model, __artifactTestKernel, applyModelDiff, buildBoxInteractionSpec, compileInteraction, createInteractionRuntime, defaultModelDefinitionId, emptyMeshTransfer, interactionRecordsDocumentHistory, isEmptyModelDiff, listInteractionSpecs, loadSpatialInteraction, mergeInteractionCallOutputs, modelDefinitionActionRegistry, modelDefinitionInteractionRegistry, parseInteractionSpec, registerInteractionSpec, runRegisteredAction, runSelectionOperationInteraction, selectionApplyParamsForInteraction, selectionSeedTargetsForOperation, selectionTargetsFromActionResult, solidRef, typologyConstructCommitActionForMode, typologyConstructKitByInteraction, typologyIdForInteractionCommit } = dependencies;
  const { describe, expect, it } = vitest;
  const { BrepjsKernel } = __artifactTestKernel!;
  const M = preciseSpatialKernelMath;
  return { BrepjsKernel, DocumentHistory, EMPTY_MODEL_DIFF, M, Model, applyModelDiff, buildBoxInteractionSpec, compileInteraction, createInteractionRuntime, defaultModelDefinitionId, describe, emptyMeshTransfer, expect, interactionRecordsDocumentHistory, isEmptyModelDiff, it, listInteractionSpecs, loadSpatialInteraction, mergeInteractionCallOutputs, modelDefinitionActionRegistry, modelDefinitionInteractionRegistry, parseInteractionSpec, registerInteractionSpec, runRegisteredAction, runSelectionOperationInteraction, selectionApplyParamsForInteraction, selectionSeedTargetsForOperation, selectionTargetsFromActionResult, solidRef, typologyConstructCommitActionForMode, typologyConstructKitByInteraction, typologyIdForInteractionCommit };
}
export type RegistriesTestKit = ReturnType<typeof createRegistriesTestKit>;
