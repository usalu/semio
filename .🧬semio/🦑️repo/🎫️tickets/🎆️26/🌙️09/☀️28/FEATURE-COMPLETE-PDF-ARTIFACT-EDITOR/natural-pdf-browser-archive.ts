/** 📬 Drives the browser half of the scoped first-genesis archive exchange. */
import type { NaturalSourceAcceptance } from "./natural-source.ts";

const encoder = new TextEncoder();
const decoder = new TextDecoder("utf-8", { fatal: true });
const requestMagic = encoder.encode("SEMIOFA1");
const replyMagic = encoder.encode("SEMIOFR1");
const HEADER = 43;
const PAGE = 4096;
const MAXIMUM_ARCHIVE_BYTES = 4 * 1024 * 1024;
const enum Action { Open = 1, Preview, Acknowledge, Seal, Cancel, Close }

export type NaturalArchiveFrontier = Readonly<{
  documentId: string; generation: bigint; revision: Uint8Array; extent: number;
}>;
export type NaturalArchiveReceipt = Readonly<{ extent: number; digest: Uint8Array; frontier: NaturalArchiveFrontier }>;
type Reply =
  | { kind: "pending" | "refused" | "acknowledged" | "closed" }
  | { kind: "opened"; frontier: NaturalArchiveFrontier }
  | { kind: "page"; bytes: Uint8Array }
  | { kind: "sealed"; digest: Uint8Array };

export type NaturalArchiveCapture = Readonly<{
  write(offset: number, page: Uint8Array): Promise<{offset: number; length: number}>;
  publish(digest: Uint8Array, signal: AbortSignal): Promise<NaturalArchiveReceipt>;
  retire(): Promise<void>;
}>;
export type NaturalArchiveExchange = Readonly<{
  run(packet: Uint8Array, closing: boolean): Promise<Uint8Array | undefined>;
  yieldTurn(): Promise<void>;
  retired(): boolean;
  now(): number;
}>;

function equalBytes(a: Uint8Array, b: Uint8Array): boolean {
  return a.length === b.length && a.every((byte, index) => byte === b[index]);
}
/** 🧾 Identifies the scoped archive reply before generic shell frame routing. */
export function isNaturalArchiveReply(bytes: Uint8Array): boolean {
  return bytes.length > HEADER && equalBytes(bytes.subarray(0,8),replyMagic);
}
function token(value: string): bigint {
  if (!/^[0-9a-f]{16}$/u.test(value) || value === "0000000000000000") throw Error("natural-archive.invalid-loan");
  return BigInt("0x" + value);
}

/** 📄 Builds an exact little-endian request under the completed natural source's identity. */
function request(accepted: NaturalSourceAcceptance, action: Action, offset = 0, length = 0): Uint8Array {
  const {loan, owner, seq, kind, extent, phase, result} = accepted;
  const instance = loan.instanceId;
  if (!Number.isInteger(instance) || instance < 1 || instance > 0xffffffff || owner !== BigInt(instance) ||
      seq < 1n || seq > 0xffffffffffffffffn || kind !== 241 || phase !== "complete" ||
      result !== "accepted" || !Number.isInteger(extent) || extent < 1 || extent !== loan.extent) throw Error("natural-archive.invalid-loan");
  const bytes = new Uint8Array(HEADER);
  bytes.set(requestMagic);
  const view = new DataView(bytes.buffer);
  view.setUint32(8, instance, true);
  view.setBigUint64(12, token(loan.operationId), true);
  view.setBigUint64(20, token(loan.generation), true);
  view.setBigUint64(28, seq, true);
  view.setUint8(36, action);
  view.setUint32(37, offset, true);
  view.setUint16(41, length, true);
  return bytes;
}

function decode(offer: Uint8Array, bytes: Uint8Array): Reply {
  if (bytes.length <= HEADER || !equalBytes(bytes.subarray(0,8),replyMagic) ||
      !equalBytes(bytes.subarray(8,HEADER),offer.subarray(8))) throw Error("natural-archive.foreign-receipt");
  const tag = bytes[HEADER];
  const action = offer[36];
  const tail = bytes.subarray(HEADER+1);
  if (tail.length === 0) {
    if (tag === 0) return { kind: "pending" };
    if (tag === 6) return { kind: "refused" };
    if (tag === 3 && action === Action.Acknowledge) return { kind: "acknowledged" };
    if (tag === 5 && (action === Action.Cancel || action === Action.Close)) return { kind: "closed" };
  }
  if (tag === 1 && action === Action.Open && tail.length >= 45 && tail.length <= 2092) {
    const view = new DataView(tail.buffer, tail.byteOffset, tail.byteLength);
    const extent = view.getUint32(0, true);
    const documentId = decoder.decode(tail.subarray(44));
    if (extent < 1 || extent > MAXIMUM_ARCHIVE_BYTES || [...documentId].length > 512) throw Error("natural-archive.invalid-extent");
    return { kind: "opened", frontier: { extent, documentId, generation: view.getBigUint64(4,true), revision: Uint8Array.from(tail.subarray(12,44)) } };
  }
  if (tag === 2 && action === Action.Preview && tail.length > 0 && tail.length <= PAGE) return { kind: "page", bytes: tail };
  if (tag === 4 && action === Action.Seal && tail.length === 32) return { kind: "sealed", digest: Uint8Array.from(tail) };
  throw Error("natural-archive.invalid-receipt");
}

