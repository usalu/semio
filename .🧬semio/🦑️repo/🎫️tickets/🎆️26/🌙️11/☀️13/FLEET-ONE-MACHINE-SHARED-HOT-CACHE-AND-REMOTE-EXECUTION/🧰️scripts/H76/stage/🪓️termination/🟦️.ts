import { spawnSync } from "node:child_process";
import { appendFileSync, mkdirSync } from "node:fs";
import { dirname } from "node:path";
import {ownProcessIdentityV1, parsePosixProcessStartV1, processPidExistsV1} from "../🔖️identity/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

/** 📐️ The declared grace of a termination.
 * @see ./🧬️schema/🔣️.json */
export const TERMINATION_POLICY_V1 = schema.definitions.Policy.const;

/** 🌳️ Captures the first-party ownership state needed to terminate a live child. */
export interface OwnedChildHandle { readonly pid?: number; readonly exitCode: number | null; readonly signalCode: string | null }

/** 🌳️ One process of the table: pid, parent, group, depth below the root and the creation time that names it together with the pid (0: not readable). */
export interface OwnedProcessRowV1 {
  readonly pid: number;
  readonly parent: number;
  readonly group: number;
  readonly depth: number;
  readonly startMs: number;
}
type OwnedProcessRow = OwnedProcessRowV1;

/** 🔌️ The host facts of a termination: the platform and the synchronous process start, replaced by a test to run the Windows branch against a `taskkill` fixture on any host. */
export interface TerminationHostV1 { readonly platform?: string; readonly spawnSync?: typeof spawnSync; readonly record?: (row: Readonly<{ pid: number; ppid?: number; group?: number; startMs?: number }>, signal: "SIGINT" | "SIGTERM" | "SIGKILL") => void; readonly table?: () => readonly ProcessTableRowV1[] | undefined; readonly rootStartMs?: number }

/** 🪓️Terminates one owned subprocess tree, including descendant-created POSIX groups, before ancestry is lost. On Windows `taskkill /T /F` ends the tree (exit 128 = the process is already gone); any other failure ends at least the root. See https://learn.microsoft.com/windows-server/administration/windows-commands/taskkill */
export function terminateOwnedProcessTree(rootPid: number, host: TerminationHostV1 = {}): void {
  if (!Number.isSafeInteger(rootPid) || rootPid <= 0) return;
  if ((host.platform ?? process.platform) === "win32") {
    let status: number | null = null;
    try { status = (host.spawnSync ?? spawnSync)("taskkill", ["/pid", String(rootPid), "/T", "/F"], { stdio: "ignore", windowsHide: true }).status; } catch {}
    if (status !== 0 && status !== 128) try { process.kill(rootPid, "SIGKILL"); } catch {}
    return;
  }
  const owned = ownedRowsV1(rootPid, host.rootStartMs, host.table);
  if (owned === undefined) {
    if (host.rootStartMs !== undefined && host.rootStartMs > 0) return;
    try { process.kill(-rootPid, "SIGKILL"); } catch {}
    try { host.record?.({ pid: rootPid }, "SIGKILL"); } catch {}
    try { process.kill(rootPid, "SIGKILL"); } catch {}
    return;
  }
  signalOwnedRowsV1(owned, "SIGKILL", host.record, host.table);
}

/** 🧾️ One row of the process table: pid, parent, group and creation time in ms (0 when `ps` did not print a readable one). */
export type ProcessTableRowV1 = Readonly<{ pid: number; parent: number; group: number; startMs: number }>;

/** 📸️ The process table now with the creation time of every process, `undefined` when it cannot be read (Windows, no `ps`). One `ps` call: the creation time is what tells a process from a later process that reuses its pid.
 * @see https://pubs.opengroup.org/onlinepubs/9799919799/utilities/ps.html */
export function readProcessTableV1(): readonly ProcessTableRowV1[] | undefined {
  if (process.platform === "win32") return undefined;
  const snapshot = spawnSync("ps", ["-axo", "pid=,ppid=,pgid=,lstart="], { encoding: "utf8", windowsHide: true, env: { ...process.env, LC_ALL: "C", TZ: "UTC0" } });
  if (snapshot.status !== 0 || typeof snapshot.stdout !== "string") return undefined;
  return snapshot.stdout.split("\n").flatMap((line): ProcessTableRowV1[] => {
    const found = /^\s*(\d+)\s+(\d+)\s+(\d+)\s+(.*?)\s*$/u.exec(line);
    if (found === null) return [];
    const pid = Number(found[1]), parent = Number(found[2]), group = Number(found[3]);
    return pid > 0 && parent > 0 && group > 0 ? [{ pid, parent, group, startMs: parsePosixProcessStartV1(found[4]!) ?? 0 }] : [];
  });
}

