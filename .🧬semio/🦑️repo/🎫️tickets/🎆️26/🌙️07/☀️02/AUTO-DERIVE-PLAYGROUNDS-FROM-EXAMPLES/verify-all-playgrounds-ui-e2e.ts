#!/usr/bin/env bun
/** 🧪️ Builds each registered playground, serves preview, and asserts mounted chrome UI. */

import { type Subprocess, spawn } from "bun";
import { chromium, type ConsoleMessage } from "playwright";

const APPS = [
  "2d",
  "3d",
  "5d",
  "gis-2d",
  "wires",
  "draw",
  "note",
  "writer",
  "raster",
  "forms",
  "flow",
  "dag",
  "imperative",
  "sequence",
  "layout",
  "lowpoly",
  "procedural-2d",
  "procedural-3d",
  "shooting",
  "vcs",
  "trinity-jack",
  "trinity-rewrite",
  "presentation",
  "cad",
] as const;

const PACKAGE_ROOT_BY_ENTRY: Readonly<Record<string, string>> = {
  draw: "draw",
  note: "note",
  writer: "writer",
  raster: "raster",
  forms: "forms",
  flow: "flow",
  dag: "mathematical/graph/port/directed/dag",
  imperative: "imperative",
  sequence: "sequence",
  layout: "layout",
  lowpoly: "lowpoly",
  "procedural-2d": "procedural/2d",
  "procedural-3d": "procedural/3d",
  shooting: "shooting",
  vcs: "vcs",
  "gis-2d": "gis/2d",
  wires: "reasoning/mindmap/wires",
  "trinity-jack": "trinity/jack/host-core",
  "trinity-rewrite": "trinity/rewrite",
  presentation: "framework/product/presentation",
  cad: "cad/renderer",
  "2d": "puzzle/2d",
  "3d": "puzzle/3d",
  "5d": "puzzle/5d",
};

const bunExe = process.env.BUN ?? Bun.which("bun") ?? "bun";
const repoRoot = `${import.meta.dir}/../../../../../..`;
const devDir = `${repoRoot}/framework/product/playground/dev`;
const skipBuild = process.argv.includes("--skip-build");
const onlyApp = process.argv.find((arg, index) => process.argv[index - 1] === "--app");
const previewPort = Number(process.env.PLAYGROUND_E2E_PORT ?? 14000 + (process.pid % 1000));
const bootTimeoutMs = 180_000;

async function runBuild(app: string): Promise<void> {
  const proc = spawn({
    cmd: [bunExe, "./📜️script.ts", "build", "--app", app],
    cwd: devDir,
    stdout: "pipe",
    stderr: "pipe",
    env: {
      ...process.env,
      PLAYGROUND_APP: app,
      PUZZLE_PLAY_ENTRY: app,
      PLAYGROUND_PACKAGE_ROOT: PACKAGE_ROOT_BY_ENTRY[app] ?? "",
    },
  });
  const code = await proc.exited;
  if (code !== 0) {
    const stderr = await new Response(proc.stderr).text();
    throw new Error(`build failed (${code}): ${stderr.slice(-1200)}`);
  }
}

async function waitForPreview(url: string): Promise<void> {
  const deadline = Date.now() + 45_000;
  while (Date.now() < deadline) {
    try {
      const response = await fetch(url);
      if (response.ok) return;
    } catch {
      // retry
    }
    await Bun.sleep(300);
  }
  throw new Error(`preview not ready at ${url}`);
}

function isBootFailure(message: ConsoleMessage): boolean {
  if (message.type() !== "error") return false;
  const text = message.text();
  if (text.includes("playground-dev boot failed")) return true;
  if (text.includes("Failed to fetch dynamically imported module")) return true;
  if (text.includes("Failed to resolve import")) return true;
  if (text.includes("Uncaught")) return true;
  return false;
}

async function smokeBoot(app: string): Promise<{
  readonly title: string;
  readonly nav: boolean;
  readonly buttons: number;
  readonly panels: number;
  readonly rootChildren: number;
  readonly errors: string[];
}> {
  const url = `http://127.0.0.1:${previewPort}/`;
  const preview = spawn({
    cmd: [bunExe, "run", "vite", "preview", "--host", "127.0.0.1", "--port", String(previewPort), "--strictPort"],
    cwd: devDir,
    stdout: "pipe",
    stderr: "pipe",
    env: {
      ...process.env,
      PLAYGROUND_APP: app,
      PUZZLE_PLAY_ENTRY: app,
      PLAYGROUND_PACKAGE_ROOT: PACKAGE_ROOT_BY_ENTRY[app] ?? "",
    },
  });

  const errors: string[] = [];
  const browser = await chromium.launch({
    headless: true,
    args: ["--enable-unsafe-webgpu", "--use-angle=swiftshader", "--enable-features=Vulkan"],
  });
  try {
    await waitForPreview(url);
    const page = await browser.newPage();
    page.on("console", (message) => {
      if (isBootFailure(message)) errors.push(message.text());
    });
    page.on("pageerror", (error) => errors.push(error.message));
    await page.goto(url, { waitUntil: "domcontentloaded", timeout: bootTimeoutMs });
    await page.waitForFunction(
      () => {
        const root = document.querySelector("#root");
        return (root?.childElementCount ?? 0) > 0 || !!document.querySelector("nav");
      },
      undefined,
      { timeout: bootTimeoutMs },
    );
    const state = await page.evaluate(() => ({
      title: document.title,
      nav: !!document.querySelector("nav"),
      buttons: document.querySelectorAll("button").length,
      panels: document.querySelectorAll("[data-panel]").length,
      rootChildren: document.getElementById("root")?.childElementCount ?? 0,
    }));
    return { ...state, errors };
  } finally {
    await browser.close();
    preview.kill();
    await preview.exited;
  }
}

let failed = 0;
const logLines: string[] = [];
const targets = onlyApp ? APPS.filter((app) => app === onlyApp) : APPS;
if (onlyApp && targets.length === 0) {
  console.error(`unknown app: ${onlyApp}`);
  process.exit(1);
}

for (const app of targets) {
  try {
    if (!skipBuild) {
      process.stdout.write(`BUILD ${app}... `);
      await runBuild(app);
      console.log("ok");
    }
    process.stdout.write(`BOOT ${app}... `);
    const result = await smokeBoot(app);
    if (result.errors.length > 0) throw new Error(result.errors.join(" | "));
    if (result.rootChildren < 1 && !result.nav) throw new Error("no mounted UI");
    if (!result.nav) throw new Error(`missing nav (buttons=${result.buttons})`);
    if (result.buttons < 1) throw new Error("no interactive chrome");
    const line = `ok title=${result.title} nav=${result.nav} panels=${result.panels} buttons=${result.buttons}`;
    console.log(line);
    logLines.push(`${app}: ${line}`);
  } catch (error) {
    failed += 1;
    const message = error instanceof Error ? error.message : String(error);
    console.log(`FAIL ${message}`);
    logLines.push(`${app}: FAIL ${message}`);
  }
}

const logPath = `${import.meta.dir}/verify-all-playgrounds-ui-e2e.log`;
await Bun.write(logPath, `${logLines.join("\n")}\n`);
console.log(`\nlog: ${logPath}`);
process.exit(failed > 0 ? 1 : 0);
