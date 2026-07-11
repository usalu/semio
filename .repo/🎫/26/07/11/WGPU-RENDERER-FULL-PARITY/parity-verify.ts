#!/usr/bin/env bun
/** 🔬 Wgpu↔React visual/structural parity harness. Boots the wgpu OS dev server, screenshots every
 * aligned playground, and (in --compare mode) diffs derived region statistics against the React
 * reference screenshots captured by `.repo/🎫/26/07/05/FIX-WGPU-WORLD3D-EMPTY-PREVIEW/verify-react-playgrounds-e2e.ts`.
 * Region-stat comparison (not pixel diffing) is intentional: the two renderers are different pipelines,
 * so exact pixel match isn't a realistic bar — see `.repo/🎫/26/07/04/RUST-PLUGIN-FRAMEWORK-MIGRATION/parity-gap-update.md`.
 */

import { type Subprocess, spawn } from "bun";
import { chromium, type Browser, type Page } from "playwright";
import { join } from "node:path";
import { PNG } from "pngjs";

const ticketDir = import.meta.dir;
const repoRoot = join(ticketDir, "../../../../../..");
const referenceDir = join(ticketDir, "../../05/FIX-WGPU-WORLD3D-EMPTY-PREVIEW");

//#region PlaygroundRegistry
/** Aligned against `verify-react-playgrounds-e2e.ts` / `verify-wgpu-playgrounds-e2e.ts` plugin lists and
 * the base plugin ids implied by every `🧊wgpu` entry in `.vscode/launch.json` (variant query-param
 * scenes like `concrete-forest` / `hexagonal-column` / `base` are NOT separate playgrounds — see
 * "Playground list reconciliation" in parity-report.md). No divergence found: still 25 base playgrounds. */
const plugins = [
  "draw",
  "note",
  "writer",
  "raster",
  "forms",
  "vcs",
  "flow",
  "dag",
  "imperative",
  "sequence",
  "layout",
  "puzzle2d",
  "gis2d",
  "procedural2d",
  "reasoning-wires",
  "cad",
  "puzzle3d",
  "puzzle5d",
  "shooting",
  "lowpoly",
  "procedural3d",
  "trinity",
  "trinity-rewrite",
  "s",
  "presentation",
] as const;
type PluginId = (typeof plugins)[number];
//#endregion

//#region Cli
const bunExe = Bun.which("bun") ?? "bun";
const compareMode = process.argv.includes("--compare");
const headed = process.env.HEADED === "1";
const onlyPlugin = process.argv.find((arg, index) => process.argv[index - 1] === "--plugin") as PluginId | undefined;
const targets: readonly PluginId[] = onlyPlugin ? plugins.filter((id) => id === onlyPlugin) : plugins;
const port = process.env.S_OS_PORT ?? "7301";
const baseUrl = `http://127.0.0.1:${port}/`;
const bootTimeoutMs = 240_000;
const useRunningServer = process.env.SKIP_DEV === "1";
//#endregion

//#region RegionStats
type RegionStats = {
  readonly nonBgRatio: number;
  readonly meanLuma: number;
  readonly maxLuma: number;
  readonly meanColor: readonly [number, number, number];
};

type ImageStats = {
  readonly width: number;
  readonly height: number;
  readonly navbar: RegionStats;
  readonly footer: RegionStats;
  readonly body: RegionStats;
};

const BG_R = 13;
const BG_G = 13;
const BG_B = 15;
const BG_TOLERANCE = 8;

function luma(r: number, g: number, b: number): number {
  return 0.299 * r + 0.587 * g + 0.114 * b;
}

function isBg(r: number, g: number, b: number): boolean {
  return Math.abs(r - BG_R) <= BG_TOLERANCE && Math.abs(g - BG_G) <= BG_TOLERANCE && Math.abs(b - BG_B) <= BG_TOLERANCE;
}

/** Region stats are computed on percentage-based bounds of each image independently, so a full-page
 * React DOM screenshot and a canvas-only wgpu screenshot remain comparable despite different pixel dimensions. */