/** 🌳️ The processes of the owned tree of `rootPid` now: its descendants and every member of its own process group (an orphan that was reparented keeps the group); pid, parent, group, depth and creation time. With `rootStartMs` the root is the process of that creation time only (a reused pid owns nothing and no row is invented for a root that is gone); without it the caller holds the live child and a missing root row is invented. `undefined` when the process table cannot be read. */
export function ownedRowsV1(rootPid: number, rootStartMs?: number, read: () => readonly ProcessTableRowV1[] | undefined = readProcessTableV1): OwnedProcessRow[] | undefined {
  const rows = read();
  if (rows === undefined) return undefined;
  const pinned = rootStartMs !== undefined && rootStartMs > 0, rootRow = rows.find((row) => row.pid === rootPid);
  if (pinned && rootRow !== undefined && Math.abs(rootRow.startMs - rootStartMs) > 1500) return [];
  const depth = new Map<number, number>();
  if (rootRow !== undefined) depth.set(rootPid, 0);
  for (let changed = true; changed;) {
    changed = false;
    for (const row of rows) {
      if (depth.has(row.pid)) continue;
      const parentDepth = depth.get(row.parent);
      if (parentDepth !== undefined) { depth.set(row.pid, parentDepth + 1); changed = true; }
      else if (row.group === rootPid && row.pid !== rootPid) { depth.set(row.pid, 1); changed = true; }
    }
  }
  const owned: OwnedProcessRow[] = rows.filter((row) => depth.has(row.pid)).map((row) => ({ ...row, depth: depth.get(row.pid)! }));
  if (rootRow === undefined && !pinned) owned.push({ pid: rootPid, parent: 0, group: rootPid, depth: 0, startMs: 0 });
  return owned;
}

/** 🛡️ The owned rows that are still the same process now: a row whose pid is gone or now belongs to a process of another creation time is dropped, so a pid reused between the snapshot and the signal is never signalled. `fresh` is a table read right before the signal; without one (no `ps`) every row stays. */
export function verifiedOwnedRowsV1(owned: readonly OwnedProcessRow[], fresh: readonly ProcessTableRowV1[] | undefined): OwnedProcessRow[] {
  if (fresh === undefined) return [...owned];
  const starts = new Map(fresh.map((row) => [row.pid, row.startMs]));
  return owned.filter((row) => { const seen = starts.get(row.pid); return seen !== undefined && (row.startMs === 0 || seen === 0 || Math.abs(seen - row.startMs) <= 1500); });
}

/** 📨️ Sends `signal` to the verified owned rows: the process groups that consist of owned processes only (deepest first), then each process, announcing each process with its identity through `record` before. A group that holds any process outside the owned set is never signalled as a group, its owned members get the signal one by one. */
function signalOwnedRowsV1(owned: OwnedProcessRow[], signal: "SIGINT" | "SIGTERM" | "SIGKILL", record?: TerminationHostV1["record"], table: () => readonly ProcessTableRowV1[] | undefined = readProcessTableV1): void {
  const fresh = table(), verified = verifiedOwnedRowsV1(owned, fresh);
  for (const row of verified) try { record?.({ pid: row.pid, ppid: row.parent, group: row.group, startMs: row.startMs }, signal); } catch {}
  if (fresh !== undefined) {
    const members = new Set(verified.map((row) => row.pid)), groups = new Map<number, number>();
    for (const row of verified) {
      if (groups.has(row.group)) { groups.set(row.group, Math.max(groups.get(row.group)!, row.depth)); continue; }
      if (fresh.every((other) => other.group !== row.group || members.has(other.pid))) groups.set(row.group, row.depth);
    }
    for (const [group] of [...groups].sort((left, right) => right[1] - left[1])) {
      if (fresh.some((other) => other.group === group && !members.has(other.pid))) continue;
      try { process.kill(-group, signal); } catch {}
    }
  }
  for (const row of [...verified].sort((left, right) => right.depth - left.depth)) {
    try { process.kill(row.pid, signal); } catch {}
  }
}

/** ⏹️ Terminates one still-running spawned child with its whole descendant tree — POSIX groups and `taskkill /T`
 * on Windows through [[terminateOwnedProcessTree]] — and never signals a pid its exited child no longer owns. */
export function terminateOwnedChildTree(child: OwnedChildHandle, host: TerminationHostV1 = {}): void {
  if (child.exitCode === null && child.signalCode === null && child.pid) terminateOwnedProcessTree(child.pid, host);
}


/** 🗣️ The record of one signal in English and German, `<English> / <German>`: the victim, the signal, the killing process, the command and the reason, and when both creation times are given the identity of victim and sender (`startMs`, `killerStartMs`, ms since the epoch). A pure function.
 * @see ./🧪️tests/🐍️.py */
