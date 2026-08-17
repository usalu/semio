#!/usr/bin/env bun
/** Runtime probe for block apps — boot, load page, collect console errors, assert canvas/root. */
import { chromium } from "@playwright/test";
import { spawn } from "node:child_process";
import { createWriteStream, existsSync, mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const ticketDir = process.argv[2];
const repoRoot = process.argv[3];
if (!ticketDir || !repoRoot) {
  console.error("usage: bun probe-runtime.mts <ticketDir> <repoRoot>");
  process.exit(2);
}

const apps = [
  { id: "block2d", port: 6024, env: "BLOCK_2D_PLAY_PORT", script: ["bun", "run", "dev:block:2d"] },
  { id: "block3d", port: 6025, env: "BLOCK_3D_PLAY_PORT", script: ["bun", "run", "dev:block:3d"] },
  { id: "block5d", port: 6026, env: "BLOCK_5D_PLAY_PORT", script: ["bun", "run", "dev:block:5d"] },
] as const;

const READY_MS = 25 * 60_000;

async function waitForPort(port: number, logPath: string, child: ReturnType<typeof spawn>): Promise<void> {
  const start = Date.now();
  while (Date.now() - start < READY_MS) {
    if (child.exitCode !== null) throw new Error(`dev server exited early (${child.exitCode}); see ${logPath}`);
    try {
      const res = await fetch(`http://127.0.0.1:${port}/`, { signal: AbortSignal.timeout(2000) });
      if (res.ok) return;
    } catch {}
    await new Promise((r) => setTimeout(r, 1500));
  }
  throw new Error(`timeout waiting for :${port}`);
}

async function probeOne(app: (typeof apps)[number]) {
  const logPath = join(ticketDir, `${app.id}-runtime-dev.log`);
  const out = createWriteStream(logPath);
  const env = { ...process.env, [app.env]: String(app.port), SKIP_ENGINE_BUILD: "1" };
  console.log(`[runtime] starting ${app.id}`);
  const child = spawn(app.script[0], app.script.slice(1), { cwd: repoRoot, env, stdio: ["ignore", "pipe", "pipe"], detached: true });
  child.stdout?.pipe(out);
  child.stderr?.pipe(out);
  try {
    await waitForPort(app.port, logPath, child);
    const browser = await chromium.launch({ headless: true });
    const page = await browser.newPage();
    const errors: string[] = [];
    const pageErrors: string[] = [];
    const failed: string[] = [];
    page.on("console", (msg) => {
      if (msg.type() === "error") errors.push(msg.text());
    });
    page.on("pageerror", (err) => pageErrors.push(String(err)));
    page.on("requestfailed", (req) => failed.push(`${req.failure()?.errorText ?? "fail"} ${req.url()}`));
    await page.goto(`http://127.0.0.1:${app.port}/`, { waitUntil: "networkidle", timeout: 120_000 });
    // Give React/wasm a moment to mount.
    await page.waitForTimeout(8000);
    const title = await page.title();
    const bodyText = await page.locator("body").innerText().catch(() => "");
    const canvasCount = await page.locator("canvas").count();
    const rootHtml = await page.locator("#root, #app, [data-semio-root]").count().catch(() => 0);
    await browser.close();
    return {
      id: app.id,
      port: app.port,
      title,
      canvasCount,
      rootHtml,
      bodySnippet: bodyText.slice(0, 400),
      consoleErrors: errors.slice(0, 30),
      pageErrors: pageErrors.slice(0, 30),
      failedRequests: failed.filter((u) => !u.includes("favicon")).slice(0, 30),
      ok:
        title.toLowerCase().includes("block") &&
        bodyText.includes("Example") &&
        !bodyText.includes("packValueFromBase64") &&
        !bodyText.includes("setActiveExample") &&
        !bodyText.includes("setContributions") &&
        !bodyText.includes("typed command channel") &&
        !errors.some((e) => e.includes("setActiveExample") || e.includes("setContributions") || e.includes("packValueFromBase64")),
    };
  } finally {
    child.kill("SIGTERM");
    await new Promise((r) => setTimeout(r, 2000));
    try { child.kill("SIGKILL"); } catch {}
    out.end();
  }
}

const results = [];
for (const app of apps) {
  try {
    results.push(await probeOne(app));
  } catch (e) {
    results.push({ id: app.id, ok: false, error: String(e) });
  }
  console.log(JSON.stringify(results[results.length - 1], null, 2));
}
const outPath = join(ticketDir, "e2e-runtime-results.json");
writeFileSync(outPath, JSON.stringify(results, null, 2) + "\n");
console.log("wrote", outPath);
if (!results.every((r) => r.ok)) process.exit(1);
