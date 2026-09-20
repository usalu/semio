/** 🧺️ B3f — 🪵️sourcing's SECOND `TableCell::Stepper` (the Curated window's `count` cell,
 * `🧺️curated/🦀️.rs:82`). The Pool stepper creates the curated item; this one has to raise its count,
 * proving the one React renderer covers both windows rather than only the measured one. */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { fileURLToPath } from "node:url";
import { mkdirSync, writeFileSync } from "node:fs";

const PORT = process.env.SEMIO_PROBE_PORT ?? "6081";
const KIND = process.env.SEMIO_PROBE_KIND ?? "beam-glulam-gl24h";
const OUT = fileURLToPath(new URL("🗑️generated/b3f-curated-stepper/", import.meta.url));
mkdirSync(OUT, { recursive: true });

const faults = [];
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
page.on("console", (m) => {
  const text = m.text();
  if (/refused|panicked|unreachable|dispatch-failed/i.test(text)) faults.push(text.slice(0, 200));
});
page.setDefaultNavigationTimeout(240_000);
await page.goto(`http://127.0.0.1:${PORT}/?plugin=sourcing`, { waitUntil: "domcontentloaded", timeout: 240_000 });
await page.waitForFunction(() => document.querySelector("[data-semio-os-ready]"), null, { timeout: 240_000 });
await page.waitForTimeout(Number(process.env.SEMIO_PROBE_SETTLE_MS ?? "60000"));

const poolId = `window:sourcing-pool.${KIND}.curated`;
const curatedId = `window:sourcing-curated.${KIND}.count`;
const readCurated = () =>
  page.evaluate((id) => {
    const input = document.querySelector(`[id="${id}"]`);
    return input === null
      ? null
      : {
          value: input.value,
          role: input.getAttribute("role"),
          valueNow: input.getAttribute("aria-valuenow"),
          label: input.getAttribute("aria-label"),
          decrementDisabled: document.querySelector(`[data-stepper-control="decrement"][data-stepper-for="${id}"]`)?.hasAttribute("disabled") ?? null,
        };
  }, curatedId);

const press = (control, id) => page.locator(`[data-stepper-control="${control}"][data-stepper-for="${id}"]`).first().click({ timeout: 10_000 }).then(() => "ok").catch((e) => String(e).split("\n")[0].slice(0, 90));

const seedCuratedRow = await press("increment", poolId);
await page.waitForTimeout(6000);
const afterSeed = await readCurated();
const raiseCount = await press("increment", curatedId);
await page.waitForTimeout(6000);
const afterRaise = await readCurated();
const lowerCount = await press("decrement", curatedId);
await page.waitForTimeout(6000);
const afterLower = await readCurated();

await page.screenshot({ path: `${OUT}/curated-stepper.png` }).catch(() => {});
const report = { poolId, curatedId, seedCuratedRow, afterSeed, raiseCount, afterRaise, lowerCount, afterLower, faults };
writeFileSync(`${OUT}/report.json`, JSON.stringify(report, null, 1));
console.log(JSON.stringify(report, null, 1));
await browser.close();
