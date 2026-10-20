import { spawn, spawnSync } from "node:child_process";
import { realpathSync } from "node:fs";
import { isAbsolute, relative, sep } from "node:path";
import { fileURLToPath } from "node:url";
import { OwnedCommandFailure, runOwnedCommand, type OwnedCommandOptions, type OwnedCommandStreamV1 } from "../../🎛️owned-execution/🟦️.ts";
import { terminateOwnedChildTree } from "../../🪓️termination/🟦️.ts";
import { hostCapacityDirectoryV1, newHostCapacityIdV1, sizeHostRequestV1, withHostCapacityV1, type HostActionKindV1, type HostCapacityPolicyV1 } from "../../🏗️build/🌡️host-capacity/🟦️.ts";
import { findToolV1, toolCallerAllowedV1, toolLanguageV1, toolMessageV1, TOOL_REGISTRY_V1, type ToolEntryV1, type ToolRegistryV1 } from "../📖️registry/🟦️.ts";
import { resolveToolV1, systemToolHostV1, toolRootV1, type ToolHostV1, type ToolResolutionV1 } from "../🔎️resolve/🟦️.ts";
import { toolEnvironmentV1 } from "../🌿️environment/🟦️.ts";

/** 🛑️ Why a tool run did not produce a result: the code names the cause, the message is in the caller's language. */
export class ToolErrorV1 extends Error {
  readonly code: "unknown-tool" | "tool-missing" | "caller-refused" | "sync-refused" | "cancelled" | "failed";
  readonly tool: string;
  readonly status: number | null;
  readonly signal: string | null;
  constructor(code: ToolErrorV1["code"], tool: string, message: string, outcome: Readonly<{ status: number | null; signal: string | null }> = { status: null, signal: null }) {
    super(message);
    this.name = "ToolErrorV1";
    this.code = code;
    this.tool = tool;
    this.status = outcome.status;
    this.signal = outcome.signal;
  }
}

/** 📣️ A progress event of one tool run. */
export type ToolProgressV1 = Readonly<{ phase: "admitting" | "running" | "finished"; tool: string; elapsedMs: number; status?: number | null }>;
/** 🎟️ What a tool asks the machine for before it starts. */
export type ToolCapacityAskV1 = Readonly<{ tool: string; operation: string; kind: HostActionKindV1 | undefined; label: string }>;
/** 🚪️ Admits a tool run against machine capacity: waits fairly, hands the body the grant's environment, gives the grant back when the body ends. */
export type ToolAdmissionV1 = <T>(ask: ToolCapacityAskV1, body: (grant: Readonly<{ environment: Readonly<Record<string, string>>; jobs: number }>) => Promise<T>, signal?: AbortSignal) => Promise<T>;

/** ⚙️ What one tool run can be told. */
export type ToolRunOptionsV1 = Readonly<{
  cwd?: string; root?: string; env?: Readonly<Record<string, string | undefined>>; environment?: Readonly<Record<string, string | undefined>>; signal?: AbortSignal; timeoutMs?: number; label?: string;
  check?: boolean; capture?: boolean; stdout?: OwnedCommandOptions["stdout"]; input?: string | Uint8Array; maximumOutputBytes?: number;
  onLine?: (line: string, stream: OwnedCommandStreamV1) => void; onProgress?: (event: ToolProgressV1) => void; heavy?: boolean; admission?: ToolAdmissionV1 | false;
  caller?: string; trusted?: boolean; registry?: ToolRegistryV1; host?: ToolHostV1; cacheDirectory?: (...segments: string[]) => string;
}>;
/** 🏁️ What a finished tool run left behind. */
export type ToolRunResultV1 = Readonly<{ tool: string; command: string; arguments: readonly string[]; status: number | null; signal: string | null; stdout: string; stderr: string; durationMs: number; resolution: ToolResolutionV1 }>;

const TOOLS_SEGMENT = "/🔨️modules/🏃️process/🧰️tools/", TESTS_SEGMENT = "/🧰️tools/🧪️tests/";
let defaultAdmission: ToolAdmissionV1 | undefined;
const resolved = new Map<string, ToolResolutionV1>();

/** 🔧️ Installs the machine capacity gate every admitted tool run goes through; the repository entry installs it once. */
export function setDefaultToolAdmissionV1(admission: ToolAdmissionV1 | undefined): void { defaultAdmission = admission; }

