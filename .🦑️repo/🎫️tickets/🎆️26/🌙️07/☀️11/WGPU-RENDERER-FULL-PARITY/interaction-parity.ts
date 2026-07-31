#!/usr/bin/env bun
/** 🎬️ Wgpu↔React interaction-delta parity harness. Boots BOTH renderers of the same OS dev host
 * (`@semio-tech/framework-os-dev:dev`, one process with `SEMIO_RENDERER=react`, one with
 * `SEMIO_RENDERER=wgpu`) side by side and drives an IDENTICAL scripted interaction sequence against
 * each. Rather than comparing absolute pixels (the two renderers are fundamentally different
 * pipelines — see `parity-verify.ts`'s header), each probe compares the BEFORE/AFTER delta of the
 * same region-stat metrics `parity-verify.ts` uses: did both renderers change in the same direction,
 * by a roughly comparable order of magnitude. That sidesteps absolute-pixel mismatch entirely.
 *
 * Probes (modeled on the interaction smokes already in
 * `.repo/🎫️/26/07/04/WGPU-PLAYGROUND-E2E/verify-wgpu-playgrounds-e2e.ts`):
 *   1. Command palette: Meta/Control+P opens an overlay (non-background delta), Escape returns to baseline.
 *   2. Forms: two scripted viewport clicks produce a body-region stat change.
 *   3. Generate-mode toggle (flow/procedural2d/procedural3d): toggling produces a same-direction,
 *      same-rough-magnitude-class body-region stat change in both renderers.
 *
 * Ports: wgpu defaults to 7301 (parity-verify.ts's own default — never run both scripts at once
 * against the same live server), react defaults to 7300. Both fall back to the next free port in
 * REACT_PORT_FALLBACKS/WGPU_PORT_FALLBACKS if the preferred port is already bound, and the chosen
 * ports are logged + written into the report so a reader knows if a fallback was used. */

import { type Subprocess, spawn } from "bun";
import { chromium, type Browser, type Page } from "playwright";
import { createServer } from "node:net";
import { join } from "node:path";
import { PNG } from "pngjs";

const ticketDir = import.meta.dir;
const repoRoot = join(ticketDir, "../../../../../..");
const bunExe = Bun.which("bun") ?? "bun";
const headed = process.env.HEADED === "1";
const bootTimeoutMs = 240_000;

//#region PortSelection
function isPortFree(port: number): Promise<boolean> {
  return new Promise((resolve) => {
    const server = createServer();
    server.once("error", () => resolve(false));
    server.once("listening", () => server.close(() => resolve(true)));
    server.listen(port, "127.0.0.1");
  });
}

async function pickPort(label: string, explicit: string | undefined, preferred: number, fallbacks: readonly number[]): Promise<{ port: number; usedFallback: boolean }> {
  if (explicit) return { port: Number(explicit), usedFallback: false };
  if (await isPortFree(preferred)) return { port: preferred, usedFallback: false };
  for (const fallback of fallbacks) {
    if (await isPortFree(fallback)) {
      console.log(`[DEBUG] ${label}: preferred port ${preferred} unavailable, using fallback ${fallback}`);
      return { port: fallback, usedFallback: true };
    }
  }
  throw new Error(`${label}: no free port among ${[preferred, ...fallbacks].join(", ")}`);
}
//#endregion

//#region RegionStats
type RegionStats = { readonly nonBgRatio: number; readonly meanLuma: number; readonly meanColor: readonly [number, number, number] };
type ImageStats = { readonly overall: RegionStats; readonly navbar: RegionStats; readonly body: RegionStats; readonly footer: RegionStats };

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

