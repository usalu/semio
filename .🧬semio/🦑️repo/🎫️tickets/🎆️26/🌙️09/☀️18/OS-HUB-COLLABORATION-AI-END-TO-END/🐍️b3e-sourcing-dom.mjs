/** 🔎️ One-shot DOM census of 🪵️sourcing's Pool table: what the curated Stepper actually renders as. */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
page.setDefaultNavigationTimeout(180_000);
await page.goto("http://127.0.0.1:6081/?plugin=sourcing", { waitUntil: "domcontentloaded", timeout: 180_000 });
await page.waitForFunction(() => document.querySelector("[data-semio-os-ready]"), null, { timeout: 180_000 });
await page.waitForTimeout(30_000);
const out = await page.evaluate(() => {
  const pane = document.querySelector('[data-surface-id="window:sourcing-pool"]') ?? document.body;
  const stepper = pane.querySelector('[id$=".curated"]');
  const cell = stepper?.closest("td,[role=cell],div");
  return {
    ready: document.querySelector("[data-semio-os-ready]")?.getAttribute("data-semio-os-ready"),
    stepperId: stepper?.id ?? null,
    stepperOuter: stepper?.outerHTML?.slice(0, 500) ?? null,
    cellOuter: cell?.outerHTML?.slice(0, 1400) ?? null,
    parentOuter: stepper?.parentElement?.outerHTML?.slice(0, 1400) ?? null,
  };
});
console.log(JSON.stringify(out, null, 1));
await browser.close();
