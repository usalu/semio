/** @emoji 👷 Brep tessellation worker — offloads heavy meshing from the UI thread. */

import initBrep, { initSync, tessellate } from "@semio-tech/flow-module-brep/pkg/flow_module_brep.js";
import brepWasmUrl from "@semio-tech/flow-module-brep/pkg/flow_module_brep_bg.wasm?url";

type TessellateWorkerRequest = { readonly op: "tessellate"; readonly reqId: number; readonly handle: string; readonly tolerance: number };

type TessellateWorkerResponse = { readonly op: "result"; readonly reqId: number; readonly json: string } | { readonly op: "error"; readonly reqId: number; readonly message: string };

let ready: Promise<void> | null = null;

function ensureWorkerWasm(): Promise<void> {
  if (!ready) {
    ready = (async () => {
      if (typeof initSync === "function") {
        const bytes = await fetch(brepWasmUrl).then((r) => r.arrayBuffer());
        initSync({ module: bytes });
      } else if (typeof initBrep === "function") {
        await initBrep({ module_or_path: brepWasmUrl });
      }
    })();
  }
  return ready;
}

self.onmessage = async (event: MessageEvent<TessellateWorkerRequest>) => {
  const msg = event.data;
  try {
    await ensureWorkerWasm();
    if (msg.op === "tessellate") {
      const json = tessellate(msg.handle, msg.tolerance);
      const response: TessellateWorkerResponse = { op: "result", reqId: msg.reqId, json };
      self.postMessage(response);
    }
  } catch (err) {
    const response: TessellateWorkerResponse = {
      op: "error",
      reqId: msg.reqId,
      message: err instanceof Error ? err.message : String(err),
    };
    self.postMessage(response);
  }
};
