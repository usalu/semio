/** 📥️ Streams a selected natural file only after its unpublished actor admits the source. */
import { ACTOR_BYTE_PAGE_BYTES, createActorBytePage } from "@semio-tech/framework-actor/page";
import type { ShardEventEnvelope, ShardCommandIngressPage } from "@semio-tech/framework/actor/shard-client/wire";
import { sendSelectedMedia, type OriginalMediaSource, type OriginalMediaTransferControl, type OriginalMediaFrame } from "./original-media-sender.ts";
import { rawPageBackpressure } from "./original-media-wire.ts";

export const NATURAL_SOURCE_COMMAND_KIND = 241;
export const NATURAL_SOURCE_TRANSPORT_MAXIMUM_BYTES = 512 * 1024 * 1024;

export type NaturalSourceDeclaration = Readonly<{ formatKind: string; extension: string; mediaType: string; binary: boolean }>;
type Offer = Readonly<{ instanceId: number; seq: bigint; declaration: NaturalSourceDeclaration; extent: number }>;
type Loan = Readonly<{ operationId: string; generation: string; instanceId: number; declaration: NaturalSourceDeclaration; extent: number }>;
type Phase = "prepare" | OriginalMediaFrame<Loan>["kind"];
export type NaturalSourceAcceptance = Readonly<{
  loan: Loan; owner: bigint; seq: bigint; kind: typeof NATURAL_SOURCE_COMMAND_KIND; extent: number;
  phase: "complete"; result: "accepted";
}>;
export type NaturalSourceCrossing = Readonly<{ events: readonly ShardEventEnvelope[]; page?: ShardCommandIngressPage }>;

export type NaturalSourceActor = Readonly<{
  retired(): boolean;
  assertOriginalActivation(): void;
  serialize<T>(run: () => Promise<T>): Promise<T>;
  turn(crossing: NaturalSourceCrossing | undefined): Promise<{ readonly ingress: unknown; readonly deferredEvents?: number }>;
  settle(): Promise<void>;
  yieldTurn(): Promise<void>;
  now(): number;
}>;

const token = /^[0-9a-f]{16}$/u;
const exactToken = (value: unknown): value is string => typeof value === "string" && token.test(value) && value !== "0000000000000000";
const record = (value: unknown): Record<string, unknown> | undefined =>
  value !== null && typeof value === "object" ? value as Record<string, unknown> : undefined;

function admittedLoan(value: unknown, offer: Offer, prior: Loan | undefined): Loan {
  const loan = record(value);
  const format = record(loan?.declaration);
  const { declaration } = offer;
  if (!loan || Object.keys(loan).length !== 5 || !format || Object.keys(format).length !== 4 ||
      loan.instanceId !== offer.instanceId || loan.extent !== offer.extent ||
      !exactToken(loan.operationId) || !exactToken(loan.generation) ||
      format.formatKind !== declaration.formatKind || format.extension !== declaration.extension ||
      format.mediaType !== declaration.mediaType || format.binary !== declaration.binary ||
      (prior && (loan.operationId !== prior.operationId || loan.generation !== prior.generation))) {
    throw Error("natural-source.foreign-loan");
  }
  return prior ?? Object.freeze({
    operationId: loan.operationId, generation: loan.generation, instanceId: offer.instanceId,
    declaration, extent: offer.extent,
  });
}

