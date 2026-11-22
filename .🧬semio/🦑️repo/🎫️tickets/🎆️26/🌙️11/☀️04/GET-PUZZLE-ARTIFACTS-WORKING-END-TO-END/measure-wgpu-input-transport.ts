import { FakeWorker, transport } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📨️browser-frame-transport/🧰️transport-test-support/🟦️.ts";

const modifiers = { shift: false, ctrl: false, alt: false, meta: false };
const warmupFrames = 1_000;
const measuredFrames = 5_000;

for (const profile of [{ pointers: 1, moves: 8 }, { pointers: 2, moves: 32 }]) {
  const worker = new FakeWorker();
  const subject = transport(worker);
  worker.reply({ kind: "booted", lifecycle: 1 });
  const move = {
    kind: "pointer-move" as const, pointerKind: "mouse" as const, pointerId: 1, x: 0, y: 0, ...modifiers,
  };
  const samples: number[] = [];
  for (let frame = 1; frame <= warmupFrames + measuredFrames; frame++) {
    const start = performance.now();
    if (!subject.enqueueLossless({ ...move, kind: "pointer-down", button: 0 })) throw new Error("pointer down refused");
    for (let index = 0; index < profile.moves; index++) {
      const pointerId = 1 + index % profile.pointers;
      if (!subject.enqueueReplaceable({ ...move, pointerId, x: index, y: index })) throw new Error("pointer move refused");
    }
    if (!subject.enqueueLossless({ ...move, kind: "pointer-up", button: 0 })) throw new Error("pointer up refused");
    if (!subject.flush(frame)) throw new Error("frame refused");
    const batch = worker.messages.at(-1);
    if (batch?.kind !== "batch" || batch.events.length !== profile.pointers + 2) throw new Error("input did not coalesce");
    worker.reply({ kind: "batch-accepted", lifecycle: 1, inputSequence: batch.inputSequence, generation: batch.generation });
    worker.messages.length = 0;
    worker.transfers.length = 0;
    if (frame > warmupFrames) samples.push(performance.now() - start);
  }
  samples.sort((left, right) => left - right);
  const percentile = (fraction: number) => Math.round(samples[Math.ceil(samples.length * fraction) - 1]! * 1_000);
  console.log(JSON.stringify({ ...profile, frames: samples.length, p50Us: percentile(0.5), p95Us: percentile(0.95), p99Us: percentile(0.99) }));
}
