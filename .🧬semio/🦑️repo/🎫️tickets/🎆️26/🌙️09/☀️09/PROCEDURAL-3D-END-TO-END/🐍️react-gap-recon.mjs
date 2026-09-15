/** 🔦 Recon for `🐍️react-gap-probe.mjs`: dumps the REAL ids the React shell publishes for the four
 * steps whose selectors missed — document/inspection panel rows, the window Actions pane, the dock
 * tab focus markers and the flow graph's port-handle probe — so the gap probe aims at what exists.
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=react-verify/recon bun 🐍️react-gap-recon.mjs
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "react-verify/recon");
mkdirSync(outDir, { recursive: true });
const SURFACE = "window:procedural-main";
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 900)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 900)}`));
const out = {};
const dump = (k, v) => { out[k] = v; console.log(`[DEBUG] ${k} ${JSON.stringify(v).slice(0, 1800)}`); writeFileSync(join(outDir, "recon.json"), JSON.stringify(out, null, 2)); };

await page.goto(url, { waitUntil: "domcontentloaded" });
for (let i = 0; i < 180; i++) { await page.waitForTimeout(1000); if (await page.locator(`[data-surface-id="${SURFACE}"]`).count()) break; }
await page.waitForTimeout(10000);

dump("windows", await page.evaluate(() => ({
  windowRoots: [...document.querySelectorAll('[data-slot="window"]')].map((el) => ({ id: el.id, active: el.getAttribute("data-active") })),
  dockTabs: [...document.querySelectorAll('[data-slot="mode-dock-tab"]')].map((el) => ({ windowId: el.getAttribute("data-window-id"), active: el.getAttribute("data-active"), stackActive: el.getAttribute("data-stack-active"), html: el.outerHTML.slice(0, 260) })),
  engagementIds: [...document.querySelectorAll('[id*="engagement"]')].map((el) => ({ id: el.id, slot: el.getAttribute("data-slot"), folded: el.getAttribute("data-folded") })),
  frameworkWindowIds: [...document.querySelectorAll('[id^="framework.window."]')].map((el) => el.id).slice(0, 40),
})));

// 🗂️ Open the Document panel, then click one tree row and watch what the DOM and the shell do.
await page.locator('[id="framework.panel.artifact"]').first().click({ timeout: 8000 }).catch((e) => lines.push(`artifact tab ${String(e).slice(0,120)}`));
await page.waitForTimeout(2500);
dump("documentRows", await page.evaluate(() => [...document.querySelectorAll('[data-slot="panel"] [role="treeitem"]')].map((el) => { const r = el.getBoundingClientRect(); const cs = getComputedStyle(el); return { id: el.id, selected: el.getAttribute("aria-selected"), text: (el.textContent ?? "").trim().slice(0, 30), rect: [Math.round(r.x), Math.round(r.y), Math.round(r.width), Math.round(r.height)], pe: cs.pointerEvents, vis: cs.visibility, panel: el.closest('[data-slot="panel"]')?.getAttribute("data-anchor") ?? null }; })));

await page.evaluate(() => { window.__gapHits = []; document.addEventListener("click", (e) => { const t = e.target; window.__gapHits.push({ tag: t.tagName, id: t.id || null, slot: t.getAttribute?.("data-slot") ?? null, path: e.composedPath().slice(0, 6).map((n) => n.id || n.getAttribute?.("data-slot") || n.tagName).filter(Boolean) }); }, true); });
const row = await page.locator('[data-slot="panel"] [id="panel:procedural-play-document/height"]').first();
const rowCount = await row.count();
const mark = lines.length;
if (rowCount) await row.click({ timeout: 8000 }).catch((e) => lines.push(`row click ${String(e).slice(0,200)}`));
await page.waitForTimeout(4000);
dump("afterRowClick", { rowCount, hits: await page.evaluate(() => window.__gapHits ?? []), dispatched: lines.slice(mark).filter((l) => /performInvocation \{/.test(l)).map((l) => l.slice(0, 140)), selected: await page.evaluate(() => [...document.querySelectorAll('[role="treeitem"][aria-selected="true"]')].map((el) => el.id)), selectionJson: await page.evaluate(() => [...document.querySelectorAll("[data-selection-json]")].map((el) => (el.getAttribute("data-selection-json") ?? "").slice(0, 160))) });

await page.locator('[id="framework.panel.inspection"]').first().click({ timeout: 8000 }).catch(() => {});
await page.waitForTimeout(2500);
dump("inspector", await page.evaluate(() => ({
  ids: [...document.querySelectorAll('[data-slot="panel"] [id^="panel:procedural-play-inspector"]')].map((el) => ({ id: el.id, tag: el.tagName, value: el.value ?? null, text: (el.textContent ?? "").trim().slice(0, 60) })),
  inputs: [...document.querySelectorAll('[data-slot="panel"] input')].map((el) => ({ id: el.id, type: el.type, value: el.value })),
})));

// 🪟️ Focus the flow window, then ask why the Actions toggle refuses a click.
await page.locator(`[data-surface-id="${SURFACE}"]`).first().click({ position: { x: 20, y: 20 } }).catch(() => {});
await page.waitForTimeout(2500);
dump("toggleShape", await page.evaluate(() => [...document.querySelectorAll('[data-slot="window-pane-chrome-toggle"]')].map((el) => { const r = el.getBoundingClientRect(); const cs = getComputedStyle(el); const pane = el.closest('[data-slot="window-engagement-overlay"], [data-level="pane"]'); const pcs = pane ? getComputedStyle(pane) : null; const pr = pane?.getBoundingClientRect(); return { id: el.id, text: (el.textContent ?? "").trim().slice(0, 24), rect: [Math.round(r.x), Math.round(r.y), Math.round(r.width), Math.round(r.height)], opacity: cs.opacity, pe: cs.pointerEvents, vis: cs.visibility, display: cs.display, pane: pane ? { id: pane.id, folded: pane.getAttribute("data-folded"), opacity: pcs.opacity, pe: pcs.pointerEvents, rect: [Math.round(pr.x), Math.round(pr.y), Math.round(pr.width), Math.round(pr.height)] } : null, topAtCentre: document.elementFromPoint(r.x + r.width / 2, r.y + r.height / 2)?.outerHTML?.slice(0, 140) ?? null }; })));

const toggle = page.locator('[id="framework.window.proceduralMain.engagement.toggle"]').first();
const toggleCount = await toggle.count();
const mark2 = lines.length;
if (toggleCount) {
  await page.locator('[id="framework.window.proceduralMain"]').first().hover({ timeout: 5000 }).catch(() => {});
  await page.waitForTimeout(900);
  await toggle.click({ timeout: 6000, force: true }).catch((e) => lines.push(`toggle force click ${String(e).slice(0, 200)}`));
}
await page.waitForTimeout(3000);
dump("actionsPane", { toggleCount, dispatched: lines.slice(mark2).filter((l) => /performInvocation \{/.test(l)).map((l) => l.slice(0, 140)), paneFolded: await page.evaluate(() => document.querySelector('[id="framework.window.proceduralMain.engagement"]')?.getAttribute("data-folded") ?? null), rows: await page.evaluate(() => [...document.querySelectorAll('[id^="action."]')].map((el) => ({ id: el.id, label: (el.querySelector('[data-slot="tree-label"]')?.textContent ?? el.textContent ?? "").trim().slice(0, 40) })).slice(0, 40)) });

// 🔌️ The flow graph's own port-handle probe, polled the way 🐍️flow-window-probe.mjs polls it.
let handles = null;
for (let i = 0; i < 20; i++) {
  handles = await page.evaluate((surface) => {
    const probe = window.__semioFlowGraphProbe?.[surface];
    if (!probe) return null;
    let fx = null; try { fx = JSON.parse(probe.hostSnapshotJson?.() ?? "null"); } catch {}
    const syn = fx?.synapses ?? [];
    return { api: Object.keys(probe), kinds: ["handle", "port", "node", "wire"].map((k) => ({ k, sample: probe.entity?.(k, `${syn[0]?.to}@${syn[0]?.toPort ?? syn[0]?.to_port}`) ?? null })), nodeSample: probe.entity?.("node", syn[0]?.to ?? "") ?? null, synapses: syn };
  }, SURFACE);
  if (handles?.kinds?.some((k) => k.sample)) break;
  await page.waitForTimeout(1000);
}
dump("flowProbe", handles);

writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
await page.screenshot({ path: join(outDir, "recon.png") });
console.log("[DEBUG] RECON DONE");
await browser.close();
