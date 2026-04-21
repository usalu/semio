// @ts-nocheck
// #region 🧵KitWorker
// Web Worker entry: loads the semio WASM module (host-configured), hosts [`KitStoreHandle`], exposes RPC via Comlink.

import * as Comlink from "comlink";

let handle: any = null;

function settle(p: Promise<any>): Promise<any> {
  return p.catch((e) => ({ ok: false, error: { kind: "Internal", message: String(e) } }));
}

const api = {
  async init(wasmSpecifier: string, dto: unknown) {
    const mod = await import(/* @vite-ignore */ wasmSpecifier);
    if (typeof mod.default === "function") {
      await mod.default();
    }
    const { KitStoreHandle } = mod;
    handle = KitStoreHandle.create(dto as any);
  },
  snapshot() {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return handle.snapshot();
  },
  getField(kind: string, guid: string, field: string) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return handle.getField(kind, guid, field);
  },
  setField(kind: string, guid: string, field: string, value: unknown) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.setField(kind, guid, field, value)));
  },
  addChild(parentKind: string, parentGuid: string, childKind: string, dto: unknown) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.addChild(parentKind, parentGuid, childKind, dto)));
  },
  removeChild(parentKind: string, parentGuid: string, childKind: string, childGuid: string) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.removeChild(parentKind, parentGuid, childKind, childGuid)));
  },
  applyDesignDiff(designGuid: string, diff: unknown) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.applyDesignDiff(designGuid, diff)));
  },
  subscribe(cb: (ev: unknown) => void) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    const proxy = Comlink.proxy(cb);
    handle.subscribe(proxy);
  },
};

Comlink.expose(api);

// #endregion 🧵KitWorker
