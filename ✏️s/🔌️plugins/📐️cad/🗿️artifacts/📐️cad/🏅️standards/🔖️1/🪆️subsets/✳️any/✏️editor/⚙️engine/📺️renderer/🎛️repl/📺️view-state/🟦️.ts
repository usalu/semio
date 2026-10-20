/** 📺️ CAD renderer REPL: the host-controlled chrome state, the projection and gumball preview state, the refs, the derived pick targets and the scene and selection toggles of the REPL view. */
import { reactHostPort, type ThreeCamera } from "@semio-tech/ui-react";
import { worldProjectionFamily, type WorldProjectionSpec } from "../../../../../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/♾️infinite/🌍️world/🎨️r3f/🟦️.ts";
import { defaultModelDefinitionId, listModelDefinitionManifests } from "@semio-tech/s-spatial-kernel-ts/geometry/definition-catalog";
import { listTransformationsFromModelDefinition, listTransformationsIntoModelDefinition } from "@semio-tech/s-spatial-kernel-ts/geometry/transformation-spec";
import { Model } from "@semio-tech/s-spatial-kernel-ts/geometry/model";
import { listSpatialInteractionsForModelDefinition, modelDefinitionUsesGeometryPicking } from "@semio-tech/s-spatial-kernel-ts/geometry/definition-ownership";
import { resolveModelDefinitionScope } from "@semio-tech/s-spatial-kernel-ts/geometry/selection-operations";
import { applyModelDiff, isEmptyModelDiff, type ModelDiff } from "@semio-tech/s-spatial-kernel-ts/spatial";
import { cadGumballConfigVisible } from "../../../🎬️actions/🧲️selection-targets/🟦️.ts";
import { isInteractionSessionActive } from "../../../🗿️artifact/🏃️interaction-runtime/🟦️.ts";
import { archivedBoxesFromHistory, filterFootprintBoxPreviewDisplayItems, mergeDisplayWithArchivedBoxes, replBaseDisplayForHistory } from "../../🧲️geometry-targets/🟦️.tsx";
import { geometryPreviewTransformFromDisplay } from "../../🧲️geometry-targets/📐️preview-layout/🟦️.ts";
import { filterSpatialPickTargetsForActiveView, filterSpatialPickTargetsForEntityFlags, filterSpatialPickTargetsForPrimitiveToggles, filterSpatialPickTargetsForTypologyToggles, intersectSpatialPickKindToggles, modelDefinitionTypologyIds, SPATIAL_PRIMITIVE_KINDS, type SpatialDragSelectionState, spatialPickKindTogglesFromTypologyFilteredTargets, spatialSceneKindTogglesForModelDefinition, spatialSelectionTargetKey, spatialToggleGroupState } from "../../🧲️geometry-targets/🎯️pick-model/🟦️.ts";
import { createSpatialPickTargets } from "../../🧲️geometry-targets/🧬️geometry-index/🟦️.ts";
import { pinnedPickTargetKeys } from "../../🧲️geometry-targets/🎨️pick-styling/🟦️.ts";
import { filterCommittedMeshesForModelDefinition, listFactoryFaceMeshesForModelDefinition, resolveSpatialEntityFlags, useDocumentMeshes } from "../../🧲️geometry-targets/🧊️mesh-transfers/🟦️.ts";
import { useHostState, useInteractionSnapshot } from "../../🪩️canvas/🟦️.tsx";
import { defaultInteractionReplChromeState, type InteractionReplProps, replDisplayedSelectionTargets } from "../../🧾️repl-model/🟦️.tsx";

