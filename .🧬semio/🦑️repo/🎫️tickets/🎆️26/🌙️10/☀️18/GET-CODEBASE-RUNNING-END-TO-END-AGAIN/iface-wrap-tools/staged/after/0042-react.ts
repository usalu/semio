/** 🔸️ The meshes ref phase of useWorldFitAndPresence. */
import { useCallback, useEffect, useMemo, useRef, useState } from "../../../⚛️react-port/🟦️.ts";
import { marqueeCoverageFromGesture, useLabel, type SelectionMarqueeCoverage, type SelectionMarqueePoint } from "@semio-tech/ui-react";
import { GestureRecognizer } from "@semio-tech/framework";
import { DEFAULT_LOD_GRID_FACTOR, type WorldCameraState } from "@semio-tech/framework/infinite/world/r3f";
import { CAMERA_SYNC_DEBOUNCE_MS } from "../../../📐️Canvas2dHost/🟦️.tsx";
import { createCoalescingActionDispatcher } from "../../../🛠️ShellHelpers/🛑️extension-io/🟦️.ts";
import { suggestionMenuOwnsWindow, suggestionPopupOwnsWindow } from "../../../🎣️suggestion-submenu/🟦️.ts";
import { mergeWorldViewportCamera, worldCameraSetCameraDispatchArgs } from "../../🔭️scene-schema/🟦️.ts";
import { WORLD3D_DEFAULT_MARKER_GRANULARITY, world3dMarkerInteractionTarget } from "../../🔗️instance-selection/🟦️.ts";
import { gumballConfigForTransformMode, WORLD_GUMBALL_IDLE, type WorldGumballGesture } from "../../🧮️gumball-math/🟦️.ts";
import { MARQUEE_DRAG_THRESHOLD_PX, paneSuffixFromSurfaceId, resolveWorldMergeMode } from "../../🎯️marquee-and-bridges/🟦️.tsx";
import { type World3dRelocateSession } from "../../🗂️catalogue-drop/🟦️.tsx";
import { world3dHoverActionArgs, world3dSelectionActionArgs } from "../../🖱️interaction-domain-args/🟦️.ts";
import type { useAttractionsState } from "../🔶️attractions/🟦️.ts";