function matchingFrontier(left: NaturalArchiveFrontier, right: NaturalArchiveFrontier) {
  return left.documentId === right.documentId && left.generation === right.generation && left.extent === right.extent &&
    equalBytes(left.revision,right.revision);
}

/** 💾 Waits for each durable page receipt and seals the same Store frontier before returning it. */
export async function persistNaturalArchive(
  accepted: NaturalSourceAcceptance, documentId: string, exchange: NaturalArchiveExchange,
  capture: (frontier: NaturalArchiveFrontier, signal: AbortSignal) => Promise<NaturalArchiveCapture>,
  signal: AbortSignal, current: () => boolean, progress?: (completed: number, total: number) => void,
): Promise<NaturalArchiveReceipt> {
  const ask = async (action: Action, closing = false, offset = 0, length = 0): Promise<Reply> => {
    const packet = request(accepted, action, offset, length);
    const deadline = exchange.now() + 300_000;
    for (;;) {
      if (closing) {
        if (exchange.retired()) return { kind: "closed" };
      } else {
        signal.throwIfAborted();
        if (!current() || exchange.retired()) throw Error("natural-archive.stale-owner");
      }
      if (exchange.now() >= deadline) throw Error("natural-archive.receipt-stalled");
      const bytes = await exchange.run(packet,closing);
      const reply = bytes ? decode(packet,bytes) : { kind: "pending" as const };
      if (reply.kind === "refused") throw Error("natural-archive.refused");
      if (reply.kind !== "pending") return reply;
      await exchange.yieldTurn();
    }
  };
  let sink: NaturalArchiveCapture | undefined;
  let guestClosed = false;
  try {
    const opened = await ask(Action.Open);
    if (opened.kind !== "opened" || opened.frontier.documentId !== documentId) throw Error("natural-archive.foreign-document");
    const frontier = opened.frontier;
    signal.throwIfAborted();
    sink = await capture({ ...frontier, revision: frontier.revision.slice() },signal);
    progress?.(0,frontier.extent);
    for (let offset = 0; offset < frontier.extent;) {
      const length = Math.min(PAGE,frontier.extent-offset);
      const page = await ask(Action.Preview,false,offset);
      if (page.kind !== "page" || page.bytes.length !== length) throw Error("natural-archive.page-extent");
      signal.throwIfAborted();
      const written = await sink.write(offset,page.bytes);
      if (written.offset !== offset || written.length !== length) throw Error("natural-archive.foreign-write");
      if ((await ask(Action.Acknowledge,false,offset,length)).kind !== "acknowledged") throw Error("natural-archive.page-unacknowledged");
      offset += length;
      progress?.(offset,frontier.extent);
    }
    const sealed = await ask(Action.Seal);
    if (sealed.kind !== "sealed") throw Error("natural-archive.unsealed");
    signal.throwIfAborted();
    const saved = await sink.publish(sealed.digest,signal);
    if (saved.extent !== frontier.extent || !equalBytes(saved.digest,sealed.digest) || !matchingFrontier(saved.frontier,frontier)) {
      throw Error("natural-archive.foreign-persistence");
    }
    const verified = await ask(Action.Seal);
    if (verified.kind !== "sealed" || !equalBytes(verified.digest,sealed.digest)) throw Error("natural-archive.frontier-changed");
    if ((await ask(Action.Close,true)).kind !== "closed") throw Error("natural-archive.close-unacknowledged");
    guestClosed = true;
    await sink.retire();
    sink = undefined;
    signal.throwIfAborted();
    if (!current() || exchange.retired()) throw Error("natural-archive.stale-owner");
    return { extent: saved.extent, digest: Uint8Array.from(saved.digest), frontier };
  } catch (failure) {
    const settled = await Promise.allSettled([guestClosed ? undefined : ask(Action.Cancel,true),sink?.retire()]);
    const faults = settled.flatMap(result=>result.status === "rejected" ? [result.reason] : []);
    if (faults.length) throw new AggregateError([failure,...faults],"Natural archive capture and retirement failed");
    throw failure;
  }
}
