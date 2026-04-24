// @ts-nocheck
// #region 🧵KitWorker
// Web Worker entry: loads the semio WASM module (host-configured), hosts [`KitStoreHandle`], exposes RPC via Comlink.

import * as Comlink from "comlink";
import { kitGraphqlExecuteRead, kitGraphqlExecuteStoreCommand, kitGraphqlSubscribeLoop, type KitGraphqlHandle } from "./kitGraphLive";
import type { ReadCommandBatch, ReadCommandBatchResult } from "./readCommandTypes";

let handle: any = null;
const kitEventListeners = new Map<number, (ev: unknown) => void>();
let nextKitEventListenerId = 0;
let kitEventGqlStarted = false;

function gqlHandle(): KitGraphqlHandle {
  if (!handle) throw new Error("KitStoreHandle not initialized");
  return {
    execute: (requestJson: string, onMessage: (line: string) => void) => handle.execute(requestJson, onMessage),
  };
}

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
    if (typeof mod.boot === "function") {
      mod.boot();
    }
    const { KitStoreHandle } = mod;
    handle = KitStoreHandle.create(dto as any);
  },
  snapshot() {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return handle.snapshot();
  },
  setField(kind: string, id: string, field: string, value: unknown) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(
      (async () => {
        try {
          const cmds = handle.changeKitCommandsForFieldPatch(kind, id, field, value);
          handle.executeChangeKitCommands(cmds);
          return { ok: true };
        } catch (e) {
          return { ok: false, error: { kind: "Internal", message: String(e) } };
        }
      })(),
    );
  },
  addChild(parentKind: string, parentId: string, childKind: string, dto: unknown) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(
      (async () => {
        try {
          const cmds = handle.changeKitCommandsForAddChild(parentKind, parentId, childKind, dto);
          handle.executeChangeKitCommands(cmds);
          return { ok: true };
        } catch (e) {
          return { ok: false, error: { kind: "Internal", message: String(e) } };
        }
      })(),
    );
  },
  removeChild(parentKind: string, parentId: string, childKind: string, childId: string) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(
      (async () => {
        try {
          const cmds = handle.changeKitCommandsForRemoveChild(parentKind, parentId, childKind, childId);
          handle.executeChangeKitCommands(cmds);
          return { ok: true };
        } catch (e) {
          return { ok: false, error: { kind: "Internal", message: String(e) } };
        }
      })(),
    );
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
  subscribe(cb: (ev: unknown) => void) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    const proxy = Comlink.proxy(cb);
    const id = nextKitEventListenerId++;
    const forward = (payload: unknown) => {
      try {
        proxy(payload);
      } catch {
        /* ignore */
      }
    };
    kitEventListeners.set(id, forward);
    if (!kitEventGqlStarted) {
      kitEventGqlStarted = true;
      kitGraphqlSubscribeLoop(gqlHandle(), (payload) => {
        for (const fn of kitEventListeners.values()) fn(payload);
      });
    }
    return () => {
      kitEventListeners.delete(id);
      if (kitEventListeners.size === 0) kitEventGqlStarted = false;
    };
  },

  async execute(cmd: unknown) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    try {
      const result = await kitGraphqlExecuteStoreCommand(gqlHandle(), cmd);
      return { ok: true, result };
    } catch (e) {
      return { ok: false, error: { kind: "Internal", message: String(e) } };
    }
  },

  async executeRead(cmds: ReadCommandBatch): Promise<ReadCommandBatchResult> {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return await kitGraphqlExecuteRead(gqlHandle(), cmds);
  },

  vcsState() {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return handle.vcsState();
  },

  theKitDto() {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return handle.theKitDto();
  },

  materializeAt(at: unknown) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return handle.materializeAt(at);
  },

  attachBackbone(config: unknown) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(
      (async () => {
        try {
          const result = await kitGraphqlExecuteStoreCommand(gqlHandle(), { attachBackbone: { config } });
          return { ok: true, result };
        } catch (e) {
          return { ok: false, error: { kind: "Internal", message: String(e) } };
        }
      })(),
    );
  },

  detachBackbone() {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(
      (async () => {
        try {
          const result = await kitGraphqlExecuteStoreCommand(gqlHandle(), { detachBackbone: null });
          return { ok: true, result };
        } catch (e) {
          return { ok: false, error: { kind: "Internal", message: String(e) } };
        }
      })(),
    );
  },

  async backboneStatus() {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    try {
      const result = await kitGraphqlExecuteStoreCommand(gqlHandle(), { backboneStatus: null });
      return { ok: true, result };
    } catch (e) {
      return { ok: false, error: { kind: "Internal", message: String(e) } };
    }
  },

  async listConflicts() {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    try {
      const result = await kitGraphqlExecuteStoreCommand(gqlHandle(), { listConflicts: null });
      return { ok: true, result };
    } catch (e) {
      return { ok: false, error: { kind: "Internal", message: String(e) } };
    }
  },

  resolveConflict(conflictId: string, resolution: unknown) {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(
      (async () => {
        try {
          const result = await kitGraphqlExecuteStoreCommand(gqlHandle(), {
            resolveConflict: { id: conflictId, strategy: resolution },
          });
          return { ok: true, result };
        } catch (e) {
          return { ok: false, error: { kind: "Internal", message: String(e) } };
        }
      })(),
    );
  },

  syncNow() {
    if (!handle) throw new Error("KitStoreHandle not initialized");
    return settle(
      (async () => {
        try {
          const result = await kitGraphqlExecuteStoreCommand(gqlHandle(), { syncNow: null });
          return { ok: true, result };
        } catch (e) {
          return { ok: false, error: { kind: "Internal", message: String(e) } };
        }
      })(),
    );
  },
};

Comlink.expose(api);

// #endregion 🧵KitWorker
