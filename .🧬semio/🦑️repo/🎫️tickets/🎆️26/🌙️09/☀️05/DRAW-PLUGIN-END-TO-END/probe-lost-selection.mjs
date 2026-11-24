import { chromium } from "playwright";
import { mkdirSync, writeFileSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
const [out, iterations = "15", rate = "5", mode = "journey"] = process.argv.slice(2);
mkdirSync(out, { recursive: true });
const fixture = JSON.parse(readFileSync(resolve(process.env.FIXTURE), "utf8"));
const labels = { artifact: "Artifact", catalogue: "Catalogue", inspection: "Inspection", addPath: "Add Path", path: "Path" };
const browser = await chromium.launch({ headless: true, args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
const results = [];
for (let i = 0; i < Number(iterations); i++) {
  const context = await browser.newContext({ locale: "en", viewport: { width: 1600, height: 1000 }, acceptDownloads: true });
  const page = await context.newPage();
  const logs = [];
  page.on("console", m => { const t = m.text(); if (t.includes("interactionSelect") || t.startsWith("[PROBE]")) logs.push(t.slice(0, 200)); });
  await page.addInitScript(() => {
    globalThis.__ev = [];
    const rowSel = '[role="treeitem"][id^="panel:framework.panel.artifact/drawing-play-layers."]';
    document.addEventListener("click", e => { const r = e.target.closest?.(rowSel); globalThis.__ev.push({ t: Math.round(performance.now()), type: "click", id: r?.id ?? e.target.id ?? e.target.tagName, row: !!r, connected: e.target.isConnected, sel: r?.getAttribute("aria-selected") }); }, true);
    let last = "";
    const snap = () => { const ids = [...document.querySelectorAll('[role="treeitem"][id^="panel:framework.panel.artifact/drawing-play-layers."]')].map(e => e.id.split("drawing-play-layers.")[1]).filter(x => !x.startsWith("add.")).join(","); if (ids !== last) { last = ids; globalThis.__ev.push({ t: Math.round(performance.now()), type: "rows", ids }); } };
    new MutationObserver(snap).observe(document, { subtree: true, childList: true });
    const W = globalThis.Worker; const pm = W.prototype.postMessage;
    W.prototype.postMessage = function (m, ...r) { try { const s = JSON.stringify(m, (k, v) => typeof v === "bigint" ? v + "n" : v instanceof Uint8Array ? "u8" : v) ?? ""; if (s.includes("interactionSelect")) globalThis.__ev.push({ t: Math.round(performance.now()), type: "post-select", s: s.slice(0, 300) }); } catch {} return pm.call(this, m, ...r); };
  });
  const cdp = await context.newCDPSession(page);
  await cdp.send("Emulation.setCPUThrottlingRate", { rate: Number(rate) });
  const r = { i, ok: false };
  const engagement = page.locator('[id^="framework.window."][id$=".engagement.toggle"]').first();
  const status = page.locator('[id="drawing-composite"] [data-slot="engagement-status"]');
  try {
    await page.goto("http://127.0.0.1:6064/?plugin=draw", { waitUntil: "domcontentloaded", timeout: 120000 });
    await page.waitForFunction(() => document.documentElement.dataset.semioOsReady || document.documentElement.dataset.semioOsError, undefined, { timeout: 180000 });
    await page.locator('[data-surface-role="editor"] [data-ui-node-key="drawing.play.composite"] canvas').first().waitFor({ state: "visible", timeout: 60000 });
    await page.locator('[id="playground.navbar.fixture"]').first().getByText(fixture.example.label, { exact: true }).waitFor({ state: "visible", timeout: 90000 });
    if (!(await status.isVisible())) await engagement.click({ timeout: 30000 });
    await page.waitForFunction(({ sel, n }) => { const m = document.querySelector(sel)?.textContent?.match(/(\d+) (?:layers?|Ebenen?)/); return m && Number(m[1]) >= n; }, { sel: '[id="drawing-composite"] [data-slot="engagement-status"]', n: fixture.minimumBootLayers }, { timeout: 90000 });
    if (await status.isVisible()) await engagement.click({ timeout: 30000 });
    await status.waitFor({ state: "hidden", timeout: 30000 });
    const openPanel = async (id, name) => {
      const button = page.locator(`button[id="${id}"]`).first();
      const panel = page.locator(`[data-slot="panel"][id="framework.panelTab.${id}"]`).first();
      if (!(await panel.isVisible())) await button.click({ timeout: 30000 });
      await panel.waitFor({ state: "visible", timeout: 30000 });
      return panel;
    };
    const lp = await openPanel("framework.panel.artifact", labels.artifact);
    await lp.getByText(labels.addPath, { exact: true }).first().waitFor({ state: "visible", timeout: 30000 });
    const row = mode === "old" ? lp.locator(`[role="treeitem"][id^="panel:framework.panel.artifact/drawing-play-layers."]:not([id^="panel:framework.panel.artifact/drawing-play-layers.add."]):not([id$="/drawing-play-layers.empty"])`).first() : lp.locator(`[role="treeitem"][id="panel:framework.panel.artifact/drawing-play-layers.path.${fixture.exampleLayer}"]`);
    await row.click({ timeout: 30000 });
    const ip = await openPanel("framework.panel.inspection", labels.inspection);
    await ip.locator(`[id*="/drawing-inspector."]`).first().waitFor({ state: "visible", timeout: 15000 * Math.max(1, Number(rate) / 2) });
    r.ok = true;
  } catch (e) { r.error = String(e).slice(0, 300); await page.screenshot({ path: join(out, `fail-${i}.png`) }).catch(() => {}); }
  r.events = await page.evaluate(() => globalThis.__ev).catch(() => null);
  r.logs = logs;
  r.rowState = await page.evaluate(() => [...document.querySelectorAll('[role="treeitem"][id^="panel:framework.panel.artifact/drawing-play-layers."]')].map(e => e.id + "|" + e.getAttribute("aria-selected"))).catch(() => null);
  results.push(r);
  console.log(`iter ${i} ok=${r.ok} ${r.error ?? ""} events=${JSON.stringify(r.events)?.slice(0, 400)}`);
  writeFileSync(join(out, "results.json"), JSON.stringify(results, null, 2));
  await context.close();
}
console.log(`DONE iterations=${results.length} failures=${results.filter(r => !r.ok).length}`);
await browser.close();
