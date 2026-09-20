/** 🎯️ WHY a staged-argument combobox inside a Tree property row cannot be clicked (B3c §FL1.3.4).
 *
 * FL1 proved the control is in the viewport after the pane scrolls and that `elementFromPoint` at its
 * centre still answers `tree-gutter`. This probe names the geometry: the control's rect, its row's
 * rect, every `[data-slot="tree-gutter"]` rect in that row, and the FULL `elementsFromPoint` stack at
 * the control's centre with each entry's computed `pointer-events`/`position`/`z-index` — which is
 * what separates "the gutter is drawn over the control" from "the control is 0×0 and the gutter is
 * simply what is there".
 */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { fileURLToPath } from "node:url";
import { mkdirSync, writeFileSync } from "node:fs";
import { join, dirname } from "node:path";

const TICKET = dirname(fileURLToPath(new URL(import.meta.url)));
const OUT = join(TICKET, "🗑️generated");
mkdirSync(OUT, { recursive: true });

const variant = process.env.PB1_VARIANT ?? "generation3d";
const port = Number(process.env.PB1_PORT ?? 6018);
const row = process.env.PB1_ROW ?? "action.addWidget";
const tag = process.env.PB1_TAG ?? "before";
const panels = process.env.PB1_PANELS ?? "framework.panel.inspection,framework.panel.artifact,framework.panel.history";

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
await page.goto(`http://127.0.0.1:${port}/?plugin=${variant}`, { waitUntil: "domcontentloaded" });
for (let i = 0; i < 90; i++) {
  await page.waitForTimeout(1000);
  if (await page.evaluate(() => document.documentElement.getAttribute("data-semio-os-ready") !== null)) break;
}
await page.waitForTimeout(6000);

for (const panel of panels.split(",").filter(Boolean)) {
  await page.locator(`[data-slot="panel-tab-button"][id="${panel}"], [id="${panel}"]`).first().click({ timeout: 8000, force: true }).catch(() => {});
  await page.waitForTimeout(1200);
}
const toggles = await page.evaluate(() => [...document.querySelectorAll('[id$=".engagement.toggle"]')].map((el) => el.id));
for (const toggle of toggles) {
  await page.locator(`[id="${toggle}"]`).first().click({ timeout: 8000, force: true }).catch(() => {});
  await page.waitForTimeout(2000);
}
const rowPress = await page
  .locator(`[id="${row}"]`)
  .first()
  .click({ timeout: 8000, force: true })
  .then(() => "ok")
  .catch((error) => String(error).split("\n")[0]);
await page.waitForTimeout(2000);
const scrolled = await page
  .locator('[id*=".arg."] [role="combobox"]')
  .first()
  .scrollIntoViewIfNeeded({ timeout: 6000 })
  .then(() => "ok")
  .catch((error) => String(error).split("\n")[0]);

const facts = await page.evaluate(() => {
  const rect = (el) => {
    const r = el.getBoundingClientRect();
    return { x: Math.round(r.x), y: Math.round(r.y), w: Math.round(r.width), h: Math.round(r.height) };
  };
  const describe = (el) => {
    const style = getComputedStyle(el);
    return {
      tag: el.tagName,
      slot: el.getAttribute("data-slot") ?? undefined,
      id: el.id || undefined,
      ...rect(el),
      pointerEvents: style.pointerEvents,
      position: style.position,
      zIndex: style.zIndex,
      dim: el.hasAttribute("data-dim") || undefined,
    };
  };
  const control = [...document.querySelectorAll('[id*=".arg."] [role="combobox"]')][0];
  if (!control) return { control: null };
  const r = control.getBoundingClientRect();
  const cx = r.x + r.width / 2;
  const cy = r.y + r.height / 2;
  const propertyRow = control.closest('[data-slot="tree-property-item"]') ?? control.closest('[data-slot="tree-item"]');
  return {
    control: describe(control),
    propertyRow: propertyRow ? describe(propertyRow) : null,
    guttersInRow: propertyRow ? [...propertyRow.querySelectorAll('[data-slot="tree-gutter"]')].map(describe) : [],
    gutterSlotsInRow: propertyRow ? [...propertyRow.querySelectorAll('[data-slot="tree-gutter-slot"]')].map(describe) : [],
    rowChildren: propertyRow ? [...propertyRow.children].map(describe) : [],
    stackAtCentre: document.elementsFromPoint(cx, cy).slice(0, 12).map(describe),
    centre: { x: Math.round(cx), y: Math.round(cy) },
    containers: ["window-action-pane", "window-engagement-body", "window-engagement-zone", "window-engagement-overlay", "window-body", "window"].map((slot) => {
      const node = control.closest(`[data-slot="${slot}"]`);
      return node ? { ...describe(node), overflowX: getComputedStyle(node).overflowX } : { slot, missing: true };
    }),
    dockOfCovering: (() => {
      const hit = document.elementsFromPoint(cx, cy)[0];
      const dock = hit?.closest('[data-slot="mode-dock-panel"], [data-slot="panel-dock"], [data-slot="scroll-area"]')?.parentElement;
      return dock ? describe(dock) : null;
    })(),
    leftEdgeHit: (() => {
      const hit = document.elementFromPoint(r.x + 6, cy);
      return hit ? describe(hit) : null;
    })(),
  };
});

const clicked = await page
  .locator('[id*=".arg."] [role="combobox"]')
  .first()
  .click({ timeout: 8000 })
  .then(() => "ok")
  .catch((error) => String(error).split("\n")[0]);
await page.waitForTimeout(800);
const options = await page.evaluate(() => [...document.querySelectorAll('[role="option"]')].map((el) => el.textContent?.trim()).filter(Boolean));

const report = { variant, port, row, rowPress, scrolled, ...facts, clicked, options };
writeFileSync(join(OUT, `pb1-${variant}-gutter-${tag}.txt`), JSON.stringify(report, null, 2));
console.log(JSON.stringify({ rowPress, scrolled, clicked, options, control: report.control, stackAtCentre: report.stackAtCentre }, null, 1));
await browser.close();
