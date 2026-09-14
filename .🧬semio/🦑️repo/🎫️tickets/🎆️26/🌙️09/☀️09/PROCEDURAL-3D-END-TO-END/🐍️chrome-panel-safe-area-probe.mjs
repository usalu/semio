// 🛟️ Runtime `elementFromPoint` evidence for the chrome-panel safe area (ticket 26/09/09, lane
// chrome-panel-safe-area). On the live React generation3d playground it opens the `top-right` chrome
// panel whose tab `SEMIO_PROBE_PANEL_TAB` names (default `framework.panel.toolRun`, the panel that
// swallowed the world pane's overlay rail in `📓️react-oracle-hardening-2026-09-14.md` §4.3) and reports,
// for the rail and every button in it: the boxes, the reserve the rail took, and what
// `document.elementFromPoint` returns at each centre — AFTER (the reserve applied) and BEFORE (the rail
// forced back to its unreserved offset, which is the pre-fix tree's geometry). Writes `report.json`.
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6022/?plugin=generation3d";
const seconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 90);
const panelTabId = process.env.SEMIO_PROBE_PANEL_TAB ?? "framework.panel.toolRun";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "safe-area");
mkdirSync(outDir, { recursive: true });

const lines = [];
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const t0 = Date.now();
page.on("console", (msg) => lines.push(`${Date.now() - t0} ${msg.type()} ${msg.text().slice(0, 1200)}`));
page.on("pageerror", (err) => lines.push(`${Date.now() - t0} pageerror ${String(err).slice(0, 1200)}`));
await page.goto(url, { waitUntil: "domcontentloaded" });

const deadline = Date.now() + seconds * 1000;
while (Date.now() < deadline) {
  if (await page.evaluate(() => Boolean(document.querySelector('[data-slot="world-view-overlay-rail"]')))) break;
  await page.waitForTimeout(1000);
}

const openPanel = await page.evaluate((tabId) => {
  const tab = document.getElementById(tabId);
  if (!tab) return "tab-missing";
  tab.click();
  return "clicked";
}, panelTabId);
await page.waitForTimeout(1500);

const measure = () =>
  page.evaluate(() => {
    const boxOf = (el) => {
      const r = el.getBoundingClientRect();
      return { left: Math.round(r.left), top: Math.round(r.top), right: Math.round(r.right), bottom: Math.round(r.bottom), width: Math.round(r.width), height: Math.round(r.height) };
    };
    const describe = (el) => (el ? `${el.tagName.toLowerCase()}[data-slot=${el.getAttribute("data-slot") ?? "-"}]${el.id ? `#${el.id}` : ""}` : "null");
    const rail = document.querySelector('[data-slot="world-view-overlay-rail"]');
    if (!rail) return { rail: null, panels: [], buttons: [] };
    const panels = [...document.querySelectorAll('[data-slot="panel"]')].map((el) => ({ anchor: el.getAttribute("data-anchor"), visible: el.getAttribute("data-panel-visible"), tab: el.getAttribute("data-active-tab-id"), box: boxOf(el) }));
    const at = (box) => {
      const x = (box.left + box.right) / 2;
      const y = (box.top + box.bottom) / 2;
      return document.elementFromPoint(x, y);
    };
    const buttons = [...rail.querySelectorAll("button")].map((el) => {
      const box = boxOf(el);
      const hit = at(box);
      return { slot: el.getAttribute("data-slot"), text: (el.textContent ?? "").trim().slice(0, 40), box, hit: describe(hit), reachable: Boolean(hit) && el.contains(hit) };
    });
    const railBox = boxOf(rail);
    return {
      rail: { box: railBox, safeArea: { block: Number(rail.getAttribute("data-safe-area-block") ?? 0), inline: Number(rail.getAttribute("data-safe-area-inline") ?? 0) }, styleTop: rail.style.top, styleRight: rail.style.right, hit: describe(at(railBox)) },
      panels,
      buttons,
    };
  });

const after = await measure();
await page.evaluate(() => {
  const rail = document.querySelector('[data-slot="world-view-overlay-rail"]');
  if (!rail) return;
  const block = Number(rail.getAttribute("data-safe-area-block") ?? 0);
  const inline = Number(rail.getAttribute("data-safe-area-inline") ?? 0);
  const parent = rail.offsetParent;
  rail.style.setProperty("top", `${rail.offsetTop - block}px`, "important");
  if (parent) rail.style.setProperty("right", `${parent.clientWidth - rail.offsetLeft - rail.offsetWidth - inline}px`, "important");
});
await page.waitForTimeout(300);
const before = await measure();

await page.screenshot({ path: join(outDir, "final.png"), type: "png" });
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
const report = { url, panelTabId, openPanel, after, before, pageerrors: lines.filter((line) => line.includes(" pageerror ")).length };
writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
console.log(JSON.stringify(report, null, 2));
await browser.close();
