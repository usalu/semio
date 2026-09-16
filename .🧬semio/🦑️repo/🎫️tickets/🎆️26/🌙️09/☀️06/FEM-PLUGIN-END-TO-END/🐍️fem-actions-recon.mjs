/** 🔎️ Recon: what the fem window's Actions pane DOM looks like (ids, fold state, rows) before and after clicking its toggle. */
import { chromium } from "playwright";
const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6086/?plugin=fem2d";
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
await page.goto(url, { waitUntil: "domcontentloaded" });
for (let i = 0; i < 60; i++) { await page.waitForTimeout(1000); if (await page.evaluate(() => document.documentElement.getAttribute("data-semio-os-ready"))) break; }
await page.waitForTimeout(3000);
const dump = () => page.evaluate(() => ({
  engagement: [...document.querySelectorAll('[id*="engagement"]')].map((el) => ({ id: el.id, tag: el.tagName, folded: el.getAttribute("data-folded"), text: (el.innerText ?? "").replace(/\s+/g, " ").slice(0, 60) })).slice(0, 20),
  actionsButtons: [...document.querySelectorAll("button")].filter((b) => /actions/i.test(b.innerText)).map((b) => ({ id: b.id, text: b.innerText.trim(), expanded: b.getAttribute("aria-expanded") })),
  actionRows: [...document.querySelectorAll('[id^="action."]')].map((el) => el.id).slice(0, 40),
}));
console.log("[DEBUG] before", JSON.stringify(await dump()).slice(0, 1500));
const coverage = await page.evaluate(() => {
  const el = document.getElementById("framework.window.fem2dModel.engagement.toggle");
  if (!el) return null;
  const r = el.getBoundingClientRect(); const x = Math.round(r.x + r.width / 2), y = Math.round(r.y + r.height / 2);
  const hit = document.elementFromPoint(x, y);
  return { rect: [Math.round(r.x), Math.round(r.y), Math.round(r.width), Math.round(r.height)], hit: hit ? `${hit.tagName}#${hit.id}.${String(hit.className).slice(0, 60)}` : null, covered: !(el === hit || el.contains(hit)) };
});
console.log("[DEBUG] coverage", JSON.stringify(coverage));
await page.locator('[data-surface-id="window:fem2d-model"]').first().click({ position: { x: 40, y: 40 } }).catch(() => {});
await page.waitForTimeout(800);
await page.locator('[id="framework.window.fem2dModel.engagement.toggle"]').first().click({ timeout: 5000 }).catch((e) => console.log("click failed", String(e).slice(0, 100)));
await page.waitForTimeout(2000);
console.log("[DEBUG] after-click", JSON.stringify((await dump()).engagement.slice(0, 2)));
const rows = await page.evaluate(() => [...document.querySelectorAll('[id="framework.window.fem2dModel.engagement"] [id]')].map((el) => `${el.tagName}#${el.id}`).slice(0, 60));
console.log("[DEBUG] rows", JSON.stringify(rows));
await page.locator('[id="action.addNode"]').first().click({ timeout: 5000 }).catch((e) => console.log("addNode click failed", String(e).slice(0, 100)));
await page.waitForTimeout(1500);
const form = await page.evaluate(() => [...document.querySelectorAll('input, select, [role="combobox"], button, [role="button"]')].filter((el) => el.closest('[id="framework.window.fem2dModel.engagement"], [role="dialog"], form, [data-slot="popover-content"]')).map((el) => `${el.tagName}#${el.id}[${el.getAttribute("name") ?? ""}|${el.getAttribute("type") ?? ""}]:${(el.innerText ?? el.value ?? "").toString().trim().replace(/\s+/g, " ").slice(0, 24)}`).slice(0, 60));
console.log("[DEBUG] form", JSON.stringify(form));
await page.screenshot({ path: "🗑️generated/actions-recon-form.png" });
console.log("[DEBUG] after", JSON.stringify(await dump()).slice(0, 2500));
await page.screenshot({ path: "🗑️generated/actions-recon.png" });
await browser.close();
