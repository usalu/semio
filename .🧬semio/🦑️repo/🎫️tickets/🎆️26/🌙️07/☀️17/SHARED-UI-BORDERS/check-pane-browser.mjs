import { chromium } from "playwright";
import { browserOptions } from "./browser-env.mjs";
import { readFile, writeFile } from "node:fs/promises";
const vectors = JSON.parse(await readFile("🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪟️Window/🧪️tests/🧫️fixtures/pane-edges.json", "utf8"));
const moduleUrl = "/" + new URL("pane-browser.tsx", import.meta.url).pathname.slice(process.cwd().length + 1);
const browser = await chromium.launch(browserOptions());
const results = [];
try {
  const page = await browser.newPage({ viewport: { width: 1200, height: 800 } });
  const errors = [];
  page.on("pageerror", error => errors.push(error.message));
  await page.goto("http://127.0.0.1:6011/iframe.html?id=" + encodeURIComponent("🖱️ui⚛️react-app--default") + "&viewMode=story");
  await page.locator('[data-slot="utility-bar"]').first().waitFor();
  for (const direction of vectors.directions) for (const folded of vectors.folds) for (const row of vectors.anchors) {
    await page.evaluate(async ({ moduleUrl, direction, folded, anchor }) => (await import(moduleUrl)).mount({ direction, folded, anchor }), { moduleUrl, direction, folded, anchor: row.anchor });
    await page.waitForFunction(({ folded, anchor }) => {
      const pane = document.querySelector('#pane-probe [id="contributed-pane"]');
      return pane?.dataset.anchor === anchor && (pane.dataset.folded === 'true') === folded && pane.getBoundingClientRect().width > 0;
    }, { folded, anchor: row.anchor });
    await page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve))));
    const result = await page.evaluate(({ row, folded, direction, windows }) => {
      const scope = document.querySelector('#pane-probe');
      const body = scope.querySelector('[data-slot="window-body"]').getBoundingClientRect();
      const panes = [...windows.filter(p => p.slot !== 'window-search-overlay'), { ...row, slot: 'contributed-pane' }].map(p => {
        const element = scope.querySelector(p.slot === 'contributed-pane' ? '[id="contributed-pane"]' : `[data-slot="${p.slot}"]`);
        const chrome = element.querySelector('[data-window-silhouette]');
        const rect = chrome.getBoundingClientRect();
        return { slot: p.slot, edges: p.edges.map(edge => ({ edge, delta: rect[edge] - body[edge] })), width: rect.width, height: rect.height, clip: getComputedStyle(chrome).clipPath, paths: [...chrome.querySelectorAll('[data-window-silhouette-border] path')].map(p => p.getAttribute('d')) };
      });
      return { direction, folded, anchor: row.anchor, panes };
    }, { row, direction, folded, windows: vectors.windowPanes });
    for (const pane of result.panes) for (const edge of pane.edges) if (Math.abs(edge.delta) > 0.1) throw new Error(JSON.stringify(result));
    results.push(result);
  }
  if (errors.length) throw new Error(errors.join("\n"));
  await writeFile(new URL("🗑️generated/pane-browser.json", import.meta.url), JSON.stringify(results, null, 2));
  console.log(`Passed ${results.length} actual Mode/Window pane geometry scenarios in Chromium`);
} finally { await browser.close(); }
