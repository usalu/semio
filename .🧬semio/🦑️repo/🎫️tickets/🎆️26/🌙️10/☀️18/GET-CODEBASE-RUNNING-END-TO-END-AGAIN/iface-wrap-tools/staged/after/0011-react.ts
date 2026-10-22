/** 🎛️ World3dHost marquee and gumball: the marquee preview and its commit hold, the selection preview, the gumball gesture and handlers and the relocate drag. */
import * as React from "../../⚛️react-port/🟦️.ts";
import { useCallback, useEffect, useLayoutEffect, useMemo, useRef } from "../../⚛️react-port/🟦️.ts";
import { type GumballHandleKind, type GumballPose, type SelectionMarqueePoint } from "@semio-tech/ui-react";
import { type WorldGumballEvent, worldGumballStep } from "../🧮️gumball-math/🟦️.ts";
import { mergeIdSet, raycastGroundPoint, resolveClickInstanceId, resolveMarqueeComponentIds, resolveMarqueeInstanceIds } from "../🎯️marquee-and-bridges/🟦️.tsx";
import { clearWorldCatalogueDropPreview, clearWorldSelectionPreview, setWorldCatalogueDropPreview, setWorldSelectionPreview, world3dRelocateDispatchArgsV1, world3dRelocateDragTargetV1 } from "../🗂️catalogue-drop/🟦️.tsx";
import type { WorldCore5 } from "../🔀️paint-and-projection/🟦️.ts";

// #region World3dHost
/**
 * 🎛️ Wires the marquee preview, the gumball gesture and the relocate drag.
 *
 * 🧯️ Host aborts of a live gesture whose ticks opened the app's tool transaction: window blur and unmount.
 */
