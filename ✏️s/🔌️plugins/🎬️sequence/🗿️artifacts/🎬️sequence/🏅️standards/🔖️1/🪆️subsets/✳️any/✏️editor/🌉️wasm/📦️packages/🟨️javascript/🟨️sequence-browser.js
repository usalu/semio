//! 🌐️ Production Sequence browser entry over the owned serialized Wasm ABI.

import { createSequenceFeatures, createSequenceHost } from "./🟨️sequence-host.js";

//#region 🌐️BrowserConsumer

export async function createSequenceBrowserFeatures({ source, imports = {}, instantiate = WebAssembly.instantiate, ...hostOptions } = {}) {
  if (source === undefined) throw new Error("Sequence Wasm source is required");
  const instantiated = await instantiate(source, imports);
  const instance = instantiated?.instance ?? instantiated;
  const exports = instance?.exports;
  const memory = exports?.memory;
  if (!exports || !(memory instanceof WebAssembly.Memory)) throw new Error("Sequence Wasm instance must export memory");
  return createSequenceFeatures(createSequenceHost({ ...hostOptions, exports, memory }));
}

//#endregion 🌐️BrowserConsumer
