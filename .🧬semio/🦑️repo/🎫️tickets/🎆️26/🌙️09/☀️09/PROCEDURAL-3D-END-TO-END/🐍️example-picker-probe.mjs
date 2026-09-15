/** 🗂️ What the editor's example picker actually OFFERS, and whether every offered example can be
 * selected from it.
 *
 * 🪪️ The journey probe died mid-walk on `click: Timeout … waiting for locator('[role="option"]')
 * .filter({ hasText: 'Box Shell Preview' })` after seven successful picks, which is either "the
 * eighth example is not published" or "the popup lists it but something covers it". This probe
 * answers the first question on its own, without a 10-minute journey in front of it.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=picker bun 🐍️example-picker-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "example-picker");
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 1200)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 1200)}`));

await page.goto(url, { waitUntil: "domcontentloaded" });
for (let i = 0; i < 180; i += 1) {
  const ready = await page.evaluate(() => document.querySelectorAll("[data-status-json]").length > 0);
  if (ready) break;
  await page.waitForTimeout(1000);
}
await page.waitForTimeout(4000);

const combo = page.locator('[role="combobox"]').first();
await combo.click({ timeout: 10000 });
await page.waitForTimeout(600);
const options = await page.locator('[role="option"]').allInnerTexts();
const trimmed = options.map((text) => text.replace(/\s+/g, " ").trim()).filter(Boolean);
await page.screenshot({ path: join(outDir, "picker-open.png") });
await page.keyboard.press("Escape");

const result = { options: trimmed, count: trimmed.length };
writeFileSync(join(outDir, "results.json"), JSON.stringify(result, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log(`[DEBUG] picker offers ${trimmed.length}: ${JSON.stringify(trimmed)}`);
await browser.close();
