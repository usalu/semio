import assert from "node:assert/strict";
import { chromium } from "playwright";
import catalog from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔣️.json" with { type: "json" };

const origin = "http://127.0.0.1:6281";
const browser = await chromium.launch({ channel: "chromium" });
try {
  const context = await browser.newContext({ baseURL: origin, viewport: { width: 1280, height: 800 }, locale: "en-US" });
  await globalThis.routeTicketBrowser?.(context, origin);
  const page = await context.newPage();
  const problems = [];
  page.on("pageerror", (error) => problems.push(error.message));
  await page.goto("/", { waitUntil: "domcontentloaded" });
  const root = '[data-level="Archive overview"][data-grid-active] [data-mode="strip"]';
  try {
    await page.waitForFunction((selector) => document.querySelector(selector) !== null, root, { timeout: 60000 });
  } catch (error) {
    console.error("[DEBUG] Staffage desktop startup", await page.evaluate(() => ({
      title: document.title,
      text: document.body.innerText.slice(0, 600),
      grids: [...document.querySelectorAll("[data-layered-overview]")].map((node) => [node.getAttribute("data-mode"), node.parentElement?.outerHTML.split(">")[0]]),
    })), problems);
    throw error;
  }
  const grid = page.locator(root).first();
  const rootCards = grid.locator(":scope > [data-layered-overlay] > [data-layered-card]");
  assert.equal(await rootCards.count(), catalog.categories.length + 4);
  const humans = rootCards.locator('a[href="#/humans"]').first();
  await humans.waitFor({ timeout: 15000 });
  await humans.focus();
  await humans.press("Enter");
  await page.waitForURL("**/#/humans");
  const category = page.locator('[data-level="Humans"][data-grid-active] [data-mode="strip"]').first();
  await category.waitFor({ state: "attached", timeout: 10000 });
  const humanCount = catalog.entries.filter((entry) => entry.category === "humans").length;
  assert.equal(await category.locator(":scope > [data-layered-overlay] > [data-layered-card]").count(), humanCount);
  assert.equal(await page.locator("[data-staffage-asset-list]").count(), 0);
  assert.ok(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth));
  assert.deepEqual(problems, []);
  console.info(`[DEBUG] Desktop Staffage retained its layered root and ${humanCount}-card Human gallery`);
} finally {
  await browser.close();
}
