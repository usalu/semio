import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

/** 🩹️ Third-party twin of the GESTURE RE-ARM LAW (`🧫️fixtures/⏯️preview-eval-run.json`, `gestureRearm`)
 * — an independent TypeScript model of what a landed gesture owes the attached preview windows, of what
 * that gesture must carry on its own emit, and of how many run actions the debt may cost, driven from
 * the SAME fixture the Rust laws drive.
 *
 * Rust replays each row against the real `FlowEvalSession` latches and the real
 * `preview_eval_run_effects`; this file rebuilds all three from the fixture's own prose — the per-window
 * arming latch, the link's request latch and the poll ladder — so the two implementations can only
 * agree by agreeing on the law rather than on one another's code.
 *
 * 🪪️ Three live defects the rows pin down, all measured on 6018. (1) A gesture that MOVED THE DOCUMENT
 * owed the previews nothing unless its tool id happened to be on a hand-kept roster, so the inspector's
 * `patchFlowWidgets` moved `height` 6 → 7 and the preview's published payload stayed byte-identical.
 * (2) Once it owed, the debt was still never read: the guest recorded it and the console then went
 * silent for 60 s, because the host polls `pending_effects` once per refresh and refreshes on ACTIVITY,
 * and a settled, finalized run leaves none. (3) A start was re-asked once per REFRESH rather than once
 * per ANSWER, which spent three `toolRunStart` invocations in 500 ms on a quiet boot and 1 053 in one
 * busy session (`📓️preview-rearm-after-inspector-edit-2026-09-14.md`,
 * ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */

type RunState = "starting" | "running" | "paused" | "complete" | "finalizing" | "finalized" | "aborting" | "aborted" | "faulted";
type RunRequest = "start" | "finalize";

interface JobSupersessionRow {
  id: string;
  linkRun: number | null;
  closingRun: number;
  settled: boolean;
  expected: { owns: boolean; cancels: boolean; clearsPort: boolean };
}

interface GestureRearmRow {
  id: string;
  windows: string[];
  windowsSettled: boolean;
  artifactMutations: number;
  servable: boolean;
  runIsLive: boolean;
  runHasSettled: boolean;
  polls: number;
  startLagPolls: number;
  pollStartView: "none" | RunState;
  expected: { owes: boolean; owedWindows: string[]; hops: string[]; starts: number; gestureEffects: string[] };
}

/** 🔒️ The per-window arming latch, rebuilt from `📓️tick-arming-latch-2026-09-12.md`'s prose: a window
 * owes a tick when its own last tick reported unfinished work and nothing is already chasing it, and a
 * window that has never ticked owes its first. */
class WindowLatch {
  seen = false;
  armed = false;
  inFlight = 0;
  owed = false;
  unfinished = false;

  arm(): boolean {
    this.seen = true;
    if (this.armed) return false;
    if (this.inFlight > 0) {
      this.owed = true;
      return false;
    }
    this.armed = true;
    this.owed = false;
    return true;
  }

  begin(): void {
    this.armed = false;
  }

  noteOutcome(unfinished: boolean): void {
    this.seen = true;
    this.unfinished = unfinished;
  }

  tickOwed(): boolean {
    if (!this.seen) return true;
    return this.unfinished && !this.armed && this.inFlight === 0;
  }
}

/** 🚦️ The surface-owned half of the run: the roster it schedules over, whether a job is live to be
 * woken, the generation it last settled, and the request it has asked for and not yet seen answered. */
class RunLink {
  settled: string | null = null;
  restartOwed = false;
  live = false;
  requested: RunRequest | null = null;
}

/** ⏯️ What answers an outstanding request. Deliberately not "the view changed": an unrelated generation
 * bump would clear a latch whose request is still outstanding, and surviving exactly that is the point. */
const answeredBy = (request: RunRequest, state: RunState | null): boolean =>
  request === "start" ? !(state === null || state === "finalized" || state === "aborted" || state === "faulted") : state !== "complete";

/** 🩹️ The law under test, half one: what a landed gesture owes, and what it must carry itself. */
function oweAttachedPreviewsForMutations(latches: Map<string, WindowLatch>, link: RunLink, windows: string[], artifactMutations: number, servable: boolean): { owes: boolean; effects: string[] } {
  if (artifactMutations === 0) return { owes: false, effects: [] };
  for (const window of windows) latches.get(window)!.noteOutcome(true);
  link.requested = null;
  const effects: string[] = [];
  // ⏰️ Only a job that has NOT settled can be woken into more work: a settled one answers "complete" to
  // every wake, so a surface holding one is as unable to pay a fresh debt as a surface holding none.
  const woken = link.live && link.settled === null;
  if (windows.length > 0 && servable && !woken && link.requested === null) {
    link.requested = "start";
    effects.push("toolRunStart");
  }
  return { owes: true, effects };
}

