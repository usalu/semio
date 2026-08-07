#!/usr/bin/env bun
/** 🔍️ Hex column eval settle probe — asserts per-node statusJson reaches all ok after flowEvalTick convergence. */
import { chromium, type Page } from "playwright";
import { mkdir, writeFile } from "node:fs/promises";
import path from "node:path";

const ticketDir = path.dirname(new URL(import.meta.url).pathname);
const renderer = process.env.PROCEDURAL_3D_RENDERER ?? "react";
const port = renderer === "wgpu" ? "6118" : "6018";
const BASE_URL = process.env.PROCEDURAL_3D_URL ?? `http://127.0.0.1:${port}/?plugin=procedural3d&example=hexagonal-mushroom-column`;
const BOOT_TIMEOUT_MS = 240_000;

type StatusMap = Record<string, { status?: string; message?: string; ports?: string[] }>;

function parseStatus(raw: string | null): StatusMap {
  if (!raw) return {};
  try {
    return JSON.parse(raw) as StatusMap;
  } catch {
    return {};
  }
}

function allOk(status: StatusMap): boolean {
  const entries = Object.values(status);
  if (entries.length === 0) return false;
  return entries.every((entry) => entry.status === "ok");
}

async function readStatusJson(page: Page): Promise<string | null> {
  return page.evaluate(() => document.querySelector(".semio-node-graph-host")?.getAttribute("data-status-json") ?? null);
}

async function waitForSettle(page: Page, label: string): Promise<StatusMap> {
  const deadline = Date.now() + 120_000;
  let last: StatusMap = {};
  while (Date.now() < deadline) {
    const raw = await readStatusJson(page);
    last = parseStatus(raw);
    if (allOk(last)) return last;
    await page.waitForTimeout(500);
  }
  throw new Error(`${label}: timed out waiting for all nodes ok; last=${JSON.stringify(last)}`);
}

async function main(): Promise<void> {
  const logs: string[] = [];
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();
  page.on("console", (msg) => logs.push(`[${msg.type()}] ${msg.text()}`));
  await page.goto(BASE_URL, { waitUntil: "domcontentloaded", timeout: BOOT_TIMEOUT_MS });
  await page.waitForSelector(".semio-node-graph-host", { timeout: BOOT_TIMEOUT_MS });
  const initial = await waitForSettle(page, "initial load");
  console.log("[DEBUG] initial settle", JSON.stringify(initial));
  const sidesSlider = page.locator('input[type="range"]').first();
  if (await sidesSlider.count()) {
    await sidesSlider.focus();
    await sidesSlider.press("ArrowRight");
    await page.waitForTimeout(300);
    const midRaw = await readStatusJson(page);
    console.log("[DEBUG] mid-edit status", midRaw);
    await waitForSettle(page, "after sides edit");
  }
  await page.screenshot({ path: path.join(ticketDir, `hex-column-${renderer}.png`), fullPage: true });
  await mkdir(ticketDir, { recursive: true });
  await writeFile(path.join(ticketDir, `hex-column-${renderer}-console.log`), logs.join("\n"), "utf8");
  await browser.close();
  console.log(`probe ok (${renderer})`);
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
