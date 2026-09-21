/** 🌎️ Slice PR1 — holds one live hub the same way `🐍️ds1-hub-hold.ts` does, but polls `/readyz`
 * itself for up to five minutes instead of inheriting `waitForReadiness`'s 30 s deadline. Under
 * fleet load a cold GM1 boot needs longer than 30 s to verify its trusted catalog, and the shared
 * helper's throw tears the freshly started child down with it.
 *
 * Usage: `bun 🐍️pr1-hub-hold.ts <port> <absoluteDataDir> <absoluteBinaryPath>` */
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
const { startLocalHub, localHubReadinessAdmitted } = await import(join(repoRoot, "🌎️hub", "🚀️local-bootstrap", "🏃️execution", "🟦️.ts"));

const port = Number(process.argv[2] ?? 7611);
const dataRoot = process.argv[3]!;
const binaryPath = process.argv[4]!;
mkdirSync(dataRoot, { recursive: true });
if (!existsSync(binaryPath)) throw new Error(`build the hub first: ${binaryPath} does not exist`);

const run = await startLocalHub(repoRoot, hubRustRoot, [{ profileId: "developer", subject: "local-developer-pr1", displayName: "Local Developer", allowedClientClasses: ["native", "mcp"] }], {
  port,
  dataDir: dataRoot,
  binaryPath,
  capture: true,
});
console.log(`STARTED pid=${run.child.pid} port=${port} dataRoot=${dataRoot}`);
const deadline = Date.now() + 300_000;
let readiness: Record<string, unknown> | null = null;
while (Date.now() < deadline && readiness === null) {
  if (run.child.exitCode !== null) throw new Error(`hub exited before readiness with status ${run.child.exitCode}\n${run.output().slice(-4000)}`);
  try {
    const response = await fetch(`http://127.0.0.1:${port}/readyz`, { signal: AbortSignal.timeout(2000) });
    const body = (await response.json()) as Record<string, any>;
    if (localHubReadinessAdmitted(body, response.status, run.runId, true, run.publicSessionIssuance)) readiness = body;
    else console.log(`WAITING status=${body.status} artifactAuthority=${JSON.stringify(body.artifactAuthority)} blockedBy=${JSON.stringify(body.blockedBy ?? [])}`);
  } catch {
    /* 🕰️ a hub that has not bound yet answers nothing; keep polling until the deadline. */
  }
  await new Promise<void>((resolve) => setTimeout(resolve, 1000));
}
if (readiness === null) throw new Error("hub never became ready within 300 s");
console.log(`HOLD origin=http://127.0.0.1:${port} pid=${run.child.pid} status=${readiness.status} artifactAuthority=${JSON.stringify(readiness.artifactAuthority)}`);
console.log(`READINESS ${JSON.stringify(readiness)}`);
await new Promise(() => undefined);