/** 🏗️ Builds the admission port over the host capacity gate of the process module with an explicit policy. */
export function createToolAdmissionV1(policy: HostCapacityPolicyV1, ports: Readonly<{ environment?: NodeJS.ProcessEnv; directory?: string; write?: (line: string) => void }> = {}): ToolAdmissionV1 {
  return async (ask, body, signal) => {
    const request = sizeHostRequestV1(policy, { id: newHostCapacityIdV1(), operation: ask.operation, ...ask.kind === undefined ? {} : { kinds: [ask.kind] } });
    if (request === undefined) return body({ environment: {}, jobs: 1 });
    const controller = new AbortController(), forward = (): void => controller.abort(signal?.reason);
    signal?.addEventListener("abort", forward, { once: true });
    const control = Object.freeze({ signal: controller.signal, remainingMilliseconds: () => null, publish: () => undefined, yieldContinuation: () => new Promise<void>(done => setImmediate(done)) });
    try {
      const environment = ports.environment ?? process.env;
      return await withHostCapacityV1({ directory: ports.directory ?? hostCapacityDirectoryV1(environment), policy, request, control, environment, ...ports.write === undefined ? {} : { write: ports.write } }, handle => body({ environment: handle.environment, jobs: handle.jobs }));
    } finally { signal?.removeEventListener("abort", forward); }
  };
}

/** 🧭️ Answers the repository-relative path of the first stack frame outside the tools module, or undefined when it cannot be told. */
export function toolCallerFileV1(root: string, stack: string = new Error().stack ?? ""): string | undefined {
  for (const line of stack.split("\n")) {
    const match = /(file:\/\/[^\s()]+?|\/[^\s()]+?|[A-Za-z]:[\\/][^\s()]+?):\d+:\d+\)?\s*$/u.exec(line);
    if (match === null) continue;
    let path = match[1]!;
    try { if (path.startsWith("file://")) path = fileURLToPath(path); } catch { continue; }
    if (path.includes(TOOLS_SEGMENT) && !path.includes(TESTS_SEGMENT)) continue;
    const direct = relative(root, path).split(sep);
    if (direct[0] !== "..") return direct.join("/");
    try { const real = relative(realpathSync(root), path).split(sep); return real[0] === ".." ? undefined : real.join("/"); } catch { return undefined; }
  }
  return undefined;
}

const entryOf = (toolId: string, options: ToolRunOptionsV1): { entry: ToolEntryV1; registry: ToolRegistryV1; language: ReturnType<typeof toolLanguageV1> } => {
  const registry = options.registry ?? TOOL_REGISTRY_V1, language = toolLanguageV1(options.environment ?? process.env), entry = findToolV1(toolId, registry);
  if (entry === undefined) throw new ToolErrorV1("unknown-tool", toolId, toolMessageV1("unknownTool", { tool: toolId, known: registry.tools.map(tool => tool.id).join(", ") }, language, registry));
  return { entry, registry, language };
};

const normalizeCaller = (root: string, caller: string): string => {
  let path = caller;
  try { if (path.startsWith("file://")) path = fileURLToPath(path); } catch { return caller; }
  if (!isAbsolute(path)) return path.split(sep).join("/");
  const direct = relative(root, path).split(sep);
  if (direct[0] !== "..") return direct.join("/");
  try { const real = relative(realpathSync(root), path).split(sep); return real[0] === ".." ? caller : real.join("/"); } catch { return caller; }
};

const checkCaller = (entry: ToolEntryV1, registry: ToolRegistryV1, root: string, options: ToolRunOptionsV1, language: ReturnType<typeof toolLanguageV1>): void => {
  const environment = options.environment ?? process.env, mode = environment.SEMIO_TOOL_GUARD ?? "refuse";
  if (mode === "off" || options.trusted === true || options.caller === undefined) return;
  const caller = normalizeCaller(root, options.caller);
  if (toolCallerAllowedV1(entry, caller, registry)) return;
  const message = toolMessageV1("callerRefused", { caller, tool: entry.id, allowed: entry.allowedCallers.join(", "), adapter: entry.adapter }, language, registry);
  if (mode === "warn") process.stderr.write(message + "\n"); else throw new ToolErrorV1("caller-refused", entry.id, message);
};

/** 🔎️ Resolves a tool on the host, remembering a hit per root so repeated runs do not search again. */
export function resolveRegisteredToolV1(entry: ToolEntryV1, root: string, options: ToolRunOptionsV1 = {}): ToolResolutionV1 {
  const registry = options.registry ?? TOOL_REGISTRY_V1, key = `${root}\0${entry.id}`, cached = options.host === undefined && options.environment === undefined && options.registry === undefined ? resolved.get(key) : undefined;
  if (cached !== undefined) return cached;
  const resolution = resolveToolV1(entry, options.host ?? systemToolHostV1(root, options.environment ?? process.env), registry);
  if (resolution.command !== null && options.host === undefined && options.environment === undefined && options.registry === undefined) resolved.set(key, resolution);
  return resolution;
}

