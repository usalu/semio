import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import { nextArchivePollTurnV1 } from "../../../../../../../🧰️framework/🔨️modules/🧾️app-channel/🗃️archive-load-host/🟦️.ts";
import fixture from "./note-archive-poll-cadence.json";

const timer = globalThis.setTimeout;

test("the next status request waits for the progress refresh interval", async () => {
  using oracle = new Database(":memory:");
  const ready = oracle.query<{ settled: number }, [number, number]>("select ? >= ? as settled");
  for (const sample of fixture.samples) {
    expect(Boolean(ready.get(sample.elapsed, fixture.intervalMilliseconds)?.settled)).toBe(sample.settled);
    let resolveTimer: (() => void) | undefined;
    let delay = 0;
    let settled = false;
    globalThis.setTimeout = ((callback: () => void, milliseconds: number) => {
      resolveTimer = callback;
      delay = milliseconds;
    }) as unknown as typeof setTimeout;
    try {
      const pending = nextArchivePollTurnV1().then(() => { settled = true; });
      if (sample.elapsed >= delay) {
        resolveTimer?.();
        await pending;
      }
      expect(settled).toBe(sample.settled);
      if (!settled) {
        resolveTimer?.();
        await pending;
      }
    } finally {
      globalThis.setTimeout = timer;
    }
  }
});
