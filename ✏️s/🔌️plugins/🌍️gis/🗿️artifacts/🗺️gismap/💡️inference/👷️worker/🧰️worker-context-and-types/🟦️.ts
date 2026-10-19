/** 🧰️ The GIS inference worker context: the host port, the `post` sink, the mutable port and undo owner state and the shared operation types.
 *
 * 🧱️ Child module of {@link ../🟦️.ts | createGisMapInferenceWorkerV1}: the functions take the worker context `ctx` explicitly instead of closing over the factory. */
import { type GisMapInferenceApprovalReceiptV1, type GisMapInferenceJobRequestV1, type GisMapInferencePortStatusV1 } from "../../🧬️schema/🟦️.ts";
import type { DocumentScope } from "@semio-tech/framework/document/scope";
import { type DocumentServiceLeaseFieldsV1, type DocumentServiceWorkerHostV1, type DocumentServiceWorkerMessageV1 } from "@semio-tech/framework/inference/service/worker-host";

export type ArtifactState = NonNullable<ReturnType<DocumentServiceWorkerHostV1["artifactState"]>>;

export type VerifiedColdDocumentPair = NonNullable<ArtifactState["verifiedColdPair"]>;

export type DocumentBrowserActorReservation = NonNullable<ArtifactState["browserActorReservation"]>;

export type BrowserSessionOperationFenceV1 = NonNullable<ReturnType<DocumentServiceWorkerHostV1["captureBrowserSessionOperationFence"]>>;

export type InferencePortStatusMessageV1 = Readonly<{ kind: "inference-port-status"; operationEpoch: number; scope: DocumentScope; status: Readonly<{ owner: string; serviceId: string; payload: unknown }> }>;

/** 💡️ One private request owner and bounded poll timer. A verified writable lease admits it;
 * cancel/reconcile cleanup outlives document retirement. Unknown outcomes retain the original
 * request without resubmission. Only the Hub's approved command mutates the document. */
export const INFERENCE_PORT_CAPACITY = 1;

/** ⏱️ Bounded, jitter-free poll cadence for one running job — a `setTimeout` chain, never an
 * interval and never a busy loop. */
export const INFERENCE_POLL_INTERVAL_MS = 750;

/** 🔁️ Highest number of poll turns one job may take before it is reported indeterminate. */
export const INFERENCE_MAX_POLL_TURNS = 240;

/** ⏳️ Lifetime one submitted job asks the hub for. */
export const INFERENCE_JOB_LIFETIME_MS = 60_000;

export type InferenceOperationV1 = {
  readonly operationEpoch: number;
  readonly scope: DocumentScope;
  readonly abort: AbortController;
  readonly sessionEpoch: number;
  readonly sessionFence: BrowserSessionOperationFenceV1;
  readonly clientInstanceId: string;
  readonly leaseFields: DocumentServiceLeaseFieldsV1;
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

export type InferenceApprovalUndoMountV1 = Readonly<{
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

export type InferenceApprovalUndoOwnerV1 = {
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

/** 🗃️ The mutable inference state one worker installation owns: the retained port, the approval undo owner and its epoch. */
export interface GisInferenceState {
  inferencePort: InferenceOperationV1 | null;
  inferenceApprovalUndoEpoch: number;
  inferenceApprovalUndoOwner: InferenceApprovalUndoOwnerV1 | null;
  /** 🔭️ Observation seam mirroring {@link executionTargetStatusObserver}: a harness without a worker
   * scope still sees the exact bounded payload the renderer would receive. */
  inferencePortStatusObserver: ((status: InferencePortStatusMessageV1) => void) | null;
}

/** 🧭️ What every inference function reads: the framework host, the `post` sink and the mutable state. */
export interface GisInferenceContext {
  readonly host: DocumentServiceWorkerHostV1;
  readonly post: (message: DocumentServiceWorkerMessageV1) => ReturnType<DocumentServiceWorkerHostV1["post"]>;
  readonly state: GisInferenceState;
}

/** 🪝️ The parameter list of a context function without its leading context, for the bound test seam. */
export type DropGisInferenceContext<A extends readonly unknown[]> = A extends readonly [GisInferenceContext, ...infer R] ? R : never;

/** 🏭️ Builds the context of one worker installation over the framework host port. */
export function createGisInferenceContext(host: DocumentServiceWorkerHostV1): GisInferenceContext {
  return {
    host,
    post: (message) => host.post({ ...message, owner: "gis", serviceId: "s.gis.gismap.inference" }),
    state: { inferencePort: null, inferenceApprovalUndoEpoch: 0, inferenceApprovalUndoOwner: null, inferencePortStatusObserver: null },
  };
}