function stripStats(data: Buffer, w: number, x0: number, y0: number, x1: number, y1: number): RegionStats {
  let nonBg = 0;
  let count = 0;
  let sumLuma = 0;
  let sumR = 0;
  let sumG = 0;
  let sumB = 0;
  for (let y = y0; y < y1; y++) {
    for (let x = x0; x < x1; x++) {
      const i = (y * w + x) * 4;
      const r = data[i]!;
      const g = data[i + 1]!;
      const b = data[i + 2]!;
      count += 1;
      sumLuma += luma(r, g, b);
      sumR += r;
      sumG += g;
      sumB += b;
      if (!isBg(r, g, b)) nonBg += 1;
    }
  }
  return {
    nonBgRatio: count > 0 ? nonBg / count : 0,
    meanLuma: count > 0 ? sumLuma / count : 0,
    meanColor: count > 0 ? [sumR / count, sumG / count, sumB / count] : [0, 0, 0],
  };
}

function analyzeImage(png: Buffer): ImageStats {
  const { data, width: w, height: h } = PNG.sync.read(png);
  if (w < 8 || h < 8) throw new Error("screenshot too small");
  const navbarH = Math.max(8, Math.floor(h * 0.06));
  const footerH = Math.max(8, Math.floor(h * 0.05));
  return {
    overall: stripStats(data, w, 0, 0, w, h),
    navbar: stripStats(data, w, 0, 0, w, navbarH),
    footer: stripStats(data, w, 0, h - footerH, w, h),
    body: stripStats(data, w, Math.floor(w * 0.06), Math.floor(h * 0.08), Math.floor(w * 0.94), Math.floor(h * 0.92)),
  };
}
//#endregion

//#region Capture
async function captureReactStats(page: Page): Promise<ImageStats> {
  const png = await page.screenshot({ type: "png", animations: "disabled", timeout: 60_000 });
  return analyzeImage(Buffer.from(png));
}

async function captureWgpuStats(page: Page): Promise<ImageStats> {
  const canvas = page.locator("#semio-wgpu-canvas");
  const box = await canvas.boundingBox();
  if (!box) throw new Error("wgpu canvas missing for screenshot");
  const png = await page.screenshot({ type: "png", clip: { x: box.x, y: box.y, width: box.width, height: box.height }, animations: "disabled", timeout: 60_000 });
  return analyzeImage(Buffer.from(png));
}

async function clickViewport(page: Page, rx: number, ry: number): Promise<void> {
  const viewport = page.viewportSize() ?? { width: 1280, height: 720 };
  await page.mouse.click(viewport.width * rx, viewport.height * ry);
}

