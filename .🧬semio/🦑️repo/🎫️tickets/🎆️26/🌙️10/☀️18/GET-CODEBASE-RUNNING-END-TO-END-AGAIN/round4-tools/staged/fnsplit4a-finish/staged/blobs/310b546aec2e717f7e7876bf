/** 🔸️ The publish catalogue drag phase of Canvas2dHost. */
import { useCallback, useEffect, type DragEvent } from "react";
import { CATALOGUE_DRAG_MIME, getActiveCataloguePointerDragData, registerIntroductionSurfaceResolver, sampleBezierSegments, windowElementId, type IntroductionResolvedGeometry } from "@semio-tech/ui-react";
import { worldToScreenLogical } from "../📷️camera-math/🟦️.ts";
import type { useSceneState } from "../🔶️scene/🟦️.tsx";
import { DRAG_OVER_THROTTLE_DISTANCE, DRAG_OVER_THROTTLE_MS, clientPointOverRect } from "../🟢️shared/🟦️.tsx";

/** 🔹 Builds publishCatalogueDragOverAt, handleDragOver, handleDragLeave, publishCatalogueDropAt of Canvas2dHost. */
export function usePublishCatalogueDragState({ windowInstanceId, containerRef, cameraRef, readLayers, dragOverStateRef, dispatch }: Pick<ReturnType<typeof useSceneState>, "windowInstanceId" | "containerRef" | "cameraRef" | "readLayers" | "dragOverStateRef" | "dispatch">) {
  useEffect(() => {
    if (!windowInstanceId) return;
    return registerIntroductionSurfaceResolver(windowElementId(windowInstanceId), {
      canvasPoint: (x, y) => {
        const container = containerRef.current;
        if (!container) return null;
        const rect = container.getBoundingClientRect();
        const screen = worldToScreenLogical(x, y, cameraRef.current, rect.width, rect.height);
        return { x: rect.left + screen.x, y: rect.top + screen.y, visible: true };
      },
      entity: (domain, entityId): IntroductionResolvedGeometry | null => {
        if (domain !== "layer") return null;
        const container = containerRef.current;
        if (!container) return null;
        const rect = container.getBoundingClientRect();
        const layer = entityId === "*" ? readLayers().find((candidate) => candidate.id) : readLayers().find((candidate) => candidate.id === entityId);
        if (!layer) return null;
        const toScreen = (wx: number, wy: number) => {
          const screen = worldToScreenLogical(wx, wy, cameraRef.current, rect.width, rect.height);
          return { x: rect.left + screen.x, y: rect.top + screen.y };
        };
        if (layer.segments && layer.segments.length > 0) {
          const polyline = sampleBezierSegments(layer.segments).map((point) => toScreen(point.x, point.y));
          const point=polyline[Math.floor(polyline.length/2)];if(point)return{point,polyline,visible:true};
        }
        if (layer.points && layer.points.length > 0) {
          const polyline = layer.points.map(([px, py]) => toScreen(px, py));
          const point=polyline[Math.floor(polyline.length/2)];if(point)return{point,polyline,visible:true};
        }
        const bounds =
          layer.x !== undefined && layer.y !== undefined && layer.width !== undefined && layer.height !== undefined
            ? { x: layer.x, y: layer.y, width: layer.width, height: layer.height }
            : layer.x0 !== undefined && layer.y0 !== undefined && layer.x1 !== undefined && layer.y1 !== undefined
              ? { x: Math.min(layer.x0, layer.x1), y: Math.min(layer.y0, layer.y1), width: Math.abs(layer.x1 - layer.x0), height: Math.abs(layer.y1 - layer.y0) }
              : null;
        if (!bounds) return null;
        const topLeft = toScreen(bounds.x, bounds.y);
        const bottomRight = toScreen(bounds.x + bounds.width, bounds.y + bounds.height);
        return {
          point: { x: (topLeft.x + bottomRight.x) / 2, y: (topLeft.y + bottomRight.y) / 2 },
          rect: { x: topLeft.x, y: topLeft.y, width: bottomRight.x - topLeft.x, height: bottomRight.y - topLeft.y },
          visible: true,
        };
      },
    });
  }, [windowInstanceId, readLayers]);

  const publishCatalogueDragOverAt = useCallback(
    (clientX: number, clientY: number, types: readonly string[]): boolean => {
      const rect = containerRef.current?.getBoundingClientRect();
      if (!rect || !clientPointOverRect(clientX, clientY, rect)) return false;
      const x = clientX - rect.left;
      const y = clientY - rect.top;
      const last = dragOverStateRef.current;
      const now = Date.now();
      if (last && now - last.time < DRAG_OVER_THROTTLE_MS && Math.abs(x - last.x) < DRAG_OVER_THROTTLE_DISTANCE && Math.abs(y - last.y) < DRAG_OVER_THROTTLE_DISTANCE) return true;
      dragOverStateRef.current = { x, y, time: now };
      dispatch("canvasDragOver", { x, y, width: rect.width, height: rect.height, types: [...types] });
      return true;
    },
    [dispatch],
  );

  const handleDragOver = useCallback(
    (event: DragEvent<HTMLDivElement>) => {
      if (!event.dataTransfer.types.includes(CATALOGUE_DRAG_MIME)) return;
      event.preventDefault();
      publishCatalogueDragOverAt(event.clientX, event.clientY, event.dataTransfer.types);
    },
    [publishCatalogueDragOverAt],
  );

  const handleDragLeave = useCallback(() => {
    dragOverStateRef.current = null;
    dispatch("canvasDragLeave");
  }, [dispatch]);

  const publishCatalogueDropAt = useCallback(
    (clientX: number, clientY: number, raw: string, pointerTypes?: readonly string[]) => {
      const hadPreview = dragOverStateRef.current !== null;
      dragOverStateRef.current = null;
      const rect = containerRef.current?.getBoundingClientRect();
      if (!rect) return;
      const previewArgs = { x: clientX - rect.left, y: clientY - rect.top, width: rect.width, height: rect.height, types: pointerTypes ? [...pointerTypes] : [] };
      const args = { x: clientX - rect.left, y: clientY - rect.top, width: rect.width, height: rect.height, dragData: raw };
      void (async () => {
        try {
          if (pointerTypes && !hadPreview) await dispatch("canvasDragOver", previewArgs);
        } finally {
          try {
            await dispatch("canvasDragLeave");
          } finally {
            if (raw) await dispatch("canvasDrop", args);
          }
        }
      })().catch(() => undefined);
    },
    [dispatch],
  );

  const handleDrop = useCallback(
    (event: DragEvent<HTMLDivElement>) => {
      if (!event.dataTransfer.types.includes(CATALOGUE_DRAG_MIME)) return;
      const raw = event.dataTransfer.getData(CATALOGUE_DRAG_MIME);
      event.preventDefault();
      publishCatalogueDropAt(event.clientX, event.clientY, raw);
    },
    [publishCatalogueDropAt],
  );

  useEffect(() => {
    const retirePointerPreview = () => {
      if (!dragOverStateRef.current) return;
      dragOverStateRef.current = null;
      dispatch("canvasDragLeave");
    };
    const onPointerMove = (event: PointerEvent) => {
      const data = getActiveCataloguePointerDragData();
      if (!data) {
        retirePointerPreview();
        return;
      }
      if (!publishCatalogueDragOverAt(event.clientX, event.clientY, data.types)) retirePointerPreview();
    };
    const onPointerUp = (event: PointerEvent) => {
      const data = getActiveCataloguePointerDragData();
      if (!data) return;
      const rect = containerRef.current?.getBoundingClientRect();
      if (rect && clientPointOverRect(event.clientX, event.clientY, rect)) {
        publishCatalogueDropAt(event.clientX, event.clientY, data.payload, data.types);
      } else {
        retirePointerPreview();
      }
    };
    const onPointerCancel = () => retirePointerPreview();
    window.addEventListener("pointermove", onPointerMove);
    window.addEventListener("pointerup", onPointerUp, true);
    window.addEventListener("pointercancel", onPointerCancel, true);
    return () => {
      window.removeEventListener("pointermove", onPointerMove);
      window.removeEventListener("pointerup", onPointerUp, true);
      window.removeEventListener("pointercancel", onPointerCancel, true);
    };
  }, [dispatch, publishCatalogueDragOverAt, publishCatalogueDropAt]);
  return { handleDragOver, handleDragLeave, handleDrop };
}
