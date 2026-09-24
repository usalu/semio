#!/usr/bin/env bun
/** Durable M10b hub park — stays in foreground of a long-lived shell. */
import { appendFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const wp = "/Users/ueli/Documents/semio/.tmp-ticket/wp-m10b";
const hubRoot = `${wp}/links/hub`;
const hubRust = join(hubRoot, "📦️packages", "🦀️rust");
const { startLocalHub, waitForReadiness } = await import(`${hubRoot}/🚀️local-bootstrap/🏃️execution/🟦️.ts`);

const data = process.argv[2]!;
const bin = process.argv[3]!;
const port = Number(process.argv[4] ?? 7651);
const statusFile = `${wp}/generated/hub-${port}-park-status.txt`;
const captureFile = `${wp}/generated/hub-${port}-park-capture.txt`;
process.env.OS_HUB_CREDENTIAL_SIGN_IN = "true";

const log = (m: string) => {
  const line = `${new Date().toISOString()} ${m}`;
  console.log(line);
  writeFileSync(statusFile, `${line}\n`);
  appendFileSync(`${wp}/generated/hub-${port}-park.log`, `${line}\n`);
};

log(`park start port=${port} data=${data}`);
const run = await startLocalHub("/Users/ueli/Documents/semio", hubRust, [{ profileId: "developer", subject: "local-developer-m10b", displayName: "Local Developer", allowedClientClasses: ["native", "mcp"] }], {
  port,
  dataDir: data,
  binaryPath: bin,
  capture: true,
});
writeFileSync(`${wp}/generated/hub-${port}.pids`, `${process.pid}\n${run.child.pid ?? ""}\n`);
log(`SPAWNED hold=${process.pid} child=${run.child.pid} runId=${run.runId}`);
run.child.on("exit", (c, s) => {
  writeFileSync(captureFile, run.output());
  log(`CHILD_EXIT code=${c} signal=${s}`);
});
setInterval(() => {
  writeFileSync(captureFile, run.output());
  log(`heartbeat exit=${run.child.exitCode} bytes=${run.output().length}`);
}, 20_000);
const ready = await waitForReadiness(run, false, 600_000);
log(`HOLD mcpWorkspace=${ready.features?.mcpWorkspace} status=${ready.status}`);
writeFileSync(`${wp}/generated/hub-${port}-status-ready.json`, `${JSON.stringify(ready)}\n`);
await new Promise(() => undefined);
