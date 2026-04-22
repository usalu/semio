// @ts-nocheck
// #region 🧵KitWorker
// Web Worker entry: loads the semio WASM module (host-configured), hosts [`KitStoreHandle`], exposes RPC via Comlink.

import * as Comlink from "comlink";

let handle: any = null;

async function importWasmModule(specifier: string) {
  if (specifier === "@semio/rs-wasm") {
    return import("@semio/rs-wasm");
  }
  return import(/* @vite-ignore */ specifier);
}

function settle(p: Promise<any>): Promise<any> {
  return p.catch((e) => ({ ok: false, error: { kind: "Internal", message: String(e) } }));
}

const api = {
  async init(wasmSpecifier: string, dto: unknown) {
    const mod = await importWasmModule(wasmSpecifier);
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
  getField(kind: string, id: string, field: string) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return handle.getField(kind, id, field);
  },
  setField(kind: string, id: string, field: string, value: unknown) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.setField(kind, id, field, value)));
  },
  addChild(parentKind: string, parentId: string, childKind: string, dto: unknown) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.addChild(parentKind, parentId, childKind, dto)));
  },
  removeChild(parentKind: string, parentId: string, childKind: string, childId: string) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.removeChild(parentKind, parentId, childKind, childId)));
  },
  applyDesignDiff(designId: string, diff: unknown) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.applyDesignDiff(designId, diff)));
  },
  applyKitDiff(diff: unknown) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.applyKitDiff(diff)));
  },
  clusterPieces(designId: string, pieceIds: string[], clusterName: string) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.clusterPieces(designId, pieceIds, clusterName)));
  },
  dragPieces(designId: string, pieceIds: string[], du: number, dv: number) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.dragPieces(designId, pieceIds, du, dv)));
  },
  movePieces(designId: string, pieceIds: string[], gap: number, shift: number, rise: number) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.movePieces(designId, pieceIds, gap, shift, rise)));
  },
  fixPieces(designId: string, pieceIds: string[]) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.fixPieces(designId, pieceIds)));
  },
  flattenDesign(designId: string) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.flattenDesign(designId)));
  },
  expandDesign(parentDesignId: string, nestedDesignId: string) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.expandDesign(parentDesignId, nestedDesignId)));
  },
  deleteConnection(designId: string, connectionId: string) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.deleteConnection(designId, connectionId)));
  },
  changePieceType(designId: string, pieceId: string, newTypeId: string) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.changePieceType(designId, pieceId, newTypeId)));
  },
  pasteDesignSelection(designId: string, selection: unknown, plane: unknown) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.pasteDesignSelection(designId, selection, plane)));
  },
  createHangingPieces(designId: string, typeIds: string[], plane: unknown) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.createHangingPieces(designId, typeIds, plane)));
  },
  createConnectedPiece(designId: string, parentPiece: string, parentPort: string, childType: string, childPort: string) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.createConnectedPiece(designId, parentPiece, parentPort, childType, childPort)));
  },
  createFixedPiece(designId: string, typeId: string, plane: unknown) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.createFixedPiece(designId, typeId, plane)));
  },
  getPiecesMetadata(designId: string) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.getPiecesMetadata(designId)));
  },
  getPieces(designId: string) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.getPieces(designId)));
  },
  getConnections(designId: string) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.getConnections(designId)));
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
  undo() {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.undo()));
  },
  redo() {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.redo()));
  },
  canUndo() {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return handle.canUndo();
  },
  canRedo() {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return handle.canRedo();
  },
  beginTx() {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.beginTx()));
  },
  commitTx() {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.commitTx()));
  },
  abortTx() {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(Promise.resolve(handle.abortTx()));
  },
  subscribe(cb: (ev: unknown) => void) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    const proxy = Comlink.proxy(cb);
    handle.subscribe(proxy);
  },
};

Comlink.expose(api);

// #endregion 🧵KitWorker
