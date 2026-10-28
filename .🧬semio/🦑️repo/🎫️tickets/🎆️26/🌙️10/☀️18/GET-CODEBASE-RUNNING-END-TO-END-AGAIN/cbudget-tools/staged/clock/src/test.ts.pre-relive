import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import Ajv from "ajv";
import { runBudgetedTestCommand } from "../../../🎛️execution/🟦️.ts";
import { budgetStretchV1, admitContentionPolicyV1, CONTENTION_POLICY_V1, createBudgetClockV1, describeTestBudgetV1, hostContentionV1, spinSlowdownV1, watchTestBudgetV1, wallClockContentionV1, type ContentionSampleV1 } from "../🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";

/** ⏲️ Laws of the load-aware test budget clock: a budget counts work, so a run that fits it on an idle host survives any load up to the stretch bound and a run that never ends is stopped after at most that bound; an independent Python integration of the same traces agrees to the millisecond. */
const policy = CONTENTION_POLICY_V1;
const tick = policy.tickMilliseconds, cap = policy.stretch.maximumBasisPoints / 10000;
const artifactParent = process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir();

type Trace = readonly (readonly [number, number])[];
type ClockCase = Readonly<{ budget: number; work: number; cores: number; trace: Trace }>;

const oracle = (clocks: readonly ClockCase[], probes: readonly unknown[] = []): { clocks: { expiry: number; finish: number; survives: boolean }[]; probes: number[][] } => {
  const run = spawnSync("python3", ["-I", join(import.meta.dir, "🐍️oracle", "🐍️.py")], { input: JSON.stringify({ policy, clocks, probes }), encoding: "utf8" });
  expect(run.status, run.stderr).toBe(0);
  return JSON.parse(run.stdout);
};

const simulate = (row: ClockCase, options: Readonly<{ horizon?: number; report?: (line: string) => void }> = {}): number | undefined => {
  let time = 0, sequence = 0, expiredAt: number | undefined;
  const queue: { at: number; run: () => void; id: number }[] = [], rows = [...row.trace].sort((a, b) => a[0] - b[0]);
  const loadAt = (at: number): number => rows.filter(entry => entry[0] <= at).at(-1)![1];
  const watch = watchTestBudgetV1(row.budget, {
    source: (): ContentionSampleV1 => { const load = loadAt(time); return { ratio: load / row.cores, load, cores: row.cores, source: "loadavg" }; },
    now: () => time,
    schedule: (run, milliseconds) => { const entry = { at: time + milliseconds, run, id: sequence++ }; queue.push(entry); return entry; },
    cancel: handle => { const at = queue.indexOf(handle as never); if (at >= 0) queue.splice(at, 1); },
    expire: () => { expiredAt = time; },
    ...(options.report ? { report: options.report } : {}),
  });
  const horizon = options.horizon ?? row.budget * cap * 2 + tick;
  while (expiredAt === undefined) {
    queue.sort((a, b) => a.at - b.at || a.id - b.id);
    const next = queue.shift();
    if (!next || next.at > horizon) break;
    time = next.at;
    next.run();
  }
  watch.stop();
  return expiredAt;
};

test("the policy is closed under its schema and refuses what the schema refuses (third-party Ajv agrees)", () => {
  const ajv = new Ajv({ strict: true }), validate = ajv.compile(schema);
  expect(validate(policy), JSON.stringify(validate.errors)).toBe(true);
  expect(admitContentionPolicyV1(JSON.parse(JSON.stringify(policy)))).toEqual(policy);
  const broken: unknown[] = [
    { ...policy, extra: 1 },
    { ...policy, tickMilliseconds: 0 },
    { ...policy, stretch: { minimumBasisPoints: 5000, maximumBasisPoints: 80000 } },
    { ...policy, stretch: { ...policy.stretch, maximumBasisPoints: 9999 } },
    { ...policy, probe: { ...policy.probe, cpuMilliseconds: 1 } },
    { ...policy, messages: { ...policy.messages, expired: { en: "only english" } } },
    { ...policy, messages: { stretched: policy.messages.stretched } },
  ];
  for (const value of broken) {
    expect(validate(value), JSON.stringify(value)).toBe(false);
    expect(() => admitContentionPolicyV1(value), JSON.stringify(value)).toThrow("Closed test budget contention policy required");
  }
  expect(policy.stretch.maximumBasisPoints).toBeGreaterThanOrEqual(70000);
});

