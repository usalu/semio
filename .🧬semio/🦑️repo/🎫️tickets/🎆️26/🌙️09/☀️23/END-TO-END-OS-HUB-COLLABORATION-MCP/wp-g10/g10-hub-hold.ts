#!/usr/bin/env bun
/** 🌎️ G10 hub hold (copy of W2's w2-hub-hold.ts): owns the local-bootstrap pipe for one hub and never exits while it runs.
 * Usage: nohup bun w2-hub-hold.ts <port> <dataDir> <binary> <stateDir> > <stateDir>/hold.txt 2>&1 < /dev/null & disown
 * Writes <stateDir>/{pids.txt,status.txt,ready.json,capture.txt,admin-capability.json}. Credential sign-in on; the launcher
 * profile is an admin subject and re-issues an `admin-relay` session through the pipe every 10 min (sessions live 15 min).
 * Readiness is bounded by 30 min of no progress (the fleet runs at load 50–60, where the hub's own 300 s bound starves). */
import { chmodSync, existsSync, mkdirSync, writeFileSync, readdirSync } from "node:fs";
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
const { issueLocalCredential } = await import(join(hubRoot, "🚀️local-bootstrap", "🔐️credential-issuance", "🟦️.ts"));
const PROFILE = { profileId: "developer", subject: "local-developer-g10", displayName: "Local Developer", allowedClientClasses: ["native", "mcp", "react-relay", "admin-relay"] };

const port = Number(process.argv[2]);
const dataRoot = process.argv[3]!;
const binaryPath = process.argv[4]!;
const stateDir = process.argv[5]!;
mkdirSync(stateDir, { recursive: true });
if (!existsSync(binaryPath)) throw new Error(`missing binary: ${binaryPath}`);
if (!existsSync(join(dataRoot, "trusted-catalog", "current.json"))) throw new Error(`no trusted catalog under ${dataRoot}`);

const status = (line: string) => {
  const stamped = `${new Date().toISOString()} ${line}`;
  console.log(stamped);
  writeFileSync(join(stateDir, "status.txt"), `${stamped}\n`);
};
const flushCapture = (run: { output: () => string }, tag: string) => writeFileSync(join(stateDir, "capture.txt"), `${new Date().toISOString()} ${tag}\n${run.output()}\n`);

process.env.OS_HUB_CREDENTIAL_SIGN_IN = "true";
status(`starting port=${port} data=${dataRoot} bin=${binaryPath}`);
const run = await startLocalHub(repoRoot, hubRust, [PROFILE], {
  port,
  dataDir: dataRoot,
  binaryPath,
  capture: true,
  adminSubjects: [`semio.local.bootstrap/v1:${PROFILE.subject}`],
});
let sequence = 2;
const issueAdmin = async () => {
  const envelope = await issueLocalCredential(run, PROFILE.profileId, "admin-relay", sequence);
  sequence += 1;
  const path = join(stateDir, "admin-capability.json");
  writeFileSync(path, `${JSON.stringify({ origin: `http://127.0.0.1:${port}`, capability: envelope.capability, sessionId: envelope.sessionId, expiresAt: envelope.expiresAt })}\n`, { mode: 0o600 });
  chmodSync(path, 0o600);
  status(`ADMIN issued session=${envelope.sessionId} expiresAt=${envelope.expiresAt}`);
};
writeFileSync(join(stateDir, "pids.txt"), `hold=${process.pid}\nhub=${run.child.pid ?? ""}\nrunId=${run.runId}\n`);
status(`SPAWNED hold=${process.pid} hub=${run.child.pid} port=${run.port} runId=${run.runId}`);
run.child.on("exit", (code: number | null, signal: string | null) => {
  flushCapture(run, `CHILD_EXIT code=${code} signal=${signal}`);
  status(`CHILD_EXIT code=${code} signal=${signal}`);
  process.exit(1);
});
setInterval(() => flushCapture(run, "heartbeat"), 30_000);

try {
  const readiness = await waitForReadiness(run, false, 1_800_000);
  writeFileSync(join(stateDir, "ready.json"), `${JSON.stringify(readiness, null, 2)}\n`);
  flushCapture(run, "HOLD");
  status(`HOLD origin=http://127.0.0.1:${port} hub=${run.child.pid} status=${readiness.status}`);
  await issueAdmin();
  setInterval(() => issueAdmin().catch((error) => status(`ADMIN_FAIL ${error instanceof Error ? error.message : String(error)}`)), 600_000);
  await new Promise(() => undefined);
} catch (error) {
  flushCapture(run, "WAIT_FAIL");
  status(`WAIT_FAIL ${error instanceof Error ? error.message : String(error)}`);
  await finishLocalHub(run).catch(() => undefined);
  process.exit(1);
}
