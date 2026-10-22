/** 📏️ Replacing an unpresentable canvas and the single owner of both canvases' backing-store size.
 *
 * 🧱️ Child hook module of {@link ../🟦️.tsx | FlowGraphCanvasHost}: `useFlowCanvasSurface` runs the original statements in order and returns the bindings later hooks and the render read; the component merges them into the shared core object.
 */

import { useCallback, useEffect } from "../../⚛️react-port/🟦️.ts";
import { resizeCanvasBackingStore } from "../🖼️dag-overlays/🟦️.tsx";
import { observeFlowTask } from "../🔄️flow-session-sync/🟦️.ts";
import type { FlowCoreAfterFlowOverlayAndRender } from "../🎨️flow-overlay-render/🟦️.ts";
//#region 🔖️NodeGraphHost
//#region 🔖️flow-graph-canvas-host
//#region FlowGraphCanvasHost

export type FlowCoreAfterFlowCanvasSurface = FlowCoreAfterFlowOverlayAndRender & ReturnType<typeof useFlowCanvasSurface>;

export function useFlowCanvasSurface(core: FlowCoreAfterFlowOverlayAndRender) {
  const {
    surfaceId, sessionRef, surfaceReadyRef, gpuCanvasRef, labelCanvasRef, containerRef, setCanvasGeneration,
    attachedSurfaceRef, replaceUnpresentableCanvasRef, setContainerSize, paintOverlays, renderFlow
  } = core;
  /** ♻️ Retires a canvas that can no longer be presented onto and mints its successor.
   *
   * The guest owns exactly one surface at a time and refuses a replacement that was not closed
   * first (`attach_surface` → `Flow surface requires exact close before replacement`), so the old
   * surface is cancelled — which also releases its `CanvasGpuSession` — before the new element's
   * attach effect runs. Idempotent per generation: a burst of frames all reporting the same dead
   * canvas must mint ONE successor, not one each.
   *
   * Only a canvas that WAS presenting on the GPU can be resurrected this way, and that bound is the
   * whole termination argument: a successor re-attaches, and an attach that finds no adapter leaves
   * the element 2D-capable, so there is at most one replacement per device loss. An element that
   * refuses a 2D context while its surface never had a device is not poisoned — it is an
   * environment with no 2D canvas at all (jsdom, a stripped embedder), where a successor would
   * refuse in exactly the same way and the host would remount forever. */
  const replaceUnpresentableCanvas = useCallback(() => {
    const session = sessionRef.current;
    const retired = attachedSurfaceRef.current;
    if (!retired?.presentsOnGpu) return;
    attachedSurfaceRef.current = null;
    surfaceReadyRef.current = false;
    if (session) observeFlowTask(session, "surfaceStatus:unpresentable", session.surfaceStatus({ surface: retired.surface, surfaceGeneration: retired.surfaceGeneration, status: "cancelled" }));
    setCanvasGeneration((generation) => generation + 1);
  }, [surfaceId]);
  replaceUnpresentableCanvasRef.current = replaceUnpresentableCanvas;

  /** 📏️ Single owner of both canvases' backing-store size: measures the container and writes the
   * device-pixel store synchronously, then forwards the logical size to the wasm surface when one is
   * already attached. Session-independent on purpose — before this, the GPU canvas was only ever sized
   * from inside `attachCanvas().then(...)` and the label canvas only from `paintOverlays`, so a boot
   * where the surface attach is slow (or a hidden tab, where the shared `WasmCanvas` layout wait
   * never ticks) left both at the HTML default 300×150 in a 966×836 window. */
  const syncSurfaceSize = useCallback(() => {
    const container = containerRef.current;
    if (!container) return;
    const rect = container.getBoundingClientRect();
    const dpr = globalThis.devicePixelRatio || 1;
    resizeCanvasBackingStore(gpuCanvasRef.current, rect.width, rect.height, dpr);
    resizeCanvasBackingStore(labelCanvasRef.current, rect.width, rect.height, dpr);
    setContainerSize((prev) => (prev.w === rect.width && prev.h === rect.height ? prev : { w: rect.width, h: rect.height }));
    const session = sessionRef.current;
    if (session && surfaceReadyRef.current) observeFlowTask(session, "setSize", session.setSize(Math.max(1, Math.round(rect.width)), Math.max(1, Math.round(rect.height)), dpr));
  }, []);

  useEffect(() => {
    const container = containerRef.current;
    if (!container) return;
    syncSurfaceSize();
    const observer = new ResizeObserver(() => {
      syncSurfaceSize();
      renderFlow();
      paintOverlays();
    });
    observer.observe(container);
    return () => observer.disconnect();
  }, [paintOverlays, renderFlow, syncSurfaceSize]);

  return { syncSurfaceSize };
}
//#endregion FlowGraphCanvasHost
//#endregion 🔖️flow-graph-canvas-host
//#endregion 🔖️NodeGraphHost
