/** ↩️ The retained approval undo owner: minting, retirement, rebootstrap re-issue and binding to the mounted cold pair.
 *
 * 🧱️ Child module of {@link ../🟦️.ts | createGisMapInferenceWorkerV1}: the functions take the worker context `ctx` explicitly instead of closing over the factory. */
import { type GisMapApprovalHistoryStatusV1, type GisMapInferenceApprovalReceiptV1 } from "../../🧬️schema/🟦️.ts";
import type { ArtifactState, DocumentBrowserActorReservation, GisInferenceContext, InferenceApprovalUndoOwnerV1, InferenceOperationV1, VerifiedColdDocumentPair } from "../🧰️worker-context-and-types/🟦️.ts";

export function mintInferenceApprovalUndoIdempotencyKeyV1(): string {
  const bytes = crypto.getRandomValues(new Uint8Array(16));
  bytes[0] = (bytes[0] ?? 0) | 1;
  return Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");
}

export function approvalUndoStatus(ctx: GisInferenceContext, owner: InferenceApprovalUndoOwnerV1, status: GisMapApprovalHistoryStatusV1): void {
  ctx.post({ owner: "gis", serviceId: "s.gis.gismap.inference", kind: "inference-history-status", historyEpoch: owner.historyEpoch, clientInstanceId: owner.clientInstanceId, scope: owner.scope, status });
}

export function retireInferenceApprovalUndoForAuthority(ctx: GisInferenceContext, owner: InferenceApprovalUndoOwnerV1): void {
  if (owner.phase !== "submitting" && owner.phase !== "failed") {
    retireInferenceApprovalUndo(ctx, owner);
    return;
  }
  if (owner.phase === "failed" && !owner.retryable) return;
  owner.abort.abort(new Error("gis map approval undo authority retired"));
  owner.phase = "failed";
  owner.retryable = false;
  approvalUndoStatus(ctx, owner, { phase: "failed", canUndo: false, code: "inference.transport" });
}

export function sameApprovalUndoFrontierV1(ctx: GisInferenceContext, owner: InferenceApprovalUndoOwnerV1, pair: VerifiedColdDocumentPair): boolean {
  const { bytesHex } = ctx.host;
  const expected = owner.receipt.undo.expectedCurrent;
  return pair.frontier.documentId === expected.documentId
    && pair.frontier.headEditOrdinal === BigInt(expected.headEditOrdinal)
    && pair.frontier.headEditId === expected.headEditId
    && pair.frontier.lastCommitSeq === BigInt(expected.lastCommitSeq)
    && bytesHex(pair.frontier.chainSha256) === expected.chainSha256;
}

export function sameApprovalUndoMountV1(ctx: GisInferenceContext, state: ArtifactState, owner: InferenceApprovalUndoOwnerV1): boolean {
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
    && sameApprovalUndoFrontierV1(ctx, owner, pair);
}

export function retireInferenceApprovalUndo(ctx: GisInferenceContext, owner: InferenceApprovalUndoOwnerV1, notify = true): void {
  if (ctx.state.inferenceApprovalUndoOwner !== owner) return;
  ctx.state.inferenceApprovalUndoOwner = null;
  owner.abort.abort(new Error("gis map approval undo owner retired"));
  if (notify) approvalUndoStatus(ctx, owner, { phase: "unavailable", canUndo: false, code: null });
}

export function reissueInferenceApprovalUndoForRebootstrap(ctx: GisInferenceContext, state: ArtifactState): void {
  const { artifactScope, sameBrowserSessionOperationFence } = ctx.host;
  const owner = ctx.state.inferenceApprovalUndoOwner;
  if (owner === null || owner.scope.spaceId !== artifactScope(state)?.spaceId || owner.scope.documentId !== state.config.documentId || owner.clientInstanceId !== state.openClientInstanceId) return;
  if (!sameBrowserSessionOperationFence(owner.sessionFence)) {
    retireInferenceApprovalUndoForAuthority(ctx, owner);
    return;
  }
  ctx.state.inferenceApprovalUndoOwner = null;
  owner.abort.abort(new Error("gis map approval undo owner rebootstrap"));
  approvalUndoStatus(ctx, owner, { phase: "unavailable", canUndo: false, code: null });
  if (ctx.state.inferenceApprovalUndoEpoch >= Number.MAX_SAFE_INTEGER) return;
  ctx.state.inferenceApprovalUndoOwner = {
    historyEpoch: ++ctx.state.inferenceApprovalUndoEpoch,
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

export function bindInferenceApprovalUndoToMountedPair(ctx: GisInferenceContext, state: ArtifactState, reservation: DocumentBrowserActorReservation, pair: VerifiedColdDocumentPair): void {
  const { sameBrowserSessionOperationFence, artifactScope } = ctx.host;
  const owner = ctx.state.inferenceApprovalUndoOwner;
  const lease = state.executionTargetLease;
  if (owner === null || owner.phase !== "awaiting-mount" || lease === null || state.browserActorReservation !== reservation || state.verifiedColdPair !== pair) return;
  if (!sameBrowserSessionOperationFence(owner.sessionFence)) {
    retireInferenceApprovalUndo(ctx, owner);
    return;
  }
  const scope = artifactScope(state);
  if (!scope || scope.spaceId !== owner.scope.spaceId || scope.documentId !== owner.scope.documentId || state.openClientInstanceId !== owner.clientInstanceId) return;
  if (!sameApprovalUndoFrontierV1(ctx, owner, pair)) {
    retireInferenceApprovalUndo(ctx, owner);
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
    retireInferenceApprovalUndo(ctx, owner);
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
  approvalUndoStatus(ctx, owner, { phase: "available", canUndo: true, code: null });
}

export function retainInferenceApprovalUndo(ctx: GisInferenceContext, operation: InferenceOperationV1, receipt: GisMapInferenceApprovalReceiptV1): void {
  const { artifactState, sameBrowserSessionOperationFence } = ctx.host;
  const state = artifactState(operation.scope.documentId, operation.scope.spaceId);
  if (state === undefined || state.closed || state.openClientInstanceId !== operation.clientInstanceId || operation.sessionEpoch !== ctx.host.directorySessionEpoch || !sameBrowserSessionOperationFence(operation.sessionFence)) return;
  const fields = operation.leaseFields;
  if (!receipt.applied || receipt.undo.expectedCurrent.documentId !== operation.scope.documentId || operation.clientInstanceId.length === 0 || fields.browserActor.kind !== "closed-browser-actor") throw new Error("gis map approval undo: invalid source owner");
  if (ctx.state.inferenceApprovalUndoOwner !== null) {
    if (ctx.state.inferenceApprovalUndoOwner.phase === "failed" && !ctx.state.inferenceApprovalUndoOwner.retryable) throw new Error("gis map approval undo: capacity");
    retireInferenceApprovalUndo(ctx, ctx.state.inferenceApprovalUndoOwner);
  }
  if (ctx.state.inferenceApprovalUndoEpoch >= Number.MAX_SAFE_INTEGER) throw new Error("gis map approval undo: epoch exhausted");
  const owner: InferenceApprovalUndoOwnerV1 = {
    historyEpoch: ++ctx.state.inferenceApprovalUndoEpoch,
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
  ctx.state.inferenceApprovalUndoOwner = owner;
  state?.browserActorReservation?.bindApprovalUndoIfMounted();
}
