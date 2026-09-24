/**
 * 🚪️ Live probe (H9 item f): boots this tree's os-hub on a copy of catalog B, closes the launcher pipe while the hub
 * is still loading the catalog, and measures how long the hub takes to exit and what it reports.
 * usage: bun eof-probe.ts <os-hub> <catalog trusted-catalog dir> <data parent> <port> <close-after-ms> <capture.json>
 */
import { cpSync, mkdirSync, mkdtempSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { startLocalHub } from "../../../../../../../../🌎️hub/🚀️local-bootstrap/🏃️execution/🟦️.ts";

const [binary, catalog, parent, portText, closeAfterText, capture] = process.argv.slice(2);
const repoRoot = realpathSync(join(import.meta.dir, "../../../../../../../.."));
mkdirSync(parent!, { recursive: true, mode: 0o700 });
const dataRoot = mkdtempSync(join(realpathSync(parent!), "eof-"));
cpSync(catalog!, join(dataRoot, "trusted-catalog"), { recursive: true });
process.env.SEMIO_TRACE_LEVEL = "info";
const run = await startLocalHub(repoRoot, join(repoRoot, "🌎️hub/📦️packages/🦀️rust"), [{ profileId: "a", subject: "author-a", displayName: "Author A", allowedClientClasses: ["native"] }], { port: Number(portText), dataDir: dataRoot, binaryPath: binary, capture: true });
await new Promise((resolve) => setTimeout(resolve, Number(closeAfterText)));
const loading = !run.output().includes("\"server.readiness\"");
const closedAt = Date.now();
run.pipe.end();
const exit = await new Promise<{ code: number | null; signal: string | null }>((resolve) => {
  if (run.child.exitCode !== null) return resolve({ code: run.child.exitCode, signal: run.child.signalCode });
  const timer = setTimeout(() => resolve({ code: null, signal: "still-running-after-30s" }), 30_000);
  run.child.once("exit", (code: number | null, signal: string | null) => {
    clearTimeout(timer);
    resolve({ code, signal });
  });
});
const exitMs = Date.now() - closedAt;
if (exit.signal === "still-running-after-30s") run.child.kill("SIGKILL");
const trace = run.output().split("\n").filter((line: string) => line.includes("\"server.shutdown\"") || line.includes("\"server.catalog.publication\""));
const result = { closedWhileLoading: loading, exitAfterEofMs: exitMs, exit, lastCatalogRecord: trace.filter((line: string) => line.includes("catalog")).at(-1), shutdownRecords: trace.filter((line: string) => line.includes("server.shutdown")) };
writeFileSync(capture!, JSON.stringify(result, null, 2));
console.log(JSON.stringify(result, null, 2));
rmSync(dataRoot, { recursive: true, force: true });
process.exit(0);