/** 🚦️ The law under test, half two: what one `pending_effects` poll owes the run. */
function previewEvalRunEffects(latches: Map<string, WindowLatch>, link: RunLink, windows: string[], run: { id: number; generation: number; state: RunState } | null, servable: boolean): string[] {
  if (windows.length === 0 || !servable) return [];
  const owed = windows.some((window) => latches.get(window)!.tickOwed());
  const state = run?.state ?? null;
  if (link.requested !== null && answeredBy(link.requested, state)) link.requested = null;
  if (link.requested !== null) return [];
  let request: RunRequest | null = null;
  let effects: string[] = [];
  if (state === "complete") {
    // 🏁️ A complete run is FINALIZED as soon as it is seen, owed or not: `complete` means the job is done
    // and the run awaits its finalize, and a run left standing in it makes every later `toolRunStart` a
    // no-op against the run already there. Pairing the start into this poll does not help either — the
    // host schedules a poll's effects independently, so it would land while the run is still complete.
    link.restartOwed = owed;
    request = "finalize";
    effects = ["toolRunFinalize"];
  } else if ((state === null || state === "finalized" || state === "aborted" || state === "faulted") && (owed || link.restartOwed)) {
    link.restartOwed = false;
    request = "start";
    effects = ["toolRunStart"];
  }
  link.requested = request;
  return effects;
}

/** 🦶️ The scheduling law: dispatch the first window that owes a hop nothing is chasing. */
function nextHop(latches: Map<string, WindowLatch>, windows: string[]): number | "wait" | "settled" {
  const index = windows.findIndex((window) => latches.get(window)!.tickOwed());
  if (index >= 0) return index;
  return windows.some((window) => latches.get(window)!.armed || latches.get(window)!.inFlight > 0) ? "wait" : "settled";
}

