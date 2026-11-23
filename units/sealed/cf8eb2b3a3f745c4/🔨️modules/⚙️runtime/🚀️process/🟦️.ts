import { requireRuntimeMemberV1 } from "../🧭️selection/🟦️.ts";
import type { RuntimeTestApiV1, RuntimeByteStreamV1, RuntimeChildV1, RuntimeSpawnOptionsV1, RuntimeSpawnSyncResultV1, RuntimeTestCommandOptionsV1, RuntimeWhichOptionsV1 } from "../📋️contract/🟦️.ts";

/** 🔎️ Finds an executable on `PATH`. */
export function runtimeWhichV1(command: string, options?: RuntimeWhichOptionsV1): string | null { return requireRuntimeMemberV1("which")(command, options); }
/** 🧭️ Resolves a module specifier from a directory or file. */
export function runtimeResolveV1(specifier: string, from: string): string { return requireRuntimeMemberV1("resolve")(specifier, from); }
/** 💤️ Waits for a number of milliseconds. */
export function runtimeSleepV1(milliseconds: number): Promise<void> { return requireRuntimeMemberV1("sleep")(milliseconds); }
let spawnObserver: ((command: readonly string[], milliseconds: number) => void) | undefined;
/** 👁️ Reports every child started through this family to one observer, with its duration in milliseconds. */
export function runtimeObserveSpawnV1(observer: ((command: readonly string[], milliseconds: number) => void) | undefined): void { spawnObserver = observer; }
/** 🚀️ Starts a child process. */
export function runtimeSpawnV1(command: readonly string[], options?: RuntimeSpawnOptionsV1): RuntimeChildV1 {
  const child = requireRuntimeMemberV1("spawn")(command, options), observer = spawnObserver;
  if (observer !== undefined) { const start = performance.now(), report = (): void => observer(command, performance.now() - start); child.exited.then(report, report); }
  return child;
}
/** 🏃️ Runs a child process to completion. */
export function runtimeSpawnSyncV1(command: readonly string[], options?: RuntimeSpawnOptionsV1): RuntimeSpawnSyncResultV1 {
  const observer = spawnObserver, start = observer === undefined ? 0 : performance.now();
  try { return requireRuntimeMemberV1("spawnSync")(command, options); } finally { observer?.(command, performance.now() - start); }
}
/** 📥️ Reads standard input as text. */
export function runtimeStdinTextV1(): Promise<string> { return requireRuntimeMemberV1("stdinText")(); }
/** 📥️ Exposes standard input as a byte stream. */
export function runtimeStdinStreamV1(): RuntimeByteStreamV1 { return requireRuntimeMemberV1("stdinStream")(); }
/** 🧪️ Builds the arguments that start the hosting runtime's test runner on files; flags in `items` pass through. */
export function runtimeTestArgumentsV1(items: readonly string[], options?: RuntimeTestCommandOptionsV1): string[] { return [...requireRuntimeMemberV1("testCommand")(items, options)]; }
/** 🧪️ Gives the thin test API of the hosting runtime's test runner, or nothing outside a test run. */
export function runtimeTestApiV1(): RuntimeTestApiV1 | undefined { try { return requireRuntimeMemberV1("testApi")(); } catch { return undefined; } }
/** 🛡️ Inserts a preload module into a test command of the hosting runtime; every other command has no answer. */
export function runtimeTestPreloadV1(args: readonly string[], preload: string): readonly string[] | undefined { try { return requireRuntimeMemberV1("testPreload")(args, preload); } catch { return undefined; } }
