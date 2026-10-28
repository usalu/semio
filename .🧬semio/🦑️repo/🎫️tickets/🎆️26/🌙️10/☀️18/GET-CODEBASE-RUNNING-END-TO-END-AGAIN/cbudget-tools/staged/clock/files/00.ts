import { availableParallelism, loadavg, platform as osPlatform } from "node:os";
import { validateJsonSchemaSubset } from "../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import table from "./🔣️.json" with { type: "json" };
import {refuseProcessV1} from "../../../📖️texts/🟦️.ts";

/** ⏲️ The closed policy of the load-aware test budget clock: tick, stretch bounds, progress words and the platforms that measure the scheduler themselves. */
export type ContentionPolicyV1 = Readonly<{
  version: 1;
  tickMilliseconds: number;
  stretch: Readonly<{ minimumBasisPoints: number; maximumBasisPoints: number }>;
  announce: Readonly<{ thresholdBasisPoints: number; stepBasisPoints: number; everyMilliseconds: number }>;
  probe: Readonly<{ platforms: readonly string[]; cpuMilliseconds: number; everyMilliseconds: number; smoothingBasisPoints: number }>;
  messages: Readonly<Record<"stretched" | "expired", Readonly<{ en: string; de: string }>>>;
}>;

/** 🐌 What the host answers: how many tasks compete per core (`ratio`, 1 or less is an idle host), the raw load and cores it came from, and who measured it. */
export type ContentionSampleV1 = Readonly<{ ratio: number; load: number; cores: number; source: "loadavg" | "probe" | "wall" }>;
/** 🌫️ Answers the contention of the host right now. */
export type ContentionSourceV1 = () => ContentionSampleV1;

/** 🔐️ Admits a policy against its schema. */
export function admitContentionPolicyV1(value: unknown): ContentionPolicyV1 {
  if (validateJsonSchemaSubset(schema, value, schema).length) throw refuseProcessV1("invalid","testing.contention-policy-required");
  const policy = value as ContentionPolicyV1;
  if (policy.stretch.maximumBasisPoints < policy.stretch.minimumBasisPoints) throw refuseProcessV1("invalid","testing.contention-policy-required");
  return policy;
}

/** 📚️ The authored policy. */
export const CONTENTION_POLICY_V1: ContentionPolicyV1 = admitContentionPolicyV1(table);

/** 🧮️ The factor a budget stretches by: the contention ratio clamped to the policy bounds; an answer that is not a finite number stretches nothing. */
export function budgetStretchV1(ratio: number, policy: ContentionPolicyV1 = CONTENTION_POLICY_V1): number {
  const low = policy.stretch.minimumBasisPoints / 10000, high = policy.stretch.maximumBasisPoints / 10000;
  return Number.isFinite(ratio) ? Math.min(high, Math.max(low, ratio)) : low;
}

/** ⚖️ The budget as work, not as wall time: every wall millisecond is worth `1/stretch` budget milliseconds, so a run that needs its budget on an idle host is never stopped on a busy one and a run that never ends is stopped after at most `stretch` times its budget. */
export type BudgetClockV1 = Readonly<{
  advance: (wallMilliseconds: number, stretch: number) => void;
  creditMilliseconds: () => number;
  expired: () => boolean;
  wallUntilExpiryMilliseconds: (stretch: number) => number;
}>;

const EPSILON = 1e-6;

/** ⏳️ Creates the clock of one budget; it is pure, the caller measures the wall time and the stretch. */
export function createBudgetClockV1(budgetMilliseconds: number): BudgetClockV1 {
  if (!Number.isFinite(budgetMilliseconds) || budgetMilliseconds < 0) throw refuseProcessV1("invalid","testing.command-budget-invalid");
  let credit = 0;
  return Object.freeze({
    advance: (wall: number, stretch: number): void => {
      if (!(stretch >= 1) || !Number.isFinite(stretch)) throw refuseProcessV1("invalid","testing.stretch-invalid");
      if (wall > 0) credit += wall / stretch;
    },
    creditMilliseconds: (): number => credit,
    expired: (): boolean => credit + EPSILON >= budgetMilliseconds,
    wallUntilExpiryMilliseconds: (stretch: number): number => Math.max(0, budgetMilliseconds - credit) * stretch,
  });
}

