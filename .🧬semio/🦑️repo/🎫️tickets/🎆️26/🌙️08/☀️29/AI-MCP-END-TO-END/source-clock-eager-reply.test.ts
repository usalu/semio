import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import fixture from "./source-clock-eager-reply.json";

type Phase = (typeof fixture.steps)[number];
type Case = (typeof fixture.cases)[number];

function drive(law: Case, eager: boolean) {
  const observations: (number | null)[] = [];
  const spent: Phase[] = [];
  let reply: string | null = null;
  let refused = 0;
  let turns = 0;
  let delivered = false;
  let failedClock = false;
  for (const [index, maximumItems] of law.turnItems.entries()) {
    turns++;
    let items = maximumItems;
    let madeProgress = false;
    for (let phase = 0; phase < (eager ? 3 : 1); phase++) {
      const next = fixture.steps[spent.length];
      if (!next) break;
      if (index >= (law.cancelAt ?? Infinity) || spent.length >= (law.cancelAfter ?? Infinity) || items < next.items) {
        if (!madeProgress) refused++;
        break;
      }
      items -= next.items;
      spent.push(next);
      madeProgress = true;
      if (next.name === "prepare") {
        const stamp = law.clockUnavailable ? null : fixture.observations[observations.length]!;
        observations.push(stamp);
        if (stamp === null) {
          failedClock = true;
          refused++;
          break;
        }
        const measured = Buffer.alloc(16);
        measured.writeUInt32LE(1, 0);
        measured.writeBigUInt64LE(BigInt(stamp), 8);
        reply = measured.toString("hex");
      }
      if (next.name === "deliver") delivered = true;
    }
    if (failedClock || spent.length === fixture.steps.length) break;
  }
  return { reply, spent, observations, refused, turns, delivered, failedClock, ready: spent.length === fixture.steps.length };
}

function byteString(value: string) {
  const bytes = [...new TextEncoder().encode(value)];
  return [bytes.length, ...bytes];
}

function wasmClockOracle() {
  const namespace = "semio:framework/pure@1.0.0";
  const name = "operation-clock-us";
  const sections = [
    [1, [2, 0x60, 2, 0x7e, 0x7f, 0, 0x60, 0, 0]],
    [2, [1, ...byteString(namespace), ...byteString(name), 0, 0]],
    [3, [1, 1]],
    [5, [1, 0, 1]],
    [7, [2, ...byteString("memory"), 2, 0, ...byteString("run"), 0, 1]],
    [10, [1, 9, 0, 0x42, 0xc7, 0x00, 0x41, 0, 0x10, 0, 0x0b]],
  ] as const;
  const wasm = new Uint8Array([0, 97, 115, 109, 1, 0, 0, 0, ...sections.flatMap(([id, bytes]) => [id, bytes.length, ...bytes])]);
  const calls: bigint[] = [];
  let memory: WebAssembly.Memory;
  const instance = new WebAssembly.Instance(new WebAssembly.Module(wasm), { [namespace]: {
    [name](binding: bigint, address: number) {
      calls.push(binding);
      const bytes = new DataView(memory.buffer, address, 16);
      bytes.setUint32(0, 1, true);
      bytes.setBigUint64(8, BigInt(fixture.observations[0]!), true);
    },
  } });
  memory = instance.exports.memory as WebAssembly.Memory;
  (instance.exports.run as () => void)();
  return { calls, reply: Buffer.from(memory.buffer, 0, 16).toString("hex") };
}

test("SourceClock keeps one real observation, the staged receipt and its finite refusals", () => {
  const oracle = wasmClockOracle();
  expect(oracle.calls).toEqual([BigInt(fixture.origin)]);
  expect(oracle.reply).toBe(fixture.replyHex);
  const db = new Database(":memory:");
  try {
    const sums = db.query(`
      SELECT total(json_extract(value,'$.items')) AS items,
             total(json_extract(value,'$.bytes')) AS bytes,
             total(json_extract(value,'$.work')) AS work
        FROM json_each(?)
    `);
    for (const law of fixture.cases) {
      const stepped = drive(law, false);
      const eager = drive(law, true);
      const { turns: _steppedTurns, ...steppedResult } = stepped;
      const { turns: _eagerTurns, ...eagerResult } = eager;
      expect(eagerResult, law.name).toEqual(steppedResult);
      expect([stepped.turns, eager.turns], law.name).toEqual(law.attempts);
      expect(eager.observations.length, law.name).toBe(law.observed);
      expect(eager.refused, law.name).toBe(law.refused);
      expect(eager.reply, law.name).toBe(law.observed && !law.clockUnavailable ? oracle.reply : null);
      expect(sums.get(JSON.stringify(eager.spent)), law.name).toEqual(sums.get(JSON.stringify(fixture.steps.slice(0, eager.spent.length))));
      if (law.cancelAt !== undefined || law.cancelAfter !== undefined || law.clockUnavailable) {
        expect(eager.delivered, law.name).toBe(false);
        expect(eager.ready, law.name).toBe(false);
      } else {
        expect(eager.delivered, law.name).toBe(true);
        expect(eager.ready, law.name).toBe(true);
      }
    }
  } finally {
    db.close();
  }
});