test("the stretch is the contention ratio clamped to the policy bounds", () => {
  for (const row of fixture.ratios) expect(budgetStretchV1(row.ratio as number, policy), String(row.ratio)).toBe(row.stretch);
  expect(budgetStretchV1(Number.NaN)).toBe(1);
  expect(budgetStretchV1(Number.POSITIVE_INFINITY)).toBe(1);
});

test("the clock counts wall time at 1/stretch, expires on the work-equivalent budget and refuses nonsense", () => {
  const clock = createBudgetClockV1(1000);
  clock.advance(500, 1);
  clock.advance(500, 5);
  expect(clock.creditMilliseconds()).toBeCloseTo(600, 9);
  expect(clock.expired()).toBe(false);
  expect(clock.wallUntilExpiryMilliseconds(2)).toBeCloseTo(800, 9);
  clock.advance(2000, 5);
  expect(clock.expired()).toBe(true);
  expect(clock.wallUntilExpiryMilliseconds(8)).toBe(0);
  expect(() => createBudgetClockV1(-1)).toThrow("Invalid test command budget");
  expect(() => createBudgetClockV1(Number.NaN)).toThrow("Invalid test command budget");
  expect(() => clock.advance(1, 0.5)).toThrow("Stretch of at least one required");
  expect(() => clock.advance(1, Number.NaN)).toThrow("Stretch of at least one required");
});

test("every authored host trace expires and survives exactly when the independent integration says", () => {
  const rows = fixture.clocks as unknown as (ClockCase & { name: string; expiry: number; survives: boolean })[], answers = oracle(rows).clocks;
  rows.forEach((row, index) => {
    expect(answers[index]!.expiry, row.name).toBeCloseTo(row.expiry, 6);
    expect(answers[index]!.survives, row.name).toBe(row.survives);
    const simulated = simulate(row);
    expect(simulated, row.name).toBeDefined();
    expect(simulated! - row.expiry, `${row.name}: ${simulated} against ${row.expiry}`).toBeGreaterThan(-1e-3);
    expect(simulated! - row.expiry, `${row.name}: ${simulated} against ${row.expiry}`).toBeLessThan(1 + 1e-3);
  });
});

test("on 400 random host traces the stepped clock equals the exact integration; a replay that fits its budget survives, a run past the bound does not", () => {
  let seed = 20261027;
  const next = (bound: number): number => { seed = (seed * 1103515245 + 12345) % 2147483648; return Math.floor(seed / 256) % bound; };
  const cases: ClockCase[] = [];
  for (let round = 0; round < 400; round++) {
    const cores = 1 + next(16), budget = tick * (3 + next(300)), segments = 1 + next(5), trace: [number, number][] = [[0, next(Math.floor(cores * 12))]];
    for (let segment = 1; segment < segments; segment++) trace.push([tick * (segment * 7 + next(7)), next(Math.floor(cores * 12))]);
    cases.push({ budget, work: Math.floor(budget * (50 + next(50)) / 100), cores, trace: trace.sort((a, b) => a[0] - b[0]) });
  }
  const answers = oracle(cases).clocks;
  let survivors = 0, killed = 0;
  cases.forEach((row, index) => {
    const answer = answers[index]!, simulated = simulate(row);
    expect(simulated, JSON.stringify(row)).toBeDefined();
    expect(simulated! - answer.expiry, `timers are whole milliseconds, so the stop comes at most one later: ${JSON.stringify(row)}`).toBeGreaterThan(-1e-3);
    expect(simulated! - answer.expiry, JSON.stringify(row)).toBeLessThan(1 + 1e-3);
    expect(answer.expiry, "never earlier than the wall budget").toBeGreaterThanOrEqual(row.budget - 1e-6);
    expect(answer.expiry, "never later than the stretch bound").toBeLessThanOrEqual(row.budget * cap + 1e-6);
    if (Math.max(...row.trace.map(entry => entry[1] / row.cores)) <= cap) { expect(answer.survives, "a run that fits its budget on an idle host survives a host up to the bound").toBe(true); survivors++; } else if (!answer.survives) killed++;
  });
  expect(survivors).toBeGreaterThan(50);
  expect(killed).toBeGreaterThan(5);
});