/** 🔬️ Measures how much slower a runnable thread of this process runs than on an idle host: wall time per CPU millisecond it consumed while it spun for `cpuMilliseconds` of CPU. */
export function spinSlowdownV1(cpuMilliseconds: number, wall: () => number = () => performance.now(), cpu: (previous?: NodeJS.CpuUsage) => NodeJS.CpuUsage = previous => process.cpuUsage(previous)): number {
  const started = wall(), origin = cpu();
  let used = 0;
  while (used < cpuMilliseconds * 1000) {
    const now = cpu(origin);
    used = now.user + now.system;
  }
  return Math.max(1, (wall() - started) / (used / 1000));
}

/** 🌍️ The contention of the host: the one-minute load average per core where the platform keeps one; where it does not (Windows reports zeros) a smoothed measurement of the scheduler, a thread's wall time per CPU millisecond, taken every few seconds. */
export function hostContentionV1(options: Readonly<{ policy?: ContentionPolicyV1; platform?: string; loadAverage?: () => readonly number[]; cores?: () => number; now?: () => number; slowdown?: (cpuMilliseconds: number) => number }> = {}): ContentionSourceV1 {
  const policy = options.policy ?? CONTENTION_POLICY_V1, platform = options.platform ?? osPlatform();
  const cores = options.cores ?? availableParallelism, load = options.loadAverage ?? loadavg, now = options.now ?? (() => performance.now()), slowdown = options.slowdown ?? ((milliseconds: number) => spinSlowdownV1(milliseconds));
  if (!policy.probe.platforms.includes(platform)) return () => {
    const count = Math.max(1, cores()), average = load()[0] ?? 0;
    return Object.freeze({ ratio: average / count, load: average, cores: count, source: "loadavg" as const });
  };
  let smoothed = 1, probedAt = Number.NEGATIVE_INFINITY;
  return () => {
    const at = now(), count = Math.max(1, cores());
    if (at - probedAt >= policy.probe.everyMilliseconds) {
      const weight = policy.probe.smoothingBasisPoints / 10000, measured = slowdown(policy.probe.cpuMilliseconds);
      smoothed = probedAt === Number.NEGATIVE_INFINITY ? measured : smoothed + weight * (measured - smoothed);
      probedAt = at;
    }
    return Object.freeze({ ratio: smoothed, load: smoothed * count, cores: count, source: "probe" as const });
  };
}

/** 🕰️ The wall clock: no host is ever busy, the budget counts exact wall time (for callers that need a fixed deadline). */
export const wallClockContentionV1: ContentionSourceV1 = () => Object.freeze({ ratio: 1, load: 0, cores: 1, source: "wall" as const });

let shared: ContentionSourceV1 | undefined;

/** 🧵️ The contention source of this process, kept so a probing platform smooths across all budgets of the process. */
export function sharedHostContentionV1(): ContentionSourceV1 {
  shared ??= hostContentionV1();
  return shared;
}

/** 👁️ What a running watch can answer at any moment. */
export type TestBudgetSnapshotV1 = Readonly<{ budgetMilliseconds: number; wallMilliseconds: number; usedMilliseconds: number; stretch: number; sample: ContentionSampleV1; ceilingMilliseconds: number }>;
/** 🛎️ The running budget of one test command. */
export type TestBudgetWatchV1 = Readonly<{ stop: () => void; snapshot: () => TestBudgetSnapshotV1 }>;
/** 🔧️ What a watch needs from its surroundings; every part has a real default, laws replace them with a simulated host. */
export type TestBudgetWatchOptionsV1 = Readonly<{
  policy?: ContentionPolicyV1;
  source?: ContentionSourceV1;
  now?: () => number;
  schedule?: (run: () => void, milliseconds: number) => unknown;
  cancel?: (handle: unknown) => void;
  expire: (snapshot: TestBudgetSnapshotV1) => void;
  report?: (line: string) => void;
}>;