export function testGeneration3dGestureRearm(): void {
  const here = fileURLToPath(new URL(".", import.meta.url));
  const fixture = JSON.parse(readFileSync(`${here}/../../🧫️fixtures/⏯️preview-eval-run.json`, "utf8")) as {
    format: string;
    version: number;
    gestureRearm: { note: string; rows: GestureRearmRow[] };
    jobSupersession: { note: string; rows: JobSupersessionRow[] };
  };
  assert.equal(fixture.format, "semio.generation3d.preview-eval-run");
  assert.equal(fixture.version, 1);
  const rows = fixture.gestureRearm.rows;
  assert.ok(rows.length > 0, "the gesture re-arm law must declare rows");

  for (const row of rows) {
    const stage = (): [Map<string, WindowLatch>, RunLink] => {
      const latches = new Map<string, WindowLatch>();
      for (const window of row.windows) {
        const latch = new WindowLatch();
        if (row.windowsSettled) {
          latch.arm();
          latch.begin();
          latch.noteOutcome(false);
        }
        latches.set(window, latch);
      }
      const link = new RunLink();
      link.settled = row.runHasSettled ? "1/0" : null;
      link.live = row.runIsLive;
      return [latches, link];
    };

    // 🤫️ A settled run stays quiet however often the host polls it — the baseline the storm broke.
    const [quiet, quietLink] = stage();
    if (row.windowsSettled) {
      // 🏁️ A complete run is finalized ONCE, however often the host polls it while the view still answers
      // `complete` — the request latch, not the poll cadence, decides how often it asks.
      const asked: string[] = [];
      for (let poll = 0; poll < row.polls; poll += 1) asked.push(...previewEvalRunEffects(quiet, quietLink, row.windows, { id: 1, generation: 0, state: "complete" }, true));
      assert.deepEqual(asked, row.windows.length === 0 ? [] : ["toolRunFinalize"], `${row.id}: a settled complete run is finalized exactly once, however often the host polls it`);
    }

    // 🩹️ The gesture.
    const [latches, link] = stage();
    const gesture = oweAttachedPreviewsForMutations(latches, link, row.windows, row.artifactMutations, row.servable);
    assert.equal(gesture.owes, row.expected.owes, `${row.id}: a gesture owes the previews exactly when it moved the document`);
    assert.deepEqual(gesture.effects, row.expected.gestureEffects, `${row.id}: what the gesture's own emit carries — a settled, finalized run leaves nothing that would ask the host for another poll`);
    const owedWindows = row.windows.filter((window) => latches.get(window)!.tickOwed());
    assert.deepEqual(owedWindows, row.expected.owedWindows, `${row.id}: owed windows`);

    // 🦶️ One landed mutation arms each attached window exactly ONCE.
    const hops: string[] = [];
    for (let guard = 0; guard <= row.windows.length; guard += 1) {
      const hop = nextHop(latches, row.windows);
      if (typeof hop !== "number") break;
      const window = row.windows[hop]!;
      hops.push(window);
      assert.equal(latches.get(window)!.arm(), true, `${row.id}: a dispatched hop arms its window`);
      latches.get(window)!.begin();
      latches.get(window)!.noteOutcome(false);
      assert.ok(hops.length <= row.windows.length, `${row.id}: one landed mutation may never arm a window twice`);
    }
    assert.deepEqual(hops, row.expected.hops, `${row.id}: hop schedule`);

    // ⏯️ The storm half, on a fresh staging: the gesture again, then the host's own poll ladder with its
    // run view lagging `startLagPolls` behind every request it was handed.
    const [spinning, spinningLink] = stage();
    let starts = oweAttachedPreviewsForMutations(spinning, spinningLink, row.windows, row.artifactMutations, row.servable).effects.length;
    let view: { id: number; generation: number; state: RunState } | null = row.pollStartView === "none" ? null : { id: 1, generation: 0, state: row.pollStartView };
    let landing: { remaining: number; state: RunState } | null = null;
    for (let poll = 0; poll < row.polls; poll += 1) {
      for (const effect of previewEvalRunEffects(spinning, spinningLink, row.windows, view, row.servable)) {
        if (effect === "toolRunStart") starts += 1;
        landing = { remaining: row.startLagPolls, state: effect === "toolRunStart" ? "running" : "finalized" };
      }
      if (landing && landing.remaining === 0) {
        view = { id: view?.id ?? 1, generation: view?.generation ?? 0, state: landing.state };
        landing = null;
      } else if (landing) {
        landing = { ...landing, remaining: landing.remaining - 1 };
      }
    }
    assert.equal(starts, row.expected.starts, `${row.id}: one gesture owes at most one run start, however slowly the run view catches up`);
    console.log(`generation3d gestureRearm ${row.id} owes=${gesture.owes} owed=${owedWindows.join(",") || "-"} hops=${hops.join(",") || "-"} carried=${gesture.effects.join(",") || "-"} starts=${starts}`);
  }

  // 🧹️ Only the run job the link currently belongs to may quiesce the session on close. A start retires
  // the previous entry, so the OLD job's close runs after the NEW job installed its own port and reset
  // the settled marker; a closing job that reads those as its own cancels the evaluation the fresh run
  // is in the middle of.
  for (const row of fixture.jobSupersession.rows) {
    const link = { jobRun: row.linkRun, port: "live" as string | null, settled: row.settled ? "1/0" : null };
    const owns = link.jobRun === row.closingRun && link.jobRun !== null;
    let cancels = false;
    let clearsPort = false;
    if (owns) {
      link.port = null;
      link.jobRun = null;
      clearsPort = true;
      cancels = link.settled === null;
    }
    assert.equal(owns, row.expected.owns, `${row.id}: ownership`);
    assert.equal(cancels, row.expected.cancels, `${row.id}: cancels the evaluation`);
    assert.equal(clearsPort, row.expected.clearsPort, `${row.id}: clears the link's port`);
    assert.equal(link.port === null, clearsPort, `${row.id}: a superseded job may never take the live run's port away`);
    console.log(`generation3d jobSupersession ${row.id} owns=${owns} cancels=${cancels} clearsPort=${clearsPort}`);
  }

  // 🩹️ The rule is the EMIT, not the roster: the same window, the same latch, only the mutation count
  // differs — and that alone decides.
  const one = new Map([["only", new WindowLatch()]]);
  one.get("only")!.arm();
  one.get("only")!.begin();
  one.get("only")!.noteOutcome(false);
  assert.equal(oweAttachedPreviewsForMutations(one, new RunLink(), ["only"], 0, true).owes, false, "a gesture that authored no artifact mutation owes a settled preview nothing");
  assert.equal(one.get("only")!.tickOwed(), false);
  assert.equal(oweAttachedPreviewsForMutations(one, new RunLink(), ["only"], 1, true).owes, true, "a gesture that authored one owes it an evaluation");
  assert.equal(one.get("only")!.tickOwed(), true);
}