export function useWorldMarqueeAndGumball(core: WorldCore5) {
  const { cameraRef, dispatch, dispatchSettled, gestureRecognizer, gridFactor, gridSnapEnabled, gumballDragChainRef, gumballGestureRef, gumballLiveDispatch, hostRef, instances, instancesRef, marqueeCommitHold, marqueeCoverage, marqueeDragActive, marqueeMergeMode, marqueePath, marqueeStartRef, meshes, meshesRef, method, node, relocateMode, relocateSessionRef, selection, selectionArgs, selectionMode, setFaceDragSession, setMarqueeCommitHold, setMarqueePath, sharedSelectionPreview, windowInstanceId } = core;
  const marqueePreview = useMemo<{ readonly mergedComponentIds: readonly number[] | null; readonly mergedInstanceIds: readonly string[] | null }>(() => {
    if (!marqueeDragActive || !hostRef.current || !cameraRef.current) return { mergedComponentIds: null, mergedInstanceIds: null };
    const rect = hostRef.current.getBoundingClientRect();
    const camera = cameraRef.current;
    if (selectionMode === "mesh" || selectionMode === "object") {
      const hits = resolveMarqueeInstanceIds(instances, meshes, marqueePath, rect, camera, method, marqueeCoverage);
      return { mergedComponentIds: null, mergedInstanceIds: mergeIdSet(marqueeMergeMode, selection.ids ?? [], hits) };
    }
    const hits = resolveMarqueeComponentIds(instances, meshes, selectionMode, selection.activeObjectId ?? undefined, marqueePath, rect, camera, method, marqueeCoverage);
    return { mergedComponentIds: mergeIdSet(marqueeMergeMode, selection.componentIds ?? [], hits), mergedInstanceIds: null };
  }, [instances, marqueeCoverage, marqueeDragActive, marqueeMergeMode, marqueePath, meshes, method, selection.activeObjectId, selection.componentIds, selection.ids, selectionMode]);
  const selectionPreviewSourceId = windowInstanceId ?? node.surfaceId;
  const localSelectionPreviewActive = marqueePreview.mergedComponentIds !== null || marqueePreview.mergedInstanceIds !== null;
  const heldSelectionPreviewActive = marqueeCommitHold !== null && (marqueeCommitHold.mergedComponentIds !== null || marqueeCommitHold.mergedInstanceIds !== null);
  useEffect(() => {
    if (!marqueeCommitHold) return;
    const selectionIdSet = new Set(selection.ids ?? []);
    const selectionComponentSet = new Set(selection.componentIds ?? []);
    const instanceMatch =
      marqueeCommitHold.mergedInstanceIds != null &&
      marqueeCommitHold.mergedInstanceIds.length > 0 &&
      marqueeCommitHold.mergedInstanceIds.length === selectionIdSet.size &&
      marqueeCommitHold.mergedInstanceIds.every((id) => selectionIdSet.has(id));
    const componentMatch =
      marqueeCommitHold.mergedComponentIds != null &&
      marqueeCommitHold.mergedComponentIds.length > 0 &&
      marqueeCommitHold.mergedComponentIds.length === selectionComponentSet.size &&
      marqueeCommitHold.mergedComponentIds.every((id) => selectionComponentSet.has(id));
    if (instanceMatch || componentMatch) setMarqueeCommitHold(null);
  }, [marqueeCommitHold, selection.componentIds, selection.ids]);
  useLayoutEffect(() => {
    if (localSelectionPreviewActive) {
      setWorldSelectionPreview(node.controllerId, { sourceId: selectionPreviewSourceId, ...marqueePreview });
    } else if (heldSelectionPreviewActive && marqueeCommitHold) {
      setWorldSelectionPreview(node.controllerId, { sourceId: selectionPreviewSourceId, ...marqueeCommitHold });
    } else {
      clearWorldSelectionPreview(node.controllerId, selectionPreviewSourceId);
    }
  }, [heldSelectionPreviewActive, localSelectionPreviewActive, marqueeCommitHold, marqueePreview, node.controllerId, selectionPreviewSourceId]);
  const marqueePreviewRef = useRef(marqueePreview);
  marqueePreviewRef.current = marqueePreview;
  const marqueeDragActiveRef = useRef(marqueeDragActive);
  marqueeDragActiveRef.current = marqueeDragActive;
  const marqueeFinalizeOnceRef = useRef(false);
  useEffect(
    () => () => {
      clearWorldSelectionPreview(node.controllerId, selectionPreviewSourceId);
    },
    [node.controllerId, selectionPreviewSourceId],
  );
  const visibleSelectionPreview = localSelectionPreviewActive
    ? marqueePreview
    : heldSelectionPreviewActive && marqueeCommitHold
      ? marqueeCommitHold
      : (sharedSelectionPreview ?? marqueePreview);

  /** 🛠️ Steps the gumball gesture by one `worldGumballStep` event and enqueues the dispatch it owes on the gesture chain;
   * a stream tick settles back into the gesture, so the newest pose that arrived meanwhile goes out as the next tick. A
   * skipped delta is logged as a PERMANENT record, not a `[TRACE]` trace: a drag whose pose never moved along its handle
   * commits NOTHING — it once synthesized a fixed 0.5 translate along the handle's axis instead, a document edit the user
   * never made (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B31) — so the cause is read off the gesture. */
  const stepGumballGestureRef = useRef<(event: WorldGumballEvent) => void>(() => undefined);
  stepGumballGestureRef.current = (event: WorldGumballEvent) => {
    const step = worldGumballStep(gumballGestureRef.current, event);
    gumballGestureRef.current = step.gesture;
    if (step.skipped && (event.kind === "release" || step.skipped === "no-selection-ids")) {
      console.info("gumball pose delta skipped", { reason: step.skipped, event: event.kind, transformMode: step.gesture.transformMode ?? selection.transformMode });
    }
    const sent = step.dispatch;
    if (!sent) return;
    gumballDragChainRef.current = gumballDragChainRef.current
      .then(async () => {
        await dispatchSettled(sent.action, sent.args);
      })
      .catch(() => undefined)
      .finally(() => {
        if (sent.args.phase === "stream") stepGumballGestureRef.current({ kind: "settled" });
      });
  };

  const handleGumballDragStart = useCallback(
    (kind: GumballHandleKind, before: GumballPose) => {
      (globalThis as { __gumballDragEntered?: boolean }).__gumballDragEntered = true;
      stepGumballGestureRef.current({ kind: "start", handle: kind, pose: before, targets: selectionArgs(), live: gumballLiveDispatch, transformMode: selection.transformMode });
    },
    [gumballLiveDispatch, selection.transformMode, selectionArgs],
  );

  useEffect(() => {
    const onBlur = () => stepGumballGestureRef.current({ kind: "cancel", reason: "blur" });
    window.addEventListener("blur", onBlur);
    return () => {
      window.removeEventListener("blur", onBlur);
      stepGumballGestureRef.current({ kind: "cancel", reason: "captureLost" });
    };
  }, []);

  const handleGumballDrag = useCallback((kind: GumballHandleKind, pose: GumballPose) => stepGumballGestureRef.current({ kind: "drag", handle: kind, pose }), []);

  const handleGumballDragEnd = useCallback(
    (kind: GumballHandleKind, before: GumballPose, after: GumballPose) => stepGumballGestureRef.current({ kind: "release", handle: kind, before, after, targets: selectionArgs() }),
    [selectionArgs],
  );

  const handleFaceDragStart = useCallback((args: { objectId: string; faceId: number; normal: readonly [number, number, number]; point: readonly [number, number, number]; faceExtent?: readonly [number, number] }) => {
    setFaceDragSession({ objectId: args.objectId, faceId: args.faceId, normal: args.normal, startPoint: args.point, faceExtent: args.faceExtent });
  }, []);

  const toLocalPoint = useCallback((event: React.PointerEvent<HTMLDivElement>): SelectionMarqueePoint => {
    const rect = hostRef.current?.getBoundingClientRect();
    if (!rect) return { x: event.clientX, y: event.clientY };
    return { x: event.clientX - rect.left, y: event.clientY - rect.top };
  }, []);

  /** 🚚️ Takes a Relocate-utility press: grabs the object under it (gated by the selection, see
   * {@link world3dRelocateDragTargetV1}), pins the ground point the drag starts from and paints the shared
   * world ghost at the object's own origin. Answers whether it took the press, so the marquee never opens
   * underneath an in-progress relocate. A LOCKED object is grabbed on purpose — the guest answers the
   * commit with its `selection_locked` notice, which is a visible refusal instead of a dead gesture.
   * A press on empty ground with a live selection is the gesture's BASE POINT, not a miss — see
   * {@link world3dRelocateDragTargetV1} for why the base point never had to be on the object. */
  const beginRelocateDrag = useCallback(
    (event: React.PointerEvent<HTMLDivElement>): boolean => {
      const host = hostRef.current;
      const camera = cameraRef.current;
      if (!host || !camera) return false;
      const rect = host.getBoundingClientRect();
      const from = raycastGroundPoint(event.clientX, event.clientY, rect, camera);
      if (!from) return false;
      const pressed = resolveClickInstanceId(instancesRef.current, meshesRef.current, toLocalPoint(event), rect, camera);
      const objectId = world3dRelocateDragTargetV1(pressed, selection.ids ?? []);
      const instance = objectId ? instancesRef.current.find((entry) => entry.id === objectId) : undefined;
      if (!objectId || !instance) return false;
      const origin = instance.position ?? [instance.x ?? 0, instance.y ?? 0, instance.z ?? 0];
      const objectKind = instance.objectKind ?? "";
      const meshUrl = meshesRef.current.find((mesh) => mesh.id === (instance.meshId ?? instance.id))?.url;
      relocateSessionRef.current = { objectId, origin, from, objectKind, meshUrl };
      event.currentTarget.setPointerCapture?.(event.pointerId);
      setWorldCatalogueDropPreview(node.controllerId, { objectKind, meshUrl, origin });
      return true;
    },
    [node.controllerId, selection.ids, toLocalPoint],
  );

  /** 🚚️ Moves the live relocate ghost to the grabbed object's would-be origin — grid-snapped exactly like a
   * catalogue drop, zero dispatches until release. */
  const updateRelocateDrag = useCallback(
    (event: React.PointerEvent<HTMLDivElement>): boolean => {
      const session = relocateSessionRef.current;
      const host = hostRef.current;
      const camera = cameraRef.current;
      if (!session || !host || !camera) return false;
      const to = raycastGroundPoint(event.clientX, event.clientY, host.getBoundingClientRect(), camera);
      if (!to) return true;
      const args = world3dRelocateDispatchArgsV1(session.objectId, session.origin, session.from, to, { gridSnapEnabled, gridFactor });
      setWorldCatalogueDropPreview(node.controllerId, { objectKind: session.objectKind, meshUrl: session.meshUrl, origin: args?.position ?? session.origin });
      return true;
    },
    [gridFactor, gridSnapEnabled, node.controllerId],
  );

  /** 🚚️ Closes the relocate drag. A pointer-UP event commits ONE absolute `worldRelocate`; `null` (Escape) and
   * a `pointercancel` both drop the ghost and dispatch nothing — the host binds `onPointerCancel` to the same
   * pointer-up handler, so the event kind is the only thing separating a commit from an abort. */
  const endRelocateDrag = useCallback(
    (event: React.PointerEvent<HTMLDivElement> | null): boolean => {
      const session = relocateSessionRef.current;
      if (!session) return false;
      relocateSessionRef.current = null;
      clearWorldCatalogueDropPreview(node.controllerId);
      const host = hostRef.current;
      const camera = cameraRef.current;
      if (!event || event.type === "pointercancel" || !host || !camera) return true;
      const to = raycastGroundPoint(event.clientX, event.clientY, host.getBoundingClientRect(), camera);
      const args = to ? world3dRelocateDispatchArgsV1(session.objectId, session.origin, session.from, to, { gridSnapEnabled, gridFactor }) : null;
      if (args) void Promise.resolve(dispatch("worldRelocate", args));
      return true;
    },
    [dispatch, gridFactor, gridSnapEnabled, node.controllerId],
  );

  useEffect(() => {
    if (!relocateMode) return undefined;
    const onKeyDown = (event: KeyboardEvent): void => {
      if (event.key !== "Escape") return;
      if (endRelocateDrag(null)) event.stopPropagation();
    };
    window.addEventListener("keydown", onKeyDown, true);
    return () => window.removeEventListener("keydown", onKeyDown, true);
  }, [endRelocateDrag, relocateMode]);

  /** 🤏️ Abandons whatever single-pointer gesture is in flight and releases its capture, so the
   * orbit rig owns the surface alone for the rest of a two-finger pinch/pan. */
  const yieldToPinch = useCallback(
    (event: React.PointerEvent<HTMLDivElement>) => {
      for (const tracked of gestureRecognizer.pointers) {
        if (event.currentTarget.hasPointerCapture?.(tracked.pointerId)) event.currentTarget.releasePointerCapture(tracked.pointerId);
      }
      endRelocateDrag(null);
      setMarqueePath([]);
      marqueeStartRef.current = null;
      marqueeFinalizeOnceRef.current = true;
    },
    [endRelocateDrag, gestureRecognizer],
  );
  return { beginRelocateDrag, endRelocateDrag, handleFaceDragStart, handleGumballDrag, handleGumballDragEnd, handleGumballDragStart, marqueeDragActiveRef, marqueeFinalizeOnceRef, marqueePreviewRef, toLocalPoint, updateRelocateDrag, visibleSelectionPreview, yieldToPinch };
}

export type WorldCore6 = WorldCore5 & ReturnType<typeof useWorldMarqueeAndGumball>;
// #endregion World3dHost
