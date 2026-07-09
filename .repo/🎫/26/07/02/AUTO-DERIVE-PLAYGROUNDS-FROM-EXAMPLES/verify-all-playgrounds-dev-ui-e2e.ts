#!/usr/bin/env bun
/** 🧪 Dev-server smoke: boots each registered playground and asserts mounted chrome UI. */

import { type Subprocess, spawn } from "bun";
import { chromium } from "playwright";

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

const PORT_BY_APP: Readonly<Record<string, number>> = {
  "2d": 6012,
  "3d": 6013,
  "5d": 6014,
  "gis-2d": 6040,
  wires: 6015,
  draw: 6064,
  note: 6080,
  writer: 6062,
  raster: 6060,
  forms: 6058,
  flow: 6016,
  dag: 6017,
  imperative: 6076,
  sequence: 6077,
  layout: 6079,
  lowpoly: 6078,
  "procedural-2d": 6021,
  "procedural-3d": 6018,
  shooting: 6019,
  vcs: 6075,
  "trinity-jack": 6054,
  "trinity-rewrite": 6056,
  presentation: 6051,
  cad: 6020,
};

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

const bunExe = Bun.which("bun") ?? "bun";
const repoRoot = `${import.meta.dir}/../../../../../..`;
const devDir = `${repoRoot}/framework/product/playground/dev`;
const onlyApp = process.argv.find((arg, index) => process.argv[index - 1] === "--app");
const bootTimeoutMs = 180_000;

async function waitForDev(url: string): Promise<void> {
  const deadline = Date.now() + bootTimeoutMs;
  while (Date.now() < deadline) {
    try {
      const response = await fetch(url);
      if (response.ok) return;
    } catch {}
    await Bun.sleep(400);
  }
  throw new Error(`dev server not ready at ${url}`);
}

async function smokeDev(app: string): Promise<string> {
  const port = PORT_BY_APP[app];
  if (!port) throw new Error(`no port for ${app}`);
  const url = `http://127.0.0.1:${port}/`;
  const proc = spawn({
    cmd: [bunExe, "./script.ts", "dev", "--app", app],
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
  try {
    await waitForDev(url);
    const browser = await chromium.launch({
      headless: true,
      args: ["--enable-unsafe-webgpu", "--use-angle=swiftshader", "--enable-features=Vulkan"],
    });
    try {
      const page = await browser.newPage();
      page.on("pageerror", (error) => errors.push(error.message));
      page.on("console", (message) => {
        if (message.type() !== "error") return;
        const text = message.text();
        if (text.includes("playground-dev boot failed") || text.includes("Failed to resolve import") || text.includes("Uncaught")) {
          errors.push(text);
        }
      });
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
      if (errors.length > 0) throw new Error(errors.join(" | "));
      if (state.rootChildren < 1 && !state.nav) throw new Error("no mounted UI");
      if (!state.nav) throw new Error(`missing nav (buttons=${state.buttons})`);
      if (state.buttons < 1) throw new Error("no interactive chrome");
      return `ok title=${state.title} nav=${state.nav} panels=${state.panels} buttons=${state.buttons}`;
    } finally {
      await browser.close();
    }
  } finally {
    proc.kill();
    await proc.exited;
  }
}

const targets = onlyApp ? APPS.filter((app) => app === onlyApp) : [...APPS];
if (onlyApp && targets.length === 0) {
  console.error(`unknown app: ${onlyApp}`);
  process.exit(1);
}

let failed = 0;
const logLines: string[] = [];
for (const app of targets) {
  process.stdout.write(`DEV ${app}... `);
  try {
    const line = await smokeDev(app);
    console.log(line);
    logLines.push(`${app}: ${line}`);
  } catch (error) {
    failed += 1;
    const message = error instanceof Error ? error.message : String(error);
    console.log(`FAIL ${message}`);
    logLines.push(`${app}: FAIL ${message}`);
  }
}

const logPath = `${import.meta.dir}/verify-all-playgrounds-dev-ui-e2e.log`;
await Bun.write(logPath, `${logLines.join("\n")}\n`);
console.log(`\nlog: ${logPath}`);
console.log(`\n${targets.length - failed}/${targets.length} passed`);
process.exit(failed > 0 ? 1 : 0);
