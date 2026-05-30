/** @emoji 🧪 jsdom polyfills for `@ui/react` vitest. */
// #region 🔌Adapters
import { cleanup } from "@testing-library/react";
import { afterEach } from "vitest";
// #endregion 🔌Adapters

class ResizeObserverMock {
  observe() {}
  unobserve() {}
  disconnect() {}
}

globalThis.ResizeObserver = ResizeObserverMock as typeof ResizeObserver;

afterEach(() => {
  cleanup();
});
