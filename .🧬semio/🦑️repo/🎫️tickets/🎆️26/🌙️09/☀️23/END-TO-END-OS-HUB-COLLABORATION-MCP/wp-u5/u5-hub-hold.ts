#!/usr/bin/env bun
/** 🌎️ U5 hub hold: owns the local-bootstrap pipe of one catalog-less loopback hub (sign-in, spaces, presence)
 * and never exits while it runs. Readiness admits the bootstrap-ready state (`artifactAuthority` not ready):
 * U5 needs sign-in, the directory and document sockets, not artifact creation.
 * Usage: nohup bun u5-hub-hold.ts <port> <dataDir> <binary> <stateDir> > <stateDir>/hold.txt 2>&1 < /dev/null & disown */
import { existsSync, mkdirSync, writeFileSync, readdirSync } from "node:fs";
import { dirname, join } from "node:path";

function findRepoRoot(start: string): string {
  let current = start;
  for (let depth = 0; depth < 32; depth += 1) {
    if (existsSync(join(current, ".mcp.json")) && existsSync(join(current, "bun.lock"))) return current;
    const parent = dirname(current);
    if (parent === current) break;
    current = parent;
  }
  throw new Error("repository root not found above " + start);
}

function resolveHubRoot(repoRoot: string): string {
  for (const name of readdirSync(repoRoot)) {
    const candidate = join(repoRoot, name);
    if (existsSync(join(candidate, "🚀️local-bootstrap", "🏃️execution", "🟦️.ts"))) return candidate;
  }
  throw new Error(`no hub root under ${repoRoot}`);
}

const repoRoot = findRepoRoot(import.meta.dir);
const hubRoot = resolveHubRoot(repoRoot);
const hubRust = join(hubRoot, "📦️packages", "🦀️rust");
const { startLocalHub, waitForReadiness, finishLocalHub } = await import(join(hubRoot, "🚀️local-bootstrap", "🏃️execution", "🟦️.ts"));

const port = Number(process.argv[2]);
const dataRoot = process.argv[3]!;
const binaryPath = process.argv[4]!;
const stateDir = process.argv[5]!;
mkdirSync(stateDir, { recursive: true });
if (!existsSync(binaryPath)) throw new Error(`missing binary: ${binaryPath}`);

const status = (line: string) => {
  const stamped = `${new Date().toISOString()} ${line}`;
  console.log(stamped);
  writeFileSync(join(stateDir, "status.txt"), `${stamped}\n`);
};
const flushCapture = (run: { output: () => string }, tag: string) => writeFileSync(join(stateDir, "capture.txt"), `${new Date().toISOString()} ${tag}\n${run.output().slice(-200_000)}\n`);

process.env.OS_HUB_CREDENTIAL_SIGN_IN = "true";
status(`starting port=${port} data=${dataRoot} bin=${binaryPath}`);
const run = await startLocalHub(repoRoot, hubRust, [{ profileId: "developer", subject: "local-developer-u5", displayName: "Local Developer", allowedClientClasses: ["native", "mcp", "react-relay"] }], {
  port,
  dataDir: dataRoot,
  binaryPath,
  capture: true,
  adminToken: "u5-admin",
});
writeFileSync(join(stateDir, "pids.txt"), `hold=${process.pid}\nhub=${run.child.pid ?? ""}\nrunId=${run.runId}\n`);
status(`SPAWNED hold=${process.pid} hub=${run.child.pid} port=${run.port} runId=${run.runId}`);
run.child.on("exit", (code: number | null, signal: string | null) => {
  flushCapture(run, `CHILD_EXIT code=${code} signal=${signal}`);
  status(`CHILD_EXIT code=${code} signal=${signal}`);
  process.exit(1);
});
setInterval(() => flushCapture(run, "heartbeat"), 30_000);

try {
  const readiness = await waitForReadiness(run, true, 300_000);
  writeFileSync(join(stateDir, "ready.json"), `${JSON.stringify(readiness, null, 2)}\n`);
  flushCapture(run, "HOLD");
  status(`HOLD origin=http://127.0.0.1:${port} hub=${run.child.pid} status=${readiness.status}`);
  await new Promise(() => undefined);
} catch (error) {
  flushCapture(run, "WAIT_FAIL");
  status(`WAIT_FAIL ${error instanceof Error ? error.message : String(error)}`);
  await finishLocalHub(run).catch(() => undefined);
  process.exit(1);
}
