// #region 🧲Header
// Storybook: lazy wasm init for @semio/rs-wasm
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion

import initSemio, { generateId, KitStoreHandle } from "@semio/rs-wasm";

// Bundle `semio.js` in Storybook, the default `new URL("semio_bg.wasm", import.meta.url)` is often wrong;
// point at the pkg explicitly so `fetch` loads the file.
// Path: .storybook/stories/kit-store/ → parent×4 = semio/semio → sibling rs/pkg
const semioWasmUrl = new URL("../../../../rs/pkg/semio_bg.wasm", import.meta.url);

let initPromise: Promise<void> | null = null;

/** Single-flight wasm init; safe to call from multiple components. */
export function ensureSemioWasm(): Promise<void> {
  if (typeof window === "undefined") {
    return Promise.resolve();
  }
  if (!initPromise) {
    initPromise = (async () => {
      try {
        await initSemio(semioWasmUrl);
      } catch (e) {
        initPromise = null;
        throw e;
      }
    })();
  }
  return initPromise;
}

export { generateId, initSemio, KitStoreHandle };
