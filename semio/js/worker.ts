// #region 🧲KitWorkerEntry
// Dedicated module worker: one `KitStoreHandle`, GraphQL over postMessage. All payloads are
// **complete JSON documents** (one full GraphQL response per `op:"result"` for queries/mutations,
// one full per-event response per `op:"event"` for subscriptions). No NDJSON / line-of-json.
// 2026 Ueli Saluz <ueli@semio-tech.com> — GNU LGPL-3.0 or later

type WorkerIn =
  | { op: "init"; dto: unknown }
  | { op: "execute"; reqId: string; body: string }
  | { op: "subscribe"; reqId: string; body: string }
  | { op: "snapshot"; reqId: string };

type WorkerOut =
  | { op: "ready" }
  | { op: "result"; reqId: string; json: string }
  | { op: "event"; reqId: string; json: string }
  | { op: "done"; reqId: string }
  | { op: "snapshotResult"; reqId: string; json: string }
  | { op: "error"; reqId?: string; message: string };

let handle: import("@semio/rs-wasm").KitStoreHandle | null = null;

function post(out: WorkerOut): void {
  (self as DedicatedWorkerGlobalScope).postMessage(JSON.stringify(out));
}

(self as DedicatedWorkerGlobalScope).onmessage = async (ev: MessageEvent<string>) => {
  let msg: WorkerIn;
  try {
    msg = JSON.parse(ev.data) as WorkerIn;
  } catch {
    post({ op: "error", message: "invalid worker message json" });
    return;
  }
  try {
    if (msg.op === "init") {
      const mod = await import("@semio/rs-wasm");
      if (typeof mod.default === "function") await mod.default();
      if (typeof mod.boot === "function") mod.boot();
      handle = mod.KitStoreHandle.create(msg.dto as object);
      post({ op: "ready" });
      return;
    }
    if (!handle) {
      post({ op: "error", reqId: "op" in msg && msg.op !== "init" ? (msg as { reqId?: string }).reqId : undefined, message: "worker not initialized" });
      return;
    }
    if (msg.op === "snapshot") {
      const snap = handle.snapshot();
      post({ op: "snapshotResult", reqId: msg.reqId, json: JSON.stringify(snap) });
      post({ op: "done", reqId: msg.reqId });
      return;
    }
    if (msg.op === "execute") {
      const { reqId, body } = msg;
      const json = (await handle.execute(body)) as string;
      post({ op: "result", reqId, json });
      post({ op: "done", reqId });
      return;
    }
    if (msg.op === "subscribe") {
      const { reqId, body } = msg;
      await handle.subscribe(body, (eventJson: string) => {
        post({ op: "event", reqId, json: String(eventJson) });
      });
      post({ op: "done", reqId });
      return;
    }
    post({ op: "error", message: `unknown op ${(msg as { op?: string }).op ?? ""}` });
  } catch (e) {
    const reqId = (msg as { reqId?: string }).reqId;
    post({ op: "error", reqId, message: String(e) });
  }
};
// #endregion 🧲KitWorkerEntry
