/// <reference types="vite/client" />
/// <reference types="vitest/importMeta" />
// @vitest-environment jsdom
// #region 🧲️Header
/** 🎬️ `@semio-tech/cad-js/renderer` — CAD renderer (R3F) with {@link InteractionRepl} host props/`on*` callbacks, {@link InteractionCanvas}, and {@link InteractionSpatialView}. See `cad/asset/modelDefinition/spatial.shape/interaction/📦️box.json`. */
// #endregion 🧲️Header

import { reactHostPort, uiDataLabel, WINDOW_SEARCH_USER } from "@semio-tech/framework/ui/targets/react";
import { emptyMeshTransfer, kernelGeometry, solidRef } from "@semio-tech/cad-engine-ts/registry";
import { resolveTypologyStyle, typologyStyleCacheKey } from "@semio-tech/s-spatial-kernel-ts/geometry/typology";
import { listStatDefinitionsForModelDefinition, statDefinitionAppliesToScope } from "@semio-tech/s-spatial-kernel-ts/geometry/stats";
import { defaultModelDefinitionId } from "@semio-tech/s-spatial-kernel-ts/geometry/definition-catalog";
import { Model } from "@semio-tech/s-spatial-kernel-ts/geometry/model";
import { ensureTypologyObjectFromCreateDiff } from "../🧬️typology/🟦️.ts";
import { applyModelDiff } from "@semio-tech/s-spatial-kernel-ts/spatial";
import { mergeInteractionSpatial } from "../🎬️actions/🎰️interaction-state/🟦️.ts";
import { loadSpatialInteraction } from "../🗿️artifact/📚️interaction-catalog/🟦️.ts";
import { preciseSpatialKernelMath } from "@semio-tech/s-spatial-kernel-ts/preview";
import { THREE } from "./🔌️ports/🟦️.ts";
import { resetSpatialSceneColorCache, spatialSceneColors } from "./🎨️style/🟦️.ts";
import { COMMITTED_MESH_FACE_OPACITY, filterFootprintBoxPreviewDisplayItems, historyEntryArchivesBoxFootprint, resolveCommittedMeshMaterialProps, TessellatedCommitMesh } from "./🧲️geometry-targets/🟦️.tsx";
import { defaultSpatialPrimitiveToggles, defaultSpatialTypologyTogglesForModelDefinition, filterSpatialPickTargetsForActiveView, filterSpatialPickTargetsForEntityFlags, filterSpatialPickTargetsForPrimitiveToggles, filterSpatialPickTargetsForTypologyToggles, modelDefinitionPickTargetKinds, modelDefinitionTypologyIds, pruneSelectionTargetsForEntityFlags, resolveSpatialSceneVisibility, spatialPickKindTogglesFromTypologyFilteredTargets, spatialPickTargetKey, spatialSceneKindTogglesForModelDefinition, spatialToggleCheckboxState, spatialToggleGroupFill, spatialToggleGroupState, spatialTypologyToggleLabel } from "./🧲️geometry-targets/🎯️pick-model/🟦️.ts";
import { buildGeometryObjectIndex, collectGeometryEdgeSegments, createSpatialPickEvent, createSpatialPickTargets, filterSpatialPickTargets, geometryBuckets, geometryEntityNurbsPoles, geometryEntityWireSegments, nurbsPolesFromEdge } from "./🧲️geometry-targets/🧬️geometry-index/🟦️.ts";
import { createTypologyStyledMaterial, pinnedPickTargetKeys, spatialHoverKeyAliases, spatialHoverKeysMatch, typologyStyleToMaterialProps } from "./🧲️geometry-targets/🎨️pick-styling/🟦️.ts";
import { buildPlanarFaceMeshTransfer, filterCommittedMeshesForModelDefinition, isRenderableMeshTransfer, listFactoryFaceMeshesForModelDefinition, resolveSpatialEntityFlags, visibleSolidRefsForModelDefinition } from "./🧲️geometry-targets/🧊️mesh-transfers/🟦️.ts";
import { PreviewItem } from "./🖼️display/🟦️.tsx";
import { defaultInteractionSpatialViewTheme, projectRayToVerticalZLine, projectRayToYzPlaneAtX, transformGumballMatrixDiff } from "./🔖️interaction/🟦️.tsx";
import {
  canvasHoverKeyForSelectionTarget,
  createSolidTypologyStyleResolver,
  replHostGeometryPickingEnabled,
  resolveSpatialPickTargetsToRender,
  revealedObjectIdsFromPickKeys,
  SpatialPickTargetNode,
  spatialSelectionTarget
} from "./🎯️pick-layer/🟦️.tsx";
import {
  applyCadWorldCoordinateSystem,
  CAD_WORLD_FORWARD,
  CAD_WORLD_UP,
  interactionSpatialGroundPickPlaneEnabled,
  spatialAutoFitShouldRun
} from "./🪩️canvas/🟦️.tsx";
import {
  buildInteractionReplEngagement,
  buildInteractionReplSearch,
  defaultInteractionReplChromeState,
  replDisplayedSelectionTargets,
  replFilterSuggestions,
  replInteractionIdOnSpace,
  replIsQueryTypingTarget,
  replMergeSelectionPickInView,
  replNormalizeActionText,
  replRendererSelectionTargets,
  replShouldRepeatInteractionOnSpace,
  replUserFacingSuggestionDetail
} from "./🧾️repl-model/🟦️.tsx";

