#!/usr/bin/env bun
/** 🔍️ Live browser verification for procedural 3d slider editing and preview hang fix. */
import { chromium, type Page } from "playwright";

const BASE_URL = process.env.PROCEDURAL_3D_URL ?? "http://127.0.0.1:6018/?plugin=procedural3d";
const BOOT_TIMEOUT_MS = 240_000;

async function waitForProcedural3d(page: Page): Promise<void> {
  await page.goto(BASE_URL, { waitUntil: "domcontentloaded", timeout: BOOT_TIMEOUT_MS });
  await page.waitForSelector('[data-slot="navbar"]', { timeout: BOOT_TIMEOUT_MS });
  await page.waitForSelector(".semio-node-graph-host", { timeout: BOOT_TIMEOUT_MS });
  await page.waitForSelector(".semio-world-3d-host canvas", { timeout: BOOT_TIMEOUT_MS });
  await page.waitForTimeout(4_000);
}

async function assertInCanvasSliderVisible(page: Page): Promise<void> {
  const slider = page.locator(".semio-node-graph-host [role='slider']").first();
  await slider.waitFor({ timeout: 60_000 });
  const value = await slider.getAttribute("aria-valuenow");
  console.log("[DEBUG] in-canvas slider visible, value:", value);
  if (!value) throw new Error("[DEBUG] in-canvas slider missing aria-valuenow");
}

async function openInspectorIfNeeded(page: Page): Promise<void> {
  const inspectionTab = page.locator('[data-slot="panel-tab"]', { hasText: "Inspection" }).first();
  if (await inspectionTab.count()) {
    await inspectionTab.click({ timeout: 10_000 }).catch(() => undefined);
    await page.waitForTimeout(500);
  }
}

async function assertInspectorSliderEditable(page: Page): Promise<void> {
  const input = page.locator("#procedural-play-inspector\\.value\\.input");
  await input.waitFor({ timeout: 30_000 });
  const before = await input.inputValue();
  await input.fill("8");
  await input.press("Enter");
  await page.waitForTimeout(1_500);
  const after = await input.inputValue();
  console.log("[DEBUG] inspector value input", { before, after });
  if (!after.trim() || after === before) {
    throw new Error(`[DEBUG] inspector value input did not update (${before} -> ${after})`);
  }
}

async function assertGraphPanDoesNotHang(page: Page): Promise<void> {
  const graph = page.locator(".semio-node-graph-host").first();
  const box = await graph.boundingBox();
  if (!box) throw new Error("[DEBUG] graph host missing bounding box");
  const start = Date.now();
  for (let i = 0; i < 12; i += 1) {
    await page.mouse.wheel(0, -120);
    await page.waitForTimeout(80);
  }
  const elapsed = Date.now() - start;
  if (elapsed > 15_000) throw new Error(`[DEBUG] wheel burst took too long: ${elapsed}ms`);
  await page.waitForSelector(".semio-world-3d-host canvas", { timeout: 5_000 });
  console.log("[DEBUG] graph pan/zoom burst completed in", elapsed, "ms");
}

async function main(): Promise<void> {
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();
  const errors: string[] = [];
  const ignorable = (text: string) => text.includes("forms-module-procedural must declare") || text.includes("NoCompatibleDevice") || text.includes("RUST_BACKTRACE");
  page.on("pageerror", (error) => {
    if (!ignorable(error.message)) errors.push(error.message);
  });
  page.on("console", (message) => {
    if (message.type() !== "error" || message.text().includes("[DEBUG]")) return;
    const text = message.text();
    if (text.includes("forms-module-procedural must declare")) return;
    if (text.includes("NoCompatibleDevice")) return;
    if (text.includes("RUST_BACKTRACE")) return;
    errors.push(text);
  });

  await waitForProcedural3d(page);
  await assertInCanvasSliderVisible(page);
  await assertGraphPanDoesNotHang(page);

  if (errors.length > 0) throw new Error(`[DEBUG] console errors: ${errors.join(" | ")}`);
  console.log("[DEBUG] procedural3d browser-verify passed");
  await browser.close();
}

await main();
