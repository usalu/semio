#!/usr/bin/env bun
/** H1: bind published trusted catalog (no wasm), start hub, prove /readyz ready. */
import { existsSync, mkdirSync, rmSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import {
  materializeTrustedCatalogBundle,
  trustedBootstrapSelectPackages,
  TRUSTED_BOOTSTRAP_LINKED_PACKAGES,
} from "./links/hub-rust-script.ts";
import { finishLocalHub, startLocalHub, waitForReadiness } from "./links/local-bootstrap-exec.ts";

const repoRoot = (() => {
  let dir = resolve(import.meta.dir);
  for (let i = 0; i < 12; i++) {
    if (existsSync(join(dir, "nx.json")) && existsSync(join(dir, "Cargo.toml"))) return dir;
    const parent = resolve(dir, "..");
    if (parent === dir) break;
    dir = parent;
  }
  throw new Error("could not locate monorepo root from " + import.meta.dir);
})();
const outDir = join(import.meta.dir, "generated");
mkdirSync(outDir, { recursive: true });

const hubPkg = resolve(import.meta.dir, "links/hub-rust-pkg");
const binaryPath = resolve(
  process.env.OS_HUB_BIN ?? join(hubPkg, "dist/build-dev/os-hub"),
);
const port = Number(process.env.OS_HUB_PORT ?? 7748);
const dataRoot = resolve(process.env.OS_HUB_DATA ?? join(outDir, "fresh-hub-data-bind"));
rmSync(dataRoot, { recursive: true, force: true });
mkdirSync(dataRoot, { recursive: true });

if (!process.env.OS_HUB_TRUSTED_CATALOG_SOURCE) {
  delete process.env.OS_HUB_TRUSTED_CATALOG_SOURCE;
}
console.log("BIND_SOURCE=" + (process.env.OS_HUB_TRUSTED_CATALOG_SOURCE ?? "(auto-discover)"));

const selection = trustedBootstrapSelectPackages(TRUSTED_BOOTSTRAP_LINKED_PACKAGES);
const t0 = Date.now();
const receipt = await materializeTrustedCatalogBundle(repoRoot, dataRoot, selection);
const bindMs = Date.now() - t0;
writeFileSync(join(outDir, "bind-receipt.json"), JSON.stringify({ ...receipt, bindMs, dataRoot, source: process.env.OS_HUB_TRUSTED_CATALOG_SOURCE ?? "auto-discover" }, null, 2));
console.log("BIND_MS=" + bindMs);
console.log("BIND_GENERATION=" + receipt.generationId);
console.log("BIND_BUNDLE=" + receipt.bundleSha256);
if (bindMs > 60_000) throw new Error("bind took >60s — likely rebuilt wasm instead of content-hash bind");

const profiles = [
  {
    profileId: "developer",
    subject: "local-developer-01",
    displayName: "Local Developer",
    allowedClientClasses: ["native", "mcp"] as const,
  },
];

const run = await startLocalHub(repoRoot, hubPkg, profiles, {
  port,
  dataDir: dataRoot,
  binaryPath,
  capture: true,
});
console.log("HUB_PID=" + run.child.pid + " PORT=" + run.port);
try {
  const probe = await fetch(`http://127.0.0.1:${run.port}/readyz`).then(async (r) => ({ status: r.status, text: await r.text() })).catch((e) => ({ error: String(e) }));
  console.log("EARLY_READYZ=" + JSON.stringify(probe).slice(0, 500));
  writeFileSync(join(outDir, "early-readyz.json"), JSON.stringify(probe, null, 2));
  const body = await waitForReadiness(run, false, 120_000);
  writeFileSync(join(outDir, "readyz.json"), JSON.stringify(body, null, 2));
  console.log("READYZ_STATUS=" + body.status);
  console.log("ARTIFACT_AUTHORITY=" + JSON.stringify(body.artifactAuthority));
  if (body.status !== "ready" || body.artifactAuthority?.ready !== true) {
    writeFileSync(join(outDir, "readyz-hub-output.txt"), run.output().slice(-8000));
    throw new Error("hub /readyz not fully ready: " + body.status);
  }
} catch (error) {
  writeFileSync(join(outDir, "readyz-hub-output.txt"), run.output().slice(-12000));
  console.error("HUB_OUTPUT_TAIL:\n" + run.output().slice(-4000));
  throw error;
} finally {
  await finishLocalHub(run);
}
console.log("H1_BIND_READYZ_OK");