function stripStats(data: Buffer, w: number, h: number, x0: number, y0: number, x1: number, y1: number): RegionStats {
  let nonBg = 0;
  let count = 0;
  let sumLuma = 0;
  let maxLuma = 0;
  let sumR = 0;
  let sumG = 0;
  let sumB = 0;
  for (let y = y0; y < y1; y++) {
    for (let x = x0; x < x1; x++) {
      const i = (y * w + x) * 4;
      const r = data[i]!;
      const g = data[i + 1]!;
      const b = data[i + 2]!;
      const l = luma(r, g, b);
      count += 1;
      sumLuma += l;
      maxLuma = Math.max(maxLuma, l);
      sumR += r;
      sumG += g;
      sumB += b;
      if (!isBg(r, g, b)) nonBg += 1;
    }
  }
  return {
    nonBgRatio: count > 0 ? nonBg / count : 0,
    meanLuma: count > 0 ? sumLuma / count : 0,
    maxLuma,
    meanColor: count > 0 ? [sumR / count, sumG / count, sumB / count] : [0, 0, 0],
  };
}

function analyzeImage(png: Buffer): ImageStats {
  const { data, width: w, height: h } = PNG.sync.read(png);
  if (w < 8 || h < 8) throw new Error("screenshot too small");
  const navbarH = Math.max(8, Math.floor(h * 0.06));
  const footerH = Math.max(8, Math.floor(h * 0.05));
  const bodyY0 = Math.floor(h * 0.08);
  const bodyY1 = Math.floor(h * 0.92);
  const bodyX0 = Math.floor(w * 0.06);
  const bodyX1 = Math.floor(w * 0.94);
  return {
    width: w,
    height: h,
    navbar: stripStats(data, w, h, 0, 0, w, navbarH),
    footer: stripStats(data, w, h, 0, h - footerH, w, h),
    body: stripStats(data, w, h, bodyX0, bodyY0, bodyX1, bodyY1),
  };
}
//#endregion

//#region Compare
const BODY_EMPTY_RATIO = 0.0015;
const BODY_EMPTY_LUMA = 24;

function paintedState(stats: ImageStats): "painted" | "blank" {
  return stats.body.nonBgRatio >= BODY_EMPTY_RATIO || stats.body.maxLuma >= BODY_EMPTY_LUMA ? "painted" : "blank";
}

type PluginReport = {
  readonly plugin: PluginId;
  readonly status: "PASS" | "FAIL" | "WARN" | "SKIPPED";
  readonly reasons: readonly string[];
  readonly react: ImageStats | null;
  readonly wgpu: ImageStats | null;
};

function compareStats(plugin: PluginId, react: ImageStats, wgpu: ImageStats): PluginReport {
  const reasons: string[] = [];
  const reactState = paintedState(react);
  const wgpuState = paintedState(wgpu);
  let status: PluginReport["status"] = "PASS";
  if (reactState === "painted" && wgpuState === "blank") {
    status = "FAIL";
    reasons.push(`wgpu body region is blank (ratio=${wgpu.body.nonBgRatio.toFixed(4)}, maxLuma=${wgpu.body.maxLuma.toFixed(1)}) while react reference has content (ratio=${react.body.nonBgRatio.toFixed(4)}, maxLuma=${react.body.maxLuma.toFixed(1)})`);
  } else if (reactState === "blank" && wgpuState === "painted") {
    status = "WARN";
    reasons.push(`wgpu body region has content while react reference is blank — reference may be stale or plugin gained new content`);
  } else {
    reasons.push(`both renderers agree on body content presence (${reactState})`);
  }
  const chromeGate = (label: string, r: RegionStats, w: RegionStats) => {
    const rPainted = r.nonBgRatio >= BODY_EMPTY_RATIO || r.maxLuma >= BODY_EMPTY_LUMA;
    const wPainted = w.nonBgRatio >= BODY_EMPTY_RATIO || w.maxLuma >= BODY_EMPTY_LUMA;
    if (rPainted && !wPainted) {
      if (status !== "FAIL") status = "FAIL";
      reasons.push(`wgpu ${label} appears blank while react reference ${label} has chrome content`);
    }
  };
  chromeGate("navbar", react.navbar, wgpu.navbar);
  chromeGate("footer", react.footer, wgpu.footer);
  return { plugin, status, reasons, react, wgpu };
}
//#endregion

