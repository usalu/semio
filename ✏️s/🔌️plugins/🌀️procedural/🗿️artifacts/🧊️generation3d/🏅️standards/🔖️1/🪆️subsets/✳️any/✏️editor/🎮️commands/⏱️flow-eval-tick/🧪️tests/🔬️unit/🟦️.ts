import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

/** ⚖️ Third-party twin of the contribution-gated arming fixture: `JSON.parse` must see the same two
 * arms the Rust law drives — an uncontributed graph arming nothing, a served graph keeping exactly
 * one re-arm — and `setContributions` as the one route that resumes a gated chain. */
export function testGeneration3dContributionGatedArmingContract(): void {
  const here = fileURLToPath(new URL(".", import.meta.url));
  const fixture = JSON.parse(readFileSync(`${here}/../../../../🧫️fixtures/🚧️contribution-gated-arming.json`, "utf8")) as {
    format: string;
    version: number;
    unservedKind: string;
    servedKindCandidates: string[];
    tickStepBudget: number;
    cases: Array<{ id: string; kindSource: string; widgets: number; unfinishedTick: boolean; mayRearm: boolean; armedTicks: number }>;
    resume: { command: string; rearmsPerAttachedPreview: number; attachedPreviewWindows: number };
  };
  assert.equal(fixture.format, "semio.generation3d.contribution-gated-arming");
  assert.equal(fixture.version, 1);
  const blocked = fixture.cases.find((row) => row.kindSource === "unservedKind");
  const served = fixture.cases.find((row) => row.kindSource === "servedKindCandidates");
  assert.equal(blocked?.mayRearm, false);
  assert.equal(blocked?.armedTicks, 0);
  assert.equal(served?.mayRearm, true);
  assert.equal(served?.armedTicks, 1);
  for (const row of [blocked, served]) {
    assert.equal(row?.unfinishedTick, true, "both arms must outrun one tick, or the re-arm decision is never reached");
    assert.ok((row?.widgets ?? 0) > fixture.tickStepBudget, "a graph that fits in one tick never asks for a re-arm");
  }
  assert.equal(fixture.resume.command, "setContributions");
  assert.equal(fixture.resume.rearmsPerAttachedPreview, 1);
  console.log(`generation3d contribution-gated arming blocked=${blocked?.armedTicks} served=${served?.armedTicks} resume=${fixture.resume.command} budget=${fixture.tickStepBudget}`);
}

/** 🔒️ One preview window's arming latch, re-implemented from the fixture's own prose — the
 * independent half of the language-agnostic law. Rust drives the identical rows against the real
 * `FlowEvalSession`; both must answer the same `armed` lists. */
interface TickLatch {
  armed: boolean;
  inFlight: number;
  owed: boolean;
  unfinished: boolean;
}

type TickLatchEvent = { event: string; window?: string; parks?: number; more?: boolean; ok?: boolean };

class TickLatchTable {
  private readonly latches = new Map<string, TickLatch>();

  private at(windowId: string): TickLatch {
    let latch = this.latches.get(windowId);
    if (!latch) {
      latch = { armed: false, inFlight: 0, owed: false, unfinished: false };
      this.latches.set(windowId, latch);
    }
    return latch;
  }

  arm(windowId: string): boolean {
    const latch = this.at(windowId);
    if (latch.armed) return false;
    if (latch.inFlight > 0) {
      latch.owed = true;
      return false;
    }
    latch.armed = true;
    latch.owed = false;
    return true;
  }

  owes(windowId: string): boolean {
    const latch = this.latches.get(windowId);
    if (!latch) return true;
    return latch.unfinished && !latch.armed && latch.inFlight === 0;
  }

  armOwed(windowId: string): boolean {
    return this.owes(windowId) && this.arm(windowId);
  }

  begin(windowId: string): void {
    this.at(windowId).armed = false;
  }

  noteOutcome(windowId: string, unfinished: boolean): void {
    this.at(windowId).unfinished = unfinished;
  }

  noteInFlight(windowId: string, count: number): void {
    this.at(windowId).inFlight += count;
  }

  settle(windowId: string): boolean {
    const latch = this.at(windowId);
    latch.inFlight = Math.max(0, latch.inFlight - 1);
    if (latch.inFlight === 0 && latch.owed && !latch.armed) {
      latch.armed = true;
      latch.owed = false;
      return true;
    }
    return false;
  }

  abandon(windowId: string): void {
    const latch = this.at(windowId);
    latch.unfinished = false;
    latch.owed = false;
  }

  retain(windowIds: readonly string[]): void {
    for (const key of [...this.latches.keys()]) if (!windowIds.includes(key)) this.latches.delete(key);
  }
}