const fill = (text: string, values: Readonly<Record<string, string | number>>): string => text.replace(/\$\{(\w+)\}/gu, (all, name: string) => (name in values ? String(values[name]) : all));
const seconds = (milliseconds: number): number => Math.round(milliseconds / 1000);
const one = (value: number): string => value.toFixed(1);

/** 🗣️ Words a snapshot in every language joined, because no language is a default. */
export function describeTestBudgetV1(kind: "stretched" | "expired", snapshot: TestBudgetSnapshotV1, policy: ContentionPolicyV1 = CONTENTION_POLICY_V1): string {
  const values = { load: one(snapshot.sample.load), cores: snapshot.sample.cores, budget: seconds(snapshot.budgetMilliseconds), stretch: one(snapshot.stretch), used: seconds(snapshot.usedMilliseconds), wall: seconds(snapshot.wallMilliseconds), ceiling: seconds(snapshot.ceilingMilliseconds) };
  return `[budget] ${fill(policy.messages[kind].en, values)} | ${fill(policy.messages[kind].de, values)}`;
}

/** ⏱️ Runs the budget of one test command: samples the host every tick, counts wall time at the stretch of the host, says so while it stretches and calls `expire` once when the work-equivalent budget is used up. */
export function watchTestBudgetV1(budgetMilliseconds: number, options: TestBudgetWatchOptionsV1): TestBudgetWatchV1 {
  const policy = options.policy ?? CONTENTION_POLICY_V1, source = options.source ?? sharedHostContentionV1(), now = options.now ?? (() => performance.now());
  const schedule = options.schedule ?? ((run: () => void, milliseconds: number) => setTimeout(run, milliseconds)), cancel = options.cancel ?? ((handle: unknown) => clearTimeout(handle as ReturnType<typeof setTimeout>));
  const clock = createBudgetClockV1(budgetMilliseconds), started = now(), step = policy.announce.stepBasisPoints / 10000, threshold = policy.announce.thresholdBasisPoints / 10000;
  const measure = (): ContentionSampleV1 => { try { return source(); } catch { return wallClockContentionV1(); } };
  let last = started, sample = measure(), stretch = budgetStretchV1(sample.ratio, policy), announced = 0, announcedAt = 0, handle: unknown, stopped = false;
  const snapshot = (): TestBudgetSnapshotV1 => Object.freeze({ budgetMilliseconds, wallMilliseconds: now() - started, usedMilliseconds: clock.creditMilliseconds(), stretch, sample, ceilingMilliseconds: budgetMilliseconds * policy.stretch.maximumBasisPoints / 10000 });
  const announce = (at: number): void => {
    if (stretch < threshold) { announced = 0; return; }
    if (announced !== 0 && Math.abs(stretch - announced) < step && at - announcedAt < policy.announce.everyMilliseconds) return;
    announced = stretch;
    announcedAt = at;
    options.report?.(describeTestBudgetV1("stretched", snapshot(), policy));
  };
  const arm = (): void => { handle = schedule(tick, Math.max(1, Math.min(policy.tickMilliseconds, Math.ceil(clock.wallUntilExpiryMilliseconds(stretch))))); };
  const tick = (): void => {
    if (stopped) return;
    const at = now();
    clock.advance(at - last, stretch);
    last = at;
    if (clock.expired()) { stopped = true; options.expire(snapshot()); return; }
    sample = measure();
    stretch = budgetStretchV1(sample.ratio, policy);
    announce(at);
    arm();
  };
  announce(started);
  arm();
  return Object.freeze({ stop: (): void => { stopped = true; if (handle !== undefined) cancel(handle); }, snapshot });
}
