/** 🚇️ Relays one sealed atomic edit between the same authenticated actor's local document sessions. */
import { AtomicDocumentAssembly } from "ATOMIC_MODULE";
import { decodeAtomicDocumentBackboneBatch } from "CAUSAL_MODULE";
import { encodeBackboneMessage, type DocumentBackboneTransportMessage } from "BACKBONE_MODULE";
import type { ArtifactState } from "../🧾️document-state/🟦️.ts";
import { artifacts } from "../🧾️document-state/🟦️.ts";
import { DOCUMENT_BACKBONE_RETENTION_LIMITS } from "@semio-tech/framework-replication";
import { PENDING_MUTATIONS_QUEUE_LIMIT } from "../⚙️constants/🟦️.ts";

type Row = ReturnType<typeof decodeAtomicDocumentBackboneBatch>[number];
type Assembly = AtomicDocumentAssembly<Row>;
type Part = Extract<DocumentBackboneTransportMessage, { kind: "atomic-part" }>;
type LocalSlot = { assembly: Assembly; bytes: number; messages: number };
const outgoing = new WeakMap<ArtifactState, LocalSlot>();
const incoming = new WeakMap<ArtifactState, LocalSlot>();

/** 🧷️ Retains exactly one private group per direction of the original local actor session. */
export async function localAtomicDocumentFrames(state: ArtifactState, part: Part, direction: "outgoing" | "incoming"): Promise<Readonly<{
  frames: readonly Uint8Array[]; envelopes: readonly Row[]; bytes: number; messages: number;
}> | undefined> {
  const scope = { documentId: state.config.documentId, actor: state.config.actor, generation: state.openClientInstanceId };
  if (artifacts.get(state.runtimeKey) !== state) throw new Error("atomic backbone: retired document actor");
  const slots = direction === "outgoing" ? outgoing : incoming;
  let slot = slots.get(state);
  if (slot === undefined) {
    slot = { assembly: new AtomicDocumentAssembly(scope, decodeAtomicDocumentBackboneBatch), bytes: 0, messages: 0 };
    slots.set(state, slot);
  }
  if (direction === "outgoing") {
    if (state.pendingDocumentBackboneMessages + 1 > DOCUMENT_BACKBONE_RETENTION_LIMITS.maximumMessages ||
        state.pendingDocumentBackboneBytes + part.message.length > DOCUMENT_BACKBONE_RETENTION_LIMITS.maximumBytes) {
      throw new Error("atomic backbone: local outbox capacity");
    }
    slot.bytes += part.message.length;
    slot.messages += 1;
    state.pendingDocumentBackboneBytes += part.message.length;
    state.pendingDocumentBackboneMessages += 1;
  }
  const sealed = await slot.assembly.admit(scope, part.part);
  if (artifacts.get(state.runtimeKey) !== state) throw new Error("atomic backbone: retired document actor");
  if (!sealed) return undefined;
  if (direction === "outgoing" && state.pendingMutations.length + sealed.envelopes.length > PENDING_MUTATIONS_QUEUE_LIMIT) {
    throw new Error("atomic backbone: local mutation queue capacity");
  }
  const result = {
    frames: sealed.parts.map((envelopes) => encodeBackboneMessage({ kind: "mutations", envelopes })),
    envelopes: sealed.envelopes,
    bytes: slot.bytes,
    messages: slot.messages,
  };
  slot.bytes = 0;
  slot.messages = 0;
  return result;
}

/** 🧹️ Cancels live digest work and removes both original ingress/egress slots. */
export function retireLocalAtomicDocumentFrames(state: ArtifactState): void {
  for (const slots of [outgoing, incoming]) {
    const slot = slots.get(state);
    if (slot) {
      slot.assembly.retire();
      state.pendingDocumentBackboneBytes -= slot.bytes;
      state.pendingDocumentBackboneMessages -= slot.messages;
    }
    slots.delete(state);
  }
}
