import { execFileSync, spawn, spawnSync, type ChildProcess, type ExecFileSyncOptions, type ExecFileSyncOptionsWithBufferEncoding, type ExecFileSyncOptionsWithStringEncoding, type SpawnOptions, type SpawnSyncOptions, type SpawnSyncOptionsWithBufferEncoding, type SpawnSyncOptionsWithStringEncoding, type SpawnSyncReturns } from "node:child_process";
import { routeRegisteredCommandV1 } from "../🔌️funnel/🟦️.ts";

const route = (command: string, args: readonly string[], options: { cwd?: unknown; env?: NodeJS.ProcessEnv | undefined }) => routeRegisteredCommandV1(command, args, String(options.cwd ?? process.cwd()), options.env ?? process.env);

/** 🔁️ Drop-in for `spawnSync(command, args, options)` that starts a registered tool: the registry resolves the executable and sanitizes the environment, every other option is Node's. */
export function spawnToolSyncV1(command: string, args: readonly string[], options: SpawnSyncOptionsWithStringEncoding): SpawnSyncReturns<string>;
export function spawnToolSyncV1(command: string, args: readonly string[], options: SpawnSyncOptionsWithBufferEncoding): SpawnSyncReturns<Buffer>;
export function spawnToolSyncV1(command: string, args: readonly string[], options?: SpawnSyncOptions): SpawnSyncReturns<string | Buffer>;
export function spawnToolSyncV1(command: string, args: readonly string[], options: SpawnSyncOptions = {}): SpawnSyncReturns<string | Buffer> {
  const routed = route(command, args, options);
  return spawnSync(routed.command, [...routed.arguments], { ...options, env: routed.environment as NodeJS.ProcessEnv });
}

/** 🔂️ Drop-in for `execFileSync(command, args, options)` that starts a registered tool; a failure throws like Node's. */
export function execToolSyncV1(command: string, args: readonly string[], options: ExecFileSyncOptionsWithStringEncoding): string;
export function execToolSyncV1(command: string, args: readonly string[], options: ExecFileSyncOptionsWithBufferEncoding): Buffer;
export function execToolSyncV1(command: string, args: readonly string[], options?: ExecFileSyncOptions): string | Buffer;
export function execToolSyncV1(command: string, args: readonly string[], options: ExecFileSyncOptions = {}): string | Buffer {
  const routed = route(command, args, options);
  return execFileSync(routed.command, [...routed.arguments], { ...options, env: routed.environment as NodeJS.ProcessEnv });
}

/** 🌊️ Drop-in for `spawn(command, args, options)` that starts a registered tool. */
export function spawnToolV1(command: string, args: readonly string[], options: SpawnOptions = {}): ChildProcess {
  const routed = route(command, args, options);
  return spawn(routed.command, [...routed.arguments], { ...options, env: routed.environment as NodeJS.ProcessEnv });
}
