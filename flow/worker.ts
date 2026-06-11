/** @emoji 👷 Dedicated flow orchestrator worker — runs FlowSession WASM off the UI thread. */

import initFlowWasm, { FlowSession, initSync, tessellate } from "./core/pkg/flow_core.js";
import flowCoreWasmUrl from "./core/pkg/flow_core_bg.wasm?url";

type FlowWorkerRequest =
  | { readonly op: "init" }
  | { readonly op: "loadFixture"; readonly reqId: number; readonly json: string }
  | { readonly op: "evaluate"; readonly reqId: number }
  | { readonly op: "tessellatePreviews"; readonly reqId: number; readonly outputsJson: string; readonly tolerance?: number }
  | { readonly op: "previewText"; readonly reqId: number }
  | { readonly op: "fixtureJson"; readonly reqId: number };

type FlowWorkerResponse =
  | { readonly op: "ready" }
  | { readonly op: "result"; readonly reqId: number; readonly json: string }
  | { readonly op: "error"; readonly reqId: number; readonly message: string };

type RawMeshTransfer = {
  readonly position?: readonly number[];
  readonly normal?: readonly number[];
  readonly index?: readonly number[];
  readonly edges?: readonly number[];
  readonly faceGroups?: readonly { readonly start: number; readonly count: number; readonly entityId: string }[];
  readonly face_groups?: readonly { readonly start: number; readonly count: number; readonly entity_id: string }[];
  readonly error?: string;
};

let session: FlowSession | null = null;

function post(message: FlowWorkerResponse): void {
  self.postMessage(message);
}

async function ensureSession(): Promise<FlowSession> {
  if (session) return session;
  if (typeof initSync === "function") {
    const response = await fetch(flowCoreWasmUrl);
    const bytes = await response.arrayBuffer();
    initSync({ module: bytes });
  }
  await initFlowWasm({ module_or_path: flowCoreWasmUrl });
  session = new FlowSession();
  return session;
}

const GEOMETRY_HANDLE_PATTERN = /^(vertex|edge|wire|face|shell|solid|compound|curve|surface|drawing)-/;

function collectGeometryHandles(value: unknown, out: Array<{ readonly key: string; readonly handle: string }>, key: string): void {
  if (typeof value === "string" && GEOMETRY_HANDLE_PATTERN.test(value)) {
    out.push({ key, handle: value });
    return;
  }
  if (!value || typeof value !== "object" || Array.isArray(value)) return;
  const dict = value as Record<string, unknown>;
  if (dict.$schema === "geometry" && typeof dict.handle === "string") {
    out.push({ key, handle: dict.handle });
    return;
  }
  for (const [nestedKey, nested] of Object.entries(dict)) {
    collectGeometryHandles(nested, out, `${key}.${nestedKey}`);
  }
}

function tessellatePreviewMeshes(outputsJson: string, tolerance: number): Record<string, RawMeshTransfer> {
  const meshes: Record<string, RawMeshTransfer> = {};
  let parsed: Record<string, { readonly in?: Record<string, unknown>; readonly out?: Record<string, unknown> }> = {};
  try {
    parsed = JSON.parse(outputsJson) as typeof parsed;
  } catch {
    return meshes;
  }
  for (const [widgetId, entry] of Object.entries(parsed)) {
    for (const [port, value] of Object.entries(entry.out ?? {})) {
      const refs: Array<{ readonly key: string; readonly handle: string }> = [];
      collectGeometryHandles(value, refs, `${widgetId}:${port}`);
      for (const ref of refs) {
        try {
          const raw = JSON.parse(tessellate(ref.handle, tolerance)) as RawMeshTransfer;
          if (!raw.error) meshes[ref.key] = raw;
        } catch {
          /* skip invalid handle */
        }
      }
    }
  }
  return meshes;
}

self.onmessage = async (event: MessageEvent<FlowWorkerRequest>) => {
  const msg = event.data;
  try {
    if (msg.op === "init") {
      await ensureSession();
      post({ op: "ready" });
      return;
    }
    const active = await ensureSession();
    if (msg.op === "loadFixture") {
      active.loadFixtureJson(msg.json);
      post({ op: "result", reqId: msg.reqId, json: "{}" });
      return;
    }
    if (msg.op === "evaluate") {
      const outputsJson = await active.evaluate();
      post({ op: "result", reqId: msg.reqId, json: JSON.stringify({ outputsJson }) });
      return;
    }
    if (msg.op === "tessellatePreviews") {
      const previewMeshes = tessellatePreviewMeshes(msg.outputsJson, msg.tolerance ?? 0.02);
      post({ op: "result", reqId: msg.reqId, json: JSON.stringify({ previewMeshes }) });
      return;
    }
    if (msg.op === "previewText") {
      post({ op: "result", reqId: msg.reqId, json: JSON.stringify({ text: active.previewText() }) });
      return;
    }
    if (msg.op === "fixtureJson") {
      post({ op: "result", reqId: msg.reqId, json: active.fixtureJson() });
    }
  } catch (err) {
    const reqId = "reqId" in msg ? msg.reqId : 0;
    post({ op: "error", reqId, message: err instanceof Error ? err.message : String(err) });
  }
};