// #region 🪩️Repl
/** 📺️ Holds the REPL view state: props, host state, refs, pick targets and scene toggles. */
export function useReplViewState({ interactionId, spec, onInteractionId, runtime: rt, history, document: documentModel, geometry, asideExtra, archivedBoxLayouts = [], sessionRestartNonce = 0, viewTheme, viewSlots, renderDisplayItem, autoFitMeshes = false, autoFitBehavior = "initial", committedMeshesKeepPrevious = false, tessellationTolerance, cmdLine: cmdLineProp, activeSuggestionIndex: activeSuggestionIndexProp, filterTypologyToggles: filterTypologyTogglesProp, selectionTypologyToggles: selectionTypologyTogglesProp, filterPrimitiveToggles: filterPrimitiveTogglesProp, selectionPrimitiveToggles: selectionPrimitiveTogglesProp, activeModelDefinitionId: activeModelDefinitionIdProp, selectionMethod: selectionMethodProp, modelDefinitionRevision: modelDefinitionRevisionProp, dragSelection: dragSelectionProp, selectionMenu: selectionMenuProp, hoveredPickKey: hoveredPickKeyProp, rendererSelectionByModel: rendererSelectionByModelProp, interactionSelectionByState: interactionSelectionByStateProp, interactionMenuOpen: interactionMenuOpenProp, lastFinalizedInteractionId: lastFinalizedInteractionIdProp, onCmdLineChange, onActiveSuggestionIndexChange, onFilterTypologyTogglesChange, onSelectionTypologyTogglesChange, onFilterPrimitiveTogglesChange, onSelectionPrimitiveTogglesChange, onActiveModelDefinitionIdChange, onSelectionMethodChange, onModelDefinitionRevisionChange, onDragSelectionChange, onSelectionMenuChange, onHoveredPickKeyChange, onRendererSelectionByModelChange, onInteractionSelectionByStateChange, onInteractionMenuOpenChange, onLastFinalizedInteractionIdChange, onApplyTransformation, pickGeometry: pickGeometryProp, onDocumentModelChange, onCanvasReady, onInteractionEvent: onInteractionEventProp, onGroundPick: onGroundPickProp, onScenePointerMove: onScenePointerMoveProp, onSelectionRequest: onSelectionRequestProp, onHoverTarget: onHoverTargetProp, onCameraNavigate: onCameraNavigateProp, onActionSubmit, onTransitionRun, onCancel, onUndo, onRedo, onSnapshotChange, onEscape, onDeleteSelection, rootStyle, asideStyle, showAside = true, showEngagement = false, onEngagementChange, onSearchChange, fillHost = false, captureGlobalKeys = true, hideModelDefinitionControls = false, frameloop, canvas: canvasOverrides, spatialView: spatialViewOverrides, transformGumballConfig = null, onTransformGumballCommit, worldReferences = [], selectedReferenceIds, hoveredReferenceId = null, revealedReferenceIds, referenceRelocateActive = true, onReferenceSelect, onReferenceHover, onReferenceRelocate }: InteractionReplProps) {
  const engagementActionMode = !showAside && showEngagement;
  const snapshot = useInteractionSnapshot(rt);
  const rtRef = reactHostPort.useRef(rt);
  rtRef.current = rt;
  const [gumballPreviewDiff, setGumballPreviewDiff] = reactHostPort.useState<ModelDiff | null>(null);
  const [projectionSpec, setProjectionSpec] = reactHostPort.useState<WorldProjectionSpec>(
    () => spatialViewOverrides?.projectionSpec ?? (spatialViewOverrides?.orbitProjection === "orthographic" ? { mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "plan" } } : { mode: { kind: "threePoint", fov: 50 }, orientation: { type: "free" } }),
  );
  const [externalPendingProjectionSpec, setExternalPendingProjectionSpec] = reactHostPort.useState<WorldProjectionSpec | null>(null);
  reactHostPort.useEffect(() => {
    if (spatialViewOverrides?.projectionSpec) {
      setProjectionSpec(spatialViewOverrides.projectionSpec);
    }
  }, [spatialViewOverrides?.projectionSpec]);
  const handleProjectionSpecChange = reactHostPort.useCallback(
    (spec: WorldProjectionSpec) => {
      setProjectionSpec(spec);
      setExternalPendingProjectionSpec(spec);
      spatialViewOverrides?.onProjectionSpecChange?.(spec);
      const family = worldProjectionFamily(spec);
      spatialViewOverrides?.onOrbitProjectionChange?.(family === "parallel" ? "orthographic" : "perspective");
    },
    [spatialViewOverrides],
  );
  const tessTolerance = tessellationTolerance ?? (rt.computeMode() === "fast" ? 0.02 : 0.0008);
  const gumballPreviewActive = gumballPreviewDiff !== null && !isEmptyModelDiff(gumballPreviewDiff);
  const gumballPreviewModel = reactHostPort.useMemo(() => {
    if (!gumballPreviewActive) return documentModel.model;
    const copy = Model.fromJSON(documentModel.model.toJSON());
    applyModelDiff(copy, gumballPreviewDiff!);
    return copy;
  }, [documentModel.model, gumballPreviewActive, gumballPreviewDiff]);
  const viewGeometry = gumballPreviewActive ? gumballPreviewModel : geometry;
  const committedMeshes = useDocumentMeshes(rt.kernel(), gumballPreviewModel, tessTolerance, gumballPreviewActive || committedMeshesKeepPrevious);
  const handleTransformGumballPreview = reactHostPort.useCallback((diff: ModelDiff) => {
    setGumballPreviewDiff(isEmptyModelDiff(diff) ? null : diff);
  }, []);
  const handleTransformGumballPreviewEnd = reactHostPort.useCallback(() => {
    setGumballPreviewDiff(null);
  }, []);
  const handleTransformGumballCommit = reactHostPort.useCallback(
    (diff: ModelDiff) => {
      setGumballPreviewDiff(null);
      onTransformGumballCommit?.(diff);
    },
    [onTransformGumballCommit],
  );
  const documentArchivedBoxLayouts = reactHostPort.useMemo(() => archivedBoxesFromHistory(history), [history, snapshot.revision]);
  const allArchivedBoxLayouts = reactHostPort.useMemo(() => [...documentArchivedBoxLayouts, ...archivedBoxLayouts], [documentArchivedBoxLayouts, archivedBoxLayouts]);
  const baseDisplay = reactHostPort.useMemo(() => filterFootprintBoxPreviewDisplayItems(replBaseDisplayForHistory(snapshot), documentModel.model), [snapshot, documentModel.model]);
  const mergedDisplay = reactHostPort.useMemo(() => mergeDisplayWithArchivedBoxes(baseDisplay, allArchivedBoxLayouts, documentModel.model), [baseDisplay, allArchivedBoxLayouts, documentModel.model]);
  const chromeDefaults = reactHostPort.useMemo(() => defaultInteractionReplChromeState(), []);
  const [cmdLine, setCmdLine] = useHostState(cmdLineProp, onCmdLineChange, () => chromeDefaults.cmdLine);
  const [activeIndex, setActiveIndex] = useHostState(activeSuggestionIndexProp, onActiveSuggestionIndexChange, () => chromeDefaults.activeSuggestionIndex);
  const [filterTypologyToggles, setFilterTypologyToggles] = useHostState(filterTypologyTogglesProp, onFilterTypologyTogglesChange, () => chromeDefaults.filterTypologyToggles);
  const [selectionTypologyToggles, setSelectionTypologyToggles] = useHostState(selectionTypologyTogglesProp, onSelectionTypologyTogglesChange, () => chromeDefaults.selectionTypologyToggles);
  const [filterPrimitiveToggles, setFilterPrimitiveToggles] = useHostState(filterPrimitiveTogglesProp, onFilterPrimitiveTogglesChange, () => chromeDefaults.filterPrimitiveToggles);
  const [selectionPrimitiveToggles, setSelectionPrimitiveToggles] = useHostState(selectionPrimitiveTogglesProp, onSelectionPrimitiveTogglesChange, () => chromeDefaults.selectionPrimitiveToggles);
  const [activeModelDefinitionId, setActiveModelDefinitionId] = useHostState(activeModelDefinitionIdProp ?? undefined, onActiveModelDefinitionIdChange, () => chromeDefaults.activeModelDefinitionId ?? defaultModelDefinitionId());
  const mdIdForView = activeModelDefinitionId ?? defaultModelDefinitionId();
  const [selectionMethod, setSelectionMethod] = useHostState(selectionMethodProp, onSelectionMethodChange, () => chromeDefaults.selectionMethod);
  const [modelDefinitionRevision, setModelDefinitionRevision] = useHostState(modelDefinitionRevisionProp, onModelDefinitionRevisionChange, () => chromeDefaults.modelDefinitionRevision);
  const modelDefinitions = reactHostPort.useMemo(() => listModelDefinitionManifests(), []);
  const transfersFrom = reactHostPort.useMemo(() => listTransformationsIntoModelDefinition(activeModelDefinitionId ?? defaultModelDefinitionId()), [activeModelDefinitionId]);
  const transfersTo = reactHostPort.useMemo(() => listTransformationsFromModelDefinition(activeModelDefinitionId ?? defaultModelDefinitionId()), [activeModelDefinitionId]);
  const [transfersFromResetKey, setTransfersFromResetKey] = reactHostPort.useState(0);
  const [transfersToResetKey, setTransfersToResetKey] = reactHostPort.useState(0);
  const modelDefinitionScope = reactHostPort.useMemo(() => resolveModelDefinitionScope(activeModelDefinitionId ?? defaultModelDefinitionId()), [activeModelDefinitionId]);
  const scopedInteractions = reactHostPort.useMemo(() => listSpatialInteractionsForModelDefinition(activeModelDefinitionId ?? defaultModelDefinitionId()), [activeModelDefinitionId, modelDefinitionRevision]);
  const kernel = rt.kernel();
  const [dragSelection, setDragSelection] = useHostState(dragSelectionProp, onDragSelectionChange, () => chromeDefaults.dragSelection);
  const [selectionMenu, setSelectionMenu] = useHostState(selectionMenuProp, onSelectionMenuChange, () => chromeDefaults.selectionMenu);
  const [hoveredPickKey, setHoveredPickKey] = useHostState(hoveredPickKeyProp, onHoveredPickKeyChange, () => chromeDefaults.hoveredPickKey);
  const [rendererSelectionByModel, setRendererSelectionByModel] = useHostState(rendererSelectionByModelProp, onRendererSelectionByModelChange, () => ({ ...chromeDefaults.rendererSelectionByModel }));
  const [interactionSelectionByState, setInteractionSelectionByState] = useHostState(interactionSelectionByStateProp, onInteractionSelectionByStateChange, () => ({ ...chromeDefaults.interactionSelectionByState }));
  const [interactionMenuOpen, setInteractionMenuOpen] = useHostState(interactionMenuOpenProp, onInteractionMenuOpenChange, () => chromeDefaults.interactionMenuOpen);
  const [lastFinalizedInteractionId, setLastFinalizedInteractionId] = useHostState(lastFinalizedInteractionIdProp, onLastFinalizedInteractionIdChange, () => chromeDefaults.lastFinalizedInteractionId);
  const [canvasBinding, setCanvasBinding] = reactHostPort.useState<{ readonly camera: ThreeCamera; readonly domElement: HTMLCanvasElement } | null>(null);
  const handleCanvasReady = reactHostPort.useCallback(
    (binding: { readonly camera: ThreeCamera; readonly domElement: HTMLCanvasElement }) => {
      setCanvasBinding(binding);
      onCanvasReady?.(binding);
    },
    [onCanvasReady],
  );
  reactHostPort.useEffect(() => {
    onSnapshotChange?.(snapshot);
  }, [snapshot, onSnapshotChange]);
  const cmdRef = reactHostPort.useRef<HTMLInputElement>(null);
  const numericEntryPrevStateRef = reactHostPort.useRef(snapshot.state);
  const setCmdLineRef = reactHostPort.useRef(setCmdLine);
  const rendererSelectionByModelRef = reactHostPort.useRef(rendererSelectionByModel);
  const suppressAutoStartOnceRef = reactHostPort.useRef(false);
  const lastViewsRefreshRef = reactHostPort.useRef<{ readonly model: Model | null; readonly revision: number; readonly activeModelDefinitionId: string | null }>({
    model: null,
    revision: -1,
    activeModelDefinitionId: null,
  });
  const dragSelectionRef = reactHostPort.useRef<SpatialDragSelectionState | null>(null);
  const dragCleanupRef = reactHostPort.useRef<(() => void) | null>(null);
  const cameraNavigatingRef = reactHostPort.useRef(false);
  const [cameraNavigating, setCameraNavigating] = reactHostPort.useState(false);
  const interactionActive = isInteractionSessionActive(spec, snapshot.state);
  const boundInteractionSession = Boolean(interactionId) && interactionActive;
  const canvasFrameloop = frameloop ?? (boundInteractionSession || cameraNavigating ? "always" : "demand");
  const activeTransformGumballConfig = !boundInteractionSession && cadGumballConfigVisible(transformGumballConfig) ? transformGumballConfig : null;
  const displayedSelectionTargets = reactHostPort.useMemo(
    () => replDisplayedSelectionTargets(boundInteractionSession, activeModelDefinitionId, snapshot.state, rendererSelectionByModel, interactionSelectionByState),
    [boundInteractionSession, activeModelDefinitionId, snapshot.state, rendererSelectionByModel, interactionSelectionByState],
  );
  const selectedPickKeys = reactHostPort.useMemo(() => {
    const keys = new Set(displayedSelectionTargets.map(spatialSelectionTargetKey));
    return pinnedPickTargetKeys(keys);
  }, [displayedSelectionTargets]);
  const selectedPickKey = displayedSelectionTargets[0] ? spatialSelectionTargetKey(displayedSelectionTargets[0]) : null;
  const selectionInvalidateKey = reactHostPort.useMemo(() => [...selectedPickKeys].sort().join("\0"), [selectedPickKeys]);
  const geometryPreviewTransform = reactHostPort.useMemo(() => geometryPreviewTransformFromDisplay(mergedDisplay), [mergedDisplay]);
  const pickSourceGeometry = geometry ?? pickGeometryProp;
  const pickSourceRevision = pickSourceGeometry && typeof pickSourceGeometry === "object" && "revision" in pickSourceGeometry ? Number((pickSourceGeometry as { revision?: unknown }).revision) : 0;
  const pickTargets = reactHostPort.useMemo(() => createSpatialPickTargets(pickSourceGeometry, activeModelDefinitionId), [pickSourceGeometry, pickSourceRevision, modelDefinitionRevision, activeModelDefinitionId]);
  const activeTypologyIds = reactHostPort.useMemo(() => modelDefinitionTypologyIds(activeModelDefinitionId), [activeModelDefinitionId]);
  const scopedPickTargets = reactHostPort.useMemo(() => filterSpatialPickTargetsForActiveView(pickTargets, activeModelDefinitionId), [pickTargets, activeModelDefinitionId]);
  const visiblePickTargets = reactHostPort.useMemo(() => {
    const showPrimitives = filterSpatialPickTargetsForPrimitiveToggles(scopedPickTargets, filterPrimitiveToggles);
    return filterSpatialPickTargetsForTypologyToggles(showPrimitives, filterTypologyToggles, activeTypologyIds);
  }, [scopedPickTargets, filterPrimitiveToggles, filterTypologyToggles, activeTypologyIds]);
  const viewFilterKindToggles = reactHostPort.useMemo(() => spatialPickKindTogglesFromTypologyFilteredTargets(activeModelDefinitionId, visiblePickTargets), [activeModelDefinitionId, visiblePickTargets]);
  const sceneKindToggles = reactHostPort.useMemo(() => spatialSceneKindTogglesForModelDefinition(activeModelDefinitionId, filterPrimitiveToggles), [activeModelDefinitionId, filterPrimitiveToggles]);
  const entityFlagsForId = reactHostPort.useCallback(
    (entityId: string) =>
      pickSourceGeometry && typeof pickSourceGeometry === "object" && "metadata" in pickSourceGeometry ? resolveSpatialEntityFlags(pickSourceGeometry as Model, activeModelDefinitionId ?? defaultModelDefinitionId(), entityId) : {},
    [activeModelDefinitionId, pickSourceGeometry, pickSourceRevision],
  );
  const committedMeshesForView = reactHostPort.useMemo(() => {
    if (!modelDefinitionUsesGeometryPicking(mdIdForView)) return [];
    return filterCommittedMeshesForModelDefinition(gumballPreviewModel, mdIdForView, committedMeshes, {
      flagsForId: entityFlagsForId,
      typologyToggles: filterTypologyToggles,
      filterKindToggles: sceneKindToggles,
    });
  }, [committedMeshes, entityFlagsForId, filterTypologyToggles, gumballPreviewModel, mdIdForView, sceneKindToggles]);
  const factoryFaceMeshesForView = reactHostPort.useMemo(
    () =>
      listFactoryFaceMeshesForModelDefinition(gumballPreviewModel, mdIdForView, {
        flagsForId: entityFlagsForId,
        typologyToggles: filterTypologyToggles,
        filterKindToggles: sceneKindToggles,
      }),
    [entityFlagsForId, filterTypologyToggles, gumballPreviewModel, mdIdForView, sceneKindToggles],
  );
  const selectablePickTargets = reactHostPort.useMemo(() => {
    const filterPrimitives = filterSpatialPickTargetsForPrimitiveToggles(visiblePickTargets, selectionPrimitiveToggles);
    const typologyFiltered = filterSpatialPickTargetsForTypologyToggles(filterPrimitives, selectionTypologyToggles, activeTypologyIds);
    return filterSpatialPickTargetsForEntityFlags(typologyFiltered, entityFlagsForId);
  }, [visiblePickTargets, selectionPrimitiveToggles, selectionTypologyToggles, activeTypologyIds, entityFlagsForId]);
  const effectiveSelectionKindToggles = reactHostPort.useMemo(
    () => intersectSpatialPickKindToggles(viewFilterKindToggles, spatialPickKindTogglesFromTypologyFilteredTargets(activeModelDefinitionId, selectablePickTargets)),
    [activeModelDefinitionId, selectablePickTargets, viewFilterKindToggles],
  );
  const scopeTypologyIds = reactHostPort.useMemo(() => modelDefinitionScope.typologies.map((row) => row.id), [modelDefinitionScope.typologies]);
  const primitiveShowGroupState = reactHostPort.useMemo(() => spatialToggleGroupState(SPATIAL_PRIMITIVE_KINDS, filterPrimitiveToggles), [filterPrimitiveToggles]);
  const primitiveFilterGroupState = reactHostPort.useMemo(() => spatialToggleGroupState(SPATIAL_PRIMITIVE_KINDS, selectionPrimitiveToggles), [selectionPrimitiveToggles]);
  const typologyShowGroupState = reactHostPort.useMemo(() => spatialToggleGroupState(scopeTypologyIds, filterTypologyToggles), [scopeTypologyIds, filterTypologyToggles]);
  const typologySelectionGroupState = reactHostPort.useMemo(() => spatialToggleGroupState(scopeTypologyIds, selectionTypologyToggles), [scopeTypologyIds, selectionTypologyToggles]);
  return { activeIndex, activeModelDefinitionId, activeTransformGumballConfig, asideExtra, asideStyle, autoFitBehavior, autoFitMeshes, boundInteractionSession, cameraNavigatingRef, canvasBinding, canvasFrameloop, canvasOverrides, captureGlobalKeys, cmdLine, cmdRef, committedMeshesForView, displayedSelectionTargets, documentModel, dragCleanupRef, dragSelection, dragSelectionRef, effectiveSelectionKindToggles, engagementActionMode, entityFlagsForId, externalPendingProjectionSpec, factoryFaceMeshesForView, fillHost, filterPrimitiveToggles, filterTypologyToggles, geometry, geometryPreviewTransform, handleCanvasReady, handleProjectionSpecChange, handleTransformGumballCommit, handleTransformGumballPreview, handleTransformGumballPreviewEnd, hideModelDefinitionControls, hoveredPickKey, hoveredReferenceId, interactionActive, interactionId, interactionMenuOpen, interactionSelectionByState, lastFinalizedInteractionId, mdIdForView, mergedDisplay, modelDefinitionRevision, modelDefinitionScope, modelDefinitions, numericEntryPrevStateRef, onActionSubmit, onApplyTransformation, onCameraNavigateProp, onCancel, onDeleteSelection, onDocumentModelChange, onEngagementChange, onEscape, onGroundPickProp, onHoverTargetProp, onInteractionEventProp, onInteractionId, onRedo, onReferenceHover, onReferenceRelocate, onReferenceSelect, onScenePointerMoveProp, onSearchChange, onSelectionRequestProp, onTransitionRun, onUndo, pickSourceGeometry, primitiveFilterGroupState, primitiveShowGroupState, projectionSpec, referenceRelocateActive, renderDisplayItem, rendererSelectionByModel, rendererSelectionByModelRef, revealedReferenceIds, rootStyle, rt, rtRef, sceneKindToggles, scopeTypologyIds, scopedInteractions, selectablePickTargets, selectedPickKey, selectedPickKeys, selectedReferenceIds, selectionInvalidateKey, selectionMenu, selectionMethod, selectionPrimitiveToggles, selectionTypologyToggles, sessionRestartNonce, setActiveIndex, setActiveModelDefinitionId, setCameraNavigating, setCmdLine, setCmdLineRef, setDragSelection, setExternalPendingProjectionSpec, setFilterPrimitiveToggles, setFilterTypologyToggles, setHoveredPickKey, setInteractionMenuOpen, setInteractionSelectionByState, setLastFinalizedInteractionId, setModelDefinitionRevision, setRendererSelectionByModel, setSelectionMenu, setSelectionPrimitiveToggles, setSelectionTypologyToggles, setTransfersFromResetKey, setTransfersToResetKey, showAside, showEngagement, snapshot, spatialViewOverrides, spec, suppressAutoStartOnceRef, transfersFrom, transfersFromResetKey, transfersTo, transfersToResetKey, typologySelectionGroupState, typologyShowGroupState, viewFilterKindToggles, viewGeometry, viewSlots, viewTheme, worldReferences };
}

export type ReplCoreAfter1 = ReturnType<typeof useReplViewState>;
// #endregion 🪩️Repl
