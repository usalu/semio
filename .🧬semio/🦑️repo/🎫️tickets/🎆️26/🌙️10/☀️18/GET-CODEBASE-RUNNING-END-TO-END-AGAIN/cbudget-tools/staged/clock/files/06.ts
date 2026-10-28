import { spawn } from "node:child_process";
import { mkdirSync, mkdtempSync, realpathSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { terminateOwnedProcessTree } from "../../🪓️termination/🟦️.ts";
import { detachDiskImagesUnderV1 } from "../🏖️sandbox/🖼️images/🟦️.ts";
import { guardArgumentsV1, testCheckoutRootV1, testSandboxEnvironmentV1 } from "../🏖️sandbox/🟦️.ts";
import { describeTestBudgetV1, sharedHostContentionV1, watchTestBudgetV1, type ContentionSourceV1, type TestBudgetSnapshotV1 } from "../🎚️budget/⏲️contention/🟦️.ts";
import { bunTestCeilingV1 } from "../🟨️bun/🧯️ceiling/🟦️.ts";
import { prepareTestCommandV1, testProtocolMessageV1 } from "../🚦️protocol/🟦️.ts";

export { attachedDiskImagesV1, detachDiskImagesUnderV1 } from "../🏖️sandbox/🖼️images/🟦️.ts";

/** 🎛️ Supplies a test command's complete caller-owned process policy. */
export type TestCommandOptions = Readonly<{
  cwd: string;
  budgetMs: number;
  contention?: ContentionSourceV1;
  env?: Readonly<Record<string, string | undefined>>;
  signal?: AbortSignal;
  onTimeoutHint?: string;
  throwOnFailure?: boolean;
  captureStdout?: Readonly<{ limitBytes: number; onChunk: (bytes: Uint8Array) => void }>;
}>;

/** ⏱️ Executes one explicitly bounded test process without selecting a tool or repository policy; the budget is work, not wall time: it counts at the speed the host gives (`contention`, the load of this host by default), so a slow shared host does not stop a run that fits its budget on an idle one, while a hung process is still stopped after at most the policy's stretch times its budget. */
export async function runBudgetedTestCommand(command: string, args: readonly string[], options: TestCommandOptions): Promise<void> {
  if (!Number.isSafeInteger(options.budgetMs) || options.budgetMs < 0) throw Error("Invalid test command budget");
  if (options.captureStdout && (!Number.isSafeInteger(options.captureStdout.limitBytes) || options.captureStdout.limitBytes < 1)) throw Error("Invalid captured output byte limit");
  if (!options.cwd) throw Error("Missing test command working directory");
  if (options.signal?.aborted) throw Error(`${command} cancelled`);
  const sandbox = testSandboxEnvironmentV1(options.env ?? process.env, { route: "test-command", checkout: testCheckoutRootV1(options.cwd, options.env ?? process.env) });
  const prepared = prepareTestCommandV1(command, args, sandbox.environment, sandbox.environment.TMPDIR ?? tmpdir());
  const started = performance.now();
  const ceiling = bunTestCeilingV1(command, args, options.cwd, options.env ?? process.env);
  const child = spawn(command, [...guardArgumentsV1(command, prepared.args, sandbox.environment)], { stdio: options.captureStdout ? ["inherit", "pipe", "inherit"] : "inherit", cwd: options.cwd, env: { ...sandbox.environment, ...prepared.environment }, detached: process.platform !== "win32", windowsHide: true });
  let reason: "timeout" | "cancelled" | "interrupted" | "output-limit" | "memory-ceiling" | undefined;
  const stop = (next: typeof reason): void => {
    reason ??= next;
    if (child.pid && child.exitCode === null && child.signalCode === null) terminateOwnedProcessTree(child.pid);
  };
  const release = ceiling !== undefined && child.pid !== undefined ? ceiling(child.pid, message => { console.error(message); stop("memory-ceiling"); }) : undefined;
  let bytes=0;
  child.stdout?.on("data", (chunk: Buffer) => { bytes += chunk.byteLength; if(bytes > options.captureStdout!.limitBytes) stop("output-limit"); else options.captureStdout!.onChunk(chunk); });
  const abort = (): void => stop("cancelled"), interrupt = (): void => stop("interrupted");
  options.signal?.addEventListener("abort", abort, { once: true });
  if (options.signal?.aborted) abort();
  process.once("SIGINT", interrupt);
  process.once("SIGTERM", interrupt);
  let expired: TestBudgetSnapshotV1 | undefined;
  const watch = options.budgetMs > 0 ? watchTestBudgetV1(options.budgetMs, { source: options.contention ?? sharedHostContentionV1(), report: console.error, expire: snapshot => { expired = snapshot; stop("timeout"); } }) : undefined;
  try {
    const { code, signal } = await new Promise<{ code: number | null; signal: string | null }>((accept, reject) => {
      child.once("error", reject);
      child.once("close", (code, signal) => accept({ code, signal }));
    });
    try { prepared.finish({ code, signal, durationMs: performance.now() - started, ...(reason ? { reason } : {}) }); } catch (error) { console.error(testProtocolMessageV1("reportFailed", { reason: String(error) })); }
    const failure = reason === "timeout" ? `${command} exceeded its ${options.budgetMs}ms budget` : reason ? `${command} ${reason}` : signal ? `${command} was killed by ${signal}` : code !== 0 ? `${command} exited with status ${code}` : undefined;
    if (!failure) return;
    if (reason === "timeout") console.error(`[budget] ${command} ${args.join(" ")} exceeded ${options.budgetMs}ms — killed. ${options.onTimeoutHint ?? "Trim it, or assign it to a higher level (quick/long/exhaustive)."}${expired ? "\n" + describeTestBudgetV1("expired", expired) : ""}`);
    if (options.throwOnFailure) throw Error(failure);
    process.exit(reason || signal ? 1 : code ?? 1);
  } finally {
    if (release) release();
    watch?.stop();
    options.signal?.removeEventListener("abort", abort);
    process.off("SIGINT", interrupt);
    process.off("SIGTERM", interrupt);
    sandbox.release();
  }
}

const scratchRoots = new Map<string, string>();
let exitGuard = false;

/** 🗑️ Detaches every disk image under a scratch root created by this module, then removes the tree. */
export function disposeTestScratchRootV1(root: string): void {
  const real = scratchRoots.get(resolve(root));
  if (!real) throw Error("Scratch root created by createTestScratchRootV1 required");
  try {
    detachDiskImagesUnderV1(real);
    rmSync(real, { recursive: true, force: true });
  } finally {
    scratchRoots.delete(resolve(root));
  }
}

/** 🧹️ Creates a unique scratch root that is detached and removed on disposal and when the process exits. */
export function createTestScratchRootV1(parent: string, prefix: string): string {
  mkdirSync(parent, { recursive: true });
  const root = mkdtempSync(join(parent, prefix)), real = realpathSync(root);
  if (!exitGuard) process.on("exit", () => { for (const path of [...scratchRoots.keys()]) try { disposeTestScratchRootV1(path); } catch (error) { console.error(`[leak] scratch root ${path} not released: ${String(error)}`); } });
  exitGuard = true;
  scratchRoots.set(resolve(root), real);
  return root;
}
