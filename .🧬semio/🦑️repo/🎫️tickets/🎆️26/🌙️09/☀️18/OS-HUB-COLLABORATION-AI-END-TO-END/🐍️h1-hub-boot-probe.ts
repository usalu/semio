/** 🧪️ H1 boot probe: starts the Nx-staged `os-hub` development binary against a freshly created
 * `OS_HUB_DATA` with no `SEMIO_BUILD_BUDGET_MS` in the environment, waits for `/readyz` with `curl`,
 * reports every route class the router declares (health, directory/db, presence, auth) and shuts the
 * child down by pid. Pass `--catalog <path>` to seed the fresh data root with an already published
 * trusted-catalog generation so artifact authority can reach full readiness. */
import { spawnSync } from "node:child_process";
import { cpSync, existsSync, mkdtempSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";

function findRepoRoot(start: string): string {
  let current = start;
  for (let depth = 0; depth < 32; depth += 1) {
    if (existsSync(join(current, "🌎️hub", "📦️packages", "🦀️rust", "Cargo.toml"))) return current;
    const parent = dirname(current);
    if (parent === current) break;
    current = parent;
  }
  throw new Error("probe could not locate the repository root above " + start);
}

const repoRoot = findRepoRoot(import.meta.dir);
const hubRustRoot = join(repoRoot, "🌎️hub", "📦️packages", "🦀️rust");
const { finishLocalHub, hubDevBinaryPath, startLocalHub, waitForReadiness } = await import(join(repoRoot, "🌎️hub", "🚀️local-bootstrap", "🏃️execution", "🟦️.ts"));
const { issueLocalCredential } = await import(join(repoRoot, "🌎️hub", "🚀️local-bootstrap", "🔐️credential-issuance", "🟦️.ts"));

const seedCatalog = process.argv.includes("--catalog") ? resolve(process.argv[process.argv.indexOf("--catalog") + 1]) : undefined;
if (process.env.SEMIO_BUILD_BUDGET_MS !== undefined) throw new Error("probe requires an unset SEMIO_BUILD_BUDGET_MS (the repository's unlimited default)");
const port = Number(process.env.H1_HUB_PORT ?? 8814);
const dataRoot = mkdtempSync(join(tmpdir(), "h1-os-hub-data-"));
console.log(`fresh OS_HUB_DATA=${dataRoot} port=${port} SEMIO_BUILD_BUDGET_MS=${process.env.SEMIO_BUILD_BUDGET_MS ?? "<unset>"}`);
if (seedCatalog) {
  cpSync(seedCatalog, join(dataRoot, "trusted-catalog"), { recursive: true });
  console.log(`seeded trusted-catalog from ${seedCatalog}`);
}

const binaryPath = hubDevBinaryPath(hubRustRoot);
console.log(`staged binary: ${binaryPath}`);

function curl(path: string, token?: string): string {
  const args = ["-sS", "-o", "/dev/stdout", "-w", "\\n<status %{http_code}>", `http://127.0.0.1:${port}${path}`];
  if (token) args.splice(1, 0, "-H", `Authorization: Bearer ${token}`);
  const result = spawnSync("curl", args, { encoding: "utf8" });
  return (result.stdout ?? "").trim().slice(0, 600) + (result.stderr ? ` stderr=${result.stderr.trim().slice(0, 200)}` : "");
}

const run = await startLocalHub(repoRoot, hubRustRoot, [{ profileId: "developer", subject: "local-developer-01", displayName: "Local Developer", allowedClientClasses: ["native", "mcp"] }], {
  port,
  dataDir: dataRoot,
  binaryPath,
  capture: true,
});
console.log(`hub child pid=${run.child.pid} runId=${run.runId}`);
let failure: unknown;
try {
  const readiness = await waitForReadiness(run, true);
  console.log(`readyz(admitted): status=${readiness.status} artifactAuthority=${JSON.stringify(readiness.artifactAuthority)} directory=${JSON.stringify(readiness.directory)} storage=${JSON.stringify(readiness.storage)} adminAssets=${JSON.stringify(readiness.adminAssets)}`);
  const credential = await issueLocalCredential(run, "developer", "mcp");
  const token = credential.capability as string;
  for (const [label, path, authorized] of [
    ["health /readyz", "/readyz", false],
    ["health /healthz (undeclared)", "/healthz", false],
    ["auth /auth/sessions/me (no credential)", "/auth/sessions/me", false],
    ["auth /auth/sessions/me (local credential)", "/auth/sessions/me", true],
    ["directory /directory/spaces", "/directory/spaces", true],
    ["directory /directory/event-page/v1", "/directory/event-page/v1", true],
    ["presence /admin/api/connections", "/admin/api/connections", true],
    ["db /spaces/probe-space/documents/probe-document", "/spaces/probe-space/documents/probe-document", true],
  ] as const) {
    console.log(`${label}: ${curl(path, authorized ? token : undefined)}`);
  }
} catch (error) {
  failure = error;
  console.log(`probe failure: ${error instanceof Error ? error.message : String(error)}`);
  console.log(`child output tail:\n${run.output().slice(-2000)}`);
} finally {
  const pid = run.child.pid;
  await finishLocalHub(run);
  console.log(`hub child pid=${pid} exitCode=${run.child.exitCode}`);
  rmSync(dataRoot, { recursive: true, force: true });
}
if (failure) process.exitCode = 1;
