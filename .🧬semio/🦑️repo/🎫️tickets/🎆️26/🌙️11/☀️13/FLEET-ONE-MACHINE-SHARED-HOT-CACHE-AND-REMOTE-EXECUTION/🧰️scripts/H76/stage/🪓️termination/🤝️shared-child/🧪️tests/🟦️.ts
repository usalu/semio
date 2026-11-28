import { afterAll, describe, expect, test } from "bun:test";
import { spawn, spawnSync, type ChildProcess } from "node:child_process";
import { rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { REAPER_POLICY_V1 } from "../../🧹️reaper/🟦️.ts";
import { terminateOwnedChildTreeGracefullyV1, type OwnedChildHandle } from "../../🟦️.ts";
import { createSharedChildRegistryV1, sharedChildNodeHostV1, spawnSharedChildV1 } from "../🟦️.ts";

/** 🤝️ Shared child law: one cargo-like child serves several requesters; stopping the task of one requester leaves it running (the tree of the other requester's work and the shared tree are different groups), the release of the last requester ends it with everything below it, bookkeeping counts a requester once, and a parent killed by SIGKILL leaves the shared child to the reaper. Liveness is cross-checked with `ps`, descendants with `pgrep`.
 * @see ../🧬️schema/🔣️.json */
const alive = (pid: number): boolean => {
  try { process.kill(pid, 0); } catch { return false; }
  const state = spawnSync("ps", ["-o", "stat=", "-p", String(pid)], { encoding: "utf8" });
  return state.status === 0 && !state.stdout.trim().startsWith("Z");
};
const below = (pid: number): number[] => spawnSync("pgrep", ["-P", String(pid)], { encoding: "utf8" }).stdout.split("\n").filter(Boolean).map(Number);
const until = async (predicate: () => boolean, milliseconds: number): Promise<boolean> => {
  const deadline = Date.now() + milliseconds;
  while (!predicate()) { if (Date.now() > deadline) return false; await Bun.sleep(50); }
  return true;
};
const spawned: ChildProcess[] = [];
afterAll(() => { for (const child of spawned) try { process.kill(-child.pid!, "SIGKILL"); } catch {} });

interface Fake extends OwnedChildHandle { exitCode: number | null; signalCode: string | null; readonly id: number }
const fake = (id: number): Fake => ({ id, pid: id, exitCode: null, signalCode: null });

describe("bookkeeping", () => {
  test("a requester counts once, the last release stops the child once, a second release does nothing", async () => {
    const stops: string[] = [];
    const registry = createSharedChildRegistryV1<Fake>({ stop: async (child, reason) => { stops.push(`${child.id}:${reason}`); child.exitCode = 0; } });
    let made = 0;
    const start = (): Fake => fake(++made);
    const a1 = registry.acquire("cargo", "a", start), a2 = registry.acquire("cargo", "a", start), b = registry.acquire("cargo", "b", start);
    expect([a1.started, a2.started, b.started, made, registry.size()]).toEqual([true, false, false, 1, 1]);
    expect(registry.requesters("cargo")).toEqual(["a", "b"]);
    await a1.release();
    await a2.release();
    expect(stops).toEqual([]);
    expect(registry.requesters("cargo")).toEqual(["b"]);
    await b.release();
    await b.release();
    expect(stops).toHaveLength(1);
    expect(stops[0]).toContain("the last requester b left the shared child cargo / der letzte Anforderer b hat das gemeinsame Kind cargo verlassen");
    expect(registry.size()).toBe(0);
    expect(registry.acquire("cargo", "c", start).started, "a later acquire starts afresh").toBe(true);
    expect(made).toBe(2);
  });

  test("a child that ended by itself is replaced at the next acquire and is not stopped again; releaseAll releases every key of one requester", async () => {
    const stops: number[] = [];
    const registry = createSharedChildRegistryV1<Fake>({ stop: async (child) => { stops.push(child.id); child.exitCode = 0; } });
    let made = 0;
    const first = registry.acquire("k", "a", () => fake(++made));
    first.child.exitCode = 1;
    const second = registry.acquire("k", "b", () => fake(++made));
    expect([second.started, second.child.id]).toEqual([true, 2]);
    await first.release();
    expect(stops, "the dead child is not stopped, the new one keeps its requester").toEqual([]);
    registry.acquire("x", "b", () => fake(++made));
    registry.acquire("x", "c", () => fake(++made));
    await registry.releaseAll("b");
    expect(stops).toEqual([2]);
    expect(registry.requesters("x")).toEqual(["c"]);
    expect(registry.requesters("k")).toEqual([]);
  });
});

describe.skipIf(process.platform === "win32")("real processes", () => {
  const task = (): ChildProcess => { const child = spawn("sh", ["-c", "sleep 300 & wait"], { stdio: "ignore", detached: true }); spawned.push(child); return child; };

  test("two requesters share one cargo-like child: stopping one task leaves it, stopping both ends cargo and rustc", async () => {
    const registry = createSharedChildRegistryV1(sharedChildNodeHostV1("shared cargo", () => {}));
    const start = (): ChildProcess => { const child = spawnSharedChildV1("sh", ["-c", "sleep 300 & wait"], { stdio: "ignore" }); spawned.push(child); return child; };
    const taskA = task(), taskB = task();
    const leaseA = registry.acquire("cargo:target", "task-a", start), leaseB = registry.acquire("cargo:target", "task-b", start);
    expect(leaseB.started).toBe(false);
    expect(leaseA.child).toBe(leaseB.child);
    const cargo = leaseA.child.pid!;
    await Bun.sleep(600);
    const rustc = below(cargo);
    expect(rustc.length).toBeGreaterThan(0);
    await terminateOwnedChildTreeGracefullyV1(taskA, { graceMilliseconds: 2000 });
    await leaseA.release();
    expect(await until(() => !alive(taskA.pid!), 5000)).toBe(true);
    expect(alive(taskB.pid!), "the other task runs").toBe(true);
    expect(alive(cargo), "cargo survives the stop of one requester").toBe(true);
    expect(rustc.every(alive), "rustc survives").toBe(true);
    await terminateOwnedChildTreeGracefullyV1(taskB, { graceMilliseconds: 2000 });
    await leaseB.release();
    expect(await until(() => !alive(cargo) && rustc.every((pid) => !alive(pid)), 8000), "cargo and rustc are gone after the last requester").toBe(true);
  }, 40_000);

  test("a parent killed by SIGKILL leaves no shared child behind", async () => {
    const parent = spawn(process.execPath, [join(resolve(import.meta.dir), "👷️parent/📜️script.ts")], { stdio: ["ignore", "pipe", "inherit"] });
    let text = "";
    parent.stdout!.on("data", (bytes: Buffer) => { text += bytes.toString("utf8"); });
    expect(await until(() => text.includes("\n"), 20_000)).toBe(true);
    const { parent: parentPid, child } = JSON.parse(text.split("\n")[0]!) as { parent: number; child: number };
    spawned.push({ pid: child } as ChildProcess);
    await Bun.sleep(1500);
    const rustc = below(child);
    expect(alive(child) && rustc.length > 0).toBe(true);
    process.kill(parentPid, "SIGKILL");
    expect(await until(() => !alive(child) && rustc.every((pid) => !alive(pid)), REAPER_POLICY_V1.graceMilliseconds + 8000)).toBe(true);
    rmSync(join(tmpdir(), REAPER_POLICY_V1.directoryName, `${parentPid}.log`), { force: true });
  }, 40_000);
});
