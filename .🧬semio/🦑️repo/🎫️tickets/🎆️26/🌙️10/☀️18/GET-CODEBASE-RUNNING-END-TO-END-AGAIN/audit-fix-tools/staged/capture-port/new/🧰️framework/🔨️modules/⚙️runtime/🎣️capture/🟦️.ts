import { runtimeProgramV1 } from "../🏷️identity/🟦️.ts";
import { runtimeMessageV1 } from "../🗣️messages/🟦️.ts";
import { runtimeSpawnSyncV1, runtimeSpawnV1 } from "../🚀️process/🟦️.ts";
import { constants } from "node:os";
import type { RuntimeByteStreamV1 } from "../📋️contract/🟦️.ts";

/** 🎣️ How one captured child ended: it exited by itself, ran out of time, was cancelled by its caller, or wrote more than its bound. */
export type RuntimeCaptureOutcomeV1 = "exited" | "timeout" | "cancelled" | "output-limit";
/** 🎣️ One stderr line of a running child, delivered while it runs (asynchronous capture) or replayed after it exited (synchronous capture). */
export interface RuntimeCaptureProgressV1 { readonly line: string; readonly elapsedMilliseconds: number; readonly live: boolean }
/** 🎣️ One bounded capture request; the first word of `command` is resolved through the runtime program table, so declared data may keep naming the hosting runtime. */
export interface RuntimeCaptureRequestV1 {
  readonly command: readonly string[];
  readonly cwd?: string;
  readonly env?: Readonly<Record<string, string | undefined>>;
  readonly stdin?: string | Uint8Array;
  readonly maxOutputBytes: number;
  readonly timeoutMilliseconds: number;
  readonly signal?: AbortSignal;
  readonly cancelled?: () => boolean;
  readonly onProgress?: (event: RuntimeCaptureProgressV1) => void;
}
/** 🎣️ The complete, bounded result of one capture; text is UTF-8, cut at the byte bound when the child exceeded it. */
export interface RuntimeCaptureResultV1 { readonly outcome: RuntimeCaptureOutcomeV1; readonly exitCode: number | null; readonly signalCode: string | null; readonly stdout: string; readonly stderr: string; readonly stdoutBytes: number; readonly stderrBytes: number; readonly elapsedMilliseconds: number }

const GRACE_MILLISECONDS = 2000, POLL_MILLISECONDS = 50;

function checked(request: RuntimeCaptureRequestV1): readonly string[] {
  if (request.command.length === 0 || request.command.some(word => typeof word !== "string")) throw new TypeError(runtimeMessageV1("commandRequired", {}));
  if (!Number.isSafeInteger(request.maxOutputBytes) || request.maxOutputBytes < 1) throw new RangeError(runtimeMessageV1("captureBoundRequired", { name: "maxOutputBytes" }));
  if (!Number.isFinite(request.timeoutMilliseconds) || request.timeoutMilliseconds <= 0) throw new RangeError(runtimeMessageV1("captureBoundRequired", { name: "timeoutMilliseconds" }));
  return [runtimeProgramV1(request.command[0]!), ...request.command.slice(1)];
}

function refused(request: RuntimeCaptureRequestV1): boolean {
  return request.signal?.aborted === true || request.cancelled?.() === true;
}

const signalName = (code: number): string => Object.entries(constants.signals).find(([, number]) => number === code - 128)?.[0] ?? "SIGTERM";
const text = (chunks: readonly Uint8Array[]): string => new TextDecoder().decode(Buffer.concat(chunks));
const empty = (outcome: RuntimeCaptureOutcomeV1): RuntimeCaptureResultV1 => ({ outcome, exitCode: null, signalCode: null, stdout: "", stderr: "", stdoutBytes: 0, stderrBytes: 0, elapsedMilliseconds: 0 });

function replay(request: RuntimeCaptureRequestV1, stderr: string, elapsed: number): void {
  if (request.onProgress === undefined) return;
  for (const line of stderr.split("\n").slice(0, stderr.endsWith("\n") ? -1 : undefined)) request.onProgress({ line, elapsedMilliseconds: elapsed, live: false });
}

