/** 🎛️ Opening, submitting, approving, undoing and closing the retained inference port.
 *
 * 🧱️ Child module of {@link ../🟦️.ts | createGisMapInferenceWorkerV1}: the functions take the worker context `ctx` explicitly instead of closing over the factory. */
import { gisMapInferenceRequestToJson } from "../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/💡️inferences/🌐️hub/🟦️.ts";
import { gisMapInferenceCodeFromStatusV1, gisMapInferencePortTerminalV1, idleGisMapInferencePortStatusV1, parseGisMapInferenceApprovalReceiptV1, parseGisMapApprovalUndoReceiptV1, parseGisMapInferenceJobReceiptV1, sealGisMapApprovalUndoRequestV1, sealGisMapInferenceApprovalRequestV1, sealGisMapInferenceJobRequestV1, type GisMapApprovalUndoReceiptV1 } from "../../🧬️schema/🟦️.ts";
import type { DocumentScope } from "@semio-tech/framework/document/scope";
import { DirectoryHttpError, type DocumentServiceWorkerHostV1 } from "@semio-tech/framework/inference/service/worker-host";
import { INFERENCE_JOB_LIFETIME_MS, INFERENCE_PORT_CAPACITY } from "../🧰️worker-context-and-types/🟦️.ts";
import type { GisInferenceContext, InferenceOperationV1 } from "../🧰️worker-context-and-types/🟦️.ts";
import { approvalUndoStatus, retainInferenceApprovalUndo, retireInferenceApprovalUndo, retireInferenceApprovalUndoForAuthority, sameApprovalUndoMountV1 } from "../↩️approval-undo-owner/🟦️.ts";
import { advanceInferencePort, inferenceApprovalUndoBrokerFetch, inferenceBrokerFetch, inferenceCodeFromRejection, inferenceLeaseVerified, postInferencePortStatus, readInferenceJson, retireInferencePort } from "../🔁️port-job-turns/📡️port-status-and-broker/🟦️.ts";
import { cancelInferenceJob, driveInferencePort, liveInferencePort, scheduleInferencePoll, terminateInferencePort } from "../🔁️port-job-turns/🟦️.ts";

/** 🆕️ Installs the one retained port for exactly one document scope without replacing a predecessor. The
 * lease precondition is checked BEFORE the operation exists, so a refusal never leaves a port open. */
export function openInferencePort(ctx: GisInferenceContext, operationEpoch: number, scope: DocumentScope): void {
  const { captureBrowserSessionOperationFence, artifactState } = ctx.host;
  if (!Number.isSafeInteger(operationEpoch) || operationEpoch < 1 || scope.spaceId.length === 0 || scope.documentId.length === 0) {
    ctx.post({ kind: "inference-port-opened", operationEpoch, scope, outcome: "refused", code: "inference.invalid" });
    return;
  }
  if (ctx.state.inferencePort !== null && INFERENCE_PORT_CAPACITY === 1) {
    ctx.post({ kind: "inference-port-opened", operationEpoch, scope, outcome: "refused", code: "inference.capacity" });
    return;
  }
  if (!inferenceLeaseVerified(ctx, scope)) {
    ctx.post({ kind: "inference-port-opened", operationEpoch, scope, outcome: "refused", code: "inference.lease-unverified" });
    return;
  }
  const sessionFence = captureBrowserSessionOperationFence();
  if (sessionFence === null) {
    ctx.post({ kind: "inference-port-opened", operationEpoch, scope, outcome: "refused", code: "inference.denied" });
    return;
  }
  const state = artifactState(scope.documentId, scope.spaceId)!;
  let transport: ReturnType<DocumentServiceWorkerHostV1["bindDocumentService"]>;
  try { transport = ctx.host.bindDocumentService("gis", "s.gis.gismap.inference", scope); }
  catch { ctx.post({ kind: "inference-port-opened", operationEpoch, scope, outcome: "refused", code: "inference.unavailable" }); return; }
  const operation: InferenceOperationV1 = { operationEpoch, scope: Object.freeze({ ...scope }), abort: new AbortController(), sessionEpoch: ctx.host.directorySessionEpoch, sessionFence, clientInstanceId: state.openClientInstanceId, leaseFields: structuredClone(state.executionTargetLease!.fields()), transport, request: null, closeRequested: false, reconcileRequired: false, status: idleGisMapInferencePortStatusV1(), turns: 0, pollTimer: null, inFlight: false, cancelSent: false, closed: false };
  ctx.state.inferencePort = operation;
  ctx.post({ kind: "inference-port-opened", operationEpoch, scope, outcome: "opened", code: null });
  postInferencePortStatus(ctx, operation);
}

