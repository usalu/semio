/** 📡️ The retained inference port primitives: status publication, lease check, broker fetch, job body reads and retirement.
 *
 * 🧱️ Child module of {@link ../../🟦️.ts | createGisMapInferenceWorkerV1}: the functions take the worker context `ctx` explicitly instead of closing over the factory. */
import { GIS_MAP_INFERENCE_RESPONSE_MAX_BYTES, gisMapInferenceCodeFromStatusV1, gisMapInferencePortTerminalV1, reduceGisMapInferencePortV1, type GisMapInferencePortCodeV1, type GisMapInferencePortEventV1 } from "../../../🧬️schema/🟦️.ts";
import type { DocumentScope } from "@semio-tech/framework/document/scope";
import { DirectoryHttpError } from "@semio-tech/framework/inference/service/worker-host";
import type { FetchTimeoutResponse } from "@semio-tech/framework";
import type { GisInferenceContext, InferenceApprovalUndoOwnerV1, InferenceOperationV1, InferencePortStatusMessageV1 } from "../../🧰️worker-context-and-types/🟦️.ts";
import { sameApprovalUndoMountV1 } from "../../↩️approval-undo-owner/🟦️.ts";

export function postInferencePortStatus(ctx: GisInferenceContext, operation: InferenceOperationV1): void {
  const status: InferencePortStatusMessageV1 = { kind: "inference-port-status", operationEpoch: operation.operationEpoch, scope: operation.scope, status: { owner: "gis", serviceId: "s.gis.gismap.inference", payload: operation.status } };
  ctx.state.inferencePortStatusObserver?.(status);
  ctx.post(status);
}

/** 🧮️ Applies one closed event through the shared reducer and publishes only on a real change. */
export function advanceInferencePort(ctx: GisInferenceContext, operation: InferenceOperationV1, event: GisMapInferencePortEventV1): void {
  const next = reduceGisMapInferencePortV1(operation.status, event);
  if (next === operation.status) return;
  operation.status = next;
  postInferencePortStatus(ctx, operation);
}

/** 🪪️ The precondition: only a document whose worker-private execution-target lease is minted AND
 * still live may open a port. A dropped, absent or non-writable lease is refused before any request
 * exists, and the refusal is a localized terminal, never a silent no-op. */
export function inferenceLeaseVerified(ctx: GisInferenceContext, scope: DocumentScope): boolean {
  const { artifactState } = ctx.host;
  const state = artifactState(scope.documentId, scope.spaceId);
  if (state === undefined || state.closed || state.docAbort.signal.aborted) return false;
  const lease = state.executionTargetLease;
  if (lease === null || !lease.live) return false;
  const fields = lease.fields();
  return fields.scope.spaceId === scope.spaceId && fields.scope.documentId === scope.documentId && fields.grant.write;
}

export async function inferenceBrokerFetch(ctx: GisInferenceContext, operation: InferenceOperationV1, suffix: string, init: { readonly method: "GET" | "POST"; readonly body?: string }): Promise<FetchTimeoutResponse> {
  const { sameBrowserSessionOperationFence, SOCKET_GRANT_REQUEST_TIMEOUT_MS } = ctx.host;
  if (operation.sessionEpoch !== ctx.host.directorySessionEpoch || !sameBrowserSessionOperationFence(operation.sessionFence)) throw new Error("gis map inference: original session unavailable");
  if ((suffix === "/jobs" || suffix.endsWith("/approval")) && !inferenceLeaseVerified(ctx, operation.scope)) throw new Error("gis map inference: document closed");
  const job = /^\/jobs\/([0-9a-f]{32})\/(events\?after=(\d{1,3})|cancel|approval)$/u.exec(suffix);
  const action = suffix === "/jobs" ? "submit" : suffix === "/jobs/reconcile" ? "reconcile" : job?.[2]?.startsWith("events") ? "events" : job?.[2] === "cancel" ? "cancel" : job?.[2] === "approval" ? "approve" : null;
  if (action === null) throw new Error("gis map inference: operation denied");
  const payload = init.body === undefined ? { jobId: job![1], ...(action === "events" ? { after: Number(job![3]) } : {}) } : JSON.parse(init.body);
  return operation.transport.call(action, payload, { timeoutMs: SOCKET_GRANT_REQUEST_TIMEOUT_MS, signal: operation.abort.signal,
    admit: () => operation.sessionEpoch === ctx.host.directorySessionEpoch && sameBrowserSessionOperationFence(operation.sessionFence) && !operation.closed && ((suffix !== "/jobs" && !suffix.endsWith("/approval")) || inferenceLeaseVerified(ctx, operation.scope)),
    retain: () => operation.sessionEpoch === ctx.host.directorySessionEpoch && sameBrowserSessionOperationFence(operation.sessionFence) && !operation.closed,
  });
}

