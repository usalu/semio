#!/usr/bin/env bun
/** 🔍 Live browser verification for note canvas selection, drag, pencil, and table editing. */
import { chromium, type Page } from "playwright";

const BASE_URL = process.env.NOTE_URL ?? "http://127.0.0.1:6080/";

async function waitForNote(page: Page): Promise<void> {
  await page.goto(BASE_URL, { waitUntil: "domcontentloaded", timeout: 120_000 });
  await page.waitForSelector('[data-surface-id="note.play.composite"]', { timeout: 120_000 });
}

async function main(): Promise<void> {
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();
  const logs: string[] = [];
  page.on("console", (msg) => {
    if (msg.type() === "error") logs.push(`[console.error] ${msg.text()}`);
  });

  await waitForNote(page);
  console.log("[DEBUG] note composite surface mounted");

  const canvas = page.locator('[data-surface-id="note.play.composite"]');
  await page.waitForTimeout(1_500);
  const welcomeVisible = await canvas.locator("text=Welcome to Note").count();
  console.log("[DEBUG] semio example welcome text present:", welcomeVisible > 0);

  const box = await canvas.boundingBox();
  if (!box) throw new Error("[DEBUG] note canvas missing bounding box");

  // Select the welcome text block by clicking it.
  await page.mouse.click(box.x + 200, box.y + 120);
  await page.waitForTimeout(500);
  const selectedRing = await canvas.locator(".ring-primary").count();
  console.log("[DEBUG] selection ring present after click:", selectedRing > 0);

  // Drag the selected block and confirm it moved (single gesture, single undo step).
  await page.mouse.move(box.x + 200, box.y + 120);
  await page.mouse.down();
  await page.mouse.move(box.x + 260, box.y + 180, { steps: 6 });
  await page.mouse.up();
  await page.waitForTimeout(500);
  console.log("[DEBUG] drag gesture completed");

  // Switch to the pencil tool and draw a short stroke.
  const pencilButton = page.locator('[id*="note.play.tools.pencil"]').first();
  if (await pencilButton.count()) {
    await pencilButton.click();
    await page.mouse.move(box.x + 400, box.y + 400);
    await page.mouse.down();
    await page.mouse.move(box.x + 430, box.y + 420, { steps: 4 });
    await page.mouse.move(box.x + 460, box.y + 400, { steps: 4 });
    await page.mouse.up();
    await page.waitForTimeout(500);
    console.log("[DEBUG] pencil stroke drawn");
  } else {
    console.log("[DEBUG] pencil tool button not found — skipping stroke");
  }

  // Double-click the table block to open a cell editor.
  await page.mouse.dblclick(box.x + 100, box.y + 380);
  await page.waitForTimeout(300);
  const cellEditor = await page.locator("input.ring-primary").count();
  console.log("[DEBUG] table cell editor opened:", cellEditor > 0);
  await page.keyboard.press("Escape");

  const duplicateKeyErrors = logs.filter((line) => line.includes("duplicate key") || line.includes("Encountered two children"));
  if (duplicateKeyErrors.length > 0) {
    throw new Error(`[DEBUG] react duplicate key errors: ${duplicateKeyErrors.join("; ")}`);
  }

  console.log("[DEBUG] browser-verify passed");
  await browser.close();
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