/** 📄 Crosses one bounded page at a time and retains the source until its exact completion receipt. */
export async function sendNaturalSource(
  offer: Offer, source: OriginalMediaSource, actor: NaturalSourceActor, control: OriginalMediaTransferControl,
): Promise<NaturalSourceAcceptance> {
  const { instanceId, seq, declaration, extent } = offer;
  if (!Number.isInteger(instanceId) || instanceId < 1 || instanceId > 0xffffffff ||
      seq < 1n || seq > 0xffffffffffffffffn || !declaration.binary ||
      !Number.isInteger(extent) || extent < 1 || extent > NATURAL_SOURCE_TRANSPORT_MAXIMUM_BYTES || extent !== source.size) {
    throw Error("natural-source.invalid-offer");
  }
  offer = Object.freeze({ instanceId, seq, extent, declaration: Object.freeze({ ...declaration }) });
  const owner = BigInt(instanceId);
  const kind = NATURAL_SOURCE_COMMAND_KIND;
  const pageCount = Math.ceil(extent / ACTOR_BYTE_PAGE_BYTES);
  let loan: Loan | undefined;
  let closed = false;
  let cancellation: Promise<void> | undefined;
  const check = () => {
    control.signal.throwIfAborted();
    if (!control.current() || actor.retired()) throw Error("natural-source.stale-owner");
  };
  const drive = (phase: Phase, crossing: NaturalSourceCrossing, pageIndex?: number) => actor.serialize(async () => {
    const deadline = actor.now() + 300_000;
    let offerCrossing = true;
    for (;;) {
      if (phase === "cancel" && actor.retired()) { closed = true; return; }
      actor.assertOriginalActivation();
      if (phase !== "prepare" && phase !== "cancel") check();
      if (actor.now() >= deadline) throw Error("natural-source.ack-stalled");
      const answer = await actor.turn(offerCrossing ? crossing : undefined);
      const status = record(answer.ingress);
      let retry = rawPageBackpressure(status, crossing.page);
      if (status?.tag === "natural-source-ack") {
        const ack = record(status.val);
        if (!ack || ack.owner !== owner || ack.seq !== seq || ack.kind !== kind || ack.extent !== extent ||
            ack.phase !== phase || (ack.pageIndex ?? undefined) !== pageIndex) throw Error("natural-source.foreign-ack");
        if (ack.loan !== undefined && ack.loan !== null) loan = admittedLoan(ack.loan, offer, loan);
        const result = ack.result;
        if (result === "accepted" || (phase === "cancel" && result === "stale")) {
          if (result === "accepted" && (!loan || ack.loan == null)) throw Error("natural-source.missing-loan");
          if (phase === "complete" || phase === "cancel") closed = true;
          await actor.settle();
          return;
        }
        if (result === "stale" || result === "refused") {
          await actor.settle();
          throw Error("natural-source." + result);
        }
        if (result !== "retry" && result !== "pending") throw Error("natural-source.invalid-ack");
        retry = result === "retry";
        offerCrossing = retry;
      } else {
        const deferred = (answer.deferredEvents ?? 0) > 0;
        offerCrossing = retry || (offerCrossing && (crossing.page !== undefined ? deferred : !deferred));
      }
      await actor.yieldTurn();
    }
  });
  const cross = (frame: OriginalMediaFrame<Loan>, bytes?: Uint8Array) => {
    if (frame.kind !== "page") {
      const event = { events: [{ kind: "natural-source-control", payload: {
        loan: frame.loan, owner, seq, kind, extent, control: frame.kind,
      } }] };
      return frame.kind === "cancel" ? cancellation ??= drive(frame.kind, event) : drive(frame.kind, event);
    }
    const pageIndex = frame.index;
    if (!bytes || bytes.byteLength !== Math.min(ACTOR_BYTE_PAGE_BYTES, extent - pageIndex * ACTOR_BYTE_PAGE_BYTES)) {
      throw Error("natural-source.page-extent");
    }
    const page = createActorBytePage(bytes);
    return drive("page", { events: [], page: {
      cursor: { owner, generation: BigInt("0x" + frame.loan.generation), instance: instanceId, seq, kind, pageIndex,
        pageCount, commandIndex: 0, commandCount: 1, itemCount: 1, metadata: extent },
      bytes, page,
    } }, pageIndex);
  };
  check();
  try {
    await drive("prepare", { events: [{ kind: "natural-source-prepare", payload: offer }] });
    if (!loan) throw Error("natural-source.missing-loan");
    await sendSelectedMedia(loan, source, offer.declaration.mediaType, cross, control);
    if (!closed) throw Error("natural-source.incomplete");
    return Object.freeze({ loan, owner, seq, kind, extent, phase: "complete", result: "accepted" });
  } catch (failure) {
    if (loan && !closed && !cancellation && !actor.retired()) {
      try { await cross({ kind: "cancel", loan }); }
      catch (retirement) { throw new AggregateError([failure, retirement], "Natural source transfer and retirement failed"); }
    }
    throw failure;
  }
}
