// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/Canvas2dHost/component.tsx
/** 🖌️ `Canvas2dHost` — canvas-2d `ComponentSceneHost`: interprets a `draw.document`-shaped
 * `layersJson` scene into an HTML canvas and dispatches pointer/drag/context-menu actions back to the
 * plugin. The pan/zoom camera math shared with `Paint2dHost` (`📷️camera-math`), the layer painting
 * (`🖍️layer-paint`), the pointer gesture lane (`👆️pointer-lane`) and the JSON layers session
 * (`🗂️layers-session`) live in the child folders beside this file. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { useCallback, useContext, useEffect, useMemo, useRef, useState, type DragEvent, type MouseEvent, type PointerEvent as ReactPointerEvent } from "../⚛️react-port/🟦️.ts";
import { WasmCanvas } from "@semio-tech/framework/canvas/react-renderer";
import { ContextMenuController, CATALOGUE_DRAG_MIME, getActiveCataloguePointerDragData, registerIntroductionSurfaceResolver, sampleBezierSegments, windowElementId, useLabel, type IntroductionResolvedGeometry } from "@semio-tech/ui-react";
import { type ComponentSceneHostProps } from "@semio-tech/framework";
// 🐢️ Direct element-to-element import — `🟦️Interpreter` and `Canvas2dHost` landed in the same batch.
import { WindowInstanceIdContext } from "../🗣️Interpreter/🌉️scene-host-contract/🟦️.ts";
import { useMapContextMenuSpecs, useShellContextMenuFallback, openSurfaceContextMenu, type SurfaceContextMenuResult } from "../🗣️Interpreter/🖱️surface-menu/🟦️.ts";
import { Canvas2dGumballOverlay } from "./🧭️gumball/🟦️.tsx";
import { CanvasPresenceOverlayV1, useLocalPresenceActorIdV1, usePresenceRuntimeKeyV1 } from "../👕️canvas-presence/🟦️.tsx";
import { PRESENCE_VIEW_PUBLISH_MIN_INTERVAL_MS, clearLocalPresenceWindowViewV1, publishLocalPresenceWindowViewV1 } from "../👕️canvas-presence/🟦️.ts";
import { type CanvasCamera, screenToWorldLogical, worldToScreenLogical } from "./📷️camera-math/🟦️.ts";
import type { CanvasLayerRecord } from "./🖍️layer-paint/🟦️.ts";
import { JsonLayersCanvasSession } from "./🗂️layers-session/🟦️.ts";

// #endregion 🔌️Adapters
//#region 🔖️Canvas2dHost
//#region Canvas2dHost
// 🐢️ `CAMERA_SYNC_DEBOUNCE_MS` is also referenced (unqualified) by `World3dHost`'s still-barrel-resident
// camera-dispatch debounce — exported so that outside reference keeps resolving after this extraction.
export const CAMERA_SYNC_DEBOUNCE_MS = 120;
const DRAG_OVER_THROTTLE_MS = 50;
const DRAG_OVER_THROTTLE_DISTANCE = 4;

function clientPointOverRect(clientX: number, clientY: number, rect: DOMRect): boolean {
  return clientX >= rect.left && clientX <= rect.right && clientY >= rect.top && clientY <= rect.bottom;
}

export function Canvas2dHost({ node, onAction, requestContextMenu }: ComponentSceneHostProps) {
  const scene = node.canvas2d;
  const windowInstanceId = useContext(WindowInstanceIdContext);
  const emptySceneLabel = useLabel("ui.host.emptyScene");
  const initialCamera = useMemo(() => ({ x: scene?.cameraX ?? 0, y: scene?.cameraY ?? 0, zoom: scene?.zoom ?? 1 }), [scene?.cameraX, scene?.cameraY, scene?.zoom]);
  const cameraRef = useRef<CanvasCamera>(initialCamera);
  const framingRef = useRef(scene?.framing);
  framingRef.current = scene?.framing;
  const sessionRef = useRef<JsonLayersCanvasSession | null>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const cameraSyncTimeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const dragOverStateRef = useRef<{ x: number; y: number; time: number } | null>(null);
  const [contextMenu, setContextMenu] = useState<(SurfaceContextMenuResult & { readonly x: number; readonly y: number }) | null>(null);
  const [viewportSize, setViewportSize] = useState({ width: 0, height: 0 });
  const [presenceCamera, setPresenceCamera] = useState<CanvasCamera>(initialCamera);
  const presenceRuntimeKey = usePresenceRuntimeKeyV1();
  const presenceWindowId = windowInstanceId ?? node.surfaceId ?? "canvas";
  const localPresenceActor = useLocalPresenceActorIdV1(presenceRuntimeKey);
  const presencePointerRef = useRef<readonly [number, number] | null>(null);
  const presencePublishAtRef = useRef(0);
  /** 👕️ Publishes this window's camera and the pointer in WORLD coordinates for the next presence beat (throttled), so a
   * peer paints the cursor at the same drawing position whatever its own camera. */
  const publishCanvasPresenceView = useCallback((force: boolean) => {
    const now = Date.now();
    if (!force && now - presencePublishAtRef.current < PRESENCE_VIEW_PUBLISH_MIN_INTERVAL_MS) return;
    presencePublishAtRef.current = now;
    const rect = containerRef.current?.getBoundingClientRect();
    const camera = cameraRef.current;
    const pointer = presencePointerRef.current;
    if (!presenceRuntimeKey) return;
    publishLocalPresenceWindowViewV1(presenceRuntimeKey, presenceWindowId, {
      windowId: presenceWindowId,
      space: "canvas",
      kind: { kind: "canvas", x: camera.x, y: camera.y, zoom: camera.zoom },
      size: [rect?.width ?? 0, rect?.height ?? 0],
      ...(pointer ? { pointer: [pointer[0], pointer[1], 0] as const } : {}),
    });
  }, [presenceWindowId, presenceRuntimeKey]);
  useEffect(() => {
    const timer = setTimeout(() => publishCanvasPresenceView(true), PRESENCE_VIEW_PUBLISH_MIN_INTERVAL_MS);
    return () => clearTimeout(timer);
  }, [publishCanvasPresenceView, presenceCamera]);
  useEffect(() => presenceRuntimeKey ? () => clearLocalPresenceWindowViewV1(presenceWindowId, presenceRuntimeKey) : undefined, [presenceWindowId, presenceRuntimeKey]);
  const onPresencePointerMove = useCallback((event: ReactPointerEvent<HTMLDivElement>) => {
    const rect = containerRef.current?.getBoundingClientRect();
    if (!rect) return;
    const world = screenToWorldLogical(event.clientX - rect.left, event.clientY - rect.top, cameraRef.current, rect.width, rect.height);
    presencePointerRef.current = [world.x, world.y];
    publishCanvasPresenceView(false);
  }, [publishCanvasPresenceView]);
  const onPresencePointerLeave = useCallback(() => {
    presencePointerRef.current = null;
    publishCanvasPresenceView(true);
  }, [publishCanvasPresenceView]);
  const activeUtility = useMemo(() => {
    try {
      const layers = JSON.parse(scene?.layersJson ?? "[]") as readonly { readonly role?: string; readonly utility?: string }[];
      return layers.find((layer) => layer.role === "meta")?.utility;
    } catch {
      return undefined;
    }
  }, [scene?.layersJson]);
  const contextMenuTitleLabel = useLabel(contextMenu?.titleKey ?? "ui.surfaceContextMenu.canvas");
  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>): void | Promise<unknown> => {
      const ownedArgs = { surfaceId: node.surfaceId, ...args };
      return onAction({ controllerId: node.controllerId, action, args: ownedArgs });
    },
    [node.controllerId, node.surfaceId, onAction],
  );
  const mapContextMenu = useMapContextMenuSpecs(dispatch);
  const shellContextMenuFallback = useShellContextMenuFallback();
  const sessionFactory = useMemo(() => {
    return () => {
      const session = new JsonLayersCanvasSession(
        () => layersJsonRef.current ?? "[]",
        cameraRef.current,
        (next) => {
          cameraRef.current = next;
          sessionRef.current?.updateCamera(next);
          setPresenceCamera(next);
          publishCanvasPresenceView(false);
          if (cameraSyncTimeoutRef.current) clearTimeout(cameraSyncTimeoutRef.current);
          cameraSyncTimeoutRef.current = setTimeout(() => dispatch("setCamera", { camera: next }), CAMERA_SYNC_DEBOUNCE_MS);
        },
        (action, args) => dispatch(action, args),
        () => framingRef.current,
      );
      sessionRef.current = session;
      return session;
    };
  }, [dispatch, publishCanvasPresenceView]);

  useEffect(() => {
    return () => {
      if (cameraSyncTimeoutRef.current) clearTimeout(cameraSyncTimeoutRef.current);
      cameraSyncTimeoutRef.current = null;
    };
  }, [dispatch]);

  useEffect(() => {
    sessionRef.current?.syncLayersJson();
  }, [scene?.layersJson, scene?.framing]);

  const layersJsonRef = useRef(scene?.layersJson);
  layersJsonRef.current = scene?.layersJson;
  const layersCacheRef = useRef<{ readonly json: string; readonly layers: readonly CanvasLayerRecord[] } | null>(null);

  /** 🐢️ `layersJson` can be sizeable and the resolver is called every animation frame while a
   * demonstration targets this surface — cache the parse, keyed by the source string's identity, so a
   * re-render with the same layers never re-parses. */
  const readLayers = useCallback((): readonly CanvasLayerRecord[] => {
    const json = layersJsonRef.current ?? "[]";
    const cached = layersCacheRef.current;
    if (cached && cached.json === json) return cached.layers;
    let layers: readonly CanvasLayerRecord[] = [];
    try {
      const parsed = JSON.parse(json) as unknown;
      layers = Array.isArray(parsed) ? (parsed as CanvasLayerRecord[]) : [];
    } catch {
      layers = [];
    }
    layersCacheRef.current = { json, layers };
    return layers;
  }, []);

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

  //#region ContextMenu
  /** 🖱️ No layer pick/selection is tracked at this level (`Canvas2dScene` carries only camera + `layersJson`) — `hits`/`selection` stay empty per surface convention until layer picking lands here. */
  const onContextMenu = useCallback(
    (event: MouseEvent<HTMLDivElement>): void => {
      if (!requestContextMenu) return;
      event.preventDefault();
      event.stopPropagation();
      void (async () => {
        const menu = await openSurfaceContextMenu(
          requestContextMenu,
          {
            menu: { id: "canvas2d", args: null },
            surface: { surfaceId: node.surfaceId, kind: "canvas2d", hits: [], selection: [] },
            windowInstanceId: windowInstanceId ?? undefined,
            point: { x: event.clientX, y: event.clientY },
          },
          mapContextMenu,
          shellContextMenuFallback,
        );
        setContextMenu({ x: event.clientX, y: event.clientY, ...menu });
      })();
    },
    [mapContextMenu, node.surfaceId, requestContextMenu, shellContextMenuFallback, windowInstanceId],
  );
  //#endregion ContextMenu

  useEffect(() => {
    const element = containerRef.current;
    if (!element) return;
    const observer = new ResizeObserver((entries) => {
      const entry = entries[0];
      if (!entry) return;
      setViewportSize({ width: entry.contentRect.width, height: entry.contentRect.height });
    });
    observer.observe(element);
    return () => observer.disconnect();
  }, []);

  if (!scene) return <div className="semio-canvas-2d-empty">{emptySceneLabel}</div>;

  return (
    <div
      ref={containerRef}
      className="semio-canvas-2d-host relative h-full min-h-[24rem] w-full ui-surface"
      data-level="base"
      data-controller-id={node.controllerId}
      data-surface-id={node.surfaceId}
      onDragOver={handleDragOver}
      onDragLeave={handleDragLeave}
      onDrop={handleDrop}
      onContextMenu={onContextMenu}
      onPointerMove={onPresencePointerMove}
      onPointerLeave={onPresencePointerLeave}
    >
      <WasmCanvas className="h-full w-full" sessionFactory={sessionFactory} />
      <Canvas2dGumballOverlay
        layersJson={scene.layersJson}
        activeUtility={activeUtility}
        camera={cameraRef.current}
        viewportWidth={viewportSize.width}
        viewportHeight={viewportSize.height}
        onDispatch={(action, args) => dispatch(action, args)}
      />
      <CanvasPresenceOverlayV1
        runtimeKey={presenceRuntimeKey}
        windowId={presenceWindowId}
        space="canvas"
        myActor={localPresenceActor ?? ""}
        locale={typeof document !== "undefined" ? document.documentElement.lang : undefined}
        localCanvas={presenceCamera}
        localSizePx={[viewportSize.width, viewportSize.height]}
        scenePath={`canvas/${presenceWindowId}`}
      />
      <ContextMenuController
        title={contextMenuTitleLabel}
        open={contextMenu != null}
        position={contextMenu ?? { x: 0, y: 0 }}
        items={contextMenu?.items ?? []}
        onOpenChange={(open) => {
          if (!open) setContextMenu(null);
        }}
      />
    </div>
  );
}
//#endregion Canvas2dHost
//#endregion 🔖️Canvas2dHost
