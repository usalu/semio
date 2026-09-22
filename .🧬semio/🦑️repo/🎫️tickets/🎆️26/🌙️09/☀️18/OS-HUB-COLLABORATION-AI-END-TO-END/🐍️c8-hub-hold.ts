/** 🌍️ C8 — a local-bootstrap hub hold whose WAITER never kills the hub.
 *
 * A development hub only lives while its parent holds the authenticated local-bootstrap handshake on
 * fd 3: when the holder process ends, the hub exits with
 * `UnsafeAuthConfiguration("local bootstrap endpoint closed")`. Both existing holds end themselves on
 * a readiness clock — `🐍️ds1-hub-hold.ts` through `waitForReadiness`'s 30 s stall bound and
 * `🐍️pr1-hub-hold.ts` on an absolute 300 s — so on a loaded machine (measured 2026-09-22 16:1x:
 * load average 227–270, 47 cargo / 19 rustc) the waiter throws, the pipe closes, and a hub that was
 * still opening its catalog is torn down. The readiness wait is a REPORT, not a lifetime: this one
 * polls forever, prints every change, and ends only when the child itself exits.
 *
 * Usage: `bun 🐍️c8-hub-hold.ts <port> <absoluteDataDir> <absoluteBinaryPath>` */
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

const port = Number(process.argv[2] ?? 7671);
const dataRoot = process.argv[3]!;
const binaryPath = process.argv[4]!;
mkdirSync(dataRoot, { recursive: true });
if (!existsSync(binaryPath)) throw new Error(`build the hub first: ${binaryPath} does not exist`);

const run = await startLocalHub(repoRoot, hubRustRoot, [{ profileId: "developer", subject: "local-developer-c8", displayName: "Local Developer", allowedClientClasses: ["native", "mcp"] }], {
  port,
  dataDir: dataRoot,
  binaryPath,
  capture: false,
});
console.log(`STARTED pid=${run.child.pid} port=${port} dataRoot=${dataRoot}`);

let observation = "";
let ready = false;
while (run.child.exitCode === null) {
  let next = "no-answer";
  try {
    const response = await fetch(`http://127.0.0.1:${port}/readyz`, { signal: AbortSignal.timeout(4000) });
    const body = (await response.json()) as Record<string, any>;
    const admitted = localHubReadinessAdmitted(body, response.status, run.runId, true, run.publicSessionIssuance);
    next = `${body.status} artifactAuthority=${JSON.stringify(body.artifactAuthority)} blockedBy=${JSON.stringify(body.blockedBy ?? [])} admitted=${admitted}`;
    if (admitted && !ready) {
      ready = true;
      console.log(`HOLD origin=http://127.0.0.1:${port} pid=${run.child.pid} dataRoot=${dataRoot} status=${body.status} features=${JSON.stringify(body.features)}`);
      console.log(`READINESS ${JSON.stringify(body)}`);
    }
  } catch {
    next = "no-answer";
  }
  if (next !== observation) {
    observation = next;
    console.log(`${new Date().toISOString()} ${next}`);
  }
  await new Promise<void>((resolve) => setTimeout(resolve, ready ? 15_000 : 2_000));
}
console.log(`EXITED status=${run.child.exitCode}`);