test("the load can change between two ticks: the error stays inside one tick per change", () => {
  let seed = 7;
  const next = (bound: number): number => { seed = (seed * 1103515245 + 12345) % 2147483648; return Math.floor(seed / 256) % bound; };
  const cases: ClockCase[] = [];
  for (let round = 0; round < 120; round++) {
    const cores = 2 + next(8), trace: [number, number][] = [[0, next(cores * 10)]];
    for (let segment = 1; segment < 4; segment++) trace.push([segment * 20000 + next(tick * 10), next(cores * 10)]);
    cases.push({ budget: 90000 + next(60) * tick, work: 1, cores, trace });
  }
  const answers = oracle(cases).clocks;
  cases.forEach((row, index) => {
    const simulated = simulate(row);
    expect(Math.abs(simulated! - answers[index]!.expiry), JSON.stringify(row)).toBeLessThanOrEqual(3 * tick * cap);
  });
});

test("a host that cannot be measured counts the wall clock", () => {
  const explode = (): ContentionSampleV1 => { throw Error("no load"); };
  let time = 0, expiredAt = -1;
  const queue: { at: number; run: () => void }[] = [];
  watchTestBudgetV1(2500, { source: explode, now: () => time, schedule: (run, milliseconds) => { queue.push({ at: time + milliseconds, run }); return undefined; }, cancel: () => undefined, expire: () => { expiredAt = time; } });
  while (expiredAt < 0 && queue.length) { queue.sort((a, b) => a.at - b.at); const entry = queue.shift()!; time = entry.at; entry.run(); }
  expect(expiredAt).toBe(2500);
  expect(wallClockContentionV1().ratio).toBe(1);
});

test("a stopped watch never expires and cancels its timer", () => {
  let time = 0, cancelled = 0, fired = 0;
  const queue: { at: number; run: () => void }[] = [];
  const watch = watchTestBudgetV1(5000, { source: wallClockContentionV1, now: () => time, schedule: (run, milliseconds) => { const entry = { at: time + milliseconds, run }; queue.push(entry); return entry; }, cancel: () => { cancelled++; }, expire: () => { fired++; } });
  time = 1000;
  queue.shift()!.run();
  watch.stop();
  expect(cancelled).toBe(1);
  time = 9000;
  queue.shift()!.run();
  expect(fired).toBe(0);
  expect(watch.snapshot().ceilingMilliseconds).toBe(5000 * cap);
});

test("a busy host is announced once in every language, again when it moves or after the interval, and an idle host says nothing", () => {
  const idle: string[] = [], busy: string[] = [];
  expect(simulate({ budget: 120000, work: 1, cores: 10, trace: [[0, 6]] }, { report: line => idle.push(line) })).toBe(120000);
  expect(idle).toEqual([]);
  simulate({ budget: 300000, work: 1, cores: 10, trace: [[0, 50], [90000, 51], [100000, 80], [250000, 5]] }, { report: line => busy.push(line) });
  expect(busy.length).toBeGreaterThanOrEqual(4);
  expect(busy[0]).toContain("Busy host: load 50.0 on 10 cores");
  expect(busy[0]).toContain("Ausgelasteter Rechner: Last 50.0 auf 10 Kernen");
  expect(busy[0]).toContain("1/5.0");
  expect(busy.some(line => line.includes("load 80.0"))).toBe(true);
  expect(busy.filter(line => line.includes("load 50.0")).length).toBeGreaterThanOrEqual(2);
  expect(busy.every(line => line.startsWith("[budget] ") && line.includes(" | "))).toBe(true);
  const words = describeTestBudgetV1("expired", { budgetMilliseconds: 300000, wallMilliseconds: 1500000, usedMilliseconds: 300000, stretch: 5, sample: { ratio: 5, load: 50, cores: 10, source: "loadavg" }, ceilingMilliseconds: 2400000 });
  expect(words).toBe("[budget] Budget of 300 s used up after 1500 s on the wall clock (host stretch 5.0, 300 s counted). | Budget von 300 s nach 1500 s Wanduhrzeit aufgebraucht (Rechnerdehnung 5.0, 300 s gezählt).");
});

