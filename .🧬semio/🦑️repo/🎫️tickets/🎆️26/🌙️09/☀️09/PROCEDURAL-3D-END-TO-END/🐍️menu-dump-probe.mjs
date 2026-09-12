/** 🔎 Dump what the Generations window's Actions button opens (roles, ids, texts) in generate mode. */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";
const outDir = join(import.meta.dir, "🗑️generated", "menu-dump"); mkdirSync(outDir, { recursive: true });
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.goto("http://127.0.0.1:6018/?plugin=generation3d", { waitUntil: "domcontentloaded" });
for (let i = 0; i < 60; i++) { await page.waitForTimeout(1000); const n = await page.locator('[data-surface-id="window:procedural-preview"]').count(); if (n) break; }
await page.waitForTimeout(8000);
await page.keyboard.press("Meta+Alt+ArrowRight"); await page.waitForTimeout(6000);
const before = await page.evaluate(() => document.body.innerText.length);
const buttons = page.locator('button:has-text("Actions")');
const n = await buttons.count(); console.log("[DEBUG] actions buttons", n);
for (let i = 0; i < n; i++) {
  const box = await buttons.nth(i).boundingBox();
  await buttons.nth(i).click({ timeout: 3000 }).catch((e) => console.log("click failed", String(e).slice(0, 80)));
  await page.waitForTimeout(700);
  const dump = await page.evaluate(() => {
    const els = [...document.querySelectorAll('[role="menu"], [role="menuitem"], [role="listbox"], [data-slot*="menu"], [data-radix-popper-content-wrapper], [data-state="open"], [aria-expanded="true"]')];
    return els.slice(0, 40).map((e) => ({ tag: e.tagName, role: e.getAttribute("role"), slot: e.getAttribute("data-slot"), id: e.id, aid: e.getAttribute("data-action-id"), text: (e.innerText || "").replace(/\s+/g, " ").slice(0, 120) }));
  });
  console.log(`[DEBUG] button ${i} at ${JSON.stringify(box)} ->`, JSON.stringify(dump).slice(0, 1500));
  await page.screenshot({ path: join(outDir, `actions-${i}.png`) });
  await page.keyboard.press("Escape"); await page.waitForTimeout(300);
}
console.log("DONE"); await browser.close();