/** 📮️ Seals one request before dispatch and never retries its submit, including after response loss. */
export async function submitInferenceJob(ctx: GisInferenceContext, operationEpoch: number, requestId: string): Promise<void> {
  const operation = liveInferencePort(ctx, operationEpoch);
  if (operation === null || operation.status.phase !== "idle" || operation.inFlight) return;
  let body: string;
  try {
    operation.request = Object.freeze(sealGisMapInferenceJobRequestV1(requestId, INFERENCE_JOB_LIFETIME_MS));
    body = gisMapInferenceRequestToJson(operation.request);
  } catch {
    terminateInferencePort(ctx, operation, "inference.invalid");
    return;
  }
  advanceInferencePort(ctx, operation, { kind: "start" });
  operation.inFlight = true;
  try {
    const response = await inferenceBrokerFetch(ctx, operation, "/jobs", { method: "POST", body });
    if (!response.ok) throw new DirectoryHttpError(response.status, "");
    const receipt = parseGisMapInferenceJobReceiptV1(await readInferenceJson(response));
    if (liveInferencePort(ctx, operationEpoch) !== operation) return;
    if (receipt.proposalState === "approved") {
      operation.status = { ...operation.status, jobId: receipt.jobId, proposalHash: receipt.proposalHash ?? null };
      terminateInferencePort(ctx, operation, "inference.transport");
      scheduleInferencePoll(ctx, operation);
      return;
    }
    advanceInferencePort(ctx, operation, { kind: "receipt", receipt });
    if (gisMapInferencePortTerminalV1(operation.status.phase)) retireInferencePort(ctx, operation);
    else scheduleInferencePoll(ctx, operation);
  } catch (error) {
    if (operation.closed) return;
    terminateInferencePort(ctx, operation, inferenceCodeFromRejection(error, operation.abort.signal.aborted));
  } finally {
    operation.inFlight = false;
  }
}

/** ✅️ Approves exactly the offered proposal, echoing back the server's own hash. It is never
 * retried: a duplicate approval could otherwise ask for a second Map commit. */
export async function approveInferenceProposal(ctx: GisInferenceContext, operationEpoch: number): Promise<void> {
  const operation = liveInferencePort(ctx, operationEpoch);
  if (
    operation === null ||
    operation.status.phase !== "offered" ||
    operation.status.jobId === null ||
    operation.status.proposalHash === null ||
    operation.status.preview?.jobId !== operation.status.jobId ||
    operation.status.preview.proposalHash !== operation.status.proposalHash ||
    operation.closeRequested ||
    operation.status.cancelRequested ||
    operation.inFlight
  )
    return;
  let body: string;
  try {
    body = gisMapInferenceRequestToJson(sealGisMapInferenceApprovalRequestV1(operation.status.jobId, operation.status.proposalHash));
  } catch {
    terminateInferencePort(ctx, operation, "inference.invalid");
    return;
  }
  const jobId = operation.status.jobId;
  advanceInferencePort(ctx, operation, { kind: "approve" });
  operation.inFlight = true;
  try {
    const response = await inferenceBrokerFetch(ctx, operation, `/jobs/${jobId}/approval`, { method: "POST", body });
    if (!response.ok) throw new DirectoryHttpError(response.status, "");
    const receipt = parseGisMapInferenceApprovalReceiptV1(await readInferenceJson(response));
    if (liveInferencePort(ctx, operationEpoch) !== operation) return;
    retainInferenceApprovalUndo(ctx, operation, receipt);
    advanceInferencePort(ctx, operation, { kind: "approval", receipt });
    retireInferencePort(ctx, operation);
  } catch (error) {
    if (operation.closed) return;
    terminateInferencePort(ctx, operation, inferenceCodeFromRejection(error, operation.abort.signal.aborted));
  } finally {
    operation.inFlight = false;
  }
}