export {
  bindScenePreviewKernel,
  r3fPreviewKernel,
  R3FPreviewKernel
} from "./🔌️ports/🟦️.ts";

export {
  resetSpatialSceneColorCache,
  type SpatialSceneColorPalette,
  spatialSceneColors
} from "./🎨️style/🟦️.ts";

export { type ArchivedBoxLayout, COMMITTED_MESH_FACE_OPACITY, CommittedMeshLayer, FactoryFaceSurfaceLayer, filterFootprintBoxPreviewDisplayItems, historyEntryArchivesBoxFootprint, modelHasCommittedSolidsForDisplay, modelHasFactoryFaceDisplay, resolveCommittedMeshMaterialProps, TessellatedCommitMesh, type TessellatedCommitMeshProps, tryArchivedBoxFromContext } from "./🧲️geometry-targets/🟦️.tsx";
export { bboxFromPoints, bboxWireSegments, computeBoxPreviewLayout, computeSpherePreviewLayout, geometryPreviewTransformFromDisplay, transformPointsForPreviewKind } from "./🧲️geometry-targets/📐️preview-layout/🟦️.ts";
export { defaultSpatialPickKindToggles, defaultSpatialPickKindTogglesForModelDefinition, defaultSpatialPrimitiveToggles, defaultSpatialTypologyTogglesForModelDefinition, filterSpatialPickTargetsForActiveView, filterSpatialPickTargetsForEntityFlags, filterSpatialPickTargetsForPrimitiveToggles, filterSpatialPickTargetsForTypologyToggles, filterSpatialPickTargetsForVisibility, intersectSpatialPickKindToggles, modelDefinitionPickTargetKinds, modelDefinitionTypologyIds, pickTargetPrimitiveKind, pruneSelectionTargetsForEntityFlags, resolveSpatialSceneVisibility, selectionTargetHoverKey, SPATIAL_PICK_TARGET_KINDS, SPATIAL_PRIMITIVE_KINDS, type SpatialDragSelectionState, spatialEntityFlagsForModelEntity, type SpatialGeometryPickTargetKind, type SpatialPickGeometry, type SpatialPickKind, type SpatialPickKindToggles, spatialPickKindTogglesFromTypologyFilteredTargets, type SpatialPickTarget, spatialPickTargetKey, type SpatialPickTargetKind, type SpatialPrimitiveToggles, spatialSceneKindTogglesForModelDefinition, type SpatialSelectionMethod, type SpatialSelectionRequest, spatialToggleCheckboxState, spatialToggleGroupFill, spatialToggleGroupState, type SpatialToggleGroupState, spatialTypologyToggleLabel, type SpatialTypologyToggles } from "./🧲️geometry-targets/🎯️pick-model/🟦️.ts";
export { boundsFromSpatialPickGeometry, buildGeometryObjectIndex, buildGeometryTypologyIndex, collectGeometryEdgeSegments, collectGeometryEdgeSegmentsForMembers, createSpatialPickEvent, createSpatialPickTargets, filterSpatialPickTargets, geometryEntityNurbsPoles, geometryEntityWireSegments, nurbsPolesFromEdge, polylineWireSegments } from "./🧲️geometry-targets/🧬️geometry-index/🟦️.ts";
export { createTypologyStyledMaterial, spatialHoverKeyAliases, spatialHoverKeysMatch, typologyStyleToMaterialProps } from "./🧲️geometry-targets/🎨️pick-styling/🟦️.ts";
export { boundsFromMeshTransfers, buildBufferGeometryFromMeshTransfer, buildPlanarFaceMeshTransfer, type CommittedMeshVisibilityOptions, type FactoryFaceMeshRow, filterCommittedMeshesForModelDefinition, findFaceGroupAt, isRenderableMeshTransfer, listFactoryFaceMeshesForModelDefinition, listModelSolidRefs, meshTransferContentKey, meshTransferOrigin, resolveFaceInfoFromTriangleIndex, resolveSpatialEntityFlags, useDocumentMeshes, useTessellation, visibleFaceRefsForModelDefinition, visibleSolidRefsForModelDefinition } from "./🧲️geometry-targets/🧊️mesh-transfers/🟦️.ts";

