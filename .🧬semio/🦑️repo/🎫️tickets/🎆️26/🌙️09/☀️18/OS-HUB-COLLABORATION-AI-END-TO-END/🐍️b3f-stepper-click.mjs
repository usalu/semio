/** 🖱️ B3f — clicks 🪵️sourcing's Pool curated stepper `+` and reports whether the document mutated
 * (edit count / stepper readout / history ledger), then undo and redo through the shell rails. */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { fileURLToPath } from "node:url";
import { mkdirSync, writeFileSync } from "node:fs";

const PORT = process.env.SEMIO_PROBE_PORT ?? "6081";
const SETTLE = Number(process.env.SEMIO_PROBE_SETTLE_MS ?? "60000");
const OUT = fileURLToPath(new URL(`🗑️generated/${process.env.SEMIO_PROBE_OUT ?? "b3f-stepper-click"}/`, import.meta.url));
mkdirSync(OUT, { recursive: true });

const faults = [];
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
page.on("console", (message) => {
  const text = message.text();
  if (/refused|Fault|panicked|unreachable|\[DEBUG\]/i.test(text) && !text.includes("typed-operation slots")) faults.push(`${message.type()} ${text.slice(0, 220)}`);
});
page.setDefaultNavigationTimeout(240_000);
await page.goto(`http://127.0.0.1:${PORT}/?plugin=sourcing`, { waitUntil: "domcontentloaded", timeout: 240_000 });
await page.waitForFunction(() => document.querySelector("[data-semio-os-ready]"), null, { timeout: 240_000 });
await page.waitForTimeout(SETTLE);

const state = () =>
  page.evaluate(() => {
    const input = document.querySelector('[id$=".curated"]');
    const edits = [...document.querySelectorAll("*")].map((el) => el.textContent ?? "").filter((t) => /Edits?\s*[:·]?\s*\d+/.test(t) && t.length < 60).at(-1) ?? null;
    return {
      stepper: input?.getAttribute("value") ?? input?.value ?? null,
      spin: document.querySelector('[id$=".curated"]')?.closest('[role="spinbutton"]')?.getAttribute("aria-valuenow") ?? null,
      edits,
      ledger: [...document.querySelectorAll('[id^="framework.history.entry"]')].map((el) => el.textContent?.trim()?.slice(0, 40) ?? "").slice(0, 12),
    };
  });

const before = await state();
const plus = page.locator('td:has([id$=".curated"]) button').last();
let clicked = "absent";
try {
  await plus.click({ timeout: 8000 });
  clicked = "ok";
} catch (error) {
  clicked = String(error).slice(0, 120);
}
await page.waitForTimeout(6000);
const afterClick = await state();

// keyboard route: focus the spinbutton and press ArrowUp
let keyboard = "absent";
try {
  const spin = page.locator('[data-slot="table-stepper"]').first();
  await spin.focus({ timeout: 4000 });
  await page.keyboard.press("ArrowUp");
  keyboard = "ok";
} catch (error) {
  keyboard = String(error).slice(0, 120);
}
await page.waitForTimeout(6000);
const afterKey = await state();

await page.screenshot({ path: `${OUT}/stepper.png`, fullPage: false });
const report = { before, clicked, afterClick, keyboard, afterKey, mutated: afterClick.stepper !== before.stepper || afterClick.ledger.length !== before.ledger.length, faults };
writeFileSync(`${OUT}/report.json`, JSON.stringify(report, null, 1));
console.log(JSON.stringify(report, null, 1));
await browser.close();
