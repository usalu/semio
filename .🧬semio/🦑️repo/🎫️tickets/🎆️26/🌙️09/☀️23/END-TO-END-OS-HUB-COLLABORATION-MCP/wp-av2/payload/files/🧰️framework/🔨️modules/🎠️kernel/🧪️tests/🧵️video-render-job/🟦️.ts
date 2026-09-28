import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { MEDIA_VIDEO_RENDER_CAPABILITY, type VideoRenderJobEvent, VideoRenderJobLedger, type VideoRenderJobRow } from "../../🟦️.ts";

/** 🧵️ TypeScript twin of `🧪️tests/🧵️video-render-job/🦀️.rs`, folding the SAME fixture (`🧫️fixtures/🧵️video-render-job/🔣️.json`):
 * a host's video render task list is exactly the fold of its job events, and a refused event changes nothing. */

interface JobFixture {
  readonly capability: string;
  readonly streams: readonly { readonly id: string; readonly events: readonly VideoRenderJobEvent[]; readonly running: readonly VideoRenderJobRow[]; readonly lastJob: number }[];
  readonly refusals: readonly { readonly id: string; readonly error: string; readonly events: readonly VideoRenderJobEvent[] }[];
}

function loadFixture(): JobFixture {
  return JSON.parse(readFileSync(join(dirname(fileURLToPath(import.meta.url)), "../../🧫️fixtures/🧵️video-render-job/🔣️.json"), "utf8")) as JobFixture;
}

/** ⚖️ The whole law, callable without vitest so the nx lane runs it as a plain oracle. */
export function testVideoRenderJobContract(): void {
  const fixture = loadFixture();
  assert.equal(fixture.capability, MEDIA_VIDEO_RENDER_CAPABILITY);
  for (const stream of fixture.streams) {
    const ledger = VideoRenderJobLedger.fold(stream.events);
    assert.ok(ledger instanceof VideoRenderJobLedger, `${stream.id}: ${JSON.stringify(ledger)}`);
    assert.deepEqual(ledger.running(), stream.running, `${stream.id}: running rows`);
    assert.equal(ledger.lastJob(), stream.lastJob, `${stream.id}: last job`);
    const stable = ledger.running();
    assert.equal(ledger.apply({ kind: "progressed", job: stream.running[0]!.job, completed: stream.running[0]!.completed }), null);
    assert.equal(ledger.running(), stable, `${stream.id}: an event that changes nothing keeps the task list's identity`);
  }
  for (const refusal of fixture.refusals) {
    const head = VideoRenderJobLedger.fold(refusal.events.slice(0, -1));
    assert.ok(head instanceof VideoRenderJobLedger, refusal.id);
    const before = { running: head.running(), lastJob: head.lastJob() };
    assert.equal(head.apply(refusal.events.at(-1)!), refusal.error, refusal.id);
    assert.deepEqual({ running: head.running(), lastJob: head.lastJob() }, before, `${refusal.id}: a refused event changes nothing`);
    assert.deepEqual(VideoRenderJobLedger.fold(refusal.events), { index: refusal.events.length - 1, error: refusal.error }, `${refusal.id}: fold names the refused index`);
  }
  console.log(`video-render-job streams=${fixture.streams.length} refusals=${fixture.refusals.length}`);
}