export {
  getSpatialDisplayItemKindRenderer,
  InteractionDisplay,
  registerSpatialDisplayItemKind,
  type SpatialDisplayItemRenderer
} from "./🖼️display/🟦️.tsx";

export {
  createR3FInteractionAdapter,
  defaultInteractionSpatialViewTheme,
  GroundPickPlane,
  type GroundPickPlaneProps,
  type GumballMatrixSnapshot,
  type InteractionSpatialViewSlots,
  type InteractionSpatialViewTheme,
  projectRayToVerticalZLine,
  projectRayToYzPlaneAtX,
  SpatialTransformGumball,
  transformGumballMatrixDiff
} from "./🔖️interaction/🟦️.tsx";

export {
  canvasHoverKeyForSelectionTarget,
  createSolidTypologyStyleResolver,
  pickHoverTargetFromRay,
  replGeometryPickLayerVisible,
  replHostGeometryPickingEnabled,
  resolveSpatialPickTargetsToRender,
  revealedObjectIdsFromPickKeys,
  SpatialPickGeometryLayer,
  spatialPickTargetMemberKey
} from "./🎯️pick-layer/🟦️.tsx";

export {
  applyCadWorldCoordinateSystem,
  applySpatialAutoFitCamera,
  CAD_WORLD_FORWARD,
  CAD_WORLD_UP,
  InteractionCanvas,
  type InteractionCanvasHostCallbacks,
  type InteractionCanvasProps,
  interactionSpatialGroundPickPlaneEnabled,
  InteractionSpatialView,
  type InteractionSpatialViewHostCallbacks,
  type InteractionSpatialViewProps,
  resolveHostStateNext,
  SpatialAutoFit,
  type SpatialAutoFitBehavior,
  spatialAutoFitShouldRun,
  useHostState,
  useInteractionRuntime,
  useInteractionSnapshot
} from "./🪩️canvas/🟦️.tsx";

export {
  buildEngagementControlFromResolved,
  buildInteractionReplEngagement,
  buildInteractionReplSearch,
  defaultInteractionReplChromeState,
  getReplHistoryPresentation,
  type InteractionReplEngagementInputs,
  type InteractionReplEngagementInteraction,
  type InteractionReplHostCallbacks,
  type InteractionReplHostValues,
  type InteractionReplLayoutProps,
  type InteractionReplProps,
  replActiveCompletionSuffix,
  replCompletionSuffix,
  replDisplayedSelectionTargets,
  replFilterSuggestions,
  replFinalizeSelection,
  replInteractionSelectionFromContext,
  replInteractionSelectionTargets,
  replIsQueryTypingTarget,
  replLengthEntryLiveValue,
  replMergeSelectionPickInView,
  replNormalizeActionText,
  replPaletteRows,
  replPruneSelectionByKind,
  replPruneSelectionByPrimitive,
  replPruneSelectionByTypology,
  replRendererSelectionTargets,
  replSelectionTargetsEqual,
  replUserFacingSuggestionDetail,
  replWithInteractionSelectionTargets,
  replWithRendererSelectionTargets,
  type SpatialInteractionSelectionByState,
  type SpatialRendererSelectionByModel,
  useDocumentHistory,
  useReplHistoryState
} from "./🧾️repl-model/🟦️.tsx";

export {
  ModelStatsPane,
  type ModelStatsPaneProps,
  SelectionAttributesPane,
  type SelectionAttributesPaneProps,
  SelectionPropertiesPane,
  type SelectionPropertiesPaneProps
} from "./📋️repl-panes/🟦️.tsx";