export function ownedKillRecordLineV1(input: Readonly<{ pid: number; signal: "SIGINT" | "SIGTERM" | "SIGKILL"; killer: number; label: string; reason: string; startMs?: number; killerStartMs?: number }>): string {
  const identified = (input.startMs ?? 0) > 0 && (input.killerStartMs ?? 0) > 0;
  const fill = (language: "en" | "de"): string => {
    const texts = schema.definitions.Texts.const[language];
    return (identified ? `${texts["kill-record"]} ${texts["kill-identity"]}` : texts["kill-record"]).replace(/\{(\w+)\}/gu, (_all, name: string) => String((input as Record<string, string | number>)[name] ?? ""));
  };
  return `${fill("en")} / ${fill("de")}`;
}

/** 📒️ A writer that appends each record line to the ledger file `file` as `<ISO time> <line>`, the layout of the ready stage runner's `kills.log`, so every sender of a signal can be read back from one kind of file. A ledger that cannot be written never stops the signal. */
export function ownedKillLedgerWriterV1(file: string, now: () => number = Date.now): (line: string) => void {
  return (line) => { try { mkdirSync(dirname(file), { recursive: true }); appendFileSync(file, `${new Date(now()).toISOString()} ${line}\n`); } catch {} };
}

/** 🧾️ The recorder of one owned command: every signal it sends is written as [[ownedKillRecordLineV1]] with the identity of victim and sender and the reason that holds at the time of the signal. `write` defaults to stderr. */
export function ownedKillRecorderV1(label: string, reason: () => string, write: (line: string) => void = (line) => { process.stderr.write(`${line}\n`); }): NonNullable<TerminationHostV1["record"]> {
  return (row, signal) => write(ownedKillRecordLineV1({ pid: row.pid, signal, killer: process.pid, label, reason: reason(), startMs: row.startMs, killerStartMs: ownProcessIdentityV1().startMs }));
}

/** 🕊️ The signal and grace options, the wait, whether a pid lives, and the platform ports used by graceful termination. */
export interface GracefulTerminationV1 extends TerminationHostV1 { readonly initialSignal?: "SIGINT" | "SIGTERM"; readonly graceMilliseconds?: number; readonly pollMilliseconds?: number; readonly sleep?: (milliseconds: number) => Promise<void>; readonly alive?: (pid: number) => boolean }

/** 🤝️ Sends the initial signal (SIGTERM by default) to the whole owned tree before ancestry is lost. After the grace period it sends SIGKILL to remaining members, including descendants started during the wait. Windows and an unreadable process table fall back to [[terminateOwnedProcessTree]]. Resolves when the tree is gone or the kill was sent.
 * @see ./🧬️schema/🔣️.json */
export async function terminateOwnedProcessTreeGracefullyV1(rootPid: number, host: GracefulTerminationV1 = {}): Promise<void> {
  if (!Number.isSafeInteger(rootPid) || rootPid <= 0) return;
  const owned = (host.platform ?? process.platform) === "win32" ? undefined : ownedRowsV1(rootPid, host.rootStartMs, host.table);
  if (owned === undefined) return terminateOwnedProcessTree(rootPid, host);
  const grace = host.graceMilliseconds ?? TERMINATION_POLICY_V1.graceMilliseconds, poll = host.pollMilliseconds ?? TERMINATION_POLICY_V1.pollMilliseconds;
  const sleep = host.sleep ?? ((milliseconds: number) => new Promise<void>((done) => setTimeout(done, milliseconds)));
  const lives = host.alive ?? processPidExistsV1, pin = host.rootStartMs ?? owned.find((row) => row.pid === rootPid && row.startMs > 0)?.startMs;
  signalOwnedRowsV1(owned, host.initialSignal ?? "SIGTERM", host.record, host.table);
  for (let waited = 0; waited < grace && owned.some((row) => lives(row.pid)); waited += poll) await sleep(poll);
  const left = owned.filter((row) => lives(row.pid)), fresh = lives(rootPid) || left.length > 0 ? ownedRowsV1(rootPid, pin, host.table) ?? [] : [];
  const rows = [...left, ...fresh.filter((row) => !left.some((known) => known.pid === row.pid))];
  if (rows.length > 0) signalOwnedRowsV1(rows, "SIGKILL", host.record, host.table);
}

/** ⏹️ [[terminateOwnedChildTree]] that asks first ([[terminateOwnedProcessTreeGracefullyV1]]). */
export function terminateOwnedChildTreeGracefullyV1(child: OwnedChildHandle, host: GracefulTerminationV1 = {}): Promise<void> {
  return child.exitCode === null && child.signalCode === null && child.pid ? terminateOwnedProcessTreeGracefullyV1(child.pid, host) : Promise.resolve();
}
