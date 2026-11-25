import { spawn, type ChildProcess } from "node:child_process";
import { closeSync, mkdirSync, openSync } from "node:fs";
import { readdir, readFile } from "node:fs/promises";
import { join, resolve } from "node:path";
import { setTimeout as sleep } from "node:timers/promises";
import { terminateOwnedChildTree } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🪓️termination/🟦️.ts";
import { readLiveHostCapacityReservationV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🎫️gate/🟦️.ts";
import { BUILD_HOST_CAPACITY_POLICY_V1 as policy } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🎛️control/🟦️.ts";

const ticket = import.meta.dirname;
export const root = resolve(ticket, "../../../../../../..");
export const play = join(root, "🏢️semio-tech/🎡️play");
const output = join(ticket, "🗑️generated");
const modules = join(root, ".cache/b/p/m/browser/prototype/modules");
const os = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules");
const budget = policy.envelopes.find(row => row.name === "end-to-end");
if (!budget) throw Error("The repository has no end-to-end capacity envelope");
const { megabytes, cpu } = budget;

/** 🚦️ Starts every scope only while its predecessor and shared resource grant still hold. */
export async function runSweep(scopes: readonly string[], signal: AbortSignal, run: (scope: string) => Promise<void>): Promise<void> {
  for (const scope of scopes) {
    signal.throwIfAborted();
    await run(scope);
  }
  signal.throwIfAborted();
}

/** 🌳️ Starts an owned command and binds cancellation to its process tree. */
export function startSweepCommand(command: readonly [string, ...string[]], signal: AbortSignal, options: Parameters<typeof spawn>[2]) {
  signal.throwIfAborted();
  const child = spawn(command[0], command.slice(1), options) as ChildProcess;
  const stop = () => terminateOwnedChildTree(child);
  signal.addEventListener("abort", stop, { once: true });
  const exit = new Promise<number>((accept, reject) => {
    child.once("error", reject);
    child.once("exit", (code, reason) => accept(code ?? (reason === "SIGINT" ? 130 : reason === "SIGKILL" ? 137 : 143)));
  }).finally(() => signal.removeEventListener("abort", stop));
  return { child, exit, stop };
}

function requireReservation(env: NodeJS.ProcessEnv) {
  const reservation = readLiveHostCapacityReservationV1(env, policy);
  if (!reservation || reservation.reservedMegabytes < megabytes || reservation.cpu < cpu) {
    throw Error("The Norm workflow requires the full end-to-end capacity grant");
  }
}

function logged(args: readonly string[], name: string, cwd: string, env: NodeJS.ProcessEnv, signal: AbortSignal) {
  signal.throwIfAborted();
  requireReservation(env);
  const log = openSync(join(output, name + ".log"), "w");
  try {
    return startSweepCommand(["repo", ...args], signal, { cwd, env, stdio: ["ignore", log, log], windowsHide: true });
  } finally {
    closeSync(log);
  }
}

/** 🧾️ Reads published component receipts that have matching staged core files. */
export async function publishedModules() {
  const receipts: { pluginId: string; wasmSha256: string }[] = [];
  for (const entry of await readdir(modules).catch(() => [])) {
    const dir = join(modules, entry);
    const receipt = await readFile(join(dir, "🧾️receipt.json"), "utf8").then(source => JSON.parse(source)).catch(() => undefined);
    if (receipt?.pluginId && receipt.wasmSha256 && (await readdir(dir).catch(() => [])).some(name => name.endsWith(".core.wasm"))) receipts.push(receipt);
  }
  return receipts;
}

/** ⏳️ Waits for passive publication checks before taking the browser reservation. */
export async function waitForPublication(check: () => Promise<boolean>, intervalMs: number, message: string, signal: AbortSignal) {
  const deadline = performance.now() + 1000 * Number(process.env.NORM_PUBLICATION_WAIT_SECONDS ?? 10800);
  while (performance.now() < deadline) {
    signal.throwIfAborted();
    if (await check()) return;
    await sleep(intervalMs, undefined, { signal });
  }
  throw Error(message);
}

export type PlayProbe = Readonly<{ script: string; log: string; args?: readonly string[]; cwd?: string; environment?: NodeJS.ProcessEnv }>;
export type PlayPlan = Readonly<{
  scope: string; port: string; lanes: readonly string[]; laneLog: (lane: string, phase: string) => string;
  serverLog: string; readyAttempts: number; probes: readonly PlayProbe[]; environment?: NodeJS.ProcessEnv;
}>;

/** 🎡️ Activates lanes, runs their probes and closes the owned service before releasing the grant. */
export async function runPlayPlan(plan: PlayPlan, signal: AbortSignal) {
  const origin = `http://127.0.0.1:${plan.port}`;
  const env: NodeJS.ProcessEnv = {
    ...process.env, ...plan.environment, SEMIO_TECH_PLAY_PORT: plan.port, SEMIO_TECH_PLAY_SCOPE: plan.scope, PLAYWRIGHT_BASE_URL: origin,
    PLAYWRIGHT_BROWSERS_PATH: join(root, ".🧬semio/🦑️repo/⚡️cache/tools/ms-playwright"),
    SEMIO_PLUGIN_CATALOG: join(root, "✏️s/🧑‍💻dev/🧩️catalog/🤖️generated"),
  };
  const check = async (args: string[], name: string, cwd = root, activity = signal, environment = env) => {
    const result = await logged(args, name, cwd, environment, activity).exit;
    activity.throwIfAborted();
    if (result) throw Error(`${name} exited with status ${result}`);
  };
  const registry = join(os, "🔌️plugin/📇️registry");
  const dev = join(os, "🧑‍💻dev/📦️packages/🟦️typescript");
  for (const lane of plan.lanes) {
    for (const [owner, phase, ...tail] of [[registry, "session"], [dev, "prepare", "react", "prototype"], [dev, "activate", "react", "prototype"]]) {
      await check([join(ticket, "native-exec.ts"), owner!, "./📜️script.ts", phase!, lane, ...tail], plan.laneLog(lane, phase!));
    }
  }
  const service = logged([join(ticket, "serve-play-native.ts")], plan.serverLog, root, env, signal);
  const stopped = new AbortController();
  const activity = AbortSignal.any([signal, stopped.signal]);
  void service.exit.then(code => stopped.abort(Error(`${plan.scope} Play service stopped with status ${code}`)), error => stopped.abort(error));
  const serviceReady = async () => {
    for (let attempt = 0; attempt < plan.readyAttempts; attempt++) {
      activity.throwIfAborted();
      const response = await fetch(origin, { signal: AbortSignal.any([activity, AbortSignal.timeout(2000)]) }).catch(() => undefined);
      await response?.body?.cancel();
      if (response?.ok) return;
      await sleep(2000, undefined, { signal: activity });
    }
    throw Error(`${plan.scope} Play service did not start`);
  };
  try {
    await serviceReady();
    for (const probe of plan.probes) {
      await check(["tools", "exec", "bun", "--", join(ticket, probe.script), ...(probe.args ?? [])], probe.log, probe.cwd, activity, { ...env, ...probe.environment });
    }
    activity.throwIfAborted();
  } catch (error) {
    activity.throwIfAborted();
    throw error;
  } finally {
    service.stop();
    await service.exit;
  }
}

/** 🎟️ Runs one Play entrypoint under a single live end-to-end capacity reservation. */
export async function guardedPlay(entry: string, run: (args: string[], signal: AbortSignal) => Promise<void>, before?: (args: string[], signal: AbortSignal) => Promise<void>) {
  const [first, ...tail] = process.argv.slice(2);
  const controller = new AbortController();
  let cancellation = 0;
  const cancel = (code: number) => { cancellation ||= code; controller.abort(); };
  process.once("SIGINT", () => cancel(130));
  process.once("SIGTERM", () => cancel(143));
  const signal = controller.signal;
  try {
    if (first === "--admitted") {
      requireReservation(process.env);
      mkdirSync(output, { recursive: true });
      await run(tail, signal);
    } else {
      const args = first === undefined ? [] : [first, ...tail];
      await before?.(args, signal);
      process.exitCode = await startSweepCommand(["repo", "capacity", "run", `--memory=${megabytes}`, `--cpu=${cpu}`, "--",
        "repo", entry, "--admitted", ...args], signal, { cwd: root, stdio: "inherit", windowsHide: true }).exit;
    }
  } catch (error) {
    console.error(`[DEBUG] ${error}`);
    process.exitCode = 1;
  } finally {
    if (cancellation) process.exitCode = cancellation;
  }
}
