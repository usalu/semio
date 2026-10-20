import type { DocumentScope } from "@semio-tech/framework/document/scope";
import { type DocumentServiceWorkerHostV1 } from "@semio-tech/framework/inference/service/worker-host";
import type { InstalledServiceOperationV1 } from "@semio-tech/framework/inference/service";
import { type DropGisInferenceContext, createGisInferenceContext } from "./🧰️worker-context-and-types/🟦️.ts";
import type { ArtifactState, DocumentBrowserActorReservation, VerifiedColdDocumentPair } from "./🧰️worker-context-and-types/🟦️.ts";
import { bindInferenceApprovalUndoToMountedPair, reissueInferenceApprovalUndoForRebootstrap, retainInferenceApprovalUndo, retireInferenceApprovalUndo, retireInferenceApprovalUndoForAuthority } from "./↩️approval-undo-owner/🟦️.ts";
import { cancelInferenceJob, driveInferencePort, pollInferenceJob, terminateInferencePort } from "./🔁️port-job-turns/🟦️.ts";
import { approveInferenceProposal, closeInferencePort, openInferencePort, submitInferenceJob, undoInferenceApproval } from "./🎛️port-lifecycle-and-decisions/🟦️.ts";

/** 🌍 Installs GIS lifecycle behavior over the framework document/session authority port. */
export function createGisMapInferenceWorkerV1(host: DocumentServiceWorkerHostV1) {
  const ctx = createGisInferenceContext(host);
  const { documentRuntimeKeyV1 } = host;
  //#region 💡️Inference
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
        case "open": openInferencePort(ctx, operation.operationEpoch, payload.scope as DocumentScope); break;
        case "propose": void submitInferenceJob(ctx, operation.operationEpoch, (payload.requestId as string | undefined) ?? crypto.randomUUID().replaceAll("-", "")); break;
        case "poll": void pollInferenceJob(ctx, operation.operationEpoch); break;
        case "cancel": void cancelInferenceJob(ctx, operation.operationEpoch); break;
        case "approve": void approveInferenceProposal(ctx, operation.operationEpoch); break;
        case "close": closeInferencePort(ctx, operation.operationEpoch); break;
        case "undo": void undoInferenceApproval(ctx, payload.historyEpoch as number, payload.clientInstanceId as string, payload.scope as DocumentScope); break;
        default: throw new Error("gis map inference: undeclared action");
      }
    },
    retire() {
      if (ctx.state.inferencePort !== null) { ctx.state.inferencePort.closeRequested = true; ctx.state.inferencePort.abort.abort(new Error("gis owner contribution retired")); if (ctx.state.inferencePort.pollTimer !== null) clearTimeout(ctx.state.inferencePort.pollTimer); ctx.state.inferencePort.closed = true; ctx.post({ kind: "inference-port-closed", operationEpoch: ctx.state.inferencePort.operationEpoch, scope: ctx.state.inferencePort.scope }); ctx.state.inferencePort = null; }
      if (ctx.state.inferenceApprovalUndoOwner !== null) retireInferenceApprovalUndo(ctx, ctx.state.inferenceApprovalUndoOwner);
    },
    sessionRetired() { if (ctx.state.inferencePort !== null) { ctx.state.inferencePort.closeRequested = true; terminateInferencePort(ctx, ctx.state.inferencePort, "inference.transport"); } if (ctx.state.inferenceApprovalUndoOwner !== null) retireInferenceApprovalUndoForAuthority(ctx, ctx.state.inferenceApprovalUndoOwner); },
    documentClosed(runtimeKey: string) { if (ctx.state.inferencePort !== null && documentRuntimeKeyV1({ kind: "hub", dataClass: "persistedShared", ...ctx.state.inferencePort.scope }) === runtimeKey) closeInferencePort(ctx, ctx.state.inferencePort.operationEpoch); if (ctx.state.inferenceApprovalUndoOwner !== null && documentRuntimeKeyV1({ kind: "hub", dataClass: "persistedShared", ...ctx.state.inferenceApprovalUndoOwner.scope }) === runtimeKey) retireInferenceApprovalUndo(ctx, ctx.state.inferenceApprovalUndoOwner); },
    documentRebootstrapped({ state }: {state: ArtifactState}) { reissueInferenceApprovalUndoForRebootstrap(ctx, state); if (ctx.state.inferencePort !== null && documentRuntimeKeyV1({ kind: "hub", dataClass: "persistedShared", ...ctx.state.inferencePort.scope }) === state.runtimeKey && ctx.state.inferencePort.status.phase !== "approving") closeInferencePort(ctx, ctx.state.inferencePort.operationEpoch); },
    documentMounted({ state, reservation, pair }: {state: ArtifactState;reservation?: DocumentBrowserActorReservation;pair?: VerifiedColdDocumentPair}) { if (reservation && pair) bindInferenceApprovalUndoToMountedPair(ctx, state, reservation, pair); },
    test: { get inferencePortStatusObserver() { return ctx.state.inferencePortStatusObserver; }, set inferencePortStatusObserver(value: typeof ctx.state.inferencePortStatusObserver) { ctx.state.inferencePortStatusObserver = value; }, get inferencePort() { return ctx.state.inferencePort; }, set inferencePort(value: typeof ctx.state.inferencePort) { ctx.state.inferencePort = value; }, get inferenceApprovalUndoOwner() { return ctx.state.inferenceApprovalUndoOwner; }, set inferenceApprovalUndoOwner(value: typeof ctx.state.inferenceApprovalUndoOwner) { ctx.state.inferenceApprovalUndoOwner = value; }, get inferenceApprovalUndoEpoch() { return ctx.state.inferenceApprovalUndoEpoch; }, set inferenceApprovalUndoEpoch(value: number) { ctx.state.inferenceApprovalUndoEpoch = value; }, openInferencePort: (...args: DropGisInferenceContext<Parameters<typeof openInferencePort>>) => openInferencePort(ctx, ...args), driveInferencePort: (...args: DropGisInferenceContext<Parameters<typeof driveInferencePort>>) => driveInferencePort(ctx, ...args), retainInferenceApprovalUndo: (...args: DropGisInferenceContext<Parameters<typeof retainInferenceApprovalUndo>>) => retainInferenceApprovalUndo(ctx, ...args), undoInferenceApproval: (...args: DropGisInferenceContext<Parameters<typeof undoInferenceApproval>>) => undoInferenceApproval(ctx, ...args), reissueInferenceApprovalUndoForRebootstrap: (...args: DropGisInferenceContext<Parameters<typeof reissueInferenceApprovalUndoForRebootstrap>>) => reissueInferenceApprovalUndoForRebootstrap(ctx, ...args), bindInferenceApprovalUndoToMountedPair: (...args: DropGisInferenceContext<Parameters<typeof bindInferenceApprovalUndoToMountedPair>>) => bindInferenceApprovalUndoToMountedPair(ctx, ...args) },
  };
  //#endregion 💡️Inference
}
