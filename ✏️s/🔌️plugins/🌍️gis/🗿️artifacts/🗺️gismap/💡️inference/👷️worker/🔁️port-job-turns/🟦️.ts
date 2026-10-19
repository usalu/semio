/** 🔁️ The job turns of a retained inference port: lookup, termination, poll scheduling, driving, reconciliation, polling and cancellation.
 *
 * 🧱️ Child module of {@link ../🟦️.ts | createGisMapInferenceWorkerV1}: the functions take the worker context `ctx` explicitly instead of closing over the factory. */
import { gisMapInferencePortTerminalV1, parseGisMapInferenceApprovalReceiptV1, parseGisMapInferenceEventPageV1, type GisMapInferencePortCodeV1 } from "../../🧬️schema/🟦️.ts";
import { parseInferenceReconcilePayloadV1 } from "../../🔎️reconcile/🟦️.ts";
import { JOB_RECONCILE_REQUEST_SCHEMA_V1, parseJobReconcileRequestV1, parseJobReconcileResultV1 } from "@semio-tech/framework/job/reconcile/schema";
import { DirectoryHttpError } from "@semio-tech/framework/inference/service/worker-host";
import { INFERENCE_MAX_POLL_TURNS, INFERENCE_POLL_INTERVAL_MS } from "../🧰️worker-context-and-types/🟦️.ts";
import type { GisInferenceContext, InferenceOperationV1 } from "../🧰️worker-context-and-types/🟦️.ts";
import { retainInferenceApprovalUndo } from "../↩️approval-undo-owner/🟦️.ts";
import { advanceInferencePort, inferenceBrokerFetch, inferenceCodeFromRejection, inferenceLeaseVerified, readInferenceJson, retireInferencePort } from "./📡️port-status-and-broker/🟦️.ts";

/** 🔎️ Finds the retained epoch; lease retirement records closing without losing a pending receipt. */
export function liveInferencePort(ctx: GisInferenceContext, operationEpoch: number): InferenceOperationV1 | null {
  const { sameBrowserSessionOperationFence } = ctx.host;
  const operation = ctx.state.inferencePort;
  if (operation === null || operation.closed || operation.operationEpoch !== operationEpoch) return null;
  if (operation.sessionEpoch !== ctx.host.directorySessionEpoch || !sameBrowserSessionOperationFence(operation.sessionFence)) {
    terminateInferencePort(ctx, operation, "inference.transport");
    return null;
  }
  if (!inferenceLeaseVerified(ctx, operation.scope)) {
    operation.closeRequested = true;
    advanceInferencePort(ctx, operation, { kind: "cancel" });
  }
  return operation;
}

/** 🧯️ Pre-submit failures can retire; uncertain submitted work keeps its original private owner. */
export function terminateInferencePort(ctx: GisInferenceContext, operation: InferenceOperationV1, code: GisMapInferencePortCodeV1): void {
  const { sameBrowserSessionOperationFence } = ctx.host;
  if (operation.closed) return;
  if (operation.pollTimer !== null) clearTimeout(operation.pollTimer);
  operation.pollTimer = null;
  if (operation.request === null) {
    advanceInferencePort(ctx, operation, { kind: "failed", code });
    retireInferencePort(ctx, operation);
    return;
  }
  operation.reconcileRequired = true;
  advanceInferencePort(ctx, operation, { kind: "indeterminate", code: code === "inference.cancelled" ? "inference.transport" : code });
  if (operation.turns < INFERENCE_MAX_POLL_TURNS && operation.sessionEpoch === ctx.host.directorySessionEpoch && sameBrowserSessionOperationFence(operation.sessionFence)) scheduleInferencePoll(ctx, operation);
}

/** ⏱️ Arms exactly one bounded next poll turn; a terminal phase or an exhausted turn budget arms none. */
export function scheduleInferencePoll(ctx: GisInferenceContext, operation: InferenceOperationV1): void {
  if (operation.pollTimer !== null) clearTimeout(operation.pollTimer);
  operation.pollTimer = null;
  if (operation.closed || gisMapInferencePortTerminalV1(operation.status.phase)) return;
  if (operation.turns >= INFERENCE_MAX_POLL_TURNS) {
    terminateInferencePort(ctx, operation, "inference.transport");
    return;
  }
  operation.pollTimer = setTimeout(() => {
    operation.pollTimer = null;
    void driveInferencePort(ctx, operation.operationEpoch);
  }, INFERENCE_POLL_INTERVAL_MS);
}

/** 🔄️ One bounded turn: a recorded-but-unsent cancellation always outranks the next progress read,
 * so a Cancel clicked while another call was in flight is transmitted on the very next turn. */
export async function driveInferencePort(ctx: GisInferenceContext, operationEpoch: number): Promise<void> {
  const operation = liveInferencePort(ctx, operationEpoch);
  if (operation === null || operation.inFlight) return;
  if (operation.reconcileRequired || operation.status.phase === "indeterminate") {
    await reconcileInferenceJob(ctx, operation);
    return;
  }
  if (operation.status.cancelRequested && !operation.cancelSent) {
    await cancelInferenceJob(ctx, operationEpoch);
    return;
  }
  await pollInferenceJob(ctx, operationEpoch);
}

