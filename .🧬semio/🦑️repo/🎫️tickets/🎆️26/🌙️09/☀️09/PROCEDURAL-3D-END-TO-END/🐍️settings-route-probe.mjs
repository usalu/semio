/** 🔎 Find the live route from the shell chrome to `os.setLocale`: open the Settings panel, dump its
 * controls, flip Language to German, and report what text changed. */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", "react-i18n-a11y", process.env.SEMIO_PROBE_OUT ?? "settings-route");
mkdirSync(outDir, { recursive: true });
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
await page.goto(url, { waitUntil: "domcontentloaded" });
for (let i = 0; i < 90; i++) { await page.waitForTimeout(1000); if (await page.locator('[data-surface-id="window:procedural-preview"]').count()) break; }
await page.waitForTimeout(6000);

const beforeText = await page.evaluate(() => document.body.innerText.replace(/\s+/g, " "));
await page.locator("#framework\\.settings").first().click();
await page.waitForTimeout(1500);
await page.screenshot({ path: join(outDir, "settings-open.png") });

const panel = await page.evaluate(() => {
  const ids = [...document.querySelectorAll("[id^='framework.settings']")].map((el) => ({ id: el.id, tag: el.tagName, role: el.getAttribute("role"), text: (el.innerText || "").replace(/\s+/g, " ").slice(0, 60) }));
  const combos = [...document.querySelectorAll("[role='combobox'],select")].map((el) => ({ id: el.id, role: el.getAttribute("role"), text: (el.innerText || "").replace(/\s+/g, " ").slice(0, 40) }));
  return { ids, combos, text: document.body.innerText.replace(/\s+/g, " ").slice(0, 3000) };
});
writeFileSync(join(outDir, "panel.json"), JSON.stringify(panel, null, 2));
console.log("[DEBUG] settings ids", JSON.stringify(panel.ids));
console.log("[DEBUG] combos", JSON.stringify(panel.combos));

const lang = page.locator("#framework\\.settings\\.language").first();
console.log("[DEBUG] language control count", await lang.count());
if (await lang.count()) {
  await lang.click();
  await page.waitForTimeout(900);
  await page.screenshot({ path: join(outDir, "language-open.png") });
  const opts = await page.evaluate(() => [...document.querySelectorAll("[role='option']")].map((el) => ({ v: el.getAttribute("data-value") ?? el.getAttribute("value"), text: (el.textContent || "").slice(0, 40) })));
  console.log("[DEBUG] options", JSON.stringify(opts));
  writeFileSync(join(outDir, "options.json"), JSON.stringify(opts, null, 2));
  const de = page.locator("[role='option']").filter({ hasText: /Deutsch|German/ }).first();
  if (await de.count()) { await de.click(); console.log("[DEBUG] clicked German option"); }
  else { await page.locator("[role='option']").nth(1).click(); console.log("[DEBUG] clicked option #1 as fallback"); }
  await page.waitForTimeout(3000);
}
await page.keyboard.press("Escape");
await page.waitForTimeout(2500);
const afterText = await page.evaluate(() => document.body.innerText.replace(/\s+/g, " "));
await page.screenshot({ path: join(outDir, "after-de.png") });
writeFileSync(join(outDir, "before.txt"), beforeText);
writeFileSync(join(outDir, "after.txt"), afterText);
console.log("[DEBUG] changed", beforeText !== afterText);
console.log("[DEBUG] after head", afterText.slice(0, 1200));
console.log("DONE");
await browser.close();
