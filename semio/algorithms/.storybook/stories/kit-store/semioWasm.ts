// #region 🧲Header
// Storybook: lazy wasm init for @semio/rs-wasm
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion

import initSemio, { generateId, KitStoreHandle } from "@semio/rs-wasm";

let initPromise: Promise<void> | null = null;

/** Single-flight wasm init; safe to call from multiple components. */
export function ensureSemioWasm(): Promise<void> {
  if (typeof window === "undefined") {
    return Promise.resolve();
  }
  if (!initPromise) {
    initPromise = initSemio() as Promise<void>;
  }
  return initPromise;
}

export { generateId, initSemio, KitStoreHandle };
