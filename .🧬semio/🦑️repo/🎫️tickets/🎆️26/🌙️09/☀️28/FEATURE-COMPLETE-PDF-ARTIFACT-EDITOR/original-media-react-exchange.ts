/** 🔁️ Maintains one actor-serialized control or bounded page until its exact receipt. */
import { ORIGINAL_MEDIA_PAGE_BYTES } from "./original-media-sender.ts";
import { originalMediaCrossing, rawPageBackpressure, readOriginalMediaAck, type OriginalMediaExchange, type OriginalMediaRequest } from "./original-media-wire.ts";

type Crossing = ReturnType<typeof originalMediaCrossing>;
type Driver = Readonly<{
  current(): boolean;
  retired(): boolean;
  assertOriginalActivation(): void;
  serialize<T>(run: () => Promise<T>): Promise<T>;
  turn(crossing: Crossing | undefined): Promise<{ readonly ingress: unknown; readonly deferredEvents?: number }>;
  settle(): Promise<void>;
  yieldTurn(): Promise<void>;
  now(): number;
}>;

export function createOriginalMediaExchange(
  request: OriginalMediaRequest, owner: bigint, seq: bigint, driver: Driver,
): OriginalMediaExchange {
  let extent = 0;
  let acceptedBytes = 0;
  let started = false;
  let closed = false;
  let active = false;
  return {
    current: driver.current,
    async cross(frame, bytes, owned) {
      if (closed) {
        if (frame.kind === "cancel") return;
        throw new Error("original-media.closed");
      }
      if (frame.kind === "cancel" && driver.retired()) { closed = true; return; }
      if (active) throw new Error("original-media.concurrent-crossing");
      active = true;
      try {
        if (frame.kind === "begin") {
          if (extent !== 0) throw new Error("original-media.repeated-begin");
          extent = frame.byteLength;
        }
        if ((frame.kind === "page" && (!started || frame.index * ORIGINAL_MEDIA_PAGE_BYTES !== acceptedBytes)) ||
            (frame.kind === "complete" && (!started || acceptedBytes !== extent))) throw new Error("original-media.order");
        const crossing = originalMediaCrossing(request, owner, seq, extent, frame, bytes);
        await driver.serialize(async () => {
          const deadline = driver.now() + 300_000;
          let offer = true;
          for (;;) {
            if (frame.kind === "cancel" && driver.retired()) { closed = true; return; }
            driver.assertOriginalActivation();
            if (frame.kind !== "cancel" && !owned()) throw new Error("original-media.stale-owner");
            if (driver.now() >= deadline) throw new Error("original-media.ack-stalled");
            const answer = await driver.turn(offer ? crossing : undefined);
            const ack = readOriginalMediaAck(answer.ingress, crossing.identity);
            if (ack === "accepted" || (frame.kind === "cancel" && ack === "stale")) {
              const completed = frame.kind === "complete" || frame.kind === "cancel";
              closed = completed;
              if (frame.kind === "begin") started = true;
              if (frame.kind === "page") acceptedBytes += frame.byteLength;
              await driver.settle();
              if (!completed && !owned()) throw new Error("original-media.stale-owner");
              return;
            }
            if (ack === "stale" || ack === "refused") {
              await driver.settle();
              throw new Error("original-media." + ack);
            }
            const deferred = (answer.deferredEvents ?? 0) > 0;
            offer = ack === "retry" || rawPageBackpressure(answer.ingress, crossing.page) ||
              (offer && ack === undefined && (crossing.page !== undefined ? deferred : !deferred));
            await driver.yieldTurn();
          }
        });
      } finally {
        active = false;
      }
    },
  };
}
