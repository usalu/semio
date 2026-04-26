// #region 🧲KitWorkerEntry
// Dedicated module worker: one `KitStoreHandle`, GraphQL `execute` over postMessage JSON strings.
// 2026 Ueli Saluz <ueli@semio-tech.com> — GNU LGPL-3.0 or later

type WorkerIn =
  | { op: "init"; dto: unknown }
  | { op: "execute"; reqId: string; body: string }
  | { op: "snapshot"; reqId: string };

type WorkerOut =
  | { op: "ready" }
  | { op: "chunk"; reqId: string; line: string }
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
      await handle.execute(body, (line: string) => {
        post({ op: "chunk", reqId, line: String(line) });
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
