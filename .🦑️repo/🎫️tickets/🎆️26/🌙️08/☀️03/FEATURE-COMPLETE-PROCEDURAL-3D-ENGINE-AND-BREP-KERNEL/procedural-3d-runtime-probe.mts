#!/usr/bin/env bun
/** 🔍️ Runtime probe for procedural 3D — mesh/instance counts, example switching, full console. */
import { chromium, type Page } from "playwright";
import { mkdir, writeFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const ticketDir = path.dirname(fileURLToPath(import.meta.url));
const renderer = process.env.PROCEDURAL_3D_RENDERER ?? "react";
const port = renderer === "wgpu" ? "6118" : "6018";
const BASE_URL = process.env.PROCEDURAL_3D_URL ?? `http://127.0.0.1:${port}/?plugin=procedural3d`;
const BOOT_TIMEOUT_MS = 240_000;

const EXAMPLE_LABELS = [
  "Sphere Cut With Torus",
  "Box Fillet Preview",
  "Box Shell Preview",
  "Face Sweep Extrude",
  "Hexagonal Mushroom Column",
  "Rectangle Extrude Volume",
  "Rectangle Wire Preview",
  "Sphere Box Fuse",
];

type SceneProbe = { meshCount: number; instanceCount: number; graphNodeCount: number };

async function readSceneProbe(page: Page): Promise<SceneProbe> {
  return page.evaluate(() => {
    const world = document.querySelector(".semio-world-3d-host");
    const meshesJson = world?.getAttribute("data-meshes-json") ?? world?.querySelector("[data-meshes-json]")?.getAttribute("data-meshes-json");
    const instancesJson = world?.getAttribute("data-instances-json") ?? world?.querySelector("[data-instances-json]")?.getAttribute("data-instances-json");
    let meshCount = 0;
    let instanceCount = 0;
    if (meshesJson) {
      try {
        meshCount = JSON.parse(meshesJson).length;
      } catch {
        meshCount = 0;
      }
    }
    if (instancesJson) {
      try {
        instanceCount = JSON.parse(instancesJson).length;
      } catch {
        instanceCount = 0;
      }
    }
    const fixtureJson = document.querySelector(".semio-node-graph-host")?.getAttribute("data-fixture-json");
    let graphNodeCount = 0;
    if (fixtureJson) {
      try {
        const fixture = JSON.parse(fixtureJson) as { widgets?: unknown[] };
        graphNodeCount = fixture.widgets?.length ?? 0;
      } catch {
        graphNodeCount = 0;
      }
    }
    return { meshCount, instanceCount, graphNodeCount };
  });
}

async function openExampleDropdown(page: Page): Promise<void> {
  const trigger = page.locator('[data-testid="playground.navbar.fixture"], [data-slot="navbar"] [data-testid*="fixture"], [data-slot="navbar"] button').filter({ hasText: /example|beispiel|column|mushroom/i }).first();
  if (await trigger.count()) {
    await trigger.click({ timeout: 30_000 });
    await page.waitForTimeout(400);
    return;
  }
  const fallback = page.locator('[data-slot="navbar"] select, [data-slot="navbar"] [role="combobox"]').first();
  await fallback.click({ timeout: 30_000 });
  await page.waitForTimeout(400);
}

async function pickExample(page: Page, label: string): Promise<void> {
  await openExampleDropdown(page);
  const option = page.getByRole("option", { name: new RegExp(label, "i") }).or(page.getByText(label, { exact: false }));
  await option.first().click({ timeout: 30_000 });
  const deadline = Date.now() + 60_000;
  while (Date.now() < deadline) {
    const probe = await readSceneProbe(page);
    if (probe.graphNodeCount > 0) break;
    await page.waitForTimeout(500);
  }
  await page.waitForTimeout(2_000);
}

async function waitForBoot(page: Page): Promise<void> {
  await page.goto(BASE_URL, { waitUntil: "domcontentloaded", timeout: BOOT_TIMEOUT_MS });
  await page.waitForSelector('[data-slot="navbar"]', { timeout: BOOT_TIMEOUT_MS });
  await page.waitForSelector(".semio-node-graph-host", { timeout: BOOT_TIMEOUT_MS });
  await page.waitForSelector(".semio-world-3d-host canvas", { timeout: BOOT_TIMEOUT_MS });
  const deadline = Date.now() + 90_000;
  while (Date.now() < deadline) {
    const ready = await page.evaluate(() => {
      const graph = document.querySelector(".semio-node-graph-host");
      const world = document.querySelector(".semio-world-3d-host");
      const fixtureJson = graph?.getAttribute("data-fixture-json");
      const statusJson = graph?.getAttribute("data-status-json");
      const meshesJson = world?.getAttribute("data-meshes-json");
      let graphNodeCount = 0;
      let meshCount = 0;
      try {
        if (fixtureJson) graphNodeCount = (JSON.parse(fixtureJson) as { widgets?: unknown[] }).widgets?.length ?? 0;
      } catch {}
      try {
        if (!graphNodeCount && statusJson) graphNodeCount = Object.keys(JSON.parse(statusJson) as object).length;
      } catch {}
      try {
        if (meshesJson) meshCount = (JSON.parse(meshesJson) as unknown[]).length;
      } catch {}
      return graphNodeCount > 0 || meshCount > 0;
    });
    if (ready) break;
    await page.waitForTimeout(500);
  }
  await page.waitForTimeout(2_000);
}

async function safeScreenshot(page: Page, filePath: string): Promise<boolean> {
  try {
    if (page.isClosed()) return false;
    await page.screenshot({ path: filePath, fullPage: false, timeout: 15_000 });
    return true;
  } catch (error) {
    console.log("[DEBUG] screenshot failed", filePath, error);
    return false;
  }
}

async function main(): Promise<void> {
  const consoleLines: string[] = [];
  const browser = await chromium.launch({
    headless: true,
    args: ["--enable-unsafe-webgpu", "--use-angle=swiftshader", "--enable-features=Vulkan,UseSkiaRenderer"],
  });
  const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
  page.on("console", (message) => {
    consoleLines.push(`[${message.type()}] ${message.text()}`);
    console.log(`[console:${message.type()}]`, message.text());
  });
  page.on("pageerror", (error) => {
    consoleLines.push(`[pageerror] ${error.message}`);
    console.log("[pageerror]", error.message);
  });
  page.on("crash", () => {
    consoleLines.push("[crash] page crashed");
    console.log("[crash] page crashed");
  });

  await waitForBoot(page);
  const bootProbe = await readSceneProbe(page);
  console.log("[DEBUG] bootProbe", bootProbe);
  const actionFailed = consoleLines.some((line) => line.includes("action failed") && line.includes("setActiveExample"));
  await safeScreenshot(page, path.join(ticketDir, `probe-${renderer}-boot.png`));

  const perExample: Record<string, SceneProbe> = {};
  const signatures = new Set<string>();
  for (const label of EXAMPLE_LABELS) {
    if (page.isClosed()) {
      consoleLines.push(`[crash] page closed before example ${label}`);
      break;
    }
    try {
      await pickExample(page, label);
      const probe = await readSceneProbe(page);
      perExample[label] = probe;
      const sig = `${probe.graphNodeCount}:${probe.meshCount}:${probe.instanceCount}`;
      signatures.add(sig);
      await safeScreenshot(page, path.join(ticketDir, `probe-${renderer}-${label.replace(/\s+/g, "-").toLowerCase()}.png`));
    } catch (error) {
      consoleLines.push(`[example-error] ${label}: ${String(error)}`);
      console.log("[DEBUG] example failed", label, error);
    }
  }

  const report = {
    renderer,
    baseUrl: BASE_URL,
    bootProbe,
    actionFailedSetActiveExample: actionFailed,
    perExample,
    distinctExampleSignatures: signatures.size,
    graphPresent: bootProbe.graphNodeCount > 0,
    ok: bootProbe.graphNodeCount > 0 && bootProbe.meshCount > 0 && bootProbe.instanceCount > 0 && !actionFailed && signatures.size >= 2,
    consoleLineCount: consoleLines.length,
  };

  await writeFile(path.join(ticketDir, `probe-${renderer}-report.json`), JSON.stringify(report, null, 2));
  await writeFile(path.join(ticketDir, `probe-${renderer}-console.txt`), consoleLines.join("\n"));
  await browser.close();

  console.log("[DEBUG] procedural 3d runtime probe report", report);
  if (!report.ok) {
    throw new Error(`[DEBUG] procedural 3d probe failed: ${JSON.stringify(report)}`);
  }
  console.log("[DEBUG] procedural 3d runtime probe passed", report);
}

await main();
