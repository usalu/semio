// #region 🧲Header
/** @emoji 🖼️ `@infinite/cavas/react-renderer` — r3f-style reconciler host for tile-based infinite canvases (WASM bridge supplied by leaf bundles). */
// #endregion 🧲Header

// #region 🔌Adapters
import { ContextMenuController, reactHostPort, type ContextMenuItem } from "@ui/react";
import React from "react";
import Reconciler from "react-reconciler";
import { ContinuousEventPriority, DefaultEventPriority, DiscreteEventPriority, LegacyRoot, NoEventPriority } from "react-reconciler/constants";
// #endregion 🔌Adapters

export { ContextMenuController, reactHostPort, type ContextMenuItem, React, Reconciler, ContinuousEventPriority, DefaultEventPriority, DiscreteEventPriority, LegacyRoot, NoEventPriority };

// #region 🔖EventBinding
export type CavasListenerTarget = Pick<EventTarget, "addEventListener" | "removeEventListener">;

/** @emoji 🎧 Tracks DOM listeners for deterministic teardown on canvas unmount. */
export class CavasEventBindingController {
  private readonly cleanups: Array<() => void> = [];

  listen(target: CavasListenerTarget | null | undefined, kind: string, listener: EventListenerOrEventListenerObject, options?: boolean | AddEventListenerOptions): void {
    if (!target) return;
    target.addEventListener(kind, listener, options);
    this.cleanups.push(() => target.removeEventListener(kind, listener, options));
  }

  dispose(): void {
    while (this.cleanups.length > 0) {
      this.cleanups.pop()?.();
    }
  }
}
// #endregion 🔖EventBinding

// #region 🔖CanvasWasmBridge
/** @emoji 🌐 Leaf bundles (e.g. puzzle/2d) implement this against their `cdylib` session type. */
export interface CanvasWasmBridge<Session> {
  ensureLoaded(): Promise<void>;
  createSession(): Session;
}
// #endregion 🔖CanvasWasmBridge

// #region 🔖GraphWasmCanvas
/** @emoji 🕸️ Minimal WASM graph session surface (attach, resize, RAF, optional pointer). */
export interface GraphWasmSession {
  attachCanvas(canvas: HTMLCanvasElement, logicalW: number, logicalH: number, dpr: number): Promise<unknown>;
  setSize(width: number, height: number, dpr: number): void;
  renderFrame(): void;
  pointerDown?(x: number, y: number, extend: boolean): void;
  pointerMove?(x: number, y: number): void;
  pointerUp?(): void;
}

export interface GraphWasmCanvasProps {
  readonly className?: string;
  readonly sessionFactory: () => GraphWasmSession;
  readonly onSessionReady?: (session: GraphWasmSession) => void;
  readonly enablePointer?: boolean;
}

export function GraphWasmCanvas({ className, sessionFactory, onSessionReady, enablePointer = true }: GraphWasmCanvasProps): React.JSX.Element {
  const containerRef = React.useRef<HTMLDivElement>(null);
  const canvasRef = React.useRef<HTMLCanvasElement>(null);
  const sessionRef = React.useRef<GraphWasmSession | null>(null);
  const rafRef = React.useRef<number | null>(null);

  const renderFrame = React.useCallback(() => {
    try {
      sessionRef.current?.renderFrame();
    } catch {
      /* gpu not ready */
    }
  }, []);

  React.useEffect(() => {
    const canvas = canvasRef.current;
    const container = containerRef.current;
    if (!canvas || !container) return;
    const session = sessionFactory();
    sessionRef.current = session;
    onSessionReady?.(session);
    const rect = container.getBoundingClientRect();
    const dpr = globalThis.devicePixelRatio || 1;
    const initW = Math.max(1, Math.round(rect.width));
    const initH = Math.max(1, Math.round(rect.height));
    canvas.width = Math.round(initW * dpr);
    canvas.height = Math.round(initH * dpr);
    canvas.style.width = `${initW}px`;
    canvas.style.height = `${initH}px`;
    void session.attachCanvas(canvas, initW, initH, dpr).then(() => {
      const resize = () => {
        const rect = container.getBoundingClientRect();
        const dpr = globalThis.devicePixelRatio || 1;
        const w = Math.max(1, Math.round(rect.width));
        const h = Math.max(1, Math.round(rect.height));
        canvas.width = Math.round(w * dpr);
        canvas.height = Math.round(h * dpr);
        canvas.style.width = `${w}px`;
        canvas.style.height = `${h}px`;
        session.setSize(w, h, dpr);
        renderFrame();
      };
      resize();
      const ro = new ResizeObserver(resize);
      ro.observe(container);
      const tick = () => {
        renderFrame();
        rafRef.current = requestAnimationFrame(tick);
      };
      rafRef.current = requestAnimationFrame(tick);
      const onPointerDown = (ev: PointerEvent) => {
        session.pointerDown?.(ev.clientX - canvas.getBoundingClientRect().left, ev.clientY - canvas.getBoundingClientRect().top, ev.shiftKey);
        renderFrame();
      };
      const onPointerMove = (ev: PointerEvent) => {
        session.pointerMove?.(ev.clientX - canvas.getBoundingClientRect().left, ev.clientY - canvas.getBoundingClientRect().top);
        renderFrame();
      };
      const onPointerUp = () => {
        session.pointerUp?.();
        renderFrame();
      };
      if (enablePointer) {
        canvas.addEventListener("pointerdown", onPointerDown);
        canvas.addEventListener("pointermove", onPointerMove);
        canvas.addEventListener("pointerup", onPointerUp);
        canvas.addEventListener("pointerleave", onPointerUp);
      }
      return () => {
        ro.disconnect();
        if (enablePointer) {
          canvas.removeEventListener("pointerdown", onPointerDown);
          canvas.removeEventListener("pointermove", onPointerMove);
          canvas.removeEventListener("pointerup", onPointerUp);
          canvas.removeEventListener("pointerleave", onPointerUp);
        }
        if (rafRef.current != null) cancelAnimationFrame(rafRef.current);
      };
    });
    return () => {
      if (rafRef.current != null) cancelAnimationFrame(rafRef.current);
      sessionRef.current = null;
    };
  }, [enablePointer, onSessionReady, renderFrame, sessionFactory]);

  return (
    <div ref={containerRef} className={className ?? "relative h-full w-full min-h-0 min-w-0"}>
      <canvas ref={canvasRef} className="block h-full w-full touch-none" />
    </div>
  );
}
// #endregion 🔖GraphWasmCanvas

// #region 🔖ReconcilerReexports
export type RenderMode = "main-thread" | "worker-offscreen" | "headless-test";
// #endregion 🔖ReconcilerReexports

// #region 🔖Vitest
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("CavasEventBindingController", () => {
    it("disposes registered listeners", () => {
      const ctrl = new CavasEventBindingController();
      let count = 0;
      const target = {
        addEventListener: () => {
          count += 1;
        },
        removeEventListener: () => {
          count -= 1;
        },
      };
      ctrl.listen(target, "pointermove", () => {});
      expect(count).toBe(1);
      ctrl.dispose();
      expect(count).toBe(0);
    });
  });
}
// #endregion 🔖Vitest