/** 🔹 Builds meshesRef, marqueeStartRef, gestureRecognizer, vorticesRef of useWorldFitAndPresence. */
export function useMeshesRefState({ instancesRef, instances, meshes, vortices, references, selection, lod, interaction, windowInstanceId, marqueePath, marqueeModifiers, persistentSelectionMode, onAction, node, setViewportCamera, sceneCamera, retainViewport, setViewportOwned, setDetachEpoch, lastDispatchedWorldCameraRef, activeUtility, interactionDomainId }: Pick<ReturnType<typeof useAttractionsState>, "instancesRef" | "instances" | "meshes" | "vortices" | "references" | "selection" | "lod" | "interaction" | "windowInstanceId" | "marqueePath" | "marqueeModifiers" | "persistentSelectionMode" | "onAction" | "node" | "setViewportCamera" | "sceneCamera" | "retainViewport" | "setViewportOwned" | "setDetachEpoch" | "lastDispatchedWorldCameraRef" | "activeUtility" | "interactionDomainId">) {
  instancesRef.current = instances;
  const meshesRef = useRef(meshes);
  meshesRef.current = meshes;
  const marqueeStartRef = useRef<SelectionMarqueePoint | null>(null);
  /** 🤏️ The shared `👆️gesture` recognizer for this viewport. A second finger hands the surface to the
   * orbit rig's pinch (the same recognizer inside `WorldOrbitControlsBridge` drives the camera): the
   * host's single-pointer lane — marquee, pick, paint stroke, relocate, engagement — stays quiet until
   * the LAST finger lifts, so the finger left down never grows a marquee or replays a pick. */
  const [gestureRecognizer] = useState(() => new GestureRecognizer());
  const vorticesRef = useRef(vortices);
  vorticesRef.current = vortices;
  const referencesRef = useRef(references);
  referencesRef.current = references;
  const wasMarqueeDragRef = useRef(false);
  const [marqueeCommitHold, setMarqueeCommitHold] = useState<{
    readonly mergedComponentIds: readonly number[] | null;
    readonly mergedInstanceIds: readonly string[] | null;
  } | null>(null);
  const connectDropConsumedRef = useRef(false);
  const engagementPointerMoveInFlightRef = useRef(false);
  const engagementPointerMoveLastPointRef = useRef<readonly [number, number, number] | null>(null);
  /** 🚚️ Open Relocate-utility drag, or `null`. A ref, not state: mid-drag the only thing that changes is
   * the shared ghost origin, so the gesture never re-renders this host by itself (same discipline as the
   * gumball's local preview). */
  const relocateSessionRef = useRef<World3dRelocateSession | null>(null);
  /** 🕹️ The gumball gesture, stepped by `worldGumballStep`: its ids are the ones it GRABBED, pinned at drag start.
   * Resolving them per dispatch read whatever the selection happened to be when the delta was enqueued — and a gumball
   * drag routinely changes the selection under itself (a press that misses the handle is a canvas pick that clears it).
   * An id-less delta reaches the guest, matches no document object, and completes with a refusal notice instead of an
   * edit: the long-standing `gumball-scene-delta` red, where `data-instances-json` never moved. */
  const gumballGestureRef = useRef<WorldGumballGesture>(WORLD_GUMBALL_IDLE);
  /** 🧲️ Serialized gesture chain — mid-drag is local-only unless the selection record asks for live dispatch;
   * either way every delta, the live `commit` and a host `abort` ride this one FIFO. */
  const gumballDragChainRef = useRef(Promise.resolve());
  const gumballLiveDispatch = Boolean(selection.gumballLiveDispatch);
  const selectionMode = selection.selectionMode ?? selection.granularity ?? "mesh";
  const gridSnapEnabled = lod.gridSnapEnabled ?? false;
  const suggestionMenuOpen = Boolean(interaction.suggestionMenu?.open);
  const suggestionMenuOwnsThisWindow = suggestionMenuOwnsWindow(interaction.suggestionMenu, windowInstanceId ?? undefined);
  const suggestionPopupOwnsThisWindow = suggestionPopupOwnsWindow(interaction.suggestionMenu, windowInstanceId ?? undefined);
  /** 🔦️ The trace record of the suggestion row under the pointer — the viewport draws only that candidate. */
  const [suggestionFocus, setSuggestionFocus] = useState<bigint | null>(null);
  const suggestionMenuCheckingPlacementLabel = useLabel("ui.host.checkingPlacement");
  const suggestionMenuNoPlacementLabel = useLabel("ui.host.noPlacement");
  const suggestionMenuTitleLabel = useLabel("ui.surfaceContextMenu.placementSuggestions");
  const gridFactor = lod.gridFactor ?? interaction.gridFactor ?? DEFAULT_LOD_GRID_FACTOR;
  const marqueeDown = marqueePath.length > 0;
  const method = selection.method ?? "rectangle";
  const marqueeStart = marqueePath[0];
  const marqueeEnd = marqueePath[marqueePath.length - 1];
  const marqueeDragActive = marqueeDown && marqueePath.length > 1 && marqueeStart != null && marqueeEnd != null && Math.hypot(marqueeEnd.x - marqueeStart.x, marqueeEnd.y - marqueeStart.y) > MARQUEE_DRAG_THRESHOLD_PX;
  const marqueeMergeMode = useMemo(() => resolveWorldMergeMode(selection.selectionMergeMode, marqueeModifiers, persistentSelectionMode), [marqueeModifiers, selection.selectionMergeMode, persistentSelectionMode]);
  const marqueeCoverage: SelectionMarqueeCoverage = useMemo(() => {
    if (!marqueeDragActive || !marqueeStart || !marqueeEnd) return "full";
    return marqueeCoverageFromGesture({ method, startX: marqueeStart.x, endX: marqueeEnd.x, path: marqueePath });
  }, [marqueeDragActive, marqueeEnd, marqueePath, marqueeStart, method]);

  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      onAction({
        controllerId: node.controllerId,
        action,
        args: { surfaceId: node.surfaceId, windowId: windowInstanceId ?? node.surfaceId, ...args },
      });
    },
    [node.controllerId, node.surfaceId, onAction, windowInstanceId],
  );

  /** 🏁️ `dispatch`'s awaitable twin for every SELF-GATING lane — the coalescing hover and brush target
   * dispatchers. `dispatch` discards `onAction`'s promise, and `onAction` is what settles
   * on the guest's `OperationCompleted` frame, so each gate built on "at most one round trip outstanding"
   * (`createCoalescingActionDispatcher`, `createInFlightSkippingInterval`) cleared itself on the next
   * microtask and gated NOTHING. Wave B33 measured one 70-move brush hover storm enqueuing 72
   * `interactionHover` + 85 background tick turns with 11/10 of them settled, after which the next user
   * action (`addTargetVolume`) waited behind the backlog past its 30 s budget
   * (`📓️2026-09-12-wave-B33-full-run-vs-fresh-lane.md` §3). Same envelope as `dispatch`, awaitable — the
   * shape `dispatchBrushMesh` already uses for the mesh page lane. */
  const dispatchSettled = useCallback(
    (action: string, args?: Record<string, unknown>) =>
      Promise.resolve(
        onAction({
          controllerId: node.controllerId,
          action,
          args: { surfaceId: node.surfaceId, windowId: windowInstanceId ?? node.surfaceId, ...args },
        }),
      ),
    [node.controllerId, node.surfaceId, onAction, windowInstanceId],
  );

  const adoptViewportCamera = useCallback(
    (next: WorldCameraState, applyToRig: boolean) => {
      setViewportCamera((prev) => {
        const merged = mergeWorldViewportCamera(prev ?? sceneCamera, next);
        retainViewport({ camera: merged, viewportOwned: true });
        return merged;
      });
      setViewportOwned(true);
      if (applyToRig) setDetachEpoch((epoch) => epoch + 1);
    },
    [retainViewport, sceneCamera],
  );

  const cameraDispatchTimeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const dispatchWorldCameraDebounced = useCallback(
    (state: WorldCameraState) => {
      if (cameraDispatchTimeoutRef.current) clearTimeout(cameraDispatchTimeoutRef.current);
      cameraDispatchTimeoutRef.current = setTimeout(() => {
        cameraDispatchTimeoutRef.current = null;
        lastDispatchedWorldCameraRef.current = state;
        retainViewport({ lastDispatchedCamera: state });
        dispatch("setCamera", worldCameraSetCameraDispatchArgs(windowInstanceId ?? node.surfaceId, state));
      }, CAMERA_SYNC_DEBOUNCE_MS);
    },
    [dispatch, node.surfaceId, retainViewport, windowInstanceId],
  );
  useEffect(
    () => () => {
      if (cameraDispatchTimeoutRef.current) clearTimeout(cameraDispatchTimeoutRef.current);
    },
    [],
  );

  const referenceSelectedIds = useMemo(() => {
    if (!selection.referenceSelectedId) return new Set<string>();
    return new Set([selection.referenceSelectedId]);
  }, [selection.referenceSelectedId]);

  const targetVolumeSelectedIds = useMemo(() => new Set(selection.targetVolumeIds ?? []), [selection.targetVolumeIds]);

  const volumeGumballConfig = useMemo(() => {
    if (activeUtility !== "transform") return undefined;
    return selection.gumballConfig ?? gumballConfigForTransformMode("transform");
  }, [activeUtility, selection.gumballConfig]);

  const referenceHoveredId = useMemo(() => {
    const hovered = selection.hoveredId;
    if (!hovered?.startsWith("reference:")) return null;
    return hovered.slice("reference:".length);
  }, [selection.hoveredId]);

  const handleReferenceSelect = useCallback(
    (id: string) => {
      const reference = references.find((entry) => entry.id === id);
      if (interactionDomainId) {
        const target = world3dMarkerInteractionTarget("reference", id, reference);
        dispatch("interactionSelect", world3dSelectionActionArgs(interactionDomainId, target.granularity, reference?.locked ? [] : [target.id], "replace"));
        return;
      }
      if (reference?.locked) {
        dispatch("worldPick", {
          granularity: selectionMode,
          id: null,
          merge: "replace",
        });
        return;
      }
      dispatch("setReferenceSelection", {
        pane: paneSuffixFromSurfaceId(node.surfaceId),
        referenceId: id,
      });
    },
    [dispatch, interactionDomainId, node.surfaceId, references, selectionMode],
  );

  /** 🏁️ Reference-marker hover is a POINTERMOVE lane like the instance and vortex ones, so it coalesces onto
   * the latest marker and keeps one round trip outstanding — it used to dispatch one guest turn per move with
   * no gate at all, the same unbounded shape wave B33 measured on the vortex lane (§3). The record lookup
   * reads `referencesRef` so a republished scene cannot rebuild the dispatcher and drop its in-flight state. */
  const handleReferenceHover = useMemo(
    () =>
      createCoalescingActionDispatcher<string | null>((id) => {
        if (interactionDomainId) {
          const target = id ? world3dMarkerInteractionTarget("reference", id, referencesRef.current.find((entry) => entry.id === id)) : null;
          return dispatchSettled("interactionHover", world3dHoverActionArgs(interactionDomainId, target?.granularity ?? WORLD3D_DEFAULT_MARKER_GRANULARITY.reference, target?.id));
        }
        if (!id) return dispatchSettled("referenceHover", {});
        return dispatchSettled("referenceHover", { referenceId: id });
      }),
    [dispatchSettled, interactionDomainId],
  );
  return { dispatch, selectionMode, adoptViewportCamera, connectDropConsumedRef, dispatchSettled, dispatchWorldCameraDebounced, engagementPointerMoveInFlightRef, engagementPointerMoveLastPointRef, gestureRecognizer, gridFactor, gridSnapEnabled, gumballDragChainRef, gumballGestureRef, gumballLiveDispatch, handleReferenceHover, handleReferenceSelect, marqueeCommitHold, marqueeCoverage, marqueeDown, marqueeDragActive, marqueeEnd, marqueeMergeMode, marqueeStart, marqueeStartRef, meshesRef, method, referenceHoveredId, referenceSelectedIds, relocateSessionRef, setMarqueeCommitHold, setSuggestionFocus, suggestionFocus, suggestionMenuCheckingPlacementLabel, suggestionMenuNoPlacementLabel, suggestionMenuOpen, suggestionMenuOwnsThisWindow, suggestionMenuTitleLabel, suggestionPopupOwnsThisWindow, targetVolumeSelectedIds, volumeGumballConfig, vorticesRef, wasMarqueeDragRef };
}
