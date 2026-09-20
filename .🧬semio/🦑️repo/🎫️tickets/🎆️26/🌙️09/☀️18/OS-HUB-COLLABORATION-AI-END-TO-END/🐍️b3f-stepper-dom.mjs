/** 🔎️ B3f — full DOM census of 🪵️sourcing's Pool `TableCell::Stepper` cell: the whole <td>, every
 * control in it, and whether each is enabled/visible, so the React host verdict is measured. */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";

const PORT = process.env.SEMIO_PROBE_PORT ?? "6081";
const SETTLE = Number(process.env.SEMIO_PROBE_SETTLE_MS ?? "60000");
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
page.setDefaultNavigationTimeout(240_000);
await page.goto(`http://127.0.0.1:${PORT}/?plugin=sourcing`, { waitUntil: "domcontentloaded", timeout: 240_000 });
await page.waitForFunction(() => document.querySelector("[data-semio-os-ready]"), null, { timeout: 240_000 });
await page.waitForTimeout(SETTLE);

const census = await page.evaluate(() => {
  const input = document.querySelector('[id$=".curated"]');
  if (!input) return { found: false };
  const cell = input.closest("td") ?? input.closest("[role=cell]") ?? input.parentElement?.parentElement;
  const controls = [...(cell?.querySelectorAll("button,input,[role]") ?? [])].map((el) => ({
    tag: el.tagName.toLowerCase(),
    role: el.getAttribute("role"),
    slot: el.getAttribute("data-slot"),
    aria: el.getAttribute("aria-label"),
    disabled: el.hasAttribute("disabled") || el.getAttribute("aria-disabled") === "true",
    readonly: el.hasAttribute("readonly"),
    value: el.getAttribute("value") ?? el.textContent?.trim()?.slice(0, 24) ?? null,
    box: (() => { const r = el.getBoundingClientRect(); return [Math.round(r.width), Math.round(r.height)]; })(),
  }));
  return { found: true, id: input.id, cellTag: cell?.tagName?.toLowerCase() ?? null, cellOuter: cell?.outerHTML?.slice(0, 2200) ?? null, controls };
});
console.log(JSON.stringify(census, null, 1));
await browser.close();
