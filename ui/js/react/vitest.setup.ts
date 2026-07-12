/** @emoji 🧪 jsdom polyfills for `@semio-tech/ui-react` vitest. */
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

// jsdom does not implement scrollIntoView; tab strips call it on mount to reveal the active tab.
if (!Element.prototype.scrollIntoView) {
  Element.prototype.scrollIntoView = () => {};
}

afterEach(() => {
  cleanup();
});
