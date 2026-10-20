import type { ScriptInvocation } from "@semio-tech/process/routing";
import { join, resolve } from "node:path";
import { parseServiceReadinessBudgetV1, readServiceSession, serviceReadyProgressLanguage, serviceReadyProgressLine, waitForServiceReady } from "@semio-tech/repo-lib/dev";
import readinessBudget from "../🧩️runtime/🧪️e2e/🔣️.json" with { type: "json" };
import { repoCacheDirectory } from "@semio-tech/repo-lib/process";
import { DEMONSTRATOR_E2E_OWNER, demonstratorE2eInvocationPid, demonstratorE2eSessionRoot } from "../🧩️runtime/🧪️e2e/🟦️.ts";

/** 🏃️ Runs the `TestScript` command under the caller's roots and invocation. */
export async function runTestScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, args: string[]): Promise<void> {
  if (args.length) throw new Error("Demonstrator E2E accepts no compiler or server arguments");
  const root = resolve(context.root, "../.."), sessionRoot = demonstratorE2eSessionRoot(context.repoRoot), controller = new AbortController();
  const session = readServiceSession(sessionRoot, DEMONSTRATOR_E2E_OWNER, demonstratorE2eInvocationPid(process.env));
  const interrupt = (): void => { process.exitCode = 130; controller.abort(); }, terminate = (): void => { process.exitCode = 143; controller.abort(); };
  process.once("SIGINT", interrupt); process.once("SIGTERM", terminate);
  try {
    console.log(`Waiting for Demonstrator E2E service ${session.id}`);
    const budget = parseServiceReadinessBudgetV1(readinessBudget), language = serviceReadyProgressLanguage(process.env);
    const baseURL = await waitForServiceReady(sessionRoot, session, controller.signal, budget.readyTimeoutMs, { intervalMs: budget.progressIntervalMs, report: progress => console.log(serviceReadyProgressLine(DEMONSTRATOR_E2E_OWNER, progress, language)) });
    const output = process.env.SEMIO_TICKET_DIR ? join(process.env.SEMIO_TICKET_DIR, "🗑️generated/demonstrator-e2e", session.id) : join(root, "dist/reports/e2e", session.id);
    console.log(`Testing Demonstrator at ${baseURL}`);
    const child = Bun.spawn([process.execPath, join(context.repoRoot, "node_modules/playwright/cli.js"), "test", "--config", join(root, "🔨️modules/🧪️e2e/🎚️config/🟦️.ts"), "--output", output], { cwd: context.repoRoot, env: { ...process.env, PLAYWRIGHT_BASE_URL: baseURL, PLAYWRIGHT_BROWSERS_PATH: repoCacheDirectory(context.repoRoot, "tools", "ms-playwright") }, stdout: "inherit", stderr: "inherit", stdin: "ignore" });
    const cancel = (): void => { child.kill("SIGTERM"); };
    controller.signal.addEventListener("abort", cancel, { once: true });
    if (controller.signal.aborted) cancel();
    let status: number;
    try { status = await child.exited; }
    finally { controller.signal.removeEventListener("abort", cancel); }
    if (status !== 0 && !controller.signal.aborted) throw new Error(`Demonstrator acceptance tests failed (${status})`);
  } finally { process.removeListener("SIGINT", interrupt); process.removeListener("SIGTERM", terminate); }
}
