/** 🎯️ CAD renderer REPL: the selection commit, prune and dispatch, the hover and camera callbacks, the spatial interaction event and the drag selection gesture. */
import { reactHostPort, gumballPointerConsumesCanvasEventRef } from "@semio-tech/ui-react";
import { type Vec3 } from "@semio-tech/cad-engine-ts/registry";
import { type ModelEntityKind } from "@semio-tech/s-spatial-kernel-ts/geometry";
import { defaultModelDefinitionId } from "@semio-tech/s-spatial-kernel-ts/geometry/definition-catalog";
import { type InteractionEvent, type SelectionTarget } from "@semio-tech/s-spatial-kernel-ts/geometry/selection";
import { filterSpatialPickTargetsForEntityFlags, pruneSelectionTargetsForEntityFlags, type SpatialDragSelectionState, type SpatialPickTarget, spatialPickTargetKey, type SpatialPickTargetKind, type SpatialSelectionRequest } from "../../🧲️geometry-targets/🎯️pick-model/🟦️.ts";
import { dragDistance, pointerModifiersFromNativeEvent, spatialPickTargetsFromClientPoint, spatialPickTargetsFromScreenSelection, spatialSelectionCoverageFromGesture, spatialSelectionModeFromModifiers, spatialSelectionTarget, uniqueSelectionTargets } from "../../🎯️pick-layer/🟦️.tsx";
import { replMergeSelectionPickInView, replRendererSelectionTargets, replSelectionEvent, replWithInteractionSelectionTargets, replWithRendererSelectionTargets, type SpatialInteractionSelectionByState } from "../../🧾️repl-model/🟦️.tsx";
import type { ReplCoreAfter2 } from "../🎬️session-lifecycle/🟦️.ts";

// #region 🪩️Repl
/** 🎯️ Wires the selection commit and the canvas pointer gestures of the REPL. */
export function useReplSelectionAndPointer(core: ReplCoreAfter2) {
  const { activeModelDefinitionId, activeSelectionAccept, boundInteractionSession, cameraNavigatingRef, canvasBinding, dragCleanupRef, dragSelectionRef, effectiveSelectionKindToggles, entityFlagsForId, geometryPreviewTransform, hostPickingEnabled, interactionSelectionByState, onCameraNavigateProp, onHoverTargetProp, onInteractionEventProp, onSelectionRequestProp, rendererSelectionByModel, rt, rtRef, selectablePickTargets, selectionMethod, setCameraNavigating, setDragSelection, setHoveredPickKey, setInteractionSelectionByState, setRendererSelectionByModel, setSelectionMenu, snapshot } = core;
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
  return { applySelectionPrune, dispatchSelectionTargets, onCameraNavigate, onSelectionRequest, onSpatialInteractionEvent, pointerMoveActive };
}

export type ReplCoreAfter3 = ReplCoreAfter2 & ReturnType<typeof useReplSelectionAndPointer>;
// #endregion 🪩️Repl