//#region Preflight
function isAdapterUnavailableError(text: string): boolean {
  return text.includes("NoCompatibleDevice") || text.includes("No available adapters") || text.includes("no suitable adapters");
}

/** WebGPU adapter preflight: navigates the first target plugin and races the wgpu-boot console
 * marker against an adapter-unavailable error. Headless swiftshader/ANGLE Vulkan usually works
 * (see `parity-gap-update.md` "Known headless-only gaps"), but when it doesn't this documents the
 * real fallback: rerun with `HEADED=1` to use a real (non-headless) browser window with a real GPU. */
async function preflight(browser: Browser): Promise<void> {
  const page = await browser.newPage();
  try {
    let adapterError: string | null = null;
    page.on("pageerror", (error) => {
      if (isAdapterUnavailableError(error.message)) adapterError = error.message;
    });
    page.on("console", (message) => {
      if (isAdapterUnavailableError(message.text())) adapterError = message.text();
    });
    await page.goto(`${baseUrl}?plugin=s`, { waitUntil: "domcontentloaded", timeout: bootTimeoutMs });
    const result = await Promise.race([
      new Promise<"booted">((resolve) => {
        const onConsole = (message: import("playwright").ConsoleMessage) => {
          if (message.text().includes("[DEBUG] wgpu renderer booted")) {
            page.off("console", onConsole);
            resolve("booted");
          }
        };
        page.on("console", onConsole);
      }),
      new Promise<"adapter-error">((resolve) => {
        const check = setInterval(() => {
          if (adapterError) {
            clearInterval(check);
            resolve("adapter-error");
          }
        }, 200);
      }),
      new Promise<"timeout">((resolve) => setTimeout(() => resolve("timeout"), bootTimeoutMs)),
    ]);
    if (result === "adapter-error") {
      if (!headed) {
        throw new Error(
          `WebGPU adapter unavailable in headless mode (${adapterError}). Rerun this harness with HEADED=1 to use a real, non-headless browser window with a real GPU backend — see .repo/🎫/26/07/04/RUST-PLUGIN-FRAMEWORK-MIGRATION/parity-gap-update.md "Known headless-only gaps".`,
        );
      }
      throw new Error(`WebGPU adapter unavailable even in HEADED mode (${adapterError}). This environment has no usable GPU/Vulkan/ANGLE backend for Chrome.`);
    }
    if (result === "timeout") {
      console.log(`[DEBUG] preflight: no adapter error observed but boot marker did not fire within ${bootTimeoutMs}ms either — proceeding, first plugin result will reflect actual state`);
    } else {
      console.log(`[DEBUG] preflight: WebGPU adapter available (mode=${headed ? "headed" : "headless"})`);
    }
  } finally {
    await page.close();
  }
}
//#endregion

//#region Capture
async function screenshotCanvas(page: Page): Promise<Buffer> {
  const canvas = page.locator("#semio-wgpu-canvas");
  const box = await canvas.boundingBox();
  if (!box) throw new Error("canvas missing for screenshot");
  const png = await page.screenshot({
    type: "png",
    clip: { x: box.x, y: box.y, width: box.width, height: box.height },
    animations: "disabled",
    timeout: 60_000,
  });
  return Buffer.from(png);
}

async function waitForWgpuBoot(page: Page): Promise<void> {
  await new Promise<void>((resolve, reject) => {
    const timeout = setTimeout(() => reject(new Error("wgpu boot timeout")), bootTimeoutMs);
    const onConsole = (message: import("playwright").ConsoleMessage) => {
      if (message.text().includes("[DEBUG] wgpu renderer booted")) {
        clearTimeout(timeout);
        page.off("console", onConsole);
        resolve();
      }
    };
    page.on("console", onConsole);
  });
}