/** 🩹️ Recovers only the original accepted request; this route cannot submit another job. */
export async function reconcileInferenceJob(ctx: GisInferenceContext, operation: InferenceOperationV1): Promise<void> {
  if (operation.request === null || operation.inFlight || operation.closed) return;
  operation.turns += 1;
  operation.inFlight = true;
  try {
    const request = parseJobReconcileRequestV1({ schema: JOB_RECONCILE_REQUEST_SCHEMA_V1, version: 1, requestId: operation.request.requestId });
    const response = await inferenceBrokerFetch(ctx, operation, "/jobs/reconcile", { method: "POST", body: JSON.stringify(request) });
    if (!response.ok) throw new DirectoryHttpError(response.status, "");
    const result = parseJobReconcileResultV1(await readInferenceJson(response), parseInferenceReconcilePayloadV1);
    if (liveInferencePort(ctx, operation.operationEpoch) !== operation) return;
    if (result.requestId !== operation.request.requestId) throw new Error("gis map inference: different request");
    const job = result.job;
    if (!result.found || job === null) {
      terminateInferencePort(ctx, operation, "inference.transport");
      return;
    }
    if (operation.status.jobId !== null && operation.status.jobId !== job.receipt.jobId) throw new Error("gis map inference: different job");
    operation.reconcileRequired = false;
    if (job.approval !== null) {
      operation.status = { ...operation.status, jobId: job.receipt.jobId, proposalHash: job.receipt.proposalHash };
      if (job.approval.state === "undo-prepared") {
        terminateInferencePort(ctx, operation, "inference.transport");
        scheduleInferencePoll(ctx, operation);
        return;
      }
      if (job.approval.receipt !== null) {
        const receipt = parseGisMapInferenceApprovalReceiptV1(job.approval.receipt);
        retainInferenceApprovalUndo(ctx, operation, receipt);
        advanceInferencePort(ctx, operation, { kind: "indeterminate", code: "inference.transport" });
        advanceInferencePort(ctx, operation, { kind: "approval", receipt });
        retireInferencePort(ctx, operation);
        return;
      }
    } else if (operation.status.jobId === null) {
      advanceInferencePort(ctx, operation, { kind: "receipt", receipt: job.receipt });
    }
    const { expired: _expired, ...page } = job.page;
    operation.cancelSent = job.page.cancelRequested;
    advanceInferencePort(ctx, operation, { kind: "page", page: { ...page, schema: "semio.hub.inference-job-events/v1", stale: page.proposalState === "stale" } });
    if (gisMapInferencePortTerminalV1(operation.status.phase)) retireInferencePort(ctx, operation);
    else scheduleInferencePoll(ctx, operation);
  } catch (error) {
    if (!operation.closed) terminateInferencePort(ctx, operation, inferenceCodeFromRejection(error, false));
  } finally {
    operation.inFlight = false;
  }
}

/** 📃️ Reads exactly one bounded owner-private page and folds it through the shared reducer. */
export async function pollInferenceJob(ctx: GisInferenceContext, operationEpoch: number): Promise<void> {
  const operation = liveInferencePort(ctx, operationEpoch);
  if (operation === null || operation.inFlight || operation.status.jobId === null) return;
  operation.turns += 1;
  operation.inFlight = true;
  try {
    const response = await inferenceBrokerFetch(ctx, operation, `/jobs/${operation.status.jobId}/events?after=${operation.status.cursor}`, { method: "GET" });
    if (!response.ok) throw new DirectoryHttpError(response.status, "");
    const page = parseGisMapInferenceEventPageV1(await readInferenceJson(response));
    if (liveInferencePort(ctx, operationEpoch) !== operation) return;
    if (page.proposalState === "approved") {
      terminateInferencePort(ctx, operation, "inference.transport");
      scheduleInferencePoll(ctx, operation);
      return;
    }
    advanceInferencePort(ctx, operation, { kind: "page", page });
    if (gisMapInferencePortTerminalV1(operation.status.phase)) {
      retireInferencePort(ctx, operation);
      return;
    }
    scheduleInferencePoll(ctx, operation);
  } catch (error) {
    if (operation.closed) return;
    terminateInferencePort(ctx, operation, inferenceCodeFromRejection(error, operation.abort.signal.aborted));
  } finally {
    operation.inFlight = false;
  }
}

/** 🛑️ Requests cancellation. The phase does NOT move optimistically: only the server's own answer
 * may report `cancelled`, so a hub that refuses the cancel can never be misreported as honoured. */
export async function cancelInferenceJob(ctx: GisInferenceContext, operationEpoch: number): Promise<void> {
  const operation = liveInferencePort(ctx, operationEpoch);
  if (operation === null) return;
  advanceInferencePort(ctx, operation, { kind: "cancel" });
  if (operation.status.jobId === null || operation.inFlight || operation.cancelSent) return;
  operation.cancelSent = true;
  operation.inFlight = true;
  try {
    const response = await inferenceBrokerFetch(ctx, operation, `/jobs/${operation.status.jobId}/cancel`, { method: "POST" });
    if (!response.ok) throw new DirectoryHttpError(response.status, "");
    const page = parseGisMapInferenceEventPageV1(await readInferenceJson(response));
    if (liveInferencePort(ctx, operationEpoch) !== operation) return;
    if (page.proposalState === "approved") {
      terminateInferencePort(ctx, operation, "inference.transport");
      scheduleInferencePoll(ctx, operation);
      return;
    }
    advanceInferencePort(ctx, operation, { kind: "page", page });
    if (gisMapInferencePortTerminalV1(operation.status.phase)) retireInferencePort(ctx, operation);
    else scheduleInferencePoll(ctx, operation);
  } catch (error) {
    if (operation.closed) return;
    terminateInferencePort(ctx, operation, inferenceCodeFromRejection(error, operation.abort.signal.aborted));
  } finally {
    operation.inFlight = false;
  }
}