/** 🎣️ Runs one child to completion and captures stdin, stdout and stderr within explicit bounds, without blocking the caller: the child is stopped (SIGTERM, then SIGKILL after two seconds) when it exceeds its time or output bound, when the caller's signal aborts or its `cancelled` predicate answers true; every stderr line reaches `onProgress` while the child runs. */
export async function runtimeCaptureV1(request: RuntimeCaptureRequestV1): Promise<RuntimeCaptureResultV1> {
  const command = checked(request);
  if (refused(request)) return empty("cancelled");
  const started = performance.now(), elapsed = (): number => performance.now() - started;
  const child = runtimeSpawnV1(command, { cwd: request.cwd, env: request.env, stdin: request.stdin ?? "ignore", stdout: "pipe", stderr: "pipe" });
  let outcome: RuntimeCaptureOutcomeV1 = "exited", killer: ReturnType<typeof setTimeout> | undefined;
  const stop = (reason: RuntimeCaptureOutcomeV1): void => {
    if (outcome !== "exited") return;
    outcome = reason; child.kill("SIGTERM");
    killer = setTimeout(() => child.kill("SIGKILL"), GRACE_MILLISECONDS);
  };
  const timer = setTimeout(() => stop("timeout"), request.timeoutMilliseconds);
  const poll = request.cancelled === undefined ? undefined : setInterval(() => { if (request.cancelled!()) stop("cancelled"); }, POLL_MILLISECONDS);
  const onAbort = (): void => stop("cancelled");
  request.signal?.addEventListener("abort", onAbort, { once: true });
  const drain = async (stream: RuntimeByteStreamV1, lines: boolean): Promise<{ chunks: Uint8Array[]; bytes: number }> => {
    const chunks: Uint8Array[] = [], decoder = new TextDecoder();
    let bytes = 0, remainder = "";
    for await (const chunk of stream) {
      const room = request.maxOutputBytes - bytes, kept = chunk.byteLength > room ? chunk.subarray(0, Math.max(0, room)) : chunk;
      if (kept.byteLength > 0) chunks.push(kept);
      bytes += chunk.byteLength;
      if (lines && kept.byteLength > 0) {
        remainder += decoder.decode(kept, { stream: true });
        const rows = remainder.split("\n"); remainder = rows.pop() ?? "";
        for (const line of rows) request.onProgress?.({ line, elapsedMilliseconds: elapsed(), live: true });
      }
      if (bytes > request.maxOutputBytes) stop("output-limit");
    }
    if (lines && remainder.length > 0) request.onProgress?.({ line: remainder, elapsedMilliseconds: elapsed(), live: true });
    return { chunks, bytes };
  };
  try {
    const [out, err, exitCode] = await Promise.all([drain(child.stdout, false), drain(child.stderr, true), child.exited]);
    const signalCode = exitCode > 128 && outcome !== "exited" ? signalName(exitCode) : null;
    return { outcome, exitCode: signalCode === null ? exitCode : null, signalCode, stdout: text(out.chunks), stderr: text(err.chunks), stdoutBytes: out.bytes, stderrBytes: err.bytes, elapsedMilliseconds: elapsed() };
  } finally {
    clearTimeout(timer); if (poll !== undefined) clearInterval(poll); if (killer !== undefined) clearTimeout(killer);
    request.signal?.removeEventListener("abort", onAbort);
  }
}

/** 🎣️ Runs one child to completion and captures its streams within the same explicit bounds, blocking the caller: the time and output bounds are enforced by the child process API, cancellation is honoured before the child starts, and stderr lines are replayed to `onProgress` after it exited. Use it only where the caller cannot yield. */
export function runtimeCaptureSyncV1(request: RuntimeCaptureRequestV1): RuntimeCaptureResultV1 {
  const command = checked(request);
  if (refused(request)) return empty("cancelled");
  const started = performance.now(), result = runtimeSpawnSyncV1(command, { cwd: request.cwd, env: request.env, stdin: request.stdin ?? "ignore", timeout: request.timeoutMilliseconds, maxBuffer: request.maxOutputBytes }), elapsed = performance.now() - started;
  const limited = result.stdout.byteLength >= request.maxOutputBytes || result.stderr.byteLength >= request.maxOutputBytes;
  const outcome: RuntimeCaptureOutcomeV1 = result.signalCode === null ? "exited" : limited ? "output-limit" : elapsed >= request.timeoutMilliseconds ? "timeout" : "exited";
  const bounded = (bytes: Buffer): string => (bytes.byteLength > request.maxOutputBytes ? bytes.subarray(0, request.maxOutputBytes) : bytes).toString("utf8"), stderr = bounded(result.stderr);
  replay(request, stderr, elapsed);
  return { outcome, exitCode: result.exitCode, signalCode: result.signalCode, stdout: bounded(result.stdout), stderr, stdoutBytes: result.stdout.byteLength, stderrBytes: result.stderr.byteLength, elapsedMilliseconds: elapsed };
}
