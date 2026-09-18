/** ⚛️🔎 W13b: dumps React's per-window Actions pane and Search pane DOM (ids, slots, roles) after unfolding them via their own chip ids. Read-only. */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";

const url = process.env.SEMIO_REACT_URL ?? "http://127.0.0.1:6313/?plugin=puzzle3d";
const out = process.env.SEMIO_W13B_OUT ?? "/tmp/w13b-react-dom.json";
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--use-angle=metal"] });
const ctx = await browser.newContext({ viewport: { width: 1600, height: 1000 }, deviceScaleFactor: 1 });
const page = await ctx.newPage();
const errors = [];
page.on("pageerror", (e) => errors.push(String(e).slice(0, 300)));
await page.goto(url, { waitUntil: "domcontentloaded", timeout: 180000 });
for (let s = 0; s < Number(process.env.SEMIO_REACT_BOOT ?? 420); s += 1) {
  await page.waitForTimeout(1000);
  const state = await page.evaluate(() => ({ ready: document.documentElement.getAttribute("data-semio-os-ready"), controls: document.querySelectorAll("[id],button").length })).catch(() => ({ ready: null, controls: 0 }));
  if (s % 20 === 0) console.log(`${s}s ready=${state.ready} controls=${state.controls}`);
  if (state.ready && state.controls > 40 && s > 4) { console.log(`BOOTED at ${s}s`); break; }
}

const census = () => page.evaluate(() => Array.from(document.querySelectorAll("[id]")).map((el) => el.id).filter(Boolean));
const before = await census();

const clickById = (id) => page.evaluate((target) => {
  const el = document.getElementById(target) ?? document.querySelector(`[id$='${target}']`);
  if (!el) return `missing ${target}`;
  el.click();
  return `clicked ${el.id}`;
}, id);

const dumpSlot = (slot) => page.evaluate((name) => {
  const host = document.querySelector(`[data-slot="${name}"]`);
  if (!host) return null;
  const walk = (el, depth) => ({
    depth,
    tag: el.tagName.toLowerCase(),
    id: el.id || undefined,
    slot: el.getAttribute("data-slot") || undefined,
    role: el.getAttribute("role") || undefined,
    state: el.getAttribute("data-state") || undefined,
    disabled: el.hasAttribute("disabled") || el.getAttribute("aria-disabled") === "true" || undefined,
    text: (el.children.length === 0 ? (el.textContent || "").trim().slice(0, 80) : undefined) || undefined,
    children: Array.from(el.children).map((child) => walk(child, depth + 1)),
  });
  const r = host.getBoundingClientRect();
  return { rect: [r.x, r.y, r.width, r.height], tree: walk(host, 0) };
}, slot);

const report = { url, errors };
report.chipsFound = await page.evaluate(() => Array.from(document.querySelectorAll("[id]")).map((el) => el.id).filter((id) => /\.(engagement|search|measures|utilityBar)\.(toggle|fold|unfold)$/.test(id)));

report.openActions = await clickById("framework.window.puzzle3dMainTop.engagement.toggle");
await page.waitForTimeout(700);
const afterActions = await census();
report.actionsAdded = afterActions.filter((id) => !before.includes(id));
report.actionsRemoved = before.filter((id) => !afterActions.includes(id));
report.actionPane = await dumpSlot("window-action-pane");
report.engagementBody = await dumpSlot("window-engagement-body");
report.searchBody = await dumpSlot("window-search-body");
report.searchSlot = await dumpSlot("search");

const firstRow = report.actionPane?.tree ? JSON.stringify(report.actionPane.tree).slice(0, 4000) : null;
report.firstRowPreview = firstRow;

report.expandFirstArgRow = await page.evaluate(() => {
  const pane = document.querySelector('[data-slot="window-action-pane"]');
  if (!pane) return "no pane";
  const rows = Array.from(pane.querySelectorAll("[id]")).filter((el) => /^action\./.test(el.id));
  if (!rows.length) return "no rows";
  rows[0].click();
  return `clicked ${rows[0].id} :: ${(rows[0].textContent || "").trim().slice(0, 60)}`;
});
await page.waitForTimeout(700);
report.afterFirstRow = await dumpSlot("window-action-pane");
report.afterFirstRowCensus = (await census()).filter((id) => !afterActions.includes(id));

writeFileSync(out, JSON.stringify(report, null, 2));
console.log(`wrote ${out}`);
console.log(JSON.stringify({ chips: report.chipsFound, openActions: report.openActions, added: report.actionsAdded.slice(0, 60), expand: report.expandFirstArgRow }, null, 2));
await browser.close();
