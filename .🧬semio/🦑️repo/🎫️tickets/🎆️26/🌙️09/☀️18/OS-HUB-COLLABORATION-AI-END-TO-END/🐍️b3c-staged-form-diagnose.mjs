/** 🔍️ Why `generation3d`'s staged `addWidget` form cannot be filled while `generation2d`'s can.
 *
 * The shared probe reported `kind:TimeoutError: click: Timeout 8000ms exceeded.` on :6018 and
 * `kind=inputSlider` on :6021, with the same control, the same selector and the same session family.
 * This opens the Actions rail, presses the `addWidget` row, and then reports — for every element whose
 * id ends in `.arg.kind` — its box, visibility, `pointer-events`, and what `elementFromPoint` answers
 * at its centre, which is the one thing a click timeout cannot tell you.
 */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { fileURLToPath } from "node:url";
import { mkdirSync, writeFileSync } from "node:fs";
import { join, dirname } from "node:path";

const TICKET = dirname(fileURLToPath(new URL(import.meta.url)));
const OUT = join(TICKET, "🗑️generated");
mkdirSync(OUT, { recursive: true });

const variant = process.env.B3C_VARIANT ?? "generation3d";
const port = Number(process.env.B3C_PORT ?? 6018);

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
await page.goto(`http://127.0.0.1:${port}/?plugin=${variant}`, { waitUntil: "domcontentloaded" });
for (let i = 0; i < 90; i++) {
  await page.waitForTimeout(1000);
  if (await page.evaluate(() => document.documentElement.getAttribute("data-semio-os-ready") !== null)) break;
}
await page.waitForTimeout(6000);

const toggles = await page.evaluate(() => [...document.querySelectorAll('[id$=".engagement.toggle"]')].map((el) => el.id));
for (const toggle of toggles) {
  await page.locator(`[id="${toggle}"]`).first().click({ timeout: 8000, force: true }).catch(() => {});
  await page.waitForTimeout(2500);
}
const rowPress = await page.locator('[id="action.addWidget"]').first().click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).split("\n")[0]);
await page.waitForTimeout(2000);

const facts = await page.evaluate(() => {
  const box = (el) => { const r = el.getBoundingClientRect(); return { x: Math.round(r.x), y: Math.round(r.y), w: Math.round(r.width), h: Math.round(r.height) }; };
  const describe = (el) => ({
    id: el.id,
    tag: el.tagName,
    role: el.getAttribute("role"),
    box: box(el),
    mounted: el.offsetParent !== null,
    pointerEvents: getComputedStyle(el).pointerEvents,
    visibility: getComputedStyle(el).visibility,
    opacity: getComputedStyle(el).opacity,
    atCentre: (() => {
      const r = el.getBoundingClientRect();
      const hit = document.elementFromPoint(r.x + r.width / 2, r.y + r.height / 2);
      return hit === null ? null : `${hit.tagName}#${hit.id || "(no id)"}`;
    })(),
  });
  return {
    argControls: [...document.querySelectorAll('[id*=".arg."]')].map(describe),
    comboboxes: [...document.querySelectorAll('[role="combobox"]')].map(describe),
    executeControls: [...document.querySelectorAll('[id$=".execute"]')].map(describe),
    openPanels: [...document.querySelectorAll('[data-slot="panel"]')].filter((el) => el.offsetParent !== null).map((el) => ({ id: el.id, box: box(el) })),
  };
});

writeFileSync(join(OUT, `b3c-${variant}-staged-form.txt`), JSON.stringify({ variant, port, rowPress, ...facts }, null, 2));
await page.screenshot({ path: join(OUT, `b3c-${variant}-staged-form.png`) });
console.log(JSON.stringify({ variant, rowPress, argControls: facts.argControls, comboboxes: facts.comboboxes.slice(0, 6), executeControls: facts.executeControls }, null, 1).slice(0, 4000));
await browser.close();
