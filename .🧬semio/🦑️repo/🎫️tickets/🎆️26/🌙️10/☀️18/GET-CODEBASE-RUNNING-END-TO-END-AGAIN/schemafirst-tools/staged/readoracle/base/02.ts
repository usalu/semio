import {spawnSync} from "node:child_process";
import {realpathSync, writeFileSync} from "node:fs";
import {join} from "node:path";
import {setTimeout as sleep} from "node:timers/promises";
import {splitPlanV1, type ReadPlanV1, type ReadRoundV1, type ReadRunV1} from "../🪢️policy/🟦️.ts";
import {captureV1, readOracleBothV1} from "../🏃️run/🟦️.ts";

const quoted = (text: string): string => `"${text.replaceAll("\\", "\\\\").replaceAll('"', '\\"')}"`;
const escaped = (text: string): string => text.replace(/[.^$*+?()[\]{}|\\]/gu, "\\$&");

/** 🍎️ The Seatbelt profile of a plan: every workspace file read is refused, then the rules of the plan in order (the last matching rule decides, so a refusal inside a granted tree and a grant inside a refused tree are one rule each), then the ambient trees and the named files, and last the directories, which stay listable (names are not contents). The compiled size depends on the set of paths, not on the rule form, so the cover is what keeps it small. */
export function seatbeltProfileV1(plan: ReadPlanV1): string {
  const lines = ["(version 1)", "(allow default)", `(deny file-read-data (subpath ${quoted(plan.root)}))`];
  for (const rule of plan.rules) lines.push(`(${rule.effect} file-read-data (${rule.kind} ${quoted(join(plan.root, rule.path))}))`);
  for (const tree of plan.ambient) lines.push(`(allow file-read-data (subpath ${quoted(join(plan.root, tree))}))`);
  if (plan.names.length) lines.push(`(allow file-read-data (regex #"^${escaped(plan.root)}/(.*/)?(${plan.names.map(escaped).join("|")})$"))`);
  lines.push("(allow file-read-data (vnode-type DIRECTORY))");
  return `${lines.join("\n")}\n`;
}

/** 🧾️ The workspace relative paths of the `deny(1) file-read-data` lines of the unified log that lie inside the root. */
export function seatbeltDenialsV1(log: string, root: string): string[] {
  const seen = new Set<string>(), marker = "deny(1) file-read-data ";
  for (const line of log.split("\n")) {
    const at = line.indexOf(marker);
    if (at < 0) continue;
    const path = line.slice(at + marker.length).trim();
    if (path.startsWith(`${root}/`)) seen.add(path.slice(root.length + 1));
  }
  return [...seen].sort();
}

/** 📜️ Reads the denials of the last seconds from the unified log (the kernel logs every enforced denial; report mode would be rate limited). */
export function readSeatbeltDenialsV1(since: Date, root: string): string[] {
  return seatbeltDenialsV1(readSeatbeltLogV1(since, root), root);
}

/** 🗒️ The raw `deny(1) file-read-data` lines of the unified log since a moment that mention a path. */
export function readSeatbeltLogV1(since: Date, root: string): string {
  const seconds = Math.max(1, Math.ceil((Date.now() - since.getTime()) / 1000) + 1);
  const done = spawnSync("/usr/bin/log", ["show", "--last", `${seconds}s`, "--style", "compact", "--predicate", `eventMessage CONTAINS "deny(1) file-read-data" AND eventMessage CONTAINS ${quoted(root)}`], {encoding: "utf8", maxBuffer: 256 * 1024 * 1024});
  return done.stdout ?? "";
}

/** 🔌️ Whether the Seatbelt launcher works for this user: the tool exists and accepts a trivial profile. */
export function seatbeltAvailableV1(platform: NodeJS.Platform = process.platform): {available: boolean; reason?: string} {
  if (platform !== "darwin") return {available: false, reason: readOracleBothV1("seatbeltPlatform", {platform}, " / ")};
  const probe = spawnSync("sandbox-exec", ["-p", "(version 1)(allow default)", "/usr/bin/true"], {encoding: "utf8"});
  return probe.status === 0 ? {available: true} : {available: false, reason: readOracleBothV1("seatbeltProbe", {reason: (probe.stderr || String(probe.error ?? "")).trim().slice(0, 200)}, " / ")};
}

/** 🚧️ The kernel refused to compile the profile (too many rules) or the plan needs more runs than allowed: the caller falls back to a backend without a profile limit. */
export class SeatbeltProfileRefusedV1 extends Error {
  constructor(reason: string) {
    super(reason);
    this.name = "SeatbeltProfileRefusedV1";
  }
}

/** ⛔️ Whether a run ended because the kernel refused to compile the profile, not because the command failed: the compiler reports a profile over 65,535 bytes (`sandbox-exec: data object length 75339 exceeds maximum (65535)`) or, for a larger one, aborts on an internal limit (`Assertion failed: (diff <= INSTR_JUMP_NE_MAX_LENGTH), file serialize.c`). */
export function seatbeltRefusalV1(exit: number, output: string): boolean {
  return exit !== 0 && /^(?:sandbox-exec: [^\n]*(?:exceeds maximum|invalid|syntax error|parse)|Assertion failed: [^\n]*serialize\.c)/iu.test(output.trimStart());
}

/** 💰️ How many rules one profile may hold (`capacity`, lowered whenever the kernel refuses a profile and kept for the next rounds) and how many runs a round may be split into before the caller should use another backend. */
export type SeatbeltBudgetV1 = {capacity: number; readonly maxRuns: number};

