#!/usr/bin/env bun
/** 🌎️ W2 canonical hub hold: owns the local-bootstrap pipe for one hub and SUPERVISES it on its existing data root.
 * Usage (detached): python3 w2-detach.py <stateDir>/hold.txt bun w2-hub-hold.ts <port> <dataDir> <binary> <stateDir>
 * Writes <stateDir>/{pids.txt,status.txt,ready.json,capture.txt,admin-capability.json,restarts.txt}. Credential sign-in on; the
 * launcher profile is an admin subject and issues one `admin-relay` session (15 min) at each readiness and a fresh one whenever a
 * caller creates <stateDir>/admin-request (consumed). The hub admits at most 64 pipe exchanges per run
 * (`LOCAL_BOOTSTRAP_REPLAY_MAX`) and exits on the 65th, so issuance is on demand, never periodic.
 * Supervision (coordinator R2): when the hub child exits without a stop request, the hold restarts it on the SAME data root and
 * binary (catalog and users are durable there, nothing is re-derived) after a 5 s backoff, at most RESTART_MAX times per
 * RESTART_WINDOW_MS; a crash loop beyond that ends the hold (exit 1) with the last capture kept. Creating <stateDir>/stop ends the
 * hold cleanly (pipe EOF → hub exit). Readiness is bounded by no progress for TRUSTED_CATALOG_READINESS_STALL_BOUND_MS. */
import { chmodSync, existsSync, mkdirSync, rmSync, writeFileSync, appendFileSync, readdirSync } from "node:fs";
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
const { startLocalHub, waitForReadiness, finishLocalHub, TRUSTED_CATALOG_READINESS_STALL_BOUND_MS } = await import(join(hubRoot, "🚀️local-bootstrap", "🏃️execution", "🟦️.ts"));
const { issueLocalCredential } = await import(join(hubRoot, "🚀️local-bootstrap", "🔐️credential-issuance", "🟦️.ts"));
const PROFILE = { profileId: "developer", subject: "local-developer-w2", displayName: "Local Developer", allowedClientClasses: ["native", "mcp", "react-relay", "admin-relay"] };
const RESTART_MAX = 5;
const RESTART_WINDOW_MS = 30 * 60_000;
const RESTART_BACKOFF_MS = 5_000;

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
const stopFile = join(stateDir, "stop");
const requestFile = join(stateDir, "admin-request");
rmSync(stopFile, { force: true });

process.env.OS_HUB_CREDENTIAL_SIGN_IN = "true";
const restarts: number[] = [];
let generation = 0;

async function superviseOnce(): Promise<"stopped" | "exited"> {
  generation += 1;
  status(`starting generation=${generation} port=${port} data=${dataRoot} bin=${binaryPath}`);
  const run = await startLocalHub(repoRoot, hubRust, [PROFILE], { port, dataDir: dataRoot, binaryPath, capture: true, adminSubjects: [`semio.local.bootstrap/v1:${PROFILE.subject}`] });
  let sequence = 2;
  const issueAdmin = async () => {
    const envelope = await issueLocalCredential(run, PROFILE.profileId, "admin-relay", sequence);
    sequence += 1;
    const path = join(stateDir, "admin-capability.json");
    writeFileSync(path, `${JSON.stringify({ origin: `http://127.0.0.1:${port}`, capability: envelope.capability, sessionId: envelope.sessionId, expiresAt: envelope.expiresAt })}\n`, { mode: 0o600 });
    chmodSync(path, 0o600);
    status(`ADMIN issued generation=${generation} session=${envelope.sessionId} expiresAt=${envelope.expiresAt}`);
  };
  writeFileSync(join(stateDir, "pids.txt"), `hold=${process.pid}\nhub=${run.child.pid ?? ""}\nrunId=${run.runId}\ngeneration=${generation}\n`);
  status(`SPAWNED hold=${process.pid} hub=${run.child.pid} port=${run.port} runId=${run.runId} generation=${generation}`);
  let stopping = false;
  const exited = new Promise<"stopped" | "exited">((resolve) => {
    run.child.on("exit", (code: number | null, signal: string | null) => {
      flushCapture(run, `CHILD_EXIT generation=${generation} code=${code} signal=${signal}`);
      status(`CHILD_EXIT generation=${generation} code=${code} signal=${signal}${stopping ? " (requested)" : ""}`);
      resolve(stopping ? "stopped" : "exited");
    });
  });
  const heartbeat = setInterval(() => flushCapture(run, "heartbeat"), 30_000);
  const control = setInterval(() => {
    if (existsSync(stopFile) && !stopping) {
      stopping = true;
      status("STOP requested");
      void finishLocalHub(run).catch(() => undefined);
      return;
    }
    if (!existsSync(requestFile)) return;
    rmSync(requestFile, { force: true });
    if (sequence > 60) return status("ADMIN_REFUSED pipe exchange budget reserved");
    issueAdmin().catch((error) => status(`ADMIN_FAIL ${error instanceof Error ? error.message : String(error)}`));
  }, 5_000);
  try {
    const readiness = await Promise.race([waitForReadiness(run, false, TRUSTED_CATALOG_READINESS_STALL_BOUND_MS), exited.then(() => undefined)]);
    if (readiness) {
      writeFileSync(join(stateDir, "ready.json"), `${JSON.stringify(readiness, null, 2)}\n`);
      flushCapture(run, "HOLD");
      status(`HOLD origin=http://127.0.0.1:${port} hub=${run.child.pid} status=${readiness.status} generation=${generation}`);
      await issueAdmin();
    }
  } catch (error) {
    flushCapture(run, "WAIT_FAIL");
    status(`WAIT_FAIL generation=${generation} ${error instanceof Error ? error.message : String(error)}`);
    stopping = true;
    await finishLocalHub(run).catch(() => undefined);
    clearInterval(heartbeat);
    clearInterval(control);
    return "exited";
  }
  const outcome = await exited;
  clearInterval(heartbeat);
  clearInterval(control);
  await finishLocalHub(run).catch(() => undefined);
  return outcome;
}

for (;;) {
  const outcome = await superviseOnce();
  if (outcome === "stopped") {
    status("HOLD_END stopped on request");
    process.exit(0);
  }
  const now = Date.now();
  while (restarts.length && now - restarts[0]! > RESTART_WINDOW_MS) restarts.shift();
  if (restarts.length >= RESTART_MAX) {
    status(`HOLD_END crash loop: ${restarts.length} restarts within ${RESTART_WINDOW_MS / 60_000} min`);
    process.exit(1);
  }
  restarts.push(now);
  appendFileSync(join(stateDir, "restarts.txt"), `${new Date(now).toISOString()} generation=${generation} restart ${restarts.length}/${RESTART_MAX}\n`);
  status(`RESTART in ${RESTART_BACKOFF_MS} ms on the same data root (restart ${restarts.length}/${RESTART_MAX})`);
  await Bun.sleep(RESTART_BACKOFF_MS);
}
