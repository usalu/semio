/** 🎬️ CAD renderer REPL: the chrome dismissal, the interaction cancellation, the escape key, the runtime start and restart and the selection accept resolution of an interaction session. */
import { reactHostPort } from "@semio-tech/ui-react";
import { type ModelEntityKind } from "@semio-tech/s-spatial-kernel-ts/geometry";
import { getActiveSelectionSpec, interactionCanConfirmSelection, isFinalInteractionState } from "@semio-tech/s-spatial-kernel-ts/geometry/interaction-spec";
import { isShapeModelDefinition, defaultModelDefinitionId } from "@semio-tech/s-spatial-kernel-ts/geometry/definition-catalog";
import { listSpatialInteractionsForModelDefinition, countViewObjectsForModelDefinition } from "@semio-tech/s-spatial-kernel-ts/geometry/definition-ownership";
import { modelDefinitionSelectionEntityKinds } from "@semio-tech/s-spatial-kernel-ts/geometry/selection-operations";
import { expandSelectionTargetsForAccept } from "@semio-tech/s-spatial-kernel-ts/geometry/selection";
import { abortActiveInteractionSession } from "../../../🗿️artifact/🏃️interaction-runtime/🟦️.ts";
import { scenePreview } from "../../🔌️ports/🟦️.ts";
import { defaultSpatialPrimitiveToggles, defaultSpatialTypologyTogglesForModelDefinition } from "../../🧲️geometry-targets/🎯️pick-model/🟦️.ts";
import { replGeometryPickLayerVisible, replHostGeometryPickingEnabled } from "../../🎯️pick-layer/🟦️.tsx";
import { replEscapeAction, replFinalizeSelection, replInteractionSelectionFromContext, replRendererSelectionTargets, replSelectionAccepted, replSelectionTargetsEqual, replStartEvent, replWithInteractionSelectionTargets } from "../../🧾️repl-model/🟦️.tsx";
import type { ReplCoreAfter1 } from "../📺️view-state/🟦️.ts";

// #region 🪩️Repl
/** 🎬️ Runs the interaction session lifecycle of the REPL. */
export function useReplSessionLifecycle(core: ReplCoreAfter1) {
  const { activeModelDefinitionId, boundInteractionSession, cmdLine, documentModel, dragCleanupRef, dragSelectionRef, geometry, interactionActive, interactionId, lastFinalizedInteractionId, mdIdForView, modelDefinitionRevision, onCancel, onEscape, onInteractionId, rendererSelectionByModel, rendererSelectionByModelRef, rt, selectionMenu, sessionRestartNonce, setActiveIndex, setCmdLine, setCmdLineRef, setDragSelection, setFilterPrimitiveToggles, setFilterTypologyToggles, setHoveredPickKey, setInteractionMenuOpen, setInteractionSelectionByState, setLastFinalizedInteractionId, setRendererSelectionByModel, setSelectionMenu, setSelectionPrimitiveToggles, setSelectionTypologyToggles, snapshot, spec, suppressAutoStartOnceRef } = core;
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
  return { activeSelectionAccept, confirmInteractionSelection, handleEscapeKey, hostPickingEnabled, repeatLastFinalizedInteraction, showPickLayer, viewObjectCount };
}

export type ReplCoreAfter2 = ReplCoreAfter1 & ReturnType<typeof useReplSessionLifecycle>;
// #endregion 🪩️Repl
