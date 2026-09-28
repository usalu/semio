#!/usr/bin/env bun
/**
 * 🛡️ R10 live proof (session 14b) of the session broker's admin capability: a hub on 8120 over an APFS clone of the B3
 * root with the B3 binary (the dev binary is the chain's to rebuild), the development profiles + administrator, the
 * broker with `adminProfileId` → the `0600` file appears, parses, opens `/admin/api/documents`; an `admin-request` yields
 * a fresh session; a `developer` broker session still issues; `stop` removes the file. Never prints the capability.
 * Usage: bun admin-capability-probe.ts
 */
import { cpSync, existsSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { join } from "node:path";
import { LOCAL_ADMIN_CAPABILITY_FILE, LOCAL_ADMIN_REQUEST_FILE, parseLocalAdminCapabilityV1, requestLocalBrokerSession, startLocalSessionBroker } from "/Users/ueli/Documents/semio/🌎️hub/🚀️local-bootstrap/🔐️credential-issuance/🟦️.ts";
import { finishLocalHub, LOCAL_HUB_ADMINISTRATOR_PROFILE, LOCAL_HUB_ADMINISTRATOR_SUBJECT, LOCAL_HUB_DEVELOPMENT_PROFILES, startLocalHub, TRUSTED_CATALOG_READINESS_STALL_BOUND_MS, waitForReadiness } from "/Users/ueli/Documents/semio/🌎️hub/🚀️local-bootstrap/🏃️execution/🟦️.ts";

const ROOT = "/Users/ueli/Documents/semio";
const HUB = join(ROOT, ".🧬semio/🌐hub");
const dataDir = join(HUB, "s14-r10-admin-root");
const binaryPath = join(HUB, "s13-w3-bin/s13-w3-hub-7800-b3/os-hub");
const port = 8120;
const checks: Record<string, boolean | string | number> = {};
const pause = (ms: number) => new Promise((resolveDelay) => setTimeout(resolveDelay, ms));
async function until<T>(read: () => T | null, boundMs: number): Promise<T | null> {
  for (const deadline = Date.now() + boundMs; Date.now() < deadline; await pause(250)) {
    const value = read();
    if (value !== null) return value;
  }
  return null;
}
const readFile = (): ReturnType<typeof parseLocalAdminCapabilityV1> | null => {
  try {
    return parseLocalAdminCapabilityV1(JSON.parse(readFileSync(join(dataDir, LOCAL_ADMIN_CAPABILITY_FILE), "utf8")));
  } catch {
    return null;
  }
};

if (spawnSync("lsof", ["-nP", `-iTCP:${port}`, "-sTCP:LISTEN"]).stdout.toString().trim()) throw new Error(`port ${port} is taken`);
rmSync(dataDir, { recursive: true, force: true });
const clone = spawnSync("cp", ["-c", "-R", join(HUB, "s13-w3-hub-7800-b3"), dataDir]);
if (clone.status !== 0) throw new Error(`clone failed: ${clone.stderr}`);
for (const stale of ["local-session-broker.json", LOCAL_ADMIN_CAPABILITY_FILE, LOCAL_ADMIN_REQUEST_FILE]) rmSync(join(dataDir, stale), { force: true });
const profiles = [...LOCAL_HUB_DEVELOPMENT_PROFILES, LOCAL_HUB_ADMINISTRATOR_PROFILE];
const started = Date.now();
const run = await startLocalHub(ROOT, join(ROOT, "🌎️hub/📦️packages/🦀️rust"), profiles, { port, dataDir, binaryPath, adminSubjects: [LOCAL_HUB_ADMINISTRATOR_SUBJECT], capture: true });
let broker: ReturnType<typeof startLocalSessionBroker> | null = null;
try {
  await waitForReadiness(run, false, TRUSTED_CATALOG_READINESS_STALL_BOUND_MS);
  checks.readySeconds = Math.round((Date.now() - started) / 1000);
  broker = startLocalSessionBroker(run, dataDir, profiles, 2, LOCAL_HUB_ADMINISTRATOR_PROFILE.profileId);
  const first = await until(readFile, 30_000);
  checks.fileIssued = first !== null;
  checks.fileMode = first ? (statSync(join(dataDir, LOCAL_ADMIN_CAPABILITY_FILE)).mode & 0o777).toString(8) : "none";
  checks.origin = first?.origin === `http://127.0.0.1:${port}`;
  const admin = first ? await fetch(`http://127.0.0.1:${port}/admin/api/documents`, { headers: { authorization: `Bearer ${first.capability}` }, signal: AbortSignal.timeout(30_000) }).catch(() => null) : null;
  checks.adminApiStatus = admin?.status ?? "unreachable";
  const anonymous = await fetch(`http://127.0.0.1:${port}/admin/api/documents`, { signal: AbortSignal.timeout(30_000) }).catch(() => null);
  checks.anonymousStatus = anonymous?.status ?? "unreachable";
  writeFileSync(join(dataDir, LOCAL_ADMIN_REQUEST_FILE), "");
  const second = await until(() => {
    const now = readFile();
    return now && now.sessionId !== first?.sessionId ? now : null;
  }, 20_000);
  checks.refreshed = second !== null;
  checks.requestConsumed = !existsSync(join(dataDir, LOCAL_ADMIN_REQUEST_FILE));
  const session = await requestLocalBrokerSession(dataDir, `http://127.0.0.1:${port}`, "developer").catch(() => null);
  checks.developerSession = session !== null && session.profileId === "developer";
  broker.stop();
  broker = null;
  checks.fileRemovedOnStop = !existsSync(join(dataDir, LOCAL_ADMIN_CAPABILITY_FILE));
} finally {
  broker?.stop();
  await finishLocalHub(run);
  rmSync(dataDir, { recursive: true, force: true });
}
const pass = checks.fileIssued === true && checks.fileMode === "600" && checks.origin === true && checks.adminApiStatus === 200 && checks.anonymousStatus !== 200 && checks.refreshed === true && checks.requestConsumed === true && checks.developerSession === true && checks.fileRemovedOnStop === true;
console.log(JSON.stringify({ pass, ...checks }));
process.exitCode = pass ? 0 : 1;