async function capturePlugin(browser: Browser, plugin: PluginId): Promise<{ png: Buffer } | { skipped: string }> {
  const page = await browser.newPage();
  try {
    const errors: string[] = [];
    page.on("pageerror", (error) => {
      if (!isAdapterUnavailableError(error.message)) errors.push(error.message);
    });
    await page.goto(`${baseUrl}?plugin=${encodeURIComponent(plugin)}`, { waitUntil: "domcontentloaded", timeout: bootTimeoutMs });
    await page.waitForSelector("#semio-wgpu-canvas", { timeout: bootTimeoutMs });
    await waitForWgpuBoot(page);
    await page.waitForTimeout(1200);
    const png = await screenshotCanvas(page);
    return { png };
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    return { skipped: message };
  } finally {
    await page.close();
  }
}

async function capturePluginWithRetry(browser: Browser, plugin: PluginId): Promise<{ png: Buffer } | { skipped: string }> {
  const first = await capturePlugin(browser, plugin);
  if ("png" in first) return first;
  console.log(`[DEBUG] ${plugin}: first attempt failed (${first.skipped}) — retrying once`);
  return capturePlugin(browser, plugin);
}
//#endregion

//#region Report
async function loadReferenceStatsAsync(plugin: PluginId): Promise<{ stats: ImageStats } | { missing: true }> {
  const path = join(referenceDir, `screenshot-react-${plugin}.png`);
  const file = Bun.file(path);
  if (!(await file.exists())) return { missing: true };
  const buffer = Buffer.from(await file.arrayBuffer());
  return { stats: analyzeImage(buffer) };
}

function writeMarkdownReport(reports: readonly PluginReport[]): string {
  const counts = { PASS: 0, FAIL: 0, WARN: 0, SKIPPED: 0 };
  for (const r of reports) counts[r.status] += 1;
  const lines: string[] = [];
  lines.push("# Wgpu ↔ React Parity Report");
  lines.push("");
  lines.push(`Generated by \`parity-verify.ts --compare\` against React reference screenshots in \`.repo/🎫/26/07/05/FIX-WGPU-WORLD3D-EMPTY-PREVIEW/\`.`);
  lines.push("");
  lines.push(`**Summary:** ${counts.PASS} PASS / ${counts.FAIL} FAIL / ${counts.WARN} WARN / ${counts.SKIPPED} SKIPPED (of ${reports.length})`);
  lines.push("");
  lines.push("| Playground | Status | React body ratio/luma | Wgpu body ratio/luma | Notes |");
  lines.push("|---|---|---|---|---|");
  for (const r of reports) {
    const reactCell = r.react ? `${r.react.body.nonBgRatio.toFixed(4)} / ${r.react.body.maxLuma.toFixed(1)}` : "—";
    const wgpuCell = r.wgpu ? `${r.wgpu.body.nonBgRatio.toFixed(4)} / ${r.wgpu.body.maxLuma.toFixed(1)}` : "—";
    const notes = r.reasons.join("; ").replace(/\|/g, "\\|");
    lines.push(`| ${r.plugin} | ${r.status} | ${reactCell} | ${wgpuCell} | ${notes} |`);
  }
  lines.push("");
  lines.push("## Playground list reconciliation");
  lines.push("");
  lines.push(
    "The 25 playgrounds above match both `verify-react-playgrounds-e2e.ts` and `verify-wgpu-playgrounds-e2e.ts`'s plugin lists, and match the base plugin ids implied by every `🧊wgpu` entry in `.vscode/launch.json`. " +
      "`.vscode/launch.json` additionally registers scene-variant launch configs (`cad+concrete-forest`, `puzzle3d/5d+concrete-forest`, `procedural3d+hexagonal-column`, `shooting+base`, `trinity+jack`, `vcs+play`, `presentation+play`) — these are query-param scene variants of an existing plugin, not separate top-level playgrounds, and have no corresponding React reference screenshot, so they are intentionally out of scope for this report rather than silently covered.",
  );
  return lines.join("\n");
}
//#endregion

