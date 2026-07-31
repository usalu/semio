#!/usr/bin/env bun
import { spawn } from "bun";
import { chromium } from "playwright";

const bunExe = Bun.which("bun") ?? "bun";
const repoRoot = `${import.meta.dir}/../../../../../..`;
const devDir = `${repoRoot}/framework/product/playground/dev`;
const port = 14174;
const app = "5d";

const proc = spawn({
  cmd: [bunExe, "./📜️script.ts", "build", "--app", app],
  cwd: devDir,
  stdout: "inherit",
  stderr: "inherit",
  env: { ...process.env, PLAYGROUND_APP: app, PUZZLE_PLAY_ENTRY: app, PLAYGROUND_PACKAGE_ROOT: "puzzle/5d" },
});
if ((await proc.exited) !== 0) process.exit(1);

const preview = spawn({
  cmd: [bunExe, "run", "vite", "preview", "--host", "127.0.0.1", "--port", String(port), "--strictPort"],
  cwd: devDir,
  stdout: "pipe",
  stderr: "pipe",
  env: { ...process.env, PLAYGROUND_APP: app, PUZZLE_PLAY_ENTRY: app, PLAYGROUND_PACKAGE_ROOT: "puzzle/5d" },
});

const url = `http://127.0.0.1:${port}/`;
for (let i = 0; i < 60; i++) {
  try {
    const r = await fetch(url);
    if (r.ok) break;
  } catch {}
  await Bun.sleep(500);
}

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
const logs: string[] = [];
page.on("console", (m) => logs.push(`[${m.type()}] ${m.text()}`));
page.on("pageerror", (e) => logs.push(`[pageerror] ${e.message}`));
await page.goto(url, { waitUntil: "networkidle", timeout: 120_000 });
await Bun.sleep(5000);
const state = await page.evaluate(() => ({
  title: document.title,
  html: document.body?.innerHTML?.slice(0, 2000) ?? "",
  rootChildren: document.getElementById("root")?.childElementCount ?? 0,
  nav: !!document.querySelector("nav"),
}));
console.log(JSON.stringify(state, null, 2));
console.log("--- console ---");
for (const line of logs.slice(-40)) console.log(line);
await browser.close();
preview.kill();
await preview.exited;