const prepare = (toolId: string, options: ToolRunOptionsV1) => {
  const { entry, registry, language } = entryOf(toolId, options), root = options.root ?? toolRootV1(options.cwd, options.environment ?? process.env);
  checkCaller(entry, registry, root, options, language);
  const resolution = resolveRegisteredToolV1(entry, root, options);
  if (resolution.command === null) throw new ToolErrorV1("tool-missing", entry.id, toolMessageV1("toolMissing", { tool: entry.id, tried: resolution.tried.join(", "), command: entry.provision.hints?.[process.platform as "darwin"] ?? entry.provision.command }, language, registry));
  const environment = toolEnvironmentV1(entry, options.environment ?? process.env, options.env ?? {}, { root, ...options.cacheDirectory === undefined ? {} : { cacheDirectory: options.cacheDirectory } }, registry);
  return { entry, registry, language, root, resolution, environment, cwd: options.cwd ?? root };
};

const capture = (command: string, args: readonly string[], cwd: string, environment: Record<string, string>, options: ToolRunOptionsV1, label: string): Promise<{ status: number | null; signal: string | null; stdout: string; stderr: string }> => new Promise((accept, reject) => {
  if (options.signal?.aborted) { reject(new ToolErrorV1("cancelled", label, `${label}: cancelled`)); return; }
  const limit = options.maximumOutputBytes ?? 8 * 1024 * 1024, out: Buffer[] = [], err: Buffer[] = [];
  let outBytes = 0, errBytes = 0, stopped: "cancelled" | "timeout" | undefined;
  const child = spawn(command, [...args], { cwd, env: environment, stdio: [options.input === undefined ? "ignore" : "pipe", "pipe", "pipe"], detached: process.platform !== "win32", windowsHide: true });
  const stop = (reason: "cancelled" | "timeout"): void => { stopped ??= reason; terminateOwnedChildTree(child); };
  const abort = (): void => stop("cancelled"), timer = options.timeoutMs ? setTimeout(() => stop("timeout"), options.timeoutMs) : undefined;
  options.signal?.addEventListener("abort", abort, { once: true });
  child.stdout?.on("data", (bytes: Buffer) => { if (outBytes < limit) out.push(bytes); outBytes += bytes.length; });
  child.stderr?.on("data", (bytes: Buffer) => { if (errBytes < limit) err.push(bytes); errBytes += bytes.length; });
  if (options.input !== undefined) child.stdin?.end(options.input);
  child.once("error", error => { clearTimeout(timer); options.signal?.removeEventListener("abort", abort); reject(error); });
  child.once("close", (status, signal) => {
    clearTimeout(timer);
    options.signal?.removeEventListener("abort", abort);
    if (stopped === "cancelled") { reject(new ToolErrorV1("cancelled", label, `${label}: cancelled`)); return; }
    accept({ status: stopped === "timeout" ? null : status, signal: stopped === "timeout" ? "SIGTERM" : signal, stdout: Buffer.concat(out).toString("utf8"), stderr: Buffer.concat(err).toString("utf8") });
  });
});

