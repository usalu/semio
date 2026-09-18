/** ⚛️🧾 W13b: unfolds React's Actions pane, expands the first arg-carrying row and dumps the staged form's ids/controls. Read-only. */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";

const url = process.env.SEMIO_REACT_URL ?? "http://127.0.0.1:6313/?plugin=puzzle3d";
const out = process.env.SEMIO_W13B_OUT ?? "/tmp/w13b-react-form.json";
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--use-angle=metal"] });
const ctx = await browser.newContext({ viewport: { width: 1600, height: 1000 }, deviceScaleFactor: 1 });
const page = await ctx.newPage();
await page.goto(url, { waitUntil: "domcontentloaded", timeout: 180000 });
for (let s = 0; s < Number(process.env.SEMIO_REACT_BOOT ?? 420); s += 1) {
  await page.waitForTimeout(1000);
  const state = await page.evaluate(() => ({ ready: document.documentElement.getAttribute("data-semio-os-ready"), controls: document.querySelectorAll("[id],button").length })).catch(() => ({ ready: null, controls: 0 }));
  if (state.ready && state.controls > 40 && s > 4) { console.log(`BOOTED at ${s}s`); break; }
}
await page.waitForSelector("#\\31 framework", { timeout: 1 }).catch(() => {});
for (let attempt = 0; attempt < 30; attempt += 1) {
  const open = await page.evaluate(() => Boolean(document.querySelector('[data-slot="window-action-pane"]')));
  if (open) break;
  await page.evaluate(() => document.getElementById("framework.window.puzzle3dMainTop.engagement.toggle")?.click());
  await page.waitForTimeout(1000);
}

const report = {};
report.rows = await page.evaluate(() => {
  const pane = document.querySelector('[data-slot="window-action-pane"]');
  if (!pane) return [];
  return Array.from(pane.querySelectorAll('[data-slot="tree-item-row"],[data-slot="tree-section-row"]')).map((el) => ({ id: el.id, slot: el.getAttribute("data-slot"), role: el.getAttribute("role"), text: (el.textContent || "").trim().slice(0, 60) }));
});
report.ellipsisRows = report.rows.filter((row) => row.text.endsWith("…"));
const target = process.env.SEMIO_W13B_ROW ?? report.ellipsisRows[0]?.id;
report.target = target;
if (target) {
  await page.evaluate((id) => document.getElementById(id)?.click(), target);
  await page.waitForTimeout(800);
}
const walk = (slot) => page.evaluate((name) => {
  const host = document.querySelector(`[data-slot="${name}"]`);
  if (!host) return null;
  const rec = (el, depth) => ({ depth, tag: el.tagName.toLowerCase(), id: el.id || undefined, slot: el.getAttribute("data-slot") || undefined, role: el.getAttribute("role") || undefined, disabled: el.hasAttribute("disabled") || undefined, text: el.children.length === 0 ? (el.textContent || "").trim().slice(0, 60) || undefined : undefined, children: Array.from(el.children).map((c) => rec(c, depth + 1)) });
  return rec(host, 0);
}, slot);
report.paneAfterExpand = await walk("window-action-pane");
report.formIds = await page.evaluate(() => {
  const pane = document.querySelector('[data-slot="window-action-pane"]');
  return pane ? Array.from(pane.querySelectorAll("[id]")).map((el) => ({ id: el.id, slot: el.getAttribute("data-slot") || undefined, tag: el.tagName.toLowerCase(), text: el.children.length === 0 ? (el.textContent || "").trim().slice(0, 40) : undefined })).filter((r) => r.id && !r.id.startsWith("semio-collapsible")) : [];
});
report.formSectionIds = report.formIds.filter((r) => /\.form$|\.arg\.|\.execute$|\.reset$/.test(r.id));
writeFileSync(out, JSON.stringify(report, null, 2));
console.log(JSON.stringify({ target, ellipsis: report.ellipsisRows.slice(0, 10), formSectionIds: report.formSectionIds }, null, 2));
await browser.close();