test("platforms with a load average use it per core, Windows measures the scheduler and smooths it like the independent model", () => {
  const trap = (): number => { throw Error("a platform with a load average never spins"); };
  const unix = hostContentionV1({ platform: "linux", loadAverage: () => [50, 40, 30], cores: () => 10, slowdown: trap });
  expect(unix()).toEqual({ ratio: 5, load: 50, cores: 10, source: "loadavg" });
  expect(hostContentionV1({ platform: "darwin", loadAverage: () => [3, 0, 0], cores: () => 0, slowdown: trap })().ratio).toBe(3);
  for (const row of fixture.probes) {
    let at = 0, index = 0;
    const measured = row.steps.map(step => step[1] as number), source = hostContentionV1({ platform: "win32", loadAverage: () => [0, 0, 0], cores: () => 8, now: () => at, slowdown: () => measured[index]! });
    const answers = row.steps.map((step, position) => { at = step[0] as number; index = position; return source().ratio; });
    oracle([], [row]).probes[0]!.forEach((value, position) => expect(answers[position], `model step ${position}`).toBeCloseTo(value, 9));
    row.answers.forEach((value, position) => expect(answers[position]).toBeCloseTo(value, 9));
    expect(source().source).toBe("probe");
    expect(source().load).toBeCloseTo(source().ratio * 8, 9);
  }
});

test("the scheduler probe divides wall time by the CPU it consumed, on a simulated and on the real host", () => {
  const state = { cpu: 0, wall: 0 };
  const cpu = (previous?: NodeJS.CpuUsage): NodeJS.CpuUsage => { state.cpu += 1000; state.wall += 5; return { user: previous ? state.cpu - previous.user : state.cpu, system: 0 }; };
  expect(spinSlowdownV1(10, () => state.wall, cpu)).toBeCloseTo(5.5, 9);
  const real = spinSlowdownV1(8);
  expect(Number.isFinite(real)).toBe(true);
  expect(real).toBeGreaterThanOrEqual(1);
});

const measuredHost = (ratio: number) => (): ContentionSampleV1 => ({ ratio, load: ratio * 10, cores: 10, source: "loadavg" });

test("a process that needs more wall time than its budget survives a busy host and is killed on an idle one", async () => {
  mkdirSync(artifactParent, { recursive: true });
  const directory = mkdtempSync(join(artifactParent, "budget-clock-"));
  const lines: string[] = [], original = console.error;
  console.error = (...values: unknown[]) => { lines.push(values.map(String).join(" ")); };
  try {
    const program = "setTimeout(()=>{},4000)";
    await runBudgetedTestCommand("node", ["-e", program], { cwd: directory, budgetMs: 2000, throwOnFailure: true, contention: measuredHost(4) });
    expect(lines.some(line => line.includes("Busy host") && line.includes("Ausgelasteter Rechner"))).toBe(true);
    lines.length = 0;
    await expect(runBudgetedTestCommand("node", ["-e", program], { cwd: directory, budgetMs: 2000, throwOnFailure: true, contention: wallClockContentionV1 })).rejects.toThrow("exceeded its 2000ms budget");
    expect(lines.some(line => line.includes("Busy host"))).toBe(false);
    expect(lines.some(line => line.includes("exceeded 2000ms — killed") && line.includes("Budget of 2 s used up after") && line.includes("Budget von 2 s nach"))).toBe(true);
  } finally {
    console.error = original;
    rmSync(directory, { recursive: true, force: true });
  }
}, 120000);

test("a hung process on a busy host is still killed, after its budget times the stretch and never later than the bound", async () => {
  mkdirSync(artifactParent, { recursive: true });
  const directory = mkdtempSync(join(artifactParent, "budget-hung-")), original = console.error;
  console.error = () => undefined;
  try {
    for (const [ratio, budget, lower, upper] of [[3, 600, 1700, 12000], [1000, 300, 2300, 14000]] as const) {
      const started = performance.now();
      await expect(runBudgetedTestCommand("node", ["-e", "setInterval(()=>{},1000)"], { cwd: directory, budgetMs: budget, throwOnFailure: true, contention: measuredHost(ratio) })).rejects.toThrow(`exceeded its ${budget}ms budget`);
      const elapsed = performance.now() - started;
      expect(elapsed, `ratio ${ratio}`).toBeGreaterThanOrEqual(lower);
      expect(elapsed, `ratio ${ratio}`).toBeLessThan(upper);
    }
  } finally {
    console.error = original;
    rmSync(directory, { recursive: true, force: true });
  }
}, 120000);
