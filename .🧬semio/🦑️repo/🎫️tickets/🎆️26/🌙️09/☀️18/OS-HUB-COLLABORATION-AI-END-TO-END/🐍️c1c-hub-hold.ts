/** 🌎️ Slice C1c — holds ONE live hub with credential sign-in enabled and the two collaboration
 * humans provisioned, so a browser probe can drive the real per-user identity path against it.
 *
 * It goes through `startLocalHub` rather than spawning the binary bare: a development hub completes
 * an authenticated local-bootstrap handshake over an inherited pipe on fd 3, and without it the
 * process aborts with `unexpected error when polling the I/O driver: Bad file descriptor` before it
 * binds a port. Readiness is awaited with `bootstrapSecuritySmoke`, which admits the
 * `artifactAuthority.ready === false` boundary — the narrowest legitimate configuration available
 * while the trusted stdio catalog is blocked (slice DS1).
 *
 * Usage: `bun 🐍️c1c-hub-hold.ts <port> <absoluteDataDir>` */
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync } from "node:fs";
import { dirname, join } from "node:path";

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
const { startLocalHub, waitForReadiness } = await import(join(repoRoot, "🌎️hub", "🚀️local-bootstrap", "🏃️execution", "🟦️.ts"));

const port = Number(process.argv[2] ?? 7501);
const dataRoot = process.argv[3]!;
mkdirSync(dataRoot, { recursive: true });
const binaryPath = join(hubRustRoot, "dist", "build-dev", "os-hub");
if (!existsSync(binaryPath)) throw new Error(`stage the hub first: ${binaryPath} does not exist`);

const ACCOUNTS = [
  { email: "user1@semio.dev", password: "collab e2e first human phrase", display: "Collab User One" },
  { email: "user2@semio.dev", password: "collab e2e second human phrase", display: "Collab User Two" },
];

for (const account of ACCOUNTS) {
  const result = spawnSync(binaryPath, ["credential", "set", "--email", account.email, "--display-name", account.display], {
    env: { ...process.env, OS_HUB_DATA: dataRoot },
    input: account.password,
    encoding: "utf8",
  });
  if (result.status !== 0) throw new Error(`credential set failed for ${account.email}: ${result.stderr}`);
  console.log(`provisioned ${account.email}=${result.stdout.trim()}`);
}

process.env.OS_HUB_CREDENTIAL_SIGN_IN = "1";
process.env.OS_HUB_ADMIN_TOKEN = "c1c-admin";
const run = await startLocalHub(repoRoot, hubRustRoot, [{ profileId: "developer", subject: "local-developer-c1c", displayName: "Local Developer", allowedClientClasses: ["native", "mcp"] }], {
  port,
  dataDir: dataRoot,
  binaryPath,
  capture: false,
});
const readiness = await waitForReadiness(run, true);
console.log(`HOLD origin=http://127.0.0.1:${port} dataRoot=${dataRoot} status=${readiness.status} artifactAuthority=${JSON.stringify(readiness.artifactAuthority)}`);
await new Promise(() => undefined);
