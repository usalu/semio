/** 🧪 Independently authored causal fragments enter one actor edit only through the exact hash and scope. */
import { AtomicDocumentAssembly, atomicBackbonePart } from "./🗑️generated/raw-spr-transport/atomic.ts";
import { decodeAtomicDocumentBackboneBatch, decodeDocumentBackboneEnvelopeBatchExact } from "./🗑️generated/raw-spr-transport/causal.ts";
import { encodeBackboneMessage, parseDocumentBackboneTransportMessage } from "./🗑️generated/raw-spr-transport/message.ts";

function check(state: unknown, message: string): asserts state {
  if (!state) throw new Error(message);
}

async function denied(operation: () => unknown, part: string): Promise<void> {
  try { await operation(); } catch (error) {
    check(String(error).includes(part), String(error));
    return;
  }
  throw new Error("admitted " + part);
}

const directory = new URL("./🗑️generated/raw-spr-transport/", import.meta.url);
const vector = await Bun.file(new URL("vector.json", directory)).json();
const frames = await Promise.all(Array.from({ length: vector.parts }, async (_, index) => {
  const bytes = await Bun.file(new URL(index + ".part", directory)).bytes();
  const message = encodeBackboneMessage({ kind: "mutations", envelopes: bytes });
  const parsed = parseDocumentBackboneTransportMessage(message);
  check(parsed.kind === "atomic-part", "opaque exact transport branch");
  return parsed.part;
}));
const ordinary = parseDocumentBackboneTransportMessage(encodeBackboneMessage({ kind: "mutations", envelopes: Uint8Array.of(0) }));
check(ordinary.kind === "batch" && ordinary.envelopes.length === 0, "ordinary transport branch");
const scope = { documentId: "fixture-document", actor: "fixture-actor", generation: "reservation-73" };
const pending = new AtomicDocumentAssembly(scope, decodeAtomicDocumentBackboneBatch);
check(frames.length > 1, "multi-frame case");
await denied(() => pending.admit({ ...scope, generation: "previous-reservation" }, frames[0]!), "stale binding");
const reverse = frames.slice().reverse();
for (const frame of reverse.slice(0, -1)) check(await pending.admit(scope, frame) === undefined, "partial publication");
await denied(() => pending.admit(scope, reverse[0]!), "duplicate part");
const group = await pending.admit(scope, reverse.at(-1)!);
check(group?.editId === "edit-png-82" && group.envelopes.length === 15, "exact edit roster");
check(group.parts.length === vector.parts && atomicBackbonePart(group.parts[0]!)?.extent === vector.extent, "exact physical group");
const png = new Uint8Array(vector.source);
let offset = 0;
for (const row of group.envelopes) {
  check(row.timestamp.physical_ms === 0xffff_ffff_ffff_ffffn, "exact timestamp");
  png.set(row.diff.payload, offset);
  offset += row.diff.payload.length;
}
const digest = new Uint8Array(await crypto.subtle.digest("SHA-256", png));
check(offset === vector.source && Array.from(digest, (byte) => byte.toString(16).padStart(2, "0")).join("") === vector.sha, "independent source digest");
await denied(() => decodeDocumentBackboneEnvelopeBatchExact(new Uint8Array(vector.extent)), "batch-bytes");

const wrongActor = { ...scope, actor: "wrong-actor" };
const foreign = new AtomicDocumentAssembly(wrongActor, decodeAtomicDocumentBackboneBatch);
for (const frame of frames.slice(0, -1)) await foreign.admit(wrongActor, frame);
await denied(() => foreign.admit(wrongActor, frames.at(-1)!), "row binding");
const damaged = new AtomicDocumentAssembly(scope, decodeAtomicDocumentBackboneBatch);
for (const frame of frames.slice(0, -1)) await damaged.admit(scope, frame);
const last = frames.at(-1)!.slice();
last[last.length - 1]! ^= 1;
await denied(() => damaged.admit(scope, last), "digest");
const cancelled = new AtomicDocumentAssembly(scope, decodeAtomicDocumentBackboneBatch);
for (const frame of frames.slice(0, -1)) await cancelled.admit(scope, frame);
const sealing = cancelled.admit(scope, frames.at(-1)!);
cancelled.retire();
await denied(() => sealing, "stale sealed binding");
console.log("[DEBUG] TypeScript sealed 15 rows and exact 473683-byte PNG through independent Python frames");
