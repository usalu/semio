/** 🌓️ Where does `setAppearance("dark")` actually land in the DOM on 6018 — `<html>`, the
 * `.semio-scope` root, or nowhere? Reports both before and after the flip, and after a reload. */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", "react-i18n-a11y", process.env.SEMIO_PROBE_OUT ?? "appearance-scope");
mkdirSync(outDir, { recursive: true });
const browser = await chromium.launch({ headless: true });
const context = await browser.newContext({ viewport: { width: 1600, height: 1000 } });
const page = await context.newPage();

async function boot(p, tag) {
  for (let i = 0; i < 150; i++) { await p.waitForTimeout(1000); if (await p.locator('[data-surface-id="window:procedural-preview"]').count()) { console.log(`[DEBUG] boot${tag} ${i + 1}s`); break; } }
  await p.waitForTimeout(6000);
}
const look = (p, tag) => p.evaluate((t) => {
  const scope = document.querySelector(".semio-scope") ?? document.querySelector("[data-ui-appearance]");
  const body = document.body;
  const root = document.documentElement;
  const pick = (el) => el ? { tag: el.tagName, cls: el.className, appearance: el.getAttribute("data-ui-appearance"), bg: getComputedStyle(el).backgroundColor, color: getComputedStyle(el).color } : null;
  return { t, root: pick(root), body: pick(body), scope: pick(scope), scopeFound: Boolean(scope), scopeSelectorCount: document.querySelectorAll(".semio-scope").length, appearanceAttrNodes: document.querySelectorAll("[data-ui-appearance]").length, darkNodes: document.querySelectorAll(".dark").length };
}, tag);

await page.goto(url, { waitUntil: "domcontentloaded" });
await boot(page, "-1");
const a = await look(page, "baseline");
console.log("[DEBUG] baseline", JSON.stringify(a));
await page.screenshot({ path: join(outDir, "1-light.png") });

if (!(await page.locator("#framework\\.settings\\.appearance").count())) { await page.locator("#framework\\.settings").first().click(); await page.waitForTimeout(1500); }
await page.locator("button#framework\\.settings\\.appearance").first().click();
await page.waitForTimeout(800);
const opts = await page.evaluate(() => [...document.querySelectorAll("[role='option']")].map((e) => ({ v: e.getAttribute("data-value"), t: (e.textContent || "").trim().slice(0, 30) })));
console.log("[DEBUG] appearance options", JSON.stringify(opts));
await page.locator("[role='option']").filter({ hasText: /Dark|Dunkel/ }).first().click();
await page.waitForTimeout(3000);
const b = await look(page, "after-dark");
console.log("[DEBUG] after-dark", JSON.stringify(b));
await page.screenshot({ path: join(outDir, "2-dark.png") });

await page.reload({ waitUntil: "domcontentloaded" });
await boot(page, "-reload");
const c = await look(page, "after-reload");
console.log("[DEBUG] after-reload", JSON.stringify(c));
await page.screenshot({ path: join(outDir, "3-dark-reloaded.png") });

writeFileSync(join(outDir, "result.json"), JSON.stringify({ baseline: a, afterDark: b, afterReload: c, options: opts }, null, 2));
console.log("DONE");
await browser.close();