function isAdapterUnavailableError(text: string): boolean {
  return text.includes("NoCompatibleDevice") || text.includes("No available adapters") || text.includes("no suitable adapters");
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

async function openReactPage(browser: Browser, baseUrl: string, plugin: string): Promise<Page> {
  const page = await browser.newPage();
  page.on("pageerror", (error) => console.log(`[DEBUG] react pageerror (${plugin}): ${error.message}`));
  await page.goto(`${baseUrl}?plugin=${encodeURIComponent(plugin)}`, { waitUntil: "domcontentloaded", timeout: bootTimeoutMs });
  await page.waitForSelector('[data-slot="navbar"]', { timeout: bootTimeoutMs });
  await page.waitForSelector('[data-slot="footer"]', { timeout: bootTimeoutMs });
  await page.waitForTimeout(1200);
  return page;
}

async function openWgpuPage(browser: Browser, baseUrl: string, plugin: string): Promise<Page> {
  const page = await browser.newPage();
  page.on("pageerror", (error) => {
    if (!isAdapterUnavailableError(error.message)) console.log(`[DEBUG] wgpu pageerror (${plugin}): ${error.message}`);
  });
  await page.goto(`${baseUrl}?plugin=${encodeURIComponent(plugin)}`, { waitUntil: "domcontentloaded", timeout: bootTimeoutMs });
  await page.waitForSelector("#semio-wgpu-canvas", { timeout: bootTimeoutMs });
  await waitForWgpuBoot(page);
  await page.waitForTimeout(1200);
  return page;
}
//#endregion

//#region DeltaAssertions
/** 🧮️ Rough magnitude class on a log10 scale — used to compare "did both renderers change by a
 * comparable order of magnitude" without requiring near-exact numeric agreement across two
 * structurally different rendering pipelines. */
function magnitudeClass(delta: number): number {
  const magnitude = Math.abs(delta);
  return magnitude < 1e-6 ? Number.NEGATIVE_INFINITY : Math.floor(Math.log10(magnitude));
}

type DeltaVerdict = "same-direction" | "opposite-direction" | "negligible-one-side" | "negligible-both";

function classifyDelta(reactDelta: number, wgpuDelta: number, epsilon: number): DeltaVerdict {
  const reactNegligible = Math.abs(reactDelta) < epsilon;
  const wgpuNegligible = Math.abs(wgpuDelta) < epsilon;
  if (reactNegligible && wgpuNegligible) return "negligible-both";
  if (reactNegligible || wgpuNegligible) return "negligible-one-side";
  return Math.sign(reactDelta) === Math.sign(wgpuDelta) ? "same-direction" : "opposite-direction";
}
//#endregion

//#region Probes
type ProbeStatus = "PASS" | "FAIL" | "WARN";
type ProbeReport = { readonly probe: string; readonly status: ProbeStatus; readonly reasons: readonly string[] };

const NON_BG_EPSILON = 0.003;

/** 🎛️ Opens the command palette (Meta/Control+P) and expects a new overlay to paint non-background
 * pixels over the base screen in both renderers, then expects Escape to return both close to baseline. */
async function commandPaletteProbe(reactPage: Page, wgpuPage: Page): Promise<ProbeReport> {
  const reasons: string[] = [];
  let status: ProbeStatus = "PASS";
  const shortcut = process.platform === "darwin" ? "Meta+p" : "Control+p";
  const reactBaseline = await captureReactStats(reactPage);
  const wgpuBaseline = await captureWgpuStats(wgpuPage);
  await reactPage.keyboard.press(shortcut);
  await wgpuPage.keyboard.press(shortcut);
  await reactPage.waitForTimeout(400);
  await wgpuPage.waitForTimeout(400);
  const reactOpened = await captureReactStats(reactPage);
  const wgpuOpened = await captureWgpuStats(wgpuPage);
  const reactOpenDelta = reactOpened.overall.nonBgRatio - reactBaseline.overall.nonBgRatio;
  const wgpuOpenDelta = wgpuOpened.overall.nonBgRatio - wgpuBaseline.overall.nonBgRatio;
  const openVerdict = classifyDelta(reactOpenDelta, wgpuOpenDelta, NON_BG_EPSILON);
  reasons.push(`open: react Δ=${reactOpenDelta.toFixed(4)} wgpu Δ=${wgpuOpenDelta.toFixed(4)} verdict=${openVerdict}`);
  if (openVerdict === "opposite-direction" || openVerdict === "negligible-one-side") {
    status = "FAIL";
    reasons.push("expected a same-direction non-background increase in both renderers when the command palette opens");
  } else if (openVerdict === "negligible-both") {
    status = "WARN";
    reasons.push("neither renderer showed a measurable overlay change — command palette shortcut may not have registered");
  }
  await reactPage.keyboard.press("Escape");
  await wgpuPage.keyboard.press("Escape");
  await reactPage.waitForTimeout(300);
  await wgpuPage.waitForTimeout(300);
  const reactClosed = await captureReactStats(reactPage);
  const wgpuClosed = await captureWgpuStats(wgpuPage);
  const reactCloseResidual = Math.abs(reactClosed.overall.nonBgRatio - reactBaseline.overall.nonBgRatio);
  const wgpuCloseResidual = Math.abs(wgpuClosed.overall.nonBgRatio - wgpuBaseline.overall.nonBgRatio);
  reasons.push(`close: react residual=${reactCloseResidual.toFixed(4)} wgpu residual=${wgpuCloseResidual.toFixed(4)}`);
  const CLOSE_RESIDUAL_MAX = 0.01;
  if (reactCloseResidual > CLOSE_RESIDUAL_MAX || wgpuCloseResidual > CLOSE_RESIDUAL_MAX) {
    if (status === "PASS") status = "WARN";
    reasons.push(`expected both renderers to return within ${CLOSE_RESIDUAL_MAX} of baseline nonBgRatio after Escape`);
  }
  return { probe: "command-palette (plugin=s)", status, reasons };
}

/** 📋️ Two scripted clicks into the forms playground body; expects a body-region stat change in both
 * renderers (not necessarily the same field — just that *something* responded to the same clicks). */
async function formsProbe(reactPage: Page, wgpuPage: Page): Promise<ProbeReport> {
  const reasons: string[] = [];
  let status: ProbeStatus = "PASS";
  const reactBaseline = await captureReactStats(reactPage);
  const wgpuBaseline = await captureWgpuStats(wgpuPage);
  await clickViewport(reactPage, 0.55, 0.5);
  await clickViewport(wgpuPage, 0.55, 0.5);
  await reactPage.waitForTimeout(250);
  await wgpuPage.waitForTimeout(250);
  await clickViewport(reactPage, 0.82, 0.5);
  await clickViewport(wgpuPage, 0.82, 0.5);
  await reactPage.waitForTimeout(250);
  await wgpuPage.waitForTimeout(250);
  const reactAfter = await captureReactStats(reactPage);
  const wgpuAfter = await captureWgpuStats(wgpuPage);
  const reactDelta = Math.abs(reactAfter.body.nonBgRatio - reactBaseline.body.nonBgRatio) + Math.abs(reactAfter.body.meanLuma - reactBaseline.body.meanLuma) / 255;
  const wgpuDelta = Math.abs(wgpuAfter.body.nonBgRatio - wgpuBaseline.body.nonBgRatio) + Math.abs(wgpuAfter.body.meanLuma - wgpuBaseline.body.meanLuma) / 255;
  reasons.push(`react bodyDelta=${reactDelta.toFixed(4)} wgpu bodyDelta=${wgpuDelta.toFixed(4)}`);
  const CHANGE_EPSILON = 0.001;
  if (reactDelta < CHANGE_EPSILON && wgpuDelta < CHANGE_EPSILON) {
    status = "WARN";
    reasons.push("neither renderer showed a measurable body-region change after the scripted clicks");
  } else if (reactDelta < CHANGE_EPSILON || wgpuDelta < CHANGE_EPSILON) {
    status = "FAIL";
    reasons.push("only one renderer showed a measurable body-region change after identical scripted clicks");
  } else {
    reasons.push("both renderers show a measurable body-region change after the scripted clicks");
  }
  return { probe: "forms clicks (plugin=forms)", status, reasons };
}

const GENERATE_MODE_PLUGINS = ["flow", "procedural2d", "procedural3d"] as const;

/** 🌀️ Clicks the generate-mode toggle (same relative viewport coordinate used by
 * `verify-wgpu-playgrounds-e2e.ts`'s `generateModeSmoke`) and expects a same-direction,
 * roughly-comparable-magnitude body-region change in both renderers. */
async function generateModeProbe(reactPage: Page, wgpuPage: Page, plugin: (typeof GENERATE_MODE_PLUGINS)[number]): Promise<ProbeReport> {
  const reasons: string[] = [];
  let status: ProbeStatus = "PASS";
  const reactBaseline = await captureReactStats(reactPage);
  const wgpuBaseline = await captureWgpuStats(wgpuPage);
  await clickViewport(reactPage, 0.76, 0.04);
  await clickViewport(wgpuPage, 0.76, 0.04);
  await reactPage.waitForTimeout(800);
  await wgpuPage.waitForTimeout(800);
  const reactAfter = await captureReactStats(reactPage);
  const wgpuAfter = await captureWgpuStats(wgpuPage);
  const reactDelta = reactAfter.body.nonBgRatio - reactBaseline.body.nonBgRatio;
  const wgpuDelta = wgpuAfter.body.nonBgRatio - wgpuBaseline.body.nonBgRatio;
  const verdict = classifyDelta(reactDelta, wgpuDelta, NON_BG_EPSILON);
  reasons.push(`react Δ=${reactDelta.toFixed(4)} wgpu Δ=${wgpuDelta.toFixed(4)} verdict=${verdict}`);
  if (verdict === "negligible-both") {
    status = "WARN";
    reasons.push("neither renderer's body region changed after the generate-mode toggle click — toggle control may not be at this viewport coordinate in one or both renderers");
  } else if (verdict === "negligible-one-side" || verdict === "opposite-direction") {
    status = "FAIL";
    reasons.push("expected a same-direction body-region change in both renderers after the generate-mode toggle");
  } else {
    const reactClass = magnitudeClass(reactDelta);
    const wgpuClass = magnitudeClass(wgpuDelta);
    const classDiff = Math.abs(reactClass - wgpuClass);
    reasons.push(`magnitude class: react=${reactClass} wgpu=${wgpuClass} diff=${classDiff}`);
    if (classDiff > 2) {
      status = "WARN";
      reasons.push("same direction but magnitude classes diverge by more than 2 orders of magnitude");
    }
  }
  return { probe: `generate-mode toggle (plugin=${plugin})`, status, reasons };
}
//#endregion

//#region DevServers
async function waitForDev(url: string): Promise<void> {
  const deadline = Date.now() + bootTimeoutMs;
  while (Date.now() < deadline) {
    try {
      const response = await fetch(url);
      if (response.ok) return;
    } catch {}
    await Bun.sleep(500);
  }
  throw new Error(`dev server not ready at ${url}`);
}

function spawnDevServer(renderer: "react" | "wgpu", port: number): Subprocess {
  return spawn({
    cmd: [bunExe, "nx", "run", "@semio-tech/framework-os-dev:dev"],
    cwd: repoRoot,
    stdout: "pipe",
    stderr: "pipe",
    env: {
      ...process.env,
      SKIP_PLUGIN_BUILD: process.env.SKIP_PLUGIN_BUILD ?? "1",
      SKIP_WGPU_BUILD: process.env.SKIP_WGPU_BUILD ?? "1",
      SKIP_ENGINE_BUILD: process.env.SKIP_ENGINE_BUILD ?? "1",
      S_OS_PORT: String(port),
      SEMIO_RENDERER: renderer,
      SEMIO_PLUGIN: "s",
    },
  });
}
//#endregion

//#region Report
function writeMarkdownReport(reports: readonly ProbeReport[], ports: { react: number; wgpu: number; reactFallback: boolean; wgpuFallback: boolean }): string {
  const counts = { PASS: 0, FAIL: 0, WARN: 0 };
  for (const r of reports) counts[r.status] += 1;
  const lines: string[] = [];
  lines.push("# Wgpu ↔ React Interaction-Delta Parity Report");
  lines.push("");
  lines.push(
    "Generated by `interaction-parity.ts`: an identical scripted interaction sequence is run against both renderers " +
      "(react and wgpu each on their own dev server instance of `@semio-tech/framework-os-dev:dev`), and the BEFORE/AFTER " +
      "delta of region statistics (not absolute pixels) is compared — see `parity-verify.ts`'s header for why absolute " +
      "pixel comparison isn't a realistic bar across these two structurally different rendering pipelines.",
  );
  lines.push("");
  lines.push(
    `Ports: react=${ports.react}${ports.reactFallback ? " (fallback, preferred port was taken)" : ""}, wgpu=${ports.wgpu}${ports.wgpuFallback ? " (fallback, preferred port 7301 was taken)" : ""}.`,
  );
  lines.push("");
  lines.push(`**Summary:** ${counts.PASS} PASS / ${counts.FAIL} FAIL / ${counts.WARN} WARN (of ${reports.length})`);
  lines.push("");
  lines.push("| Probe | Status | Notes |");
  lines.push("|---|---|---|");
  for (const r of reports) {
    lines.push(`| ${r.probe} | ${r.status} | ${r.reasons.join("; ").replace(/\|/g, "\\|")} |`);
  }
  return lines.join("\n");
}
//#endregion

async function main(): Promise<void> {
  const { port: wgpuPort, usedFallback: wgpuFallback } = await pickPort("wgpu", process.env.INTERACTION_WGPU_PORT, 7301, [7303, 7305, 7307]);
  const { port: reactPort, usedFallback: reactFallback } = await pickPort("react", process.env.INTERACTION_REACT_PORT, 7300, [7304, 7306, 7308]);
  console.log(`[DEBUG] interaction-parity ports: react=${reactPort} wgpu=${wgpuPort}`);

  let reactDev: Subprocess | null = null;
  let wgpuDev: Subprocess | null = null;
  const reports: ProbeReport[] = [];
  try {
    reactDev = spawnDevServer("react", reactPort);
    wgpuDev = spawnDevServer("wgpu", wgpuPort);
    const reactBaseUrl = `http://127.0.0.1:${reactPort}/`;
    const wgpuBaseUrl = `http://127.0.0.1:${wgpuPort}/`;
    await Promise.all([waitForDev(reactBaseUrl), waitForDev(wgpuBaseUrl)]);

    const browser = await chromium.launch({
      headless: !headed,
      args: ["--enable-unsafe-webgpu", "--use-angle=swiftshader", "--enable-features=Vulkan"],
    });
    try {
      const reactShellPage = await openReactPage(browser, reactBaseUrl, "s");
      const wgpuShellPage = await openWgpuPage(browser, wgpuBaseUrl, "s");
      try {
        reports.push(await commandPaletteProbe(reactShellPage, wgpuShellPage));
      } finally {
        await reactShellPage.close();
        await wgpuShellPage.close();
      }

      const reactFormsPage = await openReactPage(browser, reactBaseUrl, "forms");
      const wgpuFormsPage = await openWgpuPage(browser, wgpuBaseUrl, "forms");
      try {
        reports.push(await formsProbe(reactFormsPage, wgpuFormsPage));
      } finally {
        await reactFormsPage.close();
        await wgpuFormsPage.close();
      }

      for (const plugin of GENERATE_MODE_PLUGINS) {
        const reactPage = await openReactPage(browser, reactBaseUrl, plugin);
        const wgpuPage = await openWgpuPage(browser, wgpuBaseUrl, plugin);
        try {
          reports.push(await generateModeProbe(reactPage, wgpuPage, plugin));
        } catch (error) {
          reports.push({ probe: `generate-mode toggle (plugin=${plugin})`, status: "FAIL", reasons: [`probe threw: ${error instanceof Error ? error.message : String(error)}`] });
        } finally {
          await reactPage.close();
          await wgpuPage.close();
        }
      }
    } finally {
      await browser.close();
    }

    const mdPath = join(ticketDir, "interaction-parity-report.md");
    const jsonPath = join(ticketDir, "interaction-parity-report.json");
    const ports = { react: reactPort, wgpu: wgpuPort, reactFallback, wgpuFallback };
    await Bun.write(mdPath, `${writeMarkdownReport(reports, ports)}\n`);
    await Bun.write(jsonPath, `${JSON.stringify({ ports, reports }, null, 2)}\n`);

    const counts = { PASS: 0, FAIL: 0, WARN: 0 };
    for (const r of reports) counts[r.status] += 1;
    console.log(`\n${counts.PASS} PASS / ${counts.FAIL} FAIL / ${counts.WARN} WARN (of ${reports.length})`);
    console.log(`report: ${mdPath}`);
    console.log(`report: ${jsonPath}`);
    process.exit(counts.FAIL > 0 ? 1 : 0);
  } finally {
    reactDev?.kill();
    wgpuDev?.kill();
    await reactDev?.exited;
    await wgpuDev?.exited;
  }
}

await main();
