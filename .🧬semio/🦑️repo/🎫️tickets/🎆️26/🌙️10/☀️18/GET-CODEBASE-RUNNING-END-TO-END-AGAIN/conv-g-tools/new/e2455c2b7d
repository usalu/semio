import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, renameSync, rmSync, statSync, writeSync } from "node:fs";
import { createRequire } from "node:module";
import { homedir } from "node:os";
import { delimiter, dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { RuntimeCapabilityMissingV1, runtimeMessageV1 } from "../../../🗣️messages/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import type { NodeNativeCoreV1 } from "../🧮️system/🟦️.ts";

const protocol = schema.definitions.Protocol.const;

/** 🖥️ The facts about the machine that decide where the Node native core lives and how it is built. */
export type NodeNativeHostV1 = Readonly<{ platform: string; arch: string; env: Readonly<Record<string, string | undefined>>; home: string; source: string }>;
/** 🎯️ One declared build target of the Node native core. */
export type NodeNativeTargetV1 = Readonly<{ platform: string; arch: string; triple: string; linkArguments: readonly string[] }>;
/** 🗺️ Where the native core of one host and one source lives in the cache and which compiler call produces it. */
export type NodeNativePlanV1 = Readonly<{ key: string; directory: string; file: string; target: NodeNativeTargetV1 }>;
/** 🔌️ The outcome of one compiler run. */
export type NodeNativeRunV1 = Readonly<{ status: number | null; stderr: string }>;
/** 🧰️ Every side effect the preparation of the native core performs; tests replace it by a table. */
export type NodeNativePortsV1 = Readonly<{
  host: NodeNativeHostV1;
  read(path: string): Uint8Array;
  exists(path: string): boolean;
  mkdir(path: string): void;
  lock(path: string): boolean;
  age(path: string): number | undefined;
  remove(path: string): void;
  rename(from: string, to: string): void;
  compiler(): string | undefined;
  run(program: string, arguments_: readonly string[], options: Readonly<{ cwd: string; env: Readonly<Record<string, string | undefined>>; timeoutMilliseconds: number }>): NodeNativeRunV1;
  sleep(milliseconds: number): void;
  now(): number;
  staging(): string;
  notice(text: string): void;
}>;

function refusal(id: string, parameters: Readonly<Record<string, string | number>>): RuntimeCapabilityMissingV1 {
  return new RuntimeCapabilityMissingV1("ffi", "node", runtimeMessageV1(id, parameters));
}

/** 🧭️ Finds the declared build target of a platform and architecture or refuses by name. */
export function nodeNativeTargetV1(platform: string, arch: string): NodeNativeTargetV1 {
  const target = protocol.targets.find(row => row.platform === platform && row.arch === arch);
  if (target === undefined) throw refusal("nativeTargetUnsupported", { platform, arch });
  return target;
}

/** 🏠️ Names the directory that holds the built cores of this user: `SEMIO_RUNTIME_CACHE`, else the operating system's cache directory. */
export function nodeNativeCacheRootV1(host: NodeNativeHostV1): string {
  const named = host.env[protocol.addon.environment.cache];
  if (named !== undefined && named !== "") return join(named, ...protocol.addon.directory);
  const row = (protocol.cacheRoots as Readonly<Record<string, Readonly<{ environment: string | null; homeSegments: readonly string[]; segments: readonly string[] }>>>)[host.platform];
  if (row === undefined) throw refusal("nativeTargetUnsupported", { platform: host.platform, arch: host.arch });
  const base = row.environment === null ? undefined : host.env[row.environment];
  return join(base !== undefined && base !== "" ? base : join(host.home, ...row.homeSegments), ...row.segments, ...protocol.addon.directory);
}

/** 📍️ Plans the cache location of the core: the key joins platform, architecture and the digest of the source and the compiler call, so a changed source never reuses an old build. */
export function planNodeNativeAddonV1(host: NodeNativeHostV1, source: Uint8Array): NodeNativePlanV1 {
  const target = nodeNativeTargetV1(host.platform, host.arch);
  const digest = createHash("sha256").update(source).update("\0").update(JSON.stringify([protocol.version, protocol.compiler.arguments, target.linkArguments])).digest("hex").slice(0, protocol.addon.digestCharacters);
  const key = `${host.platform}-${host.arch}-${digest}`, directory = join(nodeNativeCacheRootV1(host), key), file = join(directory, protocol.addon.file);
  if (file.length > protocol.addon.maximumPathCharacters) throw refusal("nativePathLength", { path: file, limit: protocol.addon.maximumPathCharacters });
  return Object.freeze({ key, directory, file, target });
}

/** ⌨️ Builds the argument vector of the one compiler call that produces the core. */
export function nodeNativeCompilerArgumentsV1(plan: NodeNativePlanV1, source: string, output: string): readonly string[] {
  return Object.freeze([...protocol.compiler.arguments, ...plan.target.linkArguments, "--crate-name", protocol.compiler.crateName, "-o", output, source]);
}

/** 🔎️ Finds the compiler: `SEMIO_RUSTC`, then the `PATH`, then the `bin` directory of `CARGO_HOME` or `~/.cargo`. */
export function nodeNativeCompilerV1(host: NodeNativeHostV1, isFile: (path: string) => boolean): string | undefined {
  const named = host.env[protocol.addon.environment.compiler];
  if (named !== undefined && named !== "") return isFile(named) ? named : undefined;
  const name = host.platform === "win32" ? `${protocol.compiler.program}.exe` : protocol.compiler.program;
  const cargo = host.env[protocol.addon.environment.cargoHome];
  const directories = [...(host.env.PATH ?? host.env.Path ?? "").split(delimiter).filter(entry => entry !== ""), join(cargo !== undefined && cargo !== "" ? cargo : join(host.home, ".cargo"), "bin")];
  for (const directory of directories) if (isFile(join(directory, name))) return join(directory, name);
  return undefined;
}

function build(ports: NodeNativePortsV1, plan: NodeNativePlanV1): void {
  const host = ports.host, compiler = ports.compiler();
  if (compiler === undefined) throw refusal("nativeCompilerMissing", { program: protocol.compiler.program });
  ports.notice(runtimeMessageV1("nativeBuilding", { platform: host.platform, arch: host.arch, directory: plan.directory }));
  if (ports.exists(plan.directory)) ports.remove(plan.directory);
  const staging = `${plan.directory}.staging-${ports.staging()}`, output = join(staging, protocol.addon.file);
  ports.mkdir(staging);
  const attempt = (env: Readonly<Record<string, string | undefined>>): NodeNativeRunV1 => ports.run(compiler, nodeNativeCompilerArgumentsV1(plan, host.source, output), { cwd: dirname(host.source), env, timeoutMilliseconds: protocol.compiler.timeoutSeconds * 1000 });
  let result = attempt(host.env);
  const retry = protocol.compiler.licenseRetry;
  if (result.status !== 0 && host.platform === "darwin" && result.stderr.includes(retry.marker) && host.env.DEVELOPER_DIR === undefined && ports.exists(retry.developerDirectory)) result = attempt({ ...host.env, DEVELOPER_DIR: retry.developerDirectory });
  if (result.status !== 0 || !ports.exists(output)) {
    ports.remove(staging);
    throw refusal("nativeCompileFailed", { status: String(result.status), tail: result.stderr.trim().split("\n").slice(-6).join(" | ").slice(0, 600) });
  }
  try { ports.rename(staging, plan.directory); } catch (error) {
    ports.remove(staging);
    if (!ports.exists(plan.file)) throw error;
  }
}

/** 🏗️ Returns the path of the built core, building it once for this source: cache hit, or an exclusive lock, one compiler call into a staging directory and one atomic rename; concurrent processes wait for the holder. */
export function ensureNodeNativeAddonV1(ports: NodeNativePortsV1): string {
  const plan = planNodeNativeAddonV1(ports.host, ports.read(ports.host.source));
  if (ports.exists(plan.file)) return plan.file;
  const lock = `${plan.directory}.lock`, settings = protocol.lock, deadline = ports.now() + settings.waitSeconds * 1000;
  ports.mkdir(dirname(plan.directory));
  for (;;) {
    if (ports.lock(lock)) {
      try { if (!ports.exists(plan.file)) build(ports, plan); return plan.file; } finally { ports.remove(lock); }
    }
    if (ports.exists(plan.file)) return plan.file;
    const age = ports.age(lock);
    if (age !== undefined && age > settings.staleSeconds * 1000) { ports.remove(lock); continue; }
    if (ports.now() > deadline) throw refusal("nativeLockTimeout", { seconds: settings.waitSeconds });
    ports.sleep(settings.pollMilliseconds);
  }
}

function isFile(path: string): boolean { try { return statSync(path).isFile(); } catch { return false; } }

/** 🏭️ Binds the preparation of the core to the real file system, process table and standard error. */
export function defaultNodeNativePortsV1(environment: NodeJS.ProcessEnv = process.env): NodeNativePortsV1 {
  const host: NodeNativeHostV1 = Object.freeze({ platform: process.platform, arch: process.arch, env: environment, home: homedir(), source: fileURLToPath(new URL("../🦀️.rs", import.meta.url)) });
  const sleeper = new Int32Array(new SharedArrayBuffer(4));
  return Object.freeze({
    host,
    read: path => readFileSync(path),
    exists: path => existsSync(path),
    mkdir: path => { mkdirSync(path, { recursive: true }); },
    lock: path => { try { mkdirSync(path); return true; } catch (error) { if ((error as NodeJS.ErrnoException).code === "EEXIST") return false; throw error; } },
    age: path => { try { return Date.now() - statSync(path).mtimeMs; } catch { return undefined; } },
    remove: path => { rmSync(path, { recursive: true, force: true }); },
    rename: (from, to) => { renameSync(from, to); },
    compiler: () => nodeNativeCompilerV1(host, isFile),
    run: (program, arguments_, options) => {
      const run = spawnSync(program, [...arguments_], { cwd: options.cwd, env: options.env as NodeJS.ProcessEnv, encoding: "utf8", timeout: options.timeoutMilliseconds, windowsHide: true, stdio: ["ignore", "ignore", "pipe"] });
      return { status: run.status, stderr: `${run.stderr ?? ""}${run.error === undefined ? "" : String(run.error)}` };
    },
    sleep: milliseconds => { Atomics.wait(sleeper, 0, 0, milliseconds); },
    now: () => Date.now(),
    staging: () => `${process.pid}-${Date.now().toString(36)}`,
    notice: text => { try { writeSync(2, `${text}\n`); } catch {} },
  });
}

/** 🔩️ Loads the native core for this machine, building it on first use; `SEMIO_RUNTIME_NATIVE_ADDON` names a prebuilt core instead. */
export function loadNodeNativeCoreV1(ports: NodeNativePortsV1 = defaultNodeNativePortsV1()): NodeNativeCoreV1 {
  const named = ports.host.env[protocol.addon.environment.addon], file = named !== undefined && named !== "" ? named : ensureNodeNativeAddonV1(ports);
  let core: Partial<NodeNativeCoreV1>;
  try { core = createRequire(import.meta.url)(file) as Partial<NodeNativeCoreV1>; } catch (error) { throw refusal("nativeLoadFailed", { file, reason: String((error as Error).message).slice(0, 400) }); }
  const missing = protocol.primitives.filter(name => typeof (core as Record<string, unknown>)[name] !== "function");
  if (missing.length > 0) throw refusal("nativeCoreIncomplete", { file, missing: missing.join(", ") });
  return core as NodeNativeCoreV1;
}