export async function inferenceApprovalUndoBrokerFetch(ctx: GisInferenceContext, owner: InferenceApprovalUndoOwnerV1, body: string): Promise<FetchTimeoutResponse> {
  const { artifactState, sameBrowserSessionOperationFence, SOCKET_GRANT_REQUEST_TIMEOUT_MS } = ctx.host;
  const state = artifactState(owner.scope.documentId, owner.scope.spaceId);
  if (state === undefined || state.closed || state.openClientInstanceId !== owner.clientInstanceId || state.docAbort.signal.aborted || !sameBrowserSessionOperationFence(owner.sessionFence)) throw new Error("gis map approval undo: document closed");
  return owner.transport.call("undo", JSON.parse(body), { timeoutMs: SOCKET_GRANT_REQUEST_TIMEOUT_MS, signal: owner.abort.signal,
    admit: () => { const current = artifactState(owner.scope.documentId, owner.scope.spaceId); return ctx.state.inferenceApprovalUndoOwner === owner && owner.sessionEpoch === ctx.host.directorySessionEpoch && sameBrowserSessionOperationFence(owner.sessionFence) && current !== undefined && sameApprovalUndoMountV1(ctx, current, owner); },
    retain: () => owner.sessionEpoch === ctx.host.directorySessionEpoch && sameBrowserSessionOperationFence(owner.sessionFence),
  });
}

/** 📥️ Reads one bounded owner-private JSON body under the shared response maximum. */
export async function readInferenceJson(response: FetchTimeoutResponse): Promise<unknown> {
  const declared = response.headers.get("content-length");
  if (declared !== null && !(Number.isSafeInteger(Number(declared)) && Number(declared) >= 1 && Number(declared) <= GIS_MAP_INFERENCE_RESPONSE_MAX_BYTES)) throw new Error("gis map inference: invalid body");
  const text = await response.text();
  if (text.length === 0 || new TextEncoder().encode(text).length > GIS_MAP_INFERENCE_RESPONSE_MAX_BYTES) throw new Error("gis map inference: invalid body");
  return JSON.parse(text);
}

export function inferenceCodeFromRejection(error: unknown, aborted: boolean): GisMapInferencePortCodeV1 {
  if (aborted) return "inference.cancelled";
  const status = error instanceof DirectoryHttpError ? error.status : undefined;
  return status === undefined ? "inference.transport" : gisMapInferenceCodeFromStatusV1(status);
}

/** 🏁️ Releases a port that already reported its terminal, without publishing a second one. */
export function retireInferencePort(ctx: GisInferenceContext, operation: InferenceOperationV1): void {
  if (operation.closed) return;
  if (operation.pollTimer !== null) clearTimeout(operation.pollTimer);
  operation.pollTimer = null;
  if (!operation.closeRequested && operation.request !== null && gisMapInferencePortTerminalV1(operation.status.phase)) return;
  operation.closed = true;
  operation.abort.abort(new Error("gis map inference port retired"));
  if (ctx.state.inferencePort === operation) ctx.state.inferencePort = null;
  ctx.post({ kind: "inference-port-closed", operationEpoch: operation.operationEpoch, scope: operation.scope });
}
