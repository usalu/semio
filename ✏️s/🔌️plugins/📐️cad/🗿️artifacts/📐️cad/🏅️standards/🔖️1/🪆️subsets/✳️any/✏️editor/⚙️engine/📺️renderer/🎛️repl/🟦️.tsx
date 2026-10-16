/** 🎛️ The `InteractionRepl` component and its viewport.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | renderer}, which re-exports every public name below so import specifiers stay unchanged.
 */

// #region 🔌️Adapters
import {
  Button,
  borderNormalClass,
  canvasHostRootClass,
  Checkbox,
  cn,
  CanvasPickMenu,
  editorShellRootClass,
  floatingFieldSurfaceClass,
  floatingMenuSurfaceClass,
  floatingPaneAsideClass,
  floatingTagClass,
  floatingTagOffClass,
  floatingTagOnClass,
  focusActiveSearchInput,
  humanizeEngagementStepId,
  Input,
  LevelProvider,
  MenuItem,
  queryWindowSearchInput,
  reactHostPort,
  Select,
  SelectContent,
  SelectionMarquee,
  SelectItem,
  SelectTrigger,
  SelectValue,
  gumballPointerConsumesCanvasEventRef,
  WINDOW_SEARCH_USER,
  type EngagementControl,
  type EngagementSpec,
  type SearchSpec,
  type KeyboardEvent,
  type ReactNode,
  type ThreeCamera
} from "@semio-tech/framework/ui/targets/react";
// #endregion 🔌️Adapters
import { canvasPickTargetKey } from "@semio-tech/framework";
import { worldProjectionFamily, type WorldProjectionSpec } from "@semio-tech/infinite-world-r3f";
import { type Vec3 } from "../../../../../../../../../../⚙️engine/📔️registry/🟦️.ts";
import {
  expandSelectionTargetsForAccept,
  getActiveSelectionSpec,
  interactionCanConfirmSelection,
  isShapeModelDefinition,
  defaultModelDefinitionId,
  listModelDefinitionManifests,
  listTransformationsFromModelDefinition,
  listTransformationsIntoModelDefinition,
  listSpatialInteractionsForModelDefinition,
  modelDefinitionSelectionEntityKinds,
  modelDefinitionUsesGeometryPicking,
  countViewObjectsForModelDefinition,
  resolveModelDefinitionScope,
  qualifiedTransformationId,
  isFinalInteractionState,
  Model,
  type InteractionEvent,
  type SelectionTarget,
  type ModelEntityKind
} from "../../../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️.ts";
import { applyModelDiff, isEmptyModelDiff, type ModelDiff } from "../../../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/🗺️spatial/🟦️.ts";
import {
  abortActiveInteractionSession,
  listKeyedInteractionTransitions,
  interactionLengthEntryForState,
  interactionScalarEntryForState,
  interactionControlForState,
  interactionInNumericEntryState,
  interactionNumericEntryApplyEvent,
  interactionNumericEntryCommitEvent,
  interactionNumericEntryExplicitLockValue,
  interactionStepFinalizeEvent,
  parseNumericCommandLine,
  type InteractionKeybindRow,
  cadGumballConfigVisible
} from "../../🎬️actions/🟦️.ts";
import { isInteractionSessionActive } from "../../🗿️artifact/🟦️.ts";
import { scenePreview } from "../🔌️ports/🟦️.ts";
import { cadFieldClass } from "../🎨️style/🟦️.ts";
import {
  archivedBoxesFromHistory,
  createSpatialPickEvent,
  createSpatialPickTargets,
  defaultSpatialPrimitiveToggles,
  defaultSpatialTypologyTogglesForModelDefinition,
  filterCommittedMeshesForModelDefinition,
  filterFootprintBoxPreviewDisplayItems,
  filterSpatialPickTargetsForActiveView,
  filterSpatialPickTargetsForEntityFlags,
  filterSpatialPickTargetsForPrimitiveToggles,
  filterSpatialPickTargetsForTypologyToggles,
  geometryPreviewTransformFromDisplay,
  intersectSpatialPickKindToggles,
  listFactoryFaceMeshesForModelDefinition,
  mergeDisplayWithArchivedBoxes,
  modelDefinitionTypologyIds,
  pinnedPickTargetKeys,
  pruneSelectionTargetsForEntityFlags,
  replBaseDisplayForHistory,
  resolveSpatialEntityFlags,
  SPATIAL_PRIMITIVE_KINDS,
  type SpatialDragSelectionState,
  spatialPickCanvasRequest,
  spatialPickKindTogglesFromTypologyFilteredTargets,
  type SpatialPickTarget,
  spatialPickTargetKey,
  type SpatialPickTargetKind,
  spatialSceneKindTogglesForModelDefinition,
  type SpatialSelectionRequest,
  spatialSelectionTargetKey,
  spatialToggleGroupFill,
  spatialToggleGroupState,
  spatialTypologyToggleLabel,
  targetStyle,
  useDocumentMeshes
} from "../🧲️geometry-targets/🟦️.tsx";
import {
  dragDistance,
  pointerModifiersFromNativeEvent,
  replGeometryPickLayerVisible,
  replHostGeometryPickingEnabled,
  spatialPickTargetsFromClientPoint,
  spatialPickTargetsFromScreenSelection,
  spatialSelectionCoverageFromGesture,
  spatialSelectionModeFromModifiers,
  spatialSelectionTarget,
  uniqueSelectionTargets
} from "../🎯️pick-layer/🟦️.tsx";
import {
  InteractionCanvas,
  InteractionSelectionInvalidateBridge,
  InteractionSpatialView,
  useHostState,
  useInteractionSnapshot
} from "../🪩️canvas/🟦️.tsx";
import {
  buildEngagementControlFromResolved,
  buildInteractionReplEngagement,
  buildInteractionReplSearch,
  defaultInteractionReplChromeState,
  type InteractionReplEngagementInputs,
  type InteractionReplProps,
  replActionTextWithoutSpaces,
  replActiveCompletionSuffix,
  replBuildDispatchEvent,
  replDisplayedSelectionTargets,
  replEscapeAction,
  replFinalizeSelection,
  replInteractionIdOnSpace,
  replInteractionSelectionFromContext,
  replInteractionSuggestions,
  replIsQueryTypingTarget,
  replMergeSelectionPickInView,
  replNormalizeActionText,
  replPaletteRows,
  replPruneSelectionByPrimitive,
  replPruneSelectionByTypology,
  replRendererSelectionTargets,
  replSelectionAccepted,
  replSelectionEvent,
  replSelectionTargetsEqual,
  replShouldRepeatInteractionOnSpace,
  replStartEvent,
  type ReplSuggestion,
  replTryParseValueInteraction,
  replUserFacingSuggestionDetail,
  replWithInteractionSelectionTargets,
  replWithRendererSelectionTargets,
  resolveScopedSpatialInteractionKey,
  SpatialChromeMasterToggle,
  type SpatialInteractionSelectionByState,
  WorldOrbitProjectionSwitchPane
} from "../🧾️repl-model/🟦️.tsx";
import { SelectionAttributesPane } from "../📋️repl-panes/🟦️.tsx";

