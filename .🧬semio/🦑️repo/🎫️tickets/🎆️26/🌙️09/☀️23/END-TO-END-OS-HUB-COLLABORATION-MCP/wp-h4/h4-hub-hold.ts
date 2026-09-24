#!/usr/bin/env bun
/** 🌎️ H4 hub hold — detached-safe. Usage:
 *   nohup bun g4-hub-hold.ts <port> <dataDir> <binary> <pidFile> <statusFile> >>log 2>&1 </dev/null &
 */
import { existsSync, mkdirSync, writeFileSync, appendFileSync, readdirSync } from "node:fs";
import { dirname, join } from "node:path";

function findRepoRoot(start: string): string {
  let current = start;
  for (let depth = 0; depth < 32; depth += 1) {
    if (existsSync(join(current, ".mcp.json"))) return current;
    const parent = dirname(current);
    if (parent === current) break;
    current = parent;
  }
  throw new Error("probe could not locate the repository root above " + start);
}

function resolveHubRoot(repoRoot: string): string {
  for (const name of readdirSync(repoRoot)) {
    const candidate = join(repoRoot, name);
    if (existsSync(join(candidate, "🚀️local-bootstrap", "🏃️execution", "🟦️.ts"))) return candidate;
  }
  throw new Error(`no hub root under ${repoRoot}; looked for local-bootstrap`);
}

const repoRoot = findRepoRoot(import.meta.dir);
const hubRoot = resolveHubRoot(repoRoot);
const hubRust = join(hubRoot, "📦️packages", "🦀️rust");
const { startLocalHub, waitForReadiness, finishLocalHub } = await import(join(hubRoot, "🚀️local-bootstrap", "🏃️execution", "🟦️.ts"));

const STALL_MS = 600_000;
const port = Number(process.argv[2] ?? 7681);
const dataRoot = process.argv[3]!;
const binaryPath = process.argv[4]!;
const pidFile = process.argv[5]!;
const statusFile = process.argv[6]!;
const captureFile = statusFile.replace(/\.txt$/, "-capture.txt");
mkdirSync(dataRoot, { recursive: true });
if (!existsSync(binaryPath)) throw new Error(`build the hub first: ${binaryPath} does not exist`);

const status = (line: string) => {
  const stamped = `${new Date().toISOString()} ${line}`;
  console.log(stamped);
  writeFileSync(statusFile, `${stamped}\n`);
};

const flushCapture = (run: { output: () => string }, tag: string) => {
  const text = run.output();
  writeFileSync(captureFile, `${new Date().toISOString()} ${tag} bytes=${text.length}\n${text}\n`);
};

process.env.OS_HUB_CREDENTIAL_SIGN_IN = "true";
status(`starting port=${port} data=${dataRoot} bin=${binaryPath}`);

const run = await startLocalHub(repoRoot, hubRust, [{ profileId: "developer", subject: "local-developer-h4", displayName: "Local Developer", allowedClientClasses: ["native", "mcp"] }], {
  port,
  dataDir: dataRoot,
  binaryPath,
  capture: true,
});
writeFileSync(pidFile, `${process.pid}\n${run.child.pid ?? ""}\n`);
status(`SPAWNED holdPid=${process.pid} childPid=${run.child.pid} port=${run.port} runId=${run.runId}`);
run.child.on("exit", (code, signal) => {
  flushCapture(run, `CHILD_EXIT code=${code} signal=${signal}`);
  status(`CHILD_EXIT code=${code} signal=${signal}`);
});

const heartbeat = setInterval(() => {
  flushCapture(run, "heartbeat");
  status(`heartbeat childExit=${run.child.exitCode} outputBytes=${run.output().length}`);
}, 15_000);

try {
  const readiness = await waitForReadiness(run, false, STALL_MS);
  status(`HOLD origin=http://127.0.0.1:${port} status=${readiness.status} mcpWorkspace=${readiness.features?.mcpWorkspace}`);
  writeFileSync(statusFile.replace(/\.txt$/, "-ready.json"), `${JSON.stringify(readiness)}\n`);
  flushCapture(run, "HOLD");
  console.log(`READINESS ${JSON.stringify(readiness)}`);
  await new Promise(() => undefined);
} catch (error) {
  clearInterval(heartbeat);
  flushCapture(run, "WAIT_FAIL");
  status(`WAIT_FAIL ${error instanceof Error ? error.message : String(error)}`);
  console.log(`OUTPUT_TAIL\n${run.output().slice(-6000)}`);
  try {
    await finishLocalHub(run);
  } catch {}
  process.exit(1);
}
