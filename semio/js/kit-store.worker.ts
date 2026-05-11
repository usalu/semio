// #region 🧲Header
// 2025-2026 Ueli Saluz <ueli@semio-tech.com>
// GNU LGPL-3.0 or later — GraphQL WASM worker: JSON `execute` / `subscribe` only (no JS-side DTO marshaling).
// Bundled by Vite so `@semio/rs-wasm` resolves; Blob workers cannot import bare specifiers.
// #endregion 🧲Header

/// <reference lib="webworker" />

type WasmKitHandle = {
  execute: (body: string) => Promise<string>;
  subscribe: (body: string, onEvent: (eventJson: string) => void) => Promise<void>;
};

let handle: WasmKitHandle | null = null;

function post(out: unknown): void {
  self.postMessage(JSON.stringify(out));
}

self.onmessage = async (ev: MessageEvent<string>) => {
  let msg: {
    op?: string;
    dto?: unknown;
    body?: string;
    reqId?: string;
  };
  try {
    msg = JSON.parse(ev.data) as typeof msg;
  } catch {
    post({ op: "error", message: "invalid worker message json" });
    return;
  }
  try {
    if (msg.op === "init") {
      const mod = await import("@semio/rs-wasm");
      if (typeof mod.default === "function") await mod.default();
      if (typeof mod.boot === "function") mod.boot();
      const created = (mod as { KitStoreHandle: { create: (dto: unknown) => WasmKitHandle | Promise<WasmKitHandle> } }).KitStoreHandle.create(msg.dto);
      handle = created instanceof Promise ? await created : created;
      post({ op: "ready" });
      return;
    }
    if (!handle) {
      post({ op: "error", reqId: "op" in msg && msg.op !== "init" ? msg.reqId : undefined, message: "worker not initialized" });
      return;
    }
    if (msg.op === "execute") {
      const json = await handle.execute(msg.body as string);
      post({ op: "result", reqId: msg.reqId, json: String(json) });
      post({ op: "done", reqId: msg.reqId });
      return;
    }
    if (msg.op === "subscribe") {
      await handle.subscribe(msg.body as string, (eventJson: string) => {
        post({ op: "event", reqId: msg.reqId, json: String(eventJson) });
      });
      post({ op: "done", reqId: msg.reqId });
      return;
    }
    post({ op: "error", message: `unrecognized op ${msg.op ?? ""}` });
  } catch (e) {
    post({ op: "error", reqId: msg.reqId, message: String(e) });
  }
};