/** ↩️ Routes the ordinary Shell history action through the one exact mounted durable approval
 * owner. The retry key and Hub handle stay stable and private across an indeterminate response. */
export async function undoInferenceApproval(ctx: GisInferenceContext, historyEpoch: number, clientInstanceId: string, scope: DocumentScope): Promise<void> {
  const { sameBrowserSessionOperationFence, artifactState } = ctx.host;
  const owner = ctx.state.inferenceApprovalUndoOwner;
  if (owner === null || owner.historyEpoch !== historyEpoch || owner.clientInstanceId !== clientInstanceId || owner.scope.spaceId !== scope.spaceId || owner.scope.documentId !== scope.documentId || (owner.phase !== "available" && !(owner.phase === "failed" && owner.retryable))) return;
  if (!sameBrowserSessionOperationFence(owner.sessionFence)) {
    retireInferenceApprovalUndoForAuthority(ctx, owner);
    return;
  }
  const state = artifactState(scope.documentId, scope.spaceId);
  if (state === undefined || !sameApprovalUndoMountV1(ctx, state, owner)) {
    retireInferenceApprovalUndo(ctx, owner);
    return;
  }
  owner.phase = "submitting";
  approvalUndoStatus(ctx, owner, { phase: "submitting", canUndo: false, code: null });
  try {
    const request = sealGisMapApprovalUndoRequestV1(owner.receipt.undo, owner.idempotencyKey);
    const response = await inferenceApprovalUndoBrokerFetch(ctx, owner, JSON.stringify(request));
    if (!response.ok) throw new DirectoryHttpError(response.status, "");
    const receipt: GisMapApprovalUndoReceiptV1 = parseGisMapApprovalUndoReceiptV1(await readInferenceJson(response));
    if (ctx.state.inferenceApprovalUndoOwner !== owner || !sameBrowserSessionOperationFence(owner.sessionFence)) return;
    const currentState = artifactState(owner.scope.documentId, owner.scope.spaceId);
    if (currentState === undefined || !sameApprovalUndoMountV1(ctx, currentState, owner)) {
      retireInferenceApprovalUndo(ctx, owner);
      return;
    }
    if (!receipt.applied || receipt.targetId !== owner.receipt.undo.targetId || receipt.originalJobId !== owner.receipt.jobId || receipt.frontier.documentId !== owner.scope.documentId) throw new Error("gis map approval undo: receipt mismatch");
    approvalUndoStatus(ctx, owner, { phase: "applied", canUndo: false, code: null });
    ctx.state.inferenceApprovalUndoOwner = null;
    owner.abort.abort(new Error("gis map approval undo applied"));
  } catch (error) {
    if (ctx.state.inferenceApprovalUndoOwner !== owner || owner.abort.signal.aborted) return;
    const status = error instanceof DirectoryHttpError ? error.status : undefined;
    const code = status === undefined ? "inference.transport" : gisMapInferenceCodeFromStatusV1(status);
    const sameSession = sameBrowserSessionOperationFence(owner.sessionFence);
    owner.phase = "failed";
    owner.retryable = sameSession && (status === undefined || status === 429 || status === 503);
    approvalUndoStatus(ctx, owner, { phase: "failed", canUndo: owner.retryable, code });
    if (!owner.retryable && sameSession) {
      ctx.state.inferenceApprovalUndoOwner = null;
      owner.abort.abort(new Error("gis map approval undo rejected"));
    }
  }
}

/** 🛑️ Closing records cancellation without aborting a submit whose Hub receipt may still arrive. */
export function closeInferencePort(ctx: GisInferenceContext, operationEpoch: number): void {
  const operation = ctx.state.inferencePort;
  if (operation === null || operation.operationEpoch !== operationEpoch) return;
  operation.closeRequested = true;
  if (operation.status.phase === "idle" || gisMapInferencePortTerminalV1(operation.status.phase)) {
    retireInferencePort(ctx, operation);
    return;
  }
  if (operation.status.phase === "indeterminate") {
    advanceInferencePort(ctx, operation, { kind: "cancel" });
    void driveInferencePort(ctx, operationEpoch);
    return;
  }
  void cancelInferenceJob(ctx, operationEpoch);
}