/** 🧱️ The rules of a profile that the kernel compiled in practice for repository paths (about 800 literal rules of 130 bytes each fit); the first attempt starts below it and the budget adapts. */
export const seatbeltCapacityV1 = 600;

const SMALLEST_CAPACITY = 24;

/** 🚏️ Waits until the unified log has flushed everything up to now: a file of the scratch directory is read under a profile that refuses it, and the log is polled until that denial shows (a fixed pause is too short on a loaded host and would report a missed denial as a clean run). Returns whether the marker showed before the limit. */
export async function awaitLogFlushV1(scratch: string, since: Date, run: Pick<ReadRunV1, "env" | "signal">, limitMilliseconds = 60000): Promise<boolean> {
  const directory = realpathSync(scratch), marker = join(directory, `marker-${process.pid}-${Date.now()}-${Math.random().toString(36).slice(2)}`), profile = `${marker}.sb`;
  writeFileSync(marker, "marker");
  writeFileSync(profile, `(version 1)(allow default)(deny file-read-data (literal ${quoted(marker)}))`);
  await captureV1(["sandbox-exec", "-f", profile, "/bin/cat", marker], {cwd: directory, env: run.env, timeoutMs: 30000, signal: run.signal});
  const deadline = Date.now() + limitMilliseconds;
  for (;;) {
    if (seatbeltDenialsV1(readSeatbeltLogV1(since, marker), directory).includes(marker.slice(directory.length + 1))) return true;
    if (Date.now() >= deadline || run.signal?.aborted) return false;
    await sleep(300);
  }
}

/** 🧪️ Runs the command once under one profile and reads the enforced denials back from the unified log (which needs a moment to flush). */
async function runProfileV1(plan: ReadPlanV1, run: ReadRunV1, index: number): Promise<ReadRoundV1> {
  const profile = join(run.scratch, `profile-${run.round}-${index}.sb`), since = new Date(Date.now() - 2000);
  writeFileSync(profile, seatbeltProfileV1(plan));
  const done = await captureV1(["sandbox-exec", "-f", profile, ...run.command], {cwd: join(plan.root, run.cwd), env: run.env, timeoutMs: run.timeoutMs, signal: run.signal});
  if (seatbeltRefusalV1(done.exit, done.output)) throw new SeatbeltProfileRefusedV1(done.output.trim().slice(0, 300));
  const flushed = await awaitLogFlushV1(run.scratch, since, run);
  return {exit: done.exit, tail: done.output.slice(-1500), seconds: done.seconds, denied: readSeatbeltDenialsV1(since, plan.root), notes: flushed ? [] : ["log:unflushed"]};
}

/** 🔬️ Compiles the largest profile of a split with a command that does nothing, so a refusal costs no command run. */
async function compileProfileV1(plans: readonly ReadPlanV1[], run: ReadRunV1): Promise<void> {
  const largest = plans.reduce((left, right) => (Buffer.byteLength(seatbeltProfileV1(right)) > Buffer.byteLength(seatbeltProfileV1(left)) ? right : left));
  const probe = join(run.scratch, `probe-${run.round}.sb`);
  writeFileSync(probe, seatbeltProfileV1(largest));
  const done = await captureV1(["sandbox-exec", "-f", probe, "/usr/bin/true"], {cwd: largest.root, env: run.env, timeoutMs: run.timeoutMs, signal: run.signal});
  if (seatbeltRefusalV1(done.exit, done.output)) throw new SeatbeltProfileRefusedV1(done.output.trim().slice(0, 300));
}

/** 🍏️ Runs the command under the plan. A plan that does not fit one profile is split into runs that each carry every grant and a share of the refusals (an undeclared file is refused in the run that holds its refusal, so the union of the runs is as strict as the whole plan; the command reads the same files in each run until it hits a refusal, so a run without a denial proves the command did not read the refused files). The kernel refusing a profile lowers the capacity of the budget and splits again; a plan that needs more than `maxRuns` runs, or whose grants alone exceed the capacity, is refused so the caller can use another backend. */
export async function runSeatbeltV1(plan: ReadPlanV1, run: ReadRunV1, budget: SeatbeltBudgetV1): Promise<ReadRoundV1> {
  for (;;) {
    const runs = splitPlanV1(plan, budget.capacity);
    if (!runs || runs.length > budget.maxRuns) throw new SeatbeltProfileRefusedV1(`the plan needs ${runs ? `${runs.length} profiles of at most ${budget.capacity} rules (limit ${budget.maxRuns})` : `more than ${budget.capacity} grants in every profile`}`);
    try {
      if (runs.length > 1 || plan.rules.length * 2 > budget.capacity) await compileProfileV1(runs, run);
      const rounds: ReadRoundV1[] = [];
      for (const [index, tile] of runs.entries()) {
        rounds.push(await runProfileV1(tile, run, index));
        if (run.signal?.aborted) break;
      }
      const failed = rounds.find((round) => round.exit !== 0);
      return {exit: failed?.exit ?? 0, tail: (failed ?? rounds.at(-1)!).tail, seconds: Math.round(rounds.reduce((sum, round) => sum + round.seconds, 0) * 10) / 10, denied: [...new Set(rounds.flatMap((round) => round.denied))].sort(), notes: [`profiles:${runs.length}`, `rules:${plan.rules.length}`, `capacity:${budget.capacity}`, ...new Set(rounds.flatMap((round) => round.notes))]};
    } catch (error) {
      if (!(error instanceof SeatbeltProfileRefusedV1) || budget.capacity <= SMALLEST_CAPACITY) throw error;
      budget.capacity = Math.max(SMALLEST_CAPACITY, Math.floor(budget.capacity * 0.7));
    }
  }
}
