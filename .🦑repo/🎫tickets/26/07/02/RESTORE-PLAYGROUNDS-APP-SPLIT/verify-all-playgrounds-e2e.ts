#!/usr/bin/env bun
/** 🧪 Builds each playground, serves preview, and smoke-tests browser boot via Playwright. */

import { type Subprocess, spawn } from "bun";
import { chromium, type ConsoleMessage } from "playwright";

const APPS = [
  "2d",
  "3d",
  "5d",
  "gis-2d",
  "wires",
  "draw",
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
  "s",
  "vcs",
  "trinity-jack",
  "trinity-rewrite",
  "presentation",
  "cad",
] as const;

const PACKAGE_ROOT_BY_ENTRY: Readonly<Record<string, string>> = {
  draw: "draw",
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
  s: "s",
  vcs: "vcs",
  "gis-2d": "gis/2d",
  wires: "reasoning/mindmap/wires",
  "trinity-jack": "trinity/jack/host-core",
  "trinity-rewrite": "trinity/rewrite",
  presentation: "framework/product/presentation",
  cad: "cad/js/renderer",
  "2d": "puzzle/2d",
  "3d": "puzzle/3d",
  "5d": "puzzle/5d",
};

const bunExe = process.execPath;
const devDir = `${import.meta.dir}/../../../../../../framework/product/playground/dev`;
const skipBuild = process.argv.includes("--skip-build");
const onlyApp = process.argv.find((arg, index) => process.argv[index - 1] === "--app");
const previewPort = 14173;
const bootTimeoutMs = 120_000;

async function runBuild(app: string): Promise<void> {
  const proc = spawn({
    cmd: [bunExe, "./📜script.ts", "build", "--app", app],
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
    throw new Error(`build failed (${code}): ${stderr.slice(-800)}`);
  }
}

async function waitForPreview(url: string, proc: Subprocess): Promise<void> {
  const deadline = Date.now() + 30_000;
  while (Date.now() < deadline) {
    try {
      const response = await fetch(url);
      if (response.ok) return;
    } catch {
      // retry
    }
    await Bun.sleep(250);
  }
  throw new Error(`preview not ready at ${url}`);
}

function isBootFailure(message: ConsoleMessage): boolean {
  if (message.type() !== "error") return false;
  const text = message.text();
  if (text.includes("playground-dev boot failed")) return true;
  if (text.includes("Failed to fetch dynamically imported module")) return true;
  if (text.includes("Uncaught")) return true;
  return false;
}

async function smokeBoot(app: string): Promise<{ readonly rootChildren: number; readonly errors: string[] }> {
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
    await waitForPreview(url, preview);
    const page = await browser.newPage();
    page.on("console", (message) => {
      if (isBootFailure(message)) errors.push(message.text());
    });
    page.on("pageerror", (error) => errors.push(error.message));
    await page.goto(url, { waitUntil: "domcontentloaded", timeout: bootTimeoutMs });
    await page.waitForFunction(() => (document.querySelector("#root")?.childElementCount ?? 0) > 0, undefined, {
      timeout: bootTimeoutMs,
    });
    const rootChildren = await page.locator("#root > *").count();
    return { rootChildren, errors };
  } finally {
    await browser.close();
    preview.kill();
    await preview.exited;
  }
}

let failed = 0;
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
    const { rootChildren, errors } = await smokeBoot(app);
    if (errors.length > 0) throw new Error(errors.join(" | "));
    if (rootChildren < 1) throw new Error("#root has no mounted children");
    console.log(`ok rootChildren=${rootChildren}`);
  } catch (error) {
    failed += 1;
    const message = error instanceof Error ? error.message : String(error);
    console.log(`FAIL ${message}`);
  }
}

process.exit(failed > 0 ? 1 : 0);
