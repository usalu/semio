/** 🧪️ Provides in-memory endpoints for the actual staged host adapter without opening network connections. */
import type { WireFrontierSummary } from "@semio-tech/framework-replication";

export type ArtifactState = {
  config: { documentId: string };
  actor: string;
  hubActorReady: boolean;
  closed: boolean;
  executionTargetLease: { fields: () => { grant: { write: boolean } } };
  runtimeKey: string;
  openClientInstanceId: string;
  socket: { readyState: number; sent: Uint8Array[]; send: (bytes: Uint8Array) => void; close: () => void } | null;
  nextBatchId: number;
  pendingSocketActorId: string | null;
  pendingBatches: Map<number, unknown>;
  pendingDocumentBackboneBytes: number;
  pendingDocumentBackboneMessages: number;
  frontier: WireFrontierSummary | null;
  requiredTailFrontier: WireFrontierSummary | null;
  artifactBootstrap: unknown;
  artifactRebootstrapRequired: boolean;
  received: Uint8Array[];
  events: unknown[];
  recoveries: number;
  browserActorReservation: { receiveBackbone: (message: Uint8Array) => Promise<void> } | null;
};

export const artifacts = new Map<string, ArtifactState>();
export function emitEvent(state: ArtifactState, event: unknown) { state.events.push(event); }
export function documentAwaitsBrowserActor(_state: ArtifactState): boolean { return false; }
export function retainBackboneBeforeBrowserActor(state: ArtifactState, message: Uint8Array) { state.received.push(message); }
export function finishCatchupIfReady(_state: ArtifactState) {}
export function requestDocumentActorRecoveryV1(state: ArtifactState, _reason: string) { state.recoveries++; }
export function hubCommandRejectionV1(message: unknown): unknown { return message; }
export function hubTransientApplyRefusalV1(_messages: ArrayLike<number>): boolean { return false; }
export function closeHubSocketV1(socket: WebSocket, _reason: string) { socket.close(); }
