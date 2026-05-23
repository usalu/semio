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