export {
  InteractionRepl,
  InteractionReplViewport
} from "./🎛️repl/🟦️.tsx";

export type { SpatialComputeMode } from "../🗿️artifact/📨️interaction-model/🟦️.ts";

// #region 🧪️Tests
const __cadRendererTestRuntime = import.meta.vitest ? await import("../🏃️runtime/🟦️.ts") : null;
const __cadRendererTestKernel = import.meta.vitest ? await import("@semio-tech/cad-js/testing/spatial-kernel/brepjs") : null;

/** 🎒️ The values this module hands its extracted suite `./🧪️tests/🧪️repluserfacingsuggestiondetail/🟦️.tsx`. */
export type RendererTestDependencies = {
  readonly CAD_WORLD_FORWARD: typeof CAD_WORLD_FORWARD;
  readonly CAD_WORLD_UP: typeof CAD_WORLD_UP;
  readonly COMMITTED_MESH_FACE_OPACITY: typeof COMMITTED_MESH_FACE_OPACITY;
  readonly Model: typeof Model;
  readonly PreviewItem: typeof PreviewItem;
  readonly SpatialPickTargetNode: typeof SpatialPickTargetNode;
  readonly THREE: typeof THREE;
  readonly TessellatedCommitMesh: typeof TessellatedCommitMesh;
  readonly WINDOW_SEARCH_USER: typeof WINDOW_SEARCH_USER;
  readonly __cadRendererTestKernel: typeof __cadRendererTestKernel;
  readonly __cadRendererTestRuntime: typeof __cadRendererTestRuntime;
  readonly applyCadWorldCoordinateSystem: typeof applyCadWorldCoordinateSystem;
  readonly applyModelDiff: typeof applyModelDiff;
  readonly buildGeometryObjectIndex: typeof buildGeometryObjectIndex;
  readonly buildInteractionReplEngagement: typeof buildInteractionReplEngagement;
  readonly buildInteractionReplSearch: typeof buildInteractionReplSearch;
  readonly buildPlanarFaceMeshTransfer: typeof buildPlanarFaceMeshTransfer;
  readonly canvasHoverKeyForSelectionTarget: typeof canvasHoverKeyForSelectionTarget;
  readonly collectGeometryEdgeSegments: typeof collectGeometryEdgeSegments;
  readonly createSolidTypologyStyleResolver: typeof createSolidTypologyStyleResolver;
  readonly createSpatialPickEvent: typeof createSpatialPickEvent;
  readonly createSpatialPickTargets: typeof createSpatialPickTargets;
  readonly createTypologyStyledMaterial: typeof createTypologyStyledMaterial;
  readonly defaultInteractionReplChromeState: typeof defaultInteractionReplChromeState;
  readonly defaultInteractionSpatialViewTheme: typeof defaultInteractionSpatialViewTheme;
  readonly defaultModelDefinitionId: typeof defaultModelDefinitionId;
  readonly defaultSpatialPrimitiveToggles: typeof defaultSpatialPrimitiveToggles;
  readonly defaultSpatialTypologyTogglesForModelDefinition: typeof defaultSpatialTypologyTogglesForModelDefinition;
  readonly emptyMeshTransfer: typeof emptyMeshTransfer;
  readonly ensureTypologyObjectFromCreateDiff: typeof ensureTypologyObjectFromCreateDiff;
  readonly filterCommittedMeshesForModelDefinition: typeof filterCommittedMeshesForModelDefinition;
  readonly filterFootprintBoxPreviewDisplayItems: typeof filterFootprintBoxPreviewDisplayItems;
  readonly filterSpatialPickTargets: typeof filterSpatialPickTargets;
  readonly filterSpatialPickTargetsForActiveView: typeof filterSpatialPickTargetsForActiveView;
  readonly filterSpatialPickTargetsForEntityFlags: typeof filterSpatialPickTargetsForEntityFlags;
  readonly filterSpatialPickTargetsForPrimitiveToggles: typeof filterSpatialPickTargetsForPrimitiveToggles;
  readonly filterSpatialPickTargetsForTypologyToggles: typeof filterSpatialPickTargetsForTypologyToggles;
  readonly geometryBuckets: typeof geometryBuckets;
  readonly geometryEntityNurbsPoles: typeof geometryEntityNurbsPoles;
  readonly geometryEntityWireSegments: typeof geometryEntityWireSegments;
  readonly historyEntryArchivesBoxFootprint: typeof historyEntryArchivesBoxFootprint;
  readonly interactionSpatialGroundPickPlaneEnabled: typeof interactionSpatialGroundPickPlaneEnabled;
  readonly isRenderableMeshTransfer: typeof isRenderableMeshTransfer;
  readonly kernelGeometry: typeof kernelGeometry;
  readonly listFactoryFaceMeshesForModelDefinition: typeof listFactoryFaceMeshesForModelDefinition;
  readonly listStatDefinitionsForModelDefinition: typeof listStatDefinitionsForModelDefinition;
  readonly loadSpatialInteraction: typeof loadSpatialInteraction;
  readonly mergeInteractionSpatial: typeof mergeInteractionSpatial;
  readonly modelDefinitionPickTargetKinds: typeof modelDefinitionPickTargetKinds;
  readonly modelDefinitionTypologyIds: typeof modelDefinitionTypologyIds;
  readonly nurbsPolesFromEdge: typeof nurbsPolesFromEdge;
  readonly pinnedPickTargetKeys: typeof pinnedPickTargetKeys;
  readonly preciseSpatialKernelMath: typeof preciseSpatialKernelMath;
  readonly projectRayToVerticalZLine: typeof projectRayToVerticalZLine;
  readonly projectRayToYzPlaneAtX: typeof projectRayToYzPlaneAtX;
  readonly pruneSelectionTargetsForEntityFlags: typeof pruneSelectionTargetsForEntityFlags;
  readonly reactHostPort: typeof reactHostPort;
  readonly replDisplayedSelectionTargets: typeof replDisplayedSelectionTargets;
  readonly replFilterSuggestions: typeof replFilterSuggestions;
  readonly replHostGeometryPickingEnabled: typeof replHostGeometryPickingEnabled;
  readonly replInteractionIdOnSpace: typeof replInteractionIdOnSpace;
  readonly replIsQueryTypingTarget: typeof replIsQueryTypingTarget;
  readonly replMergeSelectionPickInView: typeof replMergeSelectionPickInView;
  readonly replNormalizeActionText: typeof replNormalizeActionText;
  readonly replRendererSelectionTargets: typeof replRendererSelectionTargets;
  readonly replShouldRepeatInteractionOnSpace: typeof replShouldRepeatInteractionOnSpace;
  readonly replUserFacingSuggestionDetail: typeof replUserFacingSuggestionDetail;
  readonly resetSpatialSceneColorCache: typeof resetSpatialSceneColorCache;
  readonly resolveCommittedMeshMaterialProps: typeof resolveCommittedMeshMaterialProps;
  readonly resolveSpatialEntityFlags: typeof resolveSpatialEntityFlags;
  readonly resolveSpatialPickTargetsToRender: typeof resolveSpatialPickTargetsToRender;
  readonly resolveSpatialSceneVisibility: typeof resolveSpatialSceneVisibility;
  readonly resolveTypologyStyle: typeof resolveTypologyStyle;
  readonly revealedObjectIdsFromPickKeys: typeof revealedObjectIdsFromPickKeys;
  readonly solidRef: typeof solidRef;
  readonly spatialAutoFitShouldRun: typeof spatialAutoFitShouldRun;
  readonly spatialHoverKeyAliases: typeof spatialHoverKeyAliases;
  readonly spatialHoverKeysMatch: typeof spatialHoverKeysMatch;
  readonly spatialPickKindTogglesFromTypologyFilteredTargets: typeof spatialPickKindTogglesFromTypologyFilteredTargets;
  readonly spatialPickTargetKey: typeof spatialPickTargetKey;
  readonly spatialSceneColors: typeof spatialSceneColors;
  readonly spatialSceneKindTogglesForModelDefinition: typeof spatialSceneKindTogglesForModelDefinition;
  readonly spatialSelectionTarget: typeof spatialSelectionTarget;
  readonly spatialToggleCheckboxState: typeof spatialToggleCheckboxState;
  readonly spatialToggleGroupFill: typeof spatialToggleGroupFill;
  readonly spatialToggleGroupState: typeof spatialToggleGroupState;
  readonly spatialTypologyToggleLabel: typeof spatialTypologyToggleLabel;
  readonly statDefinitionAppliesToScope: typeof statDefinitionAppliesToScope;
  readonly transformGumballMatrixDiff: typeof transformGumballMatrixDiff;
  readonly typologyStyleCacheKey: typeof typologyStyleCacheKey;
  readonly typologyStyleToMaterialProps: typeof typologyStyleToMaterialProps;
  readonly uiDataLabel: typeof uiDataLabel;
  readonly visibleSolidRefsForModelDefinition: typeof visibleSolidRefsForModelDefinition;
};

