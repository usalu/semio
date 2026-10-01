import { GIS_MAP_INFERENCE_RESPONSE_MAX_BYTES, gisMapInferenceCodeFromStatusV1, gisMapInferencePortTerminalV1, idleGisMapInferencePortStatusV1, parseGisMapInferenceApprovalReceiptV1, parseGisMapApprovalUndoReceiptV1, parseGisMapInferenceEventPageV1, parseGisMapInferenceJobReceiptV1, reduceGisMapInferencePortV1, sealGisMapApprovalUndoRequestV1, sealGisMapInferenceApprovalRequestV1, sealGisMapInferenceJobRequestV1, type GisMapApprovalHistoryStatusV1, type GisMapApprovalUndoReceiptV1, type GisMapInferenceApprovalReceiptV1, type GisMapInferenceJobRequestV1, type GisMapInferencePortCodeV1, type GisMapInferencePortEventV1, type GisMapInferencePortStatusV1 } from "../🧬️schema/🟦️.ts";
import { parseInferenceReconcilePayloadV1 } from "../🔎️reconcile/🟦️.ts";
import { JOB_RECONCILE_REQUEST_SCHEMA_V1, parseJobReconcileRequestV1, parseJobReconcileResultV1 } from "../../../../../../../🧰️framework/🔨️modules/🧵️job/🔎️reconcile/🧬️schema/🟦️.ts";
import { DirectoryHttpError, type DocumentScope, type BackboneWorkerResponse, type DocumentExecutionTargetLeaseFieldsV1 } from "@semio-tech/framework-os";
import type { FetchTimeoutResponse } from "@semio-tech/framework";
import type { DocumentServiceWorkerHostV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts";
import type { InstalledServiceDriverV1, InstalledServiceOperationV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🔌️service/🟦️.ts";
type ArtifactState = NonNullable<ReturnType<DocumentServiceWorkerHostV1["artifactState"]>>;
type VerifiedColdDocumentPair = NonNullable<ArtifactState["verifiedColdPair"]>;
type DocumentBrowserActorReservation = NonNullable<ArtifactState["browserActorReservation"]>;
type BrowserSessionOperationFenceV1 = NonNullable<ReturnType<DocumentServiceWorkerHostV1["captureBrowserSessionOperationFence"]>>;

/** 🌍 Installs GIS lifecycle behavior over the framework document/session authority port. */
export function createGisMapInferenceWorkerV1(host: DocumentServiceWorkerHostV1) {
 const { artifactScope, artifactState, bytesHex, captureBrowserSessionOperationFence, sameBrowserSessionOperationFence, documentRuntimeKeyV1, SOCKET_GRANT_REQUEST_TIMEOUT_MS } = host;
 const post = (message: unknown) => host.post({ ...(message as Record<string, unknown>), owner: "gis", serviceId: "s.gis.gismap.inference" } as BackboneWorkerResponse);
//#region 💡️Inference
/** 💡️ One private request owner and bounded poll timer. A verified writable lease admits it;
 * cancel/reconcile cleanup outlives document retirement. Unknown outcomes retain the original
 * request without resubmission. Only the Hub's approved command mutates the document. */
const INFERENCE_PORT_CAPACITY = 1;
/** ⏱️ Bounded, jitter-free poll cadence for one running job — a `setTimeout` chain, never an
 * interval and never a busy loop. */
const INFERENCE_POLL_INTERVAL_MS = 750;
/** 🔁️ Highest number of poll turns one job may take before it is reported indeterminate. */
const INFERENCE_MAX_POLL_TURNS = 240;
/** ⏳️ Lifetime one submitted job asks the hub for. */
const INFERENCE_JOB_LIFETIME_MS = 60_000;

type InferenceOperationV1 = {
  readonly operationEpoch: number;
  readonly scope: DocumentScope;
  readonly abort: AbortController;
  readonly sessionEpoch: number;
  readonly sessionFence: BrowserSessionOperationFenceV1;
  readonly clientInstanceId: string;
  readonly leaseFields: DocumentExecutionTargetLeaseFieldsV1;
  readonly transport: ReturnType<DocumentServiceWorkerHostV1["bindDocumentService"]>;
  request: Readonly<GisMapInferenceJobRequestV1> | null;
  closeRequested: boolean;
  reconcileRequired: boolean;
  status: GisMapInferencePortStatusV1;
  turns: number;
  pollTimer: ReturnType<typeof setTimeout> | null;
  inFlight: boolean;
  /** 🛑️ A sent cancellation is retried only after reconciliation observes it was not recorded. */
  cancelSent: boolean;
  closed: boolean;
};

let inferencePort: InferenceOperationV1 | null = null;

type InferenceApprovalUndoMountV1 = Readonly<{
  activationGeneration: bigint;
  catalogGenerationId: string;
  componentSha256: string;
  descriptorSha256: string;
  browserActorSha256: string;
  directoryRevision: number;
  membershipGeneration: number;
  sessionGeneration?: number;
  shareGeneration?: number;
}>;

type InferenceApprovalUndoOwnerV1 = {
  readonly historyEpoch: number;
  readonly scope: DocumentScope;
  readonly clientInstanceId: string;
  readonly sessionEpoch: number;
  readonly sessionFence: BrowserSessionOperationFenceV1;
  readonly receipt: GisMapInferenceApprovalReceiptV1;
  readonly transport: ReturnType<DocumentServiceWorkerHostV1["bindDocumentService"]>;
  readonly idempotencyKey: string;
  readonly sourceCatalogGenerationId: string;
  readonly sourceComponentSha256: string;
  readonly sourceDescriptorSha256: string;
  readonly sourceBrowserActorSha256: string;
  readonly sourceDirectoryRevision: number;
  readonly sourceMembershipGeneration: number;
  readonly sourceSessionGeneration?: number;
  readonly sourceShareGeneration?: number;
  readonly abort: AbortController;
  mount: InferenceApprovalUndoMountV1 | null;
  phase: "awaiting-mount" | "available" | "submitting" | "failed";
  retryable: boolean;
};

let inferenceApprovalUndoEpoch = 0;
let inferenceApprovalUndoOwner: InferenceApprovalUndoOwnerV1 | null = null;

function mintInferenceApprovalUndoIdempotencyKeyV1(): string {
  const bytes = crypto.getRandomValues(new Uint8Array(16));
  bytes[0] = (bytes[0] ?? 0) | 1;
  return Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");
}

function approvalUndoStatus(owner: InferenceApprovalUndoOwnerV1, status: GisMapApprovalHistoryStatusV1): void {
  post({ owner: "gis", serviceId: "s.gis.gismap.inference", kind: "inference-history-status", historyEpoch: owner.historyEpoch, clientInstanceId: owner.clientInstanceId, scope: owner.scope, status });
}

function retireInferenceApprovalUndoForAuthority(owner: InferenceApprovalUndoOwnerV1): void {
  if (owner.phase !== "submitting" && owner.phase !== "failed") {
    retireInferenceApprovalUndo(owner);
    return;
  }
  if (owner.phase === "failed" && !owner.retryable) return;
  owner.abort.abort(new Error("gis map approval undo authority retired"));
  owner.phase = "failed";
  owner.retryable = false;
  approvalUndoStatus(owner, { phase: "failed", canUndo: false, code: "inference.transport" });
}

function sameApprovalUndoFrontierV1(owner: InferenceApprovalUndoOwnerV1, pair: VerifiedColdDocumentPair): boolean {
  const expected = owner.receipt.undo.expectedCurrent;
  return pair.frontier.documentId === expected.documentId
    && pair.frontier.headEditOrdinal === BigInt(expected.headEditOrdinal)
    && pair.frontier.headEditId === expected.headEditId
    && pair.frontier.lastCommitSeq === BigInt(expected.lastCommitSeq)
    && bytesHex(pair.frontier.chainSha256) === expected.chainSha256;
}

function sameApprovalUndoMountV1(state: ArtifactState, owner: InferenceApprovalUndoOwnerV1): boolean {
  const lease = state.executionTargetLease;
  const reservation = state.browserActorReservation;
  const pair = state.verifiedColdPair;
  const mount = owner.mount;
  if (!lease?.live || reservation === null || pair === null || mount === null || state.closed || state.docAbort.signal.aborted) return false;
  const fields = lease.fields();
  return state.openClientInstanceId === owner.clientInstanceId
    && fields.scope.spaceId === owner.scope.spaceId
    && fields.scope.documentId === owner.scope.documentId
    && reservation.generation === mount.activationGeneration
    && fields.catalog.generationId === mount.catalogGenerationId
    && fields.package.componentSha256 === mount.componentSha256
    && fields.package.descriptorByteSha256 === mount.descriptorSha256
    && fields.browserActor.kind === "closed-browser-actor"
    && fields.browserActor.sha256 === mount.browserActorSha256
    && fields.revalidation.directoryRevision === mount.directoryRevision
    && fields.revalidation.membershipGeneration === mount.membershipGeneration
    && fields.revalidation.sessionGeneration === mount.sessionGeneration
    && fields.revalidation.shareGeneration === mount.shareGeneration
    && sameApprovalUndoFrontierV1(owner, pair);
}

function retireInferenceApprovalUndo(owner: InferenceApprovalUndoOwnerV1, notify = true): void {
  if (inferenceApprovalUndoOwner !== owner) return;
  inferenceApprovalUndoOwner = null;
  owner.abort.abort(new Error("gis map approval undo owner retired"));
  if (notify) approvalUndoStatus(owner, { phase: "unavailable", canUndo: false, code: null });
}

function reissueInferenceApprovalUndoForRebootstrap(state: ArtifactState): void {
  const owner = inferenceApprovalUndoOwner;
  if (owner === null || owner.scope.spaceId !== artifactScope(state)?.spaceId || owner.scope.documentId !== state.config.documentId || owner.clientInstanceId !== state.openClientInstanceId) return;
  if (!sameBrowserSessionOperationFence(owner.sessionFence)) {
    retireInferenceApprovalUndoForAuthority(owner);
    return;
  }
  inferenceApprovalUndoOwner = null;
  owner.abort.abort(new Error("gis map approval undo owner rebootstrap"));
  approvalUndoStatus(owner, { phase: "unavailable", canUndo: false, code: null });
  if (inferenceApprovalUndoEpoch >= Number.MAX_SAFE_INTEGER) return;
  inferenceApprovalUndoOwner = {
    historyEpoch: ++inferenceApprovalUndoEpoch,
    scope: structuredClone(owner.scope),
    clientInstanceId: owner.clientInstanceId,
    sessionEpoch: owner.sessionEpoch,
    sessionFence: owner.sessionFence,
    receipt: structuredClone(owner.receipt),
    transport: owner.transport,
    idempotencyKey: owner.idempotencyKey,
    sourceCatalogGenerationId: owner.sourceCatalogGenerationId,
    sourceComponentSha256: owner.sourceComponentSha256,
    sourceDescriptorSha256: owner.sourceDescriptorSha256,
    sourceBrowserActorSha256: owner.sourceBrowserActorSha256,
    sourceDirectoryRevision: owner.sourceDirectoryRevision,
    sourceMembershipGeneration: owner.sourceMembershipGeneration,
    ...(owner.sourceSessionGeneration === undefined ? {} : { sourceSessionGeneration: owner.sourceSessionGeneration }),
    ...(owner.sourceShareGeneration === undefined ? {} : { sourceShareGeneration: owner.sourceShareGeneration }),
    abort: new AbortController(),
    mount: null,
    phase: "awaiting-mount",
    retryable: true,
  };
}

function bindInferenceApprovalUndoToMountedPair(state: ArtifactState, reservation: DocumentBrowserActorReservation, pair: VerifiedColdDocumentPair): void {
  const owner = inferenceApprovalUndoOwner;
  const lease = state.executionTargetLease;
  if (owner === null || owner.phase !== "awaiting-mount" || lease === null || state.browserActorReservation !== reservation || state.verifiedColdPair !== pair) return;
  if (!sameBrowserSessionOperationFence(owner.sessionFence)) {
    retireInferenceApprovalUndo(owner);
    return;
  }
  const scope = artifactScope(state);
  if (!scope || scope.spaceId !== owner.scope.spaceId || scope.documentId !== owner.scope.documentId || state.openClientInstanceId !== owner.clientInstanceId) return;
  if (!sameApprovalUndoFrontierV1(owner, pair)) {
    retireInferenceApprovalUndo(owner);
    return;
  }
  const fields = lease.fields();
  if (
    fields.catalog.generationId !== owner.sourceCatalogGenerationId
    || fields.package.componentSha256 !== owner.sourceComponentSha256
    || fields.package.descriptorByteSha256 !== owner.sourceDescriptorSha256
    || fields.browserActor.kind !== "closed-browser-actor"
    || fields.browserActor.sha256 !== owner.sourceBrowserActorSha256
    || fields.revalidation.directoryRevision !== owner.sourceDirectoryRevision
    || fields.revalidation.membershipGeneration !== owner.sourceMembershipGeneration
    || fields.revalidation.sessionGeneration !== owner.sourceSessionGeneration
    || fields.revalidation.shareGeneration !== owner.sourceShareGeneration
  ) {
    retireInferenceApprovalUndo(owner);
    return;
  }
  owner.mount = Object.freeze({
    activationGeneration: reservation.generation,
    catalogGenerationId: fields.catalog.generationId,
    componentSha256: fields.package.componentSha256,
    descriptorSha256: fields.package.descriptorByteSha256,
    browserActorSha256: fields.browserActor.sha256,
    directoryRevision: fields.revalidation.directoryRevision,
    membershipGeneration: fields.revalidation.membershipGeneration,
    ...(fields.revalidation.sessionGeneration === undefined ? {} : { sessionGeneration: fields.revalidation.sessionGeneration }),
    ...(fields.revalidation.shareGeneration === undefined ? {} : { shareGeneration: fields.revalidation.shareGeneration }),
  });
  owner.phase = "available";
  owner.retryable = true;
  approvalUndoStatus(owner, { phase: "available", canUndo: true, code: null });
}

function retainInferenceApprovalUndo(operation: InferenceOperationV1, receipt: GisMapInferenceApprovalReceiptV1): void {
  const state = artifactState(operation.scope.documentId, operation.scope.spaceId);
  if (state === undefined || state.closed || state.openClientInstanceId !== operation.clientInstanceId || operation.sessionEpoch !== host.directorySessionEpoch || !sameBrowserSessionOperationFence(operation.sessionFence)) return;
  const fields = operation.leaseFields;
  if (!receipt.applied || receipt.undo.expectedCurrent.documentId !== operation.scope.documentId || operation.clientInstanceId.length === 0 || fields.browserActor.kind !== "closed-browser-actor") throw new Error("gis map approval undo: invalid source owner");
  if (inferenceApprovalUndoOwner !== null) {
    if (inferenceApprovalUndoOwner.phase === "failed" && !inferenceApprovalUndoOwner.retryable) throw new Error("gis map approval undo: capacity");
    retireInferenceApprovalUndo(inferenceApprovalUndoOwner);
  }
  if (inferenceApprovalUndoEpoch >= Number.MAX_SAFE_INTEGER) throw new Error("gis map approval undo: epoch exhausted");
  const owner: InferenceApprovalUndoOwnerV1 = {
    historyEpoch: ++inferenceApprovalUndoEpoch,
    scope: structuredClone(operation.scope),
    clientInstanceId: operation.clientInstanceId,
    sessionEpoch: operation.sessionEpoch,
    sessionFence: operation.sessionFence,
    receipt: structuredClone(receipt),
    transport: operation.transport,
    idempotencyKey: mintInferenceApprovalUndoIdempotencyKeyV1(),
    sourceCatalogGenerationId: fields.catalog.generationId,
    sourceComponentSha256: fields.package.componentSha256,
    sourceDescriptorSha256: fields.package.descriptorByteSha256,
    sourceBrowserActorSha256: fields.browserActor.sha256,
    sourceDirectoryRevision: fields.revalidation.directoryRevision,
    sourceMembershipGeneration: fields.revalidation.membershipGeneration,
    ...(fields.revalidation.sessionGeneration === undefined ? {} : { sourceSessionGeneration: fields.revalidation.sessionGeneration }),
    ...(fields.revalidation.shareGeneration === undefined ? {} : { sourceShareGeneration: fields.revalidation.shareGeneration }),
    abort: new AbortController(),
    mount: null,
    phase: "awaiting-mount",
    retryable: true,
  };
  inferenceApprovalUndoOwner = owner;
  state?.browserActorReservation?.bindApprovalUndoIfMounted();
}

/** 🔭️ Observation seam mirroring {@link executionTargetStatusObserver}: a harness without a worker
 * scope still sees the exact bounded payload the renderer would receive. */
let inferencePortStatusObserver: ((status: Extract<BackboneWorkerResponse, { kind: "inference-port-status" }>) => void) | null = null;

function postInferencePortStatus(operation: InferenceOperationV1): void {
  const status: Extract<BackboneWorkerResponse, { kind: "inference-port-status" }> = { kind: "inference-port-status", operationEpoch: operation.operationEpoch, scope: operation.scope, status: { owner: "gis", serviceId: "s.gis.gismap.inference", payload: operation.status } };
  inferencePortStatusObserver?.(status);
  post(status);
}

/** 🧮️ Applies one closed event through the shared reducer and publishes only on a real change. */
function advanceInferencePort(operation: InferenceOperationV1, event: GisMapInferencePortEventV1): void {
  const next = reduceGisMapInferencePortV1(operation.status, event);
  if (next === operation.status) return;
  operation.status = next;
  postInferencePortStatus(operation);
}

/** 🪪️ The precondition: only a document whose worker-private execution-target lease is minted AND
 * still live may open a port. A dropped, absent or non-writable lease is refused before any request
 * exists, and the refusal is a localized terminal, never a silent no-op. */
function inferenceLeaseVerified(scope: DocumentScope): boolean {
  const state = artifactState(scope.documentId, scope.spaceId);
  if (state === undefined || state.closed || state.docAbort.signal.aborted) return false;
  const lease = state.executionTargetLease;
  if (lease === null || !lease.live) return false;
  const fields = lease.fields();
  return fields.scope.spaceId === scope.spaceId && fields.scope.documentId === scope.documentId && fields.grant.write;
}

async function inferenceBrokerFetch(operation: InferenceOperationV1, suffix: string, init: { readonly method: "GET" | "POST"; readonly body?: string }): Promise<FetchTimeoutResponse> {
  if (operation.sessionEpoch !== host.directorySessionEpoch || !sameBrowserSessionOperationFence(operation.sessionFence)) throw new Error("gis map inference: original session unavailable");
  if ((suffix === "/jobs" || suffix.endsWith("/approval")) && !inferenceLeaseVerified(operation.scope)) throw new Error("gis map inference: document closed");
  const job = /^\/jobs\/([0-9a-f]{32})\/(events\?after=(\d{1,3})|cancel|approval)$/u.exec(suffix);
  const action = suffix === "/jobs" ? "submit" : suffix === "/jobs/reconcile" ? "reconcile" : job?.[2]?.startsWith("events") ? "events" : job?.[2] === "cancel" ? "cancel" : job?.[2] === "approval" ? "approve" : null;
  if (action === null) throw new Error("gis map inference: operation denied");
  const payload = init.body === undefined ? { jobId: job![1], ...(action === "events" ? { after: Number(job![3]) } : {}) } : JSON.parse(init.body);
  return operation.transport.call(action, payload, { timeoutMs: SOCKET_GRANT_REQUEST_TIMEOUT_MS, signal: operation.abort.signal,
    admit: () => operation.sessionEpoch === host.directorySessionEpoch && sameBrowserSessionOperationFence(operation.sessionFence) && !operation.closed && ((suffix !== "/jobs" && !suffix.endsWith("/approval")) || inferenceLeaseVerified(operation.scope)),
    retain: () => operation.sessionEpoch === host.directorySessionEpoch && sameBrowserSessionOperationFence(operation.sessionFence) && !operation.closed,
  });
}

async function inferenceApprovalUndoBrokerFetch(owner: InferenceApprovalUndoOwnerV1, body: string): Promise<FetchTimeoutResponse> {
  const state = artifactState(owner.scope.documentId, owner.scope.spaceId);
  if (state === undefined || state.closed || state.openClientInstanceId !== owner.clientInstanceId || state.docAbort.signal.aborted || !sameBrowserSessionOperationFence(owner.sessionFence)) throw new Error("gis map approval undo: document closed");
  return owner.transport.call("undo", JSON.parse(body), { timeoutMs: SOCKET_GRANT_REQUEST_TIMEOUT_MS, signal: owner.abort.signal,
    admit: () => { const current = artifactState(owner.scope.documentId, owner.scope.spaceId); return inferenceApprovalUndoOwner === owner && owner.sessionEpoch === host.directorySessionEpoch && sameBrowserSessionOperationFence(owner.sessionFence) && current !== undefined && sameApprovalUndoMountV1(current, owner); },
    retain: () => owner.sessionEpoch === host.directorySessionEpoch && sameBrowserSessionOperationFence(owner.sessionFence),
  });
}

/** 📥️ Reads one bounded owner-private JSON body under the shared response maximum. */
async function readInferenceJson(response: FetchTimeoutResponse): Promise<unknown> {
  const declared = response.headers.get("content-length");
  if (declared !== null && !(Number.isSafeInteger(Number(declared)) && Number(declared) >= 1 && Number(declared) <= GIS_MAP_INFERENCE_RESPONSE_MAX_BYTES)) throw new Error("gis map inference: invalid body");
  const text = await response.text();
  if (text.length === 0 || new TextEncoder().encode(text).length > GIS_MAP_INFERENCE_RESPONSE_MAX_BYTES) throw new Error("gis map inference: invalid body");
  return JSON.parse(text);
}

/** 🔎️ Finds the retained epoch; lease retirement records closing without losing a pending receipt. */
function liveInferencePort(operationEpoch: number): InferenceOperationV1 | null {
  const operation = inferencePort;
  if (operation === null || operation.closed || operation.operationEpoch !== operationEpoch) return null;
  if (operation.sessionEpoch !== host.directorySessionEpoch || !sameBrowserSessionOperationFence(operation.sessionFence)) {
    terminateInferencePort(operation, "inference.transport");
    return null;
  }
  if (!inferenceLeaseVerified(operation.scope)) {
    operation.closeRequested = true;
    advanceInferencePort(operation, { kind: "cancel" });
  }
  return operation;
}

/** 🧯️ Pre-submit failures can retire; uncertain submitted work keeps its original private owner. */
function terminateInferencePort(operation: InferenceOperationV1, code: GisMapInferencePortCodeV1): void {
  if (operation.closed) return;
  if (operation.pollTimer !== null) clearTimeout(operation.pollTimer);
  operation.pollTimer = null;
  if (operation.request === null) {
    advanceInferencePort(operation, { kind: "failed", code });
    retireInferencePort(operation);
    return;
  }
  operation.reconcileRequired = true;
  advanceInferencePort(operation, { kind: "indeterminate", code: code === "inference.cancelled" ? "inference.transport" : code });
  if (operation.turns < INFERENCE_MAX_POLL_TURNS && operation.sessionEpoch === host.directorySessionEpoch && sameBrowserSessionOperationFence(operation.sessionFence)) scheduleInferencePoll(operation);
}

function inferenceCodeFromRejection(error: unknown, aborted: boolean): GisMapInferencePortCodeV1 {
  if (aborted) return "inference.cancelled";
  const status = error instanceof DirectoryHttpError ? error.status : undefined;
  return status === undefined ? "inference.transport" : gisMapInferenceCodeFromStatusV1(status);
}

/** 🆕️ Installs the one retained port for exactly one document scope without replacing a predecessor. The
 * lease precondition is checked BEFORE the operation exists, so a refusal never leaves a port open. */
function openInferencePort(operationEpoch: number, scope: DocumentScope): void {
  if (!Number.isSafeInteger(operationEpoch) || operationEpoch < 1 || scope.spaceId.length === 0 || scope.documentId.length === 0) {
    post({ kind: "inference-port-opened", operationEpoch, scope, outcome: "refused", code: "inference.invalid" });
    return;
  }
  if (inferencePort !== null && INFERENCE_PORT_CAPACITY === 1) {
    post({ kind: "inference-port-opened", operationEpoch, scope, outcome: "refused", code: "inference.capacity" });
    return;
  }
  if (!inferenceLeaseVerified(scope)) {
    post({ kind: "inference-port-opened", operationEpoch, scope, outcome: "refused", code: "inference.lease-unverified" });
    return;
  }
  const sessionFence = captureBrowserSessionOperationFence();
  if (sessionFence === null) {
    post({ kind: "inference-port-opened", operationEpoch, scope, outcome: "refused", code: "inference.denied" });
    return;
  }
  const state = artifactState(scope.documentId, scope.spaceId)!;
  let transport: ReturnType<DocumentServiceWorkerHostV1["bindDocumentService"]>;
  try { transport = host.bindDocumentService("gis", "s.gis.gismap.inference", scope); }
  catch { post({ kind: "inference-port-opened", operationEpoch, scope, outcome: "refused", code: "inference.unavailable" }); return; }
  const operation: InferenceOperationV1 = { operationEpoch, scope: Object.freeze({ ...scope }), abort: new AbortController(), sessionEpoch: host.directorySessionEpoch, sessionFence, clientInstanceId: state.openClientInstanceId, leaseFields: structuredClone(state.executionTargetLease!.fields()), transport, request: null, closeRequested: false, reconcileRequired: false, status: idleGisMapInferencePortStatusV1(), turns: 0, pollTimer: null, inFlight: false, cancelSent: false, closed: false };
  inferencePort = operation;
  post({ kind: "inference-port-opened", operationEpoch, scope, outcome: "opened", code: null });
  postInferencePortStatus(operation);
}

/** 📮️ Seals one request before dispatch and never retries its submit, including after response loss. */
async function submitInferenceJob(operationEpoch: number, requestId: string): Promise<void> {
  const operation = liveInferencePort(operationEpoch);
  if (operation === null || operation.status.phase !== "idle" || operation.inFlight) return;
  let body: string;
  try {
    operation.request = Object.freeze(sealGisMapInferenceJobRequestV1(requestId, INFERENCE_JOB_LIFETIME_MS));
    body = JSON.stringify(operation.request);
  } catch {
    terminateInferencePort(operation, "inference.invalid");
    return;
  }
  advanceInferencePort(operation, { kind: "start" });
  operation.inFlight = true;
  try {
    const response = await inferenceBrokerFetch(operation, "/jobs", { method: "POST", body });
    if (!response.ok) throw new DirectoryHttpError(response.status, "");
    const receipt = parseGisMapInferenceJobReceiptV1(await readInferenceJson(response));
    if (liveInferencePort(operationEpoch) !== operation) return;
    if (receipt.proposalState === "approved") {
      operation.status = { ...operation.status, jobId: receipt.jobId, proposalHash: receipt.proposalHash ?? null };
      terminateInferencePort(operation, "inference.transport");
      scheduleInferencePoll(operation);
      return;
    }
    advanceInferencePort(operation, { kind: "receipt", receipt });
    if (gisMapInferencePortTerminalV1(operation.status.phase)) retireInferencePort(operation);
    else scheduleInferencePoll(operation);
  } catch (error) {
    if (operation.closed) return;
    terminateInferencePort(operation, inferenceCodeFromRejection(error, operation.abort.signal.aborted));
  } finally {
    operation.inFlight = false;
  }
}

/** 🔎️ Recovers only the original accepted request; this route cannot submit another job. */
async function reconcileInferenceJob(operation: InferenceOperationV1): Promise<void> {
  if (operation.request === null || operation.inFlight || operation.closed) return;
  operation.turns += 1;
  operation.inFlight = true;
  try {
    const request = parseJobReconcileRequestV1({ schema: JOB_RECONCILE_REQUEST_SCHEMA_V1, version: 1, requestId: operation.request.requestId });
    const response = await inferenceBrokerFetch(operation, "/jobs/reconcile", { method: "POST", body: JSON.stringify(request) });
    if (!response.ok) throw new DirectoryHttpError(response.status, "");
    const result = parseJobReconcileResultV1(await readInferenceJson(response), parseInferenceReconcilePayloadV1);
    if (liveInferencePort(operation.operationEpoch) !== operation) return;
    if (result.requestId !== operation.request.requestId) throw new Error("gis map inference: different request");
    const job = result.job;
    if (!result.found || job === null) {
      terminateInferencePort(operation, "inference.transport");
      return;
    }
    if (operation.status.jobId !== null && operation.status.jobId !== job.receipt.jobId) throw new Error("gis map inference: different job");
    operation.reconcileRequired = false;
    if (job.approval !== null) {
      operation.status = { ...operation.status, jobId: job.receipt.jobId, proposalHash: job.receipt.proposalHash };
      if (job.approval.state === "undo-prepared") {
        terminateInferencePort(operation, "inference.transport");
        scheduleInferencePoll(operation);
        return;
      }
      if (job.approval.receipt !== null) {
        const receipt = parseGisMapInferenceApprovalReceiptV1(job.approval.receipt);
        retainInferenceApprovalUndo(operation, receipt);
        advanceInferencePort(operation, { kind: "indeterminate", code: "inference.transport" });
        advanceInferencePort(operation, { kind: "approval", receipt });
        retireInferencePort(operation);
        return;
      }
    } else if (operation.status.jobId === null) {
      advanceInferencePort(operation, { kind: "receipt", receipt: job.receipt });
    }
    const { expired: _expired, ...page } = job.page;
    operation.cancelSent = job.page.cancelRequested;
    advanceInferencePort(operation, { kind: "page", page: { ...page, schema: "semio.hub.inference-job-events/v1", stale: page.proposalState === "stale" } });
    if (gisMapInferencePortTerminalV1(operation.status.phase)) retireInferencePort(operation);
    else scheduleInferencePoll(operation);
  } catch (error) {
    if (!operation.closed) terminateInferencePort(operation, inferenceCodeFromRejection(error, false));
  } finally {
    operation.inFlight = false;
  }
}

/** ⏱️ Arms exactly one bounded next poll turn; a terminal phase or an exhausted turn budget arms none. */
function scheduleInferencePoll(operation: InferenceOperationV1): void {
  if (operation.pollTimer !== null) clearTimeout(operation.pollTimer);
  operation.pollTimer = null;
  if (operation.closed || gisMapInferencePortTerminalV1(operation.status.phase)) return;
  if (operation.turns >= INFERENCE_MAX_POLL_TURNS) {
    terminateInferencePort(operation, "inference.transport");
    return;
  }
  operation.pollTimer = setTimeout(() => {
    operation.pollTimer = null;
    void driveInferencePort(operation.operationEpoch);
  }, INFERENCE_POLL_INTERVAL_MS);
}

/** 🔄️ One bounded turn: a recorded-but-unsent cancellation always outranks the next progress read,
 * so a Cancel clicked while another call was in flight is transmitted on the very next turn. */
async function driveInferencePort(operationEpoch: number): Promise<void> {
  const operation = liveInferencePort(operationEpoch);
  if (operation === null || operation.inFlight) return;
  if (operation.reconcileRequired || operation.status.phase === "indeterminate") {
    await reconcileInferenceJob(operation);
    return;
  }
  if (operation.status.cancelRequested && !operation.cancelSent) {
    await cancelInferenceJob(operationEpoch);
    return;
  }
  await pollInferenceJob(operationEpoch);
}

/** 📃️ Reads exactly one bounded owner-private page and folds it through the shared reducer. */
async function pollInferenceJob(operationEpoch: number): Promise<void> {
  const operation = liveInferencePort(operationEpoch);
  if (operation === null || operation.inFlight || operation.status.jobId === null) return;
  operation.turns += 1;
  operation.inFlight = true;
  try {
    const response = await inferenceBrokerFetch(operation, `/jobs/${operation.status.jobId}/events?after=${operation.status.cursor}`, { method: "GET" });
    if (!response.ok) throw new DirectoryHttpError(response.status, "");
    const page = parseGisMapInferenceEventPageV1(await readInferenceJson(response));
    if (liveInferencePort(operationEpoch) !== operation) return;
    if (page.proposalState === "approved") {
      terminateInferencePort(operation, "inference.transport");
      scheduleInferencePoll(operation);
      return;
    }
    advanceInferencePort(operation, { kind: "page", page });
    if (gisMapInferencePortTerminalV1(operation.status.phase)) {
      retireInferencePort(operation);
      return;
    }
    scheduleInferencePoll(operation);
  } catch (error) {
    if (operation.closed) return;
    terminateInferencePort(operation, inferenceCodeFromRejection(error, operation.abort.signal.aborted));
  } finally {
    operation.inFlight = false;
  }
}

/** 🛑️ Requests cancellation. The phase does NOT move optimistically: only the server's own answer
 * may report `cancelled`, so a hub that refuses the cancel can never be misreported as honoured. */
async function cancelInferenceJob(operationEpoch: number): Promise<void> {
  const operation = liveInferencePort(operationEpoch);
  if (operation === null) return;
  advanceInferencePort(operation, { kind: "cancel" });
  if (operation.status.jobId === null || operation.inFlight || operation.cancelSent) return;
  operation.cancelSent = true;
  operation.inFlight = true;
  try {
    const response = await inferenceBrokerFetch(operation, `/jobs/${operation.status.jobId}/cancel`, { method: "POST" });
    if (!response.ok) throw new DirectoryHttpError(response.status, "");
    const page = parseGisMapInferenceEventPageV1(await readInferenceJson(response));
    if (liveInferencePort(operationEpoch) !== operation) return;
    if (page.proposalState === "approved") {
      terminateInferencePort(operation, "inference.transport");
      scheduleInferencePoll(operation);
      return;
    }
    advanceInferencePort(operation, { kind: "page", page });
    if (gisMapInferencePortTerminalV1(operation.status.phase)) retireInferencePort(operation);
    else scheduleInferencePoll(operation);
  } catch (error) {
    if (operation.closed) return;
    terminateInferencePort(operation, inferenceCodeFromRejection(error, operation.abort.signal.aborted));
  } finally {
    operation.inFlight = false;
  }
}

/** ✅️ Approves exactly the offered proposal, echoing back the server's own hash. It is never
 * retried: a duplicate approval could otherwise ask for a second Map commit. */
async function approveInferenceProposal(operationEpoch: number): Promise<void> {
  const operation = liveInferencePort(operationEpoch);
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
    body = JSON.stringify(sealGisMapInferenceApprovalRequestV1(operation.status.jobId, operation.status.proposalHash));
  } catch {
    terminateInferencePort(operation, "inference.invalid");
    return;
  }
  const jobId = operation.status.jobId;
  advanceInferencePort(operation, { kind: "approve" });
  operation.inFlight = true;
  try {
    const response = await inferenceBrokerFetch(operation, `/jobs/${jobId}/approval`, { method: "POST", body });
    if (!response.ok) throw new DirectoryHttpError(response.status, "");
    const receipt = parseGisMapInferenceApprovalReceiptV1(await readInferenceJson(response));
    if (liveInferencePort(operationEpoch) !== operation) return;
    retainInferenceApprovalUndo(operation, receipt);
    advanceInferencePort(operation, { kind: "approval", receipt });
    retireInferencePort(operation);
  } catch (error) {
    if (operation.closed) return;
    terminateInferencePort(operation, inferenceCodeFromRejection(error, operation.abort.signal.aborted));
  } finally {
    operation.inFlight = false;
  }
}

/** ↩️ Routes the ordinary Shell history action through the one exact mounted durable approval
 * owner. The retry key and Hub handle stay stable and private across an indeterminate response. */
async function undoInferenceApproval(historyEpoch: number, clientInstanceId: string, scope: DocumentScope): Promise<void> {
  const owner = inferenceApprovalUndoOwner;
  if (owner === null || owner.historyEpoch !== historyEpoch || owner.clientInstanceId !== clientInstanceId || owner.scope.spaceId !== scope.spaceId || owner.scope.documentId !== scope.documentId || (owner.phase !== "available" && !(owner.phase === "failed" && owner.retryable))) return;
  if (!sameBrowserSessionOperationFence(owner.sessionFence)) {
    retireInferenceApprovalUndoForAuthority(owner);
    return;
  }
  const state = artifactState(scope.documentId, scope.spaceId);
  if (state === undefined || !sameApprovalUndoMountV1(state, owner)) {
    retireInferenceApprovalUndo(owner);
    return;
  }
  owner.phase = "submitting";
  approvalUndoStatus(owner, { phase: "submitting", canUndo: false, code: null });
  try {
    const request = sealGisMapApprovalUndoRequestV1(owner.receipt.undo, owner.idempotencyKey);
    const response = await inferenceApprovalUndoBrokerFetch(owner, JSON.stringify(request));
    if (!response.ok) throw new DirectoryHttpError(response.status, "");
    const receipt: GisMapApprovalUndoReceiptV1 = parseGisMapApprovalUndoReceiptV1(await readInferenceJson(response));
    if (inferenceApprovalUndoOwner !== owner || !sameBrowserSessionOperationFence(owner.sessionFence)) return;
    const currentState = artifactState(owner.scope.documentId, owner.scope.spaceId);
    if (currentState === undefined || !sameApprovalUndoMountV1(currentState, owner)) {
      retireInferenceApprovalUndo(owner);
      return;
    }
    if (!receipt.applied || receipt.targetId !== owner.receipt.undo.targetId || receipt.originalJobId !== owner.receipt.jobId || receipt.frontier.documentId !== owner.scope.documentId) throw new Error("gis map approval undo: receipt mismatch");
    approvalUndoStatus(owner, { phase: "applied", canUndo: false, code: null });
    inferenceApprovalUndoOwner = null;
    owner.abort.abort(new Error("gis map approval undo applied"));
  } catch (error) {
    if (inferenceApprovalUndoOwner !== owner || owner.abort.signal.aborted) return;
    const status = error instanceof DirectoryHttpError ? error.status : undefined;
    const code = status === undefined ? "inference.transport" : gisMapInferenceCodeFromStatusV1(status);
    const sameSession = sameBrowserSessionOperationFence(owner.sessionFence);
    owner.phase = "failed";
    owner.retryable = sameSession && (status === undefined || status === 429 || status === 503);
    approvalUndoStatus(owner, { phase: "failed", canUndo: owner.retryable, code });
    if (!owner.retryable && sameSession) {
      inferenceApprovalUndoOwner = null;
      owner.abort.abort(new Error("gis map approval undo rejected"));
    }
  }
}

/** 🏁️ Releases a port that already reported its terminal, without publishing a second one. */
function retireInferencePort(operation: InferenceOperationV1): void {
  if (operation.closed) return;
  if (operation.pollTimer !== null) clearTimeout(operation.pollTimer);
  operation.pollTimer = null;
  if (!operation.closeRequested && operation.request !== null && gisMapInferencePortTerminalV1(operation.status.phase)) return;
  operation.closed = true;
  operation.abort.abort(new Error("gis map inference port retired"));
  if (inferencePort === operation) inferencePort = null;
  post({ kind: "inference-port-closed", operationEpoch: operation.operationEpoch, scope: operation.scope });
}

/** 🛑️ Closing records cancellation without aborting a submit whose Hub receipt may still arrive. */
function closeInferencePort(operationEpoch: number): void {
  const operation = inferencePort;
  if (operation === null || operation.operationEpoch !== operationEpoch) return;
  operation.closeRequested = true;
  if (operation.status.phase === "idle" || gisMapInferencePortTerminalV1(operation.status.phase)) {
    retireInferencePort(operation);
    return;
  }
  if (operation.status.phase === "indeterminate") {
    advanceInferencePort(operation, { kind: "cancel" });
    void driveInferencePort(operationEpoch);
    return;
  }
  void cancelInferenceJob(operationEpoch);
}
//#endregion 💡️Inference

  return {
    owner: "gis", serviceId: "s.gis.gismap.inference",
    dispatch(operation: InstalledServiceOperationV1) {
      if (operation.payload === null || typeof operation.payload !== "object" || Array.isArray(operation.payload)) throw new Error("gis inference.invalid-operation");
      const payload = operation.payload as Record<string, unknown>;
      const fields = operation.action === "open" ? ["scope"] : operation.action === "propose" ? (Object.hasOwn(payload, "requestId") ? ["requestId"] : []) : operation.action === "undo" ? ["historyEpoch", "clientInstanceId", "scope"] : [];
      if (Object.keys(payload).length !== fields.length || fields.some((field) => !Object.hasOwn(payload, field))) throw new Error("gis inference.invalid-operation");
      if (operation.action === "propose" && Object.hasOwn(payload, "requestId") && (typeof payload.requestId !== "string" || !/^[0-9a-f]{32}$/u.test(payload.requestId))) throw new Error("gis inference.invalid-operation");
      if (operation.action === "open" || operation.action === "undo") {
        const scope = payload.scope as DocumentScope;
        if (scope === null || typeof scope !== "object" || Object.keys(scope).sort().join(",") !== "documentId,spaceId" || [scope.spaceId, scope.documentId].some((value) => typeof value !== "string" || value.length === 0 || new TextEncoder().encode(value).length > 256 || /\p{Cc}/u.test(value))) throw new Error("gis inference.invalid-operation");
      }
      if (operation.action === "undo" && (!Number.isSafeInteger(payload.historyEpoch) || Number(payload.historyEpoch) < 1 || typeof payload.clientInstanceId !== "string" || !/^[0-9a-f-]{36}$/u.test(payload.clientInstanceId))) throw new Error("gis inference.invalid-operation");
      switch (operation.action) {
        case "open": openInferencePort(operation.operationEpoch, payload.scope as DocumentScope); break;
        case "propose": void submitInferenceJob(operation.operationEpoch, (payload.requestId as string | undefined) ?? crypto.randomUUID().replaceAll("-", "")); break;
        case "poll": void pollInferenceJob(operation.operationEpoch); break;
        case "cancel": void cancelInferenceJob(operation.operationEpoch); break;
        case "approve": void approveInferenceProposal(operation.operationEpoch); break;
        case "close": closeInferencePort(operation.operationEpoch); break;
        case "undo": void undoInferenceApproval(payload.historyEpoch as number, payload.clientInstanceId as string, payload.scope as DocumentScope); break;
        default: throw new Error("gis map inference: undeclared action");
      }
    },
    retire() {
      if (inferencePort !== null) { inferencePort.closeRequested = true; inferencePort.abort.abort(new Error("gis owner contribution retired")); if (inferencePort.pollTimer !== null) clearTimeout(inferencePort.pollTimer); inferencePort.closed = true; post({ kind: "inference-port-closed", operationEpoch: inferencePort.operationEpoch, scope: inferencePort.scope }); inferencePort = null; }
      if (inferenceApprovalUndoOwner !== null) retireInferenceApprovalUndo(inferenceApprovalUndoOwner);
    },
    sessionRetired() { if (inferencePort !== null) { inferencePort.closeRequested = true; terminateInferencePort(inferencePort, "inference.transport"); } if (inferenceApprovalUndoOwner !== null) retireInferenceApprovalUndoForAuthority(inferenceApprovalUndoOwner); },
    documentClosed(runtimeKey: string) { if (inferencePort !== null && documentRuntimeKeyV1({ kind: "hub", dataClass: "persistedShared", ...inferencePort.scope }) === runtimeKey) closeInferencePort(inferencePort.operationEpoch); if (inferenceApprovalUndoOwner !== null && documentRuntimeKeyV1({ kind: "hub", dataClass: "persistedShared", ...inferenceApprovalUndoOwner.scope }) === runtimeKey) retireInferenceApprovalUndo(inferenceApprovalUndoOwner); },
    documentRebootstrapped({ state }: {state: ArtifactState}) { reissueInferenceApprovalUndoForRebootstrap(state); if (inferencePort !== null && documentRuntimeKeyV1({ kind: "hub", dataClass: "persistedShared", ...inferencePort.scope }) === state.runtimeKey && inferencePort.status.phase !== "approving") closeInferencePort(inferencePort.operationEpoch); },
    documentMounted({ state, reservation, pair }: {state: ArtifactState;reservation?: DocumentBrowserActorReservation;pair?: VerifiedColdDocumentPair}) { if (reservation && pair) bindInferenceApprovalUndoToMountedPair(state, reservation, pair); },
    test: { get inferencePortStatusObserver() { return inferencePortStatusObserver; }, set inferencePortStatusObserver(value: typeof inferencePortStatusObserver) { inferencePortStatusObserver = value; }, get inferencePort() { return inferencePort; }, set inferencePort(value: typeof inferencePort) { inferencePort = value; }, get inferenceApprovalUndoOwner() { return inferenceApprovalUndoOwner; }, set inferenceApprovalUndoOwner(value: typeof inferenceApprovalUndoOwner) { inferenceApprovalUndoOwner = value; }, get inferenceApprovalUndoEpoch() { return inferenceApprovalUndoEpoch; }, set inferenceApprovalUndoEpoch(value: number) { inferenceApprovalUndoEpoch = value; }, openInferencePort, driveInferencePort, retainInferenceApprovalUndo, undoInferenceApproval, reissueInferenceApprovalUndoForRebootstrap, bindInferenceApprovalUndoToMountedPair },
  };
}