/** 🏃️ Runs a registered tool: checks the declared caller (`caller: import.meta.url`; the static law reads the call sites, because a stack cannot attribute tail calls) against the registry, finds the executable, builds the sanitized environment, waits for machine capacity when the tool is heavy, and runs it with progress and process-tree cancellation. A failed run throws unless `check: false`; `capture: true` returns the output instead of forwarding it. */
export async function runToolV1(toolId: string, args: readonly string[], options: ToolRunOptionsV1 = {}): Promise<ToolRunResultV1> {
  const { entry, registry, language, resolution, environment, cwd } = prepare(toolId, options);
  const started = performance.now(), label = options.label ?? entry.id, command = resolution.command!, fullArguments = [...resolution.prefix, ...args];
  const emit = (event: Omit<ToolProgressV1, "tool" | "elapsedMs">): void => options.onProgress?.({ tool: entry.id, elapsedMs: Math.round(performance.now() - started), ...event });
  const admit = options.admission === false || entry.capacity.mode === "none" || entry.capacity.mode === "inherit" && options.heavy !== true ? undefined : options.admission ?? defaultAdmission;
  const execute = async (granted: Readonly<Record<string, string>>): Promise<ToolRunResultV1> => {
    const childEnvironment = { ...environment, ...granted };
    emit({ phase: "running" });
    let status: number | null = 0, signal: string | null = null, stdout = "", stderr = "";
    if (options.capture === true) ({ status, signal, stdout, stderr } = await capture(command, fullArguments, cwd, childEnvironment, options, label));
    else {
      try {
        await runOwnedCommand(command, [...fullArguments], cwd, label, options.timeoutMs ?? 0, {
          env: childEnvironment, ...options.signal === undefined ? {} : { signal: options.signal }, ...options.stdout === undefined ? {} : { stdout: options.stdout },
          ...options.input === undefined ? {} : { input: options.input }, onLine: (line, stream) => { options.onLine?.(line, stream); },
          ...options.onProgress === undefined ? {} : { onProgress: () => emit({ phase: "running" }) },
        });
      } catch (error) {
        if (error instanceof OwnedCommandFailure) ({ status, signal } = error.outcome);
        else if (options.signal?.aborted) throw new ToolErrorV1("cancelled", entry.id, toolMessageV1("cancelled", { tool: entry.id }, language, registry));
        else throw error;
      }
    }
    emit({ phase: "finished", status });
    const result: ToolRunResultV1 = Object.freeze({ tool: entry.id, command, arguments: Object.freeze(fullArguments), status, signal, stdout, stderr, durationMs: performance.now() - started, resolution });
    if (options.check !== false && (status !== 0 || signal !== null)) throw new ToolErrorV1("failed", entry.id, toolMessageV1("failed", { tool: entry.id, status: signal ?? String(status) }, language, registry), { status, signal });
    return result;
  };
  if (admit === undefined) return execute({});
  emit({ phase: "admitting" });
  return admit({ tool: entry.id, operation: entry.capacity.operation ?? "command", kind: entry.capacity.kind, label }, grant => execute(grant.environment), options.signal);
}

/** ⏱️ Runs a light registered tool synchronously for short queries such as `git rev-parse`; heavy tools refuse because a synchronous run can be neither cancelled nor admitted. */
export function runToolSyncV1(toolId: string, args: readonly string[], options: ToolRunOptionsV1 = {}): Readonly<{ status: number | null; signal: string | null; stdout: string; stderr: string; resolution: ToolResolutionV1 }> {
  const { entry, registry, language, resolution, environment, cwd } = prepare(toolId, options);
  if (entry.capacity.mode === "admit") throw new ToolErrorV1("sync-refused", entry.id, toolMessageV1("syncRefused", { tool: entry.id }, language, registry));
  const result = spawnSync(resolution.command!, [...resolution.prefix, ...args], { cwd, env: environment, encoding: "utf8", windowsHide: true, maxBuffer: options.maximumOutputBytes ?? 64 * 1024 * 1024, ...options.timeoutMs ? { timeout: options.timeoutMs } : {}, ...options.input === undefined ? {} : { input: options.input } });
  if (result.error !== undefined && (result.error as NodeJS.ErrnoException).code !== "ETIMEDOUT") throw result.error;
  const outcome = Object.freeze({ status: result.status, signal: result.signal, stdout: result.stdout ?? "", stderr: result.stderr ?? "", resolution });
  if (options.check === true && (outcome.status !== 0 || outcome.signal !== null)) throw new ToolErrorV1("failed", entry.id, toolMessageV1("failed", { tool: entry.id, status: outcome.signal ?? String(outcome.status) }, language, registry), outcome);
  return outcome;
}

/** 🩺️ Asks a resolved tool for its version with the registry probe; null when the tool has no probe, is missing or does not answer in time. */
export function probeToolV1(entry: ToolEntryV1, resolution: ToolResolutionV1, environment: Readonly<Record<string, string | undefined>> = process.env, root: string = process.cwd()): string | null {
  if (entry.probe === undefined || resolution.command === null) return null;
  const result = spawnSync(resolution.command, [...resolution.prefix, ...entry.probe.arguments], { cwd: root, env: toolEnvironmentV1(entry, environment, {}, { root }) , encoding: "utf8", timeout: 10_000, windowsHide: true });
  const match = new RegExp(entry.probe.pattern, "mu").exec(`${result.stdout ?? ""}\n${result.stderr ?? ""}`);
  return result.status === 0 && match !== null ? match[1] ?? match[0] : null;
}
