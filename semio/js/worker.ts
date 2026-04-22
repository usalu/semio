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
  clusterPieces(designGuid: string, pieceGuids: string[], clusterName: string) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.clusterPieces(designGuid, pieceGuids, clusterName)));
  },
  dragPieces(designGuid: string, pieceGuids: string[], du: number, dv: number) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.dragPieces(designGuid, pieceGuids, du, dv)));
  },
  movePieces(designGuid: string, pieceGuids: string[], gap: number, shift: number, rise: number) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.movePieces(designGuid, pieceGuids, gap, shift, rise)));
  },
  fixPieces(designGuid: string, pieceGuids: string[]) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.fixPieces(designGuid, pieceGuids)));
  },
  flattenDesign(designGuid: string) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.flattenDesign(designGuid)));
  },
  expandDesign(parentDesignGuid: string, nestedDesignGuid: string) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.expandDesign(parentDesignGuid, nestedDesignGuid)));
  },
  deleteConnection(designGuid: string, connectionGuid: string) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.deleteConnection(designGuid, connectionGuid)));
  },
  changePieceType(designGuid: string, pieceGuid: string, newTypeGuid: string) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.changePieceType(designGuid, pieceGuid, newTypeGuid)));
  },
  pasteDesignSelection(designGuid: string, selection: unknown, plane: unknown) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.pasteDesignSelection(designGuid, selection, plane)));
  },
  createHangingPieces(designGuid: string, typeGuids: string[], plane: unknown) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.createHangingPieces(designGuid, typeGuids, plane)));
  },
  createConnectedPiece(designGuid: string, parentPiece: string, parentPort: string, childType: string, childPort: string) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.createConnectedPiece(designGuid, parentPiece, parentPort, childType, childPort)));
  },
  createFixedPiece(designGuid: string, typeGuid: string, plane: unknown) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.createFixedPiece(designGuid, typeGuid, plane)));
  },
  getPiecesMetadata(designGuid: string) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.getPiecesMetadata(designGuid)));
  },
  getPieces(designGuid: string) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.getPieces(designGuid)));
  },
  getConnections(designGuid: string) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.getConnections(designGuid)));
  },
  getDesigns() {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.getDesigns()));
  },
  getTypes() {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.getTypes()));
  },
  getAuthors() {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.getAuthors()));
  },
  getKitMetadata() {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.getKitMetadata()));
  },
  subscribe(cb: (ev: unknown) => void) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    const proxy = Comlink.proxy(cb);
    handle.subscribe(proxy);
  },
};

Comlink.expose(api);

// #endregion 🧵KitWorker
