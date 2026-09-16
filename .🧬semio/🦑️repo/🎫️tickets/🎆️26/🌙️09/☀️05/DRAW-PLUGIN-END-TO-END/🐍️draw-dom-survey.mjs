/** 🔍️ Draw DOM survey: boots 6064 headless and lists window hosts, engagement toggles, action rows,
 * tree items, buttons and panel headings so the interaction probe can address real ids. */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6064/?plugin=draw";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "draw-survey");
mkdirSync(outDir, { recursive: true });
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
await page.goto(url, { waitUntil: "domcontentloaded" });
for (let i = 0; i < 120; i++) { await page.waitForTimeout(1000); if (await page.evaluate(() => document.documentElement.getAttribute("data-semio-os-ready"))) break; }
await page.waitForTimeout(5000);
const survey = () => page.evaluate(() => {
  const ids = (sel) => [...document.querySelectorAll(sel)].map((el) => el.id || el.getAttribute("data-surface-id") || el.tagName).slice(0, 80);
  return {
    hosts: ids("[data-surface-id]"),
    engagements: ids('[id$=".engagement"], [id$=".engagement.toggle"]'),
    actionRows: ids('[id^="action."]'),
    treeItems: [...document.querySelectorAll('[role="treeitem"]')].map((el) => el.innerText.replace(/\s+/g, " ").trim().slice(0, 60)).slice(0, 40),
    buttons: [...document.querySelectorAll("button")].map((el) => `${el.id || "-"}|${(el.getAttribute("aria-label") || el.innerText || "").replace(/\s+/g, " ").trim().slice(0, 40)}`).slice(0, 120),
    inputs: [...document.querySelectorAll("input,select,textarea")].map((el) => `${el.tagName}#${el.id || "-"}[${el.getAttribute("name") || el.getAttribute("placeholder") || el.getAttribute("aria-label") || ""}]`).slice(0, 60),
    body: document.body.innerText.replace(/\s+/g, " ").slice(0, 1500),
  };
});
const s1 = await survey();
await page.screenshot({ path: join(outDir, "1-boot.png") });
// open the canvas window's Actions pane
const toggle = page.locator('[id$=".engagement.toggle"]').first();
let toggled = "absent";
if (await toggle.count()) toggled = await toggle.click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 100));
await page.waitForTimeout(1500);
const s2 = await survey();
await page.screenshot({ path: join(outDir, "2-actions.png") });
writeFileSync(join(outDir, "survey.json"), JSON.stringify({ boot: s1, toggled, actions: s2 }, null, 2));
console.log("[DEBUG] SURVEY", JSON.stringify({ hosts: s1.hosts, engagements: s1.engagements, toggled, actionRows: s2.actionRows, tree: s2.treeItems, inputs: s2.inputs, buttons: s2.buttons.slice(0, 60) }).slice(0, 5000));
await browser.close();
