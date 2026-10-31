/** 📏 Compares keyed layout boxes on the overview while the clock runs. */
import playwright from "/home/deck/semio/node_modules/playwright/index.js";
const { chromium } = playwright;
const browser = await chromium.connectOverCDP("http://127.0.0.1:9222");
const context = browser.contexts()[0] ?? await browser.newContext();
const page = await context.newPage();
await page.setViewportSize({ width: 1440, height: 1000 });
await page.goto("http://127.0.0.1:8767/#/", { waitUntil: "domcontentloaded", timeout: 30000 });
const english = page.getByRole("button", { name: "English" });
if (await english.count()) await english.click();
await page.waitForTimeout(6000);
const read = () => page.evaluate(() => {
  const out = {};
  const main = document.querySelector("main");
  if (main) out.main = `${main.offsetWidth}x${main.offsetHeight}`;
  for (const card of document.querySelectorAll("[data-overview-card]")) {
    const id = card.getAttribute("data-card") ?? "";
    if (!id.startsWith("category:") && id !== "hero" && id !== "collections" && id !== "compare" && id !== "playground") continue;
    out[`card:${id}`] = `${card.offsetWidth}x${card.offsetHeight}`;
    const frieze = card.querySelector("[data-frieze]");
    if (frieze) out[`frieze:${id}`] = `${frieze.offsetWidth}x${frieze.offsetHeight}`;
  }
  return out;
});
const samples = [];
for (let i = 0; i < 10; i++) { samples.push(await read()); await page.waitForTimeout(200); }
const series = ["main", "card:hero", "card:compare", "card:category:humans"].map((key) => `${key}: ${samples.map((sample) => sample[key]).join(" ")}`);
console.log(series.join("\n"));
await browser.close();