function replayTickLatchEvent(table: TickLatchTable, attached: readonly string[], event: TickLatchEvent, rowId: string): string[] {
  const named = (): string => {
    assert.ok(event.window, `${rowId}: a ${event.event} event names its window`);
    return event.window as string;
  };
  switch (event.event) {
    case "refresh":
      table.retain(attached);
      return attached.filter((windowId) => table.armOwed(windowId));
    case "gesture":
      return attached.filter((windowId) => table.arm(windowId));
    case "tick": {
      const windowId = named();
      table.begin(windowId);
      table.noteOutcome(windowId, event.more === true);
      if ((event.parks ?? 0) > 0) {
        table.noteInFlight(windowId, event.parks ?? 0);
        return [];
      }
      return event.more === true && table.arm(windowId) ? [windowId] : [];
    }
    case "resolve": {
      const windowId = named();
      if (event.ok !== true) table.abandon(windowId);
      const discharged = table.settle(windowId);
      const armed = event.ok === true && event.more === true && table.arm(windowId);
      return discharged || armed ? [windowId] : [];
    }
    case "invalidate":
      table.retain([]);
      return [];
    case "detach": {
      const windowId = named();
      table.retain(attached.filter((candidate) => candidate !== windowId));
      return [];
    }
    default:
      throw new Error(`${rowId}: unknown tick-latch event ${event.event}`);
  }
}

/** ⚖️ Third-party twin of the tick-latch fixture: an independent TypeScript implementation of the
 * declared arming state machine must replay every row to the same `armed` lists the Rust law gets
 * from the real `FlowEvalSession`, and no event may ever arm a window twice. */
export function testGeneration3dTickLatchContract(): void {
  const here = fileURLToPath(new URL(".", import.meta.url));
  const fixture = JSON.parse(readFileSync(`${here}/../../../../../🧫️fixtures/🔒️tick-latch.json`, "utf8")) as {
    format: string;
    version: number;
    windowKinds: Record<string, string>;
    rows: Array<{ id: string; surface: string; attached: Array<{ id: string; kind: string }>; sequence: TickLatchEvent[]; armed: string[][] }>;
  };
  assert.equal(fixture.format, "semio.generation3d.tick-latch");
  assert.equal(fixture.version, 1);
  assert.equal(fixture.windowKinds.preview, "procedural-preview");
  assert.equal(fixture.windowKinds.generatePreview, "generation3d-generate-preview");
  assert.equal(fixture.windowKinds.viewPreview, "procedural-view-preview");
  const surfaces = new Set(fixture.rows.map((row) => row.surface));
  assert.ok(surfaces.has("editor") && surfaces.has("viewer"), "the shared chain's law must cover both surfaces");

  let totalArms = 0;
  for (const row of fixture.rows) {
    assert.equal(row.sequence.length, row.armed.length, `${row.id}: every event owes exactly one armed answer`);
    for (const window of row.attached) assert.ok(Object.values(fixture.windowKinds).includes(window.kind) || window.kind in fixture.windowKinds, `${row.id}: ${window.kind} is not a declared window kind`);
    const table = new TickLatchTable();
    const attached = row.attached.map((window) => window.id);
    const observed = row.sequence.map((event) => replayTickLatchEvent(table, attached, event, row.id));
    assert.deepEqual(observed, row.armed, `tick-latch row ${row.id}`);
    // 🔒️ The invariant itself, not just the table: no window is armed twice without a tick running
    // or an answer landing in between.
    const pending = new Map<string, number>();
    row.sequence.forEach((event, index) => {
      for (const windowId of observed[index]) pending.set(windowId, (pending.get(windowId) ?? 0) + 1);
      if (event.event === "tick" && event.window) pending.set(event.window, Math.max(0, (pending.get(event.window) ?? 0) - 1));
      // 🧹️ A tick armed at a window that then leaves the roster is discarded by the shell with it,
      // and a registry replacement abandons every chain — neither leaves a pending tick behind.
      if (event.event === "detach" && event.window) pending.set(event.window, 0);
      if (event.event === "invalidate") pending.clear();
      for (const [windowId, count] of pending) assert.ok(count <= 1, `${row.id}: ${windowId} has ${count} pending ticks after event ${index}`);
    });
    totalArms += observed.reduce((sum, armed) => sum + armed.length, 0);
  }
  console.log(`generation3d tick-latch rows=${fixture.rows.length} surfaces=${[...surfaces].sort().join("+")} arms=${totalArms}`);
}

if (import.meta.main) {
  testGeneration3dContributionGatedArmingContract();
  testGeneration3dTickLatchContract();
}
