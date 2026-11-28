import { ownedKillRecordLineV1, TERMINATION_POLICY_V1, terminateOwnedChildTree, terminateOwnedChildTreeGracefullyV1 } from "../🟦️.ts";
/** 🪓️ Process-tree termination law: [[terminateOwnedChildTree]] leaves no descendant of a spawned child alive,
 * exactly like the third-party `tree-kill`, while a root-only kill orphans every descendant, and an exited child is
 * never signalled again (its pid may already belong to someone else). POSIX descendants are cross-checked with `pgrep`.
 * @see ../🧫️fixtures/🔣️.json */
import { describe, expect, test } from "bun:test";
import { spawn, spawnSync, type ChildProcess } from "node:child_process";
import { existsSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

import treeKill from "tree-kill";
import "./🛡️identity/🟦️.ts";
import "../🧹️reaper/🧪️tests/🟦️.ts";
import "../🤝️shared-child/🧪️tests/🟦️.ts";


type Terminator = "terminateOwnedChildTree" | "terminateOwnedChildTreeGracefullyV1" | "tree-kill" | "child.kill";
type Fixture = Readonly<{
  tree: Readonly<{ depth: number; breadth: number }>;
  spawnDeadlineMs: number;
  settleDeadlineMs: number;
  cases: readonly Readonly<{ id: string; terminator: Terminator; survivors: number }>[];
  records: readonly Readonly<{ pid: number; signal: "SIGTERM" | "SIGKILL"; killer: number; label: string; reason: string; startMs?: number; killerStartMs?: number }>[];
}>;

const ownerRoot = resolve(import.meta.dir, "..");
const fixture = JSON.parse(readFileSync(join(ownerRoot, "🧪️tests/🧫️fixtures/🔣️.json"), "utf8")) as Fixture;
const SPAWNER = `const { spawn } = require("node:child_process");
const { appendFileSync } = require("node:fs");
const [depth, breadth, pidFile] = process.argv.slice(2);
appendFileSync(pidFile, process.pid + "\\n");
for (let index = 0; Number(depth) > 0 && index < Number(breadth); index++) spawn(process.execPath, [process.argv[1], String(Number(depth) - 1), breadth, pidFile], { stdio: "ignore" });
setInterval(() => {}, 1 << 30);
`;

function treeSize({ depth, breadth }: Fixture["tree"]): number {
  let size = 0;
  for (let level = 0; level <= depth; level++) size += breadth ** level;
  return size;
}

function alive(pid: number): boolean {
  try {
    process.kill(pid, 0);
  } catch {
    return false;
  }
  if (process.platform === "win32") return true;
  const state = spawnSync("ps", ["-o", "stat=", "-p", String(pid)], { encoding: "utf8" });
  return state.status === 0 && !state.stdout.trim().startsWith("Z");
}

function pgrepDescendants(root: number): number[] {
  const found: number[] = [];
  for (const frontier = [root]; frontier.length > 0;) {
    const parent = frontier.pop()!;
    const children = spawnSync("pgrep", ["-P", String(parent)], { encoding: "utf8" }).stdout.split("\n").filter(Boolean).map(Number);
    found.push(...children);
    frontier.push(...children);
  }
  return found.sort((left, right) => left - right);
}

async function until(predicate: () => boolean, deadlineMs: number): Promise<boolean> {
  const deadline = Date.now() + deadlineMs;
  while (!predicate()) {
    if (Date.now() > deadline) return false;
    await Bun.sleep(50);
  }
  return true;
}

async function spawnTree(root: string, id: string): Promise<Readonly<{ child: ChildProcess; pids: readonly number[]; exited: Promise<void> }>> {
  const script = join(root, "spawner.cjs"), pidFile = join(root, `${id}.pids`);
  if (!existsSync(script)) writeFileSync(script, SPAWNER);
  writeFileSync(pidFile, "");
  const child = spawn(process.execPath, [script, String(fixture.tree.depth), String(fixture.tree.breadth), pidFile], { stdio: "ignore" });
  const exited = new Promise<void>((accept) => child.once("exit", () => accept()));
  const read = (): number[] => readFileSync(pidFile, "utf8").split("\n").filter(Boolean).map(Number);
  expect(await until(() => read().length === treeSize(fixture.tree), fixture.spawnDeadlineMs)).toBe(true);
  return { child, pids: read(), exited };
}

async function terminate(terminator: Terminator, child: ChildProcess): Promise<void> {
  if (terminator === "terminateOwnedChildTree") terminateOwnedChildTree(child);
  else if (terminator === "terminateOwnedChildTreeGracefullyV1") await terminateOwnedChildTreeGracefullyV1(child);
  else if (terminator === "child.kill") child.kill("SIGKILL");
  else await new Promise<void>((accept, reject) => treeKill(child.pid!, "SIGKILL", (error) => (error ? reject(error) : accept())));
}

describe("process-tree termination", () => {
  

  for (const law of fixture.cases) {
    test(`${law.id}: ${law.terminator} leaves ${law.survivors} descendants alive`, async () => {
      const root = mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!, "process-tree-"));
      let pids: readonly number[] = [];
      try {
        const tree = await spawnTree(root, law.id);
        pids = tree.pids;
        const descendants = pids.filter((pid) => pid !== tree.child.pid).sort((left, right) => left - right);
        if (process.platform !== "win32") expect(pgrepDescendants(tree.child.pid!)).toEqual(descendants);
        await terminate(law.terminator, tree.child);
        await tree.exited;
        await until(() => descendants.filter(alive).length === law.survivors, fixture.settleDeadlineMs);
        expect(descendants.filter(alive).length).toBe(law.survivors);
        expect(alive(tree.child.pid!)).toBe(false);
      } finally {
        for (const pid of pids) if (alive(pid)) process.kill(pid, "SIGKILL");
        rmSync(root, { recursive: true, force: true });
      }
    }, fixture.spawnDeadlineMs + fixture.settleDeadlineMs + 5_000);
  }

  test("the declared grace asks first: a polite tree gets SIGTERM only, a tree that ignores SIGTERM is killed after the grace, and every signal is recorded in both languages", async () => {
    const root = mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!, "process-tree-"));
    expect(TERMINATION_POLICY_V1.graceMilliseconds).toBe(5000);
    const seen: string[] = [], children: ChildProcess[] = [];
    try {
      const polite = spawn("sh", ["-c", "sleep 300 & wait"], { stdio: "ignore", detached: true }), stubborn = spawn("sh", ["-c", 'trap "" TERM; while :; do sleep 1; done'], { stdio: "ignore", detached: true });
      children.push(polite, stubborn);
      await Bun.sleep(400);
      const record = (row: Readonly<{ pid: number }>, signal: string): void => { seen.push(`${row.pid}:${signal}`); };
      await terminateOwnedChildTreeGracefullyV1(polite, { record, graceMilliseconds: 3000 });
      expect(seen.filter((entry) => entry.endsWith(":SIGKILL")), "nothing that ends by itself is killed").toEqual([]);
      expect(seen.some((entry) => entry === `${polite.pid}:SIGTERM`)).toBe(true);
      const before = Date.now();
      await terminateOwnedChildTreeGracefullyV1(stubborn, { record, graceMilliseconds: 800 });
      expect(Date.now() - before, "the grace is waited for").toBeGreaterThanOrEqual(700);
      expect(seen.filter((entry) => entry.startsWith(`${stubborn.pid}:`)).slice(0, 2)).toEqual([`${stubborn.pid}:SIGTERM`, `${stubborn.pid}:SIGKILL`]);
      expect(await until(() => !alive(stubborn.pid!) && !alive(polite.pid!), fixture.settleDeadlineMs)).toBe(true);
    } finally {
      for (const child of children) try { process.kill(-child.pid!, "SIGKILL"); } catch {}
      rmSync(root, { recursive: true, force: true });
    }
  }, 30_000);

  test("the kill record names victim, signal, killer, command and reason in English and German, as the Python oracle derives it", () => {
    for (const row of fixture.records) {
      const line = ownedKillRecordLineV1(row);
      expect(line.includes("Victim created at "), "identity only when both creation times are known").toBe(row.startMs !== undefined && row.killerStartMs !== undefined);
      if (row.startMs !== undefined && row.killerStartMs !== undefined) expect(line).toContain(`Victim created at ${row.startMs}, sender created at ${row.killerStartMs} (ms since the epoch). / Prozess`);
      expect(line).toContain(`Process ${row.pid} received ${row.signal} from owned execution ${row.label} (killing process ${row.killer}): ${row.reason}`);
      expect(line).toContain(` / Prozess ${row.pid} erhielt ${row.signal} von der eigenen Ausführung ${row.label} (beendender Prozess ${row.killer}): ${row.reason}`);
    }
    const run = spawnSync("python3", ["-I", join(ownerRoot, "🧪️tests/🐍️.py"), join(ownerRoot, "🧬️schema/🔣️.json"), join(ownerRoot, "🧪️tests/🧫️fixtures/🔣️.json")], { encoding: "utf8" });
    expect(run.status, run.stderr).toBe(0);
    expect(JSON.parse(run.stdout)).toEqual(fixture.records.map((row) => ownedKillRecordLineV1(row)));
  });

  test("never signals a child that has already exited", async () => {
    const child = spawn(process.execPath, ["-e", ""], { stdio: "ignore" });
    await new Promise<void>((accept) => child.once("exit", () => accept()));
    const signals: unknown[][] = [];
    const original = process.kill;
    process.kill = ((...args: unknown[]) => { signals.push(args); return true; }) as typeof process.kill;
    try {
      terminateOwnedChildTree(child);
    } finally {
      process.kill = original;
    }
    expect(signals).toEqual([]);
  });
});
