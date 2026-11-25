/** 🔎️ Opens the walking commuter's page of a built site at one width and language and prints what an expression evaluated in the page returns.
 * Usage: node probe.mjs <release-directory|url> <width> <height> <en|de> <touch|mouse> <expression> [screenshot.png] */
import { chromium } from "playwright";
import { serve } from "../../../../☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS/serve.mjs";

const [site, width, height, language, pointer, expression, shot] = process.argv.slice(2);
const local = site.startsWith("http") ? null : await serve(site);
const base = local?.url ?? site;
const browser = await chromium.launch({ channel: "chromium", args: ["--disable-crash-reporter", "--disable-crashpad"] });
try {
  const touch = pointer === "touch";
  const context = await browser.newContext({ viewport: { width: Number(width), height: Number(height) }, hasTouch: touch, isMobile: touch, locale: "fr-FR" });
  const page = await context.newPage();
  page.on("pageerror", (error) => console.log("[DEBUG] pageerror", error.message));
  await page.goto(base, { waitUntil: "domcontentloaded" });
  const choice = page.getByRole("button", { name: language === "de" ? "Deutsch" : "English", exact: true });
  await choice.waitFor({ state: "visible", timeout: 120000 });
  await (touch ? choice.tap() : choice.click());
  await page.goto(new URL("#/humans/walking-commuter?t=0.5", base).href, { waitUntil: "domcontentloaded" });
  await page.locator('[data-page="asset:walking-commuter"]:visible [data-viewer] [data-stage] [data-staffage-figure] svg').first().waitFor({ state: "visible", timeout: 180000 });
  await page.evaluate(async () => { await document.fonts.ready; });
  await page.waitForTimeout(1500);
  console.log(JSON.stringify(await page.evaluate(expression), null, 1));
  if (shot) await page.locator('[data-page="asset:walking-commuter"]:visible [data-viewer]').first().screenshot({ path: shot });
} finally {
  await browser.close();
  local?.server.close();
}