// #region 🪩️Repl
/** 🪩️ Full spatial REPL: canvas, interaction palette, history controls, last response. */
export function InteractionRepl({
  interactionId,
  spec,
  onInteractionId,
  runtime: rt,
  history,
  document: documentModel,
  geometry,
  asideExtra,
  archivedBoxLayouts = [],
  sessionRestartNonce = 0,
  viewTheme,
  viewSlots,
  renderDisplayItem,
  autoFitMeshes = false,
  autoFitBehavior = "initial",
  committedMeshesKeepPrevious = false,
  tessellationTolerance,
  cmdLine: cmdLineProp,
  activeSuggestionIndex: activeSuggestionIndexProp,
  filterTypologyToggles: filterTypologyTogglesProp,
  selectionTypologyToggles: selectionTypologyTogglesProp,
  filterPrimitiveToggles: filterPrimitiveTogglesProp,
  selectionPrimitiveToggles: selectionPrimitiveTogglesProp,
  activeModelDefinitionId: activeModelDefinitionIdProp,
  selectionMethod: selectionMethodProp,
  modelDefinitionRevision: modelDefinitionRevisionProp,
  dragSelection: dragSelectionProp,
  selectionMenu: selectionMenuProp,
  hoveredPickKey: hoveredPickKeyProp,
  rendererSelectionByModel: rendererSelectionByModelProp,
  interactionSelectionByState: interactionSelectionByStateProp,
  interactionMenuOpen: interactionMenuOpenProp,
  lastFinalizedInteractionId: lastFinalizedInteractionIdProp,
  onCmdLineChange,
  onActiveSuggestionIndexChange,
  onFilterTypologyTogglesChange,
  onSelectionTypologyTogglesChange,
  onFilterPrimitiveTogglesChange,
  onSelectionPrimitiveTogglesChange,
  onActiveModelDefinitionIdChange,
  onSelectionMethodChange,
  onModelDefinitionRevisionChange,
  onDragSelectionChange,
  onSelectionMenuChange,
  onHoveredPickKeyChange,
  onRendererSelectionByModelChange,
  onInteractionSelectionByStateChange,
  onInteractionMenuOpenChange,
  onLastFinalizedInteractionIdChange,
  onApplyTransformation,
  pickGeometry: pickGeometryProp,
  onDocumentModelChange,
  onCanvasReady,
  onInteractionEvent: onInteractionEventProp,
  onGroundPick: onGroundPickProp,
  onScenePointerMove: onScenePointerMoveProp,
  onSelectionRequest: onSelectionRequestProp,
  onHoverTarget: onHoverTargetProp,
  onCameraNavigate: onCameraNavigateProp,
  onActionSubmit,
  onTransitionRun,
  onCancel,
  onUndo,
  onRedo,
  onSnapshotChange,
  onEscape,
  onDeleteSelection,
  rootStyle,
  asideStyle,
  showAside = true,
  showEngagement = false,
  onEngagementChange,
  onSearchChange,
  fillHost = false,
  captureGlobalKeys = true,
  hideModelDefinitionControls = false,
  frameloop,
  canvas: canvasOverrides,
  spatialView: spatialViewOverrides,
  transformGumballConfig = null,
  onTransformGumballCommit,
  worldReferences = [],
  selectedReferenceIds,
  hoveredReferenceId = null,
  revealedReferenceIds,
  referenceRelocateActive = true,
  onReferenceSelect,
  onReferenceHover,
  onReferenceRelocate,
}: InteractionReplProps): ReactNode {
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
  reactHostPort.useEffect(() => {
    setCmdLineRef.current = setCmdLine;
  }, [setCmdLine]);

  reactHostPort.useEffect(() => {
    rendererSelectionByModelRef.current = rendererSelectionByModel;
  }, [rendererSelectionByModel]);

  const dismissReplChrome = reactHostPort.useCallback(() => {
    dragCleanupRef.current?.();
    dragCleanupRef.current = null;
    dragSelectionRef.current = null;
    setDragSelection(null);
    setCmdLine("");
    setSelectionMenu(null);
    setHoveredPickKey(null);
    setInteractionMenuOpen(false);
  }, []);

  const cancelActiveInteraction = reactHostPort.useCallback(() => {
    const aborted = abortActiveInteractionSession(rt);
    if (!aborted && !interactionId) return false;
    if (!aborted) rt.cancel();
    suppressAutoStartOnceRef.current = true;
    setInteractionSelectionByState({});
    dismissReplChrome();
    if (interactionId) onInteractionId("");
    onCancel?.();
    return true;
  }, [rt, interactionId, onInteractionId, dismissReplChrome, onCancel, setInteractionSelectionByState]);

  reactHostPort.useEffect(() => {
    if (!interactionId || !snapshot.lastResponse?.ok) return;
    setLastFinalizedInteractionId(interactionId);
    setRendererSelectionByModel((prev) => replFinalizeSelection(prev, activeModelDefinitionId, snapshot.lastResponse));
    setInteractionSelectionByState((prev) => (Object.keys(prev).length === 0 ? prev : {}));
    setCmdLine("");
  }, [interactionId, snapshot.lastResponse, activeModelDefinitionId, setInteractionSelectionByState, setLastFinalizedInteractionId, setRendererSelectionByModel, setCmdLine]);

  reactHostPort.useEffect(() => {
    if (!interactionId || !isFinalInteractionState(spec, snapshot.state)) return;
    setCmdLine("");
  }, [interactionId, spec, snapshot.state, setCmdLine]);

  reactHostPort.useEffect(() => {
    if (!interactionId || interactionActive) return;
    if (!isFinalInteractionState(spec, snapshot.state)) return;
    onInteractionId("");
  }, [interactionId, interactionActive, spec, snapshot.state, onInteractionId]);

  const handleEscapeKey = reactHostPort.useCallback(() => {
    if (selectionMenu !== null) {
      setSelectionMenu(null);
      setHoveredPickKey(null);
      onEscape?.();
      return;
    }
    switch (replEscapeAction({ hasInteraction: Boolean(interactionId), interactionActive, cmdLine, hasSelectionMenu: selectionMenu !== null })) {
      case "abort":
        cancelActiveInteraction();
        onEscape?.();
        return;
      case "dismiss":
        dismissReplChrome();
        onEscape?.();
        return;
      default:
        return;
    }
  }, [interactionId, interactionActive, cmdLine, selectionMenu, dismissReplChrome, cancelActiveInteraction, onEscape, setSelectionMenu, setHoveredPickKey]);

  const startRuntime = reactHostPort.useCallback(async () => {
    const mdId = activeModelDefinitionId ?? defaultModelDefinitionId();
    const rendererSel = replRendererSelectionTargets(rendererSelectionByModelRef.current, mdId);
    const selSpec = getActiveSelectionSpec(spec, rt.getSnapshot().state);
    const accepted = selSpec ? expandSelectionTargetsForAccept(documentModel.model, selSpec, rendererSel) : replSelectionAccepted(rt.listActiveSelectionAccept() as readonly ModelEntityKind[], rendererSel);
    setInteractionSelectionByState({ [rt.getSnapshot().state]: [...accepted] });
    await rt.send(replStartEvent(accepted));
  }, [rt, spec, documentModel.model, activeModelDefinitionId, setInteractionSelectionByState]);

  reactHostPort.useEffect(() => {
    if (!interactionId) return;
    if (suppressAutoStartOnceRef.current) {
      suppressAutoStartOnceRef.current = false;
      return;
    }
    void startRuntime();
  }, [interactionId, startRuntime]);

  reactHostPort.useEffect(() => {
    if (sessionRestartNonce <= 0) return;
    rt.cancel();
    void startRuntime();
  }, [sessionRestartNonce, rt, startRuntime]);

  const repeatLastFinalizedInteraction = reactHostPort.useCallback(() => {
    if (!lastFinalizedInteractionId) return;
    onInteractionId(lastFinalizedInteractionId);
  }, [lastFinalizedInteractionId, onInteractionId]);

  const modelRevision = documentModel.model.revision;
  const hostPickingEnabled = replHostGeometryPickingEnabled(interactionId, spec, snapshot.state);
  const showPickLayer = replGeometryPickLayerVisible(mdIdForView);

  reactHostPort.useEffect(() => {
    setSelectionMenu(null);
    setHoveredPickKey(null);
  }, [geometry, snapshot.state, modelDefinitionRevision]);

  reactHostPort.useEffect(() => {
    const mdId = activeModelDefinitionId ?? defaultModelDefinitionId();
    const typologyDefaults = defaultSpatialTypologyTogglesForModelDefinition(mdId);
    const primitiveDefaults = defaultSpatialPrimitiveToggles();
    setFilterTypologyToggles(typologyDefaults);
    setSelectionTypologyToggles(typologyDefaults);
    setFilterPrimitiveToggles(primitiveDefaults);
    setSelectionPrimitiveToggles(primitiveDefaults);
    const allowed = listSpatialInteractionsForModelDefinition(mdId);
    if (interactionId && !allowed.some((row) => row.id === interactionId)) onInteractionId("");
    setLastFinalizedInteractionId("");
  }, [activeModelDefinitionId, modelDefinitionRevision, interactionId, onInteractionId, setFilterTypologyToggles, setSelectionTypologyToggles, setFilterPrimitiveToggles, setSelectionPrimitiveToggles, setLastFinalizedInteractionId]);

  reactHostPort.useEffect(() => {
    setCmdLine("");
    setActiveIndex(0);
    setSelectionMenu(null);
    setHoveredPickKey(null);
    setInteractionMenuOpen(false);
    setInteractionSelectionByState((prev) => (Object.keys(prev).length === 0 ? prev : {}));
  }, [interactionId, rt, setInteractionSelectionByState]);

  const confirmInteractionSelection = reactHostPort.useCallback(() => {
    const snap = rt.getSnapshot();
    if (!interactionCanConfirmSelection(spec, snap.state, snap.context, scenePreview())) return false;
    void rt.send({ kind: "confirm", modifiers: {} });
    return true;
  }, [rt, spec]);

  reactHostPort.useEffect(() => {
    if (!interactionId || !interactionActive) {
      setInteractionSelectionByState((prev) => (Object.keys(prev).length === 0 ? prev : {}));
      return;
    }
    const stateId = snapshot.state;
    const machineTargets = replInteractionSelectionFromContext(snapshot.context);
    setInteractionSelectionByState((prev) => {
      const current = prev[stateId] ?? [];
      if (hostPickingEnabled && machineTargets.length === 0 && current.length > 0) {
        return prev;
      }
      if (replSelectionTargetsEqual(current, machineTargets)) return prev;
      return replWithInteractionSelectionTargets(prev, stateId, machineTargets);
    });
  }, [interactionId, interactionActive, hostPickingEnabled, snapshot.revision, snapshot.state, snapshot.context, setInteractionSelectionByState]);

  const runtimeSelectionAccept = reactHostPort.useMemo(() => rt.listActiveSelectionAccept(), [rt, snapshot.state]);
  const defaultSelectionAccept = reactHostPort.useMemo(() => modelDefinitionSelectionEntityKinds(activeModelDefinitionId ?? defaultModelDefinitionId()), [activeModelDefinitionId]);
  const activeSelectionAccept = reactHostPort.useMemo((): readonly ModelEntityKind[] => {
    if (runtimeSelectionAccept.length > 0) {
      const allowed = new Set(defaultSelectionAccept);
      return runtimeSelectionAccept.filter((kind) => allowed.has(kind));
    }
    if (boundInteractionSession && runtimeSelectionAccept.length === 0) return [];
    return defaultSelectionAccept;
  }, [runtimeSelectionAccept, boundInteractionSession, defaultSelectionAccept]);
  const viewObjectCount = reactHostPort.useMemo(() => {
    if (isShapeModelDefinition(activeModelDefinitionId)) return 0;
    return countViewObjectsForModelDefinition(documentModel.model, mdIdForView);
  }, [activeModelDefinitionId, documentModel.model, mdIdForView, modelDefinitionRevision]);

  const commitSelection = reactHostPort.useCallback(
    (selection: readonly SelectionTarget[]) => {
      setSelectionMenu(null);
      setHoveredPickKey(null);
      const mdId = activeModelDefinitionId ?? defaultModelDefinitionId();
      if (boundInteractionSession) {
        setInteractionSelectionByState((prev) => replWithInteractionSelectionTargets(prev, snapshot.state, selection));
      } else {
        setRendererSelectionByModel((prev) => replWithRendererSelectionTargets(prev, mdId, selection));
      }
    },
    [boundInteractionSession, activeModelDefinitionId, snapshot.state, setInteractionSelectionByState, setRendererSelectionByModel, setSelectionMenu, setHoveredPickKey],
  );

  const applySelectionPrune = reactHostPort.useCallback(
    (map: (selection: readonly SelectionTarget[]) => readonly SelectionTarget[]) => {
      const mdId = activeModelDefinitionId ?? defaultModelDefinitionId();
      setRendererSelectionByModel((prev) => replWithRendererSelectionTargets(prev, mdId, map(replRendererSelectionTargets(prev, mdId))));
      setInteractionSelectionByState((prev) => {
        let next: SpatialInteractionSelectionByState = prev;
        for (const stateId of Object.keys(prev)) {
          const pruned = map(prev[stateId] ?? []);
          next = replWithInteractionSelectionTargets(next, stateId, pruned);
        }
        return next;
      });
    },
    [activeModelDefinitionId, setRendererSelectionByModel, setInteractionSelectionByState],
  );

  const dispatchSelectionTargets = reactHostPort.useCallback(
    (targets: readonly SpatialPickTarget[], modifiers: InteractionEvent["modifiers"] = {}, point?: Vec3) => {
      const eligibleTargets = filterSpatialPickTargetsForEntityFlags(targets, entityFlagsForId);
      const picked = uniqueSelectionTargets(pruneSelectionTargetsForEntityFlags(eligibleTargets.map(spatialSelectionTarget), entityFlagsForId));
      const nextSelection = replMergeSelectionPickInView(boundInteractionSession, activeModelDefinitionId, snapshot.state, rendererSelectionByModel, interactionSelectionByState, picked, modifiers);
      commitSelection(nextSelection);
      if (boundInteractionSession && picked.length > 0) void rt.send({ ...replSelectionEvent(picked, point), modifiers });
    },
    [commitSelection, boundInteractionSession, entityFlagsForId, interactionSelectionByState, activeModelDefinitionId, snapshot.state, rt, rendererSelectionByModel],
  );

  const onSelectionRequest = reactHostPort.useCallback(
    (request: SpatialSelectionRequest) => {
      onSelectionRequestProp?.(request);
      if (request.targets.length === 1) {
        dispatchSelectionTargets([request.targets[0]!], request.modifiers, request.point);
        return;
      }
      setSelectionMenu(request);
      setHoveredPickKey(request.targets[0] ? spatialPickTargetKey(request.targets[0]) : null);
    },
    [dispatchSelectionTargets, onSelectionRequestProp, setSelectionMenu, setHoveredPickKey],
  );

  const onHoverTarget = reactHostPort.useCallback(
    (target: SpatialPickTarget | null) => {
      const key = target ? spatialPickTargetKey(target) : null;
      setHoveredPickKey((prev) => (prev === key ? prev : key));
      onHoverTargetProp?.(target);
    },
    [onHoverTargetProp, setHoveredPickKey],
  );

  const onCameraNavigate = reactHostPort.useCallback(
    (active: boolean) => {
      cameraNavigatingRef.current = active;
      setCameraNavigating((prev) => (prev === active ? prev : active));
      if (active) onHoverTarget(null);
      onCameraNavigateProp?.(active);
    },
    [onHoverTarget, onCameraNavigateProp],
  );

  reactHostPort.useEffect(() => {
    const canvas = canvasBinding?.domElement;
    const camera = canvasBinding?.camera;
    if (!canvas || !camera || !hostPickingEnabled) return;
    let lastHoverAt = 0;
    const onMove = (event: PointerEvent) => {
      if (cameraNavigatingRef.current || event.buttons !== 0) {
        onHoverTarget(null);
        return;
      }
      const now = performance.now();
      if (now - lastHoverAt < 32) return;
      lastHoverAt = now;
      const rect = canvas.getBoundingClientRect();
      const hits = spatialPickTargetsFromClientPoint({ x: event.clientX, y: event.clientY }, camera, rect, selectablePickTargets, [], {});
      onHoverTarget(hits[0] ?? null);
    };
    const onLeave = () => onHoverTarget(null);
    canvas.addEventListener("pointermove", onMove, { passive: true });
    canvas.addEventListener("pointerleave", onLeave, { passive: true });
    return () => {
      canvas.removeEventListener("pointermove", onMove);
      canvas.removeEventListener("pointerleave", onLeave);
    };
  }, [canvasBinding, hostPickingEnabled, selectablePickTargets, onHoverTarget]);

  const pointerMoveActive = reactHostPort.useMemo(() => {
    const si = snapshot.spatialInteraction;
    return si.spatialGroundPick && (si.groundPointerMoveStates.includes(snapshot.state) || si.heightDragStates.includes(snapshot.state) || si.verticalRodStates.includes(snapshot.state));
  }, [snapshot.state, snapshot.spatialInteraction]);

  const onSpatialInteractionEvent = reactHostPort.useCallback(
    (ev: InteractionEvent) => {
      onInteractionEventProp?.(ev);
      const activeRt = rtRef.current;
      if (ev.kind === "pointer.down") {
        const st = activeRt.getSnapshot().state;
        const hi = activeRt.getSnapshot().spatialInteraction.heightConfirmState;
        const snapEv = (ev as { snap?: { kind: string; id: string } }).snap;
        if (hi && st === hi && !snapEv) {
          void activeRt.send({ kind: "confirm", modifiers: (ev as { modifiers?: Record<string, unknown> }).modifiers ?? {} });
          return;
        }
        if (snapEv && activeSelectionAccept.length > 0 && activeSelectionAccept.includes(snapEv.kind as ModelEntityKind) && effectiveSelectionKindToggles[snapEv.kind as SpatialPickTargetKind] !== false) {
          const snapTarget: SpatialPickTarget = {
            kind: snapEv.kind as SpatialPickTargetKind,
            id: snapEv.id,
            point: (ev as { point?: Vec3 }).point ?? [0, 0, 0],
          };
          const selection = spatialSelectionTarget(snapTarget);
          const modifiers = (ev as { modifiers?: InteractionEvent["modifiers"] }).modifiers ?? {};
          commitSelection(replMergeSelectionPickInView(boundInteractionSession, activeModelDefinitionId, snapshot.state, rendererSelectionByModel, interactionSelectionByState, [selection], modifiers));
          if (boundInteractionSession) void activeRt.send({ ...replSelectionEvent([selection], (ev as { point?: Vec3 }).point), modifiers });
          return;
        }
      }
      if (ev.kind === "pointer.move" && !pointerMoveActive) return;
      if (ev.kind === "pointer.down" || ev.kind === "pointer.move" || ev.kind === "contextmenu") void activeRt.send(ev);
    },
    [activeSelectionAccept, commitSelection, boundInteractionSession, interactionSelectionByState, rendererSelectionByModel, snapshot.state, pointerMoveActive, activeModelDefinitionId, effectiveSelectionKindToggles, onInteractionEventProp],
  );

  reactHostPort.useEffect(() => {
    const canvas = canvasBinding?.domElement;
    const camera = canvasBinding?.camera;
    if (!canvas || !camera || !hostPickingEnabled || activeSelectionAccept.length === 0) return;
    const clearDragSelection = () => {
      dragCleanupRef.current = null;
      dragSelectionRef.current = null;
      setDragSelection(null);
    };
    const beginDragSelection = (event: PointerEvent) => {
      if (event.button !== 0 || gumballPointerConsumesCanvasEventRef.current) return;
      dragCleanupRef.current?.();
      const rect = canvas.getBoundingClientRect();
      const startClient = { x: event.clientX, y: event.clientY };
      const initial: SpatialDragSelectionState = {
        method: selectionMethod,
        coverage: "full",
        startClient,
        currentClient: startClient,
        path: [startClient],
        modifiers: pointerModifiersFromNativeEvent(event),
      };
      dragSelectionRef.current = initial;
      const moveSelection = (moveEvent: PointerEvent) => {
        const current = dragSelectionRef.current;
        if (!current) return;
        const nextClient = { x: moveEvent.clientX, y: moveEvent.clientY };
        const nextPath = current.method === "lasso" && dragDistance(current.path[current.path.length - 1]!, nextClient) >= 2 ? [...current.path, nextClient] : current.method === "lasso" ? current.path : [current.startClient, nextClient];
        const nextState: SpatialDragSelectionState = {
          ...current,
          currentClient: nextClient,
          path: nextPath,
          coverage: spatialSelectionCoverageFromGesture(current.method, nextPath),
          modifiers: pointerModifiersFromNativeEvent(moveEvent),
        };
        dragSelectionRef.current = nextState;
        if (dragDistance(nextState.startClient, nextClient) >= 4) setDragSelection(nextState);
      };
      const finishSelection = (upEvent: PointerEvent) => {
        window.removeEventListener("pointermove", moveSelection, true);
        window.removeEventListener("pointerup", finishSelection, true);
        const current = dragSelectionRef.current;
        clearDragSelection();
        if (!current) return;
        const finalState: SpatialDragSelectionState = {
          ...current,
          currentClient: { x: upEvent.clientX, y: upEvent.clientY },
          path: current.method === "lasso" ? [...current.path, { x: upEvent.clientX, y: upEvent.clientY }] : [current.startClient, { x: upEvent.clientX, y: upEvent.clientY }],
          modifiers: pointerModifiersFromNativeEvent(upEvent),
        };
        const distance = dragDistance(finalState.startClient, finalState.currentClient);
        const clearSelectionOnEmptyBackgroundPick = () => {
          if (spatialSelectionModeFromModifiers(finalState.modifiers as { readonly alt?: boolean; readonly ctrl?: boolean; readonly meta?: boolean; readonly shift?: boolean }) === "default") {
            commitSelection(replMergeSelectionPickInView(boundInteractionSession, activeModelDefinitionId, snapshot.state, rendererSelectionByModel, interactionSelectionByState, [], finalState.modifiers));
          }
        };
        if (distance < 4) {
          const candidates = spatialPickTargetsFromClientPoint(finalState.currentClient, camera, rect, selectablePickTargets, activeSelectionAccept, {});
          if (candidates.length === 0) {
            clearSelectionOnEmptyBackgroundPick();
            return;
          }
          onSelectionRequest({
            targets: candidates,
            point: candidates[0]!.point,
            client: finalState.currentClient,
            modifiers: finalState.modifiers,
          });
          return;
        }
        const targets = spatialPickTargetsFromScreenSelection(
          { ...finalState, coverage: spatialSelectionCoverageFromGesture(finalState.method, finalState.path) },
          selectablePickTargets,
          camera,
          canvas.getBoundingClientRect(),
          activeSelectionAccept,
          {},
          geometryPreviewTransform,
        );
        if (targets.length === 0) {
          clearSelectionOnEmptyBackgroundPick();
          return;
        }
        dispatchSelectionTargets(targets, finalState.modifiers);
      };
      dragCleanupRef.current = () => {
        window.removeEventListener("pointermove", moveSelection, true);
        window.removeEventListener("pointerup", finishSelection, true);
        clearDragSelection();
      };
      window.addEventListener("pointermove", moveSelection, true);
      window.addEventListener("pointerup", finishSelection, true);
    };
    canvas.addEventListener("pointerdown", beginDragSelection, true);
    return () => {
      dragCleanupRef.current?.();
      canvas.removeEventListener("pointerdown", beginDragSelection, true);
    };
  }, [
    activeSelectionAccept,
    canvasBinding,
    commitSelection,
    dispatchSelectionTargets,
    boundInteractionSession,
    interactionSelectionByState,
    onSelectionRequest,
    activeModelDefinitionId,
    snapshot.state,
    rendererSelectionByModel,
    selectionMethod,
    geometryPreviewTransform,
    selectablePickTargets,
    hostPickingEnabled,
  ]);

  const dispatchTransition = reactHostPort.useCallback(
    (row: InteractionKeybindRow) => {
      onTransitionRun?.(row);
      const ev = replBuildDispatchEvent(row, { interactionId: spec.id, model: documentModel.model });
      if (ev) void rtRef.current.send(ev);
    },
    [spec.id, documentModel.model, onTransitionRun],
  );

  const transitionRows = reactHostPort.useMemo(() => listKeyedInteractionTransitions(spec, snapshot.state), [spec, snapshot.state]);

  const allSuggestions = reactHostPort.useMemo((): ReplSuggestion[] => {
    const out: ReplSuggestion[] = [];
    for (const p of scopedInteractions) {
      out.push({
        kind: "interaction",
        key: p.key,
        label: p.label,
        detail: replUserFacingSuggestionDetail(p.key),
        interactionId: p.id,
        onRun: () => onInteractionId(p.id),
      });
    }
    for (const row of transitionRows) {
      out.push({
        kind: "transition",
        key: row.key,
        label: row.label,
        detail: undefined,
        transition: row,
        onRun: () => dispatchTransition(row),
      });
    }
    for (const defn of modelDefinitionScope.selectionOperations) {
      out.push({
        kind: "selection",
        key: defn.key,
        label: defn.label,
        detail: undefined,
        onRun: () => {
          void rt.query(`CALL ${defn.id}({}) YIELD data.targets AS targets`);
        },
      });
    }
    for (const actionId of modelDefinitionScope.actions) {
      if (actionId.startsWith("selection.")) continue;
      const tail = actionId.includes(".") ? actionId.slice(actionId.lastIndexOf(".") + 1) : actionId;
      out.push({
        kind: "action",
        key: tail,
        label: humanizeEngagementStepId(tail),
        detail: undefined,
        onRun: () => {
          void rt.query(`CALL ${actionId}({})`);
        },
      });
    }
    return out;
  }, [scopedInteractions, transitionRows, modelDefinitionScope, onInteractionId, dispatchTransition, rt]);

  const filtered = reactHostPort.useMemo(() => replPaletteRows(cmdLine, allSuggestions), [cmdLine, allSuggestions]);
  const interactionMatches = reactHostPort.useMemo(() => replInteractionSuggestions(cmdLine, allSuggestions), [cmdLine, allSuggestions]);
  const completionSuffix = reactHostPort.useMemo(() => replActiveCompletionSuffix(cmdLine, filtered, activeIndex), [cmdLine, filtered, activeIndex]);

  reactHostPort.useEffect(() => {
    setActiveIndex((i) => (filtered.length ? Math.min(i, filtered.length - 1) : 0));
  }, [filtered.length, cmdLine]);

  const runSuggestion = reactHostPort.useCallback((s: ReplSuggestion) => {
    s.onRun();
    setCmdLine("");
    setActiveIndex(0);
    setInteractionMenuOpen(false);
  }, []);

  const runInteractionIdFromSpace = reactHostPort.useCallback(
    (id: string | null): boolean => {
      if (!id) return false;
      onInteractionId(id);
      setCmdLine("");
      setActiveIndex(0);
      setInteractionMenuOpen(false);
      return true;
    },
    [onInteractionId],
  );

  const replCmdLineValue = reactHostPort.useCallback((): string => cmdRef.current?.value ?? cmdLine, [cmdLine]);

  const tryCommitNumericEntry = reactHostPort.useCallback(async (): Promise<boolean> => {
    const snap = rt.getSnapshot();
    const state = snap.state;
    if (!interactionInNumericEntryState(spec, state)) return false;
    const parsed = parseNumericCommandLine(replCmdLineValue());
    const value = parsed !== null && parsed !== undefined ? parsed : parsed === null ? interactionNumericEntryExplicitLockValue(spec, state, snap.context) : null;
    if (value == null) return false;
    const applyEv = interactionNumericEntryApplyEvent(spec, state, value);
    if (applyEv) await rt.send(applyEv);
    const after = rt.getSnapshot();
    const commitEv = interactionNumericEntryCommitEvent(spec, after.state, after.context, rt.previewKernel());
    if (!commitEv) return false;
    await rt.send(commitEv);
    setCmdLine("");
    setInteractionMenuOpen(false);
    return true;
  }, [replCmdLineValue, rt, spec, setCmdLine]);

  const tryFinalizeInteractionStep = reactHostPort.useCallback(async (): Promise<boolean> => {
    const snap = rt.getSnapshot();
    const ev = interactionStepFinalizeEvent(spec, snap.state, snap.context, rt.previewKernel());
    if (!ev) return false;
    await rt.send(ev);
    setCmdLine("");
    setInteractionMenuOpen(false);
    return true;
  }, [rt, spec, setCmdLine]);

  const tryConfirmOrNumericCommit = reactHostPort.useCallback(async (): Promise<boolean> => {
    const snap = rt.getSnapshot();
    const inNumeric = interactionInNumericEntryState(spec, snap.state);
    const emptyCmd = !replCmdLineValue().trim();
    if (inNumeric && emptyCmd && (await tryFinalizeInteractionStep())) return true;
    if (inNumeric && (await tryCommitNumericEntry())) return true;
    if (inNumeric && emptyCmd) {
      const ev = interactionNumericEntryCommitEvent(spec, snap.state, snap.context, rt.previewKernel());
      if (ev) {
        await rt.send(ev);
        setCmdLine("");
        setInteractionMenuOpen(false);
        return true;
      }
    }
    if (emptyCmd && (await tryFinalizeInteractionStep())) return true;
    return false;
  }, [replCmdLineValue, rt, spec, tryCommitNumericEntry, tryFinalizeInteractionStep, setCmdLine]);

  const trySubmitLine = reactHostPort.useCallback((): boolean => {
    const raw = cmdLine.trim();
    if (!raw) return false;
    if (onActionSubmit?.(raw)) {
      setCmdLine("");
      return true;
    }
    const valEv = replTryParseValueInteraction(raw, spec, rt.getSnapshot().state);
    if (valEv) {
      void rt.send(valEv);
      setCmdLine("");
      return true;
    }
    const interactionHit = resolveScopedSpatialInteractionKey(raw, activeModelDefinitionId ?? defaultModelDefinitionId());
    if (interactionHit) {
      onInteractionId(interactionHit.id);
      setCmdLine("");
      return true;
    }
    const rows = listKeyedInteractionTransitions(spec, rt.getSnapshot().state);
    for (const row of rows) {
      if (row.eventKind === "set.height" || row.eventKind === "set.distance" || row.eventKind === "set.footprint") continue;
      if (row.key === raw || row.key.toLowerCase() === raw.toLowerCase() || row.eventKind.toLowerCase() === raw.toLowerCase()) {
        dispatchTransition(row);
        setCmdLine("");
        return true;
      }
    }
    return false;
  }, [cmdLine, spec, rt, dispatchTransition, onInteractionId, onActionSubmit, setCmdLine, activeModelDefinitionId]);

  const runTransitionRow = reactHostPort.useCallback(
    (row: InteractionKeybindRow) => {
      if (row.eventKind.startsWith("set.")) {
        setCmdLine(row.key);
        window.setTimeout(() => cmdRef.current?.focus(), 0);
        return;
      }
      dispatchTransition(row);
    },
    [dispatchTransition],
  );

  const onInputKeyDown = reactHostPort.useCallback(
    (e: KeyboardEvent<HTMLInputElement>) => {
      if (e.key === "Escape") {
        e.preventDefault();
        handleEscapeKey();
        return;
      }
      if (e.key === " " && !e.ctrlKey && !e.metaKey && !e.altKey) {
        e.preventDefault();
        if (interactionInNumericEntryState(spec, rt.getSnapshot().state)) {
          void tryConfirmOrNumericCommit();
          return;
        }
        const interactionIdOnSpace = replInteractionIdOnSpace(cmdLine, filtered, allSuggestions, lastFinalizedInteractionId, !interactionId);
        if (runInteractionIdFromSpace(interactionIdOnSpace)) return;
        if (!cmdLine.trim() && !interactionId && lastFinalizedInteractionId) {
          repeatLastFinalizedInteraction();
        }
        setInteractionMenuOpen(false);
        return;
      }
      if (e.key === "ArrowDown" && filtered.length) {
        e.preventDefault();
        setInteractionMenuOpen(false);
        setActiveIndex((i) => (i + 1) % filtered.length);
        return;
      }
      if (e.key === "ArrowUp" && filtered.length) {
        e.preventDefault();
        setInteractionMenuOpen(false);
        setActiveIndex((i) => (i - 1 + filtered.length) % filtered.length);
        return;
      }
      if (e.key === "Tab" && filtered.length) {
        e.preventDefault();
        const suffix = replActiveCompletionSuffix(cmdLine, filtered, activeIndex);
        if (suffix) {
          setCmdLine(replActionTextWithoutSpaces(cmdLine + suffix));
          return;
        }
        runSuggestion(filtered[activeIndex] ?? filtered[0]!);
        return;
      }
      if (e.key === "Enter") {
        e.preventDefault();
        setInteractionMenuOpen(false);
        if (!cmdLine.trim()) {
          void (async () => {
            if (interactionInNumericEntryState(spec, rt.getSnapshot().state) && (await tryConfirmOrNumericCommit())) return;
            if (await tryFinalizeInteractionStep()) return;
            if (confirmInteractionSelection()) return;
            if (trySubmitLine()) return;
            if (filtered.length) runSuggestion(filtered[activeIndex]!);
          })();
          return;
        }
        if (interactionInNumericEntryState(spec, rt.getSnapshot().state)) {
          void tryCommitNumericEntry();
          return;
        }
        if (trySubmitLine()) return;
        if (filtered.length) runSuggestion(filtered[activeIndex]!);
        return;
      }
    },
    [
      cmdLine,
      allSuggestions,
      filtered,
      activeIndex,
      runSuggestion,
      trySubmitLine,
      tryCommitNumericEntry,
      tryConfirmOrNumericCommit,
      tryFinalizeInteractionStep,
      replCmdLineValue,
      handleEscapeKey,
      lastFinalizedInteractionId,
      runInteractionIdFromSpace,
      confirmInteractionSelection,
      spec,
      rt,
      interactionId,
      repeatLastFinalizedInteraction,
    ],
  );

  const submitEngagementLine = reactHostPort.useCallback(() => {
    setInteractionMenuOpen(false);
    void (async () => {
      if (!cmdLine.trim()) {
        if (interactionInNumericEntryState(spec, rt.getSnapshot().state) && (await tryConfirmOrNumericCommit())) return;
        if (await tryFinalizeInteractionStep()) return;
        if (confirmInteractionSelection()) return;
      }
      if (interactionInNumericEntryState(spec, rt.getSnapshot().state)) {
        if (await tryCommitNumericEntry()) return;
      }
      if (trySubmitLine()) return;
      if (filtered.length) runSuggestion(filtered[activeIndex] ?? filtered[0]!);
    })();
  }, [cmdLine, confirmInteractionSelection, spec, rt, tryCommitNumericEntry, tryConfirmOrNumericCommit, tryFinalizeInteractionStep, trySubmitLine, filtered, runSuggestion, activeIndex]);

  const applyEngagementNumericValue = reactHostPort.useCallback(
    (value: number) => {
      setCmdLine(String(value));
      const state = rt.getSnapshot().state;
      const applyEv = interactionNumericEntryApplyEvent(spec, state, value);
      if (applyEv) void rt.send(applyEv);
    },
    [spec, rt, setCmdLine],
  );

  const commitEngagementNumericValue = reactHostPort.useCallback(
    (value: number) => {
      applyEngagementNumericValue(value);
      void (async () => {
        if (interactionInNumericEntryState(spec, rt.getSnapshot().state)) {
          await tryCommitNumericEntry();
        }
      })();
    },
    [applyEngagementNumericValue, spec, rt, tryCommitNumericEntry],
  );

  const engagementControl = reactHostPort.useMemo((): EngagementControl | undefined => {
    if (!showEngagement || !boundInteractionSession || !interactionInNumericEntryState(spec, snapshot.state)) return undefined;
    const resolved = interactionControlForState(spec, snapshot.state, rt.getSnapshot().context as Record<string, unknown>);
    if (!resolved) return undefined;
    return buildEngagementControlFromResolved(resolved, {
      onNumericChange: applyEngagementNumericValue,
      onNumericCommit: commitEngagementNumericValue,
    });
  }, [applyEngagementNumericValue, boundInteractionSession, commitEngagementNumericValue, rt, showEngagement, snapshot.state, spec]);

  const replEngagementInputs = reactHostPort.useMemo<InteractionReplEngagementInputs>(
    () => ({
      showEngagement,
      boundInteractionSession,
      interactionId,
      state: snapshot.state,
      lastResponseOk: snapshot.lastResponse ? snapshot.lastResponse.ok : null,
      lastResponseErrorCount: snapshot.lastResponse?.errors?.length ?? 0,
      selectionCount: displayedSelectionTargets.length,
      cmdLine,
      control: engagementControl,
      transitions: transitionRows,
      interactions: scopedInteractions,
      onTransition: runTransitionRow,
      onStartInteraction: (id: string) => onInteractionId(id),
      onInputChange: (value: string) => setCmdLine(replNormalizeActionText(value, engagementActionMode)),
      onInputSubmit: () => submitEngagementLine(),
      onRepeatLast: lastFinalizedInteractionId ? repeatLastFinalizedInteraction : undefined,
      onAbort: handleEscapeKey,
    }),
    [
      showEngagement,
      engagementActionMode,
      boundInteractionSession,
      engagementControl,
      transitionRows,
      scopedInteractions,
      runTransitionRow,
      interactionId,
      snapshot.state,
      snapshot.lastResponse,
      displayedSelectionTargets,
      cmdLine,
      setCmdLine,
      submitEngagementLine,
      handleEscapeKey,
      lastFinalizedInteractionId,
      repeatLastFinalizedInteraction,
    ],
  );

  const engagementSpec = reactHostPort.useMemo<EngagementSpec | null>(() => buildInteractionReplEngagement(replEngagementInputs), [replEngagementInputs]);
  const searchSpec = reactHostPort.useMemo<SearchSpec | null>(() => buildInteractionReplSearch(replEngagementInputs), [replEngagementInputs]);

  reactHostPort.useEffect(() => {
    onEngagementChange?.(engagementSpec);
  }, [engagementSpec, onEngagementChange]);

  reactHostPort.useEffect(() => () => onEngagementChange?.(null), [onEngagementChange]);

  reactHostPort.useEffect(() => {
    onSearchChange?.(searchSpec);
  }, [searchSpec, onSearchChange]);

  reactHostPort.useEffect(() => () => onSearchChange?.(null), [onSearchChange]);

  reactHostPort.useEffect(() => {
    const state = snapshot.state;
    const lengthEntry = interactionLengthEntryForState(spec, state);
    const scalarEntry = interactionScalarEntryForState(spec, state);
    const prevLength = interactionLengthEntryForState(spec, numericEntryPrevStateRef.current);
    const prevScalar = interactionScalarEntryForState(spec, numericEntryPrevStateRef.current);
    const leftNumeric = (prevLength && (!lengthEntry || prevLength.state !== lengthEntry.state)) || (prevScalar && (!scalarEntry || prevScalar.state !== scalarEntry.state));
    if (leftNumeric) setCmdLine("");
    numericEntryPrevStateRef.current = state;
    const live = parseNumericCommandLine(cmdLine);
    if (live === undefined) return;
    if (live === null) return;
    const applyEv = interactionNumericEntryApplyEvent(spec, state, live);
    if (applyEv) void rt.send(applyEv);
  }, [cmdLine, snapshot.state, spec, rt, setCmdLine]);

  const focusReplActionInput = reactHostPort.useCallback(() => {
    if (!showAside && showEngagement && focusActiveSearchInput()) return;
    cmdRef.current?.focus({ preventScroll: true });
  }, [showAside, showEngagement]);

  const replActionInputElement = reactHostPort.useCallback((): HTMLInputElement | null => {
    if (!showAside && showEngagement) {
      return queryWindowSearchInput(true) ?? queryWindowSearchInput(false);
    }
    return cmdRef.current;
  }, [showAside, showEngagement]);

  reactHostPort.useEffect(() => {
    if (!captureGlobalKeys) return;
    const onWinCapture = (e: globalThis.KeyboardEvent) => {
      if (e.defaultPrevented || e.isComposing) return;
      const t = e.target;
      const one = e.key.length === 1 ? e.key : "";
      if (replIsQueryTypingTarget(t)) return;
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "z") {
        e.preventDefault();
        e.stopPropagation();
        if (e.shiftKey) {
          rt.redo();
          onRedo?.();
        } else {
          rt.undo();
          onUndo?.();
        }
        return;
      }
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "y") {
        e.preventDefault();
        e.stopPropagation();
        rt.redo();
        onRedo?.();
        return;
      }
      if ((e.key === "Delete" || e.key === "Backspace") && !e.ctrlKey && !e.metaKey && !e.altKey && onDeleteSelection?.()) {
        e.preventDefault();
        e.stopPropagation();
        return;
      }
      if (e.key === " " && !e.ctrlKey && !e.metaKey && !e.altKey) {
        if (engagementActionMode) {
          e.preventDefault();
          e.stopPropagation();
          if (!cmdLine.trim() && !boundInteractionSession && lastFinalizedInteractionId) {
            repeatLastFinalizedInteraction();
            return;
          }
          focusReplActionInput();
          submitEngagementLine();
          return;
        }
        e.preventDefault();
        e.stopPropagation();
        const snap = rt.getSnapshot();
        if (interactionInNumericEntryState(spec, snap.state)) {
          void tryConfirmOrNumericCommit();
          return;
        }
        if (!cmdLine.trim()) {
          void (async () => {
            if (await tryFinalizeInteractionStep()) return;
            if (confirmInteractionSelection()) return;
          })();
          return;
        }
        const matches = replPaletteRows(cmdLine, allSuggestions);
        const interactionIdOnSpace = replInteractionIdOnSpace(cmdLine, matches, allSuggestions, lastFinalizedInteractionId, !interactionId);
        if (runInteractionIdFromSpace(interactionIdOnSpace)) return;
        else if (replShouldRepeatInteractionOnSpace(e, { interactionId, interactionActive, cmdTarget: cmdRef.current })) repeatLastFinalizedInteraction();
        return;
      }
      const actionInput = replActionInputElement();
      if (actionInput && t !== actionInput && e.key === "Backspace") {
        e.preventDefault();
        e.stopPropagation();
        focusReplActionInput();
        setCmdLineRef.current((prev) => prev.slice(0, -1));
        return;
      }
      if (e.key === "Escape" && actionInput) {
        if (t !== actionInput) {
          e.preventDefault();
          e.stopPropagation();
          focusReplActionInput();
        }
        handleEscapeKey();
        return;
      }
      if (actionInput && t !== actionInput && e.key === "Enter") {
        e.preventDefault();
        e.stopPropagation();
        focusReplActionInput();
        const snap = rt.getSnapshot();
        if (!cmdLine.trim()) {
          void (async () => {
            if (interactionInNumericEntryState(spec, snap.state) && (await tryConfirmOrNumericCommit())) return;
            if (await tryFinalizeInteractionStep()) return;
            if (confirmInteractionSelection()) return;
          })();
          return;
        }
        if (interactionInNumericEntryState(spec, snap.state)) {
          void tryCommitNumericEntry();
          return;
        }
        if (cmdLine.trim()) void trySubmitLine();
        return;
      }
      if (!showAside && showEngagement && e.key === "Tab" && !e.ctrlKey && !e.metaKey && !e.altKey) {
        const field = queryWindowSearchInput(true) ?? queryWindowSearchInput(false);
        if (field && t !== field) {
          e.preventDefault();
          e.stopPropagation();
          focusReplActionInput();
          field.dispatchEvent(new KeyboardEvent("keydown", { key: "Tab", bubbles: true, cancelable: true }));
          return;
        }
      }
      if (!one || e.ctrlKey || e.metaKey || e.altKey) return;
      if (t === actionInput) return;
      e.preventDefault();
      e.stopPropagation();
      focusReplActionInput();
      setCmdLineRef.current((prev) => replNormalizeActionText(`${prev}${one}`, engagementActionMode));
    };
    window.addEventListener("keydown", onWinCapture, true);
    return () => window.removeEventListener("keydown", onWinCapture, true);
  }, [
    captureGlobalKeys,
    rt,
    spec,
    cmdLine,
    allSuggestions,
    trySubmitLine,
    tryCommitNumericEntry,
    tryConfirmOrNumericCommit,
    tryFinalizeInteractionStep,
    handleEscapeKey,
    interactionId,
    interactionActive,
    boundInteractionSession,
    repeatLastFinalizedInteraction,
    lastFinalizedInteractionId,
    runInteractionIdFromSpace,
    confirmInteractionSelection,
    onUndo,
    onRedo,
    onDeleteSelection,
    focusReplActionInput,
    replActionInputElement,
    showAside,
    showEngagement,
    engagementActionMode,
    submitEngagementLine,
  ]);

  const onScenePointerMove = reactHostPort.useCallback(
    (p: Vec3) => {
      const event = createSpatialPickEvent("pointer.move", p, null);
      void rtRef.current.send(event);
      onScenePointerMoveProp?.(p, event);
    },
    [onScenePointerMoveProp],
  );

  const pickPlaneOn = snapshot.spatialInteraction.spatialGroundPick ? !snapshot.spatialInteraction.pickDisabledStates.includes(snapshot.state) : false;

  const lr = snapshot.lastResponse;
  const dragOverlayRect = canvasBinding?.domElement.getBoundingClientRect() ?? null;
  const dragOverlayPoints = dragSelection && dragOverlayRect ? dragSelection.path.map((point) => ({ x: point.x - dragOverlayRect.left, y: point.y - dragOverlayRect.top })) : [];

  return (
    <div className={fillHost ? canvasHostRootClass : editorShellRootClass} data-level="base" style={rootStyle}>
      <div className="relative min-h-0 min-w-0 flex-1">
        <InteractionCanvas
          {...canvasOverrides}
          managedCamera={spatialViewOverrides?.cameraView !== undefined}
          frameloop={canvasFrameloop}
          className={cn("bg-transparent", canvasOverrides?.className)}
          onCanvasReady={handleCanvasReady}
          overlay={<WorldOrbitProjectionSwitchPane spec={projectionSpec} onSpecChange={handleProjectionSpecChange} />}
        >
          <InteractionSelectionInvalidateBridge selectionKey={selectionInvalidateKey} />
          <InteractionSpatialView
            previewKernel={rt.previewKernel()}
            snapshot={snapshot}
            onInteractionEvent={onSpatialInteractionEvent}
            onGroundPick={onGroundPickProp}
            onScenePointerMove={pointerMoveActive ? onScenePointerMove : undefined}
            pickEnabled={pickPlaneOn}
            geometry={viewGeometry}
            pickGeometry={pickSourceGeometry}
            committedMeshes={committedMeshesForView}
            factoryFaceMeshes={factoryFaceMeshesForView}
            transformGumballModel={documentModel.model}
            onTransformGumballPreview={handleTransformGumballPreview}
            onTransformGumballPreviewEnd={handleTransformGumballPreviewEnd}
            onTransformGumballCommit={handleTransformGumballCommit}
            activeModelDefinitionId={activeModelDefinitionId}
            modelDefinitionRevision={modelDefinitionRevision}
            displayModel={mergedDisplay}
            renderDisplayItem={renderDisplayItem}
            selectionAccept={hostPickingEnabled ? activeSelectionAccept : []}
            filterKindToggles={viewFilterKindToggles}
            sceneKindToggles={sceneKindToggles}
            selectionKindToggles={effectiveSelectionKindToggles}
            hoveredTargetKey={hoveredPickKey}
            selectedTargetKey={selectedPickKey}
            selectedTargetKeys={selectedPickKeys}
            hostSelectionEnabled={hostPickingEnabled}
            showPickLayer={showPickLayer}
            onSelectionRequest={onSelectionRequest}
            onCameraNavigate={onCameraNavigate}
            autoFitMeshes={autoFitMeshes}
            autoFitBehavior={autoFitBehavior}
            theme={viewTheme}
            slots={viewSlots}
            transformGumballConfig={activeTransformGumballConfig}
            transformGumballTargets={displayedSelectionTargets}
            worldReferences={worldReferences}
            selectedReferenceIds={selectedReferenceIds}
            hoveredReferenceId={hoveredReferenceId}
            revealedReferenceIds={revealedReferenceIds}
            referenceRelocateActive={referenceRelocateActive}
            onReferenceSelect={onReferenceSelect}
            onReferenceHover={onReferenceHover}
            onReferenceRelocate={onReferenceRelocate}
            {...spatialViewOverrides}
            projectionSpec={projectionSpec}
            externalPendingProjectionSpec={externalPendingProjectionSpec}
            onExternalPendingProjectionSpecClear={() => setExternalPendingProjectionSpec(null)}
            onProjectionSpecChange={handleProjectionSpecChange}
          />
        </InteractionCanvas>
        {dragSelection && dragOverlayRect ? (
          dragSelection.method === "rectangle" ? (
            <SelectionMarquee
              className="z-[4]"
              coverage={dragSelection.coverage}
              shape="rect"
              rect={{
                x: Math.min(dragOverlayPoints[0]?.x ?? 0, dragOverlayPoints[1]?.x ?? 0),
                y: Math.min(dragOverlayPoints[0]?.y ?? 0, dragOverlayPoints[1]?.y ?? 0),
                width: Math.abs((dragOverlayPoints[1]?.x ?? 0) - (dragOverlayPoints[0]?.x ?? 0)),
                height: Math.abs((dragOverlayPoints[1]?.y ?? 0) - (dragOverlayPoints[0]?.y ?? 0)),
              }}
            />
          ) : (
            <SelectionMarquee className="z-[4]" coverage={dragSelection.coverage} shape="polygon" points={dragOverlayPoints} />
          )
        ) : null}
        {selectionMenu ? (
          <CanvasPickMenu
            request={spatialPickCanvasRequest(selectionMenu)}
            hoveredKey={hoveredPickKey}
            onHoverKey={(key) => setHoveredPickKey(key)}
            onPick={(target) => {
              const spatial = selectionMenu.targets.find((row) => spatialPickTargetKey(row) === canvasPickTargetKey(target));
              if (spatial) dispatchSelectionTargets([spatial], selectionMenu.modifiers, selectionMenu.point);
            }}
            onDismiss={() => setSelectionMenu(null)}
            renderRow={(target, active) => {
              const spatial = selectionMenu.targets.find((row) => spatialPickTargetKey(row) === canvasPickTargetKey(target));
              if (!spatial) return <span>{target.label}</span>;
              return (
                <>
                  <span className="mr-single inline-block size-2 rounded-xs" style={{ background: targetStyle(spatial, false, false).color }} />
                  <span className="text-muted-foreground">{target.domain}</span> <code className="text-foreground">{target.label}</code>
                </>
              );
            }}
          />
        ) : null}
      </div>
      {showAside ? (
        <aside className={cn(floatingPaneAsideClass, fillHost ? "max-h-[45%] w-full shrink-0 border-l-0 border-t" : "w-layout-floating-menu-lg shrink-0")} style={asideStyle} data-level="pane">
        <LevelProvider level="pane">
          <strong className="text-sm font-semibold">Editor</strong>
          <div className="flex flex-wrap gap-half">
            {transitionRows.map((row) => (
              <Button
                key={`${row.key}-${row.eventKind}-${row.label}`}
                id={`cad.replTransition.${row.eventKind}.${row.key}`}
                type="button"
                variant="outline"
                icon="arrow-right"
                className="h-auto px-single py-half text-xs"
                onClick={() => runTransitionRow(row)}
              >
                <span className="font-bold underline">{row.key}</span> {row.label}
              </Button>
            ))}
          </div>
          <div className={cn("relative grid overflow-visible", floatingFieldSurfaceClass)} data-level="menu">
            <Input
              id="cad.replCommand"
              ref={cmdRef}
              type="text"
              autoComplete="off"
              spellCheck={false}
              value={cmdLine}
              onChange={(e) => {
                setCmdLine(replNormalizeActionText(e.target.value, engagementActionMode));
                if (interactionMenuOpen) setInteractionMenuOpen(true);
              }}
              onKeyDown={onInputKeyDown}
              placeholder={WINDOW_SEARCH_USER.actionPlaceholder}
              aria-label={WINDOW_SEARCH_USER.actionPlaceholder}
              className="col-start-1 row-start-1 border-0 bg-transparent pr-large shadow-none focus-visible:ring-0"
            />
            <Button
              id="cad.replSuggestionsToggle"
              type="button"
              variant="outline"
              icon={interactionMenuOpen ? "chevron-up" : "chevron-down"}
              className={cn("col-start-1 row-start-1 z-[1] mr-single size-medium justify-self-end self-center p-0 text-2xs", interactionMenuOpen && "bg-accent text-accent-foreground")}
              onMouseDown={(e) => e.preventDefault()}
              onClick={() => {
                setInteractionMenuOpen((open) => !open);
                cmdRef.current?.focus();
              }}
              aria-label={WINDOW_SEARCH_USER.suggestionsAria}
            />
            {completionSuffix ? (
              <div aria-hidden className="text-element pointer-events-none col-start-1 row-start-1 overflow-hidden pr-large pl-double py-tiny text-sm leading-normal whitespace-pre">
                <span className="text-transparent">{cmdLine}</span>
                <span className="text-muted-foreground">{completionSuffix}</span>
              </div>
            ) : null}
            {interactionMenuOpen ? (
              <div
                role="menu"
                aria-label={WINDOW_SEARCH_USER.suggestionsAria}
                onPointerDown={(e) => e.stopPropagation()}
                className={cn("absolute top-[calc(100%+var(--spacing-double))] right-0 z-[3] max-h-layout-floating-menu-sm w-layout-floating-menu-md max-w-[calc(100vw-var(--size-xl))] overflow-y-auto p-single", floatingMenuSurfaceClass)}
                data-level="menu"
              >
                {interactionMatches.length ? (
                  interactionMatches.map((suggestion) => (
                    <MenuItem key={`${suggestion.kind}:${suggestion.key}:${suggestion.detail}`} onClick={() => runSuggestion(suggestion)}>
                      <div className="flex items-center gap-single">
                        <span className={cn(floatingTagClass, floatingTagOffClass, "inline-flex min-w-6 items-center justify-center px-half py-0 text-2xs font-bold uppercase")}>{suggestion.key}</span>
                        <span>{suggestion.label}</span>
                      </div>
                      {suggestion.detail ? <div className="text-muted-foreground text-2xs">{suggestion.detail}</div> : null}
                    </MenuItem>
                  ))
                ) : (
                  <div className="text-muted-foreground px-single py-half text-xs">{WINDOW_SEARCH_USER.noMatches}</div>
                )}
              </div>
            ) : null}
          </div>
          {asideExtra}
          {onDocumentModelChange ? (
            <SelectionAttributesPane
              model={documentModel.model}
              activeModelDefinitionId={activeModelDefinitionId ?? defaultModelDefinitionId()}
              selection={displayedSelectionTargets}
              selectionCount={displayedSelectionTargets.length}
              onModelChange={onDocumentModelChange}
            />
          ) : null}
          <div className="flex flex-col gap-single text-xs">
            {hideModelDefinitionControls ? null : (
              <>
                <label className="flex flex-col gap-half">
                  <span>Model definition</span>
                  <Select
                    id="cad.modelDefinitionPicker"
                    value={activeModelDefinitionId ?? defaultModelDefinitionId()}
                    onValueChange={(next) => {
                      setActiveModelDefinitionId(next || defaultModelDefinitionId());
                      setModelDefinitionRevision((r) => r + 1);
                      setSelectionMenu(null);
                      setHoveredPickKey(null);
                    }}
                  >
                    <SelectTrigger className={cn(cadFieldClass, "rounded-md border px-single py-half", borderNormalClass)}>
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      {modelDefinitions.map((row) => (
                        <SelectItem key={row.id} value={row.id}>
                          {row.label} ({row.id})
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </label>
                <span className="text-muted-foreground">
                  {modelDefinitionScope.typologies.length} kind{modelDefinitionScope.typologies.length === 1 ? "" : "s"}
                  {" · "}
                  {modelDefinitionScope.interactions.length} tool{modelDefinitionScope.interactions.length === 1 ? "" : "s"}
                  {" · "}
                  {modelDefinitionScope.attributeDefinitions.length} attribute{modelDefinitionScope.attributeDefinitions.length === 1 ? "" : "s"}
                  {" · "}
                  {modelDefinitionScope.propertyDefinitions.length} propert{modelDefinitionScope.propertyDefinitions.length === 1 ? "y" : "ies"}
                  {" · "}
                  {modelDefinitionScope.statDefinitions.length} stat{modelDefinitionScope.statDefinitions.length === 1 ? "" : "s"}
                </span>
                {transfersFrom.length ? (
                  <label className="flex flex-col gap-half">
                    <span>Transfer from</span>
                    <Select
                      id="cad.transferFromPicker"
                      key={transfersFromResetKey}
                      onValueChange={(qid) => {
                        const spec = transfersFrom.find((row) => qualifiedTransformationId(row.modelDefinitionId, row.id) === qid);
                        if (spec) onApplyTransformation?.(spec);
                        setTransfersFromResetKey((k) => k + 1);
                      }}
                    >
                      <SelectTrigger className={cn(cadFieldClass, "rounded-md border px-single py-half", borderNormalClass)}>
                        <SelectValue placeholder="Select incoming transformation…" />
                      </SelectTrigger>
                      <SelectContent>
                        {transfersFrom.map((row) => (
                          <SelectItem key={qualifiedTransformationId(row.modelDefinitionId, row.id)} value={qualifiedTransformationId(row.modelDefinitionId, row.id)}>
                            {row.label} ({row.source.modelDefinition} → {row.target.modelDefinition})
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                  </label>
                ) : null}
                {transfersTo.length ? (
                  <label className="flex flex-col gap-half">
                    <span>Transfer to</span>
                    <Select
                      id="cad.transferToPicker"
                      key={transfersToResetKey}
                      onValueChange={(qid) => {
                        const spec = transfersTo.find((row) => qualifiedTransformationId(row.modelDefinitionId, row.id) === qid);
                        if (spec) onApplyTransformation?.(spec);
                        setTransfersToResetKey((k) => k + 1);
                      }}
                    >
                      <SelectTrigger className={cn(cadFieldClass, "rounded-md border px-single py-half", borderNormalClass)}>
                        <SelectValue placeholder="Select outgoing transformation…" />
                      </SelectTrigger>
                      <SelectContent>
                        {transfersTo.map((row) => (
                          <SelectItem key={qualifiedTransformationId(row.modelDefinitionId, row.id)} value={qualifiedTransformationId(row.modelDefinitionId, row.id)}>
                            {row.label} ({row.source.modelDefinition} → {row.target.modelDefinition})
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                  </label>
                ) : null}
              </>
            )}
            {!isShapeModelDefinition(activeModelDefinitionId) ? (
              <span className="text-muted-foreground">
                {viewObjectCount} object{viewObjectCount === 1 ? "" : "s"}
              </span>
            ) : null}
            <label className="text-foreground flex items-center gap-single text-xs font-semibold">
              <SpatialChromeMasterToggle state={primitiveShowGroupState} ariaLabel="Show all primitives" onEnabledChange={(enabled) => setFilterPrimitiveToggles(spatialToggleGroupFill(SPATIAL_PRIMITIVE_KINDS, enabled))} />
              Primitives · Show
            </label>
            <div role="group" aria-label="Show primitives" className="flex flex-wrap gap-half">
              {SPATIAL_PRIMITIVE_KINDS.map((kind) => (
                <label key={`show-primitive-${kind}`} className={cn(floatingTagClass, filterPrimitiveToggles[kind] !== false ? floatingTagOnClass : floatingTagOffClass)}>
                  <Checkbox
                    checked={filterPrimitiveToggles[kind] !== false}
                    onChange={(e) => {
                      setFilterPrimitiveToggles((prev) => ({ ...prev, [kind]: e.target.checked }));
                    }}
                  />
                  {kind}
                </label>
              ))}
            </div>
            <label className="text-foreground flex items-center gap-single text-xs font-semibold">
              <SpatialChromeMasterToggle
                state={primitiveFilterGroupState}
                ariaLabel="Filter all primitives"
                onEnabledChange={(enabled) => {
                  setSelectionPrimitiveToggles(spatialToggleGroupFill(SPATIAL_PRIMITIVE_KINDS, enabled));
                  setSelectionMenu(null);
                  setHoveredPickKey(null);
                  if (!enabled) {
                    applySelectionPrune((prev) => {
                      let next = prev;
                      for (const kind of SPATIAL_PRIMITIVE_KINDS) {
                        next = replPruneSelectionByPrimitive(next, kind);
                      }
                      return next;
                    });
                  }
                }}
              />
              Primitives · Filter
            </label>
            <div role="group" aria-label="Filter primitives" className="flex flex-wrap gap-half">
              {SPATIAL_PRIMITIVE_KINDS.map((kind) => (
                <label key={`filter-primitive-${kind}`} className={cn(floatingTagClass, selectionPrimitiveToggles[kind] !== false ? floatingTagOnClass : floatingTagOffClass)}>
                  <Checkbox
                    checked={selectionPrimitiveToggles[kind] !== false}
                    onChange={(e) => {
                      const checked = e.target.checked;
                      setSelectionPrimitiveToggles((prev) => ({ ...prev, [kind]: checked }));
                      setSelectionMenu(null);
                      setHoveredPickKey(null);
                      if (!checked) {
                        applySelectionPrune((prev) => replPruneSelectionByPrimitive(prev, kind));
                      }
                    }}
                  />
                  {kind}
                </label>
              ))}
            </div>
            <label className="text-foreground flex items-center gap-single text-xs font-semibold">
              <SpatialChromeMasterToggle state={typologyShowGroupState} ariaLabel="Show all typologies" onEnabledChange={(enabled) => setFilterTypologyToggles(spatialToggleGroupFill(scopeTypologyIds, enabled))} />
              Typologies · Show
            </label>
            <div role="group" aria-label="Show typologies" className="flex flex-wrap gap-half">
              {modelDefinitionScope.typologies.map((typology) => {
                const label = spatialTypologyToggleLabel(typology.id, typology.label);
                return (
                  <label key={`show-${typology.id}`} className={cn(floatingTagClass, filterTypologyToggles[typology.id] !== false ? floatingTagOnClass : floatingTagOffClass)}>
                    <Checkbox
                      checked={filterTypologyToggles[typology.id] !== false}
                      onChange={(e) => {
                        setFilterTypologyToggles((prev) => ({ ...prev, [typology.id]: e.target.checked }));
                      }}
                    />
                    {label}
                  </label>
                );
              })}
            </div>
            <label className="text-foreground flex items-center gap-single text-xs font-semibold">
              <SpatialChromeMasterToggle
                state={typologySelectionGroupState}
                ariaLabel="Select all typologies"
                onEnabledChange={(enabled) => {
                  setSelectionTypologyToggles(spatialToggleGroupFill(scopeTypologyIds, enabled));
                  setSelectionMenu(null);
                  setHoveredPickKey(null);
                  if (!enabled) {
                    applySelectionPrune((prev) => {
                      let next = prev;
                      for (const typologyId of scopeTypologyIds) {
                        next = replPruneSelectionByTypology(next, documentModel.model, activeModelDefinitionId, typologyId);
                      }
                      return next;
                    });
                  }
                }}
              />
              Typologies · Selection
            </label>
            <div role="group" aria-label="Selection typologies" className="flex flex-wrap gap-half">
              {modelDefinitionScope.typologies.map((typology) => {
                const label = spatialTypologyToggleLabel(typology.id, typology.label);
                return (
                  <label key={`select-${typology.id}`} className={cn(floatingTagClass, selectionTypologyToggles[typology.id] !== false ? floatingTagOnClass : floatingTagOffClass)}>
                    <Checkbox
                      checked={selectionTypologyToggles[typology.id] !== false}
                      onChange={(e) => {
                        const checked = e.target.checked;
                        setSelectionTypologyToggles((prev) => ({ ...prev, [typology.id]: checked }));
                        setSelectionMenu(null);
                        setHoveredPickKey(null);
                        if (!checked) {
                          applySelectionPrune((prev) => replPruneSelectionByTypology(prev, documentModel.model, activeModelDefinitionId, typology.id));
                        }
                      }}
                    />
                    {label}
                  </label>
                );
              })}
            </div>
          </div>
          <div className="text-muted-foreground text-xs">
            {interactionId ? (
              <>
                Interaction <code className="text-foreground">{interactionId}</code> · state <code className="text-foreground">{snapshot.state}</code> · rev {snapshot.revision}
              </>
            ) : (
              <>
                No interaction selected · state <code className="text-foreground">{snapshot.state}</code> · rev {snapshot.revision}
              </>
            )}
          </div>
          <div className="border-border text-xs border-t pt-single">
            <strong className="text-foreground">Last response</strong>
            <pre className="text-muted-foreground mt-half mb-0 max-h-layout-popover overflow-auto text-2xs">{lr ? JSON.stringify(lr, null, 2) : "—"}</pre>
            {snapshot.diagnostics.length ? (
              <ul className="text-muted-foreground m-0 list-inside list-disc text-2xs">
                {snapshot.diagnostics.map((d, i) => (
                  <li key={`${d.code}-${i}`}>
                    [{d.severity}] {d.code}: {d.message}
                  </li>
                ))}
              </ul>
            ) : null}
          </div>
        </LevelProvider>
        </aside>
      ) : null}
    </div>
  );
}

/** 🪟️ Canvas-only {@link InteractionRepl} (no model-definition aside); full host props and `on*` callbacks. */
export function InteractionReplViewport(props: InteractionReplProps): ReactNode {
  return <InteractionRepl {...props} showAside={false} fillHost />;
}
// #endregion 🪩️Repl