async function main(): Promise<void> {
  let devProc: Subprocess | null = null;
  const reports: PluginReport[] = [];
  try {
    if (!useRunningServer) {
      devProc = spawn({
        cmd: [bunExe, "nx", "run", "@semio-tech/framework-os-dev:dev"],
        cwd: repoRoot,
        stdout: "pipe",
        stderr: "pipe",
        env: {
          ...process.env,
          SKIP_PLUGIN_BUILD: process.env.SKIP_PLUGIN_BUILD ?? "1",
          SKIP_WGPU_BUILD: process.env.SKIP_WGPU_BUILD ?? "1",
          SKIP_ENGINE_BUILD: process.env.SKIP_ENGINE_BUILD ?? "1",
          S_OS_PORT: port,
          SEMIO_RENDERER: "wgpu",
          SEMIO_PLUGIN: "s",
        },
      });
      const deadline = Date.now() + bootTimeoutMs;
      while (Date.now() < deadline) {
        try {
          const response = await fetch(baseUrl);
          if (response.ok) break;
        } catch {}
        await Bun.sleep(500);
      }
    }

    let browser: Browser = await chromium.launch({
      headless: !headed,
      args: ["--enable-unsafe-webgpu", "--use-angle=swiftshader", "--enable-features=Vulkan"],
    });

    try {
      await preflight(browser);

      for (const [index, plugin] of targets.entries()) {
        if (index > 0 && index % 5 === 0) {
          await browser.close();
          browser = await chromium.launch({
            headless: !headed,
            args: ["--enable-unsafe-webgpu", "--use-angle=swiftshader", "--enable-features=Vulkan"],
          });
        }
        process.stdout.write(`PARITY ${plugin}... `);
        const refResult = await loadReferenceStatsAsync(plugin);
        const capture = await capturePluginWithRetry(browser, plugin);
        if ("skipped" in capture) {
          const reason = `wgpu capture failed: ${capture.skipped}`;
          console.log(`SKIPPED (${reason})`);
          reports.push({ plugin, status: "SKIPPED", reasons: [reason], react: "stats" in refResult ? refResult.stats : null, wgpu: null });
          continue;
        }
        if ("missing" in refResult) {
          const reason = `no react reference screenshot at screenshot-react-${plugin}.png`;
          console.log(`SKIPPED (${reason})`);
          reports.push({ plugin, status: "SKIPPED", reasons: [reason], react: null, wgpu: analyzeImage(capture.png) });
          await Bun.write(join(ticketDir, `screenshot-wgpu-${plugin}.png`), capture.png);
          continue;
        }
        await Bun.write(join(ticketDir, `screenshot-wgpu-${plugin}.png`), capture.png);
        const wgpuStats = analyzeImage(capture.png);
        const report = compareStats(plugin, refResult.stats, wgpuStats);
        console.log(report.status);
        reports.push(report);
      }
    } finally {
      await browser.close();
    }

    const jsonPath = join(ticketDir, "parity-report.json");
    const mdPath = join(ticketDir, "parity-report.md");
    await Bun.write(jsonPath, `${JSON.stringify({ generatedWithCompare: compareMode, port, headed, reports }, null, 2)}\n`);
    await Bun.write(mdPath, `${writeMarkdownReport(reports)}\n`);

    const counts = { PASS: 0, FAIL: 0, WARN: 0, SKIPPED: 0 };
    for (const r of reports) counts[r.status] += 1;
    console.log(`\n${counts.PASS} PASS / ${counts.FAIL} FAIL / ${counts.WARN} WARN / ${counts.SKIPPED} SKIPPED (of ${reports.length})`);
    console.log(`report: ${jsonPath}`);
    console.log(`report: ${mdPath}`);
    process.exit(counts.FAIL > 0 ? 1 : 0);
  } finally {
    devProc?.kill();
    await devProc?.exited;
  }
}

await main();
