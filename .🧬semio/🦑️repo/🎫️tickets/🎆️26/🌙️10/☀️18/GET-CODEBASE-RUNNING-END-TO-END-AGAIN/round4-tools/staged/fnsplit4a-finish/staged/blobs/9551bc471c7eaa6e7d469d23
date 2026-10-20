/** 🔸️ The handle pointer move phase of useInkPointer. */
import React, { useCallback } from "react";
import { applyPinchToOffsetCamera, inkCanvasActions } from "@semio-tech/framework";
import { screenRectFromPoints } from "@semio-tech/ui-react";
import { type InkCanvasEvent, type InkItem } from "../../🧾️ink-model/🟦️.ts";
import { eraseInkStrokeEventsAtPoint, eraseInkStrokePointEventsNearPoint, findInkItem, inkItemsAtPoint, inkItemsIntersectingRect, inkResizeBounds, inkScaleItemWithinGroup, inkSnapWorldPoint, screenToWorld } from "../../📐️ink-geometry/🟦️.ts";
import { INK_CAMERA_ZOOM_BOUNDS } from "../../🧲️ink-host-state/🟦️.ts";
import type { useAbortGestureState } from "../🔶️abort-gesture/🟦️.ts";

/** 🔹 Builds handlePointerMove, releaseContact, handlePointerUp, handlePointerCancel of useInkPointer. */
export function useHandlePointerMoveState({ rootRef, doc, presencePointerRef, publishInkPresenceView, gestureRecognizer, pinchCameraRef, setDraftDoc, dragState, interactive, publishInteractionHover, dispatch, liveGesture, setMarqueePoints, setDragState, commitGesture, takePendingLive, gestureActiveRef, marqueePoints, publishInteractionSelection, abortGesture }: Pick<ReturnType<typeof useAbortGestureState>, "rootRef" | "doc" | "presencePointerRef" | "publishInkPresenceView" | "gestureRecognizer" | "pinchCameraRef" | "setDraftDoc" | "dragState" | "interactive" | "publishInteractionHover" | "dispatch" | "liveGesture" | "setMarqueePoints" | "setDragState" | "commitGesture" | "takePendingLive" | "gestureActiveRef" | "marqueePoints" | "publishInteractionSelection" | "abortGesture">) {
  const handlePointerMove = useCallback(
    (event: React.PointerEvent<HTMLDivElement>) => {
      if (!rootRef.current || !doc) return;
      const camera = doc.camera;
      const rect = rootRef.current.getBoundingClientRect();
      const screenX = event.clientX - rect.left;
      const screenY = event.clientY - rect.top;
      presencePointerRef.current = screenToWorld(camera, screenX, screenY);
      publishInkPresenceView(false);
      const verdict = gestureRecognizer.move({ pointerId: event.pointerId, x: screenX, y: screenY });
      if (verdict.kind === "pinch") {
        const nextCamera = applyPinchToOffsetCamera(pinchCameraRef.current ?? camera, verdict.step, INK_CAMERA_ZOOM_BOUNDS);
        pinchCameraRef.current = nextCamera;
        setDraftDoc((current) => ({ ...(current ?? doc), camera: nextCamera }));
        return;
      }
      if (verdict.kind !== "single") return;
      const [worldX, worldY] = screenToWorld(camera, screenX, screenY);
      if (!dragState) {
        if (!interactive) return;
        const hits = inkItemsAtPoint(doc.blocks, worldX, worldY);
        const top = hits[0] ?? null;
        publishInteractionHover(top);
        return;
      }
      if (dragState.kind === "pan") {
        const nextCamera = { ...dragState.camera, x: dragState.camera.x + (screenX - dragState.startX), y: dragState.camera.y + (screenY - dragState.startY) };
        setDraftDoc((current) => ({ ...(current ?? doc), camera: nextCamera }));
        dispatch(inkCanvasActions.setCamera, { camera: nextCamera });
        return;
      }
      if (dragState.kind === "move") {
        const dx = (screenX - dragState.startX) / camera.zoom;
        const dy = (screenY - dragState.startY) / camera.zoom;
        const events: InkCanvasEvent[] = [];
        for (const [blockId, origin] of Object.entries(dragState.origins)) {
          const block = findInkItem(doc, blockId);
          if (!block) continue;
          events.push({ operation: "updateBlock", blockId, block: { ...block, x: origin.x + dx, y: origin.y + dy } });
        }
        if (events.length) liveGesture(events, { kind: "drag", ids: Object.keys(dragState.origins), dx, dy });
        return;
      }
      if (dragState.kind === "marquee") {
        setMarqueePoints([dragState.start, { x: screenX, y: screenY }]);
        return;
      }
      if (dragState.kind === "stroke") {
        const block = findInkItem(doc, dragState.blockId);
        if (!block || block.kind !== "stroke") return;
        const localX = worldX - block.x;
        const localY = worldY - block.y;
        liveGesture([{ operation: "updateBlock", blockId: block.id, block: { ...block, points: [...block.points, [localX, localY]] } }]);
        return;
      }
      if (dragState.kind === "eraser") {
        const events = dragState.mode === "eraserStroke" ? eraseInkStrokeEventsAtPoint(doc, worldX, worldY) : eraseInkStrokePointEventsNearPoint(doc, worldX, worldY, doc.eraserRadius ?? 12);
        if (events.length) liveGesture(events);
        return;
      }
      if (dragState.kind === "resize") {
        const dx = (screenX - dragState.startX) / camera.zoom;
        const dy = (screenY - dragState.startY) / camera.zoom;
        const toBounds = inkResizeBounds(dragState.fromBounds, dragState.handle, dx, dy);
        const events: InkCanvasEvent[] = [];
        for (const blockId of dragState.selectedIds) {
          const block = findInkItem(doc, blockId);
          if (!block) continue;
          events.push({ operation: "updateBlock", blockId, block: inkScaleItemWithinGroup(block, dragState.fromBounds, toBounds) });
        }
        if (events.length) liveGesture(events);
      }
    },
    [dispatch, doc, dragState, gestureRecognizer, interactive, liveGesture, publishInteractionHover, publishInkPresenceView],
  );

  /** 🤏️ Routes a lifted/cancelled contact through the shared recognizer: the LAST finger of a pinch
   * commits its camera exactly once; any other contact of a latched gesture is swallowed. Answers whether
   * the single-pointer lane may handle the release. */
  const releaseContact = useCallback(
    (pointerId: number): boolean => {
      const verdict = gestureRecognizer.up(pointerId);
      if (verdict.kind === "pinchEnd" && pinchCameraRef.current) {
        dispatch(inkCanvasActions.setCamera, { camera: pinchCameraRef.current });
        pinchCameraRef.current = null;
      }
      return verdict.kind === "single";
    },
    [dispatch, gestureRecognizer],
  );

  const handlePointerUp = useCallback(() => {
    if (!doc) {
      setDragState(null);
      setMarqueePoints([]);
      return;
    }
    if (dragState?.kind === "move") {
      const ids = Object.keys(dragState.origins);
      const anchor = ids.map((id) => ({ origin: dragState.origins[id]!, block: findInkItem(doc, id) })).find(({ block }) => block !== null);
      if (anchor?.block) {
        const [x, y] = doc.snapEnabled ? inkSnapWorldPoint(anchor.block.x, anchor.block.y, doc.snapGridSpacing ?? 8) : [anchor.block.x, anchor.block.y];
        commitGesture([], { kind: "drag", ids, dx: x - anchor.origin.x, dy: y - anchor.origin.y });
      } else commitGesture([]);
    } else if (dragState?.kind === "stroke") {
      const block = findInkItem(doc, dragState.blockId);
      if (block) commitGesture([{ operation: "updateBlock", blockId: block.id, block }]);
      else commitGesture([]);
    } else if (dragState?.kind === "resize") {
      const events: InkCanvasEvent[] = [];
      for (const blockId of dragState.selectedIds) {
        const block = findInkItem(doc, blockId);
        if (block) events.push({ operation: "updateBlock", blockId, block });
      }
      commitGesture(events);
    } else if (dragState?.kind === "eraser") {
      commitGesture([]);
    } else if (dragState?.kind === "pan") {
      takePendingLive();
      gestureActiveRef.current = false;
    }
    if (dragState?.kind === "marquee" && marqueePoints.length >= 2 && rootRef.current) {
      const screenRect = screenRectFromPoints(marqueePoints);
      if (screenRect) {
        const camera = doc.camera;
        const worldRect = { x: (screenRect.x - camera.x) / camera.zoom, y: (screenRect.y - camera.y) / camera.zoom, width: screenRect.width / camera.zoom, height: screenRect.height / camera.zoom };
        const blocks = inkItemsIntersectingRect(doc.blocks, worldRect).map((id) => findInkItem(doc, id)).filter((block): block is InkItem => block !== null);
        publishInteractionSelection(blocks, "replace", "rectangle");
      }
    }
    setDragState(null);
    setMarqueePoints([]);
  }, [commitGesture, doc, dragState, marqueePoints, publishInteractionSelection, takePendingLive]);

  const handlePointerCancel = useCallback(() => {
    abortGesture();
    setDraftDoc(null);
    setDragState(null);
    setMarqueePoints([]);
  }, [abortGesture]);
  return { handlePointerCancel, handlePointerMove, handlePointerUp, releaseContact };
}
