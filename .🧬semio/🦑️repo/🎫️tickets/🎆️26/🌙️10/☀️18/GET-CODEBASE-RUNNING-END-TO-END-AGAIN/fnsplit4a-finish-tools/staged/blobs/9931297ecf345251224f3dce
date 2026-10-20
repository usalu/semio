/** 🟠️ The handle wheel phase of useInkPointer. */
import React, { useCallback } from "react";
import { inkCanvasActions } from "@semio-tech/framework";
import { inkItemsAtPoint, inkMaybeSnapWorldPoint, inkTableCellAtPoint, screenToWorld } from "../../📐️ink-geometry/🟦️.ts";
import { createInkItemByKind } from "../../🌳️ink-document-ops/🟦️.ts";
import { INK_CAMERA_ZOOM_BOUNDS } from "../../🧲️ink-host-state/🟦️.ts";
import type { useAbortGestureState } from "../🔶️abort-gesture/🟦️.ts";

/** 🟡 Builds handleWheel, handleDoubleClick of useInkPointer. */
export function useHandleWheelState({ rootRef, doc, isNavigator, setDraftDoc, dispatch, interactive, setTableEdit, setTextEdit, publishInteractionSelection, atomicGesture }: Pick<ReturnType<typeof useAbortGestureState>, "rootRef" | "doc" | "isNavigator" | "setDraftDoc" | "dispatch" | "interactive" | "setTableEdit" | "setTextEdit" | "publishInteractionSelection" | "atomicGesture">) {
  const handleWheel = useCallback(
    (event: React.WheelEvent<HTMLDivElement>) => {
      if (!rootRef.current || !doc || isNavigator) return;
      event.preventDefault();
      const camera = doc.camera;
      const rect = rootRef.current.getBoundingClientRect();
      const screenX = event.clientX - rect.left;
      const screenY = event.clientY - rect.top;
      const zoomFactor = event.deltaY < 0 ? 1.08 : 0.92;
      const nextZoom = Math.min(INK_CAMERA_ZOOM_BOUNDS.max, Math.max(INK_CAMERA_ZOOM_BOUNDS.min, camera.zoom * zoomFactor));
      const worldX = (screenX - camera.x) / camera.zoom;
      const worldY = (screenY - camera.y) / camera.zoom;
      const nextCamera = { x: screenX - worldX * nextZoom, y: screenY - worldY * nextZoom, zoom: nextZoom };
      setDraftDoc((current) => ({ ...(current ?? doc), camera: nextCamera }));
      dispatch(inkCanvasActions.setCamera, { camera: nextCamera });
    },
    [dispatch, doc, isNavigator],
  );

  const handleDoubleClick = useCallback(
    (event: React.MouseEvent<HTMLDivElement>) => {
      if (!rootRef.current || !doc || isNavigator || !interactive) return;
      const camera = doc.camera;
      const rect = rootRef.current.getBoundingClientRect();
      const screenX = event.clientX - rect.left;
      const screenY = event.clientY - rect.top;
      const [worldX, worldY] = screenToWorld(camera, screenX, screenY);
      const hits = inkItemsAtPoint(doc.blocks, worldX, worldY);
      const top = hits[0];
      if (top?.kind === "text" && !top.locked) {
        setTableEdit(null);
        setTextEdit({ blockId: top.id });
        publishInteractionSelection([top], "replace", "pick");
        return;
      }
      if (top?.kind === "table" && !top.locked) {
        const cell = inkTableCellAtPoint(top, worldX - top.x, worldY - top.y);
        if (!cell) return;
        setTextEdit(null);
        setTableEdit({ blockId: top.id, row: cell.row, col: cell.col });
        publishInteractionSelection([top], "replace", "pick");
        return;
      }
      if (top) return;
      const [placeX, placeY] = inkMaybeSnapWorldPoint(doc, worldX, worldY);
      const block = createInkItemByKind("text", placeX, placeY);
      atomicGesture([{ operation: "addBlock", block }], [block.id]);
      setTextEdit({ blockId: block.id, created: true });
    },
    [atomicGesture, doc, interactive, isNavigator, publishInteractionSelection],
  );
  return { handleDoubleClick, handleWheel };
}
