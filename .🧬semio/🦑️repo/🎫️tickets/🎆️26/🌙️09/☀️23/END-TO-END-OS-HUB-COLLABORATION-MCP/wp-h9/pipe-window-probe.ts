/**
 * 🪟️ Live probe (H9 item e): boots this tree's os-hub on a copy of catalog B, then issues local credentials over the
 * launcher pipe past the 64-exchange window. Before the fix the 65th exchange of a run made the hub exit; now the
 * window refuses with a signed `resource-limit` answer, the hub stays ready, and after one window the pipe admits again.
 * usage: bun pipe-window-probe.ts <os-hub> <catalog trusted-catalog dir> <data parent> <port> <capture.json>
 */
import { cpSync, mkdirSync, mkdtempSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { finishLocalHub, startLocalHub, waitForReadiness, TRUSTED_CATALOG_READINESS_STALL_BOUND_MS } from "../../../../../../../../🌎️hub/🚀️local-bootstrap/🏃️execution/🟦️.ts";
import { issueLocalCredential, LocalBootstrapRefusedError } from "../../../../../../../../🌎️hub/🚀️local-bootstrap/🔐️credential-issuance/🟦️.ts";

const [binary, catalog, parent, portText, capture] = process.argv.slice(2);
const repoRoot = realpathSync(join(import.meta.dir, "../../../../../../../.."));
mkdirSync(parent!, { recursive: true, mode: 0o700 });
const dataRoot = mkdtempSync(join(realpathSync(parent!), "window-"));
cpSync(catalog!, join(dataRoot, "trusted-catalog"), { recursive: true });
const run = await startLocalHub(repoRoot, join(repoRoot, "🌎️hub/📦️packages/🦀️rust"), [{ profileId: "a", subject: "author-a", displayName: "Author A", allowedClientClasses: ["native"] }], { port: Number(portText), dataDir: dataRoot, binaryPath: binary, capture: true });
const outcomes: string[] = [];
let sequence = 2;
try {
  await waitForReadiness(run, true, TRUSTED_CATALOG_READINESS_STALL_BOUND_MS);
  const burstStarted = Date.now();
  for (let index = 0; index < 70; index++) {
    try {
      await issueLocalCredential(run, "a", "native", sequence++);
      outcomes.push("issued");
    } catch (error) {
      outcomes.push(error instanceof LocalBootstrapRefusedError ? `refused:${error.code}` : `error:${(error as Error).message}`);
    }
  }
  const burstMs = Date.now() - burstStarted;
  await new Promise((resolve) => setTimeout(resolve, Math.max(0, 15_500 - (Date.now() - burstStarted))));
  let afterWindow = "issued";
  try {
    await issueLocalCredential(run, "a", "native", sequence++);
  } catch (error) {
    afterWindow = error instanceof LocalBootstrapRefusedError ? `refused:${error.code}` : `error:${(error as Error).message}`;
  }
  const ready = await fetch(`http://127.0.0.1:${run.port}/readyz`).then((response) => response.status).catch(() => 0);
  const counts: Record<string, number> = {};
  for (const outcome of outcomes) counts[outcome] = (counts[outcome] ?? 0) + 1;
  const result = { burstExchanges: outcomes.length, burstMs, counts, firstRefusalAt: outcomes.findIndex((outcome) => outcome !== "issued") + 1, afterWindow, hubAlive: run.child.exitCode === null, readyz: ready };
  writeFileSync(capture!, JSON.stringify(result, null, 2));
  console.log(JSON.stringify(result, null, 2));
} finally {
  await finishLocalHub(run).catch(() => {});
  rmSync(dataRoot, { recursive: true, force: true });
}
process.exit(0);
