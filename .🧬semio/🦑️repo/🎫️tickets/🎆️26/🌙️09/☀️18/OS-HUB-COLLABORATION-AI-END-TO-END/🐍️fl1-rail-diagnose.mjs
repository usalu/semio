/** 🔍️ Why a LONG Actions rail's staged-argument form is unreachable at 1600×1000 (B3c §4.3).
 *
 * Opens the rail, presses the arg-carrying row, then walks the ancestor chain of the staged control
 * reporting for each box: slot, class, overflow-y, offsetHeight/scrollHeight/clientHeight, computed
 * max-height and position — the one thing a `click: Timeout` cannot tell you. Names the first
 * ancestor that could scroll and whether it actually can.
 */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { fileURLToPath } from "node:url";
import { mkdirSync, writeFileSync } from "node:fs";
import { join, dirname } from "node:path";

const TICKET = dirname(fileURLToPath(new URL(import.meta.url)));
const OUT = join(TICKET, "🗑️generated");
mkdirSync(OUT, { recursive: true });

const variant = process.env.FL1_VARIANT ?? "generation3d";
const port = Number(process.env.FL1_PORT ?? 6018);
const row = process.env.FL1_ROW ?? "action.addWidget";
const tag = process.env.FL1_TAG ?? "before";

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
await page.goto(`http://127.0.0.1:${port}/?plugin=${variant}`, { waitUntil: "domcontentloaded" });
for (let i = 0; i < 90; i++) {
  await page.waitForTimeout(1000);
  if (await page.evaluate(() => document.documentElement.getAttribute("data-semio-os-ready") !== null)) break;
}
await page.waitForTimeout(6000);

// 🪟️ Mirror the shared bar probe's own state: it opens the app panels and the History panel before it
// ever presses the rail, and a docked panel changes the window band the rail has to live in.
for (const panel of (process.env.FL1_PANELS ?? "").split(",").filter(Boolean)) {
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

const facts = await page.evaluate(() => {
  const box = (el) => {
    const r = el.getBoundingClientRect();
    return { y: Math.round(r.y), h: Math.round(r.height) };
  };
  const chain = (el) => {
    const rows = [];
    for (let node = el; node && node !== document.documentElement; node = node.parentElement) {
      const style = getComputedStyle(node);
      rows.push({
        slot: node.getAttribute("data-slot") ?? node.tagName.toLowerCase(),
        id: node.id || undefined,
        overflowY: style.overflowY,
        position: style.position,
        maxHeight: style.maxHeight,
        minHeight: style.minHeight,
        offsetHeight: node.offsetHeight,
        scrollHeight: node.scrollHeight,
        clientHeight: node.clientHeight,
        scrollable: node.scrollHeight > node.clientHeight && /auto|scroll/.test(style.overflowY),
      });
    }
    return rows;
  };
  const control = [...document.querySelectorAll('[id*=".arg."] [role="combobox"], [id*=".arg."] input, [id*=".arg."]')].find((el) => el.getBoundingClientRect().height > 0);
  const execute = [...document.querySelectorAll('[id$=".execute"]')].find((el) => el.getBoundingClientRect().height > 0);
  return {
    viewportHeight: window.innerHeight,
    control: control ? { id: control.id, ...box(control), chain: chain(control) } : null,
    execute: execute ? { id: execute.id, ...box(execute) } : null,
    inViewport: control ? control.getBoundingClientRect().bottom <= window.innerHeight && control.getBoundingClientRect().top >= 0 : null,
  };
});

// 🎯️ The user's own recourse: ask the browser to bring it into view, then re-measure and click it.
const scrolled = await page
  .locator('[id*=".arg."] [role="combobox"]')
  .first()
  .scrollIntoViewIfNeeded({ timeout: 6000 })
  .then(() => "ok")
  .catch((error) => String(error).split("\n")[0]);
const afterScroll = await page.evaluate(() => {
  const el = [...document.querySelectorAll('[id*=".arg."] [role="combobox"]')][0];
  if (!el) return null;
  const r = el.getBoundingClientRect();
  const hit = document.elementFromPoint(r.x + r.width / 2, r.y + r.height / 2);
  const pane = el.closest('[data-slot="window-action-pane"]');
  return {
    y: Math.round(r.y),
    bottom: Math.round(r.bottom),
    inViewport: r.top >= 0 && r.bottom <= window.innerHeight,
    paneScrollTop: pane ? pane.scrollTop : null,
    paneScrollRange: pane ? pane.scrollHeight - pane.clientHeight : null,
    atCentre: hit === null ? null : `${hit.tagName}#${hit.id || "(no id)"}[${hit.getAttribute("data-slot") ?? ""}]`,
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

const report = { variant, port, row, rowPress, ...facts, scrolled, afterScroll, clicked, options };
writeFileSync(join(OUT, `fl1-${variant}-rail-${tag}.txt`), JSON.stringify(report, null, 2));
await page.screenshot({ path: join(OUT, `fl1-${variant}-rail-${tag}.png`) });
console.log(JSON.stringify({ rowPress, viewportHeight: report.viewportHeight, control: report.control && { id: report.control.id, y: report.control.y }, inViewport: report.inViewport, scrolled, clicked, options: options.slice(0, 6) }));
console.log(JSON.stringify(report.control?.chain?.slice(0, 14) ?? [], null, 1));
await browser.close();
