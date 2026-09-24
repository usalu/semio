/** 🌍️ WP-O3 / C9 hub hold — local-bootstrap + credential sign-in, waiter never kills the hub. */
import { existsSync, mkdirSync } from "node:fs";
import { dirname, join } from "node:path";

function findRepoRoot(start: string): string {
  let current = start;
  for (let depth = 0; depth < 32; depth += 1) {
    if (existsSync(join(current, "package.json")) && existsSync(join(current, "bun.lock"))) return current;
    const parent = dirname(current);
    if (parent === current) break;
    current = parent;
  }
  throw new Error("repo root not found above " + start);
}

const repoRoot = findRepoRoot(import.meta.dir);
const hubRustRoot = join(repoRoot, ...(await (async () => {
  const { readdirSync } = await import("node:fs");
  const hub = readdirSync(repoRoot).find((name) => name.includes("hub") && !name.startsWith("."));
  if (!hub) throw new Error("hub dir missing");
  const packages = readdirSync(join(repoRoot, hub)).find((name) => name.includes("packages"));
  const rust = readdirSync(join(repoRoot, hub, packages!)).find((name) => name.includes("rust"));
  return [hub, packages!, rust!];
})()));

process.env.OS_HUB_CREDENTIAL_SIGN_IN = "1";
process.env.OS_HUB_ADMIN_TOKEN = process.env.OS_HUB_ADMIN_TOKEN ?? "e2e-admin";

const { startLocalHub, localHubReadinessAdmitted } = await import(
  join(repoRoot, ...(await (async () => {
    const { readdirSync } = await import("node:fs");
    const hub = readdirSync(repoRoot).find((name) => name.includes("hub") && !name.startsWith("."))!;
    const lb = readdirSync(join(repoRoot, hub)).find((name) => name.includes("local-bootstrap"))!;
    const exec = readdirSync(join(repoRoot, hub, lb)).find((name) => name.includes("execution"))!;
    const file = readdirSync(join(repoRoot, hub, lb, exec)).find((name) => name.endsWith(".ts"))!;
    return [hub, lb, exec, file];
  })())),
);

const port = Number(process.argv[2] ?? 7701);
const dataRoot = process.argv[3]!;
const binaryPath = process.argv[4]!;
mkdirSync(dataRoot, { recursive: true });
if (!existsSync(binaryPath)) throw new Error(`missing binary: ${binaryPath}`);

const run = await startLocalHub(
  repoRoot,
  hubRustRoot,
  [{ profileId: "developer", subject: "local-developer-o3", displayName: "Local Developer", allowedClientClasses: ["native", "mcp"] }],
  { port, dataDir: dataRoot, binaryPath, capture: false },
);
console.log(`STARTED pid=${run.child.pid} port=${port} dataRoot=${dataRoot}`);

let observation = "";
let ready = false;
while (run.child.exitCode === null) {
  let next = "no-answer";
  try {
    const response = await fetch(`http://127.0.0.1:${port}/readyz`, { signal: AbortSignal.timeout(4000) });
    const body = (await response.json()) as Record<string, any>;
    const admitted = localHubReadinessAdmitted(body, response.status, run.runId, true, run.publicSessionIssuance);
    next = `${body.status} admitted=${admitted} features=${JSON.stringify(body.features)}`;
    if (admitted && !ready) {
      ready = true;
      console.log(`HOLD origin=http://127.0.0.1:${port} pid=${run.child.pid} status=${body.status}`);
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
