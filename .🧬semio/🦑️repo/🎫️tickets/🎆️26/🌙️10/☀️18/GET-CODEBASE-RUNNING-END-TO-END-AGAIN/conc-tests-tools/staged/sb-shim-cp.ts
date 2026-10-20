import * as real from "node:child_process";

export * from "node:child_process";
type Guard = { write(fn: string, args: readonly unknown[]): void; open(fn: string, args: readonly unknown[]): void; spawn(command: unknown, args: readonly unknown[]): void; exec(command: unknown): void };
const guard = (): Guard | undefined => (globalThis as { __semioTestGuardV1?: Guard }).__semioTestGuardV1;
const spawns = <Fn extends (...args: never[]) => unknown>(fn: Fn): Fn => ((...args: unknown[]) => { guard()?.spawn(args[0], Array.isArray(args[1]) ? args[1] : []); return (fn as unknown as (...rest: unknown[]) => unknown)(...args); }) as unknown as Fn;
const execs = <Fn extends (...args: never[]) => unknown>(fn: Fn): Fn => ((...args: unknown[]) => { guard()?.exec(args[0]); return (fn as unknown as (...rest: unknown[]) => unknown)(...args); }) as unknown as Fn;
export const spawn = spawns(real.spawn);
export const spawnSync = spawns(real.spawnSync);
export const execFile = spawns(real.execFile);
export const execFileSync = spawns(real.execFileSync);
export const exec = execs(real.exec);
export const execSync = execs(real.execSync);
export default { ...real, spawn, spawnSync, execFile, execFileSync, exec, execSync };
