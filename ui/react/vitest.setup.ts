/** @emoji 🧪 jsdom polyfills for `@ui/react` vitest. */
import { cleanup } from "@testing-library/react";
import { afterEach } from "vitest";

class ResizeObserverMock {
  observe() {}
  unobserve() {}
  disconnect() {}
}

globalThis.ResizeObserver = ResizeObserverMock as typeof ResizeObserver;

afterEach(() => {
  cleanup();
});
