/** 🔬️ Energy panel DOM dump — the diagnostic that tells you what the live shell actually publishes, so a probe can be
 * written against facts instead of guesses. Boots the playground, then for each step dumps every panel-tab button,
 * every `panel:<ns>/<row>` namespace and row, and every form control under the inspection namespace.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=dom-dump bun 🐍️energy-dom-dump.mjs
 * Env: SEMIO_PROBE_URL, SEMIO_PROBE_OUT, SEMIO_PROBE_SECONDS, SEMIO_PROBE_ROWS (comma ids to click, default "40,50").
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6106/?plugin=energy";
const bootSeconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 120);
const rowsToClick = (process.env.SEMIO_PROBE_ROWS ?? "40,50").split(",").map((s) => s.trim()).filter(Boolean);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "dom-dump");
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const dump = { url, steps: [] };
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 1500)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 1500)}`));
const flush = () => { writeFileSync(join(outDir, "dump.json"), JSON.stringify(dump, null, 2)); writeFileSync(join(outDir, "console.txt"), lines.join("\n")); };

const snapshot = () => page.evaluate(() => {
  const rows = {};
  for (const el of document.querySelectorAll('[id^="panel:"]')) {
    const m = /^panel:([^/]+)\/(.*)$/.exec(el.id);
    if (!m) continue;
    const r = el.getBoundingClientRect();
    (rows[m[1]] ??= []).push({
      id: m[2], tag: el.tagName, role: el.getAttribute("role"), selected: el.getAttribute("aria-selected"), expanded: el.getAttribute("aria-expanded"),
      w: Math.round(r.width), h: Math.round(r.height), visible: r.width > 0 && r.height > 0,
      type: el.getAttribute("type"), value: "value" in el ? String(el.value).slice(0, 30) : null,
      text: (el.textContent ?? "").trim().replace(/\s+/g, " ").slice(0, 70),
    });
  }
  const tabs = [...document.querySelectorAll('[id^="framework.panel"]')].map((b) => {
    const r = b.getBoundingClientRect();
    return { id: b.id, tag: b.tagName, text: (b.textContent ?? "").trim().slice(0, 30), state: b.getAttribute("data-state") ?? b.getAttribute("aria-selected"), w: Math.round(r.width), h: Math.round(r.height) };
  });
  const inputs = [...document.querySelectorAll("input, select, textarea, [role=\"spinbutton\"], [role=\"combobox\"], [contenteditable]")].map((el) => ({
    id: el.id || null, tag: el.tagName, type: el.getAttribute("type"), role: el.getAttribute("role"), inputmode: el.getAttribute("inputmode"),
    value: "value" in el ? String(el.value).slice(0, 30) : el.getAttribute("aria-valuenow"), path: el.getAttribute("data-ui-path"),
    aria: el.getAttribute("aria-label"),
  }));
  return { namespaces: Object.fromEntries(Object.entries(rows).map(([k, v]) => [k, v.length])), rows, tabs, inputs, surfaces: [...document.querySelectorAll("[data-surface-id]")].map((e) => e.getAttribute("data-surface-id")) };
});
const step = async (name, extra = {}) => { const snap = await snapshot(); dump.steps.push({ name, t: Date.now() - t0, ...extra, ...snap }); console.log(`[DEBUG] ${name} ns=${JSON.stringify(snap.namespaces)} inputs=${snap.inputs.length}`); flush(); await page.screenshot({ path: join(outDir, `${dump.steps.length}-${name}.png`) }).catch(() => {}); };
const openTab = async (id, wantNs, seconds = 20) => {
  const tab = page.locator(`[id="${id}"]`).first();
  if (!(await tab.count())) return `absent:${id}`;
  await tab.click({ force: true }).catch(() => {});
  for (let i = 0; i < seconds * 2; i++) {
    await page.waitForTimeout(500);
    const snap = await page.evaluate((ns) => Object.keys([...document.querySelectorAll('[id^="panel:"]')].reduce((acc, el) => { const m = /^panel:([^/]+)\//.exec(el.id); if (m) acc[m[1]] = 1; return acc; }, {})).filter((k) => k.includes(ns)), wantNs);
    if (snap.length) return `ok after ${(i + 1) * 500}ms → ${snap.join(",")}`;
  }
  return `opened but no ns matching '${wantNs}' within ${seconds}s`;
};
const clickRow = async (domId) => {
  const row = page.locator(`[id="${domId}"]`).first();
  if (!(await row.count())) return `absent:${domId}`;
  await row.evaluate((el) => el.scrollIntoView({ block: "center" })).catch(() => {});
  await page.waitForTimeout(250);
  const box = await row.boundingBox();
  if (!box) return `no-box:${domId}`;
  await page.mouse.click(box.x + Math.min(100, box.width / 2), box.y + box.height / 2);
  await page.waitForTimeout(2000);
  return `ok box=${Math.round(box.width)}x${Math.round(box.height)}`;
};

await page.goto(url, { waitUntil: "domcontentloaded" });
for (let i = 0; i < bootSeconds; i++) { await page.waitForTimeout(1000); const r = await page.evaluate(() => document.documentElement.getAttribute("data-semio-os-ready")); if (r && i > 8) break; }
await step("boot");
await step("artifact-tab", { open: await openTab("framework.panel.artifact", "artifact") });
for (const id of rowsToClick) {
  const clicked = await clickRow(`panel:energy-model-artifact/${id}`);
  await step(`clicked-${id}`, { clicked });
  const open = await openTab("framework.panel.inspection", "inspection");
  await step(`inspection-after-${id}`, { open });
  const back = await openTab("framework.panel.artifact", "artifact");
  await step(`artifact-again-after-${id}`, { back });
}
flush();
console.log("DONE", outDir);
await browser.close();
