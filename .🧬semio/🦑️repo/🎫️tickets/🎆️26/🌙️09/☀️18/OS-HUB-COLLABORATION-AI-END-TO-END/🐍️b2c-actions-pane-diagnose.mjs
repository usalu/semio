/** 🔬️ Actions-pane projection diagnostic (slice B2c).
 *
 * B2b measured `imperative` booting with an engagement/Actions chip and ZERO `action.*` rows while a
 * native test proved both window kinds carry the verbs. The React projection has exactly two places a
 * row can be lost, and they are distinguishable in the DOM:
 *
 *   A. `windowActionPaneNode` returned `undefined` — `resolveWindowActions(app, kind)` was empty, or
 *      every action was dropped by its `.filter((action) => action.inPalette)`. Then there is NO
 *      `[data-slot="window-action-pane"]` element at all, and the chip that IS visible comes from the
 *      window's engagement payload instead (`Window`'s `engagementVisible = !!(engagement||actionPane)`).
 *   B. the pane mounted but its `<Tree>` painted nothing — the element exists and carries no
 *      `action.*` descendant.
 *
 * Usage: node 🐍️b2c-actions-pane-diagnose.mjs <variant> <port> [label]
 */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const TICKET = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END";
const [variant, port, label = variant] = process.argv.slice(2);
if (!variant || !port) throw new Error("usage: b2c-actions-pane-diagnose.mjs <variant> <port> [label]");

const dump = (page) => page.evaluate(() => {
  const text = (el) => (el?.innerText ?? "").replace(/\s+/g, " ").trim();
  const ids = (selector) => [...document.querySelectorAll(selector)].map((el) => el.id).filter(Boolean);
  const panes = [...document.querySelectorAll('[data-slot="window-action-pane"]')].map((el) => ({
    parentWindow: el.closest('[data-slot="window"]')?.id ?? null,
    actionRows: [...el.querySelectorAll('[id^="action."]')].map((row) => row.id),
    sections: [...el.querySelectorAll('[id^="action.category."]')].map((row) => row.id),
    treeNodes: el.querySelectorAll("[id]").length,
    text: text(el).slice(0, 400),
  }));
  return {
    ready: document.documentElement.getAttribute("data-semio-os-ready"),
    error: document.documentElement.getAttribute("data-semio-os-error"),
    windows: ids('[data-slot="window"]'),
    actionPaneElements: panes,
    engagementToggles: ids('[id$=".engagement.toggle"]'),
    anyActionIds: [...new Set(ids('[id^="action."]'))],
    engagementContainers: [...document.querySelectorAll('[id*="engagement"]')].map((el) => ({ id: el.id, children: el.querySelectorAll("[id]").length, text: text(el).slice(0, 200) })),
    utilityBars: ids('[data-slot="window-utility-bar"], [data-slot="utility-bar"]'),
  };
});

const outDir = join(TICKET, "🗑️generated");
mkdirSync(outDir, { recursive: true });
const lines = [];
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-unsafe-webgpu", "--ignore-gpu-blocklist"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (m) => lines.push(`${m.type()} ${m.text().slice(0, 1200)}`));
page.on("pageerror", (e) => lines.push(`pageerror ${String(e).slice(0, 1200)}`));
await page.goto(`http://127.0.0.1:${port}/?plugin=${variant}`, { waitUntil: "domcontentloaded" });
for (let i = 0; i < 180; i++) {
  await page.waitForTimeout(1000);
  const state = await page.evaluate(() => ({ ready: document.documentElement.getAttribute("data-semio-os-ready"), error: document.documentElement.getAttribute("data-semio-os-error"), windows: document.querySelectorAll('[data-slot="window"]').length }));
  if (state.error) break;
  if (state.ready && state.windows > 0 && i > 6) break;
}
await page.waitForTimeout(3000);
const folded = await dump(page);
for (const toggle of folded.engagementToggles) {
  // 🖱️ A real hit-tested click AND a synthetic one: an overlaid chrome panel can swallow the first,
  // and the difference between "the row is missing" and "the pane never unfolded" is the whole answer.
  await page.locator(`[id="${toggle}"]`).first().click({ timeout: 8000, force: true }).catch(() => undefined);
  await page.waitForTimeout(1200);
  if ((await page.locator('[data-slot="window-action-pane"]').count()) === 0) {
    await page.evaluate((id) => document.getElementById(id)?.click(), toggle).catch(() => undefined);
    await page.waitForTimeout(1500);
  }
}
const unfolded = await dump(page);
const report = { variant, port, folded, unfolded, console: lines.slice(-120) };
writeFileSync(join(outDir, `b2c-${label}-actions-pane.json`), JSON.stringify(report, null, 2));
console.log(JSON.stringify({ ready: unfolded.ready, error: unfolded.error, windows: unfolded.windows, actionPanes: unfolded.actionPaneElements.length, rowsPerPane: unfolded.actionPaneElements.map((pane) => pane.actionRows.length), toggles: unfolded.engagementToggles, actionIds: unfolded.anyActionIds.length }, null, 2));
await browser.close();
