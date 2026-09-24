#!/usr/bin/env bun
/** H1 probe: startLocalHub on prepared data root, curl /readyz, capture, stop. */
import { mkdirSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { finishLocalHub, startLocalHub, waitForReadiness } from '/Users/ueli/Documents/semio/🌎️hub/🚀️local-bootstrap/🏃️execution/🟦️.ts';

const repoRoot = resolve(import.meta.dir, "../..");
const hubRoot = resolve(process.env.OS_HUB_ROOT ?? '/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust');
const dataRoot = resolve(process.env.OS_HUB_DATA!);
const port = Number(process.env.OS_HUB_PORT ?? 7741);
const binaryPath = resolve(process.env.OS_HUB_BIN!);
const outDir = join(import.meta.dir, "generated");
mkdirSync(outDir, { recursive: true });

const profiles = [{ profileId: "developer", subject: "local-developer-01", displayName: "Local Developer", allowedClientClasses: ["native", "mcp"] as const }];
const run = await startLocalHub(repoRoot, hubRoot, profiles, { port, dataDir: dataRoot, binaryPath, capture: true });
try {
  const body = await waitForReadiness(run);
  const text = JSON.stringify(body, null, 2);
  writeFileSync(join(outDir, "readyz.json"), text);
  console.log("READYZ_STATUS=" + body.status);
  console.log("ARTIFACT_AUTHORITY=" + JSON.stringify(body.artifactAuthority));
  console.log(text);
  if (body.status !== "ready" || body.artifactAuthority?.ready !== true) {
    writeFileSync(join(outDir, "readyz-hub-output.txt"), run.output().slice(-8000));
    throw new Error("hub /readyz not fully ready: " + body.status);
  }
} finally {
  await finishLocalHub(run);
}