if (import.meta.vitest) {
  const testDependencies: RendererTestDependencies = { CAD_WORLD_FORWARD, CAD_WORLD_UP, COMMITTED_MESH_FACE_OPACITY, Model, PreviewItem, SpatialPickTargetNode, THREE, TessellatedCommitMesh, WINDOW_SEARCH_USER, __cadRendererTestKernel, __cadRendererTestRuntime, applyCadWorldCoordinateSystem, applyModelDiff, buildGeometryObjectIndex, buildInteractionReplEngagement, buildInteractionReplSearch, buildPlanarFaceMeshTransfer, canvasHoverKeyForSelectionTarget, collectGeometryEdgeSegments, createSolidTypologyStyleResolver, createSpatialPickEvent, createSpatialPickTargets, createTypologyStyledMaterial, defaultInteractionReplChromeState, defaultInteractionSpatialViewTheme, defaultModelDefinitionId, defaultSpatialPrimitiveToggles, defaultSpatialTypologyTogglesForModelDefinition, emptyMeshTransfer, ensureTypologyObjectFromCreateDiff, filterCommittedMeshesForModelDefinition, filterFootprintBoxPreviewDisplayItems, filterSpatialPickTargets, filterSpatialPickTargetsForActiveView, filterSpatialPickTargetsForEntityFlags, filterSpatialPickTargetsForPrimitiveToggles, filterSpatialPickTargetsForTypologyToggles, geometryBuckets, geometryEntityNurbsPoles, geometryEntityWireSegments, historyEntryArchivesBoxFootprint, interactionSpatialGroundPickPlaneEnabled, isRenderableMeshTransfer, kernelGeometry, listFactoryFaceMeshesForModelDefinition, listStatDefinitionsForModelDefinition, loadSpatialInteraction, mergeInteractionSpatial, modelDefinitionPickTargetKinds, modelDefinitionTypologyIds, nurbsPolesFromEdge, pinnedPickTargetKeys, preciseSpatialKernelMath, projectRayToVerticalZLine, projectRayToYzPlaneAtX, pruneSelectionTargetsForEntityFlags, reactHostPort, replDisplayedSelectionTargets, replFilterSuggestions, replHostGeometryPickingEnabled, replInteractionIdOnSpace, replIsQueryTypingTarget, replMergeSelectionPickInView, replNormalizeActionText, replRendererSelectionTargets, replShouldRepeatInteractionOnSpace, replUserFacingSuggestionDetail, resetSpatialSceneColorCache, resolveCommittedMeshMaterialProps, resolveSpatialEntityFlags, resolveSpatialPickTargetsToRender, resolveSpatialSceneVisibility, resolveTypologyStyle, revealedObjectIdsFromPickKeys, solidRef, spatialAutoFitShouldRun, spatialHoverKeyAliases, spatialHoverKeysMatch, spatialPickKindTogglesFromTypologyFilteredTargets, spatialPickTargetKey, spatialSceneColors, spatialSceneKindTogglesForModelDefinition, spatialSelectionTarget, spatialToggleCheckboxState, spatialToggleGroupFill, spatialToggleGroupState, spatialTypologyToggleLabel, statDefinitionAppliesToScope, transformGumballMatrixDiff, typologyStyleCacheKey, typologyStyleToMaterialProps, uiDataLabel, visibleSolidRefsForModelDefinition };
  const testSource = { url: import.meta.url };
  const { registerTests1 } = await import("./🧪️tests/🧪️repluserfacingsuggestiondetail/🟦️.tsx");
  await registerTests1(import.meta.vitest, testDependencies, testSource);
  const { registerTests2 } = await import("./🧪️tests/🪝️hookorder/🟦️.tsx");
  await registerTests2(import.meta.vitest, testDependencies, testSource);
}
// #endregion 🧪️Tests
