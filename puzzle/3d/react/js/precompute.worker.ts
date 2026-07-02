//#region 🧲Header
/// <reference lib="webworker" />
/** @emoji 🧵 Puzzle 3d brush/fill precompute worker — parry3d WASM session + idle cache warming. */
//#endregion 🧲Header

/// <reference types="vite/client" />

import initPuzzle3dWasm, { initSync, Puzzle3dPrecomputeSession } from "../../rs/pkg/puzzle_3d.js";

//#region 📤Wire
function post(out: unknown): void {
  self.postMessage(JSON.stringify(out));
}
//#endregion 📤Wire

//#region 🧷Session
let session: Puzzle3dPrecomputeSession | null = null;
let idleTimer: ReturnType<typeof setTimeout> | null = null;
const PRECOMPUTE_CHUNK_BUDGET = 8;

function scheduleIdlePrecompute(): void {
  if (idleTimer !== null || !session) {
    return;
  }
  idleTimer = setTimeout(() => {
    idleTimer = null;
    if (!session) {
      return;
    }
    const more = session.precompute_step(PRECOMPUTE_CHUNK_BUDGET);
    if (more) {
      scheduleIdlePrecompute();
    }
  }, 0);
}
//#endregion 🧷Session

//#region 🧵OnMessage
self.onmessage = async (ev: MessageEvent<string | Record<string, unknown>>) => {
  let msg: {
    op?: string;
    reqId?: string;
    json?: string;
    url?: string;
    positions?: Float32Array;
    indices?: Uint32Array;
    vortexFullId?: string;
    budget?: number;
  };
  try {
    msg = typeof ev.data === "string" ? (JSON.parse(ev.data) as typeof msg) : (ev.data as typeof msg);
  } catch {
    post({ op: "error", message: "invalid worker message json" });
    return;
  }
  try {
    if (msg.op === "init") {
      const wasmUrl = new URL("../../rs/pkg/puzzle_3d_bg.wasm", import.meta.url);
      const wasmBytes = await fetch(wasmUrl).then((r) => r.arrayBuffer());
      initSync({ module: wasmBytes });
      await initPuzzle3dWasm();
      session = new Puzzle3dPrecomputeSession();
      post({ op: "ready" });
      return;
    }
    if (!session) {
      post({ op: "error", reqId: msg.reqId, message: "worker not initialized" });
      return;
    }
    if (msg.op === "setScene") {
      session.set_scene(msg.json ?? "{}");
      scheduleIdlePrecompute();
      post({ op: "done", reqId: msg.reqId });
      return;
    }
    if (msg.op === "registerMesh") {
      if (typeof msg.url === "string" && msg.positions instanceof Float32Array && msg.indices instanceof Uint32Array) {
        session.register_mesh(msg.url, msg.positions, msg.indices);
        scheduleIdlePrecompute();
      }
      post({ op: "done", reqId: msg.reqId });
      return;
    }
    if (msg.op === "precomputeStep") {
      const more = session.precompute_step(Math.max(0, Math.round(msg.budget ?? PRECOMPUTE_CHUNK_BUDGET)));
      post({ op: "result", reqId: msg.reqId, json: JSON.stringify({ more }) });
      post({ op: "done", reqId: msg.reqId });
      if (more) {
        scheduleIdlePrecompute();
      }
      return;
    }
    if (msg.op === "brushCandidates") {
      const json = session.brush_candidates(msg.vortexFullId ?? "");
      post({ op: "result", reqId: msg.reqId, json });
      post({ op: "done", reqId: msg.reqId });
      return;
    }
    if (msg.op === "fillProgress") {
      const json = session.fill_progress();
      post({ op: "result", reqId: msg.reqId, json });
      post({ op: "done", reqId: msg.reqId });
      return;
    }
    post({ op: "error", reqId: msg.reqId, message: `unrecognized op ${msg.op ?? ""}` });
  } catch (e) {
    post({ op: "error", reqId: msg.reqId, message: String(e) });
  }
};
//#endregion 🧵OnMessage
