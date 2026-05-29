/** @emoji 🧪 jsdom polyfills for `@elements/ui` vitest. */
class ResizeObserverMock {
  observe() {}
  unobserve() {}
  disconnect() {}
}

globalThis.ResizeObserver = ResizeObserverMock as typeof ResizeObserver;
